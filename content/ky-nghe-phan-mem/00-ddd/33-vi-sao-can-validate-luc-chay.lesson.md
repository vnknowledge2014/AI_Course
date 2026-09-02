---
id: ky-nghe-phan-mem.ddd.vi-sao-can-validate-luc-chay
title: "Vì sao cần validate dữ liệu lúc chạy — kiểu TypeScript biến mất"
summary: "TypeScript type BIẾN MẤT lúc runtime — JSON.parse() trả về any, dữ liệu từ API/DB/file KHÔNG đáng tin, compiler không bảo vệ được. Viết TAY một hàm parse nhận unknown, trả Result<T, string[]> — GOM HẾT lỗi tìm được (nối bài 30), tự viết không import gì."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 33
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.runtime-validation]
requires: [ddd.dto]
concepts: [ddd.runtime-validation]
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
`JSON.parse` trả kiểu `any` — TypeScript "tin" bất cứ gì bạn gán cho
nó. Dữ liệu THẬT từ bên ngoài (API request, file) có đúng hình dạng
không? Compiler KHÔNG biết.
::::

::::explain{#kieu-bien-mat-luc-chay}
Kiểu TypeScript CHỈ tồn tại LÚC BIÊN DỊCH — sau khi biên dịch xong,
CHƯƠNG TRÌNH chạy KHÔNG còn "nhớ" kiểu nào cả. `JSON.parse()` trả về
`any` — ép kiểu (`as TaoDonHang`) chỉ là LỜI HỨA với compiler, KHÔNG
kiểm tra GÌ lúc chạy:

```typescript
type TaoDonHang = { maKhachHang: string; matHangs: string[] };

const vanBanTuMang = '{"maKhachHang":123,"matHangs":"khong-phai-mang"}';
const duLieu = JSON.parse(vanBanTuMang) as TaoDonHang;

console.log(typeof duLieu.maKhachHang); // KHÔNG phải "string" như đã "hứa"
console.log(Array.isArray(duLieu.matHangs)); // KHÔNG phải mảng
```

```text
number
false
```

`as TaoDonHang` KHÔNG hề CHUYỂN ĐỔI hay KIỂM TRA — nó chỉ NÓI với
TypeScript "hãy TIN tôi, đây LÀ `TaoDonHang`". `maKhachHang` THỰC RA
là `number` (`123`), `matHangs` THỰC RA là `string` — TypeScript
KHÔNG cảnh báo gì (vì ép kiểu qua `any` luôn được CHẤP NHẬN), nhưng
code SAU ĐÓ gọi `duLieu.matHangs.length` hay `duLieu.maKhachHang.trim()`
sẽ **CRASH LÚC CHẠY**. Dữ liệu từ API request/DB/file LUÔN "không
đáng tin" — PHẢI kiểm THẬT, KHÔNG chỉ ép kiểu.
::::

::::example{#viet-tay-ham-parse}
Giải pháp: viết TAY một hàm nhận `unknown` (KHÔNG `any` — `unknown`
buộc PHẢI kiểm tra trước khi dùng), TRẢ `Result<T, string[]>` — GOM
HẾT lỗi tìm được (nối kỹ thuật `map2GomLoi`/mảng lỗi ở bài 30), KHÔNG
import thư viện nào (đúng tinh thần smart constructor, T4.3):

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type TaoDonHang = { maKhachHang: string; matHangs: string[] };

function parseTaoDonHang(input: unknown): Result<TaoDonHang, string[]> {
  if (typeof input !== "object" || input === null) {
    return loi(["dữ liệu phải là một object"]);
  }
  const obj = input as Record<string, unknown>;
  const loiList: string[] = [];

  if (typeof obj.maKhachHang !== "string" || obj.maKhachHang.trim() === "") {
    loiList.push("maKhachHang phải là chuỗi không rỗng");
  }
  if (!Array.isArray(obj.matHangs) || obj.matHangs.length === 0) {
    loiList.push("matHangs phải là mảng không rỗng");
  }

  if (loiList.length > 0) return loi(loiList);
  return ok({ maKhachHang: obj.maKhachHang as string, matHangs: obj.matHangs as string[] });
}

const vanBanHopLe = '{"maKhachHang":"KH-01","matHangs":["SP-01","SP-02"]}';
console.log(JSON.stringify(parseTaoDonHang(JSON.parse(vanBanHopLe))));

const vanBanCaHaiSai = '{"maKhachHang":"","matHangs":[]}';
console.log(JSON.stringify(parseTaoDonHang(JSON.parse(vanBanCaHaiSai))));
```

```text title=readonly
{"kind":"ok","giaTri":{"maKhachHang":"KH-01","matHangs":["SP-01","SP-02"]}}
{"kind":"loi","loi":["maKhachHang phải là chuỗi không rỗng","matHangs phải là mảng không rỗng"]}
```

Dữ liệu SAI CẢ HAI field → GOM ĐỦ HAI thông điệp, KHÔNG dừng ở lỗi
đầu (giống bài 30). Tham số `input: unknown` (KHÔNG PHẢI `any`) BUỘC
hàm phải KIỂM `typeof`/`Array.isArray` TRƯỚC KHI đọc bất kỳ field
nào — TypeScript KHÔNG cho phép truy cập `input.maKhachHang` trực
tiếp trên `unknown` mà không thu hẹp kiểu trước (`obj = input as
Record<string, unknown>` sau khi ĐÃ xác nhận `input` là object).
::::

::::predict{#doan-parse-null commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type TaoDonHang = { maKhachHang: string; matHangs: string[] };
function parseTaoDonHang(input: unknown): Result<TaoDonHang, string[]> {
  if (typeof input !== "object" || input === null) {
    return loi(["dữ liệu phải là một object"]);
  }
  const obj = input as Record<string, unknown>;
  const loiList: string[] = [];
  if (typeof obj.maKhachHang !== "string" || obj.maKhachHang.trim() === "") {
    loiList.push("maKhachHang phải là chuỗi không rỗng");
  }
  if (!Array.isArray(obj.matHangs) || obj.matHangs.length === 0) {
    loiList.push("matHangs phải là mảng không rỗng");
  }
  if (loiList.length > 0) return loi(loiList);
  return ok({ maKhachHang: obj.maKhachHang as string, matHangs: obj.matHangs as string[] });
}

// truyền null -- một API request bị thiếu body
console.log(parseTaoDonHang(null).kind);
```

Dòng cuối in ra gì?

:::opt{correct}
`loi`
:::

:::opt
Chương trình crash — vì `typeof null` trả về `"object"` trong
JavaScript (một sự kỳ quặc nổi tiếng của ngôn ngữ), nên điều kiện
`typeof input !== "object"` là `false`, hàm ĐI TIẾP tới
`const obj = input as Record<string, unknown>` với `input` LÀ `null`,
rồi đọc `obj.maKhachHang` trên `null` gây crash NGAY
::why
Gần đúng ở việc bạn nhớ ĐÚNG `typeof null === "object"` — MỘT SỰ THẬT
NỔI TIẾNG của JavaScript (di sản lịch sử của ngôn ngữ), và đúng là
NẾU hàm CHỈ kiểm `typeof input !== "object"` mà KHÔNG kiểm gì thêm,
kịch bản crash bạn mô tả LÀ CÓ THẬT.

Chỗ lệch: điều kiện Ở ĐÂY là `typeof input !== "object" || input ===
null` — CÓ THÊM một kiểm tra RIÊNG cho `null` bằng `||` (HOẶC). Với
`input = null`: `typeof null !== "object"` là `false`, NHƯNG `input
=== null` là `true` — `false || true` = `true` — điều kiện TỔNG vẫn
ĐÚNG, hàm `return loi([...])` NGAY, KHÔNG bao giờ chạm tới dòng đọc
`obj.maKhachHang`. Đây CHÍNH LÀ lý do phải kiểm `=== null` RIÊNG,
KHÔNG thể chỉ dựa vào `typeof`.
::
:::

:::opt
Máy báo lỗi biên dịch — `parseTaoDonHang` khai tham số `input:
unknown`, TypeScript không cho phép truyền `null` vào một tham số
kiểu `unknown`
::why
Gần đúng ở việc bạn để ý `unknown` LÀ một kiểu ĐẶC BIỆT, hạn chế hơn
`any` — một quan sát ĐÚNG (đó CHÍNH LÀ lý do dùng `unknown`, để ÉP
phải kiểm tra trước khi dùng).

Chỗ lệch: `unknown` có nghĩa là "CÓ THỂ là BẤT KỲ GIÁ TRỊ nào" —
NHẬN mọi giá trị (kể cả `null`, `undefined`, số, chuỗi, object...)
mà KHÔNG báo lỗi khi TRUYỀN VÀO. Giới hạn của `unknown` chỉ áp dụng
KHI ĐỌC giá trị đó ra (không cho phép gọi method/đọc field mà chưa
thu hẹp kiểu) — không áp dụng lúc TRUYỀN vào tham số. Biên dịch sạch.
::
:::
::::

::::code{#viet_parsetaodonhang}
Tự viết hai thông điệp lỗi trong `parseTaoDonHang`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type TaoDonHang = { maKhachHang: string; matHangs: string[] };

function parseTaoDonHang(input: unknown): Result<TaoDonHang, string[]> {
  if (typeof input !== "object" || input === null) {
    return loi(["dữ liệu phải là một object"]);
  }
  const obj = input as Record<string, unknown>;
  const loiList: string[] = [];

  if (typeof obj.maKhachHang !== "string" || obj.maKhachHang.trim() === "") {
    loiList.push(___);
  }
  if (!Array.isArray(obj.matHangs) || obj.matHangs.length === 0) {
    loiList.push(___);
  }

  if (loiList.length > 0) return loi(loiList);
  return ok({ maKhachHang: obj.maKhachHang as string, matHangs: obj.matHangs as string[] });
}

console.log(JSON.stringify(parseTaoDonHang({ maKhachHang: "KH-01", matHangs: ["SP-01"] })));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type TaoDonHang = { maKhachHang: string; matHangs: string[] };

function parseTaoDonHang(input: unknown): Result<TaoDonHang, string[]> {
  if (typeof input !== "object" || input === null) {
    return loi(["dữ liệu phải là một object"]);
  }
  const obj = input as Record<string, unknown>;
  const loiList: string[] = [];

  if (typeof obj.maKhachHang !== "string" || obj.maKhachHang.trim() === "") {
    loiList.push("maKhachHang phải là chuỗi không rỗng");
  }
  if (!Array.isArray(obj.matHangs) || obj.matHangs.length === 0) {
    loiList.push("matHangs phải là mảng không rỗng");
  }

  if (loiList.length > 0) return loi(loiList);
  return ok({ maKhachHang: obj.maKhachHang as string, matHangs: obj.matHangs as string[] });
}

console.log(JSON.stringify(parseTaoDonHang({ maKhachHang: "KH-01", matHangs: ["SP-01"] })));
```

```typescript title=test
const hopLe = parseTaoDonHang({ maKhachHang: "KH-01", matHangs: ["SP-01"] });
if (hopLe.kind !== "ok") throw new Error("dữ liệu hợp lệ phải ra ok");

const chiSaiTen = parseTaoDonHang({ maKhachHang: "", matHangs: ["SP-01"] });
if (chiSaiTen.kind !== "loi") throw new Error("maKhachHang rỗng phải ra loi");
if (chiSaiTen.kind === "loi" && chiSaiTen.loi.length !== 1) throw new Error("chỉ maKhachHang sai thì mảng lỗi phải có đúng MỘT phần tử");

const chiSaiMatHangs = parseTaoDonHang({ maKhachHang: "KH-01", matHangs: [] });
if (chiSaiMatHangs.kind !== "loi") throw new Error("matHangs rỗng phải ra loi");
if (chiSaiMatHangs.kind === "loi" && chiSaiMatHangs.loi.length !== 1) throw new Error("chỉ matHangs sai thì mảng lỗi phải có đúng MỘT phần tử");

if (chiSaiTen.kind === "loi" && (!chiSaiTen.loi[0] || chiSaiTen.loi[0].trim() === "")) throw new Error("thông điệp lỗi maKhachHang KHÔNG được rỗng");
if (chiSaiMatHangs.kind === "loi" && (!chiSaiMatHangs.loi[0] || chiSaiMatHangs.loi[0].trim() === "")) throw new Error("thông điệp lỗi matHangs KHÔNG được rỗng");
if (chiSaiTen.kind === "loi" && chiSaiMatHangs.kind === "loi" && chiSaiTen.loi[0] === chiSaiMatHangs.loi[0]) throw new Error("hai thông điệp lỗi phải KHÁC NHAU — mỗi lỗi mô tả đúng field của riêng nó");

const caHaiSai = parseTaoDonHang({ maKhachHang: "", matHangs: [] });
if (caHaiSai.kind !== "loi") throw new Error("sai cả hai field phải ra loi");
if (caHaiSai.kind === "loi" && caHaiSai.loi.length !== 2) throw new Error("sai cả hai field phải gom ĐỦ HAI lỗi, không dừng ở lỗi đầu");
```

:::hints
- kind: attention
  body: "Mỗi thông điệp là một chuỗi RIÊNG, mô tả ĐÚNG field đang kiểm — thông điệp maKhachHang và thông điệp matHangs PHẢI khác nhau, không dùng chung một câu chung chung."
- kind: strategy
  body: '"maKhachHang phải là chuỗi không rỗng" : "matHangs phải là mảng không rỗng" — mỗi push() một chuỗi mô tả đúng field.'
- kind: one-line
  body: '___ (maKhachHang) = "maKhachHang phải là chuỗi không rỗng"\n___ (matHangs) = "matHangs phải là mảng không rỗng"'
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
Validate lúc chạy: `unknown` buộc kiểm tra trước khi dùng, gom HẾT
lỗi tìm được — dữ liệu từ bên ngoài KHÔNG BAO GIỜ được tin ngay.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`parseTaoDonHang` (chiều NHẬP, DTO→Domain) CÓ THỂ lỗi (dữ liệu sai
hình dạng). Chiều NGƯỢC LẠI (Domain→DTO, như `mienSangDto` bài 32) —
domain object ĐÃ hợp lệ SẴN (smart constructor đã kiểm rồi) — mapping
đó CÓ CẦN `Result` không?
::::

::::checkpoint{mastery=0.8}
::::
