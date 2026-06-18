# Chapter 29 — TDD with pytest

> **Bạn sẽ học được**:
> - TDD cycle: Red → Green → Refactor
> - `pytest` basics: `assert`, fixtures, parametrize
> - Testing pure functions — simple and fast
> - Test organization: arrange-act-assert
>
> **Yêu cầu trước**: Chapter 28 (Parser Combinators)
> **Thời gian đọc**: ~30 phút | **Level**: Principal

---

## 29.1 — TDD Cycle

```python
# Step 1: RED — Write failing test
# test_calculator.py
def test_add():
    assert add(2, 3) == 5

# Step 2: GREEN — Minimal code to pass
def add(a: int, b: int) -> int:
    return a + b

# Step 3: REFACTOR — Improve without changing behavior
assert add(2, 3) == 5
assert add(0, 0) == 0
assert add(-1, 1) == 0
```

## 29.2 — pytest Features

```python
import pytest
from dataclasses import dataclass

@dataclass(frozen=True)
class Money:
    amount: int
    currency: str = "VND"
    def add(self, other: "Money") -> "Money":
        assert self.currency == other.currency
        return Money(self.amount + other.amount, self.currency)

# Basic test
def test_money_add():
    a = Money(100)
    b = Money(200)
    assert a.add(b) == Money(300)

# Parametrize — multiple test cases
@pytest.mark.parametrize("a,b,expected", [
    (100, 200, 300),
    (0, 0, 0),
    (1000, -500, 500),
])
def test_money_add_parametrize(a: int, b: int, expected: int):
    assert Money(a).add(Money(b)) == Money(expected)

# Test exceptions
def test_money_negative():
    with pytest.raises(ValueError):
        Money(-1)

# Fixture
@pytest.fixture
def sample_order():
    return {"customer": "An", "items": [("Coffee", 35_000)], "total": 35_000}

def test_order_total(sample_order):
    assert sample_order["total"] == 35_000

# Arrange-Act-Assert pattern
def test_confirm_order():
    # Arrange
    order = {"status": "pending", "items": ["Coffee"]}

    # Act
    confirmed = {**order, "status": "confirmed"}

    # Assert
    assert confirmed["status"] == "confirmed"
    assert confirmed["items"] == order["items"]
```

## 29.3 — Testing Pure Functions

```python
# Pure functions are TRIVIAL to test — no setup, no mocks!

def discount(price: float, percent: float) -> float:
    return price * (1 - percent / 100)

def test_discount_10_percent():
    assert discount(100_000, 10) == 90_000

def test_discount_zero():
    assert discount(100_000, 0) == 100_000

def test_discount_100_percent():
    assert discount(100_000, 100) == 0

# Compare with impure function — needs mock, setup, teardown...
# FP makes testing EASY by default.
```

---

## Tóm tắt

- ✅ **TDD**: Red → Green → Refactor.
- ✅ **pytest**: `assert`, `@pytest.mark.parametrize`, `@pytest.fixture`.
- ✅ **Pure functions**: Trivial to test — no mocks needed.
- ✅ **Arrange-Act-Assert**: Standard test structure.

## Tiếp theo

→ Chapter 30: **Property-Based Testing** — Hypothesis, random inputs.
