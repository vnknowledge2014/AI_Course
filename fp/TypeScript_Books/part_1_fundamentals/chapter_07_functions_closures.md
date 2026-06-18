# Chapter 7 — Functions & Closures

> **Bạn sẽ học được**:
> - Arrow functions deep dive — function types, optional/default params
> - Generics `<T>` — viết functions hoạt động với MỌI type
> - Higher-Order Functions — functions nhận/trả functions
> - Closures — functions "nhớ" môi trường tạo ra chúng
> - Currying & Partial Application — chia nhỏ functions
> - Function composition — nối functions thành pipeline
>
> **Yêu cầu trước**: Chapter 6 (narrowing, discriminated unions, type guards).
> **Thời gian đọc**: ~45 phút | **Level**: Beginner → Intermediate
> **Kết quả cuối cùng**: Viết functions theo phong cách FP — pure, composable, generic.

---

## Functions & Closures — Building Blocks của FP

Trong TypeScript (và JavaScript), functions là first-class citizens — bạn truyền chúng như arguments, trả về từ functions khác, lưu vào variables. Arrow functions `(x) => x + 1` gọn gàng hơn `function(x) { return x + 1 }` và giữ `this` binding đúng.

Nhưng power thực sự đến từ **higher-order functions**: `.map()`, `.filter()`, `.reduce()` — ba HOFs mà bạn dùng hàng ngày. Thay vì `for` loops với index variables, bạn diễn đạt **ý định**: "biến mỗi item thành X" (map), "giữ items thỏa Y" (filter), "gom tất cả thành Z" (reduce).

Chapter này dạy bạn viết functions hiệu quả, dùng generics `<T>` cho type-safe reusability, và master closures — nền tảng cho mọi pattern FP.


## 7.1 — Arrow Functions: Nền tảng FP

### Nhắc lại và đi sâu

Chapter 0 đã giới thiệu arrow functions. Bây giờ tìm hiểu chi tiết:

```typescript
// filename: src/arrow_deep.ts
import assert from "node:assert/strict";

// === Cú pháp cơ bản ===

// Một parameter — không cần ()
const double = (x: number): number => x * 2;

// Nhiều parameters
const add = (a: number, b: number): number => a + b;

// Không parameter
const now = (): number => Date.now();

// Body nhiều dòng — cần { } và return
const clamp = (value: number, min: number, max: number): number => {
    if (value < min) return min;
    if (value > max) return max;
    return value;
};

assert.strictEqual(double(5), 10);
assert.strictEqual(add(3, 4), 7);
assert.strictEqual(clamp(15, 0, 10), 10);
assert.strictEqual(clamp(-5, 0, 10), 0);
assert.strictEqual(clamp(5, 0, 10), 5);

console.log("Arrow functions OK ✅");
```

### Function Types

Functions cũng có types — bạn có thể khai báo "shape" của một function:

```typescript
// filename: src/function_types.ts
import assert from "node:assert/strict";

// Khai báo function type
type Predicate<T> = (item: T) => boolean;
type Transform<A, B> = (input: A) => B;
type Reducer<T, R> = (acc: R, item: T) => R;

// Dùng function types
const isPositive: Predicate<number> = (n) => n > 0;
const toString: Transform<number, string> = (n) => n.toFixed(2);
const sum: Reducer<number, number> = (acc, n) => acc + n;

assert.strictEqual(isPositive(5), true);
assert.strictEqual(isPositive(-3), false);
assert.strictEqual(toString(3.14159), "3.14");
assert.strictEqual(sum(10, 5), 15);

// Function type = contract — đảm bảo function đúng "shape"
const applyPredicate = (pred: Predicate<number>, value: number): boolean =>
    pred(value);

assert.strictEqual(applyPredicate(isPositive, 42), true);

console.log("Function types OK ✅");
```

> **💡 Function types = contracts**: `Predicate<T>`, `Transform<A, B>`, `Reducer<T, R>` là **patterns bạn sẽ thấy lại** ở mọi chương sau. Đặt tên cho function shapes giúp code dễ đọc hơn.

### Optional & Default Parameters

```typescript
// filename: src/optional_params.ts
import assert from "node:assert/strict";

// Optional parameter — thêm ?
const greet = (name: string, greeting?: string): string =>
    `${greeting ?? "Hello"}, ${name}!`;

assert.strictEqual(greet("An"), "Hello, An!");
assert.strictEqual(greet("An", "Chào"), "Chào, An!");

// Default parameter — gán giá trị mặc định
const formatPrice = (amount: number, currency: string = "VND"): string =>
    `${amount.toLocaleString()} ${currency}`;

assert.strictEqual(formatPrice(50000), "50,000 VND");
assert.strictEqual(formatPrice(100, "USD"), "100 USD");

console.log(greet("An"));            // Hello, An!
console.log(formatPrice(50000));      // 50,000 VND
```

> **💡 Optional vs Default**: `greeting?: string` → type `string | undefined`. `currency = "VND"` → type `string` (compiler biết luôn có giá trị). Default rõ ràng hơn — ưu tiên default khi có thể.

---

## ✅ Checkpoint 7.1

> Đến đây bạn phải hiểu:
> 1. **Arrow functions** = `(params) => expression` hoặc `(params) => { ... return ... }`
> 2. **Function types**: `type Fn = (input: A) => B` — khai báo "shape" của function
> 3. **Optional** (`?`) vs **Default** (`= value`) parameters
> 4. `Predicate<T>`, `Transform<A, B>`, `Reducer<T, R>` = patterns phổ biến
>
> **Test nhanh**: `const f = (x: number, y?: number): number => x + (y ?? 0);` — `f(5)` trả gì?
> <details><summary>Đáp án</summary>`5`. `y` là `undefined`, `undefined ?? 0` = `0`, nên `5 + 0 = 5`.</details>

---

## 7.2 — Generics: Viết một lần, dùng mọi type

### Vấn đề: Lặp code cho mỗi type

```typescript
// ❌ Viết riêng cho mỗi type — lặp lại!
const firstNumber = (arr: readonly number[]): number | undefined => arr[0];
const firstString = (arr: readonly string[]): string | undefined => arr[0];
const firstBoolean = (arr: readonly boolean[]): boolean | undefined => arr[0];
// ... phải viết cho MỌI type?
```

### Generics — Type parameters

```typescript
// filename: src/generics_basic.ts
import assert from "node:assert/strict";

// ✅ Generic — viết 1 lần, hoạt động với MỌI type
const first = <T>(arr: readonly T[]): T | undefined => arr[0];

// TypeScript TỰ suy luận T từ argument
const a = first([1, 2, 3]);         // T = number → number | undefined
const b = first(["x", "y"]);        // T = string → string | undefined
const c = first<boolean>([true]);   // T = boolean (ghi rõ)

assert.strictEqual(a, 1);
assert.strictEqual(b, "x");
assert.strictEqual(c, true);

// Generic với 2 type parameters
const pair = <A, B>(a: A, b: B): readonly [A, B] => [a, b] as const;

const p = pair("name", 42);  // readonly ["name", 42]
assert.deepStrictEqual(p, ["name", 42]);

console.log("Generics OK ✅");
```

> **💡 `<T>` = "type placeholder"**: Giống biến, nhưng cho types. Khi gọi `first([1,2,3])`, TypeScript thay `T` bằng `number`. Bạn viết logic 1 lần — compiler "nhân bản" cho mỗi type.

### Generic constraints

Đôi khi bạn cần `T` phải có một số properties:

```typescript
// filename: src/generic_constraints.ts
import assert from "node:assert/strict";

// ❌ T quá rộng — không biết có .length
// const getLength = <T>(x: T): number => x.length;

// ✅ Constraint: T phải có property length
const getLength = <T extends { readonly length: number }>(x: T): number =>
    x.length;

assert.strictEqual(getLength("hello"), 5);         // string có length
assert.strictEqual(getLength([1, 2, 3]), 3);        // array có length
// getLength(42);  // ❌ Error: number không có length

// Constraint với union
type HasId = { readonly id: string };
type HasName = { readonly name: string };

const identify = <T extends HasId & HasName>(item: T): string =>
    `${item.id}: ${item.name}`;

assert.strictEqual(
    identify({ id: "U1", name: "An", age: 25 }),  // extra field OK!
    "U1: An"
);

console.log("Constraints OK ✅");
```

> **💡 `extends` = "must have at least"**: `T extends { length: number }` = "T must have at least a `length` property of type number". `T` có thể có nhiều fields hơn — đây là structural typing.

### Generic type aliases

```typescript
// filename: src/generic_types.ts

// Result type — dùng ở mọi nơi trong sách
type Result<T> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: string };

// Option type — giá trị có thể không tồn tại
type Option<T> =
    | { readonly tag: "some"; readonly value: T }
    | { readonly tag: "none" };

// Pair
type Pair<A, B> = {
    readonly first: A;
    readonly second: B;
};

// Dùng
const success: Result<number> = { tag: "ok", value: 42 };
const failure: Result<number> = { tag: "err", error: "Not found" };
const some: Option<string> = { tag: "some", value: "hello" };
const none: Option<string> = { tag: "none" };
```

> **💡 `Result<T>` và `Option<T>`**: Hai generic types quan trọng nhất trong FP. `Result` = thay thế try/catch. `Option` = thay thế null. Chúng ta sẽ xây dựng chúng đầy đủ ở Part II.

---

## ✅ Checkpoint 7.2

> Đến đây bạn phải hiểu:
> 1. **`<T>`** = type parameter. Viết 1 lần, hoạt động với mọi type
> 2. **`T extends X`** = constraint — T phải "có ít nhất" properties của X
> 3. TypeScript tự suy luận `T` từ arguments — thường không cần ghi `<number>`
> 4. **`Result<T>`** và **`Option<T>`** = FP alternatives cho try/catch và null
>
> **Test nhanh**: `const f = <T>(x: T): T => x;` — `f(42)` trả type gì?
> <details><summary>Đáp án</summary>Type `42` (literal type!). TypeScript infer T = `42`, nên return type = `42`. Generic giữ nguyên type chính xác.</details>

---

## 7.3 — Higher-Order Functions: Functions nhận functions

### Ẩn dụ: Quán cà phê với menu tùy chỉnh

Tưởng tượng quán cà phê cho phép bạn mang **công thức riêng**. Quán cung cấp nguyên liệu (data), bạn mang công thức (function) — quán pha theo công thức bạn. Đó là Higher-Order Function (HOF): function nhận function khác làm tham số.

### `.map()`, `.filter()`, `.reduce()` — HOF hàng ngày

Bạn đã dùng chúng từ Chapter 2. Bây giờ hiểu chúng **là gì**:

```typescript
// filename: src/hof_basics.ts
import assert from "node:assert/strict";

const numbers: readonly number[] = [1, 2, 3, 4, 5];

// map — nhận function (number => number), áp dụng cho mỗi phần tử
const doubled = numbers.map(x => x * 2);
assert.deepStrictEqual(doubled, [2, 4, 6, 8, 10]);

// filter — nhận function (number => boolean), giữ phần tử thỏa mãn
const evens = numbers.filter(x => x % 2 === 0);
assert.deepStrictEqual(evens, [2, 4]);

// reduce — nhận function ((acc, item) => acc), gộp tất cả thành 1 giá trị
const total = numbers.reduce((sum, x) => sum + x, 0);
assert.strictEqual(total, 15);

// Chúng đều là HOFs — nhận function làm tham số
console.log("HOFs OK ✅");
```

### Viết HOF của riêng bạn

```typescript
// filename: src/custom_hof.ts
import assert from "node:assert/strict";

// HOF: nhận function, trả function mới
const repeat = <T>(fn: (input: T) => T, times: number) =>
    (input: T): T => {
        // Mutation nội bộ (let + for) — OK vì function tổng thể vẫn pure
        // (giống BFS pattern ở Chapter 3)
        let result = input;
        for (let i = 0; i < times; i++) {
            result = fn(result);
        }
        return result;
    };

const doubleOnce = (x: number): number => x * 2;
const doubleTwice = repeat(doubleOnce, 2);   // áp dụng 2 lần
const tripleApply = repeat(doubleOnce, 3);   // áp dụng 3 lần

assert.strictEqual(doubleTwice(5), 20);   // 5 → 10 → 20
assert.strictEqual(tripleApply(5), 40);   // 5 → 10 → 20 → 40

// HOF: strategy pattern
type SortOrder = "asc" | "desc";

const sortBy = <T>(key: (item: T) => number, order: SortOrder = "asc") =>
    (a: T, b: T): number => {
        const diff = key(a) - key(b);
        return order === "asc" ? diff : -diff;
    };

type Product = { readonly name: string; readonly price: number };

const products: readonly Product[] = [
    { name: "Laptop", price: 20000000 },
    { name: "Phone", price: 8000000 },
    { name: "Tablet", price: 12000000 },
];

const byPriceAsc = [...products].sort(sortBy(p => p.price, "asc"));
const byPriceDesc = [...products].sort(sortBy(p => p.price, "desc"));

assert.strictEqual(byPriceAsc[0]!.name, "Phone");
assert.strictEqual(byPriceDesc[0]!.name, "Laptop");

console.log("Custom HOFs OK ✅");
```

> **💡 HOF = Strategy Pattern tự nhiên**: Thay vì class + interface (OOP), FP truyền function trực tiếp. `sortBy` nhận **strategy** (cách lấy key) và trả **comparator** phù hợp. Đơn giản hơn class hierarchy.

---

## ✅ Checkpoint 7.3

> Đến đây bạn phải hiểu:
> 1. **HOF** = function nhận hoặc trả function. `.map()`, `.filter()`, `.reduce()` là HOFs
> 2. **Viết HOF**: `(fn) => (input) => ...` — nhận function, trả function mới
> 3. HOF = **strategy pattern** tự nhiên trong FP
>
> **Test nhanh**: `const apply = (f: (x: number) => number, x: number) => f(x);` — `apply` có phải HOF không?
> <details><summary>Đáp án</summary>CÓ! `apply` nhận function `f` làm tham số → đó là HOF. HOF = bất kỳ function nào nhận hoặc trả function khác.</details>

---

## 7.4 — Closures: Functions "nhớ" môi trường

### Câu chuyện: Phiếu giảm giá

Bạn nhận phiếu giảm giá 20% từ quán cà phê. Phiếu này **nhớ** mức giảm (20%) dù bạn mang đi đâu. Closure hoạt động giống vậy — function "nhớ" biến từ scope tạo ra nó.

```typescript
// filename: src/closure.ts
import assert from "node:assert/strict";

// createDiscount TRẢ function mới
// Function mới "nhớ" percent — dù createDiscount đã chạy xong
const createDiscount = (percent: number) =>
    (price: number): number =>
        Math.round(price * (1 - percent / 100));

const discount20 = createDiscount(20);   // "nhớ" percent = 20
const discount50 = createDiscount(50);   // "nhớ" percent = 50

assert.strictEqual(discount20(100000), 80000);   // 100000 × 0.8
assert.strictEqual(discount50(100000), 50000);   // 100000 × 0.5

// discount20 và discount50 là KHÁC NHAU — mỗi cái "nhớ" percent riêng
console.log(`20% off 100,000: ${discount20(100000).toLocaleString()}đ`);
console.log(`50% off 100,000: ${discount50(100000).toLocaleString()}đ`);
// Output:
// 20% off 100,000: 80,000đ
// 50% off 100,000: 50,000đ
```

### Closure = Scope capture

```typescript
// filename: src/closure_scope.ts
import assert from "node:assert/strict";

// Factory pattern với closure
const createCounter = (start: number = 0) => {
    let count = start;  // biến "captured" bởi closures bên dưới
    return {
        increment: () => { count += 1; return count; },
        decrement: () => { count -= 1; return count; },
        value: () => count,
    } as const;
};

const counter = createCounter(10);
assert.strictEqual(counter.value(), 10);
assert.strictEqual(counter.increment(), 11);
assert.strictEqual(counter.increment(), 12);
assert.strictEqual(counter.decrement(), 11);

// ⚠️ Đây có mutation (let count) — counter KHÔNG pure
// Trong FP, closure thường dùng cho READ-ONLY captured values
// (như createDiscount — percent không bao giờ thay đổi)
```

> **💡 Closure trong FP**: Closure mạnh nhất khi captured values là **readonly**. `createDiscount` capture `percent` (không đổi) = pure. `createCounter` capture `count` (mutable) = impure. Ưu tiên pattern đầu.

---

## ✅ Checkpoint 7.4

> Đến đây bạn phải hiểu:
> 1. **Closure** = function "nhớ" biến từ scope tạo ra nó
> 2. Captured values **tồn tại** dù scope gốc đã kết thúc
> 3. FP ưu tiên closures với **readonly** captured values (pure)
> 4. **Factory pattern** = function trả object với closures — thay thế classes
>
> **Test nhanh**: `const make = (x: number) => () => x;` — `make(42)()` trả gì?
> <details><summary>Đáp án</summary>`42`. `make(42)` trả function `() => x` với `x = 42` captured. Gọi function đó trả `42`.</details>

---

## 7.5 — Currying & Partial Application

### Currying: Functions từng bước

Currying = biến function nhiều tham số thành chuỗi functions 1 tham số:

```typescript
// filename: src/currying.ts
import assert from "node:assert/strict";

// ❌ Nhiều tham số — phải cung cấp tất cả cùng lúc
const addNormal = (a: number, b: number): number => a + b;

// ✅ Curried — cung cấp từng bước
const addCurried = (a: number) => (b: number): number => a + b;

// Dùng đầy đủ
assert.strictEqual(addCurried(3)(4), 7);

// Partial application — "cố định" 1 tham số
const add5 = addCurried(5);         // "nhớ" a = 5
assert.strictEqual(add5(10), 15);   // 5 + 10
assert.strictEqual(add5(20), 25);   // 5 + 20

// Thực tế: tạo specialized functions
const multiply = (factor: number) => (value: number): number =>
    factor * value;

const double = multiply(2);
const triple = multiply(3);
const toPercent = multiply(100);

assert.strictEqual(double(5), 10);
assert.strictEqual(triple(5), 15);
assert.strictEqual(toPercent(0.75), 75);

console.log("Currying OK ✅");
```

### Currying + `.map()` = sạch đẹp

```typescript
// filename: src/curry_map.ts
import assert from "node:assert/strict";

const multiply = (factor: number) => (value: number): number =>
    factor * value;

const addTax = (rate: number) => (price: number): number =>
    Math.round(price * (1 + rate));

const formatVND = (amount: number): string =>
    `${amount.toLocaleString()}đ`;

// Pipeline sạch đẹp — mỗi step là 1 function
const prices = [10000, 20000, 35000];

const withTax = prices.map(addTax(0.1));          // +10% VAT
const doubled = prices.map(multiply(2));           // ×2
const formatted = withTax.map(formatVND);          // format

assert.deepStrictEqual(withTax, [11000, 22000, 38500]);
assert.deepStrictEqual(doubled, [20000, 40000, 70000]);

console.log(formatted);
// Output: [ '11,000đ', '22,000đ', '38,500đ' ]
```

> **💡 Currying + HOF = FP idiom**: `prices.map(multiply(2))` đọc như tiếng Anh: "map each price by multiplying with 2". Currying tạo functions chuyên biệt từ functions tổng quát — rồi truyền vào HOFs.

---

## ✅ Checkpoint 7.5

> Đến đây bạn phải hiểu:
> 1. **Currying** = `(a, b) => ...` → `(a) => (b) => ...`. Cung cấp tham số từng bước
> 2. **Partial application** = gọi curried function với 1 phần tham số → function mới
> 3. **Currying + `.map()`** = code sạch, đọc như tiếng Anh
> 4. Pattern: `multiply(2)`, `addTax(0.1)` = specialized functions
>
> **Test nhanh**: `const f = (a: string) => (b: string) => a + b; f("Hello ")("World")` — kết quả?
> <details><summary>Đáp án</summary>`"Hello World"`. `f("Hello ")` trả `(b) => "Hello " + b`. Gọi tiếp với `"World"` → `"Hello World"`.</details>

---

## 7.6 — Function Composition: Nối functions

### Ẩn dụ: Dây chuyền sản xuất

Nhà máy có 3 máy nối nhau: Máy 1 (cắt) → Máy 2 (uốn) → Máy 3 (sơn). Nguyên liệu đi qua từng máy, mỗi máy biến đổi 1 bước. Composition = nối machines thành pipeline.

### `pipe` — Trái qua phải

```typescript
// filename: src/pipe.ts
import assert from "node:assert/strict";

// pipe: áp dụng functions từ trái qua phải
// pipe(x, f, g, h) → áp dụng f trước, rồi g, rồi h → kết quả: h(g(f(x)))
const pipe = <A, B>(a: A, f: (a: A) => B): B => f(a);
const pipe2 = <A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C => g(f(a));
const pipe3 = <A, B, C, D>(
    a: A,
    f: (a: A) => B,
    g: (b: B) => C,
    h: (c: C) => D
): D => h(g(f(a)));

// Dùng
const result = pipe3(
    "  Hello World  ",
    (s: string) => s.trim(),        // "Hello World"
    (s: string) => s.toLowerCase(), // "hello world"
    (s: string) => s.split(" "),    // ["hello", "world"]
);

assert.deepStrictEqual(result, ["hello", "world"]);

// Đọc từ TRÊN xuống DƯỚI — dễ hiểu flow
console.log(result);
// Output: [ 'hello', 'world' ]
```

### Pipeline thực tế: Xử lý dữ liệu

```typescript
// filename: src/pipeline_real.ts
import assert from "node:assert/strict";

type Product = {
    readonly name: string;
    readonly price: number;
    readonly category: string;
    readonly inStock: boolean;
};

const products: readonly Product[] = [
    { name: "MacBook", price: 30000000, category: "laptop", inStock: true },
    { name: "iPhone", price: 25000000, category: "phone", inStock: false },
    { name: "iPad", price: 15000000, category: "tablet", inStock: true },
    { name: "ThinkPad", price: 20000000, category: "laptop", inStock: true },
    { name: "Pixel", price: 18000000, category: "phone", inStock: true },
];

// Pipeline: lọc → transform → tổng hợp
const inStockLaptopTotal = products
    .filter(p => p.inStock)                    // chỉ còn hàng
    .filter(p => p.category === "laptop")      // chỉ laptop
    .map(p => p.price)                         // lấy giá
    .reduce((sum, price) => sum + price, 0);   // tổng

assert.strictEqual(inStockLaptopTotal, 50000000);  // 30M + 20M

// Readable hơn — tách thành named functions
const isInStock = (p: Product): boolean => p.inStock;
const isCategory = (cat: string) => (p: Product): boolean => p.category === cat;
const getPrice = (p: Product): number => p.price;
const sum = (a: number, b: number): number => a + b;

const result = products
    .filter(isInStock)
    .filter(isCategory("laptop"))
    .map(getPrice)
    .reduce(sum, 0);

assert.strictEqual(result, 50000000);

console.log(`In-stock laptops total: ${result.toLocaleString()}đ`);
// Output: In-stock laptops total: 50,000,000đ
```

> **💡 Pipeline = FP workflow**: Mỗi step là pure function. Data chảy từ trên xuống dưới, mỗi step biến đổi 1 bước. Không mutation, không side effects, dễ test từng step riêng.

---

## ✅ Checkpoint 7.6

> Đến đây bạn phải hiểu:
> 1. **Composition** = nối functions: output của f = input của g
> 2. **`pipe`** = trái qua phải. Đọc tự nhiên từ trên xuống
> 3. **Pipeline** (`.filter().map().reduce()`) = composition trên arrays
> 4. Tách thành **named functions** = code đọc như tiếng Anh
>
> **Test nhanh**: `[1,2,3,4,5].filter(x => x > 2).map(x => x * 10).reduce((s,x) => s+x, 0)` — kết quả?
> <details><summary>Đáp án</summary>`[3,4,5]` → `[30,40,50]` → `30+40+50 = 120`.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Generic utility

Viết generic function `mapOption` — áp dụng function cho `Option<T>`:

```typescript
type Option<T> =
    | { readonly tag: "some"; readonly value: T }
    | { readonly tag: "none" };

// mapOption(some(5), x => x * 2) → some(10)
// mapOption(none, x => x * 2) → none
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type Option<T> =
    | { readonly tag: "some"; readonly value: T }
    | { readonly tag: "none" };

const some = <T>(value: T): Option<T> => ({ tag: "some", value });
const none: Option<never> = { tag: "none" };
// Option<never> gán được cho Option<T> với mọi T — vì never là subtype của mọi type

const mapOption = <A, B>(opt: Option<A>, fn: (value: A) => B): Option<B> => {
    switch (opt.tag) {
        case "some": return some(fn(opt.value));
        case "none": return none;
    }
};

assert.deepStrictEqual(mapOption(some(5), x => x * 2), some(10));
assert.deepStrictEqual(mapOption(none, x => x * 2), none);
```

</details>

---

**Bài 2** (10 phút): Currying + pipeline

Viết hệ thống format text với curried functions:

```typescript
// 1. prefix(pre: string) => (text: string) => string — thêm prefix
// 2. suffix(suf: string) => (text: string) => string — thêm suffix
// 3. wrap(left: string, right: string) => (text: string) => string — bao quanh

// Pipeline: ["hello", "world"]
//   .map(prefix("👋 "))
//   .map(suffix("!"))
//   .map(wrap("[", "]"))
// → ["[👋 hello!]", "[👋 world!]"]
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

const prefix = (pre: string) => (text: string): string => pre + text;
const suffix = (suf: string) => (text: string): string => text + suf;
const wrap = (left: string, right: string) => (text: string): string =>
    left + text + right;

const result = ["hello", "world"]
    .map(prefix("👋 "))
    .map(suffix("!"))
    .map(wrap("[", "]"));

assert.deepStrictEqual(result, ["[👋 hello!]", "[👋 world!]"]);

console.log(result);
// Output: [ '[👋 hello!]', '[👋 world!]' ]
```

</details>

---

**Bài 3** (15 phút): Data processing pipeline

Viết pipeline xử lý danh sách transactions:

```typescript
type Transaction = {
    readonly id: string;
    readonly amount: number;
    readonly type: "income" | "expense";
    readonly category: string;
};

// 1. Curried filter: byType(type) => (t) => boolean
// 2. Curried filter: minAmount(min) => (t) => boolean
// 3. groupByCategory(transactions) → Map<string, readonly Transaction[]>
// 4. Pipeline: lọc expenses > 100000, group by category, tính tổng mỗi category

const transactions: readonly Transaction[] = [
    { id: "T1", amount: 50000, type: "expense", category: "food" },
    { id: "T2", amount: 200000, type: "expense", category: "tech" },
    { id: "T3", amount: 150000, type: "expense", category: "food" },
    { id: "T4", amount: 5000000, type: "income", category: "salary" },
    { id: "T5", amount: 300000, type: "expense", category: "tech" },
];

// Expected: Map { "tech" => 500000, "food" => 150000 }
```

<details><summary>💡 Gợi ý</summary>

- `byType`: `(type) => (t) => t.type === type`
- `minAmount`: `(min) => (t) => t.amount >= min`
- `groupByCategory`: `.reduce()` với `Map` accumulator

</details>

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Transaction = {
    readonly id: string;
    readonly amount: number;
    readonly type: "income" | "expense";
    readonly category: string;
};

// Curried filters
const byType = (type: Transaction["type"]) =>
    (t: Transaction): boolean => t.type === type;

const minAmount = (min: number) =>
    (t: Transaction): boolean => t.amount >= min;

// Group by category
const groupByCategory = (
    transactions: readonly Transaction[]
): ReadonlyMap<string, number> => {
    const map = new Map<string, number>();
    for (const t of transactions) {
        map.set(t.category, (map.get(t.category) ?? 0) + t.amount);
    }
    return map;
};

const transactions: readonly Transaction[] = [
    { id: "T1", amount: 50000, type: "expense", category: "food" },
    { id: "T2", amount: 200000, type: "expense", category: "tech" },
    { id: "T3", amount: 150000, type: "expense", category: "food" },
    { id: "T4", amount: 5000000, type: "income", category: "salary" },
    { id: "T5", amount: 300000, type: "expense", category: "tech" },
];

// Pipeline
const expenseSummary = groupByCategory(
    transactions
        .filter(byType("expense"))
        .filter(minAmount(100000))
);

assert.strictEqual(expenseSummary.get("tech"), 500000);   // 200K + 300K
assert.strictEqual(expenseSummary.get("food"), 150000);   // 150K only (50K < 100K)
assert.strictEqual(expenseSummary.has("salary"), false);   // income, not expense

console.log("Expense summary:", Object.fromEntries(expenseSummary));
// Output: Expense summary: { tech: 500000, food: 150000 }
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Generic type không suy luận đúng | TypeScript cần thêm context | Ghi rõ `<number>` khi gọi, hoặc thêm type annotation |
| Curried function trả `any` | Thiếu return type annotation | Ghi return type cho inner function: `(a) => (b): ReturnType => ...` |
| `.filter()` không thu hẹp type | Callback thiếu type guard return | Dùng `(x): x is T => ...` thay vì `(x) => ...` (Chapter 6) |
| Closure capture sai giá trị | `let` trong loop — classic JS trap | Dùng `const`, `.map()`, hoặc IIFE |
| Generic constraint quá chặt | `T extends` yêu cầu quá nhiều properties | Chỉ constraint properties thật sự cần |

---

## Tóm tắt

- ✅ **Function types** (`Predicate<T>`, `Transform<A,B>`) = khai báo "shape" của functions.
- ✅ **Generics `<T>`** = viết 1 lần, hoạt động mọi type. `extends` = constraint.
- ✅ **HOF** = functions nhận/trả functions. `.map()`, `.filter()`, `.reduce()` = HOFs hàng ngày.
- ✅ **Closures** = functions "nhớ" scope. FP ưu tiên readonly captured values.
- ✅ **Currying** = `(a, b) => ...` → `(a) => (b) => ...`. Partial application tạo specialized functions.
- ✅ **Composition/Pipeline** = nối functions. `.filter().map().reduce()` = FP workflow.

## Tiếp theo

→ Chapter 8: **Objects, Interfaces & Classes** — Object types, interfaces vs type aliases, structural typing, `readonly` objects, và khi nào (hiếm khi) dùng classes trong FP.
