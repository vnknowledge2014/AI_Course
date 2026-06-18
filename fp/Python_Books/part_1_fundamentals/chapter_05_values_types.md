# Chapter 5 — Values, Types & Type Hints

> **Bạn sẽ học được**:
> - Python type system deep dive: primitives, collections, generics
> - `Final`, `Literal`, `TypeAlias`, `NewType`
> - `mypy --strict` — biến Python thành statically typed
> - Type narrowing với `isinstance` và `assert`
>
> **Yêu cầu trước**: Chapter 4 (Getting Started)
> **Thời gian đọc**: ~30 phút | **Level**: Beginner

---

## 5.1 — Primitive Types

```python
# Python primitives
name: str = "Python"
age: int = 33
pi: float = 3.14159
is_active: bool = True
nothing: None = None

# Type inference — mypy tự suy ra type
x = 42          # mypy biết x: int
y = "hello"     # mypy biết y: str
z = [1, 2, 3]   # mypy biết z: list[int]

# ❌ Type mismatch — mypy sẽ báo lỗi
# x = "not a number"  # error: Incompatible types in assignment

print(f"name={name}, age={age}, pi={pi:.2f}")
```

## 5.2 — Collection Types

```python
# List — mutable, ordered
names: list[str] = ["An", "Binh"]
names.append("Cuong")
assert len(names) == 3

# Tuple — immutable, ordered
coords: tuple[float, float] = (3.0, 4.0)
# coords[0] = 5.0  ← TypeError

# Dict — mutable, key-value
scores: dict[str, int] = {"An": 95, "Binh": 87}
scores["Cuong"] = 92

# Set — mutable, unique, unordered
tags: set[str] = {"python", "fp"}
tags.add("ddd")

# Frozenset — immutable set
immutable_tags: frozenset[str] = frozenset({"python", "fp"})

# Nested types
matrix: list[list[int]] = [[1, 2], [3, 4]]
user_roles: dict[str, list[str]] = {"An": ["admin", "user"]}

print(f"scores = {scores}")
```

## 5.3 — Advanced Type Hints

### Final — Constants

```python
from typing import Final

MAX_RETRIES: Final = 3
API_URL: Final[str] = "https://api.example.com"

# MAX_RETRIES = 5  ← mypy error: Cannot assign to final name

print(f"MAX_RETRIES = {MAX_RETRIES}")
```

### Literal — Specific values

```python
from typing import Literal

Direction = Literal["north", "south", "east", "west"]

def move(direction: Direction) -> str:
    return f"Moving {direction}"

assert move("north") == "Moving north"
# move("up")  ← mypy error: "up" is not a valid Direction
```

### TypeAlias — Type names

```python
from typing import TypeAlias

UserId: TypeAlias = int
Email: TypeAlias = str
UserMap: TypeAlias = dict[UserId, Email]

users: UserMap = {1: "an@mail.com", 2: "binh@mail.com"}
assert users[1] == "an@mail.com"
```

### NewType — Distinct types

```python
from typing import NewType

UserId = NewType("UserId", int)
OrderId = NewType("OrderId", int)

def get_user(user_id: UserId) -> str:
    return f"User {user_id}"

uid = UserId(42)
# oid = OrderId(42)
# get_user(oid)  ← mypy error: OrderId ≠ UserId

assert get_user(uid) == "User 42"
```

## 5.4 — Union & Optional

```python
from typing import Union

# Python 3.10+ syntax
def parse_input(value: str | int) -> str:
    match value:
        case str():
            return value.upper()
        case int():
            return str(value)

assert parse_input("hello") == "HELLO"
assert parse_input(42) == "42"

# Optional = Union[X, None]
def find(name: str) -> str | None:
    db = {"An": "an@mail.com"}
    return db.get(name)

result = find("An")
if result is not None:  # type narrowing
    assert result == "an@mail.com"
```

---

## ✅ Checkpoint 5

> 1. `Final` = constant (không reassign). `Literal` = specific values only.
> 2. `NewType` = distinct types (UserId ≠ OrderId dù cả hai là int).
> 3. `str | None` = Optional. Luôn check `is not None` trước khi dùng.
> 4. `mypy --strict` bắt tất cả type errors tại compile time.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Tạo `NewType` cho `Email` và `Password`. Viết function nhận `Email` — xác nhận mypy báo lỗi nếu truyền `Password`.

**Bài 2** (10 phút): Viết function `safe_divide(a: float, b: float) -> float | None` trả `None` khi chia cho 0. Thêm type narrowing.

<details><summary>✅ Lời giải</summary>

```python
def safe_divide(a: float, b: float) -> float | None:
    return a / b if b != 0 else None

result = safe_divide(10, 3)
if result is not None:
    assert abs(result - 3.333) < 0.01

assert safe_divide(10, 0) is None
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `mypy: Name "X" is not defined` | Import thiếu | `from typing import X` |
| `Incompatible types in assignment` | Gán sai type | Kiểm tra lại type annotation |
| `NewType` runtime behavior | `NewType` chỉ là hint, runtime vẫn là base type | Đúng — chỉ mypy enforce |

---

## Tóm tắt

- ✅ **Primitives**: `str`, `int`, `float`, `bool`, `None`.
- ✅ **Collections**: `list[T]`, `dict[K, V]`, `set[T]`, `tuple[T, ...]`.
- ✅ **Advanced**: `Final`, `Literal`, `TypeAlias`, `NewType`.
- ✅ **Union**: `A | B` (3.10+). Optional: `X | None`.
- ✅ **mypy --strict**: Biến Python thành statically typed.

## Tiếp theo

→ Chapter 6: **Control Flow & Pattern Matching** — `if/elif`, `for`, `while`, `match/case` (3.10+), guards, destructuring.
