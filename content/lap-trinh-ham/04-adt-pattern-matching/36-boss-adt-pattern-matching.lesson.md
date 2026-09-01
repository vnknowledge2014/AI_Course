---
id: lap-trinh-ham.adt-pattern-matching.boss-adt-pattern-matching
title: "BOSS — Khép track ADT & Pattern Matching"
summary: "Viết một chương trình nhỏ dùng ĐỦ sáu mảnh của track: discriminant field, switch + assertNever exhaustiveness, sum-of-products lồng nhau, union thay cờ boolean để illegal states unrepresentable, branded type + smart constructor, và một ADT đệ quy. Khép track — không dạy khái niệm mới, chỉ đòi ghép lại."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 36
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.adt-gate-boss]
requires: [ts.adt-gate-review]
concepts: [ts.adt-gate-boss]
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
Ba mươi lăm bài để tới đây. Một Ý gốc xuyên suốt tất cả: một kiểu dữ
liệu có thể mang NHIỀU HÌNH DẠNG khác nhau — và nếu trình biên dịch
được cho biết ĐỦ hình dạng, nó tự tìm ra chỗ code CHƯA xử lý hết. Giờ
ghép hết lại: một chương trình duy nhất, dùng đủ sáu mảnh track này đã
dựng.
::::

::::explain{#sau-manh-ghep}
Sáu mảnh, tất cả cùng phục vụ đúng một Ý:

1. **Discriminant field** (cụm 1) — mỗi biến thể tự mang một field
   `kind` kiểu literal, phân biệt được lúc chạy mà không cần đoán qua
   field nào có mặt.
2. **`switch` + `assertNever`** (cụm 2) — mỗi `case` tự NARROW kiểu;
   nhánh `default` gọi một hàm chỉ nhận `never`, buộc trình biên dịch
   tự báo lỗi nếu union được mở rộng mà code cũ chưa theo kịp.
3. **Sum-of-products lồng nhau** (cụm 3) — mỗi biến thể của một sum
   type (union) tự nó là một product type (nhiều field cùng lúc) —
   hình dạng THẬT của mọi ADT trong mã sản xuất.
4. **Union thay cờ boolean rời rạc** (cụm 4) — gộp các cờ/optional rời
   rạc thành MỘT union, một trạng thái vô nghĩa KHÔNG CÒN VIẾT RA ĐƯỢC,
   thay vì phải nhớ kiểm nó lúc chạy.
5. **Branded type + smart constructor** (cụm 5) — một kiểu gắn nhãn
   chỉ tồn tại lúc biên dịch; cách DUY NHẤT có một giá trị mang nhãn đó
   là gọi qua một hàm tự kiểm điều kiện hợp lệ trước.
6. **ADT đệ quy** (cụm 6) — một kiểu tự tham chiếu chính nó, biểu diễn
   một cấu trúc lồng bao nhiêu tầng cũng được; hàm xử lý nó cũng đệ
   quy, phản ánh đúng cấu trúc kiểu.

Sáu mảnh đó là toàn bộ nguyên liệu để viết một trình TÍNH BIỂU THỨC số
học nhỏ: mỗi số phải hợp lệ trước khi được dùng (mảnh 5), biểu thức
lồng bốn phép toán cộng/trừ/nhân/chia bao nhiêu tầng cũng được (mảnh
6), và chia cho 0 phải trả về một KẾT QUẢ LỖI rõ ràng — không phải một
số vô nghĩa âm thầm trôi qua (mảnh 4):

```typescript
type SoHopLe = number & { readonly __brand: "SoHopLe" };

function taoSoHopLe(n: number): SoHopLe | null {
  if (!Number.isFinite(n)) return null;
  return n as SoHopLe;
}

type Expr =
  | { kind: "so"; giaTri: SoHopLe }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "tru"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr }
  | { kind: "chia"; trai: Expr; phai: Expr };

function so(n: number): Expr | null {
  const gt = taoSoHopLe(n);
  if (gt === null) return null;
  return { kind: "so", giaTri: gt };
}

type KetQua =
  | { kind: "thanhCong"; giaTri: number }
  | { kind: "loi"; thongDiep: string };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý biến thể");
}

function tinhToan(e: Expr): KetQua {
  switch (e.kind) {
    case "so":
      return { kind: "thanhCong", giaTri: e.giaTri };
    case "cong": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri + p.giaTri };
    }
    case "tru": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri - p.giaTri };
    }
    case "nhan": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri * p.giaTri };
    }
    case "chia": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      if (p.giaTri === 0) return { kind: "loi", thongDiep: "chia cho 0" };
      return { kind: "thanhCong", giaTri: t.giaTri / p.giaTri };
    }
    default:
      return chuaXuLy(e);
  }
}

const a = so(3);
const b = so(4);
const hai = so(2);
if (a !== null && b !== null && hai !== null) {
  const congAB: Expr = { kind: "cong", trai: a, phai: b };
  const bieuThuc: Expr = { kind: "nhan", trai: congAB, phai: hai };
  console.log(tinhToan(bieuThuc));
}

const muoi = so(10);
const khong = so(0);
if (muoi !== null && khong !== null) {
  const chiaChoKhong: Expr = { kind: "chia", trai: muoi, phai: khong };
  console.log(tinhToan(chiaChoKhong));
}
```

```text
{"kind":"thanhCong","giaTri":14}
{"kind":"loi","thongDiep":"chia cho 0"}
```

Đọc ra đủ sáu mảnh trong đúng chương trình này. `type Expr = { kind:
"so"; ... } | { kind: "cong"; ... } | ...` — mỗi biến thể tự mang field
`kind` kiểu literal (mảnh 1). `tinhToan` là một `switch` trên `e.kind`,
nhánh `default` gọi `chuaXuLy(e)` — biên dịch được vì cả năm `case`
phía trên đã thu hẹp `e` còn `never` tại đó (mảnh 2). Biến thể
`"cong"`/`"tru"`/`"nhan"`/`"chia"` mỗi cái là một PRODUCT (`kind` cộng
`trai` cộng `phai`) bên trong sum type `Expr` (mảnh 3). `KetQua` là
`{ kind: "thanhCong"; giaTri: number } | { kind: "loi"; thongDiep:
string }` — không có cách nào viết ra một `KetQua` VỪA thành công VỪA
lỗi cùng lúc (mảnh 4). `SoHopLe` là branded type, `taoSoHopLe`/`so` là
smart constructor — cách DUY NHẤT có một `Expr` biến thể `"so"` là gọi
qua `so(n)`, tự kiểm `Number.isFinite` trước (mảnh 5). Và `Expr` TỰ
THAM CHIẾU chính nó qua `trai`/`phai`, `tinhToan` GỌI LẠI chính nó để
tính từng nhánh con trước khi gộp kết quả (mảnh 6).

Dòng cuối in `{"kind":"loi","thongDiep":"chia cho 0"}` — không phải
một số vô nghĩa nào. Nhánh `"chia"` TỰ kiểm `p.giaTri === 0` TRƯỚC KHI
chia, chặn phép chia trước khi nó kịp sinh ra một giá trị không hữu
hạn.
::::

::::example{#mo-rong-loi-hien-ngay-lan-cuoi}
Cụm 2 (bài 7–13) đã dạy: mở rộng một union, `assertNever` tự bắt đúng
chỗ code cũ chưa theo kịp. Lần CUỐI CÙNG của track này, đúng cơ chế đó
áp lên TOÀN BỘ chương trình vừa ghép — thêm một phép toán MỚI (luỹ
thừa) vào `Expr`, nhưng CHƯA sửa `tinhToan`:

```typescript title=readonly
type SoHopLe = number & { readonly __brand: "SoHopLe" };

function taoSoHopLe(n: number): SoHopLe | null {
  if (!Number.isFinite(n)) return null;
  return n as SoHopLe;
}

type Expr =
  | { kind: "so"; giaTri: SoHopLe }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "tru"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr }
  | { kind: "chia"; trai: Expr; phai: Expr }
  | { kind: "luyThua"; trai: Expr; phai: Expr };

function so(n: number): Expr | null {
  const gt = taoSoHopLe(n);
  if (gt === null) return null;
  return { kind: "so", giaTri: gt };
}

type KetQua =
  | { kind: "thanhCong"; giaTri: number }
  | { kind: "loi"; thongDiep: string };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý biến thể");
}

function tinhToan(e: Expr): KetQua {
  switch (e.kind) {
    case "so":
      return { kind: "thanhCong", giaTri: e.giaTri };
    case "cong": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri + p.giaTri };
    }
    case "tru": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri - p.giaTri };
    }
    case "nhan": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri * p.giaTri };
    }
    case "chia": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      if (p.giaTri === 0) return { kind: "loi", thongDiep: "chia cho 0" };
      return { kind: "thanhCong", giaTri: t.giaTri / p.giaTri };
    }
    default:
      return chuaXuLy(e);
  }
}

const a = so(3);
const b = so(4);
if (a !== null && b !== null) {
  console.log(tinhToan({ kind: "cong", trai: a, phai: b }));
}
```

```text title=readonly
(không in ra gì cả)

TS2345 (dòng 64, cột 23): Argument of type '{ kind: "luyThua"; trai:
Expr; phai: Expr; }' is not assignable to parameter of type 'never'.
```

Để ý: dòng gọi DUY NHẤT trong chương trình chỉ dùng `"cong"` — không hề
có `"luyThua"` thật nào được tạo ra hay chạy qua. Không quan trọng.
Ngay khi `"luyThua"` gia nhập union `Expr` mà `tinhToan` CHƯA có `case`
xử lý, kiểu của `e` tại `default` không còn là `never` nữa — trình biên
dịch bắt lỗi NGAY, trước khi bất kỳ dòng nào chạy. Thêm đúng
`case "luyThua"` xử lý phép luỹ thừa, lỗi biến mất — không cần sửa gì
khác, không cần grep tay tìm mọi chỗ chương trình dùng `Expr`.
::::

::::predict{#doan-lan-truyen-loi commitOnce}
```typescript
type SoHopLe = number & { readonly __brand: "SoHopLe" };

function taoSoHopLe(n: number): SoHopLe | null {
  if (!Number.isFinite(n)) return null;
  return n as SoHopLe;
}

type Expr =
  | { kind: "so"; giaTri: SoHopLe }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "tru"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr }
  | { kind: "chia"; trai: Expr; phai: Expr };

function so(n: number): Expr | null {
  const gt = taoSoHopLe(n);
  if (gt === null) return null;
  return { kind: "so", giaTri: gt };
}

type KetQua =
  | { kind: "thanhCong"; giaTri: number }
  | { kind: "loi"; thongDiep: string };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý biến thể");
}

function tinhToan(e: Expr): KetQua {
  switch (e.kind) {
    case "so":
      return { kind: "thanhCong", giaTri: e.giaTri };
    case "cong": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri + p.giaTri };
    }
    case "tru": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri - p.giaTri };
    }
    case "nhan": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri * p.giaTri };
    }
    case "chia": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      if (p.giaTri === 0) return { kind: "loi", thongDiep: "chia cho 0" };
      return { kind: "thanhCong", giaTri: t.giaTri / p.giaTri };
    }
    default:
      return chuaXuLy(e);
  }
}

const a = so(10);
const z = so(0);
const c = so(5);
if (a !== null && z !== null && c !== null) {
  const chia: Expr = { kind: "chia", trai: a, phai: z };
  const bieuThuc: Expr = { kind: "cong", trai: chia, phai: c };
  console.log(tinhToan(bieuThuc));
}
```

`bieuThuc` là `(10 / 0) + 5`. Khối `if` này in ra gì?

:::opt{correct}
`{"kind":"loi","thongDiep":"chia cho 0"}`
:::

:::opt
`{"kind":"thanhCong","giaTri":5}` — phép chia `10 / 0` thất bại ÂM
THẦM, coi như bằng `0`, rồi `0 + 5` cho ra `5`
::why
Gần đúng ở việc bạn nhận ra `10 / 0` có gì đó BẤT THƯỜNG, cần được xử
lý riêng — quan sát đó đúng, đúng lý do `case "chia"` có nhánh kiểm
`p.giaTri === 0`.

Chỗ lệch: `tinhToan` KHÔNG có cơ chế "âm thầm coi như 0" nào — nhánh
`"chia"` rẽ THẲNG sang biến thể `loi` tường minh
(`{ kind: "loi", thongDiep: "chia cho 0" }`), một biến thể HOÀN TOÀN
khác `thanhCong`. Và vì nhánh `"cong"` có `if (t.kind === "loi")
return t;` NGAY sau khi tính `trai`, kết quả `loi` đó bị TRẢ VỀ LẬP TỨC
— `phai` (giá trị `5`) không bao giờ được tính tới, không bao giờ được
cộng vào.
::
:::

:::opt
Chương trình ném lỗi lúc CHẠY (uncaught exception) — vì `chuaXuLy` sẽ
bị gọi khi gặp phép chia cho 0 không xử lý được
::why
Gần đúng ở việc bạn nhớ `chuaXuLy` CÓ `throw` một `Error` thật — đúng,
hàm đó có `throw new Error(...)`.

Chỗ lệch: `chuaXuLy` CHỈ được gọi ở nhánh `default` của `switch`, dành
cho biến thể CHƯA có `case` xử lý — `"chia"` đã có `case` riêng, không
bao giờ rơi vào `default`. Và phép chia `10 / 0` trong JavaScript
KHÔNG ném ngoại lệ nào — nó cho ra `Infinity`, một giá trị `number`
hợp lệ (không phải `NaN`). Chính vì runtime không TỰ báo lỗi, code
phải TỰ kiểm `p.giaTri === 0` bằng tay TRƯỚC KHI chia — đúng việc
`case "chia"` đang làm.
::
:::
::::

::::code{#chuong-trinh-boss}
Chương trình dưới đây đã ghép đủ sáu mảnh: discriminant field,
`switch` + `assertNever`, sum-of-products lồng nhau trong `Expr`,
`KetQua` thay cờ boolean rời rạc, `SoHopLe` branded type + smart
constructor, và chính `Expr` là một ADT đệ quy. Điền chỗ trống DUY
NHẤT: điều kiện phát hiện chia cho 0 TRƯỚC KHI phép chia thực sự chạy.

```typescript title=starter
type SoHopLe = number & { readonly __brand: "SoHopLe" };

function taoSoHopLe(n: number): SoHopLe | null {
  if (!Number.isFinite(n)) return null;
  return n as SoHopLe;
}

type Expr =
  | { kind: "so"; giaTri: SoHopLe }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "tru"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr }
  | { kind: "chia"; trai: Expr; phai: Expr };

function so(n: number): Expr | null {
  const gt = taoSoHopLe(n);
  if (gt === null) return null;
  return { kind: "so", giaTri: gt };
}

type KetQua =
  | { kind: "thanhCong"; giaTri: number }
  | { kind: "loi"; thongDiep: string };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý biến thể");
}

function tinhToan(e: Expr): KetQua {
  switch (e.kind) {
    case "so":
      return { kind: "thanhCong", giaTri: e.giaTri };
    case "cong": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri + p.giaTri };
    }
    case "tru": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri - p.giaTri };
    }
    case "nhan": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri * p.giaTri };
    }
    case "chia": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      if (___) return { kind: "loi", thongDiep: "chia cho 0" };
      return { kind: "thanhCong", giaTri: t.giaTri / p.giaTri };
    }
    default:
      return chuaXuLy(e);
  }
}

const a = so(3);
const b = so(4);
const hai = so(2);
if (a !== null && b !== null && hai !== null) {
  const congAB: Expr = { kind: "cong", trai: a, phai: b };
  const bieuThuc: Expr = { kind: "nhan", trai: congAB, phai: hai };
  console.log(tinhToan(bieuThuc));
}

const muoi = so(10);
const khong = so(0);
if (muoi !== null && khong !== null) {
  const chiaChoKhong: Expr = { kind: "chia", trai: muoi, phai: khong };
  console.log(tinhToan(chiaChoKhong));
}
```

```typescript title=solution
type SoHopLe = number & { readonly __brand: "SoHopLe" };

function taoSoHopLe(n: number): SoHopLe | null {
  if (!Number.isFinite(n)) return null;
  return n as SoHopLe;
}

type Expr =
  | { kind: "so"; giaTri: SoHopLe }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "tru"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr }
  | { kind: "chia"; trai: Expr; phai: Expr };

function so(n: number): Expr | null {
  const gt = taoSoHopLe(n);
  if (gt === null) return null;
  return { kind: "so", giaTri: gt };
}

type KetQua =
  | { kind: "thanhCong"; giaTri: number }
  | { kind: "loi"; thongDiep: string };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý biến thể");
}

function tinhToan(e: Expr): KetQua {
  switch (e.kind) {
    case "so":
      return { kind: "thanhCong", giaTri: e.giaTri };
    case "cong": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri + p.giaTri };
    }
    case "tru": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri - p.giaTri };
    }
    case "nhan": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      return { kind: "thanhCong", giaTri: t.giaTri * p.giaTri };
    }
    case "chia": {
      const t = tinhToan(e.trai);
      if (t.kind === "loi") return t;
      const p = tinhToan(e.phai);
      if (p.kind === "loi") return p;
      if (p.giaTri === 0) return { kind: "loi", thongDiep: "chia cho 0" };
      return { kind: "thanhCong", giaTri: t.giaTri / p.giaTri };
    }
    default:
      return chuaXuLy(e);
  }
}

const a = so(3);
const b = so(4);
const hai = so(2);
if (a !== null && b !== null && hai !== null) {
  const congAB: Expr = { kind: "cong", trai: a, phai: b };
  const bieuThuc: Expr = { kind: "nhan", trai: congAB, phai: hai };
  console.log(tinhToan(bieuThuc));
}

const muoi = so(10);
const khong = so(0);
if (muoi !== null && khong !== null) {
  const chiaChoKhong: Expr = { kind: "chia", trai: muoi, phai: khong };
  console.log(tinhToan(chiaChoKhong));
}
```

```typescript title=test
function layExpr(n: number): Expr {
  const e = so(n);
  if (e === null) throw new Error("so(" + n + ") phải hợp lệ trong test này — n hữu hạn");
  return e;
}

const bieuThuc1: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: layExpr(3), phai: layExpr(4) },
  phai: layExpr(2),
};
const kq1 = tinhToan(bieuThuc1);
if (kq1.kind !== "thanhCong") throw new Error("(3 + 4) * 2 phải thành công — đang là " + JSON.stringify(kq1));
if (kq1.giaTri !== 14) throw new Error("(3 + 4) * 2 phải bằng 14 — đang là " + kq1.giaTri);

const bieuThuc2: Expr = { kind: "tru", trai: layExpr(10), phai: layExpr(3) };
const kq2 = tinhToan(bieuThuc2);
if (kq2.kind !== "thanhCong") throw new Error("10 - 3 phải thành công — đang là " + JSON.stringify(kq2));
if (kq2.giaTri !== 7) throw new Error("10 - 3 phải bằng 7 — đang là " + kq2.giaTri);

const bieuThuc3: Expr = { kind: "chia", trai: layExpr(10), phai: layExpr(2) };
const kq3 = tinhToan(bieuThuc3);
if (kq3.kind !== "thanhCong") throw new Error("10 / 2 phải thành công — đang là " + JSON.stringify(kq3));
if (kq3.giaTri !== 5) throw new Error("10 / 2 phải bằng 5 — đang là " + kq3.giaTri);

const bieuThuc4: Expr = { kind: "chia", trai: layExpr(10), phai: layExpr(0) };
const kq4 = tinhToan(bieuThuc4);
if (kq4.kind !== "loi") throw new Error("10 / 0 phải trả về loi, không phải thanhCong — đang là " + JSON.stringify(kq4));
if (kq4.thongDiep !== "chia cho 0") throw new Error("10 / 0 phải có thongDiep là \"chia cho 0\" — đang là " + kq4.thongDiep);

const bieuThuc5: Expr = { kind: "cong", trai: bieuThuc4, phai: layExpr(5) };
const kq5 = tinhToan(bieuThuc5);
if (kq5.kind !== "loi") throw new Error("(10 / 0) + 5 phải LAN TRUYỀN lỗi, không được cộng thêm 5 — đang là " + JSON.stringify(kq5));
if (kq5.thongDiep !== "chia cho 0") throw new Error("(10 / 0) + 5 phải giữ nguyên thongDiep \"chia cho 0\" — đang là " + kq5.thongDiep);

if (so(NaN) !== null) throw new Error("so(NaN) phải trả về null — NaN không phải SoHopLe hợp lệ");
```

:::hints
- kind: attention
  body: "Chỗ trống là TOÀN BỘ điều kiện của if, ngay TRƯỚC dòng chia (t.giaTri / p.giaTri) — mục đích là chặn ĐÚNG trường hợp khiến phép chia sinh Infinity."
- kind: strategy
  body: "Chia cho 0 là trường hợp cần chặn — so sánh p.giaTri với đúng số 0 (number, không phải chuỗi \"0\"), dùng ===."
- kind: one-line
  body: "Chỗ trống là: p.giaTri === 0"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "chia cho 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu mảnh, một chương trình — không chỗ nào viết ra được một trạng thái
vô nghĩa, và trình biên dịch tự tìm ra chỗ bạn quên xử lý. Track ADT &
Pattern Matching khép lại ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả track này.

Ba mươi sáu bài, đi từ một `type Hinh = Vuong | Tron` không có cách
nào phân biệt lúc chạy, tới một chương trình mà một kiểu dữ liệu tự
mang đủ NHÃN để trình biên dịch tự tìm ra chỗ code chưa xử lý hết — dù
kiểu đó lồng sâu bao nhiêu tầng, dù trạng thái nghiệp vụ phức tạp tới
đâu.

Có một thứ bạn đã DÙNG xuyên suốt track này mà chưa từng gọi tên: mọi
mảng bạn viết — `dsHinh: Hinh[]` ở bài 6, hay bất kỳ `T[]` nào khác —
chỉ là cách viết gọn của `Array<Hinh>`, một kiểu THAM SỐ HOÁ theo kiểu
`Hinh`. Đó CHÍNH LÀ một generic của TypeScript, dùng từ rất sớm, chỉ
chưa ai chỉ tên nó ra.

Track sau (T4.4, Generics/Traits, quay lại Rust) hỏi tiếp: nếu một
kiểu dữ liệu BẠN TỰ VIẾT cần THAM SỐ HOÁ theo một kiểu KHÁC — một
`Hop<T>` chứa MỘT giá trị kiểu `T` bất kỳ, không cố định trước `T` là
gì — viết thế nào? Và Rust's trait/generic (ngôn ngữ Cổng Rust đã dạy ở
T4.0b) khác gì thứ generic TypeScript bạn vừa nhận ra mình đã dùng?
::::

::::checkpoint{mastery=0.85}
::::
