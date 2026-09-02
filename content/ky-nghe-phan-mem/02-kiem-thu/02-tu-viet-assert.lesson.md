---
id: ky-nghe-phan-mem.kiem-thu.tu-viet-assert
title: "Tự viết assertEqual/assertDeepEqual — không có thư viện test nào"
summary: "Sandbox không có node:assert/jest/vitest — một test đơn giản LÀ một lời gọi hàm NÉM lỗi khi SAI, in PASS khi ĐÚNG. assertEqual so sánh ===; assertDeepEqual so sánh qua JSON.stringify. Hai hàm này dùng lại XUYÊN SUỐT cả track."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kt.hand-rolled-assert]
requires: [kt.tdd-cycle]
concepts: [kt.hand-rolled-assert]
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
Track này KHÔNG có `node:assert`, jest, hay vitest. "Viết test" nghĩa
LÀ gì nếu KHÔNG có công cụ NÀO cả?
::::

::::explain{#assertequal-tu-viet}
MỘT "test" ĐƠN GIẢN CHỈ LÀ: một lời gọi hàm **NÉM LỖI** khi SAI, **IN
PASS** khi ĐÚNG. `assertEqual<T>(actual, expected, label)` so sánh
`===` — TỰ VIẾT được TRONG BỐN DÒNG:

```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertEqual(2 + 2, 4, "cong hai so");
assertEqual("an" + "binh", "anbinh", "noi chuoi");
```

```text
[PASS] cong hai so
[PASS] noi chuoi
```

`assertEqual` **KHÔNG** trả về giá trị GÌ đặc biệt (`void`) — nó CHỈ
CÓ **HAI HÀNH VI**: `throw` (SAI) hoặc `console.log` (ĐÚNG). Đây LÀ
TOÀN BỘ những gì MỘT test framework THẬT (jest, vitest) làm PHÍA
DƯỚI, đóng gói kỹ hơn — CỐT LÕI VẪN CHỈ LÀ so sánh RỒI báo.
::::

::::example{#assertdeepequal-cho-object}
`===` CHỈ so sánh được kiểu NGUYÊN THUỶ (số, chuỗi, boolean) — HAI
`object`/`array` KHÁC NHAU VỀ THAM CHIẾU LUÔN `!==` nhau dù NỘI DUNG
GIỐNG HỆT. `assertDeepEqual<T>` so sánh qua `JSON.stringify` (đủ
dùng cho object/mảng PHẲNG trong track này):

```typescript title=readonly
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

const nguoiDung1 = { ten: "An", tuoi: 20 };
const nguoiDung2 = { ten: "An", tuoi: 20 };
console.log(nguoiDung1 === nguoiDung2);
assertDeepEqual(nguoiDung1, nguoiDung2, "hai object khac tham chieu nhung cung noi dung");
```

```text title=readonly
false
[PASS] hai object khac tham chieu nhung cung noi dung
```

`nguoiDung1 === nguoiDung2` là `false` (HAI object KHÁC NHAU trong bộ
nhớ, dù NỘI DUNG giống hệt) — NHƯNG `assertDeepEqual` VẪN báo PASS,
vì nó so sánh **CHUỖI JSON** (`'{"ten":"An","tuoi":20}'` CẢ HAI BÊN
GIỐNG HỆT), KHÔNG so sánh THAM CHIẾU.
::::

::::predict{#doan-assertequal-thu-tu-tham-so commitOnce}
```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

try {
  assertEqual(3, 5, "test co the that bai");
} catch (e) {
  console.log((e as Error).message);
}
```

Dòng cuối in ra gì?

:::opt{correct}
`[FAIL] test co the that bai: mong 5, nhan 3`
:::

:::opt
`[FAIL] test co the that bai: mong 3, nhan 5` — vì tham số ĐẦU TIÊN
(`3`) LÀ giá trị "mong đợi" (expected — thường LÀ giá trị viết ĐẦU
TIÊN, quen thuộc như đọc "3 nên bằng 5"), tham số THỨ HAI LÀ giá trị
thực tế
::why
Gần đúng ở việc bạn nghĩ tới một QUY ƯỚC đặt tham số THEO THỨ TỰ đọc
tự nhiên — MỘT SỐ framework THẬT (ví dụ Jest's `expect(actual)
.toBe(expected)`) đặt "actual" TRƯỚC, nhưng CÓ framework KHÁC đặt
NGƯỢC LẠI — không có quy ước DUY NHẤT trên toàn ngành, nên đoán theo
"thứ tự đọc quen" DỄ nhầm.

Chỗ lệch: chữ ký hàm `assertEqual<T>(actual: T, expected: T, label:
string)` khai RÕ **THAM SỐ ĐẦU** LÀ `actual` (giá trị THỰC TẾ nhận
được), **THAM SỐ THỨ HAI** LÀ `expected` (giá trị MONG ĐỢI). Lời gọi
`assertEqual(3, 5, ...)`: `actual = 3`, `expected = 5`. Thông điệp lỗi
`` `[FAIL] ${label}: mong ${expected}, nhan ${actual}` `` GHÉP ĐÚNG
THEO thứ tự khai — `"mong 5, nhan 3"`, KHÔNG PHẢI ngược lại.
::
:::

:::opt
Máy báo lỗi biên dịch — `assertEqual<T>` là hàm GENERIC, gọi với hai
số `3` VÀ `5` (cùng kiểu `number`) không đủ để TypeScript suy ra `T`
LÀ gì, cần khai TƯỜNG MINH `assertEqual<number>(3, 5, ...)`
::why
Gần đúng ở việc bạn nhớ ĐÚNG `assertEqual` LÀ hàm generic (`<T>`) —
một quan sát ĐÚNG về CẤU TRÚC hàm.

Chỗ lệch: TypeScript **TỰ SUY RA** `T` từ CÁC đối số THỰC TẾ truyền
vào (đã dùng kỹ thuật NÀY xuyên suốt track FP TRƯỚC ĐÓ) — `3` VÀ `5`
ĐỀU LÀ `number`, TypeScript suy `T = number` mà KHÔNG CẦN khai tường
minh `<number>`. Chỉ CẦN khai tường minh khi TypeScript **KHÔNG THỂ**
suy ra (ví dụ mảng RỖNG không rõ kiểu phần tử). Biên dịch sạch.
::
:::
::::

::::code{#viet_assertequal}
Tự viết `assertEqual`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (___) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertEqual(10, 10, "hai so bang nhau");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertEqual(10, 10, "hai so bang nhau");
```

```typescript title=test
let khongThrow = true;
try { assertEqual(7, 7, "phai qua khi bang nhau"); } catch { khongThrow = false; }
if (!khongThrow) throw new Error("assertEqual KHÔNG được throw khi actual === expected");

let coThrow = false;
try { assertEqual(7, 8, "phai fail khi khac nhau"); } catch { coThrow = true; }
if (!coThrow) throw new Error("assertEqual PHẢI throw khi actual !== expected");

try {
  assertEqual("a", "b", "kiem thong diep loi");
} catch (e) {
  const msg = (e as Error).message;
  if (!msg.includes("b") || !msg.includes("a")) throw new Error("thông điệp lỗi phải nhắc CẢ giá trị mong đợi lẫn giá trị nhận được");
}

// label PHẢI xuất hiện ĐÚNG như nó được truyền vào, KHÔNG bị lẫn với actual/expected
try {
  assertEqual(100, 200, "gia tri kiem tra");
} catch (e) {
  const msg = (e as Error).message;
  if (!msg.includes("gia tri kiem tra")) throw new Error("thông điệp lỗi phải chứa ĐÚNG NGUYÊN VĂN nhãn (label) đã truyền vào, không đọc nhầm actual/expected thành nhãn");
}
```

:::hints
- kind: attention
  body: "So sánh actual VỚI expected bằng !== — nếu KHÁC nhau thì throw, không cần else vì throw đã dừng hàm ngay."
- kind: strategy
  body: "actual !== expected — một biểu thức boolean duy nhất."
- kind: one-line
  body: "___ = actual !== expected"
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
assertEqual/assertDeepEqual: công cụ test tối thiểu, dùng lại xuyên
suốt track. Bước tiếp theo: dùng chúng cho nhịp Đỏ-Xanh thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Với `assertEqual` trong tay, viết assertion cho `parsePrice` (chuỗi
giá tiền → số, hoặc `null` nếu không hợp lệ) TRƯỚC KHI cài đặt hàm đó
— trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
