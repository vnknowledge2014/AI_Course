# Chapter 9 — Dataclasses & Structured Data ⭐

> **Bạn sẽ học được**:
> - `@dataclass` — Python's answer to structs/records
> - `frozen=True` — immutable dataclasses = Value Objects
> - `__post_init__` — validation khi tạo
> - `field()`, `dataclasses.replace()`, `asdict()`
> - So sánh: dataclass vs NamedTuple vs dict vs Pydantic
>
> **Yêu cầu trước**: Chapter 8 (Data Structures)
> **Thời gian đọc**: ~35 phút | **Level**: Beginner
> **Đây là chapter quan trọng** — nền tảng cho toàn bộ Domain Modeling ở Part IV.

---

## 9.1 — `@dataclass` Basics

```python
from dataclasses import dataclass, field, replace, asdict

# Mutable dataclass
@dataclass
class MutablePerson:
    name: str
    age: int

p = MutablePerson("An", 25)
assert p.name == "An"
p.age = 26  # OK — mutable
assert p.age == 26

# Auto-generated: __init__, __repr__, __eq__
p2 = MutablePerson("An", 26)
assert p == p2  # Structural equality (so sánh giá trị, không phải reference)
print(p)  # MutablePerson(name='An', age=26)
```

## 9.2 — `frozen=True` — Immutable Records ⭐

```python
@dataclass(frozen=True)
class Person:
    name: str
    age: int

p = Person("An", 25)
# p.age = 26  ← FrozenInstanceError!

# "Sửa" = tạo bản mới
p2 = replace(p, age=26)
assert p.age == 25   # Gốc không đổi
assert p2.age == 26  # Bản mới

# Hashable → dùng trong set, dict key
people = {Person("An", 25), Person("An", 25), Person("Binh", 30)}
assert len(people) == 2  # Duplicate bị loại

# Convert to dict
d = asdict(p)
assert d == {"name": "An", "age": 25}
```

## 9.3 — `__post_init__` — Validation

```python
@dataclass(frozen=True)
class Email:
    value: str

    def __post_init__(self) -> None:
        if "@" not in self.value:
            raise ValueError(f"Invalid email: {self.value}")

# Valid
e = Email("an@mail.com")
assert e.value == "an@mail.com"

# Invalid → raises immediately
try:
    Email("not-an-email")
    assert False, "Should have raised"
except ValueError as err:
    assert "Invalid email" in str(err)

print(f"Email created: {e}")
```

## 9.4 — Complex Dataclasses

```python
from dataclasses import dataclass, field, replace
from typing import Union
from enum import Enum

class OrderStatus(Enum):
    PENDING = "pending"
    CONFIRMED = "confirmed"
    SHIPPED = "shipped"

@dataclass(frozen=True)
class Money:
    amount: int
    currency: str = "VND"

    def __post_init__(self) -> None:
        if self.amount < 0:
            raise ValueError(f"Money cannot be negative: {self.amount}")

    def add(self, other: "Money") -> "Money":
        assert self.currency == other.currency, "Currency mismatch"
        return Money(self.amount + other.amount, self.currency)

@dataclass(frozen=True)
class OrderItem:
    name: str
    price: Money
    quantity: int = 1

    @property
    def subtotal(self) -> Money:
        return Money(self.price.amount * self.quantity, self.price.currency)

@dataclass(frozen=True)
class Order:
    customer: str
    items: tuple[OrderItem, ...]  # tuple, not list — truly immutable!
    status: OrderStatus = OrderStatus.PENDING

    @property
    def total(self) -> Money:
        from functools import reduce
        return reduce(
            lambda acc, item: acc.add(item.subtotal),
            self.items,
            Money(0)
        )

# Usage
order = Order(
    customer="An",
    items=(
        OrderItem("Coffee", Money(35_000), 2),
        OrderItem("Cake", Money(50_000), 1),
    ),
)

assert order.total == Money(120_000)  # 35k*2 + 50k = 120k
assert order.status == OrderStatus.PENDING

# Transition state (immutable)
confirmed = replace(order, status=OrderStatus.CONFIRMED)
assert confirmed.status == OrderStatus.CONFIRMED
assert order.status == OrderStatus.PENDING  # Original unchanged

print(f"Order total: {order.total}")
print(f"Status: {order.status.value} → {confirmed.status.value}")
```

## 9.5 — Comparison: Dataclass vs Others

```python
from typing import NamedTuple

# NamedTuple — simpler, immutable by default
class PointNT(NamedTuple):
    x: float
    y: float

# Dataclass — more features, mutable by default
@dataclass(frozen=True)
class PointDC:
    x: float
    y: float

# | Feature | NamedTuple | dataclass(frozen) | dict | Pydantic |
# |---------|------------|-------------------|------|----------|
# | Immutable | Always | Yes | No | Config |
# | Type hints | Yes | Yes | No | Yes |
# | Validation | No | __post_init__ | No | Automatic |
# | Inheritance | No | Yes | N/A | Yes |
# | Serialization | ._asdict() | asdict() | native | .model_dump() |
# | Performance | Fastest | Fast | Fast | Slower |

# Rule of thumb:
# - Simple value types → NamedTuple
# - Domain entities → @dataclass(frozen=True)
# - API I/O → Pydantic BaseModel (Ch14)
```

---

## ✅ Checkpoint 9

> 1. `@dataclass(frozen=True)` = immutable record = Value Object
> 2. `__post_init__` = validate on creation (smart constructor)
> 3. `replace()` = create modified copy (original unchanged)
> 4. Dùng `tuple` thay `list` cho fields — truly immutable
> 5. NamedTuple cho simple, dataclass cho complex

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Tạo frozen `Color(r, g, b)` với validation: 0 ≤ r,g,b ≤ 255.

**Bài 2** (10 phút): Tạo `BankAccount(owner, balance)` với `deposit(amount)` và `withdraw(amount)` trả account MỚI.

<details><summary>✅ Lời giải Bài 2</summary>

```python
@dataclass(frozen=True)
class BankAccount:
    owner: str
    balance: int

    def deposit(self, amount: int) -> "BankAccount":
        return replace(self, balance=self.balance + amount)

    def withdraw(self, amount: int) -> "BankAccount":
        if amount > self.balance:
            raise ValueError("Insufficient funds")
        return replace(self, balance=self.balance - amount)

acc = BankAccount("An", 100_000)
acc2 = acc.deposit(50_000)
assert acc.balance == 100_000  # unchanged
assert acc2.balance == 150_000
```

</details>

---

## Tóm tắt

- ✅ **`@dataclass`**: Auto-generates `__init__`, `__repr__`, `__eq__`.
- ✅ **`frozen=True`**: Immutable, hashable, safe. Nền tảng cho DDD.
- ✅ **`__post_init__`**: Validate on construction. Smart constructor pattern.
- ✅ **`replace()`**: Create modified copy. Immutable update.
- ✅ **Use `tuple` for collection fields** — ensures deep immutability.

## Tiếp theo

→ Chapter 10: **Modules & Packages** — `import`, `__init__.py`, `pyproject.toml`, project structure.
