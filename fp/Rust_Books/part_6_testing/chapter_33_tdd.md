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

Cuốn *Learn Go with Tests* (một trong 4 cuốn sách gốc của series này) dạy rằng: viết test **trước** code không chỉ giúp bắt bugs — nó thay đổi cách bạn thiết kế. Khi bạn viết test trước, bạn buộc phải suy nghĩ về API trước khi implement. Kết quả: API tự nhiên hơn, code modular hơn, và bạn luôn có safety net cho refactoring.

Rust đặc biệt phù hợp cho TDD: compiler đã bắt nhiều bugs, nên tests tập trung vào **business logic** thay vì null checks và type errors. `cargo test` tích hợp sẵn, không cần setup framework phức tạp.

---

Cuốn *Learn Go with Tests* dạy một bài học quan trọng: viết test TRƯỚC code không chỉ là kỷ luật — đó là **phương pháp thiết kế**. Khi bạn viết test trước, bạn đang hỏi: "API của function này trông như thế nào từ perspective của người dùng?" Bạn thiết kế interface trước, implementation sau.

Cycle TDD là 3 bước lặp lại: **Red** (viết test, test PHẢI fail vì code chưa có), **Green** (viết code đơn giản nhất để pass), **Refactor** (cải thiện code mà tests vẫn pass). Red phải đến trước vì nếu test pass ngay, nó không kiểm tra gì cả.

Trong Rust, `cargo test` tích hợp sẵn — `#[test]` attribute, `assert_eq!` macro, `#[should_panic]` cho error cases. Đơn giản, nhanh, và powerful.

## 33.1 — Anatomy of a Rust Test

```rust
// filename: src/lib.rs

// ═══ Production code ═══
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

// ═══ Tests — chỉ compile trong test mode ═══
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
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
    fn test_divide_ok() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10.0, 0.0), Err("Division by zero".into()));
    }
}
```

### Assert macros

| Macro | Ý nghĩa | Ví dụ |
|-------|---------|-------|
| `assert!(expr)` | `expr` phải true | `assert!(is_even(4))` |
| `assert_eq!(a, b)` | `a == b` | `assert_eq!(add(1,2), 3)` |
| `assert_ne!(a, b)` | `a ≠ b` | `assert_ne!(add(1,2), 0)` |
| `assert!(expr, "msg")` | Custom error message | `assert!(x > 0, "x={} not positive", x)` |

### Run tests

```bash
cargo test                    # chạy tất cả
cargo test test_add           # chạy tests có "test_add" trong tên
cargo test -- --nocapture     # show println! output
cargo test -- --test-threads=1  # chạy tuần tự
```

---

## 33.2 — Red → Green → Refactor ⭐

### TDD Cycle

```
1. 🔴 RED:     Viết test TRƯỚC → chạy → FAIL (chưa có code)
2. 🟢 GREEN:   Viết CODE tối thiểu để test PASS
3. 🔵 REFACTOR: Clean up code, giữ tests PASS
4. Repeat!
```

### Ví dụ: Build `Stack<T>` step-by-step

**Round 1: Empty stack**

```rust
// filename: src/lib.rs

// 🔴 RED — viết test trước!
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

// 🟢 GREEN — implement tối thiểu
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { Stack { items: vec![] } }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn len(&self) -> usize { self.items.len() }
}
```

**Round 2: Push & Peek**

```rust
// filename: src/lib.rs

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { Stack { items: vec![] } }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn len(&self) -> usize { self.items.len() }

    // 🟢 GREEN — thêm push và peek
    pub fn push(&mut self, item: T) { self.items.push(item); }
    pub fn peek(&self) -> Option<&T> { self.items.last() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    // 🔴 RED → 🟢 GREEN
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
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn peek_empty_returns_none() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.peek(), None);
    }
}
```

**Round 3: Pop**

```rust
// filename: src/lib.rs

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { Stack { items: vec![] } }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn push(&mut self, item: T) { self.items.push(item); }
    pub fn peek(&self) -> Option<&T> { self.items.last() }

    // Round 3
    pub fn pop(&mut self) -> Option<T> { self.items.pop() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));  // LIFO
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);     // empty
        assert!(stack.is_empty());
    }

    #[test]
    fn push_pop_maintains_order() {
        let mut stack = Stack::new();
        for i in 1..=5 { stack.push(i); }
        let mut result = vec![];
        while let Some(val) = stack.pop() { result.push(val); }
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }
}
```

---

## ✅ Checkpoint 33.2

> TDD Rhythm:
> 1. **Test first** — nghĩ behavior trước code
> 2. **Minimal implementation** — chỉ viết đủ để pass
> 3. **Refactor** — clean up, giữ tests green
> 4. **Each test = 1 behavior** — tên test mô tả behavior

---

## 33.3 — Testing Domain Logic

```rust
// filename: src/lib.rs

// ═══ Domain: Money ═══
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(i64);

impl Money {
    pub fn new(cents: i64) -> Result<Self, String> {
        if cents < 0 { Err("Money cannot be negative".into()) }
        else { Ok(Money(cents)) }
    }

    pub fn zero() -> Self { Money(0) }
    pub fn cents(&self) -> i64 { self.0 }

    pub fn add(&self, other: &Money) -> Money { Money(self.0 + other.0) }

    pub fn subtract(&self, other: &Money) -> Result<Money, String> {
        if other.0 > self.0 {
            Err(format!("Insufficient: {}¢ - {}¢", self.0, other.0))
        } else {
            Ok(Money(self.0 - other.0))
        }
    }

    pub fn multiply(&self, factor: u32) -> Money { Money(self.0 * factor as i64) }

    pub fn discount(&self, percent: u32) -> Money {
        Money(self.0 * (100 - percent as i64) / 100)
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}đ", self.0 / 100, self.0 % 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ═══ Construction ═══
    #[test]
    fn create_money() {
        assert_eq!(Money::new(500).unwrap().cents(), 500);
    }

    #[test]
    fn reject_negative() {
        assert!(Money::new(-100).is_err());
    }

    #[test]
    fn zero_is_valid() {
        assert_eq!(Money::zero().cents(), 0);
    }

    // ═══ Arithmetic ═══
    #[test]
    fn add_money() {
        let a = Money::new(300).unwrap();
        let b = Money::new(200).unwrap();
        assert_eq!(a.add(&b), Money::new(500).unwrap());
    }

    #[test]
    fn subtract_ok() {
        let a = Money::new(500).unwrap();
        let b = Money::new(200).unwrap();
        assert_eq!(a.subtract(&b), Ok(Money::new(300).unwrap()));
    }

    #[test]
    fn subtract_insufficient() {
        let a = Money::new(100).unwrap();
        let b = Money::new(500).unwrap();
        assert!(a.subtract(&b).is_err());
    }

    #[test]
    fn multiply_money() {
        let m = Money::new(100).unwrap();
        assert_eq!(m.multiply(3), Money::new(300).unwrap());
    }

    // ═══ Discount ═══
    #[test]
    fn discount_10_percent() {
        let price = Money::new(1000).unwrap();
        assert_eq!(price.discount(10), Money::new(900).unwrap());
    }

    #[test]
    fn discount_zero() {
        let price = Money::new(1000).unwrap();
        assert_eq!(price.discount(0), price);
    }

    #[test]
    fn discount_100_percent() {
        let price = Money::new(1000).unwrap();
        assert_eq!(price.discount(100), Money::zero());
    }

    // ═══ Display ═══
    #[test]
    fn display_format() {
        assert_eq!(format!("{}", Money::new(12345).unwrap()), "123.45đ");
    }
}
```

---

## 33.4 — Testing Errors & Edge Cases

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
    #[test] fn parse_age_boundary_low() { assert_eq!(parse_age("1"), Ok(1)); }
    #[test] fn parse_age_boundary_high() { assert_eq!(parse_age("150"), Ok(150)); }

    // Error paths
    #[test] fn parse_age_empty() { assert!(parse_age("").is_err()); }
    #[test] fn parse_age_not_number() { assert!(parse_age("abc").is_err()); }
    #[test] fn parse_age_negative() { assert!(parse_age("-5").is_err()); }
    #[test] fn parse_age_zero() { assert!(parse_age("0").is_err()); }
    #[test] fn parse_age_too_high() { assert!(parse_age("151").is_err()); }
    #[test] fn parse_age_float() { assert!(parse_age("25.5").is_err()); }

    // Error message content
    #[test]
    fn parse_age_error_message() {
        let err = parse_age("abc").unwrap_err();
        assert!(err.contains("not a number"), "Got: {}", err);
    }
}
```

### `#[should_panic]`

```rust
// filename: src/lib.rs

pub fn first_element(list: &[i32]) -> i32 {
    list[0]  // panics on empty!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn first_element_panics_on_empty() {
        first_element(&[]);
    }
}
```

---

## 33.5 — Test Organization

### Unit tests (in-file)

```rust
// src/domain/order.rs

pub struct Order { /* ... */ }
impl Order { /* ... */ }

// Unit tests: cùng file, cùng module
#[cfg(test)]
mod tests {
    use super::*;
    // test Order internals
}
```

### Integration tests (separate directory)

```
my_project/
├── src/
│   ├── lib.rs
│   └── domain/
│       └── order.rs
└── tests/                    ← integration tests
    ├── order_workflow_test.rs
    └── payment_test.rs
```

```rust
// tests/order_workflow_test.rs
use my_project::domain::Order;

#[test]
fn full_order_workflow() {
    // Test public API only — black-box testing
}
```

### Test helpers

```rust
// filename: src/lib.rs

pub struct User { pub name: String, pub email: String, pub age: u32 }

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: create test fixtures
    fn sample_user() -> User {
        User { name: "Test User".into(), email: "test@co.com".into(), age: 25 }
    }

    fn sample_user_with_age(age: u32) -> User {
        User { age, ..sample_user() }
    }

    #[test]
    fn user_default() {
        let user = sample_user();
        assert_eq!(user.name, "Test User");
    }

    #[test]
    fn user_custom_age() {
        let user = sample_user_with_age(30);
        assert_eq!(user.age, 30);
        assert_eq!(user.name, "Test User"); // other fields preserved
    }
}
```

---

## 33.6 — Table-Driven Tests

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
        let cases = vec![
            (1, "1"),
            (2, "2"),
            (3, "Fizz"),
            (5, "Buzz"),
            (15, "FizzBuzz"),
            (30, "FizzBuzz"),
            (7, "7"),
            (9, "Fizz"),
            (10, "Buzz"),
        ];

        for (input, expected) in cases {
            assert_eq!(fizzbuzz(input), expected, "fizzbuzz({}) failed", input);
        }
    }
}
```

### Validation table-driven tests

```rust
// filename: src/lib.rs

pub fn validate_email(email: &str) -> bool {
    let trimmed = email.trim();
    trimmed.contains('@')
        && trimmed.len() >= 5
        && !trimmed.starts_with('@')
        && !trimmed.ends_with('@')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_validation_table() {
        let valid = vec!["a@b.com", "user@domain.org", "test@co.vn"];
        let invalid = vec!["", "noat", "@start.com", "end@", "a@b", "  "];

        for email in valid {
            assert!(validate_email(email), "'{}' should be valid", email);
        }
        for email in invalid {
            assert!(!validate_email(email), "'{}' should be invalid", email);
        }
    }
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Write tests first

Viết tests (TRƯỚC code) cho `fn reverse_string(s: &str) -> String`:
- `"hello"` → `"olleh"`
- `""` → `""`
- `"a"` → `"a"`
- Unicode: `"xin chào"` → `"oàhc nix"`

<details><summary>✅ Lời giải</summary>

```rust
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn reverse_hello() { assert_eq!(reverse_string("hello"), "olleh"); }
    #[test] fn reverse_empty() { assert_eq!(reverse_string(""), ""); }
    #[test] fn reverse_single() { assert_eq!(reverse_string("a"), "a"); }
    #[test] fn reverse_unicode() { assert_eq!(reverse_string("xin chào"), "oàhc nix"); }
}
```

</details>

---

**Bài 2** (10 phút): TDD Calculator

Dùng Red→Green→Refactor, build `Calculator` struct:
1. `new()` → value = 0
2. `add(n)` → cộng n
3. `subtract(n)` → trừ n
4. `multiply(n)` → nhân n
5. `result()` → trả giá trị hiện tại
6. `reset()` → về 0

Viết test TRƯỚC mỗi method.

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
    #[test] fn subtract_numbers() {
        let mut c = Calculator::new();
        c.add(10.0); c.subtract(3.0);
        assert_eq!(c.result(), 7.0);
    }
    #[test] fn multiply_numbers() {
        let mut c = Calculator::new();
        c.add(5.0); c.multiply(3.0);
        assert_eq!(c.result(), 15.0);
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

**Bài 3** (15 phút): TDD Password Validator

Build `validate_password(pw: &str) -> Result<(), Vec<String>>` bằng TDD:
- Min 8 chars
- At least 1 uppercase
- At least 1 lowercase
- At least 1 digit
- At least 1 special char (`!@#$%^&*`)
- Return ALL errors (not just first)

<details><summary>✅ Lời giải Bài 3</summary>

```rust
fn validate_password(pw: &str) -> Result<(), Vec<String>> {
    let mut errors = vec![];
    if pw.len() < 8 { errors.push("Min 8 characters".into()); }
    if !pw.chars().any(|c| c.is_uppercase()) { errors.push("Need uppercase".into()); }
    if !pw.chars().any(|c| c.is_lowercase()) { errors.push("Need lowercase".into()); }
    if !pw.chars().any(|c| c.is_ascii_digit()) { errors.push("Need digit".into()); }
    if !pw.chars().any(|c| "!@#$%^&*".contains(c)) { errors.push("Need special char".into()); }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn valid_password() { assert!(validate_password("Str0ng!Pw").is_ok()); }
    #[test] fn too_short() {
        let errs = validate_password("Ab1!").unwrap_err();
        assert!(errs.iter().any(|e| e.contains("8")));
    }
    #[test] fn all_errors() {
        let errs = validate_password("").unwrap_err();
        assert_eq!(errs.len(), 5);
    }
    #[test] fn no_special() {
        let errs = validate_password("Abcdefg1").unwrap_err();
        assert!(errs.iter().any(|e| e.contains("special")));
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Test pass locally, fail CI" | Environment-dependent | Avoid file system, time, random in tests |
| "Too many tests, slow" | Integration tests nặng | Tách unit (fast) vs integration (slow) |
| "Tests brittle" | Testing implementation, not behavior | Test **what** not **how** |
| "println! không hiển thị" | cargo test captures output | `cargo test -- --nocapture` |

---

## Tóm tắt

- ✅ **`#[test]` + `assert_eq!`**: Cơ bản nhất — mỗi test = 1 function, 1 behavior.
- ✅ **Red → Green → Refactor**: Test first → minimal code → clean up. Rhythm!
- ✅ **Test domain logic**: `Money`, `Stack`, validators — pure functions = easy to test.
- ✅ **Edge cases**: empty, boundary, error messages, `#[should_panic]`.
- ✅ **Organization**: `#[cfg(test)] mod tests` (unit), `tests/` dir (integration).
- ✅ **Table-driven**: `vec![(input, expected)]` — DRY, comprehensive.

## Tiếp theo

→ Chapter 34: **Property-Based Testing** — thay vì viết từng example, bạn mô tả **properties** ("reverse twice = original") → framework tự generate 1000s test cases!
