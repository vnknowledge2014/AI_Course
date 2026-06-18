# Chapter 14 — Structs, Enums & Algebraic Types

> **Bạn sẽ học được**:
> - **Product types** = `struct` (AND) — kết hợp nhiều fields
> - **Sum types** = `enum` (OR) — một trong nhiều variants
> - Newtype pattern — wrap types để thêm ý nghĩa và validation
> - **"Types as design tools"** — dùng type system để ngăn bugs lúc compile
> - Generic enums: `Option<T>`, `Result<T, E>` dưới góc nhìn algebraic
>
> **Yêu cầu trước**: Chapter 1 (Algebraic Type Sizes), Chapter 6 (Match), Chapter 12 (Immutability).
> **Thời gian đọc**: ~40 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Bạn dùng types như **công cụ thiết kế** — model domain chính xác, compiler bắt bugs thay bạn.

---

## Tại sao Types là công cụ thiết kế quan trọng nhất?

Hầu hết lập trình viên nghĩ types chỉ là cách gán nhãn cho data: "đây là string, đây là number, đây là struct có 3 fields". Cách nhìn đó bỏ lỡ 90% sức mạnh của type system.

Trong functional programming — đặc biệt trong trường phái Domain-Driven Design — types là **công cụ thiết kế** quan trọng hơn cả functions. Tại sao? Vì types quyết định **những gì CÓ THỂ xảy ra** trong chương trình. Nếu bạn thiết kế types đúng, nhiều loại bugs **không thể tồn tại** — chúng bị loại bỏ tại compile time, trước khi code chạy.

Scott Wlaschin gọi nguyên tắc này là *"Make Illegal States Unrepresentable"*. Ý tưởng: thay vì viết code kiểm tra "trạng thái này có hợp lệ không?" tại runtime, bạn thiết kế types sao cho trạng thái sai **không thể biểu diễn được**. Compiler trở thành người gác cổng — nếu code compile, bạn biết chắc data luôn ở trạng thái hợp lệ.

Nhưng để làm được điều đó, bạn cần hiểu hai khái niệm: **Product types** (struct — AND) và **Sum types** (enum — OR). Chúng là hai viên gạch nền tảng mà mọi domain model phức tạp đều được xây từ đó.

Chapter này sẽ đưa bạn từ "types = nhãn cho data" sang "types = công cụ thiết kế". Đến cuối, bạn sẽ thấy enum + struct không chỉ lưu data — chúng **mô tả luật chơi** của hệ thống, và compiler **thực thi** luật đó cho bạn.

---

## 14.1 — Product Types: Struct = AND

### Bản thiết kế căn nhà

Hãy nghĩ về bản thiết kế căn nhà. Một căn nhà **phải có** phòng khách VÀ bếp VÀ phòng ngủ — thiếu bất kỳ phòng nào thì không phải căn nhà hoàn chỉnh. Đó là **Product type** (struct) — tất cả fields **đều cần có mặt** cùng lúc.

Ngược lại, khi chọn loại nhà: biệt thự HOẶC chung cư HOẶC nhà phố — bạn chỉ chọn **một** trong các loại. Đó là **Sum type** (enum) — một trong nhiều variants.

Chapter này dạy bạn dùng struct và enum như **công cụ thiết kế** — không chỉ lưu data, mà **mô tả chính xác** các trạng thái hợp lệ của hệ thống. Ở Chapter 1, bạn đã biết: **Product type = nhân số lượng states**. Struct `{ a: bool, b: bool }` có 2 × 2 = 4 states. Giờ hãy áp dụng vào thiết kế thực tế:

### Named struct — Phổ biến nhất

```rust
// filename: src/main.rs

// Product type: mọi field CẦN CÓ MẶT cùng lúc (AND)
#[derive(Debug, Clone)]
struct Customer {
    id: u64,
    name: String,
    email: String,
    is_vip: bool,
}

impl Customer {
    fn new(id: u64, name: &str, email: &str) -> Self {
        Customer {
            id,
            name: name.to_string(),
            email: email.to_string(),
            is_vip: false,
        }
    }

    // Functional update — trả struct mới
    fn promote_to_vip(&self) -> Self {
        Customer { is_vip: true, ..self.clone() }
    }

    fn display(&self) -> String {
        let vip_badge = if self.is_vip { " ⭐" } else { "" };
        format!("[{}] {}{} <{}>", self.id, self.name, vip_badge, self.email)
    }
}

fn main() {
    let customer = Customer::new(1, "Minh", "minh@email.com");
    let vip = customer.promote_to_vip();

    println!("Regular: {}", customer.display());
    println!("VIP:     {}", vip.display());
    // Regular: [1] Minh <minh@email.com>
    // VIP:     [1] Minh ⭐ <minh@email.com>
}
```

Nhìn lại method `promote_to_vip()`: nó không sửa `self`, mà tạo **bản mới** với `is_vip: true`. Đây là "functional update" — pattern cốt lõi của immutable design (Ch12). Cú pháp `..self.clone()` nói "giữ nguyên tất cả fields khác, chỉ đổi `is_vip`". Gọn, rõ ràng, và an toàn.

Câu hỏi tự nhiên: tại sao không dùng `&mut self` và sửa trực tiếp? Vì khi bạn giữ bản gốc không đổi, bạn có thể so sánh trước/sau, undo, hoặc chạy song song mà không sợ data race. Đây là trade-off cốt lõi: tiêu tốn thêm memory (clone) để đổi lấy safety và simplicity.

### Tuple struct — Khi tên fields không cần thiết

```rust
// filename: src/main.rs

// Tuple struct: fields truy cập bằng .0, .1, ...
#[derive(Debug, Clone, Copy)]
struct Point(f64, f64);

#[derive(Debug, Clone, Copy)]
struct Color(u8, u8, u8);

impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

impl Color {
    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

fn main() {
    let a = Point(0.0, 0.0);
    let b = Point(3.0, 4.0);
    println!("Distance: {:.1}", a.distance_to(&b));  // 5.0

    let red = Color(255, 0, 0);
    println!("Red: {}", red.to_hex());  // #FF0000
}
```

### Unit struct — Không có data, chỉ có identity

```rust
// filename: src/main.rs

// Unit struct: zero-size, chỉ đánh dấu type
struct Authenticated;
struct Guest;

// Dùng làm "phantom type" — chỉ tồn tại lúc compile
fn admin_panel(_token: &Authenticated) -> &str {
    "Welcome to admin panel"
}

fn main() {
    let token = Authenticated;
    println!("{}", admin_panel(&token));

    // Guest không thể gọi admin_panel!
    // admin_panel(&Guest);  // ❌ mismatched types
}
```

---

## 14.2 — Sum Types: Enum = OR

Nếu struct nói "A **VÀ** B **VÀ** C đều phải có", thì enum nói "A **HOẶC** B **HOẶC** C — chỉ một trong ba". Sự khác biệt này nghe đơn giản nhưng hệ quả rất lớn.

Trong Java/C#, bạn mô hình "một trong nhiều" bằng inheritance: base class `Payment`, subclasses `CashPayment`, `CardPayment`, `TransferPayment`. Vấn đề: bạn có thể quên subclass nào khi xử lý, và compiler không nhắc. Trong Rust, `enum` + `match` bắt buộc bạn xử lý **tất cả** variants — quên một cái = code không compile.

Đây là lý do functional programmers yêu thích sum types: chúng biến "quên xử lý edge case" từ runtime bug thành compile error.

### Enum = một trong nhiều trường hợp

```rust
// filename: src/main.rs

// Sum type: MỘT trong các variants (OR)
// States = variant1 + variant2 + variant3
#[derive(Debug)]
enum PaymentMethod {
    Cash,                           // 1 state
    Card { number: String, cvv: String }, // nhiều states
    Transfer { bank: String, account: String },
    Wallet(String),                 // 1 field
}

fn process_payment(method: &PaymentMethod, amount: u32) -> String {
    match method {
        PaymentMethod::Cash =>
            format!("💵 Cash: {}đ", amount),
        PaymentMethod::Card { number, .. } => {
            let last_four = &number[number.len()-4..];
            format!("💳 Card ****{}: {}đ", last_four, amount)
        }
        PaymentMethod::Transfer { bank, account } =>
            format!("🏦 {} → {}: {}đ", bank, account, amount),
        PaymentMethod::Wallet(provider) =>
            format!("📱 {}: {}đ", provider, amount),
    }
}

fn main() {
    let methods = vec![
        PaymentMethod::Cash,
        PaymentMethod::Card {
            number: "4111222233334444".into(),
            cvv: "123".into(),
        },
        PaymentMethod::Transfer {
            bank: "VCB".into(),
            account: "001234567".into(),
        },
        PaymentMethod::Wallet("MoMo".into()),
    ];

    for method in &methods {
        println!("{}", process_payment(method, 100_000));
    }
    // 💵 Cash: 100000đ
    // 💳 Card ****4444: 100000đ
    // 🏦 VCB → 001234567: 100000đ
    // 📱 MoMo: 100000đ
}
```

Nhìn function `process_payment`: mỗi arm trong `match` xử lý một variant, và compiler đảm bảo bạn không quên variant nào. Nếu sau này bạn thêm `PaymentMethod::QRCode { ... }`, compiler sẽ báo lỗi ở **mọi** match block chưa xử lý `QRCode`. Trong Java, thêm subclass mới có thể "âm thầm" rơi vào default case mà không ai biết.

`..` trong `Card { number, .. }` nghĩa là "tôi chỉ cần `number`, bỏ qua `cvv` và `currency`". Đây là destructuring chọn lọc — lấy đúng data cần thiết, không thừa không thiếu.

### Đếm states: Algebraic perspective

```rust
// filename: src/main.rs
fn main() {
    // Product: struct { bool, bool } = 2 × 2 = 4 states
    // (true,true), (true,false), (false,true), (false,false)

    // Sum: enum { A(bool), B(bool) } = 2 + 2 = 4 states
    // A(true), A(false), B(true), B(false)

    // Option<bool> = None + Some(bool) = 1 + 2 = 3 states
    // None, Some(true), Some(false)

    // Result<bool, u8> = Ok(bool) + Err(u8) = 2 + 256 = 258 states

    println!("Product (bool, bool): {} states", 2 * 2);
    println!("Sum A(bool)|B(bool): {} states", 2 + 2);
    println!("Option<bool>: {} states", 1 + 2);
    println!("Result<bool, u8>: {} states", 2 + 256);
}
```

---

## ✅ Checkpoint 14.2

> Ghi nhớ:
> 1. **Struct** = Product = AND → nhân states. **Enum** = Sum = OR → cộng states.
> 2. Enum variants có thể chứa data: unit, tuple, named fields
> 3. `match` trên enum = compiler buộc xử lý MỌI variant
>
> **Test nhanh**: `enum Light { Red, Yellow, Green }` có bao nhiêu states?
> <details><summary>Đáp án</summary>3 states: Red + Yellow + Green = 1 + 1 + 1.</details>

---

## 14.3 — Newtype Pattern: Types có ý nghĩa

Bạn đã biết struct (AND) và enum (OR). Giờ hãy xem pattern quan trọng thứ ba: **Newtype** — bọc một type đơn giản (String, u64, f64) trong struct mới để thêm **ý nghĩa domain** và **validation**.

Tại sao cần? Vì primitive obsession — dùng `String` cho mọi thứ (email, tên, địa chỉ, mã sản phẩm) — là nguồn gốc của vô số bugs. Khi tất cả đều là `String`, compiler không phân biệt được email với tên, bạn có thể truyền nhầm mà không ai biết.

### Vấn đề: "Stringly typed"

```rust
// ❌ Mọi thứ đều String — dễ nhầm!
fn send_email(to: &str, from: &str, subject: &str, body: &str) {
    // to và from cùng kiểu → dễ đảo ngược!
}

// Gọi nhầm: đảo to/from
// send_email("admin@co.com", "user@co.com", "Hi", "...");
// → Gửi mail cho admin thay vì user? Compiler không bắt!
```

### Giải pháp: Newtype

```rust
// filename: src/main.rs
use std::fmt;

// Newtype: mỗi "ý nghĩa" là một type riêng
#[derive(Debug, Clone, PartialEq)]
struct EmailAddress(String);

#[derive(Debug, Clone, PartialEq)]
struct Subject(String);

#[derive(Debug, Clone)]
struct Body(String);

impl EmailAddress {
    fn new(value: &str) -> Result<Self, String> {
        if value.contains('@') && value.len() >= 5 {
            Ok(EmailAddress(value.to_lowercase()))
        } else {
            Err(format!("Invalid email: {}", value))
        }
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Bây giờ KHÔNG THỂ nhầm to/from!
fn send_email(to: &EmailAddress, from: &EmailAddress, subject: &Subject, body: &Body) {
    println!("📧 From: {} → To: {}", from, to);
    println!("   Subject: {:?}", subject);
}

fn main() {
    let admin = EmailAddress::new("admin@company.com").unwrap();
    let user = EmailAddress::new("user@company.com").unwrap();
    let subject = Subject("Welcome".to_string());
    let body = Body("Hello!".to_string());

    send_email(&user, &admin, &subject, &body);  // ✅ rõ ràng
    // send_email(&subject, &admin, ...);  // ❌ compile error — Subject ≠ EmailAddress

    // Output: 📧 From: admin@company.com → To: user@company.com
}
```

### Newtype cho domain values

```rust
// filename: src/main.rs

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Percentage(f64);

impl Percentage {
    fn new(value: f64) -> Result<Self, String> {
        if (0.0..=100.0).contains(&value) {
            Ok(Percentage(value))
        } else {
            Err(format!("{}% out of range [0, 100]", value))
        }
    }

    fn value(&self) -> f64 { self.0 }
    fn as_multiplier(&self) -> f64 { self.0 / 100.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Money(u64);  // VNĐ, đơn vị đồng

impl Money {
    fn new(amount: u64) -> Self { Money(amount) }
    fn value(&self) -> u64 { self.0 }

    fn apply_discount(&self, discount: &Percentage) -> Self {
        let reduction = (self.0 as f64 * discount.as_multiplier()) as u64;
        Money(self.0 - reduction)
    }

    fn add_tax(&self, rate: &Percentage) -> Self {
        let tax = (self.0 as f64 * rate.as_multiplier()) as u64;
        Money(self.0 + tax)
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}đ", self.0)
    }
}

fn main() {
    let price = Money::new(500_000);
    let discount = Percentage::new(20.0).unwrap();
    let tax = Percentage::new(8.0).unwrap();

    let after_discount = price.apply_discount(&discount);
    let final_price = after_discount.add_tax(&tax);

    println!("Original:  {}", price);
    println!("Discount:  {} ({:.0}% off)", after_discount, discount.value());
    println!("Final:     {} (+{:.0}% tax)", final_price, tax.value());
    // Original:  500000đ
    // Discount:  400000đ (20% off)
    // Final:     432000đ (+8% tax)

    // ❌ Invalid percentage
    println!("{:?}", Percentage::new(150.0));  // Err("150% out of range")
}
```

---

## 14.4 — "Make Illegal States Unrepresentable"

### Ý tưởng cốt lõi

Đây là ý tưởng quan trọng nhất trong toàn bộ chapter — và có lẽ trong toàn bộ cuốn sách.

Hãy tưởng tượng bạn thiết kế hệ thống đèn giao thông. Phiên bản naive dùng 3 boolean flags: `is_red`, `is_yellow`, `is_green`. Có bao nhiêu tổ hợp? 2³ = 8. Nhưng chỉ 3 là hợp lệ (mỗi lần chỉ 1 đèn sáng). 5 tổ hợp còn lại — như `is_red=true, is_green=true` — là trạng thái vô nghĩa có thể gây tai nạn.

Phiên bản đúng? `enum Light { Red, Yellow, Green }` — 3 states, tất cả hợp lệ, không thể tạo trạng thái vô nghĩa. Compiler đảm bảo.

Nguyên tắc tương tự áp dụng cho mọi domain: order lifecycle, user authentication, payment processing. Thay vì validate data **runtime** rồi hy vọng không có bug, hãy thiết kế types sao cho **states sai không thể tồn tại** — compiler bắt lỗi lúc compile.

### Ví dụ: Order lifecycle

```rust
// filename: src/main.rs

// ❌ BAD: Dùng booleans — nhiều trạng thái vô nghĩa
// struct Order {
//     is_paid: bool,
//     is_shipped: bool,
//     is_cancelled: bool,
//     tracking: Option<String>,
// }
// → is_paid=false, is_shipped=true? Giao hàng chưa trả tiền??
// → is_cancelled=true, is_shipped=true? Hủy rồi mà vẫn giao??
// → 2 × 2 × 2 × 2 = 16 states, nhưng chỉ ~5 states hợp lệ!

// ✅ GOOD: Enum — chỉ states hợp lệ mới tồn tại
#[derive(Debug)]
enum Order {
    Draft { items: Vec<String> },
    Confirmed { items: Vec<String>, total: u32 },
    Paid { items: Vec<String>, total: u32, payment_id: String },
    Shipped { tracking: String, payment_id: String },
    Delivered { tracking: String },
    Cancelled { reason: String },
}
// → 6 states — TẤT CẢ hợp lệ! Không thể tạo state vô nghĩa.

// State transitions = pure functions
fn confirm(order: Order) -> Result<Order, String> {
    match order {
        Order::Draft { items } => {
            if items.is_empty() {
                Err("Cannot confirm empty order".to_string())
            } else {
                let total = items.len() as u32 * 50_000; // simplified pricing
                Ok(Order::Confirmed { items, total })
            }
        }
        _ => Err(format!("Cannot confirm order in {:?} state", order)),
    }
}

fn pay(order: Order, payment_id: &str) -> Result<Order, String> {
    match order {
        Order::Confirmed { items, total, .. } => {
            Ok(Order::Paid {
                items, total,
                payment_id: payment_id.to_string(),
            })
        }
        _ => Err(format!("Cannot pay order in {:?} state", order)),
    }
}

fn ship(order: Order, tracking: &str) -> Result<Order, String> {
    match order {
        Order::Paid { payment_id, .. } => {
            Ok(Order::Shipped {
                tracking: tracking.to_string(),
                payment_id,
            })
        }
        _ => Err(format!("Cannot ship order in {:?} state", order)),
    }
}

fn main() {
    let order = Order::Draft {
        items: vec!["Coffee".into(), "Cake".into()],
    };
    println!("1. {:?}", order);

    let order = confirm(order).unwrap();
    println!("2. {:?}", order);

    let order = pay(order, "PAY-001").unwrap();
    println!("3. {:?}", order);

    let order = ship(order, "VN123456").unwrap();
    println!("4. {:?}", order);

    // ❌ Không thể ship Draft — compiler + runtime bảo vệ!
    let draft = Order::Draft { items: vec!["Tea".into()] };
    println!("Ship draft: {:?}", ship(draft, "XX"));
    // Err("Cannot ship order in Draft { items: [\"Tea\"] } state")
}
```

Hãy đếm: phiên bản boolean có `2 × 2 × 2 × 2 = 16` tổ hợp, nhưng chỉ ~5 hợp lệ. 11 tổ hợp còn lại là "trạng thái ma" — code phải check runtime để ngăn chặn. Phiên bản enum có đúng 6 states, tất cả hợp lệ. Không có trạng thái ma, không cần runtime validation.

Lợi ích tiếp: mỗi state transition là một pure function (`confirm`, `pay`, `ship`) trả về state mới. Bạn có thể test từng transition riêng biệt, dễ dàng và deterministic. Thêm state mới? Thêm variant vào enum → compiler chỉ mọi chỗ cần xử lý. Không có cách nào "quên".

### So sánh boolean flags vs enum states

| | Boolean flags | Enum states |
|---|---|---|
| Số states | 2ⁿ (exponential) | n (linear) |
| States vô nghĩa? | ✅ Nhiều | ❌ Không có |
| Compiler bảo vệ? | ❌ Runtime check | ✅ Compile time |
| Thêm state mới? | Thêm bool → 2× states | Thêm variant → +1 state |
| Trường hợp | `is_paid && is_shipped && !is_cancelled` | `Order::Shipped { .. }` |

---

## 14.5 — Generic Enums: Option & Result revisited

Bạn đã dùng `Option<T>` và `Result<T, E>` từ chapters trước — nhưng giờ hãy nhìn chúng dưới góc nhìn algebraic. Khi bạn hiểu chúng là **sum types**, bạn sẽ hiểu tại sao chúng mạnh đến vậy — và cách tạo custom generic enums cho domain riêng.

### `Option<T>` — Dưới góc nhìn algebraic

```rust
// Option<T> thực chất là:
enum Option<T> {
    None,    // 1 state
    Some(T), // |T| states
}
// Tổng states = 1 + |T|

// Option<bool> = 1 + 2 = 3 states: None, Some(true), Some(false)
// Option<u8>   = 1 + 256 = 257 states
```

### Tự tạo Generic Enum

```rust
// filename: src/main.rs

// Generic enum: "Validation result" — thành công HOẶC danh sách lỗi
#[derive(Debug)]
enum Validated<T> {
    Valid(T),
    Invalid(Vec<String>),
}

impl<T> Validated<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U> {
        match self {
            Validated::Valid(val) => Validated::Valid(f(val)),
            Validated::Invalid(errors) => Validated::Invalid(errors),
        }
    }

    fn and_then<U, F: FnOnce(T) -> Validated<U>>(self, f: F) -> Validated<U> {
        match self {
            Validated::Valid(val) => f(val),
            Validated::Invalid(errors) => Validated::Invalid(errors),
        }
    }
}

// Validation functions
fn validate_name(name: &str) -> Validated<String> {
    if name.trim().len() >= 2 {
        Validated::Valid(name.trim().to_string())
    } else {
        Validated::Invalid(vec!["Name must be at least 2 characters".into()])
    }
}

fn validate_age(age: i32) -> Validated<u32> {
    if (0..=150).contains(&age) {
        Validated::Valid(age as u32)
    } else {
        Validated::Invalid(vec![format!("Age {} is out of range [0, 150]", age)])
    }
}

fn main() {
    // Valid path
    let name = validate_name("Minh");
    println!("Name: {:?}", name);  // Valid("Minh")

    let mapped = validate_name("Minh").map(|n| n.to_uppercase());
    println!("Upper: {:?}", mapped);  // Valid("MINH")

    // Invalid path
    let bad = validate_name("M");
    println!("Bad: {:?}", bad);  // Invalid(["Name must be at least 2 characters"])

    let bad_age = validate_age(200);
    println!("Bad age: {:?}", bad_age);  // Invalid(["Age 200 is out of range"])
}
```

---

## 14.6 — Enum Methods: Behavior gắn với state

Trong OOP, behavior gắn với class qua methods. Trong Rust, behavior gắn với enum qua `impl` block — và mỗi method dùng `match` để xử lý khác nhau tùy variant. Đây là cách Rust thay thế polymorphism của OOP: thay vì override method ở subclass, bạn match trên variant trong cùng method.

Pattern này cực kỳ phổ biến. Mọi enum quan trọng trong domain đều nên có `impl` block với methods cho business logic — giữ logic gần data, dễ tìm, dễ maintain.

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    // Methods cho tất cả variants
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 2.0 * std::f64::consts::PI * radius,
            Shape::Rectangle { width, height } => 2.0 * (width + height),
            Shape::Triangle { base, height } => {
                let hyp = (base * base + height * height).sqrt();
                base + height + hyp
            }
        }
    }

    // Functional transform: scale trả shape MỚI
    fn scale(&self, factor: f64) -> Self {
        match self {
            Shape::Circle { radius } => Shape::Circle { radius: radius * factor },
            Shape::Rectangle { width, height } =>
                Shape::Rectangle { width: width * factor, height: height * factor },
            Shape::Triangle { base, height } =>
                Shape::Triangle { base: base * factor, height: height * factor },
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("⭕ Circle(r={:.1})", radius),
            Shape::Rectangle { width, height } => format!("🟦 Rect({:.1}×{:.1})", width, height),
            Shape::Triangle { base, height } => format!("🔺 Tri({:.1}×{:.1})", base, height),
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { base: 3.0, height: 4.0 },
    ];

    println!("{:<25} {:>10} {:>10}", "Shape", "Area", "Perimeter");
    println!("{}", "-".repeat(47));
    for shape in &shapes {
        println!("{:<25} {:>10.2} {:>10.2}", shape.describe(), shape.area(), shape.perimeter());
    }

    // Scale tất cả ×2
    println!("\n📐 After scaling ×2:");
    for shape in &shapes {
        let scaled = shape.scale(2.0);
        println!("  {} → area {:.2}", scaled.describe(), scaled.area());
    }
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Đếm states

```rust
enum A { X, Y, Z }              // ? states
struct B { flag: bool, choice: A } // ? states
enum C { P(bool), Q(A) }           // ? states
```

<details><summary>✅ Lời giải Bài 1</summary>

```
A: 3 states (X + Y + Z)
B: 2 × 3 = 6 states (struct = product = multiply)
C: 2 + 3 = 5 states (enum = sum = add: P has 2 bool states, Q has 3 A states)
```

</details>

---

**Bài 2** (10 phút): Newtype — Temperature

Tạo `Celsius(f64)` và `Fahrenheit(f64)` newtypes. Implement `to_fahrenheit()` trên Celsius, `to_celsius()` trên Fahrenheit. Compiler phải **ngăn** bạn cộng `Celsius + Fahrenheit`.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
#[derive(Debug, Clone, Copy)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

impl Celsius {
    fn new(value: f64) -> Self { Celsius(value) }
    fn value(&self) -> f64 { self.0 }
    fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

impl Fahrenheit {
    fn new(value: f64) -> Self { Fahrenheit(value) }
    fn value(&self) -> f64 { self.0 }
    fn to_celsius(&self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }
}

fn main() {
    let boiling = Celsius::new(100.0);
    let boiling_f = boiling.to_fahrenheit();
    println!("{:.1}°C = {:.1}°F", boiling.value(), boiling_f.value());

    // let wrong = boiling + boiling_f;  // ❌ Cannot add Celsius + Fahrenheit!
}
```

</details>

---

**Bài 3** (15 phút): E-commerce type design

Thiết kế enum `CartItem` và struct `ShoppingCart` cho e-commerce:
- `CartItem`: `Physical { name, price, weight_kg }`, `Digital { name, price, download_url }`, `Subscription { name, monthly_price, months }`
- `ShoppingCart`: calculate `total()`, `shipping_cost()` (chỉ Physical items), `item_count()`

<details><summary>✅ Lời giải Bài 3</summary>

```rust
#[derive(Debug, Clone)]
enum CartItem {
    Physical { name: String, price: u32, weight_kg: f64 },
    Digital { name: String, price: u32, download_url: String },
    Subscription { name: String, monthly_price: u32, months: u32 },
}

impl CartItem {
    fn total_price(&self) -> u32 {
        match self {
            CartItem::Physical { price, .. } => *price,
            CartItem::Digital { price, .. } => *price,
            CartItem::Subscription { monthly_price, months, .. } => monthly_price * months,
        }
    }

    fn name(&self) -> &str {
        match self {
            CartItem::Physical { name, .. } |
            CartItem::Digital { name, .. } |
            CartItem::Subscription { name, .. } => name,
        }
    }
}

struct ShoppingCart {
    items: Vec<CartItem>,
}

impl ShoppingCart {
    fn new() -> Self { ShoppingCart { items: vec![] } }

    fn add(mut self, item: CartItem) -> Self {
        self.items.push(item);
        self
    }

    fn total(&self) -> u32 {
        self.items.iter().map(|i| i.total_price()).sum()
    }

    fn shipping_cost(&self) -> u32 {
        let total_weight: f64 = self.items.iter()
            .filter_map(|i| match i {
                CartItem::Physical { weight_kg, .. } => Some(weight_kg),
                _ => None,
            })
            .sum();
        (total_weight * 30_000.0) as u32  // 30k per kg
    }

    fn item_count(&self) -> usize { self.items.len() }
}

fn main() {
    let cart = ShoppingCart::new()
        .add(CartItem::Physical { name: "Book".into(), price: 150_000, weight_kg: 0.5 })
        .add(CartItem::Digital { name: "Course".into(), price: 500_000, download_url: "https://...".into() })
        .add(CartItem::Subscription { name: "Cloud".into(), monthly_price: 200_000, months: 12 });

    println!("Items: {}", cart.item_count());
    println!("Subtotal: {}đ", cart.total());
    println!("Shipping: {}đ", cart.shipping_cost());
    println!("Grand total: {}đ", cart.total() + cart.shipping_cost());
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `field is private` | Struct fields private mặc định | Thêm `pub` hoặc tạo constructor |
| `non-exhaustive match` | Thiếu variant trong match | Thêm variant hoặc `_ =>` |
| `struct update syntax cannot be used on different types` | `..old` phải cùng struct type | Đảm bảo cùng type, clone nếu cần |
| "Quá nhiều newtypes" | Over-engineering | Chỉ newtype khi: (1) cần validation, (2) tránh nhầm params, (3) domain concept rõ |

---

## Tóm tắt

- ✅ **Struct = Product = AND** → nhân states. **Enum = Sum = OR** → cộng states.
- ✅ **Newtype** = wrap primitive với ý nghĩa + validation. `EmailAddress(String)`, `Money(u64)`, `Percentage(f64)`.
- ✅ **"Make Illegal States Unrepresentable"** = dùng enum thay boolean flags. 6 states rõ ràng thay 16 states phần lớn vô nghĩa.
- ✅ **Generic enums** = Option, Result, custom like `Validated<T>`.
- ✅ **Types as design tools** — compiler bắt bugs thay bạn. "If it compiles, it probably works."

## Tiếp theo

→ Chapter 15: **Pattern Matching Mastery** — bạn sẽ học patterns nâng cao: nested destructuring, guards complex, `@` bindings, and `if let` chains. Và quan trọng nhất: cách dùng exhaustive matching để **compiler** verify logic cho bạn.
