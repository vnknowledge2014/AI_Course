# Chapter 3 — Functional Data Structures

> **Bạn sẽ học được**:
> - Immutable vs Mutable — tại sao immutability quan trọng
> - Python immutable types: `tuple`, `frozenset`, `MappingProxyType`
> - `pyrsistent`: persistent data structures cho Python
> - Structural sharing — update hiệu quả mà không copy
> - Graph as ADT
>
> **Yêu cầu trước**: Chapter 2 (Algorithmic Thinking)
> **Thời gian đọc**: ~35 phút | **Level**: Pre-requisite
> **Kết quả cuối cùng**: Hiểu khi nào dùng mutable, khi nào immutable, và TẠI SAO.

---

## Tại sao cần Immutable Data Structures?

Bạn đang viết code với team 5 người. Function A sửa list, function B cũng sửa list đó. Ai sửa trước? Race condition. Bug khó tìm. Code review khó hiểu.

**Immutable data** = data không thể bị sửa sau khi tạo. Muốn "sửa"? Tạo bản copy mới. Nghe lãng phí? Không — vì **structural sharing** cho phép tái sử dụng phần không thay đổi.

---

## 3.1 — Python Built-in Immutables

### Tuple — Immutable list

```python
# Tuple: immutable sequence
coords = (3, 4)
# coords[0] = 5  ← TypeError: 'tuple' does not support item assignment

# "Sửa" = tạo tuple mới
new_coords = (coords[0], 10)
assert coords == (3, 4)      # Gốc không đổi
assert new_coords == (3, 10)  # Bản mới

# Tuple operations
names = ("An", "Binh", "Cuong")
assert names[0] == "An"
assert len(names) == 3
assert "An" in names

# Unpacking
x, y = coords
assert x == 3 and y == 4

print(f"coords = {coords}, new_coords = {new_coords}")
# Output: coords = (3, 4), new_coords = (3, 10)
```

### Frozenset — Immutable set

```python
# frozenset: immutable set — có thể dùng làm dict key
tags = frozenset({"python", "fp", "ddd"})
# tags.add("new")  ← AttributeError: 'frozenset' has no attribute 'add'

# Operations vẫn hoạt động — trả về frozenset MỚI
more_tags = tags | frozenset({"testing"})
assert "testing" in more_tags
assert "testing" not in tags  # Gốc không đổi

# Có thể dùng làm dict key (vì hashable)
tag_counts: dict[frozenset[str], int] = {
    frozenset({"python", "web"}): 42,
    frozenset({"rust", "systems"}): 18,
}
assert tag_counts[frozenset({"python", "web"})] == 42

print(f"tags = {tags}")
print(f"more_tags = {more_tags}")
```

### Frozen Dataclasses

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Point:
    x: float
    y: float

p = Point(x=3.0, y=4.0)
# p.x = 5.0  ← FrozenInstanceError!

# "Sửa" bằng replace (Python 3.13+) hoặc manual
from dataclasses import replace
p2 = replace(p, x=5.0)
assert p == Point(3.0, 4.0)   # Gốc không đổi
assert p2 == Point(5.0, 4.0)  # Bản mới

# Hashable → dùng làm dict key hoặc set member
points = {Point(0, 0), Point(1, 1), Point(0, 0)}
assert len(points) == 2  # Duplicates bị loại

print(f"p = {p}, p2 = {p2}")
# Output: p = Point(x=3.0, y=4.0), p2 = Point(x=5.0, y=4.0)
```

### MappingProxyType — Read-only dict

```python
from types import MappingProxyType

config_mutable = {"debug": True, "port": 8080}
config = MappingProxyType(config_mutable)

assert config["debug"] == True
# config["debug"] = False  ← TypeError: 'mappingproxy' does not support item assignment

# Tuy nhiên: nếu sửa dict gốc, proxy cũng thay đổi!
# → Giải pháp: không giữ reference tới dict gốc
def make_config(data: dict) -> MappingProxyType:
    return MappingProxyType(dict(data))  # copy trước

safe_config = make_config({"debug": True})
assert safe_config["debug"] == True

print(f"config = {dict(config)}")
```

> **💡 Vấn đề Python**: Python không enforce immutability triệt để. `frozen=True` chỉ ngăn reassign fields — nếu field là `list`, bạn vẫn sửa được list đó! Giải pháp: dùng `tuple` thay `list` bên trong frozen dataclass.

---

## 3.2 — Pyrsistent: Persistent Data Structures

### Vấn đề với copy

```python
# Naive immutable update: copy toàn bộ → O(n)
original = list(range(1000))
updated = original.copy()
updated[500] = -1
# → Copy 1000 elements chỉ để sửa 1! Lãng phí.
```

### Structural sharing

**Persistent data structures** giải quyết vấn đề này: khi "update", chúng CHỈ tạo lại phần thay đổi, tái sử dụng phần còn lại.

```python
# pip install pyrsistent
from pyrsistent import pvector, pmap, pset, freeze, thaw

# PVector: persistent vector (immutable list)
v = pvector([1, 2, 3, 4, 5])
v2 = v.set(2, 99)  # "Sửa" index 2

assert v[2] == 3    # Gốc không đổi
assert v2[2] == 99  # Bản mới
assert v is not v2

# Append
v3 = v.append(6)
assert len(v) == 5   # Gốc: 5 phần tử
assert len(v3) == 6   # Mới: 6 phần tử

print(f"v = {v}")
print(f"v2 = {v2}")
print(f"v3 = {v3}")
# Output:
# v = pvector([1, 2, 3, 4, 5])
# v2 = pvector([1, 2, 99, 4, 5])
# v3 = pvector([1, 2, 3, 4, 5, 6])
```

### PMap: Persistent dict

```python
from pyrsistent import pmap

config = pmap({"host": "localhost", "port": 8080, "debug": True})
config2 = config.set("port", 3000)

assert config["port"] == 8080  # Gốc không đổi
assert config2["port"] == 3000

# Remove
config3 = config.remove("debug")
assert "debug" not in config3
assert "debug" in config  # Gốc vẫn có

# Merge
extra = pmap({"ssl": True, "workers": 4})
merged = config.update(extra)
assert merged["ssl"] == True
assert merged["port"] == 8080

print(f"config = {config}")
print(f"merged = {merged}")
```

### Complexity

```python
# Pyrsistent dùng HAMT (Hash Array Mapped Trie):
# - Lookup:  O(log₃₂ n) ≈ O(1) cho n < 1 tỷ
# - Update:  O(log₃₂ n) ≈ O(1)
# - Append:  O(log₃₂ n) ≈ O(1) amortized

# So sánh với naive copy:
# - Copy + update: O(n)  ← chậm với data lớn
# - Persistent:    O(log₃₂ n) ≈ O(1) ← nhanh bất kể size

print("Persistent DS complexity: O(log₃₂ n) ≈ O(1) ✅")
```

---

## 3.3 — Graph as ADT

```python
# Graph: adjacency list dùng dict
Graph = dict[str, set[str]]

def make_graph() -> Graph:
    return {
        "A": {"B", "C"},
        "B": {"A", "D"},
        "C": {"A"},
        "D": {"B"},
    }

def add_edge(graph: Graph, src: str, dst: str) -> Graph:
    """Thêm edge — trả về graph MỚI (immutable style)."""
    new_graph = {k: set(v) for k, v in graph.items()}  # deep copy
    new_graph.setdefault(src, set()).add(dst)
    new_graph.setdefault(dst, set()).add(src)
    return new_graph

def neighbors(graph: Graph, node: str) -> set[str]:
    return graph.get(node, set())

def bfs(graph: Graph, start: str) -> list[str]:
    """Breadth-First Search — duyệt theo từng tầng."""
    visited: list[str] = []
    queue = [start]
    seen = {start}
    while queue:
        node = queue.pop(0)
        visited.append(node)
        for neighbor in sorted(graph.get(node, set())):
            if neighbor not in seen:
                seen.add(neighbor)
                queue.append(neighbor)
    return visited

g = make_graph()
assert neighbors(g, "A") == {"B", "C"}
assert bfs(g, "A") == ["A", "B", "C", "D"]

g2 = add_edge(g, "C", "D")
assert "D" in neighbors(g2, "C")
assert "D" not in neighbors(g, "C")  # Gốc không đổi

print(f"BFS from A: {bfs(g, 'A')}")
# Output: BFS from A: ['A', 'B', 'C', 'D']
```

---

## ✅ Checkpoint 3

> Ghi nhớ:
> 1. `tuple`, `frozenset`, `frozen=True` = Python immutables
> 2. `pyrsistent` = persistent data structures (O(log₃₂ n) update)
> 3. Immutable ≠ chậm — structural sharing tái sử dụng phần không đổi
> 4. Immutable code dễ test, dễ reason, không race conditions

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Tạo frozen dataclass `Color` với `r`, `g`, `b` (int). Thử sửa field → xác nhận lỗi.

<details><summary>✅ Lời giải</summary>

```python
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class Color:
    r: int
    g: int
    b: int

red = Color(255, 0, 0)
# red.r = 128  ← FrozenInstanceError
blue = replace(red, r=0, b=255)
assert blue == Color(0, 0, 255)
```

</details>

**Bài 2** (10 phút): Dùng `pyrsistent.pmap` tạo shopping cart immutable. Implement `add_item`, `remove_item`, `total`.

<details><summary>✅ Lời giải</summary>

```python
from pyrsistent import pmap

Cart = type(pmap())  # PMap type

def add_item(cart, item: str, price: float):
    return cart.set(item, price)

def remove_item(cart, item: str):
    return cart.remove(item) if item in cart else cart

def total(cart) -> float:
    return sum(cart.values())

cart = pmap()
cart = add_item(cart, "Coffee", 35_000)
cart = add_item(cart, "Tea", 25_000)
assert total(cart) == 60_000
cart = remove_item(cart, "Tea")
assert total(cart) == 35_000
```

</details>

---

## 🔧 Troubleshooting

| Lỗi | Nguyên nhân | Cách sửa |
|-----|-------------|----------|
| `FrozenInstanceError` | Cố sửa frozen dataclass | Dùng `dataclasses.replace()` |
| `ModuleNotFoundError: pyrsistent` | Chưa install | `pip install pyrsistent` |
| Frozen dataclass có mutable field | `list` bên trong vẫn sửa được | Dùng `tuple` thay `list` |

---

## Tóm tắt

- ✅ **Immutable types**: `tuple`, `frozenset`, `MappingProxyType`, `dataclass(frozen=True)`.
- ✅ **Pyrsistent**: `pvector`, `pmap`, `pset` — persistent DS với structural sharing.
- ✅ **Complexity**: Persistent update O(log₃₂ n) ≈ O(1) vs naive copy O(n).
- ✅ **Graph**: `dict[str, set[str]]` + BFS/DFS.
- ✅ **Principle**: Immutable data = predictable, testable, concurrent-safe.

## Tiếp theo

→ Chapter 4: **Getting Started with Python** — Setup Python 3.12+, `pyenv`, `uv`/`poetry`, VS Code + Pylance, type hints.
