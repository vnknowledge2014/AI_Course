---
id: toan.cam-nhan-so.cho-dung-noi-gia-tri
title: Chỗ đứng nói giá trị
summary: Cùng một chữ số 3, đứng cột khác thì mang lượng khác — vị trí gánh luôn việc phải viết tên đơn vị ra.
locale: vi
track: toan
module: cam-nhan-so
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.place-value]
requires: [math.dong-goi, math.thanh-so, core.arithmetic, core.number-literal, core.variable, core.assignment, core.print-variable, core.output]
concepts: [math.don-vi, math.bang-cot]
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
Mười ba bó với bảy hạt, viết liền thành 137. Mai đọc lại có ra đúng đống cũ không?
::::

::::explain{#moi-cot-mot-chu-so}
Bài trước dừng đúng ở chỗ bí: Byte có **13 bó và 7 hạt lẻ**, viết `13` cạnh `7`
thành `137`, rồi không biết mai mở sổ ra đọc lại có ra đúng đống cũ không.

Thử đọc lại xem. Ba người cầm cùng tờ giấy ấy:

- Người thứ nhất cắt thành `13` và `7`: 13 bó với 7 hạt lẻ → 130 + 7 = **137 hạt**.
- Người thứ hai cắt thành `1` và `37`: 1 bó với 37 hạt lẻ → 10 + 37 = **47 hạt**.
- Người thứ ba không cắt gì cả: **137 hạt lẻ**, chưa buộc bó nào.

Ba cách đọc, hai đống hạt khác hẳn nhau. Tờ giấy không hề nói nó là cách nào.

Chỗ hỏng nằm ở đâu? Ở chỗ con số `13` chiếm **hai ô** mà con `7` chỉ chiếm
**một**, nên không ai biết chỗ cắt nằm đâu. Muốn tờ giấy chỉ đọc được một cách,
phải thoả thuận trước một luật:

> Mỗi cột đúng **một** chữ số.

Một chữ số thì lớn nhất là 9. Vậy luật này đòi: không cột nào được chứa từ mười
trở lên. Mà kéo một cột tụt xuống dưới mười thì bài 6 làm rồi — đủ mười thì
buộc lên bó. Hoá ra việc đóng bó không chỉ để đếm cho nhẹ tay: **nó là điều
kiện để cách viết này đọc lại được**.

Byte đang có 13 bó, mà 13 thì lớn hơn 9. Bài 6 đã buộc lần thứ hai: mười bó
thành một bó-của-bó, còn dư 3 bó. Đống hạt bây giờ nằm gọn trong ba cột, mỗi
cột một chữ số:

| bó-của-bó | bó | hạt lẻ |
|---|---|---|
| 1 | 3 | 7 |

Viết liền ba ô lại: `137`. Vẫn ba chữ số ấy, nhưng lần này chỉ đọc được **một**
cách — vì không ô nào chứa quá một chữ số, nên không còn chỗ nào để cắt nhầm.
::::

::::explain{#vi-tri-thay-cho-ten}
Nhìn kỹ cái thứ vừa cứu Byte. Không phải mấy chữ số — chúng vẫn là 1, 3, 7 như
cũ. Thứ cứu Byte là **hàng ô đã thoả thuận trước**: ô ngoài cùng bên phải luôn
dành cho hạt lẻ, ô kế bên trái luôn dành cho bó (mười hạt), ô kế nữa luôn dành
cho bó-của-bó (một trăm hạt).

Khi mọi người đã thuộc hàng ô ấy thì không cần viết tên đơn vị ra nữa. Nhìn một
chữ số **đứng ở đâu** là biết nó đang đếm cái gì. Ý đó có tên: **giá trị theo
vị trí**.

Nên con số 3 không mang sẵn một lượng cố định. Cùng một chữ số 3:

- đứng cột hạt lẻ, nó là **3 hạt**;
- đứng cột bó, nó là ba cái bó, tức **30 hạt**;
- đứng cột bó-của-bó, nó là ba cái bó-của-bó, tức **300 hạt**.

Tiếng Việt thì vẫn đang đọc số theo kiểu cũ, gọi tên từng cột ra: "**một trăm
ba mươi bảy**" là *một* trăm, *ba* mươi, *bảy*. Dãy chữ số `137` bỏ hết mấy chữ
"trăm", "mươi" đi — và cái vị trí gánh nốt phần việc của chúng.

Còn một chỗ trả lãi nữa, cho câu hỏi bài 5 để lại. Ở đó, muốn chỉ vào chỗ của
137 trên **thanh số** thì phải đếm qua 137 cái vạch. Bảng cột trả lời luôn: ba
chữ số là ba **cỡ bước**. Một bước dài 100, ba bước dài 10, bảy bước dài 1 —
mười một bước là tới đúng chỗ, không bước nào phải đếm quá mười.
::::

::::example{#cung-chu-so-ba-cho-dung}
Hỏi thẳng máy xem chữ số 3 mang bao nhiêu hạt ở từng cột. Mỗi dòng là ba cái
đơn vị của cột ấy cộng lại:

```python title=readonly
print(3)
print(10 + 10 + 10)
print(100 + 100 + 100)
```

Máy in ra:

```text
3
30
300
```

Ba dòng, cùng một chữ số 3, ba lượng khác hẳn nhau. Thứ làm chúng khác nhau
không nằm trong chữ số — nó nằm ở **cỡ của cái đơn vị mà cột ấy đếm**.

Ráp cả ba cột của Byte lại thành một đống:

```python
print(100 + 10 + 10 + 10 + 7)
```

```text
137
```

Một bó-của-bó, ba bó, bảy hạt lẻ. Đúng đống hạt bài 6 đổ ra bàn, không thiếu
hạt nào.
::::

::::predict{#doan-hai-cach-doc commitOnce}
Byte và An cầm cùng một tờ giấy ghi `137`, nhưng cắt nó thành cột theo hai kiểu
khác nhau. Byte cắt thành `13` và `7`; An cắt thành `1` và `37`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
# Byte đọc là "13 bó và 7 hạt lẻ" — 13 bó buộc lại là 1 bó-của-bó và 3 bó
cach_byte = 100 + 10 + 10 + 10 + 7

# An đọc là "1 bó và 37 hạt lẻ"
cach_an = 10 + 37

print(cach_byte)
print(cach_an)
```

:::opt{correct}
137 rồi 47
:::

:::opt
137 rồi 137
::why
Gần đúng ở chỗ bạn giữ chắc điều bài 6 vừa dạy: buộc bó không làm mất hạt nào,
nên cùng một đống đem viết ra kiểu nào cũng vẫn chừng ấy hạt. Ý ấy đúng, và nó
là chỗ dựa của cả luật đóng gói.

Chỗ lệch nằm ở chiều. Ý ấy nói về **một đống** được viết ra **hai cách**. Ở đây
thì ngược lại: **một dãy chữ số** đang được đọc thành **hai đống**. 13 cái bó
là 130 hạt, còn 1 cái bó chỉ là 10 hạt — hai cách cắt lệch nhau đúng 90 hạt, và
đó chính là lý do tờ giấy của Byte cần một luật cắt cột.
::
:::

:::opt
47 rồi 137
::why
Gần đúng ở chỗ bạn nhớ đúng hai con số kết quả: 137 và 47 đúng là hai đống mà
hai cách đọc cho ra.

Chỗ lệch là gán nhầm cách nào cho ai. Dòng trên cắt `137` thành **13 | 7**, tức
phần lớn số hạt nằm ở cột bó — mười ba cái bó, đống to. Dòng dưới cắt thành
**1 | 37**, chỉ có một cái bó, phần còn lại là hạt rời — đống nhỏ hơn hẳn. Cứ
nhìn xem cột bó được mấy cái là biết dòng nào to hơn.
::
:::

:::opt
137 rồi 38
::why
Gần đúng ở chỗ bạn cộng đúng hai con số có mặt trong câu: một cái bó, ba mươi
bảy hạt lẻ, cộng lại là 38 **vật** đang nằm trên bàn. Đó là câu trả lời chính
xác cho câu hỏi mà bài 6 đã hỏi: trên bàn có mấy vật.

Chỗ lệch là câu hỏi ở đây khác. Ta đang hỏi đống ấy có mấy **hạt**. Một cái bó
cầm lên thì là một vật, nhưng đổ ra thì là mười hạt — nên nó góp 10 vào cột
hạt, không góp 1.
::
:::
::::

::::explain{#cho-dung-la-mot-phan-cua-so}
Rút ra một câu để mang theo: **chỗ đứng của một chữ số là một phần của con số,
không phải chuyện trình bày cho đẹp.**

Đổi chỗ hai chữ số là đổi luôn đống hạt. `45` là 4 bó với 5 hạt lẻ; `54` là 5
bó với 4 hạt lẻ. Vẫn đúng hai chữ số 4 và 5, mà hai đống lệch nhau 9 hạt.

Cách viết này đứng được là nhờ ba thoả thuận, thiếu cái nào cũng sập:

1. Hàng ô có thứ tự cố định, và **đếm từ phải sang trái**: hạt lẻ, rồi bó, rồi
   bó-của-bó, rồi bó-của-bó-của-bó.
2. Mỗi cột đúng một chữ số — nên phải đóng bó cho tới khi không cột nào còn đủ
   mười.
3. Mỗi cột to gấp mười cột bên phải nó. Đó là luật bài 6, đứng nguyên tại chỗ.
::::

::::code{#ghi-so-vuon}
Byte ghi sổ vườn. Mỗi lần ghi, Byte đếm ra từng cột rồi viết đống ấy là **mấy
hạt**. Một bó là 10 hạt, một bó-của-bó là 100 hạt.

Ba ngày dưới đây được chọn để không ngày nào chép được của ngày nào — hai ngày
đầu dùng đúng hai chữ số ấy, chỉ đổi chỗ:

- **Thứ Hai**: 4 bó, 5 hạt lẻ
- **Thứ Ba**: 5 bó, 4 hạt lẻ
- **Thứ Tư**: 1 bó-của-bó, 3 bó, 7 hạt lẻ

```python title=starter
# Thứ Hai: 4 bó, 5 hạt lẻ
thu_hai = ___

# Thứ Ba: 5 bó, 4 hạt lẻ
thu_ba = ___

# Thứ Tư: 1 bó-của-bó, 3 bó, 7 hạt lẻ
thu_tu = ___

print(thu_hai)
print(thu_ba)
print(thu_tu)
```

```python title=solution
# Thứ Hai: 4 bó, 5 hạt lẻ
thu_hai = 10 + 10 + 10 + 10 + 5

# Thứ Ba: 5 bó, 4 hạt lẻ
thu_ba = 10 + 10 + 10 + 10 + 10 + 4

# Thứ Tư: 1 bó-của-bó, 3 bó, 7 hạt lẻ
thu_tu = 100 + 10 + 10 + 10 + 7

print(thu_hai)
print(thu_ba)
print(thu_tu)
```

```python title=test
# Ba ngày, ba con số khác nhau: một con số gõ cứng vào cả ba chỗ thì trượt ít
# nhất hai câu. Câu cuối chốt lại đúng điều bài này dạy — cùng hai chữ số 4 và
# 5, chỉ đổi chỗ, mà hai đống lệch nhau 9 hạt.
assert thu_hai == 45, "4 bó là bốn lần mười hạt, cộng thêm 5 hạt lẻ"
assert thu_ba == 54, "5 bó là năm lần mười hạt, cộng thêm 4 hạt lẻ"
assert thu_tu == 137, "1 bó-của-bó là 100 hạt, 3 bó là 30 hạt, thêm 7 hạt lẻ nữa"
assert thu_hai + 9 == thu_ba, "đổi chỗ hai chữ số 4 và 5 làm đống hạt lệch đúng 9 hạt"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống hỏi đống ấy là mấy HẠT, không hỏi mấy bó. Nên trước khi viết, đổi từng cột về hạt đã — cột nào cũng phải góp phần của mình vào.
- kind: strategy
  body: Một cái bó là mười hạt, nên bốn cái bó là bốn lần mười hạt cộng lại. Một cái bó-của-bó là một trăm hạt. Cộng phần của từng cột lại là ra cả đống. Phần máy in ở trên đã làm sẵn phép cộng ấy cho ngày thứ Tư.
- kind: one-line
  body: "Chỗ trống thứ nhất viết `10 + 10 + 10 + 10 + 5`, chỗ thứ hai viết `10 + 10 + 10 + 10 + 10 + 4`, chỗ thứ ba viết `100 + 10 + 10 + 10 + 7`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi ngày phải được ráp lại từ các CỘT của nó — gõ thẳng con số cuối cùng vào thì bài không nhìn thấy cái cột nào cả
  requireAst:
  # Khung chưa có dấu cộng nào, nên luật này chặn đúng cái đáp án chép ba con
  # số 45, 54, 137 vào ba chỗ trống. `min: 3` chứ không phải 13 vì `40 + 5`
  # cũng là một cách ráp cột hợp lệ.
  - kind: uses-operator, target: +, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^45\n54\n137\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
45 với 54 — cùng hai chữ số ấy, mà lệch nhau chín hạt. Chỗ đứng nói tất cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàng ô vừa cứu Byte khỏi việc vẽ 13 cái bó ra giấy. Nhưng hôm sau Byte đổ đống
khác ra bàn, buộc xong thì được: **3 bó-của-bó, không bó lẻ nào, và 7 hạt**.

Cột giữa chẳng có gì để viết vào cả. Bỏ trống nó rồi viết `37` à?

Nhưng `37` thì ai đọc cũng đếm từ phải sang: 7 ở cột hạt, 3 ở cột bó — ra ba
mươi bảy hạt, trong khi đống thật của Byte có tới ba trăm lẻ bảy hạt. Bỏ trống
một ô không làm mất một ô; nó làm **mọi chữ số bên trái tụt xuống một cột**.

Vậy phải viết cái gì vào một cột rỗng? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
