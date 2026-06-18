# Chapter 42 — System Design Thinking

> **Bạn sẽ học được**:
> - Capacity estimation — back-of-the-envelope calculations
> - Load balancing — distribute traffic across servers
> - Caching layers — CDN, Redis, browser, application
> - API design — REST vs gRPC vs GraphQL
> - Microservices vs Monolith — Conway's Law, monolith-first
> - Serverless — Lambda, Vercel, Cloudflare Workers
> - System design exercises — URL shortener, chat, notification system
>
> **Yêu cầu trước**: Chapters 37-41 (all Production Engineering).
> **Thời gian đọc**: ~50 phút | **Level**: Principal
> **Kết quả cuối cùng**: Tư duy system design — architect bằng reasoning, không bằng guessing.

---

Bạn biết quy hoạch thành phố không? Trước khi xây thành phố, cần tính: bao nhiêu người sẽ sống? Cần bao nhiêu đường? Nước + điện cho bao nhiêu hộ? Bệnh viện, trường học đặt Ở ĐÂU? **System design** = quy hoạch cho software: tính capacity, thiết kế data flow, chọn technology, phân chia services.

System design không có đáp án "chính xác" — chỉ có trade-offs. Bạn chọn MySQL hay PostgreSQL? Monolith hay microservices? REST hay GraphQL? Mỗi lựa chọn có ưu và nhược điểm. System design = khả năng REASONING về trade-offs, không phải guessing.

Chương này dạy bạn TƯ DUY system design: bắt đầu với napkin math (nhẩm tính trên giấy), rồi chọn công nghệ, rồi thiết kế layers. Không cần đúng 100% — cần đúng BẬC ĐỘ (độ lớn): 10 QPS khác 10,000 QPS khác 1,000,000 QPS. Mỗi bậc đội kiến trúc KHÁC hoàn toàn.

---

## System Design Thinking — Từ code đến architecture

System design interviews hỏi "thiết kế URL shortener" hay "thiết kế chat system" không phải để test code — mà để test **tư duy trade-offs**. SQL hay NoSQL? Consistent hay available? Monolith hay microservices?

Chapter này dạy bạn framework tư duy: capacity estimation → bottleneck identification → component design → trade-off analysis. Mỗi example đi kèm TypeScript implementation — bạn không chỉ thiết kế trên giấy, bạn build thực.


## 42.1 — Capacity Estimation

Bước đầu tiên của mọi system design: TÍNH. Bao nhiêu user? Bao nhiêu requests/giây? Bao nhiêu data/ngày? Bạn không cần chính xác — cần BIẾT BẬC ĐỘ. 2 requests/giây = một server thừa. 20,000 requests/giây = cần load balancer + nhiều instances. Đây gọi là "back-of-the-envelope" — tính trên mặt sau phong bì.

```typescript
// filename: src/system_design/capacity.ts
import assert from "node:assert/strict";

// Back-of-the-envelope: rough calculations, order-of-magnitude
// Purpose: determine if design is FEASIBLE, not exact numbers

// === Example: E-Commerce Platform ===
const estimate = {
    // Traffic
    dailyActiveUsers: 1_000_000,        // 1M DAU
    avgOrdersPerUser: 0.1,               // 10% place order
    dailyOrders: 1_000_000 * 0.1,        // 100K orders/day
    ordersPerSecond: Math.ceil(100_000 / 86_400), // ~2 ops/sec average

    // Peak: 10x average
    peakOrdersPerSecond: 20,             // Black Friday!

    // Storage
    avgOrderSizeBytes: 2_000,            // 2KB per order
    dailyStorageBytes: 100_000 * 2_000,  // 200MB/day
    yearlyStorageGB: Math.ceil(200 * 365 / 1_000), // ~73GB/year

    // Bandwidth
    avgPageSizeKB: 500,                  // 500KB per page
    dailyPageViews: 5_000_000,           // 5M page views
    dailyBandwidthGB: Math.ceil(5_000_000 * 500 / 1_000_000), // ~2.5TB/day
};

assert.strictEqual(estimate.dailyOrders, 100_000);
assert.ok(estimate.ordersPerSecond <= 2);
assert.ok(estimate.yearlyStorageGB < 100);  // easily fits one server

// === Key Numbers to Remember ===
// 1 day = 86,400 seconds ≈ 100K seconds
// 1 server handles ~1000 concurrent connections
// SSD read: ~0.1ms. Network round-trip: ~1-100ms. DB query: 1-100ms
// 1GB = 1 million KB. 1TB = 1000 GB

console.log("Capacity estimation OK ✅");
```

---

## 42.2 — Caching Layers

Caching là cách HIỆU QUẢ NHẤT để tăng performance. Bạn có 4 lớp cache, mỗi lớp nhanh hơn lớp sau 10-100 lần. Database query 50ms. Redis 0.5ms. CDN hit 10ms thay vì 200ms. Browser cache 0ms (từ local). Chọn đúng lớp = chọn đúng điỉm cân bằng giữa “freshness” và speed.

Nhưng cache có GIÁ: **invalidation**. "Có hai vấn đề khó trong computer science: cache invalidation và naming things" (Phil Karlton). Khi data thay đổi, cache cũ phải được xóa. Chọn: TTL (tự hết hạn), write-through (cập nhật cache khi write), hoặc event-based (domain event trigger invalidation).

```typescript
// filename: src/system_design/caching_layers.ts
import assert from "node:assert/strict";

// 4 layers of caching (fastest → slowest):

// Layer 1: BROWSER cache (Cache-Control headers)
// → Static assets (JS, CSS, images). User's machine.

// Layer 2: CDN (Content Delivery Network)
// → Static + dynamic content. Cloudflare, AWS CloudFront.
// → Closest edge server to user. 10-50ms vs 200ms origin.

// Layer 3: APPLICATION cache (Redis/in-memory)
// → Database query results, computed data, sessions
// → 0.1ms Redis vs 5-50ms DB query. 50-500x faster!

// Layer 4: DATABASE cache (query cache, buffer pool)
// → PostgreSQL automatically caches frequently accessed pages

// === Cache Invalidation Strategies ===
// TTL (Time-To-Live): auto-expire after N seconds
// Write-through: update cache on every write
// Write-behind: batch cache updates asynchronously
// Event-based: invalidate on domain event

type CacheEntry<T> = { value: T; expiresAt: number };

const createTTLCache = <T>(defaultTTLMs = 60_000) => {
    const store = new Map<string, CacheEntry<T>>();

    return {
        get: (key: string): T | undefined => {
            const entry = store.get(key);
            if (!entry) return undefined;
            if (Date.now() > entry.expiresAt) { store.delete(key); return undefined; }
            return entry.value;
        },
        set: (key: string, value: T, ttlMs = defaultTTLMs): void => {
            store.set(key, { value, expiresAt: Date.now() + ttlMs });
        },
        invalidate: (key: string): boolean => store.delete(key),
        size: () => store.size,
    };
};

const cache = createTTLCache<string>(5000);  // 5s TTL

cache.set("user:1", "An");
assert.strictEqual(cache.get("user:1"), "An");

cache.invalidate("user:1");
assert.strictEqual(cache.get("user:1"), undefined);

console.log("Caching layers OK ✅");
```

---

## 42.3 — API Design: REST vs gRPC vs GraphQL

```typescript
// filename: src/system_design/api_comparison.ts

// === REST (most common) ===
// GET /api/products/123      → read one
// POST /api/products          → create
// PUT /api/products/123       → update
// DELETE /api/products/123    → delete
// Pros: simple, cacheable, universal
// Cons: over-fetching, under-fetching, N+1

// === GraphQL ===
// POST /graphql
// query { product(id: "123") { name, price, reviews { text } } }
// Pros: client chooses fields, no over-fetching, single endpoint
// Cons: caching harder, complexity, N+1 (need DataLoader)

// === gRPC ===
// Proto definition → generated client/server code
// Binary format (protobuf) → 10x smaller than JSON
// Pros: fast, typed, streaming, bi-directional
// Cons: not browser-native (need proxy), debugging harder

// === When to use ===
// REST: public APIs, simple CRUD, browser clients
// GraphQL: complex data relationships, mobile/frontend teams want flexibility
// gRPC: internal microservices, low latency, streaming

// === TypeScript libraries ===
// REST: Hono, Express, Fastify
// GraphQL: graphql-yoga, Apollo Server
// gRPC: nice-grpc, @grpc/grpc-js

console.log("API comparison OK ✅");
```

---

## 42.4 — Microservices vs Monolith

```typescript
// filename: src/system_design/architecture_choice.ts
import assert from "node:assert/strict";

// === Monolith-First ===
// Start with monolith → extract services when NEEDED
// Martin Fowler: "you shouldn't start with microservices"

// Monolith benefits: simple deploy, easy debug, no network overhead
// Monolith problems: hard to scale parts independently, tight coupling

// === When to break into services ===
// 1. Team grows > 8-10 people (Conway's Law: org = architecture)
// 2. Different scaling needs (product catalog vs order processing)
// 3. Different technology needs (ML service in Python, API in TypeScript)
// 4. Independent deploy needed

// === Modular Monolith (middle ground) ===
// Monolith but with clear module boundaries (Ch33!)
// Each module: own domain, own database schema, public API via barrel exports
// Can extract to service later if needed

type Module = {
    name: string;
    ownsTables: string[];
    publicAPI: string[];    // exported functions/types
    dependsOn: string[];    // other modules
};

const modules: Module[] = [
    {
        name: "orders",
        ownsTables: ["orders", "order_items"],
        publicAPI: ["createOrder", "confirmOrder", "Order", "OrderId"],
        dependsOn: ["products", "users"],
    },
    {
        name: "products",
        ownsTables: ["products", "categories"],
        publicAPI: ["findProduct", "Product", "ProductId"],
        dependsOn: [],
    },
    {
        name: "users",
        ownsTables: ["users", "user_profiles"],
        publicAPI: ["findUser", "User", "UserId"],
        dependsOn: [],
    },
];

// Dependency validation: no circular deps
const hasCyclicDeps = (mods: Module[]): boolean => {
    const visited = new Set<string>();
    const visiting = new Set<string>();

    const dfs = (name: string): boolean => {
        if (visiting.has(name)) return true;  // cycle!
        if (visited.has(name)) return false;
        visiting.add(name);
        const mod = mods.find(m => m.name === name);
        if (mod) {
            for (const dep of mod.dependsOn) {
                if (dfs(dep)) return true;
            }
        }
        visiting.delete(name);
        visited.add(name);
        return false;
    };

    return mods.some(m => dfs(m.name));
};

assert.strictEqual(hasCyclicDeps(modules), false);  // no cycles!

console.log("Architecture choice OK ✅");
```

---

## ✅ Checkpoint 42

> Đến đây bạn phải hiểu:
> 1. **Capacity estimation**: back-of-envelope. DAU → QPS → storage → bandwidth
> 2. **Caching layers**: browser → CDN → Redis → DB. Each 10-100x faster
> 3. **REST vs GraphQL vs gRPC**: different trade-offs, different use cases
> 4. **Monolith-first**: start simple, extract services when complexity justifies
> 5. **Modular monolith**: best of both — monolith deploy, microservice boundaries

---

## 🏋️ Bài tập

**Bài 1** (15 phút): Thiết kế URL Shortener.

<details><summary>✅ Lời giải</summary>

```
Requirements:
- Shorten URL: POST /api/shorten { url: "long-url" } → { shortCode: "abc123" }
- Redirect: GET /r/abc123 → 301 redirect to original URL
- Daily: 1M new URLs, 100M redirects

Capacity:
- Storage: 1M * 1KB = 1GB/day = 365GB/year → single PostgreSQL
- Redirects: 100M/day ÷ 86400 ≈ 1200/sec → one server + Redis cache
- Short code: base62(incrementing ID) → 6 chars = 62^6 = 56B combinations

Architecture:
┌──────────┐     ┌───────────┐     ┌──────────┐
│  CLIENT  │ ──→ │   NGINX   │ ──→ │  API     │
│          │ ←── │ (LB,cache)│ ←── │ (Hono)   │
└──────────┘     └───────────┘     └────┬─────┘
                                        │
                              ┌─────────┴─────────┐
                              │  Redis (cache)     │
                              │  shortCode → URL   │
                              └─────────┬─────────┘
                                        │ miss
                              ┌─────────┴─────────┐
                              │  PostgreSQL        │
                              │  urls table        │
                              └───────────────────┘
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Hệ thống chậm dưới tải" | Thiếu cache hoặc index | Thêm Redis cache + kiểm tra EXPLAIN ANALYZE |
| "Microservices quá phức tạp" | Chia sớm, chưa cần | Bắt đầu với monolith, tách khi cần |
| "Capacity ước lượng sai" | Quên peak hours | Nhân trung bình × 10 cho peak |
| "API chậm" | Over-fetching, thiếu pagination | Paginate mặc định, chỉ trả fields cần thiết |

---

## Tóm tắt

System design = tư duy về trade-offs ở quy mô lớn. Không có thiết kế hoàn hảo — chỉ có thiết kế PHÙ HỢP với constraints. Bắc độ khác → xử lý khác. 100 users ≠ 1M users &ne; 100M users.

- ✅ **Capacity estimation**: napkin math. DAU → QPS → storage.
- ✅ **Caching**: 4 lớp. TTL, invalidation, write-through.
- ✅ **API**: REST (đơn giản), GraphQL (linh hoạt), gRPC (nhanh).
- ✅ **Monolith-first**: bắt đầu đơn giản. Tách khi quy mô/tổ chức yêu cầu.
- ✅ **System design = trade-offs**: không có giải pháp hoàn hảo, chỉ có trade-offs.

## Tiếp theo

→ Chapter 43: **Capstone Part 2 — Production Deployment** ⭐ — full-stack project, deployed, production-ready. Tất cả chương hội tụ.
