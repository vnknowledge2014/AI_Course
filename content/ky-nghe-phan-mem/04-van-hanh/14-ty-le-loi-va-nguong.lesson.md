---
id: ky-nghe-phan-mem.van-hanh.ty-le-loi-va-nguong
title: "Tỷ lệ lỗi & ngưỡng cảnh báo — từ HAI counter ra MỘT quyết định"
summary: "tinhTyLeLoi(soLoi, soTong) = soLoi/soTong — NHƯNG PHẢI xử lý soTong=0 (KHÔNG traffic) TRƯỚC khi chia, nếu KHÔNG JavaScript trả về NaN (không lỗi runtime, chỉ ÂM THẦM sai). vuotNguong(soLoi, soTong, nguong): boolean quyết định CÓ cảnh báo hay KHÔNG dựa trên tỷ lệ TÍNH được."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.error-rate-threshold]
requires: [vh.metrics-counter]
concepts: [vh.error-rate-threshold]
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
`requestCounter.giaTri()` LÀ `100`, `errorCounter.giaTri()` LÀ `3`.
"Hệ thống ĐANG lỗi NHIỀU?" — HAI con số RIÊNG LẺ CHƯA trả lời được.
::::

::::explain{#ty-le-loi}
GHÉP hai counter THÀNH MỘT **tỷ lệ lỗi**: `soLoi / soTong`. NHƯNG
KHI `soTong` LÀ `0` (CHƯA CÓ request NÀO), PHẢI xử lý **TRƯỚC KHI
chia** — nếu KHÔNG, JavaScript trả VỀ `NaN` (KHÔNG BÁO LỖI runtime,
CHỈ ÂM THẦM sai):

```typescript title=readonly
function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}

console.log(tinhTyLeLoi(3, 100));
console.log(tinhTyLeLoi(0, 0));
```

```text title=readonly
0.03
0
```

`3/100` LÀ `0.03` (3%) — bình THƯỜNG. `0/0` (KHÔNG request nào TỪ
lúc khởi động, cả `soLoi` LẪN `soTong` ĐỀU `0`) — NẾU không CÓ dòng
`if (soTong === 0) return 0`, phép chia `0/0` sẽ trả VỀ `NaN` (KHÔNG
PHẢI lỗi, CHỈ LÀ một giá trị "không xác định" ÂM THẦM lan RA khắp hệ
thống). Guard NGĂN điều ĐÓ NGAY từ đầu.
::::

::::example{#vuot-nguong}
TỪ tỷ lệ lỗi, quyết định CÓ **vượt ngưỡng** cảnh báo HAY KHÔNG:

```typescript title=readonly
function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}
function vuotNguong(soLoi: number, soTong: number, nguong: number): boolean {
  return tinhTyLeLoi(soLoi, soTong) > nguong;
}

console.log(vuotNguong(5, 100, 0.1));
console.log(vuotNguong(15, 100, 0.1));
console.log(vuotNguong(0, 0, 0.1));
```

```text title=readonly
false
true
false
```

`5/100 = 0.05` (5%) CHƯA vượt ngưỡng `0.1` (10%) → `false`. `15/100
= 0.15` (15%) VƯỢT ngưỡng → `true`. `vuotNguong(0, 0, ...)` (KHÔNG
traffic) → tỷ lệ LÀ `0` (NHỜ guard Ở `tinhTyLeLoi`) → KHÔNG bao GIỜ
báo động GIẢ khi hệ thống ĐANG rảnh.
::::

::::predict{#doan-chia-cho-khong-khong-guard commitOnce}
```typescript
function tinhTyLeLoiSai(soLoi: number, soTong: number): number {
  return soLoi / soTong;
}
console.log(tinhTyLeLoiSai(0, 0) > 0.5);
```

`tinhTyLeLoiSai` KHÔNG CÓ guard cho `soTong === 0`. Dòng cuối in
ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì `0/0` LÀ MỘT phép chia "không xác định" trong TOÁN HỌC,
VÀ khi MỘT giá trị "không xác định" được SO SÁNH VỚI bất kỳ ngưỡng
NÀO, JavaScript coi ĐÓ LÀ trường hợp "VƯỢT quá an TOÀN" NÊN trả VỀ
`true` (ưu tiên CẢNH BÁO nhầm CÒN HƠN bỏ SÓT)
::why
Gần đúng ở việc bạn nhớ ĐÚNG `0/0` LÀ "không xác định" VỀ mặt TOÁN
HỌC — quan sát ĐÓ chính xác.

Chỗ lệch: JavaScript KHÔNG hề CÓ quy tắc "ưu tiên cảnh báo NHẦM" NÀO
CẢ. VỀ mặt kỹ THUẬT, `0/0` (VỚI kiểu `number`) tính RA `NaN` — VÀ
**MỌI** phép SO SÁNH liên quan `NaN` (`>`, `<`, `>=`, `<=`, THẬM CHÍ
`NaN === NaN`) ĐỀU trả VỀ `false`, KHÔNG ngoại lệ. `NaN > 0.5` LÀ
`false` — ĐÂY chính LÀ lý DO guard `if (soTong === 0) return 0` bắt
buộc PHẢI CÓ: THIẾU nó, hệ thống ÂM THẦM "KHÔNG BAO GIỜ cảnh báo"
đúng lúc `soTong` LÀ `0`, CHỨ KHÔNG PHẢI cảnh báo NHẦM.
::
:::

:::opt
Máy báo lỗi runtime ("Division by zero") NGAY TẠI dòng
`return soLoi / soTong;` — GIỐNG cách chia CHO `0` GÂY lỗi Ở nhiều
ngôn NGỮ khác (Python, Rust)
::why
Gần đúng ở việc bạn liên tưởng ĐÚNG rằng NHIỀU ngôn ngữ (Python,
Rust VỚI kiểu nguyên) THẬT SỰ ném lỗi KHI chia CHO `0` — một PHẢN
XẠ hợp LÝ từ kinh nghiệm NGÔN NGỮ khác.

Chỗ lệch: JavaScript/TypeScript (VỚI kiểu `number`, LÀ số THỰC dấu
PHẨY động) KHÔNG BAO GIỜ ném lỗi khi CHIA cho `0` — nó LUÔN trả VỀ
MỘT giá trị (`NaN` cho `0/0`, `Infinity` cho SỐ dương chia `0`,
`-Infinity` cho SỐ âm chia `0`). Chương trình chạy TIẾP BÌNH THƯỜNG,
KHÔNG có exception NÀO — ĐÓ chính LÀ lý do lỗi NÀY nguy hiểm: nó
ÂM THẦM, KHÔNG hề dừng chương trình LẠI để báo.
::
:::
::::

::::code{#viet_ty_le_loi_va_nguong}
Hoàn thiện `tinhTyLeLoi` (guard `soTong === 0`) VÀ `vuotNguong`
(dùng LẠI `tinhTyLeLoi`).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return ___;
  return ___;
}

function vuotNguong(soLoi: number, soTong: number, nguong: number): boolean {
  return ___;
}

assertEqual(tinhTyLeLoi(3, 100), 0.03, "ty le loi binh thuong");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function tinhTyLeLoi(soLoi: number, soTong: number): number {
  if (soTong === 0) return 0;
  return soLoi / soTong;
}

function vuotNguong(soLoi: number, soTong: number, nguong: number): boolean {
  return tinhTyLeLoi(soLoi, soTong) > nguong;
}

assertEqual(tinhTyLeLoi(3, 100), 0.03, "ty le loi binh thuong");
```

```typescript title=test
assertEqual(tinhTyLeLoi(0, 0), 0, "khong request nao -- khong chia cho 0");
assertEqual(tinhTyLeLoi(0, 100), 0, "khong loi nao");
assertEqual(tinhTyLeLoi(100, 100), 1, "toan bo la loi");
assertEqual(vuotNguong(5, 100, 0.1), false, "5% chua vuot nguong 10%");
assertEqual(vuotNguong(15, 100, 0.1), true, "15% vuot nguong 10%");
assertEqual(vuotNguong(0, 0, 0.1), false, "khong traffic -- khong bao dong gia");
assertEqual(vuotNguong(10, 100, 0.1), false, "dung bang nguong -- chua goi la vuot");
```

:::hints
- kind: attention
  body: "Guard truoc: soTong === 0 thi tra ve 0 (KHONG chia). vuotNguong dung lai tinhTyLeLoi roi so sanh > nguong."
- kind: strategy
  body: "return 0 : return soLoi / soTong : return tinhTyLeLoi(soLoi, soTong) > nguong"
- kind: one-line
  body: '___ (guard) = 0\n___ (chia) = soLoi / soTong\n___ (vuotNguong) = tinhTyLeLoi(soLoi, soTong) > nguong'
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
Tỷ lệ lỗi + ngưỡng = một quyết định rõ ràng thay vì hai con số rời
rạc. Bài tiếp theo: Counter đo SỐ LƯỢNG — còn đo TỐC ĐỘ (latency)?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`vuotNguong` trả lời "CÓ vượt hay KHÔNG" bằng MỘT con số TRUNG BÌNH
kiểu tỷ lệ. NHƯNG "request CHẬM NHẤT mất bao lâu" — trung bình CÓ
đủ để trả lời KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
