---
id: ky-nghe-phan-mem.bao-mat-ung-dung.rbac-vs-abac-danh-doi
title: "RBAC vs ABAC — đơn giản/thô đối lập linh hoạt/phức tạp"
summary: "Chạy CÙNG một tình huống qua RBAC (chỉ vai trò) và ABAC (thuộc tính) — RBAC KHÔNG phân biệt được, ABAC PHÂN BIỆT được. RBAC mặc định TỐT cho hệ thống đơn giản, ABAC cần khi luật tinh vi hơn vai trò biểu diễn được. Hệ thống thật thường KẾT HỢP cả hai."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [bmud.rbac-vs-abac]
requires: [bmud.abac]
concepts: [bmud.rbac-vs-abac]
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
RBAC (chỉ biết vai trò) và ABAC (biết thuộc tính) — chạy CÙNG một
tình huống qua CẢ HAI, kết quả có luôn GIỐNG NHAU không?
::::

::::explain{#khi-hai-cach-tra-loi-khac-nhau}
Tình huống: Anh có vai trò **`"xem"`** (RBAC: CHỈ có quyền `"doc"`),
NHƯNG là **CHỦ SỞ HỮU** tài liệu này, muốn **GHI**:

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

const an = { id: "U-1", vaiTro: "xem", phongBan: "ky-thuat" };
const taiLieuCuaAn = { chuSoHuu: "U-1", phongBan: "ky-thuat", congKhai: false };

console.log("RBAC:", coQuyen("xem" as VaiTro, "ghi"));
console.log("ABAC:", duocTruyCap(an, taiLieuCuaAn, "ghi"));
```

```text
RBAC: false
ABAC: true
```

**HAI kết quả KHÁC NHAU cho CÙNG một tình huống thực tế**: RBAC
`coQuyen("xem", "ghi")` CHỈ nhìn vào bảng `quyenTheoVaiTro["xem"]`
(KHÔNG có `"ghi"`) → **từ chối**. ABAC `duocTruyCap` nhìn RỘNG hơn —
biết Anh là **CHỦ SỞ HỮU** tài liệu ĐÓ (thuộc tính `chuSoHuu`) → quy
tắc THỨ BA thoả → **cho phép**. RBAC KHÔNG "SAI" (nó ĐÚNG với LUẬT nó
biết — "vai trò xem không ghi được") — nó chỉ **KHÔNG BIẾT** về khái
niệm "chủ sở hữu", vì RBAC KHÔNG mô hình hoá thuộc tính đó.
::::

::::example{#uu-nhuoc-hai-mo-hinh}
**RBAC** (bài 8-9): ĐƠN GIẢN, DỄ AUDIT ("liệt kê ai có vai trò gì" là
đủ để hiểu TOÀN BỘ hệ thống phân quyền) — nhưng THÔ, KHÔNG biểu diễn
được luật DỰA-TRÊN-QUAN-HỆ (chủ sở hữu, cùng nhóm...).

**ABAC** (bài 10): LINH HOẠT, biểu diễn được luật TINH VI — nhưng
PHỨC TẠP HƠN (mỗi luật MỚI = MỘT nhánh `if` MỚI trong hàm, khó "liệt
kê TOÀN BỘ" chỉ bằng MỘT bảng tra như RBAC).

Hệ thống THẬT thường **KẾT HỢP** cả hai: vai trò làm KHUNG chính
(admin/biên-tập/xem — DỄ hiểu, DỄ audit), thuộc tính TINH CHỈNH THÊM
cho các trường hợp ĐẶC BIỆT (chủ sở hữu, phòng ban) — như CHÍNH
`duocTruyCap` đã LÀM (`nd.vaiTro === "admin"` VẪN LÀ quy tắc đầu tiên,
CHỈ THÊM ba quy tắc thuộc tính SAU đó).
::::

::::predict{#doan-them-luat-abac-moi commitOnce}
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

// Nếu THÊM một luật mới "cùng phòng ban ĐƯỢC GHI" (không chỉ đọc),
// việc thêm luật đó có cần sửa hàm coQuyen (RBAC, bài 8) không?
console.log("chỉ cần sửa duocTruyCap, không đụng gì tới RBAC");
```

Nếu bạn THÊM một quy tắc ABAC MỚI (ví dụ "cùng phòng ban được GHI"),
có cần sửa `quyenTheoVaiTro`/`coQuyen` (RBAC, bài 8) không?

:::opt{correct}
Không — hai hệ thống hoàn toàn độc lập, sửa ABAC không đụng gì RBAC
:::

:::opt
Có — vì `duocTruyCap` (ABAC) VÀ `coQuyen` (RBAC) đều CÙNG kiểm tra
KHÁI NIỆM "quyền", nên thay đổi một hệ thống PHẢI đồng bộ sang hệ
thống KIA để tránh MÂU THUẪN logic
::why
Gần đúng ở việc bạn nghĩ tới việc hai hệ thống CÙNG phục vụ một MỤC
ĐÍCH (quyết định "được phép hay không") — một quan sát HỢP LÝ về mặt
Ý NGHĨA NGHIỆP VỤ chung.

Chỗ lệch: về mặt CODE, `duocTruyCap` VÀ `coQuyen` là HAI HÀM HOÀN
TOÀN TÁCH BIỆT, KHÔNG gọi lẫn nhau, KHÔNG chia sẻ biến hay bảng tra
nào. Sửa THÂN hàm `duocTruyCap` (thêm MỘT dòng `if` mới) KHÔNG ảnh
hưởng GÌ tới `coQuyen`/`quyenTheoVaiTro` — hai HỆ THỐNG này CÓ THỂ
tồn tại ĐỘC LẬP (thậm chí một hệ thống THẬT có thể CHỈ dùng MỘT trong
hai, không cần cả hai). "Đồng bộ" nếu MUỐN chỉ là quyết định THIẾT
KẾ (đảm bảo Ý NGHĨA nhất quán cho người dùng cuối), không phải RÀNG
BUỘC kỹ thuật bắt buộc.
::
:::

:::opt
Máy báo lỗi biên dịch — thêm điều kiện mới vào `duocTruyCap` mà
KHÔNG cập nhật `coQuyen` khiến hai hàm "không đồng bộ", TypeScript
phát hiện MÂU THUẪN giữa hai signature liên quan tới `HanhDong`/`Quyen`
::why
Gần đúng ở việc bạn để ý `HanhDong` (`"doc"|"ghi"|"xoa"`, dùng trong
`duocTruyCap`) và `Quyen` (`"doc"|"ghi"|"xoa"|"quan-ly-nguoi-dung"`,
dùng trong `coQuyen`) TRÔNG khá GIỐNG NHAU — một quan sát ĐÚNG về SỰ
TRÙNG LẶP tên giá trị.

Chỗ lệch: `HanhDong` VÀ `Quyen` là HAI type HOÀN TOÀN RIÊNG, khai
ĐỘC LẬP, KHÔNG có ràng buộc nào (như `extends` hay `implements`)
buộc chúng phải "khớp" nhau. TypeScript KHÔNG kiểm tra "hai type
khác nhau ở hai hàm khác nhau có nhất quán về mặt NGHIỆP VỤ hay
không" — đó là trách nhiệm THIẾT KẾ của người viết code, không phải
việc trình biên dịch kiểm được.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
RBAC đơn giản/dễ audit, ABAC linh hoạt/tinh vi hơn — hệ thống thật
thường kết hợp cả hai. Bước tiếp theo: một cách xác thực KHÁC hoàn
toàn — không cần user tự đặt mật khẩu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Login with Google" cho phép đăng nhập mà KHÔNG cần đặt mật khẩu
riêng cho ứng dụng — làm sao ứng dụng biết bạn LÀ AI mà KHÔNG BAO GIỜ
thấy mật khẩu Google của bạn?
::::

::::checkpoint{mastery=0.8}
::::
