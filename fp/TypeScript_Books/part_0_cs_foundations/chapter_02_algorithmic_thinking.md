# Chapter 2 — Algorithmic Thinking & Complexity

> **Bạn sẽ học được**:
> - Big-O là gì — tại sao code này chạy nhanh hơn code kia
> - Recursion — function gọi chính nó (cách FP thay thế vòng lặp `for`)
> - `.reduce()` — "vũ khí tối thượng" thay thế mọi vòng lặp
> - Binary search, divide & conquer — tư duy chia để trị
>
> **Yêu cầu trước**: Chapter 0 (TypeScript in 10 Minutes). Chapter 1 (Lambda, Curry-Howard).
> **Thời gian đọc**: ~45 phút | **Level**: CS Foundations
> **Kết quả cuối cùng**: Nhìn vào bất kỳ đoạn code nào và ước lượng được "code này chạy nhanh hay chậm" bằng Big-O.

---

## 2.1 — Big-O: Đo tốc độ code

Big-O cho biết code chậm đi bao nhiêu khi data lớn lên. O(n) = tuyến tính, O(n²) = bình phương, O(log n) = cực nhanh.

### Câu chuyện mở đầu

Bạn có 1.000 học sinh. Bạn cần tìm bạn "An" trong danh sách.

**Cách 1**: Đọc lần lượt từ đầu đến cuối → tệ nhất phải đọc 1.000 tên.
**Cách 2**: Danh sách đã xếp theo ABC. Mở giữa → "M" → An ở nửa đầu → mở giữa nửa đầu → lặp lại → chỉ cần ~10 lần.

Cách 1 mất 1.000 bước. Cách 2 mất 10 bước. Nhưng làm sao nói điều này một cách "khoa học"?

Đó là **Big-O** — cách các lập trình viên đo "code này chậm hay nhanh khi data lớn lên".

### Big-O là gì?

Big-O trả lời câu hỏi: **"Khi input tăng gấp đôi, thời gian tăng bao nhiêu?"**

| Big-O | Tên gọi | Ý nghĩa | Ví dụ đời thường |
|-------|---------|---------|------------------|
| O(1) | Constant | Luôn nhanh, bất kể data lớn | Mở tủ lạnh lấy nước — có 1 chai hay 100 chai, bạn biết chỗ |
| O(log n) | Logarithmic | Rất nhanh, tăng chậm | Tìm từ trong từ điển — mở giữa, loại nửa |
| O(n) | Linear | Tăng tỷ lệ thuận | Đếm tiền trong ví — nhiều tờ hơn = lâu hơn |
| O(n log n) | Linearithmic | Khá nhanh | Sắp xếp bài thi — chia nhỏ ra rồi ghép |
| O(n²) | Quadratic | Chậm! | So sánh từng cặp học sinh — 100 người = 10.000 cặp |

> **💡 Mẹo**: O(1) và O(log n) → tuyệt vời. O(n) → chấp nhận được. O(n²) → cẩn thận khi data lớn.

### Big-O trong TypeScript

Hãy xem Big-O của các thao tác phổ biến trên Array:

```typescript
// filename: src/big_o.ts
import assert from "node:assert/strict";

const numbers = [10, 20, 30, 40, 50];

// O(1) — truy cập theo index, luôn nhanh
const first = numbers[0];           // 10
const third = numbers[2];           // 30
assert.strictEqual(first, 10);
assert.strictEqual(third, 30);

// O(1) — .push() thêm cuối (amortized)
const copy = [...numbers];
copy.push(60);                      // nhanh!
assert.strictEqual(copy.length, 6);

// O(n) — .unshift() thêm đầu → phải dịch TẤT CẢ phần tử
// [10, 20, 30] → thêm 5 vào đầu → [5, 10, 20, 30]
//                  dịch: 10→, 20→, 30→ = 3 bước
const copy2 = [...numbers];
copy2.unshift(5);                   // chậm!

// O(n) — .includes() tìm phần tử → duyệt lần lượt
const has30 = numbers.includes(30); // phải kiểm tra 10, 20, 30... tìm thấy!
assert.strictEqual(has30, true);

// O(n) — .reduce() tính tổng → duyệt hết
const total = numbers.reduce((sum, x) => sum + x, 0);
assert.strictEqual(total, 150);

console.log(`First: ${first}, Total: ${total}`);
// Output: First: 10, Total: 150
```

### Bảng Big-O cho Array

| Thao tác | Big-O | Giải thích |
|----------|-------|------------|
| `arr[i]` | O(1) | Nhảy thẳng tới vị trí |
| `arr.length` | O(1) | Giá trị lưu sẵn |
| `arr.push(x)` | O(1)* | Thêm cuối (amortized) |
| `arr.pop()` | O(1) | Bỏ cuối |
| `arr.unshift(x)` | **O(n)** | Thêm đầu → dịch tất cả |
| `arr.shift()` | **O(n)** | Bỏ đầu → dịch tất cả |
| `arr.includes(x)` | O(n) | Duyệt cho đến khi tìm thấy |
| `arr.map(f)` | O(n) | Duyệt hết, tạo array mới |
| `arr.reduce(f, init)` | O(n) | Duyệt hết, gom kết quả |
| `arr.sort()` | O(n log n) | Comparison sort |
| `arr.indexOf(x)` | O(n) | Duyệt tuần tự |

\* Amortized = trung bình. Đôi khi phải mở rộng bộ nhớ → O(n), nhưng hiếm.

> **💡 Bẫy phổ biến**: `.unshift()` trông vô hại nhưng là O(n)! Nếu data lớn, tránh dùng. Nếu cần thêm đầu thường xuyên → xem xét cấu trúc dữ liệu khác (Chapter 3).

---

## ✅ Checkpoint 2.1

> Đến đây bạn phải hiểu:
> 1. **Big-O** = đo tốc độ code khi data lớn lên
> 2. O(1) → tuyệt. O(n) → ổn. O(n²) → cẩn thận
> 3. `arr[i]` O(1), `.push()` O(1), `.unshift()` **O(n)**, `.sort()` O(n log n)
>
> **Test nhanh**: Bạn có array 1.000.000 phần tử. `arr[999999]` mất bao lâu so với `arr[0]`?
> <details><summary>Đáp án</summary>Giống nhau! Cả hai đều O(1) — nhảy thẳng tới vị trí, không phụ thuộc vào index.</details>

---

## 2.2 — Recursion: Function gọi chính nó

Bạn biết TypeScript có `for` và `while`. Nhưng vòng lặp thường đi kèm `let` — nghĩa là **mutation**. Trong FP, ta ưu tiên **recursion** (cho cấu trúc đệ quy) và **higher-order functions** (`.map()`, `.reduce()` cho collections). Cả hai giữ dữ liệu bất biến.

### Recursion cơ bản

Bắt đầu với ví dụ kinh điển: tính giai thừa.

```
5! = 5 × 4 × 3 × 2 × 1 = 120
```

Recursion nghĩ khác: **"5! = 5 × (4!)"**

```
5! = 5 × 4!
4! = 4 × 3!
3! = 3 × 2!
2! = 2 × 1!
1! = 1            ← điểm dừng (base case)
```

```typescript
// filename: src/recursion.ts
import assert from "node:assert/strict";

// Giai thừa — recursion cơ bản
const factorial = (n: number): number => {
    if (n <= 1) return 1;               // base case — điểm dừng
    return n * factorial(n - 1);        // gọi chính mình với n nhỏ hơn
};

assert.strictEqual(factorial(5), 120);
assert.strictEqual(factorial(10), 3628800);

console.log(`5! = ${factorial(5)}`);
console.log(`10! = ${factorial(10)}`);
// Output:
// 5! = 120
// 10! = 3628800
```

Mọi recursion đều cần **2 phần**:
1. **Base case** — khi nào dừng? (`n <= 1 → 1`)
2. **Recursive case** — bước nhỏ hơn (`n * factorial(n - 1)`)

> **💡 Ẩn dụ**: Recursion giống **búp bê Nga (matryoshka)**. Mở búp bê to, bên trong có búp bê nhỏ hơn, mở tiếp... cho đến búp bê bé nhất (base case). Rồi ghép lại từ trong ra ngoài.

### Vấn đề: Stack Overflow

Mỗi lần function gọi chính nó, JavaScript engine **dùng thêm bộ nhớ** (gọi là "stack frame"). Nếu gọi quá nhiều lần:

```
factorial(100000)
= 100000 * factorial(99999)
= 100000 * (99999 * factorial(99998))
= 100000 * (99999 * (99998 * ...))
→ 100.000 stack frames → 💥 RangeError: Maximum call stack size exceeded
```

Trong Node.js, giới hạn thường khoảng **~10.000–15.000** lần gọi.

### Tail Recursion — Có giúp được không?

Trong nhiều ngôn ngữ FP (Roc, Erlang, Haskell), **tail-call optimization (TCO)** tái sử dụng stack frame → không bao giờ overflow. Nhưng trong JavaScript/TypeScript:

> ⚠️ **Chỉ Safari hỗ trợ TCO.** V8 (Node.js, Chrome) và SpiderMonkey (Firefox) **KHÔNG** hỗ trợ.

Điều này nghĩa là recursion trong TypeScript có **giới hạn thực tế**. Vì vậy:

```typescript
// filename: src/tail_vs_loop.ts
import assert from "node:assert/strict";

// ✅ Tail-recursive style (nhưng JS engine vẫn có thể overflow)
const factorialTail = (n: number, acc: number = 1): number => {
    if (n <= 1) return acc;
    return factorialTail(n - 1, acc * n);  // tail position
};

assert.strictEqual(factorialTail(5), 120);
assert.strictEqual(factorialTail(20), 2432902008176640000);

// ✅ Cách an toàn nhất trong TS: dùng .reduce() hoặc iteration
const factorialReduce = (n: number): number =>
    Array.from({ length: n }, (_, i) => i + 1)
        .reduce((acc, x) => acc * x, 1);

assert.strictEqual(factorialReduce(5), 120);
assert.strictEqual(factorialReduce(20), 2432902008176640000);

console.log("Both approaches work! ✅");
// Output: Both approaches work! ✅
```

> **💡 Quy tắc thực tế trong TypeScript**: Dùng recursion cho cấu trúc **nhỏ** hoặc **tự nhiên đệ quy** (trees, nested data). Cho collections lớn → dùng `.reduce()`, `.map()`, `.filter()`. Không viết recursion cho list 1 triệu phần tử.

---

## ✅ Checkpoint 2.2

> Đến đây bạn phải hiểu:
> 1. **Recursion** = function gọi chính nó, cần base case + recursive case
> 2. Recursion tốn **stack** → giới hạn ~10.000 lần trong Node.js
> 3. **TCO không đảm bảo** trong V8/SpiderMonkey → dùng `.reduce()` cho collections lớn
> 4. Recursion tốt cho: trees, nested data, thuật toán chia để trị
>
> **Test nhanh**: Function nào an toàn hơn khi n = 1.000.000?
> ```typescript
> // A — recursion
> const sumA = (arr: readonly number[]): number =>
>     arr.length === 0 ? 0 : arr[0] + sumA(arr.slice(1));
>
> // B — reduce
> const sumB = (arr: readonly number[]): number =>
>     arr.reduce((s, x) => s + x, 0);
> ```
> <details><summary>Đáp án</summary>B! sumA gọi đệ quy 1 triệu lần → stack overflow. sumB dùng .reduce() — iteration nội bộ, không tốn stack.</details>

---

## 2.3 — `.reduce()` — Thay thế mọi vòng lặp

`.reduce()` là higher-order function mạnh nhất — nó thay thế `for`, `while`, `fold`, `walk` trong một API duy nhất.

### Tại sao `.reduce()`?

Trong FP, ta muốn **không thay đổi biến**. Nhưng vòng lặp `for` thường yêu cầu `let`:

```typescript
// ❌ Cách imperative — dùng let (mutation)
let total = 0;
for (const x of [1, 2, 3, 4, 5]) {
    total += x;  // total bị thay đổi mỗi vòng
}
// total = 15
```

`.reduce()` giải quyết bằng cách **gom kết quả qua mỗi phần tử** mà không cần `let`:

```typescript
// ✅ Cách FP — dùng .reduce() (no mutation)
const total = [1, 2, 3, 4, 5].reduce((sum, x) => sum + x, 0);
// total = 15 — const, không bao giờ thay đổi!
```

### Cách hoạt động

```
arr.reduce(combineFunction, startValue)
```

Đọc: "Đi qua array, bắt đầu với `startValue`, và mỗi bước dùng `combineFunction` để gom kết quả."

```
Array:   [1,    2,    3]
          ↓     ↓     ↓
Start → [+1] → [+2] → [+3] → Kết quả
  0       1      3      6       = 6
```

```typescript
// filename: src/reduce.ts
import assert from "node:assert/strict";

const numbers = [1, 2, 3, 4, 5];

// Tính tổng — giống "for x in list: total += x"
const total = numbers.reduce((state, x) => state + x, 0);
assert.strictEqual(total, 15);

// Tìm max — giống "for x in list: max = Math.max(max, x)"
const maxVal = numbers.reduce((state, x) => x > state ? x : state, -Infinity);
assert.strictEqual(maxVal, 5);

// Đếm số chẵn — giống "for x in list: if x%2==0: count++"
const evenCount = numbers.reduce(
    (state, x) => x % 2 === 0 ? state + 1 : state, 0
);
assert.strictEqual(evenCount, 2);

// Đảo ngược
const reversed = numbers.reduce<readonly number[]>(
    (state, x) => [x, ...state], []
);
assert.deepStrictEqual(reversed, [5, 4, 3, 2, 1]);

console.log(`Total: ${total}`);
console.log(`Max: ${maxVal}`);
console.log(`Even count: ${evenCount}`);
console.log(`Reversed: ${JSON.stringify(reversed)}`);
// Output:
// Total: 15
// Max: 5
// Even count: 2
// Reversed: [5,4,3,2,1]
```

### Họ hàng của `.reduce()`

TypeScript có nhiều methods dựng sẵn thay thế các patterns phổ biến:

```typescript
// filename: src/array_methods.ts
import assert from "node:assert/strict";

const numbers = [1, 2, 3, 4, 5];

// .map() — biến đổi mỗi phần tử
const doubled = numbers.map(x => x * 2);
assert.deepStrictEqual(doubled, [2, 4, 6, 8, 10]);

// .filter() — lọc theo điều kiện
const evens = numbers.filter(x => x % 2 === 0);
assert.deepStrictEqual(evens, [2, 4]);

// .find() — tìm phần tử đầu tiên thỏa điều kiện
const firstEven = numbers.find(x => x % 2 === 0);
assert.strictEqual(firstEven, 2);

// .some() — có phần tử nào thỏa điều kiện?
const hasEven = numbers.some(x => x % 2 === 0);
assert.strictEqual(hasEven, true);

// .every() — TẤT CẢ phần tử thỏa điều kiện?
const allPositive = numbers.every(x => x > 0);
assert.strictEqual(allPositive, true);

// .flatMap() — map rồi flatten
const pairs = numbers.flatMap(x => [x, x * 10]);
assert.deepStrictEqual(pairs, [1, 10, 2, 20, 3, 30, 4, 40, 5, 50]);

console.log("All array methods passed! ✅");
// Output: All array methods passed! ✅
```

### Method chaining = Pipeline

Kết hợp nhiều bước xử lý bằng cách **nối** methods. Data chảy từ trên xuống dưới, giống dây chuyền sản xuất:

```typescript
// filename: src/pipeline.ts
import assert from "node:assert/strict";

const result = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    .map(x => x * x)              // bình phương: [1, 4, 9, 16, 25, ...]
    .filter(x => x > 10)          // giữ > 10:   [16, 25, 36, 49, 64, 81, 100]
    .reduce((s, x) => s + x, 0);  // tổng:       371

assert.strictEqual(result, 371);
console.log(`Pipeline result: ${result}`);
// Output: Pipeline result: 371
```

> **💡 Pipeline** đọc từ trên xuống dưới, giống cách bạn đọc văn bản. Rất dễ đọc, dễ debug (xóa 1 bước, log kết quả trung gian).

### `.reduce()` vs `.map()` vs `.filter()` — Khi nào dùng gì?

| Cần gì? | Method | Ví dụ |
|---------|--------|-------|
| Biến đổi **mỗi phần tử** | `.map()` | Nhân đôi mỗi số |
| Lọc **một số phần tử** | `.filter()` | Giữ số chẵn |
| Gom lại thành **1 giá trị** | `.reduce()` | Tính tổng, tìm max |
| Tìm **1 phần tử** | `.find()` | Tìm số chẵn đầu tiên |
| Kiểm tra **có/không** | `.some()` / `.every()` | Có số âm không? |
| Tất cả logic trên | `.reduce()` | Có thể thay mọi thứ, nhưng kém rõ ràng |

> **💡 Quy tắc thực tế**: Dùng `.map()`, `.filter()`, `.find()` trước. Chỉ dùng `.reduce()` khi cần logic tùy chỉnh (đếm, gom object, flatten tùy chỉnh). Code đọc dễ hơn code viết nhanh.

---

## ✅ Checkpoint 2.3

> Đến đây bạn phải hiểu:
> 1. **`.reduce()`** = duyệt array, gom kết quả — thay thế vòng lặp `for`
> 2. `.reduce(combineFunc, startValue)` — 2 tham số: hàm gom + giá trị bắt đầu
> 3. `.map()` = biến đổi, `.filter()` = lọc, `.find()` = tìm, `.some()`/`.every()` = kiểm tra
> 4. **Method chaining** kết hợp nhiều bước xử lý data thành pipeline
>
> **Test nhanh**: Viết `.reduce()` tính tích (product) của `[2, 3, 5]`?
> <details><summary>Đáp án</summary>`[2, 3, 5].reduce((state, x) => state * x, 1)` → 30. Bắt đầu từ 1 (đơn vị nhân), nhân dần.</details>

---

## 2.4 — Binary Search & Divide and Conquer

Chia đôi, tìm nửa đúng, lặp lại — O(log n) thay vì O(n). Từ 1 triệu phần tử chỉ cần 20 bước.

### Binary Search — Tìm kiếm siêu nhanh

Quay lại câu chuyện 1.000 học sinh. **Binary search** = mở giữa, loại nửa, lặp lại.

```
Tìm "An" trong ["An", "Bình", "Cường", "Dũng", "Em"]

Bước 1: Giữa = "Cường" → "An" < "Cường" → tìm nửa trái
Bước 2: Giữa = "Bình"  → "An" < "Bình"  → tìm nửa trái
Bước 3: Giữa = "An"    → Tìm thấy! ✅

Chỉ 3 bước thay vì 5 bước (linear search)
```

**Yêu cầu**: Array phải **đã sắp xếp**.
**Big-O**: O(log n) — 1.000.000 phần tử chỉ cần ~20 bước!

```typescript
// filename: src/binary_search.ts
import assert from "node:assert/strict";

// Binary search — tìm phần tử trong array đã sắp xếp
const binarySearch = (sortedArr: readonly number[], target: number): number | undefined => {
    let low = 0;
    let high = sortedArr.length - 1;

    while (low <= high) {
        const mid = Math.floor((low + high) / 2);
        const midVal = sortedArr[mid];

        if (midVal === target) return mid;           // tìm thấy!
        if (midVal < target)   low = mid + 1;        // tìm nửa phải
        else                   high = mid - 1;       // tìm nửa trái
    }

    return undefined;  // không tìm thấy
};

const numbers = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91];

assert.strictEqual(binarySearch(numbers, 23), 5);
assert.strictEqual(binarySearch(numbers, 42), undefined);

console.log(`Tìm 23: index ${binarySearch(numbers, 23)}`);
console.log(`Tìm 42: ${binarySearch(numbers, 42) ?? "không có"}`);
// Output:
// Tìm 23: index 5
// Tìm 42: không có
```

Chú ý: function trả `number | undefined` — "có index hoặc không". TypeScript **bắt buộc** bạn xử lý cả hai (nhờ strict mode). Đây là Curry-Howard từ Chapter 1 đang hoạt động!

> **💡 Tại sao dùng iteration thay recursion?** Binary search lặp tối đa ~20 lần cho 1 triệu items — recursion OK. Nhưng iteration rõ ràng hơn và là convention trong TypeScript. Cả hai cách đều O(log n).

### Divide and Conquer — Chia để trị

Binary search là ví dụ của **divide and conquer** — pattern giải quyết bài toán bằng cách:

1. **Chia** bài toán thành bài toán nhỏ hơn
2. **Trị** (giải) từng bài toán nhỏ
3. **Ghép** kết quả lại

Ví dụ kinh điển: **Merge Sort** — sắp xếp bằng chia để trị.

```
Sắp xếp [38, 27, 43, 3, 9, 82, 10]

Chia:   [38, 27, 43, 3]    [9, 82, 10]
Chia:   [38, 27] [43, 3]   [9, 82] [10]
Chia:   [38][27] [43][3]   [9][82] [10]

Ghép:   [27, 38] [3, 43]   [9, 82] [10]
Ghép:   [3, 27, 38, 43]    [9, 10, 82]
Ghép:   [3, 9, 10, 27, 38, 43, 82] ✅
```

```typescript
// filename: src/merge_sort.ts
import assert from "node:assert/strict";

// Merge sort — chia để trị, O(n log n)
const mergeSort = (arr: readonly number[]): readonly number[] => {
    if (arr.length <= 1) return arr;  // base case — 0-1 phần tử đã sorted

    const mid = Math.floor(arr.length / 2);
    const left = mergeSort(arr.slice(0, mid));    // chia trái
    const right = mergeSort(arr.slice(mid));       // chia phải
    return merge(left, right);                     // ghép lại
};

// Ghép 2 arrays đã sorted thành 1 array sorted
const merge = (
    a: readonly number[],
    b: readonly number[]
): readonly number[] => {
    const result: number[] = [];
    let i = 0;
    let j = 0;

    while (i < a.length && j < b.length) {
        if (a[i] <= b[j]) {
            result.push(a[i]);
            i++;
        } else {
            result.push(b[j]);
            j++;
        }
    }

    return [...result, ...a.slice(i), ...b.slice(j)];
};

const unsorted = [38, 27, 43, 3, 9, 82, 10];
const sorted = mergeSort(unsorted);

assert.deepStrictEqual(sorted, [3, 9, 10, 27, 38, 43, 82]);

console.log(`Trước: [${unsorted}]`);
console.log(`Sau:   [${sorted}]`);
// Output:
// Trước: [38,27,43,3,9,82,10]
// Sau:   [3,9,10,27,38,43,82]
```

**Tại sao merge sort quan trọng trong FP?**
- **Không thay đổi array gốc** — luôn tạo array mới (hàm `merge` dùng `let` + `.push()` nội bộ cho hiệu suất, nhưng kết quả trả ra là bất biến)
- O(n log n) — nhanh nhất cho comparison sort
- **Stable** — phần tử bằng nhau giữ thứ tự gốc

> **💡 Thực tế**: Trong TypeScript hàng ngày, bạn dùng `.sort()` hoặc `.toSorted()` thay vì tự viết merge sort. Nhưng hiểu cách nó hoạt động giúp bạn **chọn thuật toán đúng** và **hiểu tại sao `.sort()` là O(n log n)**.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Big-O

Xác định Big-O cho mỗi đoạn code:

```typescript
// a) Lấy phần tử đầu tiên
const first = numbers[0];

// b) Tìm phần tử lớn nhất
const max = numbers.reduce((m, x) => x > m ? x : m, -Infinity);

// c) Kiểm tra tất cả dương
const allPos = numbers.every(x => x > 0);

// d) Sắp xếp
const sorted = [...numbers].sort((a, b) => a - b);

// e) Tìm trong array đã sắp xếp bằng binary search
const found = binarySearch(sortedNumbers, target);
```

<details><summary>✅ Lời giải Bài 1</summary>

```
a) O(1) — truy cập theo index
b) O(n) — duyệt hết array
c) O(n) — duyệt hết array (tệ nhất)
d) O(n log n) — sắp xếp so sánh
e) O(log n) — mỗi bước loại nửa
```

</details>

---

**Bài 2** (10 phút): Viết bằng `.reduce()`

Dùng `.reduce()` để viết các functions sau:

```typescript
// a) Tính tích (product) của array số
// product([2, 3, 5]) = 30

// b) Đếm số phần tử lớn hơn 10
// countAbove10([5, 15, 3, 20, 8]) = 2

// c) Tìm chuỗi dài nhất
// longestStr(["hi", "hello", "hey"]) = "hello"
```

<details><summary>💡 Gợi ý</summary>

- Product: bắt đầu từ 1, nhân dần
- CountAbove10: bắt đầu từ 0, cộng 1 mỗi khi x > 10
- LongestStr: bắt đầu từ "", so sánh `.length`

</details>

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

// a) Tích
const product = (arr: readonly number[]): number =>
    arr.reduce((state, x) => state * x, 1);

assert.strictEqual(product([2, 3, 5]), 30);

// b) Đếm > 10
const countAbove10 = (arr: readonly number[]): number =>
    arr.reduce((state, x) => x > 10 ? state + 1 : state, 0);

assert.strictEqual(countAbove10([5, 15, 3, 20, 8]), 2);

// c) Chuỗi dài nhất
const longestStr = (arr: readonly string[]): string =>
    arr.reduce((state, x) => x.length > state.length ? x : state, "");

assert.strictEqual(longestStr(["hi", "hello", "hey"]), "hello");
```

</details>

---

**Bài 3** (15 phút): Recursive Fibonacci + .reduce() version

Viết function tính số Fibonacci thứ n (fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2)):

```typescript
// Yêu cầu 1: Viết recursive version
// Yêu cầu 2: Viết .reduce() version (an toàn cho n lớn)
// fibonacci(10) = 55
// fibonacci(20) = 6765
```

<details><summary>💡 Gợi ý</summary>

- Recursive: base case fib(0)=0, fib(1)=1. Recursive: fib(n-1) + fib(n-2)
- Reduce: dùng Array.from + .reduce với accumulator `[a, b]`, mỗi bước: `[b, a+b]`

</details>

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

// Recursive — đẹp nhưng chậm (O(2ⁿ)!) và giới hạn stack
const fibRecursive = (n: number): number => {
    if (n <= 0) return 0;
    if (n === 1) return 1;
    return fibRecursive(n - 1) + fibRecursive(n - 2);
};

assert.strictEqual(fibRecursive(10), 55);

// .reduce() — nhanh (O(n)) và an toàn (không stack overflow)
const fibonacci = (n: number): number => {
    if (n <= 0) return 0;
    const [, result] = Array.from({ length: n }, (_, i) => i)
        .reduce<readonly [number, number]>(
            ([a, b]) => [b, a + b],
            [0, 1]
        );
    return result;
};

assert.strictEqual(fibonacci(10), 55);
assert.strictEqual(fibonacci(20), 6765);
assert.strictEqual(fibonacci(50), 12586269025);

console.log(`fib(10) = ${fibonacci(10)}`);
console.log(`fib(50) = ${fibonacci(50)}`);
// Output:
// fib(10) = 55
// fib(50) = 12586269025
// (thử fibRecursive(50) → treo máy vì O(2⁵⁰) ≈ 10¹⁵ lần gọi!)
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `RangeError: Maximum call stack size exceeded` | Recursion quá sâu (~10K+ lần) | Chuyển sang `.reduce()` hoặc iteration |
| Recursion chạy mãi không dừng | Quên base case hoặc base case sai | Kiểm tra: recursive case có tiến gần base case không? |
| `.reduce()` cho kết quả sai | Giá trị bắt đầu sai | Tổng → bắt đầu 0. Tích → bắt đầu 1. Array → bắt đầu `[]` |
| Binary search không tìm thấy | Array chưa sắp xếp | Binary search **yêu cầu** sorted array |
| `.sort()` sắp sai (ví dụ [1, 10, 2]) | `.sort()` mặc định so sánh string | Luôn truyền compare function: `.sort((a, b) => a - b)` |

---

## Tóm tắt

- ✅ **Big-O** đo tốc độ code khi data lớn. O(1) > O(log n) > O(n) > O(n log n) > O(n²).
- ✅ **`.push()` là O(1)** nhưng **`.unshift()` là O(n)** — cẩn thận bẫy hiệu suất.
- ✅ **Recursion** = function gọi chính nó. Cần **base case** + **recursive case**. Giới hạn ~10K trong Node.js.
- ✅ **`.reduce()`** thay thế vòng lặp `for`. 90% trường hợp dùng `.map()`, `.filter()`, `.reduce()`.
- ✅ **Binary search** O(log n) — chia đôi mỗi bước. Yêu cầu array đã sorted.
- ✅ **Divide & conquer** — chia bài toán nhỏ, giải riêng, ghép lại. Merge sort O(n log n).

## Tiếp theo

→ Chapter 3: **Functional Data Structures** — tại sao `{...obj}` spread là O(n), cách **Immer** tối ưu structural sharing, `Map` vs `Set` vs `Object`, và khi nào cần persistent data structures.
