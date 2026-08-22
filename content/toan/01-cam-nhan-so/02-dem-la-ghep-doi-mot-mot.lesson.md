---
id: toan.cam-nhan-so.dem-la-ghep-doi-mot-mot
title: Đếm là ghép đôi
summary: Đếm là dán lên mỗi vật đúng một cái tên trong dãy một, hai, ba… Vì là ghép một–một nên đi đường nào đếm cũng ra cùng một con số.
locale: vi
track: toan
module: cam-nhan-so
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.dem-mot-mot]
requires: [math.don-vi, core.number-literal, core.variable, core.print-variable, ctrl.comparison]
concepts: [math.dem, math.ghep-doi, math.vuon-cua-byte]
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
Bạn hỏi vì sao hai người đếm hai kiểu mà ra cùng một số. Có một cái luật giữ.
::::

::::explain{#ghep-doi-truoc-khi-co-con-so}
Trước khi nói tới đếm, nhớ lại một việc bạn làm ở nhà: **dọn mâm cơm**.

Bạn không đếm người, cũng không đếm đũa. Bạn đi một vòng quanh mâm, mỗi chỗ ngồi
đặt xuống một đôi đũa. Đặt hết vòng mà tay còn thừa một đôi — bạn biết ngay đũa
nhiều hơn người. Đặt tới chỗ cuối mà hết đũa — thiếu.

Bạn vừa so hai đống với nhau mà chưa đọc lên một con số nào.

Cái việc bạn vừa làm có tên: **ghép đôi một–một**. Nó có đúng hai luật:

1. Mỗi chỗ ngồi nhận **đúng một** đôi đũa — không chỗ nào bị bỏ qua.
2. Mỗi đôi đũa chỉ được đặt xuống **một lần** — không đôi nào được đặt hai chỗ.

Giữ đủ hai luật ấy thì kết quả nói thật: hết cả hai bên cùng lúc nghĩa là hai
đống bằng nhau. Phá một luật thôi thì kết quả nói dối ngay — bỏ sót một chỗ
ngồi, bạn sẽ tưởng đũa thừa.
::::

::::explain{#dem-cung-la-ghep-doi}
Đếm cũng là ghép đôi. Chỉ khác một chỗ: bên kia không phải đũa, mà là một **dãy
tên có sẵn** ai cũng thuộc lòng và ai cũng đọc theo đúng thứ tự đó — *một, hai,
ba, bốn, năm…*

Byte đổ đống hạt ra chiếu và đếm. Mỗi lần Byte chỉ tay vào một hạt, Byte đọc lên
cái tên tiếp theo trong dãy. Chỉ tay = ghép hạt ấy với tên ấy.

Ba luật của việc đếm, viết ra cho rõ:

1. Mỗi hạt nhận **đúng một** tên: không hạt nào bị bỏ sót, không hạt nào bị chỉ
   vào hai lần.
2. Tên phải đọc theo **đúng thứ tự** dãy, bắt đầu từ *một*, không nhảy cóc.
   Chú ý: cái phải giữ nguyên là thứ tự của DÃY TÊN, không phải thứ tự bạn đi
   qua các hạt. Đúng chỗ khác nhau đó là toàn bộ bài này.
3. Số hạt của cả đống là **cái tên cuối cùng** bạn đọc tới.

Luật thứ ba mới là chỗ đáng dừng lại. Cái tên "mười hai" vốn được sinh ra để dán
lên **một** hạt riêng lẻ — hạt cuối cùng. Thế mà ta lại đem nó ra làm con số của
**cả đống**. Vì sao được phép?

Được phép, vì hai luật trên. Đọc hết dãy tên từ *một* tới *mười hai* nghĩa là đã
dán xuống đúng mười hai cái tên khác nhau; mỗi tên đi với một hạt khác nhau, và
không hạt nào còn trống. Nên "mười hai" vừa là tên của hạt cuối, vừa là **số tên
đã dùng** — mà số tên đã dùng thì đúng bằng số hạt.
::::

::::example{#hai-nguoi-dan-hai-kieu-nhan}
Giờ trả lời thẳng câu hỏi bài trước. Byte đếm từ phía hàng rào vào, An đếm từ
phía cổng lại. Xếp cả đống thành một hàng cho dễ nhìn:

```text
đống hạt xếp thành hàng:     o   o   o   o   o   o   o   o   o   o   o   o
Byte dán (từ hàng rào vào):  1   2   3   4   5   6   7   8   9  10  11  12
An dán (từ cổng lại):       12  11  10   9   8   7   6   5   4   3   2   1
```

Nhìn cột đầu tiên: cùng **một** hạt, Byte gọi là *một*, An gọi là *mười hai*.
Hai cái nhãn khác hẳn nhau, và không ai dán sai cả.

Nhưng nhìn cả hàng: cả hai người đều dán hết, mỗi hạt đúng một nhãn, không sót
không lặp. Nên cả hai đều xài hết đúng chừng ấy cái tên trong dãy — và cái tên
cuối cùng của cả hai đều là cái thứ **mười hai**.

Đây là câu trả lời: **đổi đường đi khi đếm — thăm vật nào trước, vật nào sau —
chỉ làm đổi cái nhãn trên từng hạt, không làm đổi số nhãn đã dùng.** Con số 12
không phải may mắn; nó bị luật ghép đôi ép phải ra như vậy.

Đưa hai cái nhãn của cùng cái hạt cạnh hàng rào cho máy giữ:

```python title=readonly
byte_goi_hat_canh_hang_rao_la = 1
an_goi_hat_canh_hang_rao_la = 12

print(byte_goi_hat_canh_hang_rao_la)
print(an_goi_hat_canh_hang_rao_la)
```

```text
1
12
```
::::

::::predict{#doan-may-tra-loi commitOnce}
Cùng một hạt, hai cái nhãn. Giờ hỏi máy xem hai cái nhãn ấy có bằng nhau không.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
byte_goi_hat_canh_hang_rao_la = 1
an_goi_hat_canh_hang_rao_la = 12
print(byte_goi_hat_canh_hang_rao_la == an_goi_hat_canh_hang_rao_la)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn bám vào một sự thật quan trọng: hai cái tên ấy đang chỉ vào
**đúng một** hạt ngoài vườn, mà hạt thì chỉ có một. Suy nghĩ ấy là đúng, và nó
chính là điều làm chuyện này thú vị.

Chỗ lệch: máy không nhìn thấy cái hạt. Bài 1 đã chỉ ra chuyện đó — thứ đi vào
máy là con số trần, đã rụng mất đơn vị và rụng luôn cả chuyện nó đang chỉ vào
vật nào. Hai bên dấu `==` chỉ còn là `1` và `12`. Hai con số ấy khác nhau thật,
nên `False` là câu trả lời đúng cho đúng câu đã hỏi.
::
:::

:::opt
12
::why
Gần đúng ở chỗ bạn đang đi tìm **một** câu trả lời đúng duy nhất cho câu hỏi
"hạt ấy là hạt số mấy" — và với rất nhiều câu hỏi thì đúng là chỉ có một câu trả
lời.

Chỗ lệch nằm ở chính giả định đó. "Hạt số mấy" không phải một tính chất của cái
hạt, như màu vỏ hay độ nặng. Nó là cái nhãn do **người đếm** dán lên, và đổi
người đếm thì nhãn đổi theo. Không có con số nào là "số thật" của hạt ấy cả.

Còn dòng lệnh này thì không hỏi hạt số mấy: nó hỏi hai con số có bằng nhau
không, và thứ đi ra khỏi `==` luôn là `True` hoặc `False`.
::
:::
::::

::::explain{#cai-gi-doi-cai-gi-khong}
Gom lại thành một câu để mang theo cả track:

> Trong một lần đếm, **cái nhãn trên từng vật thì tuỳ người đếm; còn cái tên
> cuối cùng thì không.**

Và điều đó cho bạn một cách kiểm lại chính mình. Lần sau đếm ra một con số bạn
nghi ngờ, đừng đếm lại y hệt đường cũ — hãy đếm theo một đường **khác**. Ra cùng
số thì bạn tin được hơn hẳn — hai lần đi hai đường khác nhau mà cùng phá luật y
hệt một kiểu là chuyện hiếm. Ra khác số thì chắc chắn có một lần đã phá luật: bỏ
sót một vật, chỉ vào một vật hai lần, hoặc đọc nhảy mất một cái tên trong dãy.

Đây cũng là chỗ trả lời câu "sao không thể ra 11 hay 13" của bài trước. Ra 13
nghĩa là hoặc có một hạt bị chỉ hai lần, hoặc bạn đã đọc nhảy một cái tên. Ra 11
nghĩa là hoặc bỏ quên một hạt, hoặc đọc lặp một cái tên. Cả bốn đều là phá luật,
chứ không phải xui.
::::

::::code{#hai-cach-dem-mot-ket-qua}
Byte đem chính cái phép kiểm vừa học ra dùng, trên đúng đống hạt của mình. Đống
hạt ấy được đếm ba lần, ba đường đi khác nhau:

- Byte đếm từ phía hàng rào vào, đọc tới tên cuối là `12`.
- An đếm từ phía cổng lại, cũng tới `12`.
- An đếm thêm lần nữa, đi một đường khác nữa — lần này tới `13`.

Hai chỗ trống là hai câu hỏi có–không. Mỗi câu đặt lần đếm của Byte cạnh một lần
đếm của An, để xem hai lần ấy có khớp nhau không.

Hai câu này được chọn để cho ra hai câu trả lời **ngược nhau**: một cặp lệch, một
cặp khớp. Gõ cứng `False` vào cả hai thì câu sau sai; gõ cứng `True` thì câu
trước sai. Chỉ hai câu hỏi viết thật mới qua được cả hai.

```python title=starter
# Cùng MỘT đống hạt, đếm ba lần, mỗi lần đi một đường khác.
byte_dem_ca_dong_duoc = 12
an_dem_ca_dong_duoc = 12
an_dem_lai_lan_ba_duoc = 13   # An đếm lại lần nữa, đi một đường khác nữa

# 1) Lần đếm thứ ba có khớp với hai lần trước không?
print(___)

# 2) Còn hai lần đếm đầu — có khớp nhau không?
print(___)
```

```python title=solution
# Cùng MỘT đống hạt, đếm ba lần, mỗi lần đi một đường khác.
byte_dem_ca_dong_duoc = 12
an_dem_ca_dong_duoc = 12
an_dem_lai_lan_ba_duoc = 13   # An đếm lại lần nữa, đi một đường khác nữa

# 1) Lần đếm thứ ba có khớp với hai lần trước không?
print(byte_dem_ca_dong_duoc == an_dem_lai_lan_ba_duoc)

# 2) Còn hai lần đếm đầu — có khớp nhau không?
print(byte_dem_ca_dong_duoc == an_dem_ca_dong_duoc)
```

```python title=test
# Ba assert này chốt lại chính điều bài vừa dạy, trên hai tình huống ngược
# nhau. Nếu một ngày nào đó có người sửa mấy con số ở trên cho "gọn" thì cổng
# đỏ lên, chứ bài không lặng lẽ dạy sai.
assert byte_dem_ca_dong_duoc == an_dem_ca_dong_duoc, "hai lần đếm cùng giữ đủ luật, trên cùng một đống, thì PHẢI ra cùng một số — đó là nửa đầu của bài"
assert byte_dem_ca_dong_duoc == 12, "cái tên cuối cùng của dãy một–mười hai là 12, không phải con số nào khác"
assert byte_dem_ca_dong_duoc != an_dem_lai_lan_ba_duoc, "ra 13 thì chắc chắn một lần nào đó đã phá luật — chỗ này bài tập bắt bạn nhìn ra dấu hiệu đó"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều nằm trong `print`, và bên trên mỗi chỗ đã có sẵn một dòng chú thích nói rõ nó đang hỏi cái gì. Việc của chúng không phải in lại một con số, mà là hỏi máy xem hai lần đếm có ra cùng một số không.
- kind: strategy
  body: Mỗi câu hỏi gồm ba phần — tên bên trái, dấu so sánh bằng của Realm 0 (hai dấu bằng viết liền nhau), và tên bên phải. Cả hai câu đều lấy lần đếm của Byte làm mốc: câu thứ nhất đặt nó cạnh lần đếm thứ ba, câu thứ hai đặt nó cạnh lần đếm đầu của An.
- kind: one-line
  body: 'Thay `___` thứ nhất bằng `byte_dem_ca_dong_duoc == an_dem_lai_lan_ba_duoc`, và `___` thứ hai bằng `byte_dem_ca_dong_duoc == an_dem_ca_dong_duoc`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh bằng (`==`) giữa hai lần đếm — hai lần ra khác số nghĩa là đã có một lần phá luật ghép đôi, và chỉ câu hỏi viết thật mới chỉ ra chỗ lệch đó; gõ thẳng `True` hay `False` thì không hỏi máy điều gì cả
  requireAst:
  # `min: 2` vì có hai câu hỏi. Khung khởi đầu chưa có dấu `==` nào, nên luật
  # này chặn được đúng cái đáp án gõ cứng hai chữ True/False.
  - kind: uses-operator, target: ==, min: 2
- tier: output
  match: regex
  expect: ^False\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai lần khớp, một lần lệch. Chỗ lệch ấy là dấu hiệu có lần mình đã phá luật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đếm hạt mà giữ đủ luật thì ai đếm cũng ra 12 — bạn vừa thấy vì sao.

Chiều nay Byte và An ra đo một luống đất.

Byte căng sải dây dọc luống, đặt xuống, đánh dấu, nhấc lên đặt tiếp. Đủ **bốn**
lần thì hết luống. Byte ghi: *dài 4*.

An không có dây. An đi bộ dọc luống, gót chân này chạm mũi chân kia. **Mười
hai** bước thì hết luống. An ghi: *dài 12*.

Cả hai đều làm cẩn thận: mỗi lần đặt xuống là một cái tên trong dãy, không sót
lần nào, không đếm lặp lần nào — đúng y cái luật bài này vừa dựng. Luống đất thì
nằm im, không dài ra cũng không ngắn đi.

Vậy mà một người ra 4, một người ra 12, và không ai đo sai cả.

Luật ghép đôi vẫn nguyên. Có một thứ khác đã đổi. Thứ gì? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
