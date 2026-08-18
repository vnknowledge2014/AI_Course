# Chapter 38 — System Design Thinking

> **Bạn sẽ học được**:
> - Capacity estimation: nhẩm ra QPS, dung lượng, băng thông trước khi viết dòng code nào
> - Load balancing và mô hình process của Python (GIL, Gunicorn workers, Uvicorn)
> - Bốn tầng cache và câu hỏi khó nhất: khi nào invalidate
> - Chọn giữa REST (FastAPI), gRPC (`grpcio`) và GraphQL (`strawberry`)
> - Monolith-first: tại sao chia microservices sớm thường là sai lầm đắt giá
> - Ba bài tập thiết kế kinh điển: URL shortener, rate limiter, chat
>
> **Yêu cầu trước**: Chapter 37 (Distributed Systems), Chapter 37B (Observability)
> **Thời gian đọc**: ~50 phút | **Level**: Principal
> **Kết quả cuối cùng**: Bạn ngồi vào buổi phỏng vấn System Design (hoặc buổi họp
> thiết kế thật) và biết phải hỏi gì trước khi vẽ hộp và mũi tên.

---

Bạn có để ý kiến trúc sư xây nhà không bao giờ bắt đầu bằng việc chọn màu sơn?
Họ hỏi trước: nhà cho mấy người ở, đất rộng bao nhiêu, ngân sách thế nào, nền
đất chịu được mấy tầng. Chọn màu sơn là việc của tháng thứ sáu.

System Design cũng vậy. Người mới thường nhảy ngay vào "dùng Kafka hay RabbitMQ",
trong khi câu hỏi đúng là: **hệ thống này có bao nhiêu request mỗi giây?** Rất
nhiều hệ thống mà bạn tưởng cần Kafka thực ra chạy thoải mái trên một con
PostgreSQL với một bảng `jobs`.

Chương này dạy bạn **thứ tự đặt câu hỏi**. Công nghệ chỉ là hệ quả.

---

## 38.1 — Capacity Estimation: nhẩm trước khi vẽ

Trước khi chọn database, hãy tính xem bạn đang nói về quy mô nào. Toàn bộ phép
tính này làm trên giấy trong 3 phút, sai số một bậc (10×) vẫn chấp nhận được —
mục tiêu là phân biệt "1 server" với "100 server", không phải ra con số chính xác.

**Ba con số cần nhớ:**

| Đại lượng | Con số tròn để nhẩm |
|---|---|
| 1 ngày | ~86.400 giây ≈ **10⁵ giây** |
| 1 triệu request/ngày | ≈ **12 QPS** trung bình |
| Peak thường gấp trung bình | **2–10 lần** |

```python
# filename: capacity.py
from dataclasses import dataclass

SECONDS_PER_DAY = 86_400


@dataclass(frozen=True)
class Estimate:
    """Ước lượng tải — thuần tính toán, không I/O, nên test được dễ dàng."""

    daily_active_users: int
    actions_per_user_per_day: int
    bytes_per_action: int
    peak_multiplier: float = 5.0

    @property
    def avg_qps(self) -> float:
        return self.daily_active_users * self.actions_per_user_per_day / SECONDS_PER_DAY

    @property
    def peak_qps(self) -> float:
        return self.avg_qps * self.peak_multiplier

    @property
    def storage_per_year_gb(self) -> float:
        daily = self.daily_active_users * self.actions_per_user_per_day * self.bytes_per_action
        return daily * 365 / 1_000_000_000


# Ví dụ: mạng xã hội nội bộ 1 triệu DAU, mỗi người 20 hành động/ngày, 1KB/hành động
social = Estimate(
    daily_active_users=1_000_000,
    actions_per_user_per_day=20,
    bytes_per_action=1_000,
)

assert round(social.avg_qps) == 231
assert round(social.peak_qps) == 1157
assert round(social.storage_per_year_gb) == 7300

print(f"Trung bình {social.avg_qps:.0f} QPS, peak {social.peak_qps:.0f} QPS")
print(f"Lưu trữ: {social.storage_per_year_gb:.0f} GB/năm")
```

**Đọc kết quả này thế nào?**

- **231 QPS trung bình, ~1.200 QPS peak.** Một instance FastAPI + Uvicorn xử lý
  được cỡ 1.000–3.000 QPS cho endpoint đơn giản. Vậy bạn cần **vài** server, không
  phải vài trăm. Chưa cần microservices.
- **7,3 TB/năm.** Vượt xa giới hạn thoải mái của một node PostgreSQL đơn (~vài TB).
  Đây mới là chỗ *thật sự* cần bàn về sharding hoặc chuyển log-type data sang
  object storage.

Bài học: con số về **storage** thường ép bạn thay đổi kiến trúc trước con số về
**QPS**. Người mới hay lo ngược lại.

---

## 38.2 — Load Balancing và mô hình process của Python

Đây là chỗ Python khác hẳn Go hay Rust, và bạn **bắt buộc** phải hiểu để thiết
kế đúng.

### GIL nghĩa là gì cho việc thiết kế

CPython có **GIL** (Global Interpreter Lock): tại mỗi thời điểm chỉ một thread
chạy bytecode Python. Hệ quả trực tiếp:

- **Tác vụ I/O-bound** (gọi DB, gọi API, đọc file): GIL được nhả trong lúc chờ →
  `async`/`await` cho throughput rất tốt trên **một** process.
- **Tác vụ CPU-bound** (tính toán, xử lý ảnh, parse khối lượng lớn): threads
  **không** giúp gì. Phải dùng **nhiều process**.

```python
# filename: worker_model.py
import os

# Quy tắc ngón tay cái cho Gunicorn + Uvicorn worker
def recommended_workers(cpu_cores: int, io_bound: bool) -> int:
    """
    I/O-bound (API gọi DB/HTTP): mỗi worker đã tự xử lý được nhiều request
    đồng thời nhờ async, nên bám sát số core.

    CPU-bound: vẫn là (2n + 1) — công thức kinh điển của Gunicorn, thừa 1 worker
    để lấp chỗ trống khi có worker đang bị block.
    """
    return cpu_cores if io_bound else 2 * cpu_cores + 1


assert recommended_workers(4, io_bound=True) == 4
assert recommended_workers(4, io_bound=False) == 9

print(f"Máy này có {os.cpu_count()} core")
```

Lệnh chạy production tương ứng:

```bash
# API I/O-bound (điển hình của FastAPI): 4 process, mỗi process 1 event loop
gunicorn app.main:app \
  --worker-class uvicorn.workers.UvicornWorker \
  --workers 4 \
  --bind 0.0.0.0:8000 \
  --timeout 60 \
  --graceful-timeout 30
```

> **Bẫy kinh điển:** đặt `--workers 32` trên máy 4 core vì "nhiều worker thì
> nhanh hơn". Mỗi worker là một process Python riêng, ngốn 100–300 MB RAM và một
> bộ connection pool riêng tới database. 32 worker × pool 10 = **320 connection**
> — vượt `max_connections` mặc định của PostgreSQL (100) và làm sập DB trước khi
> app kịp chậm.

### L4 vs L7

| | **L4 (TCP)** | **L7 (HTTP)** |
|---|---|---|
| Nhìn thấy | IP + port | Path, header, cookie |
| Tốc độ | Nhanh hơn | Chậm hơn chút |
| Làm được | Round-robin, least-conn | Route theo path, A/B test, sticky session, rewrite |
| Ví dụ | HAProxy (tcp mode), NLB | Nginx, Traefik, ALB |

Với FastAPI thực tế bạn hầu như luôn dùng **L7** vì cần route `/api/*` sang app
và `/static/*` sang CDN.

### Health check — phần hay bị bỏ quên

```python
# filename: health.py
from fastapi import APIRouter, Response, status

router = APIRouter()


@router.get("/healthz")
async def liveness() -> dict[str, str]:
    """Process còn sống không? KHÔNG kiểm tra dependency.

    Nếu endpoint này fail, orchestrator sẽ RESTART container. Đừng để nó fail
    chỉ vì database tạm thời sập — restart app không sửa được database.
    """
    return {"status": "ok"}


@router.get("/readyz")
async def readiness(response: Response) -> dict[str, object]:
    """Sẵn sàng NHẬN traffic chưa? CÓ kiểm tra dependency.

    Nếu fail, load balancer rút instance này khỏi pool nhưng KHÔNG restart.
    """
    checks = {"database": await _can_reach_db(), "cache": await _can_reach_redis()}
    if not all(checks.values()):
        response.status_code = status.HTTP_503_SERVICE_UNAVAILABLE
    return {"ready": all(checks.values()), "checks": checks}
```

Phân biệt `liveness` và `readiness` là một trong những khác biệt rõ nhất giữa
người đã vận hành production và người chưa.

---

## ✅ Checkpoint 38.1–38.2

Tự trả lời trước khi đọc tiếp:

1. Hệ thống 500.000 DAU, mỗi người 10 request/ngày. Peak QPS xấp xỉ bao nhiêu?
2. API của bạn chủ yếu gọi PostgreSQL. Máy 8 core. Nên đặt bao nhiêu Gunicorn worker?
3. Database sập. `/healthz` nên trả 200 hay 503? Còn `/readyz`?

<details>
<summary>Đáp án</summary>

1. `500.000 × 10 / 86.400 ≈ 58` QPS trung bình → peak ×5 ≈ **290 QPS**. Một server là đủ.
2. I/O-bound → **8 worker**. Và nhớ tính lại pool size: 8 × pool ≤ `max_connections`.
3. `/healthz` trả **200** (process vẫn sống, restart không giúp gì).
   `/readyz` trả **503** (không phục vụ được request, hãy rút khỏi load balancer).
</details>

---

## 38.3 — Bốn tầng cache

Cache không phải một thứ — nó là **bốn** thứ, mỗi tầng có TTL và cách invalidate
riêng. Request đi từ trên xuống, dừng ở tầng đầu tiên có dữ liệu.

```
Browser cache      →  Cache-Control header, tính bằng giây–phút
      ↓ miss
CDN edge           →  Cloudflare/CloudFront, tính bằng phút–giờ
      ↓ miss
Application cache  →  Redis, tính bằng giây–phút    ← chỗ bạn kiểm soát nhiều nhất
      ↓ miss
In-process cache   →  @lru_cache, sống theo đời process
      ↓ miss
Database
```

```python
# filename: caching.py
from functools import lru_cache
import json
import redis.asyncio as redis

r = redis.from_url("redis://localhost:6379", decode_responses=True)


# ── Tầng 4: in-process, cho dữ liệu gần như bất biến ────────────────────────
@lru_cache(maxsize=512)
def tax_rate_for(country: str) -> float:
    """Thuế suất đổi vài lần một năm — cache trong RAM process là hợp lý.

    ⚠️ Mỗi Gunicorn worker có bản sao riêng. Với 8 worker, bạn có 8 cache độc
    lập, và không có cách nào invalidate chúng ngoài việc restart. Chỉ dùng cho
    dữ liệu mà "cũ vài giờ" là chấp nhận được.
    """
    return {"VN": 0.10, "US": 0.07, "JP": 0.10}[country]


# ── Tầng 3: Redis, cache-aside — mẫu phổ biến nhất ──────────────────────────
async def get_product(product_id: str, *, ttl: int = 300) -> dict:
    """Cache-aside (lazy loading): đọc cache → miss thì đọc DB → ghi lại cache."""
    key = f"product:{product_id}"

    if cached := await r.get(key):
        return json.loads(cached)

    product = await _fetch_product_from_db(product_id)
    # TTL là lưới an toàn: kể cả khi quên invalidate, dữ liệu sai cũng tự hết hạn.
    await r.setex(key, ttl, json.dumps(product))
    return product


async def update_product(product_id: str, changes: dict) -> None:
    """Ghi DB rồi XOÁ cache — đừng ghi đè cache."""
    await _update_product_in_db(product_id, changes)
    await r.delete(f"product:{product_id}")
```

> **Tại sao xoá cache chứ không ghi đè?** Nếu hai request cùng update, thứ tự ghi
> vào cache có thể ngược với thứ tự ghi vào DB → cache giữ giá trị cũ **vĩnh viễn**.
> Xoá thì lần đọc sau luôn lấy lại sự thật từ DB. Quy tắc: **DB là nguồn sự thật,
> cache chỉ được phép rỗng hoặc đúng, không được phép sai.**

### Ba thất bại kinh điển của cache

| Hiện tượng | Chuyện gì xảy ra | Cách chữa |
|---|---|---|
| **Cache stampede** | Key hot hết hạn, 1.000 request cùng lúc đâm vào DB | Lock phân tán, hoặc thêm jitter vào TTL |
| **Cache penetration** | Query key không tồn tại → luôn miss → luôn xuống DB | Cache cả giá trị rỗng với TTL ngắn |
| **Cache avalanche** | Nhiều key hết hạn cùng lúc | TTL ngẫu nhiên hoá: `ttl + random(0, ttl // 10)` |

```python
import random

async def set_with_jitter(key: str, value: str, base_ttl: int = 300) -> None:
    """Chống avalanche: rải điểm hết hạn ra thay vì dồn một chỗ."""
    await r.setex(key, base_ttl + random.randint(0, base_ttl // 10), value)
```

---

## 38.4 — REST vs gRPC vs GraphQL

Không có cái nào "tốt hơn" — chúng tối ưu cho ba thứ khác nhau.

| | **REST** (FastAPI) | **gRPC** (`grpcio`) | **GraphQL** (`strawberry`) |
|---|---|---|---|
| Định dạng | JSON qua HTTP/1.1 | Protobuf qua HTTP/2 | JSON qua HTTP |
| Contract | OpenAPI (sinh tự động) | `.proto` (bắt buộc) | SDL schema |
| Điểm mạnh | Ai cũng dùng được, debug bằng `curl` | Nhanh, nhỏ, streaming 2 chiều | Client tự chọn field cần lấy |
| Điểm yếu | Over-fetch / under-fetch | Trình duyệt không gọi trực tiếp được | Query N+1, khó cache theo HTTP |
| Dùng khi | **API công khai, mặc định** | **Service ↔ service nội bộ** | **Nhiều client cần shape dữ liệu khác nhau** |

```python
# filename: api_styles.py
# ── REST với FastAPI: DTO ở rìa, domain type ở trong ────────────────────────
from fastapi import FastAPI
from pydantic import BaseModel, Field

app = FastAPI()


class CreateOrderRequest(BaseModel):
    """DTO — hình dạng của dữ liệu ĐI VÀO qua dây, không phải domain model.

    Ranh giới này chính là Anti-Corruption Layer ở Chapter 23: thay đổi phía
    client không được phép rò rỉ vào domain.
    """
    customer_id: str = Field(min_length=1)
    quantity: int = Field(gt=0, le=100)


@app.post("/orders", status_code=201)
async def create_order(req: CreateOrderRequest) -> dict[str, str]:
    order = await _place_order(req.customer_id, req.quantity)
    return {"order_id": order.id}
```

**Lời khuyên thực dụng:** bắt đầu bằng REST. Chuyển sang gRPC khi bạn **đo được**
rằng serialize JSON đang là bottleneck giữa các service nội bộ. Thêm GraphQL khi
bạn **thật sự** có nhiều loại client (web + iOS + Android) cần shape khác nhau —
không phải vì nó đang thời thượng.

---

## 38.5 — Monolith-first và Định luật Conway

> *"Tổ chức thiết kế hệ thống sẽ tạo ra thiết kế sao chép đúng cấu trúc giao tiếp
> của chính tổ chức đó."* — Melvin Conway, 1967

Nghĩa là: nếu bạn có **một** team mà chia hệ thống thành **tám** microservice,
bạn không mua được sự độc lập — bạn chỉ mua thêm tám pipeline CI, tám cách deploy,
và biến mọi lời gọi hàm thành một lời gọi mạng có thể fail.

### Đường tiến hoá đúng

```
Monolith  →  Modular Monolith  →  Tách service khi có LÝ DO ĐO ĐƯỢC
```

**Modular Monolith** là chặng bị bỏ qua nhiều nhất, và cũng là chặng có giá trị
nhất. Vẫn một process, một lần deploy, nhưng ranh giới module được thực thi:

```python
# filename: modular_monolith.py
# src/
#   orders/          ← bounded context (Chapter 18)
#     domain.py      ← pure, không import từ context khác
#     service.py     ← public API của module này
#     repository.py
#   billing/
#     domain.py
#     service.py
#   shared/
#     events.py      ← cách DUY NHẤT hai context nói chuyện với nhau

# billing/service.py
from shared.events import OrderPlaced        # ✅ qua event chung
# from orders.repository import OrderRepo    # ❌ đâm xuyên ruột module khác
```

Ép ranh giới này bằng linter, không bằng lời hứa:

```toml
# pyproject.toml — ruff chặn import xuyên context ngay từ CI
[tool.ruff.lint.flake8-tidy-imports.banned-api]
"orders.repository".msg = "billing không được đọc thẳng repo của orders — dùng shared.events"
"orders.domain".msg     = "Domain của orders là nội bộ — gọi qua orders.service"
```

Khi ranh giới đã sạch, tách một module thành service riêng chỉ là việc đổi lời
gọi hàm thành lời gọi HTTP. Khi ranh giới chưa sạch, tách service biến một
codebase rối thành một **hệ phân tán** rối — khó gấp bội.

**Chỉ tách service khi có ít nhất một lý do đo được:**

- Một module cần scale theo trục khác hẳn (ví dụ xử lý ảnh ngốn CPU, phần còn lại I/O)
- Hai team đang giẫm chân nhau khi deploy
- Một phần cần ràng buộc tuân thủ khác (dữ liệu thanh toán, PCI-DSS)

"Cho hiện đại" không nằm trong danh sách này.

---

## 38.6 — Ba bài toán thiết kế kinh điển

### A. URL Shortener

**Hỏi trước:** bao nhiêu URL mới mỗi ngày? Tỷ lệ đọc/ghi? Link có hết hạn không?
Có cần thống kê click không?

Giả sử 100 triệu URL mới/năm, đọc/ghi = 100:1.

```python
# filename: url_shortener.py
import string

ALPHABET = string.digits + string.ascii_letters  # 62 ký tự


def encode_base62(n: int) -> str:
    """ID tự tăng → chuỗi ngắn. 62^7 ≈ 3.500 tỷ — thừa cho 100 triệu/năm."""
    if n == 0:
        return ALPHABET[0]
    out: list[str] = []
    while n:
        n, rem = divmod(n, 62)
        out.append(ALPHABET[rem])
    return "".join(reversed(out))


def decode_base62(s: str) -> int:
    n = 0
    for ch in s:
        n = n * 62 + ALPHABET.index(ch)
    return n


assert encode_base62(0) == "0"
assert encode_base62(61) == "Z"
assert encode_base62(125) == "21"
assert decode_base62(encode_base62(123_456_789)) == 123_456_789
```

**Các quyết định và lý do:**

- **Base62 từ ID tự tăng**, không phải hash. Hash cần xử lý va chạm; counter thì không.
- **Đọc/ghi 100:1** → đây là bài toán *cache*. Redis đứng trước, DB chỉ nhận cache miss.
- **Redirect dùng `301` hay `302`?** `301` (permanent) được trình duyệt cache
  vĩnh viễn → nhanh nhất nhưng **mất sạch số liệu click**. Nếu cần analytics,
  phải dùng `302`. Đây là một trade-off thật, không phải chi tiết vụn vặt.

### B. Rate Limiter

```python
# filename: rate_limiter.py
import time
import redis.asyncio as redis

r = redis.from_url("redis://localhost:6379")

# Sliding window log bằng Redis sorted set. Chạy nguyên tử qua Lua để tránh
# race condition giữa nhiều worker/instance.
SLIDING_WINDOW = """
local key, now, window, limit = KEYS[1], tonumber(ARGV[1]), tonumber(ARGV[2]), tonumber(ARGV[3])
redis.call('ZREMRANGEBYSCORE', key, 0, now - window)   -- bỏ các lần gọi đã ra khỏi cửa sổ
local count = redis.call('ZCARD', key)
if count < limit then
    redis.call('ZADD', key, now, now .. ':' .. math.random())
    redis.call('EXPIRE', key, window)
    return 1
end
return 0
"""


async def allow(user_id: str, limit: int = 100, window: int = 60) -> bool:
    """True nếu request được phép. Nguyên tử ngay cả khi có 8 worker."""
    script = r.register_script(SLIDING_WINDOW)
    return bool(await script(keys=[f"rl:{user_id}"], args=[time.time(), window, limit]))
```

**Vì sao sliding window chứ không phải fixed window?** Fixed window cho phép
"burst ở mép": 100 request lúc 0:59 và 100 request nữa lúc 1:00 — 200 request
trong 2 giây, dù giới hạn là 100/phút. Sliding window loại bỏ lỗ hổng đó.

**Vì sao Lua?** `ZCARD` rồi `ZADD` ở phía Python là hai chuyến đi mạng riêng
biệt. Hai worker có thể cùng đọc `count = 99` rồi cùng cho phép. Lua chạy trọn
vẹn bên trong Redis nên nguyên tử.

### C. Chat System

**Hỏi trước:** 1-1 hay group? Có cần lịch sử tin nhắn không? Có cần "đã xem"?
Online presence?

Bộ khung tối thiểu:

```
Client ──WebSocket──> Gateway (FastAPI + `websockets`)
                          │
                          ├─> Redis Pub/Sub  (fan-out giữa các instance gateway)
                          └─> PostgreSQL     (lưu tin nhắn, phân vùng theo tháng)
```

Điểm quyết định: **gateway phải stateless**. Kết nối WebSocket của user A có thể
nằm trên instance 1 trong khi user B ở instance 2 — Redis Pub/Sub là thứ nối hai
instance đó lại. Nếu gateway giữ state trong RAM, bạn không scale ngang được.

---

## 🏋️ Bài tập

**Bài 1 (5 phút) — Nhẩm dung lượng.**
Một hệ thống logging nhận 50.000 sự kiện/giây, mỗi sự kiện 500 byte. Tính dung
lượng thô mỗi ngày. Nếu giữ 30 ngày thì cần bao nhiêu? Con số đó có gợi ý gì về
việc chọn nơi lưu trữ?

**Bài 2 (10 phút) — Sửa cấu hình sai.**
Một team chạy FastAPI với `--workers 40` trên máy 4 core, mỗi worker có
SQLAlchemy pool `pool_size=20`. PostgreSQL đặt `max_connections=100`. Chỉ ra hai
vấn đề và đề xuất con số đúng.

**Bài 3 (15 phút) — Mở rộng rate limiter.**
Sửa `allow()` để trả về `tuple[bool, int]` gồm cả `retry_after` (số giây tới khi
request cũ nhất rời khỏi cửa sổ), để API có thể trả header `Retry-After` chuẩn.

<details>
<summary>Gợi ý bài 3</summary>

Dùng `ZRANGE key 0 0 WITHSCORES` để lấy timestamp của entry cũ nhất, rồi tính
`retry_after = ceil(oldest + window - now)`. Nhớ làm luôn trong script Lua —
nếu quay lại Python để hỏi thì lại mở ra race condition đúng như lý do dùng Lua.
</details>

---

## 🔧 Troubleshooting

| Triệu chứng | Nguyên nhân thường gặp | Cách xử lý |
|---|---|---|
| `TimeoutError: QueuePool limit reached` | `workers × pool_size` > `max_connections` | Giảm worker hoặc pool; cân nhắc PgBouncer |
| Latency p99 tăng vọt còn p50 bình thường | Cache stampede trên key hot | Thêm jitter TTL, hoặc lock khi làm mới cache |
| Endpoint async nhưng vẫn chậm | Có lời gọi blocking (`requests`, `time.sleep`) trong hàm `async` | Đổi sang `httpx.AsyncClient`/`asyncio.sleep`, hoặc bọc `run_in_executor` |
| Container restart liên tục khi DB sập | `/healthz` có kiểm tra database | Chuyển kiểm tra dependency sang `/readyz` |
| `@lru_cache` trả dữ liệu cũ sau khi deploy | Cache sống theo đời process, mỗi worker một bản | Chỉ dùng cho dữ liệu bất biến; dữ liệu động để ở Redis |

---

## Tóm tắt

- **Nhẩm trước, vẽ sau.** QPS và dung lượng quyết định kiến trúc; thường chính
  con số *storage* mới là thứ ép bạn đổi thiết kế.
- **GIL định hình mô hình deploy của Python.** I/O-bound → worker ≈ số core;
  CPU-bound → `2n + 1`. Luôn nhân số worker với pool size trước khi chốt.
- **Cache có bốn tầng**, mỗi tầng invalidate khác nhau. Ghi DB rồi **xoá** cache,
  đừng ghi đè. TTL là lưới an toàn cho những lần bạn quên.
- **REST là mặc định**, gRPC cho nội bộ khi đã đo được, GraphQL khi thật sự có
  nhiều loại client.
- **Modular Monolith trước, microservices sau** — và chỉ khi có lý do đo được.
  Ranh giới module phải được ép bởi linter, không phải bởi lời hứa.

## Tiếp theo

Bạn đã có tư duy thiết kế cho hệ thống phần mềm truyền thống. Nhưng hệ thống AI
có những ràng buộc rất khác: một request có thể chạy 30 giây, GPU đắt gấp trăm
lần CPU, và model thì không tất định. Mời bạn đến với
**[Chapter 39 — AI System Design & Infrastructure](chapter_39_ai_system_design.md)**.
