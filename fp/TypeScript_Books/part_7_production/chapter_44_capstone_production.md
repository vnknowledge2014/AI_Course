# Chapter 43 — Capstone Part 2: Production Deployment ⭐

> **Bạn sẽ học được**:
> - Full-stack project: Hono API + PostgreSQL + Redis + JWT
> - Docker containerization — consistent environments
> - CI/CD pipeline — automated testing + deployment
> - Production checklist — HTTPS, logging, monitoring, rate limiting
> - Deployment: Vercel, Railway, Fly.io
> - Putting it ALL together — every pattern from Ch12-42
>
> **Yêu cầu trước**: ALL previous chapters (12-42).
> **Thời gian đọc**: ~60 phút | **Level**: Principal
> **Kết quả cuối cùng**: Sản phẩm hoàn chỉnh từ code → deploy → production.

---

Đây là chương cuối cùng. Bạn đã học tất cả nguyên liệu (types, patterns, architecture) và kỹ thuật nấu (testing, security, design). Bây giờ: MỞ NHÀ HÀNG. Không chỉ nấu ăn giỏi — còn phải có bếp sạch (Docker), bảo vệ (security), nhân viên phục vụ (API), hệ thống đặt bàn (queue), camera an ninh (monitoring), và kế hoạch kinh doanh (scaling).

Chương này không dạy khái niệm MỚI — nó ĐẶT TẤT CẢ khái niệm bạn đã học vào MỘT project hoàn chỉnh. Branded types (Ch15), DU patterns (Ch20), FP DI (Ch19), TDD (Ch31), Hexagonal Architecture (Ch33), security (Ch39-40), observability (Ch41) — tất cả xuất hiện tại đây, làm việc cùng nhau.

Nếu bạn có thể build và deploy project trong chương này — bạn đã sẵn sàng cho production. Không phải vì biết mọi thứ, mà vì có FOUNDATION đúng: type safety, testing, architecture, security. Bất kỳ technology mới nào (nến tảng cloud mới, framework mới, language mới) đều build ON TOP của những fundamentals này.

---

## Capstone Part 2 — Full Production Deployment

Docker (containerization). CI/CD (automated testing + deployment). Production checklist (security, monitoring, backup). Deployment platforms (Vercel, Railway, Fly.io, AWS). Tổng hợp TẤT CẢ 42 chapters trước vào MỘT project hoàn chỉnh.


## 43.1 — Project Architecture

Hãy nhìn tổng thể trước khi đào sâu. Architecture dưới đây tổng hợp TẤT CẢ patterns từ cả cuốn sách: Hexagonal Architecture (Ch33) cho structure, FP DI (Ch19) cho dependency management, Domain types (Ch20) cho business logic, Result/ROP (Ch22) cho error handling, JWT (Ch39) cho auth, Rate limiting (Ch40) cho security.

```typescript
// filename: src/capstone2/architecture.ts

// ┌─────────────────────────────────────────────────┐
// │                  CLIENTS                         │
// │   Browser (React)    Mobile App    3rd Party API │
// └─────────────┬───────────────────────┬───────────┘
//               │                       │
//        ┌──────┴───────────────────────┴──────┐
//        │           LOAD BALANCER              │
//        │         (Nginx / Cloudflare)         │
//        └───────────────┬──────────────────────┘
//                        │
//        ┌───────────────┴──────────────────────┐
//        │           API SERVER (Hono)           │
//        │  ┌─────────────────────────────────┐ │
//        │  │ Middleware: helmet, cors, rate-  │ │
//        │  │ limit, auth, errorHandler        │ │
//        │  ├─────────────────────────────────┤ │
//        │  │ Routes: /api/products, /orders,  │ │
//        │  │ /auth/login, /auth/refresh       │ │
//        │  ├─────────────────────────────────┤ │
//        │  │ Use Cases: confirmOrder,         │ │
//        │  │ registerUser, processPayment     │ │
//        │  ├─────────────────────────────────┤ │
//        │  │ Domain: OrderState, Money,       │ │
//        │  │ Email, pure functions            │ │
//        │  └─────────────────────────────────┘ │
//        └───────┬─────────────────┬────────────┘
//                │                 │
//        ┌───────┴───────┐ ┌──────┴───────┐
//        │  PostgreSQL   │ │    Redis     │
//        │  (Prisma ORM) │ │  (cache,     │
//        │  orders,users │ │   sessions)  │
//        └───────────────┘ └──────────────┘

console.log("Architecture OK ✅");
```

Để tưởng tượng rõ hơn, hãy thấy từng layer map với chapters nào:

| Layer | Concepts | Chapters |
|---|---|---|
| **Domain** | Branded types, DUs, State machines | Ch14, Ch15, Ch20 |
| **Application** | Use cases, Pipelines, Error handling | Ch19, Ch21, Ch22 |
| **Ports** | Repository, Gateway interfaces | Ch24, Ch33 |
| **Infrastructure** | Prisma, Redis, Hono routes | Ch34, Ch37, Ch38 |
| **Cross-cutting** | Auth, Security, Rate limiting | Ch39, Ch40 |
| **Testing** | TDD, PBT, FP DI fakes | Ch31, Ch32, Ch33 |
| **Operations** | Docker, CI/CD, Monitoring | Ch41, Ch42, Ch43 |

---

## 43.2 — Docker

```typescript
// filename: src/capstone2/docker.ts

// === Dockerfile ===
// FROM node:20-alpine AS builder
// WORKDIR /app
// COPY package*.json ./
// RUN npm ci
// COPY . .
// RUN npm run build
//
// FROM node:20-alpine AS runner
// WORKDIR /app
// COPY --from=builder /app/dist ./dist
// COPY --from=builder /app/node_modules ./node_modules
// COPY --from=builder /app/package.json ./
// EXPOSE 3000
// CMD ["node", "dist/index.js"]

// Multi-stage build: builder (compile) → runner (minimal image)
// Alpine: tiny base image (~5MB vs ~900MB full node)

// === docker-compose.yml ===
// services:
//   api:
//     build: .
//     ports: ["3000:3000"]
//     environment:
//       DATABASE_URL: postgresql://user:pass@db:5432/app
//       REDIS_URL: redis://redis:6379
//     depends_on: [db, redis]
//   db:
//     image: postgres:16-alpine
//     environment:
//       POSTGRES_USER: user
//       POSTGRES_PASSWORD: pass
//       POSTGRES_DB: app
//     volumes: ["pgdata:/var/lib/postgresql/data"]
//   redis:
//     image: redis:7-alpine
// volumes:
//   pgdata:

// === Commands ===
// docker compose up -d        ← start all services
// docker compose logs api     ← view logs
// docker compose down         ← stop all

console.log("Docker OK ✅");
```

> **💡 Docker quiz**: Tại sao dùng `node:20-alpine` thay vì `node:20`? Vì alpine = ~5MB base image. Full node = ~900MB. Nhỏ hơn = pull nhanh hơn, deploy nhanh hơn, attack surface nhỏ hơn (security). Trade-off: không có sẵn các tools (bash, curl) — nhưng production không cần.

---

## 43.3 — API Routes (Hono + FP)

Hãy xem API routes thực tế — kết hợp Hono (Ch34) với domain types (Ch20), validation (Ch14/Zod), và error handling (Ch22):

```typescript
// filename: src/capstone2/api_routes.ts
import assert from "node:assert/strict";

// === Domain types (from Ch14, Ch15, Ch20) ===
type OrderId = string & { readonly __brand: "OrderId" };
type Email = string & { readonly __brand: "Email" };
type Money = { readonly amount: number; readonly currency: "USD" | "VND" };

type CreateOrderRequest = {
    customerEmail: string;
    items: { productId: string; quantity: number }[];
};

type OrderResponse = {
    id: OrderId;
    status: "confirmed";
    total: Money;
    createdAt: string;
};

// === Validation (Zod-style, from Ch14) ===
type ValidationError = { field: string; message: string };
type Result<T> = { tag: "ok"; value: T } | { tag: "err"; errors: ValidationError[] };

const validateCreateOrder = (req: CreateOrderRequest): Result<CreateOrderRequest> => {
    const errors: ValidationError[] = [];
    if (!req.customerEmail.includes("@")) errors.push({ field: "email", message: "Invalid email" });
    if (req.items.length === 0) errors.push({ field: "items", message: "At least one item required" });
    for (const item of req.items) {
        if (item.quantity <= 0) errors.push({ field: "quantity", message: `Invalid quantity for ${item.productId}` });
    }
    return errors.length > 0 ? { tag: "err", errors } : { tag: "ok", value: req };
};

// === Use case (from Ch19, Ch21) ===
const createOrder = (req: CreateOrderRequest): Result<OrderResponse> => {
    const validated = validateCreateOrder(req);
    if (validated.tag === "err") return validated;

    // Domain logic: calculate total, create order
    const total: Money = {
        amount: req.items.reduce((sum, i) => sum + i.quantity * 100, 0),  // $1 per unit placeholder
        currency: "USD",
    };

    return {
        tag: "ok",
        value: {
            id: `ORD-${Date.now()}` as OrderId,
            status: "confirmed",
            total,
            createdAt: new Date().toISOString(),
        },
    };
};

// === Route handler pattern (Hono-style) ===
// app.post("/api/orders", async (c) => {
//     const body = await c.req.json<CreateOrderRequest>();
//     const result = createOrder(body);
//     return result.tag === "ok"
//         ? c.json(result.value, 201)
//         : c.json({ errors: result.errors }, 400);
// });

// === Test ===
const goodOrder = createOrder({
    customerEmail: "an@mail.com",
    items: [{ productId: "P1", quantity: 2 }],
});
assert.strictEqual(goodOrder.tag, "ok");

const badOrder = createOrder({
    customerEmail: "not-email",
    items: [],
});
assert.strictEqual(badOrder.tag, "err");
if (badOrder.tag === "err") {
    assert.strictEqual(badOrder.errors.length, 2);  // email + items
}

console.log("API routes OK ✅");
```

> **💡 Nhận ra pattern?** Route handler CHỈ làm: parse request → gọi use case → format response. Tất cả logic nằm trong `createOrder` (pure function). Handler là imperative shell (Ch33). Testable không cần HTTP server.


## 43.4 — CI/CD Pipeline

CI/CD = tự động hóa kiểm tra và deploy. Mỗi push → typecheck + lint + test + build. Nếu bất kỳ step fail → dừng, không deploy. Nếu tất cả pass + push lên main → tự động deploy.

Tại sao CI/CD quan trọng? Vì con người QUÊN. Quay commit, chạy tests, kiểm types, build, deploy — 5 bước mà mỗi người làm 10 lần/ngày. Skip một lần → bug lên production. CI/CD KHÔNG SKIP — máy không quên.

```typescript
// filename: src/capstone2/cicd.ts

// === GitHub Actions (.github/workflows/ci.yml) ===
// name: CI/CD
// on: [push, pull_request]
// jobs:
//   test:
//     runs-on: ubuntu-latest
//     services:
//       postgres:
//         image: postgres:16
//         env: { POSTGRES_USER: test, POSTGRES_PASSWORD: test, POSTGRES_DB: test }
//     steps:
//       - uses: actions/checkout@v4
//       - uses: actions/setup-node@v4
//         with: { node-version: 20 }
//       - run: npm ci
//       - run: npm run typecheck        ← tsc --noEmit
//       - run: npm run lint             ← eslint
//       - run: npm run test             ← vitest run
//       - run: npm run build            ← production build
//
//   deploy:
//     needs: test
//     if: github.ref == 'refs/heads/main'
//     runs-on: ubuntu-latest
//     steps:
//       - uses: actions/checkout@v4
//       - run: npx railway up           ← or vercel deploy

// Pipeline: push → typecheck → lint → test → build → deploy
// Every step must pass before next runs
// Deploy only on main branch, only if all tests pass

console.log("CI/CD OK ✅");
```

---

## 43.5 — Observability: Logging, Metrics, Tracing

Production code không có `console.log`. Production có **structured logging** (JSON logs với fields rõ ràng), **metrics** (số liệu theo thời gian), và **tracing** (theo dõi request qua nhiều services). Ba trụ cột của observability.

```typescript
// filename: src/capstone2/observability.ts
import assert from "node:assert/strict";

// === Structured Logging (Pino-style) ===
type LogLevel = "info" | "warn" | "error" | "debug";
type LogEntry = {
    level: LogLevel;
    msg: string;
    timestamp: string;
    requestId?: string;
    userId?: string;
    durationMs?: number;
    [key: string]: unknown;
};

const createLogger = () => {
    const logs: LogEntry[] = [];
    return {
        log: (level: LogLevel, msg: string, extra: Record<string, unknown> = {}): void => {
            logs.push({ level, msg, timestamp: new Date().toISOString(), ...extra });
        },
        info: (msg: string, extra?: Record<string, unknown>) => logs.push({ level: "info", msg, timestamp: new Date().toISOString(), ...extra }),
        error: (msg: string, extra?: Record<string, unknown>) => logs.push({ level: "error", msg, timestamp: new Date().toISOString(), ...extra }),
        getLogs: () => logs,
    };
};

// === Metrics (Counter, Histogram) ===
type Counter = { name: string; value: number; inc: () => void };
const createCounter = (name: string): Counter => {
    const counter = { name, value: 0, inc: () => { counter.value++; } };
    return counter;
};

const requestCount = createCounter("http_requests_total");
const errorCount = createCounter("http_errors_total");

// Simulate requests
for (let i = 0; i < 100; i++) requestCount.inc();
for (let i = 0; i < 3; i++) errorCount.inc();
assert.strictEqual(requestCount.value, 100);
assert.strictEqual(errorCount.value, 3);

// Error rate = errors / requests
const errorRate = errorCount.value / requestCount.value;
assert.ok(errorRate < 0.05);  // < 5% error rate

// === Request ID correlation ===
// Every request gets unique ID → traces through all services
// Middleware: c.set("requestId", crypto.randomUUID())
// Logger: logger.info("Order created", { requestId, orderId, userId })
// Response header: X-Request-Id: abc-123
// When debugging: grep logs by requestId → see entire request lifecycle

const logger = createLogger();
logger.info("Order created", { requestId: "req-abc", orderId: "ORD-1", durationMs: 42 });
logger.error("Payment failed", { requestId: "req-abc", error: "Card declined" });
assert.strictEqual(logger.getLogs().length, 2);
assert.strictEqual(logger.getLogs()[0].requestId, "req-abc");

console.log("Observability OK ✅");
```

> **💡 Three pillars**: Logs (what happened), Metrics (how many/how fast), Traces (where did the request go). Production KHÔNG THỂ debug bằng `console.log` — bạn cần structured, searchable, correlated data.

---

## 43.6 — Production Checklist & Deployment

### Checklist

Trước khi deploy, đi qua checklist. Mỗi item là một lớp bảo vệ. Thiếu một item = lỗ hổng. Đây là kinh nghiệm từ nhiều năm production incidents — mỗi item tờại một câu chuyện đau đớn (outage, data loss, security breach).

```typescript
// filename: src/capstone2/checklist.ts
import assert from "node:assert/strict";

// === Production Readiness Checklist ===

type ChecklistItem = {
    category: string;
    item: string;
    status: "done" | "pending" | "na";
};

const checklist: ChecklistItem[] = [
    // Security
    { category: "Security", item: "HTTPS enforced (HSTS)", status: "done" },
    { category: "Security", item: "Passwords hashed (argon2)", status: "done" },
    { category: "Security", item: "JWT with refresh tokens", status: "done" },
    { category: "Security", item: "Rate limiting on auth endpoints", status: "done" },
    { category: "Security", item: "CORS configured for allowed origins", status: "done" },
    { category: "Security", item: "Helmet security headers", status: "done" },
    { category: "Security", item: "Input validation (Zod)", status: "done" },
    { category: "Security", item: "SQL injection prevented (Prisma)", status: "done" },

    // Reliability
    { category: "Reliability", item: "Health check endpoint (/health)", status: "done" },
    { category: "Reliability", item: "Graceful shutdown (SIGTERM)", status: "done" },
    { category: "Reliability", item: "Database connection pooling", status: "done" },
    { category: "Reliability", item: "Circuit breaker for external services", status: "done" },
    { category: "Reliability", item: "Retry with exponential backoff", status: "done" },

    // Observability
    { category: "Observability", item: "Structured logging (Pino)", status: "done" },
    { category: "Observability", item: "Request ID correlation", status: "done" },
    { category: "Observability", item: "Error tracking (Sentry)", status: "done" },
    { category: "Observability", item: "Metrics endpoint (/metrics)", status: "done" },

    // Performance
    { category: "Performance", item: "Redis caching for hot paths", status: "done" },
    { category: "Performance", item: "Database indexes on query columns", status: "done" },
    { category: "Performance", item: "CDN for static assets", status: "done" },
    { category: "Performance", item: "Compression (gzip/brotli)", status: "done" },

    // DevOps
    { category: "DevOps", item: "Docker multi-stage build", status: "done" },
    { category: "DevOps", item: "CI/CD pipeline (GitHub Actions)", status: "done" },
    { category: "DevOps", item: "Environment variables for secrets", status: "done" },
    { category: "DevOps", item: "Database migrations automated", status: "done" },
    { category: "DevOps", item: "Zero-downtime deployment", status: "done" },

    // Testing
    { category: "Testing", item: "Unit tests (domain logic)", status: "done" },
    { category: "Testing", item: "Integration tests (API endpoints)", status: "done" },
    { category: "Testing", item: "Property-based tests (serialization)", status: "done" },
    { category: "Testing", item: "CI runs all tests before deploy", status: "done" },
];

const doneCount = checklist.filter(i => i.status === "done").length;
const totalCount = checklist.length;
assert.strictEqual(doneCount, totalCount);

// Group by category
const byCategory = new Map<string, number>();
for (const item of checklist) {
    byCategory.set(item.category, (byCategory.get(item.category) ?? 0) + 1);
}

assert.ok(byCategory.get("Security")! >= 8);
assert.ok(byCategory.get("Reliability")! >= 5);
assert.ok(byCategory.get("DevOps")! >= 5);

console.log(`Production checklist: ${doneCount}/${totalCount} items ✅`);
```

---

### Deployment Options

Chọn platform phù hợp với quy mô và team:

| Platform | Phù hợp nhất cho | Chi phí | Ghi chú |
|---|---|---|---|
| **Vercel** | Next.js, serverless | Free tier, trả theo dùng | Edge functions, auto-scaling |
| **Railway** | Full-stack, databases | $5/tháng credit | Hỗ trợ Docker, PostgreSQL |
| **Fly.io** | Multi-region, độ trễ thấp | Free tier, trả theo dùng | Docker, global edge |
| **AWS ECS** | Enterprise, kiểm soát cao | Phức tạp, trả theo dùng | Full control, học hỏi cao |
| **Render** | Ứng dụng đơn giản | Free tier | Tự động deploy từ Git |

> **💡 Quy tắc chọn platform**: Start với Vercel/Railway (đơn giản nhất). Khi cần control → Fly.io. Enterprise → AWS/GCP. Đừng over-engineer: startup 100 users không cần Kubernetes.

---

## ✅ Checkpoint 43

> Đến đây bạn phải hiểu:
> 1. **Full architecture**: API server + DB + cache + CDN + LB
> 2. **Docker**: multi-stage build, docker-compose for local dev
> 3. **CI/CD**: typecheck → lint → test → build → deploy. Automated
> 4. **Checklist**: security, reliability, observability, performance, devops, testing
>
> **Mọi concept từ Ch12-42 đều present**: branded types, DUs, pipelines, ROP, DTOs, repositories, Functors, Monads, TDD, architecture, security, distributed systems.

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Docker build chậm" | Không cache dependencies | Thêm `COPY package*.json ./` + `RUN npm ci` TRƯỜC `COPY . .` |
| "CI/CD fail bất thường" | Env vars thiếu trong CI | Kiểm tra secrets trong GitHub Settings |
| "Production crash ngay sau deploy" | Thiếu env vars cần thiết | Dùng `requireEnv()` — fail fast khi khởi động |
| "Logs không tìm được request" | Thiếu request ID | Thêm middleware tạo `X-Request-Id` cho mọi request |

---

---

## 💬 Đối thoại với bản thân: Capstone Q&A

**Q: Dự án này có over-engineered không? Khi nào cần tất cả những thứ này?**

A: Nếu làm side project 10 users — đúng, quá nhiều. Nhưng chương này dạy bạn BIẾT các patterns. Khi cần, bạn có sẵn. Bắt đầu: monolith + PostgreSQL + Vercel. Thêm các layers khi CẦN, không phải khi "cool".

**Q: Container + CI/CD — có thực sự cần cho một developer?**

A: Có. Bạn đã bao giờ deploy lên staging, nó chạy. Deploy lên production, nó crashes? Docker = same environment everywhere. CI/CD = không bao giờ quên chạy tests. Hai thứ này SAVE YOUR WEEKEND.

**Q: Observability nghe giống DevOps. Developer cần biết không?**

A: Bạn VIẾT code tạo logs và metrics. DevOps ĐỌc chúng. Nếu developer không thêm structured logs và request IDs — DevOps không đọc được gì. Observability BẮT ĐẦU từ code.

**Q: Tôi học xong 43 chapters. Bước tiếp theo là gì?**

A: BUILD. Lấy một ý tưởng nhỏ (todo app, blog engine, URL shortener). Áp dụng tất cả: branded types, DU state machines, Result pipelines, TDD, Docker, deploy. Bạn sẽ gặp vấn đề MỚI — và đó là lúc bạn THỰC SỰ học.

---

## 🎓 Kết thúc — Sách hoàn thành!

Chúc mừng! Bạn đã đi từ:
- **Ch0-11**: TypeScript fundamentals
- **Ch12-15**: FP foundations (function composition, currying, ADTs, branded types)
- **Ch16-20**: FP thinking (immutability, pure functions, architecture, domain modeling)
- **Ch21-24**: DDD (workflows, error handling, DTOs, repositories)
- **Ch25-30**: FP patterns (Functors, Monads, Applicatives, Monoids, Traverse)
- **Ch31-36**: Testing & full-stack (TDD, PBT, architecture, backend, frontend, capstone)
- **Ch37-43**: Production engineering (databases, security, distributed systems, system design, deployment)

Đây không chỉ là sách TypeScript — đây là hành trình từ JUNIOR đến PRINCIPAL. Từ syntax đến systems. Từ code đến architecture. Từ prototype đến production.

Mỗi chương xây dựng trên chương trước: pure functions (Ch12) → composition (Ch13) → ADTs (Ch14-15) → immutability (Ch16) → architecture (Ch19) → domain modeling (Ch20) → workflows (Ch21) → error handling (Ch22) → FP type classes (Ch26-30) → testing (Ch31-32) → production (Ch37-43). Một đường thẳng từ fundamentals đến production deployment.

Bây giờ bạn có TƯ DUY và CÔNG CỤ. Đi build. Ship with confidence. 🚀

**Build great software. Ship with confidence. 🚀**
