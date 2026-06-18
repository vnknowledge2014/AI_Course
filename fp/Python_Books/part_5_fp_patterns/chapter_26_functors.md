# Chapter 26 — Functors

> **Bạn sẽ học được**:
> - Functor = "mappable container"
> - `map` over `list`, `Optional`, `Result`
> - Functor laws: identity and composition
> - Practical applications
>
> **Yêu cầu trước**: Chapter 25 (Algebra)
> **Thời gian đọc**: ~25 phút | **Level**: Advanced

---

## 26.1 — What is a Functor?

```python
from typing import Callable, TypeVar, Union
from dataclasses import dataclass

T = TypeVar("T")
U = TypeVar("U")

# Functor = container with a .map() method
# map :: (T → U) → F[T] → F[U]

# List is a Functor
numbers = [1, 2, 3, 4, 5]
doubled = list(map(lambda x: x * 2, numbers))  # map over list
assert doubled == [2, 4, 6, 8, 10]

# Optional is a Functor
def map_optional(value: T | None, fn: Callable[[T], U]) -> U | None:
    return fn(value) if value is not None else None

assert map_optional(5, lambda x: x * 2) == 10
assert map_optional(None, lambda x: x * 2) is None

# Result is a Functor
@dataclass(frozen=True)
class Ok:
    value: object

@dataclass(frozen=True)
class Err:
    error: str

Result = Union[Ok, Err]

def map_result(result: Result, fn: Callable) -> Result:
    match result:
        case Ok(value=v):
            return Ok(fn(v))
        case Err():
            return result

assert map_result(Ok(5), lambda x: x * 2) == Ok(10)
assert map_result(Err("fail"), lambda x: x * 2) == Err("fail")
```

## 26.2 — Functor Laws

```python
# Law 1: Identity — map(id) == id
identity = lambda x: x
assert list(map(identity, [1, 2, 3])) == [1, 2, 3]
assert map_optional(5, identity) == 5
assert map_result(Ok(5), identity) == Ok(5)

# Law 2: Composition — map(f ∘ g) == map(f) ∘ map(g)
f = lambda x: x + 1
g = lambda x: x * 2
composed = lambda x: f(g(x))

data = [1, 2, 3]
assert list(map(composed, data)) == list(map(f, map(g, data)))
```

---

## Tóm tắt

- ✅ **Functor**: Container with `map()` — transforms contents.
- ✅ **Examples**: `list`, `Optional`, `Result` — all functors.
- ✅ **Laws**: Identity (map(id) = id), Composition (map(f∘g) = map(f) ∘ map(g)).
- ✅ **Practical**: Transform data inside containers uniformly.

## Tiếp theo

→ Chapter 27: **Monads** — `bind`/`flatMap`, do-notation.
