# Chapter 11 — Modules, Crates & Cargo

> **Bạn sẽ học được**:
> - Module system: `mod`, `pub`, `use` — tổ chức code thành cấu trúc rõ ràng
> - Visibility rules — ai thấy gì, kiểm soát public API
> - Crate structure — file layout conventions, `lib.rs` vs `main.rs`
> - `Cargo.toml` chi tiết — dependencies, features, profiles
> - Workspaces — quản lý multi-crate projects
>
> **Yêu cầu trước**: Chapter 4 (Getting Started), Chapter 9 (Ownership).
> **Thời gian đọc**: ~35 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn tổ chức project Rust chuyên nghiệp — code dễ tìm, API rõ ràng, dependencies được quản lý đúng cách.

---

## Module System — Tổ chức code cho dự án lớn

Khi dự án vượt quá 500 dòng, tổ chức code trở nên quan trọng hơn code itself. Rust's module system giúp bạn **phân tách concerns** rõ ràng: mỗi module có interface public (API) và implementation private (chi tiết bên trong). Module consumers chỉ thấy API, không phụ thuộc vào implementation.

Pattern này — hide implementation, expose interface — là nền tảng của mọi kiến trúc phần mềm tốt. Trong Rust, compiler thực thi nó: `pub` cho phép access, không `pub` = private. Không có cách "hack" qua.

---

## 11.1 — Modules: Tổ chức code thành "phòng"

### Ẩn dụ: Nhà có nhiều phòng

Imagine code như ngôi nhà. Nếu bạn bỏ mọi thứ vào **1 phòng** (1 file) — bếp, giường, bàn làm việc, toilet — rất hỗn loạn. Tốt hơn: chia thành **nhiều phòng** (modules), mỗi phòng một chức năng.

### Inline modules

```rust
// filename: src/main.rs

// Module = "phòng" trong code
mod kitchen {
    // pub = public — "cửa mở" cho bên ngoài thấy
    pub fn make_coffee(kind: &str) -> String {
        let water = boil_water();  // gọi private function trong cùng module
        format!("{} made with {}", kind, water)
    }

    // Không pub = private — "cửa đóng", chỉ module này dùng
    fn boil_water() -> &'static str {
        "fresh boiled water"
    }
}

mod cashier {
    pub fn calculate_total(items: &[u32]) -> u32 {
        items.iter().sum()
    }

    pub fn format_receipt(total: u32) -> String {
        format!("🧾 Total: {}đ", total)
    }
}

fn main() {
    // Dùng module::function
    let coffee = kitchen::make_coffee("Espresso");
    println!("{}", coffee);

    // kitchen::boil_water();  // ❌ private — không thể gọi từ ngoài!

    let total = cashier::calculate_total(&[35_000, 25_000, 45_000]);
    println!("{}", cashier::format_receipt(total));

    // Output:
    // Espresso made with fresh boiled water
    // 🧾 Total: 105000đ
}
```

### `use` — Shortcut để bớt gõ

```rust
// filename: src/main.rs

mod shapes {
    pub fn circle_area(radius: f64) -> f64 {
        std::f64::consts::PI * radius * radius
    }

    pub fn rect_area(width: f64, height: f64) -> f64 {
        width * height
    }
}

// use = import tên vào scope hiện tại
use shapes::circle_area;
use shapes::rect_area;

// Hoặc gom:
// use shapes::{circle_area, rect_area};

// Hoặc tất cả (cẩn thận — dễ name clash):
// use shapes::*;

fn main() {
    // Không cần shapes:: prefix nữa
    println!("Circle: {:.2}", circle_area(5.0));
    println!("Rect: {:.2}", rect_area(3.0, 4.0));
}
```

---

## 11.2 — File-based Modules: Module = File

### Convention: mỗi module = 1 file

Khi project lớn, inline modules quá dài. Rust cho phép **tách module ra file riêng**:

```
cafe_system/
├── Cargo.toml
└── src/
    ├── main.rs          # entry point
    ├── menu.rs           # mod menu
    ├── order.rs          # mod order
    └── payment.rs        # mod payment
```

```rust
// filename: src/main.rs

// Khai báo modules — Rust tìm file tương ứng
mod menu;      // → src/menu.rs
mod order;     // → src/order.rs
mod payment;   // → src/payment.rs

use menu::MenuItem;
use order::Order;

fn main() {
    let item = MenuItem::new("Coffee", 35_000);
    let order = Order::new("Minh", vec![item]);

    println!("{}", order.summary());

    let total = order.total();
    println!("{}", payment::format_receipt(total));
}
```

```rust
// filename: src/menu.rs

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub name: String,
    pub price: u32,
}

impl MenuItem {
    pub fn new(name: &str, price: u32) -> Self {
        MenuItem {
            name: name.to_string(),
            price,
        }
    }
}
```

```rust
// filename: src/order.rs

use crate::menu::MenuItem;  // crate:: = root của project

#[derive(Debug)]
pub struct Order {
    customer: String,  // private — không pub
    pub items: Vec<MenuItem>,
}

impl Order {
    pub fn new(customer: &str, items: Vec<MenuItem>) -> Self {
        Order {
            customer: customer.to_string(),
            items,
        }
    }

    pub fn total(&self) -> u32 {
        self.items.iter().map(|item| item.price).sum()
    }

    pub fn summary(&self) -> String {
        let items_str: Vec<String> = self.items.iter()
            .map(|i| format!("{} ({}đ)", i.name, i.price))
            .collect();
        format!("📋 {} — {}", self.customer, items_str.join(", "))
    }
}
```

```rust
// filename: src/payment.rs

pub fn format_receipt(total: u32) -> String {
    let tax = (total as f64 * 0.08) as u32;
    format!("🧾 Subtotal: {}đ\n   Tax (8%): {}đ\n   Total: {}đ",
        total, tax, total + tax)
}
```

### Sub-modules: Thư mục lồng nhau

Khi module có sub-modules:

```
src/
├── main.rs
└── payment/
    ├── mod.rs            # payment module root
    ├── cash.rs           # payment::cash
    └── card.rs           # payment::card
```

```rust
// filename: src/payment/mod.rs
pub mod cash;
pub mod card;

pub fn format_receipt(total: u32) -> String {
    format!("🧾 Total: {}đ", total)
}
```

```rust
// filename: src/payment/cash.rs
pub fn process(amount: u32) -> String {
    format!("💵 Cash payment: {}đ", amount)
}
```

```rust
// filename: src/payment/card.rs
pub fn process(amount: u32, card_number: &str) -> String {
    let last_four = &card_number[card_number.len()-4..];
    format!("💳 Card ****{}: {}đ", last_four, amount)
}
```

> **💡 Rust 2021 alternative**: Thay vì `payment/mod.rs`, có thể dùng `payment.rs` + thư mục `payment/`. Cả hai cách đều valid.

---

## ✅ Checkpoint 11.2

> Ghi nhớ:
> 1. `mod foo;` → Rust tìm `src/foo.rs` hoặc `src/foo/mod.rs`
> 2. `pub` = public, không `pub` = private (mặc định)
> 3. `crate::` = root, `super::` = module cha, `self::` = module hiện tại
>
> **Test nhanh**: `mod order;` trong `src/main.rs` tìm file nào?
> <details><summary>Đáp án</summary><code>src/order.rs</code> hoặc <code>src/order/mod.rs</code>.</details>

---

## 11.3 — Visibility: Ai thấy gì?

### Visibility levels

```rust
// filename: src/main.rs

mod cafe {
    pub struct Menu {
        pub name: String,       // pub: ai cũng thấy
        pub(crate) price: u32,  // pub(crate): chỉ trong crate này
        secret_recipe: String,  // private: chỉ module cafe
    }

    impl Menu {
        pub fn new(name: &str, price: u32) -> Self {
            Menu {
                name: name.to_string(),
                price,
                secret_recipe: "secret!".to_string(),
            }
        }

        // Private method — chỉ dùng trong module
        fn get_recipe(&self) -> &str {
            &self.secret_recipe
        }

        // Public method — dùng private fields bên trong
        pub fn describe(&self) -> String {
            format!("{}: {}đ (recipe: {})", self.name, self.price, self.get_recipe())
        }
    }
}

fn main() {
    let item = cafe::Menu::new("Latte", 45_000);

    // ✅ pub fields OK
    println!("Name: {}", item.name);
    println!("Price: {}", item.price);  // pub(crate) OK — cùng crate

    // ❌ Private field
    // println!("{}", item.secret_recipe);  // error: field is private
    // item.get_recipe();  // error: method is private

    // ✅ Public method truy cập private data
    println!("{}", item.describe());
}
```

| Visibility | Keyword | Ai thấy? |
|-----------|---------|----------|
| Private | (mặc định) | Chỉ module hiện tại + children |
| Pub crate | `pub(crate)` | Tất cả trong crate |
| Pub super | `pub(super)` | Module cha |
| Public | `pub` | Tất cả, kể cả external |

> **💡 Best practice**: Mặc định private. Chỉ `pub` những gì thực sự là API. Giống nguyên tắc "least privilege".

---

## 11.4 — Crate Structure: lib.rs vs main.rs

### Binary crate vs Library crate

| | Binary (`main.rs`) | Library (`lib.rs`) |
|---|---|---|
| Entry point | `fn main()` | Không cần main |
| Chạy bằng | `cargo run` | Không chạy trực tiếp |
| Mục đích | Ứng dụng chạy được | Code tái sử dụng |
| Ví dụ | CLI tool, server | SDK, utility library |

### Kết hợp cả hai

Project lớn thường có **cả hai**: library chứa logic, binary chỉ là thin wrapper:

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs       # library: chứa mọi logic
    └── main.rs      # binary: chỉ gọi library
```

```rust
// filename: src/lib.rs

pub mod menu;
pub mod order;
pub mod payment;

// Re-export quan trọng
pub use menu::MenuItem;
pub use order::Order;
```

```rust
// filename: src/main.rs

// Import từ library crate bằng tên project
use cafe_system::{MenuItem, Order};
use cafe_system::payment;

fn main() {
    let items = vec![
        MenuItem::new("Coffee", 35_000),
        MenuItem::new("Cake", 25_000),
    ];
    let order = Order::new("Minh", items);

    println!("{}", order.summary());
    println!("{}", payment::format_receipt(order.total()));
}
```

> **💡 Re-export `pub use`**: Cho phép users import từ top-level thay vì đường dẫn sâu. `use cafe_system::MenuItem` thay vì `use cafe_system::menu::MenuItem`.

---

## 11.5 — Cargo.toml chi tiết

### Dependencies

```toml
# filename: Cargo.toml

[package]
name = "cafe_system"
version = "0.1.0"
edition = "2021"
description = "A cafe order management system"
license = "MIT"

[dependencies]
# Từ crates.io — phổ biến nhất
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Version ranges
tokio = { version = "1.0", features = ["full"] }  # >= 1.0.0, < 2.0.0

# Từ Git
# my_lib = { git = "https://github.com/user/repo", branch = "main" }

# Từ local path (dev)
# my_lib = { path = "../my_lib" }

[dev-dependencies]
# Chỉ dùng khi test/benchmark — không vào production binary
assert_cmd = "2"
tempfile = "3"
```

### Version ranges

| Ký hiệu | Nghĩa | Ví dụ |
|----------|-------|-------|
| `"1.2.3"` | `>=1.2.3, <2.0.0` | Latest compatible |
| `"=1.2.3"` | Exact version | Chính xác |
| `">=1.0, <1.5"` | Range | Giới hạn |

### Features — Optional functionality

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }  # bật feature "derive"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }

# Tự định nghĩa features cho crate của bạn
[features]
default = ["json"]       # features bật mặc định
json = ["serde_json"]    # feature "json" bật serde_json
xml = ["quick-xml"]      # feature "xml" bật quick-xml
```

### Build profiles

```toml
# Dev build: nhanh compile, chậm runtime
[profile.dev]
opt-level = 0  # không tối ưu

# Release build: chậm compile, nhanh runtime
[profile.release]
opt-level = 3     # tối ưu tối đa
lto = true        # Link-Time Optimization
strip = true      # bỏ debug symbols → binary nhỏ hơn
```

```bash
cargo build            # dev profile
cargo build --release  # release profile
```

---

## 11.6 — Workspaces: Multi-crate projects

Khi project lớn, tách thành nhiều crates trong 1 workspace:

```
cafe_workspace/
├── Cargo.toml            # workspace root
├── cafe_core/             # crate 1: domain logic
│   ├── Cargo.toml
│   └── src/lib.rs
├── cafe_api/              # crate 2: REST API
│   ├── Cargo.toml
│   └── src/main.rs
└── cafe_cli/              # crate 3: CLI tool
    ├── Cargo.toml
    └── src/main.rs
```

```toml
# filename: Cargo.toml (workspace root)
[workspace]
members = [
    "cafe_core",
    "cafe_api",
    "cafe_cli",
]
```

```toml
# filename: cafe_api/Cargo.toml
[package]
name = "cafe_api"
version = "0.1.0"
edition = "2021"

[dependencies]
cafe_core = { path = "../cafe_core" }  # depend on sibling crate
```

```bash
# Build tất cả
cargo build

# Chỉ chạy 1 crate
cargo run -p cafe_cli

# Test tất cả
cargo test --workspace
```

> **💡 Workspace lợi ích**: Shared `Cargo.lock` (consistent versions), shared `target/` (compile 1 lần), dễ refactor giữa crates.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Module paths

Điền path đúng:

```rust
// src/main.rs
mod utils;
mod models;

// Trong src/models.rs:
// Gọi function greet() từ utils module
// → ???::greet()

// Trong src/utils.rs:
// Gọi struct Order từ models module
// → ???::Order
```

<details><summary>✅ Lời giải Bài 1</summary>

```rust
// Trong src/models.rs:
crate::utils::greet()   // crate:: = project root

// Trong src/utils.rs:
crate::models::Order    // crate:: = project root

// Hoặc dùng use:
use crate::utils::greet;
use crate::models::Order;
```

</details>

---

**Bài 2** (10 phút): Tổ chức code

Refactor code sau từ 1 file thành 3 modules (`menu`, `order`, `payment`):

```rust
struct MenuItem { name: String, price: u32 }
struct Order { customer: String, items: Vec<MenuItem> }
fn calculate_total(items: &[MenuItem]) -> u32 { /* ... */ }
fn format_receipt(total: u32) -> String { /* ... */ }
```

<details><summary>✅ Lời giải Bài 2</summary>

```
src/
├── main.rs           # mod menu; mod order; mod payment;
├── menu.rs           # pub struct MenuItem
├── order.rs          # pub struct Order (use crate::menu::MenuItem)
└── payment.rs        # pub fn calculate_total, pub fn format_receipt
```

Key decisions:
- `MenuItem` → `menu.rs` (domain object)
- `Order` → `order.rs` (depends on MenuItem via `use crate::menu::MenuItem`)
- `calculate_total` + `format_receipt` → `payment.rs` (business logic)
- `main.rs` glues everything: `use menu::MenuItem; use order::Order;`

</details>

---

**Bài 3** (15 phút): Mini library crate

Tạo library crate `math_utils` với:
- Module `stats`: `mean(&[f64])`, `median(&[f64])`, `mode(&[f64])`
- Module `convert`: `celsius_to_fahrenheit(f64)`, `km_to_miles(f64)`
- Re-export các functions hay dùng từ `lib.rs`
- Viết tests trong mỗi module

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/lib.rs
pub mod stats;
pub mod convert;

// Re-exports
pub use stats::{mean, median};
pub use convert::{celsius_to_fahrenheit, km_to_miles};
```

```rust
// filename: src/stats.rs
use std::collections::HashMap;

pub fn mean(data: &[f64]) -> Option<f64> {
    if data.is_empty() { return None; }
    Some(data.iter().sum::<f64>() / data.len() as f64)
}

pub fn median(data: &[f64]) -> Option<f64> {
    if data.is_empty() { return None; }
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

pub fn mode(data: &[i64]) -> Option<i64> {
    if data.is_empty() { return None; }
    let mut counts = HashMap::new();
    for &val in data {
        *counts.entry(val).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(&[1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
    }
}
```

```rust
// filename: src/convert.rs
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn km_to_miles(km: f64) -> f64 {
    km * 0.621371
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boiling() {
        assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 0.01);
    }

    #[test]
    fn test_marathon() {
        assert!((km_to_miles(42.195) - 26.219).abs() < 0.01);
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `unresolved import` | Path sai hoặc quên `pub` | Kiểm tra: `mod` khai báo chưa? Item có `pub` không? |
| `file not found for module` | File không ở đúng vị trí | `mod foo;` → cần `src/foo.rs` hoặc `src/foo/mod.rs` |
| `field is private` | Struct field không pub | Thêm `pub` hoặc tạo constructor/getter |
| `function is private` | Quên `pub` trên function | Thêm `pub fn` thay vì `fn` |
| Circular dependency giữa modules | Module A dùng B, B dùng A | Refactor: tách shared types ra module riêng |

---

## Tóm tắt

- ✅ **Modules** = tổ chức code. `mod foo;` → `src/foo.rs`. Mặc định private, `pub` cho public.
- ✅ **Paths**: `crate::` (root), `super::` (cha), `self::` (mình). `use` cho shortcuts.
- ✅ **Visibility**: private < `pub(super)` < `pub(crate)` < `pub`. Nguyên tắc least privilege.
- ✅ **Crate structure**: `lib.rs` cho library, `main.rs` cho binary. `pub use` cho re-exports.
- ✅ **Cargo.toml**: dependencies + features + profiles. **Workspaces** cho multi-crate projects.

---

## 🎉 Kết thúc Part I!

Chúc mừng! Bạn đã hoàn thành **Part I: Rust Fundamentals** — 8 chapters từ Getting Started đến Modules. Bạn giờ đã biết:

- ✅ Viết, build, test project Rust
- ✅ Variables, types, control flow, functions, closures
- ✅ Collections: Vec, String, HashMap, Iterator chains
- ✅ **Ownership & Borrowing** — superpower chỉ Rust có
- ✅ Error handling chuyên nghiệp với Result/Option/?
- ✅ Tổ chức code thành modules, crates, workspaces

## Tiếp theo

→ **Part II: Thinking Functionally** bắt đầu! Chapter 12: **Immutability & Purity** — bạn sẽ học tại sao immutability quan trọng, pure functions (không side-effects, deterministic output), và cách Rust enforce purity qua `let` immutable và `&T` shared reference. Đây là nền tảng tư duy FP cho mọi chapter từ đây về sau.
