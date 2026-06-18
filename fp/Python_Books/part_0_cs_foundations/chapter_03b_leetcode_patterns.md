# Chapter 3B — Advanced Algorithms & LeetCode Patterns

> **Bạn sẽ học được**:
> - Two Pointers, Sliding Window
> - Dynamic Programming (Quy hoạch động)
> - Graph Traversal (BFS/DFS)
> - Tại sao tư duy Functional có thể áp dụng cho Algorithms
>
> **Yêu cầu trước**: Chapter 3 (Data Structures)
> **Thời gian đọc**: ~30 phút | **Level**: Intermediate

---

## 3B.1 — Two Pointers & Sliding Window

Trong Python, ta thường làm việc với `slice` (`&[T]`) thay vì indices thô.

```python
// Two Pointers: Tìm tổng 2 số trong mảng đã sắp xếp
def two_sum(nums: list[int], target: int) -> tuple[int, int] | return None:
    left = 0
    right = len(nums) - 1

    while left < right:
            sum_val = nums[left] + nums[right]
            if sum_val == target: return (left, right)
            elif sum_val < target: left += 1
            else: right -= 1
    }
    return None
}
```

## 3B.2 — Dynamic Programming (Memoization)

DP thuần Functional thường dùng đệ quy + memoization.

```python
use std::collections::HashMap;

// Fibonacci with Memoization (Top-down)
pub fn fib_memo(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    if n <= 1 { return n as u64; }
    if let Some(&ans) = memo.get(&n) { return ans; }
    
    let ans = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo.insert(n, ans);
    ans
}
```

## Tóm tắt
Các kỹ thuật này (LeetCode patterns) là cốt lõi để qua các vòng phỏng vấn. Tư duy functional khuyên ta dùng Iterator/Slice thay vì mutation khi có thể, nhưng để tối ưu Big-O, thỉnh thoảng mutation cục bộ là cần thiết.
