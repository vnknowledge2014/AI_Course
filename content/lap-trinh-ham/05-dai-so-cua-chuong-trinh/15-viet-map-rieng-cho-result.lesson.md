---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.viet-map-rieng-cho-result
title: "Viết `map` RIÊNG cho `Result` — CÙNG khuôn"
summary: "function mapResult<T,U,E>(r: Result<T,E>, f: (x: T) => U): Result<U,E> — CÙNG cấu trúc hệt mapOption (nếu 'thành công', áp dụng f bên trong, giữ vỏ; nếu 'lỗi', giữ nguyên lỗi) — chỉ khác TÊN biến thể."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 15
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.map-result-handwritten]
requires: [alg.map-option-handwritten]
concepts: [alg.map-result-handwritten]
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
`mapOption` viết xong. `Result<T, E>` có CÙNG hình dạng `Option<T>` —
`mapResult` có khác gì không?
::::

::::explain{#map-cho-result}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok":
      return ok(f(r.giaTri));
    case "loi":
      return loi(r.loi);
  }
}

console.log(mapResult(ok<number, string>(5), (x) => x * 2));
console.log(mapResult(loi<number, string>("thất bại"), (x: number) => x * 2));
```

```text
{"kind":"ok","giaTri":10}
{"kind":"loi","loi":"thất bại"}
```

Đặt `mapResult` cạnh `mapOption` (bài 14) — CÙNG cấu trúc HỆT nhau,
chỉ khác TÊN biến thể (`co`/`khong` → `ok`/`loi`) và `Result` mang
thêm `E` (kiểu lỗi). `mapResult(ok(5), x*2)` ra `ok(10)` — áp dụng `f`
lên giá trị THÀNH CÔNG, giữ vỏ `Result`. `mapResult(loi("thất bại"),
x*2)` ra `loi("thất bại")` NGUYÊN VẸN — `f` KHÔNG chạy, lỗi được GIỮ
LẠI y hệt, không bị đổi.
::::

::::example{#loi-di-xuyen-qua-nhieu-map}
Một LỖI, đi XUYÊN QUA nhiều lần `mapResult` liên tiếp mà KHÔNG đổi:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok":
      return ok(f(r.giaTri));
    case "loi":
      return loi(r.loi);
  }
}

const batDau: Result<number, string> = loi("dữ liệu không hợp lệ");

const b1 = mapResult(batDau, (x) => x * 2);
const b2 = mapResult(b1, (x) => x + 1);
const b3 = mapResult(b2, (x) => "kết quả: " + x);

console.log(b3);
```

```text title=readonly
{"kind":"loi","loi":"dữ liệu không hợp lệ"}
```

`batDau` ĐÃ là một lỗi — BA lần `mapResult` liên tiếp đều KHÔNG gọi
`f` (không có gì để áp dụng lên), lỗi `"dữ liệu không hợp lệ"` ĐI
XUYÊN QUA nguyên vẹn, không bị thay đổi hay "mất dấu" ở bất kỳ bước
nào. Đây là lợi ích THẬT: ghép nhiều `mapResult`, không cần kiểm tra
lỗi SAU MỖI bước — lỗi tự "trôi" qua, không cần code kiểm tra riêng.
::::

::::predict{#doan-map-result-nhieu-buoc commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok":
      return ok(f(r.giaTri));
    case "loi":
      return loi(r.loi);
  }
}

const a: Result<number, string> = ok(3);
const b1 = mapResult(a, (x) => x + 1);
const b2 = mapResult(b1, (x) => x * x);

console.log(b2);
```

Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"ok","giaTri":16}`
:::

:::opt
`{"kind":"ok","giaTri":10}` — vì `mapResult` cộng dồn TẤT CẢ các phép
biến đổi lại rồi mới áp dụng, không áp dụng TUẦN TỰ từng bước
::why
Gần đúng ở việc bạn tính TỔNG các phép biến đổi theo một cách nào đó
— có nỗ lực tính toán đúng hướng.

Chỗ lệch: mỗi `mapResult` áp dụng NGAY LẬP TỨC, TUẦN TỰ — `b1 =
mapResult(a, x => x+1)` tính XONG `3 + 1 = 4` TRƯỚC, `b1` LÀ `ok(4)`
(một giá trị CỤ THỂ, không phải một "phép tính hoãn lại"). `b2 =
mapResult(b1, x => x*x)` tính TIẾP `4 * 4 = 16` trên GIÁ TRỊ ĐÃ CÓ của
`b1`, không phải áp dụng cả hai hàm dồn một lượt lên `3`.
::
:::

:::opt
Máy báo lỗi biên dịch — `mapResult` không dùng được HAI LẦN LIÊN TIẾP
trên CÙNG một chuỗi biến đổi
::why
Gần đúng ở việc bạn cân nhắc GIỚI HẠN có thể có khi gọi lặp lại một
hàm nhiều lần — một mối lo hợp lý nếu chưa chắc `mapResult` hoạt động
thế nào khi ghép chuỗi.

Chỗ lệch: `mapResult` là một hàm BÌNH THƯỜNG, gọi được BAO NHIÊU LẦN
LIÊN TIẾP tuỳ ý (đúng ví dụ ba lần liên tiếp ở khối `example`) — không
có giới hạn nào về SỐ LẦN gọi, biên dịch và chạy hoàn toàn bình
thường.
::
:::
::::

::::code{#map_result}
Tự viết `mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E>`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok":
      return ___;
    case "loi":
      return ___;
  }
}

console.log(mapResult(ok<number, string>(5), (x) => x * 2));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok":
      return ok(f(r.giaTri));
    case "loi":
      return loi(r.loi);
  }
}

console.log(mapResult(ok<number, string>(5), (x) => x * 2));
```

```typescript title=test
const a = mapResult(ok<number, string>(5), (x) => x * 2);
if (a.kind !== "ok") throw new Error("map trên ok phải giữ kind là \"ok\"");
if (a.kind === "ok" && a.giaTri !== 10) throw new Error("mapResult(ok(5), x*2) phải ra 10");
const b = mapResult(loi<number, string>("thất bại"), (x: number) => x * 2);
if (b.kind !== "loi") throw new Error("map trên loi phải giữ nguyên loi, không gọi f");
if (b.kind === "loi" && b.loi !== "thất bại") throw new Error("loi phải giữ nguyên thông điệp gốc");
```

:::hints
- kind: attention
  body: "Nhánh \"ok\": áp dụng f lên r.giaTri, đóng gói lại bằng ok(...). Nhánh \"loi\": KHÔNG gọi f — chỉ đóng gói LẠI r.loi (giữ nguyên, không đổi) bằng loi(...)."
- kind: strategy
  body: 'case "ok": return ok(f(r.giaTri)). case "loi": return loi(r.loi) — r.loi giữ nguyên, KHÔNG áp dụng f lên nó.'
- kind: one-line
  body: "case \"ok\": return ok(f(r.giaTri));  case \"loi\": return loi(r.loi);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "10"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`mapOption`, `mapResult` — cùng khuôn, khác tên biến thể. Không phải
trùng hợp — bài sau cho THẤY khuôn này rộng hơn cả hai kiểu này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã dùng `Array.prototype.map` từ T4.2. Nó có CÙNG khuôn
`mapOption`/`mapResult` không — hay là một thứ hoàn toàn khác?

Bài sau nhìn lại `Array.map` với con mắt mới.
::::

::::checkpoint{mastery=0.8}
::::
