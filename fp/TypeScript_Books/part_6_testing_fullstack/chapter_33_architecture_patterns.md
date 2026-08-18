# Chapter 33 — Architecture Patterns

> **Bạn sẽ học được**:
> - Hexagonal Architecture (Ports & Adapters) — giải phẫu một ứng dụng chuẩn mực trong TypeScript.
> - Functional Core / Imperative Shell — kỹ thuật tách pure logic khỏi side effects.
> - Clean Architecture layers — Tại sao Domain phải ở trung tâm?
> - Module boundaries — Sử dụng barrel exports để che giấu implementation details.
>
> **Yêu cầu trước**: Chapter 19 (Functional Architecture), Chapter 24 (Repository).
> **Thời gian đọc**: ~50 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Biết cách tổ chức hàng chục ngàn dòng code TypeScript thành các module độc lập, dễ test và dễ bảo trì.

---

Bạn biết ổ cắm điện tiêu chuẩn không? Bất kỳ thiết bị nào — quạt, tivi, máy giặt — đều cắm vào CÙNG một loại ổ cắm trên tường. Ổ cắm chính là **Port**. Phích cắm của cái quạt chính là **Adapter**.
Cái quạt KHÔNG CẦN BIẾT dòng điện nó xài được tạo ra từ nhà máy nhiệt điện hay thủy điện. Ổ cắm trên tường chính là một giao diện (interface) chia tách hoàn toàn hai thế giới.

**Hexagonal Architecture** (hay còn gọi là Ports & Adapters) mang triết lý y hệt vào thế giới phần mềm. 
**Domain** (Cốt lõi nghiệp vụ) nằm ở TRUNG TÂM và nó hoàn toàn "mù" với thế giới bên ngoài. Nó định nghĩa ra các "Ổ cắm" (Ports).
Còn Database, Web Server, API bên thứ ba đóng vai trò là các "Phích cắm" (Adapters). 

Bạn có thể rút phích cắm PostgreSQL ra, cắm phích cắm MongoDB vào, và Domain ở giữa vẫn chạy bình thường mà không cần sửa một dòng code nào!

## 33.1 — Giải phẫu Hexagonal Architecture

Quy tắc tối thượng của Hexagonal Architecture là **Dependency Rule** (Quy tắc phụ thuộc): *Mọi mũi tên phụ thuộc đều phải trỏ vào TRONG*.
Infrastructure (DB, Web) -> Application (Use Cases) -> Domain (Entities, Rules).
Domain KHÔNG BAO GIỜ được phép `import` bất cứ thứ gì từ Infrastructure.

Hãy cùng xây dựng một Module Đặt hàng (`Order`) hoàn chỉnh bằng TypeScript để minh họa.

### Lớp 1: Domain (Tâm lõi thuần khiết)
Ở tâm của lục giác, chúng ta chỉ có Type, Data, và Pure Functions. Không có `async`, không có `Promise`, không có `import "pg"` hay `import "express"`.

```typescript
// filename: src/modules/orders/domain/types.ts
export type OrderId = string & { __brand: "OrderId" };
export type Money = number & { __brand: "Money" };
export const Money = (n: number): Money => n as Money;

export type OrderItem = {
    readonly productId: string;
    readonly name: string;
    readonly quantity: number;
    readonly unitPrice: Money;
};

export type Order = {
    readonly id: OrderId;
    readonly items: readonly OrderItem[];
    readonly status: "draft" | "confirmed" | "cancelled";
    readonly total: Money;
};

export type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };

// filename: src/modules/orders/domain/rules.ts
// Domain logic: Thuần khiết (Pure functions), dễ dàng Unit Test mà không cần Mock
export const confirmOrder = (order: Order): Result<Order, string> =>
    order.status !== "draft"
        ? { tag: "err", error: `Cannot confirm ${order.status} order` }
        : { tag: "ok", value: { ...order, status: "confirmed" } };
```

### Lớp 1.5: Ports (Ổ cắm)
Domain không thể tự lưu chính nó vào Database, nên nó định nghĩa ra các "bản hợp đồng" (Interfaces/Types) và yêu cầu thế giới bên ngoài phải tuân theo. Chú ý: Các Port này thuộc về Domain, do Domain định nghĩa!

```typescript
// filename: src/modules/orders/ports/index.ts
import { Order, OrderId, Money, Result } from "../domain/types";

// DB Port
export type OrderRepository = {
    readonly findById: (id: OrderId) => Promise<Order | null>;
    readonly save: (order: Order) => Promise<void>;
};

// 3rd-party Port
export type PaymentGateway = {
    readonly charge: (orderId: OrderId, amount: Money) => Promise<Result<string, string>>;
};

// Email Port
export type NotificationService = {
    readonly sendConfirmation: (order: Order) => Promise<void>;
};
```

### Lớp 2: Application (Use Cases)
Lớp Application (hay Use Case) đóng vai trò nhạc trưởng (Orchestrator). Nó nhận các Port (được inject vào) và điều phối Domain Logic để hoàn thành một tác vụ của người dùng.

```typescript
// filename: src/modules/orders/application/confirm-order.ts
import { OrderId, Order, Result } from "../domain/types";
import { confirmOrder } from "../domain/rules";
import { OrderRepository, PaymentGateway, NotificationService } from "../ports";

// Dependencies được Inject vào (Dependency Injection)
export type AppDeps = {
    readonly orderRepo: OrderRepository;
    readonly payment: PaymentGateway;
    readonly notifications: NotificationService;
};

export const confirmOrderUseCase = async (
    deps: AppDeps,
    orderId: OrderId,
): Promise<Result<Order, string>> => {
    // 1. IO: Đọc từ Database qua Port
    const order = await deps.orderRepo.findById(orderId);
    if (!order) return { tag: "err", error: "Order not found" };

    // 2. Pure: Xác thực và thay đổi state (Domain Logic)
    const confirmed = confirmOrder(order);
    if (confirmed.tag === "err") return confirmed;

    // 3. IO: Trừ tiền qua Port
    const payment = await deps.payment.charge(orderId, order.total);
    if (payment.tag === "err") return { tag: "err", error: `Payment failed: ${payment.error}` };

    // 4. IO: Lưu vào Database qua Port
    await deps.orderRepo.save(confirmed.value);

    // 5. IO: Gửi Email (Fire and forget)
    await deps.notifications.sendConfirmation(confirmed.value);

    return confirmed;
};
```
Bạn thấy đấy, Use Case không hề biết Database là MySQL hay Redis. Nó chỉ biết giao tiếp qua `deps.orderRepo`. Đây chính là sức mạnh của sự trừu tượng.

### Lớp 3: Infrastructure (Adapters)
Lớp ngoài cùng chứa code "bẩn" — nơi giao tiếp trực tiếp với framework và I/O.
Vì chúng ta đã tách biệt bằng Ports, việc viết Unit Test cho Use Case cực kỳ dễ: Ta chỉ cần viết các "Fake Adapters" chạy trên RAM.

```typescript
// filename: src/modules/orders/infrastructure/fakes.ts
import { OrderRepository, PaymentGateway, NotificationService } from "../ports";
import { Order, OrderId } from "../domain/types";

// Fake Adapter: Cắm phích cắm "RAM" vào ổ cắm "OrderRepository"
export const createInMemoryOrderRepo = (orders: readonly Order[]): OrderRepository => {
    const store = new Map<OrderId, Order>();
    for (const o of orders) store.set(o.id, o);
    return {
        findById: async (id) => store.get(id) ?? null,
        save: async (order) => { store.set(order.id, order); },
    };
};

export const createFakePayment = (): PaymentGateway => ({
    charge: async (_id, _amount) => ({ tag: "ok", value: "PAY-001" }),
});
```

> **💡 Bạn có biết?**: Trong production thực tế, bạn sẽ có `PrismaOrderRepo` implement `OrderRepository`, và `StripePaymentAdapter` implement `PaymentGateway`. Bạn chỉ việc inject chúng vào `confirmOrderUseCase` ở file khởi động (`index.ts` hoặc `server.ts`).

---

## 33.2 — Functional Core / Imperative Shell

Nếu Hexagonal Architecture giải quyết cấu trúc thư mục, thì **Functional Core / Imperative Shell (FC/IS)** giải quyết cấu trúc của từng file, từng module.

Khái niệm này cực kỳ đơn giản:
1. **Core**: Nằm ở giữa, là các hàm Pure Function. Nhận Data, trả về Data. Dễ test kinh khủng.
2. **Shell**: Bao bọc bên ngoài. Chứa side-effects (IO, API call, DOM update). Nó lấy data từ thế giới thực, ném vào Core để xử lý, lấy kết quả từ Core, và đẩy lại ra thế giới thực.

Hãy xem ví dụ tính tiền giỏ hàng:

```typescript
// === FUNCTIONAL CORE (Pure - Dễ test) ===
type CartItem = { productId: string; quantity: number; price: number };
type Discount = { type: "percentage"; value: number } | { type: "fixed"; value: number };

const applyDiscount = (subtotal: number, discount: Discount): number =>
    discount.type === "percentage"
        ? subtotal * (1 - discount.value / 100)
        : Math.max(0, subtotal - discount.value);

const calculateSubtotal = (items: readonly CartItem[]): number =>
    items.reduce((sum, item) => sum + item.quantity * item.price, 0);

// Tính toán dựa trên Input, không quan tâm DB!
export const calculateCart = (items: readonly CartItem[], discount?: Discount) => {
    const subtotal = calculateSubtotal(items);
    const discounted = discount ? applyDiscount(subtotal, discount) : subtotal;
    const shipping = discounted >= 500000 ? 0 : 30000;
    return { subtotal, discounted, shipping, total: discounted + shipping };
};

// === IMPERATIVE SHELL (Dơ bẩn - Chứa IO) ===
// Hàm này là Shell, nó ôm đồm mọi thứ IO: gọi DB, gọi hàm tính toán, lưu DB
export const processCheckoutShell = async (cartId: string, db: Database, mailer: Mailer) => {
    // 1. Gather inputs (IO)
    const items = await db.getCartItems(cartId);      
    const discount = await db.getDiscountCode(cartId);
    
    // 2. Delegate to CORE (PURE!)
    const result = calculateCart(items, discount);     
    
    // 3. Apply outputs (IO)
    await db.saveOrder(cartId, result);                
    await mailer.sendReceipt(cartId, result);    
    return result;
};
```
Thay vì phải Mock Database để test hàm tính toán tổng tiền, bạn chỉ cần test hàm `calculateCart`. Hàm `processCheckoutShell` thường chỉ cần Integration Test để đảm bảo kết nối DB ổn định.

---

## 33.3 — Module Boundaries và Barrel Exports

Khi hệ thống lớn lên, bạn sẽ có hàng chục module (Orders, Users, Inventory). Nếu để chúng gọi chéo nhau tùy tiện, mã nguồn sẽ biến thành món mì Spaghetti.

Quy tắc bảo vệ biên giới Module: **Sử dụng Barrel Exports (`index.ts`)**.

```typescript
// src/modules/orders/index.ts (BARREL FILE)

// CHỈ EXPORT những gì module khác được phép dùng:
export type { Order, OrderId } from './domain/types';
export type { OrderRepository } from './ports/order-repository';
export { confirmOrderUseCase } from './application/confirm-order';

// TUYỆT ĐỐI KHÔNG EXPORT:
// export { confirmOrder } from './domain/rules'; -> Internal logic!
// export { PrismaOrderRepo } from './infrastructure/prisma'; -> Rò rỉ Infra!
```
Khi module `Users` muốn tương tác với `Orders`, nó BẮT BUỘC phải import từ `src/modules/orders`. Nó không được chọc sâu vào `src/modules/orders/domain/rules.ts`. Đây là cách Encapsulation (đóng gói) hoạt động trong TypeScript ở cấp độ kiến trúc.

---

---

## ✅ Checkpoint 33

1. Hexagonal Architecture và Functional Core / Imperative Shell — chúng bổ sung hay thay thế nhau?
2. Trong TypeScript, Port nên là `interface` hay `type`? Vì sao?
3. Làm sao **ép** ranh giới kiến trúc thay vì chỉ mong mọi người tôn trọng nó?

<details>
<summary>Đáp án</summary>

1. **Bổ sung**. Hexagonal nói về *hướng phụ thuộc* (mũi tên chỉ vào trong). Functional Core nói về *bản chất code* (thuần ở lõi, hiệu ứng ở vỏ). Bạn có thể hexagonal mà lõi vẫn đầy side effect — và lúc đó mất phần lớn lợi ích.
2. `interface` — nó hỗ trợ declaration merging và cho thông báo lỗi dễ đọc hơn khi không khớp. `type` hợp hơn cho union, mapped type và các phép biến đổi kiểu.
3. Bằng công cụ: `eslint-plugin-boundaries` hoặc `dependency-cruiser` trong CI, cộng với `paths` trong `tsconfig`. Ranh giới không được máy kiểm tra sẽ bị vi phạm trong vài sprint.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Vẽ sơ đồ hexagonal cho một API bạn từng viết. Đánh dấu mọi mũi tên đi từ trong ra ngoài — đó là các vi phạm.

**Bài 2 (15 phút).** Cấu hình `dependency-cruiser` chặn `domain/` import từ `infrastructure/`. Chứng minh CI đỏ khi cố tình vi phạm.

**Bài 3 (25 phút).** Refactor một hàm lẫn I/O và logic thành `core` thuần + `shell` mỏng. Đếm số dòng test cần viết trước và sau.

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Domain import Prisma type | Repository trả về type do Prisma sinh | Map sang domain type trong adapter |
| Circular import giữa các tầng | Phụ thuộc hai chiều | Đặt interface ở tầng trong, implement ở tầng ngoài |
| Barrel file (`index.ts`) gây import vòng | Re-export toàn bộ | Import trực tiếp từ module cụ thể |
| `import type` vẫn xuất hiện ở bundle | Dùng `import` thường cho type | Dùng `import type`; bật `verbatimModuleSyntax` |
| Quá nhiều tầng cho một CRUD nhỏ | Áp kiến trúc nặng cho bài toán nhẹ | Kiến trúc này dành cho domain phức tạp |

## Tóm tắt

- ✅ **Hexagonal** = Domain (Ở giữa) + Ports (Ổ cắm giao tiếp) + Adapters (Phích cắm thực thi). Thay Adapter (DB, Framework) không làm ảnh hưởng Domain.
- ✅ **Dependency Rule**: Mọi phụ thuộc trỏ vào trung tâm. Domain không import hạ tầng.
- ✅ **Functional Core / Imperative Shell**: Chia tách mã thuần khiết và mã dơ bẩn (Side-effects).
- ✅ **Module boundaries**: Dùng `index.ts` như một lớp giao diện API của thư mục, ẩn đi những chi tiết implementation bên trong.

## Tiếp theo

Bạn đã có một kiến trúc Backend hoàn hảo. Nhưng còn Frontend thì sao? React.js vốn dĩ rất gần với Functional Programming, nhưng đa số lập trình viên lại biến nó thành một đống bùng nhùng bằng `useEffect`.
Hãy cùng xem cách áp dụng FP vào Frontend trong **Chapter 35: React Functional Architecture**.
