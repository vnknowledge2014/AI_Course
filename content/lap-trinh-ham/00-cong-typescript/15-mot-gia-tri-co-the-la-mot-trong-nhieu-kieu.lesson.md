---
id: lap-trinh-ham.cong-typescript.mot-gia-tri-co-the-la-mot-trong-nhieu-kieu
title: "Một giá trị có thể là MỘT TRONG NHIỀU kiểu"
summary: "`x: string | number` — dấu `|` nói \"x là string HOẶC number, không biết trước cái nào\". Không có gì tương ứng trực tiếp trong Python; ở đây phải NÊU RÕ tập kiểu khả dĩ, không phải bỏ ngỏ."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 15
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.union-type]
requires: [ts.handle-optional-safely]
concepts: [ts.union-type]
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
Bài trước bạn kiểm một mảng có thể RỖNG trước khi đọc. Hôm nay một câu
hỏi khác: nếu một cái tên có thể giữ MỘT TRONG HAI kiểu khác nhau —
không phải "có hoặc không có giá trị", mà "kiểu nào cũng được, một
trong hai" — TypeScript nói về điều đó thế nào?
::::

::::explain{#mot-trong-nhieu-kieu}
Mười bốn bài trước, mỗi lời hứa kiểu chỉ nhắc tới ĐÚNG MỘT kiểu:
`string`, `number`, `Diem[]`. Nhưng có những cái tên mà bản thân bài
toán đòi phải giữ MỘT TRONG NHIỀU kiểu — một mã đơn hàng lúc là số
(`1001`), lúc là chuỗi (`"DH-2024"`), tuỳ hệ thống nào tạo ra nó.

Python không cần nói gì đặc biệt cho việc này — MỌI biến Python vốn dĩ
đã có thể giữ bất cứ kiểu gì, bất cứ lúc nào, không cần khai báo. Đó là
lý do track này không hề dạy có một dấu hiệu Python "tương đương" —
không có, vì Python không cần NÊU RÕ trước.

TypeScript thì khác: nó đòi bạn NÊU RÕ tập kiểu khả dĩ, viết bằng dấu
gạch đứng `|` (đọc là "hoặc"), gọi là **union type** (kiểu hợp):

```
let ma_don_hang: number | string = 1001;
```

Dòng này hứa: `ma_don_hang` LUÔN LUÔN là `number` HOẶC `string` — không
phải "kiểu gì cũng được" như Python, mà đúng HAI khả năng đã liệt kê
sẵn, không hơn không kém. Gán một giá trị thuộc MỘT TRONG HAI kiểu đó
thì hợp lệ; gán một kiểu thứ ba (như `boolean`) thì bị từ chối, y hệt
mọi lời hứa kiểu khác từ đầu track.
::::

::::example{#gan-ca-hai-kieu}
Cùng một biến `ma_don_hang`, khai kiểu `number | string`, gán lại hai
lần bằng hai kiểu khác nhau:

```typescript title=readonly
let ma_don_hang: number | string = 1001;
console.log("Đơn đầu:", ma_don_hang);
ma_don_hang = "DH-2024";
console.log("Đơn sau:", ma_don_hang);
```

```text title=readonly
Đơn đầu: 1001
Đơn sau: DH-2024
```

Gán từ `number` (`1001`) sang `string` (`"DH-2024"`) — cả hai đều nằm
trong tập `number | string`, nên TypeScript im lặng cho qua, dù kiểu
THẬT SỰ của giá trị đã đổi giữa chừng.

Còn nếu gán một kiểu KHÔNG có trong lời hứa:

```typescript title=readonly
let ma_don_hang: number | string = 1001;
ma_don_hang = true;
```

```text title=readonly
(không in ra gì cả)

TS2322 (dòng 2, cột 1): Type 'boolean' is not assignable to type 'string | number'.
```

`boolean` không nằm trong `number | string` — TypeScript từ chối bằng
đúng mã lỗi `TS2322` đã gặp từ bài 3, chỉ khác ở chỗ vế "kiểu đã hứa"
giờ liệt kê HAI kiểu thay vì một.
::::

::::predict{#doan-dong-nao-bi-chan commitOnce}
Byte viết năm dòng, KHÔNG chạy thử:

```typescript
let A: string | number = 5;
let B: string | number = "năm";
A = B;
B = A;
A = true;
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng 5 (`A = true;`) — `boolean` không nằm trong tập `string | number`
mà `A` đã hứa, nên bị từ chối. Ba dòng gán trước đó (`A = B`, `B = A`)
đều hợp lệ, vì cả `A` lẫn `B` cùng khai kiểu `string | number`, và giá
trị bên trong luôn nằm gọn trong tập đó
:::

:::opt
Dòng 3 (`A = B;`) — không được gán một biến union type sang một biến
union type khác, dù cả hai khai cùng một tập kiểu
::why
Gần đúng ở việc bạn nghi ngờ có gì đặc biệt khi cả hai vế đều là union
type.

Chỗ lệch: TypeScript không cấm việc đó. `B` khai `string | number`,
giá trị của nó LUÔN nằm trong tập đó — gán sang `A` (cũng khai
`string | number`) không thể phá lời hứa nào cả. Đã thử thật: dòng 3
không bị từ chối.
::
:::

:::opt
Dòng 4 (`B = A;`) — tại thời điểm này `A` đã được gán lại ở dòng 3, nên
kiểu của nó không còn rõ ràng để gán tiếp cho `B`
::why
Gần đúng ở việc bạn để ý dòng 3 và dòng 4 đứng liền nhau và có vẻ phụ
thuộc nhau.

Chỗ lệch: dù `A` vừa được gán lại, kiểu ĐÃ HỨA của `A` (`string |
number`) không đổi — chỉ giá trị bên trong đổi, không phải kiểu. `B =
A` vẫn hợp lệ với đúng lý do dòng 3 hợp lệ. Đã thử thật, dòng 4 không
bị từ chối.
::
:::

:::opt
Không dòng nào bị từ chối — một khi đã khai `string | number`, biến đó
nhận giá trị gì cũng được, kể cả `boolean`, vì union type là một cách
"nới lỏng" kiểu ra
::why
Gần đúng ở cảm giác union type "nới lỏng" hơn một kiểu đơn — đúng là nó
CHO PHÉP nhiều kiểu hơn một kiểu đơn.

Chỗ lệch: "nhiều hơn một" không phải "bất cứ kiểu gì". Union type vẫn
là một lời hứa CÓ GIỚI HẠN — chỉ đúng những kiểu đã liệt kê trong dấu
`|`. `boolean` chưa từng được liệt kê, nên dòng 5 vẫn bị từ chối thật,
đúng mã lỗi `TS2322`.
::
:::
::::

::::code{#doi-ma-don-hang}
Một mã đơn hàng bắt đầu là số, sau đó hệ thống đổi nó thành một mã
chữ. Điền chỗ trống để đổi đúng kiểu, vẫn nằm trong lời hứa
`number | string`.

```typescript title=starter
let ma_don_hang: number | string = 1001;
console.log("Đơn đầu:", ma_don_hang);

ma_don_hang = ___;                   // đổi mã đơn thành "DH-2024"
console.log("Đơn sau:", ma_don_hang);
```

```typescript title=solution
let ma_don_hang: number | string = 1001;
console.log("Đơn đầu:", ma_don_hang);

ma_don_hang = "DH-2024";
console.log("Đơn sau:", ma_don_hang);
```

```typescript title=test
if (ma_don_hang !== "DH-2024") {
  throw new Error("ma_don_hang phải là \"DH-2024\" sau khi gán lại — đang là " + ma_don_hang);
}
```

:::hints
- kind: attention
  body: ma_don_hang khai kiểu number | string — chỗ trống có thể là number HOẶC string, miễn nằm trong tập đó. Đề bài đòi đổi nó thành mã CHỮ "DH-2024".
- kind: strategy
  body: 'Byte đổi mã đơn thành "DH-2024" — một chuỗi. string nằm trong tập number | string đã khai, nên gán trực tiếp chuỗi này là hợp lệ.'
- kind: one-line
  body: 'Chỗ trống là: "DH-2024"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Đơn sau: DH-2024"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một biến, hai kiểu khả dĩ, không kiểu nào bị bỏ ngỏ. TypeScript vẫn
biết chính xác kiểu nào được phép, kiểu nào không.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa GÁN được cả hai kiểu vào một biến `string | number` — điều đó
ổn. Nhưng nếu thay vì gán, bạn muốn GỌI một phương thức chỉ tồn tại
trên MỘT TRONG HAI kiểu — ví dụ `.toUpperCase()`, vốn chỉ có ở
`string`, không có ở `number` — TypeScript có cho gọi tự do không, hay
đòi thêm một bước nào đó trước?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
