# Chapter 10 — Modules & Packages

> **Bạn sẽ học được**:
> - `import`, `from ... import`, `as`
> - `__init__.py` — package initializer
> - `pyproject.toml` — modern project config
> - Project structure best practices
> - Circular imports và cách tránh
>
> **Yêu cầu trước**: Chapter 9 (Dataclasses)
> **Thời gian đọc**: ~25 phút | **Level**: Beginner

---

## 10.1 — Import System

```python
# === Import styles ===

# Import module
import math
assert math.pi > 3.14

# Import specific names
from math import sqrt, pi
assert sqrt(16) == 4.0

# Import with alias
from collections import defaultdict as dd
groups: dd[str, list[str]] = dd(list)

# Import all (avoid in production!)
# from math import *  ← BAD: pollutes namespace
```

## 10.2 — Package Structure

```
my_project/
├── pyproject.toml
├── src/
│   └── my_project/
│       ├── __init__.py          # package root
│       ├── domain/
│       │   ├── __init__.py
│       │   ├── order.py         # Order, OrderItem
│       │   └── payment.py       # Payment types
│       ├── services/
│       │   ├── __init__.py
│       │   └── order_service.py # Business logic
│       └── api/
│           ├── __init__.py
│           └── routes.py        # HTTP endpoints
└── tests/
    ├── __init__.py
    ├── test_order.py
    └── test_payment.py
```

### `__init__.py` — Public API

```python
# src/my_project/domain/__init__.py

# Re-export public names
from .order import Order, OrderItem, OrderStatus
from .payment import Payment, Cash, Card

# Users import from package, not from files:
# from my_project.domain import Order, Payment
# NOT: from my_project.domain.order import Order
```

### Relative imports

```python
# src/my_project/services/order_service.py

# Relative import (within package)
from ..domain import Order, OrderItem, Payment

# Absolute import
from my_project.domain import Order
```

## 10.3 — `pyproject.toml`

```toml
[project]
name = "my-project"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "pydantic>=2.0",
    "returns>=0.22",
]

[project.optional-dependencies]
dev = [
    "pytest>=8.0",
    "mypy>=1.10",
    "ruff>=0.4",
]

[tool.mypy]
strict = true

[tool.ruff]
target-version = "py312"
line-length = 100

[tool.pytest.ini_options]
testpaths = ["tests"]
```

## 10.4 — Best Practices

```python
# ✅ DO: Import from package (stable API)
from my_project.domain import Order

# ❌ DON'T: Import from internal file (fragile)
from my_project.domain.order import Order

# ✅ DO: Group imports (stdlib → third-party → local)
import os                           # stdlib
from pathlib import Path            # stdlib

from pydantic import BaseModel      # third-party

from my_project.domain import Order # local

# ✅ DO: Use TYPE_CHECKING for circular imports
from __future__ import annotations
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from my_project.domain import Order  # Only for type hints

def process(order_id: int) -> "Order":
    ...
```

---

## ✅ Checkpoint 10

> 1. `__init__.py` = public API, re-export names
> 2. `pyproject.toml` = single config file
> 3. Group imports: stdlib → third-party → local
> 4. Use `TYPE_CHECKING` to break circular imports

---

## Tóm tắt

- ✅ **Imports**: `import`, `from ... import`, relative `..`
- ✅ **Packages**: `__init__.py` defines public API
- ✅ **`pyproject.toml`**: Dependencies, tools, config — all in one
- ✅ **Structure**: `src/` layout, domain/services/api separation
- ✅ **Circular imports**: Use `TYPE_CHECKING` guard

## Tiếp theo

→ Chapter 11: **Immutability & Purity** — Tại sao immutability quan trọng, pure functions, side effects.
