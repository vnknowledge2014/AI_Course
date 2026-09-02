---
id: ky-nghe-phan-mem.ddd.transition-functions-ep-dung-thu-tu
title: "Transition Functions — compiler ép gọi ĐÚNG thứ tự, không cần check runtime"
summary: "TrangThaiDonHang = 6 variant, mỗi variant field bắt buộc RIÊNG. Transition function nhận state CỤ THỂ qua intersection type: xacNhanThanhToan(dh: TrangThaiDonHang & {tag:\"cho_thanh_toan\"}): ... & {tag:\"da_thanh_toan\"} — gọi SAI thứ tự = lỗi COMPILE-TIME."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 15
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.transition-functions]
requires: [ddd.boolean-flags-vs-du]
concepts: [ddd.transition-functions]
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
Bài trước đặt vấn đề: boolean flags cho phép trạng thái phi lý. Giải
pháp: discriminated union — mỗi variant mang ĐÚNG data cần cho NÓ.
::::

::::explain{#state-machine-bang-du}
`TrangThaiDonHang` = SÁU variant, MỖI variant field BẮT BUỘC RIÊNG
(không optional):

```typescript
type TrangThaiDonHang =
  | { tag: "nhap"; maDonHang: string }
  | { tag: "cho_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_giao"; maDonHang: string; maVanDon: string }
  | { tag: "da_nhan"; maDonHang: string }
  | { tag: "da_huy"; maDonHang: string; lyDo: string };
```

Chỉ `"da_giao"` CÓ `maVanDon` (mã vận đơn — CHỈ có nghĩa SAU khi đã
giao); chỉ `"da_huy"` CÓ `lyDo`. KHÔNG có tổ hợp phi lý nào (như "đã
huỷ mà vẫn đã giao") biểu diễn được — mỗi giá trị `TrangThaiDonHang`
LÀ ĐÚNG MỘT trong sáu trạng thái, không hơn không kém.

**Transition function** nhận state CỤ THỂ qua **intersection type**
(`&`) — compiler ÉP gọi ĐÚNG thứ tự:

```typescript
function xacNhanThanhToan(
  dh: TrangThaiDonHang & { tag: "cho_thanh_toan" },
): TrangThaiDonHang & { tag: "da_thanh_toan" } {
  return { tag: "da_thanh_toan", maDonHang: dh.maDonHang, tongTien: dh.tongTien };
}

function giaoHang(
  dh: TrangThaiDonHang & { tag: "da_thanh_toan" },
  maVanDon: string,
): TrangThaiDonHang & { tag: "da_giao" } {
  return { tag: "da_giao", maDonHang: dh.maDonHang, maVanDon };
}

const dhChoTT: TrangThaiDonHang & { tag: "cho_thanh_toan" } = { tag: "cho_thanh_toan", maDonHang: "DH-01", tongTien: 100000 };
const dhTT = xacNhanThanhToan(dhChoTT);
console.log(dhTT);
const dhGiao = giaoHang(dhTT, "VD-01");
console.log(dhGiao);
```

```text
{"tag":"da_thanh_toan","maDonHang":"DH-01","tongTien":100000}
{"tag":"da_giao","maDonHang":"DH-01","maVanDon":"VD-01"}
```

`giaoHang` CHỈ nhận `TrangThaiDonHang & { tag: "da_thanh_toan" }` — một
đơn hàng CÒN ở `"nhap"` hay `"cho_thanh_toan"` KHÔNG khớp kiểu tham số
đó, TypeScript TỪ CHỐI gọi hàm NGAY LÚC BIÊN DỊCH.
::::

::::example{#goi-sai-thu-tu-loi-bien-dich}
Gọi `giaoHang` trên một đơn hàng CÒN Ở `"nhap"` (chưa hề thanh toán):

```typescript title=readonly
type TrangThaiDonHang =
  | { tag: "nhap"; maDonHang: string }
  | { tag: "cho_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_giao"; maDonHang: string; maVanDon: string }
  | { tag: "da_nhan"; maDonHang: string }
  | { tag: "da_huy"; maDonHang: string; lyDo: string };
function giaoHang(
  dh: TrangThaiDonHang & { tag: "da_thanh_toan" },
  maVanDon: string,
): TrangThaiDonHang & { tag: "da_giao" } {
  return { tag: "da_giao", maDonHang: dh.maDonHang, maVanDon };
}

const dhNhap: TrangThaiDonHang & { tag: "nhap" } = { tag: "nhap", maDonHang: "DH-02" };
// giaoHang(dhNhap, "VD-02");  // ← nếu bỏ comment: LỖI BIÊN DỊCH ngay
console.log("nếu bỏ comment dòng trên, TypeScript báo lỗi kiểu ngay lập tức");
```

```text title=readonly
nếu bỏ comment dòng trên, TypeScript báo lỗi kiểu ngay lập tức
```

`dhNhap` có kiểu `TrangThaiDonHang & { tag: "nhap" }` — THIẾU field
`tongTien` mà `TrangThaiDonHang & { tag: "da_thanh_toan" }` YÊU CẦU.
TypeScript báo `TS2345` (đối số sai kiểu) NGAY, KHÔNG cần chạy chương
trình để phát hiện lỗi thứ tự này — khác HẲN với boolean flags (bài
14), nơi lỗi CHỈ phát hiện được lúc chạy, NẾU có ai đó chủ động kiểm
tra.
::::

::::predict{#doan-tao-don-tu-nhap commitOnce}
```typescript
type TrangThaiDonHang =
  | { tag: "nhap"; maDonHang: string }
  | { tag: "cho_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_giao"; maDonHang: string; maVanDon: string }
  | { tag: "da_nhan"; maDonHang: string }
  | { tag: "da_huy"; maDonHang: string; lyDo: string };
function xacNhanThanhToan(
  dh: TrangThaiDonHang & { tag: "cho_thanh_toan" },
): TrangThaiDonHang & { tag: "da_thanh_toan" } {
  return { tag: "da_thanh_toan", maDonHang: dh.maDonHang, tongTien: dh.tongTien };
}

const dh: TrangThaiDonHang & { tag: "cho_thanh_toan" } = { tag: "cho_thanh_toan", maDonHang: "DH-09", tongTien: 250000 };
const ketQua = xacNhanThanhToan(dh);
console.log("tongTien" in ketQua);
console.log("maVanDon" in ketQua);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`true` rồi `false`
:::

:::opt
`true` rồi `true` — vì `TrangThaiDonHang` là MỘT type CHUNG, mọi field
của TẤT CẢ variant đều CÓ MẶT trên bất kỳ giá trị nào thuộc type đó
::why
Gần đúng ở việc bạn nhớ ĐÚNG `TrangThaiDonHang` là MỘT union GỘP nhiều
variant — quan sát về việc CÓ NHIỀU field khả dĩ trong union đó đúng.

Chỗ lệch: MỖI GIÁ TRỊ CỤ THỂ thuộc union chỉ MANG field của ĐÚNG MỘT
variant — `ketQua` (`{tag: "da_thanh_toan"}`) chỉ có `maDonHang` VÀ
`tongTien`, KHÔNG có `maVanDon` (field đó CHỈ tồn tại trên variant
`"da_giao"`, một variant KHÁC HẲN). `"maVanDon" in ketQua` kiểm object
THẬT có key đó không — không có, `false`.
::
:::

:::opt
Máy báo lỗi biên dịch — toán tử `in` không dùng được trên một giá trị
có kiểu là UNION của nhiều variant khác nhau
::why
Gần đúng ở việc bạn để ý `ketQua` có kiểu PHỨC TẠP (intersection của
union và một tag cụ thể) — quan sát về ĐỘ PHỨC TẠP của kiểu đó đúng.

Chỗ lệch: toán tử `in` hoạt động trên BẤT KỲ object nào lúc CHẠY, bất
kể kiểu TĨNH của nó phức tạp ra sao — không có hạn chế nào cấm dùng
`in` trên union type. Biên dịch và chạy hoàn toàn bình thường.
::
:::
::::

::::code{#viet_transition_functions}
Tự viết `xacNhanThanhToan` và `giaoHang`.

```typescript title=starter
type TrangThaiDonHang =
  | { tag: "nhap"; maDonHang: string }
  | { tag: "cho_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_giao"; maDonHang: string; maVanDon: string }
  | { tag: "da_nhan"; maDonHang: string }
  | { tag: "da_huy"; maDonHang: string; lyDo: string };

function xacNhanThanhToan(
  dh: TrangThaiDonHang & { tag: "cho_thanh_toan" },
): TrangThaiDonHang & { tag: "da_thanh_toan" } {
  return ___;
}

function giaoHang(
  dh: TrangThaiDonHang & { tag: "da_thanh_toan" },
  maVanDon: string,
): TrangThaiDonHang & { tag: "da_giao" } {
  return ___;
}

const dhChoTT: TrangThaiDonHang & { tag: "cho_thanh_toan" } = { tag: "cho_thanh_toan", maDonHang: "DH-01", tongTien: 100000 };
console.log(giaoHang(xacNhanThanhToan(dhChoTT), "VD-01"));
```

```typescript title=solution
type TrangThaiDonHang =
  | { tag: "nhap"; maDonHang: string }
  | { tag: "cho_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_thanh_toan"; maDonHang: string; tongTien: number }
  | { tag: "da_giao"; maDonHang: string; maVanDon: string }
  | { tag: "da_nhan"; maDonHang: string }
  | { tag: "da_huy"; maDonHang: string; lyDo: string };

function xacNhanThanhToan(
  dh: TrangThaiDonHang & { tag: "cho_thanh_toan" },
): TrangThaiDonHang & { tag: "da_thanh_toan" } {
  return { tag: "da_thanh_toan", maDonHang: dh.maDonHang, tongTien: dh.tongTien };
}

function giaoHang(
  dh: TrangThaiDonHang & { tag: "da_thanh_toan" },
  maVanDon: string,
): TrangThaiDonHang & { tag: "da_giao" } {
  return { tag: "da_giao", maDonHang: dh.maDonHang, maVanDon };
}

const dhChoTT: TrangThaiDonHang & { tag: "cho_thanh_toan" } = { tag: "cho_thanh_toan", maDonHang: "DH-01", tongTien: 100000 };
console.log(giaoHang(xacNhanThanhToan(dhChoTT), "VD-01"));
```

```typescript title=test
const dh1: TrangThaiDonHang & { tag: "cho_thanh_toan" } = { tag: "cho_thanh_toan", maDonHang: "DH-01", tongTien: 100000 };
const dhTT = xacNhanThanhToan(dh1);
if (dhTT.tag !== "da_thanh_toan") throw new Error("xacNhanThanhToan phải trả về trạng thái da_thanh_toan");
if (dhTT.maDonHang !== "DH-01") throw new Error("phải giữ nguyên maDonHang qua transition");
if (dhTT.tongTien !== 100000) throw new Error("phải giữ nguyên tongTien qua transition");

const dhGiao = giaoHang(dhTT, "VD-77");
if (dhGiao.tag !== "da_giao") throw new Error("giaoHang phải trả về trạng thái da_giao");
if (dhGiao.maVanDon !== "VD-77") throw new Error("phải giữ đúng maVanDon truyền vào");
if (dhGiao.maDonHang !== "DH-01") throw new Error("phải giữ nguyên maDonHang xuyên suốt cả hai transition");
```

:::hints
- kind: attention
  body: "xacNhanThanhToan: tạo object MỚI tag \"da_thanh_toan\", giữ nguyên maDonHang và tongTien từ dh. giaoHang: tạo object MỚI tag \"da_giao\", giữ nguyên maDonHang từ dh, thêm maVanDon từ tham số."
- kind: strategy
  body: '{ tag: "da_thanh_toan", maDonHang: dh.maDonHang, tongTien: dh.tongTien } — cho xacNhanThanhToan. { tag: "da_giao", maDonHang: dh.maDonHang, maVanDon } — cho giaoHang.'
- kind: one-line
  body: '___ (xacNhanThanhToan) = { tag: "da_thanh_toan", maDonHang: dh.maDonHang, tongTien: dh.tongTien }\n___ (giaoHang) = { tag: "da_giao", maDonHang: dh.maDonHang, maVanDon }'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "da_giao"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Transition function nhận state CỤ THỂ — gọi SAI thứ tự bị compiler
CHẶN NGAY, không cần chạy chương trình để phát hiện.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`TrangThaiDonHang` giải quyết vấn đề boolean flags — nhưng nếu MỘT
field CHỈ liên quan tới MỘT ĐIỀU KIỆN khác (không phải toàn bộ trạng
thái), boolean flags vẫn có thể lén quay lại. Ở đâu?
::::

::::checkpoint{mastery=0.8}
::::
