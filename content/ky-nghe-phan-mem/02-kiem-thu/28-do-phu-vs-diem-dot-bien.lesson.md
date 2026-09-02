---
id: ky-nghe-phan-mem.kiem-thu.do-phu-vs-diem-dot-bien
title: "Độ phủ dòng lệnh KHÁC điểm đột biến — CHẠY qua khác KIỂM ĐÚNG"
summary: "Độ phủ (code coverage): dòng code có được CHẠY QUA hay không lúc test — 100% độ phủ CHỈ nghĩa là mọi dòng chạy ít nhất một lần, KHÔNG nghĩa là kết quả được kiểm ĐÚNG. Điểm đột biến (mutation score): tỉ lệ đột biến BỊ GIẾT trên tổng số đột biến thử — đo xem test có THẬT SỰ kiểm đúng hành vi."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 28
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.coverage-vs-mutation-score]
requires: [kt.mutation-testing-intro]
concepts: [kt.coverage-vs-mutation-score]
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
Một test gọi `laSoDuong(5)` nhưng KHÔNG assert gì cả — dòng code VẪN
được CHẠY QUA (độ phủ 100%). Điều đó có nghĩa test kiểm ĐÚNG không?
::::

::::explain{#do-phu-khac-diem-dot-bien}
**Độ phủ** (code coverage): DÒNG code đó có **ĐƯỢC CHẠY QUA** hay
KHÔNG lúc test — `100%` độ phủ CHỈ nghĩa LÀ MỌI dòng chạy ÍT NHẤT
MỘT lần, **KHÔNG** nghĩa LÀ KẾT QUẢ được **KIỂM ĐÚNG**. **Điểm đột
biến** (mutation score): TỈ LỆ đột biến **BỊ GIẾT** TRÊN tổng SỐ đột
biến thử — ĐO xem test có THẬT SỰ kiểm ĐÚNG hành VI hay KHÔNG:

```typescript
function laSoDuong(n: number): boolean {
  return n > 0;
}

// test A: KHONG assert gi ca -- CHI goi ham
laSoDuong(5); // dong "return n > 0" DA duoc CHAY QUA
console.log("dong code da chay -- do phu 100% -- nhung KHONG kiem gi ca");
```

```text
dong code da chay -- do phu 100% -- nhung KHONG kiem gi ca
```

Dòng `return n > 0` **THỰC SỰ** đã CHẠY (bất KỲ công cụ đo độ phủ NÀO
cũng báo `100%`) — NHƯNG KHÔNG CÓ `assertEqual` NÀO Ở ĐÂY, nên **KHÔNG
CÓ CÁCH NÀO** phát hiện NẾU kết quả trả VỀ SAI. Độ phủ ĐO "code CÓ
CHẠY", KHÔNG ĐO "kết quả CÓ ĐÚNG".
::::

::::example{#tinh-diem-dot-bien-that}
Điểm đột biến **ĐO ĐÚNG** điều độ PHỦ KHÔNG đo được — CHẠY BA đột
biến KHÁC nhau TRÊN cả HAI phong CÁCH test, ĐẾM XEM BAO NHIÊU đột
biến BỊ GIẾT:

```typescript title=readonly
function laSoDuong(n: number): boolean {
  return n > 0;
}

const dotBienCacHam: Array<(n: number) => boolean> = [
  (n) => n >= 0,  // > thanh >=
  (n) => n > -1,  // hang so 0 thanh -1
  (n) => n > 1,   // hang so 0 thanh 1
];

type BoTest = (ham: (n: number) => boolean) => boolean;

// test A: KHONG assert gi -- CHI goi ham, LUON "qua"
const testKhongAssert: BoTest = (ham) => {
  ham(5);
  return true;
};

// test B: CO assert, kiem CA bien (0) va CA lan can bien (1, -5, 5)
const testCoAssert: BoTest = (ham) => {
  return ham(5) === true && ham(-5) === false && ham(0) === false && ham(1) === true;
};

function tinhDiemDotBien(botest: BoTest): number {
  const soBiGiet = dotBienCacHam.filter((db) => !botest(db)).length;
  return Math.round((soBiGiet / dotBienCacHam.length) * 100);
}

console.log("diem dot bien (test KHONG assert):", tinhDiemDotBien(testKhongAssert), "%");
console.log("diem dot bien (test CO assert):", tinhDiemDotBien(testCoAssert), "%");
```

```text title=readonly
diem dot bien (test KHONG assert): 0 %
diem dot bien (test CO assert): 100 %
```

CẢ HAI phong cách test ĐỀU "CHẠY QUA" `laSoDuong` (độ PHỦ `100%` CHO
CẢ HAI) — NHƯNG điểm đột biến khác NHAU **HOÀN TOÀN**: `testKhongAssert`
KHÔNG bắt được **BẤT KỲ** đột biến nào (`0%` — vô DỤNG NHƯ MỘT test),
`testCoAssert` bắt được **CẢ BA** (`100%` — THẬT SỰ kiểm ĐÚNG hành
vi). Độ phủ KHÔNG phân biệt được HAI phong cách NÀY — điểm đột biến
THÌ CÓ.
::::

::::predict{#doan-test-mot-phan commitOnce}
```typescript
function laSoDuong(n: number): boolean {
  return n > 0;
}
const dotBienCacHam: Array<(n: number) => boolean> = [
  (n) => n >= 0,
  (n) => n > -1,
  (n) => n > 1,
];
type BoTest = (ham: (n: number) => boolean) => boolean;
function tinhDiemDotBien(botest: BoTest): number {
  const soBiGiet = dotBienCacHam.filter((db) => !botest(db)).length;
  return Math.round((soBiGiet / dotBienCacHam.length) * 100);
}

// test CHI kiem MOT gia tri (5), KHONG kiem bien
const testMotPhan: BoTest = (ham) => ham(5) === true;
console.log(tinhDiemDotBien(testMotPhan));
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`33` — vì `testMotPhan` VẪN kiểm được MỘT giá trị (`5`), NÊN ÍT NHẤT
MỘT trong BA đột biến (những đột biến "hoàn toàn khác" TẠI `n=5`) sẽ
BỊ giết — TỈ LỆ `1/3 ≈ 33%`
::why
Gần đúng ở việc bạn nghĩ "kiểm được MỘT giá trị" VẪN NÊN bắt được
MỘT vài đột biến "RÕ RÀNG SAI" — MỘT trực giác hợp lý VỀ mặt tổng
quát của mutation testing.

Chỗ lệch: KIỂM TRA CỤ THỂ CẢ BA đột biến TẠI `n=5`: `n>=0` → `5>=0`
= `true` (KHỚP `laSoDuong(5)=true`). `n>-1` → `5>-1` = `true` (KHỚP).
`n>1` → `5>1` = `true` (KHỚP). **CẢ BA** đột biến ĐỀU cho ĐÚNG cùng
kết quả VỚI bản GỐC TẠI `n=5` — KHÔNG MỘT đột biến NÀO bị PHÂN BIỆT
bởi giá trị `5` (CHÚNG CHỈ khác bản gốc TẠI CÁC giá trị KHÁC: `0`
hoặc ÂM). `testMotPhan` KHÔNG giết được đột biến NÀO — điểm đột biến
LÀ `0`.
::
:::

:::opt
Máy báo lỗi biên dịch — `dotBienCacHam.filter((db) => !botest(db))`
không hợp lệ, vì `filter` đòi HÀM callback PHẢI trả VỀ ĐÚNG kiểu
`boolean`, NHƯNG `!botest(db)` (phủ định của `boolean`) trả VỀ kiểu
`boolean` LỒNG bên trong MỘT phủ định, KHÔNG PHẢI `boolean` trực TIẾP
::why
Gần đúng ở việc bạn nghĩ tới việc `filter` CÓ yêu CẦU kiểu CHO
callback — một quan sát ĐÚNG rằng `filter` XÁC ĐỊNH có ràng buộc kiểu.

Chỗ lệch: `!botest(db)` — toán tử `!` (phủ định logic) áp DỤNG LÊN
MỘT `boolean` (`botest(db)` LÀ `boolean`, vì `BoTest` khai `(ham) =>
boolean`) **LUÔN** trả VỀ MỘT `boolean` **TRỰC TIẾP** (KHÔNG "LỒNG"
gì cả — phủ định MỘT boolean CHO RA MỘT boolean, y HỆT các phép phủ
định ĐÃ dùng xuyên suốt track NÀY). Kiểu KHỚP HOÀN TOÀN với YÊU CẦU
của `filter`. Biên dịch sạch.
::
:::
::::

::::code{#viet_tinhdiemdotbien}
Tự viết `tinhDiemDotBien`.

```typescript title=starter
function laSoDuong(n: number): boolean {
  return n > 0;
}

const dotBienCacHam: Array<(n: number) => boolean> = [
  (n) => n >= 0,
  (n) => n > -1,
  (n) => n > 1,
];

type BoTest = (ham: (n: number) => boolean) => boolean;

function tinhDiemDotBien(botest: BoTest): number {
  const soBiGiet = dotBienCacHam.filter((db) => ___).length;
  return Math.round(___);
}

const testKhongAssert: BoTest = (ham) => { ham(5); return true; };
const diemKhongAssert = tinhDiemDotBien(testKhongAssert);
console.log(diemKhongAssert === 0 ? "[PASS] test khong assert phai co diem 0" : "[FAIL] test khong assert phai co diem 0");
```

```typescript title=solution
function laSoDuong(n: number): boolean {
  return n > 0;
}

const dotBienCacHam: Array<(n: number) => boolean> = [
  (n) => n >= 0,
  (n) => n > -1,
  (n) => n > 1,
];

type BoTest = (ham: (n: number) => boolean) => boolean;

function tinhDiemDotBien(botest: BoTest): number {
  const soBiGiet = dotBienCacHam.filter((db) => !botest(db)).length;
  return Math.round((soBiGiet / dotBienCacHam.length) * 100);
}

const testKhongAssert: BoTest = (ham) => { ham(5); return true; };
const diemKhongAssert = tinhDiemDotBien(testKhongAssert);
console.log(diemKhongAssert === 0 ? "[PASS] test khong assert phai co diem 0" : "[FAIL] test khong assert phai co diem 0");
```

```typescript title=test
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

const testKhongAssert2: BoTest = (ham) => { ham(5); return true; };
assertEqual(tinhDiemDotBien(testKhongAssert2), 0, "test khong assert phai co diem dot bien DUNG BANG 0");

const testCoAssert: BoTest = (ham) => ham(5) === true && ham(-5) === false && ham(0) === false && ham(1) === true;
assertEqual(tinhDiemDotBien(testCoAssert), 100, "test day du phai giet HET, diem DUNG BANG 100");

const testMotPhan: BoTest = (ham) => ham(5) === true;
assertEqual(tinhDiemDotBien(testMotPhan), 0, "test chi kiem mot gia tri khong bien phai co diem 0");
```

:::hints
- kind: attention
  body: "Một đột biến BỊ GIẾT khi nó KHÔNG qua được bo test (botest(db) là false). Điểm đột biến = số bị giết chia cho tổng số đột biến, nhân 100 để ra phần trăm."
- kind: strategy
  body: '!botest(db) : (soBiGiet / dotBienCacHam.length) * 100'
- kind: one-line
  body: '___ (dieu kien giet) = !botest(db)\n___ (cong thuc %) = (soBiGiet / dotBienCacHam.length) * 100'
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
Độ phủ đo CHẠY QUA — điểm đột biến đo KIỂM ĐÚNG. Bước tiếp theo: đột
biến ranh giới — loại phổ biến và quan trọng nhất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Loại đột biến PHỔ BIẾN VÀ QUAN TRỌNG NHẤT — đổi toán tử so sánh sát
nghĩa (`<`↔`<=`, `>`↔`>=`) — chỉ CÁCH nào giết được nó?
::::

::::checkpoint{mastery=0.8}
::::
