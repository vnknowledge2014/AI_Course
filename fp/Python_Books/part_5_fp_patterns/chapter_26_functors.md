# Chapter 26 — Functors

> **Bạn sẽ học được**:
> - Functor = "mappable container" — transform contents without opening
> - `map` over `list`, `Optional`, `Result` — tất cả là Functors
> - Functor laws: Identity và Composition — đảm bảo map "lành mạnh"
> - Nested containers: `Optional[Optional[str]]` — vấn đề dẫn đến Monad (Ch27)
> - Thực hành: data pipeline transformations, Protocol-based Functor
>
> **Yêu cầu trước**: Chapter 25 (Abstract Algebra — Eq, Ord, Semigroup, Monoid).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Hiểu tại sao `map()` hoạt động GIỐNG NHAU trên list, Optional, Result — vì tất cả đều là Functors.

---

Bạn có hộp quà. Bên trong là chiếc áo màu trắng. Bạn muốn nhuộm áo thành màu xanh. Cách 1: mở hộp, lấy áo ra, nhuộm, bỏ lại vào. Cách 2: hộp quà CÓ NÚT "nhuộm" — bấm nút, áo bên trong tự đổi màu, bạn KHÔNG CẦN mở hộp. Cách 2 là **Functor**: transform nội dung BÊN TRONG container mà không cần mở ra.

`list`, `Optional`, `Result` — tất cả là "hộp quà" với nút "nhuộm" khác nhau:
- `list`: nút `map()` → nhuộm TẤT CẢ áo bên trong
- `Optional`: nút `map()` → nhuộm áo NẾU CÓ, bỏ qua nếu hộp rỗng
- `Result`: nút `map()` → nhuộm áo NẾU THÀNH CÔNG, giữ nguyên lỗi

Pattern GIỐNG NHAU. Container KHÁC NHAU. Đó là Functor.

---

## Functors — Transform inside containers

Functor = container có `map()`. `map` nhận function `A → B`, áp dụng lên giá trị bên trong container, trả về container mới với giá trị đã transform. Container không thay đổi "hình dạng" — `Some` vẫn là `Some`, `list` vẫn cùng length.


## 26.1 — List: Functor đầu tiên

### map() = transform mỗi phần tử

Bạn đã dùng Functor hàng ngày với `map()` trên list. Hãy nhìn nó dưới góc nhìn mới:

```python
# filename: functor_list.py

# map :: (A → B) → list[A] → list[B]
# Transform EACH element. Container shape (length) unchanged.

numbers = [1, 2, 3, 4, 5]
doubled = list(map(lambda x: x * 2, numbers))
assert doubled == [2, 4, 6, 8, 10]
assert len(doubled) == len(numbers)  # shape preserved!

# List comprehension = map
names = ["minh", "lan", "hải"]
upper_names = [name.upper() for name in names]
assert upper_names == ["MINH", "LAN", "HẢI"]

# Chain maps = pipeline
prices_vnd = [35_000, 50_000, 120_000]
result = list(map(
    lambda p: f"{p:,}đ",                    # format
    map(lambda p: int(p * 0.9),              # discount 10%
        map(lambda p: p + 5_000, prices_vnd) # shipping
    )
))
assert result == ["36,000đ", "49,500đ", "112,500đ"]

print("List Functor OK ✅")
```

Chú ý: `map()` không thay đổi LENGTH. 3 phần tử vào → 3 phần tử ra. Container "shape" được bảo toàn. Đây là property quan trọng của Functor.

---

## 26.2 — Optional: Functor cho giá trị có thể vắng mặt

### map qua None = không làm gì

Python không có `Optional.map()` built-in, nhưng chúng ta có thể tạo:

```python
# filename: functor_optional.py
from typing import TypeVar, Callable

T = TypeVar("T")
U = TypeVar("U")

def map_optional(value: T | None, fn: Callable[[T], U]) -> U | None:
    """Functor map for Optional (T | None)."""
    if value is None:
        return None
    return fn(value)

# ── Some case: function applied ──
assert map_optional(5, lambda x: x * 2) == 10
assert map_optional("hello", str.upper) == "HELLO"
assert map_optional("2025-01-15", lambda d: d.split("-")[0]) == "2025"

# ── None case: function SKIPPED ──
assert map_optional(None, lambda x: x * 2) is None
assert map_optional(None, str.upper) is None

# ── Chaining maps ──
def pipeline(value: str | None) -> str | None:
    result = map_optional(value, str.strip)
    result = map_optional(result, str.lower)
    result = map_optional(result, lambda s: s.replace(" ", "_"))
    return result

assert pipeline("  Hello World  ") == "hello_world"
assert pipeline(None) is None  # entire pipeline skipped!

print("Optional Functor OK ✅")
```

So sánh với cách KHÔNG dùng Functor:

```python
# filename: functor_vs_manual.py

# ❌ Without Functor: nested None checks
def process_bad(value: str | None) -> str | None:
    if value is None:
        return None
    stripped = value.strip()
    if stripped is None:  # redundant but defensive
        return None
    lowered = stripped.lower()
    return lowered.replace(" ", "_")

# ✅ With Functor: clean pipeline
def map_opt(v, fn):
    return None if v is None else fn(v)

def process_good(value: str | None) -> str | None:
    return map_opt(map_opt(map_opt(value, str.strip), str.lower),
                   lambda s: s.replace(" ", "_"))

assert process_bad("  Hello  ") == process_good("  Hello  ") == "hello"
assert process_bad(None) == process_good(None) == None

print("Functor vs manual OK ✅")
```

> **💡 Functor = "auto-skip on empty"**: Map trên `None` = None. Map trên `Some(x)` = `Some(fn(x))`. Bạn KHÔNG CẦN check None ở mỗi bước — Functor tự handle.

---

## 26.3 — Result: Functor cho giá trị có thể thất bại

### map qua Error = giữ nguyên lỗi

```python
# filename: functor_result.py
from dataclasses import dataclass
from typing import TypeVar, Callable, Union

T = TypeVar("T")
U = TypeVar("U")

@dataclass(frozen=True)
class Ok:
    value: object

@dataclass(frozen=True)
class Err:
    error: str

Result = Union[Ok, Err]

def map_result(result: Result, fn: Callable) -> Result:
    """Functor map for Result."""
    match result:
        case Ok(value=v):
            return Ok(fn(v))
        case Err():
            return result  # error preserved, function NOT called

# ── Ok case: function applied ──
assert map_result(Ok(5), lambda x: x * 2) == Ok(10)
assert map_result(Ok("hello"), str.upper) == Ok("HELLO")

# ── Err case: function SKIPPED ──
assert map_result(Err("not found"), lambda x: x * 2) == Err("not found")
assert map_result(Err("timeout"), str.upper) == Err("timeout")

# ── Pipeline: parse → validate → format ──
def parse_age(raw: str) -> Result:
    try:
        return Ok(int(raw))
    except ValueError:
        return Err(f"'{raw}' is not a number")

def validate_age(age: int) -> Result:
    """Not a Functor map (returns Result) — preview of Monad in Ch27!"""
    return Ok(age) if 0 <= age <= 150 else Err(f"Age {age} out of range")

def categorize(age: int) -> str:
    if age < 13: return "child"
    if age < 18: return "teen"
    if age < 65: return "adult"
    return "senior"

# map only transforms the SUCCESS track
result = parse_age("25")
result = map_result(result, categorize)
assert result == Ok("adult")

result = parse_age("abc")
result = map_result(result, categorize)  # skipped!
assert result == Err("'abc' is not a number")

print("Result Functor OK ✅")
```

---

## ✅ Checkpoint 26.1-26.3

> Đến đây bạn phải hiểu:
> 1. **Functor** = container with `map()`. Transform inside, don't open
> 2. **List Functor**: `map(fn, [1,2,3])` → `[fn(1), fn(2), fn(3)]`. Shape preserved
> 3. **Optional Functor**: `map(fn, None)` → `None`. `map(fn, x)` → `fn(x)`. Auto-skip
> 4. **Result Functor**: `map(fn, Err)` → `Err`. `map(fn, Ok(x))` → `Ok(fn(x))`. Error preserved
>
> **Test nhanh**: `map_result(Err("fail"), lambda x: 1/0)` — exception hay không?
> <details><summary>Đáp án</summary>**KHÔNG exception!** Function KHÔNG BAO GIỜ được gọi khi Result là Err. `1/0` sẽ throw, nhưng lambda không execute. Result: `Err("fail")` unchanged.</details>

---

## 26.4 — Functor Laws

### Hai luật đảm bảo map "lành mạnh"

Không phải bất kỳ thứ gì có `map` đều là Functor hợp lệ. Functor phải thỏa 2 luật — chúng đảm bảo `map` "không làm gì bất ngờ": không thêm/xóa phần tử, không đổi container shape, không có side effects ẩn.

**Law 1: Identity** — `map(identity) == identity`. Map với hàm "không làm gì" → không thay đổi.
**Law 2: Composition** — `map(f ∘ g) == map(f) ∘ map(g)`. Map hai lần = map một lần với hàm composed.

```python
# filename: functor_laws.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def map_result(r: Result, fn: Callable) -> Result:
    match r:
        case Ok(value=v): return Ok(fn(v))
        case Err(): return r

def map_optional(v, fn):
    return None if v is None else fn(v)

identity = lambda x: x

# ═══ LAW 1: Identity ═══
# map(identity, container) == container

# List
assert list(map(identity, [1, 2, 3])) == [1, 2, 3]

# Optional
assert map_optional(5, identity) == 5
assert map_optional(None, identity) is None

# Result
assert map_result(Ok(42), identity) == Ok(42)
assert map_result(Err("fail"), identity) == Err("fail")

# ═══ LAW 2: Composition ═══
# map(f ∘ g, container) == map(f, map(g, container))

f = lambda x: x + 1
g = lambda x: x * 2
composed = lambda x: f(g(x))  # f ∘ g

# List
data = [1, 2, 3]
assert list(map(composed, data)) == list(map(f, map(g, data)))
# [3, 5, 7] == [3, 5, 7] ✅

# Optional
assert map_optional(5, composed) == map_optional(map_optional(5, g), f)
# f(g(5)) = f(10) = 11
# map(f, map(g, 5)) = map(f, 10) = 11 ✅

assert map_optional(None, composed) == map_optional(map_optional(None, g), f)
# None == None ✅

# Result
assert map_result(Ok(5), composed) == map_result(map_result(Ok(5), g), f)
assert map_result(Err("x"), composed) == map_result(map_result(Err("x"), g), f)

print("Functor Laws OK ✅")
```

### Tại sao laws quan trọng?

Laws cho phép bạn **refactor tự do**. Nếu bạn thấy hai `map` liên tiếp:
```python
result = map_result(map_result(Ok(5), double), add_one)
```
Bạn có thể gộp thành một `map` (composition law):
```python
result = map_result(Ok(5), lambda x: add_one(double(x)))
```
Kết quả GIỐNG NHAU — compiler/runtime có thể optimize. Nếu laws vi phạm, refactoring này sẽ thay đổi behavior → bugs.

> **💡 Laws = refactoring safety**: Identity law = map(id) changes nothing. Composition law = two maps = one map with composed function. If your "Functor" violates these → it's not a real Functor → refactoring may break things.

---

## 26.5 — The Nesting Problem

### map tạo ra hộp trong hộp

Đây là vấn đề CỐT LÕI dẫn đến Monad (Chapter 27). Khi function bên trong `map` CHÍNH NÓ trả về container, bạn bị LỒng:

```python
# filename: functor_nesting.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def map_result(r: Result, fn: Callable) -> Result:
    match r:
        case Ok(value=v): return Ok(fn(v))
        case Err(): return r

# ── The problem ──
def parse_int(s: str) -> Result:
    """Returns Result — NOT plain value!"""
    try: return Ok(int(s))
    except: return Err(f"Not a number: {s}")

def safe_divide(a: int, b: int) -> Result:
    """Returns Result!"""
    return Ok(a // b) if b != 0 else Err("Division by zero")

# map with function that returns Result → NESTED!
input_result: Result = Ok("42")
nested: Result = map_result(input_result, parse_int)
print(f"Nested: {nested}")  # Ok(Ok(42)) — TWO layers! 😱

# To get 42, you'd need: Ok(Ok(42)).value.value — ugly!
# And what about Ok(Err("..."))? It's inside Ok but it's actually an error!

# ── Optional nesting ──
users: dict[str, dict] = {
    "U1": {"name": "Minh", "email": "minh@co.com"},
    "U2": {"name": "Lan"},  # no email
}

def find_user(uid: str) -> dict | None:
    return users.get(uid)

def get_email(user: dict) -> str | None:
    return user.get("email")

def map_opt(v, fn):
    return None if v is None else fn(v)

# map with function that returns Optional → NESTED!
user = find_user("U1")               # dict | None
email_nested = map_opt(user, get_email)  # str | None | None — wait, that's fine
# But conceptually: Optional[Optional[str]] if we had proper types

# For user that doesn't exist:
user = find_user("U999")  # None
email = map_opt(user, get_email)  # None — OK, but...

# The REAL problem shows with 3 levels:
# map(map(map(find_user("U1"), get_profile), get_address), get_city)
# → Optional[Optional[Optional[str]]] — THREE layers!

print("Nesting problem shown ✅")
```

Giải pháp cho vấn đề nesting? **Monad** — Chapter 27. Monad thêm `bind` (aka `flat_map`, `and_then`) = map + flatten. Thay vì `Ok(Ok(42))`, bind cho bạn `Ok(42)` — phẳng.

> **💡 map creates nesting when function returns container**: fn returns `A` → use `map` → `F[A]`. fn returns `F[A]` → use `map` → `F[F[A]]` (nested!). Solution: `bind`/`flatMap` = Chapter 27.

---

## ✅ Checkpoint 26.4-26.5

> Đến đây bạn phải hiểu:
> 1. **Law 1 (Identity)**: `map(id, x) == x`. Map với "không làm gì" = không đổi
> 2. **Law 2 (Composition)**: `map(f∘g, x) == map(f, map(g, x))`. Hai maps = một map
> 3. **Laws enable refactoring**: Gộp/tách map tự do mà không đổi behavior
> 4. **Nesting problem**: `map(fn_returns_container)` → `F[F[A]]`. Giải pháp: Monad
>
> **Test nhanh**: `map_result(Ok("42"), parse_int)` kết quả gì?
> <details><summary>Đáp án</summary>`Ok(Ok(42))`! parse_int returns `Ok(42)`, map wraps in another Ok → nested. Nếu dùng `bind` (Ch27): kết quả là `Ok(42)` — flat.</details>

---

## 26.6 — Protocol-based Functor

### Abstract Functor cho Python

Python không có Higher-Kinded Types (HKTs) — không thể viết `Functor[F[_]]` như Haskell. Nhưng có thể dùng Protocol + concrete implementations:

```python
# filename: functor_protocol.py
from dataclasses import dataclass
from typing import Callable, TypeVar, Generic

T = TypeVar("T")
U = TypeVar("U")

# ── Box: simplest Functor ──
@dataclass(frozen=True)
class Box(Generic[T]):
    """A container holding exactly one value. The simplest Functor."""
    value: T

    def map(self, fn: Callable[[T], U]) -> "Box[U]":
        return Box(fn(self.value))

# Test Box Functor
box = Box(42)
assert box.map(lambda x: x * 2) == Box(84)
assert box.map(str) == Box("42")

# Laws
identity = lambda x: x
assert box.map(identity) == box  # Identity ✅

f = lambda x: x + 1
g = lambda x: x * 2
assert box.map(lambda x: f(g(x))) == box.map(g).map(f)  # Composition ✅

```

#### Validated: Functor for validation results

```python
from typing import Union

@dataclass(frozen=True)
class Valid(Generic[T]):
    value: T

    def map(self, fn: Callable) -> "Valid":
        return Valid(fn(self.value))

@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]

    def map(self, fn: Callable) -> "Invalid":
        return self  # errors preserved, function not called

Validated = Union[Valid, Invalid]

def map_validated(v: Validated, fn: Callable) -> Validated:
    match v:
        case Valid(value=val): return Valid(fn(val))
        case Invalid(): return v

# Tests
assert map_validated(Valid(10), lambda x: x + 5) == Valid(15)
assert map_validated(Invalid(("too short",)), lambda x: x + 5) == Invalid(("too short",))

```

#### Tree: recursive Functor

```python
@dataclass(frozen=True)
class Leaf(Generic[T]):
    value: T

@dataclass(frozen=True)
class Branch(Generic[T]):
    left: "Tree"
    right: "Tree"

Tree = Union[Leaf, Branch]

def map_tree(tree: Tree, fn: Callable) -> Tree:
    """Functor map for binary tree — applies fn to ALL leaves."""
    match tree:
        case Leaf(value=v):
            return Leaf(fn(v))
        case Branch(left=l, right=r):
            return Branch(map_tree(l, fn), map_tree(r, fn))

# Test Tree Functor
tree = Branch(
    Branch(Leaf(1), Leaf(2)),
    Leaf(3),
)
doubled_tree = map_tree(tree, lambda x: x * 2)
assert doubled_tree == Branch(
    Branch(Leaf(2), Leaf(4)),
    Leaf(6),
)

# Identity law
assert map_tree(tree, identity) == tree

print("Protocol-based Functors OK ✅")
```

> **💡 Functor pattern is universal**: Box, List, Optional, Result, Validated, Tree — tất cả đều là Functors. Pattern: "apply function to contents, preserve container structure." Python thiếu HKTs nên không thể abstract thành một trait duy nhất, nhưng PATTERN giống nhau.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): map vs manual None checks

```python
# Rewrite this using map_optional:
def get_domain(email: str | None) -> str | None:
    if email is None:
        return None
    parts = email.split("@")
    if len(parts) != 2:
        return None
    return parts[1].lower()
```

<details><summary>✅ Lời giải Bài 1</summary>

```python
def map_opt(v, fn):
    return None if v is None else fn(v)

def get_domain(email: str | None) -> str | None:
    def extract(e: str) -> str | None:
        parts = e.split("@")
        return parts[1].lower() if len(parts) == 2 else None
    return map_opt(email, extract)
    # Note: extract returns Optional → this creates nesting!
    # We'll solve this with bind/flatMap in Ch27

assert get_domain("minh@company.com") == "company.com"
assert get_domain(None) is None
```

</details>

---

**Bài 2** (10 phút): Functor cho `Lazy[T]`

```python
# Implement Lazy — a container that defers computation until .run()
# Lazy.map(fn) should create a NEW Lazy, not execute fn
# Only .run() triggers execution
```

<details><summary>✅ Lời giải Bài 2</summary>

```python
from dataclasses import dataclass
from typing import Callable, TypeVar

T = TypeVar("T")
U = TypeVar("U")

@dataclass(frozen=True)
class Lazy:
    _thunk: Callable[[], object]

    def map(self, fn: Callable) -> "Lazy":
        """Create new Lazy — does NOT execute fn yet!"""
        outer_thunk = self._thunk
        return Lazy(lambda: fn(outer_thunk()))

    def run(self):
        """Execute the deferred computation."""
        return self._thunk()

# Test
counter = {"calls": 0}

def expensive():
    counter["calls"] += 1
    return 42

lazy = Lazy(expensive)
assert counter["calls"] == 0  # NOT executed yet!

mapped = lazy.map(lambda x: x * 2)
assert counter["calls"] == 0  # STILL not executed!

result = mapped.run()
assert result == 84
assert counter["calls"] == 1  # NOW executed

# Laws
identity = lambda x: x
lazy2 = Lazy(lambda: 10)
assert lazy2.map(identity).run() == lazy2.run()  # Identity ✅

f = lambda x: x + 1
g = lambda x: x * 2
assert lazy2.map(lambda x: f(g(x))).run() == lazy2.map(g).map(f).run()  # Composition ✅
```

</details>

---

**Bài 3** (15 phút): Verify Functor laws cho `map_tree`

```python
# 1. Build a tree with 5+ leaves
# 2. Verify Identity law
# 3. Verify Composition law with two non-trivial functions
# 4. Test with Leaf-only tree (edge case)
```

<details><summary>✅ Lời giải Bài 3</summary>

```python
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Leaf:
    value: object
@dataclass(frozen=True)
class Branch:
    left: object
    right: object
Tree = Union[Leaf, Branch]

def map_tree(tree: Tree, fn: Callable) -> Tree:
    match tree:
        case Leaf(value=v): return Leaf(fn(v))
        case Branch(left=l, right=r): return Branch(map_tree(l, fn), map_tree(r, fn))

identity = lambda x: x

# Build tree:   /\
#              /\  5
#             /\ 3
#            1  2
tree = Branch(
    Branch(
        Branch(Leaf(1), Leaf(2)),
        Leaf(3),
    ),
    Leaf(5),
)

# Law 1: Identity
assert map_tree(tree, identity) == tree

# Law 2: Composition
f = lambda x: x ** 2   # square
g = lambda x: x + 10   # add 10
composed = lambda x: f(g(x))

assert map_tree(tree, composed) == map_tree(map_tree(tree, g), f)

# Edge case: single leaf
single = Leaf(42)
assert map_tree(single, identity) == single
assert map_tree(single, composed) == map_tree(map_tree(single, g), f)
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `Optional[Optional[A]]` nested | map với fn trả Optional | Dùng `bind`/`flatMap` (Ch27) thay map |
| `Ok(Ok(value))` nested | map với fn trả Result | Dùng `bind`/`flatMap` (Ch27) thay map |
| Map thay đổi container length | Implementation sai | Functor map PHẢI preserve shape |
| Map có side effects | fn không pure | Functor map nên dùng pure functions |
| "Functor quá abstract" | Theory-first thinking | Nhớ: Functor = container + map. Bạn đã dùng `list(map(...))` |

---

## 💬 Đối thoại với bản thân: Functor Q&A

**Q: Python `map()` built-in đã là Functor rồi, tại sao cần học thêm?**

A: `map()` chỉ hoạt động cho iterables. Functor CONCEPT áp dụng cho Optional, Result, Tree, Lazy, Future, IO — BẤT KỲ container nào. Khi bạn hiểu Functor, bạn thấy cùng pattern KHẮP NƠI.

**Q: Tại sao Python không có `Optional.map()`?**

A: Thiếu sót thiết kế. Rust có `Option::map()`, Haskell có `fmap`, Java có `Optional.map()`. Python phải tự viết — nhưng thư viện `returns` cung cấp sẵn (Ch27).

**Q: Functor có giống Decorator pattern không?**

A: Decorator THÊM behavior bên ngoài. Functor TRANSFORM behavior bên trong. Decorator = wrapping. Functor = reaching inside without unwrapping. Khác nhau ở chiều.

**Q: Khi nào dùng `map` vs khi nào dùng `bind`?**

A: Rule đơn giản: function trả **giá trị thuần** (`int`, `str`) → dùng `map`. Function trả **container** (`Result`, `Optional`) → dùng `bind` (Ch27). Map bọc thêm, bind giữ phẳng.

---

## Tóm tắt

Functor là một trong những abstractions CỐT LÕI của FP — và bạn đã dùng nó mỗi ngày với `map()`:

- ✅ **Functor** = container + `map()`. Transform inside, preserve shape.
- ✅ **List**: `map(fn, [1,2,3])` → `[fn(1), fn(2), fn(3)]`.
- ✅ **Optional**: `map(fn, None)` → `None`. `map(fn, x)` → `fn(x)`.
- ✅ **Result**: `map(fn, Err)` → `Err`. `map(fn, Ok(x))` → `Ok(fn(x))`.
- ✅ **Laws**: Identity + Composition. Enable safe refactoring.
- ✅ **Nesting problem**: map + fn returns container → `F[F[A]]`. Solution: Monad.
- ✅ **Universal pattern**: List, Optional, Result, Tree, Lazy, Box — all Functors.

## Tiếp theo

→ Chapter 27: **Monads** — giải quyết vấn đề nesting. `bind`/`flatMap` = map + flatten. Bạn đã dùng nó: `list.flatMap()` trong JS, `.and_then()` trong Rust, `>>=` trong Haskell. Python: `returns.Maybe.bind()`.
