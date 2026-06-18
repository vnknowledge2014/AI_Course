# Chapter 36 — Application Security

> **Bạn sẽ học được**:
> - CORS configuration
> - SQL injection prevention
> - Rate limiting
> - HTTPS, headers, CSP
>
> **Yêu cầu trước**: Chapter 35 (Security)
> **Thời gian đọc**: ~25 phút | **Level**: Principal

---

## 36.1 — SQL Injection Prevention

```python
# ❌ NEVER: string concatenation for SQL
def bad_query(user_input: str):
    query = f"SELECT * FROM users WHERE name = '{user_input}'"
    # Attacker: user_input = "'; DROP TABLE users; --"
    # → SELECT * FROM users WHERE name = ''; DROP TABLE users; --'

# ✅ ALWAYS: parameterized queries
from sqlalchemy import text

def safe_query(engine, user_input: str):
    with engine.connect() as conn:
        result = conn.execute(
            text("SELECT * FROM users WHERE name = :name"),
            {"name": user_input}  # Parameter binding — safe!
        )
        return result.fetchall()
```

## 36.2 — CORS in FastAPI

```python
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["https://myapp.com"],  # NOT "*" in production!
    allow_credentials=True,
    allow_methods=["GET", "POST"],
    allow_headers=["Authorization", "Content-Type"],
)
```

## 36.3 — Rate Limiting

```python
import time
from collections import defaultdict

class RateLimiter:
    def __init__(self, max_requests: int, window_seconds: int):
        self.max_requests = max_requests
        self.window = window_seconds
        self._requests: defaultdict[str, list[float]] = defaultdict(list)

    def is_allowed(self, client_id: str) -> bool:
        now = time.time()
        window_start = now - self.window
        self._requests[client_id] = [
            t for t in self._requests[client_id] if t > window_start
        ]
        if len(self._requests[client_id]) >= self.max_requests:
            return False
        self._requests[client_id].append(now)
        return True

limiter = RateLimiter(max_requests=5, window_seconds=60)
for _ in range(5):
    assert limiter.is_allowed("user-1") == True
assert limiter.is_allowed("user-1") == False  # 6th request blocked
```

## 36.4 — Security Headers

```python
from fastapi import FastAPI, Request, Response
from fastapi.middleware.trustedhost import TrustedHostMiddleware

app = FastAPI()

@app.middleware("http")
async def add_security_headers(request: Request, call_next):
    response: Response = await call_next(request)
    response.headers["X-Content-Type-Options"] = "nosniff"
    response.headers["X-Frame-Options"] = "DENY"
    response.headers["Strict-Transport-Security"] = "max-age=31536000"
    response.headers["Content-Security-Policy"] = "default-src 'self'"
    return response
```

---

## Tóm tắt

- ✅ **SQL injection**: Always use parameterized queries.
- ✅ **CORS**: Whitelist specific origins, not `*`.
- ✅ **Rate limiting**: Sliding window per client.
- ✅ **Headers**: CSP, HSTS, X-Frame-Options.

## Tiếp theo

→ Chapter 37: **Distributed Systems** — Microservices, message queues.
