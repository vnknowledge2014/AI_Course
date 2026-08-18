# Chapter 27 — Monads

> **Bạn sẽ học được**:
> - Monad = Functor + `bind` (flatMap) — giải quyết nesting problem từ Ch26
> - `bind` vs `map`: khi nào dùng cái nào — rule đơn giản
> - Monad Laws — 3 luật đảm bảo chain hoạt động đúng
> - Do-notation bằng generators — viết imperative, chạy monadic
> - `Result`, `Optional`, `list` — TẤT CẢ là Monads
> - Thư viện `returns` — production-grade Monads cho Python
>
> **Yêu cầu trước**: Chapter 26 (Functors — map, nesting problem).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Monad không còn đáng sợ — nó chỉ là `bind`/`flatMap`. Và bạn đã dùng nó suốt.

---

Bạn nhớ vấn đề cuối Ch26 không? `Ok(Ok(42))` — hộp trong hộp. Bạn muốn giá trị bên trong, nhưng bị KẸT hai lớp vỏ.

Nghĩ thế này: bạn gửi thư cho bạn bè qua bưu điện. Bạn bỏ thư vào phong bì (lớp 1). Bưu điện bỏ phong bì vào bao thư lớn (lớp 2). Bạn bè nhận bao thư, mở ra, thấy phong bì, mở tiếp — rồi mới đọc thư. Phiền! Lý tưởng: bưu điện **là phẳng** — bạn bè nhận TRỰC TIẾP phong bì của bạn, không cần bao ngoài.

**Monad** = Functor + **flatten**. `bind` (hay `flatMap`) = `map` rồi `flatten`. Thay vì `Ok(Ok(42))`, bind cho bạn `Ok(42)` — gỡ bỏ lớp vỏ thừa. Và đây là bí mật: bạn đã DÙNG Monad suốt mà không biết! Hàm `list` comprehension với nested `for` chính là list monad. `asyncio.await` chính là Future monad. `and_then` từ Ch21-22 chính là `bind`. Tên khác, pattern GIỐNG.

---

## Monads — Chain dependent computations

`bind` / `flatMap` / `and_then` = Monad. Kết quả step 1 quyết định step 2. Nếu step 1 fail → skip tất cả sau. Đây là "railway" của ROP (Ch22) ở level type class.


## 27.1 — The Envelope Problem

### map tạo ra hộp trong hộp

Hãy bắt đầu bằng ví dụ cụ thể. Ba lookups: tìm user, lấy department ID từ user, tìm department. Mỗi lookup có thể THẤT BẠI (return `None`). Với chỉ `map` (Functor), mỗi lần function return container, bạn thêm MỘT lớp vỏ.

```python
# filename: nesting_problem.py
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

def parse_int(s: str) -> Result:
    try: return Ok(int(s))
    except: return Err(f"Not a number: {s}")

# ❌ map with function that returns Result → NESTED!
result = map_result(Ok("42"), parse_int)
assert result == Ok(Ok(42))  # Ok(Ok(42)) — TWO layers! 😱

# To get 42: result.value.value — ugly and brittle
# And what about Ok(Err("..."))? It's Ok on outside but error inside!

print("Nesting problem shown ✅")
```

### bind = map + flatten

So sánh: `map(fn)(Ok(x))` = `Ok(fn(x))` — LUÔN bọc thêm `Ok`. Nhưng nếu `fn` đã return `Result` rồi? Kết quả: `Ok(Result)` = lồng!

`bind(fn)(Ok(x))` = `fn(x)` — dùng kết quả của `fn` TRỰC TIẾP. Không bọc thêm. Phẳng.

Khác biệt chỉ MỘT dòng code, nhưng hệ quả KHỔNG LỒ.

```python
# filename: bind_solution.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Ok:
    value: object

@dataclass(frozen=True)
class Err:
    error: str

Result = Union[Ok, Err]

# map: bọc kết quả trong Ok
def map_result(r: Result, fn: Callable) -> Result:
    match r:
        case Ok(value=v): return Ok(fn(v))  # ← WRAPS in Ok
        case Err(): return r

# bind: dùng kết quả TRỰC TIẾP (fn đã trả Result rồi)
def bind(r: Result, fn: Callable) -> Result:
    match r:
        case Ok(value=v): return fn(v)      # ← NO wrapping! fn returns Result
        case Err(): return r

def parse_int(s: str) -> Result:
    try: return Ok(int(s))
    except: return Err(f"Not a number: {s}")

def ensure_positive(n: int) -> Result:
    return Ok(n) if n > 0 else Err(f"{n} is not positive")

def ensure_reasonable(n: int) -> Result:
    return Ok(n) if n <= 1_000_000 else Err(f"{n} is too large")

# ✅ bind: FLAT results!
result = bind(Ok("42"), parse_int)
assert result == Ok(42)  # Ok(42) — flat! Not Ok(Ok(42))

result = bind(Ok("abc"), parse_int)
assert result == Err("Not a number: abc")  # error propagated

# ✅ Chain multiple binds — pipeline!
def parse_positive_price(raw: str) -> Result:
    r1 = parse_int(raw)
    r2 = bind(r1, ensure_positive)
    r3 = bind(r2, ensure_reasonable)
    return r3

assert parse_positive_price("42") == Ok(42)
assert parse_positive_price("abc") == Err("Not a number: abc")
assert parse_positive_price("-5") == Err("-5 is not positive")
assert parse_positive_price("9999999") == Err("9999999 is too large")

print("bind solution OK ✅")
```

> **💡 map vs bind — the rule**: Function returns PLAIN value (`int`, `str`) → use `map`. Function returns CONTAINER (`Result`, `Optional`) → use `bind`. Map wraps. Bind doesn't.

---

## ✅ Checkpoint 27.1

> Đến đây bạn phải hiểu:
> 1. **map** wraps: `map(Ok(x), fn) = Ok(fn(x))`. Creates nesting if fn returns container
> 2. **bind** doesn't wrap: `bind(Ok(x), fn) = fn(x)`. Flat result
> 3. **When to use**: fn returns `A` → map. fn returns `Result[A]` → bind
> 4. **bind = flatMap = and_then** (from Ch21-22). Same concept
>
> **Test nhanh**: `bind(Ok(5), lambda x: Ok(x * 2))` = ?
> <details><summary>Đáp án</summary>`Ok(10)`. bind applies lambda to 5, lambda returns `Ok(10)`. bind uses result directly = `Ok(10)`. If we used map: `Ok(Ok(10))` ← nested!</details>

---

## 27.2 — Monad = Functor + bind + unit

### Định nghĩa chính thức

Monad xây dựng TRÊN Functor (Ch26). Nếu Functor = container + map, thì Monad = Functor + hai thứ nữa: `unit` (đưa giá trị vào container — `Ok`, `Some`, `[x]`) và `bind` (map + flatten). Ba thành phần, không hơn.

Bất ngờ: `list`, `Optional`, và cả `asyncio` — TẤT CẢ đều là Monads.

```python
# filename: monad_definition.py
from dataclasses import dataclass
from typing import Union, Callable

# ═══ Result Monad ═══
@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

class ResultMonad:
    @staticmethod
    def unit(value) -> Result:
        return Ok(value)

    @staticmethod
    def map(r: Result, fn: Callable) -> Result:
        match r:
            case Ok(value=v): return Ok(fn(v))
            case Err(): return r

    @staticmethod
    def bind(r: Result, fn: Callable) -> Result:
        match r:
            case Ok(value=v): return fn(v)
            case Err(): return r

# ═══ Optional Monad ═══
class OptionalMonad:
    @staticmethod
    def unit(value):
        return value  # Some = just the value itself

    @staticmethod
    def map(v, fn):
        return None if v is None else fn(v)

    @staticmethod
    def bind(v, fn):
        return None if v is None else fn(v)
        # Note: for Optional, map and bind look similar!
        # Difference: fn in bind returns Optional, fn in map returns plain value

# ═══ List Monad ═══
class ListMonad:
    @staticmethod
    def unit(value) -> list:
        return [value]

    @staticmethod
    def map(lst: list, fn: Callable) -> list:
        return [fn(x) for x in lst]

    @staticmethod
    def bind(lst: list, fn: Callable) -> list:
        """flatMap! Each element → list, then flatten."""
        result = []
        for x in lst:
            result.extend(fn(x))
        return result

```

#### Tests: List flatMap = Monad bind!

```python
# map: each element → element (same length)
assert ListMonad.map([1, 2, 3], lambda x: x * 2) == [2, 4, 6]

# bind: each element → LIST (length changes = flattened)
assert ListMonad.bind([1, 2, 3], lambda x: [x, x * 10]) == [1, 10, 2, 20, 3, 30]

# vs map (creates nested):
assert ListMonad.map([1, 2, 3], lambda x: [x, x * 10]) == [[1, 10], [2, 20], [3, 30]]
# ^^^ nested! bind flattens.

```

#### Practical: cartesian product via List Monad

```python
colors = ["Red", "Blue"]
sizes = ["S", "M", "L"]

combos = ListMonad.bind(colors, lambda c:
    ListMonad.bind(sizes, lambda s:
        [f"{c}-{s}"]
    )
)
assert combos == ["Red-S", "Red-M", "Red-L", "Blue-S", "Blue-M", "Blue-L"]

# Compare with list comprehension (Pythonic list monad!)
combos2 = [f"{c}-{s}" for c in colors for s in sizes]
assert combos == combos2  # Same! List comprehension IS list monad!

print("Monad definition OK ✅")
```

> **💡 List comprehension = List Monad!** `[f(x,y) for x in xs for y in ys]` = `bind(xs, λx → bind(ys, λy → [f(x,y)]))`. Python baked the List Monad into its syntax without calling it that.

---

## 27.3 — Monad Laws

### Ba luật đảm bảo bind hoạt động đúng

Giống Functor có 2 laws (Ch26), Monad có 3 laws. Chúng đảm bảo bạn có thể refactor bind chains tự do.

```python
# filename: monad_laws.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def bind(r: Result, fn: Callable) -> Result:
    match r:
        case Ok(value=v): return fn(v)
        case Err(): return r

unit = Ok  # unit = Ok constructor

f = lambda x: Ok(x * 2) if x > 0 else Err("negative")
g = lambda x: Ok(x + 10)

# ═══ LAW 1: Left Identity ═══
# bind(unit(a), f) == f(a)
# "Wrap in container then bind = just apply function"
a = 5
assert bind(unit(a), f) == f(a)  # Ok(10) == Ok(10) ✅

# ═══ LAW 2: Right Identity ═══
# bind(m, unit) == m
# "Bind with wrapper = no change"
m = Ok(42)
assert bind(m, unit) == m  # Ok(42) == Ok(42) ✅
assert bind(Err("x"), unit) == Err("x")  # Err preserved ✅

# ═══ LAW 3: Associativity ═══
# bind(bind(m, f), g) == bind(m, lambda x: bind(f(x), g))
# "Order of binding doesn't matter"
m = Ok(5)
way1 = bind(bind(m, f), g)                          # bind first, then bind
way2 = bind(m, lambda x: bind(f(x), g))             # nested bind
assert way1 == way2  # Ok(20) == Ok(20) ✅  (5*2=10, 10+10=20)

# Test with failure:
m2 = Ok(-1)
way1_fail = bind(bind(m2, f), g)
way2_fail = bind(m2, lambda x: bind(f(x), g))
assert way1_fail == way2_fail  # Err("negative") == Err("negative") ✅

print("Monad Laws OK ✅")
```

| Law | Nói bằng lời | Ý nghĩa |
|-----|-------------|---------|
| **Left Identity** | unit(a) rồi bind(f) = f(a) | Container wrapping is transparent |
| **Right Identity** | bind(m, unit) = m | Wrapping in bind = no-op |
| **Associativity** | bind order doesn't matter | Can restructure bind, still same result |

---

## 27.4 — Chaining with bind — Real Pipelines

### Multi-step validation pipeline

```python
# filename: monad_pipeline.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def bind(r: Result, fn: Callable) -> Result:
    match r:
        case Ok(value=v): return fn(v)
        case Err(): return r

def map_result(r: Result, fn: Callable) -> Result:
    match r:
        case Ok(value=v): return Ok(fn(v))
        case Err(): return r

```

#### pipe helper

```python
def pipe(value, *fns):
    result = value
    for fn in fns:
        result = fn(result)
    return result

```

#### Domain: User registration

```python
@dataclass(frozen=True)
class UserInput:
    name: str
    email: str
    age: str  # raw string from form

@dataclass(frozen=True)
class ValidUser:
    name: str
    email: str
    age: int

def validate_name(input: UserInput) -> Result:
    name = input.name.strip()
    if len(name) < 2:
        return Err("Name too short (min 2 chars)")
    if len(name) > 50:
        return Err("Name too long (max 50 chars)")
    return Ok(input)

def validate_email(input: UserInput) -> Result:
    if "@" not in input.email:
        return Err(f"Invalid email: {input.email}")
    return Ok(input)

def validate_age(input: UserInput) -> Result:
    try:
        age = int(input.age)
    except ValueError:
        return Err(f"Age '{input.age}' is not a number")
    if age < 0 or age > 150:
        return Err(f"Age {age} out of range (0-150)")
    return Ok(ValidUser(name=input.name.strip(), email=input.email.lower(), age=age))

```

#### Pipeline: bind chains

```python
def register_user(raw: UserInput) -> Result:
    """Chain validations — short-circuit on first error."""
    return pipe(
        Ok(raw),
        lambda r: bind(r, validate_name),
        lambda r: bind(r, validate_email),
        lambda r: bind(r, validate_age),
    )

# Tests
good = register_user(UserInput("Minh", "minh@co.com", "25"))
assert good == Ok(ValidUser(name="Minh", email="minh@co.com", age=25))

bad_name = register_user(UserInput("M", "minh@co.com", "25"))
assert bad_name == Err("Name too short (min 2 chars)")

bad_email = register_user(UserInput("Minh", "no-at", "25"))
assert bad_email == Err("Invalid email: no-at")

bad_age = register_user(UserInput("Minh", "minh@co.com", "abc"))
assert bad_age == Err("Age 'abc' is not a number")

# Short-circuit: bad name → email validation NEVER runs
bad_both = register_user(UserInput("M", "bad", "abc"))
assert bad_both == Err("Name too short (min 2 chars)")  # first error wins

print("Monad pipeline OK ✅")
```

---

## 27.5 — Do-notation: Generator-based Monadic Syntax

### Viết imperative, chạy monadic

Nếu bạn có 5 bước bind, code trông như "callback hell": `bind(r, lambda a: bind(r2, lambda b: ...))`. Python có thuốc: **generators**. Giống `async/await` cho Promise, generators có thể "unwrap" từng bước monadic và viết code TRÔNG sequential.

```python
# filename: do_notation.py
from dataclasses import dataclass
from typing import Union, Callable, Generator

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

# ── Do-notation via generators ──
def do(gen_fn: Callable[[], Generator]) -> Result:
    """
    Monadic do-notation for Result using generators.
    Each `yield` unwraps a Result:
    - Ok(value) → continues with value
    - Err(error) → short-circuits the entire generator
    """
    gen = gen_fn()
    try:
        result = next(gen)  # get first yielded Result
        while True:
            match result:
                case Ok(value=v):
                    result = gen.send(v)  # send unwrapped value back
                case Err():
                    return result  # short-circuit!
    except StopIteration as e:
        return Ok(e.value)  # generator returned = success

```

#### Domain

```python
users = {"U1": {"name": "Minh", "dept_id": "D1"}, "U2": {"name": "Lan"}}
depts = {"D1": {"name": "Engineering", "manager_id": "M1"}}
managers = {"M1": {"name": "Cường", "email": "cuong@co.com"}}

def find_user(uid: str) -> Result:
    return Ok(users[uid]) if uid in users else Err(f"User {uid} not found")

def get_dept_id(user: dict) -> Result:
    did = user.get("dept_id")
    return Ok(did) if did else Err(f"User {user['name']} has no department")

def find_dept(did: str) -> Result:
    return Ok(depts[did]) if did in depts else Err(f"Dept {did} not found")

def get_manager_id(dept: dict) -> Result:
    mid = dept.get("manager_id")
    return Ok(mid) if mid else Err(f"Dept {dept['name']} has no manager")

def find_manager(mid: str) -> Result:
    return Ok(managers[mid]) if mid in managers else Err(f"Manager {mid} not found")

```

#### Without do-notation: nested binds (ugly!)

```python
def bind(r, fn):
    match r:
        case Ok(value=v): return fn(v)
        case Err(): return r

def get_manager_email_ugly(uid: str) -> Result:
    return bind(find_user(uid), lambda user:
        bind(get_dept_id(user), lambda dept_id:
            bind(find_dept(dept_id), lambda dept:
                bind(get_manager_id(dept), lambda mgr_id:
                    bind(find_manager(mgr_id), lambda mgr:
                        Ok(mgr["email"])
                    )
                )
            )
        )
    )

```

#### WITH do-notation: clean sequential code!

```python
def get_manager_email(uid: str) -> Result:
    def gen():
        user = yield find_user(uid)
        dept_id = yield get_dept_id(user)
        dept = yield find_dept(dept_id)
        mgr_id = yield get_manager_id(dept)
        mgr = yield find_manager(mgr_id)
        return mgr["email"]
    return do(gen)

# Both give same results!
assert get_manager_email("U1") == Ok("cuong@co.com")
assert get_manager_email("U2") == Err("User Lan has no department")
assert get_manager_email("U999") == Err("User U999 not found")

assert get_manager_email_ugly("U1") == get_manager_email("U1")
assert get_manager_email_ugly("U999") == get_manager_email("U999")

print("Do-notation OK ✅")
```

> **💡 Do-notation = "write imperative, execute monadic"**: Each `yield` = implicit bind. If any step returns Err → short-circuit. Reads like sequential code, executes like pipeline. Compare: `async/await` is do-notation for the Promise/Future monad!

---

## ✅ Checkpoint 27.2-27.5

> Đến đây bạn phải hiểu:
> 1. **Monad = Functor + unit + bind**. Three operations
> 2. **3 laws**: left identity, right identity, associativity
> 3. **List comprehension = List Monad**. `[f(x,y) for x in xs for y in ys]`
> 4. **Do-notation via generators**: sequential syntax for monadic chains
> 5. **async/await = do-notation for Future Monad**
>
> **Test nhanh**: `bind(Err("fail"), lambda x: Ok(x * 2))` = ?
> <details><summary>Đáp án</summary>`Err("fail")`! bind on Err → returns Err immediately. Lambda NEVER executes. Short-circuit.</details>

---

## 27.6 — The `returns` Library: Production-grade Monads

### The `returns` Library

Chúng ta đã tự viết `bind`, `map`, `do` — giống việc tự đóng bàn để hiểu gỗ. Nhưng production code cần library battle-tested. **`returns`** (dry-python) cung cấp:

- `Maybe` (= Optional Monad)
- `Result` (= Either Monad)
- `IO` (= IO Monad — wraps side effects)
- `Future` (= async Monad)
- `RequiresContext` (= Reader Monad — DI)
- `@safe` decorator — auto-wrap exceptions into Result

#### Bước 1: Ý tưởng từ thư viện returns (Maybe & Result)

```python
# filename: src/monad/returns_step1.py
# NOTE: This is conceptual code showing returns library API.
# Install: pip install returns

# ── Maybe: Optional Monad ──
# from returns.maybe import Maybe, Some, Nothing
#
# result = Some(42).bind(lambda x: Some(x * 2) if x > 0 else Nothing)
# assert result == Some(84)
#
# result = Nothing.bind(lambda x: Some(x * 2))
# assert result == Nothing  # short-circuit
#
# # Chaining
# result = (
#     Some("  Hello World  ")
#     .map(str.strip)
#     .map(str.lower)
#     .bind(lambda s: Some(s) if len(s) > 0 else Nothing)
# )
# assert result == Some("hello world")

# ── Result: Either Monad ──
# from returns.result import Result, Success, Failure, safe
#
# @safe
# def parse_json(raw: str) -> dict:
#     """@safe wraps exceptions → Failure automatically!"""
#     import json
#     return json.loads(raw)
#
# result = parse_json('{"name": "Minh"}')
# assert result == Success({"name": "Minh"})
#
# result = parse_json("invalid json")
# assert isinstance(result, Failure)  # auto-wrapped exception
```

#### Bước 2: Tự tạo một API tương tự

(Dành cho những bạn chưa thể cài đặt thư viện ngay lúc này).

```python
# filename: src/monad/returns_step2.py
# ── Simulated returns-style API ──
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Success:
    _value: object

    def map(self, fn: Callable) -> "Success":
        return Success(fn(self._value))

    def bind(self, fn: Callable) -> "ResultType":
        return fn(self._value)

    @property
    def value(self):
        return self._value

@dataclass(frozen=True)
class Failure:
    _error: object

    def map(self, fn: Callable) -> "Failure":
        return self  # skip

    def bind(self, fn: Callable) -> "Failure":
        return self  # skip

    @property
    def error(self):
        return self._error

ResultType = Union[Success, Failure]

def safe(fn: Callable) -> Callable:
    """Decorator: wrap exceptions → Failure."""
    def wrapper(*args, **kwargs):
        try:
            return Success(fn(*args, **kwargs))
        except Exception as e:
            return Failure(e)
    return wrapper
```

#### Bước 3: Ứng dụng Method Chaining với Monad

```python
# filename: src/monad/returns_step3.py
# ── Using returns-style API ──
@safe
def parse_int(s: str) -> int:
    return int(s)

@safe
def divide(a: int, b: int) -> float:
    return a / b

# Method chaining — Pythonic monadic pipeline!
result = (
    parse_int("42")
    .map(lambda x: x * 2)
    .bind(lambda x: divide(x, 7))
    .map(lambda x: f"Result: {x}")
)
assert result == Success("Result: 12.0")

# Error propagation
result_err = (
    parse_int("abc")
    .map(lambda x: x * 2)          # skipped!
    .bind(lambda x: divide(x, 7))  # skipped!
    .map(lambda x: f"Result: {x}") # skipped!
)
assert isinstance(result_err, Failure)

# Division by zero
result_zero = (
    parse_int("42")
    .bind(lambda x: divide(x, 0))   # Failure(ZeroDivisionError)
    .map(lambda x: f"Result: {x}")  # skipped!
)
assert isinstance(result_zero, Failure)

print("returns-style API OK ✅")
```

> **💡 `@safe` = bridge from exceptions to Results**: Any function decorated with `@safe` catches exceptions and wraps them in `Failure`. No more `try/except` scattered everywhere. Exceptions become VALUES you can pipe through.

---

## 🏋️ Bài tập

**Bài 1** (5 phút): map vs bind

```python
# Cho mỗi case, chọn map hay bind:
# a) Ok(5).???(lambda x: x + 1)       → Result  # fn returns int
# b) Ok(5).???(lambda x: Ok(x + 1))   → Result  # fn returns Result
# c) Ok("42").???(parse_int)            → Result  # parse_int returns Result
# d) Ok(42).???(lambda x: str(x))      → Result  # fn returns str
```

<details><summary>✅ Lời giải Bài 1</summary>

```python
# a) .map()  — fn returns int (plain value)
# b) .bind() — fn returns Result (container)
# c) .bind() — parse_int returns Result (container)
# d) .map()  — fn returns str (plain value)
```

</details>

---

**Bài 2** (10 phút): JSON pipeline with do-notation

```python
# Write a pipeline using do-notation:
# 1. Parse JSON string → dict
# 2. Extract "email" field → str
# 3. Validate email has "@"
# 4. Return lowercased email
# Handle all errors!
```

<details><summary>✅ Lời giải Bài 2</summary>

```python
import json
from dataclasses import dataclass
from typing import Union, Callable, Generator

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def do(gen_fn):
    gen = gen_fn()
    try:
        result = next(gen)
        while True:
            match result:
                case Ok(value=v): result = gen.send(v)
                case Err(): return result
    except StopIteration as e:
        return Ok(e.value)

def safe_json_parse(raw: str) -> Result:
    try: return Ok(json.loads(raw))
    except: return Err(f"Invalid JSON: {raw[:20]}...")

def extract_field(data: dict, field: str) -> Result:
    return Ok(data[field]) if field in data else Err(f"Missing field: {field}")

def validate_email(email: str) -> Result:
    return Ok(email) if "@" in email else Err(f"Invalid email: {email}")

def process_email(raw_json: str) -> Result:
    def gen():
        data = yield safe_json_parse(raw_json)
        email = yield extract_field(data, "email")
        valid = yield validate_email(email)
        return valid.lower()
    return do(gen)

assert process_email('{"email": "Minh@CO.com"}') == Ok("minh@co.com")
assert process_email("bad json") == Err("Invalid JSON: bad json...")
assert process_email('{"name": "Minh"}') == Err("Missing field: email")
assert process_email('{"email": "nope"}') == Err("Invalid email: nope")
```

</details>

---

**Bài 3** (15 phút): List Monad — generate permutations

```python
# Use ListMonad.bind to generate all permutations of a list.
# Hint: for each element, generate "element + permutations of rest"
```

<details><summary>✅ Lời giải Bài 3</summary>

```python
def flat_map(lst: list, fn) -> list:
    result = []
    for x in lst:
        result.extend(fn(x))
    return result

def permutations(items: list) -> list[list]:
    """Generate all permutations using List Monad."""
    if len(items) <= 1:
        return [items]

    return flat_map(
        list(range(len(items))),
        lambda i: [
            [items[i]] + rest
            for rest in permutations(items[:i] + items[i+1:])
        ]
    )

assert sorted(permutations([1, 2, 3])) == [
    [1, 2, 3], [1, 3, 2],
    [2, 1, 3], [2, 3, 1],
    [3, 1, 2], [3, 2, 1],
]
assert permutations([1]) == [[1]]
assert permutations([]) == [[]]
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `Ok(Ok(value))` nested | Used map instead of bind | fn returns container → use bind |
| "Monad quá abstract" | Theory-first thinking | Nhớ: `bind = and_then` từ Ch21. Bạn đã dùng! |
| Deep bind nesting | Nhiều dependent steps | Dùng do-notation (generators) |
| `async/await` là Monad? | Có! | `await promise` = unwrap. Error → skip. = Monad bind |
| Generator do-notation type errors | StopIteration handling | Ensure `do()` catches StopIteration correctly |

---

## 💬 Đối thoại với bản thân: Monad Q&A

**Q: Monad và Functor khác nhau chỗ nào?**

A: Functor `map`: fn KHÔNG trả container. `A → B` + `F[A]` = `F[B]`. Monad `bind`: fn TRẢ container. `A → F[B]` + `F[A]` = `F[B]` (đã flatten). Nếu dùng `map` với fn trả container: `F[F[B]]` — nested! `bind` = `map` + `flatten`.

**Q: List comprehension là Monad thật sao?**

A: `[f(x,y) for x in xs for y in ys]` = `flatMap(xs, λx → flatMap(ys, λy → [f(x,y)]))`. Python designers built the List Monad into syntax. Haskell: `do { x <- xs; y <- ys; return (f x y) }` — identical semantics.

**Q: Khi nào dùng `map` vs `bind`?**

A: Nếu function trả giá trị THƯỜNG (str, int, dict) → `map`. Nếu trả Result/Optional/list → `bind`. Đơn giản thế.

**Q: Tại sao gọi là "Monad" mà không gọi là "Bindable" hay "FlatMappable"?**

A: Tên gốc từ Category Theory (toán học). Trong thực hành: Monad = "chainable container". Tên không quan trọng bằng PATTERN: nhận kết quả bước trước, quyết định bước sau, short-circuit on error.

---

## Tóm tắt

Chương này vén màn Monad — và bạn thấy: chẳng có gì huyền bí. Monad = Functor + `unit` + `bind`. `bind` = map + flatten.

- ✅ **Monad** = Functor + `unit` + `bind`. Solves nested container problem.
- ✅ **bind/flatMap** = map + flatten. `Ok(Ok(42))` → `Ok(42)`.
- ✅ **When**: fn returns plain value → `map`. fn returns container → `bind`.
- ✅ **3 laws**: left/right identity + associativity. Enable safe refactoring.
- ✅ **List Monad**: comprehensions = monadic. `flatMap` = bind.
- ✅ **Do-notation**: generators for sequential-looking monadic code.
- ✅ **`returns` library**: production-grade `Maybe`, `Result`, `IO`, `@safe`.
- ✅ **You've used Monads**: `async/await`, list comprehensions, `.and_then()` (Ch21).

## Tiếp theo

→ Chapter 27b: **Applicative & Validation** — khi bạn cần validate NHIỀU fields song song và thu thập TẤT CẢ lỗi. Monad FAIL ở đây — chỉ bắt lỗi đầu tiên. Applicative giải quyết.
