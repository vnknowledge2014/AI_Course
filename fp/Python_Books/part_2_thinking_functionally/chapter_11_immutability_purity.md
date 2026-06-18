# Chapter 11 — Immutability & Purity

> **Bạn sẽ học được**:
> - Tại sao immutability quan trọng — concurrency, testability, reasoning
> - Python immutability strategies: `frozen=True`, `tuple`, `Final`, `MappingProxyType`
> - Pure functions — no side effects, deterministic output
> - Side effects — IO, mutation, exceptions
>
> **Yêu cầu trước**: Chapter 10 (Modules)
> **Thời gian đọc**: ~30 phút | **Level**: Intermediate

---

## 11.1 — Why Immutability?

```python
# ❌ PROBLEM: Shared mutable state
def add_tax(prices: list[float], rate: float) -> list[float]:
    for i in range(len(prices)):
        prices[i] *= (1 + rate)  # MUTATES input!
    return prices

original = [100.0, 200.0, 300.0]
with_tax = add_tax(original, 0.1)
# SURPRISE: original is now [110.0, 220.0, 330.0]!
# Caller didn't expect their data to change.

# ✅ SOLUTION: Return new data
def add_tax_pure(prices: list[float], rate: float) -> list[float]:
    return [p * (1 + rate) for p in prices]  # NEW list

original2 = [100.0, 200.0, 300.0]
with_tax2 = add_tax_pure(original2, 0.1)
assert original2 == [100.0, 200.0, 300.0]  # UNCHANGED
assert with_tax2 == [110.0, 220.0, 330.0]
```

## 11.2 — Immutability Strategies in Python

```python
from dataclasses import dataclass, replace
from types import MappingProxyType
from typing import Final

# 1. frozen dataclass
@dataclass(frozen=True)
class Config:
    host: str
    port: int
    debug: bool = False

cfg = Config("localhost", 8080)
# cfg.port = 3000  ← FrozenInstanceError

cfg2 = replace(cfg, port=3000)
assert cfg.port == 8080

# 2. tuple instead of list
items: tuple[str, ...] = ("coffee", "tea", "juice")
# items.append("milk")  ← AttributeError

# 3. Final
MAX_CONNECTIONS: Final = 100
# MAX_CONNECTIONS = 200  ← mypy error

# 4. MappingProxyType (read-only dict view)
settings = MappingProxyType({"theme": "dark", "lang": "vi"})
# settings["theme"] = "light"  ← TypeError
```

## 11.3 — Pure Functions

```python
# PURE: same input → same output, no side effects
def add(a: int, b: int) -> int:
    return a + b  # ✅ Pure

def discount(price: float, percent: float) -> float:
    return price * (1 - percent / 100)  # ✅ Pure

# IMPURE: depends on external state or causes side effects
import random
import datetime

def impure_random() -> int:
    return random.randint(1, 100)  # ❌ Different output each call

def impure_now() -> str:
    return str(datetime.datetime.now())  # ❌ Different output each call

def impure_print(msg: str) -> None:
    print(msg)  # ❌ Side effect: IO

# Make impure → pure by injecting dependencies
from typing import Callable

def greet(name: str, get_time: Callable[[], str] = lambda: "morning") -> str:
    return f"Good {get_time()}, {name}!"

# In production: greet("An", lambda: "afternoon")
# In tests: greet("An", lambda: "morning") ← deterministic!
assert greet("An") == "Good morning, An!"
assert greet("An", lambda: "evening") == "Good evening, An!"
```

## 11.4 — Functional Patterns for State

```python
from dataclasses import dataclass, replace
from typing import Union

# State machine with immutable transitions
@dataclass(frozen=True)
class Draft:
    content: str

@dataclass(frozen=True)
class Published:
    content: str
    published_at: str

@dataclass(frozen=True)
class Archived:
    content: str
    reason: str

Article = Union[Draft, Published, Archived]

def publish(article: Article) -> Article:
    match article:
        case Draft(content):
            return Published(content=content, published_at="2024-01-01")
        case _:
            raise ValueError("Can only publish drafts")

def archive(article: Article, reason: str) -> Article:
    match article:
        case Published(content=c):
            return Archived(content=c, reason=reason)
        case _:
            raise ValueError("Can only archive published articles")

draft = Draft("Hello World")
pub = publish(draft)
assert isinstance(pub, Published)
archived = archive(pub, "Outdated")
assert isinstance(archived, Archived)
assert archived.reason == "Outdated"
```

---

## ✅ Checkpoint 11

> 1. Immutable data: `frozen=True`, `tuple`, `Final`, `MappingProxyType`
> 2. Pure functions: same input → same output, no side effects
> 3. Inject dependencies to make impure code testable
> 4. State machines: immutable transitions return NEW state

---

## 🏋️ Bài tập

**Bài 1**: Refactor impure function thành pure:
```python
# Impure
total_orders = 0
def process_order(amount):
    global total_orders
    total_orders += 1
    print(f"Order #{total_orders}: {amount}")
```

<details><summary>✅ Lời giải</summary>

```python
@dataclass(frozen=True)
class OrderState:
    count: int
    log: tuple[str, ...]

def process_order(state: OrderState, amount: float) -> OrderState:
    new_count = state.count + 1
    entry = f"Order #{new_count}: {amount}"
    return OrderState(count=new_count, log=(*state.log, entry))

s0 = OrderState(0, ())
s1 = process_order(s0, 100)
s2 = process_order(s1, 200)
assert s2.count == 2
assert len(s2.log) == 2
```

</details>

---

## Tóm tắt

- ✅ **Immutability**: Prevents shared mutable state bugs.
- ✅ **Pure functions**: Deterministic, testable, composable.
- ✅ **DI for purity**: Inject time, random, IO as parameters.
- ✅ **State machines**: Immutable transitions = safe, predictable state changes.

## Tiếp theo

→ Chapter 12: **Composition & Pipelines** — `toolz.pipe`, `returns.pipeline`, function chaining.
