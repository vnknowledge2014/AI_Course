---
id: ky-nghe-phan-mem.bao-mat-ung-dung.sql-injection-la-gi
title: "SQL Injection — khi input người dùng trở thành LỆNH thực thi"
summary: "Nối chuỗi TRỰC TIẾP khiến database KHÔNG phân biệt được code và dữ liệu — input '; DROP TABLE users; -- biến thành LỆNH SQL THẬT được thực thi. TypeScript không cảnh báo gì (chuỗi vẫn hợp lệ về mặt KIỂU) — lỗ hổng nằm ở TẦNG Ý NGHĨA, không phải cú pháp."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 15
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [bmud.sql-injection-attack]
requires: [bmud.defense-in-depth]
concepts: [bmud.sql-injection-attack]
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
`` `SELECT * FROM users WHERE email = '${email}'` `` — nối TRỰC TIẾP
dữ liệu người dùng vào SQL. Nếu `email` chứa ký tự đặc biệt của SQL?
::::

::::explain{#input-thanh-lenh}
Nối chuỗi **TRỰC TIẾP** khiến database **KHÔNG phân biệt được** "CODE"
(câu lệnh SQL) và "DỮ LIỆU" (nội dung người dùng nhập) — input `"';
DROP TABLE users; --"` biến thành **LỆNH SQL THẬT** được thực thi:

```typescript
function xayDungTruyVanKhongAnToan(email: string): string {
  return `SELECT * FROM users WHERE email = '${email}'`;
}

const emailBinhThuong = "an@shop.vn";
console.log(xayDungTruyVanKhongAnToan(emailBinhThuong));

const emailDocHai = "'; DROP TABLE users; --";
console.log(xayDungTruyVanKhongAnToan(emailDocHai));
```

```text
SELECT * FROM users WHERE email = 'an@shop.vn'
SELECT * FROM users WHERE email = ''; DROP TABLE users; --'
```

Dấu nháy đơn (`'`) TRONG `emailDocHai` **KẾT THÚC SỚM** chuỗi giá trị
SQL DỰ ĐỊNH — phần CÒN LẠI (`; DROP TABLE users; --`) trở thành MỘT
CÂU LỆNH SQL **RIÊNG**, tiếp nối câu lệnh gốc (dấu `;` phân tách hai
câu lệnh), `--` biến phần SAU thành COMMENT (vô hiệu hoá dấu nháy đơn
CÒN LẠI ở cuối template gốc, tránh lỗi cú pháp). Nếu database CHẤP
NHẬN chuỗi ĐÓ NGUYÊN VĂN, nó sẽ **XOÁ TOÀN BỘ bảng `users`**.
::::

::::example{#khong-canh-bao-ve-kieu}
Minh hoạ bằng CHUỖI (không database thật): `.includes("DROP TABLE")`
xác nhận câu lệnh ĐỘC HẠI đã "LỌT" vào chuỗi SQL CUỐI CÙNG:

```typescript title=readonly
function xayDungTruyVanKhongAnToan(email: string): string {
  return `SELECT * FROM users WHERE email = '${email}'`;
}

const emailDocHai = "'; DROP TABLE users; --";
const truyVanDocHai = xayDungTruyVanKhongAnToan(emailDocHai);
console.log(truyVanDocHai.includes("DROP TABLE"));
```

```text title=readonly
true
```

TypeScript **KHÔNG cảnh báo GÌ** — `emailDocHai` VẪN LÀ một `string`
HỢP LỆ về mặt KIỂU (KHÔNG có ràng buộc "không chứa ký tự SQL đặc
biệt" nào trên kiểu `string`), `xayDungTruyVanKhongAnToan` biên dịch
SẠCH, chạy ĐÚNG như ĐÃ khai — TRẢ VỀ một chuỗi. Lỗ hổng nằm Ở **TẦNG Ý
NGHĨA** (chuỗi TRẢ VỀ có Ý NGHĨA nguy hiểm KHI đưa vào database THẬT),
**KHÔNG PHẢI** tầng cú pháp/kiểu mà compiler kiểm được.
::::

::::predict{#doan-email-binh-thuong-khong-anh-huong commitOnce}
```typescript
function xayDungTruyVanKhongAnToan(email: string): string {
  return `SELECT * FROM users WHERE email = '${email}'`;
}

const emailCoDauNhay = "O'Brien@shop.vn"; // tên có dấu nháy đơn HỢP PHÁP (không phải tấn công)
const truyVan = xayDungTruyVanKhongAnToan(emailCoDauNhay);
console.log(truyVan);
```

Dòng cuối in ra gì?

:::opt{correct}
`SELECT * FROM users WHERE email = 'O'Brien@shop.vn'`
:::

:::opt
`SELECT * FROM users WHERE email = 'O''Brien@shop.vn'` — vì hàm TỰ
ĐỘNG NHÂN ĐÔI mọi dấu nháy đơn trong input để "escape" chúng, tránh
gây lỗi cú pháp SQL
::why
Gần đúng ở việc bạn nghĩ tới kỹ thuật "nhân đôi dấu nháy đơn" — đây
LÀ một kỹ thuật escaping SQL THẬT SỰ tồn tại (một trong nhiều cách
thoát ký tự đặc biệt trong SQL) — kiến thức ĐÓ đúng.

Chỗ lệch: `xayDungTruyVanKhongAnToan` (đúng như TÊN gọi — "KHÔNG an
toàn") KHÔNG hề làm BẤT KỲ escaping nào — nó CHỈ là MỘT template
string ĐƠN GIẢN, chèn `email` NGUYÊN VĂN vào GIỮA hai dấu nháy đơn.
KHÔNG có logic "nhân đôi dấu nháy" nào tồn tại trong thân hàm này —
`"O'Brien@shop.vn"` được chèn THẲNG, dấu nháy đơn của NÓ (hợp pháp,
KHÔNG PHẢI tấn công) VẪN gây ra VẤN ĐỀ TƯƠNG TỰ như tấn công (kết
thúc chuỗi SQL SỚM) — đây CHÍNH LÀ lý do "escape thủ công" KHÔNG đủ
tin cậy (bài SAU sẽ dạy giải pháp ĐÚNG: tham số hoá).
::
:::

:::opt
Máy báo lỗi biên dịch — chuỗi `"O'Brien@shop.vn"` chứa dấu nháy đơn
KHÔNG ĐƯỢC ESCAPE bên trong một chuỗi TypeScript khai bằng dấu nháy
kép, gây lỗi cú pháp NGAY khi khai biến
::why
Gần đúng ở việc bạn để ý dấu nháy ĐƠN (`'`) xuất hiện BÊN TRONG một
chuỗi — một chi tiết CÓ THẬT cần quan tâm khi VIẾT chuỗi TypeScript.

Chỗ lệch: `const emailCoDauNhay = "O'Brien@shop.vn";` dùng dấu **NHÁY
KÉP** (`"`) để BAO chuỗi — dấu nháy ĐƠN (`'`) BÊN TRONG hoàn toàn HỢP
LỆ, KHÔNG cần escape gì cả (TypeScript/JavaScript chỉ yêu cầu escape
dấu nháy TRÙNG LOẠI với dấu bao ngoài). Biên dịch sạch — vấn đề CHỈ
xuất hiện SAU ĐÓ, khi chuỗi này bị chèn vào MỘT chuỗi SQL khác (dùng
dấu nháy ĐƠN để bao giá trị).
::
:::
::::

::::code{#viet_xaydungtruyvan}
Tự viết `xayDungTruyVanKhongAnToan`.

```typescript title=starter
function xayDungTruyVanKhongAnToan(email: string): string {
  return ___;
}

console.log(xayDungTruyVanKhongAnToan("an@shop.vn"));
```

```typescript title=solution
function xayDungTruyVanKhongAnToan(email: string): string {
  return `SELECT * FROM users WHERE email = '${email}'`;
}

console.log(xayDungTruyVanKhongAnToan("an@shop.vn"));
```

```typescript title=test
const kqBinhThuong = xayDungTruyVanKhongAnToan("an@shop.vn");
if (!kqBinhThuong.includes("an@shop.vn")) throw new Error("câu lệnh phải chứa đúng email truyền vào");
if (!kqBinhThuong.startsWith("SELECT")) throw new Error("câu lệnh phải bắt đầu bằng SELECT");

const kqDocHai = xayDungTruyVanKhongAnToan("'; DROP TABLE users; --");
if (!kqDocHai.includes("DROP TABLE")) throw new Error("hàm KHÔNG an toàn phải để lộ đúng vấn đề: input độc hại lọt nguyên văn vào câu lệnh");
```

:::hints
- kind: attention
  body: "Ghép câu lệnh SQL bằng template string, chèn email TRỰC TIẾP giữa hai dấu nháy đơn của SQL — đây là bản KHÔNG AN TOÀN, cố ý minh hoạ vấn đề, không phải giải pháp."
- kind: strategy
  body: "`SELECT * FROM users WHERE email = '${email}'` — template string với email chèn thẳng."
- kind: one-line
  body: "___ = `SELECT * FROM users WHERE email = '${email}'`"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "SELECT"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
SQL Injection: nối chuỗi trực tiếp làm database không phân biệt được
lệnh và dữ liệu. Bước tiếp theo: giải pháp thật — tách lệnh khỏi dữ
liệu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vấn đề GỐC là nối chuỗi TRỰC TIẾP. Nếu TÁCH RIÊNG câu lệnh SQL (cố
định) khỏi DỮ LIỆU (truyền riêng), database có còn hiểu nhầm dữ liệu
là lệnh không?
::::

::::checkpoint{mastery=0.8}
::::
