---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.viet-map-rieng-cho-option
title: "Viết `map` RIÊNG cho `Option`"
summary: "function mapOption<T,U>(o: Option<T>, f: (x: T) => U): Option<U> — nếu Option 'có', áp dụng f lên giá trị BÊN TRONG, giữ nguyên vỏ Option; nếu 'không', giữ nguyên 'không', KHÔNG gọi f."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.map-option-handwritten]
requires: [alg.review-option-result]
concepts: [alg.map-option-handwritten]
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
Cụm 2 chốt lại: bạn có `Option`/`Result`, biết dùng đúng lúc. Cụm 3
hỏi: nếu muốn BIẾN ĐỔI giá trị BÊN TRONG một `Option`, phải viết
`switch` mỗi lần sao?
::::

::::explain{#map-cho-option}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co":
      return co(f(o.giaTri));
    case "khong":
      return khong();
  }
}

console.log(mapOption(co(5), (x) => x * 2));
console.log(mapOption(khong<number>(), (x: number) => x * 2));
```

```text
{"kind":"co","giaTri":10}
{"kind":"khong"}
```

`mapOption(o, f)` — nếu `o` "có" giá trị, áp dụng `f` lên giá trị ĐÓ,
đóng gói kết quả LẠI vào `co(...)` (vẫn là một `Option`, không "mở
vỏ" ra ngoài). Nếu `o` "không" có gì — GIỮ NGUYÊN "không", `f` KHÔNG
BAO GIỜ được gọi (không có giá trị nào để áp dụng lên).

`mapOption(co(5), x => x*2)` ra `co(10)` — vẫn LÀ một `Option`, không
phải `10` trần trụi. `mapOption(khong(), x => x*2)` ra `khong()` —
`f` không hề chạy, không có lỗi "gọi hàm với giá trị không tồn tại".
::::

::::example{#chuoi-nhieu-map}
Gọi `mapOption` NHIỀU LẦN liên tiếp, biến đổi qua từng bước:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co":
      return co(f(o.giaTri));
    case "khong":
      return khong();
  }
}

const b1 = mapOption(co(5), (x) => x * 2);
const b2 = mapOption(b1, (x) => x + 1);
const b3 = mapOption(b2, (x) => "kết quả: " + x);

console.log(b3);
```

```text title=readonly
{"kind":"co","giaTri":"kết quả: 11"}
```

`5` → `10` (nhân đôi) → `11` (cộng 1) → `"kết quả: 11"` (chuyển thành
chuỗi) — MỖI bước `mapOption` giữ NGUYÊN vỏ `Option`, chỉ biến đổi
giá trị BÊN TRONG. Kiểu của giá trị bên trong ĐỔI qua từng bước
(`number` → `number` → `string`), nhưng "hình dạng" bên ngoài
(`Option<...>`) không đổi.
::::

::::predict{#doan-map-option-khong commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co":
      return co(f(o.giaTri));
    case "khong":
      return khong();
  }
}

let soLanGoi = 0;
const ketQua = mapOption(khong<number>(), (x) => {
  soLanGoi = soLanGoi + 1;
  return x * 2;
});

console.log(soLanGoi);
console.log(ketQua);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`0` rồi `{"kind":"khong"}`
:::

:::opt
`1` rồi `{"kind":"khong"}` — vì `mapOption` LUÔN gọi `f` một lần để
"thử" trước khi quyết định trả về gì
::why
Gần đúng ở việc bạn đúng dòng cuối (`ketQua` là `khong()`) — kết quả
CUỐI CÙNG đúng.

Chỗ lệch: `mapOption` KHÔNG "thử gọi f rồi bỏ" — nhìn lại thân hàm:
nhánh `case "khong"` trả về `khong()` NGAY, KHÔNG hề chạm tới `f` ở bất
kỳ đâu. `f` (tăng `soLanGoi`) CHỈ được gọi trong nhánh `case "co"`. Với
`o` là `khong()`, nhánh đó KHÔNG BAO GIỜ chạy — `soLanGoi` giữ nguyên
`0`.
::
:::

:::opt
Máy báo lỗi — không thể gọi `mapOption` với `khong<number>()` khi hàm
`f` có tác dụng phụ (thay đổi `soLanGoi`)
::why
Gần đúng ở việc bạn để ý hàm `f` ở đây KHÔNG THUẦN (có side effect,
sửa `soLanGoi` bên ngoài) — quan sát đó đúng, đây LÀ một hàm không
thuần (T4.1 đã dạy khái niệm này).

Chỗ lệch: `mapOption` không hề CẤM hay YÊU CẦU `f` phải thuần — nó chỉ
GỌI `f` (hoặc KHÔNG gọi) tuỳ biến thể của `o`. Không có kiểm tra hay
lỗi nào liên quan tới việc `f` có side effect hay không.
::
:::
::::

::::code{#map_option}
Tự viết `mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U>`.

```typescript title=starter
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co":
      return ___;
    case "khong":
      return khong();
  }
}

console.log(mapOption(co(5), (x) => x * 2));
```

```typescript title=solution
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co":
      return co(f(o.giaTri));
    case "khong":
      return khong();
  }
}

console.log(mapOption(co(5), (x) => x * 2));
```

```typescript title=test
const a = mapOption(co(5), (x) => x * 2);
if (a.kind !== "co") throw new Error("map trên co phải giữ kind là \"co\"");
if (a.kind === "co" && a.giaTri !== 10) throw new Error("mapOption(co(5), x*2) phải ra 10");
const b = mapOption(khong<number>(), (x: number) => x * 2);
if (b.kind !== "khong") throw new Error("map trên khong phải giữ nguyên khong, không gọi f");
const c = mapOption(co(0), (x) => x + 1);
if (c.kind !== "co" || (c.kind === "co" && c.giaTri !== 1)) throw new Error("map phải hoạt động đúng với giá trị falsy như 0");
```

:::hints
- kind: attention
  body: "Chỗ trống là biểu thức của nhánh case \"co\" — phải áp dụng f lên o.giaTri, RỒI đóng gói kết quả lại bằng co(...), không trả về giá trị trần."
- kind: strategy
  body: 'return co(f(o.giaTri)) — f(o.giaTri) tính giá trị MỚI, co(...) đóng gói nó lại thành một Option.'
- kind: one-line
  body: "return co(f(o.giaTri));"
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
`mapOption` biến đổi giá trị BÊN TRONG, giữ NGUYÊN vỏ `Option`. Không
gọi `f` khi "không có gì" — an toàn tuyệt đối.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa viết `mapOption`. `Result<T, E>` có CÙNG hình dạng `Option<T>`
— viết `mapResult` có khác gì không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
