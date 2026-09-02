---
id: ky-nghe-phan-mem.ddd.domain-hierarchy-nhieu-cap
title: "Domain Hierarchy nhiều cấp — DU lồng DU lồng DU"
summary: "DU chứa DU chứa DU mô hình hoá domain phức tạp thành \"cây kiểu\". SanPham (vat_ly/so/dang_ky) → mỗi loại lại có sub-DU RIÊNG: TinhTrangKho, LoaiGiayPhep. Exhaustive switch điều hướng CẢ cây."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 17
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.domain-hierarchy]
requires: [ddd.optional-field-to-du]
concepts: [ddd.domain-hierarchy]
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
`TrangThaiXacMinh` là MỘT DU đơn giản — hai variant, phẳng. Domain
THẬT thường sâu hơn nhiều tầng.
::::

::::explain{#du-long-du}
`SanPham` có BA loại — MỖI loại lại có TRẠNG THÁI RIÊNG, mã hoá bằng
sub-DU RIÊNG của loại đó:

```typescript
type TinhTrangKho =
  | { tag: "con_hang"; soLuong: number }
  | { tag: "het_hang" }
  | { tag: "sap_ve"; ngayVe: string }
  | { tag: "ngung_kinh_doanh" };

type LoaiGiayPhep =
  | { tag: "vinh_vien" }
  | { tag: "thue_bao"; ngayHetHan: string }
  | { tag: "dung_thu"; soNgayConLai: number };

type SanPham =
  | { tag: "vat_ly"; ten: string; kho: TinhTrangKho }
  | { tag: "so"; ten: string; kichThuocMb: number }
  | { tag: "dang_ky"; ten: string; giayPhep: LoaiGiayPhep };
```

`TinhTrangKho` CHỈ có nghĩa cho sản phẩm `"vat_ly"` (một file `.pdf`
không có "tình trạng kho"); `LoaiGiayPhep` CHỈ có nghĩa cho `"dang_ky"`
(một cái ly thuỷ tinh không có "giấy phép"). Đây LÀ "cây kiểu" — DU Ở
CẤP NGOÀI (`SanPham`) chứa DU KHÁC Ở CẤP TRONG (`TinhTrangKho`,
`LoaiGiayPhep`), MỖI cấp tự khử được KHẢ NĂNG phi lý CỦA RIÊNG nó.
::::

::::example{#switch-di-qua-hai-cap}
Đọc "cây kiểu" bằng `switch` LỒNG NHAU — cấp NGOÀI switch trên
`sp.tag`, cấp TRONG switch/so sánh trên sub-DU tương ứng:

```typescript title=readonly
type TinhTrangKho = { tag: "con_hang"; soLuong: number } | { tag: "het_hang" } | { tag: "sap_ve"; ngayVe: string } | { tag: "ngung_kinh_doanh" };
type LoaiGiayPhep = { tag: "vinh_vien" } | { tag: "thue_bao"; ngayHetHan: string } | { tag: "dung_thu"; soNgayConLai: number };
type SanPham = { tag: "vat_ly"; ten: string; kho: TinhTrangKho } | { tag: "so"; ten: string; kichThuocMb: number } | { tag: "dang_ky"; ten: string; giayPhep: LoaiGiayPhep };

function coTheMuaDuoc(sp: SanPham): boolean {
  switch (sp.tag) {
    case "vat_ly":
      return sp.kho.tag === "con_hang" && sp.kho.soLuong > 0;
    case "so":
      return sp.kichThuocMb > 0;
    case "dang_ky":
      return sp.giayPhep.tag !== "dung_thu" || sp.giayPhep.soNgayConLai > 0;
  }
}

const sp1: SanPham = { tag: "vat_ly", ten: "Ly", kho: { tag: "con_hang", soLuong: 5 } };
const sp2: SanPham = { tag: "vat_ly", ten: "Ban", kho: { tag: "het_hang" } };
const sp3: SanPham = { tag: "dang_ky", ten: "Phan mem", giayPhep: { tag: "dung_thu", soNgayConLai: 0 } };
console.log(coTheMuaDuoc(sp1));
console.log(coTheMuaDuoc(sp2));
console.log(coTheMuaDuoc(sp3));
```

```text title=readonly
true
false
false
```

Ở nhánh `"vat_ly"`, `sp.kho.tag` (CẤP TRONG) mới quyết định kết quả —
`sp2.kho.tag === "het_hang"` khiến `coTheMuaDuoc(sp2)` là `false`, dù
`sp2.tag === "vat_ly"` (CẤP NGOÀI) đã khớp nhánh ĐẦU TIÊN. `sp3` là
`"dang_ky"` với giấy phép DÙNG THỬ ĐÃ HẾT (`soNgayConLai: 0`) — cũng
`false`. Exhaustive switch (T4.3) áp dụng ở CẢ HAI cấp — thêm MỘT
variant MỚI ở BẤT KỲ cấp nào (ví dụ thêm `"bundle"` vào `SanPham`,
hay thêm `"tam_ngung"` vào `TinhTrangKho`) khiến compiler báo lỗi TẤT
CẢ switch thiếu case liên quan, dù switch đó nằm ở tầng NÀO.
::::

::::predict{#doan-san-pham-so commitOnce}
```typescript
type TinhTrangKho = { tag: "con_hang"; soLuong: number } | { tag: "het_hang" } | { tag: "sap_ve"; ngayVe: string } | { tag: "ngung_kinh_doanh" };
type LoaiGiayPhep = { tag: "vinh_vien" } | { tag: "thue_bao"; ngayHetHan: string } | { tag: "dung_thu"; soNgayConLai: number };
type SanPham = { tag: "vat_ly"; ten: string; kho: TinhTrangKho } | { tag: "so"; ten: string; kichThuocMb: number } | { tag: "dang_ky"; ten: string; giayPhep: LoaiGiayPhep };
function coTheMuaDuoc(sp: SanPham): boolean {
  switch (sp.tag) {
    case "vat_ly": return sp.kho.tag === "con_hang" && sp.kho.soLuong > 0;
    case "so": return sp.kichThuocMb > 0;
    case "dang_ky": return sp.giayPhep.tag !== "dung_thu" || sp.giayPhep.soNgayConLai > 0;
  }
}

const spSap: SanPham = { tag: "vat_ly", ten: "Ao", kho: { tag: "sap_ve", ngayVe: "2024-06-01" } };
console.log(coTheMuaDuoc(spSap));
```

`spSap.kho.tag` là `"sap_ve"` (SẮP về hàng — KHÔNG phải `"con_hang"`).
Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì "sắp về" nghĩa là hàng SẮP CÓ, gần như tương đương "còn
hàng", nên vẫn coi là mua được
::why
Gần đúng ở việc bạn hiểu ĐÚNG Ý NGHĨA nghiệp vụ của `"sap_ve"` (hàng
SẮP có, không phải hết hẳn) — quan sát về ngữ nghĩa đó hợp lý.

Chỗ lệch: `coTheMuaDuoc` (ĐÚNG như code viết) chỉ kiểm
`sp.kho.tag === "con_hang"` — SO SÁNH CHÍNH XÁC với chuỗi `"con_hang"`,
KHÔNG có logic "gần giống cũng được". `"sap_ve"` là MỘT variant KHÁC
HẲN `"con_hang"` (dù cùng nói về việc "sắp có hàng"), điều kiện
`sp.kho.tag === "con_hang"` cho `false` — `&&` với bất kỳ gì phía sau
CŨNG là `false`.
::
:::

:::opt
Máy báo lỗi biên dịch — nhánh `case "vat_ly"` truy cập `sp.kho.soLuong`
nhưng `TinhTrangKho` KHÔNG PHẢI variant nào cũng có field `soLuong`
::why
Gần đúng ở việc bạn để ý ĐÚNG rằng chỉ variant `"con_hang"` của
`TinhTrangKho` CÓ field `soLuong` — ba variant còn lại KHÔNG có —
quan sát đó đúng.

Chỗ lệch: `sp.kho.soLuong` chỉ được TRUY CẬP Ở NHÁNH `&&` THỨ HAI, SAU
KHI `sp.kho.tag === "con_hang"` đã kiểm tra ĐÚNG (nhờ `&&` ngắn mạch —
short-circuit, đã học từ trước) — TypeScript NARROW `sp.kho` xuống
ĐÚNG variant `"con_hang"` tại thời điểm đó, `soLuong` CHẮC CHẮN tồn
tại. Biên dịch hoàn toàn sạch.
::
:::
::::

::::code{#viet_co_the_mua_duoc}
Tự viết `coTheMuaDuoc(sp: SanPham): boolean`.

```typescript title=starter
type TinhTrangKho = { tag: "con_hang"; soLuong: number } | { tag: "het_hang" } | { tag: "sap_ve"; ngayVe: string } | { tag: "ngung_kinh_doanh" };
type LoaiGiayPhep = { tag: "vinh_vien" } | { tag: "thue_bao"; ngayHetHan: string } | { tag: "dung_thu"; soNgayConLai: number };
type SanPham = { tag: "vat_ly"; ten: string; kho: TinhTrangKho } | { tag: "so"; ten: string; kichThuocMb: number } | { tag: "dang_ky"; ten: string; giayPhep: LoaiGiayPhep };

function coTheMuaDuoc(sp: SanPham): boolean {
  switch (sp.tag) {
    case "vat_ly":
      return ___;
    case "so":
      return ___;
    case "dang_ky":
      return ___;
  }
}

const sp1: SanPham = { tag: "vat_ly", ten: "Ly", kho: { tag: "con_hang", soLuong: 5 } };
console.log(coTheMuaDuoc(sp1));
```

```typescript title=solution
type TinhTrangKho = { tag: "con_hang"; soLuong: number } | { tag: "het_hang" } | { tag: "sap_ve"; ngayVe: string } | { tag: "ngung_kinh_doanh" };
type LoaiGiayPhep = { tag: "vinh_vien" } | { tag: "thue_bao"; ngayHetHan: string } | { tag: "dung_thu"; soNgayConLai: number };
type SanPham = { tag: "vat_ly"; ten: string; kho: TinhTrangKho } | { tag: "so"; ten: string; kichThuocMb: number } | { tag: "dang_ky"; ten: string; giayPhep: LoaiGiayPhep };

function coTheMuaDuoc(sp: SanPham): boolean {
  switch (sp.tag) {
    case "vat_ly":
      return sp.kho.tag === "con_hang" && sp.kho.soLuong > 0;
    case "so":
      return sp.kichThuocMb > 0;
    case "dang_ky":
      return sp.giayPhep.tag !== "dung_thu" || sp.giayPhep.soNgayConLai > 0;
  }
}

const sp1: SanPham = { tag: "vat_ly", ten: "Ly", kho: { tag: "con_hang", soLuong: 5 } };
console.log(coTheMuaDuoc(sp1));
```

```typescript title=test
const spConHang: SanPham = { tag: "vat_ly", ten: "Ly", kho: { tag: "con_hang", soLuong: 5 } };
if (coTheMuaDuoc(spConHang) !== true) throw new Error("sản phẩm vật lý còn hàng phải mua được");

const spHetHang: SanPham = { tag: "vat_ly", ten: "Ban", kho: { tag: "het_hang" } };
if (coTheMuaDuoc(spHetHang) !== false) throw new Error("sản phẩm vật lý hết hàng không được mua được");

const spSoHopLe: SanPham = { tag: "so", ten: "Ebook", kichThuocMb: 5 };
if (coTheMuaDuoc(spSoHopLe) !== true) throw new Error("sản phẩm số có kích thước dương phải mua được");

const spSoLoi: SanPham = { tag: "so", ten: "File hong", kichThuocMb: 0 };
if (coTheMuaDuoc(spSoLoi) !== false) throw new Error("sản phẩm số kích thước 0 không được mua được");

const spDangKyConHan: SanPham = { tag: "dang_ky", ten: "PM", giayPhep: { tag: "dung_thu", soNgayConLai: 3 } };
if (coTheMuaDuoc(spDangKyConHan) !== true) throw new Error("dùng thử còn ngày phải mua được");

const spDangKyHetHan: SanPham = { tag: "dang_ky", ten: "PM", giayPhep: { tag: "dung_thu", soNgayConLai: 0 } };
if (coTheMuaDuoc(spDangKyHetHan) !== false) throw new Error("dùng thử hết ngày không được mua được");
```

:::hints
- kind: attention
  body: "vat_ly: còn hàng VÀ số lượng dương. so: kích thước dương. dang_ky: KHÔNG phải dùng thử, HOẶC dùng thử còn ngày."
- kind: strategy
  body: 'sp.kho.tag === "con_hang" && sp.kho.soLuong > 0 — vat_ly. sp.kichThuocMb > 0 — so. sp.giayPhep.tag !== "dung_thu" || sp.giayPhep.soNgayConLai > 0 — dang_ky.'
- kind: one-line
  body: '___ (vat_ly) = sp.kho.tag === "con_hang" && sp.kho.soLuong > 0\n___ (so) = sp.kichThuocMb > 0\n___ (dang_ky) = sp.giayPhep.tag !== "dung_thu" || sp.giayPhep.soNgayConLai > 0'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Domain Hierarchy: DU lồng DU, mỗi tầng tự khử khả năng phi lý CỦA
RIÊNG nó. Exhaustive switch bảo vệ TẤT CẢ các tầng, không chỉ tầng
ngoài cùng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có VO, DU state machine, domain hierarchy. Ghép TẤT CẢ vào MỘT
entity duy nhất, chịu trách nhiệm toàn vẹn dữ liệu — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
