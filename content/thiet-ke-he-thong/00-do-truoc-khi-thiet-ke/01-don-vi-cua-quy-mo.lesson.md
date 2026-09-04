---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.don-vi-cua-quy-mo
title: "Đơn vị của quy mô — luỹ thừa của 2"
summary: "soByte(soLuong, donVi) quy đổi KB/MB/GB/TB về byte bằng luỹ thừa của 2 (1 KB=2^10=1024 byte, 1 MB=2^20=1.048.576 byte, 1 GB=2^30=1.073.741.824 byte, 1 TB=2^40 byte) -- KHÔNG phải bội số của 1000. tongDungLuongGB(soPhanTu, kichThuocMoiPhanTuKB) tính tổng dung lượng rồi làm tròn 2 chữ số thập phân: app ghi âm giả định GhiAmNhanh với 900.000 bản ghi x 240KB/bản ra đúng 205.99 GB -- không phải 216 GB nếu lỡ quy đổi theo bội số 1000."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.don-vi-cua-quy-mo]
requires: []
concepts: [sd.don-vi-cua-quy-mo]
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
ByteLedger (q21) khép lại Realm 6 — bạn vừa xây một cơ sở dữ liệu từ
SỐ 0. Đổi vai: từ giờ KHÔNG xây database nữa, mà TRẢ LỜI câu hỏi "hệ
thống này chịu được BAO NHIÊU tải?" — TRƯỚC khi đặt viên gạch đầu
tiên. Câu hỏi đầu tiên của MỌI ước lượng: một byte LÀ bao nhiêu byte?
::::

::::explain{#luy-thua-cua-2}
Bộ nhớ máy tính đếm theo luỹ thừa CỦA 2, không phải bội số CỦA 1000
— `1 KB = 2^10 byte = 1024 byte`, `1 MB = 2^20 byte = 1.048.576
byte`, `1 GB = 2^30 byte`, `1 TB = 2^40 byte`. `soByte` quy đổi MỘT
số lượng ở một đơn vị VỀ đúng số byte:

```typescript title=readonly
const KB = 2 ** 10;
const MB = 2 ** 20;
const GB = 2 ** 30;
const TB = 2 ** 40;

function soByte(soLuong: number, donVi: 'B' | 'KB' | 'MB' | 'GB' | 'TB'): number {
  const heSo = donVi === 'B' ? 1 : donVi === 'KB' ? KB : donVi === 'MB' ? MB : donVi === 'GB' ? GB : TB;
  return soLuong * heSo;
}

console.log("1 KB =", soByte(1, 'KB'), "byte");
console.log("1 MB =", soByte(1, 'MB'), "byte");
console.log("1 GB =", soByte(1, 'GB'), "byte");
console.log("500 MB =", soByte(500, 'MB'), "byte");
```

```text title=readonly
1 KB = 1024 byte
1 MB = 1048576 byte
1 GB = 1073741824 byte
500 MB = 524288000 byte
```

`1 MB` KHÔNG phải `1.000.000 byte` — nó LÀ `1024 × 1024 = 1.048.576
byte`, nhiều hơn một triệu đúng `48.576` byte. Chênh lệch NÀY nhỏ ở
quy mô MB, nhưng CÀNG lên GB/TB thì phần "dư" so VỚI bội số 1000 CÀNG
lớn — ước lượng dùng SAI công thức (nhân 1000 thay VÌ 1024) sẽ lệch
tích luỹ qua NHIỀU lần nhân.
::::

::::example{#tinh-tong-dung-luong}
Ứng dụng giả định "GhiAmNhanh" — ghi chú thoại ngắn, mỗi bản ghi nén
xuống còn `240 KB`. Nếu người dùng tạo ra `900.000` bản ghi (một
tháng hoạt động), tổng dung lượng LÀ:

```typescript title=readonly
const KB = 2 ** 10;
const GB = 2 ** 30;

function tongDungLuongGB(soPhanTu: number, kichThuocMoiPhanTuKB: number): number {
  const tongByte = soPhanTu * kichThuocMoiPhanTuKB * KB;
  return Math.round((tongByte / GB) * 100) / 100;
}

console.log("GhiAmNhanh -- 900.000 ban ghi x 240KB:", tongDungLuongGB(900000, 240), "GB");
```

```text title=readonly
GhiAmNhanh -- 900.000 ban ghi x 240KB: 205.99 GB
```

`900.000 × 240 = 216.000.000 KB` — nghe "gần 216 GB" nếu tính nhẩm
theo bội số `1000`, nhưng chia đúng CHO `2^30` (không phải `10^9`)
cho ra `205.99 GB` — Ít hơn con số nhẩm nhanh đúng gần `5%`. Sai số
NÀY LÀ lý do MỌI ước lượng quy mô phải dùng đúng luỹ thừa của 2, chứ
không làm tròn tuỳ tiện.
::::

::::predict{#doan-gap-doi-so-luong commitOnce}
`tongDungLuongGB(900000, 240)` LÀ `205.99` GB (readonly Ở trên). Nếu
số PHẦN tử tăng gấp ĐÔI (`1.800.000` bản ghi) NHƯNG kích thước mỗi
phần tử GIỮ nguyên (`240 KB`), tổng dung lượng (GB) thay ĐỔI thế nào?

:::opt{correct}
Gấp ĐÔI (`411.99` GB) — công thức nhân TUYẾN tính theo `soPhanTu`,
số lượng gấp đôi thì tổng byte cũng gấp đôi
:::
:::opt
KHÔNG đổi — `GB` LÀ một đơn vị CỐ định, tăng số lượng bản ghi không
LÀM đơn vị đo lường thay đổi
::why
Nhầm "đơn vị đo" (GB LÀ hằng số quy đổi, ĐÚNG — cố định) VỚI "kết quả
đo" (tổng dung lượng TÍNH bằng GB, phụ thuộc trực tiếp và SỐ lượng).

Chỗ lệch: `tongDungLuongGB` nhân `soPhanTu × kichThuocMoiPhanTuKB ×
KB` — TUYẾN tính theo `soPhanTu`. Gấp đôi số phần TỬ (giữ nguyên kích
thước MỖI phần tử) làm tổng byte gấp đôi, VÀ do ĐÓ số GB kết quả cũng
gấp đôi — không CÓ gì "cố định" Ở kết quả cả, chỉ CÓ hệ số quy đổi
(`2^30`) LÀ cố định.
::
:::
::::

::::code{#viet_tong_dung_luong_gb}
Hoàn thiện `tongDungLuongGB` — tính tổng số byte (`soPhanTu ×
kichThuocMoiPhanTuKB × KB`), quy đổi VỀ GB (chia cho `GB = 2^30`),
rồi làm tròn 2 chữ số thập phân.

```typescript title=starter
const KB = 2 ** 10;
const MB = 2 ** 20;
const GB = 2 ** 30;
const TB = 2 ** 40;

function tongDungLuongGB(soPhanTu: number, kichThuocMoiPhanTuKB: number): number {
  const tongByte = soPhanTu * kichThuocMoiPhanTuKB * KB;
  return ___;
}

console.log(tongDungLuongGB(900000, 240));
```

```typescript title=solution
const KB = 2 ** 10;
const MB = 2 ** 20;
const GB = 2 ** 30;
const TB = 2 ** 40;

function tongDungLuongGB(soPhanTu: number, kichThuocMoiPhanTuKB: number): number {
  const tongByte = soPhanTu * kichThuocMoiPhanTuKB * KB;
  return Math.round((tongByte / GB) * 100) / 100;
}

console.log(tongDungLuongGB(900000, 240));
```

```typescript title=test
if (tongDungLuongGB(1048576, 1) !== 1) throw new Error("1.048.576 phan tu x 1KB phai dung bang 1 GB (bien gioi chinh xac)");
if (tongDungLuongGB(5000, 100) !== 0.48) throw new Error("5000 x 100KB phai lam tron thanh 0.48 GB (gia tri tho la 0.4768...)");
if (tongDungLuongGB(1000, 1) !== 0) throw new Error("1000 phan tu x 1KB (~0.00095 GB) phai lam tron xuong 0");
if (tongDungLuongGB(900000, 240) !== 205.99) throw new Error("GhiAmNhanh: 900.000 ban ghi x 240KB phai la 205.99 GB");
if (tongDungLuongGB(1800000, 240) !== 411.99) throw new Error("gap doi so phan tu phai cho tong gap doi (411.99 GB)");
```

:::hints
- kind: attention
  body: "Chia tongByte cho GB roi lam tron 2 chu so thap phan -- mot dong."
- kind: strategy
  body: "Math.round((tongByte / GB) * 100) / 100"
- kind: one-line
  body: "return Math.round((tongByte / GB) * 100) / 100;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "205.99"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Dung lượng đã có đơn vị đúng. Nhưng "lưu được bao nhiêu" chỉ LÀ một
nửa câu chuyện — "TRUY cập nhanh cỡ nào" LÀ nửa còn lại.
::::

::::reflect{#nghi-lai}
`tongDungLuongGB` chỉ LÀ một phép nhân VÀ một phép chia — nhưng chọn
ĐÚNG hệ số (`2^30` chứ không phải `10^9`) LÀ ranh giới GIỮA một ước
lượng ĐÁNG tin và một con số nhẩm sai LỆCH gần `5%`. Mọi bài SAU của
quest NÀY đều dùng lại đúng bộ đơn vị `KB/MB/GB/TB` NÀY — không định
nghĩa LẠI.
::::

::::checkpoint{mastery=0.75}
::::
