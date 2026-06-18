# Chapter 8 — Data Structures

> **Bạn sẽ học được**:
> - `Vec<T>` sâu hơn — capacity, slices `&[T]`, và các methods quan trọng
> - `String` vs `&str` — vì sao Rust có 2 kiểu chuỗi và khi nào dùng gì
> - `HashMap` và `HashSet` — lookup O(1), entry API
> - Iterators và iterator chains — `.map()`, `.filter()`, `.fold()` — cách FP xử lý collections
>
> **Yêu cầu trước**: Chapter 7 (Functions, Closures).
> **Thời gian đọc**: ~45 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn thành thạo bộ collections cốt lõi của Rust và viết data pipelines bằng iterator chains.

---

## Structs & Enums — Xây dựng domain từ types

Trong OOP, bạn mô hình domain bằng classes với methods và inheritance. Trong Rust/FP, bạn mô hình domain bằng **data types** (structs + enums) tách biệt với **behavior** (impl blocks + traits). Data là data, behavior là behavior — không trộn lẫn.

Sự tách biệt này không phải hạn chế — đó là lợi thế. Khi data và behavior tách nhau, bạn có thể thêm behavior mới (impl trait mới) mà không sửa type. Bạn có thể test data transformations riêng biệt. Và compiler verify data ở compile time mà không cần runtime reflection.

Chapter này dạy bạn xây dựng domain models từ structs và enums — hai viên gạch cơ bản nhất của Rust.

---

## 8.1 — `Vec<T>`: Mảng động

### Tạo và thao tác cơ bản

```rust
// filename: src/main.rs
fn main() {
    // Tạo Vec
    let mut drinks = vec!["Coffee", "Tea", "Smoothie"]; // macro vec!
    let empty: Vec<i32> = Vec::new();                    // constructor
    let zeros = vec![0; 5];                              // [0, 0, 0, 0, 0]

    println!("Drinks: {:?}", drinks);
    println!("Empty: {:?}", empty);
    println!("Zeros: {:?}", zeros);

    // Thêm / Xóa
    drinks.push("Juice");         // thêm cuối — O(1) amortized
    drinks.push("Milk");
    let last = drinks.pop();      // xóa cuối — O(1)
    println!("Popped: {:?}", last);  // Some("Milk")
    println!("Now: {:?}", drinks);   // ["Coffee", "Tea", "Smoothie", "Juice"]

    // Insert / Remove theo index — O(n) vì phải dịch phần tử
    drinks.insert(1, "Matcha");   // chèn tại vị trí 1
    drinks.remove(3);             // xóa vị trí 3 (Smoothie)
    println!("After: {:?}", drinks); // ["Coffee", "Matcha", "Tea", "Juice"]

    // Output:
    // Drinks: ["Coffee", "Tea", "Smoothie"]
    // Empty: []
    // Zeros: [0, 0, 0, 0, 0]
    // Popped: Some("Milk")
    // Now: ["Coffee", "Tea", "Smoothie", "Juice"]
    // After: ["Coffee", "Matcha", "Tea", "Juice"]
}
```

### Capacity vs Length

```rust
// filename: src/main.rs
fn main() {
    // Length = số phần tử hiện tại
    // Capacity = kích thước bộ nhớ đã cấp (>= length)
    let mut v = Vec::with_capacity(10);  // pre-allocate 10 slots
    println!("len={}, capacity={}", v.len(), v.capacity());
    // len=0, capacity=10

    for i in 0..10 {
        v.push(i);
    }
    println!("len={}, capacity={}", v.len(), v.capacity());
    // len=10, capacity=10 — vừa khít, không resize

    v.push(10);  // phải resize!
    println!("len={}, capacity={}", v.len(), v.capacity());
    // len=11, capacity=20 — doubled

    // 💡 Nếu biết trước số lượng, dùng with_capacity → tránh resize
}
```

### Slices `&[T]` — "Cửa sổ" nhìn vào data

Slice là **view** (tham chiếu) vào một phần data, không copy:

```rust
// filename: src/main.rs

// Function nhận slice — linh hoạt hơn nhận &Vec
fn average(items: &[f64]) -> f64 {
    let sum: f64 = items.iter().sum();
    sum / items.len() as f64
}

fn main() {
    let scores = vec![8.5, 7.0, 9.0, 6.5, 8.0];

    // Slice của toàn bộ Vec
    println!("All: {:.1}", average(&scores));

    // Slice một phần
    println!("First 3: {:.1}", average(&scores[..3]));
    println!("Last 2: {:.1}", average(&scores[3..]));

    // Array cũng truyền được — vì &[T; N] coerce thành &[T]
    let fixed = [1.0, 2.0, 3.0];
    println!("Fixed avg: {:.1}", average(&fixed));

    // Output:
    // All: 7.8
    // First 3: 8.2
    // Last 2: 7.3
    // Fixed avg: 2.0
}
```

> **💡 Pro tip**: Viết function nhận `&[T]` thay vì `&Vec<T>` — chấp nhận cả Vec slice, array slice, và sub-slices.

### Các methods hay dùng

```rust
// filename: src/main.rs
fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // Kiểm tra
    println!("Contains 5? {}", numbers.contains(&5));        // true
    println!("Is empty? {}", numbers.is_empty());            // false

    // Tìm kiếm
    println!("First: {:?}", numbers.first());                // Some(3)
    println!("Last: {:?}", numbers.last());                  // Some(6)
    println!("Position of 9: {:?}", numbers.iter().position(|&x| x == 9)); // Some(5)

    // Sort (cần mut)
    let mut sorted = numbers.clone();
    sorted.sort();
    println!("Sorted: {:?}", sorted);  // [1, 1, 2, 3, 4, 5, 6, 9]
    sorted.dedup();                    // loại bỏ duplicates liên tiếp
    println!("Dedup: {:?}", sorted);   // [1, 2, 3, 4, 5, 6, 9]

    // Chunks
    let chunks: Vec<&[i32]> = numbers.chunks(3).collect();
    println!("Chunks of 3: {:?}", chunks); // [[3,1,4], [1,5,9], [2,6]]

    // Windows (sliding window)
    let windows: Vec<&[i32]> = numbers.windows(2).collect();
    println!("Windows of 2: {:?}", windows);
    // [[3,1], [1,4], [4,1], [1,5], [5,9], [9,2], [2,6]]
}
```

---

## ✅ Checkpoint 8.1

> Ghi nhớ:
> 1. `Vec` = dynamic array, `push`/`pop` O(1) amortized
> 2. `&[T]` slice = view, không copy. Viết function nhận `&[T]` thay vì `&Vec<T>`
> 3. `with_capacity(n)` khi biết trước size → tránh resize
>
> **Test nhanh**: `vec![1, 2, 3].windows(2)` cho ra bao nhiêu windows?
> <details><summary>Đáp án</summary>2 windows: <code>[1,2]</code> và <code>[2,3]</code>. Sliding window 2 phần tử trên 3 items.</details>

---

## 8.2 — `String` vs `&str`

### Tại sao Rust có 2 kiểu chuỗi?

Câu hỏi số 1 của người mới. Câu trả lời đơn giản:

| | `String` | `&str` |
|---|---|---|
| Ẩn dụ | **Cuốn sổ** — bạn sở hữu, viết thêm được | **Tấm poster** — bạn chỉ đọc |
| Ownership | Sở hữu data (heap) | Tham chiếu (mượn) |
| Thay đổi? | ✅ `.push_str()`, `+`, etc. | ❌ Read-only |
| Kích thước | Dynamic | Fixed (view) |
| Tạo bằng | `String::from("hello")` | `"hello"` literal |

```rust
// filename: src/main.rs
fn main() {
    // &str — string literal, read-only, sống trong binary
    let greeting: &str = "Hello";

    // String — owned, heap-allocated, mutable
    let mut message = String::from("Hello");
    message.push_str(", Rust!");
    message.push('🦀');
    println!("{}", message);  // Hello, Rust!🦀

    // Chuyển đổi
    let s: String = greeting.to_string();   // &str → String (clone data)
    let r: &str = &s;                        // String → &str (cheap, borrow)
    let also_r: &str = s.as_str();           // tương đương

    println!("s={} r={} also_r={}", s, r, also_r);
}
```

### Quy tắc chọn: `&str` cho input, `String` cho output

```rust
// filename: src/main.rs

// ✅ Nhận &str — chấp nhận cả String lẫn &str
fn greet(name: &str) -> String {
    format!("Hello, {}! 👋", name)  // trả String mới
}

fn main() {
    // Truyền &str
    println!("{}", greet("Rust"));

    // Truyền String (auto-deref thành &str)
    let name = String::from("World");
    println!("{}", greet(&name));

    // Output:
    // Hello, Rust! 👋
    // Hello, World! 👋
}
```

> **💡 Quy tắc**: Function parameters → `&str`. Function return / struct fields / owned data → `String`.

### String methods

```rust
// filename: src/main.rs
fn main() {
    let s = "  Hello, Rust Programming! ";

    // Kiểm tra
    println!("Length (bytes): {}", s.len());       // 28
    println!("Starts with '  H': {}", s.starts_with("  H"));
    println!("Contains 'Rust': {}", s.contains("Rust"));

    // Biến đổi (trả String mới)
    println!("Trimmed: '{}'", s.trim());           // 'Hello, Rust Programming!'
    println!("Upper: {}", s.to_uppercase());
    println!("Replace: {}", s.replace("Rust", "FP"));

    // Split
    let words: Vec<&str> = s.trim().split_whitespace().collect();
    println!("Words: {:?}", words);  // ["Hello,", "Rust", "Programming!"]

    let parts: Vec<&str> = "a,b,c,d".split(',').collect();
    println!("CSV: {:?}", parts);  // ["a", "b", "c", "d"]

    // Join
    let joined = words.join(" → ");
    println!("Joined: {}", joined);  // Hello, → Rust → Programming!

    // ⚠️ String là UTF-8! Indexing theo BYTE, không phải character
    let vietnamese = "Việt Nam";
    println!("Bytes: {}", vietnamese.len());  // 10 (ệ = 3 bytes)
    println!("Chars: {}", vietnamese.chars().count());  // 8
    // vietnamese[0]  // ❌ Không thể index String!
    // Phải dùng .chars().nth(0)
    println!("First char: {:?}", vietnamese.chars().next());  // Some('V')
}
```

---

## 8.3 — `HashMap` & `HashSet`

### `HashMap<K, V>` — Key-value lookup O(1)

```rust
// filename: src/main.rs
use std::collections::HashMap;

fn main() {
    // Tạo
    let mut menu: HashMap<&str, u32> = HashMap::new();
    menu.insert("coffee", 35_000);
    menu.insert("tea", 25_000);
    menu.insert("smoothie", 45_000);

    // Lookup — O(1) average
    println!("Coffee: {:?}", menu.get("coffee"));   // Some(35000)
    println!("Juice: {:?}", menu.get("juice"));      // None

    // Sửa
    menu.insert("coffee", 40_000);  // overwrite
    println!("Coffee (new): {}", menu["coffee"]);  // 40000

    // Xóa
    let removed = menu.remove("tea");
    println!("Removed: {:?}", removed);  // Some(25000)

    // Duyệt
    for (drink, price) in &menu {
        println!("  {}: {}đ", drink, price);
    }

    println!("Total items: {}", menu.len());
}
```

### Entry API — "Thêm nếu chưa có"

```rust
// filename: src/main.rs
use std::collections::HashMap;

fn main() {
    let words = vec!["hello", "world", "hello", "rust", "hello", "world"];

    // Đếm tần suất — entry API
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in &words {
        *counts.entry(word).or_insert(0) += 1;
        // entry(key) → nếu chưa có, insert 0 → rồi +1
    }
    println!("Counts: {:?}", counts);
    // {"hello": 3, "world": 2, "rust": 1}

    // or_insert_with — lazy initialization
    let mut cache: HashMap<u32, String> = HashMap::new();
    let value = cache.entry(42).or_insert_with(|| {
        println!("  Computing value for 42...");
        "forty-two".to_string()
    });
    println!("Value: {}", value);

    // Gọi lần 2 — không compute lại!
    let value = cache.entry(42).or_insert_with(|| {
        println!("  Computing again...");  // KHÔNG chạy
        "new value".to_string()
    });
    println!("Value: {}", value);

    // Output:
    // Counts: {"hello": 3, ...}
    //   Computing value for 42...
    // Value: forty-two
    // Value: forty-two
}
```

### `HashSet<T>` — Tập hợp không trùng lặp

```rust
// filename: src/main.rs
use std::collections::HashSet;

fn main() {
    let mut languages: HashSet<&str> = HashSet::new();
    languages.insert("Rust");
    languages.insert("Go");
    languages.insert("Zig");
    languages.insert("Rust");  // trùng → bị bỏ qua

    println!("Languages: {:?}", languages);
    println!("Contains Rust: {}", languages.contains("Rust"));
    println!("Size: {}", languages.len());  // 3, không phải 4

    // Set operations
    let systems: HashSet<&str> = ["Rust", "C", "Zig"].iter().cloned().collect();
    let modern: HashSet<&str> = ["Rust", "Go", "Zig"].iter().cloned().collect();

    // Giao (intersection): có trong CẢ HAI
    let both: HashSet<&&str> = systems.intersection(&modern).collect();
    println!("Both systems & modern: {:?}", both);  // {"Rust", "Zig"}

    // Hợp (union): có trong ÍT NHẤT MỘT
    let any: HashSet<&&str> = systems.union(&modern).collect();
    println!("Either: {:?}", any);  // {"Rust", "C", "Zig", "Go"}

    // Hiệu (difference): có trong A nhưng không trong B
    let only_systems: HashSet<&&str> = systems.difference(&modern).collect();
    println!("Only systems: {:?}", only_systems);  // {"C"}
}
```

---

## ✅ Checkpoint 8.3

> Ghi nhớ:
> 1. `HashMap` = key-value O(1). Entry API `or_insert` cho "upsert"
> 2. `HashSet` = unique values. Set operations: intersection, union, difference
> 3. `HashMap`/`HashSet` không có thứ tự! Dùng `BTreeMap`/`BTreeSet` nếu cần sorted
>
> **Test nhanh**: `entry(key).or_insert(0)` trả về gì?
> <details><summary>Đáp án</summary><code>&mut V</code> — mutable reference tới giá trị (cũ nếu đã có, hoặc 0 nếu mới insert).</details>

---

## 8.4 — Iterators: Trái tim của FP trong Rust

### Iterator là gì?

Iterator = **luồng giá trị lười biếng** (lazy stream). Nó không tính toán cho đến khi bạn "kéo" giá trị ra.

```rust
// filename: src/main.rs
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // .iter() tạo iterator — chưa tính toán gì!
    let iter = numbers.iter();

    // Chỉ khi "consume" (for, collect, sum...) mới chạy
    let total: i32 = iter.sum();
    println!("Sum: {}", total);  // 15

    // ⚠️ Iterator đã consumed — không dùng lại được
    // let total2: i32 = iter.sum();  // ❌ error: value used after move
}
```

### Ba loại iterator

```rust
// filename: src/main.rs
fn main() {
    let mut names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Carol"),
    ];

    // .iter() → &T (shared reference, không sửa, không lấy)
    for name in names.iter() {
        println!("Hello, {}!", name);  // mượn đọc
    }
    println!("names still exists: {:?}", names);  // ✅

    // .iter_mut() → &mut T (mutable reference, sửa được)
    for name in names.iter_mut() {
        *name = name.to_uppercase();
    }
    println!("Uppercased: {:?}", names);

    // .into_iter() → T (lấy ownership, consume collection)
    for name in names.into_iter() {
        println!("Got: {}", name);
    }
    // println!("{:?}", names);  // ❌ names đã bị consumed!
}
```

| Method | Yields | Collection sau đó? |
|--------|--------|---------------------|
| `.iter()` | `&T` | Vẫn còn ✅ |
| `.iter_mut()` | `&mut T` | Vẫn còn (đã sửa) ✅ |
| `.into_iter()` | `T` | Bị consumed ❌ |

### Iterator Chains — Data pipeline

Sức mạnh thật sự: **nối nhiều bước** thành pipeline. Mỗi bước là lazy — chỉ chạy khi cần.

```rust
// filename: src/main.rs

#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
    in_stock: bool,
}

fn main() {
    let catalog = vec![
        Product { name: "Laptop".into(), price: 25_000_000, in_stock: true },
        Product { name: "Mouse".into(), price: 500_000, in_stock: true },
        Product { name: "Keyboard".into(), price: 1_200_000, in_stock: false },
        Product { name: "Monitor".into(), price: 8_000_000, in_stock: true },
        Product { name: "Webcam".into(), price: 900_000, in_stock: false },
        Product { name: "Headset".into(), price: 2_500_000, in_stock: true },
    ];

    // Pipeline: lọc in-stock → giá > 1M → sắp xếp → format
    let mut available_premium: Vec<String> = catalog.iter()
        .filter(|p| p.in_stock)                       // chỉ còn hàng
        .filter(|p| p.price > 1_000_000)              // giá > 1M
        .map(|p| format!("{}: {}đ", p.name, p.price)) // format
        .collect();
    available_premium.sort();

    println!("🏷️ Premium products in stock:");
    for item in &available_premium {
        println!("  {}", item);
    }

    // Stats
    let total_value: u32 = catalog.iter()
        .filter(|p| p.in_stock)
        .map(|p| p.price)
        .sum();

    let count = catalog.iter().filter(|p| p.in_stock).count();
    let avg = total_value / count as u32;

    println!("\n📊 In-stock: {} items, total {}đ, avg {}đ", count, total_value, avg);

    // Output:
    // 🏷️ Premium products in stock:
    //   Headset: 2500000đ
    //   Laptop: 25000000đ
    //   Monitor: 8000000đ
    //
    // 📊 In-stock: 4 items, total 36000000đ, avg 9000000đ
}
```

### `.fold()` — Swiss army knife

`.fold()` (giống `reduce`/`inject` trong các ngôn ngữ khác) là iterator method tổng quát nhất — mọi method khác đều viết được bằng `.fold()`:

```rust
// filename: src/main.rs
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // sum bằng fold
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);  // 15

    // product bằng fold
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product: {}", product);  // 120

    // max bằng fold
    let max = numbers.iter().fold(i32::MIN, |acc, &x| if x > acc { x } else { acc });
    println!("Max: {}", max);  // 5

    // Build string bằng fold
    let csv = numbers.iter().fold(String::new(), |acc, &x| {
        if acc.is_empty() {
            x.to_string()
        } else {
            format!("{},{}", acc, x)
        }
    });
    println!("CSV: {}", csv);  // 1,2,3,4,5
}
```

### Bảng quick reference — Iterator methods

| Method | Kiểu | Làm gì | Ví dụ |
|--------|------|--------|-------|
| `.map(f)` | Adapter | Biến đổi mỗi phần tử | `[1,2,3].map(\|x\| x*2)` → [2,4,6] |
| `.filter(f)` | Adapter | Giữ phần tử thỏa điều kiện | `[1,2,3].filter(\|x\| x>1)` → [2,3] |
| `.enumerate()` | Adapter | Thêm index | `["a","b"].enumerate()` → [(0,"a"),(1,"b")] |
| `.take(n)` | Adapter | Lấy n phần tử đầu | `[1,2,3,4].take(2)` → [1,2] |
| `.skip(n)` | Adapter | Bỏ n phần tử đầu | `[1,2,3,4].skip(2)` → [3,4] |
| `.zip(iter)` | Adapter | Ghép cặp 2 iterators | `[1,2].zip(["a","b"])` → [(1,"a"),(2,"b")] |
| `.flat_map(f)` | Adapter | map rồi flatten | Nested collections |
| `.chain(iter)` | Adapter | Nối 2 iterators | `[1,2].chain([3,4])` → [1,2,3,4] |
| `.collect()` | Consumer | Thu thập vào collection | `→ Vec<T>`, `→ HashMap`, etc. |
| `.sum()` | Consumer | Tổng | `[1,2,3].sum()` → 6 |
| `.count()` | Consumer | Đếm | `[1,2,3].count()` → 3 |
| `.any(f)` | Consumer | Có phần tử nào thỏa? | `[1,2,3].any(\|x\| x>2)` → true |
| `.all(f)` | Consumer | Tất cả thỏa? | `[1,2,3].all(\|x\| x>0)` → true |
| `.find(f)` | Consumer | Tìm phần tử đầu tiên thỏa | `→ Option<T>` |
| `.fold(init, f)` | Consumer | Reduce với accumulator | Tổng quát nhất |

> **💡 Adapter vs Consumer**: Adapters (map, filter) trả iterator mới — lazy. Consumers (collect, sum) "kéo" data ra — eager. Phải có consumer ở cuối chain!

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Iterator chain

Cho `vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`, viết iterator chain:
- a) Lấy số chẵn, nhân đôi, collect thành `Vec<i32>`
- b) Tính tổng số lẻ
- c) Tìm số đầu tiên > 7

<details><summary>✅ Lời giải Bài 1</summary>

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // a) Chẵn × 2
    let a: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).collect();
    println!("a: {:?}", a);  // [4, 8, 12, 16, 20]

    // b) Tổng lẻ
    let b: i32 = v.iter().filter(|&&x| x % 2 != 0).sum();
    println!("b: {}", b);  // 25

    // c) Đầu tiên > 7
    let c = v.iter().find(|&&x| x > 7);
    println!("c: {:?}", c);  // Some(8)
}
```

</details>

---

**Bài 2** (10 phút): Word frequency counter

Viết function nhận `&str`, trả `HashMap<String, usize>` đếm tần suất mỗi từ (lowercase, bỏ dấu câu).

<details><summary>✅ Lời giải Bài 2</summary>

```rust
use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        if !clean.is_empty() {
            *counts.entry(clean).or_insert(0) += 1;
        }
    }
    counts
}

fn main() {
    let text = "Rust is great. Rust is fast. Rust is safe!";
    let freq = word_frequency(text);

    let mut sorted: Vec<_> = freq.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));  // sắp theo tần suất giảm dần

    for (word, count) in &sorted {
        println!("{}: {}", word, count);
    }
    // Output:
    // rust: 3
    // is: 3
    // great: 1
    // fast: 1
    // safe: 1
}
```

</details>

---

**Bài 3** (15 phút): Student report card

Cho struct `Student { name, scores: Vec<u32> }`. Viết pipeline:
1. Tính average cho mỗi student
2. Lọc students có avg >= 7.0
3. Sort theo avg giảm dần
4. Format output đẹp

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs

#[derive(Debug)]
struct Student {
    name: String,
    scores: Vec<u32>,
}

impl Student {
    fn average(&self) -> f64 {
        let sum: u32 = self.scores.iter().sum();
        sum as f64 / self.scores.len() as f64
    }
}

fn main() {
    let students = vec![
        Student { name: "Minh".into(), scores: vec![8, 9, 7, 8, 9] },
        Student { name: "Lan".into(), scores: vec![5, 6, 4, 5, 6] },
        Student { name: "Hùng".into(), scores: vec![9, 10, 8, 9, 10] },
        Student { name: "Mai".into(), scores: vec![7, 7, 8, 7, 6] },
        Student { name: "Dũng".into(), scores: vec![3, 4, 5, 4, 3] },
    ];

    // Pipeline
    let mut honor_roll: Vec<(String, f64)> = students.iter()
        .map(|s| (s.name.clone(), s.average()))
        .filter(|(_, avg)| *avg >= 7.0)
        .collect();

    honor_roll.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("🏆 Honor Roll (avg ≥ 7.0):");
    for (i, (name, avg)) in honor_roll.iter().enumerate() {
        let medal = match i {
            0 => "🥇",
            1 => "🥈",
            2 => "🥉",
            _ => "  ",
        };
        println!("{} {}: {:.1}", medal, name, avg);
    }

    // Output:
    // 🏆 Honor Roll (avg ≥ 7.0):
    // 🥇 Hùng: 9.2
    // 🥈 Minh: 8.2
    // 🥉 Mai: 7.0
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `cannot index into String` | String là UTF-8, index theo byte | Dùng `.chars().nth(n)` hoặc `.as_bytes()[n]` |
| `value used after move` trên iterator | Iterator đã consumed | Tạo iterator mới bằng `.iter()` lại |
| `lazy iterator not consumed` | Quên `.collect()` hoặc consumer | Thêm consumer cuối chain |
| `type annotations needed for collect` | Rust không đoán được target type | Ghi rõ: `let v: Vec<_> = ...collect()` |
| `borrowed value does not live long enough` | Slice tham chiếu data đã drop | Đảm bảo data source sống lâu hơn slice |

---

## Tóm tắt

- ✅ **`Vec<T>`**: dynamic array, `push`/`pop` O(1). Slices `&[T]` cho function params. `with_capacity` tránh resize.
- ✅ **`String` vs `&str`**: String = owned (sổ). `&str` = borrowed (poster). Params → `&str`, return/fields → `String`.
- ✅ **`HashMap`/`HashSet`**: O(1) lookup. Entry API cho upsert. HashSet cho unique + set operations.
- ✅ **Iterators**: lazy pipeline. Adapters (map, filter) + Consumer (collect, sum, fold). `.fold()` là tổng quát nhất.

## Tiếp theo

→ Chapter 9: **Ownership & Borrowing** — chapter quan trọng nhất của Rust! Bạn sẽ hiểu tại sao Rust không cần garbage collector, ownership rules, borrow checker, và lifetimes. Đây là superpower mà không ngôn ngữ nào khác có.
