# Chapter 6 — Control Flow

> **Bạn sẽ học được**:
> - `if/else` là **expression** — trả về giá trị, không chỉ là câu lệnh
> - `match` — pattern matching exhaustive, vũ khí chính của FP trong Rust
> - Loops: `loop`, `while`, `for..in` — và cách FP thay thế loops bằng iterators
> - Patterns nâng cao: destructuring, guards, ranges, bindings
>
> **Yêu cầu trước**: Chapter 5 (Variables, Types).
> **Thời gian đọc**: ~40 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn viết control flow biểu cảm, tận dụng pattern matching để code vừa ngắn gọn vừa an toàn.

---

## Control Flow trong Rust — Biểu thức, không phải lệnh

Điều đầu tiên cần hiểu: trong Rust, hầu hết mọi thứ là **expression** (biểu thức trả về giá trị), không phải statement (câu lệnh). `if/else` trả về giá trị, `match` trả về giá trị, thậm chí blocks `{ ... }` cũng trả về giá trị. Điều này thay đổi cách bạn viết code — thay vì `let x; if ... { x = 1 } else { x = 2 }`, bạn viết `let x = if ... { 1 } else { 2 }`.

Tại sao quan trọng? Vì expressions-based code tự nhiên hơn, tránh uninitialized variables, và dẫn bạn về functional style — nơi data chảy qua transformations thay vì bị mutate in-place.

---

## 6.1 — `if/else` là Expression

### Câu chuyện: Quán cà phê tự động

Hãy tưởng tượng máy order tự động ở quán cà phê. Khách bấm nút, máy **trả ra** đồ uống — không phải máy "làm gì đó" rồi im lặng.

Trong nhiều ngôn ngữ, `if/else` là **statement** — nó "làm gì đó" nhưng không trả giá trị. Trong Rust, `if/else` là **expression** — nó **trả về giá trị**. Đây là khác biệt lớn.

```rust
// filename: src/main.rs
fn main() {
    let score = 85;

    // ❌ Kiểu "statement" (vẫn chạy nhưng dài dòng)
    let grade_verbose;
    if score >= 90 {
        grade_verbose = "A";
    } else if score >= 70 {
        grade_verbose = "B";
    } else {
        grade_verbose = "C";
    }

    // ✅ Kiểu "expression" (idiomatic Rust) — gọn, rõ, an toàn
    let grade = if score >= 90 {
        "A"
    } else if score >= 70 {
        "B"
    } else {
        "C"
    };  // ← chú ý dấu ;

    assert_eq!(grade_verbose, grade);
    println!("Score {}: {}", score, grade);
    // Output: Score 85: B
}
```

Lợi ích của expression:
- Biến luôn được **khởi tạo đầy đủ** — không bao giờ bỏ quên nhánh
- Code **gọn hơn** — 1 `let` thay vì 3 assignments
- **FP mindset** — data "chảy qua" expressions, không phải "bị sửa" bởi statements

### `if let` — Khi chỉ quan tâm một trường hợp

```rust
// filename: src/main.rs
fn main() {
    let maybe_name: Option<&str> = Some("Minh");

    // Cách dài: match
    match maybe_name {
        Some(name) => println!("Hello, {}!", name),
        None => {} // không làm gì
    }

    // Cách gọn: if let — chỉ xử lý Some, bỏ qua None
    if let Some(name) = maybe_name {
        println!("Welcome, {}!", name);
    }

    // if let với else
    let config: Option<u32> = None;
    let port = if let Some(p) = config {
        p
    } else {
        8080 // giá trị mặc định
    };
    println!("Port: {}", port);

    // Output:
    // Hello, Minh!
    // Welcome, Minh!
    // Port: 8080
}
```

> **💡 Khi nào dùng `if let`?** Khi bạn chỉ quan tâm **một** variant của enum. Nếu cần xử lý tất cả variants → dùng `match`.

---

## ✅ Checkpoint 6.1

> Ghi nhớ:
> 1. `if/else` trả về giá trị → gán thẳng vào `let`
> 2. Các nhánh phải cùng kiểu trả về
> 3. `if let` = shortcut khi chỉ quan tâm một pattern
>
> **Test nhanh**: `let x = if true { 1 } else { 2 };` — x = ?
> <details><summary>Đáp án</summary>x = 1. Nhánh <code>true</code> trả 1.</details>

---

## 6.2 — `match` — Pattern Matching Exhaustive

### Tại sao `match` là vũ khí chính?

`match` giống `switch/case` nhưng mạnh hơn ở **3 điểm**:
1. **Exhaustive**: compiler buộc bạn xử lý **mọi** trường hợp
2. **Destructuring**: tách data ra từ enum/struct/tuple
3. **Trả giá trị**: `match` cũng là expression

### Cơ bản: Match trên giá trị

```rust
// filename: src/main.rs
fn main() {
    let status_code = 404;

    let message = match status_code {
        200 => "OK",
        301 => "Moved Permanently",
        404 => "Not Found",
        500 => "Internal Server Error",
        _   => "Unknown",  // _ = "mọi trường hợp còn lại"
    };

    println!("{}: {}", status_code, message);
    // Output: 404: Not Found
}
```

### Destructuring enums

```rust
// filename: src/main.rs

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        // Tách radius ra từ Circle
        Shape::Circle(r) => std::f64::consts::PI * r * r,

        // Tách width, height ra từ Rectangle
        Shape::Rectangle(w, h) => w * h,

        // Tách named fields từ Triangle
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}

fn describe(shape: &Shape) -> String {
    match shape {
        Shape::Circle(r) => format!("⭕ Circle r={:.1}", r),
        Shape::Rectangle(w, h) => format!("🟦 Rect {}×{}", w, h),
        Shape::Triangle { .. } => "🔺 Triangle".to_string(),
        //                 ^^ bỏ qua fields không cần
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle { base: 6.0, height: 3.0 },
    ];

    for s in &shapes {
        println!("{}: area = {:.2}", describe(s), area(s));
    }
    // Output:
    // ⭕ Circle r=5.0: area = 78.54
    // 🟦 Rect 3×4: area = 12.00
    // 🔺 Triangle: area = 9.00
}
```

### Destructuring tuples

```rust
// filename: src/main.rs
fn main() {
    let point = (3, 7);

    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis at {}", x),
        (0, y) => println!("On y-axis at {}", y),
        (x, y) => println!("Point ({}, {})", x, y),
    }
    // Output: Point (3, 7)
}
```

### Match on `Option` and `Result`

```rust
// filename: src/main.rs
fn main() {
    // Option
    let items = vec![10, 20, 30];
    let first = items.first();  // Option<&i32>

    match first {
        Some(val) => println!("First item: {}", val),
        None => println!("Empty list!"),
    }

    // Result
    let input = "42abc";
    let parsed: Result<i32, _> = input.parse();

    match parsed {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {} (input: '{}')", e, input),
    }

    // Output:
    // First item: 10
    // Parse error: invalid digit found in string (input: '42abc')
}
```

---

## ✅ Checkpoint 6.2

> Ghi nhớ:
> 1. `match` là **exhaustive** — compiler bắt buộc xử lý mọi trường hợp
> 2. `_` = wildcard, match mọi thứ còn lại
> 3. Destructuring: tách data từ enum, tuple, struct ngay trong pattern
>
> **Test nhanh**: Nếu enum có 4 variants và bạn chỉ `match` 3 — compiler phản ứng thế nào?
> <details><summary>Đáp án</summary>Error: <code>non-exhaustive patterns</code>. Phải thêm variant thứ 4 hoặc dùng <code>_ =></code>.</details>

---

## 6.3 — Pattern nâng cao

### Match Guards — Thêm điều kiện

```rust
// filename: src/main.rs
fn main() {
    let temperature = 35;

    let description = match temperature {
        t if t < 0   => "Đóng băng 🥶",
        t if t < 15  => "Lạnh 🧥",
        t if t < 25  => "Mát mẻ 😊",
        t if t < 35  => "Nóng ☀️",
        _            => "Rất nóng 🔥",
    };

    println!("{}°C: {}", temperature, description);
    // Output: 35°C: Rất nóng 🔥
}
```

### Multiple patterns với `|`

```rust
// filename: src/main.rs
fn main() {
    let day = "Sat";

    let day_type = match day {
        "Mon" | "Tue" | "Wed" | "Thu" | "Fri" => "Weekday",
        "Sat" | "Sun" => "Weekend",
        _ => "Invalid",
    };

    println!("{}: {}", day, day_type);
    // Output: Sat: Weekend
}
```

### Range patterns

```rust
// filename: src/main.rs
fn main() {
    let score = 75;

    let grade = match score {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        0..=59   => "F",
        _        => "Invalid",
    };

    println!("Score {}: {}", score, grade);
    // Output: Score 75: C
}
```

### Binding với `@`

```rust
// filename: src/main.rs

#[derive(Debug)]
enum Order {
    Pending,
    Processing { items: u32 },
    Shipped { tracking: String },
}

fn describe_order(order: &Order) {
    match order {
        Order::Pending => println!("⏳ Pending"),

        // @ binding: bắt giá trị VÀ kiểm tra điều kiện
        Order::Processing { items: n @ 1..=5 } => {
            println!("🔄 Processing {} items (small order)", n);
        }
        Order::Processing { items: n } => {
            println!("🔄 Processing {} items (large order)", n);
        }

        // Binding toàn bộ inner value
        Order::Shipped { tracking: ref id } => {
            println!("📦 Shipped — tracking: {}", id);
        }
    }
}

fn main() {
    describe_order(&Order::Pending);
    describe_order(&Order::Processing { items: 3 });
    describe_order(&Order::Processing { items: 50 });
    describe_order(&Order::Shipped { tracking: "VN123456".to_string() });

    // Output:
    // ⏳ Pending
    // 🔄 Processing 3 items (small order)
    // 🔄 Processing 50 items (large order)
    // 📦 Shipped — tracking: VN123456
}
```

### Nested destructuring

```rust
// filename: src/main.rs
fn main() {
    let data: Vec<Option<i32>> = vec![Some(1), None, Some(3), Some(4), None];

    // Duyệt + destructure lồng nhau
    let valid: Vec<i32> = data.iter()
        .filter_map(|item| *item)  // bỏ None, unwrap Some
        .collect();

    println!("Valid: {:?}", valid);
    // Output: Valid: [1, 3, 4]

    // Match trên nested structure
    let nested = Some(Some(42));
    match nested {
        Some(Some(n)) => println!("Got: {}", n),
        Some(None) => println!("Outer Some, inner None"),
        None => println!("Nothing"),
    }
    // Output: Got: 42
}
```

---

## 6.4 — Loops: `loop`, `while`, `for`

### `loop` — Vòng lặp vô hạn (có break)

```rust
// filename: src/main.rs
fn main() {
    // loop + break — loop cũng là expression!
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break VỚI giá trị → loop trả về 20
        }
    };
    println!("Result: {}", result);
    // Output: Result: 20
}
```

### `while` — Lặp khi điều kiện đúng

```rust
// filename: src/main.rs
fn main() {
    let mut stock = 5;

    while stock > 0 {
        println!("☕ Serving coffee #{}", 6 - stock);
        stock -= 1;
    }
    println!("❌ Out of stock!");

    // Output:
    // ☕ Serving coffee #1
    // ☕ Serving coffee #2
    // ... (tới #5)
    // ❌ Out of stock!
}
```

### `while let` — Loop + pattern matching

```rust
// filename: src/main.rs
fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];

    // Pop cho đến khi hết
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    println!("Stack empty!");

    // Output:
    // Popped: 5
    // Popped: 4
    // Popped: 3
    // Popped: 2
    // Popped: 1
    // Stack empty!
}
```

### `for..in` — Duyệt collections

```rust
// filename: src/main.rs
fn main() {
    // Range
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();  // 1 2 3 4 5

    // Vec
    let drinks = vec!["Coffee", "Tea", "Smoothie"];
    for drink in &drinks {
        println!("☕ {}", drink);
    }

    // Enumerate — index + value
    for (i, drink) in drinks.iter().enumerate() {
        println!("#{}: {}", i + 1, drink);
    }

    // Output:
    // 1 2 3 4 5
    // ☕ Coffee
    // ☕ Tea
    // ☕ Smoothie
    // #1: Coffee
    // #2: Tea
    // #3: Smoothie
}
```

### Labeled breaks — Thoát vòng lặp ngoài

```rust
// filename: src/main.rs
fn main() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    let target = 5;
    let mut found = false;

    'outer: for (row, cols) in matrix.iter().enumerate() {
        for (col, &val) in cols.iter().enumerate() {
            if val == target {
                println!("Found {} at ({}, {})", target, row, col);
                found = true;
                break 'outer;  // thoát cả 2 vòng lặp!
            }
        }
    }

    if !found {
        println!("{} not found", target);
    }
    // Output: Found 5 at (1, 1)
}
```

### FP alternative: Iterators thay loops

```rust
// filename: src/main.rs
fn main() {
    let orders = vec![35_000, 25_000, 45_000, 40_000, 30_000];

    // ❌ Imperative: mutable variable + loop
    let mut total_imp = 0;
    for &price in &orders {
        if price >= 35_000 {
            total_imp += price;
        }
    }

    // ✅ FP: iterator chain — không cần mut
    let total_fp: u32 = orders.iter()
        .filter(|&&p| p >= 35_000)  // chỉ lấy ≥ 35k
        .sum();                      // tính tổng

    assert_eq!(total_imp, total_fp);
    println!("Premium orders total: {}đ", total_fp);
    // Output: Premium orders total: 120000đ

    // Nhiều bước hơn
    let report: Vec<String> = orders.iter()
        .enumerate()
        .map(|(i, &p)| format!("#{}: {}đ", i + 1, p))
        .collect();
    println!("Orders: {:?}", report);
    // Output: Orders: ["#1: 35000đ", "#2: 25000đ", "#3: 45000đ", "#4: 40000đ", "#5: 30000đ"]
}
```

> **💡 Trong Rust FP**: Bạn sẽ dùng **iterator chains** (`.map().filter().fold()`) nhiều hơn `for` loops. Cùng hiệu suất nhưng rõ ý đồ hơn. Sẽ học kỹ ở Chapter 12.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Rewrite bằng `match` expression

Chuyển code `if/else` sau thành `match`:

```rust
let temp = 28;
let advice;
if temp < 10 { advice = "Wear a coat"; }
else if temp < 20 { advice = "Wear a jacket"; }
else if temp < 30 { advice = "T-shirt weather"; }
else { advice = "Stay hydrated!"; }
```

<details><summary>✅ Lời giải Bài 1</summary>

```rust
let temp = 28;
let advice = match temp {
    t if t < 10 => "Wear a coat",
    t if t < 20 => "Wear a jacket",
    t if t < 30 => "T-shirt weather",
    _ => "Stay hydrated!",
};
```

</details>

---

**Bài 2** (10 phút): Command parser

Viết function `parse_command(input: &str) -> String` dùng `match` xử lý:
- `"quit"` hoặc `"exit"` → "Goodbye!"
- `"help"` → "Available: quit, help, version, greet NAME"
- `"version"` → "v1.0.0"
- Bắt đầu bằng `"greet "` → "Hello, NAME!"
- Khác → "Unknown command: ..."

<details><summary>💡 Gợi ý</summary>Dùng <code>match</code> cho exact matches, <code>if input.starts_with("greet ")</code> cho prefix check, hoặc kết hợp match guard.</details>

<details><summary>✅ Lời giải Bài 2</summary>

```rust
// filename: src/main.rs

fn parse_command(input: &str) -> String {
    match input.trim() {
        "quit" | "exit" => "Goodbye!".to_string(),
        "help" => "Available: quit, help, version, greet NAME".to_string(),
        "version" => "v1.0.0".to_string(),
        cmd if cmd.starts_with("greet ") => {
            let name = &cmd[6..]; // sau "greet "
            format!("Hello, {}!", name)
        }
        other => format!("Unknown command: '{}'", other),
    }
}

fn main() {
    let commands = ["help", "version", "greet Rust", "quit", "dance"];
    for cmd in &commands {
        println!("> {} → {}", cmd, parse_command(cmd));
    }
    // Output:
    // > help → Available: quit, help, version, greet NAME
    // > version → v1.0.0
    // > greet Rust → Hello, Rust!
    // > quit → Goodbye!
    // > dance → Unknown command: 'dance'
}
```

</details>

---

**Bài 3** (15 phút): FizzBuzz nhưng FP

Viết FizzBuzz (1–30) bằng **iterator chain** (không dùng `for` loop với `mut`). Rules: chia hết 15 → "FizzBuzz", chia hết 3 → "Fizz", chia hết 5 → "Buzz", còn lại → số.

<details><summary>💡 Gợi ý</summary>Dùng <code>(1..=30).map(|n| match ...)</code> rồi <code>.collect()</code> thành <code>Vec&lt;String&gt;</code>.</details>

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs
fn main() {
    let result: Vec<String> = (1..=30)
        .map(|n| match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _      => n.to_string(),
        })
        .collect();

    // In 10 items mỗi dòng
    for chunk in result.chunks(10) {
        println!("{}", chunk.join(", "));
    }
    // Output:
    // 1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz
    // 11, Fizz, 13, 14, FizzBuzz, 16, 17, Fizz, 19, Buzz
    // Fizz, 22, 23, Fizz, Buzz, 26, Fizz, 28, 29, FizzBuzz
}
```

Chú ý: `match (n % 3, n % 5)` — match trên **tuple** ! Pattern `(0, 0)` = chia hết cả 3 lẫn 5.

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `non-exhaustive patterns` | `match` chưa cover hết variants | Thêm variant thiếu hoặc `_ => ...` |
| `match arms have incompatible types` | Các nhánh trả về kiểu khác nhau | Đảm bảo tất cả nhánh cùng kiểu |
| `if/else` thiếu `else` khi gán vào `let` | `if` trả giá trị cần `else` | Thêm `else` branch |
| `irrefutable pattern in let` | Dùng `if let` với pattern luôn match | Dùng `let` thường hoặc `match` |
| `unreachable pattern` | Pattern trước đã cover pattern sau | Đặt pattern cụ thể trước, `_` cuối cùng |

---

## Tóm tắt

- ✅ **`if/else` là expression**: gán thẳng vào `let`, các nhánh phải cùng kiểu. `if let` cho pattern đơn lẻ.
- ✅ **`match` là vũ khí chính**: exhaustive, destructuring, trả giá trị. Guards `if`, multi-patterns `|`, ranges `..=`, binding `@`.
- ✅ **Loops**: `loop` (infinite + break with value), `while`/`while let`, `for..in` (phổ biến nhất), labeled `'label: break`.
- ✅ **FP thay loops**: Iterator chains `.map().filter().sum()` — cùng hiệu suất, rõ ý đồ hơn, không cần `mut`.

## Tiếp theo

→ Chapter 7: **Functions & Closures** — bạn sẽ học closures sâu hơn (capture by reference vs by value, `Fn`/`FnMut`/`FnOnce`), first-class functions, và higher-order functions — nền tảng cho mọi pattern FP.
