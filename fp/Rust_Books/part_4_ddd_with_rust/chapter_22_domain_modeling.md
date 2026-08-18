# Chapter 22 — Domain Modeling with Rust Types

> **Bạn sẽ học được**:
> - **Value Objects** — Sử dụng Newtypes + Smart constructors để đảm bảo dữ liệu luôn đúng
> - **Entities** — Danh tính (Identity), vòng đời, và thay đổi trạng thái theo phong cách hàm (functional update)
> - **Aggregates** — Thiết lập ranh giới bảo vệ, không cho phép truy cập bừa bãi vào dữ liệu
> - **State machines** hoàn chỉnh — Bắt lỗi logic nghiệp vụ ngay từ lúc Compile bằng Phantom Types!
> - Module encapsulation — **Private constructors**, chỉ mở những API an toàn ra ngoài.
>
> **Yêu cầu trước**: Chapter 14 (Algebraic Types), Chapter 20 (DDD Intro), Chapter 21 (Architecture).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn sẽ nặn ra được những Struct/Enum sao cho **trình biên dịch tự động ngăn cản các lỗi nghiệp vụ** — Triết lý "Parse, don't validate."

---

## 22.1 — Value Objects: "Giá trị, không phải vật thể"

### Định nghĩa

Hãy tưởng tượng bạn đang cầm 2 tờ 100 nghìn đồng. Tờ nào cũng có giá trị như nhau — bạn không bao giờ nói "tờ NÀY đặc biệt hơn tờ KIA". Khi đi mua cà phê, bạn đưa tờ nào cũng được. Không ai quan tâm đến số Seri của tờ tiền.

Trong lập trình, có rất nhiều dữ liệu hoạt động y hệt như vậy: email `minh@company.com` dù lưu ở biến nào thì cũng trỏ về cùng hòm thư đó. Số tiền `500,000đ` dù nằm ở hóa đơn A hay hóa đơn B cũng mang giá trị là 500 nghìn. Chúng được định danh bởi **giá trị**, không phải bởi danh tính (ID).

Đó gọi là Value Object — Đối tượng **không có ID**. Hai Value Objects được coi là **bằng nhau** nếu mọi thuộc tính bên trong chúng bằng nhau.

Điều quan trọng nhất của Value Object là nó phải **luôn hợp lệ**. Bạn không muốn có một email thiếu ký tự `@`, hay số tiền âm. Vì thế ta dùng **Smart Constructor** — một hàm khởi tạo sẽ kiểm tra dữ liệu kĩ càng, và trả về lỗi nếu không đạt chuẩn. Một khi đã khởi tạo thành công, bạn biết chắc nó hợp lệ 100% — không cần kiểm tra lại ở bất kì đâu nữa!

### Smart Constructor Pattern

Đầu tiên là kiểu `Email`. Nó ẩn String bên trong (private field) để không ai tự ý sửa đổi được.

```rust
// filename: src/main.rs
use std::fmt;

// ═══════ Value Objects ═══════

/// Email — validated, normalized, immutable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String); // Trường này là private! Không có chữ `pub`

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
```

Tương tự, kiểu `Money` (Tiền) không được phép âm, và kiểu `Quantity` (Số lượng) phải lớn hơn hoặc bằng 1.

```rust
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

    pub fn apply_percentage(&self, percent: u32) -> Money {
        Money(self.0 * percent as u64 / 100)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}đ", self.0) }
}

/// Quantity — luôn >= 1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(u32);

impl Quantity {
    pub fn new(value: u32) -> Result<Self, String> {
        if value == 0 { Err("Quantity must be at least 1".into()) }
        else { Ok(Quantity(value)) }
    }
}
```

Sử dụng chúng trong thực tế:

```rust
fn main() {
    // Email: tự động dọn dẹp (trim + lowercase)
    let email = Email::new("  MINH@Company.COM  ").unwrap();
    println!("Email: {} (domain: {})", email, email.domain());

    // Hai biến khởi tạo khác nhau nhưng cùng giá trị -> Bằng nhau!
    let email2 = Email::new("minh@company.com").unwrap();
    println!("Same? {}", email == email2); // true!

    // Money: Toán học an toàn
    let price = Money::new(500_000);
    let tax = price.apply_percentage(8);
    let total = price.add(&tax);
    println!("Price: {}, Tax: {}, Total: {}", price, tax, total);

    // Quantity: Bắt lỗi Số lượng = 0
    println!("Qty(3): {:?}", Quantity::new(3));  // Ok
    println!("Qty(0): {:?}", Quantity::new(0));  // Err
}
```

### Triết lý "Parse, don't validate"

Nhìn lại: `Email::new()` không chỉ khởi tạo đối tượng — nó **Kiểm định (validate)** và **Chuẩn hóa (normalize)** trong cùng một bước. Email lỡ viết hoa hay thừa khoảng trắng? Không sao, Smart Constructor sẽ tự sửa. Thiếu dấu `@`? Nó lập tức trả về `Err`. 

Một khi bạn được ai đó truyền cho một biến kiểu `Email`, bạn **BIẾT CHẮC** nó hợp lệ — bạn không cần viết code kiểm tra định dạng lại ở hàm của bạn nữa. Nếu làm tốt, bạn sẽ diệt trừ toàn bộ các dòng `if !email.contains("@")` rải rác khắp mã nguồn.

### Bảng nội quy Value Object

| Quy tắc | Giải thích |
|------|-----------|
| **Immutable (Bất biến)** | Sau khi tạo, không thay đổi. Muốn "Update" thì phải tạo một bản mới |
| **Bằng nhau bởi giá trị** | `Email("a@b") == Email("a@b")` luôn trả về true |
| **Self-validating** | Luôn tự kiểm tra tính đúng đắn khi khởi tạo |
| **Không có ID** | Tờ 100k nào cũng như nhau |
| **Che giấu dữ liệu** | `struct Email(String)` — Cột String không có `pub`, không ai ở ngoài sờ vào được |

---

## 22.2 — Entities: "Vật thể có danh tính"

Ngược lại với Value Objects, có những thứ trong đời bạn quan tâm đến **danh tính** chứ không chỉ là dữ liệu bề ngoài. 
Bạn đổi tên trên Facebook, đổi Avatar, chuyển nhà đi nơi khác — nhưng bạn vẫn là BẠN. Lý do là vì thẻ Căn cước / Mã số công dân của bạn không thay đổi.

Trong lập trình, một khách hàng có mã `id = 42` dù họ có đổi email hay đổi hạng thẻ thành viên thì hệ thống vẫn nhận diện đó là cùng một người. 
Entity (Thực thể) chính là những Object có **Identity (ID)**.

Đây là điểm khác biệt cốt lõi: 
- `Email("a@b.com") == Email("a@b.com")` vì chữ giống chữ.
- Nhưng `Customer { id: 1, name: "Minh" }` hoàn toàn **BẰNG** `Customer { id: 1, name: "Minh Khác" }` vì chúng chia sẻ chung một ID!

Hãy xem cách tạo một `Customer` Entity.

```rust
// filename: src/main.rs

// ═══════ Entity ID — Value Object đóng vai trò danh tính ═══════
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomerId(u64);

impl CustomerId {
    pub fn new(id: u64) -> Self { CustomerId(id) }
}

// ═══════ Entity ═══════
#[derive(Debug, Clone)]
pub struct Customer {
    id: CustomerId,            // identity — Trường này BẤT BIẾN!
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
}
```

Vì chúng ta đang dùng Functional Programming, thay vì dùng `&mut self` để sửa trực tiếp biến, ta sẽ trả về một bản clone mới (Functional Update).

```rust
impl Customer {
    // ...

    // Trả về một Entity MỚI với email đã thay đổi
    pub fn update_email(&self, new_email: &str) -> Self {
        Customer { email: new_email.into(), ..self.clone() }
    }

    // Nghiệp vụ: Mua hàng xong thì tích điểm / lên hạng
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
}

// Định nghĩa cách so sánh hai Entity: CHỈ SO SÁNH ID!
impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
```

Hãy thử nghiệm:

```rust
fn main() {
    let customer = Customer::new(CustomerId::new(1), "Minh", "minh@co.com");
    
    // Mua sắm -> Tự động lên hạng
    let customer = customer.record_purchase(2_000_000);
    println!("After 2M: {:?}", customer.tier); // Lên Silver

    let customer = customer.record_purchase(3_500_000);
    println!("After 5.5M: {:?}", customer.tier); // Lên Gold

    // Entity Equality
    let modified = customer.update_email("new@co.com");
    
    // Dù khác Email, chúng vẫn là 1 Entity
    println!("Same entity? {}", customer == modified); // true!
}
```

---

## 22.3 — Aggregates: Consistency Boundaries

Bạn đã có Value Objects (như tờ tiền) và Entities (như con người). Nhưng trong thực tế, chúng không tồn tại rời rạc — chúng **thuộc về nhau**.

Hãy nghĩ về một chiếc Xe Hơi. Bên trong nó có vô lăng, động cơ, lốp xe. Bạn không thể tự tiện thay lốp khi xe đang chạy, hay tháo động cơ của xe này lắp vào xe kia mà không thông qua xưởng bảo trì. Chiếc Xe là một **Ranh giới (Boundary)**.
Mọi tương tác với bánh xe phải được thực hiện thông qua bảng điều khiển của chiếc Xe (đạp phanh, bẻ lái).

Trong DDD, một **Aggregate (Cụm)** là một nhóm các Entity và Value Object gắn kết chặt chẽ với nhau. Và Entity đứng đầu bảo vệ cả cụm đó gọi là **Aggregate Root**.

Chúng ta sẽ thiết kế một Cụm tên là `Order` (Đơn hàng). Bên trong nó chứa rất nhiều `OrderLine` (Sản phẩm con). 
`Order` sẽ thiết quân luật các quy tắc sau: Không quá 20 món hàng; Chỉ được thêm hàng khi đơn còn đang Draft (Bản nháp); Không được thanh toán đơn rỗng.

```rust
// filename: src/main.rs

// --- Các chi tiết bên trong ---
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

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Draft, Confirmed, Paid, Shipped, Delivered }

#[derive(Debug)]
enum OrderError {
    EmptyOrder,
    MaxItemsExceeded,
    InvalidTransition(String),
}
```

Giờ hãy dựng Aggregate Root:

```rust
// --- Aggregate Root ---
#[derive(Debug, Clone)]
struct Order {
    id: OrderId,
    customer: String,
    lines: Vec<OrderLine>,
    status: OrderStatus,
}

const MAX_ITEMS: usize = 20;

impl Order {
    fn new(id: u64, customer: &str) -> Self {
        Order {
            id: OrderId(id), customer: customer.into(),
            lines: vec![], status: OrderStatus::Draft,
        }
    }

    // Chỉ có Order mới được quyền thêm Items!
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

    // Các chuyển đổi trạng thái an toàn
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
}
```

Hãy chú ý, người dùng không thể can thiệp thẳng vào field `lines` (vì ta có thể set nó private). Họ BẮT BUỘC phải gọi `add_item()`. Bằng cách đó, hệ thống không bao giờ lọt lưới một đơn hàng vi phạm quy định (như > 20 món).

---

## 22.4 — State Machines: Typed Transitions (Kiểm duyệt bằng Compile)

Ở ví dụ trên, hàm `pay()` kiểm tra trạng thái bằng câu lệnh `if self.status != OrderStatus::Confirmed`. Đó là kiểm tra **Runtime (Lúc chạy)**.
Tức là mã nguồn vẫn biên dịch bình thường nếu lập trình viên gọi lệnh `order.pay()` trên một đơn hàng nháp. Ứng dụng phải chạy thì nó mới ném ra lỗi.

Nếu ta quên xử lý lỗi đó, ứng dụng sẽ Crash!
Sẽ tốt hơn rất nhiều nếu Trình Biên Dịch (Compiler) tự động la lên: *"Mày điên à, cái đơn hàng Draft làm sao mà Pay được?"* ngay lúc đang code!

Rust hỗ trợ kĩ thuật này qua **Phantom Types** (Kiểu dữ liệu ma quỷ). 
Tưởng tượng ta gắn Tag Đỏ vào Đơn Nháp, Tag Xanh vào Đơn đã Giao. Mỗi hàm chỉ nhận đúng màu Tag của nó.

```rust
// filename: src/main.rs
use std::marker::PhantomData;

// Các Tag (Zero-size, biến mất khi ứng dụng chạy)
struct Draft;
struct Confirmed;
struct Shipped;

// Order giờ mang theo cái Tag (State) 
struct Order<State> {
    id: u64,
    items: Vec<String>,
    _state: PhantomData<State>, // Dán tag vào đây
}

// KHỐI 1: Chỉ Đơn Nháp mới có các hàm này!
impl Order<Draft> {
    fn new(id: u64) -> Self {
        Order { id, items: vec![], _state: PhantomData }
    }

    fn add_item(mut self, name: &str) -> Self {
        self.items.push(name.into());
        self
    }

    // Phép màu: Chuyển Tag từ Draft -> Confirmed
    fn confirm(self) -> Result<Order<Confirmed>, String> {
        if self.items.is_empty() { return Err("Trống không!".into()); }
        
        Ok(Order {
            id: self.id, items: self.items,
            _state: PhantomData, // Đeo tag mới!
        })
    }
}

// KHỐI 2: Chỉ Đơn Đã Xác Nhận mới được Giao hàng!
impl Order<Confirmed> {
    // Chuyển Tag từ Confirmed -> Shipped
    fn ship(self) -> Order<Shipped> {
        Order {
            id: self.id, items: self.items,
            _state: PhantomData,
        }
    }
}
```

Giờ hãy gọi nó:

```rust
fn main() {
    let order = Order::<Draft>::new(1)
        .add_item("Coffee");

    let confirmed = order.confirm().unwrap();
    
    // Thử Mở Khóa dòng này xem -> COMPILER LỖI NGAY TỨC KHẮC!
    // confirmed.add_item("Tea"); 
    // Lỗi: `Order<Confirmed>` không có hàm `add_item`!

    let shipped = confirmed.ship();
    
    // shipped.confirm(); 
    // Lỗi: `Order<Shipped>` không thể Confirm lại!
}
```

> **💡 Tinh túy**: Lỗi nghiệp vụ (Business Rule Violations) đã trở thành **Lỗi Cú Pháp (Compile Error)**. Không cần viết Unit Test để kiểm tra luồng đi sai nữa, vì nó không thể biên dịch được!

---

## 22.5 — Tổng hợp: E-commerce Domain Model

Bây giờ hãy gộp tất cả lại: Value Objects, Entities, Validation, Smart Constructors — tất cả trong một Domain Model hoàn chỉnh cho E-commerce. Chú ý cách mỗi đoạn code dưới đây đều tự bảo vệ bản thân nó khỏi trạng thái "không hợp lệ":

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
    // Thu thập tất cả lỗi cùng lúc thay vì Fail Fast
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
```

Hãy dùng nó:

```rust
fn main() {
    // Tạo sản phẩm chuẩn
    let coffee = Product::new("PROD-001", "Premium Coffee", 85_000).unwrap();
    let coffee = coffee.restock(100);
    println!("Product: {} — {}đ (stock: {})", coffee.name.value(), coffee.price.value(), coffee.stock);

    // Mua hàng hợp lệ
    let coffee = coffee.reserve(5).unwrap();
    println!("Sau khi đặt mua 5 ly, kho còn: {}", coffee.stock);

    // Thử tạo một sản phẩm Tên luyên thuyên và Giá bậy bạ -> Gom được cả 2 lỗi
    let invalid = Product::new("X", "", 0);
    println!("Lỗi khởi tạo: {:?}", invalid);

    // Mua quá số lượng kho
    let err = coffee.reserve(999);
    println!("Lỗi mua hàng: {:?}", err);
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Value Object design

Tạo 3 Value Objects cho lĩnh vực Ngân hàng: `AccountNumber` (Yêu cầu đúng 10 số), `PositiveAmount` (> 0), `Currency` (enum: VND, USD, EUR).

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

Tạo Entity `Ticket` (Phiếu hỗ trợ) có các chu trình: `Open → InProgress → Resolved → Closed`. Dùng Functional Update để chuyển đổi trạng thái và không cho phép Đóng một phiếu khi nó chưa được Giải quyết (Resolved).

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
}

impl Ticket {
    fn new(id: u64, title: &str) -> Self {
        Ticket { id, title: title.into(), status: TicketStatus::Open, assignee: None }
    }

    fn assign(&self, assignee: &str) -> Result<Self, String> {
        if self.status != TicketStatus::Open { return Err("Can only assign open tickets".into()); }
        Ok(Ticket { status: TicketStatus::InProgress, assignee: Some(assignee.into()), ..self.clone() })
    }

    fn resolve(&self) -> Result<Self, String> {
        if self.status != TicketStatus::InProgress { return Err("Can only resolve in-progress tickets".into()); }
        Ok(Ticket { status: TicketStatus::Resolved, ..self.clone() })
    }

    fn close(&self) -> Result<Self, String> {
        if self.status != TicketStatus::Resolved { return Err("Can only close resolved tickets".into()); }
        Ok(Ticket { status: TicketStatus::Closed, ..self.clone() })
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Ai cũng tạo struct trực tiếp bằng ngoặc nhọn `{}`" | Do bạn lỡ tay cho các biến bên trong thành `pub` | Xóa chữ `pub` đi để làm Private fields. Người khác bắt buộc phải gọi hàm `pub fn new()` của bạn! |
| "Entity so sánh sai bét" | `#[derive(PartialEq)]` mặc định đi so sánh tất cả các trường dữ liệu | Viết tay `impl PartialEq` chỉ để so sánh cái ID của Entity mà thôi. |
| "Phantom type làm code rối quá" | Do bạn lạm dụng | Chỉ dùng nó khi luồng trạng thái cực kì rủi ro (đơn hàng, giao dịch tiền). Còn bình thường cứ xài Enum kiểm tra Runtime (If-else) là đủ sống rồi. |

---

---

## ✅ Checkpoint 22

1. Newtype `struct Email(String)` có chi phí runtime bao nhiêu?
2. Vì sao smart constructor phải đi kèm field private?
3. Mô hình state machine bằng enum và bằng nhiều struct riêng — chọn thế nào?

<details>
<summary>Đáp án</summary>

1. **Bằng không.** Newtype bị xoá hoàn toàn lúc biên dịch — cùng biểu diễn bộ nhớ như `String`. Bạn được an toàn kiểu mà không trả giá gì.
2. Vì field public cho phép `Email("không-phải-email".into())`, vòng qua constructor. Bất biến chỉ thật sự là bất biến khi **không tồn tại đường nào khác** để dựng giá trị.
3. Enum khi các trạng thái chia sẻ nhiều dữ liệu và bạn hay `match` trên chúng. Nhiều struct riêng (typestate) khi bạn muốn compiler **cấm** gọi sai method — `ship()` chỉ tồn tại trên `Order<Confirmed>`, nên gọi nhầm là lỗi biên dịch chứ không phải nhánh `Err`.
</details>

## Tóm tắt

Chapter này dạy bạn **Thiết kế nội thất** cho Domain — phân rõ chức năng từng món:

- ✅ **Value Objects** = Dữ liệu bất biến, so sánh bằng giá trị, luôn được xác thực nhờ Smart Constructors.
- ✅ **Entities** = Vật thể có Danh tính (ID). Sống thọ hơn, có vòng đời. Dù cập nhật Dữ liệu nhưng ID không đổi.
- ✅ **Aggregates** = Kẻ bảo vệ. Tập hợp các Entity và Value Object thành một chùm. Ngăn người ngoài không được sửa đổi lén lút bên trong.
- ✅ **State machines**: Nâng cấp từ kiểm tra State ở Runtime lên kiểm tra bằng **Trình Biên Dịch (Compile time)** nhờ vào Phantom Types.

## Tiếp theo

Bạn đã nặn ra được các Object. Bây giờ phải **Xâu chuỗi chúng thành quy trình**. Nguyên liệu đưa vào phải chạy qua các trạm trên dây chuyền để biến thành Thành Phẩm.

→ Chapter 23: **Workflows as Pipelines** — Bạn sẽ học cách dùng Method Chaining, `and_then`, và cú pháp `pipe!` để ráp lệnh theo đúng chuẩn Functional Programming.
