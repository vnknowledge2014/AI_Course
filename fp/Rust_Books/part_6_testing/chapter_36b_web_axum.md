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

```text
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

```http
GET /api/pho/bo?size=large HTTP/1.1
Host: quanpho.com
Authorization: Bearer abc123
Content-Type: application/json
```

| Phần | Nghĩa | Ví dụ quán phở |
|------|--------|----------------|
| **Method** | Hành động | `GET`=xem menu, `POST`=gọi món, `PUT`=đổi món, `DELETE`=hủy |
| **Path** | Món gì | `/api/pho/bo` = phở bò |
| **Query** | Tùy chọn | `?size=large&extra=trung` |
| **Headers** | Thông tin kèm | Thẻ VIP (auth), loại data |
| **Body** | Chi tiết order | `{ "quantity": 2, "note": "ít hành" }` |

### HTTP Response = Tô phở ra bàn

```http
HTTP/1.1 200 OK
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
```

Viết hàm `main` khởi động Server và ánh xạ (map) các đường dẫn (routes) tới các handler:

```rust
#[tokio::main]
async fn main() {
    // Router = "bồi bàn" — route request đến đúng handler
    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health));

    // Khởi động server tại cổng 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
```

> **💡 Ghi nhớ**: `async fn handler() -> impl IntoResponse` — mọi thứ implement trait `IntoResponse` đều trả về HTTP Response được: `String`, `Json<T>`, `(StatusCode, Json<T>)`, HTML...

---

## 36B.3 — Extractors: Lấy dữ liệu từ request

### Path — Lấy giá trị từ URL

```rust
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
```

### Query — Lấy tham số tìm kiếm

Query parameters là những phần phía sau dấu `?` trong URL. Axum tự động bóc tách (deserialize) chúng vào trong Struct cho bạn nhờ `serde`:

```rust
use axum::{extract::Query, response::Json};
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
    results: Vec<String>,
}

async fn search(Query(params): Query<SearchParams>) -> Json<SearchResult> {
    let query = params.q.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    
    // Giả lập logic search
    let results = if query.is_empty() { vec![] } else { vec![format!("Found {}", query)] };
    Json(SearchResult { query, page, results })
}
```

### Json Body — Nhận dữ liệu từ client

Với các API cập nhật (POST, PUT), client sẽ gửi JSON lên. Ta dùng extractor `Json`:

```rust
use axum::{routing::post, extract::Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // Validation
    if payload.name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Name is required"})),
        );
    }

    // Trả về HTTP 201 Created cùng dữ liệu JSON
    (StatusCode::CREATED, Json(serde_json::json!({
        "message": "User created",
        "name": payload.name
    })))
}
```

---

## 36B.4 — State: Chia sẻ dữ liệu giữa handlers

Mỗi request được xử lý trên một luồng tách biệt (Concurrent). Làm sao chúng chia sẻ chung một Database Connection Pool hoặc bộ đệm nhớ tạm (In-memory storage)? 

Giải pháp là `State<T>`, hoạt động cùng với `Arc<Mutex<T>>`.

```rust
// filename: src/main.rs
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// --- DTO và Types ---
#[derive(Debug, Clone, Serialize)]
struct User { id: u64, name: String }

#[derive(Deserialize)]
struct CreateUserRequest { name: String }

// --- App State ---
#[derive(Clone)]
struct AppState {
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
```

Bạn tiêm (inject) `State` vào trong các handler dưới dạng argument:

```rust
// Handler lấy danh sách User
async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.users.lock().unwrap();
    let mut list: Vec<User> = users.values().cloned().collect();
    list.sort_by_key(|u| u.id);
    Json(list)
}

// Handler tạo User mới
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    if payload.name.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Name required"})));
    }

    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let user = User { id, name: payload.name.trim().to_string() };
    state.users.lock().unwrap().insert(id, user.clone());

    (StatusCode::CREATED, Json(serde_json::json!(user)))
}
```

Và kết nối State vào Router:

```rust
#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .with_state(state); // Cung cấp State cho Router!
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---

## 36B.5 — Error Handling: Cầu nối ROP

Ở phần ROP (Railway Oriented Programming), bạn đã biết cách gom lỗi bằng `Result<T, AppError>`. Nhưng làm sao báo cho Axum biết AppError tương ứng với HTTP Status Code nào? 

Chỉ cần Implement `IntoResponse` cho enum lỗi của bạn:

```rust
use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(Debug)]
enum AppError {
    NotFound(String),
    Validation(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // Tự động map Error sang HTTP Status
        let (status, msg) = match self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::Validation(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };

        let body = serde_json::json!({
            "error": msg,
            "code": status.as_u16(),
        });

        (status, Json(body)).into_response()
    }
}
```

Và khi đó, các Handler của bạn có thể sạch bong, chỉ việc trả về `Result` và thả lỗi bằng `?`. Axum sẽ tự hiểu!

```rust
// Trả về `Result` thay vì phải match thủ công
async fn get_product(id: u64) -> Result<Json<Product>, AppError> {
    let product = find_product_in_db(id) // Giả sử hàm này trả về Option
        .ok_or_else(|| AppError::NotFound(format!("Product {} not found", id)))?;
        
    Ok(Json(product))
}
```

---

## 36B.6 — Middleware: Bảo vệ tòa nhà

Middleware là các lớp chặn (intercept) request đến và response đi, giống như cổng bảo vệ quét thẻ khách trước khi vào tòa nhà.

Ví dụ về một Logging Middleware tự chế:

```rust
use axum::{middleware, response::IntoResponse, extract::Request};
use std::time::Instant;

async fn log_request(
    req: Request,
    next: middleware::Next,
) -> impl IntoResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    // Để request đi qua handler thực sự
    let response = next.run(req).await;

    // Đoán xem bao lâu thì chạy xong
    let duration = start.elapsed();
    println!("{} {} → {} ({:?})", method, uri, response.status().as_u16(), duration);

    response
}
```

Gắn nó vào Router như sau:

```rust
let app = Router::new()
    .route("/", get(hello))
    .layer(middleware::from_fn(log_request)); // <--- Đăng ký Middleware
```

---

## 36B.7 — CRUD API hoàn chỉnh: Quản lý sách

Giờ là lúc gom mọi thứ lại: Router + Extractors + State + Error Handling thành một API chuyên nghiệp.

Đầu tiên là định nghĩa Models và State:

```rust
use axum::{
    Router, routing::{get, post, put, delete},
    extract::{State, Path, Query, Json},
    http::StatusCode, response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// --- Models ---
#[derive(Debug, Clone, Serialize)]
struct Book { id: u64, title: String, author: String, price: u32, in_stock: bool }

#[derive(Deserialize)]
struct CreateBook { title: String, author: String, price: u32 }

#[derive(Deserialize)]
struct UpdateBook { price: Option<u32>, in_stock: Option<bool> }

// --- App State ---
#[derive(Clone)]
struct AppState {
    books: Arc<Mutex<HashMap<u64, Book>>>,
    next_id: Arc<Mutex<u64>>,
}
```

Tiếp theo là các Handlers xử lý 4 thao tác cốt lõi C-R-U-D:

```rust
// CREATE — POST /books
async fn create_book(
    State(state): State<AppState>,
    Json(payload): Json<CreateBook>,
) -> Result<(StatusCode, Json<Book>), String> { // Tạm dùng String làm lỗi cho gọn
    
    if payload.title.is_empty() { return Err("Title required".into()); }
    
    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let book = Book {
        id, title: payload.title, author: payload.author, price: payload.price, in_stock: true,
    };
    state.books.lock().unwrap().insert(id, book.clone());
    Ok((StatusCode::CREATED, Json(book)))
}

// READ — GET /books
async fn list_books(State(state): State<AppState>) -> Json<Vec<Book>> {
    let books = state.books.lock().unwrap();
    let mut result: Vec<Book> = books.values().cloned().collect();
    result.sort_by_key(|b| b.id);
    Json(result)
}

// UPDATE — PUT /books/:id
async fn update_book(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, String> {
    let mut books = state.books.lock().unwrap();
    let book = books.get_mut(&id).ok_or("Book not found")?;

    if let Some(price) = payload.price { book.price = price; }
    if let Some(in_stock) = payload.in_stock { book.in_stock = in_stock; }

    Ok(Json(book.clone()))
}

// DELETE — DELETE /books/:id
async fn delete_book(State(state): State<AppState>, Path(id): Path<u64>) -> Result<StatusCode, String> {
    let mut books = state.books.lock().unwrap();
    if books.remove(&id).is_some() { Ok(StatusCode::NO_CONTENT) } 
    else { Err("Book not found".into()) }
}
```

Và luồng Server chính:

```rust
#[tokio::main]
async fn main() {
    let state = AppState {
        books: Arc::new(Mutex::new(HashMap::new())),
        next_id: Arc::new(Mutex::new(1)),
    };

    let app = Router::new()
        .route("/books", get(list_books).post(create_book))
        .route("/books/{id}", put(update_book).delete(delete_book))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Try it!

Bạn có thể tương tác với API này qua Terminal (sử dụng curl):

```bash
# Thêm sách
curl -X POST http://localhost:3000/books -H "Content-Type: application/json" -d '{"title":"Tắt Đèn","author":"Ngô Tất Tố","price":50000}'

# Lấy danh sách
curl http://localhost:3000/books

# Xoá sách
curl -X DELETE http://localhost:3000/books/1
```

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| `error: unmatched route` khi curl | Sai method hoặc path | Kiểm tra `GET` vs `POST`, trailing slash |
| `Json<T>` parse fail → 422 | Body không match `Deserialize` struct | Kiểm tra JSON keys match struct fields |
| `State` not found | Quên `.with_state(state)` trên Router | Thêm `.with_state(state)` cuối cùng |
| "Address already in use" | Port 3000 đã bị chiếm | Đổi port hoặc `kill` process cũ |
| Handler return type complex | Nhiều extractors + error types | Dùng `impl IntoResponse` thay vì Result rối rắm |
| Middleware order sai | `layer` outermost chạy trước | Layer thêm sau = chạy trước (outer) |

---

---

## ✅ Checkpoint 36B

1. Extractor của Axum chạy theo thứ tự nào, và vì sao `Body` phải là extractor cuối?
2. `impl IntoResponse` cho kiểu lỗi domain mang lại điều gì?
3. `State<T>` yêu cầu `T: Clone`. Vì sao đó không phải là vấn đề hiệu năng?

<details>
<summary>Đáp án</summary>

1. Theo thứ tự khai báo trong chữ ký handler. `Body` (và `Json`, `Form`) **tiêu thụ** request body, nên chỉ có một extractor như vậy và nó phải đứng cuối — sau nó không còn gì để đọc.
2. Nó biến lỗi domain thành HTTP response ngay tại ranh giới, nên handler chỉ cần `?`. Đây chính là ROP (Chapter 24) chạm tới tầng web: không còn `match` lỗi rải khắp handler.
3. Vì `T` thường là `Arc<AppState>` hoặc một struct chứa toàn `Arc`/pool — `Clone` chỉ là tăng bộ đếm tham chiếu, không sao chép dữ liệu.
</details>

---

## 🏋️ Bài tập

**Bài 1 (10 phút).** Thêm endpoint `GET /orders/:id` dùng extractor `Path`, trả 404 khi không tìm thấy — qua `IntoResponse` của kiểu lỗi.

**Bài 2 (15 phút).** Viết middleware log mỗi request kèm thời gian xử lý, dùng `tower_http::trace::TraceLayer`.

**Bài 3 (25 phút).** Viết integration test cho toàn bộ router bằng `tower::ServiceExt::oneshot` — không cần bind cổng thật.

<details>
<summary>Gợi ý bài 3</summary>

`app.oneshot(request)` gọi thẳng vào `Router` như một `Service`, bỏ qua hoàn toàn
tầng mạng. Test chạy tính bằng mili-giây và không tranh chấp cổng — đây là cách
test web service nhanh nhất trong hệ sinh thái Rust.
</details>

## Tóm tắt

- ✅ **HTTP** = Request (method + path + body) → Response (status + body). Ẩn dụ quán phở.
- ✅ **Axum** = web framework xịn xò trên `tokio`. Cú pháp `Router::new().route("/path", get(handler))`.
- ✅ **Extractors**: `Path`, `Query`, `Json`, `State` — công cụ "bóc tách" dữ liệu thần thánh.
- ✅ **Error handling**: `impl IntoResponse for AppError` → Cho phép dùng ROP (`Result` + `?`) một cách tự nhiên.
- ✅ **Middleware**: `middleware::from_fn(func)` — Công cụ chặn (intercept) flow để làm Auth, Logging.
- ✅ **CRUD**: `POST` create, `GET` read, `PUT` update, `DELETE` delete — pattern chuẩn cho REST API.

## Tiếp theo

→ Bạn đã nắm được cách dựng Server, bắt lỗi, tách dữ liệu. Tiếp theo, hãy đi sâu vào **Phần 7: Production** để kết nối cái mớ HTTP rỗng tuếch này vào Database thực sự (PostgreSQL) và các luồng kiến trúc cực đỉnh!
