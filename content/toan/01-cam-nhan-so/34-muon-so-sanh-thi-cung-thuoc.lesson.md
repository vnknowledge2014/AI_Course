---
id: toan.cam-nhan-so.muon-so-sanh-thi-cung-thuoc
title: Muốn so sánh thì phải cùng thước
summary: Hai phân số viết bằng hai cỡ thước khác nhau thì không so thẳng được. Quy cả hai về một thước rồi mới đếm xem bên nào nhiều phần hơn.
locale: vi
track: toan
module: cam-nhan-so
order: 34
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.fraction-compare]
requires: [math.fraction, math.compare-on-number-line, math.like-units, math.thanh-so, core.arithmetic, core.variable, core.print-variable, core.boolean, ctrl.comparison]
concepts: [math.phan-so, math.mau-chung, math.thanh-so]
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
Hai con số đo bằng hai cái thước khác nhau thì chưa nói chuyện được với nhau.
::::

::::explain{#hai-thuoc-chua-noi-chuyen-duoc}
Bài trước để lại một câu hỏi thẳng: luống dài **2/3** sải dây và luống dài
**3/4** sải dây — bên nào dài hơn?

Trước khi tính gì, hãy nhớ lại **so sánh nghĩa là gì**. Bài 17 nói: `a > b`
khi a đứng **bên phải** b trên thanh số. Muốn biết bên nào đứng bên phải thì
phải đặt được cả hai lên **cùng một** thanh số — mà muốn đặt lên thanh số thì
phải đếm được cả hai bằng **cùng một** thước.

Bây giờ nhìn hai phân số ấy bằng con mắt của bài 31:

- `2/3` sải là **2 bản sao** của cái thước cỡ `1/3` sải.
- `3/4` sải là **3 bản sao** của cái thước cỡ `1/4` sải.

Hai bên đang đếm bằng hai cái thước khác cỡ. Đây đúng là tình huống của bài
11 — *chỉ gộp được thứ cùng đơn vị* — chỉ khác chỗ lần này ta không gộp mà so.
Nói "3 nhiều hơn 2 nên `3/4` dài hơn" là so **số phần** trong khi hai loại
phần không bằng nhau, y như nói "3 hạt nhiều hơn 2 bó".

Còn so mẫu với mẫu thì trả lời một câu hỏi khác hẳn: mẫu nói **cỡ thước**, và
thước cỡ `1/4` thì nhỏ hơn thước cỡ `1/3`, chứ không nói gì về việc lấy mấy
cái. Hai cách so ấy đều lấy đúng một nửa dữ kiện rồi bỏ nửa kia.

Cách thoát ra thì bài 33 đã đưa sẵn: **đổi thước, không đổi lượng.** Bẻ mỗi
`1/3` thành 4 phần bằng nhau thì được thước `1/12`. Bẻ mỗi `1/4` thành 3 phần
thì cũng được thước `1/12`. Hai bên bây giờ đếm bằng một loại phần duy nhất —
và đếm hai đống cùng loại phần thì so được ngay.

Con số `12` không phải phép màu: nó là `3 × 4`. Lấy mẫu bên này nhân mẫu bên
kia thì **luôn** ra một cỡ thước mà cả hai bên đều đếm chẵn được.
::::

::::example{#dat-hai-luong-len-mot-thuoc}
Byte nhờ máy làm trọng tài. Máy không so phân số hộ Byte — Byte phải tự quy
về cùng thước trước, máy chỉ đếm và trả lời.

```python title=readonly
# Luống A dài 2/3 sải dây, luống B dài 3/4 sải dây.
# Bẻ mỗi 1/3 thành 4 phần, bẻ mỗi 1/4 thành 3 phần — cả hai ra thước 1/12 sải.
a = 2 * 4      # 2/3 sải = 8 phần của thước 1/12
b = 3 * 3      # 3/4 sải = 9 phần của thước 1/12

print(a, b)
print(a > b)
```

Máy in ra:

```text
8 9
False
```

Đọc lại dòng đầu cho kỹ: `8` và `9` **không** phải hai phân số mới. Chúng là
số phần — 8 cái thước `1/12` và 9 cái thước `1/12`. Cùng một loại phần, nên
đem so với nhau được.

Và lượng đất thì không suy suyển tí nào: `2/3` sải vẫn đúng là `2/3` sải, chỉ
là bây giờ nó được đếm bằng thước nhỏ hơn nên ra con số lớn hơn. Đó là bài 3
và bài 33 nói lại một lần nữa.

Dòng thứ hai là `False`: luống A **không** dài hơn luống B. Vậy `2/3 < 3/4`.
::::

::::predict{#doan-luong-nao-dai-hon commitOnce}
Cặp luống thứ hai khó hơn, vì lần này bên có tử lớn hơn lại là bên ngắn hơn.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
# Luống C dài 5/8 sải dây, luống D dài 2/3 sải dây.
# Bẻ mỗi 1/8 thành 3 phần, bẻ mỗi 1/3 thành 8 phần — cả hai ra thước 1/24 sải.
c = 5 * 3
d = 2 * 8

print(c > d)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn so **số phần**: luống C lấy 5 phần còn luống D chỉ lấy 2
phần, và nếu hai bên đang đếm cùng một loại phần thì 5 nhiều hơn 2 là đúng
không cãi được.

Chỗ lệch nằm ở cỡ của cái phần. Phần của C là `1/8` sải, phần của D là `1/3`
sải — phần của D **to hơn** phần của C gần ba lần. Quy cả hai về thước `1/24`
thì C được `5 × 3 = 15` phần, còn D được `2 × 8 = 16` phần. Ít cái thước hơn
mà mỗi cái to hơn, nên D nhỉnh hơn đúng một phần.
::
:::

:::opt
15 16
::why
Gần đúng ở chỗ bạn nhớ đúng hai con số mà hai dòng trên vừa tạo ra, và chúng
đúng là hai số phần cần dùng để trả lời bài toán này. Bạn đã làm xong phần
toán khó nhất.

Chỗ lệch nằm ở việc dòng `print` đang đưa cái gì lên màn hình. Trong ngoặc
không phải `c, d` mà là `c > d` — một **câu hỏi có–không** như Realm 0 đã dạy.
Thứ đi ra khỏi một câu hỏi có–không luôn là `True` hoặc `False`, chưa bao giờ
là một con số. Muốn thấy `15 16` thì viết `print(c, d)`.
::
:::

:::opt
c > d
::why
Gần đúng ở chỗ bạn nghĩ `print` đưa lên màn hình đúng thứ nằm trong ngoặc — và
với `print("c > d")`, có dấu nháy, thì đó là chuyện đang xảy ra thật.

Chỗ lệch: không có dấu nháy thì `c > d` không phải một câu chữ, nó là một việc
cần làm. Máy làm xong việc ấy trước, rồi mới đưa **kết quả** cho `print`. Cái
đi lên màn hình là câu trả lời, không phải câu hỏi.
::
:::
::::

::::explain{#thuoc-nao-cung-duoc-mien-la-cung-mot}
Thước `1/12` ở cặp đầu và thước `1/24` ở cặp sau không phải hai lựa chọn duy
nhất. Cặp đầu dùng thước `1/24` cũng chạy: `2/3` thành 16 phần, `3/4` thành 18
phần, và 16 vẫn nhỏ hơn 18 — cùng một câu trả lời.

Đó không phải may mắn. Bài 33 đã chốt: đổi thước thì con số đổi, **lượng thì
không**. Mà so sánh là so hai lượng, nên câu trả lời không thể phụ thuộc vào
việc bạn chọn cỡ thước nào. Điều duy nhất bắt buộc là **cả hai bên dùng chung
một thước**.

Rút thành một câu mang theo: **muốn so thì quy về cùng mẫu, rồi so tử.** Câu
ấy nghe giống một quy trình học thuộc, nhưng nó không phải mẹo — nó chỉ là
cách viết gọn của "đếm hai lượng bằng cùng một đơn vị rồi xem bên nào nhiều
đơn vị hơn", đúng thứ bạn vẫn làm với hạt và với bó từ bài 6.

Và nó cũng nói luôn vì sao **không** được so tử với tử: hai cái tử ấy đang đếm
bằng hai thước khác cỡ, nên đem so thì so nhầm thứ.
::::

::::code{#trong-tai-cho-hai-cap-luong}
Byte đo bốn luống trong vườn và muốn máy trả lời hai câu hỏi:

- **Sáng** — luống A dài `2/3` sải, luống B dài `3/4` sải. *Luống A có dài hơn
  luống B không?*
- **Chiều** — luống C dài `5/8` sải, luống D dài `2/3` sải. *Luống D có dài
  hơn luống C không?*

Phần quy về cùng thước đã viết sẵn cho bạn ở bốn dòng gán. Việc của bạn là
đặt đúng **câu hỏi** vào hai chỗ trống — và để ý: hai câu hỏi trên hỏi ngược
chiều nhau.

Hai cặp này được chọn để cho ra hai câu trả lời **khác nhau**. Gõ cứng `False`
vào cả hai chỗ thì buổi chiều sai; gõ cứng `True` thì buổi sáng sai. Chỉ hai
câu hỏi viết thật mới qua được cả hai.

```python title=starter
# Sáng — luống A dài 2/3 sải, luống B dài 3/4 sải.
# Cùng thước 1/12 sải: mỗi 1/3 gồm 4 phần, mỗi 1/4 gồm 3 phần.
a = 2 * 4
b = 3 * 3
print(___)

# Chiều — luống C dài 5/8 sải, luống D dài 2/3 sải.
# Cùng thước 1/24 sải: mỗi 1/8 gồm 3 phần, mỗi 1/3 gồm 8 phần.
c = 5 * 3
d = 2 * 8
print(___)
```

```python title=solution
# Sáng — luống A dài 2/3 sải, luống B dài 3/4 sải.
# Cùng thước 1/12 sải: mỗi 1/3 gồm 4 phần, mỗi 1/4 gồm 3 phần.
a = 2 * 4
b = 3 * 3
print(a > b)

# Chiều — luống C dài 5/8 sải, luống D dài 2/3 sải.
# Cùng thước 1/24 sải: mỗi 1/8 gồm 3 phần, mỗi 1/3 gồm 8 phần.
c = 5 * 3
d = 2 * 8
print(d > c)
```

```python title=test
# Bốn assert này chốt lại chính điều bài vừa dạy: quy về cùng thước thì mỗi
# phân số biến thành một SỐ PHẦN, và số phần mới là thứ đem so được.
assert a == 8, "2/3 sải đếm bằng thước 1/12 thì được 8 phần (2 x 4)"
assert b == 9, "3/4 sải đếm bằng thước 1/12 thì được 9 phần (3 x 3)"
assert c == 15, "5/8 sải đếm bằng thước 1/24 thì được 15 phần (5 x 3)"
assert d == 16, "2/3 sải đếm bằng thước 1/24 thì được 16 phần (2 x 8)"
assert a < b, "2/3 NGẮN hơn 3/4, dù 2 và 3 nhìn thì thấy tử bên nào cũng nhỏ hơn mẫu"
assert d > c, "2/3 DÀI hơn 5/8, dù tử 2 nhỏ hơn tử 5 — đây là chỗ so tử với tử sai"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều nằm trong `print`, ngay dưới hai dòng vừa đếm xong số phần. Đọc kỹ câu hỏi của mỗi buổi ở phần đề bài — buổi sáng hỏi A có hơn B, buổi chiều hỏi D có hơn C.
- kind: strategy
  body: Mỗi chỗ trống là một câu hỏi có–không của Realm 0: tên bên trái, dấu lớn hơn, tên bên phải. Vì hai buổi hỏi ngược chiều nhau nên hai cái tên đổi chỗ giữa hai dòng, không chép được dòng này sang dòng kia.
- kind: one-line
  body: "Viết `a > b` vào chỗ trống thứ nhất và `d > c` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh bằng dấu `>` giữa hai số phần — gõ thẳng `True` hay `False` thì không hỏi máy điều gì cả
  requireAst:
  - kind: uses-operator, target: >, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^False\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tám phần với chín phần. Cùng một thước thì mình đếm ra ngay ai hơn ai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cùng thước thì so được. Nhưng vườn thì ít khi chỉ hỏi bên nào dài hơn — sáng
Byte căng `1/2` sải dây, chiều căng thêm `1/3` sải nữa, và câu hỏi là **cả
luống dài bao nhiêu**.

`1/2 + 1/3` bằng bao nhiêu? Có một luật rất dễ nghĩ ra: cộng tử với tử, cộng
mẫu với mẫu, ra `2/5`. Nó gọn, nó đối xứng, nó trông y như cách người ta cộng
mọi thứ khác.

Thử luật ấy trên một phép cộng mà bạn đã biết đáp án: nửa sải cộng nửa sải.
Theo luật ấy thì `1/2 + 1/2 = 2/4`, mà `2/4` rút gọn lại là `1/2`. Gộp hai nửa
sải dây vào nhau xong vẫn còn đúng nửa sải — nửa kia đi đâu mất?

Vậy `1/2 + 1/3` thật ra bằng bao nhiêu, và vì sao nó **không** phải `2/5`? Bài
sau trả lời, và câu trả lời là một luật bạn đã học từ bài 11.
::::

::::checkpoint{mastery=0.8}
::::
