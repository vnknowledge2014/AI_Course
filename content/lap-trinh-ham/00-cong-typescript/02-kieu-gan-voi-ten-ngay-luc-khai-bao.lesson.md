---
id: lap-trinh-ham.cong-typescript.kieu-gan-voi-ten-ngay-luc-khai-bao
title: "Kiểu gắn liền với cái tên NGAY LÚC khai báo"
summary: "`let n: number = 5` — dấu hai chấm sau tên là một LỜI HỨA: biến này SẼ LUÔN là số. Python không có lời hứa này — một biến ở đó giữ bất cứ kiểu gì, bất cứ lúc nào. Đây là Ý thật sự mới của cả track."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.type-annotation]
requires: [ts.syntax-bridge]
concepts: [ts.type-annotation]
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
Dấu hai chấm sau tên biến — cái bạn hỏi ở cuối bài trước. Hôm nay Byte
gọi tên nó, và cho biết nó đòi hỏi gì.
::::

::::explain{#mot-loi-hua-that}
Trong Python, một biến có thể giữ BẤT CỨ KIỂU GÌ, ở BẤT CỨ LÚC NÀO.
`tuoi = 10` rồi `tuoi = "mười"` sau đó — Python không phàn nàn một chữ.
Cùng một cái tên, kiểu đổi tuỳ ý, tuỳ dòng.

TypeScript cho bạn một công cụ Python không có: viết thêm một dấu hai
chấm và một TÊN KIỂU ngay sau tên biến, TRƯỚC dấu `=`:

```
let n: number = 5;
```

Đọc thành lời: "`n` LUÔN LUÔN là một `number`." Không phải một gợi ý,
không phải một ghi chú cho người đọc — một **lời hứa** mà TypeScript
GHI NHỚ và sẽ KIỂM ở mọi dòng sau, suốt phần đời còn lại của `n` trong
chương trình.

Ba tên kiểu cơ bản nhất, dùng ngay từ hôm nay:

- `number` — mọi con số, nguyên hay thập phân, TypeScript không phân
  biệt hai loại đó như một số ngôn ngữ khác.
- `string` — chuỗi ký tự, viết trong `"..."` hoặc `'...'`.
- `boolean` — chỉ hai giá trị `true` / `false`.

Cú pháp chung: `<từ khoá let/const> <tên>: <kiểu> = <giá trị>;` — dấu
hai chấm luôn đứng NGAY SAU tên, TRƯỚC dấu bằng.
::::

::::example{#ba-loi-hua}
Ba biến, ba lời hứa khác nhau:

```typescript title=readonly
let n: number = 5;
let ten: string = "Byte";
let dang_hoc: boolean = true;

console.log(n, ten, dang_hoc);
```

```text title=readonly
5 Byte true
```

Ba dòng, ba kiểu tên khác nhau, đúng ba lời hứa. Giá trị bên phải dấu
`=` khớp đúng kiểu đã hứa bên trái — TypeScript không nói gì cả, im
lặng cho qua. Bài sau sẽ cho xem điều gì xảy ra khi giá trị KHÔNG khớp.
::::

::::predict{#dung-cu-phap-loi-hua commitOnce}
Byte muốn viết lại `diem_thi = 85` (Python) bằng TypeScript, VÀ thêm
một lời hứa: `diem_thi` sẽ luôn là `number`. **Trước khi đọc đáp án**,
cách viết nào đúng?

:::opt{correct}
`let diem_thi: number = 85;` — dấu hai chấm ngay sau tên, tên kiểu
ngay sau dấu hai chấm, rồi mới tới dấu `=` và giá trị
:::

:::opt
`let diem_thi = number: 85;` — viết tên kiểu ngay trước giá trị, sau
dấu bằng
::why
Gần đúng ở việc bạn nhớ đúng CẢ HAI mảnh cần có: tên kiểu `number` và
giá trị `85` — chỉ là đặt sai chỗ.

Chỗ lệch: dấu hai chấm khai kiểu phải đứng NGAY SAU TÊN BIẾN, trước
dấu `=` — không phải chen giữa dấu `=` và giá trị. Đã thử thật: dòng
này không biên dịch được, TypeScript báo lỗi cú pháp "',' expected."
ngay tại vị trí dấu hai chấm đặt sai chỗ.
::
:::

:::opt
`let number diem_thi: 85;` — viết tên kiểu trước, tên biến sau, rồi
dấu hai chấm và giá trị
::why
Gần đúng ở việc cả ba mảnh `number`, `diem_thi`, `85` đều có mặt đầy
đủ trong dòng này.

Chỗ lệch: TypeScript đọc tên biến TRƯỚC, dấu hai chấm và kiểu SAU —
không phải kiểu đứng trước tên. Đã thử thật: dòng này không biên dịch
được, TypeScript báo lỗi cú pháp "',' expected." ngay sau `number`.
::
:::

:::opt
`let diem_thi number = 85;` — viết tên biến rồi tên kiểu liền nhau,
không cần dấu hai chấm ở giữa
::why
Gần đúng ở việc bạn đặt đúng thứ tự: tên biến trước, tên kiểu sau —
chỉ thiếu đúng một dấu.

Chỗ lệch: TypeScript CẦN dấu hai chấm để biết `number` là một lời hứa
về KIỂU, không phải một tên biến thứ hai. Thiếu nó, TypeScript đọc
`diem_thi` và `number` như hai tên đứng cạnh nhau vô nghĩa. Đã thử
thật: lỗi cú pháp "',' expected." ngay sau `diem_thi`.
::
:::
::::

::::code{#ba-loi-hua-cua-byte}
Byte cần ba biến, mỗi biến một lời hứa kiểu khác nhau. Điền đúng giá
trị khớp từng kiểu đã khai — TypeScript sẽ từ chối biên dịch nếu bạn
điền sai kiểu.

```typescript title=starter
let ten: string = ___;
let tuoi: number = ___;
let dang_hoc: boolean = ___;

console.log(ten, tuoi, dang_hoc);
```

```typescript title=solution
let ten: string = "Byte";
let tuoi: number = 5;
let dang_hoc: boolean = true;

console.log(ten, tuoi, dang_hoc);
```

```typescript title=test
if (ten !== "Byte") throw new Error("ten phải là chuỗi \"Byte\" — đang là " + ten);
if (tuoi !== 5) throw new Error("tuoi phải là 5 — đang là " + tuoi);
if (dang_hoc !== true) throw new Error("dang_hoc phải là true — đang là " + dang_hoc);
```

:::hints
- kind: attention
  body: Ba chỗ trống, mỗi chỗ là một GIÁ TRỊ khớp đúng kiểu đã khai bên trái — ten cần một chuỗi, tuoi cần một số, dang_hoc cần true hoặc false.
- kind: strategy
  body: 'Byte tên là "Byte", 5 tuổi tính theo track, và đang thật sự học — true. Điền đúng ba giá trị đó, đúng kiểu chuỗi/số/boolean khớp với dấu hai chấm bên trái mỗi dòng.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là: "Byte", 5, true'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Byte 5 true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lời hứa, ba kiểu, không dòng nào bị phàn nàn — vì cả ba đều giữ
đúng lời hứa đã khai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa viết ba lời hứa, và giữ đúng cả ba — không dòng nào phá vỡ
điều nó đã hứa. Nhưng nếu một dòng PHÁ VỠ lời hứa đó thì sao — ví dụ
gán một chuỗi cho một biến đã hứa là `number`?

Chương trình có chạy hết những dòng phía trước, rồi mới báo lỗi ở đúng
dòng đó — giống hệt cách Python vẫn làm với `TypeError` — hay có gì
khác hẳn? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
