# Chapter 44 — Capstone Part 2: Production Deployment ⭐

> **Bạn sẽ học được**:
> - **Tổng hợp MỌI THỨ** từ 44 chapters trước vào 1 production system
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

Đây là project tổng kết toàn bộ cuốn sách. Từng dòng code, từng thư mục đều mang ý nghĩa triết học mà chúng ta đã rèn luyện. Mỗi thư mục tương ứng với một lớp (layer) kiến trúc: `domain/` (chỉ chứa pure types, không side-effect), `application/` (nơi định nghĩa các use cases/workflows), `infrastructure/` (giao tiếp DB, cache, Auth), và `api/` (nơi Axum xử lý HTTP). 
Hiểu được cấu trúc này, bạn đã thực sự làm chủ Clean Architecture.

```text
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
│   ├── domain/              ← pure domain logic
│   │   ├── mod.rs
│   │   ├── types.rs         ← value objects
│   │   ├── order.rs         ← order state machine
│   │   └── errors.rs        ← domain errors
│   ├── application/         ← use cases
│   │   ├── mod.rs
│   │   └── order_service.rs
│   ├── infrastructure/      ← adapters
│   │   ├── mod.rs
│   │   ├── db.rs            ← PostgreSQL repos
│   │   ├── cache.rs         ← Redis cache
│   │   └── auth.rs          ← JWT + RBAC
│   ├── api/                 ← HTTP layer
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

Trước khi viết logic, chúng ta cần một nền móng vững chắc: cấu hình môi trường và cơ sở dữ liệu.
`Config` được load trực tiếp từ biến môi trường (Environment variables) để tuân thủ nguyên tắc 12-Factor App.

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

Và dưới đây là file migration cho PostgreSQL, sử dụng `sqlx` để tự động tạo schema:

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

## 44.3 — Domain Layer: Trái tim của hệ thống

Domain layer là nơi **bất khả xâm phạm**. Nó không được phép biết bất kỳ điều gì về cơ sở dữ liệu (`sqlx`), framework web (`axum`), hay thậm chí là cách parse JSON (`serde`). Nó chỉ chứa các `struct`, `enum` và logic tính toán thuần túy. Nếu công ty bạn quyết định vứt bỏ PostgreSQL để chuyển sang MongoDB, thư mục này không được phép thay đổi dù chỉ một dòng!

Chúng ta sử dụng Value Objects để đảm bảo dữ liệu luôn hợp lệ ngay từ lúc sinh ra (Parse, don't validate).

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
pub struct Money(i64); // Lưu bằng đơn vị nhỏ nhất (cents) để tránh sai số dấu phẩy động
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

Và đây là logic tính thuế và giảm giá. Chú ý rằng các hàm này hoàn toàn là pure functions (không có side effect):

```rust
// filename: src/domain/order.rs
use super::types::*;

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

// Logic tính thuế 10%
pub fn calculate_tax(subtotal: &Money) -> Money {
    Money::new(subtotal.cents() * 10 / 100).unwrap() 
}

// Logic giảm giá theo mốc tiền
pub fn calculate_discount(subtotal: &Money) -> i64 {
    match subtotal.cents() {
        s if s > 1_000_000 => 10,   // 10% for >1M
        s if s > 500_000 => 5,      // 5% for >500K
        _ => 0,
    }
}
```

---

## 44.4 — Infrastructure: Cầu nối ra thế giới thực

Ở đây chúng ta định nghĩa các Interfaces (Traits) (hay gọi là Port theo ngôn ngữ của Hexagonal Architecture). Lớp Application sẽ gọi các traits này, còn lớp Infrastructure sẽ cung cấp implement thực tế cho nó.

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

#[derive(Debug)]
pub struct AppError(pub String);
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}
```

Để tiện cho việc test và chạy độc lập (không cần setup PostgreSQL phức tạp), ta sẽ viết nhanh một Adapter In-Memory:

```rust
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
        InMemoryProductRepo { products: Mutex::new(products) }
    }
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

## 44.5 — Application: Dàn nhạc trưởng

Application layer (hay Use Cases layer) điều phối toàn bộ workflow: xác thực đầu vào → truy vấn DB → tính toán giá → cập nhật DB. Nhờ thiết kế theo Railway Oriented Programming, các lỗi được bắt và trả về sớm (fast-fail) thông qua toán tử `?`.

```rust
// filename: src/application/order_service.rs

use crate::domain::order::*;
use crate::domain::types::*;
use crate::infrastructure::db::*;

pub fn place_order(
    product_repo: &dyn ProductRepo,
    request: OrderRequest,
) -> Result<PricedOrder, AppError> {
    
    // Bước 1: Validate đầu vào & truy vấn sản phẩm
    let mut lines = vec![];
    let mut subtotal_cents: i64 = 0;

    for item in &request.items {
        let qty = Quantity::new(item.quantity).map_err(|e| AppError(e))?;
        let product = product_repo.find_by_code(&item.product_code)?
            .ok_or_else(|| AppError(format!("Unknown product: {}", item.product_code)))?;

        if product.stock < item.quantity as i32 {
            return Err(AppError(format!("{}: need {}, have {}", product.code, item.quantity, product.stock)));
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

    // Bước 2: Tính giảm giá
    let subtotal = Money::new(subtotal_cents).unwrap();
    let discount_pct = calculate_discount(&subtotal);
    let discounted = Money::new(subtotal_cents * (100 - discount_pct) / 100).unwrap();

    // Bước 3: Tính thuế
    let tax = calculate_tax(&discounted);
    let total = discounted.add(&tax);

    // Bước 4: Trừ kho (Side effect)
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

## 44.6 — API Layer (Tất cả hội tụ ở đây)

Đây là nơi mọi thứ được đóng gói thành một server HTTP thực thụ chạy bằng thư viện **Axum**. Để bạn dễ hình dung, chúng ta sẽ gộp tất cả DTOs, Error handler, State và Router vào cùng một file `main.rs`. 

### 44.6.1: Khai báo Data Transfer Objects (DTO) và Error Handler

Chúng ta cần DTOs (Data Transfer Objects) vì cấu trúc dữ liệu của API (đầu ra JSON) và cấu trúc Domain bên trong thường không giống nhau. DTOs là tấm khiên che chắn cho Domain.

```rust
// filename: src/main.rs (Phần 1)
use axum::{
    Router, routing::{get, post},
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// --- DTOs ---
#[derive(Deserialize)]
struct CreateOrderRequest { user_id: i64, items: Vec<OrderItemDto> }

#[derive(Deserialize)]
struct OrderItemDto { product_code: String, quantity: u32 }

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

// Tiện ích format tiền
fn format_money(cents: i64) -> String {
    format!("{}.{:02}đ", cents / 100, (cents % 100).abs())
}

// --- App Error Handler ---
// Giúp tự động map lỗi của hệ thống thành HTTP Status Codes!
#[derive(Debug)]
enum HttpError {
    NotFound(String),
    Validation(String),
    InsufficientStock { product: String, need: u32, have: i32 },
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            HttpError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            HttpError::Validation(m) => (StatusCode::BAD_REQUEST, m),
            HttpError::InsufficientStock { product, need, have } =>
                (StatusCode::CONFLICT, format!("{}: need {}, have {}", product, need, have)),
        };
        (status, Json(serde_json::json!({"error": msg}))).into_response()
    }
}
```

### 44.6.2: Khởi tạo Application State (Bộ nhớ tạm)

Để mô phỏng database mà không cần cài PostgreSQL, ta dùng `Arc<Mutex<HashMap>>` cho an toàn luồng (thread-safe):

```rust
// filename: src/main.rs (Phần 2)

#[derive(Debug, Clone, Serialize)]
struct ProductEntity { id: i64, code: String, name: String, price: i32, stock: i32 }

#[derive(Clone)]
struct AppState {
    products: Arc<Mutex<HashMap<String, ProductEntity>>>,
    // Giả lập bảng orders trong DB
    orders: Arc<Mutex<Vec<serde_json::Value>>>, 
}

impl AppState {
    fn new() -> Self {
        let mut products = HashMap::new();
        for p in [
            ProductEntity { id: 1, code: "W1234".into(), name: "Premium Widget".into(), price: 85000, stock: 100 },
            ProductEntity { id: 2, code: "G567".into(), name: "Standard Gizmo".into(), price: 45000, stock: 200 },
        ] {
            products.insert(p.code.clone(), p);
        }
        AppState {
            products: Arc::new(Mutex::new(products)),
            orders: Arc::new(Mutex::new(vec![])),
        }
    }
}
```

### 44.6.3: Axum Route Handlers

Đây là cầu nối nhận HTTP Request, bóc tách JSON, xử lý logic (hoặc gọi Application Service) và trả về HTTP Response.

```rust
// filename: src/main.rs (Phần 3)

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "version": "1.0.0",
        "service": "order-taking-system"
    }))
}

async fn list_products(State(state): State<AppState>) -> Json<Vec<ProductEntity>> {
    let products = state.products.lock().unwrap();
    let mut list: Vec<ProductEntity> = products.values().cloned().collect();
    list.sort_by_key(|p| p.id);
    Json(list)
}

async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), HttpError> {
    // 1. Kiểm tra tồn kho và lấy giá sản phẩm
    let mut lines = vec![];
    let mut subtotal_cents: i64 = 0;

    {
        let products = state.products.lock().unwrap();
        for item in &req.items {
            if item.quantity == 0 { return Err(HttpError::Validation("Qty > 0".into())); }
            let product = products.get(&item.product_code)
                .ok_or_else(|| HttpError::NotFound(format!("Unknown: {}", item.product_code)))?;

            if product.stock < item.quantity as i32 {
                return Err(HttpError::InsufficientStock {
                    product: product.code.clone(), need: item.quantity, have: product.stock,
                });
            }

            let line_total = product.price as i64 * item.quantity as i64;
            subtotal_cents += line_total;
            lines.push((product.clone(), item.quantity, line_total));
        }
    }

    // 2. Tính toán (Giả lập gọi Domain function)
    let discount_pct = if subtotal_cents > 1_000_000 { 10 } else { 0 };
    let discounted = subtotal_cents * (100 - discount_pct) / 100;
    let tax = discounted * 10 / 100;
    let total = discounted + tax;

    // 3. Trừ kho
    {
        let mut products = state.products.lock().unwrap();
        for line in &lines {
            if let Some(p) = products.get_mut(&line.0.code) {
                p.stock -= line.1 as i32;
            }
        }
    }

    // 4. Định dạng Response
    let mut res_lines = vec![];
    for (p, qty, ltotal) in lines {
        res_lines.push(LineResponse {
            product: format!("{} ({})", p.name, p.code),
            quantity: qty,
            unit_price: format_money(p.price as i64),
            line_total: format_money(ltotal),
        });
    }

    let response = OrderResponse {
        order_id: state.orders.lock().unwrap().len() + 1,
        status: "confirmed".into(),
        lines: res_lines,
        subtotal: format_money(subtotal_cents),
        discount: format!("{}%", discount_pct),
        tax: format_money(tax),
        total: format_money(total),
    };
    
    // Lưu tạm vào state
    state.orders.lock().unwrap().push(serde_json::to_value(&response).unwrap());

    println!("📦 Order #{} placed — total: {}", response.order_id, format_money(total));
    Ok((StatusCode::CREATED, Json(response)))
}
```

### 44.6.4: Khởi động Server (main)

Cuối cùng, chúng ta kết nối router lại và nổ máy!

```rust
// filename: src/main.rs (Phần 4)

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/products", get(list_products))
        .route("/api/v1/orders", post(create_order))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 Order-Taking System running at http://localhost:3000");
    println!("  GET  /health           → Health check");
    println!("  GET  /api/v1/products  → List products");
    println!("  POST /api/v1/orders    → Place order");

    axum::serve(listener, app).await.unwrap();
}
```

Hãy mở Terminal lên, chạy `cargo run` và dùng lệnh `curl` để gọi API thử xem:

```bash
curl -X POST http://localhost:3000/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{"user_id":1,"items":[{"product_code":"W1234","quantity":2}]}'
```

---

## 44.7 — Docker & CI/CD

Code chạy trên máy bạn không có nghĩa là sẽ chạy trên server. Docker đảm bảo môi trường giống nhau mọi nơi. CI/CD tự động test và build khi bạn đẩy code.

### Dockerfile (Multi-stage)

Kỹ thuật Multi-stage build của Rust giúp image cuối cùng cực kỳ nhỏ nhẹ (chỉ khoảng 20MB) vì nó không chứa thư viện biên dịch của Rust nữa, chỉ giữ lại file thực thi!

```dockerfile
# Stage 1: Xây dựng (Nặng)
FROM rust:1.77-alpine AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

# Stage 2: Môi trường chạy (Siêu nhẹ)
FROM alpine:3.19
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/order-system /usr/local/bin/
COPY migrations/ /app/migrations/

ENV HOST=0.0.0.0
ENV PORT=3000
EXPOSE 3000

CMD ["order-system"]
```

### CI/CD Pipeline với GitHub Actions

Chúng ta thiết lập một Pipeline tự động chạy test, check định dạng, chạy clippy để bắt lỗi, và cuối cùng build Docker image nếu code merge vào nhánh `main`.

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
```

---

## 44.8 — Telemetry & Monitoring (Giám sát hệ thống)

Làm sao để biết hệ thống đang gặp lỗi gì trên production? 
Với hệ thống lớn, dùng `println!` là không đủ. Chúng ta cần **Structured Logging** để xuất log dưới định dạng JSON, giúp các công cụ (như ELK stack, Grafana) dễ dàng phân tích và cảnh báo.

```rust
use tracing::info;

// Thêm vào main() trước khi khởi tạo Router:
fn setup_logging() {
    tracing_subscriber::fmt()
        .with_target(false)
        .json() // Bật JSON log
        .init();
}

// Bọc toàn bộ app bằng Middleware để log MỌI request tự động:
// let app = Router::new()
//     .route(...)
//     .layer(TraceLayer::new_for_http()) 
```

Bạn cũng có thể chèn log tùy chỉnh vào trong hàm `create_order` cực kỳ chuyên nghiệp:

```rust
info!(
    order_id = response.order_id,
    user_id = req.user_id,
    total = total,
    items = lines.len(),
    "Order placed successfully"
);
// Log sẽ ra dạng JSON chứa mọi trường dữ liệu bạn cần!
```

---

## 44.9 — Tóm lược kiến trúc

Nhìn lại toàn bộ hệ thống: mỗi lớp (layer) chỉ biết đến lớp ngay dưới nó (thông qua traits), lớp Domain không biết gì về thế giới bên ngoài. Đây là cách bạn xây hệ thống lớn mà vẫn testable, maintainable!

```text
┌─────────────────────────────────────────────────────────┐
│                    Production Stack                     │
│                                                         │
│  ┌──────────┐    ┌──────────────┐    ┌───────────────┐  │
│  │  Nginx   │───▶│  Axum App    │───▶│  PostgreSQL   │  │
│  │  (LB)    │    │  (Rust)      │    │  (persistence)│  │
│  └──────────┘    └──────┬───────┘    └───────────────┘  │
│                         │                               │
│                    ┌────┴─────┐                         │
│                    │  Redis   │                         │
│                    │  (cache) │                         │
│                    └──────────┘                         │
│                                                         │
│  Observability:                                         │
│    tracing → stdout/Loki → Grafana                      │
│    metrics → Prometheus → Grafana                       │
│    traces  → OpenTelemetry → Jaeger                     │
└─────────────────────────────────────────────────────────┘
```

### Tổng kết hành trình Chapter 0 → 44
- ✅ **Domain**: Khai báo Value objects (`Email`, `Money`), hàm tính toán thuần túy, state machine.
- ✅ **Database**: PostgreSQL kết hợp `sqlx`, dùng file migrations tự tạo bảng.
- ✅ **API**: Định nghĩa Axum routes, xử lý JSON đầu vào/ra chuyên nghiệp.
- ✅ **Hạ tầng**: Docker Multi-stage build nhỏ gọn, CI/CD GitHub Actions bảo vệ code.
- ✅ **Vận hành**: Gắn Telemetry (tracing) để quan sát hệ thống.

---

## 🎉🎉🎉 CHÚC MỪNG! BẠN ĐÃ HOÀN THÀNH "DOMAIN-DRIVEN FP WITH RUST"!

Bạn vừa đi qua một hành trình 45 chương từ cơ bản đến cao cấp nhất của Rust và Lập trình Hàm.

```text
Part 1: Foundations     — Rust basics, ownership, error handling
Part 2: FP Core         — Immutability, HOF, traits, generics
Part 3: Design Patterns — OOP Patterns → FP, CQRS/ES
Part 4: DDD with Rust   — Domain modeling, workflows, ROP
Part 5: FP Patterns     — Algebra, functors, monads, parsers, folds
Part 6: Testing         — TDD, PBT, mocking
Part 7: Production      — Capstone, DB, security, observability
```

Bạn không chỉ biết viết Rust. Bạn đã học được **CÁCH KIẾN TRÚC MỘT HỆ THỐNG LỚN (ARCHITECTURE)** bằng tư duy Lập trình Hàm. 
Đây là nền tảng vững chắc đưa bạn vươn lên tầm Senior/Principal Engineer trong thế giới backend.

> *"The only way to learn is to build."* — Hãy bắt tay vào dự án thực tế của riêng bạn ngay hôm nay! 🚀

---

## ✅ Checkpoint 44

1. Multi-stage Dockerfile cho binary Rust nhỏ hơn nhiều so với image Python. Vì sao?
2. Trong kiến trúc capstone này, tầng nào **không** biết `axum` tồn tại?
3. Vì sao dùng `tracing` chứ không phải `println!` cho logging production?

<details>
<summary>Đáp án</summary>

1. Vì Rust biên dịch thành một binary tĩnh — stage cuối chỉ cần binary đó (và có thể vài thư viện hệ thống). Không cần runtime, không cần interpreter, không cần cây dependency. Image cuối thường vài chục MB, có khi dưới 10 MB với `scratch`/`distroless`.
2. **Domain**. Nó chỉ biết kiểu của chính nó và `Result`. Việc HTTP hoá diễn ra ở tầng API qua `impl IntoResponse` — chính là Anti-Corruption Layer ở hướng ra.
3. `tracing` cho log **có cấu trúc** và có **span** — mỗi dòng log mang theo ngữ cảnh của request đang chạy (trace id, user id) mà không phải truyền tay. `println!` cho chuỗi phẳng, không truy vấn được và không nối được giữa các service.
</details>

---

## 🏋️ Bài tập

**Bài 1 (15 phút).** Thêm endpoint `/readyz` kiểm tra kết nối database và Redis, tách biệt khỏi `/healthz`. Giải thích vì sao phải tách.

**Bài 2 (25 phút).** Thêm graceful shutdown: bắt SIGTERM, ngừng nhận request mới, chờ request đang chạy hoàn tất (tối đa 30s), rồi đóng pool.

**Bài 3 (40 phút).** Bổ sung vào CI: `cargo clippy -- -D warnings`, `cargo test`, `cargo audit`, và build image rồi quét bằng `trivy`. Cho pipeline fail ở bất kỳ bước nào.

<details>
<summary>Gợi ý bài 2</summary>

Axum có `.with_graceful_shutdown(signal)`. Phần khó không nằm ở đó mà ở **thứ tự
dọn dẹp**: ngừng nhận request → chờ request đang chạy → đóng connection pool.
Đóng pool trước là làm hỏng chính những request bạn đang cố hoàn tất.
</details>

---

## 🔧 Troubleshooting

| Vấn đề | Vì sao xảy ra | Hướng xử lý |
|---|---|---|
| Image vẫn hàng trăm MB | Build debug, chưa strip | `--release`, `strip = true`, base `distroless` |
| `GLIBC_2.x not found` khi chạy | Build trên glibc mới hơn runtime | Target `musl`, hoặc base image cùng distro |
| Docker build lại từ đầu mỗi lần | Copy source trước khi cache dependency | Copy `Cargo.toml`/`Cargo.lock` và build dependency trước; hoặc `cargo-chef` |
| Container bị kill lúc deploy | Không xử lý SIGTERM | Graceful shutdown + `terminationGracePeriodSeconds` phù hợp |
| Pool cạn khi tải cao | `max_connections` chưa tính theo số replica | Tổng pool của mọi replica phải nhỏ hơn `max_connections` của Postgres |

---

## Tóm tắt

- **Multi-stage build + binary tĩnh** là lợi thế lớn nhất của Rust khi deploy:
  image nhỏ, cold start gần bằng không, bề mặt tấn công hẹp.
- **Domain không biết gì về HTTP.** `impl IntoResponse` cho kiểu lỗi domain là
  chỗ duy nhất hai thế giới gặp nhau.
- **`tracing` chứ không `println!`** — log có cấu trúc và có span mới truy vấn
  được ở quy mô production.
- **Graceful shutdown là chi tiết dễ bỏ sót nhất** và cũng là thứ gây lỗi 502
  nhiều nhất khi deploy.
- **CI phải chặn được merge.** Pipeline chỉ cảnh báo thì tương đương không có.
