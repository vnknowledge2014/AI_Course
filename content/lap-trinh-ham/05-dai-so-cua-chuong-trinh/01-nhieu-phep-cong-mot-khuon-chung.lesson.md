---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.nhieu-phep-cong-mot-khuon-chung
title: "Cộng số, nối chuỗi, gộp mảng — MỘT khuôn chung ẩn bên dưới"
summary: "1+2, \"a\"+\"b\", [1,2].concat([3,4]) — ba phép toán trông khác hẳn nhau, nhưng CÙNG hình dạng: nhận HAI giá trị CÙNG KIỂU, trả về MỘT giá trị CÙNG KIỂU ĐÓ."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [alg.spot-the-pattern]
requires: [fp.reduce-basics]
concepts: [alg.spot-the-pattern]
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
Track trước (T4.3) dạy một kiểu dữ liệu có thể mang nhiều HÌNH DẠNG.
Track này hỏi một câu khác: nhiều ĐOẠN MÃ trông khác hẳn nhau — có khi
nào chúng cùng CHUNG một khuôn không?
::::

::::explain{#ba-vi-du-mot-khuon}
```typescript
console.log(1 + 2);
console.log("a" + "b");
console.log([1, 2].concat([3, 4]));
```

```text
3
ab
[1,2,3,4]
```

Ba dòng, ba phép toán TRÔNG hoàn toàn khác nhau — một phép cộng số,
một phép nối chuỗi, một phép gộp mảng. Nhưng nhìn kỹ HÌNH DẠNG (không
phải Ý NGHĨA), cả ba giống hệt nhau:

- Nhận ĐÚNG HAI giá trị.
- Cả hai giá trị CÙNG một kiểu (`number` + `number`, `string` +
  `string`, `number[]` + `number[]`).
- Trả về MỘT giá trị, CÙNG kiểu đó (`number`, `string`, `number[]`).

Không có phép nào ở đây TRỘN kiểu (không cộng `number` với `string`),
không có phép nào ĐỔI kiểu (kết quả luôn CÙNG kiểu với đầu vào). Bài
này chưa đặt tên cho khuôn đó — chỉ dừng lại ở việc THẤY nó lặp lại ba
lần, ở ba chỗ tưởng chừng chẳng liên quan gì nhau.
::::

::::example{#them-vi-du-khac-cung-khuon}
Khuôn đó không dừng ở ba ví dụ trên — nó xuất hiện ở nhiều chỗ khác,
kể cả những nơi không "cộng" theo nghĩa thông thường:

```typescript title=readonly
console.log(Math.max(3, 7));
console.log(true && false);
console.log(Math.min(10, 4));
```

```text title=readonly
7
false
4
```

`Math.max(a, b)` — nhận HAI số, trả MỘT số (số lớn hơn). `true && false`
— nhận HAI boolean, trả MỘT boolean. `Math.min(a, b)` — nhận HAI số,
trả MỘT số (số nhỏ hơn). Không có phép nào ở đây "cộng" theo nghĩa số
học — nhưng CẢ BA vẫn đúng khuôn: hai giá trị CÙNG kiểu vào, một giá
trị CÙNG kiểu đó ra. Khuôn này RỘNG hơn "phép cộng" rất nhiều.
::::

::::predict{#doan-ket-hop-lien-tiep commitOnce}
```typescript
console.log(Math.max(3, 7));
console.log(Math.max(Math.max(3, 7), 2));
```

Hai dòng in ra gì?

:::opt{correct}
`7` rồi `7`
:::

:::opt
`7` rồi `2` — vì `Math.max` chỉ so sánh ĐÚNG HAI đối số ngay trước nó,
không "nhớ" kết quả của lần gọi trước
::why
Gần đúng ở việc bạn tính đúng dòng ĐẦU (`Math.max(3, 7) = 7`) — phép so
sánh đó đúng.

Chỗ lệch: dòng THỨ HAI là `Math.max(Math.max(3, 7), 2)` — biểu thức
LỒNG, `Math.max(3, 7)` chạy TRƯỚC (ra `7`), rồi kết quả đó (`7`) được
đưa làm đối số ĐẦU cho lời gọi `Math.max` NGOÀI: `Math.max(7, 2)`. Số
lớn hơn giữa `7` và `2` VẪN là `7`, không phải `2`.
::
:::

:::opt
Máy báo lỗi — `Math.max` không nhận được một lời gọi `Math.max` KHÁC
làm đối số của chính nó
::why
Gần đúng ở việc bạn cảnh giác về việc LỒNG một lời gọi hàm vào chính
đối số của một lời gọi hàm khác — một cấu trúc trông có vẻ "đệ quy"
lạ mắt nếu chưa quen.

Chỗ lệch: đây KHÔNG phải đệ quy, và hoàn toàn hợp lệ — `Math.max(3,
7)` (biểu thức BÊN TRONG) được TÍNH TRƯỚC, ra một GIÁ TRỊ THÔNG THƯỜNG
(`7`), rồi giá trị đó được dùng làm đối số cho `Math.max` BÊN NGOÀI —
y hệt cách `(2 + 3) + 4` tính `2+3` trước rồi mới cộng `4`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba phép toán trông chẳng liên quan — cùng một khuôn. Bạn vừa THẤY nó
lần đầu. Track này dành 54 bài để tìm hiểu VÌ SAO khuôn đó quan trọng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Math.max(7, 2)` gọi HAI LẦN LIÊN TIẾP (bài predict vừa rồi) vẫn ra kết
quả ĐÚNG, dù gọi theo thứ tự nào (`Math.max(Math.max(3,7), 2)` hay
`Math.max(3, Math.max(7,2))` — cả hai đều ra `7`, dù bạn chưa thử).
Có phải phép GỘP nào cũng có một tính chất ĐẶC BIỆT: một giá trị "trung
tính", kết hợp với nó thì KHÔNG đổi gì cả?

Bài sau tìm giá trị đó cho từng phép toán đã gặp.
::::

::::checkpoint{mastery=0.8}
::::
