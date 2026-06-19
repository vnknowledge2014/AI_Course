# Chapter 28b — Traverse & Sequence

> **Bạn sẽ học được**:
> - Vấn đề "Inside-out": `list[Result]` vs `Result[list]`
> - `sequence`: lật ngược container (flip) — `[Ok(1), Ok(2)] → Ok([1, 2])`
> - `traverse`: `map` kết hợp `sequence` — xử lý mảng và gom kết quả an toàn
> - Fail-fast (Monadic) vs Collect-all (Applicative) trong Traverse
> - Sự thật bất ngờ: `asyncio.gather` chính là `sequence`!
> - Practical: parse list of strings, process multiple files
>
> **Yêu cầu trước**: Chapter 27 (Monads), Chapter 27b (Applicative).
> **Thời gian đọc**: ~30 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Biết cách xử lý danh sách (lists) một cách an toàn mà không phải vất vả xử lý lỗi từng phần tử bằng tay.

---

Giả sử bạn có 10 string chứa IDs: `["1", "2", "3", "4"]`. Bạn muốn chuyển tất cả sang số nguyên.
Bạn dùng `parse_int(s) -> Result[int, str]`.
Bạn chạy vòng lặp hoặc list comprehension: `[parse_int(x) for x in ids]`.
Kết quả bạn nhận được: `[Ok(1), Ok(2), Ok(3), Ok(4)]` — kiểu dữ liệu là `list[Result]`.

Điều này rất khó chịu! Để dùng danh sách này, bạn phải loop qua nó một lần nữa để check từng phần tử xem có `Err` nào không. Nếu có 1 phần tử lỗi (ví dụ `ids = ["1", "abc"]`), bạn muốn TOÀN BỘ quá trình thất bại (Fail-fast) hoặc thu thập tất cả lỗi (Collect-all). Cái bạn THỰC SỰ cần là `Result[list]`: một `Ok([1, 2, 3])` duy nhất, hoặc một `Err` nếu có lỗi.

Biến `list[Result]` thành `Result[list]` gọi là **Sequence**.
Và phép toán map sau đó lập tức sequence gọi là **Traverse**.

---

## Traverse & Sequence — Flipping Containers Inside Out

`sequence` giải quyết vấn đề "lật ngược": nó đưa container bên trong (Result/Optional) ra ngoài, và đẩy container bên ngoài (list) vào trong.

## 28b.1 — Vấn đề `list[Result]`

### Khi list comprehension không đủ

```python
# filename: the_list_result_problem.py
from dataclasses import dataclass
from typing import Union

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def parse_int(s: str) -> Result:
    try: return Ok(int(s))
    except ValueError: return Err(f"Not a number: {s}")

# ── The Problem ──
raw_data = ["42", "100", "7"]
raw_bad = ["42", "abc", "7"]

# Dùng list comprehension thông thường
results_good = [parse_int(x) for x in raw_data]
results_bad = [parse_int(x) for x in raw_bad]

# Kiểu dữ liệu là list[Result]
assert results_good == [Ok(42), Ok(100), Ok(7)]
assert results_bad == [Ok(42), Err("Not a number: abc"), Ok(7)]

# Để tính tổng của mảng kết quả, bạn không thể sum() trực tiếp.
# Bạn phải tự viết logic "nếu tất cả đều Ok thì trả về Ok(sum), nếu có 1 Err thì trả về Err"
def sum_results(results: list[Result]) -> Result:
    total = 0
    for r in results:
        match r:
            case Ok(value=v): total += v
            case Err(): return r # Fail fast
    return Ok(total)

assert sum_results(results_good) == Ok(149)
assert sum_results(results_bad) == Err("Not a number: abc")

print("Vấn đề list[Result] đã được mô phỏng ✅")
```

Hàm `sum_results` phải tự tay giải nén (unwrap) mảng. Rất thủ công! Chúng ta cần một abstraction chung cho việc này.

---

## 28b.2 — `sequence`: Lật ngược cấu trúc

### `list[Result[A]] -> Result[list[A]]`

Hàm `sequence` duyệt qua `list[Result]`. Nếu gặp TẤT CẢ là `Ok`, nó gom giá trị lại thành một mảng và bọc trong `Ok()`. Nếu có BẤT KỲ `Err` nào, nó lập tức short-circuit (fail-fast) và trả về `Err` đó.

```python
# filename: sequence.py
from dataclasses import dataclass
from typing import Union, List, TypeVar

T = TypeVar("T")

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def sequence(results: List[Result]) -> Result:
    """Lật ngược list[Result] thành Result[list] (fail-fast)."""
    values = []
    for r in results:
        match r:
            case Ok(value=v):
                values.append(v)
            case Err():
                return r  # Ngừng ngay lập tức ở lỗi đầu tiên
    return Ok(values)

# ── Tests ──
# 1. Toàn bộ Ok
assert sequence([Ok(1), Ok(2), Ok(3)]) == Ok([1, 2, 3])

# 2. Có 1 Err -> Trả về Err đó
assert sequence([Ok(1), Err("Lỗi ở 2"), Ok(3)]) == Err("Lỗi ở 2")

# 3. List rỗng
assert sequence([]) == Ok([])

# ── Kết hợp với code lúc trước ──
def parse_int(s: str) -> Result:
    try: return Ok(int(s))
    except ValueError: return Err(f"Not a number: {s}")

raw_data = ["10", "20", "30"]
raw_bad = ["10", "x", "30"]

# Thay vì list[Result], giờ ta có Result[list]
good_parsed = sequence([parse_int(x) for x in raw_data])
bad_parsed = sequence([parse_int(x) for x in raw_bad])

assert good_parsed == Ok([10, 20, 30])
assert bad_parsed == Err("Not a number: x")

print("Sequence OK ✅")
```

Với `sequence`, bạn đã chuyển một bài toán xử lý chuỗi lỗi phức tạp về dạng `Result` quen thuộc. Giờ bạn có thể dùng `map`, `bind` lên kết quả `Ok([10, 20, 30])`.

---

## 28b.3 — `traverse`: Bản giao hưởng của `map` và `sequence`

### Đừng map rồi sequence, hãy traverse!

Nhìn vào đoạn code trên: `sequence([parse_int(x) for x in raw_data])`.
Bạn đang:
1. Duyệt qua mảng để biến `x` thành `Result` (bằng list comprehension = `map`).
2. Duyệt qua mảng LẦN NỮA bằng `sequence` để gom kết quả.

Duyệt mảng 2 lần là không cần thiết. Hàm `traverse` làm luôn 2 việc cùng một lúc: vừa apply hàm (`A -> Result[B]`), vừa gom kết quả lại.

```python
# filename: traverse.py
from dataclasses import dataclass
from typing import Union, List, Callable, TypeVar

T = TypeVar("T")
U = TypeVar("U")

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def traverse(items: List[T], fn: Callable[[T], Result]) -> Result:
    """Map items với hàm trả về Result, và gom ngay kết quả."""
    values = []
    for item in items:
        match fn(item):
            case Ok(value=v):
                values.append(v)
            case Err() as e:
                return e  # Fail-fast
    return Ok(values)

# ── Tests ──
def parse_int(s: str) -> Result:
    try: return Ok(int(s))
    except ValueError: return Err(f"Not a number: {s}")

raw_data = ["42", "100", "7"]
raw_bad = ["42", "abc", "7"]

# Chỉ cần 1 lần gọi traverse! Dễ đọc và hiệu quả.
assert traverse(raw_data, parse_int) == Ok([42, 100, 7])
assert traverse(raw_bad, parse_int) == Err("Not a number: abc")

print("Traverse OK ✅")
```

> **💡 Traverse = "Map producing effects"**: Khi hàm của bạn sinh ra effect (như `Result` hoặc `Optional` hay `Future`), đừng dùng `map` thông thường vì nó sẽ trả về mảng effects. Dùng `traverse` để lấy mảng giá trị bên TONG effect!

---

## 28b.4 — Sequence cho Optional (Maybe)

Pattern này không chỉ dùng cho `Result`. Nó là design pattern cho MỌI Monad/Applicative!
Hãy thử với `Optional` (`Type | None`):

```python
# filename: sequence_optional.py
from typing import List, TypeVar

T = TypeVar("T")

def sequence_opt(options: List[T | None]) -> List[T] | None:
    """Lật ngược list[Optional] thành Optional[list]."""
    values = []
    for opt in options:
        if opt is None:
            return None  # Một cái rỗng là TOÀN BỘ rỗng
        values.append(opt)
    return values

def traverse_opt(items: List[T], fn: Callable[[T], U | None]) -> List[U] | None:
    """Map và gom cho Optional."""
    values = []
    for item in items:
        result = fn(item)
        if result is None:
            return None
        values.append(result)
    return values

# ── Vấn đề: Tìm user theo ID ──
users_db = {1: "Minh", 2: "Lan", 3: "Hải"}

def find_user(uid: int) -> str | None:
    return users_db.get(uid)

# Nếu ta dùng map:
found_users = [find_user(id) for id in [1, 2]]
assert found_users == ["Minh", "Lan"] # list[Optional[str]]

# Cần tất cả hoặc không ai cả (All or nothing):
assert traverse_opt([1, 2], find_user) == ["Minh", "Lan"]
assert traverse_opt([1, 99], find_user) is None  # User 99 không tồn tại -> Fail fast toàn bộ

print("Sequence/Traverse cho Optional OK ✅")
```

---

## 28b.5 — Bất ngờ: `asyncio.gather` chính là `sequence`!

Trong Python, bạn có thể đã dùng `sequence` rất nhiều lần mà không để ý.
`Future` (hay Coroutine/Task) chính là một Monad. Khi bạn có một mảng các promises:
`[fetch(url1), fetch(url2)]` -> Kiểu của nó là `list[Future]`.

Để chờ tất cả và lấy mảng kết quả, bạn dùng `asyncio.gather(*tasks)`. Hàm này trả về `Future[list]`.
Nó chính là hàm `sequence` cho Future Monad!

```python
# filename: async_gather_is_sequence.py
import asyncio

async def fetch_data(id: int) -> str:
    await asyncio.sleep(0.01) # Giả lập network
    return f"Data {id}"

async def main():
    ids = [1, 2, 3]
    
    # Map: tạo ra mảng coroutines (Futures) -> list[Future[str]]
    tasks = [fetch_data(uid) for uid in ids]
    
    # Sequence: asyncio.gather lật ngược list[Future] -> Future[list]
    # Khi await, nó unwrap ra list[str]
    results = await asyncio.gather(*tasks)
    
    assert results == ["Data 1", "Data 2", "Data 3"]
    print("asyncio.gather is sequence! ✅")

# Chạy code
asyncio.run(main())
```

---

## 28b.6 — Monadic Traverse (Fail-fast) vs Applicative Traverse (Collect-all)

Trong Ch27b (Applicative), chúng ta đã học cách gom lỗi (collect-all). Hàm `traverse` ở trên là phiên bản **Monadic** (Fail-fast): gặp lỗi đầu tiên là ngừng.
Bạn có thể viết một hàm `traverse` dùng Applicative logic để **gom toàn bộ lỗi** của mảng.

```python
# filename: applicative_traverse.py
from dataclasses import dataclass
from typing import Union, List, Callable, TypeVar

T = TypeVar("T")
U = TypeVar("U")

@dataclass(frozen=True)
class Valid:
    value: object
@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]
Validated = Union[Valid, Invalid]

def traverse_validated(items: List[T], fn: Callable[[T], Validated]) -> Validated:
    """Applicative Traverse: Collect ALL errors from mapping elements."""
    all_values = []
    all_errors = []
    
    for item in items:
        match fn(item):
            case Valid(value=v):
                all_values.append(v)
            case Invalid(errors=e):
                all_errors.extend(e) # Không return ngay, lưu lỗi lại!
                
    if all_errors:
        return Invalid(tuple(all_errors))
    return Valid(all_values)

# ── Ứng dụng: Validate mảng dữ liệu ──
def check_positive(n_str: str) -> Validated:
    try:
        n = int(n_str)
        return Valid(n) if n > 0 else Invalid((f"{n} not positive",))
    except:
        return Invalid((f"'{n_str}' not a number",))

raw_input = ["10", "-5", "abc", "20"]

# Gom TẤT CẢ lỗi trong mảng!
result = traverse_validated(raw_input, check_positive)

assert isinstance(result, Invalid)
assert result.errors == ("-5 not positive", "'abc' not a number")

print("Applicative Traverse OK ✅")
```

Tính chất này cực kỳ hữu ích khi bạn import file CSV hàng nghìn dòng. Bạn không muốn báo lỗi ở dòng 5 rồi bắt người dùng upload lại, rồi lại báo lỗi ở dòng 10. Bạn muốn dùng `traverse_validated` để chạy hết và báo "Lỗi ở các dòng 5, 10, 42".

---

## ✅ Checkpoint 28b.1-28b.6

> Đến đây bạn phải hiểu:
> 1. **`list[Result]` problem**: Cần giải nén để xài, khó code.
> 2. **`sequence`**: lật ngược container (flip). `[Ok(1), Ok(2)] -> Ok([1, 2])`.
> 3. **`traverse`**: Map rồi sequence trong 1 bước.
> 4. **Applicative traverse**: Giữ nguyên danh sách các lỗi thu thập được.
> 5. **`asyncio.gather`**: Chính là sequence áp dụng cho Future.
>
> **Test nhanh**: Nếu hàm trả về `Result` (có thể Fail), để xử lý một mảng ta nên dùng list comprehension hay `traverse`?
> <details><summary>Đáp án</summary>Dùng `traverse`. Nếu dùng list comprehension ta sẽ có `list[Result]`, nếu dùng `traverse` ta sẽ nhận về một `Result[list]` dễ dàng map và bind tiếp.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Dùng Traverse để đọc file an toàn

```python
# Cho hàm read_file(path: str) -> Result[str] (mô phỏng)
# Viết code để đọc mảng paths = ["a.txt", "b.txt", "c.txt"] thành một đoạn string gộp, hoặc trả lỗi nếu có bất kỳ file nào đọc lỗi.

from dataclasses import dataclass
from typing import Union

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def read_file(path: str) -> Result:
    # Mô phỏng: chỉ b.txt bị lỗi
    if path == "b.txt": return Err("File not found")
    return Ok(f"Content of {path}")

# HINT: Dùng hàm traverse ở trên, sau đó map kết quả Ok với "".join()
```

<details><summary>✅ Lời giải Bài 1</summary>

```python
def traverse(items, fn):
    values = []
    for item in items:
        match fn(item):
            case Ok(value=v): values.append(v)
            case Err() as e: return e
    return Ok(values)

paths = ["a.txt", "c.txt"]
result1 = traverse(paths, read_file)
# Nối kết quả
if isinstance(result1, Ok):
    result1 = Ok(", ".join(result1.value))
assert result1 == Ok("Content of a.txt, Content of c.txt")

paths_bad = ["a.txt", "b.txt", "c.txt"]
result2 = traverse(paths_bad, read_file)
assert result2 == Err("File not found")
```

</details>

---

**Bài 2** (10 phút): Tự viết hàm sequence từ traverse

```python
# Mọi sequence đều có thể được viết bằng traverse. Làm thế nào?
# Hãy định nghĩa hàm sequence(results) BẰNG CÁCH GỌI traverse(results, ...)
```

<details><summary>✅ Lời giải Bài 2</summary>

```python
# Traverse cần 1 hàm fn(item) trả về Result.
# Ở đây item ĐÃ LÀ Result rồi! Nên hàm truyền vào traverse chỉ đơn giản là identity.
def sequence_from_traverse(results):
    return traverse(results, lambda x: x)

# Test:
assert sequence_from_traverse([Ok(1), Ok(2)]) == Ok([1, 2])
assert sequence_from_traverse([Ok(1), Err("x")]) == Err("x")
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Bị lồng mảng (Nested) | Dùng map thay vì traverse | Thay vì `[fn(x) for x in l]`, đổi thành `traverse(l, fn)` |
| Traverse không bắt đủ lỗi | Đang dùng Monadic Traverse (Result) | Chuyển sang Applicative Traverse (`Validated`) để collect-all lỗi |
| Kiểu dữ liệu không rõ ràng | Type hints không đầy đủ | Dùng `TypeVar` (T, U) cẩn thận cho hàm traverse |
| Code dường như đồng bộ chậm chạp | Đang traverse IO tasks tuần tự | Với IO, dùng `asyncio.gather` (Sequence cho coroutines) thay cho vòng lặp tuần tự. |

---

## Tóm tắt

- ✅ **`sequence`**: Hàm quyền lực lật ngược hai lớp container: `list[F[A]] -> F[list[A]]`.
- ✅ **`traverse`**: Kết hợp hoàn hảo giữa `map` và `sequence`.
- ✅ **"All-or-Nothing"**: Nếu có 1 Result thất bại, toàn bộ traverse thất bại và trả về Err.
- ✅ **Applicative Traverse**: Phiên bản gom TẤT CẢ lỗi thay vì short-circuit. Cực mạnh khi validate data batch lớn.
- ✅ **Everyday Sequence**: `asyncio.gather()` chính là sequence cho thế giới bất đồng bộ (Future Monad).

## Tiếp theo

→ Chapter 28c: **Recursive Types & Folds** — làm thế nào để FP xử lý các cấu trúc đệ quy (như Tree, AST) mà không dùng vòng lặp, thay vào đó dùng Catamorphisms.
