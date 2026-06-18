# Chapter 33 — Database & SQL

> **Bạn sẽ học được**:
> - SQLAlchemy Core — functional SQL
> - Alembic migrations
> - Repository implementation with PostgreSQL
> - Connection pooling, transactions
>
> **Yêu cầu trước**: Chapter 32 (Capstone)
> **Thời gian đọc**: ~35 phút | **Level**: Principal

---

## 33.1 — SQLAlchemy Core (Not ORM)

```python
from sqlalchemy import create_engine, MetaData, Table, Column, Integer, String, text

metadata = MetaData()

orders_table = Table(
    "orders", metadata,
    Column("id", String, primary_key=True),
    Column("customer_email", String, nullable=False),
    Column("total", Integer, nullable=False),
    Column("status", String, nullable=False, default="pending"),
)

# Create in-memory SQLite for demo
engine = create_engine("sqlite:///:memory:")
metadata.create_all(engine)

# Insert
with engine.begin() as conn:
    conn.execute(orders_table.insert().values(
        id="ORD-001", customer_email="an@mail.com", total=120_000, status="confirmed"
    ))

# Query
with engine.connect() as conn:
    result = conn.execute(
        orders_table.select().where(orders_table.c.id == "ORD-001")
    ).fetchone()
    assert result is not None
    assert result.total == 120_000
```

## 33.2 — Repository with SQLAlchemy

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Order:
    id: str
    customer_email: str
    total: int
    status: str

class SqlOrderRepository:
    def __init__(self, engine):
        self.engine = engine

    def find_by_id(self, order_id: str) -> Order | None:
        with self.engine.connect() as conn:
            row = conn.execute(
                orders_table.select().where(orders_table.c.id == order_id)
            ).fetchone()
            if row is None:
                return None
            return Order(id=row.id, customer_email=row.customer_email,
                        total=row.total, status=row.status)

    def save(self, order: Order) -> None:
        with self.engine.begin() as conn:
            existing = conn.execute(
                orders_table.select().where(orders_table.c.id == order.id)
            ).fetchone()
            if existing:
                conn.execute(
                    orders_table.update()
                    .where(orders_table.c.id == order.id)
                    .values(status=order.status, total=order.total)
                )
            else:
                conn.execute(orders_table.insert().values(
                    id=order.id, customer_email=order.customer_email,
                    total=order.total, status=order.status
                ))

# Test
repo = SqlOrderRepository(engine)
order = repo.find_by_id("ORD-001")
assert order is not None
assert order.total == 120_000
```

## 33.3 — Alembic Migrations

```bash
# Setup
# uv add alembic sqlalchemy
# alembic init migrations

# Create migration
# alembic revision --autogenerate -m "create orders table"

# Apply
# alembic upgrade head

# Rollback
# alembic downgrade -1
```

---

## Tóm tắt

- ✅ **SQLAlchemy Core**: Functional SQL — no ORM magic.
- ✅ **Repository**: Domain ↔ SQL translation.
- ✅ **Alembic**: Database migrations.
- ✅ **Transactions**: `engine.begin()` for atomicity.

## Tiếp theo

→ Chapter 34: **Advanced Data** — Redis caching, background tasks.
