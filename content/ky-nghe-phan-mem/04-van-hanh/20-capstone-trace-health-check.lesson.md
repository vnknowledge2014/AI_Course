---
id: ky-nghe-phan-mem.van-hanh.capstone-trace-health-check
title: "Capstone cụm — ghép Trace + Health Check thành báo cáo request"
summary: "taoBaoCaoRequest(cacSpan, cacDependency): {buocChamNhat, tongThoiGian, tinhTrang} — ghép timBuocChamNhat (bài 17) và kiemTraSucKhoe (bài 18) thành MỘT báo cáo cho MỘT request: bước nào chậm nhất, tổng thời gian, VÀ hệ thống có khoẻ không — hai phần ĐỘC LẬP, không ảnh hưởng lẫn nhau."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 20
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.gate-boss-tracing-health]
requires: [vh.alerting-threshold]
concepts: [vh.gate-boss-tracing-health]
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
Trace (bài 17) VÀ health check (bài 18) đứng RIÊNG. Bài chốt cụm:
gộp CẢ HAI thành MỘT báo cáo cho MỘT request.
::::

::::explain{#bao-cao-request}
`taoBaoCaoRequest` ghép `timBuocChamNhat` VÀ `kiemTraSucKhoe` THÀNH
MỘT báo cáo TỔNG HỢP:

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

type Dependency = { ten: string; khoe: boolean };
function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return cacDependency.every((d) => d.khoe) ? "khoe" : "om";
}

function taoBaoCaoRequest(cacSpan: Span[], cacDependency: Dependency[]): {
  buocChamNhat: string;
  tongThoiGian: number;
  tinhTrang: "khoe" | "om";
} {
  return {
    buocChamNhat: timBuocChamNhat(cacSpan),
    tongThoiGian: cacSpan.reduce((tong, s) => tong + tinhThoiGian(s), 0),
    tinhTrang: kiemTraSucKhoe(cacDependency),
  };
}

const cacSpan: Span[] = [
  { tenBuoc: "validate", batDau: 0, ketThuc: 5 },
  { tenBuoc: "truy-van", batDau: 5, ketThuc: 120 },
  { tenBuoc: "tinh-toan", batDau: 120, ketThuc: 135 },
];
console.log(taoBaoCaoRequest(cacSpan, [{ ten: "db", khoe: true }, { ten: "cache", khoe: true }]));
```

```text title=readonly
{"buocChamNhat":"truy-van","tongThoiGian":135,"tinhTrang":"khoe"}
```

Ba PHẦN của báo CÁO đến từ BA nguồn KHÁC nhau: `buocChamNhat` (bài
17), `tongThoiGian` (tổng thời GIAN của TỪNG span, dùng `.reduce`),
`tinhTrang` (bài 18) — GHÉP LẠI thành MỘT object DUY NHẤT.
::::

::::example{#hai-phan-doc-lap}
`buocChamNhat`/`tongThoiGian` (đến TỪ `cacSpan`) VÀ `tinhTrang`
(đến TỪ `cacDependency`) LÀ HAI mảnh dữ liệu HOÀN TOÀN TÁCH BIỆT —
đổi MỘT bên KHÔNG ảnh hưởng bên KIA:

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
type Dependency = { ten: string; khoe: boolean };
function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return cacDependency.every((d) => d.khoe) ? "khoe" : "om";
}
function taoBaoCaoRequest(cacSpan: Span[], cacDependency: Dependency[]): {
  buocChamNhat: string; tongThoiGian: number; tinhTrang: "khoe" | "om";
} {
  return {
    buocChamNhat: timBuocChamNhat(cacSpan),
    tongThoiGian: cacSpan.reduce((tong, s) => tong + tinhThoiGian(s), 0),
    tinhTrang: kiemTraSucKhoe(cacDependency),
  };
}
const cacSpan: Span[] = [
  { tenBuoc: "validate", batDau: 0, ketThuc: 5 },
  { tenBuoc: "truy-van", batDau: 5, ketThuc: 120 },
  { tenBuoc: "tinh-toan", batDau: 120, ketThuc: 135 },
];
console.log(taoBaoCaoRequest(cacSpan, [{ ten: "db", khoe: true }, { ten: "cache", khoe: false }]));
```

```text title=readonly
{"buocChamNhat":"truy-van","tongThoiGian":135,"tinhTrang":"om"}
```

CÙNG `cacSpan` NHƯ ví dụ TRƯỚC (`buocChamNhat`/`tongThoiGian` GIỮ
NGUYÊN), CHỈ `cacDependency` đổi (`cache` giờ HỎNG) → `tinhTrang`
đổi thành `"om"`. Đúng NHƯ dự kiến: HAI mảnh dữ liệu KHÔNG chạm tới
nhau.
::::

::::predict{#doan-hai-phan-doc-lap-nguoc-lai commitOnce}
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
type Dependency = { ten: string; khoe: boolean };
function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return cacDependency.every((d) => d.khoe) ? "khoe" : "om";
}
function taoBaoCaoRequest(cacSpan: Span[], cacDependency: Dependency[]): {
  buocChamNhat: string; tongThoiGian: number; tinhTrang: "khoe" | "om";
} {
  return {
    buocChamNhat: timBuocChamNhat(cacSpan),
    tongThoiGian: cacSpan.reduce((tong, s) => tong + tinhThoiGian(s), 0),
    tinhTrang: kiemTraSucKhoe(cacDependency),
  };
}
const chiMotBuoc: Span[] = [{ tenBuoc: "xu-ly-nhanh", batDau: 0, ketThuc: 3 }];
const capOm: Dependency[] = [{ ten: "db", khoe: true }, { ten: "cache", khoe: false }];
console.log(taoBaoCaoRequest(chiMotBuoc, capOm).buocChamNhat);
```

`cacDependency` (`capOm`) CÓ MỘT dependency HỎNG (`tinhTrang` SẼ LÀ
`"om"`). Dòng cuối in ra `buocChamNhat` — giá trị NÀO?

:::opt{correct}
`"xu-ly-nhanh"`
:::

:::opt
`undefined` — vì `kiemTraSucKhoe` phát hiện dependency HỎNG (`"om"`)
TRƯỚC KHI `taoBaoCaoRequest` kịp tính `buocChamNhat`, VÀ MỘT khi hệ
thống "om", các trường KHÁC của báo CÁO (như `buocChamNhat`) KHÔNG
còn Ý NGHĨA để tính NỮA nên bị bỏ TRỐNG
::why
Gần đúng ở việc bạn nghĩ TỚI hệ QUẢ hợp lý của "hệ thống om" —
NHIỀU hệ thống THẬT quả CÓ ngừng thu thập metric KHI phát hiện lỗi
nghiêm TRỌNG.

Chỗ lệch: NHÌN LẠI `taoBaoCaoRequest` — CẢ BA trường (`buocChamNhat`,
`tongThoiGian`, `tinhTrang`) được TÍNH **ĐỘC LẬP**, KHÔNG CÓ điều
kiện `if` NÀO liên kết CHÚNG. `buocChamNhat: timBuocChamNhat(cacSpan)`
CHỈ phụ THUỘC `cacSpan` — HOÀN TOÀN KHÔNG quan tâm `cacDependency`
"khoe" hay "om". VỚI `chiMotBuoc` CHỈ CÓ MỘT span DUY NHẤT
(`"xu-ly-nhanh"`), `timBuocChamNhat` trả VỀ CHÍNH nó — BẤT KỂ
`tinhTrang` LÀ GÌ.
::
:::

:::opt
Máy báo lỗi biên dịch — `cacDependency` CÓ MỘT phần TỬ `khoe: false`
(HỎNG), NHƯNG kiểu trả VỀ của `taoBaoCaoRequest` khai `tinhTrang:
"khoe" | "om"` KHÔNG cho PHÉP trạng thái "hỏng MỘT PHẦN" — TypeScript
YÊU CẦU TOÀN BỘ dependency PHẢI CÙNG trạng thái
::why
Gần đúng ở việc bạn để ý `cacDependency` chứa HỖN HỢP `khoe: true`
VÀ `khoe: false` — MỘT quan sát ĐÚNG về DỮ LIỆU đầu VÀO.

Chỗ lệch: `Dependency[]` (mảng) HOÀN TOÀN cho PHÉP các phần TỬ có
`khoe` KHÁC nhau — ĐÓ chính LÀ tình huống BÌNH THƯỜNG (một VÀI
dependency khoe, MỘT vài hỏng). `kiemTraSucKhoe` XỬ LÝ sự "hỗn hợp"
NÀY bằng `.every()`, trả VỀ ĐÚNG MỘT giá trị TỔNG hợp (`"khoe"` HAY
`"om"`). Biên dịch SẠCH.
::
:::
::::

::::code{#viet_tao_bao_cao_request}
Hoàn thiện `taoBaoCaoRequest` — ghép `timBuocChamNhat`, tổng thời
gian (`.reduce`), VÀ `kiemTraSucKhoe`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Span = { tenBuoc: string; batDau: number; ketThuc: number };
function tinhThoiGian(span: Span): number { return span.ketThuc - span.batDau; }
function timBuocChamNhat(cacSpan: Span[]): string {
  let baiChamNhat = cacSpan[0]!;
  for (const span of cacSpan) {
    if (tinhThoiGian(span) > tinhThoiGian(baiChamNhat)) baiChamNhat = span;
  }
  return baiChamNhat.tenBuoc;
}

type Dependency = { ten: string; khoe: boolean };
function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return cacDependency.every((d) => d.khoe) ? "khoe" : "om";
}

function taoBaoCaoRequest(cacSpan: Span[], cacDependency: Dependency[]): {
  buocChamNhat: string;
  tongThoiGian: number;
  tinhTrang: "khoe" | "om";
} {
  return {
    buocChamNhat: ___,
    tongThoiGian: cacSpan.reduce((tong, s) => tong + tinhThoiGian(s), 0),
    tinhTrang: ___,
  };
}

const cacSpan1: Span[] = [{ tenBuoc: "chi-mot", batDau: 0, ketThuc: 10 }];
assertEqual(taoBaoCaoRequest(cacSpan1, [{ ten: "db", khoe: true }]).buocChamNhat, "chi-mot", "buoc cham nhat dung");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Span = { tenBuoc: string; batDau: number; ketThuc: number };
function tinhThoiGian(span: Span): number { return span.ketThuc - span.batDau; }
function timBuocChamNhat(cacSpan: Span[]): string {
  let baiChamNhat = cacSpan[0]!;
  for (const span of cacSpan) {
    if (tinhThoiGian(span) > tinhThoiGian(baiChamNhat)) baiChamNhat = span;
  }
  return baiChamNhat.tenBuoc;
}

type Dependency = { ten: string; khoe: boolean };
function kiemTraSucKhoe(cacDependency: Dependency[]): "khoe" | "om" {
  return cacDependency.every((d) => d.khoe) ? "khoe" : "om";
}

function taoBaoCaoRequest(cacSpan: Span[], cacDependency: Dependency[]): {
  buocChamNhat: string;
  tongThoiGian: number;
  tinhTrang: "khoe" | "om";
} {
  return {
    buocChamNhat: timBuocChamNhat(cacSpan),
    tongThoiGian: cacSpan.reduce((tong, s) => tong + tinhThoiGian(s), 0),
    tinhTrang: kiemTraSucKhoe(cacDependency),
  };
}

const cacSpan1: Span[] = [{ tenBuoc: "chi-mot", batDau: 0, ketThuc: 10 }];
assertEqual(taoBaoCaoRequest(cacSpan1, [{ ten: "db", khoe: true }]).buocChamNhat, "chi-mot", "buoc cham nhat dung");
```

```typescript title=test
const cacSpan: Span[] = [
  { tenBuoc: "validate", batDau: 0, ketThuc: 5 },
  { tenBuoc: "truy-van", batDau: 5, ketThuc: 120 },
  { tenBuoc: "tinh-toan", batDau: 120, ketThuc: 135 },
];
const capsKhoe: Dependency[] = [{ ten: "db", khoe: true }, { ten: "cache", khoe: true }];
const capsOm: Dependency[] = [{ ten: "db", khoe: true }, { ten: "cache", khoe: false }];

assertEqual(taoBaoCaoRequest(cacSpan, capsKhoe).buocChamNhat, "truy-van", "buoc cham nhat trong ba buoc");
assertEqual(taoBaoCaoRequest(cacSpan, capsKhoe).tongThoiGian, 135, "tong thoi gian dung");
assertEqual(taoBaoCaoRequest(cacSpan, capsKhoe).tinhTrang, "khoe", "tinh trang khoe khi ca hai dependency on");
assertEqual(taoBaoCaoRequest(cacSpan, capsOm).tinhTrang, "om", "tinh trang om khi mot dependency hong");
assertEqual(taoBaoCaoRequest(cacSpan, capsOm).buocChamNhat, "truy-van", "buoc cham nhat khong doi du tinh trang om");
```

:::hints
- kind: attention
  body: "buocChamNhat goi timBuocChamNhat(cacSpan). tinhTrang goi kiemTraSucKhoe(cacDependency)."
- kind: strategy
  body: "timBuocChamNhat(cacSpan) : kiemTraSucKhoe(cacDependency)"
- kind: one-line
  body: '___ (buocChamNhat) = timBuocChamNhat(cacSpan)\n___ (tinhTrang) = kiemTraSucKhoe(cacDependency)'
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
Trace + health check = một báo cáo request đầy đủ. Cụm cuối cùng:
SLI/SLO, runbook, và BOSS ghép toàn bộ track thành một hệ vận hành.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`taoBaoCaoRequest` mô tả TRẠNG THÁI HIỆN TẠI. NHƯNG "hệ thống ĐANG
đạt mục TIÊU cam kết VỚI người dùng hay CHƯA" — cần MỘT khái niệm
khác để trả lời.
::::

::::checkpoint{mastery=0.8}
::::
