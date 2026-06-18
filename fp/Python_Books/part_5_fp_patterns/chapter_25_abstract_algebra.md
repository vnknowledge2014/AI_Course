# Chapter 25 — Abstract Algebra for Programmers

> **Bạn sẽ học được**:
> - Semigroup: binary associative operation
> - Monoid: semigroup + identity element
> - Why Monoids matter: `reduce`, parallel processing
> - Practical monoids: `Sum`, `Product`, `All`, `Any`, `list`
>
> **Yêu cầu trước**: Chapter 24 (Persistence)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 25.1 — Semigroup

```python
from typing import Protocol, TypeVar

T = TypeVar("T")

class Semigroup(Protocol):
    def combine(self, other: "Semigroup") -> "Semigroup": ...

# Rule: combine must be ASSOCIATIVE
# (a.combine(b)).combine(c) == a.combine(b.combine(c))

from dataclasses import dataclass

@dataclass(frozen=True)
class Sum:
    value: int
    def combine(self, other: "Sum") -> "Sum":
        return Sum(self.value + other.value)

@dataclass(frozen=True)
class Product:
    value: int
    def combine(self, other: "Product") -> "Product":
        return Product(self.value * other.value)

assert Sum(3).combine(Sum(4)) == Sum(7)
assert Product(3).combine(Product(4)) == Product(12)

# Associative:
a, b, c = Sum(1), Sum(2), Sum(3)
assert a.combine(b).combine(c) == a.combine(b.combine(c))  # (1+2)+3 == 1+(2+3)
```

## 25.2 — Monoid = Semigroup + Identity

```python
from functools import reduce

@dataclass(frozen=True)
class Sum:
    value: int
    def combine(self, other: "Sum") -> "Sum":
        return Sum(self.value + other.value)
    @staticmethod
    def empty() -> "Sum":
        return Sum(0)  # Identity: x + 0 == x

@dataclass(frozen=True)
class All:
    value: bool
    def combine(self, other: "All") -> "All":
        return All(self.value and other.value)
    @staticmethod
    def empty() -> "All":
        return All(True)  # Identity: x and True == x

# reduce with monoid
numbers = [Sum(1), Sum(2), Sum(3), Sum(4)]
total = reduce(lambda a, b: a.combine(b), numbers, Sum.empty())
assert total == Sum(10)

flags = [All(True), All(True), All(False)]
result = reduce(lambda a, b: a.combine(b), flags, All.empty())
assert result == All(False)
```

## 25.3 — Practical Monoids

```python
# Python built-in monoids (you use them every day!)

# list: combine = +, empty = []
assert [1, 2] + [3, 4] == [1, 2, 3, 4]

# str: combine = +, empty = ""
assert "hello" + " " + "world" == "hello world"

# dict: combine = |, empty = {}
assert {"a": 1} | {"b": 2} == {"a": 1, "b": 2}

# Money as Monoid
@dataclass(frozen=True)
class Money:
    amount: int
    currency: str = "VND"
    def combine(self, other: "Money") -> "Money":
        assert self.currency == other.currency
        return Money(self.amount + other.amount, self.currency)
    @staticmethod
    def empty(currency: str = "VND") -> "Money":
        return Money(0, currency)

prices = [Money(35_000), Money(50_000), Money(25_000)]
total = reduce(lambda a, b: a.combine(b), prices, Money.empty())
assert total == Money(110_000)
```

---

## Tóm tắt

- ✅ **Semigroup**: `combine` — associative binary operation.
- ✅ **Monoid**: Semigroup + `empty` — identity element.
- ✅ **`reduce`**: Fold any monoid collection into single value.
- ✅ **Everywhere**: `list`, `str`, `dict`, `Sum`, `Money` — all monoids.

## Tiếp theo

→ Chapter 26: **Functors** — Mapping over containers.
