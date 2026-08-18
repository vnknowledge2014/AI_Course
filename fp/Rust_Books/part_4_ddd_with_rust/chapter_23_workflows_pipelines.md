# Chapter 23 — Workflows as Pipelines

> **Bạn sẽ học được**:
> - **Workflow = chuỗi các hàm (chain of functions)**: `Kiểm duyệt` → `Tính giá` → `Đóng gói` → `Gửi email`
> - **Method chaining** — Gọi hàm liên hoàn tạo nên các Pipeline tuyệt đẹp
> - Sử dụng **`and_then`** — Ghép nối các hàm có khả năng trả về Lỗi (`Result`) một cách an toàn
> - **Pipeline patterns**: Xử lý rẽ nhánh, và xử lý song song (song song không đồng bộ)
> - Tự viết Macro **`pipe!`** để code Rust trông giống như Elixir/F#
> - Thực hành viết một hệ thống xử lý Đơn Hàng (Order Processing) hoàn chỉnh.
>
> **Yêu cầu trước**: Chapter 10 (Error Handling), Chapter 13 (HOF & Composition), Chapter 22 (Domain Modeling).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn sẽ ghép nối hàng chục nghiệp vụ nhỏ lẻ thành một **Quy trình type-safe (Type-safe Pipelines)**. Lỗi sẽ được bắt tự động, và code sẽ vô cùng dễ đọc.

---

## Workflows as Pipelines — Mảnh ghép cốt lõi của DDD Functional

Đây là chapter quan trọng nhất trong Part IV — Nó kết nối mọi concept trước đó (Value Objects, Entities, Ports) thành một quy trình hoàn chỉnh.

Scott Wlaschin (tác giả cuốn sách kinh điển *Domain Modeling Made Functional*) mô tả các quy trình nghiệp vụ (Business Workflows) dưới dạng các **Pipelines (Đường ống)**. Dữ liệu thô đi vào đường ống, chảy qua một chuỗi các trạm biến đổi: `validate → enrich → calculate → persist`, và cuối cùng tuôn ra Thành phẩm (Output).

Mỗi trạm trong đường ống là một **Pure function** (Hàm thuần túy). Các trạm giao tiếp với nhau qua `Result` (Thành công / Thất bại). Toàn bộ quy trình này cực kỳ dễ test, dễ tách ghép, và dễ đọc như một câu chuyện.

Trong Rust, ta sẽ hiện thực hóa đường ống này bằng `Result` chains, hàm `and_then()`, và kĩ thuật Method Chaining.

---

## 23.1 — Workflow = Chain of Functions

Khi bạn đặt một ly cà phê ở quán, có một chuỗi các bước diễn ra tuần tự:
1. Nhận đơn và thu tiền
2. Kiểm tra xem kho còn cà phê không
3. Pha chế
4. Đóng nắp
5. Giao cho khách

Mỗi bước làm **Đúng một việc** rồi truyền "ly cà phê đang làm dở" cho bước sau. Nếu có bất kỳ bước nào thất bại (ví dụ: kho hết hạt cà phê) → Dây chuyền lập tức ĐÌNH CHỈ, và báo lỗi cho khách.

Trong lập trình cũng y hệt vậy. Một Workflow là một chuỗi các Hàm, hàm sau nhận kết quả của hàm trước. 
Chúng ta dùng hàm `and_then` (Nếu thành công thì làm tiếp...) để nối chúng lại.

### Workflow đơn giản

Đầu tiên, ta viết các Trạm Xử Lý độc lập:

```rust
// filename: src/main.rs

// Trạm 1: Chuyển chữ thành số
fn parse_amount(input: &str) -> Result<u32, String> {
    input.trim().parse::<u32>()
        .map_err(|_| format!("Invalid amount: '{}'", input))
}

// Trạm 2: Kiểm duyệt số tiền
fn validate_amount(amount: u32) -> Result<u32, String> {
    if amount == 0 { Err("Amount must be > 0".into()) }
    else if amount > 10_000_000 { Err(format!("Amount {} exceeds limit", amount)) }
    else { Ok(amount) }
}

// Trạm 3: Tính thuế (Hàm này không bao giờ lỗi, nên trả về thẳng u32)
fn apply_tax(amount: u32) -> u32 {
    amount + amount * 8 / 100
}

// Trạm 4: In biên lai
fn format_receipt(amount: u32) -> String {
    format!("═══ RECEIPT ═══\n  Total: {}đ\n  Tax included\n═══════════════", amount)
}
```

Bây giờ ta nối chúng lại thành một Đường Ống (Pipeline):

```rust
fn main() {
    // Pipeline: parse → validate → tax → format
    let result = parse_amount("  500000  ")
        .and_then(validate_amount) // Dùng and_then vì validate có thể sinh Lỗi
        .map(apply_tax)            // Dùng map vì apply_tax không bao giờ sinh Lỗi
        .map(format_receipt);

    match result {
        Ok(receipt) => println!("{}", receipt),
        Err(e) => println!("❌ Lỗi: {}", e),
    }

    // Trường hợp Lỗi — Đường ống tự động ĐỨT GÃY ngay tại trạm parse_amount
    let err = parse_amount("abc")
        .and_then(validate_amount)
        .map(apply_tax);
        
    println!("\nInvalid: {:?}", err); // Sẽ in ra lỗi Parse, không bao giờ chạy đến Validate hay Tax
}
```

> **💡 Tinh túy**: `and_then` có nghĩa là: *"Nếu OK thì tui ném cho trạm tiếp theo, còn nếu tui LỖI thì tui báo Lỗi thẳng ra ngoài luôn, khỏi chạy tiếp"*. Pipeline tự động xử lý rủi ro mà không cần viết hàng chục dòng lệnh `if error { return error }`!

---

## 23.2 — Method Chaining: Builder-style Pipelines

Phần trước ta truyền dữ liệu rời rạc. Nhưng khi Workflow phức tạp lên, dữ liệu sẽ phình to thành một Struct. Lúc này, viết các Method gắn liền với Struct đó sẽ giúp code gọn gàng hơn — gọi là Builder Pattern.

Mỗi method sẽ luôn trả về chính nó (`Self`) để có thể chấm phẩy nối tiếp (chainable), hoặc trả về `Result<Self>` nếu method đó có khả năng thất bại.

### Workflow trên Domain Types

Hãy tạo một giỏ hàng phức tạp.

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
struct OrderWorkflow {
    customer: String,
    items: Vec<(String, u32, u32)>, // name, price, qty
    discount: u32,
    tax_rate: u32,
    subtotal: u32,
    total: u32,
    status: String,
    notes: Vec<String>, // Nhật ký ghi lại những gì đã xảy ra
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

    // Step 1: Thêm hàng (Trả thẳng về Self vì không bao giờ lỗi)
    fn add_item(mut self, name: &str, price: u32, qty: u32) -> Self {
        self.items.push((name.into(), price, qty));
        self.notes.push(format!("Added {} x{}", name, qty));
        self
    }

    // Step 2: Áp mã giảm giá
    fn with_discount(mut self, percent: u32) -> Self {
        self.discount = percent;
        self.notes.push(format!("Discount: {}%", percent));
        self
    }
```

Đến bước tính tiền và kiểm duyệt, có nguy cơ thất bại, nên ta sẽ trả về `Result`:

```rust
    // Step 3: Tính toán giá (Có thể lỗi nếu giỏ hàng trống)
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

    // Step 4: Duyệt đơn (Có thể lỗi nếu mua lố 100 củ)
    fn validate(self) -> Result<Self, String> {
        if self.total > 100_000_000 {
            Err(format!("Order total {} exceeds limit", self.total))
        } else {
            Ok(self)
        }
    }

    // Step 5: Chốt đơn
    fn confirm(mut self) -> Self {
        self.status = "confirmed".into();
        self.notes.push("Order confirmed ✅".into());
        self
    }
}
```

Hãy nhìn cách chúng ta "Lắp ghép" đường ống lại: Trơn tru, lưu loát, và cực kỳ dễ đọc.

```rust
fn main() {
    // Fluent pipeline (Đường ống trôi chảy)
    let result = OrderWorkflow::new("Minh")
        .add_item("Coffee", 35_000, 3)
        .add_item("Cake", 25_000, 2)
        .with_discount(10)
        .calculate()
        .and_then(|o| o.validate()) // Từ đây trở đi là Result, phải dùng and_then/map
        .map(|o| o.confirm());

    match result {
        Ok(order) => {
            println!("Thành công! Tình trạng: {}", order.status);
            println!("📝 Nhật ký xử lý:");
            for note in &order.notes { println!("  → {}", note); }
        }
        Err(e) => println!("❌ {}", e),
    }
}
```

---

## ✅ Checkpoint 23.2

> Ghi nhớ:
> 1. **Method chaining**: Các hàm kết thúc bằng việc `return self` sẽ cho phép ta viết mã nối đuôi nhau bằng dấu chấm.
> 2. **`and_then`**: Nối các hàm trả về `Result`. Nếu một khâu gãy, toàn bộ đường ống sẽ dừng lại và văng Lỗi ra ngoài.
> 3. **`.map`**: Dùng để đẩy dữ liệu qua các hàm chỉ Trả về Dữ liệu (Không bao giờ trả Lỗi).

---

## 23.3 — Composable Steps: Thiết kế dựa trên kiểu dữ liệu (Type-driven)

Bước tiếp theo của đẳng cấp: Thay vì dùng **CÙNG MỘT CÁI STRUCT** từ đầu đến cuối như ví dụ trên, ta sẽ thiết kế để mỗi Step nhận vào một **Struct A** và đẻ ra một **Struct B** khác biệt. 

`UnvalidatedOrder` đi vào → Trở thành `ValidatedOrder` → Trở thành `PricedOrder` → Cuối cùng là `ConfirmedOrder`. 
Điều này buộc Lập trình viên không bao giờ có thể nhảy cóc, không thể lỡ quên chạy hàm `validate` mà đòi đi tính giá (Vì hàm tính giá đòi đầu vào phải là struct `ValidatedOrder`). Trình biên dịch (Compiler) sẽ tát bạn nếu bạn làm sai!

### Định nghĩa các Trạm Types

```rust
// filename: src/main.rs

// ═══ Domain Types (Các Trạng thái tiến hóa của Đơn hàng) ═══

// 1. Vừa mới vào, chưa biết đúng sai
struct UnvalidatedOrder {
    customer_name: String,
    items: Vec<(String, u32)>, // name, qty
}

// 2. Đã được xác thực tên tuổi, số lượng
struct ValidatedOrder {
    customer_name: String,
    items: Vec<(String, u32)>,
}

// 3. Đã được áp giá tiền và tính tổng
struct PricedOrder {
    customer_name: String,
    total: u32,
}

// 4. Đã chốt hạ thành công
struct ConfirmedOrder {
    order_id: String,
    total: u32,
}
```

### Xây dựng các Trạm Xử lý (Pure Functions)

Chú ý rằng **Input** của hàm sau chính là **Output** của hàm trước!

```rust
// Trạm 1: Validate -> Trả về ValidatedOrder
fn validate(order: UnvalidatedOrder) -> Result<ValidatedOrder, String> {
    if order.customer_name.trim().len() < 2 { return Err("Name too short".into()); }
    if order.items.is_empty() { return Err("No items".into()); }
    
    Ok(ValidatedOrder {
        customer_name: order.customer_name.trim().into(),
        items: order.items,
    })
}

// Trạm 2: Tính giá -> Nhận Validated, trả về Priced
fn price(order: ValidatedOrder) -> Result<PricedOrder, String> {
    // Giả sử giá mặc định là 50_000đ mỗi món
    let total: u32 = order.items.iter().map(|(_, qty)| 50_000 * qty).sum();

    if total == 0 { return Err("Order total cannot be zero".into()); }

    Ok(PricedOrder {
        customer_name: order.customer_name,
        total,
    })
}

// Trạm 3: Chốt đơn -> Nhận Priced, trả về Confirmed
fn confirm(order: PricedOrder) -> ConfirmedOrder {
    ConfirmedOrder {
        order_id: format!("ORD-{}", 42),
        total: order.total,
    }
}
```

Và ráp dây chuyền lại, thấy không? Nó chạy như một bài thơ!

```rust
fn main() {
    let input = UnvalidatedOrder {
        customer_name: "  Minh Nguyen  ".into(),
        items: vec![("Coffee".into(), 3), ("Pastry".into(), 2)],
    };

    // Pipeline: Unvalidated → Validated → Priced → Confirmed
    let result = validate(input)
        .and_then(price)
        .map(confirm);

    match result {
        Ok(order) => println!("✅ Đơn hàng {} thành công. Thu: {}đ", order.order_id, order.total),
        Err(e) => println!("❌ Dây chuyền đổ vỡ: {}", e),
    }
}
```

> **💡 Key insight**: Mỗi step là một cuộc Lột xác (Transform) từ Type này sang Type khác. Nhờ vậy, Compiler sẽ thay bạn rà soát xem liệu có kịch bản nào đơn hàng bay thẳng tới Đóng gói mà chưa qua Tính tiền hay không!

---

## 23.4 — Branching & Parallel Pipelines

Thực tế không phải lúc nào Workflow cũng đi một đường thẳng đuột. Ví dụ: Khách thanh toán bằng Thẻ thì phải gọi API Stripe, thanh toán bằng Chuyển Khoản thì phải sinh mã QR, COD thì phải kiểm tra giới hạn 5 triệu. Ta gọi đó là Rẽ nhánh (Branching).

### Rẽ nhánh: Bẻ lái dựa trên Điều kiện (Branching)

```rust
// filename: src/main.rs

#[derive(Debug)]
struct Payment { amount: u32, method: String }
#[derive(Debug)]
struct Receipt { id: String, amount: u32, method: String }

fn process_card(payment: &Payment) -> Result<Receipt, String> {
    // Gọi API ngân hàng...
    Ok(Receipt { id: "CARD-123".into(), amount: payment.amount, method: "card".into() })
}

fn process_cod(payment: &Payment) -> Result<Receipt, String> {
    if payment.amount > 5_000_000 {
        Err("COD không được vượt quá 5 triệu".into())
    } else {
        Ok(Receipt { id: "COD-999".into(), amount: payment.amount, method: "cod".into() })
    }
}

// Trạm Ngã 3: Nhận nhiệm vụ Route (Phân luồng)
fn process_payment(payment: &Payment) -> Result<Receipt, String> {
    match payment.method.as_str() {
        "card" => process_card(payment),
        "cod" => process_cod(payment),
        other => Err(format!("Unknown method: {}", other)),
    }
}
```

### Xử lý Hàng loạt (Batch Processing)

Bạn có 100 email cần kiểm duyệt, bạn muốn tách ra làm 2 rổ: Rổ thành công và Rổ thất bại.

```rust
// filename: src/main.rs

#[derive(Debug)]
struct BatchResult { successes: Vec<String>, failures: Vec<String> }

fn validate_email(email: &str) -> Result<String, String> {
    if email.contains('@') { Ok(email.to_lowercase()) }
    else { Err(format!("Lỗi định dạng: {}", email)) }
}

fn process_batch(emails: &[&str]) -> BatchResult {
    // Dùng .partition() của Rust để chẻ Iterator thành 2 nhánh: Ok và Err
    let (ok, err): (Vec<_>, Vec<_>) = emails.iter()
        .map(|e| validate_email(e))
        .partition(Result::is_ok);

    BatchResult {
        successes: ok.into_iter().map(|r| r.unwrap()).collect(),
        failures: err.into_iter().map(|r| r.unwrap_err()).collect(),
    }
}
```

---

## 23.5 — Tự chế `pipe!` Macro (Dành cho Fan của Elixir/F#)

Rust không có toán tử `|>` như các ngôn ngữ Functional khác (để truyền dữ liệu từ trái sang phải mà không cần đóng ngoặc nhì nhằng). Nhưng Rust có Macro. Ta có thể chế ra một cái!

```rust
// filename: src/main.rs

/// pipe! macro — Truyền kết quả từ Hàm trước sang Hàm sau
macro_rules! pipe {
    ($value:expr) => { $value };
    ($value:expr => $func:expr) => { $func($value) };
    ($value:expr => $func:expr $(=> $rest:expr)+) => {
        pipe!($func($value) $(=> $rest)+) // Đệ quy gọi hàm
    };
}

/// pipe_result! — Y hệt ở trên, nhưng dùng and_then cho các hàm có thể Lỗi
macro_rules! pipe_result {
    ($value:expr) => { Ok($value) };
    ($value:expr => $func:expr) => { $func($value) };
    ($value:expr => $func:expr $(=> $rest:expr)+) => {
        $func($value).and_then(|v| pipe_result!(v $(=> $rest)+))
    };
}
```

Hãy thử sức mạnh của nó:

```rust
fn add_ten(x: i32) -> i32 { x + 10 }
fn double(x: i32) -> i32 { x * 2 }
fn to_string(x: i32) -> String { format!("Kết quả: {}", x) }

fn parse(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| format!("Parse error: {}", s))
}

fn main() {
    // Dữ liệu = 5, ném qua add_ten (thành 15), ném qua double (thành 30), ném qua to_string
    let result = pipe!(5 => add_ten => double => to_string);
    println!("{}", result);  // In ra: Kết quả: 30

    // Dữ liệu "42" (chuỗi), ném qua parse (thành Ok(42)), ném tiếp qua add_ten
    // Chú ý: Hàm add_ten không trả về Result, nên dùng pipe_result ở đây sẽ báo lỗi Type.
    // Thực tế bạn sẽ cần thiết kế kỹ hơn hoặc dùng .map() kết hợp.
}
```

---

## 23.6 — Ứng dụng thực tế: Order Processing Pipeline

Đây là lúc áp dụng mọi thứ: 
Từ dữ liệu người dùng nhập (`RawInput`) → `Validate` → `Apply Coupon` → `Calculate` → Đơn hàng Hoàn chỉnh (`FulfilledOrder`).

```rust
// filename: src/main.rs

#[derive(Debug)]
struct RawInput { customer: String, coupon: Option<String>, items: u32 }

#[derive(Debug)]
struct FulfilledOrder { customer: String, subtotal: u32, total: u32 }

// Trạm 1
fn validate_input(input: RawInput) -> Result<RawInput, String> {
    if input.customer.trim().len() < 2 { Err("Name required".into()) }
    else { Ok(input) }
}

// Trạm 2 (Vừa trả về Giỏ hàng, vừa trả thêm Phần trăm Giảm giá)
fn apply_coupon(input: RawInput) -> Result<(RawInput, u32), String> {
    let discount = match input.coupon.as_deref() {
        Some("SAVE10") => 10,
        Some(code) => return Err(format!("Unknown coupon: {}", code)),
        None => 0,
    };
    Ok((input, discount))
}

// Trạm 3 (Nhận dữ liệu gộp từ Trạm 2)
fn calculate_totals(input: RawInput, discount_pct: u32) -> FulfilledOrder {
    let subtotal = input.items * 50_000;
    let total = subtotal * (100 - discount_pct) / 100;

    FulfilledOrder { customer: input.customer, subtotal, total }
}

// GHÉP NỐI TOÀN BỘ ĐƯỜNG ỐNG DÂY CHUYỀN
fn process_order(input: RawInput) -> Result<FulfilledOrder, String> {
    validate_input(input)
        .and_then(apply_coupon)
        .map(|(input, discount)| calculate_totals(input, discount)) // Tách tuple ra xử lý
}

fn main() {
    let order = RawInput { customer: "Minh".into(), coupon: Some("SAVE10".into()), items: 4 };

    match process_order(order) {
        Ok(fulfilled) => println!("✅ Thành công! Phải trả: {}đ", fulfilled.total),
        Err(e) => println!("❌ {}", e),
    }
}
```

---

## 🏋️ Bài tập

**Bài 1** (15 phút): Hóa đơn (Invoice Pipeline)

Viết Workflow tạo hóa đơn. Input gồm: Tên khách, Email, Danh sách Món (Tên, Giá, SL).
1. `validate_customer`: Kiểm tra chuỗi rỗng.
2. `validate_items`: Phải có món, giá > 0.
3. `calculate_invoice`: Tính tổng và VAT 10%.
4. Dùng `and_then` và `map` để gắn chúng lại. In ra lỗi hoặc Hóa đơn.

<details><summary>✅ Gợi ý Lời giải</summary>

Dùng Type-driven design. Khai báo `InvoiceInput`, `ValidatedInvoice`, và `FinalInvoice`. Dùng `and_then` cho bước 1 và 2. Dùng `map` cho bước 3.

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| Gõ `and_then` bị báo Lỗi Type | Return type của hàm bên trong khác với `Result<T, E>` của hàm trước đó | Đảm bảo Type Lỗi (Chữ `E`) phải giống nhau, ví dụ cùng là `String`. Dùng `.map_err()` để ép kiểu lỗi nếu cần. |
| Code `and_then` nối nhau mỏi tay và quá thụt lùi (như bậc thang) | Đừng nhồi nhét Logic (Closure) vào trong and_then | Hãy tách Logic ra thành các hàm rời rạc đứng ngoài, rồi gọi tên nó: `and_then(check_price)` |
| "Ủa nếu tôi muốn báo TẤT CẢ các lỗi sai (Sai email + sai tên) cùng lúc thì sao?" | `and_then` luôn dừng lại ngay ở Lỗi đầu tiên (Fail fast) | Xin chúc mừng, câu hỏi của bạn sẽ được trả lời ở Bài tiếp theo! (Railway-Oriented Programming). |

---

## Tóm tắt

- ✅ **Workflow = Chain of Functions**: Quy trình nghiệp vụ là một Dây chuyền. Nguyên liệu đưa vào → Đi qua các Trạm → Thành phẩm.
- ✅ **`and_then`**: Phép màu kết nối các Trạm có nguy cơ gãy vỡ. Lỗi một cái là Dây chuyền Tự Dừng.
- ✅ **Method Chaining**: Dùng `.hàm()` nối tiếp nhau để làm Code sạch sẽ, dễ đọc như đọc tiểu thuyết.
- ✅ **Type-driven**: Dùng Struct/Enum làm các Chốt chặn. Phải biến hóa ra đúng loại Type mới được đi tiếp. Nhờ đó Compiler sẽ tóm cổ bạn nếu bạn phá vỡ Dây chuyền.

## Tiếp theo

Bạn đã biết cách Dừng Dây chuyền ngay khi chạm trán Lỗi đầu tiên. Nhưng đôi khi Khách Hàng nhập form sai tới 5 chỗ, và họ muốn bạn báo lỗi Cả 5 Chỗ đó cùng lúc (Thay vì báo từng lỗi một rất ức chế). Làm thế nào?

→ Chapter 24: **Railway-Oriented Programming** ⭐ — Chương mang tính Cách Mạng của Functional Programming. Bạn sẽ học cách cho phép Tàu chạy trên 2 đường ray: Đường Thành Công và Đường Báo Lỗi, đồng thời Thu gom toàn bộ rác (Lỗi) về đến ga cuối!
