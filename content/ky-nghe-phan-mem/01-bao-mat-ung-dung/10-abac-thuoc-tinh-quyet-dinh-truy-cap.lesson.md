---
id: ky-nghe-phan-mem.bao-mat-ung-dung.abac-thuoc-tinh-quyet-dinh-truy-cap
title: "ABAC — khi RBAC quá THÔ, quyết định dựa trên THUỘC TÍNH"
summary: "Attribute-Based Access Control: khi vai trò không đủ tinh (user CHỈ được sửa tài nguyên CỦA CHÍNH MÌNH, cùng phòng ban mới đọc được), quyết định dựa trên thuộc tính của user VÀ tài nguyên, không chỉ vai trò. Vẫn là hàm THUẦN."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [bmud.abac]
requires: [bmud.authorize-pure-du]
concepts: [bmud.abac]
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
RBAC chỉ biết "vai trò". "User chỉ được sửa tài nguyên CỦA CHÍNH
MÌNH" — RBAC (chỉ dựa vào vai trò, không biết AI SỞ HỮU cái gì) trả
lời được câu đó không?
::::

::::explain{#abac-thuoc-tinh}
**Attribute-Based Access Control (ABAC)**: khi RBAC KHÔNG ĐỦ TINH
(ví dụ "user CHỈ sửa được tài nguyên CỦA CHÍNH MÌNH", "user CÙNG
phòng ban mới đọc được") — quyết định dựa trên **THUỘC TÍNH** của
user VÀ tài nguyên, KHÔNG CHỈ vai trò:

```typescript
type NguoiDung = { id: string; vaiTro: string; phongBan: string };
type TaiNguyen = { chuSoHuu: string; phongBan: string; congKhai: boolean };
type HanhDong = "doc" | "ghi" | "xoa";

function duocTruyCap(nd: NguoiDung, tn: TaiNguyen, hd: HanhDong): boolean {
  if (nd.vaiTro === "admin") return true;
  if (tn.congKhai && hd === "doc") return true;
  if (tn.chuSoHuu === nd.id) return true;
  if (nd.phongBan === tn.phongBan && hd === "doc") return true;
  return false;
}

const an = { id: "U-1", vaiTro: "nhan-vien", phongBan: "ky-thuat" };
const taiLieuCuaAn = { chuSoHuu: "U-1", phongBan: "ky-thuat", congKhai: false };
const taiLieuCuaBinh = { chuSoHuu: "U-2", phongBan: "ky-thuat", congKhai: false };

console.log(duocTruyCap(an, taiLieuCuaAn, "ghi"));
console.log(duocTruyCap(an, taiLieuCuaBinh, "doc"));
```

```text
true
true
```

`duocTruyCap` kiểm LẦN LƯỢT bốn quy tắc: admin LUÔN được; tài nguyên
CÔNG KHAI + hành động ĐỌC LUÔN được; CHỦ SỞ HỮU LUÔN được; CÙNG phòng
ban + ĐỌC được. `an` GHI ĐƯỢC tài liệu CỦA CHÍNH MÌNH (chủ sở hữu),
VÀ ĐỌC ĐƯỢC tài liệu của Bình (chủ sở hữu KHÁC, nhưng CÙNG phòng ban)
— RBAC (chỉ biết "vai trò nhân-vien") KHÔNG THỂ phân biệt được HAI
tình huống KHÁC NHAU này.
::::

::::example{#tu-choi-khi-khong-thoa-dieu-kien-nao}
Tài nguyên RIÊNG TƯ, KHÁC phòng ban, KHÔNG PHẢI chủ sở hữu — KHÔNG
quy tắc NÀO thoả, kết quả LÀ TỪ CHỐI:

```typescript title=readonly
type NguoiDung = { id: string; vaiTro: string; phongBan: string };
type TaiNguyen = { chuSoHuu: string; phongBan: string; congKhai: boolean };
type HanhDong = "doc" | "ghi" | "xoa";
function duocTruyCap(nd: NguoiDung, tn: TaiNguyen, hd: HanhDong): boolean {
  if (nd.vaiTro === "admin") return true;
  if (tn.congKhai && hd === "doc") return true;
  if (tn.chuSoHuu === nd.id) return true;
  if (nd.phongBan === tn.phongBan && hd === "doc") return true;
  return false;
}

const an = { id: "U-1", vaiTro: "nhan-vien", phongBan: "ky-thuat" };
const taiLieuCongKhaiKhacPhongBan = { chuSoHuu: "U-3", phongBan: "marketing", congKhai: true };
const taiLieuRiengTuKhacPhongBan = { chuSoHuu: "U-4", phongBan: "marketing", congKhai: false };

console.log(duocTruyCap(an, taiLieuCongKhaiKhacPhongBan, "doc"));
console.log(duocTruyCap(an, taiLieuRiengTuKhacPhongBan, "doc"));
```

```text title=readonly
true
false
```

Tài liệu CÔNG KHAI (dù KHÁC phòng ban) → `true` (quy tắc THỨ HAI
thoả). Tài liệu RIÊNG TƯ, KHÁC phòng ban, KHÔNG PHẢI chủ sở hữu, an
KHÔNG PHẢI admin → **KHÔNG MỘT** quy tắc nào thoả → `false`. Mỗi
"case" thực tế được PHÂN LOẠI qua TỔ HỢP thuộc tính (`congKhai`,
`chuSoHuu`, `phongBan`), KHÔNG PHẢI qua MỘT nhãn "vai trò" duy nhất.
::::

::::predict{#doan-thu-tu-quy-tac-khong-anh-huong commitOnce}
```typescript
type NguoiDung = { id: string; vaiTro: string; phongBan: string };
type TaiNguyen = { chuSoHuu: string; phongBan: string; congKhai: boolean };
type HanhDong = "doc" | "ghi" | "xoa";
function duocTruyCap(nd: NguoiDung, tn: TaiNguyen, hd: HanhDong): boolean {
  if (nd.vaiTro === "admin") return true;
  if (tn.congKhai && hd === "doc") return true;
  if (tn.chuSoHuu === nd.id) return true;
  if (nd.phongBan === tn.phongBan && hd === "doc") return true;
  return false;
}

// Admin, NHƯNG tài nguyên KHÔNG công khai, KHÔNG cùng phòng ban, KHÔNG phải chủ sở hữu
const quanTri = { id: "U-99", vaiTro: "admin", phongBan: "van-phong" };
const taiLieuBiMat = { chuSoHuu: "U-1", phongBan: "ky-thuat", congKhai: false };

console.log(duocTruyCap(quanTri, taiLieuBiMat, "xoa"));
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
`false` — vì admin KHÔNG thoả BA quy tắc CÒN LẠI (không công khai,
không phải chủ sở hữu, không cùng phòng ban), và hàm CHỈ trả `true`
khi CÓ ÍT NHẤT một quy tắc CỤ THỂ khớp
::why
Gần đúng ở việc bạn nhớ ĐÚNG `quanTri` KHÔNG thoả BA quy tắc SAU (từ
quy tắc thứ hai trở đi) — quan sát về việc BA quy tắc ĐÓ không khớp
đúng.

Chỗ lệch: hàm kiểm CÁC quy tắc THEO THỨ TỰ, `return true` NGAY khi
gặp quy tắc ĐẦU TIÊN khớp — `if (nd.vaiTro === "admin") return true;`
LÀ quy tắc **ĐẦU TIÊN**. `quanTri.vaiTro === "admin"` là `true` — hàm
`return true` NGAY, KHÔNG BAO GIỜ CHẠM tới ba quy tắc CÒN LẠI. Vai
trò admin LUÔN được phép, BẤT KỂ tài nguyên thuộc phòng ban nào, ai
sở hữu, công khai hay không — đây LÀ quy tắc "toàn quyền" đứng ĐẦU.
::
:::

:::opt
Máy báo lỗi biên dịch — `vaiTro: "admin"` (một chuỗi) không tương
thích với kiểu `string` được suy ra từ đối tượng `quanTri`, vì các
biến trước đó (`an`) đã "cố định" kiểu `vaiTro` là `"nhan-vien"`
::why
Gần đúng ở việc bạn nghĩ tới việc TypeScript có SUY LUẬN KIỂU khá
tinh vi cho literal — một mối lo hợp lý khi làm việc với string
literal type (đã gặp vài lần trong track).

Chỗ lệch: `NguoiDung.vaiTro` khai kiểu `string` (KHÔNG PHẢI union
literal cụ thể như `VaiTro` ở bài 8-9) — chấp nhận **BẤT KỲ** chuỗi
nào, bao gồm `"admin"`, `"nhan-vien"`, hay bất cứ giá trị nào khác.
Biến `an` (khai TRƯỚC) và `quanTri` (khai SAU) HOÀN TOÀN ĐỘC LẬP về
giá trị — không có "cố định kiểu qua biến trước" nào cả. Biên dịch
sạch.
::
:::
::::

::::code{#viet_duoctruycap}
Tự viết quy tắc "chủ sở hữu" trong `duocTruyCap`.

```typescript title=starter
type NguoiDung = { id: string; vaiTro: string; phongBan: string };
type TaiNguyen = { chuSoHuu: string; phongBan: string; congKhai: boolean };
type HanhDong = "doc" | "ghi" | "xoa";

function duocTruyCap(nd: NguoiDung, tn: TaiNguyen, hd: HanhDong): boolean {
  if (nd.vaiTro === "admin") return true;
  if (tn.congKhai && hd === "doc") return true;
  if (___) return true;
  if (nd.phongBan === tn.phongBan && hd === "doc") return true;
  return false;
}

const an = { id: "U-1", vaiTro: "nhan-vien", phongBan: "ky-thuat" };
const taiLieuCuaAn = { chuSoHuu: "U-1", phongBan: "marketing", congKhai: false };
console.log(duocTruyCap(an, taiLieuCuaAn, "ghi"));
```

```typescript title=solution
type NguoiDung = { id: string; vaiTro: string; phongBan: string };
type TaiNguyen = { chuSoHuu: string; phongBan: string; congKhai: boolean };
type HanhDong = "doc" | "ghi" | "xoa";

function duocTruyCap(nd: NguoiDung, tn: TaiNguyen, hd: HanhDong): boolean {
  if (nd.vaiTro === "admin") return true;
  if (tn.congKhai && hd === "doc") return true;
  if (tn.chuSoHuu === nd.id) return true;
  if (nd.phongBan === tn.phongBan && hd === "doc") return true;
  return false;
}

const an = { id: "U-1", vaiTro: "nhan-vien", phongBan: "ky-thuat" };
const taiLieuCuaAn = { chuSoHuu: "U-1", phongBan: "marketing", congKhai: false };
console.log(duocTruyCap(an, taiLieuCuaAn, "ghi"));
```

```typescript title=test
const nd1 = { id: "U-1", vaiTro: "nhan-vien", phongBan: "ky-thuat" };
const tnCuaU1 = { chuSoHuu: "U-1", phongBan: "marketing", congKhai: false };
if (duocTruyCap(nd1, tnCuaU1, "ghi") !== true) throw new Error("chủ sở hữu phải được phép, dù khác phòng ban và không công khai");

const nd2 = { id: "U-2", vaiTro: "nhan-vien", phongBan: "ky-thuat" };
const tnKhongThoaGiDo = { chuSoHuu: "U-9", phongBan: "marketing", congKhai: false };
if (duocTruyCap(nd2, tnKhongThoaGiDo, "ghi") !== false) throw new Error("không thoả điều kiện nào phải bị từ chối");

const nd3 = { id: "U-3", vaiTro: "nhan-vien", phongBan: "ky-thuat" };
const tnKhacChuSoHuu = { chuSoHuu: "U-4", phongBan: "ky-thuat", congKhai: false };
if (duocTruyCap(nd3, tnKhacChuSoHuu, "doc") !== true) throw new Error("cùng phòng ban, đọc, dù không phải chủ sở hữu, vẫn phải được phép");
```

:::hints
- kind: attention
  body: "Kiểm tra tn.chuSoHuu có ĐÚNG BẰNG nd.id không — người dùng sở hữu tài nguyên đó thì luôn được phép, bất kể hành động gì."
- kind: strategy
  body: "tn.chuSoHuu === nd.id — so sánh trực tiếp id chủ sở hữu với id người dùng đang yêu cầu."
- kind: one-line
  body: "___ = tn.chuSoHuu === nd.id"
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
ABAC: quyết định dựa trên thuộc tính, tinh vi hơn RBAC. Bước tiếp
theo: so sánh trực tiếp hai cách tiếp cận trên CÙNG một tình huống.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

RBAC (bài 8-9, chỉ biết vai trò) và ABAC (bài này, biết thuộc tính) —
chạy CÙNG một tình huống qua CẢ HAI, kết quả có luôn GIỐNG NHAU
không?
::::

::::checkpoint{mastery=0.8}
::::
