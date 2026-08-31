---
id: lap-trinh-ham.cong-typescript.null-va-undefined-la-hai-thu-khac-nhau
title: "null và undefined là hai điều khác nhau"
summary: "undefined là 'chưa từng có giá trị'; null là 'CỐ Ý đặt là không có gì' — TypeScript phân biệt rạch ròi hai mã lỗi khác nhau (TS18048 với TS18047), dù cả hai đều cần kiểm tra trước khi dùng."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.null-vs-undefined]
requires: [ts.narrowing-check]
concepts: [ts.null-vs-undefined]
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
Bài trước để lại một câu hỏi: có một cách khác để nói "không có gì",
không tự nhiên xuất hiện như `undefined`. Đây là câu trả lời.
::::

::::explain{#hai-loai-khong-co-gi}
`undefined` xuất hiện khi một chỗ CHƯA TỪNG được gán giá trị — một
tham số không truyền, một ô mảng chưa từng tồn tại (bài 11), một biến
khai mà chưa gán. Không ai VIẾT `undefined` ra một cách chủ ý trong
những trường hợp đó — nó là điều máy tự điền vào chỗ trống.

`null` khác hẳn: nó là một GIÁ TRỊ, và ai đó phải CHỦ Ý gán nó. Ví dụ
thường gặp nhất — một hàm TÌM KIẾM, không tìm thấy gì, và muốn nói
thẳng "tôi đã tìm, và kết quả là không có gì cả", thay vì lặng lẽ
không trả về gì (giống `undefined`). Đó là lý do nhiều hàm tra cứu
khai kiểu trả về dạng `T | null` — không phải TypeScript BẮT BUỘC dùng
`null`, mà người viết hàm CHỌN `null` để nói rõ ràng: "đã kiểm tra
xong, và câu trả lời là không có".

TypeScript coi `null` và `undefined` là hai kiểu HOÀN TOÀN TÁCH BIỆT.
`number | null` và `number | undefined` là hai kiểu khác nhau — một
biến hứa `number | null` không tự động chấp nhận `undefined`, và
ngược lại. Khi bạn dùng một giá trị có thể `null` mà chưa kiểm tra,
TSC báo mã lỗi khác hẳn: `TS18047` (có thể `null`), thay vì `TS18048`
(có thể `undefined`) bạn đã gặp ở bài trước. Cách sửa cũng phải ĐÚNG
LOẠI: kiểm tra `!== null` cho mã `TS18047`, không phải `!== undefined`.
::::

::::example{#tim-gia-co-the-khong-thay}
Một hàm tra giá trái cây — không tìm thấy thì CỐ Ý trả về `null`:

```typescript title=readonly
function timGia(ten: string): number | null {
  if (ten === "tao") return 8000;
  return null;
}

const giaCam: number | null = timGia("cam");
console.log(giaCam + 1000);
```

```text title=readonly
(không có mã nào được sinh ra)

TS18047 (dòng 7, cột 13): 'giaCam' is possibly 'null'.
```

Đã chạy thật: `"cam"` không khớp `"tao"`, hàm chạy tới `return null;`
— hoàn toàn hợp lệ, vì `null` là một phần của kiểu trả về đã hứa
`number | null`. Nhưng dòng `giaCam + 1000` bị từ chối, mã `TS18047`
— khác `TS18048` của bài trước, vì lần này khả năng "không có" là
`null`, không phải `undefined`.

So sánh: nếu `timGia` trả về `undefined` thay vì `null` khi không tìm
thấy (một cách viết khác, cũng hợp lệ), lỗi sẽ đổi sang `TS18048` — và
cách sửa cũng đổi theo, kiểm tra `!== undefined` thay vì `!== null`.
Hai mã lỗi này không hoán đổi cho nhau được — kiểm tra sai loại không
sửa được gì.
::::

::::predict{#kiem-sai-loai commitOnce}
Byte viết chín dòng, KHÔNG chạy thử. Chú ý: `timGia` trả về
`number | null`, không phải `number | undefined`.

```typescript
function timGia(ten: string): number | null {
  if (ten === "tao") return 8000;
  return null;
}

const gia: number | null = timGia("cam");

if (gia !== undefined) {
  console.log("Giá:", gia + 500);
}
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng `console.log("Giá:", gia + 500);` — điều kiện `if` kiểm tra
`gia !== undefined`, nhưng kiểu của `gia` là `number | null`, KHÔNG
chứa `undefined`. Kiểm tra sai loại không thu hẹp được `null` ra khỏi
kiểu, nên bên trong `if`, `gia` vẫn có thể là `null`
:::

:::opt
Dòng `if (gia !== undefined)` — so sánh một biến kiểu `number | null`
với `undefined` là kiểu không tương thích, TSC từ chối ngay tại đó
::why
Gần đúng ở việc bạn nghi ngờ đúng dòng `if` — đây chính là chỗ mấu
chốt của cả đoạn.

Chỗ lệch: TypeScript CHO PHÉP phép so sánh này về mặt kiểu, dù nó
không giúp ích gì — đã thử thật, dòng `if` không bị từ chối. Vấn đề
không phải cú pháp sai, mà là điều kiện đó KHÔNG LOẠI TRỪ được `null`,
nên chỗ dùng `gia` bên trong mới là chỗ bị từ chối.
::
:::

:::opt
Dòng `return null;` bên trong `timGia` — hàm khai kiểu trả về
`number | null` nhưng cách viết "return null trần" không hợp lệ, phải
viết `return null as null`
::why
Gần đúng ở việc bạn nghi ngờ dòng liên quan tới `null` — đúng, `null`
là trọng tâm của bài này.

Chỗ lệch: `return null;` viết trần hoàn toàn hợp lệ khi kiểu trả về đã
hứa CHỨA `null` (ở đây là `number | null`) — không cần ép kiểu gì
thêm. Dòng này khớp đúng lời hứa, đã thử thật, không bị từ chối.
::
:::

:::opt
Không dòng nào bị từ chối — đã có `if (gia !== undefined)` kiểm tra
trước khi dùng `gia`, đúng luật bài trước vừa dạy
::why
Gần đúng ở việc bạn nhớ đúng luật "kiểm tra trước khi dùng" (bài
trước) — có một câu `if` kiểm tra thật trong đoạn này.

Chỗ lệch: kiểm tra đó kiểm tra SAI LOẠI "không có gì". `gia` có thể là
`null`, không phải `undefined` — so `gia !== undefined` không loại
trừ được khả năng `null`. Bên trong khối `if`, TSC vẫn coi `gia` có
thể là `null`, và dòng dùng `gia + 500` bị từ chối thật, mã `TS18047`.
::
:::
::::

::::code{#kiem-dung-loai-khong-co-gi}
Hoàn thiện một hàm mô tả giá trái cây — điền đúng điều kiện kiểm tra
ĐÚNG LOẠI "không có gì" mà `timGia` thật sự trả về.

```typescript title=starter
function timGia(ten: string): number | null {
  if (ten === "tao") return 8000;
  if (ten === "cam") return 6000;
  return null;
}

function moTaGia(ten: string): string {
  const gia = timGia(ten);
  if (___) {
    return ten + ": " + gia + "đ";
  }
  return ten + ": không có giá";
}

console.log(moTaGia("tao"));
console.log(moTaGia("xoai"));
```

```typescript title=solution
function timGia(ten: string): number | null {
  if (ten === "tao") return 8000;
  if (ten === "cam") return 6000;
  return null;
}

function moTaGia(ten: string): string {
  const gia = timGia(ten);
  if (gia !== null) {
    return ten + ": " + gia + "đ";
  }
  return ten + ": không có giá";
}

console.log(moTaGia("tao"));
console.log(moTaGia("xoai"));
```

```typescript title=test
if (moTaGia("tao") !== "tao: 8000đ") throw new Error("moTaGia(\"tao\") phải trả về \"tao: 8000đ\" — đang trả về " + moTaGia("tao"));
if (moTaGia("xoai") !== "xoai: không có giá") throw new Error("moTaGia(\"xoai\") phải trả về \"xoai: không có giá\" — đang trả về " + moTaGia("xoai"));
```

:::hints
- kind: attention
  body: timGia trả về number | null, không phải number | undefined. Điều kiện if phải kiểm tra ĐÚNG loại "không có gì" mà hàm này thật sự dùng.
- kind: strategy
  body: 'gia có kiểu number | null. Kiểm tra undefined không loại trừ được null — phải so gia !== null để TSC thu hẹp gia xuống number bên trong nhánh if.'
- kind: one-line
  body: 'Chỗ trống là: gia !== null'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "tao: 8000đ"
- tier: output
  match: contains
  expect: "xoai: không có giá"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`gia !== null`, không phải `gia !== undefined` — cùng ý tưởng thu hẹp
của bài trước, nhưng phải chọn ĐÚNG loại "không có gì" mà kiểu đã hứa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp, khép lại cụm bài này.

Ba bài liền, mỗi bài xử lý một mảnh riêng: đọc một ô mảng có thể trống
(bài 11), kiểm tra trước khi dùng (bài 12), phân biệt đúng loại "không
có gì" (bài này). Nhưng mọi ví dụ đều TÁCH RIÊNG từng mảnh.

Một hàm THẬT nhận một mảng có thể rỗng, đọc phần tử đầu, kiểm tra đúng
cách, và tự quyết định trả `null` hay giá trị thật — ghép cả ba mảnh
vào MỘT hàm sẽ trông ra sao? Bài sau ghép lại.
::::

::::checkpoint{mastery=0.8}
::::
