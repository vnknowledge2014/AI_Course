# Chapter 8 — Data Structures

> **Bạn sẽ học được**:
> - `list`, `tuple`, `dict`, `set` deep dive
> - Comprehensions — Pythonic data transformation
> - `NamedTuple` — typed tuples
> - Iterators & generators — lazy evaluation
> - `collections` module highlights
>
> **Yêu cầu trước**: Chapter 7 (Functions)
> **Thời gian đọc**: ~30 phút | **Level**: Beginner

---

## 8.1 — List Deep Dive

```python
# Slicing
data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
assert data[2:5] == [2, 3, 4]
assert data[:3] == [0, 1, 2]
assert data[-3:] == [7, 8, 9]
assert data[::2] == [0, 2, 4, 6, 8]  # step=2
assert data[::-1] == [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]  # reverse

# Unpacking
first, *middle, last = [1, 2, 3, 4, 5]
assert first == 1
assert middle == [2, 3, 4]
assert last == 5

# FP-style operations (immutable)
original = [3, 1, 4, 1, 5]
sorted_copy = sorted(original)  # returns NEW list
assert original == [3, 1, 4, 1, 5]  # unchanged
assert sorted_copy == [1, 1, 3, 4, 5]
```

## 8.2 — Comprehensions

```python
# List comprehension — most Pythonic data transformation
squares = [x**2 for x in range(10)]
assert squares == [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]

# With filter
even_squares = [x**2 for x in range(10) if x % 2 == 0]
assert even_squares == [0, 4, 16, 36, 64]

# Dict comprehension
word_len = {w: len(w) for w in ["hello", "world", "python"]}
assert word_len == {"hello": 5, "world": 5, "python": 6}

# Set comprehension
unique_lengths = {len(w) for w in ["hi", "hello", "hey"]}
assert unique_lengths == {2, 5, 3}

# Nested comprehension (flatten)
matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
flat = [x for row in matrix for x in row]
assert flat == [1, 2, 3, 4, 5, 6, 7, 8, 9]

# Generator expression — lazy (memory efficient)
gen = (x**2 for x in range(1_000_000))
first_five = [next(gen) for _ in range(5)]
assert first_five == [0, 1, 4, 9, 16]
```

## 8.3 — NamedTuple

```python
from typing import NamedTuple

class Point(NamedTuple):
    x: float
    y: float

class Color(NamedTuple):
    r: int
    g: int
    b: int

# Immutable (like tuple)
p = Point(3.0, 4.0)
# p.x = 5.0  ← AttributeError

# Unpacking
x, y = p
assert x == 3.0

# Access by name or index
assert p.x == p[0] == 3.0

# As dict
assert p._asdict() == {"x": 3.0, "y": 4.0}

# Replace (creates new)
p2 = p._replace(x=5.0)
assert p2 == Point(5.0, 4.0)
assert p == Point(3.0, 4.0)  # original unchanged

red = Color(255, 0, 0)
assert red.r == 255
```

## 8.4 — Iterators & Generators

```python
from typing import Iterator, Generator

# Generator function — lazy evaluation
def fibonacci() -> Generator[int, None, None]:
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

# Take first N
def take(n: int, gen: Iterator[int]) -> list[int]:
    return [next(gen) for _ in range(n)]

fib = fibonacci()
assert take(8, fib) == [0, 1, 1, 2, 3, 5, 8, 13]

# Chain operations (lazy)
from itertools import islice, chain, count

# Infinite counter, take first 5 evens
evens = (x for x in count(0) if x % 2 == 0)
first_evens = list(islice(evens, 5))
assert first_evens == [0, 2, 4, 6, 8]

# Chain iterables
combined = list(chain([1, 2], [3, 4], [5]))
assert combined == [1, 2, 3, 4, 5]
```

## 8.5 — Collections Module

```python
from collections import Counter, defaultdict, deque

# Counter — count occurrences
words = ["python", "is", "great", "python", "is", "fun"]
counts = Counter(words)
assert counts["python"] == 2
assert counts.most_common(1) == [("python", 2)]

# defaultdict — auto-initialize missing keys
groups: defaultdict[str, list[str]] = defaultdict(list)
for name, dept in [("An", "eng"), ("Binh", "eng"), ("Cuong", "sales")]:
    groups[dept].append(name)
assert groups["eng"] == ["An", "Binh"]

# deque — double-ended queue, O(1) append/popleft
q: deque[int] = deque()
q.append(1)     # right
q.appendleft(0) # left
q.append(2)     # right
assert list(q) == [0, 1, 2]
assert q.popleft() == 0  # O(1) vs list.pop(0) O(n)
```

---

## ✅ Checkpoint 8

> 1. Comprehensions > `map`/`filter` in Python (more readable)
> 2. `NamedTuple` = immutable struct with names
> 3. Generators = lazy evaluation — memory efficient for large data
> 4. `Counter`, `defaultdict`, `deque` = specialized collections

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Dùng comprehension tạo dict mapping letters → positions: `"abc" → {"a": 0, "b": 1, "c": 2}`.

**Bài 2** (10 phút): Viết generator `primes()` yield số nguyên tố vô hạn. Lấy 10 số nguyên tố đầu.

<details><summary>✅ Lời giải Bài 2</summary>

```python
def primes():
    yield 2
    n = 3
    while True:
        if all(n % p != 0 for p in range(2, int(n**0.5) + 1)):
            yield n
        n += 2

assert take(10, primes()) == [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
```

</details>

---

## Tóm tắt

- ✅ **Lists**: Slicing, unpacking, `sorted()` (immutable sort).
- ✅ **Comprehensions**: List, dict, set, generator — prefer over loops.
- ✅ **NamedTuple**: Immutable, typed, named access. Use for simple value types.
- ✅ **Generators**: Lazy, memory efficient. `yield` = produce values on demand.
- ✅ **Collections**: `Counter`, `defaultdict`, `deque`.

## Tiếp theo

→ Chapter 9: **Dataclasses & Structured Data** — `@dataclass`, `frozen=True`, `__post_init__`, comparison with NamedTuple.
