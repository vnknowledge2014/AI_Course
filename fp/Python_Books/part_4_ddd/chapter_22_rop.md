# Chapter 22 — Railway-Oriented Programming ⭐

> **Bạn sẽ học được**:
> - Result type: `Ok | Err` thay cho exceptions
> - `returns` library cho ROP trong Python
> - `bind`/`map` — chain functions that might fail
> - Error accumulation
>
> **Yêu cầu trước**: Chapter 21 (Workflows)
> **Thời gian đọc**: ~35 phút | **Level**: Advanced

---

## 22.1 — The Problem with Exceptions

```python
# ❌ Exceptions: invisible control flow
def validate_email(email: str) -> str:
    if "@" not in email:
        raise ValueError("Invalid email")
    return email.lower()

def validate_age(age: int) -> int:
    if age < 0 or age > 150:
        raise ValueError("Invalid age")
    return age

# Caller MUST know to catch — but type signature doesn't say so!
# def register(email: str, age: int) -> User:  ← can it fail? How?
```

## 22.2 — Result Type (DIY)

```python
from dataclasses import dataclass
from typing import Union, TypeVar, Generic, Callable

T = TypeVar("T")
E = TypeVar("E")

@dataclass(frozen=True)
class Ok(Generic[T]):
    value: T

@dataclass(frozen=True)
class Err(Generic[E]):
    error: E

Result = Union[Ok[T], Err[E]]

def validate_email(email: str) -> Result[str, str]:
    if "@" not in email:
        return Err("Invalid email")
    return Ok(email.lower())

def validate_age(age: int) -> Result[int, str]:
    if age < 0 or age > 150:
        return Err(f"Invalid age: {age}")
    return Ok(age)

# Now the type signature TELLS you it can fail!
# And HOW it fails (str error message)

assert validate_email("an@mail.com") == Ok("an@mail.com")
assert validate_email("bad") == Err("Invalid email")
assert validate_age(25) == Ok(25)
assert validate_age(-1) == Err("Invalid age: -1")
```

## 22.3 — bind/chain — Railway Composition

```python
def bind(result: Result, fn: Callable) -> Result:
    """If Ok, apply fn. If Err, short-circuit."""
    match result:
        case Ok(value):
            return fn(value)
        case Err():
            return result

def map_result(result: Result, fn: Callable) -> Result:
    """If Ok, transform value. If Err, pass through."""
    match result:
        case Ok(value):
            return Ok(fn(value))
        case Err():
            return result

# === Railway pipeline ===
@dataclass(frozen=True)
class User:
    email: str
    age: int

def create_user(email: str, age: int) -> Result[User, str]:
    result = validate_email(email)
    result = bind(result, lambda e: 
        map_result(validate_age(age), lambda a: User(email=e, age=a))
    )
    return result

assert create_user("an@mail.com", 25) == Ok(User("an@mail.com", 25))
assert create_user("bad", 25) == Err("Invalid email")
assert create_user("an@mail.com", -1) == Err("Invalid age: -1")
```

## 22.4 — Using `returns` Library

```python
# pip install returns
from returns.result import Result, Success, Failure, safe
from returns.pipeline import flow
from returns.pointfree import bind as rbind

@safe  # Wraps exceptions into Result
def parse_int(value: str) -> int:
    return int(value)

@safe
def divide(a: int, b: int) -> float:
    return a / b

# parse_int("10") → Success(10)
# parse_int("abc") → Failure(ValueError(...))

assert parse_int("10") == Success(10)
assert isinstance(parse_int("abc"), Failure)

# Pipeline with returns
def safe_divide_strings(a: str, b: str) -> Result[float, Exception]:
    return parse_int(a).bind(
        lambda x: parse_int(b).bind(
            lambda y: divide(x, y)
        )
    )

assert safe_divide_strings("10", "2") == Success(5.0)
assert isinstance(safe_divide_strings("10", "0"), Failure)
assert isinstance(safe_divide_strings("abc", "2"), Failure)
```

---

## ✅ Checkpoint 22

> 1. Result = `Ok(value) | Err(error)` — errors as data, not exceptions
> 2. `bind` = chain functions that return Result (short-circuit on error)
> 3. `map` = transform Ok value, pass through Err
> 4. `returns` library = production-ready ROP for Python

---

## Tóm tắt

- ✅ **Result type**: Explicit error handling in type signature.
- ✅ **Railway**: Ok track (success) and Err track (failure).
- ✅ **bind/chain**: Compose functions that might fail.
- ✅ **`returns` library**: `Success`, `Failure`, `@safe`, `.bind()`.

## Tiếp theo

→ Chapter 23: **Serialization & Anti-Corruption Layer** — Pydantic ↔ Domain, ACL.
