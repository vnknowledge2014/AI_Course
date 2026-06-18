# Chapter 38 — Database Fundamentals & SQL

> **Bạn sẽ học được**:
> - **Relational model**: tables, rows, keys (PK, FK)
> - **SQL**: SELECT, JOIN, GROUP BY, subqueries, CTEs
> - **Normalization**: 1NF → 3NF
> - **Indexing**: B-Tree, composite index, EXPLAIN
> - **Transactions**: ACID, isolation levels
> - **Rust crates**: `sqlx` (compile-time checked), `diesel`
>
> **Yêu cầu trước**: Chapter 26 (Repository Pattern).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn hiểu databases đủ sâu để design schemas, tối ưu queries, và dùng Rust crates hiệu quả.

---

## Database & SQL Fundamentals — Nền tảng persistence

Mọi ứng dụng production đều cần database. Chapter này dạy bạn SQL fundamentals qua lăng kính Rust: từ relational model, queries, đến Rust ORMs (Diesel, SQLx, SeaORM). Bạn sẽ thấy Rust's type system bảo vệ bạn khỏi SQL injection tự động và verify queries tại compile time (với SQLx).

---

## 38.1 — Relational Model

Nếu bạn đã dùng Value Objects và Entities ở Chapter 22, bạn đã biết cách model domain trong code. Database là nơi bạn **lưu** chúng. Relational model tổ chức dữ liệu thành tables (bảng), mỗi row là một entity, mỗi column là một attribute. Các tables liên kết với nhau qua keys — giống cách aggregate root trong DDD kiểm soát các entities bên trong.

### Tables, Rows, Keys

```sql
-- ═══ Schema Design ═══

-- Users table
CREATE TABLE users (
    id          BIGSERIAL PRIMARY KEY,
    email       VARCHAR(255) NOT NULL UNIQUE,
    name        VARCHAR(100) NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Products table
CREATE TABLE products (
    id          BIGSERIAL PRIMARY KEY,
    code        VARCHAR(10) NOT NULL UNIQUE,
    name        VARCHAR(200) NOT NULL,
    price       INTEGER NOT NULL CHECK (price > 0),  -- cents
    stock       INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0)
);

-- Orders table (FK → users)
CREATE TABLE orders (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id),
    status          VARCHAR(20) NOT NULL DEFAULT 'draft',
    total           INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
    confirmed_at    TIMESTAMP
);

-- Order lines (FK → orders, products)
CREATE TABLE order_lines (
    id          BIGSERIAL PRIMARY KEY,
    order_id    BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id  BIGINT NOT NULL REFERENCES products(id),
    quantity    INTEGER NOT NULL CHECK (quantity > 0),
    unit_price  INTEGER NOT NULL,
    line_total  INTEGER NOT NULL
);
```

### Lựa chọn thực tế

| Hệ thống | Loại | Lý do | Ví dụ |
|-----|---------|-------|
| **Primary Key (PK)** | Định danh duy nhất | `users.id` |
| **Foreign Key (FK)** | Tham chiếu tới PK bảng khác | `orders.user_id → users.id` |
| **Unique** | Không trùng lặp | `users.email` |
| **Composite** | Nhiều cột = 1 key | `(order_id, product_id)` |

---

## 38.2 — Các lệnh SQL cơ bản (SQL Essentials)

SQL là ngôn ngữ để "nói chuyện" với database. Bắt đầu từ đơn giản (SELECT/WHERE) đến phức tạp hơn (JOIN, GROUP BY), rồi đến CTE — cách viết queries dài mà vẫn đọc được. Phần lớn ứng dụng thực tế chỉ cần những gì được dạy ở đây.

### SELECT & WHERE

```sql
-- Các truy vấn cơ bản
SELECT id, name, email FROM users WHERE id = 1;

SELECT * FROM products WHERE price > 50000 AND stock > 0;

SELECT * FROM orders WHERE status = 'confirmed' ORDER BY created_at DESC LIMIT 10;
```

### JOINs

```sql
-- INNER JOIN: orders + user info
SELECT o.id, u.name, u.email, o.total, o.status
FROM orders o
INNER JOIN users u ON o.user_id = u.id
WHERE o.status = 'confirmed';

-- LEFT JOIN: all users, even without orders
SELECT u.name, COUNT(o.id) AS order_count
FROM users u
LEFT JOIN orders o ON u.id = o.user_id
GROUP BY u.name
ORDER BY order_count DESC;

-- Multi JOIN: order details
SELECT o.id AS order_id, u.name AS customer,
       p.name AS product, ol.quantity, ol.line_total
FROM order_lines ol
INNER JOIN orders o ON ol.order_id = o.id
INNER JOIN users u ON o.user_id = u.id
INNER JOIN products p ON ol.product_id = p.id
WHERE o.id = 1;
```

### GROUP BY & Tổng hợp (Aggregation)

```sql
-- Doanh thu theo khách hàng
SELECT u.name, SUM(o.total) AS total_spent, COUNT(o.id) AS orders
FROM users u
INNER JOIN orders o ON u.id = o.user_id
WHERE o.status = 'confirmed'
GROUP BY u.name
HAVING SUM(o.total) > 1000000
ORDER BY total_spent DESC;

-- Sản phẩm bán chạy nhất theo số lượng
SELECT p.name, SUM(ol.quantity) AS total_sold, SUM(ol.line_total) AS revenue
FROM products p
INNER JOIN order_lines ol ON p.id = ol.product_id
GROUP BY p.name
ORDER BY total_sold DESC
LIMIT 5;
```

### CTEs (WITH)

```sql
-- Common Table Expression: readable complex queries
WITH monthly_revenue AS (
    SELECT DATE_TRUNC('month', o.created_at) AS month,
           SUM(o.total) AS revenue,
           COUNT(o.id) AS order_count
    FROM orders o
    WHERE o.status = 'confirmed'
    GROUP BY DATE_TRUNC('month', o.created_at)
),
growth AS (
    SELECT month, revenue, order_count,
           LAG(revenue) OVER (ORDER BY month) AS prev_revenue
    FROM monthly_revenue
)
SELECT month, revenue, order_count,
       CASE WHEN prev_revenue > 0
            THEN ROUND((revenue - prev_revenue)::numeric / prev_revenue * 100, 1)
            ELSE NULL
       END AS growth_pct
FROM growth
ORDER BY month;
```

---

## 38.3 — Chuẩn hóa (Normalization)

Khi bạn thiết kế schema, câu hỏi lớn nhất là: nên đặt dữ liệu ở bảng nào? Normalization là bộ quy tắc giúp loại bỏ dữ liệu trùng lặp — tương tự "Don't Repeat Yourself" trong code. Dưới đây là quá trình từ bảng lộn xộn đến schema chuẩn:

### 1NF → 3NF

```
❌ UN-NORMALIZED (denormalized)
┌────────────────────────────────────────────────────────┐
│ order_id │ customer │ email      │ products             │
│ 1        │ Minh     │ m@co.com   │ Coffee:2, Tea:1      │ ← multi-value!
└────────────────────────────────────────────────────────┘

✅ 1NF: Atomic values (no multi-value columns)
┌──────────────────────────────────────────────────┐
│ order_id │ customer │ email    │ product │ qty   │
│ 1        │ Minh     │ m@co.com │ Coffee  │ 2     │
│ 1        │ Minh     │ m@co.com │ Tea     │ 1     │ ← customer duplicated!
└──────────────────────────────────────────────────┘

✅ 2NF: Remove partial dependencies
orders:      id, customer_id, ...
order_lines: id, order_id, product, qty

✅ 3NF: Remove transitive dependencies
users:       id, name, email
orders:      id, user_id, status
order_lines: id, order_id, product_id, qty
products:    id, name, price
```

---

## ✅ Checkpoint 38.3

> Ghi nhớ:
> 1. **1NF**: Mỗi cell = 1 value (atomic)
> 2. **2NF**: No partial dependency (mọi non-key column phụ thuộc toàn bộ PK)
> 3. **3NF**: No transitive dependency (non-key không phụ thuộc non-key khác)
> 4. **Denormalize** khi: read performance quan trọng hơn write consistency

---

## 38.4 — Đánh chỉ mục (Indexing)

Query chậm thường không phải do SQL viết sai, mà vì thiếu index. Index giống mục lục cuối sách — thay vì đọc từ đầu đến cuối, nhảy thẳng đến trang cần tìm. Nhưng mỗi index là một cuốn mục lục thêm — càng nhiều index, INSERT/UPDATE càng chậm.

### Chỉ mục B-Tree (B-Tree Index - default)

```sql
-- Create indexes for common queries
CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_order_lines_order_id ON order_lines(order_id);
CREATE INDEX idx_products_code ON products(code);

-- Composite index: queries filtering on BOTH columns
CREATE INDEX idx_orders_user_status ON orders(user_id, status);

-- EXPLAIN: see query plan
EXPLAIN ANALYZE
SELECT * FROM orders WHERE user_id = 1 AND status = 'confirmed';
-- Expect: "Index Scan using idx_orders_user_status"
```

### Khi nào nên đánh chỉ mục (Khi nào nên index)

| Loại Index | Tốt cho | Không nên khi |
|-------|---------|---------|
| Single column | `WHERE col = value` | Query hiếm khi chạy |
| Composite | `WHERE a = x AND b = y` | Sai thứ tự cột |
| Covering | `SELECT a, b WHERE a = x` | Bảng quá rộng |
| Partial | `WHERE status = 'active'` | Phân bố đều (ít selective) |

> **💡 Rule of thumb**: Index columns dùng trong WHERE, JOIN ON, ORDER BY. Không index mọi thứ — mỗi index = overhead cho INSERT/UPDATE.

---

## 38.5 — Giao dịch & ACID (Transactions & ACID)

Chuyển tiền từ tài khoản A sang B gồm 2 bước: trừ A, cộng B. Nếu server chết giữa chừng — A mất tiền, B không nhận được. Transaction đảm bảo hai bước này hoặc **cả hai thành công**, hoặc **cả hai không xảy ra**.

### ACID

| Property | Ý nghĩa | Ví dụ |
|----------|---------|-------|
| **Atomicity** | All or nothing | Transfer: debit + credit = 1 unit |
| **Consistency** | Data valid trước + sau | Balance ≥ 0 |
| **Isolation** | Concurrent txn không interfere | 2 transfers cùng lúc |
| **Durability** | Committed = persisted | Server crash → data safe |

### Giao dịch trong SQL (Transaction trong SQL)

```sql
-- Transfer money: MUST be atomic
BEGIN;

UPDATE accounts SET balance = balance - 200000 WHERE id = 1;
UPDATE accounts SET balance = balance + 200000 WHERE id = 2;

-- Check constraint
SELECT balance FROM accounts WHERE id = 1;
-- If balance < 0 → ROLLBACK, else → COMMIT

COMMIT;
```

### Isolation levels

```
Read Uncommitted  → sees uncommitted changes (dirty reads)
Read Committed    → sees only committed (DEFAULT PostgreSQL)
Repeatable Read   → snapshot at txn start (no phantom reads)
Serializable      → fully isolated (slowest)
```

---

## 38.6 — Rust: sqlx (Compile-Time Checked Queries)

Rust có nhiều database crates, nhưng `sqlx` nổi bật với khả năng kiểm tra SQL query **lúc compile** — nếu column không tồn tại hoặc type sai, code không compile. Kết hợp với Repository pattern từ Chapter 26, domain code hoàn toàn không biết database là gì.

```rust
// filename: src/main.rs

// Cargo.toml:
// [dependencies]
// sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
// tokio = { version = "1", features = ["full"] }

use sqlx::postgres::PgPoolOptions;

// ═══ Domain types ═══
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[derive(Debug, sqlx::FromRow)]
struct OrderSummary {
    order_id: i64,
    customer_name: String,
    total: i32,
    status: String,
}

async fn run_queries(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    // ═══ INSERT ═══
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email"
    )
    .bind("Minh")
    .bind("minh@co.com")
    .fetch_one(pool)
    .await?;
    println!("Created: {:?}", user);

    // ═══ SELECT ═══
    let users = sqlx::query_as::<_, User>(
        "SELECT id, name, email FROM users WHERE email LIKE $1"
    )
    .bind("%@co.com")
    .fetch_all(pool)
    .await?;
    println!("Found {} users", users.len());

    // ═══ Transaction ═══
    let mut tx = pool.begin().await?;

    sqlx::query("UPDATE products SET stock = stock - $1 WHERE id = $2 AND stock >= $1")
        .bind(5)
        .bind(1)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "INSERT INTO orders (user_id, status, total) VALUES ($1, 'confirmed', $2)"
    )
    .bind(user.id)
    .bind(85000 * 5)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    println!("Transaction committed");

    Ok(())
}

#[tokio::main]
async fn main() {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://user:pass@localhost/mydb")
    //     .await
    //     .expect("Failed to create pool");
    //
    // run_queries(&pool).await.unwrap();

    println!("sqlx example (requires PostgreSQL connection)");
    println!("Key features:");
    println!("  - Compile-time query checking (sqlx::query!)");
    println!("  - Connection pooling (PgPoolOptions)");
    println!("  - Async/await (tokio runtime)");
    println!("  - Type-safe results (FromRow derive)");
}
```

### Repository pattern with sqlx

```rust
// filename: src/main.rs

// ═══ Port (trait) ═══
trait ProductRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<Product>, String>;
    async fn find_by_code(&self, code: &str) -> Result<Option<Product>, String>;
    async fn save(&self, product: &NewProduct) -> Result<Product, String>;
    async fn update_stock(&self, id: i64, delta: i32) -> Result<(), String>;
}

#[derive(Debug, Clone)]
struct Product { id: i64, code: String, name: String, price: i32, stock: i32 }
struct NewProduct { code: String, name: String, price: i32, stock: i32 }

// ═══ Adapter (sqlx implementation) ═══
// struct PgProductRepo { pool: sqlx::PgPool }
//
// impl ProductRepository for PgProductRepo {
//     async fn find_by_id(&self, id: i64) -> Result<Option<Product>, String> {
//         sqlx::query_as("SELECT * FROM products WHERE id = $1")
//             .bind(id)
//             .fetch_optional(&self.pool)
//             .await
//             .map_err(|e| e.to_string())
//     }
//     // ... other methods
// }

fn main() {
    println!("Repository pattern: trait (port) + sqlx (adapter)");
    println!("Domain code depends on trait, not database!");
}
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Viết SQL query

Viết query: "Top 3 customers by total spending (chỉ confirmed orders)"

<details><summary>✅ Lời giải</summary>

```sql
SELECT u.name, SUM(o.total) AS total_spent
FROM users u
INNER JOIN orders o ON u.id = o.user_id
WHERE o.status = 'confirmed'
GROUP BY u.name
ORDER BY total_spent DESC
LIMIT 3;
```

</details>

---

**Bài 2** (10 phút): Thiết kế schema

Design schema cho blog system:
- Users (id, name, email)
- Posts (id, author, title, content, published_at)
- Comments (id, post, commenter, content)
- Tags (many-to-many with posts)

<details><summary>✅ Lời giải Bài 2</summary>

```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    author_id BIGINT NOT NULL REFERENCES users(id),
    title VARCHAR(300) NOT NULL,
    content TEXT NOT NULL,
    published_at TIMESTAMP
);

CREATE TABLE comments (
    id BIGSERIAL PRIMARY KEY,
    post_id BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Junction table for many-to-many
CREATE TABLE post_tags (
    post_id BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, tag_id)
);

CREATE INDEX idx_posts_author ON posts(author_id);
CREATE INDEX idx_comments_post ON comments(post_id);
```

</details>

---

**Bài 3** (15 phút): Thiết kế transaction

Viết SQL transaction cho "Place Order":
1. Check stock cho mỗi item
2. Deduct stock
3. Create order + order_lines
4. Rollback nếu bất kỳ stock insufficient

<details><summary>✅ Lời giải Bài 3</summary>

```sql
BEGIN;

-- Check stock (SELECT FOR UPDATE locks rows)
SELECT id, stock FROM products WHERE id = 1 FOR UPDATE;
-- If stock < requested_qty → ROLLBACK

-- Deduct stock
UPDATE products SET stock = stock - 2 WHERE id = 1;

-- Create order
INSERT INTO orders (user_id, status, total)
VALUES (1, 'confirmed', 170000)
RETURNING id;
-- Assuming returned id = 42

-- Create order lines
INSERT INTO order_lines (order_id, product_id, quantity, unit_price, line_total)
VALUES (42, 1, 2, 85000, 170000);

COMMIT;
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Query chậm" | Thiếu index | `EXPLAIN ANALYZE`, thêm index |
| "N+1 queries" | Gọi DB trong vòng lặp | Dùng JOIN hoặc batch query |
| "Deadlock" | 2 txn lock cùng rows khác thứ tự | Đảm bảo lock ordering nhất quán |
| "sqlx compile error" | Schema không khớp | `sqlx prepare` / kiểm tra migrations |

---

## Tóm tắt

- ✅ **Relational Model**: Tables + PK/FK + constraints. Normalize → 3NF.
- ✅ **SQL**: SELECT/JOIN/GROUP BY/CTE — đủ cho 90% queries thực tế.
- ✅ **Indexing**: B-Tree mặc định, composite cho multi-column WHERE, EXPLAIN để verify.
- ✅ **ACID**: Transactions = atomicity + consistency + isolation + durability.
- ✅ **Rust**: `sqlx` (compile-time checked, async), `FromRow` derive, connection pooling.
- ✅ **Repository + sqlx**: Trait (port) → PgProductRepo (adapter). Clean architecture.

## Tiếp theo

→ Chapter 39: **Advanced Data Patterns** — migrations, CQRS persistence, event store, NoSQL khi nào, caching (Redis), Rust crates.
