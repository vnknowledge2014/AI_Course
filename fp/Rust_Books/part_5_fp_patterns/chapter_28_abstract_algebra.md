# Chapter 28 — Abstract Algebra for Rust Developers

> **Bạn sẽ học được**:
> - **Semigroup**: trait cho "combine 2 values cùng type"
> - **Monoid**: Semigroup + empty value
> - Bạn đã dùng chúng hàng ngày: `String`, `Vec`, `Option`
> - **Reduce** vs **Fold** — semigroup vs monoid
> - Domain modeling với algebraic structures
> - Custom Semigroup/Monoid cho business logic
>
> **Yêu cầu trước**: Chapter 13 (HOF), Chapter 16 (Traits), Chapter 17 (Generics).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn nhận ra patterns ẩn trong code hàng ngày — và dùng chúng **có ý thức** để viết code đẹp hơn.

---

## Abstract Algebra — Toán học đằng sau FP

Chapter này có thể khiến bạn sợ vì tên gọi "abstract algebra". Đừng lo — bạn đã dùng abstract algebra mỗi ngày mà không biết. Mỗi lần bạn dùng `vec.iter().sum()`, bạn đang dùng **Monoid**. Mỗi lần dùng `.map()`, bạn đang dùng **Functor**. Mỗi lần chain `.and_then()`, bạn đang dùng **Monad**.

Chapter này đặt tên cho những patterns bạn đã sử dụng, và cho thấy tại sao chúng mạnh: khi bạn biết một structure là Monoid, bạn biết nó có thể `fold`, `concat`, và parallelize — miễn phí, không cần code thêm.

---

## 28.1 — Semigroup: "Kết hợp hai thứ cùng loại"

### Ẩn dụ: Trộn màu

Trộn đỏ + xanh → tím. Trộn output với vàng → output khác. Quan trọng: **input và output CÙNG TYPE** (đều là "màu").

### Định nghĩa

**Semigroup** = type `T` có operation `combine(a: T, b: T) -> T` thỏa **associativity**:
```
combine(combine(a, b), c) == combine(a, combine(b, c))
```

Giống phép cộng: `(1 + 2) + 3 == 1 + (2 + 3)`.

### Trait definition

```rust
// filename: src/main.rs

// ═══ Semigroup trait ═══
trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

// String: combine = concatenation
impl Semigroup for String {
    fn combine(self, other: Self) -> Self {
        format!("{}{}", self, other)
    }
}

// Vec<T>: combine = append
impl<T> Semigroup for Vec<T> {
    fn combine(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

// i64: combine = addition
impl Semigroup for i64 {
    fn combine(self, other: Self) -> Self {
        self + other
    }
}

// Option<T>: combine = keep first Some, or combine inner values
impl<T: Semigroup> Semigroup for Option<T> {
    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.combine(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}

fn main() {
    // String
    let hello = "Hello ".to_string().combine("World!".to_string());
    println!("{}", hello);

    // Vec
    let nums = vec![1, 2, 3].combine(vec![4, 5]);
    println!("{:?}", nums);

    // i64
    let sum = 10_i64.combine(20);
    println!("{}", sum);

    // Option<String>
    let greeting = Some("Hello ".to_string()).combine(Some("World".to_string()));
    println!("{:?}", greeting); // Some("Hello World")

    let partial = Some("Hello".to_string()).combine(None);
    println!("{:?}", partial); // Some("Hello")
}
```

---

## 28.2 — Monoid: Semigroup + Empty

### Monoid = Semigroup + **identity element** (phần tử đơn vị)

```
combine(a, empty) == a
combine(empty, a) == a
```

Giống: `x + 0 = x`, `s + "" = s`, `v ++ [] = v`.

```rust
// filename: src/main.rs

trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

trait Monoid: Semigroup {
    fn empty() -> Self;
}

// String monoid
impl Semigroup for String {
    fn combine(self, other: Self) -> Self { self + &other }
}
impl Monoid for String {
    fn empty() -> Self { String::new() }
}

// Vec monoid
impl<T> Semigroup for Vec<T> {
    fn combine(mut self, mut other: Self) -> Self { self.append(&mut other); self }
}
impl<T> Monoid for Vec<T> {
    fn empty() -> Self { Vec::new() }
}

// i64 monoid (addition)
impl Semigroup for i64 {
    fn combine(self, other: Self) -> Self { self + other }
}
impl Monoid for i64 {
    fn empty() -> Self { 0 }
}

// bool monoid (AND)
impl Semigroup for bool {
    fn combine(self, other: Self) -> Self { self && other }
}
impl Monoid for bool {
    fn empty() -> Self { true }  // true && x == x
}

// ═══ Generic functions powered by Monoid ═══

fn concat_all<M: Monoid>(items: Vec<M>) -> M {
    items.into_iter().fold(M::empty(), |acc, x| acc.combine(x))
}

fn main() {
    let words = vec!["Hello ".to_string(), "Functional ".into(), "World!".into()];
    println!("{}", concat_all(words));

    let numbers: Vec<i64> = vec![10, 20, 30, 40];
    println!("Sum: {}", concat_all(numbers));

    let flags = vec![true, true, true, false];
    println!("All true? {}", concat_all(flags));

    let lists = vec![vec![1, 2], vec![3, 4], vec![5]];
    println!("Flat: {:?}", concat_all(lists));
}
```

> **💡 Insight**: `concat_all` hoạt động cho **MỌI monoid** — String, numbers, booleans, vectors. 1 function, vô số types!

---

## ✅ Checkpoint 28.2

> Ghi nhớ:
> 1. **Semigroup** = `combine(a, b) -> same_type` + associative
> 2. **Monoid** = Semigroup + `empty()` (identity element)
> 3. `reduce` = Semigroup (cần ≥1 phần tử). `fold(empty, combine)` = Monoid (chạy với 0 phần tử)
> 4. Bạn đã dùng monoids: `String::new()` + `+`, `Vec::new()` + `extend`, `0` + `+`

---

## 28.3 — Domain Monoids: Business Logic

### Statistics aggregation

```rust
// filename: src/main.rs

trait Semigroup { fn combine(self, other: Self) -> Self; }
trait Monoid: Semigroup { fn empty() -> Self; }

// ═══ Order Statistics ═══
#[derive(Debug, Clone)]
struct OrderStats {
    count: u32,
    total_revenue: u64,
    avg_order_value: f64,
    min_order: u64,
    max_order: u64,
}

impl Semigroup for OrderStats {
    fn combine(self, other: Self) -> Self {
        let count = self.count + other.count;
        let revenue = self.total_revenue + other.total_revenue;
        OrderStats {
            count,
            total_revenue: revenue,
            avg_order_value: if count > 0 { revenue as f64 / count as f64 } else { 0.0 },
            min_order: self.min_order.min(other.min_order),
            max_order: self.max_order.max(other.max_order),
        }
    }
}

impl Monoid for OrderStats {
    fn empty() -> Self {
        OrderStats {
            count: 0,
            total_revenue: 0,
            avg_order_value: 0.0,
            min_order: u64::MAX,
            max_order: 0,
        }
    }
}

impl OrderStats {
    fn from_order(amount: u64) -> Self {
        OrderStats {
            count: 1,
            total_revenue: amount,
            avg_order_value: amount as f64,
            min_order: amount,
            max_order: amount,
        }
    }
}

fn concat_all<M: Monoid>(items: Vec<M>) -> M {
    items.into_iter().fold(M::empty(), |acc, x| acc.combine(x))
}

fn main() {
    let orders = vec![150_000_u64, 85_000, 320_000, 45_000, 500_000, 200_000];

    let stats = concat_all(
        orders.iter().map(|&a| OrderStats::from_order(a)).collect()
    );

    println!("📊 Order Statistics:");
    println!("  Orders: {}", stats.count);
    println!("  Revenue: {}đ", stats.total_revenue);
    println!("  Average: {:.0}đ", stats.avg_order_value);
    println!("  Min: {}đ", stats.min_order);
    println!("  Max: {}đ", stats.max_order);
}
```

### Log aggregation

```rust
// filename: src/main.rs

trait Semigroup { fn combine(self, other: Self) -> Self; }
trait Monoid: Semigroup { fn empty() -> Self; }

#[derive(Debug, Clone)]
struct LogSummary {
    total: u32,
    errors: u32,
    warnings: u32,
    info: u32,
    error_messages: Vec<String>,
}

impl Semigroup for LogSummary {
    fn combine(self, other: Self) -> Self {
        LogSummary {
            total: self.total + other.total,
            errors: self.errors + other.errors,
            warnings: self.warnings + other.warnings,
            info: self.info + other.info,
            error_messages: {
                let mut msgs = self.error_messages;
                msgs.extend(other.error_messages);
                msgs
            },
        }
    }
}

impl Monoid for LogSummary {
    fn empty() -> Self {
        LogSummary { total: 0, errors: 0, warnings: 0, info: 0, error_messages: vec![] }
    }
}

impl LogSummary {
    fn error(msg: &str) -> Self {
        LogSummary { total: 1, errors: 1, warnings: 0, info: 0, error_messages: vec![msg.into()] }
    }
    fn warn() -> Self {
        LogSummary { total: 1, errors: 0, warnings: 1, info: 0, error_messages: vec![] }
    }
    fn info() -> Self {
        LogSummary { total: 1, errors: 0, warnings: 0, info: 1, error_messages: vec![] }
    }
}

fn concat_all<M: Monoid>(items: Vec<M>) -> M {
    items.into_iter().fold(M::empty(), |acc, x| acc.combine(x))
}

fn main() {
    let logs = vec![
        LogSummary::info(),
        LogSummary::info(),
        LogSummary::warn(),
        LogSummary::error("DB connection timeout"),
        LogSummary::info(),
        LogSummary::error("Auth failed: invalid token"),
        LogSummary::warn(),
    ];

    let summary = concat_all(logs);
    println!("📋 Log Summary:");
    println!("  Total: {} (✅ {} info, ⚠️ {} warn, ❌ {} error)",
        summary.total, summary.info, summary.warnings, summary.errors);
    if !summary.error_messages.is_empty() {
        println!("  Errors:");
        for msg in &summary.error_messages { println!("    - {}", msg); }
    }
}
```

---

## 28.4 — Bảng Semigroup/Monoid trong Rust std

| Type | Semigroup (combine) | Monoid (empty) |
|------|:-------------------:|:--------------:|
| `String` | `+` (concat) | `""` |
| `Vec<T>` | `extend`/`append` | `vec![]` |
| Numbers (`i32`, `f64`...) | `+` (addition) | `0` |
| Numbers | `*` (multiplication) | `1` |
| `bool` | `&&` (AND) | `true` |
| `bool` | `\|\|` (OR) | `false` |
| `Option<T: Monoid>` | combine inner or keep Some | `None` |
| `HashMap<K, V: Semigroup>` | merge, combine values | `{}` empty map |
| `HashSet<T>` | `union` | `{}` empty set |
| `Duration` | `+` | `Duration::ZERO` |

> **💡 Pattern recognition**: Khi thấy `fold(initial, |acc, x| ...)` trong code → đó là Monoid pattern! `initial` = `empty()`, closure = `combine()`.

---

## 28.5 — Newtype Wrappers: Same Type, Different Monoid

Numbers có **2 monoids**: addition (`0, +`) và multiplication (`1, *`). Dùng newtypes để phân biệt:

```rust
// filename: src/main.rs

trait Semigroup { fn combine(self, other: Self) -> Self; }
trait Monoid: Semigroup { fn empty() -> Self; }

// Sum monoid: 0 + x
#[derive(Debug, Clone, Copy)]
struct Sum(i64);

impl Semigroup for Sum {
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}
impl Monoid for Sum {
    fn empty() -> Self { Sum(0) }
}

// Product monoid: 1 * x
#[derive(Debug, Clone, Copy)]
struct Product(i64);

impl Semigroup for Product {
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}
impl Monoid for Product {
    fn empty() -> Self { Product(1) }
}

// All (AND) monoid
#[derive(Debug, Clone, Copy)]
struct All(bool);

impl Semigroup for All {
    fn combine(self, other: Self) -> Self { All(self.0 && other.0) }
}
impl Monoid for All {
    fn empty() -> Self { All(true) }
}

// Any (OR) monoid
#[derive(Debug, Clone, Copy)]
struct Any(bool);

impl Semigroup for Any {
    fn combine(self, other: Self) -> Self { Any(self.0 || other.0) }
}
impl Monoid for Any {
    fn empty() -> Self { Any(false) }
}

// Min monoid
#[derive(Debug, Clone, Copy)]
struct Min(i64);

impl Semigroup for Min {
    fn combine(self, other: Self) -> Self { Min(self.0.min(other.0)) }
}
impl Monoid for Min {
    fn empty() -> Self { Min(i64::MAX) }
}

// Max monoid
#[derive(Debug, Clone, Copy)]
struct Max(i64);

impl Semigroup for Max {
    fn combine(self, other: Self) -> Self { Max(self.0.max(other.0)) }
}
impl Monoid for Max {
    fn empty() -> Self { Max(i64::MIN) }
}

fn concat_all<M: Monoid>(items: Vec<M>) -> M {
    items.into_iter().fold(M::empty(), |acc, x| acc.combine(x))
}

fn main() {
    let values = vec![3, 7, 2, 9, 5_i64];

    let sum = concat_all(values.iter().map(|&x| Sum(x)).collect());
    let product = concat_all(values.iter().map(|&x| Product(x)).collect());
    let min = concat_all(values.iter().map(|&x| Min(x)).collect());
    let max = concat_all(values.iter().map(|&x| Max(x)).collect());

    println!("Values: {:?}", values);
    println!("Sum: {}", sum.0);
    println!("Product: {}", product.0);
    println!("Min: {}", min.0);
    println!("Max: {}", max.0);

    // Boolean aggregation
    let checks = vec![true, true, true, false];
    let all = concat_all(checks.iter().map(|&x| All(x)).collect());
    let any = concat_all(checks.iter().map(|&x| Any(x)).collect());
    println!("\nChecks: {:?}", checks);
    println!("All true? {}", all.0);
    println!("Any true? {}", any.0);
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Identify monoids

Cái nào là Monoid? Cho mỗi cái, nêu `empty` và `combine`:

1. Non-negative integers under addition
2. Strings under concatenation
3. Non-zero integers under division
4. Lists under append
5. booleans under XOR

<details><summary>✅ Lời giải Bài 1</summary>

1. ✅ **Monoid**: empty = 0, combine = +
2. ✅ **Monoid**: empty = "", combine = concat
3. ❌ **Không**: division không associative: `(8/4)/2 ≠ 8/(4/2)`
4. ✅ **Monoid**: empty = [], combine = append
5. ❌ **Semigroup only**: XOR associative, nhưng `true XOR true = false`, `false XOR false = false`. Empty = `false` thì `false XOR x = x` ✅. Actually nó LÀ monoid! empty = false.

Correction 5: ✅ **Monoid**: XOR is associative, empty = `false`.

</details>

---

**Bài 2** (10 phút): Inventory aggregation

Tạo `InventoryStats` monoid cho warehouse system:
- `total_items: u32`
- `total_value: u64`
- `categories: HashSet<String>`
- `low_stock_count: u32` (items với stock < 10)

Implement `Semigroup` + `Monoid`. Combine stats từ nhiều warehouses.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct InventoryStats {
    total_items: u32,
    total_value: u64,
    categories: HashSet<String>,
    low_stock_count: u32,
}

impl Semigroup for InventoryStats {
    fn combine(self, other: Self) -> Self {
        InventoryStats {
            total_items: self.total_items + other.total_items,
            total_value: self.total_value + other.total_value,
            categories: self.categories.union(&other.categories).cloned().collect(),
            low_stock_count: self.low_stock_count + other.low_stock_count,
        }
    }
}

impl Monoid for InventoryStats {
    fn empty() -> Self {
        InventoryStats {
            total_items: 0, total_value: 0,
            categories: HashSet::new(), low_stock_count: 0,
        }
    }
}
```

</details>

---

**Bài 3** (15 phút): Config merge monoid

Tạo `AppConfig` monoid cho merging config layers (defaults → file → env → CLI):
- `host: Option<String>` — last Some wins
- `port: Option<u16>` — last Some wins
- `features: Vec<String>` — append all
- `debug: All` — all layers phải đồng ý

Implement merge sao cho: `defaults.combine(file).combine(env).combine(cli)`.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
#[derive(Debug, Clone)]
struct AppConfig {
    host: Option<String>,
    port: Option<u16>,
    features: Vec<String>,
    debug: bool,
}

// "Last writer wins" for Option, append for Vec, AND for bool
impl Semigroup for AppConfig {
    fn combine(self, other: Self) -> Self {
        AppConfig {
            host: other.host.or(self.host),       // last Some wins
            port: other.port.or(self.port),       // last Some wins
            features: {
                let mut f = self.features;
                f.extend(other.features);
                f
            },
            debug: self.debug && other.debug,     // AND: all must agree
        }
    }
}

impl Monoid for AppConfig {
    fn empty() -> Self {
        AppConfig { host: None, port: None, features: vec![], debug: true }
    }
}

fn main() {
    let defaults = AppConfig { host: Some("localhost".into()), port: Some(8080), features: vec!["core".into()], debug: true };
    let file = AppConfig { host: None, port: Some(3000), features: vec!["auth".into()], debug: true };
    let env = AppConfig { host: Some("0.0.0.0".into()), port: None, features: vec![], debug: false };

    let merged = defaults.combine(file).combine(env);
    // host: 0.0.0.0 (env), port: 3000 (file), features: [core, auth], debug: false (env said no)
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Associativity bị vi phạm" | Operation không thỏa `(a+b)+c == a+(b+c)` | Kiểm tra: float subtraction, division KHÔNG associative |
| "Empty value không đúng" | `combine(empty, x) ≠ x` | Verify identity law cả 2 chiều |
| "Nhiều monoids cho 1 type" | Number: `+` vs `*` | Dùng newtype wrappers: `Sum(i64)`, `Product(i64)` |
| "Quên implement Monoid" | `fold` cần initial value | Semigroup → dùng `reduce`. Monoid → dùng `fold` |

---

## Tóm tắt

- ✅ **Semigroup** = `combine(a: T, b: T) -> T` + associative. "Kết hợp 2 values cùng type."
- ✅ **Monoid** = Semigroup + `empty()`. "Semigroup có điểm khởi đầu."
- ✅ **Rust std đầy monoids**: `String("", +)`, `Vec([], extend)`, `i64(0, +)`, `bool(true, &&)`.
- ✅ **`fold` = Monoid pattern**: `items.fold(M::empty(), |acc, x| acc.combine(x))`.
- ✅ **Domain monoids**: `OrderStats`, `LogSummary`, `InventoryStats` — combine business data tự nhiên.
- ✅ **Newtype wrappers**: `Sum`, `Product`, `All`, `Any`, `Min`, `Max` — same type, different behavior.

## Tiếp theo

→ Chapter 29: **Functors & Map in Rust** — bạn sẽ nhận ra `.map()` trên `Option`, `Result`, `Iterator` là **cùng 1 pattern**: Functor. Và tại sao Rust không abstract nó thành trait (hint: thiếu Higher-Kinded Types).
