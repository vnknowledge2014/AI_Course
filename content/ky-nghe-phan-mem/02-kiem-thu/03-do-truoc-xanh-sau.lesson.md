---
id: ky-nghe-phan-mem.kiem-thu.do-truoc-xanh-sau
title: "Đỏ trước, Xanh sau — viết assertion trước, code tối thiểu sau"
summary: "Viết assertion CHO hành vi mong muốn của parsePrice TRƯỚC KHI cài đặt (Đỏ — thất bại vì hàm chưa tồn tại/chưa đúng), RỒI viết code TỐI THIỂU khiến assertion đó qua (Xanh). Trải nghiệm TRỰC TIẾP nhịp Đỏ→Xanh, không chỉ đọc VỀ nó."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.red-then-green]
requires: [kt.hand-rolled-assert]
concepts: [kt.red-then-green]
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
Có `assertEqual` trong tay. Viết assertion cho `parsePrice` (chuỗi
giá tiền → số, hoặc `null` nếu không hợp lệ) TRƯỚC KHI cài đặt?
::::

::::explain{#do-parseprice}
Bước **Đỏ**: viết assertion CHO hành vi MONG MUỐN CỦA `parsePrice`
**TRƯỚC**, chạy TRÊN một hàm **CHƯA cài đặt** (chỉ `throw`) — assertion
THẤT BẠI, ĐÚNG như DỰ KIẾN:

```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// ĐỎ: parsePrice CHƯA cài đặt
function parsePrice(input: string): number | null {
  throw new Error("chua cai dat");
}

try {
  assertEqual(parsePrice("42,500.99"), 42500.99, "parse gia co dau phay");
} catch (e) {
  console.log("DO:", (e as Error).message);
}
```

```text
DO: chua cai dat
```

Assertion NÀY **ĐÃ** viết XONG — TRỌN VẸN, RÕ RÀNG — TRƯỚC KHI có
BẤT KỲ dòng code THẬT nào cài đặt `parsePrice`. Bước ĐỎ CHỨNG MINH:
assertion CHẠY TỚI đúng chỗ (KHÔNG BỊ lỗi cú pháp), VÀ hàm THẬT SỰ
CHƯA làm gì (nếu SAU NÀY, đâu đó, VÔ TÌNH quên gọi `parsePrice`,
assertion vẫn THẤT BẠI — KHÔNG "xanh giả").
::::

::::example{#xanh-parseprice}
Bước **Xanh**: viết code TỐI THIỂU khiến assertion QUA:

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

assertEqual(parsePrice("42,500.99"), 42500.99, "parse gia co dau phay");
assertEqual(parsePrice("abc"), null, "chuoi khong phai so tra ve null");
```

```text title=readonly
[PASS] parse gia co dau phay
[PASS] chuoi khong phai so tra ve null
```

`parsePrice` XOÁ dấu phẩy (`replace(/,/g, "")`), CHUYỂN chuỗi sang số
(`Number`), TỪ CHỐI (`null`) nếu KHÔNG PHẢI số HỢP LỆ (`isNaN`) hoặc
ÂM. HAI assertion ĐÃ VIẾT SẴN (bước Đỏ) GIỜ ĐỀU `[PASS]` — KHÔNG cần
sửa MỘT assertion nào, CHỈ THÊM code CÀI ĐẶT.
::::

::::predict{#doan-them-mot-truong-hop-moi commitOnce}
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

// Chuỗi TOÀN dấu phẩy, KHÔNG số nào
try {
  assertEqual(parsePrice(",,,"), null, "toan dau phay khong co so");
} catch (e) {
  console.log((e as Error).message);
}
```

Dòng cuối in ra gì?

:::opt{correct}
`[FAIL] toan dau phay khong co so: mong null, nhan 0`
:::

:::opt
Không in gì cả — assertion ĐÃ QUA (`[PASS]`), vì `",,,"` KHÔNG chứa
chữ số nào, nên `Number(...)` PHẢI ra `NaN`, `Number.isNaN(NaN)` LÀ
`true`, hàm trả `null` ĐÚNG NHƯ MONG ĐỢI
::why
Gần đúng ở việc bạn nghĩ `",,,"` "trông" như KHÔNG PHẢI một số HỢP
LỆ — một trực giác NGÔN NGỮ TỰ NHIÊN hợp lý (con người ĐỌC nó không
thấy chữ số nào).

Chỗ lệch: `input.replace(/,/g, "")` XOÁ **HẾT** dấu phẩy khỏi `",,,"`,
để lại **CHUỖI RỖNG** (`""`) — VÀ `Number("")` trong JavaScript
**KHÔNG** ra `NaN`, nó ra **`0`** (một sự thật DỄ NHẦM: chuỗi RỖNG
chuyển số LUÔN ra `0`, CHỈ chuỗi CHỨA ký tự KHÔNG phải số như `"abc"`
mới ra `NaN`). `Number.isNaN(0)` là `false`, `0 < 0` CŨNG `false` —
điều kiện từ chối **KHÔNG** thoả, hàm ĐI TỚI `Math.round(0 * 100) /
100 = 0`. `parsePrice(",,,")` trả `0` — assertion (mong đợi `null`)
**THẤT BẠI**, đúng cái BẪY mà bước "liệt kê trường hợp biên" (bài
sau) tồn tại để BẮT.
::
:::

:::opt
Máy báo lỗi biên dịch — `parsePrice(",,,")` không hợp lệ, vì chuỗi
CHỈ CHỨA dấu phẩy không khớp kiểu tham số `input: string` (TypeScript
đòi chuỗi PHẢI CHỨA ít nhất MỘT chữ số)
::why
Gần đúng ở việc bạn nghĩ tới việc CÓ THỂ có RÀNG BUỘC nội dung cho
tham số `string` — một trực giác dễ hiểu khi hàm RÕ RÀNG "chỉ nên"
nhận chuỗi SỐ.

Chỗ lệch: kiểu `string` trong TypeScript chấp nhận **BẤT KỲ** chuỗi
nào — KHÔNG CÓ cách khai "chuỗi PHẢI chứa chữ số" Ở TẦNG KIỂU (đó LÀ
ràng buộc NGHIỆP VỤ, kiểm LÚC CHẠY, giống MỌI validate khác đã học
xuyên suốt dự án). `",,,"` LÀ một `string` HOÀN TOÀN hợp lệ về mặt
kiểu. Biên dịch sạch — hành vi "đúng hay sai" CHỈ lộ ra LÚC CHẠY, như
đã phân tích Ở phương án ĐÚNG.
::
:::
::::

::::code{#viet_parseprice}
Tự viết phần lõi của `parsePrice`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function parsePrice(input: string): number | null {
  const cleaned = input.replace(/,/g, "");
  const n = Number(cleaned);
  if (___) return ___;
  return Math.round(n * 100) / 100;
}

assertEqual(parsePrice("1,000"), 1000, "so co dau phay");
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

assertEqual(parsePrice("1,000"), 1000, "so co dau phay");
```

```typescript title=test
assertEqual(parsePrice("42,500.99"), 42500.99, "so co dau phay va thap phan");
assertEqual(parsePrice("abc"), null, "chuoi khong phai so");
assertEqual(parsePrice("-100"), null, "so am phai tra ve null");
assertEqual(parsePrice("100"), 100, "so nguyen don gian");

// Biên: đúng ngưỡng 0 phải được CHẤP NHẬN (dùng <, không phải <=)
assertEqual(parsePrice("0"), 0, "so khong dung nguong phai duoc chap nhan");
```

:::hints
- kind: attention
  body: "Điều kiện từ chối: n KHÔNG PHẢI số hợp lệ (Number.isNaN) HOẶC n ÂM. Nếu từ chối thì trả null."
- kind: strategy
  body: "Number.isNaN(n) || n < 0 : null — điều kiện gộp hai lý do từ chối, trả null nếu MỘT trong hai đúng."
- kind: one-line
  body: "___ (điều kiện) = Number.isNaN(n) || n < 0\n___ (giá trị trả) = null"
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
Đỏ (assertion viết trước, thất bại có chủ đích) → Xanh (code tối
thiểu). Bước tiếp theo: liệt kê trường hợp biên TRƯỚC khi viết code.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`parsePrice("")` (chuỗi rỗng), `parsePrice("0")` (số không), và các
trường hợp KỲ LẠ khác — TDD xử lý chúng như thế nào, TRƯỚC khi viết
code hay SAU?
::::

::::checkpoint{mastery=0.8}
::::
