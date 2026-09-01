---
id: lap-trinh-ham.adt-pattern-matching.doan-truoc-brand-bien-mat-luc-chay
title: "Dự đoán: brand có 'sống sót' lúc chạy không?"
summary: "Một biến kiểu `Email` — lúc CHẠY, `typeof bienDo` là gì, `JSON.stringify(bienDo)` in ra gì? Đáp án đúng: `\"string\"` và y hệt một chuỗi thường, không field `__brand` nào xuất hiện — brand CHỈ tồn tại lúc biên dịch, bị XOÁ hoàn toàn khi sinh ra JavaScript."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 28
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [ts.predict-brand-erasure]
requires: [ts.branded-type-basics]
concepts: [ts.predict-brand-erasure]
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
Bài trước: `Email` là một `string` GẮN NHÃN — nhãn đó CHẶN nhầm tham
số lúc biên dịch, nhưng bạn mới NGHE nói `typeof` nó vẫn là `"string"`
lúc chạy, chưa tự kiểm chi tiết. Đoán trước hôm nay: nhãn đó "sống sót"
tới đâu?
::::

::::explain{#typeof-va-json-stringify-cua-brand}
```typescript
type Email = string & { readonly __brand: "Email" };

function taoEmail(s: string): Email {
  return s as Email;
}

const bienDo: Email = taoEmail("an@vidu.com");

console.log(typeof bienDo);
console.log(JSON.stringify(bienDo));
console.log(bienDo === "an@vidu.com");
```

```text
string
"an@vidu.com"
true
```

`bienDo` được khai kiểu `Email` — theo bài trước, `Email` là
`string & { readonly __brand: "Email" }`. Nhìn phần khai kiểu, dễ
tưởng có một object với field `__brand` thật sự nằm đâu đó trong bộ
nhớ. Nhưng `typeof bienDo` trả về `"string"` — KHÔNG PHẢI `"object"`.
`JSON.stringify(bienDo)` in ra `"an@vidu.com"` — y hệt việc
`JSON.stringify` một chuỗi trần trụi, không có field `__brand` nào
chen vào. Và `bienDo === "an@vidu.com"` trả `true` — so sánh CHẶT với
một chuỗi thường, không lỗi, không khác biệt nào.

Lý do: `& { readonly __brand: "Email" }` chỉ là một PHẦN của KHAI BÁO
KIỂU — nó nói cho TRÌNH BIÊN DỊCH biết "đừng lẫn `Email` với `string`
thường", nhưng KHÔNG sinh ra field nào trong mã JavaScript thật. Dòng
`return s as Email;` trong `taoEmail` cũng không "đóng gói" `s` vào bất
cứ thứ gì — `as Email` chỉ là một lời khẳng định VỚI TRÌNH BIÊN DỊCH
("tin tôi, coi chuỗi này là `Email`"), không sinh thêm MỘT DÒNG mã máy
nào. Biên dịch xong, `Email` biến MẤT HOÀN TOÀN — chỉ còn lại đúng một
chuỗi JavaScript.
::::

::::example{#brand-hoat-dong-y-het-string}
Truyền `bienDo` (kiểu `Email`) vào một hàm đòi tham số `string` bình
thường, gọi thẳng phương thức của `string` trên nó:

```typescript title=readonly
type Email = string & { readonly __brand: "Email" };

function taoEmail(s: string): Email {
  return s as Email;
}

function inHoa(s: string): string {
  return s.toUpperCase();
}

const bienDo: Email = taoEmail("an@vidu.com");

console.log(inHoa(bienDo));
console.log(bienDo.length);

const bienGoc: string = bienDo;
console.log(bienGoc === bienDo);
```

```text title=readonly
AN@VIDU.COM
11
true
```

`inHoa` khai tham số `s: string` — không hề biết gì về `Email` — vẫn
NHẬN `bienDo` bình thường (mọi `Email` ĐỀU LÀ một `string`, chiều
ngược lại mới bị chặn, như bài trước đã đo). `bienDo.length` chạy y hệt
phương thức `.length` trên bất kỳ chuỗi nào — vì LÚC CHẠY, `bienDo`
CHÍNH LÀ một chuỗi, không có lớp bọc nào. Gán thẳng vào một biến khai
kiểu `string` (`bienGoc`) không cần ép kiểu gì — thu hẹp nhãn chỉ là
"quên bớt" một điều kiện kiểu, giá trị THẬT không đổi.
::::

::::predict{#doan-truoc-erasure commitOnce}
```typescript
type MaSoThue = string & { readonly __brand: "MaSoThue" };

function taoMaSoThue(s: string): MaSoThue {
  return s as MaSoThue;
}

const bienDo: MaSoThue = taoMaSoThue("0312345678");

console.log(typeof bienDo);
console.log(JSON.stringify(bienDo));
```

Hai dòng `console.log` in ra gì?

:::opt{correct}
`"string"` rồi `"0312345678"` — y hệt kết quả nếu `bienDo` được khai
thẳng kiểu `string`, không branded gì cả
:::

:::opt
`"object"` rồi `{"__brand":"MaSoThue","value":"0312345678"}` — vì
`MaSoThue` được TypeScript "đóng gói" thành một object mang field
`__brand`, để phân biệt với `string` thường lúc chạy
::why
Gần đúng ở việc bạn nhớ đúng HÌNH DẠNG của khai báo kiểu —
`type MaSoThue = string & { readonly __brand: "MaSoThue" }` đúng là CÓ
field `__brand` xuất hiện trong cú pháp khai kiểu.

Chỗ lệch: field `__brand` đó chỉ tồn tại trong THẾ GIỚI KIỂU, lúc biên
dịch — không sinh ra bất kỳ mã JavaScript nào. Hàm `taoMaSoThue` chỉ
trả về ĐÚNG chuỗi `s` truyền vào (qua `as MaSoThue`, một lời khẳng định
kiểu, không đổi giá trị) — không "đóng gói" gì hết. Đã đo:
`typeof bienDo` trả `"string"`, không phải `"object"`, và
`JSON.stringify` không hề in ra field `__brand` nào.
::
:::

:::opt
`"string"` rồi `{"__brand":"MaSoThue","value":"0312345678"}` — `typeof`
đúng là `"string"`, nhưng `JSON.stringify` vẫn "nhìn thấy" và in kèm
nhãn kiểu, vì nhãn đó có ẢNH HƯỞNG tới cách chuỗi được SERIALIZE
::why
Gần đúng ở việc bạn đoán ĐÚNG `typeof bienDo` — nhãn `__brand` không
đổi KIỂU CƠ SỞ của giá trị lúc chạy, `bienDo` thật sự là một `string`,
nên `typeof` trả `"string"`.

Chỗ lệch: `JSON.stringify` không hề "nhìn thấy" khai báo kiểu — nó chỉ
đọc GIÁ TRỊ THẬT lúc chạy, mà giá trị thật ở đây là một chuỗi trần
trụi, không mang field `__brand` nào để serialize. Đã đo: kết quả là
`"0312345678"` — dấu ngoặc kép bao quanh vì đó là cách `JSON.stringify`
biểu diễn MỌI chuỗi, không có gì khác với `JSON.stringify` một `string`
thường.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đoán đúng: brand KHÔNG "sống sót" một mẩu nào lúc chạy — không field,
không object bọc, không phép kiểm nào tự động xảy ra. Nó là một RÀNG
BUỘC chỉ trình biên dịch theo dõi, biến mất hoàn toàn khi sinh mã.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu brand không để lại DẤU VẾT nào lúc chạy, thì điều gì THẬT SỰ ngăn
một chuỗi bất kỳ — kể cả một chuỗi KHÔNG hợp lệ như
`"khong-phai-email"` — bị ép thành `Email` qua `as Email`? Ai viết dòng
đó cũng được, không ai kiểm tra gì cả.

Bài sau trả lời: một mẫu để CHỈ CÓ MỘT CHỖ DUY NHẤT trong cả chương
trình được viết `as Email`.
::::

::::checkpoint{mastery=0.8}
::::
