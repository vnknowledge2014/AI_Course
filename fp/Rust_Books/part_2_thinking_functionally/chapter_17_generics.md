# Chapter 17 — Generics & Trait Bounds

> **Bạn sẽ học được**:
> - Generic functions, structs, enums — viết code cho **mọi type**
> - Trait bounds nâng cao — `where` clauses phức tạp, multiple bounds
> - Associated types vs generic params — khi nào dùng cái nào
> - Monomorphization — Rust tạo code riêng cho mỗi type, zero-cost
> - Blanket implementations — impl trait cho "mọi T thỏa điều kiện"
> - Generic patterns thực tế: Repository, Converter, Validator
>
> **Yêu cầu trước**: Chapter 16 (Traits).
> **Thời gian đọc**: ~40 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Bạn viết **generic, reusable abstractions** — code linh hoạt mà không mất performance.

---

## 17.1 — Generic Functions: "Viết một lần, chạy mọi type"

### Khuôn bánh vạn năng

Hãy nghĩ về khuôn bánh cookie cutter. Một khuôn hình ngôi sao có thể cắt bột mì, bột gạo, hay socola — bất kỳ loại bột nào. Khuôn không quan tâm **đó là bột gì** — nó chỉ cần bột "cắt được" (đủ dẻỏ, không quá ướt). Đó là generic: viết code một lần, dùng với mọi type thỏa điều kiện.

`fn max<T: PartialOrd>(a: T, b: T) -> T` là khuôn bánh: nó hoạt động với `i32`, `f64`, `String`, hay bất kỳ type nào "so sánh được" (`PartialOrd`). Không cần viết `max_i32`, `max_f64`, `max_string` riêng.

### Tại sao cần generics?

```rust
// filename: src/main.rs

// ❌ BÀI 1: Viết function cho TỪNG type? Duplicate!
fn max_i32(a: i32, b: i32) -> i32 { if a >= b { a } else { b } }
fn max_f64(a: f64, b: f64) -> f64 { if a >= b { a } else { b } }
fn max_str<'a>(a: &'a str, b: &'a str) -> &'a str { if a >= b { a } else { b } }

// ✅ Generic: viết 1 lần, chạy cho MỌI type có PartialOrd
fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

fn main() {
    println!("max(3, 5) = {}", max_of(3, 5));
    println!("max(3.14, 2.71) = {}", max_of(3.14, 2.71));
    println!("max(\"apple\", \"banana\") = {}", max_of("apple", "banana"));
}
```

### Multiple type parameters

```rust
// filename: src/main.rs
use std::fmt;

// Hai type parameters
fn print_pair<A: fmt::Display, B: fmt::Debug>(first: A, second: B) {
    println!("({}, {:?})", first, second);
}

// Where clause cho dễ đọc
fn convert_and_display<T, U>(input: T) -> String
where
    T: Into<U>,
    U: fmt::Display,
{
    let converted: U = input.into();
    format!("{}", converted)
}

fn main() {
    print_pair(42, vec![1, 2, 3]);       // (42, [1, 2, 3])
    print_pair("hello", (true, 3.14));   // (hello, (true, 3.14))
}
```

---

## 17.2 — Generic Structs & Enums

Generic functions hoạt động trên mọi type, nhưng bạn cũng cần **lưu trữ** giá trị generic. `Pair<T>`, `Stack<T>`, `Cache<K, V>` — đây là containers viết một lần, dùng với mọi type.

### Generic struct

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
}

// Methods chỉ cho Pair<T> khi T: PartialOrd
impl<T: PartialOrd> Pair<T> {
    fn max(&self) -> &T {
        if self.first >= self.second { &self.first } else { &self.second }
    }
}

// Methods chỉ cho Pair<T> khi T: Display
impl<T: std::fmt::Display> Pair<T> {
    fn display(&self) -> String {
        format!("({}, {})", self.first, self.second)
    }
}

fn main() {
    let numbers = Pair::new(10, 20);
    println!("Max: {}", numbers.max());      // 20
    println!("Display: {}", numbers.display()); // (10, 20)

    let names = Pair::new("Alice", "Bob");
    println!("Max: {}", names.max());        // Bob
}
```

### Generic enum — thực tế

```rust
// filename: src/main.rs

// Cache: giá trị có thể đã compute hoặc chưa
#[derive(Debug)]
enum Cached<T> {
    Pending,
    Computing,
    Ready(T),
    Error(String),
}

impl<T: Clone> Cached<T> {
    fn get(&self) -> Option<&T> {
        match self {
            Cached::Ready(value) => Some(value),
            _ => None,
        }
    }

    fn get_or_compute<F: FnOnce() -> Result<T, String>>(self, compute: F) -> Cached<T> {
        match self {
            Cached::Ready(v) => Cached::Ready(v),
            _ => match compute() {
                Ok(v) => Cached::Ready(v),
                Err(e) => Cached::Error(e),
            }
        }
    }

    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Cached<U> {
        match self {
            Cached::Ready(v) => Cached::Ready(f(v)),
            Cached::Error(e) => Cached::Error(e),
            Cached::Pending => Cached::Pending,
            Cached::Computing => Cached::Computing,
        }
    }
}

fn main() {
    let mut data: Cached<Vec<i32>> = Cached::Pending;
    println!("1. {:?}", data);

    data = data.get_or_compute(|| {
        println!("  Computing...");
        Ok(vec![1, 2, 3, 4, 5])
    });
    println!("2. {:?}", data);
    println!("3. Value: {:?}", data.get());

    // map: transform bên trong
    let lengths = Cached::Ready(vec!["hello", "world"]).map(|v| v.len());
    println!("4. {:?}", lengths);  // Ready(2)
}
```

---

## ✅ Checkpoint 17.2

> Ghi nhớ:
> 1. `struct Pair<T>` = generic over T. `impl<T>` cho mọi T, `impl<T: Bound>` cho T thỏa bound.
> 2. **Conditional methods**: chỉ tồn tại khi T thỏa trait bound. `max()` chỉ Pair có PartialOrd.
> 3. Generic enums: `Option<T>`, `Result<T, E>`, custom `Cached<T>` — mô hình data linh hoạt.
>
> **Test nhanh**: `Pair<String>` có method `max()` không?
> <details><summary>Đáp án</summary>Có! <code>String</code> implements <code>PartialOrd</code>, nên <code>max()</code> available.</details>

---

## 17.3 — Associated Types vs Generic Params

### Khi nào dùng Associated Type?

```rust
// filename: src/main.rs

// GENERIC param: Iterator "có thể" trả nhiều types khác nhau
// trait Iterator<Item> { fn next(&mut self) -> Option<Item>; }
// → MyIter có thể impl Iterator<i32> VÀ Iterator<String>? Confusing!

// ASSOCIATED type: Iterator trả ĐÚNG 1 type
// trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }
// → MyIter chỉ impl Iterator MỘT LẦN, với 1 Item type.

// Ví dụ: Converter pattern
trait Converter {
    type Input;
    type Output;
    type Error;

    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

// String → u32 converter
struct StringToU32;

impl Converter for StringToU32 {
    type Input = String;
    type Output = u32;
    type Error = String;

    fn convert(&self, input: String) -> Result<u32, String> {
        input.trim().parse::<u32>()
            .map_err(|e| format!("Parse error: {}", e))
    }
}

// Celsius → Fahrenheit converter
struct CelsiusToFahrenheit;

impl Converter for CelsiusToFahrenheit {
    type Input = f64;
    type Output = f64;
    type Error = String;

    fn convert(&self, celsius: f64) -> Result<f64, String> {
        if celsius < -273.15 {
            Err("Below absolute zero".into())
        } else {
            Ok(celsius * 9.0 / 5.0 + 32.0)
        }
    }
}

fn main() {
    let parser = StringToU32;
    println!("{:?}", parser.convert("42".into()));      // Ok(42)
    println!("{:?}", parser.convert("abc".into()));     // Err(...)

    let temp = CelsiusToFahrenheit;
    println!("{:?}", temp.convert(100.0));  // Ok(212.0)
    println!("{:?}", temp.convert(-300.0)); // Err(...)
}
```

### Quy tắc chọn

| | Associated Type | Generic Param |
|---|---|---|
| Impl bao nhiêu lần? | **1 lần** per type | **Nhiều lần** per type |
| Ví dụ | `Iterator { type Item }` | `From<T>` (impl nhiều T) |
| Khi nào | "Type này có 1 output type" | "Type này convert từ nhiều nguồn" |

---

## 17.4 — Monomorphization: Zero-cost Abstractions

### Rust tạo code riêng cho mỗi type

```rust
// filename: src/main.rs

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let x = add(1_i32, 2);    // compiler tạo: fn add_i32(a: i32, b: i32) -> i32
    let y = add(1.0_f64, 2.0); // compiler tạo: fn add_f64(a: f64, b: f64) -> f64
    println!("{} {}", x, y);
}

// Sau compilation, code GIỐNG NHƯ bạn viết tay từng function!
// Không có vtable, không có boxing, không có runtime dispatch.
// → ZERO COST
```

> **💡 Monomorphization** = compiler "stamp out" (đóng dấu) bản riêng cho mỗi concrete type. Generic code compile thành specialized code — nhanh như viết tay.

### Trade-off

```
Monomorphization (static dispatch):
  ✅ Performance = hand-written code
  ✅ Compiler optimize từng specialization
  ❌ Binary size lớn hơn (nhiều bản)
  ❌ Compile time lâu hơn

Dynamic dispatch (dyn Trait):
  ✅ Binary size nhỏ (1 bản code + vtable)
  ✅ Compile nhanh hơn
  ❌ Có vtable overhead (thường <5%)
  ❌ Không inline được
```

---

## 17.5 — Blanket Implementations

### Impl trait cho "mọi T thỏa điều kiện"

```rust
// filename: src/main.rs
use std::fmt;

// Trait của chúng ta
trait Loggable {
    fn log(&self);
}

// Blanket impl: MỌI type có Display → tự động có Loggable
impl<T: fmt::Display> Loggable for T {
    fn log(&self) {
        println!("[LOG] {}", self);
    }
}

fn main() {
    42.log();                // i32 impl Display → có Loggable
    "hello".log();           // &str impl Display → có Loggable
    3.14.log();              // f64 impl Display → có Loggable
    String::from("Rust").log(); // String impl Display → có Loggable
}
```

### Standard library ví dụ nổi tiếng

```rust
// Trong std:
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String { ... }
// }
// → Mọi type có Display tự động có .to_string()!

// impl<T> From<T> for T {
//     fn from(t: T) -> T { t }
// }
// → Mọi type convert thành chính nó!

fn main() {
    let s: String = 42.to_string();  // i32: Display → ToString
    println!("{}", s);
}
```

---

## 17.6 — Generic Patterns thực tế

Domain-Driven Design dùng generics ở khắp nơi: Repository<T> cho CRUD, Pipeline<I,O> cho workflows, Validator<T> cho validation. Mỗi pattern viết một lần, áp dụng cho `Product`, `Order`, `User` — không duplicate code.

### Pattern 1: Repository (generic CRUD)

```rust
// filename: src/main.rs
use std::collections::HashMap;

trait Repository<T: Clone> {
    fn get(&self, id: u64) -> Option<T>;
    fn save(&mut self, id: u64, item: T);
    fn delete(&mut self, id: u64) -> bool;
    fn list(&self) -> Vec<T>;
}

// In-memory implementation — hoạt động cho BẤT KỲ type T
struct InMemoryRepo<T> {
    store: HashMap<u64, T>,
}

impl<T> InMemoryRepo<T> {
    fn new() -> Self {
        InMemoryRepo { store: HashMap::new() }
    }
}

impl<T: Clone> Repository<T> for InMemoryRepo<T> {
    fn get(&self, id: u64) -> Option<T> {
        self.store.get(&id).cloned()
    }

    fn save(&mut self, id: u64, item: T) {
        self.store.insert(id, item);
    }

    fn delete(&mut self, id: u64) -> bool {
        self.store.remove(&id).is_some()
    }

    fn list(&self) -> Vec<T> {
        self.store.values().cloned().collect()
    }
}

#[derive(Debug, Clone)]
struct User { name: String, email: String }

#[derive(Debug, Clone)]
struct Product { name: String, price: u32 }

fn main() {
    // Cùng InMemoryRepo — hoạt động cho User VÀ Product
    let mut users: InMemoryRepo<User> = InMemoryRepo::new();
    users.save(1, User { name: "Minh".into(), email: "minh@co.com".into() });
    users.save(2, User { name: "Lan".into(), email: "lan@co.com".into() });

    let mut products: InMemoryRepo<Product> = InMemoryRepo::new();
    products.save(1, Product { name: "Coffee".into(), price: 35_000 });

    println!("Users: {:?}", users.list());
    println!("Products: {:?}", products.list());
    println!("User 1: {:?}", users.get(1));
}
```

### Pattern 2: Validator chain (generic)

```rust
// filename: src/main.rs

type ValidationResult = Result<(), String>;

trait Validator<T> {
    fn validate(&self, value: &T) -> ValidationResult;
}

// Concrete validators
struct NotEmpty;
impl Validator<String> for NotEmpty {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.trim().is_empty() { Err("Must not be empty".into()) } else { Ok(()) }
    }
}

struct MinLength(usize);
impl Validator<String> for MinLength {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.len() < self.0 { Err(format!("Min length: {}", self.0)) } else { Ok(()) }
    }
}

struct InRange { min: i64, max: i64 }
impl Validator<i64> for InRange {
    fn validate(&self, value: &i64) -> ValidationResult {
        if *value >= self.min && *value <= self.max { Ok(()) }
        else { Err(format!("Must be in [{}, {}]", self.min, self.max)) }
    }
}

// Generic validation runner
fn validate_all<T>(value: &T, validators: &[&dyn Validator<T>]) -> Vec<String> {
    validators.iter()
        .filter_map(|v| v.validate(value).err())
        .collect()
}

fn main() {
    let name = String::from("Mi");
    let name_errors = validate_all(&name, &[&NotEmpty, &MinLength(3)]);
    println!("Name '{}': {:?}", name, name_errors);
    // Name 'Mi': ["Min length: 3"]

    let age: i64 = 200;
    let age_errors = validate_all(&age, &[&InRange { min: 0, max: 150 }]);
    println!("Age {}: {:?}", age, age_errors);
    // Age 200: ["Must be in [0, 150]"]

    let valid_name = String::from("Minh Nguyen");
    let no_errors = validate_all(&valid_name, &[&NotEmpty, &MinLength(3)]);
    println!("Name '{}': {:?}", valid_name, no_errors);
    // Name 'Minh Nguyen': []
}
```

---

## 17.7 — Advanced Lifetimes

### `'static` — Sống mãi

```rust
// filename: src/main.rs

// 'static = giá trị sống toàn bộ chương trình
// String literals là 'static:
fn get_greeting() -> &'static str {
    "Hello, world!"  // embedded trong binary → sống mãi
}

// T: 'static KHÔNG có nghĩa "T phải là reference"
// Mà: T KHÔNG chứa non-'static references
// → String: 'static ✅ (owned, no borrows)
// → &'a str: KHÔNG 'static (trừ khi 'a = 'static)
// → Vec<i32>: 'static ✅ (owned)

fn spawn_task<T: Send + 'static>(data: T) {
    // Thread mới cần 'static vì thread có thể sống lâu hơn caller
    std::thread::spawn(move || {
        println!("Task completed");
        drop(data);
    }).join().unwrap();
}

fn main() {
    println!("{}", get_greeting());

    // ✅ String is 'static (owned)
    spawn_task(String::from("hello"));

    // ✅ Vec is 'static (owned)
    spawn_task(vec![1, 2, 3]);

    // ❌ &str (reference) is NOT 'static unless literal
    // let s = String::from("hi");
    // spawn_task(&s);  // error: borrowed value does not live long enough
}
```

### Lifetime bounds trên generics

```rust
// filename: src/main.rs

// T: 'a = "T sống ít nhất bằng 'a"
struct Wrapper<'a, T: 'a> {
    data: &'a T,
}

impl<'a, T: 'a + std::fmt::Display> Wrapper<'a, T> {
    fn show(&self) {
        println!("Wrapped: {}", self.data);
    }
}

fn main() {
    let value = 42;
    let w = Wrapper { data: &value };
    w.show();  // Wrapped: 42
}
```

### Higher-Rank Trait Bounds (HRTB)

```rust
// filename: src/main.rs

// HRTB: "function hoạt động cho MỌI lifetime 'a"
// for<'a> Fn(&'a str) -> &'a str

fn apply_transform<F>(items: &[String], transform: F) -> Vec<String>
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    items.iter().map(|s| transform(s).to_string()).collect()
}

fn main() {
    let items = vec!["  hello  ".into(), "  world  ".into()];
    let trimmed = apply_transform(&items, |s| s.trim());
    println!("{:?}", trimmed);  // ["hello", "world"]
}
```

### Self-referential structs — Tại sao khó?

```rust
// ❌ KHÔNG compile — self-referential struct
// struct SelfRef {
//     data: String,
//     slice: &str,  // muốn trỏ vào data → không thể!
// }
// Nếu struct move, data di chuyển → slice trỏ sai vùng nhớ!

// Giải pháp:
// 1. Tách thành 2 structs riêng
// 2. Dùng index thay reference: lưu (start, end) thay vì &str
// 3. Pin<Box<T>> — pin struct tại chỗ, không cho move
// 4. Crate: `ouroboros`, `self_cell` (helper macros)
```

### Variance — Đề cập ngắn

```
Variance = "khi nào có thể dùng lifetime ngắn hơn / dài hơn?"

- &'a T       → COvariant trên 'a: có thể dùng lifetime DÀI HƠN
- &'a mut T   → INvariant trên 'a: phải EXACT
- fn(T) -> U  → CONTRAvariant trên T

Thực tế: hầu hết bạn không cần nghĩ về variance.
Khi nào cần? Khi lifetime errors khó hiểu với &mut references.
```

> **💡 Ghi nhớ**: `'static` = owned/long-lived, KHÔNG phải "static variable". Lifetime bounds (`T: 'a`) xuất hiện nhiều trong async code. HRTB (`for<'a>`) hiếm gặp nhưng cần biết khi viết higher-order functions nhận references.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Generic function

Viết `fn first_and_last<T: Clone>(items: &[T]) -> Option<(T, T)>` — trả tuple (phần tử đầu, phần tử cuối). Return `None` nếu slice rỗng.

<details><summary>✅ Lời giải Bài 1</summary>

```rust
fn first_and_last<T: Clone>(items: &[T]) -> Option<(T, T)> {
    let first = items.first()?.clone();
    let last = items.last()?.clone();
    Some((first, last))
}

fn main() {
    println!("{:?}", first_and_last(&[1, 2, 3, 4, 5]));  // Some((1, 5))
    println!("{:?}", first_and_last(&["a", "b", "c"]));   // Some(("a", "c"))
    println!("{:?}", first_and_last::<i32>(&[]));          // None
}
```

</details>

---

**Bài 2** (10 phút): Generic Stack

Implement `Stack<T>` với: `push`, `pop -> Option<T>`, `peek -> Option<&T>`, `is_empty`, `len`. Dùng `Vec<T>` bên trong.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Stack { items: vec![] } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
    fn peek(&self) -> Option<&T> { self.items.last() }
    fn is_empty(&self) -> bool { self.items.is_empty() }
    fn len(&self) -> usize { self.items.len() }
}

// Conditional: chỉ Display khi T: Display
impl<T: std::fmt::Display> Stack<T> {
    fn display(&self) -> String {
        let items: Vec<String> = self.items.iter().map(|i| i.to_string()).collect();
        format!("[{}]", items.join(" → "))
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("{}", stack.display());         // [10 → 20 → 30]
    println!("Peek: {:?}", stack.peek());    // Some(30)
    println!("Pop: {:?}", stack.pop());      // Some(30)
    println!("Len: {}", stack.len());        // 2
}
```

</details>

---

**Bài 3** (15 phút): Pipeline builder (generic)

Viết `Pipeline<T>` struct lưu `Vec<Box<dyn Fn(T) -> T>>`. Methods: `add_step(f)` (builder pattern), `execute(input: T) -> T` chạy tất cả steps. Test với `Pipeline<String>` (text processing) và `Pipeline<i32>` (math).

<details><summary>✅ Lời giải Bài 3</summary>

```rust
struct Pipeline<T> {
    steps: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T> Pipeline<T> {
    fn new() -> Self { Pipeline { steps: vec![] } }

    fn add_step<F: Fn(T) -> T + 'static>(mut self, f: F) -> Self {
        self.steps.push(Box::new(f));
        self
    }

    fn execute(&self, input: T) -> T {
        self.steps.iter().fold(input, |acc, step| step(acc))
    }
}

fn main() {
    // String pipeline
    let text_pipe = Pipeline::new()
        .add_step(|s: String| s.trim().to_string())
        .add_step(|s| s.to_lowercase())
        .add_step(|s| s.replace("rust", "Rust 🦀"));

    println!("{}", text_pipe.execute("  HELLO RUST WORLD  ".into()));
    // hello Rust 🦀 world

    // Math pipeline
    let math_pipe = Pipeline::new()
        .add_step(|x: i32| x + 10)
        .add_step(|x| x * 2)
        .add_step(|x| x - 5);

    println!("{}", math_pipe.execute(5));  // (5+10)*2 - 5 = 25
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `the trait bound T: X is not satisfied` | T thiếu trait | Thêm bound: `T: X` |
| `cannot infer type for T` | Compiler không đoán T | Ghi rõ type: `func::<i32>(...)` (turbofish) |
| `conflicting implementations` | 2 blanket impls overlap | Thu hẹp bounds hoặc dùng specialization (nightly) |
| `parameter T is never used` | Phantom type parameter | Dùng `PhantomData<T>` hoặc bỏ param |
| `expected associated type, found generic` | Nhầm lẫn cú pháp | `type Item = X` (associated), `<T>` (generic) |

---

## Tóm tắt

Chapter này dạy bạn làm **khuôn bánh vạn năng** — viết code một lần, dùng với mọi type:

- ✅ **Generic functions/structs**: Khuôn bắt kỳ bột nào. `fn max<T: PartialOrd>` hoạt động với mọi T so sánh được.
- ✅ **Conditional methods**: `impl<T: Display> Pair<T>` — khuôn chỉ hoạt động với bột đủ dẻỏ.
- ✅ **Associated types**: `type Item` — 1 impl per type. Generic params `<T>` — nhiều impls.
- ✅ **Monomorphization**: Compiler "đóng dấu" bản riêng cho mỗi T → nhanh như viết tay.
- ✅ **Blanket impls**: `impl<T: Display> Loggable for T` — khuôn tự động áp cho mọi bột thỏa điều kiện.
- ✅ **Patterns**: Repository `<T: Clone>`, Validator chain, Pipeline builder.

---

## 🎉 Kết thúc Part II!

Hãy nhìn lại hành trình "tư duy hàm":

- **Ch 12**: Nền móng — data bất biến, pure functions, bảng giá khắc trên gỗ
- **Ch 13**: LEGO — closures là viên gạch, composition là cách ghép
- **Ch 14**: Bản thiết kế — struct (AND) và enum (OR), loại bỏ trạng thái không hợp lệ
- **Ch 15**: Máy X-quang — nhìn xuyên data, phân loại chính xác
- **Ch 16**: Chứng chỉ — traits là hợp đồng kỹ năng cho types
- **Ch 17**: Khuôn bánh — generics viết một lần, dùng với mọi type

Bạn giờ có **bộ công cụ tư duy FP hoàn chỉnh** để bước vào Design Patterns và DDD.

## Tiếp theo

→ **Part III: Design Patterns** bắt đầu! Chapter 18: **GoF → FP Translation** — bạn sẽ thấy Strategy, Observer, Command patterns chuyển từ OOP sang FP thế nào. Và Chapter 19: **CQRS & Event Sourcing** — tách read/write models, lưu events thay vì state.
