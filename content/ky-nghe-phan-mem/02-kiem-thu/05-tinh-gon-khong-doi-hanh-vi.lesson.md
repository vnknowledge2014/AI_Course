---
id: ky-nghe-phan-mem.kiem-thu.tinh-gon-khong-doi-hanh-vi
title: "Capstone: Tinh gọn — đổi CÁCH LÀM, giữ NGUYÊN test"
summary: "Bài chốt cụm 1: viết parsePriceV2 (tách hàm lamSachChuoiGia riêng, đảo thứ tự điều kiện) — cài đặt KHÁC, CHẠY LẠI Y NGUYÊN bộ assertion đã viết (bài 3-4), không sửa MỘT assertion nào. Test là LƯỚI AN TOÀN, không phải thứ viết lại mỗi lần đổi cài đặt."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kt.refactor-safety-net]
requires: [kt.edge-cases-first]
concepts: [kt.refactor-safety-net]
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
Bài chốt cụm 1. `parsePrice` có THỂ viết LẠI theo CÁCH KHÁC, CÙNG hành
vi — bộ assertion ĐÃ VIẾT (bài 3-4) có cần SỬA gì không?
::::

::::explain{#refactor-tach-ham}
Viết `parsePriceV2` — **TÁCH** bước "làm sạch chuỗi" thành MỘT hàm
**RIÊNG** (`lamSachChuoiGia`, dễ ĐỌC/TÁI SỬ DỤNG hơn) VÀ **ĐẢO thứ
tự** hai điều kiện từ chối (`||` giao hoán, Ý NGHĨA GIỮ NGUYÊN) —
**CÀI ĐẶT KHÁC**, HÀNH VI **PHẢI GIỮ NGUYÊN**:

```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// V1 (bài 3-4)
function parsePrice(input: string): number | null {
  const cleaned = input.replace(/,/g, "");
  const n = Number(cleaned);
  if (Number.isNaN(n) || n < 0) return null;
  return Math.round(n * 100) / 100;
}

// V2: TÁCH hàm làm sạch riêng, ĐẢO thứ tự điều kiện
function lamSachChuoiGia(input: string): string {
  return input.replace(/,/g, "");
}
function parsePriceV2(input: string): number | null {
  const n = Number(lamSachChuoiGia(input));
  if (n < 0 || Number.isNaN(n)) return null;
  return Math.round(n * 100) / 100;
}

assertEqual(parsePriceV2("42,500.99"), 42500.99, "so co dau phay va thap phan");
assertEqual(parsePriceV2("-5"), null, "so am tra ve null");
```

```text
[PASS] so co dau phay va thap phan
[PASS] so am tra ve null
```

`parsePriceV2` **TRÔNG KHÁC** (thêm MỘT hàm phụ, đảo THỨ TỰ điều
kiện) — NHƯNG hành vi QUAN SÁT ĐƯỢC (input → output) **GIỐNG HỆT**
`parsePrice`. Assertion VIẾT CHO `parsePrice` (bài 3-4) DÙNG được
NGUYÊN VẸN CHO `parsePriceV2`, CHỈ đổi TÊN hàm gọi.
::::

::::example{#chay-toan-bo-bo-test-tren-ca-hai}
CHẠY **TOÀN BỘ** bộ test (kể cả trường hợp biên KỲ LẠ — chuỗi RỖNG,
bài 3-4) TRÊN **CẢ HAI** cài đặt — bằng chứng ĐẦY ĐỦ HÀNH VI GIỮ
NGUYÊN, KHÔNG CHỈ vài ví dụ "dễ":

```typescript title=readonly
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}
function parsePrice(input: string): number | null {
  const cleaned = input.replace(/,/g, "");
  const n = Number(cleaned);
  if (Number.isNaN(n) || n < 0) return null;
  return Math.round(n * 100) / 100;
}
function lamSachChuoiGia(input: string): string {
  return input.replace(/,/g, "");
}
function parsePriceV2(input: string): number | null {
  const n = Number(lamSachChuoiGia(input));
  if (n < 0 || Number.isNaN(n)) return null;
  return Math.round(n * 100) / 100;
}

const boTest: Array<[string, number | null, string]> = [
  ["42,500.99", 42500.99, "so co dau phay va thap phan"],
  ["abc", null, "chuoi khong phai so"],
  ["0", 0, "zero la gia tri hop le"],
  ["-5", null, "so am tra ve null"],
  ["100.005", 100.01, "lam tron dung"],
  ["", 0, "chuoi rong -- ca bien KY LA da phat hien o bai truoc"],
];

for (const [input, kyVong, nhan] of boTest) assertEqual(parsePrice(input), kyVong, `V1: ${nhan}`);
for (const [input, kyVong, nhan] of boTest) assertEqual(parsePriceV2(input), kyVong, `V2: ${nhan}`);
```

```text title=readonly
[PASS] V1: so co dau phay va thap phan
[PASS] V1: chuoi khong phai so
[PASS] V1: zero la gia tri hop le
[PASS] V1: so am tra ve null
[PASS] V1: lam tron dung
[PASS] V1: chuoi rong -- ca bien KY LA da phat hien o bai truoc
[PASS] V2: so co dau phay va thap phan
[PASS] V2: chuoi khong phai so
[PASS] V2: zero la gia tri hop le
[PASS] V2: so am tra ve null
[PASS] V2: lam tron dung
[PASS] V2: chuoi rong -- ca bien KY LA da phat hien o bai truoc
```

**MƯỜI HAI** dòng `[PASS]` — KỂ CẢ ca biên KỲ LẠ (chuỗi rỗng → `0`,
bài 3 PHÁT HIỆN, KHÔNG PHẢI direct chọn). Đây LÀ ĐÚNG Ý NGHĨA của
"Tinh gọn" (Refactor): đổi **CÁCH** làm (HOW — thêm hàm phụ, đảo thứ
tự), giữ NGUYÊN **CÁI** làm ra (WHAT — MỌI input→output GIỐNG HỆT) —
test LÀ LƯỚI AN TOÀN, KHÔNG PHẢI thứ phải viết LẠI MỖI LẦN đổi cài
đặt.
::::

::::predict{#doan-refactor-tiep-theo commitOnce}
```typescript
function lamSachChuoiGia(input: string): string {
  return input.replace(/,/g, "");
}
function parsePriceV2(input: string): number | null {
  const n = Number(lamSachChuoiGia(input));
  if (n < 0 || Number.isNaN(n)) return null;
  return Math.round(n * 100) / 100;
}

// Refactor TIẾP: đảo LẠI thứ tự điều kiện một LẦN NỮA (về gần giống V1)
function parsePriceV3(input: string): number | null {
  const n = Number(lamSachChuoiGia(input));
  if (Number.isNaN(n) || n < 0) return null;
  return Math.round(n * 100) / 100;
}

console.log(parsePriceV2("-5") === parsePriceV3("-5"));
console.log(parsePriceV2("abc") === parsePriceV3("abc"));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`true` rồi `true`
:::

:::opt
`false` rồi `true` — vì `parsePriceV3` kiểm `Number.isNaN(n)` TRƯỚC
`n < 0`, còn `parsePriceV2` kiểm NGƯỢC LẠI, nên với input `"-5"`
(một số ÂM HỢP LỆ, KHÔNG PHẢI `NaN`), hai hàm ĐI qua NHÁNH kiểm tra
KHÁC NHAU và có thể ra KẾT QUẢ khác nhau
::why
Gần đúng ở việc bạn nhớ ĐÚNG `parsePriceV2` VÀ `parsePriceV3` kiểm
HAI điều kiện THEO THỨ TỰ **NGƯỢC** nhau (`n < 0` trước vs
`Number.isNaN(n)` trước) — một quan sát CHÍNH XÁC về CẤU TRÚC code.

Chỗ lệch: `||` (HOẶC) là toán tử **GIAO HOÁN** VỀ MẶT KẾT QUẢ CUỐI
CÙNG — `if (A || B)` VÀ `if (B || A)` LUÔN cho ra **CÙNG** quyết định
`true`/`false` (dù THỨ TỰ ĐÁNH GIÁ từng vế có khác, kết quả TỔNG
GIỐNG NHAU MIỄN LÀ CẢ HAI vế KHÔNG có side-effect GÂY ẢNH HƯỞNG LẪN
NHAU — Ở ĐÂY, `n < 0` VÀ `Number.isNaN(n)` là HAI PHÉP KIỂM ĐỘC LẬP,
KHÔNG side-effect). Với `"-5"`: `n = -5`, `n < 0` là `true`, cả HAI
hàm ĐỀU `return null` (chỉ khác THỨ TỰ kiểm, KHÔNG khác KẾT QUẢ). Với
`"abc"`: `n = NaN`, `Number.isNaN(n)` là `true`, CẢ HAI hàm ĐỀU
`return null`. Đảo thứ tự các vế của `||`/`&&` (khi CẢ HAI vế THUẦN
TUÝ, không side-effect) là một refactor AN TOÀN, KHÔNG đổi hành vi.
::
:::

:::opt
Máy báo lỗi biên dịch — `parsePriceV3` khai TRÙNG cấu trúc THÂN hàm
với `parsePriceV2` (chỉ khác thứ tự HAI điều kiện), TypeScript coi
đây LÀ khai HÀM TRÙNG LẶP không hợp lệ
::why
Gần đúng ở việc bạn để ý `parsePriceV2` VÀ `parsePriceV3` có THÂN hàm
RẤT GIỐNG NHAU (chỉ khác MỘT chi tiết nhỏ) — một quan sát ĐÚNG về SỰ
TƯƠNG ĐỒNG cấu trúc.

Chỗ lệch: TypeScript KHÔNG có khái niệm "hai hàm KHÁC TÊN nhưng THÂN
GIỐNG NHAU LÀ trùng lặp không hợp lệ" — MỖI hàm là MỘT khai báo ĐỘC
LẬP, được PHÉP có THÂN GIỐNG NHAU (hoặc GẦN giống) tuỳ Ý người viết.
Biên dịch sạch — TypeScript chỉ quan tâm TÊN có TRÙNG NHAU trong CÙNG
MỘT phạm vi hay không (`parsePriceV2` VÀ `parsePriceV3` là HAI TÊN
KHÁC NHAU, không xung đột).
::
:::
::::

::::code{#viet_parsepricev2}
Tự viết `lamSachChuoiGia` VÀ điều kiện từ chối trong `parsePriceV2`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function lamSachChuoiGia(input: string): string {
  return ___;
}
function parsePriceV2(input: string): number | null {
  const n = Number(lamSachChuoiGia(input));
  if (___) return null;
  return Math.round(n * 100) / 100;
}

assertEqual(parsePriceV2("1,000"), 1000, "so co dau phay");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function lamSachChuoiGia(input: string): string {
  return input.replace(/,/g, "");
}
function parsePriceV2(input: string): number | null {
  const n = Number(lamSachChuoiGia(input));
  if (n < 0 || Number.isNaN(n)) return null;
  return Math.round(n * 100) / 100;
}

assertEqual(parsePriceV2("1,000"), 1000, "so co dau phay");
```

```typescript title=test
const boTest: Array<[string, number | null, string]> = [
  ["42,500.99", 42500.99, "so co dau phay va thap phan"],
  ["abc", null, "chuoi khong phai so"],
  ["0", 0, "zero la gia tri hop le"],
  ["-5", null, "so am tra ve null"],
  ["100.005", 100.01, "lam tron dung"],
  ["", 0, "chuoi rong -- ca bien"],
];
for (const [input, kyVong, nhan] of boTest) {
  assertEqual(parsePriceV2(input), kyVong, `V2: ${nhan}`);
}
```

:::hints
- kind: attention
  body: "lamSachChuoiGia: giống hệt bước làm sạch của parsePrice bài 3 (xoá dấu phẩy). parsePriceV2's điều kiện từ chối: GIỐNG NGHĨA bài 3-4 (NaN hoặc âm), chỉ ĐẢO thứ tự hai vế của ||."
- kind: strategy
  body: 'input.replace(/,/g, "") : n < 0 || Number.isNaN(n) — tách hàm làm sạch, đảo thứ tự OR (kết quả tổng KHÔNG đổi).'
- kind: one-line
  body: '___ (lamSachChuoiGia) = input.replace(/,/g, "")\n___ (điều kiện) = n < 0 || Number.isNaN(n)'
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
Cụm 1 hoàn tất: Đỏ-Xanh-Tinh gọn, tự viết assert, biên có ý thức,
refactor không đổi hành vi. Cụm tiếp theo: test domain logic thật —
Result, máy trạng thái.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`parsePrice` trả `number | null` — một hàm KHÁC trả `Result<T,E>`
(đã học T4.5, dùng lại xuyên suốt dự án) — test hàm ĐÓ có gì KHÁC?
::::

::::checkpoint{mastery=0.8}
::::
