# Chapter 38 — Observability

> **Bạn sẽ học được**:
> - Sự khác biệt giữa Monitoring và Observability
> - 3 Pillars: Logs, Metrics, Traces
> - Cấu hình structured logging (JSON) bằng structlog
> - Distributed Tracing với OpenTelemetry
>
> **Yêu cầu trước**: Chapter 37 (Distributed Systems)
> **Thời gian đọc**: ~30 phút | **Level**: Principal

---

## 38.1 — Structured Logging (structlog)

```python
# ❌ Standard logging: Dạng text, khó query (tìm bằng regex).
import logging
logging.info("User An created order ORD-123 with amount 100000")
# Muốn query tất cả order của user An? Rất khó!

# ✅ Structured logging: Dạng JSON, dễ query trên ELK/Datadog/CloudWatch.
import structlog

logger = structlog.get_logger()

# Log dạng Key-Value
logger.info("order_created", user_id="An", order_id="ORD-123", amount=100000)
# Output (JSON): 
# {"event": "order_created", "user_id": "An", "order_id": "ORD-123", "amount": 100000, "timestamp": "..."}

# Query dễ dàng: `user_id="An" AND event="order_created"`
```

## 38.2 — Metrics (Prometheus)

```python
# Metrics: Đo lường hệ thống (Count, Gauge, Histogram, Summary).
# pip install prometheus-client

from prometheus_client import Counter, Histogram
import time

# Khai báo metrics
ORDER_COUNT = Counter("app_orders_total", "Total number of orders")
ORDER_LATENCY = Histogram("app_order_processing_seconds", "Time spent processing order")

def process_order():
    start_time = time.time()
    try:
        # Xử lý logic...
        time.sleep(0.1)
        
        # Tăng counter
        ORDER_COUNT.inc()
    finally:
        # Ghi nhận thời gian chạy
        ORDER_LATENCY.observe(time.time() - start_time)

# Dashboard (Grafana) sẽ query metrics này để vẽ biểu đồ TPS (Transactions Per Second), p99 latency...
```

## 38.3 — Distributed Tracing (OpenTelemetry)

```python
# Trong Microservices, 1 request đi qua nhiều services (API -> Auth -> Order -> DB).
# Làm sao biết request đang chậm ở đâu? -> Tracing.
# Trace = Một chuỗi các Spans. Span = 1 đơn vị công việc (vd: DB query).

from opentelemetry import trace

tracer = trace.get_tracer(__name__)

def handle_request(request_id: str):
    # Tạo Root Span
    with tracer.start_as_current_span("handle_order_request") as span:
        span.set_attribute("request_id", request_id)
        
        # Gọi DB (sẽ tự động tạo child span nếu cấu hình đúng)
        user = get_user_from_db() 
        
        # Gọi External API (tạo child span thủ công)
        with tracer.start_as_current_span("call_payment_gateway") as payment_span:
            payment_span.set_attribute("payment_method", "credit_card")
            # call API...
            
# Jaeger / Zipkin sẽ vẽ biểu đồ Gantt cho thấy thời gian chạy của TỪNG BƯỚC.
```

---

## Tóm tắt

- ✅ **Logs**: Ghi lại NHỮNG GÌ đã xảy ra (dùng JSON/structlog).
- ✅ **Metrics**: Đo lường xu hướng (dùng Prometheus: Counter, Histogram).
- ✅ **Traces**: Đo lường luồng đi của request xuyên suốt hệ thống (OpenTelemetry).

## Tiếp theo

→ Chapter 39: **Deployment & DevOps** — Docker, CI/CD, Kubernetes basics.
