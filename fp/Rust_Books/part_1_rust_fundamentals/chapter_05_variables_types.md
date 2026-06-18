# Chapter 5 — Variables, Types & Operators

> **Bạn sẽ học được**:
> - `let` bindings — tại sao Rust chọn immutable là mặc định
> - `mut` và shadowing — hai cách "thay đổi" giá trị, khác nhau hoàn toàn
> - Scalar types (số, boolean, ký tự) và compound types (tuples, arrays)
> - Type inference — Rust đoán kiểu cho bạn, nhưng bạn cần biết lúc nào ghi rõ
> - Operators và type casting
>
> **Yêu cầu trước**: Chapter 4 (Getting Started — biết tạo project và `cargo run`).
> **Thời gian đọc**: ~35 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn hiểu rõ hệ thống types của Rust và viết code biểu cảm, an toàn.

---

## Types — Nền tảng mà mọi thứ xây lên trên

Trong Rust, type system không phải "thủ tục hành chính" bạn phải hoàn thành để code compile. Đó là **đối tác thiết kế** — giúp bạn suy nghĩ rõ ràng về data, ngăn ngừa bugs, và tạo code tự document.

Điều đặc biệt ở Rust: compiler **suy luận** types cho bạn trong hầu hết trường hợp (type inference), nhưng bạn vẫn có thể khai báo tường minh khi cần làm rõ ý đồ. Kết quả: code gọn như Python nhưng an toàn như Haskell.

Chapter này đi qua mọi loại type trong Rust — từ primitives đến compound types. Đây là vocabulary bạn dùng mỗi ngày.

---

## 5.1 — `let` Bindings: Immutable by default

### Câu chuyện: Bảng giá khắc trên gỗ

Bạn vào quán cà phê, thấy bảng giá **khắc trên gỗ**. Muốn đổi giá? Phải thay cả tấm bảng. Bất tiện, nhưng an toàn — không ai "lỡ tay" sửa giá.

Ngược lại, quán bên cạnh viết giá bằng **phấn trên bảng đen**. Đổi giá nhanh, nhưng ai cũng có thể "vô tình" xóa hoặc sửa sai.

Rust chọn cách **khắc gỗ là mặc định**: mọi biến đều immutable. Muốn "bảng phấn" phải nói rõ bằng `mut`.

```rust
// filename: src/main.rs
fn main() {
    // "Khắc gỗ" — immutable (mặc định)
    let price = 35_000;
    println!("Coffee: {}đ", price);

    // price = 40_000;  // ❌ error[E0384]: cannot assign twice to immutable variable

    // "Bảng phấn" — mutable (phải nói rõ)
    let mut stock = 100;
    println!("Stock: {}", stock);
    stock -= 1;  // ✅ OK — có mut
    println!("After sale: {}", stock);

    // Output:
    // Coffee: 35000đ
    // Stock: 100
    // After sale: 99
}
```

### Tại sao immutable mặc định?

3 lý do thực tế:

1. **An toàn**: Không ai "lỡ tay" sửa biến ở dòng 200 khi bạn assume nó không đổi
2. **Dễ đọc**: Thấy `let x = 5` → biết chắc x **luôn là 5** trong scope này
3. **FP mindset**: Trong FP, data "chảy qua" các functions mà không bị sửa giữa đường. Immutable biến makes this natural

> **💡 So sánh**: F# cũng `let` immutable mặc định, cần `let mutable` để sửa. JavaScript `const` tương tự nhưng không strict bằng (object vẫn sửa properties được). Rust `let` strict hơn — không thể sửa gì cả.

### Shadowing — "Tấm bảng gỗ mới"

Shadowing là khai báo biến **cùng tên** nhưng **giá trị mới** (và có thể **kiểu mới**). Không phải sửa biến cũ — mà tạo biến mới trùng tên:

```rust
// filename: src/main.rs
fn main() {
    let price = "35000";             // &str
    println!("String: {}", price);

    let price = price.parse::<u32>().unwrap();  // Bây giờ là u32!
    println!("Number: {}", price);

    let price = price + 5_000;       // Tính toán với u32
    println!("After markup: {}đ", price);

    // Output:
    // String: 35000
    // Number: 35000
    // After markup: 40000đ
}
```

Shadowing ≠ mutation:

| | `mut` (Mutation) | Shadowing |
|---|---|---|
| Kiểu thay đổi? | ❌ Không — cùng kiểu | ✅ Có thể khác kiểu |
| Biến cũ còn không? | Bị ghi đè | Bị "che" nhưng vẫn tồn tại |
| FP-friendly? | ❌ | ✅ — giống bind mới |
| Dùng khi nào? | Cần cập nhật giá trị cùng kiểu | Transform data qua nhiều bước |

```rust
// filename: src/main.rs
fn main() {
    // Shadowing: OK — đổi kiểu từ &str → usize
    let input = "hello";
    let input = input.len();  // Bây giờ là usize = 5
    println!("Length: {}", input);

    // mut: KHÔNG OK — không đổi được kiểu
    // let mut value = "hello";
    // value = 42;  // ❌ error: mismatched types

    // Output: Length: 5
}
```

> **💡 Khi nào dùng gì?**
> - **Shadowing**: khi transform data qua nhiều bước (parse string → number → tính toán)
> - **`mut`**: khi cần cập nhật cùng một giá trị nhiều lần (counter, accumulator, buffer)

---

## ✅ Checkpoint 5.1

> Ghi nhớ:
> 1. `let x = 5` → immutable (mặc định). `let mut x = 5` → mutable (phải nói rõ)
> 2. Shadowing = `let x` lại → biến **mới** trùng tên, có thể khác kiểu
> 3. Mutation ≠ Shadowing: `mut` giữ kiểu, shadowing cho đổi kiểu
>
> **Test nhanh**: Code sau có compile không?
> ```rust
> let x = 5;
> let x = x + 1;
> let x = x * 2;
> ```
> <details><summary>Đáp án</summary>Có! Đây là shadowing — mỗi <code>let x</code> tạo biến mới. Kết quả: x = (5+1)*2 = 12.</details>

---

## 5.2 — Scalar Types: Bốn kiểu "đơn lẻ"

### Integer — Số nguyên

| Kiểu | Bits | Phạm vi | Khi nào dùng |
|------|------|---------|-------------|
| `i8` | 8 | -128 → 127 | Hiếm dùng |
| `i16` | 16 | -32,768 → 32,767 | Hiếm dùng |
| `i32` | 32 | ±2.1 tỷ | **Mặc định** — dùng cho hầu hết |
| `i64` | 64 | ±9.2 × 10¹⁸ | Tiền tệ, timestamps |
| `u8` | 8 | 0 → 255 | Bytes, RGB colors |
| `u32` | 32 | 0 → 4.2 tỷ | IDs, counts |
| `u64` | 64 | 0 → 1.8 × 10¹⁹ | File sizes |
| `usize` | 32/64 | Phụ thuộc platform | Index, length |
| `isize` | 32/64 | Phụ thuộc platform | Pointer arithmetic |

```rust
// filename: src/main.rs
fn main() {
    // Rust tự suy kiểu i32 cho số nguyên
    let age = 25;                // i32
    let price: u64 = 1_500_000;  // _ giúp đọc dễ hơn
    let byte: u8 = 255;
    let hex = 0xFF;              // hex literal = 255
    let binary = 0b1111_0000;    // binary literal = 240
    let octal = 0o77;            // octal literal = 63

    println!("age={} price={} byte={} hex={} binary={} octal={}",
        age, price, byte, hex, binary, octal);
    // Output: age=25 price=1500000 byte=255 hex=255 binary=240 octal=63

    // Integer overflow → panic trong debug, wrap trong release
    // let overflow: u8 = 256;  // ❌ error: literal out of range
}
```

### Float — Số thực

```rust
// filename: src/main.rs
fn main() {
    let pi = 3.14159;            // f64 — mặc định (double precision)
    let temperature: f32 = 36.5; // f32 — single precision

    // ⚠️ Floating point KHÔNG chính xác
    let result = 0.1 + 0.2;
    println!("0.1 + 0.2 = {}", result);       // 0.30000000000000004
    println!("0.1 + 0.2 == 0.3? {}", result == 0.3);  // false!

    // Cách so sánh đúng: dùng epsilon
    let epsilon = 1e-10;
    println!("Gần bằng 0.3? {}", (result - 0.3).abs() < epsilon);  // true

    println!("pi={}, temp={}", pi, temperature);
    // Output:
    // 0.1 + 0.2 = 0.30000000000000004
    // 0.1 + 0.2 == 0.3? false
    // Gần bằng 0.3? true
    // pi=3.14159, temp=36.5
}
```

> **💡 Tip**: Không dùng `f64` cho tiền tệ! Dùng integer (đơn vị: đồng/cent) hoặc crate `rust_decimal`. `35_000u64` = 35,000đ.

### Boolean & Char

```rust
// filename: src/main.rs
fn main() {
    let is_active: bool = true;
    let is_admin = false;

    // bool dùng trong if, match, && || !
    let can_edit = is_active && is_admin;
    println!("Can edit? {}", can_edit);  // false

    // char = 1 Unicode scalar value (4 bytes!)
    let letter = 'A';
    let emoji = '🦀';
    let vietnamese = 'ệ';
    println!("{} {} {} — all chars!", letter, emoji, vietnamese);

    // Output:
    // Can edit? false
    // A 🦀 ệ — all chars!
}
```

---

## 5.3 — Compound Types: Gom nhiều giá trị

### Tuples — Nhóm cố định, khác kiểu

**Tuple** gom nhiều giá trị **khác kiểu** vào một nhóm. Có kích thước cố định.

```rust
// filename: src/main.rs
fn main() {
    // Tuple: (kiểu1, kiểu2, kiểu3)
    let order: (&str, u32, bool) = ("Coffee", 35_000, true);

    // Truy cập bằng .0, .1, .2 (index)
    println!("Drink: {}, Price: {}đ, Paid: {}", order.0, order.1, order.2);

    // Destructuring — tách tuple thành biến riêng
    let (drink, price, paid) = order;
    println!("{}: {}đ (paid: {})", drink, price, paid);

    // Tuple 1 phần tử = cần dấu phẩy
    let single = (42,);  // tuple chứa 1 số
    let not_tuple = (42); // chỉ là số 42 trong ngoặc!
    println!("single: {:?}, not_tuple: {}", single, not_tuple);

    // Unit type () = tuple rỗng = "không có gì"
    let nothing: () = ();
    println!("Unit: {:?}", nothing);

    // Output:
    // Drink: Coffee, Price: 35000đ, Paid: true
    // Coffee: 35000đ (paid: true)
    // single: (42,), not_tuple: 42
    // Unit: ()
}
```

### Functions trả về nhiều giá trị bằng tuple

```rust
// filename: src/main.rs

// Trả về tuple — cách Rust trả nhiều giá trị
fn min_max(items: &[i32]) -> (i32, i32) {
    let mut min = items[0];
    let mut max = items[0];
    for &item in &items[1..] {
        if item < min { min = item; }
        if item > max { max = item; }
    }
    (min, max)
}

fn main() {
    let data = vec![3, 7, 1, 9, 4];
    let (min, max) = min_max(&data);
    println!("Min: {}, Max: {}", min, max);
    // Output: Min: 1, Max: 9
}
```

### Arrays — Nhóm cố định, cùng kiểu

**Array** chứa nhiều giá trị **cùng kiểu**, kích thước **cố định tại compile time**:

```rust
// filename: src/main.rs
fn main() {
    // Array: [kiểu; kích_thước]
    let days: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let zeros = [0; 5];  // [0, 0, 0, 0, 0] — lặp giá trị

    println!("Days: {:?}", days);
    println!("First day: {}", days[0]);
    println!("Length: {}", days.len());
    println!("Zeros: {:?}", zeros);

    // ⚠️ Array có kích thước cố định — CỐ ĐỊNH
    // let mut arr = [1, 2, 3];
    // arr.push(4);  // ❌ Không có push! Dùng Vec nếu cần grow

    // Slices — "mượn" một phần array
    let weekdays = &days[0..5]; // Mon-Fri
    let weekend = &days[5..7];  // Sat-Sun
    println!("Weekdays: {:?}", weekdays);
    println!("Weekend: {:?}", weekend);

    // Output:
    // Days: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
    // First day: Mon
    // Length: 7
    // Zeros: [0, 0, 0, 0, 0]
    // Weekdays: ["Mon", "Tue", "Wed", "Thu", "Fri"]
    // Weekend: ["Sat", "Sun"]
}
```

### Array vs Vec — Khi nào dùng gì?

| | Array `[T; N]` | Vec `Vec<T>` |
|---|---|---|
| Kích thước | Cố định (compile time) | Động (runtime) |
| Lưu ở đâu | Stack (nhanh) | Heap (linh hoạt) |
| push/pop? | ❌ Không | ✅ Có |
| Khi nào dùng | Biết trước số lượng (7 ngày, 12 tháng, RGB) | Không biết trước (danh sách orders) |

---

## ✅ Checkpoint 5.3

> Ghi nhớ:
> 1. **Tuple** `(a, b, c)` = nhiều kiểu, cố định, destructure bằng `let (x, y, z) = tuple`
> 2. **Array** `[T; N]` = cùng kiểu, kích thước cố định. Slice `&[T]` "mượn" một phần
> 3. **Vec** khi cần kích thước động. **Array** khi biết trước
>
> **Test nhanh**: `let pair = (true, 42);` — `pair.0` giá trị gì, kiểu gì?
> <details><summary>Đáp án</summary><code>true</code>, kiểu <code>bool</code>. Tuple truy cập bằng <code>.index</code>.</details>

---

## 5.4 — Type Inference & Casting

### Type Inference — Rust đoán kiểu cho bạn

```rust
// filename: src/main.rs
fn main() {
    let x = 42;           // Rust đoán: i32
    let y = 3.14;         // Rust đoán: f64
    let active = true;    // Rust đoán: bool
    let name = "Rust";    // Rust đoán: &str

    // Rust đoán từ ngữ cảnh
    let numbers: Vec<i32> = vec![1, 2, 3]; // ghi rõ kiểu cho Vec
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    //                ^  _ = "Rust ơi, tự đoán kiểu element đi"
    println!("{:?}", doubled);  // [2, 4, 6]

    // Khi nào PHẢI ghi rõ kiểu?
    // 1. Rust không đủ thông tin để đoán
    let parsed = "42".parse::<i32>().unwrap();  // turbofish ::<i32>
    // let parsed = "42".parse().unwrap();  // ❌ Đoán không ra parse thành gì

    // 2. Muốn kiểu khác mặc định
    let small: i8 = 42;     // muốn i8 thay vì i32
    let big: u64 = 42;      // muốn u64

    println!("parsed={} small={} big={}", parsed, small, big);
}
```

### Type Casting — Chuyển đổi rõ ràng với `as`

Rust **không** tự chuyển kiểu (no implicit coercion). Phải dùng `as`:

```rust
// filename: src/main.rs
fn main() {
    let x: i32 = 42;
    let y: f64 = x as f64;    // i32 → f64: OK, không mất data
    let z: i32 = 3.99_f64 as i32;  // f64 → i32: CẮT phần thập phân!
    println!("x={} y={} z={}", x, y, z);
    // Output: x=42 y=42 z=3  (3.99 → 3, KHÔNG làm tròn!)

    // ⚠️ Cẩn thận: casting có thể mất data
    let big: i32 = 300;
    let small: u8 = big as u8;  // 300 không fit u8 (max 255)!
    println!("300 as u8 = {}", small);
    // Output: 300 as u8 = 44  (300 % 256 = 44, overflow wrap!)

    // Cách an toàn: dùng try_from
    match u8::try_from(300_i32) {
        Ok(val) => println!("OK: {}", val),
        Err(e) => println!("Error: {}", e),
    }
    // Output: Error: out of range integral type conversion attempted
}
```

> **💡 Quy tắc**: `as` nhanh nhưng **có thể mất data**. Dùng `TryFrom`/`TryInto` khi cần an toàn — trả `Result` thay vì âm thầm sai.

---

## 5.5 — Operators

### Arithmetic & Comparison

```rust
// filename: src/main.rs
fn main() {
    // Arithmetic
    println!("10 + 3 = {}", 10 + 3);    // 13
    println!("10 - 3 = {}", 10 - 3);    // 7
    println!("10 * 3 = {}", 10 * 3);    // 30
    println!("10 / 3 = {}", 10 / 3);    // 3 (integer division!)
    println!("10 % 3 = {}", 10 % 3);    // 1 (remainder)
    println!("10.0 / 3.0 = {:.2}", 10.0 / 3.0);  // 3.33 (float division)

    // Comparison — trả bool
    println!("5 == 5: {}", 5 == 5);     // true
    println!("5 != 3: {}", 5 != 3);     // true
    println!("5 > 3: {}", 5 > 3);       // true
    println!("5 <= 5: {}", 5 <= 5);     // true

    // Logical
    println!("true && false: {}", true && false); // false
    println!("true || false: {}", true || false); // true
    println!("!true: {}", !true);                 // false

    // ⚠️ Không thể so sánh khác kiểu!
    // println!("{}", 5_i32 == 5_i64);  // ❌ mismatched types
    println!("{}", 5_i32 == 5);         // ✅ cùng kiểu
}
```

### Range operators

```rust
// filename: src/main.rs
fn main() {
    // Range: start..end (exclusive end)
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();  // 0 1 2 3 4

    // Range inclusive: start..=end
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();  // 1 2 3 4 5

    // Range trong slicing
    let data = [10, 20, 30, 40, 50];
    println!("data[1..4] = {:?}", &data[1..4]);   // [20, 30, 40]
    println!("data[..3] = {:?}", &data[..3]);      // [10, 20, 30]
    println!("data[2..] = {:?}", &data[2..]);      // [30, 40, 50]
}
```

---

## 5.6 — Constants vs `let`

```rust
// filename: src/main.rs

// const: biết lúc compile, PHẢI ghi kiểu, naming = SCREAMING_SNAKE_CASE
const MAX_ORDERS: u32 = 1_000;
const TAX_RATE: f64 = 0.08;

// static: giống const nhưng có địa chỉ bộ nhớ cố định
// Hiếm dùng — chỉ khi cần reference lâu dài
static APP_NAME: &str = "Cafe System";

fn main() {
    println!("{}: max {} orders, tax {}%",
        APP_NAME, MAX_ORDERS, TAX_RATE * 100.0);

    let price = 35_000;
    let total = price as f64 * (1.0 + TAX_RATE);
    println!("Price: {}đ → Total with tax: {:.0}đ", price, total);

    // Output:
    // Cafe System: max 1000 orders, tax 8%
    // Price: 35000đ → Total with tax: 37800đ
}
```

| | `let` | `const` | `static` |
|---|---|---|---|
| Ghi kiểu | Tùy chọn (inference) | **Bắt buộc** | **Bắt buộc** |
| Mutable? | Có `mut` | ❌ Không bao giờ | Có `mut` (unsafe) |
| Scope | Trong block | Global | Global |
| Khi nào dùng | Hầu hết | Config values, limits | Hiếm — shared global state |

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Đọc code — biến nào gì, kiểu gì?

```rust
let a = 42;
let b: u8 = 255;
let c = 3.14;
let d = true;
let e = 'x';
let f = (1, "hello", true);
let g = [0; 10];
let h = "world";
```

<details><summary>✅ Lời giải Bài 1</summary>

```
a: i32 (mặc định)
b: u8 (ghi rõ)
c: f64 (mặc định cho float)
d: bool
e: char
f: (i32, &str, bool) — tuple 3 phần tử
g: [i32; 10] — array 10 phần tử, tất cả = 0
h: &str — string slice
```

</details>

---

**Bài 2** (10 phút): Viết function `bmi_calculator`

Nhận chiều cao (cm) và cân nặng (kg), trả về tuple `(f64, &str)` gồm BMI và đánh giá:
- BMI < 18.5 → "Underweight"
- 18.5 ≤ BMI < 25 → "Normal"
- 25 ≤ BMI < 30 → "Overweight"
- BMI ≥ 30 → "Obese"

<details><summary>💡 Gợi ý</summary>Công thức: BMI = weight / (height_m²). height_m = height_cm / 100.0</details>

<details><summary>✅ Lời giải Bài 2</summary>

```rust
// filename: src/main.rs

fn bmi_calculator(height_cm: f64, weight_kg: f64) -> (f64, &'static str) {
    let height_m = height_cm / 100.0;
    let bmi = weight_kg / (height_m * height_m);

    let category = if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Normal"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obese"
    };

    (bmi, category)
}

fn main() {
    let people = [
        ("Minh", 170.0, 65.0),
        ("Lan", 160.0, 45.0),
        ("Hùng", 175.0, 95.0),
    ];

    for (name, height, weight) in &people {
        let (bmi, category) = bmi_calculator(*height, *weight);
        println!("{}: BMI = {:.1} → {}", name, bmi, category);
    }
    // Output:
    // Minh: BMI = 22.5 → Normal
    // Lan: BMI = 17.6 → Underweight
    // Hùng: BMI = 31.0 → Obese
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_bmi() {
        let (bmi, cat) = bmi_calculator(170.0, 65.0);
        assert!((bmi - 22.49).abs() < 0.1);
        assert_eq!(cat, "Normal");
    }
}
```

</details>

---

**Bài 3** (15 phút): Grade report

Viết chương trình nhận array điểm `[f64; 5]` đại diện 5 môn. Tính: trung bình, điểm cao nhất, điểm thấp nhất, xếp loại (≥8.0: Giỏi, ≥6.5: Khá, ≥5.0: Trung bình, <5.0: Yếu). Dùng tuples, shadowing, `match`/`if`.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs

fn grade_report(scores: [f64; 5]) -> (f64, f64, f64, &'static str) {
    let sum: f64 = scores.iter().sum();
    let average = sum / scores.len() as f64;

    let min = scores.iter().cloned().reduce(f64::min).unwrap();
    let max = scores.iter().cloned().reduce(f64::max).unwrap();

    let rank = if average >= 8.0 {
        "Giỏi"
    } else if average >= 6.5 {
        "Khá"
    } else if average >= 5.0 {
        "Trung bình"
    } else {
        "Yếu"
    };

    (average, min, max, rank)
}

fn main() {
    let scores = [8.5, 7.0, 9.0, 6.5, 8.0];
    let (avg, min, max, rank) = grade_report(scores);

    println!("📊 Grade Report");
    println!("Scores: {:?}", scores);
    println!("Average: {:.1}", avg);
    println!("Min: {:.1}, Max: {:.1}", min, max);
    println!("Rank: {}", rank);
    // Output:
    // 📊 Grade Report
    // Scores: [8.5, 7.0, 9.0, 6.5, 8.0]
    // Average: 7.8
    // Min: 6.5, Max: 9.0
    // Rank: Khá
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `cannot assign twice to immutable variable` | Sửa biến không có `mut` | Thêm `mut` hoặc dùng shadowing |
| `mismatched types` | Trộn kiểu (i32 vs i64, i32 vs f64) | Dùng `as` để cast hoặc ghi rõ kiểu |
| `literal out of range for u8` | Giá trị > 255 cho u8 | Dùng kiểu lớn hơn (u16, u32) |
| `0.1 + 0.2 != 0.3` | Floating point precision | So sánh bằng epsilon: `(a - b).abs() < 1e-10` |
| `type annotations needed` | Rust không đoán được kiểu | Ghi rõ: `let x: i32 = ...` hoặc turbofish `::<Type>` |

---

## Tóm tắt

- ✅ **`let` = immutable** mặc định. `let mut` = mutable. Rust ép bạn suy nghĩ trước khi cho phép thay đổi.
- ✅ **Shadowing** = `let x` lại → biến mới, có thể đổi kiểu. Khác với mutation.
- ✅ **Scalars**: `i32`/`u32` (mặc định), `f64` (mặc định float), `bool`, `char` (Unicode 4 bytes).
- ✅ **Compounds**: Tuple `(A, B, C)` = nhiều kiểu, cố định. Array `[T; N]` = cùng kiểu, cố định. Vec cho dynamic.
- ✅ **Type inference** thông minh nhưng đôi khi cần ghi rõ. `as` cast explicit — cẩn thận mất data.

## Tiếp theo

→ Chapter 6: **Control Flow** — bạn sẽ học `if/else` là expression (trả giá trị!), `match` exhaustive, `loop`/`while`/`for`, và pattern matching — kỹ năng nền tảng cho toàn bộ phần FP sau này.
