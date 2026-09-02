---
id: ky-nghe-phan-mem.kiem-thu.arrange-act-assert-va-dat-ten
title: "Arrange-Act-Assert & đặt tên \"nên X khi Y\""
summary: "CẤU TRÚC mỗi test thành BA phần thấy rõ (Arrange — dựng dữ liệu/fake; Act — gọi hàm cần test; Assert — kiểm kết quả), đặt tên theo mẫu 'nên [kết quả] khi [điều kiện]'. Test kiểm CÁCH LÀM (gọi đúng bao nhiêu hàm nội bộ, đúng thứ tự) vỡ mỗi lần refactor — test tốt kiểm KẾT QUẢ, sống sót qua refactor."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.aaa-naming]
requires: [kt.fake-factory]
concepts: [kt.aaa-naming]
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
Test hiện tại viết khá "phẳng" — dựng dữ liệu, gọi hàm, kiểm kết quả,
lẫn vào nhau. Cấu trúc RÕ RÀNG hơn, đặt tên DỄ HIỂU hơn — thế nào?
::::

::::explain{#arrange-act-assert}
CẤU TRÚC MỖI test thành **BA phần** thấy RÕ: **Arrange** (dựng dữ
liệu/fake), **Act** (GỌI hàm CẦN test), **Assert** (kiểm kết quả) —
VÀ đặt tên MỖI assertion theo mẫu **"nên [kết quả] khi [điều kiện]"**:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = { timTheoEmail: (email: string) => NguoiDung | null; luu: (nd: NguoiDung) => void };
function dangKyNguoiDung(deps: Deps, ten: string, email: string): Result<NguoiDung, LoiDangKy> {
  if (!email.includes("@")) return loi("email_khong_hop_le");
  const daCo = deps.timTheoEmail(email);
  if (daCo) return loi("email_da_ton_tai");
  const nguoiDungMoi: NguoiDung = { id: `u_${email}`, ten, email };
  deps.luu(nguoiDungMoi);
  return ok(nguoiDungMoi);
}
function taoFakeDeps(emailDaTonTai: string[] = []): { deps: Deps; daLuu: NguoiDung[] } {
  const daLuu: NguoiDung[] = [];
  const deps: Deps = {
    timTheoEmail: (email) => (emailDaTonTai.includes(email) ? { id: "u_x", ten: "X", email } : null),
    luu: (nd) => { daLuu.push(nd); },
  };
  return { deps, daLuu };
}
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

function test_nen_tra_ve_ok_khi_email_moi(): void {
  // Arrange
  const f = taoFakeDeps();
  // Act
  const ketQua = dangKyNguoiDung(f.deps, "An", "an@mail.com");
  // Assert
  assertDeepEqual(ketQua, ok({ id: "u_an@mail.com", ten: "An", email: "an@mail.com" }), "nen tra ve ok khi email moi");
}
test_nen_tra_ve_ok_khi_email_moi();
```

```text
[PASS] nen tra ve ok khi email moi
```

BA COMMENT (`// Arrange`, `// Act`, `// Assert`) TÁCH RÕ ba phần —
NGƯỜI ĐỌC (KỂ CẢ chính bạn, ba tháng SAU) thấy NGAY đâu LÀ "dựng
cảnh", đâu LÀ "hành động cần test", đâu LÀ "kiểm tra". Tên hàm test
(`nen_tra_ve_ok_khi_email_moi`) TỰ GIẢI THÍCH ý định — KHI test NÀY
`[FAIL]`, thông điệp lỗi đọc ĐƯỢC NGAY LÀ "cái GÌ" đã sai, KHÔNG cần
đọc THÂN hàm.
::::

::::example{#test-vo-moi-lan-refactor}
Test kiểm **CÁCH LÀM** (gọi ĐÚNG bao nhiêu hàm nội bộ, ĐÚNG thứ tự)
**VỠ MỖI LẦN refactor** — dù KẾT QUẢ (WHAT) KHÔNG hề đổi:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = { timTheoEmail: (email: string) => NguoiDung | null; luu: (nd: NguoiDung) => void };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function dangKyNguoiDung(deps: Deps, ten: string, email: string): Result<NguoiDung, LoiDangKy> {
  if (!email.includes("@")) return loi("email_khong_hop_le");
  const daCo = deps.timTheoEmail(email);
  if (daCo) return loi("email_da_ton_tai");
  const nguoiDungMoi: NguoiDung = { id: `u_${email}`, ten, email };
  deps.luu(nguoiDungMoi);
  return ok(nguoiDungMoi);
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

// TEST GION (brittle) -- kiem CACH LAM: goi log THEO DUNG THU TU noi bo
const goiLog: string[] = [];
const fakeDepsLog: Deps = {
  timTheoEmail: (email) => { goiLog.push("timTheoEmail"); return null; },
  luu: (nd) => { goiLog.push("luu"); },
};
dangKyNguoiDung(fakeDepsLog, "An", "khongco-at"); // email KHONG hop le
assertDeepEqual(goiLog, [], "email khong hop le: khong goi ham noi bo nao");
```

```text title=readonly
[PASS] email khong hop le: khong goi ham noi bo nao
```

VỚI email KHÔNG HỢP LỆ, `dangKyNguoiDung` **THOÁT NGAY** Ở dòng ĐẦU
(`!email.includes("@")`) — KHÔNG BAO GIỜ chạm tới `deps.timTheoEmail`
HAY `deps.luu`. Test kiểu `goiLog` NÀY **NHẠY CẢM** VỚI CÀI ĐẶT NỘI
BỘ (bao nhiêu hàm được gọi, ĐÚNG thứ tự nào) — MỘT refactor VÔ HẠI
(ví dụ THÊM một lần gọi log NỘI BỘ, KHÔNG đổi Result trả về) sẽ khiến
test NÀY `[FAIL]` dù HÀNH VI quan sát được (input→output) HOÀN TOÀN
GIỐNG NHAU. Test TỐT (Ở phần explain) kiểm **Result** — SỐNG SÓT qua
MỌI refactor kiểu đó.
::::

::::predict{#doan-goilog-khi-that-bai-som commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = { timTheoEmail: (email: string) => NguoiDung | null; luu: (nd: NguoiDung) => void };
function dangKyNguoiDung(deps: Deps, ten: string, email: string): Result<NguoiDung, LoiDangKy> {
  if (!email.includes("@")) return loi("email_khong_hop_le");
  const daCo = deps.timTheoEmail(email);
  if (daCo) return loi("email_da_ton_tai");
  const nguoiDungMoi: NguoiDung = { id: `u_${email}`, ten, email };
  deps.luu(nguoiDungMoi);
  return ok(nguoiDungMoi);
}

const goiLog: string[] = [];
const fakeDepsLog: Deps = {
  timTheoEmail: (email) => { goiLog.push("timTheoEmail"); return null; },
  luu: (nd) => { goiLog.push("luu"); },
};
dangKyNguoiDung(fakeDepsLog, "An", "khongco-at");
console.log(JSON.stringify(goiLog));
```

Dòng cuối in ra gì?

:::opt{correct}
`[]`
:::

:::opt
`["timTheoEmail"]` — vì `dangKyNguoiDung` LUÔN gọi
`deps.timTheoEmail` TRƯỚC KHI kiểm bất kỳ điều kiện nào KHÁC, để TRA
CỨU xem người dùng đã tồn tại chưa TRƯỚC — kiểm email HỢP LỆ CHỈ diễn
ra SAU bước tra cứu ĐÓ
::why
Gần đúng ở việc bạn nhớ `deps.timTheoEmail` LÀ MỘT trong HAI lời gọi
CÓ THỂ xảy ra TRONG `dangKyNguoiDung` — một quan sát ĐÚNG rằng hàm
NÀY CÓ dùng tới `deps.timTheoEmail` Ở ĐÂU đó.

Chỗ lệch: NHÌN LẠI thứ tự CÁC dòng trong THÂN `dangKyNguoiDung`: dòng
**ĐẦU TIÊN** LÀ `if (!email.includes("@")) return loi(...)` — kiểm
email hợp lệ diễn ra **TRƯỚC**, VÀ nếu email KHÔNG hợp lệ (như
`"khongco-at"`, KHÔNG chứa `"@"`), hàm **RETURN NGAY** Ở dòng NÀY,
KHÔNG BAO GIỜ chạy tới dòng gọi `deps.timTheoEmail`. `goiLog` VẪN
**RỖNG** — KHÔNG MỘT mục nào được `push` vào.
::
:::

:::opt
Máy báo lỗi biên dịch — `fakeDepsLog.timTheoEmail` khai kiểu trả về
`NguoiDung | null`, NHƯNG thân hàm arrow `(email) => { goiLog.push
("timTheoEmail"); return null; }` CÓ HAI câu lệnh (statement) trong
thân, TypeScript CHỈ CHO PHÉP arrow function MỘT BIỂU THỨC DUY NHẤT
::why
Gần đúng ở việc bạn để ý hàm arrow NÀY có `{ ... }` BAO quanh THÂN,
KHÁC dạng "MỘT biểu thức" NGẮN GỌN (`(x) => x + 1`) đã quen thuộc —
một quan sát ĐÚNG về SỰ khác biệt CÚ PHÁP.

Chỗ lệch: Arrow function VỚI `{ ... }` (dấu ngoặc nhọn) LÀ dạng **THÂN
KHỐI** (block body) — CHO PHÉP **NHIỀU** câu lệnh, GIỐNG HỆT thân của
`function` thường, CHỈ khác LÀ PHẢI dùng `return` TƯỜNG MINH để trả
giá trị (khác dạng "MỘT biểu thức" tự động trả). Cả HAI dạng ĐỀU LÀ
arrow function HỢP LỆ — TypeScript KHÔNG giới hạn số câu lệnh Ở dạng
thân khối. Biên dịch sạch.
::
:::
::::

::::code{#viet_test_aaa}
Cho `dangKyNguoiDung`/`taoFakeDeps` ĐÃ định nghĩa. Viết HAI test theo
đúng cấu trúc Arrange-Act-Assert VÀ đặt tên "nên X khi Y".

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = { timTheoEmail: (email: string) => NguoiDung | null; luu: (nd: NguoiDung) => void };
function dangKyNguoiDung(deps: Deps, ten: string, email: string): Result<NguoiDung, LoiDangKy> {
  if (!email.includes("@")) return loi("email_khong_hop_le");
  const daCo = deps.timTheoEmail(email);
  if (daCo) return loi("email_da_ton_tai");
  const nguoiDungMoi: NguoiDung = { id: `u_${email}`, ten, email };
  deps.luu(nguoiDungMoi);
  return ok(nguoiDungMoi);
}
function taoFakeDeps(emailDaTonTai: string[] = []): { deps: Deps; daLuu: NguoiDung[] } {
  const daLuu: NguoiDung[] = [];
  const deps: Deps = {
    timTheoEmail: (email) => (emailDaTonTai.includes(email) ? { id: "u_x", ten: "X", email } : null),
    luu: (nd) => { daLuu.push(nd); },
  };
  return { deps, daLuu };
}
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function test_nen_tra_ve_loi_khi_email_thieu_at(): void {
  // Arrange
  const f = taoFakeDeps();
  // Act
  const ketQua = dangKyNguoiDung(f.deps, "Cuong", "khongco-at");
  // Assert
  assertDeepEqual(ketQua, ___, "nen tra ve loi khi email thieu @");
}
test_nen_tra_ve_loi_khi_email_thieu_at();

function test_nen_luu_dung_mot_lan_du_co_mot_lan_that_bai(): void {
  // Arrange
  const f = taoFakeDeps();
  // Act: mot lan that bai (thieu @), mot lan thanh cong
  dangKyNguoiDung(f.deps, "Cuong", "khongco-at");
  dangKyNguoiDung(f.deps, "Duyen", "duyen@mail.com");
  // Assert
  assertEqual(f.daLuu.length, ___, "nen luu dung mot lan du co mot lan that bai truoc do");
}
test_nen_luu_dung_mot_lan_du_co_mot_lan_that_bai();
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = { timTheoEmail: (email: string) => NguoiDung | null; luu: (nd: NguoiDung) => void };
function dangKyNguoiDung(deps: Deps, ten: string, email: string): Result<NguoiDung, LoiDangKy> {
  if (!email.includes("@")) return loi("email_khong_hop_le");
  const daCo = deps.timTheoEmail(email);
  if (daCo) return loi("email_da_ton_tai");
  const nguoiDungMoi: NguoiDung = { id: `u_${email}`, ten, email };
  deps.luu(nguoiDungMoi);
  return ok(nguoiDungMoi);
}
function taoFakeDeps(emailDaTonTai: string[] = []): { deps: Deps; daLuu: NguoiDung[] } {
  const daLuu: NguoiDung[] = [];
  const deps: Deps = {
    timTheoEmail: (email) => (emailDaTonTai.includes(email) ? { id: "u_x", ten: "X", email } : null),
    luu: (nd) => { daLuu.push(nd); },
  };
  return { deps, daLuu };
}
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function test_nen_tra_ve_loi_khi_email_thieu_at(): void {
  // Arrange
  const f = taoFakeDeps();
  // Act
  const ketQua = dangKyNguoiDung(f.deps, "Cuong", "khongco-at");
  // Assert
  assertDeepEqual(ketQua, loi("email_khong_hop_le"), "nen tra ve loi khi email thieu @");
}
test_nen_tra_ve_loi_khi_email_thieu_at();

function test_nen_luu_dung_mot_lan_du_co_mot_lan_that_bai(): void {
  // Arrange
  const f = taoFakeDeps();
  // Act: mot lan that bai (thieu @), mot lan thanh cong
  dangKyNguoiDung(f.deps, "Cuong", "khongco-at");
  dangKyNguoiDung(f.deps, "Duyen", "duyen@mail.com");
  // Assert
  assertEqual(f.daLuu.length, 1, "nen luu dung mot lan du co mot lan that bai truoc do");
}
test_nen_luu_dung_mot_lan_du_co_mot_lan_that_bai();
```

```typescript title=test
const fRieng = taoFakeDeps();
const ketQuaRieng = dangKyNguoiDung(fRieng.deps, "Test", "khong-hop-le");
assertEqual(ketQuaRieng.kind, "loi", "email thieu @ phai co kind la loi, khong phai ok");
if (ketQuaRieng.kind === "loi") {
  assertEqual(ketQuaRieng.loi, "email_khong_hop_le", "ma loi phai dung la email_khong_hop_le");
}

const fSoLan = taoFakeDeps();
dangKyNguoiDung(fSoLan.deps, "Cuong", "khongco-at");
dangKyNguoiDung(fSoLan.deps, "Duyen", "duyen@mail.com");
assertEqual(fSoLan.daLuu.length, 1, "gia tri dung phai la 1, khong phai 0 hay so khac");
```

:::hints
- kind: attention
  body: "Blank 1: Result mong đợi khi email KHÔNG chứa @ — xem lại nhánh đầu tiên của dangKyNguoiDung. Blank 2: số lần luu được gọi sau MỘT lần thất bại (không lưu) và MỘT lần thành công (có lưu)."
- kind: strategy
  body: 'loi("email_khong_hop_le") : 1 — giá trị lỗi tương ứng với nhánh sớm nhất, và tổng số lần lưu thành công qua hai lần gọi.'
- kind: one-line
  body: '___ (1) = loi("email_khong_hop_le")\n___ (2) = 1'
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
Arrange-Act-Assert, đặt tên "nên X khi Y", kiểm KẾT QUẢ không kiểm
CÁCH LÀM. Bài chốt cụm: ghép trọn cụm 3 vào một use case thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ghép TRỌN cụm 3 (FP DI, nhà máy fake, AAA/đặt tên) vào MỘT use case
THẬT — `chuyenTien` (chuyển tiền giữa hai tài khoản, dùng `Map` làm
kho fake) — sẽ trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
