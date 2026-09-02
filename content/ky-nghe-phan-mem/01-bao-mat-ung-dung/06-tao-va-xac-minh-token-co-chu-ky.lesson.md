---
id: ky-nghe-phan-mem.bao-mat-ung-dung.tao-va-xac-minh-token-co-chu-ky
title: "Capstone: Tạo và xác minh token có chữ ký, chống giả mạo & hết hạn"
summary: "Bài chốt cụm 1: ghép salt+hash và cấu trúc token thành hệ thống ký/xác minh hoàn chỉnh. ky() — checksum tự viết (KHÔNG PHẢI mã hoá thật). taoToken ghép nội dung + chữ ký. xacMinhToken TỪ CHỐI nếu chữ ký sai (giả mạo) HOẶC đã hết hạn."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [bmud.token-sign-verify]
requires: [bmud.jwt-stateless]
concepts: [bmud.token-sign-verify]
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
Bài chốt cụm 1. Ai cũng tự tạo được một object `{nguoiDung, vaiTro,
hetHan}` — làm sao server biết một token THẬT do CHÍNH NÓ phát hành?
::::

::::explain{#chu-ky}
Ghép **salt+hash** (bài 2-4) và **cấu trúc token** (bài 5) thành MỘT
hệ thống KÝ/XÁC MINH: `ky(noiDung)` — hàm checksum **TỰ VIẾT** (thay
`Buffer`/module crypto không có trong sandbox — **KHÔNG PHẢI mã hoá
thật**, chỉ minh hoạ HÌNH DẠNG kỹ thuật). Chỉ AI biết `BI_MAT` (khoá
bí mật, CHỈ server có) mới TÍNH ĐÚNG được chữ ký:

```typescript
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };
const BI_MAT = "khoa-bi-mat-cua-server";

function ky(noiDung: string): string {
  let tong = 0;
  for (const c of noiDung + BI_MAT) tong += c.charCodeAt(0);
  return tong.toString(16);
}

function taoToken(nguoiDung: string, vaiTro: string, ttlGiay: number, gioHienTai: number): string {
  const noiDung = JSON.stringify({ nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay });
  return `${noiDung}.${ky(noiDung)}`;
}

const token = taoToken("KH-01", "admin", 3600, 1000);
console.log(token);
```

```text
{"nguoiDung":"KH-01","vaiTro":"admin","hetHan":4600}.17f7
```

Token = `noiDung.chuKy` — HAI phần nối bằng dấu chấm. `chuKy` (`17f7`)
được TÍNH TỪ `noiDung` **VÀ** `BI_MAT` — bất kỳ ai KHÔNG biết `BI_MAT`
KHÔNG THỂ tính ĐÚNG chữ ký cho một `noiDung` GIẢ MẠO (dù họ TỰ TẠO
được `noiDung` dễ dàng, vì đó chỉ là JSON thường).
::::

::::example{#xac-minh-tu-choi-gia-mao-va-het-han}
`xacMinhToken` TỪ CHỐI **NẾU** chữ ký SAI (giả mạo) **HOẶC** đã HẾT
HẠN — trả `null` cho CẢ HAI trường hợp:

```typescript title=readonly
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };
const BI_MAT = "khoa-bi-mat-cua-server";
function ky(noiDung: string): string {
  let tong = 0;
  for (const c of noiDung + BI_MAT) tong += c.charCodeAt(0);
  return tong.toString(16);
}
function taoToken(nguoiDung: string, vaiTro: string, ttlGiay: number, gioHienTai: number): string {
  const noiDung = JSON.stringify({ nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay });
  return `${noiDung}.${ky(noiDung)}`;
}

function xacMinhToken(token: string, gioHienTai: number): ThongTinToken | null {
  const i = token.lastIndexOf(".");
  const noiDung = token.slice(0, i);
  const chuKy = token.slice(i + 1);
  if (chuKy !== ky(noiDung)) return null; // chữ ký SAI -- giả mạo
  const tt = JSON.parse(noiDung) as ThongTinToken;
  return tt.hetHan < gioHienTai ? null : tt; // đã HẾT HẠN
}

const token = taoToken("KH-01", "admin", 3600, 1000);
console.log(xacMinhToken(token, 2000));              // hợp lệ
console.log(xacMinhToken(token + "GIA_MAO", 2000));  // giả mạo -- chữ ký lệch
console.log(xacMinhToken(token, 9999));               // hết hạn
```

```text title=readonly
{"nguoiDung":"KH-01","vaiTro":"admin","hetHan":4600}
null
null
```

Thêm CHỈ MỘT chuỗi vào cuối token (`+ "GIA_MAO"`) khiến `noiDung` bị
LỆCH (phần "nội dung" trước dấu chấm CUỐI CÙNG thay đổi), `ky(noiDung)`
TÍNH LẠI ra chữ ký KHÁC hẳn `chuKy` đã tách được từ token — thất bại
NGAY LẬP TỨC, KHÔNG cần biết `BI_MAT` để PHÁT HIỆN giả mạo (chỉ CẦN
`BI_MAT` để TẠO chữ ký ĐÚNG, không cần để KIỂM chữ ký).
::::

::::predict{#doan-doi-mot-ky-tu-payload commitOnce}
```typescript
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };
const BI_MAT = "khoa-bi-mat-cua-server";
function ky(noiDung: string): string {
  let tong = 0;
  for (const c of noiDung + BI_MAT) tong += c.charCodeAt(0);
  return tong.toString(16);
}
function xacMinhToken(token: string, gioHienTai: number): ThongTinToken | null {
  const i = token.lastIndexOf(".");
  const noiDung = token.slice(0, i);
  const chuKy = token.slice(i + 1);
  if (chuKy !== ky(noiDung)) return null;
  const tt = JSON.parse(noiDung) as ThongTinToken;
  return tt.hetHan < gioHienTai ? null : tt;
}

// Token GỐC hợp lệ
const tokenGoc = `{"nguoiDung":"KH-01","vaiTro":"xem","hetHan":4600}.${ky('{"nguoiDung":"KH-01","vaiTro":"xem","hetHan":4600}')}`;

// Kẻ tấn công SỬA "xem" thành "admin" NGAY TRONG payload -- CHỮ KÝ CŨ giữ nguyên
const tokenBiSua = tokenGoc.replace('"vaiTro":"xem"', '"vaiTro":"admin"');
console.log(xacMinhToken(tokenBiSua, 2000));
```

Dòng cuối in ra gì?

:::opt{correct}
`null`
:::

:::opt
`{ nguoiDung: 'KH-01', vaiTro: 'admin', hetHan: 4600 }` — vì kẻ tấn
công CHỈ sửa payload, KHÔNG đụng vào phần chữ ký (nằm SAU dấu chấm
cuối), nên `xacMinhToken` vẫn đọc được payload MỚI (đã sửa) bình
thường
::why
Gần đúng ở việc bạn nhớ ĐÚNG kẻ tấn công CHỈ sửa PHẦN PAYLOAD
(`"vaiTro":"xem"` → `"vaiTro":"admin"`), KHÔNG đụng gì tới phần SAU
dấu chấm — một quan sát CHÍNH XÁC về hành động của `.replace(...)`.

Chỗ lệch: `xacMinhToken` KHÔNG chỉ "đọc" payload — nó **TÍNH LẠI**
`ky(noiDung)` TRÊN payload MỚI (đã sửa, chứa `"admin"`) rồi SO SÁNH
với `chuKy` đã TÁCH RA từ token (chữ ký ĐÓ được tính TRÊN payload
GỐC, chứa `"xem"`). Vì `noiDung` đã ĐỔI (dù chỉ MỘT TỪ), TỔNG mã ký
tự (`tong` bên trong `ky`) đổi theo, chữ ký TÍNH LẠI ra KHÁC HẲN
`chuKy` CŨ — `chuKy !== ky(noiDung)` là `true`, `xacMinhToken` trả
`null` NGAY, KHÔNG BAO GIỜ chạm tới bước đọc `vaiTro`. Đây CHÍNH LÀ
lý do chữ ký PHẢI phủ TOÀN BỘ payload — sửa DÙ MỘT ký tự cũng bị bắt.
::
:::

:::opt
Máy báo lỗi biên dịch — `tokenGoc.replace(...)` không hợp lệ vì
`tokenGoc` được khai bằng template string PHỨC TẠP (gọi `ky(...)`
bên trong), TypeScript không cho `.replace` trên kiểu chuỗi ĐÓ
::why
Gần đúng ở việc bạn để ý `tokenGoc` được khai theo cách KHÁ PHỨC TẠP
(template string LỒNG một lời gọi hàm `ky(...)` bên trong) — một
quan sát ĐÚNG về ĐỘ phức tạp của biểu thức.

Chỗ lệch: DÙ biểu thức TẠO RA `tokenGoc` phức tạp thế nào, KẾT QUẢ
CUỐI CÙNG vẫn CHỈ là một `string` bình thường — `.replace(...)` (một
method CÓ SẴN trên MỌI `string`) hoạt động HOÀN TOÀN bình thường,
không quan tâm chuỗi đó được TẠO RA bằng cách nào. Biên dịch sạch.
::
:::
::::

::::code{#viet_taotoken_va_xacminh}
Tự viết phần cốt lõi của `taoToken` và `xacMinhToken`.

```typescript title=starter
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };
const BI_MAT = "khoa-bi-mat-cua-server";

function ky(noiDung: string): string {
  let tong = 0;
  for (const c of noiDung + BI_MAT) tong += c.charCodeAt(0);
  return tong.toString(16);
}

function taoToken(nguoiDung: string, vaiTro: string, ttlGiay: number, gioHienTai: number): string {
  const noiDung = JSON.stringify({ nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay });
  return ___;
}

function xacMinhToken(token: string, gioHienTai: number): ThongTinToken | null {
  const i = token.lastIndexOf(".");
  const noiDung = token.slice(0, i);
  const chuKy = token.slice(i + 1);
  if (___) return null;
  const tt = JSON.parse(noiDung) as ThongTinToken;
  return ___;
}

const token = taoToken("KH-01", "admin", 3600, 1000);
console.log(xacMinhToken(token, 2000));
```

```typescript title=solution
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };
const BI_MAT = "khoa-bi-mat-cua-server";

function ky(noiDung: string): string {
  let tong = 0;
  for (const c of noiDung + BI_MAT) tong += c.charCodeAt(0);
  return tong.toString(16);
}

function taoToken(nguoiDung: string, vaiTro: string, ttlGiay: number, gioHienTai: number): string {
  const noiDung = JSON.stringify({ nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay });
  return `${noiDung}.${ky(noiDung)}`;
}

function xacMinhToken(token: string, gioHienTai: number): ThongTinToken | null {
  const i = token.lastIndexOf(".");
  const noiDung = token.slice(0, i);
  const chuKy = token.slice(i + 1);
  if (chuKy !== ky(noiDung)) return null;
  const tt = JSON.parse(noiDung) as ThongTinToken;
  return tt.hetHan < gioHienTai ? null : tt;
}

const token = taoToken("KH-01", "admin", 3600, 1000);
console.log(xacMinhToken(token, 2000));
```

```typescript title=test
const tokenTest = taoToken("KH-77", "bien-tap", 100, 500);

const hopLe = xacMinhToken(tokenTest, 550);
if (hopLe === null) throw new Error("token hợp lệ, chưa hết hạn phải xác minh được");
if (hopLe.nguoiDung !== "KH-77") throw new Error("token xác minh được phải giữ đúng nguoiDung");
if (hopLe.vaiTro !== "bien-tap") throw new Error("token xác minh được phải giữ đúng vaiTro");

const gioiMao = xacMinhToken(tokenTest + "X", 550);
if (gioiMao !== null) throw new Error("token bị thêm ký tự (giả mạo) phải xác minh thất bại (null)");

const hetHanTest = xacMinhToken(tokenTest, 601);
if (hetHanTest !== null) throw new Error("token đã hết hạn phải xác minh thất bại (null)");

// Biên: đúng ngưỡng hetHan (600) chưa hết hạn, ngưỡng+1 (601) đã hết hạn
const bienDung = xacMinhToken(tokenTest, 600);
if (bienDung === null) throw new Error("gioHienTai đúng bằng hetHan phải CHƯA tính là hết hạn");
```

:::hints
- kind: attention
  body: "taoToken: nối noiDung và ky(noiDung) bằng dấu chấm. xacMinhToken: nhánh từ chối kiểm chuKy khác ky(noiDung); nhánh cuối trả null nếu hết hạn, ngược lại trả tt."
- kind: strategy
  body: '`${noiDung}.${ky(noiDung)}` : chuKy !== ky(noiDung) : tt.hetHan < gioHienTai ? null : tt — ba mảnh ghép của hệ thống ký/xác minh.'
- kind: one-line
  body: '___ (taoToken) = `${noiDung}.${ky(noiDung)}`\n___ (xacMinhToken, kiểm chữ ký) = chuKy !== ky(noiDung)\n___ (xacMinhToken, kiểm hạn) = tt.hetHan < gioHienTai ? null : tt'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "KH-01"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 1 hoàn tất: xác thực/phân quyền tách biệt, mật khẩu băm một chiều
có salt riêng và cố ý chậm, JWT stateless ký/xác minh chống giả mạo
và hết hạn. Cụm tiếp theo: phân quyền thực tế và auth đầy đủ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Token stateless (bài trước) khó thu hồi TRƯỚC hạn. Nếu chọn `ttlGiay`
NGẮN để giảm rủi ro, người dùng phải đăng nhập LẠI liên tục — có cách
nào cân bằng giữa AN TOÀN và TIỆN LỢI không?
::::

::::checkpoint{mastery=0.8}
::::
