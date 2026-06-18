# Chapter 16 — GoF → FP Translation

> **Bạn sẽ học được**:
> - Classical GoF patterns → FP equivalents
> - Strategy = HOF, Command = dataclass, Visitor = match
> - Factory = classmethod, Decorator = @decorator
> - Observer = callback/event, Middleware = ASGI
>
> **Yêu cầu trước**: Chapter 15 (Protocols)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## Strategy → Higher-Order Function

```python
from typing import Callable

# OOP Strategy: interface + classes
# FP Strategy: just pass a function!

PricingStrategy = Callable[[float], float]

def no_discount(price: float) -> float:
    return price

def percent_off(percent: float) -> PricingStrategy:
    return lambda price: price * (1 - percent / 100)

def flat_off(amount: float) -> PricingStrategy:
    return lambda price: max(0, price - amount)

def calculate(price: float, strategy: PricingStrategy) -> float:
    return strategy(price)

assert calculate(100_000, no_discount) == 100_000
assert calculate(100_000, percent_off(10)) == 90_000
assert calculate(100_000, flat_off(15_000)) == 85_000
```

## Command → Dataclass

```python
from dataclasses import dataclass
from typing import Union

@dataclass(frozen=True)
class AddItem:
    product: str
    quantity: int

@dataclass(frozen=True)
class RemoveItem:
    product: str

@dataclass(frozen=True)
class Checkout:
    payment_method: str

Command = Union[AddItem, RemoveItem, Checkout]

def execute(state: dict, cmd: Command) -> dict:
    match cmd:
        case AddItem(product=p, quantity=q):
            new_state = dict(state)
            new_state[p] = new_state.get(p, 0) + q
            return new_state
        case RemoveItem(product=p):
            new_state = dict(state)
            new_state.pop(p, None)
            return new_state
        case Checkout():
            return {}  # clear cart

cart: dict = {}
cart = execute(cart, AddItem("Coffee", 2))
cart = execute(cart, AddItem("Tea", 1))
assert cart == {"Coffee": 2, "Tea": 1}
cart = execute(cart, RemoveItem("Tea"))
assert cart == {"Coffee": 2}
```

## Visitor → Pattern Match

```python
from dataclasses import dataclass
from typing import Union
import math

@dataclass(frozen=True)
class Circle:
    radius: float

@dataclass(frozen=True)
class Rect:
    w: float
    h: float

Shape = Union[Circle, Rect]

# Instead of Visitor interface with visit_circle, visit_rect...
# Just use match!
def area(s: Shape) -> float:
    match s:
        case Circle(r): return math.pi * r * r
        case Rect(w, h): return w * h

def perimeter(s: Shape) -> float:
    match s:
        case Circle(r): return 2 * math.pi * r
        case Rect(w, h): return 2 * (w + h)

c = Circle(5)
assert abs(area(c) - 78.54) < 0.01
assert abs(perimeter(c) - 31.42) < 0.01
```

## Decorator → Python @decorator (native!)

```python
import time
from typing import Callable, TypeVar
from functools import wraps

F = TypeVar("F", bound=Callable)

def timed(fn: F) -> F:
    @wraps(fn)
    def wrapper(*args, **kwargs):
        start = time.time()
        result = fn(*args, **kwargs)
        elapsed = time.time() - start
        print(f"{fn.__name__} took {elapsed:.4f}s")
        return result
    return wrapper  # type: ignore

def retry(max_attempts: int = 3):
    def decorator(fn: F) -> F:
        @wraps(fn)
        def wrapper(*args, **kwargs):
            for attempt in range(max_attempts):
                try:
                    return fn(*args, **kwargs)
                except Exception as e:
                    if attempt == max_attempts - 1:
                        raise
            return None
        return wrapper  # type: ignore
    return decorator

@timed
def slow_add(a: int, b: int) -> int:
    return a + b

assert slow_add(3, 4) == 7
```

---

## Tóm tắt — GoF → FP Cheat Sheet

| GoF Pattern | FP Equivalent | Python |
|-------------|---------------|--------|
| **Strategy** | Higher-Order Function | `Callable` parameter |
| **Command** | Discriminated Union | `Union[AddItem, RemoveItem]` + `match` |
| **Visitor** | Pattern Match | `match shape` |
| **Factory** | Smart Constructor | `@classmethod` / Pydantic |
| **Decorator** | Function wrapper | `@decorator` (native!) |
| **Observer** | Callback / Event | `Callable` list or `asyncio` |
| **Iterator** | Generator | `yield` (native!) |
| **Middleware** | Function composition | ASGI middleware |

## Tiếp theo

→ Chapter 17: **CQRS & Event Sourcing** — tách read/write, event sourcing với `reduce`.
