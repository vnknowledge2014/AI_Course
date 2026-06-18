# Chapter 19 — Functional Architecture

> **Bạn sẽ học được**:
> - Hexagonal Architecture (Ports & Adapters)
> - Onion Architecture
> - IO at the edges, pure logic at the core
> - Python project structure for DDD
>
> **Yêu cầu trước**: Chapter 18 (Intro DDD)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 19.1 — Hexagonal Architecture

```
         ┌──────────────────────────┐
         │     Application Core     │
         │   (Pure Domain Logic)    │
         │                          │
         │  ┌────────────────────┐  │
         │  │   Domain Model     │  │
         │  │  (Types, Rules)    │  │
         │  └────────────────────┘  │
         │  ┌────────────────────┐  │
         │  │   Domain Services  │  │
         │  │  (Workflows)       │  │
         │  └────────────────────┘  │
         │                          │
    Port ◄──────────────────────────► Port
    (Protocol)                  (Protocol)
         │                          │
    Adapter                    Adapter
    (FastAPI)                  (PostgreSQL)
    (CLI)                      (Redis)
         └──────────────────────────┘
```

## 19.2 — Implementation in Python

```python
from typing import Protocol
from dataclasses import dataclass, replace
from typing import Union

# === DOMAIN LAYER (Pure — no IO, no frameworks) ===

@dataclass(frozen=True)
class Order:
    id: str
    customer: str
    items: tuple[str, ...]
    total: int
    status: str = "pending"

# Port = Protocol (interface)
class OrderRepository(Protocol):
    def find_by_id(self, order_id: str) -> Order | None: ...
    def save(self, order: Order) -> None: ...

class PaymentGateway(Protocol):
    def charge(self, amount: int, method: str) -> bool: ...

# Domain Service (pure function)
def confirm_order(order: Order) -> Order:
    if order.status != "pending":
        raise ValueError(f"Cannot confirm {order.status} order")
    return replace(order, status="confirmed")

# === APPLICATION LAYER (orchestrates, uses ports) ===
def confirm_and_charge(
    order_id: str,
    payment_method: str,
    repo: OrderRepository,
    payment: PaymentGateway,
) -> Order:
    order = repo.find_by_id(order_id)
    if order is None:
        raise ValueError(f"Order {order_id} not found")

    confirmed = confirm_order(order)  # Pure domain logic
    success = payment.charge(confirmed.total, payment_method)  # IO at edge

    if not success:
        raise ValueError("Payment failed")

    repo.save(confirmed)  # IO at edge
    return confirmed

# === ADAPTER LAYER (implements ports) ===
class InMemoryOrderRepo:
    def __init__(self) -> None:
        self._orders: dict[str, Order] = {}

    def find_by_id(self, order_id: str) -> Order | None:
        return self._orders.get(order_id)

    def save(self, order: Order) -> None:
        self._orders[order.id] = order

class FakePaymentGateway:
    def charge(self, amount: int, method: str) -> bool:
        return True  # Always succeeds in tests

# === TEST ===
repo = InMemoryOrderRepo()
repo.save(Order("ORD-1", "An", ("Coffee", "Cake"), 120_000))

result = confirm_and_charge("ORD-1", "card", repo, FakePaymentGateway())
assert result.status == "confirmed"
```

## 19.3 — Project Structure

```
cafe_order/
├── src/cafe_order/
│   ├── domain/           # PURE — no imports from outside
│   │   ├── __init__.py
│   │   ├── order.py      # Order dataclass, OrderStatus
│   │   ├── payment.py    # Payment types
│   │   └── services.py   # Pure domain functions
│   ├── ports/            # PROTOCOLS — interfaces
│   │   ├── __init__.py
│   │   ├── repository.py # OrderRepository Protocol
│   │   └── gateway.py    # PaymentGateway Protocol
│   ├── adapters/         # IO — implementations
│   │   ├── __init__.py
│   │   ├── postgres.py   # PostgresOrderRepo
│   │   ├── redis_cache.py
│   │   └── stripe.py     # StripePaymentGateway
│   ├── application/      # Orchestration
│   │   ├── __init__.py
│   │   └── use_cases.py  # confirm_and_charge, etc.
│   └── api/              # Entry point
│       ├── __init__.py
│       └── routes.py     # FastAPI routes
└── tests/
    ├── test_domain.py    # Unit tests (pure, fast)
    ├── test_use_cases.py # Integration (with fakes)
    └── test_api.py       # E2E (with test client)
```

---

## ✅ Checkpoint 19

> 1. Domain = pure, no IO, no framework imports
> 2. Ports = Protocols (interfaces)
> 3. Adapters = implementations (DB, API, cache)
> 4. IO at edges, pure logic at core

---

## Tóm tắt

- ✅ **Hexagonal**: Core ← Ports → Adapters.
- ✅ **Domain**: Pure types + pure functions. No IO.
- ✅ **Ports**: `Protocol` — define WHAT, not HOW.
- ✅ **Adapters**: Implement ports for specific tech.
- ✅ **Testing**: Swap adapters with in-memory fakes.

## Tiếp theo

→ Chapter 20: **Domain Modeling** — Value Objects, Entities, Aggregates.
