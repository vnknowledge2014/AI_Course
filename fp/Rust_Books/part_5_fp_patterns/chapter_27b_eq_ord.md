# Chapter 27b — Đại số trừu tượng: Eq, PartialEq, Ord và PartialOrd

> **Bạn sẽ học được**:
> - Tại sao Rust chia việc "so sánh bằng" thành 2 traits: `PartialEq` và `Eq`.
> - Tại sao Rust chia việc "sắp xếp" thành 2 traits: `PartialOrd` và `Ord`.
> - Vấn đề kinh điển của số thập phân (`f32`, `f64` / `NaN`).
> - Cách sử dụng `#[derive]` để tự động hóa việc so sánh cấu trúc.
> - Cách tự `impl` (triển khai) logic sắp xếp tùy chỉnh để dùng cho `Vec::sort`.
>
> **Yêu cầu trước**: Hiểu về Trait (Ch12).
> **Thời gian đọc**: ~30 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Hiểu sâu sắc hệ thống typeclass so sánh của Rust. Không còn bất ngờ khi thấy lỗi compiler từ chối hàm `.sort()`.

---

Ở các chương trước, chúng ta thấy Python phải dùng thư viện/Protocol, và TypeScript phải dùng `Effect` để giải quyết bài toán so sánh cấu trúc và sắp xếp. 

Nhưng ở Rust, mọi thứ được **tích hợp thẳng vào cốt lõi của ngôn ngữ** thông qua hệ thống Trait (chính là Typeclasses trong Haskell). Trình biên dịch Rust siêu nghiêm ngặt: Nó không cho phép bạn gọi hàm `sort()` trên một mảng nếu kiểu dữ liệu đó không thỏa mãn các điều kiện toán học.

---

## 27b.1 — PartialEq và Eq: Sự tinh tế của Toán Học

### Tại sao lại cần hai Traits chỉ để so sánh bằng?

Theo toán học, một quan hệ Tương Đương (Equivalence Relation) phải thỏa mãn 3 tính chất:
1. **Phản xạ (Reflexivity)**: `a == a`
2. **Đối xứng (Symmetry)**: Nếu `a == b` thì `b == a`
3. **Bắc cầu (Transitivity)**: Nếu `a == b` và `b == c` thì `a == c`

Trait `Eq` yêu cầu thỏa mãn ĐỦ 3 tính chất trên.
Nhưng... có một kiểu dữ liệu kinh điển phá vỡ tính chất đầu tiên: **Số thực thập phân (Floating Point - f32, f64)**.

Theo chuẩn IEEE 754, giá trị `NaN` (Not a Number) **KHÔNG bằng chính nó**.
Tức là `f32::NAN == f32::NAN` trả về `false`! Tính phản xạ bị phá vỡ.

Vì vậy, Rust tạo ra **`PartialEq`** (Bằng nhau một phần):
- `PartialEq` chỉ yêu cầu tính Đối xứng và Bắc cầu. (f32 và f64 implement trait này).
- `Eq` là một trait rỗng (marker trait), kế thừa từ `PartialEq`, dùng để khẳng định với compiler: "Kiểu của tôi thỏa mãn cả tính Phản xạ!". Các số nguyên (`i32`, `u64`), `String` implement `Eq`.

### Sử dụng và Tự động hóa

Thông thường, bạn không tự viết logic so sánh từng trường. Bạn nhờ Macro `derive` làm hộ.

```rust
// filename: src/equality.rs

// Derive tự động tạo code kiểm tra từng field một từ trên xuống dưới.
#[derive(PartialEq, Eq, Debug)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let u1 = User { id: 1, name: String::from("Alice") };
    let u2 = User { id: 1, name: String::from("Alice") };
    let u3 = User { id: 2, name: String::from("Bob") };

    // So sánh cấu trúc (Structural Equality) hoạt động ngay lập tức!
    assert!(u1 == u2);
    assert!(u1 != u3);
    
    // Lưu ý về f32:
    #[derive(PartialEq)] // KHÔNG THỂ derive Eq vì có field f32!
    struct Point {
        x: f32,
        y: f32,
    }
    
    let p1 = Point { x: f32::NAN, y: 1.0 };
    assert!(p1 != p1); // Tính phản xạ bị phá vỡ!
}
```

---

## 27b.2 — PartialOrd và Ord: Sắp xếp an toàn

Cũng như sự khác biệt giữa `PartialEq` và `Eq`, ta có:
- **`Ord`**: Sắp xếp tuyệt đối (Total Order). Mọi cặp giá trị đều có thể so sánh được (lớn hơn, nhỏ hơn, hoặc bằng).
- **`PartialOrd`**: Sắp xếp một phần. Có một số giá trị không thể so sánh được với nhau. (Ví dụ: `NaN` so với `5.0` là vô nghĩa).

### Hệ quả trên mảng (Vec)

Hàm `vec.sort()` của Rust **YÊU CẦU** phần tử phải implement `Ord`. Điều này bảo vệ bạn khỏi những bug sort ngớ ngẩn ở runtime.

```rust
// filename: src/ordering.rs

fn main() {
    let mut numbers = vec![5, 1, 3, 2];
    numbers.sort(); // OK! i32 implement Ord.
    
    let mut floats = vec![5.1, 1.2, 3.3];
    // floats.sort(); 
    // ❌ LỖI BIÊN DỊCH: the trait `Ord` is not implemented for `{float}`.
    
    // Cách sửa để sort mảng f32:
    // Dùng sort_by và cung cấp hàm so sánh. Ta ép nó dùng f32::total_cmp.
    floats.sort_by(|a, b| a.total_cmp(b));
}
```

---

## 27b.3 — Triển khai (Impl) Custom Ordering

Điều gì xảy ra khi bạn muốn sắp xếp mảng `User`, nhưng chỉ muốn sắp xếp dựa trên `id` (nếu `name` khác nhau nhưng `id` giống nhau thì coi như bằng nhau)?

Bạn không thể dùng `#[derive(Ord)]` vì nó sẽ so sánh `id` trước, nếu trùng nó sẽ so sánh tiếp `name`. Bạn phải tự `impl`!

> **Quy tắc**: Để implement `Ord`, bạn phải implement `PartialOrd`, `Eq`, và `PartialEq`. Chúng liên kết chặt chẽ với nhau.

```rust
// filename: src/custom_ord.rs
use std::cmp::Ordering;

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
    salary: u32,
}

// 1. Impl PartialEq (Bắt buộc)
impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id // Chỉ so sánh ID
    }
}

// 2. Impl Eq (Marker trait)
impl Eq for Employee {}

// 3. Impl Ord (Bắt buộc)
impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        // Chỉ sắp xếp dựa trên ID
        self.id.cmp(&other.id)
    }
}

// 4. Impl PartialOrd (Bắt buộc, gọi ngược lại Ord)
impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut team = vec![
        Employee { id: 3, name: String::from("Charlie"), salary: 100 },
        Employee { id: 1, name: String::from("Alice"), salary: 500 },
        Employee { id: 2, name: String::from("Bob"), salary: 300 },
    ];

    // Sắp xếp tự động!
    team.sort();
    
    println!("{:#?}", team);
    // Kết quả: Alice (id 1) -> Bob (id 2) -> Charlie (id 3)
}
```

### Cách đơn giản hơn: `sort_by_key`

Trong nhiều trường hợp, bạn không muốn cứng hóa (hardcode) logic sort vào struct. Giống như `mapInput` trong Effect-TS, Rust cung cấp các hàm closure:

```rust
// filename: src/sort_by.rs

#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
}

fn main() {
    let mut catalog = vec![
        Product { name: String::from("Laptop"), price: 1000 },
        Product { name: String::from("Mouse"), price: 50 },
        Product { name: String::from("Keyboard"), price: 100 },
    ];

    // Sắp xếp theo giá (Price) - Truyền closure lấy ra key
    catalog.sort_by_key(|p| p.price);
    
    // Đảo ngược (Giảm dần) - dùng Reverse modifier
    use std::cmp::Reverse;
    catalog.sort_by_key(|p| Reverse(p.price));
    
    // Sắp xếp nhiều tiêu chí (Multi-criteria) giống Order.combine!
    // Trả về một Tuple, Rust sẽ so sánh Tuple từ trái qua phải.
    // VD: Giá giảm dần. Nếu trùng giá thì Tên tăng dần.
    catalog.sort_by_key(|p| (Reverse(p.price), p.name.clone()));
}
```

---

## ✅ Checkpoint 27b.1-27b.3

> Đến đây bạn phải hiểu:
> 1. **Toán học nghiêm ngặt**: Tại sao `NaN` khiến số thập phân không có tính chất `Eq` và `Ord`.
> 2. **Derive**: Macro mạnh mẽ tạo tự động logic so sánh cấu trúc.
> 3. **Hệ thống liên kết**: Để có `Ord`, phải có `PartialOrd`, `Eq`, `PartialEq`.
> 4. **Sort methods**: `.sort()` cần `Ord`. `.sort_by_key()` cần closure trả về thứ thỏa mãn `Ord`.
>
> **Test nhanh**: Để sắp xếp mảng các `Ticket` theo mức độ quan trọng `priority: u8`. Tôi có cần tự `impl Ord` cho cấu trúc Ticket không?
> <details><summary>Đáp án</summary>KHÔNG. Bạn chỉ cần dùng `tickets.sort_by_key(|t| t.priority)` là đủ, nhanh gọn, không dính líu tới việc phải override trait toàn cục.</details>

---

## 🏋️ Bài tập

**Bài 1** (10 phút): So sánh đa tiêu chí bằng Custom Ord.

```rust
// Cho struct Player
#[derive(Eq, PartialEq)]
struct Player {
    name: String,
    score: u32,
    level: u32,
}

// YÊU CẦU:
// Hãy implement `Ord` và `PartialOrd` cho Player sao cho:
// - So sánh Score (Điểm cao hơn lên trước / giảm dần).
// - Nếu trùng Score, so sánh Level (Level thấp hơn lên trước / tăng dần).
//
// Gợi ý: bạn có thể dùng `cmp` trên tuple. Ví dụ:
// `(other.score, self.level).cmp(&(self.score, other.level))`
```

<details><summary>✅ Lời giải Bài 1</summary>

```rust
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
struct Player {
    name: String,
    score: u32,
    level: u32,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        // So sánh tuple. 
        // Lấy other.score cmp self.score (để giảm dần)
        // Lấy self.level cmp other.level (để tăng dần)
        (other.score, self.level).cmp(&(self.score, other.level))
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut players = vec![
        Player { name: String::from("A"), score: 100, level: 5 },
        Player { name: String::from("B"), score: 100, level: 2 },
        Player { name: String::from("C"), score: 200, level: 10 },
    ];

    players.sort();
    
    // Kết quả: C (score 200) -> B (score 100, lvl 2) -> A (score 100, lvl 5)
}
```
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Báo lỗi "the trait bound `{float}: Ord` is not satisfied" | Gọi `.sort()` trên mảng f32/f64 | `float` chỉ có `PartialOrd`. Dùng `.sort_by(|a, b| a.total_cmp(b))` hoặc bọc nó lại bằng một struct tự định nghĩa (wrapper). |
| Báo lỗi "Conflicting implementations" khi `impl PartialEq` | Dùng cả `#[derive(PartialEq)]` lẫn tự viết khối `impl` | Xóa chữ `PartialEq` khỏi danh sách `#[derive(...)]`. |
| `.sort_by_key(|x| x.name)` bị lỗi Lifetime / Borrow | String không thực thi `Copy` | Trong `.sort_by_key`, khóa trả về phải sở hữu dữ liệu hoặc được mượn cẩn thận. Với String, thường bạn phải dùng `.clone()` hoặc chuyển sang dùng `.sort_by(|a, b| a.name.cmp(&b.name))` để tránh clone. |

---

## Tóm tắt

- ✅ **`PartialEq` & `Eq`**: Dùng để so sánh `==` và `!=`. Số thập phân chỉ có `Partial`.
- ✅ **`PartialOrd` & `Ord`**: Dùng để so sánh `>`, `<` và sắp xếp. Hàm `.sort()` bắt buộc cần `Ord`.
- ✅ **Tuples / Key functions**: Các closure như `sort_by_key` trả về một Tuple là một cách tuyệt vời để kết hợp (combine) nhiều tiêu chí sắp xếp.
- Hệ thống Typeclass của Rust ép bạn đối mặt với các vấn đề biên giới (như NaN) ngay tại lúc compile, loại bỏ hoàn toàn các lỗi sắp xếp tiềm ẩn.

## Tiếp theo

Đại số đã ổn định, chúng ta sẽ xem xét cách Rust xử lý Validation tập hợp (Applicative) — điều mà Monad (`Result::and_then`) thất bại, thông qua Chapter 31b!
