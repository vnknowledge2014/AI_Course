# Chapter 21 — Workflows & Pipelines

> **Bạn sẽ học được**:
> - DDD Workflows as function pipelines
> - Input → Validate → Process → Output pattern
> - Composing domain steps
>
> **Yêu cầu trước**: Chapter 20 (Domain Modeling)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 21.1 — Workflow Pattern

```python
from dataclasses import dataclass
from typing import Union

# === Input (unvalidated) ===
@dataclass(frozen=True)
class PlaceOrderInput:
    customer_email: str
    items: list[dict]  # raw, unvalidated

# === Domain Types (validated) ===
@dataclass(frozen=True)
class ValidatedOrder:
    customer_email: str
    items: tuple[tuple[str, int, int], ...]  # (product, qty, price)

@dataclass(frozen=True)
class PricedOrder:
    customer_email: str
    items: tuple[tuple[str, int, int], ...]
    total: int

@dataclass(frozen=True)
class OrderPlacedEvent:
    order_id: str
    customer_email: str
    total: int

# === Workflow Steps (pure functions) ===
def validate_order(input: PlaceOrderInput) -> ValidatedOrder:
    if "@" not in input.customer_email:
        raise ValueError("Invalid email")
    if not input.items:
        raise ValueError("Order must have at least one item")
    items = tuple(
        (item["product"], item["quantity"], item["price"])
        for item in input.items
    )
    return ValidatedOrder(customer_email=input.customer_email, items=items)

def price_order(order: ValidatedOrder) -> PricedOrder:
    total = sum(qty * price for _, qty, price in order.items)
    return PricedOrder(
        customer_email=order.customer_email,
        items=order.items,
        total=total,
    )

def create_event(order: PricedOrder) -> OrderPlacedEvent:
    import uuid
    return OrderPlacedEvent(
        order_id=str(uuid.uuid4())[:8],
        customer_email=order.customer_email,
        total=order.total,
    )

# === Composed Workflow ===
def place_order_workflow(input: PlaceOrderInput) -> OrderPlacedEvent:
    validated = validate_order(input)
    priced = price_order(validated)
    event = create_event(priced)
    return event

# === Test ===
input_data = PlaceOrderInput(
    customer_email="an@mail.com",
    items=[
        {"product": "Coffee", "quantity": 2, "price": 35_000},
        {"product": "Cake", "quantity": 1, "price": 50_000},
    ],
)

result = place_order_workflow(input_data)
assert result.total == 120_000
assert result.customer_email == "an@mail.com"
print(f"Order placed: {result}")
```

## 21.2 — Pipe-style Workflow

```python
from functools import reduce

def pipe(value, *fns):
    return reduce(lambda acc, fn: fn(acc), fns, value)

# Same workflow, using pipe
def place_order_v2(input: PlaceOrderInput) -> OrderPlacedEvent:
    return pipe(input, validate_order, price_order, create_event)

result2 = place_order_v2(input_data)
assert result2.total == 120_000
```

---

## Tóm tắt

- ✅ **Workflow**: Input → Validate → Process → Output/Event
- ✅ **Each step**: Pure function, specific input/output type
- ✅ **Compose**: `pipe(input, step1, step2, step3)`
- ✅ **Types document the flow**: Read types = understand workflow

## Tiếp theo

→ Chapter 22: **Railway-Oriented Programming** — Error handling as data flow.
