# Chapter 21 — Functional Architecture

> **Bạn sẽ học được**:
> - **Onion Architecture** — IO ở rìa, pure domain logic ở core
> - **Ports & Adapters** (Hexagonal) — domain không phụ thuộc infrastructure
> - Rust module system **enforce** architectural boundaries
> - **Dependency Injection bằng traits** — thay thế DI containers
> - Cấu trúc project DDD hoàn chỉnh
>
> **Yêu cầu trước**: Chapter 12 (Purity), Chapter 16 (Traits), Chapter 20 (DDD Intro).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn cấu trúc project Rust theo kiến trúc clean — testable, maintainable, domain-focused.

---

## 21.1 — Onion Architecture: Pure Core, IO Shell

### Quả trứng và kiến trúc phần mềm

Hãy nghĩ về quả trứng. Lòng đỏ ở trung tâm — chứa mọi dinh dưỡng, được bảo vệ. Lòng trắng bao quanh — chắc chắn, nhưng có thể thay thế. Vỏ ở ngoài cùng — tiếp xúc thế giới bên ngoài, có thể vỡ và thay vỏ mới.

**Onion Architecture** là quả trứng cho phần mềm: **Domain** (lòng đỏ) = pure business logic, không biết gì về database, HTTP, email. **Application** (lòng trắng) = orchestration, gọi domain và IO. **Infrastructure** (vỏ) = IO thực tế: PostgreSQL, Redis, SendGrid.

Tại sao quan trọng? Vì khi bạn cần đổi database từ PostgreSQL sang MongoDB, bạn chỉ thay **vỏ** — lòng đỏ (domain logic) không đổi. Khi test, bạn test lòng đỏ **không cần database** — pure functions, input vào output ra, không side effects.

Nhưng để hiểu tại sao cần kiến trúc, hãy nhìn code **không có** kiến trúc:

```
❌ handler() {
    let input = read_from_http();     // IO
    validate(input);                   // Logic
    save_to_db(input);                // IO
    calculate_tax(input);             // Logic
    send_email();                     // IO
    update_cache();                   // IO
    log_audit();                      // IO
}
Logic và IO trộn lẫn như mì Sài Gòn — thơm ngon nhưng không tách được. Muốn test `calculate_tax()` mà không gửi email thật? Không được — vì `send_email()` nằm giữa dòng logic. Muốn chạy local mà không cần database? Không được — vì `save_to_db()` hardcoded liền kề `validate()`.

### Onion = Quả trứng (nhưng gọi là hành)

Onion Architecture tách các layer như lớp vỏ hành: **mỗi lớp chỉ biết lớp bên trong nó**, không bao giờ ngược lại. Domain (lòng đỏ) không biết Application tồn tại. Application không biết Infrastructure dùng PostgreSQL hay MongoDB. Tất cả phụ thuộc trỏ **vào trong**.
┌─────────────────────────────────────┐
│         Infrastructure              │  ← IO: HTTP, DB, Email, Files
│  ┌───────────────────────────────┐  │
│  │       Application             │  │  ← Orchestration: gọi domain, gọi IO
│  │  ┌─────────────────────────┐  │  │
│  │  │      Domain (CORE)      │  │  │  ← PURE: types, rules, validations
│  │  │   No IO. No deps.      │  │  │     Không biết DB, HTTP, Email
│  │  │   Only Rust types.     │  │  │     → TEST ĐƯỢC 100%
│  │  └─────────────────────────┘  │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘

Quy tắc vàng: **Dependencies chỉ trỏ VÀO TRONG**. Domain KHÔNG biết Infrastructure. Nếu bạn thấy `domain.rs` import `sqlx` — bạn đang phá kiến trúc.

### Code minh họa: 3 lớp rõ ràng

Hãy nhìn thực tế: một hệ thống đăng ký user với 3 layers tách biệt. Chú ý Domain layer không có bất kỳ IO nào — chỉ types và pure functions:

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

    // Pure function — domain rule
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

// ═══════════════════════════════════════════
// LAYER 2: APPLICATION — orchestration
// ═══════════════════════════════════════════
mod application {
    use super::domain;

    // Port: trait cho database (domain KHÔNG biết implementation)
    pub trait UserRepository {
        fn next_id(&self) -> u64;
        fn save(&mut self, user: &domain::User) -> Result<(), String>;
        fn find_by_email(&self, email: &domain::Email) -> Option<domain::User>;
    }

    // Port: trait cho notifications
    pub trait Notifier {
        fn send_welcome(&self, user: &domain::User) -> Result<(), String>;
    }

    // Use case: register user
    pub fn register_user(
        name: &str,
        email: &str,
        repo: &mut dyn UserRepository,
        notifier: &dyn Notifier,
    ) -> Result<domain::User, Vec<String>> {
        // 1. Validate (pure domain logic)
        let (name, email) = domain::validate_registration(name, email)?;

        // 2. Check duplicate (IO via trait)
        if repo.find_by_email(&email).is_some() {
            return Err(vec!["Email already registered".into()]);
        }

        // 3. Create & save
        let user = domain::User { id: repo.next_id(), name, email };
        repo.save(&user).map_err(|e| vec![e])?;

        // 4. Notify
        let _ = notifier.send_welcome(&user); // best-effort

        Ok(user)
    }
}

// ═══════════════════════════════════════════
// LAYER 3: INFRASTRUCTURE — IO implementations
// ═══════════════════════════════════════════
mod infrastructure {
    use super::{domain, application};
    use std::collections::HashMap;

    // In-memory repo (thay thế bằng PostgreSQL, MongoDB, etc.)
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

    // Console notifier (thay thế bằng SendGrid, SES, etc.)
    pub struct ConsoleNotifier;

    impl application::Notifier for ConsoleNotifier {
        fn send_welcome(&self, user: &domain::User) -> Result<(), String> {
            println!("  [EMAIL] Welcome {}! (sent to {})", user.name, user.email.value());
            Ok(())
        }
    }
}

fn main() {
    use application::register_user;

    let mut repo = infrastructure::InMemoryUserRepo::new();
    let notifier = infrastructure::ConsoleNotifier;

    // Happy path
    match register_user("Minh", "minh@email.com", &mut repo, &notifier) {
        Ok(user) => println!("✅ Registered: {:?}\n", user),
        Err(errors) => println!("❌ Failed: {:?}\n", errors),
    }

    // Duplicate email
    match register_user("Lan", "minh@email.com", &mut repo, &notifier) {
        Ok(user) => println!("✅ Registered: {:?}\n", user),
        Err(errors) => println!("❌ Failed: {:?}\n", errors),
    }

    // Validation error
    match register_user("M", "bad", &mut repo, &notifier) {
        Ok(user) => println!("✅ Registered: {:?}\n", user),
        Err(errors) => println!("❌ Failed: {:?}\n", errors),
    }
}
```

Hãy đọc lại đoạn code trên và chú ý 3 điều:

1. **Domain module** không có `use std::io`, không có `println!`, không có bất kỳ IO nào. `validate_registration()` là pure function — cùng input luôn cho cùng output. Test 1000 lần vẫn giống nhau.

2. **Application module** định nghĩa traits (`UserRepository`, `Notifier`) nhưng **không implement** chúng. Nó chỉ nói: "tôi cần AI ĐÓ biết save user và gửi email" — nhưng không quan tâm AI ĐÓ là PostgreSQL hay MongoDB, SendGrid hay console.

3. **Infrastructure module** implement traits bằng concrete technology. Đổi từ `InMemoryUserRepo` sang `PostgresUserRepo`? Chỉ cần thay 1 dòng trong `main()` — domain và application không biết, không cần sửa.

---

## ✅ Checkpoint 21.1

> Ghi nhớ:
> 1. **Domain** = pure, no IO, no deps. Chỉ types + business rules. Như lòng đỏ quả trứng — được bảo vệ
> 2. **Application** = orchestration. Gọi domain + IO qua traits (ports). Như lòng trắng — kết nối
> 3. **Infrastructure** = IO implementations (DB, email, cache). Như vỏ — thay được
> 4. Dependencies trỏ **vào trong**: Infra → App → Domain. KHÔNG ngược lại.

---

## 21.2 — Ports & Adapters: Traits = Ports

### Ổ cắm điện và thiết bị điện

Bạn biết ổ cắm điện chứ? Ổ cắm là **port** — nó định nghĩa hình dạng lỗ cắm. Thiết bị điện (quạt, đèn, tủ lạnh) là **adapters** — chúng implement interface của ổ cắm. Bạn có thể cắm bất kỳ thiết bị nào vào ổ cắm, miễn là nó đúng phích.

Trong Rust, **trait = port** (lỗ cắm). Struct implement trait = **adapter** (thiết bị). Application layer định nghĩa traits: "tôi cần thứ gì đó save được user". Infrastructure implement traits: "tôi là PostgreSQL, tôi save được user". Đổi PostgreSQL sang MongoDB? Thêm adapter mới, không sửa domain.

### Code: Ports và Adapters thực tế
// filename: src/main.rs

// ═══════ PORTS (traits) ═══════
// Domain/Application định nghĩa → Infrastructure implement

trait OrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), String>;
    fn find(&self, id: u64) -> Option<Order>;
    fn find_by_customer(&self, customer: &str) -> Vec<Order>;
}

trait PaymentGateway {
    fn charge(&self, amount: u32, method: &str) -> Result<String, String>;
    fn refund(&self, payment_id: &str) -> Result<(), String>;
}

trait EventPublisher {
    fn publish(&self, event: &DomainEvent);
}

// ═══════ DOMAIN ═══════
#[derive(Debug, Clone)]
struct Order {
    id: u64,
    customer: String,
    total: u32,
    status: String,
}

#[derive(Debug)]
enum DomainEvent {
    OrderPlaced { order_id: u64, total: u32 },
    PaymentReceived { order_id: u64, payment_id: String },
}

// ═══════ APPLICATION ═══════
fn place_order(
    customer: &str,
    total: u32,
    repo: &mut dyn OrderRepository,
    payment: &dyn PaymentGateway,
    events: &dyn EventPublisher,
) -> Result<Order, String> {
    // Charge payment
    let payment_id = payment.charge(total, "card")?;

    // Save order
    let order = Order { id: 1, customer: customer.into(), total, status: "paid".into() };
    repo.save(&order)?;

    // Publish events
    events.publish(&DomainEvent::OrderPlaced { order_id: order.id, total });
    events.publish(&DomainEvent::PaymentReceived { order_id: order.id, payment_id });

    Ok(order)
}

// ═══════ ADAPTERS (Infrastructure) ═══════
struct MemoryOrderRepo { orders: Vec<Order> }

impl MemoryOrderRepo {
    fn new() -> Self { MemoryOrderRepo { orders: vec![] } }
}

impl OrderRepository for MemoryOrderRepo {
    fn save(&mut self, order: &Order) -> Result<(), String> {
        self.orders.push(order.clone());
        Ok(())
    }
    fn find(&self, id: u64) -> Option<Order> {
        self.orders.iter().find(|o| o.id == id).cloned()
    }
    fn find_by_customer(&self, customer: &str) -> Vec<Order> {
        self.orders.iter().filter(|o| o.customer == customer).cloned().collect()
    }
}

struct FakePayment;
impl PaymentGateway for FakePayment {
    fn charge(&self, amount: u32, _method: &str) -> Result<String, String> {
        println!("  [PAYMENT] Charged {}đ", amount);
        Ok("PAY-001".into())
    }
    fn refund(&self, id: &str) -> Result<(), String> {
        println!("  [PAYMENT] Refunded {}", id);
        Ok(())
    }
}

struct ConsoleEventPublisher;
impl EventPublisher for ConsoleEventPublisher {
    fn publish(&self, event: &DomainEvent) {
        println!("  [EVENT] {:?}", event);
    }
}

fn main() {
    let mut repo = MemoryOrderRepo::new();
    let payment = FakePayment;
    let events = ConsoleEventPublisher;

    match place_order("Minh", 500_000, &mut repo, &payment, &events) {
        Ok(order) => println!("✅ {:?}", order),
        Err(e) => println!("❌ {}", e),
    }
}
```

### Lợi ích: Swap adapters dễ dàng

Đây là sức mạnh của Ports & Adapters: bạn có thể đổi **toàn bộ** infrastructure mà domain không biết. Giống như cắm quạt thay đèn — ổ cắm không quan tâm:

```
Production:  PostgresRepo + StripePayment + KafkaPublisher
Testing:     MemoryRepo   + FakePayment   + VecPublisher
Staging:     PostgresRepo + FakePayment   + ConsolePublisher
```

Chú ý: **domain code giống nhau 100%** trong cả 3 môi trường. Chỉ adapters thay đổi. Đó là "swappable infrastructure".

---

## 21.3 — Project Structure cho DDD

### Bản đồ thư mục = Bản đồ kiến trúc

Nếu kiến trúc là quả trứng, thì thư mục project phải **phản ánh** các lớp của quả trứng. Developer mới mở project, nhìn folder structure, phải hiểu ngay: "à, `domain/` là business logic, `infrastructure/` là IO".

### Layout khuyến nghị

```
my_app/
├── Cargo.toml
└── src/
    ├── main.rs                 # Entry point, wiring
    ├── domain/                 # PURE — no IO, no deps
    │   ├── mod.rs
    │   ├── model.rs            # Entities, Value Objects
    │   ├── events.rs           # Domain Events
    │   └── rules.rs            # Business rules (pure functions)
    ├── application/            # USE CASES — orchestration
    │   ├── mod.rs
    │   ├── ports.rs            # Trait definitions (ports)
    │   ├── register_user.rs    # Use case
    │   └── place_order.rs      # Use case
    └── infrastructure/         # IO — DB, HTTP, Email
        ├── mod.rs
        ├── persistence/
        │   ├── postgres_repo.rs
        │   └── in_memory_repo.rs
        ├── messaging/
        │   └── kafka_publisher.rs
        └── external/
            └── stripe_payment.rs
```

### Module visibility enforce boundaries

```rust
// domain/mod.rs — KHÔNG import từ application hoặc infrastructure!
pub mod model;
pub mod events;
pub mod rules;
// ❌ use crate::infrastructure;  // KHÔNG ĐƯỢC!

// application/mod.rs — chỉ import domain
pub mod ports;
pub mod register_user;
use crate::domain;  // ✅ OK
// ❌ use crate::infrastructure;  // KHÔNG ĐƯỢC!

// infrastructure/mod.rs — import domain + application
pub mod persistence;
pub mod messaging;
use crate::domain;       // ✅ OK
use crate::application;  // ✅ OK — implement traits
```

---

## 21.4 — Testing Architecture

### Lòng đỏ test dễ, vỏ test khó

Và đây là lý do lớn nhất để dùng Onion Architecture: **testing**. Lòng đỏ (domain) là pure functions — không cần mock, không cần database, không cần setup. Input vào, output ra, assert. Xong. 100 tests chạy trong 1 giây.

Lòng trắng (application) cần mock adapters — nhưng vì adapters là traits, mock chỉ là struct implement trait với logic giả. Vẫn nhanh.

Vỏ (infrastructure) cần real database, real API — chậm, nhưng chỉ có vài tests ở lớp này.

### Domain = test thuần túy, không mock

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

    // Domain rule: pure → test dễ!
    pub fn calculate_total(items: &[(u32, u32)], discount: u32, tax: u32) -> Result<Money, String> {
        let subtotal: u32 = items.iter().map(|(price, qty)| price * qty).sum();
        let after_discount = Money::new(subtotal).apply_discount(discount)?;
        Ok(after_discount.add_tax(tax))
    }
}

// TESTS — domain tests = pure, fast, no mocks!
#[cfg(test)]
mod domain_tests {
    use super::domain::*;

    #[test]
    fn discount_10_percent() {
        let price = Money::new(100_000);
        assert_eq!(price.apply_discount(10).unwrap().value(), 90_000);
    }

    #[test]
    fn discount_over_100_is_error() {
        let price = Money::new(100_000);
        assert!(price.apply_discount(150).is_err());
    }

    #[test]
    fn calculate_total_with_discount_and_tax() {
        let items = vec![(50_000, 2), (30_000, 1)]; // 130_000
        let total = calculate_total(&items, 10, 8).unwrap();
        // 130_000 * 90% = 117_000, + 8% tax = 126_360
        assert_eq!(total.value(), 126_360);
    }

    #[test]
    fn empty_cart_is_zero() {
        let total = calculate_total(&[], 0, 8).unwrap();
        assert_eq!(total.value(), 0);
    }
}

fn main() {
    use domain::calculate_total;
    let items = vec![(50_000, 2), (30_000, 1)];
    match calculate_total(&items, 10, 8) {
        Ok(total) => println!("Total: {}đ", total.value()),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Application = test với mock adapters

```rust
// filename: src/main.rs

// Test application layer: inject fake implementations
mod test_helpers {
    use std::collections::HashMap;

    pub struct MockRepo {
        pub saved: Vec<(u64, String)>,
        pub existing_emails: Vec<String>,
    }

    impl MockRepo {
        pub fn new() -> Self {
            MockRepo { saved: vec![], existing_emails: vec![] }
        }

        pub fn with_existing(mut self, email: &str) -> Self {
            self.existing_emails.push(email.into());
            self
        }
    }

    pub struct MockNotifier {
        pub notifications: std::cell::RefCell<Vec<String>>,
    }

    impl MockNotifier {
        pub fn new() -> Self {
            MockNotifier { notifications: std::cell::RefCell::new(vec![]) }
        }

        pub fn sent_count(&self) -> usize {
            self.notifications.borrow().len()
        }
    }
}

fn main() {
    println!("See test module for application layer testing patterns");
}
```

### Bảng Test Strategy

| Layer | Test kiểu | Mock? | Speed |
|-------|-----------|-------|-------|
| **Domain** | Unit tests | ❌ Không cần | ⚡ Cực nhanh |
| **Application** | Integration tests | ✅ Mock ports (traits) | ⚡ Nhanh |
| **Infrastructure** | Integration tests | ❌ Real DB/API | 🐢 Chậm hơn |
| **E2E** | Full system | ❌ Real everything | 🐌 Chậm nhất |

---

## 21.5 — Anti-patterns: Khi bản đồ vẽ sai

Kiến trúc tốt cần kỷ luật. Dưới đây là những lỗi thường gặp mà phá vỡ Onion Architecture — giống như đặt nhà máy ở giữa lòng đỏ quả trứng.

### ❌ Domain import Infrastructure

```rust
// ❌ WRONG: domain biết về database!
mod domain {
    use sqlx::PgPool;  // ← dependency ngược!

    pub fn create_user(pool: &PgPool, name: &str) {
        // Domain logic trộn với IO
    }
}
Domain mà import `sqlx::PgPool`? Rồi. Lòng đỏ đã bị đục lỗ — không thể test mà không có database. Giải pháp: định nghĩa trait `UserRepo` trong application, inject vào domain qua tham số.

### ❌ Anemic Domain Model

Một anti-pattern khác từ thế giới OOP/Java: structs chỉ chứa data, logic rải rác ở những "service" khác nhau. Giống tòa nhà không có nội thất — trống rỗng, mọi thứ để ngoài đường:

```rust
// ❌ WRONG: struct chỉ có data, logic ở nơi khác
struct Order { total: u32, status: String }  // chỉ data

// Logic rải rác trong services
fn validate_order(o: &Order) -> bool { ... }
fn calculate_total(o: &Order) -> u32 { ... }
fn apply_discount(o: &Order) -> u32 { ... }
Order không "biết" gì về business rules của chính nó — nó chỉ là túi đựng data. Ai cũng có thể sửa `total` trực tiếp mà không qua validation. Giải pháp: **Rich Domain Model** — data và behavior sống cùng nhau:

### ✅ Rich Domain Model

```rust
// ✅ CORRECT: data + behavior cùng chỗ
struct Order { items: Vec<OrderLine>, status: OrderStatus }

impl Order {
    fn subtotal(&self) -> u32 { /* domain logic */ }
    fn apply_discount(&self, discount: Discount) -> Result<Self, DomainError> { /* */ }
    fn can_be_shipped(&self) -> bool { self.status == OrderStatus::Paid }
}
// → Order "biết" business rules → encapsulated
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Layer identification

Phân loại các function sau vào đúng layer (Domain / Application / Infrastructure):

```rust
fn validate_email(s: &str) -> Result<Email, Error> { ... }
fn save_to_postgres(user: &User) -> Result<(), Error> { ... }
fn register_user(name: &str, email: &str) -> Result<User, Error> { ... }
fn send_ses_email(to: &str, body: &str) -> Result<(), Error> { ... }
fn calculate_shipping(weight: f64) -> u32 { ... }
```

<details><summary>✅ Lời giải Bài 1</summary>

- `validate_email` → **Domain** (pure validation)
- `save_to_postgres` → **Infrastructure** (IO, specific tech)
- `register_user` → **Application** (orchestration)
- `send_ses_email` → **Infrastructure** (IO, specific service)
- `calculate_shipping` → **Domain** (pure business rule)

</details>

---

**Bài 2** (10 phút): Extract Port

Cho function chứa hardcoded IO:

```rust
fn process_payment(amount: u32) -> Result<String, String> {
    // Gọi Stripe API trực tiếp
    let response = reqwest::blocking::get(&format!("https://stripe.com/charge/{}", amount));
    // ...
}
```

Refactor: tạo `PaymentGateway` trait (port), `StripeGateway` (adapter), và update `process_payment` nhận trait.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
// Port
trait PaymentGateway {
    fn charge(&self, amount: u32) -> Result<String, String>;
}

// Adapter
struct StripeGateway { api_key: String }
impl PaymentGateway for StripeGateway {
    fn charge(&self, amount: u32) -> Result<String, String> {
        // Real Stripe API call
        Ok(format!("stripe_payment_{}", amount))
    }
}

// For tests
struct FakeGateway;
impl PaymentGateway for FakeGateway {
    fn charge(&self, amount: u32) -> Result<String, String> {
        Ok(format!("fake_{}", amount))
    }
}

// Application: nhận trait, không biết concrete type
fn process_payment(amount: u32, gateway: &dyn PaymentGateway) -> Result<String, String> {
    if amount == 0 { return Err("Amount must be positive".into()); }
    gateway.charge(amount)
}
```

</details>

---

**Bài 3** (15 phút): Full architecture

Thiết kế 3-layer architecture cho "Library System":
- **Domain**: `Book`, `Member`, `Loan` types. `borrow_book()` pure rule (max 3 books per member)
- **Application**: `BorrowBookUseCase` dùng `BookRepo` + `MemberRepo` traits
- **Infrastructure**: in-memory implementations

<details><summary>✅ Lời giải Bài 3</summary>

```rust
mod domain {
    #[derive(Debug, Clone)]
    pub struct Book { pub isbn: String, pub title: String, pub available: bool }

    #[derive(Debug, Clone)]
    pub struct Member { pub id: u64, pub name: String, pub active_loans: u32 }

    #[derive(Debug, Clone)]
    pub struct Loan { pub member_id: u64, pub isbn: String }

    pub fn can_borrow(member: &Member, book: &Book) -> Result<(), String> {
        if !book.available { return Err("Book not available".into()); }
        if member.active_loans >= 3 { return Err("Max 3 loans reached".into()); }
        Ok(())
    }
}

mod application {
    use super::domain;

    pub trait BookRepo {
        fn find(&self, isbn: &str) -> Option<domain::Book>;
        fn mark_borrowed(&mut self, isbn: &str);
    }

    pub trait MemberRepo {
        fn find(&self, id: u64) -> Option<domain::Member>;
        fn increment_loans(&mut self, id: u64);
    }

    pub fn borrow_book(
        member_id: u64, isbn: &str,
        books: &mut dyn BookRepo, members: &mut dyn MemberRepo,
    ) -> Result<domain::Loan, String> {
        let member = members.find(member_id).ok_or("Member not found")?;
        let book = books.find(isbn).ok_or("Book not found")?;

        domain::can_borrow(&member, &book)?;

        books.mark_borrowed(isbn);
        members.increment_loans(member_id);
        Ok(domain::Loan { member_id, isbn: isbn.into() })
    }
}

fn main() {
    println!("Library system architecture ready!");
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| Domain import infrastructure | Dependency ngược | Move IO vào traits, inject từ ngoài |
| "Quá nhiều traits/layers cho app nhỏ" | Over-engineering | Start simple, refactor khi complexity tăng |
| Circular dependency giữa modules | Architecture violation | Domain KHÔNG import app/infra. Một chiều! |
| "Khó pass traits everywhere" | Manual DI verbose | Group traits vào `AppContext` struct |

---

## Tóm tắt

Chapter này dạy bạn **cấu trúc tòa nhà** trong thành phố DDD — mỗi tầng có chức năng rõ ràng:

- ✅ **Onion Architecture**: Domain (lòng đỏ, pure) → Application (lòng trắng, orchestration) → Infrastructure (vỏ, IO). Dependencies trỏ vào trong.
- ✅ **Ports** = Traits (ổ cắm). **Adapters** = Trait implementations (thiết bị). Swap Production ↔ Testing dễ dàng.
- ✅ **Module boundaries** enforce architecture: domain **không import** infrastructure. Compiler bắt lỗi kiến trúc.
- ✅ **Project structure**: `domain/`, `application/`, `infrastructure/` directories phản ánh kiến trúc.
- ✅ **Testing**: Domain = pure tests (nhanh, không mock). App = mock traits. Infra = real integration.
- ✅ **Rich Domain Model**: data + behavior cùng struct — không Anemic, không logic rải rác.

## Tiếp theo

Bạn đã biết **cấu trúc tòa nhà** — giờ hãy **thiết kế nội thất**: domain types chính xác cho từng căn phòng.

→ Chapter 22: **Domain Modeling with Rust Types** — bạn sẽ model domain cụ thể: Value Objects (newtypes + smart constructors), Entities (identity + lifecycle), Aggregates (consistency boundaries), và state machines hoàn chỉnh.
