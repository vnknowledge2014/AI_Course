# Chapter 32 — Recursive Types & Folds

> **Bạn sẽ học được**:
> - **Recursive types** — `enum Expr { Lit(i32), Add(Box<Expr>, Box<Expr>) }`
> - **`Box<T>`** cho recursive types — tại sao Rust cần nó
> - **Fold** (catamorphism) — universal pattern cho processing recursive types
> - Tree traversal — in-order, pre-order, post-order
> - Expression evaluator — parsers → AST → evaluate
> - **Visitor pattern** = fold in disguise
>
> **Yêu cầu trước**: Chapter 14 (Enums), Chapter 28 (Algebra), Chapter 31 (Parsers).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn model và process **bất kỳ recursive structure nào** — trees, expressions, file systems — bằng enums + fold.

---

## Recursive Types & Folds — Cấu trúc dữ liệu đệ quy

Trees, lists, ASTs, file systems, HTML DOM — tất cả đều là **recursive data structures**: cấu trúc chứa chính mình. Trong OOP, bạn xử lý chúng bằng Visitor pattern + dynamic dispatch. Trong FP, bạn dùng **folds** (catamorphisms) — hàm duyệt cấu trúc đệ quy và "gấp" nó thành giá trị đơn.

Fold là tổng quát hóa của `iter().fold()` mà bạn đã dùng với lists — nhưng áp dụng cho bất kỳ recursive type nào: trees, expressions, JSON...

---

## 32.1 — Recursive Types & Box

### Búp bê Matryoshka

Bạn biết búp bê Nga Matryoshka? Mở búp bê ngoài — bên trong có búp bê nhỏ hơn. Mở tiếp — thêm một búp bê. Cứ thế cho đến búp bê cuối cùng — nhỏ nhất, rỗng, không chứa gì nữa.

Recursive types trong Rust giống hệt: `Expr::Add` chứa 2 `Expr` bên trong, mỗi `Expr` lại có thể chứa `Expr` khác. Búp bê cuối cùng (base case) là `Expr::Lit(42)` — không chứa `Expr` nào nữa.

Nhưng có vấn đề: Rust cần biết **kích thước** mỗi type lúc compile. Búp bê lồng vô hạn = kích thước vô hạn. `Box<T>` giải quyết: thay vì chứa búp bê con trực tiếp, bạn để búp bê con trên kệ (heap) và giữ **tấm thẻ ghi địa chỉ** (pointer). Tấm thẻ luôn cùng kích thước — 8 bytes.

### Vấn đề: Enum tham chiếu chính nó

```rust
// ❌ KHÔNG COMPILE!
// enum Expr {
//     Lit(i32),
//     Add(Expr, Expr),  // Error: recursive type has infinite size
// }
```

Rust cần biết **size** lúc compile time. `Expr` chứa `Expr` chứa `Expr`... = infinite.

### Giải pháp: `Box<T>` = heap allocation

```rust
// filename: src/main.rs

// ═══ Arithmetic expressions ═══
#[derive(Debug, Clone)]
enum Expr {
    Lit(i32),                           // number literal: 42
    Add(Box<Expr>, Box<Expr>),          // a + b
    Mul(Box<Expr>, Box<Expr>),          // a * b
    Neg(Box<Expr>),                     // -a
}

// Helper: tạo Box<Expr> gọn hơn
fn lit(n: i32) -> Box<Expr> { Box::new(Expr::Lit(n)) }
fn add(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> { Box::new(Expr::Add(a, b)) }
fn mul(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> { Box::new(Expr::Mul(a, b)) }
fn neg(a: Box<Expr>) -> Box<Expr> { Box::new(Expr::Neg(a)) }

fn main() {
    // (3 + 4) * -(5 + 2)
    let expr = mul(
        add(lit(3), lit(4)),
        neg(add(lit(5), lit(2)))
    );
    println!("{:?}", expr);
}
```

> **💡 `Box<T>`** = pointer tới heap. Size cố định (8 bytes trên 64-bit). Giải quyết infinite size problem.

---

## 32.2 — Evaluate: Pattern Matching Recursive

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
enum Expr {
    Lit(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn lit(n: i32) -> Box<Expr> { Box::new(Expr::Lit(n)) }
fn add(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> { Box::new(Expr::Add(a, b)) }
fn mul(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> { Box::new(Expr::Mul(a, b)) }
fn neg(a: Box<Expr>) -> Box<Expr> { Box::new(Expr::Neg(a)) }

// ═══ Evaluate: recursive pattern matching ═══
fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Lit(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(a) => -eval(a),
    }
}

// ═══ Pretty-print ═══
fn display(expr: &Expr) -> String {
    match expr {
        Expr::Lit(n) => n.to_string(),
        Expr::Add(a, b) => format!("({} + {})", display(a), display(b)),
        Expr::Mul(a, b) => format!("({} × {})", display(a), display(b)),
        Expr::Neg(a) => format!("(-{})", display(a)),
    }
}

// ═══ Count nodes ═══
fn count_nodes(expr: &Expr) -> usize {
    match expr {
        Expr::Lit(_) => 1,
        Expr::Add(a, b) | Expr::Mul(a, b) => 1 + count_nodes(a) + count_nodes(b),
        Expr::Neg(a) => 1 + count_nodes(a),
    }
}

// ═══ Depth of expression tree ═══
fn depth(expr: &Expr) -> usize {
    match expr {
        Expr::Lit(_) => 1,
        Expr::Add(a, b) | Expr::Mul(a, b) => 1 + depth(a).max(depth(b)),
        Expr::Neg(a) => 1 + depth(a),
    }
}

fn main() {
    // (3 + 4) * -(5 + 2)
    let expr = mul(add(lit(3), lit(4)), neg(add(lit(5), lit(2))));

    println!("Expression: {}", display(&expr));
    println!("Result: {}", eval(&expr));  // 7 * -7 = -49
    println!("Nodes: {}", count_nodes(&expr));
    println!("Depth: {}", depth(&expr));
}
```

> **Nhận ra pattern?** `eval`, `display`, `count_nodes`, `depth` — tất cả cùng structure: match mỗi variant, recurse vào children, combine kết quả. Đó là **fold**!

---

## 32.3 — Fold: Universal Pattern (Catamorphism)

### Fold = "collapse recursive structure thành 1 value"

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
enum Expr {
    Lit(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn lit(n: i32) -> Box<Expr> { Box::new(Expr::Lit(n)) }
fn add(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> { Box::new(Expr::Add(a, b)) }
fn mul(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> { Box::new(Expr::Mul(a, b)) }
fn neg(a: Box<Expr>) -> Box<Expr> { Box::new(Expr::Neg(a)) }

// ═══ Generic fold (catamorphism) ═══
// Thay vì viết eval, display, count riêng → 1 fold function
fn fold<T>(
    expr: &Expr,
    on_lit: &dyn Fn(i32) -> T,
    on_add: &dyn Fn(T, T) -> T,
    on_mul: &dyn Fn(T, T) -> T,
    on_neg: &dyn Fn(T) -> T,
) -> T {
    match expr {
        Expr::Lit(n) => on_lit(*n),
        Expr::Add(a, b) => {
            let va = fold(a, on_lit, on_add, on_mul, on_neg);
            let vb = fold(b, on_lit, on_add, on_mul, on_neg);
            on_add(va, vb)
        }
        Expr::Mul(a, b) => {
            let va = fold(a, on_lit, on_add, on_mul, on_neg);
            let vb = fold(b, on_lit, on_add, on_mul, on_neg);
            on_mul(va, vb)
        }
        Expr::Neg(a) => {
            let va = fold(a, on_lit, on_add, on_mul, on_neg);
            on_neg(va)
        }
    }
}

fn main() {
    let expr = mul(add(lit(3), lit(4)), neg(add(lit(5), lit(2))));

    // eval = fold with arithmetic operations
    let result = fold(&expr,
        &|n| n,
        &|a, b| a + b,
        &|a, b| a * b,
        &|a| -a,
    );
    println!("Eval: {}", result);  // -49

    // display = fold with string formatting
    let text = fold(&expr,
        &|n| n.to_string(),
        &|a, b| format!("({} + {})", a, b),
        &|a, b| format!("({} × {})", a, b),
        &|a| format!("(-{})", a),
    );
    println!("Display: {}", text);

    // count = fold with counting
    let nodes = fold(&expr,
        &|_| 1_usize,
        &|a, b| 1 + a + b,
        &|a, b| 1 + a + b,
        &|a| 1 + a,
    );
    println!("Nodes: {}", nodes);

    // depth = fold with max
    let d = fold(&expr,
        &|_| 1_usize,
        &|a, b| 1 + a.max(b),
        &|a, b| 1 + a.max(b),
        &|a| 1 + a,
    );
    println!("Depth: {}", d);

    // all_literals = fold collecting values
    let lits = fold(&expr,
        &|n| vec![n],
        &|mut a, b| { a.extend(b); a },
        &|mut a, b| { a.extend(b); a },
        &|a| a,
    );
    println!("Literals: {:?}", lits);  // [3, 4, 5, 2]
}
```

> **💡 Catamorphism**: 1 fold function → vô số operations. Thay callback → thay behavior. **"Tell me what to do at each node, I'll traverse for you."**

---

## ✅ Checkpoint 32.3

> Ghi nhớ:
> 1. **Fold** = "define handler cho mỗi variant → fold traverses + combines"
> 2. `eval` = fold with `(+, *, -, id)`. `display` = fold with `(format, format, format, to_string)`.
> 3. Fold **tách traversal khỏi logic** — same traversal, different operations.
> 4. Catamorphism = fold cho recursive types. Bạn đã dùng `fold` cho `Vec` (Chapter 13) — đây là generalization!

---

## 32.4 — Trees: Binary Search Tree

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
enum BST<T> {
    Empty,
    Node {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
}

impl<T: Ord + Clone + std::fmt::Debug> BST<T> {
    fn new() -> Self { BST::Empty }

    fn insert(&self, val: T) -> Self {
        match self {
            BST::Empty => BST::Node {
                value: val,
                left: Box::new(BST::Empty),
                right: Box::new(BST::Empty),
            },
            BST::Node { value, left, right } => {
                if val < *value {
                    BST::Node { value: value.clone(), left: Box::new(left.insert(val)), right: right.clone() }
                } else if val > *value {
                    BST::Node { value: value.clone(), left: left.clone(), right: Box::new(right.insert(val)) }
                } else {
                    self.clone() // duplicate: no change
                }
            }
        }
    }

    fn contains(&self, target: &T) -> bool {
        match self {
            BST::Empty => false,
            BST::Node { value, left, right } => {
                if target == value { true }
                else if target < value { left.contains(target) }
                else { right.contains(target) }
            }
        }
    }

    // In-order traversal → sorted output
    fn to_sorted_vec(&self) -> Vec<T> {
        match self {
            BST::Empty => vec![],
            BST::Node { value, left, right } => {
                let mut result = left.to_sorted_vec();
                result.push(value.clone());
                result.extend(right.to_sorted_vec());
                result
            }
        }
    }

    fn size(&self) -> usize {
        match self {
            BST::Empty => 0,
            BST::Node { left, right, .. } => 1 + left.size() + right.size(),
        }
    }

    fn height(&self) -> usize {
        match self {
            BST::Empty => 0,
            BST::Node { left, right, .. } => 1 + left.height().max(right.height()),
        }
    }

    // Fold for BST
    fn fold<R>(&self, on_empty: R, on_node: &dyn Fn(R, &T, R) -> R) -> R
    where R: Clone {
        match self {
            BST::Empty => on_empty,
            BST::Node { value, left, right } => {
                let l = left.fold(on_empty.clone(), on_node);
                let r = right.fold(on_empty.clone(), on_node);
                on_node(l, value, r)
            }
        }
    }
}

fn main() {
    let tree = BST::new()
        .insert(5)
        .insert(3)
        .insert(7)
        .insert(1)
        .insert(4)
        .insert(6)
        .insert(9);

    println!("Sorted: {:?}", tree.to_sorted_vec());
    println!("Size: {}", tree.size());
    println!("Height: {}", tree.height());
    println!("Contains 4: {}", tree.contains(&4));
    println!("Contains 8: {}", tree.contains(&8));

    // Fold: sum all values
    let sum = tree.fold(0, &|l, val, r| l + val + r);
    println!("Sum: {}", sum);

    // Fold: count leaves
    let leaves = tree.fold(0_usize, &|l, _, r| {
        if l == 0 && r == 0 { 1 } else { l + r }
    });
    println!("Leaves: {}", leaves);
}
```

---

## 32.5 — File System: Recursive Directory Tree

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
enum FSEntry {
    File { name: String, size: u64 },
    Dir { name: String, children: Vec<FSEntry> },
}

impl FSEntry {
    fn file(name: &str, size: u64) -> Self {
        FSEntry::File { name: name.into(), size }
    }
    fn dir(name: &str, children: Vec<FSEntry>) -> Self {
        FSEntry::Dir { name: name.into(), children }
    }

    fn name(&self) -> &str {
        match self { FSEntry::File { name, .. } | FSEntry::Dir { name, .. } => name }
    }

    // Fold for file system
    fn fold<T>(&self, on_file: &dyn Fn(&str, u64) -> T, on_dir: &dyn Fn(&str, Vec<T>) -> T) -> T {
        match self {
            FSEntry::File { name, size } => on_file(name, *size),
            FSEntry::Dir { name, children } => {
                let child_results: Vec<T> = children.iter()
                    .map(|c| c.fold(on_file, on_dir))
                    .collect();
                on_dir(name, child_results)
            }
        }
    }
}

fn main() {
    let project = FSEntry::dir("my_project", vec![
        FSEntry::file("Cargo.toml", 250),
        FSEntry::dir("src", vec![
            FSEntry::file("main.rs", 1200),
            FSEntry::file("lib.rs", 800),
            FSEntry::dir("models", vec![
                FSEntry::file("user.rs", 500),
                FSEntry::file("order.rs", 650),
            ]),
        ]),
        FSEntry::dir("tests", vec![
            FSEntry::file("integration_test.rs", 900),
        ]),
        FSEntry::file("README.md", 400),
    ]);

    // Total size (fold!)
    let total = project.fold(
        &|_, size| size,
        &|_, sizes| sizes.iter().sum(),
    );
    println!("Total size: {} bytes", total);

    // File count
    let files = project.fold(
        &|_, _| 1_usize,
        &|_, counts| counts.iter().sum(),
    );
    println!("Files: {}", files);

    // Directory listing (fold into strings!)
    let listing = project.fold(
        &|name, size| format!("📄 {} ({}B)", name, size),
        &|name, children| {
            let items: String = children.iter()
                .map(|c| format!("  {}", c.replace('\n', "\n  ")))
                .collect::<Vec<_>>()
                .join("\n");
            format!("📁 {}/\n{}", name, items)
        },
    );
    println!("\n{}", listing);

    // Find large files (> 600 bytes)
    let large = project.fold(
        &|name, size| {
            if size > 600 { vec![format!("{} ({}B)", name, size)] } else { vec![] }
        },
        &|_, lists| lists.into_iter().flatten().collect(),
    );
    println!("\nLarge files: {:?}", large);
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Expression evaluation

```rust
let expr = add(mul(lit(2), lit(3)), neg(lit(4)));
```
Kết quả `eval` = ?. Kết quả `display` = ?

<details><summary>✅ Lời giải</summary>

```
eval: (2 * 3) + (-4) = 6 + (-4) = 2
display: "((2 × 3) + (-4))"
```

</details>

---

**Bài 2** (10 phút): Extend Expr

Thêm 2 variants vào `Expr`:
- `Sub(Box<Expr>, Box<Expr>)` — phép trừ
- `If(Box<Expr>, Box<Expr>, Box<Expr>)` — if condition ≠ 0 then a else b

Implement `eval` và `display` cho cả 2.

<details><summary>✅ Lời giải Bài 2</summary>

```rust
enum Expr {
    Lit(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Lit(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Sub(a, b) => eval(a) - eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(a) => -eval(a),
        Expr::If(cond, then, else_) => {
            if eval(cond) != 0 { eval(then) } else { eval(else_) }
        }
    }
}

fn display(expr: &Expr) -> String {
    match expr {
        Expr::Lit(n) => n.to_string(),
        Expr::Add(a, b) => format!("({} + {})", display(a), display(b)),
        Expr::Sub(a, b) => format!("({} - {})", display(a), display(b)),
        Expr::Mul(a, b) => format!("({} × {})", display(a), display(b)),
        Expr::Neg(a) => format!("(-{})", display(a)),
        Expr::If(c, t, e) => format!("(if {} then {} else {})", display(c), display(t), display(e)),
    }
}
```

</details>

---

**Bài 3** (15 phút): HTML DOM mini

Model HTML DOM:
```rust
enum Html {
    Text(String),
    Element { tag: String, attrs: Vec<(String, String)>, children: Vec<Html> },
}
```
Implement:
1. `render(&self) -> String` — output HTML
2. `count_elements` — đếm Element nodes (không đếm Text)
3. `find_by_tag` — tìm tất cả elements có tag nhất định

<details><summary>✅ Lời giải Bài 3</summary>

```rust
enum Html {
    Text(String),
    Element { tag: String, attrs: Vec<(String, String)>, children: Vec<Html> },
}

impl Html {
    fn render(&self) -> String {
        match self {
            Html::Text(t) => t.clone(),
            Html::Element { tag, attrs, children } => {
                let attr_str: String = attrs.iter()
                    .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                    .collect();
                let inner: String = children.iter().map(|c| c.render()).collect();
                format!("<{}{}>{}  </{}>", tag, attr_str, inner, tag)
            }
        }
    }

    fn count_elements(&self) -> usize {
        match self {
            Html::Text(_) => 0,
            Html::Element { children, .. } =>
                1 + children.iter().map(|c| c.count_elements()).sum::<usize>(),
        }
    }

    fn find_by_tag(&self, target: &str) -> Vec<&Html> {
        match self {
            Html::Text(_) => vec![],
            Html::Element { tag, children, .. } => {
                let mut found = vec![];
                if tag == target { found.push(self); }
                for child in children { found.extend(child.find_by_tag(target)); }
                found
            }
        }
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Infinite size" lúc compile | Recursive enum không dùng Box | Wrap recursive field trong `Box<T>` |
| Stack overflow | Tree quá sâu | Dùng iterative approach hoặc increase stack size |
| "Fold phải viết nhiều callbacks" | Generic fold cần 1 fn per variant | Dùng struct `ExprVisitor` thay vì nhiều closures |
| Clone overhead | `Clone` toàn bộ tree | Dùng `Rc<T>` cho shared subtrees |

---

## Tóm tắt

Chapter này dạy bạn làm việc với **búp bê Matryoshka** — data chứa chính nó:

- ✅ **Recursive types**: `Box<T>` = tấm thẻ địa chỉ, giải quyết infinite size.
- ✅ **Pattern matching**: Recursive match = tự nhiên nhất để đọc recursive types.
- ✅ **Fold (Catamorphism)**: 1 function, cho callback mỗi variant, fold traverses + combines. "Mở từng búp bê và làm gì đó với mỗi cái."
- ✅ **BST**: Cây tìm kiếm nhị phân — insert, contains, sorted output, tất cả recursive.
- ✅ **File System**: `File | Dir` — fold để tính total size, đếm files, tạo listing.
- ✅ **Insight**: `Vec::fold` (Ch 13) → `Tree::fold` (Ch 32) → **cùng ý tưởng**, khác hình dạng!

---

## 🎉 Kết thúc Part V — FP Patterns in Rust!

Hãy nhìn lại hành trình qua vùng đất lý thuyết FP:

- **Ch 28**: Trộn màu — Abstract Algebra, Semigroups, Monoids
- **Ch 29**: Hộp quà — Functors, `.map()` biến đổi thứ bên trong
- **Ch 30**: Gỡ giấy gói — Monads, `.and_then()` không lồng
- **Ch 31**: Làm bánh — Parser Combinators, compose từ nhỏ lên lớn
- **Ch 32**: Búp bê Matryoshka — Recursive Types, fold mở từng lớp

Bạn giờ hiểu **TẠI SAO** code FP hoạt động, không chỉ CÁCH viết. Đây là nền tảng vững chắc cho mọi thứ tiếp theo.

## Tiếp theo

→ **Part VI: Testing & Software Engineering** — Chapter 33: **TDD with Rust** — `#[test]`, `assert_eq!`, Red→Green→Refactor cycle.
