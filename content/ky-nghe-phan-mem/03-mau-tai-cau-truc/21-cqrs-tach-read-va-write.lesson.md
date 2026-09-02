---
id: ky-nghe-phan-mem.mau-tai-cau-truc.cqrs-tach-read-va-write
title: "CQRS — Tách mô hình ĐỌC khỏi mô hình GHI"
summary: "CQRS: thay vì MỘT model dùng chung cho ghi VÀ đọc, tách thành HAI — Write model (nhận Command, validate, SINH sự kiện) VÀ Read model (denormalized, tối ưu cho TRUY VẤN). xuLyLenhTaoDonHang(cmd): SuKien[] hoàn toàn TÁCH RỜI khỏi xemDanhSachDonHang(): DonHangView[] — hai hàm, hai shape dữ liệu, không cần giống nhau."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 21
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.cqrs-read-write-split]
requires: [mau.gate-boss-middleware-refactor]
concepts: [mau.cqrs-read-write-split]
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
Cụm CUỐI. "TẠO đơn hàng" (ghi) VÀ "XEM danh sách đơn hàng" (đọc) —
tại sao KHÔNG dùng CHUNG một model cho cả hai?
::::

::::explain{#tach-read-write}
CQRS (Command Query Responsibility Segregation): thay vì MỘT model
DÙNG CHUNG cho cả GHI VÀ ĐỌC (thường phải THOẢ HIỆP — model TỐT cho
ghi (validate chặt, ÍT trường) LẠI KHÔNG tối ưu cho đọc (cần NHIỀU
view khác nhau)), TÁCH thành HAI: **Write model** (nhận Command,
validate, SINH sự kiện) VÀ **Read model** (denormalized, tối ưu cho
TRUY VẤN cụ thể):

```typescript title=readonly
type LenhTaoDonHang = { khachHang: string; mucGia: number[] };
type SuKienDonHang = { tag: "daTaoDonHang"; maDon: string; khachHang: string; tongTien: number };

// WRITE: nhận Command, validate, SINH sự kiện
function xuLyLenhTaoDonHang(lenh: LenhTaoDonHang, maDon: string): SuKienDonHang[] {
  const tongTien = lenh.mucGia.reduce((t, g) => t + g, 0);
  if (tongTien <= 0) return [];
  return [{ tag: "daTaoDonHang", maDon, khachHang: lenh.khachHang, tongTien }];
}

type DonHangView = { maDon: string; khachHang: string; tongTien: number };

// READ: đọc TỪ sự kiện, tạo view TỐI ƯU cho hiển thị
function xemDanhSachDonHang(cacSuKien: SuKienDonHang[]): DonHangView[] {
  return cacSuKien
    .filter((sk) => sk.tag === "daTaoDonHang")
    .map((sk) => ({ maDon: sk.maDon, khachHang: sk.khachHang, tongTien: sk.tongTien }));
}

let cacSuKien: SuKienDonHang[] = [];
cacSuKien = [...cacSuKien, ...xuLyLenhTaoDonHang({ khachHang: "An", mucGia: [100, 200] }, "D1")];
cacSuKien = [...cacSuKien, ...xuLyLenhTaoDonHang({ khachHang: "Binh", mucGia: [50] }, "D2")];

console.log(xemDanhSachDonHang(cacSuKien));
```

```text title=readonly
[{"maDon":"D1","khachHang":"An","tongTien":300},{"maDon":"D2","khachHang":"Binh","tongTien":50}]
```

`xuLyLenhTaoDonHang` (write) VÀ `xemDanhSachDonHang` (read) LÀ HAI
HÀM HOÀN TOÀN TÁCH RỜI — HAI SHAPE dữ liệu KHÁC NHAU
(`SuKienDonHang[]` VS `DonHangView[]`), KHÔNG cần giống nhau, KHÔNG
gọi lẫn nhau.
::::

::::example{#ghi-tu-choi-doc-khong-thay-doi}
Write model CÓ VALIDATE (từ chối đơn hàng KHÔNG hợp lệ) — Read model
CHỈ đọc NHỮNG gì write model ĐÃ chấp nhận:

```typescript title=readonly
type LenhTaoDonHang = { khachHang: string; mucGia: number[] };
type SuKienDonHang = { tag: "daTaoDonHang"; maDon: string; khachHang: string; tongTien: number };
function xuLyLenhTaoDonHang(lenh: LenhTaoDonHang, maDon: string): SuKienDonHang[] {
  const tongTien = lenh.mucGia.reduce((t, g) => t + g, 0);
  if (tongTien <= 0) return [];
  return [{ tag: "daTaoDonHang", maDon, khachHang: lenh.khachHang, tongTien }];
}

const suKienRong = xuLyLenhTaoDonHang({ khachHang: "Chi", mucGia: [] }, "D3");
console.log(suKienRong.length);
```

```text title=readonly
0
```

Đơn hàng KHÔNG có sản phẩm nào (`mucGia: []`) → `tongTien = 0` → BỊ
TỪ CHỐI (`return []`, KHÔNG sinh sự kiện NÀO) — "D3" SẼ KHÔNG BAO GIỜ
xuất hiện Ở `xemDanhSachDonHang`, vì KHÔNG có SỰ KIỆN nào ghi nhận nó
đã TỪNG tồn tại.
::::

::::predict{#doan-don-hang-rong-bi-tu-choi commitOnce}
```typescript
type LenhTaoDonHang = { khachHang: string; mucGia: number[] };
type SuKienDonHang = { tag: "daTaoDonHang"; maDon: string; khachHang: string; tongTien: number };
function xuLyLenhTaoDonHang(lenh: LenhTaoDonHang, maDon: string): SuKienDonHang[] {
  const tongTien = lenh.mucGia.reduce((t, g) => t + g, 0);
  if (tongTien <= 0) return [];
  return [{ tag: "daTaoDonHang", maDon, khachHang: lenh.khachHang, tongTien }];
}

const suKien = xuLyLenhTaoDonHang({ khachHang: "Chi", mucGia: [] }, "D3");
console.log(suKien.length);
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì `xuLyLenhTaoDonHang` LUÔN sinh ĐÚNG MỘT sự kiện cho MỖI
lệnh gọi, KHÔNG PHỤ THUỘC nội dung `mucGia` — hàm CHỈ "ghi lại" lệnh
đã ĐƯỢC gọi, KHÔNG "từ chối" gì cả
::why
Gần đúng ở việc bạn nhớ ĐÚNG `xuLyLenhTaoDonHang` LÀ hàm "ghi" (write)
— quan sát ĐÓ đúng HƯỚNG chức năng.

Chỗ lệch: `xuLyLenhTaoDonHang` **CÓ VALIDATE**, KHÔNG CHỈ đơn thuần
"ghi lại lệnh" — dòng `const tongTien = lenh.mucGia.reduce(...)` tính
`tongTien = 0` (mảng RỖNG), VÀ `if (tongTien <= 0) return [];` TỪ
CHỐI NGAY (trả MẢNG RỖNG, KHÔNG sinh sự kiện nào). `suKien.length`
LÀ `0`, KHÔNG PHẢI `1` — write model TỪ CHỐI đơn hàng KHÔNG hợp lệ,
ĐÚNG vai trò VALIDATE của nó.
::
:::

:::opt
Máy báo lỗi biên dịch — `xuLyLenhTaoDonHang` khai kiểu trả về
`SuKienDonHang[]`, NHƯNG nhánh `if (tongTien <= 0) return [];` trả
về MẢNG RỖNG (KHÔNG khớp `SuKienDonHang`), TypeScript CẤM trả về
mảng RỖNG cho kiểu MẢNG CÓ PHẦN TỬ cụ thể
::why
Gần đúng ở việc bạn để ý `SuKienDonHang[]` LÀ kiểu MẢNG chứa PHẦN TỬ
CỤ THỂ (`SuKienDonHang`) — một quan sát ĐÚNG về CẤU TRÚC kiểu.

Chỗ lệch: `T[]` (mảng CỦA `T`) LUÔN chấp nhận mảng **RỖNG** (`[]`)
LÀM giá trị hợp lệ — "mảng CỦA T" nghĩa LÀ "MỘT mảng mà MỖI phần tử
(NẾU CÓ) khớp `T`", KHÔNG BẮT BUỘC phải CÓ phần tử nào. `return []`
khớp `SuKienDonHang[]` HOÀN TOÀN hợp lệ — biên dịch SẠCH.
::
:::
::::

::::code{#viet_cqrs_tach_read_write}
Hoàn thiện `xuLyLenhTaoDonHang` (write, sinh sự kiện) VÀ
`xemDanhSachDonHang` (read, tạo view).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LenhTaoDonHang = { khachHang: string; mucGia: number[] };
type SuKienDonHang = { tag: "daTaoDonHang"; maDon: string; khachHang: string; tongTien: number };

function xuLyLenhTaoDonHang(lenh: LenhTaoDonHang, maDon: string): SuKienDonHang[] {
  const tongTien = lenh.mucGia.reduce((t, g) => t + g, 0);
  if (tongTien <= 0) return [];
  return ___;
}

type DonHangView = { maDon: string; khachHang: string; tongTien: number };

function xemDanhSachDonHang(cacSuKien: SuKienDonHang[]): DonHangView[] {
  return ___;
}

const sk1 = xuLyLenhTaoDonHang({ khachHang: "An", mucGia: [100, 200] }, "D1");
assertEqual(sk1.length, 1, "mot su kien duoc tao");
assertEqual(sk1[0]!.tongTien, 300, "tong tien dung");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LenhTaoDonHang = { khachHang: string; mucGia: number[] };
type SuKienDonHang = { tag: "daTaoDonHang"; maDon: string; khachHang: string; tongTien: number };

function xuLyLenhTaoDonHang(lenh: LenhTaoDonHang, maDon: string): SuKienDonHang[] {
  const tongTien = lenh.mucGia.reduce((t, g) => t + g, 0);
  if (tongTien <= 0) return [];
  return [{ tag: "daTaoDonHang", maDon, khachHang: lenh.khachHang, tongTien }];
}

type DonHangView = { maDon: string; khachHang: string; tongTien: number };

function xemDanhSachDonHang(cacSuKien: SuKienDonHang[]): DonHangView[] {
  return cacSuKien
    .filter((sk) => sk.tag === "daTaoDonHang")
    .map((sk) => ({ maDon: sk.maDon, khachHang: sk.khachHang, tongTien: sk.tongTien }));
}

const sk1 = xuLyLenhTaoDonHang({ khachHang: "An", mucGia: [100, 200] }, "D1");
assertEqual(sk1.length, 1, "mot su kien duoc tao");
assertEqual(sk1[0]!.tongTien, 300, "tong tien dung");
```

```typescript title=test
const sk2 = xuLyLenhTaoDonHang({ khachHang: "Binh", mucGia: [] }, "D2");
assertEqual(sk2.length, 0, "khong tao su kien neu tong tien khong duong");

const view = xemDanhSachDonHang([...sk1, ...sk2]);
assertEqual(view.length, 1, "view chi chua don hang hop le");
assertEqual(view[0]!.khachHang, "An", "view dung khach hang");
```

:::hints
- kind: attention
  body: "xuLyLenhTaoDonHang: mảng MỘT sự kiện {tag:\"daTaoDonHang\", maDon, khachHang: lenh.khachHang, tongTien}. xemDanhSachDonHang: lọc đúng tag, RỒI ánh xạ ba trường cần thiết."
- kind: strategy
  body: '[{ tag: "daTaoDonHang", maDon, khachHang: lenh.khachHang, tongTien }] : cacSuKien.filter((sk) => sk.tag === "daTaoDonHang").map((sk) => ({ maDon: sk.maDon, khachHang: sk.khachHang, tongTien: sk.tongTien }))'
- kind: one-line
  body: '___ (write) = [{ tag: "daTaoDonHang", maDon, khachHang: lenh.khachHang, tongTien }]\n___ (read) = cacSuKien.filter((sk) => sk.tag === "daTaoDonHang").map((sk) => ({ maDon: sk.maDon, khachHang: sk.khachHang, tongTien: sk.tongTien }))'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
CQRS: write validate rồi sinh sự kiện, read đọc sự kiện tạo view.
Bài tiếp theo: sự kiện KHÔNG CHỈ phục vụ đọc — nó CÒN LÀ nguồn sự
thật duy nhất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`cacSuKien` hiện LÀ một mảng "sống" TRONG bộ nhớ — nếu chương trình
TẮT ĐI, sự kiện MẤT HẾT. Nếu, THAY VÌ lưu "trạng thái HIỆN TẠI" (tổng
đơn hàng), ta CHỈ lưu TOÀN BỘ CHUỖI sự kiện ĐÃ xảy ra — trạng thái
HIỆN TẠI được TÍNH LẠI thế nào?
::::

::::checkpoint{mastery=0.8}
::::
