---
id: ky-nghe-phan-mem.bao-mat-ung-dung.csrf-samesite-va-token
title: "CSRF — trình duyệt tự gửi cookie, SameSite và token chặn giả mạo"
summary: "Trình duyệt TỰ ĐỘNG đính kèm cookie khi gửi request tới domain đã đăng nhập, kể cả khi request xuất phát từ trang KHÁC. SameSite=Strict chặn cookie đính kèm cross-site (tường thuật). CSRF token: server phát token gắn vào form, request không có token ĐÚNG bị từ chối."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 19
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [bmud.csrf-samesite-token]
requires: [bmud.csp-allowlist]
concepts: [bmud.csrf-samesite-token]
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
Trình duyệt TỰ ĐỘNG đính kèm cookie khi gửi request tới domain đã
đăng nhập — KỂ CẢ khi request xuất phát từ một trang WEB KHÁC. Nguy
hiểm gì?
::::

::::explain{#trinh-duyet-tu-gui-cookie}
**Cross-Site Request Forgery (CSRF)**: bạn đăng nhập `bank.com`
(cookie phiên được LƯU). Bạn MỞ MỘT TAB KHÁC, ghé `evil.com` — trang
ĐÓ có một FORM ẨN tự động GỬI TỚI `bank.com/chuyen-tien`. Trình duyệt
**TỰ ĐỘNG** đính kèm cookie `bank.com` vào request ĐÓ (VÌ request LÀ
GỬI TỚI `bank.com`, trình duyệt KHÔNG quan tâm request XUẤT PHÁT từ
ĐÂU) — `bank.com` NHẬN được request TRÔNG NHƯ hợp lệ (CÓ cookie phiên
ĐÚNG), thực hiện chuyển tiền MÀ BẠN KHÔNG HỀ Ý ĐỊNH.

**Phần TƯỜNG THUẬT** (hành vi trình duyệt, KHÔNG giả lập được):
`SameSite=Strict` (thuộc tính cookie) chặn trình duyệt đính kèm cookie
ĐÓ vào request CROSS-SITE (xuất phát từ domain KHÁC) — cookie CHỈ
được gửi khi request xuất phát TỪ CHÍNH `bank.com`.
::::

::::example{#csrf-token}
**Phần CODE ĐƯỢC** (server-side): CSRF token — server phát MỘT token
NGẪU NHIÊN, gắn vào FORM, request THAY ĐỔI trạng thái (KHÔNG PHẢI
`GET`) KHÔNG có token ĐÚNG bị **TỪ CHỐI**:

```typescript title=readonly
function generateCsrfToken(): string {
  return Array.from({ length: 32 }, () => Math.floor(Math.random() * 36).toString(36)).join("");
}

function validateCsrfToken(sent: string, expected: string): boolean {
  return sent === expected;
}

const token = generateCsrfToken();
console.log(token.length);
console.log(validateCsrfToken(token, token));
console.log(validateCsrfToken("forged", token));
```

```text title=readonly
32
true
false
```

`evil.com` KHÔNG THỂ ĐỌC được token thật của bạn (token nằm TRONG
form của `bank.com`, `evil.com` không có QUYỀN truy cập nội dung
TRANG bank.com đang hiển thị — chính sách bảo mật của trình duyệt
CẤM đọc chéo domain) — form giả mạo GỬI ĐI **KHÔNG CÓ** token ĐÚNG,
`validateCsrfToken` từ chối. Lưu ý: `Math.random()` **KHÔNG PHẢI**
CSPRNG THẬT (bộ sinh số ngẫu nhiên MẬT MÃ), chỉ minh hoạ HÌNH DẠNG kỹ
thuật — sản phẩm THẬT dùng bộ sinh chuyên dụng.
::::

::::predict{#doan-form-gia-mao-khong-co-token commitOnce}
```typescript
function generateCsrfToken(): string {
  return Array.from({ length: 32 }, () => Math.floor(Math.random() * 36).toString(36)).join("");
}
function validateCsrfToken(sent: string, expected: string): boolean {
  return sent === expected;
}

// Server TẠO token, LƯU nó cho phiên đăng nhập
const tokenThat = generateCsrfToken();

// Form GIẢ MẠO từ evil.com KHÔNG đọc được tokenThat -- gửi một chuỗi RỖNG (không có field token nào)
const tokenNhanTuFormGiaMao = "";

console.log(validateCsrfToken(tokenNhanTuFormGiaMao, tokenThat));
```

Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì chuỗi RỖNG (`""`) là giá trị "mặc định"/"chưa xác định"
trong nhiều hệ thống, và `validateCsrfToken` có thể coi trường hợp
"chưa gửi token" là hợp lệ để không chặn nhầm request bình thường
::why
Gần đúng ở việc bạn nghĩ tới việc một số hệ thống CÓ xử lý ĐẶC BIỆT
cho "giá trị chưa gửi" (ví dụ optional field) — một trực giác hợp lý
cho MỘT SỐ tình huống KHÁC.

Chỗ lệch: `validateCsrfToken` ở ĐÂY KHÔNG có xử lý ĐẶC BIỆT nào cho
chuỗi rỗng — nó CHỈ có MỘT phép so sánh DUY NHẤT: `sent === expected`.
`tokenNhanTuFormGiaMao` là `""` (chuỗi RỖNG), `tokenThat` là chuỗi
NGẪU NHIÊN 32 KÝ TỰ (từ `generateCsrfToken`) — HAI chuỗi NÀY hầu như
CHẮC CHẮN khác nhau (`"" !== "abc123..."`) — `===` cho `false`.
KHÔNG có "trường hợp đặc biệt" nào được code này xử lý riêng.
::
:::

:::opt
Máy báo lỗi biên dịch — `generateCsrfToken()` được gọi để tạo
`tokenThat`, nhưng KHÔNG được gán vào biến `token` giống ví dụ trước,
TypeScript yêu cầu tên biến PHẢI khớp giữa các đoạn code
::why
Gần đúng ở việc bạn để ý TÊN BIẾN Ở ĐÂY (`tokenThat`) KHÁC với TÊN
BIẾN trong ví dụ TRƯỚC (`token`) — một quan sát ĐÚNG về sự KHÁC BIỆT
văn bản.

Chỗ lệch: TypeScript KHÔNG hề yêu cầu TÊN BIẾN phải "khớp" giữa các
đoạn code KHÁC NHAU — MỖI khối code trong lesson chạy Ở PHẠM VI RIÊNG
BIỆT, đặt tên biến THẾ NÀO tuỳ Ý (miễn tuân quy tắc đặt tên biến hợp
lệ của ngôn ngữ). Biên dịch sạch.
::
:::
::::

::::code{#viet_validatecsrftoken}
Tự viết `validateCsrfToken`.

```typescript title=starter
function generateCsrfToken(): string {
  return Array.from({ length: 32 }, () => Math.floor(Math.random() * 36).toString(36)).join("");
}

function validateCsrfToken(sent: string, expected: string): boolean {
  return ___;
}

const token = generateCsrfToken();
console.log(validateCsrfToken(token, token));
```

```typescript title=solution
function generateCsrfToken(): string {
  return Array.from({ length: 32 }, () => Math.floor(Math.random() * 36).toString(36)).join("");
}

function validateCsrfToken(sent: string, expected: string): boolean {
  return sent === expected;
}

const token = generateCsrfToken();
console.log(validateCsrfToken(token, token));
```

```typescript title=test
if (validateCsrfToken("abc", "abc") !== true) throw new Error("token GIỐNG NHAU phải hợp lệ");
if (validateCsrfToken("abc", "xyz") !== false) throw new Error("token KHÁC NHAU phải bị từ chối");
if (validateCsrfToken("", "abc") !== false) throw new Error("token RỖNG (không gửi) phải bị từ chối");

const t1 = generateCsrfToken();
const t2 = generateCsrfToken();
if (t1.length !== 32) throw new Error("token phải dài đúng 32 ký tự");
if (validateCsrfToken(t1, t2) === true && t1 !== t2) throw new Error("hai token NGẪU NHIÊN khác nhau không được coi là khớp");
```

:::hints
- kind: attention
  body: "So sánh sent và expected bằng === -- khớp CHÍNH XÁC thì hợp lệ, khác dù chỉ một ký tự cũng bị từ chối."
- kind: strategy
  body: "sent === expected — một phép so sánh chuỗi duy nhất."
- kind: one-line
  body: "___ = sent === expected"
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
CSRF: trình duyệt tự gửi cookie cross-site, SameSite + token chặn giả
mạo. Bước tiếp theo: chốt cụm — ghép tất cả kỹ thuật thành một bộ
nhận diện lỗ hổng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có NĂM dấu hiệu nhận biết (SQLi, XSS, CSP thiếu, CSRF thiếu, rò
rỉ bí mật hardcode). Viết MỘT hàm phân loại một đoạn code THEO loại
lỗ hổng nào — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
