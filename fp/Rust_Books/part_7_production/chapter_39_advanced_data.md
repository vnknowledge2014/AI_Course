# Chapter 39 — Advanced Data Patterns

> **Bạn sẽ học được**:
> - **Zero-downtime Migrations**: Nâng cấp Database không làm sập hệ thống.
> - **CQRS**: Tách biệt hoàn toàn Database Đọc và Ghi.
> - **Event Sourcing**: Lưu trữ mọi thay đổi (Append-only) thay vì ghi đè.
> - **Caching Patterns**: Các chiến lược Read-Through, Write-Through.
> - **NoSQL vs SQL**: Khi nào dùng Redis, MongoDB, hay ScyllaDB?
>
> **Yêu cầu trước**: Chapter 19 (CQRS/ES), Chapter 38 (Database).
> **Thời gian đọc**: ~45 phút | **Level**: Principal
> **Kết quả cuối cùng**: Nắm vững các pattern thiết kế Dữ liệu cấp độ Production, vượt xa khỏi các ứng dụng CRUD thông thường.

---

Hầu hết các khóa học chỉ dạy bạn CRUD (Create, Read, Update, Delete). Đó là bước đầu tiên. 
Nhưng ở môi trường Production với hàng triệu users, CRUD là không đủ. Làm sao nâng cấp schema DB mà không dừng Server? Làm sao xử lý khi lượng request ĐỌC gấp 100 lần request GHI? Làm sao khôi phục lại trạng thái giỏ hàng của user vào 2 ngày trước? 

Chương này sẽ trang bị cho bạn tư duy thiết kế Dữ liệu (Data Design) của một Principal Engineer.

## 39.1 — Schema Migrations & Zero Downtime

Ở môi trường Dev, nếu bạn muốn thêm cột, bạn có thể dễ dàng xóa bảng và tạo lại (`DROP TABLE`). Ở Production, làm vậy đồng nghĩa với đuổi việc.
**Migrations** là tập hợp các file SQL được đánh số thứ tự, diễn tả sự thay đổi của Database theo thời gian.

```sql
-- migrations/001_create_users.sql
CREATE TABLE users (id SERIAL PRIMARY KEY, name VARCHAR);

-- migrations/002_add_avatar.sql
ALTER TABLE users ADD COLUMN avatar_url VARCHAR;
```

### Chiến lược Zero-downtime (Không sập mạng khi update)

Khi bạn gõ lệnh `ALTER TABLE users DROP COLUMN phone;`, database sẽ bị khóa (Lock) và toàn bộ Server sập ngay lập tức. Hãy áp dụng **Backward-Compatible Transitions** (Chuyển đổi tương thích ngược).

Ví dụ: Đổi tên cột từ `name` sang `full_name`.
1. **Tuyệt đối KHÔNG gõ `ALTER TABLE users RENAME COLUMN name TO full_name;`** (Sập Server vì app cũ chưa cập nhật kịp sẽ báo lỗi cột name bị mất).
2. **Bước 1**: Thêm cột `full_name` mới (`ALTER TABLE ADD COLUMN`).
3. **Bước 2**: Sửa code App để ghi vào CẢ 2 cột (Dual write). Deploy App mới.
4. **Bước 3**: Chạy script Copy dữ liệu cũ từ `name` sang `full_name`.
5. **Bước 4**: Sửa code App chỉ đọc/ghi `full_name`. Deploy.
6. **Bước 5**: Xóa cột `name` cũ (`ALTER TABLE DROP COLUMN`). 

Nghe có vẻ dài dòng? Đúng vậy. Nhưng đó là cách DUY NHẤT để hệ thống tỷ đô sống sót. Thêm (Add) luôn an toàn. Sửa/Xóa (Modify/Delete) phải làm nhiều bước.

---

## 39.2 — CQRS: Chia rẽ để trị

Trong 90% hệ thống, lượng ĐỌC (Query) gấp 10-100 lần lượng GHI (Command). Cấu trúc bảng tối ưu cho Ghi (Chuẩn hóa thứ 3 - 3NF) lại cực kỳ chậm cho Đọc (Phải JOIN chằng chịt).

**CQRS (Command Query Responsibility Segregation)** giải quyết bằng cách: Cắt đôi Database!

1. **Write DB (PostgreSQL)**: Tối ưu cho Ghi. Các bảng được chuẩn hóa chặt chẽ (ACID).
2. **Read DB (Elasticsearch / Redis / Materialized View)**: Tối ưu cho Đọc. Dữ liệu được gộp sẵn (Denormalized) thành 1 cục JSON, client gọi phát ăn ngay không cần JOIN.

```rust
// Rust implementation sketch cho CQRS
trait AccountCommands {
    // Chỉ nhận lệnh cập nhật (Mutation), không trả về dữ liệu (trừ Ok/Err)
    fn deposit(&mut self, account_id: u64, amount: u64) -> Result<(), String>;
    fn transfer(&mut self, from: u64, to: u64, amount: u64) -> Result<(), String>;
}

trait AccountQueries {
    // Chỉ lấy dữ liệu (Read-only), tuyệt đối không sửa state
    fn get_summary(&self, account_id: u64) -> Option<AccountSummary>;
    fn get_monthly_report(&self, year: u32, month: u32) -> MonthlyReport;
}
```

Vấn đề duy nhất của CQRS là **Eventual Consistency** (Nhất quán trễ). Khi Write DB cập nhật xong, phải mất 1-2 giây để đồng bộ (sync) sang Read DB. Người dùng vừa đổi Tên, F5 lại vẫn thấy Tên cũ. Bạn cần thiết kế UX khéo léo để che giấu sự delay này.

---

## 39.3 — Event Sourcing: Cỗ máy thời gian

Các hệ thống CRUD truyền thống mang bản chất **Destructive Update** (Ghi đè phá hủy). Khi số dư tài khoản đổi từ 10$ sang 50$, con số 10$ biến mất VĨNH VIỄN. Nếu có lỗi xảy ra, bạn không biết tại sao tài khoản lại có 50$.

**Event Sourcing** thay đổi hoàn toàn tư duy: ĐỪNG lưu trạng thái cuối cùng. Hãy lưu **TOÀN BỘ SỰ KIỆN** (Events) đã xảy ra dưới dạng Append-Only (Chỉ thêm vào cuối).

```sql
-- Thay vì bảng `accounts` có cột `balance`
CREATE TABLE events (
    stream_id VARCHAR(100),  -- Ví dụ: "acc_123"
    event_type VARCHAR(50),  -- Ví dụ: "MoneyDeposited"
    data JSONB,              -- Ví dụ: {"amount": 40}
    created_at TIMESTAMP
);
```

Để biết số dư hiện tại của `acc_123`, hệ thống sẽ lấy tất cả Events của `acc_123` và **Replay** (Chiếu lại từ đầu).
`0$ (Khởi tạo) + 10$ (Deposit) + 40$ (Deposit) = 50$`.

Trong Rust, Fold (Reduce) là công cụ hoàn hảo để tính toán Event Sourcing:

```rust
enum AccountEvent {
    Created { id: String },
    Deposited { amount: u32 },
    Withdrawn { amount: u32 },
}

#[derive(Default, Debug)]
struct AccountState {
    balance: u32,
}

// Pure function: (State, Event) -> NewState
fn apply_event(state: AccountState, event: &AccountEvent) -> AccountState {
    match event {
        AccountEvent::Created { .. } => state,
        AccountEvent::Deposited { amount } => AccountState { balance: state.balance + amount },
        AccountEvent::Withdrawn { amount } => AccountState { balance: state.balance - amount },
    }
}

// Lấy danh sách events từ DB và tính toán
fn rebuild_state(events: &[AccountEvent]) -> AccountState {
    events.iter().fold(AccountState::default(), |state, event| apply_event(state, event))
}
```

Nhờ Event Sourcing, ngân hàng có thể biết chính xác số dư của bạn vào lúc 2h chiều ngày 1/1/2023 bằng cách chỉ Replay các event trước thời điểm đó.

---

## 39.4 — Caching Patterns

Để tối ưu hóa Đọc, Cache là vũ khí mạnh nhất. Có 3 chiến lược chính:

### 1. Read-Through (Phổ biến nhất)
App kiểm tra Cache (Redis). Nếu CÓ (Hit) -> Trả về ngay. Nếu KHÔNG (Miss) -> App chọc vào DB -> Lưu vào Cache -> Trả về.
- *Ưu điểm*: Đơn giản, logic nằm hết ở App.
- *Lỗi hay gặp*: **Cache Stampede** (Hàng ngàn user cùng Miss cache 1 lúc, lao vào DB làm sập DB).

### 2. Write-Through
Mỗi khi App ghi dữ liệu vào DB, App đồng bộ ghi luôn vào Cache. 
- *Ưu điểm*: Cache LUÔN MỚI. Gần như không bao giờ có Cache Miss.
- *Nhược điểm*: Ghi chậm hơn (Phải ghi 2 nơi).

### 3. Write-Behind (Dùng cho Ghi số lượng khủng)
Thay vì ghi thẳng vào DB, App chỉ ghi vào Cache/Memory. Sau đó có 1 background job sẽ gom (batch) hàng ngàn record từ Cache ghi xuống DB 1 lần.
- *Ưu điểm*: Tốc độ Ghi khủng khiếp (Phù hợp đếm lượt View Youtube, đếm Like).
- *Nhược điểm*: Nếu Redis sập, dữ liệu chưa kịp flush xuống DB sẽ mất sạch!

---

## 39.5 — Quyết định NoSQL

Đừng dùng MongoDB chỉ vì "Viết JSON cho nhanh". Mặc định, **LUÔN BẮT ĐẦU VỚI POSTGRESQL**. Chỉ dùng NoSQL khi SQL thực sự bộc lộ giới hạn:

| Hệ quản trị | Loại NoSQL | Khi nào nên dùng? | Khi nào KHÔNG dùng? |
|-------------|-------------|-------------------|---------------------|
| **Redis** | Key-Value | Caching siêu tốc, Rate Limiting, Đếm số (Counters). | Lưu dữ liệu bền vững, Query phức tạp. |
| **MongoDB** | Document | Dữ liệu dạng lồng ghép (Cây/JSON), Schema linh hoạt không xác định. | Cần JOIN nhiều bảng. Cần Transaction phức tạp (ACID). |
| **ScyllaDB** | Column | Ghi cực kỳ khổng lồ (Metrics, IoT). Tốc độ ghi bàn thờ. | Đọc dữ liệu linh tinh, Query có WHERE đa dạng. |
| **Neo4j** | Graph | Phân tích quan hệ (Mạng xã hội X quen Y, AI Recommendation). | Lưu dữ liệu bảng biểu bình thường. |

---

## ✅ Checkpoint 39

1. Vì sao "cache invalidation" được gọi là một trong hai bài toán khó nhất?
2. Event store append-only. Vậy làm sao sửa một event ghi sai?
3. `sqlx` kiểm tra query lúc biên dịch. Điều đó đòi hỏi gì khi chạy CI?

<details>
<summary>Đáp án</summary>

1. Vì không có tín hiệu nào cho biết dữ liệu đã cũ — bạn phải tự suy ra từ mọi đường ghi có thể. Bỏ sót một đường là cache sai vĩnh viễn, mà lại không có lỗi nào nổ ra.
2. **Không sửa.** Bạn ghi thêm một event bù trừ (`OrderAmountCorrected`). Lịch sử là bất biến — đó chính là tài sản của Event Sourcing, không phải hạn chế.
3. Cần một database thật lúc biên dịch, hoặc file `.sqlx/` (offline mode) được commit. Trong CI, chạy `cargo sqlx prepare --check` để chắc chắn cache offline còn khớp với schema.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Viết migration `sqlx` thêm cột `version` vào bảng `orders`, kèm cả bản `down`.

**Bài 2 (15 phút).** Cài cache-aside với `redis-rs`: đọc cache → miss thì đọc DB → ghi lại kèm TTL có jitter.

**Bài 3 (25 phút).** Cài event store tối giản: bảng append-only + hàm `rebuild(aggregate_id) -> Order` dùng `fold`. Thêm snapshot mỗi 100 event.

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| `sqlx` không biên dịch được trong CI | Không có DATABASE_URL, thiếu cache offline | `cargo sqlx prepare`, commit thư mục `.sqlx/` |
| Deadlock trong connection pool | Giữ connection trong lúc chờ connection khác | Không lồng lời gọi cần pool; lấy connection sớm, trả sớm |
| Redis timeout khi tải cao | Pool quá nhỏ | Tăng pool; dùng pipelining cho thao tác theo lô |
| Rebuild event ngày càng chậm | Không có snapshot | Snapshot định kỳ, replay từ snapshot gần nhất |
| Migration chạy hai lần trên nhiều instance | Không có khoá | Dùng advisory lock của PostgreSQL quanh bước migrate |

## Tóm tắt

- ✅ **Zero-Downtime Migration**: Thêm cột là an toàn. Xóa/Đổi tên cột phải làm qua nhiều bước trung gian.
- ✅ **CQRS**: Cắt đôi hệ thống. Dùng RDBMS cho Write, và Elastic/Redis/View cho Read.
- ✅ **Event Sourcing**: Lưu History thay vì ghi đè State. Cực mạnh kết hợp với `fold` trong Functional Programming.
- ✅ **Caching**: Read-through là mặc định. Chú ý nguy cơ Cache Stampede.
- ✅ **NoSQL**: Dùng khi và chỉ khi PostgreSQL đạt giới hạn vật lý về scale.

## Tiếp theo

Dữ liệu đã được lưu trữ, nhưng chúng có an toàn không? Hãy bước vào chiến trường khốc liệt nhất của Backend Engineer trong **Chapter 40: Security Essentials**.
