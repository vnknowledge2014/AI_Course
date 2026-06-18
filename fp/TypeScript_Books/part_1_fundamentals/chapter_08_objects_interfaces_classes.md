# Chapter 8 — Objects, Interfaces & Classes

> **Bạn sẽ học được**:
> - Object types — khai báo "shape" của objects
> - `type` vs `interface` — khi nào dùng cái nào
> - Structural typing — "nếu nó trông như vịt, nó là vịt"
> - `readonly` objects — immutability ở object level
> - Intersection types — kết hợp types
> - Classes trong FP — khi nào dùng, khi nào tránh
>
> **Yêu cầu trước**: Chapter 7 (generics, closures, function types).
> **Thời gian đọc**: ~40 phút | **Level**: Beginner → Intermediate
> **Kết quả cuối cùng**: Thiết kế data models chặt chẽ — chọn đúng tool giữa `type`, `interface`, và class.

---

## Objects, Interfaces & Classes — Structural Typing

TypeScript có điều khác biệt lớn so với Java/C#: **structural typing** thay vì nominal typing. Một object thỏa interface không vì nó "implements" interface (explicit) — mà vì nó có đúng **hình dạng** (implicit). Nếu interface yêu cầu `{ name: string, age: number }` và object có hai fields đó (cùng types), object tự động thỏa interface.

Điều này thay đổi cách bạn nghĩ về types: thay vì "object thuộc class nào", bạn hỏi "object có hình dạng gì". Flexibility tăng lên — bạn có thể mix objects từ các sources khác nhau miễn chúng có đúng shape. TypeScript gọi đây là "duck typing with compile-time checking".

Chapter này dạy bạn tận dụng structural typing hiệu quả, khi nào dùng `type` vs `interface`, và tại sao `readonly` là mặc định tốt nhất.


## 8.1 — Object Types: Shape của data

### Khai báo inline

```typescript
// filename: src/object_types.ts
import assert from "node:assert/strict";

// Object type — khai báo inline
const greet = (user: { readonly name: string; readonly age: number }): string =>
    `Hello ${user.name}, you are ${user.age}`;

assert.strictEqual(greet({ name: "An", age: 25 }), "Hello An, you are 25");
```

Inline types dài dòng. Đặt tên cho chúng:

### `type` alias

```typescript
// filename: src/type_alias.ts
import assert from "node:assert/strict";

// type alias — đặt tên cho shape
type User = {
    readonly name: string;
    readonly age: number;
    readonly email: string;
};

const greet = (user: User): string => `Hello ${user.name}`;
const isAdult = (user: User): boolean => user.age >= 18;

const an: User = { name: "An", age: 25, email: "an@mail.com" };

assert.strictEqual(greet(an), "Hello An");
assert.strictEqual(isAdult(an), true);

// ❌ Thiếu field → compile error
// const bad: User = { name: "Bình" };
// Error: Property 'age' is missing

// ❌ Thừa field khi gán trực tiếp → compile error
// const bad2: User = { name: "An", age: 25, email: "x", extra: true };
// Error: Object literal may only specify known properties

console.log("Type alias OK ✅");
```

### Optional properties

```typescript
// filename: src/optional_props.ts
import assert from "node:assert/strict";

type Config = {
    readonly host: string;
    readonly port: number;
    readonly debug?: boolean;        // optional — có hoặc không
    readonly timeout?: number;       // optional
};

// Chỉ cần required fields
const minimal: Config = { host: "localhost", port: 3000 };
const full: Config = { host: "0.0.0.0", port: 8080, debug: true, timeout: 5000 };

// Optional = T | undefined — cần check
const debugMode = (config: Config): string =>
    config.debug ? "🔧 Debug ON" : "🚀 Production";

assert.strictEqual(debugMode(minimal), "🚀 Production");
assert.strictEqual(debugMode(full), "🔧 Debug ON");

console.log(debugMode(minimal));  // 🚀 Production
```

> **💡 Optional vs Default**: Optional property = field có thể thiếu. Default parameter (Chapter 7) = giá trị mặc định khi không truyền. Optional cho config objects, default cho function parameters.

---

## ✅ Checkpoint 8.1

> Đến đây bạn phải hiểu:
> 1. **Object types** = shape: tên fields + types. Khai báo inline hoặc `type` alias
> 2. **`type`** = đặt tên cho bất kỳ type nào (objects, unions, functions, ...)
> 3. **`?`** = optional property. Type thành `T | undefined`
> 4. Object literal thừa field → compile error (excess property checking)
>
> **Test nhanh**: `type A = { x: number; y?: string };` — `{ x: 1 }` có hợp lệ không?
> <details><summary>Đáp án</summary>CÓ! `y` là optional — không bắt buộc. `{ x: 1 }` thỏa mãn type `A`.</details>

---

## 8.2 — `type` vs `interface`

### `interface` — Cách khác khai báo shape

```typescript
// filename: src/interface.ts
import assert from "node:assert/strict";

// interface — khai báo shape (giống type cho objects)
interface User {
    readonly name: string;
    readonly age: number;
}

// Dùng giống hệt type alias
const greet = (user: User): string => `Hello ${user.name}`;

assert.strictEqual(greet({ name: "An", age: 25 }), "Hello An");
```

### Bảng so sánh

| Feature | `type` | `interface` |
|---------|--------|-------------|
| Object shape | ✅ | ✅ |
| Union types | ✅ `type A = B \| C` | ❌ |
| Intersection | ✅ `type A = B & C` | ❌ (dùng `extends`) |
| Function types | ✅ `type Fn = (x: A) => B` | ⚠️ verbose |
| Primitives & tuples | ✅ `type ID = string` | ❌ |
| Declaration merging | ❌ | ✅ (auto-merge) |
| `extends` | ✅ (dùng `&` thay thế) | ✅ (`interface B extends A`) |
| Dùng khi | **FP, unions, functions** | **OOP, libraries, extensible APIs** |

### Quy tắc của sách này

```typescript
// ✅ type — cho mọi thứ trong FP
type UserId = string;
type Drink = { readonly tag: "coffee" } | { readonly tag: "tea" };
type Transform<A, B> = (input: A) => B;

// ✅ interface — chỉ khi cần declaration merging hoặc extends
// (hiếm trong FP — phổ biến hơn trong library APIs)
```

> **💡 Quy tắc đơn giản**: Dùng `type` cho mọi thứ. Chỉ dùng `interface` khi làm việc với thư viện OOP cần `extends` hoặc declaration merging. Sách này chủ yếu dùng `type`.

### Declaration merging — Tại sao biết

```typescript
// filename: src/declaration_merging.ts

// interface — auto merge
interface Window {
    myApp: { version: string };
}
// Giờ Window global có thêm myApp!

// type — KHÔNG merge
// type Window = { myApp: { version: string } };
// ❌ Error: Duplicate identifier 'Window'

// Declaration merging hữu ích cho:
// - Mở rộng global types (Window, NodeJS.ProcessEnv)
// - Plugin systems
// - Library augmentation
```

---

## ✅ Checkpoint 8.2

> Đến đây bạn phải hiểu:
> 1. **`type`** = mọi thứ (objects, unions, functions, primitives). FP chọn `type`
> 2. **`interface`** = objects, có `extends` và declaration merging
> 3. **Quy tắc**: `type` mặc định. `interface` cho OOP/library APIs
> 4. **Declaration merging** = interface auto-merge, type không
>
> **Test nhanh**: Có thể viết `type Drink = "coffee" | "tea"` bằng `interface` không?
> <details><summary>Đáp án</summary>KHÔNG! `interface` chỉ khai báo object shapes. Union types CHỈ dùng `type`.</details>

---

## 8.3 — Structural Typing: "Trông giống vịt = là vịt"

### TypeScript không quan tâm TÊN type

```typescript
// filename: src/structural.ts
import assert from "node:assert/strict";

type Point2D = { readonly x: number; readonly y: number };
type Vector2D = { readonly x: number; readonly y: number };

// Point2D và Vector2D có CÙNG structure → TypeScript coi chúng giống nhau!
const magnitude = (v: Vector2D): number =>
    Math.sqrt(v.x ** 2 + v.y ** 2);

const point: Point2D = { x: 3, y: 4 };
assert.strictEqual(magnitude(point), 5);  // ✅ Point2D dùng được cho Vector2D!

// TypeScript kiểm tra SHAPE, không kiểm tra TÊN type
// Đây gọi là "structural typing" — khác với Java/C# "nominal typing"
console.log(`Magnitude: ${magnitude(point)}`);
// Output: Magnitude: 5
```

### "Enough shape" = đủ fields

```typescript
// filename: src/enough_shape.ts
import assert from "node:assert/strict";

type HasName = { readonly name: string };

const greet = (thing: HasName): string => `Hello ${thing.name}`;

// Mọi object có field "name: string" đều OK!
assert.strictEqual(greet({ name: "An" }), "Hello An");

// Object có thừa field? Gán vào biến trước — structural typing chấp nhận
const ts = { name: "TypeScript", version: "5.0" };
assert.strictEqual(greet(ts), "Hello TypeScript");
// Lưu ý: greet({ name: "TS", version: "5.0" }) trực tiếp sẽ báo excess property error

// Điều này cho phép:
type User = { readonly name: string; readonly age: number };
type Company = { readonly name: string; readonly employees: number };

const user: User = { name: "An", age: 25 };
const company: Company = { name: "StartupX", employees: 10 };

// Cả User VÀ Company đều thỏa HasName!
assert.strictEqual(greet(user), "Hello An");
assert.strictEqual(greet(company), "Hello StartupX");

console.log("Structural typing OK ✅");
```

> **💡 Structural typing = FP-friendly**: Bạn không cần `implements` hay `extends`. Nếu object có đúng fields → compiler chấp nhận. Giảm boilerplate so với OOP (Java: `class User implements HasName`).

### Cẩn thận: Structural typing có bẫy

```typescript
// filename: src/structural_trap.ts

type Point2D = { readonly x: number; readonly y: number };
type Point3D = { readonly x: number; readonly y: number; readonly z: number };

const distanceFromOrigin = (p: Point2D): number =>
    Math.sqrt(p.x ** 2 + p.y ** 2);

const p3d: Point3D = { x: 3, y: 4, z: 12 };

// ✅ Nhưng hợp lý không? Point3D "thỏa" Point2D vì có x, y
// distanceFromOrigin bỏ qua z → khoảng cách KHÔNG đúng trong 3D!
const d = distanceFromOrigin(p3d);  // 5 — thiếu z!

// Đây là trade-off của structural typing:
// + Flexible, ít boilerplate
// - Có thể chấp nhận objects không intended
```

> **⚠️ Branded types** (Chapter 9) giải quyết bẫy này: tạo "tên brand" cho types để phân biệt Point2D vs Point3D dù cùng structure.

---

## ✅ Checkpoint 8.3

> Đến đây bạn phải hiểu:
> 1. **Structural typing** = kiểm tra SHAPE, không kiểm tra TÊN type
> 2. Object có **đủ fields** (hoặc nhiều hơn) → compiler chấp nhận
> 3. **Ưu điểm**: flexible, ít boilerplate (vs Java/C# nominal typing)
> 4. **Bẫy**: types cùng shape nhưng **khác ý nghĩa** có thể bị trộn lẫn
>
> **Test nhanh**: `type A = { x: number }; type B = { x: number; y: number };` — A gán cho B được không?
> <details><summary>Đáp án</summary>KHÔNG! B cần `y` — A thiếu `y`. Nhưng ngược lại: B gán cho A ĐƯỢC — B có thừa field `y`, nhưng đủ field `x` mà A cần.</details>

---

## 8.4 — Readonly Objects & Immutable Patterns

### Kết hợp `readonly` + `type`

```typescript
// filename: src/readonly_objects.ts
import assert from "node:assert/strict";

// Pattern 1: readonly từng field
type User = {
    readonly name: string;
    readonly age: number;
    readonly scores: readonly number[];
};

const user: User = { name: "An", age: 25, scores: [85, 92, 78] };

// user.age = 26;              // ❌ readonly!
// user.scores.push(100);      // ❌ readonly array!
// user.scores[0] = 99;        // ❌ readonly array!

// Pattern 2: Readonly<T> utility
type MutableConfig = {
    host: string;
    port: number;
    debug: boolean;
};

type Config = Readonly<MutableConfig>;
// = { readonly host: string; readonly port: number; readonly debug: boolean }

// Pattern 3: Immutable updates — tạo bản mới
const updateAge = (user: User, newAge: number): User => ({
    ...user,
    age: newAge
});

const older = updateAge(user, 26);
assert.strictEqual(user.age, 25);   // gốc KHÔNG ĐỔI
assert.strictEqual(older.age, 26);  // bản mới

console.log("Readonly objects OK ✅");
```

### Record type — dynamic keys

```typescript
// filename: src/record_type.ts
import assert from "node:assert/strict";

// Record<K, V> — object với keys type K, values type V
type Scores = Readonly<Record<string, number>>;

const scores: Scores = {
    math: 95,
    english: 87,
    physics: 92,
};

assert.strictEqual(scores["math"], 95);
// Lưu ý: với noUncheckedIndexedAccess (Ch4), Record<string, number>
// cho type number | undefined. Phải check trước khi dùng.
// scores["math"] = 100;  // ❌ Readonly!

// Record hữu ích cho:
type StatusLabels = Record<"active" | "inactive" | "banned", string>;

const labels: StatusLabels = {
    active: "🟢 Active",
    inactive: "⚪ Inactive",
    banned: "🔴 Banned",
};

// Thiếu key → compile error!
// const bad: StatusLabels = { active: "x" };
// Error: Properties 'inactive' and 'banned' are missing

assert.strictEqual(labels.active, "🟢 Active");
console.log("Record types OK ✅");
```

> **💡 `Record` vs `Map`**: `Record<K, V>` = object literal, keys phải là string/number/symbol. `Map<K, V>` (Chapter 3) = runtime collection, keys bất kỳ. Dùng `Record` cho static config, `Map` cho dynamic data.

---

## ✅ Checkpoint 8.4

> Đến đây bạn phải hiểu:
> 1. **`readonly`** trên từng field + **`Readonly<T>`** utility (shallow!)
> 2. **Immutable update** = spread `{...obj, field: newValue}` → bản mới
> 3. **`Record<K, V>`** = object type với keys xác định
> 4. `Record` cho static config. `Map` cho dynamic data (Chapter 3)
>
> **Test nhanh**: `Readonly<{ a: number; b: { c: string } }>` — `b.c` có readonly không?
> <details><summary>Đáp án</summary>KHÔNG! `Readonly` chỉ SHALLOW — `a` readonly, `b` readonly (không gán b mới), nhưng `b.c` KHÔNG readonly (vẫn mutate được). Cần ghi readonly cho nested type riêng.</details>

---

## 8.5 — Intersection Types: Kết hợp types

### `&` — "VÀ" — phải thỏa cả hai

```typescript
// filename: src/intersection.ts
import assert from "node:assert/strict";

type HasId = { readonly id: string };
type HasName = { readonly name: string };
type HasAge = { readonly age: number };

// Intersection — kết hợp: phải có TẤT CẢ fields
type User = HasId & HasName & HasAge;
// = { readonly id: string; readonly name: string; readonly age: number }

const user: User = { id: "U1", name: "An", age: 25 };

assert.strictEqual(user.id, "U1");
assert.strictEqual(user.name, "An");
assert.strictEqual(user.age, 25);

console.log("Intersection OK ✅");
```

### Intersection cho "mixin" pattern

```typescript
// filename: src/mixins.ts
import assert from "node:assert/strict";

// Base components
type Timestamped = {
    readonly createdAt: Date;
    readonly updatedAt: Date;
};

type SoftDeletable = {
    readonly deletedAt: Date | null;
};

type Identifiable = {
    readonly id: string;
};

// Mix them together!
type BaseEntity = Identifiable & Timestamped;
type DeletableEntity = BaseEntity & SoftDeletable;

// Domain types
type User = DeletableEntity & {
    readonly name: string;
    readonly email: string;
};

type Order = BaseEntity & {
    readonly userId: string;
    readonly total: number;
};

// User có: id, createdAt, updatedAt, deletedAt, name, email
// Order có: id, createdAt, updatedAt, userId, total

const now = new Date();
const user: User = {
    id: "U1",
    name: "An",
    email: "an@mail.com",
    createdAt: now,
    updatedAt: now,
    deletedAt: null,
};

assert.strictEqual(user.id, "U1");
assert.strictEqual(user.deletedAt, null);

console.log(`User: ${user.name} (${user.id})`);
// Output: User: An (U1)
```

> **💡 Intersection vs Union**: `A & B` = "phải là A VÀ B" (nhiều fields hơn). `A | B` = "có thể là A HOẶC B" (ít fields truy cập được). Union cho variants (DU). Intersection cho composition (mixins).

---

## ✅ Checkpoint 8.5

> Đến đây bạn phải hiểu:
> 1. **`A & B`** = intersection = phải thỏa TẤT CẢ types
> 2. **Mixin pattern** = compose types từ components nhỏ (`Timestamped & Identifiable`)
> 3. **Union** (`|`) = variants (OR). **Intersection** (`&`) = combination (AND)
>
> **Test nhanh**: `type A = { x: number } & { y: string };` — `A` có bao nhiêu required fields?
> <details><summary>Đáp án</summary>2 fields: `x: number` VÀ `y: string`. Intersection yêu cầu cả hai types.</details>

---

## 8.6 — Classes trong FP: Khi nào dùng?

### Lập trường của sách

Classes là công cụ OOP. Trong FP, chúng ta ưu tiên **types + functions**. Nhưng TypeScript vẫn có classes — và có **vài** use cases hợp lệ.

### Use case 1: Error classes (hợp lệ)

```typescript
// filename: src/error_classes.ts

// ✅ Error classes — dùng vì JS Error system dựa trên classes
class AppError extends Error {
    readonly code: number;
    constructor(code: number, message: string) {
        super(message);
        this.name = "AppError";
        this.code = code;
    }
}

class NotFoundError extends AppError {
    constructor(resource: string) {
        super(404, `${resource} not found`);
        this.name = "NotFoundError";
    }
}

class ValidationError extends AppError {
    readonly field: string;
    constructor(field: string, message: string) {
        super(400, message);
        this.name = "ValidationError";
        this.field = field;
    }
}

// Dùng với instanceof (Chapter 6)
const handleError = (err: Error): string => {
    if (err instanceof NotFoundError) return `404: ${err.message}`;
    if (err instanceof ValidationError) return `400 [${err.field}]: ${err.message}`;
    return `Unknown: ${err.message}`;
};
```

### Use case 2: Classes thay thế bằng types + closures

```typescript
// filename: src/class_vs_fp.ts
import assert from "node:assert/strict";

// ❌ OOP style — class với methods
class DiscountCalculatorOOP {
    constructor(private readonly percent: number) {}

    apply(price: number): number {
        return Math.round(price * (1 - this.percent / 100));
    }

    describe(): string {
        return `${this.percent}% off`;
    }
}

// ✅ FP style — type + closures (Chapter 7)
type DiscountCalculator = {
    readonly apply: (price: number) => number;
    readonly describe: () => string;
};

const createDiscount = (percent: number): DiscountCalculator => ({
    apply: (price) => Math.round(price * (1 - percent / 100)),
    describe: () => `${percent}% off`,
});

// Cả hai hoạt động như nhau:
const oopCalc = new DiscountCalculatorOOP(20);
const fpCalc = createDiscount(20);

assert.strictEqual(oopCalc.apply(100000), 80000);
assert.strictEqual(fpCalc.apply(100000), 80000);

assert.strictEqual(oopCalc.describe(), "20% off");
assert.strictEqual(fpCalc.describe(), "20% off");

console.log("FP wins (simpler, no 'this') ✅");
```

### So sánh: Class vs FP

| | Class (OOP) | Type + Closures (FP) |
|---|---|---|
| Khai báo | `class X { constructor() {} }` | `type X = { ... }` + factory function |
| State | `this.field` | Captured variables (closure) |
| `this` issues | ⚠️ Dễ mất context | ✅ Không có `this` |
| Testability | Phải mock class | Truyền function trực tiếp |
| Structural typing | ✅ (nhưng `this` binding phức tạp) | ✅ Tự nhiên |
| Dùng khi | Error types, interop OOP libs | **Mọi thứ khác** |

> **💡 Quy tắc**: Trong sách này, classes chỉ cho **Error types** (vì JS Error system bắt buộc). Mọi thứ khác → `type` + factory functions + closures.

---

## ✅ Checkpoint 8.6

> Đến đây bạn phải hiểu:
> 1. **FP ưu tiên `type` + closures** thay vì classes
> 2. **Error classes** = use case hợp lệ duy nhất (JS Error system)
> 3. **Factory functions** thay thế constructors — không `this`, dễ test
> 4. Classes có `this` issues — FP tránh bằng cách không dùng `this`
>
> **Test nhanh**: Tại sao FP không thích `this`?
> <details><summary>Đáp án</summary>`this` trong JS phụ thuộc vào CÁCH GỌI, không phải nơi khai báo. `const fn = obj.method; fn()` → `this` = `undefined`! Closure captures không có vấn đề này.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Type composition

Tạo domain types cho e-commerce:

```typescript
// 1. Type Identifiable = { readonly id: string }
// 2. Type Timestamped = { readonly createdAt: Date }
// 3. Type Product = Identifiable & Timestamped & { readonly name: string; readonly price: number }
// 4. Type OrderItem = { readonly product: Product; readonly quantity: number }
// 5. Type Order = Identifiable & Timestamped & { readonly items: readonly OrderItem[]; readonly customerId: string }
// 6. Function: orderTotal(order: Order): number — tính tổng
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type Identifiable = { readonly id: string };
type Timestamped = { readonly createdAt: Date };

type Product = Identifiable & Timestamped & {
    readonly name: string;
    readonly price: number;
};

type OrderItem = {
    readonly product: Product;
    readonly quantity: number;
};

type Order = Identifiable & Timestamped & {
    readonly items: readonly OrderItem[];
    readonly customerId: string;
};

const orderTotal = (order: Order): number =>
    order.items.reduce((sum, item) => sum + item.product.price * item.quantity, 0);

const now = new Date();
const laptop: Product = { id: "P1", name: "MacBook", price: 30000000, createdAt: now };
const mouse: Product = { id: "P2", name: "Mouse", price: 500000, createdAt: now };

const order: Order = {
    id: "O1",
    customerId: "C1",
    createdAt: now,
    items: [
        { product: laptop, quantity: 1 },
        { product: mouse, quantity: 2 },
    ],
};

assert.strictEqual(orderTotal(order), 31000000);  // 30M + 2×500K
```

</details>

---

**Bài 2** (10 phút): Structural typing exercise

Viết generic function `pick` — lấy subset fields từ object:

```typescript
// pick({ name: "An", age: 25, email: "x" }, ["name", "age"])
// → { name: "An", age: 25 }

// Hint: dùng keyof, generic constraints, Record
```

<details><summary>💡 Gợi ý</summary>`<T, K extends keyof T>` — K phải là key hợp lệ của T. Dùng `Object.fromEntries()` để build object mới.</details>

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

const pick = <T extends Record<string, unknown>, K extends keyof T>(
    obj: T,
    keys: readonly K[]
): Pick<T, K> => {
    const result = {} as Record<string, unknown>;
    for (const key of keys) {
        result[key as string] = obj[key];
    }
    return result as Pick<T, K>;
};

type User = { readonly name: string; readonly age: number; readonly email: string };
const user: User = { name: "An", age: 25, email: "an@mail.com" };

const subset = pick(user, ["name", "age"]);
assert.strictEqual(subset.name, "An");
assert.strictEqual(subset.age, 25);
// subset.email  // ❌ Error: Property 'email' does not exist — narrowed!

console.log(subset);
// Output: { name: 'An', age: 25 }
```

</details>

---

**Bài 3** (15 phút): Factory vs Class

Refactor class thành FP style:

```typescript
// Refactor this OOP code:
class BankAccount {
    private balance: number;
    constructor(initialBalance: number) {
        this.balance = initialBalance;
    }
    deposit(amount: number): void { this.balance += amount; }
    withdraw(amount: number): boolean {
        if (amount > this.balance) return false;
        this.balance -= amount;
        return true;
    }
    getBalance(): number { return this.balance; }
}

// Refactor thành:
// 1. Type BankAccount = { readonly balance: number }
// 2. Pure functions: deposit, withdraw (trả account mới, không mutation)
// 3. withdraw trả Result<BankAccount> (success hoặc error)
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type BankAccount = {
    readonly balance: number;
};

type Result<T> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: string };

const createAccount = (initialBalance: number): BankAccount => ({
    balance: initialBalance,
});

const deposit = (account: BankAccount, amount: number): BankAccount => ({
    balance: account.balance + amount,
});

const withdraw = (account: BankAccount, amount: number): Result<BankAccount> => {
    if (amount > account.balance) {
        return { tag: "err", error: `Insufficient funds: need ${amount}, have ${account.balance}` };
    }
    return { tag: "ok", value: { balance: account.balance - amount } };
};

// Test
const acc = createAccount(100000);
const afterDeposit = deposit(acc, 50000);
assert.strictEqual(afterDeposit.balance, 150000);
assert.strictEqual(acc.balance, 100000);  // gốc KHÔNG ĐỔI!

const result1 = withdraw(afterDeposit, 30000);
assert.strictEqual(result1.tag, "ok");
if (result1.tag === "ok") {
    assert.strictEqual(result1.value.balance, 120000);
}

const result2 = withdraw(acc, 200000);
assert.strictEqual(result2.tag, "err");

console.log("FP BankAccount OK ✅");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Object literal thừa field báo lỗi | Excess property checking chỉ áp dụng cho **literal** trực tiếp | Gán vào biến trước, hoặc dùng `as Type` |
| `type` và `interface` cùng tên | `interface` auto-merges, `type` không | Tránh trùng tên. Dùng 1 cách nhất quán |
| `Readonly<T>` không deep | `Readonly` chỉ shallow — 1 tầng | Ghi `readonly` cho nested types, hoặc dùng `as const` |
| Structural typing trộn types không đúng | Types cùng shape nhưng khác ý nghĩa | Dùng branded types (Chapter 9) |
| Class `this` bị `undefined` | Method gán cho biến mất context | Tránh classes — dùng closures/factories |

---

## Tóm tắt

- ✅ **`type`** = đặt tên cho mọi type. **`interface`** = chỉ objects, có declaration merging.
- ✅ **Structural typing** = kiểm tra SHAPE, không kiểm tra TÊN. Đủ fields = hợp lệ.
- ✅ **`readonly`** + **`Readonly<T>`** + **`as const`** = immutability. Spread cho updates.
- ✅ **Intersection `A & B`** = phải thỏa tất cả. Dùng cho mixin/composition.
- ✅ **Classes chỉ cho Error types**. Mọi thứ khác → `type` + factory functions.
- ✅ **`Record<K, V>`** cho static configs. `Map<K, V>` cho dynamic data.

## Tiếp theo

→ Chapter 9: **Advanced Type System** — Utility types (`Pick`, `Omit`, `Partial`), conditional types, mapped types, template literal types, branded types, và `infer` keyword.
