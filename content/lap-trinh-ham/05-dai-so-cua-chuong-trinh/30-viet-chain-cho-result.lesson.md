---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.viet-chain-cho-result
title: "Viết `chain` cho `Result` — CÙNG khuôn"
summary: "function chainResult<T,U,E>(r, f): Result<U,E> { switch(r.kind) { case \"ok\": return f(r.giaTri); case \"loi\": return loi(r.loi); } } — CÙNG khuôn chainOption. Bất kỳ kiểu có chain đúng luật được gọi là một Monad."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 30
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.chain-result-handwritten]
requires: [alg.chain-flattens]
concepts: [alg.chain-result-handwritten]
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
`chainOption` viết xong. `Result<T, E>` có CÙNG hình dạng — `chainResult`
có khác gì không?
::::

::::explain{#chain-cho-result-va-dat-ten-monad}
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

const chiaAnToan = (a: number, b: number): Result<number, string> => (b === 0 ? loi("chia cho 0") : ok(a / b));

console.log(JSON.stringify(chainResult(ok<number, string>(10), (x) => chiaAnToan(x, 2))));
```

```text
{"kind":"ok","giaTri":5}
```

Đặt `chainResult` cạnh `chainOption` (bài 29) — CÙNG cấu trúc HỆT
nhau, chỉ đổi tên biến thể (`co`/`khong` → `ok`/`loi`). Nhánh `"ok"`
TRẢ THẲNG kết quả của `f` (không bọc); nhánh `"loi"` GIỮ NGUYÊN lỗi
(đóng gói LẠI qua `loi(r.loi)`, không đổi nội dung lỗi).

Bất kỳ kiểu có `chain` ĐÚNG LUẬT (giống `Functor` có `map` đúng luật,
T4.5's bài 17-19) được gọi là một **Monad**. `Option`/`Result` VỪA là
`Functor` (có `map`) VỪA là `Monad` (có `chain`) — hai tính chất KHÔNG
loại trừ nhau, một kiểu có thể có CẢ HAI cùng lúc.
::::

::::example{#chainresult-giu-nguyen-loi}
`chainResult` trên một `"loi"` — giữ nguyên lỗi ĐÓ, `f` không hề chạy:

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

const chiaAnToan = (a: number, b: number): Result<number, string> => (b === 0 ? loi("chia cho 0") : ok(a / b));

const daLoi: Result<number, string> = loi("dữ liệu đầu vào sai");
const ketQua = chainResult(daLoi, (x) => chiaAnToan(x, 2));

console.log(JSON.stringify(ketQua));
```

```text title=readonly
{"kind":"loi","loi":"dữ liệu đầu vào sai"}
```

`daLoi` ĐÃ là một lỗi — `chainResult` trả về NGUYÊN VẸN lỗi đó, KHÔNG
gọi `chiaAnToan` một lần nào. Đúng bản chất fail-fast của `chain`:
bước SAU không thể chạy nếu KHÔNG có giá trị THẬT từ bước TRƯỚC.
::::

::::predict{#doan-chainresult-hai-buoc commitOnce}
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

const chiaAnToan = (a: number, b: number): Result<number, string> => (b === 0 ? loi("chia cho 0") : ok(a / b));

const buoc1 = chainResult(ok<number, string>(100), (x) => chiaAnToan(x, 5));
const buoc2 = chainResult(buoc1, (x) => chiaAnToan(x, 0));
const buoc3 = chainResult(buoc2, (x) => chiaAnToan(x, 2));

console.log(buoc3.kind);
```

Dòng cuối in ra gì?

:::opt{correct}
`loi`
:::

:::opt
`ok` — vì `buoc3` là bước CUỐI CÙNG, và `chiaAnToan(x, 2)` (chia cho
`2`, không phải `0`) không hề gây lỗi
::why
Gần đúng ở việc bạn tính đúng phép chia CỦA RIÊNG `buoc3`
(`chiaAnToan(x, 2)` — nếu `x` có giá trị, chia cho `2` không lỗi) —
phép tính CỤC BỘ đó đúng.

Chỗ lệch: `buoc3` PHỤ THUỘC `buoc2`, và `buoc2 = chainResult(buoc1, x
=> chiaAnToan(x, 0))` — chia cho `0`, GÂY LỖI ("chia cho 0") NGAY tại
bước NÀY. `buoc2` LÀ một `loi`. `chainResult(buoc2, ...)` (bước
`buoc3`) kiểm `buoc2.kind === "loi"` TRƯỚC, trả NGUYÊN VẸN lỗi đó,
KHÔNG hề gọi `chiaAnToan(x, 2)`. `buoc3.kind` là `"loi"`.
::
:::

:::opt
Máy báo lỗi biên dịch — không thể GHÉP BA `chainResult` liên tiếp
trên CÙNG một chuỗi biến đổi
::why
Gần đúng ở việc bạn cân nhắc GIỚI HẠN có thể có khi ghép nhiều
`chainResult` liên tiếp — một mối lo hợp lý.

Chỗ lệch: `chainResult` là một hàm BÌNH THƯỜNG, ghép được LIÊN TIẾP
bao nhiêu lần tuỳ ý — CHÍNH LÀ mục đích nó tồn tại (chuỗi nhiều bước
phụ thuộc nhau). Biên dịch và chạy hoàn toàn bình thường, dù ba lần
hay nhiều hơn.
::
:::
::::

::::code{#chain_result}
Tự viết `chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U,
E>): Result<U, E>`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return ___;
    case "loi": return ___;
  }
}

const chiaAnToan = (a: number, b: number): Result<number, string> => (b === 0 ? loi("chia cho 0") : ok(a / b));
console.log(JSON.stringify(chainResult(ok<number, string>(10), (x) => chiaAnToan(x, 2))));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

const chiaAnToan = (a: number, b: number): Result<number, string> => (b === 0 ? loi("chia cho 0") : ok(a / b));
console.log(JSON.stringify(chainResult(ok<number, string>(10), (x) => chiaAnToan(x, 2))));
```

```typescript title=test
const a = chainResult(ok<number, string>(10), (x) => chiaAnToan(x, 2));
if (a.kind !== "ok") throw new Error("chain trên ok phải giữ kind là \"ok\"");
if (a.kind === "ok" && a.giaTri !== 5) throw new Error("chain phải ra 5");

const b = chainResult(ok<number, string>(10), (x) => chiaAnToan(x, 0));
if (b.kind !== "loi") throw new Error("chain phải LẤY ĐÚNG kết quả thật của f (một lỗi ở đây)");
if (b.kind === "loi" && b.loi !== "chia cho 0") throw new Error("thông điệp lỗi phải khớp đúng lỗi mà f trả về");

const c = chainResult(loi<number, string>("lỗi trước"), (x: number) => chiaAnToan(x, 2));
if (c.kind !== "loi") throw new Error("chain trên loi phải giữ nguyên loi, không gọi f");
if (c.kind === "loi" && c.loi !== "lỗi trước") throw new Error("lỗi phải giữ nguyên nội dung gốc");
```

:::hints
- kind: attention
  body: "Nhánh \"ok\": TRẢ THẲNG f(r.giaTri) — KHÔNG bọc thêm ok(...). Nhánh \"loi\": giữ nguyên nội dung lỗi, đóng gói lại bằng loi(r.loi)."
- kind: strategy
  body: 'case "ok": return f(r.giaTri); case "loi": return loi(r.loi); — CÙNG khuôn chainOption, chỉ đổi tên biến thể.'
- kind: one-line
  body: "case \"ok\": return f(r.giaTri);\ncase \"loi\": return loi(r.loi);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`chainOption`, `chainResult` — cùng khuôn Monad, khác tên biến thể.
Ghép được nhiều bước liên tiếp, mỗi bước phụ thuộc bước trước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã có `chain` cho MỘT bước. Ghép NHIỀU bước liên tiếp — mỗi bước
NHẬN kết quả bước trước — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
