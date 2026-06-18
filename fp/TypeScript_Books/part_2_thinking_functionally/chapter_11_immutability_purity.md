# Chapter 11 — Immutability & Purity

> **Bạn sẽ học được**:
> - Tại sao immutability là nền tảng FP — "không bao giờ thay đổi, chỉ tạo mới"
> - 4 tầng immutability: `const` → `readonly` → `as const` → `Object.freeze`
> - Immutable update patterns — spread, nested updates, Immer
> - Pure functions — deterministic, no side effects
> - Side effects — nhận diện và cô lập
> - Referential transparency — "thay function bằng kết quả"
>
> **Yêu cầu trước**: Part I hoàn thành (Chapters 4–10).
> **Thời gian đọc**: ~45 phút | **Level**: Intermediate
> **Kết quả cuối cùng**: Viết code FP thực sự — immutable data, pure functions, side effects cô lập.

---

## Chào mừng đến Part II

Part I dạy bạn **công cụ** — types, generics, narrowing, modules. Part II dạy bạn **tư duy**. Sự khác biệt giống học cách cầm búa (Part I) vs học cách xây nhà (Part II).

Chapter 11 là nền tảng quan trọng nhất: **Immutability & Purity**. Mọi pattern FP trong các chương sau đều dựa trên 2 nguyên tắc này.

---

## Immutability & Purity — Nền tảng của FP thinking

JavaScript/TypeScript cho phép mutation ở khắp nơi — `array.push()`, `object.key = value`, `delete obj.prop`. Dễ viết nhưng khó debug: "biến này bị sửa ở đâu?"

Chapter này dạy bạn **chống lại mặc định** của JavaScript bằng `as const`, `Object.freeze()`, `readonly`, và spread operators. Mục tiêu: data flows through transformations thay vì bị mutated in-place. Kết quả: code dễ hiểu, dễ test, dễ parallel.


## 11.1 — Tại sao Immutability?

### Câu chuyện: Bản hợp đồng

Bạn ký hợp đồng thuê nhà. Sau 1 tháng, chủ nhà đổi giá trong HỢP ĐỒNG CỦA BẠN (cùng tờ giấy). Bạn kiện — nhưng không chứng minh được giá cũ vì bản gốc đã bị sửa.

Mutable data = hợp đồng bị sửa. Immutable data = hợp đồng KHÔNG BAO GIỜ thay đổi. Muốn đổi giá? Ký hợp đồng MỚI.

### Mutation = Bug factory

```typescript
// filename: src/mutation_bugs.ts

// ❌ Mutation — ai đổi orders?
const processOrder = (order: { items: string[]; total: number }) => {
    order.items.push("shipping-fee");   // Mutation!
    order.total += 50000;                // Mutation!
    return order;
};

const myOrder = { items: ["laptop"], total: 20000000 };
processOrder(myOrder);

// myOrder đã BỊ THAY ĐỔI mà bạn không biết!
// myOrder.items = ["laptop", "shipping-fee"]
// myOrder.total = 20050000
// Bug: gọi processOrder lần 2 → thêm shipping-fee LẦN NỮA

// ✅ Immutable — tạo bản mới
const processOrderPure = (
    order: { readonly items: readonly string[]; readonly total: number }
) => ({
    items: [...order.items, "shipping-fee"],
    total: order.total + 50000,
});

const myOrder2 = { items: ["laptop"], total: 20000000 } as const;
const processed = processOrderPure(myOrder2);

// myOrder2 KHÔNG ĐỔI! processed là bản MỚI
// Gọi processOrderPure bao nhiêu lần cũng cho cùng kết quả
```

### Lợi ích của immutability

| | Mutable | Immutable |
|---|---|---|
| Debug | ❌ "Ai đổi giá trị?" | ✅ Giá trị không đổi — dễ trace |
| Concurrency | ❌ Race conditions | ✅ Không shared mutable state |
| Testing | ❌ Setup + reset state | ✅ Input → Output, done |
| Undo/Redo | ❌ Phải backup trước khi đổi | ✅ Giữ history bản cũ |
| Predictability | ❌ Function có thể đổi input | ✅ Input không bao giờ đổi |

---

## ✅ Checkpoint 11.1

> Đến đây bạn phải hiểu:
> 1. **Mutation** = thay đổi data tại chỗ → bugs khó debug
> 2. **Immutability** = không đổi data cũ, tạo bản mới
> 3. Immutable data dễ **debug**, **test**, **predict**
>
> **Test nhanh**: `const arr = [1,2,3]; arr.push(4);` — `arr` có immutable không?
> <details><summary>Đáp án</summary>KHÔNG! `const` chỉ ngăn **gán lại** biến, KHÔNG ngăn **mutation** nội dung. `arr.push(4)` vẫn hoạt động. Cần `readonly` để ngăn mutation.</details>

---

## 11.2 — 4 Tầng Immutability

### Tầng 1: `const` — Ngăn gán lại (shallow nhất)

```typescript
// filename: src/const_level.ts

// const ngăn GÁN LẠI biến
const x = 42;
// x = 100;  // ❌ Error: Cannot assign to 'x'

// NHƯNG const KHÔNG ngăn mutation nội dung!
const arr = [1, 2, 3];
arr.push(4);       // ✅ Mutation OK — const chỉ ngăn arr = [...]
arr[0] = 99;       // ✅ Mutation OK

const obj = { name: "An" };
obj.name = "Bình"; // ✅ Mutation OK — const chỉ ngăn obj = {...}

// const = "biến chỉ gán 1 lần" — KHÔNG phải immutability!
```

### Tầng 2: `readonly` — Ngăn mutation ở type level

```typescript
// filename: src/readonly_level.ts
import assert from "node:assert/strict";

type User = {
    readonly name: string;
    readonly age: number;
    readonly scores: readonly number[];
};

const user: User = { name: "An", age: 25, scores: [85, 92] };

// user.name = "Bình";        // ❌ readonly field
// user.scores.push(100);     // ❌ readonly array
// user.scores[0] = 99;       // ❌ readonly array

// Nhưng: readonly CHỈ ở compile-time — runtime vẫn là JS bình thường
// (user as any).name = "hack";  // 💔 Runtime vẫn hoạt động

// Update = tạo bản mới
const older: User = { ...user, age: 26 };
assert.strictEqual(user.age, 25);   // gốc KHÔNG ĐỔI
assert.strictEqual(older.age, 26);  // bản mới

console.log("Readonly OK ✅");
```

### Tầng 3: `as const` — Deep readonly + literal types

```typescript
// filename: src/as_const.ts
import assert from "node:assert/strict";

// as const = deep readonly + literal types
const config = {
    host: "localhost",
    port: 3000,
    features: ["auth", "logging"],
} as const;

// Type: {
//     readonly host: "localhost";        ← literal type, không phải string!
//     readonly port: 3000;              ← literal type, không phải number!
//     readonly features: readonly ["auth", "logging"];
// }

// config.host = "x";           // ❌ readonly
// config.features.push("x");   // ❌ readonly

// as const hữu ích cho:
// 1. Config objects
// 2. Lookup tables
// 3. Enum alternatives
const ROLES = ["admin", "user", "guest"] as const;
type Role = typeof ROLES[number];  // "admin" | "user" | "guest"

assert.strictEqual(ROLES.length, 3);
console.log("as const OK ✅");
```

### Tầng 4: `Object.freeze` — Runtime immutability

```typescript
// filename: src/freeze.ts
import assert from "node:assert/strict";

// Object.freeze — runtime enforcement (shallow!)
const frozen = Object.freeze({ name: "An", age: 25 });

// frozen.name = "Bình";  // ❌ Runtime: silent fail (strict mode: TypeError)

// ⚠️ freeze CHỈ shallow!
const nested = Object.freeze({
    user: { name: "An" },  // nested object KHÔNG frozen!
});

// nested.user = { name: "Bình" };  // ❌ Frozen
// nested.user.name = "Bình";       // ✅ Nested KHÔNG frozen — silent mutation!

// Khi nào dùng freeze?
// - Hiếm khi! readonly + as const đủ cho hầu hết cases
// - Freeze khi cần runtime safety (nhận data từ untrusted source)
// - Hạn chế: shallow, chậm hơn, không freeze arrays methods
```

### So sánh 4 tầng

| Tầng | Tool | Compile-time | Runtime | Depth |
|------|------|-------------|---------|-------|
| 1 | `const` | Ngăn gán lại | Ngăn gán lại | Variable only |
| 2 | `readonly` | ✅ Ngăn mutation | ❌ | Shallow |
| 3 | `as const` | ✅ Deep readonly + literals | ❌ | Deep |
| 4 | `Object.freeze` | ❌ (cần type) | ✅ | **Shallow** |

> **💡 Quy tắc sách**: `readonly` + `as const` là đủ cho 95% cases. `Object.freeze` hiếm khi cần. `const` luôn dùng (nhưng nhớ nó KHÔNG phải immutability).

---

## ✅ Checkpoint 11.2

> Đến đây bạn phải hiểu:
> 1. **`const`** = ngăn gán lại. KHÔNG ngăn mutation nội dung
> 2. **`readonly`** = compile-time, shallow. Dùng cho type annotations
> 3. **`as const`** = deep readonly + literal types. Dùng cho config/constants
> 4. **`Object.freeze`** = runtime, **shallow**. Hiếm khi cần
>
> **Test nhanh**: `const obj = { a: { b: 1 } } as const; obj.a.b = 2;` — lỗi compile hay runtime?
> <details><summary>Đáp án</summary>**Compile lỗi**. `as const` makes ALL properties deeply readonly. `obj.a.b` = readonly → compiler cấm. Nhưng nếu dùng `as any` thì runtime vẫn mutate được (vì `as const` chỉ compile-time).</details>

---

## 11.3 — Immutable Update Patterns

### Spread — Clone + override

```typescript
// filename: src/spread_patterns.ts
import assert from "node:assert/strict";

type User = {
    readonly name: string;
    readonly age: number;
    readonly email: string;
};

// Update 1 field
const updateAge = (user: User, age: number): User => ({
    ...user,
    age,
});

// Update nhiều fields
const updateProfile = (
    user: User,
    changes: Partial<User>
): User => ({
    ...user,
    ...changes,
});

const an: User = { name: "An", age: 25, email: "an@mail.com" };
const older = updateAge(an, 26);
const updated = updateProfile(an, { name: "An Nguyễn", age: 26 });

assert.strictEqual(an.age, 25);          // gốc KHÔNG ĐỔI
assert.strictEqual(older.age, 26);
assert.strictEqual(updated.name, "An Nguyễn");

console.log("Spread OK ✅");
```

### Nested updates — Pain point!

```typescript
// filename: src/nested_update.ts
import assert from "node:assert/strict";

type Address = {
    readonly street: string;
    readonly city: string;
};

type Company = {
    readonly name: string;
    readonly address: Address;
};

type Employee = {
    readonly name: string;
    readonly company: Company;
};

// ❌ Update nested field = spread hell!
const updateCity = (emp: Employee, city: string): Employee => ({
    ...emp,
    company: {
        ...emp.company,
        address: {
            ...emp.company.address,
            city,
        },
    },
});

const emp: Employee = {
    name: "An",
    company: {
        name: "StartupX",
        address: { street: "123 Nguyễn Huệ", city: "HCM" },
    },
};

const moved = updateCity(emp, "Hà Nội");
assert.strictEqual(emp.company.address.city, "HCM");      // gốc
assert.strictEqual(moved.company.address.city, "Hà Nội"); // mới

// 😰 3 tầng spread cho 1 field — Immer giải quyết vấn đề này
```

### Immer — Nested updates dễ dàng

```typescript
// filename: src/immer_update.ts
// npm install immer

import { produce } from "immer";
import assert from "node:assert/strict";

type Employee = {
    readonly name: string;
    readonly company: {
        readonly name: string;
        readonly address: {
            readonly street: string;
            readonly city: string;
        };
    };
};

const emp: Employee = {
    name: "An",
    company: {
        name: "StartupX",
        address: { street: "123 Nguyễn Huệ", city: "HCM" },
    },
};

// Immer: viết mutation syntax → tự động tạo bản mới!
// (draft là mutable proxy — Immer tạo immutable copy phía sau)
const moved = produce(emp, (draft) => {
    draft.company.address.city = "Hà Nội";
});

assert.strictEqual(emp.company.address.city, "HCM");      // gốc KHÔNG ĐỔI
assert.strictEqual(moved.company.address.city, "Hà Nội"); // bản mới

// Immer = best of both worlds:
// - Viết code mutation (dễ đọc)
// - Kết quả immutable (an toàn)

console.log("Immer OK ✅");
```

> **💡 Khi nào dùng Immer?** Khi nested updates quá 2 tầng deep. 1–2 tầng spread vẫn OK. Immer thêm dependency nhưng giảm rất nhiều boilerplate.

### Array immutable updates

```typescript
// filename: src/array_updates.ts
import assert from "node:assert/strict";

const numbers: readonly number[] = [1, 2, 3, 4, 5];

// Thêm phần tử — spread, KHÔNG push
const added = [...numbers, 6];
assert.deepStrictEqual(added, [1, 2, 3, 4, 5, 6]);

// Xóa phần tử — filter, KHÔNG splice
const without3 = numbers.filter(n => n !== 3);
assert.deepStrictEqual(without3, [1, 2, 4, 5]);

// Cập nhật phần tử — map, KHÔNG gán index
const doubled = numbers.map(n => n * 2);
assert.deepStrictEqual(doubled, [2, 4, 6, 8, 10]);

// Sắp xếp — spread + sort, KHÔNG sort trực tiếp
// (Array.sort() MUTATES! Phải copy trước)
const sorted = [...numbers].sort((a, b) => b - a);
assert.deepStrictEqual(sorted, [5, 4, 3, 2, 1]);
assert.deepStrictEqual(numbers, [1, 2, 3, 4, 5]);  // gốc KHÔNG ĐỔI

console.log("Array immutable OK ✅");
```

---

## ✅ Checkpoint 11.3

> Đến đây bạn phải hiểu:
> 1. **Spread** = clone + override. `{...obj, field: newVal}` cho objects
> 2. **Nested updates** = spread hell! **Immer** giải quyết
> 3. **Array updates**: spread (add), filter (remove), map (update), `[...arr].sort()` (sort)
> 4. **Array.sort() MUTATES** — luôn copy trước!
>
> **Test nhanh**: `const arr = [3,1,2]; const sorted = arr.sort();` — `arr` bây giờ là gì?
> <details><summary>Đáp án</summary>`[1,2,3]`! `Array.sort()` MUTATES array gốc VÀ trả reference cùng array. `arr` và `sorted` cùng trỏ đến `[1,2,3]`. Dùng `[...arr].sort()` để tránh.</details>

---

## 11.4 — Pure Functions: Deterministic & No Side Effects

### Định nghĩa: Pure function

Pure function = function thỏa 2 điều kiện:
1. **Deterministic** — cùng input → LUÔN cùng output
2. **No side effects** — không thay đổi gì bên ngoài function

```typescript
// filename: src/pure_functions.ts
import assert from "node:assert/strict";

// ✅ Pure — cùng input → cùng output, không side effects
const add = (a: number, b: number): number => a + b;
const greet = (name: string): string => `Hello ${name}`;
const double = (arr: readonly number[]): readonly number[] =>
    arr.map(n => n * 2);

// ❌ Impure — kết quả khác nhau mỗi lần gọi
const now = (): number => Date.now();        // phụ thuộc thời gian
const random = (): number => Math.random();  // phụ thuộc random state

// ❌ Impure — side effects
let count = 0;
const increment = (): number => { count += 1; return count; };  // mutation!
const log = (msg: string): void => { console.log(msg); };       // I/O!

assert.strictEqual(add(3, 4), 7);
assert.strictEqual(add(3, 4), 7);  // LUÔN 7 — deterministic!

console.log("Pure functions OK ✅");
```

### Side effects — Nhận diện

| Side effect | Ví dụ | Tại sao impure? |
|---|---|---|
| **Mutation** | `arr.push(x)`, `obj.name = "x"` | Thay đổi data bên ngoài |
| **I/O** | `console.log()`, `fetch()`, `fs.readFile()` | Tương tác với thế giới bên ngoài |
| **Random** | `Math.random()`, `Date.now()` | Kết quả không deterministic |
| **Global state** | Đọc/ghi biến global | Phụ thuộc/thay đổi state chia sẻ |
| **Exceptions** | `throw new Error()` | Làm gián đoạn control flow |

### Pure function trong thực tế

```typescript
// filename: src/pure_practical.ts
import assert from "node:assert/strict";

type Product = {
    readonly name: string;
    readonly price: number;
    readonly quantity: number;
};

type CartSummary = {
    readonly itemCount: number;
    readonly subtotal: number;
    readonly tax: number;
    readonly total: number;
};

// ✅ Tất cả đều pure — dễ test, dễ predict
const itemTotal = (product: Product): number =>
    product.price * product.quantity;

const subtotal = (products: readonly Product[]): number =>
    products.reduce((sum, p) => sum + itemTotal(p), 0);

const calculateTax = (amount: number, rate: number): number =>
    Math.round(amount * rate);

const summarizeCart = (
    products: readonly Product[],
    taxRate: number
): CartSummary => {
    const sub = subtotal(products);
    const tax = calculateTax(sub, taxRate);
    return {
        itemCount: products.length,
        subtotal: sub,
        tax,
        total: sub + tax,
    };
};

const cart: readonly Product[] = [
    { name: "Laptop", price: 20000000, quantity: 1 },
    { name: "Mouse", price: 500000, quantity: 2 },
];

const summary = summarizeCart(cart, 0.1);

assert.strictEqual(summary.subtotal, 21000000);  // 20M + 1M
assert.strictEqual(summary.tax, 2100000);         // 21M × 10%
assert.strictEqual(summary.total, 23100000);       // 21M + 2.1M

console.log(`Total: ${summary.total.toLocaleString()}đ`);
// Output: Total: 23,100,000đ
```

> **💡 Pure functions = testability**: `summarizeCart` không cần database, API, mock. Truyền data → nhận kết quả → assert. Đó là sức mạnh của FP.

---

## ✅ Checkpoint 11.4

> Đến đây bạn phải hiểu:
> 1. **Pure function** = deterministic + no side effects
> 2. **Side effects**: mutation, I/O, random, global state, exceptions
> 3. **Pure = testable**: cùng input → cùng output, không cần setup
> 4. **FP goal**: viết NHIỀU pure functions, CÔ LẬP side effects
>
> **Test nhanh**: `const f = (arr: number[]) => { arr.sort(); return arr[0]; }` — pure không?
> <details><summary>Đáp án</summary>KHÔNG pure! `arr.sort()` MUTATES input array = side effect. Fix: `[...arr].sort()` để không đổi input.</details>

---

## 11.5 — Referential Transparency

### "Thay function bằng kết quả"

Một expression là **referentially transparent** nếu bạn có thể **thay nó bằng kết quả** mà program không thay đổi behavior:

```typescript
// filename: src/referential_transparency.ts
import assert from "node:assert/strict";

// ✅ Referentially transparent
const double = (x: number): number => x * 2;

// Hai dòng này GIỐNG HỆT nhau:
const a = double(5) + double(5);   // = 10 + 10 = 20
const b = 10 + 10;                  // = 20

assert.strictEqual(a, b);  // ✅ Luôn đúng!

// ❌ KHÔNG referentially transparent
let counter = 0;
const next = (): number => { counter += 1; return counter; };

// Hai dòng này KHÁC nhau:
const c = next() + next();   // = 1 + 2 = 3
// KHÔNG thể thay next() bằng kết quả — mỗi lần gọi khác nhau!
```

### Tại sao referential transparency quan trọng?

```typescript
// filename: src/rt_benefits.ts
import assert from "node:assert/strict";

const multiply = (a: number, b: number): number => a * b;
const add = (a: number, b: number): number => a + b;

// Compiler/runtime có thể:
// 1. Cache kết quả (memoize) — vì cùng input = cùng output
// 2. Chạy song song — vì không phụ thuộc order
// 3. Tối ưu hóa — vì thay thế được bằng kết quả

// Ví dụ: pipeline đọc rõ ràng
const prices = [100000, 200000, 300000];
const taxRate = 0.1;

const total = prices
    .map(p => multiply(p, 1 + taxRate))     // thêm thuế
    .reduce(add, 0);                         // tổng

assert.strictEqual(total, 660000);  // (100K + 200K + 300K) × 1.1

console.log("RT OK ✅");
```

> **💡 Referential transparency = reason about code**: Bạn có thể đọc code từ trong ra ngoài, thay mỗi expression bằng giá trị, và hiểu chương trình. Không cần track "state hiện tại là gì".

---

## ✅ Checkpoint 11.5

> Đến đây bạn phải hiểu:
> 1. **Referential transparency** = thay function call bằng kết quả, program không đổi
> 2. Pure functions **luôn** referentially transparent
> 3. RT cho phép: **memoization**, **parallelism**, **reasoning**
>
> **Test nhanh**: `console.log("hi")` có referentially transparent không?
> <details><summary>Đáp án</summary>KHÔNG! `console.log` là side effect (I/O). Thay `console.log("hi")` bằng `undefined` (return value) → program MẤT output. Behavior thay đổi.</details>

---

## 11.6 — Cô lập Side Effects: Functional Core, Imperative Shell

### Pattern: Tách pure logic khỏi side effects

```typescript
// filename: src/functional_core.ts
import assert from "node:assert/strict";

// === FUNCTIONAL CORE — Pure functions ===

type Order = {
    readonly id: string;
    readonly items: readonly string[];
    readonly total: number;
    readonly status: "pending" | "confirmed" | "shipped";
};

type OrderEvent =
    | { readonly tag: "confirm" }
    | { readonly tag: "ship" }
    | { readonly tag: "add_item"; readonly item: string; readonly price: number };

// Pure — nhận state + event → trả state mới
const applyEvent = (order: Order, event: OrderEvent): Order => {
    switch (event.tag) {
        case "confirm":
            return { ...order, status: "confirmed" };
        case "ship":
            return { ...order, status: "shipped" };
        case "add_item":
            return {
                ...order,
                items: [...order.items, event.item],
                total: order.total + event.price,
            };
    }
};

// Pure — xử lý chuỗi events
const processEvents = (
    order: Order,
    events: readonly OrderEvent[]
): Order =>
    events.reduce(applyEvent, order);

// Test — không cần database, API, mock!
const initial: Order = {
    id: "O1",
    items: [],
    total: 0,
    status: "pending",
};

const events: readonly OrderEvent[] = [
    { tag: "add_item", item: "Laptop", price: 20000000 },
    { tag: "add_item", item: "Mouse", price: 500000 },
    { tag: "confirm" },
];

const result = processEvents(initial, events);

assert.strictEqual(result.status, "confirmed");
assert.strictEqual(result.total, 20500000);
assert.deepStrictEqual(result.items, ["Laptop", "Mouse"]);
assert.strictEqual(initial.status, "pending");  // gốc KHÔNG ĐỔI

console.log(`Order ${result.id}: ${result.status}, ${result.total.toLocaleString()}đ`);
// Output: Order O1: confirmed, 20,500,000đ
```

```typescript
// === IMPERATIVE SHELL — Side effects ở biên ===
// (pseudocode — chưa cần hiểu chi tiết)

// const main = async () => {
//     // I/O: đọc từ database
//     const order = await db.getOrder("O1");
//     const events = await eventStore.getEvents("O1");
//
//     // PURE: xử lý logic
//     const updated = processEvents(order, events);
//
//     // I/O: ghi vào database
//     await db.saveOrder(updated);
//
//     // I/O: gửi notification
//     await notify.send(`Order ${updated.id} is ${updated.status}`);
// };
```

> **💡 Functional Core, Imperative Shell**: Viết **logic** trong pure functions (dễ test). Đẩy **side effects** ra biên (main, handlers). Core lớn = code tốt. Shell mỏng = ít bugs.

---

## ✅ Checkpoint 11.6

> Đến đây bạn phải hiểu:
> 1. **Functional Core** = pure functions xử lý logic
> 2. **Imperative Shell** = thin layer xử lý I/O (database, network, console)
> 3. Events → reduce → state mới = **event sourcing pattern**
> 4. Core **dễ test** (không mock). Shell **khó test** (cần integration tests)
>
> **Test nhanh**: `fetch` API call nên ở Core hay Shell?
> <details><summary>Đáp án</summary>**Shell**! `fetch` là I/O = side effect. Core nhận DATA (đã fetch), xử lý, trả kết quả. Shell gọi fetch → truyền data vào core → ghi kết quả.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Pure or Impure?

Phân loại mỗi function — pure hay impure? Tại sao?

```typescript
const a = (x: number): number => x * 2;
const b = (arr: number[]): number => { arr.reverse(); return arr[0]!; };
const c = (n: number): string => n > 0 ? "positive" : "non-positive";
const d = (): string => new Date().toISOString();
const e = (user: { name: string }): string => user.name.toUpperCase();
```

<details><summary>✅ Lời giải Bài 1</summary>

| Function | Pure? | Lý do |
|----------|-------|-------|
| `a` | ✅ Pure | Deterministic, no side effects |
| `b` | ❌ Impure | `arr.reverse()` MUTATES input array |
| `c` | ✅ Pure | Deterministic, no side effects |
| `d` | ❌ Impure | `Date()` = non-deterministic (thời gian thay đổi) |
| `e` | ✅ Pure | Chỉ ĐỌC `user.name`, không mutation, deterministic |

</details>

---

**Bài 2** (10 phút): Immutable operations

Viết tất cả operations cho `TodoList` mà KHÔNG mutation:

```typescript
type Todo = {
    readonly id: number;
    readonly text: string;
    readonly done: boolean;
};

type TodoList = readonly Todo[];

// 1. addTodo(list, text): TodoList — thêm todo mới (done = false)
// 2. toggleTodo(list, id): TodoList — đổi done của todo có id
// 3. removeTodo(list, id): TodoList — xóa todo có id
// 4. completedCount(list): number — đếm todos đã done
// 5. pendingTodos(list): TodoList — chỉ giữ todos chưa done
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type Todo = {
    readonly id: number;
    readonly text: string;
    readonly done: boolean;
};

type TodoList = readonly Todo[];

// Cách 1: impure (dùng let — đơn giản nhưng không pure)
// let nextId = 1;
// const addTodo = (list: TodoList, text: string): TodoList => [...list, { id: nextId++, text, done: false }];

// Cách 2: pure — truyền nextId từ bên ngoài
const addTodo = (list: TodoList, text: string, nextId: number): TodoList => [
    ...list,
    { id: nextId, text, done: false },
];

const toggleTodo = (list: TodoList, id: number): TodoList =>
    list.map(todo => todo.id === id ? { ...todo, done: !todo.done } : todo);

const removeTodo = (list: TodoList, id: number): TodoList =>
    list.filter(todo => todo.id !== id);

const completedCount = (list: TodoList): number =>
    list.filter(todo => todo.done).length;

const pendingTodos = (list: TodoList): TodoList =>
    list.filter(todo => !todo.done);

// Test
let todos: TodoList = [];
todos = addTodo(todos, "Learn TS", 1);
todos = addTodo(todos, "Learn FP", 2);
todos = addTodo(todos, "Build project", 3);

todos = toggleTodo(todos, 1);  // "Learn TS" = done

assert.strictEqual(completedCount(todos), 1);
assert.strictEqual(pendingTodos(todos).length, 2);

todos = removeTodo(todos, 2);  // remove "Learn FP"
assert.strictEqual(todos.length, 2);

// Lưu ý: `let todos` bên trên là imperative shell pattern —
// trong thực tế, state management sẽ dùng reduce (xem Functional Core, 11.6)
```

</details>

---

**Bài 3** (15 phút): Functional Core, Imperative Shell

Viết scoring system cho quiz game:

```typescript
// Functional Core (PURE):
type Question = { readonly id: number; readonly correct: string };
type Answer = { readonly questionId: number; readonly answer: string };
type ScoreResult = {
    readonly correct: number;
    readonly wrong: number;
    readonly score: number;      // correct × 10 - wrong × 5
    readonly percentage: number; // correct / total × 100
};

// 1. gradeAnswer(question, answer): boolean — đúng hay sai?
// 2. gradeQuiz(questions, answers): ScoreResult — chấm toàn bộ
// 3. isPassing(result: ScoreResult, threshold: number): boolean — đậu hay rớt?

// Imperative Shell (pseudocode):
// - Đọc questions từ file
// - Nhận answers từ user input
// - Gọi gradeQuiz (PURE)
// - In kết quả ra console
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Question = { readonly id: number; readonly correct: string };
type Answer = { readonly questionId: number; readonly answer: string };
type ScoreResult = {
    readonly correct: number;
    readonly wrong: number;
    readonly score: number;
    readonly percentage: number;
};

// Pure functions — Functional Core
const gradeAnswer = (question: Question, answer: Answer): boolean =>
    question.correct.toLowerCase() === answer.answer.toLowerCase();

const gradeQuiz = (
    questions: readonly Question[],
    answers: readonly Answer[]
): ScoreResult => {
    const results = questions.map(q => {
        const answer = answers.find(a => a.questionId === q.id);
        return answer ? gradeAnswer(q, answer) : false;
    });

    const correct = results.filter(Boolean).length;
    const wrong = questions.length - correct;

    return {
        correct,
        wrong,
        score: correct * 10 - wrong * 5,
        percentage: Math.round((correct / questions.length) * 100),
    };
};

const isPassing = (result: ScoreResult, threshold: number): boolean =>
    result.percentage >= threshold;

// Test — không cần file, không cần input!
const questions: readonly Question[] = [
    { id: 1, correct: "TypeScript" },
    { id: 2, correct: "Immutable" },
    { id: 3, correct: "Pure" },
];

const answers: readonly Answer[] = [
    { questionId: 1, answer: "typescript" },  // ✅ correct (case insensitive)
    { questionId: 2, answer: "Mutable" },     // ❌ wrong
    { questionId: 3, answer: "Pure" },        // ✅ correct
];

const result = gradeQuiz(questions, answers);

assert.strictEqual(result.correct, 2);
assert.strictEqual(result.wrong, 1);
assert.strictEqual(result.score, 15);       // 2×10 - 1×5
assert.strictEqual(result.percentage, 67);  // round(2/3 × 100)
assert.strictEqual(isPassing(result, 60), true);
assert.strictEqual(isPassing(result, 80), false);
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `readonly` không ngăn được mutation runtime | `readonly` chỉ compile-time | Chấp nhận — đây là TypeScript trade-off. Dùng `Object.freeze` nếu cần runtime |
| Nested spread quá sâu | Object structure phức tạp | Dùng Immer cho updates > 2 tầng. Hoặc flatten structure |
| `Array.sort()` mutates | Built-in JS sort modifies in-place | Luôn `[...arr].sort()`. Hoặc `toSorted()` (ES2023+ — cần target ES2023 trong tsconfig) |
| Pure function gọi impure function | Side effect "lây" ra pure code | Truyền result của impure function VÀO pure function thay vì gọi bên trong |
| `as const` gây type quá narrow | Literal types đôi khi quá strict | Dùng `satisfies Type` để giữ literal types + type check |

---

## Tóm tắt

- ✅ **Immutability** = không đổi data cũ, tạo bản mới. Dễ debug, test, predict.
- ✅ **4 tầng**: `const` (gán lại) → `readonly` (compile, shallow) → `as const` (compile, deep) → `Object.freeze` (runtime, shallow).
- ✅ **Update patterns**: spread objects, spread+filter/map arrays, Immer cho nested.
- ✅ **Pure functions** = deterministic + no side effects = testable.
- ✅ **Referential transparency** = thay call bằng kết quả, program không đổi.
- ✅ **Functional Core, Imperative Shell** = pure logic bên trong, I/O ở biên.

## Tiếp theo

→ Chapter 12: **Composition & Pipelines** — `compose(f, g)`, `pipe()`, method chaining, point-free style, data pipelines, và FP builder pattern.
