# Chapter 38 — Observability: Thấu thị Hệ thống

> **Bạn sẽ học được**:
> - Tại sao Monitoring (Giám sát) là chưa đủ, và bạn cần **Observability** (Thấu thị) để tìm ra các lỗi "chưa từng biết đến".
> - Bộ 3 trụ cột (3 Pillars): Logs, Metrics, và Traces.
> - Bỏ ngay `logging` tiêu chuẩn để chuyển sang **Structured Logging** (JSON).
> - Cách vẽ biểu đồ đo lường với **Metrics (Prometheus)**.
> - Cách theo dõi một Request xuyên qua 10 microservices bằng **Distributed Tracing (OpenTelemetry)**.
>
> **Yêu cầu trước**: Chapter 37 (Distributed Systems)
> **Thời gian đọc**: ~30 phút | **Level**: Principal
> **Kết quả cuối cùng**: Khi có sự cố Production lúc 2 giờ sáng, bạn biết chính xác cần nhìn vào đâu để sửa lỗi trong 5 phút thay vì mò mẫm cả đêm.

---

## 38.1 — Monitoring vs Observability

- **Monitoring (Giám sát)**: Trả lời câu hỏi *"Hệ thống có đang hỏng không?"*. Bạn đặt cảnh báo (Alert) cho những thứ bạn *biết trước là có thể hỏng* (Ví dụ: CPU > 90%, RAM > 80%).
- **Observability (Thấu thị/Khả năng quan sát)**: Trả lời câu hỏi *"TẠI SAO nó lại hỏng?"*. Trong hệ thống phân tán, các lỗi thường rất kỳ lạ và bạn chưa từng lường trước (Unknown Unknowns). Observability cung cấp đủ dữ liệu để bạn debug mọi vấn đề xảy ra trên Production y như đang debug ở máy Local.

Để làm được điều này, chúng ta cần 3 trụ cột: **Logs, Metrics, và Traces**.

---

## 38.2 — Trụ cột 1: Logs (Structured Logging)

Log là những chuỗi văn bản ghi lại **những gì đã xảy ra**. 
Nhưng nếu bạn dùng thư viện `logging` mặc định của Python, bạn đang tự làm khổ mình.

```python
# ❌ CHUẨN LOGGING CŨ (Dạng văn bản thuần)
import logging
logging.info("User An created order ORD-123 with amount 100000")
```
Vấn đề: Giả sử hệ thống của bạn có 1 triệu dòng log mỗi ngày. Sếp yêu cầu: *"Tìm cho anh tổng doanh thu của các order bị lỗi hôm qua"*.
Bạn sẽ phải dùng Regex (Regular Expression) cực kỳ phức tạp để móc ra chữ `An`, `ORD-123`, và `100000` từ chuỗi text trên. Rất chậm và dễ sai.

**Giải pháp: Structured Logging (Log có cấu trúc JSON).**
Sử dụng thư viện `structlog`, mọi dòng log đều là một object JSON (Key-Value). Các hệ thống phân tích log (ElasticSearch, Datadog) sinh ra là để parse JSON!

```python
# ✅ STRUCTURED LOGGING
import structlog

logger = structlog.get_logger()

# Log dưới dạng Key-Value arguments
logger.info(
    "order_created", 
    user_id="An", 
    order_id="ORD-123", 
    amount=100000,
    status="success"
)
# Output thực tế ghi ra file: 
# {"event": "order_created", "user_id": "An", "order_id": "ORD-123", "amount": 100000, "status": "success", "timestamp": "2023-10-27T10:00:00Z"}
```
Bây giờ truy vấn cực kỳ đơn giản: `SELECT SUM(amount) FROM logs WHERE status='error' AND date='yesterday'`.

---

## 38.3 — Trụ cột 2: Metrics (Đo lường với Prometheus)

Logs tốn rất nhiều dung lượng ổ cứng. Bạn không thể dùng Logs để đếm xem có bao nhiêu request mỗi giây, vì nó sẽ làm sập server ghi log.
Thay vào đó, ta dùng **Metrics**. Metrics là các con số thống kê được cộng dồn trên RAM. Thư viện phổ biến nhất là `prometheus_client`.

Có 3 loại Metrics chính:
1. **Counter**: Bộ đếm chỉ tăng lên (Ví dụ: Tổng số đơn hàng, Tổng số lỗi).
2. **Gauge**: Bộ đếm có thể tăng/giảm (Ví dụ: CPU Usage, Số người đang online).
3. **Histogram**: Đo lường sự phân bổ (Ví dụ: Phân bổ thời gian phản hồi API để tìm ra p99 latency).

```python
from prometheus_client import Counter, Histogram
import time

# Khai báo Metrics
ORDER_COUNT = Counter("app_orders_total", "Tổng số đơn hàng đã tạo")
ORDER_LATENCY = Histogram("app_order_processing_seconds", "Thời gian xử lý 1 đơn hàng")

def process_order():
    start_time = time.time()
    try:
        # Xử lý logic...
        time.sleep(0.1)
        
        # Tăng counter thêm 1
        ORDER_COUNT.inc()
    finally:
        # Ghi nhận thời gian chạy vào Histogram
        ORDER_LATENCY.observe(time.time() - start_time)
```
Grafana sẽ định kỳ (vd: 10 giây/lần) gọi vào API `/metrics` của bạn để hút các con số này về và vẽ thành biểu đồ tuyệt đẹp.

---

## 38.4 — Trụ cột 3: Distributed Tracing (OpenTelemetry)

Logs và Metrics là đủ cho một ứng dụng Monolith. Nhưng với Microservices, khi 1 user click nút "Mua hàng", Request của họ có thể đi qua: `API Gateway -> Auth Service -> Order Service -> Payment Service -> Database`.
Làm sao biết Request đang tắc (chậm) ở đoạn nào? Làm sao nối log của 5 con server lại với nhau?

**Distributed Tracing (OpenTelemetry)** giải quyết bằng cách sinh ra một `TraceID` duy nhất ở API Gateway, và truyền cái ID đó qua tất cả các Services.
- **Trace**: Là toàn bộ hành trình của 1 Request.
- **Span**: Là 1 đoạn nhỏ trong hành trình đó (Ví dụ: Span 1 là gọi DB, Span 2 là gọi Payment API).

```python
from opentelemetry import trace

# Lấy tracer
tracer = trace.get_tracer(__name__)

def handle_request(request_id: str):
    # Bắt đầu một đo lường (Span)
    with tracer.start_as_current_span("handle_order_request") as span:
        # Đính kèm metadata vào Span (giống như Log)
        span.set_attribute("request_id", request_id)
        
        user = get_user_from_db()  # Giả sử hàm này tốn 50ms
        
        # Bắt đầu một Span con (Tự động lồng vào Span cha)
        with tracer.start_as_current_span("call_payment_gateway") as payment_span:
            payment_span.set_attribute("payment_method", "credit_card")
            call_api() # Giả sử API này tốn 2000ms
```

Dữ liệu Tracing này sẽ được gửi về một hệ thống như **Jaeger** hoặc **Datadog**. Tại đó, bạn sẽ thấy một biểu đồ Gantt trực quan:
```text
[handle_order_request] (2050ms)
  |-- [get_user_from_db] (50ms)
  |-- [call_payment_gateway] (2000ms) ⚠️ CHẬM Ở ĐÂY NÀY!
```
Bạn vừa tiết kiệm được 3 tiếng ngồi mò mẫm đọc log!

---

## Tóm tắt

- ✅ **Logs (structlog)**: Chuyển sang JSON để máy móc có thể dễ dàng query và thống kê.
- ✅ **Metrics (Prometheus)**: Các con số thống kê nhẹ nhàng (Counter, Histogram) để vẽ Dashboard thời gian thực.
- ✅ **Traces (OpenTelemetry)**: Bản đồ X-Quang soi rõ hành trình của một request đi xuyên qua hàng chục microservices.

## Tiếp theo

Bạn đã có một hệ thống Code xịn, Test phủ kín, Bảo mật tận răng, Thấu thị rõ ràng. Giờ chỉ còn một bước cuối cùng: Đưa nó ra cho cả thế giới sử dụng.
Hẹn gặp bạn ở chương cuối cùng của cuốn sách này: **Chapter 40: Deployment & DevOps**.
