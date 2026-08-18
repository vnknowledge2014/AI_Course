# Chapter 20 — Domain Modeling ⭐

> **Bạn sẽ học được**:
> - Value Objects: immutable, equality by value
> - Entities: identity-based equality
> - Aggregates: consistency boundaries
> - Smart constructors with Pydantic
>
> **Yêu cầu trước**: Chapter 19 (Architecture)
> **Thời gian đọc**: ~35 phút | **Level**: Advanced

---

## 20.1 — Value Objects

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Email:
    value: str
    def __post_init__(self):
        if "@" not in self.value:
            raise ValueError(f"Invalid email: {self.value}")

@dataclass(frozen=True)
class Money:
    amount: int
    currency: str = "VND"
    def __post_init__(self):
        if self.amount < 0:
            raise ValueError("Negative amount")
    def add(self, other: "Money") -> "Money":
        assert self.currency == other.currency
        return Money(self.amount + other.amount, self.currency)

# Value Objects: equal by VALUE
assert Money(100, "VND") == Money(100, "VND")
assert Email("a@b.com") == Email("a@b.com")
```

## 20.2 — Entities

```python
from dataclasses import dataclass, replace, field
import uuid

@dataclass(frozen=True)
class OrderId:
    value: str = field(default_factory=lambda: str(uuid.uuid4()))

@dataclass(frozen=True)
class OrderItem:
    product: str
    price: Money
    quantity: int = 1
    @property
    def subtotal(self) -> Money:
        return Money(self.price.amount * self.quantity, self.price.currency)

@dataclass(frozen=True)
class Order:
    id: OrderId
    customer_email: Email
    items: tuple[OrderItem, ...]
    status: str = "draft"

    @property
    def total(self) -> Money:
        from functools import reduce
        return reduce(lambda acc, i: acc.add(i.subtotal), self.items, Money(0))

    def add_item(self, item: OrderItem) -> "Order":
        return replace(self, items=(*self.items, item))

    def confirm(self) -> "Order":
        if self.status != "draft":
            raise ValueError(f"Cannot confirm {self.status} order")
        if not self.items:
            raise ValueError("Cannot confirm empty order")
        return replace(self, status="confirmed")

# Entity: equal by ID
order = Order(
    id=OrderId("ORD-001"),
    customer_email=Email("an@mail.com"),
    items=(OrderItem("Coffee", Money(35_000), 2),),
)
assert order.total == Money(70_000)
confirmed = order.confirm()
assert confirmed.status == "confirmed"
assert order.status == "draft"  # Original unchanged
```

## 20.3 — Aggregates

```python
# Aggregate = consistency boundary
# Order is an aggregate root:
# - OrderItems belong to Order
# - All changes go through Order methods
# - Invariants enforced by Order (e.g., can't confirm empty order)

# Rule: reference other aggregates by ID only
@dataclass(frozen=True)
class ShippingInfo:
    order_id: OrderId  # Reference by ID, not by Order object
    address: str
    tracking: str | None = None
```

---

## ✅ Checkpoint 20

> 1. Value Objects: frozen dataclass, equality by value, validated
> 2. Entities: have identity (ID), equality by ID
> 3. Aggregates: consistency boundary, all changes via root methods
> 4. Reference other aggregates by ID only

---

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Chuyển `email: str` thành một Value Object `Email` có smart constructor trả `Result`. Chứng minh không thể tạo `Email` không hợp lệ.

**Bài 2 (15 phút).** Mô hình hoá vòng đời `Order` bằng enum trạng thái sao cho **không thể** gọi `ship()` trên một đơn `Draft` — ép bằng kiểu, không bằng `if`.

**Bài 3 (20 phút).** Xác định ranh giới Aggregate cho `Order` và `OrderLine`: bất biến nào phải đúng trong **cùng một transaction**? Bất biến đó chính là định nghĩa của aggregate.

<details>
<summary>Gợi ý bài 2</summary>

Cách mạnh nhất trong Python: mỗi trạng thái là một class riêng
(`DraftOrder`, `ConfirmedOrder`, `ShippedOrder`), và `ship()` chỉ tồn tại trên
`ConfirmedOrder`, trả về `ShippedOrder`. Trạng thái sai không còn là lỗi runtime —
nó là lỗi type-check.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Value Object vẫn sửa được | Quên `frozen=True` | `@dataclass(frozen=True, slots=True)` |
| Hai Value Object bằng nhau lại không `==` | Tự viết `__eq__` sai | Để dataclass sinh; đừng viết tay |
| Aggregate quá lớn, khoá quá nhiều | Gộp thứ không cần nhất quán tức thì | Tách aggregate, nối bằng domain event |
| Entity không dùng được làm key dict | `frozen=True` nhưng có field mutable | Dùng `tuple` thay `list` cho field bên trong |

## Tóm tắt

- ✅ **Value Object**: `frozen=True`, validated, equal by value.
- ✅ **Entity**: Has ID, mutable state (via replace), equal by ID.
- ✅ **Aggregate**: Consistency boundary. Root enforces invariants.
- ✅ **Smart constructors**: `__post_init__` validates on creation.

## Tiếp theo

→ Chapter 21: **Workflows & Pipelines** — DDD workflows as function pipelines.
