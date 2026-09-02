---
id: ky-nghe-phan-mem.kiem-thu.vi-du-vs-tinh-chat
title: "Test VÍ DỤ vs test TÍNH CHẤT — đoán input hay khẳng định bất biến"
summary: "Test ví dụ khẳng định MỘT cặp input→output cụ thể — chỉ tốt bằng sự sáng suốt của người chọn ví dụ. Test tính chất khẳng định MỘT bất biến phải đúng với RẤT NHIỀU input — bắt được edge case không ai nghĩ tới liệt kê thủ công."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 15
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.example-vs-property]
requires: [kt.gate-boss-fp-di]
concepts: [kt.example-vs-property]
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
Cụm mới. Đã test HÀNG CHỤC hàm bằng cách chọn VÍ DỤ cụ thể. Có cách
nào test KHÔNG cần "đoán" ví dụ nào đáng thử không?
::::

::::explain{#vi-du-vs-tinh-chat}
Test **VÍ DỤ** (đã làm XUYÊN SUỐT cụm 1-3): khẳng định MỘT cặp
input→output **CỤ THỂ** — chỉ TỐT bằng SỰ SÁNG SUỐT của người CHỌN ví
dụ. Test **TÍNH CHẤT** (Property-Based Testing): khẳng định MỘT
**BẤT BIẾN** phải ĐÚNG với **RẤT NHIỀU** input — bắt được edge case
KHÔNG AI nghĩ tới liệt kê thủ công:

```typescript
// "sap xep mot luot" -- cai dat TU VIET (bug an, chua biet)
function sapXepMotLuot(mang: number[]): number[] {
  const ket = [...mang];
  for (let i = 0; i < ket.length - 1; i++) {
    const a = ket[i];
    const b = ket[i + 1];
    if (a === undefined || b === undefined) continue;
    if (a > b) { ket[i] = b; ket[i + 1] = a; }
  }
  return ket;
}

// TEST VI DU: hai truong hop "chon tay"
console.log(JSON.stringify(sapXepMotLuot([3, 1, 2])));
console.log(JSON.stringify(sapXepMotLuot([5, 2])));
```

```text
[1,2,3]
[2,5]
```

CẢ HAI ví dụ "TRÔNG ổn" — `sapXepMotLuot` DƯỜNG NHƯ hoạt động ĐÚNG.
NHƯNG hai ví dụ NÀY được **CHỌN TAY**, KHÔNG PHẢI ĐẠI DIỆN cho MỌI khả
năng — chúng CHỈ chứng minh hàm ĐÚNG TRÊN ĐÚNG hai input ĐÓ.
::::

::::example{#tinh-chat-bat-nhieu-mau-hon}
Test **TÍNH CHẤT** "kết quả PHẢI có thứ tự TĂNG DẦN" TRÊN NHIỀU mẫu
HƠN lộ ra ĐIỀU hai ví dụ TRÊN KHÔNG THẤY:

```typescript title=readonly
function laDayTangDan(mang: number[]): boolean {
  for (let i = 0; i < mang.length - 1; i++) {
    const a = mang[i];
    const b = mang[i + 1];
    if (a === undefined || b === undefined) continue;
    if (a > b) return false;
  }
  return true;
}
function sapXepMotLuot(mang: number[]): number[] {
  const ket = [...mang];
  for (let i = 0; i < ket.length - 1; i++) {
    const a = ket[i];
    const b = ket[i + 1];
    if (a === undefined || b === undefined) continue;
    if (a > b) { ket[i] = b; ket[i + 1] = a; }
  }
  return ket;
}

const mauThu: number[][] = [
  [3, 1, 2], [5, 2], [1], [], [2, 2, 2], [9, 5, 7, 3, 1],
  [4, 3, 2, 1], [10, -5, 0, 3], [1, 1, 1, 1, 1], [6, 5, 4, 3, 2, 1],
];

let soLuotThatBai = 0;
let viTriThatBaiDauTien = -1;
for (let i = 0; i < mauThu.length; i++) {
  const m = mauThu[i];
  if (m === undefined) continue;
  if (!laDayTangDan(sapXepMotLuot(m))) {
    soLuotThatBai++;
    if (viTriThatBaiDauTien === -1) viTriThatBaiDauTien = i;
  }
}
console.log("so lan that bai:", soLuotThatBai, "/ vi tri dau tien:", viTriThatBaiDauTien);

const mauLoi = mauThu[viTriThatBaiDauTien] ?? [];
console.log("mau lam lo bug:", JSON.stringify(mauLoi), "->", JSON.stringify(sapXepMotLuot(mauLoi)));
```

```text title=readonly
so lan that bai: 3 / vi tri dau tien: 5
mau lam lo bug: [9,5,7,3,1] -> [5,7,3,1,9]
```

TRÊN **MƯỜI** mẫu (VẪN chưa "ngẫu nhiên" đúng nghĩa, CHỈ LÀ danh sách
LỚN HƠN hai ví dụ TRÊN), tính chất `laDayTangDan` PHÁT HIỆN **BA** ca
`sapXepMotLuot` **THẤT BẠI** — `[9,5,7,3,1]` cho kết quả
`[5,7,3,1,9]`, RÕ RÀNG KHÔNG tăng dần. `sapXepMotLuot` CHỈ đi **MỘT
LƯỢT** so sánh/đổi chỗ CÁC cặp liền kề — ĐỦ để sắp xếp mảng NGẮN,
NHƯNG KHÔNG ĐỦ cho mảng DÀI/rối hơn. Hai ví dụ "chọn tay" ĐẦU (bài
explain) **KHÔNG BAO GIỜ** lộ ra ĐIỀU này, vì CẢ HAI đều NGẮN.
::::

::::predict{#doan-sapxepmotluot-tren-mang-nguoc commitOnce}
```typescript
function laDayTangDan(mang: number[]): boolean {
  for (let i = 0; i < mang.length - 1; i++) {
    const a = mang[i];
    const b = mang[i + 1];
    if (a === undefined || b === undefined) continue;
    if (a > b) return false;
  }
  return true;
}
function sapXepMotLuot(mang: number[]): number[] {
  const ket = [...mang];
  for (let i = 0; i < ket.length - 1; i++) {
    const a = ket[i];
    const b = ket[i + 1];
    if (a === undefined || b === undefined) continue;
    if (a > b) { ket[i] = b; ket[i + 1] = a; }
  }
  return ket;
}

const ketQua = sapXepMotLuot([4, 3, 2, 1]);
console.log(JSON.stringify(ketQua), laDayTangDan(ketQua));
```

Dòng cuối in ra gì?

:::opt{correct}
`[3,2,1,4] false`
:::

:::opt
`[1,2,3,4] true` — vì `sapXepMotLuot` LẶP qua TOÀN BỘ mảng, so sánh
VÀ đổi chỗ MỌI cặp liền kề "không đúng thứ tự" — với MỘT mảng ĐẢO
NGƯỢC HOÀN TOÀN như `[4,3,2,1]`, VÒNG LẶP MỘT LƯỢT ĐỦ để đưa nó VỀ
thứ tự tăng dần
::why
Gần đúng ở việc bạn nhớ ĐÚNG `sapXepMotLuot` CÓ đi qua TỪNG cặp liền
kề, đổi chỗ khi SAI thứ tự — một quan sát ĐÚNG về Ý ĐỒ của thuật
toán.

Chỗ lệch: "MỘT LƯỢT" (single pass) CHỈ đủ để MỘT phần tử SAI vị trí
"nổi" LÊN (hoặc XUỐNG) **ĐÚNG MỘT bước** MỖI lượt — KHÔNG đủ để SỬA
TOÀN BỘ một mảng ĐẢO NGƯỢC (cần NHIỀU lượt lặp lại, giống bubble sort
THẬT). Trace TAY: `i=0`: `4>3` → đổi → `[3,4,2,1]`; `i=1`: `4>2` →
đổi → `[3,2,4,1]`; `i=2`: `4>1` → đổi → `[3,2,1,4]`. Kết quả CUỐI
`[3,2,1,4]` — số `4` ĐÃ "trôi" ĐÚNG tới cuối (đúng ĐÍCH), NHƯNG PHẦN
CÒN LẠI (`3,2,1`) VẪN Y NGUYÊN thứ tự GIẢM DẦN gốc — `laDayTangDan`
trả `false`.
::
:::

:::opt
Máy báo lỗi biên dịch — `laDayTangDan(ketQua)` không hợp lệ, vì
`ketQua` (kết quả của `sapXepMotLuot`) có kiểu SUY RA LÀ `(number |
undefined)[]` (do `noUncheckedIndexedAccess`), KHÔNG khớp tham số
`mang: number[]` mà `laDayTangDan` đòi hỏi
::why
Gần đúng ở việc bạn nhớ ĐÚNG `noUncheckedIndexedAccess` khiến TRUY
CẬP `mang[i]` BÊN TRONG một hàm trả VỀ `number | undefined` (đã gặp
NHIỀU lần trong track NÀY) — một quan sát ĐÚNG về QUY TẮC chung.

Chỗ lệch: quy tắc ĐÓ áp dụng cho **TRUY CẬP INDEX** (`mang[i]`), KHÔNG
áp dụng cho **KIỂU TRẢ VỀ** của cả hàm — `sapXepMotLuot` khai RÕ kiểu
trả `: number[]` (MỘT mảng HOÀN CHỈNH, KHÔNG PHẢI phần tử ĐƠN LẺ), và
`ket` bên trong LÀ `number[]` TỪ ĐẦU (khởi tạo BẰNG `[...mang]`, sao
chép TỪ MỘT `number[]` khác) — gán TỪNG phần tử (`ket[i] = b`) VẪN
GIỮ kiểu mảng LÀ `number[]`. `ketQua` CÓ kiểu `number[]`, khớp HOÀN
TOÀN tham số của `laDayTangDan`. Biên dịch sạch.
::
:::
::::

::::code{#viet_ladaytangdan}
Tự viết `laDayTangDan` — hàm kiểm TÍNH CHẤT "có thứ tự tăng dần".

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function laDayTangDan(mang: number[]): boolean {
  for (let i = 0; i < mang.length - 1; i++) {
    const a = mang[i];
    const b = mang[i + 1];
    if (a === undefined || b === undefined) continue;
    if (___) return ___;
  }
  return true;
}

assertEqual(laDayTangDan([1, 2, 3]), true, "day tang dan thuc su");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function laDayTangDan(mang: number[]): boolean {
  for (let i = 0; i < mang.length - 1; i++) {
    const a = mang[i];
    const b = mang[i + 1];
    if (a === undefined || b === undefined) continue;
    if (a > b) return false;
  }
  return true;
}

assertEqual(laDayTangDan([1, 2, 3]), true, "day tang dan thuc su");
```

```typescript title=test
assertEqual(laDayTangDan([]), true, "mang rong la day tang dan (khong co gi vi pham)");
assertEqual(laDayTangDan([5]), true, "mang mot phan tu la day tang dan");
assertEqual(laDayTangDan([2, 2, 2]), true, "day bang nhau (khong giam) van la tang dan khong giam");
assertEqual(laDayTangDan([3, 1, 2]), false, "day KHONG tang dan phai tra ve false");
assertEqual(laDayTangDan([9, 5, 7, 3, 1]), false, "day giam dan roi tang lai van la KHONG tang dan");
```

:::hints
- kind: attention
  body: "Điều kiện: cặp liền kề SAI thứ tự khi phần tử TRƯỚC lớn hơn phần tử SAU. Khi phát hiện MỘT cặp sai, dừng ngay và báo KHÔNG phải dãy tăng dần."
- kind: strategy
  body: 'a > b : false — điều kiện phát hiện vi phạm, giá trị trả về khi phát hiện.'
- kind: one-line
  body: '___ (điều kiện) = a > b\n___ (giá trị trả) = false'
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
Test tính chất bắt được edge case test ví dụ bỏ sót. Bước tiếp theo:
tự viết vòng lặp "sinh rồi kiểm" — cốt lõi của MỌI công cụ PBT.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm 1-10 mẫu (chọn TAY) ĐÃ bắt được một bug. Nếu SINH input **NGẪU
NHIÊN** thay vì chọn tay, VÀ lặp lại quy trình NHIỀU lần — vòng lặp
ĐÓ trông như thế nào?
::::

::::checkpoint{mastery=0.8}
::::
