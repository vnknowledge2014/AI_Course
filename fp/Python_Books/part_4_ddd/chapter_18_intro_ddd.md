# Chapter 18 — Introduction to DDD

> **Bạn sẽ học được**:
> - Domain-Driven Design là gì và tại sao quan trọng
> - Ubiquitous Language — ngôn ngữ chung
> - Bounded Contexts — ranh giới domain
> - Event Storming workflow
>
> **Yêu cầu trước**: Chapter 17 (CQRS)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 18.1 — DDD là gì?

DDD = **Domain-Driven Design** — phương pháp thiết kế phần mềm đặt **business domain** làm trung tâm.

```
Traditional:                    DDD:
Database → Code → UI           Domain Expert → Ubiquitous Language
                                     → Domain Model → Code
```

Core ideas:
1. **Code phản ánh business** — không phải database schema
2. **Domain experts + Developers nói cùng ngôn ngữ** — Ubiquitous Language
3. **Chia hệ thống thành Bounded Contexts** — mỗi context có model riêng

## 18.2 — Ubiquitous Language

```python
# ❌ BAD: Technical terms không ai hiểu
class EntityManager:
    def process_entity(self, entity_dto): ...

# ✅ GOOD: Business terms mọi người đều hiểu
class OrderService:
    def place_order(self, order_request): ...

# Ubiquitous Language cho Cafe domain:
# - Order (đơn hàng), not "transaction" or "entity"
# - Customer (khách hàng), not "user" or "account"
# - Barista (pha chế), not "processor"
# - Menu (thực đơn), not "product catalog"
# - Bill (hóa đơn), not "payment receipt DTO"
```

## 18.3 — Bounded Contexts

```python
# Cùng "Order" nhưng KHÁC NHAU ở mỗi context!

from dataclasses import dataclass

# === Sales Context ===
@dataclass(frozen=True)
class SalesOrder:
    customer_name: str
    items: tuple[str, ...]
    total: int
    # Sales cần: giá, khuyến mãi, khách hàng

# === Shipping Context ===
@dataclass(frozen=True)
class ShippingOrder:
    order_id: str
    address: str
    weight_kg: float
    # Shipping cần: địa chỉ, trọng lượng — KHÔNG cần giá!

# === Billing Context ===
@dataclass(frozen=True)
class BillingOrder:
    order_id: str
    amount: int
    payment_method: str
    # Billing cần: số tiền, phương thức thanh toán

# Mỗi context có model riêng — không share!
# Giao tiếp giữa contexts qua Events hoặc ACL (Anti-Corruption Layer)
```

## 18.4 — Event Storming

```python
# Event Storming: workshop technique để khám phá domain
# 1. Liệt kê Domain Events (quá khứ): "Order Placed", "Payment Received"
# 2. Tìm Commands (triggers): "Place Order", "Process Payment"
# 3. Tìm Aggregates: Order, Payment, Inventory
# 4. Tìm Bounded Contexts: Sales, Shipping, Billing

# Result → Domain Model

from typing import Union

# Domain Events (past tense!)
@dataclass(frozen=True)
class OrderPlaced:
    order_id: str
    customer: str
    items: tuple[str, ...]

@dataclass(frozen=True)
class PaymentReceived:
    order_id: str
    amount: int

@dataclass(frozen=True)
class OrderShipped:
    order_id: str
    tracking: str

CafeDomainEvent = Union[OrderPlaced, PaymentReceived, OrderShipped]
```

---

## ✅ Checkpoint 18

> 1. DDD = business domain drives code structure
> 2. Ubiquitous Language = domain experts and developers use same terms
> 3. Bounded Contexts = separate models for separate concerns
> 4. Event Storming = discover domain events, commands, aggregates

---

---

## 🏋️ Bài tập

**Bài 1 (5 phút).** Chọn một hệ thống bạn đang làm và viết ra 10 danh từ mà **cả** dân kỹ thuật lẫn dân nghiệp vụ đều dùng. Có từ nào hai bên hiểu khác nhau không? Đó chính là chỗ Ubiquitous Language đang rò rỉ.

**Bài 2 (15 phút).** Làm Event Storming cho luồng "đặt món ăn online": liệt kê Command (mệnh lệnh), Event (sự kiện đã xảy ra), và Aggregate. Nhớ viết Event ở **thì quá khứ**.

**Bài 3 (20 phút).** Nhận diện ít nhất hai Bounded Context trong bài 2 và chỉ ra một khái niệm mang **nghĩa khác nhau** ở hai context đó (ví dụ: "Order" với bộ phận bếp và với bộ phận giao hàng).

<details>
<summary>Gợi ý bài 3</summary>

Đây chính là điểm cốt lõi của Bounded Context: cùng một từ mang nghĩa khác nhau
là chuyện **bình thường và lành mạnh**. Sai lầm là cố ép một model "Order" duy
nhất phục vụ mọi phòng ban — kết quả luôn là một class khổng lồ mà không ai hiểu trọn.
</details>

---

## 🔧 Troubleshooting

| Vấn đề thường gặp | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Domain model biến thành túi getter/setter | Bắt đầu từ bảng database thay vì từ nghiệp vụ | Mô hình hoá hành vi trước, lưu trữ sau |
| Một model dùng cho mọi phòng ban | Chưa tách Bounded Context | Chấp nhận vài model, nối bằng ACL (Ch23) |
| Event đặt tên như mệnh lệnh | Nhầm Command với Event | Event ở quá khứ (`OrderPlaced`), Command ở mệnh lệnh (`PlaceOrder`) |
| Dân nghiệp vụ không hiểu tên class | Đặt tên theo thuật ngữ kỹ thuật | Đổi sang chính từ họ dùng — đó là Ubiquitous Language |

## Tóm tắt

- ✅ **DDD**: Domain model = source of truth. Code mirrors business.
- ✅ **Ubiquitous Language**: `OrderService.place_order()`, not `EntityManager.process()`.
- ✅ **Bounded Contexts**: Same concept, different models per context.
- ✅ **Event Storming**: Discover events → commands → aggregates.

## Tiếp theo

→ Chapter 19: **Functional Architecture** — Hexagonal, Onion, ports & adapters in Python.
