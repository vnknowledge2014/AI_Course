# Chapter 34 — Advanced Data: Caching & Background Tasks

> **Bạn sẽ học được**:
> - Redis caching patterns
> - Background tasks with Celery / asyncio
> - Rate limiting
> - Data pipelines
>
> **Yêu cầu trước**: Chapter 33 (Database)
> **Thời gian đọc**: ~25 phút | **Level**: Principal

---

## 34.1 — Caching Pattern

```python
from typing import Protocol, TypeVar, Callable
from dataclasses import dataclass

T = TypeVar("T")

class Cache(Protocol):
    def get(self, key: str) -> object | None: ...
    def set(self, key: str, value: object, ttl: int = 300) -> None: ...

class InMemoryCache:
    def __init__(self):
        self._store: dict[str, object] = {}
    def get(self, key: str) -> object | None:
        return self._store.get(key)
    def set(self, key: str, value: object, ttl: int = 300) -> None:
        self._store[key] = value

def cached(cache: Cache, key_fn: Callable, ttl: int = 300):
    def decorator(fn):
        def wrapper(*args, **kwargs):
            key = key_fn(*args, **kwargs)
            result = cache.get(key)
            if result is not None:
                return result
            result = fn(*args, **kwargs)
            cache.set(key, result, ttl)
            return result
        return wrapper
    return decorator

# Usage
cache = InMemoryCache()

@cached(cache, lambda order_id: f"order:{order_id}")
def get_order(order_id: str) -> dict:
    print(f"DB query for {order_id}")  # Only called once!
    return {"id": order_id, "total": 100_000}

r1 = get_order("ORD-1")  # DB query
r2 = get_order("ORD-1")  # From cache!
assert r1 == r2
```

## 34.2 — Background Tasks

```python
import asyncio
from typing import Callable

# Simple async task queue
class TaskQueue:
    def __init__(self):
        self._tasks: list[Callable] = []

    def enqueue(self, task: Callable) -> None:
        self._tasks.append(task)

    async def process_all(self) -> None:
        for task in self._tasks:
            if asyncio.iscoroutinefunction(task):
                await task()
            else:
                task()
        self._tasks.clear()

# Usage
queue = TaskQueue()
results = []

queue.enqueue(lambda: results.append("email sent"))
queue.enqueue(lambda: results.append("log written"))
asyncio.run(queue.process_all())
assert results == ["email sent", "log written"]
```

---

## Tóm tắt

- ✅ **Caching**: Cache Protocol + decorator pattern.
- ✅ **Background tasks**: Async queue or Celery for production.
- ✅ **Rate limiting**: Token bucket or sliding window.

## Tiếp theo

→ Chapter 35: **Security** — Authentication, authorization, hashing.
