# Chapter 17 — CQRS & Event Sourcing

> **Bạn sẽ học được**:
> - CQRS — Command Query Responsibility Segregation
> - Tại sao tách read/write có lợi
> - Event Sourcing — lưu events thay vì state
> - `reduce` rebuild state từ events — giống Command pattern (Ch16)!
> - Projections — tạo read models từ event stream
> - Temporal queries — "trạng thái tại thời điểm X?"
>
> **Yêu cầu trước**: Chapter 11 (immutability), Chapter 13 (ADTs, DU), Chapter 16 (Command pattern).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Hiểu kiến trúc event-driven — FP + immutability = event sourcing tự nhiên.

---

Bạn có bao giờ mở sổ ngân hàng cũ không?

Đó không phải là số dư — mà là danh sách giao dịch: "ngày 1/1 nộp 5 triệu", "ngày 5/1 rút 1 triệu", "ngày 10/1 nộp 3 triệu". Số dư hiện tại? Cộng tất cả lại. Số dư ngày 5/1? Cộng đến ngày 5/1. Muốn biết tại sao tháng 3 âm? Lọc giao dịch tháng 3. Mọi câu hỏi đều trả lời được — vì bạn có **toàn bộ lịch sử**, không chỉ trạng thái cuối.

Đây chính là **Event Sourcing**: thay vì lưu "balance = 7 triệu" (state), bạn lưu danh sách events đã xảy ra. State = `events.reduce(apply, initialState)` — giống hệt `commands.reduce(execute, initialState)` ở Command pattern (Ch16). Và CQRS bổ sung thêm: tách "ghi sổ" (write — validate, enforce rules) khỏi "đọc sổ" (read — queries, reports). Cuốn sổ kế toán gốc chỉ cho phép ghi thêm, không bao giờ tẩy — append only, immutable. Báo cáo thuế, báo cáo doanh thu, bảng cân đối — chỉ là **projections** khác nhau từ cùng một cuốn sổ.

---

## 17.1 — CQRS: Tách Read và Write

### Khi một cuốn sổ phải phục vụ hai mục đích

Trong mô hình CRUD truyền thống, bạn có MỘT model phục vụ CẢ đọc lẫn ghi. Giống một cuốn sổ vừa dùng ghi giao dịch (cần chi tiết, validation, business rules) vừa dùng tra cứu nhanh (chỉ cần tên, tổng, trạng thái). Kết quả: model quá phức tạp cho queries, quá đơn giản cho business logic.

CQRS tách hai: **Write model** (commands, domain logic, validation) và **Read model** (queries, denormalized, tối ưu tốc độ). Write side phức tạp — kiểm tra quyền, validate transitions, enforce invariants. Read side đơn giản — pre-computed, cached, flat. Scale independently: reads 100x writes? Cache read model. Complex business rules? Enrich write model. Hai bên không ảnh hưởng nhau.

```typescript
// filename: src/crud_problem.ts

// ❌ Traditional CRUD — 1 model cho cả read VÀ write
type Order = {
    id: string;
    customerId: string;
    items: Array<{ productId: string; quantity: number; price: number }>;
    status: "draft" | "confirmed" | "shipped" | "delivered";
    total: number;
    createdAt: Date;
    updatedAt: Date;
};

// Vấn đề:
// 1. READ cần gì? → name, total, status (ít fields)
// 2. WRITE cần gì? → validation, business rules, state transitions
// 3. 1 model phục vụ CẢ HAI → quá phức tạp cho read, thiếu chi tiết cho write
// 4. Scale: read 100x nhiều hơn write → bottleneck!
```

### CQRS: 2 models riêng biệt

```typescript
// filename: src/cqrs_basics.ts
import assert from "node:assert/strict";

// --- WRITE side: Commands + Domain logic ---

type OrderCommand =
    | { readonly tag: "create"; readonly customerId: string; readonly items: readonly OrderItem[] }
    | { readonly tag: "confirm"; readonly orderId: string }
    | { readonly tag: "ship"; readonly orderId: string; readonly trackingCode: string }
    | { readonly tag: "cancel"; readonly orderId: string; readonly reason: string };

type OrderItem = {
    readonly productId: string;
    readonly quantity: number;
    readonly unitPrice: number;
};

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// Write model — đầy đủ business logic
type OrderAggregate = {
    readonly id: string;
    readonly customerId: string;
    readonly items: readonly OrderItem[];
    readonly status: "draft" | "confirmed" | "shipped" | "cancelled";
};

const createOrder = (
    id: string,
    cmd: OrderCommand & { readonly tag: "create" }
): Result<OrderAggregate, string> => {
    if (cmd.items.length === 0) return err("Order phải có ít nhất 1 item");
    return ok({
        id,
        customerId: cmd.customerId,
        items: cmd.items,
        status: "draft",
    });
};

const confirmOrder = (order: OrderAggregate): Result<OrderAggregate, string> => {
    if (order.status !== "draft") return err(`Không thể confirm order ${order.status}`);
    return ok({ ...order, status: "confirmed" });
};

// --- READ side: Denormalized, optimized for queries ---

type OrderSummary = {
    readonly id: string;
    readonly customerName: string;
    readonly itemCount: number;
    readonly total: number;
    readonly status: string;
};

type OrderDetail = {
    readonly id: string;
    readonly customerName: string;
    readonly items: readonly {
        readonly productName: string;
        readonly quantity: number;
        readonly lineTotal: number;
    }[];
    readonly total: number;
    readonly status: string;
};

// Read queries — simple, fast, no business logic
const getOrderSummaries = (summaries: readonly OrderSummary[]): readonly OrderSummary[] =>
    summaries;

const getOrdersByStatus = (
    summaries: readonly OrderSummary[],
    status: string
): readonly OrderSummary[] =>
    summaries.filter(s => s.status === status);

// Test
const order = createOrder("ORD-1", {
    tag: "create",
    customerId: "C1",
    items: [{ productId: "P1", quantity: 2, unitPrice: 100000 }],
});
assert.strictEqual(order.tag, "ok");

const emptyOrder = createOrder("ORD-2", {
    tag: "create",
    customerId: "C2",
    items: [],
});
assert.strictEqual(emptyOrder.tag, "err");

console.log("CQRS basics OK ✅");
```

> **💡 CQRS = "Đọc và ghi là hai concerns KHÁC nhau"**:
> - **Write** = validate, enforce business rules, state transitions → phức tạp
> - **Read** = query data, denormalized, tối ưu tốc độ → đơn giản
> - Tách ra → mỗi bên optimize independently

---

## ✅ Checkpoint 17.1

> Đến đây bạn phải hiểu:
> 1. **CRUD** = 1 model cho read+write → compromise cho cả hai
> 2. **CQRS** = tách: Write model (commands, validation) + Read model (queries, denormalized)
> 3. Write model phức tạp (business rules). Read model đơn giản (fast queries)
> 4. Scale independently: read cache, write event-driven
>
> **Test nhanh**: Dashboard hiện "tổng đơn hàng hôm nay" — dùng write hay read model?
> <details><summary>Đáp án</summary>**Read model**! Dashboard = query, không cần business logic. Read model denormalized, có thể pre-computed. Write model chỉ cho mutations.</details>

---

## 17.2 — Event Sourcing: Events thay vì State

### Cuốn sổ kế toán — ghi transation, không ghi số dư

Quay lại cuốn sổ ngân hàng: kế toán viên KHÔNG bao giờ tẩy dòng cũ — họ ghi thêm dòng mới. Mỗi bút toán là một **event đã xảy ra** (past tense): "account_opened", "money_deposited", "money_withdrawn". Số dư hiện tại? `events.reduce(apply, initialState)`. Số dư ngày hôm qua? `events.filter(e => e.timestamp <= yesterday).reduce(apply, initialState)`.

Điểm then chốt: events dùng **thì quá khứ** — "money_deposited", không phải "deposit_money". Vì event là sự kiện ĐÃ XẢY RA, không thể undo, không thể sửa. Đây là nguyên tắc **immutable append-only log** — giống cuốn sổ kế toán vàng mà kiểm toán viên yêu.

```typescript
// filename: src/event_sourcing.ts
import assert from "node:assert/strict";

// --- Events = sự kiện ĐÃ xảy ra (past tense!) ---
type BankEvent =
    | { readonly tag: "account_opened"; readonly accountId: string; readonly name: string; readonly timestamp: number }
    | { readonly tag: "money_deposited"; readonly accountId: string; readonly amount: number; readonly timestamp: number }
    | { readonly tag: "money_withdrawn"; readonly accountId: string; readonly amount: number; readonly timestamp: number }
    | { readonly tag: "account_closed"; readonly accountId: string; readonly reason: string; readonly timestamp: number };

// --- State = rebuild từ events ---
type AccountState = {
    readonly id: string;
    readonly name: string;
    readonly balance: number;
    readonly status: "active" | "closed";
    readonly transactionCount: number;
};

const initialState: AccountState = {
    id: "",
    name: "",
    balance: 0,
    status: "active",
    transactionCount: 0,
};

// apply = (state, event) → newState — GIỐNG reduce!
const apply = (state: AccountState, event: BankEvent): AccountState => {
    switch (event.tag) {
        case "account_opened":
            return { ...state, id: event.accountId, name: event.name };
        case "money_deposited":
            return {
                ...state,
                balance: state.balance + event.amount,
                transactionCount: state.transactionCount + 1,
            };
        case "money_withdrawn":
            return {
                ...state,
                balance: state.balance - event.amount,
                transactionCount: state.transactionCount + 1,
            };
        case "account_closed":
            return { ...state, status: "closed" };
    }
};

// --- Event stream ---
const events: readonly BankEvent[] = [
    { tag: "account_opened", accountId: "ACC-1", name: "An", timestamp: 1000 },
    { tag: "money_deposited", accountId: "ACC-1", amount: 5000000, timestamp: 2000 },
    { tag: "money_deposited", accountId: "ACC-1", amount: 3000000, timestamp: 3000 },
    { tag: "money_withdrawn", accountId: "ACC-1", amount: 1000000, timestamp: 4000 },
    { tag: "money_deposited", accountId: "ACC-1", amount: 2000000, timestamp: 5000 },
];

// Rebuild state = reduce!
const currentState = events.reduce(apply, initialState);

assert.strictEqual(currentState.balance, 9000000);
// 0 + 5M + 3M - 1M + 2M = 9M
assert.strictEqual(currentState.transactionCount, 4);
assert.strictEqual(currentState.name, "An");

console.log("Event sourcing OK ✅");
```

Bạn có nhận ra không? `events.reduce(apply, initialState)` GIỐNG HỆT `commands.reduce(execute, initialState)` ở Command pattern Ch16. Events = commands ĐÃ THỰC HIỆN. Event log = command history. FP concepts kết nối — và đây không phải trùng hợp: Event Sourcing chính là Command pattern được nâng lên tầm kiến trúc hệ thống.

> **💡 Event Sourcing = Command pattern (Ch16) cho data**: `events.reduce(apply, initialState)` = giống hệt `commands.reduce(execute, initialState)`. Events = commands ĐÃ THỰC HIỆN. Event log = command history. FP concepts KẾT NỐI!

---

## ✅ Checkpoint 17.2

> Đến đây bạn phải hiểu:
> 1. **Event** = sự kiện past tense: "money_deposited", không phải "deposit_money"
> 2. **State** = `events.reduce(apply, initialState)` — rebuild từ event stream
> 3. **Giống Command pattern**: events = executed commands. Apply = execute
> 4. **Events immutable**: không sửa, không xóa. Append only
>
> **Test nhanh**: `balance = 9M`. Thêm event `money_withdrawn(2M)`. Balance mới?
> <details><summary>Đáp án</summary>`7M`. `apply(state, { tag: "money_withdrawn", amount: 2000000 })` = `9M - 2M = 7M`.</details>

---

## 17.3 — Tại sao Event Sourcing?

### Bốn siêu năng lực của cuốn sổ kế toán

Tại sao giữ toàn bộ lịch sử thay vì chỉ trạng thái cuối? Vì lịch sử cho bạn 4 khả năng mà CRUD không có: **temporal queries** (trạng thái tại thời điểm X), **audit trail** (ai làm gì khi nào), **debug replay** (tại sao trạng thái hiện tại như thế), và **what-if analysis** (nếu thay đổi quy tắc, kết quả ra sao).

```typescript
// filename: src/es_advantages.ts
import assert from "node:assert/strict";

type BankEvent =
    | { readonly tag: "account_opened"; readonly accountId: string; readonly name: string; readonly timestamp: number }
    | { readonly tag: "money_deposited"; readonly accountId: string; readonly amount: number; readonly timestamp: number }
    | { readonly tag: "money_withdrawn"; readonly accountId: string; readonly amount: number; readonly timestamp: number };

type AccountState = {
    readonly id: string;
    readonly name: string;
    readonly balance: number;
    readonly transactionCount: number;
};

const initialState: AccountState = { id: "", name: "", balance: 0, transactionCount: 0 };

const apply = (state: AccountState, event: BankEvent): AccountState => {
    switch (event.tag) {
        case "account_opened":
            return { ...state, id: event.accountId, name: event.name };
        case "money_deposited":
            return { ...state, balance: state.balance + event.amount, transactionCount: state.transactionCount + 1 };
        case "money_withdrawn":
            return { ...state, balance: state.balance - event.amount, transactionCount: state.transactionCount + 1 };
    }
};

const events: readonly BankEvent[] = [
    { tag: "account_opened", accountId: "ACC-1", name: "An", timestamp: 1000 },
    { tag: "money_deposited", accountId: "ACC-1", amount: 5000000, timestamp: 2000 },
    { tag: "money_deposited", accountId: "ACC-1", amount: 3000000, timestamp: 3000 },
    { tag: "money_withdrawn", accountId: "ACC-1", amount: 1000000, timestamp: 4000 },
];

// --- Advantage 1: TEMPORAL QUERIES ---
// "Số dư tại thời điểm timestamp 3000?"
const stateAtTime = (events: readonly BankEvent[], timestamp: number): AccountState =>
    events
        .filter(e => e.timestamp <= timestamp)
        .reduce(apply, initialState);

assert.strictEqual(stateAtTime(events, 2000).balance, 5000000);   // chỉ deposit 5M
assert.strictEqual(stateAtTime(events, 3000).balance, 8000000);   // 5M + 3M
assert.strictEqual(stateAtTime(events, 4000).balance, 7000000);   // 5M + 3M - 1M

// --- Advantage 2: AUDIT TRAIL ---
// "Liệt kê TẤT CẢ giao dịch > 2M?"
const largeTransactions = events.filter(e =>
    (e.tag === "money_deposited" || e.tag === "money_withdrawn") && e.amount > 2000000
);
assert.strictEqual(largeTransactions.length, 2);  // 5M deposit + 3M deposit

// --- Advantage 3: DEBUG ---
// "Tại sao balance = 7M?" → Replay events step by step
const trace = events.map((e, i) => ({
    step: i + 1,
    event: e.tag,
    state: events.slice(0, i + 1).reduce(apply, initialState),
}));

// trace shows EXACTLY how balance changed over time

// --- Advantage 4: REPLAY with new logic ---
// "Nếu phí rút tiền 2%?" — replay with different apply function
const applyWithFee = (state: AccountState, event: BankEvent): AccountState => {
    switch (event.tag) {
        case "account_opened":
            return apply(state, event);
        case "money_deposited":
            return apply(state, event);
        case "money_withdrawn":
            // Trừ thêm 2% phí
            const fee = Math.round(event.amount * 0.02);
            return {
                ...state,
                balance: state.balance - event.amount - fee,
                transactionCount: state.transactionCount + 1,
            };
    }
};

const withFeeState = events.reduce(applyWithFee, initialState);
assert.strictEqual(withFeeState.balance, 6980000);
// 5M + 3M - 1M - (1M × 0.02 = 20K) = 6,980,000

console.log("ES advantages OK ✅");
```

### Trade-offs

Tất nhiên cuốn sổ kế toán đầy đủ cũng có nhược điểm: nó DÀI. Event store chỉ tăng, không bao giờ giảm. Rebuild state từ triệu events thì chậm (giải pháp: snapshots — section 17.6). Schema evolution phức tạp (event shapes thay đổi qua thời gian). Và read models eventually consistent — có độ trễ nhỏ giữa event xảy ra và read model cập nhật.

| Pro | Con |
|-----|-----|
| Full audit trail | Event store grows indefinitely |
| Temporal queries | Rebuild state = expensive cho nhiều events |
| Debug: replay step-by-step | Schema evolution phức tạp |
| What-if analysis (new logic) | Eventually consistent (read lag) |
| Natural fit cho FP | Learning curve |

---

## ✅ Checkpoint 17.3

> Đến đây bạn phải hiểu:
> 1. **Temporal queries**: filter events by timestamp → state tại bất kỳ thời điểm
> 2. **Audit trail**: events = complete history. Không mất thông tin
> 3. **Replay**: thay `apply` function → "what if?" analysis
> 4. **Trade-offs**: growth, rebuild cost, schema evolution
>
> **Test nhanh**: Event Sourcing WITHOUT CQRS — query "top 10 khách hàng theo tổng chi" có nhanh không?
> <details><summary>Đáp án</summary>**CHẬM!** Phải rebuild state TẤT CẢ tài khoản rồi sort. ES cần CQRS: read model (projection) pre-computed, query trực tiếp.</details>

---

## 17.4 — Projections: Read Models từ Events

### Nhiều báo cáo từ một cuốn sổ

Từ cùng một cuốn sổ kế toán, bạn tạo được: báo cáo thuế (tổng thu/chi theo tháng), bảng cân đối (assets vs liabilities), báo cáo doanh thu (revenue by product), và dashboard (summary numbers). Mỗi báo cáo nhìn cùng events nhưng **từ góc độ khác** — đó là **projection**.

Projection = `events.reduce(projectFn, initialState)`. Mỗi projection function chỉ "quan tâm" một số event types, bỏ qua phần còn lại. Thêm projection mới? Viết 1 function, replay existing events — **không sửa** write side, không sửa event schema.

```typescript
// filename: src/projections.ts
import assert from "node:assert/strict";

// Events cho e-commerce
type OrderEvent =
    | { readonly tag: "order_created"; readonly orderId: string; readonly customerId: string; readonly total: number; readonly timestamp: number }
    | { readonly tag: "order_confirmed"; readonly orderId: string; readonly timestamp: number }
    | { readonly tag: "order_shipped"; readonly orderId: string; readonly trackingCode: string; readonly timestamp: number }
    | { readonly tag: "order_delivered"; readonly orderId: string; readonly timestamp: number }
    | { readonly tag: "order_cancelled"; readonly orderId: string; readonly reason: string; readonly timestamp: number };

// --- Projection 1: Dashboard summary ---
type DashboardProjection = {
    readonly totalOrders: number;
    readonly totalRevenue: number;
    readonly cancelledOrders: number;
    readonly deliveredOrders: number;
};

const dashboardInitial: DashboardProjection = {
    totalOrders: 0,
    totalRevenue: 0,
    cancelledOrders: 0,
    deliveredOrders: 0,
};

const projectDashboard = (state: DashboardProjection, event: OrderEvent): DashboardProjection => {
    switch (event.tag) {
        case "order_created":
            return { ...state, totalOrders: state.totalOrders + 1, totalRevenue: state.totalRevenue + event.total };
        case "order_cancelled":
            return { ...state, cancelledOrders: state.cancelledOrders + 1 };
        case "order_delivered":
            return { ...state, deliveredOrders: state.deliveredOrders + 1 };
        default:
            return state;  // confirmed, shipped — không ảnh hưởng dashboard
    }
};

// --- Projection 2: Customer spending ---
type CustomerSpending = ReadonlyMap<string, number>;

const projectSpending = (state: CustomerSpending, event: OrderEvent): CustomerSpending => {
    switch (event.tag) {
        case "order_created": {
            const current = state.get(event.customerId) ?? 0;
            return new Map([...state, [event.customerId, current + event.total]]);
        }
        case "order_cancelled": {
            // Note: cần lookup orderId → customerId để trừ. Simplified version
            return state;
        }
        default:
            return state;
    }
};

// --- Projection 3: Order status tracker ---
type OrderStatus = ReadonlyMap<string, string>;

const projectStatus = (state: OrderStatus, event: OrderEvent): OrderStatus => {
    switch (event.tag) {
        case "order_created":
            return new Map([...state, [event.orderId, "created"]]);
        case "order_confirmed":
            return new Map([...state, [event.orderId, "confirmed"]]);
        case "order_shipped":
            return new Map([...state, [event.orderId, "shipped"]]);
        case "order_delivered":
            return new Map([...state, [event.orderId, "delivered"]]);
        case "order_cancelled":
            return new Map([...state, [event.orderId, "cancelled"]]);
    }
};

// --- Test với event stream ---
const events: readonly OrderEvent[] = [
    { tag: "order_created", orderId: "O1", customerId: "C1", total: 5000000, timestamp: 1000 },
    { tag: "order_created", orderId: "O2", customerId: "C2", total: 3000000, timestamp: 2000 },
    { tag: "order_confirmed", orderId: "O1", timestamp: 3000 },
    { tag: "order_created", orderId: "O3", customerId: "C1", total: 2000000, timestamp: 4000 },
    { tag: "order_shipped", orderId: "O1", trackingCode: "TRK-001", timestamp: 5000 },
    { tag: "order_cancelled", orderId: "O2", reason: "customer request", timestamp: 6000 },
    { tag: "order_delivered", orderId: "O1", timestamp: 7000 },
];

// SAME events → DIFFERENT projections!
const dashboard = events.reduce(projectDashboard, dashboardInitial);
assert.strictEqual(dashboard.totalOrders, 3);
assert.strictEqual(dashboard.totalRevenue, 10000000);  // 5M + 3M + 2M
assert.strictEqual(dashboard.cancelledOrders, 1);
assert.strictEqual(dashboard.deliveredOrders, 1);

const spending = events.reduce(projectSpending, new Map() as CustomerSpending);
assert.strictEqual(spending.get("C1"), 7000000);  // 5M + 2M
assert.strictEqual(spending.get("C2"), 3000000);

const statuses = events.reduce(projectStatus, new Map() as OrderStatus);
assert.strictEqual(statuses.get("O1"), "delivered");
assert.strictEqual(statuses.get("O2"), "cancelled");
assert.strictEqual(statuses.get("O3"), "created");

console.log("Projections OK ✅");
```

> **💡 Projections = "Nhiều views từ 1 nguồn sự thật"**: Cùng event stream → dashboard, spending report, status tracker. CQRS read models = projections. Thêm projection mới = thêm 1 `reduce` function. Events không đổi!

---

## ✅ Checkpoint 17.4

> Đến đây bạn phải hiểu:
> 1. **Projection** = `events.reduce(projectFn, initialState)` — 1 view từ events
> 2. **Multiple projections**: same events → different read models
> 3. **Add new projection** = viết 1 function, replay existing events
> 4. **CQRS read model** = projection, denormalized, optimized for queries
>
> **Test nhanh**: Boss yêu cầu "report: top 5 sản phẩm bán chạy nhất". Cần làm gì?
> <details><summary>Đáp án</summary>Viết **1 projection mới**: `projectProductSales(state, event)`. Replay existing events. Không sửa gì ở write side. Không sửa event schema. Projection = read-only view.</details>

---

## 17.5 — Putting It Together: CQRS + Event Sourcing

### Hệ thống quản lý kho: từ command đến projection

Đây là ví dụ tổng hợp — kết nối mọi thứ: Commands validate business rules → emit Events → Apply events cập nhật write model → Projections tạo read models. Flow hoàn chỉnh cho inventory management: thêm sản phẩm, bán hàng (kiểm tra tồn kho), nhập thêm, thay đổi giá. Mỗi bước là pure function — không mutation, không side effects.

Chú ý `handleCommand` return `Result<readonly InventoryEvent[], string>` — TRẢ VỀ EVENTS, không mutate trực tiếp. Events chỉ được emit khi validation pass. Nếu fail (bán nhiều hơn tồn kho), KHÔNG có event nào — state unchanged, audit log clean.

```typescript
// filename: src/cqrs_es.ts
import assert from "node:assert/strict";

// === DOMAIN EVENTS ===
type InventoryEvent =
    | { readonly tag: "product_added"; readonly productId: string; readonly name: string; readonly price: number; readonly initialStock: number; readonly timestamp: number }
    | { readonly tag: "stock_received"; readonly productId: string; readonly quantity: number; readonly timestamp: number }
    | { readonly tag: "stock_sold"; readonly productId: string; readonly quantity: number; readonly orderId: string; readonly timestamp: number }
    | { readonly tag: "price_changed"; readonly productId: string; readonly oldPrice: number; readonly newPrice: number; readonly timestamp: number }
    | { readonly tag: "product_discontinued"; readonly productId: string; readonly timestamp: number };

// === WRITE SIDE: Commands + Validation ===
type InventoryCommand =
    | { readonly tag: "add_product"; readonly productId: string; readonly name: string; readonly price: number; readonly stock: number }
    | { readonly tag: "sell"; readonly productId: string; readonly quantity: number; readonly orderId: string }
    | { readonly tag: "restock"; readonly productId: string; readonly quantity: number }
    | { readonly tag: "change_price"; readonly productId: string; readonly newPrice: number };

type ProductState = {
    readonly id: string;
    readonly name: string;
    readonly price: number;
    readonly stock: number;
    readonly active: boolean;
};

type InventoryState = ReadonlyMap<string, ProductState>;

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// Command → validate → events
const handleCommand = (
    state: InventoryState,
    command: InventoryCommand,
    timestamp: number
): Result<readonly InventoryEvent[], string> => {
    switch (command.tag) {
        case "add_product": {
            if (state.has(command.productId)) return err("Product already exists");
            if (command.price <= 0) return err("Price must be > 0");
            return ok([{
                tag: "product_added",
                productId: command.productId,
                name: command.name,
                price: command.price,
                initialStock: command.stock,
                timestamp,
            }]);
        }
        case "sell": {
            const product = state.get(command.productId);
            if (!product) return err("Product not found");
            if (!product.active) return err("Product discontinued");
            if (product.stock < command.quantity) return err(`Insufficient stock: ${product.stock} < ${command.quantity}`);
            return ok([{
                tag: "stock_sold",
                productId: command.productId,
                quantity: command.quantity,
                orderId: command.orderId,
                timestamp,
            }]);
        }
        case "restock": {
            const product = state.get(command.productId);
            if (!product) return err("Product not found");
            if (command.quantity <= 0) return err("Quantity must be > 0");
            return ok([{
                tag: "stock_received",
                productId: command.productId,
                quantity: command.quantity,
                timestamp,
            }]);
        }
        case "change_price": {
            const product = state.get(command.productId);
            if (!product) return err("Product not found");
            if (command.newPrice <= 0) return err("Price must be > 0");
            return ok([{
                tag: "price_changed",
                productId: command.productId,
                oldPrice: product.price,
                newPrice: command.newPrice,
                timestamp,
            }]);
        }
    }
};

// Apply event → update write model state
const applyEvent = (state: InventoryState, event: InventoryEvent): InventoryState => {
    switch (event.tag) {
        case "product_added":
            return new Map([...state, [event.productId, {
                id: event.productId,
                name: event.name,
                price: event.price,
                stock: event.initialStock,
                active: true,
            }]]);
        case "stock_received": {
            const product = state.get(event.productId)!;
            return new Map([...state, [event.productId, {
                ...product,
                stock: product.stock + event.quantity,
            }]]);
        }
        case "stock_sold": {
            const product = state.get(event.productId)!;
            return new Map([...state, [event.productId, {
                ...product,
                stock: product.stock - event.quantity,
            }]]);
        }
        case "price_changed": {
            const product = state.get(event.productId)!;
            return new Map([...state, [event.productId, {
                ...product,
                price: event.newPrice,
            }]]);
        }
        case "product_discontinued": {
            const product = state.get(event.productId)!;
            return new Map([...state, [event.productId, {
                ...product,
                active: false,
            }]]);
        }
    }
};

// === READ SIDE: Projections ===

// Projection: Low stock alerts
type LowStockAlert = {
    readonly productId: string;
    readonly name: string;
    readonly currentStock: number;
};

const projectLowStock = (
    events: readonly InventoryEvent[],
    threshold: number
): readonly LowStockAlert[] => {
    const state = events.reduce(applyEvent, new Map() as InventoryState);
    return [...state.values()]
        .filter(p => p.active && p.stock <= threshold)
        .map(p => ({ productId: p.id, name: p.name, currentStock: p.stock }));
};

// Projection: Revenue by product
type RevenueReport = ReadonlyMap<string, number>;

const projectRevenue = (events: readonly InventoryEvent[]): RevenueReport => {
    const state = events.reduce(applyEvent, new Map() as InventoryState);

    return events.reduce((revenue, event) => {
        if (event.tag !== "stock_sold") return revenue;
        // ⚠️ Simplified: dùng price HIỆN TẠI, không phải price tại thời điểm bán
        // Production: lưu price trong event stock_sold, hoặc track price history
        const product = state.get(event.productId);
        if (!product) return revenue;
        const saleRevenue = product.price * event.quantity;
        const current = revenue.get(event.productId) ?? 0;
        return new Map([...revenue, [event.productId, current + saleRevenue]]);
    }, new Map() as RevenueReport);
};

// === TEST ===
const allEvents: InventoryEvent[] = [];
let currentState: InventoryState = new Map();

// Helper: process command → emit events
const process = (command: InventoryCommand, timestamp: number): void => {
    const result = handleCommand(currentState, command, timestamp);
    if (result.tag === "ok") {
        for (const event of result.value) {
            allEvents.push(event);
            currentState = applyEvent(currentState, event);
        }
    }
};

// Scenario
process({ tag: "add_product", productId: "P1", name: "Laptop", price: 20000000, stock: 10 }, 1000);
process({ tag: "add_product", productId: "P2", name: "Mouse", price: 500000, stock: 50 }, 2000);
process({ tag: "sell", productId: "P1", quantity: 3, orderId: "O1" }, 3000);
process({ tag: "sell", productId: "P2", quantity: 45, orderId: "O2" }, 4000);
process({ tag: "restock", productId: "P1", quantity: 5 }, 5000);

// Write model: current state
assert.strictEqual(currentState.get("P1")!.stock, 12);  // 10 - 3 + 5
assert.strictEqual(currentState.get("P2")!.stock, 5);   // 50 - 45

// Read model: low stock alerts
const lowStock = projectLowStock(allEvents, 10);
assert.strictEqual(lowStock.length, 1);  // P2 has 5 stock
assert.strictEqual(lowStock[0].productId, "P2");

// Validation: sell more than stock
const oversell = handleCommand(currentState, {
    tag: "sell", productId: "P2", quantity: 100, orderId: "O4"
}, 6000);
assert.strictEqual(oversell.tag, "err");  // Insufficient stock!

console.log("CQRS + ES OK ✅");
```

---

## ✅ Checkpoint 17.5

> Đến đây bạn phải hiểu:
> 1. **Command → validate → events**: command handler returns events (không mutate trực tiếp)
> 2. **Events → state**: `reduce(applyEvent, initialState)` rebuild write model
> 3. **Events → projections**: same events, different read models (low stock, revenue)
> 4. **Flow**: Command → validate against state → emit events → apply → update read models
>
> **Test nhanh**: Command "sell 100 P2" khi stock = 5. Xảy ra gì?
> <details><summary>Đáp án</summary>`err("Insufficient stock: 5 < 100")`. Validation fails → NO events emitted. State unchanged. Event log unchanged. Safe!</details>

---

## 17.6 — Snapshots & Performance

### Khi cuốn sổ kế toán quá dày

Sau 10 năm, cuốn sổ có triệu bút toán. Muốn biết số dư hiện tại, phải cộng từ dòng 1? Không — kế toán viên ghi "Tổng kết cuối năm 2023: 50 triệu", rồi bắt đầu sổ mới từ đó. Đây là **snapshot**: "ảnh chụp" state tại event N. Rebuild chỉ cần snapshot + events SAU snapshot — từ triệu events xuống còn vài nghìn.

```typescript
// filename: src/snapshots.ts
import assert from "node:assert/strict";

// Snapshot = "ảnh chụp" state tại event N
type Snapshot<S> = {
    readonly state: S;
    readonly eventIndex: number;  // áp dụng events TỪ index này trở đi
    readonly timestamp: number;
};

type AccountState = {
    readonly balance: number;
    readonly transactionCount: number;
};

type BankEvent =
    | { readonly tag: "deposited"; readonly amount: number }
    | { readonly tag: "withdrawn"; readonly amount: number };

const apply = (state: AccountState, event: BankEvent): AccountState => {
    switch (event.tag) {
        case "deposited":
            return { balance: state.balance + event.amount, transactionCount: state.transactionCount + 1 };
        case "withdrawn":
            return { balance: state.balance - event.amount, transactionCount: state.transactionCount + 1 };
    }
};

// Rebuild from snapshot + events AFTER snapshot
const rebuildFromSnapshot = <S>(
    snapshot: Snapshot<S>,
    allEvents: readonly BankEvent[],
    applyFn: (state: S, event: BankEvent) => S
): S => {
    const remainingEvents = allEvents.slice(snapshot.eventIndex);
    return remainingEvents.reduce(applyFn, snapshot.state);
};

// Test
const allEvents: readonly BankEvent[] = [
    { tag: "deposited", amount: 1000 },
    { tag: "deposited", amount: 2000 },
    { tag: "withdrawn", amount: 500 },
    // --- Snapshot taken here (eventIndex = 3, balance = 2500) ---
    { tag: "deposited", amount: 3000 },
    { tag: "withdrawn", amount: 1000 },
];

// Full rebuild: 5 events
const fullState = allEvents.reduce(apply, { balance: 0, transactionCount: 0 });
assert.strictEqual(fullState.balance, 4500);
assert.strictEqual(fullState.transactionCount, 5);

// Snapshot rebuild: only 2 events after snapshot!
const snapshot: Snapshot<AccountState> = {
    state: { balance: 2500, transactionCount: 3 },
    eventIndex: 3,
    timestamp: Date.now(),
};

const fromSnapshot = rebuildFromSnapshot(snapshot, allEvents, apply);
assert.strictEqual(fromSnapshot.balance, 4500);  // same result!
assert.strictEqual(fromSnapshot.transactionCount, 5);

console.log("Snapshots OK ✅");
```

> **💡 Snapshots = "checkpoint saves"**: Thay vì replay 1 triệu events, snapshot tại event 999,000 → chỉ replay 1,000 events. Full rebuild khi cần (migration, debug). Game save analogy!

---

## ✅ Checkpoint 17.6

> Đến đây bạn phải hiểu:
> 1. **Snapshot** = state tại event N. Rebuild chỉ events sau N
> 2. **Performance**: 1M events → snapshot tại 999K → replay 1K. Fast!
> 3. **Full rebuild** vẫn possible — snapshot = optimization, không phải requirement
> 4. **Strategy**: snapshot mỗi N events hoặc mỗi T thời gian
>
> **Test nhanh**: Snapshot tại event 100. Event 101-110 đã xảy ra. Cần replay bao nhiêu events?
> <details><summary>Đáp án</summary>**10** events (101-110). Snapshot state + 10 events = current state. Thay vì 110 events từ đầu.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Simple event sourcing

```typescript
// Viết shopping cart event sourced:
// Events: item_added | item_removed | cart_cleared
// State: { items: readonly string[], total: number }
// apply(state, event) → newState
// Test: add("A", 100) → add("B", 200) → remove("A", 100) = { items: ["B"], total: 200 }
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type CartEvent =
    | { readonly tag: "item_added"; readonly item: string; readonly price: number }
    | { readonly tag: "item_removed"; readonly item: string; readonly price: number }
    | { readonly tag: "cart_cleared" };

type CartState = {
    readonly items: readonly string[];
    readonly total: number;
};

const initialCart: CartState = { items: [], total: 0 };

const applyCart = (state: CartState, event: CartEvent): CartState => {
    switch (event.tag) {
        case "item_added":
            return { items: [...state.items, event.item], total: state.total + event.price };
        case "item_removed":
            return {
                items: state.items.filter(i => i !== event.item),
                total: Math.max(0, state.total - event.price),
            };
        case "cart_cleared":
            return initialCart;
    }
};

const events: readonly CartEvent[] = [
    { tag: "item_added", item: "A", price: 100 },
    { tag: "item_added", item: "B", price: 200 },
    { tag: "item_removed", item: "A", price: 100 },
];

const cart = events.reduce(applyCart, initialCart);
assert.deepStrictEqual(cart.items, ["B"]);
assert.strictEqual(cart.total, 200);
```

</details>

---

**Bài 2** (10 phút): Projections

```typescript
// Từ shopping cart events ở trên, viết 2 projections:
// 1. Projection: MostPopularItems — { item: string, addCount: number }[]
// 2. Projection: CartAbandonment — carts cleared WITHOUT order_placed event
//    (simplified: count "cart_cleared" events)
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type CartEvent =
    | { readonly tag: "item_added"; readonly item: string; readonly price: number }
    | { readonly tag: "item_removed"; readonly item: string; readonly price: number }
    | { readonly tag: "cart_cleared" };

// Projection 1: Most popular items
type ItemPopularity = ReadonlyMap<string, number>;

const projectPopularity = (events: readonly CartEvent[]): ItemPopularity =>
    events.reduce((acc, event) => {
        if (event.tag !== "item_added") return acc;
        const count = acc.get(event.item) ?? 0;
        return new Map([...acc, [event.item, count + 1]]);
    }, new Map() as ItemPopularity);

// Projection 2: Abandonment count
const projectAbandonment = (events: readonly CartEvent[]): number =>
    events.filter(e => e.tag === "cart_cleared").length;

// Test
const events: readonly CartEvent[] = [
    { tag: "item_added", item: "Laptop", price: 20000000 },
    { tag: "item_added", item: "Mouse", price: 500000 },
    { tag: "item_added", item: "Laptop", price: 20000000 },
    { tag: "cart_cleared" },
    { tag: "item_added", item: "Keyboard", price: 1000000 },
    { tag: "item_added", item: "Laptop", price: 20000000 },
];

const popularity = projectPopularity(events);
assert.strictEqual(popularity.get("Laptop"), 3);
assert.strictEqual(popularity.get("Mouse"), 1);

const abandonments = projectAbandonment(events);
assert.strictEqual(abandonments, 1);
```

</details>

---

**Bài 3** (15 phút): Full CQRS system

```typescript
// Viết mini ticket system với CQRS + Event Sourcing:
//
// Commands: create_ticket | assign | resolve | close
// Events (past tense): ticket_created | ticket_assigned | ticket_resolved | ticket_closed
//
// Write: validate transitions (create → assign → resolve → close)
// Read: 2 projections:
//   1. AgentWorkload: Map<agentId, ticketCount>
//   2. AverageResolutionTime: average (resolvedAt - createdAt)
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

// Events
type TicketEvent =
    | { readonly tag: "ticket_created"; readonly ticketId: string; readonly title: string; readonly timestamp: number }
    | { readonly tag: "ticket_assigned"; readonly ticketId: string; readonly agentId: string; readonly timestamp: number }
    | { readonly tag: "ticket_resolved"; readonly ticketId: string; readonly resolution: string; readonly timestamp: number }
    | { readonly tag: "ticket_closed"; readonly ticketId: string; readonly timestamp: number };

// Write model
type TicketState = {
    readonly id: string;
    readonly status: "open" | "assigned" | "resolved" | "closed";
    readonly agentId?: string;
    readonly createdAt: number;
};

type TicketStore = ReadonlyMap<string, TicketState>;

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// Commands
type TicketCommand =
    | { readonly tag: "create"; readonly ticketId: string; readonly title: string }
    | { readonly tag: "assign"; readonly ticketId: string; readonly agentId: string }
    | { readonly tag: "resolve"; readonly ticketId: string; readonly resolution: string }
    | { readonly tag: "close"; readonly ticketId: string };

const handleTicketCommand = (
    store: TicketStore,
    cmd: TicketCommand,
    timestamp: number
): Result<readonly TicketEvent[], string> => {
    switch (cmd.tag) {
        case "create":
            if (store.has(cmd.ticketId)) return err("Ticket exists");
            return ok([{ tag: "ticket_created", ticketId: cmd.ticketId, title: cmd.title, timestamp }]);
        case "assign": {
            const ticket = store.get(cmd.ticketId);
            if (!ticket) return err("Ticket not found");
            if (ticket.status !== "open") return err("Can only assign open tickets");
            return ok([{ tag: "ticket_assigned", ticketId: cmd.ticketId, agentId: cmd.agentId, timestamp }]);
        }
        case "resolve": {
            const ticket = store.get(cmd.ticketId);
            if (!ticket) return err("Ticket not found");
            if (ticket.status !== "assigned") return err("Can only resolve assigned tickets");
            return ok([{ tag: "ticket_resolved", ticketId: cmd.ticketId, resolution: cmd.resolution, timestamp }]);
        }
        case "close": {
            const ticket = store.get(cmd.ticketId);
            if (!ticket) return err("Ticket not found");
            if (ticket.status !== "resolved") return err("Can only close resolved tickets");
            return ok([{ tag: "ticket_closed", ticketId: cmd.ticketId, timestamp }]);
        }
    }
};

const applyTicketEvent = (store: TicketStore, event: TicketEvent): TicketStore => {
    switch (event.tag) {
        case "ticket_created":
            return new Map([...store, [event.ticketId, {
                id: event.ticketId, status: "open", createdAt: event.timestamp,
            }]]);
        case "ticket_assigned": {
            const ticket = store.get(event.ticketId)!;
            return new Map([...store, [event.ticketId, { ...ticket, status: "assigned", agentId: event.agentId }]]);
        }
        case "ticket_resolved": {
            const ticket = store.get(event.ticketId)!;
            return new Map([...store, [event.ticketId, { ...ticket, status: "resolved" }]]);
        }
        case "ticket_closed": {
            const ticket = store.get(event.ticketId)!;
            return new Map([...store, [event.ticketId, { ...ticket, status: "closed" }]]);
        }
    }
};

// Projection 1: Agent workload
const projectWorkload = (events: readonly TicketEvent[]): ReadonlyMap<string, number> =>
    events.reduce((acc, event) => {
        if (event.tag !== "ticket_assigned") return acc;
        const count = acc.get(event.agentId) ?? 0;
        return new Map([...acc, [event.agentId, count + 1]]);
    }, new Map() as ReadonlyMap<string, number>);

// Projection 2: Average resolution time
const projectAvgResolution = (events: readonly TicketEvent[]): number => {
    const created = new Map<string, number>();
    const times: number[] = [];

    for (const event of events) {
        if (event.tag === "ticket_created") created.set(event.ticketId, event.timestamp);
        if (event.tag === "ticket_resolved") {
            const start = created.get(event.ticketId);
            if (start !== undefined) times.push(event.timestamp - start);
        }
    }

    return times.length > 0 ? times.reduce((a, b) => a + b, 0) / times.length : 0;
};

// Test
const events: TicketEvent[] = [];
let store: TicketStore = new Map();

const process = (cmd: TicketCommand, ts: number) => {
    const result = handleTicketCommand(store, cmd, ts);
    if (result.tag === "ok") {
        for (const e of result.value) {
            events.push(e);
            store = applyTicketEvent(store, e);
        }
    }
};

process({ tag: "create", ticketId: "T1", title: "Bug #1" }, 1000);
process({ tag: "create", ticketId: "T2", title: "Bug #2" }, 2000);
process({ tag: "assign", ticketId: "T1", agentId: "Agent-A" }, 3000);
process({ tag: "assign", ticketId: "T2", agentId: "Agent-A" }, 4000);
process({ tag: "resolve", ticketId: "T1", resolution: "Fixed" }, 5000);

const workload = projectWorkload(events);
assert.strictEqual(workload.get("Agent-A"), 2);

const avgTime = projectAvgResolution(events);
assert.strictEqual(avgTime, 4000);  // T1: 5000 - 1000 = 4000. Only T1 resolved
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Event store quá lớn | Không snapshot | Snapshot mỗi N events |
| Schema evolution | Event shape thay đổi | Versioned events: `{ version: 2, ... }` + upcaster |
| Projection lag | Read model chưa catch up | Eventual consistency OK cho dashboards. Real-time → sync projection |
| Duplicate events | Event emitted 2 lần | Idempotent apply: check event ID trước khi apply |
| State inconsistent | Apply order sai | Events phải ordered by timestamp/sequence number |

---

## Tóm tắt

Chương này đã đưa bạn từ cuốn sổ kế toán đơn giản (event log) qua hệ thống báo cáo đa chiều (projections) đến kiến trúc hoàn chỉnh (CQRS + Event Sourcing + Snapshots). Mỗi concept xây trên những gì bạn đã học:

- ✅ **CQRS** = tách write (commands, validation) và read (queries, projections). Optimize independently.
- ✅ **Event Sourcing** = lưu events thay state. State = `events.reduce(apply, initial)`.
- ✅ **Events = past tense data** (immutable, append-only). Full audit trail.
- ✅ **Projections** = different views từ same events. `reduce` with different functions.
- ✅ **Temporal queries** = `events.filter(e => e.timestamp <= T).reduce(apply, initial)`.
- ✅ **Snapshots** = performance optimization. Rebuild from snapshot + remaining events.

## Part III Complete! 🎉

Chapters 16–17 dạy bạn **design patterns** FP: GoF → FP translation (giàn giáo → bê tông đúc sẵn) và CQRS + Event Sourcing (cuốn sổ kế toán bất biến). Cả hai patterns DỰA TRÊN immutability, ADTs, và function composition từ Part II — bạn thấy mọi thứ kết nối. Part IV sẽ áp dụng tất cả vào **Domain-Driven Design** — nơi code gặp nghiệp vụ.

## Tiếp theo

→ Part IV: **Domain-Driven Design** — Chapter 18: **Introduction to DDD**. Ubiquitous Language, Bounded Contexts, Event Storming.
