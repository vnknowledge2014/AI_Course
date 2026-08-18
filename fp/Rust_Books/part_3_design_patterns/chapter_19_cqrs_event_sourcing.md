# Chapter 19 — CQRS & Event Sourcing

> **Bạn sẽ học được**:
> - **CQRS**: Phân thân thuật — Tách riêng Read model (Đọc dữ liệu) và Write model (Ghi dữ liệu) để tăng tốc độ.
> - **Event Sourcing**: Cuốn băng thời gian — Lưu lại Lịch sử (Events) thay vì kết quả hiện tại, và khôi phục (rebuild) bằng `fold`.
> - Cách tạo **Event store**, **Projections**, và **Snapshots**.
> - Tại sao Event Sourcing + FP lại là Cặp Đôi Hoàn Hảo.
> - Khi nào dùng, khi nào KHÔNG nên dùng (Tránh dùng dao mổ trâu giết gà).
>
> **Yêu cầu trước**: Chapter 12 (Immutability), Chapter 14 (Enums), Chapter 18 (Command pattern).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn sẽ thấu hiểu sâu sắc kiến trúc Event-Driven (Hướng sự kiện) — nền tảng tối quan trọng cho DDD ở Phần IV.

---

## 19.1 — CQRS: Hai Cuốn Sổ Của Quán Cà Phê

Hãy tưởng tượng bạn mở một quán cà phê. Ban đầu nhỏ, bạn dùng **một cuốn sổ duy nhất**: ghi đơn hàng, tính tiền, xem báo cáo doanh thu, kiểm tra tồn kho — tất cả nằm chung một chỗ. Khi quán chỉ có 10 khách/ngày thì không sao. Nhưng khi lên 200 đơn/ngày, cuốn sổ trở thành nút thắt cổ chai: Nhân viên thu ngân muốn ghi đơn mới phải đứng chờ Quản lý đang lật sổ xem báo cáo, và ngược lại.

Giải pháp rất tự nhiên: **Tách thành 2 cuốn sổ**!
- Cuốn 1 (Ghi): Chuyên ghi đơn hàng nháp, chốt đơn (Tối ưu cho tốc độ Ghi).
- Cuốn 2 (Đọc): Chuyên báo cáo, hiển thị menu (Tối ưu cho tốc độ Đọc).
Cuốn 1 sẽ liên tục cập nhật dữ liệu sang Cuốn 2. Đơn giản vậy thôi, nhưng hiệu suất tăng gấp ngàn lần.

CQRS (Command Query Responsibility Segregation) chính là lấy ý tưởng đó làm nền tảng.

### Vấn đề: Thập Cẩm 1 Model

Trong code truyền thống, "Cuốn sổ duy nhất" trông như thế này:

```text
Truyền thống (CRUD):
┌──────────────────────┐
│       Order          │  ← Cùng 1 struct nhồi nhét:
│  - create()          │     đọc (query)
│  - update()          │     ghi (command)
│  - get_summary()     │     validation
│  - get_report()      │     reporting
└──────────────────────┘
   → Struct phình to, rối rắm, và chậm chạp!
```

Khi bạn tối ưu để Đọc nhanh (vd: tạo Cache), nó làm việc Ghi chậm lại (vì phải xóa Cache). Và ngược lại.

### Giải pháp: CQRS — Command (Ghi) và Query (Đọc) tách rời

CQRS giải quyết bằng cách chặt đứt chúng ra:

```text
CQRS:
┌─────────────────┐         ┌─────────────────┐
│  Write Model    │         │  Read Model     │
│  (Commands)     │         │  (Queries)      │
│                 │ ──────→ │                 │
│  - PlaceOrder   │ events  │  - OrderSummary │
│  - CancelOrder  │         │  - OrderReport  │
└─────────────────┘         └─────────────────┘
   Tối ưu cho sự                 Tối ưu cho
   NHẤT QUÁN                     TỐC ĐỘ (HIỂU XUẤT)
```

Trong Rust, ta sẽ dùng **Enum** để làm các Lệnh Ghi (Commands), và **Struct** để làm Dữ liệu Đọc (Views).

### Thiết kế Write Model (Ghi)

```rust
// filename: src/main.rs

// ═══════════════════════════════════════
// COMMANDS — Write side (Yêu cầu làm gì đó)
// ═══════════════════════════════════════
#[derive(Debug)]
enum OrderCommand {
    Create { customer: String },
    AddItem { name: String, price: u32, quantity: u32 },
    Submit,
}

// ═══════════════════════════════════════
// WRITE MODEL — Xử lý Commands
// ═══════════════════════════════════════
#[derive(Debug, Clone)]
struct Order {
    customer: String,
    items: Vec<(String, u32, u32)>, // name, price, qty
    status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Draft, Submitted }

impl Order {
    fn new(customer: &str) -> Self {
        Order { customer: customer.to_string(), items: vec![], status: OrderStatus::Draft }
    }

    // Nhận một lệnh, và trả về một Order MỚI (Functional Update)
    fn handle(self, cmd: OrderCommand) -> Result<Self, String> {
        match cmd {
            OrderCommand::Create { customer } => Ok(Order::new(&customer)),
            OrderCommand::AddItem { name, price, quantity } => {
                if self.status != OrderStatus::Draft {
                    return Err("Chỉ được thêm món khi đơn hàng đang nháp".into());
                }
                let mut items = self.items;
                items.push((name, price, quantity));
                Ok(Order { items, ..self })
            }
            OrderCommand::Submit => {
                if self.items.is_empty() {
                    return Err("Không thể chốt đơn rỗng".into());
                }
                Ok(Order { status: OrderStatus::Submitted, ..self })
            }
        }
    }
}
```

Chú ý: `handle()` nhận vào `self` và sinh ra Order mới. Đó là **Immutability (Bất biến)** mà ta đã học ở Chapter 12!

### Thiết kế Read Model (Đọc)

Trái ngược với việc Ghi, Read Model chỉ chuyên trả về dữ liệu nhanh nhất có thể.

```rust
// ═══════════════════════════════════════
// QUERIES — Read side (Chỉ để Hiển thị)
// ═══════════════════════════════════════
#[derive(Debug)]
struct OrderSummary {
    customer: String,
    total: u32,
    status: String,
}

impl Order {
    // READ MODEL — Rút trích thông tin từ Write model
    fn summary(&self) -> OrderSummary {
        OrderSummary {
            customer: self.customer.clone(),
            total: self.items.iter().map(|(_, p, q)| p * q).sum(),
            status: format!("{:?}", self.status),
        }
    }
}
```

Và ráp chúng lại với nhau:

```rust
fn main() {
    let order = Order::new("Minh");

    // Command (Write)
    let order = order.handle(OrderCommand::AddItem {
        name: "Coffee".into(), price: 35_000, quantity: 2
    }).unwrap();

    // Query (Read)
    println!("Trạng thái: {:?}", order.summary());

    // Command (Write)
    let order = order.handle(OrderCommand::Submit).unwrap();
    println!("Đã chốt: {:?}", order.summary());
}
```

Nhờ chia tách, bạn có thể thêm 10 loại Report (Báo cáo) mới mà chẳng đụng chạm gì đến Code kiểm duyệt Đơn hàng!

---

## ✅ Checkpoint 19.1

> Ghi nhớ:
> 1. **CQRS** = Tách Nhánh Ghi (Command) và Nhánh Đọc (Query) riêng biệt.
> 2. Write model tối ưu cho tính Nhất Quán (Validation).
> 3. Read model tối ưu cho Hiệu Năng và Hiển thị.

---

## 19.2 — Event Sourcing: Cuộn Phim Lịch Sử

Bạn đã tách được Đọc và Ghi. Câu hỏi tiếp theo: **Write Model sẽ lưu dữ liệu xuống Database như thế nào?**

Hầu hết mọi người dùng CRUD — Tức là mỗi lần có thay đổi, ta **Ghi Đè (Update)** lên trạng thái cũ. Ví dụ: Ngân hàng có 1 triệu, bạn rút 200 nghìn → Cập nhật số dư = 800,000. Xong. Chuyện gì đã xảy ra trong quá khứ? Biến mất vĩnh viễn!

Nhưng Ngân Hàng Thật sự không làm vậy! Họ có một **Cuốn Sổ Cái (Ledger)**. Nó ghi lại TỪNG GIAO DỊCH: Nộp 1tr, Rút 200k. Số dư hiện tại chỉ là kết quả của việc CỘNG TẤT CẢ các giao dịch lại với nhau.

**Event Sourcing** là như thế: Thay vì chụp 1 tấm ảnh hiện tại (State), ta quay lại Cả Một Cuộn Phim (Events). Bạn có thể tua đi, tua lại, và xem số dư tại bất kỳ thời điểm nào trong lịch sử!

### Ý tưởng cốt lõi

```text
CRUD:          Account { balance: 500 }  ← Chỉ biết kết quả, mù tịt quá trình
Event Source:  [Mở(1000), Rút(300), Nạp(200), Rút(400)]
               → Cộng dồn (fold) → balance = 500  ← Biết MỌI THỨ đã xảy ra
```

### Events: Sự Thật Đã Diễn Ra (Dùng Enum)

Sự kiện là những thứ đã xảy ra ở thì quá khứ (Past tense), và không thể đảo ngược.

```rust
// filename: src/main.rs

// Sự kiện (Luôn dùng Thì Quá Khứ)
#[derive(Debug, Clone)]
enum AccountEvent {
    Opened { id: u64, initial_balance: u64 },
    Deposited { amount: u64 },
    Withdrawn { amount: u64 },
}
```

### State: Kết quả Của Việc Tua Cuộn Phim

State không được lưu thẳng xuống DB. State được **Tính Toán (Fold)** từ Events.

```rust
#[derive(Debug, Clone)]
struct AccountState {
    id: u64,
    balance: u64,
}

impl AccountState {
    // Trạng thái Trống (Blank)
    fn initial() -> Self {
        AccountState { id: 0, balance: 0 }
    }

    // Tua (Apply) 1 sự kiện vào State
    fn apply(self, event: &AccountEvent) -> Self {
        match event {
            AccountEvent::Opened { id, initial_balance } => AccountState {
                id: *id, balance: *initial_balance,
            },
            AccountEvent::Deposited { amount } => AccountState {
                balance: self.balance + amount, ..self
            },
            AccountEvent::Withdrawn { amount } => AccountState {
                balance: self.balance - amount, ..self
            },
        }
    }
}
```

Bí quyết nằm ở hàm Rebuild (Phục dựng). Nó chính xác là hàm `fold` kinh điển mà bạn đã học ở Functional Programming!

```rust
// Phục dựng lại Hiện Thực
fn rebuild_state(events: &[AccountEvent]) -> AccountState {
    events.iter().fold(AccountState::initial(), |state, event| state.apply(event))
}

fn main() {
    let events = vec![
        AccountEvent::Opened { id: 1, initial_balance: 1_000_000 },
        AccountEvent::Deposited { amount: 500_000 },
        AccountEvent::Withdrawn { amount: 200_000 },
    ];

    // Xem số dư tại thời điểm cuối
    let current = rebuild_state(&events);
    println!("Số dư hiện tại: {}đ", current.balance); // 1,300,000

    // Tua ngược thời gian (Time Travel)
    println!("Số dư sau khi Open: {}đ", rebuild_state(&events[..1]).balance);
}
```

> **💡 Event Sourcing + FP = Perfect Match**: Events = Immutable Data. State = Fold(Events). Apply = Pure Function. Quá đẹp!

---

## 19.3 — Từ Lệnh biến thành Sự Kiện (Command → Event)

Nhiều người nghĩ lầm: Cứ gửi Lệnh là sẽ có Sự Kiện. Không phải!

Khi khách bảo "Tôi muốn rút 5 triệu" — Đó là **Command (Lệnh/Yêu cầu)**. Nó có thể bị Ngân hàng TỪ CHỐI (Vì hết tiền, vì bị khóa thẻ).
Chỉ khi nào Ngân hàng kiểm duyệt (Validate) thành công, họ mới tạo ra **Event (Sự kiện "Đã rút 5 triệu")**.

Đây chính là lớp **Validation Layer**.

```rust
// filename: src/main.rs

#[derive(Debug)]
enum BankCommand { Withdraw { amount: u64 } }

#[derive(Debug, Clone)]
enum BankEvent { Withdrawn { amount: u64 } }

#[derive(Debug, Clone)]
struct BankState { balance: u64 }

// Trạm Gác (Command Handler): Kiểm duyệt Command, Đẻ ra Event
fn handle_command(state: &BankState, cmd: &BankCommand) -> Result<Vec<BankEvent>, String> {
    match cmd {
        BankCommand::Withdraw { amount } => {
            if *amount > state.balance {
                Err(format!("Không đủ tiền. Có: {}, Cần: {}", state.balance, amount))
            } else {
                Ok(vec![BankEvent::Withdrawn { amount: *amount }])
            }
        }
    }
}

// Ứng dụng Event vào State
fn apply_event(state: BankState, event: &BankEvent) -> BankState {
    match event {
        BankEvent::Withdrawn { amount } => BankState { balance: state.balance - amount },
    }
}
```

Cùng chạy thử quá trình Khách hàng thực hiện Lệnh:

```rust
fn main() {
    let mut state = BankState { balance: 500 }; // Đang có 500đ
    let mut history = vec![];

    let commands = vec![
        BankCommand::Withdraw { amount: 200 }, // OK
        BankCommand::Withdraw { amount: 999 }, // TỪ CHỐI!
    ];

    for cmd in commands {
        match handle_command(&state, &cmd) {
            Ok(events) => {
                println!("✅ Hợp lệ: {:?}", cmd);
                // Lưu vào sổ Lịch sử, và cập nhật số dư mới
                for e in events {
                    state = apply_event(state, &e);
                    history.push(e);
                }
            }
            Err(err) => println!("❌ Bị từ chối: {} (Lệnh: {:?})", err, cmd),
        }
    }
    
    println!("Số dư cuối: {}", state.balance);
}
```

---

## 19.4 — Projections (Mỗi người một góc nhìn)

Bạn đã có Sổ Lịch sử. Bạn có thể Rebuild lại Số dư. 
Nhưng nếu Kế toán muốn xem Doanh thu? Còn Trưởng kho muốn xem Hàng Tồn?

CQRS nói rằng: **Hãy tạo nhiều Góc Nhìn (Projections) khác nhau từ CÙNG 1 nguồn Lịch sử!**
Giống như cùng một trận đá bóng (Lịch sử Event): Bình luận viên nhìn vào Chiến thuật, Khán giả nhìn vào Bàn thắng, Trọng tài nhìn vào Phạm lỗi.

```rust
// filename: src/main.rs
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ShopEvent {
    ProductAdded { name: String, qty: u32 },
    ProductSold { name: String, revenue: u32 },
}

// Góc nhìn của TRƯỞNG KHO (Chỉ quan tâm Hàng hóa)
#[derive(Debug, Default)]
struct InventoryView {
    stock: HashMap<String, u32>,
}
impl InventoryView {
    fn apply(&mut self, event: &ShopEvent) {
        match event {
            ShopEvent::ProductAdded { name, qty } => *self.stock.entry(name.clone()).or_insert(0) += qty,
            ShopEvent::ProductSold { name, .. } => *self.stock.entry(name.clone()).or_insert(0) -= 1,
        }
    }
}

// Góc nhìn của KẾ TOÁN (Chỉ quan tâm Tiền)
#[derive(Debug, Default)]
struct RevenueView {
    total: u32,
}
impl RevenueView {
    fn apply(&mut self, event: &ShopEvent) {
        if let ShopEvent::ProductSold { revenue, .. } = event {
            self.total += revenue;
        }
    }
}
```

Hãy cho cả 2 ngồi xem cùng 1 cuộn phim:

```rust
fn main() {
    let events = vec![
        ShopEvent::ProductAdded { name: "Macbook".into(), qty: 10 },
        ShopEvent::ProductSold { name: "Macbook".into(), revenue: 3000 },
    ];

    let mut inventory = InventoryView::default();
    let mut revenue = RevenueView::default();

    for e in &events {
        inventory.apply(e); // Kho tự xem
        revenue.apply(e);   // Kế toán tự xem
    }

    println!("Kho: {:?}", inventory.stock); // Còn 9 cái
    println!("Doanh thu: {}", revenue.total); // 3000
}
```

> **💡 Tinh Túy**: Bạn có thể thêm 100 cái View mới ở tương lai, và chỉ cần tua lại (replay) cuộn phim là View đó sẽ tự Đầy dữ liệu! Không cần thiết kế lại Database!

---

## 19.5 — Snapshots: Điểm Nhớ Game (Tối ưu Hiệu suất)

Lý thuyết thì hay, nhưng thực tế: Nếu ngân hàng có 500,000 giao dịch. Chẳng lẽ mỗi lần lấy số dư, ta phải Tua lại 500 ngàn Event từ đầu? Nó sẽ làm cháy Server!

Giải pháp hệt như Cắm Thẻ Nhớ chơi Game: **Save Game (Snapshot)**.
Cứ sau 1000 Event, ta lưu (Save) lại State một lần. 
Khi cần Load, ta tìm bản Save gần nhất, và chỉ Tua tiếp những đoạn Event ngắn ngủn đằng sau bản Save đó.

```rust
// filename: src/main.rs

// Điểm Nhớ Game
#[derive(Debug, Clone)]
struct Snapshot<S> {
    state: S,
    version: usize,  // Lưu lúc event thứ mấy?
}

// (Giả sử ta dùng Counter State có hàm Apply đơn giản)
// ...

// Phục dựng nhưng Có Thẻ Nhớ
fn rebuild_with_snapshot<S, E, F>(
    events: &[E],
    snapshot: Option<&Snapshot<S>>,
    initial_state: S,
    apply: F,
) -> S 
where 
    S: Clone,
    F: Fn(S, &E) -> S 
{
    // Tìm điểm bắt đầu
    let (base_state, start_index) = match snapshot {
        Some(snap) => (snap.state.clone(), snap.version),
        None => (initial_state, 0),
    };

    // Chỉ Fold những đoạn sự kiện SAU bản Save
    events[start_index..].iter().fold(base_state, |s, e| apply(s, e))
}
```

Nhờ Snapshot, thay vì chạy 500,000 vòng lặp, ta chỉ chạy cỡ 100 vòng lặp. Nhanh như chớp!

---

## 19.6 — Khi nào dùng Event Sourcing?

### ✅ Nên dùng khi

| Trường hợp | Lý do |
|----------|-------|
| Bắt buộc có **Nhật ký Kiểm Toán (Audit trail)** | Mọi thay đổi tài chính, y tế đều được ghi lại vĩnh viễn |
| Tính năng **Undo/Redo** | Có thể tua ngược lịch sử dễ dàng |
| Gỡ lỗi quay ngược thời gian (Time travel debugging) | Khôi phục hệ thống tại đúng 13:02 PM ngày hôm qua |
| Cần quá nhiều Báo cáo/View đa dạng | 1 Event Stream đẻ ra 100 Projections |

### ❌ Tránh xa khi (Anti-patterns)

| Trường hợp | Lý do |
|----------|-------|
| CRUD Ứng dụng đơn giản | Dùng Dao Mổ Trâu để cắt Cà Chua (Over-engineering) |
| Quyền riêng tư (Luật GDPR) | Luật bắt XÓA dữ liệu người dùng, nhưng Event là BẤT BIẾN (Rất nhức đầu xử lý) |
| Team chưa có kinh nghiệm | Đường cong học tập (Learning curve) vô cùng dốc |

---

## 🏋️ Bài tập

**Bài 1** (15 phút): Giỏ hàng Siêu Thị

Viết Event Sourcing cho một cái Giỏ Hàng: `ItemAdded`, `ItemRemoved`, `CartCleared`. 
Viết hàm `apply(state, event)` và rebuild state. Cuối cùng tính tổng tiền.

<details><summary>✅ Gợi ý Lời giải</summary>

Dùng `HashMap<String, u32>` (Tên món -> Số tiền). Hàm apply dùng `match` để kiểm tra event. Thêm thì Insert, Xóa thì Remove, Xóa Sạch thì `.clear()`.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| Events quá nhiều → Rebuild siêu chậm | Thiếu Thẻ Nhớ | Thêm Snapshots mỗi 1000 events |
| Khó lấy dữ liệu chéo (Join) | Do dữ liệu nằm ở Lịch Sử rải rác | Tạo Projection dạng Denormalized (Gộp sẵn thành Table) để Query cho dễ. |

---

## Tóm tắt

- ✅ **CQRS**: Tách bạch Lệnh (Ghi) và Lệnh Query (Đọc) để tối ưu hóa riêng biệt.
- ✅ **Event Sourcing**: Lưu băng Ghi Hình (Sự kiện) thay vì Kết Quả. Lấy kết quả bằng Phép màu `Fold`.
- ✅ **Command → Event**: Command là Lời Yêu Cầu, Event là Sự Thật Đã Xảy Ra.
- ✅ **Projections & Snapshots**: Cách tạo vô số View từ 1 băng hình, và Tối ưu hóa bằng Thẻ Nhớ Save Game.

---

## 🎉 Kết thúc Part III!

Bạn đã học **Design Patterns qua góc nhìn FP** — GoF patterns đơn giản hơn với closures/enums/traits, và CQRS+Event Sourcing cho kiến trúc event-driven đỉnh cao.

## Tiếp theo

Đã đến lúc áp dụng MỌI THỨ bạn đã học vào Thế Giới Thực.
→ **Part IV: Domain-Driven Design with Rust** bắt đầu! 
Chapter 20: **Introduction to DDD** — Ubiquitous Language, Bounded Contexts, Event Storming. Hãy chuẩn bị tinh thần!
