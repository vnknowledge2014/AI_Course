# Chapter 7 — Functions & Closures

> **Bạn sẽ học được**:
> - Functions chi tiết: expressions vs statements, multiple return, early return
> - Closures sâu hơn: capture by reference vs by value
> - `Fn`, `FnMut`, `FnOnce` — ba "cấp độ" closure
> - First-class functions — functions là values, truyền qua biến
> - Higher-order functions — functions nhận/trả functions
>
> **Yêu cầu trước**: Chapter 5 (Variables, Types), Chapter 6 (Control Flow, `match`).
> **Thời gian đọc**: ~40 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn hiểu closures đủ sâu để dùng `.map()`, `.filter()`, `.fold()` tự tin, và thiết kế higher-order functions cho riêng mình.

---

## Functions & Closures — Công dân hạng nhất

Trong functional programming, functions là "công dân hạng nhất" (first-class citizens) — bạn có thể truyền chúng như arguments, trả về từ functions khác, lưu vào variables. Đây là nền tảng cho mọi pattern FP: higher-order functions, composition, callbacks.

Rust đi xa hơn most languages: closures (anonymous functions) có ownership semantics riêng — `Fn`, `FnMut`, `FnOnce` — cho phép compiler kiểm tra chúng capture environment an toàn. Không memory leaks, không dangling references, tất cả tại compile time.

Chapter này trang bị cho bạn hai skill: viết functions hiệu quả, và sử dụng closures — cả hai bạn sẽ dùng trong mọi dòng Rust code.

---

## 7.1 — Functions: Nền tảng

### Expressions vs Statements

Trong Rust, gần như mọi thứ đều là **expression** (trả giá trị). Sự khác biệt nhỏ nhưng quan trọng:

```rust
// filename: src/main.rs

fn classify(score: u32) -> &'static str {
    // Toàn bộ match là expression → trả thẳng, không cần return
    match score {
        90..=100 => "Excellent",
        70..=89  => "Good",
        50..=69  => "Average",
        _        => "Needs improvement",
    }
    // ↑ không có ; → giá trị này được return
}

fn add(a: i32, b: i32) -> i32 {
    a + b   // expression — trả giá trị
    // a + b;  // ← thêm ; → thành statement → trả () → LỖI type mismatch!
}

fn main() {
    println!("{}", classify(85));  // Good
    println!("{}", add(3, 5));     // 8
}
```

> **💡 Quy tắc vàng**: Dòng cuối function **không có `;`** = giá trị return. Thêm `;` = trả `()`.

### Early Return

```rust
// filename: src/main.rs

fn find_first_negative(items: &[i32]) -> Option<i32> {
    for &item in items {
        if item < 0 {
            return Some(item);  // thoát sớm — tìm thấy rồi, không cần duyệt tiếp
        }
    }
    None  // dòng cuối, không có ; → return None
}

fn main() {
    let data = vec![3, 7, -2, 9, -5];
    println!("First negative: {:?}", find_first_negative(&data));
    println!("No negatives: {:?}", find_first_negative(&[1, 2, 3]));
    // Output:
    // First negative: Some(-2)
    // No negatives: None
}
```

### Functions trả về multiple values qua tuples

```rust
// filename: src/main.rs

fn stats(items: &[f64]) -> (f64, f64, f64) {
    let sum: f64 = items.iter().sum();
    let count = items.len() as f64;
    let avg = sum / count;

    let min = items.iter().cloned().reduce(f64::min).unwrap();
    let max = items.iter().cloned().reduce(f64::max).unwrap();

    (avg, min, max)  // trả tuple 3 phần tử
}

fn main() {
    let prices = vec![35.0, 25.0, 45.0, 40.0, 30.0];
    let (avg, min, max) = stats(&prices);  // destructure
    println!("Avg: {:.1}, Min: {:.1}, Max: {:.1}", avg, min, max);
    // Output: Avg: 35.0, Min: 25.0, Max: 45.0
}
```

---

## ✅ Checkpoint 7.1

> Ghi nhớ:
> 1. Dòng cuối không `;` = return (expression). Có `;` = statement trả `()`
> 2. `return` keyword chỉ dùng cho early return
> 3. Trả nhiều giá trị bằng tuple, destructure tại call site
>
> **Test nhanh**: Function trả `i32` nhưng dòng cuối là `x + 1;` — chuyện gì xảy ra?
> <details><summary>Đáp án</summary>Error: <code>mismatched types — expected i32, found ()</code>. Dấu <code>;</code> biến expression thành statement trả <code>()</code>.</details>

---

## 7.2 — Closures: Functions có "ký ức"

### Closure vs Function

Function thường chỉ biết **tham số** truyền vào. Closure biết thêm **biến xung quanh** (environment):

```rust
// filename: src/main.rs
fn main() {
    let tax_rate = 0.08;  // biến bên ngoài

    // Closure "bắt" (capture) tax_rate từ environment
    let calculate_total = |price: f64| -> f64 {
        price * (1.0 + tax_rate)  // dùng tax_rate — không phải tham số!
    };

    println!("35000đ + tax = {:.0}đ", calculate_total(35_000.0));
    println!("25000đ + tax = {:.0}đ", calculate_total(25_000.0));

    // Output:
    // 35000đ + tax = 37800đ
    // 25000đ + tax = 27000đ
}
```

`calculate_total` "nhớ" `tax_rate` — dù không nhận nó qua tham số. Đây là tính năng mà function thường **không có**.

### Ba cách capture

Rust compiler tự chọn cách capture **tối thiểu cần thiết**:

```rust
// filename: src/main.rs
fn main() {
    // 1. Capture by shared reference (&T) — đọc nhưng không sửa
    let name = String::from("Rust");
    let greet = || println!("Hello, {}!", name);  // chỉ đọc name
    greet();
    greet();  // gọi nhiều lần OK
    println!("name still exists: {}", name);  // name vẫn dùng được

    // 2. Capture by mutable reference (&mut T) — đọc VÀ sửa
    let mut count = 0;
    let mut increment = || {
        count += 1;  // sửa count → capture &mut
        println!("Count: {}", count);
    };
    increment();  // Count: 1
    increment();  // Count: 2
    // println!("{}", count);  // ❌ không dùng count được khi increment còn sống
    drop(increment);  // giải phóng mutable borrow
    println!("Final count: {}", count);  // ✅ OK — 2

    // 3. Capture by value (move) — lấy luôn ownership
    let data = vec![1, 2, 3];
    let consume = move || {
        println!("Data: {:?}", data);  // data bị move vào closure
    };
    consume();
    // println!("{:?}", data);  // ❌ data đã bị move!
}
```

### `move` keyword

`move` ép closure **lấy ownership** tất cả captured variables. Quan trọng nhất khi truyền closure sang thread khác:

```rust
// filename: src/main.rs
fn main() {
    let message = String::from("Hello from closure!");

    // Không move: closure mượn &message
    let borrow_closure = || println!("{}", message);
    borrow_closure();
    println!("Still mine: {}", message);  // ✅ OK

    // Với move: closure SỞ HỮU message
    let move_closure = move || println!("{}", message);
    move_closure();
    // println!("{}", message);  // ❌ message đã bị move

    // Output:
    // Hello from closure!
    // Still mine: Hello from closure!
    // Hello from closure!
}
```

---

## ✅ Checkpoint 7.2

> Ghi nhớ:
> 1. **Closure** = function + environment (capture biến xung quanh)
> 2. Rust tự chọn: `&T` (đọc), `&mut T` (sửa), hay move (lấy ownership)
> 3. `move ||` = ép closure lấy ownership — dùng khi truyền qua threads
>
> **Test nhanh**: Closure `|| println!("{}", name)` capture `name` bằng cách nào?
> <details><summary>Đáp án</summary>Shared reference <code>&name</code> — chỉ đọc, không sửa, nên Rust chọn cách ít restrictive nhất.</details>

---

## 7.3 — `Fn`, `FnMut`, `FnOnce`: Ba cấp độ

Khi bạn viết function nhận closure làm tham số, bạn cần chọn **trait bound** phù hợp:

| Trait | Capture | Gọi được | Ẩn dụ |
|-------|---------|----------|-------|
| `Fn` | `&T` (shared ref) | Nhiều lần ✅ | Đọc sách thư viện — ai cũng đọc được |
| `FnMut` | `&mut T` (mutable ref) | Nhiều lần ✅ | Viết lên bảng — chỉ 1 người viết |
| `FnOnce` | Move (ownership) | **Đúng 1 lần** ❌ | Mở quà — chỉ mở được 1 lần |

Hierarchy: `FnOnce` ⊃ `FnMut` ⊃ `Fn`

Mọi `Fn` closure cũng là `FnMut` và `FnOnce`. Nhưng không phải ngược lại.

```rust
// filename: src/main.rs

// Nhận Fn — closure chỉ đọc, gọi nhiều lần
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))  // gọi f 2 lần
}

// Nhận FnMut — closure có thể sửa captured variables
fn call_many_times<F: FnMut()>(mut f: F, times: u32) {
    for _ in 0..times {
        f();
    }
}

// Nhận FnOnce — closure chỉ gọi 1 lần (có thể consume data)
fn call_once<F: FnOnce() -> String>(f: F) -> String {
    f()  // chỉ gọi 1 lần
    // f()  // ❌ gọi lần 2 → error: value used after move
}

fn main() {
    // Fn: closure chỉ đọc
    let double = |x: i32| x * 2;
    println!("apply_twice(double, 3) = {}", apply_twice(double, 3));
    // 3 → 6 → 12

    // FnMut: closure sửa biến
    let mut total = 0;
    call_many_times(|| {
        total += 10;
    }, 5);
    println!("Total: {}", total);  // 50

    // FnOnce: closure consume data
    let data = vec![1, 2, 3];
    let result = call_once(move || {
        format!("Data: {:?}", data)  // data bị consumed
    });
    println!("{}", result);

    // Output:
    // apply_twice(double, 3) = 12
    // Total: 50
    // Data: [1, 2, 3]
}
```

### Quy tắc chọn trait bound

> **💡 Luôn chọn `Fn` trước**. Chỉ "nới lỏng" lên `FnMut` hoặc `FnOnce` khi compiler bảo.

| Closure làm gì | Dùng trait |
|---|---|
| Chỉ đọc captured vars | `Fn` |
| Sửa captured vars | `FnMut` |
| Move/consume captured vars | `FnOnce` |
| Không biết → bắt đầu từ | `Fn` |

---

## 7.4 — First-Class Functions

"First-class functions" = functions là **values** — gán vào biến, truyền vào function khác, trả về từ function.

### Gán function vào biến

```rust
// filename: src/main.rs

fn add(a: i32, b: i32) -> i32 { a + b }
fn multiply(a: i32, b: i32) -> i32 { a * b }

fn main() {
    // Function thường cũng gán vào biến được
    let operation: fn(i32, i32) -> i32 = add;
    println!("add(3, 5) = {}", operation(3, 5));

    let operation = multiply;  // đổi sang multiply
    println!("multiply(3, 5) = {}", operation(3, 5));

    // Mảng functions
    let ops: Vec<fn(i32, i32) -> i32> = vec![add, multiply];
    for op in &ops {
        println!("op(10, 3) = {}", op(10, 3));
    }

    // Output:
    // add(3, 5) = 8
    // multiply(3, 5) = 15
    // op(10, 3) = 13
    // op(10, 3) = 30
}
```

### Function pointers vs Closures

```rust
// filename: src/main.rs

// fn pointer: fn(i32) -> i32 — chỉ cho function KHÔNG capture
// Closure trait: Fn(i32) -> i32 — cho cả function VÀ closure

// Dùng generic → chấp nhận cả hai
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn double(x: i32) -> i32 { x * 2 }

fn main() {
    // Truyền function thường
    println!("double(5) = {}", apply(double, 5));

    // Truyền closure
    let offset = 10;
    println!("add_10(5) = {}", apply(|x| x + offset, 5));

    // Output:
    // double(5) = 10
    // add_10(5) = 15
}
```

---

## 7.5 — Higher-Order Functions: Xây dựng pipeline

Higher-order functions (HOF) = functions **nhận functions** hoặc **trả functions**. Đây là trái tim của FP.

### Functions nhận functions

```rust
// filename: src/main.rs

// "Máy lọc" — nhận predicate (function trả bool)
fn filter_items<T, F>(items: &[T], predicate: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|item| predicate(item)).collect()
}

// "Máy biến đổi" — nhận transform function
fn transform_items<T, U, F>(items: &[T], transform: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    items.iter().map(|item| transform(item)).collect()
}

fn main() {
    let prices = vec![35_000, 25_000, 45_000, 15_000, 55_000];

    // Lọc: chỉ lấy giá >= 30k
    let premium = filter_items(&prices, |&p| p >= 30_000);
    println!("Premium: {:?}", premium);

    // Biến đổi: format thành chuỗi
    let labels = transform_items(&prices, |&p| format!("{}đ", p));
    println!("Labels: {:?}", labels);

    // Output:
    // Premium: [35000, 45000, 55000]
    // Labels: ["35000đ", "25000đ", "45000đ", "15000đ", "55000đ"]
}
```

### Functions trả functions

```rust
// filename: src/main.rs

// Function trả closure — Factory pattern!
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor  // capture factor bằng move
}

// Function trả closure với config
fn make_validator(min: i32, max: i32) -> impl Fn(i32) -> bool {
    move |value| value >= min && value <= max
}

fn main() {
    // Tạo "máy nhân" tùy chỉnh
    let double = make_multiplier(2);
    let triple = make_multiplier(3);

    println!("double(5) = {}", double(5));   // 10
    println!("triple(5) = {}", triple(5));   // 15

    // Tạo "máy kiểm tra" tùy chỉnh
    let is_valid_age = make_validator(0, 150);
    let is_valid_score = make_validator(0, 100);

    println!("Age 25: {}", is_valid_age(25));      // true
    println!("Age 200: {}", is_valid_age(200));     // false
    println!("Score 85: {}", is_valid_score(85));   // true
    println!("Score 105: {}", is_valid_score(105)); // false
}
```

### Compose functions — Nối pipeline

```rust
// filename: src/main.rs

fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    let to_string = |x: i32| format!("Result: {}", x);

    // Compose: add_one → double
    let add_then_double = compose(add_one, double);
    println!("{}", add_then_double(5));  // (5+1)*2 = 12

    // Compose tiếp: add_one → double → to_string
    let pipeline = compose(add_then_double, to_string);
    println!("{}", pipeline(5));  // "Result: 12"

    // Output:
    // 12
    // Result: 12
}
```

### Thực tế: Pipeline với iterator chains

```rust
// filename: src/main.rs

#[derive(Debug)]
struct Order {
    customer: String,
    amount: u32,
    is_paid: bool,
}

fn main() {
    let orders = vec![
        Order { customer: "Minh".into(), amount: 35_000, is_paid: true },
        Order { customer: "Lan".into(), amount: 55_000, is_paid: false },
        Order { customer: "Hùng".into(), amount: 25_000, is_paid: true },
        Order { customer: "Mai".into(), amount: 45_000, is_paid: true },
        Order { customer: "Dũng".into(), amount: 15_000, is_paid: false },
    ];

    // Pipeline: lọc đã thanh toán → lấy amount → tính tổng
    let total_paid: u32 = orders.iter()
        .filter(|o| o.is_paid)               // HOF: filter nhận closure
        .map(|o| o.amount)                    // HOF: map nhận closure
        .sum();                               // HOF: fold/reduce

    // Pipeline: tìm top spenders (paid, >= 30k)
    let top_customers: Vec<&str> = orders.iter()
        .filter(|o| o.is_paid && o.amount >= 30_000)
        .map(|o| o.customer.as_str())
        .collect();

    println!("Total paid: {}đ", total_paid);
    println!("Top customers: {:?}", top_customers);

    // Output:
    // Total paid: 105000đ
    // Top customers: ["Minh", "Mai"]
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Đọc closures

Mỗi closure dưới đây capture kiểu gì? (`Fn`, `FnMut`, hay `FnOnce`)

```rust
let name = String::from("Rust");
let a = || println!("{}", name);            // ?
let mut count = 0;
let mut b = || { count += 1; };            // ?
let data = vec![1, 2, 3];
let c = move || { drop(data); };           // ?
```

<details><summary>✅ Lời giải Bài 1</summary>

```
a: Fn — chỉ đọc name (&name)
b: FnMut — sửa count (&mut count)
c: FnOnce — move + drop data (consume ownership)
```

</details>

---

**Bài 2** (10 phút): Viết `make_greeting`

Viết function trả closure: `make_greeting(prefix: &str) -> impl Fn(&str) -> String`. `make_greeting("Hi")("Rust")` → `"Hi, Rust!"`.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
// filename: src/main.rs

fn make_greeting(prefix: String) -> impl Fn(&str) -> String {
    move |name| format!("{}, {}!", prefix, name)
}

// 💡 Phiên bản nâng cao: dùng &str + lifetime (học thêm ở Chapter 9)
// fn make_greeting_ref(prefix: &str) -> impl Fn(&str) -> String + '_ {
//     move |name| format!("{}, {}!", prefix, name)
// }

fn main() {
    let hi = make_greeting("Hi".to_string());
    let hello = make_greeting("Hello".to_string());

    println!("{}", hi("Rust"));     // Hi, Rust!
    println!("{}", hello("World")); // Hello, World!
}
```

</details>

---

**Bài 3** (15 phút): Pipeline processor

Viết struct `Pipeline` lưu vector closures `Vec<Box<dyn Fn(i32) -> i32>>`. Methods: `add_step(closure)`, `execute(input) -> i32` (chạy tất cả steps tuần tự). Test: `add 1 → double → subtract 3` trên input 5 → `(5+1)*2-3 = 9`.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs

struct Pipeline {
    steps: Vec<Box<dyn Fn(i32) -> i32>>,
}

impl Pipeline {
    fn new() -> Self {
        Pipeline { steps: vec![] }
    }

    fn add_step<F: Fn(i32) -> i32 + 'static>(mut self, step: F) -> Self {
        self.steps.push(Box::new(step));
        self  // return self cho method chaining
    }

    fn execute(&self, input: i32) -> i32 {
        self.steps.iter().fold(input, |acc, step| step(acc))
    }
}

fn main() {
    let pipeline = Pipeline::new()
        .add_step(|x| x + 1)      // +1
        .add_step(|x| x * 2)      // ×2
        .add_step(|x| x - 3);     // -3

    let result = pipeline.execute(5);
    println!("Pipeline(5) = {}", result);
    assert_eq!(result, 9);  // (5+1)*2-3 = 9

    // Reuse pipeline
    println!("Pipeline(10) = {}", pipeline.execute(10));
    assert_eq!(pipeline.execute(10), 19);  // (10+1)*2-3 = 19

    // Output:
    // Pipeline(5) = 9
    // Pipeline(10) = 19
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_pipeline() {
        let p = Pipeline::new();
        assert_eq!(p.execute(42), 42);
    }

    #[test]
    fn test_single_step() {
        let p = Pipeline::new().add_step(|x| x * 10);
        assert_eq!(p.execute(5), 50);
    }

    #[test]
    fn test_multi_steps() {
        let p = Pipeline::new()
            .add_step(|x| x + 1)
            .add_step(|x| x * 2)
            .add_step(|x| x - 3);
        assert_eq!(p.execute(5), 9);
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `closure may outlive the current function` | Closure capture reference nhưng function muốn return nó | Dùng `move` để lấy ownership |
| `expected Fn, found FnMut` | Closure sửa biến nhưng API yêu cầu `Fn` | Refactor: tránh mutation trong closure, hoặc đổi API thành `FnMut` |
| `cannot move out of captured variable` | Closure dùng `FnOnce` nhưng bạn gọi nhiều lần | Giảm xuống thành `Fn` (đừng consume) hoặc clone data |
| `expected fn pointer, found closure` | Closure capture environment nhưng API cần `fn` pointer | Dùng generic `F: Fn(..)` thay vì `fn(..)` |
| `value used after move` | `move` closure đã lấy ownership | Clone trước khi move, hoặc redesign |

---

## Tóm tắt

- ✅ **Functions**: expressions (không `;` = return), early `return`, multiple return qua tuple.
- ✅ **Closures** = functions + environment. Rust tự chọn capture: `&T` → `&mut T` → move. `move` ép lấy ownership.
- ✅ **`Fn`/`FnMut`/`FnOnce`**: ba "cấp độ" closure. Bắt đầu từ `Fn`, nới lỏng khi cần.
- ✅ **First-class functions**: gán vào biến, truyền qua function, trả từ function. `impl Fn(..) -> ..` cho return type.
- ✅ **HOFs + Pipeline**: `.map().filter().fold()` là HOFs. `compose()` và `Pipeline` struct cho custom pipelines.

## Tiếp theo

→ Chapter 8: **Data Structures** — bạn sẽ học sâu về `Vec`, `String` vs `&str`, `HashMap`, `HashSet`, slices, và iterator chains — toolbox chính của Rust developer.
