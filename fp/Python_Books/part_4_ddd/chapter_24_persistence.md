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

---

## ✅ Checkpoint 24

1. Vì sao Repository nên được định nghĩa bằng `Protocol` ở tầng domain chứ không phải class ở tầng infrastructure?
2. Repository nên trả về domain object hay ORM row? Vì sao?
3. Transaction boundary nên đặt ở tầng nào?

<details>
<summary>Đáp án</summary>

1. Vì domain là bên **sở hữu nhu cầu**. Đặt interface ở domain và implementation ở infrastructure khiến mũi tên phụ thuộc chỉ vào trong — đúng Dependency Inversion. Đặt ngược lại thì domain phải import infrastructure.
2. **Domain object.** Trả ORM row là để hạ tầng rò rỉ ra: lazy loading, session lifecycle, và cả cấu trúc bảng sẽ dính vào business logic.
3. Ở **application layer** (use case), không ở repository. Một use case thường phải ghi nhiều aggregate trong cùng một transaction; nếu mỗi repository tự commit thì không thể đảm bảo tính nguyên tử.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Viết `InMemoryOrderRepository` cài đúng `Protocol` đã có. Dùng nó để test use case mà không cần database.

**Bài 2 (15 phút).** Thêm `find_by_customer(customer_id) -> list[Order]` vào Protocol và cài cho cả bản in-memory lẫn bản SQL. Chạy cùng một bộ test cho cả hai.

**Bài 3 (25 phút).** Cài Unit of Work: một context manager mở transaction, cấp các repository, commit khi thoát bình thường và rollback khi có lỗi.

<details>
<summary>Gợi ý bài 2</summary>

Chạy **cùng một bộ test** cho cả hai implementation là phép thử tốt nhất cho một
abstraction: nếu bộ test phải rẽ nhánh theo implementation, tức là abstraction
đang rò rỉ.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Domain import SQLAlchemy | Protocol đặt nhầm tầng | Protocol ở domain, implement ở infrastructure |
| N+1 query | Lazy loading khi lặp qua danh sách | Nạp sẵn bằng join, hoặc trả về DTO đọc riêng |
| Test chậm vì phải dựng DB | Test qua repository thật | Test use case bằng in-memory repo; test SQL repo riêng |
| Ghi thành công một phần | Mỗi repository tự commit | Đưa transaction lên application layer / Unit of Work |
| Object cũ sau khi commit | Session đã hết hạn thuộc tính | Trả domain object (thuần) thay vì ORM instance |

## Tóm tắt

- ✅ **Repository Protocol**: Abstract DB access behind Protocol.
- ✅ **In-Memory**: Fast, deterministic tests.
- ✅ **SQL**: SQLAlchemy Core for production.
- ✅ **Application layer**: Uses Protocol, not implementation.

## Tiếp theo

→ Chapter 25: **Abstract Algebra for Programmers** — Monoids, Semigroups.
