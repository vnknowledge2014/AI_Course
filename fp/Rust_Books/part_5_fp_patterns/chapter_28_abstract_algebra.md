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

Chapter này có thể khiến bạn sợ vì tên gọi "abstract algebra" (Đại số trừu tượng). Khái niệm này nghe như một môn học khô khan trên giảng đường đại học. Nhưng đừng lo — sự thật là bạn đã dùng abstract algebra mỗi ngày mà không hề nhận ra! 

Mỗi lần bạn dùng `vec.iter().sum()`, bạn đang dùng một pattern có tên là **Monoid**. Mỗi lần bạn gọi `.map()`, bạn đang tận dụng **Functor**. Mỗi lần bạn nối chuỗi các `.and_then()`, bạn đang làm việc với **Monad**.

Chapter này không dạy bạn toán học hàn lâm. Nó đơn giản là "đặt tên" cho những khuôn mẫu (patterns) mà bạn đã sử dụng. Tại sao việc đặt tên lại quan trọng? Bởi vì khi bạn nhận ra một cấu trúc dữ liệu là một Monoid, bạn ngay lập tức biết rằng nó có thể được `fold`, có thể `concat`, và đặc biệt là có thể chia nhỏ ra để chạy song song (parallelize) một cách an toàn. Bạn được hưởng lợi từ những tính chất này hoàn toàn miễn phí, không cần phát minh lại bánh xe.

---

## 28.1 — Semigroup: "Kết hợp hai thứ cùng loại"

### Ẩn dụ: Trộn màu

Hãy tưởng tượng bạn đang vẽ tranh. Trộn màu đỏ với màu xanh, bạn được màu tím. Lấy màu tím trộn tiếp với màu vàng, bạn lại được một màu mới. 

Điều quan trọng nhất ở đây là gì? Đó là **đầu vào và đầu ra luôn CÙNG MỘT LOẠI**. Bạn lấy hai màu, trộn lại, và kết quả vẫn là một màu (chứ không biến thành một cái cọ hay một bức canvas). Đây chính là cốt lõi của Semigroup.

### Định nghĩa

**Semigroup** đơn giản là một kiểu dữ liệu `T` có khả năng cung cấp một hành động `combine(a: T, b: T) -> T`. Hành động này phải thỏa mãn tính **kết hợp (associativity)**:

```rust
// Bạn gom nhóm thế nào cũng được, kết quả cuối cùng vẫn giống nhau
combine(combine(a, b), c) == combine(a, combine(b, c))
```

Nó giống hệt phép cộng cơ bản mà chúng ta học cấp 1: `(1 + 2) + 3` thì cũng bằng `1 + (2 + 3)`.

### Viết Code Semigroup

Thay vì ném toàn bộ code vào mặt bạn, chúng ta hãy đi từng bước. Đầu tiên, hãy định nghĩa Trait:

```rust
// filename: src/main.rs

// ═══ Semigroup trait ═══
trait Semigroup {
    fn combine(self, other: Self) -> Self;
}
```

Bây giờ, hãy thử "trộn" hai chuỗi String. Kết hợp hai chuỗi rõ ràng là nối chúng lại với nhau:

```rust
// String: combine = concatenation
impl Semigroup for String {
    fn combine(self, other: Self) -> Self {
        format!("{}{}", self, other)
    }
}
```

Tương tự, làm sao để "trộn" hai cái mảng (`Vec`)? Rất tự nhiên, ta append cái này vào sau cái kia:

```rust
// Vec<T>: combine = append
impl<T> Semigroup for Vec<T> {
    fn combine(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}
```

Và với số nguyên (`i64`), chúng ta có thể định nghĩa phép "trộn" là phép cộng:

```rust
// i64: combine = addition
impl Semigroup for i64 {
    fn combine(self, other: Self) -> Self {
        self + other
    }
}
```

Một trường hợp thú vị hơn là `Option`. Làm sao để kết hợp hai giá trị có thể tồn tại hoặc không? Luật khá đơn giản: Nếu cả hai đều có giá trị, ta `combine` giá trị bên trong (nhờ ràng buộc `T: Semigroup`). Nếu chỉ có một, ta giữ nó. Nếu cả hai đều None, kết quả là None.

```rust
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
```

Chạy thử nào:

```rust
fn main() {
    let hello = "Hello ".to_string().combine("World!".to_string());
    println!("{}", hello); // "Hello World!"

    let sum = 10_i64.combine(20);
    println!("{}", sum); // 30

    let partial = Some("Hello".to_string()).combine(None);
    println!("{:?}", partial); // Some("Hello")
}
```

Tuyệt vời! Chúng ta vừa định nghĩa một khái niệm thống nhất cho hành động "kết hợp" của hàng loạt kiểu dữ liệu khác nhau.

---

## 28.2 — Monoid: Semigroup + Điểm khởi đầu

### Định nghĩa

Vậy Monoid là gì? **Monoid** chỉ đơn giản là một Semigroup nhưng được nâng cấp thêm một "điểm khởi đầu" — hay thuật ngữ gọi là **phần tử đơn vị (identity element)**.

Phần tử đơn vị này có tính chất ma thuật: Khi bạn kết hợp bất kỳ giá trị `a` nào với nó, kết quả vẫn chính là `a`:

```rust
combine(a, empty) == a
combine(empty, a) == a
```

Ví dụ thực tế: `x + 0 = x`, `chuỗi + "" = chuỗi`, `mảng ++ [] = mảng`. 
Vậy `0`, `""`, và `[]` chính là các phần tử đơn vị (empty).

Hãy xem định nghĩa Trait:

```rust
trait Monoid: Semigroup {
    fn empty() -> Self;
}
```

Và việc implement nó cực kỳ dễ dàng khi chúng ta đã có Semigroup:

```rust
impl Monoid for String {
    fn empty() -> Self { String::new() } // Chuỗi rỗng
}

impl<T> Monoid for Vec<T> {
    fn empty() -> Self { Vec::new() } // Mảng rỗng
}

impl Monoid for i64 {
    fn empty() -> Self { 0 } // Số không
}

impl Monoid for bool {
    fn empty() -> Self { true }  // true && x == x
}
```

### Tại sao Monoid lại đáng quan tâm?

Hãy tưởng tượng bạn có một danh sách khổng lồ các giá trị và bạn muốn gộp tất cả chúng lại thành một. Nếu chỉ có Semigroup, bạn bế tắc khi danh sách trống rỗng! (Bạn không biết lấy giá trị nào để bắt đầu).
Nhưng với Monoid, bạn luôn có một điểm xuất phát (`empty()`), do đó bạn có thể viết một hàm gộp (reduce/fold) cực kỳ tổng quát, hoạt động cho BẤT KỲ kiểu dữ liệu nào.

```rust
// Hàm này có thể nối String, cộng i64, gộp Vec, hoặc AND boolean!
fn concat_all<M: Monoid>(items: Vec<M>) -> M {
    items.into_iter().fold(M::empty(), |acc, x| acc.combine(x))
}

fn main() {
    let words = vec!["Hello ".to_string(), "Functional ".into(), "World!".into()];
    println!("{}", concat_all(words)); // Nối chuỗi

    let numbers: Vec<i64> = vec![10, 20, 30, 40];
    println!("Sum: {}", concat_all(numbers)); // Cộng tổng

    let flags = vec![true, true, true, false];
    println!("All true? {}", concat_all(flags)); // AND logic
}
```

> **💡 Insight**: `concat_all` hoạt động cho **MỌI monoid**. Một function duy nhất, phục vụ vô số kiểu dữ liệu! Đó là sức mạnh của sự trừu tượng hóa toán học.

---

## ✅ Checkpoint 28.2

> Ghi nhớ:
> 1. **Semigroup** = `combine(a, b) -> same_type` + associative.
> 2. **Monoid** = Semigroup + `empty()` (identity element).
> 3. `reduce` = Dùng cho Semigroup (yêu cầu danh sách phải có ≥1 phần tử vì không có điểm khởi đầu). 
> 4. `fold(empty, combine)` = Dùng cho Monoid (chạy an toàn ngay cả với danh sách rỗng).

---

## 28.3 — Domain Monoids: Đưa toán học vào Business Logic

Lý thuyết như vậy là đủ rồi. Làm sao để áp dụng nó vào dự án thực tế của bạn?
Hãy nghĩ về một hệ thống e-commerce xử lý đơn hàng. Mỗi ngày bạn có hàng triệu đơn hàng. Cuối ngày, sếp yêu cầu bạn tạo ra một bản báo cáo tổng hợp.

Thay vì viết các vòng lặp lồng nhau phức tạp để cộng dồn doanh thu, đếm số lượng lỗi, v.v., bạn có thể thiết kế `OrderStats` như một Monoid. Việc tổng hợp dữ liệu giờ đây trở nên tự nhiên như hơi thở.

```rust
#[derive(Debug, Clone)]
struct OrderStats {
    total_orders: u32,
    revenue: u64,
    errors: u32,
}
```

Để hai bản thống kê "trộn" được vào nhau, ta định nghĩa Semigroup:

```rust
impl Semigroup for OrderStats {
    fn combine(self, other: Self) -> Self {
        OrderStats {
            total_orders: self.total_orders + other.total_orders,
            revenue: self.revenue + other.revenue,
            errors: self.errors + other.errors,
        }
    }
}
```

Và điểm khởi đầu (Monoid) dĩ nhiên là một bản thống kê toàn số 0:

```rust
impl Monoid for OrderStats {
    fn empty() -> Self {
        OrderStats { total_orders: 0, revenue: 0, errors: 0 }
    }
}
```

Sau khi thiết kế xong, quá trình tổng hợp (aggregation) từ hàng triệu đơn hàng thu gọn lại thành một dòng `fold`:

```rust
fn aggregate_stats(stats: Vec<OrderStats>) -> OrderStats {
    // 💡 Đẹp, an toàn, và có thể dễ dàng chạy song song bằng Rayon!
    stats.into_iter().fold(OrderStats::empty(), |acc, x| acc.combine(x))
}
```

---

## 28.4 — stdlib Monoids: Chúng ở khắp mọi nơi

Rust Standard Library không có sẵn trait `Monoid` (vì thiếu Higher-Kinded Types và một số lý do thiết kế). Tuy nhiên, **pattern** này thì có mặt ở khắp mọi nơi, chỉ là dưới các tên gọi khác nhau:

| Type | Semigroup operation | Monoid `empty` |
|------|--------------------|----------------|
| `String` | `+` (or `push_str`) | `""` |
| `Vec<T>` | `extend` | `[]` |
| `Option<T>` | `or` / `and` | `None` / `Some` |
| `Result<T, E>` | `or` / `and` | — |
| `HashMap<K, V: Semigroup>` | merge, combine values | `{}` empty map |
| `HashSet<T>` | `union` | `{}` empty set |
| `Duration` | `+` | `Duration::ZERO` |

> **💡 Pattern recognition**: Bất cứ khi nào bạn thấy cấu trúc `fold(initial, |acc, x| ...)` trong code → Chúc mừng, bạn vừa bắt gặp một Monoid! Trong đó `initial` đóng vai trò là `empty()`, còn closure chính là `combine()`.

---

## 28.5 — Newtype Wrappers: Giải quyết sự nhập nhằng

### Khi một kiểu dữ liệu có quá nhiều bản ngã

Hãy nghĩ về những con số. Nếu tôi đưa cho bạn số `3` và số `4`, yêu cầu bạn `combine` chúng lại. Bạn sẽ làm gì?
Bạn sẽ cộng chúng ra `7`, hay nhân chúng ra `12`? Cả hai phép toán `+` và `*` đều hợp lệ, đều có tính kết hợp (associative), và đều có điểm khởi đầu (`0` cho cộng, `1` cho nhân).

Sự thật là: Numbers có **2 monoids** khác biệt. Nếu chúng ta chỉ có thể implement trait `Monoid` một lần cho `i64`, chúng ta sẽ phải chọn một. Điều này dẫn đến giới hạn.

Giải pháp của Rust (và FP nói chung) là sử dụng **Newtype wrappers** (bọc kiểu cũ vào một cái vỏ mới) để chỉ định rõ ràng hành vi nào chúng ta muốn dùng.

### Ví dụ với Addition (Sum) và Multiplication (Product)

Chúng ta tạo ra hai cái vỏ mới: `Sum` đại diện cho phép cộng, và `Product` đại diện cho phép nhân.

```rust
// Sum monoid: 0 + x
#[derive(Debug, Clone, Copy)]
struct Sum(i64);

impl Semigroup for Sum {
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}
impl Monoid for Sum {
    fn empty() -> Self { Sum(0) } // Điểm khởi đầu của phép cộng là 0
}

// Product monoid: 1 * x
#[derive(Debug, Clone, Copy)]
struct Product(i64);

impl Semigroup for Product {
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}
impl Monoid for Product {
    fn empty() -> Self { Product(1) } // Điểm khởi đầu của phép nhân là 1
}
```

### Ứng dụng tương tự cho Booleans và Min/Max

Tương tự như con số, logic boolean cũng có hai monoid: AND (`All`) và OR (`Any`). Ta lại tạo wrappers cho chúng. Đồng thời, bài toán tìm giá trị nhỏ nhất/lớn nhất (`Min`, `Max`) cũng có thể biểu diễn dưới dạng Monoid!

```rust
// All (AND) monoid - Phải thỏa mãn TẤT CẢ
#[derive(Debug, Clone, Copy)]
struct All(bool);

impl Semigroup for All {
    fn combine(self, other: Self) -> Self { All(self.0 && other.0) }
}
impl Monoid for All {
    fn empty() -> Self { All(true) } // true AND x == x
}

// Any (OR) monoid - Chỉ cần MỘT CÁI đúng
#[derive(Debug, Clone, Copy)]
struct Any(bool);

impl Semigroup for Any {
    fn combine(self, other: Self) -> Self { Any(self.0 || other.0) }
}
impl Monoid for Any {
    fn empty() -> Self { Any(false) } // false OR x == x
}

// Max monoid - Tìm kẻ mạnh nhất
#[derive(Debug, Clone, Copy)]
struct Max(i64);

impl Semigroup for Max {
    fn combine(self, other: Self) -> Self { Max(self.0.max(other.0)) }
}
impl Monoid for Max {
    fn empty() -> Self { Max(i64::MIN) } // Giá trị nhỏ nhất là điểm xuất phát
}
```

Đến lúc xem cách chúng hoạt động trơn tru cùng nhau thông qua hàm `concat_all` thần thánh của chúng ta:

```rust
fn main() {
    let values = vec![3, 7, 2, 9, 5_i64];

    // Cùng một danh sách data, nhưng tuỳ vào "lăng kính" (Wrapper) ta áp vào, 
    // hàm concat_all sẽ hành xử khác nhau!
    let sum = concat_all(values.iter().map(|&x| Sum(x)).collect());
    let product = concat_all(values.iter().map(|&x| Product(x)).collect());
    let max = concat_all(values.iter().map(|&x| Max(x)).collect());

    println!("Sum: {}", sum.0);         // Sum: 26
    println!("Product: {}", product.0); // Product: 1890
    println!("Max: {}", max.0);         // Max: 9

    // Áp dụng với boolean
    let checks = vec![true, true, true, false];
    let all = concat_all(checks.iter().map(|&x| All(x)).collect());
    println!("All true? {}", all.0);    // false
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
5. ✅ **Monoid**: XOR is associative, empty = `false`. Vì `false XOR x = x`.

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

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Không viết được trait `Monoid` tổng quát | Rust thiếu Higher-Kinded Types | Định nghĩa trait cho từng kiểu cụ thể, hoặc dùng GAT với phạm vi hẹp |
| Orphan rule chặn `impl Trait for Vec<T>` | Cả trait lẫn kiểu đều ở crate khác | Bọc bằng newtype `struct MyVec<T>(Vec<T>)` |
| `fold` với `String` chậm | Cấp phát lại ở mỗi bước nối | Dùng `String::with_capacity` + `push_str`, hoặc `concat()`/`join()` |
| Fold song song ra kết quả khác | Phép toán không kết hợp (associative) | Kiểm lại luật; chỉ phép kết hợp mới song song hoá an toàn |
| `empty()` không phải phần tử đơn vị thật | Chọn nhầm giá trị | Kiểm bằng property test: `combine(x, empty()) == x` với mọi `x` |

## Tóm tắt

- ✅ **Semigroup** = `combine(a: T, b: T) -> T` + associative. "Kết hợp 2 values cùng type."
- ✅ **Monoid** = Semigroup + `empty()`. "Semigroup có điểm khởi đầu."
- ✅ **Rust std đầy monoids**: `String("", +)`, `Vec([], extend)`, `i64(0, +)`, `bool(true, &&)`.
- ✅ **`fold` = Monoid pattern**: `items.fold(M::empty(), |acc, x| acc.combine(x))`.
- ✅ **Domain monoids**: `OrderStats`, `LogSummary`, `InventoryStats` — combine business data tự nhiên.
- ✅ **Newtype wrappers**: `Sum`, `Product`, `All`, `Any`, `Min`, `Max` — same type, different behavior.

## Tiếp theo

→ Chapter 29: **Functors & Map in Rust** — bạn sẽ nhận ra `.map()` trên `Option`, `Result`, `Iterator` là **cùng 1 pattern**: Functor. Và tại sao Rust không abstract nó thành trait (hint: thiếu Higher-Kinded Types).
