# Chapter 3B — Advanced Algorithms & LeetCode Patterns

> **Bạn sẽ học được**:
> - Cách giải các bài toán LeetCode phổ biến bằng Python
> - Two Pointers & Sliding Window
> - Dynamic Programming (Quy hoạch động) với `@cache`
> - Graph Traversal (DFS/BFS) bằng List và Deque
>
> **Yêu cầu trước**: Chapter 3 (Data Structures)
> **Thời gian đọc**: ~45 phút | **Level**: Intermediate

---

## 3B.1 — Tại sao Python Engineer cần LeetCode?

Python được coi là ngôn ngữ "vua" trong các cuộc thi thuật toán và vòng phỏng vấn kỹ thuật vì cú pháp ngắn gọn, dễ mô tả logic. Trong AI Engineering, kỹ năng tối ưu thuật toán $O(N)$ trở nên cực kỳ quan trọng khi bạn viết vòng lặp xử lý hàng triệu token văn bản hoặc duyệt qua các Knowledge Graph khổng lồ.

Mặc dù khóa học hướng tới Tư duy Hàm (FP) và hạn chế state mutation, nhưng trong bài toán tối ưu Big-O, đôi lúc local mutation lại là cách tốt nhất.

---

## 3B.2 — Pattern 1: Two Pointers & Sliding Window

Kỹ thuật **Two Pointers** dùng 2 con trỏ di chuyển từ 2 đầu mảng hoặc chạy cùng nhau để tối ưu hóa thời gian.

### Ví dụ: Two Sum II (Mảng đã sắp xếp)

```python
def two_sum(nums: list[int], target: int) -> tuple[int, int] | None:
    left = 0
    right = len(nums) - 1

    while left < right:
        sum_val = nums[left] + nums[right]
        if sum_val == target:
            return (left, right)
        elif sum_val < target:
            left += 1
        else:
            right -= 1
            
    return None
```

### Sliding Window: Chuỗi con dài nhất

Dùng cho các bài toán "tập hợp con liên tiếp".

```python
def longest_subarray_sum(nums: list[int], max_sum: int) -> int:
    max_len = 0
    current_sum = 0
    left = 0

    for right in range(len(nums)):
        current_sum += nums[right]

        # Shrink window từ bên trái nếu tổng vượt quá
        while current_sum > max_sum and left <= right:
            current_sum -= nums[left]
            left += 1

        max_len = max(max_len, right - left + 1)

    return max_len
```

---

## 3B.3 — Pattern 2: Dynamic Programming (Quy hoạch động)

Trong Python, bạn có thể triển khai DP theo kiểu "Functional" cực kỳ thanh lịch nhờ công cụ tích hợp sẵn `@lru_cache` hoặc `@cache`.

### Top-Down (Memoization) chuẩn Python

Thay vì tự quản lý một dictionary để lưu cache, hãy để Python làm.

```python
from functools import cache

@cache
def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(100)) # Chạy trong nháy mắt nhờ memoization
```

*Lưu ý*: Với dữ liệu siêu lớn, đệ quy trong Python có thể gây ra lỗi giới hạn đệ quy (`RecursionError`). Khi đó, ta dùng Bottom-Up.

### Bottom-Up (Tabulation)

```python
def fib_tab(n: int) -> int:
    if n <= 1:
        return n
    
    dp = [0] * (n + 1)
    dp[1] = 1

    for i in range(2, n + 1):
        dp[i] = dp[i - 1] + dp[i - 2]
        
    return dp[n]
```

---

## 3B.4 — Pattern 3: Graph Traversal (DFS/BFS)

Python xử lý Graph cực kỳ tốt nhờ cấu trúc `dict` và `collections.deque`. Đồ thị thường được biểu diễn dưới dạng Adjacency List.

```python
from collections import deque

Graph = dict[int, list[int]]

def bfs(graph: Graph, start_node: int, target: int) -> bool:
    visited = set()
    queue = deque([start_node]) # deque: Pop hai đầu O(1)
    visited.add(start_node)

    while queue:
        current = queue.popleft() # Pop bên trái

        if current == target:
            return True

        for neighbor in graph.get(current, []):
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append(neighbor)
                
    return False
```

Dùng `deque` cho BFS là nguyên tắc bắt buộc trong Python vì `list.pop(0)` có độ phức tạp $O(N)$ rất chậm.

---

## Checkpoint

- [x] **Sliding Window**: Là một cách duyệt "State Machine". Hoàn hảo cho RAG Text chunking.
- [x] **Dynamic Programming**: `@cache` trong Python biến đệ quy thường thành thuật toán tối ưu $O(N)$ cực kỳ dễ dàng.
- [x] **Graphs**: Tránh dùng objects lồng nhau vòng tròn. Luôn dùng `dict` của các Node ID, và `deque` cho BFS.

## Tiếp theo
Khả năng code thuật toán xịn đã có. Nhưng tại sao mô hình AI viết bằng Python lại vẫn có thể tính toán hàng tỷ parameter mỗi giây? Đó là nhờ C++ và kiến trúc GPU. Hãy sang Chapter 3C để tìm hiểu sâu về Hardware dưới nắp capo.
