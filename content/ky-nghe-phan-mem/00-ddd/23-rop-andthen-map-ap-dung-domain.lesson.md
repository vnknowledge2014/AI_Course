---
id: ky-nghe-phan-mem.ddd.rop-andthen-map-ap-dung-domain
title: "ROP áp dụng vào domain — andThen chính là chainResult đã học"
summary: "Railway-Oriented Programming: track chính (ok) và track lỗi (loi) song song, một trạm lỗi khiến các trạm SAU bị SKIP. andThen (sách) CHÍNH LÀ chainResult (T4.5) — không dạy lại lý thuyết, áp dụng vào domain với lỗi giàu ngữ cảnh."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 23
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.rop-domain-application]
requires: [ddd.tap]
concepts: [ddd.rop-domain-application]
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
`pipe`/`flow`/`tap` ghép hàm THUẦN. Bước nào đó CÓ THỂ LỖI thì công cụ
nào ghép được?
::::

::::explain{#rop-la-chainresult}
**Railway-Oriented Programming (ROP)**: ẩn dụ đường ray — track CHÍNH
(`ok`) và track LỖI (`loi`) chạy SONG SONG; khi một trạm PHÁT HIỆN lỗi,
CÁC TRẠM CÒN LẠI bị SKIP tự động, "tàu" chuyển hẳn sang track lỗi.

Nhiều tài liệu gọi công cụ ghép các trạm CÓ THỂ LỖI là `andThen` — cái
tên ĐÓ CHÍNH LÀ `chainResult` bạn ĐÃ HỌC ở T4.5 (bài 30) — CÙNG khuôn,
KHÔNG có gì mới về lý thuyết:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

// Lỗi mô hình hoá bằng DU GIÀU NGỮ CẢNH (đã học), KHÔNG phải string đơn
type LoiDonHang =
  | { tag: "loi_khach_hang"; thongDiep: string }
  | { tag: "loi_ton_kho"; maSanPham: string; conLai: number; canMua: number };

type DonHangTho = { maKhachHang: string; maSanPham: string; soLuong: number };
type DonHangDaKiem = { maKhachHang: string; maSanPham: string; soLuong: number };
const TON_KHO: Record<string, number> = { "SP-01": 3, "SP-02": 100 };

function validateKhachHang(dh: DonHangTho): Result<DonHangTho, LoiDonHang> {
  if (dh.maKhachHang.trim() === "") return loi({ tag: "loi_khach_hang", thongDiep: "thiếu mã khách hàng" });
  return ok(dh);
}
function kiemKho(dh: DonHangTho): Result<DonHangDaKiem, LoiDonHang> {
  const conLai = TON_KHO[dh.maSanPham] ?? 0;
  if (conLai < dh.soLuong) {
    return loi({ tag: "loi_ton_kho", maSanPham: dh.maSanPham, conLai, canMua: dh.soLuong });
  }
  return ok(dh);
}

const donHangHetHang: DonHangTho = { maKhachHang: "KH-01", maSanPham: "SP-01", soLuong: 5 };
const ketQua = chainResult(validateKhachHang(donHangHetHang), kiemKho);
console.log(JSON.stringify(ketQua));
```

```text
{"kind":"loi","loi":{"tag":"loi_ton_kho","maSanPham":"SP-01","conLai":3,"canMua":5}}
```

Lỗi ghi RÕ: `maSanPham` nào, CÒN LẠI bao nhiêu, CẦN MUA bao nhiêu —
KHÔNG phải chuỗi mập mờ `"lỗi tồn kho"`. Đây LÀ Ubiquitous Language
(bài 2) áp dụng CHO LỖI: tên field ĐÚNG ngôn ngữ domain expert dùng.
::::

::::example{#short-circuit-o-domain}
Lỗi Ở BƯỚC ĐẦU khiến trạm SAU bị SKIP hoàn toàn — nối thẳng T4.5's
short-circuit (bài 31), giờ áp dụng cho lỗi domain THẬT:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}
type LoiDonHang = { tag: "loi_khach_hang"; thongDiep: string } | { tag: "loi_ton_kho"; maSanPham: string; conLai: number; canMua: number };
type DonHangTho = { maKhachHang: string; maSanPham: string; soLuong: number };
type DonHangDaKiem = { maKhachHang: string; maSanPham: string; soLuong: number };
const TON_KHO: Record<string, number> = { "SP-01": 3 };
function validateKhachHang(dh: DonHangTho): Result<DonHangTho, LoiDonHang> {
  if (dh.maKhachHang.trim() === "") return loi({ tag: "loi_khach_hang", thongDiep: "thiếu mã khách hàng" });
  return ok(dh);
}
function kiemKho(dh: DonHangTho): Result<DonHangDaKiem, LoiDonHang> {
  const conLai = TON_KHO[dh.maSanPham] ?? 0;
  if (conLai < dh.soLuong) return loi({ tag: "loi_ton_kho", maSanPham: dh.maSanPham, conLai, canMua: dh.soLuong });
  return ok(dh);
}

let soLanGoiKiemKho = 0;
const kiemKhoCoDem = (dh: DonHangTho): Result<DonHangDaKiem, LoiDonHang> => {
  soLanGoiKiemKho = soLanGoiKiemKho + 1;
  return kiemKho(dh);
};

const donHangThieuKH: DonHangTho = { maKhachHang: "", maSanPham: "SP-01", soLuong: 1 };
const ketQua = chainResult(validateKhachHang(donHangThieuKH), kiemKhoCoDem);
console.log(JSON.stringify(ketQua));
console.log(soLanGoiKiemKho);
```

```text title=readonly
{"kind":"loi","loi":{"tag":"loi_khach_hang","thongDiep":"thiếu mã khách hàng"}}
0
```

`validateKhachHang` lỗi NGAY (thiếu mã khách hàng) — `kiemKhoCoDem`
(trạm KIỂM TRA TỒN KHO) KHÔNG BAO GIỜ được gọi (`soLanGoiKiemKho`
giữ nguyên `0`), dù `SP-01` CÓ tồn tại trong kho. "Tàu" đã CHUYỂN sang
track lỗi ngay từ trạm ĐẦU TIÊN.
::::

::::predict{#doan-hai-loi-cung-luc commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}
type LoiDonHang = { tag: "loi_khach_hang"; thongDiep: string } | { tag: "loi_ton_kho"; maSanPham: string; conLai: number; canMua: number };
type DonHangTho = { maKhachHang: string; maSanPham: string; soLuong: number };
type DonHangDaKiem = { maKhachHang: string; maSanPham: string; soLuong: number };
const TON_KHO: Record<string, number> = { "SP-99": 0 };
function validateKhachHang(dh: DonHangTho): Result<DonHangTho, LoiDonHang> {
  if (dh.maKhachHang.trim() === "") return loi({ tag: "loi_khach_hang", thongDiep: "thiếu mã khách hàng" });
  return ok(dh);
}
function kiemKho(dh: DonHangTho): Result<DonHangDaKiem, LoiDonHang> {
  const conLai = TON_KHO[dh.maSanPham] ?? 0;
  if (conLai < dh.soLuong) return loi({ tag: "loi_ton_kho", maSanPham: dh.maSanPham, conLai, canMua: dh.soLuong });
  return ok(dh);
}

// CẢ HAI đều sai: thiếu mã khách hàng VÀ hết hàng
const donHangHaiLoi: DonHangTho = { maKhachHang: "", maSanPham: "SP-99", soLuong: 10 };
const ketQua = chainResult(validateKhachHang(donHangHaiLoi), kiemKho);
console.log(ketQua.kind === "loi" ? ketQua.loi.tag : "khong loi");
```

Dòng cuối in ra gì (đơn hàng sai CẢ HAI điều kiện)?

:::opt{correct}
`loi_khach_hang`
:::

:::opt
`loi_ton_kho` — vì `soLuong` sai LỆCH nhiều hơn (thiếu 10, trong khi
`maKhachHang` chỉ thiếu MỘT chuỗi rỗng), nên lỗi "nghiêm trọng hơn"
được ưu tiên báo trước
::why
Gần đúng ở việc bạn nghĩ tới việc CÓ THỂ có một cơ chế "ưu tiên lỗi
nghiêm trọng hơn" — một Ý HỢP LÝ cho một số hệ thống báo lỗi khác.

Chỗ lệch: `chainResult` KHÔNG so sánh "mức độ nghiêm trọng" của lỗi —
nó CHỈ chạy TUẦN TỰ, dừng NGAY tại lỗi ĐẦU TIÊN THEO THỨ TỰ trong
chuỗi (`validateKhachHang` TRƯỚC, `kiemKho` SAU). `validateKhachHang`
lỗi TRƯỚC (`maKhachHang` rỗng), `chainResult` `return loi(...)` NGAY,
`kiemKho` KHÔNG BAO GIỜ được gọi — dù đơn hàng CŨNG hết hàng, lỗi ĐÓ
không bao giờ được PHÁT HIỆN (không phải "không nghiêm trọng", mà đơn
giản KHÔNG BAO GIỜ được kiểm tới).
::
:::

:::opt
Máy báo lỗi biên dịch — `LoiDonHang` không cho phép MỘT đơn hàng có
CẢ HAI lỗi CÙNG lúc, vì nó là discriminated union (chỉ MỘT tag)
::why
Gần đúng ở việc bạn nhớ ĐÚNG `LoiDonHang` là discriminated union, CHỈ
mang MỘT `tag` tại một thời điểm — quan sát về cấu trúc đó đúng.

Chỗ lệch: `donHangHaiLoi` (một `DonHangTho`) KHÔNG chứa lỗi nào cả —
nó chỉ là DỮ LIỆU THÔ chưa validate, CÓ THỂ VI PHẠM nhiều quy tắc.
`LoiDonHang` chỉ xuất hiện SAU KHI validate PHÁT HIỆN MỘT lỗi CỤ THỂ
— và vì `chainResult` dừng Ở LỖI ĐẦU TIÊN, KẾT QUẢ CUỐI luôn CHỈ mang
MỘT `LoiDonHang`, không có mâu thuẫn gì với thiết kế discriminated
union.
::
:::
::::

::::code{#viet_validate_va_kiem_kho}
Tự viết `validateKhachHang` và `kiemKho`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

type LoiDonHang =
  | { tag: "loi_khach_hang"; thongDiep: string }
  | { tag: "loi_ton_kho"; maSanPham: string; conLai: number; canMua: number };
type DonHangTho = { maKhachHang: string; maSanPham: string; soLuong: number };
type DonHangDaKiem = { maKhachHang: string; maSanPham: string; soLuong: number };
const TON_KHO: Record<string, number> = { "SP-01": 3, "SP-02": 100 };

function validateKhachHang(dh: DonHangTho): Result<DonHangTho, LoiDonHang> {
  if (dh.maKhachHang.trim() === "") return ___;
  return ___;
}

function kiemKho(dh: DonHangTho): Result<DonHangDaKiem, LoiDonHang> {
  const conLai = TON_KHO[dh.maSanPham] ?? 0;
  if (conLai < dh.soLuong) {
    return ___;
  }
  return ___;
}

console.log(JSON.stringify(chainResult(validateKhachHang({ maKhachHang: "KH-01", maSanPham: "SP-02", soLuong: 5 }), kiemKho)));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

type LoiDonHang =
  | { tag: "loi_khach_hang"; thongDiep: string }
  | { tag: "loi_ton_kho"; maSanPham: string; conLai: number; canMua: number };
type DonHangTho = { maKhachHang: string; maSanPham: string; soLuong: number };
type DonHangDaKiem = { maKhachHang: string; maSanPham: string; soLuong: number };
const TON_KHO: Record<string, number> = { "SP-01": 3, "SP-02": 100 };

function validateKhachHang(dh: DonHangTho): Result<DonHangTho, LoiDonHang> {
  if (dh.maKhachHang.trim() === "") return loi({ tag: "loi_khach_hang", thongDiep: "thiếu mã khách hàng" });
  return ok(dh);
}

function kiemKho(dh: DonHangTho): Result<DonHangDaKiem, LoiDonHang> {
  const conLai = TON_KHO[dh.maSanPham] ?? 0;
  if (conLai < dh.soLuong) {
    return loi({ tag: "loi_ton_kho", maSanPham: dh.maSanPham, conLai, canMua: dh.soLuong });
  }
  return ok(dh);
}

console.log(JSON.stringify(chainResult(validateKhachHang({ maKhachHang: "KH-01", maSanPham: "SP-02", soLuong: 5 }), kiemKho)));
```

```typescript title=test
const hopLe = chainResult(validateKhachHang({ maKhachHang: "KH-01", maSanPham: "SP-02", soLuong: 5 }), kiemKho);
if (hopLe.kind !== "ok") throw new Error("khách hàng hợp lệ, đủ hàng phải ra ok");

const thieuKH = chainResult(validateKhachHang({ maKhachHang: "", maSanPham: "SP-02", soLuong: 5 }), kiemKho);
if (thieuKH.kind !== "loi") throw new Error("thiếu mã khách hàng phải ra loi");
if (thieuKH.kind === "loi" && thieuKH.loi.tag !== "loi_khach_hang") throw new Error("lỗi phải đúng tag loi_khach_hang");

const hetHang = chainResult(validateKhachHang({ maKhachHang: "KH-01", maSanPham: "SP-01", soLuong: 10 }), kiemKho);
if (hetHang.kind !== "loi") throw new Error("số lượng vượt tồn kho phải ra loi");
if (hetHang.kind === "loi" && hetHang.loi.tag === "loi_ton_kho" && hetHang.loi.conLai !== 3) throw new Error("lỗi tồn kho phải ghi đúng số lượng còn lại");
```

:::hints
- kind: attention
  body: "validateKhachHang: nhánh lỗi bọc loi({tag: \"loi_khach_hang\", thongDiep: ...}); nhánh hợp lệ trả ok(dh). kiemKho: nhánh lỗi bọc loi({tag: \"loi_ton_kho\", maSanPham, conLai, canMua: dh.soLuong}); nhánh hợp lệ trả ok(dh)."
- kind: strategy
  body: 'loi({ tag: "loi_khach_hang", thongDiep: "thiếu mã khách hàng" }) : ok(dh) — validateKhachHang. loi({ tag: "loi_ton_kho", maSanPham: dh.maSanPham, conLai, canMua: dh.soLuong }) : ok(dh) — kiemKho.'
- kind: one-line
  body: '___ (validateKhachHang lỗi) = loi({ tag: "loi_khach_hang", thongDiep: "thiếu mã khách hàng" })\n___ (validateKhachHang ok) = ok(dh)\n___ (kiemKho lỗi) = loi({ tag: "loi_ton_kho", maSanPham: dh.maSanPham, conLai, canMua: dh.soLuong })\n___ (kiemKho ok) = ok(dh)'
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
ROP: `andThen` chính là `chainResult` — không có lý thuyết mới, chỉ áp
dụng công cụ đã học vào lỗi domain giàu ngữ cảnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có TẤT CẢ công cụ pipeline. Ghép chúng thành MỘT workflow HOÀN
CHỈNH, nhiều bước, đọc như quy trình nghiệp vụ thật — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
