# Chapter 25 — Serialization & Anti-Corruption Layer

> **Bạn sẽ học được**:
> - **`serde`** — derive `Serialize`/`Deserialize` cho Rust types
> - **Domain types ↔ DTOs** — tách domain model khỏi wire format
> - **`From`/`Into`** trait cho mapping giữa layers
> - **Anti-Corruption Layer** — bảo vệ domain khỏi external systems
> - JSON, TOML, các format khác
> - Validation tại boundaries
>
> **Yêu cầu trước**: Chapter 16 (Traits, From/Into), Chapter 21 (Architecture), Chapter 22 (Domain Modeling).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Domain model **độc lập** khỏi wire format — thay API, đổi format, domain không đổi.

---

## 25.1 — `serde`: Serialize Everything

### Biên giới và hải quan

Hãy tưởng tượng domain của bạn là một quốc gia. Bên trong, mọi thứ gọn gàng: `Email`, `Money`, `OrderStatus` — types có nghĩa, validated, immutable. Nhưng thế giới bên ngoài không nói ngôn ngữ của bạn. API gửi JSON. Client gửi form data. Legacy system gửi CSV với field names đầy viết tắt. File cấu hình dùng TOML.

**Serialization** là trạm biên giới: nơi dữ liệu được "dịch" từ ngôn ngữ nội địa (Rust types) sang ngôn ngữ quốc tế (JSON, TOML), và ngược lại. **Anti-Corruption Layer** là hải quan: kiểm tra hàng hóa nhập khẩu (data từ bên ngoài), từ chối hàng không đạt chuẩn, và dịch sang format nội địa.

`serde` là công cụ để xây trạm biên giới này trong Rust.

### Setup

```toml
# Cargo.toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
```

### Derive cơ bản

```rust
// filename: src/main.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
    max_connections: u32,
}

fn main() {
    let config = Config {
        host: "localhost".into(),
        port: 8080,
        debug: true,
        max_connections: 100,
    };

    // Struct → JSON
    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("JSON:\n{}\n", json);

    // JSON → Struct
    let parsed: Config = serde_json::from_str(&json).unwrap();
    println!("Parsed: {:?}\n", parsed);

    // Struct → TOML
    let toml_str = toml::to_string_pretty(&config).unwrap();
    println!("TOML:\n{}", toml_str);
}
```

### Serde attributes

```rust
// filename: src/main.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // JSON convention
struct UserResponse {
    user_id: u64,
    full_name: String,
    email_address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<String>,

    #[serde(default)]  // nếu thiếu → dùng default
    is_active: bool,

    #[serde(rename = "type")]  // "type" là reserved keyword
    user_type: String,
}

fn main() {
    // Deserialize từ JSON (camelCase)
    let json = r#"{
        "userId": 1,
        "fullName": "Minh Nguyen",
        "emailAddress": "minh@co.com",
        "isActive": true,
        "type": "admin"
    }"#;

    let user: UserResponse = serde_json::from_str(json).unwrap();
    println!("{:#?}", user);
    // phone_number = None (missing in JSON, skip when serialize)

    // Serialize back (camelCase, skip None)
    println!("\n{}", serde_json::to_string_pretty(&user).unwrap());
}
```

### Enum serialization

```rust
// filename: src/main.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]  // adjacently tagged
enum PaymentEvent {
    Charged { amount: u32, currency: String },
    Refunded { amount: u32, reason: String },
    Failed { error_code: String },
}

fn main() {
    let events = vec![
        PaymentEvent::Charged { amount: 500_000, currency: "VND".into() },
        PaymentEvent::Refunded { amount: 100_000, reason: "Damaged".into() },
        PaymentEvent::Failed { error_code: "INSUFFICIENT_FUNDS".into() },
    ];

    let json = serde_json::to_string_pretty(&events).unwrap();
    println!("{}", json);
    // [
    //   { "type": "Charged", "data": { "amount": 500000, "currency": "VND" } },
    //   { "type": "Refunded", "data": { "amount": 100000, "reason": "Damaged" } },
    //   ...
    // ]

    // Roundtrip: JSON → Vec<PaymentEvent>
    let parsed: Vec<PaymentEvent> = serde_json::from_str(&json).unwrap();
    println!("\nParsed {} events", parsed.len());
}
```

---

## ✅ Checkpoint 25.1

> Ghi nhớ:
> 1. `#[derive(Serialize, Deserialize)]` = auto JSON/TOML/etc
> 2. `#[serde(rename_all = "camelCase")]` cho API conventions
> 3. `#[serde(tag = "type")]` cho enum = discriminated union trong JSON
> 4. `skip_serializing_if`, `default`, `rename` = fine-tune

---

## 25.2 — Domain Types ≠ DTOs

### Tại sao không nên để domain types đi qua biên giới?

Hãy nghĩ thế này: nếu bạn cho công dân (domain types) đi ra nước ngoài mà không có hộ chiếu (DTO) — họ phải mang theo GIẤY Tờ GỐC. Nước ngoài thay đổi luật nhập cảnh? Domain types bị ảnh hưởng trực tiếp. Ngược lại, domain thêm field mới? API break vì client chưa biết field đó.

DTO (Data Transfer Object) là "hộ chiếu" — một bản sao đơn giản của domain data, chỉ chứa thông tin cần thiết cho chuyến đi (API call). Domain thay đổi? Chỉ cập nhật mapping, client không biết. API thay đổi? Chỉ cập nhật DTO, domain không biết.

### Vấn đề: Domain types + serde = coupling

```rust
// ❌ BAD: Domain type trực tiếp serialize
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: u64,
    customer: String,
    total: u32,
    // API thay đổi? → Domain thay đổi!
    // Domain thêm field? → API break!
}
```

### Giải pháp: DTO = Data Transfer Object

```rust
// filename: src/main.rs
use serde::{Serialize, Deserialize};

// ═══ DOMAIN (pure, no serde) ═══
mod domain {
    #[derive(Debug, Clone)]
    pub struct Email(String);
    impl Email {
        pub fn new(value: &str) -> Result<Self, String> {
            if value.contains('@') { Ok(Email(value.to_lowercase())) }
            else { Err("Invalid email".into()) }
        }
        pub fn value(&self) -> &str { &self.0 }
    }

    #[derive(Debug, Clone)]
    pub struct Money(u64);
    impl Money {
        pub fn new(amount: u64) -> Self { Money(amount) }
        pub fn value(&self) -> u64 { self.0 }
    }

    #[derive(Debug, Clone)]
    pub struct Order {
        pub id: u64,
        pub customer_name: String,
        pub customer_email: Email,
        pub total: Money,
        pub status: OrderStatus,
    }

    #[derive(Debug, Clone)]
    pub enum OrderStatus { Draft, Confirmed, Shipped }
}

// ═══ DTOs (serde, API format) ═══
mod dto {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OrderDto {
        pub id: u64,
        pub customer_name: String,
        pub customer_email: String,  // String, không phải Email!
        pub total_amount: u64,       // u64, không phải Money!
        pub status: String,          // String, không phải enum!
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateOrderRequest {
        pub customer_name: String,
        pub customer_email: String,
        pub items: Vec<OrderItemDto>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OrderItemDto {
        pub product_name: String,
        pub price: u64,
        pub quantity: u32,
    }
}

// ═══ MAPPING: Domain ↔ DTO ═══
impl From<domain::Order> for dto::OrderDto {
    fn from(order: domain::Order) -> Self {
        dto::OrderDto {
            id: order.id,
            customer_name: order.customer_name,
            customer_email: order.customer_email.value().to_string(),
            total_amount: order.total.value(),
            status: format!("{:?}", order.status).to_lowercase(),
        }
    }
}

impl TryFrom<dto::CreateOrderRequest> for (String, domain::Email) {
    type Error = String;
    fn try_from(req: dto::CreateOrderRequest) -> Result<Self, String> {
        let email = domain::Email::new(&req.customer_email)?;
        Ok((req.customer_name, email))
    }
}

fn main() {
    // Domain → DTO → JSON (outbound)
    let order = domain::Order {
        id: 42,
        customer_name: "Minh".into(),
        customer_email: domain::Email::new("minh@co.com").unwrap(),
        total: domain::Money::new(500_000),
        status: domain::OrderStatus::Confirmed,
    };

    let dto: dto::OrderDto = order.into();
    let json = serde_json::to_string_pretty(&dto).unwrap();
    println!("Outbound JSON:\n{}\n", json);

    // JSON → DTO → Domain (inbound)
    let input_json = r#"{
        "customerName": "Lan",
        "customerEmail": "lan@co.com",
        "items": [
            {"productName": "Coffee", "price": 35000, "quantity": 2}
        ]
    }"#;

    let request: dto::CreateOrderRequest = serde_json::from_str(input_json).unwrap();
    match <(String, domain::Email)>::try_from(request) {
        Ok((name, email)) => {
            println!("Inbound: {} <{}>", name, email.value());
        }
        Err(e) => println!("❌ {}", e),
    }
}
```

### Tại sao tách?

| | Domain trực tiếp | DTO riêng |
|---|---|---|
| API thay đổi? | Domain phải sửa ❌ | Chỉ sửa DTO ✅ |
| Domain thêm field? | API có thể break ❌ | Mapping quyết định expose gì ✅ |
| Validation? | Ai validate? ❌ | DTO → Domain = validate tại boundary ✅ |
| Testing? | Cần JSON fixtures ❌ | Domain test pure, DTO test riêng ✅ |

---

## 25.3 — Anti-Corruption Layer

### Bảo vệ domain khỏi external systems

Bây giờ hãy nói về hải quan — Anti-Corruption Layer. Legacy system từ đối tác gửi data với field names như `cust_no`, `first_nm`, `acct_status: -1`. Bạn có muốn vứt rác này vào giữa domain không? Không. ACL là trạm hải quan: nhận hàng (JSON xấu), kiểm định (validate), dịch sang format nội địa (domain types), từ chối hàng lỗi:

```rust
// filename: src/main.rs
use serde::{Serialize, Deserialize};

// ═══ External system format (legacy, ugly) ═══
mod external {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct LegacyCustomer {
        pub cust_no: String,       // "C-00042"
        pub first_nm: String,      // "MINH"
        pub last_nm: String,       // "NGUYEN"
        pub email_addr: String,    // "MINH@CO.COM  "
        pub acct_status: i32,      // 1=active, 0=inactive, -1=suspended
        pub cr_limit: f64,         // 5000000.0
    }
}

// ═══ Our domain (clean) ═══
mod domain {
    #[derive(Debug, Clone)]
    pub struct CustomerId(pub u64);

    #[derive(Debug, Clone)]
    pub struct Customer {
        pub id: CustomerId,
        pub name: String,
        pub email: String,
        pub status: CustomerStatus,
        pub credit_limit: u64,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CustomerStatus { Active, Inactive, Suspended }
}

// ═══ Anti-Corruption Layer ═══
mod acl {
    use super::{external, domain};

    pub fn translate_customer(legacy: external::LegacyCustomer) -> Result<domain::Customer, String> {
        // Parse ID: "C-00042" → 42
        let id = legacy.cust_no
            .strip_prefix("C-")
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| format!("Invalid customer number: {}", legacy.cust_no))?;

        // Name: "MINH" "NGUYEN" → "Minh Nguyen"
        let name = format!(
            "{} {}",
            capitalize(&legacy.first_nm),
            capitalize(&legacy.last_nm),
        );

        // Email: normalize
        let email = legacy.email_addr.trim().to_lowercase();

        // Status: int → enum
        let status = match legacy.acct_status {
            1 => domain::CustomerStatus::Active,
            0 => domain::CustomerStatus::Inactive,
            -1 => domain::CustomerStatus::Suspended,
            other => return Err(format!("Unknown status: {}", other)),
        };

        // Credit limit: f64 → u64
        let credit_limit = legacy.cr_limit as u64;

        Ok(domain::Customer {
            id: domain::CustomerId(id),
            name, email, status, credit_limit,
        })
    }

    fn capitalize(s: &str) -> String {
        let lower = s.to_lowercase();
        let mut chars = lower.chars();
        match chars.next() {
            Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            None => String::new(),
        }
    }
}

fn main() {
    // Simulate receiving legacy data
    let legacy_json = r#"{
        "cust_no": "C-00042",
        "first_nm": "MINH",
        "last_nm": "NGUYEN",
        "email_addr": "MINH@CO.COM  ",
        "acct_status": 1,
        "cr_limit": 5000000.0
    }"#;

    let legacy: external::LegacyCustomer = serde_json::from_str(legacy_json).unwrap();
    println!("Legacy: {:?}\n", legacy);

    match acl::translate_customer(legacy) {
        Ok(customer) => {
            println!("Domain: {:?}", customer);
            // Customer { id: CustomerId(42), name: "Minh Nguyen",
            //   email: "minh@co.com", status: Active, credit_limit: 5000000 }
        }
        Err(e) => println!("❌ Translation failed: {}", e),
    }
}
```

### ACL Pattern diagram

```
┌──────────────┐     ┌──────────────────┐     ┌──────────────┐
│  External    │     │  Anti-Corruption  │     │   Domain     │
│  System      │────→│  Layer (ACL)      │────→│   Model      │
│  (legacy)    │     │  translate()      │     │  (clean)     │
│              │     │  validate()       │     │              │
│  cust_no     │     │  normalize()      │     │  CustomerId  │
│  first_nm    │     │                   │     │  Customer    │
│  acct_status │     │  From/TryFrom     │     │  Status enum │
└──────────────┘     └──────────────────┘     └──────────────┘
     Ugly format          Translation              Clean types
```

---

## 25.4 — Validation at Boundaries

DTO chấp nhận mọi thứ từ bên ngoài (strings, numbers âm, giá trị vô nghĩa). Domain types chỉ chấp nhận giá trị hợp lệ. Phần chuyển đổi giữa hai lớp này — boundary — là nơi validate và thu gom lỗi:

```rust
// filename: src/main.rs
use serde::Deserialize;

// DTO: chấp nhận MỌI input từ user
#[derive(Debug, Deserialize)]
struct CreateProductRequest {
    name: String,
    price: f64,
    category: String,
    stock: i32,
}

// Domain: chỉ valid values
#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
    category: ProductCategory,
    stock: u32,
}

#[derive(Debug)]
enum ProductCategory { Electronics, Books, Food, Clothing }

// Validate + convert tại boundary
fn validate_product(req: CreateProductRequest) -> Result<Product, Vec<String>> {
    let mut errors = vec![];

    let name = req.name.trim().to_string();
    if name.len() < 2 || name.len() > 200 {
        errors.push("Name: 2-200 chars required".into());
    }

    if req.price <= 0.0 || req.price > 1_000_000_000.0 {
        errors.push("Price: must be positive and under 1 billion".into());
    }

    let category = match req.category.to_lowercase().as_str() {
        "electronics" => Some(ProductCategory::Electronics),
        "books" => Some(ProductCategory::Books),
        "food" => Some(ProductCategory::Food),
        "clothing" => Some(ProductCategory::Clothing),
        other => { errors.push(format!("Unknown category: {}", other)); None }
    };

    if req.stock < 0 {
        errors.push("Stock cannot be negative".into());
    }

    if errors.is_empty() {
        Ok(Product {
            name,
            price: req.price as u32,
            category: category.unwrap(),
            stock: req.stock as u32,
        })
    } else {
        Err(errors)
    }
}

fn main() {
    // Valid input
    let json = r#"{"name": "Laptop Pro", "price": 25000000, "category": "Electronics", "stock": 50}"#;
    let req: CreateProductRequest = serde_json::from_str(json).unwrap();
    println!("Valid: {:?}\n", validate_product(req));

    // Invalid input — multiple errors
    let json = r#"{"name": "X", "price": -500, "category": "toys", "stock": -10}"#;
    let req: CreateProductRequest = serde_json::from_str(json).unwrap();
    match validate_product(req) {
        Err(errors) => {
            println!("Errors:");
            for e in &errors { println!("  ❌ {}", e); }
        }
        Ok(_) => unreachable!(),
    }
}
```

> **💡 Boundary rule**: External data (JSON, CSV, user input) = **untrusted**. Validate + convert ở boundary. Bên trong domain = **trusted** types (newtypes, smart constructors).

---

## 25.5 — Multiple Formats

```rust
// filename: src/main.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    app_name: String,
    version: String,
    database: DatabaseConfig,
    features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    name: String,
    pool_size: u32,
}

fn main() {
    let config = AppConfig {
        app_name: "MyApp".into(),
        version: "1.0.0".into(),
        database: DatabaseConfig {
            host: "localhost".into(),
            port: 5432,
            name: "mydb".into(),
            pool_size: 10,
        },
        features: vec!["auth".into(), "cache".into(), "metrics".into()],
    };

    // JSON
    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("═══ JSON ═══\n{}\n", json);

    // TOML
    let toml_str = toml::to_string_pretty(&config).unwrap();
    println!("═══ TOML ═══\n{}\n", toml_str);

    // Roundtrip: TOML → struct → JSON
    let from_toml: AppConfig = toml::from_str(&toml_str).unwrap();
    let back_to_json = serde_json::to_string_pretty(&from_toml).unwrap();
    println!("═══ TOML → JSON roundtrip ═══\n{}", back_to_json);
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Serde attributes

Làm sao để struct sau serialize thành JSON `{"firstName": "Minh", "age": 25}` (bỏ `None` fields)?

```rust
struct User { first_name: String, last_name: Option<String>, age: u32 }
```

<details><summary>✅ Lời giải</summary>

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct User {
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    age: u32,
}
```

</details>

---

**Bài 2** (10 phút): DTO mapping

External API trả:
```json
{"usr_id": 42, "usr_name": "MINH NGUYEN", "usr_email": "MINH@CO.COM", "is_del": false}
```

Viết: (1) `ExternalUserDto` với serde, (2) `User` domain type, (3) `From<ExternalUserDto> for User`.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
#[derive(Deserialize)]
struct ExternalUserDto {
    usr_id: u64,
    usr_name: String,
    usr_email: String,
    is_del: bool,
}

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
    is_active: bool,
}

impl From<ExternalUserDto> for User {
    fn from(dto: ExternalUserDto) -> Self {
        User {
            id: dto.usr_id,
            name: title_case(&dto.usr_name),
            email: dto.usr_email.trim().to_lowercase(),
            is_active: !dto.is_del,  // invert
        }
    }
}

fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|w| {
            let mut c = w.to_lowercase().chars();
            match c.next() {
                Some(first) => first.to_uppercase().to_string() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
```

</details>

---

**Bài 3** (15 phút): Full ACL pipeline

Viết Anti-Corruption Layer cho payment gateway:
- External format: `{"tx_id": "TX001", "amt_cents": 50000, "curr": "VND", "stat": "OK", "ts": "2024-01-15T10:30:00Z"}`
- Domain: `Payment { id, amount: Money, currency: Currency, status: PaymentStatus, timestamp }`
- ACL: parse, validate, normalize. Handle errors.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct ExternalPayment {
    tx_id: String,
    amt_cents: i64,
    curr: String,
    stat: String,
    ts: String,
}

#[derive(Debug)]
struct Payment {
    id: String,
    amount: u64,
    currency: Currency,
    status: PaymentStatus,
    timestamp: String,
}

#[derive(Debug)]
enum Currency { VND, USD, EUR }

#[derive(Debug)]
enum PaymentStatus { Success, Failed, Pending }

fn translate(ext: ExternalPayment) -> Result<Payment, Vec<String>> {
    let mut errors = vec![];

    if ext.amt_cents <= 0 { errors.push("Amount must be positive".into()); }

    let currency = match ext.curr.as_str() {
        "VND" => Some(Currency::VND),
        "USD" => Some(Currency::USD),
        "EUR" => Some(Currency::EUR),
        c => { errors.push(format!("Unknown currency: {}", c)); None }
    };

    let status = match ext.stat.as_str() {
        "OK" | "SUCCESS" => Some(PaymentStatus::Success),
        "FAIL" | "ERROR" => Some(PaymentStatus::Failed),
        "PENDING" => Some(PaymentStatus::Pending),
        s => { errors.push(format!("Unknown status: {}", s)); None }
    };

    if errors.is_empty() {
        Ok(Payment {
            id: ext.tx_id,
            amount: ext.amt_cents as u64,
            currency: currency.unwrap(),
            status: status.unwrap(),
            timestamp: ext.ts,
        })
    } else {
        Err(errors)
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| `unknown field` khi deserialize | JSON có fields không trong struct | `#[serde(deny_unknown_fields)]` hoặc bỏ qua |
| Domain type dùng serde trực tiếp | Coupling domain ↔ wire format | Tách DTO, dùng `From`/`TryFrom` |
| Enum serialize không đẹp | Mặc định: `{"Variant": {...}}` | `#[serde(tag = "type")]` hoặc `rename_all` |
| Float → int precision | `f64` → `u32` mất precision | Dùng cents/đồng (integer), không dùng float cho tiền |

---

## Tóm tắt

Chapter này dạy bạn xây **trạm biên giới** cho domain — nơi dữ liệu ra vào được kiểm soát chặt chẽ:

- ✅ **`serde`**: `#[derive(Serialize, Deserialize)]` + attributes — xây trạm biên giới nhanh chóng.
- ✅ **Domain ≠ DTO**: Domain types là công dân, DTOs là hộ chiếu. Map qua `From`/`TryFrom` — domain và API độc lập.
- ✅ **Anti-Corruption Layer**: Hải quan kiểm hàng nhập. Legacy format (xấu) → ACL (dịch, validate) → Domain (sạch).
- ✅ **Validation at boundaries**: External data = untrusted. Validate + convert ở biên giới. Bên trong domain = trusted.
- ✅ **Multiple formats**: Cùng struct → JSON, TOML, MessagePack. Serde makes it trivial.

## Tiếp theo

Dữ liệu đã qua biên giới an toàn — giờ cần **lưu trữ** nó. Như thư viện lưu sách: bạn mang sách đến, thủ thư biết xếp ở đâu, và tìm lại khi bạn cần.

→ Chapter 26: **Persistence & Repository Pattern** — bạn sẽ implement Repository trait với real database, mapping Domain ↔ Persistence models, transaction handling, và testing với in-memory stores.
