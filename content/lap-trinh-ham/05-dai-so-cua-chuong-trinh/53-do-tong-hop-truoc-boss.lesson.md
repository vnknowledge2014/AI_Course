---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.do-tong-hop-truoc-boss
title: "Đo tổng hợp trước BOSS"
summary: "Một hàm nhỏ ghép bốn khái niệm đã học: `foldExpr` (Catamorphism) tính điểm từ cây biểu thức, `.map()` (Functor) biến đổi mảng, `traverse` kiểm không âm, `gopTatCa` (Monoid) cộng dồn — không khái niệm mới, chỉ đo khả năng GHÉP trước khi vào BOSS."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 53
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.gate-review]
requires: [alg.recall-python-tracks]
concepts: [alg.gate-review]
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
Bài 52 nối ngược track này với hai track Python — cùng khuôn, khác
tên gọi. Bài này hỏi một câu THỰC TẾ hơn: bạn ghép được NHIỀU khuôn đó
vào MỘT chương trình, hay chỉ dùng được TỪNG cái riêng lẻ?
::::

::::explain{#ghep-ba-y-mot-ham}
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

interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T; }
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); }
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

const baiThi: Expr[] = [
  { kind: "cong", trai: { kind: "so", giaTri: 6 }, phai: { kind: "so", giaTri: 2 } },
  { kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 3 } },
];

const diem = baiThi.map((e) => tinh(e));
console.log(diem);
console.log(gopTatCa(monoidCong, diem));
```

```text
[8,9]
17
```

Mỗi bài thi là MỘT cây biểu thức `Expr` (T4.3, cụm 6-7 track này). `tinh`
không tự viết đệ quy — nó gọi `foldExpr` (**Catamorphism**, cụm 7) với một
`BoXuLyExpr<number>` khai rõ "gặp số thì trả nguyên số, gặp cộng/nhân thì
gộp hai nhánh con thế nào". `baiThi.map((e) => tinh(e))` là **Functor**
(cụm 3, bài 16: `Array.prototype.map` cùng khuôn `map` đã học) — biến đổi
TỪNG phần tử BÊN TRONG mảng, giữ nguyên cái mảng làm "vỏ". `gopTatCa
(monoidCong, diem)` là **Monoid** (cụm 1) — gộp CẢ mảng số `[8, 9]` thành
MỘT tổng `17`.

Ba khái niệm, một hàm ngắn ba dòng — không có gì mới ở đây, chỉ là ba thứ
đã học đứng CẠNH NHAU, không đụng độ.
::::

::::example{#vi-du-bon-khai-niem}
Thêm MỘT khái niệm nữa vào cùng luồng: kiểm tra điểm hợp lệ TRƯỚC khi
cộng.

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
  foldExpr(e, { so: (g) => g, cong: (t, p) => t + p, nhan: (t, p) => t * p });

type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) { case "co": return co(f(o.giaTri)); case "khong": return khong(); }
}

function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T; }
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); }
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function tongDiemThang100(baiThi: Expr[]): Option<number> {
  const diemGoc = baiThi.map((e) => tinh(e));
  const diemThang100 = diemGoc.map((d) => d * 10);
  const daKiemTra = traverse(diemThang100, (d): Option<number> => (d <= 100 ? co(d) : khong()));
  return mapOption(daKiemTra, (mang) => gopTatCa(monoidCong, mang));
}

const hopLe: Expr[] = [
  { kind: "cong", trai: { kind: "so", giaTri: 6 }, phai: { kind: "so", giaTri: 2 } },
  { kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 2 } },
];

const coLoi: Expr[] = [
  { kind: "cong", trai: { kind: "so", giaTri: 6 }, phai: { kind: "so", giaTri: 2 } },
  { kind: "nhan", trai: { kind: "so", giaTri: 4 }, phai: { kind: "so", giaTri: 3 } },
];

console.log(JSON.stringify(tongDiemThang100(hopLe)));
console.log(JSON.stringify(tongDiemThang100(coLoi)));
```

```text title=readonly
{"kind":"co","giaTri":140}
{"kind":"khong"}
```

Bước thứ hai `.map((d) => d * 10)` vẫn là **Functor** — chuẩn hoá từng
điểm về thang 100. Bước kế tiếp là **Traverse** (cụm 6, bài 36): lật mảng
`number[]` qua một phép kiểm CÓ THỂ THẤT BẠI (`d <= 100 ? co(d) : khong()`)
thành `Option<number[]>` — "có TOÀN BỘ mảng hợp lệ" hoặc "không, có ít
nhất MỘT điểm vượt 100". `mapOption(daKiemTra, mang => gopTatCa(...))` áp
**Monoid** BÊN TRONG lớp `Option`: nếu `daKiemTra` là `khong()`, `mapOption`
giữ nguyên `khong()`, KHÔNG chạm tới `gopTatCa` (đúng luật Functor — bài
18: `map` trên "không" ra thẳng "không", không gọi hàm áp dụng).

`hopLe` (80 + 60, cả hai đều ≤ 100) ra `co(140)`. `coLoi` có một điểm 120
(vượt 100) — `traverse` dừng NGAY ở đó, trả `khong()`, `gopTatCa` không
bao giờ được gọi. Bốn khái niệm — Catamorphism, Functor, Traverse, Monoid
— đứng trong CÙNG một hàm chín dòng, không xung đột.
::::

::::predict{#doan-truong-hop-mang-rong commitOnce}
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

type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) { case "co": return co(f(o.giaTri)); case "khong": return khong(); }
}

function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T; }
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); }
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function tongDiemThuong(baiThi: Expr[]): Option<number> {
  const diem = baiThi.map((e) => tinh(e));
  const daKiemTra = traverse(diem, (d): Option<number> => (d >= 0 ? co(d) : khong()));
  return mapOption(daKiemTra, (mang) => gopTatCa(monoidCong, mang));
}

console.log(JSON.stringify(tongDiemThuong([])));
```

`baiThi` là mảng RỖNG — không có bài thi nào. Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"co","giaTri":0}`
:::

:::opt
`{"kind":"khong"}` — vì mảng rỗng không có điểm nào để kiểm, nên coi là
không hợp lệ
::why
Gần đúng ở việc bạn nhận ra mảng rỗng là một CA BIÊN cần suy nghĩ riêng —
nhiều hàm THẬT SỰ xử lý sai ca này nếu không cẩn thận.

Chỗ lệch: `traverse([], f)` — vòng lặp `for` KHÔNG chạy lần nào (không có
phần tử nào để đưa vào `f`), nên "không có phần tử nào thất bại" đúng một
cách TRỐNG RỖNG (vacuously) — hàm trả ngay `co([])`, không phải `khong()`.
`mapOption(co([]), mang => gopTatCa(monoidCong, mang))` sau đó chạy bình
thường trên mảng rỗng.
::
:::

:::opt
Máy báo lỗi runtime — `gopTatCa` không gộp được mảng rỗng, `reduce` cần
ít nhất MỘT phần tử để có giá trị khởi đầu
::why
Gần đúng ở việc `Array.prototype.reduce` TRẦN (không truyền giá trị khởi
tạo) THẬT SỰ báo lỗi trên mảng rỗng — quan sát đó đúng với `reduce` KHÔNG
có đối số thứ hai.

Chỗ lệch: `gopTatCa` LUÔN truyền `m.rong` làm giá trị khởi tạo cho
`reduce` (`ds.reduce(m.ketHop, m.rong)` — bài 5 đã đo đúng: mảng rỗng vẫn
ra đúng giá trị trung tính). Không có lỗi nào — `gopTatCa(monoidCong, [])`
trả về ĐÚNG `rong`, tức `0`. Đây chính là LÝ DO một Monoid cần một giá trị
trung tính: để "gộp mảng rỗng" luôn có nghĩa.
::
:::
::::

::::code{#tong_diem_thuong}
Viết hoàn chỉnh `tongDiemThuong(baiThi: Expr[]): Option<number>` — MỖI
điểm phải KHÔNG ÂM (`>= 0`) mới hợp lệ; nếu có điểm âm, kết quả là
`khong()`, không cộng gì cả. Ghép đủ bốn khái niệm: Catamorphism (`tinh`,
đã có sẵn), Functor (`baiThi.map`, đã có sẵn), Traverse (chỗ bạn viết),
Monoid (`gopTatCa`, đã có sẵn).

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
  foldExpr(e, { so: (g) => g, cong: (t, p) => t + p, nhan: (t, p) => t * p });

type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) { case "co": return co(f(o.giaTri)); case "khong": return khong(); }
}

function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T; }
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); }
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function tongDiemThuong(baiThi: Expr[]): Option<number> {
  const diem = baiThi.map((e) => tinh(e));
  const daKiemTra = traverse(diem, (d): Option<number> => ___);
  return mapOption(daKiemTra, (mang) => gopTatCa(monoidCong, mang));
}

const hopLe: Expr[] = [
  { kind: "cong", trai: { kind: "so", giaTri: 6 }, phai: { kind: "so", giaTri: 2 } },
  { kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 2 } },
];

const coDiemAm: Expr[] = [
  { kind: "cong", trai: { kind: "so", giaTri: 10 }, phai: { kind: "so", giaTri: -15 } },
  { kind: "so", giaTri: 4 },
];

console.log("hop le:", JSON.stringify(tongDiemThuong(hopLe)));
console.log("co diem am:", JSON.stringify(tongDiemThuong(coDiemAm)));
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
  foldExpr(e, { so: (g) => g, cong: (t, p) => t + p, nhan: (t, p) => t * p });

type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) { case "co": return co(f(o.giaTri)); case "khong": return khong(); }
}

function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T; }
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); }
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function tongDiemThuong(baiThi: Expr[]): Option<number> {
  const diem = baiThi.map((e) => tinh(e));
  const daKiemTra = traverse(diem, (d): Option<number> => (d >= 0 ? co(d) : khong()));
  return mapOption(daKiemTra, (mang) => gopTatCa(monoidCong, mang));
}

const hopLe: Expr[] = [
  { kind: "cong", trai: { kind: "so", giaTri: 6 }, phai: { kind: "so", giaTri: 2 } },
  { kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 2 } },
];

const coDiemAm: Expr[] = [
  { kind: "cong", trai: { kind: "so", giaTri: 10 }, phai: { kind: "so", giaTri: -15 } },
  { kind: "so", giaTri: 4 },
];

console.log("hop le:", JSON.stringify(tongDiemThuong(hopLe)));
console.log("co diem am:", JSON.stringify(tongDiemThuong(coDiemAm)));
```

```typescript title=test
if (JSON.stringify(tongDiemThuong(hopLe)) !== JSON.stringify(co(14))) throw new Error("tongDiemThuong(hopLe) phải ra co(14) — 8 + 6 = 14, cả hai điểm đều không âm");
if (tongDiemThuong(coDiemAm).kind !== "khong") throw new Error("coDiemAm có một điểm âm (10 + -15 = -5) — kết quả phải là khong()");
if (JSON.stringify(tongDiemThuong([{ kind: "so", giaTri: 0 }])) !== JSON.stringify(co(0))) throw new Error("điểm bằng 0 vẫn hợp lệ (không âm) — phải ra co(0)");
```

:::hints
- kind: attention
  body: "Chỗ trống phải trả về MỘT Option<number> hoàn chỉnh (co(...) HOẶC khong()), không phải một boolean hay một số trần. Điều kiện hợp lệ là KHÔNG ÂM — dùng >=, không phải >."
- kind: strategy
  body: "Đúng khuôn traverse đã học ở bài 36: (d): Option<number> => (dieu_kien ? co(d) : khong()). Điều kiện ở đây: d >= 0 — điểm âm thì khong(), điểm không âm (kể cả 0) thì co(d)."
- kind: one-line
  body: "(d >= 0 ? co(d) : khong())"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: 'co diem am: {"kind":"khong"}'
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn khái niệm — Catamorphism, Functor, Traverse, Monoid — vừa chạy chung
trong MỘT hàm, không xung đột. BOSS (bài sau) đòi đủ SÁU.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài này ghép bốn trên sáu khái niệm: Catamorphism, Functor, Traverse,
Monoid. Còn thiếu Applicative (kết hợp NHIỀU giá trị ĐỘC LẬP, gom lỗi —
cụm 4) và Monad (chuỗi PHỤ THUỘC qua `chain` — cụm 5). Bài 50 từng ghép
ĐÚNG hai khái niệm đó với nhau, nhưng CHƯA đứng chung khung với bốn khái
niệm còn lại trong CÙNG một chương trình.

Bài sau là BOSS — đòi ĐỦ cả sáu khái niệm, trong MỘT chương trình DUY
NHẤT. Không có khái niệm mới nào chờ ở đó — chỉ còn việc ghép TRỌN VẸN.
::::

::::checkpoint{mastery=0.8}
::::
