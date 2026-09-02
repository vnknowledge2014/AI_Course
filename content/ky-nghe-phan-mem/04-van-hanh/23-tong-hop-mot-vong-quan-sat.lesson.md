---
id: ky-nghe-phan-mem.van-hanh.tong-hop-mot-vong-quan-sat
title: "Tổng hợp một VÒNG quan sát — từ metric ĐẾN quyết định"
summary: "vongQuanSat(tyLeLoi, nguong): string — MỘT chuỗi hàm thuần nối tiếp: so metric VỚI ngưỡng, NẾU vượt thì tra loại cảnh báo, RỒI runbook (bài 22) trả VỀ hành động — vongQuanSat KHÔNG tự viết thông điệp riêng, nó ỦY THÁC hoàn toàn cho layHanhDong (tái dùng thật, không sao chép logic)."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 23
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.observability-feedback-loop]
requires: [vh.runbook-alert-to-action]
concepts: [vh.observability-feedback-loop]
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
`layHanhDong` (bài 22) cần MỘT `LoaiCanhBao` LÀM đầu vào. NHƯNG
metric (tỷ lệ lỗi) LÀ một CON SỐ — làm SAO đi TỪ con số ĐẾN đúng
hành động?
::::

::::explain{#vong-quan-sat}
**Vòng quan sát** ĐẦY ĐỦ: metric → SO ngưỡng → NẾU vượt, tra LOẠI
cảnh báo → runbook TRẢ hành động — MỘT chuỗi HÀM THUẦN nối TIẾP,
GIỐNG pipeline (T5.4 bài 17) NHƯNG cho DỮ LIỆU vận hành:

```typescript title=readonly
type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";
function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao": return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong": return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao": return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: { const _kiemTraDayDu: never = loai; return _kiemTraDayDu; }
  }
}

function vongQuanSat(tyLeLoi: number, nguong: number): string {
  if (tyLeLoi > nguong) {
    return layHanhDong("ty_le_loi_cao");
  }
  return "Binh thuong, khong can hanh dong";
}

console.log(vongQuanSat(0.15, 0.1));
console.log(vongQuanSat(0.03, 0.1));
```

```text title=readonly
Kiem tra log loi gan nhat, xac dinh deploy nao gay ra
Binh thuong, khong can hanh dong
```

`0.15 > 0.1` (VƯỢT) → gọi `layHanhDong("ty_le_loi_cao")`, trả VỀ
hành động CỤ THỂ (bài 22). `0.03 > 0.1` LÀ `false` → KHÔNG cần
hành động, chỉ MỘT thông báo NGẮN "bình thường". `vongQuanSat`
**KHÔNG** tự VIẾT thông điệp CHO nhánh vượt — nó ỦY THÁC toàn BỘ
cho `layHanhDong`, TÁI DÙNG THẬT (KHÔNG sao CHÉP logic).
::::

::::example{#dung-bang-nguong-van-binh-thuong}
NGƯỠNG (`nguong`) VẪN tuân theo kỷ luật "vượt QUÁ MỚI cảnh báo"
(`>`, KHÔNG `>=`) — đúng bằng ngưỡng CHƯA gọi LÀ vượt:

```typescript title=readonly
type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";
function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao": return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong": return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao": return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: { const _kiemTraDayDu: never = loai; return _kiemTraDayDu; }
  }
}
function vongQuanSat(tyLeLoi: number, nguong: number): string {
  if (tyLeLoi > nguong) return layHanhDong("ty_le_loi_cao");
  return "Binh thuong, khong can hanh dong";
}
console.log(vongQuanSat(0.1, 0.1));
```

```text title=readonly
Binh thuong, khong can hanh dong
```

`tyLeLoi` (`0.1`) ĐÚNG BẰNG `nguong` (`0.1`) — `0.1 > 0.1` LÀ
`false` → "bình thường", KHÔNG gọi `layHanhDong`.
::::

::::predict{#doan-vong-quan-sat-uy-thac-that commitOnce}
```typescript
type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";
function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao": return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong": return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao": return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: { const _kiemTraDayDu: never = loai; return _kiemTraDayDu; }
  }
}
function vongQuanSat(tyLeLoi: number, nguong: number): string {
  if (tyLeLoi > nguong) return layHanhDong("ty_le_loi_cao");
  return "Binh thuong, khong can hanh dong";
}
console.log(vongQuanSat(0.5, 0.1));
```

`0.5` VƯỢT xa ngưỡng `0.1`. Dòng cuối in ra CHUỖI nào?

:::opt{correct}
`Kiem tra log loi gan nhat, xac dinh deploy nao gay ra`
:::

:::opt
`Vuot nguong!` — vì `vongQuanSat` LÀ hàm CẤP CAO hơn, chịu trách
NHIỆM tóm tắt tình HUỐNG NGẮN GỌN cho người vận hành LIẾC nhanh,
NÊN nó có THÔNG ĐIỆP RIÊNG (ngắn HƠN) THAY VÌ lặp LẠI câu dài của
`layHanhDong`
::why
Gần đúng ở việc bạn nghĩ TỚI một thiết kế UX hợp LÝ (thông điệp
NGẮN GỌN Ở tầng CAO, chi TIẾT Ở tầng THẤP hơn) — MỘT mẫu THIẾT KẾ
THẬT SỰ tồn TẠI trong nhiều hệ thống.

Chỗ lệch: NHÌN LẠI nhánh `if` — CHỈ CÓ ĐÚNG MỘT dòng
`return layHanhDong("ty_le_loi_cao")`. `vongQuanSat` KHÔNG hề CÓ
thông điệp RIÊNG cho trường hợp VƯỢT — nó GỌI THẲNG `layHanhDong`
VÀ trả VỀ **CHÍNH XÁC** giá trị ĐÓ, KHÔNG BỌC thêm, KHÔNG rút GỌN.
Đây LÀ Ý đồ THIẾT KẾ: `vongQuanSat` LÀ MỘT lớp ĐIỀU PHỐI mỏng
(orchestration), TOÀN BỘ nội DUNG hành động THẬT thuộc VỀ
`layHanhDong` — sửa NỘI DUNG hành động Ở MỘT nơi DUY NHẤT (bài 22),
`vongQuanSat` TỰ ĐỘNG dùng bản MỚI, KHÔNG cần sửa Ở ĐÂY.
::
:::

:::opt
Máy báo lỗi biên dịch — `vongQuanSat` khai kiểu trả về `string`,
NHƯNG nhánh `if` gọi `layHanhDong("ty_le_loi_cao")` (CŨNG trả
`string`) TRONG khi nhánh CÒN LẠI trả MỘT chuỗi CHỮ trực tiếp,
TypeScript CẤM MỘT hàm trả VỀ `string` từ HAI NGUỒN khác nhau
::why
Gần đúng ở việc bạn để ý HAI nhánh trả VỀ chuỗi theo HAI CÁCH khác
nhau (MỘT qua lời gọi hàm, MỘT trực TIẾP) — MỘT quan sát ĐÚNG về
CẤU TRÚC code.

Chỗ lệch: TypeScript KHÔNG hề quan TÂM "NGUỒN GỐC" của giá trị trả
về — CHỈ quan TÂM **KIỂU**. Cả `layHanhDong(...)` (trả `string`)
LẪN chuỗi chữ trực TIẾP (`"Binh thuong, ..."`, CŨNG kiểu `string`)
ĐỀU khớp kiểu trả về ĐÃ khai. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_vong_quan_sat}
Hoàn thiện `vongQuanSat` — so ngưỡng, RỒI ỦY THÁC hành động CHO
`layHanhDong`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";
function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao": return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong": return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao": return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: { const _kiemTraDayDu: never = loai; return _kiemTraDayDu; }
  }
}

function vongQuanSat(tyLeLoi: number, nguong: number): string {
  if (___) {
    return ___;
  }
  return "Binh thuong, khong can hanh dong";
}

assertEqual(
  vongQuanSat(0.03, 0.1),
  "Binh thuong, khong can hanh dong",
  "duoi nguong -- binh thuong",
);
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";
function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao": return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong": return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao": return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: { const _kiemTraDayDu: never = loai; return _kiemTraDayDu; }
  }
}

function vongQuanSat(tyLeLoi: number, nguong: number): string {
  if (tyLeLoi > nguong) {
    return layHanhDong("ty_le_loi_cao");
  }
  return "Binh thuong, khong can hanh dong";
}

assertEqual(
  vongQuanSat(0.03, 0.1),
  "Binh thuong, khong can hanh dong",
  "duoi nguong -- binh thuong",
);
```

```typescript title=test
assertEqual(
  vongQuanSat(0.15, 0.1),
  "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra",
  "vuot nguong -- hanh dong dung",
);
assertEqual(
  vongQuanSat(0.1, 0.1),
  "Binh thuong, khong can hanh dong",
  "dung bang nguong -- CHUA vuot",
);
```

:::hints
- kind: attention
  body: "Dieu kien: tyLeLoi > nguong. Neu dung, GOI layHanhDong(\"ty_le_loi_cao\") -- dung tu viet chuoi rieng."
- kind: strategy
  body: "tyLeLoi > nguong : layHanhDong(\"ty_le_loi_cao\")"
- kind: one-line
  body: '___ (dieu kien) = tyLeLoi > nguong\n___ (hanh dong) = layHanhDong("ty_le_loi_cao")'
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
Vòng quan sát nối metric đến quyết định. Bài BOSS: ghép TOÀN BỘ sáu
cụm — CI/CD, logging, metrics, tracing, alerting — thành MỘT hệ
vận hành nhỏ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`vongQuanSat` ghép BA bước (metric → ngưỡng → runbook) THÀNH một
chuỗi. Ghép TOÀN BỘ sáu CỤM của track này (CI/CD, logging, metrics,
tracing, alerting) thành MỘT hệ vận hành trông NHƯ thế nào?
::::

::::checkpoint{mastery=0.8}
::::
