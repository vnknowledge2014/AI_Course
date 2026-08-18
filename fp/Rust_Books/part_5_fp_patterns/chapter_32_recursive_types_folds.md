# Chapter 32 — Recursive Types & Folds: Búp bê Nga và Nghệ thuật "Gấp" Dữ liệu

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

Trees, lists, AST (Cây cú pháp trừu tượng), hệ thống tập tin (file systems), HTML DOM — tất cả chúng đều chia sẻ một điểm chung kỳ diệu: chúng là **cấu trúc dữ liệu đệ quy (recursive data structures)**. Nghĩa là, cấu trúc này có thể chứa một phiên bản nhỏ hơn của chính nó.

Trong lập trình Hướng đối tượng (OOP), bạn thường xử lý các cấu trúc đệ quy này bằng **Visitor pattern** kết hợp với dynamic dispatch (đa hình động qua interface). 

Nhưng trong lập trình Hàm (FP), chúng ta sử dụng một công cụ thanh lịch hơn nhiều: **folds (catamorphisms)**. Fold là một hàm đi dạo quanh cấu trúc đệ quy, "gấp" (collapse) từng nhánh nhỏ lại cho đến khi toàn bộ cấu trúc biến thành một giá trị đơn nhất. 

Chương này sẽ giúp bạn nhận ra rằng: `iter().fold()` mà bạn vẫn dùng hàng ngày để tính tổng một danh sách, thực chất chỉ là "bề nổi của tảng băng chìm" cho một khái niệm toán học khổng lồ và mạnh mẽ.

---

## 32.1 — Recursive Types & Box

### Ẩn dụ: Búp bê Matryoshka

Bạn đã bao giờ chơi búp bê Nga (Matryoshka) chưa? Mở con búp bê to nhất ra — bên trong có một con búp bê nhỏ hơn. Mở con búp bê nhỏ hơn — lại có một con búp bê khác. Cứ thế lặp lại cho đến con búp bê cuối cùng — nhỏ nhất, đặc ruột, không thể chứa thêm con nào nữa.

Recursive types (kiểu dữ liệu đệ quy) trong Rust giống hệt như vậy. Chẳng hạn, một biểu thức toán học `Expr::Add` sẽ chứa 2 biểu thức `Expr` bên trong. Mỗi `Expr` con này lại có thể là một `Expr::Add` khác, hoặc là một phép nhân `Expr::Mul`. Con búp bê đặc ruột cuối cùng (base case) chính là những con số tĩnh `Expr::Lit(42)`.

### Vấn đề "Kích thước vô hạn" trong Rust

Hãy thử mô hình hóa búp bê Matryoshka bằng code Rust một cách ngây thơ nhất:

```rust
// ❌ KHÔNG COMPILE ĐƯỢC!
// enum Expr {
//     Lit(i32),
//     Add(Expr, Expr),  // Lỗi: recursive type has infinite size
// }
```

Tại sao Rust lại nổi giận? Rust là một ngôn ngữ quản lý bộ nhớ cực kỳ chặt chẽ. Để phân bổ bộ nhớ trên Stack một cách hiệu quả, trình biên dịch **phải biết trước kích thước chính xác** của mọi biến ngay từ lúc compile.

Nhưng `Expr` chứa `Expr`, con này lại chứa `Expr` khác... Chuỗi này có thể kéo dài vô tận! Do đó, kích thước của kiểu `Expr` bị đánh giá là **infinite (vô hạn)**.

### Giải pháp: Chiếc hộp `Box<T>` 

Làm sao để phá vỡ vòng lặp vô hạn này? Giải pháp là **không chứa trực tiếp** con búp bê bên trong nữa. Thay vì giấu con búp bê nhỏ trong bụng con to, ta mang con nhỏ đem cất lên kho (Heap memory), và chỉ bỏ một **tấm thẻ ghi địa chỉ** vào trong bụng con to.

Tấm thẻ địa chỉ này chính là `Box<T>` (một smart pointer). Dù cái kho (Heap) có chứa thứ to đến đâu, tấm thẻ địa chỉ luôn luôn có kích thước cố định (8 bytes trên hệ thống 64-bit). Vấn đề kích thước vô hạn được giải quyết!

```rust
// filename: src/main.rs

// ═══ Arithmetic expressions ═══
#[derive(Debug, Clone)]
enum Expr {
    Lit(i32),                           // Con búp bê cuối cùng: số 42
    Add(Box<Expr>, Box<Expr>),          // Tấm thẻ trỏ đến a + b trên Heap
    Mul(Box<Expr>, Box<Expr>),          // Tấm thẻ trỏ đến a * b trên Heap
    Neg(Box<Expr>),                     // Tấm thẻ trỏ đến -a
}
```

Để code không bị vướng víu bởi hàng tá chữ `Box::new`, ta viết vài hàm helper (hàm tiện ích) nhỏ gọn:

```rust
// Helper: tạo Box<Expr> gọn hơn
fn lit(n: i32) -> Box<Expr> { Box::new(Expr::Lit(n)) }
fn add(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> { Box::new(Expr::Add(a, b)) }
fn mul(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> { Box::new(Expr::Mul(a, b)) }
fn neg(a: Box<Expr>) -> Box<Expr> { Box::new(Expr::Neg(a)) }

fn main() {
    // Biểu diễn: (3 + 4) * -(5 + 2)
    let expr = mul(
        add(lit(3), lit(4)),
        neg(add(lit(5), lit(2)))
    );
    println!("{:?}", expr);
}
```

> **💡 Mẹo nhớ**: Bất cứ khi nào bạn định nghĩa một cấu trúc dữ liệu A mà bên trong nó chứa tham chiếu trực tiếp đến chính A, bạn bắt buộc phải dùng `Box<A>` (hoặc `Rc`, `Arc`).

---

## 32.2 — Đọc dữ liệu đệ quy bằng Pattern Matching

Chúng ta đã xây xong cấu trúc cây biểu thức đệ quy. Giờ làm sao để đọc nó, đánh giá nó, hoặc in nó ra màn hình? Công cụ tự nhiên nhất để đi đôi với đệ quy chính là... hàm đệ quy kết hợp với `match`.

Đầu tiên, hãy viết hàm tính toán (`eval`) để trả về kết quả cuối cùng:

```rust
// ═══ Evaluate: recursive pattern matching ═══
fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Lit(n) => *n,                       // Base case: trả về con số
        Expr::Add(a, b) => eval(a) + eval(b),     // Tính a, tính b, cộng lại
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(a) => -eval(a),
    }
}
```

Rất gọn gàng! Còn nếu ta muốn in nó ra thành chuỗi để người dùng đọc thì sao? Ta chỉ thay đổi logic tính toán thành logic nối chuỗi:

```rust
// ═══ Pretty-print ═══
fn display(expr: &Expr) -> String {
    match expr {
        Expr::Lit(n) => n.to_string(),
        Expr::Add(a, b) => format!("({} + {})", display(a), display(b)),
        Expr::Mul(a, b) => format!("({} × {})", display(a), display(b)),
        Expr::Neg(a) => format!("(-{})", display(a)),
    }
}
```

Điều gì xảy ra nếu ta muốn đếm xem biểu thức này có bao nhiêu "nút" (node), hoặc độ sâu tối đa của nó là bao nhiêu?

```rust
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
```

Hãy dừng lại và nhìn kỹ 4 hàm chúng ta vừa viết: `eval`, `display`, `count_nodes`, `depth`. 
Bạn có nhận ra điểm bất thường nào không?
Đúng vậy! Cấu trúc (Skeleton) của chúng **giống hệt nhau**. 
- Bước 1: `match expr`. 
- Bước 2: Với base case (`Lit`), làm việc X. 
- Bước 3: Với đệ quy (`Add`, `Mul`, `Neg`), gọi đệ quy vào các nút con, sau đó gom kết quả lại bằng hành động Y.

Chúng ta đang lặp lại logic duyệt cây (traversal logic). Liệu ta có thể tách riêng việc "duyệt" (traverse) và việc "xử lý" (operate) ra không?
Câu trả lời là CÓ. Đó chính là ý nghĩa thực sự của Fold.

---

## 32.3 — Fold: Khuôn mẫu tối thượng (Catamorphism)

### Fold = "Ép cấu trúc đệ quy thành một giá trị duy nhất"

Thuật ngữ toán học cho hành động này là **Catamorphism**. Đừng sợ cái tên này. Catamorphism có nghĩa là "phá vỡ hình dạng". Nó nhận vào một cấu trúc dữ liệu phức tạp (như cây đệ quy) và phá vỡ nó, gấp gọn nó lại thành một giá trị đơn giản (chẳng hạn như 1 con số, hoặc 1 chuỗi).

Để viết một hàm fold tổng quát, chúng ta cần nó nhận vào cách thức xử lý cho **TỪNG** biến thể (variant) của Enum:

```rust
// ═══ Generic fold (catamorphism) ═══
// T là kiểu dữ liệu đầu ra mong muốn (i32, String, usize...)
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
            // Bước 1: Duyệt đệ quy (tự động)
            let va = fold(a, on_lit, on_add, on_mul, on_neg);
            let vb = fold(b, on_lit, on_add, on_mul, on_neg);
            // Bước 2: Áp dụng hàm kết hợp
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
```

Bây giờ, với DUY NHẤT một hàm `fold` này, chúng ta có thể tái tạo lại toàn bộ 4 hàm ở phần trước một cách gọn gàng đến ngỡ ngàng:

```rust
fn main() {
    let expr = mul(add(lit(3), lit(4)), neg(add(lit(5), lit(2))));

    // 1. Tính toán (Eval)
    let eval_result = fold(&expr,
        &|n| n,
        &|a, b| a + b,
        &|a, b| a * b,
        &|a| -a,
    );
    println!("Eval: {}", eval_result);  // -49

    // 2. In chuỗi (Display)
    let display_text = fold(&expr,
        &|n| n.to_string(),
        &|a, b| format!("({} + {})", a, b),
        &|a, b| format!("({} × {})", a, b),
        &|a| format!("(-{})", a),
    );
    println!("Display: {}", display_text);

    // 3. Đếm số nút (Count)
    let node_count = fold(&expr,
        &|_| 1_usize,         // Lá tính là 1 nút
        &|a, b| 1 + a + b,    // Nút cha tính là 1, cộng với tổng nút con
        &|a, b| 1 + a + b,
        &|a| 1 + a,
    );
    println!("Nodes: {}", node_count);
}
```

> **💡 Tại sao Pattern này đỉnh cao?**: "Fold" cho phép chúng ta nói với máy tính: *"Đây là việc anh cần làm khi gặp lá, và đây là việc anh cần làm khi gặp cành. Tôi không quan tâm cấu trúc cây ra sao, anh tự lặn xuống và áp dụng luật này cho tôi!"* Trách nhiệm duyệt cây đã được tách bạch hoàn toàn khỏi Business Logic!

---

## 32.4 — Ứng dụng: Cây Tìm Kiếm Nhị Phân (BST)

Để hiểu sâu hơn, hãy áp dụng tư duy Búp bê Matryoshka này vào một cấu trúc kinh điển: Cây tìm kiếm nhị phân (Binary Search Tree).

Đầu tiên là định nghĩa cây đệ quy:

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
```

Chúng ta sẽ không cần Box nếu cây rỗng (`Empty`). Nhưng nếu nó là `Node`, nó phải giữ địa chỉ trỏ tới cây con trái và phải. Hãy viết vài thao tác cơ bản: thêm (insert) và kiểm tra tồn tại (contains). 

Lưu ý rằng FP ưu tiên **tính bất biến (immutability)**, do đó thay vì sửa đổi (mutate) cây hiện tại, hàm `insert` sẽ tạo ra một cây mới toanh ở những nhánh có sự thay đổi.

```rust
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
                    BST::Node { 
                        value: value.clone(), 
                        left: Box::new(left.insert(val)), // Nhánh phải giữ nguyên, nhánh trái tự rẽ đệ quy
                        right: right.clone() 
                    }
                } else if val > *value {
                    BST::Node { 
                        value: value.clone(), 
                        left: left.clone(), 
                        right: Box::new(right.insert(val)) 
                    }
                } else {
                    self.clone() // Nếu đã tồn tại, không làm gì cả
                }
            }
        }
    }
}
```

Bây giờ là lúc chứng tỏ sức mạnh của Fold. Thay vì viết lẻ tẻ các hàm đếm số nút, tính tổng, thu thập phần tử... ta chỉ viết một hàm `fold` mạnh nhất:

```rust
impl<T: Ord + Clone + std::fmt::Debug> BST<T> {
    // ... code trước đó ...

    // Hàm Fold thần thánh cho cây BST
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
```

Và sử dụng nó trong `main`:

```rust
fn main() {
    let tree = BST::new()
        .insert(5).insert(3).insert(7).insert(1).insert(4).insert(6).insert(9);

    // Tính tổng tất cả giá trị
    let sum = tree.fold(0, &|l, val, r| l + val + r);
    println!("Sum: {}", sum); // 35

    // Đếm số lượng lá (leaf nodes)
    let leaves = tree.fold(0_usize, &|l, _, r| {
        // Nút lá là nút mà hai con trái phải đều rỗng (0)
        if l == 0 && r == 0 { 1 } else { l + r }
    });
    println!("Leaves: {}", leaves); // 4
}
```

---

## 32.5 — Ứng dụng: File System (Hệ thống tập tin)

Cuối cùng, một ví dụ cực kỳ thực tiễn: Thư mục và File. Một thư mục (Dir) có thể chứa nhiều thư mục con và file con. Đây là đệ quy đa phân (nhiều nhánh).

```rust
// filename: src/main.rs

#[derive(Debug, Clone)]
enum FSEntry {
    File { name: String, size: u64 },
    Dir { name: String, children: Vec<FSEntry> },
}
```

Một lần nữa, chúng ta phó mặc toàn bộ quá trình duyệt thư mục cho một hàm `fold`. Cấu trúc của hàm `fold` luôn bám sát theo định nghĩa của Enum:

```rust
impl FSEntry {
    // Tạo data giả cho gọn
    fn file(name: &str, size: u64) -> Self { FSEntry::File { name: name.into(), size } }
    fn dir(name: &str, children: Vec<FSEntry>) -> Self { FSEntry::Dir { name: name.into(), children } }

    // Hàm fold: Nếu là File làm gì, nếu là Dir làm gì?
    fn fold<T>(&self, on_file: &dyn Fn(&str, u64) -> T, on_dir: &dyn Fn(&str, Vec<T>) -> T) -> T {
        match self {
            FSEntry::File { name, size } => on_file(name, *size),
            FSEntry::Dir { name, children } => {
                // Đi sâu vào thư mục con, gom kết quả thành Vec<T>
                let child_results: Vec<T> = children.iter()
                    .map(|c| c.fold(on_file, on_dir))
                    .collect();
                on_dir(name, child_results)
            }
        }
    }
}
```

Hãy tạo một File System ảo và tính toán tổng dung lượng của toàn bộ hệ thống dự án:

```rust
fn main() {
    let project = FSEntry::dir("my_project", vec![
        FSEntry::file("Cargo.toml", 250),
        FSEntry::dir("src", vec![
            FSEntry::file("main.rs", 1200),
            FSEntry::file("lib.rs", 800),
        ]),
        FSEntry::file("README.md", 400),
    ]);

    // Tính tổng kích thước
    let total_size = project.fold(
        &|_, size| size,                     // Nếu là file, trả về kích thước của nó
        &|_, sizes| sizes.iter().sum(),      // Nếu là thư mục, tính tổng kết quả các con
    );
    
    println!("Total size: {} bytes", total_size); // 2650 bytes
}
```

Bạn có thể thêm dễ dàng các logic kinh doanh khác: lọc ra file quá nặng, tạo cây thư mục để in ra console, đếm số lượng file... tất cả chỉ thông qua hàm `fold` được viết đúng 1 lần!

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
    // ... cũ ...
    Sub(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        // ... cũ ...
        Expr::Sub(a, b) => eval(a) - eval(b),
        Expr::If(cond, then, else_) => {
            if eval(cond) != 0 { eval(then) } else { eval(else_) }
        }
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
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| Lỗi "Infinite size" lúc compile | Recursive enum gọi trực tiếp kiểu của mình nhưng không bọc trong Box | Wrap các node con đệ quy trong `Box<T>` |
| Tràn bộ nhớ (Stack overflow) lúc chạy | Cây dữ liệu quá sâu (deep tree), đệ quy bị tràn bộ nhớ stack | Sử dụng vòng lặp (iterative) với mảng `Vec` làm stack nhân tạo, hoặc cân bằng lại cây |
| "Hàm Fold này phải viết quá nhiều closure rắc rối" | Hàm generic fold cần truyền 1 closure cho mỗi variant, khó đọc. | Sử dụng Struct `ExprVisitor` và định nghĩa Trait (Visitor pattern của Rust) thay vì dùng closure tự do. |
| Overhead vì Clone nhiều | Gọi `.clone()` toàn bộ cây gây tốn RAM | Thay `Box<T>` bằng `Rc<T>` hoặc `Arc<T>` để chia sẻ nhánh cây (shared subtrees) mà không copy. |

---

---

## ✅ Checkpoint 32

1. Vì sao `enum Expr { Add(Expr, Expr) }` không biên dịch được, mà `Add(Box<Expr>, Box<Expr>)` thì được?
2. Catamorphism (fold) tách bạch hai thứ gì?
3. Fold đệ quy trên cây rất sâu có rủi ro gì trong Rust?

<details>
<summary>Đáp án</summary>

1. Vì compiler phải tính kích thước của mỗi kiểu lúc biên dịch. `Expr` chứa `Expr` cho ra kích thước vô hạn. `Box` là con trỏ có kích thước cố định, cắt đứt chuỗi đệ quy đó.
2. Tách **cách duyệt** cấu trúc khỏi **việc cần tính**. Viết fold một lần, rồi mọi phép tính mới chỉ là truyền vào một hàm khác — không phải viết lại đệ quy.
3. **Stack overflow**. Rust không có tối ưu đệ quy đuôi được bảo đảm, và fold trên cây thì vốn không phải tail-recursive. Với cây có thể rất sâu, hãy chuyển sang duyệt tường minh bằng một `Vec` làm stack.
</details>

## Tóm tắt

Chapter này dạy bạn cách giải phẫu **búp bê Matryoshka** — thứ mà dữ liệu được nhét vào trong chính nó:

- ✅ **Recursive types**: Để vượt qua rào cản Infinite Size của trình biên dịch, hãy dùng tấm thẻ địa chỉ `Box<T>`.
- ✅ **Pattern matching**: Lối thoát tự nhiên nhất khỏi vòng đệ quy chính là dùng recursive match.
- ✅ **Fold (Catamorphism)**: 1 hàm duy nhất nhận nhiệm vụ thám hiểm rừng cây. Bạn chỉ việc giao cho nó tấm bản đồ (closures mô tả việc cần làm ở mỗi kiểu nút) — fold duyệt và nối kết quả hộ bạn.
- ✅ **Thực tiễn**: Mọi cấu trúc như BST (cây nhị phân) hay File System (cây đa phân) đều được xử lý thanh lịch bởi Fold.
- ✅ **Insight (Ngộ ra)**: `Vec::fold` (Chapter 13) và `Tree::fold` (Chapter 32) **hoàn toàn là một**! Chúng có chung linh hồn, chỉ khác cái vỏ bọc mà thôi.

---

## 🎉 Kết thúc Part V — FP Patterns in Rust!

Thật đáng tự hào! Hãy nhìn lại hành trình phi thường qua vùng đất "khó nhai" nhất của lập trình Hàm:

- **Chapter 28**: Trộn màu — Nhận thức về Semigroups, Monoids (Abstract Algebra).
- **Chapter 29**: Hộp quà bí ẩn — Functors, và cách `.map()` can thiệp thế giới bên trong.
- **Chapter 30**: Nghệ thuật gỡ rối — Monads, và cách `.and_then()` làm phẳng sự hỗn loạn.
- **Chapter 31**: Xếp Lego — Parser Combinators, lắp những bộ não khổng lồ từ các hạt bụi.
- **Chapter 32**: Búp bê Nga — Recursive Types, và vũ khí tối thượng Fold.

Giờ đây bạn không chỉ biết CÁCH viết code FP (vốn đã học ở Part I, II), bạn còn hiểu sâu thẳm **TẠI SAO** các thư viện lại được thiết kế như vậy. Bạn đang nhìn code bằng "Matrix vision".

## Tiếp theo

Đã đến lúc trở lại thế giới thực và chiến đấu trong môi trường sản xuất! 
→ **Part VI: Testing & Software Engineering** — Chapter 33: **TDD with Rust** — Học cách tạo nhịp điệu Red→Green→Refactor hoàn hảo.
