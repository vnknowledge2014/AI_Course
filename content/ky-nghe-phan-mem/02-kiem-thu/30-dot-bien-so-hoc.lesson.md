---
id: ky-nghe-phan-mem.kiem-thu.dot-bien-so-hoc
title: "Đột biến SỐ HỌC — cộng thành trừ, nhân thành chia"
summary: "Đổi toán tử số học (+↔-, *↔/) — nguy hiểm vì với test LỎNG LẺO (chỉ kiểm dấu/khoảng, không kiểm giá trị chính xác), kết quả đột biến có thể 'tình cờ' vẫn qua. Giết loại đột biến này cần test GIÁ TRỊ CHÍNH XÁC với GIÁ TRỊ TUYỆT ĐỐI LỚN và HIỆU SỐ NHỎ."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 30
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.mutation-arithmetic]
requires: [kt.mutation-boundary]
concepts: [kt.mutation-arithmetic]
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
Đổi `+` thành `-` trong hàm tính tuổi (`gioHienTai - ngaySinh`) — kết
quả đột biến ĐÔI KHI "tình cờ" vẫn QUA test. Test thế nào để bắt được?
::::

::::explain{#dot-bien-so-hoc-nguy-hiem}
Đổi toán TỬ số học (`+`↔`-`, `*`↔`/`) **NGUY HIỂM** vì VỚI test
**LỎNG LẺO** (CHỈ kiểm DẤU/khoảng, KHÔNG kiểm GIÁ TRỊ chính XÁC), kết
quả đột biến CÓ THỂ "tình cờ" **VẪN QUA**:

```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function tinhTuoi(namHienTai: number, namSinh: number): number {
  return namHienTai - namSinh;
}

// bo test LONG LEO -- CHI kiem DAU (duong/am), KHONG kiem GIA TRI chinh xac
assertEqual(tinhTuoi(5, 3) > 0, true, "tuoi phai la so duong");
```

```text
[PASS] tuoi phai la so duong
```

Test NÀY TRÔNG hợp lý ("tuổi PHẢI dương") — NHƯNG chỉ kiểm **DẤU**,
KHÔNG kiểm **GIÁ TRỊ CHÍNH XÁC**. `tinhTuoi(5, 3)` trả `2` — `2 > 0`
LÀ `true`, test `[PASS]`.
::::

::::example{#dot-bien-song-sot-test-long-leo}
Đổi `-` thành `+` (đột biến số HỌC) — CHẠY LẠI **ĐÚNG** test lỏng
LẺO TRÊN — VẪN `[PASS]`, VÌ CẢ HAI kết QUẢ đều DƯƠNG:

```typescript title=readonly
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// DOT BIEN: - thanh +
function tinhTuoiDotBien(namHienTai: number, namSinh: number): number {
  return namHienTai + namSinh;
}

try {
  assertEqual(tinhTuoiDotBien(5, 3) > 0, true, "tuoi phai la so duong");
  console.log("DOT BIEN SONG SOT -- ca hai deu > 0:", tinhTuoiDotBien(5, 3));
} catch (e) {
  console.log("bi giet:", (e as Error).message);
}

// test CHINH XAC, GIA TRI TUYET DOI LON, HIEU SO NHO
try {
  assertEqual(tinhTuoiDotBien(2024, 2020), 4, "tuoi chinh xac tu nam lon");
} catch (e) {
  console.log("DOT BIEN BI GIET boi test chinh xac gia tri lon:", (e as Error).message);
}
```

```text title=readonly
DOT BIEN SONG SOT -- ca hai deu > 0: 8
DOT BIEN BI GIET boi test chinh xac gia tri lon: [FAIL] tuoi chinh xac tu nam lon: mong 4, nhan 4044
```

`tinhTuoiDotBien(5, 3)` = `5 + 3 = 8` — **VẪN DƯƠNG**, giống HỆT bản
GỐC (`5 - 3 = 2`, CŨNG dương) — test "CHỈ kiểm dấu" **KHÔNG PHÂN
BIỆT** được `2` VÀ `8`. NHƯNG VỚI `namHienTai = 2024`, `namSinh =
2020` (GIÁ TRỊ TUYỆT ĐỐI LỚN, HIỆU SỐ NHỎ) VÀ kiểm **GIÁ TRỊ CHÍNH
XÁC**: bản GỐC ra `4` (ĐÚNG — tuổi NHỎ, hợp LÝ), đột biến ra `4044`
(**SAI LỆCH HẲN** — MỘT con số VÔ LÝ cho "tuổi") — KHÔNG THỂ nhầm
lẫn.
::::

::::predict{#doan-dot-bien-nam-sinh-khac commitOnce}
```typescript
function tinhTuoiDotBien(namHienTai: number, namSinh: number): number {
  return namHienTai + namSinh;
}

console.log(tinhTuoiDotBien(2024, 1990));
```

Dòng cuối in ra gì?

:::opt{correct}
`4014`
:::

:::opt
`34` — vì `tinhTuoiDotBien` VẪN tính TUỔI (dù bị đột biến), VÀ `2024
- 1990 = 34` LÀ TUỔI ĐÚNG NGHIỆP VỤ — hàm SẼ VẪN cho ra CON SỐ HỢP
LÝ NÀY, CHỈ khác Ở CÁCH TÍNH bên TRONG
::why
Gần đúng ở việc bạn nhớ `34` LÀ TUỔI ĐÚNG (nghiệp vụ THẬT) của MỘT
người sinh năm `1990` VÀO năm `2024` — MỘT hiểu biết ĐÚNG về Ý ĐỒ
nghiệp vụ.

Chỗ lệch: `tinhTuoiDotBien` LÀ hàm **ĐÃ BỊ ĐỘT BIẾN** — THÂN hàm
THỰC SỰ LÀ `return namHienTai + namSinh;` (PHÉP CỘNG, KHÔNG PHẢI
TRỪ). Hàm KHÔNG "tự biết" tính ĐÚNG Ý ĐỒ nghiệp vụ — nó CHỈ chạy
ĐÚNG CÁI CÔNG THỨC ĐÃ VIẾT (dù SAI). `2024 + 1990 = 4014` — MỘT con
số VÔ LÝ cho "tuổi" (KHÔNG AI `4014` tuổi), NHƯNG đó CHÍNH XÁC LÀ
những GÌ hàm ĐỘT BIẾN trả VỀ. Đây LÀ chính LÝ DO mutation testing tồn
TẠI: PHÁT HIỆN những LỖI như thế NÀY BẰNG cách kiểm GIÁ TRỊ CHÍNH
XÁC, KHÔNG dựa VÀO "trông có Vẻ hợp lý".
::
:::

:::opt
Máy báo lỗi biên dịch — `tinhTuoiDotBien(2024, 1990)` không hợp lệ,
vì TÊN hàm CHỨA CHỮ "DotBien" (đột biến), TypeScript CẤM đặt TÊN hàm
gợi ý "đây LÀ MỘT phiên bản LỖI"
::why
Gần đúng ở việc bạn để ý TÊN hàm `tinhTuoiDotBien` KHÁC BIỆT rõ RỆT
so VỚI `tinhTuoi` — MỘT quan sát ĐÚNG rằng TÊN có Ý NGHĨA MÔ TẢ.

Chỗ lệch: TypeScript (VÀ JavaScript) KHÔNG hề GÁN Ý NGHĨA ĐẶC BIỆT
cho BẤT KỲ CHUỖI ký TỰ NÀO xuất hiện TRONG tên định danh (identifier)
— TÊN hàm CHỈ LÀ MỘT nhãn, dùng để THAM CHIẾU, KHÔNG ảnh HƯỞNG gì
tới việc BIÊN dịch có THÀNH CÔNG hay KHÔNG. `tinhTuoiDotBien` LÀ MỘT
tên định danh HOÀN TOÀN hợp lệ, giống BẤT KỲ tên NÀO khác. Biên dịch
sạch.
::
:::
::::

::::code{#viet_test_dot_bien_so_hoc}
Cho `tinhTuoi` ĐÃ cài đặt sẵn. Tự viết THÊM assertion GIÁ TRỊ TUYỆT
ĐỐI LỚN, HIỆU SỐ NHỎ để giết đột biến `-`↔`+`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function tinhTuoi(namHienTai: number, namSinh: number): number {
  return namHienTai - namSinh;
}

assertEqual(tinhTuoi(5, 3) > 0, true, "tuoi phai la so duong");
assertEqual(tinhTuoi(2024, 1990), ___, "tuoi chinh xac tu gia tri tuyet doi lon, hieu so nho");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function tinhTuoi(namHienTai: number, namSinh: number): number {
  return namHienTai - namSinh;
}

assertEqual(tinhTuoi(5, 3) > 0, true, "tuoi phai la so duong");
assertEqual(tinhTuoi(2024, 1990), 34, "tuoi chinh xac tu gia tri tuyet doi lon, hieu so nho");
```

```typescript title=test
// mo phong CHINH dot bien -- so + thanh + -- neu blank sai, dong nay se PHAT HIEN
function tinhTuoiDotBien(namHienTai: number, namSinh: number): number {
  return namHienTai + namSinh;
}
let gietDotBien = false;
try {
  assertEqual(tinhTuoiDotBien(2024, 1990), 34, "kiem dot bien so hoc tren gia tri lon");
} catch {
  gietDotBien = true;
}
if (!gietDotBien) throw new Error("bo test PHAI giet duoc dot bien -+ (can gia tri CHINH XAC, tuyet doi lon, hieu so nho)");
console.log("[PASS] bo test giet duoc dot bien so hoc");
```

:::hints
- kind: attention
  body: "Để phân biệt - với +, cần kiểm GIÁ TRỊ CHÍNH XÁC (không chỉ dấu) TẠI hai số lớn với hiệu số nhỏ: 2024 - 1990 = 34."
- kind: strategy
  body: '34 — kết quả CHÍNH XÁC của phép trừ 2024 - 1990.'
- kind: one-line
  body: '___ = 34'
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
Đột biến số học: giá trị tuyệt đối lớn, hiệu số nhỏ, kiểm CHÍNH XÁC
không chỉ kiểm dấu. Bước tiếp theo: đột biến logic — && thành ||.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`duocPhepMua(coTien, duTuoi)` trả `coTien && duTuoi` — đổi `&&` thành
`||` — bộ test CHỈ thử (true,true)/(false,false) có giết được đột
biến đó không?
::::

::::checkpoint{mastery=0.8}
::::
