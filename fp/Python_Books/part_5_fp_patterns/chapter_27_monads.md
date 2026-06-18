# Chapter 27 — Monads

> **Bạn sẽ học được**:
> - Monad = Functor with `bind`/`flatMap`
> - Why flatMap: avoid nested containers
> - Maybe/Option monad, Result monad, List monad
> - `returns` library for Monads in Python
>
> **Yêu cầu trước**: Chapter 26 (Functors)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 27.1 — The Problem: Nested Containers

```python
from typing import Union
from dataclasses import dataclass

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def map_result(r: Result, fn) -> Result:
    match r:
        case Ok(v): return Ok(fn(v))
        case Err(): return r

def parse_int(s: str) -> Result:
    try: return Ok(int(s))
    except: return Err(f"Not a number: {s}")

# Problem: map gives us Result[Result[int]] — nested!
result = map_result(Ok("42"), parse_int)
assert result == Ok(Ok(42))  # Ok(Ok(42)) — double wrapped!

# Solution: bind/flatMap — unwraps one level
def bind(r: Result, fn) -> Result:
    match r:
        case Ok(v): return fn(v)  # fn returns Result, no double wrap
        case Err(): return r

result = bind(Ok("42"), parse_int)
assert result == Ok(42)  # Ok(42) — flat!

result = bind(Ok("abc"), parse_int)
assert result == Err("Not a number: abc")  # Short-circuit
```

## 27.2 — Chaining with bind

```python
def divide(a: int, b: int) -> Result:
    return Ok(a // b) if b != 0 else Err("Division by zero")

# Chain: parse → parse → divide
def safe_calc(a_str: str, b_str: str) -> Result:
    return bind(
        parse_int(a_str),
        lambda a: bind(
            parse_int(b_str),
            lambda b: divide(a, b)
        )
    )

assert safe_calc("10", "2") == Ok(5)
assert safe_calc("10", "0") == Err("Division by zero")
assert safe_calc("abc", "2") == Err("Not a number: abc")
```

## 27.3 — Monad Laws

```python
# Law 1: Left identity — bind(unit(a), f) == f(a)
assert bind(Ok(42), parse_int) == parse_int(42)  # both err, but same result

# Law 2: Right identity — bind(m, unit) == m
assert bind(Ok(42), Ok) == Ok(42)

# Law 3: Associativity — bind(bind(m, f), g) == bind(m, lambda x: bind(f(x), g))
# (chaining order doesn't matter)
```

## 27.4 — List Monad

```python
# List flatMap = flatten after map
def flat_map(lst: list, fn) -> list:
    result = []
    for x in lst:
        result.extend(fn(x))
    return result

# "For each number, generate its divisors"
def divisors(n: int) -> list[int]:
    return [d for d in range(1, n+1) if n % d == 0]

assert flat_map([6, 10], divisors) == [1, 2, 3, 6, 1, 2, 5, 10]

# Compare with map:
# map → [[1,2,3,6], [1,2,5,10]]  ← nested!
# flatMap → [1,2,3,6,1,2,5,10]   ← flat!
```

---

## Tóm tắt

- ✅ **Monad**: Functor + `bind` (flatMap). Chains computations in context.
- ✅ **bind**: `fn(value)` — fn itself returns wrapped value, no double-wrap.
- ✅ **map vs bind**: map wraps result, bind doesn't double-wrap.
- ✅ **Result monad**: Chain fallible computations, short-circuit on error.
- ✅ **List monad**: flatMap = map + flatten.

## Tiếp theo

→ Chapter 28: **Parser Combinators** — Building parsers from small pieces.
