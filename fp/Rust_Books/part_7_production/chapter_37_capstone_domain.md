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

## Capstone Project: Full Domain Model

Đây là chapter tổng hợp — mọi concept từ Part I đến Part VI được áp dụng vào một dự án hoàn chỉnh. Thay vì examples nhỏ, bạn xây dựng **domain model thực tế** từ đầu đến cuối: types, workflows, error handling, testing, persistence.

Mục tiêu: khi hoàn thành chapter này, bạn có thể áp dụng quy trình tương tự cho bất kỳ domain nào — e-commerce, fintech, SaaS, healthcare.

---

Đã đến lúc kết hợp mọi thứ.

Trong 36 chapters trước, bạn đã học riêng lẻ: types (Part I), FP thinking (Part II), design patterns (Part III), DDD (Part IV), abstractions (Part V), testing (Part VI). Giống như học nhạc — bạn biết từng nốt, từng hợp âm, từng kỹ thuật. Chapter này là buổi **trình diễn đầu tiên** — chơi một bài hoàn chỉnh.

Chúng ta sẽ xây dựng domain model cho hệ thống quản lý đơn hàng — đủ phức tạp để minh họa mọi concept, nhưng đủ nhỏ để hoàn thành trong 1 chapter. Bạn sẽ thấy types, workflows, error handling, testing, và persistence hoạt động cùng nhau.

Mục tiêu: sau chapter này, bạn có **template** để áp dụng cho bất kỳ domain nào.

36 chapters. Types, FP thinking, design patterns, DDD, abstractions, testing. Từng concept riêng lẻ — giống học từng nhạc cụ. Chapter này là **buổi hòa nhạc đầu tiên** — tất cả instruments chơi cùng nhau.

Bạn xây domain model cho hệ thống quản lý đơn hàng: types encode business rules (Ch14), state machines ngăn invalid transitions (Ch15), workflows pipeline data (Ch23), errors typed at each step (Ch24), tests verify behavior (Ch33). Đủ phức tạp để minh họa, đủ nhỏ để hoàn thành trong 1 chapter.

Khi hoàn thành: bạn có **template** — copy approach này cho bất kỳ domain: e-commerce, fintech, SaaS, healthcare.

## 37.1 — Domain Discovery: Event Storming

### Bài toán: Order-Taking System

Hệ thống nhận đơn hàng từ khách, validate, tính giá, fulfillment.

### Events (những gì xảy ra)

```
OrderPlaced → OrderValidated → OrderPriced → OrderConfirmed → OrderShipped
         ↘                                            ↗
          ValidationFailed                    ShippingFailed
```

### Commands (trigger events)

```
PlaceOrder → ValidateOrder → PriceOrder → ConfirmOrder → ShipOrder
```

### Domain types emerge

```rust
// filename: src/main.rs

// ═══ VALUE OBJECTS ═══

/// Order ID — unique, non-empty
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

/// Customer Email — validated format
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

/// Quantity — positive, bounded
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

/// Price — non-negative cents
#[derive(Debug, Clone, Copy, PartialEq)]
struct Price(u64);

impl Price {
    fn new(cents: u64) -> Result<Self, String> {
        if cents > 100_000_000 { return Err("Price exceeds max".into()); }
        Ok(Price(cents))
    }
    fn cents(&self) -> u64 { self.0 }
    fn multiply(&self, qty: Quantity) -> Price {
        Price(self.0 * qty.value() as u64)
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}đ", self.0 / 100, self.0 % 100)
    }
}

/// Product Code
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

fn main() {
    // Smart constructors enforce ALL rules at creation
    println!("{:?}", OrderId::new("ORD-001"));
    println!("{:?}", EmailAddress::new("minh@co.com"));
    println!("{:?}", Quantity::new(5));
    println!("{:?}", Price::new(85_000));
    println!("{:?}", ProductCode::new("W1234"));
    println!("{:?}", ProductCode::new("G567"));

    // Errors caught early
    println!("{:?}", OrderId::new(""));        // Err
    println!("{:?}", Quantity::new(0));          // Err
    println!("{:?}", ProductCode::new("X999"));  // Err
}
```

---

## 37.2 — Order State Machine

```rust
// filename: src/main.rs

// Re-use value objects from 37.1 (simplified here)
#[derive(Debug, Clone)] struct OrderId(String);
#[derive(Debug, Clone)] struct EmailAddress(String);
#[derive(Debug, Clone, Copy)] struct Quantity(u32);
#[derive(Debug, Clone, Copy)] struct Price(u64);
#[derive(Debug, Clone)] struct ProductCode(String);

// ═══ ORDER LINE ═══
#[derive(Debug, Clone)]
struct OrderLine {
    product: ProductCode,
    quantity: Quantity,
    price: Price,
}

impl OrderLine {
    fn line_total(&self) -> Price {
        Price(self.price.0 * self.quantity.0 as u64)
    }
}

// ═══ STATE MACHINE: each state = different type ═══

/// Unvalidated — raw input from customer
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

/// Validated — all fields parsed and validated
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

/// Priced — prices looked up, totals calculated
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

/// Confirmed — ready to ship
#[derive(Debug)]
struct ConfirmedOrder {
    order_id: OrderId,
    customer_email: EmailAddress,
    total: Price,
    confirmation_number: String,
}

fn main() {
    // Types ENFORCE the workflow:
    // UnvalidatedOrder → ValidatedOrder → PricedOrder → ConfirmedOrder
    //
    // Cannot price an unvalidated order (different types!)
    // Cannot confirm an unpriced order (different types!)
    println!("State machine: types enforce workflow.");
}
```

---

## 37.3 — Workflow Pipeline

```rust
// filename: src/main.rs

use std::collections::HashMap;

// ═══ Value Objects (simplified) ═══
#[derive(Debug, Clone)] struct OrderId(String);
#[derive(Debug, Clone)] struct EmailAddress(String);
#[derive(Debug, Clone, Copy)] struct Quantity(u32);
#[derive(Debug, Clone, Copy)] struct Price(u64);
#[derive(Debug, Clone)] struct ProductCode(String);

// ═══ States ═══
#[derive(Debug)]
struct UnvalidatedOrder {
    order_id: String,
    customer_email: String,
    lines: Vec<(String, u32)>, // (product_code, qty)
}

#[derive(Debug)]
struct ValidatedOrder {
    order_id: OrderId,
    customer_email: EmailAddress,
    lines: Vec<(ProductCode, Quantity)>,
}

#[derive(Debug)]
struct PricedOrder {
    order_id: OrderId,
    customer_email: EmailAddress,
    lines: Vec<(ProductCode, Quantity, Price, Price)>, // product, qty, unit_price, line_total
    subtotal: Price,
    tax: Price,
    total: Price,
}

#[derive(Debug)]
struct ConfirmedOrder {
    order_id: OrderId,
    total: Price,
    confirmation: String,
}

// ═══ ERRORS ═══
#[derive(Debug)]
enum OrderError {
    Validation(Vec<String>),
    Pricing(String),
    Confirmation(String),
}

// ═══ WORKFLOW: Pipeline of pure functions ═══

// Step 1: Validate (collect ALL errors)
fn validate_order(input: UnvalidatedOrder) -> Result<ValidatedOrder, OrderError> {
    let mut errors = vec![];

    let order_id = if input.order_id.trim().is_empty() {
        errors.push("OrderId is required".into()); None
    } else { Some(OrderId(input.order_id.trim().into())) };

    let email = if !input.customer_email.contains('@') {
        errors.push(format!("Invalid email: {}", input.customer_email)); None
    } else { Some(EmailAddress(input.customer_email.to_lowercase())) };

    let mut validated_lines = vec![];
    for (i, (code, qty)) in input.lines.iter().enumerate() {
        if code.is_empty() { errors.push(format!("Line {}: empty product code", i + 1)); }
        if *qty == 0 { errors.push(format!("Line {}: quantity must be > 0", i + 1)); }
        if *qty > 10_000 { errors.push(format!("Line {}: quantity too large", i + 1)); }
        if !code.is_empty() && *qty > 0 && *qty <= 10_000 {
            validated_lines.push((ProductCode(code.clone()), Quantity(*qty)));
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

// Step 2: Price (lookup prices, calculate totals)
fn price_order(
    order: ValidatedOrder,
    catalog: &HashMap<String, u64>,
) -> Result<PricedOrder, OrderError> {
    let mut priced_lines = vec![];

    for (product, qty) in &order.lines {
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

// Step 3: Confirm
fn confirm_order(order: PricedOrder) -> Result<ConfirmedOrder, OrderError> {
    if order.total.0 == 0 {
        return Err(OrderError::Confirmation("Order total cannot be 0".into()));
    }

    Ok(ConfirmedOrder {
        order_id: order.order_id,
        total: order.total,
        confirmation: format!("CONF-{}", chrono_like_id()),
    })
}

fn chrono_like_id() -> String { "20260304-001".into() }

// ═══ COMPOSE: Full workflow ═══
fn place_order(
    input: UnvalidatedOrder,
    catalog: &HashMap<String, u64>,
) -> Result<ConfirmedOrder, OrderError> {
    validate_order(input)
        .and_then(|validated| price_order(validated, catalog))
        .and_then(confirm_order)
}

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

fn main() {
    let mut catalog = HashMap::new();
    catalog.insert("W1234".into(), 85_000_u64);
    catalog.insert("G567".into(), 45_000_u64);
    catalog.insert("W5678".into(), 120_000_u64);

    // ═══ Happy path ═══
    println!("=== Happy Path ===");
    let order = UnvalidatedOrder {
        order_id: "ORD-001".into(),
        customer_email: "minh@company.com".into(),
        lines: vec![
            ("W1234".into(), 2),
            ("G567".into(), 5),
        ],
    };

    let result = place_order(order, &catalog);
    for event in to_events(&result) { println!("  Event: {:?}", event); }
    match &result {
        Ok(confirmed) => {
            println!("  ✅ Confirmed: {} — total: {}đ",
                confirmed.confirmation,
                confirmed.total.0);
        }
        Err(e) => println!("  ❌ {:?}", e),
    }

    // ═══ Validation failure ═══
    println!("\n=== Validation Failure ===");
    let bad_order = UnvalidatedOrder {
        order_id: "".into(),
        customer_email: "not-an-email".into(),
        lines: vec![("".into(), 0)],
    };

    let result = place_order(bad_order, &catalog);
    for event in to_events(&result) { println!("  Event: {:?}", event); }

    // ═══ Pricing failure ═══
    println!("\n=== Pricing Failure ===");
    let unknown = UnvalidatedOrder {
        order_id: "ORD-002".into(),
        customer_email: "lan@co.com".into(),
        lines: vec![("UNKNOWN".into(), 1)],
    };

    match place_order(unknown, &catalog) {
        Err(e) => println!("  ❌ {:?}", e),
        Ok(_) => println!("  ✅ ok"),
    }
}
```

---

## 37.4 — Testing the Domain

```rust
// filename: src/lib.rs (tests section)

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_catalog() -> HashMap<String, u64> {
        let mut c = HashMap::new();
        c.insert("W1234".into(), 85_000);
        c.insert("G567".into(), 45_000);
        c
    }

    fn valid_order() -> UnvalidatedOrder {
        UnvalidatedOrder {
            order_id: "ORD-TEST".into(),
            customer_email: "test@co.com".into(),
            lines: vec![("W1234".into(), 2)],
        }
    }

    // ═══ Validation tests ═══
    #[test]
    fn validates_good_order() {
        assert!(validate_order(valid_order()).is_ok());
    }

    #[test]
    fn rejects_empty_order_id() {
        let mut o = valid_order();
        o.order_id = "".into();
        let errs = validate_order(o).unwrap_err();
        match errs {
            OrderError::Validation(e) => assert!(e.iter().any(|s| s.contains("OrderId"))),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn rejects_invalid_email() {
        let mut o = valid_order();
        o.customer_email = "nope".into();
        assert!(validate_order(o).is_err());
    }

    #[test]
    fn rejects_zero_quantity() {
        let mut o = valid_order();
        o.lines = vec![("W1234".into(), 0)];
        assert!(validate_order(o).is_err());
    }

    #[test]
    fn collects_all_validation_errors() {
        let o = UnvalidatedOrder {
            order_id: "".into(),
            customer_email: "bad".into(),
            lines: vec![("".into(), 0)],
        };
        match validate_order(o) {
            Err(OrderError::Validation(errors)) => assert!(errors.len() >= 3),
            _ => panic!("Expected multiple errors"),
        }
    }

    // ═══ Pricing tests ═══
    #[test]
    fn prices_order_correctly() {
        let validated = validate_order(valid_order()).unwrap();
        let priced = price_order(validated, &sample_catalog()).unwrap();
        assert_eq!(priced.subtotal.0, 170_000); // 85k * 2
        assert_eq!(priced.tax.0, 17_000);       // 10%
        assert_eq!(priced.total.0, 187_000);     // subtotal + tax
    }

    #[test]
    fn unknown_product_fails_pricing() {
        let o = UnvalidatedOrder {
            order_id: "ORD-X".into(),
            customer_email: "x@co.com".into(),
            lines: vec![("UNKNOWN".into(), 1)],
        };
        let validated = validate_order(o).unwrap();
        assert!(price_order(validated, &sample_catalog()).is_err());
    }

    // ═══ Full workflow tests ═══
    #[test]
    fn full_happy_path() {
        let result = place_order(valid_order(), &sample_catalog());
        assert!(result.is_ok());
        let confirmed = result.unwrap();
        assert!(confirmed.confirmation.starts_with("CONF-"));
    }

    #[test]
    fn full_validation_failure() {
        let bad = UnvalidatedOrder {
            order_id: "".into(),
            customer_email: "bad".into(),
            lines: vec![],
        };
        assert!(place_order(bad, &sample_catalog()).is_err());
    }
}
```

---

## 37.5 — Architecture Overview

```
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
│  ═══ ALL PURE FUNCTIONS ═══                          │
│  No IO, No side effects, No database                 │
│  Just types + functions + Result                     │
└──────────────────────────────────────────────────────┘
```

### What we used from each chapter

| Chapter | Concept | Used for |
|---------|---------|----------|
| Ch 14 — Enums | `ProductCode::Widget\|Gizmo` | Typed product codes |
| Ch 15 — Pattern Matching | `match` in validate/price | Branching |
| Ch 22 — Domain Modeling | Newtypes, smart constructors | `OrderId`, `Email`, `Quantity`, `Price` |
| Ch 23 — Workflows | Pipeline `→` | State transitions |
| Ch 24 — ROP | `and_then` chain, collect errors | Validation + pricing flow |
| Ch 25 — Serialization | Event types | `OrderEvent` |
| Ch 27 — Evolution | Add variants safely | `ProductCode`, `OrderError` |
| Ch 33 — TDD | Tests | 9 domain tests |
| Ch 34 — PBT | Property tests | Round-trip, invariants |

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Add shipping

Thêm `ShippingMethod` vào order:
```rust
enum ShippingMethod { Standard, Express, SameDay }
```
Mỗi method có shipping cost khác nhau. Thêm vào pipeline giữa price và confirm.

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
        .map(|p| add_shipping(p, shipping))
        .and_then(confirm_order)
}
```

</details>

---

**Bài 2** (15 phút): Discount rules

Implement business rules:
- Orders > 500.000đ → 5% discount
- Orders > 1.000.000đ → 10% discount
- VIP customers → extra 5% on top

Thêm `apply_discount` step vào pipeline.

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

fn apply_discount(order: PricedOrder, is_vip: bool) -> PricedOrder {
    let discount_pct = calculate_discount(order.subtotal.0, is_vip);
    let discount_amount = order.subtotal.0 * discount_pct as u64 / 100;
    let new_subtotal = order.subtotal.0 - discount_amount;
    let new_tax = new_subtotal * 10 / 100;
    PricedOrder {
        subtotal: Price(new_subtotal),
        tax: Price(new_tax),
        total: Price(new_subtotal + new_tax),
        ..order
    }
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
| "Quá nhiều types" | Mỗi state = type riêng | Đúng! Types = documentation. Compiler enforces transitions |
| "Pipeline dài" | Nhiều steps | Tách helper functions, keep pipeline flat |
| "Validation collect errors phức tạp" | Manual Vec<String> | Validated pattern (Ch 24) hoặc custom macro |
| "Clone overhead" | Chuyển ownership qua steps | Dùng owned values, consume mỗi step |

---

## Tóm tắt

- ✅ **Event Storming** → Commands → Events → Types: Discovery → Design → Code.
- ✅ **Value Objects**: `OrderId`, `EmailAddress`, `Quantity`, `Price`, `ProductCode` — smart constructors, validated at creation.
- ✅ **State Machine**: `UnvalidatedOrder` → `ValidatedOrder` → `PricedOrder` → `ConfirmedOrder`. Compiler enforces transitions!
- ✅ **Pipeline**: `validate.and_then(price).and_then(confirm)` — ROP in action.
- ✅ **Error handling**: Collect ALL validation errors, `OrderError` enum.
- ✅ **Testing**: 9 unit tests covering validation, pricing, full workflow.
- ✅ **All pure functions**: No IO, no database — domain logic readable, testable, composable.

## Tiếp theo

→ Chapter 38: **Database Fundamentals & SQL** — relational model, JOINs, indexing, transactions, Rust crates (`sqlx`, `diesel`).
