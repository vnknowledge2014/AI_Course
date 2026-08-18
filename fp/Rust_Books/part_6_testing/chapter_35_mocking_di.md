# Chapter 35 — Mocking, DI & Hexagonal Architecture

> **Bạn sẽ học được**:
> - **Trait-based Dependency Injection** — không cần DI container cồng kềnh
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

Những hàm thuần túy (Pure functions) rất dễ kiểm thử — không cần phải giả lập (mock) gì cả. Bạn chỉ cần ném dữ liệu vào và kiểm tra đầu ra. 
Nhưng code thực tế không chỉ tính toán suông, nó phải gọi database, kết nối HTTP APIs, và ghi ra file. Làm sao để test những hàm đó nhanh gọn mà không cần một con Database thật đang chạy?

Câu trả lời của Rust là: **Trait-based Dependency Injection**. Thay vì gọi trực tiếp `Database::query()`, bạn tiêm (inject) một Trait tên là `Repository` vào hàm. 
Trong Production, bạn đưa một Implementation thật vào. Trong Test, bạn đưa một bản Mock giả vào. Hệ thống Type của Rust sẽ lo liệu phần còn lại (type-safe và zero-cost) mà không cần đến công cụ Reflection hay DI Framework ma thuật nào!

---

## 35.1 — Vấn đề: Code không thể test nổi

Hãy xem một đoạn code ngây thơ thường gặp:

```rust
// ❌ BAD: Business logic trộn lẫn rườm rà với IO
fn process_order(order_id: u64) -> Result<String, String> {
    // Đọc database trực tiếp (Hard-dependency)
    // let order = database::find_order(order_id)?;
    
    // Gọi API trực tiếp (Hard-dependency)
    // let receipt = payment_api::charge(order.total)?;
    
    // Gửi email trực tiếp (Hard-dependency)
    // email::send(order.customer_email, receipt)?;
    Ok("done".into())
}
// Test function này kiểu gì? 
// Bạn sẽ phải cần database thật, payment API thật (tốn tiền), email server thật!
```

### Giải pháp: Tiêm phụ thuộc qua Traits (DI)

```rust
// ✅ GOOD: Business logic nhận các traits, không quan tâm ai thực thi chúng
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
// Test function này? Quá đơn giản! Tạo 3 cái Mocks giả lập và ném vào hàm!
```

---

## 35.2 — Trait-based DI in Action (Thực chiến)

Chúng ta hãy xây dựng một tính năng Đăng ký Người dùng (Register User). Tính năng này cần gọi Database, Gửi Email, và Băm Mật khẩu. Thay vì trói chặt chúng, ta sẽ tách chúng ra.

Bước 1: Định nghĩa các Cổng giao tiếp (Ports hay Traits) và Domain Object.

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
```

Bước 2: Viết Use Case. Logic Đăng ký sẽ chỉ tương tác với các Trait (không hề dính líu đến PostgreSQL hay Gmail).

```rust
// ═══ USE CASE (depends on traits only!) ═══
fn register_user(
    repo: &mut dyn UserRepository,
    email_svc: &dyn EmailService,
    hasher: &dyn PasswordHasher,
    name: &str,
    email: &str,
    password: &str,
) -> Result<User, String> {
    // Validation cơ bản
    if name.trim().len() < 2 { return Err("Name too short".into()); }
    if !email.contains('@') { return Err("Invalid email".into()); }
    if password.len() < 8 { return Err("Password too short".into()); }

    // Business rule: Email không được trùng
    if repo.find_by_email(email).is_some() {
        return Err(format!("Email {} already registered", email));
    }

    // Hash password và lưu user
    let user = User {
        id: 1, // Để đơn giản
        email: email.to_lowercase(),
        name: name.trim().into(),
        password_hash: hasher.hash(password),
    };

    repo.save(&user)?;
    
    // Gửi email chào mừng
    email_svc.send(&user.email, "Welcome!", &format!("Hi {}!", user.name))?;

    Ok(user)
}
```

Bước 3: Để chạy được đoạn mã này mà không cần cài đặt cơ sở dữ liệu, ta chỉ cần tạo các bản Fake (Mocks). Chúng lưu dữ liệu thẳng vào RAM (`HashMap` hoặc `Vec`) thay vì ổ cứng!

```rust
// ═══ TEST ADAPTERS (mocks) ═══
use std::collections::HashMap;

// --- Mock Database ---
struct MockUserRepo {
    users: HashMap<u64, User>,
}

impl MockUserRepo {
    fn new() -> Self { MockUserRepo { users: HashMap::new() } }
    
    // Builder pattern tiện lợi cho test
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

// --- Mock Email Server ---
struct MockEmailService {
    // Dùng RefCell để lách luật cho mượn reference
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

// --- Mock Password Hasher ---
struct MockHasher;
impl PasswordHasher for MockHasher {
    fn hash(&self, password: &str) -> String { format!("hashed_{}", password) }
    fn verify(&self, password: &str, hash: &str) -> bool {
        hash == &format!("hashed_{}", password)
    }
}
```

Hãy đưa chúng vào hàm `main()` để thấy phép màu:

```rust
fn main() {
    let mut repo = MockUserRepo::new();
    let email_svc = MockEmailService::new();
    let hasher = MockHasher;

    // Chạy Use Case mà không cần Internet hay Server!
    let result = register_user(&mut repo, &email_svc, &hasher, "Minh", "minh@co.com", "Str0ngPass!");
    
    println!("{:?}", result);
    println!("Emails sent: {}", email_svc.sent_count());
}
```

---

## 35.3 — Testing with Mocks

Chuyển các đoạn chạy thử trên thành Unit Tests nghiêm túc:

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
        
        // Cài sẵn một User vào Mock Database
        let mut repo = repo.with_user(existing);

        let result = register_user(&mut repo, &email, &hasher, "Other", "minh@co.com", "Pass1234!");
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already registered"));
        assert_eq!(email.sent_count(), 0); // Lỗi xảy ra thì không được gửi email!
    }
}
```

---

## 35.4 — Hexagonal Architecture (Ports & Adapters)

Mô hình kiến trúc Lục Giác (Hexagonal) hay Kiến trúc Sạch (Clean Architecture) là bước tiến hóa tất yếu của kĩ thuật trên.

```text
                    ┌─────────────────────────┐
     Driving        │      Application        │       Driven
     Adapters       │         Core            │       Adapters
                    │                         │
  ┌─────────┐      │  ┌─────────────────┐    │      ┌──────────┐
  │ REST API │─────▶│  │  Use Cases      │    │─────▶│ Postgres │
  └─────────┘ Port │  │  (register,     │Port│      └──────────┘
                    │  │   process_order)│    │
  ┌─────────┐      │  │                 │    │      ┌──────────┐
  │   CLI   │─────▶│  │  Domain Logic   │    │─────▶│   SMTP   │
  └─────────┘      │  │  (pure)         │    │      └──────────┘
                    │  └─────────────────┘    │
  ┌─────────┐      │                         │      ┌──────────┐
  │  Tests  │─────▶│                         │─────▶│ In-Memory│
  └─────────┘      └─────────────────────────┘      └──────────┘
```

Trái tim của hệ thống là `Domain Logic` (chỉ toàn các Pure Functions tính toán). Bao bọc nó là `Application` (nơi định nghĩa các luồng nghiệp vụ). 
Và nằm rìa ngoài cùng là `Adapters` — những công nhân dơ dáy chuyên làm việc với IO, Web, hay DB.

Hãy minh họa nó:

```rust
// filename: src/main.rs

// ═══ LỚP 1: DOMAIN (Trái tim thuần khiết, không gọi thư viện ngoài) ═══
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

// ═══ LỚP 2: PORTS (Bộ giao thức mà Core yêu cầu thế giới ngoài phải tuân thủ) ═══
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
```

Bây giờ đến lớp Use Cases. Nó sẽ dùng Domain Models kết hợp với Ports để làm nên chuyện:

```rust
// ═══ LỚP 3: APPLICATION (Kịch bản điều phối) ═══
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
        // Bóc dữ liệu ra từ Port
        let product = repo.find(product_id).ok_or("Product not found".to_string())?;

        // Gửi vào Domain Logic để kiểm tra
        can_fulfill(&product, qty).map_err(|e| format!("{:?}", e))?;
        let total = calculate_total(product.price, qty, discount);

        // Gọi các Port khác để thanh toán và cập nhật DB
        let receipt = payment.charge(total, &format!("{} x{}", product.name, qty))?;
        repo.update_stock(product_id, product.stock - qty)?;

        Ok(format!("Order confirmed: {} — {}đ (receipt: {})", product.name, total, receipt))
    }
}
```

Cuối cùng, ở rìa ngoài hệ thống, ta cung cấp Adapter (Implement thực sự cho các Trait). Để test, ta chỉ cần Fake Adapter:

```rust
// ═══ LỚP 4: ADAPTERS (Công nhân đào đất) ═══
mod adapters {
    use super::domain::*;
    use super::ports::*;
    use std::collections::HashMap;

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
            self.products.get_mut(&id).map(|p| p.stock = stock).ok_or("Not found".into())
        }
    }

    // Mock payment
    pub struct FakePaymentGateway;
    impl PaymentGateway for FakePaymentGateway {
        fn charge(&self, amount: u32, _desc: &str) -> Result<String, String> {
            Ok(format!("FAKE-{}", amount))
        }
    }
}

// Chạy thử!
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

Đây là triết lý sâu thẳm nhất của Functional Programming. "Lõi Hàm (Thuần), Vỏ Mệnh Lệnh".

Nếu bạn có thể đẩy MỌI logic tính toán vào trong các **Pure Functions**, việc kiểm thử sẽ sướng vô cùng (vì chúng KHÔNG CẦN BẤT KỲ MOCK NÀO CẢ). Các mocks cồng kềnh phía trên chỉ dùng để test cho `Imperative Shell` — phần vỏ bọc gọi API và DB mà thôi.

Hãy xem sức mạnh của Pure Functions:

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
```

Kiểm thử chúng sướng thế nào? Nhìn đây:

```rust
// ═══ Tests: no mocks needed! Pure functions! ═══
#[cfg(test)]
mod tests {
    use super::domain::*;

    #[test]
    fn tier_levels() {
        assert_eq!(tier_from_total_spent(500_000), "Bronze");
        assert_eq!(tier_from_total_spent(2_000_000), "Silver");
        assert_eq!(tier_from_total_spent(10_000_000), "Gold");
        assert_eq!(tier_from_total_spent(50_000_000), "Platinum");
    }

    #[test]
    fn shipping_heavy_domestic() {
        // 1500g = 3 × 500g surcharges
        assert_eq!(shipping_cost(1500, "domestic"), 30_000 + 15_000);
    }
}
```

### Bảng tóm tắt chiến lược Kiểm Thử

| Layer | Nên kiểm thử gì? | Bằng cách nào? | Tốc độ chạy |
|-------|-------------|-----|-------|
| **Domain** (lõi thuần) | Các thuật toán, luật nghiệp vụ | Gọi hàm trực tiếp, **không cần mock** | ⚡ ms (Siêu tốc) |
| **Application** (use cases) | Luồng nghiệp vụ, thứ tự gọi | Dùng Trait Mocks để giả lập IO | ⚡ ms (Nhanh) |
| **Infrastructure** (adapters) | Lệnh SQL có đúng không | Cắm vào một DB thật (Testcontainers) | 🐢 seconds (Chậm) |
| **E2E** | Cả hệ thống có chạy nổi không | Dựng cả app bằng Docker | 🐌 minutes (Rất chậm) |

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Nhận diện Cổng (Identify ports)

Cho hệ thống Gửi Thông Báo, liệt kê các Ports (traits) cần thiết:
- Gửi email, SMS, push notification
- Đọc ưu tiên của User (thích nhận qua kênh nào)
- Ghi log các sự kiện

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

Viết một Use case `send_notification(user_id)` thực hiện 3 bước:
1. Tra cứu xem User thích nhận kênh nào (Email hay SMS)
2. Gửi tin nhắn qua kênh đó
3. Ghi log sự kiện

Đồng thời, tạo các Mock cho nó và viết 1 Test case cho trường hợp gọi thành công.

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
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Sao phải tách tận 4 File để viết một hàm thế này?" | Hexagonal tốn nhiều Boilerplate lúc ban đầu. | Với project nhỏ, cứ quăng hết vào 1 file. Mô hình Hexagonal này dành cho hệ thống lớn cần sự chia tách rạch ròi. |
| "Mock code của tôi dài khủng khiếp để đếm số lần hàm được gọi" | Viết tay khá vất vả | Thật ra bạn có thể dùng Crate `mockall` để tự sinh đống code Mock đó bằng Macro! |
| "Hiệu năng khi dùng Trait Objects (`&dyn Trait`) bị chậm đi" | Do Dynamic Dispatch làm tốn thêm bước tra cứu bảng Vtable. | Chuyển qua dùng Generic cho tĩnh hóa (Ví dụ: `fn register<R: UserRepository>(repo: &mut R)`)! |

---

---

## ✅ Checkpoint 35

1. Trait làm port. Nên dùng generic `T: Repo` hay `Box<dyn Repo>`?
2. Vì sao "functional core / imperative shell" giảm nhu cầu mock?
3. `mockall` sinh mock tự động. Khi nào mock tay lại tốt hơn?

<details>
<summary>Đáp án</summary>

1. Generic khi số lượng implementation biết trước lúc biên dịch và bạn cần tốc độ (monomorphization, inline được). `Box<dyn Repo>` khi cần chọn lúc runtime hoặc muốn tránh code bloat. Mặc định: bắt đầu bằng generic, đổi sang `dyn` khi generic lan quá rộng.
2. Vì phần lõi thuần **không có** dependency để mock — chỉ có dữ liệu vào và dữ liệu ra. Mock chỉ còn cần ở lớp vỏ mỏng, và lớp đó ít logic tới mức test tích hợp phủ được.
3. Khi mock cần **hành vi**, không chỉ giá trị trả về — ví dụ một repo in-memory thật sự lưu và đọc lại được. Mock tay kiểu đó thường dùng lại được ở nhiều test và đọc dễ hơn hẳn một chuỗi `expect_*().returning(...)`.
</details>

## Tóm tắt

- ✅ **Trait-based DI**: Thay vì viết DI Framework phức tạp, truyền thẳng `Trait` vào tham số hàm. 
- ✅ **Manual mocks**: Bạn có thể viết Mock bằng tay cực kì dễ bằng cách nhét `HashMap` hoặc `Vec` vào Struct để thực thi `Trait`.
- ✅ **Hexagonal Architecture**: Lõi là Logic tính toán (Pure) → Lớp bọc ngoài là Điều phối viên (Use Cases + Traits) → Vỏ ngoài cùng là Công nhân Adapter (Database, Web).
- ✅ **Functional core / Imperative shell**: Đẩy càng nhiều xử lý về "Lõi tính toán thuần túy" thì việc Test càng nhanh và dễ.

## Tiếp theo

→ Chapter 36: **Concurrency & Async** — bạn sẽ học cách khiến các nhân CPU làm việc cùng lúc với `std::thread`, `Arc<Mutex<T>>`, channels, `async/await`, `tokio`. Sự kì diệu của Ownership System trong Rust sẽ tỏa sáng ở đây (Fearless Concurrency)!
