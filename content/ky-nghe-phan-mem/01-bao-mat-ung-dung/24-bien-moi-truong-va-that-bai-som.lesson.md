---
id: ky-nghe-phan-mem.bao-mat-ung-dung.bien-moi-truong-va-that-bai-som
title: "Biến môi trường & thất bại SỚM — đừng hardcode bí mật"
summary: "Bí mật KHÔNG BAO GIỜ hardcode trong code — đọc từ MÔI TRƯỜNG lúc chạy. requireEnv kiểm NGAY lúc khởi động, THIẾU biến nào thì báo lỗi RÕ RÀNG NGAY (fail-fast), thay vì lỗi undefined mập mờ giữa chừng. Trả Result thay vì throw, nối phong cách track."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 24
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [bmud.secrets-env-fail-fast]
requires: [bmud.security-headers]
concepts: [bmud.secrets-env-fail-fast]
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
Mật khẩu database, khoá API — hardcode TRỰC TIẾP trong code (bài
20's ví dụ `ro_ri_bi_mat`) là SAI. Đọc từ MÔI TRƯỜNG lúc chạy — nếu
THIẾU thì SAO?
::::

::::explain{#require-env}
Bí mật (mật khẩu database, khoá API) KHÔNG BAO GIỜ hardcode — đọc TỪ
**MÔI TRƯỜNG** lúc chạy. `requireEnv` kiểm **NGAY lúc khởi động**,
THIẾU biến nào thì báo lỗi **RÕ RÀNG NGAY** (**fail-fast**) — trả
`Result` (nối phong cách track, KHÔNG `throw`):

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function requireEnv(env: Record<string, string | undefined>, name: string): Result<string, string> {
  const value = env[name];
  if (!value) return loi(`thiếu biến môi trường bắt buộc: ${name}`);
  return ok(value);
}

const env = { DB_PASSWORD: "s3cr3t" };
console.log(JSON.stringify(requireEnv(env, "DB_PASSWORD")));
console.log(JSON.stringify(requireEnv(env, "JWT_SECRET")));
```

```text
{"kind":"ok","giaTri":"s3cr3t"}
{"kind":"loi","loi":"thiếu biến môi trường bắt buộc: JWT_SECRET"}
```

`env: Record<string, string | undefined>` (giả lập `process.env` —
Node global KHÔNG có trong sandbox) — `requireEnv` báo lỗi **NGAY**
với TÊN CHÍNH XÁC biến còn THIẾU, thay vì để `undefined` "trôi" sâu
vào code, gây lỗi MẬP MỜ ("Cannot read property of undefined") Ở MỘT
chỗ HOÀN TOÀN khác, XA nguyên nhân THẬT.
::::

::::example{#kiem-tra-khoi-dong-gom-het-loi}
Ứng dụng THẬT cần **NHIỀU** biến môi trường — kiểm **TẤT CẢ** ngay LÚC
KHỞI ĐỘNG, gom **HẾT** lỗi (nối kỹ thuật đã học), KHÔNG dừng ở lỗi
ĐẦU:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function requireEnv(env: Record<string, string | undefined>, name: string): Result<string, string> {
  const value = env[name];
  if (!value) return loi(`thiếu biến môi trường bắt buộc: ${name}`);
  return ok(value);
}

function kiemTraKhoiDong(env: Record<string, string | undefined>, tenBienCanThiet: string[]): Result<null, string[]> {
  const thieu: string[] = [];
  for (const ten of tenBienCanThiet) {
    const kq = requireEnv(env, ten);
    if (kq.kind === "loi") thieu.push(kq.loi);
  }
  if (thieu.length > 0) return loi(thieu);
  return ok(null);
}

const envThieuCa2 = {};
console.log(JSON.stringify(kiemTraKhoiDong(envThieuCa2, ["DB_PASSWORD", "JWT_SECRET"])));
```

```text title=readonly
{"kind":"loi","loi":["thiếu biến môi trường bắt buộc: DB_PASSWORD","thiếu biến môi trường bắt buộc: JWT_SECRET"]}
```

`kiemTraKhoiDong` gọi `requireEnv` cho **TỪNG** biến CẦN THIẾT, GOM
**HẾT** lỗi tìm được vào MẢNG `thieu` (nối kỹ thuật form validation,
gom-lỗi đã học trong track) — người triển khai ứng dụng thấy NGAY
**TOÀN BỘ** danh sách biến CÒN THIẾU, sửa **MỘT LẦN**, không phải
chạy LẠI ứng dụng nhiều lần chỉ để phát hiện THÊM MỘT biến thiếu MỖI
lần.
::::

::::predict{#doan-bien-rong-cung-la-thieu commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function requireEnv(env: Record<string, string | undefined>, name: string): Result<string, string> {
  const value = env[name];
  if (!value) return loi(`thiếu biến môi trường bắt buộc: ${name}`);
  return ok(value);
}

// Biến TỒN TẠI trong object, NHƯNG là chuỗi RỖNG (đã khai, chưa điền giá trị)
const envRong = { DB_PASSWORD: "" };
console.log(requireEnv(envRong, "DB_PASSWORD").kind);
```

Dòng cuối in ra gì?

:::opt{correct}
`loi`
:::

:::opt
`ok` — vì `"DB_PASSWORD"` LÀ MỘT KHOÁ **CÓ TỒN TẠI** trong `envRong`
(`"DB_PASSWORD" in envRong` là `true`), và `requireEnv` chỉ kiểm
"BIẾN CÓ ĐƯỢC KHAI hay không", không quan tâm GIÁ TRỊ cụ thể là gì
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"DB_PASSWORD"` LÀ một khoá **CÓ TỒN
TẠI** trong `envRong` (khác với KHÔNG tồn tại KHOÁ nào cả) — một
quan sát ĐÚNG về CẤU TRÚC object.

Chỗ lệch: `requireEnv` KHÔNG kiểm "khoá có tồn tại" (`in`/`hasOwn
Property`) — nó đọc `env[name]` (GIÁ TRỊ, có thể LÀ `undefined` HOẶC
chuỗi RỖNG HOẶC bất kỳ chuỗi nào), rồi kiểm `if (!value)`. Toán tử
`!` áp dụng lên MỘT `string`: `!""` (chuỗi RỖNG) là **`true`** (chuỗi
rỗng LÀ giá trị "falsy" trong JavaScript, GIỐNG `undefined`/`null`/`0`)
— điều kiện `!value` ĐÚNG, hàm trả `loi(...)` NGAY. Một biến MÔI
TRƯỜNG được khai NHƯNG ĐỂ RỖNG (`DB_PASSWORD=` trong file `.env`,
KHÔNG có giá trị SAU dấu `=`) bị coi LÀ "THIẾU" — hợp lý về mặt
NGHIỆP VỤ (mật khẩu RỖNG hầu như CHẮC CHẮN không phải Ý ĐỊNH thật).
::
:::

:::opt
Máy báo lỗi biên dịch — `envRong` khai `DB_PASSWORD: ""` (chuỗi
RỖNG) không hợp lệ với kiểu tham số `env: Record<string, string |
undefined>`, vì chuỗi RỖNG không thuộc `string | undefined`
::why
Gần đúng ở việc bạn để ý CHUỖI RỖNG (`""`) có vẻ "ĐẶC BIỆT" — MỘT
trực giác dễ hiểu, vì chuỗi rỗng THƯỜNG được coi LÀ trường hợp biên
CẦN chú ý.

Chỗ lệch: `""` (chuỗi rỗng) **LÀ MỘT `string` HOÀN TOÀN HỢP LỆ** —
không có gì "đặc biệt" về mặt KIỂU khiến nó KHÔNG thuộc `string`
(`string | undefined` CHẤP NHẬN MỌI chuỗi, kể cả rỗng). Biên dịch
sạch — sự "thất bại" xảy ra Ở TẦNG **LOGIC** (`!value`, LÚC CHẠY),
không phải Ở TẦNG kiểu.
::
:::
::::

::::code{#viet_requireenv_va_kiemtrakhoidong}
Tự viết `requireEnv` và phần GOM lỗi trong `kiemTraKhoiDong`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function requireEnv(env: Record<string, string | undefined>, name: string): Result<string, string> {
  const value = env[name];
  if (!value) return ___;
  return ___;
}

function kiemTraKhoiDong(env: Record<string, string | undefined>, tenBienCanThiet: string[]): Result<null, string[]> {
  const thieu: string[] = [];
  for (const ten of tenBienCanThiet) {
    const kq = requireEnv(env, ten);
    if (kq.kind === "loi") thieu.push(kq.loi);
  }
  if (thieu.length > 0) return loi(thieu);
  return ok(null);
}

console.log(JSON.stringify(kiemTraKhoiDong({ DB_PASSWORD: "x" }, ["DB_PASSWORD"])));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function requireEnv(env: Record<string, string | undefined>, name: string): Result<string, string> {
  const value = env[name];
  if (!value) return loi(`thiếu biến môi trường bắt buộc: ${name}`);
  return ok(value);
}

function kiemTraKhoiDong(env: Record<string, string | undefined>, tenBienCanThiet: string[]): Result<null, string[]> {
  const thieu: string[] = [];
  for (const ten of tenBienCanThiet) {
    const kq = requireEnv(env, ten);
    if (kq.kind === "loi") thieu.push(kq.loi);
  }
  if (thieu.length > 0) return loi(thieu);
  return ok(null);
}

console.log(JSON.stringify(kiemTraKhoiDong({ DB_PASSWORD: "x" }, ["DB_PASSWORD"])));
```

```typescript title=test
if (requireEnv({}, "X").kind !== "loi") throw new Error("biến thiếu phải ra loi");
if (requireEnv({ X: "gia-tri" }, "X").kind !== "ok") throw new Error("biến có mặt phải ra ok");
const kqOk = requireEnv({ X: "gia-tri" }, "X");
if (kqOk.kind === "ok" && kqOk.giaTri !== "gia-tri") throw new Error("giá trị ok phải đúng giá trị biến môi trường");

const kqThieuJwt = requireEnv({}, "JWT_SECRET");
if (kqThieuJwt.kind === "loi" && !kqThieuJwt.loi.includes("JWT_SECRET")) throw new Error("thông điệp lỗi phải nhắc ĐÚNG TÊN biến bị thiếu (JWT_SECRET), không phải giá trị (undefined) của nó");
const kqThieuDb = requireEnv({}, "DB_HOST");
if (kqThieuDb.kind === "loi" && !kqThieuDb.loi.includes("DB_HOST")) throw new Error("thông điệp lỗi phải nhắc ĐÚNG TÊN biến bị thiếu (DB_HOST) — hai tên khác nhau phải ra hai thông điệp khác nhau");
if (kqThieuJwt.kind === "loi" && kqThieuDb.kind === "loi" && kqThieuJwt.loi === kqThieuDb.loi) throw new Error("hai biến khác nhau phải cho hai thông điệp lỗi KHÁC NHAU, không được giống hệt nhau");

const kqKhoiDongDu = kiemTraKhoiDong({ A: "1", B: "2" }, ["A", "B"]);
if (kqKhoiDongDu.kind !== "ok") throw new Error("đủ mọi biến phải ra ok");

const kqKhoiDongThieu = kiemTraKhoiDong({ A: "1" }, ["A", "B", "C"]);
if (kqKhoiDongThieu.kind !== "loi") throw new Error("thiếu biến phải ra loi");
if (kqKhoiDongThieu.kind === "loi" && kqKhoiDongThieu.loi.length !== 2) throw new Error("phải gom ĐỦ HAI lỗi (B và C thiếu), không dừng ở lỗi đầu");
```

:::hints
- kind: attention
  body: "requireEnv: nhánh thiếu bọc loi(...) kèm thông điệp NHẮC RÕ tên biến; nhánh có giá trị bọc ok(value)."
- kind: strategy
  body: 'loi(`thiếu biến môi trường bắt buộc: ${name}`) : ok(value) — hai nhánh của requireEnv.'
- kind: one-line
  body: '___ (thiếu) = loi(`thiếu biến môi trường bắt buộc: ${name}`)\n___ (có giá trị) = ok(value)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "ok"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Fail-fast: biến môi trường thiếu, phát hiện NGAY lúc khởi động, gom
hết lỗi một lần. Bước tiếp theo: nếu bí mật LỠ vào Git, xoá commit
sau có đủ không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn LỠ commit một khoá API vào Git, rồi commit "xoá" nó Ở LẦN SAU.
Khoá đó CÒN AN TOÀN không?
::::

::::checkpoint{mastery=0.8}
::::
