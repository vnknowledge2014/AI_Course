---
id: ky-nghe-phan-mem.ddd.boss-full-application-wiring
title: "BOSS — Full Application Wiring: gom TẤT CẢ dependency vào AppDeps"
summary: "Bài BOSS của track: hiện thực hoá ĐẦY ĐỦ FP DI — MỘT use case function nhận MỘT object deps: AppDeps chứa NHIỀU repository + service khác + helper thuần; orchestrate XEN KẼ bước thuần và bước IO — Sandwich Pattern ở QUY MÔ ứng dụng đầy đủ. dangKyKhachHang(deps, ten, emailTho): Result<KhachHang, string[]>. Test TOÀN BỘ luồng bằng testDeps, KHÔNG mock library nào."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 42
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.gate-boss]
requires: [ddd.unit-of-work]
concepts: [ddd.gate-boss]
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
BÀI CUỐI track. Gom TẤT CẢ (repository, service khác, helper thuần)
vào MỘT object dependency, viết use case ĐẦY ĐỦ như ứng dụng thật.
::::

::::explain{#appdeps-va-sandwich}
`AppDeps` gom **TẤT CẢ** thứ use case CẦN — nhiều repository (bài
37-41), service KHÁC (thông báo), helper THUẦN (sinh mã, lấy giờ) —
thành **MỘT object DUY NHẤT**, hiện thực hoá ĐẦY ĐỦ FP DI (bài 10):

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

type MaKhachHang = string & { readonly __brand: "MaKhachHang" };
type KhachHang = { id: MaKhachHang; ten: string; email: string; ngayTao: number };

type KhoKhachHang = {
  timTheoId: (id: MaKhachHang) => KhachHang | null;
  timTheoEmail: (email: string) => KhachHang | null;
  luu: (kh: KhachHang) => void;
};
type DichVuThongBao = { guiChaoMung: (email: string, ten: string) => void };

// AppDeps: MỌI thứ use case cần, gom vào MỘT object
type AppDeps = {
  khachHang: KhoKhachHang;
  thongBao: DichVuThongBao;
  sinhId: () => MaKhachHang;
  gioHienTai: () => number;
};

function validateTen(ten: string): Result<string, string[]> {
  if (ten.trim() === "") return loi(["tên không được để trống"]);
  return ok(ten);
}
function validateEmail(email: string): Result<string, string[]> {
  if (!email.includes("@")) return loi(["email không hợp lệ"]);
  return ok(email);
}

function dangKyKhachHang(deps: AppDeps, ten: string, emailTho: string): Result<KhachHang, string[]> {
  // THUẦN: validate, gom lỗi (bài 30)
  const ketQuaValidate = map2GomLoi(validateTen(ten), validateEmail(emailTho), (t, e) => ({ ten: t, email: e }));
  if (ketQuaValidate.kind === "loi") return loi(ketQuaValidate.loi);

  // IO: kiểm trùng email
  const daTonTai = deps.khachHang.timTheoEmail(ketQuaValidate.giaTri.email);
  if (daTonTai !== null) return loi(["email đã được đăng ký"]);

  // THUẦN: tạo Entity (bài 4)
  const khachHangMoi: KhachHang = {
    id: deps.sinhId(),
    ten: ketQuaValidate.giaTri.ten,
    email: ketQuaValidate.giaTri.email,
    ngayTao: deps.gioHienTai(),
  };

  // IO: lưu
  deps.khachHang.luu(khachHangMoi);
  // IO: gửi thông báo
  deps.thongBao.guiChaoMung(khachHangMoi.email, khachHangMoi.ten);

  return ok(khachHangMoi);
}
```

`dangKyKhachHang` là **Sandwich Pattern** (bài 8) Ở QUY MÔ ứng dụng
ĐẦY ĐỦ: **THUẦN** (validate) → **IO** (kiểm trùng) → **THUẦN** (tạo
Entity) → **IO** (lưu) → **IO** (thông báo). `deps: AppDeps` LÀ tham
số DUY NHẤT mang MỌI phụ thuộc — hàm KHÔNG tự `import` database
client hay service thông báo nào, TẤT CẢ được TRUYỀN VÀO.
::::

::::example{#testdeps-khong-can-mock}
Test TOÀN BỘ luồng bằng `testDeps` — **in-memory repo** (bài 38) +
**fake notification** (mảng ghi lại email đã "gửi") — **KHÔNG mock
library nào**, KỂ CẢ side-effect (gửi email):

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}
type MaKhachHang = string & { readonly __brand: "MaKhachHang" };
type KhachHang = { id: MaKhachHang; ten: string; email: string; ngayTao: number };
type KhoKhachHang = { timTheoId: (id: MaKhachHang) => KhachHang | null; timTheoEmail: (email: string) => KhachHang | null; luu: (kh: KhachHang) => void };
type DichVuThongBao = { guiChaoMung: (email: string, ten: string) => void };
type AppDeps = { khachHang: KhoKhachHang; thongBao: DichVuThongBao; sinhId: () => MaKhachHang; gioHienTai: () => number };
function validateTen(ten: string): Result<string, string[]> { if (ten.trim() === "") return loi(["tên không được để trống"]); return ok(ten); }
function validateEmail(email: string): Result<string, string[]> { if (!email.includes("@")) return loi(["email không hợp lệ"]); return ok(email); }
function dangKyKhachHang(deps: AppDeps, ten: string, emailTho: string): Result<KhachHang, string[]> {
  const ketQuaValidate = map2GomLoi(validateTen(ten), validateEmail(emailTho), (t, e) => ({ ten: t, email: e }));
  if (ketQuaValidate.kind === "loi") return loi(ketQuaValidate.loi);
  const daTonTai = deps.khachHang.timTheoEmail(ketQuaValidate.giaTri.email);
  if (daTonTai !== null) return loi(["email đã được đăng ký"]);
  const khachHangMoi: KhachHang = { id: deps.sinhId(), ten: ketQuaValidate.giaTri.ten, email: ketQuaValidate.giaTri.email, ngayTao: deps.gioHienTai() };
  deps.khachHang.luu(khachHangMoi);
  deps.thongBao.guiChaoMung(khachHangMoi.email, khachHangMoi.ten);
  return ok(khachHangMoi);
}

function taoTestDeps() {
  const duLieu = new Map<MaKhachHang, KhachHang>();
  const emailDaGui: string[] = [];
  let demId = 0;
  const deps: AppDeps = {
    khachHang: {
      timTheoId: (id) => duLieu.get(id) ?? null,
      timTheoEmail: (email) => [...duLieu.values()].find((kh) => kh.email === email) ?? null,
      luu: (kh) => { duLieu.set(kh.id, kh); },
    },
    thongBao: { guiChaoMung: (email) => { emailDaGui.push(email); } },
    sinhId: () => { demId += 1; return `KH-${demId}` as MaKhachHang; },
    gioHienTai: () => 1000,
  };
  return { deps, emailDaGui };
}

const { deps, emailDaGui } = taoTestDeps();

const kq1 = dangKyKhachHang(deps, "An", "an@shop.vn");
console.log(JSON.stringify(kq1));
console.log(emailDaGui);

const kq2 = dangKyKhachHang(deps, "An Khac", "an@shop.vn"); // TRÙNG email
console.log(JSON.stringify(kq2));
console.log(emailDaGui.length);
```

```text title=readonly
{"kind":"ok","giaTri":{"id":"KH-1","ten":"An","email":"an@shop.vn","ngayTao":1000}}
["an@shop.vn"]
{"kind":"loi","loi":["email đã được đăng ký"]}
1
```

Đăng ký TRÙNG email THẤT BẠI **TRƯỚC** khi chạm tới bước gửi thông
báo — `emailDaGui.length` VẪN LÀ `1` (KHÔNG gửi thêm email chào mừng
NÀO cho lần thất bại). Đây LÀ giá trị của Sandwich Pattern: side-
effect (gửi email) CHỈ xảy ra SAU KHI mọi bước kiểm tra (thuần lẫn
IO) đã ĐI QUA — `testDeps` xác nhận ĐIỀU NÀY mà KHÔNG cần thư viện
mocking nào, chỉ MỘT mảng ghi lại lời gọi.
::::

::::predict{#doan-sai-du-lieu-khong-goi-io commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}
type MaKhachHang = string & { readonly __brand: "MaKhachHang" };
type KhachHang = { id: MaKhachHang; ten: string; email: string; ngayTao: number };
type KhoKhachHang = { timTheoId: (id: MaKhachHang) => KhachHang | null; timTheoEmail: (email: string) => KhachHang | null; luu: (kh: KhachHang) => void };
type DichVuThongBao = { guiChaoMung: (email: string, ten: string) => void };
type AppDeps = { khachHang: KhoKhachHang; thongBao: DichVuThongBao; sinhId: () => MaKhachHang; gioHienTai: () => number };
function validateTen(ten: string): Result<string, string[]> { if (ten.trim() === "") return loi(["tên không được để trống"]); return ok(ten); }
function validateEmail(email: string): Result<string, string[]> { if (!email.includes("@")) return loi(["email không hợp lệ"]); return ok(email); }
function dangKyKhachHang(deps: AppDeps, ten: string, emailTho: string): Result<KhachHang, string[]> {
  const ketQuaValidate = map2GomLoi(validateTen(ten), validateEmail(emailTho), (t, e) => ({ ten: t, email: e }));
  if (ketQuaValidate.kind === "loi") return loi(ketQuaValidate.loi);
  const daTonTai = deps.khachHang.timTheoEmail(ketQuaValidate.giaTri.email);
  if (daTonTai !== null) return loi(["email đã được đăng ký"]);
  const khachHangMoi: KhachHang = { id: deps.sinhId(), ten: ketQuaValidate.giaTri.ten, email: ketQuaValidate.giaTri.email, ngayTao: deps.gioHienTai() };
  deps.khachHang.luu(khachHangMoi);
  deps.thongBao.guiChaoMung(khachHangMoi.email, khachHangMoi.ten);
  return ok(khachHangMoi);
}
function taoTestDeps() {
  const duLieu = new Map<MaKhachHang, KhachHang>();
  const emailDaGui: string[] = [];
  let demId = 0;
  const deps: AppDeps = {
    khachHang: { timTheoId: (id) => duLieu.get(id) ?? null, timTheoEmail: (email) => [...duLieu.values()].find((kh) => kh.email === email) ?? null, luu: (kh) => { duLieu.set(kh.id, kh); } },
    thongBao: { guiChaoMung: (email) => { emailDaGui.push(email); } },
    sinhId: () => { demId += 1; return `KH-${demId}` as MaKhachHang; },
    gioHienTai: () => 1000,
  };
  return { deps, emailDaGui };
}

const { deps, emailDaGui } = taoTestDeps();
// Dữ liệu SAI cả hai field NGAY TỪ ĐẦU -- chưa từng có ai đăng ký trước đó
const ketQua = dangKyKhachHang(deps, "", "khong-hop-le");
console.log(ketQua.kind === "loi" ? ketQua.loi.length : -1);
console.log(emailDaGui.length);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`2` rồi `0`
:::

:::opt
`2` rồi `1` — vì `dangKyKhachHang` LUÔN gọi `deps.thongBao.guiChaoMung`
ở CUỐI hàm dù validate thất bại, chỉ là gửi với dữ liệu RỖNG/không
hợp lệ, KHÔNG bỏ qua bước đó
::why
Gần đúng ở việc bạn nhớ ĐÚNG `deps.thongBao.guiChaoMung` LÀ dòng CUỐI
CÙNG trước `return ok(...)` trong THÂN hàm — đúng, dòng đó tồn tại Ở
CUỐI.

Chỗ lệch: dòng `if (ketQuaValidate.kind === "loi") return
loi(ketQuaValidate.loi);` NẰM Ở ĐẦU hàm — khi `ten`/`emailTho` sai,
hàm **`return` NGAY LẬP TỨC** tại đó, KHÔNG BAO GIỜ chạy tới bất kỳ
dòng NÀO phía sau (kiểm trùng email, tạo Entity, `luu`, và ĐẶC BIỆT
`guiChaoMung`). `emailDaGui` giữ nguyên MẢNG RỖNG (`[]`, độ dài `0`)
— KHÔNG một email nào được "gửi" cho một lượt đăng ký THẤT BẠI ngay
từ bước validate.
::
:::

:::opt
Máy báo lỗi biên dịch — `dangKyKhachHang(deps, "", "khong-hop-le")`
không hợp lệ vì tham số `ten`/`emailTho` có kiểu `string`, không chấp
nhận chuỗi RỖNG hay chuỗi thiếu ký tự `@`
::why
Gần đúng ở việc bạn nhớ ĐÚNG chuỗi rỗng VÀ chuỗi thiếu `@` LÀ dữ liệu
KHÔNG HỢP LỆ về mặt NGHIỆP VỤ (chính XÁC là điều `validateTen`/
`validateEmail` từ chối) — quan sát đó đúng.

Chỗ lệch: kiểu `string` chấp nhận **MỌI** chuỗi, kể cả RỖNG hay
KHÔNG chứa `@` — không có ràng buộc NGHIỆP VỤ nào Ở TẦNG KIỂU. Việc
từ chối là quy tắc kiểm LÚC CHẠY (`validateTen`/`validateEmail`),
không phải giới hạn của kiểu `string` — biên dịch sạch.
::
:::
::::

::::code{#viet_dangkykhachhang}
Tự viết BA phần cốt lõi của `dangKyKhachHang`: từ chối email trùng,
sinh mã khách hàng mới, và trả kết quả thành công.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}
type MaKhachHang = string & { readonly __brand: "MaKhachHang" };
type KhachHang = { id: MaKhachHang; ten: string; email: string; ngayTao: number };
type KhoKhachHang = { timTheoId: (id: MaKhachHang) => KhachHang | null; timTheoEmail: (email: string) => KhachHang | null; luu: (kh: KhachHang) => void };
type DichVuThongBao = { guiChaoMung: (email: string, ten: string) => void };
type AppDeps = { khachHang: KhoKhachHang; thongBao: DichVuThongBao; sinhId: () => MaKhachHang; gioHienTai: () => number };
function validateTen(ten: string): Result<string, string[]> { if (ten.trim() === "") return loi(["tên không được để trống"]); return ok(ten); }
function validateEmail(email: string): Result<string, string[]> { if (!email.includes("@")) return loi(["email không hợp lệ"]); return ok(email); }

function dangKyKhachHang(deps: AppDeps, ten: string, emailTho: string): Result<KhachHang, string[]> {
  const ketQuaValidate = map2GomLoi(validateTen(ten), validateEmail(emailTho), (t, e) => ({ ten: t, email: e }));
  if (ketQuaValidate.kind === "loi") return loi(ketQuaValidate.loi);

  const daTonTai = deps.khachHang.timTheoEmail(ketQuaValidate.giaTri.email);
  if (daTonTai !== null) return ___;

  const khachHangMoi: KhachHang = {
    id: ___,
    ten: ketQuaValidate.giaTri.ten,
    email: ketQuaValidate.giaTri.email,
    ngayTao: deps.gioHienTai(),
  };

  deps.khachHang.luu(khachHangMoi);
  deps.thongBao.guiChaoMung(khachHangMoi.email, khachHangMoi.ten);

  return ___;
}

function taoTestDeps() {
  const duLieu = new Map<MaKhachHang, KhachHang>();
  const emailDaGui: string[] = [];
  let demId = 0;
  const deps: AppDeps = {
    khachHang: { timTheoId: (id) => duLieu.get(id) ?? null, timTheoEmail: (email) => [...duLieu.values()].find((kh) => kh.email === email) ?? null, luu: (kh) => { duLieu.set(kh.id, kh); } },
    thongBao: { guiChaoMung: (email) => { emailDaGui.push(email); } },
    sinhId: () => { demId += 1; return `KH-${demId}` as MaKhachHang; },
    gioHienTai: () => 1000,
  };
  return { deps, emailDaGui };
}

const { deps } = taoTestDeps();
console.log(JSON.stringify(dangKyKhachHang(deps, "An", "an@shop.vn")));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}
type MaKhachHang = string & { readonly __brand: "MaKhachHang" };
type KhachHang = { id: MaKhachHang; ten: string; email: string; ngayTao: number };
type KhoKhachHang = { timTheoId: (id: MaKhachHang) => KhachHang | null; timTheoEmail: (email: string) => KhachHang | null; luu: (kh: KhachHang) => void };
type DichVuThongBao = { guiChaoMung: (email: string, ten: string) => void };
type AppDeps = { khachHang: KhoKhachHang; thongBao: DichVuThongBao; sinhId: () => MaKhachHang; gioHienTai: () => number };
function validateTen(ten: string): Result<string, string[]> { if (ten.trim() === "") return loi(["tên không được để trống"]); return ok(ten); }
function validateEmail(email: string): Result<string, string[]> { if (!email.includes("@")) return loi(["email không hợp lệ"]); return ok(email); }

function dangKyKhachHang(deps: AppDeps, ten: string, emailTho: string): Result<KhachHang, string[]> {
  const ketQuaValidate = map2GomLoi(validateTen(ten), validateEmail(emailTho), (t, e) => ({ ten: t, email: e }));
  if (ketQuaValidate.kind === "loi") return loi(ketQuaValidate.loi);

  const daTonTai = deps.khachHang.timTheoEmail(ketQuaValidate.giaTri.email);
  if (daTonTai !== null) return loi(["email đã được đăng ký"]);

  const khachHangMoi: KhachHang = {
    id: deps.sinhId(),
    ten: ketQuaValidate.giaTri.ten,
    email: ketQuaValidate.giaTri.email,
    ngayTao: deps.gioHienTai(),
  };

  deps.khachHang.luu(khachHangMoi);
  deps.thongBao.guiChaoMung(khachHangMoi.email, khachHangMoi.ten);

  return ok(khachHangMoi);
}

function taoTestDeps() {
  const duLieu = new Map<MaKhachHang, KhachHang>();
  const emailDaGui: string[] = [];
  let demId = 0;
  const deps: AppDeps = {
    khachHang: { timTheoId: (id) => duLieu.get(id) ?? null, timTheoEmail: (email) => [...duLieu.values()].find((kh) => kh.email === email) ?? null, luu: (kh) => { duLieu.set(kh.id, kh); } },
    thongBao: { guiChaoMung: (email) => { emailDaGui.push(email); } },
    sinhId: () => { demId += 1; return `KH-${demId}` as MaKhachHang; },
    gioHienTai: () => 1000,
  };
  return { deps, emailDaGui };
}

const { deps } = taoTestDeps();
console.log(JSON.stringify(dangKyKhachHang(deps, "An", "an@shop.vn")));
```

```typescript title=test
const { deps: depsTest, emailDaGui: emailDaGuiTest } = taoTestDeps();

const kqOk = dangKyKhachHang(depsTest, "An", "an@shop.vn");
if (kqOk.kind !== "ok") throw new Error("đăng ký hợp lệ phải ra ok");
if (kqOk.kind === "ok" && kqOk.giaTri.ten !== "An") throw new Error("KhachHang tạo ra phải giữ đúng ten");
if (emailDaGuiTest.length !== 1) throw new Error("đăng ký hợp lệ phải gửi ĐÚNG một email chào mừng");
if (emailDaGuiTest[0] !== "an@shop.vn") throw new Error("email chào mừng phải gửi đúng địa chỉ vừa đăng ký");

const kqTrung = dangKyKhachHang(depsTest, "An Khac", "an@shop.vn");
if (kqTrung.kind !== "loi") throw new Error("đăng ký email TRÙNG phải ra loi");
if (emailDaGuiTest.length !== 1) throw new Error("đăng ký thất bại vì trùng email KHÔNG được gửi thêm email chào mừng");

const kqSaiCaHai = dangKyKhachHang(depsTest, "", "khong-hop-le");
if (kqSaiCaHai.kind !== "loi") throw new Error("dữ liệu sai cả hai field phải ra loi");
if (kqSaiCaHai.kind === "loi" && kqSaiCaHai.loi.length !== 2) throw new Error("sai cả hai field phải gom ĐỦ HAI lỗi (nối bài 30)");
if (emailDaGuiTest.length !== 1) throw new Error("đăng ký thất bại NGAY từ validate KHÔNG được gửi email nào");
```

:::hints
- kind: attention
  body: "Trùng email: bọc thông điệp lỗi bằng loi([...]). Sinh mã khách hàng: gọi deps.sinhId() (KHÔNG tự tạo mã, PHẢI qua dependency). Kết quả cuối: bọc khachHangMoi bằng ok(...)."
- kind: strategy
  body: 'loi(["email đã được đăng ký"]) : deps.sinhId() : ok(khachHangMoi) — ba mảnh còn thiếu của Sandwich Pattern.'
- kind: one-line
  body: '___ (trùng email) = loi(["email đã được đăng ký"])\n___ (sinh mã) = deps.sinhId()\n___ (kết quả cuối) = ok(khachHangMoi)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "ok"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Track T5.3 hoàn tất — 42/42 bài. DDD từ khái niệm (Ubiquitous
Language, Entity, Value Object) tới ứng dụng THẬT (Repository, Unit
of Work, full dependency wiring) — mọi mảnh ghép nối liền thành một
use case hoàn chỉnh, test được TOÀN BỘ mà không cần mock library nào.
::::

::::reflect{#nghi-lai}
Track DDD đã xong. Bạn đã đi từ domain modeling (Entity, Value
Object, Aggregate) qua workflow-as-pipeline (chainResult, ROP), error
handling sâu (combine, form validation), serialization (DTO,
versioning), tới persistence đầy đủ (Repository, Unit of Work) — một
ứng dụng THẬT, mọi mảnh test được, không mock library nào.
::::

::::checkpoint{mastery=0.85}
::::
