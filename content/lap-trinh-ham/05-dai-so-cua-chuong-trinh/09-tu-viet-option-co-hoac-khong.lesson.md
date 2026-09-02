---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.tu-viet-option-co-hoac-khong
title: "Tự viết `Option<T>` — `\"co\"` hoặc `\"khong\"`"
summary: "type Option<T> = { kind: \"co\"; giaTri: T } | { kind: \"khong\" } cùng co<T>()/khong<T>() — CHÍNH LÀ discriminated union T4.3 đã dạy, dùng để thay null/undefined, tránh bẫy falsy-value bài 8 đã đo."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.option-type]
requires: [alg.avoid-null-motivation]
concepts: [alg.option-type]
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
`undefined` cho phép nhầm giá trị `0`/`""` với "không có gì". Hôm nay
tự viết một kiểu KHÔNG mắc bẫy đó.
::::

::::explain{#tu-viet-option}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };

function co<T>(giaTri: T): Option<T> {
  return { kind: "co", giaTri };
}
function khong<T>(): Option<T> {
  return { kind: "khong" };
}

function timSoChan(ds: number[]): Option<number> {
  const x = ds.find((n) => n % 2 === 0);
  return x === undefined ? khong() : co(x);
}

const ketQua = timSoChan([5, 3, 0, 7]);
console.log(ketQua);
if (ketQua.kind === "co") {
  console.log("tìm thấy: " + ketQua.giaTri);
} else {
  console.log("không tìm thấy");
}
```

```text
{"kind":"co","giaTri":0}
tìm thấy: 0
```

`Option<T>` KHÔNG phải khái niệm hạ tầng mới — nó CHÍNH LÀ discriminated
union (T4.3 đã dạy hết), chỉ dùng cho MỘT việc CỤ THỂ: thay `undefined`.
`co(giaTri)` "gói" một giá trị THẬT SỰ có; `khong()` biểu diễn "không
có gì" — nhưng dưới dạng một BIẾN THỂ RIÊNG, không phải một giá trị
"đặc biệt" (`undefined`) LẪN VÀO cùng loại với giá trị thật.

Kết quả: `ds.find(...)` tìm thấy `0` → `timSoChan` trả `co(0)` — kiểm
`ketQua.kind === "co"` ĐÚNG (`0` được gói trong biến thể `"co"`, không
liên quan gì tới việc `0` là "falsy" hay không). Bẫy bài 8 KHÔNG THỂ
xảy ra ở đây — không có kiểm tra "truthy/falsy" nào, chỉ có kiểm NHÃN
`kind`.
::::

::::example{#option-buoc-kiem-day-du}
`Option<T>` BUỘC bạn phải kiểm ĐỦ hai nhánh — quên một nhánh, TypeScript
không cho đọc `giaTri` một cách tuỳ tiện:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };

function co<T>(giaTri: T): Option<T> {
  return { kind: "co", giaTri };
}
function khong<T>(): Option<T> {
  return { kind: "khong" };
}

const a: Option<number> = co(5);
const b: Option<number> = khong();

console.log(a.kind === "co" ? a.giaTri : "không có");
console.log(b.kind === "co" ? b.giaTri : "không có");
```

```text title=readonly
5
không có
```

Đọc `a.giaTri` TRỰC TIẾP (không kiểm `a.kind` trước) sẽ bị TypeScript
CHẶN — `Option<number>` là UNION, biến thể `"khong"` KHÔNG CÓ field
`giaTri`. Buộc phải NARROW (`a.kind === "co" ? a.giaTri : ...`) trước
khi đọc — không có cách nào "quên kiểm rồi lỡ đọc phải `undefined`"
như với `T | undefined` trần.
::::

::::predict{#doan-option-tu-viet commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };

function co<T>(giaTri: T): Option<T> {
  return { kind: "co", giaTri };
}
function khong<T>(): Option<T> {
  return { kind: "khong" };
}

const rong: Option<string> = co("");
console.log(rong.kind);
console.log(rong.kind === "co" ? "có giá trị" : "không có");
```

Hai dòng cuối in ra gì?

:::opt{correct}
`co` rồi `có giá trị`
:::

:::opt
`khong` rồi `không có` — vì `co("")` với chuỗi RỖNG bị coi như KHÔNG
có giá trị, giống bẫy `if (x)` của `undefined`
::why
Gần đúng ở việc bạn nhớ ĐÚNG bẫy falsy-value từ bài 8 (`""` là giá trị
"falsy") — bẫy đó có thật, với `undefined`/`if (x)`.

Chỗ lệch: `Option<T>` KHÔNG dùng `if (x)` — nó kiểm field `kind`
(`rong.kind`), một CHUỖI LITERAL (`"co"` hoặc `"khong"`), KHÔNG PHẢI
kiểm truthy/falsy của `giaTri`. `co("")` LUÔN có `kind: "co"`, bất kể
giá trị BÊN TRONG là gì (kể cả chuỗi rỗng) — đây CHÍNH LÀ lý do
`Option<T>` tránh được bẫy đó.
::
:::

:::opt
Máy báo lỗi biên dịch — `co("")` không hợp lệ vì chuỗi rỗng không
"đáng" được gói vào `Option`
::why
Gần đúng ở việc bạn cảnh giác chuỗi rỗng có gì đó "đặc biệt" — một mối
lo hợp lý nếu nhớ tới các bẫy falsy-value đã học.

Chỗ lệch: `co<T>(giaTri: T)` nhận BẤT KỲ giá trị nào kiểu `T` — không
có giới hạn "giá trị phải khác rỗng/khác 0". `co("")` biên dịch và
chạy hoàn toàn bình thường, y hệt `co("chao")` hay `co(0)`.
::
:::
::::

::::code{#option_constructors}
Tự viết `co<T>(giaTri: T): Option<T>` và `khong<T>(): Option<T>`.

```typescript title=starter
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };

function co<T>(giaTri: T): Option<T> {
  return ___;
}
function khong<T>(): Option<T> {
  return ___;
}

console.log(co(5));
```

```typescript title=solution
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };

function co<T>(giaTri: T): Option<T> {
  return { kind: "co", giaTri };
}
function khong<T>(): Option<T> {
  return { kind: "khong" };
}

console.log(co(5));
```

```typescript title=test
const a = co(5);
if (a.kind !== "co") throw new Error("co() phải trả về kind là \"co\"");
if (a.kind === "co" && a.giaTri !== 5) throw new Error("co(5) phải giữ giaTri là 5");
const b = khong<number>();
if (b.kind !== "khong") throw new Error("khong() phải trả về kind là \"khong\"");
const c = co(0);
if (c.kind !== "co") throw new Error("co(0) phải có kind là \"co\", kể cả với giá trị falsy như 0");
```

:::hints
- kind: attention
  body: "co() phải trả về một object có kind: \"co\" VÀ giaTri (giá trị truyền vào). khong() phải trả về object có kind: \"khong\", KHÔNG có field giaTri."
- kind: strategy
  body: 'return { kind: "co", giaTri } — dùng shorthand property (giaTri: giaTri viết gọn thành giaTri). return { kind: "khong" } — không field nào khác.'
- kind: one-line
  body: "co: return { kind: \"co\", giaTri };  khong: return { kind: \"khong\" };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "co"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Option<T>` — cùng ADT bạn đã biết từ T4.3, dùng cho một việc cụ thể:
thay `undefined`, tránh bẫy falsy-value hoàn toàn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã có `Option<T>` — nhưng vẫn phải dùng `if (x.kind === "co")` để
đọc nó. Điều đó có buộc TypeScript kiểm ĐỦ cả hai nhánh không, hay vẫn
có thể "quên" một nhánh mà không bị báo?

Bài sau kiểm tra đúng câu hỏi đó, nối thẳng T4.3.
::::

::::checkpoint{mastery=0.8}
::::
