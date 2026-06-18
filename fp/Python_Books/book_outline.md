# 🐍 Domain-Driven FP with Python — Combined Book Outline

> Kết hợp: DDD Functional + FP Made Easier + Learn Go with Tests + F# Fun & Profit
> Coverage: **~60%** FP/DDD · Full CS foundations included
> Approach: Foundations → Beginner → Intermediate → Advanced → Principal
> Tổng: **39 chapters** + Appendices

---

## Part 0: CS Foundations (Pre-requisite)

### Chapter 1 — Math Foundations for FP
Lambda Calculus: `lambda x: x+1`. Curry-Howard (relaxed). Discrete Math. Algebraic type sizes: Union = sum, dataclass = product.

### Chapter 2 — Algorithmic Thinking & Complexity
Big-O: `list.append` O(1), `list.insert(0)` O(n), `dict` O(1). Recursion limit (`sys.setrecursionlimit`). `@lru_cache`. Sorting: Timsort.

### Chapter 3 — Functional Data Structures
Immutable: `tuple`, `frozenset`, `frozen=True`. `MappingProxyType`. `pyrsistent`: PVector, PMap. Graph: `dict[str, set[str]]`.

---

## Part I: Python Fundamentals (Beginner)

### Chapter 4 — Getting Started
Python 3.12+, `pyenv`, `poetry`/`uv`. VS Code + Pylance. Type hints intro.

### Chapter 5 — Values, Types & Type Hints
Primitives. `mypy --strict`. `Final`, `Literal`, `TypeAlias`.

### Chapter 6 — Control Flow & Pattern Matching
`if/elif/else`, `for`, `while`. `match` (3.10+). Guards. Destructuring.

### Chapter 7 — Functions & Closures
`def`, `lambda`, `functools.partial`, `toolz.curry`. HOFs.

### Chapter 8 — Data Structures
`list`, `tuple`, `dict`, `set`. Comprehensions. `NamedTuple`.

### Chapter 9 — Dataclasses & Structured Data ⭐
`@dataclass`, `frozen=True`. `__post_init__`. = F# records / Rust structs.

### Chapter 10 — Modules & Packages
`import`. `__init__.py`. `pyproject.toml`.

---

## Part II: Thinking Functionally (Intermediate)

### Chapter 11 — Immutability & Purity
`frozen=True`, `tuple`, `MappingProxyType`, `Final`. Convention-based.

### Chapter 12 — Composition & Pipelines
`toolz.pipe(value, f, g, h)`. `returns.pipeline.pipe`. Comprehensions.

### Chapter 13 — ADTs in Python
Sum types via `@dataclass` subclasses + `Shape = Circle | Rectangle`. `match`. `assert_never`.

### Chapter 14 — Validation with Pydantic ⭐
`BaseModel`: auto-validation. `@validator`. `Field()`. = smart constructors + serialization.

### Chapter 15 — Protocols — Structural Typing
`typing.Protocol` (PEP 544). ≈ Rust traits / OCaml module types.

---

## Part III: Design Patterns — FP & Classical (Advanced)

### Chapter 16 — GoF → FP Translation
Strategy = HOF. Command = dataclass. Visitor = `match`. Factory = classmethod. Decorator = `@decorator` (Python native!). Middleware = ASGI.

### Chapter 17 — CQRS & Event Sourcing
Tách read/write. Event Sourcing: `functools.reduce` rebuild. Projections.

---

## Part IV: Domain-Driven Design (Advanced)

### Chapter 18 — Introduction to DDD
Ubiquitous Language, Bounded Contexts, Event Storming.

### Chapter 19 — Functional Architecture
Layered. IO at edges — `returns.IO` helps.

### Chapter 20 — Domain Modeling
Frozen dataclasses = value objects. `match` for states.

### Chapter 21 — Workflows as Pipelines
`returns.pipeline.flow(validate, price, acknowledge)`.

### Chapter 22 — Error Handling with `returns` ⭐
`Result[Success, Failure]`. `.bind()`, `.map()`. `@safe`. `RequiresContext` for DI.

### Chapter 23 — Serialization & DTOs
Pydantic `BaseModel`. `.model_dump()`, `.model_validate_json()`.

### Chapter 24 — Persistence & Repository
`Protocol`-based repos. SQLAlchemy, SQLModel. DI via `RequiresContext`.

---

## Part V: FP Patterns with `returns` (Advanced)

### Chapter 25 — The `returns` Ecosystem
`Maybe`, `Result`, `IO`, `IOResult`, `FutureResult`. `@safe`, `@impure_safe`.

### Chapter 26 — Functors & Monads
`.map()` = Functor. `.bind()` = Monad. Concrete per type (no HKTs).

### Chapter 27 — Applicative & Validation
`ResultE` for collecting errors. Pydantic validators as alternative.

### Chapter 28 — Monadic Stacking
`IOResult`, `FutureResult`, `RequiresContextIOResult`. Pre-built stacks.

---

## Part VI: Testing & Web (Principal)

### Chapter 29 — TDD with pytest
`pytest`, fixtures, parametrize, `pytest-mock`. Red → Green → Refactor.

### Chapter 30 — Property-Based Testing
`hypothesis`. Strategies, `@given`, shrinking. Stateful testing.

### Chapter 31 — FastAPI + DDD ⭐
FastAPI = Pydantic + async + OpenAPI auto-docs. DDD integration.

### Chapter 32 — Capstone Part 1: Domain Model ⭐
Frozen dataclasses domain, `returns.Result` pipelines, Pydantic DTOs, CQRS.

---

## Part VII: Production Engineering (Principal)

> *Database, Security, Distributed Systems, System Design — self-contained*

### Chapter 33 — Database Fundamentals & SQL
**Relational model**: tables, keys. **SQL**: `SELECT`, `JOIN`, `GROUP BY`, CTEs. **Normalization** 1NF→BCNF. **Indexing**: B-Tree, composite, `EXPLAIN`. **Transactions**: ACID, isolation levels. **Python**: SQLAlchemy (ORM + Core), SQLModel (FastAPI integration), Alembic (migrations). `asyncpg` for async.

### Chapter 34 — Advanced Data Patterns
**Migrations**: Alembic, zero-downtime strategies. **CQRS persistence**: read/write separation. **Event Store**. **NoSQL**: MongoDB (`motor`), Redis (`redis-py`/`aioredis`), DynamoDB (`boto3`). **Caching**: `cachetools`, Redis patterns, `fastapi-cache`.

### Chapter 35 — Security Essentials
**Auth**: Password hashing (`passlib` + `argon2`/`bcrypt`). **Sessions vs Tokens**: JWT (`python-jose`/`PyJWT`), refresh tokens. **OAuth 2.0**: `authlib`, PKCE. **Authorization**: RBAC, ABAC, FastAPI dependencies as guards. **Python**: `python-multipart`, `itsdangerous`.

### Chapter 36 — Application Security & Hardening
**OWASP Top 10**: SQL injection (parameterized queries — SQLAlchemy auto-escapes), XSS (templating auto-escape), CSRF, SSRF. **Pydantic validation = defense in depth**. **CORS**: `fastapi.middleware.cors`. **Headers**: `secure` library. **Secrets**: `python-dotenv`, `pydantic-settings`. **Rate limiting**: `slowapi`. **Audit logging**: structured `structlog`.

### Chapter 37 — Distributed Systems Fundamentals
**CAP Theorem**: CP vs AP. **Consistency models**. **Replication**: leader-follower, leaderless. CRDTs. **Partitioning**: consistent hashing. **Consensus**: Raft. **Message Queues**: Celery (Redis/RabbitMQ), `dramatiq`, `arq` (async). Kafka (`aiokafka`). **Patterns**: Saga, Circuit Breaker (`pybreaker`), Retry (`tenacity`), Outbox. **Observability**: `structlog`, Prometheus (`prometheus-client`), OpenTelemetry.

### Chapter 38 — System Design Thinking
**Capacity estimation**. **Load balancing**: Nginx, Gunicorn workers. **Caching**: CDN, Redis, `@lru_cache`. **API design**: REST (FastAPI) vs gRPC (`grpcio`) vs GraphQL (`strawberry`). **Microservices**: monolith-first. **ASGI servers**: Uvicorn, Hypercorn. **Serverless**: AWS Lambda, Vercel. **Design exercises**: URL shortener, chat, rate limiter.

### Chapter 39 — Capstone Part 2: Production Deployment ⭐
FastAPI + PostgreSQL (SQLAlchemy), Redis cache, JWT auth, CORS, rate limiting, `structlog`, Docker + docker-compose, CI/CD (GitHub Actions), Fly.io/Railway deployment.

---

## Appendices
### A — `mypy --strict` Configuration
### B — From F#/TypeScript to Python Translation Table
### C — `returns` vs `fp-ts` vs `Result` Comparison
