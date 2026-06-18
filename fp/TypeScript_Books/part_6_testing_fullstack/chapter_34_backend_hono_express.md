# Chapter 34 — Backend: Hono / Express

> **Bạn sẽ học được**:
> - REST API design với FP patterns
> - Hono framework — lightweight, type-safe, edge-ready
> - Middleware as function composition — logging, auth, error handling
> - Request validation → domain logic → response mapping (pipeline!)
> - Error handling middleware — consistent API error responses
> - Database integration via repository pattern (Ch24)
>
> **Yêu cầu trước**: Chapter 19 (FP DI), Chapter 23 (DTOs), Chapter 24 (Repository), Chapter 33 (Architecture).
> **Thời gian đọc**: ~50 phút | **Level**: Advanced
> **Kết quả cuối cùng**: REST API với typed errors, validated requests, domain logic — production-ready.

---

Bạn biết quầy phục vụ bureau (reception desk) trong khách sạn không? Khách đến, nói yêu cầu (request). Receptionist kiểm tra (validate), gọi bộ phận phù hợp (domain service), nhận kết quả, trả lời khách (response). Receptionist KHÔNG dọn phòng — receptionist chỉ điều phối. Nếu có vấn đề, receptionist trả lời lịch sự với mã lỗi (404: phòng không tồn tại, 401: chưa đặt phòng).

**API handler** = receptionist. Nhận request → validate → gọi domain → format response. Handler KHÔNG chứa business logic — handler chỉ điều phối.

---

## Backend Development — Hono & Express

Đây là chapter thực hành — xây dựng REST API hoàn chỉnh.

Hono là framework web mới, nhẹ, type-safe, chạy trên mọi runtime (Node, Deno, Bun, Cloudflare Workers). Express là standard trong 10 năm qua. Chapter này dạy cả hai — Hono cho type-safety và edge deployment, Express cho legacy compatibility và ecosystem.

Pattern chung: routes → middleware → handlers → services → repositories. Mỗi layer testable riêng biệt. Error handling dùng `Result` pattern từ Ch22. Validation dùng Zod từ Ch14.


## Backend with Hono — API as pipelines

Hono = lightweight TypeScript framework. Routes = functions. Middleware = composition. Request → validate → process → respond = pipeline từ Ch21. Type-safe, edge-ready, 10x faster than Express.


## 34.1 — Hono: Modern TypeScript API Framework

```typescript
// filename: src/backend/hono_setup.ts

// Hono = lightweight web framework
// - Type-safe route params, query, body
// - Middleware composition
// - Works on: Node, Deno, Bun, Cloudflare Workers, Vercel Edge

// Installation: npm install hono
// Basic structure:

// import { Hono } from 'hono';
// const app = new Hono();
//
// app.get('/api/health', (c) => c.json({ status: 'ok' }));
//
// app.get('/api/users/:id', async (c) => {
//     const id = c.req.param('id');
//     const user = await findUser(id);
//     return user ? c.json(user) : c.json({ error: 'Not found' }, 404);
// });
//
// export default app;

console.log("Hono setup concept OK ✅");
```

### API Handler as Pipeline

```typescript
// filename: src/backend/handler_pipeline.ts
import assert from "node:assert/strict";

// Simulate Hono context
type Context = {
    req: {
        param: (key: string) => string;
        json: () => Promise<unknown>;
    };
    json: (data: unknown, status?: number) => Response;
};

// === Domain types ===
type ProductId = string & { __brand: "ProductId" };
type Money = number & { __brand: "Money" };
const Money = (n: number): Money => n as Money;

type Product = {
    readonly id: ProductId;
    readonly name: string;
    readonly price: Money;
    readonly stock: number;
};

type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };

// === Repository port ===
type ProductRepo = {
    readonly findById: (id: ProductId) => Promise<Product | null>;
    readonly save: (p: Product) => Promise<void>;
};

// === Request DTO + validation ===
type CreateProductRequest = {
    readonly name: string;
    readonly price: number;
    readonly stock: number;
};

type ValidationError = { field: string; message: string };

const validateCreateProduct = (body: unknown): Result<CreateProductRequest, ValidationError[]> => {
    if (typeof body !== "object" || body === null) {
        return { tag: "err", error: [{ field: "body", message: "Expected object" }] };
    }
    const obj = body as Record<string, unknown>;
    const errors: ValidationError[] = [];

    if (typeof obj.name !== "string" || obj.name.length < 2) {
        errors.push({ field: "name", message: "Min 2 chars" });
    }
    if (typeof obj.price !== "number" || obj.price <= 0) {
        errors.push({ field: "price", message: "Must be > 0" });
    }
    if (typeof obj.stock !== "number" || !Number.isInteger(obj.stock) || obj.stock < 0) {
        errors.push({ field: "stock", message: "Non-negative integer" });
    }

    return errors.length > 0
        ? { tag: "err", error: errors }
        : { tag: "ok", value: obj as unknown as CreateProductRequest };
};

// === Response DTO ===
type ProductResponse = {
    readonly id: string;
    readonly name: string;
    readonly price: number;
    readonly stock: number;
};

const toResponse = (p: Product): ProductResponse => ({
    id: p.id, name: p.name, price: p.price, stock: p.stock,
});

// === Pipeline: validate → domain → respond ===
// const createProductHandler = async (c: Context, deps: { repo: ProductRepo; genId: () => ProductId }) => {
//     // 1. Parse + validate request
//     const body = await c.req.json();
//     const validated = validateCreateProduct(body);
//     if (validated.tag === "err") {
//         return c.json({ error: { code: "VALIDATION_ERROR", details: validated.error } }, 422);
//     }
//     
//     // 2. Domain logic (pure)
//     const product: Product = {
//         id: deps.genId(),
//         name: validated.value.name,
//         price: Money(validated.value.price),
//         stock: validated.value.stock,
//     };
//     
//     // 3. Persist (IO)
//     await deps.repo.save(product);
//     
//     // 4. Map to response DTO
//     return c.json(toResponse(product), 201);
// };

// Test validation separately (it's pure!)
const good = validateCreateProduct({ name: "Laptop", price: 20000000, stock: 5 });
assert.strictEqual(good.tag, "ok");

const bad = validateCreateProduct({ name: "X", price: -1, stock: 1.5 });
assert.strictEqual(bad.tag, "err");
if (bad.tag === "err") assert.strictEqual(bad.error.length, 3);

console.log("Handler pipeline OK ✅");
```

> **💡 Handler = receptionist**: 1) Validate request (parse body). 2) Call domain logic (pure). 3) Persist (IO). 4) Map to response DTO. Handler contains NO business logic — just plumbing.

---

Chú ý flow trong handler: **validate → domain → persist → respond**. Mỗi bước là function riêng, testable riêng. `validateCreateProduct` test mà không cần server. Domain logic (`calculateTotal`) test mà không cần database. Chỉ integration test mới cần cả pipeline.

So sánh với Express handler "truyền thống" mà hầu hết tutorials dạy: mọi thứ trong 1 function — validate inline, query DB inline, respond inline. 200 dòng, test = phải mock request/response objects. Refactor = sợ break everything.

Pipeline pattern tách mọi thứ: validation là pure function, domain logic là pure function, persistence dùng injected repository. Mỗi phần nhỏ, dễ hiểu, dễ test. Handler chỉ **orchestrate** — giống receptionist trong analogy đầu chapter.

## ✅ Checkpoint 34.1

> Đến đây bạn phải hiểu:
> 1. **Hono** = lightweight, type-safe API framework for TypeScript
> 2. **Handler pipeline**: validate → domain → persist → respond
> 3. **Validation is pure** — test separately, no server needed
> 4. **DTOs**: request DTO (inbound) + response DTO (outbound) from Ch23
>
> **Test nhanh**: Business rule "price cannot exceed 1 billion" — goes in handler or domain?
> <details><summary>Đáp án</summary>**DOMAIN!** Business rules = domain layer. Handler chỉ validate format (is number?). Domain validates business (is price reasonable?).</details>

---

## 34.2 — Middleware & Error Handling

```typescript
// filename: src/backend/middleware.ts
import assert from "node:assert/strict";

// Middleware = function that wraps handler
// Middleware pattern: (handler) => (request) => response
// Composition: middleware3(middleware2(middleware1(handler)))

// === Simulated middleware chain ===
type Request = {
    readonly headers: Record<string, string>;
    readonly path: string;
    readonly body: unknown;
};

type Response = {
    readonly status: number;
    readonly body: unknown;
};

type Handler = (req: Request) => Promise<Response>;
type Middleware = (next: Handler) => Handler;

// Logging middleware
const withLogging: Middleware = (next) => async (req) => {
    const start = Date.now();
    const res = await next(req);
    const duration = Date.now() - start;
    // In production: console.log(`${req.path} → ${res.status} (${duration}ms)`);
    return res;
};

// Auth middleware
const withAuth: Middleware = (next) => async (req) => {
    const token = req.headers["authorization"];
    if (!token || !token.startsWith("Bearer ")) {
        return { status: 401, body: { error: { code: "UNAUTHORIZED", message: "Missing token" } } };
    }
    return next(req);
};

// Error handling middleware
const withErrorHandling: Middleware = (next) => async (req) => {
    try {
        return await next(req);
    } catch (e) {
        return {
            status: 500,
            body: { error: { code: "INTERNAL_ERROR", message: "An unexpected error occurred" } },
        };
    }
};

// Compose middlewares
const compose = (...middlewares: Middleware[]): Middleware =>
    middlewares.reduceRight((composed, mw) => (handler) => mw(composed(handler)));

// Handler
const getUser: Handler = async (req) => ({
    status: 200,
    body: { id: "U1", name: "An" },
});

// Compose: errors → logging → auth → handler
const pipeline = compose(withErrorHandling, withLogging, withAuth);
const protectedGetUser = pipeline(getUser);

// Tests
const run = async () => {
    // With valid token
    const good = await protectedGetUser({
        headers: { authorization: "Bearer token123" },
        path: "/api/users/U1",
        body: null,
    });
    assert.strictEqual(good.status, 200);

    // Without token
    const noAuth = await protectedGetUser({
        headers: {},
        path: "/api/users/U1",
        body: null,
    });
    assert.strictEqual(noAuth.status, 401);

    console.log("Middleware OK ✅");
};

run();
```

> **💡 Middleware = function composition!** `compose(errorHandling, logging, auth)` = pipeline from Ch12. Each middleware wraps the next. FP patterns IN infrastructure.

---

`compose(withErrorHandling, withLogging, withAuth)` — đây là function composition ở infrastructure level. FP patterns không chỉ cho domain logic — chúng cho mọi layer. Middleware chain = pipeline. Error handling middleware ở outermost = catch-all. Auth ở middle = guard. Logging ở giữa = observe.

Khi bạn thêm middleware mới (rate limiting, CORS, caching), bạn chỉ thêm vào compose chain. Middleware cũ không thay đổi. Đây là Open/Closed Principle — open for extension, closed for modification — đạt được tự nhiên qua function composition.

## ✅ Checkpoint 34.2

> Đến đây bạn phải hiểu:
> 1. **Middleware** = (handler) → handler. Wraps behavior
> 2. **Compose**: `compose(m1, m2, m3)` = m1(m2(m3(handler))). Left = outermost
> 3. **Error middleware**: catch unexpected errors, return consistent format
> 4. **Auth middleware**: check token, short-circuit on failure

---

## 🏋️ Bài tập

**Bài 1** (15 phút): Build complete CRUD

```typescript
// Design a Products API:
// GET /api/products/:id → ProductResponse | 404
// POST /api/products → validate → save → 201
// PUT /api/products/:id → validate → update → 200 | 404
// DELETE /api/products/:id → 204 | 404
// Use handler pipeline pattern from 34.1
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type ProductId = string & { __brand: "ProductId" };
type Product = { id: ProductId; name: string; price: number; stock: number };
type ProductRepo = {
    findById: (id: ProductId) => Promise<Product | null>;
    save: (p: Product) => Promise<void>;
    delete: (id: ProductId) => Promise<boolean>;
};

type ApiResponse = { status: number; body: unknown };

// Handlers
const getProduct = async (repo: ProductRepo, id: string): Promise<ApiResponse> => {
    const product = await repo.findById(id as ProductId);
    return product
        ? { status: 200, body: product }
        : { status: 404, body: { error: { code: "NOT_FOUND" } } };
};

const createProduct = async (repo: ProductRepo, body: unknown, genId: () => ProductId): Promise<ApiResponse> => {
    if (typeof body !== "object" || body === null) {
        return { status: 422, body: { error: { code: "VALIDATION_ERROR" } } };
    }
    const obj = body as Record<string, unknown>;
    if (typeof obj.name !== "string" || typeof obj.price !== "number") {
        return { status: 422, body: { error: { code: "VALIDATION_ERROR" } } };
    }
    const product: Product = {
        id: genId(), name: String(obj.name), price: Number(obj.price), stock: Number(obj.stock ?? 0),
    };
    await repo.save(product);
    return { status: 201, body: product };
};

// Test
const run = async () => {
    const store = new Map<string, Product>();
    const repo: ProductRepo = {
        findById: async (id) => store.get(id) ?? null,
        save: async (p) => { store.set(p.id, p); },
        delete: async (id) => store.delete(id),
    };
    let id = 0;
    const genId = () => `P-${++id}` as ProductId;

    const created = await createProduct(repo, { name: "Laptop", price: 20000000, stock: 5 }, genId);
    assert.strictEqual(created.status, 201);

    const found = await getProduct(repo, "P-1");
    assert.strictEqual(found.status, 200);

    const notFound = await getProduct(repo, "P-999");
    assert.strictEqual(notFound.status, 404);

    console.log("CRUD handlers OK ✅");
};
run();
```

</details>

---

## Tóm tắt

- ✅ **Hono** = lightweight TypeScript API framework. Type-safe, edge-ready.
- ✅ **Handler pipeline**: validate → domain → persist → respond.
- ✅ **Middleware = function composition**: logging, auth, error handling.
- ✅ **compose()**: chain middlewares. FP patterns in infrastructure.
- ✅ **DTOs**: request validation (Zod/manual) + response mapping.

## Tiếp theo

→ Chapter 35: **Frontend — React + FP** — React as pure functions, FP state management, fp-ts in React.
