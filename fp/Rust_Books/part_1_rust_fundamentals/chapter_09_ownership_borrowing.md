# Chapter 9 — Ownership & Borrowing

> **Bạn sẽ học được**:
> - Ownership — mỗi giá trị có **đúng một chủ sở hữu**, tại sao điều này quan trọng
> - Move semantics — khi gán biến = **chuyển quyền sở hữu**, không copy
> - Borrowing — mượn data mà không lấy ownership (`&T`, `&mut T`)
> - Borrow checker — compiler kiểm tra "ai đang giữ cái gì" lúc compile
> - Lifetimes cơ bản — đảm bảo references không trỏ vào data đã bị xóa
>
> **Yêu cầu trước**: Chapter 8 (Data Structures, đặc biệt Vec và String).
> **Thời gian đọc**: ~50 phút | **Level**: Beginner (nhưng là chapter khó nhất!)
> **Kết quả cuối cùng**: Bạn hiểu borrow checker — không còn "chiến đấu" với compiler, mà **hợp tác** để viết code an toàn.

---

## Tại sao chapter này đặc biệt?

Ownership là tính năng **chỉ Rust có**. Không Go, không Java, không Python — không ngôn ngữ mainstream nào có hệ thống này. Nó thay thế garbage collector bằng **kiểm tra lúc compile** — zero runtime cost, zero bugs liên quan đến memory.

Nếu bạn từng gặp:
- **Null pointer** trong Java/C# → Rust không có null
- **Use-after-free** trong C/C++ → Rust compiler ngăn chặn
- **Data races** trong multi-threaded code → Rust compiler ngăn chặn
- **Memory leaks** từ quên `free()` → Rust tự `drop()` khi owner ra khỏi scope

Ownership giải quyết **tất cả** những vấn đề trên, **lúc compile**, **miễn phí**.

---

## 9.1 — Stack vs Heap: Hai vùng bộ nhớ

### Ẩn dụ: Bàn làm việc vs Kho hàng

- **Stack** = **bàn làm việc**: nhỏ, nhanh, có tổ chức. Đặt đồ lên, lấy ra — LIFO. Kích thước cố định.
- **Heap** = **kho hàng**: rộng, chậm hơn, tìm chỗ trống để đặt. Kích thước linh hoạt.

```rust
// filename: src/main.rs
fn main() {
    // Stack: kích thước cố định, nhanh
    let x: i32 = 42;        // 4 bytes trên stack
    let y: f64 = 3.14;      // 8 bytes trên stack
    let z: bool = true;     // 1 byte trên stack
    let arr = [1, 2, 3];    // 12 bytes trên stack (3 × i32)

    // Heap: kích thước động, chậm hơn
    let name = String::from("Hello");  // data trên heap, pointer trên stack
    let numbers = vec![1, 2, 3, 4];    // data trên heap, metadata trên stack

    println!("Stack: x={}, y={}, z={}, arr={:?}", x, y, z, arr);
    println!("Heap: name={}, numbers={:?}", name, numbers);
}
```

Mô hình bộ nhớ của `String`:

```
Stack                    Heap
┌──────────────┐        ┌───┬───┬───┬───┬───┐
│ ptr ─────────────────→│ H │ e │ l │ l │ o │
│ len: 5       │        └───┴───┴───┴───┴───┘
│ capacity: 5  │
└──────────────┘
```

- **Stack** chứa: pointer, length, capacity (3 × 8 bytes = 24 bytes)
- **Heap** chứa: data thực tế ("Hello" = 5 bytes)

---

## 9.2 — Ownership Rules: Ba luật

Rust có **đúng 3 luật** ownership. Ghi nhớ 3 luật này = hiểu 80% hệ thống:

> 1. **Mỗi giá trị có đúng một owner** (biến sở hữu nó)
> 2. **Chỉ có một owner tại một thời điểm**
> 3. **Khi owner ra khỏi scope → giá trị bị drop** (giải phóng bộ nhớ)

### Luật 3: Drop khi ra khỏi scope

```rust
// filename: src/main.rs
fn main() {
    {
        let name = String::from("Rust");
        println!("Inside: {}", name);
    } // ← name ra khỏi scope → Rust tự gọi drop() → giải phóng heap memory
    // println!("{}", name);  // ❌ error: not found in this scope

    // Giống như: trong C bạn phải gọi free(name) ở đây
    // Rust làm tự động — KHÔNG BAO GIỜ quên!
    println!("name đã bị drop");
}
```

### Luật 1 & 2: Move semantics

```rust
// filename: src/main.rs
fn main() {
    // Với types trên stack (i32, bool, f64): COPY
    let a = 42;
    let b = a;     // COPY — a vẫn dùng được
    println!("a={}, b={}", a, b);  // ✅ OK

    // Với types trên heap (String, Vec): MOVE (chuyển ownership)
    let s1 = String::from("Hello");
    let s2 = s1;   // MOVE — s1 đã bị "chuyển" cho s2
    // println!("{}", s1);  // ❌ error: borrow of moved value: `s1`
    println!("s2={}", s2);  // ✅ OK — s2 là owner mới
}
```

Tại sao Rust chọn **move** thay vì **copy** cho heap types?

```
Nếu COPY:
  s1 ───→ "Hello" (heap)
  s2 ───→ "Hello" (heap copy)   ← tốn gấp đôi bộ nhớ!
  Và ai free? s1 hay s2? → double free bug!

Nếu MOVE:
  s1 (INVALID)
  s2 ───→ "Hello" (heap)        ← chỉ 1 owner, 1 lần free
  Khi s2 ra khỏi scope → free. An toàn!
```

### Move trong function calls

```rust
// filename: src/main.rs

fn print_greeting(msg: String) {  // msg NHẬN ownership
    println!("📩 {}", msg);
} // ← msg ra khỏi scope → drop!

fn main() {
    let greeting = String::from("Hello, Rust!");
    print_greeting(greeting);         // greeting bị MOVE vào function
    // println!("{}", greeting);      // ❌ greeting đã bị move!

    // Giải pháp 1: Clone (copy data)
    let g2 = String::from("Hello again!");
    print_greeting(g2.clone());       // clone truyền vào
    println!("Still mine: {}", g2);   // ✅ g2 vẫn còn

    // Giải pháp 2 (tốt hơn): Borrowing — xem phần 9.3
}
```

### Copy trait — Types tự copy

Một số types nhỏ, rẻ tiền implement `Copy` — gán = copy, không move:

```rust
// filename: src/main.rs
fn main() {
    // Copy types: tất cả scalar types
    let x: i32 = 42;
    let y = x;           // COPY — x vẫn valid
    println!("x={}, y={}", x, y);  // ✅

    let flag = true;
    let flag2 = flag;    // COPY
    println!("{} {}", flag, flag2); // ✅

    let point = (1.0, 2.0);  // tuple of Copy types → cũng Copy
    let point2 = point;
    println!("{:?} {:?}", point, point2); // ✅

    // NON-Copy: String, Vec, HashMap, bất kỳ type chứa heap data
    // let v1 = vec![1, 2, 3];
    // let v2 = v1;  // MOVE — v1 invalid
}
```

| Type | Copy hay Move? | Lý do |
|------|----------------|-------|
| `i32`, `f64`, `bool`, `char` | **Copy** | Nhỏ, rẻ, trên stack |
| Tuples of Copy types | **Copy** | `(i32, bool)` → copy |
| `&T` (shared reference) | **Copy** | Pointer = rẻ |
| `String`, `Vec<T>` | **Move** | Data trên heap, copy = tốn |
| `&mut T` | **Move** | Chỉ 1 mutable ref tại 1 thời điểm |

---

## ✅ Checkpoint 9.2

> Ghi nhớ 3 luật:
> 1. Mỗi giá trị có **đúng 1 owner**
> 2. Assignment (`=`) cho heap types = **move** (chuyển ownership)
> 3. Owner ra khỏi scope → **drop** tự động
>
> **Test nhanh**: Sau `let s2 = s1;` (s1 là String), s1 có dùng được không?
> <details><summary>Đáp án</summary>Không! String move → s1 invalid. Phải dùng <code>.clone()</code> hoặc borrowing.</details>

---

## 9.3 — Borrowing: Mượn mà không lấy

### Ẩn dụ: Mượn sách thư viện

- **Move** = bạn **tặng** cuốn sách cho bạn bè. Bạn không còn sách nữa.
- **Borrow** = bạn **cho mượn** cuốn sách. Bạn vẫn là chủ, bạn bè chỉ đọc rồi trả.

Rust có 2 loại borrow:

| | Shared borrow `&T` | Mutable borrow `&mut T` |
|---|---|---|
| Ẩn dụ | Cho mượn **đọc** | Cho mượn **viết** |
| Bao nhiêu cùng lúc? | **Nhiều** người đọc ✅ | **Đúng 1** người viết ❌ |
| Owner dùng được? | ✅ Đọc OK | ❌ Không, cho đến khi trả |

### Shared borrow `&T`

```rust
// filename: src/main.rs

fn calculate_length(s: &String) -> usize {
    s.len()  // chỉ đọc — không cần ownership
} // ← s ra khỏi scope nhưng KHÔNG drop — vì chỉ là reference

fn main() {
    let greeting = String::from("Hello, Rust!");

    // &greeting = cho mượn, greeting vẫn là owner
    let len = calculate_length(&greeting);
    println!("\"{}\" has {} chars", greeting, len);  // ✅ greeting vẫn valid!

    // Nhiều shared borrows cùng lúc — OK!
    let r1 = &greeting;
    let r2 = &greeting;
    let r3 = &greeting;
    println!("{} {} {}", r1, r2, r3);  // ✅

    // Output:
    // "Hello, Rust!" has 12 chars
    // Hello, Rust! Hello, Rust! Hello, Rust!
}
```

### Mutable borrow `&mut T`

```rust
// filename: src/main.rs

fn add_exclamation(s: &mut String) {
    s.push_str("!!!");  // sửa data — cần &mut
}

fn main() {
    let mut message = String::from("Hello");

    add_exclamation(&mut message);
    println!("{}", message);  // Hello!!!

    // ⚠️ Chỉ 1 mutable borrow tại 1 thời điểm!
    // let r1 = &mut message;
    // let r2 = &mut message;  // ❌ cannot borrow as mutable more than once

    // ⚠️ Không thể có &T và &mut T cùng lúc!
    // let r1 = &message;
    // let r2 = &mut message;  // ❌ cannot borrow as mutable — already borrowed as shared
}
```

### Borrow Rules (2 luật)

> 1. **Tại một thời điểm**: nhiều `&T` **HOẶC** đúng 1 `&mut T` (không cả hai)
> 2. **References phải luôn valid** (không trỏ vào data đã bị drop)

Tại sao luật 1? Ngăn **data races** — nếu ai đó đọc trong khi người khác sửa, bạn có bug. Rust ngăn cả ở compile time.

### Non-Lexical Lifetimes (NLL) — Compiler thông minh hơn bạn tưởng

```rust
// filename: src/main.rs
fn main() {
    let mut data = vec![1, 2, 3];

    let first = &data[0];  // shared borrow bắt đầu
    println!("First: {}", first);  // dùng shared borrow LẦN CUỐI ở đây

    // Compiler biết: first không dùng nữa sau dòng trên
    // → shared borrow KẾT THÚC (NLL)
    // → mutable borrow OK!
    data.push(4);  // ✅ mutable borrow — OK vì first đã "hết hạn"
    println!("Data: {:?}", data);
    // Output:
    // First: 1
    // Data: [1, 2, 3, 4]
}
```

> **💡 NLL (Non-Lexical Lifetimes)**: Từ Rust 2018, borrow checker kết thúc borrow **tại lần sử dụng cuối cùng**, không phải cuối scope. Thông minh hơn nhiều.

---

## ✅ Checkpoint 9.3

> Ghi nhớ:
> 1. `&T` = shared borrow (đọc), nhiều cùng lúc OK
> 2. `&mut T` = mutable borrow (sửa), chỉ 1 tại 1 thời điểm
> 3. Không thể có `&T` và `&mut T` cùng lúc (ngăn data races)
> 4. NLL: borrow kết thúc tại lần dùng cuối, không phải cuối scope
>
> **Test nhanh**: Code sau có compile không?
> ```rust
> let mut v = vec![1, 2, 3];
> let first = &v[0];
> v.push(4);
> println!("{}", first);
> ```
> <details><summary>Đáp án</summary>Không! <code>first</code> (shared borrow) vẫn dùng sau <code>v.push(4)</code> (mutable borrow). Di chuyển <code>println!</code> lên trước <code>push</code> thì OK.</details>

---

## 9.4 — Lifetimes: "Sống bao lâu?"

### Vấn đề: Dangling reference

```rust
// ❌ KHÔNG compile — và đúng vậy!
// fn dangling() -> &String {
//     let s = String::from("hello");
//     &s  // trả reference tới s
// } // ← s bị drop ở đây → reference trỏ vào "xác chết" (dangling!)
```

Rust compiler ngăn chặn: **reference không được sống lâu hơn data nó trỏ tới**.

### Lifetime annotation `'a`

Khi function nhận/trả references, đôi khi compiler cần bạn ghi rõ **mối quan hệ thời gian sống**:

```rust
// filename: src/main.rs

// "Kết quả sống ít nhất lâu bằng x VÀ y"
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let name1 = String::from("Rust Programming");
    let result;

    {
        let name2 = String::from("Go");
        result = longer(&name1, &name2);
        println!("Longer: {}", result);  // ✅ cả name1 và name2 còn sống
    }
    // println!("{}", result);  // ❌ name2 đã bị drop — result có thể trỏ vào nó

    // Output: Longer: Rust Programming
}
```

### Đọc lifetime annotations

```rust
// fn foo<'a>(x: &'a str) → &'a str
// Đọc: "kết quả sống ít nhất lâu bằng x"

// fn bar<'a, 'b>(x: &'a str, y: &'b str) → &'a str
// Đọc: "kết quả sống ít nhất lâu bằng x (không phụ thuộc y)"

// struct Excerpt<'a> { part: &'a str }
// Đọc: "Excerpt không thể sống lâu hơn chuỗi nó tham chiếu"
```

### Lifetime elision — Compiler tự đoán

Phần lớn thời gian, bạn **KHÔNG cần** viết lifetimes. Compiler có 3 rules tự đoán:

```rust
// filename: src/main.rs

// Compiler tự đoán — không cần ghi lifetime
fn first_word(s: &str) -> &str {        // Rule 1 + Rule 2: OK
    s.split_whitespace().next().unwrap_or("")
}

// Cũng tự đoán
fn longest_in_list(items: &[&str]) -> &str {  // 1 input reference → output cùng lifetime
    items.iter().max_by_key(|s| s.len()).unwrap()
}

fn main() {
    let text = "Hello Rust World";
    println!("First word: {}", first_word(text));
    // Output: First word: Hello

    let words = vec!["short", "medium-length", "the longest string here"];
    println!("Longest: {}", longest_in_list(&words));
    // Output: Longest: the longest string here
}
```

> **💡 Khi nào phải ghi lifetime?** Khi function có **nhiều input references** và trả reference — compiler không biết output sống theo input nào. Lúc đó mới cần `'a`.

### Struct chứa references

```rust
// filename: src/main.rs

// Struct chứa reference → cần lifetime annotation
#[derive(Debug)]
struct Highlight<'a> {
    text: &'a str,
    label: &'a str,
}

impl<'a> Highlight<'a> {
    fn display(&self) -> String {
        format!("[{}] {}", self.label, self.text)
    }
}

fn main() {
    let content = String::from("Ownership is Rust's most unique feature");
    let label = "Important";

    let highlight = Highlight {
        text: &content[0..9],  // "Ownership"
        label,
    };

    println!("{}", highlight.display());
    println!("{:?}", highlight);

    // Output:
    // [Important] Ownership
    // Highlight { text: "Ownership", label: "Important" }
}
```

---

## 9.5 — Patterns thực tế

### Pattern 1: Clone khi cần giữ ownership

```rust
// filename: src/main.rs
fn process(data: Vec<i32>) -> i32 {
    data.iter().sum()  // data bị consumed sau function
}

fn main() {
    let original = vec![1, 2, 3, 4, 5];

    let sum = process(original.clone());  // clone → original vẫn còn
    println!("Sum: {}, Original: {:?}", sum, original);
    // Output: Sum: 15, Original: [1, 2, 3, 4, 5]
}
```

### Pattern 2: Borrow thay vì Move (tốt hơn)

```rust
// filename: src/main.rs
fn process(data: &[i32]) -> i32 {  // nhận slice, không lấy ownership
    data.iter().sum()
}

fn main() {
    let original = vec![1, 2, 3, 4, 5];

    let sum = process(&original);  // borrow — không cần clone!
    println!("Sum: {}, Original: {:?}", sum, original);
    // Output: Sum: 15, Original: [1, 2, 3, 4, 5]
}
```

### Pattern 3: Return ownership khi function tạo data

```rust
// filename: src/main.rs
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // tạo String mới → return ownership
}

fn main() {
    let greeting = create_greeting("Rust");
    println!("{}", greeting);  // caller sở hữu String
    // Output: Hello, Rust!
}
```

### Pattern 4: `to_owned()` / `to_string()` khi cần `String` từ `&str`

```rust
// filename: src/main.rs
fn main() {
    let borrowed: &str = "hello";

    let owned1: String = borrowed.to_string();    // &str → String
    let owned2: String = borrowed.to_owned();     // tương tự
    let owned3: String = String::from(borrowed);  // tương tự

    println!("{} {} {}", owned1, owned2, owned3);
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Move hay Copy?

Mỗi dòng sau, biến gốc có dùng được sau assignment không?

```rust
let a = 42;        let b = a;     // a dùng được?
let c = "hello";   let d = c;     // c dùng được?
let e = String::from("hi"); let f = e; // e dùng được?
let g = vec![1,2]; let h = g;     // g dùng được?
let i = (1, true); let j = i;     // i dùng được?
let m = String::from("yo");
let k = &m;        let l = k;     // k dùng được?
```

<details><summary>✅ Lời giải Bài 1</summary>

```
a: ✅ Yes — i32 implements Copy
c: ✅ Yes — &str implements Copy (it's a reference)
e: ❌ No — String is Move (heap data)
g: ❌ No — Vec is Move (heap data)
i: ✅ Yes — (i32, bool) all Copy → tuple is Copy
k: ✅ Yes — &T (shared reference) implements Copy (k borrows m, which is valid)
```

</details>

---

**Bài 2** (10 phút): Fix the borrow checker

Sửa code sau cho compile:

```rust
fn main() {
    let mut names = vec!["Alice", "Bob", "Carol"];
    let first = &names[0];
    names.push("Dave");
    println!("First: {}", first);
}
```

<details><summary>💡 Gợi ý</summary>Vấn đề: shared borrow (<code>first</code>) vẫn active khi <code>push</code> (mutable borrow) xảy ra. Di chuyển thứ tự hoặc clone.</details>

<details><summary>✅ Lời giải Bài 2</summary>

```rust
fn main() {
    let mut names = vec!["Alice", "Bob", "Carol"];

    // Cách 1: Dùng first trước khi push
    let first = &names[0];
    println!("First: {}", first);  // dùng xong → borrow kết thúc (NLL)
    names.push("Dave");            // ✅ OK
    println!("All: {:?}", names);

    // Cách 2: Clone giá trị ra
    let first_owned = names[0].to_string();
    names.push("Eve");
    println!("First: {}, All: {:?}", first_owned, names);
}
```

</details>

---

**Bài 3** (15 phút): Inventory manager

Viết `Inventory` struct quản lý danh sách items. Methods:
- `add_item(&mut self, name: &str)` — thêm item
- `remove_item(&mut self, name: &str) -> bool` — xóa, trả true nếu tìm thấy
- `search(&self, query: &str) -> Vec<&str>` — tìm items chứa query
- `summary(&self) -> String` — format "N items: [...]"

Chú ý borrowing: `search` trả references vào items — cần lifetime đúng.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs

#[derive(Debug)]
struct Inventory {
    items: Vec<String>,
}

impl Inventory {
    fn new() -> Self {
        Inventory { items: vec![] }
    }

    fn add_item(&mut self, name: &str) {
        self.items.push(name.to_string());
    }

    fn remove_item(&mut self, name: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|item| item == name) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    // Trả &str tham chiếu vào self.items
    // Lifetime: kết quả sống ít nhất lâu bằng &self
    fn search(&self, query: &str) -> Vec<&str> {
        self.items.iter()
            .filter(|item| item.to_lowercase().contains(&query.to_lowercase()))
            .map(|item| item.as_str())
            .collect()
    }

    fn summary(&self) -> String {
        format!("{} items: {:?}", self.items.len(), self.items)
    }
}

fn main() {
    let mut inv = Inventory::new();
    inv.add_item("Laptop");
    inv.add_item("Wireless Mouse");
    inv.add_item("Mechanical Keyboard");
    inv.add_item("USB Mouse");

    println!("{}", inv.summary());

    let results = inv.search("mouse");
    println!("Search 'mouse': {:?}", results);

    let removed = inv.remove_item("USB Mouse");
    println!("Removed 'USB Mouse': {}", removed);
    println!("{}", inv.summary());

    // Output:
    // 4 items: ["Laptop", "Wireless Mouse", "Mechanical Keyboard", "USB Mouse"]
    // Search 'mouse': ["Wireless Mouse", "USB Mouse"]
    // Removed 'USB Mouse': true
    // 3 items: ["Laptop", "Wireless Mouse", "Mechanical Keyboard"]
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `borrow of moved value` | Dùng biến đã bị move | Clone trước khi move, hoặc dùng borrow `&` |
| `cannot borrow as mutable more than once` | 2+ `&mut` cùng lúc | Tách thành scope riêng, hoặc redesign |
| `cannot borrow as mutable — already borrowed as immutable` | `&T` và `&mut T` cùng lúc | Dùng xong `&T` trước, rồi mới `&mut T` |
| `missing lifetime specifier` | Function trả reference nhưng thiếu lifetime | Thêm `'a` — xem phần 9.4 |
| `returns a reference to data owned by current function` | Trả reference tới local variable | Return owned data (`String`, `Vec`) thay vì reference |

---

## Tóm tắt

- ✅ **3 luật ownership**: 1 owner, 1 tại 1 thời điểm, drop khi ra scope.
- ✅ **Move vs Copy**: Heap types (String, Vec) move. Stack types (i32, bool) copy. `Clone` cho explicit copy.
- ✅ **Borrowing**: `&T` = shared (nhiều, đọc). `&mut T` = exclusive (1, sửa). Không cả hai cùng lúc.
- ✅ **Lifetimes**: Reference phải sống ngắn hơn data. `'a` annotation khi compiler không tự đoán được. Elision rules xử lý phần lớn.
- ✅ **Patterns**: Nhận `&T`/`&[T]` thay `T` khi không cần ownership. Return `String`/`Vec` khi tạo data mới.

## Tiếp theo

→ Chapter 10: **Error Handling** — bạn sẽ học `Result<T, E>` và `Option<T>` sâu hơn, `?` operator, custom error types, và nền tảng cho Railway-Oriented Programming ở Part IV.
