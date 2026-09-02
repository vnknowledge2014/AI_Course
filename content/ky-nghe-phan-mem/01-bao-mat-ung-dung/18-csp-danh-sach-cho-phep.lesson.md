---
id: ky-nghe-phan-mem.bao-mat-ung-dung.csp-danh-sach-cho-phep
title: "CSP — chính sách khai báo NGUỒN nào được phép chạy"
summary: "Content Security Policy: HTTP header khai báo NGUỒN script được PHÉP chạy — lớp phòng thủ THỨ HAI, hoạt động NGAY CẢ nếu escape bị bỏ sót. Mô hình hoá bằng policy object + checker function (KHÔNG PHẢI CSP thật — cần trình duyệt thật)."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 18
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [bmud.csp-allowlist]
requires: [bmud.xss-escape-output]
concepts: [bmud.csp-allowlist]
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
Escape (bài trước) là lớp phòng thủ Ở TẦNG CODE — dễ quên MỘT chỗ.
Ra lệnh cho TRÌNH DUYỆT tự chặn script LẠ, dù escape có sót — được
không?
::::

::::explain{#csp-la-danh-sach-cho-phep}
**Content Security Policy (CSP)**: HTTP header khai báo **NGUỒN**
script được **PHÉP CHẠY** (`script-src 'self'` = CHỈ script TỪ CHÍNH
domain) — lớp phòng thủ **THỨ HAI**, hoạt động **NGAY CẢ** nếu escape
(bài 17) bị bỏ sót Ở ĐÂU ĐÓ. **Mô hình hoá, KHÔNG PHẢI CSP thật**
(CSP cần trình duyệt THẬT để thực thi — KHÔNG giả lập được trong
sandbox):

```typescript
type CspPolicy = { scriptSrc: string[] };

function isScriptAllowed(src: string, policy: CspPolicy): boolean {
  return policy.scriptSrc.includes(src);
}

const policy: CspPolicy = { scriptSrc: ["'self'", "https://cdn.tin-cay.vn"] };
console.log(isScriptAllowed("'self'", policy));
console.log(isScriptAllowed("https://evil.com", policy));
```

```text
true
false
```

`isScriptAllowed` kiểm `policy.scriptSrc.includes(src)` — GIỮ Ý
TƯỞNG **allowlist** (chỉ NGUỒN được LIỆT KÊ mới "chạy"), **KHÔNG
PHẢI** cơ chế header/trình duyệt THẬT (trình duyệt THẬT đọc header
`Content-Security-Policy` VÀ TỰ CHẶN request tải script từ nguồn
KHÔNG có trong danh sách — hành vi ĐÓ không thể chạy trong sandbox).
::::

::::example{#lop-phong-thu-doc-lap-voi-escape}
CSP là lớp **ĐỘC LẬP** với escape (bài 17) — dù `escapeHtml` (bài
trước) có QUÊN escape MỘT chỗ, CSP VẪN chặn script từ nguồn LẠ:

```typescript title=readonly
type CspPolicy = { scriptSrc: string[] };
function isScriptAllowed(src: string, policy: CspPolicy): boolean {
  return policy.scriptSrc.includes(src);
}

function escapeHtml(unsafe: string): string {
  return unsafe.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

// Giả sử một chỗ QUÊN gọi escapeHtml (bug) -- nội dung độc hại LỌT qua
const noiDungBiBoQuenEscape = '<script src="https://evil.com/steal.js"></script>';

// NHƯNG CSP (lớp KHÁC, độc lập) vẫn kiểm được nguồn script
const policy: CspPolicy = { scriptSrc: ["'self'"] };
console.log(isScriptAllowed("https://evil.com/steal.js", policy));
```

```text title=readonly
false
```

CHÍNH VÌ escape VÀ CSP là HAI LỚP **ĐỘC LẬP** (nối bài 14's "phòng
thủ nhiều lớp"), MỘT bug quên escape (lớp THỨ NHẤT thất bại) KHÔNG
đồng nghĩa TOÀN BỘ hệ thống mất phòng thủ — CSP (lớp THỨ HAI) VẪN
đứng vững, TRÌNH DUYỆT THẬT sẽ TỪ CHỐI tải script từ `evil.com` dù
thẻ `<script>` ĐÓ có xuất hiện TRONG HTML.
::::

::::predict{#doan-them-nguon-vao-policy commitOnce}
```typescript
type CspPolicy = { scriptSrc: string[] };
function isScriptAllowed(src: string, policy: CspPolicy): boolean {
  return policy.scriptSrc.includes(src);
}

const policy: CspPolicy = { scriptSrc: ["'self'"] };
console.log(isScriptAllowed("https://cdn.moi.vn", policy));

// THÊM một nguồn MỚI vào policy (mảng SCRIPTSRC, KHÔNG tạo policy MỚI)
policy.scriptSrc.push("https://cdn.moi.vn");
console.log(isScriptAllowed("https://cdn.moi.vn", policy));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`false` rồi `true`
:::

:::opt
`false` rồi `false` — vì `policy` được khai bằng `const`, KHÔNG THỂ
thay đổi SAU KHI tạo, `policy.scriptSrc.push(...)` bị bỏ qua ÂM THẦM
::why
Gần đúng ở việc bạn nhớ ĐÚNG `policy` khai bằng `const` — MỘT quan
sát ĐÚNG về TỪ KHOÁ khai báo (đã học: `const` ràng buộc BIẾN, không
cho GÁN LẠI).

Chỗ lệch: `const` CHỈ ngăn **GÁN LẠI TOÀN BỘ biến** (`policy = {...
mới}` MỚI bị lỗi) — nó **KHÔNG** ngăn việc **THAY ĐỔI NỘI DUNG BÊN
TRONG** object mà biến đó TRỎ TỚI. `policy.scriptSrc.push(...)`
KHÔNG gán lại `policy` — nó gọi `.push` TRÊN MẢNG `scriptSrc` (một
object CON, MUTABLE), THÊM MỘT phần tử vào MẢNG ĐÓ. `policy` (biến)
vẫn TRỎ tới CÙNG MỘT object, chỉ NỘI DUNG mảng bên trong đã ĐỔI. Lần
gọi `isScriptAllowed` THỨ HAI đọc `policy.scriptSrc` (ĐÃ có
`"https://cdn.moi.vn"`) → `true`.
::
:::

:::opt
Máy báo lỗi biên dịch — `policy.scriptSrc.push(...)` không hợp lệ vì
`scriptSrc: string[]` được suy ra là MẢNG READONLY từ khai báo
`CspPolicy`
::why
Gần đúng ở việc bạn nghĩ tới khả năng MỘT mảng bị khai `readonly` —
một mối lo HỢP LÝ, vì track NÀY có dùng `readonly Quyen[]` ở vài nơi
KHÁC (bài 8).

Chỗ lệch: `CspPolicy` khai `scriptSrc: string[]` — **KHÔNG** có từ
khoá `readonly` (khác VỚI `quyenTheoVaiTro`'s `readonly Quyen[]` ở
bài 8). `string[]` LÀ mảng THƯỜNG, CHO PHÉP mọi thao tác biến đổi,
bao gồm `.push(...)`. Biên dịch sạch.
::
:::
::::

::::code{#viet_isscriptallowed}
Tự viết `isScriptAllowed`.

```typescript title=starter
type CspPolicy = { scriptSrc: string[] };

function isScriptAllowed(src: string, policy: CspPolicy): boolean {
  return ___;
}

const policy: CspPolicy = { scriptSrc: ["'self'"] };
console.log(isScriptAllowed("'self'", policy));
```

```typescript title=solution
type CspPolicy = { scriptSrc: string[] };

function isScriptAllowed(src: string, policy: CspPolicy): boolean {
  return policy.scriptSrc.includes(src);
}

const policy: CspPolicy = { scriptSrc: ["'self'"] };
console.log(isScriptAllowed("'self'", policy));
```

```typescript title=test
const policyTest: CspPolicy = { scriptSrc: ["'self'", "https://cdn.tin-cay.vn"] };
if (isScriptAllowed("'self'", policyTest) !== true) throw new Error("nguồn có trong danh sách phải được cho phép");
if (isScriptAllowed("https://cdn.tin-cay.vn", policyTest) !== true) throw new Error("nguồn thứ hai trong danh sách cũng phải được cho phép");
if (isScriptAllowed("https://evil.com", policyTest) !== false) throw new Error("nguồn KHÔNG có trong danh sách phải bị từ chối");

const policyRong: CspPolicy = { scriptSrc: [] };
if (isScriptAllowed("'self'", policyRong) !== false) throw new Error("danh sách rỗng phải từ chối MỌI nguồn, kể cả 'self'");
```

:::hints
- kind: attention
  body: "Kiểm src có nằm trong mảng policy.scriptSrc không, dùng Array.includes."
- kind: strategy
  body: "policy.scriptSrc.includes(src) — một biểu thức duy nhất."
- kind: one-line
  body: "___ = policy.scriptSrc.includes(src)"
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
CSP: chính sách allowlist, lớp phòng thủ độc lập với escape. Bước
tiếp theo: một lỗ hổng KHÁC HẲN — trình duyệt tự gửi cookie tới nơi
bạn không hề định gửi tới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trình duyệt TỰ ĐỘNG đính kèm cookie khi gửi request TỚI một domain đã
đăng nhập — KỂ CẢ khi request đó xuất phát từ một trang WEB KHÁC.
Điều đó có nguy hiểm gì?
::::

::::checkpoint{mastery=0.8}
::::
