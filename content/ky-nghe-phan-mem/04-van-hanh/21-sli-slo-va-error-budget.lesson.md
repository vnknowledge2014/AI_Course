---
id: ky-nghe-phan-mem.van-hanh.sli-slo-va-error-budget
title: "SLI/SLO — chỉ số ĐO ĐƯỢC vs mục tiêu CAM KẾT, error budget"
summary: "SLI (đo được thật) so với SLO (mục tiêu cam kết, tính bằng ĐIỂM PHẦN TRĂM để tránh sai số dấu phẩy động). tinhErrorBudget(sliHienTai, slo) = sliHienTai - slo — DƯƠNG nghĩa CÒN DƯ, ÂM nghĩa ĐÃ VI PHẠM, KHÔNG kẹp về 0 (số âm LÀ tín hiệu cảnh báo có ý nghĩa, không phải lỗi cần che giấu)."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 21
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [vh.sli-slo-error-budget]
requires: [vh.gate-boss-tracing-health]
concepts: [vh.sli-slo-error-budget]
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
`taoBaoCaoRequest` (bài 20) mô tả trạng thái HIỆN TẠI. NHƯNG "hệ
thống ĐANG đạt mục TIÊU cam kết VỚI người dùng hay CHƯA" LÀ câu hỏi
khác.
::::

::::explain{#sli-slo-error-budget}
**SLI** (Service Level Indicator) = con số ĐO ĐƯỢC THẬT (VÍ DỤ: bao
nhiêu ĐIỂM PHẦN TRĂM request THÀNH CÔNG). **SLO** (Service Level
Objective) = mục tiêu CAM KẾT cho SLI ĐÓ. **Error budget** = "ngân
sách LỖI" CÒN LẠI trước khi VI PHẠM cam kết:

```typescript title=readonly
function tinhErrorBudget(sliHienTai: number, slo: number): number {
  return sliHienTai - slo;
}

console.log(tinhErrorBudget(99, 95));
console.log(tinhErrorBudget(93, 95));
```

```text title=readonly
4
-2
```

SLI hiện TẠI LÀ `99` (điểm PHẦN TRĂM), SLO cam KẾT LÀ `95` → budget
`4` (**DƯƠNG**: hệ thống ĐANG làm TỐT HƠN cam kết `4` điểm, CÒN
"dư"). Ở dòng HAI, SLI TỤT xuống `93` (DƯỚI SLO `95`) → budget
`-2` (**ÂM**: ĐÃ vi phạm cam kết `2` điểm). Chú Ý: `tinhErrorBudget`
dùng ĐIỂM PHẦN TRĂM (`99`, KHÔNG PHẢI `0.99`) — TRÁNH sai số dấu
PHẨY động khi TRỪ hai số thập PHÂN nhỏ.
::::

::::example{#budget-am-la-tin-hieu-that}
Budget **ÂM** KHÔNG PHẢI lỗi CẦN che giấu — nó LÀ tín hiệu Ý NGHĨA:
"đã vi phạm cam kết BAO NHIÊU":

```typescript title=readonly
function tinhErrorBudget(sliHienTai: number, slo: number): number {
  return sliHienTai - slo;
}
console.log(tinhErrorBudget(95, 95));
console.log(tinhErrorBudget(100, 90));
```

```text title=readonly
0
10
```

`tinhErrorBudget(95, 95)` LÀ `0` — SLI ĐÚNG BẰNG SLO, budget CẠN
KIỆT (KHÔNG còn "dư" NHƯNG CŨNG CHƯA vi phạm). `tinhErrorBudget(100,
90)` LÀ `10` — hệ thống hoạt động HOÀN HẢO (`100`), CAM kết CHỈ
`90` → dư TỚI `10` điểm.
::::

::::predict{#doan-budget-am-khong-bi-kep-ve-0 commitOnce}
```typescript
function tinhErrorBudget(sliHienTai: number, slo: number): number {
  return sliHienTai - slo;
}
console.log(tinhErrorBudget(93, 95));
```

SLI hiện tại (`93`) THẤP HƠN SLO cam kết (`95`). Dòng cuối in ra gì?

:::opt{correct}
`-2`
:::

:::opt
`0` — vì "ngân SÁCH" (budget) theo NGHĨA thông THƯỜNG KHÔNG THỂ ÂM
(GIỐNG ngân sách tiền BẠC — tiêu HẾT thì DỪNG Ở `0`, KHÔNG "nợ ÂM"),
NÊN `tinhErrorBudget` PHẢI kẹp kết quả VỀ `0` khi SLI TỤT dưới SLO
::why
Gần đúng ở việc bạn LIÊN TƯỞNG đúng TỚI Ý NGHĨA thông THƯỜNG của từ
"ngân sách" (BUDGET tiền BẠC quả THỰC thường KHÔNG âm) — MỘT phép
loại suy NGÔN NGỮ hợp lý.

Chỗ lệch: NHÌN LẠI `tinhErrorBudget` — chỉ CÓ ĐÚNG MỘT dòng
`return sliHienTai - slo`, KHÔNG CÓ `Math.max(0, ...)` HAY BẤT KỲ
bước "kẹp" (clamp) NÀO. Đây LÀ lựa CHỌN THIẾT KẾ CÓ CHỦ Ý: budget
ÂM (`-2`) MANG THÔNG TIN QUAN TRỌNG HƠN `0` — nó nói RÕ "đã vi phạm
BAO NHIÊU điểm", GIÚP người vận hành ƯỚC LƯỢNG mức độ NGHIÊM TRỌNG.
KẸP về `0` sẽ XOÁ MẤT thông tin ĐÓ, biến "vi phạm NHẸ" VÀ "vi phạm
NẶNG" thành MỘT con số GIỐNG hệt nhau.
::
:::

:::opt
Máy báo lỗi biên dịch — `tinhErrorBudget` khai kiểu trả về
`number`, NHƯNG kết quả `93 - 95` LÀ số ÂM, TypeScript CẤM hàm khai
`number` trả VỀ giá trị ÂM TRỪ KHI khai RÕ RÀNG kiểu `number` CÓ
DẤU
::why
Gần đúng ở việc bạn để ý kết quả PHÉP tính NÀY LÀ số ÂM — MỘT quan
sát ĐÚNG về GIÁ TRỊ.

Chỗ lệch: TypeScript KHÔNG hề PHÂN biệt "number CÓ dấu" VÀ "number
KHÔNG dấu" — kiểu `number` BAO GỒM MỌI số thực (ÂM, DƯƠNG, `0`),
KHÔNG CÓ biến THỂ nào bị GIỚI hạn dấu. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_tinh_error_budget}
Hoàn thiện `tinhErrorBudget` — TRỪ trực tiếp, KHÔNG kẹp về `0`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function tinhErrorBudget(sliHienTai: number, slo: number): number {
  return ___;
}

assertEqual(tinhErrorBudget(99, 95), 4, "con du 4 diem phan tram");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function tinhErrorBudget(sliHienTai: number, slo: number): number {
  return sliHienTai - slo;
}

assertEqual(tinhErrorBudget(99, 95), 4, "con du 4 diem phan tram");
```

```typescript title=test
assertEqual(tinhErrorBudget(93, 95), -2, "vi pham 2 diem phan tram");
assertEqual(tinhErrorBudget(95, 95), 0, "dung bang SLO -- budget dung 0");
assertEqual(tinhErrorBudget(100, 90), 10, "con du nhieu");
```

:::hints
- kind: attention
  body: "Chi mot phep tru don gian: sliHienTai - slo. KHONG dung Math.max de kep ve 0."
- kind: strategy
  body: "sliHienTai - slo"
- kind: one-line
  body: '___ = sliHienTai - slo'
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
Error budget âm là tín hiệu thật, không phải lỗi. Bài tiếp theo:
khi cảnh báo kích hoạt, ai làm GÌ tiếp theo?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinhErrorBudget` cho biết ĐÃ vi phạm cam kết hay CHƯA. NHƯNG BIẾT
"đã vi phạm" KHÔNG TỰ nó giải QUYẾT gì — cần GÌ để biến MỘT cảnh
báo thành MỘT hành động CỤ THỂ?
::::

::::checkpoint{mastery=0.8}
::::
