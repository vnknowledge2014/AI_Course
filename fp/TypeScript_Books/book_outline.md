# 📘 Domain-Driven FP with TypeScript — Combined Book Outline

> Kết hợp: DDD Functional + FP Made Easier + Learn Go with Tests + F# Fun & Profit
> Coverage: **~75%** FP/DDD · Full CS foundations included
> Approach: Foundations → Beginner → Intermediate → Advanced → Principal
> Tổng: **50 chương** (Ch0–Ch43, gồm các chương phụ 3B/3C/25b/30b/30c/42B) + **2 phụ lục**

---

## Part 0: CS Foundations (Lý thuyết Khoa học Máy tính)

> ⚠️ **LƯU Ý DÀNH CHO NGƯỜI MỚI (BEGINNERS)**:
> Phần này chứa nhiều lý thuyết và tư duy hệ thống. Nếu bạn không có nền tảng toán học hoặc mới học lập trình, việc đọc phần này có thể gây "ngợp".
> **HÃY BỎ QUA PART 0 VÀ NHẢY THẲNG ĐẾN PART 1 (TYPESCRIPT FUNDAMENTALS)** để bắt tay vào cài đặt và code thực hành ngay. Bạn có thể quay lại đọc Part 0 sau khi đã quen với TypeScript!
### [Chapter 0 — TypeScript in 10 Minutes](part_0_cs_foundations/chapter_00_typescript_in_10_minutes.md) ✅
Quick primer: `const`/`let`, arrow functions, `type` aliases, discriminated unions, `switch` + `assertNever`, destructuring, `.map()`/`.filter()`/`.reduce()`, `readonly`, `unknown` vs `any`, generics đọc hiểu. `strict: true` + no `any` = hai trụ cột.
*544 dòng · 18 code blocks · 5 assertions*

### [Chapter 1 — Math Foundations for FP](part_0_cs_foundations/chapter_01_math_foundations.md) ✅
Lambda Calculus: `(x) => x+1`. Curry-Howard. Discrete Math: Product (objects) = AND, Union = OR. Algebraic type sizes.
*832 dòng · 25 code blocks · 13 assertions*

### [Chapter 2 — Algorithmic Thinking & Complexity](part_0_cs_foundations/chapter_02_algorithmic_thinking.md) ✅
Big-O. `Array.push` O(1), `unshift` O(n). Recursion + HOFs. Binary search, memoization. Sorting.
*709 dòng · 23 code blocks · 32 assertions*

### [Chapter 3 — Functional Data Structures](part_0_cs_foundations/chapter_03_functional_data_structures.md)

### [Chapter 3B — Advanced Algorithms & LeetCode Patterns](part_0_cs_foundations/chapter_03b_leetcode_patterns.md) ✅
Two Pointers, Sliding Window, DP, Graph Traversal in TypeScript.

### [Chapter 3C — Hardware, Compute: LeetCPU & LeetGPU](part_0_cs_foundations/chapter_03c_hardware_leetgpu.md) ✅
CPU Caches, Memory Hierarchy, V8 optimizations, GPU Compute concepts. ✅
Immutable updates: spread `{...obj}` = O(n). **Immer** structural sharing. `immutable-js` HAMT. Graph: `Map<string, Set<string>>`.
*135 dòng · 1 code blocks · 0 assertions*

---

## Part I: TypeScript Fundamentals (Beginner)

### [Chapter 4 — The TypeScript Ecosystem & Tooling](part_1_fundamentals/chapter_04_getting_started.md) ✅
Node.js vs Bun, `npm`, `tsconfig.json` (`strict: true`), ESLint, Prettier, VSCode IDE setup.
*149 dòng · 7 code blocks · 0 assertions*

### [Chapter 5 — Types & Variables](part_1_fundamentals/chapter_05_types_and_variables.md) ✅
`const`/`let`. Primitives, inference. `unknown` vs `any`. `readonly`.
*695 dòng · 21 code blocks · 26 assertions*

### [Chapter 6 — Control Flow & Narrowing](part_1_fundamentals/chapter_06_control_flow_narrowing.md) ✅
Type narrowing: `typeof`, `instanceof`, `in`. Discriminated unions + `switch`. Exhaustive `never`.
*742 dòng · 18 code blocks · 33 assertions*

### [Chapter 7 — Functions & Closures](part_1_fundamentals/chapter_07_functions_closures.md) ✅
Arrow functions, generics `<T>`, HOFs: `map`, `filter`, `reduce`.
*822 dòng · 21 code blocks · 51 assertions*

### [Chapter 8 — Objects, Interfaces & Classes](part_1_fundamentals/chapter_08_objects_interfaces_classes.md) ✅
Object types, interfaces, `type` aliases. `readonly`. Structural typing.
*816 dòng · 21 code blocks · 32 assertions*

### [Chapter 9 — Advanced Type System](part_1_fundamentals/chapter_09_advanced_type_system.md) ✅
Union, intersection, literal, DUs. Utility types. Conditional, mapped, template literal types.
*737 dòng · 20 code blocks · 11 assertions*

### [Chapter 10 — Modules & Project Structure](part_1_fundamentals/chapter_10_modules_project_structure.md) ✅
ES modules. Barrel files. Monorepo.
*673 dòng · 33 code blocks · 0 assertions*

---

## Part II: Thinking Functionally (Intermediate)

### [Chapter 11 — Immutability & Purity](part_2_thinking_functionally/chapter_11_immutability_purity.md) ✅
`const`, `Object.freeze`, `readonly`, `as const`. Spread. Pure functions.
*931 dòng · 20 code blocks · 36 assertions*

### [Chapter 12 — Composition & Pipelines](part_2_thinking_functionally/chapter_12_composition_pipelines.md) ✅
`compose(f, g)`. `pipe()` from `fp-ts`. Method chaining. Point-free.
*892 dòng · 16 code blocks · 26 assertions*

### [Chapter 13 — ADTs in TypeScript](part_2_thinking_functionally/chapter_13_adts_in_typescript.md) ✅
Discriminated unions. **Branded types**: `type Email = string & { __brand: 'Email' }`.
*1.218 dòng · 22 code blocks · 32 assertions*

### [Chapter 14 — Smart Constructors & Validation](part_2_thinking_functionally/chapter_14_smart_constructors_validation.md) ✅
`Zod` / `io-ts`. Parse, don't validate.
*1.168 dòng · 21 code blocks · 49 assertions*

### [Chapter 15 — Generics Deep Dive](part_2_thinking_functionally/chapter_15_generics_deep_dive.md) ✅
Constraints `extends`. Conditional types. Infer. Parametric polymorphism.
*1.026 dòng · 23 code blocks · 27 assertions*

---

## Part III: Design Patterns — FP & Classical (Advanced)

### [Chapter 16 — GoF → FP Translation](part_3_design_patterns/chapter_16_gof_fp_translation.md) ✅
Strategy = HOF. Observer = EventEmitter/RxJS. Command = DU. Visitor = switch. Decorator = wrapper. Middleware = compose.
*903 dòng · 15 code blocks · 23 assertions*

### [Chapter 17 — CQRS & Event Sourcing](part_3_design_patterns/chapter_17_cqrs_event_sourcing.md) ✅
Tách read/write. Event Sourcing: `reduce` rebuild. Projections.
*1.132 dòng · 25 code blocks · 31 assertions*

---

## Part IV: Domain-Driven Design (Advanced)

### [Chapter 18 — Introduction to DDD](part_4_ddd/chapter_18_introduction_to_ddd.md) ✅
Ubiquitous Language, Bounded Contexts, Event Storming.
*913 dòng · 19 code blocks · 12 assertions*

### [Chapter 19 — Functional Architecture](part_4_ddd/chapter_19_functional_architecture.md) ✅
Layered. Module boundaries via barrel exports. IO at edges.
*1.175 dòng · 23 code blocks · 35 assertions*

### [Chapter 20 — Domain Modeling with DUs](part_4_ddd/chapter_20_domain_modeling_with_dus.md) ✅
Value Objects = branded types. State machines via DUs.
*1.247 dòng · 26 code blocks · 49 assertions*

### [Chapter 21 — Workflows as Pipelines](part_4_ddd/chapter_21_workflows_as_pipelines.md) ✅
`pipe()`, `flow()`. Async pipelines.
*1.038 dòng · 20 code blocks · 47 assertions*

### [Chapter 22 — Error Handling — `neverthrow` / ROP](part_4_ddd/chapter_22_error_handling_rop.md) ✅
`Result<T, E>`. Railway-Oriented Programming in TypeScript.
*1.056 dòng · 19 code blocks · 56 assertions*

### [Chapter 23 — Serialization & DTOs](part_4_ddd/chapter_23_serialization_dtos.md) ✅
JSON + `Zod` schemas. Domain ↔ DTO.
*1.260 dòng · 20 code blocks · 56 assertions*

### [Chapter 24 — Persistence & Repository](part_4_ddd/chapter_24_persistence_repository.md) ✅
Repository interface. Prisma, Drizzle. DI via `Effect` services.
*1.232 dòng · 22 code blocks · 42 assertions*

---

## Part V: FP Patterns with fp-ts / Effect (Advanced)

### [Chapter 25 — Introduction to fp-ts (Legacy) and Effect (Modern)](part_5_fp_patterns/chapter_25_intro_fpts_effect.md) ✅
`Effect<A, E, R>`, `Effect.gen`. fp-ts types (`Option`, `Either`, `TaskEither`). Migration decision framework.

### [Chapter 25b — Abstract Algebra: Equivalence & Order (Effect-TS)](part_5_fp_patterns/chapter_25b_eq_ord.md) ✅
`Data.struct`, `Equivalence` via `mapInput`, `Order` combinators for structural sorting.

### [Chapter 26 — Functors & Map](part_5_fp_patterns/chapter_26_functors_map.md) ✅
`Option.map`, `Either.map`. Functor laws.
*603 dòng · 12 code blocks · 40 assertions*

### [Chapter 27 — Monads & Bind](part_5_fp_patterns/chapter_27_monads_bind.md) ✅
`chain`, `flatMap`. Do-notation via `Effect.gen()`.
*570 dòng · 10 code blocks · 19 assertions*

### [Chapter 28 — Applicative & Validation](part_5_fp_patterns/chapter_28_applicative_validation.md) ✅
Fail-fast vs collect ALL errors. `sequenceT`.
*372 dòng · 6 code blocks · 19 assertions*

### [Chapter 29 — Monoids & Abstract Algebra](part_5_fp_patterns/chapter_29_monoids_algebra.md) ✅
`Semigroup`, `Monoid`. `concatAll`. Merging configs.
*356 dòng · 6 code blocks · 26 assertions*

### [Chapter 30 — Traverse & Sequence](part_5_fp_patterns/chapter_30_traverse_sequence.md) ✅
`Array.traverse`. `Array<Option<A>>` → `Option<Array<A>>`.
*414 dòng · 9 code blocks · 18 assertions*

### [Chapter 30b — Advanced Effect: Context (DI), Fibers & Scheduling](part_5_fp_patterns/chapter_30b_effect_advanced.md) ✅
The `R` in `Effect`. `Context.GenericTag`, `Layer`, `Effect.provide`. `Effect.fork`, `Effect.all`, `Schedule`.

### [Chapter 30c — Recursive Types & Folds](part_5_fp_patterns/chapter_30c_recursive_types_folds.md) ✅
Catamorphisms. Trees, LinkedLists, AST. Factoring out recursion via `fold`. AST Evaluator.

---

## Part VI: Testing & Full-Stack (Principal)

### [Chapter 31 — TDD with TypeScript](part_6_testing_fullstack/chapter_31_tdd_typescript.md) ✅
`vitest` / `jest`. Red → Green → Refactor.
*601 dòng · 12 code blocks · 41 assertions*

### [Chapter 32 — Property-Based Testing](part_6_testing_fullstack/chapter_32_property_based_testing.md) ✅
`fast-check`. Arbitraries, shrinking. Round-trip, idempotency, invariant, oracle.
*356 dòng · 5 code blocks · 16 assertions*

### [Chapter 33 — Architecture Patterns](part_6_testing_fullstack/chapter_33_architecture_patterns.md) ✅
Hexagonal. Ports & Adapters. Functional core / imperative shell.
*244 dòng · 6 code blocks · 0 assertions*

### [Chapter 34 — Backend — Hono / Express](part_6_testing_fullstack/chapter_34_backend_hono_express.md) ✅
REST API, middleware, error handling. Database integration.
*412 dòng · 7 code blocks · 8 assertions*

### [Chapter 35 — Frontend — React + FP](part_6_testing_fullstack/chapter_35_frontend_react_fp.md) ✅
React as pure functions. `fp-ts` in React. FP state management.
*171 dòng · 5 code blocks · 0 assertions*

### [Chapter 36 — Capstone Part 1: Domain Model ⭐](part_6_testing_fullstack/chapter_36_capstone_domain_model.md) ✅
DDD model, `Effect`/`neverthrow` pipelines, Zod validation, CQRS, vitest.
*490 dòng · 9 code blocks · 15 assertions*

---

## Part VII: Production Engineering (Principal)

> *Database, Security, Distributed Systems, System Design — self-contained*

### [Chapter 37 — Database Fundamentals & SQL](part_7_production/chapter_37_database_fundamentals.md) ✅
**Relational model**: tables, keys. **SQL**: `SELECT`, `JOIN`, `GROUP BY`, CTEs. **Normalization** 1NF→BCNF. **Indexing**: B-Tree, composite, `EXPLAIN ANALYZE`. **Transactions**: ACID, isolation levels. **TypeScript**: Prisma (type-safe ORM), Drizzle (SQL-like), Kysely (query builder). Connection pooling.
*528 dòng · 8 code blocks · 14 assertions*

### [Chapter 38 — Advanced Data Patterns](part_7_production/chapter_38_advanced_data_patterns.md) ✅
**Migrations**: Prisma migrate, Drizzle kit. **CQRS persistence**: read/write separation. **Event Store**. **NoSQL**: MongoDB (`mongoose`), Redis (`ioredis`), DynamoDB. **Caching**: strategies, `node-cache`, Redis patterns.
*406 dòng · 5 code blocks · 17 assertions*

### [Chapter 39 — Security Essentials](part_7_production/chapter_39_security_essentials.md) ✅
**Auth**: Password hashing (`bcrypt`/`argon2`), salt. **Sessions vs Tokens**: JWT (`jose`), refresh tokens, PASETO. **OAuth 2.0**: PKCE, Passport.js / `arctic`. **Authorization**: RBAC, ABAC, middleware guards. **TypeScript**: `helmet`, `cors`, `express-rate-limit`.
*416 dòng · 7 code blocks · 17 assertions*

### [Chapter 40 — Application Security & Hardening](part_7_production/chapter_40_app_security_hardening.md) ✅
**OWASP Top 10**: SQL injection (parameterized queries), XSS (`DOMPurify`, CSP), CSRF (SameSite cookies, tokens), SSRF. **Zod validation = defense in depth**. **HTTPS**: TLS, HSTS. **Headers**: CSP, X-Frame-Options. **Secrets**: env vars, `.env` (dev), Vault/AWS SSM (prod). **Rate limiting**: sliding window. **Audit logging**: structured logs.
*393 dòng · 8 code blocks · 13 assertions*

### [Chapter 41 — Distributed Systems Fundamentals](part_7_production/chapter_41_distributed_systems.md) ✅
**CAP Theorem**: CP vs AP. **Consistency**: strong, eventual, causal. **Replication**: leader-follower, leaderless. CRDTs. **Partitioning**: hash, range, consistent hashing. **Consensus**: Raft basics. **Message Queues**: BullMQ (Redis-based), Kafka (`kafkajs`), RabbitMQ (`amqplib`). **Patterns**: Saga, Circuit Breaker (`opossum`), Retry+backoff, Outbox, CDC. **Observability**: Winston/Pino logging, Prometheus, OpenTelemetry.
*482 dòng · 7 code blocks · 21 assertions*

### [Chapter 42 — System Design Thinking](part_7_production/chapter_42_system_design.md) ✅
**Capacity estimation**. **Load balancing**: Nginx, Cloudflare. **Caching layers**: CDN, Redis, browser. **API design**: REST vs gRPC (`nice-grpc`) vs GraphQL (`graphql-yoga`). **Microservices**: Conway's Law, monolith-first. **BFF** (Backend for Frontend). **Serverless**: Lambda, Vercel, Cloudflare Workers. **Design exercises**: URL shortener, chat, notification, rate limiter.
*167 dòng · 3 code blocks · 0 assertions*

### [Chapter 42B — AI System Design & Infrastructure](part_7_production/chapter_42b_ai_system_design.md) ✅
**TypeScript/Node.js trong kiến trúc AI**, thiết kế AI Gateway, RAG System Design quy mô lớn, Multi-Agent Orchestration.

### [Chapter 43 — Capstone Part 2: Production Deployment ⭐](part_7_production/chapter_43_capstone_production.md) ✅
Full-stack: Next.js/Hono + PostgreSQL (Prisma), Redis cache, JWT auth, HTTPS, rate limiting, structured logging, Docker, CI/CD, Vercel/Railway deployment.
*518 dòng · 7 code blocks · 12 assertions*

---

## Appendices
### [A — fp-ts ↔ Effect Comparison](appendices/appendix_a_fpts_effect_comparison.md) ✅
### [B — From F#/PureScript to TypeScript Translation Table](appendices/appendix_b_fsharp_purescript_to_typescript.md) ✅

---

## 📊 Book Statistics

> Số liệu đo trực tiếp từ file `.md` trong repo.

| Part | Chương | Dòng | Code blocks |
|------|--------|------|-------------|
| **Part 0** CS Foundations | Ch0–3C (6) | 3.143 | 91 |
| **Part I** Fundamentals | Ch4–10 (7) | 4.634 | 141 |
| **Part II** Thinking FP | Ch11–15 (5) | 5.235 | 102 |
| **Part III** Design Patterns | Ch16–17 (2) | 2.035 | 40 |
| **Part IV** DDD | Ch18–24 (7) | 7.921 | 149 |
| **Part V** FP Patterns | Ch25–30c (9) | 3.673 | 65 |
| **Part VI** Testing & Fullstack | Ch31–36 (6) | 2.274 | 44 |
| **Part VII** Production | Ch37–43 (8) | 3.023 | 47 |
| **Phụ lục** | A–B (2) | 244 | 4 |
| **TỔNG** | **50 chương + 2 phụ lục** | **32.550** | **683** |
