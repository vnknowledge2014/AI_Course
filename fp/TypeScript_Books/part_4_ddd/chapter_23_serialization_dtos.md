# Chapter 23 — Serialization & DTOs

> **Bạn sẽ học được**:
> - Tại sao Domain types ≠ API/DB types
> - DTO pattern — Data Transfer Objects cho boundaries
> - Zod runtime validation — parse external data an toàn
> - Domain ↔ DTO mapping — chuyển đổi hai chiều
> - API contracts — type-safe request/response
> - Versioning DTOs — backward compatibility
>
> **Yêu cầu trước**: Chapter 14 (smart constructors, Zod), Chapter 19 (architecture layers), Chapter 20 (domain modeling).
> **Thời gian đọc**: ~45 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Boundaries an toàn — external data LUÔN được validate trước khi vào domain.

---

Bạn biết trạm hải quan tại biên giới quốc gia?

Bên trong nước (domain), mọi người nói cùng ngôn ngữ, dùng cùng tiền, tuân cùng luật. Nhưng hàng hóa từ nước ngoài (external data — JSON, API response, DB row) = đóng gói theo tiêu chuẩn khác, ghi nhãn bằng ngôn ngữ khác. Trạm hải quan **kiểm tra**, **chuyển đổi**, **gắn nhãn lại** trước khi cho vào nội địa. Chiều ngược lại cũng vậy: hàng xuất khẩu (domain → response) phải đóng gói theo format quốc tế.

**Serialization & DTOs** chính là hải quan cho data: DTO (Data Transfer Object) = format "quốc tế" cho boundaries. Domain types = format "nội địa" cho business logic. Mapping functions = nhân viên hải quan chuyển đổi qua lại. Zod schemas = máy quét kiểm tra hàng hóa tại cửa khẩu.

---

## 23.1 — Domain Types ≠ External Types

### Tại sao KHÔNG dùng chung một type?

Lập trình viên mới thường dùng 1 type cho tất cả: database row = API response = domain object. Tiện, nhưng TAI HẠI:

```typescript
// filename: src/shared_type_problem.ts

// ❌ CHUNG 1 type cho mọi thứ
type User = {
    id: number;              // DB: auto-increment
    name: string;
    email: string;
    password_hash: string;   // DB: sensitive! NEVER in API response!
    created_at: string;      // DB: ISO string. Domain: Date. API: formatted string
    is_active: boolean;      // DB: 0/1. Domain: boolean. API: "active"/"inactive"
    role_id: number;         // DB: foreign key. Domain: Role enum. API: role name
};

// Vấn đề:
// 1. API response LỘ password_hash → security breach!
// 2. DB column names (snake_case) LEAK vào domain (camelCase)
// 3. DB thay schema → domain phải sửa → API response thay đổi → frontend break
// 4. Validation logic không rõ ở đâu — ai validate email format?
```

### Giải pháp: types riêng cho từng layer

```typescript
// filename: src/separate_types.ts
import assert from "node:assert/strict";

// === Domain types (pure business logic) ===
type UserId = string & { readonly __brand: "UserId" };
type Email = string & { readonly __brand: "Email" };

type UserRole = "admin" | "editor" | "viewer";

type DomainUser = {
    readonly id: UserId;
    readonly name: string;
    readonly email: Email;
    readonly role: UserRole;
    readonly isActive: boolean;
    readonly createdAt: Date;
};

// === DB types (persistence layer) ===
type DbUserRow = {
    readonly id: number;
    readonly name: string;
    readonly email: string;
    readonly password_hash: string;
    readonly role_id: number;
    readonly is_active: number;  // SQLite: 0/1
    readonly created_at: string; // ISO string
};

// === API types (presentation layer) ===
type ApiUserResponse = {
    readonly id: string;
    readonly name: string;
    readonly email: string;
    readonly role: string;
    readonly status: "active" | "inactive";
    readonly memberSince: string;  // "June 2024"
};

// Mapping: DB → Domain
const ROLE_MAP: Record<number, UserRole> = { 1: "admin", 2: "editor", 3: "viewer" };

const dbToDomain = (row: DbUserRow): DomainUser => ({
    id: String(row.id) as UserId,
    name: row.name,
    email: row.email as Email,
    role: ROLE_MAP[row.role_id] ?? "viewer",
    isActive: row.is_active === 1,
    createdAt: new Date(row.created_at),
});

// Mapping: Domain → API
const domainToApi = (user: DomainUser): ApiUserResponse => ({
    id: user.id,
    name: user.name,
    email: user.email,
    role: user.role,
    status: user.isActive ? "active" : "inactive",
    memberSince: user.createdAt.toLocaleDateString("en-US", { month: "long", year: "numeric" }),
});

// Test
const dbRow: DbUserRow = {
    id: 1,
    name: "An",
    email: "an@mail.com",
    password_hash: "bcrypt$2b$10$...",
    role_id: 1,
    is_active: 1,
    created_at: "2024-06-15T10:00:00Z",
};

const domainUser = dbToDomain(dbRow);
assert.strictEqual(domainUser.role, "admin");
assert.strictEqual(domainUser.isActive, true);
assert.ok(domainUser.createdAt instanceof Date);

const apiResponse = domainToApi(domainUser);
assert.strictEqual(apiResponse.status, "active");
assert.strictEqual(apiResponse.role, "admin");
// password_hash KHÔNG có trong apiResponse! ✅

console.log("Separate types OK ✅");
```

> **💡 Layer isolation**: DB schema thay đổi → chỉ sửa `dbToDomain`. API format thay đổi → chỉ sửa `domainToApi`. Domain logic KHÔNG BỊ ẢNH HƯỞNG. Đây là sức mạnh của boundaries.

---

## ✅ Checkpoint 23.1

> Đến đây bạn phải hiểu:
> 1. **Domain types ≠ DB types ≠ API types** — tách layer, tách responsibility
> 2. **password_hash** KHÔNG ĐƯỢC leak vào API response
> 3. **Mapping functions**: `dbToDomain`, `domainToApi` = hải quan chuyển đổi
> 4. **Schema changes isolated**: DB thay → chỉ sửa mapper, domain bất biến
>
> **Test nhanh**: DB thêm column `phone_number` → cần sửa gì?
> <details><summary>Đáp án</summary>1. Thêm `phone_number` vào `DbUserRow`. 2. Nếu domain cần → thêm vào `DomainUser` + update `dbToDomain`. 3. Nếu API cần → thêm vào `ApiUserResponse` + update `domainToApi`. Mỗi layer quyết định RIÊNG.</details>

---

## 23.2 — DTO Pattern

### Data Transfer Objects = postal packages

DTO = "bưu kiện" cho data đi qua boundary. Bên trong bưu kiện: data format chuẩn (JSON-compatible). Không có methods, không có business logic, không có branded types. Chỉ plain data — serializable.

```typescript
// filename: src/dto_pattern.ts
import assert from "node:assert/strict";

// === DTOs = plain, serializable objects ===

// Request DTO (incoming — from API client)
type CreateOrderDto = {
    readonly customerId: string;
    readonly items: readonly {
        readonly productId: string;
        readonly quantity: number;
    }[];
    readonly couponCode?: string;
};

// Response DTO (outgoing — to API client)
type OrderResponseDto = {
    readonly orderId: string;
    readonly status: string;
    readonly items: readonly {
        readonly productName: string;
        readonly quantity: number;
        readonly unitPrice: number;
        readonly lineTotal: number;
    }[];
    readonly subtotal: number;
    readonly discount: number;
    readonly tax: number;
    readonly total: number;
    readonly createdAt: string;  // ISO string — JSON doesn't have Date
};

// === Domain types (rich, typed) ===
type OrderId = string & { readonly __brand: "OrderId" };
type Money = number & { readonly __brand: "Money" };
const Money = (n: number): Money => n as Money;

type OrderStatus = "draft" | "confirmed" | "shipped" | "delivered" | "cancelled";

type DomainOrder = {
    readonly id: OrderId;
    readonly customerId: string;
    readonly status: OrderStatus;
    readonly items: readonly {
        readonly productName: string;
        readonly quantity: number;
        readonly unitPrice: Money;
        readonly lineTotal: Money;
    }[];
    readonly subtotal: Money;
    readonly discount: Money;
    readonly tax: Money;
    readonly total: Money;
    readonly createdAt: Date;
};

// === Mappers ===

// Domain → Response DTO
const toOrderResponseDto = (order: DomainOrder): OrderResponseDto => ({
    orderId: order.id,
    status: order.status,
    items: order.items.map(item => ({
        productName: item.productName,
        quantity: item.quantity,
        unitPrice: item.unitPrice,
        lineTotal: item.lineTotal,
    })),
    subtotal: order.subtotal,
    discount: order.discount,
    tax: order.tax,
    total: order.total,
    createdAt: order.createdAt.toISOString(),
});

// Test
const order: DomainOrder = {
    id: "ORD-1" as OrderId,
    customerId: "C1",
    status: "confirmed",
    items: [
        { productName: "Laptop", quantity: 1, unitPrice: Money(20000000), lineTotal: Money(20000000) },
    ],
    subtotal: Money(20000000),
    discount: Money(0),
    tax: Money(2000000),
    total: Money(22000000),
    createdAt: new Date("2024-06-15T10:00:00Z"),
};

const dto = toOrderResponseDto(order);
assert.strictEqual(dto.orderId, "ORD-1");
assert.strictEqual(dto.createdAt, "2024-06-15T10:00:00.000Z");  // ISO string!
assert.strictEqual(typeof dto.total, "number");  // no branded types in DTO

// DTO is JSON-serializable
const json = JSON.stringify(dto);
const parsed = JSON.parse(json);
assert.strictEqual(parsed.orderId, "ORD-1");

console.log("DTO pattern OK ✅");
```

### DTO rules

| Rule | Domain Type | DTO |
|------|------------|-----|
| **Brand** | `OrderId`, `Money` | `string`, `number` |
| **Date** | `Date` object | ISO string |
| **Enum** | Union literal | String |
| **Methods** | Business logic | NONE — plain data |
| **Validation** | Smart constructors | Zod schemas (next section) |
| **Immutable** | `readonly` | `readonly` |

> **💡 DTO = dumb data container**: No logic, no brands, no Date objects. JSON-compatible. Browser, mobile app, microservice — everyone speaks JSON. DTO = lingua franca.

---

## ✅ Checkpoint 23.2

> Đến đây bạn phải hiểu:
> 1. **DTO** = plain, JSON-serializable object. No business logic
> 2. **Domain → DTO**: strip brands, Date → ISO string, rich → plain
> 3. **DTO → Domain**: parse + validate + construct (next sections)
> 4. **JSON.stringify(dto)** always works. `JSON.stringify(domainObj)` may not (Date, branded types)
>
> **Test nhanh**: DTO nên có method `calculateTotal()` không?
> <details><summary>Đáp án</summary>**KHÔNG!** DTO = dumb data container. Chỉ fields, không methods. Business logic ở domain layer. DTO chỉ transfer data qua boundary.</details>

---

## 23.3 — Zod Runtime Validation

### Máy quét hải quan — validate tại biên giới

TypeScript types biến mất ở runtime — `JSON.parse()` trả `any`. Data từ API request, database, file = **untrusted**. Zod = "máy quét" tại biên giới: parse external data → typed & validated, hoặc reject with detailed errors.

```typescript
// filename: src/zod_validation.ts
// import { z } from "zod";
// (Ví dụ mô phỏng Zod API)

import assert from "node:assert/strict";

// Simulating Zod-style schemas
type ZodResult<T> =
    | { readonly success: true; readonly data: T }
    | { readonly success: false; readonly error: readonly { path: string; message: string }[] };

// Schema builders (simplified Zod API)
const string = () => ({
    min: (n: number) => ({
        max: (m: number) => ({
            parse: (input: unknown): ZodResult<string> => {
                if (typeof input !== "string") return { success: false, error: [{ path: "", message: "Expected string" }] };
                if (input.length < n) return { success: false, error: [{ path: "", message: `Min ${n} chars` }] };
                if (input.length > m) return { success: false, error: [{ path: "", message: `Max ${m} chars` }] };
                return { success: true, data: input };
            }
        }),
        email: () => ({
            parse: (input: unknown): ZodResult<string> => {
                if (typeof input !== "string") return { success: false, error: [{ path: "", message: "Expected string" }] };
                if (input.length < n) return { success: false, error: [{ path: "", message: `Min ${n} chars` }] };
                if (!input.includes("@")) return { success: false, error: [{ path: "", message: "Invalid email" }] };
                return { success: true, data: input };
            }
        }),
        parse: (input: unknown): ZodResult<string> => {
            if (typeof input !== "string") return { success: false, error: [{ path: "", message: "Expected string" }] };
            if (input.length < n) return { success: false, error: [{ path: "", message: `Min ${n} chars` }] };
            return { success: true, data: input };
        }
    }),
    parse: (input: unknown): ZodResult<string> => {
        if (typeof input !== "string") return { success: false, error: [{ path: "", message: "Expected string" }] };
        return { success: true, data: input };
    }
});

const number = () => ({
    positive: () => ({
        int: () => ({
            parse: (input: unknown): ZodResult<number> => {
                if (typeof input !== "number") return { success: false, error: [{ path: "", message: "Expected number" }] };
                if (input <= 0) return { success: false, error: [{ path: "", message: "Must be positive" }] };
                if (!Number.isInteger(input)) return { success: false, error: [{ path: "", message: "Must be integer" }] };
                return { success: true, data: input };
            }
        }),
        parse: (input: unknown): ZodResult<number> => {
            if (typeof input !== "number") return { success: false, error: [{ path: "", message: "Expected number" }] };
            if (input <= 0) return { success: false, error: [{ path: "", message: "Must be positive" }] };
            return { success: true, data: input };
        }
    }),
});

// Real Zod example (conceptual):
// const CreateOrderSchema = z.object({
//     customerId: z.string().min(1),
//     items: z.array(z.object({
//         productId: z.string().min(1),
//         quantity: z.number().positive().int(),
//     })).min(1),
//     couponCode: z.string().optional(),
// });
// type CreateOrderDto = z.infer<typeof CreateOrderSchema>;

// Manual implementation showing the CONCEPT
type CreateOrderDto = {
    readonly customerId: string;
    readonly items: readonly {
        readonly productId: string;
        readonly quantity: number;
    }[];
    readonly couponCode?: string;
};

const parseCreateOrder = (input: unknown): ZodResult<CreateOrderDto> => {
    if (typeof input !== "object" || input === null) {
        return { success: false, error: [{ path: "", message: "Expected object" }] };
    }
    const obj = input as Record<string, unknown>;
    const errors: { path: string; message: string }[] = [];

    // customerId
    if (typeof obj.customerId !== "string" || obj.customerId.length === 0) {
        errors.push({ path: "customerId", message: "Required, non-empty string" });
    }

    // items
    if (!Array.isArray(obj.items) || obj.items.length === 0) {
        errors.push({ path: "items", message: "Required, non-empty array" });
    } else {
        obj.items.forEach((item: unknown, i: number) => {
            if (typeof item !== "object" || item === null) {
                errors.push({ path: `items[${i}]`, message: "Expected object" });
                return;
            }
            const it = item as Record<string, unknown>;
            if (typeof it.productId !== "string") errors.push({ path: `items[${i}].productId`, message: "Required string" });
            if (typeof it.quantity !== "number" || it.quantity <= 0) errors.push({ path: `items[${i}].quantity`, message: "Positive number" });
        });
    }

    if (errors.length > 0) return { success: false, error: errors };

    return {
        success: true,
        data: {
            customerId: obj.customerId as string,
            items: (obj.items as any[]).map(i => ({ productId: i.productId, quantity: i.quantity })),
            couponCode: typeof obj.couponCode === "string" ? obj.couponCode : undefined,
        },
    };
};

// Test: valid input
const validResult = parseCreateOrder({
    customerId: "C1",
    items: [{ productId: "P1", quantity: 2 }],
});
assert.strictEqual(validResult.success, true);

// Test: invalid input — accumulates errors
const invalidResult = parseCreateOrder({
    customerId: "",
    items: [{ productId: 123, quantity: -1 }],
});
assert.strictEqual(invalidResult.success, false);
if (!invalidResult.success) {
    assert.ok(invalidResult.error.length >= 2);  // multiple errors!
}

// Test: completely wrong type
const wrongType = parseCreateOrder("not an object");
assert.strictEqual(wrongType.success, false);

console.log("Zod validation OK ✅");
```

> **💡 "Parse, don't validate" (Ch14 revisited)**: Zod schema = parser. `unknown` → `CreateOrderDto | Error`. Parser PROVES data is valid — downstream code gets typed value. No defensive checks needed.

---

## ✅ Checkpoint 23.3

> Đến đây bạn phải hiểu:
> 1. **Runtime validation** cần thiết vì TypeScript types biến mất ở runtime
> 2. **Zod**: `z.object({ ... }).parse(input)` = parse OR throw. `.safeParse()` = parse → Result
> 3. **Error accumulation**: Zod collects ALL validation errors
> 4. **`z.infer<typeof Schema>`** = derive TypeScript type from schema. Single source of truth
>
> **Test nhanh**: `JSON.parse(body)` có type safety không?
> <details><summary>Đáp án</summary>**KHÔNG!** `JSON.parse` return `any`. TypeScript coi nó là bất kỳ type nào — no runtime check. Cần Zod `.parse()` để validate + type at runtime.</details>

---

## 23.4 — Domain ↔ DTO Mapping

### Nhân viên hải quan: chuyển đổi hai chiều

Request đến: DTO → Domain (import). Response đi: Domain → DTO (export). Mapping function = nhân viên hải quan làm cả hai chiều. Inbound mapping cần validation (Zod), outbound mapping chỉ cần transform (đã valid).

```typescript
// filename: src/domain_dto_mapping.ts
import assert from "node:assert/strict";

// === Domain Types ===
type ProductId = string & { readonly __brand: "ProductId" };
type Money = number & { readonly __brand: "Money" };
const Money = (n: number): Money => n as Money;

type Product = {
    readonly id: ProductId;
    readonly name: string;
    readonly price: Money;
    readonly category: "electronics" | "books" | "clothing";
    readonly isAvailable: boolean;
    readonly createdAt: Date;
};

// === DTOs ===
type CreateProductDto = {
    readonly name: string;
    readonly price: number;
    readonly category: string;
};

type ProductResponseDto = {
    readonly id: string;
    readonly name: string;
    readonly price: number;
    readonly category: string;
    readonly available: boolean;
    readonly createdAt: string;
};

type ProductListResponseDto = {
    readonly products: readonly ProductResponseDto[];
    readonly total: number;
    readonly page: number;
    readonly pageSize: number;
};

// === Result type ===
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// === Inbound: DTO → Domain (with validation) ===
type MappingError =
    | { readonly field: string; readonly message: string };

const VALID_CATEGORIES = new Set(["electronics", "books", "clothing"]);

const dtoToDomainProduct = (
    dto: CreateProductDto,
    generateId: () => ProductId,
    now: Date,
): Result<Product, readonly MappingError[]> => {
    const errors: MappingError[] = [];

    if (dto.name.trim().length < 2) {
        errors.push({ field: "name", message: "Name must be ≥ 2 characters" });
    }
    if (dto.price <= 0) {
        errors.push({ field: "price", message: "Price must be > 0" });
    }
    if (!VALID_CATEGORIES.has(dto.category)) {
        errors.push({ field: "category", message: `Invalid category: ${dto.category}` });
    }

    if (errors.length > 0) return err(errors);

    return ok({
        id: generateId(),
        name: dto.name.trim(),
        price: Money(dto.price),
        category: dto.category as "electronics" | "books" | "clothing",
        isAvailable: true,
        createdAt: now,
    });
};

// === Outbound: Domain → DTO (always succeeds) ===
const domainToProductDto = (product: Product): ProductResponseDto => ({
    id: product.id,
    name: product.name,
    price: product.price,
    category: product.category,
    available: product.isAvailable,
    createdAt: product.createdAt.toISOString(),
});

const domainToProductListDto = (
    products: readonly Product[],
    page: number,
    pageSize: number,
): ProductListResponseDto => ({
    products: products.map(domainToProductDto),
    total: products.length,
    page,
    pageSize,
});

// === Tests ===
const now = new Date("2024-06-15T10:00:00Z");
let idCounter = 0;
const generateId = (): ProductId => `PROD-${++idCounter}` as ProductId;

// Inbound: valid DTO → Domain
const validDto: CreateProductDto = { name: "MacBook Pro", price: 50000000, category: "electronics" };
const domainResult = dtoToDomainProduct(validDto, generateId, now);
assert.strictEqual(domainResult.tag, "ok");
if (domainResult.tag === "ok") {
    assert.strictEqual(domainResult.value.name, "MacBook Pro");
    assert.strictEqual(domainResult.value.price, 50000000);
    assert.ok(domainResult.value.createdAt instanceof Date);
}

// Inbound: invalid DTO → errors
const invalidDto: CreateProductDto = { name: "A", price: -1, category: "food" };
const errorResult = dtoToDomainProduct(invalidDto, generateId, now);
assert.strictEqual(errorResult.tag, "err");
if (errorResult.tag === "err") {
    assert.strictEqual(errorResult.error.length, 3);  // 3 errors!
}

// Outbound: Domain → DTO
if (domainResult.tag === "ok") {
    const responseDto = domainToProductDto(domainResult.value);
    assert.strictEqual(typeof responseDto.createdAt, "string");  // ISO string
    assert.strictEqual(responseDto.available, true);

    // JSON-safe
    const json = JSON.stringify(responseDto);
    assert.ok(json.includes("MacBook Pro"));
}

console.log("Domain ↔ DTO mapping OK ✅");
```

> **💡 Mapping direction matters**: Inbound (DTO→Domain) = VALIDATE + transform. Outbound (Domain→DTO) = just transform (data already valid). Inbound là hải quan nhập khẩu (kiểm tra kỹ). Outbound là xuất khẩu (đóng gói theo format yêu cầu).

---

## ✅ Checkpoint 23.4

> Đến đây bạn phải hiểu:
> 1. **Inbound mapping** (DTO→Domain): validate + construct. Returns `Result<Domain, Error[]>`
> 2. **Outbound mapping** (Domain→DTO): transform only. Always succeeds
> 3. **Branded types stripped** in outbound: `Money → number`, `Date → string`
> 4. **List DTOs** include pagination: `{ products, total, page, pageSize }`
>
> **Test nhanh**: `domainToProductDto` có thể fail không?
> <details><summary>Đáp án</summary>**KHÔNG!** Domain objects đã valid by construction. Outbound mapping chỉ transform format — luôn thành công.</details>

---

## 23.5 — API Contracts

### Hợp đồng giữa client và server

API contract = thỏa thuận "client gửi format này, server trả format kia". Zod schemas = bản hợp đồng tự enforce. Request schema validate inbound. Response schema đảm bảo outbound format.

```typescript
// filename: src/api_contracts.ts
import assert from "node:assert/strict";

// === Request/Response contracts ===

// Contract: POST /api/orders
type CreateOrderRequest = {
    readonly customerId: string;
    readonly items: readonly {
        readonly productId: string;
        readonly quantity: number;
    }[];
    readonly notes?: string;
};

type CreateOrderResponse = {
    readonly orderId: string;
    readonly status: "confirmed";
    readonly total: number;
    readonly estimatedDelivery: string;
};

// Contract: GET /api/orders/:id
type GetOrderResponse = {
    readonly orderId: string;
    readonly status: string;
    readonly items: readonly {
        readonly productName: string;
        readonly quantity: number;
        readonly unitPrice: number;
        readonly lineTotal: number;
    }[];
    readonly total: number;
    readonly createdAt: string;
};

// Contract: Error response (consistent format)
type ApiErrorResponse = {
    readonly error: {
        readonly code: string;
        readonly message: string;
        readonly details?: readonly { readonly field: string; readonly message: string }[];
    };
};

// === Handler with contracts ===
type Result<T, E> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: E };

const ok = <T>(value: T): Result<T, never> => ({ tag: "ok", value });
const err = <E>(error: E): Result<never, E> => ({ tag: "err", error });

// Validate request
const validateCreateOrder = (body: unknown): Result<CreateOrderRequest, ApiErrorResponse> => {
    if (typeof body !== "object" || body === null) {
        return err({
            error: { code: "INVALID_REQUEST", message: "Request body must be an object" },
        });
    }
    const obj = body as Record<string, unknown>;
    const details: { field: string; message: string }[] = [];

    if (typeof obj.customerId !== "string" || obj.customerId.length === 0) {
        details.push({ field: "customerId", message: "Required" });
    }
    if (!Array.isArray(obj.items) || obj.items.length === 0) {
        details.push({ field: "items", message: "At least 1 item required" });
    }

    if (details.length > 0) {
        return err({
            error: { code: "VALIDATION_ERROR", message: "Invalid input", details },
        });
    }

    return ok(body as CreateOrderRequest);
};

// Simulate handler
const handleCreateOrder = (body: unknown): Result<CreateOrderResponse, ApiErrorResponse> => {
    const validated = validateCreateOrder(body);
    if (validated.tag === "err") return validated;

    const request = validated.value;

    // Simulate order creation
    return ok({
        orderId: `ORD-${Date.now()}`,
        status: "confirmed" as const,
        total: request.items.reduce((sum, item) => sum + item.quantity * 100000, 0),
        estimatedDelivery: new Date(Date.now() + 3 * 24 * 60 * 60 * 1000).toISOString().split("T")[0],
    });
};

// Test: valid request
const goodReq = handleCreateOrder({
    customerId: "C1",
    items: [{ productId: "P1", quantity: 2 }],
});
assert.strictEqual(goodReq.tag, "ok");
if (goodReq.tag === "ok") {
    assert.ok(goodReq.value.orderId.startsWith("ORD-"));
    assert.strictEqual(goodReq.value.status, "confirmed");
}

// Test: invalid request → structured error
const badReq = handleCreateOrder({ items: [] });
assert.strictEqual(badReq.tag, "err");
if (badReq.tag === "err") {
    assert.strictEqual(badReq.error.error.code, "VALIDATION_ERROR");
    assert.ok(badReq.error.error.details!.length >= 1);
}

console.log("API contracts OK ✅");
```

> **💡 API contract pattern**: Request schema = "what client MUST send". Response schema = "what server WILL return". Error schema = "what client gets on failure". All typed at compile-time, validated at runtime.

---

## ✅ Checkpoint 23.5

> Đến đây bạn phải hiểu:
> 1. **API contract** = typed request + response + error types
> 2. **Validate inbound** at boundary. Return `Result<Response, ErrorResponse>`
> 3. **Error response** has consistent structure: `{ error: { code, message, details? } }`
> 4. **Handler** = validate → process → respond. No try/catch
>
> **Test nhanh**: Client gửi `{ customerId: 123 }` (number thay vì string) — chuyện gì xảy ra?
> <details><summary>Đáp án</summary>Validation FAIL! `customerId` phải là string. Server trả `{ error: { code: "VALIDATION_ERROR", details: [{ field: "customerId", message: "Required" }] } }`. Client nhận error response rõ ràng.</details>

---

## 23.6 — Versioning DTOs

### Cập nhật hải quan — backward compatibility

API thay đổi theo thời gian. Client cũ gửi DTO v1, client mới gửi DTO v2. Server phải handle CẢ HAI. DTO versioning = hải quan thay đổi quy tắc nhưng vẫn chấp nhận hộ chiếu cũ.

```typescript
// filename: src/dto_versioning.ts
import assert from "node:assert/strict";

// === V1: Original DTO ===
type CreateUserDtoV1 = {
    readonly name: string;
    readonly email: string;
};

// === V2: Added phone (optional for backward compat) ===
type CreateUserDtoV2 = {
    readonly name: string;
    readonly email: string;
    readonly phone?: string;           // NEW — optional for V1 clients
    readonly preferredLanguage?: string; // NEW — optional
};

// === V3: Changed name → firstName + lastName ===
type CreateUserDtoV3 = {
    readonly firstName: string;    // BREAKING CHANGE from "name"
    readonly lastName: string;     // BREAKING CHANGE from "name"
    readonly email: string;
    readonly phone?: string;
    readonly preferredLanguage?: string;
};

// === Domain (internal — doesn't version) ===
type DomainUser = {
    readonly firstName: string;
    readonly lastName: string;
    readonly email: string;
    readonly phone: string | null;
    readonly preferredLanguage: string;
};

// === Version-aware parser ===
type ParseResult<T> =
    | { readonly tag: "ok"; readonly value: T }
    | { readonly tag: "err"; readonly error: string };

const parseCreateUser = (input: unknown): ParseResult<DomainUser> => {
    if (typeof input !== "object" || input === null) {
        return { tag: "err", error: "Expected object" };
    }
    const obj = input as Record<string, unknown>;

    // Detect version by shape
    if ("firstName" in obj && "lastName" in obj) {
        // V3 format
        return {
            tag: "ok",
            value: {
                firstName: String(obj.firstName).trim(),
                lastName: String(obj.lastName).trim(),
                email: String(obj.email),
                phone: typeof obj.phone === "string" ? obj.phone : null,
                preferredLanguage: typeof obj.preferredLanguage === "string" ? obj.preferredLanguage : "vi",
            },
        };
    }

    if ("name" in obj) {
        // V1 or V2 format — split name
        const fullName = String(obj.name).trim();
        const spaceIndex = fullName.indexOf(" ");
        const firstName = spaceIndex > 0 ? fullName.slice(0, spaceIndex) : fullName;
        const lastName = spaceIndex > 0 ? fullName.slice(spaceIndex + 1) : "";

        return {
            tag: "ok",
            value: {
                firstName,
                lastName,
                email: String(obj.email),
                phone: typeof obj.phone === "string" ? obj.phone : null,
                preferredLanguage: typeof obj.preferredLanguage === "string" ? obj.preferredLanguage : "vi",
            },
        };
    }

    return { tag: "err", error: "Missing 'name' or 'firstName'+'lastName'" };
};

// Test V1
const v1 = parseCreateUser({ name: "Nguyễn An", email: "an@mail.com" });
assert.strictEqual(v1.tag, "ok");
if (v1.tag === "ok") {
    assert.strictEqual(v1.value.firstName, "Nguyễn");
    assert.strictEqual(v1.value.lastName, "An");
    assert.strictEqual(v1.value.phone, null);  // default
    assert.strictEqual(v1.value.preferredLanguage, "vi");  // default
}

// Test V2
const v2 = parseCreateUser({
    name: "Trần Bình",
    email: "binh@mail.com",
    phone: "0987654321",
    preferredLanguage: "en",
});
assert.strictEqual(v2.tag, "ok");
if (v2.tag === "ok") {
    assert.strictEqual(v2.value.phone, "0987654321");
    assert.strictEqual(v2.value.preferredLanguage, "en");
}

// Test V3
const v3 = parseCreateUser({
    firstName: "Lê",
    lastName: "Cường",
    email: "cuong@mail.com",
});
assert.strictEqual(v3.tag, "ok");
if (v3.tag === "ok") {
    assert.strictEqual(v3.value.firstName, "Lê");
    assert.strictEqual(v3.value.lastName, "Cường");
}

console.log("DTO versioning OK ✅");
```

### Versioning strategies

| Strategy | Approach | Use case |
|----------|----------|----------|
| **Additive** | Add optional fields | Non-breaking. V1 clients unaffected |
| **Shape detection** | Check fields to detect version | Multi-version support |
| **URL versioning** | `/api/v1/users`, `/api/v2/users` | Breaking changes |
| **Header versioning** | `Accept: application/vnd.api+json;version=2` | Fine-grained control |

> **💡 "Don't break old clients"**: Add optional fields (backward compat). When MUST break → URL version + deprecation period. Server-side: normalize all versions to domain type via version-aware parser.

---

## ✅ Checkpoint 23.6

> Đến đây bạn phải hiểu:
> 1. **DTO versioning**: add optional fields (non-breaking) or version URL (breaking)
> 2. **Shape detection**: check for key fields to detect version
> 3. **Normalize**: all versions → same domain type via parser
> 4. **Domain types DON'T version** — only DTOs version. Domain = stable internal model
>
> **Test nhanh**: API cần thêm `avatarUrl` vào response — breaking change?
> <details><summary>Đáp án</summary>**KHÔNG!** Thêm field vào response = additive. Clients cũ ignore field mới. Chỉ REMOVE hoặc RENAME fields mới là breaking.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Domain ↔ DTO

```typescript
// Cho domain type:
// type BlogPost = { id: PostId; title: string; content: string; author: UserId; publishedAt: Date; tags: string[] }
//
// 1. Viết BlogPostResponseDto (JSON-safe)
// 2. Viết domainToDto mapper
// 3. Viết CreateBlogPostDto + dtoToDomain mapper (with validation)
```

<details><summary>✅ Lời giải Bài 1</summary>

```typescript
import assert from "node:assert/strict";

type PostId = string & { readonly __brand: "PostId" };
type UserId = string & { readonly __brand: "UserId" };

type BlogPost = {
    readonly id: PostId;
    readonly title: string;
    readonly content: string;
    readonly author: UserId;
    readonly publishedAt: Date;
    readonly tags: readonly string[];
};

// Response DTO
type BlogPostResponseDto = {
    readonly id: string;
    readonly title: string;
    readonly content: string;
    readonly authorId: string;
    readonly publishedAt: string;
    readonly tags: readonly string[];
};

// Domain → DTO
const toResponseDto = (post: BlogPost): BlogPostResponseDto => ({
    id: post.id,
    title: post.title,
    content: post.content,
    authorId: post.author,
    publishedAt: post.publishedAt.toISOString(),
    tags: post.tags,
});

// Create DTO → Domain
type CreateBlogPostDto = {
    readonly title: string;
    readonly content: string;
    readonly tags?: readonly string[];
};

type Result<T, E> = { tag: "ok"; value: T } | { tag: "err"; error: E };
const ok = <T>(v: T): Result<T, never> => ({ tag: "ok", value: v });
const err = <E>(e: E): Result<never, E> => ({ tag: "err", error: e });

const fromCreateDto = (
    dto: CreateBlogPostDto,
    authorId: UserId,
    generateId: () => PostId,
    now: Date,
): Result<BlogPost, string> => {
    if (dto.title.trim().length < 5) return err("Title too short (min 5 chars)");
    if (dto.content.trim().length < 10) return err("Content too short (min 10 chars)");

    return ok({
        id: generateId(),
        title: dto.title.trim(),
        content: dto.content.trim(),
        author: authorId,
        publishedAt: now,
        tags: dto.tags ?? [],
    });
};

// Test
const now = new Date("2024-06-15");
const post = fromCreateDto(
    { title: "Hello World", content: "This is my first blog post about FP" },
    "U1" as UserId,
    () => "POST-1" as PostId,
    now,
);
assert.strictEqual(post.tag, "ok");
if (post.tag === "ok") {
    const dto = toResponseDto(post.value);
    assert.strictEqual(typeof dto.publishedAt, "string");
    assert.strictEqual(dto.authorId, "U1");
}

const badPost = fromCreateDto(
    { title: "Hi", content: "Short" },
    "U1" as UserId, () => "POST-2" as PostId, now,
);
assert.strictEqual(badPost.tag, "err");
```

</details>

---

**Bài 2** (10 phút): API error contract

```typescript
// Thiết kế error response contract cho REST API:
// 1. ValidationError: 422 — field-level errors
// 2. NotFoundError: 404 — resource not found
// 3. UnauthorizedError: 401 — missing/invalid token
// 4. InternalError: 500 — generic server error
//
// Viết type ApiError (DU), toHttpResponse mapper
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type ApiError =
    | {
        readonly tag: "validation";
        readonly message: string;
        readonly details: readonly { readonly field: string; readonly message: string }[];
    }
    | { readonly tag: "not_found"; readonly resource: string; readonly id: string }
    | { readonly tag: "unauthorized"; readonly reason: string }
    | { readonly tag: "internal"; readonly message: string };

type HttpResponse = {
    readonly statusCode: number;
    readonly body: {
        readonly error: {
            readonly code: string;
            readonly message: string;
            readonly details?: unknown;
        };
    };
};

const toHttpResponse = (error: ApiError): HttpResponse => {
    switch (error.tag) {
        case "validation":
            return {
                statusCode: 422,
                body: { error: { code: "VALIDATION_ERROR", message: error.message, details: error.details } },
            };
        case "not_found":
            return {
                statusCode: 404,
                body: { error: { code: "NOT_FOUND", message: `${error.resource} '${error.id}' not found` } },
            };
        case "unauthorized":
            return {
                statusCode: 401,
                body: { error: { code: "UNAUTHORIZED", message: error.reason } },
            };
        case "internal":
            return {
                statusCode: 500,
                body: { error: { code: "INTERNAL_ERROR", message: "An unexpected error occurred" } },
                // Note: DON'T expose error.message to client in production!
            };
    }
};

// Test
const validationErr = toHttpResponse({
    tag: "validation",
    message: "Invalid input",
    details: [{ field: "email", message: "Required" }],
});
assert.strictEqual(validationErr.statusCode, 422);

const notFound = toHttpResponse({ tag: "not_found", resource: "Order", id: "ORD-999" });
assert.strictEqual(notFound.statusCode, 404);
assert.ok(notFound.body.error.message.includes("ORD-999"));

const unauthorized = toHttpResponse({ tag: "unauthorized", reason: "Token expired" });
assert.strictEqual(unauthorized.statusCode, 401);
```

</details>

---

**Bài 3** (15 phút): DTO versioning

```typescript
// Viết version-aware parser cho "Product" DTO:
// V1: { name: string, price: number }
// V2: { name: string, price: number, currency: string, sku?: string }
// V3: { name: string, price: { amount: number, currency: string }, sku: string }
//
// Parser detect version, normalize to domain Product type
```

<details><summary>✅ Lời giải Bài 3</summary>

```typescript
import assert from "node:assert/strict";

type DomainProduct = {
    readonly name: string;
    readonly amount: number;
    readonly currency: string;
    readonly sku: string | null;
};

type Result<T> = { tag: "ok"; value: T } | { tag: "err"; error: string };

const parseProduct = (input: unknown): Result<DomainProduct> => {
    if (typeof input !== "object" || input === null) return { tag: "err", error: "Expected object" };
    const obj = input as Record<string, unknown>;

    if (typeof obj.name !== "string") return { tag: "err", error: "Missing name" };

    // V3: price is object { amount, currency }
    if (typeof obj.price === "object" && obj.price !== null) {
        const price = obj.price as Record<string, unknown>;
        if (typeof price.amount !== "number") return { tag: "err", error: "price.amount required" };
        return {
            tag: "ok",
            value: {
                name: obj.name,
                amount: price.amount,
                currency: typeof price.currency === "string" ? price.currency : "VND",
                sku: typeof obj.sku === "string" ? obj.sku : null,
            },
        };
    }

    // V1 or V2: price is number
    if (typeof obj.price === "number") {
        return {
            tag: "ok",
            value: {
                name: obj.name,
                amount: obj.price,
                currency: typeof obj.currency === "string" ? obj.currency : "VND",  // V2 field
                sku: typeof obj.sku === "string" ? obj.sku : null,  // V2 field
            },
        };
    }

    return { tag: "err", error: "Invalid price format" };
};

// V1
const v1 = parseProduct({ name: "Laptop", price: 20000000 });
assert.strictEqual(v1.tag, "ok");
if (v1.tag === "ok") {
    assert.strictEqual(v1.value.currency, "VND");  // default
    assert.strictEqual(v1.value.sku, null);
}

// V2
const v2 = parseProduct({ name: "Mouse", price: 500000, currency: "USD", sku: "MSE-001" });
assert.strictEqual(v2.tag, "ok");
if (v2.tag === "ok") {
    assert.strictEqual(v2.value.currency, "USD");
    assert.strictEqual(v2.value.sku, "MSE-001");
}

// V3
const v3 = parseProduct({ name: "Keyboard", price: { amount: 2000000, currency: "VND" }, sku: "KBD-001" });
assert.strictEqual(v3.tag, "ok");
if (v3.tag === "ok") {
    assert.strictEqual(v3.value.amount, 2000000);
    assert.strictEqual(v3.value.sku, "KBD-001");
}
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Domain types leak to API | Shared types between layers | Separate types. ALWAYS map through DTO |
| `Date` serialization broken | `JSON.stringify(new Date())` = string but `JSON.parse` returns string, not Date | Use ISO strings in DTO. Parse back to Date in mapper |
| Branded types in JSON | `__brand` field appears | `as` casts strip brands. Or use `JSON.parse` (brands = compile-time only) |
| "Too many mapping functions" | Every type needs mapper | Create generic patterns. Or generate with codegen |
| Backward compat breaks silently | Required field added to response | ONLY add optional fields. Required = new version |
| Zod error messages not useful | Default messages | Use `.refine()` with custom messages |

---

## Tóm tắt

Chương này thiết lập trạm hải quan cho data — mọi thứ đi vào domain phải qua kiểm tra, mọi thứ đi ra phải đóng gói đúng format.

- ✅ **Domain ≠ DB ≠ API types**: tách layer, tách responsibility. No password_hash in API response!
- ✅ **DTO** = plain, JSON-serializable. No branded types, no Date objects, no methods.
- ✅ **Zod** = runtime validation. Parse external data → typed value or detailed errors.
- ✅ **Inbound mapping**: DTO → Domain = validate + construct. Returns `Result`.
- ✅ **Outbound mapping**: Domain → DTO = transform only. Always succeeds.
- ✅ **API contracts**: typed request + response + error. Consistent error format.
- ✅ **DTO versioning**: additive (optional fields) or URL versioning. Domain types don't version.

## Tiếp theo

→ Chapter 24: **Persistence & Repository** — Repository pattern, Prisma/Drizzle integration, DI for data access, unit of work, testing with in-memory repositories.
