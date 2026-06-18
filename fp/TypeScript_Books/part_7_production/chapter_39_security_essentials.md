# Chapter 39 — Security Essentials

> **Bạn sẽ học được**:
> - Authentication vs Authorization — ai bạn? bạn được làm gì?
> - Password hashing — bcrypt/argon2 — KHÔNG BAO GIỜ lưu plaintext
> - JWT (JSON Web Tokens) — stateless auth, refresh tokens
> - OAuth 2.0 — "Login with Google/GitHub"
> - RBAC vs ABAC — role-based vs attribute-based access control
> - Security middleware — helmet, cors, rate-limit
>
> **Yêu cầu trước**: Chapter 34 (Backend basics), Chapter 37 (Database).
> **Thời gian đọc**: ~50 phút | **Level**: Principal
> **Kết quả cuối cùng**: Xây authentication + authorization system type-safe — biết tại sao và làm thế nào.

---

Bạn biết sân bay có bao nhiêu lớp kiểm tra không?

Vé máy bay = **authentication** (bạn LÀ AI? chứng minh đi). Boarding pass = **authorization** (bạn ĐƯỢC LÊN chuyến nào?). Hải quan = kiểm tra thêm (bạn mang gì?). Mỗi lớp kiểm tra MỘT thứ khác nhau. Authentication nói "bạn là Nguyễn Văn An". Authorization nói "An được lên VN-123 nhưng KHÔNG được vào phòng VIP".

Trong phần mềm, authentication = xác nhận danh tính (login). Authorization = xác nhận quyền (ai được làm gì). Chương này cover cả hai, từ password hashing (tại sao KHÔNG BAO GIỜ lưu plaintext) đến JWT tokens (cách truyền danh tính giữa client-server) đến RBAC (Role-Based Access Control).

Tại sao chương này trong sách FP? Vì security logic là PURE LOGIC: validate token → extract claims → check permissions → decide access. Pipeline đẹp. Error types rõ: `InvalidToken | Expired | InsufficientPermissions`. FP patterns từ Ch22 (Result, ROP) áp dụng trực tiếp.

---

## Security Essentials — Auth, JWT, RBAC

Authentication (ai bạn?) vs Authorization (được làm gì?). Password hashing (bcrypt/argon2). JWT cho stateless auth. OAuth 2.0 cho third-party login. RBAC/ABAC cho permission control. Security = FP pipeline: validate → extract → decide.


## 39.1 — Password Hashing

### Tại sao KHÔNG LƯU plaintext?

Năm 2012, LinkedIn bị hack — 6.5 TRIỆU passwords lộ. Passwords được hash bằng SHA-1 NHƯNG KHÔNG CÓ SALT. Kết quả: hackers bẻ khóa MỊI hash trong vài giờ dùng rainbow tables (bảng đã tính sẵn hash cho mọi tổ hợp passwords phổ biến). Năm 2016, con số thật được tiết lộ: 117 TRIỆU.

Năm 2013, Adobe bị hack — 153 TRIỆU accounts. Passwords được mã hóa (3DES, không phải hash). Nhưng dùng CHUNG KEY cho tất cả. Cùng password = cùng ciphertext. Hackers nhìn các passwords phổ biến nhất và đoán ngược.

Bài học: **hash** (không phải encrypt), dùng **salt** (mỗi user khác nhau), dùng **slow hash** (bcrypt/argon2, không MD5/SHA). Slow hash = cố tình chậm để brute force tốn thời gian.

```typescript
// filename: src/security/password_hashing.ts
import assert from "node:assert/strict";

// ❌ NEVER: store plaintext passwords
// ❌ NEVER: use MD5/SHA for passwords (too fast → brute-forceable)
// ✅ ALWAYS: use bcrypt or argon2 (intentionally SLOW)

// === How hashing works ===
// hash("secure123") → "$2b$10$X7Y8Z9..." (one-way, irreversible)
// Same password → different hash each time (salt)
// Verify: bcrypt.compare("secure123", storedHash) → true/false

// Simulating (real code uses 'bcryptjs' or 'argon2' packages):
const createSimpleHash = (password: string, salt: string): string =>
    `HASH(${salt}:${password})`;  // NOT real — educational only!

const salt1 = "RANDOM_SALT_1";
const salt2 = "RANDOM_SALT_2";

const hash1 = createSimpleHash("secure123", salt1);
const hash2 = createSimpleHash("secure123", salt2);

// Same password → DIFFERENT hashes (different salt)
assert.notStrictEqual(hash1, hash2);

// Verify: re-hash with SAME salt, compare
assert.strictEqual(createSimpleHash("secure123", salt1), hash1);  // match!
assert.notStrictEqual(createSimpleHash("wrong", salt1), hash1);    // no match

// Real usage:
// import bcrypt from 'bcryptjs';
// const hash = await bcrypt.hash("secure123", 10);  // 10 = cost factor
// const isValid = await bcrypt.compare("secure123", hash);  // true

console.log("Password hashing OK ✅");
```

> **💡 Tham số quan trọng**: `bcrypt.hash(password, 10)` — số 10 là "cost factor". Mỗi +1 = chậm gấp đôi. Cost 10 ≈ 100ms. Cost 12 ≈ 400ms. Chọn sao cho verify mất 200-500ms — đủ chậm để chống brute force, đủ nhanh để user không cáu.

---

## ✅ Checkpoint 39.1

> Đến đây bạn phải hiểu:
> 1. **KHÔNG BAO GIỜ** plaintext. **KHÔNG** MD5/SHA (quá nhanh = brute forceable)
> 2. **bcrypt/argon2**: slow hash + automatic salt
> 3. **Salt**: mỗi user một salt khác. Cùng password → khác hash
>
> **Test nhanh**: Tại sao không dùng SHA-256 cho passwords? Nó có 256-bit security mà!
> <details><summary>Đáp án</summary>SHA-256 là **fast hash** — thiết kế để NHANH. GPU hiện đại tính 10 TỈ SHA-256/giây. bcrypt tính ~1000/giây. Chậm 10 TRIỆU lần = brute force mất 10 TRIỆU lần lâu hơn.</details>

---

## 39.2 — JWT Authentication

JWT giải quyết vấn đề **stateless auth**. Trước JWT (session-based): user login → server tạo session ID → lưu trong database/memory → trả cookie. Mỗi request: server đọc cookie → lookup session → xác thực. Vấn đề: nhiều servers? Session nằm ở server nào? Cần sticky sessions hoặc shared session store (Redis).

JWT: user login → server tạo TOKEN chứa thông tin user (payload) + chữ ký (signature). Client gửi token mỗi request. Server chỉ cần VERIFY chữ ký — không cần lookup database. State nằm TRONG token, không trên server. Scale dễ: thêm bao nhiêu servers cũng được, tất cả đều verify được.

Nhược điểm: không thể "revoke" JWT trước khi hết hạn (vì stateless). Giải pháp: access token ngắn (15 phút) + refresh token dài (7 ngày). Revoke = xóa refresh token từ database.

```typescript
// filename: src/security/jwt_auth.ts
import assert from "node:assert/strict";

// JWT = JSON Web Token
// Structure: header.payload.signature (base64 encoded)
// Self-contained: payload has user info. Server DOESN'T store sessions.

type JwtPayload = {
    sub: string;     // subject (user id)
    role: string;
    iat: number;     // issued at (timestamp)
    exp: number;     // expiration (timestamp)
};

// Simplified JWT (real: use 'jose' library)
const SECRET = "my-secret-key";

const createToken = (userId: string, role: string, ttlSeconds = 3600): string => {
    const payload: JwtPayload = {
        sub: userId,
        role,
        iat: Math.floor(Date.now() / 1000),
        exp: Math.floor(Date.now() / 1000) + ttlSeconds,
    };
    const encoded = Buffer.from(JSON.stringify(payload)).toString("base64url");
    const signature = Buffer.from(`${encoded}:${SECRET}`).toString("base64url");
    return `${encoded}.${signature}`;
};

const verifyToken = (token: string): JwtPayload | null => {
    const [encoded, signature] = token.split(".");
    const expectedSig = Buffer.from(`${encoded}:${SECRET}`).toString("base64url");
    if (signature !== expectedSig) return null;  // tampered!

    const payload = JSON.parse(Buffer.from(encoded, "base64url").toString()) as JwtPayload;
    if (payload.exp < Math.floor(Date.now() / 1000)) return null;  // expired!

    return payload;
};

// === Test ===
const token = createToken("U-1", "admin");
const payload = verifyToken(token);
assert.strictEqual(payload!.sub, "U-1");
assert.strictEqual(payload!.role, "admin");

// Tampered token
assert.strictEqual(verifyToken(token + "TAMPERED"), null);

// === Refresh Token Pattern ===
// Access Token: short-lived (15 min), in memory/header
// Refresh Token: long-lived (7 days), in httpOnly cookie
// Flow:
// 1. Login → server returns { accessToken, refreshToken }
// 2. API requests: Authorization: Bearer <accessToken>
// 3. Access expired → POST /refresh with refreshToken → new accessToken

console.log("JWT auth OK ✅");
```

---

## 39.3 — Authorization: RBAC

### Tách authentication khỏi authorization

Sau khi biết user LÀ AI (authentication), câu hỏi tiếp: user ĐƯỢC LÀM GÌ? RBAC (Role-Based Access Control) là mô hình phổ biến nhất: gán ROLES cho users, gán PERMISSIONS cho roles.

Tại sao FP phù hợp? Vì authorization là PURE FUNCTION: `(userRole, requiredPermission) => allowed | denied`. Không side effects, không database lookup (permissions hardcoded hoặc cached). Dễ test, dễ reason about. Và return type là DU: `{ allowed: true } | { allowed: false, reason: string }` — chính xác pattern từ Ch22 (Railway Oriented Programming).

```typescript
// filename: src/security/rbac.ts
import assert from "node:assert/strict";

// RBAC = Role-Based Access Control
// Users have ROLES → Roles have PERMISSIONS

type Role = "admin" | "editor" | "viewer";
type Permission = "read" | "write" | "delete" | "manage_users";

const rolePermissions: Record<Role, readonly Permission[]> = {
    admin: ["read", "write", "delete", "manage_users"],
    editor: ["read", "write"],
    viewer: ["read"],
};

const hasPermission = (role: Role, permission: Permission): boolean =>
    rolePermissions[role].includes(permission);

// Authorization middleware (pure function!)
type AuthResult = { allowed: true } | { allowed: false; reason: string };

const authorize = (
    userRole: Role,
    requiredPermission: Permission,
): AuthResult =>
    hasPermission(userRole, requiredPermission)
        ? { allowed: true }
        : { allowed: false, reason: `Role '${userRole}' lacks '${requiredPermission}'` };

// Tests
assert.deepStrictEqual(authorize("admin", "delete"), { allowed: true });
assert.deepStrictEqual(authorize("viewer", "delete"), {
    allowed: false, reason: "Role 'viewer' lacks 'delete'",
});
assert.deepStrictEqual(authorize("editor", "write"), { allowed: true });
assert.deepStrictEqual(authorize("editor", "manage_users"), {
    allowed: false, reason: "Role 'editor' lacks 'manage_users'",
});

// Middleware pattern:
// app.delete('/api/posts/:id', authMiddleware, requirePermission('delete'), handler);

console.log("RBAC OK ✅");
```

---

## ✅ Checkpoint 39.2-39.3

> Đến đây bạn phải hiểu:
> 1. **Authn vs Authz**: WHO are you? (authentication) vs WHAT can you do? (authorization)
> 2. **JWT**: stateless, self-contained tokens. Access (15min) + Refresh (7 days)
> 3. **RBAC**: Roles → Permissions. Pure function: `(role, permission) → allowed`
> 4. **FP connection**: Authorization = pipeline. DU return types. Testable without DB
>
> **Test nhanh**: User đã logout nhưng còn access token hợp lệ (chưa hết hạn). Bạn làm gì?
> <details><summary>Đáp án</summary>Access token vẫn hợp lệ cho đến khi hết hạn (JWT stateless). Xóa refresh token từ DB → khi access token hết hạn, user không refresh được. Hoặc: dùng token blacklist (trade-off: stateful).</details>

---

## 39.4 — OAuth 2.0 Overview

"Login with Google" — bạn nhấn nút, được chuyển sang Google, đăng nhập Google, quay lại app đã logged in. Bạn KHÔNG BAO GIỜ cho app password Google của bạn. App nhận được auth code từ Google, đổi lấy access token, dùng token để lấy profile.

Đây là OAuth 2.0 Authorization Code flow với PKCE (Proof Key for Code Exchange). PKCE chống interception: app tạo random string (code_verifier), hash nó (code_challenge), gửi hash cho Google. Khi đổi code lấy token, app gửi ORIGINAL string. Google verify: hash(original) == challenge? Nếu attacker bắt được code nhưng không có verifier → không đổi được token.

```typescript
// filename: src/security/oauth_flow.ts

// OAuth 2.0 Authorization Code + PKCE flow:
//
// 1. User clicks "Login with Google"
// 2. App generates code_verifier (random) + code_challenge (SHA-256 hash)
// 3. Redirect to Google:
//    GET https://accounts.google.com/o/oauth2/auth?
//      client_id=xxx&
//      redirect_uri=https://myapp.com/callback&
//      response_type=code&
//      scope=openid+profile+email&
//      code_challenge=HASH&
//      code_challenge_method=S256
//
// 4. User logs in at Google, grants permission
// 5. Google redirects back:
//    GET https://myapp.com/callback?code=AUTH_CODE
//
// 6. App exchanges code for tokens:
//    POST https://oauth2.googleapis.com/token
//      { code: AUTH_CODE, code_verifier: ORIGINAL, client_id, redirect_uri }
//    Response: { access_token, id_token, refresh_token }
//
// 7. App uses access_token to GET user profile:
//    GET https://www.googleapis.com/userinfo
//    Authorization: Bearer access_token
//    Response: { sub: "123", name: "An", email: "an@gmail.com" }
//
// Libraries:
// - passport.js (Express middleware)
// - arctic (lightweight, works with any framework)
// - @auth/core (Auth.js / NextAuth)

console.log("OAuth flow OK ✅");
```

> **💡 Khi nào dùng OAuth vs tự build auth?**
> - **OAuth**: khi muốn "Login with Google/GitHub", không muốn quản lý passwords
> - **Tự build**: khi cần full control, enterprise, hoặc phải có email/password login
> - **Thường**: Kết hợp cả hai. Tự build email/password + OAuth social login.

---

## 🏋️ Bài tập

**Bài 1** (10 phút): Triển khai ABAC (Attribute-Based).

<details><summary>✅ Lời giải</summary>

```typescript
import assert from "node:assert/strict";

type User = { id: string; role: string; department: string };
type Resource = { ownerId: string; department: string; isPublic: boolean };
type Action = "read" | "write" | "delete";

// ABAC policy: rules based on attributes, not just roles
const canAccess = (user: User, resource: Resource, action: Action): boolean => {
    if (user.role === "admin") return true;
    if (resource.isPublic && action === "read") return true;
    if (resource.ownerId === user.id) return true;  // owner can do anything
    if (user.department === resource.department && action === "read") return true;
    return false;
};

const alice: User = { id: "U1", role: "editor", department: "engineering" };
const doc: Resource = { ownerId: "U2", department: "engineering", isPublic: false };

assert.strictEqual(canAccess(alice, doc, "read"), true);    // same dept
assert.strictEqual(canAccess(alice, doc, "write"), false);  // not owner
assert.strictEqual(canAccess(alice, { ...doc, isPublic: true }, "read"), true);
assert.strictEqual(canAccess(alice, { ...doc, ownerId: "U1" }, "write"), true);  // owner
```

</details>

**Bài 2** (10 phút): Viết auth middleware pipeline

```typescript
// Viết auth pipeline dùng FP patterns:
// 1. extractToken(request) → Result<string, AuthError>
// 2. verifyToken(token) → Result<JwtPayload, AuthError>
// 3. checkPermission(payload, requiredRole) → Result<User, AuthError>
// AuthError = "missing_token" | "invalid_token" | "expired" | "insufficient_permissions"
// Compose: extractToken >> verifyToken >> checkPermission
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

type AuthError = "missing_token" | "invalid_token" | "expired" | "insufficient_permissions";
type Result<T> = { tag: "ok"; value: T } | { tag: "err"; error: AuthError };
type Payload = { sub: string; role: string; exp: number };

const extractToken = (header?: string): Result<string> =>
    !header ? { tag: "err", error: "missing_token" }
    : !header.startsWith("Bearer ") ? { tag: "err", error: "invalid_token" }
    : { tag: "ok", value: header.slice(7) };

const verifyToken = (token: string): Result<Payload> => {
    if (token === "VALID") return { tag: "ok", value: { sub: "U1", role: "admin", exp: Date.now() + 60000 } };
    if (token === "EXPIRED") return { tag: "err", error: "expired" };
    return { tag: "err", error: "invalid_token" };
};

const checkPermission = (payload: Payload, required: string): Result<Payload> =>
    payload.role === required || payload.role === "admin"
        ? { tag: "ok", value: payload }
        : { tag: "err", error: "insufficient_permissions" };

// Pipeline!
const authPipeline = (header: string | undefined, requiredRole: string): Result<Payload> => {
    const tokenR = extractToken(header);
    if (tokenR.tag === "err") return tokenR;
    const payloadR = verifyToken(tokenR.value);
    if (payloadR.tag === "err") return payloadR;
    return checkPermission(payloadR.value, requiredRole);
};

assert.strictEqual(authPipeline("Bearer VALID", "admin").tag, "ok");
assert.deepStrictEqual(authPipeline(undefined, "admin"), { tag: "err", error: "missing_token" });
assert.deepStrictEqual(authPipeline("Bearer EXPIRED", "admin"), { tag: "err", error: "expired" });
console.log("Auth pipeline OK ✅");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Token expired" mỗi 15 phút | Access token TTL quá ngắn | Implement refresh token flow (auto-refresh) |
| User logged out nhưng vẫn access được | JWT stateless — không revoke được | Xóa refresh token + dùng short access TTL |
| Passwords lộ khi database bị hack | Storing plaintext hoặc MD5 | Migrate to bcrypt/argon2 immediately |
| OAuth callback fails | Redirect URI mismatch | Check exact URL match in OAuth provider settings |
| RBAC too rigid | Cần fine-grained control | Switch to ABAC (attribute-based) hoặc kết hợp |

---

## 💬 Đối thoại với bản thân: Security Q&A

**Q: JWT hay Sessions? Cái nào tốt hơn?**

A: Không có "tốt hơn" — chỉ có trade-offs. JWT: stateless, scale dễ, không revoke được (phải chờ hết hạn). Sessions: stateful, revoke ngay, cần shared store (Redis). Dùng BOTH: JWT cho access (15min), sessions/DB cho refresh tokens.

**Q: bcrypt hay argon2?**

A: Argon2 mới hơn, thắng Password Hashing Competition 2015. Memory-hard = chống GPU attacks tốt hơn. Nhưng bcrypt "đủ tốt" và supported rộng hơn. Chọn cái nào cũng WINS so với MD5/SHA/plaintext.

**Q: OAuth phức tạp quá. Cần học không?**

A: Nếu app cần "Login with Google" — bạn PHẢI hiểu flow. Nhưng không cần implement từ đầu: dùng libraries (arctic, passport.js, Auth.js). Hiểu flow = debug được khi breaks.

---

## Tóm tắt

Bảo mật không phải là thứ thêm vào SAU — nó phải ĐƯỢC THIẾT KẾ TỪ ĐẦU. Chương này đã đề cập đến các nền tảng: hash mật khẩu đúng cách, JWT cho xác thực phi trạng thái, OAuth cho đăng nhập bên thứ ba, RBAC/ABAC cho ủy quyền. Tất cả đều phù hợp tốt với các mẫu FP: validate → extract → decide = pipeline.

- ✅ **Xác thực (Authentication) vs Ủy quyền (Authorization)**: BẠN LÀ AI? → BẠN ĐƯỢC LÀM GÌ?
- ✅ **Hash mật khẩu**: bcrypt/argon2. KHÔNG BAO GIỜ lưu plaintext.
- ✅ **JWT**: token phi trạng thái. Thư viện `jose`. Xoay vòng refresh token.
- ✅ **OAuth 2.0**: luồng PKCE. Passport.js / arctic.
- ✅ **RBAC/ABAC**: middleware guards. TypeScript union types.

## Tiếp theo

→ Chapter 40: **Application Security & Hardening** — OWASP Top 10, SQL injection, XSS, CSRF, rate limiting, security headers.
