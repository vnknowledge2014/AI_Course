# Appendix B — From F#/PureScript to Rust

> Translation table cho developers đến từ F#, PureScript, Haskell. Tìm khái niệm quen thuộc → Rust equivalent.

---

## B.1 — Type System

| F# / PureScript | Rust | Note |
|-----------------|------|------|
| `int`, `float` | `i32`, `f64` | Rust có nhiều sizes: `i8`–`i128` |
| `string` | `String` / `&str` | Owned vs borrowed |
| `bool` | `bool` | Giống |
| `unit` (`()`) | `()` | Giống |
| `'a` (type variable) | `T` (generic) | `fn id<T>(x: T) -> T` |
| `option<'a>` / `Maybe a` | `Option<T>` | `Some(x)` / `None` |
| `Result<'a, 'err>` / `Either e a` | `Result<T, E>` | `Ok(x)` / `Err(e)` |
| `list<'a>` / `List a` | `Vec<T>` | Rust Vec = mutable, contiguous |
| `Map<k, v>` | `HashMap<K, V>` | `use std::collections` |
| `Set<'a>` | `HashSet<T>` | `use std::collections` |
| `tuple` `(a, b)` | `(A, B)` | Giống |
| `array<'a>` | `[T; N]` (fixed) / `&[T]` (slice) | Fixed size vs dynamic |

---

## B.2 — Algebraic Types

| F# / PureScript | Rust | Example |
|-----------------|------|---------|
| **Record** | `struct` | `struct User { name: String, age: u32 }` |
| **Discriminated Union** / **ADT** | `enum` | `enum Shape { Circle(f64), Rect(f64, f64) }` |
| **Newtype** | `struct Name(Inner)` | `struct Email(String)` |
| **Type alias** | `type Name = ...` | `type UserId = u64` |
| **Anonymous record** | Tuple struct / tuple | `struct Point(f64, f64)` |

```fsharp
// F#
type Shape =
    | Circle of radius: float
    | Rectangle of width: float * height: float
```

```rust
// Rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}
```

---

## B.3 — Pattern Matching

| F# | Rust |
|----|------|
| `match x with` | `match x {` |
| `\| Some v -> ...` | `Some(v) => ...` |
| `\| None -> ...` | `None => ...` |
| `\| _ -> ...` (wildcard) | `_ => ...` |
| `when` guard | `if` guard |
| `as` binding | `@` binding |

```fsharp
// F#
match shape with
| Circle r when r > 0.0 -> pi * r * r
| Rectangle (w, h) -> w * h
| _ -> 0.0
```

```rust
// Rust
match shape {
    Shape::Circle { radius: r } if r > 0.0 => PI * r * r,
    Shape::Rectangle { width: w, height: h } => w * h,
    _ => 0.0,
}
```

---

## B.4 — Functions & Composition

| F# / PureScript | Rust | Note |
|-----------------|------|------|
| `let f x = x + 1` | `fn f(x: i32) -> i32 { x + 1 }` | Explicit types in Rust |
| `fun x -> x + 1` / `\x -> x + 1` | `\|x\| x + 1` | Closures |
| `f >> g` (compose) | `\|x\| g(f(x))` | No built-in operator |
| `x \|> f` (pipe) | `f(x)` or method chain | `.map().filter()` |
| `List.map f xs` | `xs.iter().map(f)` | Iterator-based |
| `List.filter f xs` | `xs.iter().filter(f)` | Iterator-based |
| `List.fold init f xs` | `xs.iter().fold(init, f)` | Iterator-based |
| Partial application | Closures | `let add5 = \|x\| add(5, x);` |
| Currying (auto) | Manual closures | Not automatic in Rust |

```fsharp
// F# pipeline
[1; 2; 3; 4; 5]
|> List.filter (fun x -> x % 2 = 0)
|> List.map (fun x -> x * 10)
|> List.sum
```

```rust
// Rust iterator chain
vec![1, 2, 3, 4, 5].iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 10)
    .sum::<i32>()
```

---

## B.5 — Typeclasses → Traits

| PureScript / Haskell | Rust | Note |
|---------------------|------|------|
| `class Show a` | `trait Display` | `fmt::Display` |
| `class Eq a` | `trait PartialEq + Eq` | Derive with `#[derive(Eq)]` |
| `class Ord a` | `trait PartialOrd + Ord` | Derive with `#[derive(Ord)]` |
| `class Semigroup a` | Custom `trait Append` | No stdlib equivalent |
| `class Monoid a` | Custom trait | `Default` ≈ `mempty` |
| `class Functor f` | No trait (no HKTs) | `.map()` by convention |
| `class Monad m` | No trait | `and_then` by convention |
| `class Foldable t` | `trait Iterator` | Rust iterators = foldable |
| `instance Show MyType` | `impl Display for MyType` | |
| `derive` | `#[derive(...)]` | Proc macros |
| Orphan rule | ✅ Same rule | Cannot impl external trait for external type |

---

## B.6 — Error Handling

| F# / PureScript | Rust | Note |
|-----------------|------|------|
| `Result.map` | `Result::map` | Giống |
| `Result.bind` / `>>=` | `and_then` / `?` | `?` = monadic bind |
| `Result.mapError` | `map_err` | |
| Railway-Oriented | `?` operator chain | Built into language! |
| `try...with` | `match` on `Result` | No exceptions in Rust |
| `raise`/`throw` | `panic!` (unrecoverable) | Avoid in library code |

```fsharp
// F#
let result =
    validateName input
    |> Result.bind validateEmail
    |> Result.bind saveToDb
```

```rust
// Rust
fn process(input: Input) -> Result<Output, Error> {
    let name = validate_name(input)?;
    let email = validate_email(&name)?;
    save_to_db(&email)
}
```

---

## B.7 — Key Differences

| Concept | F#/PureScript | Rust |
|---------|---------------|------|
| **Memory** | GC (garbage collected) | Ownership (compile-time) |
| **Mutability** | Immutable default | Immutable default + `mut` |
| **Null** | `Option` (F#), no null (PS) | `Option` ✅ no null |
| **Effects** | Monads (PS), computation expressions (F#) | Direct with `Result`/`async` |
| **HKTs** | ✅ Full support | ❌ No (GATs partial) |
| **Inference** | Hindley-Milner (full) | Local inference only |
| **Compilation** | JIT (F#), JS (PS) | Native binary (LLVM) |
| **Performance** | Good (F#), varies (PS) | C/C++ level |
