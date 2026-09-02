---
id: ky-nghe-phan-mem.ddd.versioning-dto
title: "Versioning DTO — thêm field & nhận diện version cũ"
summary: "Bài chốt cụm 6: thêm field MỚI dưới dạng optional để client CŨ vẫn hoạt động (V1{ten,email}→V2 thêm soDienThoai?). Breaking change (V3: ten tách ho+ten) cần parser TỰ NHẬN DIỆN version qua field HIỆN DIỆN, chuẩn hoá TẤT CẢ về CÙNG một domain type. Nguyên tắc: domain type KHÔNG BAO GIỜ version — chỉ DTO version."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 36
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.dto-versioning]
requires: [ddd.api-contract]
concepts: [ddd.dto-versioning]
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
Bài chốt cụm 6. Hợp đồng API (bài trước) KHÔNG đứng yên mãi — thêm
field mới, hay đổi cấu trúc, mà KHÔNG phá vỡ client CŨ, làm sao?
::::

::::explain{#them-field-optional}
Cách AN TOÀN nhất: thêm field MỚI dưới dạng **optional** (`?`) — client
CŨ (chưa cập nhật, KHÔNG gửi field đó) vẫn hoạt động BÌNH THƯỜNG:

```typescript
// V1: hình dạng GỐC
type TaoNguoiDungDtoV1 = { ten: string; email: string };

// V2: thêm soDienThoai OPTIONAL -- client V1 (không gửi field này) VẪN hợp lệ
type TaoNguoiDungDtoV2 = { ten: string; email: string; soDienThoai?: string };

const tuClientCu: TaoNguoiDungDtoV2 = { ten: "An", email: "an@shop.vn" };
const tuClientMoi: TaoNguoiDungDtoV2 = { ten: "Binh", email: "binh@shop.vn", soDienThoai: "0900000000" };
console.log(tuClientCu.soDienThoai);
console.log(tuClientMoi.soDienThoai);
```

```text
undefined
0900000000
```

`soDienThoai?: string` (dấu `?`) nghĩa là field CÓ THỂ VẮNG MẶT hoàn
toàn — `TaoNguoiDungDtoV2` VẪN CHẤP NHẬN một object CHỈ có `ten`/
`email` (giống HỆT `TaoNguoiDungDtoV1`) mà KHÔNG lỗi biên dịch. Đây là
**non-breaking change** (thay đổi KHÔNG phá vỡ): mọi client CŨ tiếp
tục hoạt động, KHÔNG cần cập nhật GÌ.
::::

::::example{#breaking-change-tu-nhan-dien}
Khi buộc phải đổi **CẤU TRÚC** (breaking change — ví dụ `ten` tách
thành `ho` + `ten` riêng), field optional KHÔNG đủ — cần MỘT parser
**tự nhận diện version** qua field NÀO **HIỆN DIỆN** trong dữ liệu,
rồi chuẩn hoá **TẤT CẢ** về **CÙNG MỘT** domain type:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

// Domain type -- KHÔNG BAO GIỜ version, LUÔN một hình dạng duy nhất
type NguoiDungMoi = { hoTen: string; email: string; soDienThoai: string | null };

function parseTaoNguoiDung(input: unknown): Result<NguoiDungMoi, string[]> {
  if (typeof input !== "object" || input === null) return loi(["dữ liệu phải là object"]);
  const obj = input as Record<string, unknown>;

  // V3 (breaking): CÓ CẢ ho VÀ ten
  if ("ho" in obj && "ten" in obj) {
    const ho = typeof obj.ho === "string" ? obj.ho : "";
    const ten = typeof obj.ten === "string" ? obj.ten : "";
    const email = typeof obj.email === "string" ? obj.email : "";
    if (ho.trim() === "" || ten.trim() === "" || email.trim() === "") return loi(["thiếu trường bắt buộc"]);
    const soDienThoai = typeof obj.soDienThoai === "string" ? obj.soDienThoai : null;
    return ok({ hoTen: `${ho} ${ten}`, email, soDienThoai });
  }
  // V1/V2 (fallback): CHỈ CÓ ten
  if ("ten" in obj) {
    const ten = typeof obj.ten === "string" ? obj.ten : "";
    const email = typeof obj.email === "string" ? obj.email : "";
    if (ten.trim() === "" || email.trim() === "") return loi(["thiếu trường bắt buộc"]);
    const soDienThoai = typeof obj.soDienThoai === "string" ? obj.soDienThoai : null;
    return ok({ hoTen: ten, email, soDienThoai });
  }
  return loi(["không nhận diện được định dạng dữ liệu"]);
}

console.log(JSON.stringify(parseTaoNguoiDung({ ten: "An", email: "an@shop.vn" })));
console.log(JSON.stringify(parseTaoNguoiDung({ ten: "Binh", email: "binh@shop.vn", soDienThoai: "0900000000" })));
console.log(JSON.stringify(parseTaoNguoiDung({ ho: "Nguyen", ten: "Van Cuong", email: "cuong@shop.vn" })));
```

```text title=readonly
{"kind":"ok","giaTri":{"hoTen":"An","email":"an@shop.vn","soDienThoai":null}}
{"kind":"ok","giaTri":{"hoTen":"Binh","email":"binh@shop.vn","soDienThoai":"0900000000"}}
{"kind":"ok","giaTri":{"hoTen":"Nguyen Van Cuong","email":"cuong@shop.vn","soDienThoai":null}}
```

CẢ BA phiên bản (V1, V2, V3) ĐỀU chuẩn hoá về CÙNG hình dạng
`NguoiDungMoi` — code XỬ LÝ domain logic ĐỌC MỘT hình dạng DUY NHẤT,
KHÔNG BAO GIỜ cần biết dữ liệu GỐC đến từ version nào. **Nguyên tắc
cốt lõi**: domain type **KHÔNG BAO GIỜ** version — CHỈ DTO version;
mọi "rối rắm" của việc tương thích NGƯỢC bị NHỐT trong MỘT hàm parse
DUY NHẤT, ở BIÊN.
::::

::::predict{#doan-du-lieu-khong-nhan-dien-duoc commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDungMoi = { hoTen: string; email: string; soDienThoai: string | null };
function parseTaoNguoiDung(input: unknown): Result<NguoiDungMoi, string[]> {
  if (typeof input !== "object" || input === null) return loi(["dữ liệu phải là object"]);
  const obj = input as Record<string, unknown>;
  if ("ho" in obj && "ten" in obj) {
    const ho = typeof obj.ho === "string" ? obj.ho : "";
    const ten = typeof obj.ten === "string" ? obj.ten : "";
    const email = typeof obj.email === "string" ? obj.email : "";
    if (ho.trim() === "" || ten.trim() === "" || email.trim() === "") return loi(["thiếu trường bắt buộc"]);
    const soDienThoai = typeof obj.soDienThoai === "string" ? obj.soDienThoai : null;
    return ok({ hoTen: `${ho} ${ten}`, email, soDienThoai });
  }
  if ("ten" in obj) {
    const ten = typeof obj.ten === "string" ? obj.ten : "";
    const email = typeof obj.email === "string" ? obj.email : "";
    if (ten.trim() === "" || email.trim() === "") return loi(["thiếu trường bắt buộc"]);
    const soDienThoai = typeof obj.soDienThoai === "string" ? obj.soDienThoai : null;
    return ok({ hoTen: ten, email, soDienThoai });
  }
  return loi(["không nhận diện được định dạng dữ liệu"]);
}

// dữ liệu CHỈ có "ho" (không có "ten") -- không khớp V3 (cần CẢ HAI), không khớp V1/V2 (cần "ten")
const ketQua = parseTaoNguoiDung({ ho: "Nguyen", email: "x@shop.vn" });
console.log(ketQua.kind);
```

Dòng cuối in ra gì?

:::opt{correct}
`loi`
:::

:::opt
`ok` — vì `"ho" in obj` là `true`, nhánh V3 được kích hoạt NGAY (chỉ
cần CÓ `ho`, không cần kiểm thêm `"ten" in obj`)
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"ho" in obj` LÀ `true` cho dữ liệu này
— quan sát về field `ho` CÓ hiện diện đó đúng.

Chỗ lệch: điều kiện nhánh V3 là `"ho" in obj && "ten" in obj` — dùng
`&&` (VÀ), đòi CẢ HAI field CÙNG hiện diện. Với dữ liệu CHỈ có `ho`
(KHÔNG có `ten`): `"ho" in obj` là `true`, NHƯNG `"ten" in obj` là
`false` — `true && false` = `false` — nhánh V3 KHÔNG được vào. Nhánh
TIẾP THEO `if ("ten" in obj)` CŨNG `false` (không có `ten`). CẢ HAI
nhánh đều bị BỎ QUA, luồng chạy TỚI dòng CUỐI CÙNG: `return loi([...])`.
::
:::

:::opt
Máy báo lỗi biên dịch — `"ho" in obj` không hợp lệ khi `obj` có kiểu
`Record<string, unknown>`, toán tử `in` chỉ dùng được trên kiểu đã
biết TRƯỚC danh sách field
::why
Gần đúng ở việc bạn để ý `Record<string, unknown>` là một kiểu KHÁ
"mở" (không liệt kê field cụ thể nào) — một quan sát hợp lý về việc
kiểu này TRÔNG có vẻ "thiếu thông tin".

Chỗ lệch: toán tử `in` (kiểm tra MỘT property CÓ tồn tại trên object
hay không, LÚC CHẠY) hoạt động trên **MỌI** kiểu object, KỂ CẢ
`Record<string, unknown>` — đây CHÍNH LÀ trường hợp SỬ DỤNG chuẩn của
`in`: kiểm tra "field NÀY có mặt hay không" khi kiểu KHÔNG cho biết
trước. Biên dịch sạch.
::
:::
::::

::::code{#viet_parsetaonguoidung}
Tự viết phần dựng `hoTen` cho CẢ HAI nhánh nhận diện version.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDungMoi = { hoTen: string; email: string; soDienThoai: string | null };

function parseTaoNguoiDung(input: unknown): Result<NguoiDungMoi, string[]> {
  if (typeof input !== "object" || input === null) return loi(["dữ liệu phải là object"]);
  const obj = input as Record<string, unknown>;

  if ("ho" in obj && "ten" in obj) {
    const ho = typeof obj.ho === "string" ? obj.ho : "";
    const ten = typeof obj.ten === "string" ? obj.ten : "";
    const email = typeof obj.email === "string" ? obj.email : "";
    if (ho.trim() === "" || ten.trim() === "" || email.trim() === "") return loi(["thiếu trường bắt buộc"]);
    const soDienThoai = typeof obj.soDienThoai === "string" ? obj.soDienThoai : null;
    return ok({ hoTen: ___, email, soDienThoai });
  }
  if ("ten" in obj) {
    const ten = typeof obj.ten === "string" ? obj.ten : "";
    const email = typeof obj.email === "string" ? obj.email : "";
    if (ten.trim() === "" || email.trim() === "") return loi(["thiếu trường bắt buộc"]);
    const soDienThoai = typeof obj.soDienThoai === "string" ? obj.soDienThoai : null;
    return ok({ hoTen: ___, email, soDienThoai });
  }
  return loi(["không nhận diện được định dạng dữ liệu"]);
}

console.log(JSON.stringify(parseTaoNguoiDung({ ho: "Nguyen", ten: "Van Cuong", email: "cuong@shop.vn" })));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDungMoi = { hoTen: string; email: string; soDienThoai: string | null };

function parseTaoNguoiDung(input: unknown): Result<NguoiDungMoi, string[]> {
  if (typeof input !== "object" || input === null) return loi(["dữ liệu phải là object"]);
  const obj = input as Record<string, unknown>;

  if ("ho" in obj && "ten" in obj) {
    const ho = typeof obj.ho === "string" ? obj.ho : "";
    const ten = typeof obj.ten === "string" ? obj.ten : "";
    const email = typeof obj.email === "string" ? obj.email : "";
    if (ho.trim() === "" || ten.trim() === "" || email.trim() === "") return loi(["thiếu trường bắt buộc"]);
    const soDienThoai = typeof obj.soDienThoai === "string" ? obj.soDienThoai : null;
    return ok({ hoTen: `${ho} ${ten}`, email, soDienThoai });
  }
  if ("ten" in obj) {
    const ten = typeof obj.ten === "string" ? obj.ten : "";
    const email = typeof obj.email === "string" ? obj.email : "";
    if (ten.trim() === "" || email.trim() === "") return loi(["thiếu trường bắt buộc"]);
    const soDienThoai = typeof obj.soDienThoai === "string" ? obj.soDienThoai : null;
    return ok({ hoTen: ten, email, soDienThoai });
  }
  return loi(["không nhận diện được định dạng dữ liệu"]);
}

console.log(JSON.stringify(parseTaoNguoiDung({ ho: "Nguyen", ten: "Van Cuong", email: "cuong@shop.vn" })));
```

```typescript title=test
const kqV1 = parseTaoNguoiDung({ ten: "An", email: "an@shop.vn" });
if (kqV1.kind !== "ok") throw new Error("V1 hợp lệ phải ra ok");
if (kqV1.kind === "ok" && kqV1.giaTri.hoTen !== "An") throw new Error("V1: hoTen phải đúng bằng ten gốc");

const kqV2 = parseTaoNguoiDung({ ten: "Binh", email: "binh@shop.vn", soDienThoai: "0900000000" });
if (kqV2.kind !== "ok") throw new Error("V2 hợp lệ phải ra ok");
if (kqV2.kind === "ok" && kqV2.giaTri.soDienThoai !== "0900000000") throw new Error("V2: soDienThoai phải được giữ lại");

const kqV3 = parseTaoNguoiDung({ ho: "Nguyen", ten: "Van Cuong", email: "cuong@shop.vn" });
if (kqV3.kind !== "ok") throw new Error("V3 hợp lệ phải ra ok");
if (kqV3.kind === "ok" && kqV3.giaTri.hoTen !== "Nguyen Van Cuong") throw new Error("V3: hoTen phải nối ho VÀ ten, đúng thứ tự, cách nhau một dấu cách");

const khongNhanDien = parseTaoNguoiDung({ email: "x@shop.vn" });
if (khongNhanDien.kind !== "loi") throw new Error("dữ liệu thiếu cả ho lẫn ten phải ra loi");
```

:::hints
- kind: attention
  body: "Nhánh V3 (có cả ho VÀ ten): hoTen phải NỐI hai phần bằng template string, cách nhau một dấu cách (`ho ten`). Nhánh V1/V2 (chỉ có ten): hoTen đơn giản LÀ ten, không cần nối gì."
- kind: strategy
  body: '`${ho} ${ten}` : ten — nhánh V3 nối hai field bằng template string; nhánh V1/V2 dùng thẳng ten (đã là hoTen đầy đủ).'
- kind: one-line
  body: "___ (nhánh V3) = `${ho} ${ten}`\n___ (nhánh V1/V2) = ten"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Nguyen Van Cuong"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 6 hoàn tất: tách type theo layer, DTO JSON-safe, validate lúc
chạy, mapping hai chiều, API contract, versioning DTO — domain type
KHÔNG BAO GIỜ version. Cụm cuối: Persistence & Repository.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Domain object cần được LƯU vào database và ĐỌC lại sau. Repository
(nơi trung gian giữa domain và database) nên có hình dạng ra sao?
::::

::::checkpoint{mastery=0.8}
::::
