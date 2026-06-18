# Chapter 3B — Advanced Algorithms & LeetCode Patterns

> **Bạn sẽ học được**:
> - Cách giải các bài toán LeetCode bằng tư duy Functional
> - Two Pointers & Sliding Window trong TypeScript
> - Dynamic Programming (Quy hoạch động) an toàn với bộ nhớ
> - Graph Traversal (DFS/BFS) trong TypeScript (tránh state ẩn)
>
> **Yêu cầu trước**: Chapter 3 (Functional Data Structures)
> **Thời gian đọc**: ~45 phút | **Level**: Intermediate

---

## 3B.1 — Tại sao phải học LeetCode Patterns?

Là TypeScript/JavaScript developer, bạn đã quen thuộc với việc viết logic UI hoặc Backend REST APIs. Tuy nhiên, khi làm AI Engineering hoặc phỏng vấn tại các tập đoàn lớn:
1. **Phỏng vấn**: Các bài kiểm tra Data Structures & Algorithms là bắt buộc.
2. **AI Engineering**: Khi parse token từ LLMs, làm việc với Text Chunks cho RAG, hoặc duyệt các cây Knowledge Graph, bạn cần thuật toán có độ phức tạp $O(N)$ hoặc $O(log N)$ thay vì viết vòng lặp lồng nhau $O(N^2)$.

Áp dụng Functional Programming vào thuật toán đôi lúc cần sự cân bằng: Ta hướng tới pure functions, nhưng không ngại mutation cục bộ (local mutation) để tối ưu hiệu năng V8 Engine.

---

## 3B.2 — Pattern 1: Two Pointers & Sliding Window

Kỹ thuật **Two Pointers** dùng 2 con trỏ di chuyển để giải bài toán tuyến tính $O(N)$ thay vì phải loop 2 lần.

### Ví dụ: Two Sum II (Mảng đã sắp xếp)

```typescript
// Tìm 2 số có tổng bằng target. Trả về index.
function twoSum(nums: number[], target: number): [number, number] | null {
    let left = 0;
    let right = nums.length - 1;

    // Local mutation: An toàn vì không thay đổi mảng nums truyền vào
    while (left < right) {
        const sum = nums[left] + nums[right];
        if (sum === target) {
            return [left, right];
        } else if (sum < target) {
            left++;
        } else {
            right--;
        }
    }
    return null;
}
```

### Sliding Window: Tìm chuỗi con dài nhất

Dùng cho các bài toán "subarray/substring liên tiếp".

```typescript
// Tìm độ dài mảng con liên tiếp dài nhất có tổng <= maxSum
function longestSubarraySum(nums: number[], maxSum: number): number {
    let maxLen = 0;
    let currentSum = 0;
    let left = 0;

    for (let right = 0; right < nums.length; right++) {
        currentSum += nums[right];

        // Shrink the window từ bên trái nếu tổng vượt quá
        while (currentSum > maxSum && left <= right) {
            currentSum -= nums[left];
            left++;
        }

        maxLen = Math.max(maxLen, right - left + 1);
    }

    return maxLen;
}
```

---

## 3B.3 — Pattern 2: Dynamic Programming (Quy hoạch động)

Dynamic Programming chia bài toán lớn thành bài toán con chồng chéo và lưu kết quả lại (memoization).

### DP Top-Down (Memoization)

```typescript
// Fibonacci với Memoization, sử dụng closure để giấu state
function makeFib(): (n: number) => number {
    const memo: Record<number, number> = {};

    function fib(n: number): number {
        if (n <= 1) return n;
        if (n in memo) return memo[n];

        const ans = fib(n - 1) + fib(n - 2);
        memo[n] = ans;
        return ans;
    }

    return fib;
}

const fib = makeFib();
console.log(fib(50));
```

### DP Bottom-Up (Tabulation)

Thường an toàn và nhanh hơn trong V8 Engine vì không gây ra Call Stack Overflow.

```typescript
function fibTab(n: number): number {
    if (n <= 1) return n;
    
    // Khởi tạo mảng DP
    const dp = new Array(n + 1).fill(0);
    dp[1] = 1;

    for (let i = 2; i <= n; i++) {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    return dp[n];
}
```

---

## 3B.4 — Pattern 3: Graph Traversal (DFS/BFS)

Trong JS/TS, biểu diễn Graph bằng Adjacency List (Mảng của mảng, hoặc Object của mảng) là tốt nhất để truy xuất dữ liệu.

```typescript
type Graph = number[][]; // Index là Node ID

// Breadth-First Search (BFS)
function bfs(graph: Graph, startNode: number, target: number): boolean {
    const visited = new Array(graph.length).fill(false);
    const queue: number[] = []; // JS arrays can act as queues

    visited[startNode] = true;
    queue.push(startNode);

    while (queue.length > 0) {
        // Dùng shift() cho queue nhỏ. Array lớn nên dùng cấu trúc Queue thật.
        const current = queue.shift()!; 

        if (current === target) return true;

        for (const neighbor of graph[current]) {
            if (!visited[neighbor]) {
                visited[neighbor] = true;
                queue.push(neighbor);
            }
        }
    }
    return false;
}
```

---

## Checkpoint

- [x] **Sliding Window** và **Two Pointers**: Tối ưu tốc độ xử lý chuỗi và mảng, vô cùng hữu ích khi tiền xử lý dữ liệu RAG chunks.
- [x] **Dynamic Programming**: Tối ưu bài toán tối ưu hóa, nhớ dùng Tabulation để tránh Stack Overflow.
- [x] **Graphs**: Tránh object trỏ vòng tròn phức tạp. Sử dụng Adjacency List (mảng ID) thân thiện với bộ nhớ và dễ serialize.

## Tiếp theo
Tiếp theo ở Chapter 3C, chúng ta sẽ tạm gác lại phần mềm để nhìn vào nền tảng vật lý: Hardware, Memory Hierarchy và sức mạnh của GPU. Tại sao JS/NodeJS không phải là ngôn ngữ lý tưởng cho tính toán AI lõi, nhưng lại tuyệt vời ở tầng điều phối?
