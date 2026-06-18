# Chapter 43 — System Design Thinking

> **Bạn sẽ học được**:
> - **Capacity estimation**: QPS, storage, bandwidth — back-of-envelope
> - **Load balancing**: L4 vs L7, round-robin, consistent hashing
> - **Caching layers**: client → CDN → app → DB
> - **API design**: REST vs gRPC vs GraphQL
> - **Microservices**: khi nào split, monolith first
> - **Design exercises**: URL shortener, chat system
>
> **Yêu cầu trước**: Chapter 42 (Distributed Systems).
> **Thời gian đọc**: ~45 phút | **Level**: Principal
> **Kết quả cuối cùng**: Bạn **tư duy hệ thống** — trade-offs, bottlenecks, scalability.

---

## System Design Thinking — Từ code đến kiến trúc

System design là kỹ năng khác biệt lớn nhất giữa junior và senior engineer. Không phải vì syntax hay algorithms — mà vì khả năng **đánh giá trade-offs** ở mức hệ thống: SQL hay NoSQL? Monolith hay microservices? REST hay gRPC?

Chapter này dạy bạn **framework tư duy** cho system design: capacity estimation, bottleneck identification, caching strategies, API design. Mỗi section đi kèm Rust implementation examples.

---

## 43.1 — Capacity Estimation (Back-of-Envelope)

Trước khi viết dòng code đầu tiên cho hệ thống lớn, bạn cần biết: bao nhiêu requests/giây? Cần bao nhiêu ổ cứng? Bandwidth bao nhiêu? Không cần chính xác — chỉ cần ước lượng đủ để biết cần 1 server hay 100.

### Con số mọi kỹ sư nên biết

| Thao tác | Độ trễ | Ghi chú |
|-----------|---------|------|
| L1 cache | 1 ns | Thanh ghi CPU |
| L2 cache | 4 ns | |
| Truy cập RAM | 100 ns | |
| Đọc SSD | 150 μs | 150.000 ns |
| Tìm HDD | 10 ms | 10.000.000 ns |
| Mạng nội bộ (cùng DC) | 500 μs | |
| Mạng xuyên lục địa | 150 ms | |

### Bài tập ước lượng: Ứng dụng mạng xã hội

```
Given:
  - 10M daily active users (DAU)
  - Each user: 20 page views/day
  - Each page: 300KB average

QPS (Queries Per Second):
  10M × 20 / 86,400 ≈ 2,300 QPS
  Peak (2× average): ~5,000 QPS

Bandwidth:
  2,300 × 300KB = 690 MB/s ≈ 700 MB/s

Storage (1 year, posts only):
  10M users × 2 posts/day × 365 = 7.3B posts
  Average post: 500 bytes
  7.3B × 500B = 3.65 TB/year

Database reads:
  2,300 QPS × 5 queries/page = 11,500 DB QPS
  → Need read replicas or caching!
```

### Bảng tra nhanh ước lượng

```
1 day  = 86,400 seconds ≈ 100,000 (for estimation)
1 year = 31.5M seconds  ≈ 30M

1 KB = 1,000 bytes (text content)
1 MB = 1,000 KB (image)
1 GB = 1,000 MB (video)
1 TB = 1,000 GB (database)

QPS = DAU × actions_per_user / 86,400
Peak QPS ≈ 2-3 × average QPS
```

---

## 43.2 — Load Balancing

Khi 1 server không đủ, bạn thêm server. Nhưng ai quyết định request nào vào server nào? Load balancer. Có nhiều thuật toán — từ đơn giản (luân phiên) đến thông minh (consistent hashing để cache không bị invalidate khi thêm server).

### L4 vs L7

```
L4 (Transport):                        L7 (Application):
  Routes by IP + Port                    Routes by HTTP path/header
  Fast, simple                           Content-aware
  TCP/UDP level                          HTTP level

  Client → LB → Server                  Client → LB → /api → API servers
                                                      → /static → CDN
                                                      → /ws → WebSocket servers
```

### Thuật toán phân tải

| Thuật toán | Cách hoạt động | Phù hợp với |
|-----------|-----|----------|
| **Round-Robin** | 1, 2, 3, 1, 2, 3... | Các server giống nhau |
| **Weighted RR** | Server A: 3×, B: 1× | Cấu hình khác nhau |
| **Least Connections** | Gửi đến server rảnh nhất | Kết nối dài hạn |
| **IP Hash** | Hash(IP) mod N | Gắn session |
| **Consistent Hashing** | Vòng hash | Cache servers (thêm/bớt node) |

### Consistent Hashing

```rust
// filename: src/main.rs

use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct ConsistentHash {
    ring: BTreeMap<u64, String>,
    virtual_nodes: u32,
}

impl ConsistentHash {
    fn new(virtual_nodes: u32) -> Self {
        ConsistentHash { ring: BTreeMap::new(), virtual_nodes }
    }

    fn add_node(&mut self, node: &str) {
        for i in 0..self.virtual_nodes {
            let key = hash_key(&format!("{}:{}", node, i));
            self.ring.insert(key, node.into());
        }
    }

    fn remove_node(&mut self, node: &str) {
        for i in 0..self.virtual_nodes {
            let key = hash_key(&format!("{}:{}", node, i));
            self.ring.remove(&key);
        }
    }

    fn get_node(&self, key: &str) -> Option<&String> {
        if self.ring.is_empty() { return None; }
        let hash = hash_key(key);
        // Find next node clockwise on the ring
        self.ring.range(hash..).next()
            .or_else(|| self.ring.iter().next()) // wrap around
            .map(|(_, node)| node)
    }
}

fn hash_key(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let mut ring = ConsistentHash::new(100); // 100 virtual nodes per server

    ring.add_node("server-1");
    ring.add_node("server-2");
    ring.add_node("server-3");

    // Route requests
    let keys = ["user:100", "user:200", "user:300", "order:1", "order:2"];
    println!("=== 3 servers ===");
    for key in &keys {
        println!("  {} → {}", key, ring.get_node(key).unwrap());
    }

    // Add server (minimal disruption!)
    ring.add_node("server-4");
    println!("\n=== 4 servers (added server-4) ===");
    for key in &keys {
        println!("  {} → {}", key, ring.get_node(key).unwrap());
    }
    // Most keys stay on same server!
}
```

---

## 43.3 — Caching Layers

Request đi qua nhiều lớp trước khi đến database. Mỗi lớp cache giữ lại kết quả, lớp sau nhận ít tải hơn. Giống bề mặt nước: 95% requests chưa bao giờ chạm database.

```
Request flow:
  Client → Browser Cache → CDN → App Cache (Redis) → DB Cache → Database

┌──────────┐  ┌──────┐  ┌───────────┐  ┌─────────┐  ┌──────────┐
│ Browser  │→ │ CDN  │→ │ App Cache │→ │ DB Cache│→ │ Database │
│          │  │      │  │ (Redis)   │  │ (query) │  │          │
│ Local    │  │ Edge │  │ In-memory │  │ Buffer  │  │ Disk     │
│ <1ms     │  │ 10ms │  │ <1ms      │  │ <10ms   │  │ 10-100ms │
└──────────┘  └──────┘  └───────────┘  └─────────┘  └──────────┘
      95%        90%         80%           50%           100%
      hit        hit         hit           hit           miss=query
```

### Khi nào cache cái gì

| Lớp | Cache cái gì | TTL | Invalidation |
|-------|--------------|-----|-------------|
| **Trình duyệt** | File tĩnh (JS, CSS, ảnh) | 1 năm (theo version) | Deploy mới = URL mới |
| **CDN** | Trang công khai, media | 1 giờ - 1 ngày | Purge API |
| **App (Redis)** | Kết quả query DB, sessions | 5 phút - 1 giờ | Khi ghi, theo event |
| **DB** | Cache kế hoạch query | Tự động | Thay đổi schema |

---

## 43.4 — API Design

API là "hợp đồng" giữa frontend và backend. Chọn sai format hoặc thiết kế kém = đau khổ lâu dài. 3 lựa chọn chính: REST (public, dễ debug), gRPC (nội bộ, nhanh), GraphQL (frontend linh hoạt).

### REST vs gRPC vs GraphQL

| | REST | gRPC | GraphQL |
|---|---|---|---|
| **Định dạng** | JSON | Protobuf (binary) | JSON |
| **Giao thức** | HTTP/1.1 | HTTP/2 | HTTP |
| **An toàn kiểu** | Bên ngoài (OpenAPI) | Tích hợp (.proto) | Tích hợp (schema) |
| **Streaming** | SSE/WebSocket | Hai chiều | Subscriptions |
| **Phù hợp nhất** | API công khai | Microservices nội bộ | Frontend linh hoạt |
| **Hiệu năng** | Trung bình | Nhanh (nhỏ 10×) | Tùy thuộc |

### REST design principles

```
Resources (nouns, not verbs):
  GET    /api/orders          → list orders
  POST   /api/orders          → create order
  GET    /api/orders/123      → get order #123
  PUT    /api/orders/123      → replace order
  PATCH  /api/orders/123      → partial update
  DELETE /api/orders/123      → delete order

  GET    /api/orders/123/items → list items of order 123

Status codes:
  200 OK              → success
  201 Created         → resource created
  204 No Content      → success, no body (delete)
  400 Bad Request     → validation error
  401 Unauthorized    → not authenticated
  403 Forbidden       → not authorized
  404 Not Found       → resource doesn't exist
  409 Conflict        → duplicate, stale data
  429 Too Many Reqs   → rate limited
  500 Internal Error  → server bug
```

### API versioning

```
URL path:      /api/v1/orders    → simple, explicit
Header:        Accept: application/vnd.api.v2+json
Query param:   /api/orders?version=2

Recommendation: URL path for major versions ✅
```

---

## 43.5 — Microservices vs Monolith

Câu hỏi quen thuộc: "nên dùng microservices không?" Câu trả lời: **monolith trước**, tách sau khi có lý do rõ ràng. Microservices giải quyết vấn đề của team 50+ người, nhưng tạo vấn đề mới (distributed transactions, deployment phức tạp) cho team nhỏ.

### Evolution path

```
┌────────────┐     ┌─────────────────┐     ┌───────────────────┐
│ Monolith   │ ──▶ │ Modular         │ ──▶ │ Microservices     │
│            │     │ Monolith        │     │                   │
│ All in one │     │ Clear modules   │     │ Independent deploy│
│ 1 deploy   │     │ 1 deploy        │     │ N deploys         │
│ Simple ops │     │ Moderate ops    │     │ Complex ops       │
│            │     │                 │     │                   │
│ Start here │     │ 10-50 devs     │     │ 50+ devs          │
└────────────┘     └─────────────────┘     └───────────────────┘
```

### Khi nào tách

| Tín hiệu | Hành động |
|--------|--------|
| Nhu cầu scale khác nhau | Tách: API vs background worker |
| Tần suất deploy khác nhau | Tách: auth service (ổn định) vs feature (nhanh) |
| Ranh giới team (Conway's Law) | Tách: team A sở hữu service A |
| Database chung bị nghẽ | Tách: mỗi service sở hữu data riêng |
| >50 lập trình viên trên 1 repo | Cân nhắc tách |

### Khi nào KHÔNG nên tách

| Tín hiệu | Giữ monolith |
|--------|--------------|
| < 10 lập trình viên | Chi phí quản lý > lợi ích |
| Startup / MVP | Tốc độ quan trọng hơn |
| Data liên kết chặt | Distributed transactions = đau đầu |
| Không có DevOps | Microservices cần tự động hóa |

---

## 43.6 — Design Exercise: URL Shortener

Đây là bài tập thiết kế kinh điển: ghép capacity estimation, API design, caching, và database sharding vào một hệ thống duy nhất. Bạn sẽ thấy các concepts ở trên được áp dụng cụ thể.

### Requirements

```
Functional:
  - Shorten URL: long → short (e.g., bit.ly/abc123)
  - Redirect: short → long (302 redirect)
  - Optional: analytics (click count, geo)

Non-functional:
  - 100M URLs created/month
  - 10:1 read/write ratio → 1B redirects/month
  - URL expires after 5 years
  - High availability (AP system)
```

### Back-of-envelope

```
Write QPS: 100M / (30 × 86,400) ≈ 40 QPS
Read QPS: 40 × 10 = 400 QPS (peak: ~1,000)
Storage: 100M × 12 months × 5 years × 500B ≈ 3 TB
```

### Design

```rust
// filename: src/main.rs

use std::collections::HashMap;

// ═══ URL Shortener design ═══

const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn id_to_short(mut id: u64) -> String {
    if id == 0 { return "0".into(); }
    let mut result = vec![];
    while id > 0 {
        result.push(BASE62[(id % 62) as usize] as char);
        id /= 62;
    }
    result.into_iter().rev().collect()
}

struct UrlShortener {
    urls: HashMap<String, UrlEntry>,
    next_id: u64,
}

struct UrlEntry {
    long_url: String,
    short_code: String,
    clicks: u64,
}

impl UrlShortener {
    fn new() -> Self { UrlShortener { urls: HashMap::new(), next_id: 1_000_000 } }

    fn shorten(&mut self, long_url: &str) -> String {
        let code = id_to_short(self.next_id);
        self.next_id += 1;
        let short = format!("https://short.vn/{}", code);
        self.urls.insert(code.clone(), UrlEntry {
            long_url: long_url.into(), short_code: code, clicks: 0,
        });
        short
    }

    fn redirect(&mut self, code: &str) -> Option<&str> {
        self.urls.get_mut(code).map(|entry| {
            entry.clicks += 1;
            entry.long_url.as_str()
        })
    }

    fn stats(&self, code: &str) -> Option<u64> {
        self.urls.get(code).map(|e| e.clicks)
    }
}

fn main() {
    let mut shortener = UrlShortener::new();

    let short1 = shortener.shorten("https://example.com/very/long/path/to/page");
    let short2 = shortener.shorten("https://rust-lang.org/book/chapter42");

    println!("Short URLs:");
    println!("  {} → long URL", short1);
    println!("  {} → long URL", short2);

    // Simulate redirects
    let code = short1.split('/').last().unwrap();
    for _ in 0..5 { shortener.redirect(code); }
    println!("\nClicks on {}: {}", code, shortener.stats(code).unwrap());

    println!("\nArchitecture:");
    println!("  Write: Client → API → ID Generator → DB (write)");
    println!("  Read:  Client → CDN → Cache (Redis) → DB (fallback)");
    println!("  Cache hit ratio: ~99% (URLs rarely change)");
}
```

### Architecture diagram

```
                        ┌──────────┐
  Client ──── LB ──────▶│ API      │
                        │ Servers  │
                        │ (stateless)
                        └────┬─────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
         ┌─────────┐  ┌──────────┐  ┌────────────┐
         │ Redis   │  │ ID Gen   │  │ Analytics  │
         │ Cache   │  │ (Snowflake│  │ (Kafka →   │
         │         │  │  or auto) │  │  ClickHouse)│
         └─────────┘  └──────────┘  └────────────┘
              │              │
              └──────┬───────┘
                     ▼
              ┌──────────┐
              │ Database │
              │ (sharded │
              │  by code)│
              └──────────┘
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Ước lượng capacity

Estimate cho chat app: 5M DAU, 50 messages/user/day, avg 200 bytes/message.
Calculate: QPS, daily storage, yearly storage.

<details><summary>✅ Lời giải</summary>

```
Messages/day: 5M × 50 = 250M messages/day
QPS: 250M / 86,400 ≈ 2,900 QPS (peak: ~6,000)
Daily storage: 250M × 200B = 50 GB/day
Yearly storage: 50 × 365 = 18.25 TB/year
Bandwidth: 2,900 × 200B ≈ 580 KB/s (very manageable)
```

</details>

---

**Bài 2** (10 phút): Thiết kế API

Design REST API cho blog platform:
- Posts (CRUD, list with pagination)
- Comments (nested under posts)
- Tags (many-to-many)
- Search

<details><summary>✅ Lời giải Bài 2</summary>

```
Posts:
  GET    /api/v1/posts?page=1&limit=20&tag=rust    → list
  POST   /api/v1/posts                              → create
  GET    /api/v1/posts/:id                           → get
  PUT    /api/v1/posts/:id                           → update
  DELETE /api/v1/posts/:id                           → delete

Comments:
  GET    /api/v1/posts/:id/comments                  → list
  POST   /api/v1/posts/:id/comments                  → create
  DELETE /api/v1/posts/:id/comments/:commentId       → delete

Tags:
  GET    /api/v1/tags                                → list all
  GET    /api/v1/tags/:tag/posts                     → posts by tag

Search:
  GET    /api/v1/search?q=rust+fp&type=post          → full-text search

Pagination response:
  { "data": [...], "meta": { "page": 1, "total": 42, "has_next": true } }
```

</details>

---

**Bài 3** (15 phút): Thiết kế hệ thống thông báo

Thiết kế hệ thống gửi thông báo (email, push, SMS) cho 1 triệu người dùng:
- Yêu cầu: cấu hình kênh theo từng user, retry khi lỗi, rate limit
- Vẽ kiến trúc, ước lượng capacity, chọn components

<details><summary>✅ Lời giải Bài 3</summary>

```
Architecture:
  Event Source → Message Queue (Kafka) → Notification Service → Channels
                                              │
                                    ┌─────────┼──────────┐
                                    ▼         ▼          ▼
                                  Email     Push       SMS
                                  (SES)    (FCM/APNs) (Twilio)

Components:
  - Kafka: buffer + retry (at-least-once)
  - Redis: user preferences cache + dedup + rate limit
  - PostgreSQL: notification log + audit trail
  - Worker pool: 10 consumers, 100 concurrent per consumer

Capacity:
  1M notifications → 1M / 3600 ≈ 278 QPS (spread over 1 hour)
  Peak: 1,000 QPS (batch send)
  Storage: 1M × 500B = 500 MB per batch
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Giải pháp |
|---------|-------------|-----------|
| "Tính QPS sai" | Quên hệ số peak | Peak = 2-3× trung bình |
| "Cache miss ồ ạt" | Key phổ biến hết hạn cùng lúc | Giãn TTL, lock per key |
| "Monolith quá lớn" | 100+ lập trình viên, deploy chậm | Bắt đầu với modular monolith |
| "Micro quá phức tạp" | 3 người, 20 services | Gộp lại, monolith trước! |

---

## Tóm tắt

- ✅ **Capacity estimation**: QPS, storage, bandwidth — back-of-envelope with known numbers.
- ✅ **Load balancing**: Round-robin (simple), consistent hashing (cache-friendly).
- ✅ **Caching layers**: Browser → CDN → Redis → DB. Each layer = less load on next.
- ✅ **API design**: REST (public), gRPC (internal), GraphQL (flexible frontend).
- ✅ **Microservices**: Monolith first → modular monolith → split when needed.
- ✅ **Design exercise**: URL shortener — estimation, API, caching, sharding.

## Tiếp theo

→ Chapter 44: **Capstone Part 2 — Production Deployment** ⭐ — The final chapter! Order-Taking System + PostgreSQL, Redis, JWT auth, Docker, CI/CD, monitoring.
