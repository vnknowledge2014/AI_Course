---
id: toan.cam-nhan-so.viet-gon-phep-nhan-lap-lai
title: Viết gọn phép nhân lặp
summary: 10⁵ nghĩa là mười nhân với chính nó, năm thừa số. Số mũ đếm số thừa số — không phải kết quả, cũng không phải số dấu nhân.
locale: vi
track: toan
module: cam-nhan-so
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.exponent]
requires: [math.multiply-as-scaling, math.multiplication, math.dong-goi, core.variable, core.assignment, core.arithmetic, core.number-literal, core.print-variable, core.output]
concepts: [math.luy-thua, math.co-so, math.so-mu]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Hai mươi con 10 cạnh nhau thì mình đếm nhầm mất. Viết cách khác được không?
::::

::::explain{#hai-muoi-con-10-canh-nhau}
Bài trước để lại một sự bất tiện rất cụ thể. Mỗi lượt kéo giãn thanh số 10 lần
thì viết `10`. Làm hai lượt liên tiếp thì `10 × 10`. Ba lượt thì
`10 × 10 × 10`. Năm lượt:

```text
10 × 10 × 10 × 10 × 10
```

Tay mỏi chưa thì chưa chắc, nhưng có một chuyện tệ hơn mỏi tay: **nhìn vào dòng
ấy, bạn không đếm ngay ra được có mấy con 10.** Phải rà từng cái. Tới hai mươi
lượt thì rà xong lại nghi mình đếm nhầm, và người đọc sau bạn cũng thế.

Chỗ bất tiện nằm ở đâu? Ở chỗ dòng chữ ấy **lặp lại** đúng một thứ — con số 10
— rồi bắt người đọc tự đếm xem nó lặp mấy lần. Vậy viết thẳng cả hai điều ra:
lặp cái gì, và lặp mấy lần.

```text
10 × 10 × 10 × 10 × 10   viết gọn thành   10⁵
```

Đọc là "**mười mũ năm**". Hai vai của hai con số:

- **`10`** viết cỡ bình thường, gọi là **cơ số** — nó nói *lặp lại cái gì*, tức
  mỗi lượt kéo giãn mấy lần.
- **`5`** viết nhỏ ở trên, gọi là **số mũ** — nó **đếm xem có bao nhiêu thừa
  số** `10` trong tích, tức làm bao nhiêu lượt.

Cả cụm gọi là một **luỹ thừa**. Nó không phải một phép tính mới: nó là cách viết
gọn của một phép nhân bạn đã biết từ bài 19, và mọi tính chất của phép nhân vẫn
nguyên vẹn trong đó.

Python viết luỹ thừa bằng **hai dấu sao liền nhau**: `10 ** 5`.
::::

::::explain{#so-mu-dem-thua-so}
Đây là chỗ dễ trượt nhất của cả bài, nên nói thẳng ra: **số mũ đếm thừa số.**
Nó không phải kết quả, và nó cũng không đếm dấu nhân.

Trong `10 × 10 × 10 × 10 × 10` có **năm** con 10, nhưng chỉ có **bốn** dấu nhân
— dấu nhân luôn ít hơn thừa số đúng một cái, vì nó đứng ở khe giữa hai con số.
Số mũ bám theo thừa số, không bám theo dấu nhân. Bằng chứng gọn nhất:

| Viết gọn | Viết đủ | Mấy thừa số | Mấy dấu nhân | Bằng |
|---|---|---|---|---|
| `10¹` | `10` | 1 | 0 | 10 |
| `10²` | `10 × 10` | 2 | 1 | 100 |
| `10³` | `10 × 10 × 10` | 3 | 2 | 1000 |
| `10⁴` | `10 × 10 × 10 × 10` | 4 | 3 | 10000 |
| `10⁵` | `10 × 10 × 10 × 10 × 10` | 5 | 4 | 100000 |

Dòng đầu đáng nhìn kỹ. `10¹` có **một** thừa số và **không** dấu nhân nào — chưa
nhân lần nào cả, nên nó bằng đúng 10. Người quen nghĩ "mũ mấy là nhân mấy lần"
sẽ vấp ngay ở dòng này.

Còn một chuyện dễ chịu ở cột cuối: `10⁵` bằng `100000`, và con số ấy có đúng
**năm** chữ số 0. Số mũ đếm thừa số, mà mỗi thừa số 10 lại thêm đúng một chữ số
0 vào đuôi — nên với cơ số 10 thì số mũ đọc thẳng ra từ số chữ số 0.

Cơ số không bắt buộc phải là 10. Byte gấp đôi số hạt mỗi ngày thì cơ số là 2:

```text
2¹ = 2        2² = 2 × 2 = 4       2³ = 2 × 2 × 2 = 8
5³ = 5 × 5 × 5 = 125
```

Và nhìn `2³` trên thanh số của bài 22 thì nó là ba lượt kéo giãn liên tiếp, mỗi
lượt gấp đôi: vạch 1 đi từ 1 tới 2, từ 2 tới 4, từ 4 tới 8. Cùng một con số 8,
hai bức tranh — một cái là ba thừa số 2 xếp cạnh nhau, một cái là ba lần bàn tay
kéo sợi dây thun.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Hai dấu sao liền nhau là dấu mũ, không phải hai phép nhân. Nhìn kỹ rồi hẵng đoán.
::::

::::predict{#doan-ba-luy-thua commitOnce}
Byte hỏi máy ba luỹ thừa. Nhớ rằng `**` trong Python là dấu mũ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(10 ** 5)
print(2 ** 3)
print(2 ** 1)
```

:::opt{correct}
100000, rồi 8, rồi 2
:::

:::opt
50, rồi 6, rồi 2
::why
Gần đúng ở chỗ bạn đọc hai con số theo đúng bài 19: một cái nói *lô to bao
nhiêu*, cái kia nói *lấy mấy lô*, rồi nhân hai vai ấy với nhau. Cách đọc đó đúng
cho dấu `*`, và `10 × 5` đúng là 50.

Chỗ lệch nằm ở việc dấu `**` hỏi một câu khác. Nó không hỏi "mấy lô", nó hỏi
"**mấy thừa số**". `10 ** 5` là năm con 10 nhân với nhau, không phải hai con số
nhân với nhau. Để ý dòng cuối: `2 ** 1` ra 2 theo cả hai cách đọc — nên một mình
dòng ấy không bao giờ cho bạn biết mình đang đọc đúng hay sai.
::
:::

:::opt
1000000, rồi 16, rồi 4
::why
Gần đúng ở chỗ bạn đếm **dấu nhân**, và đếm rất chuẩn: `10 × 10 × 10 × 10 × 10`
đúng là có bốn dấu nhân cho năm con 10, nên bạn hiểu số mũ 5 thành "nhân năm
lần" và viết ra sáu con 10.

Chỗ lệch là thứ được đếm. Số mũ đếm **thừa số**, không đếm dấu nhân — và hai con
số ấy luôn lệch nhau đúng một. Phép thử nhanh nhất là dòng `10¹`: nếu số mũ đếm
dấu nhân thì `10¹` phải là `10 × 10` bằng 100, trong khi ai cũng viết `10¹` là
10.
::
:::

:::opt
100000, rồi 8, rồi 1
::why
Gần đúng ở chỗ bạn nhận ra `2 ** 1` là ca đặc biệt: nó chưa nhân lần nào cả.
Quan sát ấy chính xác, và hai dòng trên bạn tính đúng hết.

Chỗ lệch: "chưa nhân lần nào" không có nghĩa là "chưa có gì". Số mũ 1 nói trong
tích có **đúng một** thừa số, và thừa số ấy là 2 — nó đứng đó một mình, nguyên
vẹn. Nên `2 ** 1` bằng 2.
::
:::
::::

::::example{#may-tra-loi}
Chạy lên, máy in ra:

```text
100000
8
2
```

`10 ** 5` là 100000 — một trăm nghìn, đúng năm chữ số 0. `2 ** 3` là 8, ba lượt
gấp đôi. `2 ** 1` là 2, một thừa số duy nhất.

Ba dòng, và không dòng nào cần bảng cửu chương: chúng chỉ là phép nhân lặp được
viết ngắn lại.
::::

::::code{#keo-gian-nhieu-luot}
Byte kéo sợi dây thun ở bài 22, nhưng lần này kéo **nhiều lượt liên tiếp**.
Mỗi lượt kéo giãn 10 lần. Bạn viết chỗ mà vạch 1 rơi vào sau 1 lượt, sau 3 lượt,
sau 5 lượt — bằng luỹ thừa, đừng nhân tay.

Dòng cuối đổi cơ số: mỗi lượt chỉ **gấp đôi**, làm 3 lượt.

Bốn chỗ trống, bốn số mũ khác nhau và hai cơ số khác nhau, nên không con số nào
chép được sang chỗ khác. Cách chấm còn đối chiếu một dòng với phép nhân viết đủ
của chính nó, để bảo đảm luỹ thừa bạn viết ra đúng là phép nhân lặp.

```python title=starter
sau_1_luot = ___
sau_3_luot = ___
sau_5_luot = ___
gap_doi_3_luot = ___

print(sau_1_luot)
print(sau_3_luot)
print(sau_5_luot)
print(gap_doi_3_luot)
```

```python title=solution
sau_1_luot = 10 ** 1
sau_3_luot = 10 ** 3
sau_5_luot = 10 ** 5
gap_doi_3_luot = 2 ** 3

print(sau_1_luot)
print(sau_3_luot)
print(sau_5_luot)
print(gap_doi_3_luot)
```

```python title=test
# Bốn tình huống: ba số mũ khác nhau trên cơ số 10, cộng thêm một cơ số khác.
# Hai câu cuối chốt lại đúng điều bài dạy — luỹ thừa CHỈ là phép nhân lặp viết
# gọn, và số mũ đếm thừa số chứ không phải một phép nhân với chính nó.
assert sau_1_luot == 10, "một lượt: đúng một thừa số 10, chưa nhân lần nào, nên bằng 10"
assert sau_3_luot == 1000, "ba lượt: 10 × 10 × 10, ba thừa số 10"
assert sau_5_luot == 100000, "năm lượt: năm thừa số 10, và con số ấy có đúng năm chữ số 0"
assert gap_doi_3_luot == 8, "ba lượt gấp đôi: 2 × 2 × 2, vạch 1 đi 1 → 2 → 4 → 8"
assert sau_3_luot == 10 * 10 * 10, "luỹ thừa là cách viết gọn của phép nhân lặp, không phải phép tính mới"
assert gap_doi_3_luot != 6, "số mũ không phải một thừa số thứ hai — 2 mũ 3 không phải 2 nhân 3"
```

:::hints
- kind: attention
  body: Mỗi dòng có hai con số phải nói ra: kéo giãn mấy lần trong MỘT lượt, và làm bao nhiêu lượt. Python đặt chúng hai bên hai dấu sao liền nhau.
- kind: strategy
  body: Cơ số đứng trước, số mũ đứng sau. Ba dòng đầu cùng cơ số 10 và chỉ khác nhau ở số lượt; dòng cuối giữ số lượt là 3 nhưng đổi cơ số vì mỗi lượt chỉ gấp đôi.
- kind: one-line
  body: "Bốn chỗ trống lần lượt là `10 ** 1`, `10 ** 3`, `10 ** 5` và `2 ** 3`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: bốn chỗ trống phải viết bằng dấu mũ `**`, không phải chép cứng kết quả — chép cứng thì cái bài này vừa dạy không xuất hiện ở đâu cả
  requireAst:
  # Bốn luỹ thừa, một cho mỗi chỗ trống. Điền bừa vào chỗ trống thì không có
  # dấu mũ nào, nên luật này phân biệt được đúng với sai.
  - kind: uses-operator, target: **, min: 4
- tier: output
  match: regex
  expect: ^10\n1000\n100000\n8\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm lượt kéo, năm chữ số 0. Giờ mình đếm bằng mắt, không phải rà từng con.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại cột cuối của cái bảng ban nãy, chỉ lấy cơ số 10:

```text
10¹ = 10        10² = 100        10³ = 1000
```

Mười, một trăm, một nghìn. Nhìn quen chưa? Đó đúng là mấy cái **cột** bạn đã kẻ
từ bài 7 để đọc một dãy chữ số: cột chục, cột trăm, cột nghìn. Bốn cái cột tưởng
là bốn thứ rời nhau, hoá ra chúng là một dãy luỹ thừa của cùng một con số 10 —
và cái luật đóng bó ở bài 6 là thứ nối chúng lại.

Nhưng bảng vị trí còn một cột nữa, cột đứng ngoài cùng bên phải: **cột hạt lẻ**,
cột "một". Một hạt chưa buộc vào bó nào.

Nếu cột chục là `10¹` và cột trăm là `10²`, thì cột "một" là 10 mũ mấy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
