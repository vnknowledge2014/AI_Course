---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.luat-functor-hai-ghep-hai-map-bang-mot-map-ghep
title: "Luật Functor 2: ghép hai `map` bằng MỘT `map` đã ghép hàm"
summary: "mapOption(mapOption(o, f), g) phải ra CÙNG kết quả với mapOption(o, compose(g, f)). Nối thẳng T4.2's compose: gọi map hai lần liên tiếp TƯƠNG ĐƯƠNG gọi map MỘT lần với hai hàm đã GHÉP LẠI trước."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 19
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.functor-law-composition]
requires: [alg.functor-law-identity]
concepts: [alg.functor-law-composition]
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
Luật 1 kiểm `map` với MỘT hàm (`id`). Hôm nay: luật thứ hai — kiểm khi
GHÉP nhiều hàm, nối thẳng T4.2's `compose`.
::::

::::explain{#luat-composition}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}

function compose<A, B, C>(g: (b: B) => C, f: (a: A) => B): (a: A) => C {
  return (a) => g(f(a));
}

const nhanDoi = (x: number) => x * 2;
const congMot = (x: number) => x + 1;

const a = mapOption(mapOption(co(5), nhanDoi), congMot);
const b = mapOption(co(5), compose(congMot, nhanDoi));

console.log(a);
console.log(b);
```

```text
{"kind":"co","giaTri":11}
{"kind":"co","giaTri":11}
```

`a` gọi `mapOption` HAI LẦN LIÊN TIẾP: `nhanDoi` trước (`5 → 10`), rồi
`congMot` (`10 → 11`). `b` gọi `mapOption` MỘT LẦN, với `compose(congMot,
nhanDoi)` — một hàm ĐÃ GHÉP SẴN cả hai (T4.2's `compose`, chạy `nhanDoi`
trước, `congMot` sau — đúng thứ tự phải-sang-trái đã học). CẢ HAI cách
ra CÙNG kết quả (`co(11)`) — **luật Functor thứ hai**: gọi `map` nhiều
lần TƯƠNG ĐƯƠNG gọi `map` MỘT lần với các hàm đã ghép lại trước.

Luật này đảm bảo `map` "đáng tin": không có hiệu ứng lạ khi ghép nhiều
lần liên tiếp, kết quả LUÔN dự đoán được từ CÁC HÀM đang áp dụng, không
phụ thuộc "gọi map bao nhiêu lần" theo cách nào khác.
::::

::::example{#luat-hai-dung-voi-ba-ham}
Luật NÀY mở rộng được cho BA hàm (không chỉ hai), ghép LẦN LƯỢT:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}

const nhanDoi = (x: number) => x * 2;
const congMot = (x: number) => x + 1;
const binhPhuong = (x: number) => x * x;

const bangBaMap = mapOption(mapOption(mapOption(co(3), nhanDoi), congMot), binhPhuong);
const bangMotMap = mapOption(co(3), (x) => binhPhuong(congMot(nhanDoi(x))));

console.log(bangBaMap);
console.log(bangMotMap);
```

```text title=readonly
{"kind":"co","giaTri":49}
{"kind":"co","giaTri":49}
```

`3 → 6 (nhanDoi) → 7 (congMot) → 49 (binhPhuong)` — CÙNG kết quả, dù
gọi `mapOption` BA LẦN liên tiếp hay MỘT LẦN với ba hàm đã ghép sẵn
(viết tay bằng hàm lồng, không cần `compose` chính thức ở đây). Luật
composition không giới hạn ở HAI hàm — áp dụng được với BAO NHIÊU hàm
ghép nối tiếp cũng đúng.
::::

::::predict{#doan-luat-composition commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}

const tru3 = (x: number) => x - 3;
const nhan5 = (x: number) => x * 5;

const cach1 = mapOption(mapOption(khong<number>(), tru3), nhan5);
const cach2 = mapOption(khong<number>(), (x) => nhan5(tru3(x)));

console.log(cach1);
console.log(cach2);
console.log(JSON.stringify(cach1) === JSON.stringify(cach2));
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
`false` — vì luật composition CHỈ đúng khi `Option` "có" giá trị, với
`khong()` cả hai cách sẽ cho ra kết quả KHÁC nhau
::why
Gần đúng ở việc bạn cân nhắc `khong()` có thể là TRƯỜNG HỢP ĐẶC BIỆT
cần xét riêng — một cảnh giác hợp lý khi kiểm luật trên nhiều tình
huống.

Chỗ lệch: luật composition đúng với CẢ hai biến thể — với `khong()`,
CẢ HAI cách (`mapOption` hai lần hay một lần với hàm ghép) đều KHÔNG
gọi hàm nào, đều trả về `khong()` NGUYÊN VẸN, giống hệt nhau. Không có
"trường hợp đặc biệt" nào phá luật.
::
:::

:::opt
Máy báo lỗi — không so sánh được hai `Option<number>` bằng
`JSON.stringify(...) === JSON.stringify(...)`
::why
Gần đúng ở việc bạn cân nhắc CÁCH SO SÁNH hai object có đáng tin không
— một câu hỏi hợp lý (object trong JavaScript không so sánh được bằng
`===` trực tiếp).

Chỗ lệch: `JSON.stringify(...)` CHUYỂN object thành CHUỖI trước khi so
sánh — hai CHUỖI so sánh được bằng `===` bình thường, không lỗi gì.
Đây CHÍNH LÀ cách kiểm hai `Option` "bằng nhau về nội dung" mà không
cần viết hàm so sánh riêng.
::
:::
::::

::::code{#kiem_luat_ghep}
Viết `ghepHaiMap` — kiểm luật composition CÓ ĐÚNG với một `Option` và
hai hàm bất kỳ hay không.

```typescript title=starter
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}
function compose<A, B, C>(g: (b: B) => C, f: (a: A) => B): (a: A) => C {
  return (a) => g(f(a));
}

function ghepHaiMap(o: Option<number>, f: (x: number) => number, g: (x: number) => number): boolean {
  const cach1 = mapOption(mapOption(o, f), g);
  const cach2 = ___;
  return JSON.stringify(cach1) === JSON.stringify(cach2);
}

console.log(ghepHaiMap(co(5), (x) => x * 2, (x) => x + 1));
```

```typescript title=solution
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}
function compose<A, B, C>(g: (b: B) => C, f: (a: A) => B): (a: A) => C {
  return (a) => g(f(a));
}

function ghepHaiMap(o: Option<number>, f: (x: number) => number, g: (x: number) => number): boolean {
  const cach1 = mapOption(mapOption(o, f), g);
  const cach2 = mapOption(o, compose(g, f));
  return JSON.stringify(cach1) === JSON.stringify(cach2);
}

console.log(ghepHaiMap(co(5), (x) => x * 2, (x) => x + 1));
```

```typescript title=test
if (ghepHaiMap(co(5), (x) => x * 2, (x) => x + 1) !== true) throw new Error("luật ghép phải đúng với co(5)");
if (ghepHaiMap(khong(), (x) => x * 2, (x) => x + 1) !== true) throw new Error("luật ghép phải đúng với khong()");
if (ghepHaiMap(co(10), (x) => x - 3, (x) => x * x) !== true) throw new Error("luật ghép phải đúng với hàm khác");
```

:::hints
- kind: attention
  body: "cach2 phải áp dụng MỘT lần mapOption, với ĐÚNG hàm đã compose(g, f) — không phải compose(f, g) (thứ tự đảo ngược sẽ SAI, vì compose chạy phải-sang-trái)."
- kind: strategy
  body: 'cach2 = mapOption(o, compose(g, f)) — compose(g, f) chạy f TRƯỚC (giống mapOption(o,f) làm ở cach1), rồi g SAU (giống bước mapOption thứ hai của cach1).'
- kind: one-line
  body: "const cach2 = mapOption(o, compose(g, f));"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai luật Functor — identity và composition — cùng đảm bảo `map` "đáng
tin", không có hiệu ứng lạ dù ghép bao nhiêu lần, gọi bao nhiêu bước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã hiểu `Functor` đủ sâu — định nghĩa, hai luật. Bài sau chốt cụm:
tự viết một `map` MỚI, tự kiểm nó tuân luật, không chỉ đọc ví dụ có
sẵn.
::::

::::checkpoint{mastery=0.8}
::::
