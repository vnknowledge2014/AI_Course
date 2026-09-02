---
id: ky-nghe-phan-mem.bao-mat-ung-dung.header-bao-mat-tong-quan
title: "Security Headers — mỗi header chặn MỘT loại tấn công cụ thể"
summary: "Bảng ánh xạ header→mối đe doạ: CSP (bài 18) chặn script LẠ, HSTS ép HTTPS, X-Frame-Options chặn clickjacking, X-Content-Type-Options chặn MIME-sniffing. Mô hình hoá bằng hàm thuần addSecurityHeaders trả object MỚI (spread), không mutate object gốc."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 23
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [bmud.security-headers]
requires: [bmud.rate-limit-implement]
concepts: [bmud.security-headers]
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
Rate limiting bảo vệ khỏi brute-force. Nhiều mối đe doạ KHÁC
(clickjacking, MIME-sniffing) có header HTTP riêng để chặn — chúng
là gì?
::::

::::explain{#bang-anh-xa-header}
BẢNG ánh xạ header→mối đe doạ: **CSP** (bài 18, chặn script LẠ),
**HSTS** (ép HTTPS, chặn downgrade-attack), **X-Frame-Options** (chặn
**clickjacking** — nhúng trang vào iframe ẩn, lừa người dùng CLICK
vào NÚT thật của trang KHÁC mà KHÔNG BIẾT), **X-Content-Type-Options**
(chặn **MIME-sniffing** — trình duyệt "đoán" SAI loại file, thực thi
MỘT file ẢNH như JAVASCRIPT):

```typescript
type SecurityHeaders = Record<string, string>;

function addSecurityHeaders(headers: SecurityHeaders): SecurityHeaders {
  return {
    ...headers,
    "Content-Security-Policy": "default-src 'self'",
    "X-Frame-Options": "DENY",
    "X-Content-Type-Options": "nosniff",
  };
}

const headersGoc: SecurityHeaders = { "Content-Type": "application/json" };
console.log(JSON.stringify(addSecurityHeaders(headersGoc)));
```

```text
{"Content-Type":"application/json","Content-Security-Policy":"default-src 'self'","X-Frame-Options":"DENY","X-Content-Type-Options":"nosniff"}
```

Thư viện `helmet` (thực tế) set CẢ BỘ header NÀY CHỈ MỘT DÒNG (KHÔNG
dùng được trong sandbox — chỉ nhắc TÊN, tương tự cách track ĐÃ nhắc
bcrypt/jose Ở các bài trước) — `addSecurityHeaders` mô hình hoá CÙNG
Ý TƯỞNG bằng hàm THUẦN.
::::

::::example{#ham-thuan-khong-mutate}
`addSecurityHeaders` là hàm **THUẦN** — trả về object **MỚI** (dùng
spread `...headers`), **KHÔNG** thay đổi object GỐC truyền vào:

```typescript title=readonly
type SecurityHeaders = Record<string, string>;
function addSecurityHeaders(headers: SecurityHeaders): SecurityHeaders {
  return {
    ...headers,
    "Content-Security-Policy": "default-src 'self'",
    "X-Frame-Options": "DENY",
    "X-Content-Type-Options": "nosniff",
  };
}

const headersGoc: SecurityHeaders = { "Content-Type": "application/json" };
const headersMoi = addSecurityHeaders(headersGoc);

console.log(Object.keys(headersGoc).length);
console.log(Object.keys(headersMoi).length);
console.log(headersGoc === headersMoi);
```

```text title=readonly
1
4
false
```

`headersGoc` VẪN CHỈ CÓ `1` field (`"Content-Type"`) SAU KHI gọi
`addSecurityHeaders` — object GỐC KHÔNG BỊ ĐỘNG tới. `headersMoi` LÀ
object **MỚI HOÀN TOÀN** (`headersGoc === headersMoi` là `false`,
hai THAM CHIẾU KHÁC NHAU), CÓ `4` field (GỐC + BA header MỚI). Đây LÀ
tính THUẦN đã học xuyên suốt track FP: HÀM KHÔNG "phá" dữ liệu ĐẦU
VÀO, CHỈ TẠO MỚI.
::::

::::predict{#doan-goi-hai-lan commitOnce}
```typescript
type SecurityHeaders = Record<string, string>;
function addSecurityHeaders(headers: SecurityHeaders): SecurityHeaders {
  return {
    ...headers,
    "Content-Security-Policy": "default-src 'self'",
    "X-Frame-Options": "DENY",
    "X-Content-Type-Options": "nosniff",
  };
}

const headersGoc: SecurityHeaders = { "Content-Type": "application/json" };
const lanMot = addSecurityHeaders(headersGoc);
const lanHai = addSecurityHeaders(lanMot); // GỌI TIẾP trên KẾT QUẢ lần một

console.log(Object.keys(lanHai).length);
```

Dòng cuối in ra gì?

:::opt{correct}
`4`
:::

:::opt
`7` — vì `addSecurityHeaders` THÊM BA header MỚI Ở MỖI LẦN gọi, gọi
HAI LẦN LIÊN TIẾP thì `lanHai` phải có tổng CỘNG DỒN sáu field mới
cộng thêm một field gốc
::why
Gần đúng ở việc bạn nhớ ĐÚNG `addSecurityHeaders` THÊM BA header (CSP,
X-Frame-Options, X-Content-Type-Options) — quan sát về BA header đó
đúng.

Chỗ lệch: `{ ...headers, "Content-Security-Policy": "...", ... }` —
spread `...headers` COPY các field CỦA `headers` VÀO object MỚI, RỒI
BA dòng SAU **GHI ĐÈ** (KHÔNG PHẢI "thêm thêm") lên field CÙNG TÊN
NẾU đã tồn tại. Ở LẦN GỌI THỨ HAI (`addSecurityHeaders(lanMot)`),
`lanMot` ĐÃ CÓ SẴN BA header ĐÓ (từ lần MỘT) — spread COPY chúng
VÀO, RỒI BA dòng gán **GHI ĐÈ LẠI CHÍNH XÁC CÙNG BA KHOÁ** (giá trị
GIỐNG HỆT lần trước, vì hàm LUÔN gán CÙNG giá trị CỐ ĐỊNH) — KHÔNG
tạo THÊM field MỚI nào. `lanHai` VẪN chỉ có `4` field: `"Content-Type"`
(gốc) + BA header bảo mật (KHÔNG NHÂN ĐÔI).
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `addSecurityHeaders(lanMot)` (truyền KẾT
QUẢ của MỘT lời gọi TRƯỚC làm ĐẦU VÀO cho lời gọi SAU) không hợp lệ,
vì `lanMot` đã có kiểu CỤ THỂ HƠN `SecurityHeaders` (nhiều field hơn)
::why
Gần đúng ở việc bạn để ý `lanMot` (kết quả của LẦN GỌI ĐẦU) có NHIỀU
field HƠN input GỐC — một quan sát ĐÚNG về mặt SỐ LƯỢNG field.

Chỗ lệch: `lanMot` VẪN có kiểu `SecurityHeaders` (`Record<string,
string>`) — CÓ NHIỀU field HƠN KHÔNG làm nó "kiểu KHÁC" hay "cụ thể
hơn" theo nghĩa TypeScript hiểu — `Record<string, string>` chấp nhận
BẤT KỲ object nào có TOÀN BỘ giá trị LÀ `string`, BẤT KỂ có BAO NHIÊU
field. `addSecurityHeaders(lanMot)` hoàn toàn hợp lệ — TRUYỀN kết quả
của MỘT hàm làm ĐẦU VÀO cho CHÍNH NÓ (hoặc hàm KHÁC cùng kiểu) là kỹ
thuật CHUẨN (đã dùng NHIỀU lần: `chainResult`, pipeline...). Biên
dịch sạch.
::
:::
::::

::::code{#viet_addsecurityheaders}
Tự viết BA giá trị header trong `addSecurityHeaders`.

```typescript title=starter
type SecurityHeaders = Record<string, string>;

function addSecurityHeaders(headers: SecurityHeaders): SecurityHeaders {
  return {
    ...headers,
    "Content-Security-Policy": ___,
    "X-Frame-Options": ___,
    "X-Content-Type-Options": ___,
  };
}

console.log(JSON.stringify(addSecurityHeaders({})));
```

```typescript title=solution
type SecurityHeaders = Record<string, string>;

function addSecurityHeaders(headers: SecurityHeaders): SecurityHeaders {
  return {
    ...headers,
    "Content-Security-Policy": "default-src 'self'",
    "X-Frame-Options": "DENY",
    "X-Content-Type-Options": "nosniff",
  };
}

console.log(JSON.stringify(addSecurityHeaders({})));
```

```typescript title=test
const r = addSecurityHeaders({});
if (r["X-Frame-Options"] !== "DENY") throw new Error("X-Frame-Options phải là DENY (chặn nhúng trang vào iframe)");
if (r["X-Content-Type-Options"] !== "nosniff") throw new Error("X-Content-Type-Options phải là nosniff (chặn MIME-sniffing)");
if (!r["Content-Security-Policy"] || !r["Content-Security-Policy"].includes("'self'")) throw new Error("Content-Security-Policy phải chứa 'self'");

const headersGocTest: SecurityHeaders = { "X-Custom": "giu-nguyen" };
const ketQuaTest = addSecurityHeaders(headersGocTest);
if (headersGocTest["Content-Security-Policy"] !== undefined) throw new Error("addSecurityHeaders KHÔNG được mutate object gốc truyền vào");
if (ketQuaTest["X-Custom"] !== "giu-nguyen") throw new Error("header gốc phải được giữ lại trong object mới");
```

:::hints
- kind: attention
  body: "Ba header dùng giá trị CHUẨN của bảng ánh xạ đã học: CSP giới hạn nguồn về 'self', X-Frame-Options chặn TOÀN BỘ nhúng iframe (DENY), X-Content-Type-Options chặn đoán loại file (nosniff)."
- kind: strategy
  body: '"default-src \'self\'" : "DENY" : "nosniff" — ba giá trị chuẩn, đúng thứ tự ba header.'
- kind: one-line
  body: '___ (CSP) = "default-src \'self\'"\n___ (X-Frame-Options) = "DENY"\n___ (X-Content-Type-Options) = "nosniff"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "DENY"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Security headers: mỗi header chặn một loại tấn công riêng, hàm thuần
không mutate object gốc. Bước tiếp theo: không hardcode bí mật —
đọc từ môi trường, thất bại SỚM nếu thiếu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mật khẩu database, khoá API — hardcode TRỰC TIẾP trong code (như bài
20's ví dụ `ro_ri_bi_mat`) là SAI. Đọc từ MÔI TRƯỜNG lúc chạy, và LÀM
GÌ nếu biến môi trường đó THIẾU?
::::

::::checkpoint{mastery=0.8}
::::
