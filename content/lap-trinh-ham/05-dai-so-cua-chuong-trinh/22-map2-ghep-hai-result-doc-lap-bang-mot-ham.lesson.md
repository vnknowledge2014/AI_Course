---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.map2-ghep-hai-result-doc-lap-bang-mot-ham
title: "`map2` — ghép HAI `Result` độc lập bằng một hàm"
summary: "function map2<A,B,C,E>(ra, rb, f) { if (ra.kind===\"loi\") return loi(ra.loi); if (rb.kind===\"loi\") return loi(rb.loi); return ok(f(ra.giaTri, rb.giaTri)); } — kiểu FAIL-FAST: kiểm ra trước, nếu lỗi TRẢ NGAY."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 22
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.map2-handwritten]
requires: [alg.independent-values-motivation]
concepts: [alg.map2-handwritten]
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
Ghép "tay" bằng `if/else` cồng kềnh, và như bài trước cho thấy, dễ bỏ
sót lỗi. Hôm nay đóng gói khuôn đó thành một hàm dùng lại được.
::::

::::explain{#map2-la-gi}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function map2<A, B, C, E>(
  ra: Result<A, E>,
  rb: Result<B, E>,
  f: (a: A, b: B) => C
): Result<C, E> {
  if (ra.kind === "loi") return loi(ra.loi);
  if (rb.kind === "loi") return loi(rb.loi);
  return ok(f(ra.giaTri, rb.giaTri));
}

console.log(map2(ok<number, string>(3), ok<number, string>(4), (a, b) => a + b));
```

```text
{"kind":"ok","giaTri":7}
```

`map2(ra, rb, f)` — nếu `ra` LỖI, trả về LỖI ĐÓ NGAY (không cần xem
`rb`). Nếu `ra` ổn nhưng `rb` LỖI, trả LỖI CỦA `rb`. Chỉ khi CẢ HAI
đều `ok`, gọi `f` với CẢ hai giá trị, đóng gói kết quả vào `ok(...)`.

Đây là kiểu **fail-fast**: dừng lại NGAY tại lỗi ĐẦU TIÊN gặp phải,
theo THỨ TỰ kiểm (`ra` trước, `rb` sau). Không giống `mapResult` (chỉ
xử lý MỘT `Result`), `map2` GHÉP HAI `Result` ĐỘC LẬP thành MỘT kết
quả — đúng vấn đề bài trước đã nêu, giờ đóng gói thành một hàm dùng
lại được, không cần viết `if/else` tay mỗi lần.
::::

::::example{#map2-thay-the-ghep-tay}
So sánh TRỰC TIẾP: `map2` thay thế đúng đoạn `if/else` cồng kềnh bài
trước đã viết tay:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2<A, B, C, E>(ra: Result<A, E>, rb: Result<B, E>, f: (a: A, b: B) => C): Result<C, E> {
  if (ra.kind === "loi") return loi(ra.loi);
  if (rb.kind === "loi") return loi(rb.loi);
  return ok(f(ra.giaTri, rb.giaTri));
}

function kiemTen(ten: string): Result<string, string> {
  return ten.length > 0 ? ok(ten) : loi("tên không được rỗng");
}
function kiemTuoi(tuoi: number): Result<number, string> {
  return tuoi >= 0 && tuoi <= 150 ? ok(tuoi) : loi("tuổi phải trong khoảng 0-150");
}

const ketQua = map2(kiemTen("An"), kiemTuoi(30), (ten, tuoi) => ten + ", " + tuoi + " tuổi");
console.log(ketQua);
```

```text title=readonly
{"kind":"ok","giaTri":"An, 30 tuổi"}
```

MỘT lời gọi `map2` thay thế TOÀN BỘ khối `if/else if/else` bài 21 đã
viết tay — cùng logic (fail-fast trên hai `Result` độc lập), nhưng
đóng gói lại, dùng LẠI được cho BẤT KỲ cặp `Result` nào, không cần
viết lại `if/else` mỗi lần cần ghép.
::::

::::predict{#doan-map2-goi-mot-lan commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2<A, B, C, E>(ra: Result<A, E>, rb: Result<B, E>, f: (a: A, b: B) => C): Result<C, E> {
  if (ra.kind === "loi") return loi(ra.loi);
  if (rb.kind === "loi") return loi(rb.loi);
  return ok(f(ra.giaTri, rb.giaTri));
}

let soLanGoiF = 0;
const ra: Result<number, string> = loi("lỗi ra");
const rb: Result<number, string> = ok(5);

const ketQua = map2(ra, rb, (a, b) => {
  soLanGoiF = soLanGoiF + 1;
  return a + b;
});

console.log(soLanGoiF);
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì `map2` LUÔN gọi `f` một lần để có kết quả CUỐI CÙNG, sau đó
mới quyết định trả về `ok` hay `loi`
::why
Gần đúng ở việc bạn nghĩ tới cách `map2` "tính trước rồi mới quyết
định" — một luồng hoạt động hợp lý cho một số hàm khác (không phải
`map2`).

Chỗ lệch: đọc lại thân hàm — `if (ra.kind === "loi") return loi(ra.loi);`
là DÒNG ĐẦU TIÊN, và `ra` Ở ĐÂY thật sự là `loi("lỗi ra")`. Hàm TRẢ VỀ
NGAY tại dòng đó — các dòng SAU (kể cả `f(ra.giaTri, rb.giaTri)`)
KHÔNG BAO GIỜ chạy tới. `f` không hề được gọi, `soLanGoiF` giữ nguyên
`0`.
::
:::

:::opt
Máy báo lỗi biên dịch — `ra.giaTri` không tồn tại được truy cập trong
hàm `f` khi `ra` thật sự là một `loi`
::why
Gần đúng ở việc bạn để ý `ra.giaTri` CHỈ tồn tại trên biến thể `"ok"`
— quan sát về CẤU TRÚC kiểu đó đúng (T4.3 đã dạy).

Chỗ lệch: bên trong thân `map2`, TypeScript đã NARROW `ra` xuống
`{kind: "ok"; giaTri: A}` tại dòng `return ok(f(ra.giaTri,
rb.giaTri))` (vì hai `if` TRƯỚC ĐÓ đã LOẠI khả năng `ra`/`rb` là
`"loi"`) — truy cập `ra.giaTri` ở ĐÓ hoàn toàn hợp lệ, không lỗi biên
dịch. Vấn đề của bài predict này là hàm CÓ CHẠY TỚI dòng đó hay không
(không, như đã giải thích ở đáp án đúng), không phải lỗi kiểu.
::
:::
::::

::::code{#map2}
Tự viết `map2<A, B, C, E>(ra, rb, f): Result<C, E>` — fail-fast trên
hai `Result` độc lập.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function map2<A, B, C, E>(
  ra: Result<A, E>,
  rb: Result<B, E>,
  f: (a: A, b: B) => C
): Result<C, E> {
  if (ra.kind === "loi") return ___;
  if (rb.kind === "loi") return ___;
  return ___;
}

console.log(map2(ok<number, string>(3), ok<number, string>(4), (a, b) => a + b));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function map2<A, B, C, E>(
  ra: Result<A, E>,
  rb: Result<B, E>,
  f: (a: A, b: B) => C
): Result<C, E> {
  if (ra.kind === "loi") return loi(ra.loi);
  if (rb.kind === "loi") return loi(rb.loi);
  return ok(f(ra.giaTri, rb.giaTri));
}

console.log(map2(ok<number, string>(3), ok<number, string>(4), (a, b) => a + b));
```

```typescript title=test
const a = map2(ok<number, string>(3), ok<number, string>(4), (x, y) => x + y);
if (a.kind !== "ok") throw new Error("cả hai ok thì kết quả phải là ok");
if (a.kind === "ok" && a.giaTri !== 7) throw new Error("map2 phải ra 7");
const b = map2(loi<number, string>("lỗi a"), ok<number, string>(4), (x, y) => x + y);
if (b.kind !== "loi") throw new Error("ra lỗi thì kết quả phải là loi");
if (b.kind === "loi" && b.loi !== "lỗi a") throw new Error("phải báo đúng lỗi của ra");
const c = map2(ok<number, string>(3), loi<number, string>("lỗi b"), (x, y) => x + y);
if (c.kind !== "loi") throw new Error("rb lỗi thì kết quả phải là loi");
if (c.kind === "loi" && c.loi !== "lỗi b") throw new Error("phải báo đúng lỗi của rb");
```

:::hints
- kind: attention
  body: "Ba chỗ trống, ba trường hợp: ra lỗi (trả loi(ra.loi)), rb lỗi (trả loi(rb.loi)), cả hai ok (gọi f rồi bọc ok(...))."
- kind: strategy
  body: 'if (ra.kind === "loi") return loi(ra.loi); if (rb.kind === "loi") return loi(rb.loi); return ok(f(ra.giaTri, rb.giaTri)); — kiểm THEO THỨ TỰ ra rồi rb.'
- kind: one-line
  body: "return loi(ra.loi);  /  return loi(rb.loi);  /  return ok(f(ra.giaTri, rb.giaTri));"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "7"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`map2` ghép được hai `Result` độc lập bằng MỘT hàm — không cần viết
`if/else` tay mỗi lần cần ghép.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`map2` dừng NGAY ở lỗi ĐẦU TIÊN — nếu CẢ HAI `ra` VÀ `rb` đều lỗi, chỉ
MỘT lỗi được báo, lỗi kia "biến mất". Với một FORM có nhiều trường sai
CÙNG LÚC, người dùng có muốn biết TẤT CẢ lỗi trong MỘT lần không, thay
vì sửa từng lỗi một, chạy lại, thấy lỗi tiếp theo?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
