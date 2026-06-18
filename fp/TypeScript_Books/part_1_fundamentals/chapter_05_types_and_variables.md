# Chapter 5 — Types & Variables

> **Bạn sẽ học được**:
> - `const` vs `let` — tại sao FP gần như chỉ dùng `const`
> - Type inference — khi nào TypeScript tự suy luận, khi nào cần ghi rõ
> - Primitives: `string`, `number`, `boolean`, `bigint`, `symbol`
> - `unknown` vs `any` — tại sao `unknown` an toàn và `any` là "cửa sau"
> - `readonly` deep — từ biến đến objects đến arrays
> - Literal types và `as const` — thu hẹp types tối đa
>
> **Yêu cầu trước**: Chapter 4 (project setup, `strict: true`).
> **Thời gian đọc**: ~40 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Khai báo biến và types một cách **an toàn nhất**, hiểu khi nào compiler cần giúp đỡ.

---

## TypeScript Types — Vũ khí bí mật của bạn

Trong JavaScript, types là "ảo giác" — `typeof` nói một đằng, runtime làm một nẻo. `"5" + 1` = `"51"`, `"5" - 1` = `4`. Đây không phải bugs — đây là "features" của type coercion.

TypeScript xóa sổ toàn bộ class bugs đó. Khi bạn viết `const price: number = 35_000`, compiler **đảm bảo** `price` luôn là number. Không có `NaN` bất ngờ, không có `undefined is not a function`, không có `cannot read property of null`.

Nhưng TypeScript type system đi xa hơn "gán nhãn cho variables". Nó có type inference (tự suy luận), literal types (`"GET"` thay vì `string`), union types (`string | number`), `never` (impossible type), và `unknown` (safe alternative cho `any`). Mỗi feature giúp bạn mô tả domain chính xác hơn.

Chapter này dạy bạn vocabulary cơ bản — mọi chapter sau xây dựng trên nền tảng này.


## 5.1 — `const` vs `let`: Cuộc chiến giữa bất biến và thay đổi

### Quy tắc vàng: `const` trước, `let` khi bắt buộc

Trong FP, **data không thay đổi**. TypeScript có 2 cách khai báo biến:

```typescript
// filename: src/const_vs_let.ts
import assert from "node:assert/strict";

// const — KHÔNG BAO GIỜ gán lại
const name = "An";
// name = "Bình";  // ❌ Error: Cannot assign to 'name' because it is a constant

// let — CÓ THỂ gán lại
let score = 85;
score = 90;  // ✅ OK — nhưng mỗi lần gán = 1 cơ hội cho bug

assert.strictEqual(name, "An");
assert.strictEqual(score, 90);
```

> **💡 Ẩn dụ**: `const` = viết bằng bút mực — ghi rồi không xóa được. `let` = viết bằng bút chì — có thể tẩy và viết lại. FP ưu tiên bút mực vì: nếu giá trị không bao giờ thay đổi, bạn **không bao giờ** cần debug "ai đã thay đổi nó?".

### `var` — Kẻ bị trục xuất

```typescript
// ❌ KHÔNG bao giờ dùng var
// var x = 10;
// Tại sao? var có function scope (không phải block scope)
// và bị hoisting — gây bugs khó hiểu

// Sách này sẽ KHÔNG bao giờ dùng var. Coi như nó không tồn tại.
```

### `const` không có nghĩa immutable!

Đây là **bẫy phổ biến nhất**:

```typescript
// filename: src/const_trap.ts
import assert from "node:assert/strict";

// const ngăn GÁN LẠI — không ngăn MUTATION
const user = { name: "An", age: 25 };
// user = { name: "Bình", age: 30 };  // ❌ Cannot assign

user.age = 26;  // ✅ MUTATION ĐƯỢC! const chỉ giữ reference, không giữ nội dung
assert.strictEqual(user.age, 26);  // object bị thay đổi!

const numbers = [1, 2, 3];
numbers.push(4);  // ✅ MUTATION ĐƯỢC!
assert.strictEqual(numbers.length, 4);
```

`const` = **reference không đổi**. Muốn **nội dung không đổi** → cần `readonly` (Section 5.5).

### Khi nào dùng `let`?

Trong FP, `let` cực hiếm. Những trường hợp hợp lệ:

```typescript
// filename: src/when_let.ts

// ✅ Binary search — cần cập nhật low/high
const binarySearch = (arr: readonly number[], target: number): number | undefined => {
    let low = 0;
    let high = arr.length - 1;
    while (low <= high) {
        const mid = Math.floor((low + high) / 2);
        const midVal = arr[mid];  // type: number | undefined (noUncheckedIndexedAccess)
        if (midVal === undefined) return undefined;
        if (midVal === target) return mid;
        if (midVal < target) low = mid + 1;
        else high = mid - 1;
    }
    return undefined;
};

// ✅ Accumulator khi .reduce() quá phức tạp
// (nhưng 90% trường hợp .reduce() đủ tốt)
```

> **💡 Quy tắc thực tế**: Bắt đầu mọi biến bằng `const`. Chỉ đổi sang `let` khi compiler phàn nàn. Nếu bạn dùng `let` nhiều hơn 1-2 lần trong một function → xem lại thiết kế.

---

## ✅ Checkpoint 5.1

> Đến đây bạn phải hiểu:
> 1. **`const`** = không gán lại. **`let`** = có thể gán lại. **`var`** = cấm.
> 2. `const` **KHÔNG** ngăn mutation — chỉ giữ reference
> 3. FP: dùng `const` gần như mọi nơi. `let` chỉ cho thuật toán cần cập nhật biến
>
> **Test nhanh**: `const arr = [1, 2, 3]; arr.push(4);` — có lỗi không?
> <details><summary>Đáp án</summary>KHÔNG lỗi! `const` ngăn `arr = [...]` (gán lại), nhưng `.push()` mutate NỘI DUNG — được phép. Cần `readonly number[]` để ngăn `.push()`.</details>

---

## 5.2 — Type Inference: Compiler tự suy luận

### TypeScript thông minh hơn bạn nghĩ

Bạn không cần ghi type cho MỌI thứ. TypeScript **suy luận** (infer) types từ giá trị:

```typescript
// filename: src/inference.ts
import assert from "node:assert/strict";

// TypeScript TỰ biết type — không cần ghi
const name = "An";          // type: "An" (literal type!)
const age = 25;              // type: 25  (literal type!)
const active = true;         // type: true

// let → type rộng hơn (widening)
let score = 85;              // type: number (không phải 85)
let city = "HCM";            // type: string (không phải "HCM")

// Function return type — tự suy luận
const double = (x: number) => x * 2;  // return type: number — tự biết!
assert.strictEqual(double(5), 10);

// Array — tự suy luận element type
const fruits = ["apple", "banana"];    // type: string[]
const mixed = [1, "two", true];        // type: (string | number | boolean)[]
```

### `const` vs `let`: Ảnh hưởng đến type

Đây là chi tiết quan trọng mà nhiều người bỏ qua:

```typescript
// filename: src/widening.ts

// const → LITERAL type (hẹp nhất)
const direction = "north";   // type: "north" — chính xác giá trị này
const count = 42;            // type: 42

// let → WIDENED type (rộng hơn)
let direction2 = "north";    // type: string — có thể gán bất kỳ string
let count2 = 42;             // type: number — có thể gán bất kỳ number

// Tại sao? Vì let CÓ THỂ thay đổi — compiler không biết giá trị tương lai
// → phải dùng type rộng để chấp nhận mọi giá trị có thể
```

> **💡 Đây gọi là "type widening"**: `const` → literal type (hẹp). `let` → widened type (rộng). Hẹp = an toàn hơn. Lý do nữa để ưu tiên `const`!

### Khi nào cần ghi type?

```typescript
// filename: src/when_annotate.ts

// ✅ KHÔNG cần — inference đủ tốt
const name = "An";                          // "An"
const total = [1, 2, 3].reduce((s, x) => s + x, 0); // number
const double = (x: number) => x * 2;       // return: number

// ✅ CẦN ghi — khi inference không đủ
// 1. Function PARAMETERS — luôn cần type
const greet = (name: string): string => `Hi ${name}`;

// 2. Khi muốn type CỤ THỂ hơn inference
type Direction = "north" | "south" | "east" | "west";
const dir: Direction = "north";  // ghi rõ "đây là Direction, không phải string"

// 3. Empty arrays — TypeScript không biết element type
const items: string[] = [];  // phải ghi, không thì là never[]

// 4. Return type phức tạp — ghi rõ cho người đọc
const parseAge = (input: string): number | undefined => {
    const trimmed = input.trim();
    if (trimmed === "") return undefined;  // Number("") = 0, không phải NaN!
    const n = Number(trimmed);
    return Number.isNaN(n) ? undefined : n;
};
```

> **💡 Quy tắc**: Để TypeScript infer khi có thể. Ghi type khi: (1) function parameters, (2) muốn type hẹp hơn inference, (3) empty collections, (4) public API.

---

## ✅ Checkpoint 5.2

> Đến đây bạn phải hiểu:
> 1. **Inference** = TypeScript tự suy luận type từ giá trị
> 2. **`const` → literal type** (hẹp). **`let` → widened type** (rộng)
> 3. Ghi type khi: parameters, empty arrays, muốn hẹp hơn inference
> 4. **Không** ghi type khi inference đã đúng — tránh dư thừa
>
> **Test nhanh**: `const x = 5;` vs `let x = 5;` — types khác nhau thế nào?
> <details><summary>Đáp án</summary>`const x = 5` → type `5` (literal). `let x = 5` → type `number` (widened). Vì `let` có thể gán lại, compiler mở rộng type.</details>

---

## 5.3 — Primitive Types: Nền tảng

### 5 primitives phổ biến

```typescript
// filename: src/primitives.ts
import assert from "node:assert/strict";

// string — văn bản
const greeting = "Xin chào";            // inferred: "Xin chào" (literal)
const template = `Hello ${greeting}`;    // inferred: string
assert.strictEqual(template, "Hello Xin chào");

// number — số (integer VÀ float, cùng 1 type!)
const age = 25;                          // inferred: 25 (literal)
const pi = 3.14159;                      // inferred: 3.14159
const hex = 0xFF;                        // inferred: 255
// (ghi : number khi cần type rộng, không ghi khi inference đủ — nhớ Section 5.2)

// boolean — true / false
const active = true;                     // inferred: true

// bigint — số nguyên rất lớn (> 2⁵³)
const huge = 9007199254740993n;          // inferred: 9007199254740993n
// bigint KHÔNG trộn với number:
// const bad = huge + 1;  // ❌ Error: cannot mix bigint and number

// symbol — giá trị duy nhất, không bao giờ trùng
const id1 = Symbol("user");
const id2 = Symbol("user");
assert.notStrictEqual(id1, id2);  // khác nhau dù cùng description!

console.log("All primitives OK ✅");
```

### `null` và `undefined` — Hai "trống" khác nhau

```typescript
// filename: src/null_undefined.ts
import assert from "node:assert/strict";

// undefined = "chưa có giá trị" — biến khai báo nhưng chưa gán
let x: number | undefined;
assert.strictEqual(x, undefined);

// null = "cố ý để trống" — biết giá trị có thể không tồn tại
const findUser = (id: number): string | null => {
    if (id === 1) return "An";
    return null;  // không tìm thấy — cố ý trả null
};

assert.strictEqual(findUser(1), "An");
assert.strictEqual(findUser(999), null);
```

> **💡 Convention của sách**: Ưu tiên `undefined` (TypeScript tự trả khi missing). Dùng `null` chỉ khi API bắt buộc. Không trộn `null` và `undefined` trong cùng function.

### `void` và `never` — Types đặc biệt

```typescript
// filename: src/void_never.ts

// void = "function không trả gì"
const log = (msg: string): void => {
    console.log(msg);
    // không return — void
};

// never = "function KHÔNG BAO GIỜ return"
const throwError = (msg: string): never => {
    throw new Error(msg);
    // code sau throw không bao giờ chạy → never
};

// never cũng xuất hiện ở exhaustive switch (Chapter 1)
const assertNever = (x: never): never => {
    throw new Error(`Unexpected value: ${x}`);
};
```

---

## ✅ Checkpoint 5.3

> Đến đây bạn phải hiểu:
> 1. **5 primitives**: `string`, `number`, `boolean`, `bigint`, `symbol`
> 2. `number` = cả integer lẫn float. `bigint` cho số > 2⁵³
> 3. **`undefined`** = chưa có. **`null`** = cố ý trống. Ưu tiên `undefined`
> 4. **`void`** = không trả gì. **`never`** = không bao giờ return
>
> **Test nhanh**: `typeof null` trả gì trong JavaScript?
> <details><summary>Đáp án</summary>`"object"` — đây là bug lịch sử từ 1995, không sửa được vì sẽ phá web. TypeScript sửa bằng cách phân biệt `null` và `object` ở type level.</details>

---

## 5.4 — `unknown` vs `any`: An toàn vs Cửa sau

### `any` — Tắt TypeScript

```typescript
// filename: src/any_danger.ts

// any = "bất kỳ type nào" = TẮT type checking
const dangerous: any = "hello";
dangerous.foo();          // ✅ Compiler OK — runtime 💥 TypeError!
dangerous.bar.baz.qux;   // ✅ Compiler OK — runtime 💥 TypeError!
dangerous * 2;            // ✅ Compiler OK — runtime = NaN (silent bug!)

// any LÂY LAN — 1 any sẽ "nhiễm" sang các biến khác
const result = dangerous + 1;  // result: any — lan truyền!
```

> **⚠️ `any` = vô hiệu hóa TypeScript.** Mỗi `any` trong code = 1 lỗ hổng mà compiler không thể bảo vệ. Strict mode (`noImplicitAny`) cấm `any` ngầm — nhưng bạn vẫn có thể viết `any` tường minh. **Đừng.**

### `unknown` — `any` nhưng an toàn

```typescript
// filename: src/unknown_safe.ts
import assert from "node:assert/strict";

// unknown = "không biết type" — nhưng PHẢI kiểm tra trước khi dùng
const input: unknown = JSON.parse('{"name": "An"}');

// input.name;        // ❌ Error: 'input' is of type 'unknown'
// input + 1;         // ❌ Error: 'input' is of type 'unknown'

// ✅ Phải kiểm tra (narrowing) trước
if (typeof input === "object" && input !== null && "name" in input) {
    console.log((input as { name: string }).name);  // "An" — an toàn!
}

// Pattern phổ biến: type guard function
const isUser = (val: unknown): val is { name: string; age: number } =>
    typeof val === "object" &&
    val !== null &&
    "name" in val &&
    "age" in val &&
    typeof (val as { name: string }).name === "string" &&
    typeof (val as { age: number }).age === "number";

const data: unknown = JSON.parse('{"name": "An", "age": 25}');

if (isUser(data)) {
    // Trong block này: data type = { name: string; age: number }
    assert.strictEqual(data.name, "An");
    assert.strictEqual(data.age, 25);
    console.log(`User: ${data.name}, ${data.age}`);
}
```

### So sánh: `any` vs `unknown`

| | `any` | `unknown` |
|---|---|---|
| Gán vào | ✅ mọi thứ | ✅ mọi thứ |
| Dùng trực tiếp | ✅ không cần check | ❌ **phải check** |
| Type safety | ❌ TẮT hoàn toàn | ✅ BẬT — bắt check |
| Lây lan | ⚠️ kết quả cũng `any` | ✅ không lan |
| Dùng khi | ❌ KHÔNG BAO GIỜ | ✅ data từ bên ngoài (API, JSON, user input) |

> **💡 Quy tắc**: Thấy `any` → đổi thành `unknown`. Rồi viết type guard hoặc schema validation. Cuốn sách này sẽ **KHÔNG BAO GIỜ** dùng `any`.

---

## ✅ Checkpoint 5.4

> Đến đây bạn phải hiểu:
> 1. **`any`** = tắt TypeScript. Nguy hiểm, lây lan, KHÔNG dùng.
> 2. **`unknown`** = "chưa biết type" — phải check trước khi dùng. AN TOÀN.
> 3. **Type guard** (`is` keyword) = thu hẹp `unknown` thành type cụ thể
> 4. Data từ bên ngoài (API, JSON) → luôn bắt đầu với `unknown`
>
> **Test nhanh**: `const x: any = "hello"; x.toFixed(2);` — compile hay crash?
> <details><summary>Đáp án</summary>COMPILE thành công, RUNTIME crash! `"hello".toFixed(2)` → TypeError. Đây là lý do `any` nguy hiểm — compiler bỏ qua lỗi mà runtime gặp.</details>

---

## 5.5 — `readonly`: Immutability thật sự

### Nhắc lại: `const` ≠ immutable

Section 5.1 đã nói: `const` chỉ giữ reference. `readonly` giữ **nội dung**:

```typescript
// filename: src/readonly.ts
import assert from "node:assert/strict";

// === readonly properties ===
type User = {
    readonly name: string;
    readonly age: number;
};

const user: User = { name: "An", age: 25 };
// user.age = 26;  // ❌ Error: Cannot assign to 'age' because it is a read-only property

// Muốn "sửa" → tạo bản mới (immutable update từ Chapter 3)
const updated = { ...user, age: 26 };
assert.strictEqual(user.age, 25);     // gốc KHÔNG ĐỔI
assert.strictEqual(updated.age, 26);  // bản mới

// === readonly arrays ===
const fruits: readonly string[] = ["apple", "banana", "cherry"];
// fruits.push("date");    // ❌ Error: Property 'push' does not exist on type 'readonly string[]'
// fruits[0] = "grape";    // ❌ Error: Index signature in type 'readonly string[]' only permits reading

// Muốn "thêm" → spread ra array mới
const moreFruits = [...fruits, "date"];
assert.strictEqual(fruits.length, 3);      // gốc KHÔNG ĐỔI
assert.strictEqual(moreFruits.length, 4);

// === ReadonlyArray<T> — cú pháp thay thế ===
const numbers: ReadonlyArray<number> = [1, 2, 3];
// Giống hệt readonly number[]

console.log("Readonly working ✅");
```

### `Readonly<T>` — Utility type

```typescript
// filename: src/readonly_utility.ts
import assert from "node:assert/strict";

// Không muốn ghi readonly cho từng field?
type MutableUser = {
    name: string;
    age: number;
    email: string;
};

// Readonly<T> — đổi TẤT CẢ fields thành readonly
type User = Readonly<MutableUser>;
// = { readonly name: string; readonly age: number; readonly email: string }

const user: User = { name: "An", age: 25, email: "an@mail.com" };
// user.name = "Bình";  // ❌ Error!

// ⚠️ Readonly<T> chỉ SHALLOW — 1 tầng
type Company = {
    name: string;
    address: { city: string; zip: string };
};

type ReadonlyCompany = Readonly<Company>;
const co: ReadonlyCompany = {
    name: "Startup X",
    address: { city: "HCM", zip: "700000" }
};

// co.name = "New";              // ❌ blocked!
// co.address.city = "Hà Nội";  // ✅ ALLOWED! — Readonly chỉ 1 tầng!
```

> **💡 Deep readonly**: `Readonly<T>` chỉ shallow (1 tầng). Muốn deep → ghi `readonly` cho từng nested type, hoặc dùng thư viện (Chapter 9 sẽ cover `DeepReadonly` custom type).

### `as const` — Readonly tối đa

`as const` = **literal + readonly + recursive**. Mạnh nhất:

```typescript
// filename: src/as_const.ts
import assert from "node:assert/strict";

// Không có as const:
const config1 = {
    host: "localhost",
    port: 3000,
    debug: true
};
// type: { host: string; port: number; debug: boolean }
// → string, number — types rộng, có thể gán lại

// Với as const:
const config2 = {
    host: "localhost",
    port: 3000,
    debug: true
} as const;
// type: { readonly host: "localhost"; readonly port: 3000; readonly debug: true }
// → literal types + readonly — hẹp nhất, an toàn nhất!

// config2.port = 8080;  // ❌ Error: Cannot assign to 'port' (readonly)
// config2.host = "x";   // ❌ Error: Type '"x"' is not assignable to type '"localhost"'

// as const cho arrays:
const sizes = ["S", "M", "L"] as const;
// type: readonly ["S", "M", "L"] — tuple, không phải string[]!

type Size = typeof sizes[number];  // "S" | "M" | "L"
// Tạo union type từ array — pattern rất phổ biến!

assert.strictEqual(sizes.length, 3);
console.log("as const working ✅");
```

> **💡 `as const` là công cụ FP yêu thích**: Biến mọi object/array thành immutable + literal types. Pattern `typeof arr[number]` tạo union type từ array — dùng thường xuyên.

---

## ✅ Checkpoint 5.5

> Đến đây bạn phải hiểu:
> 1. **`readonly`** trên property/array = ngăn mutation compile-time
> 2. **`Readonly<T>`** = utility type, nhưng chỉ SHALLOW (1 tầng)
> 3. **`as const`** = literal types + readonly + recursive — mạnh nhất
> 4. **`typeof arr[number]`** = tạo union type từ `as const` array
>
> **Test nhanh**: `const x = [1, 2, 3] as const; x.push(4);` — có lỗi không?
> <details><summary>Đáp án</summary>CÓ lỗi compile-time! `as const` → `readonly [1, 2, 3]` (tuple). `.push()` không tồn tại trên readonly tuple.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Literal types

Dùng `as const` + `typeof` để tạo union type `HttpMethod` từ array:

```typescript
// Yêu cầu: Tạo type HttpMethod = "GET" | "POST" | "PUT" | "DELETE"
// Bắt đầu từ một array, KHÔNG hardcode union type
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
const HTTP_METHODS = ["GET", "POST", "PUT", "DELETE"] as const;
type HttpMethod = typeof HTTP_METHODS[number];
// = "GET" | "POST" | "PUT" | "DELETE"
```

</details>

---

**Bài 2** (10 phút): Type guard cho API response

Viết type guard `isApiError` kiểm tra unknown data có phải error response:

```typescript
type ApiError = {
    readonly code: number;
    readonly message: string;
};

// isApiError(data: unknown): data is ApiError
// Phải kiểm tra: object, có "code" (number), có "message" (string)
```

<details><summary>💡 Gợi ý</summary>Chain: `typeof val === "object"` → `val !== null` → `"code" in val` → `typeof code === "number"` → tương tự cho message.</details>

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type ApiError = {
    readonly code: number;
    readonly message: string;
};

const isApiError = (val: unknown): val is ApiError =>
    typeof val === "object" &&
    val !== null &&
    "code" in val &&
    "message" in val &&
    typeof (val as { code: unknown }).code === "number" &&
    typeof (val as { message: unknown }).message === "string";

// Test
const data1: unknown = { code: 404, message: "Not found" };
const data2: unknown = { name: "An" };
const data3: unknown = null;

assert.strictEqual(isApiError(data1), true);
assert.strictEqual(isApiError(data2), false);
assert.strictEqual(isApiError(data3), false);

if (isApiError(data1)) {
    // data1 tự động thu hẹp thành ApiError!
    assert.strictEqual(data1.code, 404);
    assert.strictEqual(data1.message, "Not found");
}
```

</details>

---

**Bài 3** (15 phút): Config system với `as const` + `readonly`

Viết hệ thống config hoàn chỉnh:

```typescript
// 1. Tạo ENVIRONMENTS as const = ["development", "staging", "production"]
// 2. Type Environment = union từ array trên
// 3. Type Config = { readonly env: Environment; readonly port: number; readonly debug: boolean }
// 4. Function getConfig(env: Environment): Config
//    - development: port 3000, debug true
//    - staging: port 8080, debug true
//    - production: port 80, debug false
// 5. Dùng exhaustive switch — thêm environment mới = compiler báo lỗi
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

const ENVIRONMENTS = ["development", "staging", "production"] as const;
type Environment = typeof ENVIRONMENTS[number];

type Config = {
    readonly env: Environment;
    readonly port: number;
    readonly debug: boolean;
};

const getConfig = (env: Environment): Config => {
    switch (env) {
        case "development": return { env, port: 3000, debug: true };
        case "staging":     return { env, port: 8080, debug: true };
        case "production":  return { env, port: 80, debug: false };
    }
};

const devConfig = getConfig("development");
assert.strictEqual(devConfig.port, 3000);
assert.strictEqual(devConfig.debug, true);

const prodConfig = getConfig("production");
assert.strictEqual(prodConfig.port, 80);
assert.strictEqual(prodConfig.debug, false);

// devConfig.port = 9999;  // ❌ Error: readonly!

console.log(`Dev: port ${devConfig.port}, debug ${devConfig.debug}`);
console.log(`Prod: port ${prodConfig.port}, debug ${prodConfig.debug}`);
// Output:
// Dev: port 3000, debug true
// Prod: port 80, debug false
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `const x = "hello"` type là `string` thay vì `"hello"` | Khai báo bằng `let` hoặc trong mutable context | Dùng `const` hoặc `as const` |
| `readonly` field vẫn bị mutate | `Readonly<T>` chỉ shallow | Ghi `readonly` cho nested types, hoặc dùng `as const` |
| `any` bị lây lan khắp code | 1 `any` → kết quả cũng `any` | Tìm nguồn `any`, đổi thành `unknown` + type guard |
| Type guard không thu hẹp đúng | Thiếu return type `val is T` | Ghi rõ return type: `(val: unknown): val is MyType` |
| `as const` array không cho `.push()` | Đúng! `as const` → readonly tuple | Đây là hành vi mong muốn. Spread ra array mới nếu cần thêm |

---

## Tóm tắt

- ✅ **`const` trước, `let` khi bắt buộc, `var` KHÔNG BAO GIỜ.**
- ✅ **Type inference**: để TypeScript suy luận. Ghi type cho parameters, empty arrays, public API.
- ✅ **`const` → literal type** (hẹp). **`let` → widened type** (rộng). Hẹp = an toàn.
- ✅ **`unknown` thay `any`**. `any` = tắt TypeScript. `unknown` = bắt kiểm tra trước.
- ✅ **`readonly`** cho properties + arrays. **`Readonly<T>`** shallow. **`as const`** deep + literal.
- ✅ **`typeof arr[number]`** = union type từ `as const` array.

## Tiếp theo

→ Chapter 6: **Control Flow & Narrowing** — Type narrowing (`typeof`, `instanceof`, `in`), discriminated unions + exhaustive `switch`, user-defined type guards, và assertion functions.
