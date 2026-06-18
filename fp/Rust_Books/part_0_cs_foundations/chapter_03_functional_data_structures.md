# Chapter 3 — Functional Data Structures

> **Bạn sẽ học được**:
> - Persistent data structures — cập nhật mà không phá dữ liệu cũ
> - Structural sharing — tại sao "copy" trong FP không chậm như bạn tưởng
> - HAMT (Hash Array Mapped Trie) — nền tảng của `im` crate
> - Tại sao `Vec::push` là O(1) nhưng linked list append là O(n)
>
> **Yêu cầu trước**: Chapter 0 (Rust syntax). Chapter 2 (Big-O, recursion).
> **Thời gian đọc**: ~45 phút | **Level**: CS Foundations
> **Kết quả cuối cùng**: Bạn hiểu tradeoff giữa các data structures trong FP, biết khi nào dùng `Vec`, khi nào cần persistent data structures.

---

## 3.1 — Mutable vs Immutable: Bài toán cốt lõi

### Câu chuyện

Bạn và đồng nghiệp cùng chỉnh sửa một file Google Docs. Bạn sửa đoạn 1, đồng nghiệp sửa đoạn 3, cả hai bấm Save cùng lúc. Google Docs xử lý được — vì nó giữ **lịch sử versions**. Bạn luôn có thể quay lại phiên bản trước.

Đây chính là ý tưởng của **persistent data structures**: khi "thay đổi" data, bạn **không xóa bản cũ** mà tạo bản mới. Bản cũ vẫn tồn tại nguyên vẹn.

### Vấn đề với mutable data

```rust
// filename: src/main.rs
fn main() {
    // ❌ Mutable: đổi trực tiếp → bản cũ biến mất
    let mut prices = vec![100, 200, 300];
    println!("Before: {:?}", prices);

    prices[1] = 250;  // Giá cũ 200 biến mất hoàn toàn
    println!("After:  {:?}", prices);

    // Nếu muốn undo → không thể!
    // Nếu thread khác đang đọc prices → race condition!

    // Output:
    // Before: [100, 200, 300]
    // After:  [100, 250, 300]
}
```

### Immutable approach: tạo bản mới

```rust
// filename: src/main.rs
fn main() {
    // ✅ Immutable: tạo bản mới, giữ bản cũ
    let prices_v1 = vec![100, 200, 300];

    // Tạo bản mới với giá thay đổi
    let mut prices_v2 = prices_v1.clone();
    prices_v2[1] = 250;

    // Cả hai versions cùng tồn tại!
    println!("V1: {:?}", prices_v1);  // bản gốc
    println!("V2: {:?}", prices_v2);  // bản mới

    // Undo? Dùng lại v1!
    // Thread-safe? Không ai sửa v1 → an toàn!

    // Output:
    // V1: [100, 200, 300]
    // V2: [100, 250, 300]
}
```

Nhưng `.clone()` copy **toàn bộ** data → O(n). Nếu Vec có 1 triệu phần tử, mỗi lần thay 1 số phải copy 1 triệu phần tử? Quá lãng phí!

Đây là lúc **structural sharing** cứu ngày.

---

## ✅ Checkpoint 3.1

> Ghi nhớ:
> 1. Mutable = sửa trực tiếp → mất bản cũ, nguy hiểm với concurrency
> 2. Immutable + clone = an toàn nhưng chậm (copy toàn bộ O(n))
> 3. Cần giải pháp: giữ immutable nhưng không clone toàn bộ → structural sharing
>
> **Test nhanh**: Nếu Vec có 1 triệu items, `.clone()` tốn bao nhiêu bước?
> <details><summary>Đáp án</summary>O(n) = 1 triệu bước — copy từng phần tử. Quá chậm nếu chỉ thay 1 phần tử.</details>

---

## 3.2 — Structural Sharing: Tái sử dụng thông minh

### Ẩn dụ: Git commits

Git không copy toàn bộ codebase mỗi commit. Nó chỉ lưu **phần thay đổi** (diff), phần không đổi được **chia sẻ** (share) giữa các commits.

Persistent data structures hoạt động y hệt:

```
Bản cũ:  [A, B, C, D, E]
                 ↓ thay C bằng X
Bản mới: [A, B, X, D, E]

Nhưng A, B, D, E KHÔNG được copy!
Cả hai bản CÙNG CHIA SẺ các nodes A, B, D, E.
Chỉ node C → X là mới.
```

### Linked List — Persistent tự nhiên

Linked list là data structure persistent đơn giản nhất:

```rust
// filename: src/main.rs
use std::rc::Rc;

// Persistent linked list dùng Rc (Reference Counting)
#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T: std::fmt::Debug> List<T> {
    fn new() -> Rc<List<T>> {
        Rc::new(List::Nil)
    }

    // Thêm phần tử vào đầu — O(1)!
    // Không copy list cũ, chỉ tạo node mới trỏ đến list cũ
    fn prepend(value: T, tail: &Rc<List<T>>) -> Rc<List<T>> {
        Rc::new(List::Cons(value, Rc::clone(tail)))
    }

    fn to_vec(list: &Rc<List<T>>) -> Vec<&T> {
        let mut result = vec![];
        let mut current = list.as_ref();
        loop {
            match current {
                List::Nil => break,
                List::Cons(val, next) => {
                    result.push(val);
                    current = next.as_ref();
                }
            }
        }
        result
    }
}

fn main() {
    // Tạo list: 3 → 2 → 1
    let list_v1 = List::prepend(1, &List::new());
    let list_v1 = List::prepend(2, &list_v1);
    let list_v1 = List::prepend(3, &list_v1);

    // Tạo bản mới: thêm 99 vào đầu
    // list_v1 KHÔNG bị thay đổi!
    let list_v2 = List::prepend(99, &list_v1);

    // Tạo bản khác: thêm 42 vào đầu list_v1
    let list_v3 = List::prepend(42, &list_v1);

    println!("V1: {:?}", List::to_vec(&list_v1));
    println!("V2: {:?}", List::to_vec(&list_v2));
    println!("V3: {:?}", List::to_vec(&list_v3));

    // Output:
    // V1: [3, 2, 1]
    // V2: [99, 3, 2, 1]
    // V3: [42, 3, 2, 1]

    // V2 và V3 CHIA SẺ phần đuôi [3, 2, 1] với V1!
    // Không copy — chỉ thêm 1 node mới mỗi bản.
}
```

Mô hình trong bộ nhớ:

```
list_v1: 3 → 2 → 1 → Nil
              ↑
list_v2: 99 ─┘  (chia sẻ [3, 2, 1])
              ↑
list_v3: 42 ─┘  (chia sẻ [3, 2, 1])
```

Ba phiên bản cùng tồn tại, chia sẻ data. **Prepend = O(1), không clone.**

### Tradeoff của Linked List

| Thao tác | Linked List | Vec |
|----------|-------------|-----|
| Prepend (thêm đầu) | **O(1)** ✅ | O(n) — phải dịch tất cả |
| Append (thêm cuối) | **O(n)** — phải duyệt đến cuối | **O(1)** amortized ✅ |
| Access by index | **O(n)** — duyệt tuần tự | **O(1)** ✅ |
| Persistent? | ✅ Tự nhiên | ❌ Phải clone O(n) |

> **💡 Khi nào dùng linked list?** Khi bạn cần prepend nhiều + persistence. Khi cần random access hoặc append → dùng `Vec`.

---

## ✅ Checkpoint 3.2

> Ghi nhớ:
> 1. **Structural sharing** = phần không đổi được chia sẻ giữa các versions
> 2. **Linked list prepend** = O(1) persistent — chỉ tạo 1 node mới
> 3. Linked list **KHÔNG** tốt cho: append (O(n)), random access (O(n))
>
> **Test nhanh**: 3 phiên bản linked list chia sẻ đuôi [A, B, C]. Tổng bộ nhớ dùng bao nhiêu nodes (không tính node đầu mỗi version)?
> <details><summary>Đáp án</summary>3 nodes [A, B, C] chia sẻ + 3 node đầu = 6 nodes tổng. Nếu clone đầy đủ: 3 × 4 = 12 nodes. Tiết kiệm 50%!</details>

---

## 3.3 — HAMT: Persistent HashMap

### Vấn đề: HashMap thường không persistent

`HashMap` chuẩn của Rust là mutable. Muốn "phiên bản mới" phải `.clone()` — O(n).

### HAMT là gì?

**HAMT** (Hash Array Mapped Trie) là cấu trúc cây mà mỗi nhánh chứa tối đa 32 phần tử. Khi cập nhật 1 key, chỉ cần **tạo lại nhánh bị ảnh hưởng** — phần còn lại chia sẻ.

Ẩn dụ: Tòa nhà 10 tầng, mỗi tầng 32 phòng. Muốn sửa phòng ở tầng 3? Chỉ cần xây lại **tầng 3** — 9 tầng còn lại giữ nguyên.

```
Tầng 1:  [────────── 32 phòng ──────────]  ← chia sẻ
Tầng 2:  [────────── 32 phòng ──────────]  ← chia sẻ
Tầng 3:  [── sửa 1 phòng ở đây ────────]  ← tạo mới
Tầng 4+: [────────── chia sẻ ───────────]  ← chia sẻ
```

Chi phí: O(log₃₂ n). Với 1 triệu items → chỉ ~4 tầng → update tạo lại ~4 nodes.

### Dùng `im` crate

```rust
// filename: src/main.rs
// Cargo.toml: [dependencies] im = "15"

use im::HashMap;

fn main() {
    // Tạo persistent HashMap
    let prices_v1: HashMap<&str, i32> = HashMap::from(vec![
        ("coffee", 35_000),
        ("tea", 25_000),
        ("smoothie", 45_000),
    ]);

    // Clone = O(1) nhờ structural sharing! Rồi insert bình thường.
    let mut prices_v2 = prices_v1.clone();  // O(1) — không copy data!
    prices_v2.insert("coffee", 40_000);     // copy-on-write nội bộ

    // Thêm item mới
    let mut prices_v3 = prices_v2.clone();  // O(1)
    prices_v3.insert("juice", 30_000);

    println!("V1: {:?}", prices_v1);
    println!("V2: {:?}", prices_v2);
    println!("V3: {:?}", prices_v3);

    // Cả 3 versions cùng tồn tại!
    assert_eq!(prices_v1.get("coffee"), Some(&35_000));  // V1: giá cũ
    assert_eq!(prices_v2.get("coffee"), Some(&40_000));  // V2: giá mới
    assert_eq!(prices_v3.get("juice"), Some(&30_000));   // V3: có juice
    assert_eq!(prices_v1.get("juice"), None);             // V1: chưa có juice

    println!("\nV1 coffee: {}", prices_v1["coffee"]);
    println!("V2 coffee: {}", prices_v2["coffee"]);
    // Output:
    // V1 coffee: 35000
    // V2 coffee: 40000
}
```

```toml
# Cargo.toml
[dependencies]
im = "15"
```

### So sánh: `std::HashMap` vs `im::HashMap`

| | `std::collections::HashMap` | `im::HashMap` (HAMT) |
|---|---|---|
| Insert/Update | O(1) amortized | O(log₃₂ n) ≈ O(1) thực tế |
| Lookup | O(1) | O(log₃₂ n) ≈ O(1) thực tế |
| Clone | **O(n)** — copy toàn bộ | **O(1)** — structural sharing |
| Persistent? | ❌ | ✅ |
| Thread-safe? | Cần lock | ✅ (immutable = tự nhiên safe) |
| Khi nào dùng? | Single-owner, performance critical | Cần history/undo, concurrent, FP |

> **💡 O(log₃₂ n) thực tế**: Với 4 tỷ items (2³²), log₃₂ chỉ = ~6.4. Gần O(1) trong thực tế.

---

## ✅ Checkpoint 3.3

> Ghi nhớ:
> 1. **HAMT** = cây nhánh 32. Update = tạo lại nhánh bị ảnh hưởng, chia sẻ phần còn lại
> 2. `im` crate cung cấp persistent `HashMap`, `Vector`, `OrdMap`
> 3. Clone persistent HashMap = O(1). Clone std HashMap = O(n)
>
> **Test nhanh**: HAMT với 1 triệu items, update 1 key tạo lại ~bao nhiêu nodes?
> <details><summary>Đáp án</summary>~4 nodes (log₃₂(1,000,000) ≈ 4). Phần còn lại chia sẻ.</details>

---

## 3.4 — Amortized Queue: 2 Stacks = 1 Queue

### Bài toán

Queue (hàng đợi): FIFO — vào trước, ra trước. Giống xếp hàng mua cà phê.

Với linked list thuần: `enqueue` (thêm cuối) = O(n). Có cách nào O(1)?

### Ý tưởng của Okasaki

Chris Okasaki (tác giả cuốn "Purely Functional Data Structures") đưa ra trick thông minh: **dùng 2 stacks**.

- **inbox** (stack): items mới push vào đây — O(1)
- **outbox** (stack): lấy items ra từ đây — O(1)
- Khi outbox rỗng: đảo ngược inbox → outbox — O(n) nhưng amortized O(1)

Ẩn dụ: Hai chồng đĩa. Đĩa bẩn chồng lên stack trái. Khi cần rửa, lật ngược cả stack sang phải → đĩa cũ nhất ở trên cùng → rửa từ trên xuống.

```rust
// filename: src/main.rs

#[derive(Debug)]
struct FunctionalQueue<T> {
    inbox: Vec<T>,   // thêm vào đây (push)
    outbox: Vec<T>,  // lấy ra từ đây (pop)
}

impl<T: std::fmt::Debug> FunctionalQueue<T> {
    fn new() -> Self {
        FunctionalQueue { inbox: vec![], outbox: vec![] }
    }

    // Enqueue: thêm vào inbox — O(1)
    fn enqueue(mut self, item: T) -> Self {
        self.inbox.push(item);
        self
    }

    // Dequeue: lấy từ outbox — amortized O(1)
    fn dequeue(mut self) -> (Option<T>, Self) {
        if self.outbox.is_empty() {
            // Đảo ngược inbox → outbox
            // Chỉ xảy ra khi outbox rỗng → amortized O(1)
            while let Some(item) = self.inbox.pop() {
                self.outbox.push(item);
            }
        }
        let item = self.outbox.pop();
        (item, self)
    }

    fn len(&self) -> usize {
        self.inbox.len() + self.outbox.len()
    }
}

fn main() {
    let q = FunctionalQueue::new();

    // Enqueue: A, B, C
    let q = q.enqueue("A");
    let q = q.enqueue("B");
    let q = q.enqueue("C");
    println!("Queue length: {}", q.len());

    // Dequeue — phải ra A trước (FIFO)
    let (item, q) = q.dequeue();
    println!("Dequeue: {:?}", item);   // Some("A")

    let (item, q) = q.dequeue();
    println!("Dequeue: {:?}", item);   // Some("B")

    // Enqueue thêm D
    let q = q.enqueue("D");

    let (item, q) = q.dequeue();
    println!("Dequeue: {:?}", item);   // Some("C")

    let (item, _q) = q.dequeue();
    println!("Dequeue: {:?}", item);   // Some("D")

    // Output:
    // Queue length: 3
    // Dequeue: Some("A")
    // Dequeue: Some("B")
    // Dequeue: Some("C")
    // Dequeue: Some("D")
}
```

> **💡 Tại sao amortized O(1)?** Mỗi item được move từ inbox → outbox **đúng 1 lần**. N items → N moves tổng cộng. Chia đều = O(1) mỗi thao tác.

---

## 3.5 — Graph as ADT

Graph (đồ thị) dùng nhiều trong DDD: quan hệ giữa entities, dependency graph, workflow.

Cách đơn giản nhất: **adjacency list** bằng `HashMap<Node, HashSet<Node>>`:

```rust
// filename: src/main.rs
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashSet<String>>;

fn add_edge(graph: &mut Graph, from: &str, to: &str) {
    graph.entry(from.to_string())
        .or_insert_with(HashSet::new)
        .insert(to.to_string());
}

fn neighbors(graph: &Graph, node: &str) -> Vec<&String> {
    match graph.get(node) {
        Some(edges) => edges.iter().collect(),
        None => vec![],
    }
}

fn has_path(graph: &Graph, from: &str, to: &str) -> bool {
    // DFS đơn giản (dùng stack — Vec::pop lấy từ cuối)
    let mut visited = HashSet::new();
    let mut queue = vec![from.to_string()];

    while let Some(current) = queue.pop() {
        if current == to {
            return true;
        }
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        if let Some(edges) = graph.get(&current) {
            for neighbor in edges {
                if !visited.contains(neighbor) {
                    queue.push(neighbor.clone());
                }
            }
        }
    }
    false
}

fn main() {
    let mut workflow = Graph::new();

    // Workflow: Order → Payment → Prepare → Deliver
    add_edge(&mut workflow, "Order", "Payment");
    add_edge(&mut workflow, "Payment", "Prepare");
    add_edge(&mut workflow, "Prepare", "Deliver");
    add_edge(&mut workflow, "Order", "Cancel");  // đường rẽ

    println!("Order → {:?}", neighbors(&workflow, "Order"));
    println!("Payment → {:?}", neighbors(&workflow, "Payment"));

    println!("\nCan reach Deliver from Order? {}",
        has_path(&workflow, "Order", "Deliver"));
    println!("Can reach Deliver from Cancel? {}",
        has_path(&workflow, "Cancel", "Deliver"));

    // Output:
    // Order → ["Payment", "Cancel"] (thứ tự có thể khác)
    // Payment → ["Prepare"]
    //
    // Can reach Deliver from Order? true
    // Can reach Deliver from Cancel? false
}
```

> **💡 Dùng graph trong DDD**: Ở Part IV, bạn sẽ dùng graph để model bounded context dependencies, aggregate relationships, và event flow.

---

## 3.6 — Big-O trong FP: Bảng tổng hợp

| Data Structure | Prepend | Append | Lookup | Update | Persistent? |
|---------------|---------|--------|--------|--------|-------------|
| `Vec` | O(n) | **O(1)** amort | **O(1)** | **O(1)** | ❌ clone O(n) |
| Linked List (`Rc`) | **O(1)** | O(n) | O(n) | O(n) | ✅ |
| `std::HashMap` | — | **O(1)** | **O(1)** | **O(1)** | ❌ clone O(n) |
| `im::HashMap` (HAMT) | — | O(log₃₂ n) | O(log₃₂ n) | O(log₃₂ n) | ✅ O(1) clone |
| `im::Vector` | O(log₃₂ n) | O(log₃₂ n) | O(log₃₂ n) | O(log₃₂ n) | ✅ O(1) clone |
| Amortized Queue | — | **O(1)** amort | — | — | ✅ |

### Khi nào dùng gì?

- **Mặc định**: `Vec` và `std::HashMap` — nhanh nhất cho single-owner
- **Cần persistence/undo**: `im::HashMap`, `im::Vector`
- **Cần concurrent sharing**: `im::*` (immutable = thread-safe tự nhiên)
- **Cần prepend nhiều**: Linked list hoặc `im::Vector`
- **Modeling domain graph**: `HashMap<K, HashSet<V>>`

---

## 🏋️ Bài tập

**Bài 1** (5 phút): So sánh Big-O

Cho 2 cách update 1 phần tử trong collection 1 triệu items:

```rust
// Cách A: clone Vec rồi sửa
let mut v2 = original.clone();  // O(?)
v2[500_000] = new_value;        // O(?)

// Cách B: dùng im::Vector
let v2 = original.update(500_000, new_value);  // O(?)
```

<details><summary>✅ Lời giải Bài 1</summary>

```
Cách A: clone O(n) + update O(1) = O(n) tổng → 1 triệu bước
Cách B: update O(log₃₂ n) ≈ O(4) → ~4 bước
Cách B nhanh hơn ~250,000 lần!
```

</details>

---

**Bài 2** (10 phút): Implement persistent stack

Viết `PersistentStack` dùng `Rc<List<T>>` với methods: `push`, `pop`, `peek`, `is_empty`.

<details><summary>💡 Gợi ý</summary>Stack = LIFO. Linked list prepend = stack push. Đây là data structure persistent đơn giản nhất!</details>

<details><summary>✅ Lời giải Bài 2</summary>

```rust
// filename: src/main.rs
use std::rc::Rc;

#[derive(Debug)]
enum Stack<T> {
    Empty,
    Node(T, Rc<Stack<T>>),
}

impl<T: Clone + std::fmt::Debug> Stack<T> {
    fn new() -> Rc<Stack<T>> {
        Rc::new(Stack::Empty)
    }

    fn push(value: T, stack: &Rc<Stack<T>>) -> Rc<Stack<T>> {
        Rc::new(Stack::Node(value, Rc::clone(stack)))
    }

    fn pop(stack: &Rc<Stack<T>>) -> (Option<T>, Rc<Stack<T>>) {
        match stack.as_ref() {
            Stack::Empty => (None, Rc::clone(stack)),
            Stack::Node(val, rest) => (Some(val.clone()), Rc::clone(rest)),
        }
    }

    fn peek(stack: &Rc<Stack<T>>) -> Option<&T> {
        match stack.as_ref() {
            Stack::Empty => None,
            Stack::Node(val, _) => Some(val),
        }
    }

    fn is_empty(stack: &Rc<Stack<T>>) -> bool {
        matches!(stack.as_ref(), Stack::Empty)
    }
}

fn main() {
    let s0 = Stack::<i32>::new();
    let s1 = Stack::push(10, &s0);
    let s2 = Stack::push(20, &s1);
    let s3 = Stack::push(30, &s2);

    println!("Top: {:?}", Stack::peek(&s3));  // Some(30)

    let (val, s4) = Stack::pop(&s3);
    println!("Popped: {:?}", val);            // Some(30)
    println!("New top: {:?}", Stack::peek(&s4)); // Some(20)

    // s3 vẫn tồn tại nguyên vẹn!
    println!("s3 top: {:?}", Stack::peek(&s3)); // Some(30)

    // Output:
    // Top: Some(30)
    // Popped: Some(30)
    // New top: Some(20)
    // s3 top: Some(30)
}
```

</details>

---

**Bài 3** (15 phút): Price history tracker

Dùng `im::HashMap` để build hệ thống theo dõi lịch sử giá. Mỗi lần cập nhật giá → tạo version mới. Lưu tất cả versions. Viết function `price_at_version(versions, version_number, product)`.

<details><summary>💡 Gợi ý</summary>Dùng `Vec<im::HashMap>` để lưu history. Mỗi update: clone persistent HashMap (O(1)!) rồi push vào Vec.</details>

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs
// Cargo.toml: [dependencies] im = "15"

use im::HashMap;

struct PriceTracker {
    history: Vec<HashMap<String, u64>>,
}

impl PriceTracker {
    fn new() -> Self {
        PriceTracker {
            history: vec![HashMap::new()],
        }
    }

    fn update_price(&mut self, product: &str, price: u64) {
        // Clone persistent HashMap = O(1)!
        let current = self.history.last().unwrap();
        let new_version = current.update(product.to_string(), price);
        self.history.push(new_version);
    }

    fn price_at_version(&self, version: usize, product: &str) -> Option<&u64> {
        self.history.get(version)?.get(product)
    }

    fn current_version(&self) -> usize {
        self.history.len() - 1
    }
}

fn main() {
    let mut tracker = PriceTracker::new();

    // V0: empty
    tracker.update_price("coffee", 35_000);    // V1
    tracker.update_price("tea", 25_000);       // V2
    tracker.update_price("coffee", 40_000);    // V3 — coffee tăng giá

    println!("Current version: {}", tracker.current_version());

    // Tra cứu giá qua các versions
    println!("Coffee at V1: {:?}", tracker.price_at_version(1, "coffee"));
    println!("Coffee at V3: {:?}", tracker.price_at_version(3, "coffee"));
    println!("Tea at V1: {:?}", tracker.price_at_version(1, "tea"));
    println!("Tea at V2: {:?}", tracker.price_at_version(2, "tea"));

    // Output:
    // Current version: 3
    // Coffee at V1: Some(35000)
    // Coffee at V3: Some(40000)
    // Tea at V1: None
    // Tea at V2: Some(25000)
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `Rc` cannot be sent between threads | `Rc` không thread-safe | Dùng `Arc` cho multi-threaded code |
| `im` crate compile chậm | HAMT code phức tạp | Bình thường — chỉ chậm lần build đầu |
| Persistent DS chậm hơn mutable | O(log₃₂ n) > O(1) | Đúng — trade-off cho persistence. Dùng `Vec`/`HashMap` nếu không cần persistence |
| Memory leak với `Rc` cycles | Circular references | Tránh cycle trong linked structures, hoặc dùng `Weak` |

---

## Tóm tắt

- ✅ **Persistent data structures** = update tạo bản mới, giữ bản cũ. An toàn, undo được, thread-safe.
- ✅ **Structural sharing** = phần không đổi được chia sẻ. Giống Git chỉ lưu diff.
- ✅ **Linked list prepend** = O(1) persistent. **HAMT** = O(log₃₂ n) ≈ O(1) cho HashMap persistent.
- ✅ **Amortized Queue** = 2 stacks → O(1) amortized enqueue/dequeue.
- ✅ **Mặc định dùng `Vec`/`HashMap`**. Chỉ dùng `im::*` khi cần persistence hoặc concurrency.

## Tiếp theo

→ Part I bắt đầu! Chapter 4: **Getting Started with Rust** — bạn sẽ setup Cargo project, hiểu module system, và viết chương trình Rust đầu tiên từ A đến Z.
