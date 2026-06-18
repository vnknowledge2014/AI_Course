# Chapter 23 — Workflows as Pipelines

> **Bạn sẽ học được**:
> - **Workflow = chain of functions**: validate → price → fulfill → notify
> - **Method chaining** — builder-style pipelines
> - **`and_then`** chains — compose `Result`-returning functions
> - **Pipeline patterns**: linear, branching, parallel
> - Custom **`pipe!` macro** cho Rust
> - Real-world workflow: Order processing end-to-end
>
> **Yêu cầu trước**: Chapter 10 (Error Handling), Chapter 13 (HOF & Composition), Chapter 22 (Domain Modeling).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn compose domain operations thành **type-safe pipelines** — mỗi step là một function nhỏ, testable.

---

## Workflows as Pipelines — Core Pattern của DDD Functional

Đây là chapter quan trọng nhất trong Part IV — nó kết nối mọi concept trước đó thành một pattern hoàn chỉnh.

Scott Wlaschin trong *Domain Modeling Made Functional* mô tả business workflows như **pipelines**: data đi vào, chảy qua chuỗi transformations (validate → enrich → calculate → persist), và ra output. Mỗi step là pure function. Errors được xử lý qua Railway-Oriented Programming (ROP). Toàn bộ workflow testable, composable, và dễ hiểu.

Trong Rust, pattern này hiện thực hóa bằng `Result` chains, `and_then()`, và method chaining. Chapter này cho bạn template để xây dựng bất kỳ business workflow nào.

---

## 23.1 — Workflow = Chain of Functions

Khi bạn đặt một ly cà phê ở quán, có một chuỗi các bước diễn ra: nhận đơn → kiểm tra còn nguyên liệu → pha chế → đóng gói → gửi cho bạn. Mỗi bước làm **một việc** rồi truyền kết quả cho bước sau. Nếu bước nào fail (hết nguyên liệu) → dừng ngay, báo lỗi.

Trong lập trình, workflow cũng vậy: một chuỗi functions, mỗi function nhận đầu ra của function trước. `and_then` nối chúng lại — nếu Ok thì chạy tiếp, nếu Err thì dừng.

### Workflow đơn giản

```rust
// filename: src/main.rs

// Mỗi step = 1 pure function
fn parse_amount(input: &str) -> Result<u32, String> {
    input.trim().parse::<u32>()
        .map_err(|_| format!("Invalid amount: '{}'", input))
}

fn validate_amount(amount: u32) -> Result<u32, String> {
    if amount == 0 { Err("Amount must be > 0".into()) }
    else if amount > 10_000_000 { Err(format!("Amount {} exceeds limit", amount)) }
    else { Ok(amount) }
}

fn apply_tax(amount: u32) -> u32 {
    amount + amount * 8 / 100
}

fn format_receipt(amount: u32) -> String {
    format!("═══ RECEIPT ═══\n  Total: {}đ\n  Tax included\n═══════════════", amount)
}

fn main() {
    // Pipeline: parse → validate → tax → format
    let result = parse_amount("  500000  ")
        .and_then(validate_amount)
        .map(apply_tax)
        .map(format_receipt);

    match result {
        Ok(receipt) => println!("{}", receipt),
        Err(e) => println!("❌ {}", e),
    }

    // Error case — pipeline stops at first error
    let err = parse_amount("abc")
        .and_then(validate_amount)
        .map(apply_tax);
    println!("\nInvalid: {:?}", err);
}
```

> **💡 Key insight**: `and_then` = "nếu Ok → chạy step tiếp, nếu Err → dừng". Pipeline **tự động** xử lý errors!

---

## 23.2 — Method Chaining: Builder-style Pipelines

Section trước dùng free functions. Nhưng khi workflow phức tạp hơn, viết methods trên struct sẽ gọn hơn — giống builder pattern. Mỗi method trả `Self` (chainable) hoặc `Result<Self>` (có validation).

### Workflow trên domain types

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
struct OrderWorkflow {
    customer: String,
    items: Vec<(String, u32, u32)>,
    discount: u32,
    tax_rate: u32,
    subtotal: u32,
    total: u32,
    status: String,
    notes: Vec<String>,
}

impl OrderWorkflow {
    fn new(customer: &str) -> Self {
        OrderWorkflow {
            customer: customer.into(),
            items: vec![], discount: 0, tax_rate: 8,
            subtotal: 0, total: 0,
            status: "created".into(),
            notes: vec![],
        }
    }

    // Step 1: Add items (chainable)
    fn add_item(mut self, name: &str, price: u32, qty: u32) -> Self {
        self.items.push((name.into(), price, qty));
        self.notes.push(format!("Added {} x{}", name, qty));
        self
    }

    // Step 2: Apply discount
    fn with_discount(mut self, percent: u32) -> Self {
        self.discount = percent;
        self.notes.push(format!("Discount: {}%", percent));
        self
    }

    // Step 3: Calculate pricing
    fn calculate(mut self) -> Result<Self, String> {
        if self.items.is_empty() {
            return Err("Cannot calculate empty order".into());
        }
        self.subtotal = self.items.iter().map(|(_, p, q)| p * q).sum();
        let after_discount = self.subtotal * (100 - self.discount) / 100;
        self.total = after_discount + after_discount * self.tax_rate / 100;
        self.status = "priced".into();
        self.notes.push(format!("Subtotal: {}đ → Total: {}đ", self.subtotal, self.total));
        Ok(self)
    }

    // Step 4: Validate
    fn validate(self) -> Result<Self, String> {
        if self.total > 100_000_000 {
            Err(format!("Order total {} exceeds limit", self.total))
        } else {
            Ok(self)
        }
    }

    // Step 5: Confirm
    fn confirm(mut self) -> Self {
        self.status = "confirmed".into();
        self.notes.push("Order confirmed ✅".into());
        self
    }

    fn summary(&self) -> String {
        format!(
            "Order for {} | {} items | Total: {}đ | Status: {}",
            self.customer, self.items.len(), self.total, self.status
        )
    }
}

fn main() {
    // Fluent pipeline
    let result = OrderWorkflow::new("Minh")
        .add_item("Coffee", 35_000, 3)
        .add_item("Cake", 25_000, 2)
        .with_discount(10)
        .calculate()
        .and_then(|o| o.validate())
        .map(|o| o.confirm());

    match result {
        Ok(order) => {
            println!("{}\n", order.summary());
            println!("📝 Workflow log:");
            for note in &order.notes {
                println!("  → {}", note);
            }
        }
        Err(e) => println!("❌ {}", e),
    }
}
```

---

## ✅ Checkpoint 23.2

> Ghi nhớ:
> 1. **Method chaining**: `self` methods trả `Self` — chainable. `Result<Self>` khi có validation.
> 2. **`and_then`**: nối `Result`-returning methods. Auto-stops on `Err`.
> 3. **`.map`**: transform `Ok` value. Giữ nguyên `Err`.
>
> **Test nhanh**: Thứ tự nào LỖI: `Ok(5).and_then(|x| Err("boom")).map(|x| x + 1)`?
> <details><summary>Đáp án</summary><code>Err("boom")</code> — <code>and_then</code> trả Err, <code>.map</code> skip vì đã Err.</details>

---

## 23.3 — Composable Steps: Tách workflow thành building blocks

Bước tiếp theo: thay vì dùng cùng struct xuyên suốt, mỗi step nhận **một type** và trả **type khác**. `UnvalidatedOrder` vào → `ValidatedOrder` ra → `PricedOrder` ra → `ConfirmedOrder`. Compiler đảm bảo bạn không skip bước nào — nếu hàm `price()` đòi `ValidatedOrder` mà bạn đưa `UnvalidatedOrder` vào, code không compile.

### Mỗi step = independent function

```rust
// filename: src/main.rs
use std::collections::HashMap;

// ═══ Domain Types ═══
#[derive(Debug, Clone)]
struct UnvalidatedOrder {
    customer_name: String,
    customer_email: String,
    items: Vec<(String, u32, u32)>,
}

#[derive(Debug, Clone)]
struct ValidatedOrder {
    customer_name: String,
    customer_email: String,
    items: Vec<(String, u32, u32)>,
}

#[derive(Debug, Clone)]
struct PricedOrder {
    customer_name: String,
    customer_email: String,
    lines: Vec<OrderLine>,
    total: u32,
}

#[derive(Debug, Clone)]
struct OrderLine {
    product: String,
    price: u32,
    quantity: u32,
    subtotal: u32,
}

#[derive(Debug, Clone)]
struct ConfirmedOrder {
    order_id: String,
    customer_name: String,
    customer_email: String,
    total: u32,
}

// ═══ Pipeline Steps (each is a pure function!) ═══

fn validate(order: UnvalidatedOrder) -> Result<ValidatedOrder, String> {
    if order.customer_name.trim().len() < 2 {
        return Err("Name too short".into());
    }
    if !order.customer_email.contains('@') {
        return Err("Invalid email".into());
    }
    if order.items.is_empty() {
        return Err("No items".into());
    }
    Ok(ValidatedOrder {
        customer_name: order.customer_name.trim().into(),
        customer_email: order.customer_email.to_lowercase(),
        items: order.items,
    })
}

fn price(order: ValidatedOrder) -> Result<PricedOrder, String> {
    let lines: Vec<OrderLine> = order.items.iter()
        .map(|(name, price, qty)| OrderLine {
            product: name.clone(),
            price: *price,
            quantity: *qty,
            subtotal: price * qty,
        })
        .collect();

    let total: u32 = lines.iter().map(|l| l.subtotal).sum();

    if total == 0 {
        return Err("Order total cannot be zero".into());
    }

    Ok(PricedOrder {
        customer_name: order.customer_name,
        customer_email: order.customer_email,
        lines, total,
    })
}

fn confirm(order: PricedOrder) -> ConfirmedOrder {
    ConfirmedOrder {
        order_id: format!("ORD-{}", rand_id()),
        customer_name: order.customer_name,
        customer_email: order.customer_email,
        total: order.total,
    }
}

fn rand_id() -> u32 {
    // Simplified — dùng uuid trong production
    42
}

fn notify(order: &ConfirmedOrder) -> String {
    format!(
        "📧 To: {}\n   Order {} confirmed!\n   Total: {}đ",
        order.customer_email, order.order_id, order.total
    )
}

fn main() {
    let input = UnvalidatedOrder {
        customer_name: "  Minh Nguyen  ".into(),
        customer_email: "Minh@Company.COM".into(),
        items: vec![
            ("Coffee".into(), 35_000, 3),
            ("Pastry".into(), 45_000, 2),
        ],
    };

    // Pipeline: Unvalidated → Validated → Priced → Confirmed
    let result = validate(input)
        .and_then(price)
        .map(confirm);

    match result {
        Ok(order) => {
            println!("✅ {}", notify(&order));
        }
        Err(e) => println!("❌ Pipeline failed: {}", e),
    }
}
```

### Type-driven workflow

```
UnvalidatedOrder ──validate──→ ValidatedOrder ──price──→ PricedOrder ──confirm──→ ConfirmedOrder
      ↓ Err                         ↓ Err                                            ↓
   (parse error)              (business error)                                  (send email)
```

> **💡 Key insight**: Input type ≠ Output type. Mỗi step **transform** từ type này sang type khác. Compiler verify rằng bạn không skip step nào!

---

## 23.4 — Branching & Parallel Pipelines

Không phải workflow nào cũng thẳng. Thanh toán có nhiều phương thức (card, bank transfer, COD) — mỗi loại xử lý khác. Hoặc bạn cần validate 100 emails cùng lúc, gom kết quả thành/ại riêng.

### Branching: Different paths based on condition

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
struct Payment { amount: u32, method: String }

#[derive(Debug)]
struct Receipt { id: String, amount: u32, method: String }

fn process_card(payment: &Payment) -> Result<Receipt, String> {
    if payment.amount > 50_000_000 {
        Err("Card limit exceeded".into())
    } else {
        Ok(Receipt {
            id: format!("CARD-{}", payment.amount),
            amount: payment.amount,
            method: "card".into(),
        })
    }
}

fn process_transfer(payment: &Payment) -> Result<Receipt, String> {
    Ok(Receipt {
        id: format!("TRF-{}", payment.amount),
        amount: payment.amount,
        method: "transfer".into(),
    })
}

fn process_cod(payment: &Payment) -> Result<Receipt, String> {
    if payment.amount > 5_000_000 {
        Err("COD limit: 5,000,000đ".into())
    } else {
        Ok(Receipt {
            id: format!("COD-{}", payment.amount),
            amount: payment.amount,
            method: "cod".into(),
        })
    }
}

// Branch: route to different processors
fn process_payment(payment: &Payment) -> Result<Receipt, String> {
    match payment.method.as_str() {
        "card" => process_card(payment),
        "transfer" => process_transfer(payment),
        "cod" => process_cod(payment),
        other => Err(format!("Unknown method: {}", other)),
    }
}

fn main() {
    let payments = vec![
        Payment { amount: 500_000, method: "card".into() },
        Payment { amount: 2_000_000, method: "transfer".into() },
        Payment { amount: 3_000_000, method: "cod".into() },
        Payment { amount: 10_000_000, method: "cod".into() },  // exceeds COD limit
    ];

    for p in &payments {
        match process_payment(p) {
            Ok(r) => println!("✅ {:?}", r),
            Err(e) => println!("❌ {} {}: {}", p.method, p.amount, e),
        }
    }
}
```

### Parallel: Process multiple items independently

```rust
// filename: src/main.rs

#[derive(Debug)]
struct BatchResult {
    successes: Vec<String>,
    failures: Vec<String>,
}

fn validate_email(email: &str) -> Result<String, String> {
    if email.contains('@') && email.len() >= 5 {
        Ok(email.to_lowercase())
    } else {
        Err(format!("Invalid: {}", email))
    }
}

fn process_batch(emails: &[&str]) -> BatchResult {
    let (ok, err): (Vec<_>, Vec<_>) = emails.iter()
        .map(|e| validate_email(e))
        .partition(Result::is_ok);

    BatchResult {
        successes: ok.into_iter().map(|r| r.unwrap()).collect(),
        failures: err.into_iter().map(|r| r.unwrap_err()).collect(),
    }
}

fn main() {
    let emails = vec!["minh@co.com", "bad", "lan@co.com", "x", "hai@co.com"];
    let result = process_batch(&emails);

    println!("✅ Valid ({}):", result.successes.len());
    for e in &result.successes { println!("  {}", e); }
    println!("❌ Invalid ({}):", result.failures.len());
    for e in &result.failures { println!("  {}", e); }
}
```

---

## 23.5 — Custom `pipe!` Macro

Rust không có `|>` operator như Elixir/F#. Nhưng dùng macro, bạn có thể viết pipeline trái-sang-phải thay vì lồng nhất như `to_string(double(add_ten(5)))`:

```rust
// filename: src/main.rs

/// pipe! macro — chain functions trái-sang-phải
macro_rules! pipe {
    // Base: 1 value
    ($value:expr) => { $value };
    // Chain: value |> f1 |> f2 ...
    ($value:expr => $func:expr) => { $func($value) };
    ($value:expr => $func:expr $(=> $rest:expr)+) => {
        pipe!($func($value) $(=> $rest)+)
    };
}

/// pipe_result! — chain Result-returning functions
macro_rules! pipe_result {
    ($value:expr) => { Ok($value) };
    ($value:expr => $func:expr) => { $func($value) };
    ($value:expr => $func:expr $(=> $rest:expr)+) => {
        $func($value).and_then(|v| pipe_result!(v $(=> $rest)+))
    };
}

fn add_ten(x: i32) -> i32 { x + 10 }
fn double(x: i32) -> i32 { x * 2 }
fn to_string(x: i32) -> String { format!("Result: {}", x) }

fn parse(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| format!("Parse error: {}", s))
}
fn validate_positive(x: i32) -> Result<i32, String> {
    if x > 0 { Ok(x) } else { Err("Must be positive".into()) }
}
fn format_money(x: i32) -> Result<String, String> {
    Ok(format!("{}đ", x))
}

fn main() {
    // pipe! — infallible chain
    let result = pipe!(5 => add_ten => double => to_string);
    println!("{}", result);  // Result: 30

    // pipe_result! — fallible chain (stops on Err)
    let ok = pipe_result!("42" => parse => validate_positive => format_money);
    println!("{:?}", ok);  // Ok("42đ")

    let err = pipe_result!("-5" => parse => validate_positive => format_money);
    println!("{:?}", err);  // Err("Must be positive")
}
```

---

## 23.6 — Real-world: Order Processing Pipeline

Đây là workflow hoàn chỉnh: từ raw input người dùng đến đơn hàng thành phẩm. Tất cả concepts ở trên gộp lại: validation, coupon lookup, pricing, và `and_then`/`map` nối chúng thành một pipeline duy nhất.

```rust
// filename: src/main.rs
use std::collections::HashMap;

// ═══ Complete workflow: raw input → fulfilled order ═══

#[derive(Debug)]
struct RawInput {
    customer: String,
    email: String,
    items: Vec<(String, u32)>,
    coupon: Option<String>,
}

#[derive(Debug)]
struct FulfilledOrder {
    order_id: String,
    customer: String,
    email: String,
    items: Vec<String>,
    subtotal: u32,
    discount: u32,
    tax: u32,
    total: u32,
}

// Step 1: Validate input
fn validate_input(input: RawInput) -> Result<RawInput, String> {
    if input.customer.trim().len() < 2 { return Err("Name required".into()); }
    if !input.email.contains('@') { return Err("Invalid email".into()); }
    if input.items.is_empty() { return Err("No items".into()); }
    Ok(input)
}

// Step 2: Look up coupon
fn apply_coupon(input: RawInput) -> Result<(RawInput, u32), String> {
    let discount = match input.coupon.as_deref() {
        Some("SAVE10") => 10,
        Some("VIP20") => 20,
        Some(code) => return Err(format!("Unknown coupon: {}", code)),
        None => 0,
    };
    Ok((input, discount))
}

// Step 3: Calculate totals
fn calculate_totals(input: RawInput, discount_pct: u32) -> FulfilledOrder {
    let subtotal: u32 = input.items.iter().map(|(_, p)| p).sum();
    let discount = subtotal * discount_pct / 100;
    let after_discount = subtotal - discount;
    let tax = after_discount * 8 / 100;
    let total = after_discount + tax;

    FulfilledOrder {
        order_id: format!("ORD-{:04}", 1),
        customer: input.customer.trim().into(),
        email: input.email.to_lowercase(),
        items: input.items.iter().map(|(n, _)| n.clone()).collect(),
        subtotal, discount, tax, total,
    }
}

// Full pipeline
fn process_order(input: RawInput) -> Result<FulfilledOrder, String> {
    validate_input(input)
        .and_then(apply_coupon)
        .map(|(input, discount)| calculate_totals(input, discount))
}

fn main() {
    let order = RawInput {
        customer: "Minh Nguyen".into(),
        email: "minh@co.com".into(),
        items: vec![
            ("Coffee beans 500g".into(), 185_000),
            ("Filter set".into(), 95_000),
            ("Mug ceramic".into(), 120_000),
        ],
        coupon: Some("SAVE10".into()),
    };

    match process_order(order) {
        Ok(fulfilled) => {
            println!("╔══════════════════════════╗");
            println!("║    ORDER CONFIRMATION    ║");
            println!("╠══════════════════════════╣");
            println!("║ {} ", fulfilled.order_id);
            println!("║ Customer: {}", fulfilled.customer);
            println!("║ Email: {}", fulfilled.email);
            println!("║──────────────────────────║");
            for item in &fulfilled.items {
                println!("║  📦 {}", item);
            }
            println!("║──────────────────────────║");
            println!("║ Subtotal:  {:>10}đ", fulfilled.subtotal);
            println!("║ Discount: -{:>10}đ", fulfilled.discount);
            println!("║ Tax (8%):  {:>10}đ", fulfilled.tax);
            println!("║ TOTAL:     {:>10}đ", fulfilled.total);
            println!("╚══════════════════════════╝");
        }
        Err(e) => println!("❌ {}", e),
    }
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Chain reading

```rust
let x = Ok(10)
    .and_then(|n| if n > 5 { Ok(n * 2) } else { Err("too small") })
    .map(|n| n + 1)
    .and_then(|n| if n > 25 { Err("too big") } else { Ok(n) });
```
`x` = ?

<details><summary>✅ Lời giải</summary>

```
Ok(10) → n=10 > 5 → Ok(20) → map +1 → Ok(21) → 21 > 25? No → Ok(21)
x = Ok(21)
```

</details>

---

**Bài 2** (10 phút): Registration pipeline

Viết pipeline 4 steps: `parse_input` → `validate_age` (18-120) → `check_duplicate` (against list) → `create_account`. Mỗi step trả `Result`.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
fn parse_input(input: &str) -> Result<(String, u32), String> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 2 { return Err("Format: name,age".into()); }
    let age = parts[1].trim().parse().map_err(|_| "Invalid age")?;
    Ok((parts[0].trim().into(), age))
}

fn validate_age((name, age): (String, u32)) -> Result<(String, u32), String> {
    if age < 18 || age > 120 { Err(format!("Age {} not in 18-120", age)) }
    else { Ok((name, age)) }
}

fn check_duplicate(existing: &[&str]) -> impl Fn((String, u32)) -> Result<(String, u32), String> + '_ {
    move |(name, age)| {
        if existing.iter().any(|&e| e == name) { Err(format!("{} already exists", name)) }
        else { Ok((name, age)) }
    }
}

fn create_account((name, age): (String, u32)) -> Result<String, String> {
    Ok(format!("Account created: {} (age {})", name, age))
}

fn main() {
    let existing = vec!["Minh", "Lan"];

    let result = parse_input("Hùng, 25")
        .and_then(validate_age)
        .and_then(check_duplicate(&existing))
        .and_then(create_account);

    println!("{:?}", result);  // Ok("Account created: Hùng (age 25)")
}
```

</details>

---

**Bài 3** (15 phút): Invoice pipeline

Viết workflow tạo invoice:
1. `validate_customer` → kiểm tra name, email
2. `validate_line_items` → mỗi item phải có price > 0, qty > 0
3. `calculate_invoice` → subtotal, tax, total
4. `generate_invoice_number` → format: `INV-YYYY-NNNN`
5. Compose thành 1 pipeline, test cả happy path và error cases

<details><summary>✅ Lời giải Bài 3</summary>

```rust
#[derive(Debug, Clone)]
struct InvoiceInput {
    customer: String,
    email: String,
    items: Vec<(String, u32, u32)>, // name, price, qty
}

#[derive(Debug)]
struct Invoice {
    number: String,
    customer: String,
    subtotal: u32,
    tax: u32,
    total: u32,
}

fn validate_customer(input: InvoiceInput) -> Result<InvoiceInput, String> {
    if input.customer.trim().len() < 2 { return Err("Customer name required".into()); }
    if !input.email.contains('@') { return Err("Invalid email".into()); }
    Ok(input)
}

fn validate_items(input: InvoiceInput) -> Result<InvoiceInput, String> {
    if input.items.is_empty() { return Err("No items".into()); }
    for (name, price, qty) in &input.items {
        if *price == 0 { return Err(format!("{}: price must be > 0", name)); }
        if *qty == 0 { return Err(format!("{}: qty must be > 0", name)); }
    }
    Ok(input)
}

fn calculate(input: InvoiceInput) -> Invoice {
    let subtotal: u32 = input.items.iter().map(|(_, p, q)| p * q).sum();
    let tax = subtotal * 10 / 100;
    Invoice {
        number: format!("INV-2024-{:04}", 1),
        customer: input.customer,
        subtotal, tax, total: subtotal + tax,
    }
}

fn process(input: InvoiceInput) -> Result<Invoice, String> {
    validate_customer(input)
        .and_then(validate_items)
        .map(calculate)
}

fn main() {
    let input = InvoiceInput {
        customer: "ABC Corp".into(),
        email: "billing@abc.com".into(),
        items: vec![("Service A".into(), 1_000_000, 2), ("Service B".into(), 500_000, 1)],
    };
    match process(input) {
        Ok(inv) => println!("{} — {}đ", inv.number, inv.total),
        Err(e) => println!("❌ {}", e),
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| `and_then` type mismatch | Return type khác `Result<T, E>` | Đảm bảo Error type thống nhất (dùng `String` hoặc custom error) |
| Pipeline quá dài, khó đọc | >5 chained calls | Tách thành named functions, group vào modules |
| "Muốn collect ALL errors" | `and_then` stops at first | Dùng `Validated` pattern (Chapter 24 sẽ học!) |
| Macro compile error | Syntax phức tạp | Start simple, test từng case |

---

## Tóm tắt

- ✅ **Workflow = chain of functions**: `validate() → price() → confirm()`. Mỗi step = 1 pure function.
- ✅ **`and_then`** chains: `Ok → next step, Err → stop`. Automatic error handling.
- ✅ **Method chaining**: `Order::new().add_item().with_discount().calculate()` — builder-style.
- ✅ **Type-driven**: Input type ≠ Output type. `UnvalidatedOrder → ValidatedOrder → PricedOrder`. Compiler verify.
- ✅ **Branching**: `match` trên payment method → different processors.
- ✅ **`pipe!` macro**: `pipe!(5 => add_ten => double)` — F#/Elixir style.

## Tiếp theo

→ Chapter 24: **Railway-Oriented Programming** ⭐ — chapter quan trọng nhất của Part IV! Bạn sẽ học `Result` = two-track model, compose validations thu thập **TẤT CẢ** errors (không chỉ error đầu tiên), `map`/`and_then`/`map_err` chaining patterns, và custom `Validated<T>` type.
