# Phụ lục C — `returns` vs `fp-ts` vs Rust `Result`

> **Mục tiêu**: so sánh ba hệ sinh thái Result/Either để bạn đọc được code của
> cả ba, và biết cái nào mạnh ở đâu.
> **Liên quan**: Chapter 22 (ROP), Chapter 27 (Monads), Chapter 27b (Applicative)

---

## C.1 — Cùng một bài toán, ba ngôn ngữ

Bài toán: nhận chuỗi thô → parse số → kiểm tra dương → nhân đôi. Fail ở bất kỳ
bước nào thì dừng và giữ lại lỗi.

```python
# ── Python + returns ────────────────────────────────────────────────────────
from returns.result import Result, Success, Failure
from returns.pipeline import flow
from returns.pointfree import bind, map_


def parse(raw: str) -> Result[int, str]:
    try:
        return Success(int(raw))
    except ValueError:
        return Failure(f"không phải số: {raw!r}")


def ensure_positive(n: int) -> Result[int, str]:
    return Success(n) if n > 0 else Failure(f"phải > 0, nhận {n}")


def double(n: int) -> int:
    return n * 2


def run(raw: str) -> Result[int, str]:
    return flow(raw, parse, bind(ensure_positive), map_(double))


assert run("21") == Success(42)
assert run("-1") == Failure("phải > 0, nhận -1")
assert run("abc") == Failure("không phải số: 'abc'")
```

```typescript
// ── TypeScript + fp-ts ──────────────────────────────────────────────────────
import { pipe } from "fp-ts/function";
import * as E from "fp-ts/Either";

const parse = (raw: string): E.Either<string, number> => {
  const n = Number(raw);
  return Number.isNaN(n) ? E.left(`không phải số: ${raw}`) : E.right(n);
};

const ensurePositive = (n: number): E.Either<string, number> =>
  n > 0 ? E.right(n) : E.left(`phải > 0, nhận ${n}`);

const run = (raw: string) =>
  pipe(raw, parse, E.chain(ensurePositive), E.map((n) => n * 2));
```

```rust
// ── Rust ────────────────────────────────────────────────────────────────────
fn parse(raw: &str) -> Result<i32, String> {
    raw.parse().map_err(|_| format!("không phải số: {raw}"))
}

fn ensure_positive(n: i32) -> Result<i32, String> {
    if n > 0 { Ok(n) } else { Err(format!("phải > 0, nhận {n}")) }
}

fn run(raw: &str) -> Result<i32, String> {
    Ok(ensure_positive(parse(raw)?)? * 2)   // `?` = bind, có cú pháp riêng
}
```

---

## C.2 — Bảng đối chiếu API

| Thao tác | Rust `Result` | fp-ts `Either` | Python `returns` |
|---|---|---|---|
| Thành công | `Ok(x)` | `E.right(x)` | `Success(x)` |
| Thất bại | `Err(e)` | `E.left(e)` | `Failure(e)` |
| map giá trị | `.map(f)` | `E.map(f)` | `.map(f)` |
| bind / flatMap | `.and_then(f)` | `E.chain(f)` | `.bind(f)` |
| map lỗi | `.map_err(f)` | `E.mapLeft(f)` | `.alt(f)` |
| Lấy hoặc mặc định | `.unwrap_or(d)` | `E.getOrElse(() => d)` | `.value_or(d)` |
| Lấy hoặc nổ | `.unwrap()` | — | `.unwrap()` |
| Kiểm tra thành công | `.is_ok()` | `E.isRight(x)` | `is_successful(x)` |
| Gộp hai nhánh | `match` | `E.fold(onL, onR)` | `match` / `.fold()` |
| Bọc exception | — | `E.tryCatch` | `@safe` |
| Đảo `list[R]` → `R[list]` | `.collect::<Result<Vec<_>,_>>()` | `A.sequence(E.Applicative)` | `Fold.collect` |
| Cú pháp tắt | **`?`** | — | — |
| Async Result | — | `TaskEither` | `FutureResult` |
| Reader / DI | — | `ReaderTaskEither` | `RequiresContext` |

> **Thứ tự tham số kiểu khác nhau — đây là bẫy khi chuyển qua lại:**
> Rust và `returns` viết **giá trị trước, lỗi sau**: `Result[int, str]`.
> fp-ts viết **lỗi trước, giá trị sau**: `Either<string, number>`.
> fp-ts làm vậy để `map` chỉ tác động lên tham số cuối, đúng quy ước functor
> của Haskell. Nhớ nhầm chiều là kiểu sẽ sai mà thông báo lỗi rất khó hiểu.

---

## C.3 — Điểm mạnh riêng của từng bên

### Rust: cú pháp `?` — không đối thủ

```rust
fn run(raw: &str) -> Result<i32, String> {
    let n = parse(raw)?;             // tự động return sớm khi Err
    let p = ensure_positive(n)?;
    Ok(p * 2)
}
```

Đây là điểm Rust hơn hẳn hai bên còn lại: code trông như code mệnh lệnh bình
thường, nhưng vẫn là monadic bind. Không có nesting, không có combinator.
Python và TypeScript đều **không có** cú pháp tương đương và phải mô phỏng bằng
generator (xem Chapter 27).

### fp-ts / Effect: hệ sinh thái đầy đủ nhất

fp-ts phủ trọn bộ abstraction: `Either`, `Option`, `Task`, `TaskEither`, `Reader`,
`ReaderTaskEither`, `State`, và các typeclass tương ứng (`Functor`, `Applicative`,
`Monad`, `Traversable`). Effect-TS đi xa hơn với hệ thống hiệu ứng có typed
error channel, dependency injection và structured concurrency.

Cái giá: đường học dốc, và stack trace khi lỗi thì gần như không đọc được.

### `returns`: sát Python nhất

```python
from returns.result import safe
from returns.future import future_safe
from returns.context import RequiresContext


@safe                     # exception -> Failure, hợp với code Python có sẵn
def read_config(path: str) -> str:
    return open(path).read()


@future_safe              # async + Result trong một decorator
async def fetch(url: str) -> dict:
    ...


# Dependency injection kiểu Reader — không cần framework DI
def get_user(uid: str) -> RequiresContext[User, Database]:
    return RequiresContext(lambda db: db.find_user(uid))
```

Điểm mạnh lớn nhất của `returns` là `@safe`: nó **bắc cầu** giữa thế giới
exception của Python có sẵn và thế giới Result, nên bạn áp dụng dần được vào
codebase cũ mà không phải viết lại toàn bộ.

---

## C.4 — Fail-fast vs Collect-all

Cả ba đều mặc định **fail-fast** (dừng ở lỗi đầu tiên). Với form nhiều trường,
bạn thường muốn **thu hết lỗi**.

| | Fail-fast | Collect-all |
|---|---|---|
| Bản chất | Monad (`bind`) | Applicative (`ap`) |
| Rust | `?`, `and_then` | tự viết, hoặc crate `validator` |
| fp-ts | `E.chain` | `E.getApplicativeValidation(semigroup)` |
| `returns` | `.bind()` | `Fold.collect` / `Validated` tự xây |

```python
# returns — thu hết lỗi bằng Validated tự xây (chi tiết ở Chapter 27b)
from dataclasses import dataclass


@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]


@dataclass(frozen=True)
class Valid:
    value: object


def validate_all(**fields: object) -> Valid | Invalid:
    """Áp dụng mọi validator, gom TẤT CẢ lỗi thay vì dừng ở lỗi đầu."""
    errors = tuple(f"{k}: {v}" for k, v in fields.items() if isinstance(v, str))
    return Invalid(errors) if errors else Valid(fields)
```

Lý do sâu xa: `bind` cho phép bước sau **phụ thuộc** vào kết quả bước trước, nên
buộc phải dừng khi bước trước fail. `ap` thì các nhánh **độc lập**, nên chạy hết
được rồi mới gộp lỗi. Đây không phải hạn chế cài đặt mà là hệ quả của định nghĩa
— xem Chapter 26 và 27b.

---

## C.5 — Chọn cái nào?

| Tình huống | Khuyến nghị |
|---|---|
| Codebase Python có sẵn, đầy exception | `returns` với `@safe` — áp dụng dần từng hàm |
| Dự án Python mới, team quen FP | `returns` đầy đủ: `Result` + `RequiresContext` + `FutureResult` |
| Python, team chưa quen FP | Tự xây `Result` bằng frozen dataclass (Chapter 22) — 20 dòng, không dependency, dễ giải thích |
| TypeScript, ưu tiên ổn định | `fp-ts` |
| TypeScript, dự án mới, chấp nhận đầu tư | `Effect` |
| Rust | Dùng luôn `Result` chuẩn — không cần thư viện |

> **Lời khuyên hay bị bỏ qua:** với một team chưa quen FP, **tự xây `Result` 20
> dòng** thường thắng việc kéo cả `returns` về. Bạn được 90% lợi ích, mọi người
> đọc được toàn bộ cài đặt, và không ai phải học `RequiresContextFutureResult`
> trong tuần đầu. Chuyển sang thư viện đầy đủ khi team đã thật sự thấy thiếu.
