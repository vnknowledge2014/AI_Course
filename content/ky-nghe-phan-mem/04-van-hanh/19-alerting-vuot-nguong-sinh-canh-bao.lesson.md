---
id: ky-nghe-phan-mem.van-hanh.alerting-vuot-nguong-sinh-canh-bao
title: "Alerting — khi metric VƯỢT ngưỡng, sinh CẢNH BÁO"
summary: "kiemTraCanhBao(giaTriMetric, nguong, tenMetric): {canhBao, thongDiep} — so metric VỚI ngưỡng ĐÃ định trước, sinh cảnh báo có THÔNG ĐIỆP rõ ràng NẾU vượt. Ranh giới: vượt QUÁ ngưỡng (>, KHÔNG >=) mới cảnh báo — nối kỷ luật \"ranh giới CHÍNH XÁC\" đã dùng T5.4 bài 24."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 19
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [vh.alerting-threshold]
requires: [vh.health-check-dependencies]
concepts: [vh.alerting-threshold]
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
`kiemTraSucKhoe` (bài 18) trả `"om"` khi CÓ dependency hỏng. NHƯNG
CHỈ biết "om" KHÔNG đủ — CẦN chủ động BÁO cho người vận hành.
::::

::::explain{#alerting}
**Alerting** = SO SÁNH metric VỚI ngưỡng ĐÃ định TRƯỚC, sinh MỘT
cảnh báo CÓ thông ĐIỆP rõ ràng NẾU vượt — ÁP DỤNG cho BẤT KỲ metric
NÀO (tỷ lệ lỗi, latency, BẤT CỨ con số NÀO đo được), KHÔNG PHẢI
riêng cho MỘT loại:

```typescript title=readonly
function kiemTraCanhBao(
  giaTriMetric: number,
  nguong: number,
  tenMetric: string,
): { canhBao: boolean; thongDiep: string } {
  const vuot = giaTriMetric > nguong;
  return {
    canhBao: vuot,
    thongDiep: vuot
      ? `${tenMetric} vuot nguong: ${giaTriMetric} > ${nguong}`
      : `${tenMetric} binh thuong`,
  };
}

console.log(kiemTraCanhBao(0.15, 0.1, "ty_le_loi"));
console.log(kiemTraCanhBao(0.03, 0.1, "ty_le_loi"));
```

```text title=readonly
{"canhBao":true,"thongDiep":"ty_le_loi vuot nguong: 0.15 > 0.1"}
{"canhBao":false,"thongDiep":"ty_le_loi binh thuong"}
```

`0.15 > 0.1` (VƯỢT) → `canhBao: true`, `thongDiep` GIẢI THÍCH RÕ
metric NÀO, giá trị BAO NHIÊU, ngưỡng LÀ bao nhiêu. `0.03 > 0.1`
LÀ `false` (CHƯA vượt) → `canhBao: false`, thông điệp NGẮN gọn
"bình thường".
::::

::::example{#dung-bang-nguong-chua-vuot}
Metric **ĐÚNG BẰNG** ngưỡng — RANH GIỚI (đã kỷ luật T5.4 bài 24):
`>` (LỚN HƠN nghiêm ngặt), KHÔNG `>=`, NÊN "đúng bằng" CHƯA gọi LÀ
vượt:

```typescript title=readonly
function kiemTraCanhBao(giaTriMetric: number, nguong: number, tenMetric: string): { canhBao: boolean; thongDiep: string } {
  const vuot = giaTriMetric > nguong;
  return { canhBao: vuot, thongDiep: vuot ? `${tenMetric} vuot nguong: ${giaTriMetric} > ${nguong}` : `${tenMetric} binh thuong` };
}
console.log(kiemTraCanhBao(500, 500, "latency_p99"));
```

```text title=readonly
{"canhBao":false,"thongDiep":"latency_p99 binh thuong"}
```

`giaTriMetric` (`500`) ĐÚNG BẰNG `nguong` (`500`) — `500 > 500` LÀ
`false` → KHÔNG cảnh báo. NGƯỠNG LÀ MỨC CHẤP NHẬN ĐƯỢC, KHÔNG PHẢI
mức bắt đầu nguy hiểm — chỉ khi VƯỢT QUA nó MỚI đáng báo.
::::

::::predict{#doan-thong-diep-dung-cau-truc commitOnce}
```typescript
function kiemTraCanhBao(giaTriMetric: number, nguong: number, tenMetric: string): { canhBao: boolean; thongDiep: string } {
  const vuot = giaTriMetric > nguong;
  return { canhBao: vuot, thongDiep: vuot ? `${tenMetric} vuot nguong: ${giaTriMetric} > ${nguong}` : `${tenMetric} binh thuong` };
}
console.log(kiemTraCanhBao(0.15, 0.1, "ty_le_loi").thongDiep);
```

Dòng cuối in ra CHUỖI nào?

:::opt{correct}
`ty_le_loi vuot nguong: 0.15 > 0.1`
:::

:::opt
`vuot nguong: ty_le_loi 0.15 > 0.1` — vì cách đọc TỰ NHIÊN của MỘT
cảnh báo THƯỜNG bắt đầu bằng TỪ KHOÁ hành động ("vuot nguong")
TRƯỚC, RỒI mới NÊU tên metric CỤ THỂ, GIỐNG cấu trúc "CẢNH BÁO:
[metric] [giá trị]" quen THUỘC trong log THỰC tế
::why
Gần đúng ở việc bạn hình DUNG một cấu trúc thông điệp cảnh báo HỢP
LÝ — NHIỀU hệ thống THẬT quả CÓ những cách trình bày tương TỰ.

Chỗ lệch: chuỗi ĐƯỢC ghép CHÍNH XÁC theo THỨ TỰ các phần TỬ nội suy
(`${...}`) xuất hiện Ở template literal — `` `${tenMetric} vuot
nguong: ${giaTriMetric} > ${nguong}` `` đặt `tenMetric` **ĐẦU
TIÊN**, RỒI mới tới `"vuot nguong: "`, RỒI `giaTriMetric`, `" > "`,
`nguong`. KHÔNG CÓ bước "sắp XẾP lại" NÀO xảy ra — template literal
LUÔN ghép chuỗi ĐÚNG theo thứ tự VIẾT trong CODE.
::
:::

:::opt
Máy báo lỗi biên dịch — template literal `` `${tenMetric} vuot
nguong: ${giaTriMetric} > ${nguong}` `` TRỘN LẪN `string` (`tenMetric`)
VÀ `number` (`giaTriMetric`, `nguong`) TRONG CÙNG một chuỗi, TypeScript
CẤM nội suy HAI kiểu KHÁC nhau chung MỘT template literal
::why
Gần đúng ở việc bạn để ý `tenMetric` (`string`) VÀ `giaTriMetric`/
`nguong` (`number`) LÀ HAI kiểu KHÁC nhau — MỘT quan sát ĐÚNG về
kiểu DỮ LIỆU.

Chỗ lệch: template literal CHẤP NHẬN nội suy **BẤT KỲ** giá trị NÀO
CÓ cách chuyển THÀNH chuỗi (MỌI kiểu nguyên thuỷ: `string`, `number`,
`boolean`, ...) — TypeScript TỰ ĐỘNG gọi `String(...)` cho MỖI biểu
thức TRONG `${...}`, KHÔNG hề CẤM trộn kiểu. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_kiem_tra_canh_bao}
Hoàn thiện `kiemTraCanhBao` — so SÁNH nghiêm ngặt (`>`), ghép thông
điệp bằng template literal.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function kiemTraCanhBao(giaTriMetric: number, nguong: number, tenMetric: string): { canhBao: boolean; thongDiep: string } {
  const vuot = ___;
  return {
    canhBao: vuot,
    thongDiep: vuot ? `${tenMetric} vuot nguong: ${giaTriMetric} > ${nguong}` : `${tenMetric} binh thuong`,
  };
}

assertEqual(kiemTraCanhBao(0.03, 0.1, "ty_le_loi").canhBao, false, "duoi nguong -- khong canh bao");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function kiemTraCanhBao(giaTriMetric: number, nguong: number, tenMetric: string): { canhBao: boolean; thongDiep: string } {
  const vuot = giaTriMetric > nguong;
  return {
    canhBao: vuot,
    thongDiep: vuot ? `${tenMetric} vuot nguong: ${giaTriMetric} > ${nguong}` : `${tenMetric} binh thuong`,
  };
}

assertEqual(kiemTraCanhBao(0.03, 0.1, "ty_le_loi").canhBao, false, "duoi nguong -- khong canh bao");
```

```typescript title=test
assertEqual(kiemTraCanhBao(0.15, 0.1, "ty_le_loi").canhBao, true, "vuot nguong -- co canh bao");
assertEqual(kiemTraCanhBao(0.1, 0.1, "ty_le_loi").canhBao, false, "dung bang nguong -- CHUA goi la vuot");
assertEqual(kiemTraCanhBao(0.15, 0.1, "ty_le_loi").thongDiep, "ty_le_loi vuot nguong: 0.15 > 0.1", "thong diep dung khi vuot");
assertEqual(kiemTraCanhBao(0.03, 0.1, "ty_le_loi").thongDiep, "ty_le_loi binh thuong", "thong diep dung khi binh thuong");
```

:::hints
- kind: attention
  body: "vuot phai la giaTriMetric > nguong (nghiem ngat, khong >=)."
- kind: strategy
  body: "giaTriMetric > nguong"
- kind: one-line
  body: '___ = giaTriMetric > nguong'
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
Cảnh báo có cấu trúc, ranh giới chính xác. Bài chốt cụm: ghép trace
với health check thành một báo cáo request đầy đủ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`timBuocChamNhat` (bài 17) VÀ `kiemTraSucKhoe` (bài 18) hiện LÀ HAI
công cụ RIÊNG. Ghép CẢ HAI vào MỘT báo cáo DUY NHẤT cho MỘT request
trông NHƯ thế nào?
::::

::::checkpoint{mastery=0.8}
::::
