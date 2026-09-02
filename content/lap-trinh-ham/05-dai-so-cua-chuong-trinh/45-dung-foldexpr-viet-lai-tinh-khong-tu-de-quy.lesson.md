---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.dung-foldexpr-viet-lai-tinh-khong-tu-de-quy
title: "Dùng `foldExpr` viết LẠI `tinh` — KHÔNG tự viết đệ quy nữa"
summary: "const tinh = (e: Expr) => foldExpr(e, { so: g => g, cong: (t,p) => t+p, nhan: (t,p) => t*p }) — ĐÃ ĐO THẬT, ra ĐÚNG kết quả `tinhCu` (viết tay đệ quy, bài 41) từng ra. `tinh` MỚI không còn `switch`, không tự gọi lại chính nó — toàn bộ phần đệ quy nằm bên trong `foldExpr`."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 45
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [alg.reimplement-eval-via-fold]
requires: [alg.write-fold-expr]
concepts: [alg.reimplement-eval-via-fold]
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
Bài trước tách phần đệ quy ra khỏi `tinh` và `demNode`, đóng gói nó vào
MỘT hàm `foldExpr`. Nhưng `tinh` (bài 41, viết tay) vẫn còn nguyên
`switch` và tự gọi lại chính nó — chưa ai DÙNG `foldExpr` để viết lại
nó cả. Hôm nay làm đúng việc đó.
::::

::::explain{#tinh-viet-lai-bang-foldexpr}
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

// tinh CU — bai 41, viet tay de quy, tu goi lai chinh no
function tinhCu(e: Expr): number {
  switch (e.kind) {
    case "so": return e.giaTri;
    case "cong": return tinhCu(e.trai) + tinhCu(e.phai);
    case "nhan": return tinhCu(e.trai) * tinhCu(e.phai);
  }
}

// tinh MOI — dung foldExpr, KHONG switch, KHONG tu goi lai chinh no
const tinh = (e: Expr): number =>
  foldExpr(e, {
    so: (g) => g,
    cong: (t, p) => t + p,
    nhan: (t, p) => t * p,
  });

const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 2 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
};

console.log(tinhCu(bieuThuc));
console.log(tinh(bieuThuc));
console.log(tinhCu(bieuThuc) === tinh(bieuThuc));
```

```text
14
14
true
```

Nhìn kỹ thân hàm `tinh` MỚI: không có `switch`, không có `case`, và
không có dòng nào gọi `tinh(...)` bên trong chính nó. Nó chỉ là MỘT
lời gọi `foldExpr` với một object literal — object đó nói "với một số
trần thì trả nguyên nó (`g => g`), với một `cong` thì CỘNG hai kết quả
con (`t + p`), với một `nhan` thì NHÂN hai kết quả con (`t * p`)".
Không có gì trong object này TỰ ĐI SÂU vào `e.trai`/`e.phai` — việc đó
đã xảy ra RỒI, bên trong `foldExpr`, TRƯỚC KHI `t` và `p` được đưa vào
tay `cong`/`nhan`.

`tinhCu(bieuThuc) === tinh(bieuThuc)` in ra `true` — hai cách viết,
MỘT kết quả giống hệt nhau trên biểu thức `2 + 3 * 4`. Cái đã đổi
không phải câu trả lời — mà là AI chịu trách nhiệm cho phần đệ quy.
Trước đây `tinh` tự lo lấy (gọi lại chính nó). Giờ `tinh` giao hẳn
việc đó cho `foldExpr`, chỉ còn giữ lại phần THẬT SỰ đặc trưng cho
"tính giá trị": cộng thì làm gì, nhân thì làm gì.
::::

::::example{#tinh-dung-cho-cay-sau-hon}
`tinh` không hề "biết" cây `Expr` sâu bao nhiêu — độ sâu là việc của
`foldExpr`, không phải việc của `tinh`. Cùng định nghĩa `tinh` như
trên, thử trên một cây SÂU HƠN cây ở trên (một `nhan` lồng bên trong
một `cong`, lồng bên trong một `nhan` ngoài cùng):

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

const tinh = (e: Expr): number =>
  foldExpr(e, {
    so: (g) => g,
    cong: (t, p) => t + p,
    nhan: (t, p) => t * p,
  });

const sau: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
  phai: {
    kind: "cong",
    trai: { kind: "so", giaTri: 4 },
    phai: { kind: "nhan", trai: { kind: "so", giaTri: 5 }, phai: { kind: "so", giaTri: 6 } },
  },
};

console.log(tinh(sau));
```

```text title=readonly
170
```

`sau` là `(2 + 3) * (4 + (5 * 6))` — bốn tầng lồng, sâu hơn hẳn `bieuThuc`
ở khối trên. `tinh` vẫn ra đúng `170` (`5 * (4 + 30) = 5 * 34 = 170`) mà
KHÔNG cần sửa lấy một dòng nào trong định nghĩa `tinh`. Định nghĩa
`tinh` chỉ dài BA dòng (`so`/`cong`/`nhan`) dù cây có sâu một tầng hay
mười tầng — vì phần "đi sâu tới đâu" chưa bao giờ thuộc về `tinh`.
::::

::::predict{#doan-tinh-long-trong-cong commitOnce}
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

const tinh = (e: Expr): number =>
  foldExpr(e, {
    so: (g) => g,
    cong: (t, p) => t + p,
    nhan: (t, p) => t * p,
  });

const a: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
  phai: { kind: "cong", trai: { kind: "so", giaTri: 4 }, phai: { kind: "so", giaTri: 5 } },
};
const b: Expr = { kind: "cong", trai: a, phai: { kind: "so", giaTri: 1 } };

console.log(tinh(a));
console.log(tinh(b));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`45` rồi `46`
:::

:::opt
`45` rồi `45` — vì `tinh` (định nghĩa ở trên) không có dòng nào tự gọi
lại chính nó, nên nó không "biết cách" tính thêm phần `+ 1` bọc BÊN
NGOÀI `a`
::why
Gần đúng ở việc bạn tính đúng dòng ĐẦU (`tinh(a) = (2+3) * (4+5) = 45`)
— phép tính đó đúng, và bạn cũng để ý đúng: thân hàm `tinh` THẬT SỰ
không có dòng nào gọi lại `tinh`.

Chỗ lệch: `b` là một node `cong` MỚI, BỌC `a` ở nhánh `trai` và
`{ kind: "so", giaTri: 1 }` ở nhánh `phai` — khi `tinh(b)` chạy,
`foldExpr` (không phải `tinh`) tự đi vào `b.trai` (gọi đệ quy, tính ra
`45` qua đúng chuỗi `bx.cong`/`bx.nhan` bên trong) và `b.phai` (ra
`1`), RỒI mới gọi `bx.cong(45, 1)` ở tầng ngoài cùng, cho `46`. Đệ quy
không hề biến mất — nó chỉ chuyển hẳn vào bên trong `foldExpr`, không
còn nằm trong `tinh` nữa.
::
:::

:::opt
Máy báo lỗi biên dịch — `cong: (t, p) => t + p` trong `tinh` cộng HAI
SỐ, nhưng đối số `foldExpr` truyền vào thực chất là hai `Expr` con,
không phải số đã tính
::why
Gần đúng ở việc bạn để ý đúng: `cong` trong `BoXuLyExpr<R>` nhận ĐÚNG
HAI đối số (`trai: R, phai: R`) — quan sát về SỐ LƯỢNG đối số đó đúng.

Chỗ lệch: hai đối số đó (`t`, `p`) là kết quả ĐÃ TÍNH của `foldExpr`
gọi đệ quy trên `e.trai`/`e.phai` — không phải hai `Expr` thô. Chữ ký
`cong: (trai: R, phai: R) => R` ghi rõ điều đó (`R`, không phải
`Expr`) — ở đây `R` là `number`, nên `t` và `p` LUÔN LÀ số, `t + p` là
phép cộng số hợp lệ, biên dịch sạch, không có lỗi nào cả.
::
:::
::::

::::code{#tinh_qua_foldexpr}
`Expr`, `BoXuLyExpr<R>`, `foldExpr` đã có sẵn (từ bài trước). Viết
LẠI `tinh` bằng cách gọi `foldExpr` — điền đúng phép tính cho `cong`
(cộng hai kết quả con) và `nhan` (nhân hai kết quả con). KHÔNG dùng
`switch`, KHÔNG tự gọi lại `tinh`/`foldExpr` bên trong hai chỗ trống.

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

const tinh = (e: Expr): number =>
  foldExpr(e, {
    so: (g) => g,
    cong: (t, p) => ___,
    nhan: (t, p) => ___,
  });

const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 2 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
};

console.log(tinh(bieuThuc));
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

const tinh = (e: Expr): number =>
  foldExpr(e, {
    so: (g) => g,
    cong: (t, p) => t + p,
    nhan: (t, p) => t * p,
  });

const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 2 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
};

console.log(tinh(bieuThuc));
```

```typescript title=test
const bieuThuc2: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
  phai: { kind: "cong", trai: { kind: "so", giaTri: 4 }, phai: { kind: "so", giaTri: 5 } },
};

if (tinh(bieuThuc) !== 14) throw new Error("tinh(bieuThuc) phai ra 14 (2 + 3*4)");
if (tinh(bieuThuc2) !== 45) throw new Error("tinh(bieuThuc2) phai ra 45 ((2+3) * (4+5))");
if (tinh({ kind: "so", giaTri: 9 }) !== 9) throw new Error("tinh tren mot node so don le phai ra chinh no");
```

:::hints
- kind: attention
  body: "Hai chỗ trống nằm trong object truyền cho foldExpr, ở nhánh cong và nhan. t và p đã LÀ kết quả tính xong của hai nhánh con (kiểu number) — không phải Expr thô, không cần gọi lại tinh hay foldExpr bên trong."
- kind: strategy
  body: "Nhớ lại tinhCu (bài 41): case \"cong\" trả về tinhCu(e.trai) + tinhCu(e.phai); case \"nhan\" trả về tinhCu(e.trai) * tinhCu(e.phai). t chính là tinhCu(e.trai) đã tính, p chính là tinhCu(e.phai) đã tính — chỉ còn viết lại phép cộng/nhân bằng t và p."
- kind: one-line
  body: "cong: (t, p) => t + p, nhan: (t, p) => t * p"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "14"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`tinh` giờ không còn `switch`, không còn tự gọi lại chính nó — mà vẫn
ra ĐÚNG kết quả cũ, trên MỌI độ sâu cây. Phần đệ quy đã có một nơi ở
CỐ ĐỊNH, viết một lần, dùng mãi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinh` chỉ là MỘT cách dùng `foldExpr` — với `BoXuLyExpr<number>` cụ
thể (`cong` là cộng, `nhan` là nhân). `demNode` (bài 41-42, đếm số
node trong cây) cũng đệ quy trên CÙNG `Expr`, theo ĐÚNG khuôn ấy — chỉ
khác phép tính ở từng biến thể. Có cần viết lại `switch` cho `demNode`
nữa không, hay CHỈ cần một `BoXuLyExpr<number>` khác, truyền thẳng vào
`foldExpr` đã có sẵn?

Bài sau trả lời — bằng cách viết `demNode` mà không chạm một dòng
`switch` nào.
::::

::::checkpoint{mastery=0.8}
::::
