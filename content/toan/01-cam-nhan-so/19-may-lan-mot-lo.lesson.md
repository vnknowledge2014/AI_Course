---
id: toan.cam-nhan-so.may-lan-mot-lo
title: Mấy lần một lô
summary: Nhân không phải một phép mới, nó là "lấy mấy lần một lượng" — và hai con số trong đó đứng hai vai khác hẳn nhau.
locale: vi
track: toan
module: cam-nhan-so
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.multiplication]
requires: [core.arithmetic, ctrl.comparison, core.boolean, core.variable, core.print-variable, core.output]
concepts: [math.lo-va-so-lo, math.don-vi-moi]
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
Cộng năm mươi lần thì mỏi tay. Mình đếm lô, không đếm từng cây.
::::

::::explain{#gop-cung-mot-luong-mai}
Bài trước để lại một câu hỏi thẳng: mỗi luống 8 cây, có 5 luống thì cộng 8 năm
lần là xong. Nhưng 50 luống thì cộng 50 lần à?

Trước khi tìm cách viết gọn, hãy nhìn kỹ cái phép cộng ấy một lần:

```text
8 + 8 + 8 + 8 + 8
```

Nó khác mọi phép cộng bạn làm từ bài 10 tới bài 18 ở một điểm: **mọi số hạng
đều giống nhau**. Ở bài 10, "3 hạt gộp 2 hạt" là hai đống khác nhau đổ chung.
Ở đây thì không có đống nào khác đống nào — chỉ có **một** lượng, là 8 cây, và
nó xuất hiện đi xuất hiện lại.

Khi mọi số hạng giống nhau, phép cộng không còn cần kể ra từng số hạng nữa.
Chỉ cần hai thông tin là dựng lại được cả dãy:

- **Một lượt lấy bao nhiêu** — ở đây là 8 cây.
- **Lấy bao nhiêu lượt** — ở đây là 5 lượt.

Byte gọi cái lượng lấy mỗi lượt là một **lô**. Một luống 8 cây là một lô. Và
câu hỏi "5 luống mỗi luống 8 cây là bao nhiêu cây" đọc lại thành: *lấy 5 lần
cái lô 8 cây*.

Đó chính là **phép nhân**. Viết ra là `8 × 5`, trong Python là `8 * 5`.

Chỗ này bạn đã đi qua một lần rồi mà chưa gọi tên. Bài 6 gom mười hạt thành
**một bó** rồi đi đếm bó — cái bó ấy đúng là một lô, lô cỡ 10. Bài 6 chỉ cho
phép lô cỡ 10; phép nhân bỏ hạn chế đó đi, lô cỡ bao nhiêu cũng được. Lô 8 cây,
lô 200 gam gạo, lô 3 mét vải.
::::

::::explain{#hai-con-so-hai-vai}
Bây giờ tới một chỗ đáng nhìn kỹ, và nó không phải chuyện tính toán.

Trong `8 * 5`, hai con số **không cùng loại**. Chúng trả lời hai câu hỏi khác
nhau:

| con số | trả lời câu hỏi | đơn vị của nó |
|---|---|---|
| 8 | một lô to bao nhiêu? | cây — nhưng là cây của MỘT luống |
| 5 | lấy mấy lô? | luống |

Kết quả 40 mang đơn vị **cây** — không phải luống, cũng không phải "cây của một
luống". Nhìn lại bài 1: một con số luôn là "mấy **cái gì**". Phép nhân là chỗ
cái "cái gì" ấy đổi giữa chừng: hai đầu vào mang hai đơn vị khác nhau, và ở đây
đầu ra mang một đơn vị thứ ba: cây.

Cái đơn vị hai tầng của số 8 — "cây của một luống" — có tên riêng và có cách
viết riêng; bài 40 sẽ lấy nó ra. Ở bài này chỉ cần nhận ra nó khác đơn vị của
số 5, thế là đủ.

Đây cũng là lý do bạn **không** cộng được 8 với 5 ở đây. Bài 11 đã chốt: chỉ
gộp được thứ cùng đơn vị. 8 cây-mỗi-luống và 5 luống không cùng đơn vị, nên
`8 + 5 = 13` là một phép tính chạy được nhưng con số 13 không đếm cái gì trong
vườn cả.

Nói cách khác: `+` lúc nào cũng **đòi** hai bên cùng đơn vị — không có ngoại
lệ. Còn `×` thì **cho phép** hai bên khác vai, và trong cách đọc "lấy mấy lần
một lô" thì chúng khác vai thật: một cái là cỡ lô, một cái là số lô. Phép nhân
còn những bức tranh khác — bài 22 sẽ đưa một cái, ở đó con số thứ hai không đo
cái gì cả.
::::

::::example{#hoi-thang-cai-may}
Bắt máy làm trọng tài cho đúng câu vừa nói: cộng 8 năm lần và nhân 8 với 5 có
ra cùng một số không?

```python title=readonly
print(8 + 8 + 8 + 8 + 8)
print(8 * 5)
print(8 + 8 + 8 + 8 + 8 == 8 * 5)
```

Máy in ra:

```text
40
40
True
```

Dòng thứ ba là câu trả lời đáng giá nhất: `True` nghĩa là hai cách viết ấy nói
về đúng một con số. Phép nhân không mang thêm một sự thật mới nào vào vườn —
nó chỉ là cách viết ngắn cho một việc bạn đã làm được từ bài 10.

Và đây là chỗ nó trả công:

```python title=readonly
print(8 * 50)
```

```text
400
```

Một dòng, không phải năm mươi dấu cộng.
::::

::::predict{#doan-ba-dong commitOnce}
Byte đặt tên cho hai con số của vườn rồi in ra ba dòng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
mot_luong = 8
so_luong = 5

print(mot_luong + mot_luong + mot_luong + mot_luong + mot_luong)
print(mot_luong * so_luong)
print(mot_luong + so_luong)
```

:::opt{correct}
40 rồi 40 rồi 13
:::

:::opt
40 rồi 40 rồi 40
::why
Gần đúng ở chỗ bạn giữ chặt một điều đúng: cả ba dòng đều nói về đúng một cái
vườn — 5 luống, mỗi luống 8 cây — nên đáng lẽ chúng phải cùng ra một số cây.

Chỗ lệch nằm ở dòng ba: nó **không** nói về cái vườn ấy. `mot_luong + so_luong`
đem "8 cây mỗi luống" cộng với "5 luống", tức là gộp hai thứ khác đơn vị. Bài
11 nói phép cộng chỉ có nghĩa khi hai lượng cùng đơn vị — và ở đây chúng không.
Máy vẫn tính được, vẫn in ra 13, nhưng 13 ấy không phải 13 cây, cũng không phải
13 luống. Nó là một con số không đếm cái gì cả.
::
:::

:::opt
40 rồi 13 rồi 13
::why
Gần đúng ở chỗ bạn đọc `mot_luong * so_luong` như một câu hỏi về đúng hai con
số 8 và 5 — và nó đúng là câu hỏi về hai con số ấy.

Chỗ lệch là ở việc nó hỏi **gì**. Dấu `*` không hỏi "đổ chung hai đống này ra
bao nhiêu"; nó hỏi "lấy cái lô 8 cây ấy **5 lượt** thì ra bao nhiêu". Đổ chung
thì con số 5 được cộng vào đúng một lần. Lấy 5 lượt thì con số 8 được lấy lại
năm lượt. Hai câu hỏi khác nhau nên hai kết quả khác nhau: 13 và 40.
::
:::

:::opt
88888 rồi 40 rồi 13
::why
Gần đúng ở chỗ bạn nhớ dấu `+` có hai việc: với chữ thì nó **ghép** hai mảnh
lại thành một câu dài. Cách đọc ấy đúng — nhưng chỉ đúng cho **dòng đầu**. Nếu
`mot_luong` giữ `"8"` có dấu nháy thì dòng đầu thật sự dán ra `88888`; còn dòng
hai sẽ dán tiếp thành `88888` chứ không phải 40, và dòng ba thì máy dừng hẳn —
nó không cộng được một mảnh chữ với một con số.

Chỗ lệch nằm ở dòng đầu tiên của chương trình: `mot_luong = 8`, viết không có
dấu nháy. Nhãn của nó là số, nên dấu `+` ở đây tính chứ không ghép.
::
:::
::::

::::explain{#lo-la-mot-cai-thuoc}
Còn một cách đọc nữa cho `8 * 5`, và nó nối thẳng về bài 3.

Bài 3 nói: đo là đếm xem cái thước đã chọn đặt lặp lại mấy lần. Bây giờ lấy
**một luống 8 cây** làm cái thước. Đo cả vườn bằng cái thước ấy thì được con số
5. Đo lại cả vườn ấy bằng cái thước "một cây" thì được 40.

Cùng một vườn, hai cái thước, hai con số — đúng y chuyện Byte và An đo luống đất
ở bài 2. Phép nhân là cái cầu nối hai con số đó: nó đổi từ *đếm bằng lô* sang
*đếm bằng cây*.

Nên `8 * 5` đọc được theo ba cách, và cả ba đều là một:

- 8 cộng lại 5 lần.
- Lấy 5 lô, mỗi lô 8.
- Cái lượng mà đem đo bằng thước cỡ 8 thì đặt vừa đúng 5 lần — lượng ấy là 40.
  (Đọc ngược cách này lại thì thành phép chia; bài 27 sẽ cầm nó lên.)
::::

::::code{#dong-gao-hai-buoi}
Byte đong gạo bằng lon. Mỗi lon **200 gam**, không lon nào vơi lon nào đầy.

- **Sáng**: đong 4 lon.
- **Chiều**: đong 7 lon.

Điền vào hai chỗ trống để máy in ra số **gam** gạo của từng buổi. Hai buổi có
số lon khác nhau, nên một con số chép cứng chỉ đúng được nhiều nhất một buổi —
và cách chấm còn đòi bạn dùng lại cái tên `mot_lon` thay vì tự nhân nhẩm rồi gõ
kết quả vào.

```python title=starter
mot_lon = 200

sang = ___
chieu = ___

print(sang)
print(chieu)
```

```python title=solution
mot_lon = 200

sang = mot_lon * 4
chieu = mot_lon * 7

print(sang)
print(chieu)
```

```python title=test
# Chấm bằng hai buổi, và mỗi buổi được đối chiếu với đúng cái phép cộng mà
# bài này thay thế. Nếu một ngày nào đó dấu `*` trong bài bị viết nhầm thành
# một phép khác, cổng đỏ lên chứ không dạy sai lặng lẽ.
assert sang == 200 + 200 + 200 + 200, "4 lon là lấy lô 200 gam bốn lượt — phải bằng đúng bốn lần cộng"
assert chieu == 200 + 200 + 200 + 200 + 200 + 200 + 200, "7 lon là lấy lô 200 gam bảy lượt"
assert sang == 800 and chieu == 1400, "sáng 800 gam, chiều 1400 gam"
assert chieu - sang == 600, "chiều hơn sáng đúng 3 lon, tức 600 gam"
```

:::hints
- kind: attention
  body: Dòng đầu đã đặt tên cho **cỡ một lô** — 200 gam. Hai chỗ trống còn thiếu thông tin thứ hai: mỗi buổi lấy **mấy lô**.
- kind: strategy
  body: Mỗi chỗ trống là một phép nhân giữa cái tên giữ cỡ lô và số lon của buổi đó. Sáng 4 lon, chiều 7 lon — hai con số khác nhau nên hai dòng không chép được cho nhau.
- kind: one-line
  body: "Viết `mot_lon * 4` vào chỗ trống thứ nhất và `mot_lon * 7` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi buổi phải là một phép nhân dùng lại cái tên `mot_lon` — gõ thẳng con số 800 hay 1400 thì bạn đã nhân hộ máy rồi
  requireAst:
  - kind: uses-operator, target: *, min: 2
  - kind: uses-name, target: mot_lon, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^800\n1400\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
800 với 1400. Mình chỉ cần biết lô to bao nhiêu và lấy mấy lô.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài này vừa nhấn mạnh rằng hai con số trong phép nhân đứng **hai vai khác
nhau**: một cái nói lô to bao nhiêu, cái kia nói lấy mấy lô.

Vậy thử đổi vai chúng cho nhau xem.

- Vườn thứ nhất: **5 luống, mỗi luống 8 cây.**
- Vườn thứ hai: **8 luống, mỗi luống 5 cây.**

Đi ngoài đồng thì hai cái vườn ấy trông khác hẳn nhau — một cái dài và ít
luống, một cái ngắn và nhiều luống. Bạn không thể nhầm cái này với cái kia.

Nhưng số cây thì sao? Có khác nhau không, và nếu bằng nhau thì **vì sao** bằng?
Cộng 8 năm lần và cộng 5 tám lần là hai dãy số chẳng giống nhau chỗ nào — nhìn
vào hai dãy ấy không thấy được lý do gì để chúng ra cùng một kết quả.

Bài sau vẽ ra một bức tranh chỉ cần nhìn một cái là thấy.
::::

::::checkpoint{mastery=0.8}
::::
