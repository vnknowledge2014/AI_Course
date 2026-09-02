---
id: ky-nghe-phan-mem.kiem-thu.truong-hop-bien-co-y-thuc
title: "Trường hợp biên có Ý THỨC — liệt kê TRƯỚC khi viết code"
summary: "TDD buộc liệt kê rỗng/âm/không/biên TRƯỚC KHI viết code — mở rộng parsePrice với \"0\" (hợp lệ), \"\" (bài trước đã lộ: ra 0, không phải null), \"-5\" (null), \"12.345\" (làm tròn). Mỗi trường hợp biên là MỘT assertion riêng, viết trước, rồi điều chỉnh cài đặt."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.edge-cases-first]
requires: [kt.red-then-green]
concepts: [kt.edge-cases-first]
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
`parsePrice("")` (chuỗi rỗng), `parsePrice("0")` (số không) — bài
trước đã lộ MỘT bất ngờ. TDD xử lý các trường hợp KỲ LẠ như thế nào?
::::

::::explain{#bien-truoc-code-sau}
TDD **BUỘC** liệt kê rỗng/âm/không/biên **TRƯỚC** KHI viết code
(KHÔNG PHẢI nghĩ thêm SAU, khi đã "quên mất" các trường hợp lạ). MỞ
RỘNG `parsePrice` (bài 3) với BỐN trường hợp biên:

```typescript
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

assertEqual(parsePrice("0"), 0, "zero la gia tri hop le");
assertEqual(parsePrice(""), 0, "chuoi rong -- da phat hien o bai truoc: ra 0, KHONG PHAI null");
assertEqual(parsePrice("-5"), null, "so am tra ve null");
assertEqual(parsePrice("12.345"), 12.35, "lam tron 2 chu so thap phan");
```

```text
[PASS] zero la gia tri hop le
[PASS] chuoi rong -- da phat hien o bai truoc: ra 0, KHONG PHAI null
[PASS] so am tra ve null
[PASS] lam tron 2 chu so thap phan
```

Assertion THỨ HAI (chuỗi rỗng) GHI RÕ hành vi **PHÁT HIỆN được** Ở
bài TRƯỚC (`parsePrice("")` ra `0`, KHÔNG PHẢI `null`) — thay vì để
đó như một Ý ĐỊNH ngầm HIỂU, nó TRỞ THÀNH một assertion **RÕ RÀNG**,
**GHI LẠI** quyết định NÀY là CÓ CHỦ Ý (không phải bug bị BỎ SÓT).
::::

::::example{#lieu-ke-truoc-tranh-quen}
LIỆT KÊ trường hợp biên **TRƯỚC** giúp TRÁNH việc "quên" — nếu VIẾT
CODE trước rồi MỚI nghĩ test, RẤT DỄ chỉ nghĩ ra những case "thuận":

```typescript title=readonly
// Nếu VIẾT CODE TRƯỚC, "trực giác" thường CHỈ nghĩ ra:
function parsePriceThieuCanThan(input: string): number | null {
  const n = Number(input.replace(/,/g, ""));
  return Number.isNaN(n) ? null : n; // QUÊN kiểm số ÂM!
}

// Assertion VIẾT TRƯỚC (bài học TDD) sẽ BẮT được thiếu sót NÀY ngay:
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}
try {
  assertEqual(parsePriceThieuCanThan("-5"), null, "so am phai tra ve null");
} catch (e) {
  console.log((e as Error).message);
}
```

```text title=readonly
[FAIL] so am phai tra ve null: mong null, nhan -5
```

`parsePriceThieuCanThan` (viết THEO trực giác, KHÔNG liệt kê biên
TRƯỚC) QUÊN kiểm số ÂM — assertion "số âm phải trả `null`" (ĐÃ VIẾT
SẴN TỪ TRƯỚC, THEO danh sách biên) **BẮT ĐƯỢC** thiếu sót ĐÓ NGAY,
KHÔNG PHẢI đợi tới khi có BUG BÁO CÁO từ người dùng THẬT.
::::

::::predict{#doan-truong-hop-bien-moi commitOnce}
```typescript
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

// Trường hợp biên MỚI: chuỗi TOÀN khoảng trắng
try {
  assertEqual(parsePrice("   "), null, "chuoi toan khoang trang phai tra ve null");
} catch (e) {
  console.log((e as Error).message);
}
```

Dòng cuối in ra gì?

:::opt{correct}
`[FAIL] chuoi toan khoang trang phai tra ve null: mong null, nhan 0`
:::

:::opt
Không in gì cả — assertion ĐÃ QUA, vì `"   "` (khoảng trắng) rõ ràng
KHÔNG PHẢI một số, `Number("   ")` PHẢI ra `NaN`, hàm trả `null` ĐÚNG
NHƯ MONG ĐỢI
::why
Gần đúng ở việc bạn nghĩ chuỗi TOÀN khoảng trắng "trông" giống chuỗi
RỖNG (bài học TRƯỚC: `Number("")` = `0`) hoặc "trông" giống KHÔNG
PHẢI số — MỘT trong hai trực giác ĐÓ đúng LÀ liên quan.

Chỗ lệch: `Number("   ")` (chuỗi TOÀN khoảng trắng) trong JavaScript
**CŨNG** ra `0` (KHÔNG PHẢI `NaN`) — quy tắc CHUYỂN ĐỔI CHUỖI SANG SỐ
của JavaScript coi khoảng trắng LÀ "phần thừa" bị BỎ QUA, chuỗi CÒN
LẠI SAU KHI bỏ khoảng trắng LÀ rỗng, VÀ (giống bài TRƯỚC) chuỗi RỖNG
chuyển số ra `0`. `parsePrice("   ")` trả `0`, KHÔNG PHẢI `null` —
assertion (mong đợi `null`) THẤT BẠI. Đây LÀ **CHÍNH XÁC** giá trị của
việc liệt kê biên TRƯỚC: assertion NÀY, MỘT KHI đã VIẾT, sẽ MÃI MÃI
nhắc NHỚ rằng "chuỗi trắng" LÀ một trường hợp CẦN xử lý RIÊNG (ví dụ
`input.trim()` TRƯỚC khi kiểm), KHÔNG bị BỎ QUÊN.
::
:::

:::opt
Máy báo lỗi biên dịch — `parsePrice("   ")` không hợp lệ, vì
TypeScript coi chuỗi CHỈ chứa khoảng trắng LÀ kiểu ĐẶC BIỆT (whitespace
string), KHÔNG khớp `input: string` bình thường
::why
Gần đúng ở việc bạn nghĩ tới việc chuỗi TOÀN khoảng trắng "trông" khá
ĐẶC BIỆT so với chuỗi thường — một trực giác dễ hiểu vì nội dung của
nó "trống rỗng" về mặt Ý NGHĨA.

Chỗ lệch: TypeScript KHÔNG có "kiểu chuỗi khoảng trắng" NÀO tách
biệt — `"   "` LÀ một `string` **BÌNH THƯỜNG**, hoàn toàn giống
`"abc"`/`""` về mặt KIỂU (chỉ khác NỘI DUNG ký tự). Biên dịch sạch —
sự khác biệt CHỈ lộ ra LÚC CHẠY, như phân tích Ở phương án ĐÚNG.
::
:::
::::

::::code{#viet_them_bien_parseprice}
Tự viết giá trị MONG ĐỢI cho BA trường hợp biên.

```typescript title=starter
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

assertEqual(parsePrice("0"), ___, "zero la gia tri hop le");
assertEqual(parsePrice("-5"), ___, "so am tra ve null");
assertEqual(parsePrice("100.005"), ___, "lam tron dung");
```

```typescript title=solution
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

assertEqual(parsePrice("0"), 0, "zero la gia tri hop le");
assertEqual(parsePrice("-5"), null, "so am tra ve null");
assertEqual(parsePrice("100.005"), 100.01, "lam tron dung");
```

```typescript title=test
assertEqual(parsePrice("0"), 0, "zero test lai");
assertEqual(parsePrice("-5"), null, "so am test lai");
assertEqual(parsePrice("100.005"), 100.01, "lam tron test lai");
assertEqual(parsePrice("1,234.5"), 1234.5, "so lon co dau phay van hoat dong");
```

:::hints
- kind: attention
  body: "Chạy THỬ parsePrice bằng trí óc theo ĐÚNG code đã cho: '0' -> Number('0')=0, không âm, không NaN, giữ nguyên. '-5' -> âm, trả null. '100.005' -> Math.round(10000.5)/100 = 10001/100 = 100.01."
- kind: strategy
  body: "0 : null : 100.01 — ba giá trị mong đợi, đúng theo hành vi CÓ SẴN của parsePrice."
- kind: one-line
  body: "___ (zero) = 0\n___ (so am) = null\n___ (lam tron) = 100.01"
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
Liệt kê biên TRƯỚC, không phải nghĩ thêm SAU — mỗi biên là MỘT
assertion riêng, ghi lại quyết định CÓ CHỦ Ý. Bước tiếp theo — chốt
cụm: đổi CÁCH LÀM, giữ NGUYÊN test.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`parsePrice` (bản HIỆN TẠI, dùng `Number()` trực tiếp) có THỂ viết
LẠI bằng regex, CÙNG hành vi — bộ assertion ĐÃ VIẾT (bài 3-4) có cần
sửa GÌ không?
::::

::::checkpoint{mastery=0.8}
::::
