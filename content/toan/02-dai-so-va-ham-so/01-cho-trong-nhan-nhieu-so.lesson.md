---
id: toan.dai-so-va-ham-so.cho-trong-nhan-nhieu-so
title: Ô trống trên bảng giá
summary: Một ô trống trong câu tính không giấu sẵn con số nào — nó nhận được nhiều số khác nhau, và mỗi lần điền là một lần đúng.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.placeholder-many-values]
requires: [math.multiplication, math.order-of-operations, core.variable, core.assignment, core.print-variable, core.number-literal, core.arithmetic, core.output]
concepts: [math.xe-banh-mi, math.o-trong]
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
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Mình dán đúng một tờ giấy, mà mỗi khách trả một số tiền khác. Tờ giấy ấy viết gì?
::::

::::explain{#mot-to-giay-cho-moi-khach}
Cái vườn đã xong mùa của nó. Từ bài này trở đi, thứ chạy suốt mạch là **xe bánh
mì của Byte** — dựng ở đầu ngõ, mỗi sáng bán tới trưa.

Bài cuối mạch trước dừng lại ở một chỗ hở. Byte viết được cả một dòng tính cho
cái vườn, nhưng mọi con số trong đó đều là số **cụ thể**: sáu luống, mười lăm
hạt. Muốn dòng ấy dùng lại được cho mùa sau, Byte đã thử viết chữ `n` và chữ `h`
vào chỗ hai con số ấy — và để lại đúng một câu hỏi: `n` với `h` rốt cuộc **là
cái gì**?

Bài này chưa đụng tới chữ cái vội. Chữ chỉ là cái nhãn, mà muốn hiểu một cái
nhãn thì phải nhìn kỹ **cái nó dán lên** trước đã. Nên ta bắt đầu ở chỗ dễ nhìn
nhất: một tờ giấy dán trên xe.

Giá đã chốt: **một ổ bánh mì 15 000 đồng**. Khách nào tới cũng hỏi đúng một câu:
*"mua mấy ổ thì bao nhiêu tiền?"*

Byte không thể viết riêng một tờ giấy cho mỗi khách — hôm nào cũng một dãy
khách khác nhau, mỗi người mua một kiểu. Nên Byte viết **một** tờ, và chừa lại một **chỗ
trống**:

```text
15 000 × ▢   đồng
```

Cái ô ▢ ấy không phải chỗ Byte giấu một con số. Byte chẳng giấu gì cả — Byte
thật sự không biết khách sắp tới mua mấy ổ, và cũng không cần biết. Ô trống là
chỗ **để dành**: mỗi khách điền số của mình vào đó.
::::

::::example{#ba-khach-ba-lan-dien}
Sáng nay có ba khách. Tờ giấy vẫn là tờ giấy ấy, dán nguyên chỗ cũ.

```text
   tờ giấy dán trên xe          15 000 × ▢

   khách thứ nhất điền 2        15 000 × 2   →     30 000 đồng
   khách thứ hai   điền 5       15 000 × 5   →     75 000 đồng
   khách thứ ba    điền 9       15 000 × 9   →    135 000 đồng
```

Ba con số khác nhau, và **cả ba đều đúng**. Không có con số nào trong đó "đúng
hơn" con số kia. Khách thứ nhất mua 2 ổ thì 2 là số của khách ấy; khách thứ ba
mua 9 ổ thì 9 là số của khách thứ ba. Cùng một tờ giấy, đúng ba lần.

Đây là chỗ nhiều người trượt ngay từ bước đầu, nên nói thẳng ra:

> Nhiều người vừa nhìn thấy một ô trống là nghĩ *"ở đó có một con số đúng đang
> bị giấu, việc của mình là tìm cho ra"*. Với tờ giấy dán trên xe thì suy nghĩ
> ấy không dùng được — **không có con số nào bị giấu**.

Suy nghĩ ấy không phải từ trên trời rơi xuống. Nó đến từ những câu đố: *"Byte
có mấy ổ trong giỏ? Gợi ý: đếm gấp đôi lên thì được mười hai."* Câu đố kiểu đó
**thật sự** có đúng một đáp số nấp sẵn, và tìm ra nó là xong việc. Quy tắc ấy
đúng — nhưng đúng trong phạm vi của câu đố.

Tờ giấy dán trên xe không phải câu đố. Nó là một cái **khuôn**, dựng sẵn để dùng
lại. Ô trống trong khuôn nhận được nhiều số, và mỗi số cho một kết quả riêng.
::::

::::explain{#dien-lai-so-cu}
Còn một chuyện nữa về cái ô trống, và nó dễ trượt hơn chuyện vừa rồi.

Ô trống **không nhớ** lần điền trước. Khách thứ nhất điền 2 xong rồi đi; tờ giấy
không giữ lại con số 2 nào cả, cũng không cộng dồn 2 với 5 của khách sau. Mỗi
lần điền là một lần **riêng**, bắt đầu lại từ đúng cái khuôn ban đầu.

Nên nếu khách thứ tư cũng mua 2 ổ, tờ giấy lại cho ra 30 000 — y hệt lần đầu.
Không phải vì tờ giấy nhớ, mà vì cùng một khuôn với cùng một số điền vào thì
phải cho cùng một kết quả.

Câu ấy đọc thì xuôi, nhưng nó cãi lại hai thói quen rất mạnh mà bạn đã có: một
cái tên trong Python thì **nhớ** giá trị mới nhất, và một cuốn sổ bán hàng thì
**cộng dồn** qua từng khách. Ô trống không làm cả hai việc đó. Trước khi đi tiếp,
thử xem thói quen nào đang lên tiếng trong đầu bạn.
::::

::::predict{#doan-ba-lan-dien commitOnce}
Byte thử một loạt khác, vẫn trên đúng cái khuôn `15 000 × ▢`, và cố ý xếp con
số 5 vào giữa hai con số 3.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(15000 * 3)
print(15000 * 5)
print(15000 * 3)
```

:::opt{correct}
45000, rồi 75000, rồi 45000
:::

:::opt
45000, rồi 75000, rồi 120000
::why
Gần đúng ở chỗ bạn đọc hai dòng đầu không sai một đồng, và bạn đang dùng một quy
tắc rất đúng của cái xe: *bán thêm cho khách nữa thì tiền cả buổi tăng lên*. Cuối
buổi Byte đúng là muốn biết một con số cộng dồn như thế.

Nhưng 120 000 dừng lại ở khách thứ hai. Ba dòng này là 3 ổ, rồi 5 ổ, rồi 3 ổ nữa
— cả buổi 11 ổ, tức 11 × 15 000 = 165 000 đồng. Nên 120 000 không phải tổng cả
buổi; nó là tổng nửa chừng.

Chỗ lệch nằm ở ranh giới giữa **một lần điền** và **cả buổi bán**. Muốn cộng dồn
thì phải có một chỗ giữ tổng — một cái tên nhận thêm sau mỗi khách, đúng như biến
cộng dồn bạn từng viết. Ở đây không có chỗ nào như thế: ba dòng là ba lần dùng
lại cùng một tờ giấy, mỗi lần bắt đầu lại từ khuôn trống. Tờ giấy không giữ gì
của khách trước.
::
:::

:::opt
45000, rồi 75000, rồi 75000
::why
Gần đúng ở chỗ bạn đang dùng một quy tắc thật của Python, và dùng đúng: một cái
**tên** giữ giá trị mới nhất, viết đè lên thì giá trị cũ mất. `so_o = 3` rồi
`so_o = 5` thì từ đó trở đi `so_o` là 5, không cách nào quay về 3.

Chỗ lệch là ô trống **không phải** một cái tên. Nó không giữ gì cả, nên cũng
chẳng có gì để bị đè lên. Dòng thứ ba không đọc lại con số của dòng thứ hai — nó
cầm cái khuôn trống rồi điền 3 vào, y như dòng thứ nhất đã làm. Cái tên thì nhớ;
ô trống thì không.
::
:::

:::opt
45000 cả ba dòng, vì trong một bài toán thì ô trống chỉ đứng cho đúng một số
::why
Gần đúng, và cái linh cảm này về sau sẽ thành một luật thật — bài ngay sau đây
nói đúng về nó. Trong **một** câu tính, hai chỗ mang cùng một dấu thì buộc phải
điền cùng một số; nếu không thì câu tính kể hai chuyện khác nhau cùng lúc.

Chỗ lệch là phạm vi của chữ "một". Ba dòng trên không phải một câu tính, chúng
là **ba** câu tính riêng, ba lần dùng lại cùng một cái khuôn. Luật "cùng một chỗ
thì cùng một số" nói về bên trong một câu tính, không nói về hai câu tính khác
nhau. Dòng thứ hai điền 5 là một lần điền hoàn toàn hợp lệ, và nó không ràng
buộc gì dòng thứ nhất.
::
:::
::::

::::explain{#dat-ten-cho-thu-vua-thay}
Gói lại thành một câu mang đi được:

> Một **ô trống** trong câu tính nhận được **nhiều** giá trị khác nhau. Mỗi lần
> điền một số vào, câu tính cho ra một kết quả — và mọi lần điền đều hợp lệ.

Hai hệ quả rơi ra ngay, cả hai đều dùng tới suốt mạch:

- **Ô trống không phải một con số bị giấu.** Không có gì để "tìm ra". Hỏi *"ô
  trống bằng bao nhiêu?"* lúc này là một câu hỏi chưa có nghĩa — như hỏi một cái
  khuôn bánh xem nó nặng mấy gam bột.
- **Ô trống làm một tờ giấy dùng được cho mọi khách.** Đây là cả lý do nó tồn
  tại: viết một lần, dùng ba mươi lần.

Và để ý một chuyện nhỏ mà quan trọng: điền số **0** cũng là một lần điền thật.
Khách ghé xem rồi đi, không mua ổ nào — `15 000 × 0` cho ra 0 đồng. Đó không
phải "ô trống rỗng", đó là ô trống đã được điền, điền bằng số 0.
::::

::::code{#ba-khach-sang-nay}
Sáng nay Byte ghi sổ. Vẫn đúng cái khuôn dán trên xe — **15 000 × ▢** — và ba
khách điền ba số khác nhau.

- Khách thứ nhất mua `2` ổ.
- Khách thứ hai mua `5` ổ.
- Khách thứ ba mua `9` ổ.

Điền ba chỗ trống để máy nói ra mỗi khách trả bao nhiêu.

Cả ba chỗ đều phải viết ra thành một **phép nhân thật**, dựng từ cái tên
`gia_mot_o`. Gõ thẳng con số tiền vào thì bạn đã tính hộ máy rồi, và ba lần điền
kia không còn gì để cho thấy nữa.

```python title=starter
# Giá đã chốt: một ổ 15 000 đồng. Con số này KHÔNG phải ô trống — nó biết rồi.
gia_mot_o = 15000

# Ô trống là SỐ Ổ. Ba khách, ba lần điền vào cùng một khuôn.
tien_khach_1 = ___   # khách thứ nhất mua 2 ổ
tien_khach_2 = ___   # khách thứ hai mua 5 ổ
tien_khach_3 = ___   # khách thứ ba mua 9 ổ

print(tien_khach_1)
print(tien_khach_2)
print(tien_khach_3)
```

```python title=solution
# Giá đã chốt: một ổ 15 000 đồng. Con số này KHÔNG phải ô trống — nó biết rồi.
gia_mot_o = 15000

# Ô trống là SỐ Ổ. Ba khách, ba lần điền vào cùng một khuôn.
tien_khach_1 = gia_mot_o * 2   # khách thứ nhất mua 2 ổ
tien_khach_2 = gia_mot_o * 5   # khách thứ hai mua 5 ổ
tien_khach_3 = gia_mot_o * 9   # khách thứ ba mua 9 ổ

print(tien_khach_1)
print(tien_khach_2)
print(tien_khach_3)
```

```python title=test
# Ba câu `!=` chốt đúng cái hiểu lầm mà bài này sinh ra để sửa, và chúng đứng
# TRƯỚC các câu `==` vì chương trình dừng ở câu vỡ đầu tiên — xếp sau một câu
# `==` bao trùm thì chúng không bao giờ chạy tới, và cái bẫy không bao giờ sập.
assert tien_khach_1 != tien_khach_2, "cùng một khuôn nhưng hai lần điền khác nhau thì phải ra hai số khác nhau — ô trống không giữ sẵn một con số cố định"
assert tien_khach_2 != tien_khach_3, "khách thứ hai mua 5 ổ, khách thứ ba mua 9 ổ — hai lần điền khác nhau, hai số tiền khác nhau"
assert tien_khach_3 != tien_khach_1 + tien_khach_2, "ba dòng là ba lần điền riêng, không phải một cái tổng cộng dồn qua từng khách"
# Ba lần điền, ba con số — nên một con số gõ cứng chỉ qua được nhiều nhất một câu.
assert tien_khach_1 == 30000, "điền 2 vào ô trống: 15 000 × 2"
assert tien_khach_2 == 75000, "điền 5 vào ô trống: 15 000 × 5"
assert tien_khach_3 == 135000, "điền 9 vào ô trống: 15 000 × 9"
# Ráp ngược lại: từ 2 ổ lên 5 ổ là thêm đúng ba ổ, nên tiền chênh phải đúng ba
# lần giá một ổ. Câu này bắt hai chỗ trống ăn khớp NHAU — nó vẫn đạt với đáp án
# chép cứng 30000 và 75000 (45000 == 45000), nên chỗ chặn chép cứng nằm ở
# `forbidAst` bên dưới, không nằm ở đây.
assert tien_khach_2 - tien_khach_1 == gia_mot_o * 3, "từ 2 ổ lên 5 ổ là thêm ba ổ, nên tiền thêm đúng ba lần giá một ổ"
```

:::hints
- kind: attention
  body: Nhìn dòng trên cùng — cái tên `gia_mot_o` đang giữ giá một ổ, và nó là thứ duy nhất bạn đã biết chắc. Rồi nhìn chú thích ở cuối mỗi chỗ trống: nó cho biết khách ấy điền số mấy vào ô trống.
- kind: strategy
  body: Tờ giấy dán trên xe viết là "giá một ổ nhân với số ổ". Mỗi chỗ trống chép lại đúng câu ấy, chỉ khác con số điền vào ô. Đừng viết thẳng số tiền — con số ấy chính là thứ bạn đang nhờ máy tính hộ, và viết nó ra là bỏ mất cả ba lần điền.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `gia_mot_o * 2`, `gia_mot_o * 5`, `gia_mot_o * 9`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một phép nhân thật dựng từ `gia_mot_o` — gõ thẳng 30000, 75000 hay 135000 vào thì máy không điền gì cả, bạn điền hộ nó rồi
  requireAst:
  # Khung khởi đầu không có dấu `*` nào và không đọc cái tên nào, nên hai luật
  # này chặn được đáp án chép cứng ba con số.
  - kind: uses-operator, target: *, min: 3
  - kind: uses-name, target: gia_mot_o, min: 3
  forbidAst:
  # Lưới thứ hai. `uses-operator` đếm trên CẢ FILE, nên đáp án gõ cứng ĐÚNG MỘT
  # chỗ — ví dụ `tien_khach_1 = 30000` rồi hai chỗ kia làm thật — vẫn có ba dấu
  # `*` và vẫn lọt. Ba luật dưới đây chặn từng con số kết quả một. Chúng không
  # cản cách viết hợp lệ nào: `gia_mot_o * 2` không chứa nguyên văn 30000.
  - kind: has-literal, target: 30000
  - kind: has-literal, target: 75000
  - kind: has-literal, target: 135000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^30000\n75000\n135000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba khách, ba con số, một tờ giấy. Mình khỏi phải viết lại lần nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte bán chạy nên dán thêm chai nước lên xe: **8 000 đồng một chai**. Giờ một
khách có thể mua vài ổ bánh mì **và** vài chai nước cùng lúc, nên tờ giấy phải
có **hai** ô trống:

```text
15 000 × ▢   +   8 000 × ▢
```

Nhìn kỹ dòng ấy đi. Có một khách mua 3 ổ bánh mì và 1 chai nước — hai ô, hai số
khác nhau. Vậy hai ô trống này có **buộc** phải điền cùng một số không?

Và nếu không buộc, thì rắc rối tới ngay: hai cái ô trông giống hệt nhau. Byte
đưa tờ giấy cho người khác đọc, người ta biết ô nào là ô của bánh mì, ô nào là ô
của nước bằng cách gì? Chỉ dựa vào chỗ nó đứng trên dòng thôi sao — thế lúc dòng
dài ra, năm bảy cái ô, thì còn nhìn ra nữa không?

Bài sau dán nhãn cho chúng.
::::

::::checkpoint{mastery=0.8}
::::
