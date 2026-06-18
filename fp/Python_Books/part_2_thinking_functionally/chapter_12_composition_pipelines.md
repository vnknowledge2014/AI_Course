# Chapter 12 — Composition & Pipelines

> **Bạn sẽ học được**:
> - Function composition: `compose(f, g)` → `f(g(x))`
> - Pipelines: `pipe(value, f, g, h)` → `h(g(f(value)))`
> - `toolz` library cho FP Python
> - `returns.pipeline` cho type-safe pipelines
> - Method chaining pattern
>
> **Yêu cầu trước**: Chapter 11 (Immutability & Purity)
> **Thời gian đọc**: ~30 phút | **Level**: Intermediate

---

## 12.1 — Function Composition

```python
from typing import Callable, TypeVar

A = TypeVar("A")
B = TypeVar("B")
C = TypeVar("C")

def compose(f: Callable[[B], C], g: Callable[[A], B]) -> Callable[[A], C]:
    """compose(f, g)(x) = f(g(x))"""
    return lambda x: f(g(x))

# Example
add_one = lambda x: x + 1
double = lambda x: x * 2
to_str = str

# Compose right-to-left: add_one → double → to_str
transform = compose(to_str, compose(double, add_one))
assert transform(3) == "8"  # (3+1)*2 = 8 → "8"
```

## 12.2 — Pipe — Left-to-Right

```python
from functools import reduce

def pipe(value, *fns):
    """Apply functions left-to-right: pipe(x, f, g, h) = h(g(f(x)))"""
    return reduce(lambda acc, fn: fn(acc), fns, value)

# More readable than compose!
result = pipe(
    3,
    lambda x: x + 1,      # 4
    lambda x: x * 2,      # 8
    str,                   # "8"
)
assert result == "8"

# Real-world: data processing pipeline
def normalize(text: str) -> str:
    return text.strip().lower()

def remove_punctuation(text: str) -> str:
    return "".join(c for c in text if c.isalnum() or c == " ")

def tokenize(text: str) -> list[str]:
    return text.split()

words = pipe(
    "  Hello, World! Welcome to Python.  ",
    normalize,
    remove_punctuation,
    tokenize,
)
assert words == ["hello", "world", "welcome", "to", "python"]
```

## 12.3 — `toolz` Library

```python
# pip install toolz
from toolz import pipe as tz_pipe, curry, compose_left

@curry
def add(a: int, b: int) -> int:
    return a + b

@curry
def multiply(a: int, b: int) -> int:
    return a * b

# Curried + pipe = elegant pipelines
result = tz_pipe(
    10,
    add(5),        # 15
    multiply(2),   # 30
    str,           # "30"
)
assert result == "30"

# compose_left = left-to-right composition (like pipe but returns function)
transform = compose_left(add(5), multiply(2), str)
assert transform(10) == "30"
assert transform(0) == "10"
```

## 12.4 — Method Chaining

```python
# Python's built-in method chaining
result = (
    "  Hello, World!  "
    .strip()
    .lower()
    .replace("!", "")
    .split(", ")
)
assert result == ["hello", "world"]

# With dataclass methods returning self-type
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class QueryBuilder:
    table: str
    conditions: tuple[str, ...] = ()
    limit_val: int | None = None

    def where(self, condition: str) -> "QueryBuilder":
        return replace(self, conditions=(*self.conditions, condition))

    def limit(self, n: int) -> "QueryBuilder":
        return replace(self, limit_val=n)

    def build(self) -> str:
        sql = f"SELECT * FROM {self.table}"
        if self.conditions:
            sql += " WHERE " + " AND ".join(self.conditions)
        if self.limit_val:
            sql += f" LIMIT {self.limit_val}"
        return sql

query = (
    QueryBuilder("users")
    .where("age > 18")
    .where("active = true")
    .limit(10)
    .build()
)
assert query == "SELECT * FROM users WHERE age > 18 AND active = true LIMIT 10"
```

---

## ✅ Checkpoint 12

> 1. `compose(f, g)` = right-to-left: f(g(x))
> 2. `pipe(value, f, g, h)` = left-to-right: h(g(f(value))) — MORE READABLE
> 3. `toolz.curry` + pipe = elegant FP pipelines
> 4. Method chaining = OOP version of pipes

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Viết data pipeline: list of dicts → filter active → extract names → sort → join with comma.

<details><summary>✅ Lời giải</summary>

```python
users = [
    {"name": "An", "active": True},
    {"name": "Binh", "active": False},
    {"name": "Cuong", "active": True},
]

result = pipe(
    users,
    lambda us: [u for u in us if u["active"]],
    lambda us: [u["name"] for u in us],
    sorted,
    lambda names: ", ".join(names),
)
assert result == "An, Cuong"
```

</details>

---

## Tóm tắt

- ✅ **Compose**: Right-to-left function composition.
- ✅ **Pipe**: Left-to-right — preferred for readability.
- ✅ **`toolz`**: `curry`, `pipe`, `compose_left` — FP toolkit.
- ✅ **Method chaining**: Immutable builder pattern = OOP pipes.

## Tiếp theo

→ Chapter 13: **ADTs in Python** — Sum types, discriminated unions, `match` patterns.
