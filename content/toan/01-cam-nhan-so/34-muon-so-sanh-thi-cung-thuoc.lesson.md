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
estimatedMinutes: 14
teaches: [math.fraction-compare]
requires: [math.equivalent-fraction, math.fraction, math.compare-on-number-line, math.like-units, math.thanh-so, core.arithmetic, core.variable, core.print-variable, core.boolean, ctrl.comparison]
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
Byte đo sáu luống trong vườn và muốn máy trả lời ba câu hỏi:

- **Sáng** — luống A dài `2/3` sải, luống B dài `3/4` sải. *Luống A có dài hơn
  luống B không?*
- **Chiều** — luống C dài `5/8` sải, luống D dài `2/3` sải. *Luống D có dài
  hơn luống C không?*
- **Tối** — luống E dài `4/7` sải, luống F dài `5/9` sải. *Luống E có dài hơn
  luống F không?*

Việc chính của bạn là **quy từng lượng về thước chung**: mỗi chỗ trống ở các
dòng gán hỏi đúng một câu — *một cái `1/…` bên này gồm mấy phần của thước
chung?* Hai buổi đầu bài đã đặt sẵn câu hỏi so sánh cho bạn nhìn mẫu; buổi tối
thì bạn tự viết câu hỏi ấy.

Ba cặp dùng ba cỡ thước khác nhau, nên không cặp nào chép được cặp nào. Và
chỗ trống trong `print` cuối là một **câu hỏi**, không phải một câu trả lời:
gõ thẳng `True` vào đó thì bạn không hỏi máy điều gì cả, và bài sẽ chặn.

```python title=starter
# Sáng — luống A dài 2/3 sải, luống B dài 3/4 sải. Cùng thước 1/12 sải.
# Mỗi 1/3 gồm mấy phần của thước 1/12? Mỗi 1/4 gồm mấy phần?
a = 2 * ___
b = 3 * ___
print(a > b)

# Chiều — luống C dài 5/8 sải, luống D dài 2/3 sải. Cùng thước 1/24 sải.
# Mỗi 1/8 gồm mấy phần của thước 1/24? Mỗi 1/3 gồm mấy phần?
c = 5 * ___
d = 2 * ___
print(d > c)

# Tối — luống E dài 4/7 sải, luống F dài 5/9 sải. Thước chung bạn tự chọn:
# mẫu này nhân mẫu kia, 7 x 9 = 63, nên cả hai đếm bằng thước 1/63 sải.
e = 4 * ___
f = 5 * ___
print(___)
```

```python title=solution
# Sáng — luống A dài 2/3 sải, luống B dài 3/4 sải. Cùng thước 1/12 sải.
# Mỗi 1/3 gồm mấy phần của thước 1/12? Mỗi 1/4 gồm mấy phần?
a = 2 * 4
b = 3 * 3
print(a > b)

# Chiều — luống C dài 5/8 sải, luống D dài 2/3 sải. Cùng thước 1/24 sải.
# Mỗi 1/8 gồm mấy phần của thước 1/24? Mỗi 1/3 gồm mấy phần?
c = 5 * 3
d = 2 * 8
print(d > c)

# Tối — luống E dài 4/7 sải, luống F dài 5/9 sải. Thước chung bạn tự chọn:
# mẫu này nhân mẫu kia, 7 x 9 = 63, nên cả hai đếm bằng thước 1/63 sải.
e = 4 * 9
f = 5 * 7
print(e > f)
```

```python title=test
# Chín assert này chốt lại chính điều bài vừa dạy: quy về cùng thước thì mỗi
# phân số biến thành một SỐ PHẦN, và số phần mới là thứ đem so được. Sáu cái
# đầu chấm đúng bước quy về thước chung — bước mà bài này mang tới.
assert a == 8, "2/3 sải đếm bằng thước 1/12 thì được 8 phần: mỗi 1/3 gồm 4 phần, nên 2 x 4"
assert b == 9, "3/4 sải đếm bằng thước 1/12 thì được 9 phần: mỗi 1/4 gồm 3 phần, nên 3 x 3"
assert c == 15, "5/8 sải đếm bằng thước 1/24 thì được 15 phần: mỗi 1/8 gồm 3 phần, nên 5 x 3"
assert d == 16, "2/3 sải đếm bằng thước 1/24 thì được 16 phần: mỗi 1/3 gồm 8 phần, nên 2 x 8"
assert e == 36, "4/7 sải đếm bằng thước 1/63 thì được 36 phần: mỗi 1/7 gồm 9 phần, nên 4 x 9"
assert f == 35, "5/9 sải đếm bằng thước 1/63 thì được 35 phần: mỗi 1/9 gồm 7 phần, nên 5 x 7"
assert a < b, "2/3 NGẮN hơn 3/4: cùng thước 1/12 thì 8 phần ít hơn 9 phần — mẫu 3 nhỏ hơn mẫu 4 chỉ nói thước bên A to hơn, không nói lượng bên A nhiều hơn"
assert d > c, "2/3 DÀI hơn 5/8, dù tử 2 nhỏ hơn tử 5 — đây là chỗ so tử với tử sai"
assert e > f, "4/7 DÀI hơn 5/9: cùng thước 1/63 thì 36 phần nhiều hơn 35 phần, dù tử 4 nhỏ hơn tử 5"
```

:::hints
- kind: attention
  body: Sáu chỗ trống ở các dòng gán hỏi đúng một kiểu câu, và bài 33 đã dạy cách trả lời — cùng một lượng đo bằng cái thước nhỏ hơn mấy lần thì đếm ra nhiều phần hơn bấy nhiêu lần. Chỗ trống thứ bảy nằm trong `print` cuối; đề bài buổi tối nói rõ nó hỏi chiều nào.
- kind: strategy
  body: Thước `1/12` nhỏ hơn thước `1/3` bốn lần và nhỏ hơn thước `1/4` ba lần, nên mỗi `1/3` gồm 4 phần còn mỗi `1/4` gồm 3 phần — đó là con số nhân vào. Làm y hệt với thước `1/24` (nhỏ hơn `1/8` ba lần, nhỏ hơn `1/3` tám lần) và thước `1/63` (nhỏ hơn `1/7` chín lần, nhỏ hơn `1/9` bảy lần). Chỗ trống cuối viết như hai dòng `print` trên, chỉ đổi hai cái tên.
- kind: one-line
  body: "Điền lần lượt `4`, `3` rồi `3`, `8` rồi `9`, `7` vào sáu dòng gán, và `e > f` vào chỗ trống trong `print` cuối."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống cuối phải là một câu hỏi so sánh bằng dấu `>` giữa hai số phần — gõ thẳng `True` hay `False` thì không hỏi máy điều gì cả
  requireAst:
  - kind: uses-operator, target: >, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^False\nTrue\nTrue\s*$
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
