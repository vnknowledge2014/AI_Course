---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.pattern-match-tren-option
title: "`switch` trên `Option` — đúng kỷ luật exhaustiveness đã học"
summary: "switch (o.kind) { case \"co\": ...; case \"khong\": ... } — nối thẳng T4.3's exhaustiveness. Quên nhánh \"khong\", không assertNever, không khai kiểu trả về tường minh: biên dịch SẠCH, undefined lúc chạy."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.pattern-match-option]
requires: [alg.option-type]
concepts: [alg.pattern-match-option]
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
Bạn đọc `Option` bằng `if (o.kind === "co")`. Có cách viết gọn hơn khi
cần xử lý CẢ hai nhánh không — và nó có ĐÒI xử lý đủ không?
::::

::::explain{#switch-tren-option}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };

function co<T>(giaTri: T): Option<T> {
  return { kind: "co", giaTri };
}
function khong<T>(): Option<T> {
  return { kind: "khong" };
}

function moTa(o: Option<number>): string {
  switch (o.kind) {
    case "co":
      return "có: " + o.giaTri;
    case "khong":
      return "không có gì";
  }
}

console.log(moTa(co(5)));
console.log(moTa(khong()));
```

```text
có: 5
không có gì
```

`switch` trên `o.kind` — đúng cú pháp T4.3 đã dạy (bài 4), mỗi `case`
NARROW `o` xuống đúng biến thể. Không có gì mới về CÚ PHÁP. Câu hỏi
thật của bài này: nếu QUÊN một nhánh, chuyện gì xảy ra?
::::

::::example{#quen-nhanh-khong-khong-bao-loi}
Quên `case "khong"`, KHÔNG khai kiểu trả về tường minh (không viết
`: string`) — đúng tình huống dễ xảy ra nhất khi viết vội:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };

function co<T>(giaTri: T): Option<T> {
  return { kind: "co", giaTri };
}
function khong<T>(): Option<T> {
  return { kind: "khong" };
}

function moTa(o: Option<number>) {
  switch (o.kind) {
    case "co":
      return "có: " + o.giaTri;
  }
}

console.log(moTa(co(5)));
const ketQua = moTa(khong());
console.log(typeof ketQua);
```

```text title=readonly
có: 5
undefined
```

`moTa` KHÔNG khai `: string` — TypeScript TỰ SUY LUẬN kiểu trả về là
`string | undefined` (vì MỘT nhánh có `return`, nhánh CÒN LẠI (thiếu
`case "khong"`) khiến hàm "chạy hết" mà không `return` gì, ngầm định
`undefined`). Vì kiểu suy luận ĐÃ BAO GỒM `undefined`, TypeScript
KHÔNG báo lỗi gì — biên dịch SẠCH. `moTa(khong())` chạy tới cuối hàm,
KHÔNG khớp `case` nào, trả về `undefined` — đúng lỗi bài 7 T4.3 đã dạy,
giờ áp dụng lên `Option` thay vì `Hinh`.
::::

::::predict{#doan-thieu-nhanh-option commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };

function khong<T>(): Option<T> {
  return { kind: "khong" };
}

function moTa(o: Option<number>): string {
  switch (o.kind) {
    case "co":
      return "có: " + o.giaTri;
    case "khong":
      return "không có gì";
  }
}

console.log(moTa(khong()));
```

Chương trình này biên dịch được không?

:::opt{correct}
Được — cả hai biến thể `"co"`/`"khong"` ĐỀU có `case` xử lý, dù có khai
`: string` tường minh
:::

:::opt
Không — thiếu `assertNever` ở nhánh `default` nên TypeScript từ chối
biên dịch, đúng lỗi bài 9 T4.3 đã dạy
::why
Gần đúng ở việc bạn nhớ ĐÚNG mẫu `assertNever` (T4.3 bài 8-9) có thể
BẮT lỗi biên dịch cho `switch` không đủ nhánh — cơ chế đó có thật.

Chỗ lệch: `assertNever` chỉ BẮT LỖI khi THIẾU một `case` — ở ĐÂY, CẢ
HAI biến thể (`"co"` VÀ `"khong"`) ĐÃ có `case` xử lý ĐỦ, không thiếu
gì cả. Không có `assertNever` không phải vấn đề — vấn đề CHỈ xảy ra
khi THIẾU case (đúng ví dụ ở khối `explain`, không phải bài predict
này).
::
:::

:::opt
Không — hàm CÓ khai `: string` tường minh nên bắt buộc PHẢI có
`default` hoặc `assertNever`, không có ngoại lệ
::why
Gần đúng ở việc bạn nhớ khai kiểu trả về TƯỜNG MINH có ảnh hưởng tới
việc TypeScript kiểm tra "đủ nhánh trả về" hay không — quan sát đó có
CĂN CỨ (khối `example` vừa cho thấy ẢNH HƯỞNG đó).

Chỗ lệch: khai `: string` CHỈ gây lỗi khi THẬT SỰ có đường chạy nào
không trả về `string` (ví dụ thiếu case). Ở đây MỌI `case` (`"co"`
LẪN `"khong"`) đều `return` một `string` — không có đường chạy nào "lọt
qua" mà không trả về gì, nên không cần `default`/`assertNever`, biên
dịch sạch.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`switch` trên `Option` hoạt động ĐÚNG như trên bất kỳ discriminated
union nào khác — quên nhánh vẫn là bẫy CŨ, nhưng bạn đã có công cụ từ
T4.3 để phòng nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Option<T>` nói "có" hoặc "không" — nhưng KHÔNG nói TẠI SAO "không".
Nếu một phép tính THẤT BẠI, và người dùng CẦN biết lý do (không chỉ
"không có kết quả") thì `Option` có đủ không?

Bài sau xây một kiểu MỚI, mang theo LÝ DO.
::::

::::checkpoint{mastery=0.8}
::::
