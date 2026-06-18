# Appendix C — Cargo & Ecosystem Guide

> Essential crates cho mỗi domain. Curated, battle-tested, production-ready.

---

## C.1 — Core Language & Error Handling

| Crate | Mô tả | Dùng khi |
|-------|--------|----------|
| **`thiserror`** | Derive `Error` trait cho custom errors | Library code |
| **`anyhow`** | Ergonomic error handling (`anyhow::Result`) | Application code |
| **`color-eyre`** | Pretty error reports + backtraces | CLI apps |
| **`tracing`** | Structured logging + spans | Mọi project |
| **`tracing-subscriber`** | Logging output (stdout, JSON, file) | Mọi project |

```toml
# Cargo.toml — recommended starter
[dependencies]
thiserror = "2"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

---

## C.2 — Serialization & Data

| Crate | Mô tả | Dùng khi |
|-------|--------|----------|
| **`serde`** | Serialize/Deserialize framework | Mọi project |
| **`serde_json`** | JSON support | API, config |
| **`toml`** | TOML support | Config files |
| **`csv`** | CSV reader/writer | Data processing |
| **`chrono`** | Date/time handling | Timestamps |
| **`uuid`** | UUID generation | Entity IDs |

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
```

---

## C.3 — Web & HTTP

| Crate | Mô tả | Dùng khi |
|-------|--------|----------|
| **`axum`** | Web framework (tokio-based) | API servers ⭐ |
| **`actix-web`** | Web framework (actor-based) | High-performance APIs |
| **`reqwest`** | HTTP client | External API calls |
| **`tower`** | Middleware framework | Axum middleware |
| **`tower-http`** | HTTP middleware (CORS, compression) | Production hardening |
| **`hyper`** | Low-level HTTP | Custom protocols |

```toml
[dependencies]
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "compression-gzip"] }
reqwest = { version = "0.12", features = ["json"] }
```

---

## C.4 — Database

| Crate | Mô tả | Dùng khi |
|-------|--------|----------|
| **`sqlx`** | Compile-time checked SQL (async) | PostgreSQL, SQLite ⭐ |
| **`diesel`** | Type-safe query builder (sync) | Complex queries |
| **`sea-orm`** | Active Record ORM (async) | Rapid development |
| **`redis`** | Redis client | Caching, sessions |
| **`deadpool`** | Connection pooling | Production DB |

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "chrono", "uuid"] }
redis = { version = "0.25", features = ["tokio-comp"] }
```

---

## C.5 — Async Runtime

| Crate | Mô tả | Dùng khi |
|-------|--------|----------|
| **`tokio`** | Async runtime | Mọi async project ⭐ |
| **`futures`** | Future utilities | Stream, select, join |
| **`async-trait`** | Async methods trong traits | Trait-based DI |

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
futures = "0.3"
async-trait = "0.1"
```

---

## C.6 — Testing

| Crate | Mô tả | Dùng khi |
|-------|--------|----------|
| **`proptest`** | Property-based testing | Finding edge cases ⭐ |
| **`mockall`** | Auto-generate mocks | Trait-based mocking |
| **`assert_cmd`** | Test CLI apps | Integration tests |
| **`wiremock`** | HTTP mock server | Testing HTTP clients |
| **`fake`** | Fake data generation | Test fixtures |
| **`rstest`** | Parameterized tests | Table-driven tests |

```toml
[dev-dependencies]
proptest = "1"
mockall = "0.12"
rstest = "0.18"
fake = { version = "2", features = ["derive"] }
```

---

## C.7 — Security

| Crate | Mô tả | Dùng khi |
|-------|--------|----------|
| **`argon2`** | Password hashing | Auth ⭐ |
| **`jsonwebtoken`** | JWT encode/decode | Token auth |
| **`oauth2`** | OAuth 2.0 client | Social login |
| **`rustls`** | TLS implementation | HTTPS |
| **`tower-sessions`** | Session middleware | Cookie sessions |

```toml
[dependencies]
argon2 = "0.5"
jsonwebtoken = "9"
```

---

## C.8 — CLI & Parsing

| Crate | Mô tả | Dùng khi |
|-------|--------|----------|
| **`clap`** | CLI argument parser | CLI apps ⭐ |
| **`nom`** | Parser combinators (macro) | Binary/text parsing |
| **`chumsky`** | Parser combinators (type) | Error-rich parsing |
| **`regex`** | Regular expressions | Text matching |
| **`dialoguer`** | Interactive prompts | CLI UX |
| **`indicatif`** | Progress bars | Long operations |

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
nom = "7"
regex = "1"
```

---

## C.9 — DevOps & Observability

| Crate | Mô tả | Dùng khi |
|-------|--------|----------|
| **`opentelemetry`** | Distributed tracing | Microservices |
| **`metrics`** | Application metrics | Production monitoring |
| **`prometheus`** | Prometheus exporter | Grafana dashboards |
| **`dotenvy`** | Load `.env` files | Development config |

---

## C.10 — Cargo Commands Cheat Sheet

```bash
# ═══ Project ═══
cargo new myapp              # Binary project
cargo new mylib --lib        # Library project
cargo init                   # Init in current dir

# ═══ Build & Run ═══
cargo run                    # Debug build + run
cargo run --release          # Release build + run
cargo build                  # Build only
cargo build --release        # Optimized build

# ═══ Test ═══
cargo test                   # All tests
cargo test test_name         # Specific test
cargo test -- --nocapture    # Show println! output
cargo test --lib             # Only unit tests
cargo test --test api_tests  # Specific integration test

# ═══ Quality ═══
cargo clippy                 # Linter (warnings + suggestions)
cargo fmt                    # Auto-format
cargo doc --open             # Generate + open docs
cargo audit                  # Security audit dependencies
cargo outdated               # Check for newer versions

# ═══ Dependencies ═══
cargo add serde              # Add dependency
cargo add tokio -F full      # Add with features
cargo add proptest --dev     # Add dev dependency
cargo update                 # Update Cargo.lock
cargo tree                   # Dependency tree
```
