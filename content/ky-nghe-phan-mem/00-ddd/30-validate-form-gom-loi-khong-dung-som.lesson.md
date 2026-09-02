---
id: ky-nghe-phan-mem.ddd.validate-form-gom-loi-khong-dung-som
title: "Ứng dụng: validate form nhiều trường — gom HẾT lỗi, không dừng sớm"
summary: "Bài chốt cụm 5: áp dụng LẠI map2GomLoi (T4.5 bài 25) vào validate MỘT form ba trường thật (tenKhachHang, email, soLuong). validateFormDatHang(raw): Result<FormHopLe, string[]> — form sai CẢ BA trường phải trả ĐỦ BA lỗi trong MỘT lần. Ôn fail-fast (chainResult) vs collect-all (map2GomLoi)."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 30
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.form-validation-accumulate]
requires: [ddd.combine]
concepts: [ddd.form-validation-accumulate]
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
Bài chốt cụm 5. Một form đăng ký đơn hàng có BA trường — người dùng
muốn thấy HẾT lỗi MỘT LẦN, không phải sửa từng cái rồi bấm gửi lại.
::::

::::explain{#validate-form-3-truong}
`map2GomLoi` (T4.5 bài 25) CHỈ gộp HAI `Result`. Với BA trường độc
lập, LỒNG hai lời gọi `map2GomLoi` (giống cách `chainResult` LỒNG hai
lớp ở bài 24's capstone) — mỗi trường validate RIÊNG, error type LUÔN
là `string[]` (mảng, để GOM được nhiều lỗi từ nhiều trường):

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

type FormRaw = { tenKhachHang: string; email: string; soLuong: string };
type FormHopLe = { tenKhachHang: string; email: string; soLuong: number };

function validateTenKhachHang(ten: string): Result<string, string[]> {
  if (ten.trim() === "") return loi(["tên khách hàng không được để trống"]);
  return ok(ten);
}
function validateEmailForm(email: string): Result<string, string[]> {
  if (!email.includes("@")) return loi(["email không hợp lệ"]);
  return ok(email);
}
function validateSoLuongForm(soLuongText: string): Result<number, string[]> {
  const n = Number(soLuongText);
  if (Number.isNaN(n) || n <= 0) return loi(["số lượng phải là số dương"]);
  return ok(n);
}

function validateFormDatHang(raw: FormRaw): Result<FormHopLe, string[]> {
  return map2GomLoi(
    map2GomLoi(validateTenKhachHang(raw.tenKhachHang), validateEmailForm(raw.email), (ten, email) => ({ ten, email })),
    validateSoLuongForm(raw.soLuong),
    (tenEmail, soLuong) => ({ tenKhachHang: tenEmail.ten, email: tenEmail.email, soLuong }),
  );
}

const formSaiCaBa: FormRaw = { tenKhachHang: "", email: "khong-hop-le", soLuong: "abc" };
console.log(JSON.stringify(validateFormDatHang(formSaiCaBa)));
```

```text
{"kind":"loi","loi":["tên khách hàng không được để trống","email không hợp lệ","số lượng phải là số dương"]}
```

CẢ BA trường SAI → CẢ BA thông điệp XUẤT HIỆN trong MỘT mảng, một lần
duy nhất. Đây là **collect-all** (T4.5 bài 23-24: `map2GomLoi`) — ĐỐI
LẬP **fail-fast** của `chainResult`/`combine` (bài 23-29): fail-fast
dùng cho các bước PHỤ THUỘC (bước sau CẦN kết quả bước trước, dừng
sớm là ĐÚNG — không có gì để làm tiếp nếu bước trước hỏng); collect-all
dùng cho các trường ĐỘC LẬP (không trường nào cần trường khác, người
dùng CẦN thấy hết để sửa MỘT LẦN).
::::

::::example{#form-hop-le-va-mot-loi}
Form HỢP LỆ trả `ok` với `FormHopLe` ĐÃ CHUYỂN KIỂU đúng
(`soLuong: string` → `number`); form chỉ SAI MỘT trường vẫn hoạt động
ĐÚNG (mảng lỗi có ĐÚNG MỘT phần tử, không lẫn lỗi từ trường ĐÚNG):

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}
type FormRaw = { tenKhachHang: string; email: string; soLuong: string };
type FormHopLe = { tenKhachHang: string; email: string; soLuong: number };
function validateTenKhachHang(ten: string): Result<string, string[]> {
  if (ten.trim() === "") return loi(["tên khách hàng không được để trống"]);
  return ok(ten);
}
function validateEmailForm(email: string): Result<string, string[]> {
  if (!email.includes("@")) return loi(["email không hợp lệ"]);
  return ok(email);
}
function validateSoLuongForm(soLuongText: string): Result<number, string[]> {
  const n = Number(soLuongText);
  if (Number.isNaN(n) || n <= 0) return loi(["số lượng phải là số dương"]);
  return ok(n);
}
function validateFormDatHang(raw: FormRaw): Result<FormHopLe, string[]> {
  return map2GomLoi(
    map2GomLoi(validateTenKhachHang(raw.tenKhachHang), validateEmailForm(raw.email), (ten, email) => ({ ten, email })),
    validateSoLuongForm(raw.soLuong),
    (tenEmail, soLuong) => ({ tenKhachHang: tenEmail.ten, email: tenEmail.email, soLuong }),
  );
}

const formHopLe: FormRaw = { tenKhachHang: "Nguyễn An", email: "an@shop.vn", soLuong: "5" };
console.log(JSON.stringify(validateFormDatHang(formHopLe)));

const formMotLoi: FormRaw = { tenKhachHang: "Nguyễn An", email: "an@shop.vn", soLuong: "0" };
console.log(JSON.stringify(validateFormDatHang(formMotLoi)));
```

```text title=readonly
{"kind":"ok","giaTri":{"tenKhachHang":"Nguyễn An","email":"an@shop.vn","soLuong":5}}
{"kind":"loi","loi":["số lượng phải là số dương"]}
```

`soLuong: "0"` (đúng NGƯỠNG `<= 0`) bị TỪ CHỐI — hai trường CÒN LẠI
ĐÚNG nên KHÔNG góp lỗi nào vào mảng, mảng lỗi chỉ có ĐÚNG MỘT phần tử.
::::

::::predict{#doan-form-hai-loi commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}
type FormRaw = { tenKhachHang: string; email: string; soLuong: string };
type FormHopLe = { tenKhachHang: string; email: string; soLuong: number };
function validateTenKhachHang(ten: string): Result<string, string[]> {
  if (ten.trim() === "") return loi(["tên khách hàng không được để trống"]);
  return ok(ten);
}
function validateEmailForm(email: string): Result<string, string[]> {
  if (!email.includes("@")) return loi(["email không hợp lệ"]);
  return ok(email);
}
function validateSoLuongForm(soLuongText: string): Result<number, string[]> {
  const n = Number(soLuongText);
  if (Number.isNaN(n) || n <= 0) return loi(["số lượng phải là số dương"]);
  return ok(n);
}
function validateFormDatHang(raw: FormRaw): Result<FormHopLe, string[]> {
  return map2GomLoi(
    map2GomLoi(validateTenKhachHang(raw.tenKhachHang), validateEmailForm(raw.email), (ten, email) => ({ ten, email })),
    validateSoLuongForm(raw.soLuong),
    (tenEmail, soLuong) => ({ tenKhachHang: tenEmail.ten, email: tenEmail.email, soLuong }),
  );
}

// tên VÀ số lượng đều SAI, email ĐÚNG
const formHaiLoi: FormRaw = { tenKhachHang: "  ", email: "an@shop.vn", soLuong: "-3" };
const ketQua = validateFormDatHang(formHaiLoi);
console.log(ketQua.kind === "loi" ? ketQua.loi.length : -1);
```

Dòng cuối in ra gì?

:::opt{correct}
`2`
:::

:::opt
`3` — vì `validateFormDatHang` LUÔN chạy CẢ BA validate trước khi
quyết định, nên dù `email` đúng, hàm `validateEmailForm` vẫn được GỌI
và GÓP một chuỗi RỖNG vào mảng lỗi, khiến mảng có độ dài 3
::why
Gần đúng ở việc bạn nhớ ĐÚNG `validateEmailForm` VẪN được GỌI (đúng —
`map2GomLoi` KHÔNG short-circuit, mọi validate ĐỀU chạy, khác
`chainResult`) — quan sát về việc CÓ gọi đó đúng.

Chỗ lệch: `validateEmailForm("an@shop.vn")` (email HỢP LỆ) trả về
`ok("an@shop.vn")`, KHÔNG PHẢI `loi([""])`. `map2GomLoi` chỉ GÓP lỗi
từ nhánh `ra`/`rb` CÓ `kind === "loi"` — một nhánh `ok` KHÔNG đóng góp
GÌ vào mảng lỗi cuối cùng (không có "lỗi rỗng" nào được thêm vào).
Mảng lỗi CHỈ chứa đúng hai phần tử: lỗi TÊN và lỗi SỐ LƯỢNG.
::
:::

:::opt
Máy báo lỗi biên dịch — `map2GomLoi` LỒNG hai lớp không hợp lệ vì kiểu
lỗi `E` của lớp TRONG (`{ ten, email }`) khác kiểu lỗi của
`validateSoLuongForm` ở lớp NGOÀI
::why
Gần đúng ở việc bạn để ý CÓ hai LỚP `map2GomLoi` LỒNG NHAU với kiểu
GIÁ TRỊ (`A`, `B`, `C`) khác nhau ở mỗi lớp — quan sát về cấu trúc
lồng phức tạp đó đúng.

Chỗ lệch: `E` (kiểu LỖI, tham số kiểu THỨ TƯ của `map2GomLoi`) là
`string` Ở CẢ HAI lớp — LỚP TRONG trả `Result<{ten,email}, string[]>`,
LỚP NGOÀI nhận NÓ làm `ra` và `validateSoLuongForm(...)` (cũng
`Result<number, string[]>`) làm `rb` — CẢ HAI đều có `E = string`,
hoàn toàn khớp yêu cầu `Result<A, E[]>`/`Result<B, E[]>` của
`map2GomLoi`. Biên dịch sạch.
::
:::
::::

::::code{#viet_validate_form}
Tự viết `validateTenKhachHang` và phần kiểm ngưỡng của
`validateSoLuongForm`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

type FormRaw = { tenKhachHang: string; email: string; soLuong: string };
type FormHopLe = { tenKhachHang: string; email: string; soLuong: number };

function validateTenKhachHang(ten: string): Result<string, string[]> {
  if (ten.trim() === "") return ___;
  return ___;
}
function validateEmailForm(email: string): Result<string, string[]> {
  if (!email.includes("@")) return loi(["email không hợp lệ"]);
  return ok(email);
}
function validateSoLuongForm(soLuongText: string): Result<number, string[]> {
  const n = Number(soLuongText);
  if (Number.isNaN(n) || n <= 0) return ___;
  return ok(n);
}

function validateFormDatHang(raw: FormRaw): Result<FormHopLe, string[]> {
  return map2GomLoi(
    map2GomLoi(validateTenKhachHang(raw.tenKhachHang), validateEmailForm(raw.email), (ten, email) => ({ ten, email })),
    validateSoLuongForm(raw.soLuong),
    (tenEmail, soLuong) => ({ tenKhachHang: tenEmail.ten, email: tenEmail.email, soLuong }),
  );
}

console.log(JSON.stringify(validateFormDatHang({ tenKhachHang: "", email: "khong-hop-le", soLuong: "abc" })));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

type FormRaw = { tenKhachHang: string; email: string; soLuong: string };
type FormHopLe = { tenKhachHang: string; email: string; soLuong: number };

function validateTenKhachHang(ten: string): Result<string, string[]> {
  if (ten.trim() === "") return loi(["tên khách hàng không được để trống"]);
  return ok(ten);
}
function validateEmailForm(email: string): Result<string, string[]> {
  if (!email.includes("@")) return loi(["email không hợp lệ"]);
  return ok(email);
}
function validateSoLuongForm(soLuongText: string): Result<number, string[]> {
  const n = Number(soLuongText);
  if (Number.isNaN(n) || n <= 0) return loi(["số lượng phải là số dương"]);
  return ok(n);
}

function validateFormDatHang(raw: FormRaw): Result<FormHopLe, string[]> {
  return map2GomLoi(
    map2GomLoi(validateTenKhachHang(raw.tenKhachHang), validateEmailForm(raw.email), (ten, email) => ({ ten, email })),
    validateSoLuongForm(raw.soLuong),
    (tenEmail, soLuong) => ({ tenKhachHang: tenEmail.ten, email: tenEmail.email, soLuong }),
  );
}

console.log(JSON.stringify(validateFormDatHang({ tenKhachHang: "", email: "khong-hop-le", soLuong: "abc" })));
```

```typescript title=test
const caBaSai = validateFormDatHang({ tenKhachHang: "", email: "khong-hop-le", soLuong: "abc" });
if (caBaSai.kind !== "loi") throw new Error("form sai cả ba trường phải ra loi");
if (caBaSai.kind === "loi" && caBaSai.loi.length !== 3) throw new Error("form sai cả ba trường phải gom ĐỦ BA lỗi, không dừng ở lỗi đầu");
if (caBaSai.kind === "loi" && (!caBaSai.loi[0] || caBaSai.loi[0].trim() === "")) throw new Error("thông điệp lỗi tên KHÔNG được rỗng — phải mô tả rõ vấn đề, không phải chuỗi trống");
if (caBaSai.kind === "loi" && !caBaSai.loi.includes("tên khách hàng không được để trống")) throw new Error("mảng lỗi phải chứa ĐÚNG NGUYÊN VĂN thông điệp lỗi tên");

const hopLe = validateFormDatHang({ tenKhachHang: "An", email: "an@shop.vn", soLuong: "3" });
if (hopLe.kind !== "ok") throw new Error("form hợp lệ phải ra ok");
if (hopLe.kind === "ok" && hopLe.giaTri.soLuong !== 3) throw new Error("soLuong phải được chuyển đúng từ chuỗi sang số");

// Biên: soLuong đúng ngưỡng "0" phải bị từ chối, "1" (ngưỡng+1) phải hợp lệ
const bienZero = validateFormDatHang({ tenKhachHang: "An", email: "an@shop.vn", soLuong: "0" });
if (bienZero.kind !== "loi") throw new Error("soLuong = 0 (đúng ngưỡng) phải bị từ chối");
if (bienZero.kind === "loi" && bienZero.loi.length !== 1) throw new Error("chỉ soLuong sai thì mảng lỗi phải có đúng MỘT phần tử");

const bienMot = validateFormDatHang({ tenKhachHang: "An", email: "an@shop.vn", soLuong: "1" });
if (bienMot.kind !== "ok") throw new Error("soLuong = 1 (ngưỡng + 1) phải hợp lệ");
```

:::hints
- kind: attention
  body: "validateTenKhachHang: nhánh lỗi bọc loi([\"...\"]) (mảng MỘT phần tử, string[]); nhánh hợp lệ trả ok(ten). validateSoLuongForm: nhánh lỗi (n là NaN HOẶC n <= 0) bọc loi([\"...\"]) cùng dạng."
- kind: strategy
  body: 'loi(["tên khách hàng không được để trống"]) : ok(ten) — validateTenKhachHang. loi(["số lượng phải là số dương"]) — validateSoLuongForm nhánh lỗi.'
- kind: one-line
  body: '___ (validateTenKhachHang lỗi) = loi(["tên khách hàng không được để trống"])\n___ (validateTenKhachHang ok) = ok(ten)\n___ (validateSoLuongForm lỗi) = loi(["số lượng phải là số dương"])'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "loi"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 5 hoàn tất: throw giấu lỗi, unwrapOr/match trích giá trị/xử lý cả
hai nhánh, fromThrowable bọc code cũ, combine/map2GomLoi gộp Result —
fail-fast khi phụ thuộc, collect-all khi độc lập. Cụm tiếp theo:
Serialization & DTOs.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Domain Model (cụm 3) và dữ liệu GỬI QUA MẠNG (JSON) thường KHÔNG cùng
hình dạng — dùng CHUNG một type cho cả hai có vấn đề gì?
::::

::::checkpoint{mastery=0.8}
::::
