---
id: toan.cam-nhan-so.do-la-dem-cai-thuoc
title: Đo là đếm cái thước
summary: Số đo là số lần cái thước đã chọn đặt lặp lại được. Đổi thước thì con số đổi, còn lượng đất thì không đổi tí nào.
locale: vi
track: toan
module: cam-nhan-so
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.thuoc-do]
requires: [math.dem-mot-mot, math.don-vi, core.number-literal, core.variable, core.print-variable, ctrl.comparison]
concepts: [math.do, math.thuoc-do, math.vuon-cua-byte]
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
Bốn và mười hai. Cùng một luống đất. Mình không đo sai, mà An cũng không.
::::

::::explain{#nhin-ky-hai-viec-do}
Nhìn chậm lại xem Byte và An đã làm gì.

**Byte**: căng sải dây dọc luống. Đặt xuống lần thứ nhất, lấy que đánh dấu chỗ
đầu dây kết thúc. Nhấc dây lên, đặt tiếp từ chỗ đánh dấu. Lần thứ hai. Lần thứ
ba. Lần thứ tư thì vừa hết luống. Byte ghi *4*.

**An**: không có dây, nên dùng bàn chân. Đặt gót chân này chạm mũi chân kia, đi
dọc luống. Một, hai, ba… tới bước thứ mười hai thì vừa hết luống. An ghi *12*.

Giờ để ý cái việc **chung** của hai người: cả hai đều **đếm**. Byte đếm số lần
đặt sải dây; An đếm số lần đặt bàn chân. Mỗi lần đặt xuống, một cái tên trong
dãy được đọc lên — một, hai, ba… Không lần nào bị bỏ sót, không chỗ nào bị đặt
đè lên chỗ cũ.

Đó đúng là luật ghép đôi một–một của bài trước, đem áp vào một đống khác: thay
vì ghép tên với **hạt**, giờ ghép tên với **những lần đặt thước xuống**.

Nên đây là điều bài này muốn nói, gọn một câu:

> **Đo là đếm — đếm xem cái thước bạn chọn đặt lặp lại được mấy lần.**

Số đo không phải một loại số bí ẩn khác với số đếm. Nó vẫn là số đếm, chỉ khác ở
chỗ *thứ được đếm* là những lần đặt thước.
::::

::::explain{#doi-thuoc-thi-doi-con-so}
Vậy vì sao hai người ra hai con số?

Vì họ đếm hai thứ khác nhau. Byte đếm **sải dây**, An đếm **bước chân**. Đây
chính là mảnh thứ hai của bài 1 quay lại: cái thước là **đơn vị** của số đo.

- Byte trả lời câu hỏi: *luống này dài mấy SẢI DÂY?* → 4.
- An trả lời câu hỏi: *luống này dài mấy BƯỚC CHÂN?* → 12.

Hai câu hỏi khác nhau, hai câu trả lời khác nhau. Không ai sai cả. Chỗ sai duy
nhất là hai tấm ghi chép — chúng ghi trần *4* và *12*, để rơi mất đơn vị, đúng
cái lỗi tấm bảng của bài 1.

Vẽ cả hai cách đo lên cùng một dải đất thì nhìn ra ngay:

```text
luống đất   ├───────────────────────────────────┤

sải dây     ├────────┼────────┼────────┼────────┤
                 1        2        3        4

bước chân   ├──┼──┼──┼──┼──┼──┼──┼──┼──┼──┼──┼──┤
              1  2  3  4  5  6  7  8  9 10 11 12
```

Ba hàng vạch, một dải đất. Hàng trên cùng là luống đất — nó dài đúng chừng ấy,
không hơn không kém. Hai hàng dưới là hai cách chia cùng dải ấy thành từng khúc
bằng nhau: khúc to là sải dây, khúc nhỏ là bước chân.

Đếm kỹ trong hình: cứ **một** sải dây thì vừa đúng **ba** bước chân. Cái thước
của An nhỏ hơn, nên nó phải đặt xuống nhiều lần hơn mới phủ hết luống.

Và đó là hình dạng chung của mọi phép đo: **thước nhỏ đi thì con số lớn lên, và
ngược lại.** Hai thứ ấy đi ngược chiều nhau, vì chúng cùng phải phủ đúng một
lượng đất.

Để ý một chuyện may ở đây: cả hai lần đo đều **vừa khít** — lần đặt thước cuối
cùng dừng đúng ở cuối luống, không thừa mẩu nào. Đời thật ít khi tử tế thế. Nếu
bàn chân An đặt tới lần thứ mười hai mà luống vẫn còn thừa một mẩu ngắn hơn một
bàn chân, thì con số phải ghi là gì? Giữ câu hỏi ấy lại, vài bài nữa sẽ tới.
::::

::::explain{#con-so-doi-luong-dat-khong-doi}
Chỗ này đáng nói thẳng ra, vì nó là cái trục chạy suốt track:

> Đổi thước thì **con số** đổi. **Lượng đất** thì không đổi tí nào.

Byte không làm luống dài thêm khi đổi sang đo bằng bước chân. Con số 4 nhảy lên
12, nhưng cái luống nằm im đó suốt.

Nghĩa là câu hỏi "*luống này dài bao nhiêu?*" chưa phải một câu hỏi đủ nghĩa.
Câu hỏi đủ nghĩa là "*luống này dài mấy sải dây?*" hoặc "*mấy bước chân?*" — và
chỉ khi đó mới có một câu trả lời.

Cũng vì thế, không có con số nào là "con số thật" của luống đất. `4` đúng chừng
nào thì `12` cũng đúng chừng ấy. Cái làm chúng khác nhau là cái thước, không
phải sự cẩn thận của người đo.
::::

::::example{#may-chep-lai-hai-cau-tra-loi}
Đưa hai con số cho máy giữ:

```python title=readonly
byte_do_bang_sai_day = 4
an_do_bang_buoc_chan = 12

print(byte_do_bang_sai_day)
print(an_do_bang_buoc_chan)
```

```text
4
12
```

Máy in ra hai con số khác nhau cho cùng một luống đất, và nó không thấy có gì
lạ. Nó chỉ chép lại hai câu trả lời cho hai câu hỏi khác nhau — đơn vị thì vẫn
nằm trong tên biến, chỗ mà máy không đọc nghĩa.
::::

::::predict{#doan-may-tra-loi commitOnce}
Giờ hỏi máy: hai con số ấy có bằng nhau không?

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
byte_do_bang_sai_day = 4
an_do_bang_buoc_chan = 12
print(byte_do_bang_sai_day == an_do_bang_buoc_chan)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn giữ chắc điều quan trọng nhất của bài: hai người đang nói về
**đúng một** luống đất, và luống đất thì chỉ dài có chừng ấy. Ý nghĩ ấy đúng, và
nó là nửa sau của bài này.

Chỗ lệch: hai bên dấu `==` không phải hai luống đất, chúng là hai **số đếm** —
4 lần đặt dây và 12 lần đặt chân. Hai con số ấy khác nhau thật, và chúng khác
nhau chính vì hai cái thước khác nhau. Nếu máy trả `True` ở đây thì mới là nó
nói sai.
::
:::

:::opt
Máy dừng lại và báo lỗi, vì hai bên đo bằng hai đơn vị khác nhau
::why
Gần đúng ở chỗ suy nghĩ này đúng ngoài đời: đem 4 sải so thẳng với 12 bước mà
không quy về cùng một thước thì đó là một phép so hỏng, và người cẩn thận sẽ
dừng lại đòi làm rõ trước khi trả lời. Bạn đang áp một luật thật.

Chỗ lệch là phạm vi của luật ấy. Cái đơn vị chỉ nằm trong **tên biến** và trong
đầu bạn — bài 1 đã chỉ ra chuyện đó. Thứ máy cầm trong tay là hai con số trần
`4` và `12`; nó so được nên nó so, và nó không có cách nào biết chúng đo bằng
hai thước khác nhau. Máy chỉ dừng khi nó **không biết** phải làm gì, chứ không
dừng vì một câu hỏi vô nghĩa.
::
:::

:::opt
12
::why
Gần đúng ở chỗ bạn chọn con số đo bằng cái thước **nhỏ hơn** — và ý nghĩ đứng
sau lựa chọn ấy có lý: đo bằng thước nhỏ thì chia được nhỏ hơn, nên nghe như nó
"đúng hơn".

Chỗ lệch có hai tầng. Tầng ngoài: dòng này không hỏi luống dài bao nhiêu, nó hỏi
hai con số có bằng nhau không — và thứ đi ra khỏi `==` luôn là `True` hoặc
`False`. Tầng trong, quan trọng hơn: `12` không "đúng hơn" `4`. Cả hai đều là số
lần đặt thước, và chúng cùng tả đúng một luống đất. Cái thước nhỏ chỉ chia nhỏ
hơn, chứ không đo thật hơn.
::
:::
::::

::::code{#giu-thuoc-hay-doi-thuoc}
An đo lần thứ ba. Lần này An mượn đúng sải dây của Byte và đo lại luống ấy từ
đầu — An cũng được **4**.

Ba con số đã có sẵn. Hai chỗ trống là hai câu hỏi có–không:

1. **Giữ nguyên thước, đổi người đo** — hai con số có giống nhau không?
2. **Đổi sang cái thước nhỏ hơn** — con số có nhỏ đi theo không?

Câu thứ hai là chỗ dễ trượt tay nhất của bài. Đừng đoán trước đáp án: cứ đọc
thẳng câu hỏi trong chú thích, viết ra phép so sánh, rồi để máy trả lời.

Hai câu này được chọn để cho ra hai câu trả lời **ngược nhau**. Gõ cứng `True`
vào cả hai thì câu sau sai; gõ cứng `False` thì câu trước sai. Chỉ hai câu hỏi
viết thật mới qua được cả hai.

```python title=starter
# Cùng MỘT luống đất, ba lần đo.
byte_do_bang_sai_day = 4        # Byte đặt sải dây, đếm được 4 lần
an_do_lai_bang_sai_day = 4      # An mượn đúng sải dây ấy, đo lại từ đầu
an_do_bang_buoc_chan = 12       # An đặt bàn chân, đếm được 12 lần

# 1) Cùng một cái thước, hai người đo — hai con số có giống nhau không?
print(___)

# 2) Đổi sang cái thước NHỎ hơn — con số có nhỏ đi theo không?
#    Hỏi máy: số đo bằng sải dây có lớn hơn số đo bằng bước chân không?
print(___)
```

```python title=solution
# Cùng MỘT luống đất, ba lần đo.
byte_do_bang_sai_day = 4        # Byte đặt sải dây, đếm được 4 lần
an_do_lai_bang_sai_day = 4      # An mượn đúng sải dây ấy, đo lại từ đầu
an_do_bang_buoc_chan = 12       # An đặt bàn chân, đếm được 12 lần

# 1) Cùng một cái thước, hai người đo — hai con số có giống nhau không?
print(byte_do_bang_sai_day == an_do_lai_bang_sai_day)

# 2) Đổi sang cái thước NHỎ hơn — con số có nhỏ đi theo không?
#    Hỏi máy: số đo bằng sải dây có lớn hơn số đo bằng bước chân không?
print(byte_do_bang_sai_day > an_do_bang_buoc_chan)
```

```python title=test
# Hai tình huống ngược nhau, chốt bằng hai assert ngược nhau: giữ thước thì con
# số phải trùng, đổi thước thì con số phải lệch. Một cái assert thôi thì không
# nói được gì — chính cặp đôi này mới là nội dung bài.
assert byte_do_bang_sai_day == an_do_lai_bang_sai_day, "cùng một cái thước, cùng một luống thì hai người PHẢI ra cùng một số — đó là luật ghép đôi của bài 2 vẫn còn nguyên"
assert byte_do_bang_sai_day != an_do_bang_buoc_chan, "đổi thước thì con số PHẢI đổi — nếu không thì cả bài này không có gì để nói"
assert an_do_bang_buoc_chan > byte_do_bang_sai_day, "bước chân nhỏ hơn sải dây, nên phải đặt xuống nhiều lần hơn: thước nhỏ đi thì con số lớn lên"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều nằm trong `print`, và dòng chú thích ngay trên mỗi chỗ nói rõ nó đang hỏi cái gì. Ba cái tên ở đầu bài đã giữ sẵn ba con số; việc của bạn là chọn đúng cặp tên cho mỗi câu hỏi, và đúng dấu so sánh cho mỗi câu — hai câu này không hỏi cùng một kiểu.
- kind: strategy
  body: Câu thứ nhất nói về hai lần đo cùng bằng sải dây, nên cả hai tên trong đó đều có chữ `sai_day`, nối nhau bằng dấu so sánh bằng của Realm 0 — hai dấu bằng viết liền nhau. Câu thứ hai không hỏi bằng nhau nữa mà hỏi bên nào lớn hơn, nên dấu ở giữa là dấu lớn hơn `>`: bên trái là số đo bằng sải dây, bên phải là số đo bằng bước chân.
- kind: one-line
  body: 'Thay `___` thứ nhất bằng `byte_do_bang_sai_day == an_do_lai_bang_sai_day`, và `___` thứ hai bằng `byte_do_bang_sai_day > an_do_bang_buoc_chan`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống hỏi hai kiểu khác nhau — một chỗ hỏi hai con số có bằng nhau không (`==`), chỗ kia hỏi con số nào lớn hơn (`>`); gõ thẳng `True` hay `False` thì không hỏi máy điều gì cả
  requireAst:
  # Hai luật cho hai chỗ trống, mỗi chỗ một dấu so sánh khác nhau. Khung khởi
  # đầu chưa có dấu nào trong hai dấu ấy, nên cặp luật này chặn được đúng cái
  # đáp án gõ cứng hai chữ True/False, và chặn luôn kiểu viết `==` cho cả hai.
  - kind: uses-operator, target: ==, min: 1
  - kind: uses-operator, target: >, min: 1
- tier: output
  match: regex
  expect: ^True\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giữ thước thì số trùng. Đổi sang thước nhỏ hơn thì số không nhỏ đi — nó lớn lên.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Quay lại cái mẩu thừa lúc nãy. Cứ cho là An đo hụt thật: đặt bàn chân tới lần
thứ mười hai mà luống vẫn còn thừa một mẩu, ngắn hơn một bàn chân. An không có
cái tên nào trong dãy để dán lên cái mẩu ấy. Cách gỡ nhìn ra ngay là **bẻ nhỏ
cái thước** — lấy nửa bàn chân đặt nốt vào chỗ thừa.

Nhưng bẻ nhỏ thước xong lại lòi ra một câu hỏi khác. Nếu đo là đếm, thì chuyện
ngược lại có đúng không — **đếm hạt cũng là đo**, với cái thước là *một hạt*?

Nghe thì khớp. Byte đặt sải dây bốn lần rồi ghi *4 sải*; Byte cũng đặt tay lên
từng hạt mười hai lần rồi ghi *12 hạt*. Cùng một việc, chỉ khác cái thước.

Vậy thì đem đúng cái phép gỡ vừa nói áp cho cả hai thước xem sao: **bẻ đôi chúng
ra.**

Nửa sải dây thì vẫn là dây, vẫn căng ra được, vẫn đặt xuống đất đo được — và
luống đất vẫn còn nguyên chừng ấy để mà đo. Bẻ tiếp nửa nữa cũng thế.

Còn nửa hạt là gì? Bổ đôi một hạt giống ra, nó có còn là hạt không? Gieo xuống
có mọc lên nửa cái cây không?

Nếu hai cái thước ấy chịu bẻ khác nhau như vậy, thì đếm và đo có thật sự là một
việc không — hay chúng giống nhau ở cách làm mà khác nhau ở một chỗ sâu hơn?

Bài sau trả lời, và chỗ khác nhau ấy nằm gọn trong cái thước.
::::

::::checkpoint{mastery=0.8}
::::
