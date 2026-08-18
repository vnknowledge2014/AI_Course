# 🐍 Domain-Driven FP with Python — Combined Book Outline

> Kết hợp: DDD Functional + FP Made Easier + Learn Go with Tests + F# Fun & Profit
> Coverage: **~60%** FP/DDD · Full CS foundations included
> Approach: Foundations → Beginner → Intermediate → Advanced → Principal
> Tổng: **46 chương** (Ch1–Ch40, gồm các chương phụ 3B/3C/27b/28b/28c/37B) + **3 phụ lục**

---

## Part 0: CS Foundations (Lý thuyết Khoa học Máy tính)

> ⚠️ **LƯU Ý DÀNH CHO NGƯỜI MỚI (BEGINNERS)**:
> Phần này chứa nhiều lý thuyết và tư duy hệ thống. Nếu bạn không có nền tảng toán học hoặc mới học lập trình, việc đọc phần này có thể gây "ngợp".
> **HÃY BỎ QUA PART 0 VÀ NHẢY THẲNG ĐẾN PART 1 (PYTHON FUNDAMENTALS)** để bắt tay vào cài đặt và code thực hành ngay. Bạn có thể quay lại đọc Part 0 sau khi đã quen với Python!

### [Chapter 1 — Math Foundations for FP](part_0_cs_foundations/chapter_01_math_foundations.md) ✅
Lambda Calculus: `lambda x: x+1`. Curry-Howard (relaxed). Discrete Math. Algebraic type sizes: Union = sum, dataclass = product.

### [Chapter 2 — Algorithmic Thinking & Complexity](part_0_cs_foundations/chapter_02_algorithmic_thinking.md) ✅
Big-O: `list.append` O(1), `list.insert(0)` O(n), `dict` O(1). Recursion limit (`sys.setrecursionlimit`). `@lru_cache`. Sorting: Timsort.

### [Chapter 3 — Functional Data Structures](part_0_cs_foundations/chapter_03_functional_data_structures.md) ✅
Immutable: `tuple`, `frozenset`, `frozen=True`. `MappingProxyType`. `pyrsistent`: PVector, PMap. Graph: `dict[str, set[str]]`. Structural sharing và Big-O của persistent data structures.

### [Chapter 3B — Advanced Algorithms & LeetCode Patterns](part_0_cs_foundations/chapter_03b_leetcode_patterns.md) ✅
Two Pointers, Sliding Window, DP, Graph Traversal. Functional approach to algorithms.

### [Chapter 3C — Hardware, Compute: LeetCPU & LeetGPU](part_0_cs_foundations/chapter_03c_hardware_leetgpu.md) ✅
Memory Hierarchy, CPU vs GPU, Tensor Cores, Memory Bandwidth vs Compute Bound.

---

## Part I: Python Fundamentals (Beginner)

### [Chapter 4 — The Python Ecosystem & Tooling](part_1_fundamentals/chapter_04_getting_started.md) ✅
`uv` (Package manager siêu tốc), `pyproject.toml`, Type Hinting Ecosystem (`mypy`/`pyright`), `ruff` (Linter & Formatter). VS Code + Pylance setup.

### [Chapter 5 — Values, Types & Type Hints](part_1_fundamentals/chapter_05_values_types.md) ✅
Primitives. `mypy --strict`. `Final`, `Literal`, `TypeAlias`.

### [Chapter 6 — Control Flow & Pattern Matching](part_1_fundamentals/chapter_06_control_flow.md) ✅
`if/elif/else`, `for`, `while`. `match` (3.10+). Guards. Destructuring.

### [Chapter 7 — Functions & Closures](part_1_fundamentals/chapter_07_functions_closures.md) ✅
`def`, `lambda`, `functools.partial`, `toolz.curry`. HOFs.

### [Chapter 8 — Data Structures](part_1_fundamentals/chapter_08_data_structures.md) ✅
`list`, `tuple`, `dict`, `set`. Comprehensions. `NamedTuple`.

### [Chapter 9 — Dataclasses & Structured Data ⭐](part_1_fundamentals/chapter_09_dataclasses.md) ✅
`@dataclass`, `frozen=True`. `__post_init__`. = F# records / Rust structs.

### [Chapter 10 — Modules & Packages](part_1_fundamentals/chapter_10_modules_packages.md) ✅
`import`. `__init__.py`. `pyproject.toml`.

---

## Part II: Thinking Functionally (Intermediate)

### [Chapter 11 — Immutability & Purity](part_2_thinking_functionally/chapter_11_immutability_purity.md) ✅
`frozen=True`, `tuple`, `MappingProxyType`, `Final`. Convention-based.

### [Chapter 12 — Composition & Pipelines](part_2_thinking_functionally/chapter_12_composition_pipelines.md) ✅
`toolz.pipe(value, f, g, h)`. `returns.pipeline.pipe`. Comprehensions.

### [Chapter 13 — ADTs in Python](part_2_thinking_functionally/chapter_13_adts.md) ✅
Sum types via `@dataclass` subclasses + `Shape = Circle | Rectangle`. `match`. `assert_never`.

### [Chapter 14 — Validation with Pydantic ⭐](part_2_thinking_functionally/chapter_14_pydantic_validation.md) ✅
`BaseModel`: auto-validation. `@validator`. `Field()`. = smart constructors + serialization.

### [Chapter 15 — Protocols — Structural Typing](part_2_thinking_functionally/chapter_15_protocols.md) ✅
`typing.Protocol` (PEP 544). ≈ Rust traits / OCaml module types.

---

## Part III: Design Patterns — FP & Classical (Advanced)

### [Chapter 16 — GoF → FP Translation](part_3_design_patterns/chapter_16_gof_fp.md) ✅
Strategy = HOF. Command = dataclass. Visitor = `match`. Factory = classmethod. Decorator = `@decorator` (Python native!). Middleware = ASGI.

### [Chapter 17 — CQRS & Event Sourcing](part_3_design_patterns/chapter_17_cqrs_event_sourcing.md) ✅
Tách read/write. Event Sourcing: `functools.reduce` rebuild. Projections.

---

## Part IV: Domain-Driven Design (Advanced)

### [Chapter 18 — Introduction to DDD](part_4_ddd/chapter_18_intro_ddd.md) ✅
Ubiquitous Language, Bounded Contexts, Event Storming.

### [Chapter 19 — Functional Architecture](part_4_ddd/chapter_19_functional_architecture.md) ✅
Layered. IO at edges — `returns.IO` helps.

### [Chapter 20 — Domain Modeling](part_4_ddd/chapter_20_domain_modeling.md) ✅
Frozen dataclasses = value objects. `match` for states.

### [Chapter 21 — Workflows as Pipelines](part_4_ddd/chapter_21_workflows_pipelines.md) ✅
`returns.pipeline.flow(validate, price, acknowledge)`.

### [Chapter 22 — Error Handling with `returns` ⭐](part_4_ddd/chapter_22_rop.md) ✅
`Result[Success, Failure]`. `.bind()`, `.map()`. `@safe`. `RequiresContext` for DI.

### [Chapter 23 — Serialization & DTOs](part_4_ddd/chapter_23_serialization_acl.md) ✅
Pydantic `BaseModel`. `.model_dump()`, `.model_validate_json()`.

### [Chapter 24 — Persistence & Repository](part_4_ddd/chapter_24_persistence.md) ✅
`Protocol`-based repos. SQLAlchemy, SQLModel. DI via `RequiresContext`.

---

## Part V: FP Patterns (Advanced)

### [Chapter 25 — Abstract Algebra & Type Classes](part_5_fp_patterns/chapter_25_abstract_algebra.md) ✅
`Eq`, `Ord`, `Semigroup`, `Monoid`. Protocols for structural comparison and merging. MapReduce parallel fold.

### [Chapter 26 — Functors](part_5_fp_patterns/chapter_26_functors.md) ✅
Mappable containers. Functor Laws. The Nesting Problem. Protocol-based Functors (`Box`, `Validated`, `Tree`).

### [Chapter 27 — Monads](part_5_fp_patterns/chapter_27_monads.md) ✅
Solving the Nesting Problem with `bind`/`flatMap`. Monad Laws. Generator-based do-notation (`do()` helper).

### [Chapter 27b — Applicative & Validation](part_5_fp_patterns/chapter_27b_applicative_validation.md) ✅
Fail-fast vs Collect-all. `Validated` type. `validate_all` combinator. `ap` operation.

### [Chapter 28 — Parser Combinators](part_5_fp_patterns/chapter_28_parser_combinators.md) ✅
Recursive parsing. Combinators: `then`, `or_else`, `many`, `between`. `bind_parser` (Parser as Monad).

### [Chapter 28b — Traverse & Sequence](part_5_fp_patterns/chapter_28b_traverse_sequence.md) ✅
Inverting structures (`list[Result]` → `Result[list]`). Applicative Traverse for collecting all errors in a list.

### [Chapter 28c — Recursive Types & Folds](part_5_fp_patterns/chapter_28c_recursive_types_folds.md) ✅
Catamorphisms. Separating traversal from computation. Tree fold and AST Evaluator.

---

## Part VI: Testing & Web (Principal)

### [Chapter 29 — TDD with pytest](part_6_testing_web/chapter_29_tdd_pytest.md) ✅
`pytest`, fixtures, parametrize, `pytest-mock`. Red → Green → Refactor.

### [Chapter 30 — Property-Based Testing](part_6_testing_web/chapter_30_property_testing.md) ✅
`hypothesis`. Strategies, `@given`, shrinking. Stateful testing.

### [Chapter 31 — FastAPI + DDD ⭐](part_6_testing_web/chapter_31_fastapi_async.md) ✅
FastAPI = Pydantic + async + OpenAPI auto-docs. DDD integration.

### [Chapter 32 — Capstone Part 1: Domain Model ⭐](part_6_testing_web/chapter_32_capstone_domain.md) ✅
Frozen dataclasses domain, `returns.Result` pipelines, Pydantic DTOs, CQRS.

---

## Part VII: Production Engineering (Principal)

> *Database, Security, Distributed Systems, System Design — self-contained*

### [Chapter 33 — Database Fundamentals & SQL](part_7_production/chapter_33_database_sql.md) ✅
**Relational model**: tables, keys. **SQL**: `SELECT`, `JOIN`, `GROUP BY`, CTEs. **Normalization** 1NF→BCNF. **Indexing**: B-Tree, composite, `EXPLAIN`. **Transactions**: ACID, isolation levels. **Python**: SQLAlchemy (ORM + Core), SQLModel (FastAPI integration), Alembic (migrations). `asyncpg` for async.

### [Chapter 34 — Advanced Data Patterns](part_7_production/chapter_34_advanced_data.md) ✅
**Migrations**: Alembic, zero-downtime strategies. **CQRS persistence**: read/write separation. **Event Store**. **NoSQL**: MongoDB (`motor`), Redis (`redis-py`/`aioredis`), DynamoDB (`boto3`). **Caching**: `cachetools`, Redis patterns, `fastapi-cache`.

### [Chapter 35 — Security Essentials](part_7_production/chapter_35_security.md) ✅
**Auth**: Password hashing (`passlib` + `argon2`/`bcrypt`). **Sessions vs Tokens**: JWT (`python-jose`/`PyJWT`), refresh tokens. **OAuth 2.0**: `authlib`, PKCE. **Authorization**: RBAC, ABAC, FastAPI dependencies as guards. **Python**: `python-multipart`, `itsdangerous`.

### [Chapter 36 — Application Security & Hardening](part_7_production/chapter_36_app_security.md) ✅
**OWASP Top 10**: SQL injection (parameterized queries — SQLAlchemy auto-escapes), XSS (templating auto-escape), CSRF, SSRF. **Pydantic validation = defense in depth**. **CORS**: `fastapi.middleware.cors`. **Headers**: `secure` library. **Secrets**: `python-dotenv`, `pydantic-settings`. **Rate limiting**: `slowapi`. **Audit logging**: structured `structlog`.

### [Chapter 37 — Distributed Systems Fundamentals](part_7_production/chapter_37_distributed_systems.md) ✅
**CAP Theorem**: CP vs AP. **Consistency models**. **Replication**: leader-follower, leaderless. CRDTs. **Partitioning**: consistent hashing. **Consensus**: Raft. **Message Queues**: Celery (Redis/RabbitMQ), `dramatiq`, `arq` (async). Kafka (`aiokafka`). **Patterns**: Saga, Circuit Breaker (`pybreaker`), Retry (`tenacity`), Outbox.

### [Chapter 37B — Observability](part_7_production/chapter_37b_observability.md) ✅
Monitoring vs Observability. Ba trụ cột: **Logs** (`structlog` — structured logging), **Metrics** (`prometheus-client`), **Distributed Tracing** (OpenTelemetry).

### [Chapter 38 — System Design Thinking](part_7_production/chapter_38_system_design.md) ✅
**Capacity estimation** (back-of-envelope). **Load balancing**: Nginx, Gunicorn workers. **Caching**: CDN, Redis, `@lru_cache`. **API design**: REST (FastAPI) vs gRPC (`grpcio`) vs GraphQL (`strawberry`). **Microservices**: monolith-first, Conway's Law. **ASGI servers**: Uvicorn, Hypercorn. **Serverless**. **Design exercises**: URL shortener, chat, rate limiter.

### [Chapter 39 — AI System Design & Infrastructure](part_7_production/chapter_39_ai_system_design.md) ✅
LLM Inference, vLLM, RAG System Design, Vector DB Scaling, Agent Orchestration.

### [Chapter 40 — Deployment & DevOps ⭐](part_7_production/chapter_40_deployment_devops.md) ✅
Docker multi-stage build, non-root user, Gunicorn + Uvicorn workers, CI/CD với GitHub Actions, và lời kết cho toàn bộ cuốn sách.

---

## Appendices

### [A — `mypy --strict` Configuration](appendices/appendix_a_mypy_strict.md) ✅
Bật `--strict` trên codebase có sẵn mà không chết ngập trong lỗi. Từng flag làm gì, chiến lược migration theo module, và các escape hatch.

### [B — From F#/TypeScript to Python Translation Table](appendices/appendix_b_fsharp_typescript_to_python.md) ✅
Bảng tra cứu: record → frozen dataclass, discriminated union → union type + `match`, `Result` → `returns.Result`, computation expression → generator do-notation.

### [C — `returns` vs `fp-ts` vs Rust `Result` Comparison](appendices/appendix_c_returns_fpts_result.md) ✅
So sánh ba hệ sinh thái Result/Either: tên method, cách compose, cách xử lý async, và điểm mạnh/yếu của từng bên.

---

## 📊 Book Statistics

> Số liệu đo trực tiếp từ file `.md` trong repo.

| Part | Chương | Dòng | Code blocks |
|------|--------|------|-------------|
| **Part 0** CS Foundations | Ch1–3C (5) | 1.544 | 40 |
| **Part I** Python Fundamentals | Ch4–10 (7) | 1.477 | 47 |
| **Part II** Thinking Functionally | Ch11–15 (5) | 983 | 23 |
| **Part III** Design Patterns | Ch16–17 (2) | 327 | 7 |
| **Part IV** DDD | Ch18–24 (7) | 973 | 21 |
| **Part V** FP Patterns | Ch25–28c (7) | 5.118 | 113 |
| **Part VI** Testing & Web | Ch29–32 (4) | 660 | 20 |
| **Part VII** Production | Ch33–40 (9) | — | — |
| **Phụ lục** | A–C (3) | — | — |

> ⚠️ Part V (FP Patterns) hiện dày hơn hẳn các phần còn lại. Part III, IV, VI là
> các phần mỏng nhất và là ưu tiên mở rộng tiếp theo.
