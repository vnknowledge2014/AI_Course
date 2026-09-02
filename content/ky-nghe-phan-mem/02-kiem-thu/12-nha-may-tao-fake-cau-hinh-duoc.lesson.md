---
id: ky-nghe-phan-mem.kiem-thu.nha-may-tao-fake-cau-hinh-duoc
title: "Nhà máy tạo fake CẤU HÌNH ĐƯỢC — mỗi test một bộ dữ liệu RIÊNG"
summary: "taoFakeDeps(emailDaTonTai: string[] = []) — NHÀ MÁY tạo fake MỚI, CẤU HÌNH ĐƯỢC theo TỪNG kịch bản test — tránh CHIA SẺ state ĐỘT BIẾN giữa các test case: mỗi lời gọi taoFakeDeps tạo mảng RIÊNG, không dùng lại của test trước."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.fake-factory]
requires: [kt.fp-di-fakes]
concepts: [kt.fake-factory]
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
Mỗi test case cần MỘT fake KHÁC nhau (mảng rỗng cho "đăng ký mới",
một email đã có cho "trùng lặp") — viết object literal LẶP LẠI có ổn?
::::

::::explain{#nha-may-tao-fake}
`taoFakeDeps(emailDaTonTai: string[] = [])` — MỘT **NHÀ MÁY** (factory
function) tạo fake **MỚI**, **CẤU HÌNH ĐƯỢC** theo TỪNG kịch bản test:

```typescript
type NguoiDung = { id: string; ten: string; email: string };
type Deps = {
  timTheoEmail: (email: string) => NguoiDung | null;
  luu: (nd: NguoiDung) => void;
};

function taoFakeDeps(emailDaTonTai: string[] = []): { deps: Deps; daLuu: NguoiDung[] } {
  const daLuu: NguoiDung[] = [];
  const deps: Deps = {
    timTheoEmail: (email) => (emailDaTonTai.includes(email) ? { id: "u_x", ten: "X", email } : null),
    luu: (nd) => { daLuu.push(nd); },
  };
  return { deps, daLuu };
}

// kich ban "email da ton tai": truyen mang co san email
const f1 = taoFakeDeps(["binh@mail.com"]);
console.log(f1.deps.timTheoEmail("binh@mail.com") !== null);

// kich ban "dang ky moi": khong truyen gi -- mac dinh mang RONG
const f2 = taoFakeDeps();
console.log(f2.deps.timTheoEmail("an@mail.com") === null);
```

```text
true
true
```

`emailDaTonTai` LÀ tham số **CẤU HÌNH** — TRUYỀN vào DANH SÁCH email
"đã tồn tại" TUỲ theo kịch bản đang test. `taoFakeDeps` **TRẢ VỀ**
CẢ `deps` (để TRUYỀN cho hàm cần test) LẪN `daLuu` (mảng SPY, để kiểm
side-effect SAU). MỖI lời gọi `taoFakeDeps` tạo `daLuu` **MỚI** — HAI
lời gọi KHÔNG chia sẻ CÙNG một mảng.
::::

::::example{#tranh-state-chia-se-giua-cac-test}
Nếu HAI test case **DÙNG CHUNG** MỘT fake (KHÔNG tạo MỚI qua nhà
máy), side-effect Ở test NÀY CÓ THỂ "rò" SANG test KHÁC — MỘT lớp bug
"test này pass hay fail TUỲ THEO thứ tự chạy":

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
function taoFakeDeps(emailDaTonTai: string[] = []): { deps: Deps; daLuu: NguoiDung[] } {
  const daLuu: NguoiDung[] = [];
  const deps: Deps = {
    timTheoEmail: (email) => (emailDaTonTai.includes(email) ? { id: "u_x", ten: "X", email } : null),
    luu: (nd) => { daLuu.push(nd); },
  };
  return { deps, daLuu };
}
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// Hai FAKE ĐỘC LẬP -- MỖI test tu tao rieng qua nha may
const fakeA = taoFakeDeps();
dangKyNguoiDung(fakeA.deps, "An", "an@mail.com");
assertEqual(fakeA.daLuu.length, 1, "test A: dung mot nguoi da luu");

const fakeB = taoFakeDeps();
assertEqual(fakeB.daLuu.length, 0, "test B: mang RIENG, khong bi anh huong boi test A");
```

```text title=readonly
[PASS] test A: dung mot nguoi da luu
[PASS] test B: mang RIENG, khong bi anh huong boi test A
```

Nếu `fakeB` **DÙNG LẠI** `fakeA` (KHÔNG gọi `taoFakeDeps` lần NỮA),
`fakeB.daLuu.length` SẼ LÀ `1` (kế thừa side-effect TỪ test A) — MỘT
bug ẨN, CHỈ lộ ra khi ĐỔI thứ tự chạy test. Nhà máy TRÁNH được điều
NÀY: MỖI lời gọi TẠO Map/mảng **RIÊNG**.
::::

::::predict{#doan-hai-fake-doc-lap commitOnce}
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
function taoFakeDeps(emailDaTonTai: string[] = []): { deps: Deps; daLuu: NguoiDung[] } {
  const daLuu: NguoiDung[] = [];
  const deps: Deps = {
    timTheoEmail: (email) => (emailDaTonTai.includes(email) ? { id: "u_x", ten: "X", email } : null),
    luu: (nd) => { daLuu.push(nd); },
  };
  return { deps, daLuu };
}

const f1 = taoFakeDeps();
const f2 = taoFakeDeps();
dangKyNguoiDung(f1.deps, "An", "an@mail.com");
console.log(f1.daLuu.length, f2.daLuu.length);
```

Dòng cuối in ra gì?

:::opt{correct}
`1 0`
:::

:::opt
`1 1` — vì `taoFakeDeps()` (KHÔNG tham số) LUÔN trả CÙNG MỘT bộ giá
trị MẶC ĐỊNH, nên `f1` VÀ `f2` THỰC RA trỏ tới CÙNG object `daLuu`
BÊN TRONG — `dangKyNguoiDung(f1.deps, ...)` ghi vào `daLuu` SẼ khiến
CẢ `f1.daLuu` LẪN `f2.daLuu` cùng thấy PHẦN TỬ MỚI
::why
Gần đúng ở việc bạn nhớ `taoFakeDeps()` gọi HAI LẦN VỚI CÙNG tham số
(KHÔNG truyền gì — CẢ HAI dùng mặc định `[]`) — một quan sát ĐÚNG về
đầu VÀO GIỐNG NHAU.

Chỗ lệch: MỖI **lời gọi** `taoFakeDeps()` chạy LẠI TOÀN BỘ thân hàm
TỪ ĐẦU — dòng `const daLuu: NguoiDung[] = [];` bên TRONG tạo MỘT
**mảng MỚI**, HOÀN TOÀN riêng biệt, MỖI lần hàm được gọi (giống MỌI
hàm khác đã học — MỖI lời gọi có "bản sao riêng" của biến local, dù
tham số đầu vào GIỐNG HỆT nhau). `f1.daLuu` VÀ `f2.daLuu` LÀ HAI mảng
KHÁC NHAU trong bộ nhớ — ghi vào MỘT KHÔNG ảnh hưởng CÁI CÒN LẠI.
`f1.daLuu.length` LÀ `1` (đã đăng ký MỘT người), `f2.daLuu.length`
VẪN `0` (chưa CÓ gì ghi vào NÓ).
::
:::

:::opt
Máy báo lỗi biên dịch — `taoFakeDeps()` gọi KHÔNG truyền tham số
không hợp lệ, vì chữ ký `taoFakeDeps(emailDaTonTai: string[] = [])`
đòi PHẢI truyền MỘT mảng TƯỜNG MINH, giá trị mặc định CHỈ áp dụng khi
tham số LÀ `undefined` chứ KHÔNG áp dụng khi BỎ HẲN đối số
::why
Gần đúng ở việc bạn nhớ CÚ PHÁP `= []` LÀ MỘT "giá trị mặc định" gắn
với tham số — một quan sát ĐÚNG rằng ĐÂY LÀ default parameter.

Chỗ lệch: TRONG TypeScript/JavaScript, **BỎ HẲN** đối số Ở lời gọi
CHÍNH LÀ CÁCH kích hoạt giá trị mặc định (tương đương truyền
`undefined` MỘT CÁCH tường minh — HAI cách nàyGIỐNG NHAU HOÀN TOÀN,
KHÔNG PHÂN BIỆT). `taoFakeDeps()` HOÀN TOÀN hợp lệ, tương đương
`taoFakeDeps(undefined)`, VÀ tham số `emailDaTonTai` sẽ NHẬN giá trị
mặc định `[]`. Biên dịch sạch.
::
:::
::::

::::code{#viet_taofakedeps}
Cho `Deps`/`NguoiDung` ĐÃ định nghĩa. Tự viết phần lõi của nhà máy
`taoFakeDeps`.

```typescript title=starter
type NguoiDung = { id: string; ten: string; email: string };
type Deps = {
  timTheoEmail: (email: string) => NguoiDung | null;
  luu: (nd: NguoiDung) => void;
};

function taoFakeDeps(emailDaTonTai: string[] = []): { deps: Deps; daLuu: NguoiDung[] } {
  const daLuu: NguoiDung[] = [];
  const deps: Deps = {
    timTheoEmail: (email) => (___ ? { id: "u_x", ten: "X", email } : null),
    luu: (nd) => { ___; },
  };
  return { deps, daLuu };
}

const koTraLoi = taoFakeDeps(["a@mail.com"]).deps.timTheoEmail("a@mail.com") !== null;
console.log(koTraLoi ? "[PASS] email co trong danh sach duoc bao da ton tai" : "[FAIL] email co trong danh sach duoc bao da ton tai");
```

```typescript title=solution
type NguoiDung = { id: string; ten: string; email: string };
type Deps = {
  timTheoEmail: (email: string) => NguoiDung | null;
  luu: (nd: NguoiDung) => void;
};

function taoFakeDeps(emailDaTonTai: string[] = []): { deps: Deps; daLuu: NguoiDung[] } {
  const daLuu: NguoiDung[] = [];
  const deps: Deps = {
    timTheoEmail: (email) => (emailDaTonTai.includes(email) ? { id: "u_x", ten: "X", email } : null),
    luu: (nd) => { daLuu.push(nd); },
  };
  return { deps, daLuu };
}

const koTraLoi = taoFakeDeps(["a@mail.com"]).deps.timTheoEmail("a@mail.com") !== null;
console.log(koTraLoi ? "[PASS] email co trong danh sach duoc bao da ton tai" : "[FAIL] email co trong danh sach duoc bao da ton tai");
```

```typescript title=test
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

const f1 = taoFakeDeps(["binh@mail.com"]);
assertEqual(f1.deps.timTheoEmail("binh@mail.com") !== null, true, "email co trong danh sach phai bao da ton tai");
assertEqual(f1.deps.timTheoEmail("khac@mail.com") === null, true, "email KHONG trong danh sach phai bao chua ton tai");

const nguoiTimThay = f1.deps.timTheoEmail("binh@mail.com");
assertEqual(nguoiTimThay?.id, "u_x", "id cua nguoi gia lap tim thay phai dung la u_x");
assertEqual(nguoiTimThay?.ten, "X", "ten cua nguoi gia lap tim thay phai dung la X, khong duoc doi cho voi id");

const f2 = taoFakeDeps();
assertEqual(f2.deps.timTheoEmail("bat_ky@mail.com") === null, true, "mac dinh mang rong: moi email deu chua ton tai");

const nguoiDungMau: NguoiDung = { id: "u_test", ten: "Test", email: "test@mail.com" };
const f3 = taoFakeDeps();
f3.deps.luu(nguoiDungMau);
assertEqual(f3.daLuu.length, 1, "luu phai push dung mot phan tu vao daLuu");
assertEqual(f3.daLuu[0]?.email, "test@mail.com", "phan tu duoc luu phai la CHINH nguoi dung vua truyen vao");

const f4 = taoFakeDeps();
assertEqual(f4.daLuu.length, 0, "fake moi phai co daLuu RIENG, khong ke thua tu f3");
```

:::hints
- kind: attention
  body: "Blank 1: điều kiện quyết định email đã tồn tại hay chưa — kiểm tra email có nằm trong danh sách cấu hình emailDaTonTai không. Blank 2: ghi người dùng mới vào mảng daLuu (mảng RIÊNG của lần gọi này)."
- kind: strategy
  body: 'emailDaTonTai.includes(email) : daLuu.push(nd) — điều kiện tra cứu danh sách cấu hình, thao tác ghi vào mảng spy.'
- kind: one-line
  body: '___ (1) = emailDaTonTai.includes(email)\n___ (2) = daLuu.push(nd)'
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
Nhà máy tạo fake cấu hình được, mỗi lời gọi state riêng — không chia
sẻ giữa các test. Bước tiếp theo: cấu trúc test rõ ràng, đặt tên dễ hiểu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Test hiện tại viết khá "phẳng" — dựng dữ liệu, gọi hàm, kiểm kết quả,
tất cả lẫn vào nhau. Cấu trúc RÕ RÀNG từng phần, đặt tên DỄ HIỂU khi
test thất bại — làm thế nào?
::::

::::checkpoint{mastery=0.8}
::::
