# Chapter 33 — Architecture Patterns

> **Bạn sẽ học được**:
> - Hexagonal Architecture (Ports & Adapters) — thực tế trong TypeScript
> - Functional Core / Imperative Shell — tách pure logic khỏi IO
> - Clean Architecture layers — domain → application → infrastructure
> - Module boundaries — barrel exports, dependency direction
> - Full example: E-Commerce Order module with all patterns
>
> **Yêu cầu trước**: Chapter 19 (Functional Architecture), Chapter 24 (Repository), Chapter 31 (TDD).
> **Thời gian đọc**: ~50 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Tổ chức TypeScript project theo architecture rõ ràng — testable, maintainable, scalable.

---

Bạn biết ổ cắm điện tiêu chuẩn không? Bất kỳ thiết bị nào — quạt, tivi, máy giặt — đều cắm vào CÙNG ổ. Ổ = **port**. Phích cắm = **adapter**. Thiết bị KHÔNG CẦN BIẾT điện đến từ nhà máy nhiệt điện hay thủy điện. Ổ tường = giao diện tiêu chuẩn.

**Hexagonal Architecture** (Ports & Adapters) = ổ cắm cho software. Domain ở TRUNG TÂM — không biết thế giới bên ngoài. Ports = ổ cắm (interfaces). Adapters = phích cắm (implementations). Swap adapter = swap implementation. Domain unchanged.

---

## Architecture Patterns — Từ code đến structure

Code compile + tests pass chưa đủ cho production. Bạn cần **architecture** — cách tổ chức code sao cho team 5-50 người có thể làm việc song song mà không conflict, features mới thêm được mà không break old code, và testing isolated ở mọi layer.

Chapter này dạy 3 patterns chính: **Hexagonal Architecture** (Ports & Adapters) — tách business logic khỏi infrastructure, **Functional Core / Imperative Shell** — pure logic bên trong, IO bên ngoài, và **Modular Monolith** — monolith với module boundaries rõ ràng (chuẩn bị cho microservices nếu cần).


## Architecture Patterns — Structure that scales

Hexagonal Architecture (Ports & Adapters) tách domain khỏi infrastructure. Functional Core / Imperative Shell giữ business logic PURE. Module boundaries chống feature creep. Kết quả: testable, maintainable, ready-to-scale.


## 33.1 — Hexagonal Architecture

```typescript
// filename: src/hexagonal/overview.ts

// ┌─────────────────────────────────────┐
// │           Infrastructure            │
// │  ┌───────────────────────────────┐  │
// │  │        Application            │  │
// │  │  ┌─────────────────────────┐  │  │
// │  │  │       DOMAIN            │  │  │
// │  │  │  (pure business logic)  │  │  │
// │  │  │  Value Objects, DUs     │  │  │
// │  │  │  Workflows, Rules       │  │  │
// │  │  └───────┬────┬────────────┘  │  │
// │  │          │    │ PORTS          │  │
// │  │  ┌───────┴────┴────────────┐  │  │
// │  │  │ Use Cases / Services    │  │  │
// │  │  │ (orchestrate domain)    │  │  │
// │  │  └─────────────────────────┘  │  │
// │  └───────────────────────────────┘  │
// │  ┌──────────┐  ┌──────────────────┐ │
// │  │ Adapter  │  │    Adapter       │ │
// │  │ (Prisma) │  │ (Express/Hono)   │ │
// │  └──────────┘  └──────────────────┘ │
// └─────────────────────────────────────┘

// Dependency Rule: arrows point INWARD
// Infrastructure → Application → Domain
// Domain NEVER imports Infrastructure

console.log("Hexagonal overview OK ✅");
```

### Full Hexagonal Example: Order Module

```typescript
// filename: src/hexagonal/order_domain.ts
import assert from "node:assert/strict";

// ═══ LAYER 1: DOMAIN (innermost — pure, no deps) ═══

type OrderId = string & { __brand: "OrderId" };
type Money = number & { __brand: "Money" };
const Money = (n: number): Money => n as Money;

type OrderItem = {
    readonly productId: string;
    readonly name: string;
    readonly quantity: number;
    readonly unitPrice: Money;
};

type Order = {
    readonly id: OrderId;
    readonly items: readonly OrderItem[];
    readonly status: "draft" | "confirmed" | "cancelled";
    readonly total: Money;
};

type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };

// Domain logic: PURE functions
const calculateTotal = (items: readonly OrderItem[]): Money =>
    Money(items.reduce((sum, item) => sum + item.quantity * item.unitPrice, 0));

const confirmOrder = (order: Order): Result<Order, string> =>
    order.status !== "draft"
        ? { tag: "err", error: `Cannot confirm ${order.status} order` }
        : { tag: "ok", value: { ...order, status: "confirmed" } };

const cancelOrder = (order: Order): Result<Order, string> =>
    order.status === "delivered" || order.status === "cancelled"
        ? { tag: "err", error: `Cannot cancel ${order.status} order` }
        : { tag: "ok", value: { ...order, status: "cancelled" } };

// ═══ LAYER 1.5: PORTS (interfaces — defined BY domain) ═══

type OrderRepository = {
    readonly findById: (id: OrderId) => Promise<Order | null>;
    readonly save: (order: Order) => Promise<void>;
};

type PaymentGateway = {
    readonly charge: (orderId: OrderId, amount: Money) => Promise<Result<string, string>>;
};

type NotificationService = {
    readonly sendConfirmation: (order: Order) => Promise<void>;
};

// ═══ LAYER 2: APPLICATION (use cases — orchestrate domain + ports) ═══

type AppDeps = {
    readonly orderRepo: OrderRepository;
    readonly payment: PaymentGateway;
    readonly notifications: NotificationService;
};

const confirmOrderUseCase = async (
    deps: AppDeps,
    orderId: OrderId,
): Promise<Result<Order, string>> => {
    // Step 1: Find (IO via port)
    const order = await deps.orderRepo.findById(orderId);
    if (!order) return { tag: "err", error: "Order not found" };

    // Step 2: Validate & transform (PURE domain)
    const confirmed = confirmOrder(order);
    if (confirmed.tag === "err") return confirmed;

    // Step 3: Charge (IO via port)
    const payment = await deps.payment.charge(orderId, order.total);
    if (payment.tag === "err") return { tag: "err", error: `Payment failed: ${payment.error}` };

    // Step 4: Save (IO via port)
    await deps.orderRepo.save(confirmed.value);

    // Step 5: Notify (IO via port — fire and forget)
    await deps.notifications.sendConfirmation(confirmed.value);

    return confirmed;
};

// ═══ LAYER 3: INFRASTRUCTURE (adapters — concrete implementations) ═══

// In-memory adapter for testing
const createInMemoryOrderRepo = (orders: readonly Order[]): OrderRepository => {
    const store = new Map<OrderId, Order>();
    for (const o of orders) store.set(o.id, o);
    return {
        findById: async (id) => store.get(id) ?? null,
        save: async (order) => { store.set(order.id, order); },
    };
};

const createFakePayment = (): PaymentGateway => ({
    charge: async (_id, _amount) => ({ tag: "ok", value: "PAY-001" }),
});

const createFakeNotifications = (): { service: NotificationService; sent: Order[] } => {
    const sent: Order[] = [];
    return {
        service: { sendConfirmation: async (order) => { sent.push(order); } },
        sent,
    };
};

// ═══ TEST: All layers together ═══
const run = async () => {
    const draftOrder: Order = {
        id: "ORD-1" as OrderId,
        items: [{ productId: "P1", name: "Laptop", quantity: 1, unitPrice: Money(20000000) }],
        status: "draft",
        total: Money(20000000),
    };

    const notifs = createFakeNotifications();
    const deps: AppDeps = {
        orderRepo: createInMemoryOrderRepo([draftOrder]),
        payment: createFakePayment(),
        notifications: notifs.service,
    };

    const result = await confirmOrderUseCase(deps, "ORD-1" as OrderId);
    assert.strictEqual(result.tag, "ok");
    if (result.tag === "ok") {
        assert.strictEqual(result.value.status, "confirmed");
    }
    assert.strictEqual(notifs.sent.length, 1);

    // Not found
    const notFound = await confirmOrderUseCase(deps, "ORD-999" as OrderId);
    assert.strictEqual(notFound.tag, "err");

    console.log("Hexagonal full example OK ✅");
};

run();
```

> **💡 "Ports belong to the domain"**: Domain DEFINES what it needs (interfaces). Infrastructure PROVIDES (implements). Not the other way around. This is Dependency Inversion Principle.

---

Nhìn lại code: `confirmOrderUseCase` gọi `deps.orderRepo.findById()`, `deps.payment.charge()`, `deps.notifications.sendConfirmation()` — nhưng **không import bất kỳ concrete implementation nào**. Nó chỉ biết interfaces (ports). Đây là Dependency Inversion — high-level module (use case) không phụ thuộc low-level module (Prisma, Stripe), cả hai phụ thuộc abstractions.

Lợi ích thực tế: bạn swap từ PostgreSQL sang MongoDB? Viết adapter mới, không sửa use case. Bạn test? Inject fake adapter, không cần database thật. Bạn thêm Stripe payment? Viết `StripeAdapter` implement `PaymentGateway`, inject vào `deps`.

Trong DMMF, Scott Wlaschin gọi pattern này là "onion architecture" — domain ở center (không depend on anything), mỗi layer bên ngoài depend on layer bên trong, không bao giờ ngược lại.

## ✅ Checkpoint 33.1

> Đến đây bạn phải hiểu:
> 1. **Hexagonal** = Domain (center) + Ports (interfaces) + Adapters (implementations)
> 2. **Dependency Rule**: arrows point inward. Domain never imports infrastructure
> 3. **Ports** = defined by domain. OrderRepository, PaymentGateway, NotificationService
> 4. **Adapters** = concrete impl. InMemoryRepo, PrismaRepo, StripeGateway
>
> **Test nhanh**: Chuyển payment từ Stripe sang PayPal — domain code sửa gì?
> <details><summary>Đáp án</summary>**KHÔNG SỬA GÌ!** PaymentGateway = port (interface). Stripe → PayPal = swap adapter. Domain + use cases unchanged.</details>

---

## 33.2 — Functional Core / Imperative Shell

### Nhân thuần khiết, vỏ IO

```typescript
// filename: src/fc_is.ts
import assert from "node:assert/strict";

// FUNCTIONAL CORE = pure functions, no IO, no side effects
// IMPERATIVE SHELL = IO orchestration, calls core functions

// === CORE (pure) ===
type CartItem = { productId: string; quantity: number; price: number };
type Discount = { type: "percentage"; value: number } | { type: "fixed"; value: number };

const applyDiscount = (subtotal: number, discount: Discount): number =>
    discount.type === "percentage"
        ? subtotal * (1 - discount.value / 100)
        : Math.max(0, subtotal - discount.value);

const calculateSubtotal = (items: readonly CartItem[]): number =>
    items.reduce((sum, item) => sum + item.quantity * item.price, 0);

const calculateShipping = (subtotal: number): number =>
    subtotal >= 500000 ? 0 : 30000;  // free shipping > 500k

const calculateCart = (items: readonly CartItem[], discount?: Discount) => {
    const subtotal = calculateSubtotal(items);
    const discounted = discount ? applyDiscount(subtotal, discount) : subtotal;
    const shipping = calculateShipping(discounted);
    return { subtotal, discounted, shipping, total: discounted + shipping };
};

// === Test CORE in isolation (no IO needed!) ===
const result = calculateCart(
    [{ productId: "P1", quantity: 2, price: 300000 }],
    { type: "percentage", value: 10 },
);
assert.strictEqual(result.subtotal, 600000);
assert.strictEqual(result.discounted, 540000);
assert.strictEqual(result.shipping, 0);      // > 500k = free
assert.strictEqual(result.total, 540000);

const small = calculateCart([{ productId: "P2", quantity: 1, price: 100000 }]);
assert.strictEqual(small.shipping, 30000);   // < 500k
assert.strictEqual(small.total, 130000);

// === SHELL (IO — calls core) ===
// const shell = async (cartId: string) => {
//     const items = await db.getCartItems(cartId);      // IO
//     const discount = await discountService.get(cartId);// IO
//     const result = calculateCart(items, discount);     // PURE!
//     await db.saveOrder(cartId, result);                // IO
//     await emailService.sendReceipt(cartId, result);    // IO
//     return result;
// };

console.log("FC/IS OK ✅");
```

---

## 33.3 — Module Boundaries

```typescript
// filename: src/module_boundaries.ts

// === Directory Structure (recommended) ===

// src/
//   modules/
//     orders/                      ← Module boundary
//       domain/
//         types.ts                 ← Order, OrderItem, OrderStatus
//         rules.ts                 ← confirmOrder, calculateTotal (PURE)
//       ports/
//         order-repository.ts      ← OrderRepository interface
//         payment-gateway.ts       ← PaymentGateway interface
//       application/
//         confirm-order.ts         ← Use case
//         create-order.ts
//       infrastructure/
//         prisma-order-repo.ts     ← Prisma adapter
//         stripe-payment.ts        ← Stripe adapter
//         in-memory-order-repo.ts  ← Test adapter
//       index.ts                   ← Barrel export (public API)
//     products/
//       ...
//     users/
//       ...

// === Barrel Export (index.ts) ===
// Only export what other modules need:
// export type { Order, OrderId } from './domain/types';
// export { confirmOrderUseCase } from './application/confirm-order';
// export type { OrderRepository } from './ports/order-repository';
// Do NOT export: internal rules, infrastructure details

console.log("Module boundaries OK ✅");
```

---

Hãy nhìn function `calculateCart`: nó nhận `items` và `discount`, trả về object `{ subtotal, discounted, shipping, total }`. Không database call, không API call, không side-effect nào. Bạn test nó bằng 2 dòng code: `calculateCart([...], {...})` → assert result. Done.

So sánh: nếu calculateCart gọi database để lấy shipping rates, bạn cần mock database, setup test fixtures, handle async... Test 2 dòng thành test 20 dòng. FC/IS tránh điều đó bằng cách **đẩy mọi IO ra shell**. Core chỉ tính toán thuần.

Pattern này xuất hiện ở mọi nơi trong production code: Redux reducers (pure state transitions), React render (pure UI), Express validators (pure validation), database mappers (pure transformation). Nhận diện được nó = bạn biết cách tổ chức bất kỳ codebase nào.

## ✅ Checkpoint 33.2-33.3

> Đến đây bạn phải hiểu:
> 1. **FC/IS**: Pure core (rules, calculations) + IO shell (DB, API, email)
> 2. **Test core without IO**: `calculateCart` = pure. Test directly
> 3. **Module boundaries**: barrel exports. Hide internals
> 4. **Directory structure**: domain/ → ports/ → application/ → infrastructure/
>
> **Test nhanh**: `calculateShipping(600000)` — cần database để test?
> <details><summary>Đáp án</summary>**KHÔNG!** Pure function. Input → output. No IO. Test instantly.</details>

---

## 🏋️ Bài tập

**Bài 1** (15 phút): Design hexagonal module cho "User Registration"

```typescript
// Thiết kế module structure:
// Domain: User type, validateEmail, validatePassword (pure)
// Ports: UserRepository, EmailService, PasswordHasher
// Application: registerUser use case
// Infrastructure: InMemory adapters
// Test: full use case with fakes
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

// Domain
type UserId = string & { __brand: "UserId" };
type User = { id: UserId; name: string; email: string; passwordHash: string };
type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };
type RegError = "invalid_email" | "weak_password" | "duplicate" | "hash_failed";

const validateEmail = (e: string): Result<string, RegError> =>
    e.includes("@") ? { tag: "ok", value: e.toLowerCase() } : { tag: "err", error: "invalid_email" };

const validatePassword = (p: string): Result<string, RegError> =>
    p.length >= 8 ? { tag: "ok", value: p } : { tag: "err", error: "weak_password" };

// Ports
type UserRepo = { findByEmail: (e: string) => Promise<User | null>; save: (u: User) => Promise<void> };
type EmailSvc = { sendWelcome: (u: User) => Promise<void> };
type Hasher = { hash: (pw: string) => Promise<Result<string, RegError>> };
type Deps = { repo: UserRepo; email: EmailSvc; hasher: Hasher; genId: () => UserId };

// Application
const register = async (deps: Deps, name: string, rawEmail: string, password: string): Promise<Result<User, RegError>> => {
    const emailR = validateEmail(rawEmail);
    if (emailR.tag === "err") return emailR;
    const pwR = validatePassword(password);
    if (pwR.tag === "err") return pwR;
    const existing = await deps.repo.findByEmail(emailR.value);
    if (existing) return { tag: "err", error: "duplicate" };
    const hashR = await deps.hasher.hash(pwR.value);
    if (hashR.tag === "err") return hashR;
    const user: User = { id: deps.genId(), name, email: emailR.value, passwordHash: hashR.value };
    await deps.repo.save(user);
    await deps.email.sendWelcome(user);
    return { tag: "ok", value: user };
};

// Infrastructure (fakes)
const run = async () => {
    const saved: User[] = [];
    const emails: string[] = [];
    let id = 0;
    const deps: Deps = {
        repo: { findByEmail: async () => null, save: async (u) => { saved.push(u); } },
        email: { sendWelcome: async (u) => { emails.push(u.email); } },
        hasher: { hash: async (pw) => ({ tag: "ok", value: `HASH(${pw})` }) },
        genId: () => `U-${++id}` as UserId,
    };
    const r = await register(deps, "An", "An@Mail.COM", "secure123");
    assert.strictEqual(r.tag, "ok");
    assert.strictEqual(saved.length, 1);
    assert.strictEqual(emails.length, 1);
    console.log("User registration OK ✅");
};
run();
```

</details>

---

## 💬 Đối thoại với bản thân: Architecture Q&A

**Q: Hexagonal Architecture quá phức tạp cho app nhỏ?**

A: Đúng — cho todo app 100 LOC, đừng dùng. Nhưng khi app > 5000 LOC với 3+ external services (DB, API, queue), Hexagonal TIẾT KIỆM thời gian. Không restructure = thêm feature mới ngày càng chậm.

**Q: "Functional Core / Imperative Shell" — shell viết ra sao?**

A: Shell = nơi duy nhất có side effects: database calls, HTTP requests, file I/O, logging. Shell GỌI core (pure functions) và THỰC HIỆN kết quả. Core quyết định WHAT, Shell thực hiện HOW.

**Q: Microservices hay Monolith?**

A: BẮT ĐẦU monolith. LUÔN LUÔN. Extract services khi: (1) team > 50 people, (2) deploy frequency khác nhau giữa modules, (3) scaling requirements khác nhau. Monolith with good module boundaries = chuẩn bị cho microservices mà KHÔNG chịu overhead.

---

## Tóm tắt

- ✅ **Hexagonal** = Domain (center) + Ports (interfaces) + Adapters (implementations).
- ✅ **Dependency Rule**: Domain never imports infrastructure. Arrows point inward.
- ✅ **Functional Core / Imperative Shell**: Pure logic testable without IO.
- ✅ **Module boundaries**: barrel exports. Hide internals, expose API.
- ✅ **All patterns combine**: Domain types (Ch20) + Repository (Ch24) + FP DI (Ch19) + TDD (Ch31).

## Tiếp theo

→ Chapter 34: **Backend — Hono/Express** — REST API, middleware, error handling with FP patterns.
