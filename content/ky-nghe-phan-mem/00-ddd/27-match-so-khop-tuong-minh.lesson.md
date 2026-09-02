---
id: ky-nghe-phan-mem.ddd.match-so-khop-tuong-minh
title: "match — so khớp tường minh, bắt buộc xử lý CẢ HAI nhánh"
summary: "match<T,E,R>(r: Result<T,E>, xuLy: {ok: (x:T)=>R; loi: (e:E)=>R}): R — thay switch(r.kind) thủ công bằng MỘT object bắt buộc xử lý CẢ HAI nhánh, TypeScript báo lỗi nếu thiếu field."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 27
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ddd.match-result]
requires: [ddd.unwrap-or]
concepts: [ddd.match-result]
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
`unwrapOr` gọn khi CHỈ cần một mặc định. Nếu CẢ HAI nhánh cần xử lý
KHÁC NHAU (`ok` thì hiển thị, `loi` thì ghi log) — công cụ nào?
::::

::::explain{#match}
`match<T, E, R>(r: Result<T, E>, xuLy: { ok: (x: T) => R; loi: (e: E)
=> R }): R` — combinator MỚI: nhận MỘT OBJECT có đúng HAI hàm xử lý,
TRẢ VỀ kết quả của hàm KHỚP với `r.kind`:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function match<T, E, R>(r: Result<T, E>, xuLy: { ok: (x: T) => R; loi: (e: E) => R }): R {
  switch (r.kind) {
    case "ok": return xuLy.ok(r.giaTri);
    case "loi": return xuLy.loi(r.loi);
  }
}

type DonHang = { ma: string };
function taoDonHang(hopLe: boolean): Result<DonHang, string> {
  if (!hopLe) return loi("dữ liệu không hợp lệ");
  return ok({ ma: "DH-01" });
}

const thongDiep1 = match(taoDonHang(true), {
  ok: (dh) => `Đã tạo ${dh.ma}`,
  loi: (msg) => `Lỗi: ${msg}`,
});
console.log(thongDiep1);

const thongDiep2 = match(taoDonHang(false), {
  ok: (dh) => `Đã tạo ${dh.ma}`,
  loi: (msg) => `Lỗi: ${msg}`,
});
console.log(thongDiep2);
```

```text
Đã tạo DH-01
Lỗi: dữ liệu không hợp lệ
```

`match` ĐỌC như một CÂU LỆNH: "với kết quả này, NẾU ok thì làm A, NẾU
loi thì làm B" — TRÊN MỘT DÒNG, không cần khai `switch`/`case`/biến
tạm thủ công mỗi lần dùng.
::::

::::example{#match-bat-buoc-du}
Điểm KHÁC `switch` viết TAY: object truyền vào `match` PHẢI có ĐỦ CẢ
HAI field (`ok` VÀ `loi`) — TypeScript BÁO LỖI NGAY nếu THIẾU một
trong hai, KHÔNG đợi tới lúc chạy mới phát hiện:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function match<T, E, R>(r: Result<T, E>, xuLy: { ok: (x: T) => R; loi: (e: E) => R }): R {
  switch (r.kind) {
    case "ok": return xuLy.ok(r.giaTri);
    case "loi": return xuLy.loi(r.loi);
  }
}

const ketQua: Result<number, string> = ok(5);
// THIẾU nhánh loi
const thongDiep = match(ketQua, {
  ok: (x) => `giá trị: ${x}`,
});
console.log(thongDiep);
```

```text title=readonly
TS2345: Property 'loi' is missing in type '{ ok: (x: number) => string; }'
but required in type '{ ok: (x: number) => unknown; loi: (e: string) => unknown; }'.
```

Đây là LỢI ÍCH của `match` so với `switch` viết TAY: `switch(r.kind) {
case "ok": ... }` (quên `case "loi"`) VẪN biên dịch SẠCH — TypeScript
KHÔNG bắt buộc `switch` phải xử lý HẾT mọi case trừ khi thân hàm ĐÒI
kiểu trả về cụ thể ở MỌI nhánh. `match` biến việc "quên một nhánh"
thành LỖI KIỂU tường minh, NGAY lúc viết code — không phải lúc chạy.
::::

::::predict{#doan-match-tra-ve-cung-kieu commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function match<T, E, R>(r: Result<T, E>, xuLy: { ok: (x: T) => R; loi: (e: E) => R }): R {
  switch (r.kind) {
    case "ok": return xuLy.ok(r.giaTri);
    case "loi": return xuLy.loi(r.loi);
  }
}

const ketQuaLoi: Result<number, string> = loi("số âm");

const soDong = match(ketQuaLoi, {
  ok: (x) => x,
  loi: (msg) => -1,
});
console.log(soDong);
console.log(typeof soDong);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`-1` rồi `number`
:::

:::opt
`-1` rồi `string` — vì `ketQuaLoi` chứa một chuỗi (`"số âm"`), và
`match` trả về GIÁ TRỊ LIÊN QUAN tới nhánh ĐÃ khớp, nên `typeof` phản
ánh kiểu CỦA CHUỖI đó
::why
Gần đúng ở việc bạn nhớ ĐÚNG `ketQuaLoi` MANG một chuỗi (`"số âm"`,
kiểu `E = string`) — quan sát về nội dung của `loi` đó đúng.

Chỗ lệch: `soDong` KHÔNG PHẢI là `msg` (chuỗi lỗi) — nó là GIÁ TRỊ hàm
`xuLy.loi` TRẢ VỀ, ở đây `loi: (msg) => -1` LUÔN trả `-1` (kiểu
`number`), bất kể `msg` chứa gì. `R` (kiểu trả về CHUNG của CẢ HAI
nhánh `ok` VÀ `loi`) được TypeScript SUY RA là `number` — vì CẢ HAI
hàm `xuLy.ok` (`(x) => x`, trả `number`) VÀ `xuLy.loi` (trả `-1`) đều
trả `number`. `typeof soDong` LUÔN là `"number"`.
::
:::

:::opt
Máy báo lỗi biên dịch — hai nhánh `ok`/`loi` trả kiểu KHÁC NHAU
(`ok` trả `T = number`, `loi` trả `-1`), TypeScript đòi hai hàm PHẢI
CÙNG một kiểu trả về TƯỜNG MINH khai trong `xuLy`
::why
Gần đúng ở việc bạn để ý `ok: (x) => x` và `loi: (msg) => -1` là HAI
BIỂU THỨC "trông" khác dạng nhau — một quan sát bề mặt hợp lý.

Chỗ lệch: `match`'s tham số kiểu `R` LÀ MỘT kiểu DUY NHẤT, ĐƯỢC
TypeScript TỰ SUY RA sao cho CẢ HAI hàm ĐỀU khớp — không cần khai
tường minh. Ở đây CẢ hai đều trả `number` (`x` có kiểu `T = number`
từ `Result<number, string>`; `-1` cũng là `number`), nên `R` suy ra
`number` — biên dịch sạch, không có mâu thuẫn kiểu nào.
::
:::
::::

::::code{#viet_match}
Tự viết PHẦN THÂN (implementation) của `match`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function match<T, E, R>(r: Result<T, E>, xuLy: { ok: (x: T) => R; loi: (e: E) => R }): R {
  switch (r.kind) {
    case "ok": return ___;
    case "loi": return ___;
  }
}

const ketQua: Result<number, string> = ok(10);
console.log(match(ketQua, { ok: (x) => x * 2, loi: (msg) => 0 }));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function match<T, E, R>(r: Result<T, E>, xuLy: { ok: (x: T) => R; loi: (e: E) => R }): R {
  switch (r.kind) {
    case "ok": return xuLy.ok(r.giaTri);
    case "loi": return xuLy.loi(r.loi);
  }
}

const ketQua: Result<number, string> = ok(10);
console.log(match(ketQua, { ok: (x) => x * 2, loi: (msg) => 0 }));
```

```typescript title=test
const okTest: Result<number, string> = ok(7);
if (match(okTest, { ok: (x) => x * 10, loi: () => -1 }) !== 70) throw new Error("match trên ok phải gọi hàm ok với đúng giá trị bên trong");

const loiTest: Result<number, string> = loi("hỏng");
if (match(loiTest, { ok: (x) => x * 10, loi: () => -1 }) !== -1) throw new Error("match trên loi phải gọi hàm loi, KHÔNG gọi hàm ok");

let daGoiLoi = false;
match(loiTest, { ok: () => "ok", loi: (msg) => { daGoiLoi = true; return msg; } });
if (!daGoiLoi) throw new Error("nhánh loi phải nhận đúng thông điệp lỗi bên trong Result");
```

:::hints
- kind: attention
  body: "Nhánh case \"ok\" gọi HÀM xuLy.ok, truyền r.giaTri; nhánh case \"loi\" gọi HÀM xuLy.loi, truyền r.loi — match ỦY QUYỀN việc xử lý cho object truyền vào, không tự tính gì."
- kind: strategy
  body: "xuLy.ok(r.giaTri) : xuLy.loi(r.loi) — gọi đúng hàm tương ứng nhánh, truyền đúng giá trị bên trong r."
- kind: one-line
  body: '___ (case "ok") = xuLy.ok(r.giaTri)\n___ (case "loi") = xuLy.loi(r.loi)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
match: xử lý CẢ HAI nhánh tường minh, TypeScript BẮT BUỘC đủ cả hai
field — không có nhánh nào "quên" mà biên dịch vẫn qua.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`unwrapOr`, `match` đều thao tác trên `Result` CÓ SẴN. Nhưng nhiều code
CŨ (thư viện có sẵn, ví dụ `JSON.parse`) vẫn `throw` — làm sao "bọc"
nó thành `Result` mà KHÔNG viết `try/catch` lặp lại ở MỌI nơi gọi?
::::

::::checkpoint{mastery=0.8}
::::
