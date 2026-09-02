---
id: ky-nghe-phan-mem.ddd.dto-doi-tuong-du-lieu-thuan
title: "DTO — đối tượng dữ liệu thuần, JSON-serializable"
summary: "DTO (Data Transfer Object) = \"bưu kiện\" chỉ chứa data, KHÔNG method, KHÔNG branded type, KHÔNG business logic. Quy tắc chuyển đổi: brand→primitive, Date→chuỗi ISO, union literal→string. JSON-serializability là LÝ DO DTO tồn tại — domain object (Date, brand) không có đảm bảo đó qua JSON.stringify/parse."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 32
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ddd.dto]
requires: [ddd.layer-type-split]
concepts: [ddd.dto]
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
`PhanHoiApiNguoiDung` (bài trước) chỉ chứa data, không method — khái
niệm đó có TÊN CHÍNH THỨC trong kỹ nghệ phần mềm.
::::

::::explain{#dto-va-json}
**DTO** (Data Transfer Object) = "bưu kiện" chỉ chứa DATA THUẦN —
KHÔNG method, KHÔNG branded type, KHÔNG business logic. Lý do DTO tồn
tại: `JSON.stringify`/`JSON.parse` (cách dữ liệu THẬT SỰ đi qua mạng)
KHÔNG bảo toàn `Date` — gửi domain object THẲNG là NGUY HIỂM:

```typescript
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type TienVND = number & { readonly __brand: "TienVND" };
type TrangThaiDonHang = "nhap" | "cho_thanh_toan" | "da_thanh_toan";

type DonHangMien = {
  ma: MaDonHang;
  tongTien: TienVND;
  trangThai: TrangThaiDonHang;
  ngayTao: Date;
};

const donHang: DonHangMien = {
  ma: "DH-01" as MaDonHang,
  tongTien: 100000 as TienVND,
  trangThai: "nhap",
  ngayTao: new Date("2026-01-01T00:00:00.000Z"),
};

const vanBan = JSON.stringify(donHang);
console.log(vanBan);

const doiLai = JSON.parse(vanBan);
console.log(typeof doiLai.ngayTao);
console.log(doiLai.ngayTao instanceof Date);
```

```text
{"ma":"DH-01","tongTien":100000,"trangThai":"nhap","ngayTao":"2026-01-01T00:00:00.000Z"}
string
false
```

`JSON.stringify` ÂM THẦM biến `Date` thành CHUỖI (gọi `.toJSON()` nội
bộ) — `JSON.parse` lại KHÔNG "nhớ" nó TỪNG là `Date`, trả về CHUỖI
THƯỜNG. `doiLai.ngayTao instanceof Date` là `false` — TypeScript
KHÔNG cảnh báo GÌ, vì `JSON.parse` trả kiểu `any`, "hứa" bất cứ gì
compiler muốn tin. Nếu code SAU ĐÓ gọi `doiLai.ngayTao.getFullYear()`
(tưởng vẫn là `Date`) → **crash lúc chạy**, không phải lỗi biên dịch.
::::

::::example{#dto-tuong-minh}
Giải pháp: khai MỘT type DTO TƯỜNG MINH — MỌI field đã là kiểu
JSON-AN TOÀN (`string`/`number`/`boolean`, KHÔNG `Date`, KHÔNG brand)
— rồi MAP domain sang DTO bằng một hàm THUẦN:

```typescript title=readonly
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type TienVND = number & { readonly __brand: "TienVND" };
type TrangThaiDonHang = "nhap" | "cho_thanh_toan" | "da_thanh_toan";
type DonHangMien = { ma: MaDonHang; tongTien: TienVND; trangThai: TrangThaiDonHang; ngayTao: Date };

// DTO: MỌI field đã là primitive JSON-an toàn
type DonHangDto = {
  ma: string;
  tongTien: number;
  trangThai: string;
  ngayTao: string; // ISO, tường minh -- không còn Date
};

function mienSangDto(dh: DonHangMien): DonHangDto {
  return { ma: dh.ma, tongTien: dh.tongTien, trangThai: dh.trangThai, ngayTao: dh.ngayTao.toISOString() };
}

const donHang: DonHangMien = { ma: "DH-01" as MaDonHang, tongTien: 100000 as TienVND, trangThai: "nhap", ngayTao: new Date("2026-01-01T00:00:00.000Z") };
const dto = mienSangDto(donHang);

const vanBan = JSON.stringify(dto);
const dtoDoiLai = JSON.parse(vanBan) as DonHangDto;
console.log(dtoDoiLai.ngayTao === dto.ngayTao);
console.log(JSON.stringify(dtoDoiLai));
```

```text title=readonly
true
{"ma":"DH-01","tongTien":100000,"trangThai":"nhap","ngayTao":"2026-01-01T00:00:00.000Z"}
```

`dtoDoiLai.ngayTao === dto.ngayTao` là `true` — vì `ngayTao` TRONG DTO
LUÔN LÀ CHUỖI (KHÔNG BAO GIỜ là `Date` từ đầu), qua vòng
`stringify`/`parse` KHÔNG có GÌ để "mất kiểu" — chuỗi vào, chuỗi ra,
ĐÚNG y hệt. Đây LÀ lý do DTO tồn tại: **JSON-serializability** — round-
trip qua JSON PHẢI giữ nguyên field, và DTO đạt được điều đó BẰNG
CÁCH thiết kế (mọi field VỐN DĨ đã là kiểu JSON hiểu được), không phải
may mắn.

**Quy tắc chuyển Domain → DTO**: brand (`MaDonHang`, `TienVND`) →
primitive (`string`, `number`); `Date` → chuỗi ISO; union literal
(`"nhap" | "cho_thanh_toan" | ...`) → `string` thường (không giữ
literal union); method/business logic → **KHÔNG có** trong DTO.
::::

::::predict{#doan-gan-thang-domain-vao-dto commitOnce}
```typescript
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type TienVND = number & { readonly __brand: "TienVND" };
type TrangThaiDonHang = "nhap" | "cho_thanh_toan" | "da_thanh_toan";
type DonHangMien = { ma: MaDonHang; tongTien: TienVND; trangThai: TrangThaiDonHang; ngayTao: Date };
type DonHangDto = { ma: string; tongTien: number; trangThai: string; ngayTao: string };

const donHang: DonHangMien = {
  ma: "DH-01" as MaDonHang,
  tongTien: 100000 as TienVND,
  trangThai: "nhap",
  ngayTao: new Date("2026-01-01T00:00:00.000Z"),
};

// Thử gán THẲNG domain object vào biến khai kiểu DTO -- KHÔNG qua mienSangDto
const dtoTat: DonHangDto = donHang;
console.log(dtoTat);
```

Chuyện gì xảy ra?

:::opt{correct}
Máy báo lỗi biên dịch
:::

:::opt
Chạy được BÌNH THƯỜNG — vì `ma` (brand→string) và `tongTien`
(brand→number) đều TỰ ĐỘNG gán được (đã học ở bài trước), `trangThai`
(union literal→string) cũng tương thích, nên TOÀN BỘ object gán được
::why
Gần đúng ở việc bạn nhớ ĐÚNG BA field (`ma`, `tongTien`, `trangThai`)
CÓ tương thích tự động — brand LÀ intersection với kiểu gốc nên gán
được sang kiểu gốc, union literal cụ thể LÀ một `string` nên gán được
sang `string` — CẢ BA quan sát đó đúng.

Chỗ lệch: field THỨ TƯ, `ngayTao`, KHÔNG tương thích — `DonHangMien`
khai `ngayTao: Date`, `DonHangDto` khai `ngayTao: string`. `Date` và
`string` là HAI kiểu HOÀN TOÀN khác nhau về cấu trúc (không phải một
là "phần thu hẹp" của kiểu kia, khác hẳn quan hệ brand↔primitive) —
TypeScript từ chối: `Type 'DonHangMien' is not assignable to type
'DonHangDto'. Types of property 'ngayTao' are incompatible. Type
'Date' is not assignable to type 'string'.` Đây CHÍNH LÀ lý do CẦN một
hàm mapping (`mienSangDto`) TƯỜNG MINH gọi `.toISOString()`, thay vì
gán thẳng.
::
:::
::::

::::code{#viet_mien_sang_dto}
Tự viết phần `ma` và `ngayTao` trong `mienSangDto`.

```typescript title=starter
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type TienVND = number & { readonly __brand: "TienVND" };
type TrangThaiDonHang = "nhap" | "cho_thanh_toan" | "da_thanh_toan";
type DonHangMien = { ma: MaDonHang; tongTien: TienVND; trangThai: TrangThaiDonHang; ngayTao: Date };
type DonHangDto = { ma: string; tongTien: number; trangThai: string; ngayTao: string };

function mienSangDto(dh: DonHangMien): DonHangDto {
  return {
    ma: ___,
    tongTien: dh.tongTien,
    trangThai: dh.trangThai,
    ngayTao: ___,
  };
}

const donHang: DonHangMien = { ma: "DH-01" as MaDonHang, tongTien: 100000 as TienVND, trangThai: "nhap", ngayTao: new Date("2026-01-01T00:00:00.000Z") };
console.log(JSON.stringify(mienSangDto(donHang)));
```

```typescript title=solution
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type TienVND = number & { readonly __brand: "TienVND" };
type TrangThaiDonHang = "nhap" | "cho_thanh_toan" | "da_thanh_toan";
type DonHangMien = { ma: MaDonHang; tongTien: TienVND; trangThai: TrangThaiDonHang; ngayTao: Date };
type DonHangDto = { ma: string; tongTien: number; trangThai: string; ngayTao: string };

function mienSangDto(dh: DonHangMien): DonHangDto {
  return {
    ma: dh.ma,
    tongTien: dh.tongTien,
    trangThai: dh.trangThai,
    ngayTao: dh.ngayTao.toISOString(),
  };
}

const donHang: DonHangMien = { ma: "DH-01" as MaDonHang, tongTien: 100000 as TienVND, trangThai: "nhap", ngayTao: new Date("2026-01-01T00:00:00.000Z") };
console.log(JSON.stringify(mienSangDto(donHang)));
```

```typescript title=test
const donHangTest: DonHangMien = { ma: "DH-77" as MaDonHang, tongTien: 250000 as TienVND, trangThai: "cho_thanh_toan", ngayTao: new Date("2026-03-15T08:30:00.000Z") };
const dtoTest = mienSangDto(donHangTest);
if (dtoTest.ma !== "DH-77") throw new Error("mienSangDto phải giữ đúng giá trị ma");
if (dtoTest.ngayTao !== "2026-03-15T08:30:00.000Z") throw new Error("ngayTao phải là chuỗi ISO đúng, gọi qua toISOString()");
if (typeof dtoTest.ngayTao !== "string") throw new Error("ngayTao trong DTO phải là string, không phải Date");

// Round-trip qua JSON phải giữ nguyên
const vanBanTest = JSON.stringify(dtoTest);
const dtoDoiLaiTest = JSON.parse(vanBanTest) as DonHangDto;
if (dtoDoiLaiTest.ngayTao !== dtoTest.ngayTao) throw new Error("DTO phải round-trip qua JSON mà không đổi giá trị ngayTao");
```

:::hints
- kind: attention
  body: "ma: gán trực tiếp dh.ma (branded string tự động tương thích với string thường). ngayTao: dh.ngayTao là Date, gọi .toISOString() để có chuỗi ISO — KHÔNG gán thẳng dh.ngayTao (kiểu Date, không khớp string)."
- kind: strategy
  body: "dh.ma : dh.ngayTao.toISOString() — brand gán thẳng được, Date phải GỌI METHOD để chuyển thành chuỗi."
- kind: one-line
  body: "___ (ma) = dh.ma\n___ (ngayTao) = dh.ngayTao.toISOString()"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "DH-01"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
DTO: bưu kiện dữ liệu thuần, JSON-serializable bằng THIẾT KẾ. Bước
tiếp theo: dữ liệu từ bên NGOÀI (API request) không đáng tin — kiểu
TypeScript không bảo vệ được lúc chạy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`JSON.parse(vanBan)` trả về kiểu `any` — TypeScript "tin" bất cứ gì
bạn gán cho nó, dù dữ liệu THẬT có đúng hình dạng hay không. Làm sao
kiểm tra dữ liệu từ bên ngoài THẬT SỰ đúng hình dạng LÚC CHƯƠNG TRÌNH
CHẠY?
::::

::::checkpoint{mastery=0.8}
::::
