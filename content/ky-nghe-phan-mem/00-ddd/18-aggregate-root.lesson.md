---
id: ky-nghe-phan-mem.ddd.aggregate-root
title: "Aggregate Root — tổng hợp VO, state machine, validate MỌI bước"
summary: "Aggregate Root gộp VO (field con) + DU state machine + smart-constructor validation Ở MỌI bước transition thành MỘT entity chịu trách nhiệm toàn vẹn dữ liệu. DonHang (5 state), business rule KHÁC NHAU tuỳ state khi huỷ."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 18
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.aggregate-root]
requires: [ddd.domain-hierarchy]
concepts: [ddd.aggregate-root]
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
Bài chốt cụm 3. VO (bài 13), DU state machine (bài 15), domain
hierarchy (bài 17) — ghép TẤT CẢ vào MỘT entity chịu trách nhiệm TOÀN
VẸN dữ liệu.
::::

::::explain{#aggregate-root}
**Aggregate Root**: gộp VO (cho field con) + DU state machine + smart-
constructor validation Ở MỌI BƯỚC transition thành MỘT entity:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type MatHang = { ten: string; soLuong: number; donGia: number };

type DonHang =
  | { tag: "nhap"; matHang: MatHang[] }
  | { tag: "da_xac_nhan"; matHang: MatHang[]; tongTien: number }
  | { tag: "da_giao"; matHang: MatHang[]; tongTien: number; maVanDon: string }
  | { tag: "da_huy"; hoanTien: number; lyDo: string };

// Smart constructor: KHÔNG cho tạo đơn hàng RỖNG
function taoNhap(matHang: MatHang[]): Result<DonHang, string> {
  if (matHang.length === 0) return loi("đơn hàng phải có ít nhất một mặt hàng");
  return ok({ tag: "nhap", matHang });
}

function xacNhan(dh: DonHang & { tag: "nhap" }): DonHang & { tag: "da_xac_nhan" } {
  const tongTien = dh.matHang.reduce((tong, mh) => tong + mh.soLuong * mh.donGia, 0);
  return { tag: "da_xac_nhan", matHang: dh.matHang, tongTien };
}

const kqTao = taoNhap([{ ten: "Ly", soLuong: 2, donGia: 50000 }]);
console.log(JSON.stringify(kqTao));
```

```text
{"kind":"ok","giaTri":{"tag":"nhap","matHang":[{"ten":"Ly","soLuong":2,"donGia":50000}]}}
```

`taoNhap` (smart constructor) VALIDATE ngay LÚC TẠO (không đơn hàng
RỖNG). `xacNhan` (transition function, bài 15) tự TÍNH `tongTien` từ
`matHang` — KHÔNG để caller tự tính rồi truyền vào (tránh sai lệch).
::::

::::example{#business-rule-khac-nhau-tuy-state}
Điểm QUAN TRỌNG NHẤT của Aggregate Root: business rule CÓ THỂ khác
nhau TUỲ trạng thái — huỷ đơn `"nhap"` (chưa xác nhận) hoàn TIỀN
KHÁC với huỷ đơn `"da_xac_nhan"` (đã tính tiền):

```typescript title=readonly
type MatHang = { ten: string; soLuong: number; donGia: number };
type DonHang =
  | { tag: "nhap"; matHang: MatHang[] }
  | { tag: "da_xac_nhan"; matHang: MatHang[]; tongTien: number }
  | { tag: "da_giao"; matHang: MatHang[]; tongTien: number; maVanDon: string }
  | { tag: "da_huy"; hoanTien: number; lyDo: string };

function huyNhap(dh: DonHang & { tag: "nhap" }, lyDo: string): DonHang & { tag: "da_huy" } {
  return { tag: "da_huy", hoanTien: 0, lyDo }; // CHƯA thanh toán -- KHÔNG có gì để hoàn
}

function huyDaXacNhan(dh: DonHang & { tag: "da_xac_nhan" }, lyDo: string): DonHang & { tag: "da_huy" } {
  return { tag: "da_huy", hoanTien: dh.tongTien, lyDo }; // ĐÃ có tongTien -- hoàn TOÀN BỘ
}

const dhNhap: DonHang & { tag: "nhap" } = { tag: "nhap", matHang: [{ ten: "Ly", soLuong: 2, donGia: 50000 }] };
const dhXacNhan: DonHang & { tag: "da_xac_nhan" } = { tag: "da_xac_nhan", matHang: dhNhap.matHang, tongTien: 100000 };

console.log(JSON.stringify(huyNhap(dhNhap, "hết hàng")));
console.log(JSON.stringify(huyDaXacNhan(dhXacNhan, "khách đổi ý")));
```

```text title=readonly
{"tag":"da_huy","hoanTien":0,"lyDo":"hết hàng"}
{"tag":"da_huy","hoanTien":100000,"lyDo":"khách đổi ý"}
```

CÙNG "huỷ đơn hàng" — NHƯNG `hoanTien` KHÁC HẲN tuỳ trạng thái LÚC
huỷ: `"nhap"` chưa hề có tiền để hoàn (`0`); `"da_xac_nhan"` đã có
`tongTien` xác định, hoàn TOÀN BỘ. Đây LÀ lý do Aggregate Root cần
CẢ HAI hàm huỷ RIÊNG BIỆT (`huyNhap`, `huyDaXacNhan`) — KHÔNG gộp
thành một hàm chung nhận `DonHang` bất kỳ, vì logic hoàn tiền THẬT SỰ
PHỤ THUỘC trạng thái.
::::

::::predict{#doan-tao-don-rong commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type MatHang = { ten: string; soLuong: number; donGia: number };
type DonHang =
  | { tag: "nhap"; matHang: MatHang[] }
  | { tag: "da_xac_nhan"; matHang: MatHang[]; tongTien: number }
  | { tag: "da_giao"; matHang: MatHang[]; tongTien: number; maVanDon: string }
  | { tag: "da_huy"; hoanTien: number; lyDo: string };
function taoNhap(matHang: MatHang[]): Result<DonHang, string> {
  if (matHang.length === 0) return loi("đơn hàng phải có ít nhất một mặt hàng");
  return ok({ tag: "nhap", matHang });
}

const ketQua = taoNhap([]);
console.log(ketQua.kind);
```

Dòng cuối in ra gì (mảng `matHang` truyền vào RỖNG)?

:::opt{correct}
`loi`
:::

:::opt
`ok` — vì `taoNhap` chỉ TẠO một object `DonHang` với `matHang` bất kỳ
được truyền vào, không thực sự kiểm tra nội dung mảng đó
::why
Gần đúng ở việc bạn nhớ ĐÚNG `taoNhap` NHẬN `matHang` làm tham số và
DÙNG nó để tạo `DonHang` — quan sát về luồng dữ liệu đó đúng.

Chỗ lệch: `taoNhap` là SMART CONSTRUCTOR (đã học từ T4.3) — nó KIỂM
TRA NGAY trước khi tạo: `if (matHang.length === 0) return loi(...)`.
Mảng RỖNG (`length === 0`) KHỚP điều kiện đó, hàm `return` NGAY một
`loi(...)`, KHÔNG BAO GIỜ chạy tới dòng `return ok({...})`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `taoNhap([])` với mảng RỖNG không khớp
kiểu `MatHang[]` mà hàm khai báo
::why
Gần đúng ở việc bạn để ý mảng `[]` (rỗng) CÓ vẻ "thiếu" gì đó so với
`MatHang[]` — quan sát về SỰ THIẾU HỤT NỘI DUNG đó hợp lý về mặt trực
giác.

Chỗ lệch: `MatHang[]` là kiểu "một MẢNG chứa các `MatHang`" — một mảng
RỖNG (`[]`) là MỘT MẢNG hợp lệ VỀ MẶT KIỂU (`length === 0` vẫn là một
`MatHang[]` hoàn toàn đúng kiểu, chỉ đơn giản không có phần tử nào).
"Không cho phép rỗng" là một RÀNG BUỘC NGHIỆP VỤ, kiểm LÚC CHẠY qua
`if`, không phải giới hạn của kiểu mảng.
::
:::
::::

::::code{#viet_huy_nhap_va_huy_da_xac_nhan}
Tự viết `huyNhap` và `huyDaXacNhan` — business rule hoàn tiền KHÁC
nhau tuỳ trạng thái.

```typescript title=starter
type MatHang = { ten: string; soLuong: number; donGia: number };
type DonHang =
  | { tag: "nhap"; matHang: MatHang[] }
  | { tag: "da_xac_nhan"; matHang: MatHang[]; tongTien: number }
  | { tag: "da_giao"; matHang: MatHang[]; tongTien: number; maVanDon: string }
  | { tag: "da_huy"; hoanTien: number; lyDo: string };

function huyNhap(dh: DonHang & { tag: "nhap" }, lyDo: string): DonHang & { tag: "da_huy" } {
  return ___;
}

function huyDaXacNhan(dh: DonHang & { tag: "da_xac_nhan" }, lyDo: string): DonHang & { tag: "da_huy" } {
  return ___;
}

const dhNhap: DonHang & { tag: "nhap" } = { tag: "nhap", matHang: [] };
console.log(JSON.stringify(huyNhap(dhNhap, "test")));
```

```typescript title=solution
type MatHang = { ten: string; soLuong: number; donGia: number };
type DonHang =
  | { tag: "nhap"; matHang: MatHang[] }
  | { tag: "da_xac_nhan"; matHang: MatHang[]; tongTien: number }
  | { tag: "da_giao"; matHang: MatHang[]; tongTien: number; maVanDon: string }
  | { tag: "da_huy"; hoanTien: number; lyDo: string };

function huyNhap(dh: DonHang & { tag: "nhap" }, lyDo: string): DonHang & { tag: "da_huy" } {
  return { tag: "da_huy", hoanTien: 0, lyDo };
}

function huyDaXacNhan(dh: DonHang & { tag: "da_xac_nhan" }, lyDo: string): DonHang & { tag: "da_huy" } {
  return { tag: "da_huy", hoanTien: dh.tongTien, lyDo };
}

const dhNhap: DonHang & { tag: "nhap" } = { tag: "nhap", matHang: [] };
console.log(JSON.stringify(huyNhap(dhNhap, "test")));
```

```typescript title=test
const dhNhapTest: DonHang & { tag: "nhap" } = { tag: "nhap", matHang: [{ ten: "Ly", soLuong: 1, donGia: 30000 }] };
const kqHuyNhap = huyNhap(dhNhapTest, "hết hàng");
if (kqHuyNhap.tag !== "da_huy") throw new Error("huyNhap phải trả về trạng thái da_huy");
if (kqHuyNhap.hoanTien !== 0) throw new Error("huỷ đơn CHƯA xác nhận phải hoàn 0đ (chưa hề thanh toán)");
if (kqHuyNhap.lyDo !== "hết hàng") throw new Error("phải giữ đúng lý do truyền vào");

const dhXacNhan: DonHang & { tag: "da_xac_nhan" } = { tag: "da_xac_nhan", matHang: dhNhapTest.matHang, tongTien: 250000 };
const kqHuyXacNhan = huyDaXacNhan(dhXacNhan, "khách đổi ý");
if (kqHuyXacNhan.tag !== "da_huy") throw new Error("huyDaXacNhan phải trả về trạng thái da_huy");
if (kqHuyXacNhan.hoanTien !== 250000) throw new Error("huỷ đơn ĐÃ xác nhận phải hoàn ĐÚNG tongTien (250000)");
if (kqHuyXacNhan.lyDo !== "khách đổi ý") throw new Error("phải giữ đúng lý do truyền vào");
```

:::hints
- kind: attention
  body: "huyNhap: đơn CHƯA xác nhận chưa hề có tongTien nào -- hoàn 0. huyDaXacNhan: đơn ĐÃ xác nhận có sẵn dh.tongTien -- hoàn ĐÚNG số đó."
- kind: strategy
  body: '{ tag: "da_huy", hoanTien: 0, lyDo } — huyNhap. { tag: "da_huy", hoanTien: dh.tongTien, lyDo } — huyDaXacNhan.'
- kind: one-line
  body: '___ (huyNhap) = { tag: "da_huy", hoanTien: 0, lyDo }\n___ (huyDaXacNhan) = { tag: "da_huy", hoanTien: dh.tongTien, lyDo }'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "da_huy"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 3 xong. Aggregate Root: VO + DU state machine + validate MỌI bước
— MỘT entity chịu trách nhiệm toàn vẹn dữ liệu, business rule có thể
khác nhau THEO trạng thái.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có Domain Model hoàn chỉnh. Giờ cần GHÉP nhiều bước xử lý (validate
→ tính giá → xác nhận) thành MỘT luồng — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
