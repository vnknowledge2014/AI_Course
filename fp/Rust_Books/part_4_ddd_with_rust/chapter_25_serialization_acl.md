# Chapter 25 — Serialization & Anti-Corruption Layer

> **Bạn sẽ học được**:
> - **`serde`** — Thần khí của Rust để tự động biến đổi (Serialize/Deserialize) Dữ liệu.
> - **Domain types ↔ DTOs** — Tuyệt kỹ tách biệt Hoàn toàn Code cốt lõi khỏi định dạng của API.
> - Sử dụng Trait **`From`/`Into`** để làm trạm trung chuyển giữa các lớp (Layers).
> - **Anti-Corruption Layer (Lớp chống suy đồi)** — Xây dựng một Bức tường thành bảo vệ Code của bạn khỏi đống rác từ các hệ thống cũ (Legacy Systems).
> - Cách xử lý JSON, TOML một cách nhẹ nhàng.
> - Validate ngay tại Biên giới (Boundaries).
>
> **Yêu cầu trước**: Chapter 16 (Traits, From/Into), Chapter 21 (Architecture), Chapter 22 (Domain Modeling).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Domain Model của bạn sẽ **hoàn toàn độc lập** với thế giới bên ngoài. Cho dù API đổi cấu trúc, hay sếp yêu cầu đổi JSON sang XML, Code Cốt lõi của bạn KHÔNG PHẢI SỬA MỘT DÒNG NÀO!

---

## 25.1 — `serde`: Trạm Hải Quan của Rust

### Khái niệm Biên giới và Hải quan

Hãy tưởng tượng Domain (Code Cốt Lõi) của bạn là một Vương quốc. Bên trong Vương quốc, mọi thứ được quy định rất nghiêm ngặt: `Email`, `Money`, `OrderStatus` — Các kiểu dữ liệu này được xác thực kỹ càng, có nghĩa rõ ràng, và bất biến (Immutable).

Nhưng thế giới bên ngoài thì hỗn loạn và không nói ngôn ngữ của bạn. API của Front-end thì gửi JSON. Client cũ rích thì gửi form data. Hệ thống Legacy của đối tác thì gửi file CSV với những cái tên viết tắt xấu xí như `cust_no`, `cr_limit`.

**Serialization (Tuần tự hóa)** chính là Trạm Biên giới: Nơi dữ liệu được "dịch" từ ngôn ngữ nội địa (Rust Structs) sang ngôn ngữ quốc tế (JSON, TOML), và ngược lại. 

**Anti-Corruption Layer (Lớp chống suy đồi)** chính là Nhân viên Hải quan: Kiểm tra hàng hóa nhập khẩu, từ chối hàng lậu (dữ liệu sai), và phiên dịch sang ngôn ngữ nội địa.

Trong Rust, `serde` là công cụ quyền năng nhất để xây dựng Trạm Biên Giới này.

### Setup (Thiết lập)

Thêm các dòng sau vào file cấu hình:

```toml
# Cargo.toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
```

### Derive Cơ bản: Phép màu tự động

Chỉ cần gắn thẻ `#[derive(Serialize, Deserialize)]`, Rust sẽ tự động viết code dịch Struct của bạn ra đủ mọi loại ngôn ngữ!

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
        host: "localhost".into(), port: 8080,
        debug: true, max_connections: 100,
    };

    // Dịch từ Struct → JSON
    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("JSON:\n{}\n", json);

    // Dịch từ JSON → Struct (Đọc từ file hoặc mạng)
    let parsed: Config = serde_json::from_str(&json).unwrap();
    println!("Đọc ngược lại: {:?}\n", parsed);
}
```

### Trang điểm cho JSON (Serde Attributes)

Thế giới bên ngoài thường thích dùng kiểu chữ `camelCase` (Ví dụ: `userId`), trong khi Rust lại dùng `snake_case` (Ví dụ: `user_id`). Serde cho phép bạn "Hóa trang" dữ liệu lúc đi qua biên giới cực kỳ dễ dàng bằng các Attributes:

```rust
// filename: src/main.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // Ép TẤT CẢ các tên biến thành camelCase khi xuất ra JSON
struct UserResponse {
    user_id: u64,
    full_name: String,
    
    // Nếu biến này bằng None, đừng in nó ra JSON luôn cho đỡ rác
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<String>,

    // Nếu JSON người ta gửi không có trường này, thì tự động gán false (hoặc giá trị mặc định)
    #[serde(default)]
    is_active: bool,

    // Trong Rust chữ `type` bị cấm, nên đặt là `user_type`, nhưng xuất ra JSON thì đổi thành `type`
    #[serde(rename = "type")]
    user_type: String,
}
```

Và đây là kết quả khi nạp JSON vào:

```rust
fn main() {
    // Nhận JSON chuẩn Front-end (camelCase, thiếu vài field)
    let json = r#"{
        "userId": 1,
        "fullName": "Minh Nguyen",
        "type": "admin"
    }"#;

    let user: UserResponse = serde_json::from_str(json).unwrap();
    println!("{:#?}", user);
    // Nhờ có attribute, nó sẽ tự động hiểu:
    // - is_active = false (Do thiếu trong JSON)
    // - phone_number = None
}
```

---

## ✅ Checkpoint 25.1

> Ghi nhớ:
> 1. `#[derive(Serialize, Deserialize)]` là phép thuật để chuyển đổi Struct ↔ JSON/TOML.
> 2. Dùng `#[serde(rename_all = "camelCase")]` để chiều lòng Front-end dev.
> 3. Các attributes như `skip_serializing_if`, `default`, `rename` giúp bạn tinh chỉnh biên giới mà không cần đổi tên biến trong Code Rust.

---

## 25.2 — Domain Types ≠ DTOs (Sự khác biệt sống còn)

### Tại sao không nên cho Công Dân đi ra nước ngoài?

Bạn đã biết dùng Serde. Bạn nghĩ: *"Ôi dễ thế, tôi sẽ gắn thẳng cái `#derive(Serialize)` vào cái Struct Core Domain của tôi để ném ra thành API luôn!"*

**ĐỪNG LÀM THẾ! Đó là một thảm họa.**

Hãy nghĩ thế này: Nếu bạn cho Công dân (Domain Types) của bạn đi ra nước ngoài, mà bạn lại ép họ phải mang bộ mặt của nước ngoài (Dùng `rename_all="camelCase"` vào Core Domain). Rồi nhỡ API đổi chuẩn, bắt buộc phải trả về `snake_case` thì sao? Bạn phải chui sâu vào Code Cốt lõi để sửa một cái Attribute râu ria? Ngược lại, nếu Core Domain cần thêm một biến `secret_password` cho thuật toán, chẳng lẽ bạn lại vô tình làm lọt nó ra API chỉ vì quên gắn thẻ `skip`?

### Giải pháp: DTO (Hộ chiếu)

**DTO (Data Transfer Object)** đóng vai trò là "Hộ chiếu". Nó là một bản sao Đơn Giản Hóa của Domain Data, chỉ chứa đúng những thông tin mà phía bên kia mạng cần, và được định dạng chuẩn theo ý họ.

Khi Core Domain thay đổi? Bạn chỉ cập nhật cách chép dữ liệu (Mapping), API Client không bị ảnh hưởng.
Khi API thay đổi? Bạn sửa DTO, Core Domain không hề hay biết!

### Tách biệt 2 Thế Giới

**Thế giới 1: Domain Tinh khiết (Không hề có Serde)**

```rust
// filename: src/main.rs

// ═══ DOMAIN (Hoàn toàn Tinh khiết, Không có tí Serde nào) ═══
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
    pub struct Order {
        pub id: u64,
        pub customer_email: Email, // Dùng Value Object xịn
    }
}
```

**Thế giới 2: DTO (Đầy đủ đồ nghề Serde, nhưng ngu ngốc về mặt Logic)**

```rust
// ═══ DTOs (Nơi giao tiếp với API) ═══
mod dto {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OrderDto {
        pub id: u64,
        pub customer_email: String,  // Chỉ là String rỗng tuếch, để vứt qua mạng cho lẹ
    }
}
```

### Trạm Trung Chuyển: Biến Hình với Trait `From`

Để DTO và Domain nói chuyện được với nhau, chúng ta dùng Trait `From` và `TryFrom` của Rust để dịch qua dịch lại.

```rust
// ═══ MAPPING: Dịch từ Domain → DTO (Để xuất JSON ra ngoài) ═══
impl From<domain::Order> for dto::OrderDto {
    fn from(order: domain::Order) -> Self {
        dto::OrderDto {
            id: order.id,
            customer_email: order.customer_email.value().to_string(), // Mở hộp Email ra lấy String
        }
    }
}
```

Và đây là cách chúng hoạt động mượt mà:

```rust
fn main() {
    // 1. Bên trong hệ thống xử lý logic xịn
    let order = domain::Order {
        id: 42,
        customer_email: domain::Email::new("minh@co.com").unwrap(),
    };

    // 2. Tới biên giới, Biến hình thành DTO (.into)
    let dto: dto::OrderDto = order.into();
    
    // 3. Đóng gói ra Tàu hỏa (Thành JSON)
    let json = serde_json::to_string_pretty(&dto).unwrap();
    println!("Outbound JSON:\n{}", json);
}
```

### Tại sao phải khổ sở tách làm 2 vậy?

| Tình huống | Dùng chung 1 Struct (Sai) | Tách riêng DTO (Đúng) |
|---|---|---|
| **API đổi luật (VD: Đổi tên field)** | Code Cốt Lõi bị dơ dáy vì phải sửa theo API ❌ | Chỉ sửa DTO, Core Domain vẫn thanh cao ✅ |
| **Bảo mật** | Vô tình trả lọt `password` ra API ❌ | Mapping quyết định chỉ expose cái gì an toàn ✅ |
| **Test** | Viết Test rất khó vì phải mồi bằng chuỗi JSON rườm rà ❌ | Code Cốt lõi Test siêu nhanh vì không dính tới JSON ✅ |

---

## 25.3 — Anti-Corruption Layer (Bức Tường Chống Suy Đồi)

### Hệ thống Đối Tác Gửi Đống Rác (Legacy Systems)

Hãy nói sâu hơn về Hải quan — **Anti-Corruption Layer (ACL)**. Giả sử hệ thống Ngân hàng cũ rích của Đối tác gọi vào API của bạn. Họ gửi một cục JSON xấu đau đớn với tên biến chắp vá như: `cust_no`, `first_nm`, `acct_status: -1`. 

Bạn có muốn cho "Cái thứ hôi hám" đó bò vào trong Logic Đẹp Đẽ của bạn không? Chắc chắn KHÔNG.

ACL chính là lớp Hải quan: Nhận hàng (JSON xấu) → Kiểm định → Lau chùi, Đánh bóng (Dịch sang format nội địa) → Từ chối hàng lậu.

### Xây dựng ACL bằng Code

```rust
// filename: src/main.rs

// 1. Định dạng Của Hệ Thống Cũ (Xấu xí, ngu ngốc)
mod external {
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    pub struct LegacyCustomer {
        pub cust_no: String,       // VD: "C-00042"
        pub first_nm: String,      // VD: "MINH"
        pub acct_status: i32,      // 1=active, 0=inactive, -1=suspended
    }
}

// 2. Định dạng Tinh Hoa của bạn
mod domain {
    #[derive(Debug)]
    pub struct Customer {
        pub id: u64,
        pub name: String,
        pub status: CustomerStatus,
    }
    
    #[derive(Debug)]
    pub enum CustomerStatus { Active, Inactive, Suspended }
}
```

**Đây chính là Lớp ACL:** Nơi nó hốt rác và biến thành Vàng.

```rust
mod acl {
    use super::{external, domain};

    // Hàm Dịch Thuật
    pub fn translate_customer(legacy: external::LegacyCustomer) -> Result<domain::Customer, String> {
        
        // Cắt bỏ chữ "C-", lấy số 42 ra
        let id = legacy.cust_no
            .strip_prefix("C-")
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| format!("Invalid ID: {}", legacy.cust_no))?;

        // Format lại tên cho đẹp: "MINH" -> "Minh"
        let name = capitalize(&legacy.first_nm);

        // Map số xấu xí thành Enum Xịn
        let status = match legacy.acct_status {
            1 => domain::CustomerStatus::Active,
            0 => domain::CustomerStatus::Inactive,
            -1 => domain::CustomerStatus::Suspended,
            other => return Err(format!("Unknown status: {}", other)),
        };

        Ok(domain::Customer { id, name, status })
    }

    fn capitalize(s: &str) -> String {
        let mut chars = s.to_lowercase().chars();
        match chars.next() {
            Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            None => String::new(),
        }
    }
}
```

Khi chạy thực tế:

```rust
fn main() {
    let legacy_json = r#"{ "cust_no": "C-00042", "first_nm": "MINH", "acct_status": 1 }"#;
    let legacy: external::LegacyCustomer = serde_json::from_str(legacy_json).unwrap();
    
    // Tường lửa ACL sẽ xử lý
    match acl::translate_customer(legacy) {
        Ok(customer) => println!("Tài sản Domain Sạch Sẽ: {:?}", customer),
        Err(e) => println!("❌ Bắt được Hàng Lậu: {}", e),
    }
}
```

### Sơ đồ Chiến Thuật ACL

```text
┌──────────────┐     ┌──────────────────┐     ┌──────────────┐
│  Hệ Thống Cũ │     │  Anti-Corruption │     │   Domain     │
│  (Bên Ngoài) │────→│  Layer (ACL)     │────→│   Model      │
│              │     │  translate()     │     │  (Xịn xò)    │
│  cust_no     │     │  normalize()     │     │  Customer    │
│  acct_status │     │                  │     │  Status Enum │
└──────────────┘     └──────────────────┘     └──────────────┘
  Dữ liệu Rác           Dịch & Lau Chùi           Ngọc quý
```

---

## 25.4 — Validation ngay tại Biên Giới

DTO là đứa "Nhận mọi thứ". Nó sẽ vui vẻ nhận một chuỗi `name: "A"` hay `price: -100` từ mạng. Nhưng Domain Types thì RẤT KÉN CHỌN (Nó dùng Newtype/Smart Constructors để đảm bảo đã được tạo ra thì chắc chắn phải đúng).

Vì vậy, **Ranh giới giữa DTO và Domain chính là nơi Tốt nhất để Validate và Bắt lỗi đầu vào**.

```rust
// filename: src/main.rs
use serde::Deserialize;

// DTO: Ngu ngốc, ai cho gì lấy nấy
#[derive(Debug, Deserialize)]
struct CreateProductRequest {
    name: String, price: f64,
}

// Domain: Tinh xảo, chặt chẽ
#[derive(Debug)]
struct Product {
    name: String, price: u32,
}

// Validation tại Biên Giới: Thu gom lỗi
fn validate_product(req: CreateProductRequest) -> Result<Product, Vec<String>> {
    let mut errors = vec![];

    if req.name.trim().len() < 2 { errors.push("Tên quá ngắn".into()); }
    if req.price <= 0.0 { errors.push("Giá phải lớn hơn 0".into()); }

    if errors.is_empty() {
        Ok(Product { name: req.name, price: req.price as u32 })
    } else {
        Err(errors)
    }
}
```

> **💡 Quy luật Bất Bãi**: Mọi dữ liệu từ bên ngoài (JSON, CSV, Input người dùng) đều là **Kẻ Dối Trá (Untrusted)**. Phải dùng DTO để hứng lấy nó, sau đó Kiểm Định ở Biên giới, rồi mới biến nó thành Công dân (Domain) **Trusted**.

---

## 25.5 — Multiple Formats (Khả năng Đa Ngôn Ngữ)

Một khi bạn đã tách Domain khỏi DTO, bạn có thể biến DTO thành mọi loại định dạng mà các hệ thống khác yêu cầu (JSON cho Web, TOML cho cấu hình Server, MessagePack cho tốc độ cao) chỉ bằng 1 dòng code!

```rust
// filename: src/main.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    app_name: String,
    version: String,
}

fn main() {
    let config = AppConfig { app_name: "MyApp".into(), version: "1.0.0".into() };

    // Bắn ra JSON
    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("JSON:\n{}", json);

    // Bắn ra TOML (Dành cho DevOps)
    let toml_str = toml::to_string_pretty(&config).unwrap();
    println!("TOML:\n{}", toml_str);
}
```

---

## 🏋️ Bài tập

**Bài 1** (15 phút): Hệ thống Cổng thanh toán (Payment Gateway ACL)

Viết Lớp ACL cho một Cổng thanh toán.
- Bên ngoài gửi JSON siêu xấu: `{"tx_id": "TX001", "amt_cents": 50000, "curr": "VND", "stat": "OK"}`
- Bên trong Domain muốn gọn gàng: `Payment { id, amount: u64, currency: Enum, status: Enum }`
- Viết 1 Struct DTO để hứng JSON, và 1 hàm `translate` để biến DTO thành Domain. Trả về Lỗi nếu tiền < 0 hoặc sai Enum.

<details><summary>✅ Gợi ý Lời giải</summary>

Viết `struct ExternalPayment` có derive `Deserialize`. 
Sau đó viết hàm `translate` dùng chuỗi `match` để kiểm tra `curr` (VND/USD) và `stat` (OK/FAIL). Nếu lọt cờ mờ thì trả `Err("Unknown status")`.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| Báo lỗi `unknown field` khi lấy JSON | Front-end gửi dư rác | Thêm `#[serde(deny_unknown_fields)]` nếu bạn muốn làm gắt, hoặc bỏ qua là xong. |
| Code dơ vì dính Serde vào Domain | Áp Serde lên Domain Struct | Tách làm đôi, viết thêm DTO, và cắn răng viết hàm `From` để đổi qua đổi lại. Tốn thời gian lúc đầu nhưng an toàn mãi mãi. |
| Ép Float (Tiền) vào Int bị mất số lẻ | `f64` → `u32` | Đừng bao giờ dùng Float cho Tiền Tệ. Luôn yêu cầu Front-end gửi Cents/Đồng (Số nguyên). |

---

## Tóm tắt

Chapter này đã cấp cho bạn Tấm Khiên Chống Đạn vững chắc nhất trong Thiết Kế Hệ Thống:
- ✅ **`serde`**: Phép màu chuyển đổi Dữ liệu một cách dễ dàng.
- ✅ **Domain ≠ DTO**: Code cốt lõi là Vàng, DTO là hộp Carton. Bỏ Vàng vào Hộp để ship đi, đừng bắt Vàng tự biến thành hộp Carton. Dùng `From`/`TryFrom` để ship.
- ✅ **Anti-Corruption Layer (ACL)**: Hải quan ngăn rác thải từ hệ thống cũ chui vào bộ máy mới của bạn.
- ✅ **Validate at Boundaries**: Biên giới là nơi thích hợp nhất để soi lỗi, bên trong Vương quốc thì mọi thứ phải tinh sạch.

## Tiếp theo

Dữ liệu đã qua biên giới an toàn, đã được nhào nặn sạch sẽ — giờ ta cần **Cất Giữ (Lưu trữ)** nó. Giống như một cái Thư viện, bạn mang sách đến, thủ thư biết xếp ở đâu và tìm lại thế nào.

→ Chapter 26: **Persistence & Repository Pattern** — Nơi bạn sẽ học cách giấu Database đằng sau Interface, và biến việc Lưu/Đọc dữ liệu trở nên nhẹ nhàng, độc lập, và siêu dễ Test.
