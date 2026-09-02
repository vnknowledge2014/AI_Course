---
id: ky-nghe-phan-mem.ddd.unwrap-or-trich-gia-tri-an-toan
title: "unwrapOr — trích giá trị an toàn với mặc định"
summary: "unwrapOr<T,E>(r: Result<T,E>, macDinh: T): T — trả giá trị THẬT nếu ok, trả macDinh nếu loi. Dùng khi caller CHỈ cần một giá trị cụ thể, không cần biết LÝ DO lỗi. Combinator MỚI, T4.5 chưa dạy."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 26
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ddd.unwrap-or]
requires: [ddd.throw-hides-errors]
concepts: [ddd.unwrap-or]
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
Có `Result<T,E>`, nhưng CHỈ cần MỘT giá trị cụ thể, không quan tâm LÝ
DO lỗi — viết `switch(r.kind)` mỗi lần cho việc ĐƠN GIẢN này hơi phí.
::::

::::explain{#unwrapor}
`unwrapOr<T, E>(r: Result<T, E>, macDinh: T): T` — combinator MỚI
(T4.5 CHƯA dạy): trả giá trị THẬT nếu `ok`, trả `macDinh` nếu `loi`.
KHÔNG cần biết lý do lỗi LÀ gì:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function unwrapOr<T, E>(r: Result<T, E>, macDinh: T): T {
  switch (r.kind) {
    case "ok": return r.giaTri;
    case "loi": return macDinh;
  }
}

function chuyenSo(vanBan: string): Result<number, string> {
  const n = Number(vanBan);
  if (Number.isNaN(n)) return loi(`"${vanBan}" không phải số`);
  return ok(n);
}

console.log(unwrapOr(chuyenSo("42"), 0));
console.log(unwrapOr(chuyenSo("abc"), 0));
```

```text
42
0
```

`chuyenSo("42")` ra `ok(42)` — `unwrapOr` trả `42` (giá trị THẬT).
`chuyenSo("abc")` ra `loi(...)` — `unwrapOr` trả `0` (giá trị `macDinh`
TRUYỀN VÀO), KHÔNG hề đọc chuỗi lỗi bên trong `loi("..."​)`. So với
`match`/`switch` thủ công: `unwrapOr` GỌN hơn hẳn khi caller CHỈ cần
một con số, KHÔNG cần phân nhánh xử lý riêng cho từng LOẠI lỗi.
::::

::::example{#unwrapor-khong-validate-lai}
Điểm DỄ NHẦM: `unwrapOr` CHỈ nhìn `r.kind` (`ok` hay `loi`) — nó KHÔNG
hề kiểm tra LẠI xem giá trị `ok` đó có "hợp lý" hay không:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function unwrapOr<T, E>(r: Result<T, E>, macDinh: T): T {
  switch (r.kind) {
    case "ok": return r.giaTri;
    case "loi": return macDinh;
  }
}
function chuyenSo(vanBan: string): Result<number, string> {
  const n = Number(vanBan);
  if (Number.isNaN(n)) return loi(`"${vanBan}" không phải số`);
  return ok(n);
}

console.log(unwrapOr(chuyenSo("-5"), 0));
console.log(unwrapOr(chuyenSo("mười"), 0));
```

```text title=readonly
-5
0
```

`chuyenSo("-5")` ra `ok(-5)` — `-5` LÀ một `number` HỢP LỆ về mặt
CHUYỂN ĐỔI (dù về NGHIỆP VỤ có thể vô lý, ví dụ số lượng hàng KHÔNG
được âm) — `chuyenSo` KHÔNG kiểm điều đó, nó CHỈ kiểm "có phải số hay
không". `unwrapOr` trả ĐÚNG `-5`, KHÔNG thay bằng `macDinh`. Muốn từ
chối số ÂM, phải thêm một bước validate RIÊNG (như `kiemKho`, bài 23)
— `unwrapOr` không "đoán" hộ ý định nghiệp vụ nào cả.
::::

::::predict{#doan-unwrapor-so-am commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function unwrapOr<T, E>(r: Result<T, E>, macDinh: T): T {
  switch (r.kind) {
    case "ok": return r.giaTri;
    case "loi": return macDinh;
  }
}
function chuyenSo(vanBan: string): Result<number, string> {
  const n = Number(vanBan);
  if (Number.isNaN(n)) return loi(`"${vanBan}" không phải số`);
  return ok(n);
}

console.log(unwrapOr(chuyenSo("0"), -1));
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`-1` — vì `0` là giá trị "rỗng"/giả (falsy) trong JavaScript, giống
như chuỗi rỗng hay `null`, nên `unwrapOr` coi nó tương đương `loi` và
trả về `macDinh`
::why
Gần đúng ở việc bạn nhớ ĐÚNG `0` LÀ một giá trị falsy trong JavaScript
— một quan sát THẬT về ngôn ngữ, hay bị nhầm áp dụng SAI chỗ.

Chỗ lệch: `unwrapOr` KHÔNG hề kiểm tra "falsy/truthy" — nó CHỈ đọc
`r.kind` (đúng LÀ chuỗi `"ok"` hay `"loi"`, MỘT thuộc tính CỤ THỂ,
không phải TOÀN BỘ object `r`). `chuyenSo("0")`: `Number("0")` là `0`,
`Number.isNaN(0)` là `false`, nên hàm trả `ok(0)` — `r.kind` LÀ
`"ok"`, `unwrapOr` đi vào nhánh `case "ok": return r.giaTri`, trả
ĐÚNG `0` (giá trị THẬT), không liên quan gì tới việc `0` có "falsy"
hay không.
::
:::

:::opt
Máy báo lỗi biên dịch — tham số THỨ HAI của `unwrapOr` (`-1`) có kiểu
khác với `chuyenSo("0")`'s giá trị `ok` (`0`), TypeScript đòi hai bên
phải BẰNG NHAU
::why
Gần đúng ở việc bạn để ý `unwrapOr` có HAI "nguồn" giá trị khả dĩ
(giá trị thật VÀ `macDinh`) — quan sát về việc CÓ hai nguồn đó đúng.

Chỗ lệch: TypeScript KHÔNG đòi hai nguồn phải "bằng nhau về GIÁ TRỊ"
— chỉ cần CÙNG KIỂU (`T`, ở đây SUY RA là `number` từ `chuyenSo`'s
`Result<number, string>`). `-1` là một `number` HỢP LỆ, khớp `T =
number` — biên dịch sạch, không có ràng buộc nào về giá trị CỤ THỂ.
::
:::
::::

::::code{#viet_unwrapor}
Tự viết PHẦN THÂN (implementation) của `unwrapOr`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function unwrapOr<T, E>(r: Result<T, E>, macDinh: T): T {
  switch (r.kind) {
    case "ok": return ___;
    case "loi": return ___;
  }
}

function chuyenSo(vanBan: string): Result<number, string> {
  const n = Number(vanBan);
  if (Number.isNaN(n)) return loi(`"${vanBan}" không phải số`);
  return ok(n);
}

console.log(unwrapOr(chuyenSo("42"), 0));
console.log(unwrapOr(chuyenSo("abc"), 0));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function unwrapOr<T, E>(r: Result<T, E>, macDinh: T): T {
  switch (r.kind) {
    case "ok": return r.giaTri;
    case "loi": return macDinh;
  }
}

function chuyenSo(vanBan: string): Result<number, string> {
  const n = Number(vanBan);
  if (Number.isNaN(n)) return loi(`"${vanBan}" không phải số`);
  return ok(n);
}

console.log(unwrapOr(chuyenSo("42"), 0));
console.log(unwrapOr(chuyenSo("abc"), 0));
```

```typescript title=test
if (unwrapOr(ok<number, string>(99), 0) !== 99) throw new Error("unwrapOr trên ok phải trả giá trị THẬT (99), không phải macDinh");
if (unwrapOr(loi<number, string>("lỗi bất kỳ"), 0) !== 0) throw new Error("unwrapOr trên loi phải trả macDinh (0)");
if (unwrapOr(ok<number, string>(0), -1) !== 0) throw new Error("unwrapOr trên ok(0) phải trả đúng 0, không được coi 0 là falsy rồi trả macDinh");
if (unwrapOr(chuyenSo("-5"), 100) !== -5) throw new Error("unwrapOr không tự validate lại giá trị ok — phải trả đúng -5");
```

:::hints
- kind: attention
  body: "Nhánh ok trả giá trị THẬT bên trong Result (r.giaTri); nhánh loi trả tham số macDinh truyền vào — không đọc r.loi ở đâu cả."
- kind: strategy
  body: 'case "ok": return r.giaTri; — lấy trực tiếp từ r. case "loi": return macDinh; — bỏ qua r.loi hoàn toàn.'
- kind: one-line
  body: '___ (case "ok") = r.giaTri\n___ (case "loi") = macDinh'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "42"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
unwrapOr: trích giá trị AN TOÀN, một mặc định thay cho việc phải xử lý
lỗi mỗi lần. Bước tiếp theo: khi CẦN xử lý cả hai nhánh, tường minh
hơn `switch` thủ công.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`unwrapOr` gọn cho trường hợp CHỈ cần một giá trị mặc định. Nhưng khi
CẦN xử lý CẢ HAI nhánh khác nhau (ví dụ: `ok` thì hiển thị, `loi` thì
ghi log) mà không muốn viết `switch` tay mỗi lần — công cụ nào?
::::

::::checkpoint{mastery=0.8}
::::
