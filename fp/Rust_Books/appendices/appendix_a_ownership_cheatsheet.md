# Appendix A — Rust Ownership Cheat Sheet

> Quick reference cho ownership, borrowing, lifetimes. In ra dán cạnh màn hình! 🖨️

---

## A.1 — Ownership Rules

| Rule | Ý nghĩa |
|------|---------|
| **1. Mỗi value có đúng 1 owner** | `let s = String::from("hi")` → `s` owns string |
| **2. Khi owner ra khỏi scope → value dropped** | `{ let s = ...; } // s dropped here` |
| **3. Assignment = move** (non-Copy types) | `let s2 = s;` → `s` không dùng được nữa |

---

## A.2 — Move vs Copy vs Clone

```rust
// ═══ MOVE (default cho heap types) ═══
let s1 = String::from("hello");
let s2 = s1;          // s1 MOVED → s2
// println!("{}", s1); // ❌ error: value moved

// ═══ COPY (stack types, implicit) ═══
let x: i32 = 5;
let y = x;            // x COPIED → y
println!("{}", x);    // ✅ still valid

// ═══ CLONE (explicit deep copy) ═══
let s1 = String::from("hello");
let s2 = s1.clone();  // deep copy
println!("{}", s1);   // ✅ both valid
```

### Copy types (stack-only, bitwise copy)

| Type | Copy? | Note |
|------|-------|------|
| `i32`, `u64`, `f64`, `bool`, `char` | ✅ | All scalar types |
| `(i32, bool)` | ✅ | Tuple of Copy types |
| `[i32; 3]` | ✅ | Array of Copy types |
| `&T` | ✅ | Shared references |
| `String`, `Vec<T>`, `Box<T>` | ❌ | Heap data → move |
| `&mut T` | ❌ | Exclusive → move |

---

## A.3 — References & Borrowing

```rust
// ═══ Shared reference: &T ═══
fn len(s: &String) -> usize { s.len() }   // borrows, doesn't own
let s = String::from("hello");
let n = len(&s);     // &s = borrow
println!("{}", s);   // ✅ s still valid

// ═══ Mutable reference: &mut T ═══
fn push_hi(s: &mut String) { s.push_str(" hi"); }
let mut s = String::from("hello");
push_hi(&mut s);     // mutable borrow
println!("{}", s);   // "hello hi"
```

### Borrowing rules

| Rule | Allowed | Forbidden |
|------|---------|-----------|
| Multiple `&T` | `let a = &s; let b = &s;` ✅ | — |
| One `&mut T` | `let m = &mut s;` ✅ | `let a = &s; let m = &mut s;` ❌ |
| `&T` + `&mut T` same time | — | ❌ Cannot mix |

> **Tóm gọn**: Nhiều readers HOẶC 1 writer. Không bao giờ cả hai.

---

## A.4 — Lifetimes

```rust
// Lifetime = "reference sống được bao lâu"
// Compiler tự suy luận (elision) hầu hết cases

// ═══ Explicit lifetime ═══
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// 'a = "result sống bằng thời gian ngắn hơn của x và y"

// ═══ Struct chứa reference ═══
struct Excerpt<'a> {
    text: &'a str,  // struct không sống lâu hơn text
}
```

### Lifetime elision rules (compiler tự thêm)

| Rule | Input | Output |
|------|-------|--------|
| **1** | Mỗi ref param nhận lifetime riêng | `fn f(x: &str, y: &str)` → `fn f<'a, 'b>(x: &'a str, y: &'b str)` |
| **2** | 1 input ref → output cùng lifetime | `fn f(x: &str) -> &str` → `fn f<'a>(x: &'a str) -> &'a str` |
| **3** | `&self` → output = `'self` | `fn f(&self) -> &str` → lifetime of self |

---

## A.5 — Quick Decision Tree

```
Cần dùng value sau khi truyền vào function?
├── Không → Pass by value (move/copy)
└── Có
    ├── Chỉ đọc → &T (shared borrow)
    └── Cần modify → &mut T (mutable borrow)

Cần shared ownership?
├── Single thread → Rc<T>
└── Multi thread → Arc<T>

Cần interior mutability?
├── Single thread → RefCell<T>
└── Multi thread → Mutex<T> / RwLock<T>

Cần heap allocation?
├── Single value → Box<T>
├── Dynamic array → Vec<T>
└── Recursive type → Box<T>
```

---

## A.6 — Common Patterns

```rust
// ═══ Return owned value (safe, simple) ═══
fn create() -> String { String::from("hello") }

// ═══ Accept borrow (flexible) ═══
fn process(s: &str) { /* read-only */ }
// Accepts both &String and &str!

// ═══ Accept Into<String> (ergonomic) ═══
fn greet(name: impl Into<String>) {
    let name = name.into();
    println!("Hello, {}!", name);
}
greet("world");                    // &str → String
greet(String::from("world"));     // String → String

// ═══ Cow (Clone on Write) ═══
use std::borrow::Cow;
fn maybe_modify(s: &str) -> Cow<str> {
    if s.contains("bad") {
        Cow::Owned(s.replace("bad", "good"))  // allocates only if needed
    } else {
        Cow::Borrowed(s)  // zero cost
    }
}
```
