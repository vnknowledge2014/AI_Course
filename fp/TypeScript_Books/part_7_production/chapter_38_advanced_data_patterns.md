# Chapter 38 — Advanced Data Patterns

> **Bạn sẽ học được**:
> - Database migrations — evolve schema safely
> - CQRS — tách read vs write models cho performance
> - Event Sourcing — lưu SỰ KIỆN, không lưu state
> - NoSQL: MongoDB, Redis, DynamoDB — khi nào dùng gì
> - Caching strategies — write-through, write-back, cache-aside
>
> **Yêu cầu trước**: Chapter 37 (Database fundamentals).
> **Thời gian đọc**: ~45 phút | **Level**: Principal
> **Kết quả cuối cùng**: Chọn đúng data pattern cho đúng bài toán.

---

Bạn biết kho hàng thông minh không? Kho truyền thống: lưu SỐ LƯỢNG hiện tại (100 laptops). Kho thông minh: lưu MỌI GIAO DỊCH (nhập 200, xuất 50, xuất 30, nhập 10 → tính ra 130). Kho thông minh = **Event Sourcing**: lưu sự kiện (events), tính state từ events. Kho truyền thống = CRUD: lưu state trực tiếp, mất lịch sử.

---

## Advanced Data Patterns — Vượt qua CRUD

Production applications cần nhiều hơn CRUD: schema migrations an toàn, CQRS cho read/write optimization, event sourcing cho audit trails, caching layers cho performance, NoSQL khi relational model không phù hợp.

Chapter này trang bị toolkit xử lý data ở mức production. Mỗi pattern đi kèm TypeScript implementation và use case rõ ràng — bạn sẽ biết khi nào dùng Redis cache, khi nào dùng Event Store, khi nào switch sang MongoDB.


CRUD (Create, Read, Update, Delete) là bước đầu. Khi ứng dụng scale, bạn gặp 4 bài toán mới:

**1. Schema evolution**: Database schema thay đổi khi features thêm. Migration scripts phải safe (reversible), tracked (version control), và tested (CI/CD).

**2. Read/Write separation** (CQRS): 90% requests là reads. Nếu read và write dùng cùng model, read bị chậm vì model optimized cho writes. CQRS tách: write store normalized, read store denormalized + cached.

**3. Caching**: Database roundtrip = 10-100ms. Redis cache = 0.1ms. 100x faster. Nhưng cache invalidation = "one of the two hard things in CS" (Phil Karlton). Strategies: cache-aside (lazy), write-through (eager), TTL (time-based).

**4. Event sourcing**: Thay vì overwrite current state, lưu **stream of events**. Reconstruct state bằng replay. Benefits: full audit trail, time travel, event replay cho debugging. Trade-off: complex reads.

## Advanced Data Patterns — CQRS, Event Sourcing, Caching

Migrations cho schema evolution. CQRS tách read/write models. Event sourcing lưu events thay vì state. Caching strategies (Redis, TTL, invalidation). Patterns này xuất hiện trong MỌI hệ thống production phức tạp.


## 38.1 — Database Migrations

```typescript
// filename: src/data_patterns/migrations.ts
import assert from "node:assert/strict";

// Migration = versioned schema changes
// Each migration: UP (apply change) + DOWN (revert)

// === Migration example ===
type Migration = {
    readonly version: string;
    readonly name: string;
    readonly up: string;    // SQL to apply
    readonly down: string;  // SQL to revert
};

const migrations: Migration[] = [
    {
        version: "001",
        name: "create_users",
        up: `CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL
        )`,
        down: "DROP TABLE users",
    },
    {
        version: "002",
        name: "add_user_role",
        up: "ALTER TABLE users ADD COLUMN role VARCHAR(50) DEFAULT 'user'",
        down: "ALTER TABLE users DROP COLUMN role",
    },
    {
        version: "003",
        name: "create_orders",
        up: `CREATE TABLE orders (
            id SERIAL PRIMARY KEY,
            user_id INT NOT NULL REFERENCES users(id),
            status VARCHAR(50) NOT NULL DEFAULT 'draft',
            total BIGINT NOT NULL DEFAULT 0
        )`,
        down: "DROP TABLE orders",
    },
];

// Simulate migration runner
const appliedMigrations: string[] = [];

const migrate = (ms: readonly Migration[], targetVersion?: string): void => {
    for (const m of ms) {
        if (targetVersion && m.version > targetVersion) break;
        if (!appliedMigrations.includes(m.version)) {
            // Execute m.up SQL
            appliedMigrations.push(m.version);
        }
    }
};

migrate(migrations);
assert.strictEqual(appliedMigrations.length, 3);
assert.deepStrictEqual(appliedMigrations, ["001", "002", "003"]);

// Prisma: npx prisma migrate dev --name add_user_role
// Drizzle: npx drizzle-kit generate && npx drizzle-kit migrate

console.log("Migrations OK ✅");
```

---

## 38.2 — CQRS: Command Query Responsibility Segregation

```typescript
// filename: src/data_patterns/cqrs.ts
import assert from "node:assert/strict";

// CQRS = Separate models for READ and WRITE

// === Write Model (commands) ===
// Normalized, consistent, optimized for writes
type OrderWriteModel = {
    id: string;
    userId: string;
    status: string;
    items: { productId: string; quantity: number; unitPrice: number }[];
};

// === Read Model (queries) ===
// Denormalized, optimized for reads, can be materialized view
type OrderReadModel = {
    id: string;
    userName: string;        // joined from users
    status: string;
    itemCount: number;       // pre-computed
    total: number;           // pre-computed
    productNames: string[];  // joined from products
};

// Write side: commands modify normalized data
const processCommand = (store: Map<string, OrderWriteModel>, cmd: {
    type: "create_order";
    order: OrderWriteModel;
}): void => {
    store.set(cmd.order.id, cmd.order);
};

// Read side: project events into denormalized read model
const buildReadModel = (
    order: OrderWriteModel,
    users: Map<string, string>,      // id → name
    products: Map<string, string>,   // id → name
): OrderReadModel => ({
    id: order.id,
    userName: users.get(order.userId) ?? "Unknown",
    status: order.status,
    itemCount: order.items.length,
    total: order.items.reduce((s, i) => s + i.quantity * i.unitPrice, 0),
    productNames: order.items.map(i => products.get(i.productId) ?? "Unknown"),
});

// Test
const users = new Map([["U1", "An"]]);
const products = new Map([["P1", "Laptop"], ["P2", "Mouse"]]);

const order: OrderWriteModel = {
    id: "ORD-1", userId: "U1", status: "confirmed",
    items: [
        { productId: "P1", quantity: 1, unitPrice: 20_000_000 },
        { productId: "P2", quantity: 2, unitPrice: 500_000 },
    ],
};

const readModel = buildReadModel(order, users, products);
assert.strictEqual(readModel.userName, "An");
assert.strictEqual(readModel.total, 21_000_000);
assert.deepStrictEqual(readModel.productNames, ["Laptop", "Mouse"]);

console.log("CQRS OK ✅");
```

---

## 38.3 — Event Sourcing

```typescript
// filename: src/data_patterns/event_sourcing.ts
import assert from "node:assert/strict";

// Event Sourcing = store EVENTS, derive current state

type AccountEvent =
    | { type: "AccountOpened"; accountId: string; ownerName: string; timestamp: Date }
    | { type: "MoneyDeposited"; accountId: string; amount: number; timestamp: Date }
    | { type: "MoneyWithdrawn"; accountId: string; amount: number; timestamp: Date }
    | { type: "AccountClosed"; accountId: string; timestamp: Date };

type AccountState = {
    id: string;
    ownerName: string;
    balance: number;
    status: "active" | "closed";
};

// Fold events → current state (PURE FUNCTION!)
const buildState = (events: readonly AccountEvent[]): AccountState | null => {
    let state: AccountState | null = null;

    for (const event of events) {
        switch (event.type) {
            case "AccountOpened":
                state = { id: event.accountId, ownerName: event.ownerName, balance: 0, status: "active" };
                break;
            case "MoneyDeposited":
                if (state) state = { ...state, balance: state.balance + event.amount };
                break;
            case "MoneyWithdrawn":
                if (state) state = { ...state, balance: state.balance - event.amount };
                break;
            case "AccountClosed":
                if (state) state = { ...state, status: "closed" };
                break;
        }
    }

    return state;
};

// Event stream
const events: AccountEvent[] = [
    { type: "AccountOpened", accountId: "A1", ownerName: "An", timestamp: new Date("2024-01-01") },
    { type: "MoneyDeposited", accountId: "A1", amount: 10_000_000, timestamp: new Date("2024-01-02") },
    { type: "MoneyWithdrawn", accountId: "A1", amount: 3_000_000, timestamp: new Date("2024-01-03") },
    { type: "MoneyDeposited", accountId: "A1", amount: 5_000_000, timestamp: new Date("2024-01-04") },
];

const state = buildState(events);
assert.strictEqual(state!.balance, 12_000_000);  // 10M - 3M + 5M
assert.strictEqual(state!.status, "active");

// Time travel: replay first 3 events only
const pastState = buildState(events.slice(0, 3));
assert.strictEqual(pastState!.balance, 7_000_000);  // 10M - 3M

// Audit trail: events ARE the log!
assert.strictEqual(events.length, 4);  // full history preserved

console.log("Event Sourcing OK ✅");
```

> **💡 Event Sourcing advantages**: full audit trail, time travel, replayable, debug-friendly. Disadvantages: eventually consistent, more complex reads, storage grows over time.

---

## 38.4 — NoSQL & Caching

```typescript
// filename: src/data_patterns/nosql_caching.ts
import assert from "node:assert/strict";

// === NoSQL Types ===
// Document (MongoDB): JSON-like docs, flexible schema
// Key-Value (Redis): fast lookup, caching, pub/sub
// Wide-Column (DynamoDB): partition + sort key, massive scale
// Graph (Neo4j): relationships first

// === When NoSQL vs SQL? ===
// SQL: relationships, transactions, complex queries, normalized data
// MongoDB: flexible schemas, embedded documents, rapid prototyping
// Redis: caching, sessions, rate limiting, queues
// DynamoDB: massive scale, predictable latency, serverless

// === Caching Strategies ===

// Cache-aside (lazy loading):
// 1. Check cache → hit? return cached
// 2. Cache miss → query DB → store in cache → return
type Cache<T> = {
    get: (key: string) => T | undefined;
    set: (key: string, value: T, ttlMs: number) => void;
};

const createCache = <T>(): Cache<T> & { store: Map<string, { value: T; expiresAt: number }> } => {
    const store = new Map<string, { value: T; expiresAt: number }>();
    return {
        store,
        get: (key) => {
            const entry = store.get(key);
            if (!entry) return undefined;
            if (Date.now() > entry.expiresAt) { store.delete(key); return undefined; }
            return entry.value;
        },
        set: (key, value, ttlMs) => {
            store.set(key, { value, expiresAt: Date.now() + ttlMs });
        },
    };
};

// Cache-aside pattern
const cache = createCache<string>();
let dbQueries = 0;

const getUserName = (id: string): string => {
    const cached = cache.get(id);
    if (cached) return cached;  // cache hit!

    // Cache miss → "query DB"
    dbQueries++;
    const name = `User-${id}`;
    cache.set(id, name, 60_000);  // cache for 60s
    return name;
};

assert.strictEqual(getUserName("U1"), "User-U1");  // miss → DB query
assert.strictEqual(dbQueries, 1);
assert.strictEqual(getUserName("U1"), "User-U1");  // hit → from cache
assert.strictEqual(dbQueries, 1);  // no additional DB query!

assert.strictEqual(getUserName("U2"), "User-U2");  // miss for different key
assert.strictEqual(dbQueries, 2);

console.log("NoSQL & Caching OK ✅");
```

---

## ✅ Checkpoint 38

> Đến đây bạn phải hiểu:
> 1. **Migrations** = versioned schema evolution. UP + DOWN
> 2. **CQRS** = separate read/write models. Writes normalized, reads denormalized
> 3. **Event Sourcing** = store events, derive state. Full audit trail + time travel
> 4. **NoSQL** = right tool for right job. SQL for relations, Redis for caching
> 5. **Cache-aside** = check cache first, query DB on miss, store result

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Event Sourcing cho shopping cart.

<details><summary>✅ Lời giải</summary>

```typescript
import assert from "node:assert/strict";

type CartEvent =
    | { type: "ItemAdded"; productId: string; quantity: number; price: number }
    | { type: "ItemRemoved"; productId: string }
    | { type: "QuantityUpdated"; productId: string; quantity: number }
    | { type: "CartCleared" };

type CartItem = { productId: string; quantity: number; price: number };
type CartState = { items: CartItem[]; total: number };

const buildCart = (events: readonly CartEvent[]): CartState => {
    let items: CartItem[] = [];
    for (const e of events) {
        switch (e.type) {
            case "ItemAdded": items = [...items, { productId: e.productId, quantity: e.quantity, price: e.price }]; break;
            case "ItemRemoved": items = items.filter(i => i.productId !== e.productId); break;
            case "QuantityUpdated": items = items.map(i => i.productId === e.productId ? { ...i, quantity: e.quantity } : i); break;
            case "CartCleared": items = []; break;
        }
    }
    return { items, total: items.reduce((s, i) => s + i.quantity * i.price, 0) };
};

const cart = buildCart([
    { type: "ItemAdded", productId: "P1", quantity: 2, price: 100 },
    { type: "ItemAdded", productId: "P2", quantity: 1, price: 200 },
    { type: "QuantityUpdated", productId: "P1", quantity: 3 },
]);
assert.strictEqual(cart.total, 500); // 3*100 + 1*200
assert.strictEqual(cart.items.length, 2);
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Migration thất bại" | SQL lỗi hoặc conflict với data hiện tại | Test migration trên staging trước. Luôn có DOWN script |
| "Event store quá lớn" | Không có snapshotting | Snapshot mỗi N events — replay từ snapshot gần nhất |
| "Cache stale" | Invalidation không đúng | Dùng event-based invalidation hoặc TTL ngắn |
| "CQRS phức tạp" | Overkill cho app nhỏ | Chỉ dùng CQRS khi read/write patterns KHÁC NHAU rõ rệt |

---

## Tóm tắt

- ✅ **Migrations**: thay đổi schema có version, có thể rollback.
- ✅ **CQRS**: tách model đọc và ghi. Writes chuẩn hóa, reads denormalized.
- ✅ **Event Sourcing**: events = nguồn sự thật. Tính state từ events. Full audit trail.
- ✅ **NoSQL**: MongoDB (documents), Redis (cache), DynamoDB (scale).
- ✅ **Caching**: cache-aside, TTL, invalidation.

## Tiếp theo

→ Chapter 39: **Security Essentials** — authentication, authorization, password hashing, JWT.
