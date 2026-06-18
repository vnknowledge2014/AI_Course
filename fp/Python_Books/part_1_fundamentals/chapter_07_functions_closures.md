# Chapter 7 — Functions & Closures

> **Bạn sẽ học được**:
> - `def` functions, `lambda`, default/keyword args
> - Higher-Order Functions: `map`, `filter`, `reduce`
> - Closures — functions "nhớ" environment
> - `functools.partial`, `functools.reduce`
> - First-class functions — functions là values
>
> **Yêu cầu trước**: Chapter 6 (Control Flow)
> **Thời gian đọc**: ~35 phút | **Level**: Beginner

---

## 7.1 — Functions Basics

```python
# def — named function
def add(a: int, b: int) -> int:
    return a + b

assert add(3, 4) == 7

# Default arguments
def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"

assert greet("An") == "Hello, An!"
assert greet("An", "Hi") == "Hi, An!"

# Keyword arguments
assert greet(greeting="Hey", name="Binh") == "Hey, Binh!"

# *args, **kwargs
def log(*messages: str, level: str = "INFO") -> str:
    return f"[{level}] {' '.join(messages)}"

assert log("Hello", "World") == "[INFO] Hello World"
assert log("Error!", level="ERROR") == "[ERROR] Error!"
```

## 7.2 — Lambda & Higher-Order Functions

```python
# Lambda — anonymous function
double = lambda x: x * 2
assert double(5) == 10

# === map — transform each element ===
numbers = [1, 2, 3, 4, 5]
doubled = list(map(lambda x: x * 2, numbers))
assert doubled == [2, 4, 6, 8, 10]

# Pythonic: list comprehension
doubled_v2 = [x * 2 for x in numbers]
assert doubled_v2 == doubled

# === filter — keep elements matching condition ===
evens = list(filter(lambda x: x % 2 == 0, numbers))
assert evens == [2, 4]

evens_v2 = [x for x in numbers if x % 2 == 0]
assert evens_v2 == evens

# === reduce — fold into single value ===
from functools import reduce

total = reduce(lambda acc, x: acc + x, numbers, 0)
assert total == 15

product = reduce(lambda acc, x: acc * x, numbers, 1)
assert product == 120

print(f"sum={total}, product={product}")
```

## 7.3 — Functions as Values (First-class)

```python
from typing import Callable

# Functions can be assigned to variables
operation: Callable[[int, int], int] = add
assert operation(3, 4) == 7

# Functions can be in collections
ops: dict[str, Callable[[int, int], int]] = {
    "+": lambda a, b: a + b,
    "-": lambda a, b: a - b,
    "*": lambda a, b: a * b,
}

assert ops["+"](10, 3) == 13
assert ops["-"](10, 3) == 7
assert ops["*"](10, 3) == 30

# Functions can be passed as arguments
def apply_to_list(lst: list[int], fn: Callable[[int], int]) -> list[int]:
    return [fn(x) for x in lst]

assert apply_to_list([1, 2, 3], lambda x: x ** 2) == [1, 4, 9]
assert apply_to_list([1, 2, 3], lambda x: x + 10) == [11, 12, 13]

# Functions can return functions
def make_multiplier(factor: int) -> Callable[[int], int]:
    def multiplier(x: int) -> int:
        return x * factor
    return multiplier

triple = make_multiplier(3)
assert triple(5) == 15
assert triple(10) == 30
```

## 7.4 — Closures

```python
# Closure: inner function captures outer variable
def make_counter(start: int = 0) -> Callable[[], int]:
    count = start
    def counter() -> int:
        nonlocal count
        count += 1
        return count
    return counter

c = make_counter()
assert c() == 1
assert c() == 2
assert c() == 3

# Immutable closure (FP-friendly)
def make_adder(n: int) -> Callable[[int], int]:
    # n is captured but never modified — pure!
    return lambda x: x + n

add_five = make_adder(5)
add_ten = make_adder(10)
assert add_five(3) == 8
assert add_ten(3) == 13
```

## 7.5 — `functools.partial`

```python
from functools import partial

def power(base: int, exponent: int) -> int:
    return base ** exponent

square = partial(power, exponent=2)
cube = partial(power, exponent=3)

assert square(5) == 25
assert cube(3) == 27

# Practical: pre-configure functions
def format_price(amount: int, currency: str = "VND") -> str:
    return f"{amount:,} {currency}"

format_vnd = partial(format_price, currency="VND")
format_usd = partial(format_price, currency="USD")

assert format_vnd(50_000) == "50,000 VND"
assert format_usd(100) == "100 USD"
```

---

## ✅ Checkpoint 7

> 1. Functions = first-class values (assign, pass, return)
> 2. `map`/`filter`/`reduce` = HOFs, but prefer comprehensions in Python
> 3. Closures capture outer variables — `make_adder(5)` remembers `n=5`
> 4. `functools.partial` = create specialized functions

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Viết `compose(f, g)` trả function `x -> f(g(x))`.

<details><summary>✅ Lời giải</summary>

```python
def compose(f, g):
    return lambda x: f(g(x))

add1 = lambda x: x + 1
double = lambda x: x * 2

add1_then_double = compose(double, add1)
assert add1_then_double(3) == 8  # double(add1(3)) = double(4) = 8
```

</details>

**Bài 2** (10 phút): Implement `pipe(value, *fns)` — áp dụng functions từ trái sang phải.

<details><summary>✅ Lời giải</summary>

```python
from functools import reduce

def pipe(value, *fns):
    return reduce(lambda acc, fn: fn(acc), fns, value)

result = pipe(3, lambda x: x + 1, lambda x: x * 2, str)
assert result == "8"  # (3+1)*2 = 8 → "8"
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `lambda` chỉ 1 expression | Python limitation | Dùng `def` cho logic phức tạp |
| `UnboundLocalError` trong closure | Quên `nonlocal` khi reassign | Thêm `nonlocal variable_name` |
| `map` trả iterator, không list | Python 3 behavior | Wrap với `list()` |

---

## Tóm tắt

- ✅ **First-class functions**: assign, pass as arg, return from function.
- ✅ **HOFs**: `map`, `filter`, `reduce` — nhưng prefer comprehensions.
- ✅ **Closures**: Inner function captures outer scope. Immutable closures = pure.
- ✅ **`partial`**: Pre-fill arguments to create specialized functions.

## Tiếp theo

→ Chapter 8: **Data Structures** — `list`, `dict`, `set`, comprehensions, `NamedTuple`, iterators.
