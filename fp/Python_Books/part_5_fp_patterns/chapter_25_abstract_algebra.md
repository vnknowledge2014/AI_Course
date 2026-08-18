# Chapter 25 — Abstract Algebra for Programmers

> **Bạn sẽ học được**:
> - **Eq** — so sánh bằng có cấu trúc, beyond `==`
> - **Ord** — sắp xếp tùy chỉnh, beyond `<`
> - **Semigroup** — phép kết hợp hai giá trị cùng loại
> - **Monoid** — Semigroup + phần tử trung tính (identity)
> - **`functools.reduce`** = fold bất kỳ Monoid nào
> - **Practical**: merge configs, aggregate reports, combine permissions, MapReduce
>
> **Yêu cầu trước**: Chapter 12 (Composition & Pipelines), Chapter 13 (ADTs).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Thấy pattern "combine" và "compare" KHẮP NƠI — và biết cách abstract chúng.

---

Bạn dùng `+` hàng ngày: `1 + 2 + 3`. Bạn dùng `==` hàng ngày: `if x == y`. Bạn dùng `sorted()` hàng ngày. Nhưng bạn có biết rằng **ba thứ này** đều là instances của các cấu trúc toán học cổ đại?

Functional Programming không phát minh ra những thứ này — nó CHỈ ĐẶT TÊN cho chúng. Và khi bạn có tên, bạn có thể ABSTRACT: viết code hoạt động với BẤT KỲ thứ gì "có thể so sánh" (`Eq`), "có thể sắp xếp" (`Ord`), hoặc "có thể ghép lại" (`Monoid`). Thay vì viết riêng logic merge cho configs, reports, permissions — bạn viết MỘT LẦN `concat_all()` và dùng lại mọi nơi.

Chương này sẽ dạy bạn bốn "siêu năng lực" theo đúng thứ tự: `Eq` → `Ord` → `Semigroup` → `Monoid`. Mỗi cái xây dựng trên cái trước. Cuối chương, bạn sẽ thấy `functools.reduce()` — hàm bạn đã dùng — chính là Monoid ở dạng thô.

---

## Eq, Ord, Semigroup, Monoid — The Algebra of Programming

Abstract algebra cho programmers: bốn "interface" nền tảng mà FP dùng để generalize patterns lặp đi lặp lại. Eq = so sánh. Ord = sắp xếp. Semigroup = ghép. Monoid = ghép + phần tử rỗng. Bạn đã dùng tất cả — chương này cho bạn TÊN và PROTOCOL để tái sử dụng có hệ thống.


## 25.1 — Eq: So sánh có cấu trúc

### Vượt qua `==`

Python `==` hoạt động tốt cho primitives: `3 == 3`, `"abc" == "abc"`. Và `@dataclass` tự generate `__eq__` dựa trên fields. Nhưng đôi khi bạn cần **so sánh tùy chỉnh**: so sánh users chỉ theo ID (bỏ qua name), so sánh emails case-insensitive, so sánh configs bỏ qua comments.

Đây là lúc `Eq` Protocol tỏa sáng — abstract hóa "cách so sánh" thành một giá trị bạn có thể truyền vào, thay đổi, compose.

```python
# filename: eq_protocol.py
from typing import Protocol, TypeVar, Callable
from dataclasses import dataclass

T = TypeVar("T")

# Eq = "a way to compare two values of the same type"
class Eq(Protocol[T]):
    def equals(self, x: T, y: T) -> bool: ...

# ── Concrete Eq instances ──

@dataclass(frozen=True)
class EqNumber:
    """Standard number equality."""
    def equals(self, x: int | float, y: int | float) -> bool:
        return x == y

@dataclass(frozen=True)
class EqString:
    """Standard string equality."""
    def equals(self, x: str, y: str) -> bool:
        return x == y

@dataclass(frozen=True)
class EqStringCaseInsensitive:
    """Case-insensitive string equality."""
    def equals(self, x: str, y: str) -> bool:
        return x.lower() == y.lower()

# Tests
eq_num = EqNumber()
assert eq_num.equals(42, 42) == True
assert eq_num.equals(42, 43) == False

eq_str = EqString()
assert eq_str.equals("hello", "hello") == True
assert eq_str.equals("hello", "Hello") == False

eq_ci = EqStringCaseInsensitive()
assert eq_ci.equals("hello", "Hello") == True
assert eq_ci.equals("Hà Nội", "hà nội") == True

print("Eq basics OK ✅")
```

### Factory function — tạo Eq từ bất kỳ logic nào

Thay vì tạo class cho mỗi loại equality, dùng factory:

```python
# filename: eq_factory.py
from dataclasses import dataclass
from typing import Callable, TypeVar

T = TypeVar("T")

@dataclass(frozen=True)
class EqBy:
    """Create Eq from any comparison function."""
    _fn: Callable  # (x, y) -> bool

    def equals(self, x, y) -> bool:
        return self._fn(x, y)

# From key function — compare by extracted key
def eq_by_key(key_fn: Callable) -> EqBy:
    return EqBy(lambda x, y: key_fn(x) == key_fn(y))

# ── Domain example: User ──
@dataclass(frozen=True)
class User:
    id: int
    name: str
    email: str

# Compare by ID only (ignore name/email changes)
eq_user_by_id = eq_by_key(lambda u: u.id)

user_v1 = User(1, "Minh", "minh@old.com")
user_v2 = User(1, "Minh Nguyễn", "minh@new.com")  # same ID, updated info
user_v3 = User(2, "Lan", "lan@co.com")

assert eq_user_by_id.equals(user_v1, user_v2) == True   # same user!
assert eq_user_by_id.equals(user_v1, user_v3) == False   # different user

# Compare by email domain
eq_same_company = eq_by_key(lambda u: u.email.split("@")[1])
assert eq_same_company.equals(
    User(1, "A", "a@company.com"),
    User(2, "B", "b@company.com"),
) == True  # same domain!

# ── Generic utilities using Eq ──
def elem(eq: EqBy, value, items: list) -> bool:
    """Check if value exists in list, using custom Eq."""
    return any(eq.equals(value, item) for item in items)

def uniq(eq: EqBy, items: list) -> list:
    """Deduplicate list using custom Eq."""
    result = []
    for item in items:
        if not elem(eq, item, result):
            result.append(item)
    return result

users = [
    User(1, "Minh v1", "minh@a.com"),
    User(1, "Minh v2", "minh@b.com"),  # same ID
    User(2, "Lan", "lan@c.com"),
]

unique_users = uniq(eq_user_by_id, users)
assert len(unique_users) == 2  # deduped by ID
assert unique_users[0].name == "Minh v1"  # keeps first

print("Eq factory + utilities OK ✅")
```

> **💡 Eq abstracts "equality"**: Thay vì hardcode `==` trong logic, bạn truyền Eq VÀO. Dedup by ID? `uniq(eq_by_id, users)`. Dedup by email? `uniq(eq_by_email, users)`. Cùng function, khác behavior.

---

## ✅ Checkpoint 25.1

> Đến đây bạn phải hiểu:
> 1. **Eq** = "cách so sánh bằng". Protocol với `equals(x, y) -> bool`
> 2. **eq_by_key** = tạo Eq từ key function. Flexible
> 3. **Generic utilities** (`elem`, `uniq`) nhận Eq → hoạt động với BẤT KỲ kiểu so sánh nào
> 4. **Practical**: dedup users by ID, compare emails case-insensitive
>
> **Test nhanh**: `eq_by_key(lambda x: x % 2).equals(3, 7)` = ?
> <details><summary>Đáp án</summary>`True`! Cả 3 và 7 đều lẻ: `3 % 2 = 1`, `7 % 2 = 1`. Key giống nhau → equals = True.</details>

---

## 25.2 — Ord: Sắp xếp có cấu trúc

### Vượt qua `sorted()` default

`sorted()` dùng `<` operator — hoạt động cho numbers, strings. Nhưng khi bạn cần sắp xếp **domain objects** (users by age, orders by priority, tasks by deadline), bạn cần Ord — "cách so sánh thứ tự" tùy chỉnh.

#### Bước 1: Ord Protocol

```python
# filename: src/algebra/ord_step1.py
from dataclasses import dataclass
from typing import Callable, TypeVar
from enum import IntEnum

T = TypeVar("T")

class Ordering(IntEnum):
    LT = -1  # Less Than
    EQ = 0   # Equal
    GT = 1   # Greater Than

@dataclass(frozen=True)
class Ord:
    """Total ordering: compare any two values."""
    _compare: Callable  # (x, y) -> Ordering

    def compare(self, x, y) -> Ordering:
        return self._compare(x, y)

    def equals(self, x, y) -> bool:
        """Ord implies Eq — equal if compare returns EQ."""
        return self.compare(x, y) == Ordering.EQ
```

#### Bước 2: Factory & Utilities

```python
# filename: src/algebra/ord_step2.py
# ── Factory ──
def ord_by_key(key_fn: Callable) -> Ord:
    def compare(x, y) -> Ordering:
        kx, ky = key_fn(x), key_fn(y)
        if kx < ky: return Ordering.LT
        if kx > ky: return Ordering.GT
        return Ordering.EQ
    return Ord(compare)

def ord_reverse(o: Ord) -> Ord:
    """Reverse an ordering — descending."""
    def compare(x, y) -> Ordering:
        result = o.compare(x, y)
        if result == Ordering.LT: return Ordering.GT
        if result == Ordering.GT: return Ordering.LT
        return Ordering.EQ
    return Ord(compare)

# ── Utilities ──
def sort_by(o: Ord, items: list) -> list:
    """Sort using custom Ord."""
    from functools import cmp_to_key
    return sorted(items, key=cmp_to_key(lambda x, y: o.compare(x, y).value))

def minimum(o: Ord, items: list):
    """Find minimum using custom Ord."""
    result = items[0]
    for item in items[1:]:
        if o.compare(item, result) == Ordering.LT:
            result = item
    return result

def maximum(o: Ord, items: list):
    """Find maximum using custom Ord."""
    result = items[0]
    for item in items[1:]:
        if o.compare(item, result) == Ordering.GT:
            result = item
    return result

def clamp(o: Ord, lo, hi, value):
    """Clamp value between lo and hi."""
    if o.compare(value, lo) == Ordering.LT: return lo
    if o.compare(value, hi) == Ordering.GT: return hi
    return value
```

#### Bước 3: Áp dụng vào Domain

```python
# filename: src/algebra/ord_step3.py
# ── Domain ──
@dataclass(frozen=True)
class Task:
    title: str
    priority: int  # 1 = highest
    deadline: str  # ISO date

tasks = [
    Task("Deploy", 2, "2025-01-15"),
    Task("Fix bug", 1, "2025-01-10"),
    Task("Write docs", 3, "2025-01-20"),
    Task("Review PR", 1, "2025-01-12"),
]

# Sort by priority (ascending — 1 is highest)
ord_priority = ord_by_key(lambda t: t.priority)
by_priority = sort_by(ord_priority, tasks)
assert by_priority[0].title == "Fix bug"
assert by_priority[1].title == "Review PR"

# Sort by deadline (ascending — earliest first)
ord_deadline = ord_by_key(lambda t: t.deadline)
by_deadline = sort_by(ord_deadline, tasks)
assert by_deadline[0].title == "Fix bug"  # Jan 10

# Sort by priority DESC (reverse)
by_priority_desc = sort_by(ord_reverse(ord_priority), tasks)
assert by_priority_desc[0].title == "Write docs"  # priority 3 first

# Min/Max
assert minimum(ord_priority, tasks).title == "Fix bug"
assert maximum(ord_priority, tasks).title == "Write docs"

# Clamp
ord_int = ord_by_key(lambda x: x)
assert clamp(ord_int, 0, 100, 150) == 100
assert clamp(ord_int, 0, 100, -5) == 0
assert clamp(ord_int, 0, 100, 42) == 42

print("Ord OK ✅")
```

> **💡 Ord builds on Eq**: Nếu bạn có Ord, bạn tự động có Eq (compare returns EQ = equals). Đây là "type class hierarchy": `Eq` → `Ord`. Tương tự `PartialEq` → `Ord` trong Rust, hoặc `Eq a => Ord a` trong Haskell.

---

## ✅ Checkpoint 25.2

> Đến đây bạn phải hiểu:
> 1. **Ord** = "cách so sánh thứ tự". `compare(x, y) -> LT | EQ | GT`
> 2. **ord_by_key** = tạo Ord từ key function. `sort_by(ord_priority, tasks)`
> 3. **ord_reverse** = đảo thứ tự. Ascending → Descending
> 4. **Utilities**: `sort_by`, `minimum`, `maximum`, `clamp` — tất cả nhận Ord
> 5. **Ord implies Eq**: Nếu có thể sắp xếp → có thể so sánh bằng
>
> **Test nhanh**: `ord_by_key(len).compare("abc", "abcde")` = ?
> <details><summary>Đáp án</summary>`Ordering.LT`! `len("abc") = 3`, `len("abcde") = 5`. `3 < 5` → LT.</details>

---

## 25.3 — Semigroup: Ghép hai thứ lại

### Phép kết hợp (association)

Bạn biết cách ghép Lego không? Lấy 2 khối, ghép thành 1. Lấy kết quả, ghép thêm khối nữa. Cứ thế. Semigroup là tên toán học cho pattern này: **một phép kết hợp hai giá trị cùng kiểu thành một giá trị cùng kiểu**, với tính chất **associativity** — thứ tự grouping không ảnh hưởng kết quả.

Tại sao associativity quan trọng? Vì nó cho phép **parallelize**. Nếu bạn có `a ⊕ b ⊕ c ⊕ d`, bạn có thể tính `(a ⊕ b)` và `(c ⊕ d)` SONG SONG, rồi ghép kết quả. Đây là nền tảng của MapReduce (Google), Spark, và mọi hệ thống distributed aggregation.

```python
# filename: semigroup.py
from dataclasses import dataclass
from typing import Protocol, TypeVar, Callable

T = TypeVar("T")

class Semigroup(Protocol[T]):
    def combine(self, x: T, y: T) -> T: ...

# ── Concrete instances ──

@dataclass(frozen=True)
class SemigroupSum:
    """Number addition semigroup."""
    def combine(self, x: int | float, y: int | float) -> int | float:
        return x + y

@dataclass(frozen=True)
class SemigroupProduct:
    """Number multiplication semigroup."""
    def combine(self, x: int | float, y: int | float) -> int | float:
        return x * y

@dataclass(frozen=True)
class SemigroupString:
    """String concatenation semigroup."""
    def combine(self, x: str, y: str) -> str:
        return x + y

@dataclass(frozen=True)
class SemigroupList:
    """List concatenation semigroup."""
    def combine(self, x: list, y: list) -> list:
        return x + y

@dataclass(frozen=True)
class SemigroupMax:
    """Maximum semigroup."""
    def combine(self, x, y):
        return max(x, y)

@dataclass(frozen=True)
class SemigroupMin:
    """Minimum semigroup."""
    def combine(self, x, y):
        return min(x, y)

# ── Tests ──
sg_sum = SemigroupSum()
assert sg_sum.combine(3, 4) == 7

sg_prod = SemigroupProduct()
assert sg_prod.combine(3, 4) == 12

sg_str = SemigroupString()
assert sg_str.combine("hello", " world") == "hello world"

sg_list = SemigroupList()
assert sg_list.combine([1, 2], [3, 4]) == [1, 2, 3, 4]

sg_max = SemigroupMax()
assert sg_max.combine(3, 7) == 7

# ── Associativity test ──
a, b, c = 1, 2, 3
assert sg_sum.combine(sg_sum.combine(a, b), c) == sg_sum.combine(a, sg_sum.combine(b, c))
# (1 + 2) + 3 == 1 + (2 + 3) == 6 ✅

assert sg_str.combine(sg_str.combine("a", "b"), "c") == sg_str.combine("a", sg_str.combine("b", "c"))
# ("a" + "b") + "c" == "a" + ("b" + "c") == "abc" ✅

print("Semigroup OK ✅")
```

> **💡 Semigroup = "combinable"**: Bất cứ khi nào bạn ghép hai thứ cùng loại → bạn đang dùng Semigroup. `+` (numbers), `+` (strings), `+` (lists), `max`, `min`, `|` (dicts) — TẤT CẢ là Semigroups.

---

## 25.4 — Monoid: Semigroup + Identity

### Phần tử trung tính

Semigroup chỉ biết GHÉP — nhưng không có "giá trị khởi đầu". Monoid thêm `empty` — phần tử TRUNG TÍNH. Ghép với bất kỳ thứ gì → không đổi:
- `0` cho phép cộng: `x + 0 = x`
- `1` cho phép nhân: `x * 1 = x`
- `""` cho string: `x + "" = x`
- `[]` cho list: `x + [] = x`
- `True` cho AND: `x and True = x`
- `False` cho OR: `x or False = x`

Tại sao cần `empty`? Vì `reduce()` cần giá trị khởi đầu. `reduce(lambda a,b: a+b, [], ???)` — `???` chính là `empty`. Và: `concat_all([])` phải return GÌ? Phải là `empty`.

```python
# filename: monoid.py
from dataclasses import dataclass
from functools import reduce
from typing import Protocol, TypeVar

T = TypeVar("T")

class Monoid(Protocol[T]):
    def combine(self, x: T, y: T) -> T: ...
    def empty(self) -> T: ...

# ── Concrete Monoids ──

@dataclass(frozen=True)
class MonoidSum:
    def combine(self, x: int, y: int) -> int: return x + y
    def empty(self) -> int: return 0

@dataclass(frozen=True)
class MonoidProduct:
    def combine(self, x: int, y: int) -> int: return x * y
    def empty(self) -> int: return 1

@dataclass(frozen=True)
class MonoidString:
    def combine(self, x: str, y: str) -> str: return x + y
    def empty(self) -> str: return ""

@dataclass(frozen=True)
class MonoidList:
    def combine(self, x: list, y: list) -> list: return x + y
    def empty(self) -> list: return []

@dataclass(frozen=True)
class MonoidAll:
    """Boolean AND monoid."""
    def combine(self, x: bool, y: bool) -> bool: return x and y
    def empty(self) -> bool: return True  # x and True = x

@dataclass(frozen=True)
class MonoidAny:
    """Boolean OR monoid."""
    def combine(self, x: bool, y: bool) -> bool: return x or y
    def empty(self) -> bool: return False  # x or False = x

@dataclass(frozen=True)
class MonoidDict:
    """Dict merge monoid (last-wins)."""
    def combine(self, x: dict, y: dict) -> dict: return {**x, **y}
    def empty(self) -> dict: return {}

```

#### concat_all = reduce with Monoid

```python
def concat_all(monoid, values: list):
    """Fold a list using a Monoid. Like reduce() but explicit."""
    return reduce(monoid.combine, values, monoid.empty())

```

#### Tests

```python
m_sum = MonoidSum()
assert concat_all(m_sum, [1, 2, 3, 4, 5]) == 15
assert concat_all(m_sum, []) == 0  # empty list → identity

m_prod = MonoidProduct()
assert concat_all(m_prod, [1, 2, 3, 4, 5]) == 120
assert concat_all(m_prod, []) == 1

m_str = MonoidString()
assert concat_all(m_str, ["hello", " ", "world"]) == "hello world"
assert concat_all(m_str, []) == ""

m_list = MonoidList()
assert concat_all(m_list, [[1, 2], [3], [4, 5]]) == [1, 2, 3, 4, 5]

m_all = MonoidAll()
assert concat_all(m_all, [True, True, True]) == True
assert concat_all(m_all, [True, False, True]) == False
assert concat_all(m_all, []) == True  # vacuous truth

m_any = MonoidAny()
assert concat_all(m_any, [False, False, True]) == True
assert concat_all(m_any, [False, False]) == False
assert concat_all(m_any, []) == False

# Identity law: combine(empty, x) == combine(x, empty) == x
assert m_sum.combine(m_sum.empty(), 5) == 5
assert m_sum.combine(5, m_sum.empty()) == 5
assert m_str.combine(m_str.empty(), "hello") == "hello"

print("Monoid + concat_all OK ✅")
```

> **💡 concat_all = reduce with Monoid!** `reduce(lambda a,b: a+b, [1,2,3], 0)` = `concat_all(MonoidSum(), [1,2,3])`. Cùng thứ, nhưng Monoid đặt TÊN cho pattern, và `concat_all` hoạt động với BẤT KỲ Monoid nào.

---

## ✅ Checkpoint 25.3-25.4

> Đến đây bạn phải hiểu:
> 1. **Semigroup** = type + `combine`. Phải associative
> 2. **Monoid** = Semigroup + `empty`. Identity element
> 3. **`concat_all`** = fold list with Monoid. `reduce()` abstracted
> 4. **Many Monoids**: sum(0), product(1), string(""), list([]), all(True), any(False), dict({})
>
> **Test nhanh**: `concat_all(MonoidProduct(), [])` = ?
> <details><summary>Đáp án</summary>`1`! Empty list → return identity element. For product, identity = 1 (multiply by 1 = no change).</details>

---

## 25.5 — Practical Monoids

### Config merging = Monoid

Đây là nơi Monoid tỏa sáng. Bạn có configs từ nhiều nguồn: defaults, environment variables, CLI flags. Cần merge theo thứ tự ưu tiên. Đó chính xác là `concat_all(monoid_config, [defaults, env, cli])`. Mỗi nguồn = một "khối Lego".

```python
# filename: monoid_config.py
from dataclasses import dataclass
from functools import reduce

@dataclass(frozen=True)
class AppConfig:
    port: int
    host: str
    debug: bool
    features: tuple[str, ...]  # immutable list

@dataclass(frozen=True)
class MonoidConfig:
    _default: AppConfig = AppConfig(port=3000, host="localhost", debug=False, features=())

    def empty(self) -> AppConfig:
        return self._default

    def combine(self, base: AppConfig, override: AppConfig) -> AppConfig:
        default = self._default
        return AppConfig(
            port=override.port if override.port != default.port else base.port,
            host=override.host if override.host != default.host else base.host,
            debug=base.debug or override.debug,  # OR: any True → True
            features=base.features + override.features,  # combine tuples
        )

def concat_all(monoid, values: list):
    return reduce(monoid.combine, values, monoid.empty())

# Layer configs: defaults → env → cli
m = MonoidConfig()
defaults = m.empty()
env_config = AppConfig(port=8080, host="localhost", debug=False, features=("auth",))
cli_config = AppConfig(port=3000, host="0.0.0.0", debug=True, features=("logging",))

final = concat_all(m, [defaults, env_config, cli_config])

assert final.port == 8080         # env overrode
assert final.host == "0.0.0.0"    # cli overrode
assert final.debug == True         # any True → True
assert final.features == ("auth", "logging")  # combined

print("Config monoid OK ✅")
```

### Report aggregation = Monoid

Báo cáo doanh số cũng là Monoid: ngày 1 + ngày 2 + ngày 3 = tuần. Tuần 1-4 = tháng. Pattern GIỐNG nhau.

```python
# filename: monoid_reports.py
from dataclasses import dataclass
from functools import reduce

@dataclass(frozen=True)
class SalesReport:
    revenue: int
    order_count: int
    unique_products: frozenset[str]

@dataclass(frozen=True)
class MonoidReport:
    def empty(self) -> SalesReport:
        return SalesReport(revenue=0, order_count=0, unique_products=frozenset())

    def combine(self, a: SalesReport, b: SalesReport) -> SalesReport:
        return SalesReport(
            revenue=a.revenue + b.revenue,
            order_count=a.order_count + b.order_count,
            unique_products=a.unique_products | b.unique_products,  # union
        )

def concat_all(monoid, values: list):
    return reduce(monoid.combine, values, monoid.empty())

daily = [
    SalesReport(5_000_000, 10, frozenset({"Laptop", "Mouse"})),
    SalesReport(3_000_000, 8,  frozenset({"Mouse", "Keyboard"})),
    SalesReport(7_000_000, 15, frozenset({"Monitor"})),
]

weekly = concat_all(MonoidReport(), daily)
assert weekly.revenue == 15_000_000
assert weekly.order_count == 33
assert weekly.unique_products == frozenset({"Laptop", "Mouse", "Keyboard", "Monitor"})

# Empty list = empty report
assert concat_all(MonoidReport(), []) == MonoidReport().empty()

print("Report monoid OK ✅")
```

### Permission union = Monoid

```python
# filename: monoid_permissions.py
from dataclasses import dataclass
from functools import reduce

@dataclass(frozen=True)
class Permissions:
    can_read: bool
    can_write: bool
    can_delete: bool

@dataclass(frozen=True)
class MonoidPermissions:
    def empty(self) -> Permissions:
        return Permissions(can_read=False, can_write=False, can_delete=False)

    def combine(self, a: Permissions, b: Permissions) -> Permissions:
        return Permissions(
            can_read=a.can_read or b.can_read,
            can_write=a.can_write or b.can_write,
            can_delete=a.can_delete or b.can_delete,
        )

def concat_all(monoid, values: list):
    return reduce(monoid.combine, values, monoid.empty())

# User has multiple roles — merge permissions
role_viewer = Permissions(can_read=True, can_write=False, can_delete=False)
role_editor = Permissions(can_read=False, can_write=True, can_delete=False)

effective = concat_all(MonoidPermissions(), [role_viewer, role_editor])
assert effective == Permissions(can_read=True, can_write=True, can_delete=False)

print("Permissions monoid OK ✅")
```

> **💡 "Anywhere you combine things → Monoid"**: merge configs, aggregate reports, combine permissions, union sets, intersect filters. Pattern is UNIVERSAL.

---

## 25.6 — Monoid + Parallelism

### MapReduce = Map + Monoid

Associativity cho phép parallelize. Đây là nền tảng MapReduce:

```python
# filename: monoid_parallel.py
from dataclasses import dataclass
from functools import reduce
from concurrent.futures import ProcessPoolExecutor, ThreadPoolExecutor

@dataclass(frozen=True)
class MonoidSum:
    def combine(self, x: int, y: int) -> int: return x + y
    def empty(self) -> int: return 0

def concat_all(monoid, values: list):
    return reduce(monoid.combine, values, monoid.empty())

# Sequential fold
data = list(range(1, 101))  # 1..100
sequential_result = concat_all(MonoidSum(), data)
assert sequential_result == 5050

# Simulated parallel: split → fold chunks → fold results
def parallel_concat(monoid, values: list, num_chunks: int = 4):
    """Fold in parallel by splitting into chunks."""
    chunk_size = max(1, len(values) // num_chunks)
    chunks = [values[i:i + chunk_size] for i in range(0, len(values), chunk_size)]

    # Each chunk can be processed independently (associativity!)
    chunk_results = [concat_all(monoid, chunk) for chunk in chunks]

    # Fold chunk results
    return concat_all(monoid, chunk_results)

parallel_result = parallel_concat(MonoidSum(), data)
assert parallel_result == 5050  # Same result! Associativity guarantees this

# Real parallelism with ThreadPoolExecutor
def _fold_chunk(args):
    monoid, chunk = args
    return concat_all(monoid, chunk)

def truly_parallel_concat(monoid, values: list, workers: int = 4):
    chunk_size = max(1, len(values) // workers)
    chunks = [values[i:i + chunk_size] for i in range(0, len(values), chunk_size)]

    with ThreadPoolExecutor(max_workers=workers) as executor:
        chunk_results = list(executor.map(
            lambda chunk: concat_all(monoid, chunk),
            chunks,
        ))

    return concat_all(monoid, chunk_results)

tp_result = truly_parallel_concat(MonoidSum(), data)
assert tp_result == 5050

print("Parallel Monoid OK ✅")
```

> **💡 MapReduce = Map + Monoid fold**: Google's MapReduce, Apache Spark `.reduceByKey()`, Python's `multiprocessing.Pool().map()` + `reduce()` — ALL rely on Monoid's associativity for safe parallelization.

---

## ✅ Checkpoint 25.5-25.6

> Đến đây bạn phải hiểu:
> 1. **Config merge** = Monoid (last-wins + tuple/frozenset concat)
> 2. **Report aggregation** = Monoid (sum numbers, union frozensets)
> 3. **Permission union** = Monoid (OR booleans)
> 4. **Associativity enables parallelism**: split, fold chunks, fold results
>
> **Test nhanh**: `concat_all(MonoidPermissions(), [])` = ?
> <details><summary>Đáp án</summary>`Permissions(can_read=False, can_write=False, can_delete=False)` — empty! No roles = no permissions.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Implement `MonoidMin` cho numbers.

```python
# Hint: empty phải là gì để combine(empty, x) == x?
# Với min: min(???, x) == x — ??? phải là giá trị lớn nhất có thể
```

<details><summary>✅ Lời giải Bài 1</summary>

```python
from dataclasses import dataclass
from functools import reduce

@dataclass(frozen=True)
class MonoidMin:
    def combine(self, x: float, y: float) -> float: return min(x, y)
    def empty(self) -> float: return float("inf")  # min(inf, x) == x

def concat_all(m, vs): return reduce(m.combine, vs, m.empty())

assert concat_all(MonoidMin(), [5, 3, 8, 1, 7]) == 1
assert concat_all(MonoidMin(), []) == float("inf")
```

</details>

---

**Bài 2** (10 phút): Monoid cho `Counter` — đếm tần suất từ.

```python
# Counter({"a": 2, "b": 1}).combine(Counter({"a": 1, "c": 3}))
# = Counter({"a": 3, "b": 1, "c": 3})  # merge by summing values
```

<details><summary>✅ Lời giải Bài 2</summary>

```python
from dataclasses import dataclass
from functools import reduce

@dataclass(frozen=True)
class WordCount:
    counts: dict[str, int]

@dataclass(frozen=True)
class MonoidWordCount:
    def empty(self) -> WordCount:
        return WordCount(counts={})

    def combine(self, a: WordCount, b: WordCount) -> WordCount:
        merged = dict(a.counts)
        for key, val in b.counts.items():
            merged[key] = merged.get(key, 0) + val
        return WordCount(counts=merged)

def concat_all(m, vs): return reduce(m.combine, vs, m.empty())

docs = [
    WordCount({"python": 3, "fp": 2}),
    WordCount({"fp": 1, "monad": 4}),
    WordCount({"python": 1}),
]

total = concat_all(MonoidWordCount(), docs)
assert total.counts == {"python": 4, "fp": 3, "monad": 4}
assert concat_all(MonoidWordCount(), []) == MonoidWordCount().empty()
```

</details>

---

**Bài 3** (15 phút): Monoid cho **TimeSeries** — merge time-series data.

```python
# Merge overlapping time series: for same timestamp, keep max value.
# Empty = no data points.
```

<details><summary>✅ Lời giải Bài 3</summary>

```python
from dataclasses import dataclass
from functools import reduce

@dataclass(frozen=True)
class TimeSeries:
    points: dict[str, float]  # timestamp → value

@dataclass(frozen=True)
class MonoidTimeSeries:
    def empty(self) -> TimeSeries:
        return TimeSeries(points={})

    def combine(self, a: TimeSeries, b: TimeSeries) -> TimeSeries:
        merged = dict(a.points)
        for ts, val in b.points.items():
            merged[ts] = max(merged.get(ts, float("-inf")), val)
        return TimeSeries(points=merged)

def concat_all(m, vs): return reduce(m.combine, vs, m.empty())

sensor_1 = TimeSeries({"10:00": 25.0, "10:05": 26.5})
sensor_2 = TimeSeries({"10:00": 24.8, "10:10": 27.0})
sensor_3 = TimeSeries({"10:05": 28.0, "10:15": 23.5})

merged = concat_all(MonoidTimeSeries(), [sensor_1, sensor_2, sensor_3])
assert merged.points["10:00"] == 25.0    # max(25.0, 24.8)
assert merged.points["10:05"] == 28.0    # max(26.5, 28.0)
assert merged.points["10:10"] == 27.0
assert merged.points["10:15"] == 23.5

# Associativity check
way1 = MonoidTimeSeries().combine(
    MonoidTimeSeries().combine(sensor_1, sensor_2), sensor_3
)
way2 = MonoidTimeSeries().combine(
    sensor_1, MonoidTimeSeries().combine(sensor_2, sensor_3)
)
assert way1 == way2
```

</details>

---

## 25.7 — Khi nào dùng Monoid?

Hãy nghĩ về Monoid như một "bộ công cụ ghép". Bất cứ khi nào bạn thấy mình viết `reduce()`, hãy tự hỏi: "Đây có phải Monoid không?"

**Dấu hiệu nhận biết Monoid trong code:**
- `reduce(lambda a, b: merge(a, b), items, empty)` — đây là `concat_all`!
- `{**defaults, **config1, **config2}` — config Monoid!
- `sum += value` trong loop — number addition Monoid!
- `results.extend(partial)` — list concat Monoid!
- `errors += more_errors` — error collection Monoid!

**Python built-in Monoids (bạn đã dùng hàng ngày):**

| Type | `combine` | `empty` |
|------|-----------|---------|
| `int` (addition) | `+` | `0` |
| `int` (multiply) | `*` | `1` |
| `str` | `+` | `""` |
| `list` | `+` | `[]` |
| `tuple` | `+` | `()` |
| `set` (union) | `\|` | `set()` |
| `frozenset` | `\|` | `frozenset()` |
| `dict` (merge) | `\|` | `{}` |
| `bool` (AND) | `and` | `True` |
| `bool` (OR) | `or` | `False` |

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| `combine` không associative | Implementation sai | Test: `combine(combine(a,b),c) == combine(a,combine(b,c))` |
| `empty` không trung tính | Sai giá trị empty | Test: `combine(a, empty) == a` và `combine(empty, a) == a` |
| `concat_all([])` crash | Không handle empty list | Trả `empty()` cho danh sách rỗng |
| Average sai khi merge | Không dùng weighted average | Lưu sum và count riêng, tính avg khi display |
| Non-commutative surprise | `combine(a,b) ≠ combine(b,a)` | Monoid chỉ cần associative, KHÔNG cần commutative |

---

## 💬 Đối thoại với bản thân: Algebra Q&A

**Q: Tại sao cần Eq/Ord Protocol khi Python đã có `==` và `sorted()`?**

A: `==` hardcoded vào `__eq__`. Bạn không thể nói "compare users by ID ở đây, by email ở chỗ khác" mà không thay đổi class. Eq Protocol = inject comparison LOGIC, không phải hardcode. Tương tự, `sorted(key=...)` CHỈ LÀ key function — Ord cho bạn full comparison (`LT/EQ/GT`), `reverse`, `clamp`, `between`, v.v.

**Q: Monoid có giống OOP interface không?**

A: Giống nhưng KHÁC. OOP interface = behavior gắn với object (`obj.combine(other)`). FP Monoid = behavior TÁCH RIÊNG khỏi data (`monoid.combine(a, b)`). Bạn có thể có NHIỀU Monoids cho cùng type: `MonoidSum` và `MonoidProduct` cho `int`. OOP chỉ cho phép MỘT `__add__`.

**Q: Khi nào KHÔNG dùng Monoid?**

A: Khi operation không associative. VD: phép trừ `(3-2)-1 = 0` nhưng `3-(2-1) = 2`. Phép chia, phép lũy thừa cũng vậy. Nếu không associative → không phải Semigroup → không thể parallelize an toàn.

---

## Tóm tắt

Chương này giới thiệu bốn "siêu năng lực" đại số mà FP dùng để abstract patterns lặp đi lặp lại:

- ✅ **Eq** = "cách so sánh bằng". `equals(x, y) -> bool`. Flexible via `eq_by_key`.
- ✅ **Ord** = "cách sắp xếp". `compare(x, y) -> LT|EQ|GT`. `sort_by`, `min`, `max`, `clamp`.
- ✅ **Semigroup** = `combine` (associative). Ghép hai giá trị cùng type.
- ✅ **Monoid** = Semigroup + `empty`. Identity element. `concat_all` = `reduce()` abstracted.
- ✅ **Practical**: merge configs, aggregate reports, combine permissions.
- ✅ **Parallelism**: Associativity → MapReduce, Spark, distributed aggregation.
- ✅ **Universal pattern**: Anywhere you combine things → Monoid.

## Tiếp theo

→ Chapter 26: **Functors** — "mappable containers". `list`, `Optional`, `Result` — tất cả là Functors. `map()` = transform bên trong container mà không cần mở ra.
