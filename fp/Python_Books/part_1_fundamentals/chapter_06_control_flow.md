# Chapter 6 — Control Flow & Pattern Matching

> **Bạn sẽ học được**:
> - `if/elif/else` — expressions và statements
> - `for`, `while`, `break`, `continue`
> - `match/case` (Python 3.10+) — structural pattern matching
> - Guards, destructuring, exhaustive matching
>
> **Yêu cầu trước**: Chapter 5 (Values & Types)
> **Thời gian đọc**: ~30 phút | **Level**: Beginner

---

## 6.1 — Conditional Expressions

```python
# if/elif/else — statement
status = "active"
if status == "active":
    message = "Welcome!"
elif status == "banned":
    message = "Access denied"
else:
    message = "Unknown status"

assert message == "Welcome!"

# Conditional expression (ternary) — expression
x = 42
label = "even" if x % 2 == 0 else "odd"
assert label == "even"

# Chaining
def classify(score: int) -> str:
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    else:
        return "F"

assert classify(95) == "A"
assert classify(85) == "B"
assert classify(60) == "F"
```

## 6.2 — Loops

```python
# for loop — iterate over sequences
total = 0
for x in [1, 2, 3, 4, 5]:
    total += x
assert total == 15

# Comprehensions — FP-style loops
squares = [x**2 for x in range(5)]
assert squares == [0, 1, 4, 9, 16]

evens = [x for x in range(10) if x % 2 == 0]
assert evens == [0, 2, 4, 6, 8]

# Dict comprehension
word_lengths = {w: len(w) for w in ["hello", "world", "python"]}
assert word_lengths["python"] == 6

# enumerate, zip
names = ["An", "Binh"]
scores = [95, 87]
paired = list(zip(names, scores))
assert paired == [("An", 95), ("Binh", 87)]

for i, name in enumerate(names):
    print(f"{i}: {name}")
```

## 6.3 — Pattern Matching (Python 3.10+)

```python
from dataclasses import dataclass
from typing import Union

# === Basic match ===
def http_status(code: int) -> str:
    match code:
        case 200:
            return "OK"
        case 404:
            return "Not Found"
        case 500:
            return "Internal Server Error"
        case _:
            return f"Unknown: {code}"

assert http_status(200) == "OK"
assert http_status(404) == "Not Found"
assert http_status(999) == "Unknown: 999"

# === Destructuring match ===
@dataclass(frozen=True)
class Point:
    x: float
    y: float

def describe_point(p: Point) -> str:
    match p:
        case Point(x=0, y=0):
            return "Origin"
        case Point(x=0, y=y):
            return f"On Y-axis at y={y}"
        case Point(x=x, y=0):
            return f"On X-axis at x={x}"
        case Point(x=x, y=y):
            return f"Point({x}, {y})"

assert describe_point(Point(0, 0)) == "Origin"
assert describe_point(Point(0, 5)) == "On Y-axis at y=5"
assert describe_point(Point(3, 4)) == "Point(3, 4)"

# === Union type matching ===
@dataclass(frozen=True)
class Circle:
    radius: float

@dataclass(frozen=True)
class Rectangle:
    width: float
    height: float

Shape = Union[Circle, Rectangle]

def area(shape: Shape) -> float:
    match shape:
        case Circle(radius=r):
            return 3.14159 * r * r
        case Rectangle(width=w, height=h):
            return w * h

assert abs(area(Circle(5)) - 78.54) < 0.01
assert area(Rectangle(3, 4)) == 12.0

# === Guards ===
def classify_age(age: int) -> str:
    match age:
        case n if n < 0:
            return "Invalid"
        case n if n < 13:
            return "Child"
        case n if n < 18:
            return "Teen"
        case n if n < 65:
            return "Adult"
        case _:
            return "Senior"

assert classify_age(5) == "Child"
assert classify_age(15) == "Teen"
assert classify_age(30) == "Adult"
assert classify_age(70) == "Senior"
```

### Exhaustive matching

```python
from typing import Never

@dataclass(frozen=True)
class Cash:
    amount: float

@dataclass(frozen=True)
class Card:
    number: str

Payment = Union[Cash, Card]

def process(payment: Payment) -> str:
    match payment:
        case Cash(amount=a):
            return f"Cash: {a}đ"
        case Card(number=n):
            return f"Card: ***{n[-4:]}"
        # Nếu thêm BankTransfer vào Union mà không thêm case →
        # mypy báo lỗi (với plugin hoặc assert_never)

assert process(Cash(50_000)) == "Cash: 50000đ"
```

---

## ✅ Checkpoint 6

> 1. `match/case` = structural pattern matching (Python 3.10+)
> 2. Destructuring: `case Point(x=0, y=y)` — trích xuất fields
> 3. Guards: `case n if n < 0` — điều kiện thêm
> 4. Comprehensions = FP-style loops (ưu tiên hơn for/append)

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Viết `match` cho HTTP methods: "GET" → "Read", "POST" → "Create", "PUT" → "Update", "DELETE" → "Remove".

**Bài 2** (10 phút): Tạo union type `Expr = Literal | Add | Multiply` và viết `evaluate(expr)` dùng match.

<details><summary>✅ Lời giải Bài 2</summary>

```python
@dataclass(frozen=True)
class Lit:
    value: int

@dataclass(frozen=True)
class Add:
    left: "Expr"
    right: "Expr"

@dataclass(frozen=True)
class Mul:
    left: "Expr"
    right: "Expr"

Expr = Union[Lit, Add, Mul]

def evaluate(expr: Expr) -> int:
    match expr:
        case Lit(v):
            return v
        case Add(l, r):
            return evaluate(l) + evaluate(r)
        case Mul(l, r):
            return evaluate(l) * evaluate(r)

# (2 + 3) * 4 = 20
expr = Mul(Add(Lit(2), Lit(3)), Lit(4))
assert evaluate(expr) == 20
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `SyntaxError: invalid syntax` trên `match` | Python < 3.10 | Upgrade hoặc dùng `if/elif` |
| `match` không exhaustive | Python không enforce | Thêm `case _: raise ValueError` |

---

## Tóm tắt

- ✅ **Conditionals**: `if/elif/else` + ternary expression.
- ✅ **Loops**: `for`, comprehensions (ưu tiên), `enumerate`, `zip`.
- ✅ **Pattern matching**: `match/case` (3.10+), destructuring, guards.
- ✅ **Exhaustive matching**: Dùng `case _` hoặc `assert_never`.

## Tiếp theo

→ Chapter 7: **Functions & Closures** — `def`, `lambda`, `functools`, higher-order functions, closures.
