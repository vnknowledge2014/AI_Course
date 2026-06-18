# Chapter 18 — Introduction to DDD

> **Bạn sẽ học được**:
> - Domain-Driven Design là gì và tại sao quan trọng
> - Ubiquitous Language — ngôn ngữ chung giữa dev và domain experts
> - Bounded Contexts — ranh giới module rõ ràng
> - Entities vs Value Objects — identity vs equality
> - Domain Events — sự kiện quan trọng trong business
> - Event Storming — collaborative discovery technique
>
> **Yêu cầu trước**: Chapter 13 (ADTs, branded types), Chapter 17 (CQRS, events).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Hiểu nền tảng DDD — cách mà FP + TypeScript model domain phức tạp.

---

Bạn đã bao giờ cầm bản đồ một thành phố chưa?

Bản đồ tốt không chỉ vẽ đường — nó **phản ánh thực tế**. Quận 1 có những con đường khác quận 7. "Chợ" ở quận 5 nghĩa khác "chợ" ở quận 2 (chợ truyền thống vs trung tâm thương mại). Bản đồ dùng tên mà cư dân sử dụng — không phải mã số kỹ thuật. Và ranh giới quận rõ ràng — bạn biết mình đang ở đâu.

**Domain-Driven Design** là việc vẽ bản đồ cho phần mềm. Domain (lĩnh vực kinh doanh) = thành phố. Types = tên đường và địa danh. Bounded Contexts = ranh giới quận. Ubiquitous Language = ngôn ngữ mà DÂN ĐỊA PHƯƠNG (domain experts) sử dụng, không phải ngôn ngữ quy hoạch đô thị (tech jargon). Khi code nói cùng ngôn ngữ với business — developer đọc code hiểu business, PO đọc types hiểu logic. Không cần "phiên dịch".

---

## 18.1 — Domain-Driven Design là gì?

### Khi bản đồ không khớp thực tế

Hãy tưởng tượng bản đồ thành phố dùng mã số thay vì tên đường: "đường A7" thay vì "Nguyễn Huệ", "khu vực 0x3F" thay vì "Quận 1". Không ai dùng được — kể cả người vẽ bản đồ sau vài tháng cũng quên. Đó chính xác là code không dùng DDD: `data.s = 1` thay vì `order.status = "confirmed"`.

```typescript
// filename: src/code_business_gap.ts

// ❌ Code KHÔNG nói "ngôn ngữ" business
const processOrder = (data: any) => {
    if (data.s === 1) {           // s = status? side? step?
        data.p = data.p * 0.9;   // p = price? position?
        data.s = 2;              // magic numbers everywhere
        updateDB(data);          // side effect ở đâu cũng có
    }
};

// Developer mới: "data.s = 1 nghĩa là gì?"
// Product owner: "Khi đơn hàng ĐÃ XÁC NHẬN, áp dụng chiết khấu 10%"
// → Code và Business nói HAI NGÔN NGỮ KHÁC NHAU

// ✅ DDD: Code IS the domain model
type OrderStatus = "draft" | "confirmed" | "shipped" | "delivered";

type Order = {
    readonly id: string;
    readonly status: OrderStatus;
    readonly totalPrice: number;      // "totalPrice" — business term!
    readonly discountPercent: number;  // "discountPercent" — business term!
};

const applyConfirmationDiscount = (order: Order): Order => ({
    ...order,
    totalPrice: order.totalPrice * (1 - order.discountPercent / 100),
    status: "confirmed",
});

// Developer mới: "applyConfirmationDiscount" — tự giải thích!
// Product owner: đọc function name → hiểu ngay
```

### Eric Evans (2003) — "Domain-Driven Design"

DDD = **phần mềm PHẢN ÁNH domain** (lĩnh vực kinh doanh) — giống bản đồ phản ánh thành phố. Ba trụ cột:

| Trụ cột | Ý nghĩa | TypeScript mapping |
|---------|---------|-------------------|
| **Ubiquitous Language** | Dev + business dùng CÙNG từ ngữ | Type names = business terms |
| **Bounded Contexts** | Ranh giới modules rõ ràng | Module boundaries, barrel exports |
| **Domain Model** | Code = business logic | Types + pure functions |

> **💡 DDD + FP = natural fit**: FP dùng immutable types + pure functions. DDD dùng Value Objects + Domain Events. TypeScript cung cấp CẢ HAI: branded types (Ch13), ADTs, `Result<T,E>`, `pipe()`.

---

## ✅ Checkpoint 18.1

> Đến đây bạn phải hiểu:
> 1. **DDD** = code phản ánh domain. Không còn magic numbers, cryptic names
> 2. **3 trụ cột**: Ubiquitous Language, Bounded Contexts, Domain Model
> 3. **FP + DDD** = natural fit: immutable types, pure functions, branded types
>
> **Test nhanh**: Variable name `s` vs `orderStatus` — cái nào DDD?
> <details><summary>Đáp án</summary>`orderStatus`! DDD yêu cầu ubiquitous language — tên phải LÀ business term. `s` = magic abbreviation, vi phạm DDD nguyên tắc đầu tiên.</details>

---

## 18.2 — Ubiquitous Language

### Tên đường phải là tên mà dân dùng

Quay lại bản đồ: nếu dân gọi là "Nguyễn Huệ" mà bản đồ ghi "Street #47" — bản đồ vô dụng. Ubiquitous Language = dùng **chính xác** từ ngữ mà domain expert (product owner, business analyst) sử dụng. `Money`, không phải `num`. `CustomerTier`, không phải `level`. `ShippingPolicy`, không phải `config`.

TypeScript type definitions CHÍNH LÀ glossary (từ điển domain). Khi PO đọc `type Discount = "percentage" | "fixed_amount" | "buy_x_get_y"` và nói "Đúng, ba loại chiết khấu của chúng tôi" — bạn có Ubiquitous Language.

```typescript
// filename: src/ubiquitous_language.ts

// --- E-commerce domain ---

// ❌ Tech language — developer hiểu, business KHÔNG hiểu
// type Record = { type: number; data: string[]; flag: boolean };
// const processRecord = (r: Record) => { ... };

// ✅ Ubiquitous Language — CẢ HAI hiểu
type Money = number & { readonly __brand: "Money" };
type ProductName = string & { readonly __brand: "ProductName" };
type CustomerId = string & { readonly __brand: "CustomerId" };

type LineItem = {
    readonly productName: ProductName;
    readonly unitPrice: Money;
    readonly quantity: number;
    readonly lineTotal: Money;    // business term: "line total"
};

type Order = {
    readonly orderId: string;
    readonly customerId: CustomerId;
    readonly items: readonly LineItem[];
    readonly subtotal: Money;     // business term: "subtotal"
    readonly tax: Money;          // business term: "tax"
    readonly total: Money;        // business term: "total"
    readonly status: OrderStatus;
};

type OrderStatus =
    | "draft"           // Nháp — chưa thanh toán
    | "pending_payment" // Chờ thanh toán
    | "confirmed"       // Đã xác nhận
    | "processing"      // Đang xử lý
    | "shipped"         // Đã giao vận chuyển
    | "delivered"       // Đã giao hàng
    | "cancelled"       // Đã hủy
    | "refunded";       // Đã hoàn tiền
```

### Glossary = Types

```typescript
// filename: src/glossary.ts

// DDD: domain glossary LIVED trong code

// "Khách hàng" = Customer
type Customer = {
    readonly id: CustomerId;
    readonly name: string;
    readonly email: string;
    readonly tier: CustomerTier;
};

// "Hạng khách" = Customer Tier
type CustomerTier =
    | "standard"    // Thường
    | "silver"      // Bạc (chi > 5M/năm)
    | "gold"        // Vàng (chi > 20M/năm)
    | "platinum";   // Bạch kim (chi > 50M/năm)

// "Chiết khấu" = Discount
type Discount =
    | { readonly tag: "percentage"; readonly percent: number }     // Giảm %
    | { readonly tag: "fixed_amount"; readonly amount: Money }     // Giảm cố định
    | { readonly tag: "buy_x_get_y"; readonly buy: number; readonly free: number };  // Mua X tặng Y

// "Chính sách vận chuyển" = Shipping Policy
type ShippingPolicy =
    | { readonly tag: "free"; readonly minOrderTotal: Money }     // Miễn phí khi >= X
    | { readonly tag: "flat_rate"; readonly rate: Money }         // Phí cố định
    | { readonly tag: "weight_based"; readonly ratePerKg: Money };  // Theo cân nặng

// Business stakeholder đọc type definitions → HIỂU ngay domain rules
```

> **💡 Litmus test**: Nếu Product Owner có thể đọc type definitions và nói "Đúng, đây là cách business hoạt động" → bạn có Ubiquitous Language.

---

## ✅ Checkpoint 18.2

> Đến đây bạn phải hiểu:
> 1. **Ubiquitous Language** = dev + business dùng CÙNG từ ngữ
> 2. **Types = glossary**: `Money`, `CustomerTier`, `ShippingPolicy` — business terms
> 3. **DU = business rules**: `Discount` có 3 loại — code PHẢN ÁNH domain
> 4. **Litmus test**: PO đọc types → hiểu không?
>
> **Test nhanh**: `type Tier = 0 | 1 | 2 | 3` vs `type CustomerTier = "standard" | "silver" | "gold" | "platinum"` — cái nào DDD?
> <details><summary>Đáp án</summary>`CustomerTier` với string literals! Self-documenting, business term. `0 | 1 | 2 | 3` = magic numbers — PO không biết 2 nghĩa là gì.</details>

---

## 18.3 — Bounded Contexts

### Cùng tên đường, khác quận

Trong thành phố Hồ Chí Minh, "đường Lê Lợi" ở Quận 1 khác hoàn toàn "đường Lê Lợi" ở Gò Vấp. Bạn cần **ranh giới quận** để biết đang nói đường nào. Bounded Context = ranh giới quận cho phần mềm: "Product" trong Sales context (cần giá, chiết khấu) khác "Product" trong Warehouse context (cần SKU, kích thước, vị trí kho) khác "Product" trong Shipping context (cần cân nặng, fragile?).

Nếu bạn cố nhét MỌI thông tin vào MỘT `Product` type — type phình to, ai cũng phụ thuộc vào nó, sửa cho đội A phá đội B. Bounded Context nói: "mỗi team có Product RIÊNG, phù hợp với công việc RIÊNG". Giao tiếp giữa contexts qua **Anti-Corruption Layer** — translator dịch ngôn ngữ quận này sang quận khác.

```typescript
// filename: src/bounded_contexts.ts

// "Product" trong SALES context
// ⚠️ Dùng namespace ở đây chỉ để MINH HỌA ranh giới.
// Thực tế: dùng separate files/folders (sales/types.ts, warehouse/types.ts)
namespace Sales {
    type Product = {
        readonly id: string;
        readonly name: string;
        readonly price: number;
        readonly discountEligible: boolean;
        // Sales cần: giá, chiết khấu, tên hiển thị
    };
}

// "Product" trong WAREHOUSE context
namespace Warehouse {
    type Product = {
        readonly id: string;
        readonly sku: string;
        readonly weight: number;
        readonly dimensions: { readonly length: number; readonly width: number; readonly height: number };
        readonly location: string;
        // Warehouse cần: SKU, kích thước, vị trí kho
    };
}

// "Product" trong SHIPPING context
namespace Shipping {
    type Product = {
        readonly id: string;
        readonly weight: number;
        readonly isFragile: boolean;
        readonly requiresColdChain: boolean;
        // Shipping cần: cân nặng, xử lý đặc biệt
    };
}

// CÙNG từ "Product" — BA nghĩa KHÁC NHAU!
// Bounded Context = ranh giới nơi từ có nghĩa NHẤT QUÁN
```

### Context map — quan hệ giữa các contexts

```typescript
// filename: src/context_map.ts

// Anti-Corruption Layer = translator giữa contexts
// Sales → Warehouse: chỉ gửi thông tin cần thiết

type SalesOrder = {
    readonly orderId: string;
    readonly items: readonly { readonly productId: string; readonly quantity: number }[];
};

type WarehousePickingList = {
    readonly orderId: string;
    readonly items: readonly { readonly sku: string; readonly quantity: number; readonly location: string }[];
};

// Translator: Sales context → Warehouse context
const toPickingList = (
    order: SalesOrder,
    productCatalog: ReadonlyMap<string, { sku: string; location: string }>
): WarehousePickingList => ({
    orderId: order.orderId,
    items: order.items.map(item => {
        const product = productCatalog.get(item.productId)!;
        return {
            sku: product.sku,
            quantity: item.quantity,
            location: product.location,
        };
    }),
});

// Sales KHÔNG cần biết về kho. Warehouse KHÔNG cần biết về giá.
// Anti-Corruption Layer dịch giữa hai ngôn ngữ.
```

### Module structure = Bounded Contexts

```
src/
├── sales/                    # Sales Bounded Context
│   ├── types.ts              # Order, LineItem, Discount
│   ├── pricing.ts            # calculateTotal, applyDiscount
│   ├── validation.ts         # validateOrder
│   └── index.ts              # barrel export (public API)
│
├── warehouse/                # Warehouse Bounded Context
│   ├── types.ts              # PickingList, StockLevel
│   ├── inventory.ts          # checkStock, reserveStock
│   └── index.ts
│
├── shipping/                 # Shipping Bounded Context
│   ├── types.ts              # Shipment, TrackingInfo
│   ├── calculator.ts         # calculateShippingCost
│   └── index.ts
│
└── shared/                   # Shared Kernel (minimal!)
    ├── types.ts              # ProductId, Money (branded types)
    └── events.ts             # Domain events
```

> **💡 Barrel exports = Context boundary**: `index.ts` chỉ export PUBLIC types. Internal types = private. Import `from "./sales"` — không `from "./sales/internal/helper"`.

---

## ✅ Checkpoint 18.3

> Đến đây bạn phải hiểu:
> 1. **Bounded Context** = ranh giới nơi ubiquitous language nhất quán
> 2. **Cùng từ khác nghĩa**: "Product" ở Sales ≠ Warehouse ≠ Shipping
> 3. **Anti-Corruption Layer** = translator giữa contexts
> 4. **Module structure** = bounded contexts. Barrel exports = public API
>
> **Test nhanh**: Sales cần biết `product.location` (vị trí kho) không?
> <details><summary>Đáp án</summary>**KHÔNG!** `location` thuộc Warehouse context. Sales chỉ cần `price`, `name`, `discountEligible`. Bounded Context ngăn thông tin rò rỉ.</details>

---

## 18.4 — Entities vs Value Objects

### Tòa nhà có địa chỉ vs tờ tiền không tên

Trong thành phố, mỗi tòa nhà có **địa chỉ duy nhất** — đó là identity. Hai tòa nhà cùng kiến trúc, cùng màu sơn vẫn KHÁC nhau vì địa chỉ khác. Tòa nhà = **Entity**. Ngược lại, tờ 50,000 VND này và tờ 50,000 VND kia — bạn có phân biệt không? Không. Chúng bằng nhau vì giá trị bằng nhau. Tiền = **Value Object**.

Entity cần ID để phân biệt — `User { id: "U1" }` và `User { id: "U2" }` khác nhau dù cùng tên. Value Object so sánh bằng giá trị — `Money { amount: 50000, currency: "VND" }` bằng bất kỳ tờ 50K VND nào khác. FP thiên về Value Objects vì chúng tự nhiên immutable, không có lifecycle, dễ reason about.

```typescript
// filename: src/entities_values.ts
import assert from "node:assert/strict";

// ENTITY: identity QUAN TRỌNG. Hai users KHÁC nhau dù cùng tên.
type UserId = string & { readonly __brand: "UserId" };

type User = {
    readonly id: UserId;
    readonly name: string;
    readonly email: string;
};

// Entity equality = by ID
const sameEntity = (a: User, b: User): boolean => a.id === b.id;

const user1: User = { id: "U1" as UserId, name: "An", email: "an@mail.com" };
const user2: User = { id: "U1" as UserId, name: "An (updated)", email: "an@new.com" };
const user3: User = { id: "U2" as UserId, name: "An", email: "an@mail.com" };

assert.strictEqual(sameEntity(user1, user2), true);   // same ID = same entity
assert.strictEqual(sameEntity(user1, user3), false);   // different ID = different entity
// user1 và user3 có cùng name + email nhưng KHÁC entity!
```

### Value Object = chỉ so sánh bằng value

```typescript
// filename: src/value_objects.ts
import assert from "node:assert/strict";

// VALUE OBJECT: identity KHÔNG quan trọng. So sánh bằng giá trị.
// 50,000 VND = 50,000 VND. Không cần "ID" cho tiền.

type Money = {
    readonly amount: number;
    readonly currency: "VND" | "USD" | "EUR";
};

// Value Object equality = by value
const sameMoney = (a: Money, b: Money): boolean =>
    a.amount === b.amount && a.currency === b.currency;

const price1: Money = { amount: 50000, currency: "VND" };
const price2: Money = { amount: 50000, currency: "VND" };
const price3: Money = { amount: 50000, currency: "USD" };

assert.strictEqual(sameMoney(price1, price2), true);   // same value = same
assert.strictEqual(sameMoney(price1, price3), false);   // different currency

// Value Object operations → return NEW value object (immutable!)
const addMoney = (a: Money, b: Money): Money => {
    if (a.currency !== b.currency) throw new Error("Cannot add different currencies");
    return { amount: a.amount + b.amount, currency: a.currency };
};

const total = addMoney(price1, price2);
assert.strictEqual(total.amount, 100000);

console.log("Entity vs Value Object OK ✅");
```

### Bảng so sánh

| | Entity | Value Object |
|--|--------|-------------|
| **Identity** | Có ID duy nhất | Không có ID |
| **Equality** | So sánh bằng ID | So sánh bằng value |
| **Mutability** | Có thể thay đổi (state) | Luôn immutable |
| **Ví dụ** | User, Order, Account | Money, Email, Address, Color |
| **TypeScript** | `{ id: UserId; ... }` | Branded types hoặc plain objects |
| **Lifespan** | Có lifecycle (create → update → delete) | Stateless, replaceable |

> **💡 FP preference**: Value Objects — immutable, no identity, no lifecycle. FP naturally models Value Objects. Entities need extra care (ID tracking, state management).

---

## ✅ Checkpoint 18.4

> Đến đây bạn phải hiểu:
> 1. **Entity** = identity (ID). Cùng tên ≠ cùng entity
> 2. **Value Object** = so sánh bằng value. 50K VND = 50K VND
> 3. **Immutability** = Value Objects tự nhiên immutable. Entities cần ID tracking
> 4. **FP loves Value Objects**: branded types = type-safe Value Objects
>
> **Test nhanh**: `Address { street: "123 Main", city: "HCM" }` — Entity hay Value Object?
> <details><summary>Đáp án</summary>**Value Object!** Hai addresses giống nhau = same address. Không cần "address ID". Compare by value.</details>

---

## 18.5 — Domain Events

### Sự kiện trên bản đồ — "Cháy ở Quận 3", không phải "Nhiệt độ tăng 500°C"

Domain Events dùng **ngôn ngữ business**, không phải tech. "OrderPlaced" (Đơn hàng đã đặt), không phải "DatabaseRowInserted". "PaymentReceived" (Thanh toán đã nhận), không phải "HTTPRequestProcessed". Chúng nói điều mà **domain expert quan tâm** — và dùng thì quá khứ (past tense) vì event = sự kiện ĐÃ XẢY RA, không thể thay đổi.

Domain Events kết nối trực tiếp với Event Sourcing (Ch17): chúng là NGUỒN cho event store. Và chúng kích hoạt workflows — event này gây ra event khác, tạo thành chuỗi nghiệp vụ hoàn chỉnh.

```typescript
// filename: src/domain_events.ts

// Domain Events = past tense, business language (từ Ch17!)
type DomainEvent =
    | { readonly tag: "OrderPlaced"; readonly orderId: string; readonly customerId: string; readonly total: number; readonly occurredAt: Date }
    | { readonly tag: "PaymentReceived"; readonly orderId: string; readonly amount: number; readonly paymentMethod: string; readonly occurredAt: Date }
    | { readonly tag: "OrderShipped"; readonly orderId: string; readonly trackingNumber: string; readonly carrier: string; readonly occurredAt: Date }
    | { readonly tag: "OrderDelivered"; readonly orderId: string; readonly deliveredAt: Date; readonly occurredAt: Date }
    | { readonly tag: "OrderCancelled"; readonly orderId: string; readonly reason: string; readonly cancelledBy: string; readonly occurredAt: Date };

// Events KHÔNG phải tech events — chúng là BUSINESS events
// Product Owner hiểu: "OrderPlaced" = "Đơn hàng đã được đặt"
// Không phải: "DatabaseRowInserted" hay "HTTPRequestReceived"

// Domain Event handlers
type EventHandler<E> = (event: E) => void;

const onOrderPlaced: EventHandler<DomainEvent & { tag: "OrderPlaced" }> = (event) => {
    console.log(`📦 Đơn #${event.orderId} đã đặt. Tổng: ${event.total.toLocaleString()} VND`);
    // Side effects: send confirmation email, reserve stock, etc.
};

const onPaymentReceived: EventHandler<DomainEvent & { tag: "PaymentReceived" }> = (event) => {
    console.log(`💰 Thanh toán #${event.orderId}: ${event.amount.toLocaleString()} via ${event.paymentMethod}`);
};
```

### Events trigger workflows

```typescript
// filename: src/event_workflows.ts

// Event → Reaction → More Events (event-driven architecture)

type DomainEvent =
    | { readonly tag: "OrderPlaced"; readonly orderId: string; readonly total: number }
    | { readonly tag: "PaymentReceived"; readonly orderId: string; readonly amount: number }
    | { readonly tag: "StockReserved"; readonly orderId: string; readonly items: readonly string[] }
    | { readonly tag: "OrderConfirmed"; readonly orderId: string };

// Workflow: OrderPlaced → check payment → reserve stock → confirm
const orderWorkflow = (event: DomainEvent): readonly DomainEvent[] => {
    switch (event.tag) {
        case "OrderPlaced":
            // Trigger: check if payment needed
            return [{ tag: "PaymentReceived", orderId: event.orderId, amount: event.total }];

        case "PaymentReceived":
            // Trigger: reserve stock
            return [{ tag: "StockReserved", orderId: event.orderId, items: ["item1"] }];

        case "StockReserved":
            // Trigger: confirm order
            return [{ tag: "OrderConfirmed", orderId: event.orderId }];

        case "OrderConfirmed":
            return [];  // Terminal event — no more reactions
    }
};

// Event chain: OrderPlaced → PaymentReceived → StockReserved → OrderConfirmed
// Mỗi event = 1 step trong workflow
// Pure functions: input event → output events
```

> **💡 Domain Events vs Technical Events**: Domain events nói business language ("OrderPlaced"), không phải tech language ("RowInserted"). Ch17 đã dạy Event Sourcing — Domain Events là NGUỒN cho event sourcing.

---

## ✅ Checkpoint 18.5

> Đến đây bạn phải hiểu:
> 1. **Domain Events** = business events, past tense, ubiquitous language
> 2. **Events trigger workflows**: OrderPlaced → ... → OrderConfirmed
> 3. **Events ≠ tech events**: "OrderPlaced" ≠ "DatabaseInsert"
> 4. **Connection to Ch17**: Domain Events = event sourcing events
>
> **Test nhanh**: `UserClickedButton` — Domain Event hay Technical Event?
> <details><summary>Đáp án</summary>**Technical Event!** Business không nói "clicked button". Domain Event: "UserRequestedCancellation" hoặc "UserSubmittedOrder".</details>

---

## 18.6 — Event Storming

### Vẽ bản đồ cùng dân địa phương

Event Storming = workshop technique do Alberto Brandolini sáng tạo. Thay vì developer TỰ vẽ bản đồ (và vẽ sai), team (dev + PO + domain experts) CÙNG khám phá domain — giống khảo sát thực địa với hướng dẫn viên bản địa. Kết quả: sticky notes trên tường trở thành TypeScript types.

### Quy trình

```
1. 🟧 DOMAIN EVENTS (orange sticky notes)
   "Đơn hàng đã đặt", "Thanh toán đã nhận", "Hàng đã xuất kho"
   → Past tense, business language
   → Đặt trên timeline từ trái → phải

2. 🟦 COMMANDS (blue sticky notes)
   "Đặt đơn hàng", "Thanh toán", "Xuất kho"
   → Hành động GÂY RA events
   → Đặt TRƯỚC event tương ứng

3. 🟨 AGGREGATES (yellow sticky notes)
   "Đơn hàng", "Kho hàng", "Thanh toán"
   → Nhóm events + commands liên quan
   → = Bounded Context candidates!

4. 🟪 POLICIES (purple sticky notes)
   "Nếu thanh toán thành công → xác nhận đơn"
   "Nếu hết hàng → thông báo khách"
   → Business rules tự động

5. 🟩 READ MODELS (green sticky notes)
   "Dashboard đơn hàng", "Báo cáo doanh thu"
   → CQRS read side!
```

### Event Storming → TypeScript Types

Sau workshop, sticky notes biến thành code. Events = DU variants. Commands = DU variants. Aggregates = module boundaries. Policies = event handlers. Read models = projections (Ch17). Bản đồ trên tường trở thành bản đồ trong code — và bây giờ compiler kiểm tra bản đồ cho bạn.

```typescript
// filename: src/event_storming_output.ts

// Sau Event Storming session, team thu được:

// --- EVENTS (orange) ---
type OrderEvent =
    | { readonly tag: "OrderPlaced" }
    | { readonly tag: "OrderConfirmed" }
    | { readonly tag: "OrderShipped" }
    | { readonly tag: "OrderDelivered" }
    | { readonly tag: "OrderCancelled" };

type PaymentEvent =
    | { readonly tag: "PaymentInitiated" }
    | { readonly tag: "PaymentSucceeded" }
    | { readonly tag: "PaymentFailed" }
    | { readonly tag: "RefundIssued" };

type InventoryEvent =
    | { readonly tag: "StockReserved" }
    | { readonly tag: "StockReleased" }
    | { readonly tag: "StockDepleted" };

// --- COMMANDS (blue) ---
type OrderCommand =
    | { readonly tag: "PlaceOrder" }
    | { readonly tag: "ConfirmOrder" }
    | { readonly tag: "CancelOrder" };

type PaymentCommand =
    | { readonly tag: "InitiatePayment" }
    | { readonly tag: "ProcessRefund" };

// --- AGGREGATES (yellow) → Bounded Contexts ---
// OrderAggregate → "sales" context
// PaymentAggregate → "payment" context
// InventoryAggregate → "warehouse" context

// --- POLICIES (purple) → Automation rules ---
// "PaymentSucceeded → ConfirmOrder" (automatic)
// "StockDepleted → NotifyPurchasing" (automatic)

// --- READ MODELS (green) → CQRS projections ---
// "Order Dashboard" → projectOrderDashboard(events)
// "Revenue Report" → projectRevenue(events)
```

> **💡 Event Storming → Code**: Sticky notes trở thành type definitions. Events = DU variants. Commands = DU variants. Aggregates = modules. Policies = event handlers. Read models = projections (Ch17).

---

## ✅ Checkpoint 18.6

> Đến đây bạn phải hiểu:
> 1. **Event Storming** = collaborative workshop. Dev + PO + domain experts
> 2. **5 elements**: Events (orange), Commands (blue), Aggregates (yellow), Policies (purple), Read Models (green)
> 3. **Output → TypeScript**: Events=DU, Commands=DU, Aggregates=modules, Policies=handlers, Read Models=projections
> 4. **Timeline**: Commands → Events → Policies → More Events
>
> **Test nhanh**: Sticky note "Nếu hết hàng → email cho bộ phận mua hàng" — color gì?
> <details><summary>Đáp án</summary>**🟪 Purple (Policy)**! Đây là automation rule: event "StockDepleted" → reaction "NotifyPurchasing". Policies = automatic business rules.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Ubiquitous Language

```typescript
// Refactor code sau thành ubiquitous language:
//
// type D = { t: number; i: { n: string; p: number; q: number }[]; f: boolean };
// const proc = (d: D) => {
//     if (d.f) d.t = d.i.reduce((s, x) => s + x.p * x.q, 0) * 0.9;
//     else d.t = d.i.reduce((s, x) => s + x.p * x.q, 0);
// };
//
// Viết lại với business terms: Order, LineItem, applyMemberDiscount, calculateTotal
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type LineItem = {
    readonly name: string;
    readonly unitPrice: number;
    readonly quantity: number;
};

type Order = {
    readonly items: readonly LineItem[];
    readonly isMember: boolean;
};

const calculateSubtotal = (items: readonly LineItem[]): number =>
    items.reduce((sum, item) => sum + item.unitPrice * item.quantity, 0);

const MEMBER_DISCOUNT = 0.1;  // 10%

const calculateTotal = (order: Order): number => {
    const subtotal = calculateSubtotal(order.items);
    return order.isMember
        ? subtotal * (1 - MEMBER_DISCOUNT)
        : subtotal;
};

// Test
const order: Order = {
    items: [
        { name: "Laptop", unitPrice: 20000000, quantity: 1 },
        { name: "Mouse", unitPrice: 500000, quantity: 2 },
    ],
    isMember: true,
};

const total = calculateTotal(order);
// subtotal = 20M + 1M = 21M, discount 10% = 18.9M
assert.strictEqual(total, 18900000);

const nonMember = { ...order, isMember: false };
assert.strictEqual(calculateTotal(nonMember), 21000000);
```

</details>

---

**Bài 2** (10 phút): Bounded Contexts

```typescript
// Thiết kế bounded contexts cho "Restaurant System":
//
// 1. Xác định 3 bounded contexts (ít nhất)
// 2. Cho mỗi context, viết type definitions cho "MenuItem"
//    (mỗi context cần thông tin KHÁC nhau về món ăn)
// 3. Viết Anti-Corruption Layer: KitchenOrder → WaiterBill
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

// Context 1: MENU (hiển thị cho khách)
namespace Menu {
    export type MenuItem = {
        readonly id: string;
        readonly name: string;
        readonly description: string;
        readonly price: number;
        readonly category: "appetizer" | "main" | "dessert" | "drink";
        readonly isAvailable: boolean;
        readonly imageUrl: string;
    };
}

// Context 2: KITCHEN (nấu ăn)
namespace Kitchen {
    export type MenuItem = {
        readonly id: string;
        readonly name: string;
        readonly recipe: readonly string[];
        readonly prepTimeMinutes: number;
        readonly allergens: readonly string[];
        readonly station: "grill" | "fry" | "cold" | "bakery";
    };

    export type KitchenOrder = {
        readonly orderId: string;
        readonly items: readonly { readonly menuItemId: string; readonly quantity: number; readonly specialNotes: string }[];
        readonly priority: "normal" | "rush";
    };
}

// Context 3: BILLING (thanh toán)
namespace Billing {
    export type MenuItem = {
        readonly id: string;
        readonly name: string;
        readonly price: number;
        readonly taxRate: number;
    };

    export type Bill = {
        readonly orderId: string;
        readonly items: readonly { readonly name: string; readonly price: number; readonly quantity: number }[];
        readonly subtotal: number;
        readonly tax: number;
        readonly total: number;
    };
}

// Anti-Corruption Layer: Kitchen → Billing
const toBill = (
    kitchenOrder: Kitchen.KitchenOrder,
    menuPrices: ReadonlyMap<string, { name: string; price: number; taxRate: number }>
): Billing.Bill => {
    const items = kitchenOrder.items.map(item => {
        const info = menuPrices.get(item.menuItemId)!;
        return { name: info.name, price: info.price, quantity: item.quantity };
    });

    const subtotal = items.reduce((sum, i) => sum + i.price * i.quantity, 0);
    const tax = Math.round(subtotal * 0.1);  // 10% tax

    return {
        orderId: kitchenOrder.orderId,
        items,
        subtotal,
        tax,
        total: subtotal + tax,
    };
};

// Test
const prices = new Map([
    ["M1", { name: "Phở", price: 50000, taxRate: 0.1 }],
    ["M2", { name: "Trà đá", price: 10000, taxRate: 0.1 }],
]);

const kitchenOrder: Kitchen.KitchenOrder = {
    orderId: "ORD-1",
    items: [
        { menuItemId: "M1", quantity: 2, specialNotes: "ít hành" },
        { menuItemId: "M2", quantity: 3, specialNotes: "" },
    ],
    priority: "normal",
};

const bill = toBill(kitchenOrder, prices);
assert.strictEqual(bill.subtotal, 130000);  // 50K*2 + 10K*3 = 130K
assert.strictEqual(bill.tax, 13000);
assert.strictEqual(bill.total, 143000);
```

</details>

---

**Bài 3** (15 phút): Event Storming → Types

```typescript
// Thực hiện "mini Event Storming" cho "Library System" (thư viện):
//
// 1. Liệt kê ≥ 5 Domain Events (past tense)
// 2. Liệt kê ≥ 3 Commands (imperative)
// 3. Xác định ≥ 2 Aggregates (bounded contexts)
// 4. Viết ≥ 1 Policy (automatic rule)
// 5. Viết TypeScript types cho tất cả
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

// --- EVENTS ---
type LibraryEvent =
    | { readonly tag: "BookBorrowed"; readonly bookId: string; readonly memberId: string; readonly dueDate: string; readonly occurredAt: string }
    | { readonly tag: "BookReturned"; readonly bookId: string; readonly memberId: string; readonly returnedAt: string; readonly occurredAt: string }
    | { readonly tag: "BookOverdue"; readonly bookId: string; readonly memberId: string; readonly daysOverdue: number; readonly occurredAt: string }
    | { readonly tag: "FineIssued"; readonly memberId: string; readonly amount: number; readonly reason: string; readonly occurredAt: string }
    | { readonly tag: "BookReserved"; readonly bookId: string; readonly memberId: string; readonly occurredAt: string }
    | { readonly tag: "ReservationExpired"; readonly bookId: string; readonly memberId: string; readonly occurredAt: string };

// --- COMMANDS ---
type LibraryCommand =
    | { readonly tag: "BorrowBook"; readonly bookId: string; readonly memberId: string }
    | { readonly tag: "ReturnBook"; readonly bookId: string; readonly memberId: string }
    | { readonly tag: "ReserveBook"; readonly bookId: string; readonly memberId: string }
    | { readonly tag: "PayFine"; readonly memberId: string; readonly amount: number };

// --- AGGREGATES ---
// 1. Catalog (books, availability)
type Book = {
    readonly id: string;
    readonly title: string;
    readonly author: string;
    readonly isbn: string;
    readonly status: "available" | "borrowed" | "reserved" | "maintenance";
};

// 2. Membership (members, borrowing, fines)
type Member = {
    readonly id: string;
    readonly name: string;
    readonly activeBorrows: number;
    readonly maxBorrows: number;  // 5 cho standard, 10 cho premium
    readonly outstandingFines: number;
};

// --- POLICIES ---
// Policy: BookOverdue → FineIssued (automatic)
const overduePolicy = (event: LibraryEvent): readonly LibraryEvent[] => {
    if (event.tag !== "BookOverdue") return [];
    const finePerDay = 5000;  // 5,000 VND / ngày
    return [{
        tag: "FineIssued",
        memberId: event.memberId,
        amount: event.daysOverdue * finePerDay,
        reason: `Trả sách trễ ${event.daysOverdue} ngày`,
        occurredAt: event.occurredAt,
    }];
};

// Test policy
const overdueEvent: LibraryEvent = {
    tag: "BookOverdue",
    bookId: "B1",
    memberId: "M1",
    daysOverdue: 3,
    occurredAt: "2024-01-15",
};

const fines = overduePolicy(overdueEvent);
assert.strictEqual(fines.length, 1);
if (fines[0].tag === "FineIssued") {
    assert.strictEqual(fines[0].amount, 15000);  // 3 * 5000
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Types dùng tech terms (`Record`, `data`) | Chưa nói chuyện với domain expert | Event Storming! Dùng business terms |
| Bounded Context quá lớn | Monolith thinking | Split khi cùng từ có nghĩa khác. "Small is beautiful" |
| Entity vs Value Object confusion | Mọi thứ đều có ID | Hỏi: "Có cần PHÂN BIỆT 2 instance cùng giá trị không?" → ID = Entity |
| Domain Events = CRUD events | Dùng "Created", "Updated", "Deleted" | KHÔNG! Dùng business terms: "OrderPlaced", "PaymentReceived" |
| Shared Kernel quá lớn | Quá nhiều shared types | Shared kernel = MINIMAL. Chỉ branded types + common events |

---

## Tóm tắt

Chương này đã trang bị cho bạn chiếc la bàn để vẽ bản đồ domain — từ Ubiquitous Language (tên đường đúng ngôn ngữ dân dùng), qua Bounded Contexts (ranh giới quận rõ ràng), đến Entities vs Value Objects (tòa nhà có địa chỉ vs tờ tiền không tên), Domain Events (sự kiện trên bản đồ), và Event Storming (vẽ bản đồ cùng dân bản địa).

- ✅ **DDD** = code phản ánh domain. Types = business glossary.
- ✅ **Ubiquitous Language** = dev + PO dùng cùng từ ngữ. Types = glossary.
- ✅ **Bounded Contexts** = ranh giới module. Cùng từ khác nghĩa. Anti-Corruption Layer.
- ✅ **Entities** = identity (ID). **Value Objects** = equality by value (immutable).
- ✅ **Domain Events** = business events, past tense. Trigger workflows.
- ✅ **Event Storming** = collaborative discovery → Types. Sticky notes = TypeScript DU.

## Tiếp theo

→ Chapter 19: **Functional Architecture** — Layered architecture, module boundaries, IO at edges, "Functional Core / Imperative Shell" revisited.
