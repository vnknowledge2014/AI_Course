# Chapter 22 — Domain Modeling with Rust Types

> **Bạn sẽ học được**:
> - **Value Objects** — newtypes + smart constructors + equality by value
> - **Entities** — identity (ID), lifecycle, mutation via functional update
> - **Aggregates** — consistency boundaries, invariant enforcement
> - **State machines** hoàn chỉnh — enum states + typed transitions
> - Module encapsulation — **private constructors**, chỉ expose API
>
> **Yêu cầu trước**: Chapter 14 (Algebraic Types), Chapter 20 (DDD Intro), Chapter 21 (Architecture).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn model domain types sao cho **compiler bắt business rule violations** — "parse, don't validate."

---

## 22.1 — Value Objects: "Giá trị, không phải vật thể"

### Định nghĩa

Bạn có 2 tờ 100 nghìn đồng. Tờ nào cũng giá trị như nhau — bạn không bao giờ nói "tờ NÀY đặc biệt hơn tờ KIA". Khi đi mua cà phê, bạn đưa tờ nào cũng được. Không ai quan tâm serial number của tờ tiền.

Trong lập trình, có rất nhiều dữ liệu hoạt động y hệt: email `minh@company.com` dù ở biến nào cũng là cùng email đó. Số tiền 500,000đ dù lưu ở đâu cũng là 500,000đ. Chúng được định nghĩa bởi **giá trị**, không phải bởi danh tính.

Đó là Value Object — đối tượng **không có identity**. Hai Value Objects **bằng nhau** nếu mọi trường bằng nhau. Giống tờ tiền: tờ 100k nào cũng như tờ 100k nào — không quan trọng "tờ NÀO".

Nhưng có một thứ quan trọng hơn: Value Objects phải luôn **hợp lệ**. Bạn không muốn có email thiếu ký tự `@`, hay số tiền âm. Vì thế ta dùng **smart constructor** — function kiểm tra dữ liệu trước khi tạo, và trả lỗi nếu không hợp lệ. Một khi đã có Value Object, bạn biết chắc nó valid — không cần kiểm tra lại.

### Smart Constructor Pattern

```rust
// filename: src/main.rs
use std::fmt;

// ═══════ Value Objects ═══════

/// Email — validated, normalized, immutable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String); // private field!

impl Email {
    /// Smart constructor: validate + normalize
    pub fn new(value: &str) -> Result<Self, String> {
        let trimmed = value.trim().to_lowercase();
        if !trimmed.contains('@') {
            return Err(format!("Email missing @: '{}'", value));
        }
        let parts: Vec<&str> = trimmed.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].len() < 3 {
            return Err(format!("Invalid email format: '{}'", value));
        }
        Ok(Email(trimmed))
    }

    pub fn value(&self) -> &str { &self.0 }
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Money — VNĐ, luôn >= 0
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money(u64);

impl Money {
    pub fn new(amount: u64) -> Self { Money(amount) }
    pub fn zero() -> Self { Money(0) }
    pub fn value(&self) -> u64 { self.0 }

    pub fn add(&self, other: &Money) -> Money { Money(self.0 + other.0) }

    pub fn subtract(&self, other: &Money) -> Result<Money, String> {
        if other.0 > self.0 {
            Err(format!("Cannot subtract {}đ from {}đ", other.0, self.0))
        } else {
            Ok(Money(self.0 - other.0))
        }
    }

    pub fn multiply(&self, factor: u32) -> Money { Money(self.0 * factor as u64) }

    pub fn apply_percentage(&self, percent: u32) -> Money {
        Money(self.0 * percent as u64 / 100)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}đ", self.0)
    }
}

/// Quantity — luôn >= 1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(u32);

impl Quantity {
    pub fn new(value: u32) -> Result<Self, String> {
        if value == 0 { Err("Quantity must be at least 1".into()) }
        else { Ok(Quantity(value)) }
    }
    pub fn value(&self) -> u32 { self.0 }
}

fn main() {
    // Email: validated, normalized
    let email = Email::new("  MINH@Company.COM  ").unwrap();
    println!("Email: {} (domain: {})", email, email.domain());

    // Equality by value
    let email2 = Email::new("minh@company.com").unwrap();
    println!("Same? {}", email == email2); // true — cùng giá trị!

    // Money: safe arithmetic
    let price = Money::new(500_000);
    let tax = price.apply_percentage(8);
    let total = price.add(&tax);
    println!("Price: {}, Tax: {}, Total: {}", price, tax, total);

    // Quantity: min 1
    println!("Qty(3): {:?}", Quantity::new(3));  // Ok
    println!("Qty(0): {:?}", Quantity::new(0));  // Err
}
```

Nhìn lại: `Email::new()` không chỉ tạo — nó **validate và normalize** trong cùng bước. Email viết hoa hay thừa space? Không sao — smart constructor xử lý. Thiếu `@`? Trả `Err`. Một khi bạn có `Email` trong tay, bạn **biết chắc** nó hợp lệ — không cần validate lại ở bất kỳ đâu khác trong cả hệ thống. Đó là "parse, don't validate" — biến data thô thành data có **ý nghĩa** và **đảm bảo** từ type system.

### Value Object rules

| Rule | Giải thích |
|------|-----------|
| **Immutable** | Sau khi tạo, không thay đổi. "Update" = tạo mới |
| **Equality by value** | `Email("a@b") == Email("a@b")` is true |
| **Self-validating** | Smart constructor đảm bảo luôn valid |
| **No identity** | Không có ID. Tờ 100k nào cũng như nhau |
| **Private inner field** | `struct Email(String)` — field private, chỉ truy cập qua methods |

---

## ✅ Checkpoint 22.1

> **"Parse, don't validate"**:
> - ❌ `fn process(email: String)` — phải validate mỗi lần dùng
> - ✅ `fn process(email: Email)` — **đã validated** lúc construction. Dùng thoải mái!
>
> Value Object = "data đã qua validation" = contract tại type level.

---

## 22.2 — Entities: "Vật thể có danh tính"

Ngược lại với Value Objects, có những thứ trong đời bạn quan tâm đến **danh tính** chứ không chỉ giá trị. Bạn đổi tên, đổi email, đổi địa chỉ — nhưng vẫn là BẠN. Cái CMND/CCCD mới biết bạn là ai, không phải tên bạn.

Trong lập trình, một khách hàng có `id = 42` dù đổi email hay đổi hạng khách hàng vẫn là cùng một người. Hai khách hàng khác tên nhưng cùng ID → cùng entity. Hai khách hàng cùng tên nhưng khác ID → hai người khác nhau.

Entity = object có **identity** (ID). Đây là điểm khác biệt cốt lõi với Value Object: `Email("a@b.com") == Email("a@b.com")` vì giá trị giống nhau. Nhưng `Customer { id: 1, name: "Minh" } == Customer { id: 1, name: "Minh Khác" }` vẫn true — vì cùng ID dù tên khác.

```rust
// filename: src/main.rs

// ═══════ Entity ID — Value Object for identity ═══════
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomerId(u64);

impl CustomerId {
    pub fn new(id: u64) -> Self { CustomerId(id) }
}

impl std::fmt::Display for CustomerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CUST-{:05}", self.0)
    }
}

// ═══════ Entity ═══════
#[derive(Debug, Clone)]
pub struct Customer {
    id: CustomerId,            // identity — immutable!
    name: String,
    email: String,
    tier: CustomerTier,
    total_spent: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CustomerTier { Regular, Silver, Gold, Platinum }

impl Customer {
    pub fn new(id: CustomerId, name: &str, email: &str) -> Self {
        Customer {
            id, name: name.into(), email: email.into(),
            tier: CustomerTier::Regular, total_spent: 0,
        }
    }

    pub fn id(&self) -> CustomerId { self.id }

    // Functional update — trả entity MỚI
    pub fn update_email(&self, new_email: &str) -> Self {
        Customer { email: new_email.into(), ..self.clone() }
    }

    pub fn record_purchase(&self, amount: u64) -> Self {
        let new_total = self.total_spent + amount;
        let new_tier = match new_total {
            0..=999_999 => CustomerTier::Regular,
            1_000_000..=4_999_999 => CustomerTier::Silver,
            5_000_000..=19_999_999 => CustomerTier::Gold,
            _ => CustomerTier::Platinum,
        };
        Customer {
            total_spent: new_total,
            tier: new_tier,
            ..self.clone()
        }
    }

    pub fn discount_rate(&self) -> u32 {
        match self.tier {
            CustomerTier::Regular => 0,
            CustomerTier::Silver => 5,
            CustomerTier::Gold => 10,
            CustomerTier::Platinum => 15,
        }
    }
}

// Entity equality = by ID (not by fields!)
impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id // chỉ so sánh ID!
    }
}

fn main() {
    let customer = Customer::new(CustomerId::new(1), "Minh", "minh@co.com");
    println!("{}: {:?}", customer.id(), customer.tier);

    // Record purchases → tier upgrades
    let customer = customer.record_purchase(2_000_000);
    println!("After 2M: {:?}, discount {}%", customer.tier, customer.discount_rate());

    let customer = customer.record_purchase(3_500_000);
    println!("After 5.5M: {:?}, discount {}%", customer.tier, customer.discount_rate());

    // Entity equality = by ID
    let modified = customer.update_email("new@co.com");
    println!("Same entity? {}", customer == modified); // true — cùng ID!
}
```

Đọc lại đoạn code: `customer.record_purchase(2_000_000)` trả về **Customer mới** với tier đã được tự động nâng cấp. Customer cũ vẫn là Regular — vì chúng ta dùng **functional update** (trả bản mới, không thay đổi bản cũ). Và dòng cuối: `customer == modified` là `true` dù email khác — vì Entity so sánh bằng **ID**, không phải bằng tất cả fields. Đó là sự khác biệt cốt lõi giữa Entity và Value Object.

## 22.3 — Aggregates: Consistency Boundaries

### Aggregate = Cluster of entities + invariants

Bạn đã có Value Objects (tờ tiền — so sánh bằng giá trị) và Entities (con người — so sánh bằng ID). Nhưng trong domain thật, chúng không tồn tại riêng lẻ — chúng **thuộc về nhau**.

Nghĩ về một bưu kiện: bên trong có nhiều món hàng, có phiếu giao hàng, có thông tin người nhận. Bạn không thể lấy 1 món hàng ra khỏi bưu kiện rồi vứt vào bưu kiện khác — mọi thay đổi phải đi qua bưu kiện đó. Bưu kiện là **ranh giới** — bên ngoài chỉ tương tác qua bưu kiện, không đụng trực tiếp vào món hàng bên trong.

Aggregate giữ **business rules** (invariants) cho một nhóm objects liên quan. Bên ngoài chỉ tương tác qua **aggregate root** — giống cách bạn chỉ gửi/nhận bưu kiện qua bưu điện, không tự mở bưu kiện người khác.

Trong ví dụ dưới đây, `Order` là aggregate root — nó kiểm soát toàn bộ OrderLines bên trong và đảm bảo các invariants: không quá 20 items, không thêm items khi đã confirmed, không confirm đơn trống.

```rust
// filename: src/main.rs

// ═══════ Value Objects ═══════
#[derive(Debug, Clone, PartialEq)]
struct OrderId(u64);

#[derive(Debug, Clone)]
struct OrderLine {
    product_name: String,
    unit_price: u32,
    quantity: u32,
}

impl OrderLine {
    fn subtotal(&self) -> u32 { self.unit_price * self.quantity }
}

// ═══════ Aggregate Root: Order ═══════
#[derive(Debug, Clone)]
struct Order {
    id: OrderId,
    customer: String,
    lines: Vec<OrderLine>,
    status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Draft, Confirmed, Paid, Shipped, Delivered }

#[derive(Debug)]
enum OrderError {
    EmptyOrder,
    MaxItemsExceeded,
    InvalidTransition(String),
    ItemNotFound(String),
}

impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderError::EmptyOrder => write!(f, "Order must have at least 1 item"),
            OrderError::MaxItemsExceeded => write!(f, "Max 20 items per order"),
            OrderError::InvalidTransition(msg) => write!(f, "Invalid: {}", msg),
            OrderError::ItemNotFound(name) => write!(f, "Item not found: {}", name),
        }
    }
}

const MAX_ITEMS: usize = 20;

impl Order {
    fn new(id: u64, customer: &str) -> Self {
        Order {
            id: OrderId(id), customer: customer.into(),
            lines: vec![], status: OrderStatus::Draft,
        }
    }

    // ═══ INVARIANTS enforced at aggregate level ═══

    fn add_item(&self, product: &str, price: u32, qty: u32) -> Result<Self, OrderError> {
        if self.status != OrderStatus::Draft {
            return Err(OrderError::InvalidTransition("Can only add items to draft".into()));
        }
        if self.lines.len() >= MAX_ITEMS {
            return Err(OrderError::MaxItemsExceeded);
        }

        let mut lines = self.lines.clone();
        // Nếu product đã có → tăng quantity
        if let Some(existing) = lines.iter_mut().find(|l| l.product_name == product) {
            existing.quantity += qty;
        } else {
            lines.push(OrderLine { product_name: product.into(), unit_price: price, quantity: qty });
        }

        Ok(Order { lines, ..self.clone() })
    }

    fn remove_item(&self, product: &str) -> Result<Self, OrderError> {
        if self.status != OrderStatus::Draft {
            return Err(OrderError::InvalidTransition("Can only remove from draft".into()));
        }
        if !self.lines.iter().any(|l| l.product_name == product) {
            return Err(OrderError::ItemNotFound(product.into()));
        }

        let lines: Vec<_> = self.lines.iter()
            .filter(|l| l.product_name != product)
            .cloned()
            .collect();

        Ok(Order { lines, ..self.clone() })
    }

    fn confirm(&self) -> Result<Self, OrderError> {
        if self.lines.is_empty() { return Err(OrderError::EmptyOrder); }
        if self.status != OrderStatus::Draft {
            return Err(OrderError::InvalidTransition("Can only confirm draft".into()));
        }
        Ok(Order { status: OrderStatus::Confirmed, ..self.clone() })
    }

    fn pay(&self) -> Result<Self, OrderError> {
        if self.status != OrderStatus::Confirmed {
            return Err(OrderError::InvalidTransition("Can only pay confirmed orders".into()));
        }
        Ok(Order { status: OrderStatus::Paid, ..self.clone() })
    }

    fn ship(&self) -> Result<Self, OrderError> {
        if self.status != OrderStatus::Paid {
            return Err(OrderError::InvalidTransition("Can only ship paid orders".into()));
        }
        Ok(Order { status: OrderStatus::Shipped, ..self.clone() })
    }

    fn total(&self) -> u32 { self.lines.iter().map(|l| l.subtotal()).sum() }
    fn item_count(&self) -> usize { self.lines.len() }
}

fn main() {
    let order = Order::new(1, "Minh");

    let order = order.add_item("Coffee", 35_000, 2).unwrap();
    let order = order.add_item("Cake", 25_000, 1).unwrap();
    let order = order.add_item("Coffee", 35_000, 1).unwrap(); // merge: Coffee qty=3
    println!("Items: {}, Total: {}đ", order.item_count(), order.total());

    let order = order.confirm().unwrap();
    let order = order.pay().unwrap();
    let order = order.ship().unwrap();
    println!("Status: {:?}", order.status);

    // ❌ Invalid transitions
    println!("Confirm shipped: {:?}", order.confirm()); // Err
    println!("Add to shipped: {:?}", order.add_item("Tea", 20_000, 1)); // Err
}
```

---

## 22.4 — State Machines: Typed Transitions

### Vấn đề: Enum state nhưng methods không bị giới hạn

Ở ví dụ trước, `ship()` kiểm tra status bằng `if self.status != OrderStatus::Paid` — đó là kiểm tra **runtime**. Nghĩa là bạn *có thể* viết `order.ship()` trên Draft order — code vẫn compile, chỉ lỗi khi chạy.

Nhưng nếu bạn quên kiểm tra thì sao? Nếu team mới join project và không biết quy tắc thì sao? Sẽ tốt hơn nếu compiler **ngăn** bạn gọi `ship()` trên Draft — lỗi ngay lúc viết code, không cần đợi đến khi chạy.

Rust cho phép làm điều này với **phantom types** — một kỹ thuật dùng generic parameter không chứa data, chỉ để đánh dấu "trạng thái" tại compile time. Tưởng tượng như camera an ninh gắn tag màu vào mỗi đơn hàng: tag xanh (Draft) → chỉ cho phép thêm items. Tag vàng (Confirmed) → chỉ cho phép ship. Gọi sai method cho sai tag? Compiler báo lỗi.

### Giải pháp nâng cao: Phantom types (compile-time state)

```rust
// filename: src/main.rs
use std::marker::PhantomData;

// Marker types — zero-size, chỉ tồn tại compile-time
struct Draft;
struct Confirmed;
struct Shipped;

// Order "biết" state lúc COMPILE TIME
struct Order<State> {
    id: u64,
    customer: String,
    items: Vec<(String, u32)>,
    _state: PhantomData<State>,
}

// Methods chỉ có trên Draft
impl Order<Draft> {
    fn new(id: u64, customer: &str) -> Self {
        Order { id, customer: customer.into(), items: vec![], _state: PhantomData }
    }

    fn add_item(mut self, name: &str, price: u32) -> Self {
        self.items.push((name.into(), price));
        self
    }

    // Draft → Confirmed (TYPE CHANGES!)
    fn confirm(self) -> Result<Order<Confirmed>, String> {
        if self.items.is_empty() {
            return Err("Cannot confirm empty order".into());
        }
        Ok(Order {
            id: self.id, customer: self.customer, items: self.items,
            _state: PhantomData,
        })
    }
}

// Methods chỉ có trên Confirmed
impl Order<Confirmed> {
    // Confirmed → Shipped (TYPE CHANGES!)
    fn ship(self, tracking: &str) -> Order<Shipped> {
        println!("Shipping {} with tracking {}", self.id, tracking);
        Order {
            id: self.id, customer: self.customer, items: self.items,
            _state: PhantomData,
        }
    }
}

// Methods chỉ có trên Shipped
impl Order<Shipped> {
    fn delivery_status(&self) -> String {
        format!("Order {} is on its way!", self.id)
    }
}

// Methods cho MỌI state
impl<S> Order<S> {
    fn total(&self) -> u32 { self.items.iter().map(|(_, p)| p).sum() }
}

fn main() {
    let order = Order::<Draft>::new(1, "Minh")
        .add_item("Coffee", 35_000)
        .add_item("Cake", 25_000);

    println!("Total: {}đ", order.total());

    let confirmed = order.confirm().unwrap();
    // confirmed.add_item("Tea", 20_000);  // ❌ COMPILE ERROR!
    //                                      // add_item chỉ có trên Draft!

    let shipped = confirmed.ship("VN123");
    // shipped.confirm();  // ❌ COMPILE ERROR! confirm chỉ có trên Draft!
    println!("{}", shipped.delivery_status());
}
```

> **💡 Key insight**: Compiler **ngăn** bạn gọi `add_item()` trên Order đã confirmed, hoặc `ship()` trên Order chưa confirmed. Bugs bị bắt lúc **compile time**, không cần runtime checks!

---

## 22.5 — Tổng hợp: E-commerce Domain Model

Bây giờ hãy gộp tất cả lại: Value Objects, Entities, validation, smart constructors — tất cả trong một domain model hoàn chỉnh cho sản phẩm e-commerce. Chú ý cách mỗi đoạn code dưới đây không chỉ lưu data — nó **bảo vệ** data khỏi trạng thái không hợp lệ:

```rust
// filename: src/main.rs

// ═══════ Complete domain model ═══════

// --- Value Objects ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProductId(String);

#[derive(Debug, Clone, PartialEq)]
struct ProductName(String);
impl ProductName {
    fn new(name: &str) -> Result<Self, String> {
        let trimmed = name.trim();
        if trimmed.len() < 2 || trimmed.len() > 100 {
            Err("Product name: 2-100 chars".into())
        } else {
            Ok(ProductName(trimmed.into()))
        }
    }
    fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Price(u32);
impl Price {
    fn new(amount: u32) -> Result<Self, String> {
        if amount == 0 { Err("Price must be > 0".into()) }
        else { Ok(Price(amount)) }
    }
    fn value(&self) -> u32 { self.0 }
}

// --- Entity ---
#[derive(Debug, Clone)]
struct Product {
    id: ProductId,
    name: ProductName,
    price: Price,
    stock: u32,
}

impl Product {
    fn new(id: &str, name: &str, price: u32) -> Result<Self, Vec<String>> {
        let mut errors = vec![];
        let name = ProductName::new(name).map_err(|e| errors.push(e)).ok();
        let price = Price::new(price).map_err(|e| errors.push(e)).ok();

        if errors.is_empty() {
            Ok(Product {
                id: ProductId(id.into()),
                name: name.unwrap(),
                price: price.unwrap(),
                stock: 0,
            })
        } else {
            Err(errors)
        }
    }

    fn restock(&self, amount: u32) -> Self {
        Product { stock: self.stock + amount, ..self.clone() }
    }

    fn reserve(&self, quantity: u32) -> Result<Self, String> {
        if quantity > self.stock {
            Err(format!("Insufficient stock: have {}, need {}", self.stock, quantity))
        } else {
            Ok(Product { stock: self.stock - quantity, ..self.clone() })
        }
    }
}

fn main() {
    // Create valid product
    let coffee = Product::new("PROD-001", "Premium Coffee", 85_000).unwrap();
    let coffee = coffee.restock(100);
    println!("Product: {} — {} (stock: {})", coffee.name.value(), coffee.price.value(), coffee.stock);

    // Reserve stock
    let coffee = coffee.reserve(5).unwrap();
    println!("After reserve 5: stock={}", coffee.stock);

    // Invalid creation
    let invalid = Product::new("X", "", 0);
    println!("Invalid: {:?}", invalid);

    // Insufficient stock
    let err = coffee.reserve(999);
    println!("Over-reserve: {:?}", err);
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Value Object design

Tạo 3 Value Objects cho Banking domain: `AccountNumber` (10 digits), `PositiveAmount` (> 0), `Currency` (enum: VND, USD, EUR).

<details><summary>✅ Lời giải Bài 1</summary>

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AccountNumber(String);
impl AccountNumber {
    fn new(value: &str) -> Result<Self, String> {
        if value.len() == 10 && value.chars().all(|c| c.is_ascii_digit()) {
            Ok(AccountNumber(value.into()))
        } else { Err("Must be exactly 10 digits".into()) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct PositiveAmount(f64);
impl PositiveAmount {
    fn new(value: f64) -> Result<Self, String> {
        if value > 0.0 { Ok(PositiveAmount(value)) }
        else { Err("Must be positive".into()) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Currency { VND, USD, EUR }
```

</details>

---

**Bài 2** (10 phút): Entity with lifecycle

Tạo `Ticket` entity cho support system: `Open → InProgress → Resolved → Closed`. Implement functional state transitions với validation (e.g., can't close unresolved ticket).

<details><summary>✅ Lời giải Bài 2</summary>

```rust
#[derive(Debug, Clone, PartialEq)]
enum TicketStatus { Open, InProgress, Resolved, Closed }

#[derive(Debug, Clone)]
struct Ticket {
    id: u64,
    title: String,
    status: TicketStatus,
    assignee: Option<String>,
    resolution: Option<String>,
}

impl Ticket {
    fn new(id: u64, title: &str) -> Self {
        Ticket { id, title: title.into(), status: TicketStatus::Open, assignee: None, resolution: None }
    }

    fn assign(&self, assignee: &str) -> Result<Self, String> {
        if self.status != TicketStatus::Open { return Err("Can only assign open tickets".into()); }
        Ok(Ticket { status: TicketStatus::InProgress, assignee: Some(assignee.into()), ..self.clone() })
    }

    fn resolve(&self, resolution: &str) -> Result<Self, String> {
        if self.status != TicketStatus::InProgress { return Err("Can only resolve in-progress tickets".into()); }
        Ok(Ticket { status: TicketStatus::Resolved, resolution: Some(resolution.into()), ..self.clone() })
    }

    fn close(&self) -> Result<Self, String> {
        if self.status != TicketStatus::Resolved { return Err("Can only close resolved tickets".into()); }
        Ok(Ticket { status: TicketStatus::Closed, ..self.clone() })
    }
}
```

</details>

---

**Bài 3** (15 phút): Full Aggregate

Tạo `ShoppingCart` aggregate:
- Value Objects: `ProductId`, `Money`, `Quantity`
- Invariants: max 10 unique items, total ≤ 50,000,000đ
- Methods: `add_item`, `remove_item`, `update_quantity`, `checkout` (→ returns `Order`)

<details><summary>✅ Lời giải Bài 3</summary>

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProductId(String);

#[derive(Debug, Clone)]
struct CartItem { product_id: ProductId, name: String, price: u32, quantity: u32 }

#[derive(Debug)]
struct ShoppingCart {
    items: HashMap<ProductId, CartItem>,
}

const MAX_ITEMS: usize = 10;
const MAX_TOTAL: u32 = 50_000_000;

impl ShoppingCart {
    fn new() -> Self { ShoppingCart { items: HashMap::new() } }

    fn add_item(&self, id: &str, name: &str, price: u32, qty: u32) -> Result<Self, String> {
        let pid = ProductId(id.into());
        let mut items = self.items.clone();

        if !items.contains_key(&pid) && items.len() >= MAX_ITEMS {
            return Err(format!("Max {} unique items", MAX_ITEMS));
        }

        let entry = items.entry(pid.clone()).or_insert(CartItem {
            product_id: pid, name: name.into(), price, quantity: 0,
        });
        entry.quantity += qty;

        let cart = ShoppingCart { items };
        if cart.total() > MAX_TOTAL {
            Err(format!("Total exceeds {} đ", MAX_TOTAL))
        } else { Ok(cart) }
    }

    fn remove_item(&self, id: &str) -> Self {
        let mut items = self.items.clone();
        items.remove(&ProductId(id.into()));
        ShoppingCart { items }
    }

    fn total(&self) -> u32 { self.items.values().map(|i| i.price * i.quantity).sum() }
    fn item_count(&self) -> usize { self.items.len() }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Ai cũng tạo struct trực tiếp" | Constructor public | Private fields + `pub fn new() -> Result` |
| "Value Object mutable" | Dùng `&mut self` | `&self` → trả bản mới. Immutable by design |
| "Entity so sánh sai" | `#[derive(PartialEq)]` so sánh tất cả fields | Custom `impl PartialEq` chỉ so sánh ID |
| "Phantom type quá phức tạp" | Over-engineering | Dùng enum state cho hầu hết cases, phantom chỉ khi cần compile-time guarantees nghiêm ngặt |

---

## Tóm tắt

Chapter này dạy bạn **thiết kế nội thất** cho căn phòng domain — mỗi loại đồ nội thất có chức năng rõ ràng:

- ✅ **Value Objects** = immutable, equality by value, smart constructors, private fields. "Parse, don't validate" — một khi có `Email`, bạn biết chắc nó valid.
- ✅ **Entities** = identity (ID), lifecycle, PartialEq by ID only, functional updates. CCCD của data — dù đổi tên vẫn là cùng người.
- ✅ **Aggregates** = consistency boundary, như bưu kiện. Invariants enforced trong methods. Bên ngoài chỉ giao tiếp qua root.
- ✅ **State machines**: Enum states (runtime checks) hoặc **Phantom types** (compile-time). Camera an ninh gắn tag màu — compiler bảo vệ bạn.
- ✅ **Module encapsulation**: Private field + smart constructor = chỉ valid instances tồn tại trong hệ thống.

## Tiếp theo

Bạn đã có types — giờ cần **kết nối chúng thành quy trình**. Giống như dây chuyền sản xuất: nguyên liệu vào, sản phẩm ra, mỗi trạm tạo giá trị mới.

→ Chapter 23: **Workflows as Pipelines** — bạn sẽ compose domain operations thành type-safe pipelines: `validate → price → fulfill → notify`. Method chaining, `and_then`, và custom `pipe!` macro.
