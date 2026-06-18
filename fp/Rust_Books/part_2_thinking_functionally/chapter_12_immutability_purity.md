# Chapter 12 — Immutability & Purity

> **Bạn sẽ học được**:
> - Tại sao immutability là **nền tảng** của FP — không chỉ là "best practice"
> - `let` vs `const` vs `static` — ba cấp độ "không đổi"
> - Frozen data structures — thiết kế types mà không ai sửa được
> - Pure functions — functions không side-effects, luôn cùng output cho cùng input
> - Tại sao purity giúp code dễ test, dễ debug, dễ parallel
>
> **Yêu cầu trước**: Part I complete (đặc biệt Chapter 5: Variables, Chapter 9: Ownership).
> **Thời gian đọc**: ~35 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Bạn chuyển từ "viết code imperative + mutable" sang "thiết kế data flows immutable + pure".

---

## Bước vào Part II: Thinking Functionally

Có một khoảnh khắc trong hành trình lập trình viên mà mọi thứ thay đổi. Không phải khi bạn học syntax mới hay framework mới — mà khi bạn **thay đổi cách tư duy** về code.

Part I dạy bạn **viết** Rust — syntax, ownership, error handling. Part II dạy bạn **tư duy** theo Functional Programming (FP). Sự khác biệt giống như học chữ vs học viết văn: biết chữ giúp bạn đọc, nhưng biết viết văn giúp bạn diễn đạt ý tưởng rõ ràng, thuyết phục, và không mơ hồ.

Part I dạy bạn **viết** Rust. Part II dạy bạn **tư duy** theo FP. Sự khác biệt:

| Imperative thinking | Functional thinking |
|---|---|
| Biến thay đổi theo thời gian | Data chảy qua transformations |
| "Sửa cái này, rồi sửa cái kia" | "Biến input thành output qua pipeline" |
| Kiểm soát **cách làm** (how) | Khai báo **muốn gì** (what) |
| Bugs: "biến bị sửa ở đâu??" | Clear: "data vào → data ra" |

Chapter này đặt nền tảng: **immutability** (data không đổi) và **purity** (functions không side-effects).

---

## 12.1 — Immutability: Tại sao quan trọng?

Nếu bạn hỏi một functional programmer "nguyên tắc quan trọng nhất là gì?", câu trả lời gần như luôn là **immutability** — data không thay đổi sau khi tạo.

Phản ứng đầu tiên của hầu hết developers: "Nếu không thể thay đổi data, làm sao viết chương trình?" Bạn vẫn "thay đổi" — nhưng bằng cách tạo **bản mới** đã transform, giữ nguyên bản gốc. Nghe tốn bộ nhớ? Đúng, nhưng đổi lại bạn được 3 điều cực kỳ giá trị:

### Ba lý do thực tế

**1. Predictability — "Đọc dòng 50, biết chắc giá trị ở dòng 5 không đổi"**

```rust
// filename: src/main.rs
fn main() {
    // Immutable: đọc ở bất kỳ dòng nào cũng biết giá trị
    let price = 35_000;
    // ... 200 dòng code ...
    // price vẫn là 35_000 — CHẮC CHẮN 100%

    // Mutable: phải trace toàn bộ code để biết giá trị hiện tại
    let mut counter = 0;
    counter += 1;
    // ... 200 dòng code, ai đó sửa counter ...
    // counter = ? — KHÔNG BIẾT nếu không đọc hết!
    println!("price={}, counter={}", price, counter);
}
```

**2. Thread safety — Immutable data tự động an toàn**

```rust
// filename: src/main.rs
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];  // immutable

    // Nhiều threads đọc cùng lúc — an toàn!
    // Không ai sửa → không data race → không cần lock
    let handles: Vec<_> = (0..3).map(|i| {
        let data = data.clone();
        thread::spawn(move || {
            let sum: i32 = data.iter().sum();
            println!("Thread {}: sum = {}", i, sum);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
```

**3. Undo/History — Giữ mọi phiên bản**

```rust
// filename: src/main.rs
fn main() {
    // Mutable: mất history
    let mut balance = 1_000_000;
    balance -= 200_000;  // mua gì đó — history biến mất
    balance += 50_000;   // nhận lương — không biết balance cũ

    // Immutable: giữ tất cả versions
    let history = vec![
        1_000_000,                    // initial
        1_000_000 - 200_000,          // purchase
        1_000_000 - 200_000 + 50_000, // salary
    ];

    println!("Current: {}", history.last().unwrap());
    println!("History: {:?}", history);
    // Undo? Dùng history[history.len() - 2]
    println!("Undo: {}", history[history.len() - 2]);
}
```

### `let` vs `const` vs `static` — Ba cấp độ

```rust
// filename: src/main.rs

// const: giá trị compile-time, inline vào mọi nơi dùng
const MAX_RETRIES: u32 = 3;
const PI: f64 = 3.14159265358979;

// static: giống const nhưng có địa chỉ bộ nhớ cố định
static APP_VERSION: &str = "1.0.0";

fn main() {
    // let: giá trị runtime, immutable trong scope
    let config_port = std::env::var("PORT")
        .unwrap_or("8080".to_string());

    println!("Max retries: {}", MAX_RETRIES);
    println!("App: {} on port {}", APP_VERSION, config_port);
}
```

| | `let` | `const` | `static` |
|---|---|---|---|
| Khi nào biết giá trị? | **Runtime** | **Compile time** | **Compile time** |
| Sống ở đâu? | Stack (local scope) | Inline (copy vào mọi nơi) | Vùng nhớ cố định |
| Shadowing? | ✅ Có | ❌ Không | ❌ Không |
| Dùng khi nào? | Variables | Mathematical constants, limits | Global strings, lookup tables |

---

## ✅ Checkpoint 12.1

> Ghi nhớ:
> 1. Immutable = predictable + thread-safe + undo-friendly
> 2. `let` = runtime immutable, `const` = compile-time inline, `static` = fixed address
> 3. Mutable code cần "trace toàn bộ" để biết giá trị — immutable thì biết ngay
>
> **Test nhanh**: Thread A đọc `data`, Thread B đọc `data` (cả hai immutable). Cần lock không?
> <details><summary>Đáp án</summary>Không! Immutable data = read-only = nhiều threads đọc cùng lúc an toàn. Chỉ cần lock khi có write.</details>

---

## 12.2 — Frozen Data Structures: Thiết kế không sửa được

Immutability bằng `let` là tốt, nhưng chưa đủ. `let` chỉ ngăn bạn gán lại biến — nếu struct có `pub` fields, code khác vẫn sửa trực tiếp được. Để đạt immutability thực sự ở mức API, bạn cần **thiết kế struct sao cho không ai sửa được** sau khi tạo.

Pattern chuẩn: private fields + Builder + read-only getters. Sau khi `build()`, struct "đông cứng" — chỉ có thể đọc, không thể sửa.

### Builder pattern → Frozen struct

Tạo struct mà sau khi "build" xong, **không ai sửa được** — không pub fields, không setters:

```rust
// filename: src/main.rs

// Struct "frozen" — mọi fields private, chỉ có getters
#[derive(Debug, Clone)]
pub struct UserProfile {
    name: String,      // private!
    email: String,     // private!
    age: u32,          // private!
}

// Builder — nơi duy nhất tạo được UserProfile
pub struct UserProfileBuilder {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
}

impl UserProfileBuilder {
    pub fn new() -> Self {
        UserProfileBuilder { name: None, email: None, age: None }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    pub fn build(self) -> Result<UserProfile, String> {
        Ok(UserProfile {
            name: self.name.ok_or("name is required")?,
            email: self.email.ok_or("email is required")?,
            age: self.age.ok_or("age is required")?,
        })
    }
}

// UserProfile chỉ có getters — KHÔNG có setters!
impl UserProfile {
    pub fn name(&self) -> &str { &self.name }
    pub fn email(&self) -> &str { &self.email }
    pub fn age(&self) -> u32 { self.age }

    // "Update" = tạo bản mới (functional update)
    pub fn with_email(&self, new_email: &str) -> Self {
        UserProfile {
            name: self.name.clone(),
            email: new_email.to_string(),
            age: self.age,
        }
    }
}

fn main() {
    let user = UserProfileBuilder::new()
        .name("Minh")
        .email("minh@email.com")
        .age(25)
        .build()
        .unwrap();

    println!("User: {} ({})", user.name(), user.email());

    // user.name = "Other".to_string();  // ❌ private — không sửa được!

    // "Update" tạo bản mới, bản cũ không đổi
    let updated = user.with_email("new@email.com");
    println!("Original: {}", user.email());   // minh@email.com
    println!("Updated: {}", updated.email()); // new@email.com
}
```

### Newtype pattern — Type-safe immutable wrappers

```rust
// filename: src/main.rs

// Newtype: wrap primitive để thêm ý nghĩa + validation
#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);  // private inner → không sửa được trực tiếp

impl Email {
    pub fn new(value: &str) -> Result<Self, String> {
        if value.contains('@') && value.contains('.') {
            Ok(Email(value.to_lowercase()))
        } else {
            Err(format!("Invalid email: {}", value))
        }
    }

    pub fn as_str(&self) -> &str { &self.0 }
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Money(u64);  // cents để tránh floating point

impl Money {
    pub fn from_dong(dong: u64) -> Self { Money(dong) }
    pub fn value(&self) -> u64 { self.0 }

    // Immutable operations — trả giá trị mới
    pub fn add(&self, other: &Money) -> Money {
        Money(self.0 + other.0)
    }

    pub fn multiply(&self, factor: u32) -> Money {
        Money(self.0 * factor as u64)
    }

    pub fn display(&self) -> String {
        format!("{}đ", self.0)
    }
}

fn main() {
    let email = Email::new("Minh@Gmail.Com").unwrap();
    println!("Email: {}, Domain: {}", email.as_str(), email.domain());
    // Email: minh@gmail.com, Domain: gmail.com

    let price = Money::from_dong(35_000);
    let quantity_price = price.multiply(3);
    let total = quantity_price.add(&Money::from_dong(5_000));
    println!("3 × {} + 5000đ = {}", price.display(), total.display());
    // 3 × 35000đ + 5000đ = 110000đ

    // price vẫn là 35000đ — không bị sửa!
    println!("Original price: {}", price.display());
}
```

> **💡 Kết nối Chapter 1**: Newtype pattern chính là cách "make illegal states unrepresentable" — `Email` **luôn valid** vì constructor validate. Không có cách tạo Email sai.

---

## 12.3 — Pure Functions: Không side-effects

Immutability nói về **data** — data không đổi. Purity nói về **functions** — functions không ảnh hưởng thế giới bên ngoài. Hai concept này bổ sung cho nhau: immutable data + pure functions = code dễ hiểu, dễ test, dễ parallel.

Tại sao purity quan trọng? Vì khi function pure, bạn biết **chính xác** nó làm gì chỉ bằng cách nhìn signature. `fn discount(price: u32, percent: u32) -> u32` — bạn biết nó nhận price và percent, trả lại giá mới. Không đọc database, không gửi email, không ghi log. Không có gì bất ngờ.

### Pure function là gì?

Function **pure** khi:
1. **Deterministic**: Cùng input → **luôn** cùng output
2. **No side-effects**: Không sửa gì bên ngoài (không mutate global, không write file, không print)

```rust
// filename: src/main.rs

// ✅ PURE: cùng input → cùng output, không side-effects
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn discount(price: u32, percent: u32) -> u32 {
    price - (price * percent / 100)
}

fn classify_age(age: u32) -> &'static str {
    match age {
        0..=12  => "Child",
        13..=17 => "Teen",
        18..=64 => "Adult",
        _       => "Senior",
    }
}

// ❌ IMPURE: side-effects
fn impure_greet(name: &str) {
    println!("Hello, {}!", name);  // side-effect: I/O
}

static mut COUNTER: u32 = 0;
fn impure_count() -> u32 {
    unsafe {
        COUNTER += 1;  // side-effect: mutate global
        COUNTER
    }
}

fn main() {
    // Pure: gọi bao nhiêu lần kết quả cũng giống nhau
    assert_eq!(add(3, 5), 8);
    assert_eq!(add(3, 5), 8);  // luôn = 8

    assert_eq!(discount(100_000, 20), 80_000);
    assert_eq!(classify_age(25), "Adult");

    println!("35000đ giảm 10% = {}đ", discount(35_000, 10));
    println!("Age 25 = {}", classify_age(25));
}
```

### Tại sao purity quan trọng?

**1. Dễ test** — không cần mock database, network, file system:

```rust
// filename: src/main.rs

// Pure function: test SIÊU ĐƠN GIẢN
fn calculate_tax(amount: u32, rate: f64) -> u32 {
    (amount as f64 * rate) as u32
}

fn apply_discount(price: u32, code: &str) -> u32 {
    match code {
        "VIP20" => price * 80 / 100,
        "SAVE10" => price * 90 / 100,
        _ => price,
    }
}

fn main() {
    println!("Tax: {}", calculate_tax(100_000, 0.08));
    println!("VIP: {}", apply_discount(100_000, "VIP20"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax() {
        assert_eq!(calculate_tax(100_000, 0.08), 8_000);
        assert_eq!(calculate_tax(0, 0.08), 0);
    }

    #[test]
    fn test_discount_vip() {
        assert_eq!(apply_discount(100_000, "VIP20"), 80_000);
    }

    #[test]
    fn test_discount_unknown_code() {
        assert_eq!(apply_discount(100_000, "FAKE"), 100_000);
    }
}
```

**2. Dễ cache (memoize)** — cùng input = cùng output → cache kết quả:

```rust
// filename: src/main.rs
use std::collections::HashMap;

// Pure function: kết quả chỉ phụ thuộc input → cache được!
fn fibonacci(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    let result = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1, cache) + fibonacci(n - 2, cache),
    };
    cache.insert(n, result);
    result
}

fn main() {
    let mut cache = HashMap::new();
    println!("fib(50) = {}", fibonacci(50, &mut cache));
    // Lần gọi thứ 2: instant — đã cache!
    println!("fib(50) = {}", fibonacci(50, &mut cache));
    println!("Cache size: {}", cache.len());
}
```

**3. Dễ parallel** — không shared mutable state → chạy song song an toàn:

```rust
// filename: src/main.rs

fn process_chunk(data: &[i32]) -> i32 {
    data.iter().filter(|&&x| x > 0).sum()
}

fn main() {
    let data = vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10];

    // Chia data thành chunks, xử lý song song
    let total: i32 = data.chunks(3)
        .map(|chunk| process_chunk(chunk))
        .sum();

    println!("Sum of positives: {}", total);  // 1+3+5+7+9 = 25
}
```

---

## ✅ Checkpoint 12.3

> Ghi nhớ:
> 1. **Pure** = cùng input → cùng output + no side-effects
> 2. Purity → dễ test (không mock), dễ cache, dễ parallel
> 3. I/O (print, file, network) = impure — đẩy ra rìa chương trình
>
> **Test nhanh**: Function `fn now() -> SystemTime { SystemTime::now() }` có pure không?
> <details><summary>Đáp án</summary>Không! Mỗi lần gọi trả giá trị khác nhau → non-deterministic = impure.</details>

---

## 12.4 — Functional Core, Imperative Shell

### Pattern quan trọng nhất trong kiến trúc FP

Đây là pattern bạn sẽ áp dụng trong **mọi** dự án Rust nghiêm túc. Gary Bernhardt gọi nó là "Functional Core, Imperative Shell" — và nó giải quyết câu hỏi lớn nhất khi áp dụng FP: "nếu pure functions không có side-effects, làm sao chương trình đọc file, kết nối database, gửi HTTP request?"

Câu trả lời: **tách rõ ràng**. Không thể 100% pure — chương trình phải đọc input, ghi output, kết nối database. Giải pháp: **tách rõ ràng**:

- **Core** (bên trong) = pure functions, business logic, transformations
- **Shell** (bên ngoài) = I/O, side-effects, `main()`, database

```
┌─────────────────────────┐
│         Shell            │  ← I/O, side-effects
│  ┌───────────────────┐  │
│  │      Core          │  │  ← Pure functions  
│  │  (business logic)  │  │  ← Easy to test
│  │  (transformations) │  │  ← Deterministic
│  └───────────────────┘  │
│    read file, DB, API    │  ← Hard to test
└─────────────────────────┘
```

```rust
// filename: src/main.rs

// ═══════════════════════════════════════════
// CORE: Pure functions — dễ test, không I/O
// ═══════════════════════════════════════════

#[derive(Debug, Clone)]
struct Order {
    items: Vec<(String, u32)>,
}

// Pure: chỉ tính toán
fn calculate_subtotal(order: &Order) -> u32 {
    order.items.iter().map(|(_, price)| price).sum()
}

// Pure: chỉ tính toán
fn apply_tax(subtotal: u32, rate: f64) -> u32 {
    subtotal + (subtotal as f64 * rate) as u32
}

// Pure: chỉ tạo string
fn format_order_receipt(order: &Order, total: u32) -> String {
    let mut lines = vec!["🧾 Receipt".to_string()];
    for (name, price) in &order.items {
        lines.push(format!("  {} — {}đ", name, price));
    }
    lines.push(format!("  ─────────"));
    lines.push(format!("  Total: {}đ", total));
    lines.join("\n")
}

// ═══════════════════════════════════════════
// SHELL: I/O, side-effects — main()
// ═══════════════════════════════════════════

fn main() {
    // Shell: tạo data (trong production: đọc từ DB/API)
    let order = Order {
        items: vec![
            ("Coffee".into(), 35_000),
            ("Cake".into(), 25_000),
            ("Juice".into(), 30_000),
        ],
    };

    // Core: pure computations
    let subtotal = calculate_subtotal(&order);
    let total = apply_tax(subtotal, 0.08);
    let receipt = format_order_receipt(&order, total);

    // Shell: I/O output
    println!("{}", receipt);
}

// Tests: chỉ test CORE — không cần mock I/O!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtotal() {
        let order = Order {
            items: vec![("A".into(), 10_000), ("B".into(), 20_000)],
        };
        assert_eq!(calculate_subtotal(&order), 30_000);
    }

    #[test]
    fn test_tax() {
        assert_eq!(apply_tax(100_000, 0.1), 110_000);
    }

    #[test]
    fn test_receipt_format() {
        let order = Order { items: vec![("Tea".into(), 25_000)] };
        let receipt = format_order_receipt(&order, 27_000);
        assert!(receipt.contains("Tea"));
        assert!(receipt.contains("27000đ"));
    }
}
```

---

## 12.5 — Immutable Patterns trong Rust

Đến đây bạn đã hiểu **tại sao** immutability và purity quan trọng. Nhưng "hiểu lý thuyết" khác xa "viết code thực tế". Phần này cho bạn **3 patterns cụ thể** để thay thế imperative habits bằng functional alternatives — bạn sẽ dùng chúng hàng ngày.

Phiên bản thực chiến của immutability: thay vì sửa dữ liệu gốc, tạo bản mới đã transform. Iterator chains là vũ khí mạnh nhất — mỗi `.map()`, `.filter()` tạo dữ liệu mới, không ai chạm vào bản gốc.

### Pattern 1: Transform thay vì Mutate

```rust
// filename: src/main.rs
fn main() {
    let prices = vec![35_000, 25_000, 45_000, 40_000];

    // ❌ Imperative: mutate in-place
    // let mut discounted = prices.clone();
    // for p in &mut discounted { *p = *p * 90 / 100; }

    // ✅ Functional: transform → new collection
    let discounted: Vec<u32> = prices.iter()
        .map(|&p| p * 90 / 100)  // 10% off
        .collect();

    println!("Original: {:?}", prices);       // không đổi
    println!("Discounted: {:?}", discounted);  // mới
}
```

### Pattern 2: Fold thay vì Accumulator

```rust
// filename: src/main.rs
fn main() {
    let transactions = vec![100, -50, 200, -30, 150];

    // ❌ Mutable accumulator
    // let mut balance = 0;
    // for &t in &transactions { balance += t; }

    // ✅ Fold — no mutation
    let balance: i32 = transactions.iter().sum();
    println!("Balance: {}", balance);  // 370

    // Running balance (scan)
    let running: Vec<i32> = transactions.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("Running: {:?}", running);  // [100, 50, 250, 220, 370]
}
```

### Pattern 3: Enum states thay vì boolean flags

```rust
// filename: src/main.rs

// ❌ Mutable flags — khó track, dễ quên
// let mut is_paid = false;
// let mut is_shipped = false;
// let mut is_cancelled = false;

// ✅ Enum — mỗi state rõ ràng, immutable transition
#[derive(Debug, Clone)]
enum OrderStatus {
    Pending,
    Paid { amount: u32 },
    Shipped { tracking: String },
    Delivered,
    Cancelled { reason: String },
}

// Pure function: transition trả state MỚI
fn process_payment(status: &OrderStatus, amount: u32) -> Result<OrderStatus, String> {
    match status {
        OrderStatus::Pending => Ok(OrderStatus::Paid { amount }),
        _ => Err(format!("Cannot pay order in {:?} state", status)),
    }
}

fn ship_order(status: &OrderStatus, tracking: &str) -> Result<OrderStatus, String> {
    match status {
        OrderStatus::Paid { .. } => Ok(OrderStatus::Shipped {
            tracking: tracking.to_string(),
        }),
        _ => Err(format!("Cannot ship order in {:?} state", status)),
    }
}

fn main() {
    let status = OrderStatus::Pending;
    println!("1. {:?}", status);

    let status = process_payment(&status, 50_000).unwrap();
    println!("2. {:?}", status);

    let status = ship_order(&status, "VN123456").unwrap();
    println!("3. {:?}", status);

    // Thử ship lại → error (đúng!)
    let error = ship_order(&status, "XX").unwrap_err();
    println!("4. Error: {}", error);

    // Output:
    // 1. Pending
    // 2. Paid { amount: 50000 }
    // 3. Shipped { tracking: "VN123456" }
    // 4. Error: Cannot ship order in Shipped { tracking: "VN123456" } state
}
```

---

## 12.6 — Khi nào KHÔNG cần Immutability / FP?

### Balanced view — Mut không phải tội lỗi

Một sai lầm phổ biến khi học FP: coi `mut` là "tội lỗi" và cố tránh bằng mọi giá. Thực tế, **local mutation** — mutation bên trong function, không leak ra ngoài — hoàn toàn OK và thường nhanh hơn.

FP patterns rất mạnh, nhưng Rust **không phải ngôn ngữ FP thuần** — nó là multi-paradigm. Có lúc `mut` là lựa chọn đúng:

**1. Performance-critical hot paths**

```rust
// Clone mỗi lần = O(n) copy. Với 1 triệu items, rất tốn:
// ❌ Functional (expensive):
// let new_vec: Vec<i32> = old_vec.iter().map(|x| x + 1).collect();  // allocates new Vec

// ✅ Mutable (fast, zero allocation):
// for x in &mut vec { *x += 1; }

// Quy tắc: Profile TRƯỚC. Chỉ tối ưu khi profiler chỉ ra bottleneck.
```

**2. Game loops, audio, real-time**

```rust
// Game state thay đổi 60 lần/giây — clone mỗi frame quá tốn
// struct GameState { player: Player, enemies: Vec<Enemy>, ... }

// Trong game: mutable state tự nhiên hơn
// fn update(state: &mut GameState, dt: f32) { ... }
```

**3. Prototyping — ship first, refactor later**

```rust
// Khi khám phá ý tưởng, `mut` + `unwrap()` = nhanh nhất:
fn prototype() {
    let mut data = vec![];
    data.push(get_input().unwrap());  // unwrap OK khi prototyping
    data.sort();
    println!("{:?}", data);
}
// Refactor sang pure functions + Result SAU khi logic đã rõ.
```

**4. Local mutation = hoàn toàn OK**

```rust
// Mutation BÊN TRONG function, không leak ra ngoài = pure!
fn sort_and_dedup(items: &[i32]) -> Vec<i32> {
    let mut result = items.to_vec();  // mut nhưng LOCAL
    result.sort();
    result.dedup();
    result  // trả immutable — caller không biết bên trong có mut
}
```

### Bảng quyết định: FP hay Imperative?

| Tình huống | FP (immutable) | Imperative (mutable) |
|-----------|----------------|---------------------|
| Business logic, domain rules | ✅ Strongly prefer | |
| Data transformations, pipelines | ✅ Strongly prefer | |
| Shared state giữa threads | ✅ Prefer | |
| Hot loop (1M+ iterations) | | ✅ Prefer (profile first) |
| Game/audio real-time | | ✅ Prefer |
| Quick prototype | | ✅ OK (refactor later) |
| Local variables trong function | | ✅ Fine — chưa share state |

> **💡 Rust's stance**: "Functional Core, Imperative Shell" (12.4) — pure business logic ở core, mutation/IO ở edges. `let mut` bên trong function = OK. `pub mut` shared globally = code smell.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Pure hay Impure?

```rust
fn a(x: i32) -> i32 { x * 2 }
fn b(items: &mut Vec<i32>) { items.push(1); }
fn c(name: &str) -> String { format!("Hi, {}", name) }
fn d() -> u64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() }
fn e(prices: &[u32]) -> u32 { prices.iter().sum() }
```

<details><summary>✅ Lời giải Bài 1</summary>

```
a: ✅ Pure — cùng input → cùng output, no side-effects
b: ❌ Impure — side-effect: mutates input
c: ✅ Pure — format chỉ tạo String mới, deterministic
d: ❌ Impure — non-deterministic (trả giá trị khác mỗi lần)
e: ✅ Pure — chỉ đọc input, trả kết quả mới
```

</details>

---

**Bài 2** (10 phút): Refactor imperative → functional

Chuyển code mutable sau thành pure functions:

```rust
let mut items = vec!["apple", "banana", "cherry"];
items.push("date");
items.sort();
items.retain(|i| i.len() > 5);
let result = items.join(", ");
println!("{}", result);
```

<details><summary>✅ Lời giải Bài 2</summary>

```rust
fn main() {
    let items = vec!["apple", "banana", "cherry"];

    // Pipeline: transform mà không mutate
    let mut extended = items.clone();
    extended.push("date");
    extended.sort();

    let result: String = extended.iter()
        .filter(|i| i.len() > 5)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");

    println!("{}", result);            // banana, cherry
    println!("Original: {:?}", items); // vẫn nguyên
}
```

</details>

---

**Bài 3** (15 phút): Bank account — Functional Core

Viết module `bank` với:
- Frozen struct `Account { id, name, balance }` (private fields, chỉ getters)
- Pure functions: `deposit(&Account, amount) -> Account`, `withdraw(&Account, amount) -> Result<Account, String>`, `transfer(from, to, amount) -> Result<(Account, Account), String>`
- Test tất cả mà **không cần mock gì**.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
struct Account {
    id: u32,
    name: String,
    balance: u64,
}

impl Account {
    fn new(id: u32, name: &str, balance: u64) -> Self {
        Account { id, name: name.to_string(), balance }
    }
    fn id(&self) -> u32 { self.id }
    fn name(&self) -> &str { &self.name }
    fn balance(&self) -> u64 { self.balance }
}

// Pure functions — trả Account MỚI
fn deposit(account: &Account, amount: u64) -> Account {
    Account { balance: account.balance + amount, ..account.clone() }
}

fn withdraw(account: &Account, amount: u64) -> Result<Account, String> {
    if amount > account.balance {
        Err(format!("Insufficient funds: have {}, need {}", account.balance, amount))
    } else {
        Ok(Account { balance: account.balance - amount, ..account.clone() })
    }
}

fn transfer(from: &Account, to: &Account, amount: u64) -> Result<(Account, Account), String> {
    let new_from = withdraw(from, amount)?;
    let new_to = deposit(to, amount);
    Ok((new_from, new_to))
}

fn main() {
    let alice = Account::new(1, "Alice", 500_000);
    let bob = Account::new(2, "Bob", 200_000);

    match transfer(&alice, &bob, 150_000) {
        Ok((new_alice, new_bob)) => {
            println!("{}: {}đ → {}đ", new_alice.name(), 500_000, new_alice.balance());
            println!("{}: {}đ → {}đ", new_bob.name(), 200_000, new_bob.balance());
        }
        Err(e) => println!("Error: {}", e),
    }
    // Alice/Bob ORIGINAL vẫn nguyên!
    println!("Original Alice: {}đ", alice.balance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let acc = Account::new(1, "Test", 100);
        let acc = deposit(&acc, 50);
        assert_eq!(acc.balance(), 150);
    }

    #[test]
    fn test_withdraw_ok() {
        let acc = Account::new(1, "Test", 100);
        let acc = withdraw(&acc, 30).unwrap();
        assert_eq!(acc.balance(), 70);
    }

    #[test]
    fn test_withdraw_insufficient() {
        let acc = Account::new(1, "Test", 100);
        assert!(withdraw(&acc, 200).is_err());
    }

    #[test]
    fn test_transfer() {
        let a = Account::new(1, "A", 500);
        let b = Account::new(2, "B", 200);
        let (new_a, new_b) = transfer(&a, &b, 150).unwrap();
        assert_eq!(new_a.balance(), 350);
        assert_eq!(new_b.balance(), 350);
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| "Cần sửa biến nhưng immutable" | Muốn update in-place | Dùng functional update: tạo bản mới `{ field: new_val, ..old }` |
| Pure function cần I/O | Design issue | Tách: core (pure) + shell (I/O) |
| "Clone quá nhiều, chậm?" | Worry về performance | Đúng cho hot paths. Phần lớn code: clarity > speed. Profile trước khi optimize |
| Struct có quá nhiều fields để clone | Tedious `..old.clone()` | Derive `Clone`, dùng builder hoặc `with_*` methods |

---

## Tóm tắt

- ✅ **Immutability** = predictable + thread-safe + undo-friendly. Rust mặc định `let` immutable.
- ✅ **Frozen structs** = private fields + builder + getters + `with_*` methods. Newtype cho validation.
- ✅ **Pure functions** = cùng input → cùng output + no side-effects. Dễ test, cache, parallel.
- ✅ **Functional Core, Imperative Shell** = pattern quan trọng nhất. Core = pure logic. Shell = I/O.
- ✅ **Patterns**: transform > mutate, fold > accumulator, enum states > boolean flags.

## Tiếp theo

→ Chapter 13: **Higher-Order Functions & Composition** — bạn sẽ đi sâu vào function composition, pipeline design, và cách xây dựng data transformations phức tạp từ các building blocks đơn giản.
