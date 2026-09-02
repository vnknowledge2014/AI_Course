---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.array-map-cung-la-cung-khuon
title: "`Array.prototype.map` CŨNG là cùng khuôn — bạn đã dùng nó từ T4.2"
summary: "[1,2,3].map(x => x*2) — nếu mảng CÓ phần tử, áp dụng f lên TỪNG phần tử, giữ nguyên 'vỏ' (vẫn là một mảng, cùng số lượng); mảng RỖNG map ra mảng RỖNG. CÙNG khuôn mapOption/mapResult."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 16
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [alg.array-map-same-pattern]
requires: [alg.map-result-handwritten]
concepts: [alg.array-map-same-pattern]
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
Bạn vừa viết `mapOption`/`mapResult` bằng tay. Nhưng `.map()` trên
MẢNG — dùng từ T4.2 — bạn CHƯA từng tự viết. Nó có cùng khuôn không?
::::

::::explain{#array-map-nhin-lai}
```typescript
console.log([1, 2, 3].map((x) => x * 2));
console.log(([] as number[]).map((x) => x * 2));
```

```text
[2,4,6]
[]
```

Nhìn LẠI `.map()` bằng đúng NGÔN NGỮ vừa học ở `mapOption`/`mapResult`:

- Mảng CÓ phần tử (`[1,2,3]`) — áp dụng `f` lên TỪNG phần tử BÊN
  TRONG, giữ nguyên "vỏ" (kết quả VẪN là một mảng, CÙNG số lượng phần
  tử: 3 vào, 3 ra).
- Mảng RỖNG (`[]`) — "không có gì bên trong" để áp dụng `f` lên — kết
  quả VẪN là mảng RỖNG, giữ nguyên "vỏ" (không mảng khác, không lỗi,
  `f` không hề được gọi lần nào).

Đây CHÍNH XÁC là khuôn `mapOption`/`mapResult` đã viết: "áp dụng `f`
lên giá trị BÊN TRONG (nếu có), giữ nguyên HÌNH DẠNG bên ngoài". Chỉ
khác: `Option`/`Result` có "bên trong" là ĐÚNG MỘT giá trị (hoặc không
có gì); mảng có "bên trong" là NHIỀU giá trị (hoặc không có gì).
::::

::::example{#ba-map-canh-nhau}
Đặt BA hàm `map` cạnh nhau — hình dạng CHỮ KÝ giống hệt nhau:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}
function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok": return ok(f(r.giaTri));
    case "loi": return loi(r.loi);
  }
}

console.log(mapOption(co(5), (x) => x + 1));
console.log(mapResult(ok<number, string>(5), (x) => x + 1));
console.log([5].map((x) => x + 1));
```

```text title=readonly
{"kind":"co","giaTri":6}
{"kind":"ok","giaTri":6}
[6]
```

CÙNG một hàm `(x) => x + 1`, áp dụng qua BA "hộp chứa" khác nhau
(`Option`, `Result`, `Array`) — CÙNG khuôn hoạt động: giá trị `5` bên
trong biến thành `6`, "vỏ" (`Option`, `Result`, hay `Array`) không
đổi hình dạng.
::::

::::predict{#doan-array-map-rong commitOnce}
```typescript
const dsRong: number[] = [];
const ketQua = dsRong.map((x) => {
  console.log("đang xử lý: " + x);
  return x * 2;
});

console.log(ketQua.length);
```

Chương trình này in ra MẤY dòng, và dòng CUỐI là gì?

:::opt{correct}
Một dòng DUY NHẤT: `0`
:::

:::opt
`đang xử lý: undefined` rồi `0` — vì mảng rỗng vẫn khiến `.map()` gọi
`f` MỘT LẦN với giá trị `undefined`
::why
Gần đúng ở việc bạn nghĩ tới trường hợp mảng rỗng có thể "vẫn chạy một
lần với giá trị trống" — một trực giác dễ hiểu nếu quen với các ngôn
ngữ có vòng lặp "chạy ít nhất một lần".

Chỗ lệch: `.map()` KHÔNG BAO GIỜ gọi hàm khi mảng RỖNG — nó chỉ lặp
qua các phần tử THẬT SỰ có mặt, và mảng rỗng KHÔNG có phần tử nào.
Không có dòng `"đang xử lý: ..."` nào được in ra — CHỈ có dòng
`console.log(ketQua.length)` chạy, in ra `0`.
::
:::

:::opt
Máy báo lỗi — `.map()` không hoạt động trên một mảng được khai TƯỜNG
MINH kiểu `number[]` nhưng lại RỖNG lúc khởi tạo
::why
Gần đúng ở việc bạn cân nhắc việc khai kiểu `number[]` cho một mảng
rỗng có thể gây xung đột gì đó — một mối lo hợp lý khi mới thấy cú
pháp `const dsRong: number[] = []`.

Chỗ lệch: khai kiểu TƯỜNG MINH cho một mảng RỖNG là cách viết HOÀN
TOÀN BÌNH THƯỜNG (thường CẦN THIẾT, vì TypeScript không suy luận được
kiểu phần tử từ một mảng rỗng không annotate) — `.map()` hoạt động
đúng như với bất kỳ mảng `number[]` nào khác.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Array.prototype.map` không phải một công cụ RIÊNG của mảng — nó là
MỘT trong ba `map` bạn đã thấy, cùng chung một khuôn với `Option`,
`Result`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Option`, `Result`, `Array` — ba kiểu dữ liệu khác hẳn nhau, CÙNG có
một `map` cùng hình dạng. Khuôn CHUNG đó có tên gọi hình thức không?

Bài sau đặt tên nó.
::::

::::checkpoint{mastery=0.8}
::::
