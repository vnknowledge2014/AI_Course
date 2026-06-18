# Chapter 37 — Database Fundamentals & SQL

> **Bạn sẽ học được**:
> - Relational model: tables, rows, keys — nền tảng mọi database
> - SQL essentials: SELECT, JOIN, GROUP BY, CTEs — đọc/viết dữ liệu
> - Normalization: 1NF → 3NF → BCNF — tổ chức data không thừa
> - Indexing: B-Tree, composite indexes, EXPLAIN ANALYZE — performance
> - Transactions: ACID, isolation levels — data integrity
> - TypeScript ORMs: Prisma, Drizzle, Kysely — type-safe database access
>
> **Yêu cầu trước**: Chapter 24 (Repository Pattern).
> **Thời gian đọc**: ~55 phút | **Level**: Principal
> **Kết quả cuối cùng**: Hiểu database từ gốc — design schemas, viết SQL, tối ưu queries.

---

Bạn biết thư viện không? Sách xếp theo KỆ (tables), mỗi kệ có CHỦ ĐỀ (schema). Mỗi cuốn có MÃ SỐ duy nhất (primary key). Catalog giúp TÌM sách nhanh (index). Thủ thư đảm bảo không ai mượn cùng cuốn sách (transactions). Database = thư viện số. SQL = ngôn ngữ nói chuyện với thủ thư.

---

## Database Fundamentals & SQL — Persistence cho TypeScript developers

TypeScript developers thường né SQL — dùng ORM rồi hy vọng nó generate đúng queries. Approach đó hoạt động 80% trường hợp. 20% còn lại — performance queries, complex joins, aggregate reports — bạn **phải** biết SQL.

Chapter này dạy SQL fundamentals qua TypeScript lens: Prisma (type-safe ORM — generate types từ schema), Drizzle (SQL-like, closer to metal), và Kysely (query builder, full type inference). Bạn sẽ thấy: biết SQL + type-safe ORM = superpower.


Nhiều TypeScript developers có tâm lý "ORM handles everything" — dùng Prisma/TypeORM rồi không bao giờ viết SQL. Điều này hoạt động cho 80% use cases. Nhưng 20% còn lại — performance queries, complex aggregations, data migrations — bạn **phải** hiểu SQL.

Tin tốt: SQL skills transfer giữa mọi ORM. Khi bạn biết `JOIN`, `GROUP BY`, `INDEX`, bạn đọc Prisma-generated queries và hiểu tại sao chúng chậm. Bạn biết khi nào cần raw query thay vì ORM abstractions.

Chapter này dạy SQL theo cách practical: mỗi concept đi kèm TypeScript implementation bằng Prisma (type-safe ORM) hoặc Kysely (type-safe query builder). Bạn sẽ thấy: biết SQL + type-safe tools = debug queries trong seconds thay vì hours.

## Database Fundamentals — SQL, NoSQL, and ORMs

Relational model (SQL) cho data có relationships. Document stores (NoSQL) cho flexiblility. ORMs (Prisma) = type-safe database access. Indexing = performance. Normalization vs denormalization = trade-off. Chapter này dạy CHỌN đúng tool cho đúng job.


## 37.1 — Relational Model

```typescript
// filename: src/database/relational_model.ts
import assert from "node:assert/strict";

// === Tables = Tập hợp records cùng cấu trúc ===

// Table: users
// | id (PK) | name    | email           | created_at |
// |---------|---------|-----------------|------------|
// | 1       | An      | an@mail.com     | 2024-01-01 |
// | 2       | Bình    | binh@mail.com   | 2024-01-02 |

// Table: orders
// | id (PK) | user_id (FK) | status    | total      |
// |---------|-------------|-----------|------------|
// | 101     | 1           | confirmed | 20000000   |
// | 102     | 1           | draft     | 500000     |
// | 103     | 2           | shipped   | 15000000   |

// === Keys ===
// Primary Key (PK): uniquely identifies each row (id)
// Foreign Key (FK): references PK of another table (user_id → users.id)
// Composite Key: PK with multiple columns (order_items: order_id + product_id)

// Simulating tables as TypeScript (what ORMs do internally):
type User = { id: number; name: string; email: string; createdAt: Date };
type Order = { id: number; userId: number; status: string; total: number };
type OrderItem = { orderId: number; productId: number; quantity: number; unitPrice: number };

// Relationships:
// User 1→N Orders (one user, many orders)
// Order 1→N OrderItems (one order, many items)
// Product 1→N OrderItems (one product, in many orders)

// In-memory simulation of a JOIN:
const users: User[] = [
    { id: 1, name: "An", email: "an@mail.com", createdAt: new Date("2024-01-01") },
    { id: 2, name: "Bình", email: "binh@mail.com", createdAt: new Date("2024-01-02") },
];

const orders: Order[] = [
    { id: 101, userId: 1, status: "confirmed", total: 20_000_000 },
    { id: 102, userId: 1, status: "draft", total: 500_000 },
    { id: 103, userId: 2, status: "shipped", total: 15_000_000 },
];

// INNER JOIN: orders + users (matching userId)
const ordersWithUsers = orders.map(order => ({
    ...order,
    userName: users.find(u => u.id === order.userId)!.name,
}));

assert.strictEqual(ordersWithUsers[0].userName, "An");
assert.strictEqual(ordersWithUsers[2].userName, "Bình");

// LEFT JOIN: all users, even those without orders
const usersWithOrderCount = users.map(user => ({
    ...user,
    orderCount: orders.filter(o => o.userId === user.id).length,
}));

assert.strictEqual(usersWithOrderCount[0].orderCount, 2);  // An: 2 orders
assert.strictEqual(usersWithOrderCount[1].orderCount, 1);  // Bình: 1 order

console.log("Relational model OK ✅");
```

---

## 37.2 — SQL Essentials

```typescript
// filename: src/database/sql_essentials.ts
import assert from "node:assert/strict";

// SQL = Structured Query Language
// Declarative: tell WHAT you want, not HOW to get it

// === SELECT — Read data ===
// SELECT name, email FROM users WHERE id = 1;
// SELECT * FROM orders WHERE status = 'confirmed' ORDER BY total DESC;
// SELECT COUNT(*) FROM orders WHERE user_id = 1;

// === JOIN — Combine tables ===
// SELECT u.name, o.id, o.total
// FROM orders o
// INNER JOIN users u ON o.user_id = u.id
// WHERE o.status = 'confirmed';

// === GROUP BY + Aggregates ===
// SELECT u.name, COUNT(o.id) as order_count, SUM(o.total) as total_spent
// FROM users u
// LEFT JOIN orders o ON u.id = o.user_id
// GROUP BY u.name
// HAVING COUNT(o.id) > 0;

// === CTEs (Common Table Expressions) — readable subqueries ===
// WITH high_value_orders AS (
//     SELECT * FROM orders WHERE total > 10000000
// ),
// user_totals AS (
//     SELECT user_id, SUM(total) as grand_total
//     FROM high_value_orders
//     GROUP BY user_id
// )
// SELECT u.name, ut.grand_total
// FROM user_totals ut
// JOIN users u ON ut.user_id = u.id;

// Simulating SQL operations in TypeScript:
type Row = Record<string, unknown>;

const users = [
    { id: 1, name: "An", email: "an@mail.com" },
    { id: 2, name: "Bình", email: "binh@mail.com" },
    { id: 3, name: "Cường", email: "cuong@mail.com" },
];

const orders = [
    { id: 101, userId: 1, status: "confirmed", total: 20_000_000 },
    { id: 102, userId: 1, status: "draft", total: 500_000 },
    { id: 103, userId: 2, status: "shipped", total: 15_000_000 },
    { id: 104, userId: 1, status: "confirmed", total: 8_000_000 },
];

// SELECT ... WHERE (filter)
const confirmed = orders.filter(o => o.status === "confirmed");
assert.strictEqual(confirmed.length, 2);

// GROUP BY + SUM (aggregate)
const userTotals = new Map<number, number>();
for (const o of orders) {
    userTotals.set(o.userId, (userTotals.get(o.userId) ?? 0) + o.total);
}
assert.strictEqual(userTotals.get(1), 28_500_000);  // An: 3 orders
assert.strictEqual(userTotals.get(2), 15_000_000);  // Bình: 1 order

// HAVING (filter aggregates)
const bigSpenders = [...userTotals.entries()].filter(([_, total]) => total > 20_000_000);
assert.strictEqual(bigSpenders.length, 1);  // Only An

console.log("SQL essentials OK ✅");
```

---

## ✅ Checkpoint 37.1-37.2

> Đến đây bạn phải hiểu:
> 1. **Tables** = collections of structured records. **PK** = unique identifier. **FK** = reference
> 2. **JOIN** = combine tables on matching keys. INNER (both match), LEFT (all left + matched right)
> 3. **GROUP BY** = aggregate per group. **HAVING** = filter groups
> 4. **CTEs** = named subqueries. Readable complex queries
>
> **Test nhanh**: User xóa account → orders của user xảy ra gì?
> <details><summary>Đáp án</summary>FK constraint! Database PREVENTS delete (or CASCADE deletes orders too). Depends on `ON DELETE` policy: RESTRICT, CASCADE, SET NULL.</details>

---

## 37.3 — Normalization

```typescript
// filename: src/database/normalization.ts
import assert from "node:assert/strict";

// === 1NF: No repeating groups, atomic values ===
// ❌ Bad: { id: 1, phones: "123, 456, 789" }  ← comma-separated
// ✅ Good: Separate table: user_phones(user_id, phone)

// === 2NF: No partial dependencies (for composite keys) ===
// ❌ Bad: order_items(order_id, product_id, product_name, quantity)
//         product_name depends on product_id ONLY, not full composite key
// ✅ Good: order_items(order_id, product_id, quantity) + products(id, name)

// === 3NF: No transitive dependencies ===
// ❌ Bad: orders(id, user_id, user_name, user_email)
//         user_name depends on user_id, not on order id
// ✅ Good: orders(id, user_id) + users(id, name, email)

// === BCNF: Every determinant is a candidate key ===
// Stronger version of 3NF. Rarely needed in practice.

// Practical example: denormalized → normalized
// ❌ Denormalized (one big table):
type DenormalizedOrder = {
    orderId: number;
    userName: string;      // duplicated per order!
    userEmail: string;     // duplicated per order!
    productName: string;   // duplicated per item!
    productPrice: number;  // duplicated per item!
    quantity: number;
};

// ✅ Normalized (3 tables):
type User_n = { id: number; name: string; email: string };
type Product_n = { id: number; name: string; price: number };
type Order_n = { id: number; userId: number };
type OrderItem_n = { orderId: number; productId: number; quantity: number };

// Benefits:
// - No data duplication (user name stored ONCE)
// - Update anomaly prevented (change name in ONE place)
// - Delete anomaly prevented (delete order doesn't lose product info)

// Trade-off: more JOINs needed for reads. Sometimes denormalize for READ performance.

console.log("Normalization OK ✅");
```

---

## 37.4 — Indexing & Performance

```typescript
// filename: src/database/indexing.ts
import assert from "node:assert/strict";

// === B-Tree Index ===
// Default index type. Sorted tree structure.
// CREATE INDEX idx_users_email ON users(email);

// Without index: FULL TABLE SCAN → O(n)
// With index: B-Tree lookup → O(log n)

// === When to index ===
// ✅ Columns in WHERE clauses
// ✅ Columns in JOIN conditions (foreign keys)
// ✅ Columns in ORDER BY
// ❌ Columns rarely queried
// ❌ Tables with < 1000 rows (scan is fine)
// ❌ Columns with very low cardinality (boolean — only 2 values)

// === Composite Index ===
// CREATE INDEX idx_orders_user_status ON orders(user_id, status);
// Works for: WHERE user_id = 1 AND status = 'confirmed'
// Works for: WHERE user_id = 1 (leftmost prefix)
// Does NOT work for: WHERE status = 'confirmed' alone (not leftmost)

// === EXPLAIN ANALYZE ===
// EXPLAIN ANALYZE SELECT * FROM orders WHERE user_id = 1;
// Shows: Seq Scan vs Index Scan, actual rows, execution time

// Simulating index lookup:
const buildIndex = <T>(items: readonly T[], key: keyof T): Map<unknown, T[]> => {
    const index = new Map<unknown, T[]>();
    for (const item of items) {
        const k = item[key];
        if (!index.has(k)) index.set(k, []);
        index.get(k)!.push(item);
    }
    return index;
};

const orders = Array.from({ length: 10000 }, (_, i) => ({
    id: i,
    userId: Math.floor(Math.random() * 100),
    status: ["draft", "confirmed", "shipped"][i % 3],
    total: Math.floor(Math.random() * 50_000_000),
}));

// Without index: scan all 10000
const withoutIndex = orders.filter(o => o.userId === 42);

// With index: direct lookup
const userIdIndex = buildIndex(orders, "userId");
const withIndex = userIdIndex.get(42) ?? [];

assert.strictEqual(withoutIndex.length, withIndex.length);
// Same result, but index lookup is O(1) vs O(n) scan

console.log("Indexing OK ✅");
```

---

## 37.5 — Transactions & ACID

```typescript
// filename: src/database/transactions.ts
import assert from "node:assert/strict";

// === ACID Properties ===
// Atomicity: all-or-nothing. Transfer $100: debit AND credit both succeed or both fail
// Consistency: database always valid. Constraints enforced
// Isolation: concurrent transactions don't interfere
// Durability: committed data survives crashes

// === Isolation Levels ===
// READ UNCOMMITTED: see other's uncommitted changes (dirty reads)
// READ COMMITTED: only see committed data (default PostgreSQL)
// REPEATABLE READ: same query = same results within transaction
// SERIALIZABLE: full isolation, as if transactions run one-by-one

// === TypeScript: Prisma transactions ===
// await prisma.$transaction(async (tx) => {
//     await tx.account.update({ where: { id: fromId }, data: { balance: { decrement: amount } } });
//     await tx.account.update({ where: { id: toId }, data: { balance: { increment: amount } } });
// });

// Simulating transaction behavior:
type Account = { id: string; balance: number };

const transfer = (
    accounts: Map<string, Account>,
    fromId: string,
    toId: string,
    amount: number,
): { ok: boolean; error?: string } => {
    const from = accounts.get(fromId);
    const to = accounts.get(toId);

    if (!from || !to) return { ok: false, error: "Account not found" };
    if (from.balance < amount) return { ok: false, error: "Insufficient funds" };

    // Atomic: both updates or neither
    from.balance -= amount;
    to.balance += amount;
    return { ok: true };
};

const accounts = new Map<string, Account>([
    ["A1", { id: "A1", balance: 1_000_000 }],
    ["A2", { id: "A2", balance: 500_000 }],
]);

const result = transfer(accounts, "A1", "A2", 300_000);
assert.strictEqual(result.ok, true);
assert.strictEqual(accounts.get("A1")!.balance, 700_000);
assert.strictEqual(accounts.get("A2")!.balance, 800_000);

// Insufficient funds
const fail = transfer(accounts, "A1", "A2", 999_000_000);
assert.strictEqual(fail.ok, false);
assert.strictEqual(accounts.get("A1")!.balance, 700_000);  // unchanged!

console.log("Transactions OK ✅");
```

---

## 37.6 — TypeScript ORMs: Prisma, Drizzle, Kysely

```typescript
// filename: src/database/typescript_orms.ts

// === Prisma — Type-safe ORM ===
// schema.prisma defines models → generates TypeScript types
// model User {
//     id    Int      @id @default(autoincrement())
//     name  String
//     email String   @unique
//     orders Order[]
// }
//
// const user = await prisma.user.findUnique({ where: { id: 1 } });
// // TypeScript KNOWS: user is User | null
// const orders = await prisma.order.findMany({
//     where: { userId: 1, status: 'confirmed' },
//     orderBy: { total: 'desc' },
// });

// === Drizzle — SQL-like, lightweight ===
// const users = pgTable('users', {
//     id: serial('id').primaryKey(),
//     name: varchar('name', { length: 255 }),
//     email: varchar('email', { length: 255 }).unique(),
// });
//
// const result = await db.select().from(users).where(eq(users.id, 1));
// // Closer to SQL syntax. Type-safe.

// === Kysely — Query builder ===
// const user = await db.selectFrom('users')
//     .selectAll()
//     .where('id', '=', 1)
//     .executeTakeFirst();

// === Comparison ===
// Prisma: highest abstraction, schema-first, migrations built-in. Best for: rapid development
// Drizzle: SQL-like syntax, lightweight. Best for: SQL-savvy developers
// Kysely: minimal, just query builder. Best for: maximum control

console.log("TypeScript ORMs OK ✅");
```

---

## ✅ Checkpoint 37.3-37.6

> Đến đây bạn phải hiểu:
> 1. **Normalization**: eliminate duplicates, separate concerns into tables
> 2. **Indexing**: B-Tree for fast lookup. Composite for multi-column. EXPLAIN ANALYZE
> 3. **ACID**: Atomicity (all-or-nothing), Consistency, Isolation, Durability
> 4. **Prisma/Drizzle/Kysely**: type-safe database access in TypeScript
>
> **Test nhanh**: Index trên `orders(status)` — tốt hay xấu?
> <details><summary>Đáp án</summary>**XẤU** nếu chỉ có 3-4 status values. Low cardinality → index không hiệu quả (mỗi bucket quá lớn). Better: composite `(user_id, status)`.</details>

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Thiết kế schema chuẩn hóa

```typescript
// Design tables for Blog system:
// - Users (id, name, email)
// - Posts (id, author, title, content, published_at)
// - Comments (id, post, author, content, created_at)
// - Tags (id, name)
// - PostTags (many-to-many)
// Define PKs, FKs, indexes
```

<details><summary>✅ Lời giải</summary>

```sql
-- Users
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);

-- Posts
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author_id INT NOT NULL REFERENCES users(id),
    title VARCHAR(500) NOT NULL,
    content TEXT NOT NULL,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_posts_author ON posts(author_id);
CREATE INDEX idx_posts_published ON posts(published_at);

-- Tags
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

-- PostTags (many-to-many junction table)
CREATE TABLE post_tags (
    post_id INT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    tag_id INT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, tag_id)  -- composite PK
);

-- Comments
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    author_id INT NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_comments_post ON comments(post_id);
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Query chậm" | Thiếu index | Chạy EXPLAIN ANALYZE. Thêm index cho cột WHERE/JOIN |
| "Dữ liệu không nhất quán" | Không dùng transaction | Wrap các updates liên quan trong transaction |
| "Quá nhiều JOINs" | Chuẩn hóa quá mức | Denormalize data đọc nhiều. Giữ writes normalized |
| "Prisma hay Drizzle?" | Trade-offs khác nhau | Prisma: dev nhanh. Drizzle: kiểm soát SQL. Chọn theo kỹ năng team |

---

## Tóm tắt

- ✅ **Relational model**: tables, PKs, FKs, relationships (1→N, M→N).
- ✅ **SQL**: SELECT, JOIN, GROUP BY, CTEs — truy vấn dữ liệu khai báo.
- ✅ **Normalization**: 1NF→3NF. Loại bỏ trùng lặp. Trade-off: nhiều JOINs hơn.
- ✅ **Indexing**: B-Tree, composite. EXPLAIN ANALYZE cho performance.
- ✅ **ACID**: transactions đảm bảo tính toàn vẹn dữ liệu.
- ✅ **TypeScript ORMs**: Prisma (schema-first), Drizzle (SQL-like), Kysely (query builder).

## Tiếp theo

→ Chapter 38: **Advanced Data Patterns** — migrations, CQRS, Event Store, NoSQL, caching.
