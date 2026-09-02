---
id: ky-nghe-phan-mem.kiem-thu.dot-bien-la-gi
title: "Đột biến là gì — cố ý gieo LỖI NHỎ, xem test có BẮT được không"
summary: "Mutation testing: cố ý sửa MỘT CHỖ NHỎ trong lời giải ĐÚNG (đột biến) rồi chạy LẠI bộ test HIỆN CÓ trên bản đột biến. Đột biến 'BỊ GIẾT' nếu MỘT test thất bại (tốt); 'SỐNG SÓT' nếu MỌI test vẫn qua (xấu — bộ test KHÔNG thật sự kiểm điều đó, dù trông đủ)."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 27
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.mutation-testing-intro]
requires: [kt.gate-boss-pbt-patterns]
concepts: [kt.mutation-testing-intro]
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
Cụm CHỐT track. "Đột biến" đã được nhắc NHIỀU lần suốt các bài trước.
Đột biến LÀ GÌ, chính XÁC — và TẠI SAO nó kiểm TEST, không kiểm code?
::::

::::explain{#dot-bien-la-gi}
**Mutation testing**: CỐ Ý sửa **MỘT CHỖ NHỎ** trong lời giải **ĐÚNG**
(gọi LÀ "đột biến" — mutant, VÍ DỤ đổi `>` thành `>=`) RỒI **CHẠY
LẠI** bộ test **HIỆN CÓ** TRÊN phiên bản đột biến ĐÓ:

```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function laSoDuong(n: number): boolean {
  return n > 0;
}

// bo test HIEN CO
assertEqual(laSoDuong(5), true, "so duong tra ve true");
assertEqual(laSoDuong(-5), false, "so am tra ve false");
```

```text
[PASS] so duong tra ve true
[PASS] so am tra ve false
```

Đột biến **"BỊ GIẾT"** NẾU MỘT test **THẤT BẠI** khi chạy TRÊN phiên
bản đột biến (TỐT — test THẬT SỰ phát hiện được sai khác). Đột biến
**"SỐNG SÓT"** NẾU **MỌI** test **VẪN qua** (XẤU — bộ test **KHÔNG
THẬT SỰ** kiểm điều ĐÓ, DÙ trông "đủ").
::::

::::example{#dot-bien-song-sot}
Đổi `>` thành `>=` (đột biến "đổi Ý NGHĨA thành 'không âm'") RỒI chạy
LẠI **ĐÚNG** bộ test TRÊN — CẢ HAI vẫn `[PASS]`:

```typescript title=readonly
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// DOT BIEN: > thanh >=
function laSoDuongDotBien(n: number): boolean {
  return n >= 0;
}

try {
  assertEqual(laSoDuongDotBien(5), true, "so duong tra ve true");
  assertEqual(laSoDuongDotBien(-5), false, "so am tra ve false");
  console.log("DOT BIEN SONG SOT -- ca hai test van qua");
} catch (e) {
  console.log("DOT BIEN BI GIET:", (e as Error).message);
}

// CHI khi THEM test o dung ranh gioi n=0 moi lo ra
try {
  assertEqual(laSoDuongDotBien(0), false, "n=0 tra ve false");
} catch (e) {
  console.log("DOT BIEN BI GIET boi ca bien:", (e as Error).message);
}
```

```text title=readonly
[PASS] so duong tra ve true
[PASS] so am tra ve false
DOT BIEN SONG SOT -- ca hai test van qua
DOT BIEN BI GIET boi ca bien: [FAIL] n=0 tra ve false: mong false, nhan true
```

Bộ test `n=5`/`n=-5` **TRÔNG** "đủ" (MỘT ca dương, MỘT ca âm) —
NHƯNG THỰC RA **KHÔNG** kiểm được RANH GIỚI `n=0` (số `0` KHÔNG PHẢI
dương, cũng KHÔNG PHẢI âm — MỘT trường hợp ĐẶC BIỆT `laSoDuong` PHẢI
xử lý ĐÚNG). CHỈ test THÊM Ở ĐÚNG ranh giới MỚI "GIẾT" được đột biến
NÀY.
::::

::::predict{#doan-dot-bien-tai-cac-gia-tri-khac commitOnce}
```typescript
function laSoDuong(n: number): boolean { return n > 0; }
function laSoDuongDotBien(n: number): boolean { return n >= 0; }

console.log(
  laSoDuong(1) === laSoDuongDotBien(1),
  laSoDuong(-1) === laSoDuongDotBien(-1),
  laSoDuong(0) === laSoDuongDotBien(0)
);
```

Dòng cuối in ra gì?

:::opt{correct}
`true true false`
:::

:::opt
`true true true` — vì `laSoDuong` VÀ `laSoDuongDotBien` CHỈ khác
NHAU Ở **CÚ PHÁP** (`>` vs `>=`), NHƯNG với MỌI số nguyên, HAI toán
tử NÀY LUÔN cho CÙNG kết quả (GIỐNG các đột biến "TƯƠNG ĐƯƠNG" đã
gặp Ở các track TRƯỚC)
::why
Gần đúng ở việc bạn nhớ ĐÚNG khái niệm "đột biến TƯƠNG ĐƯƠNG" (thay
đổi CÚ PHÁP KHÔNG đổi HÀNH VI) ĐÃ gặp NHIỀU lần Ở CÁC track TRƯỚC —
một quan sát ĐÚNG rằng khái niệm ĐÓ TỒN TẠI.

Chỗ lệch: `>` VÀ `>=` **KHÔNG PHẢI** TƯƠNG ĐƯƠNG — CHÚNG khác NHAU
CHÍNH XÁC Ở đúng **MỘT** điểm: `n = 0`. `laSoDuong(0)`: `0 > 0` LÀ
`false`. `laSoDuongDotBien(0)`: `0 >= 0` LÀ `true`. HAI kết quả
**KHÁC NHAU** Ở `n=0`, nên `laSoDuong(0) === laSoDuongDotBien(0)` LÀ
`false`. VỚI `n=1` VÀ `n=-1` (KHÔNG PHẢI ranh giới), HAI hàm CHO cùng
kết quả — ĐÂY LÀ chính xác LÝ DO tại SAO test `n=5`/`n=-5` (bài
TRƯỚC) KHÔNG bắt được đột biến NÀY: chúng KHÔNG chạm TỚI điểm khác
biệt DUY NHẤT.
::
:::

:::opt
Máy báo lỗi biên dịch — so sánh `laSoDuong(1) === laSoDuongDotBien(1)`
không hợp lệ, vì HAI hàm CÓ TÊN khác nhau nhưng CÙNG chữ ký `(n:
number) => boolean`, TypeScript CẤM so sánh KẾT QUẢ của HAI hàm
"trùng chữ ký" bằng `===`
::why
Gần đúng ở việc bạn để ý HAI hàm CÓ **CÙNG CHỮ KÝ** (`(n: number) =>
boolean`) — một quan sát ĐÚNG về KIỂU của CHÚNG.

Chỗ lệch: TypeScript KHÔNG hề CẤM so sánh GIÁ TRỊ TRẢ VỀ của HAI hàm
BẤT KỲ (dù CHỮ KÝ giống hay khác NHAU) — `laSoDuong(1)` VÀ
`laSoDuongDotBien(1)` ĐỀU LÀ `boolean`, so sánh `===` GIỮA hai
`boolean` HOÀN TOÀN hợp lệ (giống MỌI phép so sánh boolean ĐÃ dùng
xuyên suốt track). Biên dịch sạch.
::
:::
::::

::::code{#viet_test_giet_dot_bien}
Cho `laSoDuong` ĐÃ cài đặt sẵn. Tự viết THÊM assertion để GIẾT đột
biến `>`→`>=` (đã gặp Ở phần trên).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function laSoDuong(n: number): boolean {
  return n > 0;
}

assertEqual(laSoDuong(5), true, "so duong tra ve true");
assertEqual(laSoDuong(-5), false, "so am tra ve false");
assertEqual(laSoDuong(0), ___, "n bang 0 KHONG duoc tinh la so duong");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function laSoDuong(n: number): boolean {
  return n > 0;
}

assertEqual(laSoDuong(5), true, "so duong tra ve true");
assertEqual(laSoDuong(-5), false, "so am tra ve false");
assertEqual(laSoDuong(0), false, "n bang 0 KHONG duoc tinh la so duong");
```

```typescript title=test
// mo phong CHINH dot bien >=  -- neu blank sai (vi du dien true), dong nay se PHAT HIEN
function laSoDuongDotBien(n: number): boolean {
  return n >= 0;
}
let daPhatHienDotBien = false;
try {
  assertEqual(laSoDuongDotBien(0), false, "kiem tra dot bien >= tren gia tri 0");
} catch {
  daPhatHienDotBien = true;
}
if (!daPhatHienDotBien) throw new Error("bo test PHAI phat hien duoc dot bien >= (tuc la assertion o n=0 phai dung false, khong phai true)");
console.log("[PASS] test da giet duoc dot bien >=");
```

:::hints
- kind: attention
  body: "n = 0 KHÔNG PHẢI số dương — laSoDuong(0) phải trả về false. Đây chính là ranh giới mà đột biến >→>= sẽ làm sai."
- kind: strategy
  body: 'false — giá trị mong đợi tại đúng ranh giới n=0.'
- kind: one-line
  body: '___ = false'
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
Đột biến: gieo lỗi nhỏ, xem test có bắt được không. Bước tiếp theo:
độ phủ dòng lệnh KHÁC điểm đột biến — chạy qua khác kiểm đúng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một test gọi `laSoDuong(5)` nhưng KHÔNG assert gì cả — dòng code VẪN
được CHẠY QUA (độ phủ 100%). Điều đó có nghĩa test ĐÓ kiểm ĐÚNG hành
vi không?
::::

::::checkpoint{mastery=0.8}
::::
