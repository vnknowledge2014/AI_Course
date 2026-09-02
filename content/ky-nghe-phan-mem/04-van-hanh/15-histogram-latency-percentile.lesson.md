---
id: ky-nghe-phan-mem.van-hanh.histogram-latency-percentile
title: "Percentile — request CHẬM NHẤT nói lên điều trung bình GIẤU đi"
summary: "tinhPercentile(dsThoiGian, p) — TỰ tay tính percentile (KHÔNG dùng thư viện thống kê): sao chép mảng (spread) rồi sort BẰNG comparator số (a,b)=>a-b — .sort() KHÔNG comparator so sánh THEO CHUỖI, phá vỡ thứ tự SỐ một cách ÂM THẦM. p90 lộ ra request chậm mà TRUNG BÌNH che giấu."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 15
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [vh.latency-percentile]
requires: [vh.error-rate-threshold]
concepts: [vh.latency-percentile]
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
`vuotNguong` (bài 14) trả lời "CÓ vượt hay KHÔNG" bằng TỶ LỆ. NHƯNG
"request CHẬM NHẤT mất bao lâu" — MỘT con số TRUNG BÌNH CÓ đủ trả
lời KHÔNG?
::::

::::explain{#percentile-la-gi}
**Percentile** trả lời câu hỏi KHÁC hẳn TRUNG BÌNH: "p90 = 300ms"
nghĩa LÀ 90% request NHANH HƠN (HOẶC BẰNG) 300ms — CÒN 10% request
CHẬM HƠN. Tự TAY tính (KHÔNG dùng thư viện thống kê nào):

```typescript title=readonly
function tinhPercentile(dsThoiGian: number[], p: number): number {
  const daSapXep = [...dsThoiGian].sort((a, b) => a - b);
  const n = daSapXep.length;
  const chiSo = Math.min(n - 1, Math.max(0, Math.ceil((p / 100) * n) - 1));
  return daSapXep[chiSo]!;
}

const dsThoiGian = [200, 50, 900, 120, 300, 80, 450, 100, 600, 150];
console.log(tinhPercentile(dsThoiGian, 50));
console.log(tinhPercentile(dsThoiGian, 90));
```

```text title=readonly
150
600
```

Ba BƯỚC: (1) `[...dsThoiGian]` sao chép mảng (KHÔNG đụng bản gốc),
(2) `.sort((a, b) => a - b)` sắp XẾP TĂNG DẦN theo **GIÁ TRỊ SỐ**
(comparator BẮT BUỘC — sẽ THẤY tại SAO Ở phần dự đoán), (3) chọn
đúng VỊ TRÍ tương ứng `p`. Mười request: p50 (median) LÀ `150`ms —
CÒN p90 LÀ `600`ms, GẤP BỐN lần median! Trung bình đơn thuần SẼ
GIẤU mất sự CHÊNH LỆCH này.
::::

::::example{#mang-goc-khong-doi}
`tinhPercentile` KHÔNG làm THAY ĐỔI mảng GỐC — NHỜ bước sao CHÉP
`[...dsThoiGian]` TRƯỚC KHI sắp xếp:

```typescript title=readonly
function tinhPercentile(dsThoiGian: number[], p: number): number {
  const daSapXep = [...dsThoiGian].sort((a, b) => a - b);
  const n = daSapXep.length;
  const chiSo = Math.min(n - 1, Math.max(0, Math.ceil((p / 100) * n) - 1));
  return daSapXep[chiSo]!;
}
const dsThoiGian = [200, 50, 900, 120, 300, 80, 450, 100, 600, 150];
tinhPercentile(dsThoiGian, 50);
console.log(dsThoiGian);
```

```text title=readonly
[200,50,900,120,300,80,450,100,600,150]
```

DÙ ĐÃ gọi `tinhPercentile` (sắp xếp BÊN TRONG), `dsThoiGian` BÊN
NGOÀI VẪN giữ NGUYÊN thứ tự BAN ĐẦU — `Array.prototype.sort` SẼ sửa
TRỰC TIẾP mảng ĐƯỢC gọi TRÊN nó, NHƯNG `[...dsThoiGian]` tạo MỘT
mảng MỚI HOÀN TOÀN trước, nên bản GỐC KHÔNG hề bị chạm tới.
::::

::::predict{#doan-sort-khong-comparator commitOnce}
```typescript
function tinhPercentileSai(ds: number[], p: number): number {
  const daSapXep = [...ds].sort();
  const n = daSapXep.length;
  const chiSo = Math.min(n - 1, Math.max(0, Math.ceil((p / 100) * n) - 1));
  return daSapXep[chiSo]!;
}
console.log(tinhPercentileSai([3, 20, 100], 90));
```

`tinhPercentileSai` gọi `.sort()` **KHÔNG comparator**. `p90` của
BA giá trị `[3, 20, 100]` (số LỚN NHẤT LÀ `100`). Dòng cuối in ra
gì?

:::opt{correct}
`3`
:::

:::opt
`100` — vì mảng `[3, 20, 100]` TOÀN kiểu `number`, VÀ `.sort()`
MẶC ĐỊNH tự động NHẬN RA đó LÀ mảng số RỒI so sánh THEO GIÁ TRỊ (như
`tinhPercentile` ĐÚNG Ở phần explain), NÊN p90 VẪN LÀ số lớn nhất
`100`
::why
Gần đúng ở việc bạn nhớ ĐÚNG p90 CỦA `[3, 20, 100]` (VỀ mặt Ý NGHĨA
"90% nhanh hơn giá trị NÀY") NÊN phải LÀ `100`, số LỚN nhất — MỘT
kỳ vọng hợp LÝ nếu `.sort()` hoạt động ĐÚNG.

Chỗ lệch: `Array.prototype.sort()` **KHÔNG tham số** KHÔNG hề biết
mảng CHỨA kiểu `number` — nó LUÔN CHUYỂN từng phần tử THÀNH `string`
RỒI so sánh THEO TỪNG KÝ TỰ (UTF-16). `"3"`, `"20"`, `"100"` so
sánh KÝ tự ĐẦU: `'1' < '2' < '3'` → thứ tự "tăng dần" (SAI, kiểu
CHUỖI) LÀ `["100", "20", "3"]`. `chiSo` cho `p=90` VỚI `n=3` LÀ `2`
(VỊ trí CUỐI) — Ở mảng ĐÃ sắp SAI đó, VỊ trí cuối LÀ `3` (SỐ NHỎ
NHẤT!). Kết quả: hệ thống báo "request chậm nhất CHỈ 3ms" TRONG khi
THẬT SỰ CÓ request mất TỚI `100`ms — ÂM THẦM SAI, KHÔNG lỗi runtime
nào cả. Đây chính LÀ lý DO comparator `(a, b) => a - b` BẮT BUỘC
PHẢI CÓ khi sắp xếp SỐ.
::
:::

:::opt
Máy báo lỗi biên dịch — `.sort()` gọi TRÊN `number[]` mà KHÔNG
truyền comparator LÀ vi phạm kiểu, TypeScript YÊU CẦU BẮT BUỘC phải
CÓ đối SỐ so sánh cho mảng SỐ
::why
Gần đúng ở việc bạn để ý `.sort()` Ở ĐÂY THIẾU comparator — một
quan sát ĐÚNG về CÚ PHÁP.

Chỗ lệch: `Array.prototype.sort` khai overload `sort(compareFn?:
...)` — dấu `?` nghĩa LÀ đối số NÀY **TUỲ CHỌN**, hợp lệ VỚI MỌI
kiểu phần TỬ (`string[]`, `number[]`, BẤT KỲ `T[]`). Biên dịch SẠCH
— hành vi SAI CHỈ xảy RA lúc CHẠY, KHÔNG PHẢI lúc biên dịch.
::
:::
::::

::::code{#viet_tinh_percentile}
Hoàn thiện `tinhPercentile` — sắp xếp MẢNG SAO CHÉP theo comparator
SỐ, rồi chọn ĐÚNG vị trí.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function tinhPercentile(dsThoiGian: number[], p: number): number {
  const daSapXep = [...dsThoiGian].sort(___);
  const n = daSapXep.length;
  const chiSo = Math.min(n - 1, Math.max(0, Math.ceil((p / 100) * n) - 1));
  return daSapXep[chiSo]!;
}

assertEqual(tinhPercentile([100, 200, 300, 400, 500], 50), 300, "median cua 5 phan tu");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function tinhPercentile(dsThoiGian: number[], p: number): number {
  const daSapXep = [...dsThoiGian].sort((a, b) => a - b);
  const n = daSapXep.length;
  const chiSo = Math.min(n - 1, Math.max(0, Math.ceil((p / 100) * n) - 1));
  return daSapXep[chiSo]!;
}

assertEqual(tinhPercentile([100, 200, 300, 400, 500], 50), 300, "median cua 5 phan tu");
```

```typescript title=test
assertEqual(tinhPercentile([100, 200, 300, 400, 500], 90), 500, "p90 cua 5 phan tu la max");
assertEqual(tinhPercentile([50, 80, 100, 120, 150, 200, 300, 450, 600, 900], 50), 150, "median 10 phan tu");
assertEqual(tinhPercentile([50, 80, 100, 120, 150, 200, 300, 450, 600, 900], 90), 600, "p90 cua 10 phan tu");
assertEqual(tinhPercentile([200, 50, 900, 120, 300, 80, 450, 100, 600, 150], 50), 150, "median tren mang CHUA sap xep");

const goc = [3, 1, 2];
tinhPercentile(goc, 50);
assertEqual(goc[0], 3, "mang goc khong bi xao tron thu tu");
assertEqual(goc[1], 1, "mang goc khong bi xao tron thu tu 2");
```

:::hints
- kind: attention
  body: "sort() can comparator so sanh SO, khong phai chuoi: (a, b) => a - b."
- kind: strategy
  body: "(a, b) => a - b"
- kind: one-line
  body: '___ = (a, b) => a - b'
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
Percentile lộ ra request chậm mà trung bình che giấu. Bài chốt cụm:
ghép counter + tỷ lệ lỗi + percentile thành MỘT bảng theo dõi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`taoCounter` (bài 13), `vuotNguong` (bài 14), VÀ `tinhPercentile`
(bài NÀY) hiện LÀ BA công cụ RIÊNG. Ghép CẢ BA thành MỘT "bảng điều
khiển" (dashboard) DUY NHẤT trông NHƯ thế nào?
::::

::::checkpoint{mastery=0.8}
::::
