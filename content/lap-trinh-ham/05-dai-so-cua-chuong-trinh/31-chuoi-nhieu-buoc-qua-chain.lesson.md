---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.chuoi-nhieu-buoc-qua-chain
title: "Chuỗi NHIỀU bước qua `chain` — mỗi bước phụ thuộc bước trước"
summary: "chainResult(chainResult(chuyenSo(vb), x => chia(100, x)), y => canBac2(y)) — ba bước, MỖI bước NHẬN kết quả bước TRƯỚC. Khác cụm 4's Applicative (giá trị ĐỘC LẬP), chain BẮT BUỘC fail-fast."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 31
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.chain-sequential-steps]
requires: [alg.chain-result-handwritten]
concepts: [alg.chain-sequential-steps]
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
`chainResult` ghép được HAI bước. Ba bước, MỖI bước cần giá trị THẬT
của bước trước — trông thế nào?
::::

::::explain{#ba-buoc-phu-thuoc}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function chuyenSo(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi("\"" + vb + "\" không phải số") : ok(n);
}
function chia(a: number, b: number): Result<number, string> {
  return b === 0 ? loi("chia cho 0") : ok(a / b);
}
function canBac2(x: number): Result<number, string> {
  return x < 0 ? loi("không lấy được căn bậc hai của số âm") : ok(Math.sqrt(x));
}

const ketQua = chainResult(chainResult(chuyenSo("4"), (x) => chia(100, x)), (y) => canBac2(y));
console.log(ketQua);
```

```text
{"kind":"ok","giaTri":5}
```

Ba bước: `chuyenSo("4")` ra `4`, `chia(100, 4)` ra `25` (BƯỚC NÀY CẦN
`4` — giá trị THẬT từ bước trước, không phải một `Result<number,...>`
trừu tượng), `canBac2(25)` ra `5` (CẦN `25` — giá trị THẬT của bước
TRƯỚC ĐÓ). Mỗi bước NHẬN đúng giá trị SỐ đã "mở ra" từ `Result` của
bước trước — KHÁC HẲN cụm 4's Applicative (`map2`/`map2GomLoi`), nơi
CÁC giá trị ĐỘC LẬP, không bên nào CẦN giá trị của bên kia.

`chain` BẮT BUỘC fail-fast: bước SAU CẦN giá trị THẬT của bước TRƯỚC
để CHẠY ĐƯỢC — không có cách nào "chạy song song rồi gom lỗi" như
Applicative, vì đơn giản không có gì để "chạy song song" (bước sau
CHƯA CÓ đầu vào cho tới khi bước trước xong).
::::

::::example{#mot-buoc-loi-dung-toan-chuoi}
Một bước LỖI GIỮA CHUỖI — dừng NGAY, các bước SAU không hề chạy:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function chuyenSo(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi("\"" + vb + "\" không phải số") : ok(n);
}
function chia(a: number, b: number): Result<number, string> {
  return b === 0 ? loi("chia cho 0") : ok(a / b);
}
function canBac2(x: number): Result<number, string> {
  return x < 0 ? loi("không lấy được căn bậc hai của số âm") : ok(Math.sqrt(x));
}

let soLanGoiCanBac2 = 0;
const canBac2CoDem = (x: number): Result<number, string> => {
  soLanGoiCanBac2 = soLanGoiCanBac2 + 1;
  return canBac2(x);
};

const ketQua = chainResult(chainResult(chuyenSo("0"), (x) => chia(100, x)), canBac2CoDem);
console.log(ketQua);
console.log(soLanGoiCanBac2);
```

```text title=readonly
{"kind":"loi","loi":"chia cho 0"}
0
```

`chuyenSo("0")` ra `0` — HỢP LỆ, không lỗi. Nhưng `chia(100, 0)` LỖI
(chia cho `0`) — bước THỨ HAI thất bại. `canBac2CoDem` (bước THỨ BA)
KHÔNG BAO GIỜ được gọi — `soLanGoiCanBac2` giữ nguyên `0`. Một bước
lỗi GIỮA chuỗi dừng NGAY, đúng bản chất fail-fast của `chain` — nối
thẳng T4.2's pipeline error-handling (bài 28), giờ áp dụng cho một
CHUỖI `chain` thay vì `pipe`.
::::

::::predict{#doan-chuoi-ba-buoc-loi-som commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function chuyenSo(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? loi("\"" + vb + "\" không phải số") : ok(n);
}
function chia(a: number, b: number): Result<number, string> {
  return b === 0 ? loi("chia cho 0") : ok(a / b);
}
function canBac2(x: number): Result<number, string> {
  return x < 0 ? loi("không lấy được căn bậc hai của số âm") : ok(Math.sqrt(x));
}

const ketQua = chainResult(chainResult(chuyenSo("abc"), (x) => chia(100, x)), canBac2);
console.log(ketQua.kind === "loi" ? ketQua.loi : "không lỗi");
```

Dòng cuối in ra gì?

:::opt{correct}
`"abc" không phải số`
:::

:::opt
`chia cho 0` — vì `chuyenSo("abc")` không parse được, TypeScript coi
nó như `0` mặc định, rồi `chia(100, 0)` mới là bước gây lỗi thật
::why
Gần đúng ở việc bạn nghĩ tới một CƠ CHẾ "giá trị mặc định" khi parse
thất bại — một hành vi có thật ở một số ngôn ngữ/hàm khác.

Chỗ lệch: `chuyenSo` KHÔNG có cơ chế "mặc định về 0" — `Number("abc")`
ra `NaN`, và `chuyenSo` kiểm `Number.isNaN(n)` NGAY, trả về
`loi("\"abc\" không phải số")` — LỖI ngay từ BƯỚC ĐẦU TIÊN. `chia`
(bước thứ hai) KHÔNG BAO GIỜ được gọi tới, không liên quan gì tới
"chia cho 0".
::
:::

:::opt
Máy báo lỗi biên dịch — `chuyenSo("abc")` không hợp lệ vì `"abc"`
không phải một chuỗi SỐ ngay từ lúc viết mã
::why
Gần đúng ở việc bạn để ý `"abc"` không phải một chuỗi biểu diễn SỐ —
quan sát về NỘI DUNG chuỗi đó đúng.

Chỗ lệch: `chuyenSo` nhận tham số kiểu `string` — `"abc"` LÀ một
`string` HOÀN TOÀN HỢP LỆ về mặt KIỂU (TypeScript không kiểm NỘI DUNG
chuỗi lúc biên dịch, chỉ kiểm nó CÓ PHẢI `string` không). Việc `"abc"`
không parse được thành số là một vấn đề LÚC CHẠY, xử lý bằng
`Result`/`loi`, không phải lỗi biên dịch.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chuỗi `chain` — mỗi bước phụ thuộc bước trước, dừng NGAY tại lỗi đầu
tiên. Không có cách nào "bỏ qua" một bước phụ thuộc để xem tiếp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có CẢ `map` LẪN `chain` — khi nào dùng cái nào?
::::

::::checkpoint{mastery=0.8}
::::
