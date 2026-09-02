---
id: ky-nghe-phan-mem.kiem-thu.fp-di-fake-doi-tuong-thuan
title: "FP DI: dependency là THAM SỐ, fake là OBJECT THUẦN"
summary: "Thay vì thư viện mock, truyền dependency (Deps) NHƯ MỘT THAM SỐ object thường (FP DI) — 'fake' phục vụ test CHỈ LÀ MỘT object literal THỰC HIỆN cùng interface, side-effect ghi vào MỘT mảng thường (đóng vai spy). dangKyNguoiDung(deps, ten, email) với deps: {timTheoEmail, luu}."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.fp-di-fakes]
requires: [kt.tdd-new-state-machine]
concepts: [kt.fp-di-fakes]
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
Cụm mới. `dangKyNguoiDung` cần "tra email đã tồn tại chưa" — KHÔNG có
database THẬT nào trong sandbox. Test hàm ĐÓ thế nào?
::::

::::explain{#fp-di-fake-object-thuan}
Thay vì thư viện **mock** (jest.fn, sinon...), truyền dependency
(`deps`) **NHƯ MỘT THAM SỐ** object thường — "FP DI" (Dependency
Injection kiểu FP, GIỐNG tinh thần Reader Ở T4.5: HÀM cần gì thì NHẬN
CÁI đó qua THAM SỐ, KHÔNG tự đi "tìm" nó). "Fake" phục vụ test **CHỈ
LÀ MỘT object literal** THỰC HIỆN CÙNG interface:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = {
  timTheoEmail: (email: string) => NguoiDung | null;
  luu: (nd: NguoiDung) => void;
};

function dangKyNguoiDung(deps: Deps, ten: string, email: string): Result<NguoiDung, LoiDangKy> {
  if (!email.includes("@")) return loi("email_khong_hop_le");
  const daCo = deps.timTheoEmail(email);
  if (daCo) return loi("email_da_ton_tai");
  const nguoiDungMoi: NguoiDung = { id: `u_${email}`, ten, email };
  deps.luu(nguoiDungMoi);
  return ok(nguoiDungMoi);
}
```

`dangKyNguoiDung` **KHÔNG** biết `deps.timTheoEmail`/`deps.luu` đến TỪ
đâu (database THẬT, file, hay MỘT object GIẢ LẬP) — nó CHỈ gọi CÁI GÌ
được TRUYỀN vào QUA tham số `deps`. Đây LÀ TOÀN BỘ ý tưởng "Dependency
Injection" — phụ thuộc LÀ dữ liệu ĐẦU VÀO, KHÔNG PHẢI thứ hàm TỰ đi
"triệu hồi".
::::

::::example{#fake-ghi-side-effect-vao-mang}
Trong TEST, TRUYỀN vào MỘT **fake** — object literal với `timTheoEmail`
trả CỨNG giá trị GIẢ LẬP, VÀ `luu` chỉ `push` vào MỘT mảng THƯỜNG
(đóng vai "spy" — GHI LẠI những gì ĐÃ xảy ra, để assertion kiểm SAU):

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = {
  timTheoEmail: (email: string) => NguoiDung | null;
  luu: (nd: NguoiDung) => void;
};
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
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// FAKE: khong co nguoi dung nao trung email -- kich ban "dang ky moi thanh cong"
const daLuu: NguoiDung[] = [];
const fakeDeps: Deps = {
  timTheoEmail: (email) => null,
  luu: (nd) => { daLuu.push(nd); },
};

const ketQua = dangKyNguoiDung(fakeDeps, "An", "an@mail.com");
assertDeepEqual(ketQua, ok({ id: "u_an@mail.com", ten: "An", email: "an@mail.com" }), "dang ky moi thanh cong");
assertEqual(daLuu.length, 1, "luu duoc goi dung mot lan");
```

```text title=readonly
[PASS] dang ky moi thanh cong
[PASS] luu duoc goi dung mot lan
```

KHÔNG database THẬT nào chạy — `fakeDeps` LÀ MỘT object literal
BÌNH THƯỜNG, `daLuu` LÀ MỘT `NguoiDung[]` BÌNH THƯỜNG. Assertion THỨ
HAI (`daLuu.length === 1`) kiểm được **side-effect** (đã LƯU) MÀ
KHÔNG cần thư viện spy CHUYÊN DỤNG — MẢNG THƯỜNG LÀ ĐỦ.
::::

::::predict{#doan-fake-luon-bao-da-ton-tai commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = {
  timTheoEmail: (email: string) => NguoiDung | null;
  luu: (nd: NguoiDung) => void;
};
function dangKyNguoiDung(deps: Deps, ten: string, email: string): Result<NguoiDung, LoiDangKy> {
  if (!email.includes("@")) return loi("email_khong_hop_le");
  const daCo = deps.timTheoEmail(email);
  if (daCo) return loi("email_da_ton_tai");
  const nguoiDungMoi: NguoiDung = { id: `u_${email}`, ten, email };
  deps.luu(nguoiDungMoi);
  return ok(nguoiDungMoi);
}

// FAKE: LUON bao co nguoi dung trung, VA luu se throw neu bi goi
const fakeDepsLuonCoNguoiDung: Deps = {
  timTheoEmail: () => ({ id: "u_bat_ky", ten: "Ai Do", email: "khac@mail.com" }),
  luu: () => { throw new Error("khong duoc goi"); },
};

const ketQua = dangKyNguoiDung(fakeDepsLuonCoNguoiDung, "Cuong", "cuong@mail.com");
console.log(ketQua.kind === "loi" ? ketQua.loi : "thanh cong");
```

Dòng cuối in ra gì?

:::opt{correct}
`email_da_ton_tai`
:::

:::opt
Máy NÉM lỗi lúc chạy — `dangKyNguoiDung` sẽ gọi `deps.luu(...)` Ở
CUỐI (để LƯU người dùng mới `"Cuong"`), VÀ `fakeDepsLuonCoNguoiDung.luu`
LUÔN `throw` — chương trình dừng đột ngột TRƯỚC khi in được dòng nào
::why
Gần đúng ở việc bạn để ý `fakeDepsLuonCoNguoiDung.luu` ĐƯỢC viết để
`throw` KHI bị gọi — một quan sát ĐÚNG về CÀI ĐẶT của fake.

Chỗ lệch: `luu` **KHÔNG BAO GIỜ được gọi** Ở kịch bản NÀY. Nhìn THÂN
`dangKyNguoiDung`: `daCo = deps.timTheoEmail(email)` gọi
`fakeDepsLuonCoNguoiDung.timTheoEmail` (LUÔN trả MỘT `NguoiDung`
KHÔNG PHẢI `null`, BẤT KỂ email TRUYỀN vào LÀ gì — fake NÀY "LUÔN báo
đã tồn tại"), nên `daCo` LUÔN truthy → `if (daCo) return loi
("email_da_ton_tai")` chạy VÀ hàm **THOÁT NGAY**, KHÔNG BAO GIỜ chạy
tới dòng `deps.luu(...)`. `luu` KHÔNG bị gọi, KHÔNG `throw` nào xảy
ra. `ketQua.kind === "loi"` LÀ `true`, in `ketQua.loi` =
`"email_da_ton_tai"`.
::
:::

:::opt
Máy báo lỗi biên dịch — `fakeDepsLuonCoNguoiDung` không hợp lệ vì
`timTheoEmail` TRẢ VỀ MỘT `NguoiDung` cụ thể (`"khac@mail.com"`),
KHÔNG PHỤ THUỘC tham số `email` TRUYỀN VÀO, TypeScript đòi hàm PHẢI
dùng tham số của NÓ trong THÂN
::why
Gần đúng ở việc bạn để ý hàm `timTheoEmail` Ở fake NÀY **KHÔNG DÙNG**
tham số `email` TRUYỀN vào (LUÔN trả CÙNG MỘT giá trị CỨNG) — một
quan sát ĐÚNG về CÀI ĐẶT.

Chỗ lệch: TypeScript KHÔNG có RÀNG BUỘC "MỌI tham số PHẢI được DÙNG
trong THÂN hàm" — một hàm `(email: string) => NguoiDung | null` HOÀN
TOÀN hợp lệ dù KHÔNG chạm tới `email` MỘT LẦN nào (giống MỌI hàm hằng
số ĐÃ gặp — ví dụ `() => 0`). Ngược lại: ĐÂY CHÍNH LÀ đặc điểm CÓ ÍCH
của fake — "LUÔN trả CÙNG kết quả, BẤT KỂ input" LÀ MỘT kịch bản test
HỢP LỆ VÀ CÓ CHỦ ĐÍCH. Biên dịch sạch.
::
:::
::::

::::code{#viet_dangkynguoidung}
Cho `Deps`/`NguoiDung`/`Result` ĐÃ định nghĩa. Tự viết phần lõi của
`dangKyNguoiDung`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = {
  timTheoEmail: (email: string) => NguoiDung | null;
  luu: (nd: NguoiDung) => void;
};

function dangKyNguoiDung(deps: Deps, ten: string, email: string): Result<NguoiDung, LoiDangKy> {
  if (!email.includes("@")) return loi("email_khong_hop_le");
  const daCo = ___;
  if (daCo) return loi("email_da_ton_tai");
  const nguoiDungMoi: NguoiDung = { id: `u_${email}`, ten, email };
  ___;
  return ok(nguoiDungMoi);
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

const daLuu: NguoiDung[] = [];
const fakeDeps: Deps = { timTheoEmail: (email) => null, luu: (nd) => { daLuu.push(nd); } };
assertDeepEqual(dangKyNguoiDung(fakeDeps, "An", "an@mail.com"), ok({ id: "u_an@mail.com", ten: "An", email: "an@mail.com" }), "dang ky moi thanh cong");
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type NguoiDung = { id: string; ten: string; email: string };
type LoiDangKy = "email_da_ton_tai" | "email_khong_hop_le";
type Deps = {
  timTheoEmail: (email: string) => NguoiDung | null;
  luu: (nd: NguoiDung) => void;
};

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

const daLuu: NguoiDung[] = [];
const fakeDeps: Deps = { timTheoEmail: (email) => null, luu: (nd) => { daLuu.push(nd); } };
assertDeepEqual(dangKyNguoiDung(fakeDeps, "An", "an@mail.com"), ok({ id: "u_an@mail.com", ten: "An", email: "an@mail.com" }), "dang ky moi thanh cong");
```

```typescript title=test
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertEqual(daLuu.length, 1, "luu phai duoc goi DUNG MOT LAN khi dang ky moi thanh cong");

const daLuu2: NguoiDung[] = [];
const fakeDepsTonTai: Deps = {
  timTheoEmail: () => ({ id: "u_x", ten: "X", email: "binh@mail.com" }),
  luu: (nd) => { daLuu2.push(nd); },
};
assertDeepEqual(dangKyNguoiDung(fakeDepsTonTai, "Binh", "binh@mail.com"), loi("email_da_ton_tai"), "email da ton tai bi tu choi");
assertEqual(daLuu2.length, 0, "luu KHONG duoc goi khi email da ton tai");
```

:::hints
- kind: attention
  body: "Blank 1: gọi deps.timTheoEmail với email đang xét để biết đã có người dùng trùng chưa. Blank 2: gọi deps.luu để LƯU người dùng mới vừa dựng."
- kind: strategy
  body: 'deps.timTheoEmail(email) : deps.luu(nguoiDungMoi) — gọi ĐÚNG phương thức tương ứng của deps, truyền ĐÚNG giá trị đang có trong phạm vi.'
- kind: one-line
  body: '___ (1) = deps.timTheoEmail(email)\n___ (2) = deps.luu(nguoiDungMoi)'
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
FP DI: dependency là tham số, fake là object thuần, side-effect ghi
vào mảng thường. Bước tiếp theo: nhà máy tạo fake cấu hình được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mỗi test case cần MỘT fake KHÁC nhau (mảng rỗng cho "đăng ký mới",
một email đã có cho "trùng lặp") — viết object literal LẶP LẠI mỗi
lần có ổn không, hay có cách tốt hơn?
::::

::::checkpoint{mastery=0.8}
::::
