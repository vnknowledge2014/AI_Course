---
id: ky-nghe-phan-mem.mau-tai-cau-truc.capstone-refactor-class-hierarchy
title: "Capstone: Tái cấu trúc class hierarchy Strategy → hàm"
summary: "Bài chốt cụm 1: cho một class hierarchy OOP thu nhỏ (interface PhiVanChuyen, hai class implement), viết LẠI thành CongThucPhi (kiểu hàm) + hai giá trị hàm + tinhTongPhi nhận strategy làm tham số. Hành vi giữ NGUYÊN (test so kết quả), chỉ cấu trúc đổi."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.gate-boss-strategy]
requires: [mau.when-interface-still-needed]
concepts: [mau.gate-boss-strategy]
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
Bài chốt cụm 1. Một dịch vụ giao hàng có HAI mức phí: tiêu chuẩn và
nhanh — viết bằng OOP hierarchy TRƯỚC, giờ chuyển thành FP.
::::

::::explain{#oop-truoc-fp-sau}
Bản OOP (KHÔNG chạy trong sandbox — chỉ minh hoạ CẤU TRÚC, giống cách
chương sách nguồn viết code OOP dưới dạng chú thích):

```typescript
// ❌ OOP: interface + một class RIÊNG cho MỖI mức phí
// interface PhiVanChuyen { tinh(khoiLuong: number): number }
// class PhiVanChuyenTieuChuan implements PhiVanChuyen {
//   tinh(khoiLuong: number): number { return 15000 + khoiLuong * 2000; }
// }
// class PhiVanChuyenNhanh implements PhiVanChuyen {
//   tinh(khoiLuong: number): number { return 30000 + khoiLuong * 3500; }
// }
// class TinhPhi {
//   constructor(private chienLuoc: PhiVanChuyen) {}
//   tinhTong(khoiLuong: number): number { return this.chienLuoc.tinh(khoiLuong); }
// }
```

BA khai báo (`interface` + hai `class`) + MỘT class thứ tư giữ tham
chiếu — CHỈ để biểu diễn "hai công thức tính phí". Bản FP (bài 2-3 đã
học kỹ thuật NÀY):

```typescript title=readonly
type CongThucPhi = (khoiLuong: number) => number;

const phiTieuChuan: CongThucPhi = (kl) => 15000 + kl * 2000;
const phiNhanh: CongThucPhi = (kl) => 30000 + kl * 3500;

function tinhTongPhi(khoiLuong: number, congThuc: CongThucPhi): number {
  return congThuc(khoiLuong);
}

console.log(tinhTongPhi(3, phiTieuChuan));
console.log(tinhTongPhi(3, phiNhanh));
```

```text title=readonly
21000
40500
```

`tinhTongPhi(3, phiTieuChuan)`: `15000 + 3 * 2000 = 21000`.
`tinhTongPhi(3, phiNhanh)`: `30000 + 3 * 3500 = 40500`. HAI GIÁ TRỊ
HÀM (`phiTieuChuan`/`phiNhanh`) THAY THẾ hoàn toàn `interface` + hai
`class` — `tinhTongPhi` THAY THẾ `TinhPhi` (không cần constructor,
không cần `this`).
::::

::::example{#danh-sach-strategy-tim-re-nhat}
Vì strategy CHỈ LÀ giá trị, MỘT **MẢNG** các strategy dùng được với
`.map`/`Math.min` TRỰC TIẾP — KHÔNG cần vòng lặp `instanceof` kiểm
từng loại class:

```typescript title=readonly
type CongThucPhi = (khoiLuong: number) => number;
const phiTieuChuan: CongThucPhi = (kl) => 15000 + kl * 2000;
const phiNhanh: CongThucPhi = (kl) => 30000 + kl * 3500;

function tinhTongPhi(khoiLuong: number, congThuc: CongThucPhi): number {
  return congThuc(khoiLuong);
}

const cacLuaChon = [phiTieuChuan, phiNhanh];
const ketQua = cacLuaChon.map((ct) => tinhTongPhi(5, ct));
console.log(ketQua);
console.log(Math.min(...ketQua));
```

```text title=readonly
[25000,47500]
25000
```

`cacLuaChon: CongThucPhi[]` LÀ một mảng GIÁ TRỊ HÀM — `.map` áp dụng
`tinhTongPhi` cho TỪNG strategy, `Math.min(...ketQua)` tìm phí RẺ
NHẤT. Trong OOP, làm điều NÀY cần MỘT mảng `PhiVanChuyen[]` (mảng
OBJECT, mỗi object "nhớ" LOẠI của chính nó) — Ở ĐÂY, mảng HÀM là đủ.
::::

::::predict{#doan-so-sanh-hai-muc-phi commitOnce}
```typescript
type CongThucPhi = (khoiLuong: number) => number;
const phiTieuChuan: CongThucPhi = (kl) => 15000 + kl * 2000;
const phiNhanh: CongThucPhi = (kl) => 30000 + kl * 3500;

function tinhTongPhi(khoiLuong: number, congThuc: CongThucPhi): number {
  return congThuc(khoiLuong);
}

console.log(tinhTongPhi(0, phiTieuChuan));
console.log(tinhTongPhi(0, phiNhanh));
```

Hai dòng in ra gì?

:::opt{correct}
`15000` rồi `30000`
:::

:::opt
`0` rồi `0` — vì `khoiLuong = 0`, VÀ CẢ HAI công thức đều CÓ phép
NHÂN `kl * 2000`/`kl * 3500`, NHÂN với `0` LUÔN cho kết quả `0`, làm
CẢ tổng trở thành `0`
::why
Gần đúng ở việc bạn tính ĐÚNG `kl * 2000` VÀ `kl * 3500` ĐỀU bằng `0`
khi `kl = 0` — phép NHÂN với không đúng LÀ `0`.

Chỗ lệch: mỗi công thức LÀ một phép **CỘNG** giữa CHI PHÍ CƠ BẢN
(`15000`/`30000`, KHÔNG phụ thuộc khối lượng) VÀ phần theo cân nặng
(`kl * 2000`/`kl * 3500`, ĐÚNG LÀ `0` khi `kl = 0`) — CHỈ phần THEO
CÂN bằng `0`, phần CƠ BẢN VẪN CÒN NGUYÊN. `15000 + 0 = 15000`,
`30000 + 0 = 30000`. Đây LÀ Ý NGHĨA THẬT của "chi phí cơ bản": PHÍ
TỐI THIỂU dù gói hàng NẶNG `0` kg.
::
:::

:::opt
Máy báo lỗi biên dịch — `tinhTongPhi` khai tham số THỨ HAI kiểu
`CongThucPhi`, nhưng `phiTieuChuan`/`phiNhanh` là HẰNG SỐ (`const`),
TypeScript CHỈ cho truyền HÀM được khai bằng `function`, KHÔNG PHẢI
hằng số gán GIÁ TRỊ hàm
::why
Gần đúng ở việc bạn để ý `phiTieuChuan`/`phiNhanh` được khai bằng
`const` (KHÔNG PHẢI `function ten(...) {...}`) — một quan sát ĐÚNG về
CÚ PHÁP khai báo.

Chỗ lệch: TypeScript KHÔNG phân biệt "hàm khai bằng `function`" VÀ
"hằng số gán giá trị hàm (arrow function)" — CẢ HAI đều LÀ MỘT GIÁ
TRỊ có kiểu HÀM, dùng được Ở BẤT KỲ đâu cần kiểu hàm ĐÓ. `const
phiTieuChuan: CongThucPhi = (kl) => ...` khớp CHÍNH XÁC kiểu tham số
thứ hai — biên dịch SẠCH.
::
:::
::::

::::code{#viet_capstone_strategy}
Hoàn thiện `phiNhanh` (công thức phí NHANH) VÀ `tinhTongPhi` (áp dụng
strategy) — TÁI HIỆN đầy đủ hành vi của bản OOP hierarchy Ở TRÊN.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type CongThucPhi = (khoiLuong: number) => number;

const phiTieuChuan: CongThucPhi = (kl) => 15000 + kl * 2000;
const phiNhanh: CongThucPhi = (kl) => ___;

function tinhTongPhi(khoiLuong: number, congThuc: CongThucPhi): number {
  return ___;
}

assertEqual(tinhTongPhi(4, phiNhanh), 44000, "phi nhanh 4kg");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type CongThucPhi = (khoiLuong: number) => number;

const phiTieuChuan: CongThucPhi = (kl) => 15000 + kl * 2000;
const phiNhanh: CongThucPhi = (kl) => 30000 + kl * 3500;

function tinhTongPhi(khoiLuong: number, congThuc: CongThucPhi): number {
  return congThuc(khoiLuong);
}

assertEqual(tinhTongPhi(4, phiNhanh), 44000, "phi nhanh 4kg");
```

```typescript title=test
assertEqual(tinhTongPhi(4, phiTieuChuan), 23000, "phi tieu chuan 4kg");
assertEqual(tinhTongPhi(0, phiNhanh), 30000, "phi nhanh 0kg -- chi phi co ban");
assertEqual(tinhTongPhi(10, phiTieuChuan), 35000, "phi tieu chuan 10kg");
assertEqual(tinhTongPhi(2, phiNhanh) > tinhTongPhi(2, phiTieuChuan), true, "nhanh luon dat hon tieu chuan");
```

:::hints
- kind: attention
  body: "phiNhanh: CÙNG hình dạng phiTieuChuan (chi phí cơ bản + kl nhân đơn giá), chỉ khác HAI con số (30000/3500). tinhTongPhi: gọi congThuc như một hàm bình thường, truyền khoiLuong."
- kind: strategy
  body: "30000 + kl * 3500 : congThuc(khoiLuong) — công thức cộng chi phí cơ bản với phần theo cân, và gọi tham số hàm."
- kind: one-line
  body: '___ (phiNhanh) = 30000 + kl * 3500\n___ (tinhTongPhi) = congThuc(khoiLuong)'
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
Cụm 1 hoàn tất: bảng ánh xạ OOP→FP, Strategy = hàm bậc cao, ranh giới
khi vẫn cần nhóm hàm (object), refactor hierarchy → hàm. Cụm tiếp
theo: Observer = EventEmitter.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinhTongPhi` nhận MỘT strategy TẠI MỘT thời điểm gọi. Nếu NHIỀU bên
CÙNG muốn "biết" khi MỘT sự kiện xảy ra (không CHỈ một strategy được
CHỌN) — hình dạng nào phù hợp?
::::

::::checkpoint{mastery=0.8}
::::
