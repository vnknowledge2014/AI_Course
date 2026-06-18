# Chapter 24 — Persistence & Repository Pattern

> **Bạn sẽ học được**:
> - Repository pattern — tách domain khỏi database
> - Interface-first design — domain định nghĩa, infrastructure implement
> - In-memory repository — test nhanh, không cần database
> - Prisma/Drizzle integration — typed database access trong TypeScript
> - Unit of Work — transaction boundary
> - FP Dependency Injection cho data access (từ Ch19)
>
> **Yêu cầu trước**: Chapter 19 (layered architecture, FP DI), Chapter 23 (DTOs, mapping).
> **Thời gian đọc**: ~50 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Data access tách biệt hoàn toàn — swap Prisma ↔ in-memory cho testing. Domain KHÔNG biết database.

---

Bạn biết thủ thư làm gì?

Bạn đến thư viện, nói: "Tôi cần sách về TypeScript". Thủ thư tìm trong hệ thống, đi vào kho, lấy sách, đưa cho bạn. Bạn KHÔNG biết sách ở kệ nào, tầng nào, mã Dewey bao nhiêu. Bạn cũng không quan tâm sách được lưu trên kệ gỗ hay trong tủ kính — bạn chỉ cần sách. Khi trả sách, bạn đưa cho thủ thư — thủ thư cất vào đúng chỗ.

**Repository pattern** = thủ thư cho data. Domain nói: "Tìm đơn hàng ORD-001". Repository đi vào "kho" (database, file, API, in-memory Map), tìm, trả về domain object. Domain KHÔNG biết kho dùng PostgreSQL hay SQLite hay `Map<string, Order>`. Interface = "khả năng của thủ thư" (`findById`, `save`, `delete`). Implementation = "thủ thư cụ thể ở thư viện nào" (Prisma, Drizzle, in-memory).

---

## 24.1 — Repository Interface: Domain Defines, Infrastructure Implements

### Domain layer định nghĩa giao diện

Theo **Dependency Rule** (Ch19), domain layer KHÔNG import infrastructure. Nhưng domain CẦN data — giải quyết thế nào? Domain định nghĩa **interface** (khế ước): "Tôi cần một thủ thư biết làm những việc này". Infrastructure **implement** interface đó.

```typescript
// filename: src/repository/types.ts
import assert from "node:assert/strict";

// === Domain types ===
type OrderId = string & { readonly __brand: "OrderId" };
type CustomerId = string & { readonly __brand: "CustomerId" };
type Money = number & { readonly __brand: "Money" };
const Money = (n: number): Money => n as Money;

type OrderStatus = "draft" | "confirmed" | "shipped" | "delivered" | "cancelled";

type Order = {
    readonly id: OrderId;
    readonly customerId: CustomerId;
    readonly items: readonly {
        readonly productId: string;
        readonly productName: string;
        readonly quantity: number;
        readonly unitPrice: Money;
    }[];
    readonly status: OrderStatus;
    readonly total: Money;
    readonly createdAt: Date;
};

// === Repository interface (defined in DOMAIN layer) ===
// Domain nói: "Tôi cần thủ thư biết làm những việc này"
type OrderRepository = {
    readonly findById: (id: OrderId) => Promise<Order | null>;
    readonly findByCustomer: (customerId: CustomerId) => Promise<readonly Order[]>;
    readonly findByStatus: (status: OrderStatus) => Promise<readonly Order[]>;
    readonly save: (order: Order) => Promise<void>;
    readonly delete: (id: OrderId) => Promise<boolean>;
};

// === Result type ===
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// === Domain service sử dụng repository (via DI) ===
type OrderError = "not_found" | "already_confirmed" | "empty_items";

const confirmOrder = async (
    repo: OrderRepository,   // DI: passed as parameter!
    orderId: OrderId,
): Promise<Result<Order, OrderError>> => {
    const order = await repo.findById(orderId);
    if (!order) return err("not_found");
    if (order.status !== "draft") return err("already_confirmed");

    const confirmed: Order = { ...order, status: "confirmed" };
    await repo.save(confirmed);
    return ok(confirmed);
};

// Test sẽ ở phần sau — cần implementation trước!
console.log("Repository interface OK ✅");
```

Đọc code trên: `confirmOrder` nhận `repo: OrderRepository` — một object với methods `findById` và `save`. Domain KHÔNG biết repo là Prisma, Drizzle, hay in-memory Map. Domain chỉ biết: "ai đó sẽ cung cấp thủ thư". Đây là **Dependency Inversion Principle** (DIP) — domain phụ thuộc vào abstraction, không phải implementation.

> **💡 Interface = contract**: `OrderRepository` nói "bất kỳ ai implement tôi phải cung cấp `findById`, `save`, v.v." Domain code against interface. Infrastructure fulfills interface. Swap implementation = swap thủ thư, sách không thay đổi.

---

## ✅ Checkpoint 24.1

> Đến đây bạn phải hiểu:
> 1. **Repository** = interface between domain and data storage
> 2. **Domain defines** the interface (what it needs)
> 3. **Infrastructure implements** the interface (how to get data)
> 4. **DI**: pass `repo: OrderRepository` as function parameter
>
> **Test nhanh**: `confirmOrder` import Prisma không?
> <details><summary>Đáp án</summary>**KHÔNG!** `confirmOrder` chỉ biết `OrderRepository` interface. Không import Prisma, không import database driver. Pure domain logic + abstract data access.</details>

---

## 24.2 — In-Memory Repository: Testing Without Database

### Thủ thư giả — nhớ hết trong đầu

Trước khi kết nối database thật, tạo "thủ thư giả" lưu trữ trong `Map`. Nhanh, đơn giản, hoàn hảo cho testing.

```typescript
// filename: src/repository/in_memory.ts
import assert from "node:assert/strict";

// Types (from 24.1)
type OrderId = string & { readonly __brand: "OrderId" };
type CustomerId = string & { readonly __brand: "CustomerId" };
type Money = number & { readonly __brand: "Money" };
const Money = (n: number): Money => n as Money;

type OrderStatus = "draft" | "confirmed" | "shipped" | "delivered" | "cancelled";

type Order = {
    readonly id: OrderId;
    readonly customerId: CustomerId;
    readonly items: readonly {
        readonly productId: string;
        readonly productName: string;
        readonly quantity: number;
        readonly unitPrice: Money;
    }[];
    readonly status: OrderStatus;
    readonly total: Money;
    readonly createdAt: Date;
};

type OrderRepository = {
    readonly findById: (id: OrderId) => Promise<Order | null>;
    readonly findByCustomer: (customerId: CustomerId) => Promise<readonly Order[]>;
    readonly findByStatus: (status: OrderStatus) => Promise<readonly Order[]>;
    readonly save: (order: Order) => Promise<void>;
    readonly delete: (id: OrderId) => Promise<boolean>;
};

// === In-Memory Implementation ===
const createInMemoryOrderRepo = (
    initial: readonly Order[] = [],
): OrderRepository => {
    const store = new Map<OrderId, Order>();
    for (const order of initial) {
        store.set(order.id, order);
    }

    return {
        findById: async (id) => store.get(id) ?? null,

        findByCustomer: async (customerId) =>
            [...store.values()].filter(o => o.customerId === customerId),

        findByStatus: async (status) =>
            [...store.values()].filter(o => o.status === status),

        save: async (order) => { store.set(order.id, order); },

        delete: async (id) => store.delete(id),
    };
};

// === Test domain logic with in-memory repo ===
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

type OrderError = "not_found" | "already_confirmed" | "empty_items";

const confirmOrder = async (
    repo: OrderRepository,
    orderId: OrderId,
): Promise<Result<Order, OrderError>> => {
    const order = await repo.findById(orderId);
    if (!order) return err("not_found");
    if (order.status !== "draft") return err("already_confirmed");

    const confirmed: Order = { ...order, status: "confirmed" };
    await repo.save(confirmed);
    return ok(confirmed);
};

// --- Tests ---
const run = async () => {
    const draftOrder: Order = {
        id: "ORD-1" as OrderId,
        customerId: "C-1" as CustomerId,
        items: [{ productId: "P1", productName: "Laptop", quantity: 1, unitPrice: Money(20000000) }],
        status: "draft",
        total: Money(20000000),
        createdAt: new Date("2024-06-15"),
    };

    const repo = createInMemoryOrderRepo([draftOrder]);

    // Test: confirm draft order
    const result = await confirmOrder(repo, "ORD-1" as OrderId);
    assert.strictEqual(result.tag, "ok");
    if (result.tag === "ok") {
        assert.strictEqual(result.value.status, "confirmed");
    }

    // Verify persistence
    const saved = await repo.findById("ORD-1" as OrderId);
    assert.ok(saved !== null);
    assert.strictEqual(saved!.status, "confirmed");  // persisted!

    // Test: confirm again = error
    const double = await confirmOrder(repo, "ORD-1" as OrderId);
    assert.strictEqual(double.tag, "err");
    if (double.tag === "err") {
        assert.strictEqual(double.error, "already_confirmed");
    }

    // Test: not found
    const notFound = await confirmOrder(repo, "ORD-999" as OrderId);
    assert.strictEqual(notFound.tag, "err");
    if (notFound.tag === "err") {
        assert.strictEqual(notFound.error, "not_found");
    }

    // Test: findByStatus
    const drafts = await repo.findByStatus("draft");
    assert.strictEqual(drafts.length, 0);  // was draft, now confirmed

    const confirmed = await repo.findByStatus("confirmed");
    assert.strictEqual(confirmed.length, 1);

    console.log("In-memory repo OK ✅");
};

run();
```

Không có database. Không có Prisma. Không có connection string. Chỉ `Map<OrderId, Order>` — và domain logic được test hoàn chỉnh. `confirmOrder` KHÔNG BIẾT nó đang test — nó gọi `repo.findById()`, `repo.save()` bình thường. In-memory repo = perfect test double.

> **💡 In-memory repo = first-class test strategy**: Test domain logic TRƯỚC khi chọn database. Test chạy < 1ms (no I/O). Feedback loop cực nhanh. Database choice = infrastructure decision — quyết định SAU.

---

## ✅ Checkpoint 24.2

> Đến đây bạn phải hiểu:
> 1. **In-memory repo** = `Map` implements `OrderRepository` interface
> 2. **Domain code unchanged** — `confirmOrder` works with any repo implementation
> 3. **Tests run without database** — fast, no setup, no teardown
> 4. **Object literal as test double** — no mock library needed (Ch19 FP DI)
>
> **Test nhanh**: Thay in-memory bằng Prisma — `confirmOrder` phải sửa gì?
> <details><summary>Đáp án</summary>**KHÔNG SỬA GÌ!** `confirmOrder` nhận `OrderRepository` interface. Swap `createInMemoryOrderRepo()` → `createPrismaOrderRepo()` tại call site. Domain code = zero changes.</details>

---

## 24.3 — Database Integration: Prisma & Drizzle

### Thủ thư thật — kết nối kho sách

Production cần database thật. Prisma = type-safe ORM (generated types from schema). Drizzle = SQL-like query builder. Cả hai implement cùng `OrderRepository` interface — domain không biết cái nào đang chạy.

```typescript
// filename: src/repository/prisma_example.ts
import assert from "node:assert/strict";

// Types
type OrderId = string & { readonly __brand: "OrderId" };
type CustomerId = string & { readonly __brand: "CustomerId" };
type Money = number & { readonly __brand: "Money" };
const Money = (n: number): Money => n as Money;
type OrderStatus = "draft" | "confirmed" | "shipped" | "delivered" | "cancelled";

type Order = {
    readonly id: OrderId;
    readonly customerId: CustomerId;
    readonly items: readonly {
        readonly productId: string;
        readonly productName: string;
        readonly quantity: number;
        readonly unitPrice: Money;
    }[];
    readonly status: OrderStatus;
    readonly total: Money;
    readonly createdAt: Date;
};

type OrderRepository = {
    readonly findById: (id: OrderId) => Promise<Order | null>;
    readonly findByCustomer: (customerId: CustomerId) => Promise<readonly Order[]>;
    readonly findByStatus: (status: OrderStatus) => Promise<readonly Order[]>;
    readonly save: (order: Order) => Promise<void>;
    readonly delete: (id: OrderId) => Promise<boolean>;
};

// === Prisma-style implementation (conceptual) ===
// In production: import { PrismaClient } from '@prisma/client'

// Simulating Prisma client interface
type PrismaOrder = {
    id: string;
    customer_id: string;
    status: string;
    total: number;
    created_at: Date;
    items: readonly {
        product_id: string;
        product_name: string;
        quantity: number;
        unit_price: number;
    }[];
};

type PrismaClient = {
    order: {
        findUnique: (args: { where: { id: string } }) => Promise<PrismaOrder | null>;
        findMany: (args: { where: Record<string, unknown> }) => Promise<PrismaOrder[]>;
        upsert: (args: { where: { id: string }; create: unknown; update: unknown }) => Promise<PrismaOrder>;
        delete: (args: { where: { id: string } }) => Promise<PrismaOrder>;
    };
};

// Mappers: Prisma ↔ Domain (from Ch23)
const prismaToDomain = (row: PrismaOrder): Order => ({
    id: row.id as OrderId,
    customerId: row.customer_id as CustomerId,
    items: row.items.map(i => ({
        productId: i.product_id,
        productName: i.product_name,
        quantity: i.quantity,
        unitPrice: Money(i.unit_price),
    })),
    status: row.status as OrderStatus,
    total: Money(row.total),
    createdAt: row.created_at,
});

const domainToPrisma = (order: Order): PrismaOrder => ({
    id: order.id,
    customer_id: order.customerId,
    status: order.status,
    total: order.total,
    created_at: order.createdAt,
    items: order.items.map(i => ({
        product_id: i.productId,
        product_name: i.productName,
        quantity: i.quantity,
        unit_price: i.unitPrice,
    })),
});

// Factory: create Prisma repository
const createPrismaOrderRepo = (prisma: PrismaClient): OrderRepository => ({
    findById: async (id) => {
        const row = await prisma.order.findUnique({ where: { id } });
        return row ? prismaToDomain(row) : null;
    },

    findByCustomer: async (customerId) => {
        const rows = await prisma.order.findMany({ where: { customer_id: customerId } });
        return rows.map(prismaToDomain);
    },

    findByStatus: async (status) => {
        const rows = await prisma.order.findMany({ where: { status } });
        return rows.map(prismaToDomain);
    },

    save: async (order) => {
        const data = domainToPrisma(order);
        await prisma.order.upsert({
            where: { id: order.id },
            create: data,
            update: data,
        });
    },

    delete: async (id) => {
        try {
            await prisma.order.delete({ where: { id } });
            return true;
        } catch {
            return false;
        }
    },
});

// Key insight: domain code is IDENTICAL whether using Prisma, Drizzle, or in-memory
// const repo = createPrismaOrderRepo(prismaClient);
// const result = await confirmOrder(repo, orderId);  // same function!

console.log("Prisma repo concept OK ✅");
```

### Pattern: Repository factory

```typescript
// filename: src/repository/factory.ts
import assert from "node:assert/strict";

// Generic repository interface
type Repository<Id, Entity> = {
    readonly findById: (id: Id) => Promise<Entity | null>;
    readonly findAll: () => Promise<readonly Entity[]>;
    readonly save: (entity: Entity) => Promise<void>;
    readonly delete: (id: Id) => Promise<boolean>;
};

// Generic in-memory factory
const createInMemoryRepo = <Id extends string, Entity extends { readonly id: Id }>(
    initial: readonly Entity[] = [],
): Repository<Id, Entity> => {
    const store = new Map<Id, Entity>();
    for (const entity of initial) store.set(entity.id, entity);

    return {
        findById: async (id) => store.get(id) ?? null,
        findAll: async () => [...store.values()],
        save: async (entity) => { store.set(entity.id, entity); },
        delete: async (id) => store.delete(id),
    };
};

// Use for any entity
type Product = { readonly id: string; readonly name: string; readonly price: number };

const productRepo = createInMemoryRepo<string, Product>([
    { id: "P1", name: "Laptop", price: 20000000 },
    { id: "P2", name: "Mouse", price: 500000 },
]);

const run = async () => {
    const laptop = await productRepo.findById("P1");
    assert.ok(laptop !== null);
    assert.strictEqual(laptop!.name, "Laptop");

    const all = await productRepo.findAll();
    assert.strictEqual(all.length, 2);

    await productRepo.save({ id: "P3", name: "Keyboard", price: 2000000 });
    const all2 = await productRepo.findAll();
    assert.strictEqual(all2.length, 3);

    console.log("Generic repo factory OK ✅");
};

run();
```

> **💡 Generic repository**: `Repository<Id, Entity>` = base interface. Extend with domain-specific queries: `OrderRepository extends Repository<OrderId, Order>` + `findByStatus`, `findByCustomer`.

---

## ✅ Checkpoint 24.3

> Đến đây bạn phải hiểu:
> 1. **Prisma repo** = `createPrismaOrderRepo(prismaClient)` implements same interface
> 2. **Mappers**: `prismaToDomain` (DB → Domain), `domainToPrisma` (Domain → DB)
> 3. **Factory pattern**: function creates repo from client/config
> 4. **Generic repo**: `Repository<Id, Entity>` = reusable base for any entity
>
> **Test nhanh**: Chuyển từ Prisma sang Drizzle — domain code sửa gì?
> <details><summary>Đáp án</summary>**KHÔNG SỬA GÌ!** Viết `createDrizzleOrderRepo(drizzleDb)` implement cùng `OrderRepository` interface. Domain code gọi `repo.findById()` — không biết Prisma hay Drizzle.</details>

---

## 24.4 — Unit of Work: Transaction Boundary

### Nhiều thủ thư, một phiên làm việc

Đôi khi bạn cần thay đổi NHIỀU "kệ sách" cùng lúc: tạo order + giảm stock + ghi log. Nếu giảm stock thành công nhưng tạo order thất bại → inconsistent! **Unit of Work** (UoW) đảm bảo: tất cả thay đổi commit cùng lúc hoặc rollback cùng lúc. Trong database = transaction. Trong code = gom repositories vào 1 unit.

```typescript
// filename: src/repository/unit_of_work.ts
import assert from "node:assert/strict";

// Domain types
type OrderId = string & { readonly __brand: "OrderId" };
type ProductId = string & { readonly __brand: "ProductId" };
type Money = number & { readonly __brand: "Money" };
const Money = (n: number): Money => n as Money;

type Order = {
    readonly id: OrderId;
    readonly customerId: string;
    readonly status: "draft" | "confirmed";
    readonly total: Money;
};

type Product = {
    readonly id: ProductId;
    readonly name: string;
    readonly stock: number;
    readonly price: Money;
};

// Individual repositories
type OrderRepository = {
    readonly findById: (id: OrderId) => Promise<Order | null>;
    readonly save: (order: Order) => Promise<void>;
};

type ProductRepository = {
    readonly findById: (id: ProductId) => Promise<Product | null>;
    readonly save: (product: Product) => Promise<void>;
};

// Unit of Work = groups repositories + commit/rollback
type UnitOfWork = {
    readonly orders: OrderRepository;
    readonly products: ProductRepository;
    readonly commit: () => Promise<void>;
    readonly rollback: () => Promise<void>;
};

// === In-memory UoW (for testing) ===
const createInMemoryUow = (
    initialOrders: readonly Order[] = [],
    initialProducts: readonly Product[] = [],
): UnitOfWork => {
    // "Committed" state
    const committedOrders = new Map<OrderId, Order>();
    const committedProducts = new Map<ProductId, Product>();

    for (const o of initialOrders) committedOrders.set(o.id, o);
    for (const p of initialProducts) committedProducts.set(p.id, p);

    // "Pending" state (uncommitted changes)
    const pendingOrders = new Map<OrderId, Order>(committedOrders);
    const pendingProducts = new Map<ProductId, Product>(committedProducts);

    return {
        orders: {
            findById: async (id) => pendingOrders.get(id) ?? null,
            save: async (order) => { pendingOrders.set(order.id, order); },
        },
        products: {
            findById: async (id) => pendingProducts.get(id) ?? null,
            save: async (product) => { pendingProducts.set(product.id, product); },
        },
        commit: async () => {
            // Copy pending → committed
            for (const [k, v] of pendingOrders) committedOrders.set(k, v);
            for (const [k, v] of pendingProducts) committedProducts.set(k, v);
        },
        rollback: async () => {
            // Revert pending ← committed
            pendingOrders.clear();
            for (const [k, v] of committedOrders) pendingOrders.set(k, v);
            pendingProducts.clear();
            for (const [k, v] of committedProducts) pendingProducts.set(k, v);
        },
    };
};

// === Domain workflow using UoW ===

type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

type PlaceOrderError = "product_not_found" | "insufficient_stock" | "save_failed";

const placeOrder = async (
    uow: UnitOfWork,
    orderId: OrderId,
    productId: ProductId,
    quantity: number,
): Promise<Result<Order, PlaceOrderError>> => {
    // Step 1: Find product
    const product = await uow.products.findById(productId);
    if (!product) return err("product_not_found");

    // Step 2: Check stock
    if (product.stock < quantity) return err("insufficient_stock");

    // Step 3: Create order
    const order: Order = {
        id: orderId,
        customerId: "C-1",
        status: "confirmed",
        total: Money(product.price * quantity),
    };

    // Step 4: Reduce stock
    const updatedProduct: Product = {
        ...product,
        stock: product.stock - quantity,
    };

    // Save both atomically
    await uow.orders.save(order);
    await uow.products.save(updatedProduct);
    await uow.commit();  // both or nothing!

    return ok(order);
};

// === Test ===
const run = async () => {
    const uow = createInMemoryUow(
        [],
        [{ id: "P1" as ProductId, name: "Laptop", stock: 5, price: Money(20000000) }],
    );

    // Place order
    const result = await placeOrder(uow, "ORD-1" as OrderId, "P1" as ProductId, 2);
    assert.strictEqual(result.tag, "ok");

    // Verify: stock reduced
    const product = await uow.products.findById("P1" as ProductId);
    assert.ok(product !== null);
    assert.strictEqual(product!.stock, 3);  // 5 - 2 = 3

    // Verify: order saved
    const order = await uow.orders.findById("ORD-1" as OrderId);
    assert.ok(order !== null);
    assert.strictEqual(order!.total, 40000000);  // 20M * 2

    // Test: insufficient stock
    const badResult = await placeOrder(uow, "ORD-2" as OrderId, "P1" as ProductId, 10);
    assert.strictEqual(badResult.tag, "err");
    if (badResult.tag === "err") {
        assert.strictEqual(badResult.error, "insufficient_stock");
    }

    console.log("Unit of Work OK ✅");
};

run();
```

> **💡 Unit of Work = transaction boundary**: `commit()` = all changes saved. `rollback()` = all changes reverted. In-memory UoW cho testing, Prisma `$transaction()` cho production. Domain logic KHÔNG biết mechanism — chỉ gọi `commit()`.

---

## ✅ Checkpoint 24.4

> Đến đây bạn phải hiểu:
> 1. **Unit of Work** = group repositories + commit/rollback
> 2. **Atomic operations**: order + stock update = commit together or rollback together
> 3. **In-memory UoW**: pending Map + committed Map. Commit copies pending → committed
> 4. **Production UoW**: wraps `prisma.$transaction()` or SQL `BEGIN/COMMIT/ROLLBACK`
>
> **Test nhanh**: `placeOrder` gọi `uow.commit()` — nếu không gọi thì sao?
> <details><summary>Đáp án</summary>Changes ở pending state, CHƯA persisted. Nếu process crash → mất hết. `commit()` = flush to database.</details>

---

## 24.5 — Full Application Wiring

### Nối tất cả lại — hải quan, thủ thư, đầu bếp

Đây là nơi mọi thứ kết hợp: domain logic (pure), repository (interface), in-memory repo (test), Prisma repo (production), và application layer (wiring). Pattern từ Ch19 (Functional Architecture) hiện thực hóa.

```typescript
// filename: src/application/wiring.ts
import assert from "node:assert/strict";

// === Domain Types ===
type UserId = string & { readonly __brand: "UserId" };
type Email = string & { readonly __brand: "Email" };

type User = {
    readonly id: UserId;
    readonly name: string;
    readonly email: Email;
    readonly isActive: boolean;
};

// === Repository Interface ===
type UserRepository = {
    readonly findById: (id: UserId) => Promise<User | null>;
    readonly findByEmail: (email: Email) => Promise<User | null>;
    readonly save: (user: User) => Promise<void>;
};

// === Notification Service Interface ===
type NotificationService = {
    readonly sendWelcome: (user: User) => Promise<void>;
};

// === Application Dependencies ===
type AppDeps = {
    readonly userRepo: UserRepository;
    readonly notifications: NotificationService;
    readonly generateId: () => UserId;
    readonly now: () => Date;
};

// === Domain Logic (pure) ===
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

type RegisterError = "invalid_email" | "duplicate_email";

const validateEmail = (raw: string): Result<Email, RegisterError> =>
    raw.includes("@") && raw.includes(".")
        ? ok(raw.toLowerCase().trim() as Email)
        : err("invalid_email");

// === Application Use Case (shell — orchestrates) ===
const registerUser = async (
    deps: AppDeps,
    name: string,
    rawEmail: string,
): Promise<Result<User, RegisterError>> => {
    // Step 1: Validate (pure)
    const emailResult = validateEmail(rawEmail);
    if (emailResult.tag === "err") return emailResult;

    // Step 2: Check duplicate (IO)
    const existing = await deps.userRepo.findByEmail(emailResult.value);
    if (existing) return err("duplicate_email");

    // Step 3: Create user (pure)
    const user: User = {
        id: deps.generateId(),
        name: name.trim(),
        email: emailResult.value,
        isActive: true,
    };

    // Step 4: Save (IO)
    await deps.userRepo.save(user);

    // Step 5: Send notification (IO)
    await deps.notifications.sendWelcome(user);

    return ok(user);
};

// === Test with fakes ===
const run = async () => {
    const sentEmails: string[] = [];
    let idCounter = 0;

    const testDeps: AppDeps = {
        userRepo: (() => {
            const store = new Map<UserId, User>();
            const emailIndex = new Map<Email, User>();
            return {
                findById: async (id) => store.get(id) ?? null,
                findByEmail: async (email) => emailIndex.get(email) ?? null,
                save: async (user) => {
                    store.set(user.id, user);
                    emailIndex.set(user.email, user);
                },
            };
        })(),
        notifications: {
            sendWelcome: async (user) => { sentEmails.push(user.email); },
        },
        generateId: () => `U-${++idCounter}` as UserId,
        now: () => new Date("2024-06-15"),
    };

    // Test: register new user
    const result = await registerUser(testDeps, "An", "an@mail.com");
    assert.strictEqual(result.tag, "ok");
    if (result.tag === "ok") {
        assert.strictEqual(result.value.name, "An");
        assert.strictEqual(result.value.email, "an@mail.com");
    }

    // Verify: email sent
    assert.strictEqual(sentEmails.length, 1);
    assert.strictEqual(sentEmails[0], "an@mail.com");

    // Test: duplicate email
    const dup = await registerUser(testDeps, "Bình", "an@mail.com");
    assert.strictEqual(dup.tag, "err");
    if (dup.tag === "err") {
        assert.strictEqual(dup.error, "duplicate_email");
    }

    // Test: invalid email
    const bad = await registerUser(testDeps, "Cường", "not-email");
    assert.strictEqual(bad.tag, "err");

    console.log("Full wiring OK ✅");
};

run();
```

Đọc lại flow: `registerUser(deps, name, email)` — nhận **tất cả dependencies** qua `deps` parameter. Test = tạo `testDeps` với in-memory repo + fake notification. Production = tạo `prodDeps` với Prisma repo + real email service. `registerUser` KHÔNG BIẾT — nó chỉ gọi `deps.userRepo.save()`, `deps.notifications.sendWelcome()`.

> **💡 Full circle**: Ch19 (FP DI) + Ch23 (DTOs) + Ch24 (Repository) = complete architecture. Domain = pure. Infrastructure = pluggable. Testing = fast. This is Functional Architecture in practice.

---

## ✅ Checkpoint 24.5

> Đến đây bạn phải hiểu:
> 1. **AppDeps** = all dependencies in one object (FP DI from Ch19)
> 2. **Use case** = function receiving deps. Orchestrates pure + IO steps
> 3. **Test with fakes**: in-memory repo + mock notification = full test, no real DB/email
> 4. **Production**: swap testDeps → prodDeps. Same use case code.
>
> **Test nhanh**: `registerUser` cần biết email service là SendGrid hay Mailgun không?
> <details><summary>Đáp án</summary>**KHÔNG!** `registerUser` chỉ biết `notifications.sendWelcome(user)`. SendGrid/Mailgun = implementation detail, hidden behind interface.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): In-memory repository

```typescript
// Viết in-memory repository cho `Product`:
// type Product = { id: ProductId; name: string; price: Money; category: string }
// Methods: findById, findByCategory, save, delete
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type ProductId = string & { readonly __brand: "ProductId" };
type Money = number & { readonly __brand: "Money" };
const Money = (n: number): Money => n as Money;

type Product = {
    readonly id: ProductId;
    readonly name: string;
    readonly price: Money;
    readonly category: string;
};

type ProductRepository = {
    readonly findById: (id: ProductId) => Promise<Product | null>;
    readonly findByCategory: (category: string) => Promise<readonly Product[]>;
    readonly save: (product: Product) => Promise<void>;
    readonly delete: (id: ProductId) => Promise<boolean>;
};

const createInMemoryProductRepo = (initial: readonly Product[] = []): ProductRepository => {
    const store = new Map<ProductId, Product>();
    for (const p of initial) store.set(p.id, p);

    return {
        findById: async (id) => store.get(id) ?? null,
        findByCategory: async (cat) => [...store.values()].filter(p => p.category === cat),
        save: async (product) => { store.set(product.id, product); },
        delete: async (id) => store.delete(id),
    };
};

const run = async () => {
    const repo = createInMemoryProductRepo([
        { id: "P1" as ProductId, name: "Laptop", price: Money(20000000), category: "electronics" },
        { id: "P2" as ProductId, name: "TypeScript Book", price: Money(200000), category: "books" },
    ]);

    const laptop = await repo.findById("P1" as ProductId);
    assert.ok(laptop !== null);
    assert.strictEqual(laptop!.name, "Laptop");

    const electronics = await repo.findByCategory("electronics");
    assert.strictEqual(electronics.length, 1);

    await repo.delete("P1" as ProductId);
    const deleted = await repo.findById("P1" as ProductId);
    assert.strictEqual(deleted, null);

    console.log("Product repo OK ✅");
};

run();
```

</details>

---

**Bài 2** (10 phút): Unit of Work

```typescript
// Viết UoW cho "Transfer Money" use case:
// Transfer từ Account A → Account B:
// 1. Deduct from A
// 2. Add to B
// 3. Commit both atomically
// Test: happy + insufficient funds
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type AccountId = string & { readonly __brand: "AccountId" };
type Money = number & { readonly __brand: "Money" };
const Money = (n: number): Money => n as Money;

type Account = {
    readonly id: AccountId;
    readonly name: string;
    readonly balance: Money;
};

type AccountRepository = {
    readonly findById: (id: AccountId) => Promise<Account | null>;
    readonly save: (account: Account) => Promise<void>;
};

type TransferUow = {
    readonly accounts: AccountRepository;
    readonly commit: () => Promise<void>;
};

const createInMemoryTransferUow = (initial: readonly Account[]): TransferUow => {
    const committed = new Map<AccountId, Account>();
    const pending = new Map<AccountId, Account>();
    for (const a of initial) { committed.set(a.id, a); pending.set(a.id, a); }

    return {
        accounts: {
            findById: async (id) => pending.get(id) ?? null,
            save: async (account) => { pending.set(account.id, account); },
        },
        commit: async () => { for (const [k, v] of pending) committed.set(k, v); },
    };
};

type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };
const ok = <T>(v: T): Result<T, never> => ({ tag: "ok", value: v });
const err = <E>(e: E): Result<never, E> => ({ tag: "err", error: e });

type TransferError = "account_not_found" | "insufficient_funds";

const transferMoney = async (
    uow: TransferUow,
    fromId: AccountId,
    toId: AccountId,
    amount: Money,
): Promise<Result<{ from: Account; to: Account }, TransferError>> => {
    const from = await uow.accounts.findById(fromId);
    if (!from) return err("account_not_found");

    const to = await uow.accounts.findById(toId);
    if (!to) return err("account_not_found");

    if (from.balance < amount) return err("insufficient_funds");

    const updatedFrom: Account = { ...from, balance: Money(from.balance - amount) };
    const updatedTo: Account = { ...to, balance: Money(to.balance + amount) };

    await uow.accounts.save(updatedFrom);
    await uow.accounts.save(updatedTo);
    await uow.commit();

    return ok({ from: updatedFrom, to: updatedTo });
};

const run = async () => {
    const uow = createInMemoryTransferUow([
        { id: "A1" as AccountId, name: "An", balance: Money(10000000) },
        { id: "A2" as AccountId, name: "Bình", balance: Money(5000000) },
    ]);

    const result = await transferMoney(uow, "A1" as AccountId, "A2" as AccountId, Money(3000000));
    assert.strictEqual(result.tag, "ok");
    if (result.tag === "ok") {
        assert.strictEqual(result.value.from.balance, 7000000);
        assert.strictEqual(result.value.to.balance, 8000000);
    }

    const bad = await transferMoney(uow, "A1" as AccountId, "A2" as AccountId, Money(50000000));
    assert.strictEqual(bad.tag, "err");

    console.log("Transfer UoW OK ✅");
};

run();
```

</details>

---

**Bài 3** (15 phút): Full application wiring

```typescript
// Viết full use case "Create Course" cho LMS:
// Dependencies: CourseRepository, InstructorRepository, NotificationService
// Steps:
// 1. Validate instructor exists + is active
// 2. Validate course data (title 5-200 chars, price >= 0)
// 3. Save course
// 4. Notify instructor
// Test with in-memory repos + fake notification
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type CourseId = string & { readonly __brand: "CourseId" };
type InstructorId = string & { readonly __brand: "InstructorId" };

type Course = {
    readonly id: CourseId;
    readonly title: string;
    readonly instructorId: InstructorId;
    readonly price: number;
    readonly publishedAt: Date;
};

type Instructor = {
    readonly id: InstructorId;
    readonly name: string;
    readonly isActive: boolean;
};

type CourseRepo = {
    readonly save: (course: Course) => Promise<void>;
};

type InstructorRepo = {
    readonly findById: (id: InstructorId) => Promise<Instructor | null>;
};

type Notifications = {
    readonly notifyNewCourse: (instructor: Instructor, course: Course) => Promise<void>;
};

type Deps = {
    readonly courses: CourseRepo;
    readonly instructors: InstructorRepo;
    readonly notifications: Notifications;
    readonly generateId: () => CourseId;
    readonly now: () => Date;
};

type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };
const ok = <T>(v: T): Result<T, never> => ({ tag: "ok", value: v });
const err = <E>(e: E): Result<never, E> => ({ tag: "err", error: e });

type CreateError = "instructor_not_found" | "instructor_inactive" | "invalid_title" | "invalid_price";

const createCourse = async (
    deps: Deps,
    instructorId: InstructorId,
    title: string,
    price: number,
): Promise<Result<Course, CreateError>> => {
    // Validate instructor
    const instructor = await deps.instructors.findById(instructorId);
    if (!instructor) return err("instructor_not_found");
    if (!instructor.isActive) return err("instructor_inactive");

    // Validate course data
    if (title.trim().length < 5 || title.length > 200) return err("invalid_title");
    if (price < 0) return err("invalid_price");

    // Create
    const course: Course = {
        id: deps.generateId(),
        title: title.trim(),
        instructorId,
        price,
        publishedAt: deps.now(),
    };

    await deps.courses.save(course);
    await deps.notifications.notifyNewCourse(instructor, course);

    return ok(course);
};

// Test
const run = async () => {
    const savedCourses: Course[] = [];
    const notifications: string[] = [];
    let idCounter = 0;

    const testDeps: Deps = {
        courses: { save: async (c) => { savedCourses.push(c); } },
        instructors: {
            findById: async (id) => {
                const instructors: Record<string, Instructor> = {
                    "I1": { id: "I1" as InstructorId, name: "An", isActive: true },
                    "I2": { id: "I2" as InstructorId, name: "Bình", isActive: false },
                };
                return instructors[id] ?? null;
            },
        },
        notifications: {
            notifyNewCourse: async (i, c) => { notifications.push(`${i.name}: ${c.title}`); },
        },
        generateId: () => `CRS-${++idCounter}` as CourseId,
        now: () => new Date("2024-06-15"),
    };

    // Happy path
    const result = await createCourse(testDeps, "I1" as InstructorId, "TypeScript Masterclass", 500000);
    assert.strictEqual(result.tag, "ok");
    assert.strictEqual(savedCourses.length, 1);
    assert.strictEqual(notifications.length, 1);

    // Inactive instructor
    const inactive = await createCourse(testDeps, "I2" as InstructorId, "Python Basics", 300000);
    assert.strictEqual(inactive.tag, "err");

    // Invalid title
    const badTitle = await createCourse(testDeps, "I1" as InstructorId, "TS", 500000);
    assert.strictEqual(badTitle.tag, "err");

    console.log("Create course OK ✅");
};

run();
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Repository có quá nhiều methods" | God interface | Split: `OrderReadRepo` (queries) + `OrderWriteRepo` (mutations). CQRS! |
| "In-memory repo quá đơn giản" | Missing edge cases | Add errors: throw on save failure, simulate latency, test concurrent access |
| "Domain imports Prisma" | Layer violation | Domain defines interface. Infrastructure imports Prisma + implements interface |
| "Transaction across services" | Distributed transaction | Saga pattern (Ch41). Or event-driven eventual consistency |
| "Map doesn't persist" | In-memory = volatile | That's the point for testing! Production = Prisma/DB |
| "DTO → Domain mapping duplicated" | Each repo mapper repeats | Extract shared mappers module. Or use generic mapper factory |

---

## Tóm tắt

Chương này hoàn thiện vòng tròn kiến trúc — thủ thư (repository) nối domain và infrastructure, Unit of Work đảm bảo atomicity, và application wiring gom tất cả lại.

- ✅ **Repository** = interface defined by domain, implemented by infrastructure.
- ✅ **In-memory repo** = `Map`-based. Test domain logic without database.
- ✅ **Prisma/Drizzle repo** = same interface, real database. Swap at wiring time.
- ✅ **Generic repo factory** = `Repository<Id, Entity>` for any entity type.
- ✅ **Unit of Work** = group repos + commit/rollback. Atomic multi-repo operations.
- ✅ **Full wiring** = AppDeps object with repos + services + helpers. FP DI from Ch19.
- ✅ **Testing** = fakes over mocks. Object literals as test doubles.

---

## 🎉 Part IV Complete!

Bạn đã hoàn thành Part IV: Domain-Driven Design. Từ DDD vocabulary (Ch18) qua Functional Architecture (Ch19), Domain Modeling (Ch20), Workflows (Ch21), Error Handling (Ch22), Serialization (Ch23), đến Persistence (Ch24). Đây là toolkit hoàn chỉnh cho TypeScript application architecture.

## Tiếp theo

→ Part V: **FP Patterns with fp-ts / Effect** — Chapter 25: Introduction to `fp-ts` / `Effect`
