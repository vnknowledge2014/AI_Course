# Chapter 36B — Web Services with Axum

> **Bạn sẽ học được**:
> - **HTTP basics** — request/response cycle trong 5 phút
> - **Axum** — web framework chính thức của Rust ecosystem
> - **Router & Handlers** — tạo routes, nhận params, trả JSON
> - **Extractors** — Path, Query, Json, State
> - **Middleware** — logging, auth, rate limiting
> - **CRUD API hoàn chỉnh** — build REST API chạy được
>
> **Yêu cầu trước**: Chapter 36 (Async/Await, tokio).
> **Thời gian đọc**: ~50 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Bạn build được REST API hoàn chỉnh với domain types, error handling, và middleware.

---

## 36B.1 — HTTP trong 5 phút

### Ẩn dụ: Quán phở

Bạn vào quán phở. Quy trình diễn ra thế này:

```
Bạn (Client)          Bồi bàn (Router)         Bếp (Handler)
    │                      │                        │
    ├── "Cho tô phở bò" ──▶│                        │
    │   (HTTP Request)     ├── Chuyển order ───────▶│
    │                      │                        ├── Nấu phở
    │                      │◀── Phở xong ───────────┤
    │◀── Đây, tô phở ──────┤                        │
    │   (HTTP Response)    │                        │
```

- **Request** = bạn gọi món: "Cho tô phở bò, size lớn, thêm gì?"
- **Router** = bồi bàn: nhìn menu (routes), chuyển đúng order cho đúng bếp
- **Handler** = bếp: nhận order, nấu (xử lý logic), trả món
- **Response** = tô phở ra bàn: có status (ngon/hết món), có data (tô phở)

### HTTP Request = Gọi món

```
GET /api/pho/bo?size=large HTTP/1.1     ← Method + Path + Query
Host: quanpho.com                        ← Server address
Authorization: Bearer abc123             ← "Thẻ VIP" (token)
Content-Type: application/json           ← Loại data gửi kèm
```

| Phần | Nghĩa | Ví dụ quán phở |
|------|--------|----------------|
| **Method** | Hành động | `GET`=xem menu, `POST`=gọi món, `PUT`=đổi món, `DELETE`=hủy |
| **Path** | Món gì | `/api/pho/bo` = phở bò |
| **Query** | Tùy chọn | `?size=large&extra=trung` |
| **Headers** | Thông tin kèm | Thẻ VIP (auth), loại data |
| **Body** | Chi tiết order | `{ "quantity": 2, "note": "ít hành" }` |

### HTTP Response = Tô phở ra bàn

```
HTTP/1.1 200 OK                          ← Status code
Content-Type: application/json

{ "order_id": 42, "dish": "Phở bò", "status": "ready" }
```

| Status | Nghĩa | Ví dụ quán phở |
|--------|--------|----------------|
| **200** OK | Thành công | Tô phở ra bàn ✅ |
| **201** Created | Tạo mới thành công | Order đã ghi nhận |
| **400** Bad Request | Sai format | "Phở gà bò" → không hiểu |
| **401** Unauthorized | Chưa xác thực | Chưa có thẻ VIP |
| **404** Not Found | Không tìm thấy | "Phở ramen" → không có |
| **500** Server Error | Lỗi bếp | Bếp cháy → xin lỗi |

---

## 36B.2 — Hello Axum

### Setup

```toml
# filename: Cargo.toml
[package]
name = "web-api"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.5", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

### Server đơn giản nhất

```rust
// filename: src/main.rs

use axum::{Router, routing::get, response::Json};
use serde_json::{json, Value};

// Handler = "bếp" — nhận request, trả response
async fn hello() -> Json<Value> {
    Json(json!({
        "message": "Xin chào từ Axum! 🦀",
        "version": "1.0.0"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "healthy" }))
}

#[tokio::main]
async fn main() {
    // Router = "bồi bàn" — route request đến đúng handler
    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health));

    // Khởi động server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

// Test bằng:
// curl http://localhost:3000
// → {"message":"Xin chào từ Axum! 🦀","version":"1.0.0"}
//
// curl http://localhost:3000/health
// → {"status":"healthy"}
```

> **💡 Ghi nhớ**: `async fn handler() -> impl IntoResponse` — mọi thứ implement `IntoResponse` đều trả về được: `String`, `Json<T>`, `(StatusCode, Json<T>)`, HTML...

---

## ✅ Checkpoint 36B.2

> Ghi nhớ:
> 1. **Axum** = web framework, chạy trên `tokio` async runtime
> 2. **Router** = map path → handler. `Router::new().route("/path", get(handler))`
> 3. **Handler** = async function trả `impl IntoResponse`
> 4. Test bằng `curl` hoặc browser
>
> **Test nhanh**: `Router::new().route("/api/users", get(list).post(create))` — route này hỗ trợ mấy HTTP methods?
> <details><summary>Đáp án</summary>2 methods: GET (gọi `list`) và POST (gọi `create`). Cùng path, khác method → khác handler.</details>

---

## 36B.3 — Extractors: Lấy dữ liệu từ request

### Ẩn dụ: Bồi bàn ghi order

Khi khách gọi món, bồi bàn cần **lấy thông tin** từ nhiều nguồn:
- **Bàn số mấy?** → Path parameter (`/tables/5`)
- **Có yêu cầu đặc biệt?** → Query parameter (`?spicy=true`)
- **Chi tiết order?** → Request body (JSON)
- **Thẻ VIP?** → Header (Authorization)

Axum gọi các "cách lấy thông tin" này là **Extractors**.

### Path — Lấy giá trị từ URL

```rust
// filename: src/main.rs

use axum::{Router, routing::get, extract::Path, response::Json};
use serde_json::{json, Value};

// Path extractor: /users/42 → id = 42
async fn get_user(Path(id): Path<u64>) -> Json<Value> {
    Json(json!({
        "id": id,
        "name": format!("User #{}", id),
        "email": format!("user{}@example.com", id)
    }))
}

// Multiple path params: /users/42/posts/7
async fn get_user_post(
    Path((user_id, post_id)): Path<(u64, u64)>,
) -> Json<Value> {
    Json(json!({
        "user_id": user_id,
        "post_id": post_id,
        "title": format!("Post #{} by User #{}", post_id, user_id)
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/{id}", get(get_user))
        .route("/users/{user_id}/posts/{post_id}", get(get_user_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    println!("🚀 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// curl http://localhost:3000/users/42
// → {"id":42,"name":"User #42","email":"user42@example.com"}
//
// curl http://localhost:3000/users/42/posts/7
// → {"user_id":42,"post_id":7,"title":"Post #7 by User #42"}
```

### Query — Lấy tham số tìm kiếm

```rust
// filename: src/main.rs

use axum::{Router, routing::get, extract::Query, response::Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,        // ?q=rust
    page: Option<u32>,        // &page=2
    limit: Option<u32>,       // &limit=10
}

#[derive(Serialize)]
struct SearchResult {
    query: String,
    page: u32,
    limit: u32,
    results: Vec<String>,
}

async fn search(Query(params): Query<SearchParams>) -> Json<SearchResult> {
    let query = params.q.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // Giả lập search results
    let results = if query.is_empty() {
        vec![]
    } else {
        (0..limit.min(3))
            .map(|i| format!("Result {} for '{}'", i + 1, query))
            .collect()
    };

    Json(SearchResult { query, page, limit, results })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/search", get(search));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    println!("🚀 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// curl "http://localhost:3000/search?q=rust&page=1&limit=5"
// → {"query":"rust","page":1,"limit":5,"results":["Result 1 for 'rust'","Result 2 for 'rust'","Result 3 for 'rust'"]}
//
// curl "http://localhost:3000/search"
// → {"query":"","page":1,"limit":10,"results":[]}
```

### Json Body — Nhận dữ liệu từ client

```rust
// filename: src/main.rs

use axum::{Router, routing::post, extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: u64,
    name: String,
    email: String,
    message: String,
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // Validation (kết hợp Ch 24 ROP)
    if payload.name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Name is required"})),
        );
    }

    // Tạo user (in-memory)
    let user = UserResponse {
        id: 1,
        name: payload.name,
        email: payload.email.to_lowercase(),
        message: "User created successfully".into(),
    };

    (StatusCode::CREATED, Json(serde_json::json!(user)))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    println!("🚀 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// curl -X POST http://localhost:3000/users \
//   -H "Content-Type: application/json" \
//   -d '{"name":"Minh","email":"MINH@co.com"}'
// → {"id":1,"name":"Minh","email":"minh@co.com","message":"User created successfully"}
```

### Bảng tóm tắt Extractors

| Extractor | Nguồn | Ví dụ | Use case |
|-----------|-------|-------|----------|
| `Path<T>` | URL path | `/users/{id}` | Lấy resource by ID |
| `Query<T>` | Query string | `?q=rust&page=1` | Search, filter, pagination |
| `Json<T>` | Request body | `{ "name": "Minh" }` | Create/Update data |
| `State<T>` | App state | DB pool, config | Shared dependencies |
| `HeaderMap` | Headers | `Authorization: Bearer ...` | Auth tokens, metadata |

---

## ✅ Checkpoint 36B.3

> Ghi nhớ:
> 1. **Extractors** = cách lấy data từ request. Đặt trong **params của handler function**
> 2. `Path(id)` cho URL params, `Query(params)` cho query string, `Json(body)` cho request body
> 3. Axum tự **deserialize** bằng `serde` — chỉ cần `#[derive(Deserialize)]`
> 4. Response: `(StatusCode, Json<T>)` → trả status code + JSON body
>
> **Test nhanh**: Handler có signature `async fn(Path(id): Path<u64>, Json(body): Json<UpdateUser>)` — nó nhận data từ đâu?
> <details><summary>Đáp án</summary>Từ 2 nguồn: `id` từ URL path (ví dụ `/users/42`), `body` từ JSON request body. Axum tự extract cả hai.</details>

---

## 36B.4 — State: Chia sẻ dữ liệu giữa handlers

### Vấn đề

Mỗi handler là function riêng. Làm sao chúng chia sẻ database connection, config, hoặc in-memory data?

### Giải pháp: `State<T>` — "Tủ đồ chung của quán"

```rust
// filename: src/main.rs

use axum::{
    Router, routing::{get, post},
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// ═══ Domain Types (từ Ch 22) ═══
#[derive(Debug, Clone, Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

// ═══ App State = "tủ đồ chung" ═══
#[derive(Clone)]
struct AppState {
    // Arc<Mutex<...>> = shared mutable state (từ Ch 36)
    users: Arc<Mutex<HashMap<u64, User>>>,
    next_id: Arc<Mutex<u64>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            users: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

// ═══ Handlers ═══

async fn list_users(
    State(state): State<AppState>,
) -> Json<Vec<User>> {
    let users = state.users.lock().unwrap();
    let mut list: Vec<User> = users.values().cloned().collect();
    list.sort_by_key(|u| u.id);
    Json(list)
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let users = state.users.lock().unwrap();
    match users.get(&id) {
        Some(user) => (StatusCode::OK, Json(serde_json::json!(user))),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": format!("User {} not found", id)})),
        ),
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // Validation
    if payload.name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Name is required"})),
        );
    }

    // Tạo user
    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let user = User {
        id,
        name: payload.name.trim().to_string(),
        email: payload.email.trim().to_lowercase(),
    };

    state.users.lock().unwrap().insert(id, user.clone());

    (StatusCode::CREATED, Json(serde_json::json!(user)))
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    println!("🚀 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// Test:
// curl -X POST http://localhost:3000/users \
//   -H "Content-Type: application/json" \
//   -d '{"name":"Minh","email":"minh@co.com"}'
// → {"id":1,"name":"Minh","email":"minh@co.com"}
//
// curl -X POST http://localhost:3000/users \
//   -H "Content-Type: application/json" \
//   -d '{"name":"Lan","email":"lan@co.com"}'
// → {"id":2,"name":"Lan","email":"lan@co.com"}
//
// curl http://localhost:3000/users
// → [{"id":1,"name":"Minh","email":"minh@co.com"},{"id":2,...}]
//
// curl http://localhost:3000/users/1
// → {"id":1,"name":"Minh","email":"minh@co.com"}
//
// curl http://localhost:3000/users/99
// → {"error":"User 99 not found"}
```

> **💡 Kết nối với Ch 36**: `Arc<Mutex<T>>` — bạn đã học ở Concurrency chapter. Ở đây nó giữ shared state giữa các async handlers. Trong production, thay `HashMap` bằng database pool (Ch 38).

---

## 36B.5 — Error Handling: Kết hợp ROP (Ch 24)

### Structured API Errors

```rust
// filename: src/main.rs

use axum::{
    Router, routing::{get, post},
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// ═══ Domain Error (từ Ch 24 — ROP) ═══
#[derive(Debug)]
enum AppError {
    NotFound(String),
    Validation(String),
    Internal(String),
}

// Implement IntoResponse cho AppError → Axum tự convert
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = serde_json::json!({
            "error": error_message,
            "code": status.as_u16(),
        });

        (status, Json(body)).into_response()
    }
}

// ═══ Domain Types ═══
#[derive(Debug, Clone, Serialize)]
struct Product {
    id: u64,
    name: String,
    price: u32,   // đơn vị: đồng
    stock: u32,
}

#[derive(Deserialize)]
struct CreateProduct {
    name: String,
    price: u32,
}

#[derive(Clone)]
struct AppState {
    products: Arc<Mutex<HashMap<u64, Product>>>,
    next_id: Arc<Mutex<u64>>,
}

// ═══ Handlers dùng Result<T, AppError> ═══
// Axum tự gọi IntoResponse cho cả Ok và Err!

async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Product>, AppError> {
    let products = state.products.lock()
        .map_err(|_| AppError::Internal("Lock failed".into()))?;

    products.get(&id)
        .cloned()
        .map(Json)
        .ok_or_else(|| AppError::NotFound(format!("Product {} not found", id)))
}

async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> Result<(StatusCode, Json<Product>), AppError> {
    // Validation — Railway style!
    if payload.name.trim().is_empty() {
        return Err(AppError::Validation("Product name is required".into()));
    }
    if payload.price == 0 {
        return Err(AppError::Validation("Price must be > 0".into()));
    }

    let mut next_id = state.next_id.lock()
        .map_err(|_| AppError::Internal("Lock failed".into()))?;
    let id = *next_id;
    *next_id += 1;

    let product = Product {
        id,
        name: payload.name.trim().to_string(),
        price: payload.price,
        stock: 0,
    };

    state.products.lock()
        .map_err(|_| AppError::Internal("Lock failed".into()))?
        .insert(id, product.clone());

    Ok((StatusCode::CREATED, Json(product)))
}

#[tokio::main]
async fn main() {
    let state = AppState {
        products: Arc::new(Mutex::new(HashMap::new())),
        next_id: Arc::new(Mutex::new(1)),
    };

    let app = Router::new()
        .route("/products", post(create_product))
        .route("/products/{id}", get(get_product))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    println!("🚀 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// curl -X POST http://localhost:3000/products \
//   -H "Content-Type: application/json" \
//   -d '{"name":"Phở bò","price":55000}'
// → {"id":1,"name":"Phở bò","price":55000,"stock":0}   (201 Created)
//
// curl http://localhost:3000/products/1
// → {"id":1,"name":"Phở bò","price":55000,"stock":0}   (200 OK)
//
// curl http://localhost:3000/products/99
// → {"error":"Product 99 not found","code":404}          (404 Not Found)
//
// curl -X POST http://localhost:3000/products \
//   -H "Content-Type: application/json" \
//   -d '{"name":"","price":0}'
// → {"error":"Product name is required","code":400}      (400 Bad Request)
```

> **💡 Railway connection**: `Result<T, AppError>` trong handler = **Two-track model** (Ch 24). `?` operator tự chuyển sang Failure track. `AppError` implement `IntoResponse` → Axum tự trả HTTP error. Bạn không cần `match` thủ công!

---

## ✅ Checkpoint 36B.5

> Ghi nhớ:
> 1. **`State<AppState>`** = shared data giữa handlers. Cần `Clone`.
> 2. **`impl IntoResponse for AppError`** = tự động convert domain errors → HTTP responses
> 3. Handler trả **`Result<T, AppError>`** = ROP cho web — `?` chuyển sang error track
> 4. Pattern: **`Result<(StatusCode, Json<T>), AppError>`** cho create operations
>
> **Test nhanh**: Nếu handler trả `Err(AppError::Validation("bad input"))`, HTTP response sẽ có status code gì?
> <details><summary>Đáp án</summary>400 Bad Request — vì `AppError::Validation` map tới `StatusCode::BAD_REQUEST` trong `IntoResponse`.</details>

---

## 36B.6 — Middleware: "Bảo vệ tòa nhà"

### Ẩn dụ

Trước khi khách vào quán (handler), phải qua các lớp kiểm tra:
1. **Cổng bảo vệ** — ai cũng phải qua (logging)
2. **Quét thẻ** — chỉ khách có thẻ (auth middleware)
3. **Giới hạn** — tối đa 10 người/phút (rate limit)

Middleware = các lớp xử lý **trước** và **sau** handler.

### Logging middleware

```rust
// filename: src/main.rs

use axum::{Router, routing::get, response::Json, middleware, extract::Request};
use serde_json::{json, Value};
use std::time::Instant;

// Custom middleware: log mỗi request
async fn log_request(
    req: Request,
    next: middleware::Next,
) -> impl axum::response::IntoResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    // Chuyển request cho handler tiếp theo
    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();
    println!("{} {} → {} ({:?})", method, uri, status.as_u16(), duration);

    response
}

async fn hello() -> Json<Value> {
    Json(json!({"message": "Hello!"}))
}

async fn slow() -> Json<Value> {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Json(json!({"message": "Slow response"}))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/slow", get(slow))
        .layer(middleware::from_fn(log_request)); // <-- Thêm middleware!

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    println!("🚀 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// curl http://localhost:3000/
// Server log: GET / → 200 (42µs)
//
// curl http://localhost:3000/slow
// Server log: GET /slow → 200 (101ms)
```

### Auth middleware

```rust
// filename: src/main.rs

use axum::{
    Router, routing::get,
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::{self, Next},
    response::{IntoResponse, Json},
};
use serde_json::json;

// Middleware: kiểm tra API key
async fn require_auth(
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let auth = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    match auth {
        Some(token) if token.starts_with("Bearer ") => {
            let api_key = &token[7..];
            // Trong production: verify JWT (Ch 40)
            if api_key == "secret123" {
                Ok(next.run(req).await)
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid API key"})),
                ))
            }
        }
        _ => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Authorization header required (Bearer <key>)"})),
        )),
    }
}

async fn public() -> Json<serde_json::Value> {
    Json(json!({"message": "Public — ai cũng thấy"}))
}

async fn secret() -> Json<serde_json::Value> {
    Json(json!({"message": "Secret — chỉ có API key mới thấy 🔐"}))
}

#[tokio::main]
async fn main() {
    // Routes KHÔNG cần auth
    let public_routes = Router::new()
        .route("/", get(public));

    // Routes CẦN auth — wrap bằng middleware
    let protected_routes = Router::new()
        .route("/secret", get(secret))
        .layer(middleware::from_fn(require_auth));

    let app = public_routes.merge(protected_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    println!("🚀 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// curl http://localhost:3000/
// → {"message":"Public — ai cũng thấy"}
//
// curl http://localhost:3000/secret
// → {"error":"Authorization header required (Bearer <key>)"}   (401)
//
// curl -H "Authorization: Bearer wrong" http://localhost:3000/secret
// → {"error":"Invalid API key"}                                 (401)
//
// curl -H "Authorization: Bearer secret123" http://localhost:3000/secret
// → {"message":"Secret — chỉ có API key mới thấy 🔐"}          (200)
```

### Middleware order (quan trọng!)

```
Request flow:

Client ──▶ Layer 3 (outermost) ──▶ Layer 2 ──▶ Layer 1 (innermost) ──▶ Handler
           │ logging              │ auth       │ rate limit            │
           ◀──────────────────────◀────────────◀───────────────────────◀

Trong Axum: layer THÊM SAU chạy TRƯỚC (outermost)!

Router::new()
    .route(...)
    .layer(rate_limit)    // 1. thêm trước → chạy sau (innermost)
    .layer(auth)          // 2. thêm sau  → chạy trước
    .layer(logging)       // 3. thêm cuối → chạy đầu tiên (outermost)
```

---

## 36B.7 — CRUD API hoàn chỉnh: Quản lý sách

Kết hợp mọi thứ: Router + Extractors + State + Error Handling + Domain Types.

```rust
// filename: src/main.rs

use axum::{
    Router, routing::{get, post, put, delete},
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
    middleware,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// ═══ Domain Types (Ch 22 — Domain Modeling) ═══

#[derive(Debug, Clone, Serialize)]
struct Book {
    id: u64,
    title: String,
    author: String,
    price: u32,       // đồng
    in_stock: bool,
}

#[derive(Deserialize)]
struct CreateBook {
    title: String,
    author: String,
    price: u32,
}

#[derive(Deserialize)]
struct UpdateBook {
    title: Option<String>,
    author: Option<String>,
    price: Option<u32>,
    in_stock: Option<bool>,
}

#[derive(Deserialize)]
struct ListParams {
    author: Option<String>,
    min_price: Option<u32>,
    max_price: Option<u32>,
}

// ═══ Error Type (Ch 24 — ROP) ═══

#[derive(Debug)]
enum AppError {
    NotFound(String),
    Validation(Vec<String>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                serde_json::json!({"error": msg}),
            ),
            AppError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                serde_json::json!({"errors": errors}),
            ),
        };
        (status, Json(body)).into_response()
    }
}

// ═══ App State ═══

#[derive(Clone)]
struct AppState {
    books: Arc<Mutex<HashMap<u64, Book>>>,
    next_id: Arc<Mutex<u64>>,
}

impl AppState {
    fn new() -> Self {
        let mut books = HashMap::new();
        books.insert(1, Book {
            id: 1, title: "Dế Mèn Phiêu Lưu Ký".into(),
            author: "Tô Hoài".into(), price: 45_000, in_stock: true,
        });
        books.insert(2, Book {
            id: 2, title: "Số Đỏ".into(),
            author: "Vũ Trọng Phụng".into(), price: 65_000, in_stock: true,
        });
        AppState {
            books: Arc::new(Mutex::new(books)),
            next_id: Arc::new(Mutex::new(3)),
        }
    }
}

// ═══ Validation (Ch 24 — Collect all errors) ═══

fn validate_create(input: &CreateBook) -> Result<(), AppError> {
    let mut errors = vec![];
    if input.title.trim().is_empty() {
        errors.push("Title is required".into());
    }
    if input.author.trim().is_empty() {
        errors.push("Author is required".into());
    }
    if input.price == 0 {
        errors.push("Price must be > 0".into());
    }
    if errors.is_empty() { Ok(()) } else { Err(AppError::Validation(errors)) }
}

// ═══ Handlers (CRUD) ═══

// CREATE — POST /books
async fn create_book(
    State(state): State<AppState>,
    Json(payload): Json<CreateBook>,
) -> Result<(StatusCode, Json<Book>), AppError> {
    validate_create(&payload)?;

    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let book = Book {
        id,
        title: payload.title.trim().to_string(),
        author: payload.author.trim().to_string(),
        price: payload.price,
        in_stock: true,
    };

    state.books.lock().unwrap().insert(id, book.clone());
    Ok((StatusCode::CREATED, Json(book)))
}

// READ (list) — GET /books?author=X&min_price=Y
async fn list_books(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Json<Vec<Book>> {
    let books = state.books.lock().unwrap();
    let mut result: Vec<Book> = books.values()
        .filter(|b| {
            params.author.as_ref()
                .map_or(true, |a| b.author.to_lowercase().contains(&a.to_lowercase()))
        })
        .filter(|b| params.min_price.map_or(true, |min| b.price >= min))
        .filter(|b| params.max_price.map_or(true, |max| b.price <= max))
        .cloned()
        .collect();
    result.sort_by_key(|b| b.id);
    Json(result)
}

// READ (single) — GET /books/:id
async fn get_book(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Book>, AppError> {
    state.books.lock().unwrap()
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or_else(|| AppError::NotFound(format!("Book {} not found", id)))
}

// UPDATE — PUT /books/:id
async fn update_book(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, AppError> {
    let mut books = state.books.lock().unwrap();
    let book = books.get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("Book {} not found", id)))?;

    if let Some(title) = payload.title {
        book.title = title;
    }
    if let Some(author) = payload.author {
        book.author = author;
    }
    if let Some(price) = payload.price {
        book.price = price;
    }
    if let Some(in_stock) = payload.in_stock {
        book.in_stock = in_stock;
    }

    Ok(Json(book.clone()))
}

// DELETE — DELETE /books/:id
async fn delete_book(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, AppError> {
    let mut books = state.books.lock().unwrap();
    if books.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("Book {} not found", id)))
    }
}

// ═══ Main ═══

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/books", get(list_books).post(create_book))
        .route("/books/{id}", get(get_book).put(update_book).delete(delete_book))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    println!("🚀 Book API running at http://localhost:3000");
    println!();
    println!("Try:");
    println!("  curl http://localhost:3000/books");
    println!("  curl http://localhost:3000/books/1");
    println!("  curl -X POST http://localhost:3000/books \\");
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"title\":\"Tắt Đèn\",\"author\":\"Ngô Tất Tố\",\"price\":50000}}'");

    axum::serve(listener, app).await.unwrap();
}

// ═══ Full test sequence ═══
//
// 1. List (pre-seeded):
// curl http://localhost:3000/books
// → [{"id":1,"title":"Dế Mèn Phiêu Lưu Ký",...},{"id":2,...}]
//
// 2. Create:
// curl -X POST http://localhost:3000/books \
//   -H "Content-Type: application/json" \
//   -d '{"title":"Tắt Đèn","author":"Ngô Tất Tố","price":50000}'
// → {"id":3,"title":"Tắt Đèn","author":"Ngô Tất Tố","price":50000,"in_stock":true}
//
// 3. Get by ID:
// curl http://localhost:3000/books/3
// → {"id":3,"title":"Tắt Đèn",...}
//
// 4. Update:
// curl -X PUT http://localhost:3000/books/3 \
//   -H "Content-Type: application/json" \
//   -d '{"price":55000,"in_stock":false}'
// → {"id":3,"title":"Tắt Đèn","price":55000,"in_stock":false,...}
//
// 5. Filter:
// curl "http://localhost:3000/books?author=tô&min_price=40000"
// → [{"id":1,"title":"Dế Mèn Phiêu Lưu Ký","author":"Tô Hoài",...}]
//
// 6. Delete:
// curl -X DELETE http://localhost:3000/books/3
// → (204 No Content)
//
// 7. Verify deleted:
// curl http://localhost:3000/books/3
// → {"error":"Book 3 not found"}  (404)
//
// 8. Validation errors:
// curl -X POST http://localhost:3000/books \
//   -H "Content-Type: application/json" \
//   -d '{"title":"","author":"","price":0}'
// → {"errors":["Title is required","Author is required","Price must be > 0"]}
```

---

## ✅ Checkpoint 36B.7

> Ghi nhớ:
> 1. **CRUD pattern**: `POST` (create), `GET` (read), `PUT` (update), `DELETE` (delete)
> 2. Route setup: `.route("/path", get(handler).post(handler).put(handler).delete(handler))`
> 3. **State** = `Arc<Mutex<HashMap>>` — shared mutable state giữa handlers
> 4. **Error handling** = `Result<T, AppError>` + `impl IntoResponse` — ROP cho web!
> 5. **Validation** = collect all errors (Ch 24 Validated pattern)

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Thêm `GET /books/count`

Thêm endpoint trả tổng số sách trong store.

<details><summary>✅ Lời giải</summary>

```rust
async fn count_books(State(state): State<AppState>) -> Json<serde_json::Value> {
    let count = state.books.lock().unwrap().len();
    Json(serde_json::json!({"count": count}))
}

// Thêm vào router:
// .route("/books/count", get(count_books))

// Lưu ý: đặt TRƯỚC route "/books/{id}" để tránh conflict!
```

</details>

---

**Bài 2** (10 phút): Thêm pagination

Modify `list_books` để hỗ trợ `?page=1&limit=10`. Trả response có:
```json
{
  "data": [...],
  "page": 1,
  "limit": 10,
  "total": 42
}
```

<details><summary>💡 Gợi ý</summary>

Thêm `page` và `limit` vào `ListParams`. Dùng `.skip()` và `.take()` trên iterator.

</details>

<details><summary>✅ Lời giải Bài 2</summary>

```rust
#[derive(Deserialize)]
struct ListParams {
    author: Option<String>,
    min_price: Option<u32>,
    max_price: Option<u32>,
    page: Option<usize>,
    limit: Option<usize>,
}

#[derive(Serialize)]
struct PaginatedResponse {
    data: Vec<Book>,
    page: usize,
    limit: usize,
    total: usize,
}

async fn list_books(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Json<PaginatedResponse> {
    let books = state.books.lock().unwrap();
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(10).min(100);

    let filtered: Vec<Book> = books.values()
        .filter(|b| params.author.as_ref()
            .map_or(true, |a| b.author.to_lowercase().contains(&a.to_lowercase())))
        .filter(|b| params.min_price.map_or(true, |min| b.price >= min))
        .filter(|b| params.max_price.map_or(true, |max| b.price <= max))
        .cloned()
        .collect();

    let total = filtered.len();
    let data: Vec<Book> = filtered.into_iter()
        .skip((page - 1) * limit)
        .take(limit)
        .collect();

    Json(PaginatedResponse { data, page, limit, total })
}
```

</details>

---

**Bài 3** (15 phút): Thêm domain types cho Book

Áp dụng Ch 22 (Domain Modeling) — tạo newtype wrappers:
1. `BookTitle(String)` — validate: 1-200 ký tự
2. `Price(u32)` — validate: 1000 ≤ price ≤ 10_000_000
3. Dùng `Validated<T>` (Ch 24) để collect all validation errors
4. Sửa `create_book` handler dùng domain types

<details><summary>✅ Lời giải Bài 3</summary>

```rust
struct BookTitle(String);
impl BookTitle {
    fn new(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        if trimmed.is_empty() { Err("Title is required".into()) }
        else if trimmed.len() > 200 { Err("Title too long (max 200)".into()) }
        else { Ok(BookTitle(trimmed.to_string())) }
    }
    fn value(&self) -> &str { &self.0 }
}

struct Price(u32);
impl Price {
    fn new(p: u32) -> Result<Self, String> {
        if p < 1000 { Err("Price must be >= 1000đ".into()) }
        else if p > 10_000_000 { Err("Price must be <= 10,000,000đ".into()) }
        else { Ok(Price(p)) }
    }
    fn value(&self) -> u32 { self.0 }
}

fn validate_create(input: &CreateBook) -> Result<(BookTitle, String, Price), AppError> {
    let mut errors = vec![];
    let title = BookTitle::new(&input.title).map_err(|e| errors.push(e)).ok();
    let author = if input.author.trim().is_empty() {
        errors.push("Author is required".into()); None
    } else { Some(input.author.trim().to_string()) };
    let price = Price::new(input.price).map_err(|e| errors.push(e)).ok();

    if errors.is_empty() {
        Ok((title.unwrap(), author.unwrap(), price.unwrap()))
    } else {
        Err(AppError::Validation(errors))
    }
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| `error: unmatched route` khi curl | Sai method hoặc path | Kiểm tra `GET` vs `POST`, trailing slash |
| `Json<T>` parse fail → 422 | Body không match `Deserialize` struct | Kiểm tra JSON keys match struct fields |
| `State` not found | Quên `.with_state(state)` trên Router | Thêm `.with_state(state)` cuối cùng |
| "Address already in use" | Port 3000 đã bị chiếm | Đổi port hoặc `kill` process cũ |
| Handler return type complex | Nhiều extractors + error types | Dùng `impl IntoResponse` hoặc tách handler |
| Middleware order sai | `layer` outermost chạy trước | Layer thêm sau = chạy trước (outer) |

---

## Tóm tắt

- ✅ **HTTP** = Request (method + path + body) → Response (status + body). Ẩn dụ quán phở.
- ✅ **Axum** = web framework trên `tokio`. `Router::new().route("/path", get(handler))`.
- ✅ **Extractors**: `Path`, `Query`, `Json`, `State` — lấy data từ request tự động.
- ✅ **Error handling**: `impl IntoResponse for AppError` → ROP cho web, dùng `?` trong handlers.
- ✅ **Middleware**: `middleware::from_fn(func)` — logging, auth, rate limiting. Order: thêm sau = chạy trước.
- ✅ **CRUD**: `POST` create, `GET` read, `PUT` update, `DELETE` delete — pattern chuẩn cho REST API.
- ✅ **State**: `Arc<Mutex<T>>` cho in-memory. Production thay bằng DB pool (Ch 38).

## Tiếp theo

→ **Part VII: Production Engineering** bắt đầu! Chapter 37: **Capstone Part 1 — Domain Model** ⭐ — bạn sẽ build Order-Taking System hoàn chỉnh, kết hợp MỌI thứ đã học: domain modeling, workflows, ROP, testing, architecture. Và giờ bạn đã biết Axum — phần API layer sẽ không còn bí ẩn!
