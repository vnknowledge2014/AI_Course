# Chapter 30 — Property-Based Testing

> **Bạn sẽ học được**:
> - PBT: test PROPERTIES, not examples
> - `hypothesis` library
> - Strategies for generating test data
> - Shrinking — tìm minimal failing case
>
> **Yêu cầu trước**: Chapter 29 (TDD)
> **Thời gian đọc**: ~25 phút | **Level**: Principal

---

## 30.1 — Example vs Property

```python
# Example-based: test specific values
def test_reverse_example():
    assert list(reversed([1, 2, 3])) == [3, 2, 1]

# Property-based: test INVARIANT for ALL inputs
# "reverse(reverse(lst)) == lst" — for ANY list
from hypothesis import given
from hypothesis import strategies as st

@given(st.lists(st.integers()))
def test_reverse_involution(lst):
    assert list(reversed(list(reversed(lst)))) == lst

@given(st.lists(st.integers()))
def test_reverse_preserves_length(lst):
    assert len(list(reversed(lst))) == len(lst)

@given(st.lists(st.integers()))
def test_sort_is_idempotent(lst):
    assert sorted(sorted(lst)) == sorted(lst)

@given(st.integers(), st.integers())
def test_addition_commutative(a, b):
    assert a + b == b + a
```

## 30.2 — Strategies

```python
from hypothesis import strategies as st

# Primitives
# st.integers()            → any int
# st.integers(0, 100)      → int in [0, 100]
# st.floats()              → any float
# st.text()                → any string
# st.booleans()            → True/False

# Collections
# st.lists(st.integers())          → [int, ...]
# st.lists(st.integers(), min_size=1) → non-empty list
# st.tuples(st.integers(), st.text()) → (int, str)
# st.dictionaries(st.text(), st.integers()) → {str: int}

# Custom: domain types
from dataclasses import dataclass

@dataclass(frozen=True)
class Money:
    amount: int
    currency: str

money_strategy = st.builds(
    Money,
    amount=st.integers(min_value=0, max_value=1_000_000),
    currency=st.sampled_from(["VND", "USD", "EUR"]),
)

@given(money_strategy, money_strategy)
def test_money_add_commutative(a: Money, b: Money):
    if a.currency == b.currency:
        sum1 = Money(a.amount + b.amount, a.currency)
        sum2 = Money(b.amount + a.amount, b.currency)
        assert sum1 == sum2
```

## 30.3 — Testing Monoid Laws

```python
@given(st.integers(), st.integers(), st.integers())
def test_addition_associativity(a, b, c):
    assert (a + b) + c == a + (b + c)

@given(st.integers())
def test_addition_identity(a):
    assert a + 0 == a
    assert 0 + a == a

# These tests run 100 random examples by default
# Hypothesis finds edge cases you'd never think of!
```

---

## Tóm tắt

- ✅ **PBT**: Test properties/invariants, not specific examples.
- ✅ **Hypothesis**: `@given` + `st.` strategies = auto-generate test data.
- ✅ **Shrinking**: Finds minimal failing case automatically.
- ✅ **Monoid laws**: PBT is perfect for testing algebraic properties.

## Tiếp theo

→ Chapter 31: **FastAPI & Async** — Building web APIs with FastAPI.
