# Phụ lục A — Cấu hình `mypy --strict`

> **Mục tiêu**: bật `--strict` trên một codebase có sẵn mà không bị chôn vùi dưới
> 3.000 lỗi rồi bỏ cuộc sau hai ngày.
> **Liên quan**: Chapter 5 (Type Hints), Chapter 15 (Protocols)

---

## A.1 — `--strict` thực chất là gì

`--strict` không phải một tính năng riêng. Nó là **bí danh bật một lúc 9 flag**.
Biết từng flag làm gì quan trọng hơn nhiều so với việc nhớ chữ `--strict`, vì khi
migration bạn sẽ bật chúng **từng cái một**.

| Flag | Bắt lỗi gì | Độ ồn khi bật trên code cũ |
|---|---|---|
| `warn_unused_configs` | Mục thừa trong file config | Rất thấp |
| `disallow_subclassing_any` | Kế thừa từ class kiểu `Any` | Thấp |
| `disallow_untyped_decorators` | Decorator không có type làm mất type của hàm | Thấp |
| `no_implicit_optional` | `def f(x: int = None)` — ngầm thành `int \| None` | Thấp |
| `warn_redundant_casts` | `cast()` không cần thiết | Thấp |
| `warn_unused_ignores` | `# type: ignore` đã hết tác dụng | Trung bình |
| `check_untyped_defs` | Kiểm tra cả phần thân hàm chưa annotate | **Cao** |
| `disallow_untyped_defs` | Hàm thiếu annotation | **Rất cao** |
| `warn_return_any` | Hàm khai báo trả `T` nhưng thực trả `Any` | Trung bình |

Hai flag `check_untyped_defs` và `disallow_untyped_defs` sinh ra khoảng 90% số
lỗi khi bạn bật `--strict` lần đầu. Chúng cũng là hai flag đáng giá nhất.

---

## A.2 — Cấu hình khởi điểm cho dự án mới

Dự án mới thì bật hết ngay từ commit đầu tiên — rẻ hơn nhiều so với sửa sau.

```toml
# pyproject.toml
[tool.mypy]
python_version = "3.12"
strict = true

# Ngoài --strict: những flag này bắt lỗi thật mà --strict chưa bao gồm
warn_unreachable = true          # code không bao giờ chạy tới — thường là bug logic
strict_equality = true           # so sánh hai kiểu không bao giờ bằng nhau
disallow_any_explicit = false    # bật khi team đã quen; ban đầu để false
enable_error_code = [
  "redundant-expr",              # điều kiện luôn đúng/luôn sai
  "possibly-undefined",          # biến có thể chưa được gán ở nhánh nào đó
  "truthy-bool",                 # `if some_object:` khi object luôn truthy
  "ignore-without-code",         # ép `# type: ignore[mã-cụ-thể]`
]

pretty = true
show_error_codes = true
```

> **`ignore-without-code` là flag bị đánh giá thấp nhất trong danh sách.** Một
> `# type: ignore` trần sẽ nuốt **mọi** lỗi trên dòng đó — kể cả lỗi mới xuất hiện
> sau này vì lý do hoàn toàn khác. `# type: ignore[arg-type]` chỉ nuốt đúng lỗi
> bạn đã cân nhắc.

---

## A.3 — Chiến lược migration cho codebase có sẵn

Đây là phần khiến hầu hết các lần "bật strict" thất bại. Vấn đề không phải kỹ
thuật mà là tâm lý: 3.000 lỗi đỏ khiến người ta bỏ cuộc.

### Bước 1 — Đo trước, đừng sửa vội

```bash
uv add --dev mypy
uv run mypy src/ --strict 2>&1 | tail -1
# Found 2847 errors in 132 files (checked 210 source files)
```

Con số này chỉ để biết quy mô. Đừng cố sửa hết trong một PR.

### Bước 2 — Bật global ở mức thấp, strict theo từng module

Mấu chốt: **`--strict` không phải công tắc toàn cục.** mypy cho phép override
theo module, và đó là cách duy nhất khiến migration khả thi.

```toml
[tool.mypy]
python_version = "3.12"
# Nền chung: nhẹ, để CI xanh ngay từ ngày đầu
warn_unused_configs = true
no_implicit_optional = true
warn_redundant_casts = true
ignore_missing_imports = true

# Domain layer là code thuần, không I/O — dễ type nhất và đáng type nhất.
# Bắt đầu ở đây.
[[tool.mypy.overrides]]
module = "myapp.domain.*"
strict = true

# Mở rộng dần sang application layer khi domain đã xanh
[[tool.mypy.overrides]]
module = "myapp.application.*"
disallow_untyped_defs = true
check_untyped_defs = true

# Thư viện bên thứ ba không có stub — tắt riêng, đừng tắt toàn cục
[[tool.mypy.overrides]]
module = ["untyped_legacy_lib.*", "some_c_extension.*"]
ignore_missing_imports = true

# Test thường dùng nhiều fixture động — nới lỏng có chủ đích
[[tool.mypy.overrides]]
module = "tests.*"
disallow_untyped_defs = false
```

### Bước 3 — Khoá tiến độ bằng CI

```yaml
# .github/workflows/typecheck.yml
- name: Type check
  run: uv run mypy src/
```

Sau mỗi lần một module chuyển sang `strict = true`, CI sẽ **không cho phép**
lùi lại. Đây là điểm khác biệt giữa migration thành công và migration chết yểu:
tiến độ được khoá lại từng bước thay vì trôi ngược.

### Thứ tự ưu tiên module

```
1. domain/          ← pure functions, không I/O. Dễ nhất, giá trị cao nhất.
2. application/     ← orchestration. Type ở đây chặn lỗi lắp ghép sai.
3. infrastructure/  ← đụng thư viện ngoài, nhiều Any. Khó nhất, để cuối.
4. tests/           ← nới lỏng vĩnh viễn cũng được.
```

Thứ tự này khớp đúng với kiến trúc onion ở Chapter 19 — không phải trùng hợp.
Càng vào trong lõi càng thuần, càng thuần thì càng dễ type.

---

## A.4 — Escape hatch và cách dùng có trách nhiệm

Khi thật sự bí, bạn có bốn lối thoát. Xếp theo thứ tự **nên dùng trước**:

```python
from typing import Any, cast, TYPE_CHECKING

# ① TỐT NHẤT — thu hẹp bằng logic, mypy tự hiểu
def process(value: str | int) -> str:
    if isinstance(value, int):
        return str(value)
    return value.upper()          # mypy biết chắc đây là str


# ② CHẤP NHẬN ĐƯỢC — `cast` khi BẠN biết chắc còn mypy thì không
def load_config(raw: dict[str, Any]) -> str:
    # Schema đã được validate ở tầng trên bằng Pydantic
    return cast(str, raw["database_url"])


# ③ DÙNG DÈ — ignore có mã lỗi cụ thể + lý do
result = legacy_api.fetch()  # type: ignore[no-untyped-call]  # lib chưa có stub, xem #1234


# ④ TỆ NHẤT — ignore trần, nuốt mọi lỗi kể cả lỗi tương lai
result = legacy_api.fetch()  # type: ignore   ← ĐỪNG
```

Bật `warn_unused_ignores = true` để mypy tự báo khi một `# type: ignore` đã hết
cần thiết (thường là sau khi thư viện phát hành stub). Nếu không, chúng tích tụ
vĩnh viễn.

### Stub cho thư viện thiếu type

```bash
# Rất nhiều thư viện phổ biến đã có stub sẵn
uv add --dev types-requests types-redis types-PyYAML

# Kiểm tra xem có stub không trước khi tự viết
uv run mypy --install-types
```

---

## A.5 — mypy hay pyright?

Cả hai đều tốt. Khác biệt thực tế:

| | **mypy** | **pyright / Pylance** |
|---|---|---|
| Người làm | Python core team | Microsoft |
| Tốc độ | Chậm hơn (có cache) | Rất nhanh |
| Suy luận kiểu | Bảo thủ hơn | Mạnh hơn, đặc biệt với narrowing |
| Tích hợp IDE | Qua plugin | Gắn sẵn trong VS Code |
| Tính năng mới của PEP | Chậm hơn vài tháng | Thường sớm hơn |

**Khuyến nghị thực dụng:** dùng **Pylance trong IDE** (phản hồi tức thì khi gõ)
và **mypy trong CI** (nguồn phán quyết cuối cùng). Chúng đôi khi bất đồng ở các
ca narrowing khó — khi đó lấy mypy làm chuẩn vì đó là thứ chặn merge.

```json
// .vscode/settings.json
{
  "python.analysis.typeCheckingMode": "strict",
  "python.analysis.inlayHints.functionReturnTypes": true,
  "python.analysis.inlayHints.variableTypes": true
}
```

---

## A.6 — Bảng tra lỗi hay gặp

| Thông báo | Nguyên nhân | Cách sửa |
|---|---|---|
| `Function is missing a return type annotation` | Thiếu `-> T` | Thêm; hàm không trả gì thì `-> None` |
| `Need type annotation for "items"` | `items = []` chưa rõ kiểu | `items: list[str] = []` |
| `Incompatible default for argument` | `def f(x: int = None)` | `x: int \| None = None` |
| `Returning Any from function declared to return "str"` | Nguồn dữ liệu là `Any` | `cast(str, ...)` sau khi đã validate |
| `Cannot determine type of "x"` | Suy luận vòng, thường trong closure | Annotate tường minh |
| `Argument 1 has incompatible type "list[str]"; expected "Sequence[str]"` | Ngược lại mới đúng — `list` *là* `Sequence` | Kiểm tra lại chiều biến (covariance); dùng `Sequence` cho tham số đầu vào |
| `Unused "type: ignore" comment` | Lỗi đã được sửa ở đâu đó | Xoá comment |

> **Về mục áp chót:** dùng `Sequence[str]` cho **tham số** (nhận được cả `list`
> lẫn `tuple`) và `list[str]` cho **giá trị trả về** (caller được toàn quyền).
> Đây là nguyên tắc "nhận rộng, trả hẹp" — nó khiến API của bạn dễ dùng hơn mà
> vẫn chặt chẽ.
