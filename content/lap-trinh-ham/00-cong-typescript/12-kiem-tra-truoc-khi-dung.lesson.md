---
id: lap-trinh-ham.cong-typescript.kiem-tra-truoc-khi-dung
title: "TypeScript bắt bạn phải kiểm tra trước khi dùng"
summary: "Thêm `if (v[0] !== undefined)` trước khi dùng — TSC giờ CHO PHÉP, vì đã thấy bạn kiểm tra. Đây gọi là THU HẸP KIỂU (narrowing): trong nhánh if, TSC tự biết kiểu đã bớt khả năng — nhưng chỉ TRONG nhánh đó."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.narrowing-check]
requires: [ts.possibly-undefined]
concepts: [ts.narrowing-check]
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
Bài trước để lại một câu hỏi: khai đúng kiểu `number | undefined` chưa
đủ để DÙNG được giá trị đó. Vậy phải làm gì để TypeScript chịu tin?
::::

::::explain{#thu-hep-kieu}
Câu trả lời: kiểm tra, ngay trong code, bằng một `if`. TypeScript không
chỉ đọc KIỂU đã khai — nó còn đọc CẤU TRÚC ĐIỀU KIỆN của chương trình,
và tự thu hẹp kiểu theo từng nhánh. Việc này gọi là **thu hẹp kiểu**
(type narrowing).

```typescript
const dau: number | undefined = diem[0];

if (dau !== undefined) {
  // TẠI ĐÂY, TSC coi dau có kiểu number — không còn undefined nữa
}
```

Bên NGOÀI khối `if`, `dau` vẫn giữ kiểu đầy đủ `number | undefined` —
TSC không biết trước liệu điều kiện có đúng hay không. Nhưng BÊN TRONG
nhánh `if (dau !== undefined)`, TSC lý luận: "nếu code chạy tới đây,
điều kiện phải đúng, nghĩa là `dau` không phải `undefined` — vậy kiểu
còn lại chỉ có thể là `number`". Đó chính xác là ý nghĩa của "thu hẹp":
từ một tập kiểu rộng, bớt xuống một tập hẹp hơn, chỉ trong đúng phạm vi
mà lý luận đó còn đúng.

Điều quan trọng nhất, và dễ quên nhất: **thu hẹp chỉ có hiệu lực TRONG
đúng nhánh đã kiểm tra**. Ra khỏi khối `if` — dù chỉ một dòng sau dấu
`}` đóng — TSC không còn nhớ gì về việc đã kiểm tra nữa. Kiểu của biến
quay lại đầy đủ như trước, y hệt lúc chưa kiểm tra gì cả.
::::

::::example{#trong-va-ngoai-nhanh-if}
Cùng một biến `dau`, dùng ở hai chỗ khác nhau quanh một khối `if`:

```typescript title=readonly
const diem: number[] = [4, 5, 6];
const dau: number | undefined = diem[0];

if (dau !== undefined) {
  console.log("Đầu:", dau);
}
console.log("Nhân đôi:", dau * 2);
```

```text title=readonly
(không có mã nào được sinh ra)

TS18048 (dòng 7, cột 26): 'dau' is possibly 'undefined'.
```

Đã chạy thật: dòng `console.log("Đầu:", dau)` BÊN TRONG khối `if` hoàn
toàn hợp lệ — TSC đã thu hẹp `dau` xuống `number` ở đó. Nhưng dòng
`console.log("Nhân đôi:", dau * 2)` nằm SAU dấu `}` đóng khối `if` —
ngoài phạm vi thu hẹp — bị từ chối, mã `TS18048`, một mã khác hẳn
`TS2322` của bài trước: đây là lỗi "khả năng undefined chưa được loại
trừ", không phải lỗi "sai kiểu khi gán".
::::

::::predict{#pham-vi-thu-hep commitOnce}
Byte viết bảy dòng, KHÔNG chạy thử:

```typescript
const diem: number[] = [4, 5, 6];
const dau: number | undefined = diem[0];

if (dau !== undefined) {
  console.log("Đầu:", dau);
}
console.log("Nhân đôi:", dau * 2);
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng `console.log("Nhân đôi:", dau * 2);` — nó nằm NGOÀI khối `if`, nơi
việc thu hẹp đã kiểm tra không còn hiệu lực nữa; ở đó kiểu của `dau`
quay lại đầy đủ `number | undefined` như trước khi kiểm tra
:::

:::opt
Dòng `if (dau !== undefined)` — so sánh một biến kiểu
`number | undefined` với `undefined` là một phép so sánh không hợp lệ
::why
Gần đúng ở việc bạn để ý đúng CHỖ thu hẹp xảy ra — dòng `if` chính là
nơi quan trọng nhất của cả đoạn này.

Chỗ lệch: dòng `if` hoàn toàn hợp lệ về kiểu — đã thử thật, không bị
từ chối. Đây chính là CÁCH ĐÚNG để kiểm tra trước khi dùng, đúng như
phần explain vừa dạy — không phải một phép so sánh sai.
::
:::

:::opt
Dòng `console.log("Đầu:", dau);` bên trong khối `if` — vì TypeScript
vẫn coi `dau` có thể là `undefined` dù đang ở trong nhánh đã kiểm tra
::why
Gần đúng ở việc bạn thận trọng với khả năng `undefined` — đúng là điều
đáng thận trọng ở MỌI nơi khác trong đoạn này.

Chỗ lệch: đây chính xác là MỤC ĐÍCH của thu hẹp kiểu. Bên trong nhánh
`if (dau !== undefined)`, TSC đã loại `undefined` khỏi kiểu của `dau`
— dòng này hợp lệ hoàn toàn, đã thử thật, không bị từ chối.
::
:::

:::opt
Không dòng nào bị từ chối — đã có `if (dau !== undefined)` kiểm tra
trước rồi, nên mọi lần dùng `dau` sau đó đều an toàn
::why
Gần đúng ở việc bạn nhớ đúng luật "kiểm tra trước khi dùng" bài này
vừa dạy — có kiểm tra thật trong đoạn code.

Chỗ lệch: việc kiểm tra chỉ có hiệu lực TRONG PHẠM VI khối `if` đã
kiểm tra nó — không "lan" ra toàn bộ phần code còn lại. Dòng
`console.log("Nhân đôi:", ...)` nằm SAU dấu `}` đóng khối, ngoài phạm
vi đó, và bị từ chối thật — mã `TS18048`.
::
:::
::::

::::code{#chi-dung-trong-pham-vi-da-kiem}
Mô tả điểm đầu tiên của một danh sách CÓ THỂ RỖNG — điền đúng điều
kiện để chỉ dùng `dau` ở nơi TSC còn công nhận nó là `number`.

```typescript title=starter
function moTaDiem(mangDiem: number[]): string {
  const dau: number | undefined = mangDiem[0];

  if (___) {
    return "Điểm đầu: " + dau;
  }
  return "Không có điểm nào";
}

console.log(moTaDiem([7, 8, 9]));
console.log(moTaDiem([]));
```

```typescript title=solution
function moTaDiem(mangDiem: number[]): string {
  const dau: number | undefined = mangDiem[0];

  if (dau !== undefined) {
    return "Điểm đầu: " + dau;
  }
  return "Không có điểm nào";
}

console.log(moTaDiem([7, 8, 9]));
console.log(moTaDiem([]));
```

```typescript title=test
if (moTaDiem([7, 8, 9]) !== "Điểm đầu: 7") throw new Error("moTaDiem([7,8,9]) phải trả về \"Điểm đầu: 7\" — đang trả về " + moTaDiem([7, 8, 9]));
if (moTaDiem([]) !== "Không có điểm nào") throw new Error("moTaDiem([]) phải trả về \"Không có điểm nào\" khi mảng rỗng — đang trả về " + moTaDiem([]));
```

:::hints
- kind: attention
  body: Chỗ trống là điều kiện của if — phải kiểm tra ĐÚNG biến dau, so với undefined, để TSC thu hẹp dau xuống number bên trong nhánh này.
- kind: strategy
  body: 'dau có kiểu number | undefined. Muốn nhánh if thu hẹp dau xuống number, điều kiện phải là dau !== undefined — không phải kiểm tra mangDiem, không phải một giá trị luôn đúng/luôn sai.'
- kind: one-line
  body: 'Chỗ trống là: dau !== undefined'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Điểm đầu: 7"
- tier: output
  match: contains
  expect: "Không có điểm nào"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`dau !== undefined` không chỉ là một câu kiểm tra runtime — nó là chỗ
TypeScript đồng ý thu hẹp kiểu. Ngoài đúng phạm vi đó, `dau` vẫn giữ
nguyên khả năng vắng mặt.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tới giờ, "không có gì cả" luôn là `undefined` — kết quả của việc đọc
một ô mảng có thể trống. Nhưng có một cách KHÁC để nói "không có gì",
và nó không tự nhiên xuất hiện như `undefined` — nó là một giá trị bạn
phải CHỦ Ý viết ra: `null`.

Hai thứ đó có phải cùng một điều, chỉ khác tên gọi không? Bài sau trả
lời.
::::

::::checkpoint{mastery=0.8}
::::
