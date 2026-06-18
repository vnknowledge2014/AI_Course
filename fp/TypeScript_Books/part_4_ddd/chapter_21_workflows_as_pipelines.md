# Chapter 21 — Workflows as Pipelines

> **Bạn sẽ học được**:
> - Tại sao domain workflows nên là pipelines
> - `pipe()` và `flow()` — sync composition
> - Async pipelines — `Promise` chains dạng FP
> - Railway-Oriented Programming (ROP) preview — `Result` trong pipeline
> - Composing domain operations thành business workflows hoàn chỉnh
>
> **Yêu cầu trước**: Chapter 12 (composition, `pipe()`), Chapter 13 (ADTs, `Result`), Chapter 20 (domain modeling, state machines).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Biến domain operations thành pipelines — đọc như business workflow, test từng bước.

---

Bạn đã xem dây chuyền lắp ráp ô tô chưa?

Một chiếc xe không được "tạo ra cùng lúc" — nó đi qua từng **trạm**: hàn khung → lắp động cơ → đấu điện → sơn → kiểm tra chất lượng. Mỗi trạm nhận xe dở dang, làm MỘT việc, chuyền tiếp. Trạm sơn không biết (và không cần biết) trạm hàn khung làm gì — nó chỉ biết: "nhận khung xe → sơn → chuyền đi". Nếu bất kỳ trạm nào phát hiện lỗi (khung xe cong), toàn bộ dây chuyền dừng cho xe đó — nhưng các xe khác tiếp tục chạy.

**Workflows as Pipelines** biến business process thành dây chuyền: mỗi bước (function) = một trạm, data "chảy" qua pipeline. `pipe(input, step1, step2, step3)` = xe đi qua 3 trạm. Sync pipeline cho logic thuần. Async pipeline cho operations cần I/O. Railway-Oriented Programming (ROP) cho error handling — "track chính" (happy path) và "track lỗi" (error path) chạy song song.

---

## 21.1 — Tại sao Workflows nên là Pipelines?

### Imperative workflow — khó đọc, khó test

Hãy xem cách "truyền thống" viết business workflow: một function dài, if/else chồng chất, side effects rải rác. Đọc xong bạn không biết flow đi đường nào — phải trace manually. Test? Phải mock tất cả dependencies cùng lúc.

```typescript
// filename: src/imperative_workflow.ts

// ❌ Imperative workflow — monolithic, hard to test
const processOrder_imperative = async (rawInput: unknown) => {
    // Step 1: Validate
    if (typeof rawInput !== "object" || rawInput === null) throw new Error("Invalid input");
    const input = rawInput as any;
    if (!input.customerId) throw new Error("Missing customerId");
    if (!Array.isArray(input.items) || input.items.length === 0) throw new Error("Empty items");

    // Step 2: Check stock
    for (const item of input.items) {
        const stock = await db.getStock(item.productId);
        if (stock < item.quantity) throw new Error(`Insufficient stock: ${item.productId}`);
    }

    // Step 3: Calculate pricing
    let subtotal = 0;
    for (const item of input.items) {
        subtotal += item.price * item.quantity;
    }
    const tax = Math.round(subtotal * 0.1);
    const total = subtotal + tax;

    // Step 4: Create order
    const orderId = await db.createOrder({ customerId: input.customerId, items: input.items, total });

    // Step 5: Send confirmation
    await emailService.send(input.customerId, `Order ${orderId} confirmed!`);

    return { orderId, total };
};

// Vấn đề:
// 1. Không thể test step 3 (pricing) mà không mock db + email
// 2. Error handling = throw everywhere
// 3. Đọc 30 dòng mới hiểu flow
// 4. Muốn thêm "apply coupon" → sửa GIỮA function
```

### Pipeline workflow — rõ ràng, testable

```typescript
// filename: src/pipeline_workflow.ts
import assert from "node:assert/strict";

// ✅ Pipeline: mỗi step là một function. Compose lại thành workflow.

// Types
type RawOrderInput = {
    readonly customerId: string;
    readonly items: readonly { readonly productId: string; readonly price: number; readonly quantity: number }[];
};

type ValidatedOrder = {
    readonly customerId: string;
    readonly items: readonly { readonly productId: string; readonly price: number; readonly quantity: number }[];
};

type PricedOrder = ValidatedOrder & {
    readonly subtotal: number;
    readonly tax: number;
    readonly total: number;
};

// Step 1: Validate (pure)
const validateOrder = (input: RawOrderInput): ValidatedOrder | null =>
    input.customerId.length > 0 && input.items.length > 0
        ? { customerId: input.customerId, items: input.items }
        : null;

// Step 2: Calculate pricing (pure)
const calculatePricing = (order: ValidatedOrder): PricedOrder => {
    const subtotal = order.items.reduce((sum, item) => sum + item.price * item.quantity, 0);
    const tax = Math.round(subtotal * 0.1);
    return { ...order, subtotal, tax, total: subtotal + tax };
};

// Test từng step RIÊNG BIỆT — no mocking!
const input: RawOrderInput = {
    customerId: "C1",
    items: [
        { productId: "P1", price: 100000, quantity: 2 },
        { productId: "P2", price: 500000, quantity: 1 },
    ],
};

const validated = validateOrder(input);
assert.ok(validated !== null);

const priced = calculatePricing(validated!);
assert.strictEqual(priced.subtotal, 700000);
assert.strictEqual(priced.tax, 70000);
assert.strictEqual(priced.total, 770000);

// Pipeline: input → validate → price → ... (thêm steps dễ dàng)
console.log("Pipeline workflow OK ✅");
```

Đọc pipeline: `input → validateOrder → calculatePricing → ...` — hiểu flow trong 1 dòng. Test từng step: `assert.strictEqual(priced.tax, 70000)` — không cần mock. Thêm step "apply coupon"? Chèn giữa `calculatePricing` và step tiếp — không sửa code hiện tại.

> **💡 Pipeline = dây chuyền**: data chảy qua từng trạm. Mỗi trạm pure, testable riêng. Thêm/bớt trạm = thêm/bớt function trong pipe().

---

## ✅ Checkpoint 21.1

> Đến đây bạn phải hiểu:
> 1. **Imperative workflow** = monolithic, hard to test, side effects scattered
> 2. **Pipeline workflow** = mỗi step là function, data chảy qua
> 3. **Testability**: test từng step riêng, không mock
> 4. **Readability**: đọc 1 dòng `pipe(input, step1, step2, step3)` → hiểu flow
>
> **Test nhanh**: Muốn thêm "apply discount" vào workflow — imperative vs pipeline?
> <details><summary>Đáp án</summary>**Imperative**: sửa GIỮA function, risk breaking other steps. **Pipeline**: chèn `applyDiscount` function vào pipe() — không sửa code hiện tại.</details>

---

## 21.2 — `pipe()` và `flow()` — Sync Composition

### `pipe()` = dây chuyền với nguyên liệu ban đầu

`pipe()` nhận **giá trị đầu vào** rồi truyền qua chuỗi functions. Ch12 đã giới thiệu `pipe()` — giờ dùng cho domain workflows. `pipe(rawInput, validate, enrich, price, format)` = xe vào dây chuyền, đi qua 4 trạm, ra thành phẩm.

```typescript
// filename: src/pipe_sync.ts
import assert from "node:assert/strict";

// Minimal pipe() — from Ch12
function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe<A, B, C, D, E>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E): E;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

// Domain workflow as pipe()
type RawProduct = { readonly name: string; readonly priceStr: string; readonly category: string };
type CleanProduct = { readonly name: string; readonly price: number; readonly category: string };
type EnrichedProduct = CleanProduct & { readonly slug: string; readonly taxIncluded: number };
type DisplayProduct = EnrichedProduct & { readonly displayPrice: string };

const cleanInput = (raw: RawProduct): CleanProduct => ({
    name: raw.name.trim(),
    price: Number(raw.priceStr),
    category: raw.category.toLowerCase(),
});

const enrichProduct = (product: CleanProduct): EnrichedProduct => ({
    ...product,
    slug: product.name.toLowerCase().replace(/\s+/g, "-"),
    taxIncluded: Math.round(product.price * 1.1),
});

const formatForDisplay = (product: EnrichedProduct): DisplayProduct => ({
    ...product,
    displayPrice: `${product.taxIncluded.toLocaleString()} VND (incl. tax)`,
});

// Pipeline!
const result = pipe(
    { name: " MacBook Pro ", priceStr: "50000000", category: "ELECTRONICS" },
    cleanInput,
    enrichProduct,
    formatForDisplay,
);

assert.strictEqual(result.name, "MacBook Pro");
assert.strictEqual(result.slug, "macbook-pro");
assert.strictEqual(result.taxIncluded, 55000000);
assert.strictEqual(result.category, "electronics");

console.log("pipe() sync OK ✅");
```

### `flow()` = dây chuyền CHƯA có nguyên liệu

`flow()` khác `pipe()`: nó TẠO dây chuyền trước, chưa đưa nguyên liệu vào. `const processProduct = flow(clean, enrich, format)` — bạn có dây chuyền hoàn chỉnh, reusable. Sau đó gọi `processProduct(rawInput)` bất cứ lúc nào.

```typescript
// filename: src/flow_sync.ts
import assert from "node:assert/strict";

// flow() = compose functions into a new function
function flow<A, B>(ab: (a: A) => B): (a: A) => B;
function flow<A, B, C>(ab: (a: A) => B, bc: (b: B) => C): (a: A) => C;
function flow<A, B, C, D>(ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): (a: A) => D;
function flow(
    ...fns: ((x: unknown) => unknown)[]
): (x: unknown) => unknown {
    return (x) => fns.reduce((acc, fn) => fn(acc), x);
}

// Reusable types from earlier
type RawProduct = { readonly name: string; readonly priceStr: string; readonly category: string };
type CleanProduct = { readonly name: string; readonly price: number; readonly category: string };
type EnrichedProduct = CleanProduct & { readonly slug: string; readonly taxIncluded: number };
type DisplayProduct = EnrichedProduct & { readonly displayPrice: string };

const cleanInput = (raw: RawProduct): CleanProduct => ({
    name: raw.name.trim(),
    price: Number(raw.priceStr),
    category: raw.category.toLowerCase(),
});

const enrichProduct = (product: CleanProduct): EnrichedProduct => ({
    ...product,
    slug: product.name.toLowerCase().replace(/\s+/g, "-"),
    taxIncluded: Math.round(product.price * 1.1),
});

const formatForDisplay = (product: EnrichedProduct): DisplayProduct => ({
    ...product,
    displayPrice: `${product.taxIncluded.toLocaleString()} VND (incl. tax)`,
});

// flow() = create reusable pipeline
const processProduct = flow(cleanInput, enrichProduct, formatForDisplay);

// Use it multiple times!
const laptop = processProduct({ name: " MacBook Pro ", priceStr: "50000000", category: "ELECTRONICS" });
const mouse = processProduct({ name: " Logitech MX ", priceStr: "2000000", category: "Accessories" });

assert.strictEqual(laptop.slug, "macbook-pro");
assert.strictEqual(mouse.slug, "logitech-mx");
assert.strictEqual(mouse.taxIncluded, 2200000);

console.log("flow() sync OK ✅");
```

### pipe() vs flow() — khi nào dùng cái nào?

| | `pipe()` | `flow()` |
|--|---------|---------|
| **Input** | Truyền giá trị ngay | Chưa có giá trị |
| **Kết quả** | Giá trị cuối cùng | Function mới (reusable) |
| **Dùng khi** | Transform 1 giá trị cụ thể | Tạo reusable pipeline |
| **Ví dụ** | `pipe(order, validate, price)` | `const process = flow(validate, price)` |
| **Analogy** | Xe VÀO dây chuyền ngay | Thiết kế dây chuyền, xe vào SAU |

> **💡 Rule of thumb**: Dùng `pipe()` cho one-off transforms. Dùng `flow()` khi cần reusable pipeline. Cả hai đều composable — flow() bên trong pipe(), pipe() kết quả của flow().

---

## ✅ Checkpoint 21.2

> Đến đây bạn phải hiểu:
> 1. **`pipe(value, f, g, h)`** = apply f, then g, then h. Data-first
> 2. **`flow(f, g, h)`** = create new function `(x) => h(g(f(x)))`. Reusable
> 3. **pipe() = run now**. flow() = build pipeline, run later
> 4. **Both preserve types**: each step's output = next step's input
>
> **Test nhanh**: `const format = flow(trim, lowercase, slugify)` — `format` là gì?
> <details><summary>Đáp án</summary>**Function** `(input: string) => string`. Chưa chạy! Gọi `format("Hello World")` mới chạy. `flow()` tạo pipeline reusable.</details>

---

## 21.3 — Async Pipelines

### Một số trạm cần thời gian

Trong dây chuyền thật, trạm sơn cần 2 giờ khô. Trạm kiểm tra cần đợi kết quả lab. Trong code: truy cập database, gọi API, gửi email = async operations. Pipeline vẫn hoạt động — nhưng mỗi trạm return `Promise`, và pipeline phải `await` từng bước.

```typescript
// filename: src/async_pipeline.ts
import assert from "node:assert/strict";

// Async pipe — chains Promises
const pipeAsync = async <T>(
    value: T,
    ...fns: ((x: any) => any | Promise<any>)[]
): Promise<any> => {
    let result: any = value;
    for (const fn of fns) {
        result = await fn(result);
    }
    return result;
};

// Domain types
type OrderInput = {
    readonly customerId: string;
    readonly items: readonly { readonly productId: string; readonly quantity: number }[];
};

type ValidatedInput = OrderInput & { readonly validatedAt: Date };

type PricedOrder = ValidatedInput & {
    readonly subtotal: number;
    readonly tax: number;
    readonly total: number;
};

type ConfirmedOrder = PricedOrder & {
    readonly orderId: string;
    readonly confirmedAt: Date;
};

// Step 1: Validate (sync — nhanh)
const validateInput = (input: OrderInput): ValidatedInput => {
    if (input.items.length === 0) throw new Error("Empty items");
    return { ...input, validatedAt: new Date() };
};

// Step 2: Enrich with prices (async — query DB)
const enrichWithPrices = async (input: ValidatedInput): Promise<PricedOrder> => {
    // Simulate DB lookup
    const prices: Record<string, number> = { "P1": 100000, "P2": 500000 };
    const subtotal = input.items.reduce((sum, item) => {
        const price = prices[item.productId] ?? 0;
        return sum + price * item.quantity;
    }, 0);
    const tax = Math.round(subtotal * 0.1);
    return { ...input, subtotal, tax, total: subtotal + tax };
};

// Step 3: Save order (async — write DB)
const saveOrder = async (order: PricedOrder): Promise<ConfirmedOrder> => {
    // Simulate DB save
    const orderId = `ORD-${Date.now()}`;
    return { ...order, orderId, confirmedAt: new Date() };
};

// Async pipeline
const processOrder = async (input: OrderInput): Promise<ConfirmedOrder> =>
    pipeAsync(input, validateInput, enrichWithPrices, saveOrder);

// Test
const run = async () => {
    const result = await processOrder({
        customerId: "C1",
        items: [
            { productId: "P1", quantity: 2 },
            { productId: "P2", quantity: 1 },
        ],
    });

    assert.strictEqual(result.subtotal, 700000);
    assert.strictEqual(result.tax, 70000);
    assert.strictEqual(result.total, 770000);
    assert.ok(result.orderId.startsWith("ORD-"));

    console.log("Async pipeline OK ✅");
};

run();
```

### Promise chain = natural pipeline

```typescript
// filename: src/promise_chain.ts
import assert from "node:assert/strict";

// Alternative: Promise.then() chain = pipeline
type Input = { readonly value: number };
type Doubled = { readonly value: number; readonly doubled: true };
type Taxed = Doubled & { readonly tax: number; readonly total: number };

const double = (input: Input): Doubled => ({
    value: input.value * 2,
    doubled: true,
});

const addTax = async (input: Doubled): Promise<Taxed> => {
    // Simulate async tax lookup
    const taxRate = 0.1;
    const tax = Math.round(input.value * taxRate);
    return { ...input, tax, total: input.value + tax };
};

// Promise.then() chain
const result = Promise.resolve({ value: 100000 })
    .then(double)
    .then(addTax);

const run = async () => {
    const r = await result;
    assert.strictEqual(r.value, 200000);
    assert.strictEqual(r.tax, 20000);
    assert.strictEqual(r.total, 220000);

    console.log("Promise chain OK ✅");
};

run();
```

> **💡 Async pipeline patterns**: `pipeAsync()` = explicit sequential execution. `Promise.then()` chain = built-in. Cả hai đều đọc top-down, mỗi step transform data.

---

## ✅ Checkpoint 21.3

> Đến đây bạn phải hiểu:
> 1. **Async pipeline** = `pipeAsync(input, syncStep, asyncStep, asyncStep)`
> 2. **Promise.then() chain** = natural pipeline. `.then(validate).then(save)`
> 3. **Mix sync + async**: async pipeline handles both seamlessly
> 4. **Each step** still testable independently — sync steps without mock
>
> **Test nhanh**: `validateInput(input)` cần `await` không?
> <details><summary>Đáp án</summary>**KHÔNG!** Sync function — return value trực tiếp. `pipeAsync` tự handle: nếu step return value (not Promise), vẫn đi tiếp. Chỉ async steps cần await.</details>

---

## 21.4 — Railway-Oriented Programming (ROP) Preview

### Khi trạm phát hiện lỗi — xe rẽ sang track lỗi

Trong dây chuyền thật, nếu trạm kiểm tra phát hiện khung xe cong, xe KHÔNG tiếp tục qua trạm sơn — nó rẽ sang **track lỗi** (sửa chữa). Các trạm tiếp theo bị SKIP cho xe đó. Đây chính xác là Railway-Oriented Programming: hai track song song — **happy track** (success) và **error track** (failure). `Result<T, E>` = toa tàu có thể đi trên cả hai track.

Ý tưởng (sẽ đào sâu ở Ch22): mỗi step return `Result`. Nếu `ok` → tiếp tục. Nếu `err` → skip tất cả steps còn lại, "trượt" trên error track.

```typescript
// filename: src/rop_preview.ts
import assert from "node:assert/strict";

// Result type (from Ch13)
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// andThen = bind/flatMap for Result
// If ok → apply function. If err → skip (pass error through)
const andThen = <T, U, E>(
    result: Result<T, E>,
    fn: (value: T) => Result<U, E>
): Result<U, E> =>
    result.tag === "ok" ? fn(result.value) : result;

// map = transform ok value, leave err untouched
const map = <T, U, E>(
    result: Result<T, E>,
    fn: (value: T) => U
): Result<U, E> =>
    result.tag === "ok" ? ok(fn(result.value)) : result;

// --- Order workflow with ROP ---

type OrderError =
    | "empty_items"
    | "invalid_customer"
    | "insufficient_stock"
    | "price_calculation_error";

type RawOrder = {
    readonly customerId: string;
    readonly items: readonly { readonly productId: string; readonly quantity: number }[];
};

type ValidOrder = RawOrder & { readonly validated: true };
type PricedOrder = ValidOrder & { readonly total: number; readonly tax: number };

// Step 1: Validate
const validateOrder = (raw: RawOrder): Result<ValidOrder, OrderError> =>
    raw.customerId.length === 0 ? err("invalid_customer")
    : raw.items.length === 0 ? err("empty_items")
    : ok({ ...raw, validated: true as const });

// Step 2: Check stock
const checkStock = (order: ValidOrder): Result<ValidOrder, OrderError> => {
    // Simulate: all items in stock
    const outOfStock = order.items.find(i => i.quantity > 100);
    return outOfStock ? err("insufficient_stock") : ok(order);
};

// Step 3: Calculate price
const calculatePrice = (order: ValidOrder): Result<PricedOrder, OrderError> => {
    const prices: Record<string, number> = { "P1": 100000, "P2": 500000 };
    const total = order.items.reduce((sum, item) => {
        const price = prices[item.productId];
        return price ? sum + price * item.quantity : sum;
    }, 0);
    if (total === 0) return err("price_calculation_error");
    const tax = Math.round(total * 0.1);
    return ok({ ...order, total, tax });
};

// ROP Pipeline: chain with andThen
const processOrder = (raw: RawOrder): Result<PricedOrder, OrderError> => {
    const validated = validateOrder(raw);
    const stocked = andThen(validated, checkStock);
    const priced = andThen(stocked, calculatePrice);
    return priced;
};

// Alternative: more concise
const processOrder2 = (raw: RawOrder): Result<PricedOrder, OrderError> =>
    andThen(andThen(validateOrder(raw), checkStock), calculatePrice);

// Test: happy path
const goodOrder = processOrder({
    customerId: "C1",
    items: [{ productId: "P1", quantity: 2 }, { productId: "P2", quantity: 1 }],
});
assert.strictEqual(goodOrder.tag, "ok");
if (goodOrder.tag === "ok") {
    assert.strictEqual(goodOrder.value.total, 700000);
    assert.strictEqual(goodOrder.value.tax, 70000);
}

// Test: error path — empty items
const badOrder1 = processOrder({ customerId: "C1", items: [] });
assert.strictEqual(badOrder1.tag, "err");
if (badOrder1.tag === "err") {
    assert.strictEqual(badOrder1.error, "empty_items");
}

// Test: error path — steps after error are SKIPPED
const badOrder2 = processOrder({ customerId: "", items: [{ productId: "P1", quantity: 1 }] });
assert.strictEqual(badOrder2.tag, "err");
if (badOrder2.tag === "err") {
    assert.strictEqual(badOrder2.error, "invalid_customer");
    // checkStock and calculatePrice were NEVER called!
}

console.log("ROP preview OK ✅");
```

> **💡 ROP = error handling without try/catch**: `andThen` tự "rẽ ray" khi gặp err. Không cần `if (result.error)` ở mỗi step. Error propagates automatically — giống tàu trượt trên error track.

---

## ✅ Checkpoint 21.4

> Đến đây bạn phải hiểu:
> 1. **ROP** = 2 tracks: happy (ok) + error (err). Error → skip remaining steps
> 2. **`andThen(result, fn)`** = if ok → apply fn. If err → pass through
> 3. **`map(result, fn)`** = transform ok value, leave err untouched
> 4. **No try/catch**: Result + andThen = type-safe error propagation
>
> **Test nhanh**: `andThen(err("oops"), expensiveOperation)` — `expensiveOperation` có chạy không?
> <details><summary>Đáp án</summary>**KHÔNG!** `andThen` check: result là `err` → return err ngay, SKIP fn. Giống xe trên error track — không vào trạm tiếp.</details>

---

## 21.5 — Composing Domain Operations into Workflows

### Full workflow: E-commerce order processing

Kết hợp tất cả: sync `pipe()` cho pure steps, async cho I/O steps, `Result` cho error handling. Workflow đọc như business process — mỗi dòng = một bước nghiệp vụ.

```typescript
// filename: src/full_workflow.ts
import assert from "node:assert/strict";

// === Types ===
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

const andThen = <T, U, E>(
    result: Result<T, E>,
    fn: (value: T) => Result<U, E>
): Result<U, E> => result.tag === "ok" ? fn(result.value) : result;

const map = <T, U, E>(
    result: Result<T, E>,
    fn: (value: T) => U
): Result<U, E> => result.tag === "ok" ? ok(fn(result.value)) : result;

// === Domain Types ===
type WorkflowError =
    | { readonly tag: "validation_error"; readonly field: string; readonly message: string }
    | { readonly tag: "stock_error"; readonly productId: string; readonly available: number; readonly requested: number }
    | { readonly tag: "payment_error"; readonly reason: string };

type CustomerInfo = {
    readonly customerId: string;
    readonly email: string;
    readonly tier: "standard" | "silver" | "gold";
};

type OrderItem = {
    readonly productId: string;
    readonly productName: string;
    readonly quantity: number;
    readonly unitPrice: number;
};

type ValidatedOrder = {
    readonly customer: CustomerInfo;
    readonly items: readonly OrderItem[];
};

type PricedOrder = ValidatedOrder & {
    readonly subtotal: number;
    readonly discount: number;
    readonly tax: number;
    readonly total: number;
};

type ConfirmedOrder = PricedOrder & {
    readonly orderId: string;
    readonly confirmedAt: Date;
};

// === Workflow Steps (pure) ===

// Step 1: Validate customer
const validateCustomer = (customer: CustomerInfo): Result<CustomerInfo, WorkflowError> =>
    customer.customerId.length === 0
        ? err({ tag: "validation_error", field: "customerId", message: "Customer ID required" })
    : !customer.email.includes("@")
        ? err({ tag: "validation_error", field: "email", message: "Invalid email" })
    : ok(customer);

// Step 2: Validate items
const validateItems = (items: readonly OrderItem[]): Result<readonly OrderItem[], WorkflowError> =>
    items.length === 0
        ? err({ tag: "validation_error", field: "items", message: "At least 1 item required" })
    : items.some(i => i.quantity <= 0)
        ? err({ tag: "validation_error", field: "quantity", message: "Quantity must be > 0" })
    : ok(items);

// Step 3: Build validated order (combine validations)
const buildValidatedOrder = (
    customer: CustomerInfo,
    items: readonly OrderItem[]
): Result<ValidatedOrder, WorkflowError> =>
    andThen(validateCustomer(customer), validCustomer =>
        map(validateItems(items), validItems => ({
            customer: validCustomer,
            items: validItems,
        }))
    );

// Step 4: Calculate pricing
const TIER_DISCOUNTS: Record<string, number> = { standard: 0, silver: 0.05, gold: 0.1 };

const calculatePricing = (order: ValidatedOrder): PricedOrder => {
    const subtotal = order.items.reduce(
        (sum, item) => sum + item.unitPrice * item.quantity, 0
    );
    const discountRate = TIER_DISCOUNTS[order.customer.tier] ?? 0;
    const discount = Math.round(subtotal * discountRate);
    const taxableAmount = subtotal - discount;
    const tax = Math.round(taxableAmount * 0.1);
    return { ...order, subtotal, discount, tax, total: taxableAmount + tax };
};

// Step 5: Generate order ID (pure — deterministic for testing)
const generateOrderId = (order: PricedOrder, now: Date): ConfirmedOrder => ({
    ...order,
    orderId: `ORD-${now.getTime()}`,
    confirmedAt: now,
});

// === Compose Workflow ===
const processOrderWorkflow = (
    customer: CustomerInfo,
    items: readonly OrderItem[],
    now: Date,
): Result<ConfirmedOrder, WorkflowError> =>
    map(
        map(
            buildValidatedOrder(customer, items),
            calculatePricing,
        ),
        order => generateOrderId(order, now),
    );

// === Tests ===
const now = new Date("2024-06-15T10:00:00Z");

// Happy path: Gold customer
const goldResult = processOrderWorkflow(
    { customerId: "C1", email: "an@mail.com", tier: "gold" },
    [
        { productId: "P1", productName: "Laptop", quantity: 1, unitPrice: 20000000 },
        { productId: "P2", productName: "Mouse", quantity: 2, unitPrice: 500000 },
    ],
    now,
);

assert.strictEqual(goldResult.tag, "ok");
if (goldResult.tag === "ok") {
    const order = goldResult.value;
    assert.strictEqual(order.subtotal, 21000000);   // 20M + 1M
    assert.strictEqual(order.discount, 2100000);     // 10% gold discount
    assert.strictEqual(order.tax, 1890000);           // (21M - 2.1M) * 10%
    assert.strictEqual(order.total, 20790000);        // 18.9M + 1.89M
    assert.ok(order.orderId.startsWith("ORD-"));
}

// Error path: missing email
const badEmailResult = processOrderWorkflow(
    { customerId: "C2", email: "not-email", tier: "standard" },
    [{ productId: "P1", productName: "Laptop", quantity: 1, unitPrice: 20000000 }],
    now,
);
assert.strictEqual(badEmailResult.tag, "err");
if (badEmailResult.tag === "err") {
    assert.strictEqual(badEmailResult.error.tag, "validation_error");
}

// Error path: empty items
const emptyResult = processOrderWorkflow(
    { customerId: "C3", email: "c3@mail.com", tier: "silver" },
    [],
    now,
);
assert.strictEqual(emptyResult.tag, "err");

console.log("Full workflow OK ✅");
```

Flow đọc như business process: validate customer → validate items → build order → calculate pricing → confirm. Mỗi step testable riêng. Error tại bất kỳ step nào → skip remaining steps. Compiler ensures types flow correctly qua pipeline.

---

## ✅ Checkpoint 21.5

> Đến đây bạn phải hiểu:
> 1. **Workflow = composed steps**: validate → build → price → confirm
> 2. **`andThen` + `map`** chain steps. Error at any point → skip rest
> 3. **Pure workflow**: all steps pure, fully testable, no mocking
> 4. **Type safety**: compiler ensures step N output matches step N+1 input
>
> **Test nhanh**: Thêm step "apply coupon" sau calculatePricing — cần sửa gì?
> <details><summary>Đáp án</summary>Viết `const applyCoupon = (order: PricedOrder): PricedOrder => ...` rồi chèn `map(result, applyCoupon)` vào pipeline. Không sửa code hiện tại — chỉ thêm 1 function + 1 dòng map().</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): pipe() và flow()

```typescript
// Viết pipeline xử lý tên người dùng:
// 1. trim()
// 2. toLowerCase()
// 3. Thay khoảng trắng bằng "-"
// 4. Giới hạn 20 ký tự
//
// Dùng cả pipe() (cho 1 giá trị) và flow() (tạo reusable function)
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

function pipe<A>(a: A): A;
function pipe<A, B>(a: A, ab: (a: A) => B): B;
function pipe<A, B, C>(a: A, ab: (a: A) => B, bc: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): D;
function pipe<A, B, C, D, E>(a: A, ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E): E;
function pipe(a: unknown, ...fns: ((x: unknown) => unknown)[]): unknown {
    return fns.reduce((acc, fn) => fn(acc), a);
}

function flow<A, B>(ab: (a: A) => B): (a: A) => B;
function flow<A, B, C>(ab: (a: A) => B, bc: (b: B) => C): (a: A) => C;
function flow<A, B, C, D>(ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D): (a: A) => D;
function flow<A, B, C, D, E>(ab: (a: A) => B, bc: (b: B) => C, cd: (c: C) => D, de: (d: D) => E): (a: A) => E;
function flow(...fns: ((x: unknown) => unknown)[]): (x: unknown) => unknown {
    return (x) => fns.reduce((acc, fn) => fn(acc), x);
}

const trim = (s: string) => s.trim();
const toLower = (s: string) => s.toLowerCase();
const replaceSpaces = (s: string) => s.replace(/\s+/g, "-");
const limit20 = (s: string) => s.slice(0, 20);

// pipe() — one-off
const slug1 = pipe("  Hello World FP  ", trim, toLower, replaceSpaces, limit20);
assert.strictEqual(slug1, "hello-world-fp");

// flow() — reusable
const slugify = flow(trim, toLower, replaceSpaces, limit20);

assert.strictEqual(slugify("  Hello World FP  "), "hello-world-fp");
assert.strictEqual(slugify("  A Very Long Product Name That Exceeds Twenty  "), "a-very-long-product-");
```

</details>

---

**Bài 2** (10 phút): Async workflow

```typescript
// Viết async pipeline cho "User Registration":
// Step 1: validateEmail(email) — sync, return Result
// Step 2: checkDuplicate(email) — async (simulate DB check)
// Step 3: hashPassword(password) — async (simulate bcrypt)
// Step 4: saveUser(email, hashedPassword) — async
//
// Test: happy path + duplicate email + invalid email
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

type RegError = "invalid_email" | "duplicate_email" | "weak_password" | "save_failed";

// Step 1: Validate (sync)
const validateEmail = (email: string): Result<string, RegError> =>
    email.includes("@") ? ok(email.toLowerCase().trim()) : err("invalid_email");

// Step 2: Check duplicate (async)
const existingEmails = new Set(["taken@mail.com"]);
const checkDuplicate = async (email: string): Promise<Result<string, RegError>> =>
    existingEmails.has(email) ? err("duplicate_email") : ok(email);

// Step 3: Hash password (async)
const hashPassword = async (password: string): Promise<Result<string, RegError>> =>
    password.length < 8 ? err("weak_password") : ok(`hashed_${password}`);

// Step 4: Save user (async)
const saveUser = async (email: string, hashedPassword: string): Promise<Result<{ id: string; email: string }, RegError>> =>
    ok({ id: `USER-${Date.now()}`, email });

// Workflow
const registerUser = async (email: string, password: string): Promise<Result<{ id: string; email: string }, RegError>> => {
    // Step 1: sync validation
    const validEmail = validateEmail(email);
    if (validEmail.tag === "err") return validEmail;

    // Step 2: async duplicate check
    const unique = await checkDuplicate(validEmail.value);
    if (unique.tag === "err") return unique;

    // Step 3: async hash
    const hashed = await hashPassword(password);
    if (hashed.tag === "err") return hashed;

    // Step 4: async save
    return saveUser(unique.value, hashed.value);
};

// Test
const run = async () => {
    // Happy path
    const good = await registerUser("new@mail.com", "strongPassword123");
    assert.strictEqual(good.tag, "ok");

    // Duplicate
    const dup = await registerUser("taken@mail.com", "strongPassword123");
    assert.strictEqual(dup.tag, "err");
    if (dup.tag === "err") assert.strictEqual(dup.error, "duplicate_email");

    // Invalid email
    const bad = await registerUser("not-email", "strongPassword123");
    assert.strictEqual(bad.tag, "err");
    if (bad.tag === "err") assert.strictEqual(bad.error, "invalid_email");

    // Weak password
    const weak = await registerUser("ok@mail.com", "short");
    assert.strictEqual(weak.tag, "err");
    if (weak.tag === "err") assert.strictEqual(weak.error, "weak_password");

    console.log("Registration workflow OK ✅");
};

run();
```

</details>

---

**Bài 3** (15 phút): ROP workflow

```typescript
// Viết full ROP workflow cho "Invoice Processing":
// Step 1: validateInvoice(raw) → Result<Invoice, InvoiceError>
// Step 2: calculateTotals(invoice) → Result<PricedInvoice, InvoiceError>
// Step 3: applyLatePenalty(invoice, dueDate) → Result<FinalInvoice, InvoiceError>
// Step 4: formatForPrint(invoice) → string (always succeeds)
//
// Errors: "no_items", "invalid_amount", "negative_total"
// Dùng andThen + map để chain
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

const andThen = <T, U, E>(r: Result<T, E>, fn: (v: T) => Result<U, E>): Result<U, E> =>
    r.tag === "ok" ? fn(r.value) : r;

const map = <T, U, E>(r: Result<T, E>, fn: (v: T) => U): Result<U, E> =>
    r.tag === "ok" ? ok(fn(r.value)) : r;

type InvoiceError = "no_items" | "invalid_amount" | "negative_total";

type RawInvoice = {
    readonly customer: string;
    readonly items: readonly { readonly description: string; readonly amount: number }[];
    readonly dueDate: string;
};

type Invoice = {
    readonly customer: string;
    readonly items: readonly { readonly description: string; readonly amount: number }[];
    readonly dueDate: Date;
};

type PricedInvoice = Invoice & { readonly subtotal: number; readonly tax: number; readonly total: number };
type FinalInvoice = PricedInvoice & { readonly penalty: number; readonly finalTotal: number };

// Steps
const validateInvoice = (raw: RawInvoice): Result<Invoice, InvoiceError> => {
    if (raw.items.length === 0) return err("no_items");
    if (raw.items.some(i => i.amount <= 0)) return err("invalid_amount");
    return ok({ customer: raw.customer, items: raw.items, dueDate: new Date(raw.dueDate) });
};

const calculateTotals = (invoice: Invoice): Result<PricedInvoice, InvoiceError> => {
    const subtotal = invoice.items.reduce((sum, i) => sum + i.amount, 0);
    const tax = Math.round(subtotal * 0.1);
    const total = subtotal + tax;
    return total < 0 ? err("negative_total") : ok({ ...invoice, subtotal, tax, total });
};

const applyLatePenalty = (invoice: PricedInvoice, now: Date): Result<FinalInvoice, InvoiceError> => {
    const isLate = now > invoice.dueDate;
    const penalty = isLate ? Math.round(invoice.total * 0.05) : 0;
    return ok({ ...invoice, penalty, finalTotal: invoice.total + penalty });
};

const formatForPrint = (invoice: FinalInvoice): string =>
    `Invoice for ${invoice.customer}: ${invoice.finalTotal.toLocaleString()} VND` +
    (invoice.penalty > 0 ? ` (includes ${invoice.penalty.toLocaleString()} VND late fee)` : "");

// Workflow
const processInvoice = (raw: RawInvoice, now: Date): Result<string, InvoiceError> =>
    map(
        andThen(
            andThen(validateInvoice(raw), calculateTotals),
            inv => applyLatePenalty(inv, now),
        ),
        formatForPrint,
    );

// Test
const now = new Date("2024-07-01");

const onTime = processInvoice({
    customer: "Công ty ABC",
    items: [
        { description: "Consulting", amount: 10000000 },
        { description: "Development", amount: 30000000 },
    ],
    dueDate: "2024-07-15",
}, now);

assert.strictEqual(onTime.tag, "ok");
if (onTime.tag === "ok") {
    assert.ok(onTime.value.includes("44,000,000"));  // (40M + 4M tax) no penalty
    assert.ok(!onTime.value.includes("late fee"));
}

const late = processInvoice({
    customer: "Công ty XYZ",
    items: [{ description: "Service", amount: 5000000 }],
    dueDate: "2024-06-15",  // overdue!
}, now);

assert.strictEqual(late.tag, "ok");
if (late.tag === "ok") {
    assert.ok(late.value.includes("late fee"));
}

const noItems = processInvoice({ customer: "Empty", items: [], dueDate: "2024-08-01" }, now);
assert.strictEqual(noItems.tag, "err");

console.log("Invoice ROP OK ✅");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| pipe() type inference fails | Too many steps (>5) | Break into sub-pipelines. Or use explicit type annotations |
| andThen chain unreadable | Deeply nested | Use local variables: `const step1 = ...; const step2 = andThen(step1, ...)` |
| Async step in sync pipe | Mix Promise and value | Use `pipeAsync` or `.then()` chain for async |
| Error type too broad `string` | Untyped errors | Use discriminated union for errors: `{ tag: "validation"; ... }` |
| "I need to do side effects in middle" | Pipeline isn't purely pure | Use sandwich: pure pipeline for logic, IO at edges (Ch19) |

---

## Tóm tắt

Chương này biến domain operations thành dây chuyền lắp ráp — mỗi trạm (function) làm một việc, data chảy qua, lỗi rẽ sang error track.

- ✅ **Pipeline > Imperative**: readable, testable, composable. Đọc 1 dòng pipe() → hiểu flow.
- ✅ **`pipe(value, f, g, h)`**: apply ngay. **`flow(f, g, h)`**: tạo reusable pipeline.
- ✅ **Async pipelines**: `pipeAsync()` hoặc `.then()` chain. Mix sync + async.
- ✅ **ROP preview**: `Result<T,E>` + `andThen` = hai track. Error → skip remaining.
- ✅ **Full workflows**: compose validate → build → price → confirm. Type-safe, testable.

## Tiếp theo

→ Chapter 22: **Error Handling — neverthrow or Effect** — đào sâu ROP, neverthrow library, `ResultAsync`, error accumulation, và giới thiệu Effect.
