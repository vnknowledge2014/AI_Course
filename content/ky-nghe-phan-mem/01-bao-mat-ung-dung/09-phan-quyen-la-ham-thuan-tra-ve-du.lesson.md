---
id: ky-nghe-phan-mem.bao-mat-ung-dung.phan-quyen-la-ham-thuan-tra-ve-du
title: "authorize() như hàm thuần trả DU — không side-effect, test không cần mock"
summary: "Bọc kết quả boolean của coQuyen thành DU giàu ngữ cảnh {duocPhep:true} | {duocPhep:false, lyDo} — nối phong cách ROP đã học. Trả LÝ DO cụ thể thay vì false trần trụi. phanQuyen là hàm THUẦN — test được trực tiếp, không cần mock gì."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [bmud.authorize-pure-du]
requires: [bmud.rbac]
concepts: [bmud.authorize-pure-du]
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
`coQuyen` trả `boolean` trần trụi — `false` không nói RÕ tại sao bị
từ chối. Người dùng thấy lỗi "false" thì hiểu gì?
::::

::::explain{#du-giau-ngu-canh}
Bọc kết quả `coQuyen` (boolean, bài 8) thành **DU giàu ngữ cảnh**
`KetQuaPhanQuyen = {duocPhep:true} | {duocPhep:false, lyDo:string}` —
nối thẳng phong cách **ROP** (Railway-Oriented Programming) đã học ở
track DDD: thay vì trả `false` TRẦN TRỤI, trả **LÝ DO CỤ THỂ**:

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

type KetQuaPhanQuyen = { duocPhep: true } | { duocPhep: false; lyDo: string };

function phanQuyen(vaiTro: VaiTro, quyenCanCo: Quyen): KetQuaPhanQuyen {
  return coQuyen(vaiTro, quyenCanCo)
    ? { duocPhep: true }
    : { duocPhep: false, lyDo: `vai trò '${vaiTro}' thiếu quyền '${quyenCanCo}'` };
}

console.log(JSON.stringify(phanQuyen("bien-tap", "xoa")));
console.log(JSON.stringify(phanQuyen("admin", "doc")));
```

```text
{"duocPhep":false,"lyDo":"vai trò 'bien-tap' thiếu quyền 'xoa'"}
{"duocPhep":true}
```

`phanQuyen("bien-tap", "xoa")` trả `lyDo` CỤ THỂ — `"vai trò
'bien-tap' thiếu quyền 'xoa'"` — LOG/hiển thị cho người dùng ĐỌC được
NGAY LÝ DO bị từ chối, KHÔNG PHẢI đoán "sao lại false vậy?".
::::

::::example{#ham-thuan-test-khong-can-mock}
`phanQuyen` là hàm **THUẦN** (không đọc database, không gọi API,
không side-effect) — test được **TRỰC TIẾP**, **KHÔNG CẦN mock** bất
kỳ thứ gì:

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
type KetQuaPhanQuyen = { duocPhep: true } | { duocPhep: false; lyDo: string };
function phanQuyen(vaiTro: VaiTro, quyenCanCo: Quyen): KetQuaPhanQuyen {
  return coQuyen(vaiTro, quyenCanCo)
    ? { duocPhep: true }
    : { duocPhep: false, lyDo: `vai trò '${vaiTro}' thiếu quyền '${quyenCanCo}'` };
}

// Test TRỰC TIẾP -- KHÔNG database giả, KHÔNG mock framework nào
const cacTruongHop: Array<[VaiTro, Quyen, boolean]> = [
  ["admin", "xoa", true],
  ["xem", "ghi", false],
  ["bien-tap", "doc", true],
];
for (const [vaiTro, quyen, kyVong] of cacTruongHop) {
  const ketQua = phanQuyen(vaiTro, quyen);
  console.log(ketQua.duocPhep === kyVong);
}
```

```text title=readonly
true
true
true
```

Test CHỈ CẦN GỌI `phanQuyen` với input CỤ THỂ và SO SÁNH kết quả —
KHÔNG cần dựng database giả, KHÔNG cần thư viện mocking, KHÔNG cần
setup phức tạp. Đây LÀ lợi ích trực tiếp của hàm THUẦN (đã học xuyên
suốt track FP): input CỐ ĐỊNH → output CỐ ĐỊNH, test được NGAY.
::::

::::predict{#doan-hai-nhanh-tra-kieu-khac-nhau commitOnce}
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
type KetQuaPhanQuyen = { duocPhep: true } | { duocPhep: false; lyDo: string };
function phanQuyen(vaiTro: VaiTro, quyenCanCo: Quyen): KetQuaPhanQuyen {
  return coQuyen(vaiTro, quyenCanCo)
    ? { duocPhep: true }
    : { duocPhep: false, lyDo: `vai trò '${vaiTro}' thiếu quyền '${quyenCanCo}'` };
}

const ketQua = phanQuyen("xem", "xoa");
console.log("lyDo" in ketQua);
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
`false` — vì `ketQua` có kiểu KHAI BÁO là `KetQuaPhanQuyen` (một DU
CHUNG), và toán tử `in` chỉ kiểm được trên MỘT kiểu OBJECT CỤ THỂ,
không kiểm được trên union
::why
Gần đúng ở việc bạn để ý `KetQuaPhanQuyen` LÀ một union (hai nhánh
KHÁC hình dạng) — một quan sát ĐÚNG về CẤU TRÚC kiểu.

Chỗ lệch: toán tử `in` (kiểm MỘT property CÓ tồn tại LÚC CHẠY hay
không) hoạt động trên GIÁ TRỊ THỰC, không quan tâm kiểu KHAI BÁO là
union hay không. `phanQuyen("xem", "xoa")`: `coQuyen("xem", "xoa")`
là `false` (vai trò "xem" CHỈ có quyền "doc") — nhánh `{ duocPhep:
false, lyDo: ... }` được TRẢ VỀ, object THẬT SỰ CÓ field `lyDo`.
`"lyDo" in ketQua` kiểm ĐÚNG object CỤ THỂ đó LÚC CHẠY — `true`.
::
:::

:::opt
Máy báo lỗi biên dịch — dùng `in` để kiểm field `lyDo` (chỉ tồn tại ở
MỘT nhánh của DU) mà không thu hẹp kiểu `ketQua` trước bị TypeScript
chặn, giống lỗi đọc trực tiếp field ở bài 1
::why
Gần đúng ở việc bạn LIÊN TƯỞNG tới bài 1 (đọc trực tiếp field CHỈ
tồn tại ở MỘT nhánh DU bị chặn khi CHƯA thu hẹp kiểu) — một kết nối
hợp lý, vì tình huống BỀ NGOÀI khá giống.

Chỗ lệch: `"lyDo" in ketQua` (dùng toán tử `in`) là CHÍNH XÁC MỘT
TRONG những cách TypeScript CÔNG NHẬN để **THU HẸP KIỂU** — nó KHÔNG
bị chặn NHƯ đọc trực tiếp `ketQua.lyDo`. Viết `console.log(ketQua.lyDo)`
(không qua `in`) MỚI bị lỗi TS2339 — nhưng CÂU HỎI ở đây dùng `in`,
một biểu thức KIỂM TRA hợp lệ, không phải TRUY CẬP trực tiếp. Biên
dịch sạch.
::
:::
::::

::::code{#viet_phanquyen}
Tự viết `phanQuyen`.

```typescript title=starter
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

type KetQuaPhanQuyen = { duocPhep: true } | { duocPhep: false; lyDo: string };

function phanQuyen(vaiTro: VaiTro, quyenCanCo: Quyen): KetQuaPhanQuyen {
  return coQuyen(vaiTro, quyenCanCo)
    ? ___
    : ___;
}

console.log(JSON.stringify(phanQuyen("xem", "xoa")));
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

type KetQuaPhanQuyen = { duocPhep: true } | { duocPhep: false; lyDo: string };

function phanQuyen(vaiTro: VaiTro, quyenCanCo: Quyen): KetQuaPhanQuyen {
  return coQuyen(vaiTro, quyenCanCo)
    ? { duocPhep: true }
    : { duocPhep: false, lyDo: `vai trò '${vaiTro}' thiếu quyền '${quyenCanCo}'` };
}

console.log(JSON.stringify(phanQuyen("xem", "xoa")));
```

```typescript title=test
const duocPhep = phanQuyen("admin", "xoa");
if (duocPhep.duocPhep !== true) throw new Error("admin xoá phải được phép");

const khongDuocPhep = phanQuyen("xem", "ghi");
if (khongDuocPhep.duocPhep !== false) throw new Error("xem ghi phải bị từ chối");
if (khongDuocPhep.duocPhep === false && (!khongDuocPhep.lyDo || khongDuocPhep.lyDo.trim() === "")) throw new Error("lý do từ chối không được rỗng");
if (khongDuocPhep.duocPhep === false && !khongDuocPhep.lyDo.includes("xem")) throw new Error("lý do phải nhắc tới vai trò bị từ chối");
if (khongDuocPhep.duocPhep === false && !khongDuocPhep.lyDo.includes("ghi")) throw new Error("lý do phải nhắc tới quyền bị thiếu");
```

:::hints
- kind: attention
  body: "Nhánh coQuyen trả true: bọc { duocPhep: true }. Nhánh false: bọc { duocPhep: false, lyDo: ... } kèm thông điệp NHẮC ĐÚNG vai trò và quyền bị thiếu."
- kind: strategy
  body: '{ duocPhep: true } : { duocPhep: false, lyDo: `vai trò \'${vaiTro}\' thiếu quyền \'${quyenCanCo}\'` } — hai nhánh của phanQuyen.'
- kind: one-line
  body: '___ (được phép) = { duocPhep: true }\n___ (bị từ chối) = { duocPhep: false, lyDo: `vai trò \'${vaiTro}\' thiếu quyền \'${quyenCanCo}\'` }'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "lyDo"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
authorize() là hàm thuần trả DU — không side-effect, test không cần
mock. Bước tiếp theo: khi RBAC quá THÔ, cần thứ tinh vi hơn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

RBAC chỉ biết "vai trò". Nhưng đôi khi cần "user chỉ được sửa tài
nguyên CỦA CHÍNH MÌNH" — RBAC (chỉ dựa vào vai trò) trả lời được câu
đó không?
::::

::::checkpoint{mastery=0.8}
::::
