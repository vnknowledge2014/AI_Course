---
id: ky-nghe-phan-mem.kiem-thu.dot-bien-hoan-doi-gia-tri
title: "Đột biến HOÁN ĐỔI giá trị — field/hằng số cùng kiểu bị đổi chỗ"
summary: "Đột biến hoán đổi HAI giá trị CÙNG KIỂU (hai field cùng string, hai chuỗi hardcode giữa hai hàm khác nhau) — giết được cần test kiểm GIÁ TRỊ RIÊNG của MỖI field/hằng số bằng DỮ LIỆU PHÂN BIỆT RÕ, không dùng giá trị GIỐNG NHAU cho hai vị trí lẽ ra PHẢI khác biệt được."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 32
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.mutation-value-swap]
requires: [kt.mutation-logical]
concepts: [kt.mutation-value-swap]
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
Một hàm ánh xạ hoán đổi hai field cùng kiểu string (`ma`↔`ten`) —
test chỉ kiểm MỘT field (KHÔNG PHẢI hai field đó) có bắt được không?
::::

::::explain{#dot-bien-hoan-doi-gia-tri}
Đột biến **HOÁN ĐỔI** HAI giá trị **CÙNG KIỂU** (hai field CÙNG
string, hai chuỗi hardcode GIỮA hai hàm khác nhau) — GIẾT được CẦN
test kiểm **GIÁ TRỊ RIÊNG** của MỖI field bằng **DỮ LIỆU PHÂN BIỆT
RÕ**:

```typescript
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type SanPham = { ma: string; ten: string; gia: number };

function sanPhamSangDto(sp: SanPham): SanPham {
  return { ma: sp.ma, ten: sp.ten, gia: sp.gia };
}

const sp: SanPham = { ma: "SP001", ten: "Ao thun", gia: 150000 };
const dto = sanPhamSangDto(sp);

// bo test CHI kiem "gia" -- KHONG kiem ma/ten
assertEqual(dto.gia, 150000, "gia phai giu nguyen");
```

```text
[PASS] gia phai giu nguyen
```

Bộ test NÀY kiểm được `gia` (MỘT field SỐ) — NHƯNG HOÀN TOÀN KHÔNG
kiểm `ma`/`ten` (HAI field CÙNG kiểu `string`, VỊ TRÍ dễ bị **HOÁN
ĐỔI** nhất KHI viết SAI mapper).
::::

::::example{#dot-bien-song-sot-hoan-doi}
Đột biến **HOÁN ĐỔI** `ma`↔`ten` RỒI chạy LẠI **ĐÚNG** test TRÊN —
VẪN `[PASS]` (VÌ `gia` KHÔNG bị ẢNH HƯỞNG BỞI hoán đổi):

```typescript title=readonly
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type SanPham = { ma: string; ten: string; gia: number };
const sp: SanPham = { ma: "SP001", ten: "Ao thun", gia: 150000 };

// DOT BIEN: hoan doi ma/ten
function sanPhamSangDtoDotBien(sp: SanPham): SanPham {
  return { ma: sp.ten, ten: sp.ma, gia: sp.gia };
}
const dtoDotBien = sanPhamSangDtoDotBien(sp);

try {
  assertEqual(dtoDotBien.gia, 150000, "gia phai giu nguyen");
  console.log("DOT BIEN SONG SOT -- test gia van qua");
} catch (e) {
  console.log("bi giet:", (e as Error).message);
}

// CHI khi kiem RIENG ma va ten moi lo ra
try {
  assertEqual(dtoDotBien.ma, "SP001", "ma phai giu dung ma");
} catch (e) {
  console.log("DOT BIEN BI GIET boi kiem rieng ma:", (e as Error).message);
}
```

```text title=readonly
DOT BIEN SONG SOT -- test gia van qua
DOT BIEN BI GIET boi kiem rieng ma: [FAIL] ma phai giu dung ma: mong SP001, nhan Ao thun
```

`dtoDotBien.ma` NHẬN nhầm giá trị của `ten` (`"Ao thun"`) THAY VÌ
`ma` THẬT (`"SP001"`) — CHỈ khi assertion kiểm **ĐÚNG** field `ma`
(VỚI giá trị `"SP001"` **PHÂN BIỆT RÕ** với `"Ao thun"`) MỚI lộ ra
LỖI.
::::

::::predict{#doan-hoan-doi-hang-so-giua-ham commitOnce}
```typescript
function thongDiepThanhCongDotBien(): string {
  return "That bai"; // DOT BIEN: hoan doi voi noi dung cua ham duoi
}
function thongDiepThatBaiDotBien(): string {
  return "Thanh cong";
}

console.log(thongDiepThanhCongDotBien(), thongDiepThatBaiDotBien());
```

Dòng cuối in ra gì?

:::opt{correct}
`That bai Thanh cong`
:::

:::opt
`Thanh cong That bai` — vì TÊN hàm `thongDiepThanhCongDotBien` VẪN
GỢI Ý rõ "đây LÀ thông điệp THÀNH CÔNG" — TypeScript/JavaScript sẽ
ưu TIÊN Ý NGHĨA của TÊN hàm HƠN nội dung `return` bên trong KHI CÓ
mâu thuẫn
::why
Gần đúng ở việc bạn để ý TÊN hàm `thongDiepThanhCongDotBien` VẪN
"nói lên" Ý ĐỊNH BAN ĐẦU (thông điệp thành CÔNG) — một quan sát ĐÚNG
rằng TÊN hàm CHƯA bị đổi.

Chỗ lệch: JavaScript/TypeScript **KHÔNG BAO GIỜ** "đọc hiểu" Ý NGHĨA
của TÊN định danh (identifier) — TÊN hàm CHỈ LÀ MỘT NHÃN để THAM
CHIẾU, HOÀN TOÀN KHÔNG ảnh hưởng tới GIÁ TRỊ hàm ĐÓ THỰC SỰ trả VỀ.
Hàm LUÔN trả VỀ **CHÍNH XÁC** những GÌ nằm SAU `return` — ĐÂY CHÍNH
LÀ bản CHẤT của đột biến "HOÁN ĐỔI hằng số HARDCODE giữa hai hàm":
TÊN hàm KHÔNG đổi (VẪN "gợi ý" đúng ý NGHĨA GỐC), NHƯNG NỘI DUNG BÊN
TRONG đã bị HOÁN ĐỔI — MỘT loại LỖI ĐẶC BIỆT KHÓ nhận RA bằng MẮT vì
TÊN hàm "trông vẫn ĐÚNG".
::
:::

:::opt
Máy báo lỗi biên dịch — HAI hàm `thongDiepThanhCongDotBien`/
`thongDiepThatBaiDotBien` trả VỀ **CÙNG KIỂU** `string` NHƯNG NỘI
DUNG "hoán đổi nhau", TypeScript đòi TÊN hàm PHẢI khớp NỘI DUNG `return`
::why
Gần đúng ở việc bạn để ý HAI hàm NÀY trả VỀ NỘI DUNG "TRÁI NGƯỢC"
với TÊN của CHÚNG — một quan sát ĐÚNG rằng CÓ SỰ mâu thuẫn NGỮ NGHĨA.

Chỗ lệch: TypeScript **CHỈ** kiểm tra KIỂU (`string`), KHÔNG "hiểu"
VÀ CÀNG KHÔNG kiểm tra **NỘI DUNG** chuỗi CÓ "khớp" VỚI Ý NGHĨA TÊN
hàm HAY KHÔNG (KHÔNG CÓ CÁCH NÀO để MÁY "hiểu" Ý NGHĨA ngôn NGỮ tự
nhiên của MỘT tên định danh). CẢ HAI hàm ĐỀU khai `(): string` VÀ
ĐỀU trả VỀ MỘT `string` — khớp HOÀN TOÀN. Biên dịch sạch.
::
:::
::::

::::code{#viet_test_hoan_doi}
Cho `sanPhamSangDto` ĐÃ cài đặt sẵn. Tự viết THÊM assertion kiểm
RIÊNG `ma` VÀ `ten` để giết đột biến HOÁN ĐỔI.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type SanPham = { ma: string; ten: string; gia: number };

function sanPhamSangDto(sp: SanPham): SanPham {
  return { ma: sp.ma, ten: sp.ten, gia: sp.gia };
}

const sp: SanPham = { ma: "SP001", ten: "Ao thun", gia: 150000 };
const dto = sanPhamSangDto(sp);

assertEqual(dto.gia, 150000, "gia phai giu nguyen");
assertEqual(dto.ma, ___, "ma phai giu dung ma, khong doi cho voi ten");
assertEqual(dto.ten, ___, "ten phai giu dung ten, khong doi cho voi ma");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type SanPham = { ma: string; ten: string; gia: number };

function sanPhamSangDto(sp: SanPham): SanPham {
  return { ma: sp.ma, ten: sp.ten, gia: sp.gia };
}

const sp: SanPham = { ma: "SP001", ten: "Ao thun", gia: 150000 };
const dto = sanPhamSangDto(sp);

assertEqual(dto.gia, 150000, "gia phai giu nguyen");
assertEqual(dto.ma, "SP001", "ma phai giu dung ma, khong doi cho voi ten");
assertEqual(dto.ten, "Ao thun", "ten phai giu dung ten, khong doi cho voi ma");
```

```typescript title=test
// mo phong CHINH dot bien hoan doi -- neu blank sai, dong nay se PHAT HIEN
function sanPhamSangDtoDotBien(spx: SanPham): SanPham {
  return { ma: spx.ten, ten: spx.ma, gia: spx.gia };
}
const dtoDotBien = sanPhamSangDtoDotBien(sp);

let gietMa = false;
try {
  assertEqual(dtoDotBien.ma, "SP001", "kiem dot bien hoan doi tren field ma");
} catch {
  gietMa = true;
}
if (!gietMa) throw new Error("bo test PHAI giet duoc dot bien hoan doi ma/ten (can kiem RIENG field ma)");

let gietTen = false;
try {
  assertEqual(dtoDotBien.ten, "Ao thun", "kiem dot bien hoan doi tren field ten");
} catch {
  gietTen = true;
}
if (!gietTen) throw new Error("bo test PHAI giet duoc dot bien hoan doi ma/ten (can kiem RIENG field ten)");

console.log("[PASS] bo test giet duoc dot bien hoan doi gia tri");
```

:::hints
- kind: attention
  body: "sp.ma = \"SP001\", sp.ten = \"Ao thun\" — hai giá trị PHÂN BIỆT RÕ. dto.ma phải giữ đúng ma gốc, dto.ten phải giữ đúng ten gốc."
- kind: strategy
  body: '"SP001" : "Ao thun" — đúng giá trị gốc của từng field, không đổi chỗ.'
- kind: one-line
  body: '___ (1) = "SP001"\n___ (2) = "Ao thun"'
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
Đột biến hoán đổi: kiểm RIÊNG từng field bằng dữ liệu PHÂN BIỆT RÕ.
Bước tiếp theo: giới hạn thành thật của mutation testing — đột biến
TƯƠNG ĐƯƠNG.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`n * 2` đổi thành `n + n` — về mặt TOÁN HỌC, GIỐNG HỆT nhau với MỌI
`n`. Có test NÀO (dù viết tốt tới đâu) giết được đột biến NÀY không?
::::

::::checkpoint{mastery=0.8}
::::
