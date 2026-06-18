# Chapter 10 — Modules & Project Structure

> **Bạn sẽ học được**:
> - ES Modules — `import`/`export` trong TypeScript
> - Named exports vs default exports — quy tắc FP
> - Type-only imports — `import type { T }` tối ưu bundle
> - Barrel files (`index.ts`) — tổ chức public API
> - Project structure — cách tổ chức FP project thực tế
> - Dependency layers — tránh circular imports
>
> **Yêu cầu trước**: Chapter 9 (advanced types, branded types).
> **Thời gian đọc**: ~35 phút | **Level**: Beginner → Intermediate
> **Kết quả cuối cùng**: Tổ chức project TypeScript/FP sạch sẽ, dễ mở rộng, không circular dependencies.

---

## Modules & Project Structure — Tổ chức code ở mức production

ES Modules (`import`/`export`) trong TypeScript không chỉ là cách chia file — đó là cách thiết kế **boundaries**: mỗi module có public API (exports) và private implementation (everything else). Consumer chỉ thấy API, không phụ thuộc vào chi tiết bên trong.

Chapter này dạy bạn tổ chức TypeScript project cho production: barrel files (`index.ts`) để clean imports, `tsconfig.json` paths cho absolute imports, và monorepo patterns khi project vượt quá 1 package. Nền tảng cho mọi chapter sau.


## 10.1 — ES Modules: Import/Export

### Named exports — Cách chính

```typescript
// filename: src/math.ts

// Named exports — export từng item riêng
export const add = (a: number, b: number): number => a + b;
export const multiply = (a: number, b: number): number => a * b;

export type Point = {
    readonly x: number;
    readonly y: number;
};

export const origin: Point = { x: 0, y: 0 };
```

```typescript
// filename: src/app.ts

// Named imports — chọn chính xác những gì cần
import { add, multiply, type Point } from "./math.js";
// Lưu ý: import path dùng ".js" extension — TypeScript emit JS files

const result = add(3, 4);

const p: Point = { x: 1, y: 2 };
```

> **⚠️ `.js` extension**: TypeScript imports dùng `.js` (không phải `.ts`) vì compiler emit JS files. `import from "./math.js"` sẽ resolve tới `./math.ts` khi compile, và `./math.js` khi chạy. Lưu ý: bundlers như Vite/webpack có thể tự resolve mà không cần extension — nhưng luôn ghi `.js` cho code chạy trực tiếp trên Node.js.

### Re-export pattern

```typescript
// filename: src/utils/string.ts
export const capitalize = (s: string): string =>
    s.charAt(0).toUpperCase() + s.slice(1);

export const trim = (s: string): string => s.trim();
```

```typescript
// filename: src/utils/number.ts
export const clamp = (value: number, min: number, max: number): number =>
    Math.min(Math.max(value, min), max);

export const isEven = (n: number): boolean => n % 2 === 0;
```

```typescript
// filename: src/utils/index.ts

// Re-export — gom mọi thứ vào 1 entry point
export { capitalize, trim } from "./string.js";
export { clamp, isEven } from "./number.js";
```

```typescript
// filename: src/main.ts

// Import từ barrel file — sạch sẽ!
import { capitalize, clamp } from "./utils/index.js";
```

---

## ✅ Checkpoint 10.1

> Đến đây bạn phải hiểu:
> 1. **Named exports** = `export const f = ...`. Import: `import { f } from "./file.js"`
> 2. **`.js` extension** trong imports — TypeScript resolve `.ts` khi compile
> 3. **Re-export** = gom exports qua barrel file (`index.ts`)
>
> **Test nhanh**: `import { add } from "./math"` (không có `.js`) — có lỗi khi runtime không?
> <details><summary>Đáp án</summary>CÓ THỂ! Node.js ESM cần extension. Nếu dùng ts-node hoặc tsx thì OK, nhưng code compiled bởi tsc sẽ fail vì thiếu `.js`. Luôn ghi `.js`.</details>

---

## 10.2 — Named vs Default Exports

### Default export — Quen nhưng có vấn đề

```typescript
// filename: src/user.ts

// ❌ Default export
export default class UserService {
    // ...
}
```

```typescript
// filename: src/app.ts

// Import default — tên tùy ý, dễ nhầm!
import UserService from "./user.js";
import US from "./user.js";        // cùng file, khác tên — confusing!
import Banana from "./user.js";    // TypeScript không báo lỗi!
```

### Tại sao FP ưu tiên named exports

| | Named export | Default export |
|---|---|---|
| Import tên | **Phải đúng** (hoặc `as` rename) | **Tùy ý** — dễ nhầm |
| Refactor | ✅ IDE rename tự động | ❌ Tên import tùy ý, IDE không track |
| Multiple exports | ✅ Nhiều items/file | ⚠️ Chỉ 1 default/file |
| Tree-shaking | ✅ Bundler biết chính xác | ⚠️ Khó hơn |
| Quy tắc sách | ✅ **Luôn dùng** | ❌ Tránh |

```typescript
// ✅ Named exports — quy tắc của sách
// Một file có thể export nhiều functions liên quan

// filename: src/user.ts
export type User = {
    readonly id: string;
    readonly name: string;
};

export const createUser = (id: string, name: string): User => ({ id, name });
export const greetUser = (user: User): string => `Hello ${user.name}`;
```

> **💡 Quy tắc**: Luôn dùng **named exports**. Không default export. Mỗi file export **1 nhóm chức năng liên quan** — types + functions cho nhóm đó.

---

## ✅ Checkpoint 10.2

> Đến đây bạn phải hiểu:
> 1. **Named exports** = tên bắt buộc, IDE refactor-friendly
> 2. **Default exports** = tên tùy ý, dễ nhầm, khó refactor
> 3. **Quy tắc FP**: luôn named. Một file = 1 nhóm chức năng
>
> **Test nhanh**: Bạn refactor `createUser` thành `makeUser`. Named export vs default — cái nào IDE rename hết tự động?
> <details><summary>Đáp án</summary>**Named export**. IDE tìm mọi `import { createUser }` và đổi thành `{ makeUser }`. Default export thì mỗi file import tên khác nhau — IDE không biết đổi thành gì.</details>

---

## 10.3 — Type-Only Imports

### `import type` — Chỉ import types, không import runtime code

```typescript
// filename: src/types.ts
export type User = {
    readonly id: string;
    readonly name: string;
    readonly age: number;
};

export type Result<T> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: string };

// Constant — VÀ runtime value
export const MAX_AGE = 150;
```

```typescript
// filename: src/service.ts

// import type — CHỈ types, không có trong JS output
import type { User, Result } from "./types.js";
// import runtime value bình thường
import { MAX_AGE } from "./types.js";

export const validateAge = (age: number): Result<number> => {
    if (age < 0 || age > MAX_AGE) {
        return { tag: "err", error: `Age must be 0-${MAX_AGE}` };
    }
    return { tag: "ok", value: age };
};
```

### Tại sao quan trọng?

```typescript
// ❌ Không dùng import type — có thể import runtime code không cần thiết
import { User, Result, MAX_AGE } from "./types.js";
// Bundler có thể giữ lại "./types.js" vì có import

// ✅ Tách rõ type vs runtime
import type { User, Result } from "./types.js";  // biến mất trong JS
import { MAX_AGE } from "./types.js";              // giữ lại

// Hoặc dùng inline type modifier:
import { type User, type Result, MAX_AGE } from "./types.js";
```

> **💡 `import type` = intent declaration**: Nói rõ "tôi chỉ cần type, không cần runtime". Giúp bundler tree-shake tốt hơn. TypeScript có thể enforce bằng `"verbatimModuleSyntax": true` trong tsconfig.

---

## ✅ Checkpoint 10.3

> Đến đây bạn phải hiểu:
> 1. **`import type`** = chỉ types, biến mất trong JS output
> 2. **Inline syntax**: `import { type User, MAX_AGE }` = hỗn hợp type + runtime
> 3. **`verbatimModuleSyntax`** trong tsconfig = enforce tách type imports
>
> **Test nhanh**: `import type { MAX_AGE } from "./types.js"` rồi dùng `MAX_AGE` trong code — compile lỗi hay runtime lỗi?
> <details><summary>Đáp án</summary>**Compile lỗi**. `MAX_AGE` là runtime value (const), nhưng `import type` loại bỏ nó khỏi JS output. TypeScript báo lỗi: `'MAX_AGE' is a value and cannot be used with 'import type'`.</details>

---

## 10.4 — Project Structure: Tổ chức FP Project

### Structure theo feature, không theo type

```
// ❌ Tổ chức theo loại file — khó navigate
src/
  types/
    user.ts
    order.ts
    product.ts
  functions/
    user.ts
    order.ts
    product.ts
  utils/
    user.ts
    ...

// ✅ Tổ chức theo FEATURE — files liên quan gần nhau
src/
  user/
    types.ts        ← User types + branded types
    operations.ts   ← Pure functions xử lý User
    validation.ts   ← Validation functions
    index.ts        ← Public API (re-exports)
  order/
    types.ts
    operations.ts
    index.ts
  shared/
    types.ts        ← Result<T>, Option<T>, Brand<T,B>
    utils.ts        ← assertNever, pipe, compose
    index.ts
  main.ts           ← Entry point
```

### Ví dụ thực tế

```typescript
// filename: src/shared/types.ts

export type Result<T> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: string };

export type Brand<T, B extends string> = T & { readonly __brand: B };
```

```typescript
// filename: src/shared/utils.ts

export const assertNever = (x: never): never => {
    throw new Error(`Unexpected value: ${JSON.stringify(x)}`);
};
```

```typescript
// filename: src/shared/index.ts

export type { Result, Brand } from "./types.js";
export { assertNever } from "./utils.js";
```

```typescript
// filename: src/user/types.ts

import type { Brand } from "../shared/index.js";

export type UserId = Brand<string, "UserId">;
export const UserId = (id: string): UserId => id as UserId;

export type User = {
    readonly id: UserId;
    readonly name: string;
    readonly email: string;
    readonly age: number;
};
```

```typescript
// filename: src/user/operations.ts

import type { Result } from "../shared/index.js";
import type { User, UserId } from "./types.js";
import { UserId as createUserId } from "./types.js";

export const createUser = (
    name: string,
    email: string,
    age: number
): Result<User> => {
    if (name.length === 0) return { tag: "err", error: "Name required" };
    if (age < 0 || age > 150) return { tag: "err", error: "Invalid age" };

    return {
        tag: "ok",
        value: {
            id: createUserId(`USR-${Date.now()}`),
            name,
            email,
            age,
        },
    };
};

export const greetUser = (user: User): string =>
    `Hello ${user.name} (${user.id})`;
```

```typescript
// filename: src/user/index.ts

// Public API — chỉ export những gì bên ngoài cần
export type { User, UserId } from "./types.js";
export { UserId } from "./types.js";
export { createUser, greetUser } from "./operations.js";
```

```typescript
// filename: src/main.ts
import { createUser, greetUser } from "./user/index.js";

const result = createUser("An", "an@mail.com", 25);
if (result.tag === "ok") {
    console.log(greetUser(result.value));
}
```

> **💡 Feature folders**: Mỗi folder = 1 domain concept. `index.ts` = public API. Internal files (`operations.ts`, `validation.ts`) = implementation details. Bên ngoài chỉ thấy qua `index.ts`.

---

## ✅ Checkpoint 10.4

> Đến đây bạn phải hiểu:
> 1. **Feature-based** structure > file-type-based structure
> 2. **`index.ts`** = public API. Internal files ẩn phía sau
> 3. **`shared/`** = types + utilities dùng chung
> 4. Mỗi feature folder: `types.ts` + `operations.ts` + `index.ts`
>
> **Test nhanh**: File `src/user/operations.ts` import từ `../shared/index.js` — tại sao không import trực tiếp `../shared/types.js`?
> <details><summary>Đáp án</summary>Import qua `index.js` (barrel file) = decouple. Nếu shared thay đổi internal structure (tách `types.ts` thành nhiều files), imports KHÔNG cần sửa. `index.ts` = stable interface.</details>

---

## 10.5 — Dependency Layers: Tránh Circular Imports

### Vấn đề: Circular dependencies

```typescript
// ❌ user.ts imports order.ts
// ❌ order.ts imports user.ts
// → Circular dependency! Runtime: có thể nhận undefined

// filename: src/user.ts
import { Order } from "./order.js";  // ← depends on order
export type User = { name: string; orders: Order[] };

// filename: src/order.ts
import { User } from "./user.js";   // ← depends on user
export type Order = { owner: User; total: number };
// → Circular!
```

### Giải pháp: Dependency layers

```
Layer 3: Features (user, order, payment)
    ↓ import
Layer 2: Domain types (types dùng chung giữa features)
    ↓ import
Layer 1: Shared (Result, Brand, utils)
    ↓ import
Layer 0: External (node:assert, npm packages)

Quy tắc: Arrows CHỈ đi XUỐNG. Không bao giờ đi ngược lên!
```

```typescript
// ✅ Giải quyết circular: tách shared types ra layer riêng

// filename: src/domain/types.ts  ← Layer 2
// (Dùng string đơn giản ở đây — trong thực tế, dùng branded types từ Ch9)
export type UserId = string;
export type OrderId = string;

export type User = {
    readonly id: UserId;
    readonly name: string;
};

export type Order = {
    readonly id: OrderId;
    readonly userId: UserId;  // reference bằng ID, không embed object!
    readonly total: number;
};
```

```typescript
// filename: src/user/operations.ts  ← Layer 3
import type { User, UserId } from "../domain/types.js";
// Không import order — tránh circular!

export const findUser = (
    users: readonly User[],
    id: UserId
): User | undefined =>
    users.find(u => u.id === id);
```

```typescript
// filename: src/order/operations.ts  ← Layer 3
import type { Order, UserId } from "../domain/types.js";
// Không import user — tránh circular!

export const userOrders = (
    orders: readonly Order[],
    userId: UserId
): readonly Order[] =>
    orders.filter(o => o.userId === userId);
```

> **💡 "Reference by ID, not by embedding"**: Thay vì `order.owner: User` (embed → circular), dùng `order.userId: UserId` (reference → no circular). Giống database foreign keys!

---

## ✅ Checkpoint 10.5

> Đến đây bạn phải hiểu:
> 1. **Circular imports** = A imports B, B imports A → runtime bugs
> 2. **Dependency layers**: arrows chỉ đi XUỐNG (shared ← domain ← features)
> 3. **Reference by ID** thay vì embed objects — phá vỡ circular
> 4. **Layer 0** = external. **Layer 1** = shared utils. **Layer 2** = domain types. **Layer 3** = features
>
> **Test nhanh**: `src/user/` import từ `src/order/` — vi phạm quy tắc nào?
> <details><summary>Đáp án</summary>Vi phạm layer rule! Cả hai ở Layer 3 — features không nên import lẫn nhau. Tách shared types xuống Layer 2.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Module organization

Cho các items sau, hãy sắp xếp vào file structure:

```typescript
// Items:
// - type Product = { id: string; name: string; price: number }
// - type CartItem = { product: Product; quantity: number }
// - type Cart = { items: readonly CartItem[] }
// - const addToCart = (cart: Cart, item: CartItem): Cart => ...
// - const cartTotal = (cart: Cart): number => ...
// - type Result<T> = ...
// - const assertNever = (x: never): never => ...

// Vào structure:
// src/
//   shared/???
//   cart/???
//   product/???
```

<details><summary>✅ Lời giải Bài 1</summary>

```
src/
  shared/
    types.ts        → Result<T>
    utils.ts        → assertNever
    index.ts        → re-export cả hai
  product/
    types.ts        → type Product
    index.ts        → export type { Product }
  cart/
    types.ts        → type CartItem, type Cart (import Product)
    operations.ts   → addToCart, cartTotal
    index.ts        → export types + operations
```

Key decisions:
- `Product` ở `product/` vì là domain concept riêng
- `CartItem` và `Cart` ở `cart/` vì là cart domain
- `cart/types.ts` import `Product` từ `product/index.js` — OK vì chỉ import TYPE (type-only import không tạo runtime dependency, không circular)

</details>

---

**Bài 2** (10 phút): Barrel file

Viết barrel files cho structure sau:

```typescript
// src/payment/
//   types.ts    → PaymentMethod, PaymentResult
//   process.ts  → processPayment(method: PaymentMethod): PaymentResult
//   validate.ts → validateCard(card: string): Result<string>
//   format.ts   → formatPayment(result: PaymentResult): string

// Viết src/payment/index.ts:
// 1. Export TẤT CẢ public types
// 2. Export TẤT CẢ public functions
// 3. KHÔNG export internal helpers (nếu có)
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
// filename: src/payment/index.ts

// Types — dùng export type cho types
export type { PaymentMethod, PaymentResult } from "./types.js";

// Public functions
export { processPayment } from "./process.js";
export { validateCard } from "./validate.js";
export { formatPayment } from "./format.js";

// Usage bên ngoài:
// import { processPayment, formatPayment, type PaymentMethod } from "./payment/index.js";
```

</details>

---

**Bài 3** (15 phút): Dependency layers

Refactor circular dependency:

```typescript
// ❌ Current (circular!):
// src/teacher.ts
import type { Student } from "./student.js";
export type Teacher = { name: string; students: Student[] };

// src/student.ts
import type { Teacher } from "./teacher.js";
export type Student = { name: string; teacher: Teacher };

// Refactor:
// 1. Tạo Layer 2 domain types — reference bằng IDs
// 2. Tạo Layer 3 teacher/ và student/ features — KHÔNG import lẫn nhau
// 3. Function: getTeacherStudents(teacherId, allStudents) — trả students của teacher
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
// filename: src/domain/types.ts  ← Layer 2

import type { Brand } from "../shared/index.js";

export type TeacherId = Brand<string, "TeacherId">;
export type StudentId = Brand<string, "StudentId">;

export type Teacher = {
    readonly id: TeacherId;
    readonly name: string;
    // Reference bằng IDs — không embed Student[]!
};

export type Student = {
    readonly id: StudentId;
    readonly name: string;
    readonly teacherId: TeacherId;  // FK — không embed Teacher!
};
```

```typescript
// filename: src/teacher/operations.ts  ← Layer 3
import type { Teacher, TeacherId, Student } from "../domain/types.js";

export const getTeacherStudents = (
    teacherId: TeacherId,
    allStudents: readonly Student[]
): readonly Student[] =>
    allStudents.filter(s => s.teacherId === teacherId);
```

```typescript
// filename: src/student/operations.ts  ← Layer 3
import type { Student, Teacher, TeacherId } from "../domain/types.js";

export const getStudentTeacher = (
    student: Student,
    allTeachers: readonly Teacher[]
): Teacher | undefined =>
    allTeachers.find(t => t.id === student.teacherId);
```

No circular imports! Cả hai features import từ `domain/types.ts` (Layer 2), không import lẫn nhau.

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `Cannot find module './math'` | Thiếu `.js` extension | Ghi `"./math.js"` — TS resolve `.ts` khi compile |
| Circular dependency warning | Feature A import Feature B và ngược lại | Tách shared types xuống layer thấp hơn |
| `import type` bị strip nhưng cần runtime | Nhầm `import type` cho runtime value | Dùng `import { type T, value }` — tách rõ |
| Barrel file import chậm | Barrel re-export MỌI thứ — bundler load hết | Chỉ re-export public API. Import trực tiếp khi cần performance |
| tsc output thiếu files | File không được import từ entry point | Mọi file phải reachable từ entry point (hoặc thêm vào `include`) |

---

## Tóm tắt

- ✅ **Named exports** > default exports. Mọi file dùng `export const/type`.
- ✅ **`import type`** = chỉ types, biến mất trong JS output. Tối ưu bundle.
- ✅ **Barrel files** (`index.ts`) = public API. Internal files ẩn phía sau.
- ✅ **Feature-based** structure: `user/`, `order/`, `shared/`. Mỗi folder = 1 domain.
- ✅ **Dependency layers**: arrows chỉ đi XUỐNG. Reference by ID, not by embedding.
- ✅ **`.js` extension** trong imports — TypeScript resolve `.ts` khi compile.

---

## 🎉 Part I Complete!

Xin chúc mừng! Bạn đã hoàn thành **Part I: TypeScript Fundamentals** — 7 chapters (4–10) covering:

| Chapter | Nội dung |
|---------|----------|
| **4** | Setup: Node.js, npm, tsconfig, `strict: true` |
| **5** | Types: `const`, inference, primitives, `unknown`, `readonly` |
| **6** | Control Flow: narrowing, DU + switch, exhaustive checking, type guards |
| **7** | Functions: arrow functions, generics, HOFs, closures, currying |
| **8** | Objects: `type` vs `interface`, structural typing, intersections, classes |
| **9** | Advanced Types: utility types, mapped, conditional, `infer`, branded |
| **10** | Modules: ES modules, barrel files, feature folders, dependency layers |

Bạn đã có **mọi công cụ TypeScript cần thiết** để bắt đầu tư duy FP. Part II sẽ dạy bạn **cách nghĩ** — immutability, purity, error handling, và patterns FP thực tế.

## Tiếp theo

→ Part II: **Thinking Functionally** — Chapter 11: **Immutability & Purity**. Tất cả mọi concept từ Part I sẽ được áp dụng vào tư duy FP thật sự.
