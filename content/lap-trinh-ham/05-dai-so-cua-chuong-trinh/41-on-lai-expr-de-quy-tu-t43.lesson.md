---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.on-lai-expr-de-quy-tu-t43
title: "Ôn lại `Expr` đệ quy từ T4.3 — nhiều hàm, CÙNG khuôn đệ quy"
summary: "demNode(e: Expr): number — case \"so\": return 1; case \"cong\"/\"nhan\": return demNode(e.trai) + demNode(e.phai) + 1. Cấu trúc đệ quy GẦN Y HỆT tinh (T4.3) — chỉ khác PHÉP TÍNH ở từng biến thể."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 41
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [alg.recall-recursive-expr]
requires: [alg.review-traverse]
concepts: [alg.recall-recursive-expr]
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
Cụm cuối. Quay lại `Expr` — T4.3 đã dạy nó, và một hàm đệ quy trên nó.
Viết THÊM một hàm đệ quy KHÁC — thấy gì lặp lại?
::::

::::explain{#nho-lai-expr-va-tinh}
```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so": return e.giaTri;
    case "cong": return tinh(e.trai) + tinh(e.phai);
    case "nhan": return tinh(e.trai) * tinh(e.phai);
  }
}

const bieuThuc: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
  phai: { kind: "so", giaTri: 5 },
};

console.log(tinh(bieuThuc));
```

```text
35
```

`Expr` (T4.3, bài 32) — MỘT node LÁ (`"so"`, mang con số THÔ) và HAI
node NHÁNH (`"cong"`/`"nhan"`, mỗi node có `trai: Expr`, `phai: Expr`
— TỰ THAM CHIẾU chính `Expr`). `tinh` (T4.3, bài 33) đệ quy: gặp `"so"`
thì đọc số; gặp `"cong"`/`"nhan"` thì GỌI LẠI `tinh` trên `trai` VÀ
`phai`, rồi cộng/nhân hai kết quả đó. `(3 + 4) * 5 = 35`.

Viết MỘT hàm đệ quy KHÁC trên CÙNG `Expr` — `demNode(e: Expr): number`,
đếm SỐ NODE trong cả cây (kể cả node lá):

```typescript
function demNode(e: Expr): number {
  switch (e.kind) {
    case "so": return 1;
    case "cong": return demNode(e.trai) + demNode(e.phai) + 1;
    case "nhan": return demNode(e.trai) + demNode(e.phai) + 1;
  }
}

console.log(demNode(bieuThuc));
```

```text
5
```

Đặt `tinh` và `demNode` CẠNH NHAU: CẢ HAI đều `switch (e.kind)`, CẢ
HAI đều GỌI LẠI CHÍNH MÌNH trên `e.trai`/`e.phai` ở nhánh `"cong"`/
`"nhan"` — cấu trúc ĐỆ QUY gần như Y HỆT. Chỗ khác DUY NHẤT: PHÉP TÍNH
áp dụng SAU khi có kết quả đệ quy — `tinh` cộng hoặc nhân hai kết quả
con; `demNode` cộng hai kết quả con RỒI cộng thêm `1` (cho CHÍNH node
hiện tại). Nhánh `"so"` cũng khác: `tinh` đọc `e.giaTri`, `demNode`
trả LUÔN `1` (một node lá LÀ một node).
::::

::::example{#dem-node-cay-mot-la}
Cây chỉ có MỘT node lá (không nhánh nào) — `demNode` ra `1`, đúng
"một node LÀ một node":

```typescript title=readonly
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };
function demNode(e: Expr): number {
  switch (e.kind) {
    case "so": return 1;
    case "cong": return demNode(e.trai) + demNode(e.phai) + 1;
    case "nhan": return demNode(e.trai) + demNode(e.phai) + 1;
  }
}

const laMotSo: Expr = { kind: "so", giaTri: 42 };
console.log(demNode(laMotSo));

const congDon: Expr = { kind: "cong", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } };
console.log(demNode(congDon));
```

```text title=readonly
1
3
```

`laMotSo` KHÔNG có nhánh con — cây chỉ có ĐÚNG một node, `demNode` ra
`1` ngay ở nhánh `"so"`, không hề đệ quy. `congDon` có BA node (một
`"cong"` VÀ hai node lá `"so"` bên trong nó) — `demNode(congDon)` =
`demNode(so 1) + demNode(so 2) + 1` = `1 + 1 + 1` = `3`.
::::

::::predict{#doan-tinh-demnode-cay-moi commitOnce}
```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };
function tinh(e: Expr): number {
  switch (e.kind) {
    case "so": return e.giaTri;
    case "cong": return tinh(e.trai) + tinh(e.phai);
    case "nhan": return tinh(e.trai) * tinh(e.phai);
  }
}
function demNode(e: Expr): number {
  switch (e.kind) {
    case "so": return 1;
    case "cong": return demNode(e.trai) + demNode(e.phai) + 1;
    case "nhan": return demNode(e.trai) + demNode(e.phai) + 1;
  }
}

const bieuThucMoi: Expr = {
  kind: "cong",
  trai: { kind: "nhan", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 4 }, phai: { kind: "so", giaTri: 5 } },
};

console.log(tinh(bieuThucMoi));
console.log(demNode(bieuThucMoi));
```

Hai dòng cuối in ra gì (`bieuThucMoi` là `(2 * 3) + (4 * 5)`, với NĂM
node lá số VÀ hai node `"nhan"` VÀ một node `"cong"`)?

:::opt{correct}
`26` rồi `7`
:::

:::opt
`26` rồi `5` — vì `demNode` chỉ đếm các node LÁ (`"so"`), không đếm
các node NHÁNH (`"cong"`/`"nhan"`)
::why
Gần đúng ở việc bạn tính ĐÚNG `tinh(bieuThucMoi) = 26` (`2*3 + 4*5 =
6 + 20 = 26`) — phép tính đó đúng.

Chỗ lệch: `demNode` đếm TẤT CẢ node, KỂ CẢ node nhánh — nhánh
`"cong"`/`"nhan"` của nó `return demNode(e.trai) + demNode(e.phai) +
1`, CỘNG THÊM `1` cho CHÍNH node đó (không chỉ đếm node lá bên dưới).
Cây có BỐN node lá (`2`, `3`, `4`, `5`), cộng HAI node `"nhan"`, cộng
MỘT node `"cong"` — tổng `4 + 2 + 1 = 7`.
::
:::

:::opt
Máy báo lỗi biên dịch — `bieuThucMoi` lồng HAI node `"nhan"` bên
trong MỘT node `"cong"`, một cấu trúc `Expr` không hợp lệ
::why
Gần đúng ở việc bạn để ý `bieuThucMoi` có cấu trúc LỒNG khá sâu (một
`"cong"` chứa HAI `"nhan"` bên trong) — quan sát về ĐỘ SÂU đó đúng.

Chỗ lệch: `Expr` (T4.3, bài 32) là kiểu TỰ THAM CHIẾU — `trai`/`phai`
của `"cong"` VÀ `"nhan"` đều có kiểu `Expr`, LỒNG bao nhiêu tầng cũng
được, bất kể biến thể nào ở tầng nào. `bieuThucMoi` là một `Expr` HOÀN
TOÀN HỢP LỆ, biên dịch và chạy bình thường.
::
:::
::::

::::code{#viet_demnode}
Tự viết `demNode(e: Expr): number` — đếm số node trong cây, CÙNG
khuôn đệ quy `tinh`.

```typescript title=starter
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so": return e.giaTri;
    case "cong": return tinh(e.trai) + tinh(e.phai);
    case "nhan": return tinh(e.trai) * tinh(e.phai);
  }
}

function demNode(e: Expr): number {
  switch (e.kind) {
    case "so": return ___;
    case "cong": return ___;
    case "nhan": return ___;
  }
}

const bieuThuc: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
  phai: { kind: "so", giaTri: 5 },
};

console.log(demNode(bieuThuc));
```

```typescript title=solution
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so": return e.giaTri;
    case "cong": return tinh(e.trai) + tinh(e.phai);
    case "nhan": return tinh(e.trai) * tinh(e.phai);
  }
}

function demNode(e: Expr): number {
  switch (e.kind) {
    case "so": return 1;
    case "cong": return demNode(e.trai) + demNode(e.phai) + 1;
    case "nhan": return demNode(e.trai) + demNode(e.phai) + 1;
  }
}

const bieuThuc: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
  phai: { kind: "so", giaTri: 5 },
};

console.log(demNode(bieuThuc));
```

```typescript title=test
const laMotSo: Expr = { kind: "so", giaTri: 42 };
if (demNode(laMotSo) !== 1) throw new Error("một node lá đơn độc phải đếm ra 1");

const congDon: Expr = { kind: "cong", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } };
if (demNode(congDon) !== 3) throw new Error("cong với hai node lá con phải đếm ra 3 (1 cong + 2 so)");

const nhanDon: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } };
if (demNode(nhanDon) !== 3) throw new Error("nhan với hai node lá con phải đếm ra 3 (1 nhan + 2 so)");

if (demNode(bieuThuc) !== 5) throw new Error("bieuThuc có 5 node: nhan, cong, so 3, so 4, so 5");
if (tinh(bieuThuc) !== 35) throw new Error("tinh không được đổi hành vi — (3+4)*5 = 35");
```

:::hints
- kind: attention
  body: "Nhánh \"so\": một node lá LÀ một node — trả về 1 (không đệ quy). Nhánh \"cong\"/\"nhan\": đếm node của HAI nhánh con, CỘNG thêm 1 cho CHÍNH node hiện tại — demNode(e.trai) + demNode(e.phai) + 1."
- kind: strategy
  body: "case \"so\": return 1; case \"cong\": return demNode(e.trai) + demNode(e.phai) + 1; case \"nhan\": return demNode(e.trai) + demNode(e.phai) + 1; — CÙNG khuôn tinh, khác phép tính."
- kind: one-line
  body: "return 1;\nreturn demNode(e.trai) + demNode(e.phai) + 1;\nreturn demNode(e.trai) + demNode(e.phai) + 1;"
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
`demNode` và `tinh` — cấu trúc đệ quy gần như GIỐNG HỆT, chỉ khác
phép tính áp dụng ở mỗi biến thể.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Viết một hàm đệ quy MỚI trên `Expr` lần nữa sẽ LẶP LẠI y hệt phần
`switch`, phần gọi đệ quy trên `e.trai`/`e.phai` — chỉ phép tính khác.
Có cách nào tách phần LẶP LẠI đó ra, viết MỘT LẦN không?
::::

::::checkpoint{mastery=0.8}
::::
