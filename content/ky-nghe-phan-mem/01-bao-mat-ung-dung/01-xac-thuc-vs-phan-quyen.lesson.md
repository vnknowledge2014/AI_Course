---
id: ky-nghe-phan-mem.bao-mat-ung-dung.xac-thuc-vs-phan-quyen
title: "Authentication vs Authorization — bạn LÀ ai, bạn ĐƯỢC làm gì"
summary: "Xác thực (authentication — BẠN LÀ AI) và phân quyền (authorization — BẠN ĐƯỢC LÀM GÌ) là HAI kiểm tra TÁCH BIỆT, ghép được với nhau. xacThuc và phanQuyen là hai hàm ĐỘC LẬP — xác thực THÀNH CÔNG không đảm bảo phân quyền THÀNH CÔNG."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [bmud.authn-vs-authz]
requires: [ts.adt-gate-boss]
concepts: [bmud.authn-vs-authz]
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
Track mới. Một ứng dụng hỏi hai câu HOÀN TOÀN khác nhau: "bạn là AI?"
và "bạn được làm GÌ?" — vì sao phải tách RIÊNG?
::::

::::explain{#authn-vs-authz}
**Authentication** (xác thực — BẠN LÀ AI) và **Authorization** (phân
quyền — BẠN ĐƯỢC LÀM GÌ) là HAI kiểm tra TÁCH BIỆT, GHÉP được với
nhau — ẩn dụ sân bay: xác thực = kiểm tra HỘ CHIẾU ở an ninh (xác
nhận danh tính), phân quyền = THẺ LÊN MÁY BAY (xác nhận bạn được vào
CỔNG nào):

```typescript
type KetQuaXacThuc = { daXacThuc: true; nguoiDung: string } | { daXacThuc: false };
type KetQuaPhanQuyen = { duocPhep: true } | { duocPhep: false; lyDo: string };

function xacThuc(token: string): KetQuaXacThuc {
  return token === "TOKEN_HOP_LE" ? { daXacThuc: true, nguoiDung: "An" } : { daXacThuc: false };
}
function phanQuyen(vaiTro: string): KetQuaPhanQuyen {
  return vaiTro === "admin" ? { duocPhep: true } : { duocPhep: false, lyDo: `vai trò '${vaiTro}' không đủ quyền` };
}

console.log(JSON.stringify(xacThuc("TOKEN_HOP_LE")));
console.log(JSON.stringify(phanQuyen("bien-tap")));
```

```text
{"daXacThuc":true,"nguoiDung":"An"}
{"duocPhep":false,"lyDo":"vai trò 'bien-tap' không đủ quyền"}
```

`xacThuc` VÀ `phanQuyen` là **HAI HÀM ĐỘC LẬP** — hàm THỨ HAI KHÔNG hề
gọi hàm THỨ NHẤT, KHÔNG chia sẻ logic nào. Một người dùng ĐÃ xác thực
THÀNH CÔNG (đúng LÀ họ, có `token` hợp lệ) VẪN CÓ THỂ bị phân quyền
TỪ CHỐI (đúng là họ, NHƯNG vai trò `"bien-tap"` không đủ quyền làm
việc ĐANG YÊU CẦU) — đây LÀ lý do tách hai khái niệm: MỘT hệ thống
xác thực TỐT không tự động nghĩa là phân quyền ĐÚNG, và ngược lại.
::::

::::example{#doc-ket-qua-sai-nhanh}
Đọc TRỰC TIẾP một field CHỈ tồn tại ở MỘT nhánh của DU (không kiểm
`daXacThuc` trước) bị TypeScript CHẶN NGAY lúc biên dịch:

```typescript title=readonly
type KetQuaXacThuc = { daXacThuc: true; nguoiDung: string } | { daXacThuc: false };
function xacThuc(token: string): KetQuaXacThuc {
  return token === "TOKEN_HOP_LE" ? { daXacThuc: true, nguoiDung: "An" } : { daXacThuc: false };
}

const ketQua = xacThuc("token-bat-ky");
// Đọc nguoiDung TRỰC TIẾP, KHÔNG kiểm daXacThuc trước
console.log(ketQua.nguoiDung);
```

```text title=readonly
TS2339: Property 'nguoiDung' does not exist on type 'KetQuaXacThuc'.
Property 'nguoiDung' does not exist on type '{ daXacThuc: false; }'.
```

`nguoiDung` CHỈ tồn tại khi `daXacThuc: true` — TypeScript KHÔNG cho
đọc field đó khi CHƯA thu hẹp kiểu (kiểm `ketQua.daXacThuc === true`
hoặc `switch` TRƯỚC). Đây LÀ discriminated union (đã học từ T4.3) —
CHÍNH KIỂU DỮ LIỆU buộc code PHẢI kiểm tra "đã xác thực hay chưa"
TRƯỚC KHI dùng thông tin người dùng, không thể VÔ TÌNH quên.
::::

::::predict{#doan-xac-thuc-thanh-cong-phan-quyen-that-bai commitOnce}
```typescript
type KetQuaXacThuc = { daXacThuc: true; nguoiDung: string } | { daXacThuc: false };
type KetQuaPhanQuyen = { duocPhep: true } | { duocPhep: false; lyDo: string };
function xacThuc(token: string): KetQuaXacThuc {
  return token === "TOKEN_HOP_LE" ? { daXacThuc: true, nguoiDung: "An" } : { daXacThuc: false };
}
function phanQuyen(vaiTro: string): KetQuaPhanQuyen {
  return vaiTro === "admin" ? { duocPhep: true } : { duocPhep: false, lyDo: `vai trò '${vaiTro}' không đủ quyền` };
}

// An CÓ token hợp lệ (xác thực ĐÚNG) nhưng vai trò "xem" (chỉ đọc)
const ketQuaXT = xacThuc("TOKEN_HOP_LE");
const ketQuaPQ = phanQuyen("xem");
console.log(ketQuaXT.daXacThuc);
console.log(ketQuaPQ.duocPhep);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`true` rồi `false`
:::

:::opt
`true` rồi `true` — vì `xacThuc` ĐÃ xác nhận An LÀ người dùng hợp lệ,
`phanQuyen` chỉ kiểm LẠI danh tính đó, không có lý do gì để từ chối
một người dùng ĐÃ xác thực thành công
::why
Gần đúng ở việc bạn nhớ ĐÚNG `xacThuc("TOKEN_HOP_LE")` xác nhận An LÀ
người dùng hợp lệ (`daXacThuc: true`) — quan sát về BƯỚC xác thực
đó đúng.

Chỗ lệch: `phanQuyen("xem")` KHÔNG hề nhận bất kỳ thông tin nào TỪ
`ketQuaXT` — nó CHỈ nhận tham số `vaiTro` (ở đây LÀ chuỗi `"xem"`),
HOÀN TOÀN KHÔNG liên quan tới việc `xacThuc` vừa trả gì. `phanQuyen`
kiểm `vaiTro === "admin"` — `"xem"` KHÔNG khớp `"admin"`, hàm trả
`{duocPhep: false, lyDo: ...}`. Xác thực THÀNH CÔNG (biết ĐÚNG là An)
KHÔNG tự động nghĩa là được PHÉP làm MỌI VIỆC — đây CHÍNH LÀ bài học
cốt lõi: hai khái niệm HOÀN TOÀN tách biệt.
::
:::

:::opt
Máy báo lỗi biên dịch — không thể gọi `phanQuyen` mà không TRUYỀN kết
quả của `xacThuc` vào trước, vì phân quyền LUÔN phải PHỤ THUỘC vào
việc xác thực đã xảy ra
::why
Gần đúng ở việc bạn nghĩ tới một RÀNG BUỘC NGHIỆP VỤ hợp lý — trong
hệ thống THẬT, thường CHỈ phân quyền SAU KHI xác thực thành công, một
trực giác đúng đắn về THỨ TỰ nên làm.

Chỗ lệch: chữ ký hàm `phanQuyen(vaiTro: string): KetQuaPhanQuyen`
KHÔNG hề khai bất kỳ tham số nào liên quan `KetQuaXacThuc` — về mặt
KIỂU, hai hàm HOÀN TOÀN độc lập, gọi RIÊNG được, KHÔNG có ràng buộc
"phải gọi xacThuc trước". Việc ĐẢM BẢO thứ tự đó (nếu cần) là trách
nhiệm của CODE GỌI cả hai hàm, không phải của bản thân `phanQuyen`.
Biên dịch sạch.
::
:::
::::

::::code{#viet_xacthuc_va_phanquyen}
Tự viết hai hàm `xacThuc` và `phanQuyen`.

```typescript title=starter
type KetQuaXacThuc = { daXacThuc: true; nguoiDung: string } | { daXacThuc: false };
type KetQuaPhanQuyen = { duocPhep: true } | { duocPhep: false; lyDo: string };

function xacThuc(token: string): KetQuaXacThuc {
  if (token === "TOKEN_HOP_LE") return ___;
  return ___;
}
function phanQuyen(vaiTro: string): KetQuaPhanQuyen {
  if (vaiTro === "admin") return ___;
  return ___;
}

console.log(JSON.stringify(xacThuc("TOKEN_HOP_LE")));
console.log(JSON.stringify(phanQuyen("admin")));
```

```typescript title=solution
type KetQuaXacThuc = { daXacThuc: true; nguoiDung: string } | { daXacThuc: false };
type KetQuaPhanQuyen = { duocPhep: true } | { duocPhep: false; lyDo: string };

function xacThuc(token: string): KetQuaXacThuc {
  if (token === "TOKEN_HOP_LE") return { daXacThuc: true, nguoiDung: "An" };
  return { daXacThuc: false };
}
function phanQuyen(vaiTro: string): KetQuaPhanQuyen {
  if (vaiTro === "admin") return { duocPhep: true };
  return { duocPhep: false, lyDo: `vai trò '${vaiTro}' không đủ quyền` };
}

console.log(JSON.stringify(xacThuc("TOKEN_HOP_LE")));
console.log(JSON.stringify(phanQuyen("admin")));
```

```typescript title=test
const xtHopLe = xacThuc("TOKEN_HOP_LE");
if (xtHopLe.daXacThuc !== true) throw new Error("token hợp lệ phải xác thực thành công");
if (xtHopLe.daXacThuc === true && xtHopLe.nguoiDung !== "An") throw new Error("người dùng phải đúng là An");

const xtSai = xacThuc("SAI");
if (xtSai.daXacThuc !== false) throw new Error("token sai phải xác thực thất bại");

const pqAdmin = phanQuyen("admin");
if (pqAdmin.duocPhep !== true) throw new Error("vai trò admin phải được phép");

const pqKhac = phanQuyen("xem");
if (pqKhac.duocPhep !== false) throw new Error("vai trò khác admin phải bị từ chối");
if (pqKhac.duocPhep === false && (!pqKhac.lyDo || pqKhac.lyDo.trim() === "")) throw new Error("lý do từ chối không được rỗng");
```

:::hints
- kind: attention
  body: "xacThuc: nhánh token hợp lệ trả {daXacThuc: true, nguoiDung: \"An\"}; nhánh còn lại trả {daXacThuc: false}. phanQuyen: nhánh admin trả {duocPhep: true}; nhánh còn lại trả {duocPhep: false, lyDo: ...} kèm thông điệp mô tả vai trò."
- kind: strategy
  body: '{ daXacThuc: true, nguoiDung: "An" } : { daXacThuc: false } — xacThuc. { duocPhep: true } : { duocPhep: false, lyDo: `vai trò \'${vaiTro}\' không đủ quyền` } — phanQuyen.'
- kind: one-line
  body: '___ (xacThuc, hợp lệ) = { daXacThuc: true, nguoiDung: "An" }\n___ (xacThuc, sai) = { daXacThuc: false }\n___ (phanQuyen, admin) = { duocPhep: true }\n___ (phanQuyen, khác) = { duocPhep: false, lyDo: `vai trò \'${vaiTro}\' không đủ quyền` }'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "An"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Authn (LÀ ai) và authz (LÀM được gì) — hai kiểm tra tách biệt. Bước
tiếp theo: vì sao KHÔNG BAO GIỜ lưu mật khẩu người dùng ở dạng đọc
được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Xác thực CẦN so sánh mật khẩu người dùng nhập với mật khẩu ĐÃ lưu.
Lưu mật khẩu THẬT trong database — có vấn đề gì không?
::::

::::checkpoint{mastery=0.8}
::::
