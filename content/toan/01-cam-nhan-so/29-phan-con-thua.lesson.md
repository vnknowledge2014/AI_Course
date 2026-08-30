---
id: toan.cam-nhan-so.phan-con-thua
title: Phần còn thừa
summary: Đo mãi tới lúc không đặt thêm được cái thước nữa thì phần còn lại gọi là số dư — và vì thế dư luôn nhỏ hơn chính cái thước đang đo.
locale: vi
track: toan
module: cam-nhan-so
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.remainder]
requires: [math.division-by-zero, math.division-quotative, core.arithmetic, core.boolean, ctrl.comparison]
concepts: [math.chia-do, math.don-vi, math.thanh-so]
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
Cắt xong vẫn còn một mẩu trên tay mình. Mẩu ấy cũng là một con số đấy.
::::

::::explain{#mau-con-lai-tren-tay}
Bài trước dừng ở chỗ này: Byte có **13 mét** dây, vẫn muốn cắt thành từng đoạn
**3 mét**.

Đặt thước lần 1: hết 3 mét. Lần 2: hết 6 mét. Lần 3: hết 9 mét. Lần 4: hết 12
mét. Lần 5 thì sao? Lần 5 cần tới 15 mét, mà sợi dây chỉ có 13. **Không đặt
được.**

Nên Byte dừng ở 4 đoạn. Và trên tay còn lại một mẩu: `13 − 12 = 1` mét.

Mẩu ấy có tên. Nó gọi là **số dư** — phần còn lại vì *chưa đủ để đặt thêm một
lần nữa*. Không phải "phần thất bại", không phải "phần thừa ra vô ích". Nó là
phần sợi dây có thật, cầm được trên tay, chỉ là chưa đủ dài cho một lần đặt thước.

Viết đầy đủ cả câu chuyện thì được một dòng dựng lại nguyên sợi dây ban đầu:

> 13 = 3 × 4 + 1

Đọc là: *cái thước 3 mét, đặt 4 lần, còn thừa 1 mét*. Ba con số trong dòng ấy
mỗi con một vai — cỡ thước, số lần, phần thừa — và ghép lại thì ra đúng lượng
lúc đầu, không thiếu một xăng-ti-mét nào.
::::

::::explain{#du-luon-nho-hon-cai-thuoc}
Bây giờ tới chỗ đáng giá nhất của bài, và nó là một câu **vì sao**.

Phần thừa có thể là bao nhiêu? Với cái thước 3 mét, thử hết xem: thừa 0 mét được
(khi dây dài 12 mét, cắt vừa khít). Thừa 1 mét được. Thừa 2 mét được.

Thừa **3** mét thì sao? Nếu trên tay còn 3 mét, thì Byte còn đặt thêm được một
lần thước nữa. Nghĩa là Byte **chưa đo xong**, chứ không phải có số dư bằng 3.
Thừa 4 mét cũng vậy: cắt thêm một đoạn nữa rồi vẫn còn 1 mét trên tay.

Rút ra một luật, và luật này đúng vì đúng cái lý do vừa nói ra chứ không phải vì
ai đó quy định:

> **Số dư luôn nhỏ hơn cái thước đang đo.**

Hễ dư còn bằng hoặc lớn hơn cái thước thì bạn còn đặt thêm được — và bạn vẫn
đang ở giữa chừng phép đo.

Đây cũng là cách nhanh nhất để bắt lỗi người khác đếm: ai bảo "cắt 13 mét thành
đoạn 3 mét, được 3 đoạn, thừa 4 mét" thì chưa cần tính lại cũng biết có chỗ chưa
xong, vì 4 lớn hơn 3.
::::

::::example{#ba-buc-tranh-cung-mot-chuyen}
Cùng một chuyện, nhìn bằng ba bức tranh khác nhau — mỗi bức làm lộ ra một mặt.

**Sợi dây.** Trải 13 mét ra, cắt được bốn đoạn bằng nhau và một mẩu ngắn hơn hẳn:

```text
|--3--|--3--|--3--|--3--|-1-|
```

**Thanh số.** Đứng ở vạch 0 rồi bước từng bước dài 3: `0 → 3 → 6 → 9 → 12`. Bước
tiếp theo rơi vào 15, nhảy qua mất số 13. Nên 13 nằm **giữa hai vạch** 12 và 15,
và nó cách vạch 12 đúng 1 bước nhỏ. Con số 1 ấy chính là số dư.

**Mảng hạt.** Xếp 13 hạt thành các hàng, mỗi hàng 3 hạt: được 4 hàng đầy, còn 1
hạt không đủ lập hàng mới. Số dư là cái hàng dở dang.

Bắt máy làm trọng tài cho bức tranh thứ nhất:

```python title=readonly
print(3 * 4)
print(13 - (3 * 4))
print((3 * 4) + 1 == 13)
```

Máy in ra:

```text
12
1
True
```

Dòng 1: bốn đoạn ăn hết 12 mét. Dòng 2: phần còn trên tay là 1 mét. Dòng 3: bốn
đoạn cộng cái mẩu thừa thì dựng lại đúng sợi dây 13 mét ban đầu — `True`.
::::

::::predict{#doan-cau-tra-loi commitOnce}
An cũng cắt sợi dây 13 mét ấy bằng thước 3 mét, nhưng An đếm được **3 đoạn** và
bảo *"còn thừa 4 mét"*.

Byte không tranh cãi. Byte hỏi máy đúng một câu: phần thừa An nói có nhỏ hơn cái
thước không?

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
thua_theo_an = 13 - (3 * 3)
print(thua_theo_an < 3)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn đang áp đúng cái luật vừa học: số dư thì luôn nhỏ hơn cái
thước. Luật ấy không sai chút nào.

Chỗ ra ngoài phạm vi: luật đó chỉ nói về phần thừa của một phép đo **đã làm tới
cùng**. An dừng ở đoạn thứ ba trong khi vẫn còn đặt thêm được một lần nữa, nên
con số 4 của An chưa phải số dư — nó là phần dây chưa đo hết. Máy so số thật với
số thật và trả lời `False`, và chính chữ `False` ấy là dấu hiệu An dừng sớm.
::
:::

:::opt
4
::why
Gần đúng ở chỗ bạn tính không sai một bước: `3 × 3 = 9`, rồi `13 − 9 = 4`. Con số
4 đúng là thứ đang nằm trong `thua_theo_an`, và nó đúng là chỗ khiến cách đếm của
An lộ ra chỗ chưa xong.

Chỗ ra ngoài phạm vi: dòng `print` không in cái tên ấy, nó in kết quả của dấu `<`.
Dấu `<` là một câu hỏi có–không (R0 bài 24), nên thứ đi ra khỏi nó luôn là `True`
hoặc `False`, không bao giờ là một con số. Muốn thấy số 4 thì viết
`print(thua_theo_an)`.
::
:::

:::opt
1
::why
Gần đúng ở chỗ bạn nhớ đúng đáp số thật của bài: 13 mét cắt bằng thước 3 mét thì
dư 1 mét, không phải 4. Bạn đang giữ đúng con số cần giữ.

Chỗ ra ngoài phạm vi: hai dòng code này không tính phần dư thật. Chúng tính theo
**cách đếm của An** — số 3 thứ hai trong `3 * 3` là 3 đoạn An đếm được, chứ không
phải 4 đoạn của Byte. Máy tính đúng cái nó được bảo tính, và ở đây nó được bảo
tính con số của An.
::
:::
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Hai lần đo, hai cái thước khác nhau. Mẩu thừa nào cũng phải ngắn hơn thước.
::::

::::code{#do-hai-lan-trong-vuon}
Hôm nay Byte đo hai thứ trong vườn, bằng hai cái thước khác nhau:

- **Sợi dây**: 13 mét, cắt thành từng đoạn 3 mét. Byte cắt được **4 đoạn**.
- **Nắm hạt**: 20 hạt, đóng thành từng gói 6 hạt. Byte đóng được **3 gói**.

Mỗi lần đo in hai dòng: dòng trên là phần còn thừa, dòng dưới là câu kiểm
*"phần thừa có nhỏ hơn cái thước không"*. Câu kiểm của lần đo thứ nhất đã viết
sẵn làm mẫu; câu kiểm của lần đo thứ hai thì bạn tự viết lấy. Ba chỗ trống tất
cả: hai chỗ tính phần còn thừa, một chỗ phát biểu chính cái luật của bài.

Bài chấm bằng **cả hai** lần đo, và hai lần này cố tình dùng hai cỡ thước khác
nhau với hai số dư khác nhau. Gõ cứng một con số vào hai chỗ tính thì chỉ đúng
được nhiều nhất một lần; chỉ phép tính viết thật mới qua được cả hai. Chỗ trống
thứ ba cũng vậy: nó phải hỏi bằng **cái tên** vừa tính ra, không phải bằng con
số chép lại.

```python title=starter
thua_day = ___
print(thua_day)
print(thua_day < 3)

thua_hat = ___
print(thua_hat)
print(___)
```

```python title=solution
thua_day = 13 - (3 * 4)
print(thua_day)
print(thua_day < 3)

thua_hat = 20 - (6 * 3)
print(thua_hat)
print(thua_hat < 6)
```

```python title=test
# Hai lần đo, hai cỡ thước, hai số dư. Bốn assert đầu chốt lại đúng dòng
# `lượng ban đầu = thước × số lần + dư`; hai assert cuối chốt luật của bài.
assert thua_day == 1, "13 mét cắt bằng thước 3 mét, được 4 đoạn thì còn thừa 1 mét"
assert thua_hat == 2, "20 hạt đóng gói 6 hạt, được 3 gói thì còn thừa 2 hạt"
assert (3 * 4) + thua_day == 13, "bốn đoạn cộng phần thừa phải dựng lại đúng sợi dây 13 mét"
assert (6 * 3) + thua_hat == 20, "ba gói cộng phần thừa phải dựng lại đúng nắm 20 hạt"
assert thua_day < 3, "dư phải nhỏ hơn cái thước 3 mét — nếu không thì còn cắt thêm được"
assert thua_hat < 6, "dư phải nhỏ hơn cái gói 6 hạt — nếu không thì còn đóng thêm được"
```

:::hints
- kind: attention
  body: Phần còn thừa là chỗ chênh giữa lượng lúc đầu và phần đã bị các lần đặt thước ăn hết. Đề bài cho sẵn cả ba con số cần dùng cho mỗi lần đo.
- kind: strategy
  body: Với sợi dây, các đoạn ăn hết `3 * 4` mét, còn lúc đầu có 13 mét. Lấy cái lớn trừ đi cái đã bị ăn hết là ra phần trên tay. Nắm hạt làm y hệt, chỉ đổi ba con số. Chỗ trống cuối thì chép đúng hình dạng của dòng câu kiểm ở lần đo thứ nhất, đổi sang tên và cỡ thước của lần đo thứ hai.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `13 - (3 * 4)`, `___` thứ hai bằng `20 - (6 * 3)`, và `___` thứ ba bằng `thua_hat < 6`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống đầu phải là một phép tính có nhân và có trừ (`lượng ban đầu - (thước * số lần)`), còn chỗ trống thứ ba phải hỏi `thua_hat` có nhỏ hơn 6 không — chép thẳng con số đáp án thì không tính và không kiểm được gì
  requireAst:
  # Hai lần đo, mỗi lần một phép nhân và một phép trừ. Khung chưa có dấu `*`
  # hay `-` nào, nên luật này chặn được đúng cái đáp án chép cứng hai con số.
  - kind: uses-operator, target: *, min: 2
  - kind: uses-operator, target: -, min: 2
  # Khung có sẵn đúng MỘT dấu `<` (câu kiểm mẫu của lần đo thứ nhất), nên
  # `min: 2` ép chỗ trống thứ ba phải là câu kiểm thật chứ không phải `True`.
  - kind: uses-operator, target: <, min: 2
  # Và khung mới ĐỌC `thua_hat` một lần, nên `min: 2` ép câu kiểm ấy hỏi bằng
  # cái tên vừa tính ra, chặn đáp án `print(2 < 6)` chép cứng con số.
  - kind: uses-name, target: thua_hat, min: 2
- tier: output
  match: regex
  expect: ^1\nTrue\n2\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thừa 1 mét và thừa 2 hạt. Cả hai đều ngắn hơn thước — đo tới cùng rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte cầm cái mẩu 1 mét trên tay và muốn ghi nó vào sổ. Nhưng cả buổi sáng nay
Byte đo bằng đúng một đơn vị: **đoạn 3 mét**. Sổ ghi "4 đoạn". Cái mẩu này thì
ghi là mấy đoạn?

Không phải 1 đoạn — nó ngắn hơn hẳn một đoạn. Cũng không phải 0 đoạn — 0 đoạn
nghĩa là tay trắng, mà tay Byte đang cầm một mẩu dây thật.

Trên thanh số, chỗ của nó cũng vậy: đo bằng thước 3 mét thì các vạch rơi vào 0,
1, 2, 3, 4… đoạn — còn cái mẩu này rơi vào **giữa vạch 0 và vạch 1**, đúng cái
chỗ trống mà bài 4 đã chỉ tay vào.

Cái thước hiện có không với tới chỗ ấy được. Vậy phải làm gì với cái thước? Bài
sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
