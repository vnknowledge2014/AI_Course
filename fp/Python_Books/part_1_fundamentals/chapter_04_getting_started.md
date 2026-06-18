# Chapter 4 — Getting Started with Python

> **Bạn sẽ học được**:
> - Setup Python 3.12+ với `pyenv`
> - Project management: `uv` (modern) hoặc `poetry`
> - VS Code + Pylance — IDE setup tối ưu
> - Type hints — viết Python như ngôn ngữ typed
> - Development workflow chuẩn
>
> **Yêu cầu trước**: Chapter 0-3 (CS Foundations)
> **Thời gian đọc**: ~25 phút | **Level**: Beginner
> **Kết quả cuối cùng**: Bạn có môi trường phát triển Python sẵn sàng cho cuốn sách.

---

## Tại sao cần setup đúng?

Python dễ bắt đầu: `python3 hello.py` — done. Nhưng production Python cần: đúng version (3.12+), quản lý dependencies (không conflict), type checking (`mypy`), và IDE hỗ trợ tốt. Setup đúng từ đầu tiết kiệm hàng giờ debug sau này.

---

## 4.1 — Python Version Management: `pyenv`

```bash
# Cài pyenv (macOS)
brew install pyenv

# Hoặc Linux
curl https://pyenv.run | bash

# Cài Python 3.12
pyenv install 3.12.7
pyenv global 3.12.7

# Kiểm tra
python --version
# Output: Python 3.12.7
```

> **💡 Tại sao 3.12+?** Vì `match/case` (3.10), better error messages (3.11), performance gains (3.12), `type` statement (3.12). Cuốn sách này dùng features của 3.10+ trở lên.

---

## 4.2 — Project Management: `uv`

`uv` là package manager mới (by Astral, team tạo `ruff`) — nhanh hơn `pip` 10-100 lần:

```bash
# Cài uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# Tạo project mới
uv init cafe_order
cd cafe_order

# Cấu trúc project
# cafe_order/
# ├── pyproject.toml      ← "hộ khẩu" project
# ├── src/
# │   └── cafe_order/
# │       └── __init__.py
# ├── tests/
# │   └── __init__.py
# └── README.md
```

### `pyproject.toml` — File quan trọng nhất

```toml
[project]
name = "cafe-order"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = []

[tool.mypy]
strict = true
python_version = "3.12"

[tool.pytest.ini_options]
testpaths = ["tests"]
```

### Quản lý dependencies

```bash
# Thêm dependency
uv add pydantic
uv add pyrsistent
uv add returns

# Thêm dev dependency
uv add --dev pytest
uv add --dev mypy
uv add --dev ruff

# Chạy
uv run python src/cafe_order/main.py
uv run pytest
uv run mypy src/
```

---

## 4.3 — IDE Setup: VS Code + Pylance

```json
// .vscode/settings.json
{
    "python.analysis.typeCheckingMode": "strict",
    "python.analysis.diagnosticSeverityOverrides": {
        "reportMissingTypeStubs": "none"
    },
    "editor.formatOnSave": true,
    "[python]": {
        "editor.defaultFormatter": "charliermarsh.ruff"
    }
}
```

Extensions khuyên dùng:
- **Pylance** — type checking real-time
- **Ruff** — linting + formatting (thay `black` + `flake8`)
- **Python Test Explorer** — chạy tests trong IDE

---

## 4.4 — Type Hints: Python có types!

```python
# filename: src/cafe_order/main.py

# === Basic type hints ===
name: str = "Python"
age: int = 33
price: float = 35_000.0
is_active: bool = True

# Function with type hints
def greet(name: str) -> str:
    return f"Hello, {name}!"

assert greet("World") == "Hello, World!"

# === Collections ===
names: list[str] = ["An", "Binh", "Cuong"]
scores: dict[str, int] = {"An": 95, "Binh": 87}
unique_ids: set[int] = {1, 2, 3}
coords: tuple[float, float] = (3.0, 4.0)

# === Optional ===
from typing import Optional

def find_user(user_id: int) -> Optional[str]:
    """Trả về tên user hoặc None nếu không tìm thấy."""
    users = {1: "An", 2: "Binh"}
    return users.get(user_id)

assert find_user(1) == "An"
assert find_user(999) is None

# Python 3.10+ syntax:
def find_user_v2(user_id: int) -> str | None:
    users = {1: "An", 2: "Binh"}
    return users.get(user_id)

print(f"greet = {greet('World')}")
print(f"find_user(1) = {find_user(1)}")
# Output:
# greet = Hello, World!
# find_user(1) = An
```

### Tại sao type hints quan trọng?

```python
# ❌ Không có types — function signature không nói gì
def process(data, config):
    ...
# data là gì? list? dict? string?
# config nhận format nào?
# Trả về gì?

# ✅ Có types — đọc signature = hiểu function
from dataclasses import dataclass

@dataclass(frozen=True)
class Config:
    debug: bool
    max_retries: int

def process(data: list[str], config: Config) -> dict[str, int]:
    return {item: len(item) for item in data}

result = process(["hello", "world"], Config(debug=False, max_retries=3))
assert result == {"hello": 5, "world": 5}
```

---

## 4.5 — Development Workflow

```
1. uv init my_project          ← tạo project
2. Viết code trong src/
3. uv run mypy src/             ← type check
4. Sửa type errors
5. uv run python src/.../main.py ← chạy thử
6. uv run pytest                ← chạy tests
7. uv run ruff check src/       ← lint
8. uv run ruff format src/      ← format
9. Lặp lại từ bước 2
```

### Hello World với tests

```python
# filename: src/cafe_order/main.py

from dataclasses import dataclass
from enum import Enum

class DrinkType(Enum):
    COFFEE = "coffee"
    TEA = "tea"
    SMOOTHIE = "smoothie"

class Size(Enum):
    SMALL = "small"
    MEDIUM = "medium"
    LARGE = "large"

@dataclass(frozen=True)
class Order:
    drink: DrinkType
    size: Size
    customer: str

def price(order: Order) -> int:
    base = {
        DrinkType.COFFEE: 35_000,
        DrinkType.TEA: 25_000,
        DrinkType.SMOOTHIE: 45_000,
    }[order.drink]

    markup = {
        Size.SMALL: 0,
        Size.MEDIUM: 5_000,
        Size.LARGE: 10_000,
    }[order.size]

    return base + markup

def receipt(order: Order) -> str:
    return f"🧾 {order.customer}: {order.size.value} {order.drink.value} — {price(order)}đ"

if __name__ == "__main__":
    orders = [
        Order(DrinkType.COFFEE, Size.MEDIUM, "Minh"),
        Order(DrinkType.SMOOTHIE, Size.LARGE, "Lan"),
        Order(DrinkType.TEA, Size.SMALL, "Hùng"),
    ]

    print("☕ Cafe Order System\n")
    total = 0
    for order in orders:
        print(receipt(order))
        total += price(order)
    print(f"\n💰 Total: {total}đ")
```

```python
# filename: tests/test_order.py

from cafe_order.main import Order, DrinkType, Size, price

def test_coffee_small():
    order = Order(DrinkType.COFFEE, Size.SMALL, "Test")
    assert price(order) == 35_000

def test_smoothie_large():
    order = Order(DrinkType.SMOOTHIE, Size.LARGE, "Test")
    assert price(order) == 55_000  # 45_000 + 10_000

def test_tea_medium():
    order = Order(DrinkType.TEA, Size.MEDIUM, "Test")
    assert price(order) == 30_000  # 25_000 + 5_000
```

---

## ✅ Checkpoint 4

> 1. `pyenv` = version manager, `uv` = package + project manager
> 2. `pyproject.toml` = config file duy nhất (thay `setup.py`, `requirements.txt`)
> 3. `mypy --strict` = type checker nghiêm ngặt
> 4. Development loop: type check → run → test → lint → format

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `command not found: uv` | Chưa cài uv | `curl -LsSf https://astral.sh/uv/install.sh \| sh` |
| `mypy: command not found` | Chưa cài mypy | `uv add --dev mypy` |
| Pylance báo đỏ nhưng code chạy | Type annotations chưa khớp | Fix types hoặc add `# type: ignore` |
| `ModuleNotFoundError` | Package chưa install | `uv add <package>` |

---

## Tóm tắt

- ✅ **Setup**: `pyenv` (versions) + `uv` (packages + project). `pyproject.toml` = single config file.
- ✅ **Type hints**: `str`, `int`, `list[str]`, `dict[str, int]`, `str | None`. Luôn dùng.
- ✅ **IDE**: VS Code + Pylance + Ruff. `typeCheckingMode: strict`.
- ✅ **Workflow**: mypy → run → pytest → ruff.
- ✅ **frozen dataclass + Enum** = type-safe domain modeling từ day one.

## Tiếp theo

→ Chapter 5: **Values, Types & Type Hints** — deep dive vào type system: `Final`, `Literal`, `TypeAlias`, `NewType`, `mypy --strict`.
