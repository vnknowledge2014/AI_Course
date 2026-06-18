# Appendix D — Macros Essentials

> Appendix này là **reference**, không phải tutorial. Bạn không cần viết macros để dùng Rust hiệu quả — nhưng CẦN HIỂU chúng vì hệ sinh thái phụ thuộc rất nhiều vào macros.

---

## D.1 — Declarative Macros (`macro_rules!`)

### Macro đơn giản nhất

```rust
// filename: src/main.rs

// Macro = "code viết code" tại compile time
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

// Macro nhận tham số
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}! 👋", $name);
    };
}

fn main() {
    say_hello!();         // Hello from macro!
    greet!("Rust");       // Hello, Rust! 👋
    greet!(42);           // Hello, 42! 👋 (bất kỳ expression nào)
}
```

### Variadic — Nhận nhiều tham số

```rust
// filename: src/main.rs

// $($x:expr),* = zero or more expressions, separated by commas
macro_rules! make_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($x); )*
            v
        }
    };
}

// HashMap literal (thường gặp trong projects)
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $val); )*
            map
        }
    };
}

fn main() {
    let nums = make_vec![1, 2, 3, 4, 5];
    println!("{:?}", nums);  // [1, 2, 3, 4, 5]

    let config = hashmap! {
        "host" => "localhost",
        "port" => "8080",
        "debug" => "true",
    };
    println!("{:?}", config);
}
```

### Patterns phổ biến

| Pattern | Nghĩa | Ví dụ |
|---------|--------|-------|
| `$x:expr` | Expression | `42`, `a + b`, `foo()` |
| `$x:ident` | Identifier | `my_var`, `Config` |
| `$x:ty` | Type | `i32`, `String`, `Vec<T>` |
| `$x:tt` | Token tree | Bất kỳ token nào |
| `$($x:expr),*` | Zero or more | `1, 2, 3` |
| `$($x:expr),+` | One or more | Ít nhất 1 |
| `$(,)?` | Optional trailing comma | Cho phép trailing comma |

### Hygiene — Tại sao Rust macros an toàn hơn C preprocessor

```rust
// C preprocessor: text substitution → bugs!
// #define DOUBLE(x) x * x
// DOUBLE(1 + 2) → 1 + 2 * 1 + 2 = 5 (wrong!)

// Rust macro: AST-level → an toàn!
macro_rules! double {
    ($x:expr) => { $x * $x };
}

fn main() {
    println!("{}", double!(1 + 2));  // (1+2) * (1+2) = 9 ✅
    // Rust treats $x as EXPRESSION, not text!
}
```

---

## D.2 — Attribute Macros (Procedural Macros)

### Bạn **dùng** chúng mỗi ngày

```rust
// `#[derive(Debug)]` → compiler tạo impl Debug cho bạn
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

// `#[tokio::main]` → biến main() thành async runtime
// Expand thành:
// fn main() {
//     tokio::runtime::Runtime::new()
//         .unwrap()
//         .block_on(async { /* your code */ })
// }

// `#[test]` → đánh dấu function là test
#[test]
fn test_something() {
    assert_eq!(2 + 2, 4);
}

// `#[serde(rename_all = "camelCase")]` → JSON serialization
// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct ApiResponse {
//     user_name: String,      // → "userName" trong JSON
//     created_at: String,     // → "createdAt" trong JSON
// }
```

### 3 loại procedural macros

| Loại | Cú pháp | Ví dụ | Khi nào viết? |
|------|---------|-------|---------------|
| **Derive** | `#[derive(MyTrait)]` | `Debug`, `Clone`, `Serialize` | Khi trait có pattern lặp |
| **Attribute** | `#[my_attr]` | `#[tokio::main]`, `#[test]` | Khi cần transform function |
| **Function-like** | `my_macro!(...)` | `println!()`, `vec![]` | DSL, code generation |

> **💡 Bạn KHÔNG CẦN viết procedural macros** — chúng phức tạp (cần separate crate, parse token streams). Chỉ cần **hiểu** cách dùng chúng và **đọc** documentation của crates.

---

## D.3 — Macros vs Functions: Khi nào dùng gì?

| Tiêu chí | Function | Macro |
|----------|----------|-------|
| **Variadic args** | ❌ Không hỗ trợ | ✅ `$(args),*` |
| **Compile-time** | ❌ Runtime only | ✅ Generate code |
| **Type checking** | ✅ Compiler checks | ⚠️ Errors khó đọc |
| **Debug** | ✅ Dễ debug | ❌ Khó debug |
| **IDE support** | ✅ Autocomplete | ⚠️ Hạn chế |
| **Khi nào dùng** | **Hầu hết mọi lúc** | Variadic, DSL, boilerplate reduction |

**Quy tắc**: Dùng function trước. Chỉ dùng macro khi function **không thể** (variadic args, compile-time code gen, hoặc giảm repetitive boilerplate).
