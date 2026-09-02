---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.ba-ket-qua-doc-lap-muon-gop-cung-luc
title: "Ba `Result` ĐỘC LẬP — muốn kiểm tra CẢ BA cùng lúc"
summary: "kiemTen/kiemTuoi/kiemEmail — BA phép kiểm KHÔNG phụ thuộc lẫn nhau. map/mapResult (cụm 3) chỉ xử lý MỘT Result, không đủ để GHÉP ba Result ĐỘC LẬP thành một kết quả."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 21
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.independent-values-motivation]
requires: [alg.review-functor]
concepts: [alg.independent-values-motivation]
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
`map`/`mapResult` biến đổi giá trị BÊN TRONG một `Result` — MỘT cái.
Nếu có BA `Result`, cần kiểm CẢ BA cùng lúc thì sao?
::::

::::explain{#ba-phep-kiem-doc-lap}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function kiemTen(ten: string): Result<string, string> {
  return ten.length > 0 ? ok(ten) : loi("tên không được rỗng");
}
function kiemTuoi(tuoi: number): Result<number, string> {
  return tuoi >= 0 && tuoi <= 150 ? ok(tuoi) : loi("tuổi phải trong khoảng 0-150");
}
function kiemEmail(email: string): Result<string, string> {
  return email.includes("@") ? ok(email) : loi("email phải có @");
}

console.log(kiemTen("An"));
console.log(kiemTuoi(200));
console.log(kiemEmail("an.example.com"));
```

```text
{"kind":"ok","giaTri":"An"}
{"kind":"loi","loi":"tuổi phải trong khoảng 0-150"}
{"kind":"loi","loi":"email phải có @"}
```

Ba hàm kiểm — `kiemTen`, `kiemTuoi`, `kiemEmail` — HOÀN TOÀN ĐỘC LẬP:
`kiemTuoi(200)` không cần biết `kiemTen` đã trả về gì, `kiemEmail`
không phụ thuộc kết quả của hai hàm kia. Muốn biết "cả BA trường có
hợp lệ không" — phải NHÌN CẢ BA kết quả CÙNG LÚC.

`map`/`mapResult` (cụm 3) chỉ xử lý MỘT `Result` — không có cách nào
GHÉP ba `Result` RIÊNG BIỆT thành MỘT kết quả tổng hợp bằng công cụ
đã có.
::::

::::example{#ghep-tay-rat-cong-kenh}
Ghép "tay" (không dùng công cụ mới) — được, nhưng CỒNG KỀNH, và LẶP
LẠI mỗi lần cần ghép:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function kiemTen(ten: string): Result<string, string> {
  return ten.length > 0 ? ok(ten) : loi("tên không được rỗng");
}
function kiemTuoi(tuoi: number): Result<number, string> {
  return tuoi >= 0 && tuoi <= 150 ? ok(tuoi) : loi("tuổi phải trong khoảng 0-150");
}

const rTen = kiemTen("An");
const rTuoi = kiemTuoi(30);

let ketQua: string;
if (rTen.kind === "loi") {
  ketQua = "lỗi: " + rTen.loi;
} else if (rTuoi.kind === "loi") {
  ketQua = "lỗi: " + rTuoi.loi;
} else {
  ketQua = "hợp lệ: " + rTen.giaTri + ", " + rTuoi.giaTri + " tuổi";
}

console.log(ketQua);
```

```text title=readonly
hợp lệ: An, 30 tuổi
```

Chạy ĐÚNG — nhưng phải viết `if/else if/else` TAY, kiểm TỪNG `Result`
theo THỨ TỰ, và logic này PHẢI viết LẠI mỗi lần cần ghép hai (hay ba,
bốn...) `Result` độc lập khác. Cồng kềnh hơn NHIỀU so với `mapResult`
đã đóng gói được khuôn "biến đổi bên trong, giữ vỏ" thành MỘT hàm dùng
lại được.
::::

::::predict{#doan-ghep-tay-nham-thu-tu commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function kiemTen(ten: string): Result<string, string> {
  return ten.length > 0 ? ok(ten) : loi("tên rỗng");
}
function kiemTuoi(tuoi: number): Result<number, string> {
  return tuoi >= 0 ? ok(tuoi) : loi("tuổi âm");
}

const rTen = kiemTen("");
const rTuoi = kiemTuoi(-5);

let ketQua: string;
if (rTen.kind === "loi") {
  ketQua = "lỗi: " + rTen.loi;
} else if (rTuoi.kind === "loi") {
  ketQua = "lỗi: " + rTuoi.loi;
} else {
  ketQua = "hợp lệ";
}

console.log(ketQua);
```

Dòng cuối in ra gì?

:::opt{correct}
`lỗi: tên rỗng`
:::

:::opt
`lỗi: tuổi âm` — vì `-5` là lỗi RÕ RÀNG hơn (một con số cụ thể sai),
nên `if/else if` sẽ ưu tiên kiểm nó trước
::why
Gần đúng ở việc bạn nhận ra CẢ HAI phép kiểm đều lỗi (`""` rỗng VÀ
`-5` âm) — quan sát đó đúng, cả hai đều thất bại.

Chỗ lệch: `if/else if` KHÔNG "ưu tiên lỗi rõ ràng hơn" — nó kiểm ĐÚNG
THỨ TỰ VIẾT trong mã: `if (rTen.kind === "loi")` đứng TRƯỚC. `rTen`
(kiểm tên) lỗi TRƯỚC TIÊN theo thứ tự viết, nên nhánh ĐÓ chạy, in
`"lỗi: tên rỗng"` — không bao giờ CHẠM tới `else if (rTuoi...)`, dù
`rTuoi` CŨNG lỗi.
::
:::

:::opt
Máy báo lỗi — không thể kiểm `rTen.kind === "loi"` khi CẢ HAI `rTen`
VÀ `rTuoi` đều là lỗi cùng lúc
::why
Gần đúng ở việc bạn để ý CẢ HAI kết quả đều lỗi CÙNG LÚC — quan sát đó
đúng (cả `""` và `-5` đều không hợp lệ).

Chỗ lệch: không có gì "xung đột" khi CẢ HAI `Result` đều là lỗi — mỗi
biểu thức `rTen.kind === "loi"` chỉ kiểm ĐÚNG MỘT `Result` tại một
thời điểm, hoàn toàn độc lập với `rTuoi`. Không lỗi biên dịch, không
lỗi chạy — chỉ đơn giản: chỉ MỘT lỗi (lỗi đầu tiên theo thứ tự viết)
được BÁO ra, lỗi còn lại "biến mất" khỏi kết quả.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ghép tay được, nhưng cồng kềnh — và như bài predict vừa cho thấy,
`if/else if` chỉ báo được MỘT lỗi, dù có NHIỀU lỗi cùng lúc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Có cách nào đóng gói khuôn "ghép nhiều `Result` độc lập" thành MỘT hàm
dùng lại được, giống `mapResult` đã làm cho "biến đổi bên trong"
không?

Bài sau viết đúng hàm đó.
::::

::::checkpoint{mastery=0.8}
::::
