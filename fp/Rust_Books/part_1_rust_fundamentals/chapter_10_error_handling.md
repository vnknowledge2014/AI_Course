# Chapter 10 — Error Handling

> **Bạn sẽ học được**:
> - `Result<T, E>` và `Option<T>` sâu hơn — combinators, chaining
> - `?` operator — cách Rust làm error propagation ngắn gọn
> - Custom error types — thiết kế errors cho domain
> - `unwrap()`, `expect()` — khi nào dùng, khi nào TRÁNH
> - Nền tảng cho **Railway-Oriented Programming** (ROP) ở Part IV
>
> **Yêu cầu trước**: Chapter 6 (match), Chapter 9 (Ownership).
> **Thời gian đọc**: ~40 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn xử lý errors chuyên nghiệp — không panic, không unwrap bừa, luôn rõ ràng.

---

## Tại sao error handling quan trọng?

Trong nhiều ngôn ngữ, errors "bay" qua exceptions — bạn không biết function nào ném, function nào bắt. Giống ai đó ném quả bom trong phòng và hy vọng có người bắt được.

Rust chọn cách khác: **mọi error đều explicit trong type**. Nếu function có thể fail, type nói rõ. Compiler buộc bạn xử lý. Không bất ngờ, không "quả bom" ẩn.

---

## Error Handling — Không có NULL, không có Exceptions

Rust từ chối hai cách xử lý lỗi phổ biến nhất: null (Tony Hoare gọi nó là "billion dollar mistake") và exceptions (control flow ẩn, khó trace). Thay vào đó, Rust dùng **types** để biểu diễn lỗi: `Option<T>` cho "có hoặc không", `Result<T, E>` cho "thành công hoặc thất bại".

Điều này nghĩa là: mọi lỗi đều **hiển thị trong type signature**. Khi bạn thấy `fn parse(s: &str) -> Result<i32, ParseError>`, bạn biết ngay function này có thể thất bại và phải xử lý case đó. Không có "surprise exceptions" bay ra từ 5 layers sâu.

Compiler bắt buộc bạn xử lý `Result` — nếu quên, code không compile. Đây là paradigm shift: từ "hy vọng không có lỗi" sang "compiler đảm bảo mọi lỗi được xử lý".

---

## 10.1 — `Option<T>`: Có hoặc không

### Rust không có `null`

Trong Java, C#, JavaScript — `null` gây ra hàng tỷ đô bugs (Tony Hoare gọi đó là "billion-dollar mistake"). Rust thay thế bằng `Option<T>`:

```rust
// filename: src/main.rs
fn main() {
    // Option<T> = Some(T) hoặc None
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // KHÔNG thể dùng trực tiếp — phải unwrap
    // let x: i32 = some_number;  // ❌ mismatched types
    // let y: i32 = some_number + 1;  // ❌ cannot add

    // Phải xử lý CẢ HAI trường hợp
    match some_number {
        Some(n) => println!("Got: {}", n),
        None => println!("Nothing!"),
    }

    match no_number {
        Some(n) => println!("Got: {}", n),
        None => println!("Nothing!"),
    }

    // Output:
    // Got: 42
    // Nothing!
}
```

### Option methods — không cần `match` mọi lúc

```rust
// filename: src/main.rs
fn main() {
    let name: Option<&str> = Some("Rust");
    let empty: Option<&str> = None;

    // unwrap_or — giá trị mặc định nếu None
    println!("{}", name.unwrap_or("Unknown"));   // Rust
    println!("{}", empty.unwrap_or("Unknown"));  // Unknown

    // map — biến đổi giá trị bên trong Some
    let upper = name.map(|n| n.to_uppercase());
    println!("{:?}", upper);  // Some("RUST")
    let upper_empty = empty.map(|n| n.to_uppercase());
    println!("{:?}", upper_empty);  // None — map bỏ qua None

    // and_then (flatMap) — chain operations trả Option
    let parsed: Option<i32> = Some("42").and_then(|s| s.parse().ok());
    println!("Parsed: {:?}", parsed);  // Some(42)
    let failed: Option<i32> = Some("abc").and_then(|s| s.parse().ok());
    println!("Failed: {:?}", failed);  // None

    // filter — giữ Some nếu thỏa điều kiện
    let big = Some(100).filter(|&n| n > 50);
    let small = Some(10).filter(|&n| n > 50);
    println!("big={:?}, small={:?}", big, small);  // Some(100), None

    // is_some / is_none
    println!("name.is_some()={}, empty.is_none()={}", name.is_some(), empty.is_none());
}
```

### Bảng quick reference — Option methods

| Method | Input | Output | Mô tả |
|--------|-------|--------|--------|
| `unwrap_or(default)` | `Option<T>` | `T` | Some → giá trị, None → default |
| `unwrap_or_else(f)` | `Option<T>` | `T` | None → gọi f() lazy |
| `map(f)` | `Option<T>` | `Option<U>` | Some(x) → Some(f(x)), None → None |
| `and_then(f)` | `Option<T>` | `Option<U>` | Some(x) → f(x), None → None (flatMap) |
| `filter(pred)` | `Option<T>` | `Option<T>` | Giữ Some nếu pred(x) true |
| `or(other)` | `Option<T>` | `Option<T>` | self nếu Some, else other |
| `zip(other)` | `Option<T>` | `Option<(T,U)>` | Cả hai Some → Some((t,u)) |

---

## ✅ Checkpoint 10.1

> Ghi nhớ:
> 1. Rust không có `null` — dùng `Option<T>` = `Some(T)` hoặc `None`
> 2. `map` biến đổi giá trị bên trong. `and_then` chain thêm Option
> 3. `unwrap_or` cho default value — tránh `unwrap()` trong production
>
> **Test nhanh**: `Some(5).map(|x| x * 2)` trả gì?
> <details><summary>Đáp án</summary><code>Some(10)</code> — map áp dụng closure lên giá trị bên trong Some.</details>

---

## 10.2 — `Result<T, E>`: Thành công hoặc lỗi

### Result = Option nhưng lỗi có thông tin

```rust
// filename: src/main.rs
use std::num::ParseIntError;

fn parse_age(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Input is empty".to_string());
    }

    match trimmed.parse::<u32>() {
        Ok(age) if age <= 150 => Ok(age),
        Ok(age) => Err(format!("Age {} is unrealistic", age)),
        Err(e) => Err(format!("Not a number: {}", e)),
    }
}

fn main() {
    let inputs = vec!["25", "abc", "200", "", "  42  "];

    for input in inputs {
        match parse_age(input) {
            Ok(age) => println!("✅ '{}' → age {}", input, age),
            Err(e) => println!("❌ '{}' → {}", input, e),
        }
    }

    // Output:
    // ✅ '25' → age 25
    // ❌ 'abc' → Not a number: invalid digit found in string
    // ❌ '200' → Age 200 is unrealistic
    // ❌ '' → Input is empty
    // ✅ '  42  ' → age 42
}
```

### Result methods — tương tự Option

```rust
// filename: src/main.rs
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // map — biến đổi Ok value
    let result = divide(10.0, 3.0).map(|v| format!("{:.2}", v));
    println!("{:?}", result);  // Ok("3.33")

    // unwrap_or — default cho Err
    let safe = divide(10.0, 0.0).unwrap_or(0.0);
    println!("Safe: {}", safe);  // 0.0

    // and_then — chain Result-returning operations
    let chained = divide(100.0, 5.0)
        .and_then(|r| divide(r, 2.0))  // 100/5 = 20, rồi 20/2 = 10
        .map(|v| format!("Final: {:.1}", v));
    println!("{:?}", chained);  // Ok("Final: 10.0")

    // Chain thất bại — dừng ở lỗi đầu tiên
    let failed = divide(100.0, 0.0)
        .and_then(|r| divide(r, 2.0))  // KHÔNG chạy
        .map(|v| format!("Final: {:.1}", v));  // KHÔNG chạy
    println!("{:?}", failed);  // Err("Division by zero")
}
```

> **💡 Đây là nền tảng Railway-Oriented Programming**: `and_then` nối các bước thành pipeline. Ok → tiếp tục. Err → nhảy thẳng ra cuối. Sẽ học chi tiết ở Part IV.

---

## ✅ Checkpoint 10.2

> Ghi nhớ:
> 1. `Result<T, E>` = `Ok(T)` hoặc `Err(E)` — error có thông tin
> 2. `map`: biến đổi Ok. `and_then`: chain thêm Result (flatMap)
> 3. **Chain dừng tại lỗi đầu tiên** — "railway" pattern tự nhiên
>
> **Test nhanh**: `Err("oops").map(|x: i32| x + 1)` trả gì?
> <details><summary>Đáp án</summary><code>Err("oops")</code> — map bỏ qua Err, chỉ chạy trên Ok.</details>

---

## 10.3 — `?` Operator: Error propagation ngắn gọn

### Vấn đề: match lồng nhau quá dài

```rust
// ❌ Verbose: match lồng match
fn read_username_verbose(path: &str) -> Result<String, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Read error: {}", e)),
    };
    let first_line = match content.lines().next() {
        Some(line) => line,
        None => return Err("File is empty".to_string()),
    };
    Ok(first_line.trim().to_string())
}
```

### Giải pháp: `?` operator

`?` là shortcut: nếu `Ok` → lấy giá trị. Nếu `Err` → **return Err ngay**.

```rust
// filename: src/main.rs
use std::fs;
use std::io;

fn read_username(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;  // Err? → return Err
    // ↑ Nếu Ok → content = nội dung file
    // ↑ Nếu Err → RETURN Err(io::Error) ngay lập tức!

    Ok(content.lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string())
}

fn main() {
    match read_username("test_user.txt") {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error: {}", e),
    }
    // Output (nếu file không tồn tại):
    // Error: No such file or directory (os error 2)
}
```

### Chain nhiều `?`

```rust
// filename: src/main.rs
use std::collections::HashMap;

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

fn parse_config(input: &str) -> Result<Config, String> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid line: '{}'", line));
        }
        map.insert(parts[0].trim(), parts[1].trim());
    }

    let host = map.get("host")
        .ok_or("Missing 'host'")?   // Option → Result via ok_or
        .to_string();

    let port_str = map.get("port")
        .ok_or("Missing 'port'")?;

    let port: u16 = port_str.parse()
        .map_err(|e| format!("Invalid port: {}", e))?;  // đổi error type

    Ok(Config { host, port })
}

fn main() {
    let valid = "host = localhost\nport = 8080";
    println!("{:?}", parse_config(valid));
    // Ok(Config { host: "localhost", port: 8080 })

    let missing = "host = localhost";
    println!("{:?}", parse_config(missing));
    // Err("Missing 'port'")

    let bad_port = "host = localhost\nport = abc";
    println!("{:?}", parse_config(bad_port));
    // Err("Invalid port: invalid digit found in string")
}
```

### `?` cho Option (Rust 1.22+)

```rust
// filename: src/main.rs

fn first_even(numbers: &[i32]) -> Option<i32> {
    let first = numbers.first()?;  // None → return None
    if first % 2 == 0 {
        Some(*first)
    } else {
        None
    }
}

fn main() {
    println!("{:?}", first_even(&[4, 1, 3]));   // Some(4)
    println!("{:?}", first_even(&[3, 1, 4]));   // None (3 is odd)
    println!("{:?}", first_even(&[]));           // None (empty)
}
```

---

## 10.4 — `unwrap()` và `expect()`: Khi nào dùng?

### Nguy hiểm: `unwrap()` panic nếu Err/None

```rust
// filename: src/main.rs
fn main() {
    // ✅ OK khi bạn CHẮC CHẮN sẽ Ok/Some
    let x: i32 = "42".parse().unwrap();  // luôn parse được
    println!("x = {}", x);

    // ❌ NGUY HIỂM — panic nếu file không tồn tại
    // let content = std::fs::read_to_string("maybe.txt").unwrap();
    // → panic: called `Result::unwrap()` on an `Err` value

    // ✅ expect — giống unwrap nhưng có message giải thích
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be a valid number");
    println!("port = {}", port);
}
```

### Quy tắc

| Dùng | Khi nào | Ví dụ |
|------|---------|-------|
| `unwrap()` | Tests, prototypes, **chắc chắn 100%** | `"42".parse::<i32>().unwrap()` |
| `expect("msg")` | Tốt hơn unwrap — có context | `.expect("config file must exist")` |
| `unwrap_or(default)` | Có giá trị fallback hợp lý | `.unwrap_or(0)` |
| `?` | Production code, propagate lên caller | `file.read()?` |
| `match` / `if let` | Cần xử lý error tại chỗ | UI error messages |

> **💡 Quy tắc vàng**: Trong production code, **KHÔNG dùng `unwrap()`**. Dùng `?` hoặc `unwrap_or`. `unwrap()` = "tôi không quan tâm error" = bug potential.

---

## 10.5 — Custom Error Types

### Dùng `enum` cho domain errors

```rust
// filename: src/main.rs
use std::fmt;

#[derive(Debug)]
enum OrderError {
    EmptyCart,
    InvalidQuantity(u32),
    OutOfStock { item: String, available: u32 },
    PaymentFailed(String),
}

// Implement Display cho user-friendly messages
impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderError::EmptyCart =>
                write!(f, "Cart is empty"),
            OrderError::InvalidQuantity(q) =>
                write!(f, "Invalid quantity: {}", q),
            OrderError::OutOfStock { item, available } =>
                write!(f, "'{}' out of stock (only {} left)", item, available),
            OrderError::PaymentFailed(reason) =>
                write!(f, "Payment failed: {}", reason),
        }
    }
}

// Implement std::error::Error (cho compatibility)
impl std::error::Error for OrderError {}

fn place_order(item: &str, quantity: u32) -> Result<String, OrderError> {
    if item.is_empty() {
        return Err(OrderError::EmptyCart);
    }
    if quantity == 0 || quantity > 100 {
        return Err(OrderError::InvalidQuantity(quantity));
    }

    // Giả lập kiểm tra stock
    let stock = 5;
    if quantity > stock {
        return Err(OrderError::OutOfStock {
            item: item.to_string(),
            available: stock,
        });
    }

    Ok(format!("✅ Ordered {}x {}", quantity, item))
}

fn main() {
    let orders = vec![
        ("Coffee", 2),
        ("", 1),
        ("Tea", 0),
        ("Smoothie", 10),  // only 5 in stock
    ];

    for (item, qty) in orders {
        match place_order(item, qty) {
            Ok(msg) => println!("{}", msg),
            Err(e) => println!("❌ {}", e),
        }
    }

    // Output:
    // ✅ Ordered 2x Coffee
    // ❌ Cart is empty
    // ❌ Invalid quantity: 0
    // ❌ 'Smoothie' out of stock (only 5 left)
}
```

### `From` trait — Convert giữa error types

```rust
// filename: src/main.rs
use std::io;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// From conversions — cho phép ? tự động convert
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn read_port_from_file(path: &str) -> Result<u16, AppError> {
    let content = std::fs::read_to_string(path)?;  // io::Error → AppError::Io
    let port: u16 = content.trim().parse()?;         // ParseIntError → AppError::Parse

    if port < 1024 {
        return Err(AppError::Custom(format!("Port {} is reserved", port)));
    }

    Ok(port)
}

fn main() {
    match read_port_from_file("port.txt") {
        Ok(port) => println!("Port: {}", port),
        Err(e) => println!("Error: {}", e),
    }
}
```

> **💡 Trong production**: Dùng crate `thiserror` (derive macros cho custom errors) hoặc `anyhow` (any error type, for apps). Sẽ gặp lại ở Part IV.

---

## 10.6 — Error Handling Patterns

### Pattern 1: Collect kết quả, tách Ok/Err

```rust
// filename: src/main.rs
fn main() {
    let inputs = vec!["1", "two", "3", "four", "5"];

    // Cách 1: partition — tách thành 2 nhóm
    let (oks, errs): (Vec<_>, Vec<_>) = inputs.iter()
        .map(|s| s.parse::<i32>())
        .partition(|r| r.is_ok());

    let values: Vec<i32> = oks.into_iter().map(|r| r.unwrap()).collect();
    println!("Values: {:?}", values);   // [1, 3, 5]
    println!("Errors: {}", errs.len()); // 2

    // Cách 2: filter_map — chỉ lấy thành công
    let values: Vec<i32> = inputs.iter()
        .filter_map(|s| s.parse().ok())  // Err → None → bỏ qua
        .collect();
    println!("Filter_map: {:?}", values);  // [1, 3, 5]

    // Cách 3: collect Result — dừng tại lỗi đầu tiên
    let all: Result<Vec<i32>, _> = vec!["1", "2", "3"].iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("All OK: {:?}", all);  // Ok([1, 2, 3])

    let with_error: Result<Vec<i32>, _> = inputs.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("With error: {:?}", with_error);  // Err(...)
}
```

### Pattern 2: `ok_or` — Option → Result

```rust
// filename: src/main.rs
use std::collections::HashMap;

fn get_setting(config: &HashMap<&str, &str>, key: &str) -> Result<String, String> {
    config.get(key)
        .ok_or(format!("Missing setting: {}", key))?  // Option → Result
        .parse()
        .map_err(|e| format!("Invalid {}: {}", key, e))
}

fn main() {
    let mut config = HashMap::new();
    config.insert("port", "8080");
    config.insert("host", "localhost");

    println!("{:?}", get_setting(&config, "port")); // Ok("8080")
    println!("{:?}", get_setting(&config, "db"));   // Err("Missing setting: db")
}
```

---

## 10.7 — Error Handling Ecosystem

### Problem: Boilerplate

Trong 10.5, bạn viết custom error enum + `Display` + `Error` + `From` tay. Với 5 variant + 3 `From` impls, đó là ~80 dòng boilerplate. Các crate sau giúp giảm đáng kể:

### `thiserror` — Cho libraries (structured errors)

```rust
// filename: src/main.rs
// Cargo.toml: thiserror = "1"

use thiserror::Error;

#[derive(Debug, Error)]
enum OrderError {
    #[error("Cart is empty")]
    EmptyCart,

    #[error("Invalid quantity: {0}")]
    InvalidQuantity(u32),

    #[error("'{item}' out of stock (only {available} left)")]
    OutOfStock { item: String, available: u32 },

    #[error("Payment failed: {0}")]
    PaymentFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),  // auto From<io::Error>

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),  // auto From
}

// Không cần viết Display, Error, From tay nữa!
// thiserror derive tất cả từ #[error("...")] và #[from]

fn place_order(item: &str, qty: u32) -> Result<String, OrderError> {
    if item.is_empty() { return Err(OrderError::EmptyCart); }
    if qty == 0 { return Err(OrderError::InvalidQuantity(qty)); }
    Ok(format!("Ordered {}x {}", qty, item))
}

fn main() {
    match place_order("", 1) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),  // Display from #[error]
    }
    // Output: Error: Cart is empty
}
```

### `anyhow` — Cho applications (catch-all errors)

```rust
// filename: src/main.rs
// Cargo.toml: anyhow = "1"

use anyhow::{Context, Result, bail};

// Result = anyhow::Result<T> = Result<T, anyhow::Error>
// anyhow::Error chứa BẤT KỲ error nào impl std::error::Error

fn read_config(path: &str) -> Result<u16> {
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read config from '{}'", path))?;
        //       ↑ Thêm CONTEXT cho error — rất hữu ích khi debug

    let port: u16 = content.trim().parse()
        .context("Failed to parse port number")?;

    if port < 1024 {
        bail!("Port {} is reserved (must be >= 1024)", port);
        //    ↑ bail! = return Err(anyhow!("..."))
    }

    Ok(port)
}

fn main() -> Result<()> {
    let port = read_config("config.txt")?;
    println!("Port: {}", port);
    Ok(())
}

// Output (khi file không tồn tại):
// Error: Failed to read config from 'config.txt'
//
// Caused by:
//     No such file or directory (os error 2)
```

### Khi nào dùng cái nào?

| | `thiserror` | `anyhow` |
|---|---|---|
| **Dùng cho** | Libraries, shared code | Applications, `main()` |
| **Error type** | Custom enum (structured) | `anyhow::Error` (erased) |
| **Match errors?** | ✅ `match err { Variant => ... }` | ❌ Không match variant |
| **Context?** | Tự viết trong `#[error("...")]` | `.context("what happened")` |
| **Khi nào** | Cần callers xử lý specific errors | Chỉ cần log/display error |
| **Typical** | Domain layer, libraries | CLI apps, scripts, main() |

> **💡 Quy tắc**: Library → `thiserror`. Application → `anyhow`. Kết hợp được: library dùng `thiserror` enum, app dùng `anyhow` wrap library errors. Sẽ thấy pattern này rõ ở Ch 37 Capstone.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Chọn đúng method

Cho code, điền method phù hợp:

```rust
let x: Option<i32> = Some(10);
let a = x._____(0);              // a = 10, hoặc 0 nếu None
let b = x._____(|n| n * 2);     // b = Some(20)
let c = x._____(|&n| n > 5);    // c = Some(10)
```

<details><summary>✅ Lời giải Bài 1</summary>

```rust
let a = x.unwrap_or(0);
let b = x.map(|n| n * 2);
let c = x.filter(|&n| n > 5);
```

</details>

---

**Bài 2** (10 phút): Safe calculator

Viết 4 functions: `safe_add`, `safe_sub`, `safe_mul`, `safe_div` — tất cả trả `Result<f64, CalcError>`. `CalcError` có 2 variants: `DivisionByZero`, `Overflow`. Chain chúng bằng `and_then`: `(10 + 5) * 3 / 0` → phải trả Err.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
// filename: src/main.rs
use std::fmt;

#[derive(Debug)]
enum CalcError {
    DivisionByZero,
    Overflow,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "Division by zero"),
            CalcError::Overflow => write!(f, "Overflow"),
        }
    }
}

fn safe_add(a: f64, b: f64) -> Result<f64, CalcError> {
    let result = a + b;
    if result.is_infinite() { Err(CalcError::Overflow) } else { Ok(result) }
}

fn safe_sub(a: f64, b: f64) -> Result<f64, CalcError> {
    let result = a - b;
    if result.is_infinite() { Err(CalcError::Overflow) } else { Ok(result) }
}

fn safe_mul(a: f64, b: f64) -> Result<f64, CalcError> {
    let result = a * b;
    if result.is_infinite() { Err(CalcError::Overflow) } else { Ok(result) }
}

fn safe_div(a: f64, b: f64) -> Result<f64, CalcError> {
    if b == 0.0 { Err(CalcError::DivisionByZero) } else { Ok(a / b) }
}

fn main() {
    // (10 + 5) * 3 / 2 = 22.5
    let result = safe_add(10.0, 5.0)
        .and_then(|r| safe_mul(r, 3.0))
        .and_then(|r| safe_div(r, 2.0));
    println!("(10+5)*3/2 = {:?}", result);  // Ok(22.5)

    // (10 + 5) * 3 / 0 → error dừng tại div
    let result = safe_add(10.0, 5.0)
        .and_then(|r| safe_mul(r, 3.0))
        .and_then(|r| safe_div(r, 0.0));
    println!("(10+5)*3/0 = {:?}", result);  // Err(DivisionByZero)
}
```

</details>

---

**Bài 3** (15 phút): CSV parser

Viết function `parse_csv(input: &str) -> Result<Vec<Record>, ParseError>`. Mỗi dòng có format `name,age,email`. `ParseError` enum có: `EmptyInput`, `InvalidLine { line_num, content }`, `InvalidAge { line_num, value }`. Test với data có 1 dòng lỗi.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs
use std::fmt;

#[derive(Debug)]
struct Record {
    name: String,
    age: u32,
    email: String,
}

#[derive(Debug)]
enum ParseError {
    EmptyInput,
    InvalidLine { line_num: usize, content: String },
    InvalidAge { line_num: usize, value: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Input is empty"),
            ParseError::InvalidLine { line_num, content } =>
                write!(f, "Line {}: invalid format '{}'", line_num, content),
            ParseError::InvalidAge { line_num, value } =>
                write!(f, "Line {}: invalid age '{}'", line_num, value),
        }
    }
}

fn parse_csv(input: &str) -> Result<Vec<Record>, ParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let mut records = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line_num = i + 1;
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() != 3 {
            return Err(ParseError::InvalidLine {
                line_num,
                content: line.to_string(),
            });
        }

        let age: u32 = parts[1].parse().map_err(|_| ParseError::InvalidAge {
            line_num,
            value: parts[1].to_string(),
        })?;

        records.push(Record {
            name: parts[0].to_string(),
            age,
            email: parts[2].to_string(),
        });
    }

    Ok(records)
}

fn main() {
    let valid = "Minh,25,minh@email.com\nLan,30,lan@email.com";
    match parse_csv(valid) {
        Ok(records) => {
            for r in &records {
                println!("✅ {} ({}): {}", r.name, r.age, r.email);
            }
        }
        Err(e) => println!("❌ {}", e),
    }

    let invalid = "Minh,25,minh@email.com\nBad Line\nLan,30,lan@email.com";
    match parse_csv(invalid) {
        Ok(_) => println!("OK"),
        Err(e) => println!("❌ {}", e),
    }

    // Output:
    // ✅ Minh (25): minh@email.com
    // ✅ Lan (30): lan@email.com
    // ❌ Line 2: invalid format 'Bad Line'
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `the ? operator can only be used in a function that returns Result/Option` | `?` trong `main()` hoặc function trả `()` | Đổi return type thành `Result<(), Error>` |
| `cannot use the ? operator in a function that returns Option on a Result` | Mix Option `?` và Result `?` | Dùng `.ok()?` hoặc `.ok_or(err)?` để convert |
| `panicked at unwrap()` | `unwrap()` trên Err/None | Thay bằng `?`, `unwrap_or`, hoặc `match` |
| Custom error không impl Display | `?` cần error impl Display | Thêm `impl fmt::Display for MyError` |

---

## Tóm tắt

- ✅ **`Option<T>`** = `Some` hoặc `None`. Thay thế null. Methods: `map`, `and_then`, `unwrap_or`, `filter`.
- ✅ **`Result<T, E>`** = `Ok` hoặc `Err` + thông tin. Method chain dừng tại lỗi đầu tiên (railway).
- ✅ **`?` operator** = propagate errors ngắn gọn. Err → return Err ngay. Dùng trong functions trả Result/Option.
- ✅ **Custom errors** = enum + Display + Error. `From` impl cho `?` auto-convert. Production: dùng `thiserror`/`anyhow`.
- ✅ **Quy tắc**: Production → `?` hoặc `match`. Tests → `unwrap()` OK. **KHÔNG** `unwrap()` trong production.

## Tiếp theo

→ Chapter 11: **Modules, Crates & Cargo** — bạn sẽ học cách tổ chức code thành modules, publish crates, và quản lý dependencies. Chapter cuối của Part I — sau đó bước vào Thinking Functionally!
