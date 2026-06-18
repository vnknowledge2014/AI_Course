# Chapter 32 — Capstone: Domain Application ⭐

> **Bạn sẽ học được**:
> - Kết hợp TẤT CẢ patterns đã học
> - Full Order-Taking System: domain → API → tests
> - DDD + FP + TDD trong một ứng dụng hoàn chỉnh
>
> **Yêu cầu trước**: Chapter 31 (FastAPI)
> **Thời gian đọc**: ~45 phút | **Level**: Principal

---

## 32.1 — Domain Layer (Pure)

```python
from dataclasses import dataclass, replace
from typing import Union
from enum import Enum

# Value Objects
@dataclass(frozen=True)
class Email:
    value: str
    def __post_init__(self):
        if "@" not in self.value:
            raise ValueError(f"Invalid email: {self.value}")

@dataclass(frozen=True)
class Money:
    amount: int
    currency: str = "VND"
    def __post_init__(self):
        if self.amount < 0:
            raise ValueError("Negative amount")
    def add(self, other: "Money") -> "Money":
        return Money(self.amount + other.amount, self.currency)
    @staticmethod
    def zero() -> "Money":
        return Money(0)

# Entities
@dataclass(frozen=True)
class OrderItem:
    product: str
    price: Money
    quantity: int = 1
    @property
    def subtotal(self) -> Money:
        return Money(self.price.amount * self.quantity)

@dataclass(frozen=True)
class Order:
    id: str
    customer_email: Email
    items: tuple[OrderItem, ...]
    status: str = "draft"

    @property
    def total(self) -> Money:
        from functools import reduce
        return reduce(lambda acc, i: acc.add(i.subtotal), self.items, Money.zero())

    def add_item(self, item: OrderItem) -> "Order":
        return replace(self, items=(*self.items, item))

    def confirm(self) -> "Order":
        if self.status != "draft":
            raise ValueError(f"Cannot confirm {self.status} order")
        if not self.items:
            raise ValueError("Cannot confirm empty order")
        return replace(self, status="confirmed")

    def ship(self) -> "Order":
        if self.status != "confirmed":
            raise ValueError(f"Cannot ship {self.status} order")
        return replace(self, status="shipped")
```

## 32.2 — Ports (Protocols)

```python
from typing import Protocol

class OrderRepository(Protocol):
    def find_by_id(self, order_id: str) -> Order | None: ...
    def save(self, order: Order) -> None: ...

class NotificationService(Protocol):
    def send(self, email: str, message: str) -> None: ...
```

## 32.3 — Application Layer (Use Cases)

```python
import uuid

def place_order(
    customer_email: str,
    items: list[dict],
    repo: OrderRepository,
    notifier: NotificationService,
) -> Order:
    email = Email(customer_email)
    order_items = tuple(
        OrderItem(i["product"], Money(i["price"]), i.get("quantity", 1))
        for i in items
    )
    order = Order(
        id=str(uuid.uuid4())[:8],
        customer_email=email,
        items=order_items,
        status="draft",
    )
    confirmed = order.confirm()
    repo.save(confirmed)
    notifier.send(email.value, f"Order {confirmed.id} confirmed! Total: {confirmed.total.amount}đ")
    return confirmed
```

## 32.4 — Adapters

```python
class InMemoryOrderRepo:
    def __init__(self): self._store: dict[str, Order] = {}
    def find_by_id(self, order_id: str) -> Order | None:
        return self._store.get(order_id)
    def save(self, order: Order) -> None:
        self._store[order.id] = order

class ConsoleNotifier:
    def __init__(self): self.sent: list[tuple[str, str]] = []
    def send(self, email: str, message: str) -> None:
        self.sent.append((email, message))
        print(f"📧 To: {email} — {message}")
```

## 32.5 — Tests

```python
import pytest

@pytest.fixture
def repo():
    return InMemoryOrderRepo()

@pytest.fixture
def notifier():
    return ConsoleNotifier()

def test_place_order(repo, notifier):
    order = place_order(
        "an@mail.com",
        [{"product": "Coffee", "price": 35_000, "quantity": 2}],
        repo, notifier,
    )
    assert order.status == "confirmed"
    assert order.total == Money(70_000)
    assert repo.find_by_id(order.id) is not None
    assert len(notifier.sent) == 1

def test_invalid_email(repo, notifier):
    with pytest.raises(ValueError, match="Invalid email"):
        place_order("bad", [{"product": "Coffee", "price": 35_000}], repo, notifier)

def test_empty_order(repo, notifier):
    with pytest.raises(ValueError, match="Cannot confirm empty"):
        place_order("an@mail.com", [], repo, notifier)
```

---

## Tóm tắt

- ✅ **Full stack**: Domain → Ports → Adapters → Use Cases → Tests.
- ✅ **DDD**: Value Objects, Entities, Aggregates, Workflows.
- ✅ **FP**: Immutable data, pure domain logic, Result types.
- ✅ **TDD**: Tests drive design, fakes for adapters.

## Tiếp theo

→ Chapter 33: **Database & SQL** — SQLAlchemy, Alembic migrations.
