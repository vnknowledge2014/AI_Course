---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.qps-va-luu-tru
title: "QPS và lưu trữ — ráp một ước lượng end-to-end"
summary: "App giả định PhotoDrop (chia sẻ ảnh nhóm nhỏ): 2.000.000 MAU, 40% DAU (800.000), mỗi DAU đăng 3 ảnh 2MB/ngày -- 2.400.000 ảnh/ngày. qpsTrungBinh=27.78, qpsDinh (hệ số đỉnh x4, giờ cao điểm buổi tối)=111.11. luuTruGbMoiNam(2.400.000, 2, 1)=1.712.109,38 GB/năm; 3 năm=5.136.328,13 GB (~5.015,95 TB) -- tăng hệ số đỉnh từ x4 lên x8 làm QPS đỉnh tăng ĐÚNG gấp đôi (tuyến tính), không ảnh hưởng QPS trung bình hay lưu trữ."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.qps-va-luu-tru]
requires: [sd.chin-so-9]
concepts: [sd.qps-va-luu-tru]
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
Đơn vị (bài 1), độ trễ (bài 2), availability (bài 3) — ba mảnh RIÊNG
lẻ. Giờ ráp CẢ ba VÀO một bài toán THẬT: giả sử bạn phải thiết kế MỘT
dịch vụ — nó cần bao nhiêu QPS, VÀ bao nhiêu ổ cứng?
::::

::::explain{#gia-dinh-va-qps}
Ứng dụng giả định "PhotoDrop" — chia sẻ ảnh trong nhóm nhỏ (bạn bè,
gia đình). Giả định: `2.000.000` người dùng hoạt động HÀNG tháng
(MAU), `40%` trong SỐ đó hoạt động MỖI ngày (DAU), mỗi DAU đăng
trung bình `3` ảnh/ngày. Từ đó suy RA số ảnh mỗi ngày, RỒI QPS trung
bình (chia đều CHO `86400` giây MỘT ngày):

```typescript title=readonly
function qpsTrungBinh(soThaoTacMoiNgay: number): number {
  return soThaoTacMoiNgay / 86400;
}

function lamTron(x: number, chuSo: number): number {
  const heSo = 10 ** chuSo;
  return Math.round(x * heSo) / heSo;
}

const MAU = 2_000_000;
const DAU = MAU * 0.4;
const anhMoiNgay = DAU * 3;
console.log("DAU:", DAU);
console.log("anh moi ngay:", anhMoiNgay);
console.log("QPS trung binh:", lamTron(qpsTrungBinh(anhMoiNgay), 2));
```

```text title=readonly
DAU: 800000
anh moi ngay: 2400000
QPS trung binh: 27.78
```

`800.000` người dùng đăng `3` ảnh MỖI ngày ra `2.400.000` thao tác
ghi MỖI ngày — trải ĐỀU ra `86400` giây thì trung bình chỉ `27.78`
request/giây. Con số NHỎ đến bất NGỜ so VỚI "hai triệu người dùng" —
Vì traffic trải ĐỀU suốt `24` giờ, không dồn VÀO một khoảnh khắc.
::::

::::example{#dinh-va-luu-tru}
Traffic THẬT không đều — buổi tối (giờ nghỉ) thường ĐÔNG hơn hẳn ban
đêm. Giả định hệ số ĐỈNH `×4` (giờ cao điểm gấp `4` lần trung bình).
Song song, MỖI ảnh nén xuống còn `2MB`, LƯU trong `3` năm:

```typescript title=readonly
function qpsTrungBinh(soThaoTacMoiNgay: number): number {
  return soThaoTacMoiNgay / 86400;
}
function lamTron(x: number, chuSo: number): number {
  const heSo = 10 ** chuSo;
  return Math.round(x * heSo) / heSo;
}
function qpsDinh(qpsTB: number, heSoDinh: number): number {
  return qpsTB * heSoDinh;
}

function luuTruGbMoiNam(soThaoTacMoiNgay: number, kichThuocMoiThaoTacMB: number, soNam: number): number {
  const tongMB = soThaoTacMoiNgay * kichThuocMoiThaoTacMB * 365.25 * soNam;
  return Math.round((tongMB / 1024) * 100) / 100;
}

const qtb = qpsTrungBinh(2_400_000);
console.log("QPS dinh (x4):", lamTron(qpsDinh(qtb, 4), 2));
console.log("luu tru 3 nam (GB):", luuTruGbMoiNam(2_400_000, 2, 3));
console.log("luu tru 3 nam (TB):", lamTron(luuTruGbMoiNam(2_400_000, 2, 3) / 1024, 2));
```

```text title=readonly
QPS dinh (x4): 111.11
luu tru 3 nam (GB): 5136328.13
luu tru 3 nam (TB): 5015.95
```

`QPS đỉnh` (`111.11`) hơn `4` lần `QPS trung bình` (`27.78`) — hệ
thống PHẢI chịu được `111` request/giây TẠI đỉnh, dù trung bình cả
ngày chỉ CÓ `28`. Thiết kế THEO số trung bình LÀ một lỗi kinh điển —
hệ thống sập đúng lúc TẢI cao nhất. VÀ `3` năm ảnh cộng dồn TỚI hơn
`5.000 TB` (`~5 PB`) — một con số CHỈ lộ ra khi nhân đúng bốn đại
lượng: số người dùng, tần suất, kích thước, VÀ thời gian lưu trữ.
::::

::::predict{#doan-tang-he-so-dinh commitOnce}
`qpsDinh(qtb, 4)` LÀ `111.11`. Nếu hệ số ĐỈNH tăng TỪ `4` lên `8`
(giờ cao điểm CÀNG dồn hơn), NHƯNG `qtb` (QPS trung bình) GIỮ nguyên
— `qpsDinh` mới LÀ bao nhiêu?

:::opt{correct}
`222.22` — gấp ĐÔI `111.11`, vì `qpsDinh` nhân TUYẾN tính theo hệ số
đỉnh, không phụ thuộc gì khác
:::
:::opt
Vẫn LÀ `111.11` — hệ số đỉnh chỉ LÀ một con số "tham khảo", không
trực tiếp NHÂN vào QPS
::why
Nhầm hệ số đỉnh LÀ một NHÃN mô tả ("có đỉnh" hay "không"), thay VÌ
một thừa SỐ trong phép nhân.

Chỗ lệch: `qpsDinh(qpsTB, heSoDinh)` trả VỀ đúng `qpsTB × heSoDinh` —
KHÔNG có bước nào khác. Tăng hệ SỐ đỉnh từ `4` lên `8` (giữ nguyên
`qtb`) làm kết quả nhân ĐÔI, y hệt CÁCH tăng gấp đôi bất kỳ thừa số
nào TRONG một phép nhân.
::
:::
::::

::::code{#viet_luu_tru_gb_moi_nam}
Hoàn thiện `luuTruGbMoiNam` — tính tổng dung lượng (MB) rồi quy đổi
VỀ GB (chia `1024`, VÌ `1 GB = 1024 MB`), làm tròn 2 chữ số.

```typescript title=starter
function qpsTrungBinh(soThaoTacMoiNgay: number): number {
  return soThaoTacMoiNgay / 86400;
}

function qpsDinh(qpsTB: number, heSoDinh: number): number {
  return qpsTB * heSoDinh;
}

function luuTruGbMoiNam(soThaoTacMoiNgay: number, kichThuocMoiThaoTacMB: number, soNam: number): number {
  const tongMB = soThaoTacMoiNgay * kichThuocMoiThaoTacMB * 365.25 * soNam;
  return ___;
}

console.log(luuTruGbMoiNam(2_400_000, 2, 1));
```

```typescript title=solution
function qpsTrungBinh(soThaoTacMoiNgay: number): number {
  return soThaoTacMoiNgay / 86400;
}

function qpsDinh(qpsTB: number, heSoDinh: number): number {
  return qpsTB * heSoDinh;
}

function luuTruGbMoiNam(soThaoTacMoiNgay: number, kichThuocMoiThaoTacMB: number, soNam: number): number {
  const tongMB = soThaoTacMoiNgay * kichThuocMoiThaoTacMB * 365.25 * soNam;
  return Math.round((tongMB / 1024) * 100) / 100;
}

console.log(luuTruGbMoiNam(2_400_000, 2, 1));
```

```typescript title=test
function lamTron(x: number, chuSo: number): number {
  const heSo = 10 ** chuSo;
  return Math.round(x * heSo) / heSo;
}

if (lamTron(qpsTrungBinh(2_400_000), 2) !== 27.78) throw new Error("QPS trung binh cua 2.400.000 thao tac/ngay phai la 27.78");
if (lamTron(qpsDinh(qpsTrungBinh(2_400_000), 4), 2) !== 111.11) throw new Error("QPS dinh (x4) phai la 111.11");
if (lamTron(qpsDinh(qpsTrungBinh(2_400_000), 8), 2) !== 222.22) throw new Error("QPS dinh (x8) phai gap doi QPS dinh (x4)");
if (luuTruGbMoiNam(2_400_000, 2, 1) !== 1712109.38) throw new Error("luu tru 1 nam phai la 1.712.109,38 GB");
if (luuTruGbMoiNam(2_400_000, 2, 3) !== 5136328.13) throw new Error("luu tru 3 nam phai la 5.136.328,13 GB");
if (luuTruGbMoiNam(0, 2, 1) !== 0) throw new Error("0 thao tac/ngay phai cho luu tru 0");
```

:::hints
- kind: attention
  body: "Chia tongMB cho 1024 (MB->GB) roi lam tron 2 chu so -- mot dong."
- kind: strategy
  body: "Math.round((tongMB / 1024) * 100) / 100"
- kind: one-line
  body: "return Math.round((tongMB / 1024) * 100) / 100;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1712109.38"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ước lượng xong — bạn biết hệ thống cần chịu BAO nhiêu tải. Nhưng
"chịu tải" nghĩa LÀ có nhiều hơn MỘT máy chủ. Request MỚI tới, ai xử
lý nó?
::::

::::reflect{#nghi-lai}
Bốn hàm nhỏ (`qpsTrungBinh`, `qpsDinh`, `luuTruGbMoiNam`, VÀ phép
nhân giả định ban đầu) — không CÓ gì phức tạp về mặt TOÁN học. Cái
khó THẬT sự LÀ chọn giả định HỢP lý (`40%` DAU, hệ số đỉnh `×4`,
`2MB`/ảnh) VÀ ghi RÕ chúng RA — một ước lượng "đúng công thức nhưng
giả định VÔ căn cứ" cũng vô dụng NHƯ không ước lượng gì cả.
::::

::::checkpoint{mastery=0.8}
::::
