---
id: ky-nghe-phan-mem.bao-mat-ung-dung.nhan-dien-lo-hong
title: "Capstone: Nhận diện lỗ hổng — phân loại đoạn code SQLi/XSS/rò rỉ bí mật"
summary: "Bài chốt cụm 3, code có chấm điểm sống: classifyVulnerability(doanCode): LoaiLoHong nhận diện BA dấu hiệu ĐẶC TRƯNG (nối bài 15-19): nối chuỗi trực tiếp vào SQL, render innerHTML không escape, secret hardcode trực tiếp trong chuỗi code. Đoạn code AN TOÀN (đã tham số hoá) phải phân loại đúng."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 20
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [bmud.vulnerability-spotter]
requires: [bmud.csrf-samesite-token]
concepts: [bmud.vulnerability-spotter]
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
Bài chốt cụm 3. Bạn có BA dấu hiệu nhận biết (SQLi, XSS, rò rỉ bí
mật). Viết MỘT hàm phân loại một đoạn code THEO loại lỗ hổng nào?
::::

::::explain{#classify-vulnerability}
`classifyVulnerability(doanCode: string): LoaiLoHong` — DU
(`"sql_injection" | "xss" | "ro_ri_bi_mat" | "an_toan"`) nhận diện
BA dấu hiệu ĐẶC TRƯNG (nối bài 15-19): nối chuỗi TRỰC TIẾP vào SQL
(chứa `SELECT` VÀ `${`); render `.innerHTML` KHÔNG escape (chứa
`.innerHTML` VÀ `${`); secret hardcode TRỰC TIẾP trong chuỗi code
(khoá API kiểu `sk_live_...` hoặc `password = "..."`):

```typescript
type LoaiLoHong = "sql_injection" | "xss" | "ro_ri_bi_mat" | "an_toan";

function classifyVulnerability(doanCode: string): LoaiLoHong {
  if (doanCode.includes("SELECT") && doanCode.includes("${")) return "sql_injection";
  if (doanCode.includes(".innerHTML") && doanCode.includes("${")) return "xss";
  if (doanCode.includes("sk_live_") || /password\s*=\s*["']/.test(doanCode)) return "ro_ri_bi_mat";
  return "an_toan";
}

const doanSqli = "const q = `SELECT * FROM users WHERE email = '${email}'`;";
const doanXss = "el.innerHTML = `<div>${tenNguoiDung}</div>`;";

console.log(classifyVulnerability(doanSqli));
console.log(classifyVulnerability(doanXss));
```

```text
sql_injection
xss
```

MỖI dấu hiệu kiểm **HAI ĐIỀU KIỆN CÙNG LÚC** (`&&`) — CHỈ có `SELECT`
KHÔNG ĐỦ (nhiều đoạn SQL AN TOÀN vẫn CÓ `SELECT`), CẦN CẢ `${}`
(template interpolation — dấu hiệu của việc CHÈN biến TRỰC TIẾP).
::::

::::example{#doan-an-toan-khong-bi-nham-lan}
Đoạn code AN TOÀN (ĐÃ tham số hoá, bài 16) VẪN CÓ chữ `SELECT`, NHƯNG
**KHÔNG** có `${}` — `classifyVulnerability` PHÂN BIỆT được:

```typescript title=readonly
type LoaiLoHong = "sql_injection" | "xss" | "ro_ri_bi_mat" | "an_toan";
function classifyVulnerability(doanCode: string): LoaiLoHong {
  if (doanCode.includes("SELECT") && doanCode.includes("${")) return "sql_injection";
  if (doanCode.includes(".innerHTML") && doanCode.includes("${")) return "xss";
  if (doanCode.includes("sk_live_") || /password\s*=\s*["']/.test(doanCode)) return "ro_ri_bi_mat";
  return "an_toan";
}

const doanRoRi = 'const stripeKey = "sk_live_51H8xJ2K...";';
const doanAnToan = "const q = { sql: 'SELECT * FROM users WHERE email = $1', params: [email] };";

console.log(classifyVulnerability(doanRoRi));
console.log(classifyVulnerability(doanAnToan));
```

```text title=readonly
ro_ri_bi_mat
an_toan
```

`doanAnToan` **CÓ** chữ `SELECT` — NHƯNG KHÔNG có `${}` (giá trị đi
qua `params`, KHÔNG chèn TRỰC TIẾP vào chuỗi `sql`, ĐÚNG kỹ thuật bài
16) → điều kiện `&&` **THẤT BẠI** (thiếu vế `${}`) → KHÔNG rơi vào
nhánh `"sql_injection"` → cuối cùng rơi vào `"an_toan"`.
::::

::::predict{#doan-thu-tu-kiem-tra-quan-trong commitOnce}
```typescript
type LoaiLoHong = "sql_injection" | "xss" | "ro_ri_bi_mat" | "an_toan";
function classifyVulnerability(doanCode: string): LoaiLoHong {
  if (doanCode.includes("SELECT") && doanCode.includes("${")) return "sql_injection";
  if (doanCode.includes(".innerHTML") && doanCode.includes("${")) return "xss";
  if (doanCode.includes("sk_live_") || /password\s*=\s*["']/.test(doanCode)) return "ro_ri_bi_mat";
  return "an_toan";
}

// Đoạn code CÓ CẢ hai dấu hiệu: SQL injection VÀ secret hardcode
const doanHaiLoHong = 'const apiKey = "sk_live_ABC"; const q = `SELECT * FROM t WHERE k = \'${apiKey}\'`;';
console.log(classifyVulnerability(doanHaiLoHong));
```

Dòng cuối in ra gì?

:::opt{correct}
`sql_injection`
:::

:::opt
`ro_ri_bi_mat` — vì "rò rỉ bí mật" (lộ khoá API) là vấn đề NGHIÊM
TRỌNG HƠN SQL Injection, nên hàm ưu tiên báo cáo NÓ trước
::why
Gần đúng ở việc bạn nghĩ tới một hệ thống "ưu tiên theo MỨC ĐỘ
NGHIÊM TRỌNG" — một thiết kế HỢP LÝ cho một số công cụ phân tích bảo
mật THẬT (chấm điểm CVSS, xếp hạng RỦI RO).

Chỗ lệch: `classifyVulnerability` **KHÔNG** có khái niệm "mức độ
nghiêm trọng" — nó CHỈ kiểm CÁC ĐIỀU KIỆN **THEO THỨ TỰ VIẾT TRONG
CODE**, `return` NGAY khi điều kiện ĐẦU TIÊN khớp. Điều kiện SQL
injection (`SELECT` + `${}`) được viết **TRƯỚC** điều kiện rò rỉ bí
mật — `doanHaiLoHong` khớp CẢ HAI điều kiện, NHƯNG hàm `return
"sql_injection"` NGAY tại điều kiện ĐẦU TIÊN khớp, KHÔNG BAO GIỜ chạm
tới điều kiện `sk_live_` phía SAU. Đây LÀ đặc điểm CHUNG của
`if`/`else if` NỐI TIẾP (đã học xuyên suốt track): thứ tự VIẾT quyết
định KẾT QUẢ khi NHIỀU điều kiện CÙNG đúng.
::
:::

:::opt
Máy báo lỗi biên dịch — chuỗi `doanHaiLoHong` chứa CẢ dấu nháy kép
LẪN dấu nháy đơn ĐƯỢC ESCAPE (`\'`) bên trong MỘT chuỗi bao bởi dấu
nháy đơn, TypeScript không cho phép TRỘN LẪN kiểu dấu nháy như vậy
::why
Gần đúng ở việc bạn để ý chuỗi `doanHaiLoHong` khá PHỨC TẠP (CÓ dấu
nháy kép BÊN TRONG, CÓ dấu nháy đơn ĐƯỢC ESCAPE `\'`) — một quan sát
ĐÚNG về ĐỘ RẮC RỐI của cú pháp chuỗi này.

Chỗ lệch: JavaScript/TypeScript CHO PHÉP escape MỘT ký tự (`\'`) BÊN
TRONG một chuỗi bao bởi CHÍNH ký tự ĐÓ (`'`) — đây LÀ cú pháp CHUẨN
để CHÈN một dấu nháy đơn VÀO GIỮA nội dung chuỗi mà KHÔNG kết thúc
chuỗi SỚM. Việc chuỗi ĐÓ CÒN chứa dấu nháy KÉP (`"`) bên trong KHÔNG
gây xung đột gì — hai LOẠI dấu nháy hoàn toàn ĐỘC LẬP. Biên dịch
sạch.
::
:::
::::

::::code{#viet_classifyvulnerability}
Tự viết BỐN nhánh của `classifyVulnerability`.

```typescript title=starter
type LoaiLoHong = "sql_injection" | "xss" | "ro_ri_bi_mat" | "an_toan";

function classifyVulnerability(doanCode: string): LoaiLoHong {
  if (doanCode.includes("SELECT") && doanCode.includes("${")) return ___;
  if (doanCode.includes(".innerHTML") && doanCode.includes("${")) return ___;
  if (doanCode.includes("sk_live_") || /password\s*=\s*["']/.test(doanCode)) return ___;
  return ___;
}

console.log(classifyVulnerability("const q = `SELECT * FROM users WHERE email = '${email}'`;"));
```

```typescript title=solution
type LoaiLoHong = "sql_injection" | "xss" | "ro_ri_bi_mat" | "an_toan";

function classifyVulnerability(doanCode: string): LoaiLoHong {
  if (doanCode.includes("SELECT") && doanCode.includes("${")) return "sql_injection";
  if (doanCode.includes(".innerHTML") && doanCode.includes("${")) return "xss";
  if (doanCode.includes("sk_live_") || /password\s*=\s*["']/.test(doanCode)) return "ro_ri_bi_mat";
  return "an_toan";
}

console.log(classifyVulnerability("const q = `SELECT * FROM users WHERE email = '${email}'`;"));
```

```typescript title=test
if (classifyVulnerability("const q = `SELECT * FROM users WHERE email = '${email}'`;") !== "sql_injection") throw new Error("SQL nối chuỗi trực tiếp phải phân loại sql_injection");
if (classifyVulnerability("el.innerHTML = `<div>${tenNguoiDung}</div>`;") !== "xss") throw new Error("innerHTML chèn biến trực tiếp phải phân loại xss");
if (classifyVulnerability('const stripeKey = "sk_live_51H8xJ2K...";') !== "ro_ri_bi_mat") throw new Error("khoá API hardcode phải phân loại ro_ri_bi_mat");
if (classifyVulnerability('const password = "matkhau123";') !== "ro_ri_bi_mat") throw new Error("mật khẩu hardcode phải phân loại ro_ri_bi_mat");
if (classifyVulnerability("const q = { sql: 'SELECT * FROM users WHERE email = $1', params: [email] };") !== "an_toan") throw new Error("SQL đã tham số hoá phải phân loại an_toan, dù vẫn chứa chữ SELECT");
if (classifyVulnerability("const x = 1 + 1;") !== "an_toan") throw new Error("code không liên quan bảo mật phải phân loại an_toan");
```

:::hints
- kind: attention
  body: "Bốn nhánh trả về ĐÚNG bốn giá trị của LoaiLoHong theo thứ tự: sql_injection, xss, ro_ri_bi_mat, an_toan (nhánh cuối, mặc định khi không khớp gì)."
- kind: strategy
  body: '"sql_injection" : "xss" : "ro_ri_bi_mat" : "an_toan" — bốn literal string, khớp đúng bốn nhánh theo thứ tự.'
- kind: one-line
  body: '___ (SQL) = "sql_injection"\n___ (XSS) = "xss"\n___ (rò rỉ) = "ro_ri_bi_mat"\n___ (mặc định) = "an_toan"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "sql_injection"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 3 hoàn tất: SQL Injection, tham số hoá, XSS/escape, CSP, CSRF —
nhận diện được cả năm. Cụm cuối: runtime hardening — giới hạn tốc độ,
header bảo mật, quản lý bí mật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ai đó gửi HÀNG NGHÌN request MỘT GIÂY tới endpoint đăng nhập, thử mọi
mật khẩu (brute-force). Làm sao GIỚI HẠN tốc độ request từ MỘT nguồn?
::::

::::checkpoint{mastery=0.8}
::::
