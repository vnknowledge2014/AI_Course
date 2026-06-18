# 🦀 Domain-Driven FP with Rust — Combined Book Outline

> Kết hợp: DDD Functional + FP Made Easier + Learn Go with Tests + F# Fun & Profit
> Coverage: **~70%** FP/DDD · Full CS foundations included
> Approach: Quick Primer → Foundations → Beginner → Intermediate → Advanced → Principal
> Tổng: **46 chapters** (0–44 + 36B) + Appendices

---

## [Chapter 0 — Rust in 10 Minutes (Quick Primer)](part_0_cs_foundations/chapter_00_rust_primer.md)

> *Đọc trước khi vào Part 0. Chỉ dạy đủ syntax để đọc hiểu code examples — Part I sẽ dạy lại sâu hơn.*

Setup (`rustup`, `cargo new`). `let`, `fn`, return values. Scalar types (`i32`, `f64`, `bool`, `&str`/`String`). `if/else` (là expression). `enum` cơ bản, `struct` cơ bản, `match` cơ bản. Closures `|x| x + 1`. `println!`, `assert_eq!`. `Vec`, `.push()`, `for..in`. `Option<T>` và `Result<T, E>` ở mức "đọc hiểu". **Không dạy ownership/borrowing** — để dành cho Chapter 9.

---

## Part 0: CS Foundations (Pre-requisite)

> *Nền tảng CS cần thiết — không phụ thuộc ngôn ngữ nhưng ví dụ bằng Rust. Đọc Chapter 0 trước nếu chưa biết Rust.*

### [Chapter 1 — Math Foundations for FP](part_0_cs_foundations/chapter_01_math_foundations.md)
**Lambda Calculus**: `λx.x+1` = closures, β-reduction = function application, Church encoding. **Curry-Howard Correspondence**: Types = Propositions, Programs = Proofs → giải thích tại sao "make illegal states unrepresentable" works. **Discrete Math**: Set theory (Product = Cartesian product, Sum = Disjoint union), relations, partial orders. **Algebraic type sizes**: `bool × bool` = 4 states — dùng toán để tính domain space.

### [Chapter 2 — Algorithmic Thinking & Complexity](part_0_cs_foundations/chapter_02_algorithmic_thinking.md)
**Big-O**: O(1), O(log n), O(n), O(n log n), O(n²). Amortized analysis (`Vec::push`). **Recursion vs Iteration**: tail recursion, stack frames. **Common patterns**: divide & conquer (mergesort), greedy, binary search, two pointers. **Sorting**: mergesort (stable, FP-friendly), quicksort. **Searching**: linear, binary, hash-based.

### [Chapter 3 — Functional Data Structures](part_0_cs_foundations/chapter_03_functional_data_structures.md)
**Persistent DS**: structural sharing — update O(log n) thay vì copy O(n). **HAMT** (Hash Array Mapped Trie): nền tảng của `im` crate. **Amortized Queue**: 2 stacks = O(1) amortized queue (Okasaki). **Finger Trees**: deque + priority queue + sequence. **Graph as ADT**: `HashMap<Node, HashSet<Node>>`. **Big-O trong FP**: tại sao `List::append` O(n), `Vec::push` O(1), persistent `HashMap` O(log₃₂ n).

---

## Part I: Rust Fundamentals (Beginner)

> *Nền tảng ngôn ngữ — tham khảo programiz.com/rust*

### [Chapter 4 — Getting Started with Rust](part_1_rust_fundamentals/chapter_04_getting_started.md)
Setup toolchain (`rustup`, `cargo`), Hello World, `cargo new`, project structure. Comments, `println!` macro. REPL via `evcxr`.

### [Chapter 5 — Variables, Types & Operators](part_1_rust_fundamentals/chapter_05_variables_types.md)
`let` bindings (immutable by default), `mut`, shadowing. Scalar types (`i32`, `f64`, `bool`, `char`), compound types (tuples, arrays). Type casting, operators. **So sánh với F#**: `let` binding tương tự, nhưng Rust cần `mut` explicit.

### [Chapter 6 — Control Flow](part_1_rust_fundamentals/chapter_06_control_flow.md)
`if/else` (là expression, trả giá trị), `match` (exhaustive pattern matching). `loop`, `while`, `for..in`, `break`/`continue`. **Pattern matching** — nền tảng cho toàn bộ phần sau.

### [Chapter 7 — Functions & Closures](part_1_rust_fundamentals/chapter_07_functions_closures.md)
Functions, return values, expressions vs statements. Closures `|x| x + 1`. Variable scope, nested functions. **First-class functions** — functions là values (Book 2 Ch 2, Book 4 Sec II).

### [Chapter 8 — Data Structures](part_1_rust_fundamentals/chapter_08_data_structures.md)
Arrays, slices `&[T]`, Vec, String (`String` vs `&str`), HashMap, HashSet. Iterators và iterator chains (`.map()`, `.filter()`, `.fold()`). **Kết nối với Ch 3**: Big-O của mỗi operation.

### [Chapter 9 — Ownership & Borrowing](part_1_rust_fundamentals/chapter_09_ownership_borrowing.md)
Stack vs heap. **Ownership rules** (single owner, move semantics). References `&T` (shared) vs `&mut T` (exclusive). Borrowing rules. Lifetimes cơ bản. **Đây là unique feature của Rust** — không có trong bất kỳ cuốn sách nào, cần chapter riêng.

### [Chapter 10 — Error Handling Basics](part_1_rust_fundamentals/chapter_10_error_handling.md)
`Result<T, E>` và `Option<T>`. `unwrap()`, `expect()`, `?` operator. Custom error types. **Đây là nền tảng cho Railway-Oriented Programming ở Part IV**.

### [Chapter 11 — Modules, Crates & Cargo](part_1_rust_fundamentals/chapter_11_modules_crates.md)
Module system (`mod`, `pub`, `use`), crate structure, `Cargo.toml`, dependencies, workspaces.

---

## Part II: Thinking Functionally in Rust (Intermediate)

> *Book 4 Sec II + Book 2 Ch 1-5 adapted for Rust*

### [Chapter 12 — Immutability & Purity](part_2_thinking_functionally/chapter_12_immutability_purity.md)
Tại sao immutability quan trọng (Book 2 Ch 1). Rust defaults: `let` immutable, `&T` shared reference. `const` vs `let`. Frozen data structures. **Pure functions**: functions không side-effects, deterministic output.

### [Chapter 13 — Higher-Order Functions & Composition](part_2_thinking_functionally/chapter_13_hof_composition.md)
Closures deep dive, `Fn`/`FnMut`/`FnOnce` traits. Passing functions as params. Returning functions (boxed closures `Box<dyn Fn>`). **Function composition** — chuỗi iterator: `.map().filter().fold()` = pipeline (Book 1 Ch 7, Book 4 Sec II.7).

### [Chapter 14 — Structs, Enums & Algebraic Types](part_2_thinking_functionally/chapter_14_algebraic_types.md)
**Product types** = `struct` (AND). **Sum types** = `enum` (OR). Newtype pattern `struct Email(String)`. Tuple structs. Generic enums. **"Types as design tools"** (Book 4 Sec IV, Book 1 Ch 4). **Kết nối với Ch 1**: algebraic type sizes.

### [Chapter 15 — Pattern Matching Mastery](part_2_thinking_functionally/chapter_15_pattern_matching.md)
`match`, `if let`, `while let`, destructuring. Nested patterns, guards, ranges. Exhaustive matching = compiler enforces handling ALL cases. **"Make illegal states unrepresentable"** (Book 1 Ch 5, Book 4 Designing with Types).

### [Chapter 16 — Traits — Rust's Typeclass System](part_2_thinking_functionally/chapter_16_traits.md)
Trait declaration, implementation, default methods. Trait bounds `T: Display + Clone`. `impl Trait` vs `dyn Trait`. Derived traits (`#[derive(Debug, Clone, PartialEq)]`). **Typeclasses → Traits** (Book 2 Ch 6). Orphan rules.

### [Chapter 17 — Generics & Trait Bounds](part_2_thinking_functionally/chapter_17_generics.md)
Generic functions, generic structs/enums. Where clauses. Monomorphization. Associated types vs generic params. **Parametric polymorphism** (Book 2 Ch 3, Ch 12).

---

## Part III: Design Patterns — FP & Classical (Advanced)

> *GoF → FP translation + DDD patterns bổ sung*

### [Chapter 18 — GoF Patterns → FP Translation](part_3_design_patterns/chapter_18_gof_to_fp.md)
**Strategy** = Higher-Order Function. **Observer** = Event stream / channel. **Command** = Enum variant (data as command). **Visitor** = Pattern match. **Factory** = Smart constructor. **Decorator** = Function composition. **Adapter** = `From`/`Into` trait. **Iterator** = `Iterator` trait (already FP-native).

### [Chapter 19 — CQRS & Event Sourcing](part_3_design_patterns/chapter_19_cqrs_event_sourcing.md)
Command-Query Responsibility Segregation: tách read model (query) và write model (command). **Event Sourcing**: lưu events thay vì state, rebuild bằng `fold`. Event store, projections, snapshots. Khi nào dùng, khi nào không.

---

## Part IV: Domain-Driven Design with Rust (Advanced)

> *Book 1 toàn bộ, adapted for Rust idioms*

### [Chapter 20 — Introduction to DDD](part_4_ddd_with_rust/chapter_20_intro_ddd.md)
Ubiquitous Language, Bounded Contexts, Subdomains. Event Storming. **Domain discovery process** — interviews, commands, events, workflows (Book 1 Ch 1-2). Language-agnostic.

### [Chapter 21 — Functional Architecture](part_4_ddd_with_rust/chapter_21_functional_architecture.md)
Onion Architecture: IO ở rìa, pure domain logic ở core. Module boundaries = bounded contexts. **Rust module system enforce boundaries** qua `pub`/private visibility (Book 1 Ch 3).

### [Chapter 22 — Domain Modeling with Rust Types](part_4_ddd_with_rust/chapter_22_domain_modeling.md)
Newtype pattern cho domain values: `struct OrderId(u64)`, `struct Email(String)`. Smart constructors: `impl Email { pub fn new(s: &str) -> Result<Self, ValidationError> }`. State machines với enums (Book 1 Ch 4-6, Book 4 Designing with Types).

### [Chapter 23 — Workflows as Pipelines](part_4_ddd_with_rust/chapter_23_workflows_pipelines.md)
Workflow = chain of functions: `validate → price → acknowledge → save`. Iterator chaining pattern. **Pipeline composition** trong Rust: method chaining, `and_then`, custom `pipe!` macro (Book 1 Ch 7, 9).

### [Chapter 24 — Railway-Oriented Programming in Rust ⭐](part_4_ddd_with_rust/chapter_24_rop.md)
`Result<T, E>` = two-track model. `?` operator = monadic bind. `map`, `and_then`, `map_err`. **Composing validations**: custom `Validated<T>` type cho collecting ALL errors (applicative style). Error type hierarchy (Book 1 Ch 10, Book 4 ROP).

### [Chapter 25 — Serialization & Anti-Corruption Layer](part_4_ddd_with_rust/chapter_25_serialization_acl.md)
`serde` derive: `#[derive(Serialize, Deserialize)]`. Domain types ↔ DTOs. `From`/`Into` trait cho mapping. Anti-corruption layer pattern. JSON, TOML, MessagePack (Book 1 Ch 11).

### [Chapter 26 — Persistence & Side Effects at Edges](part_4_ddd_with_rust/chapter_26_persistence.md)
Repository pattern: `trait OrderRepository { fn save(&self, order: &Order) -> Result<()> }`. Dependency injection via trait objects hoặc generics. CQS. Transaction boundaries (Book 1 Ch 12).

### [Chapter 27 — Evolving the Design](part_4_ddd_with_rust/chapter_27_evolving_design.md)
Adding features without breaking design. Refactoring with compiler guidance. Feature flags via enums. Backward-compatible type evolution (Book 1 Ch 13).

---

## Part V: FP Patterns in Rust (Advanced)

> *Book 2 Intermediate + Book 4 Functional Patterns, adapted*

### [Chapter 28 — Abstract Algebra for Rust Developers](part_5_fp_patterns/chapter_28_abstract_algebra.md)
**Semigroup**: trait `Append { fn append(self, other: Self) -> Self }`. **Monoid**: `Append` + `fn empty() -> Self`. Rust std examples: `String`, `Vec`, `Option`. Derive patterns. (Book 2 Ch 8, Book 4 Monoids).

### [Chapter 29 — Functors & Map in Rust](part_5_fp_patterns/chapter_29_functors.md)
`Option::map`, `Result::map`, `Iterator::map` — bạn đã dùng Functors. **Functor = `.map()` trên bất kỳ container nào**. Tại sao Rust không abstract Functor trait (thiếu HKTs). GATs approach (Book 2 Ch 12, Book 4 Elevated World).

### [Chapter 30 — Monads in Rust — The Practical View](part_5_fp_patterns/chapter_30_monads.md)
`Option` + `and_then` = Maybe Monad. `Result` + `?` = Either Monad. **Bạn đã dùng monads hàng ngày** mà không biết. Chaining, flat-mapping. `Iterator::flat_map`. So sánh với Haskell/PureScript do-notation (Book 2 Ch 18).

### [Chapter 31 — Parser Combinators with `nom` / `chumsky`](part_5_fp_patterns/chapter_31_parser_combinators.md)
Parser = `&str → Result<(T, &str)>`. Combinators: `tag`, `alt`, `many0`, `map`, `pair`. Build JSON parser. **nom** (macro-based) vs **chumsky** (type-based). (Book 4 Parser Combinators, Book 2 Ch 17, 19).

### [Chapter 32 — Recursive Types & Folds](part_5_fp_patterns/chapter_32_recursive_types_folds.md)
`enum Expr { Lit(i32), Add(Box<Expr>, Box<Expr>) }`. `Box` cho recursive types. Implementing `fold` / catamorphisms. Tree traversal. Expression evaluators (Book 4 Fold & Recursive Types, Book 2 Ch 10).

---

## Part VI: Testing & Software Engineering (Principal)

> *Book 3 Learn Go with Tests methodology, adapted for Rust*

### [Chapter 33 — TDD with Rust](part_6_testing/chapter_33_tdd.md)
`#[cfg(test)]`, `#[test]`, `assert_eq!`, `assert!`. Test organization: unit tests (in-file), integration tests (`tests/`). `cargo test`. **Red → Green → Refactor** cycle (Book 3 throughout).

### [Chapter 34 — Property-Based Testing](part_6_testing/chapter_34_property_testing.md)
`proptest` / `quickcheck` crates. Chọn properties: round-trip, idempotent, metamorphic, oracle. Shrinking. **"The lazy programmer's guide"** (Book 4 PBT, Book 3 Ch Property-Based Tests).

### [Chapter 35 — Mocking, DI & Hexagonal Architecture](part_6_testing/chapter_35_mocking_di.md)
Trait-based DI. Mock strategies: `mockall` crate, manual mocks. **Hexagonal architecture**: Port = trait, Adapter = implementation. Functional core / imperative shell (Book 3 DI, Mocking, Working Without Mocks).

### [Chapter 36 — Concurrency & Async](part_6_testing/chapter_36_concurrency_async.md)
`std::thread`, `Arc<Mutex<T>>`, channels (`mpsc`). `async/await`, `tokio`. Fearless concurrency qua ownership system. Actors via `tokio::sync::mpsc` (Book 2 Ch 22, Book 3 Concurrency/Select).

### [Chapter 36B — Web Services with Axum](part_6_testing/chapter_36b_web_axum.md)
HTTP basics (request/response). **Axum** web framework: Router, Handlers, Extractors (`Path`, `Query`, `Json`, `State`). Middleware (logging, auth). `impl IntoResponse` cho domain errors → **ROP cho web** (Ch 24). CRUD API hoàn chỉnh với in-memory state.

---

## Part VII: Production Engineering (Principal)

> *Database, Security, Distributed Systems, System Design — self-contained*

### [Chapter 37 — Capstone Part 1: Domain Model ⭐](part_7_production/chapter_37_capstone_domain.md)
**Order-Taking System** — Domain discovery (Event Storming), type-driven domain model (newtype, enums, smart constructors), pipeline workflows với `Result` chaining, CQRS + Event Sourcing, `serde` serialization, property-based tests.

### [Chapter 38 — Database Fundamentals & SQL](part_7_production/chapter_38_database_sql.md)
**Relational model**: tables, rows, keys (PK, FK, composite). **SQL**: `SELECT`, `JOIN` (inner/left/outer/cross), `GROUP BY`, `HAVING`, subqueries, CTEs (`WITH`). **Design**: normalization (1NF→3NF→BCNF), denormalization khi nào. **Indexing**: B-Tree, Hash index, composite index, covering index, `EXPLAIN ANALYZE`. **Transactions**: ACID (Atomicity, Consistency, Isolation, Durability), isolation levels (Read Uncommitted → Serializable), optimistic vs pessimistic locking. **Rust**: `sqlx` (compile-time checked queries), `diesel` (DSL), `sea-orm` (active record). Connection pooling `bb8`/`deadpool`.

### [Chapter 39 — Advanced Data Patterns](part_7_production/chapter_39_advanced_data.md)
**Migrations**: schema versioning, `sqlx migrate`, zero-downtime migration strategies. **CQRS persistence**: separate read/write databases. **Event Store**: append-only table, snapshots. **NoSQL khi nào**: Document (MongoDB), Key-Value (Redis), Column (ScyllaDB), Graph (Neo4j) — picking the right tool. **Caching**: read-through, write-through, write-behind, cache invalidation ("the two hard problems"). **Redis** patterns: cache, pub/sub, rate limiting. Rust: `fred`, `redis-rs`.

### [Chapter 40 — Security Essentials](part_7_production/chapter_40_security.md)
**Authentication**: Password hashing (`argon2`, `bcrypt` — NEVER SHA/MD5), salt, pepper. **Sessions vs Tokens**: cookie-based sessions, JWT (header.payload.signature), refresh tokens, PASETO (safer JWT). **OAuth 2.0**: Authorization Code flow, PKCE, scopes, providers (Google, GitHub). **Authorization**: RBAC (roles), ABAC (attributes), capability-based (from Book 4). Rust: `argon2`, `jsonwebtoken`, `axum-login`, `tower-sessions`.

### [Chapter 41 — Application Security & Hardening](part_7_production/chapter_41_app_security.md)
**OWASP Top 10**: Injection (SQL injection, command injection), XSS (stored, reflected, DOM), CSRF (tokens, SameSite cookies), Broken Auth, Security Misconfiguration, SSRF, Insecure Deserialization. **Input validation**: whitelist > blacklist, `validator` crate, Zod-like validation. **HTTPS/TLS**: certificates, HSTS, `rustls`. **Headers**: `Content-Security-Policy`, `X-Frame-Options`, `Strict-Transport-Security`. **Secrets management**: env vars, `.env` (dev only), Vault/SSM (production). **Rate limiting**: sliding window, token bucket. **Logging security events**: audit trail, tamper-proof logs.

### [Chapter 42 — Distributed Systems Fundamentals](part_7_production/chapter_42_distributed_systems.md)
**CAP Theorem**: Consistency + Availability + Partition tolerance — pick 2. Real-world: CP (bank) vs AP (social media). **Consistency models**: strong, eventual, causal, read-your-writes. **Replication**: leader-follower, multi-leader, leaderless (Dynamo-style). Conflict resolution: LWW, vector clocks, CRDTs. **Partitioning/Sharding**: hash-based, range-based, consistent hashing. Rebalancing. **Consensus**: Raft basics — leader election, log replication. **Message Queues**: point-to-point vs pub/sub, delivery guarantees (at-most-once, at-least-once, exactly-once semantics). **Patterns**: Saga (choreography vs orchestration), Circuit Breaker (closed/open/half-open), Retry with exponential backoff+jitter, Outbox pattern, CDC (Change Data Capture). **Observability**: structured logging (`tracing` crate), metrics (Prometheus), distributed tracing (OpenTelemetry). Rust: `lapin` (RabbitMQ), `rdkafka` (Kafka), `tonic` (gRPC).

### [Chapter 43 — System Design Thinking](part_7_production/chapter_43_system_design.md)
**Capacity estimation**: QPS, storage, bandwidth — back-of-envelope. **Load balancing**: L4 (TCP) vs L7 (HTTP), round-robin, consistent hashing, health checks. **Caching layers**: client cache, CDN, application cache (Redis), database cache (query cache). **CDN**: static assets, edge computing. **API design**: REST (resource-oriented), gRPC (binary, streaming), GraphQL (flexible queries) — khi nào dùng cái nào. **Microservices**: khi nào split (Conway's Law), service mesh, API gateway. **Monolith first** → Modular monolith → Microservices (evolutionary). **Database scaling**: read replicas, connection pooling, sharding. **Design exercises**: URL shortener, chat system, notification service, rate limiter — practice thinking through trade-offs.

### [Chapter 44 — Capstone Part 2: Production Deployment ⭐](part_7_production/chapter_44_capstone_production.md)
Kết hợp toàn bộ: Order-Taking System + PostgreSQL (`sqlx`), Redis cache, JWT auth, HTTPS, rate limiting, structured logging (`tracing`), Docker deployment, CI/CD pipeline, monitoring dashboard.

---

## Appendices

### [A — Rust Ownership Cheat Sheet](appendices/appendix_a_ownership_cheatsheet.md)
Move, copy, borrow, lifetime rules quick reference.

### [B — From F#/PureScript to Rust](appendices/appendix_b_fsharp_purescript_to_rust.md)
Translation table: F# types → Rust types, PureScript typeclasses → Rust traits.

### [C — Cargo & Ecosystem Guide](appendices/appendix_c_cargo_ecosystem.md)
Essential crates: `serde`, `tokio`, `axum`, `sqlx`, `nom`, `proptest`, `thiserror`, `anyhow`, `tracing`, `argon2`.

### [D — Macros Essentials](appendices/appendix_d_macros.md)
Declarative macros (`macro_rules!`), attribute macros (`#[derive]`, `#[tokio::main]`), khi nào dùng macros vs functions.

### [E — Glossary & Index](appendices/appendix_e_glossary.md)
~60 thuật ngữ chính: Rust Core, FP Concepts, DDD Concepts, Production Patterns. Mỗi entry có Vietnamese explanation + chapter reference.

