# Chapter 16 — Traits: Rust's Typeclass System

> **Bạn sẽ học được**:
> - Trait declaration — định nghĩa behavior chung
> - Trait implementation — gắn behavior cho types
> - Default methods — methods có sẵn, override được
> - Trait bounds `T: Display + Clone` — ràng buộc generics
> - `impl Trait` vs `dyn Trait` — static vs dynamic dispatch
> - Derive macros — tự động implement traits
> - Orphan rules — giới hạn khi impl traits
>
> **Yêu cầu trước**: Chapter 14 (Structs, Enums), Chapter 7 (Closures).
> **Thời gian đọc**: ~45 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Bạn thiết kế abstractions bằng traits — thay thế interfaces, abstract classes, và typeclasses từ các ngôn ngữ khác.

---

## Traits trong bức tranh lớn

Nếu enums (Ch14) quyết định data có thể có **hình dạng** gì, và pattern matching (Ch15) quyết định bạn xử lý mỗi hình dạng thế nào, thì traits quyết định types có thể **làm gì**. Traits là lời hứa về behavior — "type này biết cách hiển thị chính mình", "type này có thể so sánh với type khác", "type này biết cách serialize thành JSON".

Trong Java, bạn có interfaces. Trong Haskell, typeclasses. Trong Go, implicit interfaces. Rust's traits kết hợp ưu điểm của cả ba: explicit như Java interfaces (bạn phải declare `impl Trait for Type`), powerful như Haskell typeclasses (default methods, associated types), và zero-cost như Go (static dispatch qua monomorphization).

Đây không chỉ là cách tổ chức code — đây là cách bạn xây dựng abstractions. Khi bạn viết `fn process(item: &impl Summarize)`, bạn nói "function này làm việc với **bất kỳ type nào** biết cách tóm tắt chính mình". Đó là polymorphism — nhưng không cần inheritance, không cần virtual methods, không cần runtime overhead (trừ khi bạn chọn dùng `dyn Trait`).

---

## 16.1 — Trait là gì?

### Ẩn dụ: "Hợp đồng kỹ năng"

Trait giống **chứng chỉ nghề nghiệp**. Bất kỳ ai (type) có chứng chỉ "Lái xe" (`Drive`) đều **cam kết** biết `start()`, `accelerate()`, `brake()`. Không quan tâm bạn lái xe máy hay ô tô — miễn có chứng chỉ.

```rust
// filename: src/main.rs

// Trait = "chứng chỉ" — khai báo behavior
trait Summarize {
    fn summary(&self) -> String;
}

// Bất kỳ type nào cũng "thi lấy chứng chỉ" (implement trait)
struct Article {
    title: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    text: String,
    likes: u32,
}

// Article impl Summarize
impl Summarize for Article {
    fn summary(&self) -> String {
        format!("📰 {} by {} — {}...", self.title, self.author, &self.content[..50.min(self.content.len())])
    }
}

// Tweet impl Summarize
impl Summarize for Tweet {
    fn summary(&self) -> String {
        format!("🐦 @{}: {} ({}❤️)", self.username, self.text, self.likes)
    }
}

fn main() {
    let article = Article {
        title: "Rust is awesome".into(),
        author: "Minh".into(),
        content: "Rust provides memory safety without garbage collection...".into(),
    };

    let tweet = Tweet {
        username: "rustlang".into(),
        text: "Rust 2024 roadmap released!".into(),
        likes: 5000,
    };

    // Cả hai đều có .summary() — cùng "chứng chỉ"!
    println!("{}", article.summary());
    println!("{}", tweet.summary());
}
```

Nhìn kỹ: `Article` và `Tweet` hoàn toàn khác nhau về cấu trúc — `Article` có `title`, `author`, `content`; `Tweet` có `username`, `text`, `likes`. Nhưng cả hai đều implement `Summarize`, nên bất kỳ function nào nhận `&impl Summarize` đều xử lý được cả hai. Đây là sức mạnh của traits: abstractions không phụ thuộc vào cấu trúc bên trong, mà phụ thuộc vào **behavior bên ngoài**.

So sánh với OOP inheritance: trong Java, `Article` và `Tweet` phải extend cùng base class `Summarizable` để chia sẻ interface. Nhưng nếu `Tweet` đã extend `SocialPost`? Java chỉ cho single inheritance. Trong Rust, một type có thể implement **bao nhiêu traits cũng được** — không bị giới hạn bởi class hierarchy.

---

## 16.2 — Default Methods

Default methods giải quyết bài toán "80% types dùng chung implementation, 20% cần customize". Thay vì bắt mọi type tự viết, trait cung cấp implementation mặc định — types chỉ override khi cần khác biệt:

```rust
// filename: src/main.rs

trait Printable {
    // Method BẮT BUỘC implement
    fn content(&self) -> String;

    // Methods MẶC ĐỊNH — có sẵn, override được
    fn print(&self) {
        println!("{}", self.content());
    }

    fn print_bordered(&self) {
        let content = self.content();
        let border = "─".repeat(content.len().min(50) + 4);
        println!("┌{}┐", border);
        println!("│  {}  │", content);
        println!("└{}┘", border);
    }

    fn is_empty(&self) -> bool {
        self.content().is_empty()
    }
}

struct Note {
    text: String,
}

impl Printable for Note {
    // Chỉ implement content() — print() và print_bordered() dùng mặc định
    fn content(&self) -> String {
        self.text.clone()
    }
}

struct ImportantNote {
    text: String,
}

impl Printable for ImportantNote {
    fn content(&self) -> String {
        self.text.clone()
    }

    // Override default method
    fn print(&self) {
        println!("⚠️ IMPORTANT: {}", self.content());
    }
}

fn main() {
    let note = Note { text: "Remember to buy milk".into() };
    note.print();            // dùng default
    note.print_bordered();   // dùng default

    let urgent = ImportantNote { text: "Server is down!".into() };
    urgent.print();          // dùng override
    urgent.print_bordered(); // dùng default (không override)
}
```

---

## ✅ Checkpoint 16.2

> Ghi nhớ:
> 1. **Trait** = "chứng chỉ" behavior. Types impl trait = cam kết có methods đó
> 2. **Default methods** = implementation sẵn. Override nếu cần
> 3. Chỉ method **không có body** mới bắt buộc implement
>
> **Test nhanh**: Trait có 3 methods, 2 có default body. Type cần impl bao nhiêu methods?
> <details><summary>Đáp án</summary>1 method (chỉ method không có body). 2 default methods dùng được ngay.</details>

---

## 16.3 — Trait Bounds: "Yêu cầu chứng chỉ"

Trait bounds là cách bạn nói với compiler: "function này chỉ nhận types có chứng chỉ X". Đây là generic programming — viết code một lần, hoạt động với nhiều types, nhưng vẫn đảm bảo type safety tại compile time.

Nếu bạn từ OOP: trait bounds tương đương `<T extends Comparable<T>>` trong Java, nhưng mạnh hơn vì Rust cho phép kết hợp nhiều bounds (`T: Display + Clone + Debug`) và dùng `where` clause để giữ code dễ đọc.

### Functions nhận "bất kỳ type nào có trait X"

```rust
// filename: src/main.rs
use std::fmt;

trait Describable {
    fn describe(&self) -> String;
}

// Trait bound: T phải implement Describable
fn print_description<T: Describable>(item: &T) {
    println!("→ {}", item.describe());
}

// Multiple bounds: T phải implement CẢ Describable VÀ Clone
fn clone_and_describe<T: Describable + Clone>(item: &T) -> String {
    let cloned = item.clone();
    cloned.describe()
}

// Where clause — dễ đọc hơn khi nhiều bounds
fn process<T>(item: &T) -> String
where
    T: Describable + Clone + fmt::Debug,
{
    format!("{:?} — {}", item, item.describe())
}

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: u32,
}

impl Describable for Product {
    fn describe(&self) -> String {
        format!("{}: {}đ", self.name, self.price)
    }
}

fn main() {
    let laptop = Product { name: "Laptop".into(), price: 25_000_000 };
    print_description(&laptop);
    println!("{}", clone_and_describe(&laptop));
    println!("{}", process(&laptop));
}
```

### `impl Trait` — Cú pháp ngắn gọn

```rust
// filename: src/main.rs
use std::fmt;

// Tương đương print_description<T: Display>(item: &T)
fn print_item(item: &impl fmt::Display) {
    println!("Item: {}", item);
}

// Return impl Trait — ẩn concrete type
fn make_greeting(name: &str) -> impl fmt::Display {
    format!("Hello, {}! 👋", name)
}

fn main() {
    print_item(&42);
    print_item(&"hello");
    print_item(&3.14);

    let greeting = make_greeting("Rust");
    println!("{}", greeting);
}
```

---

## 16.4 — `dyn Trait`: Dynamic Dispatch

Đến đây bạn có một câu hỏi quan trọng: nếu `impl Trait` tạo code riêng cho **mỗi type**, điều gì xảy ra khi bạn muốn lưu **nhiều types khác nhau** trong cùng một `Vec`? `Vec<impl Shape>` không hoạt động — vì mỗi phần tử phải cùng type.

Đây là lúc `dyn Trait` xuất hiện. Thay vì compiler biết concrete type lúc compile (static dispatch), `dyn Trait` dùng vtable (bảng function pointers) để tìm method lúc runtime (dynamic dispatch). Chậm hơn một chút (~1-2 nanoseconds per call), nhưng cho phép linh hoạt mà static dispatch không có.

Khi bạn dùng `impl Trait`, compiler tạo code riêng cho mỗi type (monomorphization — nhanh, nhưng binary lớn). Khi bạn cần lưu **nhiều types khác nhau** trong cùng collection (Vec, HashMap), dùng `dyn Trait` + `Box` — chậm hơn một chút (vtable lookup), nhưng linh hoạt.

### Static vs Dynamic dispatch

```rust
// filename: src/main.rs

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle { radius: f64 }
struct Rectangle { w: f64, h: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn name(&self) -> &str { "Circle" }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.w * self.h }
    fn name(&self) -> &str { "Rectangle" }
}

// Static dispatch: compiler tạo bản riêng cho mỗi type
// Nhanh (inline) nhưng code size lớn nếu nhiều types
fn print_area_static(shape: &impl Shape) {
    println!("{}: {:.2}", shape.name(), shape.area());
}

// Dynamic dispatch: dùng vtable (bảng function pointers) lúc runtime
// Linh hoạt, lưu được trong collections, nhưng có overhead nhỏ
fn print_area_dynamic(shape: &dyn Shape) {
    println!("{}: {:.2}", shape.name(), shape.area());
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { w: 3.0, h: 4.0 };

    // Static dispatch
    print_area_static(&c);
    print_area_static(&r);

    // Dynamic dispatch — cho phép lưu trong VEC!
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 10.0 }),
        Box::new(Rectangle { w: 5.0, h: 8.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];

    println!("\nAll shapes:");
    for shape in &shapes {
        print_area_dynamic(shape.as_ref());
    }

    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("Total area: {:.2}", total);
}
```

### Khi nào dùng gì?

| | `impl Trait` (Static) | `dyn Trait` (Dynamic) |
|---|---|---|
| Performance | Nhanh hơn (inline, no vtable) | Có vtable overhead |
| Lưu trong Vec? | ❌ Khác type = khác size | ✅ `Vec<Box<dyn T>>` |
| Return từ function? | ✅ 1 concrete type | ✅ Nhiều types khác nhau |
| Khi nào dùng? | Biết type lúc compile | Collections heterogeneous, plugins |

---

## 16.5 — Standard Traits & Derive

Đây là phần thực dụng nhất của chapter — bộ traits mà bạn sẽ dùng **hàng ngày**.

Rust có một bộ traits chuẩn mà hầu hết types nên implement: `Debug` (in debug), `Clone` (sao chép), `PartialEq` (so sánh), `Display` (hiển thị). Thay vì viết tay, `#[derive(...)]` tự động sinh code cho bạn.

### Derive macros — Auto-implement

```rust
// filename: src/main.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProductId(u64);

#[derive(Debug, Clone, PartialEq)]
struct Product {
    id: ProductId,
    name: String,
    price: u32,
}

fn main() {
    let a = Product { id: ProductId(1), name: "Coffee".into(), price: 35_000 };
    let b = a.clone();  // Clone

    println!("{:?}", a);        // Debug
    println!("Equal: {}", a == b);  // PartialEq

    // Hash cho phép dùng trong HashSet/HashMap
    use std::collections::HashSet;
    let mut ids = HashSet::new();
    ids.insert(ProductId(1));
    ids.insert(ProductId(2));
    ids.insert(ProductId(1));  // duplicate → bỏ qua
    println!("Unique IDs: {}", ids.len());  // 2
}
```

### Bảng Standard Traits quan trọng

| Trait | Derive? | Mục đích | Ví dụ |
|-------|---------|----------|-------|
| `Debug` | ✅ | `{:?}` formatting | `println!("{:?}", x)` |
| `Clone` | ✅ | Deep copy | `let b = a.clone()` |
| `Copy` | ✅ | Implicit copy (stack types) | `let b = a` (không move) |
| `PartialEq` | ✅ | So sánh `==` `!=` | `a == b` |
| `Eq` | ✅ | Reflexive equality | Required cho `HashMap` key |
| `Hash` | ✅ | Hashing | `HashMap`, `HashSet` key |
| `PartialOrd` | ✅ | So sánh `<` `>` | `a < b` |
| `Ord` | ✅ | Total ordering | `.sort()` |
| `Default` | ✅ | Giá trị mặc định | `T::default()` |
| `Display` | ❌ | `{}` formatting đẹp | Phải impl tay |
| `From`/`Into` | ❌ | Type conversion | `String::from("hi")` |
| `Iterator` | ❌ | Iteration | Phải impl tay |

### Implement Display

```rust
// filename: src/main.rs
use std::fmt;

#[derive(Debug, Clone)]
struct Money {
    amount: u64,
    currency: String,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.currency.as_str() {
            "VND" => write!(f, "{}đ", self.amount),
            "USD" => write!(f, "${:.2}", self.amount as f64 / 100.0),
            _ => write!(f, "{} {}", self.amount, self.currency),
        }
    }
}

impl Default for Money {
    fn default() -> Self {
        Money { amount: 0, currency: "VND".to_string() }
    }
}

fn main() {
    let price = Money { amount: 35_000, currency: "VND".into() };
    println!("Price: {}", price);    // Display: 35000đ
    println!("Debug: {:?}", price);  // Debug: Money { amount: 35000, ... }

    let zero = Money::default();
    println!("Default: {}", zero);   // 0đ
}
```

### `From` / `Into` — Type conversion

```rust
// filename: src/main.rs

#[derive(Debug)]
struct Email(String);

impl From<&str> for Email {
    fn from(s: &str) -> Self {
        Email(s.to_lowercase())
    }
}

impl From<String> for Email {
    fn from(s: String) -> Self {
        Email(s.to_lowercase())
    }
}

fn send_notification(to: impl Into<Email>, message: &str) {
    let email: Email = to.into();  // auto convert
    println!("📧 To {:?}: {}", email, message);
}

fn main() {
    // Cả &str và String đều convert được!
    send_notification("ADMIN@Company.Com", "Server restarted");
    send_notification(String::from("User@Domain.Org"), "Welcome!");
}
```

---

## 16.6 — Orphan Rule & Newtype workaround

### Orphan Rule

Orphan rule là giới hạn quan trọng mà bạn sẽ gặp sớm hay muộn — và hiểu nó giúp bạn thiết kế library boundaries tốt hơn.

Bạn **chỉ có thể** impl trait cho type khi **ít nhất 1** trong 2 (trait hoặc type) thuộc crate của bạn:

```rust
// ✅ OK: your trait + external type
trait MyTrait { fn foo(&self); }
impl MyTrait for String { fn foo(&self) { println!("{}", self); } }

// ✅ OK: external trait + your type
struct MyType(i32);
impl std::fmt::Display for MyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyType({})", self.0)
    }
}

// ❌ NOT OK: external trait + external type
// impl std::fmt::Display for Vec<i32> { ... }  // orphan rule violation!
```

### Workaround: Newtype

```rust
// filename: src/main.rs
use std::fmt;

// Wrap Vec trong newtype → bây giờ là "type của bạn"
struct PrettyList(Vec<String>);

impl fmt::Display for PrettyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

fn main() {
    let list = PrettyList(vec!["Rust".into(), "Go".into(), "Zig".into()]);
    println!("Languages: {}", list);
    // Languages: Rust, Go, Zig
}
```

---

## 16.7 — Advanced Trait Patterns

Phần này dành cho những ai muốn đi sâu hơn — bạn có thể quay lại đọc sau khi đã quen với traits cơ bản.

3 kỹ thuật nâng cao: `impl Trait` ở return position (ẩn concrete type), trait objects trong API (linh hoạt), và extension traits (thêm methods cho types người khác viết).

### Return Position `impl Trait` (RPIT)

```rust
// filename: src/main.rs

// Return impl Trait — ẩn concrete type, compiler infer
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n  // closure — type phức tạp, nhưng caller chỉ thấy "Fn(i32)->i32"
}

// Return impl Iterator — rất phổ biến
fn fibonacci() -> impl Iterator<Item = u64> {
    let mut state = (0u64, 1u64);
    std::iter::from_fn(move || {
        let next = state.0;
        state = (state.1, state.0 + state.1);
        Some(next)
    })
}

fn main() {
    let add5 = make_adder(5);
    println!("{}", add5(10));  // 15

    let fibs: Vec<u64> = fibonacci().take(10).collect();
    println!("{:?}", fibs);  // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
}
```

### `impl Trait` trong async

```rust
// Hai cách viết async function — tương đương:
// Cách 1: async fn (syntactic sugar)
// async fn fetch_data(url: &str) -> Result<String, Error> { ... }

// Cách 2: return impl Future (explicit)
// fn fetch_data(url: &str) -> impl Future<Output = Result<String, Error>> { ... }

// Cách 2 hữu ích khi:
// - Cần conditional futures
// - Cần thêm bounds: + Send (for multi-threaded)
// - Dùng trong traits (trước Rust 1.75)
```

### Object Safety — khi nào `dyn Trait` hoạt động?

```rust
// ✅ Object-safe trait: CÓ THỂ dùng dyn
trait Drawable {
    fn draw(&self) -> String;  // OK: &self, trả String
    fn area(&self) -> f64;     // OK: &self, trả f64
}

// ❌ NOT object-safe: KHÔNG dùng dyn được
trait Clonable {
    fn clone_self(&self) -> Self;  // ❌ trả Self (unsized)
}

trait Generic {
    fn process<T>(&self, item: T);  // ❌ generic method
}
```

**Object safety rules** — trait phải thỏa TẤT CẢ:
1. Methods không trả `Self` (trừ khi `Self: Sized`)
2. Methods không có generic type parameters
3. Trait không require `Self: Sized`

| Pattern | Object-safe? | Giải pháp nếu không |
|---------|-------------|---------------------|
| `fn method(&self) -> String` | ✅ | — |
| `fn clone(&self) -> Self` | ❌ | Thêm `where Self: Sized` hoặc return `Box<dyn Trait>` |
| `fn process<T>(&self, x: T)` | ❌ | Dùng `&dyn Any`, hoặc tách thành trait riêng |
| `fn new() -> Self` | ❌ | Move ra factory function, hoặc `where Self: Sized` |

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Derive đúng traits

Struct sau cần `derive` gì để: (a) dùng trong `HashMap` key, (b) sort được, (c) dùng `println!("{}", x)`?

```rust
struct UserId(u64);
```

<details><summary>✅ Lời giải Bài 1</summary>

```rust
// (a) HashMap key: cần Hash + Eq + PartialEq
// (b) sort: cần Ord + PartialOrd + Eq + PartialEq
// (c) println!("{}") = Display → phải impl tay

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct UserId(u64);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User#{}", self.0)
    }
}
```

</details>

---

**Bài 2** (10 phút): `Drawable` trait

Viết trait `Drawable` với: `fn draw(&self) -> String`, default `fn bounding_box(&self) -> (f64, f64)` trả `(0.0, 0.0)`. Implement cho `Circle` và `Rect`. Lưu vào `Vec<Box<dyn Drawable>>` và vẽ tất cả.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
trait Drawable {
    fn draw(&self) -> String;
    fn bounding_box(&self) -> (f64, f64) { (0.0, 0.0) }
}

struct Circle { x: f64, y: f64, radius: f64 }
struct Rect { x: f64, y: f64, w: f64, h: f64 }

impl Drawable for Circle {
    fn draw(&self) -> String { format!("⭕ at ({:.0},{:.0}) r={:.0}", self.x, self.y, self.radius) }
    fn bounding_box(&self) -> (f64, f64) { (self.radius * 2.0, self.radius * 2.0) }
}

impl Drawable for Rect {
    fn draw(&self) -> String { format!("🟦 at ({:.0},{:.0}) {:.0}×{:.0}", self.x, self.y, self.w, self.h) }
    fn bounding_box(&self) -> (f64, f64) { (self.w, self.h) }
}

fn main() {
    let canvas: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { x: 10.0, y: 20.0, radius: 5.0 }),
        Box::new(Rect { x: 0.0, y: 0.0, w: 100.0, h: 50.0 }),
        Box::new(Circle { x: 50.0, y: 50.0, radius: 15.0 }),
    ];
    for shape in &canvas {
        let (w, h) = shape.bounding_box();
        println!("{} [bbox: {:.0}×{:.0}]", shape.draw(), w, h);
    }
}
```

</details>

---

**Bài 3** (15 phút): Plugin system

Thiết kế trait `Plugin` với methods: `name(&self) -> &str`, `version(&self) -> &str`, `execute(&self, input: &str) -> Result<String, String>`. Tạo 3 plugins: `UpperPlugin`, `ReversePlugin`, `CountPlugin`. Viết `PluginRunner` lưu `Vec<Box<dyn Plugin>>` và chạy tất cả plugins trên input.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, input: &str) -> Result<String, String>;
}

struct UpperPlugin;
impl Plugin for UpperPlugin {
    fn name(&self) -> &str { "uppercase" }
    fn version(&self) -> &str { "1.0" }
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(input.to_uppercase())
    }
}

struct ReversePlugin;
impl Plugin for ReversePlugin {
    fn name(&self) -> &str { "reverse" }
    fn version(&self) -> &str { "1.0" }
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(input.chars().rev().collect())
    }
}

struct CountPlugin;
impl Plugin for CountPlugin {
    fn name(&self) -> &str { "word-count" }
    fn version(&self) -> &str { "1.0" }
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(format!("{} words", input.split_whitespace().count()))
    }
}

struct PluginRunner {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRunner {
    fn new() -> Self { PluginRunner { plugins: vec![] } }

    fn register(mut self, plugin: Box<dyn Plugin>) -> Self {
        self.plugins.push(plugin);
        self
    }

    fn run_all(&self, input: &str) {
        println!("Input: '{}'\n", input);
        for plugin in &self.plugins {
            match plugin.execute(input) {
                Ok(result) => println!("  [{}@{}] → {}", plugin.name(), plugin.version(), result),
                Err(e) => println!("  [{}] ❌ {}", plugin.name(), e),
            }
        }
    }
}

fn main() {
    let runner = PluginRunner::new()
        .register(Box::new(UpperPlugin))
        .register(Box::new(ReversePlugin))
        .register(Box::new(CountPlugin));

    runner.run_all("Hello Rust World");
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `the trait X is not implemented for Y` | Type chưa impl trait | Thêm `impl X for Y { ... }` hoặc `#[derive(X)]` |
| `the size for values of type dyn X cannot be known` | `dyn Trait` unsized | Dùng `Box<dyn X>` hoặc `&dyn X` |
| `cannot impl external trait for external type` | Orphan rule | Wrap trong newtype |
| `conflicting implementations` | 2 impl cùng trait cho cùng type | Chỉ giữ 1, hoặc dùng blanket impl + specialization (nightly) |
| `the trait X is not object-safe` | Trait có generic method hoặc return Self | Dùng `impl Trait` thay `dyn Trait`, hoặc refactor trait |

---

## Tóm tắt

- ✅ **Trait** = "chứng chỉ" behavior. Khai báo methods, types implement chúng.
- ✅ **Default methods** = có sẵn, override được. Chỉ methods không body mới bắt buộc.
- ✅ **Trait bounds** `T: Display + Clone` = ràng buộc generics. `where` clause cho dễ đọc.
- ✅ **Static** (`impl Trait`) = nhanh, monomorphized. **Dynamic** (`dyn Trait`) = linh hoạt, `Vec<Box<dyn T>>`.
- ✅ **Standard traits**: `Debug`, `Clone`, `PartialEq`, `Hash` derive được. `Display`, `From` impl tay.
- ✅ **Orphan rule**: trait hoặc type phải thuộc crate bạn. Newtype workaround.

## Tiếp theo

→ Chapter 17: **Generics & Trait Bounds** — bạn sẽ đi sâu vào generic programming: generic structs/enums, associated types, monomorphization, và cách Rust đạt zero-cost abstractions.
