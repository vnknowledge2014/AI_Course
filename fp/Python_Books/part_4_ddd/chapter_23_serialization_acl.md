# Chapter 23 — Serialization & Anti-Corruption Layer

> **Bạn sẽ học được**:
> - Domain types ↔ DTOs (Pydantic)
> - Anti-Corruption Layer (ACL) pattern
> - JSON, database row, API response → Domain
> - Domain → JSON, database row, API response
>
> **Yêu cầu trước**: Chapter 22 (ROP)
> **Thời gian đọc**: ~25 phút | **Level**: Advanced

---

## 23.1 — DTO vs Domain

```python
from dataclasses import dataclass
from pydantic import BaseModel, Field

# Domain (pure, validated)
@dataclass(frozen=True)
class Email:
    value: str
    def __post_init__(self):
        if "@" not in self.value:
            raise ValueError(f"Invalid: {self.value}")

@dataclass(frozen=True)
class Order:
    id: str
    customer_email: Email
    items: tuple[str, ...]
    total: int

# DTO (serialization boundary)
class OrderDTO(BaseModel):
    id: str
    customer_email: str
    items: list[str]
    total: int

# ACL: DTO → Domain
def dto_to_domain(dto: OrderDTO) -> Order:
    return Order(
        id=dto.id,
        customer_email=Email(dto.customer_email),
        items=tuple(dto.items),
        total=dto.total,
    )

# ACL: Domain → DTO
def domain_to_dto(order: Order) -> OrderDTO:
    return OrderDTO(
        id=order.id,
        customer_email=order.customer_email.value,
        items=list(order.items),
        total=order.total,
    )

# Test roundtrip
dto = OrderDTO(id="ORD-1", customer_email="an@mail.com", items=["Coffee"], total=35_000)
domain = dto_to_domain(dto)
assert domain.customer_email.value == "an@mail.com"

back = domain_to_dto(domain)
assert back == dto
```

## 23.2 — ACL for External APIs

```python
from pydantic import BaseModel

# External API response (their format)
class StripeChargeResponse(BaseModel):
    id: str
    amount: int
    currency: str
    status: str

# Our domain type
@dataclass(frozen=True)
class PaymentResult:
    payment_id: str
    amount_vnd: int
    success: bool

# ACL: translate their world → our world
def stripe_to_domain(response: StripeChargeResponse) -> PaymentResult:
    return PaymentResult(
        payment_id=response.id,
        amount_vnd=response.amount if response.currency == "vnd" else response.amount * 25_000,
        success=response.status == "succeeded",
    )

stripe_data = StripeChargeResponse(id="ch_123", amount=100, currency="usd", status="succeeded")
result = stripe_to_domain(stripe_data)
assert result.success == True
assert result.amount_vnd == 2_500_000
```

---

## Tóm tắt

- ✅ **DTO**: Pydantic model for serialization boundaries.
- ✅ **Domain**: Pure dataclasses with business validation.
- ✅ **ACL**: Translation layer between external and internal models.
- ✅ **Never let external shapes leak into domain**.

## Tiếp theo

→ Chapter 24: **Persistence** — Repository pattern, database access.
