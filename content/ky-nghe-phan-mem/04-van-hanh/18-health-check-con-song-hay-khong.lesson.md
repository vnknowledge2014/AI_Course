---
id: ky-nghe-phan-mem.van-hanh.health-check-con-song-hay-khong
title: "Health check — endpoint kiểm tra \"CÒN SỐNG\" hay không"
summary: "kiemTraSucKhoe(cacDependency): \"khoe\" | \"om\" — service CHỈ \"khoe\" khi MỌI dependency ĐỀU khoe (.every(), tái dùng kỷ luật T5.4 bài 18). MỘT dependency hỏng LÀ ĐỦ để báo \"om\", dù các dependency khác vẫn ổn — VÀ danh sách RỖNG (không dependency nào) trả \"khoe\" theo lẽ thật rỗng (vacuous truth)."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 18
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [vh.health-check-dependencies]
requires: [vh.tracing-spans]
concepts: [vh.health-check-dependencies]
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
Trace (bài 17) LỘ ra bước chậm. NHƯNG câu hỏi ĐƠN GIẢN nhất VẪN
CHƯA trả lời: service NÀY CÒN sống HAY đã HỎNG hoàn toàn?
::::

::::explain{#health-check}
**Health check**: MỘT hàm trả VỀ `"khoe" | "om"`, thường kiểm TẤT
CẢ dependency QUAN TRỌNG (database, cache, queue) CÙNG lúc:

```typescript title=readonly
type Dependency = { ten: string; khoe: boolean };

function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return cacDependency.every((d) => d.khoe) ? "khoe" : "om";
}

console.log(kiemTraSucKhoe([{ ten: "database", khoe: true }, { ten: "cache", khoe: true }]));
console.log(kiemTraSucKhoe([{ ten: "database", khoe: true }, { ten: "cache", khoe: false }]));
```

```text title=readonly
khoe
om
```

`.every()` (đã học T5.4 bài 18) trả `true` CHỈ khi **TẤT CẢ** phần
tử THOẢ điều kiện — service CHỈ `"khoe"` khi MỌI dependency ĐỀU
`khoe: true`. `database` khoe NHƯNG `cache` hỏng Ở ví dụ HAI → toàn
CỤC LÀ `"om"`, DÙ `database` VẪN ổn.
::::

::::example{#mot-hong-la-du}
VỊ trí dependency HỎNG trong mảng KHÔNG quan trọng — CHỈ CẦN MỘT
CÁI hỏng LÀ ĐỦ để toàn cục báo `"om"`:

```typescript title=readonly
type Dependency = { ten: string; khoe: boolean };
function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return cacDependency.every((d) => d.khoe) ? "khoe" : "om";
}
console.log(kiemTraSucKhoe([
  { ten: "cache", khoe: false },
  { ten: "database", khoe: true },
  { ten: "queue", khoe: true },
]));
```

```text title=readonly
om
```

LẦN NÀY dependency hỏng (`cache`) đứng **ĐẦU** mảng (KHÔNG PHẢI
cuối như ví dụ TRƯỚC) — kết quả VẪN LÀ `"om"`. `database` VÀ
`queue` ổn KHÔNG "CỨU" được TOÀN cục — MỘT mắt xích hỏng LÀ ĐỦ.
::::

::::predict{#doan-danh-sach-rong commitOnce}
```typescript
type Dependency = { ten: string; khoe: boolean };
function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return cacDependency.every((d) => d.khoe) ? "khoe" : "om";
}
console.log(kiemTraSucKhoe([]));
```

MỘT service KHÔNG khai BẤT KỲ dependency NÀO (mảng `[]`, RỖNG hoàn
TOÀN). Dòng cuối in ra gì?

:::opt{correct}
`"khoe"`
:::

:::opt
`"om"` — vì MỘT service KHÔNG khai dependency nào NGHĨA LÀ health
check "KHÔNG CÓ GÌ để kiểm TRA", VÀ KHI không CÓ dữ liệu để kết
LUẬN, quy ước AN TOÀN LÀ báo TRẠNG THÁI xấu nhất (`"om"`) — "thà
báo hỏng NHẦM còn HƠN báo khoe NHẦM"
::why
Gần đúng ở việc bạn nghĩ TỚI nguyên tắc "AN TOÀN LÀ TRÊN HẾT" — MỘT
trực giác hợp LÝ trong vận HÀNH THẬT (nhiều hệ thống CỐ Ý thiên VỀ
báo động GIẢ hơn BỎ SÓT).

Chỗ lệch: `.every()` KHÔNG áp DỤNG nguyên tắc "AN TOÀN" NÀO — nó
tuân THEO định nghĩa TOÁN HỌC của lượng từ "MỌI" (∀) TRÊN một tập
RỖNG: MỆNH ĐỀ "MỌI phần tử của TẬP RỖNG thoả điều KIỆN X" LUÔN
**ĐÚNG** (KHÔNG CÓ phần tử NÀO để làm nó SAI — gọi LÀ "lẽ thật
rỗng", vacuous truth). VÌ VẬY `[].every(...)` LUÔN trả `true`,
`kiemTraSucKhoe([])` trả `"khoe"`. Đây LÀ hành vi CHUẨN của MỌI
ngôn NGỮ implement `.every`/`all` — KHÔNG PHẢI lỗi.
::
:::

:::opt
Máy báo lỗi biên dịch — `kiemTraSucKhoe` khai kiểu trả về
`"khoe" | "om"`, NHƯNG gọi TRÊN mảng `[]` (RỖNG, KHÔNG rõ kiểu phần
tử TẠI lời gọi), TypeScript CẤM suy luận kiểu trả VỀ cho đầu VÀO
rỗng
::why
Gần đúng ở việc bạn để ý `[]` KHÔNG chứa phần TỬ nào để "gợi ý"
kiểu — MỘT quan sát HỢP LÝ về CÚ PHÁP TypeScript nói CHUNG.

Chỗ lệch: tham số `cacDependency: Dependency[]` ĐÃ khai kiểu RÕ
RÀNG Ở CHỮ KÝ hàm — `[]` (mảng RỖNG) khớp VỚI `Dependency[]` HOÀN
HẢO (mảng RỖNG LÀ giá trị hợp LỆ của BẤT KỲ kiểu mảng NÀO). Biên
dịch SẠCH.
::
:::
::::

::::code{#viet_kiem_tra_suc_khoe}
Hoàn thiện `kiemTraSucKhoe` — dùng `.every()`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Dependency = { ten: string; khoe: boolean };

function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return ___;
}

assertEqual(kiemTraSucKhoe([{ ten: "db", khoe: true }]), "khoe", "mot dependency khoe");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Dependency = { ten: string; khoe: boolean };

function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return cacDependency.every((d) => d.khoe) ? "khoe" : "om";
}

assertEqual(kiemTraSucKhoe([{ ten: "db", khoe: true }]), "khoe", "mot dependency khoe");
```

```typescript title=test
assertEqual(kiemTraSucKhoe([{ ten: "db", khoe: false }]), "om", "mot dependency om");
assertEqual(
  kiemTraSucKhoe([{ ten: "db", khoe: true }, { ten: "cache", khoe: true }, { ten: "queue", khoe: true }]),
  "khoe",
  "ba dependency deu khoe",
);
assertEqual(
  kiemTraSucKhoe([{ ten: "db", khoe: true }, { ten: "cache", khoe: false }, { ten: "queue", khoe: true }]),
  "om",
  "mot trong ba hong -- om",
);
assertEqual(kiemTraSucKhoe([]), "khoe", "khong dependency nao -- khoe vacuously");
```

:::hints
- kind: attention
  body: "Dung cacDependency.every, kiem tra tung d.khoe. Neu MOI phan tu deu khoe thi tra \"khoe\", khong thi \"om\"."
- kind: strategy
  body: "cacDependency.every((d) => d.khoe) ? \"khoe\" : \"om\""
- kind: one-line
  body: '___ = cacDependency.every((d) => d.khoe) ? "khoe" : "om"'
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
Health check = một dependency hỏng đủ để báo "om". Bài tiếp theo: khi
metric vượt ngưỡng, làm sao SINH RA một cảnh báo có cấu trúc?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`kiemTraSucKhoe` trả `"om"` khi CÓ dependency hỏng. NHƯNG chỉ BIẾT
"om" KHÔNG đủ — CẦN MỘT cơ chế nào ĐÓ chủ động BÁO cho người vận
hành, KHÔNG PHẢI chờ họ TỰ hỏi?
::::

::::checkpoint{mastery=0.8}
::::
