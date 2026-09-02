---
id: ky-nghe-phan-mem.van-hanh.runbook-alert-den-hanh-dong
title: "Runbook — khi cảnh báo kích hoạt, PHẢN ỨNG có CẤU TRÚC"
summary: "LoaiCanhBao = \"ty_le_loi_cao\" | \"dependency_hong\" | \"do_tre_cao\" (Discriminated Union). layHanhDong(loai): string — exhaustive switch, MỖI loại cảnh báo ÁNH XẠ MỘT hành động RÕ RÀNG (KHÔNG hoảng loạn tuỳ hứng khi cảnh báo kích hoạt lúc nửa đêm)."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 22
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [vh.runbook-alert-to-action]
requires: [vh.sli-slo-error-budget]
concepts: [vh.runbook-alert-to-action]
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
`tinhErrorBudget` (bài 21) BÁO "đã vi phạm". NHƯNG biết "đã vi
phạm" KHÔNG tự nó GIẢI QUYẾT gì — người trực CẦN biết làm GÌ tiếp
theo, NGAY LẬP TỨC.
::::

::::explain{#runbook}
**Runbook** = ÁNH XẠ từ LOẠI cảnh báo SANG hành động CỤ THỂ (KHÔNG
PHẢI hoảng loạn TUỲ HỨNG lúc nửa ĐÊM). `LoaiCanhBao` LÀ Discriminated
Union HẸP (đã học T4.3), `layHanhDong` LÀ exhaustive `switch`:

```typescript title=readonly
type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";

function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao":
      return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong":
      return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao":
      return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: {
      const _kiemTraDayDu: never = loai;
      return _kiemTraDayDu;
    }
  }
}

console.log(layHanhDong("ty_le_loi_cao"));
console.log(layHanhDong("dependency_hong"));
```

```text title=readonly
Kiem tra log loi gan nhat, xac dinh deploy nao gay ra
Kiem tra dependency, xem xet failover hoac restart
```

MỖI NHÁNH `case` trả VỀ MỘT hành động RÕ RÀNG, CỤ THỂ (KHÔNG PHẢI
"kiểm tra HỆ THỐNG" mơ HỒ). Nhánh `default` (`_kiemTraDayDu: never`)
LÀ kỹ thuật ĐÃ học (T4.3): NẾU thêm MỘT loại cảnh báo MỚI VÀO
`LoaiCanhBao` MÀ QUÊN thêm `case` tương ỨNG, TypeScript báo lỗi
BIÊN dịch NGAY, KHÔNG đợi tới lúc CHẠY mới phát hiện thiếu sót.
::::

::::example{#loai-thu-ba}
Loại cảnh BÁO thứ BA (`"do_tre_cao"`, liên QUAN trực TIẾP tới
`timBuocChamNhat` bài 17) CŨNG có hành động RIÊNG:

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
console.log(layHanhDong("do_tre_cao"));
```

```text title=readonly
Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau
```

Hành động Ở ĐÂY TRỎ THẲNG VỀ `timBuocChamNhat` (bài 17) — runbook
KHÔNG PHẢI lý THUYẾT SUÔNG, nó NỐI trực TIẾP VỀ công cụ ĐÃ xây Ở
CÁC bài TRƯỚC.
::::

::::predict{#doan-anh-xa-dung-loai commitOnce}
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
console.log(layHanhDong("dependency_hong"));
```

Dòng cuối in ra CHUỖI nào?

:::opt{correct}
`Kiem tra dependency, xem xet failover hoac restart`
:::

:::opt
`Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau` — vì
`"dependency_hong"` (dependency HỎNG) THƯỜNG khiến request CHẬM
BẤT THƯỜNG (do TIMEOUT khi CHỜ dependency phản HỒI), NÊN hành động
HỢP LÝ nhất LÀ kiểm tra trace để tìm bước NGHẼN
::why
Gần đúng ở việc bạn LIÊN TƯỞNG đúng: dependency HỎNG THẬT SỰ hay
gây RA độ trễ CAO (một quan HỆ NHÂN QUẢ hợp lý trong THỰC TẾ vận
hành).

Chỗ lệch: `layHanhDong` ÁNH XẠ theo **LOẠI cảnh báo ĐƯỢC TRUYỀN
VÀO**, KHÔNG PHẢI theo "NGUYÊN NHÂN gốc RỄ suy đoán". Tham SỐ TRUYỀN
VÀO LÀ `"dependency_hong"` — khớp ĐÚNG NHÁNH `case "dependency_hong"`,
trả VỀ CHÍNH XÁC hành động của NHÁNH ĐÓ (`"Kiem tra dependency,
..."`), KHÔNG PHẢI nhánh `"do_tre_cao"`. `switch` so khớp GIÁ TRỊ
CHÍNH XÁC, KHÔNG suy luận MỐI QUAN HỆ nhân quả GIỮA các loại.
::
:::

:::opt
Máy báo lỗi biên dịch — nhánh `default` khai `const _kiemTraDayDu:
never = loai`, NHƯNG `loai` CÓ kiểu `LoaiCanhBao` (KHÔNG PHẢI
`never`), TypeScript CẤM gán MỘT biến kiểu `LoaiCanhBao` cho biến
kiểu `never`
::why
Gần đúng ở việc bạn để ý `never` LÀ MỘT kiểu ĐẶC BIỆT, KHÁC hẳn
`LoaiCanhBao` — MỘT quan sát ĐÚNG về TÊN kiểu.

Chỗ lệch: ĐÂY chính LÀ ĐIỂM MẤU CHỐT của kỹ thuật exhaustiveness
check (T4.3) — VÌ `switch` ĐÃ xử LÝ ĐỦ CẢ BA nhánh `case` (KHỚP hết
BA thành viên của `LoaiCanhBao`), TypeScript TỰ THU HẸP kiểu của
`loai` TẠI nhánh `default` XUỐNG CÒN `never` (nghĩa LÀ "KHÔNG CÒN
giá trị NÀO CÓ THỂ tới ĐÂY"). Gán `loai` (kiểu `never` TẠI điểm
NÀY) cho biến `never` LÀ hợp LỆ. Biên dịch SẠCH — VÀ nếu SAU NÀY
thêm MỘT thành viên MỚI vào `LoaiCanhBao` mà QUÊN case, dòng NÀY
MỚI báo lỗi (đúng NHƯ Ý đồ thiết kế).
::
:::
::::

::::code{#viet_lay_hanh_dong}
Hoàn thiện `layHanhDong` — MỖI loại cảnh báo trả VỀ ĐÚNG hành động
tương ỨNG.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LoaiCanhBao = "ty_le_loi_cao" | "dependency_hong" | "do_tre_cao";

function layHanhDong(loai: LoaiCanhBao): string {
  switch (loai) {
    case "ty_le_loi_cao":
      return ___;
    case "dependency_hong":
      return ___;
    case "do_tre_cao":
      return ___;
    default: {
      const _kiemTraDayDu: never = loai;
      return _kiemTraDayDu;
    }
  }
}

assertEqual(
  layHanhDong("ty_le_loi_cao"),
  "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra",
  "hanh dong ty le loi cao",
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
    case "ty_le_loi_cao":
      return "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra";
    case "dependency_hong":
      return "Kiem tra dependency, xem xet failover hoac restart";
    case "do_tre_cao":
      return "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau";
    default: {
      const _kiemTraDayDu: never = loai;
      return _kiemTraDayDu;
    }
  }
}

assertEqual(
  layHanhDong("ty_le_loi_cao"),
  "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra",
  "hanh dong ty le loi cao",
);
```

```typescript title=test
assertEqual(
  layHanhDong("dependency_hong"),
  "Kiem tra dependency, xem xet failover hoac restart",
  "hanh dong dependency hong",
);
assertEqual(
  layHanhDong("do_tre_cao"),
  "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau",
  "hanh dong do tre cao",
);
```

:::hints
- kind: attention
  body: "Moi case tra ve DUNG chuoi hanh dong tuong ung -- doc lai phan explain de khop dung tung loai."
- kind: strategy
  body: "\"Kiem tra log loi...\" : \"Kiem tra dependency...\" : \"Kiem tra buoc cham nhat...\""
- kind: one-line
  body: 'case "ty_le_loi_cao": tra ve "Kiem tra log loi gan nhat, xac dinh deploy nao gay ra"\ncase "dependency_hong": tra ve "Kiem tra dependency, xem xet failover hoac restart"\ncase "do_tre_cao": tra ve "Kiem tra buoc cham nhat qua trace, xac dinh nghen o dau"'
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
Runbook nối cảnh báo với hành động cụ thể. Bài tiếp theo: ghép TOÀN
BỘ chuỗi — từ metric tới quyết định — thành MỘT vòng quan sát.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`layHanhDong` cần MỘT `LoaiCanhBao` LÀM đầu VÀO. NHƯNG metric (tỷ
lệ lỗi, độ trễ) LÀ những CON SỐ — LÀM SAO đi TỪ con số ĐẾN đúng
loại cảnh báo?
::::

::::checkpoint{mastery=0.8}
::::
