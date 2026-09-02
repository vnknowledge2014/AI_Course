---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.dung-lai-foldexpr-viet-them-demnode
title: "DÙNG LẠI CHÍNH `foldExpr` — viết `demNode` không cần đệ quy tay lần nữa"
summary: "foldExpr(e, { so: () => 1, cong: (t,p) => t+p+1, nhan: (t,p) => t+p+1 }) — CHỈ một BoXuLyExpr<number> MỚI, KHÔNG một dòng switch hay đệ quy nào, để đếm số node trong Expr. Lợi ích thật của catamorphism: thêm một hàm mới trên Expr chỉ cần khai lại phần TÍNH TOÁN, phần ĐỆ QUY đã có sẵn trong foldExpr."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 46
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [alg.write-count-nodes-via-fold]
requires: [alg.reimplement-eval-via-fold]
concepts: [alg.write-count-nodes-via-fold]
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
Bài trước, bạn viết lại `tinh` bằng `foldExpr` — không còn `switch`,
không tự gọi lại chính mình. Nhưng MỘT ví dụ chưa chứng minh được gì
nhiều. Thử thêm một hàm HOÀN TOÀN khác việc trên CÙNG `Expr` xem
`foldExpr` có còn đủ dùng không.
::::

::::explain{#demnode-qua-foldexpr}
```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const tinh = (e: Expr): number => foldExpr(e, {
  so: (g) => g,
  cong: (t, p) => t + p,
  nhan: (t, p) => t * p,
});

const demNode = (e: Expr): number => foldExpr(e, {
  so: () => 1,
  cong: (t, p) => t + p + 1,
  nhan: (t, p) => t + p + 1,
});

const bieuThuc: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
  phai: { kind: "so", giaTri: 5 },
};

console.log(tinh(bieuThuc));
console.log(demNode(bieuThuc));
```

```text
35
5
```

`Expr`, `BoXuLyExpr<R>`, `foldExpr` — CẢ BA giữ NGUYÊN, không sửa một
chữ nào so với bài trước. `tinh` cũng giữ nguyên. Thứ MỚI DUY NHẤT
là `demNode` — một `BoXuLyExpr<number>` KHÁC, mô tả một việc HOÀN
TOÀN khác `tinh` (đếm node, không phải tính giá trị số học):

- `so: () => 1` — một node lá (`"so"`) LUÔN là ĐÚNG một node. Không
  cần đọc `giaTri` (nó không liên quan tới việc ĐẾM), nên hàm không
  nhận tham số nào.
- `cong: (t, p) => t + p + 1` — `t` và `p` không phải `Expr` thô, mà
  là SỐ NODE hai nhánh con `foldExpr` đã đếm XONG. Cộng `t + p` (số
  node ở hai nhánh con) RỒI CỘNG THÊM `1` cho CHÍNH node `"cong"`
  đang xét.
- `nhan: (t, p) => t + p + 1` — hệt logic `cong`, vì việc ĐẾM node
  không phân biệt node đó là phép cộng hay phép nhân.

`bieuThuc` là `(3 + 4) * 5` — năm node: `so(3)`, `so(4)`, `cong`,
`so(5)`, `nhan`. `demNode(bieuThuc)` ra ĐÚNG `5`. Không có `switch`
nào được viết TRONG `demNode` — `foldExpr` (đã viết MỘT LẦN ở bài
trước) tự lo phần đệ quy; `demNode` chỉ khai "với node lá thì tính
gì, với hai nhánh con ĐÃ đếm xong thì gộp thế nào".
::::

::::example{#truong-hop-nen-va-canh-bien}
```typescript title=readonly
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const demNode = (e: Expr): number => foldExpr(e, {
  so: () => 1,
  cong: (t, p) => t + p + 1,
  nhan: (t, p) => t + p + 1,
});

const laDon: Expr = { kind: "so", giaTri: 9 };
const capSo: Expr = { kind: "cong", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } };

console.log(demNode(laDon));
console.log(demNode(capSo));
```

```text title=readonly
1
3
```

`laDon` chỉ là MỘT node `"so"` — không có `trai`/`phai`, `foldExpr`
gọi thẳng `bx.so(9)`, ra `1`. Đây là CA ĐÁY của đệ quy — `demNode`
không cần viết riêng "trường hợp cây một node", `foldExpr` tự xử lý
đúng vì `so: () => 1` áp dụng cho MỌI node lá, dù cây lớn hay nhỏ.

`capSo` là `1 + 2` — hai node lá cộng một node `"cong"` — `demNode`
ra `3`, khớp công thức `t + p + 1` với `t = demNode(so 1) = 1` và
`p = demNode(so 2) = 1`. Cùng MỘT `demNode`, không phân biệt cây có
mấy tầng — `foldExpr` tự lo việc gọi lại đúng chỗ.
::::

::::predict{#doan-demnode-cay-long commitOnce}
```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const demNode = (e: Expr): number => foldExpr(e, {
  so: () => 1,
  cong: (t, p) => t + p + 1,
  nhan: (t, p) => t + p + 1,
});

const exprA: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } },
  phai: { kind: "so", giaTri: 3 },
};

console.log(demNode(exprA));
```

`exprA` là `(1 + 2) * 3`. Dòng cuối in ra gì?

:::opt{correct}
`5`
:::

:::opt
`3` — vì `demNode` chỉ đếm các node `"so"` (giá trị lá thật), các
node `"cong"`/`"nhan"` chỉ là PHÉP TOÁN, không phải giá trị, nên
không được tính là node
::why
Gần đúng ở việc bạn đếm ĐÚNG số node `"so"` trong `exprA` — có đúng
BA node lá (`so(1)`, `so(2)`, `so(3)`).

Chỗ lệch: `BoXuLyExpr<number>` của `demNode` có `cong: (t,p) => t+p+1`
và `nhan: (t,p) => t+p+1` — CẢ HAI đều CỘNG THÊM `1` khi gặp node
đó, đúng nghĩa "node `"cong"`/`"nhan"` CŨNG là một node trong cây,
cần được đếm". `exprA` có MỘT node `"cong"` (bên trong) VÀ MỘT node
`"nhan"` (ở gốc) NGOÀI ba node `"so"` — tổng `3 + 1 + 1 = 5`, không
phải `3`.
::
:::

:::opt
`4` — vì `demNode` chỉ cộng thêm `1` một lần DUY NHẤT, ở node NGOÀI
CÙNG (gốc của cây), các node toán tử BÊN TRONG không cộng thêm gì
::why
Gần đúng ở việc bạn nhận ra `demNode` CÓ cộng thêm `1` ở đâu đó khi
gặp một node toán tử — hướng quan sát đó đúng.

Chỗ lệch: `foldExpr` gọi ĐỆ QUY — mỗi lần `switch` rơi vào nhánh
`"cong"` hay `"nhan"`, DÙ đó là node GỐC hay node CON bên trong, đều
đi qua ĐÚNG cùng một hàm `cong`/`nhan` trong `BoXuLyExpr<number>`,
và hàm đó LUÔN cộng thêm `1`. `exprA` có HAI node toán tử: `"cong"`
(nhánh con `trai`) VÀ `"nhan"` (gốc) — CẢ HAI đều cộng thêm `1`,
không chỉ node gốc. Tổng node: ba `"so"` + node `"cong"` + node
`"nhan"` = `5`, không phải `4`.
::
:::
::::

::::code{#demnode_qua_foldexpr}
Viết `demNode` bằng `foldExpr` — đếm số node trong `Expr`. KHÔNG
viết `switch (e.kind)` hay tự gọi lại `demNode(e.trai)`/
`demNode(e.phai)` — CHỈ khai một `BoXuLyExpr<number>` mới, đưa cho
`foldExpr` (đã có sẵn, giữ NGUYÊN không sửa).

```typescript title=starter
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const demNode = (e: Expr): number => foldExpr(e, {
  so: ___,
  cong: ___,
  nhan: ___,
});

const viDu: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } },
  phai: { kind: "so", giaTri: 3 },
};

console.log(demNode(viDu));
```

```typescript title=solution
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const demNode = (e: Expr): number => foldExpr(e, {
  so: () => 1,
  cong: (t, p) => t + p + 1,
  nhan: (t, p) => t + p + 1,
});

const viDu: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } },
  phai: { kind: "so", giaTri: 3 },
};

console.log(demNode(viDu));
```

```typescript title=test
const laDon: Expr = { kind: "so", giaTri: 9 };
const capCong: Expr = { kind: "cong", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } };
const capNhan: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } };
const long: Expr = { kind: "nhan", trai: { kind: "cong", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 2 } }, phai: { kind: "so", giaTri: 3 } };

if (demNode(laDon) !== 1) throw new Error("demNode trên một node 'so' đơn lẻ phải ra 1 — chỉ MỘT node duy nhất trong cây");
if (demNode(capCong) !== 3) throw new Error("demNode trên cong(so,so) phải ra 3 — hai node 'so' cộng một node 'cong'");
if (demNode(capNhan) !== 3) throw new Error("demNode trên nhan(so,so) phải ra 3 — hai node 'so' cộng một node 'nhan'");
if (demNode(long) !== 5) throw new Error("demNode trên cây lồng (cong bên trong, nhan ở gốc) phải ra 5 — ba node 'so', một 'cong', một 'nhan'");
```

:::hints
- kind: attention
  body: "demNode phải dùng foldExpr — KHÔNG viết switch (e.kind) hay gọi demNode(e.trai)/demNode(e.phai) trực tiếp bên trong. Ba chỗ trống là BA HÀM khai trong BoXuLyExpr<number>, không phải ba con số rời."
- kind: strategy
  body: "so: một node 'so' LUÔN là đúng 1 node trong cây — trả về hằng số 1, không cần đọc giaTri (giaTri không liên quan tới việc ĐẾM). cong/nhan: t và p là SỐ NODE của hai nhánh con foldExpr đã đếm xong (đệ quy chạy TRƯỚC) — cộng t + p RỒI CỘNG THÊM 1 cho chính node cong/nhan đang xét."
- kind: one-line
  body: "so: () => 1, cong: (t, p) => t + p + 1, nhan: (t, p) => t + p + 1"
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
Hai hàm HOÀN TOÀN khác việc — `tinh` (tính giá trị) và `demNode`
(đếm node) — dùng CHUNG một `foldExpr`, không sửa một chữ nào trong
nó. Đây chính là lợi ích thật của catamorphism: thêm hàm MỚI trên
`Expr` chỉ cần MỘT object mô tả, không viết lại cấu trúc đệ quy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinh` và `demNode` đều trả về `number`. Nhưng `foldExpr<R>` là
GENERIC — `R` có bắt buộc phải là `number` không? Nếu viết một
`BoXuLyExpr<string>` (ví dụ: in biểu thức ra chuỗi có ngoặc, như
`"(3 + 4)"`), `foldExpr` có còn dùng được y hệt không, hay cần sửa
gì đó?

Bài sau chốt cụm Catamorphism — và trả lời câu hỏi đó bằng chính
một hàm MỚI bạn tự viết.
::::

::::checkpoint{mastery=0.8}
::::
