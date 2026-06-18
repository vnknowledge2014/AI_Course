# Chapter 19 — Functional Architecture

> **Bạn sẽ học được**:
> - Layered Architecture — các tầng và dependency rules
> - Functional Core / Imperative Shell — revisited ở quy mô lớn
> - Module boundaries — barrel exports, public vs internal API
> - IO at edges — pure domain core, impure shell
> - Dependency Injection the FP way — functions, not classes
> - Kiến trúc hoàn chỉnh cho TypeScript FP application
>
> **Yêu cầu trước**: Chapter 11 (FC/IS), Chapter 14 (boundary validation), Chapter 18 (DDD, bounded contexts).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Thiết kế ứng dụng TypeScript theo kiến trúc FP — pure core, impure edges.

---

Bạn đã bao giờ vào bếp một nhà hàng chuyên nghiệp?

Có sự phân chia rõ ràng: **phòng ăn** (nơi khách ngồi) tách biệt với **nhà bếp** (nơi nấu ăn). Trong bếp, mỗi trạm (station) có nhiệm vụ riêng: trạm grill, trạm sơ chế, trạm trang trí. Nguyên liệu thô vào cửa sau (nhận hàng), đi qua sơ chế → nấu → bày đĩa, rồi ra cửa trước (phục vụ). Bếp trưởng phối hợp tất cả, nhưng **không bao giờ nấu ở phòng ăn, và không bao giờ phục vụ khách trong bếp**.

**Functional Architecture** tổ chức code giống nhà bếp: **Domain** = trạm nấu (pure, chỉ biến đổi "nguyên liệu", không biết ai gọi món). **Infrastructure** = cửa nhận hàng và cửa giao hàng (IO: database, API, email). **Application** = bếp trưởng phối hợp flow. **Presentation** = phòng ăn tiếp khách. Và quy tắc vàng: **domain KHÔNG BAO GIỜ đi ra ngoài** — nó không biết database, HTTP, hay file system. IO chỉ xảy ra ở biên (edges).

---

## 19.1 — Layered Architecture

### Bốn tầng của nhà bếp

```
┌─────────────────────────────────────┐
│         PRESENTATION                │  ← API routes, CLI, UI
│      (Controllers, Handlers)        │
├─────────────────────────────────────┤
│         APPLICATION                 │  ← Orchestration, workflows
│      (Use Cases, Services)          │
├─────────────────────────────────────┤
│            DOMAIN                   │  ← Business logic (PURE!)
│   (Entities, Value Objects, Rules)  │
├─────────────────────────────────────┤
│        INFRASTRUCTURE               │  ← DB, HTTP, File I/O
│    (Repositories, External APIs)    │
└─────────────────────────────────────┘
```

### Dependency Rule: chỉ trỏ VÀO TRONG

Trong nhà hàng, phòng ăn biết bếp (gọi món), nhưng bếp KHÔNG biết phòng ăn (bếp không cần biết bàn nào gọi — chỉ cần nấu đúng). Domain là trạm nấu — nó nhận input, trả output, không cần biết ai gọi và output đi đâu.

```typescript
// filename: src/dependency_rule.ts

// ❌ WRONG: Domain depends on Infrastructure
// import { database } from "./infrastructure/database";
// const createUser = (name: string) => database.insert({ name });
// → Domain bị TIED to specific database!

// ✅ RIGHT: Domain KHÔNG biết Infrastructure
// Domain chỉ biết TYPES, không biết implementations

// Domain layer: pure types + pure functions
type User = {
    readonly id: string;
    readonly name: string;
    readonly email: string;
};

type CreateUserError = "invalid_email" | "name_too_short" | "duplicate_email";

// Pure domain function — KHÔNG biết database, HTTP, file system
const validateNewUser = (
    name: string,
    email: string
): { tag: "ok"; user: Omit<User, "id"> } | { tag: "err"; error: CreateUserError } => {
    if (name.length < 2) return { tag: "err", error: "name_too_short" };
    if (!email.includes("@")) return { tag: "err", error: "invalid_email" };
    return { tag: "ok", user: { name, email } };
};

// Infrastructure: implements details
// Application: orchestrates domain + infrastructure
// Presentation: handles HTTP/CLI interface
```

> **💡 Dependency Rule**: Inner layers KHÔNG bao giờ import outer layers. Domain không biết database. Database biết (implements) domain interfaces. Mũi tên dependency chỉ VÀO TRONG.

---

## ✅ Checkpoint 19.1

> Đến đây bạn phải hiểu:
> 1. **4 layers**: Presentation → Application → Domain ← Infrastructure
> 2. **Dependency Rule**: imports chỉ trỏ vào trong (domain = innermost)
> 3. **Domain = PURE**: không biết DB, HTTP, file system
> 4. **Infrastructure = IMPURE**: implements domain interfaces
>
> **Test nhanh**: Domain function `import fetch from "node-fetch"` — đúng hay sai?
> <details><summary>Đáp án</summary>**SAI!** Domain KHÔNG import infrastructure. `fetch` = HTTP = infrastructure. Domain nhận data đã fetched, không tự fetch.</details>

---

## 19.2 — Functional Core / Imperative Shell (Revisited)

### Trạm nấu (pure) và bếp trưởng (orchestrator)

Ở Ch11, bạn đã học FC/IS ở quy mô nhỏ. Giờ mở rộng lên application thật: **Functional Core** = tất cả pure domain functions (tính giá, validate, state transitions). **Imperative Shell** = application layer orchestrates: đọc DB → gọi pure functions → ghi DB → gửi email. Shell CALLS core — giống bếp trưởng gọi từng trạm: "Sơ chế xong chưa? → Đưa sang grill → Trang trí → Ra phục vụ."

Điểm then chốt: bạn test Functional Core KHÔNG CẦN mock. `calculateSubtotal(items)` = pure, input → output, `assert.strictEqual` done. Chỉ Shell (use cases) mới cần fake dependencies — và FP DI cho phép dùng object literals thay mock library.

```typescript
// filename: src/fc_is_scaled.ts
import assert from "node:assert/strict";

// === FUNCTIONAL CORE (Domain) ===
// Pure functions, immutable types, NO side effects

type Product = {
    readonly id: string;
    readonly name: string;
    readonly price: number;
    readonly stock: number;
};

type CartItem = {
    readonly productId: string;
    readonly quantity: number;
    readonly unitPrice: number;
};

type Cart = {
    readonly items: readonly CartItem[];
    readonly customerId: string;
};

type OrderSummary = {
    readonly customerId: string;
    readonly items: readonly CartItem[];
    readonly subtotal: number;
    readonly tax: number;
    readonly total: number;
};

// Pure domain logic
const calculateSubtotal = (items: readonly CartItem[]): number =>
    items.reduce((sum, item) => sum + item.unitPrice * item.quantity, 0);

const calculateTax = (subtotal: number, taxRate: number): number =>
    Math.round(subtotal * taxRate);

const createOrderSummary = (cart: Cart, taxRate: number): OrderSummary => {
    const subtotal = calculateSubtotal(cart.items);
    const tax = calculateTax(subtotal, taxRate);
    return {
        customerId: cart.customerId,
        items: cart.items,
        subtotal,
        tax,
        total: subtotal + tax,
    };
};

const canFulfill = (product: Product, requestedQty: number): boolean =>
    product.stock >= requestedQty;

// Test pure core
const cart: Cart = {
    customerId: "C1",
    items: [
        { productId: "P1", quantity: 2, unitPrice: 100000 },
        { productId: "P2", quantity: 1, unitPrice: 500000 },
    ],
};

const summary = createOrderSummary(cart, 0.1);
assert.strictEqual(summary.subtotal, 700000);  // 200K + 500K
assert.strictEqual(summary.tax, 70000);
assert.strictEqual(summary.total, 770000);

console.log("FC/IS scaled OK ✅");
```

### Imperative Shell — orchestrates impure operations

```typescript
// filename: src/imperative_shell.ts

// === IMPERATIVE SHELL (Application + Infrastructure) ===
// Side effects ONLY here: DB, HTTP, logging, email

// Infrastructure interfaces (TYPES in domain, IMPLEMENTATIONS in infra)
type ProductRepository = {
    readonly findById: (id: string) => Promise<Product | null>;
    readonly updateStock: (id: string, newStock: number) => Promise<void>;
};

type OrderRepository = {
    readonly save: (order: OrderSummary) => Promise<string>;  // returns orderId
};

type EmailService = {
    readonly sendConfirmation: (customerId: string, orderId: string) => Promise<void>;
};

type Product = {
    readonly id: string;
    readonly name: string;
    readonly price: number;
    readonly stock: number;
};

type CartItem = {
    readonly productId: string;
    readonly quantity: number;
    readonly unitPrice: number;
};

type OrderSummary = {
    readonly customerId: string;
    readonly items: readonly CartItem[];
    readonly subtotal: number;
    readonly tax: number;
    readonly total: number;
};

// Application service — orchestrates FC + IS
const placeOrder = async (
    cart: { customerId: string; items: readonly CartItem[] },
    taxRate: number,
    // Dependencies injected as parameters (FP DI!)
    productRepo: ProductRepository,
    orderRepo: OrderRepository,
    emailService: EmailService,
): Promise<{ tag: "ok"; orderId: string } | { tag: "err"; error: string }> => {

    // 1. Check stock (IMPURE: reads DB)
    for (const item of cart.items) {
        const product = await productRepo.findById(item.productId);
        if (!product) return { tag: "err", error: `Product ${item.productId} not found` };
        // Use PURE domain function
        if (product.stock < item.quantity) {
            return { tag: "err", error: `Insufficient stock for ${product.name}` };
        }
    }

    // 2. Create order summary (PURE: domain logic)
    const subtotal = cart.items.reduce((sum, item) => sum + item.unitPrice * item.quantity, 0);
    const tax = Math.round(subtotal * taxRate);
    const summary: OrderSummary = {
        customerId: cart.customerId,
        items: cart.items,
        subtotal,
        tax,
        total: subtotal + tax,
    };

    // 3. Save order (IMPURE: writes DB)
    const orderId = await orderRepo.save(summary);

    // 4. Update stock (IMPURE: writes DB) — reuse data from step 1
    for (const item of cart.items) {
        const product = await productRepo.findById(item.productId);
        if (product) {
            await productRepo.updateStock(product.id, product.stock - item.quantity);
        }
    }

    // 5. Send email (IMPURE: external service)
    await emailService.sendConfirmation(cart.customerId, orderId);

    return { tag: "ok", orderId };
};
```

> **💡 Pattern**: Imperative Shell CALLS pure functions, never the other way. Shell handles: sequence of operations, error handling, side effects. Core handles: business rules, calculations, validations.

---

## ✅ Checkpoint 19.2

> Đến đây bạn phải hiểu:
> 1. **Functional Core** = pure functions, immutable types. Testable without mocking
> 2. **Imperative Shell** = orchestrates impure ops: DB, HTTP, email
> 3. **Shell CALLS core**, never reverse. Core = tested isolated
> 4. **FC/IS at scale** = domain layer (pure) + application layer (shell)
>
> **Test nhanh**: `calculateTax(1000, 0.1)` cần mock database không?
> <details><summary>Đáp án</summary>**KHÔNG!** Pure function — input → output. Không cần mock gì. Đây là sức mạnh FC/IS: domain logic testable 100% không mock.</details>

---

## 19.3 — Module Boundaries & Barrel Exports

### Cửa ra vào cho mỗi trạm

Giống mỗi trạm trong bếp có cửa sổ phục vụ riêng (pass window) — bạn nhận đồ qua cửa sổ, không cần biết bên trong trạm bày biện thế nào. **Barrel exports** (`index.ts`) là cửa sổ phục vụ: chỉ export PUBLIC API, che giấu internal implementation.

### Public API via `index.ts`

```typescript
// filename: src/modules/order/index.ts

// Barrel export = PUBLIC API of this module
// Chỉ export những gì consumers CẦN

export type { Order, OrderStatus, OrderSummary } from "./types";
export { createOrder, calculateTotal, applyDiscount } from "./domain";
export { validateOrder } from "./validation";

// KHÔNG export:
// - Internal helpers
// - Implementation details
// - Private types
```

### Ví dụ project structure

```
src/
├── modules/
│   ├── order/                     # Order bounded context
│   │   ├── types.ts               # Order, OrderStatus, LineItem
│   │   ├── domain.ts              # Pure business logic
│   │   ├── validation.ts          # Order validation (branded types)
│   │   ├── events.ts              # OrderPlaced, OrderConfirmed
│   │   ├── index.ts               # ← PUBLIC API (barrel export)
│   │   └── __tests__/
│   │       ├── domain.test.ts
│   │       └── validation.test.ts
│   │
│   ├── payment/                   # Payment bounded context
│   │   ├── types.ts
│   │   ├── domain.ts
│   │   ├── index.ts               # ← PUBLIC API
│   │   └── __tests__/
│   │
│   └── shipping/                  # Shipping bounded context
│       ├── types.ts
│       ├── domain.ts
│       └── index.ts               # ← PUBLIC API
│
├── infrastructure/                # Infrastructure layer
│   ├── database/
│   │   ├── connection.ts
│   │   ├── order-repository.ts    # implements OrderRepository
│   │   └── index.ts
│   ├── email/
│   │   └── email-service.ts       # implements EmailService
│   └── http/
│       └── api-client.ts
│
├── application/                   # Application layer (use cases)
│   ├── place-order.ts             # placeOrder use case
│   ├── cancel-order.ts
│   └── index.ts
│
└── presentation/                  # Presentation layer
    ├── routes/
    │   ├── order-routes.ts
    │   └── payment-routes.ts
    └── middleware/
```

### Import rules (enforced by convention or linter)

```typescript
// filename: src/import_rules.ts

// ✅ ALLOWED imports:
// Presentation → Application → Domain
// Infrastructure → Domain (implements interfaces)

// ❌ FORBIDDEN imports:
// Domain → Infrastructure (domain không biết DB)
// Domain → Application (domain không biết use cases)
// Domain → Presentation (domain không biết HTTP)

// ✅ Import from module's public API
import { Order, createOrder, validateOrder } from "./modules/order";

// ❌ Import from internal path
// import { internalHelper } from "./modules/order/internal/helper";
```

---

## ✅ Checkpoint 19.3

> Đến đây bạn phải hiểu:
> 1. **Barrel exports** = `index.ts` chỉ export public API
> 2. **Module = bounded context**: `order/`, `payment/`, `shipping/`
> 3. **Import rules**: outer → inner OK. Inner → outer FORBIDDEN
> 4. **Convention**: import from `"./modules/order"` không phải `"./modules/order/internal/helper"`
>
> **Test nhanh**: File `order/domain.ts` import từ `infrastructure/database` — đúng hay sai?
> <details><summary>Đáp án</summary>**SAI!** Domain không import infrastructure. Dependency rule violation. Domain chỉ import domain types.</details>

---

## 19.4 — IO at Edges

### Cửa nhận hàng → Bếp → Cửa phục vụ

Nguyên liệu thô (raw data) vào qua cửa nhận hàng (IO: read from DB/API). Bếp chế biến (pure: transform, validate, calculate). Thành phẩm ra cửa phục vụ (IO: write to DB, send email). Giữa bếp — **không có IO**. Bếp không tự đi mua rau, không tự bưng ra bàn. Đây là **Sandwich pattern**: IO (bread) → Pure (filling) → IO (bread).

"Filling càng lớn, sandwich càng ngon" = pure middle càng rộng, code càng dễ test. Push ALL IO to the edges. Bên trong = pure, fully testable, no mocks.

```typescript
// filename: src/io_at_edges.ts

// === Pattern: "Push IO to the edges" ===

// ❌ IO scattered everywhere
const processOrderBad = async (orderId: string) => {
    const order = await db.findOrder(orderId);       // IO inside
    if (order.total > 1000000) {
        await sendEmail(order.customerId, "VIP!");    // IO inside
        logger.info("VIP order processed");            // IO inside
    }
    const tax = order.total * 0.1;                    // business logic
    await db.updateOrder(orderId, { tax });           // IO inside
};

// ✅ IO at edges — pure core, impure boundary
// Step 1: Read (IO)
// Step 2: Process (PURE)
// Step 3: Write (IO)

type Order = {
    readonly id: string;
    readonly customerId: string;
    readonly total: number;
};

type ProcessResult = {
    readonly order: Order;
    readonly tax: number;
    readonly isVip: boolean;
    readonly notifications: readonly string[];
};

// PURE — no IO
const processOrderPure = (order: Order): ProcessResult => ({
    order,
    tax: Math.round(order.total * 0.1),
    isVip: order.total > 1000000,
    notifications: order.total > 1000000
        ? [`VIP order for customer ${order.customerId}`]
        : [],
});

// IMPURE — only at edges
const processOrderShell = async (
    orderId: string,
    db: { findOrder: (id: string) => Promise<Order>; updateTax: (id: string, tax: number) => Promise<void> },
    notify: (messages: readonly string[]) => Promise<void>,
) => {
    // Edge 1: READ (IO)
    const order = await db.findOrder(orderId);

    // Core: PROCESS (PURE)
    const result = processOrderPure(order);

    // Edge 2: WRITE (IO)
    await db.updateTax(orderId, result.tax);

    // Edge 3: SIDE EFFECTS (IO)
    if (result.notifications.length > 0) {
        await notify(result.notifications);
    }

    return result;
};
```

### Sandwich pattern

```
IO (read) → Pure (process) → IO (write)
```

```typescript
// filename: src/sandwich.ts
import assert from "node:assert/strict";

// Sandwich: Read → Process → Write

// 1. READ: get raw data from external source
type RawInput = { readonly name: string; readonly priceStr: string; readonly stockStr: string };

// 2. PROCESS: pure transformation
type ValidProduct = {
    readonly name: string;
    readonly price: number;
    readonly stock: number;
};

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

const parseProduct = (input: RawInput): Result<ValidProduct, string> => {
    const price = Number(input.priceStr);
    if (Number.isNaN(price) || price <= 0) return err("Invalid price");

    const stock = Number(input.stockStr);
    if (Number.isNaN(stock) || !Number.isInteger(stock) || stock < 0)
        return err("Invalid stock");

    if (input.name.trim().length < 2) return err("Name too short");

    return ok({ name: input.name.trim(), price, stock });
};

// 3. WRITE: save valid data to external store

// Test PURE part — no mocking needed!
const valid = parseProduct({ name: "Laptop", priceStr: "20000000", stockStr: "10" });
assert.strictEqual(valid.tag, "ok");
if (valid.tag === "ok") {
    assert.strictEqual(valid.value.price, 20000000);
    assert.strictEqual(valid.value.stock, 10);
}

const invalid = parseProduct({ name: "L", priceStr: "abc", stockStr: "10" });
assert.strictEqual(invalid.tag, "err");

console.log("IO at edges OK ✅");
```

> **💡 Sandwich rule**: Keep the PURE middle as large as possible. Push ALL IO to the edges. The pure middle is fully testable without mocks.

---

## ✅ Checkpoint 19.4

> Đến đây bạn phải hiểu:
> 1. **IO at edges** = read (impure) → process (pure) → write (impure)
> 2. **Sandwich pattern**: IO bread, pure filling. Pure filling = largest possible
> 3. **Pure middle** = fully testable, no mocks, no DB
> 4. **Side effects** = notifications, logging, emails — all at edges
>
> **Test nhanh**: `processOrderPure(order)` có thể test bằng `assert.strictEqual` không cần mock không?
> <details><summary>Đáp án</summary>**CÓ!** Pure function — input → output. `assert.strictEqual(result.tax, 70000)`. No DB, no HTTP, no mock.</details>

---

## 19.5 — Dependency Injection: The FP Way

### Không cần class, không cần constructor — chỉ cần function parameters

OOP Dependency Injection cần: class + constructor + DI container (Spring, NestJS). FP DI đơn giản hơn: **pass dependencies as function parameters**. Test? Tạo object literal thỏa interface — không cần mock library, không cần Sinon, không cần Jest mock.

Reader pattern mở rộng ý tưởng: factory function nhận dependencies MỘT LẦN, return focused functions đã "chôn" deps trong closure. Giống tạo trạm bếp với nguyên liệu gia vị sẵn — mỗi lần nấu không cần truyền lại muối, tiêu.

```typescript
// filename: src/fp_di.ts
import assert from "node:assert/strict";

// ❌ OOP DI: class + constructor injection
// class OrderService {
//     constructor(
//         private orderRepo: OrderRepository,
//         private emailService: EmailService,
//     ) {}
//     async placeOrder(cart: Cart) { ... }
// }

// ✅ FP DI: functions + parameter injection

// Dependencies as TYPES (interfaces)
type Logger = {
    readonly info: (msg: string) => void;
    readonly error: (msg: string) => void;
};

type Clock = {
    readonly now: () => Date;
};

type IdGenerator = {
    readonly generate: () => string;
};

// Domain function — receives dependencies as LAST parameters
const createInvoice = (
    customerId: string,
    amount: number,
    // Dependencies
    clock: Clock,
    idGen: IdGenerator,
): { id: string; customerId: string; amount: number; createdAt: Date } => ({
    id: idGen.generate(),
    customerId,
    amount,
    createdAt: clock.now(),
});

// Production dependencies
const realClock: Clock = { now: () => new Date() };
const realIdGen: IdGenerator = { generate: () => crypto.randomUUID() };

// Test dependencies — NO MOCKING LIBRARY needed!
const testClock: Clock = { now: () => new Date("2024-01-15T10:00:00Z") };
const testIdGen: IdGenerator = { generate: () => "TEST-ID-001" };

// Test
const invoice = createInvoice("C1", 500000, testClock, testIdGen);
assert.strictEqual(invoice.id, "TEST-ID-001");
assert.strictEqual(invoice.customerId, "C1");
assert.strictEqual(invoice.amount, 500000);
assert.deepStrictEqual(invoice.createdAt, new Date("2024-01-15T10:00:00Z"));

console.log("FP DI OK ✅");
```

### Reader pattern — dependency as closure

```typescript
// filename: src/reader_pattern.ts
import assert from "node:assert/strict";

// Reader pattern: function factory that captures dependencies

type Deps = {
    readonly taxRate: number;
    readonly discountRate: number;
    readonly freeShippingThreshold: number;
};

// createPricingService: inject deps ONCE, return focused functions
const createPricingService = (deps: Deps) => ({
    calculateTotal: (subtotal: number): number =>
        Math.round(subtotal * (1 + deps.taxRate) * (1 - deps.discountRate)),

    getShippingCost: (subtotal: number): number =>
        subtotal >= deps.freeShippingThreshold ? 0 : 30000,

    calculateFinalPrice: (subtotal: number): number => {
        const total = Math.round(subtotal * (1 + deps.taxRate) * (1 - deps.discountRate));
        const shipping = subtotal >= deps.freeShippingThreshold ? 0 : 30000;
        return total + shipping;
    },
});

// Production config
const prodPricing = createPricingService({
    taxRate: 0.1,
    discountRate: 0,
    freeShippingThreshold: 500000,
});

// Test config — different deps!
const testPricing = createPricingService({
    taxRate: 0.1,
    discountRate: 0.2,
    freeShippingThreshold: 100000,
});

assert.strictEqual(prodPricing.calculateTotal(1000000), 1100000);
// 1M * 1.1 * 1.0 = 1.1M

assert.strictEqual(testPricing.calculateTotal(1000000), 880000);
// 1M * 1.1 * 0.8 = 880K

assert.strictEqual(prodPricing.getShippingCost(300000), 30000);  // < threshold
assert.strictEqual(prodPricing.getShippingCost(600000), 0);      // >= threshold

console.log("Reader pattern OK ✅");
```

---

## ✅ Checkpoint 19.5

> Đến đây bạn phải hiểu:
> 1. **FP DI** = pass dependencies as function parameters. No class, no constructor
> 2. **Test without mocks**: create test dependencies với object literals
> 3. **Reader pattern** = factory function captures deps via closure
> 4. **OOP DI ≈ FP Reader**: class constructor ≈ factory function closure
>
> **Test nhanh**: `const testLogger: Logger = { info: () => {}, error: () => {} }` — cần mock library không?
> <details><summary>Đáp án</summary>**KHÔNG!** TypeScript structural typing: object literal thỏa `Logger` interface. No Jest mock, no Sinon stub. Object literal = test double.</details>

---

## 19.6 — Putting It Together: Full Architecture

### Toàn bộ nhà bếp: từ cửa nhận hàng đến phòng ăn

Ví dụ tổng hợp kết nối mọi tầng: Domain (pure types + functions) → Infrastructure interfaces (defined in domain) → Application (use cases = shell) → Test (object literals = test doubles). Chú ý: domain KHÔNG import gì ngoài chính nó. Application import domain + infra interfaces. Test tạo fake deps bằng object literal — zero mock library.

```typescript
// filename: src/architecture_complete.ts
import assert from "node:assert/strict";

// =============================================
// LAYER 1: DOMAIN (Pure — innermost)
// =============================================

type Money = number & { readonly __brand: "Money" };
const Money = (amount: number): Money => amount as Money;

type OrderId = string & { readonly __brand: "OrderId" };
type CustomerId = string & { readonly __brand: "CustomerId" };

type OrderItem = {
    readonly productName: string;
    readonly unitPrice: Money;
    readonly quantity: number;
};

type Order = {
    readonly id: OrderId;
    readonly customerId: CustomerId;
    readonly items: readonly OrderItem[];
    readonly status: "draft" | "confirmed" | "shipped";
};

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// Pure domain functions
const calculateItemTotal = (item: OrderItem): Money =>
    Money(item.unitPrice * item.quantity);

const calculateOrderTotal = (order: Order): Money =>
    Money(order.items.reduce((sum, item) => sum + calculateItemTotal(item), 0));

const canConfirm = (order: Order): Result<Order, string> =>
    order.status === "draft"
        ? ok(order)
        : err(`Cannot confirm order with status: ${order.status}`);

const confirmOrder = (order: Order): Order => ({
    ...order,
    status: "confirmed",
});

const canShip = (order: Order): Result<Order, string> =>
    order.status === "confirmed"
        ? ok(order)
        : err(`Cannot ship order with status: ${order.status}`);

const shipOrder = (order: Order): Order => ({
    ...order,
    status: "shipped",
});

// =============================================
// LAYER 2: INFRASTRUCTURE INTERFACES (in Domain)
// =============================================

type OrderRepository = {
    readonly findById: (id: OrderId) => Promise<Order | null>;
    readonly save: (order: Order) => Promise<void>;
};

type NotificationService = {
    readonly sendOrderConfirmed: (customerId: CustomerId, orderId: OrderId) => Promise<void>;
    readonly sendOrderShipped: (customerId: CustomerId, orderId: OrderId) => Promise<void>;
};

// =============================================
// LAYER 3: APPLICATION (Use Cases — Shell)
// =============================================

const confirmOrderUseCase = async (
    orderId: OrderId,
    repo: OrderRepository,
    notifications: NotificationService,
): Promise<Result<Order, string>> => {
    // IO: read
    const order = await repo.findById(orderId);
    if (!order) return err("Order not found");

    // Pure: validate + transform
    const validation = canConfirm(order);
    if (validation.tag === "err") return validation;

    const confirmed = confirmOrder(order);

    // IO: write
    await repo.save(confirmed);
    await notifications.sendOrderConfirmed(confirmed.customerId, confirmed.id);

    return ok(confirmed);
};

// =============================================
// TEST (no real DB, no real email — FP DI!)
// =============================================

const testOrder: Order = {
    id: "ORD-1" as OrderId,
    customerId: "C-1" as CustomerId,
    items: [
        { productName: "Laptop", unitPrice: Money(20000000), quantity: 1 },
        { productName: "Mouse", unitPrice: Money(500000), quantity: 2 },
    ],
    status: "draft",
};

// Test pure domain (no IO, no mock)
assert.strictEqual(calculateOrderTotal(testOrder), 21000000);

const confirmResult = canConfirm(testOrder);
assert.strictEqual(confirmResult.tag, "ok");

const confirmed = confirmOrder(testOrder);
assert.strictEqual(confirmed.status, "confirmed");

const reconfirm = canConfirm(confirmed);
assert.strictEqual(reconfirm.tag, "err");  // already confirmed!

// Test use case with fake dependencies
const fakeRepo: OrderRepository = {
    findById: async (id) => id === testOrder.id ? testOrder : null,
    save: async () => {},
};

const fakeNotifications: NotificationService = {
    sendOrderConfirmed: async () => {},
    sendOrderShipped: async () => {},
};

const run = async () => {
    const result = await confirmOrderUseCase(
        "ORD-1" as OrderId, fakeRepo, fakeNotifications
    );
    assert.strictEqual(result.tag, "ok");
    if (result.tag === "ok") {
        assert.strictEqual(result.value.status, "confirmed");
    }

    const notFound = await confirmOrderUseCase(
        "ORD-999" as OrderId, fakeRepo, fakeNotifications
    );
    assert.strictEqual(notFound.tag, "err");

    console.log("Full architecture OK ✅");
};

run();
```

---

## ✅ Checkpoint 19.6

> Đến đây bạn phải hiểu:
> 1. **Domain** = pure functions + types. Testable without any mock
> 2. **Infrastructure interfaces** = defined in domain, implemented in infra
> 3. **Application** = use cases. Orchestrates domain + infra. The "shell"
> 4. **Testing** = object literals as test doubles. No mock library needed
>
> **Test nhanh**: `calculateOrderTotal(order)` test cần `fakeRepo` không?
> <details><summary>Đáp án</summary>**KHÔNG!** Pure domain function. `assert.strictEqual(calculateOrderTotal(order), 21000000)`. Repo chỉ cần cho use case (shell), không cần cho domain (core).</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): IO Sandwich

```typescript
// Refactor function sau thành IO sandwich:
//
// const processUser = async (userId: string) => {
//     const user = await db.findUser(userId);
//     if (!user) throw new Error("Not found");
//     const age = calculateAge(user.birthDate);  // pure
//     const tier = determineTier(user.totalSpent);  // pure
//     await db.updateTier(userId, tier);
//     await sendEmail(user.email, `New tier: ${tier}`);
//     return { user, age, tier };
// };
//
// Tách: pure middle function + impure shell
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

// PURE middle
type UserData = {
    readonly birthDate: string;
    readonly totalSpent: number;
    readonly email: string;
};

type UserProfile = {
    readonly age: number;
    readonly tier: "standard" | "silver" | "gold";
};

const calculateAge = (birthDate: string): number => {
    const birth = new Date(birthDate);
    const now = new Date("2024-06-15");  // deterministic for testing
    return Math.floor((now.getTime() - birth.getTime()) / (365.25 * 24 * 60 * 60 * 1000));
};

const determineTier = (totalSpent: number): "standard" | "silver" | "gold" =>
    totalSpent >= 50000000 ? "gold"
    : totalSpent >= 10000000 ? "silver"
    : "standard";

// Pure function — fully testable
const processUserPure = (user: UserData): UserProfile => ({
    age: calculateAge(user.birthDate),
    tier: determineTier(user.totalSpent),
});

// IMPURE shell
const processUserShell = async (
    userId: string,
    db: { findUser: (id: string) => Promise<UserData | null>; updateTier: (id: string, tier: string) => Promise<void> },
    email: { send: (to: string, msg: string) => Promise<void> },
) => {
    // IO: read
    const user = await db.findUser(userId);
    if (!user) return { tag: "err" as const, error: "Not found" };

    // Pure: process
    const profile = processUserPure(user);

    // IO: write
    await db.updateTier(userId, profile.tier);
    await email.send(user.email, `New tier: ${profile.tier}`);

    return { tag: "ok" as const, value: profile };
};

// Test PURE part only
const profile = processUserPure({
    birthDate: "1990-01-15",
    totalSpent: 15000000,
    email: "test@mail.com",
});
assert.strictEqual(profile.tier, "silver");
assert.strictEqual(profile.age, 34);
```

</details>

---

**Bài 2** (10 phút): Module boundaries

```typescript
// Thiết kế module structure cho "Blog System":
// - Contexts: Post, Comment, User
// - Mỗi context: types.ts, domain.ts, index.ts
// - Viết barrel exports (index.ts) cho Post context
// - Viết import rules (allowed/forbidden)
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
// === src/modules/post/types.ts ===
type PostId = string & { readonly __brand: "PostId" };
type AuthorId = string & { readonly __brand: "AuthorId" };

type Post = {
    readonly id: PostId;
    readonly title: string;
    readonly content: string;
    readonly authorId: AuthorId;
    readonly status: "draft" | "published" | "archived";
    readonly publishedAt?: Date;
};

type CreatePostInput = {
    readonly title: string;
    readonly content: string;
    readonly authorId: AuthorId;
};

// === src/modules/post/domain.ts ===
const createPost = (id: PostId, input: CreatePostInput): Post => ({
    id,
    ...input,
    status: "draft",
});

const publishPost = (post: Post, now: Date): Post =>
    post.status === "draft"
        ? { ...post, status: "published", publishedAt: now }
        : post;

const archivePost = (post: Post): Post =>
    post.status === "published"
        ? { ...post, status: "archived" }
        : post;

// === src/modules/post/index.ts (barrel export) ===
// export type { Post, PostId, CreatePostInput } from "./types";
// export { createPost, publishPost, archivePost } from "./domain";
// 
// NOT exported: internal helpers, validation logic details

// === Import rules ===
// ✅ src/application/create-post.ts → import from "./modules/post"
// ✅ src/modules/comment/domain.ts → import { PostId } from "../post"
// ❌ src/modules/post/domain.ts → import from "../../infrastructure/database"
// ❌ src/modules/post/types.ts → import from "../comment/internal/helper"
```

</details>

---

**Bài 3** (15 phút): FP Dependency Injection

```typescript
// Viết payment processing system:
// Dependencies: PaymentGateway, FraudDetector, AuditLogger
// Use case: processPayment(amount, cardToken, deps)
//
// 1. Define dependency interfaces
// 2. Write pure validation (amount > 0, card format)
// 3. Write use case (shell): validate → check fraud → charge → log
// 4. Test: create fake deps, test happy + error paths
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

// Dependency interfaces
type PaymentGateway = {
    readonly charge: (amount: number, token: string) => Promise<{ transactionId: string }>;
};

type FraudDetector = {
    readonly check: (amount: number, token: string) => Promise<{ isFraud: boolean; score: number }>;
};

type AuditLogger = {
    readonly log: (event: string, data: Record<string, unknown>) => Promise<void>;
};

type PaymentDeps = {
    readonly gateway: PaymentGateway;
    readonly fraud: FraudDetector;
    readonly audit: AuditLogger;
};

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// PURE validation
const validatePayment = (amount: number, cardToken: string): Result<void, string> => {
    if (amount <= 0) return err("Amount must be > 0");
    if (amount > 100000000) return err("Amount exceeds max limit");
    if (cardToken.length < 10) return err("Invalid card token");
    return ok(undefined);
};

// USE CASE (shell)
const processPayment = async (
    amount: number,
    cardToken: string,
    deps: PaymentDeps,
): Promise<Result<{ transactionId: string }, string>> => {
    // Pure: validate
    const validation = validatePayment(amount, cardToken);
    if (validation.tag === "err") return validation;

    // IO: check fraud
    const fraudCheck = await deps.fraud.check(amount, cardToken);
    if (fraudCheck.isFraud) {
        await deps.audit.log("fraud_detected", { amount, score: fraudCheck.score });
        return err("Payment flagged as fraud");
    }

    // IO: charge
    try {
        const result = await deps.gateway.charge(amount, cardToken);
        await deps.audit.log("payment_success", { amount, transactionId: result.transactionId });
        return ok(result);
    } catch (e) {
        await deps.audit.log("payment_failed", { amount, error: String(e) });
        return err(`Payment failed: ${String(e)}`);
    }
};

// TEST — all fake deps, no mock library
const logs: Array<{ event: string; data: Record<string, unknown> }> = [];

const fakeDeps: PaymentDeps = {
    gateway: { charge: async (amount, _token) => ({ transactionId: `TXN-${amount}` }) },
    fraud: { check: async (_amount, _token) => ({ isFraud: false, score: 0.1 }) },
    audit: { log: async (event, data) => { logs.push({ event, data }); } },
};

// Test pure validation
assert.strictEqual(validatePayment(-100, "tok_123456789").tag, "err");
assert.strictEqual(validatePayment(50000, "tok_123456789").tag, "ok");

// Test use case
const run = async () => {
    const result = await processPayment(500000, "tok_123456789abc", fakeDeps);
    assert.strictEqual(result.tag, "ok");
    if (result.tag === "ok") {
        assert.strictEqual(result.value.transactionId, "TXN-500000");
    }
    assert.strictEqual(logs.length, 1);
    assert.strictEqual(logs[0].event, "payment_success");

    // Test fraud detected
    const fraudDeps: PaymentDeps = {
        ...fakeDeps,
        fraud: { check: async () => ({ isFraud: true, score: 0.95 }) },
    };

    const fraudResult = await processPayment(500000, "tok_123456789abc", fraudDeps);
    assert.strictEqual(fraudResult.tag, "err");

    console.log("Payment DI OK ✅");
};

run();
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Domain imports database | Dependency rule violation | Move DB access to infra. Domain define interfaces only |
| Cannot test without mock library | Too many side effects in domain | Extract pure functions. IO at edges. FP DI |
| Barrel exports too large | Module doing too much | Split module. Each module = 1 bounded context |
| Circular dependencies | Modules import each other | Introduce shared kernel for common types. Events for communication |
| "Everything is a use case" | Over-engineering | Simple CRUD doesn't need layers. DDD for complex domains only |

---

## Tóm tắt

Chương này đã bày biện nhà bếp hoàn chỉnh — từ phân chia trạm (layered architecture) qua Functional Core / Imperative Shell (bếp pure, bếp trưởng orchestrate), module boundaries (cửa sổ phục vụ cho mỗi trạm), IO at edges (sandwich pattern), đến FP DI (truyền nguyên liệu, không cần DI container).

- ✅ **4 Layers**: Presentation → Application → Domain ← Infrastructure. Dependency rule: inward.
- ✅ **FC/IS at scale**: Domain = pure core. Application = imperative shell. Shell CALLS core.
- ✅ **Module boundaries**: barrel exports = public API. Import rules enforced.
- ✅ **IO at edges**: Sandwich: Read (IO) → Process (Pure) → Write (IO).
- ✅ **FP DI**: Dependencies as function parameters. Object literals = test doubles. No mock library.
- ✅ **Architecture**: Domain types + pure functions → infra interfaces → use cases → presentation.

## Tiếp theo

→ Chapter 20: **Domain Modeling with DUs** — Value Objects as branded types, State machines via discriminated unions, complex domain hierarchies.
