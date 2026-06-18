# Chapter 2 — Algorithmic Thinking & Complexity

> **Bạn sẽ học được**:
> - Big-O notation — cách đo "tốc độ" của thuật toán
> - Complexity của Python built-in operations
> - Recursion vs Iteration — khi nào dùng cái nào
> - Common patterns: binary search, divide & conquer, memoization
>
> **Yêu cầu trước**: Chapter 1 (Math Foundations)
> **Thời gian đọc**: ~35 phút | **Level**: Pre-requisite
> **Kết quả cuối cùng**: Nhìn vào code và ước lượng được "nhanh hay chậm".

---

## Tại sao cần biết Big-O?

Bạn viết code, nó chạy. Nhưng với 100 records thì chạy 0.1 giây, với 1 triệu records thì chạy 3 giờ. Tại sao? Vì bạn không để ý **complexity** — tốc độ tăng trưởng thời gian theo kích thước input.

Big-O cho bạn ngôn ngữ để nói: "Algorithm này là O(n)" — nghĩa là thời gian tăng tuyến tính với input. Không cần chạy thử để biết — NHÌN code là ước lượng được.

---

## 2.1 — Big-O: Ngôn ngữ của tốc độ

### Các mức phổ biến

```python
import time

# O(1) — Constant: không phụ thuộc input size
def get_first(lst: list) -> object:
    return lst[0] if lst else None

# O(log n) — Logarithmic: chia đôi mỗi bước
def binary_search(sorted_lst: list[int], target: int) -> int:
    lo, hi = 0, len(sorted_lst) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if sorted_lst[mid] == target:
            return mid
        elif sorted_lst[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1

# O(n) — Linear: duyệt qua tất cả
def find_max(lst: list[int]) -> int:
    mx = lst[0]
    for x in lst:
        if x > mx:
            mx = x
    return mx

# O(n log n) — Linearithmic: sort
def sort_list(lst: list[int]) -> list[int]:
    return sorted(lst)  # Timsort = O(n log n)

# O(n²) — Quadratic: nested loops
def has_duplicate_naive(lst: list[int]) -> bool:
    for i in range(len(lst)):
        for j in range(i + 1, len(lst)):
            if lst[i] == lst[j]:
                return True
    return False

# Tests
data = list(range(1000))
assert get_first(data) == 0                          # O(1)
assert binary_search(data, 500) == 500               # O(log n)
assert find_max(data) == 999                          # O(n)
assert sort_list([3, 1, 2]) == [1, 2, 3]             # O(n log n)
assert has_duplicate_naive([1, 2, 3, 2]) == True      # O(n²)

print("All complexity tests passed ✅")
```

### Python built-in complexity

```python
# === Python Operations & Their Complexity ===

my_list = [1, 2, 3, 4, 5]
my_dict = {"a": 1, "b": 2}
my_set = {1, 2, 3}

# LIST operations
# list.append(x)     → O(1) amortized  ← NHANH
# list.insert(0, x)  → O(n)            ← CHẬM (dịch toàn bộ)
# list[i]            → O(1)            ← NHANH (index access)
# x in list          → O(n)            ← CHẬM (phải duyệt)
# list.sort()        → O(n log n)

# DICT operations
# dict[key]          → O(1) average    ← NHANH (hash lookup)
# key in dict        → O(1) average    ← NHANH
# dict[key] = val    → O(1) average    ← NHANH

# SET operations
# x in set           → O(1) average    ← NHANH
# set.add(x)         → O(1) average    ← NHANH

# === Practical lesson ===
# Muốn kiểm tra "x có trong collection không?"
# ❌ x in my_list    → O(n) — chậm với list lớn
# ✅ x in my_set     → O(1) — nhanh bất kể size
# ✅ x in my_dict    → O(1) — nhanh bất kể size

# Demo
big_list = list(range(1_000_000))
big_set = set(big_list)

# Cả hai đều tìm 999_999 nhưng tốc độ khác nhau hoàn toàn
assert 999_999 in big_list   # O(n) — duyệt gần hết list
assert 999_999 in big_set    # O(1) — hash lookup ngay lập tức

print("Built-in complexity demo ✅")
```

> **💡 Rule of thumb**: Khi cần `x in collection` nhiều lần → convert sang `set`. Khi cần lookup by key → dùng `dict`. Khi cần thứ tự + append → dùng `list`.

---

## 2.2 — Recursion vs Iteration

### Recursion = Function gọi chính nó

```python
import sys

# Factorial: n! = n × (n-1) × ... × 1
def factorial_recursive(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial_recursive(n - 1)

def factorial_iterative(n: int) -> int:
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result

assert factorial_recursive(5) == 120  # 5 × 4 × 3 × 2 × 1
assert factorial_iterative(5) == 120
assert factorial_recursive(0) == 1
assert factorial_iterative(0) == 1

print(f"5! = {factorial_recursive(5)}")
# Output: 5! = 120
```

### Python recursion limit

```python
import sys

# Python mặc định: recursion limit = 1000
print(f"Default recursion limit: {sys.getrecursionlimit()}")

# ❌ factorial_recursive(10_000) → RecursionError!
# Python KHÔNG có tail-call optimization (TCO)
# → Dùng iteration cho trường hợp n lớn

# ✅ Workaround: tăng limit (không khuyến khích production)
# sys.setrecursionlimit(10_000)

# ✅ Better: dùng @lru_cache cho memoization
from functools import lru_cache

@lru_cache(maxsize=None)
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

# Không có cache: O(2^n) — CỰC CHẬM
# Với cache: O(n) — mỗi giá trị tính MỘT LẦN
assert fibonacci(10) == 55
assert fibonacci(50) == 12586269025  # Tính được ngay nhờ cache!

print(f"fibonacci(10) = {fibonacci(10)}")
print(f"fibonacci(50) = {fibonacci(50)}")
# Output:
# fibonacci(10) = 55
# fibonacci(50) = 12586269025
```

---

## 2.3 — Common Patterns

### Binary Search — O(log n)

```python
def binary_search(arr: list[int], target: int) -> int:
    """Tìm vị trí target trong mảng đã sort. Trả -1 nếu không có."""
    lo, hi = 0, len(arr) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1

data = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]
assert binary_search(data, 23) == 5
assert binary_search(data, 100) == -1

print(f"Position of 23: {binary_search(data, 23)}")
# Output: Position of 23: 5
```

### Two Pointers — O(n)

```python
def two_sum_sorted(arr: list[int], target: int) -> tuple[int, int] | None:
    """Tìm 2 số trong sorted array có tổng = target."""
    left, right = 0, len(arr) - 1
    while left < right:
        s = arr[left] + arr[right]
        if s == target:
            return (left, right)
        elif s < target:
            left += 1
        else:
            right -= 1
    return None

data = [1, 3, 5, 7, 9, 11]
assert two_sum_sorted(data, 10) == (1, 4)  # 3 + 7 = 10
assert two_sum_sorted(data, 100) is None

print(f"two_sum(10) = {two_sum_sorted(data, 10)}")
# Output: two_sum(10) = (1, 4)
```

### Divide & Conquer — Merge Sort

```python
def merge_sort(arr: list[int]) -> list[int]:
    """Sort bằng divide & conquer. O(n log n)."""
    if len(arr) <= 1:
        return arr

    mid = len(arr) // 2
    left = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])

    return merge(left, right)

def merge(left: list[int], right: list[int]) -> list[int]:
    result = []
    i = j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    result.extend(left[i:])
    result.extend(right[j:])
    return result

data = [38, 27, 43, 3, 9, 82, 10]
assert merge_sort(data) == [3, 9, 10, 27, 38, 43, 82]

print(f"merge_sort = {merge_sort(data)}")
# Output: merge_sort = [3, 9, 10, 27, 38, 43, 82]
```

> **💡 FP Connection**: Merge sort rất "functional" — nó KHÔNG sửa array gốc, mà tạo arrays MỚI ở mỗi bước. Đây là immutable style mà chúng ta sẽ thấy xuyên suốt cuốn sách.

---

## ✅ Checkpoint 2.3

> Ghi nhớ:
> 1. O(1) < O(log n) < O(n) < O(n log n) < O(n²) < O(2^n)
> 2. Python `list.append` = O(1), `list.insert(0)` = O(n), `x in set` = O(1)
> 3. `@lru_cache` biến O(2^n) recursion → O(n)
> 4. Binary search = O(log n) — cần sorted input
>
> **Test nhanh**: `x in my_list` vs `x in my_set` — cái nào nhanh hơn với 1 triệu phần tử?
> <details><summary>Đáp án</summary><code>x in my_set</code> = O(1). <code>x in my_list</code> = O(n). Set nhanh hơn ~1,000,000 lần.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Xác định Big-O

```python
# Xác định Big-O của mỗi function:
def f1(n): return n * 2                          # ?
def f2(lst): return [x*2 for x in lst]            # ?
def f3(lst):
    for x in lst:
        for y in lst:
            print(x, y)                           # ?
```

<details><summary>✅ Lời giải</summary>f1 = O(1), f2 = O(n), f3 = O(n²)</details>

**Bài 2** (10 phút): Viết `count_occurrences(lst, target)` trả về số lần `target` xuất hiện. Phân tích complexity.

<details><summary>✅ Lời giải Bài 2</summary>

```python
def count_occurrences(lst: list, target) -> int:
    count = 0
    for x in lst:
        if x == target:
            count += 1
    return count  # O(n)

# Hoặc: lst.count(target) — cũng O(n)
assert count_occurrences([1, 2, 3, 2, 2], 2) == 3
```

</details>

**Bài 3** (15 phút): Viết `fibonacci_iterative(n)` không dùng recursion. So sánh với `@lru_cache` version.

<details><summary>✅ Lời giải Bài 3</summary>

```python
def fibonacci_iterative(n: int) -> int:
    if n <= 1:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b  # O(n) time, O(1) space — tốt hơn cached recursion về memory

assert fibonacci_iterative(10) == 55
assert fibonacci_iterative(50) == 12586269025
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `RecursionError: maximum recursion depth exceeded` | Recursion quá sâu (>1000) | Dùng iteration hoặc `sys.setrecursionlimit()` |
| Code chạy chậm với list lớn | O(n²) algorithm hoặc `x in list` | Dùng `set`/`dict` cho lookups, chọn algorithm O(n log n) |
| `@lru_cache` memory leak | Cache giữ tất cả results | Dùng `maxsize` parameter hoặc `.cache_clear()` |

---

## Tóm tắt

- ✅ **Big-O**: Ngôn ngữ đo tốc độ thuật toán. O(1) tốt nhất, O(2^n) tệ nhất.
- ✅ **Python specifics**: `list.append` O(1), `x in set` O(1), `list.sort()` O(n log n).
- ✅ **Recursion**: Elegant nhưng Python giới hạn 1000 levels. Dùng `@lru_cache` hoặc iteration.
- ✅ **Memoization**: `@lru_cache` = cache kết quả đã tính. Biến O(2^n) → O(n).
- ✅ **Common patterns**: Binary search (O(log n)), Two pointers (O(n)), Merge sort (O(n log n)).

## Tiếp theo

→ Chapter 3: **Functional Data Structures** — Immutable data structures, persistent collections, tại sao `tuple` nhanh hơn `list` trong nhiều trường hợp.
