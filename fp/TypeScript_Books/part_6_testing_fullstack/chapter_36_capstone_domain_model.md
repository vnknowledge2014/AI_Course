# Chapter 36 — Capstone Part 1: Domain Model ⭐

> **Bạn sẽ học được**:
> - Áp dụng TẤT CẢ patterns từ Ch12-35 vào MỘT project hoàn chỉnh
> - DDD domain model với branded types, DUs, state machines
> - Workflows as pipelines với typed errors
> - Repository pattern + FP DI
> - Validation (Applicative), serialization (DTOs), testing (TDD + PBT)
> - Architecture: Hexagonal, Functional Core / Imperative Shell
>
> **Yêu cầu trước**: Chapters 12-35 (all previous content).
> **Thời gian đọc**: ~60 phút | **Level**: Principal
> **Kết quả cuối cùng**: Một domain model hoàn chỉnh cho E-Commerce order system — production-grade.

---

Đây là chương "thi cuối kỳ" — không phải kiểm tra, mà là MỘT project gom tất cả kiến thức. Như xây NHÀ: Ch12-15 dạy cách trộn xi măng, đặt gạch, hàn thép. Ch16-24 dạy thiết kế phòng, hệ thống điện nước. Ch25-30 dạy nội thất cao cấp. Ch31-35 dạy kiểm tra chất lượng. Chương này: XÂY CẢ CĂN NHÀ.

---

## Capstone Part 1: Full Domain Model

Mọi concept từ 35 chapters trước kết hợp lại.

Bạn sẽ xây dựng domain model cho hệ thống order management: branded types cho domain values, discriminated unions cho states, `pipe()` + `Result` cho workflows, Zod cho validation, vitest cho TDD. Đây là blueprint mà bạn sẽ copy-paste cho mọi dự án production.


## Capstone Part 1 — Full Domain Model

Tổng hợp tất cả patterns: Branded types (Ch15) + DUs (Ch20) + State machines (Ch20) + Workflows (Ch21) + Error handling (Ch22) → complete e-commerce order system. Code chạy được, test được, production-ready domain logic.


## 36.1 — Project: E-Commerce Order System

### Domain Analysis

```typescript
// filename: src/capstone/domain_analysis.ts

// === Ubiquitous Language (from DDD — Ch18) ===
// Customer places an Order
// Order contains OrderItems (product, quantity, price)
// Order goes through states: Draft → Confirmed → Paid → Shipped → Delivered
// Order can be Cancelled from Draft or Confirmed
// Payment is processed via PaymentGateway
// Confirmation sends notification to Customer
// Order total = sum(item.qty * item.price) - discount + shipping

// === Bounded Context: Order Management ===
// Entities: Order (aggregate root)
// Value Objects: OrderId, CustomerId, Money, Address, Email
// Domain Events: OrderConfirmed, OrderPaid, OrderShipped
// Services: ConfirmOrder, PayOrder, ShipOrder

console.log("Domain analysis OK ✅");
```

---

## 36.2 — Domain Types (Ch13, Ch20)

```typescript
// filename: src/capstone/domain/types.ts
import assert from "node:assert/strict";

// === Branded Types (Ch14 — Smart Constructors) ===
type OrderId = string & { readonly __brand: "OrderId" };
type CustomerId = string & { readonly __brand: "CustomerId" };
type ProductId = string & { readonly __brand: "ProductId" };
type Email = string & { readonly __brand: "Email" };
type Money = number & { readonly __brand: "Money" };

const OrderId = (s: string): OrderId => s as OrderId;
const CustomerId = (s: string): CustomerId => s as CustomerId;
const ProductId = (s: string): ProductId => s as ProductId;
const Money = (n: number): Money => Math.round(n * 100) / 100 as unknown as Money;

// === Value Objects ===
type Address = {
    readonly street: string;
    readonly city: string;
    readonly country: string;
    readonly postalCode: string;
};

type OrderItem = {
    readonly productId: ProductId;
    readonly productName: string;
    readonly quantity: number;
    readonly unitPrice: Money;
    readonly lineTotal: Money;
};

// === DU State Machine (Ch20) ===
type Order =
    | { readonly status: "draft"; readonly id: OrderId; readonly customerId: CustomerId;
        readonly items: readonly OrderItem[]; readonly shippingAddress: Address | null; }
    | { readonly status: "confirmed"; readonly id: OrderId; readonly customerId: CustomerId;
        readonly items: readonly OrderItem[]; readonly shippingAddress: Address;
        readonly confirmedAt: Date; readonly total: Money; }
    | { readonly status: "paid"; readonly id: OrderId; readonly customerId: CustomerId;
        readonly items: readonly OrderItem[]; readonly shippingAddress: Address;
        readonly confirmedAt: Date; readonly paidAt: Date;
        readonly total: Money; readonly paymentId: string; }
    | { readonly status: "shipped"; readonly id: OrderId; readonly customerId: CustomerId;
        readonly items: readonly OrderItem[]; readonly shippingAddress: Address;
        readonly total: Money; readonly trackingId: string; readonly shippedAt: Date; }
    | { readonly status: "delivered"; readonly id: OrderId; readonly deliveredAt: Date; }
    | { readonly status: "cancelled"; readonly id: OrderId; readonly reason: string;
        readonly cancelledAt: Date; };

// === Domain Events ===
type OrderEvent =
    | { type: "OrderConfirmed"; orderId: OrderId; total: Money; confirmedAt: Date }
    | { type: "OrderPaid"; orderId: OrderId; paymentId: string; paidAt: Date }
    | { type: "OrderShipped"; orderId: OrderId; trackingId: string; shippedAt: Date }
    | { type: "OrderCancelled"; orderId: OrderId; reason: string; cancelledAt: Date };

// === Result type ===
type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };
const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// === Domain Rules (pure functions!) ===
const calculateLineTotal = (item: Omit<OrderItem, "lineTotal">): OrderItem => ({
    ...item,
    lineTotal: Money(item.quantity * item.unitPrice),
});

const calculateOrderTotal = (items: readonly OrderItem[]): Money =>
    Money(items.reduce((sum, item) => sum + item.lineTotal, 0));

// Tests
const item = calculateLineTotal({
    productId: ProductId("P1"), productName: "Laptop", quantity: 2, unitPrice: Money(20000000),
});
assert.strictEqual(item.lineTotal, 40000000);

const total = calculateOrderTotal([item, calculateLineTotal({
    productId: ProductId("P2"), productName: "Mouse", quantity: 1, unitPrice: Money(500000),
})]);
assert.strictEqual(total, 40500000);

console.log("Domain types OK ✅");
```

---

Nhìn lại types vừa rồi: `Order` là discriminated union với 6 states — `draft`, `confirmed`, `paid`, `shipped`, `delivered`, `cancelled`. Mỗi state chứa **đúng data cần thiết** cho state đó. `DraftOrder` có `shippingAddress: Address | null` (chưa bắt buộc). `ConfirmedOrder` có `shippingAddress: Address` (required — không null). `PaidOrder` thêm `paymentId` và `paidAt`.

Đây là "Make Illegal States Unrepresentable" (Ch13) ở mức production. Bạn không thể tạo `PaidOrder` thiếu `paymentId` — type system ngăn cản. Bạn không thể ship order chưa paid — vì `shipOrder` chỉ nhận `PaidOrder`, không nhận `DraftOrder`.

Đối chiếu với approach truyền thống: 1 class `Order` với boolean flags `isPaid`, `isShipped`, `isCancelled`. 2³ = 8 tổ hợp, 3 vô nghĩa (shipped nhưng chưa paid?). Runtime validate. Quên validate = bug.

Đây là lý do DDD + FP mạnh hơn OOP cho domain modeling: types encode business rules, compiler enforce chúng.

## 36.3 — State Transitions (Ch20, Ch21)

```typescript
// filename: src/capstone/domain/transitions.ts
import assert from "node:assert/strict";

// Types (from 36.2 — simplified for readability)
type OrderId = string & { __brand: "OrderId" };
type CustomerId = string & { __brand: "CustomerId" };
type ProductId = string & { __brand: "ProductId" };
type Money = number & { __brand: "Money" };
const Money = (n: number): Money => n as Money;
const OrderId = (s: string): OrderId => s as OrderId;

type Address = { street: string; city: string; country: string; postalCode: string };
type OrderItem = { productId: ProductId; productName: string; quantity: number; unitPrice: Money; lineTotal: Money };

type DraftOrder = {
    status: "draft"; id: OrderId; customerId: CustomerId;
    items: readonly OrderItem[]; shippingAddress: Address | null;
};

type ConfirmedOrder = {
    status: "confirmed"; id: OrderId; customerId: CustomerId;
    items: readonly OrderItem[]; shippingAddress: Address;
    confirmedAt: Date; total: Money;
};

type PaidOrder = {
    status: "paid"; id: OrderId; customerId: CustomerId;
    items: readonly OrderItem[]; shippingAddress: Address;
    confirmedAt: Date; paidAt: Date; total: Money; paymentId: string;
};

type CancelledOrder = { status: "cancelled"; id: OrderId; reason: string; cancelledAt: Date };

type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };
const ok = <T>(v: T): Result<T, never> => ({ tag: "ok", value: v });
const err = <E>(e: E): Result<never, E> => ({ tag: "err", error: e });

// === Transition: Draft → Confirmed ===
type ConfirmError = "empty_items" | "missing_address" | "invalid_total";

const confirmOrder = (draft: DraftOrder, now: Date): Result<ConfirmedOrder, ConfirmError> => {
    if (draft.items.length === 0) return err("empty_items");
    if (!draft.shippingAddress) return err("missing_address");

    const total = Money(draft.items.reduce((s, i) => s + i.lineTotal, 0));
    if (total <= 0) return err("invalid_total");

    return ok({
        status: "confirmed",
        id: draft.id,
        customerId: draft.customerId,
        items: draft.items,
        shippingAddress: draft.shippingAddress,
        confirmedAt: now,
        total,
    });
};

// === Transition: Confirmed → Paid ===
type PayError = "payment_failed";

const markAsPaid = (
    order: ConfirmedOrder,
    paymentId: string,
    now: Date,
): Result<PaidOrder, PayError> =>
    ok({
        ...order,
        status: "paid",
        paidAt: now,
        paymentId,
    });

// === Transition: Draft/Confirmed → Cancelled ===
type CancelError = "already_paid";

const cancelOrder = (
    order: DraftOrder | ConfirmedOrder,
    reason: string,
    now: Date,
): Result<CancelledOrder, CancelError> =>
    ok({
        status: "cancelled",
        id: order.id,
        reason,
        cancelledAt: now,
    });

// === Tests ===
const now = new Date("2024-06-15T10:00:00Z");
const address: Address = { street: "123 Lê Lợi", city: "HCMC", country: "VN", postalCode: "70000" };

const draft: DraftOrder = {
    status: "draft",
    id: OrderId("ORD-1"),
    customerId: "C-1" as CustomerId,
    items: [{ productId: "P1" as ProductId, productName: "Laptop", quantity: 1, unitPrice: Money(20000000), lineTotal: Money(20000000) }],
    shippingAddress: address,
};

// Draft → Confirmed
const confirmed = confirmOrder(draft, now);
assert.strictEqual(confirmed.tag, "ok");
if (confirmed.tag === "ok") {
    assert.strictEqual(confirmed.value.status, "confirmed");
    assert.strictEqual(confirmed.value.total, 20000000);
}

// Empty items → error
const emptyDraft: DraftOrder = { ...draft, items: [] };
assert.strictEqual(confirmOrder(emptyDraft, now).tag, "err");

// No address → error
const noAddr: DraftOrder = { ...draft, shippingAddress: null };
assert.strictEqual(confirmOrder(noAddr, now).tag, "err");

// Confirmed → Paid
if (confirmed.tag === "ok") {
    const paid = markAsPaid(confirmed.value, "PAY-001", now);
    assert.strictEqual(paid.tag, "ok");
    if (paid.tag === "ok") {
        assert.strictEqual(paid.value.status, "paid");
        assert.strictEqual(paid.value.paymentId, "PAY-001");
    }
}

console.log("State transitions OK ✅");
```

---

Mỗi transition function là **pure**: `confirmOrder(draft, now)` nhận DraftOrder, trả `Result<ConfirmedOrder, ConfirmError>`. Không database, không side-effect. Bạn test nó bằng plain objects:

`confirmOrder({ ...draft, items: [] }, now)` → `err("empty_items")`. Done.

Transition functions encode **business rules as code**: "chỉ confirm từ draft", "phải có address", "total > 0". Không có business logic ẩn trong comments hay documentation — tất cả trong types và functions.

WorkflowError union type `{ step: "find", error: "not_found" } | { step: "validate", error: "not_draft" }` cho caller biết **chính xác** lỗi ở step nào. API handler map mỗi step error thành HTTP status code phù hợp: "not_found" → 404, "validation" → 422, "payment" → 402.

## 36.4 — Workflow Pipeline (Ch21, Ch22)

```typescript
// filename: src/capstone/application/confirm_workflow.ts
import assert from "node:assert/strict";

// Types (simplified)
type OrderId = string & { __brand: "OrderId" };
type Money = number & { __brand: "Money" };
const Money = (n: number): Money => n as Money;

type Order = {
    id: OrderId; status: string;
    items: readonly { lineTotal: Money }[];
    total?: Money;
};

type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };
const ok = <T>(v: T): Result<T, never> => ({ tag: "ok", value: v });
const err = <E>(e: E): Result<never, E> => ({ tag: "err", error: e });

// Ports
type OrderRepo = {
    findById: (id: OrderId) => Promise<Order | null>;
    save: (order: Order) => Promise<void>;
};

type PaymentGateway = {
    charge: (orderId: OrderId, amount: Money) => Promise<Result<string, string>>;
};

type NotificationService = {
    sendConfirmation: (orderId: OrderId, total: Money) => Promise<void>;
};

type WorkflowDeps = {
    orderRepo: OrderRepo;
    payment: PaymentGateway;
    notifications: NotificationService;
    now: () => Date;
};

// Workflow error = union of all step errors
type WorkflowError =
    | { step: "find"; error: "not_found" }
    | { step: "validate"; error: "not_draft" | "empty_items" }
    | { step: "payment"; error: string }
    | { step: "save"; error: string };

// === Workflow: Confirm + Pay Order ===
const confirmAndPayOrder = async (
    deps: WorkflowDeps,
    orderId: OrderId,
): Promise<Result<Order, WorkflowError>> => {
    // Step 1: Find
    const order = await deps.orderRepo.findById(orderId);
    if (!order) return err({ step: "find", error: "not_found" });

    // Step 2: Validate
    if (order.status !== "draft") return err({ step: "validate", error: "not_draft" });
    if (order.items.length === 0) return err({ step: "validate", error: "empty_items" });

    // Step 3: Calculate (pure)
    const total = Money(order.items.reduce((s, i) => s + i.lineTotal, 0));

    // Step 4: Charge
    const payResult = await deps.payment.charge(orderId, total);
    if (payResult.tag === "err") return err({ step: "payment", error: payResult.error });

    // Step 5: Update
    const confirmed: Order = { ...order, status: "confirmed", total };
    await deps.orderRepo.save(confirmed);

    // Step 6: Notify
    await deps.notifications.sendConfirmation(orderId, total);

    return ok(confirmed);
};

// === Test with fakes (Ch24, Ch31) ===
const run = async () => {
    const saved: Order[] = [];
    const notifications: string[] = [];

    const deps: WorkflowDeps = {
        orderRepo: {
            findById: async (id) => id === ("ORD-1" as OrderId)
                ? { id: "ORD-1" as OrderId, status: "draft", items: [{ lineTotal: Money(20000000) }] }
                : null,
            save: async (order) => { saved.push(order); },
        },
        payment: {
            charge: async (_id, _amount) => ok("PAY-001"),
        },
        notifications: {
            sendConfirmation: async (id, total) => { notifications.push(`${id}: ${total}`); },
        },
        now: () => new Date("2024-06-15"),
    };

    // Happy path
    const result = await confirmAndPayOrder(deps, "ORD-1" as OrderId);
    assert.strictEqual(result.tag, "ok");
    assert.strictEqual(saved.length, 1);
    assert.strictEqual(notifications.length, 1);

    // Not found
    const notFound = await confirmAndPayOrder(deps, "ORD-999" as OrderId);
    assert.strictEqual(notFound.tag, "err");
    if (notFound.tag === "err") assert.strictEqual(notFound.error.step, "find");

    console.log("Workflow pipeline OK ✅");
};

run();
```

---

## ✅ Checkpoint 36.1-36.4

> Đến đây bạn phải hiểu cách TẤT CẢ patterns kết hợp:
> 1. **Branded types** (Ch14) → OrderId, Money, Email
> 2. **DU state machine** (Ch20) → Order states with valid transitions ONLY
> 3. **Workflows as pipelines** (Ch21) → find → validate → calculate → charge → save → notify
> 4. **Typed errors** (Ch22) → WorkflowError = union of step-specific errors
> 5. **Repository + FP DI** (Ch24) → OrderRepo interface, fake implementations
> 6. **TDD** (Ch31) → test transitions + workflow with plain objects
>
> **Kết luận**: Mọi concept riêng lẻ kết hợp thành architecture hoàn chỉnh.

---

## 🏋️ Bài tập cuối cùng

**Bài Capstone** (30 phút): Extend the Order System

```typescript
// Add these features using ALL patterns learned:
// 1. "Add discount code" to draft order (branded type: DiscountCode)
// 2. Validate discount (check if expired, check min order amount)
// 3. Apply discount in total calculation
// 4. New workflow: applyDiscount(deps, orderId, discountCode)
// 5. Test with TDD: happy path + expired code + min amount not met
```

<details><summary>✅ Hints</summary>

```typescript
// Branded type: type DiscountCode = string & { __brand: "DiscountCode" }
// DiscountRepo port: findByCode(code) → Promise<Discount | null>
// Discount type = { code: DiscountCode; percentage: number; minOrderAmount: Money; expiresAt: Date }
// Validation errors: "code_not_found" | "expired" | "min_amount_not_met"
// Apply: total * (1 - percentage/100)
```

</details>

---

## Tóm tắt — Part VI & Book Journey

Chương này gom TẤT CẢ kiến thức thành một project hoàn chỉnh:

- ✅ **Domain types**: branded types + DU state machines + pure functions
- ✅ **Workflows**: pipelines with typed errors at each step
- ✅ **Architecture**: Hexagonal + Functional Core / Imperative Shell
- ✅ **Infrastructure**: Repository + FP DI + fake implementations
- ✅ **Testing**: TDD with plain object fakes. No mocks

---

## 🎉 Part VI Complete!

Bạn đã hoàn thành Part VI: Testing & Full-Stack. Từ TDD (Ch31) qua Property-Based Testing (Ch32), Architecture Patterns (Ch33), Backend (Ch34), Frontend (Ch35), đến Capstone (Ch36). Đây là vòng tròn hoàn chỉnh — lý thuyết thành thực hành, patterns thành production code.

## Tiếp theo

→ Part VII: **Production Engineering** — Database, Security, Distributed Systems, System Design.
