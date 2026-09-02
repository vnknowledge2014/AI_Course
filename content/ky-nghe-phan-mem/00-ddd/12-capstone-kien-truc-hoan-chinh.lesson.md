---
id: ky-nghe-phan-mem.ddd.capstone-kien-truc-hoan-chinh
title: "Capstone: lắp ráp kiến trúc FP hoàn chỉnh"
summary: "Kết hợp TẤT CẢ — Domain (pure + branded type + Result) → Infra interfaces → Application use case (Sandwich) → Test (fake deps, không mock lib). xacNhanDonHangUseCase(deps, maDonHang): Result<DonHang, string>."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.architecture-capstone]
requires: [ddd.reader-pattern]
concepts: [ddd.architecture-capstone]
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
Bài chốt cụm 2. Bốn tầng, module boundary, FP DI, Reader Pattern — ghép
TẤT CẢ lại thành MỘT use case hoàn chỉnh.
::::

::::explain{#lap-rap-tat-ca}
Kết hợp: **Domain** (pure + `Result` đã học) → **Infra interface**
(bài 9-11, domain định nghĩa `KhoDonHang` nó CẦN) → **Application use
case** (Sandwich Pattern, bài 8 — Read→Process→Write) → **Test** (fake
deps, KHÔNG mock library):

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type DonHang = { maDonHang: string; trangThai: "nhap" | "da_xac_nhan"; tongTien: number };

// Infra interface — Domain ĐỊNH NGHĨA shape nó cần
type KhoDonHang = {
  timTheoId: (id: string) => DonHang | undefined;
  luu: (dh: DonHang) => void;
};
type Deps = { khoDonHang: KhoDonHang };

// Application use case — Sandwich Pattern
function xacNhanDonHangUseCase(deps: Deps, maDonHang: string): Result<DonHang, string> {
  const dh = deps.khoDonHang.timTheoId(maDonHang);          // Read (IO)
  if (dh === undefined) return loi("không tìm thấy đơn hàng");
  if (dh.trangThai !== "nhap") return loi("đơn hàng đã được xác nhận trước đó"); // Pure rule
  const dhMoi: DonHang = { ...dh, trangThai: "da_xac_nhan" };
  deps.khoDonHang.luu(dhMoi);                                // Write (IO)
  return ok(dhMoi);
}

// Test: fake khoDonHang — object literal, KHÔNG mock library
const boNho = new Map<string, DonHang>([["DH-01", { maDonHang: "DH-01", trangThai: "nhap", tongTien: 100000 }]]);
const khoFake: KhoDonHang = {
  timTheoId: (id) => boNho.get(id),
  luu: (dh) => boNho.set(dh.maDonHang, dh),
};

const ketQua = xacNhanDonHangUseCase({ khoDonHang: khoFake }, "DH-01");
console.log(JSON.stringify(ketQua));
console.log(boNho.get("DH-01")?.trangThai);
```

```text
{"kind":"ok","giaTri":{"maDonHang":"DH-01","trangThai":"da_xac_nhan","tongTien":100000}}
da_xac_nhan
```

`xacNhanDonHangUseCase` orchestrate ĐÚNG thứ tự Sandwich: TÌM (IO qua
`deps.khoDonHang.timTheoId`) → hai kiểm tra NGHIỆP VỤ thuần (đã tồn
tại? đã xác nhận chưa?) → GHI (IO qua `deps.khoDonHang.luu`). Test
KHÔNG cần database thật — `khoFake` (một object literal bọc `Map`) ĐỦ
để test TOÀN BỘ luồng.
::::

::::example{#goi-lai-tren-don-da-xac-nhan}
Gọi LẠI use case trên đơn hàng ĐÃ xác nhận — business rule chặn ĐÚNG:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type DonHang = { maDonHang: string; trangThai: "nhap" | "da_xac_nhan"; tongTien: number };
type KhoDonHang = { timTheoId: (id: string) => DonHang | undefined; luu: (dh: DonHang) => void };
type Deps = { khoDonHang: KhoDonHang };
function xacNhanDonHangUseCase(deps: Deps, maDonHang: string): Result<DonHang, string> {
  const dh = deps.khoDonHang.timTheoId(maDonHang);
  if (dh === undefined) return loi("không tìm thấy đơn hàng");
  if (dh.trangThai !== "nhap") return loi("đơn hàng đã được xác nhận trước đó");
  const dhMoi: DonHang = { ...dh, trangThai: "da_xac_nhan" };
  deps.khoDonHang.luu(dhMoi);
  return ok(dhMoi);
}
const boNho = new Map<string, DonHang>([["DH-01", { maDonHang: "DH-01", trangThai: "nhap", tongTien: 100000 }]]);
const khoFake: KhoDonHang = { timTheoId: (id) => boNho.get(id), luu: (dh) => boNho.set(dh.maDonHang, dh) };

xacNhanDonHangUseCase({ khoDonHang: khoFake }, "DH-01"); // lần 1: thành công
const ketQuaLan2 = xacNhanDonHangUseCase({ khoDonHang: khoFake }, "DH-01"); // lần 2: đã xác nhận rồi
console.log(JSON.stringify(ketQuaLan2));
```

```text title=readonly
{"kind":"loi","loi":"đơn hàng đã được xác nhận trước đó"}
```

Lần GỌI THỨ HAI trên CÙNG `"DH-01"` (đã được `"da_xac_nhan"` từ lần
đầu) bị CHẶN bởi kiểm tra `dh.trangThai !== "nhap"` — business rule
NGĂN xác nhận HAI LẦN, hoàn toàn TRONG use case, KHÔNG cần database
thật để kiểm tra.
::::

::::predict{#doan-khong-tim-thay-khong-ghi commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type DonHang = { maDonHang: string; trangThai: "nhap" | "da_xac_nhan"; tongTien: number };
type KhoDonHang = { timTheoId: (id: string) => DonHang | undefined; luu: (dh: DonHang) => void };
type Deps = { khoDonHang: KhoDonHang };
function xacNhanDonHangUseCase(deps: Deps, maDonHang: string): Result<DonHang, string> {
  const dh = deps.khoDonHang.timTheoId(maDonHang);
  if (dh === undefined) return loi("không tìm thấy đơn hàng");
  if (dh.trangThai !== "nhap") return loi("đơn hàng đã được xác nhận trước đó");
  const dhMoi: DonHang = { ...dh, trangThai: "da_xac_nhan" };
  deps.khoDonHang.luu(dhMoi);
  return ok(dhMoi);
}

const boNho = new Map<string, DonHang>();
let soLanGoiLuu = 0;
const khoFake: KhoDonHang = {
  timTheoId: (id) => boNho.get(id),
  luu: (dh) => { soLanGoiLuu = soLanGoiLuu + 1; boNho.set(dh.maDonHang, dh); },
};

xacNhanDonHangUseCase({ khoDonHang: khoFake }, "DH-KHONG-CO");
console.log(soLanGoiLuu);
```

`boNho` RỖNG — không có đơn hàng `"DH-KHONG-CO"` nào cả. Dòng cuối in
ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì `xacNhanDonHangUseCase` LUÔN gọi `deps.khoDonHang.luu` để ghi
lại kết quả, kể cả khi kết quả là lỗi "không tìm thấy"
::why
Gần đúng ở việc bạn nhớ ĐÚNG use case CÓ bước gọi `deps.khoDonHang.luu`
— quan sát về việc CÓ bước đó tồn tại đúng.

Chỗ lệch: `deps.khoDonHang.luu(dhMoi)` CHỈ được gọi Ở CUỐI hàm, SAU khi
đã vượt qua CẢ HAI kiểm tra sớm (`if (dh === undefined) return ...`,
`if (dh.trangThai !== "nhap") return ...`). Với `"DH-KHONG-CO"`,
`timTheoId` trả `undefined` — hàm `return` NGAY ở kiểm tra ĐẦU TIÊN,
KHÔNG BAO GIỜ chạy tới dòng `deps.khoDonHang.luu(dhMoi)`.
`soLanGoiLuu` giữ nguyên `0`.
::
:::

:::opt
Máy báo lỗi lúc chạy — `dhMoi` được tham chiếu trong `deps.khoDonHang.luu(dhMoi)`
dù `dh` là `undefined`, gây crash truy cập thuộc tính của `undefined`
::why
Gần đúng ở việc bạn để ý `dh` CÓ THỂ là `undefined` trong tình huống
này — quan sát về khả năng đó đúng, và đây CHÍNH LÀ lý do CẦN kiểm tra
sớm.

Chỗ lệch: dòng `deps.khoDonHang.luu(dhMoi)` (và cả `const dhMoi = {
...dh, ... }` ngay TRƯỚC nó) KHÔNG BAO GIỜ được thực thi khi `dh` là
`undefined`, vì hàm ĐÃ `return` từ kiểm tra `if (dh === undefined)`
NGAY TRƯỚC ĐÓ. Không có `dhMoi` nào được tạo, không có crash nào xảy
ra — luồng thực thi dừng SỚM, đúng như Sandwich Pattern thiết kế.
::
:::
::::

::::code{#viet_xac_nhan_don_hang_usecase}
Tự viết `xacNhanDonHangUseCase(deps, maDonHang): Result<DonHang, string>`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type DonHang = { maDonHang: string; trangThai: "nhap" | "da_xac_nhan"; tongTien: number };
type KhoDonHang = {
  timTheoId: (id: string) => DonHang | undefined;
  luu: (dh: DonHang) => void;
};
type Deps = { khoDonHang: KhoDonHang };

function xacNhanDonHangUseCase(deps: Deps, maDonHang: string): Result<DonHang, string> {
  const dh = deps.khoDonHang.timTheoId(maDonHang);
  if (dh === undefined) return loi("không tìm thấy đơn hàng");
  if (dh.trangThai !== "nhap") return loi("đơn hàng đã được xác nhận trước đó");
  const dhMoi: DonHang = { ...dh, trangThai: "da_xac_nhan" };
  ___;
  return ___;
}

const boNho = new Map<string, DonHang>([["DH-01", { maDonHang: "DH-01", trangThai: "nhap", tongTien: 100000 }]]);
const khoFake: KhoDonHang = { timTheoId: (id) => boNho.get(id), luu: (dh) => boNho.set(dh.maDonHang, dh) };
console.log(JSON.stringify(xacNhanDonHangUseCase({ khoDonHang: khoFake }, "DH-01")));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type DonHang = { maDonHang: string; trangThai: "nhap" | "da_xac_nhan"; tongTien: number };
type KhoDonHang = {
  timTheoId: (id: string) => DonHang | undefined;
  luu: (dh: DonHang) => void;
};
type Deps = { khoDonHang: KhoDonHang };

function xacNhanDonHangUseCase(deps: Deps, maDonHang: string): Result<DonHang, string> {
  const dh = deps.khoDonHang.timTheoId(maDonHang);
  if (dh === undefined) return loi("không tìm thấy đơn hàng");
  if (dh.trangThai !== "nhap") return loi("đơn hàng đã được xác nhận trước đó");
  const dhMoi: DonHang = { ...dh, trangThai: "da_xac_nhan" };
  deps.khoDonHang.luu(dhMoi);
  return ok(dhMoi);
}

const boNho = new Map<string, DonHang>([["DH-01", { maDonHang: "DH-01", trangThai: "nhap", tongTien: 100000 }]]);
const khoFake: KhoDonHang = { timTheoId: (id) => boNho.get(id), luu: (dh) => boNho.set(dh.maDonHang, dh) };
console.log(JSON.stringify(xacNhanDonHangUseCase({ khoDonHang: khoFake }, "DH-01")));
```

```typescript title=test
const boNho1 = new Map<string, DonHang>([["DH-01", { maDonHang: "DH-01", trangThai: "nhap", tongTien: 100000 }]]);
const kho1: KhoDonHang = { timTheoId: (id) => boNho1.get(id), luu: (dh) => boNho1.set(dh.maDonHang, dh) };

const a = xacNhanDonHangUseCase({ khoDonHang: kho1 }, "DH-01");
if (a.kind !== "ok") throw new Error("đơn hàng đang \"nhap\" phải xác nhận thành công");
if (a.kind === "ok" && a.giaTri.trangThai !== "da_xac_nhan") throw new Error("kết quả phải mang trạng thái đã xác nhận");
if (boNho1.get("DH-01")?.trangThai !== "da_xac_nhan") throw new Error("phải GHI trạng thái mới vào kho, không chỉ trả về giá trị");

const b = xacNhanDonHangUseCase({ khoDonHang: kho1 }, "DH-01");
if (b.kind !== "loi") throw new Error("xác nhận LẦN HAI trên đơn đã xác nhận phải lỗi");

const c = xacNhanDonHangUseCase({ khoDonHang: kho1 }, "DH-KHONG-TON-TAI");
if (c.kind !== "loi") throw new Error("đơn hàng không tồn tại phải lỗi");
```

:::hints
- kind: attention
  body: "Chỗ trống 1: GHI dhMoi vào kho bằng deps.khoDonHang.luu(dhMoi) — chỉ side-effect, không return gì. Chỗ trống 2: TRẢ VỀ kết quả thành công, bọc dhMoi bằng ok(...)."
- kind: strategy
  body: "deps.khoDonHang.luu(dhMoi); return ok(dhMoi); — hai dòng cuối, một IO một Result."
- kind: one-line
  body: "___ (ghi) = deps.khoDonHang.luu(dhMoi)\n___ (trả về) = ok(dhMoi)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "da_xac_nhan"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 2 xong. Kiến trúc FP hoàn chỉnh: Domain thuần, Infra interface do
Domain định nghĩa, use case orchestrate theo Sandwich, test bằng fake
— không class, không container, không mock library.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã biết ĐẶT code Ở ĐÂU (kiến trúc). Giờ quay lại câu hỏi: bên
trong Domain, làm sao MÔ HÌNH HOÁ dữ liệu sao cho trạng thái BẤT HỢP
LỆ không thể nào biểu diễn được?
::::

::::checkpoint{mastery=0.8}
::::
