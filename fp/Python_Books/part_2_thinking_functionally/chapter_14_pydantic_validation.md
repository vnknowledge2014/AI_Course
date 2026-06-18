# Chapter 14 — Validation with Pydantic ⭐

> **Bạn sẽ học được**:
> - Pydantic `BaseModel` — auto-validation + serialization
> - `@field_validator` — custom validation rules
> - Smart constructors pattern
> - Domain types vs DTOs
> - `Field()` constraints
>
> **Yêu cầu trước**: Chapter 13 (ADTs)
> **Thời gian đọc**: ~35 phút | **Level**: Intermediate

---

## 14.1 — Pydantic BaseModel

```python
from pydantic import BaseModel, Field, field_validator, ValidationError

class CreateUserRequest(BaseModel):
    name: str = Field(min_length=1, max_length=100)
    email: str
    age: int = Field(ge=0, le=150)

    @field_validator("email")
    @classmethod
    def validate_email(cls, v: str) -> str:
        if "@" not in v:
            raise ValueError("Invalid email format")
        return v.lower().strip()

# Valid
user = CreateUserRequest(name="An", email="An@Mail.COM", age=25)
assert user.email == "an@mail.com"  # normalized!
assert user.age == 25

# Invalid
try:
    CreateUserRequest(name="", email="bad", age=-1)
except ValidationError as e:
    errors = e.errors()
    assert len(errors) >= 2  # name too short, email invalid, age negative
    print(f"Validation errors: {len(errors)}")
```

## 14.2 — Smart Constructors

```python
from pydantic import BaseModel, Field, field_validator

class Email(BaseModel, frozen=True):
    value: str

    @field_validator("value")
    @classmethod
    def validate(cls, v: str) -> str:
        if "@" not in v or "." not in v.split("@")[-1]:
            raise ValueError(f"Invalid email: {v}")
        return v.lower().strip()

class Money(BaseModel, frozen=True):
    amount: int = Field(ge=0)
    currency: str = Field(pattern=r"^[A-Z]{3}$")

# Usage — validation happens automatically
email = Email(value="An@Mail.COM")
assert email.value == "an@mail.com"

money = Money(amount=50_000, currency="VND")
assert money.amount == 50_000

# Invalid → clear error
try:
    Money(amount=-100, currency="VND")
except ValidationError:
    pass  # Expected: amount must be >= 0
```

## 14.3 — Serialization (Domain ↔ JSON)

```python
from pydantic import BaseModel, Field
from datetime import datetime

class OrderDTO(BaseModel):
    id: str
    customer_email: str
    total_amount: int
    currency: str = "VND"
    created_at: datetime

# From dict/JSON → Pydantic model
data = {
    "id": "ORD-001",
    "customer_email": "an@mail.com",
    "total_amount": 120_000,
    "created_at": "2024-01-15T10:30:00",
}
order = OrderDTO.model_validate(data)
assert order.id == "ORD-001"

# From Pydantic model → dict
d = order.model_dump()
assert d["total_amount"] == 120_000

# From Pydantic model → JSON string
json_str = order.model_dump_json()
assert "ORD-001" in json_str

# From JSON string → Pydantic model
order2 = OrderDTO.model_validate_json(json_str)
assert order2 == order
```

## 14.4 — Nested Models

```python
from pydantic import BaseModel, Field

class Address(BaseModel, frozen=True):
    street: str
    city: str
    country: str = "VN"

class Customer(BaseModel, frozen=True):
    name: str = Field(min_length=1)
    email: str
    address: Address

class OrderItem(BaseModel, frozen=True):
    product: str
    quantity: int = Field(ge=1)
    unit_price: int = Field(ge=0)

class Order(BaseModel, frozen=True):
    customer: Customer
    items: list[OrderItem] = Field(min_length=1)

    @property
    def total(self) -> int:
        return sum(item.quantity * item.unit_price for item in self.items)

# Create with nested validation
order = Order(
    customer=Customer(
        name="An",
        email="an@mail.com",
        address=Address(street="123 Main St", city="HCMC"),
    ),
    items=[
        OrderItem(product="Coffee", quantity=2, unit_price=35_000),
        OrderItem(product="Cake", quantity=1, unit_price=50_000),
    ],
)
assert order.total == 120_000
```

---

## ✅ Checkpoint 14

> 1. Pydantic = validation + serialization + type safety
> 2. `@field_validator` = custom validation logic
> 3. Smart constructors: invalid data CANNOT exist
> 4. `model_dump()` / `model_dump_json()` = serialization
> 5. `frozen=True` = immutable Pydantic models

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Tạo `PhoneNumber(value)` Pydantic model validate format +84XXXXXXXXX.

**Bài 2** (15 phút): Tạo `RegistrationForm` với email, password (min 8 chars, must have digit), phone validation.

<details><summary>✅ Lời giải Bài 1</summary>

```python
import re
from pydantic import BaseModel, field_validator

class PhoneNumber(BaseModel, frozen=True):
    value: str

    @field_validator("value")
    @classmethod
    def validate(cls, v: str) -> str:
        if not re.match(r"^\+84\d{9}$", v):
            raise ValueError(f"Invalid VN phone: {v}")
        return v

pn = PhoneNumber(value="+84912345678")
assert pn.value == "+84912345678"
```

</details>

---

## Tóm tắt

- ✅ **Pydantic BaseModel**: Auto-validation on construction.
- ✅ **Smart constructors**: Invalid data cannot exist.
- ✅ **Serialization**: `model_dump()`, `model_dump_json()`, `model_validate()`.
- ✅ **`frozen=True`**: Immutable Pydantic models.
- ✅ **Pydantic = F# Records + Smart Constructors + Serialization** all-in-one.

## Tiếp theo

→ Chapter 15: **Protocols — Structural Typing** — Python's answer to interfaces/traits.
