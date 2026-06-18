# Chapter 34 — Property-Based Testing

> **Bạn sẽ học được**:
> - **Property-Based Testing** (PBT) — mô tả **quy luật**, framework tự generate data
> - **`proptest`** crate — strategies, runners, shrinking
> - **Categories of properties**: round-trip, idempotent, invariant, oracle, metamorphic
> - Khi nào dùng PBT vs example-based tests
> - **Shrinking** — tự động tìm minimal failing case
>
> **Yêu cầu trước**: Chapter 33 (TDD).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Framework kiểm tra **hàng nghìn cases** thay vì bạn viết từng cái.

---

## Property-Based Testing — Để máy nghĩ test cases

Unit tests kiểm tra specific examples: "input A → expect output B". Property-based tests kiểm tra **quy luật tổng quát**: "với BẤT KỲ input hợp lệ nào, output phải thỏa tính chất X". Framework tự sinh hàng ngàn random inputs và tìm counterexample.

Tại sao mạnh? Vì bugs thường ẩn ở edge cases mà developer không nghĩ tới. Property-based testing tìm chúng tự động. Kết hợp với Rust's type system, bạn có thể test properties như: "serialize rồi deserialize phải trả về giá trị gốc" (round-trip), "sort hai lần phải giống sort một lần" (idempotency).

---

Unit tests kiểm tra examples: "input A → output B". Vấn đề: bạn chỉ kiểm tra cases bạn **nghĩ tới**. Bugs ẩn ở cases bạn **không nghĩ tới**.

Property-based testing đảo ngược: framework **sinh random inputs** và kiểm tra output thỏa **tính chất tổng quát**. Tìm counterexample → **shrink** về input nhỏ nhất vẫn fail.

Properties phổ biến: round-trip (`deserialize(serialize(x)) == x`), idempotency (`sort(sort(xs)) == sort(xs)`), invariant (`reverse(xs).len() == xs.len()`). `proptest` crate sinh hàng ngàn test cases tự động — hiệu quả gấp 100x unit tests.

## 34.1 — Example Tests vs Property Tests

### Vấn đề: Example tests bỏ sót edge cases

```rust
// Example-based: bạn nghĩ ra 3-4 cases
#[test] fn reverse_hello() { assert_eq!(reverse("hello"), "olleh"); }
#[test] fn reverse_empty() { assert_eq!(reverse(""), ""); }
// Xong? Bạn có TỰ TIN 100% không?
```

### Property-based: mô tả QUY LUẬT

```rust
// property: "reverse 2 lần = giữ nguyên" — ĐÚNG cho MỌI string
// framework generate 1000s random strings, kiểm tra tất cả!
```

### Setup

```toml
# Cargo.toml
[dev-dependencies]
proptest = "1"
```

### First property test

```rust
// filename: src/lib.rs

fn reverse(s: &str) -> String { s.chars().rev().collect() }

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Example test: 1 case
    #[test]
    fn reverse_hello() {
        assert_eq!(reverse("hello"), "olleh");
    }

    // Property test: 1000s cases!
    proptest! {
        #[test]
        fn reverse_twice_is_identity(s in ".*") {
            assert_eq!(reverse(&reverse(&s)), s);
        }

        #[test]
        fn reverse_preserves_length(s in ".*") {
            assert_eq!(reverse(&s).len(), s.len());
        }

        #[test]
        fn reverse_of_single_char(c in any::<char>()) {
            let s = c.to_string();
            assert_eq!(reverse(&s), s);
        }
    }
}
```

> **💡 Key insight**: Bạn không nói "input X → output Y". Bạn nói "cho MỌI input, property này ĐÚNG." Framework tự tạo test data!

---

## 34.2 — Categories of Properties

### 1. Round-trip (encode → decode = identity)

```rust
// filename: src/lib.rs
use proptest::prelude::*;

fn encode_base64(bytes: &[u8]) -> String {
    // Simplified: just uppercase hex for demo
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

fn decode_base64(s: &str) -> Result<Vec<u8>, String> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16).map_err(|e| e.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn roundtrip_encode_decode(bytes in proptest::collection::vec(any::<u8>(), 0..100)) {
            let encoded = encode_base64(&bytes);
            let decoded = decode_base64(&encoded).unwrap();
            assert_eq!(decoded, bytes);
        }
    }
}
```

### 2. Idempotent (apply twice = apply once)

```rust
// filename: src/lib.rs
use proptest::prelude::*;

fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn to_lowercase_trimmed(s: &str) -> String {
    s.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn normalize_is_idempotent(s in ".*") {
            let once = normalize_whitespace(&s);
            let twice = normalize_whitespace(&once);
            assert_eq!(once, twice);  // applying again doesn't change
        }

        #[test]
        fn lowercase_trimmed_idempotent(s in ".*") {
            let once = to_lowercase_trimmed(&s);
            let twice = to_lowercase_trimmed(&once);
            assert_eq!(once, twice);
        }
    }
}
```

### 3. Invariant (property always holds)

```rust
// filename: src/lib.rs
use proptest::prelude::*;

fn sort(mut v: Vec<i32>) -> Vec<i32> { v.sort(); v }

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn sort_preserves_length(v in proptest::collection::vec(any::<i32>(), 0..50)) {
            assert_eq!(sort(v.clone()).len(), v.len());
        }

        #[test]
        fn sort_output_is_ordered(v in proptest::collection::vec(any::<i32>(), 0..50)) {
            let sorted = sort(v);
            for pair in sorted.windows(2) {
                assert!(pair[0] <= pair[1]);
            }
        }

        #[test]
        fn sort_contains_same_elements(v in proptest::collection::vec(any::<i32>(), 0..50)) {
            let mut original = v.clone();
            let mut sorted = sort(v);
            original.sort();
            assert_eq!(original, sorted);
        }
    }
}
```

### 4. Oracle (compare with known-good implementation)

```rust
// filename: src/lib.rs
use proptest::prelude::*;

// Your fast implementation
fn my_max(list: &[i32]) -> Option<i32> {
    list.iter().copied().reduce(|a, b| if a > b { a } else { b })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn my_max_matches_std(v in proptest::collection::vec(any::<i32>(), 1..100)) {
            // Oracle: std library
            let expected = v.iter().max().copied();
            assert_eq!(my_max(&v), expected);
        }
    }
}
```

### 5. Metamorphic (transform input → predictable output change)

```rust
// filename: src/lib.rs
use proptest::prelude::*;

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn adding_word_increases_count(s in "[a-z ]{0,50}", word in "[a-z]{1,10}") {
            let original = count_words(&s);
            let with_extra = format!("{} {}", s.trim(), word);
            let new_count = count_words(&with_extra);
            // Adding a word: count increases by exactly 1 (if original non-empty)
            if !s.trim().is_empty() {
                assert_eq!(new_count, original + 1, "s='{}', word='{}'", s, word);
            }
        }
    }
}
```

---

## ✅ Checkpoint 34.2

> **5 property categories**:
>
> | Category | Property | Ví dụ |
> |----------|----------|-------|
> | **Round-trip** | `decode(encode(x)) == x` | serialize/deserialize |
> | **Idempotent** | `f(f(x)) == f(x)` | normalize, sort, trim |
> | **Invariant** | property luôn true | sort giữ length, elements |
> | **Oracle** | my_f(x) == known_f(x) | so sánh với std lib |
> | **Metamorphic** | transform input → predictable change | add element → count+1 |

---

## 34.3 — Strategies: Generate Test Data

```rust
// filename: src/lib.rs

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
        // ═══ Basic strategies ═══
        #[test]
        fn any_i32(n in any::<i32>()) {
            // n là bất kỳ i32 nào
            let _ = n;
        }

        #[test]
        fn range_u32(n in 1u32..100) {
            assert!(n >= 1 && n < 100);
        }

        #[test]
        fn string_pattern(s in "[a-zA-Z]{1,20}") {
            assert!(!s.is_empty());
            assert!(s.len() <= 20);
            assert!(s.chars().all(|c| c.is_alphabetic()));
        }

        // ═══ Collection strategies ═══
        #[test]
        fn vec_of_ints(v in proptest::collection::vec(1i32..100, 0..20)) {
            assert!(v.len() <= 20);
            assert!(v.iter().all(|&n| n >= 1 && n < 100));
        }

        // ═══ Composed strategies ═══
        #[test]
        fn email_like(
            user in "[a-z]{3,10}",
            domain in "[a-z]{3,8}",
            tld in prop_oneof!["com", "org", "net", "vn"],
        ) {
            let email = format!("{}@{}.{}", user, domain, tld);
            assert!(email.contains('@'));
        }
    }

    // ═══ Custom strategy ═══
    #[derive(Debug, Clone)]
    struct Money { cents: u64 }

    fn money_strategy() -> impl Strategy<Value = Money> {
        (1u64..10_000_000).prop_map(|cents| Money { cents })
    }

    proptest! {
        #[test]
        fn money_always_positive(m in money_strategy()) {
            assert!(m.cents > 0);
        }
    }
}
```

### Bảng strategies

| Strategy | Generates | Ví dụ |
|----------|-----------|-------|
| `any::<T>()` | Random T | `any::<i32>()` |
| `min..max` | Range | `1i32..100` |
| `"regex"` | String matching regex | `"[a-z]{1,10}"` |
| `vec(strategy, len)` | Vec of elements | `vec(any::<u8>(), 0..50)` |
| `prop_oneof![a, b]` | One of alternatives | `prop_oneof!["VND", "USD"]` |
| `(s1, s2)` | Tuple | `(any::<i32>(), ".*")` |
| `.prop_map(f)` | Transform generated value | `(1..100).prop_map(\|n\| Money(n))` |
| `.prop_filter(f)` | Filter out values | `.prop_filter("positive", \|n\| *n > 0)` |

---

## 34.4 — Shrinking: Tìm Minimal Failing Case

```rust
// filename: src/lib.rs

// Buggy function: fails khi input > 100
fn process(n: i32) -> i32 {
    if n > 100 { panic!("overflow!"); }
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn process_handles_all_inputs(n in any::<i32>()) {
            let _ = process(n);
            // proptest sẽ:
            // 1. Tìm thấy n=742389 fails
            // 2. SHRINK: thử n nhỏ hơn → 101 vẫn fails
            // 3. Report: "minimal failing input: 101"
        }
    }
}
```

> **Shrinking** = khi test fail, framework tự **tìm input nhỏ nhất** vẫn fail. `n=742389` → shrink → `n=101`. Dễ debug hơn!

---

## 34.5 — Domain Property Testing

```rust
// filename: src/lib.rs

#[derive(Debug, Clone, PartialEq)]
struct ShoppingCart {
    items: Vec<(String, u32, u32)>, // name, price, qty
}

impl ShoppingCart {
    fn new() -> Self { ShoppingCart { items: vec![] } }

    fn add_item(&mut self, name: &str, price: u32, qty: u32) {
        self.items.push((name.into(), price, qty));
    }

    fn total(&self) -> u64 {
        self.items.iter().map(|(_, p, q)| *p as u64 * *q as u64).sum()
    }

    fn item_count(&self) -> usize { self.items.len() }

    fn apply_discount(&self, percent: u32) -> u64 {
        let t = self.total();
        t - t * percent as u64 / 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn cart_strategy() -> impl Strategy<Value = ShoppingCart> {
        proptest::collection::vec(
            ("[a-z]{3,10}", 1u32..100_000, 1u32..20),
            1..10
        ).prop_map(|items| {
            let mut cart = ShoppingCart::new();
            for (name, price, qty) in items {
                cart.add_item(&name, price, qty);
            }
            cart
        })
    }

    proptest! {
        // Invariant: total >= 0
        #[test]
        fn total_is_non_negative(cart in cart_strategy()) {
            assert!(cart.total() > 0);
        }

        // Invariant: discount reduces total
        #[test]
        fn discount_reduces_total(
            cart in cart_strategy(),
            percent in 1u32..100,
        ) {
            let original = cart.total();
            let discounted = cart.apply_discount(percent);
            assert!(discounted <= original, "{}% of {} > {}", percent, original, discounted);
        }

        // Invariant: 0% discount = no change
        #[test]
        fn zero_discount_unchanged(cart in cart_strategy()) {
            assert_eq!(cart.apply_discount(0), cart.total());
        }

        // Invariant: 100% discount = free
        #[test]
        fn full_discount_is_zero(cart in cart_strategy()) {
            assert_eq!(cart.apply_discount(100), 0);
        }

        // Metamorphic: add item → total increases
        #[test]
        fn adding_item_increases_total(
            cart in cart_strategy(),
            price in 1u32..100_000,
            qty in 1u32..10,
        ) {
            let before = cart.total();
            let mut cart = cart;
            cart.add_item("extra", price, qty);
            assert!(cart.total() > before);
        }
    }
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Identify properties

Cho `fn sort(v: Vec<i32>) -> Vec<i32>`, liệt kê 3 properties bạn sẽ test.

<details><summary>✅ Lời giải</summary>

1. **Invariant**: `sort(v).len() == v.len()` (giữ nguyên length)
2. **Invariant**: Output ordered: `sorted[i] <= sorted[i+1]`
3. **Idempotent**: `sort(sort(v)) == sort(v)`
4. (Bonus) **Same elements**: sorted chứa cùng elements

</details>

---

**Bài 2** (10 phút): Round-trip property

Viết property test cho JSON serialization:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct User { name: String, age: u32, active: bool }
```
Property: `deserialize(serialize(user)) == user` cho mọi `User`.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
use proptest::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct User { name: String, age: u32, active: bool }

fn user_strategy() -> impl Strategy<Value = User> {
    ("[a-zA-Z]{2,20}", 1u32..150, any::<bool>())
        .prop_map(|(name, age, active)| User { name, age, active })
}

proptest! {
    #[test]
    fn json_roundtrip(user in user_strategy()) {
        let json = serde_json::to_string(&user).unwrap();
        let parsed: User = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, user);
    }
}
```

</details>

---

**Bài 3** (15 phút): Full PBT suite

Viết property tests cho `BankAccount`:
```rust
struct BankAccount { balance: i64 }
fn deposit(acc: &mut BankAccount, amount: u64)
fn withdraw(acc: &mut BankAccount, amount: u64) -> Result<(), String>
```
Properties:
1. Deposit increases balance by exact amount
2. Withdraw decreases balance by exact amount (if sufficient)
3. Withdraw fails if insufficient (balance unchanged)
4. Deposit then withdraw same amount = original balance

<details><summary>✅ Lời giải Bài 3</summary>

```rust
struct BankAccount { balance: i64 }

impl BankAccount {
    fn new(balance: i64) -> Self { BankAccount { balance } }
    fn deposit(&mut self, amount: u64) { self.balance += amount as i64; }
    fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        if amount as i64 > self.balance { Err("Insufficient".into()) }
        else { self.balance -= amount as i64; Ok(()) }
    }
}

proptest! {
    #[test]
    fn deposit_increases_exactly(
        initial in 0i64..1_000_000,
        amount in 1u64..100_000,
    ) {
        let mut acc = BankAccount::new(initial);
        acc.deposit(amount);
        assert_eq!(acc.balance, initial + amount as i64);
    }

    #[test]
    fn withdraw_decreases_exactly(
        initial in 1000i64..1_000_000,
        amount in 1u64..1000,
    ) {
        let mut acc = BankAccount::new(initial);
        acc.withdraw(amount).unwrap();
        assert_eq!(acc.balance, initial - amount as i64);
    }

    #[test]
    fn withdraw_insufficient_unchanged(
        initial in 0i64..100,
        extra in 1u64..1000,
    ) {
        let amount = initial as u64 + extra;
        let mut acc = BankAccount::new(initial);
        assert!(acc.withdraw(amount).is_err());
        assert_eq!(acc.balance, initial);
    }

    #[test]
    fn deposit_withdraw_roundtrip(
        initial in 0i64..1_000_000,
        amount in 1u64..100_000,
    ) {
        let mut acc = BankAccount::new(initial);
        acc.deposit(amount);
        acc.withdraw(amount).unwrap();
        assert_eq!(acc.balance, initial);
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Test chạy chậm" | proptest default 256 cases | `ProptestConfig { cases: 50, .. }` |
| "Regex strategy quá rộng" | `".*"` generate mọi thứ | Thu hẹp: `"[a-z]{1,20}"` |
| "Shrinking không tìm ra min case" | Quá nhiều dimensions | Reduce strategy complexity |
| "Flaky test" | Randomized data + edge case | Set seed: `PROPTEST_SEED=12345` |

---

## Tóm tắt

- ✅ **Property tests**: Mô tả **quy luật**, framework generate **1000s test cases** tự động.
- ✅ **5 categories**: Round-trip, Idempotent, Invariant, Oracle, Metamorphic.
- ✅ **Strategies**: `any::<T>()`, ranges, regex, `vec()`, `prop_map`, `prop_oneof!`.
- ✅ **Shrinking**: Tự động tìm **minimal failing input** — dễ debug.
- ✅ **Domain PBT**: Test business rules (discount, add item, balance) — properties hold cho MỌI valid inputs.
- ✅ **Khi nào dùng**: Round-trip (serialization), math (commutative/associative), domain invariants.

## Tiếp theo

→ Chapter 35: **Mocking, DI & Hexagonal Architecture** — bạn sẽ test code có database, API calls bằng trait-based dependency injection + mock implementations.
