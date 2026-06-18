# Chapter 13 — Higher-Order Functions & Composition

> **Bạn sẽ học được**:
> - Closures deep dive: `Fn`/`FnMut`/`FnOnce` trong thực tế
> - `Box<dyn Fn>` — lưu trữ closures linh hoạt
> - Function composition — nối functions thành pipeline
> - Iterator chains nâng cao: `scan`, `flat_map`, `inspect`, custom iterators
> - Thiết kế API dùng higher-order functions
>
> **Yêu cầu trước**: Chapter 7 (Functions & Closures), Chapter 12 (Immutability & Purity).
> **Thời gian đọc**: ~40 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Bạn xây dựng data pipelines phức tạp từ các building blocks đơn giản, tái sử dụng được.

---

## 13.1 — Closures trong thực tế: Beyond basics

### LEGO và những viên gạch có thể ghép

Hãy nghĩ về LEGO. Mỗi viên gạch nhỏ bé, đơn giản, chỉ làm một việc. Nhưng nối chúng lại? Bạn xây được nhà, xe, tàu vũ trụ. Sức mạnh không nằm ở từng viên gạch — mà nằm ở cách chúng **kết hợp** với nhau.

Higher-Order Functions (HOFs) và composition là LEGO của lập trình hàm. Closures là viên gạch. `map`, `filter`, `fold` là cách ghép. Và kết quả? Từ những functions nhỏ, đơn giản, bạn xây data pipelines phức tạp mà vẫn đọc hiểu được.

Ở Chapter 7, bạn đã biết closures cơ bản. Giờ hãy xem chúng mạnh thế nào khi **kết hợp**:

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: u32,
    category: String,
    in_stock: bool,
}

fn main() {
    let catalog = vec![
        Product { name: "Laptop".into(), price: 25_000_000, category: "Electronics".into(), in_stock: true },
        Product { name: "Mouse".into(), price: 500_000, category: "Electronics".into(), in_stock: true },
        Product { name: "Novel".into(), price: 150_000, category: "Books".into(), in_stock: false },
        Product { name: "Keyboard".into(), price: 1_200_000, category: "Electronics".into(), in_stock: true },
        Product { name: "Textbook".into(), price: 350_000, category: "Books".into(), in_stock: true },
        Product { name: "Monitor".into(), price: 8_000_000, category: "Electronics".into(), in_stock: true },
    ];

    // Closures như building blocks — tái sử dụng!
    let is_available = |p: &&Product| p.in_stock;
    let is_electronics = |p: &&Product| p.category == "Electronics";
    let is_affordable = |max: u32| move |p: &&Product| p.price <= max;
    let to_label = |p: &Product| format!("{} ({}đ)", p.name, p.price);

    // Nối blocks thành pipeline
    let affordable_electronics: Vec<String> = catalog.iter()
        .filter(is_available)
        .filter(is_electronics)
        .filter(is_affordable(2_000_000))
        .map(to_label)
        .collect();

    println!("Affordable electronics:");
    for item in &affordable_electronics {
        println!("  ✅ {}", item);
    }

    // Dùng LẠI blocks cho query khác
    let available_books: Vec<String> = catalog.iter()
        .filter(is_available)
        .filter(|p| p.category == "Books")
        .map(to_label)
        .collect();

    println!("\nAvailable books:");
    for item in &available_books {
        println!("  📚 {}", item);
    }

    // Output:
    // Affordable electronics:
    //   ✅ Mouse (500000đ)
    //   ✅ Keyboard (1200000đ)
    //
    // Available books:
    //   📚 Textbook (350000đ)
}
```

### `Box<dyn Fn>` — Lưu closures linh hoạt

Khi muốn lưu closures trong collections hoặc struct, cần `Box<dyn Fn>`:

```rust
// filename: src/main.rs

type Filter = Box<dyn Fn(&i32) -> bool>;

fn build_filters(min: i32, max: i32) -> Vec<Filter> {
    vec![
        Box::new(move |x| *x >= min),      // >= min
        Box::new(move |x| *x <= max),      // <= max
        Box::new(|x| *x % 2 == 0),         // chẵn
    ]
}

fn apply_filters(data: &[i32], filters: &[Filter]) -> Vec<i32> {
    data.iter()
        .filter(|x| filters.iter().all(|f| f(x)))
        .copied()
        .collect()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let filters = build_filters(3, 10);

    let result = apply_filters(&data, &filters);
    println!("3 <= x <= 10, even: {:?}", result);
    // Output: 3 <= x <= 10, even: [4, 6, 8, 10]
}
```

---

## ✅ Checkpoint 13.1

> Ghi nhớ:
> 1. Closures = **building blocks** tái sử dụng. Đặt tên descriptive!
> 2. `Box<dyn Fn>` khi cần lưu closures khác nhau cùng collection
> 3. Factory closures: `let is_affordable = |max| move |p| p.price <= max` — tạo closure từ config
>
> **Test nhanh**: Tại sao `is_affordable` dùng `move`?
> <details><summary>Đáp án</summary><code>max</code> cần được move vào closure vì closure sẽ sống lâu hơn function call tạo ra nó.</details>

---

## 13.2 — Function Composition

### Nối LEGO thành đường ống

Bạn đã thấy closures như viên gạch riêng lẻ. Giờ hãy học cách **nối** chúng — không phải bằng tay mỗi lần, mà bằng function tự động. Composition = nối output function A thành input function B. Giống như đường ống nước: nước chảy từ van A sang bộ lọc B, rồi ra vòi C. Bạn không cần biết chi tiết bên trong — chỉ cần nối đúng thứ tự.

### Compose hai functions

```rust
// filename: src/main.rs

// compose(f, g) = |x| g(f(x))
fn compose<A, B, C>(
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

fn main() {
    let add_tax = |price: u32| (price as f64 * 1.08) as u32;
    let format_price = |price: u32| format!("{}đ", price);

    // Compose: add_tax → format_price
    let price_with_tax = compose(add_tax, format_price);
    println!("{}", price_with_tax(35_000));  // 37800đ
    println!("{}", price_with_tax(50_000));  // 54000đ
}
```

### Pipe — Áp dụng nhiều bước tuần tự

```rust
// filename: src/main.rs

// pipe: value |> f1 |> f2 |> f3 (giống Elixir/F#)
fn pipe<T>(value: T, steps: &[&dyn Fn(T) -> T]) -> T
where T: Copy
{
    steps.iter().fold(value, |acc, f| f(acc))
}

fn main() {
    let result = pipe(100_u32, &[
        &|x| x + 50,        // +50 = 150
        &|x| x * 2,         // ×2 = 300
        &|x| x - 20,        // -20 = 280
    ]);
    println!("pipe(100): {}", result);  // 280

    // Thực tế: price pipeline
    let final_price = pipe(35_000_u32, &[
        &|p| p * 90 / 100,      // discount 10%
        &|p| p + p * 8 / 100,   // add tax 8%
        &|p| (p / 1000) * 1000, // round to nearest 1000
    ]);
    println!("Final: {}đ", final_price);  // 34000đ
}
```

### Real-world: Text processing pipeline

```rust
// filename: src/main.rs

fn main() {
    let raw_input = "  Hello,   World!  This  IS    a   TEST.  ";

    // Pipeline: normalize text
    let normalized: String = raw_input
        .trim()                           // bỏ whitespace đầu/cuối
        .to_lowercase()                   // lowercase
        .split_whitespace()               // tách words (bỏ extra spaces)
        .collect::<Vec<_>>()
        .join(" ");                       // nối lại 1 space

    println!("Normalized: '{}'", normalized);
    // Output: Normalized: 'hello, world! this is a test.'

    // Pipeline: extract + transform
    let word_lengths: Vec<(String, usize)> = raw_input
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .map(|w| {
            let clean: String = w.chars().filter(|c| c.is_alphanumeric()).collect();
            let len = clean.len();
            (clean, len)
        })
        .filter(|(_, len)| *len > 2)     // chỉ words dài > 2
        .collect();

    println!("Words: {:?}", word_lengths);
    // Output: Words: [("hello", 5), ("world", 5), ("this", 4), ("test", 4)]
}
```

---

## 13.3 — Iterator Chains nâng cao

Chapter 7 dạy `map` và `filter`. Nhưng Rust còn nhiều combinators mạnh hơn: `flat_map` (xếp phẳng collections lồng), `scan` (giữ state qua mỗi iteration — như running total), `inspect` (debug mà không sửa data), và `zip` (ghép 2 iterators).

### `flat_map` — Flatten nested collections

```rust
// filename: src/main.rs
fn main() {
    let sentences = vec![
        "Rust is fast",
        "FP is elegant",
        "DDD is powerful",
    ];

    // map → Vec<Vec<&str>> (nested)
    // flat_map → Vec<&str> (flattened)
    let all_words: Vec<&str> = sentences.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();

    println!("All words: {:?}", all_words);
    // ["Rust", "is", "fast", "FP", "is", "elegant", "DDD", "is", "powerful"]

    // Unique words
    let unique: std::collections::HashSet<&str> = all_words.iter().cloned().collect();
    println!("Unique: {:?}", unique);
}
```

### `scan` — Stateful transform (running total)

```rust
// filename: src/main.rs
fn main() {
    let transactions = vec![100, -30, 50, -20, 80];

    // Running balance — scan giữ state qua iterations
    let balances: Vec<i32> = transactions.iter()
        .scan(0_i32, |balance, &tx| {
            *balance += tx;
            Some(*balance)
        })
        .collect();

    println!("Transactions: {:?}", transactions);
    println!("Balances:     {:?}", balances);
    // Transactions: [100, -30, 50, -20, 80]
    // Balances:     [100, 70, 120, 100, 180]
}
```

### `inspect` — Debug pipeline mà không ảnh hưởng data

```rust
// filename: src/main.rs
fn main() {
    let result: Vec<i32> = (1..=10)
        .inspect(|x| print!("→{} ", x))  // debug: thấy input
        .filter(|x| x % 2 == 0)
        .inspect(|x| print!("[{}] ", x))  // debug: thấy sau filter
        .map(|x| x * x)
        .collect();

    println!();
    println!("Result: {:?}", result);
    // →1 →2 [2] →3 →4 [4] →5 →6 [6] →7 →8 [8] →9 →10 [10]
    // Result: [4, 16, 36, 64, 100]
}
```

### `zip` + `enumerate` — Ghép data

```rust
// filename: src/main.rs
fn main() {
    let names = vec!["Minh", "Lan", "Hùng"];
    let scores = vec![85, 92, 78];

    // zip: ghép 2 iterators thành tuples
    let report: Vec<String> = names.iter()
        .zip(scores.iter())
        .enumerate()
        .map(|(i, (name, score))| {
            let rank = if *score >= 90 { "🌟" } else { "  " };
            format!("#{} {} {}: {}pts", i + 1, rank, name, score)
        })
        .collect();

    for line in &report {
        println!("{}", line);
    }
    // #1    Minh: 85pts
    // #2 🌟 Lan: 92pts
    // #3    Hùng: 78pts
}
```

---

## 13.4 — Custom Iterator

Bạn có thể tạo iterator **riêng** bằng cách implement trait `Iterator`:

```rust
// filename: src/main.rs

// Iterator đếm theo bước tùy chỉnh
struct StepCounter {
    current: u32,
    step: u32,
    max: u32,
}

impl StepCounter {
    fn new(start: u32, step: u32, max: u32) -> Self {
        StepCounter { current: start, step, max }
    }
}

impl Iterator for StepCounter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            None  // kết thúc
        } else {
            let value = self.current;
            self.current += self.step;
            Some(value)
        }
    }
}

fn main() {
    // Đếm từ 0 đến 20, bước 3
    let steps: Vec<u32> = StepCounter::new(0, 3, 20).collect();
    println!("Steps: {:?}", steps);
    // Steps: [0, 3, 6, 9, 12, 15, 18]

    // Kết hợp với iterator chains!
    let sum: u32 = StepCounter::new(1, 2, 15)  // odds: 1,3,5,7,9,11,13,15
        .filter(|x| x % 3 != 0)                 // bỏ chia hết cho 3
        .map(|x| x * x)                         // bình phương
        .sum();
    println!("Sum of squares: {}", sum);
    // 1,5,7,11,13 → 1,25,49,121,169 → 365
}
```

### Fibonacci iterator

```rust
// filename: src/main.rs

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(value)  // vô hạn — luôn trả Some
    }
}

fn main() {
    // Lấy 10 số Fibonacci đầu tiên
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("Fib: {:?}", fibs);
    // Fib: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

    // Tổng Fibonacci < 1000
    let sum: u64 = Fibonacci::new()
        .take_while(|&x| x < 1000)
        .sum();
    println!("Sum of Fib < 1000: {}", sum);
    // Sum of Fib < 1000: 1596
}
```

---

## 13.5 — Thiết kế API với HOFs

Closures không chỉ dùng trong pipelines — chúng là cách thiết kế API linh hoạt. Thay vì hard-code logic ("VIP giảm 15%"), nhận logic từ bên ngoài dưới dạng closure. Đây là Strategy pattern và Middleware pattern trong FP.

### Strategy pattern bằng closures

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
struct Order {
    items: Vec<(String, u32, u32)>, // (name, price, quantity)
}

impl Order {
    fn new(items: Vec<(String, u32, u32)>) -> Self {
        Order { items }
    }

    fn subtotal(&self) -> u32 {
        self.items.iter().map(|(_, p, q)| p * q).sum()
    }

    // HOF: nhận pricing strategy từ bên ngoài
    fn total_with_strategy<F: Fn(u32) -> u32>(&self, pricing: F) -> u32 {
        pricing(self.subtotal())
    }
}

// Pricing strategies — pure functions
fn no_discount(price: u32) -> u32 { price }
fn vip_discount(price: u32) -> u32 { price * 85 / 100 }
fn holiday_sale(price: u32) -> u32 { price * 70 / 100 }

fn make_coupon_discount(percent: u32) -> impl Fn(u32) -> u32 {
    move |price| price * (100 - percent) / 100
}

fn main() {
    let order = Order::new(vec![
        ("Coffee".into(), 35_000, 2),
        ("Cake".into(), 25_000, 1),
    ]);

    println!("Subtotal: {}đ", order.subtotal());
    println!("Regular:  {}đ", order.total_with_strategy(no_discount));
    println!("VIP:      {}đ", order.total_with_strategy(vip_discount));
    println!("Holiday:  {}đ", order.total_with_strategy(holiday_sale));
    println!("Coupon15: {}đ", order.total_with_strategy(make_coupon_discount(15)));

    // Output:
    // Subtotal: 95000đ
    // Regular:  95000đ
    // VIP:      80750đ
    // Holiday:  66500đ
    // Coupon15: 80750đ
}
```

### Middleware pattern

```rust
// filename: src/main.rs

type Middleware = Box<dyn Fn(&str) -> String>;

fn logger() -> Middleware {
    Box::new(|input| {
        println!("  [LOG] Processing: {}", input);
        input.to_string()
    })
}

fn trimmer() -> Middleware {
    Box::new(|input| input.trim().to_string())
}

fn lowercaser() -> Middleware {
    Box::new(|input| input.to_lowercase())
}

fn validator(min_len: usize) -> Middleware {
    Box::new(move |input| {
        if input.len() >= min_len {
            input.to_string()
        } else {
            format!("[INVALID: too short] {}", input)
        }
    })
}

fn process(input: &str, middlewares: &[Middleware]) -> String {
    middlewares.iter().fold(input.to_string(), |data, mw| mw(&data))
}

fn main() {
    let pipeline = vec![
        logger(),
        trimmer(),
        lowercaser(),
        validator(3),
    ];

    let inputs = vec!["  Hello World  ", "  Hi  ", "  RUST  "];
    for input in inputs {
        println!("Input: '{}'", input);
        let result = process(input, &pipeline);
        println!("Output: '{}'\n", result);
    }
    // Input: '  Hello World  '
    //   [LOG] Processing:   Hello World
    // Output: 'hello world'
    //
    // Input: '  Hi  '
    //   [LOG] Processing:   Hi
    // Output: '[INVALID: too short] hi'
    //
    // Input: '  RUST  '
    //   [LOG] Processing:   RUST
    // Output: 'rust'
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Chain đọc hiểu

`vec![1,2,3,4,5].iter().map(|x| x * 3).filter(|x| x > &6).sum::<i32>()` = ?

<details><summary>✅ Lời giải Bài 1</summary>

```
[1,2,3,4,5] → map(×3) → [3,6,9,12,15]
→ filter(>6) → [9,12,15]
→ sum = 36
```

</details>

---

**Bài 2** (10 phút): Custom Range iterator

Implement `FloatRange` iterator: `FloatRange::new(0.0, 1.0, 0.2)` → yields [0.0, 0.2, 0.4, 0.6, 0.8].

<details><summary>✅ Lời giải Bài 2</summary>

```rust
struct FloatRange {
    current: f64,
    end: f64,
    step: f64,
}

impl FloatRange {
    fn new(start: f64, end: f64, step: f64) -> Self {
        FloatRange { current: start, end, step }
    }
}

impl Iterator for FloatRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            None
        } else {
            let value = self.current;
            self.current += self.step;
            Some(value)
        }
    }
}

fn main() {
    let values: Vec<f64> = FloatRange::new(0.0, 1.0, 0.2).collect();
    println!("{:.1?}", values);  // [0.0, 0.2, 0.4, 0.6, 0.8]
}
```

</details>

---

**Bài 3** (15 phút): Data transformation pipeline

Cho danh sách log entries (strings), viết pipeline:
1. Parse log: `"2024-01-15 ERROR Database connection failed"` → `LogEntry { date, level, message }`
2. Filter: chỉ ERROR và WARN
3. Group by level → `HashMap<String, Vec<LogEntry>>`
4. Format summary

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct LogEntry {
    date: String,
    level: String,
    message: String,
}

fn parse_log(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return None;
    }
    Some(LogEntry {
        date: parts[0].to_string(),
        level: parts[1].to_string(),
        message: parts[2].to_string(),
    })
}

fn main() {
    let logs = vec![
        "2024-01-15 ERROR Database connection failed",
        "2024-01-15 INFO Server started",
        "2024-01-16 WARN Memory usage high",
        "2024-01-16 ERROR Payment timeout",
        "2024-01-16 INFO User logged in",
        "2024-01-17 WARN Disk space low",
        "2024-01-17 ERROR API rate limited",
    ];

    // Pipeline: parse → filter → group
    let important: Vec<LogEntry> = logs.iter()
        .filter_map(|line| parse_log(line))
        .filter(|entry| entry.level == "ERROR" || entry.level == "WARN")
        .collect();

    // Group by level
    let mut grouped: HashMap<String, Vec<&LogEntry>> = HashMap::new();
    for entry in &important {
        grouped.entry(entry.level.clone())
            .or_default()
            .push(entry);
    }

    // Format summary
    println!("📊 Log Summary ({} important entries):", important.len());
    for (level, entries) in &grouped {
        println!("\n  {} ({}):", level, entries.len());
        for e in entries {
            println!("    {} — {}", e.date, e.message);
        }
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `expected Fn, found closure` | Closure sizes differ — không lưu được cùng Vec | Dùng `Box<dyn Fn>` |
| `cannot infer type for collect` | Quên type annotation | Thêm `: Vec<_>` hoặc turbofish `.collect::<Vec<_>>()` |
| Iterator consumed — dùng lại `iter` | Iterators single-use | Gọi `.iter()` lại, hoặc `.clone()` trước consume |
| Closure capture lifetime issue | Closure sống lâu hơn data | Dùng `move` hoặc clone data trước |

---

## Tóm tắt

Chapter này dạy bạn chơi LEGO với functions — từng viên gạch nhỏ ghép thành công trình lớn:

- ✅ **Closures = building blocks**: đặt tên descriptive, tái sử dụng qua pipelines. `Box<dyn Fn>` cho dynamic storage.
- ✅ **Composition**: `compose(f, g)` = `|x| g(f(x))`. `pipe(value, steps)` cho sequential transforms. Đường ống: nước chảy từ đầu đến cuối.
- ✅ **Iterator chains nâng cao**: `flat_map` (flatten), `scan` (stateful), `inspect` (debug), `zip` (ghép).
- ✅ **Custom Iterator**: impl `Iterator` trait, define `next()`. Từ Fibonacci đến StepCounter.
- ✅ **API design với HOFs**: Strategy pattern bằng closures, middleware pipeline bằng `Vec<Box<dyn Fn>>`.

## Tiếp theo

Bạn đã biết ghép functions — giờ hãy học **ghép types**. Structs và enums là LEGO cho data — product types (AND) và sum types (OR) cho bạn công cụ **thiết kế** data, không chỉ lưu trữ.

→ Chapter 14: **Structs, Enums & Algebraic Types** — bạn sẽ dùng type system như tool thiết kế: product types (struct), sum types (enum), newtype pattern, và "types as design tools" — nền tảng cho DDD ở Part IV.
