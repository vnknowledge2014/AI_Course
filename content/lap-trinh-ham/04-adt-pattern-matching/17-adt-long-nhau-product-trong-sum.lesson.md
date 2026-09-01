---
id: lap-trinh-ham.adt-pattern-matching.adt-long-nhau-product-trong-sum
title: "ADT lồng nhau — product BÊN TRONG sum"
summary: "`type DonHang = { kind: \"choXuLy\"; ma: string } | { kind: \"daGiao\"; ma: string; ngayGiao: string; nguoiNhan: string }` — MỖI biến thể của sum type LÀ một product type riêng (field kind cộng các field khác nhau tuỳ biến thể). Đây là hình dạng THẬT SỰ của ADT trong mã sản xuất: không sum 'thuần', không product 'thuần', mà LỒNG NHAU — sum-of-products."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 17
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.nested-adt]
requires: [ts.counting-states-product-sum]
concepts: [ts.nested-adt]
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
Bài trước hỏi: nếu MỘT biến thể của sum type lại LÀ một product type —
nhiều field, không chỉ `kind` — thì hình dạng đó trông thế nào? Hôm nay
xem hàng thật.
::::

::::explain{#product-ben-trong-sum}
```typescript
type DonHang =
  | { kind: "choXuLy"; ma: string }
  | { kind: "daGiao"; ma: string; ngayGiao: string; nguoiNhan: string };

const d1: DonHang = { kind: "choXuLy", ma: "DH-1" };
const d2: DonHang = {
  kind: "daGiao",
  ma: "DH-2",
  ngayGiao: "2026-01-15",
  nguoiNhan: "Lan",
};

console.log(d1.kind, Object.keys(d1).length);
console.log(d2.kind, Object.keys(d2).length);
```

```text
choXuLy 2
daGiao 4
```

`DonHang` viết TRỰC TIẾP hai hình dạng object bên trong dấu `|`, không
qua `interface` riêng — nhưng vẫn là discriminated union quen thuộc:
field `kind` là nhãn phân biệt, mỗi nhánh của `|` là MỘT biến thể. Cái
MỚI nằm ở BÊN TRONG từng biến thể.

Nhánh đầu (`"choXuLy"`) có ĐÚNG hai field: `kind` và `ma` — một giá trị
thuộc biến thể này PHẢI có CẢ HAI CÙNG LÚC, thiếu field nào cũng không
biên dịch được (luật product type từ bài 14, áp dụng y nguyên). Nhánh
sau (`"daGiao"`) có BỐN field: `kind`, `ma`, `ngayGiao`, `nguoiNhan` —
cũng PHẢI có ĐỦ CẢ BỐN CÙNG LÚC. `Object.keys(...).length` xác nhận
đúng: `d1` có 2 field, `d2` có 4 field — KHÁC NHAU, vì MỖI biến thể là
một product type RIÊNG, với số field RIÊNG của nó.

Đây chính là câu hỏi bài trước để lại: `DonHang` là biến thể `"choXuLy"`
(một product gồm các field của NÓ) CỘNG biến thể `"daGiao"` (một product
gồm các field của NÓ) — SUM ở tầng NGOÀI (chọn ĐÚNG MỘT trong hai biến
thể), PRODUCT ở tầng TRONG (mỗi biến thể đòi đủ field riêng CÙNG LÚC).
So với `Hinh` ở bài trước (mỗi biến thể chỉ có ĐÚNG field `kind`, không
field nào khác — sum "thuần"), hay `NguoiDung` ở bài 14 (một hình dạng
cố định, không biến thể nào — product "thuần"), `DonHang` LỒNG cả hai:
sum bọc NGOÀI, product NẰM TRONG mỗi nhánh. Đây mới là hình dạng THẬT SỰ
của ADT trong mã sản xuất — sum "thuần" hay product "thuần" một mình
đều hiếm gặp.
::::

::::example{#dung-field-dung-nhanh}
Narrow trên `kind` không chỉ chọn NHÁNH chạy — nó chọn ĐÚNG product nào
đang có hiệu lực bên trong nhánh đó:

```typescript title=readonly
type DonHang =
  | { kind: "choXuLy"; ma: string }
  | { kind: "daGiao"; ma: string; ngayGiao: string; nguoiNhan: string };

function moTaDon(d: DonHang): string {
  switch (d.kind) {
    case "choXuLy":
      return "Đơn " + d.ma + " đang chờ xử lý";
    case "daGiao":
      return "Đơn " + d.ma + " đã giao ngày " + d.ngayGiao + " cho " + d.nguoiNhan;
  }
}

console.log(moTaDon({ kind: "choXuLy", ma: "DH-1" }));
console.log(
  moTaDon({ kind: "daGiao", ma: "DH-2", ngayGiao: "2026-01-15", nguoiNhan: "Lan" }),
);
```

```text title=readonly
Đơn DH-1 đang chờ xử lý
Đơn DH-2 đã giao ngày 2026-01-15 cho Lan
```

Trong nhánh `case "choXuLy"`, TypeScript thu hẹp `d` xuống ĐÚNG product
hai field (`kind`, `ma`) — đọc `d.ma` được. Trong nhánh `case "daGiao"`,
`d` thu hẹp xuống ĐÚNG product bốn field — `d.ngayGiao` và `d.nguoiNhan`
đọc được ở ĐÂY, vì hai field đó CHỈ tồn tại trong product của biến thể
này. Không nhánh nào "nhìn thấy" field của nhánh kia.
::::

::::predict{#doan-loi-doc-nham-truong commitOnce}
```typescript
type DonHang =
  | { kind: "choXuLy"; ma: string }
  | { kind: "daGiao"; ma: string; ngayGiao: string; nguoiNhan: string };

function moTa(d: DonHang): string {
  switch (d.kind) {
    case "choXuLy":
      return "Đơn " + d.ma + " giao ngày " + d.ngayGiao;
    case "daGiao":
      return "Đơn " + d.ma + " đã giao";
  }
}

console.log(moTa({ kind: "choXuLy", ma: "DH-1" }));
```

Dòng cuối in ra gì?

:::opt{correct}
Máy báo lỗi biên dịch — mã TS2339, vì bên trong `case "choXuLy"`, `d` đã
thu hẹp xuống product `{ kind: "choXuLy"; ma: string }`, không có field
`ngayGiao`
:::

:::opt
`Đơn DH-1 giao ngày undefined` — object không có field `ngayGiao`,
JavaScript đọc field không tồn tại trả `undefined`, phép nối chuỗi vẫn
chạy bình thường
::why
Gần đúng ở việc bạn nhớ đúng ngữ nghĩa JAVASCRIPT thuần: đọc field
không tồn tại trên một object trả `undefined`, không throw exception —
đúng, NẾU mã có cơ hội CHẠY tới đó.

Chỗ lệch: mã này không CHẠY tới bước đó. Bên trong `case "choXuLy"`,
TypeScript đã thu hẹp `d` xuống ĐÚNG product hai field của biến thể này
— `{ kind: "choXuLy"; ma: string }` — không có `ngayGiao` (field đó
chỉ thuộc product RIÊNG của biến thể `"daGiao"`). TypeScript CHẶN NGAY
lúc biên dịch (mã TS2339), không đợi tới lúc chạy để lộ ra `undefined`.
::
:::

:::opt
`Đơn DH-1 giao ngày ` (chuỗi rỗng nối vào) — không phải biến thể nào
của `DonHang` cũng CÓ `ngayGiao`, nên TypeScript coi field đó là
optional trên toàn union, tự điền chuỗi rỗng khi thiếu
::why
Gần đúng ở việc bạn nhận ra ĐÚNG một điều: không phải biến thể nào của
`DonHang` cũng có `ngayGiao` — chỉ `"daGiao"` mới có.

Chỗ lệch: điều đó không biến `ngayGiao` thành field OPTIONAL dùng
chung cho cả `DonHang` (kiểu `ngayGiao?: string`) — nó biến `"choXuLy"`
thành một PRODUCT TYPE RIÊNG, hoàn toàn KHÔNG khai field `ngayGiao`
(không phải "có nhưng optional", mà là "không tồn tại trong hình dạng
này"). TypeScript không có cơ chế tự điền chuỗi rỗng cho field không
tồn tại — nó CHẶN việc đọc field đó ngay lúc biên dịch, trước khi phép
nối chuỗi nào chạy.
::
:::
::::

::::code{#tao_don_da_giao}
Viết `taoDonDaGiao(ma, ngayGiao, nguoiNhan)` — trả về một `DonDaGiao`
(product BỐN field của biến thể `"daGiao"`) với CẢ BỐN field đúng giá
trị truyền vào.

```typescript title=starter
interface DonChoXuLy {
  kind: "choXuLy";
  ma: string;
}

interface DonDaGiao {
  kind: "daGiao";
  ma: string;
  ngayGiao: string;
  nguoiNhan: string;
}

type DonHang = DonChoXuLy | DonDaGiao;

function taoDonDaGiao(ma: string, ngayGiao: string, nguoiNhan: string): DonDaGiao {
  return { kind: "daGiao", ma: ma, ngayGiao: ngayGiao, nguoiNhan: ___ };
}

const d = taoDonDaGiao("DH-9", "2026-02-01", "Minh");
console.log(d.kind, d.nguoiNhan);
```

```typescript title=solution
interface DonChoXuLy {
  kind: "choXuLy";
  ma: string;
}

interface DonDaGiao {
  kind: "daGiao";
  ma: string;
  ngayGiao: string;
  nguoiNhan: string;
}

type DonHang = DonChoXuLy | DonDaGiao;

function taoDonDaGiao(ma: string, ngayGiao: string, nguoiNhan: string): DonDaGiao {
  return { kind: "daGiao", ma: ma, ngayGiao: ngayGiao, nguoiNhan: nguoiNhan };
}

const d = taoDonDaGiao("DH-9", "2026-02-01", "Minh");
console.log(d.kind, d.nguoiNhan);
```

```typescript title=test
const d1 = taoDonDaGiao("DH-9", "2026-02-01", "Minh");
if (d1.kind !== "daGiao") throw new Error("taoDonDaGiao phải trả về kind là \"daGiao\" — đang là " + d1.kind);
if (d1.ma !== "DH-9") throw new Error("taoDonDaGiao(\"DH-9\", ...) phải có ma là \"DH-9\" — đang là " + d1.ma);
if (d1.ngayGiao !== "2026-02-01") throw new Error("taoDonDaGiao(...) phải có ngayGiao là \"2026-02-01\" — đang là " + d1.ngayGiao);
if (d1.nguoiNhan !== "Minh") throw new Error("taoDonDaGiao(...) phải có nguoiNhan là \"Minh\" — đang là " + d1.nguoiNhan);
const d2 = taoDonDaGiao("DH-10", "2026-03-03", "Hoa");
if (d2.nguoiNhan !== "Hoa") throw new Error("taoDonDaGiao(...) phải có nguoiNhan là \"Hoa\" — đang là " + d2.nguoiNhan);
```

:::hints
- kind: attention
  body: "Chỗ trống là GIÁ TRỊ gán cho field nguoiNhan trong object trả về — kind, ma, ngayGiao đã được điền sẵn, nguoiNhan cần điền tương tự."
- kind: strategy
  body: "Hàm nhận tham số nguoiNhan — object trả về (thuộc product DonDaGiao, đủ BỐN field) phải LẤY đúng giá trị tham số đó, không phải một chuỗi cố định nào khác."
- kind: one-line
  body: "Chỗ trống là: nguoiNhan"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Minh"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai luật cũ (product, sum) GHÉP lại thành một: ADT thật trong mã sản
xuất luôn có hình dạng này — sum bọc ngoài, product nằm trong.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa THẤY một ADT lồng nhau hoàn chỉnh — sum ngoài, product trong,
và cách narrow đúng product của từng nhánh. Nhưng `DonChoXuLy` và
`DonDaGiao` đã được VIẾT SẴN cho bạn, bạn chỉ điền một chỗ trống. Nếu
phải TỰ TAY thiết kế một union hai biến thể — mỗi biến thể có field
riêng, không ai viết hộ khung — bạn có làm được không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
