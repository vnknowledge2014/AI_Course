# Chapter 37 — Distributed Systems & Microservices

> **Bạn sẽ học được**:
> - Vấn đề chí mạng của giao tiếp đồng bộ (Sync REST APIs): Chậm, dễ sụp đổ dây chuyền, và dính chặt (Tight Coupling).
> - Kiến trúc hướng sự kiện (Event-Driven Architecture) với Message Brokers (RabbitMQ, Kafka).
> - Giải quyết bài toán **Dual Write** bằng mẫu thiết kế Transactional Outbox.
> - Đảm bảo tính chống chịu với **Circuit Breaker** (Cầu dao điện).
>
> **Yêu cầu trước**: Chapter 36 (App Security)
> **Thời gian đọc**: ~40 phút | **Level**: Principal
> **Kết quả cuối cùng**: Chuyển đổi tư duy từ một hệ thống nguyên khối (Monolith) sang hệ thống phân tán, nơi sự cố mạng và mất dữ liệu là "chuyện bình thường ở huyện".

---

Khi hệ thống của bạn phát triển, bạn sẽ chia nhỏ nó thành các Microservices (Dịch vụ nhỏ). Ví dụ: `OrderService`, `PaymentService`, `InventoryService`.
Lúc này, cái khó nhất không phải là code logic bên trong mỗi Service, mà là **cách các Service nói chuyện với nhau**.

## 37.1 — Vấn đề của REST API đồng bộ (Synchronous)

Cách dễ nhất và... tồi tệ nhất để các Service giao tiếp là gọi API REST trực tiếp (chờ đợi nhau).

```python
import requests

def place_order_sync(order_data: dict):
    # 1. Lưu Order vào DB của OrderService
    save_order(order_data)
    
    # 2. Gọi Payment API (External)
    resp = requests.post("http://payment/charge", json={"amount": 100})
    if not resp.ok:
        raise Exception("Payment failed")
        
    # 3. Gọi Inventory API để trừ kho
    resp = requests.post("http://inventory/deduct", json={"item": "A"})
    if not resp.ok:
        raise Exception("Inventory failed")
```

Ba rủi ro thảm họa của cách làm này:
1. **Chậm trễ cộng dồn**: Thời gian phản hồi của `place_order` bằng tổng thời gian của cả 3 services. Nếu Payment mất 2s, Inventory mất 2s, user phải đợi 4s!
2. **Sụp đổ dây chuyền (Cascading Failure)**: Nếu Inventory bị sập, toàn bộ tiến trình đặt hàng bị treo theo. 1 Service chết kéo theo toàn bộ hệ thống chết.
3. **Mâu thuẫn dữ liệu (Inconsistency)**: Nếu lệnh trừ kho thất bại, nhưng tiền đã trừ ở bước 2 thì sao? Trong hệ thống phân tán, bạn không thể Rollback Database của máy chủ Payment từ máy chủ Order được!

---

## 37.2 — Event-Driven Architecture (Kiến trúc hướng sự kiện)

Thay vì ra lệnh (Command): *"Ê Payment, trừ tiền đi!"* (Coupled).
Chúng ta phát ra sự kiện (Event): *"Có một đơn hàng vừa được tạo. Ai quan tâm thì tự ra mà lấy thông tin!"* (Decoupled).

Sự kiện được đẩy vào một **Message Broker** (như RabbitMQ hoặc Kafka). Các service khác (Consumers) sẽ "đăng ký" (subscribe) để lắng nghe sự kiện này.

```python
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
    # API trả về ngay lập tức cho user! (Cực nhanh)
    message_broker.publish(topic="orders", message=json.dumps(event.__dict__))
    return order

# ------------- Ở MÁY CHỦ KHÁC -------------

# Payment Service (Listener):
def on_order_created(message: str):
    data = json.loads(message)
    charge_customer(data["amount"]) # Chạy ngầm phía sau
```
**Lợi ích**: Nếu Payment sập, Event vẫn nằm an toàn trong Queue. Khi Payment được reboot, nó sẽ đọc tiếp Event đang dở dang. Không mất mát dữ liệu, OrderService vẫn sống khỏe!

---

## 37.3 — Transactional Outbox Pattern

Nhìn vào code `place_order_async` ở trên, có một lỗ hổng chí mạng: **Bài toán Dual Write**.
Nếu bước 1 (`save_order`) thành công, nhưng ngay trước bước 3 (`publish`), máy chủ bị cúp điện thì sao?
-> Đơn hàng đã lưu vào DB, nhưng sự kiện không bao giờ được gửi đi. Khách hàng đã mua hàng nhưng không bao giờ bị trừ tiền hay nhận được hàng!

Để giải quyết, bạn không được gửi thẳng lên Broker. Bạn phải lưu Event vào **CÙNG MỘT DATABASE TRANSACTION** với Order. Đây gọi là bảng `Outbox`.

```python
from sqlalchemy import text

def place_order_outbox(engine, order_data: dict):
    event_data = {"order_id": "123", "amount": 100}
    
    # Bắt đầu một Transaction cục bộ
    with engine.begin() as conn:  
        # 1. Lưu Order vào bảng `orders`
        conn.execute(
            text("INSERT INTO orders (id, amount) VALUES (:id, :amount)"),
            {"id": "123", "amount": 100}
        )
        
        # 2. Lưu Event vào bảng `outbox` (CÙNG TRANSACTION)
        conn.execute(
            text("INSERT INTO outbox (topic, payload) VALUES (:topic, :payload)"),
            {"topic": "order_created", "payload": json.dumps(event_data)}
        )
        
    # Transaction commit. Chắc chắn cả 2 đều thành công, hoặc cùng thất bại.
```

Sau đó, bạn viết một tiến trình chạy ngầm (Message Relay hoặc dùng công cụ như Debezium) liên tục quét bảng `outbox` và gửi lên RabbitMQ. Gửi xong thì đánh dấu là `Đã gửi` hoặc xóa khỏi bảng.

---

## 37.4 — Circuit Breaker (Cầu dao điện)

Kể cả khi dùng EDA, đôi khi bạn vẫn BẮT BUỘC phải gọi API đồng bộ (Ví dụ: kiểm tra thẻ tín dụng trước khi chốt đơn). Nếu API của ngân hàng đang siêu chậm (quá tải), service của bạn cũng sẽ bị treo vì chờ đợi (Connection Timeout), dẫn đến cạn kiệt bộ nhớ.

**Circuit Breaker** hoạt động y như cầu dao điện trong nhà bạn:
- **Đóng (Closed)**: Dòng điện (request) đi qua bình thường.
- **Mở (Open)**: Nếu API lỗi hoặc quá timeout 5 lần liên tiếp -> Cúp điện! Từ chối mọi request tiếp theo ngay lập tức (Fail-fast) mà không cần gọi API nữa. Cho phép API kia có thời gian thở để phục hồi.
- **Nửa mở (Half-Open)**: Sau 30 giây, cho phép 1 request đi qua thử. Nếu thành công -> Đóng cầu dao lại bình thường. Nếu lại lỗi -> Tiếp tục Mở.

---

## Tóm tắt

- ✅ **Sync APIs**: Tránh gọi API đồng bộ nối tiếp nhau trong hệ thống phân tán.
- ✅ **EDA & Message Broker**: Cởi trói cho các service. Để chúng giao tiếp bất đồng bộ qua Events. Tăng tốc độ phản hồi và độ chịu lỗi.
- ✅ **Outbox Pattern**: Kỹ thuật bắt buộc phải biết để đảm bảo tính nhất quán (Consistency) khi bạn vừa ghi DB vừa gửi Event.
- ✅ **Circuit Breaker**: Cắt đứt các kết nối đang hấp hối để cứu sống hệ thống của bạn.

## Tiếp theo

Hệ thống phân tán giải quyết bài toán scale, nhưng tạo ra một cơn ác mộng mới: **Làm sao để Debug?**
Khi một request đi qua 5 services và có 1 lỗi văng ra, bạn làm sao biết lỗi nằm ở máy chủ nào? 
Mời bạn đến với **Chapter 38: Observability**.
