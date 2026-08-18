# Chapter 33 — TDD with Rust

> **Bạn sẽ học được**:
> - **Test fundamentals**: `#[test]`, `#[cfg(test)]`, `assert!`, `assert_eq!`, `assert_ne!`
> - **Test organization**: unit tests (in-file), integration tests (`tests/`)
> - **Red → Green → Refactor** cycle ⭐
> - **Edge cases**, error testing, `#[should_panic]`
> - `cargo test` options, filtering, output
> - Test-first workflow cho domain logic
>
> **Yêu cầu trước**: Chapter 10 (Error Handling), Chapter 22 (Domain Modeling).
> **Thời gian đọc**: ~40 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Bạn viết code **test-first** — test trước, implement sau.

---

## TDD — Test-Driven Development trong Rust

Cuốn *Learn Go with Tests* (một trong 4 cuốn sách truyền cảm hứng cho series này) dạy một bài học quan trọng: Viết test **TRƯỚC** khi viết code không chỉ là một kỷ luật — đó là **phương pháp thiết kế (Design Method)**. 

Khi bạn viết test trước, bạn đang tự hỏi mình: *"API của hàm này trông như thế nào từ góc nhìn của người dùng?"*. Bạn thiết kế Giao diện (Interface) trước, còn phần Triển khai (Implementation) tính sau. Kết quả: API tự nhiên hơn, code module hóa tốt hơn, và bạn luôn có một tấm lưới bảo vệ (safety net) vững chắc mỗi khi muốn dọn dẹp mã nguồn.

Rust đặc biệt phù hợp cho TDD: Trình biên dịch (Compiler) đã bắt hộ bạn hàng đống lỗi vặt (Type Error, Null Pointer, Memory Leak), do đó các bài Test của bạn có thể tập trung hoàn toàn vào **Nghiệp vụ (Business Logic)**. Công cụ `cargo test` được tích hợp sẵn, không cần cài đặt Framework rườm rà.

---

## 33.1 — Giải phẫu một bài Test trong Rust

Hãy xem cấu trúc chuẩn của một tệp có chứa Test.

```rust
// filename: src/lib.rs

// ═══ Production code (Code thật chạy trên Prod) ═══
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err("Division by zero".into()) }
    else { Ok(a / b) }
}
```

Và ngay bên dưới nó, trong cùng một file, ta định nghĩa một module tên là `tests`. Bằng thuộc tính `#[cfg(test)]`, ta báo cho Rust biết: **Chỉ biên dịch đoạn code này khi chạy lệnh test**. Khi build production (`cargo build --release`), nó sẽ bị bỏ qua hoàn toàn!

```rust
// ═══ Tests — chỉ compile trong test mode ═══
#[cfg(test)]
mod tests {
    use super::*; // Kéo tất cả các hàm ở trên (add, is_even...) vào module này

    #[test] // Báo cho cargo biết đây là 1 bài Test
    fn test_add() {
        assert_eq!(add(2, 3), 5); // Khẳng định 2 + 3 phải bằng 5
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(3));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10.0, 0.0), Err("Division by zero".into()));
    }
}
```

### Các hàm Assert (Khẳng định)

| Macro | Ý nghĩa | Ví dụ |
|-------|---------|-------|
| `assert!(expr)` | `expr` phải true | `assert!(is_even(4))` |
| `assert_eq!(a, b)` | `a == b` | `assert_eq!(add(1,2), 3)` |
| `assert_ne!(a, b)` | `a ≠ b` | `assert_ne!(add(1,2), 0)` |
| `assert!(expr, "msg")` | Tùy chỉnh thông báo lỗi | `assert!(x > 0, "x={} not positive", x)` |

### Run tests qua Command Line

```bash
cargo test                    # chạy tất cả tests trong dự án
cargo test test_add           # chỉ chạy tests có từ khóa "test_add" trong tên
cargo test -- --nocapture     # hiển thị các lệnh println! ra màn hình (mặc định bị ẩn)
cargo test -- --test-threads=1  # chạy từng test một (tuần tự, tránh xung đột IO)
```

---

## 33.2 — Red → Green → Refactor ⭐

Vòng lặp TDD (TDD Cycle) gồm 3 bước. Bạn phải tuân thủ đúng thứ tự này, như một điệu nhảy Walz.

```text
1. 🔴 RED (ĐỎ):      Viết test TRƯỚC → Chạy test → FAIL (báo lỗi màu đỏ vì chưa có code).
2. 🟢 GREEN (XANH):  Viết CODE cực kỳ NGU NGỐC (tối thiểu nhất có thể) để test PASS (hiện màu xanh).
3. 🔵 REFACTOR:      Dọn dẹp lại code cho đẹp, trong khi vẫn giữ màu XANH.
4. Lặp lại!
```

### Ví dụ: Xây dựng Cấu trúc dữ liệu `Stack<T>` (Ngăn xếp) step-by-step

**Round 1: Stack rỗng**

```rust
// filename: src/lib.rs

// 🔴 RED — Viết test trước!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }
}
// Chạy test lúc này sẽ báo lỗi: "Báo cáo anh, làm gì có cái Struct nào tên là Stack đâu!"
```

Giờ hãy viết code xanh (Green) tối giản nhất:

```rust
// 🟢 GREEN — Implement tối thiểu để test pass
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { Stack { items: vec![] } }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn len(&self) -> usize { self.items.len() }
}
```

**Round 2: Thêm (Push) và Nhìn (Peek)**

Lại quay về bước đỏ (Red), ta muốn thử nhét số 42 vào ngăn xếp.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // ... test cũ ...

    // 🔴 RED (Test Fail vì chưa có hàm push và peek)
    #[test]
    fn push_adds_element() {
        let mut stack = Stack::new();
        stack.push(42);
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn peek_returns_top() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2)); // Lấy ra số trên cùng (số 2)
    }
}
```

Và viết code để làm xanh lại:

```rust
impl<T> Stack<T> {
    // ... hàm cũ ...

    // 🟢 GREEN — thêm hàm push và peek
    pub fn push(&mut self, item: T) { self.items.push(item); }
    pub fn peek(&self) -> Option<&T> { self.items.last() }
}
```

**Round 3: Lấy ra (Pop)**

Tự bạn có thể đoán được chu trình này rồi chứ? Viết Test cho Pop → Thấy Đỏ → Viết hàm Pop → Thấy Xanh.

---

## 33.3 — Testing Domain Logic (Nghiệp vụ cốt lõi)

Bạn viết Test càng nhiều cho lớp Domain (các Pure Functions) thì ứng dụng càng vững chãi.
Chúng ta sẽ thiết kế một Value Object tên là `Money`.

Thay vì quăng cho bạn một khối code 100 dòng, chúng ta hãy chia nhỏ nó ra theo tư duy TDD.
Đầu tiên là khả năng Khởi tạo và báo lỗi nếu Tiền Âm:

```rust
// filename: src/lib.rs

// ═══ Domain: Money (Phần 1: Khởi tạo) ═══
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(i64);

impl Money {
    // Không cho phép khởi tạo tiền âm
    pub fn new(cents: i64) -> Result<Self, String> {
        if cents < 0 { Err("Money cannot be negative".into()) }
        else { Ok(Money(cents)) }
    }

    pub fn zero() -> Self { Money(0) }
    pub fn cents(&self) -> i64 { self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_money_success() {
        assert_eq!(Money::new(500).unwrap().cents(), 500);
    }

    #[test]
    fn reject_negative_money() {
        assert!(Money::new(-100).is_err());
    }
}
```

Tiếp theo là khả năng Cộng và Trừ tiền. Phép trừ không được làm tiền trở nên âm!

```rust
// ═══ Domain: Money (Phần 2: Tính toán) ═══
impl Money {
    // ... code phần 1

    pub fn add(&self, other: &Money) -> Money { 
        Money(self.0 + other.0) 
    }

    pub fn subtract(&self, other: &Money) -> Result<Money, String> {
        if other.0 > self.0 {
            Err(format!("Insufficient: {}¢ - {}¢", self.0, other.0))
        } else {
            Ok(Money(self.0 - other.0))
        }
    }
}

#[cfg(test)]
mod math_tests {
    use super::*;

    #[test]
    fn add_money() {
        let a = Money::new(300).unwrap();
        let b = Money::new(200).unwrap();
        assert_eq!(a.add(&b), Money::new(500).unwrap());
    }

    #[test]
    fn subtract_insufficient() {
        let a = Money::new(100).unwrap();
        let b = Money::new(500).unwrap();
        assert!(a.subtract(&b).is_err());
    }
}
```

Cuối cùng là khả năng Tính giảm giá (Discount) và Định dạng hiển thị (Display):

```rust
// ═══ Domain: Money (Phần 3: Nghiệp vụ & Hiển thị) ═══
impl Money {
    // ... code phần 2

    pub fn discount(&self, percent: u32) -> Money {
        Money(self.0 * (100 - percent as i64) / 100)
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}đ", self.0 / 100, self.0 % 100) // Đổi từ Cent ra Đồng
    }
}

#[cfg(test)]
mod feature_tests {
    use super::*;

    #[test]
    fn discount_10_percent() {
        let price = Money::new(1000).unwrap();
        assert_eq!(price.discount(10), Money::new(900).unwrap());
    }

    #[test]
    fn display_format() {
        // format! sẽ gọi trait Display
        assert_eq!(format!("{}", Money::new(12345).unwrap()), "123.45đ");
    }
}
```

---

## 33.4 — Kiểm thử Lỗi và Biên (Errors & Edge Cases)

Happy Path (trường hợp tốt đẹp) luôn dễ viết. Sức mạnh của Test nằm ở việc lôi ra được những trường hợp vỡ mặt (Edge Cases).

```rust
// filename: src/lib.rs

pub fn parse_age(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() { return Err("Age is required".into()); }
    
    let age: u32 = trimmed.parse().map_err(|_| format!("'{}' is not a number", trimmed))?;
    
    if age < 1 || age > 150 { return Err(format!("Age {} out of range 1-150", age)); }
    Ok(age)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Happy paths
    #[test] fn parse_age_valid() { assert_eq!(parse_age("25"), Ok(25)); }
    #[test] fn parse_age_with_spaces() { assert_eq!(parse_age("  30  "), Ok(30)); }
    
    // Boundary conditions (Kiểm tra biên)
    #[test] fn parse_age_boundary_low() { assert_eq!(parse_age("1"), Ok(1)); }
    #[test] fn parse_age_boundary_high() { assert_eq!(parse_age("150"), Ok(150)); }

    // Error paths (Cố tình phá hoại)
    #[test] fn parse_age_empty() { assert!(parse_age("").is_err()); }
    #[test] fn parse_age_not_number() { assert!(parse_age("abc").is_err()); }
    #[test] fn parse_age_negative() { assert!(parse_age("-5").is_err()); }
    #[test] fn parse_age_zero() { assert!(parse_age("0").is_err()); }
    #[test] fn parse_age_too_high() { assert!(parse_age("151").is_err()); }
    #[test] fn parse_age_float() { assert!(parse_age("25.5").is_err()); }
}
```

### Bắt lỗi `Panic`

Đôi khi hàm của bạn cố tình làm Crash hệ thống (Panic) khi nhận đầu vào sai. Làm sao test được "Crash"?
Hãy dùng thuộc tính `#[should_panic]`:

```rust
// filename: src/lib.rs

pub fn first_element(list: &[i32]) -> i32 {
    list[0]  // panics nếu list rỗng!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "index out of bounds")] // Báo cho Rust: Hàm này chạy MÀ KHÔNG LỖI LÀ SAI!
    fn first_element_panics_on_empty() {
        first_element(&[]);
    }
}
```

---

## 33.5 — Tổ chức thư mục Test

Rust phân biệt 2 loại Test: **Unit Tests** và **Integration Tests**.

### Unit tests (Kiểm thử chức năng nhỏ)
Như nãy giờ chúng ta làm, nó nằm ngay **bên trong** file mã nguồn `src/lib.rs` (hoặc các file module tương ứng).
Nó có quyền test luôn cả những hàm `private` (không có chữ `pub`).

### Integration tests (Kiểm thử tích hợp)
Bạn tạo hẳn một thư mục tên là `tests/` nằm ngang hàng với `src/`.
Trong thư mục này, code test đóng vai trò như một Người dùng bên thứ 3 tải Crate (Thư viện) của bạn về dùng. Nó CHỈ ĐƯỢC PHÉP gọi các hàm `pub`.

```text
my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs                 ← Unit tests nằm luôn ở đây
│   └── domain/
│       └── order.rs           ← Unit tests nằm luôn ở đây
└── tests/                     ← THƯ MỤC INTEGRATION TESTS
    ├── order_workflow_test.rs
    └── payment_test.rs
```

---

## 33.6 — Table-Driven Tests (Test hàng loạt)

Khi một hàm có quá nhiều nhánh input (như bài toán FizzBuzz), thay vì viết 10 cái `#[test]` lẻ tẻ, ta dùng mảng dữ liệu (Table).

```rust
// filename: src/lib.rs

pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".into(),
        (0, _) => "Fizz".into(),
        (_, 0) => "Buzz".into(),
        _ => n.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_table() {
        // (Đầu vào, Kết quả mong đợi)
        let cases = vec![
            (1, "1"),
            (3, "Fizz"),
            (5, "Buzz"),
            (15, "FizzBuzz"),
            (30, "FizzBuzz"),
            (7, "7"),
            (10, "Buzz"),
        ];

        // Lặp qua mảng và test
        for (input, expected) in cases {
            assert_eq!(fizzbuzz(input), expected, "fizzbuzz({}) failed", input);
        }
    }
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Viết Test Trước (TDD)

Dùng TDD viết hàm `fn reverse_string(s: &str) -> String` sao cho:
- `"hello"` → `"olleh"`
- `""` → `""`
- `"a"` → `"a"`
- Ký tự có dấu (Unicode): `"xin chào"` → `"oàhc nix"`

<details><summary>✅ Lời giải</summary>

```rust
// Viết Test TRƯỚC!
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn reverse_hello() { assert_eq!(reverse_string("hello"), "olleh"); }
    #[test] fn reverse_empty() { assert_eq!(reverse_string(""), ""); }
    #[test] fn reverse_single() { assert_eq!(reverse_string("a"), "a"); }
    #[test] fn reverse_unicode() { assert_eq!(reverse_string("xin chào"), "oàhc nix"); }
}

// Giờ mới viết Code
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
```

</details>

---

**Bài 2** (10 phút): TDD Calculator

Dùng tư duy Red→Green→Refactor, tạo Struct `Calculator`:
1. `new()` → value = 0
2. `add(n)` → cộng n
3. `subtract(n)` → trừ n
4. `multiply(n)` → nhân n
5. `result()` → trả giá trị hiện tại
6. `reset()` → về 0

Luật: Viết 1 Test -> Báo đỏ -> Viết Code cho Test đó xanh -> Lặp lại.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
struct Calculator { value: f64 }

impl Calculator {
    fn new() -> Self { Calculator { value: 0.0 } }
    fn add(&mut self, n: f64) { self.value += n; }
    fn subtract(&mut self, n: f64) { self.value -= n; }
    fn multiply(&mut self, n: f64) { self.value *= n; }
    fn result(&self) -> f64 { self.value }
    fn reset(&mut self) { self.value = 0.0; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn starts_at_zero() { assert_eq!(Calculator::new().result(), 0.0); }
    #[test] fn add_numbers() {
        let mut c = Calculator::new();
        c.add(5.0); c.add(3.0);
        assert_eq!(c.result(), 8.0);
    }
    #[test] fn reset_to_zero() {
        let mut c = Calculator::new();
        c.add(100.0); c.reset();
        assert_eq!(c.result(), 0.0);
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Test pass trên máy tôi mà sao ném lên Server CI thì nó tạch?" | Code Test phụ thuộc vào Môi trường (Đọc file ở C:\, lấy thời gian hiện tại) | Tránh dùng IO, DateTime, Random trong Unit Test. Phải Mock chúng! |
| "Nhiều test quá chạy chậm như rùa" | Bạn đẩy hết mọi thứ thành Integration tests (chạy nặng nề) | Hãy dồn thật nhiều bài kiểm tra vào Unit Tests (chạy siêu nhanh), và giữ Integration Test mỏng thôi. |
| "Lệnh `println!` viết trong Test sao chạy không thấy in ra gì?" | Rust ngầm giấu output để màn hình gọn gàng. | Thêm flag: `cargo test -- --nocapture` |

---

---

## ✅ Checkpoint 33

1. Unit test trong Rust đặt cùng file với code (`#[cfg(test)]`), integration test đặt ở `tests/`. Khác biệt về **quyền truy cập** là gì?
2. `#[cfg(test)]` ảnh hưởng thế nào tới binary release?
3. Vì sao viết test trước lại thay đổi thiết kế API, chứ không chỉ thay đổi độ phủ?

<details>
<summary>Đáp án</summary>

1. Unit test nằm trong cùng module nên thấy được cả item **private**. Integration test ở `tests/` là crate riêng, chỉ thấy API **public** — nên nó kiểm luôn cả việc bạn có export đủ thứ cần thiết hay không.
2. Hoàn toàn không có mặt: `#[cfg(test)]` chỉ biên dịch khi chạy `cargo test`. Binary release không chứa một byte test nào.
3. Vì viết test trước buộc bạn **dùng** API trước khi cài đặt nó. Một API khó test hầu như luôn là API khó dùng — và bạn phát hiện điều đó lúc còn rẻ để sửa.
</details>

## Tóm tắt

- ✅ **`#[test]` + `assert_eq!`**: Cơ bản nhất — mỗi test là 1 chức năng (behavior).
- ✅ **Red → Green → Refactor**: Vòng lặp thần thánh của TDD. Test trước, Code tối giản sau, rồi dọn dẹp.
- ✅ **Test domain logic**: Logic nghiệp vụ mà gói trong Pure Functions thì cực kì dễ test.
- ✅ **Edge cases**: Hãy viết test để cố ý đập vỡ hệ thống bằng biên trị, hoặc bắt buộc phải báo lỗi với `#[should_panic]`.
- ✅ **Table-driven**: Cú pháp `vec![(input, expected)]` — Giúp code cực kì Gọn và DRY.

## Tiếp theo

→ Chapter 34: **Property-Based Testing** — Một đẳng cấp khác. Thay vì viết từng Ví dụ (như `"hello"` thành `"olleh"`), bạn mô tả **Luật** ("Đảo ngược 2 lần thì ra như cũ") → Trí tuệ nhân tạo (Framework) sẽ tự động tạo ra Hàng Ngàn Test Cases để oanh tạc code của bạn!
