---
id: ky-nghe-phan-mem.ddd.bounded-context-va-lop-dich-acl
title: "Bounded Context & Anti-Corruption Layer — một từ, nhiều nghĩa, ranh giới rõ"
summary: "Bounded Context: ranh giới nơi MỘT từ có MỘT nghĩa nhất quán — cùng tên SanPham nhưng ba nghĩa khác nhau ở ba \"quận\". Anti-Corruption Layer (ACL): hàm dịch giữa hai bounded context — mỗi quận chỉ biết field nó cần."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.bounded-context-acl]
requires: [ddd.ubiquitous-language]
concepts: [ddd.bounded-context-acl]
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
"Sản phẩm" nghĩa là gì? Câu hỏi tưởng đơn giản — nhưng câu trả lời KHÁC
NHAU tuỳ bạn hỏi ai trong công ty.
::::

::::explain{#bounded-context}
**Bounded Context**: ranh giới nơi MỘT từ có MỘT nghĩa NHẤT QUÁN. Cùng
tên `SanPham`, nhưng BA nghĩa khác nhau ở BA "quận" của hệ thống:

```typescript
// Quận Bán Hàng: SanPham nghĩa là "thứ để BÁN"
type SanPhamBanHang = { ma: string; ten: string; gia: number; coGiamGia: boolean };

// Quận Kho Vận: SanPham nghĩa là "thứ để CẤT GIỮ"
type SanPhamKhoVan = { ma: string; maKho: string; trongLuong: number; viTri: string };

// Quận Vận Chuyển: SanPham nghĩa là "thứ để CHỞ"
type SanPhamVanChuyen = { ma: string; trongLuong: number; deVo: boolean; canLanh: boolean };
```

Ba type KHÁC TÊN (`SanPhamBanHang`/`SanPhamKhoVan`/`SanPhamVanChuyen`) —
KHÔNG dùng một type `SanPham` chung nhồi nhét TẤT CẢ field, vì Bán Hàng
không cần biết `viTri` trong kho, Vận Chuyển không cần biết `gia`. Mỗi
type chỉ mang ĐÚNG field bounded context ĐÓ quan tâm — đây LÀ ranh giới
Bounded Context THỂ HIỆN trong kiểu dữ liệu, đúng tinh thần Ubiquitous
Language (bài 2) áp dụng RIÊNG cho TỪNG ranh giới.
::::

::::example{#anti-corruption-layer}
Khi HAI bounded context cần TRAO ĐỔI dữ liệu, chúng KHÔNG chia sẻ type
trực tiếp — mà đi qua một **Anti-Corruption Layer (ACL)**: hàm DỊCH,
chỉ lấy field CẦN THIẾT:

```typescript title=readonly
type SanPhamKhoVan = { ma: string; maKho: string; trongLuong: number; viTri: string };
type SanPhamVanChuyen = { ma: string; trongLuong: number; deVo: boolean; canLanh: boolean };

function sangSanPhamVanChuyen(
  kho: SanPhamKhoVan,
  deVo: boolean,
  canLanh: boolean,
): SanPhamVanChuyen {
  return { ma: kho.ma, trongLuong: kho.trongLuong, deVo, canLanh };
}

const spKho: SanPhamKhoVan = { ma: "SP-01", maKho: "K-A1", trongLuong: 1.5, viTri: "Ke-3" };
console.log(JSON.stringify(sangSanPhamVanChuyen(spKho, true, false)));
```

```text title=readonly
{"ma":"SP-01","trongLuong":1.5,"deVo":true,"canLanh":false}
```

`sangSanPhamVanChuyen` là ACL: nó ĐỌC `SanPhamKhoVan` (có `maKho`,
`viTri` — Vận Chuyển KHÔNG cần) nhưng chỉ TRẢ những field Vận Chuyển
THẬT SỰ cần (`ma`, `trongLuong`, cộng thông tin đóng gói `deVo`/
`canLanh` — thông tin RIÊNG của quận Vận Chuyển, không có trong
`SanPhamKhoVan`). Quận Kho Vận đổi cấu trúc nội bộ (`viTri` thành GPS
toạ độ chẳng hạn) — ACL là NƠI DUY NHẤT cần sửa, quận Vận Chuyển hoàn
toàn KHÔNG BIẾT gì về sự thay đổi đó.
::::

::::predict{#doan-acl-khong-co-gia commitOnce}
```typescript
type SanPhamKhoVan = { ma: string; maKho: string; trongLuong: number; viTri: string };
type SanPhamVanChuyen = { ma: string; trongLuong: number; deVo: boolean; canLanh: boolean };
function sangSanPhamVanChuyen(
  kho: SanPhamKhoVan,
  deVo: boolean,
  canLanh: boolean,
): SanPhamVanChuyen {
  return { ma: kho.ma, trongLuong: kho.trongLuong, deVo, canLanh };
}

const spKho: SanPhamKhoVan = { ma: "SP-02", maKho: "K-B7", trongLuong: 3, viTri: "Ke-9" };
const spVanChuyen = sangSanPhamVanChuyen(spKho, false, true);
console.log("gia" in spVanChuyen);
```

Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì `SanPhamKhoVan` và `SanPhamVanChuyen` cùng nói về MỘT sản
phẩm thật, nên mọi thông tin (kể cả giá) đều được giữ lại qua ACL
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng cả hai type CÙNG nói về MỘT sản phẩm
thật ngoài đời — quan sát về danh tính chung đó đúng.

Chỗ lệch: `gia` KHÔNG hề xuất hiện trong `SanPhamKhoVan` (input của
ACL) LẪN `SanPhamVanChuyen` (output) — Kho Vận không lưu giá, Vận
Chuyển không cần giá. ACL chỉ dịch những field THẬT SỰ tồn tại ở đầu
vào sang những field bounded context đích CẦN — nó không thể "giữ lại"
một field CHƯA BAO GIỜ có mặt.
::
:::

:::opt
Máy báo lỗi biên dịch — `spVanChuyen` được suy ra kiểu chính xác
`SanPhamVanChuyen`, TypeScript không cho dùng toán tử `in` để kiểm tra
field không tồn tại trên type đó
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng TypeScript SUY RA kiểu chính xác cho
`spVanChuyen` (`SanPhamVanChuyen`) — quan sát về suy luận kiểu đó đúng.

Chỗ lệch: toán tử `in` (`"gia" in spVanChuyen`) là một phép kiểm tra
LÚC CHẠY (runtime), hoạt động trên BẤT KỲ object nào, kể cả khi field
đang hỏi KHÔNG tồn tại trong type khai báo — TypeScript hoàn toàn cho
phép, không báo lỗi biên dịch nào. Kết quả LÚC CHẠY là `false` vì object
thật sự không có key `"gia"`.
::
:::
::::

::::code{#viet_acl}
Tự viết `sangSanPhamVanChuyen` — dịch `SanPhamKhoVan` (đầu vào) sang
`SanPhamVanChuyen` (đầu ra), chỉ lấy field cần.

```typescript title=starter
type SanPhamKhoVan = { ma: string; maKho: string; trongLuong: number; viTri: string };
type SanPhamVanChuyen = { ma: string; trongLuong: number; deVo: boolean; canLanh: boolean };

function sangSanPhamVanChuyen(
  kho: SanPhamKhoVan,
  deVo: boolean,
  canLanh: boolean,
): SanPhamVanChuyen {
  return { ma: ___, trongLuong: ___, deVo, canLanh };
}

const spKho: SanPhamKhoVan = { ma: "SP-01", maKho: "K-A1", trongLuong: 1.5, viTri: "Ke-3" };
console.log(JSON.stringify(sangSanPhamVanChuyen(spKho, true, false)));
```

```typescript title=solution
type SanPhamKhoVan = { ma: string; maKho: string; trongLuong: number; viTri: string };
type SanPhamVanChuyen = { ma: string; trongLuong: number; deVo: boolean; canLanh: boolean };

function sangSanPhamVanChuyen(
  kho: SanPhamKhoVan,
  deVo: boolean,
  canLanh: boolean,
): SanPhamVanChuyen {
  return { ma: kho.ma, trongLuong: kho.trongLuong, deVo, canLanh };
}

const spKho: SanPhamKhoVan = { ma: "SP-01", maKho: "K-A1", trongLuong: 1.5, viTri: "Ke-3" };
console.log(JSON.stringify(sangSanPhamVanChuyen(spKho, true, false)));
```

```typescript title=test
const spKho1: SanPhamKhoVan = { ma: "SP-01", maKho: "K-A1", trongLuong: 1.5, viTri: "Ke-3" };
const kq1 = sangSanPhamVanChuyen(spKho1, true, false);
if (kq1.ma !== "SP-01") throw new Error("phải giữ đúng mã sản phẩm khi dịch sang");
if (kq1.trongLuong !== 1.5) throw new Error("phải giữ đúng trọng lượng khi dịch sang");
if (kq1.deVo !== true) throw new Error("phải giữ đúng cờ dễ vỡ truyền vào");
if (kq1.canLanh !== false) throw new Error("phải giữ đúng cờ cần lạnh truyền vào");
if ("maKho" in kq1) throw new Error("kết quả KHÔNG được mang field maKho — Vận Chuyển không cần biết mã kho");
if ("viTri" in kq1) throw new Error("kết quả KHÔNG được mang field viTri — Vận Chuyển không cần biết vị trí kho");

const spKho2: SanPhamKhoVan = { ma: "SP-99", maKho: "K-Z9", trongLuong: 7.2, viTri: "Ke-1" };
const kq2 = sangSanPhamVanChuyen(spKho2, false, true);
if (kq2.ma !== "SP-99") throw new Error("phải hoạt động đúng với sản phẩm khác");
if (kq2.trongLuong !== 7.2) throw new Error("trọng lượng phải khớp sản phẩm khác");
```

:::hints
- kind: attention
  body: "ACL chỉ LẤY field từ kho, KHÔNG tự nghĩ ra giá trị mới: ma của kết quả CHÍNH LÀ kho.ma; trongLuong của kết quả CHÍNH LÀ kho.trongLuong."
- kind: strategy
  body: "{ ma: kho.ma, trongLuong: kho.trongLuong, deVo, canLanh } — hai field đầu LẤY từ tham số kho, hai field sau NHẬN thẳng từ tham số hàm (đã có tên trùng, không cần viết lại)."
- kind: one-line
  body: "___ (ma) = kho.ma\n___ (trongLuong) = kho.trongLuong"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "SP-01"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bounded Context: một từ, nhiều nghĩa, ranh giới rõ ràng. Anti-Corruption
Layer: cầu nối giữa các ranh giới, không để một quận "rò rỉ" cấu trúc
nội bộ sang quận khác.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trong MỘT bounded context, có hai loại đối tượng khác nhau về bản chất
— một loại có "danh tính" (đổi tên vẫn là NÓ), một loại chỉ là "giá
trị" (đổi một chút là thành thứ KHÁC). Phân biệt hai loại đó thế nào?
::::

::::checkpoint{mastery=0.8}
::::
