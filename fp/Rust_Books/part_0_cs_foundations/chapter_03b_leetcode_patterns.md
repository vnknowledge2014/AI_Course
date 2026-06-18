# Chapter 3B — Advanced Algorithms & LeetCode Patterns

> **Bạn sẽ học được**:
> - Cách giải các bài toán LeetCode bằng tư duy Functional
> - Two Pointers & Sliding Window với `Iterator` và `slice`
> - Dynamic Programming (Quy hoạch động) an toàn với bộ nhớ
> - Graph Traversal (DFS/BFS) trong Rust (tránh rắc rối với borrow checker)
>
> **Yêu cầu trước**: Chapter 3 (Data Structures)
> **Thời gian đọc**: ~45 phút | **Level**: Intermediate

---

## 3B.1 — Tại sao phải học LeetCode Patterns?

Trong kỹ thuật phần mềm truyền thống, bạn hiếm khi phải tự viết lại thuật toán Dijkstra hay cân bằng cây nhị phân (vì standard library đã làm sẵn). Tuy nhiên:
1. **Phỏng vấn (Interviews)**: Các công ty công nghệ lớn (FAANG/MAANG) luôn yêu cầu thuật toán.
2. **AI Engineering**: Khi làm việc với LLMs, việc xử lý luồng dữ liệu lớn (streaming tokens, phân tích đồ thị tri thức - Knowledge Graphs) đòi hỏi bạn phải nắm vững cấu trúc dữ liệu và thuật toán để tối ưu độ phức tạp thời gian $O(N)$.

Trong Rust, áp dụng các pattern thuật toán đôi khi gặp khó khăn do **Borrow Checker**. Chương này hướng dẫn bạn cách viết thuật toán hiệu quả, idiomatic và thân thiện với bộ nhớ.

---

## 3B.2 — Pattern 1: Two Pointers & Sliding Window

Kỹ thuật **Two Pointers** dùng 2 con trỏ (thường là chỉ số mảng) di chuyển từ 2 phía hoặc cùng phía để giải bài toán tuyến tính $O(N)$ thay vì $O(N^2)$.

### Ví dụ: Two Sum II (Mảng đã sắp xếp)

```rust
// ❌ Cách Imperative (Giống C/C++)
pub fn two_sum_imperative(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1; // Khá rủi ro nếu right = 0 do kiểu usize
        }
    }
    None
}
```

### Sliding Window: Tìm chuỗi con dài nhất

**Sliding Window** dùng để tìm tập con liên tiếp thỏa mãn điều kiện.

```rust
use std::cmp::max;

// Tìm độ dài của mảng con liên tiếp dài nhất có tổng <= max_sum
pub fn longest_subarray_sum(nums: &[i32], max_sum: i32) -> usize {
    let mut max_len = 0;
    let mut current_sum = 0;
    let mut left = 0;

    for right in 0..nums.len() {
        current_sum += nums[right];

        // Shrink the window if current_sum is too large
        while current_sum > max_sum && left <= right {
            current_sum -= nums[left];
            left += 1;
        }

        max_len = max(max_len, right - left + 1);
    }

    max_len
}
```
*Lưu ý FP*: Thuật toán Sliding Window về bản chất là một State Machine. Trong Rust, dùng `for` loop với state mutation cục bộ thường nhanh và sạch hơn là đệ quy (do Rust chưa tối ưu hóa Tail Call hoàn toàn).

---

## 3B.3 — Pattern 2: Dynamic Programming (Quy hoạch động)

Dynamic Programming (DP) là kỹ thuật giải quyết bài toán phức tạp bằng cách chia nhỏ thành các bài toán con chồng chéo (overlapping subproblems) và lưu kết quả lại (memoization/tabulation).

### Ví dụ: Fibonacci bằng Memoization (Top-Down)

Trong các ngôn ngữ Garbage Collected, bạn có thể truyền một Hash Map đi khắp nơi. Trong Rust, bạn phải cẩn thận với mutable references (`&mut HashMap`).

```rust
use std::collections::HashMap;

// Top-Down DP (Memoization)
pub fn fib_memo(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    // Base cases
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    // Check if already computed
    if let Some(&ans) = memo.get(&n) {
        return ans;
    }

    // Compute and store
    let ans = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo.insert(n, ans);
    ans
}

// Cách gọi:
// let mut memo = HashMap::new();
// let result = fib_memo(50, &mut memo);
```

### Ví dụ: Fibonacci bằng Tabulation (Bottom-Up - Chuẩn Rust)

Cách tiếp cận Bottom-Up sử dụng một mảng (Vector) cực kỳ thân thiện với Rust cache locality và tránh đệ quy sâu.

```rust
pub fn fib_tab(n: usize) -> u64 {
    if n == 0 { return 0; }
    let mut dp = vec![0; n + 1];
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}
```
> **Tip**: Đối với DP trong Rust, **Bottom-up Tabulation luôn được ưu tiên** hơn Top-down Memoization vì nó dùng cấu trúc dữ liệu phẳng (`Vec`), ít bị phân mảnh bộ nhớ và không gặp vấn đề Borrow Checker khi đệ quy.

---

## 3B.4 — Pattern 3: Graph Traversal (DFS/BFS)

Đồ thị (Graphs) trong Rust nổi tiếng là "cơn ác mộng" nếu bạn cố gắng dùng pointers (`Rc<RefCell<Node>>`) như trong C++ hay Java. 

### Cách Idiomatic: Dùng Adjacency List (Vec<Vec<usize>>)

Thay vì các Node trỏ lẫn nhau, ta biểu diễn đồ thị bằng một mảng các mảng, nơi **ID của Node chính là index trong mảng**.

```rust
use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>; // Index là ID của Node

// Breadth-First Search (BFS)
pub fn bfs(graph: &Graph, start_node: usize, target: usize) -> bool {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    visited[start_node] = true;
    queue.push_back(start_node);

    while let Some(current) = queue.pop_front() {
        if current == target {
            return true;
        }

        for &neighbor in &graph[current] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    false
}
```

### Ứng dụng trong AI (Knowledge Graphs / RAG)

Khi bạn xây dựng một hệ thống Graph RAG (Retrieval-Augmented Generation với Knowledge Graphs), dữ liệu thường nằm trong một Graph DB (Neo4j). Nhưng ở cấp độ in-memory processing, bạn sẽ cần duyệt qua các entity liên quan (BFS/DFS) để thu thập context cho prompt. Việc lưu trữ bằng Index (Adjacency List) giúp Rust truy cập cache siêu nhanh.

---

## Checkpoint

- [x] **Sliding Window** và **Two Pointers**: Tối ưu O(N) bằng cách thao tác trên `slice` (`&[T]`).
- [x] **Dynamic Programming**: Ưu tiên Bottom-up (Dùng `Vec`) thay vì Top-down đệ quy để vượt qua Borrow Checker và tăng tốc độ.
- [x] **Graphs**: Tránh dùng con trỏ chuỗi (`Rc/RefCell`). Hãy dùng `Vec` của `Vec` (Adjacency List) với ID là kiểu `usize`.

---
## Tiếp theo

Bạn đã biết cách viết thuật toán an toàn và tối ưu. Ở Chapter tiếp theo (3C), chúng ta sẽ rời khỏi tầng Software và nhìn vào kiến trúc **Phần cứng (Hardware)**: Tại sao CPU tính toán AI lại chậm? CUDA là gì? Và tại sao bộ nhớ (VRAM) lại quyết định sức mạnh của LLM?
