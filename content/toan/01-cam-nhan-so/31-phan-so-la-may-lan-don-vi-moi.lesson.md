---
id: toan.cam-nhan-so.phan-so-la-may-lan-don-vi-moi
title: Phân số là mấy lần đơn vị mới
summary: "`3/4` không phải ba cái bánh chia tư — nó là ba lần cái thước `1/4`. Mẫu nói thước cỡ nào, tử nói lấy mấy cái."
locale: vi
track: toan
module: cam-nhan-so
order: 31
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.fraction]
requires: [math.unit-fraction, math.thanh-so, math.multiplication, math.division-partitive, math.compare-on-number-line, math.don-vi, core.output, core.arithmetic, core.division, core.boolean, ctrl.comparison]
concepts: [math.don-vi-va-thuoc, math.thanh-so, math.tu-va-mau]
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
Bài trước bạn bẻ được cái thước. Giờ mình đem nó ra đặt xuống đất.
::::

::::explain{#ba-lan-cai-thuoc}
Bài trước bẻ sải dây của Byte làm bốn phần bằng nhau. Mỗi phần là một cái thước
mới, và tên của nó là `1/4` sải.

Câu hỏi để lại: `3/4` là gì — **ba cái bánh mỗi cái chia tư**, hay **ba lần cái
thước mới ấy**?

Đem cả hai cách đọc ra vườn thử.

**Cách đọc thứ nhất — ba cái bánh.** Lấy ba cái bánh chưng, cắt mỗi cái làm tư,
mỗi cái nhặt một miếng. Được ba miếng. Không sai chỗ nào.

**Cách đọc thứ hai — ba lần cái thước.** Lấy đúng một sải dây đã bẻ tư. Đặt cái
thước `1/4` xuống đất, rồi nhấc lên đặt tiếp, rồi đặt lần thứ ba. Cũng ra chừng
ấy đất.

Hai cách cho cùng một lượng, nên nhìn thoáng qua thì tưởng chúng là một. Chỗ
khác nhau nằm ở **cần bao nhiêu cái bánh**: cách thứ nhất đòi ba cái, cách thứ
hai chỉ cần một cái thước và cho phép đặt lại bao nhiêu lần tuỳ bạn.

Bài này chọn cách đọc thứ hai. Và cái được chọn không phải là một cách nói cho
hay — nó là một câu định nghĩa:

> `a/b` là **a bản sao của cái thước `1/b`**.

Đọc lại cho quen: `3/4` là ba lần thước `1/4`. `2/5` là hai lần thước `1/5`.
`1/8` là một lần thước `1/8`.

Bạn đã gặp đúng hình dạng câu ấy ở bài 19: *mấy lần một lô*. Một con số nói lô
to bao nhiêu, con số kia nói lấy mấy lô. Phân số không phải một phép toán mới
rơi từ trên trời xuống. Nó là bài 19, với cái lô nhỏ hơn một đơn vị.
::::

::::example{#mot-cho-tren-thanh-so}
Điều quan trọng nhất của cách đọc này lộ ra trên **thanh số** (bài 5).

Trên thanh số, hai vạch liền nhau cách nhau đúng một sải. Bẻ tư cái khoảng ấy
thì giữa vạch `0` và vạch `1` mọc thêm ba vạch nhỏ — và mỗi vạch nhỏ là một
**chỗ**, y như mọi chỗ khác:

```text
0        1/4       2/4       3/4        1
├─────────┼─────────┼─────────┼─────────┤
                              ▲
                      ba lần cái thước 1/4 — tay bạn dừng ở đây
```

Đây là chỗ cách đọc "miếng bánh" không theo kịp. Một miếng bánh nằm trong đĩa,
nó không đứng ở đâu cả. Còn `3/4` thì đứng đúng một chỗ trên cùng cái đường mà
`0`, `1`, `2` và cả `−5` của bài 16 đang đứng.

Nói gọn: **phân số là một con số đàng hoàng**, không phải một mảnh vỡ của số
khác.

Cái vườn cũng nói đúng điều ấy, bằng một bức tranh thứ hai — sơ đồ dải:

```text
một sải dây      ├───────────────────────────────┤
bẻ làm bốn       ├───────┼───────┼───────┼───────┤
                    1/4     1/4     1/4     1/4

luống rau muống  ├───────┼───────┼───────┤
                 đặt thước 1/4 ba lần  →  3/4 sải
```

Giờ hỏi máy cho chắc. Máy không có gạch phân số, nhưng viết `3/4` trên một dòng
thì nó hiểu — với nó đó là phép chia của bài 26. Có một chỗ phải cẩn thận: in
thẳng `3/4` ra thì máy trả lời bằng **số thập phân**, cách viết mà mãi bài 36
mới tới. Nên suốt track này, ta chỉ nhờ máy làm **trọng tài**: phát biểu một
khẳng định rồi bắt nó trả `True` hoặc `False`.

```python title=readonly
print(3/4 == 1/4 + 1/4 + 1/4)
```

Máy in ra:

```text
True
```

Máy vừa đồng ý: ba cái thước `1/4` đặt liền nhau đúng bằng `3/4`. Nhưng máy
không chứng minh gì cả — bằng chứng nằm ngoài vườn, ở chỗ bạn đặt thước ba lần
mà không thêm không bớt một tấc đất nào.
::::

::::predict{#doan-ben-nao-dai-hon commitOnce}
Byte đo luống rau muống bằng thước `1/4` sải: đặt vừa **ba** lần.
An đo luống hành bằng thước `1/5` sải: cũng đặt vừa **ba** lần.

Hai luống cùng "ba miếng". Byte sắp hỏi máy xem luống rau muống có dài hơn
luống hành không.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(3/4 > 3/5)
```

:::opt{correct}
True
:::

:::opt
False
::why
Gần đúng ở chỗ bạn so hai con số dưới gạch và thấy `5` lớn hơn `4` — con số ấy
lớn hơn thật, bạn không đọc nhầm chữ nào.

Chỗ lệch nằm ở việc con số dưới gạch **nói cái gì**. Nó không nói lượng đất; nó
nói cái đơn vị bị bẻ làm mấy phần. Bẻ một sải làm 5 thì mỗi miếng phải **nhỏ
hơn** bẻ làm 4 — chia cho càng nhiều phần thì mỗi phần càng ít, đúng như bài 26
đã cho thấy.

Hai bên cùng lấy ba miếng, bên nào miếng to hơn thì bên ấy nhiều đất hơn. Miếng
`1/4` to hơn miếng `1/5`, nên `3/4` mới là bên dài hơn.
::
:::

:::opt
3/4
::why
Gần đúng ở chỗ bạn đọc dấu `>` như câu hỏi "bên nào lớn hơn" — và đó đúng là
câu hỏi cả bài đang theo đuổi, bạn nhắm không trượt.

Chỗ lệch nằm ở thứ đi ra khỏi dấu ấy. `>` là một **câu hỏi có–không** (Realm 0
bài 24 gọi nó là câu đúng–sai), nên thứ máy đưa lại luôn là `True` hoặc
`False`, chưa bao giờ là một con số. Muốn thấy con số thì phải hỏi máy bằng một
dòng khác.
::
:::

:::opt
Máy báo lỗi vì hai phân số khác mẫu thì chưa so được
::why
Gần đúng ở chỗ bạn thấy hai cái thước khác cỡ và nghĩ rằng phải làm cho chúng
cùng cỡ đã. Ý nghĩ ấy có thật và sẽ được dùng tới — nó là chỗ bạn tự tìm ra
việc phải làm khi muốn **cộng** hai luống đo bằng hai thước khác nhau.

Chỗ lệch: so sánh thì không cần chờ. Với máy, `3/4` và `3/5` là hai phép chia
đã có kết quả, tức là hai **chỗ** trên thanh số. Mà hai chỗ thì luôn so được —
bài 17: cái nào đứng bên phải, cái ấy lớn hơn.
::
:::
::::

::::explain{#mau-noi-co-thuoc-tu-noi-may-cai}
Rút ra hai vai, và đừng để chúng lẫn vào nhau:

- Số **dưới** gạch — gọi là **mẫu** — nói cái thước **cỡ nào**: đơn vị bị bẻ
  làm mấy phần bằng nhau. Mẫu càng lớn thì thước càng nhỏ.
- Số **trên** gạch — gọi là **tử** — nói **lấy mấy cái** thước ấy. Nó là một số
  đếm, đúng nghĩa đếm của bài 2.

Còn cái gạch ngang giữa hai số thì cũng có việc của nó: nó **gom** hai con số
lại thành một số duy nhất. `3/4` là một chỗ, không phải hai con số đứng cạnh
nhau.

Một chuyện nữa, nhỏ mà về sau đắt: cái bị bẻ ra là **đơn vị bạn đã chọn**.
Bài 1 đã dặn — con số nào cũng là "mấy *cái gì*". Nên `1/4` không phải một
lượng cố định trên đời. Có `1/4` sải dây, có `1/4` cái bánh chưng, có `1/4` lon
gạo, và về sau sẽ có cả `1/4` **của cả cái vườn**. Cùng viết `1/4`, khác cái
đơn vị thì khác lượng.
::::

::::code{#hoi-may-hai-cau}
Byte muốn tự tay hỏi máy hai câu về đúng hai luống ban nãy:

- **Luống rau muống**: thước `1/4` sải, đặt vừa 3 lần → `3/4` sải.
- **Luống hành**: thước `1/5` sải, đặt vừa 3 lần → `3/5` sải.

Hai chỗ trống là hai câu hỏi. Hai câu này được chọn để cho ra hai câu trả lời
**khác nhau**: gõ cứng `False` vào cả hai thì câu 2 sai, gõ cứng `True` thì câu
1 sai. Chỉ hai câu hỏi viết thật mới qua được cả hai.

```python title=starter
# Câu 1: hai luống cùng "ba miếng" — chúng có bằng nhau không?
print(___)

# Câu 2: 3/4 sải có đúng là BA LẦN cái thước 1/4 sải không?
print(___)
```

```python title=solution
# Câu 1: hai luống cùng "ba miếng" — chúng có bằng nhau không?
print(3/4 == 3/5)

# Câu 2: 3/4 sải có đúng là BA LẦN cái thước 1/4 sải không?
print(3/4 == 1/4 + 1/4 + 1/4)
```

```python title=test
# Ba assert này chốt lại đúng ba điều bài vừa dạy. Chúng không chấm chữ bạn
# gõ — chúng canh cho chính bài học: nếu một ngày cái máy chạy bài đổi cách
# tính, cổng sẽ đỏ lên chứ không dạy sai lặng lẽ.
assert 3/4 != 3/5, "cùng ba miếng, nhưng hai cỡ thước khác nhau thì hai lượng khác nhau"
assert 3/4 == 1/4 + 1/4 + 1/4, "3/4 đúng là ba lần cái thước 1/4 — cả bài nằm ở dòng này"
assert 3/4 > 3/5, "miếng cỡ 1/4 to hơn miếng cỡ 1/5, nên ba miếng bên trái nhiều đất hơn"
```

:::hints
- kind: attention
  body: Ngay phía trên mỗi chỗ trống có một câu hỏi viết bằng tiếng Việt. Việc của bạn là viết lại đúng câu hỏi ấy cho máy nghe, chứ không phải tự trả lời hộ nó.
- kind: strategy
  body: "Cả hai câu đều là câu hỏi bằng — dấu so sánh bằng của Realm 0, hai dấu bằng viết liền nhau. Câu 1 đặt hai luống ở hai vế. Câu 2 đặt luống ở vế trái, còn vế phải phải viết ra ba cái thước 1/4 cộng lại."
- kind: one-line
  body: "Chỗ trống thứ nhất là `3/4 == 3/5`, chỗ trống thứ hai là `3/4 == 1/4 + 1/4 + 1/4`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh bằng (`==`) viết bằng phân số — gõ thẳng `True` hay `False` thì không hỏi máy điều gì cả
  requireAst:
  # `==` min 2 vì có hai câu hỏi, và khung chưa có dấu `==` nào — luật này
  # chặn đúng cái đáp án gõ cứng hai chữ True/False. `/` min 2 buộc câu hỏi
  # phải viết bằng phân số, không phải bằng một con số đã tính sẵn.
  - kind: uses-operator, target: ==, min: 2
  - kind: uses-operator, target: /, min: 2
- tier: output
  match: regex
  expect: ^False\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một câu không, một câu có. Ba miếng của bạn với ba miếng của An không cùng cỡ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`3/4` nằm giữa `0` và `1` — nó là chỗ thứ ba trong ba chỗ mới mọc ra giữa hai
vạch cũ.

Nhưng để ý cái thước trong tay bạn: nó không biết dừng. Đặt xong miếng thứ tư
là vừa hết một sải, và không có gì chặn tay bạn đặt tiếp miếng thứ năm, thứ
sáu, thứ bảy — đất vườn vẫn còn đó.

Bảy lần cái thước `1/4`, viết theo đúng luật vừa học thì thành `7/4`.

Thanh số có chỗ nào cho nó không? Hay phân số bị nhốt trong khoảng từ `0` tới
`1`, và `7/4` là một câu viết sai? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
