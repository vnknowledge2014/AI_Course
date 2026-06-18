# Chapter 15 — Protocols — Structural Typing

> **Bạn sẽ học được**:
> - `typing.Protocol` (PEP 544) — structural subtyping
> - Protocol vs ABC — khi nào dùng cái nào
> - Dependency injection via Protocols
> - Runtime protocol checking với `runtime_checkable`
>
> **Yêu cầu trước**: Chapter 14 (Pydantic)
> **Thời gian đọc**: ~25 phút | **Level**: Intermediate

---

## 15.1 — Protocol Basics

```python
from typing import Protocol

# Protocol = "interface" — defines WHAT, not HOW
class Printable(Protocol):
    def to_string(self) -> str: ...

class Logger(Protocol):
    def log(self, message: str) -> None: ...

# ANY class that has to_string() is Printable — NO inheritance needed!
from dataclasses import dataclass

@dataclass(frozen=True)
class User:
    name: str
    email: str

    def to_string(self) -> str:
        return f"User({self.name}, {self.email})"

@dataclass(frozen=True)
class Product:
    title: str
    price: int

    def to_string(self) -> str:
        return f"Product({self.title}, {self.price}đ)"

# Both satisfy Printable — without inheriting from it!
def display(item: Printable) -> str:
    return f"[Display] {item.to_string()}"

assert display(User("An", "an@mail.com")) == "[Display] User(An, an@mail.com)"
assert display(Product("Coffee", 35_000)) == "[Display] Product(Coffee, 35000đ)"
```

## 15.2 — Protocols for DI

```python
from typing import Protocol
from dataclasses import dataclass

# Port (interface)
class UserRepository(Protocol):
    def find_by_id(self, user_id: int) -> dict | None: ...
    def save(self, user: dict) -> None: ...

# Adapter 1: In-memory (for testing)
class InMemoryUserRepo:
    def __init__(self) -> None:
        self._users: dict[int, dict] = {}

    def find_by_id(self, user_id: int) -> dict | None:
        return self._users.get(user_id)

    def save(self, user: dict) -> None:
        self._users[user["id"]] = user

# Adapter 2: SQL (for production)
class SqlUserRepo:
    def find_by_id(self, user_id: int) -> dict | None:
        # In real code: SELECT * FROM users WHERE id = ?
        return None

    def save(self, user: dict) -> None:
        # In real code: INSERT INTO users ...
        pass

# Service depends on Protocol, not implementation
def register_user(repo: UserRepository, name: str) -> dict:
    user = {"id": 1, "name": name}
    repo.save(user)
    return user

# Test with in-memory
repo = InMemoryUserRepo()
user = register_user(repo, "An")
assert repo.find_by_id(1) == {"id": 1, "name": "An"}

# Production would use: register_user(SqlUserRepo(), "An")
```

## 15.3 — Protocol vs ABC

```python
from abc import ABC, abstractmethod
from typing import Protocol

# ABC — nominal subtyping (MUST inherit)
class Animal(ABC):
    @abstractmethod
    def speak(self) -> str: ...

class Dog(Animal):  # Must explicitly inherit
    def speak(self) -> str:
        return "Woof!"

# Protocol — structural subtyping (NO inheritance needed)
class Speaker(Protocol):
    def speak(self) -> str: ...

class Cat:  # No inheritance!
    def speak(self) -> str:
        return "Meow!"

def make_noise(s: Speaker) -> str:
    return s.speak()

assert make_noise(Dog()) == "Woof!"
assert make_noise(Cat()) == "Meow!"  # Cat doesn't inherit Speaker!

# | Feature | ABC | Protocol |
# |---------|-----|----------|
# | Inheritance | Required | Not needed |
# | Checking | Nominal (by name) | Structural (by shape) |
# | Third-party | Must subclass | Just implement methods |
# | FP-friendly | Less | More |
```

---

## ✅ Checkpoint 15

> 1. `Protocol` = structural typing — "if it quacks like a duck..."
> 2. No inheritance needed — any class with right methods satisfies Protocol
> 3. Perfect for DI: define Port as Protocol, swap Adapters freely
> 4. Prefer Protocol over ABC for flexibility

---

## Tóm tắt

- ✅ **Protocol**: Structural subtyping — PEP 544.
- ✅ **DI pattern**: Port (Protocol) + Adapter (implementation).
- ✅ **No inheritance**: Any matching shape satisfies the Protocol.
- ✅ **Testing**: Use in-memory adapter for tests, real adapter for production.

## Tiếp theo

→ Chapter 16: **GoF → FP Translation** — Classical patterns reimagined functionally.
