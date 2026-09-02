---
id: ky-nghe-phan-mem.mau-tai-cau-truc.tai-cau-truc-extract-function
title: "Tái cấu trúc: Extract Function — cắt hàm DÀI thành các hàm NHỎ, ĐẶT TÊN"
summary: "Kỹ thuật refactoring phổ biến nhất: hàm DÀI làm NHIỀU việc (validate + tính toán + giảm giá TRỘN LẪN) được CẮT thành nhiều hàm NHỎ, MỖI hàm MỘT trách nhiệm, ĐẶT TÊN theo Ý ĐỊNH. xuLyDonHang (validate + tinh tong + ap dung giam gia TRỘN LẪN) → validateDonHang/tinhTongTien/apDungGiamGia RIÊNG — hành vi TỔNG THỂ giữ NGUYÊN, tinhTongTien còn TÁI DÙNG được độc lập."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 18
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.refactor-extract-function]
requires: [mau.middleware-as-pipeline]
concepts: [mau.refactor-extract-function]
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
Một hàm `xuLyDonHang` DÀI: validate, tính tổng, áp giảm giá — TẤT CẢ
TRỘN LẪN trong MỘT khối. Cắt nó ra thành TỪNG bước NHỎ, cách nào?
::::

::::explain{#extract-function}
Kỹ thuật refactoring PHỔ BIẾN NHẤT: một hàm DÀI làm NHIỀU việc được
**CẮT** thành NHIỀU hàm NHỎ, MỖI hàm MỘT trách nhiệm, ĐẶT TÊN theo Ý
ĐỊNH (KHÔNG PHẢI `"buoc1"`/`"buoc2"`):

```typescript title=readonly
type DonHang = { mucGia: number[]; maGiamGia: string | null };
type KetQuaDonHang = { hopLe: boolean; tongTien: number };

// V1: DÀI, ba việc TRỘN LẪN trong MỘT hàm
function xuLyDonHangV1(dh: DonHang): KetQuaDonHang {
  if (dh.mucGia.length === 0) return { hopLe: false, tongTien: 0 };
  for (const gia of dh.mucGia) {
    if (gia < 0) return { hopLe: false, tongTien: 0 };
  }
  let tong = 0;
  for (const gia of dh.mucGia) tong += gia;
  if (dh.maGiamGia === "GIAM10") tong = tong * 0.9;
  return { hopLe: true, tongTien: tong };
}

// V2: TÁCH ba việc thành ba hàm RIÊNG, ĐẶT TÊN theo Ý ĐỊNH
function validateDonHang(dh: DonHang): boolean {
  if (dh.mucGia.length === 0) return false;
  return dh.mucGia.every((gia) => gia >= 0);
}
function tinhTongTien(mucGia: number[]): number {
  return mucGia.reduce((tong, gia) => tong + gia, 0);
}
function apDungGiamGia(tongTien: number, maGiamGia: string | null): number {
  return maGiamGia === "GIAM10" ? tongTien * 0.9 : tongTien;
}
function xuLyDonHangV2(dh: DonHang): KetQuaDonHang {
  if (!validateDonHang(dh)) return { hopLe: false, tongTien: 0 };
  const tong = tinhTongTien(dh.mucGia);
  return { hopLe: true, tongTien: apDungGiamGia(tong, dh.maGiamGia) };
}

const dh: DonHang = { mucGia: [100, 200, 300], maGiamGia: "GIAM10" };
console.log(xuLyDonHangV1(dh));
console.log(xuLyDonHangV2(dh));
```

```text title=readonly
{"hopLe":true,"tongTien":540}
{"hopLe":true,"tongTien":540}
```

`xuLyDonHangV1` VÀ `xuLyDonHangV2` cho **CÙNG HỆT** kết quả — hành vi
TỔNG THỂ (WHAT) GIỮ NGUYÊN, chỉ cấu trúc (HOW) đổi (nối TRỰC TIẾP kỷ
luật "Tinh gọn" đã học T5.2 bài 5). `xuLyDonHangV2` giờ ĐỌC như MỘT
CÂU chuyện: kiểm hợp lệ → tính tổng → áp giảm giá.
::::

::::example{#tai-su-dung-doc-lap}
LỢI ÍCH THỰC TẾ của Extract Function: `tinhTongTien` giờ LÀ một hàm
**ĐỘC LẬP**, DÙNG được Ở BẤT KỲ đâu CẦN "tính tổng một mảng số" —
KHÔNG CHỈ bên trong `xuLyDonHang`:

```typescript title=readonly
function tinhTongTien(mucGia: number[]): number {
  return mucGia.reduce((tong, gia) => tong + gia, 0);
}
console.log(tinhTongTien([10, 20, 30]));
console.log(tinhTongTien([]));
```

```text title=readonly
60
0
```

`tinhTongTien([])` trả `0` AN TOÀN (KHÔNG lỗi) — vì `reduce` được cho
**GIÁ TRỊ KHỞI TẠO** RÕ RÀNG (`0`, tham số THỨ HAI của `reduce`).
Trong `xuLyDonHangV1` (bản DÀI), phép tính TƯƠNG TỰ BỊ CHÔN VÙI trong
MỘT vòng `for` — KHÔNG THỂ gọi RIÊNG được, dù logic BÊN TRONG hoàn
toàn có ÍCH cho việc KHÁC.
::::

::::predict{#doan-reduce-mang-rong commitOnce}
```typescript
function tinhTongTien(mucGia: number[]): number {
  return mucGia.reduce((tong, gia) => tong + gia, 0);
}
console.log(tinhTongTien([5, 15]));
console.log(tinhTongTien([]));
```

Hai dòng in ra gì?

:::opt{correct}
`20` rồi `0`
:::

:::opt
`20` rồi máy báo lỗi lúc chạy — vì `.reduce(...)` gọi trên MẢNG RỖNG
(`[]`) LUÔN ném lỗi ("Reduce of empty array with no initial value"),
KHÔNG có cách nào tính tổng của MỘT mảng RỖNG
::why
Gần đúng ở việc bạn nhớ ĐÚNG `Array.prototype.reduce` THẬT SỰ ném
lỗi trên mảng RỖNG — NHƯNG chỉ trong MỘT trường hợp cụ thể: quan sát
ĐÓ ĐÚNG MỘT PHẦN.

Chỗ lệch: `reduce` CHỈ ném lỗi trên mảng rỗng **KHI KHÔNG có giá trị
khởi tạo** (gọi `.reduce((a,b)=>a+b)` KHÔNG tham số thứ hai). Ở ĐÂY,
`tinhTongTien` gọi `.reduce((tong, gia) => tong + gia, 0)` — tham số
**THỨ HAI** (`0`) LÀ giá trị khởi tạo TƯỜNG MINH. Với mảng RỖNG,
`reduce` ĐƠN GIẢN trả về NGUYÊN giá trị khởi tạo ĐÓ (`0`), KHÔNG chạy
vòng lặp NÀO cả — KHÔNG lỗi. Đây CHÍNH LÀ lý do LUÔN truyền giá trị
khởi tạo cho `reduce`.
::
:::

:::opt
Máy báo lỗi biên dịch — `tinhTongTien` khai tham số `mucGia:
number[]`, nhưng `tinhTongTien([])` truyền một mảng RỖNG, TypeScript
suy luận KIỂU của mảng rỗng LÀ `never[]`, KHÔNG khớp `number[]`
::why
Gần đúng ở việc bạn để ý mảng RỖNG `[]` CÓ THỂ gây khó khăn cho việc
SUY LUẬN kiểu trong MỘT SỐ ngữ cảnh (ví dụ: `const a = [];` KHÔNG
biết `a` chứa gì) — một quan sát ĐÚNG VỀ NGUYÊN TẮC chung.

Chỗ lệch: Ở ĐÂY, `[]` được truyền TRỰC TIẾP LÀM đối số cho tham số
ĐÃ khai kiểu `mucGia: number[]` — TypeScript dùng NGAY kiểu THAM SỐ
ĐÃ khai (contextual typing) để hiểu `[]` LÀ `number[]` RỖNG, KHÔNG
suy luận `never[]` (điều ĐÓ chỉ xảy ra khi KHÔNG có ngữ cảnh kiểu
nào). Biên dịch SẠCH.
::
:::
::::

::::code{#viet_extract_function}
Hoàn thiện BA hàm được TÁCH RA — `validateDonHang`, `tinhTongTien`,
`apDungGiamGia` — sao cho `xuLyDonHang` (đã cho SẴN) hoạt động ĐÚNG.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type DonHang = { mucGia: number[]; maGiamGia: string | null };
type KetQuaDonHang = { hopLe: boolean; tongTien: number };

function validateDonHang(dh: DonHang): boolean {
  if (dh.mucGia.length === 0) return false;
  return ___;
}

function tinhTongTien(mucGia: number[]): number {
  return ___;
}

function apDungGiamGia(tongTien: number, maGiamGia: string | null): number {
  return ___;
}

function xuLyDonHang(dh: DonHang): KetQuaDonHang {
  if (!validateDonHang(dh)) return { hopLe: false, tongTien: 0 };
  const tong = tinhTongTien(dh.mucGia);
  return { hopLe: true, tongTien: apDungGiamGia(tong, dh.maGiamGia) };
}

const dh1: DonHang = { mucGia: [100, 200], maGiamGia: null };
assertEqual(xuLyDonHang(dh1).tongTien, 300, "tong tien khong giam gia");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type DonHang = { mucGia: number[]; maGiamGia: string | null };
type KetQuaDonHang = { hopLe: boolean; tongTien: number };

function validateDonHang(dh: DonHang): boolean {
  if (dh.mucGia.length === 0) return false;
  return dh.mucGia.every((gia) => gia >= 0);
}

function tinhTongTien(mucGia: number[]): number {
  return mucGia.reduce((tong, gia) => tong + gia, 0);
}

function apDungGiamGia(tongTien: number, maGiamGia: string | null): number {
  return maGiamGia === "GIAM10" ? tongTien * 0.9 : tongTien;
}

function xuLyDonHang(dh: DonHang): KetQuaDonHang {
  if (!validateDonHang(dh)) return { hopLe: false, tongTien: 0 };
  const tong = tinhTongTien(dh.mucGia);
  return { hopLe: true, tongTien: apDungGiamGia(tong, dh.maGiamGia) };
}

const dh1: DonHang = { mucGia: [100, 200], maGiamGia: null };
assertEqual(xuLyDonHang(dh1).tongTien, 300, "tong tien khong giam gia");
```

```typescript title=test
const dh2: DonHang = { mucGia: [100, 200, 300], maGiamGia: "GIAM10" };
assertEqual(xuLyDonHang(dh2).tongTien, 540, "giam 10% dung");
assertEqual(xuLyDonHang(dh2).hopLe, true, "don hang hop le");

const dh3: DonHang = { mucGia: [], maGiamGia: null };
assertEqual(xuLyDonHang(dh3).hopLe, false, "don hang rong khong hop le");

const dh4: DonHang = { mucGia: [50, -10], maGiamGia: null };
assertEqual(xuLyDonHang(dh4).hopLe, false, "gia am khong hop le");

const dh5: DonHang = { mucGia: [0, 100], maGiamGia: null };
assertEqual(xuLyDonHang(dh5).hopLe, true, "gia dung bang 0 la bien -- van hop le");

assertEqual(tinhTongTien([10, 20, 30]), 60, "tinhTongTien tai su dung doc lap");
```

:::hints
- kind: attention
  body: "validateDonHang: MỌI giá trong mucGia phải >= 0. tinhTongTien: cộng dồn TOÀN BỘ mucGia, bắt đầu từ 0. apDungGiamGia: nếu mã ĐÚNG \"GIAM10\" thì nhân 0.9, ngược lại giữ nguyên."
- kind: strategy
  body: 'dh.mucGia.every((gia) => gia >= 0) : mucGia.reduce((tong, gia) => tong + gia, 0) : maGiamGia === "GIAM10" ? tongTien * 0.9 : tongTien'
- kind: one-line
  body: '___ (validateDonHang) = dh.mucGia.every((gia) => gia >= 0)\n___ (tinhTongTien) = mucGia.reduce((tong, gia) => tong + gia, 0)\n___ (apDungGiamGia) = maGiamGia === "GIAM10" ? tongTien * 0.9 : tongTien'
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
Extract Function: cắt hàm dài thành nhiều hàm nhỏ, đặt tên theo ý
định, hành vi giữ nguyên. Bài tiếp theo: một kỹ thuật KHÁC — chuỗi
if/else dài.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`apDungGiamGia` dùng MỘT so sánh chuỗi (`maGiamGia === "GIAM10"`).
Nếu CÓ NHIỀU mã giảm giá (`"GIAM10"`, `"GIAM20"`, `"MIENPHI"`), một
chuỗi `if/else if` dài liệu có phải cách TỐT NHẤT?
::::

::::checkpoint{mastery=0.8}
::::
