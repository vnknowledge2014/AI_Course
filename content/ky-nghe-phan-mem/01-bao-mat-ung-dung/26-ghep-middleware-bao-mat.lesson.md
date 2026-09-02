---
id: ky-nghe-phan-mem.bao-mat-ung-dung.ghep-middleware-bao-mat
title: "BOSS — Ghép middleware bảo mật: rate limit + headers + kiểm dữ liệu vào"
summary: "Bài BOSS của track: ghép TẤT CẢ kỹ thuật runtime thành MỘT pipeline middleware ĐỒNG BỘ. composeMiddlewares nối rate-limit, security-headers, body-size-limit, content-type — mỗi middleware CHỈ chịu trách nhiệm MỘT mối lo, ghép được theo THỨ TỰ bất kỳ."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 26
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [bmud.gate-boss]
requires: [bmud.secrets-git-rotate]
concepts: [bmud.gate-boss]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
BÀI CUỐI track. Ghép rate limiting, security headers, giới hạn kích
thước, kiểm Content-Type thành MỘT pipeline như ứng dụng THẬT.
::::

::::explain{#middleware-pipeline}
`Middleware = (req: YeuCau, next: () => PhanHoi) => PhanHoi` —
`composeMiddlewares` nối NHIỀU middleware thành MỘT: mỗi middleware
hoặc tự TRẢ VỀ phản hồi (TỪ CHỐI request) hoặc gọi `next()` (CHUYỂN
TIẾP cho middleware SAU):

```typescript
type YeuCau = { ip: string; gioHienTai: number; kichThuoc: number; contentType: string };
type PhanHoi = { maTrangThai: number; headers: Record<string, string>; noiDung: string };
type Middleware = (req: YeuCau, next: () => PhanHoi) => PhanHoi;

function composeMiddlewares(danhSach: Middleware[]): Middleware {
  return (req, cuoiCung) => {
    function chay(idx: number): PhanHoi {
      if (idx >= danhSach.length) return cuoiCung();
      const mw = danhSach[idx];
      if (!mw) return cuoiCung();
      return mw(req, () => chay(idx + 1));
    }
    return chay(0);
  };
}

type RateLimiter = { check: (ip: string, gioHienTai: number) => { allowed: boolean } };
function taoRateLimitMiddleware(limiter: RateLimiter): Middleware {
  return (req, next) => {
    const kq = limiter.check(req.ip, req.gioHienTai);
    if (!kq.allowed) return { maTrangThai: 429, headers: {}, noiDung: "Too Many Requests" };
    return next();
  };
}

function taoBodySizeLimitMiddleware(maxBytes: number): Middleware {
  return (req, next) => {
    if (req.kichThuoc > maxBytes) return { maTrangThai: 413, headers: {}, noiDung: "Payload Too Large" };
    return next();
  };
}

function taoContentTypeMiddleware(allowedTypes: string[]): Middleware {
  return (req, next) => {
    if (!allowedTypes.includes(req.contentType)) return { maTrangThai: 415, headers: {}, noiDung: "Unsupported Media Type" };
    return next();
  };
}
```

MỖI hàm `taoXxxMiddleware` CHỈ chịu trách nhiệm **MỘT** mối lo (rate
limit, kích thước, Content-Type) — GHÉP được theo **THỨ TỰ BẤT KỲ**
qua `composeMiddlewares`, KHÔNG hàm nào biết GÌ về CÁC hàm KHÁC.
::::

::::example{#ghep-va-chay-toan-bo}
Ghép BỐN middleware, test TOÀN BỘ pipeline bằng request GIẢ LẬP:

```typescript title=readonly
type YeuCau = { ip: string; gioHienTai: number; kichThuoc: number; contentType: string };
type PhanHoi = { maTrangThai: number; headers: Record<string, string>; noiDung: string };
type Middleware = (req: YeuCau, next: () => PhanHoi) => PhanHoi;
function composeMiddlewares(danhSach: Middleware[]): Middleware {
  return (req, cuoiCung) => {
    function chay(idx: number): PhanHoi {
      if (idx >= danhSach.length) return cuoiCung();
      const mw = danhSach[idx];
      if (!mw) return cuoiCung();
      return mw(req, () => chay(idx + 1));
    }
    return chay(0);
  };
}
type RateLimiter = { check: (ip: string, gioHienTai: number) => { allowed: boolean } };
function createRateLimiter(maxRequests: number, windowMs: number): RateLimiter {
  const windows = new Map<string, number[]>();
  return {
    check: (ip, gioHienTai) => {
      const timestamps = (windows.get(ip) ?? []).filter((t) => gioHienTai - t < windowMs);
      if (timestamps.length >= maxRequests) return { allowed: false };
      timestamps.push(gioHienTai);
      windows.set(ip, timestamps);
      return { allowed: true };
    },
  };
}
function taoRateLimitMiddleware(limiter: RateLimiter): Middleware {
  return (req, next) => {
    const kq = limiter.check(req.ip, req.gioHienTai);
    if (!kq.allowed) return { maTrangThai: 429, headers: {}, noiDung: "Too Many Requests" };
    return next();
  };
}
function addSecurityHeaders(headers: Record<string, string>): Record<string, string> {
  return { ...headers, "X-Frame-Options": "DENY" };
}
function taoSecurityHeadersMiddleware(): Middleware {
  return (req, next) => {
    const phanHoi = next();
    return { ...phanHoi, headers: addSecurityHeaders(phanHoi.headers) };
  };
}
function taoBodySizeLimitMiddleware(maxBytes: number): Middleware {
  return (req, next) => {
    if (req.kichThuoc > maxBytes) return { maTrangThai: 413, headers: {}, noiDung: "Payload Too Large" };
    return next();
  };
}
function taoContentTypeMiddleware(allowedTypes: string[]): Middleware {
  return (req, next) => {
    if (!allowedTypes.includes(req.contentType)) return { maTrangThai: 415, headers: {}, noiDung: "Unsupported Media Type" };
    return next();
  };
}

const pipeline = composeMiddlewares([
  taoRateLimitMiddleware(createRateLimiter(2, 1000)),
  taoSecurityHeadersMiddleware(),
  taoBodySizeLimitMiddleware(1000),
  taoContentTypeMiddleware(["application/json"]),
]);
const handler = () => ({ maTrangThai: 200, headers: {}, noiDung: "OK" });

console.log(JSON.stringify(pipeline({ ip: "1.1.1.1", gioHienTai: 0, kichThuoc: 100, contentType: "application/json" }, handler)));
console.log(pipeline({ ip: "3.3.3.3", gioHienTai: 0, kichThuoc: 99999, contentType: "application/json" }, handler).maTrangThai);
console.log(pipeline({ ip: "4.4.4.4", gioHienTai: 0, kichThuoc: 10, contentType: "text/plain" }, handler).maTrangThai);
```

```text title=readonly
{"maTrangThai":200,"headers":{"X-Frame-Options":"DENY"},"noiDung":"OK"}
413
415
```

Request HỢP LỆ đi qua HẾT bốn middleware, tới `handler`, RỒI quay
NGƯỢC LẠI qua `taoSecurityHeadersMiddleware` (thêm header VÀO phản
hồi CUỐI CÙNG). Request body QUÁ LỚN bị chặn NGAY Ở
`taoBodySizeLimitMiddleware` — KHÔNG BAO GIỜ chạm `taoContentType
Middleware` hay `handler`.
::::

::::predict{#doan-thu-tu-anh-huong-header commitOnce}
```typescript
type YeuCau = { ip: string; gioHienTai: number; kichThuoc: number; contentType: string };
type PhanHoi = { maTrangThai: number; headers: Record<string, string>; noiDung: string };
type Middleware = (req: YeuCau, next: () => PhanHoi) => PhanHoi;
function composeMiddlewares(danhSach: Middleware[]): Middleware {
  return (req, cuoiCung) => {
    function chay(idx: number): PhanHoi {
      if (idx >= danhSach.length) return cuoiCung();
      const mw = danhSach[idx];
      if (!mw) return cuoiCung();
      return mw(req, () => chay(idx + 1));
    }
    return chay(0);
  };
}
type RateLimiter = { check: (ip: string, gioHienTai: number) => { allowed: boolean } };
function createRateLimiter(maxRequests: number, windowMs: number): RateLimiter {
  const windows = new Map<string, number[]>();
  return {
    check: (ip, gioHienTai) => {
      const timestamps = (windows.get(ip) ?? []).filter((t) => gioHienTai - t < windowMs);
      if (timestamps.length >= maxRequests) return { allowed: false };
      timestamps.push(gioHienTai);
      windows.set(ip, timestamps);
      return { allowed: true };
    },
  };
}
function taoRateLimitMiddleware(limiter: RateLimiter): Middleware {
  return (req, next) => {
    const kq = limiter.check(req.ip, req.gioHienTai);
    if (!kq.allowed) return { maTrangThai: 429, headers: {}, noiDung: "Too Many Requests" };
    return next();
  };
}
function addSecurityHeaders(headers: Record<string, string>): Record<string, string> {
  return { ...headers, "X-Frame-Options": "DENY" };
}
function taoSecurityHeadersMiddleware(): Middleware {
  return (req, next) => {
    const phanHoi = next();
    return { ...phanHoi, headers: addSecurityHeaders(phanHoi.headers) };
  };
}
const handler = () => ({ maTrangThai: 200, headers: {}, noiDung: "OK" });

// RATE LIMIT đứng TRƯỚC security headers trong mảng
const pipeline = composeMiddlewares([
  taoRateLimitMiddleware(createRateLimiter(1, 1000)),
  taoSecurityHeadersMiddleware(),
]);

const limiter2 = { ip: "5.5.5.5", gioHienTai: 0, kichThuoc: 0, contentType: "x" };
pipeline(limiter2, handler); // request 1/1 -- dùng hết ngưỡng
const kqBiChan = pipeline({ ...limiter2, gioHienTai: 100 }, handler); // request 2 -- bị từ chối
console.log(Object.keys(kqBiChan.headers).length);
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì `taoSecurityHeadersMiddleware` LUÔN thêm header VÀO **MỌI**
phản hồi cuối cùng, KHÔNG PHÂN BIỆT phản hồi đó THÀNH CÔNG hay bị TỪ
CHỐI bởi middleware KHÁC
::why
Gần đúng ở việc bạn nhớ ĐÚNG `taoSecurityHeadersMiddleware` CÓ NHIỆM
VỤ thêm header VÀO phản hồi — quan sát về CHỨC NĂNG của middleware đó
đúng.

Chỗ lệch: `taoRateLimitMiddleware` đứng **TRƯỚC**
`taoSecurityHeadersMiddleware` TRONG MẢNG (`composeMiddlewares([rate
Limit, securityHeaders])`) — theo cách `composeMiddlewares` hoạt động
(bài giải thích Ở TRÊN), middleware ĐẦU chạy TRƯỚC, VÀ nếu nó
**KHÔNG gọi `next()`** (như `taoRateLimitMiddleware` khi TỪ CHỐI —
`return {...429...}` NGAY, KHÔNG gọi `next()`), MỌI middleware SAU
NÓ (`taoSecurityHeadersMiddleware`) **KHÔNG BAO GIỜ CHẠY**. Phản hồi
`429` được TRẢ VỀ THẲNG từ `taoRateLimitMiddleware`, `headers` VẪN
LÀ `{}` (RỖNG, như hàm ĐÃ khai) — `Object.keys(...).length` LÀ `0`.
Đây LÀ lý do THỨ TỰ middleware TRONG mảng QUAN TRỌNG: middleware
"quan sát/BỌC phản hồi" (như security headers) PHẢI đứng **SAU**
middleware "có thể TỪ CHỐI SỚM" để CHẮC CHẮN chạy được trên MỌI
đường đi, hoặc phải TỰ ĐẢM NHIỆM việc thêm header cho CẢ trường hợp
bị từ chối.
::
:::

:::opt
Máy báo lỗi biên dịch — `{ ...limiter2, gioHienTai: 100 }` (spread
một object rồi GHI ĐÈ một field) không hợp lệ khi object GỐC ĐÃ được
dùng làm THAM SỐ cho một lời gọi `pipeline` TRƯỚC ĐÓ
::why
Gần đúng ở việc bạn để ý `limiter2` ĐÃ được dùng LÀM tham số cho
`pipeline(...)` Ở DÒNG TRƯỚC — một quan sát ĐÚNG về THỨ TỰ sử dụng
biến trong code.

Chỗ lệch: KHÔNG có ràng buộc nào trong TypeScript về việc "object đã
được TRUYỀN vào một lời gọi hàm thì KHÔNG được spread/dùng LẠI ở chỗ
KHÁC" — object VẪN NGUYÊN VẸN sau khi truyền vào MỘT hàm (trừ khi
CHÍNH hàm đó MUTATE nó, mà `pipeline` KHÔNG hề làm vậy — mọi middleware
ở đây đều TRẢ object MỚI, không sửa `req`). Spread `{ ...limiter2,
gioHienTai: 100 }` tạo object MỚI, GIỮ mọi field của `limiter2`, CHỈ
GHI ĐÈ `gioHienTai`. Biên dịch sạch.
::
:::
::::

::::code{#viet_composemiddlewares_va_gioihan}
Tự viết phần lõi của `composeMiddlewares`, `taoBodySizeLimitMiddleware`,
`taoContentTypeMiddleware`.

```typescript title=starter
type YeuCau = { ip: string; gioHienTai: number; kichThuoc: number; contentType: string };
type PhanHoi = { maTrangThai: number; headers: Record<string, string>; noiDung: string };
type Middleware = (req: YeuCau, next: () => PhanHoi) => PhanHoi;

function composeMiddlewares(danhSach: Middleware[]): Middleware {
  return (req, cuoiCung) => {
    function chay(idx: number): PhanHoi {
      if (idx >= danhSach.length) return cuoiCung();
      const mw = danhSach[idx];
      if (!mw) return cuoiCung();
      return ___;
    }
    return chay(0);
  };
}

function taoBodySizeLimitMiddleware(maxBytes: number): Middleware {
  return (req, next) => {
    if (___) return { maTrangThai: 413, headers: {}, noiDung: "Payload Too Large" };
    return next();
  };
}

function taoContentTypeMiddleware(allowedTypes: string[]): Middleware {
  return (req, next) => {
    if (___) return { maTrangThai: 415, headers: {}, noiDung: "Unsupported Media Type" };
    return next();
  };
}

const pipeline = composeMiddlewares([
  taoBodySizeLimitMiddleware(1000),
  taoContentTypeMiddleware(["application/json"]),
]);
const handler = () => ({ maTrangThai: 200, headers: {}, noiDung: "OK" });
console.log(pipeline({ ip: "x", gioHienTai: 0, kichThuoc: 10, contentType: "application/json" }, handler).maTrangThai);
```

```typescript title=solution
type YeuCau = { ip: string; gioHienTai: number; kichThuoc: number; contentType: string };
type PhanHoi = { maTrangThai: number; headers: Record<string, string>; noiDung: string };
type Middleware = (req: YeuCau, next: () => PhanHoi) => PhanHoi;

function composeMiddlewares(danhSach: Middleware[]): Middleware {
  return (req, cuoiCung) => {
    function chay(idx: number): PhanHoi {
      if (idx >= danhSach.length) return cuoiCung();
      const mw = danhSach[idx];
      if (!mw) return cuoiCung();
      return mw(req, () => chay(idx + 1));
    }
    return chay(0);
  };
}

function taoBodySizeLimitMiddleware(maxBytes: number): Middleware {
  return (req, next) => {
    if (req.kichThuoc > maxBytes) return { maTrangThai: 413, headers: {}, noiDung: "Payload Too Large" };
    return next();
  };
}

function taoContentTypeMiddleware(allowedTypes: string[]): Middleware {
  return (req, next) => {
    if (!allowedTypes.includes(req.contentType)) return { maTrangThai: 415, headers: {}, noiDung: "Unsupported Media Type" };
    return next();
  };
}

const pipeline = composeMiddlewares([
  taoBodySizeLimitMiddleware(1000),
  taoContentTypeMiddleware(["application/json"]),
]);
const handler = () => ({ maTrangThai: 200, headers: {}, noiDung: "OK" });
console.log(pipeline({ ip: "x", gioHienTai: 0, kichThuoc: 10, contentType: "application/json" }, handler).maTrangThai);
```

```typescript title=test
const handlerTest = () => ({ maTrangThai: 200, headers: {}, noiDung: "OK" });
const pipelineTest = composeMiddlewares([
  taoBodySizeLimitMiddleware(1000),
  taoContentTypeMiddleware(["application/json"]),
]);

const kqHopLe = pipelineTest({ ip: "x", gioHienTai: 0, kichThuoc: 10, contentType: "application/json" }, handlerTest);
if (kqHopLe.maTrangThai !== 200) throw new Error("request hợp lệ phải đi hết pipeline, ra 200");

const kqQuaLon = pipelineTest({ ip: "x", gioHienTai: 0, kichThuoc: 99999, contentType: "application/json" }, handlerTest);
if (kqQuaLon.maTrangThai !== 413) throw new Error("body quá lớn phải bị chặn với 413");

const kqSaiType = pipelineTest({ ip: "x", gioHienTai: 0, kichThuoc: 10, contentType: "text/xml" }, handlerTest);
if (kqSaiType.maTrangThai !== 415) throw new Error("content-type sai phải bị chặn với 415");

const pipelineRong = composeMiddlewares([]);
if (pipelineRong({ ip: "x", gioHienTai: 0, kichThuoc: 0, contentType: "x" }, handlerTest).maTrangThai !== 200) throw new Error("pipeline RỖNG (không middleware nào) phải gọi thẳng handler");
```

:::hints
- kind: attention
  body: "composeMiddlewares: gọi mw hiện tại, truyền cho nó một hàm next() gọi ĐỆ QUY chay(idx+1). taoBodySizeLimitMiddleware: từ chối khi kichThuoc VƯỢT maxBytes. taoContentTypeMiddleware: từ chối khi contentType KHÔNG nằm trong allowedTypes."
- kind: strategy
  body: "mw(req, () => chay(idx + 1)) : req.kichThuoc > maxBytes : !allowedTypes.includes(req.contentType) — ba mảnh ghép cốt lõi."
- kind: one-line
  body: "___ (composeMiddlewares) = mw(req, () => chay(idx + 1))\n___ (bodySize) = req.kichThuoc > maxBytes\n___ (contentType) = !allowedTypes.includes(req.contentType)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "200"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Track T5.6 hoàn tất — 26/26 bài. Bảo mật ứng dụng từ nền tảng (băm
mật khẩu, JWT, RBAC/ABAC) tới phòng thủ tấn công (SQLi, XSS, CSRF)
tới vận hành (rate limiting, headers, secrets, middleware) — mọi
mảnh ghép nối liền thành một pipeline thật, test được toàn bộ.
::::

::::reflect{#nghi-lai}
Track Bảo mật ứng dụng đã xong. Bạn đã đi từ xác thực/phân quyền
(hash, salt, JWT, RBAC/ABAC, OAuth2) qua các lỗ hổng phổ biến (SQL
Injection, XSS, CSRF) tới runtime hardening đầy đủ (rate limiting,
security headers, secrets management, middleware pipeline) — 26 bài,
mỗi cụm chốt bằng một bài capstone code-chấm-điểm-sống.
::::

::::checkpoint{mastery=0.85}
::::
