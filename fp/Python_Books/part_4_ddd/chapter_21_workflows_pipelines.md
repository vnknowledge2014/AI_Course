# Chapter 21 — Workflows & Pipelines

> **Bạn sẽ học được**:
> - DDD Workflows as function pipelines
> - Input → Validate → Process → Output pattern
> - Composing domain steps
>
> **Yêu cầu trước**: Chapter 20 (Domain Modeling)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 21.1 — Workflow Pattern

```python
from dataclasses import dataclass
from typing import Union

# === Input (unvalidated) ===
@dataclass(frozen=True)
class PlaceOrderInput:
    customer_email: str
    items: list[dict]  # raw, unvalidated

# === Domain Types (validated) ===
@dataclass(frozen=True)
class ValidatedOrder:
    customer_email: str
    items: tuple[tuple[str, int, int], ...]  # (product, qty, price)

@dataclass(frozen=True)
class PricedOrder:
    customer_email: str
    items: tuple[tuple[str, int, int], ...]
    total: int

@dataclass(frozen=True)
class OrderPlacedEvent:
    order_id: str
    customer_email: str
    total: int

# === Workflow Steps (pure functions) ===
def validate_order(input: PlaceOrderInput) -> ValidatedOrder:
    if "@" not in input.customer_email:
        raise ValueError("Invalid email")
    if not input.items:
        raise ValueError("Order must have at least one item")
    items = tuple(
        (item["product"], item["quantity"], item["price"])
        for item in input.items
    )
    return ValidatedOrder(customer_email=input.customer_email, items=items)

def price_order(order: ValidatedOrder) -> PricedOrder:
    total = sum(qty * price for _, qty, price in order.items)
    return PricedOrder(
        customer_email=order.customer_email,
        items=order.items,
        total=total,
    )

def create_event(order: PricedOrder) -> OrderPlacedEvent:
    import uuid
    return OrderPlacedEvent(
        order_id=str(uuid.uuid4())[:8],
        customer_email=order.customer_email,
        total=order.total,
    )

# === Composed Workflow ===
def place_order_workflow(input: PlaceOrderInput) -> OrderPlacedEvent:
    validated = validate_order(input)
    priced = price_order(validated)
    event = create_event(priced)
    return event

# === Test ===
input_data = PlaceOrderInput(
    customer_email="an@mail.com",
    items=[
        {"product": "Coffee", "quantity": 2, "price": 35_000},
        {"product": "Cake", "quantity": 1, "price": 50_000},
    ],
)

result = place_order_workflow(input_data)
assert result.total == 120_000
assert result.customer_email == "an@mail.com"
print(f"Order placed: {result}")
```

## 21.2 — Pipe-style Workflow

```python
from functools import reduce

def pipe(value, *fns):
    return reduce(lambda acc, fn: fn(acc), fns, value)

# Same workflow, using pipe
def place_order_v2(input: PlaceOrderInput) -> OrderPlacedEvent:
    return pipe(input, validate_order, price_order, create_event)

result2 = place_order_v2(input_data)
assert result2.total == 120_000
```

---

---

## ✅ Checkpoint 21

1. Vì sao mô hình workflow thành pipeline hàm lại dễ test hơn một hàm `process_order()` dài 200 dòng?
2. `flow()` và `pipe()` của `returns` khác nhau ở đâu?
3. Một bước trong pipeline cần gọi database. Đặt nó ở đâu để lõi vẫn thuần?

<details>
<summary>Đáp án</summary>

1. Vì mỗi bước là một hàm nhỏ, thuần, có chữ ký rõ ràng — test được độc lập, không cần dựng toàn bộ ngữ cảnh. Hàm 200 dòng buộc bạn phải mock mọi thứ chỉ để kiểm một nhánh.
2. `flow(value, f, g)` **áp dụng ngay** lên một giá trị. `pipe(f, g)` **tạo ra một hàm mới** để dùng sau. Cùng một phép hợp thành, khác thời điểm.
3. Đưa dữ liệu vào **trước** khi vào pipeline (đọc xong rồi truyền vào), hoặc tiêm hàm qua `RequiresContext`. Không gọi I/O ở giữa chuỗi hàm thuần.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Viết pipeline `validate → apply_discount → calculate_tax → build_receipt`, mỗi bước trả `Result`. Test riêng từng bước.

**Bài 2 (15 phút).** Chèn thêm một bước `check_inventory` vào giữa pipeline mà **không** sửa các bước khác. Đây chính là phép thử cho một thiết kế composable.

**Bài 3 (20 phút).** Dùng `RequiresContext` để `check_inventory` nhận repository qua DI thay vì import trực tiếp. So sánh độ khó khi viết test cho hai cách.

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| `flow()` báo lỗi kiểu | Bước sau nhận kiểu khác bước trước trả về | Vẽ chữ ký từng bước ra giấy; kiểu phải nối được |
| Dùng `map` khi lẽ ra phải `bind` | Hàm trả `Result` nhưng gọi bằng `map` → `Result[Result[...]]` | Hàm trả `Result` thì dùng `bind`; hàm trả giá trị thuần thì dùng `map` |
| Pipeline khó debug | Không thấy giá trị trung gian | Chèn một bước `tap` chỉ log rồi trả nguyên giá trị |
| Bước cần hai đầu vào | Pipeline chỉ chuyền một giá trị | Gói vào dataclass ngữ cảnh, hoặc `partial` tham số cố định |

## Tóm tắt

- ✅ **Workflow**: Input → Validate → Process → Output/Event
- ✅ **Each step**: Pure function, specific input/output type
- ✅ **Compose**: `pipe(input, step1, step2, step3)`
- ✅ **Types document the flow**: Read types = understand workflow

## Tiếp theo

→ Chapter 22: **Railway-Oriented Programming** — Error handling as data flow.
