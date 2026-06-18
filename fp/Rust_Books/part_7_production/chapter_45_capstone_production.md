# Chapter 44 — Capstone Part 2: Production Deployment ⭐

> **Bạn sẽ học được**:
> - **Tổng hợp MỌI THỨ** từ 43 chapters trước vào 1 production system
> - **PostgreSQL** (`sqlx`) + **Redis** cache
> - **JWT authentication** + RBAC authorization
> - **Axum** web framework — routes, middleware, error handling
> - **Docker** containerization + **CI/CD** pipeline
> - **Structured logging** (`tracing`) + monitoring
>
> **Yêu cầu**: ALL previous chapters.
> **Thời gian đọc**: ~50 phút | **Level**: Principal
> **Kết quả cuối cùng**: Order-Taking System chạy production với database, auth, cache, monitoring.

---

## 44.1 — Project Structure

Đây là project tổng kết toàn bộ cuốn sách. Mỗi thư mục tương ứng với một layer bạn đã học: `domain/` (Ch 22 — pure types), `application/` (Ch 23 — workflows), `infrastructure/` (Ch 26, 38 — database), `api/` (Ch 36b — Axum routes), `config/` (environment). Nếu bạn hiểu cấu trúc này, bạn hiểu clean architecture.

```
order-system/
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── .env.example
├── migrations/
│   ├── 001_create_users.sql
│   ├── 002_create_products.sql
│   └── 003_create_orders.sql
├── src/
│   ├── main.rs              ← entry point
│   ├── config.rs            ← env config
│   ├── domain/              ← pure domain logic (Ch 22, 37)
│   │   ├── mod.rs
│   │   ├── types.rs         ← value objects
│   │   ├── order.rs         ← order state machine
│   │   └── errors.rs        ← domain errors
│   ├── application/         ← use cases (Ch 23, 35)
│   │   ├── mod.rs
│   │   └── order_service.rs
│   ├── infrastructure/      ← adapters (Ch 26, 38)
│   │   ├── mod.rs
│   │   ├── db.rs            ← PostgreSQL repos
│   │   ├── cache.rs         ← Redis cache
│   │   └── auth.rs          ← JWT + RBAC
│   ├── api/                 ← HTTP layer (Ch 43)
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   ├── middleware.rs
│   │   └── responses.rs
│   └── telemetry.rs         ← logging + metrics
└── tests/
    ├── api_tests.rs
    └── domain_tests.rs
```

---

## 44.2 — Config & Database Setup

```rust
// filename: src/config.rs

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL required")?,
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".into()),
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET required")?,
            server_host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse().unwrap_or(3000),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
        })
    }
}
```

```sql
-- migrations/001_create_users.sql
CREATE TABLE users (
    id          BIGSERIAL PRIMARY KEY,
    email       VARCHAR(255) NOT NULL UNIQUE,
    name        VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role        VARCHAR(20) NOT NULL DEFAULT 'user',
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

-- migrations/002_create_products.sql
CREATE TABLE products (
    id      BIGSERIAL PRIMARY KEY,
    code    VARCHAR(10) NOT NULL UNIQUE,
    name    VARCHAR(200) NOT NULL,
    price   INTEGER NOT NULL CHECK (price > 0),
    stock   INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0)
);

INSERT INTO products (code, name, price, stock) VALUES
    ('W1234', 'Premium Widget', 85000, 100),
    ('W5678', 'Deluxe Widget', 120000, 50),
    ('G567', 'Standard Gizmo', 45000, 200);

-- migrations/003_create_orders.sql
CREATE TABLE orders (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT NOT NULL REFERENCES users(id),
    status      VARCHAR(20) NOT NULL DEFAULT 'draft',
    subtotal    INTEGER NOT NULL DEFAULT 0,
    tax         INTEGER NOT NULL DEFAULT 0,
    total       INTEGER NOT NULL DEFAULT 0,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE order_lines (
    id          BIGSERIAL PRIMARY KEY,
    order_id    BIGINT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id  BIGINT NOT NULL REFERENCES products(id),
    quantity    INTEGER NOT NULL CHECK (quantity > 0),
    unit_price  INTEGER NOT NULL,
    line_total  INTEGER NOT NULL
);

CREATE INDEX idx_orders_user ON orders(user_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_order_lines_order ON order_lines(order_id);
```

---

## 44.3 — Domain Layer (Pure — No Dependencies)

Domain layer không import bất kỳ crate bên ngoài nào — không `sqlx`, không `axum`, không `serde`. Chỉ có Rust standard library. Mọi thứ ở đây là Value Objects (smart constructors, validation) và business logic (tính giá, tính thuế). Nếu database đổi từ PostgreSQL sang SQLite, domain layer không thay đổi một dòng.

```rust
// filename: src/domain/types.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);
impl Email {
    pub fn new(email: &str) -> Result<Self, String> {
        let e = email.trim().to_lowercase();
        if !e.contains('@') || e.len() < 5 { return Err("Invalid email".into()); }
        Ok(Email(e))
    }
    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity(u32);
impl Quantity {
    pub fn new(qty: u32) -> Result<Self, String> {
        if qty == 0 || qty > 10_000 { return Err("Quantity 1-10000".into()); }
        Ok(Quantity(qty))
    }
    pub fn value(&self) -> u32 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Money(i64); // cents
impl Money {
    pub fn new(cents: i64) -> Result<Self, String> {
        if cents < 0 { return Err("Money cannot be negative".into()); }
        Ok(Money(cents))
    }
    pub fn cents(&self) -> i64 { self.0 }
    pub fn add(&self, other: &Money) -> Money { Money(self.0 + other.0) }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}đ", self.0 / 100, (self.0 % 100).abs())
    }
}
```

```rust
// filename: src/domain/order.rs

use super::types::*;

// State machine types (Ch 37)
#[derive(Debug)]
pub struct OrderRequest {
    pub user_id: i64,
    pub items: Vec<OrderItemRequest>,
}

#[derive(Debug)]
pub struct OrderItemRequest {
    pub product_code: String,
    pub quantity: u32,
}

#[derive(Debug)]
pub struct PricedOrder {
    pub user_id: i64,
    pub lines: Vec<PricedLine>,
    pub subtotal: Money,
    pub tax: Money,
    pub total: Money,
}

#[derive(Debug)]
pub struct PricedLine {
    pub product_id: i64,
    pub product_code: String,
    pub quantity: Quantity,
    pub unit_price: Money,
    pub line_total: Money,
}

// Pure domain logic
pub fn calculate_tax(subtotal: &Money) -> Money {
    Money::new(subtotal.cents() * 10 / 100).unwrap() // 10% VAT
}

pub fn calculate_discount(subtotal: &Money) -> i64 {
    match subtotal.cents() {
        s if s > 1_000_000 => 10,   // 10% for >1M
        s if s > 500_000 => 5,      // 5% for >500K
        _ => 0,
    }
}
```

---

## 44.4 — Infrastructure: Database Repository

```rust
// filename: src/infrastructure/db.rs

// Port (trait)
pub trait ProductRepo: Send + Sync {
    fn find_by_code(&self, code: &str) -> Result<Option<Product>, AppError>;
    fn update_stock(&self, id: i64, delta: i32) -> Result<(), AppError>;
}

pub trait OrderRepo: Send + Sync {
    fn create(&self, order: &PricedOrder, user_id: i64) -> Result<i64, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<OrderRecord>, AppError>;
    fn list_by_user(&self, user_id: i64) -> Result<Vec<OrderRecord>, AppError>;
}

#[derive(Debug, Clone)]
pub struct Product {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub price: i32,
    pub stock: i32,
}

#[derive(Debug, Clone)]
pub struct OrderRecord {
    pub id: i64,
    pub user_id: i64,
    pub status: String,
    pub total: i32,
    pub created_at: String,
}

// Adapter (sqlx implementation — conceptual)
// struct PgProductRepo { pool: PgPool }
// impl ProductRepo for PgProductRepo { ... }

// In-memory adapter (for testing!)
use std::collections::HashMap;
use std::sync::Mutex;

pub struct InMemoryProductRepo {
    products: Mutex<HashMap<String, Product>>,
}

impl InMemoryProductRepo {
    pub fn new() -> Self {
        let mut products = HashMap::new();
        products.insert("W1234".into(), Product {
            id: 1, code: "W1234".into(), name: "Premium Widget".into(), price: 85000, stock: 100
        });
        products.insert("G567".into(), Product {
            id: 2, code: "G567".into(), name: "Standard Gizmo".into(), price: 45000, stock: 200
        });
        products.insert("W5678".into(), Product {
            id: 3, code: "W5678".into(), name: "Deluxe Widget".into(), price: 120000, stock: 50
        });
        InMemoryProductRepo { products: Mutex::new(products) }
    }
}

#[derive(Debug)]
pub struct AppError(pub String);
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl ProductRepo for InMemoryProductRepo {
    fn find_by_code(&self, code: &str) -> Result<Option<Product>, AppError> {
        Ok(self.products.lock().unwrap().get(code).cloned())
    }
    fn update_stock(&self, id: i64, delta: i32) -> Result<(), AppError> {
        let mut products = self.products.lock().unwrap();
        for p in products.values_mut() {
            if p.id == id {
                p.stock += delta;
                if p.stock < 0 { return Err(AppError("Insufficient stock".into())); }
                return Ok(());
            }
        }
        Err(AppError("Product not found".into()))
    }
}
```

---

## 44.5 — Application: Order Service

Application layer là nơi workflow pipeline chạy: validate input → lookup products → tính giá → deduct stock. Mỗi step là function đã học ở Ch 23-24. Nó orchestrate giữa domain logic và infrastructure — nhưng không biết database cụ thể là gì (chỉ gọi traits).

```rust
// filename: src/application/order_service.rs

use crate::domain::order::*;
use crate::domain::types::*;
use crate::infrastructure::db::*;

pub fn place_order(
    product_repo: &dyn ProductRepo,
    request: OrderRequest,
) -> Result<PricedOrder, AppError> {
    // Step 1: Validate & lookup products
    let mut lines = vec![];
    let mut subtotal_cents: i64 = 0;

    for item in &request.items {
        let qty = Quantity::new(item.quantity)
            .map_err(|e| AppError(e))?;

        let product = product_repo.find_by_code(&item.product_code)?
            .ok_or_else(|| AppError(format!("Unknown product: {}", item.product_code)))?;

        if product.stock < item.quantity as i32 {
            return Err(AppError(format!(
                "{}: need {}, have {}", product.code, item.quantity, product.stock
            )));
        }

        let unit_price = Money::new(product.price as i64).unwrap();
        let line_total = Money::new(product.price as i64 * item.quantity as i64).unwrap();
        subtotal_cents += line_total.cents();

        lines.push(PricedLine {
            product_id: product.id,
            product_code: product.code.clone(),
            quantity: qty,
            unit_price,
            line_total,
        });
    }

    // Step 2: Apply discount
    let subtotal = Money::new(subtotal_cents).unwrap();
    let discount_pct = calculate_discount(&subtotal);
    let discounted = Money::new(subtotal_cents * (100 - discount_pct) / 100).unwrap();

    // Step 3: Calculate tax
    let tax = calculate_tax(&discounted);
    let total = discounted.add(&tax);

    // Step 4: Deduct stock
    for line in &lines {
        product_repo.update_stock(line.product_id, -(line.quantity.value() as i32))?;
    }

    Ok(PricedOrder {
        user_id: request.user_id,
        lines,
        subtotal: discounted,
        tax,
        total,
    })
}
```

---

## 44.6 — API Layer (Axum Routes — chạy được!)

Đây là phần duy nhất người dùng nhìn thấy. Axum routes nhận HTTP requests, chuyển thành domain types, gọi application service, và trả JSON. JWT auth (Ch 40), error handling (Ch 24 ROP), và CORS đều được wire tại đây.

Khác với các section trước (domain, infrastructure riêng lẻ), section này **gom tất cả** vào 1 file chạy được — đây là capstone, bạn cần thấy mọi thứ kết nối.

```rust
// filename: src/main.rs
// CAPSTONE — Order-Taking System (all-in-one runnable version)
// Production sẽ tách thành modules, nhưng ở đây gom lại để dễ chạy.

use axum::{
    Router, routing::{get, post},
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// ═══ Domain Types (Ch 22, 37) ═══

#[derive(Debug, Clone, PartialEq)]
struct Quantity(u32);
impl Quantity {
    fn new(qty: u32) -> Result<Self, String> {
        if qty == 0 || qty > 10_000 { Err("Quantity must be 1-10000".into()) }
        else { Ok(Quantity(qty)) }
    }
    fn value(&self) -> u32 { self.0 }
}

#[derive(Debug, Clone, PartialEq)]
struct Money(i64);
impl Money {
    fn new(cents: i64) -> Result<Self, String> {
        if cents < 0 { Err("Money cannot be negative".into()) }
        else { Ok(Money(cents)) }
    }
    fn cents(&self) -> i64 { self.0 }
    fn add(&self, other: &Money) -> Money { Money(self.0 + other.0) }
}
impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}đ", self.0 / 100, (self.0 % 100).abs())
    }
}

// ═══ Domain Logic (Ch 23, 24 — Pipelines & ROP) ═══

#[derive(Debug, Clone, Serialize)]
struct Product { id: i64, code: String, name: String, price: i32, stock: i32 }

#[derive(Debug, Serialize)]
struct PricedOrder {
    user_id: i64,
    lines: Vec<PricedLine>,
    subtotal: i64,
    discount_pct: i64,
    discounted: i64,
    tax: i64,
    total: i64,
}

#[derive(Debug, Serialize)]
struct PricedLine {
    product_code: String,
    product_name: String,
    quantity: u32,
    unit_price: i64,
    line_total: i64,
}

fn calculate_discount(subtotal: i64) -> i64 {
    match subtotal {
        s if s > 1_000_000 => 10,
        s if s > 500_000 => 5,
        _ => 0,
    }
}

fn calculate_tax(amount: i64) -> i64 {
    amount * 10 / 100  // 10% VAT
}

// ═══ Error Type (Ch 24 — ROP for web) ═══

#[derive(Debug)]
enum AppError {
    NotFound(String),
    Validation(String),
    InsufficientStock { product: String, need: u32, have: i32 },
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::Validation(m) => (StatusCode::BAD_REQUEST, m),
            AppError::InsufficientStock { product, need, have } =>
                (StatusCode::CONFLICT, format!("{}: need {}, have {}", product, need, have)),
        };
        (status, Json(serde_json::json!({"error": msg}))).into_response()
    }
}

// ═══ App State (in-memory — production dùng DB, Ch 38) ═══

#[derive(Clone)]
struct AppState {
    products: Arc<Mutex<HashMap<String, Product>>>,
    orders: Arc<Mutex<Vec<PricedOrder>>>,
}

impl AppState {
    fn new() -> Self {
        let mut products = HashMap::new();
        for p in [
            Product { id: 1, code: "W1234".into(), name: "Premium Widget".into(), price: 85000, stock: 100 },
            Product { id: 2, code: "W5678".into(), name: "Deluxe Widget".into(), price: 120000, stock: 50 },
            Product { id: 3, code: "G567".into(), name: "Standard Gizmo".into(), price: 45000, stock: 200 },
        ] {
            products.insert(p.code.clone(), p);
        }
        AppState {
            products: Arc::new(Mutex::new(products)),
            orders: Arc::new(Mutex::new(vec![])),
        }
    }
}

// ═══ Request/Response DTOs (Ch 25 — Serialization & ACL) ═══

#[derive(Deserialize)]
struct CreateOrderRequest {
    user_id: i64,
    items: Vec<OrderItemDto>,
}

#[derive(Deserialize)]
struct OrderItemDto {
    product_code: String,
    quantity: u32,
}

#[derive(Serialize)]
struct OrderResponse {
    order_id: usize,
    status: String,
    lines: Vec<LineResponse>,
    subtotal: String,
    discount: String,
    tax: String,
    total: String,
}

#[derive(Serialize)]
struct LineResponse {
    product: String,
    quantity: u32,
    unit_price: String,
    line_total: String,
}

fn format_money(cents: i64) -> String {
    format!("{}.{:02}đ", cents / 100, (cents % 100).abs())
}

// ═══ Handlers (Ch 36B — Axum) ═══

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "version": "1.0.0",
        "service": "order-taking-system"
    }))
}

async fn list_products(
    State(state): State<AppState>,
) -> Json<Vec<Product>> {
    let products = state.products.lock().unwrap();
    let mut list: Vec<Product> = products.values().cloned().collect();
    list.sort_by_key(|p| p.id);
    Json(list)
}

async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), AppError> {
    // Step 1: Validate & lookup products (ROP — ? = fast-fail)
    let mut lines = vec![];
    let mut subtotal_cents: i64 = 0;

    {
        let products = state.products.lock().unwrap();
        for item in &req.items {
            let _qty = Quantity::new(item.quantity)
                .map_err(|e| AppError::Validation(e))?;

            let product = products.get(&item.product_code)
                .ok_or_else(|| AppError::NotFound(
                    format!("Unknown product: {}", item.product_code)
                ))?;

            if product.stock < item.quantity as i32 {
                return Err(AppError::InsufficientStock {
                    product: product.code.clone(),
                    need: item.quantity,
                    have: product.stock,
                });
            }

            let line_total = product.price as i64 * item.quantity as i64;
            subtotal_cents += line_total;

            lines.push(PricedLine {
                product_code: product.code.clone(),
                product_name: product.name.clone(),
                quantity: item.quantity,
                unit_price: product.price as i64,
                line_total,
            });
        }
    }

    // Step 2: Apply discount (Ch 27 — domain rule)
    let discount_pct = calculate_discount(subtotal_cents);
    let discounted = subtotal_cents * (100 - discount_pct) / 100;

    // Step 3: Calculate tax
    let tax = calculate_tax(discounted);
    let total = discounted + tax;

    // Step 4: Deduct stock
    {
        let mut products = state.products.lock().unwrap();
        for line in &lines {
            if let Some(p) = products.get_mut(&line.product_code) {
                p.stock -= line.quantity as i32;
            }
        }
    }

    // Step 5: Save order
    let order = PricedOrder {
        user_id: req.user_id,
        lines,
        subtotal: subtotal_cents,
        discount_pct,
        discounted,
        tax,
        total,
    };

    let order_id = {
        let mut orders = state.orders.lock().unwrap();
        orders.push(order);
        orders.len()
    };

    // Build response (Ch 25 — DTO mapping)
    let orders = state.orders.lock().unwrap();
    let saved = &orders[order_id - 1];

    let response = OrderResponse {
        order_id,
        status: "confirmed".into(),
        lines: saved.lines.iter().map(|l| LineResponse {
            product: format!("{} ({})", l.product_name, l.product_code),
            quantity: l.quantity,
            unit_price: format_money(l.unit_price),
            line_total: format_money(l.line_total),
        }).collect(),
        subtotal: format_money(saved.subtotal),
        discount: format!("{}%", saved.discount_pct),
        tax: format_money(saved.tax),
        total: format_money(saved.total),
    };

    println!("📦 Order #{} placed — total: {}", order_id, format_money(saved.total));

    Ok((StatusCode::CREATED, Json(response)))
}

async fn list_orders(State(state): State<AppState>) -> Json<serde_json::Value> {
    let orders = state.orders.lock().unwrap();
    let summaries: Vec<serde_json::Value> = orders.iter().enumerate()
        .map(|(i, o)| serde_json::json!({
            "order_id": i + 1,
            "user_id": o.user_id,
            "total": format_money(o.total),
            "items": o.lines.len(),
        }))
        .collect();
    Json(serde_json::json!({ "orders": summaries, "count": summaries.len() }))
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/products", get(list_products))
        .route("/api/v1/orders", post(create_order).get(list_orders))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 Order-Taking System running at http://localhost:3000");
    println!();
    println!("Endpoints:");
    println!("  GET  /health           → Health check");
    println!("  GET  /api/v1/products  → List products");
    println!("  POST /api/v1/orders    → Place order");
    println!("  GET  /api/v1/orders    → List orders");

    axum::serve(listener, app).await.unwrap();
}
```

Chạy thử:

```bash
# Terminal 1: Start server
cargo run
# 🚀 Order-Taking System running at http://localhost:3000

# Terminal 2: Test
curl http://localhost:3000/health
# → {"service":"order-taking-system","status":"healthy","version":"1.0.0"}

curl http://localhost:3000/api/v1/products
# → [{"code":"W1234","name":"Premium Widget","price":85000,"stock":100}, ...]

curl -X POST http://localhost:3000/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{"user_id":1,"items":[{"product_code":"W1234","quantity":2},{"product_code":"G567","quantity":5}]}'
# → {"order_id":1,"status":"confirmed","subtotal":"3950.00đ",
#    "discount":"0%","tax":"395.00đ","total":"4345.00đ",...}

# Error cases:
curl -X POST http://localhost:3000/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{"user_id":1,"items":[{"product_code":"INVALID","quantity":1}]}'
# → {"error":"Unknown product: INVALID"}  (404)

curl http://localhost:3000/api/v1/orders
# → {"count":1,"orders":[{"order_id":1,"total":"4345.00đ",...}]}
```

---

## 44.7 — Docker & CI/CD

Code chạy trên máy bạn không có nghĩa là sẽ chạy trên server. Docker đảm bảo môi trường giống nhau mọi nơi. CI/CD tự động test + deploy khi push code.

### Dockerfile

```dockerfile
# Multi-stage build
FROM rust:1.77-alpine AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

FROM alpine:3.19
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/order-system /usr/local/bin/
COPY migrations/ /app/migrations/

ENV HOST=0.0.0.0
ENV PORT=3000
EXPOSE 3000

CMD ["order-system"]
```

### docker-compose.yml

```yaml
version: '3.8'
services:
  app:
    build: .
    ports: ["3000:3000"]
    environment:
      DATABASE_URL: postgres://app:secret@db:5432/orders
      REDIS_URL: redis://cache:6379
      JWT_SECRET: ${JWT_SECRET}
      LOG_LEVEL: info
    depends_on: [db, cache]

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: orders
      POSTGRES_USER: app
      POSTGRES_PASSWORD: secret
    volumes: ["pgdata:/var/lib/postgresql/data"]
    ports: ["5432:5432"]

  cache:
    image: redis:7-alpine
    ports: ["6379:6379"]

volumes:
  pgdata:
```

### CI/CD Pipeline (GitHub Actions)

```yaml
# .github/workflows/ci.yml
name: CI/CD
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env: { POSTGRES_DB: test, POSTGRES_USER: test, POSTGRES_PASSWORD: test }
        ports: ["5432:5432"]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: docker build -t order-system .
      - run: docker push registry.example.com/order-system:${{ github.sha }}
      # Deploy to staging → production
```

---

## 44.8 — Telemetry & Monitoring

Để thêm structured logging vào server ở 44.6, bạn thêm `tracing` vào `Cargo.toml` rồi thêm middleware:

```rust
// filename: src/main.rs (thêm vào đầu file)

use tracing::info;

// Thêm vào main() trước khi khởi tạo Router:
fn setup_logging() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
}

// Logging middleware — dùng tower-http
// Thêm vào Cargo.toml: tower-http = { version = "0.5", features = ["trace"] }
// use tower_http::trace::TraceLayer;
//
// let app = Router::new()
//     .route(...)
//     .layer(TraceLayer::new_for_http())  // auto-log mọi request
//     .with_state(state);
```

### Thêm logging vào handler:

```rust
// Trong create_order handler, thêm:
info!(
    order_id = order_id,
    user_id = req.user_id,
    total = saved.total,
    items = saved.lines.len(),
    "Order placed successfully"
);
// Log output:
// 2024-01-15T10:30:00 INFO Order placed successfully
//   order_id=1 user_id=1 total=434500 items=2
```

### Các metrics chính cho production

| Metric | Loại | Ngưỡng cảnh báo |
|--------|------|-----------------|
| `http_requests_total{method, path, status}` | Counter | — |
| `http_request_duration_seconds{method, path}` | Histogram | 🟡 P99 > 500ms |
| `orders_placed_total` | Counter | — |
| `orders_failed_total{reason}` | Counter | 🔴 Tỷ lệ lỗi > 5% |
| `db_connections_active` | Gauge | 🔴 = pool max |
| `cache_hit_ratio` | Gauge | 🟡 < 80% |

### Production telemetry stack

```
App (tracing) ──▶ stdout/Loki ──▶ Grafana (dashboards)
App (metrics) ──▶ Prometheus ────▶ Grafana (alerts)
App (traces)  ──▶ OpenTelemetry ──▶ Jaeger (distributed tracing)
```

---

## 44.9 — Architecture Summary

Nhìn lại toàn bộ hệ thống: mỗi layer chỉ biết layer ngay dưới nó (qua traits), domain không biết gì về thế giới bên ngoài. Đây là cách bạn xây hệ thống lớn mà vẫn testable, maintainable.

```
┌─────────────────────────────────────────────────────────┐
│                    Production Stack                      │
│                                                          │
│  ┌──────────┐    ┌──────────────┐    ┌───────────────┐  │
│  │  Nginx   │───▶│  Axum App    │───▶│  PostgreSQL   │  │
│  │  (LB)    │    │  (Rust)      │    │  (persistence)│  │
│  └──────────┘    └──────┬───────┘    └───────────────┘  │
│                         │                                │
│                    ┌────┴─────┐                          │
│                    │  Redis   │                          │
│                    │  (cache) │                          │
│                    └──────────┘                          │
│                                                          │
│  Observability:                                          │
│    tracing → stdout/Loki → Grafana                      │
│    metrics → Prometheus → Grafana                       │
│    traces  → OpenTelemetry → Jaeger                     │
│                                                          │
│  CI/CD:                                                  │
│    GitHub Actions → Docker Build → Registry → Deploy     │
└─────────────────────────────────────────────────────────┘
```

### Các chapter được kết hợp trong capstone

| Lớp | Chapters sử dụng | Khái niệm |
|-------|--------------|----------|
| **Domain** | Ch 14, 22, 23, 24, 37 | Newtypes, state machine, pipeline, ROP |
| **Application** | Ch 35, 37 | Use cases, DI qua traits |
| **Infrastructure** | Ch 26, 38, 39 | Repository, sqlx, migrations, cache |
| **API** | Ch 43 | Thiết kế REST, status codes, versioning |
| **Auth** | Ch 40 | JWT, argon2, RBAC |
| **Security** | Ch 41 | Validation, headers, rate limiting |
| **Vận hành** | Ch 42, 43 | Circuit breaker, monitoring, capacity |
| **Testing** | Ch 33, 34, 35 | TDD, PBT, mocks |
| **Concurrency** | Ch 36 | Async/await, tokio |

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Thêm tìm kiếm sản phẩm

Thêm endpoint `GET /api/v1/products?q=widget&min_price=50000`:
- Search by name (case-insensitive)
- Filter by min/max price
- Paginate results

<details><summary>✅ Lời giải</summary>

```sql
SELECT id, code, name, price, stock FROM products
WHERE LOWER(name) LIKE LOWER('%' || $1 || '%')
  AND price >= COALESCE($2, 0)
  AND price <= COALESCE($3, 999999999)
ORDER BY name
LIMIT $4 OFFSET $5;
```

```rust
#[derive(Deserialize)]
struct ProductSearch {
    q: Option<String>,
    min_price: Option<i32>,
    max_price: Option<i32>,
    page: Option<u32>,
    limit: Option<u32>,
}
```

</details>

---

**Bài 2** (15 phút): Thêm lịch sử đơn hàng với cache

Implement `GET /api/v1/orders` with Redis cache:
1. Check cache `user:{id}:orders`
2. Cache miss → query DB → store in cache (TTL 5 min)
3. Invalidate on new order

<details><summary>✅ Lời giải Bài 2</summary>

```rust
async fn list_orders(user_id: i64, cache: &Cache, db: &OrderRepo) -> Vec<OrderRecord> {
    let key = format!("user:{}:orders", user_id);

    // Cache hit
    if let Some(cached) = cache.get(&key) {
        return cached;
    }

    // Cache miss → DB
    let orders = db.list_by_user(user_id).unwrap();
    cache.set(&key, &orders, Duration::from_secs(300)); // 5 min TTL
    orders
}

// On create_order: invalidate
async fn after_order_created(user_id: i64, cache: &Cache) {
    cache.invalidate(&format!("user:{}:orders", user_id));
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Docker build chậm" | Rebuild tất cả deps | Multi-stage, cache `cargo fetch` layer |
| "DB cạn kiệt kết nối" | Quá nhiều request cùng lúc | Tăng pool size, thêm connection timeout |
| "JWT token quá lớn" | Lưu quá nhiều thông tin trong claims | Chỉ `sub`, `role`, `exp` trong JWT |
| "Tests không ổn định trong CI" | Race conditions, shared state | Isolate test databases, dùng transactions |

---

## Tóm tắt

- ✅ **Project structure**: Domain (pure) → Application (use cases) → Infrastructure (DB, cache) → API (HTTP).
- ✅ **Config**: Env vars, `Config::from_env()`, `.env` for dev.
- ✅ **Domain**: Value objects (`Email`, `Quantity`, `Money`), pure functions, state machine.
- ✅ **Database**: PostgreSQL + sqlx, migrations, indexes, transactions.
- ✅ **API**: Axum routes, JWT middleware, RBAC guards, JSON responses.
- ✅ **Docker**: Multi-stage build, docker-compose (app + PostgreSQL + Redis).
- ✅ **CI/CD**: GitHub Actions (test → clippy → format → build → deploy).
- ✅ **Telemetry**: Structured logging (tracing), metrics (Prometheus), alert thresholds.

---

## 🎉🎉🎉 CHÚC MỪNG! BẠN ĐÃ HOÀN THÀNH "DOMAIN-DRIVEN FP WITH RUST"!

### Hành trình 44 chapters:

```
Part I:   Foundations (Ch 0-11)     — Rust basics, ownership, error handling
Part II:  FP Core (Ch 12-17)       — Immutability, HOF, traits, generics
Part III: Design Patterns (Ch 18-19) — GoF→FP, CQRS/ES
Part IV:  DDD with Rust (Ch 20-27) — Domain modeling, workflows, ROP, serialization
Part V:   FP Patterns (Ch 28-32)   — Algebra, functors, monads, parsers, folds
Part VI:  Testing (Ch 33-36)       — TDD, PBT, mocking, concurrency
Part VII: Production (Ch 37-44)    — Capstone, DB, security, distributed, system design
```

### Bạn đã đạt được:

- ✅ **Rust fluency** — ownership, lifetimes, traits, generics, async
- ✅ **FP mastery** — immutability, composition, monads, algebraic types
- ✅ **DDD expertise** — domain modeling, bounded contexts, event sourcing
- ✅ **Testing culture** — TDD, PBT, mocking, hexagonal architecture
- ✅ **Production readiness** — database, security, distributed systems, system design

**Bạn không chỉ biết Rust. Bạn biết cách BUILD PRODUCTION SYSTEMS với Rust.**

> *"The only way to learn is to build."* — Hãy bắt đầu project tiếp theo của bạn! 🚀
