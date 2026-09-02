---
id: ky-nghe-phan-mem.ddd.tach-type-theo-layer-va-mapping
title: "Vấn đề \"một type cho mọi layer\" — tách Domain/DB/API + mapping"
summary: "Dùng CHUNG một type vừa làm DB row, vừa API response, vừa domain object → rò rỉ dữ liệu nhạy cảm. Giải pháp: tách BA type độc lập (Mien/Db/Api) + mapping function thuần nối chúng. Layer isolation: đổi DB schema chỉ sửa một hàm mapping, domain KHÔNG đổi."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 31
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.layer-type-split]
requires: [ddd.form-validation-accumulate]
concepts: [ddd.layer-type-split]
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
Cụm mới. Dữ liệu "chảy" qua BA nơi khác nhau: database, domain logic,
API trả về client. Dùng CHUNG một type cho cả ba — tiện, nhưng NGUY?
::::

::::explain{#mot-type-rui-ro}
Dùng CHUNG một type `NguoiDungMotType` (vừa DB row, vừa domain, vừa
API response) — hàm trả API "tiện" chỉ cần trả NGUYÊN object, KHÔNG
lọc field nào:

```typescript
type NguoiDungMotType = {
  id: string;
  ten: string;
  email: string;
  da_kich_hoat: number;
  mat_khau_ma_hoa: string;
};

function guiPhanHoiApi(nd: NguoiDungMotType): NguoiDungMotType {
  return nd;
}

const nguoiDungTuDb: NguoiDungMotType = {
  id: "u1", ten: "An", email: "an@shop.vn",
  da_kich_hoat: 1, mat_khau_ma_hoa: "$2b$10$abc...",
};
console.log(JSON.stringify(guiPhanHoiApi(nguoiDungTuDb)));
```

```text
{"id":"u1","ten":"An","email":"an@shop.vn","da_kich_hoat":1,"mat_khau_ma_hoa":"$2b$10$abc..."}
```

`mat_khau_ma_hoa` (mật khẩu ĐÃ MÃ HOÁ, nhưng VẪN là dữ liệu NHẠY CẢM,
KHÔNG BAO GIỜ nên rời khỏi server) LỌT thẳng vào phản hồi API — không
một dòng code nào SAI cú pháp, TypeScript KHÔNG cảnh báo GÌ, vì kiểu
`NguoiDungMotType` VỐN DĨ đã trộn LẪN mọi field của cả ba layer.
`da_kich_hoat: number` (`0`/`1`, quy ước LƯU TRỮ của DB) cũng rò vào
domain logic, dù domain thường muốn `boolean`.
::::

::::example{#tach-ba-type}
Giải pháp: tách **BA type ĐỘC LẬP** — mỗi type CHỈ chứa field ĐÚNG với
layer của NÓ — nối bằng **mapping function THUẦN**:

```typescript title=readonly
type MaNguoiDung = string & { readonly __brand: "MaNguoiDung" };

// Domain: kiểu "đúng ý nghĩa" -- Date thật, boolean thật, mã có brand
type NguoiDungMien = {
  ma: MaNguoiDung;
  ten: string;
  email: string;
  ngayTao: Date;
  daKichHoat: boolean;
};

// DB: snake_case, số thô -- khớp CỘT thật trong bảng
type HangDbNguoiDung = {
  id: string;
  ten: string;
  email: string;
  ngay_tao: string;
  da_kich_hoat: number;
  mat_khau_ma_hoa: string;
};

// API: CHỈ field được PHÉP client thấy -- KHÔNG có mat_khau_ma_hoa
type PhanHoiApiNguoiDung = {
  id: string;
  ten: string;
  email: string;
};

function dbSangMien(hang: HangDbNguoiDung): NguoiDungMien {
  return {
    ma: hang.id as MaNguoiDung,
    ten: hang.ten,
    email: hang.email,
    ngayTao: new Date(hang.ngay_tao),
    daKichHoat: hang.da_kich_hoat === 1,
  };
}

function mienSangApi(nd: NguoiDungMien): PhanHoiApiNguoiDung {
  return { id: nd.ma, ten: nd.ten, email: nd.email };
}

const hangDb: HangDbNguoiDung = {
  id: "u1", ten: "An", email: "an@shop.vn",
  ngay_tao: "2026-01-01T00:00:00.000Z", da_kich_hoat: 1,
  mat_khau_ma_hoa: "$2b$10$abc...",
};
const mien = dbSangMien(hangDb);
console.log(mien.daKichHoat, mien.ngayTao instanceof Date);
console.log(JSON.stringify(mienSangApi(mien)));
```

```text title=readonly
true true
{"id":"u1","ten":"An","email":"an@shop.vn"}
```

`mienSangApi` KHÔNG THỂ rò `mat_khau_ma_hoa` — `NguoiDungMien` (kiểu
tham số CỦA nó) **KHÔNG CÓ** field đó, `dbSangMien` đã "bỏ lại" nó ở
BIÊN giữa DB và domain rồi. Đây LÀ **layer isolation**: DB schema đổi
tên cột → CHỈ sửa `dbSangMien`; API cần thêm/bớt field trả về → CHỈ
sửa `mienSangApi`; logic domain (dùng `NguoiDungMien`) KHÔNG đổi GÌ.
::::

::::predict{#doan-khong-the-doc-truong-nhay-cam commitOnce}
```typescript
type MaNguoiDung = string & { readonly __brand: "MaNguoiDung" };
type NguoiDungMien = { ma: MaNguoiDung; ten: string; email: string; ngayTao: Date; daKichHoat: boolean };
type HangDbNguoiDung = { id: string; ten: string; email: string; ngay_tao: string; da_kich_hoat: number; mat_khau_ma_hoa: string };

function dbSangMien(hang: HangDbNguoiDung): NguoiDungMien {
  return { ma: hang.id as MaNguoiDung, ten: hang.ten, email: hang.email, ngayTao: new Date(hang.ngay_tao), daKichHoat: hang.da_kich_hoat === 1 };
}

const hangDb: HangDbNguoiDung = {
  id: "u1", ten: "An", email: "an@shop.vn",
  ngay_tao: "2026-01-01T00:00:00.000Z", da_kich_hoat: 1,
  mat_khau_ma_hoa: "$2b$10$abc...",
};
const mien = dbSangMien(hangDb);
console.log(mien.mat_khau_ma_hoa);
```

Dòng cuối in ra gì?

:::opt{correct}
Máy báo lỗi biên dịch
:::

:::opt
`undefined` — vì `mien` được TẠO từ `hangDb` (VỐN CÓ `mat_khau_ma_hoa`),
nhưng `dbSangMien` không SAO CHÉP field đó sang, nên `mien.mat_khau_ma_hoa`
đơn giản không tồn tại LÚC CHẠY, JavaScript trả `undefined` cho field
thiếu
::why
Gần đúng ở việc bạn nhớ ĐÚNG `dbSangMien` KHÔNG sao chép
`mat_khau_ma_hoa` sang `mien` — quan sát về việc field ĐÓ bị "bỏ lại"
đúng, và trong JavaScript THUẦN (không kiểm kiểu), đọc field thiếu QUẢ
THẬT ra `undefined`.

Chỗ lệch: đây là DỰ ÁN TypeScript — code phải BIÊN DỊCH được TRƯỚC KHI
chạy. `mien` có kiểu KHAI RÕ là `NguoiDungMien`, một type KHÔNG hề khai
field `mat_khau_ma_hoa`. TypeScript kiểm THEO KIỂU KHAI, không theo
"những gì OBJECT THỰC SỰ có lúc chạy" — `mien.mat_khau_ma_hoa` bị từ
chối NGAY lúc biên dịch (`TS2339: Property 'mat_khau_ma_hoa' does not
exist on type 'NguoiDungMien'`), code KHÔNG BAO GIỜ chạy tới dòng đó.
::
:::

:::opt
`"$2b$10$abc..."` — vì `dbSangMien` nhận NGUYÊN `hang` làm tham số,
và trong JavaScript, một object CÓ THỂ mang thêm field "dư" so với
kiểu khai — `mien` thực chất VẪN giữ toàn bộ field gốc của `hangDb`
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng đối tượng JavaScript LÚC CHẠY có
thể "dư" field so với KIỂU KHAI (TypeScript's structural typing đôi
khi cho phép NHẬN một object thừa field) — một quan sát THẬT về ngôn
ngữ.

Chỗ lệch: ở đây `dbSangMien` KHÔNG trả VỀ `hang` (tham số) — nó XÂY
MỘT OBJECT LITERAL MỚI (`return { ma: ..., ten: ..., email: ...,
ngayTao: ..., daKichHoat: ... }`), CHỈ liệt kê NĂM field ĐÚNG BẰNG
`NguoiDungMien`. `mat_khau_ma_hoa` KHÔNG được sao chép vào object MỚI
này ở BẤT KỲ đâu — `mien` THẬT SỰ (cả lúc chạy) không mang field đó,
không phải chỉ "ẩn" khỏi kiểu.
::
:::
::::

::::code{#viet_dbsangmien_mausangapi}
Tự viết phần chuyển đổi trong `dbSangMien` và `mienSangApi`.

```typescript title=starter
type MaNguoiDung = string & { readonly __brand: "MaNguoiDung" };
type NguoiDungMien = { ma: MaNguoiDung; ten: string; email: string; ngayTao: Date; daKichHoat: boolean };
type HangDbNguoiDung = { id: string; ten: string; email: string; ngay_tao: string; da_kich_hoat: number; mat_khau_ma_hoa: string };
type PhanHoiApiNguoiDung = { id: string; ten: string; email: string };

function dbSangMien(hang: HangDbNguoiDung): NguoiDungMien {
  return {
    ma: hang.id as MaNguoiDung,
    ten: hang.ten,
    email: hang.email,
    ngayTao: new Date(hang.ngay_tao),
    daKichHoat: ___,
  };
}

function mienSangApi(nd: NguoiDungMien): PhanHoiApiNguoiDung {
  return {
    id: ___,
    ten: nd.ten,
    email: nd.email,
  };
}

const hangDb: HangDbNguoiDung = { id: "u1", ten: "An", email: "an@shop.vn", ngay_tao: "2026-01-01T00:00:00.000Z", da_kich_hoat: 1, mat_khau_ma_hoa: "bimat" };
console.log(JSON.stringify(mienSangApi(dbSangMien(hangDb))));
```

```typescript title=solution
type MaNguoiDung = string & { readonly __brand: "MaNguoiDung" };
type NguoiDungMien = { ma: MaNguoiDung; ten: string; email: string; ngayTao: Date; daKichHoat: boolean };
type HangDbNguoiDung = { id: string; ten: string; email: string; ngay_tao: string; da_kich_hoat: number; mat_khau_ma_hoa: string };
type PhanHoiApiNguoiDung = { id: string; ten: string; email: string };

function dbSangMien(hang: HangDbNguoiDung): NguoiDungMien {
  return {
    ma: hang.id as MaNguoiDung,
    ten: hang.ten,
    email: hang.email,
    ngayTao: new Date(hang.ngay_tao),
    daKichHoat: hang.da_kich_hoat === 1,
  };
}

function mienSangApi(nd: NguoiDungMien): PhanHoiApiNguoiDung {
  return {
    id: nd.ma,
    ten: nd.ten,
    email: nd.email,
  };
}

const hangDb: HangDbNguoiDung = { id: "u1", ten: "An", email: "an@shop.vn", ngay_tao: "2026-01-01T00:00:00.000Z", da_kich_hoat: 1, mat_khau_ma_hoa: "bimat" };
console.log(JSON.stringify(mienSangApi(dbSangMien(hangDb))));
```

```typescript title=test
const kichHoat: HangDbNguoiDung = { id: "u1", ten: "An", email: "an@shop.vn", ngay_tao: "2026-01-01T00:00:00.000Z", da_kich_hoat: 1, mat_khau_ma_hoa: "x" };
if (dbSangMien(kichHoat).daKichHoat !== true) throw new Error("da_kich_hoat = 1 phải chuyển thành daKichHoat = true");

const chuaKichHoat: HangDbNguoiDung = { id: "u2", ten: "Binh", email: "binh@shop.vn", ngay_tao: "2026-01-01T00:00:00.000Z", da_kich_hoat: 0, mat_khau_ma_hoa: "y" };
if (dbSangMien(chuaKichHoat).daKichHoat !== false) throw new Error("da_kich_hoat = 0 phải chuyển thành daKichHoat = false");

const mien = dbSangMien(kichHoat);
const api = mienSangApi(mien);
if (api.id !== "u1") throw new Error("mienSangApi phải giữ đúng id");
if (Object.keys(api).length !== 3) throw new Error("PhanHoiApiNguoiDung chỉ được có ĐÚNG BA field (id, ten, email) — không thừa field nào");
if ("mat_khau_ma_hoa" in api) throw new Error("mat_khau_ma_hoa KHÔNG được lọt vào phản hồi API");
```

:::hints
- kind: attention
  body: "dbSangMien.daKichHoat: chuyển số 0/1 thành boolean thật bằng so sánh (không phải Boolean(n), vì Boolean(0)=false nhưng Boolean(BẤT KỲ số khác 0)=true, không đúng ý nghĩa \"đúng bằng 1\"). mienSangApi.id: mã miền (branded) tự động là string, gán thẳng được."
- kind: strategy
  body: "hang.da_kich_hoat === 1 : nd.ma — so sánh trực tiếp với 1 cho ra boolean chính xác; nd.ma (MaNguoiDung, string & brand) gán thẳng vào field string được vì brand chỉ chặn CHIỀU NGƯỢC LẠI."
- kind: one-line
  body: "___ (daKichHoat) = hang.da_kich_hoat === 1\n___ (id) = nd.ma"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "u1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tách type theo layer: rò rỉ dữ liệu nhạy cảm trở thành LỖI BIÊN DỊCH,
không phải lỗi phải NHỚ tự tránh. Bước tiếp theo: đặt tên chính thức
cho khái niệm "bưu kiện dữ liệu thuần" vừa xây.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`PhanHoiApiNguoiDung`/`HangDbNguoiDung` — những type CHỈ chứa dữ liệu,
KHÔNG method, KHÔNG business logic — có một cái tên RIÊNG trong kỹ
nghệ phần mềm. Tên đó là gì?
::::

::::checkpoint{mastery=0.8}
::::
