# Chapter 13 — ADTs in Python

> **Bạn sẽ học được**:
> - Sum types = Union of dataclasses
> - Product types = dataclass fields
> - Discriminated unions pattern
> - `match` for exhaustive handling
> - "Make illegal states unrepresentable"
>
> **Yêu cầu trước**: Chapter 12 (Composition)
> **Thời gian đọc**: ~30 phút | **Level**: Intermediate

---

## 13.1 — Product Types (AND)

```python
from dataclasses import dataclass

# Product: Person HAS name AND age AND email
@dataclass(frozen=True)
class Person:
    name: str      # AND
    age: int       # AND
    email: str     # AND

p = Person("An", 25, "an@mail.com")
assert p.name == "An" and p.age == 25
```

## 13.2 — Sum Types (OR)

```python
from dataclasses import dataclass
from typing import Union

# Sum: Shape IS Circle OR Rectangle OR Triangle
@dataclass(frozen=True)
class Circle:
    radius: float

@dataclass(frozen=True)
class Rectangle:
    width: float
    height: float

@dataclass(frozen=True)
class Triangle:
    base: float
    height: float

Shape = Union[Circle, Rectangle, Triangle]

def area(shape: Shape) -> float:
    match shape:
        case Circle(radius=r):
            return 3.14159 * r * r
        case Rectangle(width=w, height=h):
            return w * h
        case Triangle(base=b, height=h):
            return 0.5 * b * h

assert abs(area(Circle(5)) - 78.54) < 0.01
assert area(Rectangle(3, 4)) == 12.0
assert area(Triangle(6, 4)) == 12.0
```

## 13.3 — Real-World ADTs

```python
from dataclasses import dataclass
from typing import Union

# === Payment System ===
@dataclass(frozen=True)
class Cash:
    amount: int

@dataclass(frozen=True)
class CreditCard:
    number: str
    cvv: str
    amount: int

@dataclass(frozen=True)
class BankTransfer:
    account: str
    amount: int

Payment = Union[Cash, CreditCard, BankTransfer]

def process_payment(payment: Payment) -> str:
    match payment:
        case Cash(amount=a):
            return f"Cash: {a:,}đ"
        case CreditCard(number=n, amount=a):
            return f"Card ***{n[-4:]}: {a:,}đ"
        case BankTransfer(account=acc, amount=a):
            return f"Transfer to {acc}: {a:,}đ"

assert process_payment(Cash(50_000)) == "Cash: 50,000đ"

# === Result Type (Either) ===
@dataclass(frozen=True)
class Ok:
    value: object

@dataclass(frozen=True)
class Err:
    error: str

Result = Union[Ok, Err]

def divide(a: float, b: float) -> Result:
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)

match divide(10, 3):
    case Ok(value=v):
        assert abs(v - 3.333) < 0.01
    case Err(error=e):
        assert False, f"Unexpected error: {e}"

match divide(10, 0):
    case Ok():
        assert False
    case Err(error=e):
        assert e == "Division by zero"
```

## 13.4 — State Machines

```python
from dataclasses import dataclass
from typing import Union

@dataclass(frozen=True)
class EmptyCart:
    pass

@dataclass(frozen=True)
class ActiveCart:
    items: tuple[str, ...]

@dataclass(frozen=True)
class PaidCart:
    items: tuple[str, ...]
    payment_id: str

Cart = Union[EmptyCart, ActiveCart, PaidCart]

def add_item(cart: Cart, item: str) -> Cart:
    match cart:
        case EmptyCart():
            return ActiveCart(items=(item,))
        case ActiveCart(items=items):
            return ActiveCart(items=(*items, item))
        case PaidCart():
            raise ValueError("Cannot add items to paid cart")

def pay(cart: Cart, payment_id: str) -> Cart:
    match cart:
        case ActiveCart(items=items):
            return PaidCart(items=items, payment_id=payment_id)
        case EmptyCart():
            raise ValueError("Cannot pay for empty cart")
        case PaidCart():
            raise ValueError("Already paid")

# Valid flow: Empty → Active → Paid
cart = EmptyCart()
cart = add_item(cart, "Coffee")
cart = add_item(cart, "Cake")
cart = pay(cart, "PAY-001")
assert isinstance(cart, PaidCart)
assert cart.items == ("Coffee", "Cake")

# Invalid flow: paying empty cart → Error!
try:
    pay(EmptyCart(), "PAY-002")
    assert False
except ValueError:
    pass  # Expected!
```

---

## ✅ Checkpoint 13

> 1. Product type (dataclass) = AND: all fields must be present
> 2. Sum type (Union) = OR: exactly one variant at a time
> 3. `match` = exhaustive pattern matching on sum types
> 4. State machines with sum types prevent invalid transitions

---

## Tóm tắt

- ✅ **Product types**: `@dataclass(frozen=True)` — AND of fields.
- ✅ **Sum types**: `Union[A, B, C]` — OR of variants.
- ✅ **Pattern matching**: `match/case` for exhaustive handling.
- ✅ **State machines**: Sum types + match = type-safe state transitions.
- ✅ **"Make illegal states unrepresentable"** — the core FP design principle.

## Tiếp theo

→ Chapter 14: **Validation with Pydantic** — smart constructors, auto-validation, serialization.
