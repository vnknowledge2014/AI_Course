# Chapter 34 — Advanced Data: Caching & Background Tasks

> **Bạn sẽ học được**:
> - Các chiến lược Caching phổ biến (Cache-aside, Write-through, Write-behind).
> - Cách thiết kế Cache bằng `Protocol` và Decorator Pattern.
> - Tại sao Background Tasks cần thiết và hạn chế của `asyncio` so với Message Queues (Celery/RabbitMQ).
> - Thuật toán Rate Limiting (Token Bucket & Sliding Window) bảo vệ hệ thống.
>
> **Yêu cầu trước**: Chapter 33 (Database)
> **Thời gian đọc**: ~30 phút | **Level**: Principal
> **Kết quả cuối cùng**: Biết cách tăng tốc và bảo vệ hệ thống khỏi tải nặng thông qua Caching và Queue.

---

Khi hệ thống của bạn vượt qua mốc hàng nghìn request mỗi giây, Database quan hệ (như PostgreSQL) sẽ trở thành nút thắt cổ chai (bottleneck) lớn nhất. Bạn không thể cứ mua máy chủ to hơn mãi (Vertical Scaling). Để giải quyết, bạn cần **Caching** (giảm tải đọc) và **Background Tasks** (giảm tải ghi/xử lý đồng bộ).

## 34.1 — Các chiến lược Caching (Caching Strategies)

Cache là một lớp lưu trữ tốc độ cực cao (thường nằm trên RAM như Redis hoặc Memcached). Tuy nhiên, cái khó nhất của Caching không phải là đọc/ghi, mà là **Cache Invalidation** (Làm sao để biết dữ liệu trong cache đã cũ và cần xóa đi?).

1. **Cache-Aside (Lazy Loading)**: Phổ biến nhất.
   - *Đọc*: App tìm trong Cache. Nếu có (Hit) -> Trả về. Nếu không (Miss) -> Đọc từ DB -> Lưu vào Cache -> Trả về.
   - *Ghi*: App ghi vào DB, sau đó xóa Cache (hoặc cập nhật Cache).
2. **Write-Through**: App luôn ghi vào Cache trước, sau đó Cache tự đồng bộ xuống DB (chậm ghi, nhanh đọc).
3. **Write-Behind (Write-Back)**: Ghi vào Cache, trả về ngay. Một tiến trình nền sẽ gom các lệnh ghi và đẩy xuống DB sau (cực nhanh, rủi ro mất dữ liệu nếu Cache sập).

### Triển khai Cache-Aside với Decorator trong Python

```python
from typing import Protocol, TypeVar, Callable
from dataclasses import dataclass

T = TypeVar("T")

# 1. Định nghĩa Interface bằng Protocol (Dễ dàng thay đổi InMemory sang Redis)
class Cache(Protocol):
    def get(self, key: str) -> object | None: ...
    def set(self, key: str, value: object, ttl: int = 300) -> None: ...

class InMemoryCache:
    def __init__(self):
        self._store: dict[str, object] = {}
        
    def get(self, key: str) -> object | None:
        return self._store.get(key)
        
    def set(self, key: str, value: object, ttl: int = 300) -> None:
        # Giản lược: Bỏ qua logic xóa TTL (Time-To-Live) trong ví dụ này
        self._store[key] = value

# 2. Functional Decorator cho Cache-Aside
def cached(cache: Cache, key_fn: Callable, ttl: int = 300):
    """
    Decorator bọc một hàm. key_fn dùng để tạo Cache Key từ tham số truyền vào.
    """
    def decorator(fn):
        def wrapper(*args, **kwargs):
            key = key_fn(*args, **kwargs)
            result = cache.get(key)
            
            if result is not None:
                print(f"[CACHE HIT] {key}")
                return result
                
            print(f"[CACHE MISS] {key}")
            result = fn(*args, **kwargs)
            cache.set(key, result, ttl)
            return result
        return wrapper
    return decorator

# 3. Sử dụng
cache = InMemoryCache()

@cached(cache, lambda order_id: f"order:{order_id}")
def fetch_order(order_id: str) -> dict:
    # Mô phỏng lệnh DB chậm chạp
    return {"id": order_id, "total": 100_000}

r1 = fetch_order("ORD-1")  # [CACHE MISS] order:ORD-1
r2 = fetch_order("ORD-1")  # [CACHE HIT] order:ORD-1 (Trả về ngay lập tức, không gọi hàm fetch_order)
```

---

## 34.2 — Background Tasks & Message Queues

Giả sử API "Đăng ký User" của bạn cần:
1. Lưu User vào DB (10ms)
2. Gửi Email chào mừng (1000ms)
3. Tính toán gợi ý kết bạn (2000ms)

Nếu làm đồng bộ, User phải chờ >3 giây mới thấy màn hình thành công. Trải nghiệm tồi tệ!
Cách giải quyết: Tách việc #2 và #3 ra chạy ngầm (Background Task).

### Giới hạn của `asyncio`
Bạn có thể dùng `asyncio.create_task()` để chạy ngầm ngay trong tiến trình web (FastAPI).
Nhưng nếu Server bị crash hoặc restart (Deploy phiên bản mới), **tất cả task đang chạy dở sẽ bốc hơi**.

### Giải pháp Production: Message Queues (Celery, RQ, RabbitMQ)
Thay vì tự chạy, Server đóng gói yêu cầu thành một "Tin nhắn" (Message) gửi vào Queue (như Redis/RabbitMQ). Một cụm Server khác gọi là **Workers** sẽ bốc tin nhắn ra và xử lý.
- Đảm bảo không mất data (Persistent).
- Có cơ chế Retry tự động nếu gửi email lỗi.

Dưới đây là một ví dụ mô phỏng Task Queue đơn giản bằng `asyncio` để bạn hiểu bản chất của Worker:

```python
import asyncio
from typing import Callable, Coroutine, Any

class SimpleWorkerQueue:
    def __init__(self):
        self._tasks: list[Callable[[], Coroutine[Any, Any, None]]] = []

    def enqueue(self, task: Callable) -> None:
        # Web server ném việc vào đây (Rất nhanh)
        self._tasks.append(task)

    async def run_worker(self) -> None:
        # Worker liên tục lặp để lấy việc ra làm
        for task in self._tasks:
            try:
                await task()
            except Exception as e:
                print(f"Task failed: {e}. Should retry or move to Dead Letter Queue.")
        self._tasks.clear()

async def send_email():
    await asyncio.sleep(0.1) # Simulate network IO
    print("Email sent!")

queue = SimpleWorkerQueue()
queue.enqueue(send_email)

# Trong thực tế, run_worker chạy ở một process/máy chủ hoàn toàn khác
asyncio.run(queue.run_worker())
```

---

## 34.3 — Rate Limiting (Giới hạn tốc độ)

Để bảo vệ Database và Queue khỏi bị spam (DDOS hoặc user bấm quá nhanh), bạn cần Rate Limiting. Hai thuật toán nổi tiếng nhất:

1. **Token Bucket (Cái Xô Chứa Token)**: 
   - Xô chứa tối đa `M` token.
   - Cứ mỗi `1/R` giây, thả thêm 1 token vào xô.
   - Mỗi request rút 1 token. Hết token thì reject request.
   - Lợi ích: Cho phép burst (xử lý chớp nhoáng 1 đợt nhiều request) miễn là trong xô còn token.

2. **Sliding Window Log (Cửa Sổ Trượt)**:
   - Lưu lại timestamp của mọi request vào Redis Sorted Set.
   - Khi có request mới, xóa các timestamp cũ hơn `now - 1 phút`. Đếm số timestamp còn lại. Nếu vượt quá giới hạn -> Reject.
   - Lợi ích: Cực kỳ chính xác, không bị lỗi "đỉnh điểm ở giữa 2 cửa sổ thời gian".

---

## Tóm tắt

- ✅ **Caching**: Cứu cánh của Database. Sử dụng Decorator Pattern (rất hợp với FP) để bọc các hàm tính toán nặng/đọc DB. Luôn nhớ quản lý Cache Invalidation (TTL).
- ✅ **Background Tasks**: Đẩy những tác vụ chậm (Email, PDF, AI Inference) ra khỏi luồng xử lý Web request. Trong Production, bắt buộc dùng **Message Broker/Queue** (Celery, Redis) để tránh mất dữ liệu khi sập server.
- ✅ **Rate Limiting**: Bảo vệ tài nguyên bằng thuật toán Token Bucket hoặc Cửa sổ trượt, thường được implement ngay tại API Gateway hoặc qua middleware của FastAPI kết nối với Redis.

## Tiếp theo

Bạn đã tối ưu xong hiệu năng của hệ thống. Nhưng nếu ai cũng có thể truy cập và thay đổi data thì tốc độ chẳng có ý nghĩa gì. Hẹn gặp bạn ở **Chapter 35: Security Essentials**.
