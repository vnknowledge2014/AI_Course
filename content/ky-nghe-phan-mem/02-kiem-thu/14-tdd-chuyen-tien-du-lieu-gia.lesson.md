---
id: ky-nghe-phan-mem.kiem-thu.tdd-chuyen-tien-du-lieu-gia
title: "Capstone: TDD chuyển tiền — FP DI + fake + AAA hợp lại"
summary: "Bài chốt cụm 3: chuyenTien(kho, tuId, denId, soTien): Result<{tu,den}, LoiChuyenTien> — Map làm kho fake ĐỒNG BỘ, cấu trúc theo Arrange-Act-Assert, mọi assertion đặt tên 'nên X khi Y', phủ đủ đường thành công + ba nhánh lỗi — ghép trọn FP DI, nhà máy fake, AAA/đặt tên vào MỘT use case thật."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kt.gate-boss-fp-di]
requires: [kt.aaa-naming]
concepts: [kt.gate-boss-fp-di]
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
Bài chốt cụm 3. `chuyenTien` — chuyển tiền giữa hai tài khoản, dùng
`Map` LÀM kho fake. Ghép TRỌN FP DI, nhà máy fake, AAA/đặt tên?
::::

::::explain{#chuyentien-voi-map-lam-kho-fake}
`chuyenTien(kho, tuId, denId, soTien)` dùng `Map<string, TaiKhoan>`
LÀM "kho" — MỘT fake **ĐỒNG BỘ** (KHÔNG `Promise`, KHÔNG database
THẬT, CHỈ MỘT `Map` bình thường, ĐỌC/GHI TRỰC TIẾP):

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type TaiKhoan = { id: string; soDu: number };
type Kho = Map<string, TaiKhoan>;
type LoiChuyenTien = "khong_tim_thay_nguoi_gui" | "khong_tim_thay_nguoi_nhan" | "khong_du_so_du";

function chuyenTien(kho: Kho, tuId: string, denId: string, soTien: number): Result<{ tu: TaiKhoan; den: TaiKhoan }, LoiChuyenTien> {
  const tu = kho.get(tuId);
  if (!tu) return loi("khong_tim_thay_nguoi_gui");
  const den = kho.get(denId);
  if (!den) return loi("khong_tim_thay_nguoi_nhan");
  if (tu.soDu < soTien) return loi("khong_du_so_du");
  const tuMoi: TaiKhoan = { id: tu.id, soDu: tu.soDu - soTien };
  const denMoi: TaiKhoan = { id: den.id, soDu: den.soDu + soTien };
  kho.set(tuId, tuMoi);
  kho.set(denId, denMoi);
  return ok({ tu: tuMoi, den: denMoi });
}

function taoKho(taiKhoanBanDau: TaiKhoan[]): Kho {
  const kho: Kho = new Map();
  for (const tk of taiKhoanBanDau) kho.set(tk.id, tk);
  return kho;
}
```

`kho` LÀ tham số ĐẦU TIÊN (FP DI — bài 11), `taoKho` LÀ nhà máy tạo
fake CẤU HÌNH ĐƯỢC (bài 12 — TRUYỀN vào danh sách tài khoản BAN ĐẦU).
BA nhánh lỗi (`LoiChuyenTien`) KIỂM ĐỦ MỌI thứ có thể sai: người gửi
KHÔNG tồn tại, người nhận KHÔNG tồn tại, KHÔNG đủ số dư.
::::

::::example{#aaa-phu-du-duong-thanh-cong-va-loi}
CẤU TRÚC MỖI test theo Arrange-Act-Assert, đặt tên "nên X khi Y" —
PHỦ đủ đường THÀNH CÔNG **VÀ** cả BA nhánh lỗi:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type TaiKhoan = { id: string; soDu: number };
type Kho = Map<string, TaiKhoan>;
type LoiChuyenTien = "khong_tim_thay_nguoi_gui" | "khong_tim_thay_nguoi_nhan" | "khong_du_so_du";
function chuyenTien(kho: Kho, tuId: string, denId: string, soTien: number): Result<{ tu: TaiKhoan; den: TaiKhoan }, LoiChuyenTien> {
  const tu = kho.get(tuId);
  if (!tu) return loi("khong_tim_thay_nguoi_gui");
  const den = kho.get(denId);
  if (!den) return loi("khong_tim_thay_nguoi_nhan");
  if (tu.soDu < soTien) return loi("khong_du_so_du");
  const tuMoi: TaiKhoan = { id: tu.id, soDu: tu.soDu - soTien };
  const denMoi: TaiKhoan = { id: den.id, soDu: den.soDu + soTien };
  kho.set(tuId, tuMoi);
  kho.set(denId, denMoi);
  return ok({ tu: tuMoi, den: denMoi });
}
function taoKho(taiKhoanBanDau: TaiKhoan[]): Kho {
  const kho: Kho = new Map();
  for (const tk of taiKhoanBanDau) kho.set(tk.id, tk);
  return kho;
}
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

function test_nen_chuyen_thanh_cong_khi_so_du_du(): void {
  // Arrange
  const kho = taoKho([{ id: "an", soDu: 100 }, { id: "binh", soDu: 50 }]);
  // Act
  const ketQua = chuyenTien(kho, "an", "binh", 30);
  // Assert
  assertDeepEqual(ketQua, ok({ tu: { id: "an", soDu: 70 }, den: { id: "binh", soDu: 80 } }), "nen chuyen thanh cong khi so du du");
}
test_nen_chuyen_thanh_cong_khi_so_du_du();

function test_nen_bao_loi_khi_khong_du_so_du(): void {
  // Arrange
  const kho = taoKho([{ id: "an", soDu: 10 }, { id: "binh", soDu: 50 }]);
  // Act
  const ketQua = chuyenTien(kho, "an", "binh", 20);
  // Assert
  assertDeepEqual(ketQua, loi("khong_du_so_du"), "nen bao loi khi khong du so du");
}
test_nen_bao_loi_khi_khong_du_so_du();
```

```text title=readonly
[PASS] nen chuyen thanh cong khi so du du
[PASS] nen bao loi khi khong du so du
```

HAI test NÀY dùng `taoKho` (nhà máy, bài 12) tạo state RIÊNG, KHÔNG
chia sẻ, cấu trúc RÕ Arrange-Act-Assert (bài 13), tên MÔ TẢ đúng ý
định. CHỈ THIẾU HAI nhánh lỗi (người gửi/người nhận KHÔNG tồn tại) —
BÀI TẬP bên dưới VIẾT nốt.
::::

::::predict{#doan-that-bai-khong-lam-hong-kho commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type TaiKhoan = { id: string; soDu: number };
type Kho = Map<string, TaiKhoan>;
type LoiChuyenTien = "khong_tim_thay_nguoi_gui" | "khong_tim_thay_nguoi_nhan" | "khong_du_so_du";
function chuyenTien(kho: Kho, tuId: string, denId: string, soTien: number): Result<{ tu: TaiKhoan; den: TaiKhoan }, LoiChuyenTien> {
  const tu = kho.get(tuId);
  if (!tu) return loi("khong_tim_thay_nguoi_gui");
  const den = kho.get(denId);
  if (!den) return loi("khong_tim_thay_nguoi_nhan");
  if (tu.soDu < soTien) return loi("khong_du_so_du");
  const tuMoi: TaiKhoan = { id: tu.id, soDu: tu.soDu - soTien };
  const denMoi: TaiKhoan = { id: den.id, soDu: den.soDu + soTien };
  kho.set(tuId, tuMoi);
  kho.set(denId, denMoi);
  return ok({ tu: tuMoi, den: denMoi });
}
function taoKho(taiKhoanBanDau: TaiKhoan[]): Kho {
  const kho: Kho = new Map();
  for (const tk of taiKhoanBanDau) kho.set(tk.id, tk);
  return kho;
}

const kho = taoKho([{ id: "an", soDu: 10 }, { id: "binh", soDu: 50 }]);
chuyenTien(kho, "an", "binh", 20); // an chi co 10, khong du chuyen 20
console.log(kho.get("an")?.soDu, kho.get("binh")?.soDu);
```

Dòng cuối in ra gì?

:::opt{correct}
`10 50`
:::

:::opt
`-10 70` — vì `chuyenTien` VẪN tính TOÁN VÀ GHI số dư MỚI cho `tu`
(TRỪ `soTien`) TRƯỚC khi kịp phát hiện KHÔNG đủ số dư — số dư ÂM VÀ
số dư người nhận Ở BINH ĐÃ tăng, dù giao dịch CUỐI CÙNG bị coi LÀ lỗi
::why
Gần đúng ở việc bạn nghĩ tới KHẢ NĂNG một hàm chuyển tiền CÓ THỂ "cập
nhật RỒI mới kiểm tra" — MỘT lỗi THẬT SỰ tồn tại trong nhiều hệ thống
chuyển tiền viết CẨU THẢ, một mối lo ĐÚNG ĐẮN cần luôn kiểm tra.

Chỗ lệch: NHÌN LẠI thứ tự CÁC dòng trong THÂN `chuyenTien`: BA dòng
`if` kiểm tra (người gửi tồn tại, người nhận tồn tại, đủ số dư) ĐỀU
đứng **TRƯỚC** ba dòng TÍNH TOÁN VÀ GHI (`tuMoi`, `denMoi`, `kho.set`)
— với `soTien = 20` VÀ `tu.soDu = 10`, điều kiện `tu.soDu < soTien`
(`10 < 20`) LÀ `true`, hàm **RETURN NGAY** Ở dòng `if (tu.soDu <
soTien) return loi("khong_du_so_du");`, KHÔNG BAO GIỜ chạy tới BẤT KỲ
dòng TÍNH TOÁN/GHI nào. `kho` **GIỮ NGUYÊN** giá trị BAN ĐẦU —
`10 50`.
::
:::

:::opt
Máy báo lỗi biên dịch — `kho.get("an")?.soDu` không hợp lệ Ở vị trí
tham số của `console.log`, vì `soDu` CÓ kiểu `number | undefined`
(do `?.` optional chaining), VÀ `console.log` KHÔNG CHẤP NHẬN
`undefined` LÀM đối số
::why
Gần đúng ở việc bạn nhớ `kho.get("an")?.soDu` CÓ kiểu `number |
undefined` (vì `Map.get` trả `TaiKhoan | undefined`, VÀ `?.` LAN
TRUYỀN `undefined` NẾU vế trước LÀ `undefined`) — một quan sát ĐÚNG
về KIỂU của biểu thức NÀY.

Chỗ lệch: `console.log` LÀ MỘT hàm **BIẾN THIÊN** (variadic, `(...
args: any[]) => void`) — nó CHẤP NHẬN **BẤT KỲ** số lượng đối số CỦA
**BẤT KỲ** kiểu nào (KỂ CẢ `undefined`), KHÔNG có RÀNG BUỘC kiểu chặt
NHƯ các hàm TỰ VIẾT trong track NÀY. Biên dịch sạch — VÀ Ở TRƯỜNG HỢP
NÀY, `"an"` VÀ `"binh"` ĐỀU tồn tại trong `kho`, nên `soDu` THỰC TẾ
KHÔNG BAO GIỜ LÀ `undefined`.
::
:::
::::

::::code{#viet_chuyentien}
Cho `Result`/`TaiKhoan`/`Kho`/`taoKho` ĐÃ định nghĩa. Tự viết PHẦN LÕI
của `chuyenTien`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type TaiKhoan = { id: string; soDu: number };
type Kho = Map<string, TaiKhoan>;
type LoiChuyenTien = "khong_tim_thay_nguoi_gui" | "khong_tim_thay_nguoi_nhan" | "khong_du_so_du";

function chuyenTien(kho: Kho, tuId: string, denId: string, soTien: number): Result<{ tu: TaiKhoan; den: TaiKhoan }, LoiChuyenTien> {
  const tu = kho.get(tuId);
  if (!tu) return ___;
  const den = kho.get(denId);
  if (!den) return ___;
  if (tu.soDu < soTien) return ___;
  const tuMoi: TaiKhoan = { id: tu.id, soDu: ___ };
  const denMoi: TaiKhoan = { id: den.id, soDu: ___ };
  kho.set(tuId, tuMoi);
  kho.set(denId, denMoi);
  return ok({ tu: tuMoi, den: denMoi });
}

function taoKho(taiKhoanBanDau: TaiKhoan[]): Kho {
  const kho: Kho = new Map();
  for (const tk of taiKhoanBanDau) kho.set(tk.id, tk);
  return kho;
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

const kho1 = taoKho([{ id: "an", soDu: 100 }, { id: "binh", soDu: 50 }]);
assertDeepEqual(chuyenTien(kho1, "an", "binh", 30), ok({ tu: { id: "an", soDu: 70 }, den: { id: "binh", soDu: 80 } }), "nen chuyen thanh cong khi so du du");
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type TaiKhoan = { id: string; soDu: number };
type Kho = Map<string, TaiKhoan>;
type LoiChuyenTien = "khong_tim_thay_nguoi_gui" | "khong_tim_thay_nguoi_nhan" | "khong_du_so_du";

function chuyenTien(kho: Kho, tuId: string, denId: string, soTien: number): Result<{ tu: TaiKhoan; den: TaiKhoan }, LoiChuyenTien> {
  const tu = kho.get(tuId);
  if (!tu) return loi("khong_tim_thay_nguoi_gui");
  const den = kho.get(denId);
  if (!den) return loi("khong_tim_thay_nguoi_nhan");
  if (tu.soDu < soTien) return loi("khong_du_so_du");
  const tuMoi: TaiKhoan = { id: tu.id, soDu: tu.soDu - soTien };
  const denMoi: TaiKhoan = { id: den.id, soDu: den.soDu + soTien };
  kho.set(tuId, tuMoi);
  kho.set(denId, denMoi);
  return ok({ tu: tuMoi, den: denMoi });
}

function taoKho(taiKhoanBanDau: TaiKhoan[]): Kho {
  const kho: Kho = new Map();
  for (const tk of taiKhoanBanDau) kho.set(tk.id, tk);
  return kho;
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

const kho1 = taoKho([{ id: "an", soDu: 100 }, { id: "binh", soDu: 50 }]);
assertDeepEqual(chuyenTien(kho1, "an", "binh", 30), ok({ tu: { id: "an", soDu: 70 }, den: { id: "binh", soDu: 80 } }), "nen chuyen thanh cong khi so du du");
```

```typescript title=test
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function test_nen_bao_loi_khi_khong_tim_thay_nguoi_gui(): void {
  const kho = taoKho([{ id: "binh", soDu: 50 }]);
  const ketQua = chuyenTien(kho, "khong_ton_tai", "binh", 10);
  assertDeepEqual(ketQua, loi("khong_tim_thay_nguoi_gui"), "nen bao loi khi khong tim thay nguoi gui");
}
test_nen_bao_loi_khi_khong_tim_thay_nguoi_gui();

function test_nen_bao_loi_khi_khong_tim_thay_nguoi_nhan(): void {
  const kho = taoKho([{ id: "an", soDu: 100 }]);
  const ketQua = chuyenTien(kho, "an", "khong_ton_tai", 10);
  assertDeepEqual(ketQua, loi("khong_tim_thay_nguoi_nhan"), "nen bao loi khi khong tim thay nguoi nhan");
}
test_nen_bao_loi_khi_khong_tim_thay_nguoi_nhan();

function test_nen_bao_loi_khi_khong_du_so_du(): void {
  const kho = taoKho([{ id: "an", soDu: 10 }, { id: "binh", soDu: 50 }]);
  const ketQua = chuyenTien(kho, "an", "binh", 20);
  assertDeepEqual(ketQua, loi("khong_du_so_du"), "nen bao loi khi khong du so du");
}
test_nen_bao_loi_khi_khong_du_so_du();

function test_nen_khong_thay_doi_kho_khi_that_bai(): void {
  const kho = taoKho([{ id: "an", soDu: 10 }, { id: "binh", soDu: 50 }]);
  chuyenTien(kho, "an", "binh", 20);
  assertEqual(kho.get("an")?.soDu, 10, "so du nguoi gui phai GIU NGUYEN khi that bai");
  assertEqual(kho.get("binh")?.soDu, 50, "so du nguoi nhan phai GIU NGUYEN khi that bai");
}
test_nen_khong_thay_doi_kho_khi_that_bai();

function test_nen_cap_nhat_dung_so_du_ca_hai_ben(): void {
  const kho = taoKho([{ id: "an", soDu: 200 }, { id: "binh", soDu: 5 }]);
  chuyenTien(kho, "an", "binh", 45);
  assertEqual(kho.get("an")?.soDu, 155, "so du nguoi gui phai TRU dung so tien");
  assertEqual(kho.get("binh")?.soDu, 50, "so du nguoi nhan phai CONG dung so tien");
}
test_nen_cap_nhat_dung_so_du_ca_hai_ben();
```

:::hints
- kind: attention
  body: "Ba blank đầu: mã lỗi tương ứng với ba tình huống (không tìm thấy người gửi/người nhận, không đủ số dư). Hai blank sau: số dư MỚI của người gửi (TRỪ soTien) và người nhận (CỘNG soTien)."
- kind: strategy
  body: 'loi("khong_tim_thay_nguoi_gui") : loi("khong_tim_thay_nguoi_nhan") : loi("khong_du_so_du") : tu.soDu - soTien : den.soDu + soTien'
- kind: one-line
  body: '___ (1) = loi("khong_tim_thay_nguoi_gui")\n___ (2) = loi("khong_tim_thay_nguoi_nhan")\n___ (3) = loi("khong_du_so_du")\n___ (4) = tu.soDu - soTien\n___ (5) = den.soDu + soTien'
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
Cụm 3 hoàn tất: FP DI, nhà máy fake, Arrange-Act-Assert — ghép trọn
vào một use case chuyển tiền thật. Cụm tiếp theo: Property-Based
Testing — kiểm TÍNH CHẤT thay vì từng ví dụ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Test hiện tại kiểm TỪNG VÍ DỤ CỤ THỂ (input A → output B). Có cách
nào kiểm một TÍNH CHẤT ĐÚNG với HÀNG NGÀN ví dụ cùng lúc, thay vì
liệt kê từng cái?
::::

::::checkpoint{mastery=0.8}
::::
