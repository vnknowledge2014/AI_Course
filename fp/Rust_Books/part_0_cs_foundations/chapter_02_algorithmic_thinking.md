# Chapter 2 — Algorithmic Thinking & Complexity

> **Bạn sẽ học được**:
> - Big-O là gì — cách đo "nhanh/chậm" của thuật toán mà không cần đo thời gian
> - Recursion — function gọi chính nó, và tại sao FP yêu recursion hơn vòng lặp
> - Các pattern thuật toán phổ biến: divide & conquer, binary search
> - Sorting và Searching — hai bài toán cơ bản nhất trong CS
>
> **Yêu cầu trước**: Chapter 0 (Rust in 10 Minutes). Chapter 1 (Math Foundations) là lợi thế nhưng không bắt buộc.
> **Thời gian đọc**: ~45 phút | **Level**: CS Foundations
> **Kết quả cuối cùng**: Bạn nhìn vào một đoạn code và đánh giá được "code này nhanh hay chậm" — rồi chọn thuật toán phù hợp.

---

## 2.1 — Big-O: Đo tốc độ bằng "hình dạng", không bằng giây

### Câu chuyện

Bạn có 10 cuốn sách trên kệ. Muốn tìm cuốn "Rust Programming", có 2 cách:

- **Cách 1**: Lật từng cuốn từ trái sang phải. Nếu kệ có 10 cuốn → tối đa 10 lần. Nếu có 1000 cuốn → tối đa 1000 lần.
- **Cách 2**: Sách đã xếp theo ABC. Mở giữa kệ, so sánh, rồi bỏ nửa không cần. 10 cuốn → tối đa 4 lần. 1000 cuốn → tối đa 10 lần.

Cách 1 gấp 100 lần khi data gấp 100 lần. Cách 2 chỉ gấp ~2.5 lần. Hai cách "lớn lên" khác nhau — và Big-O là cách nói về sự khác nhau đó.

### Big-O là gì?

Big-O mô tả **tốc độ tăng trưởng** của thuật toán khi data lớn lên. Không quan tâm máy nhanh hay chậm, chỉ quan tâm **hình dạng**:

| Big-O | Tên | Ẩn dụ | Ví dụ |
|-------|-----|-------|-------|
| O(1) | Constant | Bật công tắc đèn — luôn 1 bước | Truy cập `vec[5]` |
| O(log n) | Logarithmic | Tìm trong từ điển (chia đôi) | Binary search |
| O(n) | Linear | Đếm từng người trong hàng | Duyệt mảng |
| O(n log n) | Linearithmic | Sắp xếp bài hiệu quả | Merge sort |
| O(n²) | Quadratic | So sánh mọi cặp | 2 vòng lặp lồng nhau |

### Ví dụ trực quan

```rust
// filename: src/main.rs

// O(1) — Constant: luôn 1 bước, bất kể data lớn cỡ nào
fn get_first(items: &[i32]) -> Option<&i32> {
    items.first()  // Chỉ lấy phần tử đầu — không cần duyệt
}

// O(n) — Linear: số bước tỉ lệ với data
fn find_max(items: &[i32]) -> Option<i32> {
    if items.is_empty() {
        return None;
    }
    let mut max = items[0];
    for &item in &items[1..] {  // Duyệt từng phần tử — n bước
        if item > max {
            max = item;
        }
    }
    Some(max)
}

// O(n²) — Quadratic: mỗi phần tử so với mọi phần tử khác
fn has_duplicates(items: &[i32]) -> bool {
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {  // Vòng lặp lồng → n × n
            if items[i] == items[j] {
                return true;
            }
        }
    }
    false
}

fn main() {
    let data = vec![3, 7, 1, 9, 4, 7];

    println!("First: {:?}", get_first(&data));         // O(1)
    println!("Max: {:?}", find_max(&data));             // O(n)
    println!("Has duplicates: {}", has_duplicates(&data)); // O(n²)

    // Output:
    // First: Some(3)
    // Max: Some(9)
    // Has duplicates: true
}
```

### Amortized Analysis: `Vec::push` vẫn là O(1)

Một câu hỏi thường gặp: `Vec::push` phải resize khi đầy — sao lại O(1)?

Ẩn dụ: Bạn bỏ tiền xu vào heo đất. 99 lần chỉ cần thả xu vào — nhanh. Lần thứ 100, heo đầy, bạn phải đổi sang con to hơn và chuyển hết xu qua — chậm. Nhưng **trung bình** mỗi lần vẫn rất nhanh.

```rust
// filename: src/main.rs
fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    // Vec bắt đầu với capacity = 0
    println!("Start: len={}, capacity={}", numbers.len(), numbers.capacity());

    for i in 0..20 {
        let old_cap = numbers.capacity();
        numbers.push(i);
        let new_cap = numbers.capacity();

        // Chỉ in khi capacity thay đổi (= resize xảy ra)
        if new_cap != old_cap {
            println!("  Push {}: capacity {} → {} (RESIZE!)", i, old_cap, new_cap);
        }
    }

    println!("Final: len={}, capacity={}", numbers.len(), numbers.capacity());

    // Output (ví dụ):
    // Start: len=0, capacity=0
    //   Push 0: capacity 0 → 4 (RESIZE!)
    //   Push 4: capacity 4 → 8 (RESIZE!)
    //   Push 8: capacity 8 → 16 (RESIZE!)
    //   Push 16: capacity 16 → 32 (RESIZE!)
    // Final: len=20, capacity=32
}
```

Resize chỉ xảy ra ~log(n) lần. Phân bổ chi phí cho tất cả push → **amortized O(1)** mỗi lần push.

---

## ✅ Checkpoint 2.1

> Ghi nhớ:
> 1. Big-O = tốc độ tăng trưởng, không phải tốc độ tuyệt đối
> 2. O(1) < O(log n) < O(n) < O(n log n) < O(n²) — từ nhanh đến chậm
> 3. `Vec::push` là O(1) amortized — resize hiếm khi xảy ra
>
> **Test nhanh**: Function duyệt mảng 1 lần để tính tổng — Big-O là gì?
> <details><summary>Đáp án</summary>O(n) — mỗi phần tử được visit đúng 1 lần, tổng số bước tỉ lệ với n.</details>

---

## 2.2 — Recursion: Function gọi chính nó

### Câu chuyện: Búp bê Matryoshka

Búp bê Nga Matryoshka: mở con lớn ra → bên trong có con nhỏ hơn → mở tiếp → con nhỏ hơn nữa → ... → cuối cùng đến con bé nhất (không mở được nữa).

**Recursion hoạt động y hệt**: function gọi chính nó với input nhỏ hơn, cho đến khi gặp **base case** (con búp bê bé nhất).

### Ví dụ kinh điển: Factorial

`5! = 5 × 4 × 3 × 2 × 1 = 120`

Hay viết cách khác: `5! = 5 × 4!` → `4! = 4 × 3!` → ... → `1! = 1` (base case)

```rust
// filename: src/main.rs

// Cách 1: Recursion
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1           // Base case: con búp bê bé nhất
    } else {
        n * factorial(n - 1)  // Mở con búp bê tiếp theo
    }
}

// Cách 2: Iteration (vòng lặp)
fn factorial_loop(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

fn main() {
    // Cả hai cho cùng kết quả
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial_loop(5), 120);

    for n in 0..=10 {
        println!("{}! = {}", n, factorial(n));
    }
    // Output:
    // 0! = 1
    // 1! = 1
    // 2! = 2
    // 3! = 6
    // 4! = 24
    // 5! = 120
    // 6! = 720
    // 7! = 5040
    // 8! = 40320
    // 9! = 362880
    // 10! = 3628800
}
```

### Stack Frames — Tại sao recursion tốn bộ nhớ

Mỗi lần function gọi chính nó, máy tính phải **nhớ** nơi để quay lại — giống xếp đĩa chồng lên nhau:

```
factorial(5)
  → 5 * factorial(4)          ← đĩa 1
       → 4 * factorial(3)     ← đĩa 2
            → 3 * factorial(2) ← đĩa 3
                 → 2 * factorial(1) ← đĩa 4
                      → 1       ← base case, bắt đầu gỡ đĩa
                 ← 2 * 1 = 2
            ← 3 * 2 = 6
       ← 4 * 6 = 24
  ← 5 * 24 = 120
```

Nếu n quá lớn (ví dụ 1 triệu) → quá nhiều đĩa → **stack overflow** 💥

### Tail Recursion — Giải pháp

**Tail recursion** = recursive call là việc **cuối cùng** function làm. Compiler *có thể* tối ưu thành vòng lặp (không tốn thêm stack frames).

```rust
// filename: src/main.rs

// ❌ Recursive thường: phải nhớ n * kết_quả_sau
fn factorial_normal(n: u64) -> u64 {
    if n <= 1 { 1 }
    else { n * factorial_normal(n - 1) }
    //     ↑ phải chờ kết quả rồi mới nhân → KHÔNG phải tail
}

// ✅ Tail recursive: mang theo accumulator
fn factorial_tail(n: u64, acc: u64) -> u64 {
    if n <= 1 { acc }
    else { factorial_tail(n - 1, n * acc) }
    //     ↑ recursive call là việc CUỐI CÙNG → IS tail recursive
}

fn main() {
    // Cả hai cho cùng kết quả
    assert_eq!(factorial_normal(10), 3628800);
    assert_eq!(factorial_tail(10, 1), 3628800);  // acc bắt đầu = 1

    println!("10! = {}", factorial_tail(10, 1));
    // Output: 10! = 3628800
}
```

Sự khác biệt:
- `factorial_normal(5)`: phải nhớ `5 * (4 * (3 * (2 * 1)))` → xếp 5 đĩa
- `factorial_tail(5, 1)` → `factorial_tail(4, 5)` → `factorial_tail(3, 20)` → ... → chỉ 1 đĩa tại mọi thời điểm

> **⚠️ Lưu ý**: Rust compiler hiện tại **không đảm bảo** tối ưu tail call (khác với Haskell, Roc, Scheme). Tuy nhiên, viết theo kiểu tail recursive vẫn là **thói quen tốt** — và dễ chuyển sang iteration.

### FP yêu Recursion vì sao?

Trong FP, ta tránh **mutable state** (`mut`). Vòng lặp `for` cần `mut` để cập nhật biến. Recursion thì không:

```rust
// filename: src/main.rs

// Tính tổng: cách FP (không cần mut)
fn sum_recursive(items: &[i32]) -> i32 {
    match items {
        [] => 0,                              // mảng rỗng → tổng = 0
        [first, rest @ ..] => first + sum_recursive(rest), // head + sum(tail)
    }
}

// Tính tổng: cách idiomatic Rust (dùng iterator — cũng không cần mut!)
fn sum_iter(items: &[i32]) -> i32 {
    items.iter().sum()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    assert_eq!(sum_recursive(&data), 15);
    assert_eq!(sum_iter(&data), 15);

    println!("Sum (recursive): {}", sum_recursive(&data));
    println!("Sum (iterator): {}", sum_iter(&data));
    // Output:
    // Sum (recursive): 15
    // Sum (iterator): 15
}
```

> **💡 Trong Rust thực tế**: Bạn sẽ dùng **iterators** (`.map()`, `.filter()`, `.fold()`) nhiều hơn recursion thuần. Iterators là cách Rust kết hợp hiệu suất của vòng lặp với tính thanh lịch của FP. Sẽ học kỹ ở Chapter 12.

---

## ✅ Checkpoint 2.2

> Ghi nhớ:
> 1. Recursion = function gọi chính nó + base case (điểm dừng)
> 2. Mỗi recursive call tạo stack frame → quá nhiều = stack overflow
> 3. Tail recursion = recursive call là việc cuối cùng → tiết kiệm stack
> 4. Rust thực tế dùng iterators nhiều hơn recursion thuần
>
> **Test nhanh**: Viết `countdown(n)` in từ n xuống 1, dùng recursion. Base case là gì?
> <details><summary>Đáp án</summary>Base case: n <= 0 thì dừng (return). Recursive case: in n, rồi gọi countdown(n - 1).</details>

---

## 2.3 — Divide & Conquer và Binary Search

### Divide & Conquer — Chia để trị

Ẩn dụ: Bạn cần dọn nhà 100m². Thay vì dọn cả nhà một lần (choáng ngợp), bạn:
1. **Chia** nhà thành 4 phòng (mỗi phòng 25m²)
2. **Dọn** từng phòng (bài toán nhỏ hơn, dễ hơn)
3. **Gộp**: xong cả 4 phòng = xong cả nhà

Đây chính là **Divide & Conquer** — pattern thuật toán mạnh nhất:
1. **Divide**: Chia bài toán lớn thành bài toán nhỏ
2. **Conquer**: Giải từng bài nhỏ (thường bằng recursion)
3. **Combine**: Gộp kết quả lại

### Binary Search — O(log n)

Binary search là ví dụ kinh điển của divide & conquer. Yêu cầu: **mảng đã sắp xếp**.

Ẩn dụ: Tìm từ "Rust" trong từ điển:
1. Mở giữa → thấy "M". "R" > "M" → bỏ nửa trái
2. Mở giữa nửa phải → thấy "S". "R" < "S" → bỏ nửa phải
3. Mở giữa phần còn lại → thấy "R" → Tìm thấy!

Mỗi bước loại bỏ **một nửa** data → O(log n).

```rust
// filename: src/main.rs

fn binary_search(sorted: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = sorted.len();

    while low < high {
        let mid = low + (high - low) / 2;  // tránh overflow

        if sorted[mid] == target {
            return Some(mid);               // Tìm thấy!
        } else if sorted[mid] < target {
            low = mid + 1;                  // Bỏ nửa trái
        } else {
            high = mid;                     // Bỏ nửa phải
        }
    }

    None  // Không tìm thấy
}

// Phiên bản recursive (để thấy rõ divide & conquer)
fn binary_search_recursive(sorted: &[i32], target: i32, low: usize, high: usize) -> Option<usize> {
    if low >= high {
        return None;  // Base case: không còn gì để tìm
    }

    let mid = low + (high - low) / 2;

    if sorted[mid] == target {
        Some(mid)
    } else if sorted[mid] < target {
        binary_search_recursive(sorted, target, mid + 1, high) // Tìm nửa phải
    } else {
        binary_search_recursive(sorted, target, low, mid) // Tìm nửa trái
    }
}

fn main() {
    let data = vec![2, 5, 8, 12, 16, 23, 38, 56, 72, 91];

    // Tìm thấy
    println!("Find 23: {:?}", binary_search(&data, 23));
    println!("Find 23 (recursive): {:?}",
        binary_search_recursive(&data, 23, 0, data.len()));

    // Không tìm thấy
    println!("Find 42: {:?}", binary_search(&data, 42));

    // So sánh: 10 phần tử
    // Linear search: tối đa 10 bước
    // Binary search: tối đa log2(10) ≈ 4 bước
    println!("\n10 items: linear max 10 steps, binary max {} steps",
        (10_f64).log2().ceil() as u32);

    // 1 triệu phần tử
    // Linear: tối đa 1,000,000 bước
    // Binary: tối đa 20 bước!
    println!("1M items: linear max 1000000 steps, binary max {} steps",
        (1_000_000_f64).log2().ceil() as u32);

    // Output:
    // Find 23: Some(5)
    // Find 23 (recursive): Some(5)
    // Find 42: None
    //
    // 10 items: linear max 10 steps, binary max 4 steps
    // 1M items: linear max 1000000 steps, binary max 20 steps
}
```

1 triệu phần tử: linear search = 1,000,000 bước. Binary search = **20 bước**. Đó là sức mạnh của O(log n).

> **💡 Rust stdlib**: Rust có sẵn `slice.binary_search(&target)` trả `Result<usize, usize>`. Trong production, dùng method của stdlib thay vì tự viết.

---

## ✅ Checkpoint 2.3

> Ghi nhớ:
> 1. **Divide & Conquer**: Chia → Giải từng phần → Gộp
> 2. **Binary search** = O(log n) — mỗi bước loại nửa data. Yêu cầu: data đã sorted
> 3. 1 triệu items: linear = 1M bước, binary = 20 bước
>
> **Test nhanh**: Binary search trên mảng 1024 phần tử cần tối đa bao nhiêu bước?
> <details><summary>Đáp án</summary>log₂(1024) = 10 bước. Mỗi bước chia đôi: 1024 → 512 → 256 → ... → 1.</details>

---

## 2.4 — Sorting: Merge Sort vs Quick Sort

### Merge Sort — O(n log n), stable, FP-friendly

Merge sort là ví dụ hoàn hảo của divide & conquer:
1. **Chia** mảng thành 2 nửa
2. **Sort** từng nửa (recursion)
3. **Merge** 2 nửa đã sort thành 1 mảng sorted

```rust
// filename: src/main.rs

fn merge_sort(items: &[i32]) -> Vec<i32> {
    // Base case: mảng 0 hoặc 1 phần tử đã sorted
    if items.len() <= 1 {
        return items.to_vec();
    }

    // Divide: chia đôi
    let mid = items.len() / 2;
    let left = merge_sort(&items[..mid]);   // sort nửa trái
    let right = merge_sort(&items[mid..]);  // sort nửa phải

    // Combine: merge 2 nửa đã sorted
    merge(&left, &right)
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;

    // So sánh từng cặp, lấy cái nhỏ hơn
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    // Phần còn lại
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

fn main() {
    let data = vec![38, 27, 43, 3, 9, 82, 10];

    println!("Before: {:?}", data);
    let sorted = merge_sort(&data);
    println!("After:  {:?}", sorted);

    assert_eq!(sorted, vec![3, 9, 10, 27, 38, 43, 82]);

    // Output:
    // Before: [38, 27, 43, 3, 9, 82, 10]
    // After:  [3, 9, 10, 27, 38, 43, 82]
}
```

Tại sao merge sort "FP-friendly"?
- **Không mutate** data gốc — tạo Vec mới
- **Pure function**: cùng input → cùng output
- **Divide & Conquer** tự nhiên với recursion

### So sánh sorting algorithms

| Thuật toán | Best | Average | Worst | Stable? | FP-friendly? |
|-----------|------|---------|-------|---------|---------------|
| Merge Sort | O(n log n) | O(n log n) | O(n log n) | ✅ | ✅ |
| Quick Sort | O(n log n) | O(n log n) | O(n²) | ❌ | ❌ (in-place) |
| Bubble Sort | O(n) | O(n²) | O(n²) | ✅ | ❌ |

> **💡 Stable sort** = giữ nguyên thứ tự ban đầu khi 2 phần tử bằng nhau. Ví dụ: sort học sinh theo điểm — stable sort giữ nguyên thứ tự ABC với cùng điểm.

> **💡 Rust stdlib**: `slice.sort()` dùng thuật toán hybrid (tương tự Timsort) — stable, O(n log n). `slice.sort_unstable()` nhanh hơn một chút nhưng không stable.

---

## 2.5 — Tổng hợp: Chọn thuật toán đúng

### Bảng quyết định nhanh

| Bài toán | Thuật toán | Big-O | Ghi chú |
|----------|-----------|-------|---------|
| Tìm phần tử trong mảng sorted | Binary search | O(log n) | Mảng phải sorted |
| Tìm phần tử trong mảng unsorted | Linear scan | O(n) | Hoặc dùng HashSet O(1) |
| Sort mảng | `.sort()` (stdlib) | O(n log n) | Dùng stdlib, đừng tự viết |
| Tìm max/min | Linear scan | O(n) | `.iter().max()` |
| Kiểm tra có duplicate | HashSet | O(n) | Tốt hơn O(n²) 2 vòng lặp |
| Tìm cặp tổng = target | Two pointers (sorted) | O(n) | Hoặc HashMap O(n) |

### Two Pointers — Pattern hay trong mảng sorted

```rust
// filename: src/main.rs
use std::collections::HashSet;

// O(n²) — cách "ngây thơ": 2 vòng lặp
fn has_pair_sum_naive(items: &[i32], target: i32) -> bool {
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            if items[i] + items[j] == target {
                return true;
            }
        }
    }
    false
}

// O(n) — cách thông minh: HashSet
fn has_pair_sum_hash(items: &[i32], target: i32) -> bool {
    let mut seen = HashSet::new();
    for &item in items {
        let complement = target - item;
        if seen.contains(&complement) {
            return true;
        }
        seen.insert(item);
    }
    false
}

// O(n) — Two pointers: yêu cầu mảng đã sorted
fn has_pair_sum_two_pointers(sorted: &[i32], target: i32) -> bool {
    if sorted.len() < 2 {
        return false;
    }
    let mut left = 0;
    let mut right = sorted.len() - 1;

    while left < right {
        let sum = sorted[left] + sorted[right];
        if sum == target {
            return true;
        } else if sum < target {
            left += 1;      // Tổng nhỏ quá → dịch con trỏ trái sang phải
        } else {
            right -= 1;     // Tổng lớn quá → dịch con trỏ phải sang trái
        }
    }
    false
}

fn main() {
    let data = vec![2, 5, 8, 12, 16, 23, 38];

    // Tìm cặp có tổng = 28: 5 + 23 = 28 ✓
    println!("Pair sum 28 (naive): {}", has_pair_sum_naive(&data, 28));
    println!("Pair sum 28 (hash): {}", has_pair_sum_hash(&data, 28));
    println!("Pair sum 28 (two pointers): {}", has_pair_sum_two_pointers(&data, 28));

    // Tìm cặp có tổng = 20: 8 + 12 = 20 ✓
    println!("Pair sum 20: {}", has_pair_sum_two_pointers(&data, 20));

    // Output:
    // Pair sum 28 (naive): true
    // Pair sum 28 (hash): true
    // Pair sum 28 (two pointers): true
    // Pair sum 20: true
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Đánh giá Big-O

Cho Big-O của mỗi đoạn code:

```rust
// a) 
fn sum(items: &[i32]) -> i32 {
    items.iter().sum()
}

// b)
fn print_pairs(items: &[i32]) {
    for i in items {
        for j in items {
            println!("{} {}", i, j);
        }
    }
}

// c)
fn binary_search_stdlib(sorted: &[i32], target: &i32) -> bool {
    sorted.binary_search(target).is_ok()
}
```

<details><summary>✅ Lời giải Bài 1</summary>

```
a) O(n) — duyệt mảng 1 lần
b) O(n²) — 2 vòng lặp lồng nhau
c) O(log n) — binary search chia đôi mỗi bước
```

</details>

---

**Bài 2** (10 phút): Viết `fibonacci` recursive và tail recursive

```rust
// fibonacci: 0, 1, 1, 2, 3, 5, 8, 13, 21, ...
// fib(0) = 0, fib(1) = 1
// fib(n) = fib(n-1) + fib(n-2)
```

<details><summary>💡 Gợi ý</summary>Tail recursive version cần 2 accumulators: a (fib hiện tại) và b (fib kế tiếp).</details>

<details><summary>✅ Lời giải Bài 2</summary>

```rust
// filename: src/main.rs

// ❌ Recursive thường — O(2^n): RẤT CHẬM
fn fib(n: u64) -> u64 {
    if n <= 1 { n }
    else { fib(n - 1) + fib(n - 2) }
}

// ✅ Tail recursive — O(n): nhanh
fn fib_tail(n: u64, a: u64, b: u64) -> u64 {
    if n == 0 { a }
    else { fib_tail(n - 1, b, a + b) }
}

fn main() {
    for i in 0..=10 {
        assert_eq!(fib(i), fib_tail(i, 0, 1));
        println!("fib({}) = {}", i, fib_tail(i, 0, 1));
    }
    // Output:
    // fib(0) = 0
    // fib(1) = 1
    // fib(2) = 1
    // ...
    // fib(10) = 55
}
```

Phiên bản thường O(2ⁿ) — `fib(40)` đã rất chậm. Tail version O(n) — `fib(1000)` vẫn tức thì (nếu dùng big number).

</details>

---

**Bài 3** (15 phút): Viết merge sort cho `Vec<String>` sắp xếp theo ABC

<details><summary>💡 Gợi ý</summary>Thay `i32` bằng `String`, so sánh bằng `<=` hoặc `.cmp()`. Clone strings khi cần.</details>

<details><summary>✅ Lời giải Bài 3</summary>

```rust
// filename: src/main.rs

fn merge_sort_strings(items: &[String]) -> Vec<String> {
    if items.len() <= 1 {
        return items.to_vec();
    }
    let mid = items.len() / 2;
    let left = merge_sort_strings(&items[..mid]);
    let right = merge_sort_strings(&items[mid..]);
    merge_strings(&left, &right)
}

fn merge_strings(left: &[String], right: &[String]) -> Vec<String> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    for item in &left[i..] { result.push(item.clone()); }
    for item in &right[j..] { result.push(item.clone()); }
    result
}

fn main() {
    let names: Vec<String> = vec!["Rust", "Go", "Zig", "Roc", "Elm"]
        .into_iter().map(String::from).collect();

    println!("Before: {:?}", names);
    let sorted = merge_sort_strings(&names);
    println!("After:  {:?}", sorted);
    // Output:
    // Before: ["Rust", "Go", "Zig", "Roc", "Elm"]
    // After:  ["Elm", "Go", "Roc", "Rust", "Zig"]
}
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| Stack overflow | Recursion quá sâu / thiếu base case | Thêm/kiểm tra base case. Dùng iteration hoặc tail recursion |
| Binary search sai | Mảng chưa sorted | Sort trước khi search |
| Off-by-one trong binary search | `mid + 1` vs `mid` | Dùng `low + (high - low) / 2` tránh overflow. Test với mảng 0, 1, 2 phần tử |
| `merge_sort` chậm hơn mong đợi | Clone quá nhiều | Trong production, dùng `slice.sort()` của stdlib |

---

## Tóm tắt

- ✅ **Big-O** = đo "hình dạng tăng trưởng". O(1) < O(log n) < O(n) < O(n log n) < O(n²).
- ✅ **Recursion** = function gọi chính nó + base case. **Tail recursion** = recursive call cuối cùng → tiết kiệm stack. Rust thực tế dùng **iterators**.
- ✅ **Divide & Conquer** = Chia → Giải → Gộp. **Binary search** O(log n) là ví dụ kinh điển.
- ✅ **Merge sort** = O(n log n), stable, FP-friendly. Trong production, dùng `.sort()` của stdlib.
- ✅ **Chọn thuật toán đúng** quan trọng hơn viết code nhanh. O(n) vs O(n²) = khác biệt giữa 1 giây và 11 ngày trên 1 triệu items.

## Tiếp theo

→ Chapter 3: **Functional Data Structures** — bạn sẽ học persistent data structures (cập nhật mà không phá dữ liệu cũ), structural sharing, HAMT, và tại sao `Vec::push` O(1) nhưng `list.append` O(n).
