# Chapter 12 — Composition & Pipelines

> **Bạn sẽ học được**:
> - Function composition — `compose(f, g)` = "chạy g trước, rồi f"
> - `pipe()` — dễ đọc hơn compose, trái sang phải
> - Method chaining vs pipe — khi nào dùng cái nào
> - Point-free style — "viết function mà không nhắc đến data"
> - Data pipelines thực tế — xử lý business logic bằng composition
> - Builder pattern FP — compose configuration
>
> **Yêu cầu trước**: Chapter 11 (immutability, pure functions, referential transparency).
> **Thời gian đọc**: ~45 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Xây dựng data pipelines phức tạp từ small pure functions.

---

Hãy tưởng tượng bạn đứng trong một nhà máy sản xuất bánh kẹo.

Nguyên liệu thô — bột mì, đường, trứng — được đặt lên đầu **dây chuyền sản xuất**. Mỗi trạm trên dây chuyền thực hiện đúng MỘT công việc: trạm đầu nhào bột, trạm hai cán mỏng, trạm ba cắt hình, trạm cuối nướng. Không trạm nào cần biết trạm trước làm gì — nó chỉ nhận "sản phẩm bán thành phẩm" từ băng chuyền, xử lý, rồi đẩy ra cho trạm tiếp theo.

Đó chính xác là cách **function composition** hoạt động trong lập trình hàm. Mỗi function là một trạm trên dây chuyền. Dữ liệu chảy qua từng trạm, được biến đổi từng bước, cho đến khi ra thành phẩm cuối cùng. Và giống như nhà máy thật, bạn có thể dễ dàng thêm trạm mới, đổi thứ tự, hoặc thay thế một trạm — mà không ảnh hưởng đến các trạm khác.

Trong chương trước, bạn đã học cách viết **pure functions** — những viên gạch Lego không có side effects. Giờ đến lúc ghép chúng lại. Nhưng ghép bằng cách nào? Gọi lồng nhau như `f(g(h(x)))` thì khó đọc. Gán vào biến tạm thì dài dòng. Chương này sẽ cho bạn hai công cụ thanh lịch hơn: **`compose`** và **`pipe`** — và bạn sẽ thấy tại sao `pipe` gần như luôn chiến thắng.

---

## 12.1 — Function Composition: Ghép functions lại

### Dây chuyền đọc ngược

Bạn có 3 functions nhỏ: `trim` bỏ khoảng trắng, `capitalize` viết hoa chữ đầu, `addGreeting` thêm lời chào. Mỗi function làm đúng một việc — giống ba trạm trên dây chuyền. Bây giờ bạn muốn ghép chúng thành một function duy nhất: đưa string vào, nhận lời chào hoàn chỉnh ra.

Trong toán học, phép ghép này có tên gọi chính thức: **function composition**, ký hiệu là `(f ∘ g)(x) = f(g(x))`. Bạn đã gặp nó ở Chapter 1 — giờ chúng ta sẽ biến nó thành code TypeScript thật.

Nhưng có một điểm đáng chú ý: compose đọc **phải sang trái**. `compose(f, g)` có nghĩa là "chạy `g` trước, rồi `f`". Giống như bạn đứng ở cuối dây chuyền nhìn ngược lại — trạm cuối cùng được viết đầu tiên. Điều này hoàn toàn tự nhiên trong toán, nhưng lại trái trực giác khi đọc code.

```typescript
// filename: src/compose.ts
import assert from "node:assert/strict";

// compose(f, g)(x) = f(g(x))
// "Chạy g TRƯỚC, rồi f"
// Giống toán: (f ∘ g)(x) = f(g(x))

const compose = <A, B, C>(
    f: (b: B) => C,
    g: (a: A) => B
): ((a: A) => C) =>
    (a) => f(g(a));

// Ví dụ
const double = (x: number): number => x * 2;
const addOne = (x: number): number => x + 1;

// compose(addOne, double)(5)
// = addOne(double(5))
// = addOne(10)
// = 11
const doubleThenAdd = compose(addOne, double);
assert.strictEqual(doubleThenAdd(5), 11);

// compose đọc PHẢI → TRÁI — khó đọc khi nhiều functions
const negate = (x: number): number => -x;
const doubleThenAddThenNegate = compose(negate, compose(addOne, double));
assert.strictEqual(doubleThenAddThenNegate(5), -11);
// 😵 Nested compose = khó đọc → pipe giải quyết!

console.log("compose OK ✅");
```

Hãy chú ý dòng cuối: `compose(negate, compose(addOne, double))`. Bạn muốn "nhân đôi → cộng 1 → đổi dấu", nhưng phải viết ngược lại: `negate` ở ngoài cùng, `double` ở trong cùng nhất. Với 3 functions đã khó đọc, hãy tưởng tượng dây chuyền 10 trạm — sẽ trở thành cơn ác mộng!

Đây không phải lỗi của compose — nó đúng theo định nghĩa toán học. Nhưng code không phải toán. Code cần đọc **trái sang phải**, theo **thứ tự thực thi**. Và đó là lý do `pipe` ra đời.

> **💡 Compose = toán học**: `(f ∘ g)(x) = f(g(x))` — Chapter 1 đã giới thiệu. Compose đọc phải→trái (giống toán), nhưng code đọc trái→phải. Đó là lý do chúng ta cần `pipe`.

---

## ✅ Checkpoint 12.1

> Đến đây bạn phải hiểu:
> 1. **`compose(f, g)(x)`** = `f(g(x))` — g chạy trước, rồi f
> 2. **Phải → trái** — khó đọc khi nhiều functions
> 3. compose từ Chapter 1 (toán) giờ áp dụng vào TypeScript
>
> **Test nhanh**: `compose(x => x + 10, x => x * 3)(4)` = ?
> <details><summary>Đáp án</summary>`22`. g(4) = 4 * 3 = 12, rồi f(12) = 12 + 10 = 22.</details>

---

## 12.2 — `pipe()` — Trái sang phải

### Đảo ngược dây chuyền

Nếu `compose` là đứng ở cuối dây chuyền nhìn ngược về nguồn, thì `pipe` là **bước đi dọc theo dây chuyền, từ đầu đến cuối**. Bạn bắt đầu với nguyên liệu thô ở vị trí đầu tiên, rồi đi qua từng trạm theo đúng thứ tự chúng xử lý.

Sự khác biệt này nghe đơn giản, nhưng tác động lên khả năng đọc code là rất lớn. Khi bạn viết `pipe(5, double, addOne, negate, toString)`, bạn đọc từ trái sang phải: *"lấy 5, nhân đôi, cộng 1, đổi dấu, thành chuỗi"*. Mỗi bước là một trạm trên dây chuyền, và bạn có thể trace flow dễ dàng vì **thứ tự đọc = thứ tự thực thi**.

Trong F#, Elixir, và Roc, `pipe` được tích hợp sẵn vào ngôn ngữ với toán tử `|>` (pipe operator). TypeScript chưa có toán tử này (TC39 proposal đang ở Stage 2), nên chúng ta sẽ tự xây dựng bằng function overloads — một kỹ thuật cho phép TypeScript giữ type safety qua từng bước của pipeline.

```typescript
// filename: src/pipe.ts
import assert from "node:assert/strict";

// pipe(x, f, g, h)
// = h(g(f(x)))
// Nhưng đọc trái→phải: "lấy x, áp dụng f, rồi g, rồi h"

// Overloaded pipe — hỗ trợ 1-5 functions
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe<A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, f: (a: A) => B, g: (b: B) => C, h: (c: C) => D): D;
function pipe<A, B, C, D, E>(
    a: A, f: (a: A) => B, g: (b: B) => C, h: (c: C) => D, i: (d: D) => E
): E;
function pipe(a: unknown, ...fns: ReadonlyArray<(x: unknown) => unknown>): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

const double = (x: number): number => x * 2;
const addOne = (x: number): number => x + 1;
const negate = (x: number): number => -x;
const toString = (x: number): string => `Result: ${x}`;

// pipe đọc trái→phải — dễ hiểu!
const result = pipe(
    5,          // bắt đầu: 5
    double,     // → 10
    addOne,     // → 11
    negate,     // → -11
    toString,   // → "Result: -11"
);

assert.strictEqual(result, "Result: -11");

console.log(result);
// Output: Result: -11
```

Hãy nhìn kỹ phần overloads ở trên. Mỗi overload là một "phiên bản" của `pipe` cho số lượng functions khác nhau. Overload đầu tiên cho 1 function: `pipe<A, B>(a: A, f: (a: A) => B): B` — TypeScript biết rằng output type `B` chính xác là return type của `f`. Overload thứ hai cho 2 functions: TypeScript tự suy ra rằng output của `f` (type `B`) phải khớp input của `g`, và kết quả cuối cùng là type `C`.

Đây là điều mà `reduce` thuần không thể làm được — nó sẽ mất type safety vì tất cả functions bị gom vào một array. Function overloads cho phép mỗi "tầng" của pipeline giữ nguyên type riêng, giống như mỗi trạm trên dây chuyền biết chính xác mình nhận gì và trả gì.

### So sánh: `compose` vs `pipe`

```typescript
// Cùng phép tính: double → addOne → negate → toString

// compose — đọc phải→trái (ngược thứ tự thực thi)
// compose(toString, compose(negate, compose(addOne, double)))(5)
// 😵 Khó đọc!

// pipe — đọc trái→phải (CÙNG thứ tự thực thi)
// pipe(5, double, addOne, negate, toString)
// ✅ Đọc như câu: "lấy 5, nhân đôi, cộng 1, đổi dấu, thành string"
```

| | `compose` | `pipe` |
|---|---|---|
| **Hướng** | Phải → trái | Trái → phải ✅ |
| **Đọc** | Ngược thứ tự thực thi | **CÙNG** thứ tự thực thi |
| **Dùng khi** | Tạo function mới (point-free) | **Xử lý data** (có giá trị đầu vào) |
| **Quy tắc sách** | Hiếm khi | ✅ **Ưu tiên** |

Nếu quay lại hình ảnh nhà máy: `compose` giống việc bạn thiết kế dây chuyền trên bản vẽ (chỉ định nghĩa, chưa chạy). `pipe` giống việc bạn đặt nguyên liệu lên băng chuyền và bấm nút Start. Trong thực tế, bạn sẽ dùng `pipe` cho hầu hết mọi thứ — vì bạn thường có data cần xử lý, chứ không chỉ muốn tạo function trừu tượng.

> **💡 Quy tắc**: `pipe` cho data processing (có value). `compose` cho tạo function mới (hiếm). Sách này chủ yếu dùng `pipe`.

---

## ✅ Checkpoint 12.2

> Đến đây bạn phải hiểu:
> 1. **`pipe(x, f, g, h)`** = `h(g(f(x)))` — đọc trái→phải
> 2. **pipe > compose** khi xử lý data — dễ đọc, cùng hướng thực thi
> 3. `pipe` dùng function overloads để giữ type safety
>
> **Test nhanh**: `pipe(10, x => x - 3, x => x * 2)` = ?
> <details><summary>Đáp án</summary>`14`. f(10) = 10 - 3 = 7, rồi g(7) = 7 * 2 = 14.</details>

---

## 12.3 — Method Chaining vs Pipe

### Hai kiểu đọc trái-sang-phải

Nếu bạn đã viết JavaScript một thời gian, bạn chắc chắn quen thuộc với method chaining: `[1, 2, 3].filter(...).map(...).reduce(...)`. Chuỗi này cũng đọc trái sang phải, cũng theo thứ tự thực thi — trông rất giống `pipe`. Vậy tại sao cần `pipe` khi đã có chaining?

Câu trả lời nằm ở một giới hạn cốt yếu: **method chaining chỉ hoạt động với methods THUỘC VỀ object đó**. Bạn có thể `.filter()` vì `filter` là method của `Array`. Bạn có thể `.toUpperCase()` vì đó là method của `String`. Nhưng hãy thử thêm `addTax(0.1)` — một custom function bạn tự viết — vào chuỗi `[100000, 200000].map(addTax(0.1)).???` — bạn sẽ bị kẹt. Không có cách nào "chain" một function tùy ý vào Array.

`pipe` không có giới hạn này. Nó nhận **bất kỳ function nào** — miễn output của function trước khớp input của function sau. Giống như dây chuyền sản xuất: bạn có thể lắp thêm bất kỳ trạm nào, của bất kỳ hãng nào, miễn băng chuyền khớp.

```typescript
// filename: src/chaining.ts
import assert from "node:assert/strict";

// Method chaining — built-in cho arrays
const result = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    .filter(n => n % 2 === 0)      // giữ chẵn: [2,4,6,8,10]
    .map(n => n * 3)                // nhân 3: [6,12,18,24,30]
    .reduce((sum, n) => sum + n, 0); // tổng: 90

assert.strictEqual(result, 90);

// Ưu điểm: đọc trái→phải (trên→dưới), dễ hiểu
// Nhược điểm: CHỈ hoạt động cho methods CỦA object đó
```

Đoạn code trên hoạt động hoàn hảo — vì `filter`, `map`, `reduce` đều là methods của `Array`. Nhưng ngay khi bạn muốn thêm một bước tính thuế và format sang tiền Việt — hai functions do BẠN viết, không phải methods của Array — chaining bất lực.

### Khi method chaining KHÔNG hoạt động

```typescript
// filename: src/chaining_limit.ts
import assert from "node:assert/strict";

// Custom function KHÔNG phải method của array
const addTax = (rate: number) => (amount: number): number =>
    Math.round(amount * (1 + rate));

const formatVND = (amount: number): string =>
    `${amount.toLocaleString()}đ`;

// ❌ Không thể chain custom functions!
// [100000, 200000].map(addTax(0.1)).formatVND()  // ❌ formatVND không phải array method!

// ✅ pipe giải quyết — mix array methods + custom functions
const total = pipe(
    [100000, 200000, 300000],
    (arr) => arr.map(addTax(0.1)),           // thêm thuế
    (arr) => arr.reduce((s, n) => s + n, 0), // tổng
    formatVND,                                // format
);

assert.strictEqual(total, "660,000đ");

console.log(total);
// Output: 660,000đ
```

Nhìn vào pipeline trên: bước 1 dùng `arr.map()` (method chaining bên trong), bước 2 dùng `arr.reduce()`, bước 3 dùng `formatVND` (custom function). `pipe` cho phép bạn **trộn tự do** giữa built-in methods và custom logic — mỗi bước chỉ cần trả ra giá trị phù hợp cho bước tiếp theo.

### Khi nào dùng cái nào?

| | Method chaining | `pipe` |
|---|---|---|
| Hoạt động với | Methods của object | **BẤT KỲ** function |
| Custom functions | ❌ | ✅ |
| Type safety | ✅ (IDE auto-complete) | ✅ (overloads) |
| Đọc | ✅ Trái→phải | ✅ Trái→phải |
| Dùng khi | Array ops thuần | **Mix** custom + built-in |

Trong thực tế, bạn sẽ bắt đầu với chaining cho những thao tác đơn giản trên array. Nhưng khi logic phức tạp lên — khi bạn cần thêm validation, format, logging, hoặc bất kỳ custom function nào — `pipe` sẽ trở thành công cụ chính. Hầu hết code FP production dùng `pipe`, vì business logic hiếm khi chỉ là `.filter().map().reduce()`.

> **💡 Kết hợp**: Dùng chaining cho array ops đơn giản. Dùng pipe khi cần custom functions. Trong FP nặng → pipe là chính.

---

## ✅ Checkpoint 12.3

> Đến đây bạn phải hiểu:
> 1. **Method chaining** = `.method().method()` — chỉ cho methods của object
> 2. **`pipe`** = bất kỳ function nào — linh hoạt hơn
> 3. Chaining cho array ops đơn giản. Pipe cho custom + mixed logic
>
> **Test nhanh**: `"hello".toUpperCase().split("")` — đây là chaining hay pipe?
> <details><summary>Đáp án</summary>**Method chaining**. `toUpperCase()` và `split()` là methods của String. pipe thì viết: `pipe("hello", s => s.toUpperCase(), s => s.split(""))`.</details>

---

## 12.4 — Point-Free Style

### "Chỉ vào trạm, đừng chỉ vào sản phẩm"

Quay lại dây chuyền sản xuất. Khi bạn mô tả quy trình cho đồng nghiệp, bạn nói: *"Bột đi qua trạm nhào, rồi trạm cán, rồi trạm cắt"* — bạn **gọi tên các trạm**, không cần mô tả từng cục bột di chuyển ra sao. Bạn tin rằng đồng nghiệp hiểu: bột vào trạm nhào → ra bột nhão → vào trạm cán → ra miếng dẹt.

Point-free style hoạt động y hệt: bạn **truyền tên function** thay vì viết arrow wrapper. Thay vì `arr.filter(n => isPositive(n))` — "filter, lấy từng n, gọi isPositive cho n" — bạn viết `arr.filter(isPositive)` — "filter bằng isPositive". Ngắn hơn, ít nhiễu hơn, và ý nghĩa rõ ràng hơn vì bạn đọc TÊN hành động thay vì cơ chế thực hiện.

Nhưng — và đây là cái bẫy mà mọi JavaScript developer gặp ít nhất một lần — point-free chỉ an toàn khi **function signature khớp** với callback signature. Nếu không khớp, kết quả sẽ sai một cách thầm lặng.

```typescript
// filename: src/point_free.ts
import assert from "node:assert/strict";

// ❌ Pointful — nhắc đến parameter x
const doubleAll1 = (arr: readonly number[]): readonly number[] =>
    arr.map(x => x * 2);

// ✅ Point-free — KHÔNG nhắc parameter
const double = (x: number): number => x * 2;
const doubleAll2 = (arr: readonly number[]): readonly number[] =>
    arr.map(double);

// Point-free = truyền function reference, không wrap trong arrow

// Ví dụ rõ hơn:
const isPositive = (n: number): boolean => n > 0;
const toString = (n: number): string => String(n);

// ❌ Pointful
const positives1 = [1, -2, 3, -4].filter(n => isPositive(n));
const strings1 = [1, 2, 3].map(n => toString(n));

// ✅ Point-free
const positives2 = [1, -2, 3, -4].filter(isPositive);
const strings2 = [1, 2, 3].map(toString);

assert.deepStrictEqual(positives2, [1, 3]);
assert.deepStrictEqual(strings2, ["1", "2", "3"]);

console.log("Point-free OK ✅");
```

`isPositive` nhận `(n: number) => boolean` — khớp hoàn hảo với callback mà `.filter()` mong đợi: `(value: number) => boolean`. Tương tự, `double` nhận `(x: number) => number` — khớp callback của `.map()`. Vì signature khớp, bạn có thể truyền trực tiếp mà không cần arrow wrapper.

Nhưng **đợi đã** — điều gì xảy ra khi signature KHÔNG khớp?

### ⚠️ Point-free trap: Map + parseInt

Đây là một trong những bug nổi tiếng nhất của JavaScript, và nó minh họa hoàn hảo tại sao point-free cần thận trọng. `.map()` truyền **ba** arguments cho callback: `(value, index, array)`. `parseInt` nhận **hai** arguments: `(string, radix)`. Khi bạn viết `.map(parseInt)`, JavaScript vui vẻ truyền `index` làm `radix` — và mọi thứ sụp đổ trong im lặng.

```typescript
// filename: src/point_free_trap.ts
import assert from "node:assert/strict";

// ❌ NGUY HIỂM! map truyền 3 args: (value, index, array)
// parseInt nhận 2 args: (string, radix)
// → radix = index = 0, 1, 2 → kết quả SAI!

const wrong = ["1", "2", "3"].map(parseInt);
// parseInt("1", 0) = 1  (radix 0 = default base 10)
// parseInt("2", 1) = NaN (base 1 không hợp lệ!)
// parseInt("3", 2) = NaN (base 2: "3" không hợp lệ!)

assert.deepStrictEqual(wrong, [1, NaN, NaN]);  // 😱

// ✅ Fix: wrap lại để control arguments
const correct = ["1", "2", "3"].map(s => parseInt(s, 10));
assert.deepStrictEqual(correct, [1, 2, 3]);

// ✅ Hoặc dùng Number()
const alsoCorrect = ["1", "2", "3"].map(Number);
assert.deepStrictEqual(alsoCorrect, [1, 2, 3]);

console.log("Point-free trap addressed ✅");
```

Bận có nhận ra điều gì lạ không? `Number` cũng nhận 1 argument, nhưng lại hoạt động đúng với `.map()`. Lý do: `Number(value)` bỏ qua thêm arguments, nên `Number("3", 2, arr)` vẫn cho `3`. Còn `parseInt("3", 2, arr)` đọc `2` làm base (hệ nhị phân) — và `"3"` không tồn tại trong hệ nhị phân!

Quy tắc đơn giản: **dùng point-free khi bạn chắc chắn function chỉ nhận đúng số arguments mà callback truyền**. Khi nghi ngờ, wrap lại bằng arrow. An toàn hơn là debug 2 tiếng tìm bug parseInt.

> **💡 Point-free khi an toàn**: Dùng khi function signature **KHỚP** params mà callback nhận. `isPositive: (n: number) => boolean` khớp với `.filter()` callback. `parseInt: (s: string, radix: number) => number` KHÔNG khớp — cần wrap.

---

## ✅ Checkpoint 12.4

> Đến đây bạn phải hiểu:
> 1. **Point-free** = truyền function reference, không wrap trong arrow
> 2. **An toàn** khi function signature khớp params (`.filter(isPositive)`)
> 3. **NGUY HIỂM** khi function nhận extra args (`.map(parseInt)` trap!)
> 4. **Quy tắc**: point-free khi rõ ràng, wrap khi nghi ngờ
>
> **Test nhanh**: `[true, false, true].filter(Boolean)` — point-free đúng không?
> <details><summary>Đáp án</summary>ĐÚNG! `Boolean` nhận 1 arg → trả truthy/falsy. `.filter` callback cần `(value) => boolean`. Signature khớp!</details>

---

## 12.5 — Data Pipelines Thực Tế

### Từ dây chuyền đơn giản đến nhà máy hoàn chỉnh

Cho đến giờ, các ví dụ của chúng ta còn đơn giản: nhân đôi, cộng 1, đổi dấu. Trong thực tế, dây chuyền sản xuất phần mềm phức tạp hơn nhiều. Một e-commerce system cần tính subtotal, áp dụng discount, tính thuế, format tiền — và mỗi bước đều có business rules riêng.

Nhưng — và đây là vẻ đẹp của composition — dù pipeline có dài bao nhiêu, nó vẫn chỉ là **chuỗi pure functions ghép lại**. Mỗi function nhận input, trả output, không mutation, không side effects. Bạn có thể test từng trạm riêng biệt, debug từng bước, và thêm/bỏ trạm mà không sợ phá vỡ trạm khác.

Hãy xem một pipeline xử lý đơn hàng thực tế. Nhà máy chúng ta có 4 trạm:
1. **Tính subtotal** — cộng giá tất cả sản phẩm
2. **Tính discount** — giảm 10% nếu đơn >= 1 triệu
3. **Tính thuế** — 8% VAT trên giá sau giảm
4. **Format** — chuyển thành chuỗi tiền VND

### E-commerce order processing

```typescript
// filename: src/data_pipeline.ts
import assert from "node:assert/strict";

// --- Types ---
type Product = {
    readonly name: string;
    readonly price: number;
    readonly category: "electronics" | "clothing" | "food";
};

type Order = {
    readonly id: string;
    readonly products: readonly Product[];
    readonly customerId: string;
};

type OrderSummary = {
    readonly orderId: string;
    readonly itemCount: number;
    readonly subtotal: number;
    readonly discount: number;
    readonly tax: number;
    readonly total: number;
    readonly formatted: string;
};

// --- Small pure functions (building blocks) ---
const subtotal = (products: readonly Product[]): number =>
    products.reduce((sum, p) => sum + p.price, 0);

const calculateDiscount = (amount: number, threshold: number, rate: number): number =>
    amount >= threshold ? Math.round(amount * rate) : 0;

const calculateTax = (amount: number, rate: number): number =>
    Math.round(amount * rate);

const formatVND = (amount: number): string =>
    `${amount.toLocaleString()}đ`;

// --- Pipeline: compose building blocks ---
const summarizeOrder = (order: Order): OrderSummary => {
    const sub = subtotal(order.products);
    const discount = calculateDiscount(sub, 1000000, 0.1); // 10% nếu >= 1M
    const afterDiscount = sub - discount;
    const tax = calculateTax(afterDiscount, 0.08);          // 8% VAT
    const total = afterDiscount + tax;

    return {
        orderId: order.id,
        itemCount: order.products.length,
        subtotal: sub,
        discount,
        tax,
        total,
        formatted: `Order ${order.id}: ${formatVND(total)}`,
    };
};

// --- Test ---
const order: Order = {
    id: "ORD-001",
    customerId: "C1",
    products: [
        { name: "Laptop", price: 15000000, category: "electronics" },
        { name: "Mouse", price: 500000, category: "electronics" },
        { name: "T-shirt", price: 300000, category: "clothing" },
    ],
};

const summary = summarizeOrder(order);

assert.strictEqual(summary.subtotal, 15800000);     // 15M + 500K + 300K
assert.strictEqual(summary.discount, 1580000);      // 15.8M × 10%
assert.strictEqual(summary.tax, 1137600);            // (15.8M - 1.58M) × 8% = 14.22M × 0.08
assert.strictEqual(summary.total, 15357600);         // 14.22M + 1.1376M

console.log(summary.formatted);
// Output: Order ORD-001: 15,357,600đ
```

Hãy chú ý cấu trúc của `summarizeOrder`: nó chỉ là chuỗi gọi các building blocks theo thứ tự, mỗi bước truyền kết quả cho bước tiếp. Đây chưa dùng `pipe` vì có branching logic (discount phụ thuộc threshold), nhưng **cách tư duy** vẫn là pipeline: data chảy qua từng trạm xử lý.

Và vì mỗi building block là pure function, bạn có thể test riêng: `subtotal([{price: 100}, {price: 200}])` phải trả `300`, `calculateDiscount(1000000, 1000000, 0.1)` phải trả `100000`. Pipeline dài nhưng debug đơn giản — trace từng bước cho đến khi tìm thấy trạm bị lỗi.

### Multi-order analytics pipeline

Giờ hãy xem pipeline ở quy mô lớn hơn — phân tích doanh số bán hàng từ nhiều đơn hàng. Đây là nơi `pipe` thực sự toả sáng, vì mỗi bước biến đổi data type hoàn toàn khác nhau: `Order[] → Order[] → Product[] → Map<string, Product[]> → Map<string, number>`.

```typescript
// filename: src/analytics_pipeline.ts
import assert from "node:assert/strict";

type Product = {
    readonly name: string;
    readonly price: number;
    readonly category: string;
};

type Order = {
    readonly id: string;
    readonly products: readonly Product[];
    readonly status: "completed" | "pending" | "cancelled";
};

// Building blocks
const isCompleted = (order: Order): boolean =>
    order.status === "completed";

const allProducts = (orders: readonly Order[]): readonly Product[] =>
    orders.flatMap(o => o.products);

const byCategory = (products: readonly Product[]): ReadonlyMap<string, readonly Product[]> => {
    const map = new Map<string, Product[]>();
    for (const p of products) {
        const existing = map.get(p.category) ?? [];
        map.set(p.category, [...existing, p]);
    }
    return map;
};

const categoryTotals = (grouped: ReadonlyMap<string, readonly Product[]>): ReadonlyMap<string, number> => {
    const result = new Map<string, number>();
    for (const [cat, products] of grouped) {
        result.set(cat, products.reduce((s, p) => s + p.price, 0));
    }
    return result;
};

// Pipeline
const analyzeSales = (orders: readonly Order[]): ReadonlyMap<string, number> =>
    pipe(
        orders,
        (o) => o.filter(isCompleted),
        allProducts,
        byCategory,
        categoryTotals,
    );

// Test
const orders: readonly Order[] = [
    {
        id: "O1", status: "completed",
        products: [
            { name: "Laptop", price: 20000000, category: "electronics" },
            { name: "Mouse", price: 500000, category: "electronics" },
        ],
    },
    {
        id: "O2", status: "completed",
        products: [
            { name: "T-shirt", price: 300000, category: "clothing" },
        ],
    },
    {
        id: "O3", status: "cancelled",  // bỏ qua!
        products: [
            { name: "Headphones", price: 2000000, category: "electronics" },
        ],
    },
];

const result = analyzeSales(orders);

assert.strictEqual(result.get("electronics"), 20500000);  // 20M + 500K (O1 only)
assert.strictEqual(result.get("clothing"), 300000);

console.log("Analytics pipeline OK ✅");
```

Đọc `analyzeSales` như đọc câu chuyện: *"Lấy danh sách orders → lọc completed → lấy tất cả products → nhóm theo category → tính tổng mỗi nhóm."* Năm dòng code, năm bước xử lý, không biến tạm, không mutation. Nếu ngày mai sếp yêu cầu "chỉ tính sản phẩm giá > 500K", bạn thêm MỘT dòng vào giữa pipeline — không cần sửa bất kỳ building block nào.

Đây chính là sức mạnh thực sự của composition: **pipeline = một câu chuyện mà mỗi chương (function) có thể viết, đọc, và test riêng biệt**.

> **💡 Pipeline = story**: Đọc pipeline như đọc câu chuyện: "lấy orders → lọc completed → lấy products → nhóm theo category → tính tổng mỗi nhóm". Mỗi step = 1 pure function. Dễ hiểu, dễ test, dễ sửa.

---

## ✅ Checkpoint 12.5

> Đến đây bạn phải hiểu:
> 1. **Pipeline = chuỗi pure functions** xử lý data từng bước
> 2. **Building blocks** = small pure functions, compose thành pipeline lớn
> 3. Pipeline **đọc như câu chuyện** — dễ hiểu flow
> 4. Mỗi step **testable riêng** — không cần test toàn bộ pipeline
>
> **Test nhanh**: Nếu muốn thêm "chỉ đếm orders > 500K" vào pipeline, thêm ở đâu?
> <details><summary>Đáp án</summary>Thêm 1 step filter sau `allProducts`: `(products) => products.filter(p => p.price > 500000)`. Pipeline = dễ mở rộng — chỉ thêm 1 step.</details>

---

## 12.6 — Builder Pattern FP: Compose Configuration

### Dây chuyền lắp ráp, không phải lò rèn

Trong OOP, Builder pattern thường đi kèm class, `this`, và method chaining: `new RequestBuilder().withUrl(...).withMethod(...).build()`. Đây là cách "lò rèn" — bạn tạo một object rỗng, rồi từng bước *nung* nó thành hình, mutate state bên trong cho đến khi hoàn thiện.

Trong FP, chúng ta thay lò rèn bằng **dây chuyền lắp ráp**: mỗi trạm nhận sản phẩm bán thành phẩm (immutable state), gắn thêm một phần (tạo object mới), rồi chuyển tiếp. Không mutation, không `this`, không class. Chỉ có functions và data.

Mỗi builder function có cùng một signature đẹp: `(state: Config) => Config`. Nhận config cũ, trả config mới. Và vì tất cả chia sẻ cùng signature, chúng tự nhiên composable qua `pipe` — bạn có thể ghép 3, 5, hay 20 builders theo bất kỳ thứ tự nào.

```typescript
// filename: src/fp_builder.ts
import assert from "node:assert/strict";

// --- Type ---
type HttpRequest = {
    readonly method: "GET" | "POST" | "PUT" | "DELETE";
    readonly url: string;
    readonly headers: Readonly<Record<string, string>>;
    readonly body: string | null;
    readonly timeout: number;
};

// --- Building blocks: mỗi function nhận config → trả config mới ---
type RequestBuilder = (req: HttpRequest) => HttpRequest;

const withMethod = (method: HttpRequest["method"]): RequestBuilder =>
    (req) => ({ ...req, method });

const withUrl = (url: string): RequestBuilder =>
    (req) => ({ ...req, url });

const withHeader = (key: string, value: string): RequestBuilder =>
    (req) => ({ ...req, headers: { ...req.headers, [key]: value } });

const withBody = (body: string): RequestBuilder =>
    (req) => ({ ...req, body, method: "POST" });

const withTimeout = (ms: number): RequestBuilder =>
    (req) => ({ ...req, timeout: ms });

const withAuth = (token: string): RequestBuilder =>
    withHeader("Authorization", `Bearer ${token}`);

const withJson = (data: unknown): RequestBuilder =>
    (req) => ({
        ...req,
        body: JSON.stringify(data),
        headers: { ...req.headers, "Content-Type": "application/json" },
    });

// --- Default config ---
const defaultRequest: HttpRequest = {
    method: "GET",
    url: "",
    headers: {},
    body: null,
    timeout: 5000,
};

// --- Build bằng pipe ---
const request = pipe(
    defaultRequest,
    withUrl("https://api.example.com/users"),
    withMethod("POST"),
    withAuth("my-token"),
    withJson({ name: "An", age: 25 }),
    withTimeout(10000),
);

assert.strictEqual(request.method, "POST");
assert.strictEqual(request.url, "https://api.example.com/users");
assert.strictEqual(request.headers["Authorization"], "Bearer my-token");
assert.strictEqual(request.headers["Content-Type"], "application/json");
assert.strictEqual(request.body, '{"name":"An","age":25}');
assert.strictEqual(request.timeout, 10000);

console.log(`${request.method} ${request.url}`);
// Output: POST https://api.example.com/users
```

Hãy chú ý `withAuth`: nó không tự implement logic — thay vào đó, nó **gọi** `withHeader`. Đây là composition bên trong: builders có thể được xây dựng từ builders khác, tạo thành cây abstraction levels. `withAuth` là high-level ("thêm authentication"), `withHeader` là low-level ("thêm một header bất kỳ"). Người dùng chỉ thấy `withAuth`, nhưng nếu cần custom, họ vẫn có `withHeader`.

Và pipeline `pipe(defaultRequest, withUrl(...), withMethod(...), ...)` đọc chính xác như một câu tiếng Anh: *"Bắt đầu từ default request, thêm URL, set method POST, gắn auth token, attach JSON body, đặt timeout 10 giây."* Không `new`, không `build()`, không `this`. Functional elegance.

> **💡 FP Builder = pipe + small functions**: Không cần class, constructor, hay `this`. Mỗi builder function nhận state cũ → trả state mới (immutable!). Compose chúng bằng pipe.

---

## ✅ Checkpoint 12.6

> Đến đây bạn phải hiểu:
> 1. **FP Builder** = `pipe(default, modifier1, modifier2, ...)` — thay OOP builder
> 2. Mỗi modifier: `(state) => newState` — immutable, composable
> 3. **Không class, không `this`** — chỉ functions + data
>
> **Test nhanh**: `withBody` tự động set `method: "POST"` — đây có phải pure function?
> <details><summary>Đáp án</summary>CÓ! Pure: cùng input → cùng output, không side effects. `withBody` chỉ tạo object mới với method="POST" — deterministic, no mutation.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Compose vs Pipe

Dùng cả compose và pipe để tính: "lấy số → nhân 3 → trừ 7 → lấy trị tuyệt đối":

```typescript
const triple = (x: number): number => x * 3;
const subtract7 = (x: number): number => x - 7;
const abs = (x: number): number => Math.abs(x);

// 1. Viết bằng compose
// 2. Viết bằng pipe
// 3. Test với input = 2: triple(2)=6, subtract7(6)=-1, abs(-1)=1
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

const triple = (x: number): number => x * 3;
const subtract7 = (x: number): number => x - 7;
const abs = (x: number): number => Math.abs(x);

// compose — đọc phải→trái
const withCompose = compose(abs, compose(subtract7, triple));
assert.strictEqual(withCompose(2), 1);  // triple(2)=6 → sub7=-1 → abs=1

// pipe — đọc trái→phải ✅
const withPipe = pipe(2, triple, subtract7, abs);
assert.strictEqual(withPipe, 1);

console.log(`Result: ${withPipe}`);
// Output: Result: 1
```

</details>

---

**Bài 2** (10 phút): Data pipeline

Viết pipeline xử lý danh sách students:

```typescript
type Student = {
    readonly name: string;
    readonly score: number;
    readonly grade: "A" | "B" | "C" | "D" | "F";
};

// Pipeline (dùng pipe):
// 1. Lọc students có score >= 50
// 2. Sắp xếp theo score giảm dần
// 3. Lấy top 3
// 4. Format: ["1. An (95)", "2. Bình (87)", ...]
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type Student = {
    readonly name: string;
    readonly score: number;
    readonly grade: "A" | "B" | "C" | "D" | "F";
};

const students: readonly Student[] = [
    { name: "An", score: 95, grade: "A" },
    { name: "Bình", score: 42, grade: "F" },
    { name: "Chi", score: 87, grade: "B" },
    { name: "Dũng", score: 73, grade: "C" },
    { name: "Em", score: 91, grade: "A" },
    { name: "Phúc", score: 55, grade: "D" },
];

const result = pipe(
    students,
    (s) => s.filter(st => st.score >= 50),
    (s) => [...s].sort((a, b) => b.score - a.score),
    (s) => s.slice(0, 3),
    (s) => s.map((st, i) => `${i + 1}. ${st.name} (${st.score})`),
);

assert.deepStrictEqual(result, [
    "1. An (95)",
    "2. Em (91)",
    "3. Chi (87)",
]);
```

</details>

---

**Bài 3** (15 phút): FP Builder

Viết query builder cho database queries:

```typescript
type Query = {
    readonly table: string;
    readonly conditions: readonly string[];
    readonly orderBy: string | null;
    readonly limit: number | null;
    readonly fields: readonly string[];
};

// 1. Default query: { table: "", conditions: [], orderBy: null, limit: null, fields: ["*"] }
// 2. Builders: from(table), where(condition), orderByField(field, dir), limitTo(n), select(...fields)
// 3. toSQL(query): string — convert to SQL string
// 4. Dùng pipe: pipe(defaultQuery, from("users"), where("age > 18"), select("name", "email"), limitTo(10))
//    → "SELECT name, email FROM users WHERE age > 18 LIMIT 10"
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Query = {
    readonly table: string;
    readonly conditions: readonly string[];
    readonly orderBy: string | null;
    readonly limit: number | null;
    readonly fields: readonly string[];
};

type QueryBuilder = (q: Query) => Query;

const defaultQuery: Query = {
    table: "",
    conditions: [],
    orderBy: null,
    limit: null,
    fields: ["*"],
};

const from = (table: string): QueryBuilder =>
    (q) => ({ ...q, table });

const where = (condition: string): QueryBuilder =>
    (q) => ({ ...q, conditions: [...q.conditions, condition] });

const orderByField = (field: string, dir: "ASC" | "DESC" = "ASC"): QueryBuilder =>
    (q) => ({ ...q, orderBy: `${field} ${dir}` });

const limitTo = (n: number): QueryBuilder =>
    (q) => ({ ...q, limit: n });

const select = (...fields: readonly string[]): QueryBuilder =>
    (q) => ({ ...q, fields });

const toSQL = (q: Query): string => {
    const parts = [`SELECT ${q.fields.join(", ")} FROM ${q.table}`];
    if (q.conditions.length > 0) parts.push(`WHERE ${q.conditions.join(" AND ")}`);
    if (q.orderBy) parts.push(`ORDER BY ${q.orderBy}`);
    if (q.limit !== null) parts.push(`LIMIT ${q.limit}`);
    return parts.join(" ");
};

const query = pipe(
    defaultQuery,
    from("users"),
    where("age > 18"),
    where("active = true"),
    select("name", "email"),
    orderByField("name"),
    limitTo(10),
);

const sql = toSQL(query);
assert.strictEqual(
    sql,
    "SELECT name, email FROM users WHERE age > 18 AND active = true ORDER BY name ASC LIMIT 10"
);

console.log(sql);
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `compose` type inference fail | Nested generics khó infer | Dùng `pipe` thay vì compose |
| pipe overloads không đủ | Quá 5 functions trong pipe | Tách thành nhiều pipe, hoặc dùng fp-ts pipe |
| Point-free `.map(parseInt)` bug | `parseInt` nhận 2 args, map truyền 3 | Wrap: `s => parseInt(s, 10)` hoặc dùng `Number` |
| Pipeline quá dài | Quá nhiều steps | Extract named functions cho groups of steps |
| Builder loses type | `as unknown` trong pipe | Dùng typed overloads, không `unknown` |

---

## Tóm tắt

Trong chương này, chúng ta đã đi dọc dây chuyền sản xuất — từ những trạm đơn lẻ (`compose`, `pipe`) đến hệ thống nhà máy hoàn chỉnh (data pipelines, FP builders). Hãy nhớ:

- ✅ **`compose(f, g)(x) = f(g(x))`** — phải→trái, giống toán học. Dùng khi tạo function mới.
- ✅ **`pipe(x, f, g, h) = h(g(f(x)))`** — trái→phải, tự nhiên. **Ưu tiên dùng** cho mọi data processing.
- ✅ **Method chaining** cho array ops đơn giản. **Pipe** cho custom functions và mixed logic.
- ✅ **Point-free** = truyền function reference thay vì arrow wrapper. Cẩn thận args mismatch (`parseInt` trap)!
- ✅ **Data pipelines** = chuỗi pure functions đọc như câu chuyện. Mỗi trạm testable riêng.
- ✅ **FP Builder** = `pipe(default, mod1, mod2, ...)`. Không class, không `this`, chỉ functions và data.

Bạn giờ đã có công cụ để ghép functions lại thành pipelines. Nhưng dây chuyền của bạn mới chỉ xử lý MỘT loại sản phẩm. Nếu sản phẩm có thể là laptop HOẶC điện thoại, giá tính theo cái HOẶC theo kg? Chương tiếp theo sẽ giới thiệu **Algebraic Data Types** — cách TypeScript biểu diễn "hoặc" và "và" — để dây chuyền xử lý được mọi loại sản phẩm.

## Tiếp theo

→ Chapter 13: **ADTs in TypeScript** — Discriminated unions deep dive, pattern matching, exhaustive DU handling, và real-world ADT hierarchies.
