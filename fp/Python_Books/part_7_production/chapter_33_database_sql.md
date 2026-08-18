# Chapter 33 — Database Fundamentals & SQL

> **Bạn sẽ học được**:
> - Relational model: table, row, key — nền tảng của mọi database quan hệ
> - SQL cốt lõi: `SELECT`, `JOIN`, `GROUP BY`, CTE — đọc và viết được truy vấn thật
> - Normalization 1NF → 3NF → BCNF, và khi nào cố tình phá chuẩn
> - Indexing: B-Tree, composite index, covering index, đọc `EXPLAIN ANALYZE`
> - Transactions: ACID, bốn mức isolation, optimistic vs pessimistic locking
> - Python: SQLAlchemy Core (không ORM), SQLModel, Alembic, `asyncpg`
>
> **Yêu cầu trước**: Chapter 24 (Persistence & Repository)
> **Thời gian đọc**: ~55 phút | **Level**: Principal
> **Kết quả cuối cùng**: Bạn thiết kế được schema, viết được SQL thật, và biết vì
> sao một truy vấn chậm — thay vì đoán mò rồi thêm index bừa.

---

Bạn có nhớ thư viện không? Sách xếp theo **kệ** (table), mỗi kệ một chủ đề
(schema). Mỗi cuốn có **mã số** duy nhất (primary key). Có **mục lục** để tìm
nhanh mà không phải đi dọc từng kệ (index). Và thủ thư đảm bảo hai người không
cùng mượn một cuốn (transaction).

Database quan hệ là thư viện số. SQL là ngôn ngữ nói chuyện với thủ thư.

Rất nhiều Python developer né SQL — dùng ORM rồi mong nó sinh ra truy vấn đúng.
Cách đó ổn cho 80% trường hợp. 20% còn lại — báo cáo tổng hợp, join phức tạp,
truy vấn chậm trên production — bạn **bắt buộc** phải biết SQL. Tin tốt: kỹ năng
SQL chuyển được giữa mọi ORM. Biết `JOIN` và `INDEX` rồi thì bạn đọc được truy
vấn mà SQLAlchemy sinh ra và hiểu vì sao nó chậm.

---

## 33.1 — Relational Model

Ba khái niệm, không hơn:

- **Table** (quan hệ): tập hợp các bản ghi cùng cấu trúc
- **Row** (bộ): một bản ghi
- **Key**: cách định danh và liên kết

| Loại key | Vai trò |
|---|---|
| **Primary key** | Định danh duy nhất một row. Không null, không đổi. |
| **Foreign key** | Trỏ tới primary key bảng khác. Database **ép** ràng buộc này. |
| **Composite key** | Primary key gồm nhiều cột (thường ở bảng nối) |
| **Natural vs surrogate** | Natural = dữ liệu thật (`email`). Surrogate = ID nhân tạo (`BIGSERIAL`, `UUID`) |

```sql
-- Schema cho hệ Order-Taking xuyên suốt cuốn sách
CREATE TABLE customers (
    id          BIGSERIAL PRIMARY KEY,          -- surrogate key
    email       TEXT NOT NULL UNIQUE,           -- natural key, vẫn ép duy nhất
    name        TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE orders (
    id           BIGSERIAL PRIMARY KEY,
    customer_id  BIGINT NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    status       TEXT NOT NULL CHECK (status IN ('pending','confirmed','shipped','cancelled')),
    total_cents  BIGINT NOT NULL CHECK (total_cents >= 0),
    placed_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE order_lines (
    order_id    BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    line_no     INT    NOT NULL,
    product_id  BIGINT NOT NULL,
    quantity    INT    NOT NULL CHECK (quantity > 0),
    PRIMARY KEY (order_id, line_no)              -- composite key
);
```

**Bốn quyết định trong schema trên đáng chú ý:**

1. **`total_cents BIGINT`, không phải `FLOAT`.** Tiền **không bao giờ** để kiểu
   dấu phẩy động: `0.1 + 0.2 != 0.3`. Lưu bằng đơn vị nhỏ nhất (xu) dưới dạng số
   nguyên, hoặc dùng `NUMERIC`.
2. **`CHECK (status IN ...)`** — đây chính là "make illegal states unrepresentable"
   (Chapter 13) áp dụng ở tầng database. Ứng dụng có bug cũng không ghi được
   trạng thái vô nghĩa.
3. **`ON DELETE RESTRICT` cho customer, `CASCADE` cho order line.** Xoá khách
   hàng còn đơn hàng thì phải chặn; xoá đơn hàng thì các dòng của nó đi theo.
   Ngữ nghĩa khác nhau nên hành vi khác nhau.
4. **`TIMESTAMPTZ`, không phải `TIMESTAMP`.** Bản không có timezone gần như luôn
   là bug đang chờ nổ khi bạn deploy sang region khác.

> **Ràng buộc thuộc về database, không phải chỉ ở tầng ứng dụng.** Ứng dụng có
> nhiều phiên bản chạy song song, có script chạy tay, có migration. Database là
> nơi duy nhất mọi đường ghi đều đi qua — đặt bất biến ở đó.

---

## 33.2 — SQL Essentials

### SELECT và lọc

```sql
SELECT id, status, total_cents
FROM orders
WHERE status = 'confirmed'
  AND placed_at >= now() - INTERVAL '7 days'
ORDER BY placed_at DESC
LIMIT 20;
```

### JOIN — bốn loại cần phân biệt

```sql
-- INNER: chỉ những row khớp ở CẢ HAI bảng
SELECT o.id, c.email
FROM orders o
INNER JOIN customers c ON c.id = o.customer_id;

-- LEFT: giữ MỌI customer, kể cả người chưa từng mua
SELECT c.email, o.id AS order_id
FROM customers c
LEFT JOIN orders o ON o.customer_id = c.id;

-- Mẹo hay dùng: tìm customer CHƯA có đơn nào
SELECT c.email
FROM customers c
LEFT JOIN orders o ON o.customer_id = c.id
WHERE o.id IS NULL;              -- ← "anti-join"
```

| | Giữ lại gì |
|---|---|
| `INNER JOIN` | Chỉ row khớp hai bên |
| `LEFT JOIN` | Toàn bộ bảng trái + phần khớp bên phải (thiếu thì `NULL`) |
| `RIGHT JOIN` | Ngược lại — hiếm dùng, cứ đổi chỗ bảng rồi dùng `LEFT` |
| `FULL OUTER` | Toàn bộ cả hai bên |

### GROUP BY và HAVING

```sql
SELECT c.email,
       COUNT(o.id)        AS order_count,
       SUM(o.total_cents) AS lifetime_cents
FROM customers c
JOIN orders o ON o.customer_id = c.id
WHERE o.status <> 'cancelled'      -- WHERE lọc TRƯỚC khi gom nhóm
GROUP BY c.email
HAVING COUNT(o.id) >= 3            -- HAVING lọc SAU khi gom nhóm
ORDER BY lifetime_cents DESC;
```

> **`WHERE` hay `HAVING`?** `WHERE` chạy trên từng row, **trước** `GROUP BY`.
> `HAVING` chạy trên nhóm, **sau** `GROUP BY`. Điều kiện nào lọc được bằng
> `WHERE` thì luôn nên để ở `WHERE` — lọc sớm thì có ít row hơn để gom nhóm.

### CTE — chia truy vấn lớn thành các bước đọc được

```sql
WITH recent_orders AS (
    SELECT * FROM orders
    WHERE placed_at >= now() - INTERVAL '30 days'
),
customer_totals AS (
    SELECT customer_id, SUM(total_cents) AS total
    FROM recent_orders
    GROUP BY customer_id
)
SELECT c.email, ct.total
FROM customer_totals ct
JOIN customers c ON c.id = ct.customer_id
WHERE ct.total > 1_000_000
ORDER BY ct.total DESC;
```

CTE với SQL cũng giống việc tách hàm nhỏ với Python: cùng một logic, nhưng đọc
được. Nếu bạn thấy mình đang lồng subquery ba tầng, hãy viết lại bằng CTE.

### Window function — thứ khiến SQL mạnh hơn hẳn ORM

```sql
-- Xếp hạng đơn hàng của TỪNG khách theo giá trị, giữ nguyên mọi row
SELECT customer_id,
       id,
       total_cents,
       ROW_NUMBER() OVER (PARTITION BY customer_id ORDER BY total_cents DESC) AS rn
FROM orders;
```

Khác `GROUP BY` — vốn thu gọn nhiều row thành một — window function **giữ nguyên
số row** và thêm cột tính toán theo nhóm. Đây là thứ rất khó diễn đạt bằng ORM
và cũng là lý do đáng để học SQL cho tử tế.

---

## ✅ Checkpoint 33.1–33.2

1. Tại sao `total_cents BIGINT` chứ không phải `total FLOAT`?
2. Viết truy vấn tìm customer đã đặt hàng nhưng **chưa từng** có đơn `confirmed`.
3. Điều kiện `o.status <> 'cancelled'` nên nằm ở `WHERE` hay `HAVING`? Vì sao?

<details>
<summary>Đáp án</summary>

1. Số dấu phẩy động không biểu diễn chính xác được số thập phân hệ 10 — cộng dồn
   tiền sẽ lệch dần. Dùng số nguyên đơn vị nhỏ nhất, hoặc `NUMERIC`.
2. ```sql
   SELECT DISTINCT c.email
   FROM customers c
   JOIN orders o ON o.customer_id = c.id
   WHERE NOT EXISTS (
       SELECT 1 FROM orders o2
       WHERE o2.customer_id = c.id AND o2.status = 'confirmed'
   );
   ```
3. `WHERE` — nó lọc được theo từng row, không cần chờ gom nhóm. Lọc sớm giúp
   `GROUP BY` xử lý ít dữ liệu hơn.
</details>

---

## 33.3 — Normalization

Chuẩn hoá là quá trình loại bỏ **dữ liệu lặp** — vì dữ liệu lặp là dữ liệu sẽ
mâu thuẫn.

### Bảng chưa chuẩn hoá

| order_id | customer_email | customer_name | product | qty |
|---|---|---|---|---|
| 1 | an@mail.com | An Nguyen | Bàn phím | 2 |
| 2 | an@mail.com | An Nguyen | Chuột | 1 |
| 3 | an@mail.com | **An Nguyễn** | Màn hình | 1 |

Row thứ ba là thảm hoạ: cùng một người, hai cái tên. Không có cách nào biết cái
nào đúng. Đây chính là **update anomaly**.

### Ba mức chuẩn

| Mức | Yêu cầu | Vi phạm điển hình |
|---|---|---|
| **1NF** | Mỗi ô một giá trị nguyên tử, không có nhóm lặp | Cột `tags` chứa `"a,b,c"` |
| **2NF** | 1NF + mọi cột non-key phụ thuộc **toàn bộ** primary key | Bảng có PK `(order_id, product_id)` nhưng chứa cột `order_date` |
| **3NF** | 2NF + không có cột non-key phụ thuộc cột non-key khác | Bảng orders chứa cả `customer_email` lẫn `customer_name` |
| **BCNF** | 3NF chặt hơn: mọi định thức đều phải là siêu khoá | Hiếm gặp; xuất hiện khi có nhiều candidate key chồng nhau |

Sửa bảng trên về 3NF chính là schema ở mục 33.1: tên khách nằm **một chỗ duy
nhất** trong bảng `customers`, mọi nơi khác chỉ giữ `customer_id`.

### Khi nào cố tình phá chuẩn

Denormalize là quyết định có chủ đích, không phải sự lười biếng — và luôn kèm giá:

```sql
-- Lưu sẵn total_cents ở bảng orders thay vì SUM order_lines mỗi lần đọc.
-- ĐƯỢC: trang danh sách đơn hàng nhanh hơn nhiều.
-- MẤT: total_cents có thể lệch với order_lines nếu ghi sót.
-- Chỉ làm khi (a) đã ĐO được là chậm, và (b) có cách bảo vệ tính nhất quán.

CREATE OR REPLACE FUNCTION recalc_order_total() RETURNS TRIGGER AS $$
BEGIN
    UPDATE orders o
    SET total_cents = (
        SELECT COALESCE(SUM(ol.quantity * p.price_cents), 0)
        FROM order_lines ol JOIN products p ON p.id = ol.product_id
        WHERE ol.order_id = o.id
    )
    WHERE o.id = COALESCE(NEW.order_id, OLD.order_id);
    RETURN NULL;
END $$ LANGUAGE plpgsql;
```

**Quy tắc:** chuẩn hoá trước, đo, rồi mới phá chuẩn ở đúng chỗ nghẽn. Phá chuẩn
"phòng xa" là cách chắc chắn nhất để có dữ liệu sai mà không nhanh hơn.

---

## 33.4 — Indexing & Performance

Index là mục lục sách. Không có nó, database phải đọc từ đầu tới cuối
(**sequential scan**).

### B-Tree — loại index bạn dùng 95% thời gian

```sql
-- Truy vấn hay chạy: lấy đơn của một khách, mới nhất trước
SELECT * FROM orders
WHERE customer_id = 42 AND status = 'confirmed'
ORDER BY placed_at DESC;

-- Composite index phục vụ đúng truy vấn đó
CREATE INDEX idx_orders_customer_status_placed
    ON orders (customer_id, status, placed_at DESC);
```

> **Thứ tự cột trong composite index quyết định tất cả — "quy tắc tiền tố trái".**
> Index `(a, b, c)` phục vụ được truy vấn lọc theo `a`, `(a,b)`, `(a,b,c)`.
> Nó **không** phục vụ được truy vấn chỉ lọc theo `b` hoặc chỉ theo `c`.
> Hãy nghĩ như danh bạ sắp theo (Họ, Tên): tra "mọi người tên An" thì vẫn phải
> đọc hết danh bạ.

### Các loại index khác

| Loại | Dùng khi |
|---|---|
| **B-Tree** | Mặc định. `=`, `<`, `>`, `BETWEEN`, `ORDER BY`, prefix `LIKE 'abc%'` |
| **Hash** | Chỉ `=`. Hiếm khi đáng so với B-Tree |
| **GIN** | `jsonb`, mảng, full-text search |
| **BRIN** | Bảng rất lớn, dữ liệu sắp sẵn theo thứ tự vật lý (log theo thời gian) |
| **Partial** | `CREATE INDEX ... WHERE status = 'pending'` — index nhỏ hơn nhiều |
| **Covering** | `INCLUDE (col)` — trả kết quả ngay từ index, không đụng bảng |

```sql
-- Partial index: 99% đơn hàng không ở trạng thái pending
CREATE INDEX idx_orders_pending ON orders (placed_at)
    WHERE status = 'pending';

-- Covering index: index-only scan, không cần đọc heap
CREATE INDEX idx_orders_cover ON orders (customer_id) INCLUDE (status, total_cents);
```

### Đọc `EXPLAIN ANALYZE`

```sql
EXPLAIN (ANALYZE, BUFFERS)
SELECT * FROM orders WHERE customer_id = 42;
```

```
Index Scan using idx_orders_customer_status_placed on orders
  (cost=0.29..8.31 rows=1 width=48) (actual time=0.021..0.023 rows=3 loops=1)
  Index Cond: (customer_id = 42)
  Buffers: shared hit=4
Planning Time: 0.104 ms
Execution Time: 0.041 ms
```

**Đọc từ đâu:**

| Nhìn thấy | Nghĩa là |
|---|---|
| `Seq Scan` trên bảng lớn | ❌ Thiếu index — đọc toàn bảng |
| `Index Scan` | ✅ Dùng index rồi lấy row từ bảng |
| `Index Only Scan` | ✅✅ Lấy hết dữ liệu từ index, không đụng bảng |
| `rows=1` mà `actual ... rows=50000` | ⚠️ Thống kê lệch nặng → chạy `ANALYZE` |
| `Nested Loop` với số row lớn | ⚠️ Thường nên là `Hash Join` |

Chênh lệch giữa `rows=` (ước lượng) và `actual rows=` (thực tế) là tín hiệu quan
trọng nhất. Planner chọn sai kế hoạch gần như luôn vì nó ước lượng sai.

### Giá của index

Index **không** miễn phí: mỗi `INSERT`/`UPDATE`/`DELETE` phải cập nhật mọi index
của bảng. Bảng có 8 index thì ghi chậm gấp nhiều lần bảng có 2.

```sql
-- Tìm index chưa từng được dùng — ứng viên để xoá
SELECT schemaname, relname, indexrelname, idx_scan
FROM pg_stat_user_indexes
WHERE idx_scan = 0
ORDER BY pg_relation_size(indexrelid) DESC;
```

---

## 33.5 — Transactions & ACID

```sql
BEGIN;
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
UPDATE accounts SET balance = balance + 100 WHERE id = 2;
COMMIT;      -- cả hai cùng xảy ra, hoặc không cái nào xảy ra
```

| Chữ | Nghĩa |
|---|---|
| **A**tomicity | Tất cả hoặc không gì cả |
| **C**onsistency | Ràng buộc (FK, CHECK, UNIQUE) luôn đúng trước và sau |
| **I**solation | Transaction đang chạy song song không nhìn thấy trạng thái dở dang của nhau |
| **D**urability | Đã `COMMIT` thì mất điện cũng còn |

### Bốn mức isolation

| Mức | Dirty read | Non-repeatable read | Phantom read |
|---|---|---|---|
| Read Uncommitted | Có¹ | Có | Có |
| **Read Committed** (mặc định PostgreSQL) | Không | Có | Có |
| Repeatable Read | Không | Không | Không² |
| Serializable | Không | Không | Không |

¹ PostgreSQL thực chất không bao giờ cho dirty read, kể cả khi bạn yêu cầu mức này.
² PostgreSQL dùng snapshot isolation nên Repeatable Read đã chặn được phantom read.

- **Non-repeatable read**: đọc cùng một row hai lần trong một transaction, ra hai giá trị khác nhau.
- **Phantom read**: chạy cùng một truy vấn hai lần, lần sau xuất hiện thêm row mới.

### Optimistic vs Pessimistic locking

```sql
-- PESSIMISTIC: khoá row ngay, ai đụng phải chờ
BEGIN;
SELECT * FROM orders WHERE id = 1 FOR UPDATE;   -- khoá tới khi COMMIT
UPDATE orders SET status = 'shipped' WHERE id = 1;
COMMIT;

-- OPTIMISTIC: không khoá, dùng cột version để phát hiện xung đột
UPDATE orders
SET status = 'shipped', version = version + 1
WHERE id = 1 AND version = 7;
-- Nếu trả về 0 row ⇒ có người khác đã sửa trước ⇒ đọc lại và thử lại
```

| | Pessimistic | Optimistic |
|---|---|---|
| Tốt khi | Tranh chấp cao | Tranh chấp thấp |
| Chi phí | Chờ đợi, có nguy cơ deadlock | Phải thử lại khi đụng độ |
| Hợp với | Trừ tồn kho, chuyển tiền | Sửa hồ sơ, cập nhật trạng thái |

> **Optimistic locking hợp với tinh thần FP hơn:** không có trạng thái khoá dùng
> chung, chỉ có một phép so sánh giá trị. Nó cũng hoạt động xuyên qua ranh giới
> service — nơi mà lock của database không với tới được.

---

## 33.6 — Python: SQLAlchemy Core, SQLModel, Alembic

### SQLAlchemy Core — SQL dưới dạng dữ liệu, không phải ORM

```python
# filename: db/schema.py
from sqlalchemy import (
    create_engine, MetaData, Table, Column, Integer, String, select,
)

metadata = MetaData()

orders_table = Table(
    "orders", metadata,
    Column("id", String, primary_key=True),
    Column("customer_email", String, nullable=False),
    Column("total", Integer, nullable=False),
    Column("status", String, nullable=False, default="pending"),
)

engine = create_engine("sqlite:///:memory:")
metadata.create_all(engine)

with engine.begin() as conn:                # begin() = tự COMMIT/ROLLBACK
    conn.execute(orders_table.insert().values(
        id="ORD-001", customer_email="an@mail.com",
        total=120_000, status="confirmed",
    ))

with engine.connect() as conn:
    row = conn.execute(
        select(orders_table).where(orders_table.c.id == "ORD-001")
    ).fetchone()
    assert row is not None
    assert row.total == 120_000
```

> **Vì sao Core chứ không phải ORM?** Core coi truy vấn là **dữ liệu** — bạn dựng
> nó lên bằng composition, in ra xem được, test được mà không cần database. ORM
> giấu truy vấn sau proxy object và lazy loading, dẫn tới N+1 query mà bạn không
> nhìn thấy. Với FP/DDD, Core hợp hơn: domain thuần, SQL tường minh ở rìa.

### Repository — biên dịch giữa domain và SQL

```python
# filename: db/repository.py
from dataclasses import dataclass
from typing import Protocol
from sqlalchemy import select
from sqlalchemy.engine import Engine
from sqlalchemy.dialects.sqlite import insert as sqlite_insert


@dataclass(frozen=True)
class Order:
    id: str
    customer_email: str
    total: int
    status: str


class OrderRepository(Protocol):
    """Port — domain chỉ biết Protocol này, không biết SQLAlchemy tồn tại."""
    def find_by_id(self, order_id: str) -> Order | None: ...
    def save(self, order: Order) -> None: ...


class SqlOrderRepository:
    """Adapter — nơi duy nhất biết về SQL."""

    def __init__(self, engine: Engine) -> None:
        self._engine = engine

    def find_by_id(self, order_id: str) -> Order | None:
        with self._engine.connect() as conn:
            row = conn.execute(
                select(orders_table).where(orders_table.c.id == order_id)
            ).fetchone()
        return None if row is None else Order(
            id=row.id, customer_email=row.customer_email,
            total=row.total, status=row.status,
        )

    def save(self, order: Order) -> None:
        """UPSERT nguyên tử — không có khe hở race giữa SELECT và INSERT."""
        stmt = sqlite_insert(orders_table).values(
            id=order.id, customer_email=order.customer_email,
            total=order.total, status=order.status,
        )
        stmt = stmt.on_conflict_do_update(
            index_elements=["id"],
            set_={"status": stmt.excluded.status, "total": stmt.excluded.total},
        )
        with self._engine.begin() as conn:
            conn.execute(stmt)


repo = SqlOrderRepository(engine)
found = repo.find_by_id("ORD-001")
assert found is not None and found.total == 120_000

repo.save(Order("ORD-001", "an@mail.com", 999, "shipped"))
assert repo.find_by_id("ORD-001").status == "shipped"   # type: ignore[union-attr]
```

> **So với bản `SELECT` rồi `if existing: UPDATE else: INSERT`:** cách đó có một
> khe hở — hai request đồng thời cùng thấy "chưa tồn tại" rồi cùng `INSERT`, một
> cái sẽ nổ vì trùng primary key. `ON CONFLICT DO UPDATE` để database xử lý
> nguyên tử. PostgreSQL dùng `postgresql.insert` với cùng cú pháp.

### SQLModel — khi bạn muốn một model dùng chung với FastAPI

```python
# filename: db/models.py
from sqlmodel import SQLModel, Field


class OrderRow(SQLModel, table=True):
    """Vừa là Pydantic model (validate, serialize) vừa là bảng SQL."""
    id: str = Field(primary_key=True)
    customer_email: str
    total: int
    status: str = "pending"
```

Tiện, nhưng có cái giá: bạn vừa gộp **domain model**, **DTO** và **schema
database** vào một class. Chapter 23 giải thích vì sao tách chúng ra lại đáng
giá. Dùng SQLModel cho CRUD service nhỏ; với domain phức tạp thì giữ ba lớp riêng.

### Async với `asyncpg`

```python
# filename: db/async_engine.py
from sqlalchemy.ext.asyncio import create_async_engine

engine = create_async_engine(
    "postgresql+asyncpg://user:pass@localhost/app",
    pool_size=10,            # ⚠️ nhân với số Gunicorn worker (xem Chapter 38)
    max_overflow=5,
    pool_pre_ping=True,      # phát hiện connection đã chết trước khi dùng
    pool_recycle=1800,       # tái tạo sau 30 phút, tránh bị proxy cắt ngang
)


async def fetch_order(order_id: str):
    async with engine.connect() as conn:
        result = await conn.execute(
            select(orders_table).where(orders_table.c.id == order_id)
        )
        return result.fetchone()
```

### Alembic — migration có version

```bash
uv add alembic sqlalchemy
alembic init migrations

alembic revision --autogenerate -m "create orders table"
alembic upgrade head          # áp dụng
alembic downgrade -1          # lùi 1 bước
alembic current               # đang ở revision nào
alembic history --verbose
```

> **Luôn đọc lại file mà `--autogenerate` sinh ra trước khi chạy.** Alembic phát
> hiện tốt việc thêm/xoá cột, nhưng nó **không** nhận ra việc *đổi tên* — nó sẽ
> sinh ra `DROP COLUMN old` + `ADD COLUMN new`, tức là xoá sạch dữ liệu cột đó.
> Đổi tên phải sửa tay thành `op.alter_column(..., new_column_name=...)`.

---

## ✅ Checkpoint 33.3–33.6

1. Bạn có index `(customer_id, status, placed_at)`. Truy vấn `WHERE status = 'pending'` có dùng được index này không?
2. `EXPLAIN` báo `Seq Scan` trên bảng 10 triệu row. Hai nguyên nhân có thể là gì?
3. Vì sao `ON CONFLICT DO UPDATE` an toàn hơn `SELECT` rồi `INSERT`/`UPDATE`?

<details>
<summary>Đáp án</summary>

1. **Không.** Quy tắc tiền tố trái: index sắp theo `customer_id` trước, nên lọc
   riêng `status` vẫn phải quét toàn bộ. Cần index riêng bắt đầu bằng `status`.
2. (a) Thiếu index phù hợp. (b) Có index nhưng planner cho rằng truy vấn trả về
   quá nhiều row nên seq scan rẻ hơn — thường do thống kê cũ, chạy `ANALYZE`.
   Nguyên nhân (c) hay gặp: điều kiện bọc cột trong hàm (`WHERE lower(email) = ...`)
   khiến index thường không dùng được; cần expression index.
3. Vì `SELECT` rồi `INSERT` là hai thao tác tách rời — hai request đồng thời cùng
   thấy "chưa có" rồi cùng `INSERT`. `ON CONFLICT` xử lý nguyên tử trong một câu lệnh.
</details>

---

## 🏋️ Bài tập

**Bài 1 (5 phút) — Đọc schema.**
Trong schema mục 33.1, vì sao `order_lines` dùng composite key `(order_id, line_no)`
thay vì thêm một `BIGSERIAL id`? Nêu một ưu và một nhược điểm.

**Bài 2 (10 phút) — Chuẩn hoá.**
Cho bảng: `invoices(id, customer_email, customer_address, item_name, item_price, qty)`.
Chỉ ra vi phạm 3NF và tách thành các bảng đúng chuẩn.

**Bài 3 (15 phút) — Optimistic locking.**
Bổ sung cột `version` vào `Order` và `orders_table`, rồi sửa `SqlOrderRepository.save`
để: (a) tăng `version` mỗi lần ghi, (b) ném `ConcurrentModificationError` khi
`UPDATE` trả về 0 row.

<details>
<summary>Gợi ý bài 3</summary>

`conn.execute(...)` trả về `CursorResult`; đọc `.rowcount`. Câu `UPDATE` cần
`.where(orders_table.c.id == order.id, orders_table.c.version == order.version)`.
Nhớ rằng `Order` là `frozen=True`, nên tạo bản mới bằng `dataclasses.replace`
thay vì gán trực tiếp.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| `QueuePool limit of size 10 overflow 5 reached` | Số worker × pool_size vượt `max_connections` | Giảm pool, hoặc đặt PgBouncer phía trước |
| Truy vấn nhanh ở dev, chậm ở prod | Dev ít dữ liệu nên seq scan vẫn nhanh | Test với dữ liệu thật; chạy `EXPLAIN ANALYZE` trên prod |
| `deadlock detected` | Hai transaction khoá row theo thứ tự ngược nhau | Luôn khoá theo cùng một thứ tự (ví dụ tăng dần theo id) |
| Alembic `Target database is not up to date` | Có revision chưa apply | `alembic upgrade head` trước khi tạo revision mới |
| Autogenerate sinh migration rỗng | Model chưa được import vào `env.py` | Import module chứa metadata trong `migrations/env.py` |
| Tổng tiền lệch vài xu | Dùng `FLOAT` cho tiền | Đổi sang `BIGINT` (đơn vị xu) hoặc `NUMERIC` |
| `LIKE '%abc'` cực chậm | Wildcard đứng đầu nên B-Tree vô dụng | Dùng GIN + `pg_trgm`, hoặc full-text search |

---

## Tóm tắt

- **Ràng buộc thuộc về database.** `CHECK`, `FOREIGN KEY`, `UNIQUE` là "make
  illegal states unrepresentable" ở tầng lưu trữ.
- **SQL vượt xa `SELECT *`.** CTE giúp truy vấn lớn đọc được; window function
  làm được thứ mà `GROUP BY` và ORM không làm được.
- **Chuẩn hoá trước, đo, rồi mới phá chuẩn** ở đúng chỗ nghẽn — kèm cơ chế giữ
  nhất quán.
- **Index có quy tắc tiền tố trái**, và mỗi index là một khoản thuế đánh lên mọi
  lần ghi. `EXPLAIN ANALYZE` là nguồn sự thật, không phải trực giác.
- **Optimistic locking hợp với FP hơn** và hoạt động xuyên qua ranh giới service.
- **SQLAlchemy Core** giữ SQL tường minh ở rìa hệ thống, domain vẫn thuần —
  đúng tinh thần kiến trúc onion của Chapter 19.

## Tiếp theo

Bạn đã có nền tảng dữ liệu vững. Chương sau bàn về những gì xảy ra khi một
database là không đủ: caching, background task, event store và NoSQL.
→ **[Chapter 34 — Advanced Data Patterns](chapter_34_advanced_data.md)**
