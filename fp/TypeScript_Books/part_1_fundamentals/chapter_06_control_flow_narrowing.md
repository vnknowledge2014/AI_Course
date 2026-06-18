# Chapter 6 — Control Flow & Narrowing

> **Bạn sẽ học được**:
> - Type narrowing — cách TypeScript **thu hẹp** types khi bạn kiểm tra điều kiện
> - `typeof`, `instanceof`, `in` — 3 vũ khí narrowing cơ bản
> - Discriminated unions + `switch` — pattern FP quan trọng nhất
> - Exhaustive checking với `never` — đảm bảo xử lý mọi trường hợp
> - User-defined type guards — tự viết narrowing functions
>
> **Yêu cầu trước**: Chapter 5 (types, `unknown`, `readonly`).
> **Thời gian đọc**: ~40 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Viết code không bao giờ quên xử lý 1 case — compiler canh chừng cho bạn.

---

## 6.1 — Type Narrowing: Thu hẹp để an toàn

### Câu chuyện: Kiểm tra trước khi mở

Bạn nhận được một gói hàng. Có thể là sách, có thể là đĩa thủy tinh. Bạn không vứt xuống đất — bạn **kiểm tra** trước: nếu sách thì mở bình thường, nếu thủy tinh thì mở nhẹ nhàng.

TypeScript narrowing hoạt động giống vậy. Khi biến có nhiều types (`string | number`), bạn phải **kiểm tra** để compiler biết bạn đang xử lý type nào.

### `typeof` — Kiểm tra primitive types

```typescript
// filename: src/typeof_narrowing.ts
import assert from "node:assert/strict";

// input có thể là string HOẶC number
const formatValue = (input: string | number): string => {
    // Trước if: input = string | number — compiler KHÔNG cho gọi .toUpperCase()
    // input.toUpperCase();  // ❌ Error: Property 'toUpperCase' does not exist on type 'number'

    if (typeof input === "string") {
        // Trong block này: input đã thu hẹp thành string!
        return input.toUpperCase();     // ✅ OK
    }

    // Sau if-return: input chỉ còn number (TypeScript tự loại trừ string)
    return input.toFixed(2);            // ✅ OK — compiler biết đây là number
};

assert.strictEqual(formatValue("hello"), "HELLO");
assert.strictEqual(formatValue(3.14159), "3.14");

console.log(formatValue("hello"));  // HELLO
console.log(formatValue(3.14159));  // 3.14
```

> **💡 Đây là "narrowing"**: Từ type rộng (`string | number`) → thu hẹp thành type cụ thể (`string` hoặc `number`) bằng cách kiểm tra. Compiler **theo dõi** flow code và tự biết type trong mỗi branch.

### `typeof` chỉ phân biệt 8 loại

```typescript
// typeof trả 1 trong 8 strings:
typeof "hello"     // "string"
typeof 42          // "number"
typeof true        // "boolean"
typeof 42n         // "bigint"
typeof Symbol()    // "symbol"
typeof undefined   // "undefined"
typeof null        // "object"  ← BUG lịch sử!
typeof {}          // "object"
typeof (() => {})  // "function"
```

> **⚠️ `typeof null === "object"`**: Bug từ 1995. Vì vậy, kiểm tra null phải dùng `=== null`, KHÔNG dùng `typeof`.

### Truthiness narrowing

```typescript
// filename: src/truthiness.ts
import assert from "node:assert/strict";

const printLength = (input: string | null | undefined): string => {
    // Falsy values: null, undefined, "", 0, NaN, false
    if (input) {
        // Trong block này: input = string (loại null + undefined + "")
        return `Length: ${input.length}`;
    }
    return "No input";
};

assert.strictEqual(printLength("hello"), "Length: 5");
assert.strictEqual(printLength(null), "No input");
assert.strictEqual(printLength(undefined), "No input");
assert.strictEqual(printLength(""), "No input");  // ⚠️ "" cũng falsy!

console.log(printLength("hello"));  // Length: 5
```

> **⚠️ Cẩn thận**: Truthiness narrowing loại bỏ `null`, `undefined`, `""`, `0`, `NaN`, `false`. Nếu `0` hoặc `""` là giá trị hợp lệ → dùng `!== null` thay vì truthiness.

---

## ✅ Checkpoint 6.1

> Đến đây bạn phải hiểu:
> 1. **Narrowing** = thu hẹp type bằng điều kiện. Compiler theo dõi flow
> 2. **`typeof`** phân biệt primitives: `"string"`, `"number"`, `"boolean"`, ...
> 3. **`typeof null === "object"`** — bug lịch sử. Kiểm tra null bằng `=== null`
> 4. **Truthiness** loại falsy values — nhưng cũng loại `""` và `0`!
>
> **Test nhanh**: `typeof null` trả gì? Tại sao kiểm tra null phải dùng `=== null`?
> <details><summary>Đáp án</summary>`"object"` — bug lịch sử. `typeof x === "object"` KHÔNG loại được null. Phải dùng `x !== null` riêng.</details>

---

## 6.2 — `instanceof` và `in`

### `instanceof` — Kiểm tra class/constructor

```typescript
// filename: src/instanceof.ts
import assert from "node:assert/strict";

class ApiError extends Error {
    readonly code: number;
    constructor(code: number, message: string) {
        super(message);
        this.code = code;
    }
}

class NetworkError extends Error {
    readonly url: string;
    constructor(url: string) {
        super(`Network error: ${url}`);
        this.url = url;
    }
}

const handleError = (err: Error): string => {
    if (err instanceof ApiError) {
        // err thu hẹp thành ApiError — có .code
        return `API Error ${err.code}: ${err.message}`;
    }
    if (err instanceof NetworkError) {
        // err thu hẹp thành NetworkError — có .url
        return `Network Error at ${err.url}`;
    }
    // err vẫn là Error cơ bản
    return `Unknown error: ${err.message}`;
};

assert.strictEqual(
    handleError(new ApiError(404, "Not found")),
    "API Error 404: Not found"
);
assert.strictEqual(
    handleError(new NetworkError("https://api.example.com")),
    "Network Error at https://api.example.com"
);

console.log(handleError(new ApiError(500, "Server error")));
// Output: API Error 500: Server error
```

> **💡 Khi nào dùng `instanceof`?** Khi xử lý **Error types** hoặc **class hierarchies**. Trong FP thuần túy, ta ưu tiên discriminated unions (Section 6.3) — nhưng `instanceof` vẫn cần cho errors và DOM.

### `in` — Kiểm tra property có tồn tại

```typescript
// filename: src/in_narrowing.ts
import assert from "node:assert/strict";

type Fish = { readonly swim: () => string };
type Bird = { readonly fly: () => string };

const move = (animal: Fish | Bird): string => {
    if ("swim" in animal) {
        // animal thu hẹp thành Fish
        return animal.swim();
    }
    // animal thu hẹp thành Bird (loại trừ Fish)
    return animal.fly();
};

const fish: Fish = { swim: () => "Swimming 🐟" };
const bird: Bird = { fly: () => "Flying 🐦" };

assert.strictEqual(move(fish), "Swimming 🐟");
assert.strictEqual(move(bird), "Flying 🐦");

console.log(move(fish));  // Swimming 🐟
console.log(move(bird));  // Flying 🐦
```

> **💡 `in` vs discriminated unions**: `in` kiểm tra property **tồn tại**. Discriminated unions kiểm tra property **giá trị** (`tag`). DU mạnh hơn, rõ ràng hơn — Section 6.3.

---

## ✅ Checkpoint 6.2

> Đến đây bạn phải hiểu:
> 1. **`instanceof`** kiểm tra class/constructor. Dùng cho Error types
> 2. **`in`** kiểm tra property tồn tại. Thu hẹp giữa types có/không có property đó
> 3. Cả hai là **narrowing tools** — compiler tự thu hẹp type trong mỗi branch
>
> **Test nhanh**: `"swim" in animal` thu hẹp `Fish | Bird` thành gì?
> <details><summary>Đáp án</summary>Thành `Fish` — vì chỉ `Fish` có property `swim`. TypeScript loại trừ `Bird`. Nhưng cẩn thận: nếu object có CẢ `swim` VÀ `fly`, nó vẫn vào branch Fish.</details>

---

## 6.3 — Discriminated Unions + Switch: Pattern FP số 1

### Nhắc lại từ Chapter 1

Discriminated unions = union type + **tag field** để phân biệt. `switch` trên tag = compiler biết chính xác type trong mỗi case.

Đây là pattern bạn sẽ dùng **nhiều nhất** trong FP TypeScript.

### Ví dụ: Kết quả API

```typescript
// filename: src/api_result.ts
import assert from "node:assert/strict";

// Discriminated union — tag = "tag" field
type ApiResult =
    | { readonly tag: "loading" }
    | { readonly tag: "success"; readonly data: string }
    | { readonly tag: "error"; readonly code: number; readonly message: string };

// Switch — mỗi case, compiler biết chính xác type
const renderResult = (result: ApiResult): string => {
    switch (result.tag) {
        case "loading":
            return "⏳ Loading...";

        case "success":
            // result thu hẹp: { tag: "success"; data: string }
            return `✅ ${result.data}`;

        case "error":
            // result thu hẹp: { tag: "error"; code: number; message: string }
            return `❌ Error ${result.code}: ${result.message}`;
    }
};

assert.strictEqual(
    renderResult({ tag: "loading" }),
    "⏳ Loading..."
);
assert.strictEqual(
    renderResult({ tag: "success", data: "User An loaded" }),
    "✅ User An loaded"
);
assert.strictEqual(
    renderResult({ tag: "error", code: 404, message: "Not found" }),
    "❌ Error 404: Not found"
);

console.log(renderResult({ tag: "success", data: "Hello!" }));
// Output: ✅ Hello!
```

### Tại sao switch + DU mạnh?

1. **Compiler biết type** trong mỗi case — không cần cast
2. **Không quên case** — exhaustive checking (Section 6.4)
3. **Mỗi case là pure** — không side effects, dễ test
4. **Đọc dễ** — scan nhanh mọi trường hợp

### Ví dụ thực tế: Payment processing

```typescript
// filename: src/payment.ts
import assert from "node:assert/strict";

type PaymentMethod =
    | { readonly tag: "card"; readonly last4: string; readonly expiry: string }
    | { readonly tag: "bank_transfer"; readonly bankName: string; readonly accountNo: string }
    | { readonly tag: "ewallet"; readonly provider: "momo" | "zalopay" | "vnpay" };

type PaymentResult =
    | { readonly tag: "success"; readonly transactionId: string }
    | { readonly tag: "failed"; readonly reason: string }
    | { readonly tag: "pending"; readonly estimatedTime: number };

// Pure function — không side effects
const describePayment = (method: PaymentMethod): string => {
    switch (method.tag) {
        case "card":
            return `Thẻ ****${method.last4} (HSD: ${method.expiry})`;
        case "bank_transfer":
            return `Chuyển khoản ${method.bankName} - ${method.accountNo}`;
        case "ewallet":
            return `Ví ${method.provider.toUpperCase()}`;
    }
};

const describeResult = (result: PaymentResult): string => {
    switch (result.tag) {
        case "success":
            return `✅ Thành công! Mã GD: ${result.transactionId}`;
        case "failed":
            return `❌ Thất bại: ${result.reason}`;
        case "pending":
            return `⏳ Đang xử lý (~${result.estimatedTime}s)`;
    }
};

// Test
const card: PaymentMethod = { tag: "card", last4: "1234", expiry: "12/25" };
const momo: PaymentMethod = { tag: "ewallet", provider: "momo" };

assert.strictEqual(describePayment(card), "Thẻ ****1234 (HSD: 12/25)");
assert.strictEqual(describePayment(momo), "Ví MOMO");

const success: PaymentResult = { tag: "success", transactionId: "TX-001" };
assert.strictEqual(describeResult(success), "✅ Thành công! Mã GD: TX-001");

console.log(describePayment(card));
console.log(describeResult(success));
// Output:
// Thẻ ****1234 (HSD: 12/25)
// ✅ Thành công! Mã GD: TX-001
```

> **💡 Đây là Domain Modeling**: `PaymentMethod` và `PaymentResult` mô tả **mọi trạng thái hợp lệ** của domain. Không có trạng thái "card nhưng thiếu last4" — type system cấm. Make illegal states unrepresentable!

---

## ✅ Checkpoint 6.3

> Đến đây bạn phải hiểu:
> 1. **Discriminated union** = tag field + union. Switch trên tag = compiler biết type
> 2. **Mỗi case** tự động thu hẹp — truy cập fields specific cho case đó
> 3. **Switch không cần `default`** khi cover hết cases (mỗi case returns)
> 4. DU = cách FP model **mọi trạng thái hợp lệ** của domain
>
> **Test nhanh**: Thêm `{ tag: "refund"; amount: number }` vào `PaymentResult`. Compiler báo lỗi ở đâu?
> <details><summary>Đáp án</summary>Ở function `describeResult` — switch chưa handle case "refund". Compiler: "Function lacks ending return statement". Bạn phải thêm case mới!</details>

---

## 6.4 — Exhaustive Checking: Không bao giờ quên

### Vấn đề: Thêm case mới → quên xử lý

Khi union type lớn dần, rất dễ quên 1 case khi thêm variant mới. TypeScript có 2 cách chống:

### Cách 1: Return type tự nhiên (mọi case returns)

```typescript
// filename: src/exhaustive_natural.ts

type Shape =
    | { readonly tag: "circle"; readonly radius: number }
    | { readonly tag: "square"; readonly side: number }
    | { readonly tag: "triangle"; readonly base: number; readonly height: number };

// Mỗi case return → compiler biết hết cases được cover
const area = (shape: Shape): number => {
    switch (shape.tag) {
        case "circle":   return Math.PI * shape.radius ** 2;
        case "square":   return shape.side ** 2;
        case "triangle": return 0.5 * shape.base * shape.height;
    }
    // Không cần default — compiler biết mọi case đã return
};

// Thêm { tag: "rectangle"; ... } vào Shape?
// → Compiler báo lỗi ở function area: "Function lacks ending return statement"
// → PHẢI thêm case "rectangle" mới compile được!
```

### Cách 2: `assertNever` — Tường minh

Cách 1 hoạt động khi TypeScript suy ra hết. Nhưng `assertNever` **tường minh hơn** — lỗi compile rõ ràng hơn:

```typescript
// filename: src/assert_never.ts
import assert from "node:assert/strict";

// Helper — dùng lại ở mọi nơi
const assertNever = (x: never): never => {
    throw new Error(`Unexpected value: ${JSON.stringify(x)}`);
};

type Light = "red" | "yellow" | "green";

const action = (light: Light): string => {
    switch (light) {
        case "red":    return "Stop 🛑";
        case "yellow": return "Slow down ⚠️";
        case "green":  return "Go 🟢";
        default:       return assertNever(light);
        // light ở đây có type `never` — nếu thiếu 1 case,
        // light sẽ KHÔNG phải never → compile error!
    }
};

assert.strictEqual(action("red"), "Stop 🛑");
assert.strictEqual(action("green"), "Go 🟢");

// Thêm "blue" vào Light?
// → default: assertNever(light) → Error: "blue" is not assignable to type 'never'
// → Lỗi RÕ RÀNG — biết chính xác giá trị nào chưa xử lý
```

### So sánh 2 cách

| | Return tự nhiên | `assertNever` |
|---|---|---|
| Cần `default`? | Không | Có (chứa `assertNever`) |
| Lỗi compile | "Function lacks return" | `Type 'X' not assignable to 'never'` |
| Rõ ràng | ⭐⭐ | ⭐⭐⭐ |
| Runtime safety | Không catch | ✅ Throw nếu giá trị bất ngờ |
| Dùng khi | Switch đơn giản | Switch phức tạp, public API |

> **💡 Quy tắc**: Dùng **return tự nhiên** cho switch nhỏ (3-4 cases). Dùng **`assertNever`** cho switch lớn, public functions, hoặc khi muốn lỗi compile rõ ràng hơn.

---

## ✅ Checkpoint 6.4

> Đến đây bạn phải hiểu:
> 1. **Exhaustive checking** = compiler đảm bảo mọi case được xử lý
> 2. **Return tự nhiên** — mỗi case returns → compiler tự kiểm
> 3. **`assertNever`** — tường minh, lỗi rõ, runtime safety
> 4. Thêm variant mới vào union → compiler **bắt buộc** xử lý ở mọi switch
>
> **Test nhanh**: `type T = "a" | "b"; switch(x) { case "a": ... }` — `assertNever(x)` trong default có lỗi gì?
> <details><summary>Đáp án</summary>Error: `Type '"b"' is not assignable to type 'never'`. Compiler biết case "b" chưa handle, nên x còn type "b" ≠ never.</details>

---

## 6.5 — Custom Type Guards

### Type guard functions

Chapter 5 đã giới thiệu `val is T` (nhớ `isUser` trong Section 5.4). Bây giờ đào sâu — type guard là tool mạnh để narrowing `unknown` hoặc union types:

```typescript
// filename: src/type_guards.ts
import assert from "node:assert/strict";

// === Type guard cho objects ===
type User = {
    readonly name: string;
    readonly age: number;
    readonly email: string;
};

// "val is User" = nếu return true → TypeScript coi val là User
const isUser = (val: unknown): val is User =>
    typeof val === "object" &&
    val !== null &&
    "name" in val && typeof (val as { name: unknown }).name === "string" &&
    "age" in val && typeof (val as { age: unknown }).age === "number" &&
    "email" in val && typeof (val as { email: unknown }).email === "string";

// Dùng
const data: unknown = JSON.parse('{"name":"An","age":25,"email":"an@x.com"}');

if (isUser(data)) {
    // Trong block này: data = User — compiler biết!
    assert.strictEqual(data.name, "An");
    assert.strictEqual(data.age, 25);
    console.log(`User: ${data.name}`);
}

// (Pattern giống Section 5.4 — ở đây lặp lại để bạn thấy trong context narrowing)
```

### Type guard cho discriminated unions

```typescript
// filename: src/du_guard.ts
import assert from "node:assert/strict";

type Result<T> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: string };

// Type guard — kiểm tra success
const isOk = <T>(result: Result<T>): result is { tag: "ok"; value: T } =>
    result.tag === "ok";

const isErr = <T>(result: Result<T>): result is { tag: "err"; error: string } =>
    result.tag === "err";

// Dùng
const result: Result<number> = { tag: "ok", value: 42 };

if (isOk(result)) {
    assert.strictEqual(result.value, 42);  // ✅ compiler biết có .value
}

// Pipeline với .filter()
const results: readonly Result<number>[] = [
    { tag: "ok", value: 1 },
    { tag: "err", error: "bad" },
    { tag: "ok", value: 3 },
];

// .filter(isOk) — TypeScript tự thu hẹp array type!
const successes = results.filter(isOk);
// type: { tag: "ok"; value: number }[]

const total = successes.reduce((sum, r) => sum + r.value, 0);
assert.strictEqual(total, 4);

console.log(`Total from OK results: ${total}`);
// Output: Total from OK results: 4
```

> **💡 Type guards + `.filter()`**: Đây là pattern FP cực mạnh. `.filter(isOk)` vừa lọc runtime VÀ thu hẹp type compile-time. Code an toàn ở cả 2 levels.

### Assertion functions (`asserts`)

Ngoài type guard (`val is T`), TypeScript có **assertion functions** — thay vì return boolean, chúng `throw` nếu sai:

```typescript
// filename: src/assertion_fn.ts
import assert from "node:assert/strict";

// Assertion function — throw nếu KHÔNG phải string
function assertString(val: unknown): asserts val is string {
    if (typeof val !== "string") {
        throw new TypeError(`Expected string, got ${typeof val}`);
    }
}

const input: unknown = "hello";

assertString(input);
// Sau dòng này: input = string (compiler biết!)
assert.strictEqual(input.toUpperCase(), "HELLO");

// assertString(42);  // 💥 throw TypeError
```

> **💡 `asserts val is T`** vs `val is T`: Type guard = trả boolean, dùng trong `if`. Assertion function = throw nếu sai, thu hẹp type cho code **sau** lời gọi. Dùng assertion khi failure = bug nghiêm trọng.

---

## ✅ Checkpoint 6.5

> Đến đây bạn phải hiểu:
> 1. **Type guard** (`val is T`) = return boolean + thu hẹp type trong `if` block
> 2. **`.filter(isOk)`** = lọc + thu hẹp type — pattern cực mạnh
> 3. **Assertion function** (`asserts val is T`) = throw nếu sai, thu hẹp sau lời gọi
> 4. Type guards cần viết **cẩn thận** — compiler tin bạn hoàn toàn
>
> **Test nhanh**: Type guard return `true` cho mọi input → có nguy hiểm không?
> <details><summary>Đáp án</summary>RẤT nguy hiểm! Compiler TIN hoàn toàn type guard — nếu guard sai, type system bị "lừa". Guard phải kiểm tra THẬT, không được giả.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Narrowing cơ bản

Viết function `stringify` chuyển mọi input thành string có ý nghĩa:

```typescript
// stringify(42)        → "42"
// stringify("hello")   → "hello"
// stringify(true)      → "true"
// stringify(null)      → "null"
// stringify(undefined) → "undefined"
// stringify([1, 2, 3]) → "[1,2,3]"

type Stringifiable = string | number | boolean | null | undefined | readonly unknown[];
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type Stringifiable = string | number | boolean | null | undefined | readonly unknown[];

const stringify = (input: Stringifiable): string => {
    if (input === null) return "null";
    if (input === undefined) return "undefined";
    if (Array.isArray(input)) return `[${input.join(",")}]`;
    return String(input);
};

assert.strictEqual(stringify(42), "42");
assert.strictEqual(stringify("hello"), "hello");
assert.strictEqual(stringify(true), "true");
assert.strictEqual(stringify(null), "null");
assert.strictEqual(stringify(undefined), "undefined");
assert.strictEqual(stringify([1, 2, 3]), "[1,2,3]");
```

</details>

---

**Bài 2** (10 phút): Exhaustive switch

Viết hệ thống notification với exhaustive checking:

```typescript
type Notification =
    | { readonly tag: "email"; readonly to: string; readonly subject: string }
    | { readonly tag: "sms"; readonly phone: string; readonly text: string }
    | { readonly tag: "push"; readonly deviceId: string; readonly title: string };

// 1. Viết function describe(n: Notification): string
// 2. Dùng assertNever ở default
// 3. Thêm { tag: "webhook"; url: string } → xác nhận compiler báo lỗi
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

const assertNever = (x: never): never => {
    throw new Error(`Unexpected: ${JSON.stringify(x)}`);
};

type Notification =
    | { readonly tag: "email"; readonly to: string; readonly subject: string }
    | { readonly tag: "sms"; readonly phone: string; readonly text: string }
    | { readonly tag: "push"; readonly deviceId: string; readonly title: string };

const describe = (n: Notification): string => {
    switch (n.tag) {
        case "email": return `📧 Email to ${n.to}: ${n.subject}`;
        case "sms":   return `📱 SMS to ${n.phone}: ${n.text}`;
        case "push":  return `🔔 Push to ${n.deviceId}: ${n.title}`;
        default:      return assertNever(n);
    }
};

assert.strictEqual(
    describe({ tag: "email", to: "an@mail.com", subject: "Hello" }),
    "📧 Email to an@mail.com: Hello"
);
assert.strictEqual(
    describe({ tag: "sms", phone: "0901234567", text: "Hi" }),
    "📱 SMS to 0901234567: Hi"
);
```

</details>

---

**Bài 3** (15 phút): Type guard + pipeline

Viết hệ thống xử lý đơn hàng:

```typescript
// 1. Type Order = { readonly id: string; readonly total: number; readonly status: "pending" | "paid" | "shipped" | "cancelled" }
// 2. Type guard: isPaid(order) → order is Order & { status: "paid" }
// 3. Pipeline: orders.filter(isPaid).map(o => o.total).reduce(sum)
// 4. Viết type guard isActive(order) — status "pending" | "paid" | "shipped" (không phải "cancelled")
```

<details><summary>💡 Gợi ý</summary>Type guard cho DU variant: kiểm tra `order.status === "paid"`. Cho nhiều variants: dùng `!==` hoặc array `.includes()`.</details>

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type OrderStatus = "pending" | "paid" | "shipped" | "cancelled";

type Order = {
    readonly id: string;
    readonly total: number;
    readonly status: OrderStatus;
};

// Type guard — single status
const isPaid = (order: Order): order is Order & { readonly status: "paid" } =>
    order.status === "paid";

// Type guard — multiple statuses
const isActive = (order: Order): order is Order & { readonly status: "pending" | "paid" | "shipped" } =>
    order.status !== "cancelled";

const orders: readonly Order[] = [
    { id: "O1", total: 50000, status: "paid" },
    { id: "O2", total: 30000, status: "pending" },
    { id: "O3", total: 80000, status: "paid" },
    { id: "O4", total: 20000, status: "cancelled" },
    { id: "O5", total: 60000, status: "shipped" },
];

// Pipeline: tổng đơn đã thanh toán
const paidTotal = orders
    .filter(isPaid)
    .map(o => o.total)
    .reduce((sum, t) => sum + t, 0);

assert.strictEqual(paidTotal, 130000);  // 50000 + 80000

// Pipeline: tổng đơn active (không cancelled)
const activeTotal = orders
    .filter(isActive)
    .map(o => o.total)
    .reduce((sum, t) => sum + t, 0);

assert.strictEqual(activeTotal, 220000);  // 50000 + 30000 + 80000 + 60000

console.log(`Paid total: ${paidTotal.toLocaleString()}đ`);
console.log(`Active total: ${activeTotal.toLocaleString()}đ`);
// Output:
// Paid total: 130,000đ
// Active total: 220,000đ
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `typeof x === "object"` vẫn cho null đi qua | `typeof null === "object"` — bug lịch sử | Thêm `x !== null` |
| Switch không báo lỗi khi thiếu case | Không có `assertNever` ở default, và function không khai báo return type | Thêm `default: return assertNever(x)` hoặc ghi return type rõ ràng |
| Type guard không thu hẹp trong `.filter()` | Guard function thiếu return type `val is T` | Ghi `(val: X): val is Y => ...` |
| `instanceof` không hoạt động với interfaces | `instanceof` chỉ check **runtime constructor** — interfaces không tồn tại runtime | Dùng discriminated unions hoặc `in` operator |
| Assertion function fail khi dùng arrow function | `asserts` keyword chỉ hoạt động với `function` declarations | Dùng `function assertX(val): asserts val is T {}` (lưu ý: `assertNever` dùng arrow OK vì nó trả `never`, không dùng `asserts`) |

---

## Tóm tắt

- ✅ **Narrowing** = thu hẹp type bằng điều kiện. Compiler theo dõi flow code.
- ✅ **`typeof`** cho primitives. **`instanceof`** cho classes. **`in`** cho properties.
- ✅ **Discriminated unions + `switch`** = pattern FP quan trọng nhất. Tag → case → compiler biết type.
- ✅ **Exhaustive checking**: return tự nhiên hoặc `assertNever`. Thêm variant → compiler bắt lỗi.
- ✅ **Type guards** (`val is T`) + `.filter()` = lọc + thu hẹp type. Pattern cực mạnh.
- ✅ **Assertion functions** (`asserts val is T`) = throw nếu sai, thu hẹp cho code sau.

## Tiếp theo

→ Chapter 7: **Functions & Closures** — Arrow functions deep dive, generics `<T>`, higher-order functions, closures, currying, và function composition.
