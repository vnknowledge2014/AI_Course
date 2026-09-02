---
id: ky-nghe-phan-mem.bao-mat-ung-dung.oauth2-dang-nhap-qua-ben-thu-ba
title: "OAuth 2.0 — đăng nhập qua Google/GitHub mà KHÔNG chia sẻ mật khẩu"
summary: "App KHÔNG BAO GIỜ thấy mật khẩu Google của user — user đăng nhập TRÊN trang Google, Google trả về một MÃ, app đổi mã lấy TOKEN giới hạn. PKCE thêm một bí mật tự sinh chống kẻ chặn được mã tự đổi lấy token. Bài THUẦN TƯỜNG THUẬT — luồng thật cần mạng thật."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [bmud.oauth2-pkce]
requires: [bmud.rbac-vs-abac]
concepts: [bmud.oauth2-pkce]
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
"Login with Google" cho phép đăng nhập KHÔNG cần đặt mật khẩu riêng
cho app. Ứng dụng làm sao biết bạn LÀ AI mà KHÔNG BAO GIỜ thấy mật
khẩu Google?
::::

::::explain{#luong-oauth2}
**Bài THUẦN TƯỜNG THUẬT** — luồng OAuth2 THẬT cần **redirect HTTP
tới máy chủ thật của Google**, không giả lập chạy được trong sandbox
đồng bộ. Minh hoạ bằng DỮ LIỆU mô tả TỪNG bước (KHÔNG PHẢI mạng thật):

```typescript
type BuocOAuth = { thuTu: number; moTa: string; aiThucHien: "nguoiDung" | "app" | "google" };

const luongOAuth: BuocOAuth[] = [
  { thuTu: 1, moTa: "App chuyển hướng người dùng tới accounts.google.com kèm client_id + redirect_uri", aiThucHien: "app" },
  { thuTu: 2, moTa: "Người dùng đăng nhập TRÊN trang Google (app KHÔNG BAO GIỜ thấy mật khẩu)", aiThucHien: "nguoiDung" },
  { thuTu: 3, moTa: "Google chuyển hướng NGƯỢC LẠI app kèm một MÃ (authorization code) tạm thời", aiThucHien: "google" },
  { thuTu: 4, moTa: "App gửi mã đó (POST) tới Google, đổi lấy access token", aiThucHien: "app" },
  { thuTu: 5, moTa: "Google trả về access token GIỚI HẠN (chỉ email/tên, KHÔNG toàn quyền tài khoản)", aiThucHien: "google" },
];

for (const b of luongOAuth) {
  console.log(`${b.thuTu}. [${b.aiThucHien}] ${b.moTa}`);
}
```

```text
1. [app] App chuyển hướng người dùng tới accounts.google.com kèm client_id + redirect_uri
2. [nguoiDung] Người dùng đăng nhập TRÊN trang Google (app KHÔNG BAO GIỜ thấy mật khẩu)
3. [google] Google chuyển hướng NGƯỢC LẠI app kèm một MÃ (authorization code) tạm thời
4. [app] App gửi mã đó (POST) tới Google, đổi lấy access token
5. [google] Google trả về access token GIỚI HẠN (chỉ email/tên, KHÔNG toàn quyền tài khoản)
```

Điểm CỐT LÕI: bước 2 xảy ra **TRÊN TRANG GOOGLE**, KHÔNG PHẢI trên
app — mật khẩu Google được NHẬP TRỰC TIẾP vào Google, app CHỈ nhận
được **MÃ TẠM THỜI** (bước 3), rồi ĐỔI mã đó (bước 4) lấy MỘT token
**GIỚI HẠN** (bước 5 — CHỈ email/tên, KHÔNG "toàn quyền" tài khoản
Google của user).
::::

::::example{#pkce-chong-chan-ma}
**PKCE** (Proof Key for Code Exchange): thêm MỘT "mật khẩu MỘT LẦN"
**TỰ SINH** Ở BƯỚC ĐẦU — CHỈ ai giữ nó mới ĐỔI được mã LẤY token,
chống kẻ tấn công **CHẶN ĐƯỢC** authorization code (qua log, malware,
proxy độc) tự đi đổi lấy token:

```typescript title=readonly
type BuocPKCE = { thuTu: number; moTa: string };

const buocPKCE: BuocPKCE[] = [
  { thuTu: 0, moTa: "App tự sinh code_verifier (chuỗi ngẫu nhiên) TRƯỚC KHI chuyển hướng" },
  { thuTu: 1, moTa: "App gửi code_challenge (băm từ code_verifier) kèm request chuyển hướng" },
  { thuTu: 4, moTa: "Lúc đổi mã lấy token, app gửi KÈM code_verifier GỐC để chứng minh CHÍNH MÌNH đã khởi tạo request" },
];
for (const b of buocPKCE) console.log(`PKCE ${b.thuTu}: ${b.moTa}`);
```

```text title=readonly
PKCE 0: App tự sinh code_verifier (chuỗi ngẫu nhiên) TRƯỚC KHI chuyển hướng
PKCE 1: App gửi code_challenge (băm từ code_verifier) kèm request chuyển hướng
PKCE 4: Lúc đổi mã lấy token, app gửi KÈM code_verifier GỐC để chứng minh CHÍNH MÌNH đã khởi tạo request
```

Kẻ tấn công BẮT được authorization code (bước 3, gói tin CÓ THỂ bị
lộ qua nhiều đường: log server, tab trình duyệt bị chia sẻ, malware
trên máy) **VẪN KHÔNG dùng được** — bước ĐỔI mã lấy token (bước 4) ĐÒI
`code_verifier` GỐC, KHÔNG BAO GIỜ được gửi đi TRƯỚC ĐÓ (chỉ dạng ĐÃ
BĂM, `code_challenge`, được gửi Ở BƯỚC 1) — kẻ tấn công KHÔNG có
`code_verifier` GỐC nên KHÔNG chứng minh được "chính mình" đã khởi
tạo request ban đầu.
::::

::::predict{#doan-oauth-khong-lo-mat-khau commitOnce}
```typescript
type BuocOAuth = { thuTu: number; moTa: string; aiThucHien: "nguoiDung" | "app" | "google" };
const luongOAuth: BuocOAuth[] = [
  { thuTu: 1, moTa: "App chuyển hướng tới accounts.google.com", aiThucHien: "app" },
  { thuTu: 2, moTa: "Người dùng đăng nhập TRÊN trang Google", aiThucHien: "nguoiDung" },
  { thuTu: 3, moTa: "Google trả mã tạm thời", aiThucHien: "google" },
  { thuTu: 4, moTa: "App đổi mã lấy token", aiThucHien: "app" },
];

// Đếm bao nhiêu bước do "app" thực hiện HÀNH ĐỘNG liên quan TRỰC TIẾP tới mật khẩu người dùng
const buocAppTiepXucMatKhau = luongOAuth.filter(
  (b) => b.aiThucHien === "app" && b.moTa.includes("mật khẩu")
);
console.log(buocAppTiepXucMatKhau.length);
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì `luongOAuth` CÓ một bước liên quan tới mật khẩu (bước 2), và
`filter` chỉ cần TÌM THẤY nó Ở BẤT KỲ ĐÂU trong mảng
::why
Gần đúng ở việc bạn nhớ ĐÚNG mảng `luongOAuth` CÓ MỘT bước NHẮC tới
"mật khẩu" — bước 2, `moTa` chứa cụm "đăng nhập TRÊN trang Google" —
NHƯNG chú ý, câu MÔ TẢ RÚT GỌN ở đây KHÔNG chứa chữ "mật khẩu" (khác
bản đầy đủ ở phần explain).

Chỗ lệch (quan trọng hơn): NGAY CẢ nếu bước 2 CÓ chứa chữ "mật khẩu",
điều kiện `filter` đòi **CẢ HAI**: `b.aiThucHien === "app"` **VÀ**
`b.moTa.includes("mật khẩu")`. Bước 2 có `aiThucHien: "nguoiDung"`
(KHÔNG PHẢI `"app"`) — điều kiện ĐẦU đã SAI, bước đó KHÔNG được lọc
vào dù `moTa` có chứa gì. Đây CHÍNH LÀ điểm cốt lõi của OAuth2: KHÔNG
bước NÀO do **`"app"`** thực hiện liên quan TRỰC TIẾP tới việc XỬ LÝ
mật khẩu — bước LIÊN QUAN mật khẩu (bước 2) LUÔN do **`"nguoiDung"`**
thực hiện, TRÊN TRANG CỦA GOOGLE, KHÔNG PHẢI trên app.
::
:::

:::opt
Máy báo lỗi biên dịch — `aiThucHien: "app"` không hợp lệ trong điều
kiện `filter`, vì `aiThucHien` là union kiểu literal, không so sánh
được TRỰC TIẾP bằng `===` bên trong một callback
::why
Gần đúng ở việc bạn để ý `aiThucHien` LÀ union literal
(`"nguoiDung"|"app"|"google"`) — một quan sát ĐÚNG về CẤU TRÚC kiểu
(đã gặp NHIỀU lần trong track: `VaiTro`, `TrangThaiDonHang`...).

Chỗ lệch: so sánh MỘT giá trị union literal bằng `===` với MỘT trong
các literal của nó là thao tác **HOÀN TOÀN CHUẨN**, dùng LIÊN TỤC
xuyên suốt track (`vaiTro === "admin"`, `l.ma === "validation"`...).
KHÔNG có hạn chế nào về việc dùng nó BÊN TRONG một callback của
`filter`. Biên dịch sạch.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
OAuth2: app không bao giờ thấy mật khẩu bên thứ ba, chỉ nhận token
giới hạn. PKCE chống mã bị chặn giữa đường. Bước tiếp theo: chốt cụm
— ghép pipeline auth hoàn chỉnh bằng Result.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có TẤT CẢ mảnh ghép: trích token từ header, xác minh chữ ký/hết
hạn, kiểm quyền. Ghép BA bước đó thành MỘT pipeline duy nhất, dùng
`Result`/`chainResult` đã học (T4.5) — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
