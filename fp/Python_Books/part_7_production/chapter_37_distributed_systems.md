# Chapter 37 — Distributed Systems

> **Bạn sẽ học được**:
> - Async communication vs Sync REST APIs
> - Message Queues: RabbitMQ / Kafka concepts
> - Event-Driven Architecture (EDA)
> - Outbox Pattern cho distributed transactions
>
> **Yêu cầu trước**: Chapter 36 (App Security)
> **Thời gian đọc**: ~30 phút | **Level**: Principal

---

## 37.1 — The Problem with Sync APIs

```python
# ❌ Sync HTTP Call chain: Order -> Payment -> Inventory -> Shipping
# Vấn đề:
# 1. Chậm: Order phải đợi 3 services khác.
# 2. Lỗi cục bộ = Lỗi toàn hệ thống: Nếu Inventory sập, khách không mua được hàng.
# 3. Coupling: Order service phải biết về Payment, Inventory, Shipping.

import requests

def place_order_sync(order_data: dict):
    # 1. Lưu order (Local DB)
    save_order(order_data)
    # 2. Gọi Payment API (External)
    resp = requests.post("http://payment/charge", json={"amount": 100})
    if not resp.ok:
        raise Exception("Payment failed")
    # ... Lỗi ở đây thì sao? Order đã lưu nhưng chưa trừ kho?
```

## 37.2 — Event-Driven Architecture (EDA)

```python
# ✅ Giải pháp: Bắn Event ra Message Queue.
# Order Service KHÔNG cần biết ai sẽ xử lý. Nó chỉ nói: "Có order mới!".

from dataclasses import dataclass
import json

@dataclass(frozen=True)
class OrderCreatedEvent:
    order_id: str
    amount: int
    customer: str

def place_order_async(order_data: dict, message_broker):
    # 1. Lưu order (Local DB)
    order = save_order(order_data)
    
    # 2. Tạo Event
    event = OrderCreatedEvent(order.id, order.total, order.customer)
    
    # 3. Bắn lên Queue (Topic: "orders")
    message_broker.publish(
        topic="orders", 
        message=json.dumps(event.__dict__)
    )
    
    return order

# Payment Service (Listener):
def on_order_created(message: str):
    data = json.loads(message)
    charge_customer(data["amount"])
```

## 37.3 — Transactional Outbox Pattern

```python
# Vấn đề: `save_order` (DB) và `publish` (Queue) là 2 hệ thống khác nhau.
# Nếu DB thành công nhưng Queue sập -> Mất Event! (Dual Write Problem).

# Giải pháp: Outbox Pattern. Lưu Order VÀ Event vào cùng 1 database transaction.

from sqlalchemy import text

def place_order_outbox(engine, order_data: dict):
    event_data = {"order_id": "123", "amount": 100}
    
    with engine.begin() as conn:  # Transaction start
        # 1. Lưu Order
        conn.execute(
            text("INSERT INTO orders (id, amount) VALUES (:id, :amount)"),
            {"id": "123", "amount": 100}
        )
        # 2. Lưu Event vào bảng Outbox (CÙNG TRANSACTION!)
        conn.execute(
            text("INSERT INTO outbox (topic, payload) VALUES (:topic, :payload)"),
            {"topic": "order_created", "payload": json.dumps(event_data)}
        )
    # Transaction commit. Cả 2 đều thành công hoặc cùng thất bại.

# Sau đó, có 1 process chạy ngầm (Relay) đọc bảng Outbox và bắn lên Queue.
```

---

## Tóm tắt

- ✅ **Sync APIs**: Chậm, dễ sập dây chuyền (cascading failures).
- ✅ **EDA**: Async, decoupled. Giao tiếp qua Events.
- ✅ **Outbox Pattern**: Giải quyết "Dual Write Problem" bằng local transaction.

## Tiếp theo

→ Chapter 38: **Observability** — Logging, Metrics, và Tracing.
