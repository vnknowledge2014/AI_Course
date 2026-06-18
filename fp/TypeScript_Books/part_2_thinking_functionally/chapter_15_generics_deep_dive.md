# Chapter 15 — Generics Deep Dive

> **Bạn sẽ học được**:
> - Generic constraints — `extends` nâng cao, multiple constraints
> - Generic function patterns — type-safe builders, factories, event systems
> - Variance — covariance, contravariance, và tại sao quan trọng
> - Advanced conditional types — recursive types, distributive tricks
> - `infer` patterns nâng cao — deep extraction, string parsing
> - Type-level programming — "chạy logic" trong type system
>
> **Yêu cầu trước**: Chapter 9 (utility types, mapped types, conditional types cơ bản), Chapter 13 (ADTs, Result).
> **Thời gian đọc**: ~50 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Viết generic utilities phức tạp — type system làm việc CHO bạn.

---

Bạn có một khuôn bánh hình ngôi sao. Bạn ấn nó xuống bột mì — ra bánh ngôi sao. Ấn xuống bột đất sét — ra tượng ngôi sao. Ấn xuống giấy — ra ngôi sao giấy. **Khuôn không quan tâm** chất liệu gì bên dưới — nó chỉ đảm bảo **hình dạng output** giống nhau. Đó chính là **generic** trong TypeScript.

`function identity<T>(x: T): T` là khuôn bánh đơn giản nhất: bất kỳ "chất liệu" nào bạn đưa vào (string, number, User), nó đảm bảo output cùng loại. Chapter 9 đã giới thiệu generics cơ bản — giờ chúng ta sẽ đi sâu hơn: khuôn bánh có thể **từ chối** chất liệu không phù hợp (constraints), khuôn có thể **thay đổi hình dạng** tùy chất liệu (conditional types), và khuôn có thể **phân tích** cấu trúc bên trong của chất liệu (infer).

Đây là chương advanced nhất của Part II. Nếu bạn cảm thấy một vài ví dụ ở section 15.4-15.6 quá phức tạp — đừng lo. Hãy nắm vững 15.1-15.3 trước, rồi quay lại phần sau khi cần trong dự án thực tế. Type-level programming là công cụ mạnh, nhưng không phải ai cũng dùng hàng ngày.

---

## 15.1 — Generic Constraints Nâng Cao

### Khuôn bánh kén chất liệu

Khuôn cookie đơn giản chấp nhận mọi thứ. Nhưng khuôn silicon chịu nhiệt chỉ hoạt động với chất liệu chịu được 200°C — bạn không thể dùng nó với chocolate (tan chảy!). Đây là **constraint**: khuôn chỉ chấp nhận chất liệu thỏa điều kiện nhất định.

Trong TypeScript, `<T extends { id: string }>` nghĩa là: "T phải có property `id: string`" — khuôn chỉ hoạt động với objects có `id`. Và bạn có thể đặt **nhiều constraints** cùng lúc bằng intersection: `<T extends Identifiable & Nameable>` = "T phải có CẢ `id` VÀ `name`".

```typescript
// filename: src/constraints.ts
import assert from "node:assert/strict";

// Constraint: T phải có cả id VÀ name
type Identifiable = { readonly id: string };
type Nameable = { readonly name: string };

// Multiple constraints bằng intersection
const displayEntity = <T extends Identifiable & Nameable>(entity: T): string =>
    `[${entity.id}] ${entity.name}`;

const user = { id: "U1", name: "An", age: 25 };
const product = { id: "P1", name: "Laptop", price: 20000000 };

assert.strictEqual(displayEntity(user), "[U1] An");
assert.strictEqual(displayEntity(product), "[P1] Laptop");

// ❌ Missing constraint
// displayEntity({ age: 25 });  // Error: missing id and name

console.log("Multiple constraints OK ✅");
```

`displayEntity` hoạt động với `user` (có `id` + `name` + `age`) VÀ `product` (có `id` + `name` + `price`) — vì cả hai thỏa constraint `Identifiable & Nameable`. Nhưng `{ age: 25 }` bị từ chối — thiếu cả `id` lẫn `name`. Constraint = khuôn kén chất liệu.

### `keyof` constraint — Chỉ chấp nhận keys hợp lệ

Một pattern cực kỳ phổ biến: viết function lấy property từ object, nhưng muốn compiler **đảm bảo key tồn tại**. `<K extends keyof T>` = "K chỉ có thể là key thực sự của T". Nếu object có 3 keys (`name`, `age`, `email`), K chỉ có thể là 1 trong 3 — không thể là `"phone"`.

```typescript
// filename: src/keyof_constraint.ts
import assert from "node:assert/strict";

// getProperty: CHỈ chấp nhận keys thực sự tồn tại
const getProperty = <T, K extends keyof T>(obj: T, key: K): T[K] =>
    obj[key];

const user = { name: "An", age: 25, email: "an@mail.com" } as const;

const name = getProperty(user, "name");    // type: "An"
const age = getProperty(user, "age");      // type: 25
// getProperty(user, "phone");             // ❌ "phone" not in keyof typeof user

assert.strictEqual(name, "An");
assert.strictEqual(age, 25);

// setProperty (immutable version)
const setProperty = <T, K extends keyof T>(
    obj: T,
    key: K,
    value: T[K]
): T => ({
    ...obj,
    [key]: value,
});

const updated = setProperty(user, "name", "Bình" as const);
assert.strictEqual(updated.name, "Bình");

console.log("keyof constraint OK ✅");
```

Đợi đã — nhìn kỹ return type: `T[K]`. Khi bạn gọi `getProperty(user, "name")`, TypeScript suy ra `K = "name"`, nên return type = `typeof user["name"]` = `"An"` (literal type!). Không phải `string` chung chung — mà chính xác là `"An"`. Đây là **constraint propagation**: generic giữ nguyên thông tin type exact, không mất precision.

### Constraint propagation — Generic truyền constraint qua functions

```typescript
// filename: src/constraint_propagation.ts
import assert from "node:assert/strict";

// Constraint: T extends Record<string, unknown>
// Return type PRESERVES T's exact shape
const freeze = <T extends Record<string, unknown>>(obj: T): Readonly<T> =>
    Object.freeze(obj) as Readonly<T>;

const config = freeze({ host: "localhost", port: 3000, debug: true });
// Type: Readonly<{ host: string; port: number; debug: boolean }>

// config.host = "x";  // ❌ Readonly
assert.strictEqual(config.host, "localhost");

// T's exact shape preserved — không mất fields
const fullConfig = freeze({
    db: "postgres",
    host: "localhost",
    port: 5432,
    ssl: true,
    maxConnections: 10,
});

assert.strictEqual(fullConfig.maxConnections, 10);  // field preserved!

console.log("Constraint propagation OK ✅");
```

---

## ✅ Checkpoint 15.1

> Đến đây bạn phải hiểu:
> 1. **Multiple constraints**: `T extends A & B` — T phải thỏa cả A và B
> 2. **`keyof` constraint**: `K extends keyof T` — chỉ keys hợp lệ
> 3. **Constraint propagation**: generic giữ exact type qua functions
>
> **Test nhanh**: `<T extends { length: number }>(x: T) => x.length` — `T` có thể là gì?
> <details><summary>Đáp án</summary>`string`, `Array<any>`, hoặc bất kỳ object nào có property `length: number`. Constraint chỉ yêu cầu shape, không phải type cụ thể (structural typing!).</details>

---

## 15.2 — Generic Function Patterns

### Khuôn bánh trong nhà máy thực tế

Giờ hãy xem generics hoạt động trong 3 patterns thực tế: event system, builder, và factory. Mỗi pattern dùng generics để đảm bảo type safety mà KHÔNG cần ép kiểu — compiler tự suy ra mọi thứ.

### Pattern 1: Type-safe event system

Hệ thống event thường dùng strings làm event names: `emitter.on("click", handler)`. Vấn đề: compiler không biết `"click"` event có payload gì — handler nhận `any`. Generics giải quyết: bạn định nghĩa `EventMap` ánh xạ event name → payload type, rồi generic `<K extends keyof E>` đảm bảo `on("click", handler)` chỉ chấp nhận handler khớp payload shape.

```typescript
// filename: src/event_system.ts
import assert from "node:assert/strict";

// Event map: event name → payload type
type EventMap = {
    readonly userLogin: { readonly userId: string; readonly timestamp: number };
    readonly orderPlaced: { readonly orderId: string; readonly total: number };
    readonly error: { readonly code: number; readonly message: string };
};

// Type-safe event emitter (simplified)
type EventHandler<T> = (payload: T) => void;

type EventEmitter<E extends Record<string, unknown>> = {
    readonly on: <K extends keyof E>(event: K, handler: EventHandler<E[K]>) => void;
    readonly emit: <K extends keyof E>(event: K, payload: E[K]) => void;
};

const createEmitter = <E extends Record<string, unknown>>(): EventEmitter<E> => {
    const handlers = new Map<keyof E, Array<EventHandler<unknown>>>();

    return {
        on: (event, handler) => {
            const list = handlers.get(event) ?? [];
            list.push(handler as EventHandler<unknown>);
            handlers.set(event, list);
        },
        emit: (event, payload) => {
            const list = handlers.get(event) ?? [];
            list.forEach(h => h(payload));
        },
    };
};

const emitter = createEmitter<EventMap>();

// Type-safe: handler receives CORRECT payload type
let lastLogin = "";
emitter.on("userLogin", (payload) => {
    // payload: { userId: string; timestamp: number } — auto-inferred!
    lastLogin = payload.userId;
});

emitter.emit("userLogin", { userId: "U1", timestamp: Date.now() });
assert.strictEqual(lastLogin, "U1");

// ❌ Type errors:
// emitter.emit("userLogin", { orderId: "O1" });  // Wrong payload shape!
// emitter.on("unknown", () => {});                // "unknown" not in EventMap!

console.log("Event system OK ✅");
```

### Pattern 2: Type-safe builder

```typescript
// filename: src/generic_builder.ts
import assert from "node:assert/strict";

// Builder accumulates configured fields — type tracks what's set
type Builder<T, Set extends Partial<T>> = {
    readonly set: <K extends keyof T>(
        key: K,
        value: T[K]
    ) => Builder<T, Set & Pick<T, K>>;
    readonly build: Set extends T ? () => T : never;
};

// Simplified builder (practical version)
type Config = {
    readonly host: string;
    readonly port: number;
    readonly debug: boolean;
};

type ConfigBuilder = {
    readonly host: (h: string) => ConfigBuilder;
    readonly port: (p: number) => ConfigBuilder;
    readonly debug: (d: boolean) => ConfigBuilder;
    readonly build: () => Config;
};

const configBuilder = (partial: Partial<Config> = {}): ConfigBuilder => ({
    host: (h) => configBuilder({ ...partial, host: h }),
    port: (p) => configBuilder({ ...partial, port: p }),
    debug: (d) => configBuilder({ ...partial, debug: d }),
    build: () => ({
        host: partial.host ?? "localhost",
        port: partial.port ?? 3000,
        debug: partial.debug ?? false,
    }),
});

const config = configBuilder()
    .host("api.example.com")
    .port(8080)
    .debug(true)
    .build();

assert.strictEqual(config.host, "api.example.com");
assert.strictEqual(config.port, 8080);
assert.strictEqual(config.debug, true);

// Defaults
const defaultConfig = configBuilder().build();
assert.strictEqual(defaultConfig.host, "localhost");
assert.strictEqual(defaultConfig.port, 3000);

console.log("Generic builder OK ✅");
```

### Pattern 3: Type-safe factory với registry

Cuối cùng, pattern factory registry. Bạn có `ShapeMap` định nghĩa "circle cần radius, rectangle cần width+height". Generic `createShape<K extends keyof ShapeMap>` đảm bảo khi gọi `createShape("circle", ...)`, params PHẢI khớp với circle — compiler từ chối `{ width: 3 }` vì circle cần `{ radius: number }`.

```typescript
// filename: src/generic_factory.ts
import assert from "node:assert/strict";

type ShapeMap = {
    readonly circle: { readonly radius: number };
    readonly rectangle: { readonly width: number; readonly height: number };
    readonly triangle: { readonly base: number; readonly height: number };
};

type Shape<K extends keyof ShapeMap> = {
    readonly type: K;
} & ShapeMap[K];

const createShape = <K extends keyof ShapeMap>(
    type: K, params: ShapeMap[K]
): Shape<K> => ({
    type,
    ...params,
} as Shape<K>);

const circle = createShape("circle", { radius: 5 });
// Type: { type: "circle"; radius: number }

const rect = createShape("rectangle", { width: 3, height: 4 });
// Type: { type: "rectangle"; width: number; height: number }

// ❌ createShape("circle", { width: 3 });  // Wrong params for circle!

assert.strictEqual(circle.type, "circle");
assert.strictEqual(circle.radius, 5);
assert.strictEqual(rect.width, 3);

console.log("Generic factory OK ✅");
```

---

## ✅ Checkpoint 15.2

> Đến đây bạn phải hiểu:
> 1. **Event system**: `EventMap` → type-safe `on`/`emit` với correct payload
> 2. **Builder pattern**: fluent API với method chaining, generics track state
> 3. **Factory registry**: `ShapeMap` → `createShape("circle", params)` — params type-checked
>
> **Test nhanh**: `emitter.emit("orderPlaced", { userId: "U1" })` — lỗi gì?
> <details><summary>Đáp án</summary>**Type error**: `orderPlaced` expects `{ orderId: string; total: number }`, got `{ userId: string }`. Generic `K` infers "orderPlaced" → `E[K]` = correct payload shape.</details>

---

## 15.3 — Variance: Covariance & Contravariance

### Khuôn bánh và hướng di chuyển

Đây là khái niệm mà nhiều developer bỏ qua — nhưng nó giải thích RẤT NHIỀU nghịch lý trong type system. Hãy bắt đầu với câu hỏi đơn giản: nếu `Dog` là subtype của `Animal` (mọi Dog đều là Animal), thì `Array<Dog>` có phải subtype của `Array<Animal>` không?

Trực giác nói "có" — mảng chó cũng là mảng động vật. Nhưng câu trả lời đúng là: **phụ thuộc vào `readonly`**. Nếu mảng `readonly` (chỉ đọc): CÓ, an toàn — bạn chỉ lấy phần tử ra, mà Dog IS-A Animal nên luôn OK. Nếu mảng mutable (đọc+ghi): KHÔNG — ai đó có thể push Cat vào `Array<Animal>`, nhưng `Array<Dog>` không chấp nhận Cat!

```typescript
// filename: src/variance.ts

// Câu hỏi: Animal[] có phải subtype của Dog[] không?
type Animal = { readonly name: string };
type Dog = Animal & { readonly breed: string };

// Covariance: nếu Dog extends Animal → Container<Dog> extends Container<Animal>
// "Container ĐI CÙNG HƯỚNG với type parameter"

// Contravariance: nếu Dog extends Animal → Handler<Animal> extends Handler<Dog>
// "Handler ĐI NGƯỢC HƯỚNG"
```

### Covariance — Output position (readonly)

```typescript
// filename: src/covariance.ts
import assert from "node:assert/strict";

type Animal = { readonly name: string };
type Dog = Animal & { readonly breed: string };

// readonly array = COVARIANT
// Dog[] extends Animal[] ✅ (vì readonly — không thể push sai type)
const dogs: readonly Dog[] = [
    { name: "Rex", breed: "Shepherd" },
    { name: "Buddy", breed: "Golden" },
];

const animals: readonly Animal[] = dogs;  // ✅ OK — covariant!
// animals chỉ ĐỌC — an toàn vì Dog IS-A Animal

// Function return = covariant
type Producer<T> = () => T;

const produceDog: Producer<Dog> = () => ({ name: "Rex", breed: "Shepherd" });
const produceAnimal: Producer<Animal> = produceDog;  // ✅ OK

assert.strictEqual(produceAnimal().name, "Rex");

console.log("Covariance OK ✅");
```

### Contravariance — Input position (function params)

Bạn có nhận ra điều gì lạ không? Function parameters đi **ngược hướng**. Nếu bạn cần handler cho Dog, bạn có thể dùng handler cho Animal — vì handler Animal chấp nhận MỌI animal, bao gồm Dogs. Nhưng ngược lại thì KHÔNG: handler cho Dog yêu cầu `.breed`, mà Animal không có.

```typescript
// filename: src/contravariance.ts
import assert from "node:assert/strict";

type Animal = { readonly name: string };
type Dog = Animal & { readonly breed: string };

// Function PARAMETER = contravariant
type Consumer<T> = (value: T) => void;

const feedAnimal: Consumer<Animal> = (a) => {
    console.log(`Feeding ${a.name}`);
};

const feedDog: Consumer<Dog> = feedAnimal;  // ✅ OK — contravariant!
// feedDog nhận Dog, feedAnimal chấp nhận ANY Animal (Dog IS-A Animal)

// ❌ Ngược lại KHÔNG an toàn:
// const bad: Consumer<Animal> = feedOnlyDog;
// bad({ name: "Cat" });  // 💥 feedOnlyDog cần .breed — Cat không có!

feedDog({ name: "Rex", breed: "Shepherd" });

console.log("Contravariance OK ✅");
```

### Tóm tắt variance

| Position | Variance | Quy tắc | Ví dụ |
|----------|----------|---------|-------|
| Output (return) | **Covariant** | Dog → Animal OK | `Producer<Dog>` → `Producer<Animal>` |
| Input (parameter) | **Contravariant** | Animal → Dog OK | `Consumer<Animal>` → `Consumer<Dog>` |
| Both (read/write) | **Invariant** | Phải exact match | Mutable `Array<Dog>` ≠ `Array<Animal>` |
| `readonly` | **Covariant** | Vì chỉ output | `readonly Dog[]` → `readonly Animal[]` |

Quay lại khuôn bánh: covariance = "khuôn cho Dog cũng cho ra Animal" (output đi cùng hướng). Contravariance = "khuôn nhận được Animal cũng nhận được Dog" (input đi ngược hướng). Hiểu variance giải thích tại sao chúng ta ưu tiên `readonly` — nó biến mutable (invariant, khó dùng) thành covariant (linh hoạt, an toàn).

> **💡 Tại sao cần biết?** Variance giải thích tại sao `readonly` arrays an toàn hơn mutable arrays. Nó cũng giải thích tại sao function parameters "đi ngược" — khi bạn cần handler cho Dog, handler cho Animal cũng hoạt động (vì Animal rộng hơn).

---

## ✅ Checkpoint 15.3

> Đến đây bạn phải hiểu:
> 1. **Covariant** = output position. `readonly Dog[]` → `readonly Animal[]` ✅
> 2. **Contravariant** = input position. `Consumer<Animal>` → `Consumer<Dog>` ✅
> 3. **Invariant** = both positions. Mutable arrays phải exact match
> 4. **`readonly` = safe** vì covariant (chỉ output, không push)
>
> **Test nhanh**: `const f: (a: Animal) => void = (d: Dog) => { console.log(d.breed) };` — an toàn không?
> <details><summary>Đáp án</summary>**KHÔNG an toàn!** `f` promise nhận ANY Animal, nhưng implementation cần `.breed` (Dog only). Gọi `f({ name: "Cat" })` → 💥 runtime error. TypeScript strict mode catches this.</details>

---

## 15.4 — Advanced Conditional Types

### Khuôn bánh thay đổi hình dạng tùy chất liệu

Conditional types = khuôn thông minh: "nếu chất liệu là bột mì, ra hình tròn; nếu đất sét, ra hình vuông". TypeScript syntax: `T extends Condition ? TypeA : TypeB`. Ch9 đã giới thiệu cơ bản — giờ chúng ta đi vào hai tính năng nâng cao: **recursive types** và **distributive tricks**.

### Recursive conditional types

`DeepReadonly<T>` là ví dụ kinh điển: nó đệ quy qua TẤT CẢ tầng của object, biến MỌI property thành `readonly`. Nếu T là array → wrap với `ReadonlyArray`. Nếu T là object → map qua keys, áp dụng `DeepReadonly` cho mỗi value. Nếu T là primitive → giữ nguyên.

```typescript
// filename: src/recursive_types.ts

// DeepReadonly — Ch9 giới thiệu, giờ hiểu SÂU hơn
type DeepReadonly<T> =
    T extends ReadonlyArray<infer U>
        ? ReadonlyArray<DeepReadonly<U>>
        : T extends Record<string, unknown>
            ? { readonly [K in keyof T]: DeepReadonly<T[K]> }
            : T;

// Test type
type Nested = {
    a: number;
    b: {
        c: string;
        d: {
            e: boolean;
        };
    };
    f: number[];
};

type DeepFrozen = DeepReadonly<Nested>;
// = {
//     readonly a: number;
//     readonly b: {
//         readonly c: string;
//         readonly d: {
//             readonly e: boolean;
//         };
//     };
//     readonly f: ReadonlyArray<number>;
// }
```

### Distributive conditional types — deep dive

Đây là tính năng mà nhiều developer gặp và bối rối: khi `T` là union, conditional type **phân phối** qua mỗi member. `IsString<string | number>` = `IsString<string> | IsString<number>` = `"yes" | "no"`. Nó tự động "tách" union ra, đánh giá từng member riêng, rồi ghép kết quả lại.

Nếu bạn KHÔNG muốn distributive? Wrap `T` trong tuple: `[T] extends [string]`. Tuple không phải "naked type parameter", nên TypeScript đánh giá cả union cùng lúc thay vì tách.

```typescript
// filename: src/distributive.ts

// Distributive: khi T = union, conditional "phân phối" qua mỗi member
type IsString<T> = T extends string ? "yes" : "no";

type A = IsString<string>;           // "yes"
type B = IsString<number>;           // "no"
type C = IsString<string | number>;  // "yes" | "no" — DISTRIBUTIVE!
// = IsString<string> | IsString<number> = "yes" | "no"

// Chặn distributive bằng tuple wrapping
type IsStringNonDist<T> = [T] extends [string] ? "yes" : "no";

type D = IsStringNonDist<string | number>;  // "no" — không distribute!
// [string | number] extends [string]? NO → "no"
```

### Extracting + filtering với conditional types

Power move cuối: dùng conditional types để **filter** keys từ object theo shape. Ví dụ: lấy tất cả event names có payload chứa `{ x, y }`. Pattern: map qua keys → nếu payload extends target thì giữ key, else `never` → filter `never` bằng `[keyof Events]`.

```typescript
// filename: src/extract_filter.ts
import assert from "node:assert/strict";

// Event system — extract event names by payload shape
type Events = {
    readonly click: { readonly x: number; readonly y: number };
    readonly keypress: { readonly key: string };
    readonly resize: { readonly width: number; readonly height: number };
    readonly error: { readonly message: string };
    readonly scroll: { readonly x: number; readonly y: number };
};

// Extract event names where payload has x & y
type PositionEvents = {
    [K in keyof Events]: Events[K] extends { readonly x: number; readonly y: number }
        ? K
        : never
}[keyof Events];

// PositionEvents = "click" | "resize" | "scroll"

// Generic version
type EventsWithPayload<E extends Record<string, unknown>, P> = {
    [K in keyof E]: E[K] extends P ? K : never
}[keyof E];

type HasXY = EventsWithPayload<Events, { readonly x: number; readonly y: number }>;
// = "click" | "resize" | "scroll"

type HasMessage = EventsWithPayload<Events, { readonly message: string }>;
// = "error"
```

---

## ✅ Checkpoint 15.4

> Đến đây bạn phải hiểu:
> 1. **Recursive types**: `DeepReadonly` áp dụng readonly cho mọi tầng
> 2. **Distributive**: union phân phối qua conditional type. `[T]` chặn
> 3. **Extract by shape**: filter keys where payload extends target shape
>
> **Test nhanh**: `type X = string extends string | number ? "yes" : "no"` = ?
> <details><summary>Đáp án</summary>`"yes"`. `string` extends `string | number` (vì string là subset của union). Không distributive vì T không phải NAKED type parameter — đây là concrete type check.</details>

---

## 15.5 — Advanced `infer` Patterns

### Khuôn bánh biết phân tích nguyên liệu

`infer` = "compiler, hãy tự tìm type này cho tôi". Giống khuôn bánh thông minh nhìn vào chất liệu và nói: "Ồ, đây là bột mì loại A, nhiệt độ nung 180°C" — tự phân tích thành phần. Bạn đã gặp `infer` cơ bản ở Ch9 (`ReturnType<F>`). Giờ đi sâu: infer trong function params, template literals, và recursive patterns.

### Infer trong complex positions

```typescript
// filename: src/infer_advanced.ts

// --- Function analysis ---
type FirstParam<F> = F extends (first: infer P, ...rest: readonly unknown[]) => unknown
    ? P : never;

// ⚠️ LastParam — TS KHÔNG hỗ trợ [...unknown[], infer L] trong function params
// Phải dùng tuple manipulation thay vì:
// type LastParam<F> = F extends (...args: readonly [...unknown[], infer L]) => unknown ? L : never;

type Fn1 = (a: string, b: number) => boolean;
type First = FirstParam<Fn1>;  // string

// --- Infer template literal ---
type ExtractRoute<S extends string> =
    S extends `/${infer Segment}/${infer Rest}`
        ? Segment | ExtractRoute<`/${Rest}`>
        : S extends `/${infer Segment}`
            ? Segment
            : never;

type Route = ExtractRoute<"/users/123/posts">;
// = "users" | "123" | "posts"

// --- Infer array element ---
type Flatten<T> = T extends ReadonlyArray<infer U> ? U : T;

type A = Flatten<string[]>;         // string
type B = Flatten<number[][]>;       // number[]
type C = Flatten<string>;           // string (not array → return T)

// Deep flatten
type DeepFlatten<T> = T extends ReadonlyArray<infer U> ? DeepFlatten<U> : T;

type D = DeepFlatten<number[][][]>;  // number
```

Hãy dừng lại ở `ExtractRoute` — đây là ví dụ tuyệt vời về type-level string parsing. `"/users/123/posts"` → infer Segment=`"users"`, Rest=`"123/posts"` → recurse → `"123"` | `"posts"`. Kết quả: union `"users" | "123" | "posts"`. Tất cả xảy ra **AT COMPILE TIME** — không có runtime code nào chạy.

### Infer cho Promise unwrapping

```typescript
// filename: src/infer_promise.ts
import assert from "node:assert/strict";

// Awaited<T> (built-in từ TS 4.5, nhưng hiểu cách nó hoạt động)
type MyAwaited<T> = T extends Promise<infer U> ? MyAwaited<U> : T;

type A = MyAwaited<Promise<string>>;              // string
type B = MyAwaited<Promise<Promise<number>>>;     // number (recursive!)
type C = MyAwaited<string>;                       // string (not Promise)

// Practical: extract return type of async function
type AsyncReturnType<F> = F extends (...args: readonly unknown[]) => Promise<infer R>
    ? R
    : never;

const fetchUser = async (id: string): Promise<{ name: string; age: number }> => ({
    name: "An",
    age: 25,
});

type UserData = AsyncReturnType<typeof fetchUser>;
// = { name: string; age: number }
```

### Infer cho Object path extraction

```typescript
// filename: src/infer_paths.ts

// Extract all possible paths from object (1 level deep)
type Paths<T> = T extends Record<string, unknown>
    ? { [K in keyof T & string]:
        T[K] extends Record<string, unknown>
            ? K | `${K}.${Paths<T[K]>}`
            : K
      }[keyof T & string]
    : never;

type Config = {
    readonly db: {
        readonly host: string;
        readonly port: number;
    };
    readonly api: {
        readonly baseUrl: string;
        readonly timeout: number;
    };
    readonly debug: boolean;
};

type ConfigPaths = Paths<Config>;
// = "db" | "db.host" | "db.port" | "api" | "api.baseUrl" | "api.timeout" | "debug"
```

---

## ✅ Checkpoint 15.5

> Đến đây bạn phải hiểu:
> 1. **`infer` in function params**: extract first param, return type
> 2. **`infer` in template literals**: parse route strings at type level
> 3. **Recursive `infer`**: `DeepFlatten`, `Awaited` — unwrap nested types
> 4. **Object path extraction**: recursive `Paths<T>` type
>
> **Test nhanh**: `type X = ExtractRoute<"/api/v2">` = ?
> <details><summary>Đáp án</summary>`"api" | "v2"`. Matches `/${infer Segment}/${infer Rest}`: Segment="api", Rest="v2". Then `/${infer Segment}`: Segment="v2".</details>

---

## 15.6 — Type-Level Programming: Putting It All Together

### Khi khuôn bánh trở thành nhà máy tự động

Đây là đỉnh cao: kết hợp tất cả — constraints, conditional types, infer, recursive types — thành utilities mà compiler chạy logic THAY bạn. Bạn viết type, compiler "chạy" nó và đảm bảo safety — không cần runtime checks.

### Real-world utility: Type-safe deep get

`deepGet(config, "db.host")` — compiler tự biết return type là `string` (vì `config.db.host` là string). Nếu bạn viết `"db.invalid"`, return type = `never`. Tất cả nhờ hai type utilities: `Split<S, D>` tách string thành tuple, và `DeepGet<T, Path>` navigate qua object theo path.

```typescript
// filename: src/deep_get.ts
import assert from "node:assert/strict";

// Split path string into array of keys
type Split<S extends string, D extends string> =
    S extends `${infer Head}${D}${infer Tail}`
        ? [Head, ...Split<Tail, D>]
        : [S];

type X = Split<"a.b.c", ".">;  // ["a", "b", "c"]

// Get nested value type by path
type DeepGet<T, Path extends readonly string[]> =
    Path extends readonly [infer Head extends string, ...infer Rest extends string[]]
        ? Head extends keyof T
            ? DeepGet<T[Head], Rest>
            : never
        : T;

// Safe deep get function
const deepGet = <
    T extends Record<string, unknown>,
    P extends string
>(
    obj: T,
    path: P
): DeepGet<T, Split<P, ".">> => {
    const keys = path.split(".");
    let current: unknown = obj;
    for (const key of keys) {
        current = (current as Record<string, unknown>)[key];
    }
    return current as DeepGet<T, Split<P, ".">>;
};

// Usage
const config = {
    db: {
        host: "localhost",
        port: 5432,
        credentials: {
            user: "admin",
            password: "secret",
        },
    },
    api: {
        baseUrl: "https://api.example.com",
    },
};

const host = deepGet(config, "db.host");         // type: string
const port = deepGet(config, "db.port");         // type: number
const user = deepGet(config, "db.credentials.user");  // type: string

assert.strictEqual(host, "localhost");
assert.strictEqual(port, 5432);
assert.strictEqual(user, "admin");

// ❌ deepGet(config, "db.invalid");  // type: never → compile warning

console.log("Deep get OK ✅");
```

### Real-world utility: Type-safe API router

Ví dụ cuối — và ấn tượng nhất: API router nơi MỌI handler tự động nhận đúng types từ API definition. Bạn chỉ cần ghi `registerRoute("POST /users", handler)`, và TypeScript tự biết handler cần `body: { name: string, email: string }` và phải return `{ id: string }`. Zero manual type annotations bên trong handler.

```typescript
// filename: src/typed_router.ts
import assert from "node:assert/strict";

// API route definitions
type API = {
    readonly "GET /users": {
        readonly response: readonly { readonly id: string; readonly name: string }[];
    };
    readonly "GET /users/:id": {
        readonly params: { readonly id: string };
        readonly response: { readonly id: string; readonly name: string; readonly email: string };
    };
    readonly "POST /users": {
        readonly body: { readonly name: string; readonly email: string };
        readonly response: { readonly id: string };
    };
    readonly "DELETE /users/:id": {
        readonly params: { readonly id: string };
        readonly response: { readonly success: boolean };
    };
};

// Extract method and path
type ExtractMethod<R extends string> = R extends `${infer M} ${string}` ? M : never;
type ExtractPath<R extends string> = R extends `${string} ${infer P}` ? P : never;

// Type-safe request handler
type Handler<Route extends keyof API> = (
    req: Omit<API[Route], "response">
) => API[Route]["response"];

// Register handlers
const handlers: { [R in keyof API]?: Handler<R> } = {};

const registerRoute = <R extends keyof API>(route: R, handler: Handler<R>) => {
    handlers[route] = handler as any;
};

// Usage — fully type-safe!
registerRoute("GET /users", (_req) => [
    { id: "1", name: "An" },
    { id: "2", name: "Bình" },
]);

registerRoute("POST /users", (req) => {
    // req.body: { name: string; email: string } — auto-inferred!
    return { id: "3" };
});

registerRoute("GET /users/:id", (req) => {
    // req.params: { id: string } — auto-inferred!
    return { id: req.params.id, name: "An", email: "an@mail.com" };
});

console.log("Typed router OK ✅");
```

> **💡 Type-level programming = compile-time safety**: `deepGet(config, "db.invalid")` → type `never`. `registerRoute("POST /users", handler)` → handler params auto-inferred from API definition. No runtime checks needed — compiler does the work.

---

## ✅ Checkpoint 15.6

> Đến đây bạn phải hiểu:
> 1. **`Split<S, D>`**: parse strings thành tuples AT TYPE LEVEL
> 2. **`DeepGet<T, Path>`**: navigate nested objects bằng type
> 3. **Typed router**: API definitions → auto-inferred handler types
> 4. **Type-level programming** = "chạy logic" trong compiler, không phải runtime
>
> **Test nhanh**: `Split<"a-b-c", "-">` = ?
> <details><summary>Đáp án</summary>`["a", "b", "c"]`. Recursive: Head="a", D="-", Tail="b-c". Recurse: Head="b", Tail="c". Base: ["c"]. Result: ["a", "b", "c"].</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Constraint functions

```typescript
// Viết generic function:
// 1. merge<T>(a: T, b: Partial<T>): T — merge b vào a (b overrides)
//    Constraint: T extends Record<string, unknown>
// 2. pluck<T, K extends keyof T>(arr: readonly T[], key: K): readonly T[K][]
//    — extract mảng values từ mảng objects
// Test: pluck([{name:"An",age:25}, {name:"Bình",age:30}], "name") = ["An","Bình"]
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

const merge = <T extends Record<string, unknown>>(a: T, b: Partial<T>): T => ({
    ...a,
    ...b,
});

const pluck = <T, K extends keyof T>(arr: readonly T[], key: K): readonly T[K][] =>
    arr.map(item => item[key]);

// Test merge
const base = { host: "localhost", port: 3000, debug: false };
const merged = merge(base, { port: 8080, debug: true });
assert.strictEqual(merged.host, "localhost");
assert.strictEqual(merged.port, 8080);
assert.strictEqual(merged.debug, true);

// Test pluck
const users = [
    { name: "An", age: 25 },
    { name: "Bình", age: 30 },
];

const names = pluck(users, "name");
assert.deepStrictEqual(names, ["An", "Bình"]);

const ages = pluck(users, "age");
assert.deepStrictEqual(ages, [25, 30]);
```

</details>

---

**Bài 2** (10 phút): Conditional type utilities

```typescript
// Viết các type utilities:
// 1. Mutable<T> — ngược lại Readonly<T>. Bỏ readonly từ mọi property
// 2. PickByType<T, V> — pick properties where value type = V
//    PickByType<{a: string, b: number, c: string}, string> = {a: string, c: string}
// 3. RequiredKeys<T> — extract keys that are NOT optional
//    RequiredKeys<{a: string, b?: number, c: string}> = "a" | "c"
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
// 1. Mutable — remove readonly
type Mutable<T> = {
    -readonly [K in keyof T]: T[K];
};

type Test1 = Mutable<{ readonly a: string; readonly b: number }>;
// = { a: string; b: number }

// 2. PickByType — pick properties by value type
type PickByType<T, V> = {
    [K in keyof T as T[K] extends V ? K : never]: T[K];
};

type Test2 = PickByType<{ a: string; b: number; c: string; d: boolean }, string>;
// = { a: string; c: string }

// 3. RequiredKeys — extract non-optional keys
type RequiredKeys<T> = {
    [K in keyof T]-?: undefined extends T[K] ? never : K;
}[keyof T];

type Test3 = RequiredKeys<{ a: string; b?: number; c: string }>;
// = "a" | "c"
```

</details>

---

**Bài 3** (15 phút): Type-safe form handler

```typescript
// Viết type-safe form handler system:
//
// type FormFields = {
//     name: { type: "text"; value: string };
//     age: { type: "number"; value: number };
//     newsletter: { type: "checkbox"; value: boolean };
// };
//
// 1. FormValues<F> — extract { name: string; age: number; newsletter: boolean }
// 2. FormHandler<F> — { onChange: <K>(field: K, value: F[K]["value"]) => void }
// 3. createForm<F>(fields: F): { values: FormValues<F>; handler: FormHandler<F> }
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type FieldDef = { readonly type: string; readonly value: unknown };

type FormFields = {
    readonly name: { readonly type: "text"; readonly value: string };
    readonly age: { readonly type: "number"; readonly value: number };
    readonly newsletter: { readonly type: "checkbox"; readonly value: boolean };
};

// 1. Extract values from field definitions
type FormValues<F extends Record<string, FieldDef>> = {
    [K in keyof F]: F[K]["value"];
};

type Values = FormValues<FormFields>;
// = { name: string; age: number; newsletter: boolean }

// 2. Type-safe onChange handler
type FormHandler<F extends Record<string, FieldDef>> = {
    readonly onChange: <K extends keyof F>(field: K, value: F[K]["value"]) => void;
};

// 3. Create form
const createForm = <F extends Record<string, FieldDef>>(
    defaults: FormValues<F>
): { values: FormValues<F>; handler: FormHandler<F> } => {
    const values = { ...defaults };

    const handler: FormHandler<F> = {
        onChange: (field, value) => {
            (values as Record<string, unknown>)[field as string] = value;
        },
    };

    return { values, handler };
};

// Usage
const form = createForm<FormFields>({
    name: "",
    age: 0,
    newsletter: false,
});

form.handler.onChange("name", "An");           // ✅ string
form.handler.onChange("age", 25);              // ✅ number
form.handler.onChange("newsletter", true);      // ✅ boolean

// form.handler.onChange("name", 42);           // ❌ number !== string
// form.handler.onChange("unknown", "x");       // ❌ "unknown" not in FormFields

assert.strictEqual(form.values.name, "An");
assert.strictEqual(form.values.age, 25);
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Type instantiation is excessively deep" | Recursive type quá sâu (>50 levels) | Thêm depth limit hoặc simplify recursion |
| Generic infer fails | TypeScript inference heuristic không đủ | Explicit type parameter: `fn<string>(...)` |
| Distributive khi không muốn | Naked type parameter trong conditional | Wrap trong tuple: `[T] extends [X]` |
| `any` lan tỏa qua generics | `any` bypasses mọi constraint | Dùng `unknown` thay `any`. Guard bằng `NoInfer<T>` (TS 5.4+) |
| Builder type quá phức tạp | Tracked phantom types gây inference fail | Simplify: dùng Partial<Config> + defaults |

---

## Tóm tắt

Chương này đã đưa bạn từ khuôn bánh đơn giản (generic function) qua khuôn kén chất liệu (constraints) đến khuôn biết phân tích nguyên liệu (infer) và nhà máy tự động (type-level programming). Mỗi tầng xây trên tầng trước:

- ✅ **Constraints**: `extends A & B`, `keyof T`, constraint propagation. Giới hạn chính xác.
- ✅ **Generic patterns**: event systems, builders, factory registries. Type-safe APIs.
- ✅ **Variance**: covariant (output), contravariant (input), invariant (both). `readonly` = safe.
- ✅ **Conditional types**: recursive, distributive (và cách chặn), filter by shape.
- ✅ **`infer`**: function params, template literals, deep extraction, Promise unwrap.
- ✅ **Type-level programming**: `Split`, `DeepGet`, typed routers. Compiler = runtime.

## Part II Complete! 🎉

Chapters 11–15 dạy bạn **tư duy FP**: immutability, composition, ADTs, validation, và generic type-level programming. Xuyên suốt 5 chương, bạn đã đi từ dây chuyền sản xuất (Ch12) qua tiệm kem và bộ Lego (Ch13) qua hải quan (Ch14) đến khuôn bánh (Ch15) — mỗi ẩn dụ giúp bạn nắm bắt một khía cạnh khác nhau của functional programming trong TypeScript. Part III sẽ áp dụng tất cả vào **design patterns** thực tế.

## Tiếp theo

→ Part III: **Design Patterns — FP & Classical** — Chapter 16: **GoF → FP Translation**. Strategy=HOF, Observer=EventEmitter, Command=DU, Visitor=switch, Decorator=wrapper, Middleware=compose.
