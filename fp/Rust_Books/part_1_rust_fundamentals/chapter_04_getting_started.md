# Chapter 4 — Getting Started with Rust

> **Bạn sẽ học được**:
> - Rust toolchain: `rustup`, `rustc`, `cargo` — và vai trò của từng tool
> - Cấu trúc project Rust từ đơn giản đến workspace
> - `Cargo.toml` — file cấu hình quan trọng nhất
> - Viết, build, run, test chương trình Rust
> - REPL với `evcxr` — thử nghiệm nhanh không cần tạo project
>
> **Yêu cầu trước**: Chapter 0 (Rust in 10 Minutes) — bạn đã cài Rust và chạy `cargo run` thành công.
> **Thời gian đọc**: ~30 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn tự tin tạo project mới, hiểu mọi file Cargo sinh ra, và có workflow phát triển mượt mà.

---

## Tại sao Rust đáng để học?

Bạn có thể đang tự hỏi: đã có Python, JavaScript, Go — tại sao thêm Rust? Câu trả lời nằm ở **trade-off cơ bản** mà mọi ngôn ngữ phải chọn: an toàn hay nhanh, dễ viết hay dễ maintain.

Python dễ viết nhưng chậm và thiếu type safety. C/C++ nhanh nhưng memory bugs là cơn ác mộng. Go cân bằng tốt nhưng garbage collector thêm latency. Rust là ngôn ngữ đầu tiên giải quyết trade-off này: **an toàn bộ nhớ tại compile time** mà không cần garbage collector. Zero-cost abstractions nghĩa là bạn viết code high-level nhưng chạy nhanh như C.

Trong cuộc khảo sát Stack Overflow 8 năm liên tiếp, Rust được bình chọn là "ngôn ngữ được yêu thích nhất". Không phải vì hype — mà vì developers trải nghiệm rồi không muốn quay lại. Compiler Rust khắt khe, nhưng khi code compile thành công, bạn có confidence rất cao rằng nó chạy đúng.

Chapter này giúp bạn setup môi trường và viết chương trình Rust đầu tiên. Đến cuối chapter, bạn sẽ có Rust toolchain sẵn sàng và hiểu workflow cơ bản.

---

## 4.1 — Toolchain: Bộ đồ nghề của thợ Rust

### Ba công cụ chính

Khi cài Rust qua `rustup`, bạn có 3 công cụ:

| Công cụ | Vai trò | Ẩn dụ |
|---------|---------|-------|
| `rustup` | Quản lý phiên bản Rust | Giống `nvm` cho Node.js |
| `rustc` | Compiler — biên dịch `.rs` thành binary | Giống `gcc` cho C |
| `cargo` | Build tool + package manager | Giống `npm` + `webpack` gộp lại |

Trong thực tế, bạn **gần như chỉ dùng `cargo`**. `cargo` tự gọi `rustc` cho bạn.

### `rustup` — Quản lý versions

```bash
# Xem version hiện tại
rustup show

# Update lên version mới nhất
rustup update

# Cài thêm components
rustup component add clippy     # linter
rustup component add rustfmt    # code formatter
```

### `cargo` — Mọi thứ bạn cần

```bash
# Tạo project mới
cargo new my_project        # binary (chạy được)
cargo new my_lib --lib      # library (dùng lại được)

# Build & Run
cargo build                 # compile → target/debug/
cargo build --release       # compile tối ưu → target/release/
cargo run                   # build + chạy
cargo run -- arg1 arg2      # chạy với arguments

# Kiểm tra
cargo check                 # kiểm tra lỗi nhanh (không tạo binary)
cargo test                  # chạy tests
cargo clippy                # lint — gợi ý code tốt hơn
cargo fmt                   # format code tự động
```

> **💡 Pro tip**: Dùng `cargo check` thay `cargo build` khi đang code. Nhanh hơn nhiều vì không tạo binary — chỉ kiểm tra lỗi.

---

## 4.2 — Project Structure: Anatomy của một Rust project

### `cargo new` tạo ra gì?

```bash
cargo new cafe_order
cd cafe_order
```

```
cafe_order/
├── Cargo.toml       # "Hộ khẩu" của project — tên, version, dependencies
├── .gitignore       # Bỏ qua target/ khi commit
└── src/
    └── main.rs      # Entry point — function main() bắt đầu từ đây
```

### `Cargo.toml` — File quan trọng nhất

```toml
# filename: Cargo.toml
[package]
name = "cafe_order"
version = "0.1.0"
edition = "2021"       # Rust edition — dùng 2021 cho mọi project mới

[dependencies]
# Dependencies sẽ được thêm ở đây
# Ví dụ:
# serde = { version = "1", features = ["derive"] }
# im = "15"
```

Mỗi phần:
- **`[package]`**: Metadata — tên, version, edition
- **`edition`**: Rust có 3 editions: 2015, 2018, 2021. Luôn dùng `2021`
- **`[dependencies]`**: Các thư viện bên ngoài (gọi là **crates**)

### Thêm dependency

```bash
# Cách 1: CLI (khuyên dùng)
cargo add serde --features derive
cargo add im

# Cách 2: Sửa trực tiếp Cargo.toml rồi chạy cargo build
```

Cargo tự tải crate từ [crates.io](https://crates.io) — kho thư viện chính thức của Rust.

### `Cargo.lock` — "Khóa" versions

Sau khi build lần đầu, Cargo tạo `Cargo.lock` — file ghi **chính xác** version đã dùng. Giống `package-lock.json` trong Node.js.

- **Binary project**: commit `Cargo.lock` vào git ✅
- **Library**: *không* commit, để người dùng tự resolve ❌

### Cấu trúc project lớn hơn

```
cafe_system/
├── Cargo.toml
├── src/
│   ├── main.rs          # entry point
│   ├── lib.rs           # library root (optional — khi muốn cả binary + library)
│   ├── order.rs         # module: domain logic cho orders
│   └── payment.rs       # module: payment processing
├── tests/
│   └── integration_test.rs  # integration tests
├── examples/
│   └── quick_demo.rs    # cargo run --example quick_demo
└── benches/
    └── performance.rs   # benchmarks
```

> **💡 Convention**: Rust dùng file system làm module system. File `order.rs` = module `order`. Sẽ học chi tiết ở Chapter 11.

---

## ✅ Checkpoint 4.2

> Ghi nhớ:
> 1. `Cargo.toml` = metadata + dependencies. `Cargo.lock` = khóa versions chính xác
> 2. `src/main.rs` = entry point binary. `src/lib.rs` = entry point library
> 3. `cargo check` nhanh hơn `cargo build` — dùng khi đang phát triển
>
> **Test nhanh**: `cargo new my_app --lib` tạo `main.rs` hay `lib.rs`?
> <details><summary>Đáp án</summary><code>lib.rs</code> — flag <code>--lib</code> tạo library project, không có <code>main()</code>.</details>

---

## 4.3 — Hello World chi tiết

### Phân tích từng dòng

```rust
// filename: src/main.rs

// fn = khai báo function
// main = tên function đặc biệt — Rust bắt đầu chạy từ đây
fn main() {
    // println! là MACRO (có dấu !), không phải function
    // Macro = code sinh ra code, mạnh hơn function thường
    println!("Hello, World!");
}
```

Tại sao `println!` có dấu `!`? Vì nó là **macro** — nó nhận format string và biến số arguments:

```rust
// filename: src/main.rs
fn main() {
    let name = "Rust";
    let version = 1.84;
    let features = vec!["ownership", "traits", "async"];

    // {} = Display format (đẹp, cho người đọc)
    println!("Hello, {}!", name);

    // {:.2} = số thực với 2 chữ số thập phân
    println!("Version: {:.2}", version);

    // {:?} = Debug format (chi tiết, cho dev)
    println!("Features: {:?}", features);

    // {:#?} = Debug format đẹp (xuống dòng, thụt lề)
    println!("Features (pretty):\n{:#?}", features);

    // Đặt tên cho placeholders
    println!("{language} is {adjective}!", language = "Rust", adjective = "awesome");

    // Output:
    // Hello, Rust!
    // Version: 1.84
    // Features: ["ownership", "traits", "async"]
    // Features (pretty):
    // [
    //     "ownership",
    //     "traits",
    //     "async",
    // ]
    // Rust is awesome!
}
```

### Comments

```rust
// filename: src/main.rs

// Dòng comment — giải thích logic
fn add(a: i32, b: i32) -> i32 {
    a + b // inline comment
}

/// Doc comment — tạo documentation tự động
/// Dùng cho public API: functions, structs, enums
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn documented_add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(2, 3));
    println!("{}", documented_add(2, 3));
    // Output:
    // 5
    // 5
}
```

Doc comments `///` đặc biệt: `cargo doc --open` sẽ tạo HTML documentation từ chúng. Code trong `/// # Examples` còn được chạy như test khi `cargo test`!

---

## 4.4 — REPL: Thử nghiệm nhanh với `evcxr`

Không phải lúc nào cũng cần tạo project mới. **`evcxr`** là REPL (Read-Eval-Print Loop) cho Rust:

```bash
# Cài đặt
cargo install evcxr_repl

# Chạy
evcxr
```

```
>> let x = 42;
>> x * 2
84
>> let names = vec!["Rust", "Go", "Zig"];
>> names.iter().map(|n| n.len()).collect::<Vec<_>>()
[4, 2, 3]
>> :quit
```

Rất hữu ích khi muốn thử nhanh một expression mà không cần tạo project.

> **💡 Alternative**: Nếu không muốn cài `evcxr`, dùng [Rust Playground](https://play.rust-lang.org/) trên trình duyệt — không cần cài gì.

---

## 4.5 — Workflow thực tế

### Development loop

```
1. cargo new my_project          ← tạo project
2. Viết code trong src/main.rs
3. cargo check                   ← kiểm tra lỗi nhanh
4. Sửa lỗi compiler báo
5. cargo run                     ← chạy thử
6. cargo test                    ← chạy tests
7. cargo clippy                  ← lint gợi ý cải thiện
8. cargo fmt                     ← format code
9. Lặp lại từ bước 2
```

### Ví dụ thực hành: Cafe Order System

Hãy tạo project thật và chạy:

```bash
cargo new cafe_order
cd cafe_order
```

Sửa `src/main.rs`:

```rust
// filename: src/main.rs

#[derive(Debug)]
enum DrinkType {
    Coffee,
    Tea,
    Smoothie,
}

#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
struct Order {
    drink: DrinkType,
    size: Size,
    customer: String,
}

fn price(order: &Order) -> u32 {
    let base = match order.drink {
        DrinkType::Coffee => 35_000,
        DrinkType::Tea => 25_000,
        DrinkType::Smoothie => 45_000,
    };

    // Size markup
    match order.size {
        Size::Small => base,
        Size::Medium => base + 5_000,
        Size::Large => base + 10_000,
    }
}

fn receipt(order: &Order) -> String {
    format!(
        "🧾 {}:\n   {:?} {:?} — {}đ",
        order.customer,
        order.size,
        order.drink,
        price(order)
    )
}

fn main() {
    let orders = vec![
        Order {
            drink: DrinkType::Coffee,
            size: Size::Medium,
            customer: "Minh".to_string(),
        },
        Order {
            drink: DrinkType::Smoothie,
            size: Size::Large,
            customer: "Lan".to_string(),
        },
        Order {
            drink: DrinkType::Tea,
            size: Size::Small,
            customer: "Hùng".to_string(),
        },
    ];

    println!("☕ Cafe Order System\n");

    let mut total = 0;
    for order in &orders {
        println!("{}", receipt(order));
        total += price(order);
    }

    println!("\n💰 Total: {}đ", total);
    println!("📊 Orders: {}", orders.len());
}
```

```bash
cargo run
```

```
☕ Cafe Order System

🧾 Minh:
   Medium Coffee — 40000đ
🧾 Lan:
   Large Smoothie — 55000đ
🧾 Hùng:
   Small Tea — 25000đ

💰 Total: 120000đ
📊 Orders: 3
```

### Thêm test

```rust
// filename: src/main.rs (thêm ở cuối file)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coffee_small_price() {
        let order = Order {
            drink: DrinkType::Coffee,
            size: Size::Small,
            customer: "Test".to_string(),
        };
        assert_eq!(price(&order), 35_000);
    }

    #[test]
    fn test_smoothie_large_price() {
        let order = Order {
            drink: DrinkType::Smoothie,
            size: Size::Large,
            customer: "Test".to_string(),
        };
        // Base 45_000 + Large 10_000 = 55_000
        assert_eq!(price(&order), 55_000);
    }

    #[test]
    fn test_tea_medium_price() {
        let order = Order {
            drink: DrinkType::Tea,
            size: Size::Medium,
            customer: "Test".to_string(),
        };
        // Base 25_000 + Medium 5_000 = 30_000
        assert_eq!(price(&order), 30_000);
    }
}
```

```bash
cargo test
```

```
running 3 tests
test tests::test_coffee_small_price ... ok
test tests::test_smoothie_large_price ... ok
test tests::test_tea_medium_price ... ok

test result: ok. 3 passed; 0 failed; 0 ignored
```

> **💡 `#[cfg(test)]`**: Block này chỉ được compile khi chạy `cargo test`. Không ảnh hưởng binary production.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Cargo commands

Ghép đúng command:

| Muốn làm gì | Command |
|---|---|
| Kiểm tra lỗi nhanh, không tạo binary | ? |
| Chạy tests | ? |
| Format code tự động | ? |
| Build bản production tối ưu | ? |
| Thêm dependency `serde` | ? |

<details><summary>✅ Lời giải Bài 1</summary>

| Muốn làm gì | Command |
|---|---|
| Kiểm tra lỗi nhanh | `cargo check` |
| Chạy tests | `cargo test` |
| Format code | `cargo fmt` |
| Build production | `cargo build --release` |
| Thêm dependency | `cargo add serde` |

</details>

---

**Bài 2** (10 phút): Tạo project "temperature converter"

Tạo `cargo new temp_converter`, viết function `celsius_to_fahrenheit(c: f64) -> f64` và `fahrenheit_to_celsius(f: f64) -> f64`. Thêm ít nhất 2 tests.

<details><summary>💡 Gợi ý</summary>Công thức: F = C × 9/5 + 32. C = (F - 32) × 5/9.</details>

<details><summary>✅ Lời giải Bài 2</summary>

```rust
// filename: src/main.rs

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    let temps_c = [0.0, 20.0, 37.0, 100.0];

    for &c in &temps_c {
        let f = celsius_to_fahrenheit(c);
        println!("{:.1}°C = {:.1}°F", c, f);
    }
    // Output:
    // 0.0°C = 32.0°F
    // 20.0°C = 68.0°F
    // 37.0°C = 98.6°F
    // 100.0°C = 212.0°F
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boiling_point() {
        assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 0.01);
    }

    #[test]
    fn test_freezing_point() {
        assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 0.01);
    }

    #[test]
    fn test_roundtrip() {
        let original = 37.5;
        let converted = fahrenheit_to_celsius(celsius_to_fahrenheit(original));
        assert!((converted - original).abs() < 0.01);
    }
}
```

</details>

---

**Bài 3** (15 phút): Mở rộng Cafe Order System

Thêm vào project `cafe_order`:
1. Enum `Topping { None, BubblePearl, Jelly }` — có thêm giá (0, 5000, 3000)
2. Thêm field `topping` vào `Order`
3. Cập nhật `price()` và `receipt()`
4. Thêm tests cho tổ hợp mới

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// Thêm Topping enum
#[derive(Debug)]
enum Topping {
    None,
    BubblePearl,
    Jelly,
}

fn topping_price(topping: &Topping) -> u32 {
    match topping {
        Topping::None => 0,
        Topping::BubblePearl => 5_000,
        Topping::Jelly => 3_000,
    }
}

// Cập nhật Order struct
#[derive(Debug)]
struct Order {
    drink: DrinkType,
    size: Size,
    topping: Topping,
    customer: String,
}

// Cập nhật price
fn price(order: &Order) -> u32 {
    let base = match order.drink {
        DrinkType::Coffee => 35_000,
        DrinkType::Tea => 25_000,
        DrinkType::Smoothie => 45_000,
    };
    let size_markup = match order.size {
        Size::Small => 0,
        Size::Medium => 5_000,
        Size::Large => 10_000,
    };
    base + size_markup + topping_price(&order.topping)
}

// Test
#[test]
fn test_coffee_medium_bubble() {
    let order = Order {
        drink: DrinkType::Coffee,
        size: Size::Medium,
        topping: Topping::BubblePearl,
        customer: "Test".to_string(),
    };
    // 35_000 + 5_000 + 5_000 = 45_000
    assert_eq!(price(&order), 45_000);
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `command not found: cargo` | Rust chưa cài hoặc PATH chưa set | Cài lại: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| `error[E0601]: main function not found` | Thiếu `fn main()` | Thêm `fn main() { }` vào `src/main.rs` |
| `cargo build` chậm lần đầu | Download + compile dependencies | Bình thường — lần sau nhanh hơn (cached) |
| `cargo test` chạy cả doc tests | Doc comments có code block | Bình thường — hoặc dùng `cargo test --lib` chỉ chạy unit tests |
| `evcxr` không cài được | Cần nightly hoặc thiếu dependencies | Dùng [play.rust-lang.org](https://play.rust-lang.org) thay thế |

---

## Tóm tắt

- ✅ **Toolchain**: `rustup` (version manager) + `cargo` (build tool + package manager). Gần như chỉ dùng `cargo`.
- ✅ **Project structure**: `Cargo.toml` (metadata + deps) + `src/main.rs` (entry point). `cargo new` tạo sẵn.
- ✅ **Development workflow**: `cargo check` → sửa → `cargo run` → `cargo test` → `cargo clippy` → `cargo fmt`.
- ✅ **`println!`** là macro: `{}` = Display, `{:?}` = Debug, `{:.2}` = 2 decimal places. Doc comments `///` tạo docs + tests.
- ✅ **REPL**: `evcxr` hoặc Rust Playground — thử nhanh không cần project.

## Tiếp theo

→ Chapter 5: **Variables, Types & Operators** — bạn sẽ học sâu hơn về `let`, `mut`, shadowing, scalar types, compound types (tuples, arrays), và type inference. Nền tảng để hiểu tại sao Rust là "statically typed nhưng viết ít type annotations".
