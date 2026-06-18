# Chapter 17 — CQRS & Event Sourcing

> **Bạn sẽ học được**:
> - CQRS: Command-Query Responsibility Segregation
> - Event Sourcing: lưu events thay vì state
> - `functools.reduce` rebuild state từ events
> - Projections, snapshots
>
> **Yêu cầu trước**: Chapter 16 (Design Patterns)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 17.1 — CQRS

```python
from dataclasses import dataclass
from typing import Protocol

# Tách read (Query) và write (Command)

# Command side — write
class CommandHandler(Protocol):
    def handle(self, command: object) -> None: ...

# Query side — read
class QueryHandler(Protocol):
    def handle(self, query: object) -> object: ...

# Trong thực tế:
# - Commands: CreateOrder, CancelOrder, UpdateStock
# - Queries: GetOrderById, ListOrders, SearchProducts
# - Write model: normalized, optimized for consistency
# - Read model: denormalized, optimized for read speed
```

## 17.2 — Event Sourcing

```python
from dataclasses import dataclass
from typing import Union
from functools import reduce

# Events — things that HAPPENED (past tense, immutable)
@dataclass(frozen=True)
class AccountCreated:
    owner: str
    initial_balance: int

@dataclass(frozen=True)
class MoneyDeposited:
    amount: int

@dataclass(frozen=True)
class MoneyWithdrawn:
    amount: int

Event = Union[AccountCreated, MoneyDeposited, MoneyWithdrawn]

# State — derived from events
@dataclass(frozen=True)
class AccountState:
    owner: str
    balance: int
    is_active: bool

INITIAL_STATE = AccountState(owner="", balance=0, is_active=False)

# Reducer — apply event to state
def apply_event(state: AccountState, event: Event) -> AccountState:
    match event:
        case AccountCreated(owner=o, initial_balance=b):
            return AccountState(owner=o, balance=b, is_active=True)
        case MoneyDeposited(amount=a):
            return AccountState(
                owner=state.owner,
                balance=state.balance + a,
                is_active=state.is_active,
            )
        case MoneyWithdrawn(amount=a):
            return AccountState(
                owner=state.owner,
                balance=state.balance - a,
                is_active=state.is_active,
            )

# Event stream — complete history
events: list[Event] = [
    AccountCreated("An", 100_000),
    MoneyDeposited(50_000),
    MoneyWithdrawn(30_000),
    MoneyDeposited(20_000),
]

# Rebuild state from events using reduce (fold)
current_state = reduce(apply_event, events, INITIAL_STATE)
assert current_state.owner == "An"
assert current_state.balance == 140_000  # 100k + 50k - 30k + 20k
assert current_state.is_active == True

print(f"Account: {current_state}")

# === Time travel! Replay events to any point ===
state_after_2_events = reduce(apply_event, events[:2], INITIAL_STATE)
assert state_after_2_events.balance == 150_000  # 100k + 50k

# === Projection: different view from same events ===
def transaction_count(events: list[Event]) -> int:
    return sum(1 for e in events if not isinstance(e, AccountCreated))

assert transaction_count(events) == 3  # 3 transactions (deposit, withdraw, deposit)
```

## 17.3 — When to Use

```python
# ✅ Good fit for Event Sourcing:
# - Financial systems (audit trail required)
# - Order systems (track state changes)
# - Collaboration tools (undo/redo)
# - Systems requiring temporal queries

# ❌ Bad fit:
# - Simple CRUD (overkill)
# - High-frequency updates (event log grows fast)
# - Systems without audit requirements
```

---

## ✅ Checkpoint 17

> 1. CQRS = separate read (query) and write (command) models
> 2. Event Sourcing = store events, not state. Rebuild with `reduce`.
> 3. Benefits: audit trail, time travel, different projections
> 4. Trade-off: complexity, eventual consistency

---

## Tóm tắt

- ✅ **CQRS**: Tách read/write — optimize independently.
- ✅ **Event Sourcing**: Events = source of truth. State = derived.
- ✅ **`reduce`**: Fold events into current state.
- ✅ **Time travel**: Replay events to any point in time.
- ✅ **Projections**: Different views from same event stream.

## Tiếp theo

→ Chapter 18: **Introduction to DDD** — Ubiquitous Language, Bounded Contexts, Event Storming.
