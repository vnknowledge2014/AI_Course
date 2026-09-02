---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.viet-traverse-bang-vong-lap-co-short-circuit
title: "Viết `traverse` bằng vòng lặp — DỪNG NGAY khi gặp `\"khong\"`"
summary: "function traverse<T,U>(ds, f): Option<U[]> { const kq: U[] = []; for (const x of ds) { const o = f(x); if (o.kind === \"khong\") return khong(); kq.push(o.giaTri); } return co(kq); } — dừng SỚM tại phần tử lỗi đầu tiên, không cần biết phần tử sau."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 36
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.traverse-handwritten]
requires: [alg.traverse-motivation]
concepts: [alg.traverse-handwritten]
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
Công cụ LẬT `Array<Option<T>>` thành `Option<Array<T>>` tên là
`traverse`. Viết nó bằng một vòng lặp bình thường.
::::

::::explain{#viet-traverse}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

console.log(JSON.stringify(traverse(["1", "2", "3"], chuyenSo)));
```

```text
{"kind":"co","giaTri":[1,2,3]}
```

`traverse` nhận một mảng `ds: T[]` VÀ một hàm áp dụng `f: (x: T) =>
Option<U>` (đúng `chuyenSo`, biến `T = string`, `U = number`) — GIỐNG
`.map` ở CHỖ NÀY. Nhưng thay vì GOM kết quả thành `Option<U>[]` (như
`.map` bài 35), nó gom vào MỘT mảng TRẦN `kq: U[]` — CHỈ khi TẤT CẢ
`f(x)` đều `"co"`. Ngay khi GẶP một `"khong"`, `traverse` `return
khong()` NGAY LẬP TỨC — không đợi vòng lặp `for` chạy hết.
::::

::::example{#dung-som-khong-can-biet-phan-tu-sau}
Một phần tử LỖI Ở GIỮA — `traverse` dừng NGAY, phần tử SAU không hề
được `f` gọi tới:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}
function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

let soLanGoi = 0;
const chuyenSoCoDem = (vb: string): Option<number> => {
  soLanGoi = soLanGoi + 1;
  return chuyenSo(vb);
};

const ketQua = traverse(["1", "abc", "3"], chuyenSoCoDem);
console.log(JSON.stringify(ketQua));
console.log(soLanGoi);
```

```text title=readonly
{"kind":"khong"}
2
```

Mảng có BA phần tử (`"1"`, `"abc"`, `"3"`), nhưng `chuyenSoCoDem` chỉ
được gọi HAI LẦN — trên `"1"` (thành công, đẩy vào `kq`) và `"abc"`
(thất bại, `traverse` `return khong()` NGAY). `"3"` KHÔNG BAO GIỜ được
kiểm — không CẦN, vì kết quả CUỐI đã chắc chắn là `khong()` dù `"3"`
có hợp lệ hay không. Đúng bản chất fail-fast, nối thẳng `chain`'s
short-circuit (bài 31), giờ áp dụng cho một VÒNG LẶP trên mảng thay
vì một chuỗi bước cố định.
::::

::::predict{#doan-traverse-mang-rong commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}
function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

const ketQua = traverse<string, number>([], chuyenSo);
console.log(ketQua.kind);
console.log(ketQua.kind === "co" ? ketQua.giaTri.length : -1);
```

Hai dòng cuối in ra gì (`ds` là một mảng RỖNG)?

:::opt{correct}
`co` rồi `0`
:::

:::opt
`khong` rồi `-1` — vì mảng rỗng không có phần tử nào để kiểm, nên
`traverse` không thể xác nhận "mọi phần tử đều hợp lệ"
::why
Gần đúng ở việc bạn nghĩ tới một QUY TẮC "không có gì để kiểm thì
không thể khẳng định" — một trực giác hợp lý ở một số ngữ cảnh khác.

Chỗ lệch: vòng lặp `for (const x of ds)` trên mảng RỖNG đơn giản
KHÔNG CHẠY LẦN NÀO — thân vòng lặp (nơi có thể `return khong()`)
KHÔNG BAO GIỜ được thực thi. Luồng thực thi đi THẲNG tới dòng cuối
cùng: `return co(kq)`, với `kq` vẫn là mảng rỗng `[]` (chưa từng được
`push` gì). Kết quả là `co([])` — CÓ, một mảng rỗng, không phải
`khong()`.
::
:::

:::opt
Máy báo lỗi lúc chạy — gọi `traverse` với mảng rỗng khiến vòng lặp
`for` không có gì để lặp, gây lỗi thực thi
::why
Gần đúng ở việc bạn để ý mảng RỖNG khiến vòng lặp KHÔNG chạy lần nào —
quan sát về hành vi CỦA vòng lặp đó đúng.

Chỗ lệch: một vòng lặp `for...of` trên mảng rỗng là HOÀN TOÀN HỢP LỆ
trong JavaScript/TypeScript — nó chỉ đơn giản KHÔNG THỰC THI thân
vòng lặp, không hề gây lỗi hay ngoại lệ nào. Hàm tiếp tục chạy xuống
dòng SAU vòng lặp bình thường.
::
:::
::::

::::code{#viet_traverse}
Tự viết `traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]>`
bằng vòng lặp.

```typescript title=starter
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return ___;
    kq.push(___);
  }
  return ___;
}

function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

console.log(JSON.stringify(traverse(["1", "2", "3"], chuyenSo)));
```

```typescript title=solution
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

console.log(JSON.stringify(traverse(["1", "2", "3"], chuyenSo)));
```

```typescript title=test
const a = traverse(["1", "2", "3"], chuyenSo);
if (a.kind !== "co") throw new Error("mọi phần tử chuyển được phải ra co");
if (a.kind === "co" && JSON.stringify(a.giaTri) !== JSON.stringify([1, 2, 3])) throw new Error("kq phải là [1, 2, 3]");

const b = traverse(["1", "abc", "3"], chuyenSo);
if (b.kind !== "khong") throw new Error("một phần tử lỗi phải khiến cả kết quả là khong");

let soLanGoi = 0;
const chuyenSoCoDem = (vb: string): Option<number> => {
  soLanGoi = soLanGoi + 1;
  return chuyenSo(vb);
};
traverse(["1", "abc", "3"], chuyenSoCoDem);
if (soLanGoi !== 2) throw new Error("phải dừng NGAY tại phần tử lỗi, không kiểm phần tử sau nó");

const c = traverse<string, number>([], chuyenSo);
if (c.kind !== "co") throw new Error("mảng rỗng phải ra co([]), không phải khong");
if (c.kind === "co" && c.giaTri.length !== 0) throw new Error("mảng rỗng phải ra kq rỗng");
```

:::hints
- kind: attention
  body: "Chỗ trống 1 (khi o.kind là \"khong\"): trả khong() NGAY, dừng vòng lặp. Chỗ trống 2 (push): đẩy GIÁ TRỊ THẬT bên trong o, tức o.giaTri. Chỗ trống 3 (sau vòng lặp): bọc kq đã gom được vào co(...)."
- kind: strategy
  body: "if (o.kind === \"khong\") return khong(); kq.push(o.giaTri); ... return co(kq); — ba chỗ trống, mỗi chỗ một vai trò riêng trong vòng lặp."
- kind: one-line
  body: "return khong();\nkq.push(o.giaTri);\nreturn co(kq);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1,2,3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`traverse` — LẬT `Array<Option<T>>` thành `Option<Array<T>>`, dừng
NGAY tại phần tử lỗi đầu tiên, không cần biết phần tử sau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`traverse` viết bằng vòng lặp `for`. Bạn đã học `reduce` "đóng gói"
vòng lặp (T4.2) — viết `traverse` bằng `reduce` có được không?
::::

::::checkpoint{mastery=0.8}
::::
