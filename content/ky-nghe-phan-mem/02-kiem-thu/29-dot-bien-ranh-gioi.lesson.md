---
id: ky-nghe-phan-mem.kiem-thu.dot-bien-ranh-gioi
title: "Đột biến RANH GIỚI — < thành <=, > thành >=, off-by-one"
summary: "Loại đột biến phổ biến và quan trọng nhất: đổi toán tử so sánh sát nghĩa (<↔<=, >↔>=, ===↔!==). CHỈ có test ở ĐÚNG ngưỡng (giá trị bằng ranh giới) và ngưỡng±1 mới giết được loại đột biến này — chính xác kỷ luật 'test ca biên' đã áp dụng xuyên suốt dự án."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 29
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.mutation-boundary]
requires: [kt.coverage-vs-mutation-score]
concepts: [kt.mutation-boundary]
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
Loại đột biến PHỔ BIẾN VÀ QUAN TRỌNG NHẤT — đổi toán tử so sánh sát
nghĩa (`<`↔`<=`, `>`↔`>=`) — chỉ CÁCH nào giết được nó?
::::

::::explain{#dot-bien-ranh-gioi-pho-bien-nhat}
Loại đột biến **PHỔ BIẾN VÀ QUAN TRỌNG NHẤT**: đổi toán tử so sánh
**SÁT NGHĨA** (`<`↔`<=`, `>`↔`>=`, `===`↔`!==`). **CHỈ CÓ** test Ở
**ĐÚNG NGƯỠNG** (giá trị BẰNG ranh giới) MỚI GIẾT được loại đột biến
NÀY — CHÍNH XÁC kỷ luật "test ca biên" ĐÃ ÁP DỤNG XUYÊN SUỐT dự án:

```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function trongPhamVi(n: number, min: number, max: number): boolean {
  return n >= min && n <= max;
}

// bo test THIEU ca bien -- chi thu gia tri "ro rang trong"/"ro rang ngoai"
assertEqual(trongPhamVi(15, 10, 20), true, "gia tri giua khoang");
assertEqual(trongPhamVi(5, 10, 20), false, "gia tri duoi khoang");
assertEqual(trongPhamVi(25, 10, 20), false, "gia tri tren khoang");
```

```text
[PASS] gia tri giua khoang
[PASS] gia tri duoi khoang
[PASS] gia tri tren khoang
```

Bộ test NÀY TRÔNG "kỹ lưỡng" (BA trường hợp: trong, dưới, TRÊN
khoảng) — NHƯNG KHÔNG CÓ trường hợp NÀO test ĐÚNG `n = min` hay `n =
max`.
::::

::::example{#dot-bien-song-sot-o-bien}
Đổi `>=` thành `>` Ở BIÊN `min` (đột biến "off-by-one") RỒI chạy LẠI
**ĐÚNG** bộ test TRÊN — VẪN `[PASS]`:

```typescript title=readonly
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// DOT BIEN: >= thanh > o bien min
function trongPhamViDotBien(n: number, min: number, max: number): boolean {
  return n > min && n <= max;
}

try {
  assertEqual(trongPhamViDotBien(15, 10, 20), true, "gia tri giua khoang");
  assertEqual(trongPhamViDotBien(5, 10, 20), false, "gia tri duoi khoang");
  assertEqual(trongPhamViDotBien(25, 10, 20), false, "gia tri tren khoang");
  console.log("DOT BIEN SONG SOT voi bo test cu");
} catch (e) {
  console.log("DOT BIEN BI GIET:", (e as Error).message);
}

// CHI khi THEM test dung tai n = min moi lo ra
try {
  assertEqual(trongPhamViDotBien(10, 10, 20), true, "dung bang min phai trong pham vi");
} catch (e) {
  console.log("DOT BIEN BI GIET boi ca bien min:", (e as Error).message);
}
```

```text title=readonly
DOT BIEN SONG SOT voi bo test cu
DOT BIEN BI GIET boi ca bien min: [FAIL] dung bang min phai trong pham vi: mong true, nhan false
```

`15`, `5`, `25` **KHÔNG PHẢI** ranh giới — CẢ `>=` LẪN `>` cho CÙNG
kết quả TẠI ba giá trị NÀY. CHỈ TẠI ĐÚNG `n = 10` (chính XÁC bằng
`min`) HAI toán tử MỚI khác NHAU: `10 >= 10` LÀ `true`, `10 > 10` LÀ
`false` — đây LÀ điểm DUY NHẤT phân biệt được ĐỘT BIẾN.
::::

::::predict{#doan-dot-bien-o-bien-max commitOnce}
```typescript
function trongPhamViDotBienMax(n: number, min: number, max: number): boolean {
  return n >= min && n < max; // <= thanh <
}

console.log(trongPhamViDotBienMax(20, 10, 20));
```

Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì `n = 20`, `min = 10`, `max = 20` — `20` NẰM **TRONG**
khoảng `[10, 20]` VỀ mặt NGHIỆP VỤ (bao gồm CẢ hai đầu), NÊN
`trongPhamViDotBienMax` (dù CÓ đột biến) VẪN PHẢI trả `true` để ĐÚNG
với Ý NGHĨA "trong phạm VI"
::why
Gần đúng ở việc bạn nhớ Ý NGHĨA "trong phạm VI" (bao gồm CẢ hai đầu
`min`/`max`) — MỘT hiểu biết ĐÚNG về Ý ĐỒ nghiệp VỤ của `trongPhamVi`
GỐC (KHÔNG bị đột biến).

Chỗ lệch: `trongPhamViDotBienMax` LÀ PHIÊN BẢN **ĐÃ BỊ ĐỘT BIẾN**
(`<=` ĐỔI thành `<`) — nó KHÔNG CÒN thực hiện ĐÚNG ý ĐỒ nghiệp vụ
NỮA (ĐÓ CHÍNH LÀ Ý NGHĨA của "đột biến": MỘT LỖI được CỐ Ý gieo VÀO).
`20 >= 10` LÀ `true`, NHƯNG `20 < 20` LÀ `false` — `true && false`
LÀ `false`. Hàm ĐỘT BIẾN trả `false` TẠI ĐÚNG `n = max`, SAI với ý
nghiệp vụ THẬT — đây CHÍNH LÀ loại LỖI mutation testing tồn TẠI để
PHÁT HIỆN (nếu bộ test có ca `n = max`, NÓ SẼ bắt được LỖI NÀY).
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `trongPhamViDotBienMax(20, 10, 20)`
không hợp lệ, vì `min` VÀ `max` được TRUYỀN CÙNG khoảng CÁCH TỪ `n`
(`n - min = 10`, `max - n = 0`), TypeScript đòi HAI khoảng CÁCH NÀY
PHẢI CHÊNH LỆCH
::why
Gần đúng ở việc bạn nghĩ tới MỐI QUAN HỆ SỐ HỌC giữa `n`, `min`,
`max` — MỘT trực giác về việc CÓ THỂ tồn tại RÀNG BUỘC giữa các đối
số.

Chỗ lệch: TypeScript KHÔNG kiểm tra MỐI QUAN HỆ SỐ HỌC giữa các tham
số CÙNG kiểu `number` — CHỈ kiểm KIỂU (`number`), KHÔNG kiểm GIÁ TRỊ
cụ thể (giống MỌI hàm nhận nhiều tham số `number` ĐÃ gặp xuyên suốt
track NÀY, ví dụ `dieuChinhKhoangGiaTri` Ở bài 21). Biên dịch sạch.
::
:::
::::

::::code{#viet_test_ranh_gioi}
Cho `trongPhamVi` ĐÃ cài đặt sẵn. Tự viết THÊM assertion CA BIÊN để
giết đột biến `>=`↔`>` VÀ `<=`↔`<`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function trongPhamVi(n: number, min: number, max: number): boolean {
  return n >= min && n <= max;
}

assertEqual(trongPhamVi(15, 10, 20), true, "gia tri giua khoang");
assertEqual(trongPhamVi(5, 10, 20), false, "gia tri duoi khoang");
assertEqual(trongPhamVi(25, 10, 20), false, "gia tri tren khoang");
assertEqual(trongPhamVi(___, 10, 20), true, "dung bang min phai trong pham vi");
assertEqual(trongPhamVi(___, 10, 20), true, "dung bang max phai trong pham vi");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function trongPhamVi(n: number, min: number, max: number): boolean {
  return n >= min && n <= max;
}

assertEqual(trongPhamVi(15, 10, 20), true, "gia tri giua khoang");
assertEqual(trongPhamVi(5, 10, 20), false, "gia tri duoi khoang");
assertEqual(trongPhamVi(25, 10, 20), false, "gia tri tren khoang");
assertEqual(trongPhamVi(10, 10, 20), true, "dung bang min phai trong pham vi");
assertEqual(trongPhamVi(20, 10, 20), true, "dung bang max phai trong pham vi");
```

```typescript title=test
// mo phong CHINH hai dot bien -- neu blank sai, cac dong nay se PHAT HIEN
function trongPhamViDotBienMin(n: number, min: number, max: number): boolean {
  return n > min && n <= max; // >= thanh >
}
function trongPhamViDotBienMax(n: number, min: number, max: number): boolean {
  return n >= min && n < max; // <= thanh <
}

let gietMin = false;
try {
  assertEqual(trongPhamViDotBienMin(10, 10, 20), true, "kiem dot bien >= o bien min");
} catch {
  gietMin = true;
}
if (!gietMin) throw new Error("bo test PHAI giet duoc dot bien >=-thanh-> o bien min (can assertion tai n=min)");

let gietMax = false;
try {
  assertEqual(trongPhamViDotBienMax(20, 10, 20), true, "kiem dot bien <= o bien max");
} catch {
  gietMax = true;
}
if (!gietMax) throw new Error("bo test PHAI giet duoc dot bien <=-thanh-< o bien max (can assertion tai n=max)");

console.log("[PASS] bo test giet duoc ca hai dot bien ranh gioi");
```

:::hints
- kind: attention
  body: "Để phân biệt >= với >, và <= với <, cần gọi trongPhamVi với n ĐÚNG BẰNG min và ĐÚNG BẰNG max — không phải giá trị gần đó."
- kind: strategy
  body: '10 (đúng bằng min) : 20 (đúng bằng max)'
- kind: one-line
  body: '___ (1) = 10\n___ (2) = 20'
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
Đột biến ranh giới: chỉ test đúng ngưỡng mới giết được. Bước tiếp
theo: đột biến số học — cộng thành trừ, nhân thành chia.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đổi `+` thành `-` trong một hàm tính tuổi (`gioHienTai - ngaySinh`)
— với giá trị NHỎ, kết quả đột biến ĐÔI KHI "tình cờ" giống kết quả
ĐÚNG. Test thế nào để bắt được LOẠI đột biến này?
::::

::::checkpoint{mastery=0.8}
::::
