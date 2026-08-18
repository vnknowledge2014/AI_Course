# Phụ lục B — Từ F# / TypeScript sang Python

> **Mục tiêu**: bảng tra cứu cho người đã biết FP ở ngôn ngữ khác và cần biết
> "cái này viết bằng Python thế nào".
> **Liên quan**: Chapter 9 (Dataclasses), Chapter 13 (ADTs), Chapter 22 (ROP)

---

## B.1 — Kiểu dữ liệu nền

| Khái niệm | F# | TypeScript | Python |
|---|---|---|---|
| Record bất biến | `type P = { X: int }` | `type P = { readonly x: number }` | `@dataclass(frozen=True)` |
| Tuple | `int * string` | `[number, string]` | `tuple[int, str]` |
| Danh sách | `int list` | `readonly number[]` | `tuple[int, ...]` hoặc `Sequence[int]` |
| Từ điển | `Map<string,int>` | `ReadonlyMap<string,number>` | `Mapping[str, int]` |
| Option | `int option` | `number \| undefined` | `int \| None` |
| Union | `A \| B` | `A \| B` | `A \| B` (PEP 604, 3.10+) |
| Hằng ở mức kiểu | — | `"red" \| "blue"` | `Literal["red", "blue"]` |
| Kiểu đặt tên | `type Email = Email of string` | `type Email = string & {_b: never}` | `NewType("Email", str)` |

```python
from dataclasses import dataclass
from typing import Literal, NewType

# ── Record ──────────────────────────────────────────────────────────────────
@dataclass(frozen=True, slots=True)
class Point:
    x: int
    y: int

p = Point(1, 2)
q = p                      # `with` của F# ⇒ dataclasses.replace
from dataclasses import replace
q = replace(p, x=10)
assert p == Point(1, 2) and q == Point(10, 2)   # p không đổi

# ── Newtype: chống trộn lẫn hai chuỗi khác ý nghĩa ───────────────────────────
Email = NewType("Email", str)
UserId = NewType("UserId", str)

def send(to: Email) -> None: ...
# send(UserId("u1"))       # mypy báo lỗi — đúng như F# single-case union

Color = Literal["red", "green", "blue"]
```

> **`slots=True` đáng bật gần như luôn luôn** cho frozen dataclass: tiết kiệm
> ~40% bộ nhớ và truy cập thuộc tính nhanh hơn, đổi lại mất khả năng gán thuộc
> tính động — thứ mà record bất biến vốn không cần.

---

## B.2 — Sum types và pattern matching

Đây là chỗ Python 3.10+ tiến gần F# nhất.

```fsharp
// F#
type Shape =
    | Circle of radius: float
    | Rect of width: float * height: float

let area s =
    match s with
    | Circle r -> System.Math.PI * r * r
    | Rect (w, h) -> w * h
```

```python
# Python — tương đương trực tiếp
import math
from dataclasses import dataclass
from typing import assert_never


@dataclass(frozen=True)
class Circle:
    radius: float


@dataclass(frozen=True)
class Rect:
    width: float
    height: float


Shape = Circle | Rect          # discriminated union


def area(s: Shape) -> float:
    match s:
        case Circle(radius=r):
            return math.pi * r * r
        case Rect(width=w, height=h):
            return w * h
        case _:
            assert_never(s)    # ⭐ mấu chốt — xem giải thích bên dưới


assert area(Circle(1)) == math.pi
assert area(Rect(2, 3)) == 6
```

> **`assert_never` là thứ khiến `match` của Python thành exhaustive thật sự.**
> Python **không** kiểm tra tính vét cạn của `match`. Nếu bạn thêm biến thể
> `Triangle` vào `Shape`, `match` trên vẫn chạy — chỉ là rơi vào nhánh `_` và
> nổ lúc runtime. Với `assert_never(s)`, mypy sẽ báo lỗi **lúc type-check**:
> `Argument 1 has incompatible type "Triangle"; expected "Never"`.
> Không có dòng đó, bạn mất đúng cái tính năng chính của ADT.

| | F# | TypeScript | Python |
|---|---|---|---|
| Vét cạn kiểm tra bởi | Compiler (mặc định) | `switch` + `never` | mypy + `assert_never` |
| Bỏ sót nhánh thì | Cảnh báo compile | Lỗi compile | Lỗi mypy (nếu có `assert_never`) |

---

## B.3 — Result / Either và Railway-Oriented Programming

```fsharp
// F#
let workflow input =
    input
    |> validate
    |> Result.bind priceOrder
    |> Result.map acknowledge
```

```typescript
// TypeScript (fp-ts)
pipe(input, validate, E.chain(priceOrder), E.map(acknowledge))
```

```python
# Python (returns)
from returns.result import Result, Success, Failure
from returns.pipeline import flow
from returns.pointfree import bind, map_


def workflow(input_: RawOrder) -> Result[Ack, ValidationError]:
    return flow(
        input_,
        validate,               # RawOrder -> Result[Order, ValidationError]
        bind(price_order),      # tương ứng Result.bind / E.chain
        map_(acknowledge),      # tương ứng Result.map / E.map
    )
```

### Bảng tra tên phương thức

| Ý nghĩa | F# `Result` | fp-ts `Either` | Rust `Result` | Python `returns` |
|---|---|---|---|---|
| Bọc thành công | `Ok x` | `E.right(x)` | `Ok(x)` | `Success(x)` |
| Bọc thất bại | `Error e` | `E.left(e)` | `Err(e)` | `Failure(e)` |
| Biến đổi giá trị | `Result.map` | `E.map` | `.map()` | `.map()` |
| Nối hàm cũng trả Result | `Result.bind` | `E.chain` | `.and_then()` | `.bind()` |
| Biến đổi lỗi | `Result.mapError` | `E.mapLeft` | `.map_err()` | `.alt()` |
| Lấy giá trị hoặc mặc định | `Result.defaultValue` | `E.getOrElse` | `.unwrap_or()` | `.value_or()` |
| Bọc hàm ném exception | — | `E.tryCatch` | — | `@safe` decorator |

```python
from returns.result import safe

@safe                                   # bọc exception thành Failure
def parse_age(raw: str) -> int:
    return int(raw)                     # ValueError -> Failure(ValueError(...))

assert parse_age("30").unwrap() == 30
assert parse_age("abc").failure().__class__ is ValueError
```

---

## B.4 — Computation expression ⇄ do-notation

F# có `result { }`, Haskell có `do`. Python không có cú pháp riêng, nhưng
generator cho hiệu quả tương đương:

```fsharp
// F#
let compute = result {
    let! a = parseInt "10"
    let! b = parseInt "20"
    return a + b
}
```

```python
from returns.result import Result, Success
from returns.methods import cond   # noqa: F401  (ví dụ minh hoạ)


def compute() -> Result[int, str]:
    """Mỗi `?` của Rust / `let!` của F# ⇒ một `yield` ở đây."""
    a = yield parse_int("10")
    b = yield parse_int("20")
    return Success(a + b)


# Chương 27 xây dựng helper `do()` biến generator trên thành Result thật sự.
```

Chi tiết cài đặt nằm ở **Chapter 27 — Monads**, mục do-notation.

---

## B.5 — Higher-order functions và composition

| | F# | TypeScript | Python |
|---|---|---|---|
| Pipe | `x \|> f \|> g` | `pipe(x, f, g)` | `toolz.pipe(x, f, g)` |
| Compose | `f >> g` | `flow(f, g)` | `toolz.compose_left(f, g)` |
| Partial application | tự nhiên (curry) | `f(a)` trả hàm | `functools.partial(f, a)` |
| Curry | mặc định | thủ công | `toolz.curry` |

```python
from functools import partial
from toolz import pipe, curry


@curry
def add(a: int, b: int) -> int:
    return a + b


add_10 = add(10)                 # currying như F#
assert add_10(5) == 15

assert pipe(3, add_10, str) == "13"

# functools.partial — cách chuẩn thư viện, không cần dependency ngoài
double = partial(lambda n, x: n * x, 2)
assert double(21) == 42
```

> **Python không curry mặc định** vì hàm Python có tham số mặc định, `*args`,
> `**kwargs` và keyword-only — curry tự động sẽ nhập nhằng. Vì vậy `toolz.curry`
> là opt-in. Trong code production, `functools.partial` thường đủ và dễ đọc hơn.

---

## B.6 — Typeclass ⇄ Protocol

```fsharp
// F# — dùng interface hoặc statically resolved type parameter
type IComparable<'T> =
    abstract CompareTo: 'T -> int
```

```typescript
// TypeScript — interface (structural)
interface Ord<A> { compare(x: A, y: A): -1 | 0 | 1 }
```

```python
from typing import Protocol, runtime_checkable


@runtime_checkable
class Semigroup(Protocol):
    """Structural typing — KHÔNG cần kế thừa, giống interface của TypeScript
    và khác hẳn abstract class của Java."""

    def combine(self, other: "Semigroup") -> "Semigroup": ...


from dataclasses import dataclass

@dataclass(frozen=True)
class Sum:
    value: int

    def combine(self, other: "Sum") -> "Sum":     # không `implements` gì cả
        return Sum(self.value + other.value)


def fold(items: list[Semigroup]) -> Semigroup:
    result = items[0]
    for it in items[1:]:
        result = result.combine(it)
    return result


assert fold([Sum(1), Sum(2), Sum(3)]) == Sum(6)
assert isinstance(Sum(1), Semigroup)      # nhờ @runtime_checkable
```

`Protocol` của Python gần với **structural typing của TypeScript** hơn là
typeclass của Haskell: khớp theo hình dạng, không cần khai báo quan hệ. Xem
Chapter 15 để biết giới hạn (ví dụ `runtime_checkable` chỉ kiểm tra sự tồn tại
của method, không kiểm tra chữ ký).

---

## B.7 — Những chỗ Python KHÔNG theo kịp

Thành thật về giới hạn quan trọng hơn là quảng cáo:

| Tính năng | F# / Haskell | Python |
|---|---|---|
| Immutability được ép buộc | Compiler ép | Chỉ là quy ước — `frozen=True` vẫn lách được qua `object.__setattr__` |
| `match` vét cạn | Compiler ép | Cần mypy + `assert_never`, và chỉ ở type-check |
| Higher-Kinded Types | Có | **Không có** — nên không viết được `Functor` tổng quát |
| Tối ưu đệ quy đuôi | Có | **Không có** — `RecursionError` ở ~1.000 khung |
| Hàm thuần được đảm bảo | Haskell có | Không — bất kỳ hàm nào cũng có thể có side effect |
| Đánh giá lười | Haskell mặc định | Chỉ generator/iterator |

**Hệ quả thực tế:** trong Python bạn *chọn* FP bằng kỷ luật và công cụ (mypy,
`frozen=True`, linter) chứ không được ngôn ngữ ép. Điều đó khiến các quy ước
trong cuốn sách này quan trọng hơn — không có compiler bắt lỗi giúp bạn.
