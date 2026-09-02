---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.do-tong-hop-catamorphism
title: "Đo tổng hợp: Catamorphism"
summary: "`foldExpr` (bài 44) đã viết được `tinh` (bài 45) và `demNode` (bài 46) — không `switch`, không tự đệ quy. Bài chốt cụm: viết `inBieuThuc: (e: Expr) => string`, in biểu thức ra chuỗi có ngoặc (`\"(3 + 4)\"`). Không khái niệm mới — đo khả năng tự khai ĐÚNG `BoXuLyExpr<string>` cho một việc CHƯA từng làm."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 47
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.review-catamorphism]
requires: [alg.write-count-nodes-via-fold]
concepts: [alg.review-catamorphism]
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
Bài trước bạn viết `demNode` bằng `foldExpr` — không một dòng `switch`,
không một lời gọi đệ quy tự tay. Bài này chốt cụm: thử với một việc
KHÁC HẲN — không đếm, không tính ra số, mà IN biểu thức ra thành chữ.
::::

::::explain{#hai-catamorphism-da-co}
`foldExpr` (bài 44) TÁCH sẵn phần đệ quy — `tinh` (bài 45) và `demNode`
(bài 46) chỉ khác nhau ở MỘT thứ: `BoXuLyExpr<R>` truyền vào. Đặt cả
hai cạnh nhau, trên CÙNG một cây:

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
  foldExpr(e, { so: (g) => g, cong: (t, p) => t + p, nhan: (t, p) => t * p });

const demNode = (e: Expr): number =>
  foldExpr(e, { so: () => 1, cong: (t, p) => t + p + 1, nhan: (t, p) => t + p + 1 });

const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 4 }, phai: { kind: "so", giaTri: 5 } },
};

console.log(tinh(bieuThuc));
console.log(demNode(bieuThuc));
```

```text
23
5
```

`tinh(bieuThuc)` tính `3 + (4 * 5) = 23`. `demNode(bieuThuc)` đếm đúng
5 node (`cong`, `so 3`, `nhan`, `so 4`, `so 5`). Cả hai đều có `R` là
`number` — nhưng chữ ký `foldExpr<R>` KHÔNG hề ép `R` phải là số. `R`
là generic THẬT: bất kỳ kiểu nào cũng dùng được, miễn `BoXuLyExpr<R>`
khai đúng ba hàm `so`/`cong`/`nhan`, CÙNG trả về kiểu `R` đó.
::::

::::example{#mot-catamorphism-thu-ba-do-sau-cay}
`R` không cần LÀ kết quả TÍNH TOÁN hay ĐẾM — nó có thể là bất kỳ số đo
nào khác của cây, ví dụ ĐỘ SÂU (tầng lồng sâu nhất):

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

const doSauCay = (e: Expr): number =>
  foldExpr(e, {
    so: () => 0,
    cong: (t, p) => 1 + Math.max(t, p),
    nhan: (t, p) => 1 + Math.max(t, p),
  });

const laSo: Expr = { kind: "so", giaTri: 9 };
const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 4 }, phai: { kind: "so", giaTri: 5 } },
};

console.log(doSauCay(laSo));
console.log(doSauCay(bieuThuc));
```

```text title=readonly
0
2
```

`doSauCay` KHÔNG đếm số node (đó là `demNode`) — nó lấy độ sâu LỚN
NHẤT giữa hai nhánh con (`Math.max`), cộng thêm một tầng cho CHÍNH node
hiện tại. Một `so` một mình có độ sâu `0` (không lồng gì). `bieuThuc`
(`3 + (4 * 5)`) có độ sâu `2`: tầng `cong` rồi tầng `nhan` bên trong nó
— nhánh `so 3` chỉ ở tầng `1`, không phải nhánh sâu nhất nên không
quyết định kết quả. Y HỆT `tinh`/`demNode`: không `switch`, không đệ
quy tay — chỉ một `BoXuLyExpr<number>` MỚI, với phép tính khác.
::::

::::predict{#doan-in-bieu-thuc commitOnce}
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

const inBieuThuc = (e: Expr): string =>
  foldExpr(e, {
    so: (g) => String(g),
    cong: (t, p) => `(${t} + ${p})`,
    nhan: (t, p) => `(${t} * ${p})`,
  });

const long: Expr = {
  kind: "cong",
  trai: { kind: "nhan", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
  phai: { kind: "so", giaTri: 4 },
};

console.log(inBieuThuc(long));
```

Dòng cuối in ra gì?

:::opt{correct}
`((2 * 3) + 4)`
:::

:::opt
`(2 * 3) + 4` — vì nhánh `nhan` bên trong ĐÃ có ngoặc riêng của nó
(`(2 * 3)`), nên `cong` bên ngoài không cần bọc thêm một cặp ngoặc nữa
::why
Gần đúng ở việc bạn tính đúng PHẦN TRONG — `foldExpr` gọi `nhan` cho
nhánh `trai` TRƯỚC, ra đúng chuỗi `"(2 * 3)"` (đã có ngoặc, giá trị
CÓ SẴN đó được truyền tiếp vào `cong` dưới dạng `t`).

Chỗ lệch: `cong` KHÔNG chỉ nối `t + p` — nó bọc CHÍNH KẾT QUẢ CỦA NÓ
trong một cặp ngoặc MỚI, TÁCH BIỆT với ngoặc mà `nhan` đã tự bọc. Nhìn
lại thân hàm: `` `(${t} + ${p})` `` — dấu ngoặc `(` và `)` NẰM NGOÀI cả
`t` lẫn `p`. Với `t = "(2 * 3)"`, `p = "4"`, kết quả là
`"((2 * 3) + 4)"` — hai lớp ngoặc, không phải một.
::
:::

:::opt
`((2 + 3) * 4)` — vì `cong` in ra dấu `*` (phép nhân) và `nhan` in ra
dấu `+` (phép cộng), đảo ngược ký hiệu so với tên hàm
::why
Gần đúng ở việc bạn dựng đúng SỐ LỚP ngoặc và ĐÚNG vị trí lồng (một
cặp ngoặc bên trong, một cặp bên ngoài) — cấu trúc lồng đó đúng.

Chỗ lệch: `cong` (nhánh `kind === "cong"`, phép CỘNG) in dấu `+`, và
`nhan` (nhánh `kind === "nhan"`, phép NHÂN) in dấu `*` — đúng theo TÊN
của từng nhánh, không đảo ngược. `long.trai` là `nhan(2, 3)` → in
`"(2 * 3)"` (dấu `*`, không phải `+`); `long` (tầng ngoài) là `cong` →
in `"(... + 4)"` (dấu `+`, không phải `*`). Kết quả đúng là
`"((2 * 3) + 4)"`.
::
:::
::::

::::code{#in_bieu_thuc}
Viết `inBieuThuc: (e: Expr) => string` — dùng `foldExpr`, KHÔNG viết
`switch` hay đệ quy tay nào. Một `so` in ra đúng con số của nó (dạng
chuỗi). Một `cong`/`nhan` in ra chuỗi có NGOẶC BAO NGOÀI, ví dụ
`"(3 + 4)"` hoặc `"(2 * 5)"`.

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

const inBieuThuc = (e: Expr): string =>
  foldExpr(e, {
    so: (g) => ___,
    cong: (t, p) => ___,
    nhan: (t, p) => ___,
  });

const vd: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: { kind: "so", giaTri: 4 },
};
console.log(inBieuThuc(vd));
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

const inBieuThuc = (e: Expr): string =>
  foldExpr(e, {
    so: (g) => String(g),
    cong: (t, p) => `(${t} + ${p})`,
    nhan: (t, p) => `(${t} * ${p})`,
  });

const vd: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: { kind: "so", giaTri: 4 },
};
console.log(inBieuThuc(vd));
```

```typescript title=test
if (inBieuThuc({ kind: "so", giaTri: 5 }) !== "5") throw new Error("nhánh so phải in ra đúng con số, không thêm ngoặc");
if (inBieuThuc(vd) !== "(3 + 4)") throw new Error("nhánh cong phải in dạng (trai + phai), có ngoặc bao ngoài");

const nhanDon: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 5 } };
if (inBieuThuc(nhanDon) !== "(2 * 5)") throw new Error("nhánh nhan phải in dạng (trai * phai), có ngoặc bao ngoài");

const long: Expr = {
  kind: "cong",
  trai: { kind: "nhan", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
  phai: { kind: "so", giaTri: 4 },
};
if (inBieuThuc(long) !== "((2 * 3) + 4)") throw new Error("phải dùng chuỗi ĐÃ IN của nhánh con (t, p), không phải Expr thô — và cong/nhan ngoài cùng vẫn phải tự bọc ngoặc");
```

:::hints
- kind: attention
  body: "t và p trong cong/nhan LÀ CHUỖI đã in xong của hai nhánh con (kết quả foldExpr gọi đệ quy trước), không phải Expr thô — không có .kind, .trai, .phai nào để đọc trên t/p. Mỗi cong/nhan PHẢI tự bọc ngoặc quanh CHÍNH kết quả của nó, kể cả khi t hoặc p đã có ngoặc riêng rồi."
- kind: strategy
  body: 'so: (g) => String(g) — biến số thành chuỗi. cong: (t, p) => `(${t} + ${p})` — nối t và p bằng dấu +, bọc trong một cặp ngoặc MỚI. nhan: (t, p) => `(${t} * ${p})` — y hệt cong nhưng dùng dấu * và đúng tên biến (không đảo ngược cong/nhan).'
- kind: one-line
  body: 'so: String(g) — cong: `(${t} + ${p})` — nhan: `(${t} * ${p})`'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "(3 + 4)"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không khái niệm mới — bạn tự khai được `BoXuLyExpr<string>` cho một
việc `foldExpr` chưa từng làm trước đây. Đó CHÍNH LÀ lợi ích thật của
catamorphism: thêm một hàm MỚI trên `Expr` chỉ cần MỘT object mô tả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track này vừa dạy SÁU khái niệm riêng — Monoid, Functor, Applicative,
Monad, Traverse, Catamorphism. Mỗi cái giải quyết một vấn đề khác
nhau (kết hợp, biến đổi, ghép độc lập, chuỗi phụ thuộc, lật cấu trúc,
tách đệ quy) — nhưng có phải chúng là SÁU thứ rời rạc, hay cùng MỘT
câu hỏi lặp lại nhiều lần dưới nhiều lớp áo?

Bài sau lùi lại, nhìn cả sáu cùng một lúc.
::::

::::checkpoint{mastery=0.8}
::::
