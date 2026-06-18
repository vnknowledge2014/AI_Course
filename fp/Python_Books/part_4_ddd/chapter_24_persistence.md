# Chapter 24 — Persistence

> **Bạn sẽ học được**:
> - Repository pattern — abstract database access
> - SQLAlchemy Core (not ORM) — functional style
> - In-memory repo for testing
> - Transaction boundaries
>
> **Yêu cầu trước**: Chapter 23 (ACL)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 24.1 — Repository Pattern

```python
from typing import Protocol
from dataclasses import dataclass

@dataclass(frozen=True)
class Order:
    id: str
    customer: str
    total: int
    status: str = "pending"

class OrderRepository(Protocol):
    def find_by_id(self, order_id: str) -> Order | None: ...
    def find_by_customer(self, customer: str) -> list[Order]: ...
    def save(self, order: Order) -> None: ...
    def delete(self, order_id: str) -> None: ...

# In-Memory Implementation (for tests)
class InMemoryOrderRepo:
    def __init__(self) -> None:
        self._store: dict[str, Order] = {}

    def find_by_id(self, order_id: str) -> Order | None:
        return self._store.get(order_id)

    def find_by_customer(self, customer: str) -> list[Order]:
        return [o for o in self._store.values() if o.customer == customer]

    def save(self, order: Order) -> None:
        self._store[order.id] = order

    def delete(self, order_id: str) -> None:
        self._store.pop(order_id, None)

# Test
repo = InMemoryOrderRepo()
repo.save(Order("ORD-1", "An", 100_000))
repo.save(Order("ORD-2", "An", 200_000))
repo.save(Order("ORD-3", "Binh", 150_000))

assert repo.find_by_id("ORD-1") is not None
assert len(repo.find_by_customer("An")) == 2
```

## 24.2 — SQL Repository (SQLAlchemy Core)

```python
# Production implementation sketch
# from sqlalchemy import create_engine, text

class SqlOrderRepo:
    """Real database implementation using SQLAlchemy Core."""

    def __init__(self, connection_string: str) -> None:
        # self.engine = create_engine(connection_string)
        pass

    def find_by_id(self, order_id: str) -> Order | None:
        # with self.engine.connect() as conn:
        #     row = conn.execute(
        #         text("SELECT * FROM orders WHERE id = :id"),
        #         {"id": order_id}
        #     ).fetchone()
        #     return self._row_to_domain(row) if row else None
        pass

    def save(self, order: Order) -> None:
        # with self.engine.begin() as conn:
        #     conn.execute(
        #         text("INSERT INTO orders (id, customer, total, status) "
        #              "VALUES (:id, :customer, :total, :status) "
        #              "ON CONFLICT (id) DO UPDATE SET ..."),
        #         {"id": order.id, "customer": order.customer,
        #          "total": order.total, "status": order.status}
        #     )
        pass

    def _row_to_domain(self, row) -> Order:
        return Order(id=row.id, customer=row.customer,
                     total=row.total, status=row.status)

    def find_by_customer(self, customer: str) -> list[Order]:
        pass
    def delete(self, order_id: str) -> None:
        pass
```

## 24.3 — Usage in Application Layer

```python
from dataclasses import replace

def confirm_order(order_id: str, repo: OrderRepository) -> Order:
    order = repo.find_by_id(order_id)
    if order is None:
        raise ValueError(f"Order {order_id} not found")
    if order.status != "pending":
        raise ValueError(f"Cannot confirm {order.status} order")
    confirmed = replace(order, status="confirmed")
    repo.save(confirmed)
    return confirmed

# Test with in-memory
repo = InMemoryOrderRepo()
repo.save(Order("ORD-1", "An", 100_000))
result = confirm_order("ORD-1", repo)
assert result.status == "confirmed"

# Production: confirm_order("ORD-1", SqlOrderRepo("postgresql://..."))
```

---

## Tóm tắt

- ✅ **Repository Protocol**: Abstract DB access behind Protocol.
- ✅ **In-Memory**: Fast, deterministic tests.
- ✅ **SQL**: SQLAlchemy Core for production.
- ✅ **Application layer**: Uses Protocol, not implementation.

## Tiếp theo

→ Chapter 25: **Abstract Algebra for Programmers** — Monoids, Semigroups.
