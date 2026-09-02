---
id: ky-nghe-phan-mem.van-hanh.moi-truong-staging-production
title: "Môi trường: CÙNG pipeline, KHÁC đích deploy"
summary: "MỘT pipeline (cụm 1) chạy GIỐNG HỆT cho MỌI nhánh — NHƯNG đích DEPLOY khác nhau theo TÊN nhánh: nhánh \"develop\" → môi trường \"staging\" (thử nghiệm), nhánh \"main\" → \"production\" (người dùng thật). xacDinhMoiTruong(tenNhanh): \"staging\"|\"production\"|\"khong_deploy\" — Discriminated Union hẹp, exhaustive switch, KHÔNG if/else chuỗi."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [vh.environment-staging-prod]
requires: [vh.conditional-deploy-gate]
concepts: [vh.environment-staging-prod]
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
`xetDuyetDeploy` (bài 5) CHỈ trả `true`/`false`. Deploy TỚI ĐÂU —
môi trường thử NGHIỆM hay người dùng THẬT — phụ thuộc CÁI GÌ?
::::

::::explain{#cung-pipeline-khac-dich}
MỘT pipeline (cụm 1) chạy GIỐNG HỆT cho MỌI nhánh — NHƯNG đích
**DEPLOY** khác NHAU theo TÊN nhánh: nhánh `"develop"` → môi trường
`"staging"` (thử NGHIỆM, rủi ro THẤP), nhánh `"main"` →
`"production"` (người DÙNG THẬT):

```typescript title=readonly
type MoiTruong = "staging" | "production" | "khong_deploy";

function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop":
      return "staging";
    case "main":
      return "production";
    default:
      return "khong_deploy";
  }
}

console.log(xacDinhMoiTruong("develop"));
console.log(xacDinhMoiTruong("main"));
console.log(xacDinhMoiTruong("feature/gio-hang"));
```

```text title=readonly
staging
production
khong_deploy
```

`MoiTruong` LÀ Discriminated Union HẸP (đã học T4.3) — exhaustive
`switch`, KHÔNG chuỗi `if/else` (nối TRỰC TIẾP T5.4 bài 19's kỷ
luật). NHÁNH `"feature/gio-hang"` (KHÔNG khớp `"develop"` hay
`"main"`) rơi vào `default` → `"khong_deploy"`.
::::

::::example{#moi-nhanh-mot-ket-qua-doc-lap}
`xacDinhMoiTruong` LÀ MỘT hàm THUẦN — áp dụng được cho MỘT MẢNG
nhánh qua `.map`, MỖI nhánh cho ra kết quả ĐỘC LẬP:

```typescript title=readonly
type MoiTruong = "staging" | "production" | "khong_deploy";
function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop": return "staging";
    case "main": return "production";
    default: return "khong_deploy";
  }
}

const cacNhanh = ["develop", "main", "hotfix/rollback", "develop"];
console.log(cacNhanh.map(xacDinhMoiTruong));
```

```text title=readonly
["staging","production","khong_deploy","staging"]
```

`"develop"` xuất hiện HAI LẦN (vị trí ĐẦU VÀ CUỐI) — CẢ HAI ĐỀU cho
`"staging"`, ĐÚNG tính chất HÀM THUẦN: CÙNG đầu vào, CÙNG đầu ra,
MỌI lần.
::::

::::predict{#doan-phan-biet-hoa-thuong commitOnce}
```typescript
type MoiTruong = "staging" | "production" | "khong_deploy";
function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop": return "staging";
    case "main": return "production";
    default: return "khong_deploy";
  }
}
console.log(xacDinhMoiTruong("Main"));
console.log(xacDinhMoiTruong("MAIN"));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`khong_deploy` rồi `khong_deploy`
:::

:::opt
`production` rồi `production` — vì `"Main"` VÀ `"MAIN"` đều LÀ CÙNG
MỘT tên nhánh CHỈ khác CÁCH VIẾT HOA/thường, VÀ Git THƯỜNG coi tên
nhánh LÀ KHÔNG phân biệt HOA/thường trên NHIỀU hệ điều hành
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"Main"`/`"MAIN"`/`"main"` "CÙNG Ý
nghĩa" ĐỐI VỚI người ĐỌC — quan sát ĐÓ, về mặt Ý ĐỊNH con người, hợp
lý (VÀ đúng LÀ TRÊN một số hệ thống FILE, tên KHÔNG phân biệt
hoa/thường).

Chỗ lệch: `switch (tenNhanh) { case "main": ... }` so sánh CHUỖI
BẰNG `===` — MỘT PHÉP SO SÁNH **PHÂN BIỆT HOA/THƯỜNG TUYỆT ĐỐI**
trong JavaScript/TypeScript (KHÔNG liên quan gì tới HỆ ĐIỀU HÀNH
hay Git). `"Main" === "main"` LÀ `false` (chữ CÁI đầu KHÁC nhau) —
CẢ `"Main"` LẪN `"MAIN"` ĐỀU KHÔNG khớp case `"main"`, rơi vào
`default` → `"khong_deploy"`.
::
:::

:::opt
Máy báo lỗi biên dịch — `xacDinhMoiTruong` khai tham số `tenNhanh:
string`, NHƯNG `"Main"`/`"MAIN"` LÀ hai CHUỖI viết HOA KHÁC với hai
case `"develop"`/`"main"` (viết THƯỜNG), TypeScript đòi tham số
TRUYỀN vào PHẢI khớp CÁCH VIẾT HOA/thường của MỘT case NÀO ĐÓ
::why
Gần đúng ở việc bạn để ý `"Main"`/`"MAIN"` viết HOA KHÁC với
`"develop"`/`"main"` (viết THƯỜNG) TRONG các case — một quan sát
ĐÚNG về CÚ PHÁP chuỗi.

Chỗ lệch: tham số khai `tenNhanh: string` chấp nhận **BẤT KỲ**
chuỗi NÀO — TypeScript KHÔNG kiểm tra "chuỗi TRUYỀN vào có KHỚP MỘT
case NÀO trong `switch` hay không" TẠI THỜI ĐIỂM biên dịch (đó LÀ
việc XẢY RA LÚC CHẠY). Biên dịch SẠCH — kết quả LÚC CHẠY đơn giản
LÀ rơi vào `default`.
::
:::
::::

::::code{#viet_xac_dinh_moi_truong}
Hoàn thiện `xacDinhMoiTruong` — `switch` trên tên nhánh, MỖI nhánh
ĐI TỚI đúng môi trường.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type MoiTruong = "staging" | "production" | "khong_deploy";

function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop":
      return ___;
    case "main":
      return ___;
    default:
      return "khong_deploy";
  }
}

assertEqual(xacDinhMoiTruong("develop"), "staging", "develop di toi staging");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type MoiTruong = "staging" | "production" | "khong_deploy";

function xacDinhMoiTruong(tenNhanh: string): MoiTruong {
  switch (tenNhanh) {
    case "develop":
      return "staging";
    case "main":
      return "production";
    default:
      return "khong_deploy";
  }
}

assertEqual(xacDinhMoiTruong("develop"), "staging", "develop di toi staging");
```

```typescript title=test
assertEqual(xacDinhMoiTruong("main"), "production", "main di toi production");
assertEqual(xacDinhMoiTruong("feature/gio-hang"), "khong_deploy", "nhanh tinh nang khong deploy");
assertEqual(xacDinhMoiTruong(""), "khong_deploy", "chuoi rong khong deploy");
assertEqual(xacDinhMoiTruong("Main"), "khong_deploy", "phan biet hoa thuong -- Main khac main");
```

:::hints
- kind: attention
  body: "develop trả về \"staging\", main trả về \"production\" — đúng như tên môi trường đã khai trong kiểu MoiTruong."
- kind: strategy
  body: '"staging" : "production" — hai chuỗi literal khớp union MoiTruong.'
- kind: one-line
  body: '___ (develop) = "staging"\n___ (main) = "production"'
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
Cùng pipeline, khác đích deploy theo nhánh. Bài tiếp theo: deploy
lỗi rồi, làm sao quay LẠI phiên bản TRƯỚC?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`xacDinhMoiTruong` quyết định đích deploy — NHƯNG NẾU phiên bản MỚI
deploy LÊN `"production"` LẠI CÓ lỗi (dù pipeline ĐÃ qua — test
KHÔNG bắt được MỌI thứ), LÀM SAO quay VỀ phiên bản TRƯỚC ĐÓ?
::::

::::checkpoint{mastery=0.8}
::::
