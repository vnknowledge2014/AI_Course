---
id: ky-nghe-phan-mem.ddd.api-contract-va-loi-nhat-quan
title: "API Contract & định dạng lỗi nhất quán"
summary: "Định nghĩa RÕ ba type cho MỖI endpoint như một \"hợp đồng\": YeuCau (client gửi gì), PhanHoi (server trả gì), LoiApi (khi fail trả gì). LoiApi là DU (validation|khong_tim_thay|khong_duoc_phep|noi_bo) map sang HTTP status (422/404/401/500). Mạch validate→process→respond dùng Result xuyên suốt, KHÔNG try/catch."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 35
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.api-contract]
requires: [ddd.inbound-outbound-mapping]
concepts: [ddd.api-contract]
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
Có mapping hai chiều (bài trước). Nhưng một endpoint API cần RÕ HƠN:
client gửi GÌ, server trả GÌ, và khi LỖI thì trả GÌ?
::::

::::explain{#hop-dong-api}
**API Contract**: định nghĩa RÕ RÀNG BA type cho MỖI endpoint, như một
"hợp đồng" — `YeuCauTaoDonHang` (client GỬI gì), `PhanHoiTaoDonHang`
(server TRẢ gì khi THÀNH CÔNG), `LoiApi` (server TRẢ gì khi THẤT
BẠI). `LoiApi` là **discriminated union** — MỖI biến thể ứng với MỘT
mã HTTP status:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

// Hợp đồng của endpoint "tạo đơn hàng"
type YeuCauTaoDonHang = { maKhachHang: string; matHangs: string[] };
type PhanHoiTaoDonHang = { maDonHang: string; tongTien: number };
type LoiApi =
  | { ma: "validation"; thongDiep: string; chiTiet: string[] }
  | { ma: "khong_tim_thay"; thongDiep: string }
  | { ma: "khong_duoc_phep"; thongDiep: string }
  | { ma: "noi_bo"; thongDiep: string };

function maHttpCuaLoi(l: LoiApi): number {
  switch (l.ma) {
    case "validation": return 422;
    case "khong_tim_thay": return 404;
    case "khong_duoc_phep": return 401;
    case "noi_bo": return 500;
  }
}

const loiKhongTimThay: LoiApi = { ma: "khong_tim_thay", thongDiep: "không tìm thấy đơn hàng" };
console.log(maHttpCuaLoi(loiKhongTimThay));
```

```text
404
```

Mỗi biến thể `LoiApi` có `ma` RIÊNG (`"validation"`, `"khong_tim_thay"`,
...) — `maHttpCuaLoi` ÁNH XẠ TRỰC TIẾP sang mã HTTP TIÊU CHUẨN: `422`
(Unprocessable Entity — dữ liệu SAI HÌNH DẠNG), `404` (Not Found),
`401` (Unauthorized), `500` (Internal Server Error). Client GỌI endpoint
LUÔN biết CHÍNH XÁC ba khả năng: `YeuCauTaoDonHang` cần gửi gì,
`PhanHoiTaoDonHang` nhận gì khi thành công, `LoiApi` nhận gì khi lỗi.
::::

::::example{#mach-validate-process-respond}
Handler THẬT đi qua BA bước: **validate → process → respond** — dùng
`Result` **XUYÊN SUỐT**, **KHÔNG một dòng `try/catch` nào**:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type YeuCauTaoDonHang = { maKhachHang: string; matHangs: string[] };
type PhanHoiTaoDonHang = { maDonHang: string; tongTien: number };
type LoiApi = { ma: "validation"; thongDiep: string; chiTiet: string[] } | { ma: "khong_tim_thay"; thongDiep: string } | { ma: "khong_duoc_phep"; thongDiep: string } | { ma: "noi_bo"; thongDiep: string };
function maHttpCuaLoi(l: LoiApi): number {
  switch (l.ma) {
    case "validation": return 422;
    case "khong_tim_thay": return 404;
    case "khong_duoc_phep": return 401;
    case "noi_bo": return 500;
  }
}

// validate: dữ liệu unknown -> YeuCauTaoDonHang, dùng lại kỹ thuật bài 33
function parseYeuCau(input: unknown): Result<YeuCauTaoDonHang, string[]> {
  if (typeof input !== "object" || input === null) return loi(["dữ liệu phải là object"]);
  const obj = input as Record<string, unknown>;
  const loiList: string[] = [];
  if (typeof obj.maKhachHang !== "string" || obj.maKhachHang.trim() === "") loiList.push("maKhachHang phải là chuỗi không rỗng");
  if (!Array.isArray(obj.matHangs) || obj.matHangs.length === 0) loiList.push("matHangs phải là mảng không rỗng");
  if (loiList.length > 0) return loi(loiList);
  return ok({ maKhachHang: obj.maKhachHang as string, matHangs: obj.matHangs as string[] });
}

// handler: validate -> process -> respond, dùng Result xuyên suốt
function xuLyTaoDonHang(yc: unknown): Result<PhanHoiTaoDonHang, LoiApi> {
  const ketQuaParse = parseYeuCau(yc); // validate
  if (ketQuaParse.kind === "loi") {
    return loi({ ma: "validation", thongDiep: "dữ liệu không hợp lệ", chiTiet: ketQuaParse.loi });
  }
  const yeuCau = ketQuaParse.giaTri;
  const tongTien = yeuCau.matHangs.length * 50000; // process
  return ok({ maDonHang: "DH-01", tongTien }); // respond
}

const thanhCong = xuLyTaoDonHang({ maKhachHang: "KH-01", matHangs: ["SP-01", "SP-02"] });
console.log(JSON.stringify(thanhCong));

const thatBai = xuLyTaoDonHang({ maKhachHang: "", matHangs: [] });
console.log(JSON.stringify(thatBai));
if (thatBai.kind === "loi") console.log(maHttpCuaLoi(thatBai.loi));
```

```text title=readonly
{"kind":"ok","giaTri":{"maDonHang":"DH-01","tongTien":100000}}
{"kind":"loi","loi":{"ma":"validation","thongDiep":"dữ liệu không hợp lệ","chiTiet":["maKhachHang phải là chuỗi không rỗng","matHangs phải là mảng không rỗng"]}}
422
```

Lỗi validation MANG THEO `chiTiet` (mảng TỪ `parseYeuCau`'s gom-lỗi,
bài 33) — client nhận ĐỦ thông tin để hiển thị lỗi TỪNG field, KHÔNG
CHỈ "có lỗi" chung chung. `chiTiet` là field OPTIONAL RIÊNG của biến
thể `validation` — `khong_tim_thay`/`khong_duoc_phep`/`noi_bo` KHÔNG
có `chiTiet` (không CẦN chi tiết cho "không tìm thấy" hay "lỗi nội
bộ").
::::

::::predict{#doan-loi-noi-bo-ma-http commitOnce}
```typescript
type LoiApi =
  | { ma: "validation"; thongDiep: string; chiTiet: string[] }
  | { ma: "khong_tim_thay"; thongDiep: string }
  | { ma: "khong_duoc_phep"; thongDiep: string }
  | { ma: "noi_bo"; thongDiep: string };

function maHttpCuaLoi(l: LoiApi): number {
  switch (l.ma) {
    case "validation": return 422;
    case "khong_tim_thay": return 404;
    case "khong_duoc_phep": return 401;
    case "noi_bo": return 500;
  }
}

const loiDatabase: LoiApi = { ma: "noi_bo", thongDiep: "kết nối database thất bại" };
console.log(maHttpCuaLoi(loiDatabase));
```

Dòng cuối in ra gì?

:::opt{correct}
`500`
:::

:::opt
`401` — vì "kết nối database thất bại" LÀ một vấn đề QUYỀN TRUY CẬP
(database từ chối kết nối do sai thông tin xác thực), nên nó KHỚP
`khong_duoc_phep`, không phải `noi_bo`
::why
Gần đúng ở việc bạn nghĩ tới MỘT nguyên nhân CÓ THỂ khiến kết nối
database thất bại (sai thông tin xác thực) — một khả năng NGHIỆP VỤ
hợp lý trong thế giới THẬT.

Chỗ lệch: `maHttpCuaLoi` KHÔNG đọc `thongDiep` (chuỗi MÔ TẢ, dành cho
CON NGƯỜI đọc) để "đoán" mã HTTP — nó CHỈ đọc `l.ma`, field DISCRIMINANT
CHÍNH THỨC của union. `loiDatabase.ma` được khai TƯỜNG MINH là
`"noi_bo"` (do người viết code CHỌN gán, KHÔNG phải suy luận từ nội
dung `thongDiep`) — `switch (l.ma)` khớp `case "noi_bo": return 500`
NGAY, bất kể `thongDiep` viết gì.
::
:::

:::opt
Máy báo lỗi biên dịch — `switch (l.ma)` thiếu `default` case, TypeScript
không cho phép `switch` trên union thiếu nhánh dự phòng
::why
Gần đúng ở việc bạn để ý `switch (l.ma)` KHÔNG có `default` — một
QUAN SÁT ĐÚNG về cấu trúc code (đúng LÀ không có `default` nào ở đây).

Chỗ lệch: KHÔNG CẦN `default` khi `switch` xử lý **HẾT** mọi biến thể
CÓ THỂ của MỘT discriminated union — TypeScript TỰ CHỨNG MINH được
BỐN case (`"validation"`, `"khong_tim_thay"`, `"khong_duoc_phep"`,
`"noi_bo"`) đã PHỦ HẾT bốn biến thể của `LoiApi["ma"]`, không còn giá
trị nào khác CÓ THỂ xảy ra — biên dịch sạch, giống hệt cách `Result`'s
`switch(r.kind)` (bài 22-23) chỉ cần `"ok"`/`"loi"`.
::
:::
::::

::::code{#viet_mahttpcualoi}
Tự viết BỐN nhánh ánh xạ trong `maHttpCuaLoi`.

```typescript title=starter
type LoiApi =
  | { ma: "validation"; thongDiep: string; chiTiet: string[] }
  | { ma: "khong_tim_thay"; thongDiep: string }
  | { ma: "khong_duoc_phep"; thongDiep: string }
  | { ma: "noi_bo"; thongDiep: string };

function maHttpCuaLoi(l: LoiApi): number {
  switch (l.ma) {
    case "validation": return ___;
    case "khong_tim_thay": return ___;
    case "khong_duoc_phep": return ___;
    case "noi_bo": return ___;
  }
}

console.log(maHttpCuaLoi({ ma: "validation", thongDiep: "sai", chiTiet: [] }));
```

```typescript title=solution
type LoiApi =
  | { ma: "validation"; thongDiep: string; chiTiet: string[] }
  | { ma: "khong_tim_thay"; thongDiep: string }
  | { ma: "khong_duoc_phep"; thongDiep: string }
  | { ma: "noi_bo"; thongDiep: string };

function maHttpCuaLoi(l: LoiApi): number {
  switch (l.ma) {
    case "validation": return 422;
    case "khong_tim_thay": return 404;
    case "khong_duoc_phep": return 401;
    case "noi_bo": return 500;
  }
}

console.log(maHttpCuaLoi({ ma: "validation", thongDiep: "sai", chiTiet: [] }));
```

```typescript title=test
if (maHttpCuaLoi({ ma: "validation", thongDiep: "x", chiTiet: [] }) !== 422) throw new Error("validation phải ánh xạ sang 422");
if (maHttpCuaLoi({ ma: "khong_tim_thay", thongDiep: "x" }) !== 404) throw new Error("khong_tim_thay phải ánh xạ sang 404");
if (maHttpCuaLoi({ ma: "khong_duoc_phep", thongDiep: "x" }) !== 401) throw new Error("khong_duoc_phep phải ánh xạ sang 401");
if (maHttpCuaLoi({ ma: "noi_bo", thongDiep: "x" }) !== 500) throw new Error("noi_bo phải ánh xạ sang 500");
```

:::hints
- kind: attention
  body: "Mỗi case ánh xạ sang MỘT mã HTTP status TIÊU CHUẨN khác nhau — validation=422 (Unprocessable Entity), khong_tim_thay=404 (Not Found), khong_duoc_phep=401 (Unauthorized), noi_bo=500 (Internal Server Error)."
- kind: strategy
  body: "422 : 404 : 401 : 500 — bốn mã HTTP status CHUẨN, theo đúng thứ tự bốn case liệt kê."
- kind: one-line
  body: '___ ("validation") = 422\n___ ("khong_tim_thay") = 404\n___ ("khong_duoc_phep") = 401\n___ ("noi_bo") = 500'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "422"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
API Contract: ba type RÕ RÀNG cho mỗi endpoint, lỗi theo CÙNG một cấu
trúc DU, ánh xạ HTTP status NHẤT QUÁN. Bước tiếp theo: hợp đồng CẦN
đổi theo thời gian — làm sao KHÔNG phá vỡ client CŨ?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`YeuCauTaoDonHang` cần thêm MỘT field mới (ví dụ `ghiChu`). Client CŨ
(chưa cập nhật) vẫn gửi request KHÔNG có field đó — làm sao endpoint
vẫn hoạt động cho CẢ HAI, client cũ VÀ client mới?
::::

::::checkpoint{mastery=0.8}
::::
