---
id: ky-nghe-phan-mem.ddd.repository-pattern-va-dip
title: "Repository Pattern — Domain định nghĩa interface, Infrastructure implement"
summary: "Repository = lớp trung gian giữa domain và nơi lưu trữ, ẩn dụ \"thủ thư\". Dependency Inversion Principle: domain TỰ định nghĩa interface nó cần, KHÔNG import infrastructure; infra SAU ĐÓ implement khế ước đó. FP DI (nối bài 10): domain function nhận kho làm THAM SỐ ĐẦU TIÊN."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 37
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.repository-pattern]
requires: [ddd.dto-versioning]
concepts: [ddd.repository-pattern]
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
Cụm CUỐI. Domain object cần được LƯU và ĐỌC lại — nhưng domain logic
có nên BIẾT dữ liệu nằm ở database nào, bảng nào, cột nào không?
::::

::::explain{#repository-la-thu-thu}
**Repository** = lớp trung gian giữa domain và NƠI LƯU TRỮ — ẩn dụ
"thủ thư": domain HỎI "tìm đơn hàng DH-001", thủ thư đi vào "kho" tìm
và TRẢ VỀ domain object — domain **KHÔNG BIẾT** dữ liệu nằm Ở ĐÂU
(database? file? bộ nhớ?).

**Dependency Inversion Principle (DIP)**: domain **TỰ định nghĩa**
interface NÓ CẦN, NGAY TRONG domain layer — **KHÔNG import** bất kỳ
infrastructure nào. Infrastructure SAU ĐÓ implement KHẾ ƯỚC đó:

```typescript
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };

// Domain TỰ định nghĩa interface -- KHÔNG import bất kỳ infra nào
type KhoDonHang = {
  timTheoId: (ma: MaDonHang) => DonHang | null;
  timTheoKhachHang: (maKhachHang: string) => DonHang[];
  luu: (dh: DonHang) => void;
  xoa: (ma: MaDonHang) => void;
};

// Domain function nhận kho làm THAM SỐ ĐẦU TIÊN (FP DI, bài 10)
function timDonHangTheoId(kho: KhoDonHang, ma: MaDonHang): DonHang | null {
  return kho.timTheoId(ma);
}
function demSoDonHangCuaKhachHang(kho: KhoDonHang, maKhachHang: string): number {
  return kho.timTheoKhachHang(maKhachHang).length;
}

const duLieuGia: DonHang[] = [
  { ma: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 },
  { ma: "DH-02" as MaDonHang, maKhachHang: "KH-01", tongTien: 200000 },
];
const khoGiaDon: KhoDonHang = {
  timTheoId: (ma) => duLieuGia.find((dh) => dh.ma === ma) ?? null,
  timTheoKhachHang: (mkh) => duLieuGia.filter((dh) => dh.maKhachHang === mkh),
  luu: (dh) => { duLieuGia.push(dh); },
  xoa: () => {},
};

console.log(timDonHangTheoId(khoGiaDon, "DH-01" as MaDonHang));
console.log(demSoDonHangCuaKhachHang(khoGiaDon, "KH-01"));
console.log(timDonHangTheoId(khoGiaDon, "DH-99" as MaDonHang));
```

```text
{ ma: 'DH-01', maKhachHang: 'KH-01', tongTien: 100000 }
2
null
```

`timDonHangTheoId`/`demSoDonHangCuaKhachHang` KHÔNG hề biết
`khoGiaDon` cài đặt bằng MẢNG — chúng CHỈ gọi `kho.timTheoId(...)`,
`kho.timTheoKhachHang(...)` theo ĐÚNG khế ước `KhoDonHang`. Đây LÀ
"DEPENDENCY **INVERSION**": THÔNG THƯỜNG domain sẽ PHỤ THUỘC vào
infrastructure (import database client) — Ở ĐÂY, HƯỚNG PHỤ THUỘC bị
**ĐẢO NGƯỢC**: infrastructure PHẢI khớp KHẾ ƯỚC domain đặt ra, không
phải ngược lại.
::::

::::example{#swappability}
Hệ quả QUAN TRỌNG NHẤT (nhấn mạnh XUYÊN SUỐT cụm này): vì domain CHỈ
code CHỐNG LẠI interface, đổi HẲN cách LƯU TRỮ (mảng → `Map`, hoàn
toàn khác cấu trúc) **KHÔNG cần sửa MỘT DÒNG** domain code:

```typescript title=readonly
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type KhoDonHang = { timTheoId: (ma: MaDonHang) => DonHang | null; timTheoKhachHang: (maKhachHang: string) => DonHang[]; luu: (dh: DonHang) => void; xoa: (ma: MaDonHang) => void };
function timDonHangTheoId(kho: KhoDonHang, ma: MaDonHang): DonHang | null {
  return kho.timTheoId(ma);
}

// Implementation THỨ NHẤT: mảng
const mangDuLieu: DonHang[] = [{ ma: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 }];
const khoMang: KhoDonHang = {
  timTheoId: (ma) => mangDuLieu.find((dh) => dh.ma === ma) ?? null,
  timTheoKhachHang: (mkh) => mangDuLieu.filter((dh) => dh.maKhachHang === mkh),
  luu: (dh) => { mangDuLieu.push(dh); },
  xoa: () => {},
};

// Implementation THỨ HAI: Map -- cấu trúc lưu trữ HOÀN TOÀN khác
const banDoDuLieu = new Map<string, DonHang>([["DH-01", { ma: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 }]]);
const khoMap: KhoDonHang = {
  timTheoId: (ma) => banDoDuLieu.get(ma) ?? null,
  timTheoKhachHang: (mkh) => [...banDoDuLieu.values()].filter((dh) => dh.maKhachHang === mkh),
  luu: (dh) => { banDoDuLieu.set(dh.ma, dh); },
  xoa: () => {},
};

// CÙNG MỘT domain function, HAI implementation KHÁC NHAU -- không sửa gì
console.log(JSON.stringify(timDonHangTheoId(khoMang, "DH-01" as MaDonHang)));
console.log(JSON.stringify(timDonHangTheoId(khoMap, "DH-01" as MaDonHang)));
```

```text title=readonly
{"ma":"DH-01","maKhachHang":"KH-01","tongTien":100000}
{"ma":"DH-01","maKhachHang":"KH-01","tongTien":100000}
```

`timDonHangTheoId` gọi ĐÚNG MỘT DÒNG code GIỐNG HỆT cho CẢ HAI —
KHÔNG có `if (dùng mảng) ... else (dùng Map) ...` nào cả. Đây LÀ
**swappability**: sau này đổi từ "kho giả" (bài học) sang database
THẬT, domain logic KHÔNG đổi GÌ — chỉ cần MỘT implementation MỚI khớp
`KhoDonHang`.
::::

::::predict{#doan-them-method-vao-kho commitOnce}
```typescript
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type KhoDonHang = {
  timTheoId: (ma: MaDonHang) => DonHang | null;
  timTheoKhachHang: (maKhachHang: string) => DonHang[];
  luu: (dh: DonHang) => void;
  xoa: (ma: MaDonHang) => void;
};

const duLieu: DonHang[] = [{ ma: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 }];

// THIẾU field "xoa" khi khai implementation
const khoThieu: KhoDonHang = {
  timTheoId: (ma) => duLieu.find((dh) => dh.ma === ma) ?? null,
  timTheoKhachHang: (mkh) => duLieu.filter((dh) => dh.maKhachHang === mkh),
  luu: (dh) => { duLieu.push(dh); },
};
console.log(khoThieu);
```

Chuyện gì xảy ra?

:::opt{correct}
Máy báo lỗi biên dịch
:::

:::opt
Chạy được BÌNH THƯỜNG — object literal `khoThieu` khai BA field
(`timTheoId`, `timTheoKhachHang`, `luu`), TypeScript CHỈ kiểm những
field NÀO CÓ MẶT, KHÔNG bắt buộc phải khai HẾT mọi field của interface
::why
Gần đúng ở việc bạn nhớ ĐÚNG `khoThieu` CÓ khai đủ ba field
(`timTheoId`, `timTheoKhachHang`, `luu`) — một quan sát chính xác về
NHỮNG GÌ được viết ra.

Chỗ lệch: `KhoDonHang` khai BỐN field, TẤT CẢ đều **BẮT BUỘC** (không
có dấu `?`) — một biến khai kiểu `KhoDonHang` PHẢI có ĐỦ CẢ BỐN,
KHÔNG được thiếu field nào. TypeScript báo lỗi NGAY: `Property "xoa"
is missing in type '{...}' but required in type 'KhoDonHang'`. Đây
CHÍNH LÀ ĐIỂM MẠNH của interface: MỘT implementation THIẾU method BỊ
PHÁT HIỆN lúc BIÊN DỊCH, không phải khi CHẠY code gọi `.xoa(...)` và
crash vì `undefined is not a function`.
::
:::

:::opt
Chạy được, NHƯNG khi có code khác GỌI `khoThieu.xoa(...)`, chương
trình sẽ CRASH LÚC CHẠY vì method đó không tồn tại — TypeScript CHỈ
cảnh báo (warning), không NGĂN biên dịch hoàn toàn
::why
Gần đúng ở việc bạn nghĩ tới hậu quả THẬT nếu thiếu `.xoa` mà code
KHÁC gọi tới nó — MỘT lo ngại hợp lý nếu ngôn ngữ chỉ cảnh báo mà
không chặn.

Chỗ lệch: TypeScript ở chế độ `strict` (dùng xuyên suốt track này)
KHÔNG chỉ "cảnh báo" — nó **CHẶN HẲN việc biên dịch**. Dòng
`const khoThieu: KhoDonHang = {...}` (thiếu field bắt buộc) là LỖI
KIỂU nghiêm trọng (mức `error`, không phải `warning`), code KHÔNG
BAO GIỜ được phép chạy — vấn đề bị phát hiện SỚM HƠN NHIỀU so với lúc
`.xoa` thật sự bị gọi.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Repository: domain định nghĩa khế ước, infrastructure implement.
Swappability là hệ quả — đổi cách lưu trữ không sửa domain. Bước tiếp
theo: viết implementation ĐẦU TIÊN.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Implementation ĐƠN GIẢN nhất của `KhoDonHang` — không cần database
thật, chỉ cần khớp khế ước — trông thế nào? Nó có dùng được cho TEST
không?
::::

::::checkpoint{mastery=0.8}
::::
