# Chapter 0 — Rust in 10 Minutes

> **Mục đích**: Giới thiệu đủ cú pháp Rust cơ bản để bạn **đọc hiểu** code examples trong Part 0 (Chapters 1–3). Đây không phải hướng dẫn đầy đủ — Part I sẽ dạy lại mọi thứ chi tiết hơn.
>
> **Yêu cầu trước**: Không có. Biết bất kỳ ngôn ngữ lập trình nào là lợi thế, nhưng không bắt buộc.
> **Thời gian đọc**: ~10–15 phút | **Level**: Intro
> **Kết quả cuối cùng**: Bạn có thể đọc hiểu mọi đoạn code trong Part 0 mà không bị "kẹt" ở syntax.

---

## Quick Reference — Toàn cảnh Rust trong 10 phút

Chapter này dành cho hai nhóm người: developers muốn xem nhanh Rust trước khi commit vào 40+ chapters, và readers muốn quay lại refresh syntax sau khi đọc xong. Nó hoạt động như **bản đồ du lịch** — cho bạn thấy toàn cảnh trước khi khám phá chi tiết.

Mỗi section chỉ vài dòng code + 1-2 câu giải thích. Bạn sẽ gặp lại tất cả concepts này, với giải thích sâu hơn nhiều, trong các chapters sau.

---

## 0.1 — Cài đặt & Tạo project

### Cài Rust

Mở terminal và chạy:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Kiểm tra:

```bash
rustc --version
# Output (ví dụ): rustc 1.84.0 (9fc6b43 2025-01-07)

cargo --version
# Output (ví dụ): cargo 1.84.0 (fb7f12831 2024-12-18)
```

`rustc` là compiler. `cargo` là build tool + package manager — bạn sẽ dùng `cargo` cho mọi thứ.

### Tạo project mới

```bash
cargo new hello_rust
cd hello_rust
```

Bạn sẽ thấy cấu trúc:

```
hello_rust/
├── Cargo.toml    # file cấu hình (giống package.json)
└── src/
    └── main.rs   # code chính
```

Chạy thử:

```bash
cargo run
# Output: Hello, world!
```

`cargo run` = biên dịch + chạy. Nếu chỉ muốn biên dịch: `cargo build`. Nếu muốn kiểm tra lỗi mà không tạo file: `cargo check`.

---

## 0.2 — Biến và kiểu dữ liệu

### `let` — khai báo biến

```rust
fn main() {
    let x = 5;           // Rust tự suy ra kiểu i32
    let y: f64 = 3.14;   // Ghi rõ kiểu: số thực 64-bit
    let name = "Rust";    // &str — chuỗi tham chiếu (string slice)
    let active = true;    // bool

    println!("x = {}, y = {}, name = {}, active = {}", x, y, name, active);
    // Output: x = 5, y = 3.14, name = Rust, active = true
}
```

Một điểm khác biệt lớn: **biến mặc định là bất biến** (immutable). Muốn thay đổi được thì thêm `mut`:

```rust
fn main() {
    let x = 5;
    // x = 10;  // ❌ Lỗi! Biến immutable không thay đổi được

    let mut y = 5;
    y = 10;      // ✅ OK — có mut
    println!("y = {}", y);
    // Output: y = 10
}
```

> **💡 Tại sao immutable mặc định?** Vì nó an toàn hơn. Khi biến không thay đổi, bạn không cần lo "ai đó sửa giá trị ở dòng 200". Đây là tư duy FP — bạn sẽ hiểu rõ hơn ở Part II.

### Các kiểu cơ bản

| Kiểu | Ý nghĩa | Ví dụ |
|------|---------|-------|
| `i32` | Số nguyên 32-bit | `42`, `-7` |
| `f64` | Số thực 64-bit | `3.14`, `-0.5` |
| `bool` | True/False | `true`, `false` |
| `char` | Một ký tự Unicode | `'a'`, `'🦀'` |
| `&str` | Chuỗi tham chiếu (slice) | `"hello"` |
| `String` | Chuỗi sở hữu (owned) | `String::from("hello")` |

Sự khác nhau giữa `&str` và `String` sẽ được giải thích kỹ ở Chapter 9 (Ownership). Hiện tại chỉ cần biết: `"hello"` là `&str`, còn `String::from("hello")` hay `"hello".to_string()` là `String`.

---

## 0.3 — Functions

```rust
// Khai báo function: fn + tên + tham số kèm kiểu + kiểu trả về
fn add(a: i32, b: i32) -> i32 {
    a + b  // Dòng cuối KHÔNG có dấu ; → tự động return
}

fn greet(name: &str) {
    // Không có -> ... nghĩa là trả về () (unit — "không có gì")
    println!("Hello, {}!", name);
}

fn main() {
    let sum = add(3, 5);
    println!("3 + 5 = {}", sum);
    // Output: 3 + 5 = 8

    greet("Rust");
    // Output: Hello, Rust!
}
```

Lưu ý quan trọng: **Dòng cuối không có `;`** → nó là giá trị trả về. Nếu bạn thêm `;`, function trả `()` (không có gì). Đây là lỗi phổ biến nhất của người mới.

```rust
fn double(x: i32) -> i32 {
    x * 2     // ✅ Trả về x * 2
    // x * 2; // ❌ Nếu thêm ; → trả () → compiler báo lỗi type mismatch
}
```

---

## 0.4 — `if/else` là expression

Trong Rust, `if/else` **trả về giá trị** — không chỉ là câu lệnh:

```rust
fn main() {
    let score = 85;

    // if/else trả về giá trị → gán vào biến
    let grade = if score >= 90 {
        "A"
    } else if score >= 70 {
        "B"
    } else {
        "C"
    };

    println!("Score {}: grade {}", score, grade);
    // Output: Score 85: grade B
}
```

> **💡 Ghi nhớ**: Hầu hết mọi thứ trong Rust đều là **expression** (có giá trị), không chỉ là statement (lệnh). Tư duy này rất quen với FP.

---

## 0.5 — `enum` — Một trong nhiều lựa chọn

`enum` định nghĩa một kiểu mà giá trị phải là **một trong** các variant:

```rust
// Đèn giao thông: chỉ có thể là Đỏ, Vàng, hoặc Xanh
#[derive(Debug)]        // cho phép in {:?}
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;
    println!("Light: {:?}", light);
    // Output: Light: Red
}
```

Enum variant có thể **chứa data** bên trong:

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),                        // bán kính
    Rectangle { width: f64, height: f64 }, // chiều rộng × cao
}

fn main() {
    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle { width: 3.0, height: 4.0 };
    println!("{:?}", c);  // Output: Circle(5.0)
    println!("{:?}", r);  // Output: Rectangle { width: 3.0, height: 4.0 }
}
```

---

## 0.6 — `struct` — Gom nhiều thứ lại

`struct` gom nhiều fields thành một kiểu (tất cả fields phải có):

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 2.5 };
    println!("Point: ({}, {})", p.x, p.y);
    // Output: Point: (1, 2.5)
}
```

---

## 0.7 — `match` — Pattern matching

`match` giống `switch/case` nhưng mạnh hơn rất nhiều. Compiler **bắt buộc** bạn xử lý mọi trường hợp:

```rust
#[derive(Debug)]
enum Season { Spring, Summer, Fall, Winter }

fn describe(season: &Season) -> &str {
    match season {
        Season::Spring => "Hoa nở 🌸",
        Season::Summer => "Nắng nóng ☀️",
        Season::Fall   => "Lá rụng 🍂",
        Season::Winter => "Lạnh giá ❄️",
    }
    // Nếu bạn quên một variant → compiler báo lỗi ngay!
}

fn main() {
    let now = Season::Summer;
    println!("{}: {}", format!("{:?}", now), describe(&now));
    // Output: Summer: Nắng nóng ☀️
}
```

`match` cũng destructure được data bên trong enum:

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        // Lấy radius ra từ Circle
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        // Lấy width, height ra từ Rectangle
        Shape::Rectangle { width, height } => width * height,
    }
}

fn main() {
    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle { width: 3.0, height: 4.0 };

    println!("Circle area: {:.2}", area(&c));
    println!("Rect area: {:.2}", area(&r));
    // Output:
    // Circle area: 78.54
    // Rect area: 12.00
}
```

---

## 0.8 — Closures — Function không tên

**Closure** = function nhỏ gọn, viết inline bằng `||`:

```rust
fn main() {
    // Closure: |tham_số| thân_hàm
    let add_one = |x: i32| x + 1;
    let multiply = |a: i32, b: i32| a * b;

    println!("add_one(5) = {}", add_one(5));       // 6
    println!("multiply(3, 4) = {}", multiply(3, 4)); // 12

    // Closure thường dùng với .map(), .filter()...
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    // Output: Doubled: [2, 4, 6, 8, 10]
}
```

> **💡 Ghi nhớ**: `|x| x + 1` trong Rust = `λx. x + 1` trong toán. Bạn sẽ gặp lại ở Chapter 1.

---

## 0.9 — `Vec`, vòng lặp, và `println!`

### `Vec` — Mảng động

```rust
fn main() {
    // Tạo Vec bằng macro vec![]
    let fruits = vec!["🍎", "🍊", "🍇"];

    println!("First: {}", fruits[0]);    // truy cập theo index
    println!("Length: {}", fruits.len()); // độ dài

    // Duyệt bằng for..in
    for fruit in &fruits {
        println!("- {}", fruit);
    }
    // Output:
    // First: 🍎
    // Length: 3
    // - 🍎
    // - 🍊
    // - 🍇
}
```

### `println!` — In ra console

```rust
fn main() {
    let name = "Rust";
    let version = 1.84;

    // {} = placeholder, sẽ được thay bằng giá trị
    println!("Hello, {}!", name);
    println!("{} version {}", name, version);

    // {:?} = Debug format — in struct/enum dạng debug
    let nums = vec![1, 2, 3];
    println!("nums = {:?}", nums);

    // {:.2} = số thực, 2 chữ số thập phân
    println!("Pi ≈ {:.2}", std::f64::consts::PI);

    // Output:
    // Hello, Rust!
    // Rust version 1.84
    // nums = [1, 2, 3]
    // Pi ≈ 3.14
}
```

### `assert_eq!` — Tự kiểm tra

```rust
fn main() {
    let result = 2 + 3;
    assert_eq!(result, 5);  // ✅ OK — 5 == 5, không in gì

    // assert_eq!(result, 99); // ❌ Panic! left: 5, right: 99
    println!("All assertions passed!");
    // Output: All assertions passed!
}
```

> **💡 Bạn sẽ gặp `assert_eq!`** rất nhiều trong sách — nó giúp code tự kiểm tra kết quả.

---

## 0.10 — `Option` và `Result` (đọc hiểu)

Đây là hai kiểu bạn sẽ gặp trong Part 0. Chưa cần hiểu sâu — chỉ cần **nhận ra** khi thấy nó.

### `Option<T>` — Có thể có, có thể không

```rust
fn find_even(numbers: &[i32]) -> Option<i32> {
    // Tìm số chẵn đầu tiên
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);  // Tìm thấy → Some(giá_trị)
        }
    }
    None  // Không tìm thấy → None
}

fn main() {
    match find_even(&[1, 3, 4, 7]) {
        Some(n) => println!("Found even: {}", n),
        None    => println!("No even number"),
    }
    // Output: Found even: 4

    match find_even(&[1, 3, 7]) {
        Some(n) => println!("Found even: {}", n),
        None    => println!("No even number"),
    }
    // Output: No even number
}
```

`Option<T>` = `Some(giá_trị)` hoặc `None`. Compiler buộc bạn xử lý cả hai.

### `Result<T, E>` — Thành công hoặc lỗi

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())  // Lỗi
    } else {
        Ok(a / b)  // Thành công
    }
}

fn main() {
    match divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e)     => println!("Error: {}", e),
    }
    // Output: 10 / 3 = 3.33

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {:.2}", result),
        Err(e)     => println!("Error: {}", e),
    }
    // Output: Error: Cannot divide by zero
}
```

`Result<T, E>` = `Ok(giá_trị)` hoặc `Err(lỗi)`. Tương tự `Option` — compiler buộc bạn xử lý cả hai.

---

## 🔧 Tham chiếu nhanh

| Cú pháp | Ý nghĩa | Ví dụ |
|---------|---------|-------|
| `let x = 5;` | Biến immutable | `let name = "Rust";` |
| `let mut x = 5;` | Biến mutable | `x = 10;` được |
| `fn foo(a: i32) -> i32` | Function | Tham số có kiểu, trả về kiểu |
| `\|x\| x + 1` | Closure | Function không tên |
| `enum { A, B }` | Sum type | Một trong nhiều variant |
| `struct { x, y }` | Product type | Tất cả fields |
| `match val { ... }` | Pattern matching | Phải cover hết variants |
| `Some(v)` / `None` | Option | Có hoặc không |
| `Ok(v)` / `Err(e)` | Result | Thành công hoặc lỗi |
| `vec![1, 2, 3]` | Vec (mảng động) | `Vec<i32>` |
| `println!("{}", x)` | In ra console | `{:?}` cho debug |
| `assert_eq!(a, b)` | Kiểm tra bằng nhau | Panic nếu khác |

---

## Tóm tắt

Chapter này chỉ cung cấp **đủ syntax** để bạn đọc hiểu code trong Part 0. Bạn **không cần nhớ hết** — hãy dùng bảng tham chiếu ở trên khi cần.

Những thứ **chưa đề cập** (sẽ học kỹ ở Part I):
- **Ownership & Borrowing** (Chapter 9) — hệ thống quản lý bộ nhớ độc đáo của Rust
- **Traits** (Chapter 16) — tương tự interfaces
- **Lifetimes** (Chapter 9) — quản lý thời gian sống của tham chiếu
- **Error handling** chi tiết (Chapter 10)
- **Modules & Crates** (Chapter 11)

Bạn sẽ gặp lại mọi concept ở đây nhưng ở mức **sâu hơn rất nhiều** trong Part I.

## Tiếp theo

→ Chapter 1: **Math Foundations for FP** — bạn sẽ học Lambda Calculus, Curry-Howard (types = logic), và cách dùng phép cộng/nhân để đếm trạng thái hệ thống.
