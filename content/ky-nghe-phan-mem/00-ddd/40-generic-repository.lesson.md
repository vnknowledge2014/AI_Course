---
id: ky-nghe-phan-mem.ddd.generic-repository
title: "Nhiều \"thủ thư\" cùng một Interface & Generic Repository"
summary: "Trừu tượng hoá Repository<Id, ThucThe> GENERIC dùng chung cho MỌI entity, thay vì viết lại interface cho từng loại. taoKhoBoNho<Id, ThucThe>() implement MỘT LẦN, dùng lại cho DonHang, SanPham, bất kỳ entity nào. Domain-specific repo mở rộng thêm method riêng bằng spread trên nền generic."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 40
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.generic-repository]
requires: [ddd.data-mapper]
concepts: [ddd.generic-repository]
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
`KhoDonHang` (bài 37) và một `KhoSanPham` tương lai sẽ có `timTheoId`/
`luu`/`xoa` GẦN NHƯ GIỐNG HỆT — viết lại interface + implementation
cho MỖI entity có lãng phí không?
::::

::::explain{#repository-generic}
Trừu tượng hoá thành `Repository<Id, ThucThe>` **GENERIC** — dùng
CHUNG cho MỌI entity. Quy ước: entity CẦN một field `id` (tên CHUNG,
để generic "biết" đâu là khoá). `taoKhoBoNho<Id, ThucThe>()` viết
**MỘT LẦN**, dùng lại cho BẤT KỲ entity nào:

```typescript
type Repository<Id, ThucThe> = {
  timTheoId: (id: Id) => ThucThe | null;
  layTatCa: () => ThucThe[];
  luu: (thucThe: ThucThe) => void;
  xoa: (id: Id) => void;
};

function taoKhoBoNho<Id extends string, ThucThe extends { id: Id }>(): Repository<Id, ThucThe> {
  const duLieu = new Map<Id, ThucThe>();
  return {
    timTheoId: (id) => duLieu.get(id) ?? null,
    layTatCa: () => [...duLieu.values()],
    luu: (thucThe) => { duLieu.set(thucThe.id, thucThe); },
    xoa: (id) => { duLieu.delete(id); },
  };
}

type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { id: MaDonHang; maKhachHang: string; tongTien: number };

type MaSanPham = string & { readonly __brand: "MaSanPham" };
type SanPham = { id: MaSanPham; ten: string; gia: number };

const khoDonHang = taoKhoBoNho<MaDonHang, DonHang>();
khoDonHang.luu({ id: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 });
console.log(JSON.stringify(khoDonHang.timTheoId("DH-01" as MaDonHang)));

const khoSanPham = taoKhoBoNho<MaSanPham, SanPham>();
khoSanPham.luu({ id: "SP-01" as MaSanPham, ten: "Bút chì", gia: 5000 });
console.log(JSON.stringify(khoSanPham.timTheoId("SP-01" as MaSanPham)));
```

```text
{"id":"DH-01","maKhachHang":"KH-01","tongTien":100000}
{"id":"SP-01","ten":"Bút chì","gia":5000}
```

CÙNG `taoKhoBoNho` — gọi VỚI kiểu `<MaDonHang, DonHang>` cho `DonHang`,
VỚI `<MaSanPham, SanPham>` cho `SanPham` — TypeScript SUY RA `Id`/
`ThucThe` khác nhau MỖI lần, `duLieu` (Map) bên trong closure ĐÚNG
kiểu tương ứng. `ThucThe extends { id: Id }` là RÀO CHẮN: CHỈ entity
CÓ field `id` mới dùng `taoKhoBoNho` được.
::::

::::example{#mo-rong-bang-spread}
Domain-specific repo (như `KhoDonHang`, bài 37) CÓ THỂ **mở rộng**
thêm method RIÊNG (`timTheoKhachHang`) TRÊN NỀN generic — dùng
**spread** (`...`) để "thừa hưởng" bốn method CHUNG, rồi THÊM method
mới:

```typescript title=readonly
type Repository<Id, ThucThe> = { timTheoId: (id: Id) => ThucThe | null; layTatCa: () => ThucThe[]; luu: (thucThe: ThucThe) => void; xoa: (id: Id) => void };
function taoKhoBoNho<Id extends string, ThucThe extends { id: Id }>(): Repository<Id, ThucThe> {
  const duLieu = new Map<Id, ThucThe>();
  return { timTheoId: (id) => duLieu.get(id) ?? null, layTatCa: () => [...duLieu.values()], luu: (thucThe) => { duLieu.set(thucThe.id, thucThe); }, xoa: (id) => { duLieu.delete(id); } };
}
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { id: MaDonHang; maKhachHang: string; tongTien: number };

// Domain-specific: Repository CHUNG + method RIÊNG
type KhoDonHang = Repository<MaDonHang, DonHang> & {
  timTheoKhachHang: (maKhachHang: string) => DonHang[];
};

function taoKhoDonHangMoRong(): KhoDonHang {
  const co = taoKhoBoNho<MaDonHang, DonHang>();
  return {
    ...co, // thừa hưởng timTheoId, layTatCa, luu, xoa
    timTheoKhachHang: (mkh) => co.layTatCa().filter((dh) => dh.maKhachHang === mkh),
  };
}

const khoMoRong = taoKhoDonHangMoRong();
khoMoRong.luu({ id: "DH-10" as MaDonHang, maKhachHang: "KH-05", tongTien: 1000 });
khoMoRong.luu({ id: "DH-11" as MaDonHang, maKhachHang: "KH-05", tongTien: 2000 });
console.log(khoMoRong.timTheoKhachHang("KH-05").length);
console.log(khoMoRong.timTheoId("DH-10" as MaDonHang) !== null);
```

```text title=readonly
2
true
```

`{ ...co, timTheoKhachHang: ... }` COPY bốn method của `co` (CÙNG
tham chiếu HÀM, KHÔNG sao chép `duLieu` bên trong) — `khoMoRong.luu`
CHÍNH LÀ `co.luu` (cùng closure), nên `khoMoRong.timTheoId` SAU KHI
gọi `khoMoRong.luu` VẪN tìm thấy dữ liệu — TẤT CẢ method (CẢ CŨ lẫn
MỚI) chia sẻ CÙNG MỘT `duLieu`.
::::

::::predict{#doan-spread-chia-se-closure commitOnce}
```typescript
type Repository<Id, ThucThe> = { timTheoId: (id: Id) => ThucThe | null; layTatCa: () => ThucThe[]; luu: (thucThe: ThucThe) => void; xoa: (id: Id) => void };
function taoKhoBoNho<Id extends string, ThucThe extends { id: Id }>(): Repository<Id, ThucThe> {
  const duLieu = new Map<Id, ThucThe>();
  return { timTheoId: (id) => duLieu.get(id) ?? null, layTatCa: () => [...duLieu.values()], luu: (thucThe) => { duLieu.set(thucThe.id, thucThe); }, xoa: (id) => { duLieu.delete(id); } };
}
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { id: MaDonHang; maKhachHang: string; tongTien: number };
type KhoDonHang = Repository<MaDonHang, DonHang> & { timTheoKhachHang: (maKhachHang: string) => DonHang[] };

function taoKhoDonHangMoRong(): KhoDonHang {
  const co = taoKhoBoNho<MaDonHang, DonHang>();
  return { ...co, timTheoKhachHang: (mkh) => co.layTatCa().filter((dh) => dh.maKhachHang === mkh) };
}

const khoMoRong = taoKhoDonHangMoRong();
// LUU qua khoMoRong (đã spread) -- rồi ĐỌC qua khoMoRong.timTheoId
khoMoRong.luu({ id: "DH-20" as MaDonHang, maKhachHang: "KH-08", tongTien: 3000 });
console.log(khoMoRong.timTheoId("DH-20" as MaDonHang));
```

Dòng cuối in ra gì?

:::opt{correct}
`{ id: 'DH-20', maKhachHang: 'KH-08', tongTien: 3000 }`
:::

:::opt
`null` — vì `{ ...co, timTheoKhachHang: ... }` TẠO MỘT OBJECT MỚI, và
việc SAO CHÉP tách `khoMoRong` khỏi `co` HOÀN TOÀN — `khoMoRong.luu`
ghi vào MỘT `duLieu` khác với `khoMoRong.timTheoId` đọc từ
::why
Gần đúng ở việc bạn nhớ ĐÚNG `{ ...co, ... }` TẠO một OBJECT MỚI (một
object literal riêng biệt, khác `co` VỀ MẶT THAM CHIẾU OBJECT) — quan
sát về việc CÓ một object mới đó đúng.

Chỗ lệch: spread (`...co`) chỉ sao chép **CÁC PROPERTY** của `co` —
với `co.luu` (một HÀM), "sao chép" nghĩa là `khoMoRong.luu` TRỎ tới
**CHÍNH XÁC CÙNG MỘT HÀM** mà `co.luu` trỏ tới (không tạo hàm MỚI).
Hàm ĐÓ (được định nghĩa bên trong `taoKhoBoNho`) đã "ĐÓNG" quanh
`duLieu` từ LÚC được TẠO — spread KHÔNG "cắt đứt" mối quan hệ đó.
`khoMoRong.timTheoId` CŨNG LÀ CHÍNH `co.timTheoId`, cùng đóng quanh
CÙNG `duLieu`. Ghi qua method NÀO, đọc qua method NÀO (miễn CÙNG object
gốc `co`) — đều thấy DỮ LIỆU GIỐNG NHAU.
::
:::

:::opt
Máy báo lỗi biên dịch — `taoKhoDonHangMoRong` trả về kiểu `KhoDonHang`
(giao của `Repository<MaDonHang, DonHang>` VÀ một object có
`timTheoKhachHang`), nhưng object literal `{ ...co, timTheoKhachHang:
...}` không CHỨNG MINH được nó khớp intersection type đó
::why
Gần đúng ở việc bạn để ý `KhoDonHang` là một **intersection type**
(`&`, GIAO của hai type) — một cấu trúc kiểu PHỨC TẠP hơn union bình
thường, đáng để cẩn trọng.

Chỗ lệch: object literal `{ ...co, timTheoKhachHang: ... }` CÓ ĐỦ MỌI
property intersection type ĐÒI HỎI — bốn property từ `co` (khớp
`Repository<MaDonHang, DonHang>`) CỘNG `timTheoKhachHang` (khớp phần
CÒN LẠI) — TypeScript XÁC NHẬN object có ĐỦ HÌNH DẠNG của CẢ HAI phần
giao nhau. Biên dịch sạch.
::
:::
::::

::::code{#viet_taokhobonho}
Tự viết BA method của `taoKhoBoNho` (generic).

```typescript title=starter
type Repository<Id, ThucThe> = {
  timTheoId: (id: Id) => ThucThe | null;
  layTatCa: () => ThucThe[];
  luu: (thucThe: ThucThe) => void;
  xoa: (id: Id) => void;
};

function taoKhoBoNho<Id extends string, ThucThe extends { id: Id }>(): Repository<Id, ThucThe> {
  const duLieu = new Map<Id, ThucThe>();
  return {
    timTheoId: (id) => ___,
    layTatCa: () => [...duLieu.values()],
    luu: (thucThe) => { ___; },
    xoa: (id) => { ___; },
  };
}

type MaSanPham = string & { readonly __brand: "MaSanPham" };
type SanPham = { id: MaSanPham; ten: string; gia: number };
const kho = taoKhoBoNho<MaSanPham, SanPham>();
kho.luu({ id: "SP-01" as MaSanPham, ten: "Bút chì", gia: 5000 });
console.log(JSON.stringify(kho.timTheoId("SP-01" as MaSanPham)));
```

```typescript title=solution
type Repository<Id, ThucThe> = {
  timTheoId: (id: Id) => ThucThe | null;
  layTatCa: () => ThucThe[];
  luu: (thucThe: ThucThe) => void;
  xoa: (id: Id) => void;
};

function taoKhoBoNho<Id extends string, ThucThe extends { id: Id }>(): Repository<Id, ThucThe> {
  const duLieu = new Map<Id, ThucThe>();
  return {
    timTheoId: (id) => duLieu.get(id) ?? null,
    layTatCa: () => [...duLieu.values()],
    luu: (thucThe) => { duLieu.set(thucThe.id, thucThe); },
    xoa: (id) => { duLieu.delete(id); },
  };
}

type MaSanPham = string & { readonly __brand: "MaSanPham" };
type SanPham = { id: MaSanPham; ten: string; gia: number };
const kho = taoKhoBoNho<MaSanPham, SanPham>();
kho.luu({ id: "SP-01" as MaSanPham, ten: "Bút chì", gia: 5000 });
console.log(JSON.stringify(kho.timTheoId("SP-01" as MaSanPham)));
```

```typescript title=test
type MaTest = string & { readonly __brand: "MaTest" };
type ThucTheTest = { id: MaTest; ten: string };
const khoTest = taoKhoBoNho<MaTest, ThucTheTest>();

if (khoTest.timTheoId("X-01" as MaTest) !== null) throw new Error("kho rỗng phải trả về null");

khoTest.luu({ id: "X-01" as MaTest, ten: "A" });
const timDuoc = khoTest.timTheoId("X-01" as MaTest);
if (timDuoc === null) throw new Error("luu() phải THẬT SỰ ghi vào Map");
if (timDuoc.ten !== "A") throw new Error("dữ liệu tìm được phải khớp đúng dữ liệu đã luu");

if (khoTest.layTatCa().length !== 1) throw new Error("layTatCa() phải trả đúng số lượng đã luu");

khoTest.xoa("X-01" as MaTest);
if (khoTest.timTheoId("X-01" as MaTest) !== null) throw new Error("xoa() phải THẬT SỰ xoá khỏi Map");

// Generic dùng được cho kiểu entity KHÁC hoàn toàn
type MaTest2 = string & { readonly __brand: "MaTest2" };
type ThucTheTest2 = { id: MaTest2; gia: number };
const khoTest2 = taoKhoBoNho<MaTest2, ThucTheTest2>();
khoTest2.luu({ id: "Y-01" as MaTest2, gia: 999 });
if (khoTest2.timTheoId("Y-01" as MaTest2)?.gia !== 999) throw new Error("taoKhoBoNho phải dùng được cho MỌI entity có field id, không riêng một loại");
```

:::hints
- kind: attention
  body: "Giống hệt bài 38's taoKhoDonHangBoNho, chỉ khác: dùng thucThe.id (field CHUNG, generic) thay vì dh.ma (field riêng của DonHang)."
- kind: strategy
  body: "duLieu.get(id) ?? null : duLieu.set(thucThe.id, thucThe) : duLieu.delete(id) — ba thao tác Map chuẩn, key luôn LÀ id."
- kind: one-line
  body: "___ (timTheoId) = duLieu.get(id) ?? null\n___ (luu) = duLieu.set(thucThe.id, thucThe)\n___ (xoa) = duLieu.delete(id)"
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
Generic Repository: MỘT implementation, dùng cho MỌI entity. Mở rộng
bằng spread khi cần method riêng. Bước tiếp theo: đổi NHIỀU repository
cùng lúc, an toàn — cùng thành công hoặc cùng thất bại.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đặt MỘT đơn hàng CẦN đổi CẢ `KhoDonHang` (thêm đơn) VÀ một
`KhoSanPham` (trừ tồn kho) CÙNG LÚC. Nếu bước THỨ HAI thất bại (hết
hàng), bước ĐẦU đã lưu rồi thì sao? Cần công cụ gì để đảm bảo CẢ HAI
cùng thành công hoặc cùng thất bại?
::::

::::checkpoint{mastery=0.8}
::::
