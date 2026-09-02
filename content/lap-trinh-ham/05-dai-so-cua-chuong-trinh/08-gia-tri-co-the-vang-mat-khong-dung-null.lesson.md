---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.gia-tri-co-the-vang-mat-khong-dung-null
title: "Giá trị CÓ THỂ VẮNG MẶT — không dùng `null`/`undefined`"
summary: "ds.find(...) trả về T | undefined — kiểm bằng if (x) thay vì if (x !== undefined) làm 0 (một giá trị TÌM THẤY hợp lệ) bị coi nhầm là 'không tìm thấy'. undefined tự nó không tự bảo vệ khỏi lỗi này."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.avoid-null-motivation]
requires: [ts.adt-gate-boss]
concepts: [alg.avoid-null-motivation]
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
Cụm 1 chốt lại chuyện "kết hợp nhiều giá trị". Cụm 2 quay lại một vấn
đề CŨ hơn — T4.0a từng nêu: một giá trị có thể KHÔNG CÓ. `undefined`
xử lý việc đó thế nào?
::::

::::explain{#bay-falsy-cua-undefined}
```typescript
function timSoChan(ds: number[]): number | undefined {
  return ds.find((x) => x % 2 === 0);
}

const ds = [5, 3, 0, 7];
const ketQua = timSoChan(ds);

if (ketQua) {
  console.log("tìm thấy: " + ketQua);
} else {
  console.log("không tìm thấy");
}
```

```text
không tìm thấy
```

`ds = [5, 3, 0, 7]` — CÓ một số chẵn: `0`. `timSoChan` TÌM THẤY nó
đúng — `ketQua` THẬT SỰ là `0`, không phải `undefined`. Nhưng
`if (ketQua)` in ra "không tìm thấy" — SAI. Lý do: `0` là một giá trị
"falsy" trong JavaScript/TypeScript (`if (0)` LUÔN coi là `false`,
giống `if (undefined)`) — `if (ketQua)` KHÔNG PHÂN BIỆT được "tìm thấy
giá trị `0`" với "không tìm thấy gì cả (`undefined`)".

Đây KHÔNG phải lỗi cú pháp — TypeScript KHÔNG báo gì (T4.0a's kiểm tra
`possibly undefined` chỉ bắt việc DÙNG `ketQua` mà CHƯA kiểm tra gì cả,
không bắt được việc kiểm tra SAI CÁCH như `if (ketQua)`). Đây là lỗi
LOGIC, ẩn, chỉ lộ ra khi dữ liệu THẬT chứa đúng giá trị "falsy" (`0`,
`""`, `false`, `NaN`).
::::

::::example{#sua-bang-so-sanh-tuong-minh}
Sửa bằng cách so sánh TƯỜNG MINH với `undefined`, không dựa vào
"truthy/falsy":

```typescript title=readonly
function timSoChan(ds: number[]): number | undefined {
  return ds.find((x) => x % 2 === 0);
}

const ds = [5, 3, 0, 7];
const ketQua = timSoChan(ds);

if (ketQua !== undefined) {
  console.log("tìm thấy: " + ketQua);
} else {
  console.log("không tìm thấy");
}
```

```text title=readonly
tìm thấy: 0
```

`ketQua !== undefined` — so sánh TƯỜNG MINH, không quan tâm `ketQua`
là giá trị gì (kể cả `0`, `""`, `false`), CHỈ quan tâm nó có PHẢI
`undefined` hay không. Sửa được — nhưng phải NHỚ dùng đúng cách so
sánh này ở MỌI chỗ dùng `undefined`, không có gì BUỘC bạn phải nhớ.
Quên MỘT chỗ, lỗi lặp lại.
::::

::::predict{#doan-bay-falsy commitOnce}
```typescript
function timChuoiRong(ds: string[]): string | undefined {
  return ds.find((s) => s.length === 0);
}

const danhSach = ["chao", "", "ban"];
const ketQua = timChuoiRong(danhSach);

if (ketQua) {
  console.log("tìm thấy chuỗi rỗng");
} else {
  console.log("không có chuỗi rỗng nào");
}
```

Dòng cuối in ra gì?

:::opt{correct}
`không có chuỗi rỗng nào`
:::

:::opt
`tìm thấy chuỗi rỗng` — vì `timChuoiRong` THẬT SỰ tìm thấy một chuỗi
rỗng trong `danhSach`
::why
Gần đúng ở việc bạn đúng: `danhSach` CÓ một chuỗi rỗng (`""`, phần tử
thứ hai), và `timChuoiRong` THẬT SỰ tìm ra nó — `ketQua` THẬT SỰ là
`""`, không phải `undefined`.

Chỗ lệch: `""` (chuỗi rỗng) CŨNG là giá trị "falsy", giống `0` ở ví dụ
`explain`. `if (ketQua)` với `ketQua = ""` đi vào nhánh `else` — in
"không có chuỗi rỗng nào", dù THẬT SỰ đã tìm thấy. Đúng bẫy đã học,
lần này với `string` thay vì `number`.
::
:::

:::opt
Máy báo lỗi biên dịch — so sánh `if (ketQua)` không hợp lệ với kiểu
`string | undefined`
::why
Gần đúng ở việc bạn cảnh giác về kiểu `string | undefined` có thể gây
vấn đề gì đó khi dùng trong `if` — một mối lo hợp lý.

Chỗ lệch: `if (ketQua)` biên dịch HOÀN TOÀN BÌNH THƯỜNG với BẤT KỲ
kiểu nào (TypeScript coi mọi giá trị đều "truthy" hoặc "falsy" được) —
không có lỗi cú pháp hay lỗi kiểu nào. Vấn đề THUẦN TUÝ là LOGIC (kết
quả SAI), không phải lỗi biên dịch.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`undefined` "lặn" được vào bất kỳ đâu, và cách kiểm tra nó (`if (x)`
hay `x !== undefined`) không có gì BUỘC làm đúng. Có cách mô hình hoá
KHÔNG mắc bẫy này không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

T4.3 đã dạy discriminated union — một kiểu dữ liệu mang RÕ NHÃN cho
biến thể nó thuộc về, buộc `switch` phải xử lý ĐỦ. Nếu "có giá trị"
và "không có giá trị" là HAI BIẾN THỂ của một discriminated union, thay
vì `T` với `undefined` trộn lẫn — bẫy `if (x)` ở trên còn xảy ra được
không?

Bài sau xây đúng kiểu dữ liệu đó.
::::

::::checkpoint{mastery=0.8}
::::
