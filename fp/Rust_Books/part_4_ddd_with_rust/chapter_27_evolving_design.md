# Chapter 27 — Evolving the Design

> **Bạn sẽ học được**:
> - **Thêm features** mà không break existing code
> - **Refactoring với compiler guidance** — Rust compiler = pair programmer
> - **Feature flags** bằng enums — compile-time switches
> - **Backward-compatible type evolution** — thêm variants, thêm fields an toàn
> - **Deprecation strategy** — từ từ loại bỏ old code
>
> **Yêu cầu trước**: Part IV complete (đặc biệt Chapter 14 Enums, Chapter 16 Traits).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn **evolve** codebase tự tin — compiler bắt mọi chỗ cần sửa khi thay đổi types.

---

## 27.1 — Compiler-Guided Refactoring

### Thành phố không bao giờ ngừng thay đổi

Thành phố bạn vẽ bản đồ ở Chapter 20 không đông cứng. Đường mới được xây, khu phố mới mọc lên, quận cũ sáp nhập. Domain cũng vậy: business thêm sản phẩm mới, quy trình thay đổi, luật mới ban hành. Cuốn sách domain-driven dạy bạn vẽ bản đồ — chapter này dạy bạn **cập nhật bản đồ mà không làm người dân đi lạc**.

Trong thế giới khác (Python, JavaScript), thay đổi bản đồ và hy vọng không ai đi lạc. Trong Rust, bạn có một đồng minh đặc biệt: **compiler**. Nó giống GPS — khi bạn thay đổi đường, GPS ngay lập tức nói: "47 chỗ bị ảnh hưởng bởi thay đổi này". Sửa từng chỗ theo hướng dẫn của compiler — và khi 0 errors, bạn **biết chắc** mọi thứ nhất quán.

### Rust compiler = đồng đội refactoring

```
1. Sửa type/field/variant
2. Compile → 47 errors
3. Fix error thứ 1 → 46 errors
4. Fix error thứ 2 → 45 errors
5. ... (compiler hướng dẫn mỗi chỗ)
6. 0 errors → TẤT CẢ code đã nhất quán!
```

### Ví dụ: Thêm variant mới vào enum

```rust
// filename: src/main.rs

// V1: 3 payment methods
#[derive(Debug)]
enum PaymentMethod {
    Cash,
    Card { last_four: String },
    BankTransfer { bank: String, account: String },
    // V2: thêm E-wallet! 👇
    EWallet { provider: String, phone: String },
}

// Compiler SẼ BÁO LỖI ở mọi `match` thiếu EWallet! ⭐
fn process_payment(method: &PaymentMethod, amount: u32) -> Result<String, String> {
    match method {
        PaymentMethod::Cash => {
            Ok(format!("Received {}đ cash", amount))
        }
        PaymentMethod::Card { last_four } => {
            Ok(format!("Charged {}đ to card ****{}", amount, last_four))
        }
        PaymentMethod::BankTransfer { bank, account } => {
            Ok(format!("Transfer {}đ to {} ({})", amount, bank, account))
        }
        // Bỏ dòng dưới → compiler error: "non-exhaustive patterns"
        PaymentMethod::EWallet { provider, phone } => {
            Ok(format!("Charged {}đ via {} ({})", amount, provider, phone))
        }
    }
}

fn receipt_text(method: &PaymentMethod) -> &str {
    match method {
        PaymentMethod::Cash => "Cash payment",
        PaymentMethod::Card { .. } => "Card payment",
        PaymentMethod::BankTransfer { .. } => "Bank transfer",
        PaymentMethod::EWallet { .. } => "E-wallet payment",
    }
}

fn is_instant(method: &PaymentMethod) -> bool {
    match method {
        PaymentMethod::Cash | PaymentMethod::EWallet { .. } => true,
        PaymentMethod::Card { .. } | PaymentMethod::BankTransfer { .. } => false,
    }
}

fn main() {
    let methods = vec![
        PaymentMethod::Cash,
        PaymentMethod::Card { last_four: "4242".into() },
        PaymentMethod::EWallet { provider: "MoMo".into(), phone: "0901234567".into() },
    ];

    for m in &methods {
        println!("{} — {} — instant: {}",
            receipt_text(m),
            process_payment(m, 100_000).unwrap(),
            is_instant(m));
    }
}
```

> **💡 Rule**: Tránh `_ => ...` wildcard trong match khi bạn muốn compiler catch new variants. Luôn match **explicit** từng variant.

---

## 27.2 — Non-breaking Changes: Thêm mà không sửa

Khi thành phố mở rộng, bạn thêm đường mới mà không xóa đường cũ. Cư dân vẫn lưu thông bình thường, chỉ có thêm lựa chọn. Trong code, điều này nghĩa là: **old code vẫn hoạt động** sau khi bạn thêm features mới. Hai strategies chính:

```rust
// filename: src/main.rs

// V1: original struct
#[derive(Debug, Clone)]
struct UserProfile {
    name: String,
    email: String,
    // V2: thêm fields MỚI với defaults
    avatar_url: Option<String>,      // None = chưa set
    bio: Option<String>,             // None = chưa set
    notification_prefs: NotificationPrefs,
}

#[derive(Debug, Clone)]
struct NotificationPrefs {
    email_enabled: bool,
    push_enabled: bool,
    sms_enabled: bool,
}

impl Default for NotificationPrefs {
    fn default() -> Self {
        NotificationPrefs {
            email_enabled: true,
            push_enabled: true,
            sms_enabled: false,
        }
    }
}

impl UserProfile {
    // Constructor giữ backward-compatibility
    fn new(name: &str, email: &str) -> Self {
        UserProfile {
            name: name.into(),
            email: email.into(),
            avatar_url: None,
            bio: None,
            notification_prefs: NotificationPrefs::default(),
        }
    }

    // Builder methods cho new fields
    fn with_avatar(mut self, url: &str) -> Self {
        self.avatar_url = Some(url.into());
        self
    }

    fn with_bio(mut self, bio: &str) -> Self {
        self.bio = Some(bio.into());
        self
    }

    fn with_notifications(mut self, prefs: NotificationPrefs) -> Self {
        self.notification_prefs = prefs;
        self
    }
}

fn main() {
    // Old code vẫn hoạt động!
    let user = UserProfile::new("Minh", "minh@co.com");
    println!("V1 style: {:?}\n", user);

    // New code dùng thêm features
    let user = UserProfile::new("Lan", "lan@co.com")
        .with_avatar("https://example.com/avatar.jpg")
        .with_bio("Rust enthusiast");
    println!("V2 style: {:?}", user);
}
```

### Strategy 2: Extension trait

```rust
// filename: src/main.rs

// ═══ Core trait (stable, không đổi) ═══
trait Greeter {
    fn greet(&self) -> String;
}

// ═══ Extension trait (thêm features mới) ═══
trait GreeterExt: Greeter {
    fn greet_formal(&self) -> String {
        format!("Dear Sir/Madam, {}", self.greet())
    }

    fn greet_casual(&self) -> String {
        format!("Hey! {}", self.greet())
    }
}

// Blanket impl: MỌI Greeter tự động có GreeterExt
impl<T: Greeter> GreeterExt for T {}

struct User { name: String }

impl Greeter for User {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

fn main() {
    let user = User { name: "Minh".into() };

    // Old API vẫn hoạt động
    println!("{}", user.greet());

    // New API tự động available
    println!("{}", user.greet_formal());
    println!("{}", user.greet_casual());
}
```

---

## 27.3 — Feature Flags bằng Enums

### Compile-time feature selection

```rust
// filename: src/main.rs

// Feature flags = enum variants
#[derive(Debug, Clone, PartialEq)]
enum Feature {
    DarkMode,
    BetaSearch,
    NewCheckout,
    AIRecommendations,
}

#[derive(Debug)]
struct FeatureConfig {
    enabled: Vec<Feature>,
}

impl FeatureConfig {
    fn new(features: Vec<Feature>) -> Self {
        FeatureConfig { enabled: features }
    }

    fn is_enabled(&self, feature: &Feature) -> bool {
        self.enabled.contains(feature)
    }

    // Typed feature check → compiler ensures handling
    fn checkout_flow(&self) -> CheckoutFlow {
        if self.is_enabled(&Feature::NewCheckout) {
            CheckoutFlow::New
        } else {
            CheckoutFlow::Legacy
        }
    }
}

#[derive(Debug)]
enum CheckoutFlow { Legacy, New }

fn render_checkout(flow: &CheckoutFlow, total: u32) -> String {
    match flow {
        CheckoutFlow::Legacy => {
            format!("=== Classic Checkout ===\nTotal: {}đ\n[Pay Now]", total)
        }
        CheckoutFlow::New => {
            format!("╔══ Modern Checkout ══╗\n║ Total: {}đ          ║\n║ [💳 Pay] [QR] [COD] ║\n╚═════════════════════╝", total)
        }
    }
}

fn render_search(config: &FeatureConfig, query: &str) -> String {
    if config.is_enabled(&Feature::BetaSearch) {
        format!("🔍 AI-powered search for '{}' (beta)", query)
    } else {
        format!("Search results for '{}'", query)
    }
}

fn main() {
    // Production config
    let config = FeatureConfig::new(vec![
        Feature::DarkMode,
        Feature::NewCheckout,
    ]);

    println!("{}", render_checkout(&config.checkout_flow(), 500_000));
    println!();
    println!("{}", render_search(&config, "coffee"));
    println!();

    // Beta config
    let beta = FeatureConfig::new(vec![
        Feature::DarkMode,
        Feature::NewCheckout,
        Feature::BetaSearch,
        Feature::AIRecommendations,
    ]);

    println!("{}", render_search(&beta, "coffee"));
}
```

---

## 27.4 — Safe Type Evolution

### Adding enum variants safely

```rust
// filename: src/main.rs

// ═══ Version evolution ═══

// V1: original
#[derive(Debug)]
enum NotificationV1 {
    Email { to: String, subject: String },
    Sms { phone: String, message: String },
}

// V2: thêm Push + InApp mà không sửa V1 code patterns
#[derive(Debug)]
enum Notification {
    Email { to: String, subject: String, body: String },
    Sms { phone: String, message: String },
    // NEW in V2:
    Push { device_token: String, title: String, badge: u32 },
    InApp { user_id: u64, message: String, action_url: Option<String> },
}

// Handler phải xử lý MỌI variant (exhaustive match)
fn send(notification: &Notification) -> Result<String, String> {
    match notification {
        Notification::Email { to, subject, body } => {
            Ok(format!("📧 Sent email to {} (subject: {})", to, subject))
        }
        Notification::Sms { phone, message } => {
            if phone.len() < 10 { return Err("Invalid phone".into()); }
            Ok(format!("📱 Sent SMS to {}", phone))
        }
        Notification::Push { device_token, title, badge } => {
            Ok(format!("🔔 Push '{}' to device (badge: {})", title, badge))
        }
        Notification::InApp { user_id, message, action_url } => {
            let action = action_url.as_deref().unwrap_or("none");
            Ok(format!("💬 InApp to user #{}: '{}' [action: {}]", user_id, message, action))
        }
    }
}

// Priority: can evolve independently
fn priority(notification: &Notification) -> u32 {
    match notification {
        Notification::Email { .. } => 3,     // low
        Notification::Sms { .. } => 2,       // medium
        Notification::Push { .. } => 1,      // high
        Notification::InApp { .. } => 2,     // medium
    }
}

fn main() {
    let queue = vec![
        Notification::Email {
            to: "minh@co.com".into(),
            subject: "Welcome".into(),
            body: "Hello!".into(),
        },
        Notification::Push {
            device_token: "abc123".into(),
            title: "New order!".into(),
            badge: 1,
        },
        Notification::InApp {
            user_id: 42,
            message: "Your order shipped".into(),
            action_url: Some("/orders/123".into()),
        },
    ];

    // Sort by priority, send
    let mut sorted: Vec<_> = queue.iter().collect();
    sorted.sort_by_key(|n| priority(n));

    for n in sorted {
        match send(n) {
            Ok(msg) => println!("✅ [P{}] {}", priority(n), msg),
            Err(e) => println!("❌ {}", e),
        }
    }
}
```

---

## 27.5 — Deprecation & Migration Strategy

```rust
// filename: src/main.rs

// ═══ Deprecation with type system ═══

mod v1 {
    #[derive(Debug)]
    #[deprecated(since = "2.0", note = "Use v2::Order instead")]
    pub struct Order {
        pub id: u64,
        pub customer: String,
        pub total: u32,
    }
}

mod v2 {
    #[derive(Debug)]
    pub struct Order {
        pub id: u64,
        pub customer: Customer,
        pub lines: Vec<OrderLine>,
        pub status: OrderStatus,
    }

    #[derive(Debug)]
    pub struct Customer { pub name: String, pub email: String }

    #[derive(Debug)]
    pub struct OrderLine { pub product: String, pub price: u32, pub qty: u32 }

    #[derive(Debug)]
    pub enum OrderStatus { Draft, Confirmed, Shipped }
}

// Migration helper
#[allow(deprecated)]
impl From<v1::Order> for v2::Order {
    fn from(old: v1::Order) -> Self {
        v2::Order {
            id: old.id,
            customer: v2::Customer {
                name: old.customer,
                email: "unknown@migration.com".into(),
            },
            lines: vec![v2::OrderLine {
                product: "Migrated item".into(),
                price: old.total,
                qty: 1,
            }],
            status: v2::OrderStatus::Confirmed,
        }
    }
}

fn main() {
    // New code: use v2
    let order = v2::Order {
        id: 1,
        customer: v2::Customer { name: "Minh".into(), email: "minh@co.com".into() },
        lines: vec![
            v2::OrderLine { product: "Coffee".into(), price: 85_000, qty: 2 },
        ],
        status: v2::OrderStatus::Confirmed,
    };
    println!("V2: {:?}\n", order);

    // Migrate old data
    #[allow(deprecated)]
    let old = v1::Order { id: 99, customer: "Legacy".into(), total: 100_000 };
    let migrated: v2::Order = old.into();
    println!("Migrated: {:?}", migrated);
}
```

### Evolution checklist

```
1. ✅ Thêm enum variant       → compiler báo mọi match thiếu
2. ✅ Thêm struct field        → dùng Option<T> + Default
3. ✅ Rename field             → compiler báo mọi chỗ dùng
4. ✅ Thay đổi field type      → compiler báo type mismatch
5. ✅ Deprecate old type       → #[deprecated] + From migration
6. ✅ Feature flags            → enum + is_enabled()
7. ❌ Tránh: wildcard `_ =>`  → mất compiler guidance!
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Spot the problem

```rust
fn describe(shape: &Shape) -> &str {
    match shape {
        Shape::Circle { .. } => "circle",
        Shape::Rectangle { .. } => "rectangle",
        _ => "unknown",  // ← Problem?
    }
}
```
Nếu thêm `Shape::Triangle`, compiler có báo không?

<details><summary>✅ Lời giải</summary>

**Không!** Wildcard `_` catch Triangle → compiler im lặng → bug ẩn. Fix: bỏ wildcard, match explicit từng variant. Compiler sẽ báo khi thêm variant mới.

</details>

---

**Bài 2** (10 phút): Backward-compatible evolution

Bạn có struct:
```rust
struct Config { host: String, port: u16 }
```
Cần thêm: `max_connections`, `timeout_ms`, `tls_enabled`. Viết evolution để old code `Config::new("localhost", 8080)` vẫn hoạt động.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
struct Config {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_ms: u64,
    tls_enabled: bool,
}

impl Config {
    // Old API vẫn hoạt động!
    fn new(host: &str, port: u16) -> Self {
        Config {
            host: host.into(), port,
            max_connections: 100,  // sensible default
            timeout_ms: 5000,     // sensible default
            tls_enabled: false,   // sensible default
        }
    }

    // Builder for new fields
    fn with_max_connections(mut self, n: u32) -> Self { self.max_connections = n; self }
    fn with_timeout(mut self, ms: u64) -> Self { self.timeout_ms = ms; self }
    fn with_tls(mut self) -> Self { self.tls_enabled = true; self }
}

// Old code: Config::new("localhost", 8080)  ← works!
// New code: Config::new("localhost", 8080).with_tls().with_timeout(3000)
```

</details>

---

**Bài 3** (15 phút): Full evolution scenario

Bạn có system với `UserRole { Admin, User }`. Product manager yêu cầu:
1. Thêm `Moderator` role
2. Thêm `permissions: Vec<Permission>` cho mỗi role
3. Deprecate trực tiếp check `role == Admin` → dùng `has_permission(Permission::...)` thay thế

Implement toàn bộ evolution, đảm bảo old code path vẫn hoạt động qua migration.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// V2 roles với permissions
#[derive(Debug, Clone, PartialEq)]
enum UserRole { Admin, Moderator, User }

#[derive(Debug, Clone, PartialEq)]
enum Permission {
    ReadContent,
    WriteContent,
    DeleteContent,
    ManageUsers,
    ViewAnalytics,
    ModerateComments,
}

impl UserRole {
    fn default_permissions(&self) -> Vec<Permission> {
        match self {
            UserRole::Admin => vec![
                Permission::ReadContent, Permission::WriteContent,
                Permission::DeleteContent, Permission::ManageUsers,
                Permission::ViewAnalytics, Permission::ModerateComments,
            ],
            UserRole::Moderator => vec![
                Permission::ReadContent, Permission::WriteContent,
                Permission::ModerateComments,
            ],
            UserRole::User => vec![
                Permission::ReadContent,
            ],
        }
    }
}

struct User {
    name: String,
    role: UserRole,
    extra_permissions: Vec<Permission>,
}

impl User {
    fn has_permission(&self, perm: &Permission) -> bool {
        self.role.default_permissions().contains(perm)
            || self.extra_permissions.contains(perm)
    }

    // Migration: old is_admin() → delegates to permission
    #[deprecated(note = "Use has_permission(Permission::ManageUsers)")]
    fn is_admin(&self) -> bool {
        self.has_permission(&Permission::ManageUsers)
    }
}

fn main() {
    let admin = User {
        name: "Minh".into(), role: UserRole::Admin,
        extra_permissions: vec![],
    };
    let moderator = User {
        name: "Lan".into(), role: UserRole::Moderator,
        extra_permissions: vec![Permission::ViewAnalytics],
    };

    println!("{}: manage_users={}, moderate={}",
        admin.name,
        admin.has_permission(&Permission::ManageUsers),
        admin.has_permission(&Permission::ModerateComments));

    println!("{}: manage_users={}, analytics={}",
        moderator.name,
        moderator.has_permission(&Permission::ManageUsers),
        moderator.has_permission(&Permission::ViewAnalytics));
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Thêm field break mọi constructor" | Struct literal khắp nơi | Dùng `fn new()` + `..self.clone()`, không expose raw construction |
| "Wildcard `_` ẩn bugs" | Catch-all match | Match explicit, để compiler guide |
| "Old code dùng deprecated API" | Migration chưa xong | `#[deprecated]` + keep old API, document migration path |
| "Quá nhiều Option fields" | Evolution tích lũy | Group thành sub-structs (NotificationPrefs, DisplaySettings) |

---

## Tóm tắt

Chapter này dạy bạn **cập nhật bản đồ** mà không làm ai đi lạc — kỹ năng quan trọng nhất khi domain liên tục biến đổi:

- ✅ **Compiler-guided refactoring**: Thay đổi type → compiler như GPS báo **mọi chỗ** cần sửa. Zero runtime surprises.
- ✅ **Avoid wildcard `_ =>`**: Match explicit → compiler catch new variants. GPS không được bị tắt.
- ✅ **Non-breaking changes**: `Option<T>` + `Default` cho new fields. Thêm đường mới, không xóa đường cũ.
- ✅ **Extension traits**: Thêm methods mà không sửa original trait. Mở rộng khu phố mà không đụng trung tâm.
- ✅ **Feature flags bằng enums**: Typed, exhaustive, compile-time safe.
- ✅ **Deprecation**: `#[deprecated]` + `From` migration + gradual removal. Phá đường cũ từ từ, có đường mới thay thế.

---

## 🎉 Kết thúc Part IV!

Hãy nhìn lại hành trình qua thành phố DDD:

- **Ch 20**: Vẽ bản đồ tổng thể — DDD là gì, Ubiquitous Language, Bounded Contexts
- **Ch 21**: Cấu trúc tòa nhà — Onion Architecture, Ports & Adapters
- **Ch 22**: Thiết kế nội thất — Value Objects, Entities, Aggregates, State Machines
- **Ch 23**: Dây chuyền sản xuất — Workflows as type-safe pipelines
- **Ch 24**: Đường ray an toàn — Railway-Oriented Programming
- **Ch 25**: Trạm biên giới — Serialization & Anti-Corruption Layer
- **Ch 26**: Thư viện lưu trữ — Persistence & Repository Pattern
- **Ch 27**: Cập nhật bản đồ — Evolving the Design với compiler guidance

Bạn đã có **bộ công cụ hoàn chỉnh** để xây phần mềm domain-driven bằng Rust. Types là tên đường. Modules là ranh giới quận. Compiler là GPS. Và bạn — bạn là kiến trúc sư thành phố.

## Tiếp theo

→ **Part V: FP Patterns in Rust** bắt đầu! Chapter 28: **Abstract Algebra for Rust Developers** — Semigroups, Monoids. Bạn đã dùng chúng hàng ngày (`String::append`, `Vec::concat`) mà không biết tên!
