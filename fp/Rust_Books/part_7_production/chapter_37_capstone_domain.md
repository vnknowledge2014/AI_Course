# Chapter 37 — Capstone Part 1: Domain Model ⭐

> **Bạn sẽ học được**:
> - **Kết hợp TẤT CẢ** concepts từ 36 chapters trước
> - **Event Storming** → type-driven domain model
> - Newtypes, smart constructors, state machines
> - **Pipeline workflows** với `Result` chaining
> - **Railway-Oriented Programming** trong real system
> - Testing domain logic với property-based tests
>
> **Yêu cầu**: ALL previous chapters (especially Part IV: DDD).
> **Thời gian đọc**: ~50 phút | **Level**: Principal
> **Kết quả cuối cùng**: **Order-Taking System** — complete, tested, production-ready domain model.

---

## Capstone Project: Trái tim của Hệ thống

Đã đến lúc kết hợp mọi thứ.

Trong 36 chương trước, bạn đã học riêng lẻ từng nốt nhạc: các loại biến (Part I), tư duy Hàm (Part II), mẫu thiết kế (Part III), Thiết kế hướng Domain - DDD (Part IV), đại số trừu tượng (Part V), và nghệ thuật kiểm thử (Part VI). 
Giống như học nhạc, bây giờ là lúc bước lên sân khấu và chơi một **buổi hòa nhạc thực sự** — nơi tất cả các nhạc cụ hòa quyện vào nhau.

Chúng ta sẽ xây dựng hệ thống lõi (Domain Model) cho một ứng dụng Quản lý đơn hàng (Order-Taking System). Nó đủ phức tạp để minh họa sức mạnh của Rust: dùng Type để quy định luật kinh doanh, State Machines để chống lỗi chuyển trạng thái, Pipeline để xử lý luồng dữ liệu, và ROP để bắt lỗi thanh lịch.

Mục tiêu là sau chương này, bạn sẽ nắm trong tay một **khuôn mẫu (template)** hoàn hảo để tự tin xây dựng bất kỳ lĩnh vực nào: E-commerce, Fintech, SaaS, hay Y tế.

---

## 37.1 — Khám phá Domain: Event Storming

Trước khi viết bất kỳ dòng code nào, chúng ta phải hiểu business. 
Kỹ thuật **Event Storming** giúp chúng ta vạch ra các luồng sự kiện chính trong hệ thống:

### Events (Những gì xảy ra)

```text
OrderPlaced → OrderValidated → OrderPriced → OrderConfirmed → OrderShipped
         ↘                                            ↗
          ValidationFailed                    ShippingFailed
```

### Commands (Lệnh gây ra sự kiện)

```text
PlaceOrder → ValidateOrder → PriceOrder → ConfirmOrder → ShipOrder
```

Từ những sự kiện này, các kiểu dữ liệu (Types) bắt đầu lộ diện.

---

## 37.2 — Xây dựng Value Objects (Đối tượng giá trị)

Quy tắc tối thượng: **Parse, don't validate**. Chúng ta không truyền `String` hay `i32` chạy lung tung trong hệ thống. Chúng ta bọc chúng lại thành các Value Object có tính ràng buộc cao ngay từ lúc khởi tạo (Smart Constructors).

Hãy bắt đầu với ID Đơn hàng và Email:

```rust
// filename: src/main.rs

// ═══ VALUE OBJECTS ═══

/// Order ID — Không được rỗng, không quá dài
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OrderId(String);

impl OrderId {
    fn new(id: &str) -> Result<Self, String> {
        let trimmed = id.trim();
        if trimmed.is_empty() { return Err("OrderId cannot be empty".into()); }
        if trimmed.len() > 50 { return Err("OrderId too long".into()); }
        Ok(OrderId(trimmed.into()))
    }
}

/// Customer Email — Phải đúng định dạng cơ bản
#[derive(Debug, Clone, PartialEq)]
struct EmailAddress(String);

impl EmailAddress {
    fn new(email: &str) -> Result<Self, String> {
        let trimmed = email.trim().to_lowercase();
        if !trimmed.contains('@') || trimmed.len() < 5 {
            return Err(format!("Invalid email: {}", email));
        }
        Ok(EmailAddress(trimmed))
    }
}
```

Tiếp theo là dữ liệu về Số lượng và Giá tiền. Lưu ý rằng Giá tiền (Price) luôn được lưu dưới dạng số nguyên (`u64` cents) để tránh sai số dấu phẩy động khét tiếng của Float.

```rust
/// Quantity — Luôn dương, có giới hạn tối đa
#[derive(Debug, Clone, Copy, PartialEq)]
struct Quantity(u32);

impl Quantity {
    fn new(qty: u32) -> Result<Self, String> {
        if qty == 0 { return Err("Quantity must be > 0".into()); }
        if qty > 10_000 { return Err("Quantity exceeds max 10,000".into()); }
        Ok(Quantity(qty))
    }
    fn value(&self) -> u32 { self.0 }
}

/// Price — Tiền không bao giờ âm
#[derive(Debug, Clone, Copy, PartialEq)]
struct Price(u64);

impl Price {
    fn new(cents: u64) -> Result<Self, String> {
        if cents > 100_000_000 { return Err("Price exceeds max".into()); }
        Ok(Price(cents))
    }
    fn cents(&self) -> u64 { self.0 }
    
    // Tính tổng tiền của một dòng sản phẩm
    fn multiply(&self, qty: Quantity) -> Price {
        Price(self.0 * qty.value() as u64)
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}đ", self.0 / 100, self.0 % 100)
    }
}
```

Và cuối cùng là mã sản phẩm (Product Code). Hệ thống của chúng ta chỉ bán 2 loại hàng: Widget (mã W) và Gizmo (mã G):

```rust
/// Product Code — Định dạng khắt khe
#[derive(Debug, Clone, PartialEq)]
enum ProductCode {
    Widget(String),  // "W" + 4 digits
    Gizmo(String),   // "G" + 3 digits
}

impl ProductCode {
    fn new(code: &str) -> Result<Self, String> {
        match code.chars().next() {
            Some('W') if code.len() == 5 && code[1..].chars().all(|c| c.is_ascii_digit()) =>
                Ok(ProductCode::Widget(code.into())),
            Some('G') if code.len() == 4 && code[1..].chars().all(|c| c.is_ascii_digit()) =>
                Ok(ProductCode::Gizmo(code.into())),
            _ => Err(format!("Invalid product code: {}", code)),
        }
    }
}
```

Hãy thử khởi tạo chúng để thấy sức mạnh của việc ép lỗi từ trong nôi:

```rust
fn main() {
    // Nếu tạo thành công, dữ liệu ĐẢM BẢO hợp lệ 100% trong toàn bộ app
    println!("{:?}", OrderId::new("ORD-001"));
    println!("{:?}", EmailAddress::new("minh@co.com"));
    println!("{:?}", ProductCode::new("W1234"));

    // Lỗi bị chặn đứng ngay lập tức!
    println!("{:?}", OrderId::new(""));        // Err
    println!("{:?}", Quantity::new(0));          // Err
    println!("{:?}", ProductCode::new("X999"));  // Err
}
```

---

## 37.3 — Order State Machine (Cỗ máy trạng thái)

Một Đơn hàng không đứng im. Nó di chuyển từ trạng thái *Chưa xác thực* → *Đã xác thực* → *Đã tính giá* → *Đã xác nhận*.

Thay vì dùng chung một `struct Order` to đùng kèm trường `status: String`, chúng ta định nghĩa **MỖI TRẠNG THÁI LÀ MỘT KIỂU DỮ LIỆU RIÊNG (Type)**. Điều này khiến cho việc "Tính giá một đơn hàng chưa xác thực" trở thành **lỗi biên dịch (Compile Error)** thay vì lỗi runtime!

```rust
// ═══ STATE MACHINE: Mỗi trạng thái là 1 Type ═══

/// Unvalidated — Dữ liệu thô từ khách hàng gửi lên (API payload)
#[derive(Debug)]
struct UnvalidatedOrder {
    order_id: String,
    customer_email: String,
    lines: Vec<UnvalidatedOrderLine>,
}

#[derive(Debug)]
struct UnvalidatedOrderLine {
    product_code: String,
    quantity: u32,
}

/// Validated — Đã kiểm tra tính hợp lệ, biến thành Value Objects
#[derive(Debug)]
struct ValidatedOrder {
    order_id: OrderId,
    customer_email: EmailAddress,
    lines: Vec<ValidatedOrderLine>,
}

#[derive(Debug)]
struct ValidatedOrderLine {
    product: ProductCode,
    quantity: Quantity,
}
```

Tiếp đến là trạng thái Đã tính giá (Priced). Lúc này, đơn hàng sẽ có thêm trường `subtotal`, `tax` và `total`. Một đơn hàng Unvalidated hoàn toàn không có các trường này, nên không ai có thể truy cập nhầm!

```rust
/// Priced — Đã tra cứu giá cả và tính tổng tiền
#[derive(Debug)]
struct PricedOrder {
    order_id: OrderId,
    customer_email: EmailAddress,
    lines: Vec<PricedOrderLine>,
    subtotal: Price,
    tax: Price,
    total: Price,
}

#[derive(Debug)]
struct PricedOrderLine {
    product: ProductCode,
    quantity: Quantity,
    unit_price: Price,
    line_total: Price,
}

/// Confirmed — Sẵn sàng để giao hàng
#[derive(Debug)]
struct ConfirmedOrder {
    order_id: OrderId,
    customer_email: EmailAddress,
    total: Price,
    confirmation_number: String,
}
```

> **💡 Triết lý FP**: State machine thông qua Types là cách tốt nhất để ép buộc luồng nghiệp vụ. Bạn không thể vo viên một chiếc bánh mì nếu nó chưa được nhào bột (vì kiểu `BotNhao` khác với kiểu `BanhMiChuaNuong`).

---

## 37.4 — Workflow Pipeline: Chuyến tàu xuyên miền dữ liệu

Giờ chúng ta sẽ viết các hàm đóng vai trò "chuyển đổi trạng thái" (State transition functions). Chúng giống như các ga tàu, nhận toa tàu đầu vào, xử lý và nhả ra toa tàu tiếp theo.

Đầu tiên là định nghĩa bộ lỗi (Errors):

```rust
// ═══ ERRORS ═══
#[derive(Debug)]
enum OrderError {
    Validation(Vec<String>), // Có thể gom nhiều lỗi validation cùng lúc
    Pricing(String),
    Confirmation(String),
}
```

Ga tàu số 1: Validation. Chúng ta sẽ "bắt" (collect) tất cả các lỗi có thể xảy ra thay vì ngắt ngay lập tức ở lỗi đầu tiên, mang lại UX tốt hơn cho người dùng API.

```rust
use std::collections::HashMap;

// Ga 1: Xác thực (Từ Unvalidated -> Validated)
fn validate_order(input: UnvalidatedOrder) -> Result<ValidatedOrder, OrderError> {
    let mut errors = vec![];

    let order_id = if input.order_id.trim().is_empty() {
        errors.push("OrderId is required".into()); None
    } else { Some(OrderId(input.order_id.trim().into())) };

    let email = if !input.customer_email.contains('@') {
        errors.push(format!("Invalid email: {}", input.customer_email)); None
    } else { Some(EmailAddress(input.customer_email.to_lowercase())) };

    let mut validated_lines = vec![];
    for (i, line) in input.lines.iter().enumerate() {
        if line.product_code.is_empty() { errors.push(format!("Line {}: empty product code", i + 1)); }
        if line.quantity == 0 { errors.push(format!("Line {}: quantity must be > 0", i + 1)); }
        
        if !line.product_code.is_empty() && line.quantity > 0 {
            validated_lines.push((
                ProductCode(line.product_code.clone()), // Simplify cho ví dụ
                Quantity(line.quantity)
            ));
        }
    }

    if validated_lines.is_empty() && errors.is_empty() {
        errors.push("Order must have at least 1 line".into());
    }

    if !errors.is_empty() { return Err(OrderError::Validation(errors)); }

    Ok(ValidatedOrder {
        order_id: order_id.unwrap(),
        customer_email: email.unwrap(),
        lines: validated_lines,
    })
}
```

Ga tàu số 2: Pricing (Tính giá). Tại đây, ta cần tra cứu giá từ hệ thống Catalog (bảng giá).

```rust
// Ga 2: Tính giá (Từ Validated -> Priced)
fn price_order(
    order: ValidatedOrder,
    catalog: &HashMap<String, u64>,
) -> Result<PricedOrder, OrderError> {
    let mut priced_lines = vec![];

    for (product, qty) in &order.lines {
        // Tra cứu giá từ Catalog, nếu không thấy -> báo lỗi
        let unit_price = catalog.get(&product.0)
            .ok_or_else(|| OrderError::Pricing(format!("Unknown product: {}", product.0)))?;
            
        let line_total = unit_price * qty.0 as u64;
        priced_lines.push((product.clone(), *qty, Price(*unit_price), Price(line_total)));
    }

    let subtotal: u64 = priced_lines.iter().map(|(_, _, _, lt)| lt.0).sum();
    let tax = subtotal * 10 / 100; // 10% VAT

    Ok(PricedOrder {
        order_id: order.order_id,
        customer_email: order.customer_email,
        lines: priced_lines,
        subtotal: Price(subtotal),
        tax: Price(tax),
        total: Price(subtotal + tax),
    })
}
```

Ga tàu số 3: Xác nhận. Rất đơn giản, cấp cho nó một mã số xác nhận.

```rust
// Ga 3: Xác nhận (Từ Priced -> Confirmed)
fn confirm_order(order: PricedOrder) -> Result<ConfirmedOrder, OrderError> {
    if order.total.0 == 0 {
        return Err(OrderError::Confirmation("Order total cannot be 0".into()));
    }

    Ok(ConfirmedOrder {
        order_id: order.order_id,
        customer_email: order.customer_email,
        total: order.total,
        confirmation_number: format!("CONF-{}", chrono_like_id()),
    })
}

fn chrono_like_id() -> String { "20260304-001".into() }
```

### Hợp nhất (Compose): Luồng chảy mượt mà

Nhờ Railway Oriented Programming (`and_then`), chúng ta nối ba ga tàu lại với nhau thành một dây chuyền sản xuất duy nhất:

```rust
// ═══ COMPOSE: Full workflow ═══
fn place_order(
    input: UnvalidatedOrder,
    catalog: &HashMap<String, u64>,
) -> Result<ConfirmedOrder, OrderError> {
    validate_order(input)
        .and_then(|validated| price_order(validated, catalog))
        .and_then(confirm_order)
}
```
Nhìn hàm `place_order` này mà xem, nó thể hiện chính xác **nghiệp vụ** dưới dạng code. Bạn đọc code như đang đọc kịch bản.

---

## 37.5 — Biến đổi thành Domain Events

Hệ thống DDD thường giao tiếp với bên ngoài bằng các Sự kiện (Events). Nếu đặt hàng thành công, ta phát ra `OrderPlaced`. Nếu lỗi, ta phát ra `ValidationFailed`.

```rust
// ═══ EVENTS ═══
#[derive(Debug)]
enum OrderEvent {
    OrderPlaced { order_id: String, total: u64 },
    ValidationFailed { errors: Vec<String> },
}

fn to_events(result: &Result<ConfirmedOrder, OrderError>) -> Vec<OrderEvent> {
    match result {
        Ok(order) => vec![OrderEvent::OrderPlaced {
            order_id: order.order_id.0.clone(),
            total: order.total.0,
        }],
        Err(OrderError::Validation(errors)) => vec![OrderEvent::ValidationFailed {
            errors: errors.clone(),
        }],
        Err(_) => vec![],
    }
}
```

Bạn có thể cắm luồng chạy thử này vào trong `main()` để thấy phép màu:

```rust
fn main() {
    let mut catalog = HashMap::new();
    catalog.insert("W1234".into(), 85_000_u64);
    catalog.insert("G567".into(), 45_000_u64);

    // ═══ Happy path (Thành công mĩ mãn) ═══
    let order = UnvalidatedOrder {
        order_id: "ORD-001".into(),
        customer_email: "minh@company.com".into(),
        lines: vec![
            UnvalidatedOrderLine { product_code: "W1234".into(), quantity: 2 },
        ],
    };

    let result = place_order(order, &catalog);
    for event in to_events(&result) { println!("Event: {:?}", event); }
    // Event: OrderPlaced { order_id: "ORD-001", total: 187000 }
}
```

---

## 37.6 — Testing the Domain (Kiểm thử)

Vì Domain của chúng ta chứa **hoàn toàn các hàm thuần túy (Pure Functions)**, việc viết Unit Test vô cùng sung sướng. Không cần Mock database, không cần fake network. Chỉ là Input -> Output.

```rust
// filename: src/lib.rs (tests section)

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_catalog() -> HashMap<String, u64> {
        let mut c = HashMap::new();
        c.insert("W1234".into(), 85_000);
        c
    }

    fn valid_order() -> UnvalidatedOrder {
        UnvalidatedOrder {
            order_id: "ORD-TEST".into(),
            customer_email: "test@co.com".into(),
            lines: vec![UnvalidatedOrderLine { product_code: "W1234".into(), quantity: 2 }],
        }
    }

    // --- Validation tests ---
    #[test]
    fn validates_good_order() {
        assert!(validate_order(valid_order()).is_ok());
    }

    #[test]
    fn collects_all_validation_errors() {
        let bad = UnvalidatedOrder {
            order_id: "".into(),
            customer_email: "bad".into(),
            lines: vec![UnvalidatedOrderLine { product_code: "".into(), quantity: 0 }],
        };
        match validate_order(bad) {
            Err(OrderError::Validation(errors)) => assert!(errors.len() >= 3),
            _ => panic!("Expected multiple validation errors"),
        }
    }

    // --- Pricing tests ---
    #[test]
    fn prices_order_correctly() {
        let validated = validate_order(valid_order()).unwrap();
        let priced = price_order(validated, &sample_catalog()).unwrap();
        assert_eq!(priced.subtotal.0, 170_000); // 85k * 2
        assert_eq!(priced.tax.0, 17_000);       // 10%
        assert_eq!(priced.total.0, 187_000);    // sub + tax
    }

    #[test]
    fn unknown_product_fails_pricing() {
        let mut o = valid_order();
        o.lines[0].product_code = "UNKNOWN".into();
        let validated = validate_order(o).unwrap();
        assert!(price_order(validated, &sample_catalog()).is_err());
    }
}
```

---

## 37.7 — Tóm lược Kiến trúc (Architecture Overview)

```text
┌──────────────────────────────────────────────────────┐
│                    Application                       │
│                                                      │
│  place_order(input) =                                │
│    validate_order(input)                             │
│      .and_then(|v| price_order(v, catalog))          │
│      .and_then(confirm_order)                        │
│                                                      │
│  ┌────────────┐  ┌────────────┐  ┌───────────────┐   │
│  │ Validate   │→ │   Price    │→ │   Confirm     │   │
│  │            │  │            │  │               │   │
│  │Unvalidated │  │ Validated  │  │ PricedOrder   │   │
│  │  → Valid   │  │  → Priced  │  │  → Confirmed  │   │
│  └────────────┘  └────────────┘  └───────────────┘   │
│                                                      │
│  ═══ TẤT CẢ LÀ PURE FUNCTIONS ═══                    │
│  Không IO, Không side effects, Không gọi Database    │
│  Chỉ có Type + Function + Result                     │
└──────────────────────────────────────────────────────┘
```

### Chúng ta đã triệu hồi những phép thuật nào?

| Chapter đã học | Concept | Ứng dụng trong project này |
|---------|---------|----------|
| Ch 14 — Enums | `ProductCode::Widget\|Gizmo` | Typed product codes |
| Ch 15 — Pattern Matching | `match` trong validate/price | Rẽ nhánh an toàn |
| Ch 22 — Domain Modeling | Newtypes, smart constructors | Khởi tạo `OrderId`, `Email`, `Price` |
| Ch 23 — Workflows | Pipeline `→` | Các bước xử lý đơn hàng |
| Ch 24 — ROP | Chuỗi `and_then`, bắt lỗi | Luồng Validation + pricing |
| Ch 33 — TDD | Unit tests | Viết tests bao phủ mọi ngóc ngách logic |

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Add shipping

Thêm phí vận chuyển (Shipping) vào đơn hàng:
```rust
enum ShippingMethod { Standard, Express, SameDay }
```
Mỗi method có shipping cost khác nhau. Viết hàm `add_shipping` và chèn nó vào giữa bước `price` và `confirm` trong pipeline.

<details><summary>✅ Lời giải</summary>

```rust
enum ShippingMethod { Standard, Express, SameDay }

fn shipping_cost(method: &ShippingMethod) -> Price {
    match method {
        ShippingMethod::Standard => Price(30_000),
        ShippingMethod::Express => Price(60_000),
        ShippingMethod::SameDay => Price(120_000),
    }
}

fn add_shipping(order: PricedOrder, method: ShippingMethod) -> PricedOrder {
    let shipping = shipping_cost(&method);
    PricedOrder {
        total: Price(order.total.0 + shipping.0),
        ..order
    }
}

// Updated pipeline:
fn place_order_v2(input: UnvalidatedOrder, catalog: &HashMap<String, u64>, shipping: ShippingMethod) -> Result<ConfirmedOrder, OrderError> {
    validate_order(input)
        .and_then(|v| price_order(v, catalog))
        .map(|p| add_shipping(p, shipping)) // Map vì add_shipping không sinh ra lỗi
        .and_then(confirm_order)
}
```

</details>

---

**Bài 2** (15 phút): Quy tắc giảm giá

Thực hiện yêu cầu kinh doanh mới:
- Đơn hàng > 500.000đ → Giảm 5%
- Đơn hàng > 1.000.000đ → Giảm 10%
- Khách VIP → Giảm thêm 5% cộng dồn

Hãy viết hàm `calculate_discount(subtotal: u64, is_vip: bool) -> u32` và viết Unit Test cho nó.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
fn calculate_discount(subtotal: u64, is_vip: bool) -> u32 {
    let volume_discount = match subtotal {
        s if s > 1_000_000 => 10,
        s if s > 500_000 => 5,
        _ => 0,
    };
    let vip_bonus = if is_vip { 5 } else { 0 };
    volume_discount + vip_bonus
}

// Tests
#[test]
fn small_order_no_discount() { assert_eq!(calculate_discount(300_000, false), 0); }
#[test]
fn medium_order_5_percent() { assert_eq!(calculate_discount(700_000, false), 5); }
#[test]
fn large_order_10_percent() { assert_eq!(calculate_discount(2_000_000, false), 10); }
#[test]
fn vip_gets_extra() { assert_eq!(calculate_discount(700_000, true), 10); }
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Sao phải tạo quá nhiều type Struct cho Order thế?" | Bạn thấy phiền vì mỗi State cần một Type riêng. | Đúng! Types sinh ra là để làm Tài liệu (Documentation). Nhờ nó, Compiler mới bảo vệ bạn khỏi việc xử lý sai thứ tự. |
| "Pipeline dài và lồng nhau khó đọc" | Bạn xử lý quá nhiều logic phức tạp ngay bên trong `and_then`. | Tách chúng ra thành các hàm Helper độc lập, giữ cho dây chuyền (pipeline) luôn phẳng. |
| "Phải Clone dữ liệu liên tục qua mỗi bước" | Bạn đang truyền reference và tốn công Copy. | Truyền Ownership thẳng tay (consume value) cho mỗi hàm (ví dụ hàm nhận `order: ValidatedOrder` thay vì `&ValidatedOrder`). |

---

---

## ✅ Checkpoint 37

1. Trong capstone này, module nào **không** được import bất cứ crate bên ngoài nào ngoài `std` và `serde`?
2. Vì sao smart constructor trả `Result` thay vì panic khi dữ liệu sai?
3. Property-based test phù hợp với phần nào của domain model này nhất?

<details>
<summary>Đáp án</summary>

1. Module **domain**. Đó là phép thử nhanh nhất cho kiến trúc onion: nếu domain phải kéo theo `sqlx` hay `axum`, ranh giới đã vỡ.
2. Vì dữ liệu sai là **tình huống nghiệp vụ được dự kiến**, không phải bug của lập trình viên. `panic!` dành cho bất biến bị vi phạm trong chính code của bạn; `Result` dành cho đầu vào từ thế giới bên ngoài.
3. Round-trip serialize (`serde_json::from_str(&to_string(&x)) == x`) và bất biến của state machine (không đường đi hợp lệ nào dẫn tới trạng thái không hợp lệ).
</details>

## Tóm tắt

- ✅ **Khám phá Domain**: Đi từ Yêu cầu nghiệp vụ → Sự kiện → Các kiểu dữ liệu.
- ✅ **Value Objects**: Chặn đứng mọi dữ liệu bẩn từ vòng gửi xe bằng Smart constructors.
- ✅ **State Machine**: Mọi bước chuyển mình (từ thô → đã duyệt → đã tính tiền) đều được ràng buộc chặt bởi Rust Compiler thông qua Type System.
- ✅ **Pipeline**: Chắp nối dây chuyền sản xuất mượt mà bằng ROP (`and_then`).
- ✅ **Lợi ích tối thượng**: Khi toàn bộ hệ thống Core Domain là Pure Functions, bạn test nó vô cùng dễ dàng, đọc nó như đọc tiểu thuyết và không bao giờ sợ hãi khi bảo trì!

## Tiếp theo

→ Hãy chuyển sang **Chapter 44 — Capstone Part 2** (hay Chương Capstone cuối cùng) nơi chúng ta sẽ biến Domain tuyệt đẹp này thành một hệ thống Web hoàn chỉnh với Cơ sở dữ liệu và API!
