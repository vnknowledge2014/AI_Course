# Appendix E — Glossary & Index: Từ điển thuật ngữ Rust

> Dù bạn đang ở chương nào, đôi khi bạn sẽ bắt gặp những thuật ngữ lạ lẫm hoặc tạm quên mất ý nghĩa của chúng. Đừng lo lắng! Phần phụ lục này chính là cuốn "từ điển mini" gom nhặt toàn bộ các khái niệm chính yếu trong cuốn sách. Các thuật ngữ được phân nhóm hợp lý, đi kèm giải thích siêu ngắn gọn và chỉ dẫn ngược về chương gốc để bạn dễ dàng tra cứu.

---

## Rust Core

| Term | Giải thích | Chapter |
|------|-----------|---------|
| **Ownership** | Mỗi value có đúng 1 owner. Khi owner out of scope → value drop | Ch 9 |
| **Move** | Chuyển ownership — gốc không dùng được nữa. Ẩn dụ: "tặng quà" | Ch 9 |
| **Borrow** | Mượn reference (`&T` shared, `&mut T` exclusive). Ẩn dụ: "thư viện" | Ch 9, App A |
| **Lifetime** | Compiler track scope của references. `'a` = "sống ít nhất bằng `'a`" | Ch 9, Ch 17 |
| **`'static`** | Sống toàn bộ chương trình. Owned types (String, Vec) thỏa `'static` | Ch 17 |
| **Trait** | "Chứng chỉ kỹ năng" — interface/typeclass. `trait X { fn method(); }` | Ch 16 |
| **Generic** | Code cho mọi type: `fn max<T: Ord>(a: T, b: T) -> T` | Ch 17 |
| **Monomorphization** | Compiler tạo code riêng cho mỗi concrete type → zero-cost | Ch 17 |
| **Enum** | Sum type (OR). `enum Color { Red, Green, Blue }` | Ch 6, Ch 14 |
| **Struct** | Product type (AND). `struct Point { x: f64, y: f64 }` | Ch 14 |
| **Pattern Matching** | `match` exhaustive trên enums. Guards, destructuring | Ch 6, Ch 15 |
| **Closure** | Anonymous function. `|x| x + 1`. Captures environment | Ch 7, Ch 13 |
| **`impl Trait`** | Static dispatch — compiler monomorphize | Ch 16 |
| **`dyn Trait`** | Dynamic dispatch — vtable at runtime. `Box<dyn Trait>` | Ch 16 |
| **Macro** | Code viết code. `macro_rules!` (declarative), `#[derive]` (procedural) | App D |
| **`?` operator** | Propagate errors. `Ok` → unwrap, `Err` → early return | Ch 10 |
| **RAII / Drop** | Resource cleanup khi out of scope. "Dọn phòng tự động" | Ch 9 |
| **`Send`** | Type an toàn chuyển giữa threads | Ch 36 |
| **`Sync`** | Type an toàn chia sẻ references giữa threads | Ch 36 |

---

## FP Concepts

| Term | Giải thích | Chapter |
|------|-----------|---------|
| **Pure Function** | Cùng input → cùng output. Không side-effects | Ch 12 |
| **Immutability** | Giá trị không thay đổi sau tạo. Rust: `let` mặc định immutable | Ch 12 |
| **Higher-Order Function** | Function nhận/trả function. `map`, `filter`, `fold` | Ch 13 |
| **Composition** | Nối functions: `f(g(x))`. Pipeline: `x |> g |> f` | Ch 13, Ch 23 |
| **Algebraic Data Types** | Product (struct, AND) + Sum (enum, OR). Count states | Ch 1, Ch 14 |
| **Functor** | Type có `.map()`. Option, Result, Iterator | Ch 29 |
| **Monad** | Functor + `.and_then()` (flatMap). Chain computations | Ch 30 |
| **Railway-Oriented (ROP)** | Two-track: Ok track + Error track. `?` = switch track | Ch 24 |
| **Currying** | `f(a, b)` → `f(a)(b)`. Partial application | Ch 1, Ch 13 |
| **Lambda Calculus** | Foundation of FP. Function = first-class value. "Máy xay" | Ch 1 |
| **Curry-Howard** | Types ≅ Propositions. Programs ≅ Proofs | Ch 1 |
| **Fold** | Reduce collection to single value. `iter.fold(init, f)` | Ch 13 |
| **Functional Core** | Pure business logic, no I/O. Paired with Imperative Shell | Ch 12 |

---

## DDD Concepts

| Term | Giải thích | Chapter |
|------|-----------|---------|
| **Domain** | Lĩnh vực business. "Map vs Territory" — code phản ánh reality | Ch 20 |
| **Value Object** | Defined by value, not identity. "Tờ 100k₫" | Ch 22 |
| **Entity** | Has unique identity. `UserId(42)` | Ch 22 |
| **Aggregate** | Cluster of entities + value objects. Consistency boundary | Ch 22 |
| **Bounded Context** | Ranh giới ngữ cảnh. "User" có nghĩa khác trong Billing vs Shipping | Ch 20 |
| **Ubiquitous Language** | Cùng thuật ngữ giữa dev và domain expert | Ch 20 |
| **Domain Event** | Something that happened. `OrderPlaced`, `PaymentReceived` | Ch 21 |
| **Workflow / Pipeline** | Chuỗi steps xử lý domain logic. "Dây chuyền" | Ch 23 |
| **Smart Constructor** | Function validate + tạo value. `Email::new("x@y.com")` | Ch 22 |
| **Newtype** | Wrap primitive cho type safety. `struct OrderId(u64)` | Ch 12, Ch 22 |
| **State Machine** | Enum states + transition functions. Illegal transitions = compile error | Ch 15, Ch 22 |
| **CQRS** | Tách Command (write) và Query (read) | Ch 19 |
| **Event Sourcing** | Lưu events, không lưu state. "Video vs Photo" | Ch 19 |

---

## Production Patterns

| Term | Giải thích | Chapter |
|------|-----------|---------|
| **Hexagonal Architecture** | Core (domain) + Ports (traits) + Adapters (implementations) | Ch 35 |
| **Onion Architecture** | Domain ở giữa, I/O ở ngoài. "Hành tây" | Ch 35 |
| **Repository** | Trait abstract database access. `trait UserRepo { ... }` | Ch 17, Ch 35 |
| **DI (Dependency Injection)** | Truyền dependencies qua constructor/params. Testable | Ch 35 |
| **Middleware** | Xử lý trước/sau handler. Logging, auth, rate-limit | Ch 36B |
| **Circuit Breaker** | Ngắt mạch khi service fail liên tục. Closed → Open → Half-open | Ch 42 |
| **Saga** | Distributed transactions. Choreography vs Orchestration | Ch 42 |
| **ACID** | Atomicity, Consistency, Isolation, Durability — transaction properties | Ch 38 |
| **CAP Theorem** | Consistency + Availability + Partition tolerance — pick 2 | Ch 42 |
| **JWT** | JSON Web Token — stateless auth. Header.Payload.Signature | Ch 40 |
| **Rate Limiting** | Giới hạn requests/time. Token bucket, sliding window | Ch 41 |
| **Structured Logging** | JSON logs với fields. `tracing` crate | Ch 44 |
| **thiserror** | Derive macro cho custom error types. Dùng trong libraries | Ch 10 |
| **anyhow** | Catch-all error type. Dùng trong applications | Ch 10 |

---

## Agentic AI Concepts

| Term | Giải thích | Chapter |
|------|-----------|---------|
| **Agent** | Một thực thể AI tự trị, có khả năng dùng Tool và suy luận | Ch 45 |
| **Tool Calling** | Khả năng của LLM tự quyết định gọi function bên ngoài (API/CLI) | Ch 47 |
| **Extractor** | Ép LLM trả về dữ liệu có cấu trúc (thường là JSON schema) | Ch 46 |
| **RAG** | Retrieval-Augmented Generation — cung cấp thêm Context từ DB cho LLM | Ch 49 |
| **MCP** | Model Context Protocol — chuẩn kết nối LLM với external tools | Ch 51 |
| **LTM / STM** | Long-Term Memory / Short-Term Memory của Agent | Ch 54 |
| **Lossless Memory** | Bộ nhớ lưu dữ liệu gốc (Verbatim) thay vì bản tóm tắt bị mất mát | Ch 54 |
| **Incremental Sync** | Chỉ mục gia tăng. Chỉ update phần $\Delta$ bị thay đổi, tiết kiệm cost | Ch 54 |
| **AST-Aware Chunking** | Băm dữ liệu dựa trên Cây cú pháp (AST) để bảo toàn logic của Code | Ch 54 |
| **Dreaming Loop** | Vòng lặp tự tiến hóa của Agent (đánh giá STM, chuyển lên LTM) | Ch 54 |
