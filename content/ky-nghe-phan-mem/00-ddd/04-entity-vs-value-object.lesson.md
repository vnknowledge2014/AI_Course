---
id: ky-nghe-phan-mem.ddd.entity-vs-value-object
title: "Entity vs Value Object — so sánh bằng ID hay bằng giá trị"
summary: "Entity: có identity (ID) — hai instance CÙNG ID vẫn là \"cùng thực thể\" dù field đổi. Value Object: KHÔNG identity — hai instance CÙNG giá trị LÀ bằng nhau, immutable. Smart constructor cho VO trả THẲNG Result<T,E> đã học ở T4.5."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.entity-vs-value-object]
requires: [alg.gate-boss]
concepts: [ddd.entity-vs-value-object]
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
Trong domain, có HAI loại đối tượng khác nhau về BẢN CHẤT — không phải
khác về cấu trúc dữ liệu, mà khác về CÁCH so sánh "đây có phải CÙNG một
thứ không".
::::

::::explain{#entity-vs-vo}
**Entity**: có **identity** (một ID định danh). Hai instance CÙNG ID
VẪN LÀ "cùng thực thể" — dù các field khác đổi. Hai instance CÙNG field
nhưng KHÁC ID vẫn là HAI thực thể khác nhau:

```typescript
type KhachHang = { maKhachHang: string; ten: string; email: string };

function cungThucThe(a: KhachHang, b: KhachHang): boolean {
  return a.maKhachHang === b.maKhachHang;
}

const kh1: KhachHang = { maKhachHang: "KH-01", ten: "An", email: "an@example.com" };
const kh2: KhachHang = { maKhachHang: "KH-01", ten: "An Nguyen", email: "an2@example.com" };
const kh3: KhachHang = { maKhachHang: "KH-02", ten: "An", email: "an@example.com" };

console.log(cungThucThe(kh1, kh2));
console.log(cungThucThe(kh1, kh3));
```

```text
true
false
```

`kh1` và `kh2` có `maKhachHang` GIỐNG NHAU (`"KH-01"`) dù `ten`/`email`
KHÁC — VẪN là cùng một khách hàng (có lẽ `kh2` là `kh1` SAU KHI cập
nhật thông tin). `kh1` và `kh3` có `ten`/`email` GIỐNG HỆT nhưng
`maKhachHang` KHÁC — HAI khách hàng KHÁC NHAU (trùng tên trùng email
là có thật, không đủ để coi là "cùng người").

**Value Object (VO)**: KHÔNG có identity riêng — hai instance CÙNG
giá trị LÀ BẰNG NHAU, immutable, mọi phép toán trả về object MỚI. Smart
constructor cho VO trả THẲNG `Result<T,E>` đã học ở T4.5 (không phải
`| undefined`):

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type Email = string & { readonly __brand: "Email" };
function Email(vb: string): Result<Email, string> {
  return vb.includes("@") ? ok(vb as Email) : loi(`"${vb}" thiếu ký tự @`);
}

console.log(JSON.stringify(Email("an@example.com")));
console.log(JSON.stringify(Email("khong-hop-le")));
```

```text
{"kind":"ok","giaTri":"an@example.com"}
{"kind":"loi","loi":"\"khong-hop-le\" thiếu ký tự @"}
```

`Email` KHÔNG có ID — hai `Email` CÙNG chuỗi ký tự LÀ bằng nhau, không
cần so sánh gì thêm ngoài giá trị bên trong.
::::

::::example{#tienvnd-va-cong-tien}
Value Object có phép toán RIÊNG — luôn trả về VO MỚI, giữ type-safety
xuyên chuỗi:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type TienVND = number & { readonly __brand: "TienVND" };
function TienVND(soTien: number): Result<TienVND, string> {
  return soTien >= 0 ? ok(soTien as TienVND) : loi("tiền không được âm");
}
function congTien(a: TienVND, b: TienVND): TienVND {
  return (a + b) as TienVND;
}

const t1 = TienVND(50000);
const t2 = TienVND(30000);
if (t1.kind === "ok" && t2.kind === "ok") {
  console.log(congTien(t1.giaTri, t2.giaTri));
}
```

```text title=readonly
80000
```

`congTien` nhận HAI `TienVND` (đã ĐƯỢC KIỂM qua smart constructor,
không âm), trả về MỘT `TienVND` MỚI — người gọi tiếp theo KHÔNG cần
kiểm lại `>= 0`, vì `TienVND` type ĐÃ chứng minh điều đó (đúng "Parse,
don't validate" đã học ở T4.3's smart constructor).
::::

::::predict{#doan-entity-doi-ten commitOnce}
```typescript
type KhachHang = { maKhachHang: string; ten: string; email: string };
function cungThucThe(a: KhachHang, b: KhachHang): boolean {
  return a.maKhachHang === b.maKhachHang;
}

const banDau: KhachHang = { maKhachHang: "KH-07", ten: "Binh", email: "binh@example.com" };
const sauKhiDoiTen: KhachHang = { maKhachHang: "KH-07", ten: "Binh Tran", email: "binh@example.com" };

console.log(cungThucThe(banDau, sauKhiDoiTen));
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
`false` — vì `ten` đã đổi từ `"Binh"` thành `"Binh Tran"`, hai object
không còn GIỐNG HỆT nhau nữa
::why
Gần đúng ở việc bạn nhận ra `ten` THẬT SỰ khác nhau giữa hai object —
quan sát về sự khác biệt Ở FIELD đó đúng.

Chỗ lệch: `cungThucThe` KHÔNG so sánh TOÀN BỘ object (không phải deep
equal) — nó CHỈ so sánh `maKhachHang` (`a.maKhachHang === b.maKhachHang`).
Đây LÀ điểm CỐT LÕI của Entity: identity KHÔNG phụ thuộc field nào khác
ngoài ID. `banDau.maKhachHang` và `sauKhiDoiTen.maKhachHang` CÙNG là
`"KH-07"` — VẪN là cùng một khách hàng, dù tên đã đổi.
::
:::

:::opt
Máy báo lỗi biên dịch — `KhachHang` không có phương thức so sánh riêng
nên `cungThucThe` không thể hoạt động đúng
::why
Gần đúng ở việc bạn nghĩ tới việc `KhachHang` cần một CÁCH so sánh —
quan tâm đó hợp lý cho một Entity.

Chỗ lệch: `cungThucThe` LÀ chính cách so sánh đó — một hàm THUẦN, không
cần method gắn trên `KhachHang`. Nó chỉ đọc field `maKhachHang` của cả
hai tham số (phép truy cập field bình thường, `object.property`), biên
dịch và chạy hoàn toàn bình thường, không cần `KhachHang` "có" gì đặc
biệt.
::
:::
::::

::::code{#viet_email_va_sosanhtien}
Tự viết smart constructor `Email` và hàm so sánh `soSanhTien` cho Value
Object `TienVND`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type Email = string & { readonly __brand: "Email" };
function Email(vb: string): Result<Email, string> {
  return vb.includes("@") ? ___ : ___;
}

type TienVND = number & { readonly __brand: "TienVND" };
function soSanhTien(a: TienVND, b: TienVND): boolean {
  return ___;
}

console.log(JSON.stringify(Email("an@example.com")));
console.log(soSanhTien(50000 as TienVND, 50000 as TienVND));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type Email = string & { readonly __brand: "Email" };
function Email(vb: string): Result<Email, string> {
  return vb.includes("@") ? ok(vb as Email) : loi(`"${vb}" thiếu ký tự @`);
}

type TienVND = number & { readonly __brand: "TienVND" };
function soSanhTien(a: TienVND, b: TienVND): boolean {
  return a === b;
}

console.log(JSON.stringify(Email("an@example.com")));
console.log(soSanhTien(50000 as TienVND, 50000 as TienVND));
```

```typescript title=test
const e1 = Email("an@example.com");
if (e1.kind !== "ok") throw new Error("email hợp lệ (có @) phải trả về ok");
if (e1.kind === "ok" && e1.giaTri !== "an@example.com") throw new Error("giá trị Email phải giữ nguyên chuỗi gốc");

const e2 = Email("khong-co-a-cong");
if (e2.kind !== "loi") throw new Error("email thiếu @ phải trả về loi");

if (soSanhTien(50000 as TienVND, 50000 as TienVND) !== true) throw new Error("hai TienVND CÙNG giá trị phải bằng nhau");
if (soSanhTien(50000 as TienVND, 30000 as TienVND) !== false) throw new Error("hai TienVND KHÁC giá trị không được bằng nhau");
```

:::hints
- kind: attention
  body: "Email: nhánh hợp lệ bọc ok(vb as Email); nhánh lỗi bọc loi(thông điệp). soSanhTien: Value Object so sánh bằng GIÁ TRỊ — chỉ cần so sánh a và b trực tiếp."
- kind: strategy
  body: 'ok(vb as Email) : loi(`"${vb}" thiếu ký tự @`) — cho Email. a === b — cho soSanhTien.'
- kind: one-line
  body: "___ (nhánh hợp lệ) = ok(vb as Email)\n___ (nhánh lỗi) = loi(`\"${vb}\" thiếu ký tự @`)\n___ (soSanhTien) = a === b"
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
Entity: so sánh bằng ID, field khác vẫn cùng thực thể. Value Object: so
sánh bằng giá trị, immutable, smart constructor trả `Result<T,E>` —
công cụ đã học, áp dụng vào domain mới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Có những chuyện XẢY RA trong domain — "khách hàng đã đặt đơn", "đơn
hàng đã giao" — không phải Entity, không phải Value Object. Chúng LÀ
gì?
::::

::::checkpoint{mastery=0.8}
::::
