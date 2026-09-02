---
id: ky-nghe-phan-mem.bao-mat-ung-dung.jwt-token-khong-trang-thai
title: "JWT — token TỰ CHỨA thông tin, server không cần tra cứu"
summary: "Session truyền thống: server LƯU trạng thái, MỖI request tra cứu lại. JWT: token TỰ CHỨA danh tính + hạn dùng, server CHỈ xác minh chữ ký, KHÔNG tra cứu database. Đánh đổi: nhanh hơn, nhưng khó thu hồi TRƯỚC hạn."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [bmud.jwt-stateless]
requires: [bmud.slow-hash-vs-fast]
concepts: [bmud.jwt-stateless]
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
Đăng nhập THÀNH CÔNG rồi — server cần "nhớ" bạn cho các request TIẾP
THEO. Session TRUYỀN THỐNG lưu Ở SERVER — có cách nào KHÁC không?
::::

::::explain{#jwt-tu-chua}
Session truyền thống: server **LƯU** trạng thái đăng nhập (database
hoặc bộ nhớ), **MỖI** request phải **TRA CỨU LẠI** để biết "ai đang
gọi". **JWT** (JSON Web Token): server **KHÔNG lưu gì** — token **TỰ
CHỨA** danh tính + hạn dùng, server CHỈ xác minh CHỮ KÝ, KHÔNG tra
cứu database:

```typescript
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };

function taoThongTin(nguoiDung: string, vaiTro: string, ttlGiay: number, gioHienTai: number): ThongTinToken {
  return { nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay };
}

const tt = taoThongTin("KH-01", "admin", 3600, 1000);
console.log(JSON.stringify(tt));

function daHetHan(tt: ThongTinToken, gioHienTai: number): boolean {
  return tt.hetHan < gioHienTai;
}
console.log(daHetHan(tt, 2000));
console.log(daHetHan(tt, 9999));
```

```text
{"nguoiDung":"KH-01","vaiTro":"admin","hetHan":4600}
false
true
```

`ThongTinToken` MANG SẴN `nguoiDung`, `vaiTro`, `hetHan` — server đọc
BA field đó **TRỰC TIẾP TỪ token**, KHÔNG cần TRA MỘT database nào
để biết "KH-01 là ai" hay "còn hạn không" — đó CHÍNH LÀ ý nghĩa
"**stateless**" (không trạng thái): server KHÔNG lưu MỘT bảng "phiên
đăng nhập" nào cả, MỌI thông tin CẦN đã nằm SẴN trong token.
::::

::::example{#danh-doi-toc-do-va-thu-hoi}
Đánh đổi CHÍNH: JWT NHANH HƠN (không round-trip database MỖI request)
nhưng KHÓ thu hồi TRƯỚC hạn — MỘT khi đã phát hành, server KHÔNG có
cách nào "xoá" nó khỏi TAY người dùng:

```typescript title=readonly
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };
function taoThongTin(nguoiDung: string, vaiTro: string, ttlGiay: number, gioHienTai: number): ThongTinToken {
  return { nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay };
}
function daHetHan(tt: ThongTinToken, gioHienTai: number): boolean {
  return tt.hetHan < gioHienTai;
}

// User BỊ SA THẢI lúc 5000 (giây), NHƯNG token của họ tạo lúc 1000, hết hạn lúc 4600
const ttNhanVienCu = taoThongTin("NV-99", "nhan-vien", 3600, 1000);
console.log(daHetHan(ttNhanVienCu, 5000)); // may mắn: đã TỰ hết hạn TRƯỚC lúc bị sa thải

// NHƯNG nếu ttlGiay DÀI HƠN (ví dụ 10 tiếng thay vì 1)
const ttNhanVienCu2 = taoThongTin("NV-99", "nhan-vien", 36000, 1000);
console.log(daHetHan(ttNhanVienCu2, 5000)); // false -- token VẪN hợp lệ, server KHÔNG có cách "rút lại"
```

```text title=readonly
true
false
```

Ca THỨ HAI: token của nhân viên bị sa thải **VẪN hợp lệ** — server
KHÔNG có "bảng phiên" nào để XOÁ, JWT CHỈ TỰ hết hạn khi `hetHan` tới
— **không có cách can thiệp SỚM HƠN**. Chọn `ttlGiay` NGẮN giảm thiệt
hại (cửa sổ rủi ro nhỏ), nhưng KHÔNG giải quyết HOÀN TOÀN vấn đề "thu
hồi ngay lập tức" — vấn đề này dẫn tới kỹ thuật ở bài sau.
::::

::::predict{#doan-hai-token-doc-lap commitOnce}
```typescript
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };
function taoThongTin(nguoiDung: string, vaiTro: string, ttlGiay: number, gioHienTai: number): ThongTinToken {
  return { nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay };
}
function daHetHan(tt: ThongTinToken, gioHienTai: number): boolean {
  return tt.hetHan < gioHienTai;
}

// Cấp HAI token, CÙNG một người dùng, CÙNG thời điểm, ttl KHÁC nhau
const ttNgan = taoThongTin("KH-05", "admin", 60, 1000);   // 1 phút
const ttDai = taoThongTin("KH-05", "admin", 7200, 1000);  // 2 tiếng

console.log(daHetHan(ttNgan, 1100));
console.log(daHetHan(ttDai, 1100));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`true` rồi `false`
:::

:::opt
`true` rồi `true` — vì CẢ HAI token thuộc CÙNG MỘT người dùng
(`"KH-05"`), một khi MỘT token hết hạn thì server coi NHƯ NGƯỜI DÙNG
ĐÓ đã hết hạn PHIÊN, ảnh hưởng tới token KIA
::why
Gần đúng ở việc bạn nhớ ĐÚNG CẢ HAI token đều thuộc `"KH-05"` — quan
sát về việc CÙNG người dùng đó đúng.

Chỗ lệch: MỖI token là MỘT object `ThongTinToken` HOÀN TOÀN ĐỘC LẬP,
mang `hetHan` **RIÊNG CỦA NÓ**, được TÍNH TỪ `ttlGiay` LÚC TẠO —
KHÔNG có "liên kết" nào giữa hai token CÙNG người dùng (đây LÀ hệ quả
TRỰC TIẾP của "stateless": server KHÔNG lưu bảng nào theo dõi "user
KH-05 có bao nhiêu token đang hoạt động"). `ttNgan.hetHan = 1060`,
`ttDai.hetHan = 8200` — `daHetHan(ttNgan, 1100)`: `1060 < 1100` =
`true`. `daHetHan(ttDai, 1100)`: `8200 < 1100` = `false`. Hai kết quả
HOÀN TOÀN ĐỘC LẬP, dựa trên `hetHan` RIÊNG của MỖI token.
::
:::

:::opt
Máy báo lỗi biên dịch — không thể tạo HAI `ThongTinToken` cho CÙNG
một `nguoiDung` (`"KH-05"`) trong cùng đoạn code, vì `ThongTinToken`
không cho phép trùng lặp danh tính
::why
Gần đúng ở việc bạn nghĩ tới một RÀNG BUỘC "một user một token" — một
trực giác NGHIỆP VỤ hợp lý cho một số hệ thống muốn giới hạn phiên.

Chỗ lệch: `ThongTinToken` (kiểu dữ liệu) KHÔNG hề mã hoá ràng buộc
"chỉ một token mỗi user" — đó là một QUY TẮC NGHIỆP VỤ (nếu MUỐN áp
dụng) phải được CODE KHÁC thực thi (ví dụ lưu danh sách token đang
hoạt động Ở SERVER — nhưng đó chính LÀ quay lại mô hình "có trạng
thái", đối lập với JWT thuần stateless). Về mặt KIỂU, tạo BAO NHIÊU
`ThongTinToken` tuỳ ý, kể cả trùng `nguoiDung`, đều hợp lệ.
::
:::
::::

::::code{#viet_taothongtin_va_dahethanhan}
Tự viết `taoThongTin` và `daHetHan`.

```typescript title=starter
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };

function taoThongTin(nguoiDung: string, vaiTro: string, ttlGiay: number, gioHienTai: number): ThongTinToken {
  return ___;
}
function daHetHan(tt: ThongTinToken, gioHienTai: number): boolean {
  return ___;
}

console.log(JSON.stringify(taoThongTin("KH-01", "admin", 3600, 1000)));
```

```typescript title=solution
type ThongTinToken = { nguoiDung: string; vaiTro: string; hetHan: number };

function taoThongTin(nguoiDung: string, vaiTro: string, ttlGiay: number, gioHienTai: number): ThongTinToken {
  return { nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay };
}
function daHetHan(tt: ThongTinToken, gioHienTai: number): boolean {
  return tt.hetHan < gioHienTai;
}

console.log(JSON.stringify(taoThongTin("KH-01", "admin", 3600, 1000)));
```

```typescript title=test
const ttTest = taoThongTin("KH-09", "xem", 100, 500);
if (ttTest.nguoiDung !== "KH-09") throw new Error("taoThongTin phải giữ đúng nguoiDung");
if (ttTest.vaiTro !== "xem") throw new Error("taoThongTin phải giữ đúng vaiTro");
if (ttTest.hetHan !== 600) throw new Error("hetHan phải là gioHienTai + ttlGiay (500 + 100 = 600)");

// Biên: đúng ngưỡng hetHan CHƯA hết hạn, ngưỡng+1 ĐÃ hết hạn
if (daHetHan(ttTest, 600) !== false) throw new Error("gioHienTai đúng bằng hetHan phải CHƯA tính là hết hạn");
if (daHetHan(ttTest, 601) !== true) throw new Error("gioHienTai vượt hetHan (dù chỉ 1) phải tính là hết hạn");
```

:::hints
- kind: attention
  body: "taoThongTin: hetHan = gioHienTai CỘNG ttlGiay (thời điểm HẾT HẠN = bây giờ + thời lượng sống). daHetHan: so sánh hetHan với gioHienTai bằng < (nhỏ hơn NGHIÊM NGẶT — đúng ngưỡng vẫn CHƯA hết hạn)."
- kind: strategy
  body: "{ nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay } : tt.hetHan < gioHienTai — cộng để tính hạn, so sánh để kiểm hạn."
- kind: one-line
  body: "___ (taoThongTin) = { nguoiDung, vaiTro, hetHan: gioHienTai + ttlGiay }\n___ (daHetHan) = tt.hetHan < gioHienTai"
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
JWT: token tự chứa thông tin, server không tra cứu — nhanh nhưng khó
thu hồi sớm. Bước tiếp theo: chốt cụm — ký token để chống giả mạo.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`ThongTinToken` là DỮ LIỆU THUẦN — ai cũng có thể TỰ TẠO một object
`{nguoiDung: "admin-gia", vaiTro: "admin", hetHan: 999999}` mà không
cần đăng nhập gì cả. Làm sao server BIẾT một token là THẬT (do CHÍNH
server phát hành) chứ không phải GIẢ MẠO?
::::

::::checkpoint{mastery=0.8}
::::
