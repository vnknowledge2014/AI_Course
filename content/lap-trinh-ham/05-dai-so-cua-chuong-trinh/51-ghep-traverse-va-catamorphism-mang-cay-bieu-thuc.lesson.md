---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.ghep-traverse-va-catamorphism-mang-cay-bieu-thuc
title: "Ghép Traverse + Catamorphism: một MẢNG cây biểu thức, tính TOÀN BỘ hoặc báo lỗi"
summary: "dsExpr.map(tinh) tính TOÀN BỘ một mảng cây biểu thức thành number[] — Catamorphism (foldExpr bên trong tinh) ghép với Functor (map trên mảng). tinh không bao giờ lỗi nên map là đủ; nếu tinh CÓ THỂ lỗi, cần traverse (cụm 6) thay vì map."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 51
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.combine-traverse-catamorphism]
requires: [alg.combine-applicative-monad]
concepts: [alg.combine-traverse-catamorphism]
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
Bài trước ghép Applicative với Monad — hai bước ĐỘC LẬP rồi một bước
PHỤ THUỘC, trong MỘT form. Hôm nay đổi bài toán: không phải MỘT giá
trị nữa, mà MỘT MẢNG — nhiều cây biểu thức, muốn tính HẾT.
::::

::::explain{#tinh-toan-bo-mang-expr}
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

const e1: Expr = { kind: "so", giaTri: 5 };
const e2: Expr = { kind: "cong", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } };
const e3: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
  phai: { kind: "so", giaTri: 4 },
};

const dsExpr: Expr[] = [e1, e2, e3];

const dsKetQua: number[] = dsExpr.map(tinh);
console.log(JSON.stringify(dsKetQua));
```

```text
[5,7,20]
```

`dsExpr` là một MẢNG ba cây biểu thức khác nhau — `e1` chỉ là một số,
`e2` là một phép cộng, `e3` là một phép nhân LỒNG một phép cộng bên
trong. `tinh` (dựng từ `foldExpr` ở cụm 6 — Catamorphism) tính ĐÚNG
MỘT cây, trả về MỘT `number`. Muốn tính CẢ BA cây cùng lúc, không cần
công cụ MỚI nào: `dsExpr.map(tinh)` — `Array.prototype.map` (Functor,
bài 16) áp `tinh` lên TỪNG phần tử của mảng, giữ nguyên "vỏ mảng", ra
`number[]`.

Đây CHÍNH LÀ phép ghép mà bài này dạy: Catamorphism (`foldExpr` nằm
BÊN TRONG `tinh`, tính TỪNG cây) cộng với Functor (`map` LẶP qua CẢ
mảng, gọi `tinh` một lần cho mỗi cây). Chưa cần đụng tới Traverse
(cụm 6) — lý do vì sao thì phần tiếp theo làm rõ.
::::

::::example{#khi-tinh-co-the-loi title=readonly}
```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

const chiaCho2 = (n: number): Result<number, string> =>
  n % 2 === 0 ? ok(n / 2) : loi(`${n} không chia hết cho 2`);

const dsSo: number[] = [10, 7, 20];

const dsKetQua: Result<number, string>[] = dsSo.map(chiaCho2);
console.log(JSON.stringify(dsKetQua));
```

```text title=readonly
[{"kind":"ok","giaTri":5},{"kind":"loi","loi":"7 không chia hết cho 2"},{"kind":"ok","giaTri":10}]
```

Đây là chỗ `.map()` KHÔNG còn đủ. `chiaCho2` CÓ THỂ lỗi (số lẻ không
chia hết cho `2`) — tự nó đã trả về `Result<number, string>`, không
phải `number` trần (đúng vấn đề "map tạo ra lồng" bài 28 đã gặp, chỉ
khác chỗ này lồng qua MỘT MẢNG chứ không phải một `Option` đơn). Kết
quả `dsSo.map(chiaCho2)` là MỘT MẢNG BA `Result` RIÊNG LẺ — mỗi phần
tử tự báo "ok" hay "loi" của CHÍNH NÓ, không có cách nào đọc trực tiếp
"CẢ mảng có ổn không" mà không lặp qua kiểm từng phần tử.

Cái ta THƯỜNG muốn là ngược lại: MỘT `Result<number[], string>` duy
nhất — "cả mảng tính được" hoặc "có một phần tử lỗi, đây là lỗi đó".
Đó CHÍNH LÀ bài toán `traverse` (cụm 6, bài 36) giải — lật
`Array<Result<_>>` thành `Result<Array<_>>`. `tinh` ở bài này (tính
`Expr` chỉ gồm `"so"`/`"cong"`/`"nhan"`) không rơi vào tình huống này
vì nó KHÔNG BAO GIỜ lỗi — không nhánh nào của `foldExpr` trả `Result`.
Nếu `Expr` có thêm một biến thể CÓ THỂ lỗi (ví dụ phép chia, chia cho
`0`), `tinh` sẽ phải trả `Result<number, string>`, và ghép nó với một
mảng cây sẽ cần `traverse` — không phải `.map()` thường như bài này.
::::

::::predict{#doan-map-tinh-tren-mang commitOnce}
```typescript
const eA: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 0 }, phai: { kind: "so", giaTri: 100 } };
const eB: Expr = { kind: "cong", trai: { kind: "so", giaTri: 5 }, phai: { kind: "so", giaTri: 5 } };
const eC: Expr = { kind: "so", giaTri: 9 };

const ds: Expr[] = [eA, eB, eC];
console.log(JSON.stringify(ds.map(tinh)));
```

Dòng cuối in ra gì?

:::opt{correct}
`[0,10,9]`
:::

:::opt
`[10,9]` — vì phần tử ĐẦU (`nhan(0, 100)`) ra `0`, `.map()` coi đó là
một "thất bại" và bỏ qua, không đưa vào mảng kết quả
::why
Gần đúng ở việc bạn để ý `eA` cho ra một kết quả ĐẶC BIỆT — `0` — có
vẻ "khác thường" so với hai kết quả kia.

Chỗ lệch: `0` là một `number` HOÀN TOÀN HỢP LỆ, không phải tín hiệu
lỗi. `tinh` (dựng từ `foldExpr` trên `Expr` hiện tại) KHÔNG BAO GIỜ
thất bại, và `.map()` (Functor) áp `tinh` lên TỪNG phần tử ĐỘC LẬP,
không có short-circuit như `traverse` (bài 36) — không phần tử nào
"làm hỏng" hay "loại" phần tử khác. Kết quả có ĐỦ BA số: `[0, 10, 9]`.
::
:::

:::opt
`[100,10,9]` — vì `nhan(0, 100)` được tính như phép CỘNG, ra `100`
thay vì `0`
::why
Gần đúng ở việc bạn tính ĐÚNG hai cây sau — `cong(5, 5)` ra `10`,
`eC` (chỉ một số) ra `9`.

Chỗ lệch: `eA` là biến thể `"nhan"` (NHÂN, không phải CỘNG) — bên
trong `tinh`, nhánh `nhan: (t, p) => t * p` tính `t * p`, không phải
`t + p`.

`0 * 100 = 0`, không phải `0 + 100 = 100`.
::
:::
::::

::::code{#tinh-tat-ca-mang-expr}
Viết `tinhTatCa(ds: Expr[]): number[]` — tính GIÁ TRỊ của TỪNG cây
biểu thức trong mảng `ds`, dùng lại `tinh` (đã có sẵn, dựng từ
`foldExpr`) và `.map()`. Không viết `switch`/đệ quy mới nào.

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

function tinhTatCa(ds: Expr[]): number[] {
  return ___;
}

const e1: Expr = { kind: "so", giaTri: 5 };
const e2: Expr = { kind: "cong", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } };
const e3: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 6 } };

console.log(JSON.stringify(tinhTatCa([e1, e2, e3])));
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

function tinhTatCa(ds: Expr[]): number[] {
  return ds.map(tinh);
}

const e1: Expr = { kind: "so", giaTri: 5 };
const e2: Expr = { kind: "cong", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } };
const e3: Expr = { kind: "nhan", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 6 } };

console.log(JSON.stringify(tinhTatCa([e1, e2, e3])));
```

```typescript title=test
if (JSON.stringify(tinhTatCa([e1])) !== JSON.stringify([5])) throw new Error("tinhTatCa phải tính đúng cây 'so' đơn lẻ");
if (JSON.stringify(tinhTatCa([e2])) !== JSON.stringify([7])) throw new Error("tinhTatCa phải tính đúng cây 'cong' (3 + 4 = 7)");
if (JSON.stringify(tinhTatCa([e3])) !== JSON.stringify([12])) throw new Error("tinhTatCa phải tính đúng cây 'nhan' (2 * 6 = 12)");
if (JSON.stringify(tinhTatCa([e1, e2, e3])) !== JSON.stringify([5, 7, 12])) throw new Error("tinhTatCa phải giữ ĐÚNG thứ tự — kết quả từng cây khớp vị trí cây đó trong mảng đầu vào");
if (JSON.stringify(tinhTatCa([])) !== JSON.stringify([])) throw new Error("mảng Expr[] rỗng phải cho ra mảng number[] rỗng");
```

:::hints
- kind: attention
  body: "Không viết switch/đệ quy tay mới ở đây — tinh ĐÃ dựng xong từ foldExpr rồi. Việc còn lại CHỈ là áp tinh lên TỪNG phần tử của ds."
- kind: strategy
  body: "ds là một Expr[] — cần một number[] cùng thứ tự, mỗi phần tử là tinh(cây đó). Đây đúng khuôn Array.prototype.map (Functor, bài 16): ds.map(tinh)."
- kind: one-line
  body: "return ds.map(tinh);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "[5,7,12]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không viết `switch` mới, không viết đệ quy mới — chỉ MỘT `.map(tinh)`.
Catamorphism (`foldExpr` bên trong `tinh`) cộng Functor (`map` trên
mảng) — hai khuôn nhỏ, ghép lại giải một bài mới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track T4.1-T4.2 (Python) đã dùng `reduce`, `compose`, `map` — RẤT LÂU
trước khi track này đặt tên "Monoid", "Functor", "Catamorphism". Có
phải những khuôn Python ĐÓ chính là những khuôn TypeScript vừa học,
chỉ khác NGÔN NGỮ diễn đạt?

Bài sau đối chiếu ngược lại hai track Python — TÊN GỌI mới, khuôn thì
đã dùng từ lâu.
::::

::::checkpoint{mastery=0.8}
::::
