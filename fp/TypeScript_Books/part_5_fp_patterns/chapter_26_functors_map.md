# Chapter 26 — Functors & Map

> **Bạn sẽ học được**:
> - Functor là gì — THỰC SỰ, không phải toán trừu tượng
> - `map` pattern thống nhất: Array, Option, Either, Task — ĐỀU là Functor
> - Functor laws — 2 luật đơn giản đảm bảo `map` hoạt động đúng
> - Lifting functions — biến `(A) => B` thành `(F<A>) => F<B>`
> - Nested functors — `map` bên trong `map`
>
> **Yêu cầu trước**: Chapter 25 (fp-ts overview, Option, Either, Task).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Nhìn thấy `map` pattern KHẮP NƠI — và hiểu tại sao nó hoạt động.

---

Bạn biết ống kính máy ảnh không?

Ống kính (lens) cho phép bạn nhìn vào BÊN TRONG một thứ gì đó mà KHÔNG cần mở nó ra. Bạn không tháo máy ảnh để xem ảnh — ống kính "map" ánh sáng từ bên ngoài vào sensor bên trong. Ống kính KHÔNG thay đổi cấu trúc máy ảnh — nó chỉ transform nội dung bên trong.

**Functor** chính xác là ống kính cho containers. `Array`, `Option`, `Either`, `Task` — đều là containers (hộp chứa giá trị). `map` = ống kính: nhìn vào bên trong container, transform giá trị, giữ nguyên cấu trúc container. `[1,2,3].map(x => x*2)` = array VẪN là array (3 phần tử), nội dung thay đổi (2,4,6). `Option.map(some(5), x => x*2)` = vẫn Some, giá trị 10.

Nếu bạn mở hộp quà, thay đổi nội dung bên trong (đổi sách thành bánh), rồi đóng lại — cái hộp vẫn là cái hộp. Màu sắc, kích thước, ruy-băng — không đổi. Chỉ **nội dung** thay đổi. Đó là `map`: mở container, transform nội dung, đóng lại. Container intact.

Tại sao pattern này quan trọng? Vì nó xuất hiện KHẮP NƠI trong code. Mỗi lần bạn gọi `.map()` trên array, `.then()` trên Promise, hay `map` trên Option/Either — bạn đang dùng MỘT pattern duy nhất. Hiểu pattern = hiểu TẤT CẢ. Đó là sức mạnh của abstraction: học một lần, áp dụng vạn lần.

Chương này sẽ đưa bạn từ "đã dùng map" (unconsciously competent) sang "HIỂU tại sao map hoạt động và phải tuân luật gì" (consciously competent). Sự khác biệt giữa thợ mộc biết đóng đinh và thợ mộc hiểu lực học — cả hai đóng được đinh, nhưng người thứ hai biết KHI NÀO đinh sẽ gãy.

---

## Functors & Map — Transform giá trị trong container

`map` chỉ làm MỘT viỆc: apply function cho giá trị BÊN TRONG container, không đụng đến container. `Array.map`, `Option.map`, `Either.map`, `Promise.then` — tất cả là Functor. Biết pattern này = biết CÁCH ĐỌc code FP.


## 26.1 — The Gift Box Model

### Bạn đã dùng Functor mỗi ngày

Đây là điều thú vị nhất về Functors: bạn đã dùng chúng HÀNG NGÀY mà không biết tên. Mỗi lần viết `[1,2,3].map(x => x*2)`, bạn đang dùng Array functor. Mỗi lần viết `promise.then(data => data.name)`, bạn đang dùng Promise functor (gần đúng). Functor không phải khái niệm mới — nó là TÊN CHO THỨ BẠN ĐÃ BIẾT.

Tại sao đặt tên? Cùng lý do bạn gọi "xe" thay vì "thứ có 4 bánh, động cơ, chạy trên đường". Có tên = nói chuyện dễ hơn, suy nghĩ rõ hơn, code tổ chức tốt hơn.

Hãy xem functor pattern xuất hiện ở đâu trong code bạn viết hàng ngày:

```typescript
// filename: src/functor_everywhere.ts
import assert from "node:assert/strict";

// Array = Functor
const nums = [1, 2, 3];
const doubled = nums.map(x => x * 2);
assert.deepStrictEqual(doubled, [2, 4, 6]);
// Array.map: transform elements, keep structure (still array, still 3 elements)

// Promise = Functor (sort of)
const p = Promise.resolve(5).then(x => x * 2);
// Promise.then with non-Promise return = map

// --- Nhưng Option, Either cũng là Functor! ---

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

const optionMap = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

// Option.map: transform value inside, keep structure
const maybeFive: Option<number> = some(5);
const maybeTen = optionMap((x: number) => x * 2)(maybeFive);
assert.deepStrictEqual(maybeTen, some(10));

const nothing: Option<number> = none;
const stillNothing = optionMap((x: number) => x * 2)(nothing);
assert.deepStrictEqual(stillNothing, none);
// None is still None after map — structure preserved!

// Either = Functor (maps Right, ignores Left)
type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

const eitherMap = <A, B>(fn: (a: A) => B) => <E>(fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : right(fn(fa.right));

const good = eitherMap((x: number) => x * 2)(right(21));
assert.deepStrictEqual(good, right(42));

const bad = eitherMap((x: number) => x * 2)(left("error"));
assert.deepStrictEqual(bad, left("error"));
// Left is still Left — structure preserved!

console.log("Functor everywhere OK ✅");
```

Đọc lại code trên: ba containers KHÁC NHAU (Array, Option, Either) nhưng `map` ĐỀU hoạt động cùng cách — transform nội dung, giữ cấu trúc. `None` vẫn `None` sau map. `Left` vẫn `Left` sau map. Empty array vẫn empty sau map. Pattern THỐNG NHẤT.

Ở Ch13 bạn đã viết `Result` với `map`. Ở Ch25 bạn thấy fp-ts dùng `Either` với `map`. Giờ bạn thấy: tất cả đều là Functor. Tên khác, concept GIỐNG.

### Definition: Functor = Type F + map function

Bây giờ chính thức hóa. Functor yêu cầu đúng 2 thứ: một type constructor (container) và một function `map`. Không hơn, không kém.

```typescript
// filename: src/functor_definition.ts

// Functor yêu cầu:
// 1. Một type constructor F<A> (container for A)
// 2. Một function: map :: (A → B) → F<A> → F<B>

// Nói bằng TypeScript:
// type Functor<F> = {
//     map: <A, B>(fn: (a: A) => B) => (fa: F<A>) => F<B>;
// };

// Array: F = Array, map = Array.map
// Option: F = Option, map = optionMap
// Either: F = Either<E, _>, map = eitherMap (maps Right)
// Task: F = Task, map = taskMap
// Promise: F = Promise, map = .then() (kinda)

console.log("Functor definition OK ✅");
```

Điểm tinh tế: Functor interface trong TypeScript không thể biểu diễn hoàn hảo (higher-kinded types — TypeScript chưa hỗ trợ). fp-ts giải quyết bằng URI encoding. Effect giải quyết bằng pipe + method chaining. Nhưng CONCEPT đằng sau giống nhau: "cho tôi map, tôi sẽ transform nội dung container."

> **💡 "Map = look inside, transform content, preserve structure"**: Array(3 items) → map → Array(3 items). Option(Some) → map → Option(Some). Either(Left) → map → Either(Left). Container shape UNCHANGED.

---

## ✅ Checkpoint 26.1

> Đến đây bạn phải hiểu:
> 1. **Functor** = container (`F<A>`) + `map` function
> 2. **`map`** transforms VALUE inside, preserves STRUCTURE (container)
> 3. **Array, Option, Either, Task** — all Functors with consistent map behavior
> 4. **None stays None**, **Left stays Left** after map — structure preserved
>
> **Test nhanh**: `[].map(x => x * 2)` kết quả gì?
> <details><summary>Đáp án</summary>`[]`! Empty array → map → empty array. Structure preserved (still array, still 0 elements). Map never adds or removes elements.</details>

---

## 26.2 — Functor Laws

### Hai luật đơn giản đảm bảo map hoạt động đúng

Bạn có thể viết một function gọi là `map` nhưng nó KHÔNG phải Functor. Tại sao? Vì Functor đòi hỏi `map` tuân theo **2 luật (laws)**. Laws nghe toán học, nhưng thực ra rất thực dụng: chúng đảm bảo code của bạn refactorable, optimizable, và predictable.

Nghĩ thế này: khi bạn nói "ổ cắm điện tuân chuẩn", nghĩa là BẤT KỲ thiết bị nào cắm vào đều hoạt động (đúng voltage, đúng tần số). Functor laws = chuẩn cho `map`. Bất kỳ function nào bạn truyền vào `map` đều hoạt động predictably — không có side effects bất ngờ, không có anomaly.

Hai luật rất đơn giản, nhưng hệ quả rất sâu:

```typescript
// filename: src/functor_laws.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

const map = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

const identity = <A>(a: A): A => a;

// === LAW 1: Identity ===
// map(identity)(fa) === fa
// "Map với hàm không-làm-gì = không thay đổi"

const value: Option<number> = some(42);
assert.deepStrictEqual(map(identity)(value), value);
assert.deepStrictEqual(map(identity)(none), none);

// Nếu vi phạm: map(identity) thay đổi structure → BUG
// Ví dụ: nếu map(identity)(some(5)) trả some(5, extraField) → vi phạm!

// === LAW 2: Composition ===
// map(f ∘ g)(fa) === map(f)(map(g)(fa))
// "Map hai lần = map function kết hợp một lần"

const double = (x: number) => x * 2;
const addOne = (x: number) => x + 1;

// Hai cách phải cho cùng kết quả:
const way1 = map((x: number) => addOne(double(x)))(some(5));  // map(f∘g)
const way2 = map(addOne)(map(double)(some(5)));                // map(f)(map(g))

assert.deepStrictEqual(way1, way2);
assert.deepStrictEqual(way1, some(11));  // 5*2+1 = 11

// Both None:
const noneWay1 = map((x: number) => addOne(double(x)))(none);
const noneWay2 = map(addOne)(map(double)(none));
assert.deepStrictEqual(noneWay1, noneWay2);

console.log("Functor laws OK ✅");
```

### Tại sao cần laws?

Bạn có thể tự hỏi: "Laws chỉ là toán, có ai kiểm tra đâu?" Đúng — TypeScript compiler KHÔNG kiểm tra functor laws. Nhưng bạn PHẢI tự đảm bảo. Nếu không:

- **Refactoring sẽ break**: bạn NGHĨ `map(f).map(g)` = `map(x => g(f(x)))`, nhưng nếu map vi phạm composition → hai cách cho kết quả KHÁC nhau.
- **Optimization sẽ sai**: Library có thể merge hai `.map()` thành một pass — nhưng chỉ đúng khi composition law đúng.
- **Team trust sẽ mất**: đồng nghiệp gọi `.map()` tin rằng nó SAFE. Nếu map "lén" thay đổi structure → subtle bugs.

Hãy xem ví dụ vi phạm để hiểu hậu quả:

```typescript
// filename: src/why_laws.ts

// ❌ "map" VI PHẠM identity law:
// const badMap = (fn) => (fa) => {
//     if (fa._tag === "None") return some(0);  // CHANGED structure!
//     return some(fn(fa.value));
// };
// badMap(identity)(none) === some(0) !== none ← VIOLATION!
// Hậu quả: refactoring breaks. Optimizations invalid.

// ❌ "map" VI PHẠM composition law:
// const badMap2 = (fn) => (fa) => {
//     if (fa._tag === "Some") return some(fn(fa.value) + 1);  // sneaky +1!
//     return none;
// };
// map(f∘g)(x) ≠ map(f)(map(g)(x)) vì +1 applied different times
// Hậu quả: hai map().map() ≠ một map(composed). Can't optimize.

// ✅ Laws guarantee:
// 1. map is SAFE — no surprises
// 2. Refactoring: map(f).map(g) → map(x => g(f(x))) — always valid
// 3. Library code can optimize: chain maps into single pass

console.log("Why laws matter OK ✅");
```

| Law | Nói bằng lời | Tại sao quan trọng |
|-----|-------------|-------------------|
| **Identity** | map(x → x) = do nothing | map is safe, no side effects |
| **Composition** | map(f).map(g) = map(g∘f) | Optimization valid, refactoring safe |

> **💡 Laws = guarantees**: Không phải "toán học cho vui" — laws cho phép refactor, optimize, compose code an toàn. Nếu map vi phạm laws, toàn bộ pipeline logic sụp đổ.

---

## ✅ Checkpoint 26.2

> Đến đây bạn phải hiểu:
> 1. **Identity law**: `map(x => x)(fa) === fa`. Map identity = no change
> 2. **Composition law**: `map(f)(map(g)(fa)) === map(x => f(g(x)))(fa)`
> 3. **Laws enable**: safe refactoring, optimization (merge maps), reliable composition
> 4. **Violating laws**: breaks pipeline assumptions, causes subtle bugs
>
> **Test nhanh**: `array.map(x => x).map(x => x * 2)` — có thể refactor thành?
> <details><summary>Đáp án</summary>`array.map(x => x * 2)`! Composition law: map(identity) rồi map(double) = map(double). Một pass thay vì hai — performance tốt hơn.</details>

---

## 26.3 — Lifting Functions

### Nâng function "bình thường" lên "thế giới container"

Đây là insight quan trọng nhất của chương. Bạn đã viết hàng trăm pure functions: `toUpper`, `formatName`, `calculateTax`, `isAdult`... Tất cả nhận giá trị PLAIN và trả giá trị PLAIN. Nhưng trong ứng dụng thực, data thường được bọc trong containers: `Option<string>` (có thể null), `Either<Error, number>` (có thể lỗi), `Array<User>` (nhiều users).

Bạn phải viết lại functions cho mỗi container? `toUpperOption`, `toUpperEither`, `toUpperArray`? KHÔNG. `map` làm việc đó cho bạn. `map(toUpper)` biến `toUpper: (string) => string` thành function hoạt động với BẤT KỲ container nào.

Trong category theory, đây gọi là **lifting** — nâng function từ "thế giới giá trị thuần" lên "thế giới container". `map` là cầu nối giữa hai thế giới. Giống thang máy: bạn ở tầng 1 (plain values), cần lên tầng 2 (containers) — `map` là thang máy.

```typescript
// filename: src/lifting.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

const map = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

// Regular function: works with plain values
const toUpper = (s: string): string => s.toUpperCase();
const getLength = (s: string): number => s.length;

// Use directly:
assert.strictEqual(toUpper("hello"), "HELLO");
assert.strictEqual(getLength("hello"), 5);

// LIFTED: works with Option values
const toUpperOpt: (fa: Option<string>) => Option<string> = map(toUpper);
const getLengthOpt: (fa: Option<string>) => Option<number> = map(getLength);

// Now works with containers!
assert.deepStrictEqual(toUpperOpt(some("hello")), some("HELLO"));
assert.deepStrictEqual(toUpperOpt(none), none);

assert.deepStrictEqual(getLengthOpt(some("hello")), some(5));
assert.deepStrictEqual(getLengthOpt(none), none);

// --- Lift for Either ---
type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

const eitherMap = <A, B>(fn: (a: A) => B) => <E>(fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : right(fn(fa.right));

// Same function, lifted to Either world:
const toUpperE = eitherMap(toUpper);

assert.deepStrictEqual(toUpperE(right("hello")), right("HELLO"));
assert.deepStrictEqual(toUpperE(left("error")), left("error"));

// Key insight: toUpper DOESN'T know about Option/Either.
// map LIFTS it to work with any Functor container.

console.log("Lifting OK ✅");
```

> **💡 "Write once, lift anywhere"**: Viết `toUpper` một lần. `map(toUpper)` hoạt động với Array, Option, Either, Task — bất kỳ Functor nào. Function không cần biết container. Container không cần biết function. `map` nối hai thế giới.

---

## 26.4 — Nested Functors

### Functor trong Functor — map tầng tầng

Trong thực tế, dữ liệu hiếm khi ở một tầng container duy nhất. Hãy nghĩ: bạn lookup user từ database (có thể `None`), user có danh sách orders (Array), mỗi order có thể đã cancel hoặc chưa (`Either`). Kết quả: `Option<Array<Either<Error, Order>>>` — ba tầng container lồng nhau!

Map chỉ đi vào MỘT tầng. Muốn đi sâu hơn? **Map trong map**. `Array.map(optionMap(fn))` = đi vào Array trước, rồi vào Option bên trong. Mỗi tầng map = một lớp "mở hộp".

Đây cũng chính xác là lý do Monad (Ch27) tồn tại: khi function bên trong tạo ra container MỚI, bạn bị "double-wrapped" — `Option<Option<A>>`. Functor (map) không flatten được. Monad (chain/flatMap) = map + flatten. Nhưng trước hết, hãy xem nested functors hoạt động thế nào:

```typescript
// filename: src/nested_functors.ts
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

const optionMap = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

// === Array<Option<number>> ===
const values: Option<number>[] = [some(1), none, some(3), none, some(5)];

// Map vào Option (inner functor):
const doubled = values.map(optionMap((x: number) => x * 2));
// [some(2), none, some(6), none, some(10)]

assert.deepStrictEqual(doubled[0], some(2));
assert.deepStrictEqual(doubled[1], none);
assert.deepStrictEqual(doubled[2], some(6));

// === Option<Array<number>> ===
const maybeNums: Option<number[]> = some([1, 2, 3]);

// Map vào Array (inner):
const maybeDoubled = pipe(
    maybeNums,
    optionMap(nums => nums.map(x => x * 2)),
);
assert.deepStrictEqual(maybeDoubled, some([2, 4, 6]));

// None case:
const noNums: Option<number[]> = none;
const noDoubled = pipe(noNums, optionMap(nums => nums.map(x => x * 2)));
assert.deepStrictEqual(noDoubled, none);

// === Option<Option<number>> — double-wrapped ===
const nested: Option<Option<number>> = some(some(42));

// Map outer: transform the inner Option
const outerMapped = pipe(nested, optionMap(optionMap((x: number) => x + 1)));
assert.deepStrictEqual(outerMapped, some(some(43)));

// This is where chain/flatMap comes in (Ch27) — flatten nested containers

console.log("Nested functors OK ✅");
```

> **💡 Nested map = function composition**: `Array<Option<A>>` → outer map (Array) + inner map (Option). `map(map(fn))` = go 2 levels deep. This is why Monad (Ch27) exists — to FLATTEN nested containers.

---

## ✅ Checkpoint 26.3-26.4

> Đến đây bạn phải hiểu:
> 1. **Lifting**: `map(fn)` lifts `(A→B)` to `(F<A>→F<B>)`. Works with any Functor
> 2. **Write once**: `toUpper` works with string, Option, Either, Array — via `map`
> 3. **Nested functors**: `map(map(fn))` for 2 levels deep
> 4. **Problem**: `Option<Option<A>>` — nested containers. `chain` (Ch27) flattens!
>
> **Test nhanh**: `Option<Array<string>>` — map vào Array bên trong?
> <details><summary>Đáp án</summary>`optionMap(arr => arr.map(fn))`. Outer map = optionMap (go into Option). Inner map = Array.map (transform array elements).</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Functor laws verification

```typescript
// Verify functor laws cho Array.map:
// 1. Identity: [1,2,3].map(x => x) === [1,2,3]
// 2. Composition: .map(f).map(g) === .map(x => g(f(x)))
// Test với f = (x => x * 2) và g = (x => x + 1)
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

const arr = [1, 2, 3];

// Law 1: Identity
assert.deepStrictEqual(arr.map(x => x), [1, 2, 3]);

// Law 2: Composition
const f = (x: number) => x * 2;
const g = (x: number) => x + 1;

const way1 = arr.map(f).map(g);          // two maps
const way2 = arr.map(x => g(f(x)));      // one map, composed
assert.deepStrictEqual(way1, way2);       // [3, 5, 7]
assert.deepStrictEqual(way1, [3, 5, 7]); // 1*2+1, 2*2+1, 3*2+1
```

</details>

---

**Bài 2** (10 phút): Lift domain functions

```typescript
// Viết 3 domain functions:
// 1. formatName(name: string): string — capitalize first letter
// 2. calculateTax(amount: number): number — 10%
// 3. isAdult(age: number): boolean
//
// Lift tất cả vào: Option, Either, Array
// Test mỗi lifted function
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });

type Either<E, A> = { _tag: "Left"; left: E } | { _tag: "Right"; right: A };
const left = <E>(e: E): Either<E, never> => ({ _tag: "Left", left: e });
const right = <A>(a: A): Either<never, A> => ({ _tag: "Right", right: a });

const optMap = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

const eiMap = <A, B>(fn: (a: A) => B) => <E>(fa: Either<E, A>): Either<E, B> =>
    fa._tag === "Left" ? fa : right(fn(fa.right));

// Domain functions
const formatName = (name: string): string =>
    name.charAt(0).toUpperCase() + name.slice(1).toLowerCase();

const calculateTax = (amount: number): number => Math.round(amount * 0.1);

const isAdult = (age: number): boolean => age >= 18;

// Lift to Option
assert.deepStrictEqual(optMap(formatName)(some("aN")), some("An"));
assert.deepStrictEqual(optMap(formatName)(none), none);

assert.deepStrictEqual(optMap(calculateTax)(some(1000000)), some(100000));

assert.deepStrictEqual(optMap(isAdult)(some(25)), some(true));
assert.deepStrictEqual(optMap(isAdult)(some(15)), some(false));

// Lift to Either
assert.deepStrictEqual(eiMap(formatName)(right("aN")), right("An"));
assert.deepStrictEqual(eiMap(formatName)(left("err")), left("err"));

// Lift to Array (built-in!)
assert.deepStrictEqual(["an", "BÌNH", "cường"].map(formatName), ["An", "Bình", "Cường"]);
assert.deepStrictEqual([1000000, 2000000].map(calculateTax), [100000, 200000]);
assert.deepStrictEqual([10, 18, 25].map(isAdult), [false, true, true]);
```

</details>

---

**Bài 3** (10 phút): Nested functors

```typescript
// Cho data: users: Map<string, { name: string; email?: string }>
// 1. lookupUser(id) → Option<User>
// 2. getEmail(user) → Option<string>
// 3. getDomain(email) → string
//
// Viết function: getUserDomain(id) → Option<Option<string>>
// Rồi giải thích TẠI SAO cần Monad (chain/flatMap) — Ch27
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Option<A> = { _tag: "None" } | { _tag: "Some"; value: A };
const none: Option<never> = { _tag: "None" };
const some = <A>(v: A): Option<A> => ({ _tag: "Some", value: v });
const fromNullable = <A>(v: A | null | undefined): Option<A> => v == null ? none : some(v);

const map = <A, B>(fn: (a: A) => B) => (fa: Option<A>): Option<B> =>
    fa._tag === "None" ? none : some(fn(fa.value));

type User = { name: string; email?: string };
const users = new Map<string, User>([
    ["U1", { name: "An", email: "an@mail.com" }],
    ["U2", { name: "Bình" }],
]);

const lookupUser = (id: string): Option<User> => fromNullable(users.get(id));
const getEmail = (user: User): Option<string> => fromNullable(user.email);
const getDomain = (email: string): string => email.split("@")[1];

// Only using map (Functor):
const getUserDomain = (id: string): Option<Option<string>> =>
    map(
        (user: User) => map(getDomain)(getEmail(user))
    )(lookupUser(id));

// Result: Option<Option<string>> — NESTED!
assert.deepStrictEqual(getUserDomain("U1"), some(some("mail.com")));
assert.deepStrictEqual(getUserDomain("U2"), some(none));  // user exists, no email
assert.deepStrictEqual(getUserDomain("U999"), none);       // no user

// Problem: Option<Option<string>> is annoying!
// We want Option<string> — flat, not nested.
// Solution: chain/flatMap (Ch27) = map + flatten
// chain(getEmail)(lookupUser("U1")) → some("an@mail.com") — FLAT!
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "map returns nested Option<Option<A>>" | Using map when you need chain | map = always succeeds. chain/flatMap = function returns container (Ch27) |
| "Not sure if X is a Functor" | Need to check | Does it have `map`? Does map follow identity + composition laws? → Functor! |
| "Array.map vs Option.map — why different syntax?" | JS vs fp-ts | Array: method `.map()`. fp-ts: function `map(fn)(container)` — curried for pipe |
| "Functor laws seem obvious" | They ARE obvious for correct implementations! | Laws catch BROKEN implementations. They're guardrails, not features |

---

## 💬 Đối thoại với bản thân: Functor Q&A

**Q: Functor nghe giống `Array.map` quá — có gì mới?**

A: Đúng! `Array.map` LÀ Functor. Nhưng Functor không CHỈ là Array. `Option.map`, `Either.map`, `Promise.then`, `Observable.pipe(map(...))` — tất cả là Functor. Biết điều này, bạn nhận ra: "mình đã dùng Functor mỗi ngày, chỉ không biết tên."

**Q: Tại sao cần `map` cho Option/Either? Không unwrap rồi transform được sao?**

A: Được, nhưng nguy hiểm. Unwrap = bạn phải check null/error TRƯỚC. Quên check = runtime crash. `map` làm auto: nếu None/Left, skip. Nếu Some/Right, transform. Bạn KHÔNG BAO GIỜ chạm vào giá trị bên trong trực tiếp. Container bảo vệ bạn.

**Q: Functor Laws (định luật) là gì? Quan trọng không?**

A: Hai luật: (1) `map(id) === id` (map identity = không đổi gì). (2) `map(f . g) === map(f) . map(g)` (map composition = composable). Nếu `map` của bạn tuân thúo 2 luật này, nó "behaves correctly" — bạn có thể refactor, reorder, compose mà không sợ side effects.

---

## Tóm tắt

Chương này reveal pattern ẩn đằng sau `map` — Functor. Bạn đã đi từ "biết dùng `.map()`" sang "hiểu TẠI SAO map hoạt động và PHẢI tuân luật gì". Đây không phải toán trừu tượng — đây là engineering discipline.

Mỗi lần bạn viết `.map()`, bạn đang nói: "Tôi muốn transform NỘI DUNG mà KHÔNG thay đổi CẤU TRÚC." Laws đảm bảo lời hứa đó được giữ. Lifting giúp bạn tái sử dụng functions XUYÊN SUỐT container types. Và nested functors cho thấy giới hạn — khi nào map KHÔNG đủ.

- ✅ **Functor** = container (`F<A>`) + `map: (A→B) → F<A> → F<B>`.
- ✅ **Identity law**: `map(x => x) = no change`. Map is safe.
- ✅ **Composition law**: `map(f).map(g) = map(g∘f)`. Optimize + refactor safe.
- ✅ **Lifting**: `map(fn)` lifts plain function to container world. Write once, use anywhere.
- ✅ **Nested functors**: `map(map(fn))` for 2 levels. Leads to Monad need (Ch27).

Functor là NỀN TẢNG cho mọi thứ tiếp theo. Monad = Functor + flatten. Applicative = Functor + apply. Tất cả BUILD ON TOP of Functor. Hiểu Functor = hiểu 80% FP type classes.

## Tiếp theo

→ Chapter 27: **Monads & Bind** — `chain`/`flatMap` = map + flatten. Giải quyết nested `Option<Option<A>>`. Do-notation. Why Monads MATTER.
