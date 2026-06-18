# Chapter 1 — Math Foundations for FP

> **Bạn sẽ học được**:
> - Lambda Calculus — nền tảng toán học của mọi ngôn ngữ lập trình
> - Curry-Howard Correspondence — tại sao types = logic
> - Discrete Math — Set theory, relations, tại sao chúng quan trọng cho FP
> - Algebraic type sizes — đếm trạng thái domain bằng toán
>
> **Yêu cầu trước**: Biết đọc code Python cơ bản (biến, function, `if/else`)
> **Thời gian đọc**: ~40 phút | **Level**: Pre-requisite
> **Kết quả cuối cùng**: Hiểu TẠI SAO functional programming hoạt động — không chỉ HOW.

---

## Tại sao cần toán cho lập trình?

Bạn có thể nghĩ: "Tôi muốn code, không muốn học toán." Hoàn toàn hợp lý. Nhưng hãy tưởng tượng thế này: bạn muốn xây nhà mà không biết vật lý cơ bản. Có thể xây được — nhưng khi nào nhà sập, bạn không hiểu tại sao.

FP không phải là toán — nhưng FP **ĐƯỢC PHÁT MINH TỪ TOÁN**. Lambda Calculus ra đời năm 1936, trước khi máy tính đầu tiên xuất hiện. Hiểu nền tảng giúp bạn hiểu tại sao `map`, `filter`, `reduce` hoạt động — không chỉ biết dùng.

Chapter này **KHÔNG** dạy toán nặng. Chỉ đủ để bạn có intuition đúng. Mỗi phần đều có code Python minh họa.

---

## 1.1 — Lambda Calculus: Nơi mọi thứ bắt đầu

### Lambda là gì?

Năm 1936, nhà toán học Alonzo Church phát minh ra một hệ thống gọi là **Lambda Calculus** — cách biểu diễn MỌI phép tính chỉ bằng **functions**. Không có biến lưu trữ, không có loop, không có `if/else` — chỉ có functions.

Nghe điên rồ? Đúng. Nhưng hệ thống này **Turing-complete** — nghĩa là nó có thể tính toán BẤT KỲ thứ gì mà máy tính hiện đại có thể tính.

```python
# Lambda Calculus trong Python: lambda là anonymous function

# Toán: λx.x+1  (nhận x, trả về x+1)
# Python:
add_one = lambda x: x + 1

assert add_one(5) == 6
assert add_one(0) == 1
print(f"add_one(5) = {add_one(5)}")
# Output: add_one(5) = 6
```

### β-reduction = Gọi function

Trong Lambda Calculus, "tính toán" = **β-reduction** = thay thế tham số bằng giá trị:

```
(λx.x+1) 5  →  5+1  →  6
```

Trong Python, đây chính là gọi function:

```python
# β-reduction = function application
result = (lambda x: x + 1)(5)  # thay x = 5 → 5+1 → 6
assert result == 6
print(f"(λx.x+1)(5) = {result}")
# Output: (λx.x+1)(5) = 6
```

### Higher-Order Functions: Functions nhận functions

Lambda Calculus cho phép functions nhận functions làm input — gọi là **Higher-Order Functions** (HOF):

```python
# HOF: function nhận function làm parameter
def apply_twice(f, x):
    """Áp dụng function f hai lần: f(f(x))"""
    return f(f(x))

double = lambda x: x * 2
add_ten = lambda x: x + 10

assert apply_twice(double, 3) == 12    # double(double(3)) = double(6) = 12
assert apply_twice(add_ten, 0) == 20   # add_ten(add_ten(0)) = add_ten(10) = 20

print(f"apply_twice(double, 3) = {apply_twice(double, 3)}")
print(f"apply_twice(add_ten, 0) = {apply_twice(add_ten, 0)}")
# Output:
# apply_twice(double, 3) = 12
# apply_twice(add_ten, 0) = 20
```

### Currying: Tách function nhiều tham số

**Currying** = biến function nhận N tham số thành chuỗi N functions, mỗi cái nhận 1 tham số:

```python
from functools import partial

# Uncurried: nhận 2 tham số cùng lúc
def add(x: int, y: int) -> int:
    return x + y

# Curried: nhận từng tham số một
def add_curried(x: int):
    def inner(y: int) -> int:
        return x + y
    return inner

# Sử dụng
assert add(3, 4) == 7
assert add_curried(3)(4) == 7

# Tạo functions chuyên biệt từ curried version
add_five = add_curried(5)
assert add_five(10) == 15
assert add_five(20) == 25

# Python cũng có partial() cho uncurried functions
add_five_v2 = partial(add, 5)
assert add_five_v2(10) == 15

print(f"add_curried(3)(4) = {add_curried(3)(4)}")
print(f"add_five(10) = {add_five(10)}")
# Output:
# add_curried(3)(4) = 7
# add_five(10) = 15
```

> **💡 Tại sao currying quan trọng?** Vì nó cho phép **partial application** — tạo functions chuyên biệt từ functions tổng quát. `add_five = add_curried(5)` — bạn vừa tạo một tool mới chỉ bằng cách "cố định" một tham số.

---

## ✅ Checkpoint 1.1

> Ghi nhớ:
> 1. Lambda Calculus = nền tảng của mọi FP — mọi computation = function application
> 2. β-reduction = gọi function — thay tham số bằng giá trị
> 3. HOF = function nhận/trả function — `map`, `filter` đều là HOF
> 4. Currying = tách function nhiều tham số → chuỗi functions 1 tham số
>
> **Test nhanh**: `(lambda f: lambda x: f(f(x)))(lambda y: y * 2)(3)` = ?
> <details><summary>Đáp án</summary>12. Vì: inner function áp dụng f hai lần: f(f(3)) = (3*2)*2 = 12.</details>

---

## 1.2 — Curry-Howard Correspondence: Types = Logic

### Ý tưởng chính

**Curry-Howard Correspondence** nói rằng:

| Lập trình | Logic |
|-----------|-------|
| **Type** | Mệnh đề (Proposition) |
| **Program** (giá trị thuộc type đó) | Bằng chứng (Proof) |
| **Function `A → B`** | "Nếu A đúng thì B đúng" (Implication) |
| **Product `(A, B)`** | "A VÀ B đều đúng" (Conjunction) |
| **Sum `A \| B`** | "A HOẶC B đúng" (Disjunction) |
| **Empty type (Never)** | Mệnh đề sai (False) |
| **Unit ()** | Mệnh đề đúng tầm thường (True) |

Nghe trừu tượng? Hãy xem ví dụ cụ thể:

```python
from dataclasses import dataclass
from typing import Union

# Product type: A VÀ B (AND)
# "Một Person CÓ name VÀ CÓ age" — cả hai đều phải tồn tại
@dataclass(frozen=True)
class Person:
    name: str
    age: int

# Sum type: A HOẶC B (OR)
# "Một Shape LÀ Circle HOẶC Rectangle" — chỉ một trong hai
@dataclass(frozen=True)
class Circle:
    radius: float

@dataclass(frozen=True)
class Rectangle:
    width: float
    height: float

Shape = Union[Circle, Rectangle]  # Sum type = OR

# Function A → B: "Nếu có Shape, tôi CÓ THỂ tính area"
def area(shape: Shape) -> float:
    match shape:
        case Circle(r):
            return 3.14159 * r * r
        case Rectangle(w, h):
            return w * h

# Test
p = Person(name="An", age=25)
assert p.name == "An"
assert p.age == 25

c = Circle(radius=5.0)
r = Rectangle(width=3.0, height=4.0)
assert abs(area(c) - 78.54) < 0.01
assert area(r) == 12.0

print(f"area(Circle(5)) = {area(c):.2f}")
print(f"area(Rect(3,4)) = {area(r)}")
# Output:
# area(Circle(5)) = 78.54
# area(Rect(3,4)) = 12.0
```

### "Make Illegal States Unrepresentable"

Đây là hệ quả quan trọng nhất của Curry-Howard: nếu type system CẤM một trạng thái — thì trạng thái đó KHÔNG THỂ TỒN TẠI trong chương trình. Compiler trở thành bảo vệ.

```python
from dataclasses import dataclass
from typing import Union

# ❌ BAD: dùng string cho payment method — bất kỳ giá trị nào đều hợp lệ
class BadOrder:
    payment: str  # "cash"? "card"? "bitcoin"? "typo"? bất cứ gì!

# ✅ GOOD: dùng Union type — chỉ có 3 giá trị hợp lệ
@dataclass(frozen=True)
class Cash:
    amount: float

@dataclass(frozen=True)
class Card:
    number: str
    cvv: str

@dataclass(frozen=True)
class BankTransfer:
    account: str

Payment = Union[Cash, Card, BankTransfer]

def process_payment(payment: Payment) -> str:
    match payment:
        case Cash(amount):
            return f"Nhận tiền mặt: {amount}đ"
        case Card(number, _):
            return f"Charge thẻ: ***{number[-4:]}"
        case BankTransfer(account):
            return f"Chuyển khoản: {account}"

# Không thể tạo payment không hợp lệ (mypy sẽ báo lỗi nếu dùng string)
p1 = Cash(amount=100_000)
p2 = Card(number="4111111111111111", cvv="123")

assert process_payment(p1) == "Nhận tiền mặt: 100000đ"
assert "1111" in process_payment(p2)

print(process_payment(p1))
print(process_payment(p2))
# Output:
# Nhận tiền mặt: 100000đ
# Charge thẻ: ***1111
```

---

## 1.3 — Discrete Math: Set Theory cho Programmers

### Product Type = Cartesian Product

**Product type** (struct/dataclass) = tích Cartesian. Số trạng thái = tích các thành phần:

```python
from dataclasses import dataclass

# bool × bool = 2 × 2 = 4 trạng thái
@dataclass(frozen=True)
class Switches:
    light: bool      # True/False = 2 giá trị
    fan: bool        # True/False = 2 giá trị

# Liệt kê TẤT CẢ trạng thái có thể
all_states = [
    Switches(light=False, fan=False),
    Switches(light=False, fan=True),
    Switches(light=True, fan=False),
    Switches(light=True, fan=True),
]
assert len(all_states) == 4  # 2 × 2 = 4
print(f"Tổng trạng thái Switches: {len(all_states)}")
# Output: Tổng trạng thái Switches: 4

# Size = Shirt × Color = ?
@dataclass(frozen=True)
class TShirt:
    size: str    # "S", "M", "L" = 3 giá trị
    color: str   # "Red", "Blue" = 2 giá trị

# Tổng: 3 × 2 = 6 trạng thái
sizes = ["S", "M", "L"]
colors = ["Red", "Blue"]
all_tshirts = [TShirt(s, c) for s in sizes for c in colors]
assert len(all_tshirts) == 6
print(f"Tổng loại TShirt: {len(all_tshirts)}")
# Output: Tổng loại TShirt: 6
```

### Sum Type = Disjoint Union

**Sum type** (union/enum) = hợp rời. Số trạng thái = tổng các thành phần:

```python
from dataclasses import dataclass
from typing import Union

# Light có 3 trạng thái: Red | Yellow | Green
@dataclass(frozen=True)
class Red:
    pass

@dataclass(frozen=True)
class Yellow:
    pass

@dataclass(frozen=True)
class Green:
    pass

TrafficLight = Union[Red, Yellow, Green]  # 1 + 1 + 1 = 3 trạng thái

# So sánh: nếu dùng string → vô hạn trạng thái!
# TrafficLight = str  ← BAD: "red", "RED", "redd", "purple", "abc123"...

def next_light(light: TrafficLight) -> TrafficLight:
    match light:
        case Red():
            return Green()
        case Green():
            return Yellow()
        case Yellow():
            return Red()

current = Red()
assert isinstance(next_light(current), Green)
assert isinstance(next_light(Green()), Yellow)

print(f"Red -> {type(next_light(Red())).__name__}")
print(f"Green -> {type(next_light(Green())).__name__}")
print(f"Yellow -> {type(next_light(Yellow())).__name__}")
# Output:
# Red -> Green
# Green -> Yellow
# Yellow -> Red
```

### Tính domain space

```python
# === Bài toán thực tế: Order system ===

# OrderStatus = Pending | Confirmed | Shipped | Delivered | Cancelled
# = 5 trạng thái

# PaymentMethod = Cash | Card | BankTransfer
# = 3 trạng thái

# ShippingSpeed = Standard | Express
# = 2 trạng thái

# Tổng tổ hợp (Product): 5 × 3 × 2 = 30 trạng thái

# Nếu dùng string cho tất cả → VÔ HẠN trạng thái → vô hạn bugs tiềm ẩn
# Nếu dùng types → 30 trạng thái → kiểm soát được hết

total_states = 5 * 3 * 2
print(f"Domain space: {total_states} trạng thái")
assert total_states == 30
# Output: Domain space: 30 trạng thái
```

> **💡 Insight quan trọng**: Khi bạn thiết kế types, bạn đang GIỚI HẠN domain space. Ít trạng thái hơn = ít bugs hơn = dễ test hơn. Đây là lý do FP nhấn mạnh "make illegal states unrepresentable."

---

## ✅ Checkpoint 1.3

> Ghi nhớ:
> 1. Product type (dataclass) = AND = tích các thành phần
> 2. Sum type (Union) = OR = tổng các thành phần
> 3. Ít trạng thái = ít bugs = dễ kiểm soát
>
> **Test nhanh**: Một dataclass có 1 field `bool` và 1 field `Union[A, B, C]` có bao nhiêu trạng thái?
> <details><summary>Đáp án</summary>2 × 3 = 6 trạng thái. Product type = tích.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Lambda basics

Viết lambda expression cho:
1. Nhân đôi một số: `double(4) == 8`
2. Kiểm tra số chẵn: `is_even(3) == False`
3. Lấy ký tự đầu: `first_char("hello") == "h"`

<details><summary>✅ Lời giải Bài 1</summary>

```python
double = lambda x: x * 2
is_even = lambda x: x % 2 == 0
first_char = lambda s: s[0]

assert double(4) == 8
assert is_even(3) == False
assert first_char("hello") == "h"
```

</details>

---

**Bài 2** (10 phút): Currying

Viết curried version của `multiply(x, y)` và tạo `triple = multiply_curried(3)`:

<details><summary>✅ Lời giải Bài 2</summary>

```python
def multiply_curried(x: int):
    def inner(y: int) -> int:
        return x * y
    return inner

triple = multiply_curried(3)
assert triple(5) == 15
assert triple(10) == 30
assert multiply_curried(2)(7) == 14
```

</details>

---

**Bài 3** (15 phút): Domain modeling

Thiết kế type system cho hệ thống đèn giao thông thông minh: mỗi đèn có `color` (Red/Yellow/Green), `duration_seconds` (int), và `is_blinking` (bool). Tính tổng domain space và liệt kê tất cả trạng thái khi `duration_seconds` chỉ có 3 giá trị (10, 30, 60).

<details><summary>✅ Lời giải Bài 3</summary>

```python
from dataclasses import dataclass
from typing import Union

@dataclass(frozen=True)
class RedLight: pass

@dataclass(frozen=True)
class YellowLight: pass

@dataclass(frozen=True)
class GreenLight: pass

Color = Union[RedLight, YellowLight, GreenLight]  # 3

@dataclass(frozen=True)
class SmartLight:
    color: Color           # 3 giá trị
    duration: int          # 3 giá trị (10, 30, 60)
    is_blinking: bool      # 2 giá trị

# Domain space = 3 × 3 × 2 = 18
durations = [10, 30, 60]
colors = [RedLight(), YellowLight(), GreenLight()]
blinks = [True, False]

all_states = [
    SmartLight(c, d, b)
    for c in colors for d in durations for b in blinks
]
assert len(all_states) == 18
print(f"Total states: {len(all_states)}")
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `match` not working | Python < 3.10 | Upgrade lên 3.10+ hoặc dùng `if isinstance()` |
| `lambda` chỉ 1 expression | Python limitation | Dùng `def` cho logic phức tạp |
| `mypy` không bắt Union errors | Thiếu `--strict` flag | Chạy `mypy --strict filename.py` |
| `frozen=True` nhưng vẫn sửa được | Đang sửa nested mutable object | Dùng `tuple` thay `list` bên trong |

---

## Tóm tắt

- ✅ **Lambda Calculus**: Mọi computation = function application. Python `lambda` = anonymous function.
- ✅ **Currying**: Tách function nhiều tham số → chuỗi functions 1 tham số. `functools.partial` hỗ trợ.
- ✅ **Curry-Howard**: Types = Propositions, Programs = Proofs. "Make illegal states unrepresentable."
- ✅ **Product types** (dataclass): AND — domain size = tích.
- ✅ **Sum types** (Union): OR — domain size = tổng.
- ✅ **Domain space**: Ít trạng thái hơn = ít bugs hơn. Dùng types để giới hạn.

## Tiếp theo

→ Chapter 2: **Algorithmic Thinking & Complexity** — Big-O, recursion, common patterns. Bạn sẽ hiểu tại sao `list.append()` nhanh hơn `list.insert(0)` và cách tư duy về hiệu năng.
