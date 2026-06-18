# Chapter 31 — FastAPI & Async

> **Bạn sẽ học được**:
> - FastAPI basics: routes, Pydantic request/response
> - `async/await` — non-blocking IO
> - Dependency injection in FastAPI
> - Connecting domain logic to HTTP
>
> **Yêu cầu trước**: Chapter 30 (PBT)
> **Thời gian đọc**: ~35 phút | **Level**: Principal

---

## 31.1 — FastAPI Basics

```python
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel, Field

app = FastAPI(title="Cafe Order API")

# Request/Response DTOs
class CreateOrderRequest(BaseModel):
    customer_email: str
    items: list[dict[str, int | str]]

class OrderResponse(BaseModel):
    id: str
    customer_email: str
    total: int
    status: str

# In-memory store
orders: dict[str, OrderResponse] = {}

@app.post("/orders", response_model=OrderResponse)
async def create_order(request: CreateOrderRequest) -> OrderResponse:
    import uuid
    order_id = str(uuid.uuid4())[:8]
    total = sum(item.get("price", 0) for item in request.items)
    order = OrderResponse(
        id=order_id,
        customer_email=request.customer_email,
        total=total,
        status="pending",
    )
    orders[order_id] = order
    return order

@app.get("/orders/{order_id}", response_model=OrderResponse)
async def get_order(order_id: str) -> OrderResponse:
    if order_id not in orders:
        raise HTTPException(status_code=404, detail="Order not found")
    return orders[order_id]
```

## 31.2 — Async/Await

```python
import asyncio

async def fetch_price(product: str) -> int:
    await asyncio.sleep(0.1)  # Simulate IO
    prices = {"Coffee": 35_000, "Tea": 25_000, "Cake": 50_000}
    return prices.get(product, 0)

async def total_price(products: list[str]) -> int:
    # Run ALL fetches concurrently!
    prices = await asyncio.gather(*[fetch_price(p) for p in products])
    return sum(prices)

# asyncio.run(total_price(["Coffee", "Tea", "Cake"]))
# → 110_000 (but runs in ~0.1s, not 0.3s!)
```

## 31.3 — Dependency Injection

```python
from fastapi import Depends
from typing import Protocol

class OrderRepo(Protocol):
    def find_by_id(self, order_id: str) -> dict | None: ...
    def save(self, order: dict) -> None: ...

class InMemoryRepo:
    def __init__(self):
        self._store = {}
    def find_by_id(self, order_id: str):
        return self._store.get(order_id)
    def save(self, order: dict):
        self._store[order["id"]] = order

repo = InMemoryRepo()

def get_repo() -> OrderRepo:
    return repo

@app.get("/orders-v2/{order_id}")
async def get_order_v2(order_id: str, repo: OrderRepo = Depends(get_repo)):
    order = repo.find_by_id(order_id)
    if order is None:
        raise HTTPException(404, "Not found")
    return order
```

## 31.4 — Testing FastAPI

```python
from fastapi.testclient import TestClient

client = TestClient(app)

def test_create_order():
    response = client.post("/orders", json={
        "customer_email": "an@mail.com",
        "items": [{"product": "Coffee", "price": 35_000}],
    })
    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "pending"
    assert data["total"] == 35_000
```

---

## Tóm tắt

- ✅ **FastAPI**: Modern Python web framework — Pydantic + async.
- ✅ **async/await**: Non-blocking IO, `asyncio.gather` for concurrency.
- ✅ **DI**: `Depends()` — inject repos, services.
- ✅ **Testing**: `TestClient` for integration tests.

## Tiếp theo

→ Chapter 32: **Capstone: Domain Application** — Full DDD application.
