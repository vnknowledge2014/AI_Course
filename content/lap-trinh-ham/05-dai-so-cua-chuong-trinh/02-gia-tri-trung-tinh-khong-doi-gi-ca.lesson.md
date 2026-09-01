---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.gia-tri-trung-tinh-khong-doi-gi-ca
title: "Giá trị TRUNG TÍNH — kết hợp với nó, không đổi gì cả"
summary: "5+0===5, \"chao\"+\"\"===\"chao\", [1,2].concat([]) giữ nguyên [1,2] — mỗi phép kết hợp có MỘT giá trị đặc biệt: kết hợp với nó, không đổi gì. 0, \"\", [] — ba giá trị khác nhau, CÙNG một vai trò."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [alg.identity-element]
requires: [alg.spot-the-pattern]
concepts: [alg.identity-element]
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
Bài trước để lại một câu hỏi: mỗi phép kết hợp có một giá trị "trung
tính" — kết hợp với nó, không đổi gì cả. Hôm nay tìm giá trị đó cho
từng phép toán đã gặp.
::::

::::explain{#gia-tri-trung-tinh}
```typescript
console.log(5 + 0);
console.log("chao" + "");
console.log([1, 2].concat([]));
```

```text
5
chao
[1,2]
```

Ba phép kết hợp bài trước đã thấy — mỗi phép có MỘT giá trị ĐẶC BIỆT:
`0` cho phép cộng, `""` (chuỗi rỗng) cho phép nối chuỗi, `[]` (mảng
rỗng) cho phép gộp mảng. Kết hợp BẤT KỲ giá trị nào với giá trị đặc
biệt này — kết quả VẪN LÀ giá trị gốc, không đổi gì:

`5 + 0 === 5`, `"chao" + "" === "chao"`, kết quả `[1,2].concat([])`
vẫn là `[1,2]`.

Ba giá trị này KHÁC NHAU (`0` là số, `""` là chuỗi, `[]` là mảng) —
nhưng CÙNG một VAI TRÒ: "trung tính" đối với đúng PHÉP KẾT HỢP của
kiểu dữ liệu đó. Không phải trùng hợp — mỗi phép kết hợp trong bài
trước ĐỀU có một giá trị trung tính riêng của nó.
::::

::::example{#trung-tinh-o-hai-phia}
Giá trị trung tính hoạt động ở CẢ HAI phía — không quan trọng nó đứng
trước hay sau:

```typescript title=readonly
console.log(0 + 5);
console.log(5 + 0);
console.log("" + "chao");
console.log("chao" + "");
```

```text title=readonly
5
5
chao
chao
```

`0 + 5` và `5 + 0` CÙNG ra `5` — giá trị trung tính không phân biệt
đứng TRƯỚC hay SAU giá trị kia, kết quả luôn là giá trị KHÔNG PHẢI nó.
Đây là một tính chất RIÊNG của giá trị trung tính, không phải mọi giá
trị đều có tính chất này (`5 + 3` và `3 + 5` cùng ra `8` — phép CỘNG
số THÌ có tính chất này với MỌI cặp số, nhưng không phải phép kết hợp
nào cũng vậy — phép TRỪ, `5 - 3` khác `3 - 5`, không "đối xứng" như
cộng).
::::

::::predict{#doan-trung-tinh-cua-max commitOnce}
```typescript
console.log(Math.max(5, -Infinity));
console.log(Math.max(-3, -Infinity));
```

Hai dòng in ra gì?

:::opt{correct}
`5` rồi `-3`
:::

:::opt
`5` rồi `0` — vì giá trị trung tính của MỌI phép toán số học đều là
`0`, kể cả `Math.max`
::why
Gần đúng ở việc bạn tính đúng dòng ĐẦU (`Math.max(5, -Infinity) = 5`,
vì `5` lớn hơn `-Infinity`) — kết quả đó đúng.

Chỗ lệch: `0` KHÔNG phải giá trị trung tính của `Math.max` — thử
`Math.max(-3, 0)` sẽ ra `0`, KHÔNG giữ nguyên `-3` (`0` LỚN HƠN `-3`,
nên nó "thắng", làm ĐỔI kết quả — trái với định nghĩa "trung tính:
kết hợp thì không đổi gì"). Giá trị trung tính THẬT của `Math.max` là
`-Infinity` — SO VỚI BẤT KỲ số nào, số đó luôn LỚN HƠN `-Infinity`,
nên `Math.max(x, -Infinity)` LUÔN ra `x`, không đổi. `Math.max(-3,
-Infinity)` ra `-3`, không phải `0`.
::
:::

:::opt
Máy báo lỗi — `-Infinity` không phải một `number` hợp lệ để truyền vào
`Math.max`
::why
Gần đúng ở việc bạn cảnh giác về việc `Infinity` có thể không phải một
số "bình thường" — một mối lo hợp lý nếu chưa quen với nó trong
JavaScript/TypeScript.

Chỗ lệch: `Infinity`/`-Infinity` LÀ những giá trị `number` HOÀN TOÀN
HỢP LỆ trong JavaScript/TypeScript (kết quả của việc chia cho `0`, hay
viết trực tiếp như ở đây) — dùng được ở BẤT KỲ đâu một `number` được
chấp nhận, kể cả làm đối số cho `Math.max`, không lỗi gì.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mỗi phép kết hợp có một giá trị trung tính RIÊNG — không có công thức
chung "luôn là `0`". Phải hiểu ĐÚNG phép kết hợp mới tìm đúng nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã thấy: một phép KẾT HỢP (nhận hai giá trị cùng kiểu, trả một giá
trị cùng kiểu) VÀ một giá trị TRUNG TÍNH riêng của nó. Hai thứ này luôn
đi CÙNG NHAU — có cách nào GÓI GỌN cả hai vào MỘT khai báo TypeScript
duy nhất không?

Bài sau đặt tên và định nghĩa hình thức cho cặp đó.
::::

::::checkpoint{mastery=0.8}
::::
