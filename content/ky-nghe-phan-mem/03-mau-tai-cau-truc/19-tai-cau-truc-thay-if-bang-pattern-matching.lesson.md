---
id: ky-nghe-phan-mem.mau-tai-cau-truc.tai-cau-truc-thay-if-bang-pattern-matching
title: "Tái cấu trúc: Replace Conditional with Pattern Matching"
summary: "Chuỗi if/else if DÀI kiểm một biến string LÀ dấu hiệu nên chuyển sang Discriminated Union + exhaustive switch. Trình biên dịch CẢNH BÁO khi THIẾU một nhánh; if/else với string THUẦN chấp nhận BẤT KỲ chuỗi nào, kể cả một chuỗi GÕ SAI (typo) — lặng lẽ rơi vào nhánh mặc định, KHÔNG cảnh báo gì."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 19
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.refactor-conditional-to-pattern-match]
requires: [mau.refactor-extract-function]
concepts: [mau.refactor-conditional-to-pattern-match]
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
`if (loai === "thuong") ... else if (loai === "vip") ...` — chuỗi
`if/else if` DÀI kiểm MỘT biến chuỗi. Có gì RỦI RO Ở cách viết NÀY?
::::

::::explain{#if-else-vs-switch-union}
Chuỗi `if/else if` DÀI kiểm tra MỘT biến CHUỖI/SỐ LÀ dấu hiệu NÊN
chuyển sang Discriminated Union + exhaustive `switch` (đã học T4.3)
— trình biên dịch **CẢNH BÁO** khi THIẾU một nhánh, `if/else` KHÔNG
cảnh báo được điều ĐÓ:

```typescript title=readonly
// V1: if/else, loaiThanhVien LÀ string THUẦN (BẤT KỲ chuỗi nào)
function tinhPhanTramGiamV1(loaiThanhVien: string): number {
  if (loaiThanhVien === "thuong") return 0;
  else if (loaiThanhVien === "vip") return 10;
  else if (loaiThanhVien === "vip_vang") return 20;
  return 0;
}

// V2: switch trên UNION KIỂU CHUỖI HẸP
type LoaiThanhVien = "thuong" | "vip" | "vip_vang";
function tinhPhanTramGiamV2(loaiThanhVien: LoaiThanhVien): number {
  switch (loaiThanhVien) {
    case "thuong": return 0;
    case "vip": return 10;
    case "vip_vang": return 20;
  }
}

console.log(tinhPhanTramGiamV1("vip"));
console.log(tinhPhanTramGiamV2("vip"));
```

```text title=readonly
10
10
```

CÙNG kết quả — NHƯNG `LoaiThanhVien` (V2) LÀ union CHỈ ĐÚNG BA giá
trị chuỗi, TypeScript CHỈ CHO PHÉP gọi `tinhPhanTramGiamV2` VỚI ĐÚNG
BA chuỗi ĐÓ. `loaiThanhVien: string` (V1) chấp nhận **BẤT KỲ** chuỗi
nào — KHÔNG có RÀNG BUỘC nào cả.
::::

::::example{#exhaustive-check-that-su}
Kỹ thuật `never` giúp trình biên dịch **THẬT SỰ** cảnh báo nếu thiếu
một nhánh: nếu `LoaiThanhVien` sau NÀY thêm giá trị MỚI mà QUÊN cập
nhật `switch`, tham số truyền vào nhánh `default` KHÔNG còn khớp kiểu
`never` — biên dịch LỖI NGAY:

```typescript title=readonly
type LoaiThanhVien = "thuong" | "vip" | "vip_vang";

function kiemTraKhongDatToi(x: never): never {
  throw new Error("truong hop khong duoc xu ly: " + x);
}

function tinhPhanTramGiam(loaiThanhVien: LoaiThanhVien): number {
  switch (loaiThanhVien) {
    case "thuong": return 0;
    case "vip": return 10;
    case "vip_vang": return 20;
    default: return kiemTraKhongDatToi(loaiThanhVien);
  }
}

console.log(tinhPhanTramGiam("thuong"));
console.log(tinhPhanTramGiam("vip"));
```

```text title=readonly
0
10
```

`kiemTraKhongDatToi(x: never)` CHỈ nhận được tham số nếu `x` "KHÔNG
CÒN khả năng nào" (kiểu `never`) — khi `switch` ĐÃ xử lý HẾT BA giá
trị của `LoaiThanhVien`, nhánh `default` KHÔNG BAO GIỜ chạy TỚI (VÔ
DỤNG về mặt CHẠY, NHƯNG hữu ích về mặt BIÊN DỊCH: nếu thêm MỘT giá
trị MỚI vào `LoaiThanhVien` mà QUÊN case, `loaiThanhVien` Ở nhánh
`default` sẽ KHÔNG còn LÀ `never`, TypeScript báo lỗi TẠI ĐÓ).
::::

::::predict{#doan-typo-im-lang commitOnce}
```typescript
function tinhPhanTramGiamV1(loaiThanhVien: string): number {
  if (loaiThanhVien === "thuong") return 0;
  else if (loaiThanhVien === "vip") return 10;
  else if (loaiThanhVien === "vip_vang") return 20;
  return 0;
}

console.log(tinhPhanTramGiamV1("vipp"));
```

`"vipp"` LÀ MỘT LỖI GÕ (thiếu KIỂM TRA, nhẽ RA phải LÀ `"vip"`).
Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
Máy báo lỗi lúc chạy — `"vipp"` KHÔNG khớp BẤT KỲ nhánh `if/else`
nào, JavaScript NÉM lỗi "giá trị KHÔNG hợp lệ" khi MỘT chuỗi KHÔNG
khớp trường hợp NÀO được liệt kê
::why
Gần đúng ở việc bạn nhận ra `"vipp"` KHÔNG khớp BẤT KỲ điều kiện `===`
NÀO (`"thuong"`, `"vip"`, `"vip_vang"`) — quan sát ĐÓ, về SO SÁNH
chuỗi, chính xác.

Chỗ lệch: JavaScript/TypeScript **KHÔNG TỰ ĐỘNG** ném lỗi khi MỘT
chuỗi KHÔNG khớp case NÀO — hàm ĐƠN GIẢN chạy TỚI dòng `return 0;`
CUỐI CÙNG (nhánh "mặc định" IM LẶNG), KHÔNG có gì báo hiệu "vipp" LÀ
LỖI GÕ. In THẲNG `0` — GIỐNG HỆT kết quả CHO `"thuong"` (hợp lệ)!
Đây CHÍNH LÀ RỦI RO THẬT của `if/else` với `string` THUẦN: LỖI GÕ bị
NUỐT ÂM THẦM, KHÔNG cảnh báo gì. `LoaiThanhVien` (V2, union HẸP) sẽ
CHẶN lỗi NÀY NGAY LÚC BIÊN DỊCH — `tinhPhanTramGiamV2("vipp")`
KHÔNG BAO GIỜ biên dịch được.
::
:::

:::opt
Máy báo lỗi biên dịch — `tinhPhanTramGiamV1("vipp")` truyền MỘT chuỗi
KHÔNG khớp bất kỳ literal nào so sánh BÊN TRONG hàm, TypeScript kiểm
tra được ĐIỀU NÀY dù tham số khai `string` THUẦN
::why
Gần đúng ở việc bạn tin TypeScript có THỂ "nhìn vào" nội dung SO SÁNH
BÊN TRONG hàm — một quan sát THỂ HIỆN kỳ vọng hợp lý VỀ một trình
kiểm kiểu MẠNH.

Chỗ lệch: `loaiThanhVien: string` khai kiểu **RẤT RỘNG** ("bất kỳ
chuỗi nào") — TypeScript KHÔNG phân tích LOGIC BÊN TRONG hàm (các
so sánh `===`) để "đoán" giá trị hợp lệ LÀ gì. Bất kỳ `string` NÀO
(kể cả `"vipp"`) ĐỀU khớp KIỂU tham số — biên dịch SẠCH, dù hành vi
LÚC CHẠY LÀ sai LẦM (rơi vào default).
::
:::
::::

::::code{#viet_switch_union}
Hoàn thiện `tinhPhanTramGiam` — `switch` trên union hẹp `LoaiThanhVien`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LoaiThanhVien = "thuong" | "vip" | "vip_vang";

function tinhPhanTramGiam(loaiThanhVien: LoaiThanhVien): number {
  switch (loaiThanhVien) {
    case "thuong":
      return ___;
    case "vip":
      return ___;
    case "vip_vang":
      return ___;
  }
}

assertEqual(tinhPhanTramGiam("thuong"), 0, "thuong khong giam");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type LoaiThanhVien = "thuong" | "vip" | "vip_vang";

function tinhPhanTramGiam(loaiThanhVien: LoaiThanhVien): number {
  switch (loaiThanhVien) {
    case "thuong":
      return 0;
    case "vip":
      return 10;
    case "vip_vang":
      return 20;
  }
}

assertEqual(tinhPhanTramGiam("thuong"), 0, "thuong khong giam");
```

```typescript title=test
assertEqual(tinhPhanTramGiam("vip"), 10, "vip giam 10");
assertEqual(tinhPhanTramGiam("vip_vang"), 20, "vip vang giam 20");
```

:::hints
- kind: attention
  body: "thuong: không giảm. vip: giảm 10. vip_vang: giảm 20 (số phần trăm, không phải hệ số nhân)."
- kind: strategy
  body: "0 : 10 : 20 — ba số phần trăm giảm giá, tăng dần theo hạng thành viên."
- kind: one-line
  body: '___ (thuong) = 0\n___ (vip) = 10\n___ (vip_vang) = 20'
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
Chuỗi if/else + string thuần = lỗi gõ bị nuốt âm thầm. Union hẹp +
switch = biên dịch chặn NGAY. Bài chốt cụm: ghép pipeline + refactor
vào MỘT ví dụ thực.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`taoPipeline` (bài 17) ghép NHIỀU hàm nhỏ. Extract Function (bài 18)
cắt hàm dài THÀNH nhiều hàm nhỏ. Nếu MỘT hàm xử lý request LỒNG NHAU
sâu (validate BÊN TRONG if, RỒI auth BÊN TRONG if ĐÓ...) — GHÉP hai
kỹ thuật NÀY lại thế nào?
::::

::::checkpoint{mastery=0.8}
::::
