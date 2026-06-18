# Chapter 9 — Advanced Type System

> **Bạn sẽ học được**:
> - Utility types: `Pick`, `Omit`, `Partial`, `Required`, `Exclude`, `Extract`
> - Mapped types — biến đổi types hàng loạt
> - Conditional types — `T extends U ? A : B` — phân nhánh ở type level
> - Template literal types — string manipulation ở type level
> - Branded types — ngăn structural typing trộn lẫn types
> - `infer` keyword — "bắt" type bên trong pattern
>
> **Yêu cầu trước**: Chapter 8 (objects, interfaces, structural typing, intersections).
> **Thời gian đọc**: ~50 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Sử dụng hệ thống type nâng cao để **encode business rules** vào types.

---

## Advanced Type System — TypeScript's Superpower

Đây là chapter phân biệt TypeScript khỏi JavaScript+annotations. Union types, intersection types, discriminated unions, conditional types, mapped types, template literal types — đây là công cụ mà không ngôn ngữ mainstream nào khác cung cấp ở mức compile-time.

Discriminated unions đặc biệt quan trọng: chúng là **Sum types** — "một trong nhiều khả năng" — nền tảng cho Domain-Driven Design. Pattern `type Shape = Circle | Rectangle | Triangle` với field chung `kind` cho phép TypeScript **narrowing** tự động trong `switch`. Đây là cách TypeScript mô phỏng Rust's exhaustive enums.

Chapter này nặng — bạn có thể bookmark và quay lại sau. Nhưng nắm vững nó = nắm vững TypeScript ở mức senior.


## 9.1 — Utility Types: Công cụ có sẵn

### TypeScript cung cấp sẵn nhiều type helpers

Thay vì tự viết, dùng built-in utility types:

```typescript
// filename: src/utility_types.ts
import assert from "node:assert/strict";

type User = {
    readonly id: string;
    readonly name: string;
    readonly email: string;
    readonly age: number;
};

// === Pick<T, K> — chọn 1 số fields ===
type UserPreview = Pick<User, "id" | "name">;
// = { readonly id: string; readonly name: string }

const preview: UserPreview = { id: "U1", name: "An" };
assert.strictEqual(preview.name, "An");

// === Omit<T, K> — bỏ 1 số fields ===
type UserWithoutEmail = Omit<User, "email">;
// = { readonly id: string; readonly name: string; readonly age: number }

// === Partial<T> — TẤT CẢ fields thành optional ===
type UserUpdate = Partial<User>;
// = { readonly id?: string; readonly name?: string; ... }

const update: UserUpdate = { name: "Bình" };  // chỉ update name

// === Required<T> — TẤT CẢ fields thành required ===
type StrictConfig = Required<{ host?: string; port?: number }>;
// = { host: string; port: number }

console.log("Utility types OK ✅");
```

### `Partial` + immutable update = pattern phổ biến

```typescript
// filename: src/partial_update.ts
import assert from "node:assert/strict";

type User = {
    readonly id: string;
    readonly name: string;
    readonly email: string;
    readonly age: number;
};

// Immutable update — chỉ cập nhật fields được truyền vào
const updateUser = (user: User, changes: Partial<Omit<User, "id">>): User => ({
    ...user,
    ...changes,
});

const an: User = { id: "U1", name: "An", email: "an@mail.com", age: 25 };
const updated = updateUser(an, { name: "An Nguyễn", age: 26 });

assert.strictEqual(an.name, "An");            // gốc KHÔNG ĐỔI
assert.strictEqual(updated.name, "An Nguyễn");
assert.strictEqual(updated.age, 26);
assert.strictEqual(updated.id, "U1");          // id giữ nguyên

console.log(`Updated: ${updated.name}, ${updated.age}`);
// Output: Updated: An Nguyễn, 26
```

> **💡 `Partial<Omit<User, "id">>`**: Cho phép update bất kỳ field nào NGOẠI TRỪ `id` (vì id bất biến). Compose utility types = type-safe APIs.

### `Exclude` và `Extract` — Lọc union types

```typescript
// filename: src/exclude_extract.ts

type Status = "active" | "inactive" | "banned" | "pending";

// Exclude — loại bỏ từ union
type ActiveStatuses = Exclude<Status, "banned" | "inactive">;
// = "active" | "pending"

// Extract — chỉ giữ matches
type DangerStatuses = Extract<Status, "banned" | "inactive">;
// = "banned" | "inactive"

// Thực tế: lọc DU variants
type Event =
    | { readonly tag: "click"; readonly x: number; readonly y: number }
    | { readonly tag: "keypress"; readonly key: string }
    | { readonly tag: "scroll"; readonly offset: number };

// Extract events có field x
type MouseEvent = Extract<Event, { readonly tag: "click" }>;
// = { readonly tag: "click"; readonly x: number; readonly y: number }
```

---

## ✅ Checkpoint 9.1

> Đến đây bạn phải hiểu:
> 1. **`Pick<T, K>`** = chọn fields. **`Omit<T, K>`** = bỏ fields
> 2. **`Partial<T>`** = tất cả optional. **`Required<T>`** = tất cả required
> 3. **`Exclude<T, U>`** = loại khỏi union. **`Extract<T, U>`** = giữ từ union
> 4. **Compose** utility types: `Partial<Omit<User, "id">>` = update pattern
>
> **Test nhanh**: `Pick<{ a: number; b: string; c: boolean }, "a" | "c">` — type gì?
> <details><summary>Đáp án</summary>`{ a: number; c: boolean }`. Pick giữ chỉ fields `a` và `c`.</details>

---

## 9.2 — Mapped Types: Biến đổi types hàng loạt

### Ý tưởng: `.map()` nhưng cho types

Mapped types = lặp qua MỌI keys của một type và biến đổi chúng. Giống `.map()` cho arrays, nhưng ở **type level**.

```typescript
// filename: src/mapped_types.ts

// Readonly<T> THẬT RA là mapped type:
type MyReadonly<T> = {
    readonly [K in keyof T]: T[K];
};
// [K in keyof T] = "lặp qua mọi key K của T"
// T[K] = "giữ nguyên type của key K"
// readonly = "thêm readonly cho mọi field"

// Partial<T> cũng là mapped type:
type MyPartial<T> = {
    [K in keyof T]?: T[K];
};
// ? = "thêm optional cho mọi field"

// Tự tạo: làm mọi field mutable (bỏ readonly)
type Mutable<T> = {
    -readonly [K in keyof T]: T[K];
};
// -readonly = "bỏ readonly"

type User = {
    readonly name: string;
    readonly age: number;
};

type MutableUser = Mutable<User>;
// = { name: string; age: number } — không còn readonly!
```

### Custom mapped types

```typescript
// filename: src/custom_mapped.ts

// Nullable<T> — mọi field có thể null
type Nullable<T> = {
    readonly [K in keyof T]: T[K] | null;
};

type User = {
    readonly name: string;
    readonly age: number;
    readonly email: string;
};

type NullableUser = Nullable<User>;
// = { readonly name: string | null; readonly age: number | null; readonly email: string | null }

// Getters<T> — tạo getter functions cho mọi field
type Getters<T> = {
    readonly [K in keyof T as `get${Capitalize<string & K>}`]: () => T[K];
};

type UserGetters = Getters<User>;
// = { readonly getName: () => string; readonly getAge: () => number; readonly getEmail: () => string }
```

> **💡 Key remapping (`as`)**: `K as \`get${Capitalize<K>}\`` biến key `name` thành `getName`. Mapped types + key remapping = tạo type APIs tự động!

---

## ✅ Checkpoint 9.2

> Đến đây bạn phải hiểu:
> 1. **`[K in keyof T]`** = lặp qua mọi key. **`T[K]`** = type của key đó
> 2. **`readonly`** / **`?`** = thêm modifier. **`-readonly`** / **`-?`** = bỏ modifier
> 3. `Readonly<T>`, `Partial<T>` **LÀ** mapped types — bạn có thể tự viết!
> 4. **Key remapping** (`as`) = đổi tên keys trong mapped type
>
> **Test nhanh**: `type X<T> = { [K in keyof T]-?: T[K] };` — `X` làm gì?
> <details><summary>Đáp án</summary>Bỏ `?` (optional) khỏi TẤT CẢ fields → giống `Required<T>`. `-?` = "remove optional".</details>

---

## 9.3 — Conditional Types: Phân nhánh ở type level

### `T extends U ? A : B` — If/else cho types

```typescript
// filename: src/conditional.ts

// Ternary nhưng cho types!
type IsString<T> = T extends string ? "yes" : "no";

type A = IsString<"hello">;   // "yes"
type B = IsString<42>;        // "no"
type C = IsString<string>;    // "yes"

// Thực tế hơn:
// (Lưu ý: đây chỉ là IMPLEMENTATION rút gọn — TS built-in NonNullable<T> cũng hoạt động giống vậy)
type MyNonNullable<T> = T extends null | undefined ? never : T;
// Nếu T là null/undefined → never (loại bỏ)
// Nếu T là khác → giữ nguyên

type D = MyNonNullable<string | null | undefined>;  // string
type E = MyNonNullable<number | null>;               // number
```

### Distributive conditional types

```typescript
// filename: src/distributive.ts

// Khi T là UNION → conditional type "distribute" qua từng member
type ToArray<T> = T extends unknown ? T[] : never;

type F = ToArray<string | number>;
// = string[] | number[] — KHÔNG phải (string | number)[]!
// Vì: ToArray<string> | ToArray<number> = string[] | number[]

// Nếu muốn KHÔNG distribute → bọc trong tuple:
type ToArrayND<T> = [T] extends [unknown] ? T[] : never;

type G = ToArrayND<string | number>;
// = (string | number)[] — đúng như mong đợi!
```

> **💡 Distributive = tự động**: Conditional type trên naked type parameter sẽ distribute qua union. Đây là feature mạnh nhưng **bất ngờ** — biết để không bị confused.

---

## ✅ Checkpoint 9.3

> Đến đây bạn phải hiểu:
> 1. **`T extends U ? A : B`** = ternary ở type level
> 2. **`NonNullable<T>`** = loại null/undefined — là conditional type
> 3. **Distributive**: union + conditional = áp dụng cho TỪNG member
> 4. **Tránh distribute**: bọc `[T] extends [U]`
>
> **Test nhanh**: `type X<T> = T extends string ? T : never;` — `X<"a" | 42 | "b">` = gì?
> <details><summary>Đáp án</summary>`"a" | "b"`. Distributive: `X<"a">` = `"a"`, `X<42>` = `never`, `X<"b">` = `"b"`. `never` biến mất trong union.</details>

---

## 9.4 — `infer` — "Bắt" type bên trong pattern

### Ý tưởng: Regex cho types

`infer` = khai báo type variable **bên trong** conditional type. TypeScript "bắt" (infer) type tại vị trí đó.

```typescript
// filename: src/infer.ts

// "Bắt" return type của function
// Lưu ý: dùng `any[]` cho args vì `unknown[]` không assignable cho specific param types
// (đây là 1 trong số ít chỗ `any` hợp lý — TS built-in ReturnType cũng dùng `any`)
type ReturnOf<T> = T extends (...args: any[]) => infer R ? R : never;

type A = ReturnOf<() => string>;              // string
type B = ReturnOf<(x: number) => boolean>;    // boolean
type C = ReturnOf<string>;                    // never — không phải function

// "Bắt" element type của array
type ElementOf<T> = T extends readonly (infer E)[] ? E : never;

type D = ElementOf<string[]>;        // string
type E2 = ElementOf<readonly [1, "a", true]>;  // 1 | "a" | true

// "Bắt" type argument từ generic
type UnwrapPromise<T> = T extends Promise<infer V> ? V : T;

type F = UnwrapPromise<Promise<number>>;  // number
type G = UnwrapPromise<string>;           // string (không phải Promise → giữ nguyên)
```

### `infer` thực tế: Extract DU variant

```typescript
// filename: src/infer_du.ts

type Event =
    | { readonly tag: "click"; readonly x: number; readonly y: number }
    | { readonly tag: "keypress"; readonly key: string }
    | { readonly tag: "scroll"; readonly offset: number };

// Extract payload (loại tag) từ một variant cụ thể
type EventPayload<Tag extends Event["tag"]> =
    Omit<Extract<Event, { readonly tag: Tag }>, "tag">;

type ClickPayload = EventPayload<"click">;
// = { readonly x: number; readonly y: number }

type KeyPayload = EventPayload<"keypress">;
// = { readonly key: string }

// Cách lấy nguyên variant (đã giới thiệu ở 9.1 với Extract):
type VariantOf<T, Tag> = Extract<T, { readonly tag: Tag }>;

type ClickEvent = VariantOf<Event, "click">;
// = { readonly tag: "click"; readonly x: number; readonly y: number }
```

> **💡 `infer` = type-level pattern matching**: Bạn nói "nếu T có shape này → bắt phần X". Rất mạnh cho library types, nhưng **dễ phức tạp** — dùng khi utility types built-in không đủ.

---

## ✅ Checkpoint 9.4

> Đến đây bạn phải hiểu:
> 1. **`infer R`** = "bắt type tại vị trí R" trong conditional type
> 2. **`ReturnType<T>`**, **`Parameters<T>`** LÀ `infer`-based (built-in)
> 3. `infer` = pattern matching: `T extends X<infer V>` → "nếu T match X, bắt V"
> 4. Dùng khi: unwrap generics, extract return/params, analyze complex types
>
> **Test nhanh**: `type X<T> = T extends Promise<infer V> ? V : T;` — `X<Promise<string[]>>` = gì?
> <details><summary>Đáp án</summary>`string[]`. `Promise<string[]>` match `Promise<infer V>` → V = `string[]`.</details>

---

## 9.5 — Template Literal Types

### String manipulation ở type level

```typescript
// filename: src/template_literal.ts

// Template literal types — string patterns ở type level
type Greeting = `Hello ${string}`;

const a: Greeting = "Hello An";       // ✅
const b: Greeting = "Hello World";    // ✅
// const c: Greeting = "Hi An";       // ❌ Phải bắt đầu bằng "Hello "

// Union + template literal = tạo combinations
type Color = "red" | "green" | "blue";
type Size = "sm" | "md" | "lg";

type ClassName = `${Color}-${Size}`;
// = "red-sm" | "red-md" | "red-lg" | "green-sm" | ... (9 combinations!)

const cls: ClassName = "red-lg";       // ✅
// const bad: ClassName = "red-xl";    // ❌ "xl" không trong Size
```

### Built-in string utilities

```typescript
// filename: src/string_utils.ts

type Event = "click" | "scroll" | "keypress";

// Capitalize — viết hoa chữ đầu
type Handler = `on${Capitalize<Event>}`;
// = "onClick" | "onScroll" | "onKeypress"

// Uppercase / Lowercase
type Upper = Uppercase<"hello">;    // "HELLO"
type Lower = Lowercase<"HELLO">;    // "hello"

// Uncapitalize
type Uncap = Uncapitalize<"Hello">;  // "hello"

// Thực tế: tạo event handler types tự động
type EventHandlers = {
    readonly [E in Event as `on${Capitalize<E>}`]: (event: E) => void;
};
// = {
//     readonly onClick: (event: "click") => void;
//     readonly onScroll: (event: "scroll") => void;
//     readonly onKeypress: (event: "keypress") => void;
// }
```

> **💡 Template literals + Mapped types = code generation**: Kết hợp 2 features → tự động tạo type-safe APIs từ string patterns. Dùng thường xuyên trong library types.

---

## ✅ Checkpoint 9.5

> Đến đây bạn phải hiểu:
> 1. **`` `${A}-${B}` ``** = template literal type. Union → tạo MỌI combinations
> 2. **`Capitalize`**, **`Uppercase`**, **`Lowercase`** = string type utilities
> 3. **Mapped type + template literal** = auto-generate type-safe APIs
>
> **Test nhanh**: `type X = \`${"a"|"b"}-${"1"|"2"}\`` — bao nhiêu members?
> <details><summary>Đáp án</summary>4 members: `"a-1" | "a-2" | "b-1" | "b-2"`. 2 × 2 = 4 combinations.</details>

---

## 9.6 — Branded Types: Ngăn trộn lẫn

### Vấn đề: Structural typing quá linh hoạt

Chapter 8 đã cảnh báo — types cùng structure nhưng khác ý nghĩa bị trộn lẫn:

```typescript
// ❌ Cả hai đều là string — compiler không phân biệt!
type UserId = string;
type OrderId = string;

const getUser = (id: UserId): string => `User ${id}`;

const orderId: OrderId = "ORD-001";
getUser(orderId);  // ✅ Nhưng SAI! — orderId không phải userId
```

### Giải pháp: Branded types

```typescript
// filename: src/branded.ts
import assert from "node:assert/strict";

// Brand = symbol ẩn, KHÔNG tồn tại runtime
// Chỉ tồn tại ở TYPE LEVEL — không ảnh hưởng performance

type Brand<T, B extends string> = T & { readonly __brand: B };

type UserId = Brand<string, "UserId">;
type OrderId = Brand<string, "OrderId">;
type Email = Brand<string, "Email">;

// Constructor functions — nơi DUY NHẤT tạo branded values
const UserId = (id: string): UserId => id as UserId;
const OrderId = (id: string): OrderId => id as OrderId;
const Email = (email: string): Email => {
    if (!email.includes("@")) throw new Error(`Invalid email: ${email}`);
    return email as Email;
};

// Giờ compiler PHÂN BIỆT!
const getUser = (id: UserId): string => `User ${id}`;

const userId = UserId("USR-001");
const orderId = OrderId("ORD-001");

getUser(userId);    // ✅ OK
// getUser(orderId); // ❌ Error: OrderId not assignable to UserId!
// getUser("raw");   // ❌ Error: string not assignable to UserId!

// Email validation tại constructor
const email = Email("an@mail.com");  // ✅
// Email("invalid");                  // 💥 throw Error!

assert.strictEqual(getUser(userId), "User USR-001");
console.log("Branded types OK ✅");
```

### Branded types cho domain safety

```typescript
// filename: src/branded_domain.ts
import assert from "node:assert/strict";

type Brand<T, B extends string> = T & { readonly __brand: B };

// Tiền — không trộn VND và USD!
type VND = Brand<number, "VND">;
type USD = Brand<number, "USD">;

const VND = (amount: number): VND => amount as VND;
const USD = (amount: number): USD => amount as USD;

const displayVND = (amount: VND): string =>
    `${amount.toLocaleString()}đ`;

const displayUSD = (amount: USD): string =>
    `$${amount.toFixed(2)}`;

const price = VND(50000);
const salary = USD(3000);

assert.strictEqual(displayVND(price), "50,000đ");
assert.strictEqual(displayUSD(salary), "$3000.00");

// displayVND(salary);  // ❌ Error: USD not assignable to VND!
// displayUSD(price);   // ❌ Error: VND not assignable to USD!

// Chuyển đổi — phải TƯỜNG MINH
const vndToUsd = (amount: VND, rate: number): USD =>
    USD(amount / rate);

const converted = vndToUsd(price, 25000);
assert.strictEqual(displayUSD(converted), "$2.00");

console.log(`${displayVND(price)} = ${displayUSD(converted)}`);
// Output: 50,000đ = $2.00
```

> **💡 Branded types = compile-time safety WITHOUT runtime cost**: `__brand` chỉ tồn tại ở type level — runtime vẫn là `number`/`string` bình thường. Không ảnh hưởng performance!

---

## ✅ Checkpoint 9.6

> Đến đây bạn phải hiểu:
> 1. **Branded types** = `T & { __brand: B }` — phân biệt types cùng structure
> 2. **Constructor functions** = nơi duy nhất cast `as Brand` — validation point
> 3. **Zero runtime cost** — `__brand` chỉ ở type level
> 4. Use cases: IDs (UserId vs OrderId), tiền (VND vs USD), validated strings (Email)
>
> **Test nhanh**: `type X = Brand<number, "X">; type Y = Brand<number, "Y">;` — X gán cho Y được không?
> <details><summary>Đáp án</summary>KHÔNG! Mặc dù cả hai runtime là `number`, brands khác nhau → compiler cấm. Đây là mục đích chính.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Utility types

Dùng utility types để tạo type cho update form:

```typescript
type Product = {
    readonly id: string;
    readonly name: string;
    readonly price: number;
    readonly category: string;
    readonly createdAt: Date;
};

// 1. UpdateProduct = Partial, nhưng KHÔNG cho update id và createdAt
// 2. ProductPreview = chỉ id, name, price
// Dùng Pick, Omit, Partial — KHÔNG viết fields thủ công
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
type UpdateProduct = Partial<Omit<Product, "id" | "createdAt">>;
// = { name?: string; price?: number; category?: string }

type ProductPreview = Pick<Product, "id" | "name" | "price">;
// = { readonly id: string; readonly name: string; readonly price: number }
```

</details>

---

**Bài 2** (10 phút): Custom mapped type

Viết `DeepReadonly<T>` — đệ quy readonly mọi tầng:

```typescript
// DeepReadonly<{ a: { b: { c: number } } }>
// → { readonly a: { readonly b: { readonly c: number } } }

// Hint: Nếu T[K] là object → đệ quy. Nếu primitive → giữ nguyên.
```

<details><summary>💡 Gợi ý</summary>`T[K] extends object ? DeepReadonly<T[K]> : T[K]` — nhưng cần exclude `Function` và `Date` khỏi đệ quy.</details>

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
type DeepReadonly<T> = T extends Function
    ? T
    : T extends Date
    ? T
    : T extends ReadonlyArray<infer E>
    ? ReadonlyArray<DeepReadonly<E>>
    : T extends object
    ? { readonly [K in keyof T]: DeepReadonly<T[K]> }
    : T;

// Test
type Nested = {
    a: number;
    b: {
        c: string;
        d: {
            e: boolean;
        };
    };
    fn: (x: number) => void;
    date: Date;
};

type ReadonlyNested = DeepReadonly<Nested>;
// a: readonly number ✅
// b.c: readonly string ✅
// b.d.e: readonly boolean ✅
// fn: (x: number) => void ✅ (không đệ quy Function)
// date: Date ✅ (không đệ quy Date)
```

</details>

---

**Bài 3** (15 phút): Branded types cho e-commerce

Viết type-safe pricing system:

```typescript
// 1. Branded types: VND, USD, Percentage
// 2. Constructor với validation:
//    - VND: phải >= 0
//    - USD: phải >= 0
//    - Percentage: phải 0-100
// 3. Functions:
//    - applyDiscount(price: VND, discount: Percentage): VND
//    - vndToUsd(amount: VND, rate: number): USD
//    - formatVND(amount: VND): string
//    - formatUSD(amount: USD): string
// 4. KHÔNG cho trộn: applyDiscount(usdPrice, ...) phải compile error!
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Brand<T, B extends string> = T & { readonly __brand: B };

type VND = Brand<number, "VND">;
type USD = Brand<number, "USD">;
type Percentage = Brand<number, "Percentage">;

const VND = (amount: number): VND => {
    if (amount < 0) throw new RangeError(`VND must be >= 0, got ${amount}`);
    return amount as VND;
};

const USD = (amount: number): USD => {
    if (amount < 0) throw new RangeError(`USD must be >= 0, got ${amount}`);
    return amount as USD;
};

const Percentage = (value: number): Percentage => {
    if (value < 0 || value > 100) {
        throw new RangeError(`Percentage must be 0-100, got ${value}`);
    }
    return value as Percentage;
};

const applyDiscount = (price: VND, discount: Percentage): VND =>
    VND(Math.round(price * (1 - discount / 100)));

const vndToUsd = (amount: VND, rate: number): USD =>
    USD(Number((amount / rate).toFixed(2)));

const formatVND = (amount: VND): string =>
    `${amount.toLocaleString()}đ`;

const formatUSD = (amount: USD): string =>
    `$${amount.toFixed(2)}`;

// Test
const price = VND(500000);
const discount = Percentage(20);
const discounted = applyDiscount(price, discount);

assert.strictEqual(formatVND(discounted), "400,000đ");

const usd = vndToUsd(price, 25000);
assert.strictEqual(formatUSD(usd), "$20.00");

// applyDiscount(usd, discount);  // ❌ Error: USD not assignable to VND!

console.log(`Original: ${formatVND(price)}`);
console.log(`After 20% off: ${formatVND(discounted)}`);
console.log(`In USD: ${formatUSD(usd)}`);
// Output:
// Original: 500,000đ
// After 20% off: 400,000đ
// In USD: $20.00
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Mapped type bỏ readonly/optional | Không thêm modifiers | Ghi `readonly [K in keyof T]: T[K]` rõ ràng |
| Conditional type distribute bất ngờ | Naked type parameter trong condition | Bọc `[T] extends [U]` để tránh distribute |
| `infer` không bắt đúng type | Pattern match sai | Kiểm tra shape: `T extends X<infer V>` — X phải đúng |
| Template literal quá nhiều combinations | Union × Union = tích Descartes | Giới hạn union size, dùng constraints |
| Branded type error cast | Quên `as Brand` trong constructor | Mọi branded value phải qua constructor function |

---

## Tóm tắt

- ✅ **Utility types**: `Pick`, `Omit`, `Partial`, `Required`, `Exclude`, `Extract` — compose được!
- ✅ **Mapped types**: `[K in keyof T]` = lặp qua keys. `-readonly`, `-?` = bỏ modifiers.
- ✅ **Conditional types**: `T extends U ? A : B`. Distributive trên unions.
- ✅ **`infer`** = pattern matching ở type level. Bắt type từ trong pattern.
- ✅ **Template literal types**: `` `${A}-${B}` `` + `Capitalize` = string type manipulation.
- ✅ **Branded types**: `T & { __brand: B }` = phân biệt types cùng structure. Zero runtime cost.

## Tiếp theo

→ Chapter 10: **Modules & Project Structure** — ES modules, barrel files (`index.ts`), monorepo patterns, và tổ chức code FP project lớn.
