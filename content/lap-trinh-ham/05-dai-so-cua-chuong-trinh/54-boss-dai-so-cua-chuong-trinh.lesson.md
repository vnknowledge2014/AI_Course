---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.boss-dai-so-cua-chuong-trinh
title: "BOSS — Khép track Đại số của chương trình"
summary: "`tongKetBaiThi` — một hàm nhỏ dùng ĐỦ sáu khái niệm: Traverse (parse một mảng chuỗi, dừng sớm nếu lỗi), Monoid (cộng dồn), Functor + Catamorphism (`.map()` áp `foldExpr` lên từng `Expr`), Applicative (`map2GomLoi` gom lỗi độc lập), Monad (`chainResult` một bước phụ thuộc, tự nó cũng có thể lỗi). Không khái niệm mới — chỉ ghép lại."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 54
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [alg.gate-boss]
requires: [alg.gate-review]
concepts: [alg.gate-boss]
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
Bài trước đo bạn dùng ÍT NHẤT bốn trong sáu khái niệm. Hôm nay là
BOSS — dùng ĐỦ CẢ SÁU, trong đúng MỘT chương trình nhỏ.
::::

::::explain{#tong-ket-bai-thi-du-sau-khai-niem}
Một buổi thi nhỏ: một mảng điểm từng câu hỏi (dạng CHUỖI, cần parse),
và một mảng biểu thức "điểm thưởng" (`Expr` — đúng ADT track T4.3 đã
tự thiết kế, dùng lại xuyên suốt track này). `tongKetBaiThi` tính ra
MỘT con số cuối, hoặc một DANH SÁCH lỗi nếu có gì sai — ghép ĐỦ sáu
khái niệm, mỗi khái niệm đúng MỘT vai trò nó đã học:

```typescript
function tongKetBaiThi(diemTho: string[], bieuThucThuong: Expr[]): Result<number, string[]> {
  // Traverse: lật Array<Option<number>> thành Option<Array<number>>,
  // dừng ngay nếu MỘT chuỗi không parse được.
  const diemCauHoi = traverse(diemTho, chuyenSo);
  if (diemCauHoi.kind === "khong") return loiMot("một điểm câu hỏi không hợp lệ");

  // Monoid: cộng dồn một danh sách number thành MỘT number.
  const tongCauHoi = gopTatCa(monoidCong, diemCauHoi.giaTri);

  // Functor + Catamorphism: .map() biến đổi TỪNG Expr thành number
  // bằng foldExpr, rồi Monoid cộng dồn lại như trên.
  const diemThuongDs = bieuThucThuong.map((e) => foldExpr(e, boXuLyTinh));
  const tongThuong = gopTatCa(monoidCong, diemThuongDs);

  // Applicative: hai điều kiện ĐỘC LẬP, gom CẢ HAI lỗi nếu cả hai sai.
  const hopLe = map2GomLoi(
    kiemTraKhoang("điểm câu hỏi", tongCauHoi, 0, 100),
    kiemTraKhoang("điểm thưởng", tongThuong, 0, 20),
    (a, b) => a + b
  );

  // Monad: bước cuối PHỤ THUỘC kết quả đã hợp lệ ở trên, và CHÍNH
  // bước này cũng có thể lỗi (chia cho 0 câu hỏi).
  return chainResult(hopLe, (tong) => chiaDeu(tong, diemTho.length));
}

const expr1: Expr = { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } };
const expr2: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 4 } };

console.log(JSON.stringify(tongKetBaiThi(["8", "9", "10"], [expr1, expr2])));
```

```text
{"kind":"ok","giaTri":12}
```

`diemTho = ["8", "9", "10"]` — cả ba parse được, TRAVERSE ra
`co([8, 9, 10])`, không dừng sớm. `tongCauHoi = gopTatCa(monoidCong,
[8, 9, 10])` — MONOID cộng dồn — ra `27`. `expr1` là `2 + 3` (`5`),
`expr2` là `1 * 4` (`4`) — `.map()` (FUNCTOR trên mảng) áp `foldExpr`
(CATAMORPHISM) lên TỪNG cây, ra `[5, 4]`, rồi lại MONOID cộng dồn —
`9`. `map2GomLoi` (APPLICATIVE) kiểm hai điều kiện ĐỘC LẬP — `27`
nằm trong `[0, 100]`, `9` nằm trong `[0, 20]` — cả hai đúng, ra
`ok(36)`. Cuối cùng `chainResult` (MONAD) đưa `36` vào `chiaDeu(36,
3)` — bước PHỤ THUỘC kết quả vừa hợp lệ, và chính bước đó cũng có thể
lỗi — ra `ok(12)`.

Sáu khái niệm, mỗi khái niệm đúng một việc nó đã học, không việc nào
thừa, không việc nào thiếu.
::::

::::example{#khi-mot-buoc-that-bai}
Ghép sáu khái niệm không chỉ để CHẠY ĐÚNG khi mọi thứ suôn sẻ — mà để
lỗi Ở BẤT KỲ bước nào cũng được XỬ LÝ đúng chỗ. Hai tình huống khác:

```typescript title=readonly
function tongKetBaiThi(diemTho: string[], bieuThucThuong: Expr[]): Result<number, string[]> {
  const diemCauHoi = traverse(diemTho, chuyenSo);
  if (diemCauHoi.kind === "khong") return loiMot("một điểm câu hỏi không hợp lệ");

  const tongCauHoi = gopTatCa(monoidCong, diemCauHoi.giaTri);

  const diemThuongDs = bieuThucThuong.map((e) => foldExpr(e, boXuLyTinh));
  const tongThuong = gopTatCa(monoidCong, diemThuongDs);

  const hopLe = map2GomLoi(
    kiemTraKhoang("điểm câu hỏi", tongCauHoi, 0, 100),
    kiemTraKhoang("điểm thưởng", tongThuong, 0, 20),
    (a, b) => a + b
  );

  return chainResult(hopLe, (tong) => chiaDeu(tong, diemTho.length));
}

const expr1: Expr = { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } };
const expr2: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 4 } };

console.log(JSON.stringify(tongKetBaiThi(["8", "abc", "10"], [expr1, expr2])));
console.log(JSON.stringify(tongKetBaiThi([], [])));
```

```text title=readonly
{"kind":"loi","loi":["một điểm câu hỏi không hợp lệ"]}
{"kind":"loi","loi":["không thể chia cho 0 câu hỏi"]}
```

Dòng ĐẦU — `"abc"` không parse được — TRAVERSE thất bại NGAY, hàm
`return` sớm tại dòng kiểm `diemCauHoi.kind === "khong"`. Toàn bộ
phần SAU (Monoid, Functor, Catamorphism, Applicative, Monad) KHÔNG hề
chạy — không có lý do tính điểm thưởng hay kiểm khoảng hợp lệ khi
điểm câu hỏi còn chưa đọc được.

Dòng HAI — mảng RỖNG cả hai phía — TRAVERSE trên mảng rỗng ra
`co([])` (bài 5, "mảng rỗng vẫn ra đúng giá trị trung tính"), MONOID
cộng dồn mảng rỗng ra `0`, Applicative kiểm `0` nằm trong cả hai
khoảng (đúng), `hopLe = ok(0)`. Pipeline chạy HẾT mọi bước TRƯỚC — chỉ
lỗi Ở BƯỚC CUỐI CÙNG, khi MONAD gọi `chiaDeu(0, 0)` (chia cho `0` câu
hỏi). Hai lỗi, hai NGUỒN khác hẳn nhau — một lỗi dừng NGAY từ
Traverse, một lỗi chỉ lộ ra SAU KHI mọi bước trước đã qua.
::::

::::predict{#doan-buoc-nao-chay-truoc commitOnce}
```typescript
function tongKetBaiThi(diemTho: string[], bieuThucThuong: Expr[]): Result<number, string[]> {
  const diemCauHoi = traverse(diemTho, chuyenSo);
  if (diemCauHoi.kind === "khong") return loiMot("một điểm câu hỏi không hợp lệ");

  const tongCauHoi = gopTatCa(monoidCong, diemCauHoi.giaTri);

  const diemThuongDs = bieuThucThuong.map((e) => foldExpr(e, boXuLyTinh));
  const tongThuong = gopTatCa(monoidCong, diemThuongDs);

  const hopLe = map2GomLoi(
    kiemTraKhoang("điểm câu hỏi", tongCauHoi, 0, 100),
    kiemTraKhoang("điểm thưởng", tongThuong, 0, 20),
    (a, b) => a + b
  );

  return chainResult(hopLe, (tong) => chiaDeu(tong, diemTho.length));
}

const expr1: Expr = { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } };
const expr2: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 4 } };

console.log(JSON.stringify(tongKetBaiThi(["8", "abc", "10"], [expr1, expr2])));
```

Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"loi","loi":["một điểm câu hỏi không hợp lệ"]}`
:::

:::opt
`{"kind":"khong"}`
::why
Gần đúng ở việc bạn nhận ra ĐÚNG rằng `traverse` (bước Traverse) trả
về một `Option<number[]>`, và `"abc"` khiến `chuyenSo("abc")` ra
`khong()`, nên `traverse` THẬT SỰ ra `khong()` — quan sát đó khớp
đúng dòng đầu hàm chạy.

Chỗ lệch: `tongKetBaiThi` khai kiểu trả về là `Result<number,
string[]>`, không phải `Option<...>`. Dòng `if (diemCauHoi.kind ===
"khong") return loiMot(...)` CHUYỂN trạng thái `"khong"` của `Option`
THÀNH một `Result` dạng `"loi"` có thông điệp rõ ràng — không trả
thẳng `khong()` ra ngoài. Kết quả CHẮC CHẮN có `"kind":"loi"`, không
phải `"kind":"khong"`.
::
:::

:::opt
Máy báo lỗi lúc CHẠY (runtime crash) — vì `diemCauHoi.giaTri` được
đọc trong khi `diemCauHoi` có thể đang ở dạng `"khong"` (không có
`giaTri`)
::why
Gần đúng ở việc bạn cảnh giác đúng chỗ: đọc `.giaTri` trên một
`Option` khi nó CÓ THỂ đang ở dạng `"khong"` THẬT SỰ là nguy hiểm —
nếu code không kiểm `kind` trước, đây đúng là lỗi cần tránh.

Chỗ lệch: `tongKetBaiThi` ĐÃ kiểm `if (diemCauHoi.kind === "khong")
return ...;` NGAY sau khi gọi `traverse`, RỒI mới đọc
`diemCauHoi.giaTri` (dòng `tongCauHoi = gopTatCa(...)`). Sau dòng
kiểm đó, TypeScript tự THU HẸP kiểu `diemCauHoi` về đúng nhánh `{
kind: "co"; giaTri: number[] }` (pattern narrowing, T4.3) — không có
lỗi runtime nào, hàm chỉ đơn giản `return` SỚM trước khi tới dòng đọc
`giaTri`.
::
:::
::::

::::code{#tong_ket_bai_thi}
Hoàn thiện `tongKetBaiThi` — hàm đã dùng Traverse, Monoid, Applicative
và Monad. HAI chỗ trống còn thiếu đúng phần Functor + Catamorphism
(tính từng biểu thức thưởng) và phần Monad (bước phụ thuộc cuối
cùng, tự nó cũng có thể lỗi).

```typescript title=starter
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T; }
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); }
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

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

const boXuLyTinh: BoXuLyExpr<number> = {
  so: (g) => g,
  cong: (t, p) => t + p,
  nhan: (t, p) => t * p,
};

function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

function kiemTraKhoang(ten: string, giaTri: number, min: number, max: number): Result<number, string[]> {
  if (giaTri < min || giaTri > max) return loiMot(ten + " phải nằm trong [" + min + ", " + max + "]");
  return ok(giaTri);
}

function chiaDeu(tong: number, soPhan: number): Result<number, string[]> {
  if (soPhan === 0) return loiMot("không thể chia cho 0 câu hỏi");
  return ok(tong / soPhan);
}

function tongKetBaiThi(diemTho: string[], bieuThucThuong: Expr[]): Result<number, string[]> {
  const diemCauHoi = traverse(diemTho, chuyenSo);
  if (diemCauHoi.kind === "khong") return loiMot("một điểm câu hỏi không hợp lệ");

  const tongCauHoi = gopTatCa(monoidCong, diemCauHoi.giaTri);

  const diemThuongDs = bieuThucThuong.map((e) => ___);
  const tongThuong = gopTatCa(monoidCong, diemThuongDs);

  const hopLe = map2GomLoi(
    kiemTraKhoang("điểm câu hỏi", tongCauHoi, 0, 100),
    kiemTraKhoang("điểm thưởng", tongThuong, 0, 20),
    (a, b) => a + b
  );

  return chainResult(hopLe, (tong) => ___);
}

const expr1: Expr = { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } };
const expr2: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 4 } };

const ketQua = tongKetBaiThi(["8", "9", "10"], [expr1, expr2]);
if (ketQua.kind === "ok") {
  console.log("tong hop le: " + ketQua.giaTri);
}
```

```typescript title=solution
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }

function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T; }
function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); }
const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}

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

const boXuLyTinh: BoXuLyExpr<number> = {
  so: (g) => g,
  cong: (t, p) => t + p,
  nhan: (t, p) => t * p,
};

function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

function kiemTraKhoang(ten: string, giaTri: number, min: number, max: number): Result<number, string[]> {
  if (giaTri < min || giaTri > max) return loiMot(ten + " phải nằm trong [" + min + ", " + max + "]");
  return ok(giaTri);
}

function chiaDeu(tong: number, soPhan: number): Result<number, string[]> {
  if (soPhan === 0) return loiMot("không thể chia cho 0 câu hỏi");
  return ok(tong / soPhan);
}

function tongKetBaiThi(diemTho: string[], bieuThucThuong: Expr[]): Result<number, string[]> {
  const diemCauHoi = traverse(diemTho, chuyenSo);
  if (diemCauHoi.kind === "khong") return loiMot("một điểm câu hỏi không hợp lệ");

  const tongCauHoi = gopTatCa(monoidCong, diemCauHoi.giaTri);

  const diemThuongDs = bieuThucThuong.map((e) => foldExpr(e, boXuLyTinh));
  const tongThuong = gopTatCa(monoidCong, diemThuongDs);

  const hopLe = map2GomLoi(
    kiemTraKhoang("điểm câu hỏi", tongCauHoi, 0, 100),
    kiemTraKhoang("điểm thưởng", tongThuong, 0, 20),
    (a, b) => a + b
  );

  return chainResult(hopLe, (tong) => chiaDeu(tong, diemTho.length));
}

const expr1: Expr = { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } };
const expr2: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 4 } };

const ketQua = tongKetBaiThi(["8", "9", "10"], [expr1, expr2]);
if (ketQua.kind === "ok") {
  console.log("tong hop le: " + ketQua.giaTri);
}
```

```typescript title=test
const exprC: Expr = { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } };
const exprD: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 1 }, phai: { kind: "so", giaTri: 4 } };

const kq1 = tongKetBaiThi(["8", "9", "10"], [exprC, exprD]);
if (kq1.kind !== "ok") throw new Error("với điểm câu hỏi và điểm thưởng đều hợp lệ, tongKetBaiThi phải trả về 'ok'");
if (kq1.giaTri !== 12) throw new Error("kết quả hợp lệ phải là 12 — (8+9+10 cộng 5+4) chia cho 3 câu hỏi");

const kq2 = tongKetBaiThi(["8", "abc", "10"], [exprC, exprD]);
if (kq2.kind !== "loi") throw new Error("một điểm câu hỏi không phải số phải khiến traverse thất bại và trả về 'loi'");
if (kq2.loi.length !== 1 || !kq2.loi.join(" ").includes("không hợp lệ")) throw new Error("lỗi điểm câu hỏi không hợp lệ phải có đúng một thông điệp nhắc 'không hợp lệ'");

const kq3 = tongKetBaiThi(["200", "9", "10"], [{ kind: "so", giaTri: 500 }]);
if (kq3.kind !== "loi") throw new Error("điểm câu hỏi 219 vượt quá 100 VÀ điểm thưởng 500 vượt quá 20 — cả hai đều phải bị báo lỗi");
if (kq3.loi.length !== 2) throw new Error("map2GomLoi phải GOM CẢ HAI lỗi độc lập, không dừng ở lỗi đầu tiên — mảng lỗi phải có đúng 2 phần tử");

const kq4 = tongKetBaiThi([], []);
if (kq4.kind !== "loi") throw new Error("không có câu hỏi nào (mảng rỗng) khiến chia cho 0 câu — phải là 'loi'");
if (!kq4.loi.join(" ").includes("chia cho 0")) throw new Error("lỗi ở bước cuối phải nhắc tới việc chia cho 0 câu hỏi");
```

:::hints
- kind: attention
  body: "Hai chỗ trống là HAI THÂN HÀM MŨI TÊN, không phải hai số rời. Chỗ 1 nằm trong .map((e) => ___) — phải trả về một number tính từ e (một Expr). Chỗ 2 nằm trong chainResult(hopLe, (tong) => ___) — phải trả về một Result<number, string[]>, không phải một number trần."
- kind: strategy
  body: "Chỗ 1: bạn đã có foldExpr và boXuLyTinh (một BoXuLyExpr<number> tính giá trị số học) — gọi foldExpr(e, boXuLyTinh) cho TỪNG e trong .map(). Chỗ 2: bước cuối PHỤ THUỘC tong (kết quả Applicative vừa hợp lệ) và có thể lỗi (chia cho 0 câu) — hàm chiaDeu(tong, soPhan) đã viết sẵn ở trên, gọi chiaDeu(tong, diemTho.length)."
- kind: one-line
  body: "foldExpr(e, boXuLyTinh) và chiaDeu(tong, diemTho.length)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "tong hop le: 12"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Monoid, Functor, Applicative, Monad, Traverse, Catamorphism — cả sáu,
trong MỘT hàm. Track dài nhất Realm 4, khép lại đúng ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track sau (T4.6) giới thiệu `Parser<T>` — một hàm nhận một CHUỖI, trả
về HOẶC một giá trị `T` (cộng phần chuỗi CHƯA đọc), HOẶC một lỗi. Đó
cũng là một "hộp chứa" — y hệt `Option<T>`/`Result<T, E>` bạn vừa gom
lại bằng sáu khuôn hôm nay.

`Parser<T>` có `map` được không? `chain` được không? Sáu khuôn vừa
ghép — có áp dụng LÊN NÓ được không, hay `Parser<T>` cần thêm điều gì
khác? Track sau trả lời, bằng Rust.
::::

::::checkpoint{mastery=0.85}
::::
