# Chapter 21 — Functional Architecture

> **Bạn sẽ học được**:
> - **Onion Architecture** — IO ở ngoài rìa, Pure Logic cốt lõi ở trung tâm
> - **Ports & Adapters** (Hexagonal) — Domain không bị phụ thuộc vào Cơ sở dữ liệu (Database) hay HTTP
> - Sử dụng Rust module system để **ép buộc** tuân thủ Kiến trúc
> - **Dependency Injection bằng Traits** — Cắm và rút các thành phần dễ dàng mà không cần Framework phức tạp
> - Cấu trúc thư mục chuẩn chỉnh cho một dự án DDD (Domain-Driven Design)
>
> **Yêu cầu trước**: Chapter 12 (Purity), Chapter 16 (Traits), Chapter 20 (DDD Intro).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn sẽ biết cách tổ chức một dự án Rust theo Clean Architecture — siêu dễ test, dễ bảo trì, và tập trung vào Nghiệp vụ (Domain-focused).

---

## 21.1 — Onion Architecture: Pure Core, IO Shell

### Quả trứng và Kiến trúc phần mềm

Hãy nghĩ về một quả trứng. 
- **Lòng đỏ** ở trung tâm — chứa mọi chất dinh dưỡng quý giá nhất, được bảo vệ kỹ lưỡng.
- **Lòng trắng** bao quanh — đóng vai trò kết nối, vững chắc nhưng có thể thay thế.
- **Vỏ** ở ngoài cùng — trực tiếp tiếp xúc với thế giới bên ngoài, có thể bị vỡ và dễ dàng thay vỏ mới.

**Onion Architecture (Kiến trúc củ hành)** chính là quả trứng dành cho phần mềm:
- **Domain (lòng đỏ)** = Logic nghiệp vụ thuần túy (Pure business logic), không biết gì về Database, HTTP, hay Email.
- **Application (lòng trắng)** = Người điều phối (Orchestration), gọi Domain xử lý rồi gọi IO để lưu lại.
- **Infrastructure (vỏ)** = Nơi thực thi IO thực tế: PostgreSQL, Redis, SendGrid, REST API.

Tại sao mô hình này lại quan trọng? Vì khi sếp yêu cầu đổi Database từ PostgreSQL sang MongoDB, bạn chỉ cần thay **lớp Vỏ** — Lòng đỏ (Logic cốt lõi) không thay đổi một dòng code nào! 
Khi viết Unit Test, bạn chỉ test Lòng đỏ **mà không cần cài Database** — chỉ cần truyền Input vào và lấy Output ra (Pure functions).

Nhưng để hiểu tại sao cần Kiến trúc, hãy nhìn một đoạn code **không có** Kiến trúc:

```rust
// ❌ Đoạn code trộn lẫn Logic và IO (Spaghetti Code)
fn handler() {
    let input = read_from_http();     // IO
    validate(&input);                 // Logic
    save_to_db(&input);               // IO
    calculate_tax(&input);            // Logic
    send_email();                     // IO
    update_cache();                   // IO
}
```

Logic và IO trộn lẫn như một bát Mì trộn — ăn thì ngon nhưng không gỡ sợi mì ra được. 
Bạn muốn test hàm `calculate_tax()` mà không vô tình gửi một email thật cho khách hàng? KHÔNG ĐƯỢC — vì `send_email()` dính chặt ngay bên dưới. 
Bạn muốn chạy thử ứng dụng Local mà không có Database? KHÔNG ĐƯỢC — vì `save_to_db()` đã bị "hardcode" (code cứng).

### Onion = Quả trứng (Nhưng lại gọi là Củ hành)

Onion Architecture tách các layer ra như các lớp vỏ hành: **Mỗi lớp chỉ biết lớp bên trong nó**, không bao giờ được phép nhìn ra ngoài. 

- Domain (lòng đỏ) không biết Application tồn tại. 
- Application không biết Infrastructure dùng PostgreSQL hay MongoDB. 
- Tất cả phụ thuộc mũi tên đều trỏ **VÀO TRONG**.

```text
┌─────────────────────────────────────┐
│         Infrastructure              │  ← IO: HTTP, DB, Email, Files
│  ┌───────────────────────────────┐  │
│  │       Application             │  │  ← Điều phối: Gọi domain, gọi IO
│  │  ┌─────────────────────────┐  │  │
│  │  │      Domain (CORE)      │  │  │  ← PURE: types, rules, validations
│  │  │   No IO. No deps.      │  │  │     Không biết DB, HTTP, Email
│  │  │   Only Rust types.     │  │  │     → TEST ĐƯỢC 100% NHANH CHÓNG
│  │  └─────────────────────────┘  │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

> **Quy tắc vàng:** Dependencies (Phụ thuộc) chỉ trỏ VÀO TRONG. Domain KHÔNG biết Infrastructure. Nếu bạn thấy file `domain.rs` có dùng lệnh `use sqlx::PgPool` — bạn đang phá vỡ Kiến trúc!

### Code minh họa: Chia cắt 3 lớp rõ ràng

Hãy nhìn thực tế: Một hệ thống Đăng ký người dùng (Register User) với 3 lớp tách biệt. Chú ý Lớp Domain không có bất kỳ lệnh IO nào — chỉ chứa các Struct và Hàm thuần túy (Pure functions):

#### Lớp 1: DOMAIN (Lòng đỏ)
```rust
// filename: src/main.rs

// ═══════════════════════════════════════════
// LAYER 1: DOMAIN (innermost) — PURE, no IO
// ═══════════════════════════════════════════
mod domain {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Email(String);

    impl Email {
        pub fn new(value: &str) -> Result<Self, String> {
            if value.contains('@') && value.len() >= 5 {
                Ok(Email(value.to_lowercase()))
            } else {
                Err(format!("Invalid email: {}", value))
            }
        }
        pub fn value(&self) -> &str { &self.0 }
    }

    #[derive(Debug, Clone)]
    pub struct User {
        pub id: u64,
        pub name: String,
        pub email: Email,
    }

    // Pure function — Logic nghiệp vụ kiểm tra tính hợp lệ
    pub fn validate_registration(name: &str, email: &str) -> Result<(String, Email), Vec<String>> {
        let mut errors = vec![];

        if name.trim().len() < 2 {
            errors.push("Name must be at least 2 characters".into());
        }

        let email = match Email::new(email) {
            Ok(e) => Some(e),
            Err(e) => { errors.push(e); None }
        };

        if errors.is_empty() {
            Ok((name.trim().to_string(), email.unwrap()))
        } else {
            Err(errors)
        }
    }
}
```

#### Lớp 2: APPLICATION (Lòng trắng)
Tiếp theo là Lớp Application. Nó sẽ định nghĩa các "Giao thức" (Traits) cần thiết, nhưng lại không tự mình thực thi chúng.

```rust
// ═══════════════════════════════════════════
// LAYER 2: APPLICATION — Orchestration
// ═══════════════════════════════════════════
mod application {
    use super::domain;

    // Port: Định nghĩa Trait cho Database (Domain KHÔNG biết Implementation là MySQL hay Redis)
    pub trait UserRepository {
        fn next_id(&self) -> u64;
        fn save(&mut self, user: &domain::User) -> Result<(), String>;
        fn find_by_email(&self, email: &domain::Email) -> Option<domain::User>;
    }

    // Port: Trait cho việc Gửi thông báo
    pub trait Notifier {
        fn send_welcome(&self, user: &domain::User) -> Result<(), String>;
    }

    // Use case: kịch bản Đăng ký người dùng
    pub fn register_user(
        name: &str,
        email: &str,
        repo: &mut dyn UserRepository, // Ai nhét repo gì vào cũng được!
        notifier: &dyn Notifier,       // Ai nhét notifier gì vào cũng được!
    ) -> Result<domain::User, Vec<String>> {
        // 1. Validate (Pure logic)
        let (name, email) = domain::validate_registration(name, email)?;

        // 2. Check duplicate (IO qua Trait)
        if repo.find_by_email(&email).is_some() {
            return Err(vec!["Email already registered".into()]);
        }

        // 3. Create & save
        let user = domain::User { id: repo.next_id(), name, email };
        repo.save(&user).map_err(|e| vec![e])?;

        // 4. Notify (Cố gắng gửi email, fail cũng không sao)
        let _ = notifier.send_welcome(&user); 

        Ok(user)
    }
}
```

#### Lớp 3: INFRASTRUCTURE (Lớp Vỏ)
Và cuối cùng là Lớp vỏ. Tại đây, ta viết code thực sự làm việc với Hệ điều hành, Ổ cứng, và Network. Trong ví dụ này, ta sẽ dùng Hashmap in-memory làm Database ảo và Console (in ra màn hình) làm Email ảo.

```rust
// ═══════════════════════════════════════════
// LAYER 3: INFRASTRUCTURE — IO implementations
// ═══════════════════════════════════════════
mod infrastructure {
    use super::{domain, application};
    use std::collections::HashMap;

    // In-memory Repo (Có thể thay thế bằng PostgreSQL, MongoDB,...)
    pub struct InMemoryUserRepo {
        users: HashMap<u64, domain::User>,
        counter: u64,
    }

    impl InMemoryUserRepo {
        pub fn new() -> Self {
            InMemoryUserRepo { users: HashMap::new(), counter: 0 }
        }
    }

    impl application::UserRepository for InMemoryUserRepo {
        fn next_id(&self) -> u64 { self.counter + 1 }

        fn save(&mut self, user: &domain::User) -> Result<(), String> {
            self.counter += 1;
            self.users.insert(user.id, user.clone());
            println!("  [DB] Saved user #{}", user.id);
            Ok(())
        }

        fn find_by_email(&self, email: &domain::Email) -> Option<domain::User> {
            self.users.values().find(|u| u.email == *email).cloned()
        }
    }

    // Console Notifier (Có thể thay thế bằng SendGrid, AWS SES,...)
    pub struct ConsoleNotifier;

    impl application::Notifier for ConsoleNotifier {
        fn send_welcome(&self, user: &domain::User) -> Result<(), String> {
            println!("  [EMAIL] Welcome {}! (sent to {})", user.name, user.email.value());
            Ok(())
        }
    }
}
```

Và ở tầng trên cùng (Main), ta ghép chúng lại với nhau (Dependency Injection).

```rust
fn main() {
    use application::register_user;

    // Khởi tạo các "Thiết bị cắm"
    let mut repo = infrastructure::InMemoryUserRepo::new();
    let notifier = infrastructure::ConsoleNotifier;

    // Happy path
    match register_user("Minh", "minh@email.com", &mut repo, &notifier) {
        Ok(user) => println!("✅ Registered: {:?}\n", user),
        Err(errors) => println!("❌ Failed: {:?}\n", errors),
    }

    // Báo lỗi: Trùng Email!
    match register_user("Lan", "minh@email.com", &mut repo, &notifier) {
        Ok(user) => println!("✅ Registered: {:?}\n", user),
        Err(errors) => println!("❌ Failed: {:?}\n", errors),
    }
}
```

Hãy đọc lại đoạn code trên và chú ý 3 điều tuyệt vời sau:

1. **Module `domain`** không có lệnh `use std::io`, không có `println!`, không có mạng mẽo gì sất. Hàm `validate_registration()` là Pure Function — nhập đầu vào thế nào thì luôn cho ra kết quả thế đó. Bạn viết Test 1000 lần vẫn chạy đúng cả 1000.
2. **Module `application`** định nghĩa ra các Trait (`UserRepository`, `Notifier`) nhưng **không implement chúng**. Nó chỉ nói: *"Tao cần một đứa biết lưu User và một đứa biết gửi Email"*. Còn "đứa đó" là MySQL hay Redis, là SendGrid hay Console, thì nó không thèm quan tâm.
3. **Module `infrastructure`** mới là nơi Implement Traits. Giả sử ngày mai công ty đổi từ `InMemoryUserRepo` sang `PostgresUserRepo`? Bạn chỉ cần viết Class mới và thay đúng 1 dòng ở hàm `main()`. Modules `domain` và `application` hoàn toàn mù tịt và không cần sửa 1 chữ nào!

---

## ✅ Checkpoint 21.1

> Ghi nhớ:
> 1. **Domain** = Thuần khiết (Pure), không IO, không Dependencies bên ngoài. Như lòng đỏ — Nơi chứa đựng Core Business.
> 2. **Application** = Kẻ điều phối. Gọi Domain logic và gọi IO qua các Hợp đồng Traits (Ports). Như lòng trắng — Kết nối và nâng đỡ.
> 3. **Infrastructure** = Vùng đất IO thực sự (DB, network, file, cache). Như lớp vỏ — Sẵn sàng đập bỏ và thay thế bất cứ lúc nào.
> 4. Phụ thuộc luôn trỏ **VÀO TRONG**: Infra → App → Domain. Tuyệt đối KHÔNG LÀM NGƯỢC LẠI.

---

## 21.2 — Ports & Adapters: Cổng kết nối và Ổ cắm

### Ổ cắm điện và thiết bị điện

Bạn biết cách cái ổ cắm điện trong nhà hoạt động chứ? **Ổ cắm (Port)** quy định hình dáng lỗ cắm (2 chấu tròn, 3 chấu dẹt). Còn cái Quạt, cái Đèn, cái Tủ lạnh chính là các **Thiết bị (Adapters)** — chúng được chế tạo sao cho nhét vừa vặn vào Ổ cắm. Bạn cắm cái gì vào cũng được, miễn là nó đúng chuẩn. Ổ cắm trên tường không bao giờ quan tâm việc nó đang cấp điện cho cái Quạt hay cái Đèn.

Trong lập trình Rust, **Trait chính là Port** (Lỗ cắm). Struct nào đi implement cái Trait đó thì gọi là **Adapter** (Thiết bị). 

Lớp Application sẽ định nghĩa các Traits: *"Tối nay tôi cần ăn một cái gì đó chứa Cơm"*.
Lớp Infrastructure sẽ cung cấp Adapter: *"Tôi là quán Cơm Sườn, tôi sẽ phục vụ anh"*.

### Code: Ports và Adapters thực tế

```rust
// filename: src/main.rs

// ═══════ PORTS (Traits) ═══════
// Domain/Application định nghĩa → Infrastructure cắm vào
trait OrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), String>;
}

trait PaymentGateway {
    fn charge(&self, amount: u32) -> Result<String, String>;
}

// ═══════ DOMAIN ═══════
#[derive(Debug, Clone)]
struct Order {
    id: u64,
    customer: String,
    total: u32,
}

// ═══════ APPLICATION ═══════
fn place_order(
    customer: &str,
    total: u32,
    repo: &mut dyn OrderRepository,    // Cắm Port Database
    payment: &dyn PaymentGateway,      // Cắm Port Payment
) -> Result<Order, String> {
    
    // Cà thẻ trước
    payment.charge(total)?;

    // Lưu DB sau
    let order = Order { id: 1, customer: customer.into(), total };
    repo.save(&order)?;

    Ok(order)
}
```

Và giờ là các Adapters để cắm vào hệ thống:

```rust
// ═══════ ADAPTERS (Infrastructure) ═══════

// Adapter 1: Database trên RAM (Dùng để Test hoặc chạy Local)
struct MemoryOrderRepo { orders: Vec<Order> }
impl OrderRepository for MemoryOrderRepo {
    fn save(&mut self, order: &Order) -> Result<(), String> {
        self.orders.push(order.clone());
        Ok(())
    }
}

// Adapter 2: Cổng thanh toán "Fake"
struct FakePayment;
impl PaymentGateway for FakePayment {
    fn charge(&self, amount: u32) -> Result<String, String> {
        println!("  [PAYMENT] Đã trừ ẢO {}đ", amount);
        Ok("PAY-001".into())
    }
}
```

### Lợi ích: Hoán đổi Adapters siêu tốc

Đây là quyền năng của Ports & Adapters: Bạn có thể thay đổi **toàn bộ** kho ứng dụng Infrastructure mà Lớp Core Domain không mảy may hay biết. Giống như việc bạn rút dây cắm của cái Đèn và thay bằng cái Quạt.

Thử hình dung:
- **Trên Production (Chạy Thật)**: `PostgresRepo` + `StripePayment` + `KafkaPublisher`
- **Lúc Chạy Test (Nhanh)**: `MemoryRepo` + `FakePayment` + `VecPublisher`
- **Trên Server Staging (Test Lỗi)**: `PostgresRepo` + `FakePayment` + `ConsolePublisher`

Chú ý: **Đoạn code Domain giống nhau 100%** trong cả 3 môi trường. Chỉ có các Adapter là bị thay thế. Kiến trúc này được mệnh danh là "Swappable Infrastructure".

---

## 21.3 — Cấu trúc thư mục (Project Structure) cho DDD

Nếu kiến trúc là Quả trứng, thì cấu trúc thư mục (Folder) của bạn phải **phản ánh** đúng các lớp vỏ đó. 
Một Lập trình viên mới vào công ty (Junior), khi mở cái source code lên, nhìn cấu trúc Folder là phải "Giác ngộ" ngay: *"À, code trong thư mục `domain/` là logic tính toán, còn vô `infrastructure/` là để fix cái lỗi kết nối Database!"*.

### Layout khuyến nghị

```text
my_app/
├── Cargo.toml
└── src/
    ├── main.rs                 # File gốc, đóng vai trò Cắm Dây Điện (Wiring)
    ├── domain/                 # PURE — Không IO, Không Dependency ngoài!
    │   ├── mod.rs
    │   ├── model.rs            # Struct Entity, Enum Value Object
    │   ├── events.rs           # Các Sự kiện hệ thống (Domain Events)
    │   └── rules.rs            # Quy tắc kinh doanh (Thuần Hàm)
    ├── application/            # KỊCH BẢN (Use cases) — Đứng ra điều phối
    │   ├── mod.rs
    │   ├── ports.rs            # Các Trait để hứng Adapter
    │   ├── register_user.rs    # Kịch bản "Đăng ký"
    │   └── place_order.rs      # Kịch bản "Mua hàng"
    └── infrastructure/         # IO Thực Sự — Giao tiếp với thế giới ngoài
        ├── mod.rs
        ├── persistence/
        │   ├── postgres_repo.rs
        │   └── in_memory_repo.rs
        ├── messaging/
        │   └── kafka_publisher.rs
        └── external/
            └── stripe_payment.rs
```

### Dùng Module Visibility để Thiết Quân Luật

Rust có một sức mạnh ẩn giấu: Nó cho phép bạn quản lý việc ai được dùng file nào (`pub` keyword).

```rust
// file: domain/mod.rs 
// Lớp Core KHÔNG BAO GIỜ import cái gì từ bên ngoài!
pub mod model;
pub mod rules;
// ❌ use crate::infrastructure;  // <--- LỖI! Dẹp ngay ý tưởng này!

// file: application/mod.rs 
// Lớp Application thì ĐƯỢC PHÉP import Domain
pub mod ports;
pub mod register_user;
use crate::domain;  // ✅ OK
// ❌ use crate::infrastructure;  // <--- KHÔNG ĐƯỢC PHÉP!

// file: infrastructure/mod.rs 
// Lớp vỏ bọc ngoài cùng thì ĐƯỢC QUYỀN nhìn thấy cả 2 lớp trong!
pub mod persistence;
use crate::domain;       // ✅ OK
use crate::application;  // ✅ OK — Phải có Application để implement Traits chứ!
```

---

## 21.4 — Testing Architecture

Và đây là lí do LỚN NHẤT, đáng tiền nhất để công ty bạn chuyển sang dùng Onion Architecture: **Sự Sung Sướng Khi Viết Test**.

- **Lòng đỏ (Domain)** là nơi chứa các Pure Functions. Khỏi cần Database, khỏi cần giả mạo (Mocking) gì mệt mỏi. Input thả vào, check Output chui ra. Chạy 1000 test case mất 0.1 giây.
- **Lòng trắng (Application)** thì cần phải test xem kịch bản (Orchestration) có gọi đúng hàm lưu xuống kho hay không. Ta chỉ cần nhét vài cái Fake Adapters (Giả lập) vào là xong.
- **Vỏ (Infrastructure)** thì cần Database thật (Postgres), chạy sẽ khá rùa bò. Nhưng may mắn là ở lớp này chỉ có lác đác vài Test Case kiểm tra xem câu lệnh SQL có gõ sai chính tả không.

### Bảng Chiến lược Viết Test

| Layer (Tầng) | Viết Test Kiểu Gì? | Có Mock không? | Tốc độ chạy |
|-------|-----------|-------|-------|
| **Domain** | Unit tests | ❌ Không cần! | ⚡ Cực nhanh |
| **Application** | Integration tests | ✅ Dùng Mock Ports | ⚡ Nhanh |
| **Infrastructure** | Integration tests | ❌ Xài DB thật | 🐢 Chậm hơn |
| **E2E Toàn Cục** | Chạy full app | ❌ Xài MỌI THỨ thật | 🐌 Chậm như rùa |

Dưới đây là ví dụ Test Lớp Domain "Siêu Cấp Nhanh":

```rust
// filename: src/main.rs

mod domain {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Money(u32);

    impl Money {
        pub fn new(amount: u32) -> Self { Money(amount) }
        pub fn value(&self) -> u32 { self.0 }

        pub fn apply_discount(&self, percent: u32) -> Result<Self, String> {
            if percent > 100 { return Err("Discount > 100%".into()); }
            Ok(Money(self.0 * (100 - percent) / 100))
        }

        pub fn add_tax(&self, rate_percent: u32) -> Self {
            Money(self.0 + self.0 * rate_percent / 100)
        }
    }

    // Domain rule: Pure function → Mời viết Test thỏa thích!
    pub fn calculate_total(items: &[(u32, u32)], discount: u32, tax: u32) -> Result<Money, String> {
        let subtotal: u32 = items.iter().map(|(price, qty)| price * qty).sum();
        let after_discount = Money::new(subtotal).apply_discount(discount)?;
        Ok(after_discount.add_tax(tax))
    }
}

// VIẾT TEST NGAY BÊN DƯỚI, CHẠY KHÔNG CẦN CÀI ĐẶT GÌ!
#[cfg(test)]
mod domain_tests {
    use super::domain::*;

    #[test]
    fn test_discount_10_percent() {
        let price = Money::new(100_000);
        assert_eq!(price.apply_discount(10).unwrap().value(), 90_000); // Ngọt sớt!
    }

    #[test]
    fn test_discount_qua_ho() {
        let price = Money::new(100_000);
        assert!(price.apply_discount(150).is_err()); // Sale 150% là công ty phá sản
    }

    #[test]
    fn test_tinh_tong_don_hang() {
        let items = vec![(50_000, 2), (30_000, 1)]; // = 130k
        let total = calculate_total(&items, 10, 8).unwrap();
        
        // Toán học lớp 5: 130,000 * 90% = 117,000. Lấy 117,000 + 8% VAT = 126,360
        assert_eq!(total.value(), 126_360);
    }
}
```

---

## 21.5 — Anti-patterns: Khi bản đồ vẽ sai

Kiến trúc tốt cần kỷ luật thép. Dưới đây là những Lỗi thường gặp nhất phá nát hệ thống Onion Architecture — Giống như đem nhà máy rác xây ngay giữa công viên trung tâm vậy!

### ❌ Anti-pattern 1: Domain import Infrastructure

```rust
// ❌ SAI: Domain đi làm thân với Database!
mod domain {
    use sqlx::PgPool;  // ← Lệnh mũi tên đi NGƯỢC!

    pub fn create_user(pool: &PgPool, name: &str) {
        // Domain logic bị trộn với Code Gọi SQL
    }
}
```
Khi Domain mà dám import `sqlx::PgPool`? Xong đời. Lòng đỏ của bạn đã bị tiêm thuốc độc — Từ nay bạn không thể test hàm này nếu cái máy tính không có cài đặt SQL Database. 

**Cách sửa**: Định nghĩa một Trait `UserRepository` ở phần Application, và truyền nó (truyền Reference) vào hàm của Domain.

### ❌ Anti-pattern 2: Anemic Domain Model (Người mẫu Thiếu máu)

Một Căn Bệnh trầm kha lây từ giới lập trình viên Java (thời kỳ cũ): Các file `models` chỉ chứa Data (Cấu trúc dữ liệu Struct), còn bao nhiêu Não (Hàm xử lý Logic) thì bị moi ra hết và ném tứ tung ở các file `services`. Giống như một cái xác vô hồn:

```rust
// ❌ SAI: Struct chỉ dùng làm TÚI ĐỰNG DỮ LIỆU
struct Order { total: u32, status: String }  // Không có tí trí tuệ nào

// Não xử lý rải rác ngoài đường phố
fn validate_order(o: &Order) -> bool { /* ... */ }
fn calculate_total(o: &Order) -> u32 { /* ... */ }
```
Cách này khiến `Order` không thể tự bảo vệ bản thân. Ai đó ở xa lắc xa lơ có thể cố tình nhảy vào sửa field `total = -50` mà hàm `validate_order` chẳng hay biết.

### ✅ Giải pháp: Rich Domain Model (Cấu trúc thông minh)

```rust
// ✅ ĐÚNG: Não (Behavior) và Thể xác (Data) sống trong cùng một Struct
struct Order { items: Vec<OrderLine>, status: OrderStatus }

impl Order {
    fn subtotal(&self) -> u32 { /* Tự tui tính cho tui! */ }
    fn apply_discount(&self, discount: Discount) -> Result<Self, DomainError> { /* ... */ }
    fn can_be_shipped(&self) -> bool { self.status == OrderStatus::Paid }
}
```
Kỹ thuật này gọi là Đóng gói (Encapsulation). Đơn hàng tự nó biết nó tính tổng như thế nào, và khi nào thì nó được quyền gửi đi.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Layer identification

Phân loại các function sau vào đúng Layer của nó (Domain / Application / Infrastructure):

```rust
fn validate_email(s: &str) -> Result<Email, Error> { ... }
fn save_to_postgres(user: &User) -> Result<(), Error> { ... }
fn register_user(name: &str, email: &str) -> Result<User, Error> { ... }
fn send_ses_email(to: &str, body: &str) -> Result<(), Error> { ... }
fn calculate_shipping(weight: f64) -> u32 { ... }
```

<details><summary>✅ Lời giải Bài 1</summary>

- `validate_email` → **Domain** (Bắt lỗi Logic thuần túy)
- `save_to_postgres` → **Infrastructure** (Chạm tới DB thật)
- `register_user` → **Application** (Kịch bản điều phối user)
- `send_ses_email` → **Infrastructure** (Gọi API mạng ngoài)
- `calculate_shipping` → **Domain** (Thuật toán định giá - Pure rule)

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| Domain lỡ dại Import Infrastructure | Mũi tên phụ thuộc cắm ngược đầu | Đẩy mã IO ra tạo thành Traits, đứng ở Core gọi ra ngoài. |
| "Khó xách các Traits đi lòng vòng quá!" | Manual DI (Dependency Injection) bằng tay khá mỏi | Bạn có thể gom cụm tất cả Trait vào một cục `AppContext Struct` rồi chuyền cho khỏe. |

---

## Tóm tắt

Chapter này dạy bạn cách **Xây dựng khung nhà** cho bộ môn kiến trúc DDD:

- ✅ **Onion Architecture**: Củ hành. Domain (lòng đỏ) → Application (Lòng trắng) → Infrastructure (Vỏ cứng). Phụ thuộc luôn trỏ vào giữa.
- ✅ **Ports** = Các ổ cắm chờ chực (Traits). **Adapters** = Thiết bị được cắm vào. Tháo ra cắm vào dễ dàng nên Test rất êm.
- ✅ **Quản lý Import**: Lớp bên trong KHÔNG ĐƯỢC PHÉP `use` (import) thư viện của lớp ngoài. Trình Biên Dịch (Compiler) sẽ tát bạn nếu bạn làm sai!
- ✅ **Testing chiến lược**: Cốt lõi bên trong phải Chạy Thật Nhanh (Pure functions, không Mocking).
- ✅ **Rich Domain Model**: Dữ liệu và Hàm tính toán sống chung với nhau trong 1 file Struct, chứ không rải rác giang hồ.

## Tiếp theo

Khung nhà (Architecture) đã lên! Giờ là lúc đi sắm **Nội thất (Domain Types)**.
→ Chapter 22: **Domain Modeling with Rust Types** — Bạn sẽ học cách khắc họa Value Objects, Entities, Aggregates, và bảo vệ trạng thái của ứng dụng bằng các Phép Màu Của Trình Biên Dịch (State Machines).
