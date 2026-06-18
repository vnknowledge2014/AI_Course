# Chapter 39 — Advanced Data Patterns

> **Bạn sẽ học được**:
> - **Migrations**: schema versioning, zero-downtime strategies
> - **CQRS persistence**: separate read/write databases
> - **Event Store**: append-only table, snapshots, replay
> - **NoSQL khi nào**: Document, Key-Value, Column, Graph
> - **Caching**: read-through, write-through, invalidation
> - **Redis patterns**: cache, pub/sub, rate limiting
>
> **Yêu cầu trước**: Chapter 19 (CQRS/ES), Chapter 38 (Database).
> **Thời gian đọc**: ~45 phút | **Level**: Principal
> **Kết quả cuối cùng**: Bạn chọn đúng data pattern cho từng bài toán.

---

## Advanced Data Patterns — Vượt qua CRUD

Hầu hết tutorials dạy bạn CRUD — Create, Read, Update, Delete. Đó là bước đầu, nhưng production systems cần nhiều hơn: migrations, CQRS (tách read/write), event sourcing, caching layers, NoSQL khi SQL không đủ.

Chapter này trang bị cho bạn toolkit xử lý data ở mức production: từ schema migrations an toàn đến caching strategies giảm load database 10-100x.

---

Ch38 dạy SQL fundamentals. Chapter này đi xa hơn — vào lãnh thổ mà production systems thực sự cần.

**Schema migrations**: database thay đổi theo thời gian. Thêm column, rename table, split data — tất cả cần migration scripts an toàn, reversible, và tracked trong version control.

**CQRS persistence**: khi read patterns khác write patterns (100:1 read/write ratio phổ biến), bạn tách read store và write store để optimize riêng.

**Caching**: database query mất 10-100ms. Redis cache query mất 0.1ms. Caching layers giảm load database 10-100x — nhưng cache invalidation là "one of the two hard things in computer science" (Phil Karlton). Chapter này dạy bạn strategies: cache-aside, write-through, TTL.

**NoSQL**: khi relational model không phù hợp — document stores (MongoDB), key-value stores (Redis), time-series — bạn cần biết khi nào chọn gì.

## 39.1 — Migrations: Schema Versioning

### Tại sao cần migrations

```
Dev:    CREATE TABLE users (id, name, email)
V2:     ALTER TABLE users ADD COLUMN avatar_url
V3:     CREATE TABLE orders (...)
V4:     ALTER TABLE users ADD COLUMN role DEFAULT 'user'

PROBLEM: Làm sao đảm bảo MỌI environment (dev, staging, prod)
         có CÙNG schema?
ANSWER:  Migrations = ordered, versioned SQL scripts
```

### Migration files

```
migrations/
├── 001_create_users.sql
├── 002_create_products.sql
├── 003_create_orders.sql
├── 004_add_user_avatar.sql
└── 005_add_user_role.sql
```

```sql
-- migrations/001_create_users.sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- migrations/004_add_user_avatar.sql
ALTER TABLE users ADD COLUMN avatar_url VARCHAR(500);

-- migrations/005_add_user_role.sql
ALTER TABLE users ADD COLUMN role VARCHAR(20) NOT NULL DEFAULT 'user';
CREATE INDEX idx_users_role ON users(role);
```

### Zero-downtime migration strategies

| Chiến lược | Cách làm | Khi nào |
|----------|-----|---------|
| **Thêm cột** | `ALTER TABLE ADD COLUMN` (nullable/default) | Thêm field mới ✅ |
| **Đổi tên cột** | Thêm mới → copy data → xóa cũ (3 bước!) | Đổi tên ⚠️ |
| **Xóa cột** | Deploy code bỏ dùng → rồi mới drop column | Xóa field |
| **Thêm index** | `CREATE INDEX CONCURRENTLY` | Tránh lock table |
| **Đổi kiểu** | Thêm cột mới → migrate data → swap | Thay đổi type |

> **💡 Rule**: Thêm = safe. Xóa/đổi = multi-step. **Luôn backward-compatible** trong transition period.

---

## 39.2 — CQRS Persistence

### Tách Read và Write Databases

```
                    ┌──────────────┐
   Commands ───────▶│ Write DB     │──── Events ────▶ Projections
   (INSERT/UPDATE)  │ (PostgreSQL) │                      │
                    └──────────────┘                      ▼
                                                   ┌──────────┐
   Queries ────────────────────────────────────────▶│ Read DB   │
   (SELECT)                                        │ (denorm)  │
                                                   └──────────┘
```

### Write side: normalized, consistent

```sql
-- Write database: normalized, ACID
CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY,
    owner_id BIGINT NOT NULL REFERENCES users(id),
    balance BIGINT NOT NULL DEFAULT 0 CHECK (balance >= 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'VND'
);

CREATE TABLE transactions (
    id BIGSERIAL PRIMARY KEY,
    from_account BIGINT REFERENCES accounts(id),
    to_account BIGINT REFERENCES accounts(id),
    amount BIGINT NOT NULL CHECK (amount > 0),
    type VARCHAR(20) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

### Read side: denormalized, fast

```sql
-- Read database: denormalized for queries
CREATE TABLE account_summaries (
    account_id BIGINT PRIMARY KEY,
    owner_name VARCHAR(100),
    owner_email VARCHAR(255),
    balance BIGINT,
    currency VARCHAR(3),
    transaction_count INTEGER,
    last_transaction_at TIMESTAMP
);

-- Projection: rebuild from events/transactions
-- UPDATE account_summaries SET ... after each transaction
```

### Rust implementation sketch

```rust
// filename: src/main.rs

// ═══ CQRS: separate command and query models ═══

// Command side
trait AccountCommands {
    fn deposit(&mut self, account_id: u64, amount: u64) -> Result<(), String>;
    fn withdraw(&mut self, account_id: u64, amount: u64) -> Result<(), String>;
    fn transfer(&mut self, from: u64, to: u64, amount: u64) -> Result<(), String>;
}

// Query side (different model, optimized for reads)
trait AccountQueries {
    fn get_summary(&self, account_id: u64) -> Option<AccountSummary>;
    fn get_top_accounts(&self, limit: usize) -> Vec<AccountSummary>;
    fn get_monthly_report(&self, year: u32, month: u32) -> MonthlyReport;
}

#[derive(Debug)]
struct AccountSummary {
    account_id: u64,
    owner: String,
    balance: i64,
    transaction_count: u32,
}

#[derive(Debug)]
struct MonthlyReport {
    total_deposits: u64,
    total_withdrawals: u64,
    active_accounts: u32,
}

fn main() {
    println!("CQRS: Commands (write, normalized) vs Queries (read, denormalized)");
    println!("Write DB: PostgreSQL (ACID, consistent)");
    println!("Read DB: Materialized views or separate store (fast queries)");
}
```

---

## 39.3 — Event Store

### Append-only event log

```sql
-- Event Store: append-only, immutable
CREATE TABLE events (
    id          BIGSERIAL PRIMARY KEY,
    stream_id   VARCHAR(100) NOT NULL,  -- e.g. "order-123"
    event_type  VARCHAR(100) NOT NULL,  -- e.g. "OrderPlaced"
    data        JSONB NOT NULL,
    metadata    JSONB DEFAULT '{}',
    version     INTEGER NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(stream_id, version)
);

CREATE INDEX idx_events_stream ON events(stream_id, version);

-- Snapshots: periodic state capture (optimization)
CREATE TABLE snapshots (
    stream_id   VARCHAR(100) PRIMARY KEY,
    state       JSONB NOT NULL,
    version     INTEGER NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);
```

### Rust Event Store

```rust
// filename: src/main.rs

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// ═══ Events ═══
#[derive(Debug, Clone, Serialize, Deserialize)]
enum OrderEvent {
    OrderPlaced { order_id: String, customer: String, items: Vec<(String, u32)> },
    OrderConfirmed { order_id: String, total: u64 },
    OrderShipped { order_id: String, tracking: String },
    OrderCancelled { order_id: String, reason: String },
}

// ═══ Event Store ═══
#[derive(Debug)]
struct StoredEvent {
    stream_id: String,
    event_type: String,
    data: String, // JSON
    version: u32,
}

struct InMemoryEventStore {
    events: Vec<StoredEvent>,
}

impl InMemoryEventStore {
    fn new() -> Self { InMemoryEventStore { events: vec![] } }

    fn append(&mut self, stream_id: &str, event: &OrderEvent) {
        let version = self.events.iter()
            .filter(|e| e.stream_id == stream_id)
            .count() as u32 + 1;

        let event_type = match event {
            OrderEvent::OrderPlaced { .. } => "OrderPlaced",
            OrderEvent::OrderConfirmed { .. } => "OrderConfirmed",
            OrderEvent::OrderShipped { .. } => "OrderShipped",
            OrderEvent::OrderCancelled { .. } => "OrderCancelled",
        };

        self.events.push(StoredEvent {
            stream_id: stream_id.into(),
            event_type: event_type.into(),
            data: serde_json::to_string(event).unwrap(),
            version,
        });
    }

    fn get_stream(&self, stream_id: &str) -> Vec<&StoredEvent> {
        self.events.iter()
            .filter(|e| e.stream_id == stream_id)
            .collect()
    }
}

// ═══ Rebuild state from events ═══
#[derive(Debug, Default)]
struct OrderState {
    order_id: String,
    customer: String,
    items: Vec<(String, u32)>,
    total: u64,
    status: String,
    tracking: Option<String>,
}

fn rebuild_order(events: &[&StoredEvent]) -> OrderState {
    let mut state = OrderState::default();

    for stored in events {
        let event: OrderEvent = serde_json::from_str(&stored.data).unwrap();
        match event {
            OrderEvent::OrderPlaced { order_id, customer, items } => {
                state.order_id = order_id;
                state.customer = customer;
                state.items = items;
                state.status = "placed".into();
            }
            OrderEvent::OrderConfirmed { total, .. } => {
                state.total = total;
                state.status = "confirmed".into();
            }
            OrderEvent::OrderShipped { tracking, .. } => {
                state.tracking = Some(tracking);
                state.status = "shipped".into();
            }
            OrderEvent::OrderCancelled { .. } => {
                state.status = "cancelled".into();
            }
        }
    }

    state
}

fn main() {
    let mut store = InMemoryEventStore::new();

    // Record events
    store.append("order-1", &OrderEvent::OrderPlaced {
        order_id: "order-1".into(), customer: "Minh".into(),
        items: vec![("Coffee".into(), 2), ("Tea".into(), 1)],
    });
    store.append("order-1", &OrderEvent::OrderConfirmed {
        order_id: "order-1".into(), total: 215_000,
    });
    store.append("order-1", &OrderEvent::OrderShipped {
        order_id: "order-1".into(), tracking: "VN123456".into(),
    });

    // Rebuild current state
    let events = store.get_stream("order-1");
    let state = rebuild_order(&events);
    println!("Order state: {:?}", state);
    println!("Events count: {}", events.len());
}
```

---

## 39.4 — NoSQL: Khi nào dùng

Không phải bài toán nào cũng cần bảng quan hệ. Khi dữ liệu lồng nhau phức tạp (document), cần tốc độ cực nhanh (key-value), ghi liên tục hàng triệu dòng/giây (column), hoặc quan hệ chằng chịt (graph) — SQL truyền thống không phải lựa chọn tối ưu. Bảng dưới giúp bạn chọn đúng công cụ:

| Loại | Engine | Tốt cho | Không phù hợp khi |
|------|--------|---------|---------|
| **Document** | MongoDB | Schema linh hoạt, data lồng nhau | Cần JOIN phức tạp |
| **Key-Value** | Redis | Cache, sessions, counters | Query phức tạp |
| **Column** | ScyllaDB, Cassandra | Time-series, ghi nhiều | Query ad-hoc |
| **Graph** | Neo4j | Quan hệ (social, recommendation) | Data dạng bảng |
| **Relational** | PostgreSQL | Đa năng, ACID, JOIN | Ghi cực lớn (extreme scale) |

### Hướng dẫn chọn nhanh

```
Cần ACID + complex queries?        → PostgreSQL
Cần cache + real-time?             → Redis
Schema thay đổi liên tục?          → MongoDB
Write-heavy time-series?           → ScyllaDB
Complex relationships?             → Neo4j
Default choice (khi chưa biết)?    → PostgreSQL ✅
```

---

## 39.5 — Caching Patterns

### Read-through cache

```rust
// filename: src/main.rs

use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

struct Cache<T: Clone> {
    entries: HashMap<String, CacheEntry<T>>,
    ttl: Duration,
}

impl<T: Clone> Cache<T> {
    fn new(ttl_secs: u64) -> Self {
        Cache { entries: HashMap::new(), ttl: Duration::from_secs(ttl_secs) }
    }

    fn get(&self, key: &str) -> Option<T> {
        self.entries.get(key).and_then(|entry| {
            if Instant::now() < entry.expires_at {
                Some(entry.value.clone())
            } else {
                None // expired
            }
        })
    }

    fn set(&mut self, key: String, value: T) {
        self.entries.insert(key, CacheEntry {
            value,
            expires_at: Instant::now() + self.ttl,
        });
    }
}

// Read-through pattern
fn get_product(cache: &mut Cache<Product>, db: &Database, id: u64) -> Option<Product> {
    let key = format!("product:{}", id);

    // 1. Check cache
    if let Some(product) = cache.get(&key) {
        println!("  Cache HIT: {}", key);
        return Some(product);
    }

    // 2. Cache miss → query DB
    println!("  Cache MISS: {}", key);
    if let Some(product) = db.find_product(id) {
        cache.set(key, product.clone());
        Some(product)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
struct Product { id: u64, name: String, price: u32 }

struct Database {
    products: HashMap<u64, Product>,
}

impl Database {
    fn find_product(&self, id: u64) -> Option<Product> {
        self.products.get(&id).cloned()
    }
}

fn main() {
    let mut cache = Cache::new(300); // 5 min TTL
    let mut db_products = HashMap::new();
    db_products.insert(1, Product { id: 1, name: "Coffee".into(), price: 85_000 });
    let db = Database { products: db_products };

    println!("First call (miss):");
    let _ = get_product(&mut cache, &db, 1);

    println!("Second call (hit):");
    let _ = get_product(&mut cache, &db, 1);

    println!("Not found:");
    let _ = get_product(&mut cache, &db, 99);
}
```

### Caching strategies

| Chiến lược | Cách hoạt động | Dùng khi |
|----------|-----|----------|
| **Read-through** | Cache miss → đọc từ DB → lưu cache | Phổ biến nhất ✅ |
| **Write-through** | Ghi cache + DB cùng lúc | Ưu tiên nhất quán |
| **Write-behind** | Ghi cache → flush async xuống DB | Ưu tiên hiệu năng |
| **Cache-aside** | App tự quản lý cache | Cần kiểm soát chi tiết |

### Cache invalidation

```
"There are only two hard things in CS:
 cache invalidation and naming things."

Strategies:
1. TTL (Time-To-Live)          — simplest, eventual consistency
2. Event-based invalidation    — on write, invalidate related keys
3. Version tags                — cache key includes version number
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Lên kế hoạch migration

Bạn cần thêm `phone` column vào `users` table (production, 1M rows). Describe migration strategy.

<details><summary>✅ Lời giải</summary>

```sql
-- Step 1: Add nullable column (no lock, instant)
ALTER TABLE users ADD COLUMN phone VARCHAR(20);

-- Step 2: Deploy code that writes phone (optional for now)
-- Step 3: Backfill existing rows (batched!)
-- UPDATE users SET phone = '' WHERE phone IS NULL LIMIT 10000;

-- Step 4 (optional): Add NOT NULL after backfill
-- ALTER TABLE users ALTER COLUMN phone SET NOT NULL DEFAULT '';
```
Key: nullable first → backfill → then add constraint. Never add NOT NULL column without default on large tables.

</details>

---

**Bài 2** (10 phút): Thiết kế Event Store

Design event store cho `ShoppingCart`:
- Events: `ItemAdded`, `ItemRemoved`, `QuantityChanged`, `CartCleared`
- Write `rebuild_cart(events) -> CartState`

<details><summary>✅ Lời giải Bài 2</summary>

```rust
enum CartEvent {
    ItemAdded { product_id: u64, name: String, price: u32, qty: u32 },
    ItemRemoved { product_id: u64 },
    QuantityChanged { product_id: u64, new_qty: u32 },
    CartCleared,
}

struct CartItem { product_id: u64, name: String, price: u32, qty: u32 }
struct CartState { items: HashMap<u64, CartItem>, total: u64 }

fn rebuild_cart(events: &[CartEvent]) -> CartState {
    let mut items = HashMap::new();
    for event in events {
        match event {
            CartEvent::ItemAdded { product_id, name, price, qty } => {
                items.insert(*product_id, CartItem { product_id: *product_id, name: name.clone(), price: *price, qty: *qty });
            }
            CartEvent::ItemRemoved { product_id } => { items.remove(product_id); }
            CartEvent::QuantityChanged { product_id, new_qty } => {
                if let Some(item) = items.get_mut(product_id) { item.qty = *new_qty; }
            }
            CartEvent::CartCleared => { items.clear(); }
        }
    }
    let total = items.values().map(|i| i.price as u64 * i.qty as u64).sum();
    CartState { items, total }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Migration khóa bảng" | ALTER TABLE trên bảng lớn | `ADD COLUMN` nullable, `CREATE INDEX CONCURRENTLY` |
| "Event store phình to" | Không dọn dẹp | Snapshots + archive events cũ |
| "Cache stampede" | Nhiều thread miss cache cùng lúc | Lock per key, hoặc refresh sớm xác suất |
| "Cache cũ" | TTL quá dài | Invalidation dựa trên event |

---

## Tóm tắt

- ✅ **Migrations**: Versioned SQL scripts. Add = safe. Rename/Delete = multi-step.
- ✅ **CQRS Persistence**: Write DB (normalized, ACID) vs Read DB (denormalized, fast).
- ✅ **Event Store**: Append-only events + rebuild state. Snapshots for performance.
- ✅ **NoSQL decision**: PostgreSQL default. Redis cache. MongoDB flexible. ScyllaDB scale.
- ✅ **Caching**: Read-through (most common), TTL + event invalidation, cache-aside for control.

## Tiếp theo

→ Chapter 40: **Security Essentials** — password hashing (argon2), JWT, OAuth 2.0, RBAC authorization.
