---
id: ky-nghe-phan-mem.kiem-thu.san-dot-bien-that
title: "BOSS — Săn đột biến THẬT: thêm test cho tới khi giết HẾT"
summary: "Bài BOSS của track: phanLoaiTuoi(tuoi) với BA ranh giới (<13, <18, còn lại) và một bộ test ban đầu CHỈ có ba ví dụ RÕ RÀNG — trông như 'đủ' nhưng chứa đột biến ranh giới còn SỐNG. Thêm đủ test ca biên để giết HẾT — ghép trọn kỷ luật cả track vào một bài tập cuối cùng."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 34
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kt.gate-boss]
requires: [kt.equivalent-mutants]
concepts: [kt.gate-boss]
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
Bài BOSS của track. `phanLoaiTuoi(tuoi)` — bộ test BAN ĐẦU chỉ có BA
ví dụ RÕ RÀNG, TRÔNG như "đủ". Thêm test tới khi GIẾT HẾT đột biến?
::::

::::explain{#boss-phanloaituoi}
`phanLoaiTuoi(tuoi): "tre_em" | "thieu_nien" | "nguoi_lon"` — BA
nhánh phân LOẠI theo RANH GIỚI `<13`, `<18`, CÒN LẠI. Bộ test BAN
ĐẦU **CHỈ CÓ BA** ví dụ RÕ RÀNG — TRÔNG "đủ" NHƯNG THỰC RA **CHỨA
NHIỀU** đột biến SỐNG SÓT:

```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function phanLoaiTuoi(tuoi: number): "tre_em" | "thieu_nien" | "nguoi_lon" {
  if (tuoi < 13) return "tre_em";
  if (tuoi < 18) return "thieu_nien";
  return "nguoi_lon";
}

// bo test BAN DAU -- CHI ba vi du RO RANG
assertEqual(phanLoaiTuoi(5), "tre_em", "5 tuoi la tre em");
assertEqual(phanLoaiTuoi(15), "thieu_nien", "15 tuoi la thieu nien");
assertEqual(phanLoaiTuoi(30), "nguoi_lon", "30 tuoi la nguoi lon");
```

```text
[PASS] 5 tuoi la tre em
[PASS] 15 tuoi la thieu nien
[PASS] 30 tuoi la nguoi lon
```

BA assertion, BA nhánh — TRÔNG như "phủ đủ" — NHƯNG `5`, `15`, `30`
ĐỀU LÀ giá trị **RÕ RÀNG NẰM GIỮA** MỖI khoảng, KHÔNG PHẢI ranh giới
`13` HAY `18`.
::::

::::example{#hai-dot-bien-song-sot}
CẢ HAI ranh giới (`<13`, `<18`) ĐỀU CÓ đột biến `<`↔`<=` SỐNG SÓT VỚI
bộ test TRÊN:

```typescript title=readonly
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// DOT BIEN 1: tuoi < 13 thanh tuoi <= 13
function phanLoaiTuoiDotBien1(tuoi: number): "tre_em" | "thieu_nien" | "nguoi_lon" {
  if (tuoi <= 13) return "tre_em";
  if (tuoi < 18) return "thieu_nien";
  return "nguoi_lon";
}
try {
  assertEqual(phanLoaiTuoiDotBien1(5), "tre_em", "5 tuoi la tre em");
  assertEqual(phanLoaiTuoiDotBien1(15), "thieu_nien", "15 tuoi la thieu nien");
  assertEqual(phanLoaiTuoiDotBien1(30), "nguoi_lon", "30 tuoi la nguoi lon");
  console.log("DOT BIEN 1 SONG SOT voi bo test ban dau");
} catch (e) {
  console.log("bi giet:", (e as Error).message);
}

// CHI khi THEM ca bien tuoi = 13 moi lo ra
try {
  assertEqual(phanLoaiTuoiDotBien1(13), "thieu_nien", "dung 13 tuoi la thieu nien, khong phai tre em");
} catch (e) {
  console.log("DOT BIEN 1 BI GIET boi ca bien 13:", (e as Error).message);
}
```

```text title=readonly
DOT BIEN 1 SONG SOT voi bo test ban dau
DOT BIEN 1 BI GIET boi ca bien 13: [FAIL] dung 13 tuoi la thieu nien, khong phai tre em: mong thieu_nien, nhan tre_em
```

ĐỘT BIẾN THỨ HAI (`<18` thành `<=18`) HOẠT ĐỘNG **HỆT** vậy — SỐNG
SÓT VỚI bộ test BAN ĐẦU, CHỈ bị GIẾT KHI THÊM test đúng `tuoi = 18`.
HỌC VIÊN cần THÊM **CẢ HAI** ca biên (`13` VÀ `18`) — GHÉP TRỌN kỷ
luật "test ca biên" (bài 29) VÀO bài toán MỚI.
::::

::::predict{#doan-boss-gia-tri-lan-can-bien commitOnce}
```typescript
function phanLoaiTuoi(tuoi: number): "tre_em" | "thieu_nien" | "nguoi_lon" {
  if (tuoi < 13) return "tre_em";
  if (tuoi < 18) return "thieu_nien";
  return "nguoi_lon";
}

console.log(phanLoaiTuoi(13), phanLoaiTuoi(18), phanLoaiTuoi(12), phanLoaiTuoi(17));
```

Dòng cuối in ra gì?

:::opt{correct}
`thieu_nien nguoi_lon tre_em thieu_nien`
:::

:::opt
`tre_em nguoi_lon tre_em thieu_nien` — vì RANH GIỚI `13` (giống MỌI
ranh giới "nhỏ HƠN" TRONG track NÀY) đóng vai TRÒ "điểm CHUYỂN", VÀ
theo QUY ƯỚC thông thường, chính GIÁ TRỊ ranh giới VẪN thuộc NHÓM
"dưới" (giống `dieuChinhKhoangGiaTri`, bài 21, `min` thuộc nhóm HỢP
LỆ)
::why
Gần đúng ở việc bạn liên HỆ tới `dieuChinhKhoangGiaTri` (bài 21) —
MỘT so sánh HỢP LÝ (CẢ HAI đều LÀ hàm phân LOẠI theo NGƯỠNG).

Chỗ lệch: MỖI hàm CÓ **RÀNG BUỘC RIÊNG**, đọc TRỰC TIẾP TỪ code, KHÔNG
suy RA từ hàm KHÁC — Ở ĐÂY, điều kiện LÀ `tuoi < 13` (KHÔNG PHẢI `<=
13`), NÊN `tuoi = 13` **KHÔNG THOẢ** điều kiện NÀY (`13 < 13` LÀ
`false`) — rơi TIẾP xuống `tuoi < 18` (`13 < 18` LÀ `true`) → trả
`"thieu_nien"`. TƯƠNG TỰ, `phanLoaiTuoi(18)`: `18 < 13` false, `18 <
18` false → trả `"nguoi_lon"`. `phanLoaiTuoi(12)`: `12 < 13` true →
`"tre_em"`. `phanLoaiTuoi(17)`: `17 < 13` false, `17 < 18` true →
`"thieu_nien"`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `console.log` VỚI **BỐN** đối số (bốn
lời gọi `phanLoaiTuoi` KHÁC nhau) không hợp lệ, vì `console.log` CHỈ
CHẤP NHẬN TỐI ĐA **BA** đối số TRONG TypeScript
::why
Gần đúng ở việc bạn để ý lời GỌI NÀY CÓ **BỐN** đối số — một quan
sát ĐÚNG về SỐ LƯỢNG.

Chỗ lệch: `console.log` LÀ hàm **BIẾN THIÊN** (variadic — `(...args:
any[]) => void`), CHẤP NHẬN **BẤT KỲ** số lượng đối số NÀO (`0`, `1`,
`4`, `100`, ...), KHÔNG có GIỚI HẠN `3`. Đây LÀ đặc điểm ĐÃ tận DỤNG
NHIỀU lần xuyên suốt TOÀN BỘ dự án (VÍ DỤ MỌI `console.log` in NHIỀU
giá trị CÙNG lúc, phân TÁCH bằng dấu PHẨY). Biên dịch VÀ chạy sạch.
::
:::
::::

::::code{#san_dot_bien_boss}
Cho `phanLoaiTuoi` ĐÃ cài đặt sẵn VÀ bộ test BAN ĐẦU (ba ví dụ). Tự
viết THÊM BỐN assertion ca biên để giết HẾT đột biến ranh giới.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function phanLoaiTuoi(tuoi: number): "tre_em" | "thieu_nien" | "nguoi_lon" {
  if (tuoi < 13) return "tre_em";
  if (tuoi < 18) return "thieu_nien";
  return "nguoi_lon";
}

assertEqual(phanLoaiTuoi(5), "tre_em", "5 tuoi la tre em");
assertEqual(phanLoaiTuoi(15), "thieu_nien", "15 tuoi la thieu nien");
assertEqual(phanLoaiTuoi(30), "nguoi_lon", "30 tuoi la nguoi lon");

const tuoiDuoiBien1 = 12;
const tuoiDungBien1 = 13;
const tuoiDuoiBien2 = 17;
const tuoiDungBien2 = 18;
assertEqual(phanLoaiTuoi(tuoiDuoiBien1), ___, "12 tuoi (ngay duoi bien 13) van la tre em");
assertEqual(phanLoaiTuoi(tuoiDungBien1), ___, "dung 13 tuoi la thieu nien, khong phai tre em");
assertEqual(phanLoaiTuoi(tuoiDuoiBien2), ___, "17 tuoi (ngay duoi bien 18) van la thieu nien");
assertEqual(phanLoaiTuoi(tuoiDungBien2), ___, "dung 18 tuoi la nguoi lon, khong phai thieu nien");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function phanLoaiTuoi(tuoi: number): "tre_em" | "thieu_nien" | "nguoi_lon" {
  if (tuoi < 13) return "tre_em";
  if (tuoi < 18) return "thieu_nien";
  return "nguoi_lon";
}

assertEqual(phanLoaiTuoi(5), "tre_em", "5 tuoi la tre em");
assertEqual(phanLoaiTuoi(15), "thieu_nien", "15 tuoi la thieu nien");
assertEqual(phanLoaiTuoi(30), "nguoi_lon", "30 tuoi la nguoi lon");

const tuoiDuoiBien1 = 12;
const tuoiDungBien1 = 13;
const tuoiDuoiBien2 = 17;
const tuoiDungBien2 = 18;
assertEqual(phanLoaiTuoi(tuoiDuoiBien1), "tre_em", "12 tuoi (ngay duoi bien 13) van la tre em");
assertEqual(phanLoaiTuoi(tuoiDungBien1), "thieu_nien", "dung 13 tuoi la thieu nien, khong phai tre em");
assertEqual(phanLoaiTuoi(tuoiDuoiBien2), "thieu_nien", "17 tuoi (ngay duoi bien 18) van la thieu nien");
assertEqual(phanLoaiTuoi(tuoiDungBien2), "nguoi_lon", "dung 18 tuoi la nguoi lon, khong phai thieu nien");
```

```typescript title=test
// mo phong CA HAI dot bien ranh gioi -- neu blank sai, cac dong nay se PHAT HIEN
function phanLoaiTuoiDotBien1(tuoi: number): "tre_em" | "thieu_nien" | "nguoi_lon" {
  if (tuoi <= 13) return "tre_em"; // < thanh <=
  if (tuoi < 18) return "thieu_nien";
  return "nguoi_lon";
}
function phanLoaiTuoiDotBien2(tuoi: number): "tre_em" | "thieu_nien" | "nguoi_lon" {
  if (tuoi < 13) return "tre_em";
  if (tuoi <= 18) return "thieu_nien"; // < thanh <=
  return "nguoi_lon";
}

let gietDotBien1 = false;
try {
  assertEqual(phanLoaiTuoiDotBien1(13), "thieu_nien", "kiem dot bien ranh gioi 13");
} catch {
  gietDotBien1 = true;
}
if (!gietDotBien1) throw new Error("bo test PHAI giet duoc dot bien ranh gioi 13 (< thanh <=)");

let gietDotBien2 = false;
try {
  assertEqual(phanLoaiTuoiDotBien2(18), "nguoi_lon", "kiem dot bien ranh gioi 18");
} catch {
  gietDotBien2 = true;
}
if (!gietDotBien2) throw new Error("bo test PHAI giet duoc dot bien ranh gioi 18 (< thanh <=)");

console.log("[PASS] bo test giet duoc CA HAI dot bien ranh gioi -- SAN DOT BIEN THANH CONG");
```

:::hints
- kind: attention
  body: "Bốn giá trị cần kiểm: 12 (ngay dưới biên 13, vẫn tre_em), 13 (đúng biên, đã sang thieu_nien), 17 (ngay dưới biên 18, vẫn thieu_nien), 18 (đúng biên, đã sang nguoi_lon)."
- kind: strategy
  body: '"tre_em" : "thieu_nien" : "thieu_nien" : "nguoi_lon" — theo đúng thứ tự bốn giá trị 12, 13, 17, 18.'
- kind: one-line
  body: '___ (12) = "tre_em"\n___ (13) = "thieu_nien"\n___ (17) = "thieu_nien"\n___ (18) = "nguoi_lon"'
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
BOSS đánh bại. Track TDD→Property→Mutation hoàn tất: chu trình Đỏ-
Xanh-Tinh gọn, test hàm thuần, FP DI, bốn mẫu tính chất, shrinking,
mutation testing — trọn vẹn từ nền tảng tới kỷ luật chấm điểm thật
của chính dự án này.
::::

::::reflect{#nghi-lai}
Track TDD→Property→Mutation đã hoàn tất — từ Đỏ-Xanh-Tinh gọn, test
hàm thuần và máy trạng thái, FP DI, Arrange-Act-Assert, property-based
testing với bốn mẫu tính chất, tự viết shrinking, tới mutation testing
— chính kỷ luật "chấm trượt được đáp án sai" mà dự án này đã áp dụng
xuyên suốt, giờ học viên đã tự tay thực hành nó.
::::

::::checkpoint{mastery=0.85}
::::
