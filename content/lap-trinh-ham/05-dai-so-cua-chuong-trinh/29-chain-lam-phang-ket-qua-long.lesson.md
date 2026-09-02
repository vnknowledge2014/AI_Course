---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.chain-lam-phang-ket-qua-long
title: "`chain` — LÀM PHẲNG kết quả lồng"
summary: "function chainOption<T,U>(o, f): Option<U> { switch(o.kind) { case \"co\": return f(o.giaTri); case \"khong\": return khong(); } } — chain TRẢ THẲNG kết quả của f (đã LÀ Option rồi), không có lớp lồng nào."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 29
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.chain-flattens]
requires: [alg.map-insufficient-nested]
concepts: [alg.chain-flattens]
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
`map` bọc kết quả của `f` vào một lớp MỚI, dù `f` đã tự trả về một
`Option`. Hôm nay: một công cụ KHÔNG bọc — trả THẲNG kết quả đó.
::::

::::explain{#chain-tra-thang}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}

const chia = (a: number, b: number): Option<number> => (b === 0 ? khong() : co(a / b));

console.log(JSON.stringify(chainOption(co(10), (x) => chia(x, 2))));
```

```text
{"kind":"co","giaTri":5}
```

So sánh với `mapOption` (bài 14): `case "co": return co(f(o.giaTri))`
— BỌC kết quả của `f` vào MỘT `co(...)` MỚI. `chainOption` thì
`case "co": return f(o.giaTri)` — TRẢ THẲNG kết quả của `f`, KHÔNG
bọc gì thêm cả. Vì `f` (ở đây là `x => chia(x, 2)`) TỰ NÓ ĐÃ trả về
một `Option<number>` (`chia` trả `Option`), trả THẲNG kết quả đó
nghĩa là KHÔNG có lớp lồng nào — `chainOption(co(10), x => chia(x,
2))` ra ĐÚNG `co(5)`, PHẲNG, không phải `co(co(5))` như `mapOption`
đã tạo ra ở bài 28.

Chỉ MỘT chữ khác trong thân hàm (`f(o.giaTri)` thay vì
`co(f(o.giaTri))`) — nhưng đủ để tránh HOÀN TOÀN vấn đề lồng nhau.
::::

::::example{#chain-tren-khong-giu-nguyen}
`chainOption` trên `khong()` — giữ nguyên `khong()`, `f` KHÔNG BAO GIỜ
được gọi, đúng khuôn `mapOption` đã dạy:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}

const chia = (a: number, b: number): Option<number> => (b === 0 ? khong() : co(a / b));

let soLanGoi = 0;
const dem = (x: number): Option<number> => {
  soLanGoi = soLanGoi + 1;
  return chia(x, 2);
};

const ketQua = chainOption(khong<number>(), dem);
console.log(JSON.stringify(ketQua));
console.log(soLanGoi);
```

```text title=readonly
{"kind":"khong"}
0
```

`o = khong()` — nhánh `case "khong"` trả về `khong()` NGAY, `dem`
(đóng vai `f`) KHÔNG BAO GIỜ được gọi tới, `soLanGoi` giữ nguyên `0`.
Đúng an toàn `mapOption` (bài 14) đã có — `chainOption` giữ y hệt tính
chất đó, chỉ khác cách xử lý nhánh `"co"`.
::::

::::predict{#doan-chain-hai-buoc commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}

const chia = (a: number, b: number): Option<number> => (b === 0 ? khong() : co(a / b));

const buoc1 = chainOption(co(20), (x) => chia(x, 4));
const buoc2 = chainOption(buoc1, (x) => chia(x, 0));

console.log(buoc1.kind);
console.log(buoc2.kind);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`co` rồi `khong`
:::

:::opt
`co` rồi `co` — vì `buoc2` chỉ ÁP DỤNG PHÉP CHIA lần nữa lên kết quả
CỦA `buoc1`, không quan tâm đến việc chia cho `0`
::why
Gần đúng ở việc bạn tính đúng `buoc1` (`chia(20, 4) = 5`, một `Option`
"có") — kết quả TRUNG GIAN đó đúng.

Chỗ lệch: `buoc2 = chainOption(buoc1, x => chia(x, 0))` — hàm áp dụng
`x => chia(x, 0)` chia cho `0`, một phép chia THẤT BẠI (`chia` trả
`khong()` khi chia cho `0`, đã học từ bài 28). Vì `buoc1` LÀ "có"
(`co(5)`), `chainOption` GỌI `f(5)` — chính là `chia(5, 0)`, ra
`khong()`. `chainOption` TRẢ THẲNG kết quả đó — `buoc2.kind` là
`"khong"`, không phải `"co"`.
::
:::

:::opt
Máy báo lỗi biên dịch — `chainOption` không dùng được HAI LẦN LIÊN
TIẾP trên KẾT QUẢ của chính nó
::why
Gần đúng ở việc bạn cân nhắc GIỚI HẠN có thể có khi ghép `chainOption`
nhiều lần liên tiếp — một mối lo hợp lý khi mới thấy kỹ thuật này.

Chỗ lệch: `chainOption` là một hàm BÌNH THƯỜNG, dùng được LIÊN TIẾP
bao nhiêu lần tuỳ ý (đúng lý do CHÍNH nó tồn tại: để ghép được nhiều
bước phụ thuộc nhau mà không lồng nhau). `buoc2 = chainOption(buoc1,
...)` biên dịch và chạy hoàn toàn bình thường.
::
:::
::::

::::code{#chain_option}
Tự viết `chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>):
Option<U>`.

```typescript title=starter
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return ___;
    case "khong": return ___;
  }
}

const chia = (a: number, b: number): Option<number> => (b === 0 ? khong() : co(a / b));
console.log(JSON.stringify(chainOption(co(10), (x) => chia(x, 2))));
```

```typescript title=solution
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function chainOption<T, U>(o: Option<T>, f: (x: T) => Option<U>): Option<U> {
  switch (o.kind) {
    case "co": return f(o.giaTri);
    case "khong": return khong();
  }
}

const chia = (a: number, b: number): Option<number> => (b === 0 ? khong() : co(a / b));
console.log(JSON.stringify(chainOption(co(10), (x) => chia(x, 2))));
```

```typescript title=test
const a = chainOption(co(10), (x) => chia(x, 2));
if (a.kind !== "co") throw new Error("chain trên co phải trả về kind là \"co\"");
if (a.kind === "co" && a.giaTri !== 5) throw new Error("chain phải ra 5, PHẲNG, không lồng");

const b = chainOption(co(10), (x) => chia(x, 0));
if (b.kind !== "khong") throw new Error("chain phải LẤY THẲNG kết quả thật của f, không tự gán thêm lớp \"co\" bọc ngoài");

const c = chainOption(khong<number>(), (x) => chia(x, 2));
if (c.kind !== "khong") throw new Error("chain trên khong phải giữ nguyên khong, không gọi f");
```

:::hints
- kind: attention
  body: "Nhánh \"co\": TRẢ THẲNG f(o.giaTri) — KHÔNG bọc thêm co(...) như mapOption đã làm. Nhánh \"khong\": giữ nguyên, gọi khong()."
- kind: strategy
  body: 'case "co": return f(o.giaTri); — đúng CHỮ f(o.giaTri), không phải co(f(o.giaTri)). case "khong": return khong();'
- kind: one-line
  body: "case \"co\": return f(o.giaTri);\ncase \"khong\": return khong();"
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
`chain` trả THẲNG kết quả của `f`, không bọc thêm — làm phẳng kết quả
lồng mà `map` tạo ra khi `f` tự nó đã trả về một `Option`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Result<T, E>` có CÙNG hình dạng `Option<T>` — viết `chainResult` có
khác gì `chainOption` không?
::::

::::checkpoint{mastery=0.8}
::::
