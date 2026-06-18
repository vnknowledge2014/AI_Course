# Chapter 35 — Mocking, DI & Hexagonal Architecture

> **Bạn sẽ học được**:
> - **Trait-based Dependency Injection** — không cần DI container
> - **Manual mocks** vs `mockall` crate
> - **Hexagonal Architecture** — Port = trait, Adapter = implementation
> - **Functional core / Imperative shell** pattern
> - Testing strategies cho mỗi layer
>
> **Yêu cầu trước**: Chapter 21 (Architecture), Chapter 26 (Repository), Chapter 33 (TDD).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Domain logic **100% testable** — không cần database, HTTP, hay file system.

---

## Mocking & Dependency Injection — Test code có side-effects

Pure functions dễ test — không cần mock gì cả. Nhưng code thực tế phải gọi database, HTTP APIs, file system. Làm sao test những phần đó mà không cần database thật?

Câu trả lời: **trait-based dependency injection**. Thay vì gọi trực tiếp `Database::query()`, bạn inject trait `Repository` qua constructor. Trong production, inject real implementation. Trong test, inject mock. Rust's trait system làm pattern này type-safe và zero-cost — không cần reflection hay runtime DI container.

---

Trong Part VI, mỗi chapter xây dựng lên chapter trước. Ch33 dạy TDD cycle (Red→Green→Refactor). Ch34 dạy property-based testing cho logic thuần. Chapter này xử lý **phần khó nhất**: test code có side-effects — database reads, HTTP calls, file writes.

Triết lý FP giúp ở đây: "Functional Core, Imperative Shell" (Ch12.4) nghĩa là business logic thuần có thể test trực tiếp. Chỉ **shell** — phần gọi database, API — cần mocking. Và trong Rust, mocking dùng **trait-based DI**: define trait interface, inject implementation qua constructor. Production dùng real DB, test dùng mock. Type-safe, zero-cost, không cần reflection.

Đây là pattern bạn sẽ dùng trong **mọi** Rust project production. Hiểu nó = hiểu cách architect code cho testability.

Ch33 dạy test pure functions — dễ, không cần mock. Ch34 dạy property testing — mạnh, nhưng vẫn cho pure functions. Chapter này giải bài toán cuối: **test code có side-effects**.

Rust's trait system là **cơ chế DI tự nhiên nhất**: define trait → impl for real → impl for mock → inject qua parameter. Compiler verify tất cả tại compile time. Không cần DI container hay reflection.

Pattern: `fn process(repo: &dyn OrderRepository, order: Order) -> Result<...>`. Production: `process(&PrismaOrderRepo::new(), order)`. Test: `process(&MockOrderRepo::new(), order)`. Cùng function, cùng logic, khác implementation. Zero overhead khi dùng `impl Trait` (monomorphization).

## 35.1 — Vấn đề: Code không testable

```rust
// ❌ BAD: Business logic trộn lẫn IO
fn process_order(order_id: u64) -> Result<String, String> {
    // Đọc database trực tiếp
    // let order = database::find_order(order_id)?;
    // Gọi API trực tiếp
    // let receipt = payment_api::charge(order.total)?;
    // Gửi email trực tiếp
    // email::send(order.customer_email, receipt)?;
    Ok("done".into())
}
// Test function này? Cần database thật, API thật, email server thật!
```

### Giải pháp: Inject dependencies qua traits

```rust
// ✅ GOOD: Business logic nhận traits, không biết implementation
fn process_order(
    repo: &dyn OrderRepository,
    payment: &dyn PaymentGateway,
    notifier: &dyn Notifier,
    order_id: u64,
) -> Result<String, String> {
    let order = repo.find(order_id).ok_or("Not found")?;
    let receipt = payment.charge(order.total)?;
    notifier.notify(&order.email, &receipt)?;
    Ok(receipt)
}
// Test? Inject mock implementations!
```

---

## 35.2 — Trait-based DI in Action

```rust
// filename: src/main.rs

// ═══ PORTS (traits) ═══
trait UserRepository {
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn find_by_email(&self, email: &str) -> Option<User>;
    fn save(&mut self, user: &User) -> Result<(), String>;
}

trait EmailService {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

trait PasswordHasher {
    fn hash(&self, password: &str) -> String;
    fn verify(&self, password: &str, hash: &str) -> bool;
}

// ═══ DOMAIN ═══
#[derive(Debug, Clone)]
struct User {
    id: u64,
    email: String,
    name: String,
    password_hash: String,
}

// ═══ USE CASE (depends on traits only!) ═══
fn register_user(
    repo: &mut dyn UserRepository,
    email_svc: &dyn EmailService,
    hasher: &dyn PasswordHasher,
    name: &str,
    email: &str,
    password: &str,
) -> Result<User, String> {
    // Validation
    if name.trim().len() < 2 { return Err("Name too short".into()); }
    if !email.contains('@') { return Err("Invalid email".into()); }
    if password.len() < 8 { return Err("Password too short".into()); }

    // Business rule: unique email
    if repo.find_by_email(email).is_some() {
        return Err(format!("Email {} already registered", email));
    }

    // Create user
    let user = User {
        id: 1, // simplified
        email: email.to_lowercase(),
        name: name.trim().into(),
        password_hash: hasher.hash(password),
    };

    repo.save(&user)?;
    email_svc.send(&user.email, "Welcome!", &format!("Hi {}!", user.name))?;

    Ok(user)
}

// ═══ PRODUCTION ADAPTERS ═══
// (In real app: PostgresUserRepo, SmtpEmailService, Argon2Hasher)

// ═══ TEST ADAPTERS (mocks) ═══
use std::collections::HashMap;

struct MockUserRepo {
    users: HashMap<u64, User>,
}

impl MockUserRepo {
    fn new() -> Self { MockUserRepo { users: HashMap::new() } }
    fn with_user(mut self, user: User) -> Self {
        self.users.insert(user.id, user);
        self
    }
}

impl UserRepository for MockUserRepo {
    fn find_by_id(&self, id: u64) -> Option<User> { self.users.get(&id).cloned() }
    fn find_by_email(&self, email: &str) -> Option<User> {
        self.users.values().find(|u| u.email == email).cloned()
    }
    fn save(&mut self, user: &User) -> Result<(), String> {
        self.users.insert(user.id, user.clone());
        Ok(())
    }
}

struct MockEmailService {
    sent: std::cell::RefCell<Vec<(String, String)>>, // (to, subject)
}

impl MockEmailService {
    fn new() -> Self { MockEmailService { sent: std::cell::RefCell::new(vec![]) } }
    fn sent_count(&self) -> usize { self.sent.borrow().len() }
}

impl EmailService for MockEmailService {
    fn send(&self, to: &str, subject: &str, _body: &str) -> Result<(), String> {
        self.sent.borrow_mut().push((to.into(), subject.into()));
        Ok(())
    }
}

struct MockHasher;
impl PasswordHasher for MockHasher {
    fn hash(&self, password: &str) -> String { format!("hashed_{}", password) }
    fn verify(&self, password: &str, hash: &str) -> bool {
        hash == &format!("hashed_{}", password)
    }
}

fn main() {
    let mut repo = MockUserRepo::new();
    let email_svc = MockEmailService::new();
    let hasher = MockHasher;

    let result = register_user(&mut repo, &email_svc, &hasher, "Minh", "minh@co.com", "Str0ngPass!");
    println!("{:?}", result);
    println!("Emails sent: {}", email_svc.sent_count());
}
```

---

## 35.3 — Testing with Mocks

```rust
// filename: src/lib.rs (test section)

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (MockUserRepo, MockEmailService, MockHasher) {
        (MockUserRepo::new(), MockEmailService::new(), MockHasher)
    }

    #[test]
    fn register_success() {
        let (mut repo, email, hasher) = setup();
        let user = register_user(&mut repo, &email, &hasher, "Minh", "minh@co.com", "Str0ngPass!").unwrap();

        assert_eq!(user.name, "Minh");
        assert_eq!(user.email, "minh@co.com");
        assert_eq!(email.sent_count(), 1); // welcome email sent
    }

    #[test]
    fn register_duplicate_email() {
        let existing = User {
            id: 1, email: "minh@co.com".into(),
            name: "Minh".into(), password_hash: "xxx".into(),
        };
        let (mut repo, email, hasher) = setup();
        let mut repo = repo.with_user(existing);

        let result = register_user(&mut repo, &email, &hasher, "Other", "minh@co.com", "Pass1234!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already registered"));
        assert_eq!(email.sent_count(), 0); // no email on failure
    }

    #[test]
    fn register_validates_name() {
        let (mut repo, email, hasher) = setup();
        assert!(register_user(&mut repo, &email, &hasher, "M", "m@co.com", "Pass1234!").is_err());
    }

    #[test]
    fn register_validates_email() {
        let (mut repo, email, hasher) = setup();
        assert!(register_user(&mut repo, &email, &hasher, "Minh", "invalid", "Pass1234!").is_err());
    }

    #[test]
    fn register_validates_password() {
        let (mut repo, email, hasher) = setup();
        assert!(register_user(&mut repo, &email, &hasher, "Minh", "m@co.com", "short").is_err());
    }

    #[test]
    fn register_hashes_password() {
        let (mut repo, email, hasher) = setup();
        let user = register_user(&mut repo, &email, &hasher, "Minh", "m@co.com", "Str0ngPass!").unwrap();
        assert_eq!(user.password_hash, "hashed_Str0ngPass!");
        assert!(hasher.verify("Str0ngPass!", &user.password_hash));
    }
}
```

---

## ✅ Checkpoint 35.3

> Ghi nhớ:
> 1. **Port** = trait, **Adapter** = implementation
> 2. Production: `PostgresRepo`, `SmtpEmail`, `Argon2Hasher`
> 3. Test: `MockRepo`, `MockEmail`, `MockHasher`
> 4. **Business logic** chỉ biết traits → test **không cần** infrastructure!

---

## 35.4 — Hexagonal Architecture (Ports & Adapters)

```
                    ┌─────────────────────────┐
     Driving        │      Application        │       Driven
     Adapters       │         Core            │       Adapters
                    │                         │
  ┌─────────┐      │  ┌─────────────────┐    │      ┌──────────┐
  │ REST API │─────▶│  │  Use Cases      │    │─────▶│ Postgres │
  └─────────┘ Port │  │  (register,     │Port│      └──────────┘
                    │  │   login,        │    │
  ┌─────────┐      │  │   process_order)│    │      ┌──────────┐
  │   CLI   │─────▶│  │                 │    │─────▶│   SMTP   │
  └─────────┘      │  │  Domain Logic   │    │      └──────────┘
                    │  │  (pure)         │    │
  ┌─────────┐      │  └─────────────────┘    │      ┌──────────┐
  │  Tests  │─────▶│                         │─────▶│ In-Memory│
  └─────────┘      └─────────────────────────┘      └──────────┘
```

### Complete example

```rust
// filename: src/main.rs

// ═══ DOMAIN (pure, no IO, no traits) ═══
mod domain {
    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: u64,
        pub name: String,
        pub price: u32,
        pub stock: u32,
    }

    #[derive(Debug)]
    pub enum OrderError {
        OutOfStock(String),
        InvalidQuantity,
        ProductNotFound,
    }

    // Pure domain logic — no dependencies!
    pub fn can_fulfill(product: &Product, qty: u32) -> Result<(), OrderError> {
        if qty == 0 { return Err(OrderError::InvalidQuantity); }
        if product.stock < qty {
            Err(OrderError::OutOfStock(format!("{}: have {}, need {}", product.name, product.stock, qty)))
        } else {
            Ok(())
        }
    }

    pub fn calculate_total(price: u32, qty: u32, discount_pct: u32) -> u32 {
        let subtotal = price * qty;
        subtotal - subtotal * discount_pct / 100
    }
}

// ═══ PORTS (traits at application boundary) ═══
mod ports {
    use super::domain::*;

    pub trait ProductRepo {
        fn find(&self, id: u64) -> Option<Product>;
        fn update_stock(&mut self, id: u64, new_stock: u32) -> Result<(), String>;
    }

    pub trait PaymentGateway {
        fn charge(&self, amount: u32, description: &str) -> Result<String, String>;
    }
}

// ═══ APPLICATION (use cases, orchestration) ═══
mod application {
    use super::domain::*;
    use super::ports::*;

    pub fn purchase(
        repo: &mut dyn ProductRepo,
        payment: &dyn PaymentGateway,
        product_id: u64,
        qty: u32,
        discount: u32,
    ) -> Result<String, String> {
        let product = repo.find(product_id)
            .ok_or("Product not found".to_string())?;

        can_fulfill(&product, qty)
            .map_err(|e| format!("{:?}", e))?;

        let total = calculate_total(product.price, qty, discount);

        let receipt = payment.charge(total, &format!("{} x{}", product.name, qty))?;

        repo.update_stock(product_id, product.stock - qty)?;

        Ok(format!("Order confirmed: {} — {}đ (receipt: {})", product.name, total, receipt))
    }
}

// ═══ ADAPTERS ═══
mod adapters {
    use super::domain::*;
    use super::ports::*;
    use std::collections::HashMap;

    // Production adapter (simplified)
    pub struct InMemoryProductRepo {
        products: HashMap<u64, Product>,
    }

    impl InMemoryProductRepo {
        pub fn new() -> Self { InMemoryProductRepo { products: HashMap::new() } }
        pub fn seed(mut self, product: Product) -> Self {
            self.products.insert(product.id, product);
            self
        }
    }

    impl ProductRepo for InMemoryProductRepo {
        fn find(&self, id: u64) -> Option<Product> { self.products.get(&id).cloned() }
        fn update_stock(&mut self, id: u64, stock: u32) -> Result<(), String> {
            self.products.get_mut(&id)
                .map(|p| p.stock = stock)
                .ok_or("Not found".into())
        }
    }

    // Mock payment
    pub struct FakePaymentGateway;
    impl PaymentGateway for FakePaymentGateway {
        fn charge(&self, amount: u32, desc: &str) -> Result<String, String> {
            Ok(format!("FAKE-{}", amount))
        }
    }

    // Failing payment (for error testing)
    pub struct FailingPaymentGateway;
    impl PaymentGateway for FailingPaymentGateway {
        fn charge(&self, _: u32, _: &str) -> Result<String, String> {
            Err("Payment declined".into())
        }
    }
}

fn main() {
    use domain::Product;

    let mut repo = adapters::InMemoryProductRepo::new()
        .seed(Product { id: 1, name: "Coffee".into(), price: 85_000, stock: 50 });
    let payment = adapters::FakePaymentGateway;

    match application::purchase(&mut repo, &payment, 1, 3, 10) {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => println!("❌ {}", e),
    }
}
```

---

## 35.5 — Functional Core / Imperative Shell

### Pure domain functions = dễ test nhất

```rust
// filename: src/lib.rs

mod domain {
    // ═══ FUNCTIONAL CORE — pure, no IO ═══
    pub fn validate_discount(price: u32, discount_pct: u32) -> Result<u32, String> {
        if discount_pct > 50 { return Err("Max discount is 50%".into()); }
        Ok(price * (100 - discount_pct) / 100)
    }

    pub fn tier_from_total_spent(total: u64) -> &'static str {
        match total {
            0..=999_999 => "Bronze",
            1_000_000..=4_999_999 => "Silver",
            5_000_000..=19_999_999 => "Gold",
            _ => "Platinum",
        }
    }

    pub fn shipping_cost(weight_grams: u32, zone: &str) -> u32 {
        let base = match zone {
            "local" => 15_000,
            "domestic" => 30_000,
            "international" => 150_000,
            _ => 50_000,
        };
        let weight_surcharge = (weight_grams / 500) * 5_000;
        base + weight_surcharge
    }
}

// ═══ Tests: no mocks needed! Pure functions! ═══
#[cfg(test)]
mod tests {
    use super::domain::*;

    #[test]
    fn discount_valid() {
        assert_eq!(validate_discount(100_000, 10), Ok(90_000));
    }

    #[test]
    fn discount_max_50() {
        assert!(validate_discount(100_000, 51).is_err());
    }

    #[test]
    fn tier_levels() {
        assert_eq!(tier_from_total_spent(500_000), "Bronze");
        assert_eq!(tier_from_total_spent(2_000_000), "Silver");
        assert_eq!(tier_from_total_spent(10_000_000), "Gold");
        assert_eq!(tier_from_total_spent(50_000_000), "Platinum");
    }

    #[test]
    fn shipping_local() {
        assert_eq!(shipping_cost(300, "local"), 15_000);
    }

    #[test]
    fn shipping_heavy_domestic() {
        // 1500g = 3 × 500g surcharges
        assert_eq!(shipping_cost(1500, "domestic"), 30_000 + 15_000);
    }
}
```

### Testing strategy summary

| Layer | What to test | How | Speed |
|-------|-------------|-----|-------|
| **Domain** (pure) | Business rules, calculations | Direct function calls, no mocks | ⚡ ms |
| **Application** (use cases) | Orchestration, flow | Mock traits (ports) | ⚡ ms |
| **Infrastructure** (adapters) | DB queries, API calls | Real DB/services | 🐢 seconds |
| **E2E** | Full system | Docker compose | 🐌 minutes |

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Identify ports

Cho hệ thống notification, liệt kê ports (traits) cần thiết:
- Gửi email, SMS, push notification
- Đọc user preferences
- Log events

<details><summary>✅ Lời giải</summary>

```rust
trait EmailSender { fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String>; }
trait SmsSender { fn send(&self, phone: &str, message: &str) -> Result<(), String>; }
trait PushSender { fn send(&self, device_token: &str, title: &str) -> Result<(), String>; }
trait UserPreferences { fn get(&self, user_id: u64) -> Option<NotifPrefs>; }
trait EventLogger { fn log(&self, event: &str); }
```

</details>

---

**Bài 2** (10 phút): Mock and test

Viết use case `send_notification(user_id)`:
1. Lookup user preferences
2. Send via preferred channel (email/sms/push)
3. Log the event

Viết mocks + 3 tests (prefer email, prefer sms, user not found).

<details><summary>✅ Lời giải Bài 2</summary>

```rust
struct NotifPrefs { channel: String, contact: String }

trait PrefsRepo { fn get(&self, id: u64) -> Option<NotifPrefs>; }
trait Sender { fn send(&self, to: &str, msg: &str) -> Result<(), String>; }

fn send_notification(prefs: &dyn PrefsRepo, sender: &dyn Sender, user_id: u64, msg: &str) -> Result<(), String> {
    let pref = prefs.get(user_id).ok_or("User not found")?;
    sender.send(&pref.contact, msg)
}

// Tests
struct MockPrefs(Option<NotifPrefs>);
impl PrefsRepo for MockPrefs { fn get(&self, _: u64) -> Option<NotifPrefs> { self.0.clone() } }

struct MockSender(Result<(), String>);
impl Sender for MockSender { fn send(&self, _: &str, _: &str) -> Result<(), String> { self.0.clone() } }

#[test]
fn sends_to_preferred_channel() {
    let prefs = MockPrefs(Some(NotifPrefs { channel: "email".into(), contact: "a@b.com".into() }));
    let sender = MockSender(Ok(()));
    assert!(send_notification(&prefs, &sender, 1, "Hello").is_ok());
}

#[test]
fn user_not_found() {
    let prefs = MockPrefs(None);
    let sender = MockSender(Ok(()));
    assert!(send_notification(&prefs, &sender, 99, "Hello").is_err());
}
```

</details>

---

**Bài 3** (15 phút): Full hexagonal

Thiết kế `TransferService`:
1. Ports: `AccountRepo`, `AuditLogger`, `NotificationService`
2. Use case: `transfer(from_id, to_id, amount)` — validate, debit, credit, log, notify
3. Implement mocks + test: success, insufficient funds, account not found

<details><summary>✅ Lời giải Bài 3</summary>

```rust
trait AccountRepo {
    fn find(&self, id: u64) -> Option<Account>;
    fn save(&mut self, acc: &Account) -> Result<(), String>;
}
trait AuditLogger { fn log(&self, event: &str); }
trait Notifier { fn notify(&self, to: &str, msg: &str) -> Result<(), String>; }

struct Account { id: u64, name: String, balance: i64 }

fn transfer(
    repo: &mut dyn AccountRepo,
    logger: &dyn AuditLogger,
    notifier: &dyn Notifier,
    from: u64, to: u64, amount: u64,
) -> Result<String, String> {
    let from_acc = repo.find(from).ok_or("Source not found")?;
    let to_acc = repo.find(to).ok_or("Target not found")?;
    if from_acc.balance < amount as i64 { return Err("Insufficient funds".into()); }

    let updated_from = Account { balance: from_acc.balance - amount as i64, ..from_acc };
    let updated_to = Account { balance: to_acc.balance + amount as i64, ..to_acc };

    repo.save(&updated_from)?;
    repo.save(&updated_to)?;
    logger.log(&format!("Transfer {}đ: {} → {}", amount, from, to));
    notifier.notify(&updated_from.name, &format!("Sent {}đ", amount))?;

    Ok(format!("Transferred {}đ", amount))
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Quá nhiều traits/mocks" | Over-abstraction | Chỉ abstract cái CẦN swap (DB, API). Pure functions → test trực tiếp |
| "Mock verification phức tạp" | Checking call counts/args | Dùng `RefCell<Vec<_>>` record calls |
| "Hexagonal quá boilerplate" | Nhỏ project | Scale: small=direct, medium=traits, large=full hexagonal |
| "Trait object performance" | Dynamic dispatch overhead | Dùng generics `<R: Repo>` thay `&dyn Repo` |

---

## Tóm tắt

- ✅ **Trait-based DI**: Port = trait, inject qua function params. Không cần DI container.
- ✅ **Manual mocks**: `MockUserRepo`, `MockEmailService` — implement traits, record calls.
- ✅ **Hexagonal Architecture**: Domain (pure) → Application (orchestration + ports) → Infrastructure (adapters).
- ✅ **Functional core / Imperative shell**: Domain logic pure → test trực tiếp, zero mocks.
- ✅ **Testing strategy**: Domain = pure tests ⚡ | App = mock ports | Infra = real integration 🐢.

## Tiếp theo

→ Chapter 36: **Concurrency & Async** — bạn sẽ learn `std::thread`, `Arc<Mutex<T>>`, channels, `async/await`, `tokio`. Rust's "fearless concurrency" qua ownership system.
