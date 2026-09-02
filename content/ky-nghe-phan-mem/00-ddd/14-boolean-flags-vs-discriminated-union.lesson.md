---
id: ky-nghe-phan-mem.ddd.boolean-flags-vs-discriminated-union
title: "Vấn đề của boolean flags — illegal states are representable"
summary: "N boolean flags tạo 2ⁿ tổ hợp, đa số BẤT HỢP LỆ. DonHangTe với 4 boolean (daThanhToan/daGiao/daNhan/daHuy) → 16 tổ hợp, chỉ 5 hợp lệ — nghịch lý \"đã giao nhưng CHƯA thanh toán\" biểu diễn được dù vô lý."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [ddd.boolean-flags-vs-du]
requires: [ddd.value-object-handwritten]
concepts: [ddd.boolean-flags-vs-du]
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
Một đơn hàng có nhiều "giai đoạn" — đã thanh toán chưa, đã giao chưa,
đã huỷ chưa. Cách TỰ NHIÊN nhất để mã hoá: MỘT cờ `boolean` cho MỖI
giai đoạn. Cách đó có vấn đề gì?
::::

::::explain{#van-de-boolean-flags}
```typescript
type DonHangTe = {
  daThanhToan: boolean;
  daGiao: boolean;
  daNhan: boolean;
  daHuy: boolean;
};

// Trạng thái "đã giao nhưng CHƯA thanh toán" — VÔ LÝ (sao giao được khi
// chưa thanh toán?), nhưng TypeScript CHO PHÉP viết ra
const donHangPhiLy: DonHangTe = {
  daThanhToan: false,
  daGiao: true,
  daNhan: false,
  daHuy: false,
};
console.log(JSON.stringify(donHangPhiLy));
```

```text
{"daThanhToan":false,"daGiao":true,"daNhan":false,"daHuy":false}
```

`DonHangTe` biên dịch và chạy HOÀN TOÀN bình thường — TypeScript KHÔNG
hề phàn nàn về trạng thái PHI LÝ này. Đây LÀ vấn đề: **N boolean flags
tạo `2^N` TỔ HỢP** — với BỐN cờ, đó là `2^4 = 16` tổ hợp — nhưng ĐA SỐ
trong 16 tổ hợp đó là BẤT HỢP LỆ (đã huỷ mà vẫn đã giao? đã nhận mà
chưa thanh toán?). Kiểu dữ liệu KHÔNG hề NGĂN việc tạo ra một trạng
thái phi lý — lỗi này chỉ phát hiện được LÚC CHẠY (nếu có ai đó KIỂM
TRA), không phải LÚC BIÊN DỊCH.
::::

::::example{#dem-to-hop}
Đếm CHÍNH XÁC: BAO NHIÊU trong 16 tổ hợp là HỢP LÝ?

```typescript title=readonly
type DonHangTe = { daThanhToan: boolean; daGiao: boolean; daNhan: boolean; daHuy: boolean };

// Chỉ NĂM trạng thái THẬT SỰ hợp lý theo trình tự nghiệp vụ:
// 1. Chưa gì cả:        { false, false, false, false }
// 2. Đã thanh toán:     { true,  false, false, false }
// 3. Đã giao:           { true,  true,  false, false }
// 4. Đã nhận:           { true,  true,  true,  false }
// 5. Đã huỷ (từ đầu):   { false, false, false, true  }
//
// 11 tổ hợp CÒN LẠI (16 - 5) đều PHI LÝ — ví dụ: đã giao mà chưa thanh
// toán, đã huỷ mà vẫn đã nhận, đã nhận mà chưa giao...
console.log(Math.pow(2, 4));
console.log(16 - 5);
```

```text title=readonly
16
11
```

MƯỜI MỘT trong MƯỜI SÁU tổ hợp — GẦN 70% — là trạng thái PHI LÝ mà
kiểu `DonHangTe` VẪN cho phép biểu diễn. Đây gọi là "illegal states are
representable" (trạng thái bất hợp lệ VẪN biểu diễn được) — một VẤN ĐỀ
THẬT của thiết kế dữ liệu, không phải lý thuyết suông.
::::

::::predict{#doan-dem-to-hop-huy-va-giao commitOnce}
```typescript
type DonHangTe = { daThanhToan: boolean; daGiao: boolean; daNhan: boolean; daHuy: boolean };

const donHangKyLa: DonHangTe = {
  daThanhToan: true,
  daGiao: true,
  daNhan: false,
  daHuy: true,
};

console.log(donHangKyLa.daGiao && donHangKyLa.daHuy);
```

`donHangKyLa` vừa `daGiao: true` VỪA `daHuy: true` (đã giao NHƯNG cũng
đã huỷ — mâu thuẫn nghiệp vụ). Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
Máy báo lỗi biên dịch — TypeScript phát hiện `daGiao: true` VÀ
`daHuy: true` CÙNG lúc là mâu thuẫn nghiệp vụ, từ chối biên dịch
::why
Gần đúng ở việc bạn nhận ra ĐÂY LÀ một mâu thuẫn NGHIỆP VỤ THẬT (một
đơn hàng không thể VỪA giao VỪA huỷ) — quan sát về BẢN CHẤT phi lý đó
đúng, và CHÍNH LÀ điều bài học này muốn chỉ ra.

Chỗ lệch: TypeScript CHỈ kiểm tra KIỂU DỮ LIỆU (mỗi field ĐÚNG là
`boolean` không) — nó KHÔNG HỀ biết (và không có cách nào biết, với
thiết kế `DonHangTe` này) rằng "giao" và "huỷ" không nên cùng `true`.
Đây CHÍNH XÁC LÀ vấn đề bài học đặt ra: kiểu dữ liệu boolean flags
KHÔNG NGĂN được mâu thuẫn nghiệp vụ — code biên dịch VÀ CHẠY sạch, dù
dữ liệu vô lý.
::
:::

:::opt
`false` — vì hai cờ mâu thuẫn nhau, TypeScript tự động đặt LẠI một
trong hai cờ về `false` lúc gán giá trị
::why
Gần đúng ở việc bạn nhận ra CÓ mâu thuẫn giữa `daGiao` và `daHuy` —
quan sát về việc CÓ xung đột đó đúng.

Chỗ lệch: TypeScript KHÔNG có cơ chế "tự sửa" giá trị field dựa trên
field khác — mỗi field ĐƯỢC GÁN chính xác giá trị bạn viết, KHÔNG hề
kiểm tra chéo. `daGiao` VẪN là `true`, `daHuy` VẪN là `true`, phép `&&`
giữa hai giá trị `true` cho ra `true` — đúng như logic Boolean thông
thường, không có "tự sửa" nào can thiệp.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
N boolean flags = 2ⁿ tổ hợp, đa số bất hợp lệ. TypeScript không ngăn
được — cần một cách khác để mô hình hoá trạng thái.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã học discriminated union (T4.3) — mỗi variant mang ĐÚNG data cần
cho NÓ. Áp dụng nó vào bài toán trạng thái đơn hàng thì trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
