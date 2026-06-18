# Chapter 29 — Functors & Map in Rust

> **Bạn sẽ học được**:
> - **Functor** = "container có `.map()`" — bạn đã dùng hàng ngày!
> - `Option::map`, `Result::map`, `Iterator::map` — cùng 1 pattern
> - Tại sao Rust **không thể** abstract Functor thành trait (HKT)
> - **GATs** (Generic Associated Types) — Rust approach
> - Custom containers với `.map()`
> - Functor laws — quy tắc mà mọi `.map()` phải tuân theo
>
> **Yêu cầu trước**: Chapter 13 (HOF), Chapter 17 (Generics), Chapter 28 (Algebra).
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn nhìn `.map()` qua lăng kính toán học — và hiểu tại sao nó xuất hiện khắp nơi.

---

## 29.1 — Bạn đã dùng Functors rồi!

Trước khi vào lý thuyết, hãy nhìn vào thứ bạn đã viết hàng ngày — có thể ngày nào cũng gọi mà không biết nó có tên.

Nghĩ về một **hộp quà**. Bạn có hộp chứa một chiếc áo. Bạn muốn thay chiếc áo bằng chiếc khác — nhưng vẫn giữ nguyên cái hộp. Bạn không vứt hộp đi làm hộp mới — bạn chỉ **thay đổi thứ bên trong**, giữ nguyên structure hộp.

Đó chính là `.map()` — và cũng là Functor. Option là hộp có thể rỗng hoặc chứa 1 giá trị. Result là hộp có thể chứa Ok hoặc Err. Vec là hộp chứa nhiều giá trị. `.map(f)` biến đổi thứ bên trong hộp, giữ nguyên "kiểu hộp".

### `.map()` ở mọi nơi

```rust
// filename: src/main.rs

fn main() {
    // Option::map — transform giá trị BÊN TRONG Option
    let name: Option<&str> = Some("minh");
    let upper: Option<String> = name.map(|n| n.to_uppercase());
    println!("Option: {:?}", upper);  // Some("MINH")

    let none: Option<&str> = None;
    let still_none: Option<String> = none.map(|n| n.to_uppercase());
    println!("None: {:?}", still_none);  // None (function KHÔNG chạy!)

    // Result::map — transform Ok value
    let parsed: Result<i32, _> = "42".parse::<i32>();
    let doubled: Result<i32, _> = parsed.map(|n| n * 2);
    println!("Result: {:?}", doubled);  // Ok(84)

    let err: Result<i32, _> = "abc".parse::<i32>();
    let still_err = err.map(|n| n * 2);
    println!("Err: {:?}", still_err);  // Err(...) (function KHÔNG chạy!)

    // Iterator::map — transform MỌI phần tử
    let nums = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = nums.iter().map(|n| n * n).collect();
    println!("Iterator: {:?}", squared);  // [1, 4, 9, 16, 25]

    // Vec::iter().map() ≈ "Vec functor"
    let names = vec!["minh", "lan", "hải"];
    let uppers: Vec<String> = names.iter().map(|n| n.to_uppercase()).collect();
    println!("Names: {:?}", uppers);
}
```

Và một điều tinh tế mà bạn có thể đã nhận ra: khi hộp rỗng (`None`, `Err`), `.map()` không chạy function — nó chỉ trả về hộp rỗng. Giống như bạn gửi một cái hộp rỗng vào máy xử lý — máy không làm gì, trả ra hộp rỗng.

> **💡 Pattern**: `.map(f)` = "apply function `f` vào giá trị **bên trong** container, giữ nguyên container structure."

---

## 29.2 — Functor = Container + map

### Định nghĩa (tường thuật)

Bây giờ bạn đã thấy pattern — hãy đặt tên cho nó. Trong toán học và FP, bất kỳ "container" nào có `.map()` hoạt động đúng cách đều được gọi là **Functor**. Tên nghe oai, nhưng ý tưởng thì bạn vừa thấy rồi:

**Functor** = bất kỳ type `F<A>` nào có operation:
```
map: F<A> → (A → B) → F<B>
```

Nói đơn giản: "cho tôi container chứa `A` và function từ `A → B`, tôi trả container chứa `B`. **Structure không đổi**."

### Hình dung

```
Option<i32>          .map(|x| x.to_string())      Option<String>
┌─────────┐                                        ┌───────────┐
│ Some(42) │  ──── f(42) = "42" ─────────────────→  │ Some("42")│
└─────────┘                                        └───────────┘

┌──────┐                                           ┌──────┐
│ None │  ──── (skip) ──────────────────────────→   │ None │
└──────┘                                           └──────┘

Vec<i32>             .map(|x| x * 2)               Vec<i32>
┌──────────┐                                       ┌──────────┐
│ [1, 2, 3]│  ──── f(1), f(2), f(3) ────────────→  │ [2, 4, 6]│
└──────────┘                                       └──────────┘
  3 phần tử                                          3 phần tử (structure giữ nguyên!)
```

---

## 29.3 — Functor Laws: 2 Quy Tắc

Để `.map()` thực sự tin cậy, nó phải tuân theo 2 quy tắc. Nghe "laws" thì nặng nề, nhưng thực ra chúng chỉ nói: `.map()` không được thêm sêde effects, không được thay đổi structure, không được làm gì ngoài việc transform giá trị bên trong. Nếu bạn viết custom `.map()` và nó thỏa 2 laws này, bạn có thể yên tâm dùng nó mọi nơi mà không sợ surprise.


### Law 1: Identity

```rust
// map(x, |a| a) == x
// "Map với identity function → không đổi"

fn main() {
    let x = Some(42);
    assert_eq!(x.map(|a| a), x);  // ✅

    let v = vec![1, 2, 3];
    let same: Vec<_> = v.iter().map(|&a| a).collect();
    assert_eq!(same, vec![1, 2, 3]);  // ✅
}
```

### Law 2: Composition

```rust
// map(x, |a| g(f(a))) == map(map(x, f), g)
// "Map f rồi map g == Map f∘g một lần"

fn main() {
    let x = Some(5);
    let f = |n: i32| n * 2;
    let g = |n: i32| n + 10;

    // Hai lần map
    let two_maps = x.map(f).map(g);

    // Một lần map (composed)
    let one_map = x.map(|n| g(f(n)));

    assert_eq!(two_maps, one_map);  // ✅ Some(20)
    println!("Law 2: {:?} == {:?}", two_maps, one_map);
}
```

> **💡 Tại sao laws quan trọng?** Vì chúng đảm bảo `.map()` **predictable** — không Side effects, không thay đổi structure, không surprise.

---

## ✅ Checkpoint 29.3

> Ghi nhớ:
> 1. Functor = container + `.map()` — transform inner value, keep structure
> 2. **Law 1**: `map(id) == id` (map identity = no change)
> 3. **Law 2**: `map(f).map(g) == map(f∘g)` (chaining = composition)
> 4. Option, Result, Iterator, Vec — tất cả là Functors!

---

## 29.4 — Custom Functors

### Box-like container

```rust
// filename: src/main.rs

// Custom container: "Labeled value"
#[derive(Debug, Clone)]
struct Labeled<T> {
    label: String,
    value: T,
}

impl<T> Labeled<T> {
    fn new(label: &str, value: T) -> Self {
        Labeled { label: label.into(), value }
    }

    // map: transform value, keep label
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Labeled<U> {
        Labeled {
            label: self.label,
            value: f(self.value),
        }
    }
}

fn main() {
    let temp = Labeled::new("Temperature", 36.5_f64);
    println!("{:?}", temp);

    let formatted = temp.map(|t| format!("{:.1}°C", t));
    println!("{:?}", formatted);
    // Labeled { label: "Temperature", value: "36.5°C" }

    // Chain maps
    let score = Labeled::new("Math", 85_u32)
        .map(|s| s as f64 / 100.0)
        .map(|pct| format!("{:.0}%", pct * 100.0));
    println!("{:?}", score);
    // Labeled { label: "Math", value: "85%" }
}
```

### Tree Functor

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
enum Tree<T> {
    Leaf(T),
    Branch { left: Box<Tree<T>>, right: Box<Tree<T>> },
}

impl<T> Tree<T> {
    fn leaf(value: T) -> Self { Tree::Leaf(value) }

    fn branch(left: Tree<T>, right: Tree<T>) -> Self {
        Tree::Branch { left: Box::new(left), right: Box::new(right) }
    }

    // Functor map: transform ALL values, keep tree structure
    fn map<U, F: Fn(&T) -> U>(&self, f: &F) -> Tree<U> {
        match self {
            Tree::Leaf(val) => Tree::Leaf(f(val)),
            Tree::Branch { left, right } => Tree::Branch {
                left: Box::new(left.map(f)),
                right: Box::new(right.map(f)),
            },
        }
    }

    fn to_vec(&self) -> Vec<&T> {
        match self {
            Tree::Leaf(val) => vec![val],
            Tree::Branch { left, right } => {
                let mut v = left.to_vec();
                v.extend(right.to_vec());
                v
            }
        }
    }
}

fn main() {
    //       Branch
    //      /      \
    //   Branch    Leaf(4)
    //   /    \
    // Leaf(1) Leaf(2)
    let tree = Tree::branch(
        Tree::branch(Tree::leaf(1), Tree::leaf(2)),
        Tree::leaf(4),
    );

    println!("Original: {:?}", tree.to_vec());

    let doubled = tree.map(&|x| x * 2);
    println!("Doubled: {:?}", doubled.to_vec());

    let strings = tree.map(&|x| format!("#{}", x));
    println!("Strings: {:?}", strings.to_vec());
}
```

---

## 29.5 — Tại sao Rust không có Functor trait?

### Vấn đề: Higher-Kinded Types (HKT)

Haskell có thể viết:
```haskell
class Functor f where
  fmap :: (a -> b) -> f a -> f b

-- f là "type constructor": Option, Vec, Result...
-- Haskell nói: "f là TYPE CHƯA HOÀN CHỈNH cần 1 param"
```

Rust **KHÔNG** làm được vì generics yêu cầu **concrete types**:

```rust
// ❌ KHÔNG COMPILE! Rust không có Higher-Kinded Types
// trait Functor<F> {             // F = Option? Vec? Result?
//     fn map<A, B>(fa: F<A>, f: fn(A) -> B) -> F<B>;
// }
// Error: F<A> — F không phải type constructor
```

### Hệ quả thực tế

Bạn **không thể** viết generic function hoạt động cho "bất kỳ Functor nào":

```rust
// ❌ Không thể viết:
// fn double_inside<F: Functor>(container: F<i32>) -> F<i32> {
//     container.map(|x| x * 2)
// }
// double_inside(Some(5))    → Some(10)
// double_inside(vec![1,2])  → vec![2,4]
// double_inside(Ok(3))      → Ok(6)

// ✅ Phải viết riêng cho mỗi type:
fn double_option(x: Option<i32>) -> Option<i32> { x.map(|n| n * 2) }
fn double_vec(x: Vec<i32>) -> Vec<i32> { x.iter().map(|n| n * 2).collect() }
fn double_result(x: Result<i32, String>) -> Result<i32, String> { x.map(|n| n * 2) }
```

### GATs: Rust approach (partial solution)

```rust
// filename: src/main.rs

// GATs (Generic Associated Types) — since Rust 1.65
trait Mappable {
    type Item;
    type Output<U>;

    fn map_items<U, F: Fn(Self::Item) -> U>(self, f: F) -> Self::Output<U>;
}

impl<T> Mappable for Option<T> {
    type Item = T;
    type Output<U> = Option<U>;

    fn map_items<U, F: Fn(T) -> U>(self, f: F) -> Option<U> {
        self.map(f)
    }
}

impl<T> Mappable for Vec<T> {
    type Item = T;
    type Output<U> = Vec<U>;

    fn map_items<U, F: Fn(T) -> U>(self, f: F) -> Vec<U> {
        self.into_iter().map(f).collect()
    }
}

impl<T, E> Mappable for Result<T, E> {
    type Item = T;
    type Output<U> = Result<U, E>;

    fn map_items<U, F: Fn(T) -> U>(self, f: F) -> Result<U, E> {
        self.map(f)
    }
}

// Bây giờ có thể viết generic function!
fn stringify<C: Mappable<Item = i32>>(container: C) -> C::Output<String> {
    container.map_items(|x| format!("#{}", x))
}

fn main() {
    println!("{:?}", stringify(Some(42)));       // Some("#42")
    println!("{:?}", stringify(vec![1, 2, 3]));  // ["#1", "#2", "#3"]
    println!("{:?}", stringify(Ok::<i32, String>(7)));  // Ok("#7")
}
```

> **💡 GATs** = Rust workaround cho thiếu HKT. Không đẹp bằng Haskell, nhưng **đủ dùng** cho most practical cases.

---

## 29.6 — Functor Family: map, map_err, and_then

### Functor gia đình

| Method | Áp dụng lên | Container |
|--------|-------------|-----------|
| `.map(f)` | Ok value / Some value | Option, Result |
| `.map_err(f)` | Err value | Result |
| `.and_then(f)` | Ok/Some → Result/Option | Option, Result (= Monad!) |
| `.map(f)` | Mỗi phần tử | Iterator |
| `.flat_map(f)` | Mỗi phần tử → Iterator | Iterator (= Monad!) |

```rust
// filename: src/main.rs

fn main() {
    let result: Result<i32, String> = Ok(42);

    // map: transform Ok
    let a = result.map(|n| n * 2);             // Ok(84)

    // map_err: transform Err  (Bifunctor!)
    let b: Result<i32, String> = Err("oops".into());
    let c = b.map_err(|e| format!("Error: {}", e));   // Err("Error: oops")

    // and_then: Ok → maybe another Result (Monad!)
    let d = Ok(42).and_then(|n: i32| {
        if n > 0 { Ok(n.to_string()) } else { Err("negative".into()) }
    });

    println!("map: {:?}", a);
    println!("map_err: {:?}", c);
    println!("and_then: {:?}", d);

    // Iterator: map vs flat_map
    let nested = vec![vec![1, 2], vec![3], vec![4, 5, 6]];

    // map: Vec<Vec<i32>> → Vec<Vec<i32>>
    let mapped: Vec<Vec<i32>> = nested.iter()
        .map(|inner| inner.iter().map(|x| x * 10).collect())
        .collect();
    println!("\nmap: {:?}", mapped);

    // flat_map: Vec<Vec<i32>> → Vec<i32> (flatten!)
    let flatted: Vec<i32> = nested.iter()
        .flat_map(|inner| inner.iter().map(|x| x * 10))
        .collect();
    println!("flat_map: {:?}", flatted);
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Law verification

Verify Functor laws cho `Result`:
```rust
let x: Result<i32, String> = Ok(5);
```
1. Identity: `x.map(|a| a) == x`?
2. Composition: `x.map(f).map(g) == x.map(|a| g(f(a)))`? (f = `*2`, g = `+10`)

<details><summary>✅ Lời giải</summary>

```rust
let x: Result<i32, String> = Ok(5);

// Law 1: Identity
assert_eq!(x.map(|a| a), Ok(5)); // ✅

// Law 2: Composition
let f = |n: i32| n * 2;
let g = |n: i32| n + 10;
assert_eq!(x.map(f).map(g), x.map(|a| g(f(a)))); // ✅ Ok(20) == Ok(20)
```

</details>

---

**Bài 2** (10 phút): Custom Functor

Tạo `Pair<T>` struct chứa 2 values cùng type. Implement `.map()` transform cả 2. Verify cả 2 Functor laws.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
#[derive(Debug, Clone, PartialEq)]
struct Pair<T>(T, T);

impl<T> Pair<T> {
    fn map<U, F: Fn(&T) -> U>(&self, f: F) -> Pair<U> {
        Pair(f(&self.0), f(&self.1))
    }
}

fn main() {
    let p = Pair(3, 7);
    println!("{:?}", p.map(|x| x * 2));  // Pair(6, 14)
    println!("{:?}", p.map(|x| x.to_string()));  // Pair("3", "7")

    // Law 1: Identity
    assert_eq!(p.map(|x| *x), Pair(3, 7));  // ✅

    // Law 2: Composition
    let f = |x: &i32| x * 2;
    let g = |x: &i32| x + 10;
    assert_eq!(p.map(f).map(g), p.map(|x| g(&f(x))));  // ✅
}
```

</details>

---

**Bài 3** (15 phút): Mappable trait + domain

Implement `Mappable` (GATs) cho custom `Cached<T>` type:
```rust
enum Cached<T> { Fresh(T), Stale(T, Duration), Missing }
```
`.map_items(f)` transform `T` bên trong Fresh/Stale, giữ Missing.

<details><summary>✅ Lời giải Bài 3</summary>

```rust
use std::time::Duration;

#[derive(Debug)]
enum Cached<T> {
    Fresh(T),
    Stale(T, Duration),
    Missing,
}

impl<T> Cached<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Cached<U> {
        match self {
            Cached::Fresh(val) => Cached::Fresh(f(val)),
            Cached::Stale(val, age) => Cached::Stale(f(val), age),
            Cached::Missing => Cached::Missing,
        }
    }
}

// GATs version
trait Mappable {
    type Item;
    type Output<U>;
    fn map_items<U, F: FnOnce(Self::Item) -> U>(self, f: F) -> Self::Output<U>;
}

impl<T> Mappable for Cached<T> {
    type Item = T;
    type Output<U> = Cached<U>;

    fn map_items<U, F: FnOnce(T) -> U>(self, f: F) -> Cached<U> {
        self.map(f)
    }
}

fn main() {
    let fresh = Cached::Fresh(42);
    println!("{:?}", fresh.map(|x| x.to_string()));

    let stale = Cached::Stale(100, Duration::from_secs(60));
    println!("{:?}", stale.map(|x| x * 2));

    let missing: Cached<i32> = Cached::Missing;
    println!("{:?}", missing.map(|x| x + 1));
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Functor trait không có trong std" | Rust thiếu HKT | Dùng `.map()` trực tiếp, hoặc GATs |
| `.map()` thay đổi container size | Law violation! | map giữ structure — dùng `filter_map` nếu muốn đổi |
| "Functor chỉ là `.map()`?" | Đúng! | Nhưng nhận ra pattern → dùng consistenly, compose tốt hơn |
| GATs quá phức tạp | Advanced Rust | Bắt đầu với `.map()` trực tiếp, GATs khi thực sự cần abstraction |

---

## Tóm tắt

- ✅ **Functor** = container + `.map()`. Transform giá trị bên trong, **giữ nguyên structure**.
- ✅ **Bạn đã dùng**: `Option::map`, `Result::map`, `Iterator::map` — tất cả là Functors.
- ✅ **2 Laws**: Identity (`map(id) == id`), Composition (`map(f).map(g) == map(f∘g)`).
- ✅ **Custom Functors**: `Labeled<T>`, `Tree<T>`, `Cached<T>` — implement `.map()`.
- ✅ **HKT limitation**: Rust không abstract Functor vào 1 trait. **GATs** = partial workaround.
- ✅ **Family**: `.map()` = Functor, `.and_then()` = Monad (Chapter 30!), `.map_err()` = Bifunctor.

## Tiếp theo

→ Chapter 30: **Monads in Rust — The Practical View** — bạn đã dùng monads hàng ngày! `Option + and_then = Maybe Monad`, `Result + ? = Either Monad`. Chúng ta sẽ demystify chúng.
