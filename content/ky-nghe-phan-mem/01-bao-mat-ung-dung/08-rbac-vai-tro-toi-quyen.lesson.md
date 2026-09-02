---
id: ky-nghe-phan-mem.bao-mat-ung-dung.rbac-vai-tro-toi-quyen
title: "RBAC — vai trò sở hữu quyền, kiểm tra là phép thuộc tập hợp"
summary: "Role-Based Access Control: quyền gán cho VAI TRÒ, KHÔNG gán trực tiếp cho TỪNG người dùng — bảng tra quyenTheoVaiTro. Kiểm tra quyền = phép Array.includes (thuộc tập hợp), không if/else lồng nhau theo từng user. Thêm user mới không cần sửa logic phân quyền."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [bmud.rbac]
requires: [bmud.refresh-token-revocation]
concepts: [bmud.rbac]
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
Xác thực xong, biết ĐÚNG người dùng là ai. Nhưng "đọc, ghi, xoá được
gì" — gán quyền TRỰC TIẾP cho TỪNG người, hay có cách GỌN hơn?
::::

::::explain{#vai-tro-so-huu-quyen}
**Role-Based Access Control (RBAC)**: quyền (permission) gán cho
**VAI TRÒ** (role), **KHÔNG** gán trực tiếp cho TỪNG người dùng —
bảng tra `quyenTheoVaiTro: Record<VaiTro, readonly Quyen[]>`:

```typescript
type VaiTro = "admin" | "bien-tap" | "xem";
type Quyen = "doc" | "ghi" | "xoa" | "quan-ly-nguoi-dung";

const quyenTheoVaiTro: Record<VaiTro, readonly Quyen[]> = {
  admin: ["doc", "ghi", "xoa", "quan-ly-nguoi-dung"],
  "bien-tap": ["doc", "ghi"],
  xem: ["doc"],
};

function coQuyen(vaiTro: VaiTro, quyen: Quyen): boolean {
  return quyenTheoVaiTro[vaiTro].includes(quyen);
}

console.log(coQuyen("bien-tap", "xoa"));
console.log(coQuyen("bien-tap", "ghi"));
console.log(coQuyen("xem", "doc"));
```

```text
false
true
true
```

Kiểm tra quyền = phép **`Array.includes`** (thuộc TẬP HỢP quyền của
vai trò ĐÓ) — **KHÔNG** `if`/`else` LỒNG NHAU theo TỪNG người dùng
(`if (user === "An") ... else if (user === "Binh") ...`). Thêm MỘT
user MỚI (ví dụ user thứ 1000) **KHÔNG cần sửa** `coQuyen` hay bảng
`quyenTheoVaiTro` — CHỈ cần GÁN vai trò CÓ SẴN cho user đó.
::::

::::example{#gan-vai-tro-khong-doi-logic}
Thêm user MỚI CHỈ là gán MỘT `VaiTro` — logic phân quyền (`coQuyen`,
`quyenTheoVaiTro`) **KHÔNG ĐỔI MỘT DÒNG**:

```typescript title=readonly
type VaiTro = "admin" | "bien-tap" | "xem";
type Quyen = "doc" | "ghi" | "xoa" | "quan-ly-nguoi-dung";
const quyenTheoVaiTro: Record<VaiTro, readonly Quyen[]> = {
  admin: ["doc", "ghi", "xoa", "quan-ly-nguoi-dung"],
  "bien-tap": ["doc", "ghi"],
  xem: ["doc"],
};
function coQuyen(vaiTro: VaiTro, quyen: Quyen): boolean {
  return quyenTheoVaiTro[vaiTro].includes(quyen);
}

type NguoiDung = { ten: string; vaiTro: VaiTro };

// Thêm user MỚI -- CHỈ gán vaiTro, KHÔNG cần biết chi tiết coQuyen làm gì
const nguoiDungMoi: NguoiDung = { ten: "Nhan vien 1000", vaiTro: "bien-tap" };
console.log(coQuyen(nguoiDungMoi.vaiTro, "ghi"));
console.log(coQuyen(nguoiDungMoi.vaiTro, "quan-ly-nguoi-dung"));
```

```text title=readonly
true
false
```

`nguoiDungMoi` chỉ MANG `vaiTro: "bien-tap"` — `coQuyen` đọc BẢNG
`quyenTheoVaiTro["bien-tap"]` (đã CÓ SẴN, KHÔNG cần thêm gì) để trả
lời. Nếu MAI SAU cần đổi quyền của MỌI "bien-tap" (ví dụ thêm quyền
`"xoa"`), CHỈ SỬA **MỘT DÒNG** trong `quyenTheoVaiTro` — ẢNH HƯỞNG
NGAY tới TẤT CẢ user mang vai trò đó, không cần sửa TỪNG user.
::::

::::predict{#doan-vai-tro-khong-ton-tai commitOnce}
```typescript
type VaiTro = "admin" | "bien-tap" | "xem";
type Quyen = "doc" | "ghi" | "xoa" | "quan-ly-nguoi-dung";
const quyenTheoVaiTro: Record<VaiTro, readonly Quyen[]> = {
  admin: ["doc", "ghi", "xoa", "quan-ly-nguoi-dung"],
  "bien-tap": ["doc", "ghi"],
  xem: ["doc"],
};
function coQuyen(vaiTro: VaiTro, quyen: Quyen): boolean {
  return quyenTheoVaiTro[vaiTro].includes(quyen);
}

// Thử truyền một vai trò KHÔNG nằm trong ba giá trị đã khai
console.log(coQuyen("khach" as VaiTro, "doc"));
```

Chuyện gì xảy ra?

:::opt{correct}
Chương trình crash lúc chạy (đọc `undefined.includes`)
:::

:::opt
`false` — vì `coQuyen` kiểm TOÀN BỘ vai trò HỢP LỆ trước khi tra
bảng, "khach" không nằm trong ba giá trị hợp lệ nên trả về `false`
NGAY, không lỗi gì cả
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"khach"` KHÔNG nằm trong BA giá trị
`VaiTro` hợp lệ (`"admin"`/`"bien-tap"`/`"xem"`) — quan sát đó đúng.

Chỗ lệch: đọc LẠI thân hàm `coQuyen` — nó KHÔNG hề có bước "kiểm vai
trò hợp lệ trước" — CHỈ có MỘT dòng DUY NHẤT:
`quyenTheoVaiTro[vaiTro].includes(quyen)`. `as VaiTro` (ép kiểu, đã
học từ trước) BUỘC TypeScript TIN `"khach"` LÀ một `VaiTro` — bỏ qua
mọi kiểm tra kiểu, KHÔNG kiểm tra LÚC CHẠY. `quyenTheoVaiTro["khach"]`
LÚC CHẠY trả về `undefined` (key không tồn tại trong object), rồi
`.includes(...)` được gọi TRÊN `undefined` — CRASH ("Cannot read
properties of undefined").
::
:::

:::opt
Máy báo lỗi biên dịch — `"khach" as VaiTro` không hợp lệ, TypeScript
chặn việc ép kiểu một chuỗi KHÔNG nằm trong union literal đã khai
::why
Gần đúng ở việc bạn nhớ ĐÚNG `VaiTro` là union LITERAL (chỉ ba giá
trị cụ thể) — một quan sát ĐÚNG về CẤU TRÚC kiểu.

Chỗ lệch: `as` (ép kiểu) là công cụ CỐ Ý VÔ HIỆU HOÁ một PHẦN kiểm
tra kiểu — TypeScript CHO PHÉP ép MỘT `string` sang MỘT union literal
CỦA `string` (không phải ép sang một kiểu HOÀN TOÀN không liên quan,
như `string as number` mới bị chặn). Đây CHÍNH LÀ nguy hiểm của `as`:
nó là LỜI HỨA của người viết code với compiler, KHÔNG được KIỂM TRA
THẬT — nếu lời hứa SAI (như ở đây), lỗi CHỈ lộ ra LÚC CHẠY.
::
:::
::::

::::code{#viet_coquyen}
Tự viết `coQuyen`.

```typescript title=starter
type VaiTro = "admin" | "bien-tap" | "xem";
type Quyen = "doc" | "ghi" | "xoa" | "quan-ly-nguoi-dung";

const quyenTheoVaiTro: Record<VaiTro, readonly Quyen[]> = {
  admin: ["doc", "ghi", "xoa", "quan-ly-nguoi-dung"],
  "bien-tap": ["doc", "ghi"],
  xem: ["doc"],
};

function coQuyen(vaiTro: VaiTro, quyen: Quyen): boolean {
  return ___;
}

console.log(coQuyen("bien-tap", "ghi"));
```

```typescript title=solution
type VaiTro = "admin" | "bien-tap" | "xem";
type Quyen = "doc" | "ghi" | "xoa" | "quan-ly-nguoi-dung";

const quyenTheoVaiTro: Record<VaiTro, readonly Quyen[]> = {
  admin: ["doc", "ghi", "xoa", "quan-ly-nguoi-dung"],
  "bien-tap": ["doc", "ghi"],
  xem: ["doc"],
};

function coQuyen(vaiTro: VaiTro, quyen: Quyen): boolean {
  return quyenTheoVaiTro[vaiTro].includes(quyen);
}

console.log(coQuyen("bien-tap", "ghi"));
```

```typescript title=test
if (coQuyen("admin", "quan-ly-nguoi-dung") !== true) throw new Error("admin phải có mọi quyền, kể cả quan-ly-nguoi-dung");
if (coQuyen("bien-tap", "ghi") !== true) throw new Error("bien-tap phải có quyền ghi");
if (coQuyen("bien-tap", "xoa") !== false) throw new Error("bien-tap KHÔNG được có quyền xoa");
if (coQuyen("xem", "doc") !== true) throw new Error("xem phải có quyền doc");
if (coQuyen("xem", "ghi") !== false) throw new Error("xem KHÔNG được có quyền ghi");
```

:::hints
- kind: attention
  body: "Tra bảng quyenTheoVaiTro theo vaiTro, rồi kiểm quyen có nằm trong mảng đó không (Array.includes)."
- kind: strategy
  body: "quyenTheoVaiTro[vaiTro].includes(quyen) — một biểu thức duy nhất: tra bảng rồi kiểm thuộc tập hợp."
- kind: one-line
  body: "___ = quyenTheoVaiTro[vaiTro].includes(quyen)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
RBAC: quyền gán cho vai trò, kiểm tra là phép thuộc tập hợp. Bước
tiếp theo: trả về boolean trần trụi hay giàu ngữ cảnh hơn?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`coQuyen` trả `boolean` trần trụi — `false` không nói RÕ TẠI SAO bị
từ chối. Bọc kết quả thành một DU giàu ngữ cảnh (nối phong cách ROP
đã học) trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
