---
id: ky-nghe-phan-mem.ddd.mapping-hai-chieu-inbound-outbound
title: "Mapping hai chiều: Inbound VALIDATE, Outbound chỉ TRANSFORM"
summary: "Chiều NHẬP (DTO→Domain): dữ liệu chưa chắc hợp lệ, validate rồi mới dựng domain object, trả Result<Mien, string[]>. Chiều XUẤT (Domain→DTO): domain object ĐÃ hợp lệ by construction, mapping CHỈ format lại — KHÔNG cần Result, KHÔNG THỂ fail."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 34
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ddd.inbound-outbound-mapping]
requires: [ddd.runtime-validation]
concepts: [ddd.inbound-outbound-mapping]
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
Bài trước: DTO→Domain CÓ THỂ lỗi (`Result`). Domain→DTO (bài 32) LẠI
KHÔNG dùng `Result` — vì sao HAI CHIỀU lại KHÁC NHAU?
::::

::::explain{#inbound-vs-outbound}
Hai chiều mapping có bản chất HOÀN TOÀN khác nhau:

**Chiều NHẬP** (Inbound, DTO→Domain): dữ liệu đến từ BÊN NGOÀI (API
request, `unknown`) — **CHƯA CHẮC hợp lệ**, PHẢI validate (bài 33)
rồi MỚI dựng domain object → trả `Result<Mien, string[]>`.

**Chiều XUẤT** (Outbound, Domain→DTO): domain object **ĐÃ hợp lệ BY
CONSTRUCTION** (smart constructor, bài 13, đã kiểm rồi — KHÔNG THỂ
tồn tại một `SanPhamMien` "sai") — mapping CHỈ **format lại** (bỏ
brand, `Date`→chuỗi ISO) — **KHÔNG cần `Result`**, **KHÔNG THỂ fail**:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type MaSanPham = string & { readonly __brand: "MaSanPham" };
type TienVND = number & { readonly __brand: "TienVND" };
type SanPhamMien = { ma: MaSanPham; ten: string; gia: TienVND };
type SanPhamDto = { ma: string; ten: string; gia: number };

// OUTBOUND: domain đã hợp lệ SẴN -- LUÔN thành công, chữ ký KHÔNG có Result
function mienSangDtoSanPham(sp: SanPhamMien): SanPhamDto {
  return { ma: sp.ma, ten: sp.ten, gia: sp.gia };
}

// INBOUND: input CHƯA CHẮC hợp lệ -- chữ ký TRẢ Result
function dtoSangMienSanPham(input: unknown): Result<SanPhamMien, string[]> {
  if (typeof input !== "object" || input === null) return loi(["dữ liệu phải là object"]);
  const obj = input as Record<string, unknown>;
  const loiList: string[] = [];
  if (typeof obj.ma !== "string" || obj.ma.trim() === "") loiList.push("ma phải là chuỗi không rỗng");
  if (typeof obj.ten !== "string" || obj.ten.trim() === "") loiList.push("ten phải là chuỗi không rỗng");
  if (typeof obj.gia !== "number" || obj.gia <= 0) loiList.push("gia phải là số dương");
  if (loiList.length > 0) return loi(loiList);
  return ok({ ma: obj.ma as MaSanPham, ten: obj.ten as string, gia: obj.gia as TienVND });
}

const spHopLe: SanPhamMien = { ma: "SP-01" as MaSanPham, ten: "Bút chì", gia: 5000 as TienVND };
console.log(JSON.stringify(mienSangDtoSanPham(spHopLe)));

console.log(JSON.stringify(dtoSangMienSanPham({ ma: "SP-02", ten: "Vở", gia: 10000 })));
console.log(JSON.stringify(dtoSangMienSanPham({ ma: "", ten: "", gia: -5 })));
```

```text
{"ma":"SP-01","ten":"Bút chì","gia":5000}
{"kind":"ok","giaTri":{"ma":"SP-02","ten":"Vở","gia":10000}}
{"kind":"loi","loi":["ma phải là chuỗi không rỗng","ten phải là chuỗi không rỗng","gia phải là số dương"]}
```

`mienSangDtoSanPham` trả THẲNG `SanPhamDto` — KHÔNG `{kind:"ok",...}`
nào bọc ngoài, vì KHÔNG có nhánh lỗi nào TỒN TẠI trong hàm. Ngược lại,
`dtoSangMienSanPham` LUÔN trả `Result` — dữ liệu SAI (`ma`/`ten` rỗng,
`gia` âm) là TÌNH HUỐNG THẬT SỰ có thể xảy ra khi dữ liệu đến từ BÊN
NGOÀI hệ thống.
::::

::::example{#tai-sao-outbound-khong-loi}
Vì SAO outbound KHÔNG THỂ fail: `SanPhamMien` chỉ được TẠO qua smart
constructor (bài 13) — MỌI giá trị `gia` bên trong MỘT `SanPhamMien`
ĐÃ được kiểm dương, MỌI `ten` ĐÃ được kiểm không rỗng TỪ TRƯỚC KHI
object đó TỒN TẠI. `mienSangDtoSanPham` KHÔNG CẦN kiểm LẠI những điều
đó — nó CHỈ đổi HÌNH DẠNG (bỏ brand, đổi tên field), KHÔNG đổi Ý
NGHĨA:

```typescript title=readonly
type MaSanPham = string & { readonly __brand: "MaSanPham" };
type TienVND = number & { readonly __brand: "TienVND" };
type SanPhamMien = { ma: MaSanPham; ten: string; gia: TienVND };
type SanPhamDto = { ma: string; ten: string; gia: number };
function mienSangDtoSanPham(sp: SanPhamMien): SanPhamDto {
  return { ma: sp.ma, ten: sp.ten, gia: sp.gia };
}

const spHopLe: SanPhamMien = { ma: "SP-01" as MaSanPham, ten: "Bút chì", gia: 5000 as TienVND };
const ketQua = mienSangDtoSanPham(spHopLe);
// thử đọc .kind như thể nó là Result
console.log(ketQua.kind);
```

```text title=readonly
TS2339: Property 'kind' does not exist on type 'SanPhamDto'.
```

`ketQua` có kiểu `SanPhamDto` TRỰC TIẾP, KHÔNG PHẢI `Result<SanPhamDto,
...>` — KHÔNG có field `kind` nào để đọc. Nếu `mienSangDtoSanPham` (SAI
LẦM) trả `Result` dù KHÔNG THỂ fail, MỌI nơi gọi nó sẽ PHẢI viết
`switch(r.kind)`/`match` một cách VÔ NGHĨA cho một nhánh `loi` KHÔNG
BAO GIỜ xảy ra — `Result` chỉ nên xuất hiện khi CÓ THẬT một khả năng
lỗi.
::::

::::predict{#doan-inbound-outbound-nguoc-chieu commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type MaSanPham = string & { readonly __brand: "MaSanPham" };
type TienVND = number & { readonly __brand: "TienVND" };
type SanPhamMien = { ma: MaSanPham; ten: string; gia: TienVND };

function dtoSangMienSanPham(input: unknown): Result<SanPhamMien, string[]> {
  if (typeof input !== "object" || input === null) return loi(["dữ liệu phải là object"]);
  const obj = input as Record<string, unknown>;
  const loiList: string[] = [];
  if (typeof obj.ma !== "string" || obj.ma.trim() === "") loiList.push("ma phải là chuỗi không rỗng");
  if (typeof obj.ten !== "string" || obj.ten.trim() === "") loiList.push("ten phải là chuỗi không rỗng");
  if (typeof obj.gia !== "number" || obj.gia <= 0) loiList.push("gia phải là số dương");
  if (loiList.length > 0) return loi(loiList);
  return ok({ ma: obj.ma as MaSanPham, ten: obj.ten as string, gia: obj.gia as TienVND });
}

// gia đúng NGƯỠNG 0
console.log(dtoSangMienSanPham({ ma: "SP-01", ten: "Bút chì", gia: 0 }).kind);
```

Dòng cuối in ra gì?

:::opt{correct}
`loi`
:::

:::opt
`ok` — vì `0` là một `number` HỢP LỆ về mặt KIỂU (`typeof 0 ===
"number"` là `true`), và điều kiện chỉ kiểm `typeof obj.gia !==
"number"`, không quan tâm giá trị CỤ THỂ là bao nhiêu
::why
Gần đúng ở việc bạn nhớ ĐÚNG `typeof 0 === "number"` — một quan sát
THẬT về JavaScript, và đúng LÀ nếu điều kiện CHỈ kiểm `typeof`, `0`
sẽ qua được vòng kiểm đó.

Chỗ lệch: điều kiện Ở ĐÂY là `typeof obj.gia !== "number" || obj.gia
<= 0` — có THÊM `|| obj.gia <= 0` (HOẶC giá trị nhỏ hơn hoặc BẰNG 0).
Với `obj.gia = 0`: `typeof 0 !== "number"` là `false`, NHƯNG `0 <= 0`
là `true` — `false || true` = `true` — nhánh lỗi ĐƯỢC kích hoạt,
`loiList.push("gia phải là số dương")` chạy, hàm trả `loi([...])`.
`gia` phải LỚN HƠN `0`, không chỉ "là một số".
::
:::

:::opt
Máy báo lỗi biên dịch — `gia: 0` trong object truyền vào không hợp lệ
vì `SanPhamMien`'s field `gia` có kiểu `TienVND` (branded), không
chấp nhận số `0` trần
::why
Gần đúng ở việc bạn nhớ ĐÚNG `SanPhamMien.gia` có kiểu BRANDED
(`TienVND`) — một quan sát ĐÚNG về CẤU TRÚC domain type.

Chỗ lệch: object truyền vào `dtoSangMienSanPham` có kiểu THAM SỐ là
`unknown` — KHÔNG PHẢI `SanPhamMien`. `{ ma: "SP-01", ten: "Bút chì",
gia: 0 }` là một OBJECT LITERAL BÌNH THƯỜNG (không có brand nào cả),
hoàn toàn hợp lệ để gán cho tham số `unknown` (nhận MỌI giá trị). Kiểu
`TienVND` CHỈ xuất hiện Ở ĐẦU RA của hàm (SAU KHI validate) — không
ràng buộc gì lên hình dạng INPUT.
::
:::
::::

::::code{#viet_mapping_hai_chieu}
Tự viết phần `gia` trong CẢ HAI chiều mapping.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type MaSanPham = string & { readonly __brand: "MaSanPham" };
type TienVND = number & { readonly __brand: "TienVND" };
type SanPhamMien = { ma: MaSanPham; ten: string; gia: TienVND };
type SanPhamDto = { ma: string; ten: string; gia: number };

function mienSangDtoSanPham(sp: SanPhamMien): SanPhamDto {
  return { ma: sp.ma, ten: sp.ten, gia: ___ };
}

function dtoSangMienSanPham(input: unknown): Result<SanPhamMien, string[]> {
  if (typeof input !== "object" || input === null) return loi(["dữ liệu phải là object"]);
  const obj = input as Record<string, unknown>;
  const loiList: string[] = [];
  if (typeof obj.ma !== "string" || obj.ma.trim() === "") loiList.push("ma phải là chuỗi không rỗng");
  if (typeof obj.ten !== "string" || obj.ten.trim() === "") loiList.push("ten phải là chuỗi không rỗng");
  if (typeof obj.gia !== "number" || obj.gia <= 0) loiList.push(___);
  if (loiList.length > 0) return loi(loiList);
  return ok({ ma: obj.ma as MaSanPham, ten: obj.ten as string, gia: obj.gia as TienVND });
}

const spHopLe: SanPhamMien = { ma: "SP-01" as MaSanPham, ten: "Bút chì", gia: 5000 as TienVND };
console.log(JSON.stringify(mienSangDtoSanPham(spHopLe)));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type MaSanPham = string & { readonly __brand: "MaSanPham" };
type TienVND = number & { readonly __brand: "TienVND" };
type SanPhamMien = { ma: MaSanPham; ten: string; gia: TienVND };
type SanPhamDto = { ma: string; ten: string; gia: number };

function mienSangDtoSanPham(sp: SanPhamMien): SanPhamDto {
  return { ma: sp.ma, ten: sp.ten, gia: sp.gia };
}

function dtoSangMienSanPham(input: unknown): Result<SanPhamMien, string[]> {
  if (typeof input !== "object" || input === null) return loi(["dữ liệu phải là object"]);
  const obj = input as Record<string, unknown>;
  const loiList: string[] = [];
  if (typeof obj.ma !== "string" || obj.ma.trim() === "") loiList.push("ma phải là chuỗi không rỗng");
  if (typeof obj.ten !== "string" || obj.ten.trim() === "") loiList.push("ten phải là chuỗi không rỗng");
  if (typeof obj.gia !== "number" || obj.gia <= 0) loiList.push("gia phải là số dương");
  if (loiList.length > 0) return loi(loiList);
  return ok({ ma: obj.ma as MaSanPham, ten: obj.ten as string, gia: obj.gia as TienVND });
}

const spHopLe: SanPhamMien = { ma: "SP-01" as MaSanPham, ten: "Bút chì", gia: 5000 as TienVND };
console.log(JSON.stringify(mienSangDtoSanPham(spHopLe)));
```

```typescript title=test
const spTest: SanPhamMien = { ma: "SP-99" as MaSanPham, ten: "Thước", gia: 12000 as TienVND };
const dtoTest = mienSangDtoSanPham(spTest);
if (dtoTest.gia !== 12000) throw new Error("mienSangDtoSanPham phải giữ đúng giá trị gia");
if (dtoTest.ma !== "SP-99") throw new Error("mienSangDtoSanPham: field ma phải giữ đúng ma, không lẫn với ten");
if (dtoTest.ten !== "Thước") throw new Error("mienSangDtoSanPham: field ten phải giữ đúng ten, không lẫn với ma");

const bienZero = dtoSangMienSanPham({ ma: "SP-01", ten: "Bút chì", gia: 0 });
if (bienZero.kind !== "loi") throw new Error("gia = 0 (đúng ngưỡng) phải bị từ chối");
const bienMot = dtoSangMienSanPham({ ma: "SP-01", ten: "Bút chì", gia: 1 });
if (bienMot.kind !== "ok") throw new Error("gia = 1 (ngưỡng + 1) phải hợp lệ");
if (bienMot.kind === "ok" && bienMot.giaTri.ma !== "SP-01") throw new Error("dtoSangMienSanPham: field ma phải giữ đúng ma, không lẫn với ten");
if (bienMot.kind === "ok" && bienMot.giaTri.ten !== "Bút chì") throw new Error("dtoSangMienSanPham: field ten phải giữ đúng ten, không lẫn với ma");

const chiSaiGia = dtoSangMienSanPham({ ma: "SP-01", ten: "Bút chì", gia: -100 });
if (chiSaiGia.kind !== "loi") throw new Error("gia âm phải ra loi");
if (chiSaiGia.kind === "loi" && (!chiSaiGia.loi[0] || chiSaiGia.loi[0].trim() === "")) throw new Error("thông điệp lỗi gia KHÔNG được rỗng");
if (chiSaiGia.kind === "loi" && chiSaiGia.loi.length !== 1) throw new Error("chỉ gia sai thì mảng lỗi phải có đúng MỘT phần tử");
```

:::hints
- kind: attention
  body: "mienSangDtoSanPham.gia: chỉ đơn giản lấy sp.gia (brand tự động gán được sang number). dtoSangMienSanPham: push MỘT thông điệp mô tả đúng vấn đề của gia (không dùng chung câu với ma/ten)."
- kind: strategy
  body: 'sp.gia : "gia phải là số dương" — outbound chỉ lấy giá trị có sẵn, inbound mô tả lỗi RÕ RÀNG cho field gia.'
- kind: one-line
  body: '___ (mienSangDtoSanPham) = sp.gia\n___ (dtoSangMienSanPham) = "gia phải là số dương"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "5000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Inbound validate (Result), outbound chỉ transform (không Result) —
`Result` chỉ xuất hiện khi CÓ THẬT một khả năng lỗi. Bước tiếp theo:
định nghĩa "hợp đồng" chính thức cho một API endpoint.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một endpoint API cần rõ ràng: client GỬI gì, server TRẢ gì, và khi
LỖI thì trả gì. Định nghĩa RÕ cả ba như một "hợp đồng" trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
