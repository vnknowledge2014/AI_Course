---
id: ky-nghe-phan-mem.van-hanh.trace-theo-doi-request-qua-nhieu-buoc
title: "Trace — theo dõi MỘT request qua NHIỀU bước xử lý"
summary: "Span = {tenBuoc, batDau, ketThuc} (thời gian ĐƯỢC TRUYỀN VÀO — injected, không Date.now()). tinhThoiGian(span) = ketThuc - batDau. timBuocChamNhat(cacSpan): string — LẶP qua, GIỮ lại bước có thời gian LỚN NHẤT (dùng > nên KHI hoà, bước ĐẦU TIÊN thắng)."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 17
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.tracing-spans]
requires: [vh.gate-boss-metrics]
concepts: [vh.tracing-spans]
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
`taoDashboard` (bài 16) cho biết p90 CAO bất thường. NHƯNG "BƯỚC NÀO
trong request LÀM CHẬM" — số liệu tổng hợp KHÔNG hề trả lời.
::::

::::explain{#trace-va-span}
MỘT request THẬT đi QUA nhiều BƯỚC (validate → truy vấn dữ liệu →
tính toán → trả về). **Trace** ghi LẠI thời GIAN bắt đầu/kết thúc
của TỪNG bước — MỖI bước LÀ MỘT "**span**":

```typescript title=readonly
type Span = { tenBuoc: string; batDau: number; ketThuc: number };

function tinhThoiGian(span: Span): number {
  return span.ketThuc - span.batDau;
}

function timBuocChamNhat(cacSpan: Span[]): string {
  let baiChamNhat = cacSpan[0]!;
  for (const span of cacSpan) {
    if (tinhThoiGian(span) > tinhThoiGian(baiChamNhat)) {
      baiChamNhat = span;
    }
  }
  return baiChamNhat.tenBuoc;
}

const cacSpan: Span[] = [
  { tenBuoc: "validate", batDau: 0, ketThuc: 5 },
  { tenBuoc: "truy-van", batDau: 5, ketThuc: 120 },
  { tenBuoc: "tinh-toan", batDau: 120, ketThuc: 135 },
];
console.log(tinhThoiGian(cacSpan[1]!));
console.log(timBuocChamNhat(cacSpan));
```

```text title=readonly
115
truy-van
```

`batDau`/`ketThuc` LÀ thời GIAN **ĐƯỢC TRUYỀN VÀO** (injected — quy
ước dự án, KHÔNG `Date.now()`). `tinhThoiGian` TRỪ trực tiếp. Ba
bước mất `5`, `115`, `15` — `"truy-van"` (`115`) CHẬM nhất — CHÍNH
LÀ bước ĐÁNG điều TRA khi p90 tổng thể CAO bất thường.
::::

::::example{#thu-tu-mang-khong-anh-huong}
`timBuocChamNhat` LẶP qua **TOÀN BỘ** mảng, NÊN thứ tự CÁC phần TỬ
trong mảng ĐẦU VÀO KHÔNG ảnh hưởng kết quả:

```typescript title=readonly
type Span = { tenBuoc: string; batDau: number; ketThuc: number };
function tinhThoiGian(span: Span): number { return span.ketThuc - span.batDau; }
function timBuocChamNhat(cacSpan: Span[]): string {
  let baiChamNhat = cacSpan[0]!;
  for (const span of cacSpan) {
    if (tinhThoiGian(span) > tinhThoiGian(baiChamNhat)) baiChamNhat = span;
  }
  return baiChamNhat.tenBuoc;
}
const tinhToan: Span = { tenBuoc: "tinh-toan", batDau: 120, ketThuc: 135 };
const validate: Span = { tenBuoc: "validate", batDau: 0, ketThuc: 5 };
const truyVan: Span = { tenBuoc: "truy-van", batDau: 5, ketThuc: 120 };
console.log(timBuocChamNhat([tinhToan, validate, truyVan]));
```

```text title=readonly
truy-van
```

DÙ `tinhToan` (đứng ĐẦU mảng lần NÀY) KHÔNG PHẢI bước chậm nhất,
kết quả VẪN LÀ `"truy-van"` — GIỐNG HỆT ví dụ TRƯỚC, DÙ thứ tự
mảng đã bị đảo HOÀN TOÀN.
::::

::::predict{#doan-hoa-thoi-gian-buoc-dau-thang commitOnce}
```typescript
type Span = { tenBuoc: string; batDau: number; ketThuc: number };
function tinhThoiGian(span: Span): number { return span.ketThuc - span.batDau; }
function timBuocChamNhat(cacSpan: Span[]): string {
  let baiChamNhat = cacSpan[0]!;
  for (const span of cacSpan) {
    if (tinhThoiGian(span) > tinhThoiGian(baiChamNhat)) baiChamNhat = span;
  }
  return baiChamNhat.tenBuoc;
}
const cacSpan: Span[] = [
  { tenBuoc: "A", batDau: 0, ketThuc: 10 },
  { tenBuoc: "B", batDau: 0, ketThuc: 10 },
];
console.log(timBuocChamNhat(cacSpan));
```

HAI bước `"A"` VÀ `"B"` mất ĐÚNG **CÙNG** thời gian (`10`). Dòng
cuối in ra gì?

:::opt{correct}
`"A"`
:::

:::opt
`"B"` — vì VÒNG LẶP `for` xử lý TUẦN TỰ, VÀ khi GẶP một bước MỚI CÓ
thời gian **BẰNG** bước ĐANG giữ, `timBuocChamNhat` CẬP NHẬT sang
bước MỚI đó (ưu tiên bước GẦN ĐÂY nhất trong LẦN duyệt)
::why
Gần đúng ở việc bạn nhận RA hai bước `"A"`/`"B"` CÓ CÙNG thời gian
(`10`) — MỘT quan sát ĐÚNG về DỮ LIỆU.

Chỗ lệch: điều kiện CẬP NHẬT LÀ `tinhThoiGian(span) > tinhThoiGian
(baiChamNhat)` — dùng **`>`** (LỚN HƠN NGHIÊM NGẶT), KHÔNG PHẢI
`>=`. Khi xét TỚI `"B"` (`10`), SO với `baiChamNhat` LÚC ĐÓ ĐANG LÀ
`"A"` (CŨNG `10`): `10 > 10` LÀ `false` — ĐIỀU KIỆN KHÔNG thoả,
`baiChamNhat` **KHÔNG** đổi, VẪN LÀ `"A"`. Quy tắc: khi HOÀ, bước
XUẤT HIỆN **ĐẦU TIÊN** trong mảng LUÔN thắng.
::
:::

:::opt
Máy báo lỗi biên dịch — `Span[]` chứa HAI phần tử có `batDau`/
`ketThuc` GIỐNG HỆT NHAU (`0` và `10`), TypeScript CẤM mảng object
CÓ hai phần tử TRÙNG giá trị Ở CÙNG trường
::why
Gần đúng ở việc bạn để ý HAI span `"A"`/`"B"` có `batDau`/`ketThuc`
GIỐNG hệt nhau — MỘT quan sát ĐÚNG về NỘI DUNG dữ liệu.

Chỗ lệch: TypeScript HOÀN TOÀN KHÔNG có khái niệm "CẤM giá trị
TRÙNG LẶP" cho mảng object — HAI (HAY NHIỀU) phần tử có CÙNG giá
trị Ở BẤT KỲ trường NÀO ĐỀU hợp lệ. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_trace_span}
Hoàn thiện `tinhThoiGian` VÀ `timBuocChamNhat`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Span = { tenBuoc: string; batDau: number; ketThuc: number };

function tinhThoiGian(span: Span): number {
  return ___;
}

function timBuocChamNhat(cacSpan: Span[]): string {
  let baiChamNhat = cacSpan[0]!;
  for (const span of cacSpan) {
    if (tinhThoiGian(span) > tinhThoiGian(baiChamNhat)) {
      baiChamNhat = ___;
    }
  }
  return baiChamNhat.tenBuoc;
}

assertEqual(tinhThoiGian({ tenBuoc: "x", batDau: 10, ketThuc: 25 }), 15, "tinh thoi gian dung");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Span = { tenBuoc: string; batDau: number; ketThuc: number };

function tinhThoiGian(span: Span): number {
  return span.ketThuc - span.batDau;
}

function timBuocChamNhat(cacSpan: Span[]): string {
  let baiChamNhat = cacSpan[0]!;
  for (const span of cacSpan) {
    if (tinhThoiGian(span) > tinhThoiGian(baiChamNhat)) {
      baiChamNhat = span;
    }
  }
  return baiChamNhat.tenBuoc;
}

assertEqual(tinhThoiGian({ tenBuoc: "x", batDau: 10, ketThuc: 25 }), 15, "tinh thoi gian dung");
```

```typescript title=test
const cacSpan: Span[] = [
  { tenBuoc: "validate", batDau: 0, ketThuc: 5 },
  { tenBuoc: "truy-van", batDau: 5, ketThuc: 120 },
  { tenBuoc: "tinh-toan", batDau: 120, ketThuc: 135 },
];
assertEqual(timBuocChamNhat(cacSpan), "truy-van", "tim buoc cham nhat trong ba buoc");
assertEqual(timBuocChamNhat([{ tenBuoc: "chi-mot", batDau: 0, ketThuc: 5 }]), "chi-mot", "chi mot span");

const hoaDuration: Span[] = [
  { tenBuoc: "A", batDau: 0, ketThuc: 10 },
  { tenBuoc: "B", batDau: 0, ketThuc: 10 },
];
assertEqual(timBuocChamNhat(hoaDuration), "A", "hoa -- buoc dau tien thang");
```

:::hints
- kind: attention
  body: "tinhThoiGian: ketThuc tru batDau. timBuocChamNhat: cap nhat baiChamNhat = span hien tai (khong phai .tenBuoc)."
- kind: strategy
  body: "span.ketThuc - span.batDau : span"
- kind: one-line
  body: '___ (tinhThoiGian) = span.ketThuc - span.batDau\n___ (cap nhat) = span'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Trace lộ ra bước chậm nhất trong một request. Bài tiếp theo: request
CÒN chạy được không — hay CÓ dependency đã hỏng?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`timBuocChamNhat` cho biết bước NÀO chậm — NHƯNG "hệ thống CÒN sống
hay đã HỎNG hoàn toàn" LÀ một câu hỏi KHÁC. Cần kiểm tra GÌ để trả
lời câu ĐÓ?
::::

::::checkpoint{mastery=0.8}
::::
