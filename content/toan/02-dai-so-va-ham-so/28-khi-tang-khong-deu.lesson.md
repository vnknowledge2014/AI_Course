---
id: toan.dai-so-va-ham-so.khi-tang-khong-deu
title: Khi mỗi bước tăng một kiểu
summary: Mảnh sân vuông cạnh n mét có cột thứ ba không chịu đứng im — phải thêm cột thứ tư mới thấy con số cố định, và chính chỗ đó làm đồ thị cong.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.quadratic-function]
practices: [math.slope, math.value-table, math.expand-brackets, math.multiplication, math.multiply-distributive, math.multiply-by-negative]
requires: [math.linear-function, math.slope, math.value-table, math.function-notation, math.graph-of-expression, math.expand-brackets, math.multiplication, math.multiply-distributive, math.multiply-by-negative, math.negative-number, math.compare-on-number-line, core.function-def, core.function-call, core.function-parameter, core.function-argument, core.function-return, core.variable, core.assignment, core.arithmetic, core.print-variable, core.number-literal, core.output]
concepts: [math.chenh-lech-cua-chenh-lech, math.mien-dau-vao, math.xe-banh-mi]
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
Mảnh sân vuông. Nới cạnh thêm một mét là lại tốn nhiều gạch hơn lần trước.
::::

::::explain{#cot-thu-ba-khong-chiu-dung-im}
Bài trước khép lại bằng một lời khoe rất to: hai con số `a` và `b` tả trọn
**mọi** đường thẳng. Nhưng câu ấy chỉ có nghĩa nếu mọi cái máy đều vẽ ra đường
thẳng — và bài trước cũng chỉ luôn cái máy đáng ngờ đầu tiên.

Mảnh sân hình vuông sau nhà Byte, cạnh `n` mét, cần lát gạch:

> `s(n) = n × n`

Đem đúng công cụ của bài 26 ra soi nó: lập bảng, rồi thêm **cột thứ ba** — cột
ghi *kết quả đổi bao nhiêu so với dòng ngay trên*.

```text
  n   │ s(n) │ cột 3: bước từ dòng trên
──────┼──────┼──────────────────────────
    0 │    0 │
    1 │    1 │                       +1
    2 │    4 │                       +3
    3 │    9 │                       +5
    4 │   16 │                       +7
    5 │   25 │                       +9
    6 │   36 │                      +11
```

Cột thứ ba **không** đứng im. Nới cạnh từ 1 lên 2 mét thì tốn thêm 3 mét vuông
gạch; nới từ 5 lên 6 mét thì tốn thêm tận 11.

Theo đúng câu mà bài 26 đã cảnh báo, chuyện này kéo theo hai hệ quả cùng lúc:
không có **độ dốc** nào để ghi (vì độ dốc là *một* con số, mà đây thì mỗi chỗ
một khác), và cái hình cũng **mất quyền thẳng** — đoạn này dốc hơn đoạn kia thì
chỗ nối phải gãy.

Vậy `s` không phải hàm bậc nhất. Bộ đồ nghề `a` và `b` hết dùng được ở đây.
::::

::::example{#cot-thu-tu}
Nhưng đừng bỏ cột thứ ba vội. Đọc lại nó một mình, tách khỏi bảng:

```text
  +1, +3, +5, +7, +9, +11
```

Hàng này thì **có** đứng im theo kiểu của nó: mỗi số hơn số trước đúng 2.

Bài 26 dạy một động tác — lấy dòng dưới trừ dòng trên — và không có gì cấm làm
động tác ấy lần thứ hai, lần này trên chính cột thứ ba. Thêm một **cột thứ
tư**:

```text
  n   │ s(n) │ cột 3: bước từ dòng trên │ cột 4: bước của cột 3
──────┼──────┼──────────────────────────┼──────────────────────
    0 │    0 │                          │
    1 │    1 │                       +1 │
    2 │    4 │                       +3 │                    +2
    3 │    9 │                       +5 │                    +2
    4 │   16 │                       +7 │                    +2
    5 │   25 │                       +9 │                    +2
    6 │   36 │                      +11 │                    +2
```

Con số đứng im vẫn có thật. Nó chỉ nằm sâu hơn một tầng.

**Vì sao lại là 2?** Không phải phép màu — nhìn thẳng vào mảnh sân. Nới cạnh từ
4 lên 5 mét nghĩa là lát thêm gạch vào đâu?

```text
      cạnh 4 mét                cạnh 5 mét
       # # # #                   # # # # o
       # # # #                   # # # # o
       # # # #        ==>        # # # # o
       # # # #                   # # # # o
                                 o o o o @
```

Thêm **một cột** 4 ô, **một hàng** 4 ô, và **một ô góc**: `4 + 4 + 1 = 9`. Đúng
con số trong cột thứ ba.

Nới từ 5 lên 6 thì thêm một cột 5 ô, một hàng 5 ô, một ô góc: `5 + 5 + 1 = 11`.

Viết chung cho mọi `n`, phần lát thêm luôn là `n + n + 1`, tức `2n + 1` — đây
chính là luật phân phối của bài 7 đọc trên một hình vuông. Và `2n + 1` là một
biểu thức **bậc nhất**: `n` thêm 1 thì nó thêm đúng 2. Con số `2` ở cột thứ tư
chính là **hai cái cạnh** mà mỗi lần nới đều phải lát thêm.

Thử cột thứ tư trên cái máy cũ để thấy nó không thiên vị ai. Với
`thu(n) = 15000 × n` của xe bánh mì:

```text
  n   │ thu(n)  │ cột 3: bước từ dòng trên │ cột 4: bước của cột 3
──────┼─────────┼──────────────────────────┼──────────────────────
    0 │       0 │                          │
    1 │   15000 │                   +15000 │
    2 │   30000 │                   +15000 │                    +0
    3 │   45000 │                   +15000 │                    +0
    4 │   60000 │                   +15000 │                    +0
```

Cột thứ ba đứng im thì cột thứ tư toàn số 0. Một đường thẳng chính là trường
hợp mà tầng thứ hai chẳng còn gì để nói.
::::

::::explain{#vi-sao-do-thi-cong}
Đem bảng của mảnh sân chấm lên mặt phẳng như bài 23. Cho mỗi mét vuông một ô,
xếp thành thanh ngang cho dễ so:

```text
  n = 1  |#  1
  n = 2  |####  4
  n = 3  |#########  9
  n = 4  |################  16
  n = 5  |#########################  25
  n = 6  |####################################  36
  n = 7  |#################################################  49
```

Đầu các thanh không nằm trên một đường thẳng. Khoảng hụt giữa hai thanh liền
nhau mỗi lần một rộng hơn: 3, rồi 5, rồi 7…

Đó đúng là định nghĩa của "cong". Bài 26 nói: hình thẳng **vì** hai đoạn nối
liên tiếp có cùng "sang phải bao nhiêu, lên bao nhiêu". Ở đây sang phải vẫn 1 ô
mỗi lần, nhưng lên thì mỗi lần một cao hơn — và cao hơn đúng 2 đơn vị. Mỗi chỗ
nối là một chỗ gãy nhẹ, gãy đều đặn về cùng một phía. Gãy đều đặn như thế thì
mắt người đọc ra là một đường **uốn**.

Đặt tên cho cái vừa thấy, để mang đi được:

> Một máy nhân chính đầu vào với đầu vào — như `s(n) = n × n` — gọi là **hàm
> bậc hai**. Dấu hiệu nhận ra nó trên bảng: cột thứ ba **không** đứng im, nhưng
> cột thứ tư thì **có**. Trên hình, nó là một đường **cong**.

Ba loại máy, cùng một câu hỏi "mỗi bước đổi thế nào":

```text
  máy đứng yên  :  ngay cột thứ HAI đã đứng im    ( s(n) = 7 )
  máy bậc nhất  :  cột thứ BA đứng im             ( thu(n) = 15000n )
  máy bậc hai   :  cột thứ TƯ đứng im             ( s(n) = n x n )
```
::::

::::predict{#doan-ba-dong commitOnce}
Byte gõ cái máy sân vào Python đúng như đã viết trên giấy, rồi hỏi nó ba câu.

Hai câu đầu là hai ô liền nhau trong cột thứ ba. Câu thứ ba thì Byte cố tình bỏ
vào một số **âm** để xem máy nói gì.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def san(n):
    return n * n

print(san(4) - san(3))
print(san(5) - san(4))
print(san(-3))
```

:::opt{correct}
`7`, rồi `9`, rồi `9`
:::

:::opt
`7`, rồi `7`, rồi `9`
::why
Chỗ đúng trong suy nghĩ của bạn là một luật lớn, và bạn đang dùng nó rất chuẩn:
*đo cột thứ ba ở một chỗ là biết nó ở mọi chỗ*. Bài 26 dựng đúng câu ấy, và với
`thu(n) = 15000 × n` nó đúng tuyệt đối — đo ở `n = 1` hay ở `n = 9` cũng ra
15 000.

Ranh giới nằm ở chỗ luật ấy chỉ nói về máy **bậc nhất**, tức những máy mà cột
thứ ba đứng im. Cái làm nó đứng im là phép nhân với một con số **đã cố định
sẵn** (15 000). Còn ở đây đầu vào nhân với **chính nó**, nên thứ nhân vào cũng
lớn lên theo. Bảng phía trên nói thẳng điều đó: +1, +3, +5, +7, +9, +11 —
không ô nào lặp lại ô trước. Bước từ 4 sang 5 là `2 × 4 + 1 = 9`.
::
:::

:::opt
`7`, rồi `9`, rồi `-9`
::why
Chỗ đúng: bạn nhớ rất chắc rằng số âm dính vào phép nhân thì kết quả lật sang
bên kia số 0. Bài 23 của track trước dựng đúng hình ấy — nhân với một số âm là
**lật cả thanh số** một vòng.

Ranh giới là **lật mấy lần**. `(-3) × (-3)` có hai thừa số âm, tức lật hai lần:
lật một lần thì sang trái, lật thêm lần nữa thì quay về bên phải. Nên nó ra
`9`, không phải `-9`. Luật "âm nhân ra âm" chỉ đúng khi có **đúng một** thừa số
âm.
::
:::

:::opt
`7`, rồi `9`, rồi máy báo lỗi — không có mảnh sân nào cạnh `-3` mét
::why
Chỗ đúng ở đây là một điều rất đáng giữ, và phần lớn người học không nghĩ tới
nó: bạn đang đối chiếu câu trả lời của máy **với đời thật**. Không có mảnh sân
cạnh `-3` mét, và một đáp án vô nghĩa ngoài đời thì đáng ngờ thật. Thói quen ấy
sẽ cứu bạn nhiều lần.

Ranh giới: cái máy và mảnh sân **không phải một thứ**. `san` chỉ biết nhận một
số rồi nhân nó với chính nó — nó chưa bao giờ nghe nói tới sân, tới mét, tới
viên gạch. Nên nó nhận `-3` và trả về `9` một cách bình thản. Phần "cạnh sân
không âm được" là điều **bạn** biết, không phải điều máy biết. Mục ngay sau đây
nói kỹ chỗ này.
::
:::
::::

::::explain{#may-khac-manh-san}
Đáp án `9` cho `san(-3)` đáng dừng lại một chút, vì nó tách đôi hai thứ suốt
nãy giờ bị dính làm một.

- **Cái máy** `s(n) = n × n` nhận **mọi** số. Số âm, số 0, số lẻ — bỏ vào là
  nhân được, vì nhân thì lúc nào cũng nhân được. `s(-3) = 9`, `s(0) = 0`.
- **Mảnh sân** thì không. Chưa ai đo được một mảnh sân cạnh `-3` mét.

Nói cho gọn: **mảnh sân chỉ là một lần dùng cái máy ấy**, trên những số mà đời
thật chịu nhận. Cái máy sống rộng hơn cái sân.

Bảng đầy đủ của cái **máy** chạy được cả sang trái mốc 0, chỗ mà mảnh sân không
theo được:

```text
  n:          -3   -2   -1    0    1    2    3
  s(n):        9    4    1    0    1    4    9
```

Giữ lấy chỗ tách đôi này. Vài bài nữa bạn sẽ hỏi cái máy một câu ngược — *rộng
9 mét vuông thì cạnh mấy mét* — và lúc đó việc `s` nhận cả `-3` lẫn `3` sẽ
thành chuyện lớn.
::::

::::code{#do-cot-thu-tu}
Cho máy tự đo lấy cột thứ ba và cột thứ tư của nó.

Bạn điền năm chỗ trống: dòng đầu là cái máy, ba dòng giữa là ba ô liền nhau của
cột thứ ba, dòng cuối là một ô của cột thứ tư.

Bài chấm bằng cả bốn con số, và chúng khác nhau từng đôi một. Gõ cứng `9` vào
mọi chỗ thì màn hình ra `0, 9, 9, 9` — sai ở dòng 1, 3 và 4. Gõ cứng `2` thì ra
`0, 2, 2, 2` — sai ở dòng 1, 2 và 3. Con số nào cũng vấp, chỉ vấp ở chỗ khác
nhau, nên không có một con số nào gõ cứng mà qua được cả bốn dòng. Chỉ cái máy
viết thật mới qua được.

```python title=starter
def san(n):
    return ___                     # sân vuông cạnh n mét thì rộng bao nhiêu?

# Cột thứ ba, ba ô liền nhau: 3 sang 4, 4 sang 5, 5 sang 6.
buoc_3_4 = san(4) - san(3)
buoc_4_5 = ___
buoc_5_6 = ___

# Cột thứ tư: hai ô liền nhau của cột thứ ba chênh nhau bao nhiêu?
buoc_cua_buoc = ___

print(buoc_3_4)
print(buoc_4_5)
print(buoc_5_6)
print(buoc_cua_buoc)
```

```python title=solution
def san(n):
    return n * n                   # sân vuông cạnh n mét thì rộng bao nhiêu?

# Cột thứ ba, ba ô liền nhau: 3 sang 4, 4 sang 5, 5 sang 6.
buoc_3_4 = san(4) - san(3)
buoc_4_5 = san(5) - san(4)
buoc_5_6 = san(6) - san(5)

# Cột thứ tư: hai ô liền nhau của cột thứ ba chênh nhau bao nhiêu?
buoc_cua_buoc = buoc_4_5 - buoc_3_4

print(buoc_3_4)
print(buoc_4_5)
print(buoc_5_6)
print(buoc_cua_buoc)
```

```python title=test
# Câu != đứng trước: nếu ba ô của cột thứ ba hoá ra bằng nhau thì cái máy bạn
# viết là máy bậc nhất, và mọi câu == phía dưới có đúng cũng chẳng nói lên gì.
assert buoc_3_4 != buoc_4_5, "ba ô của cột thứ ba phải KHÁC nhau — bằng nhau nghĩa là cột thứ ba đứng im, tức máy bạn viết là bậc nhất chứ không phải bậc hai"
assert buoc_4_5 != buoc_5_6, "ô 4 sang 5 và ô 5 sang 6 cũng phải khác nhau, vì mỗi lần nới cạnh lại lát thêm nhiều gạch hơn lần trước"
assert buoc_3_4 == 7, "nới cạnh từ 3 lên 4 thì lát thêm một cột 3 ô, một hàng 3 ô và một ô góc: 3 + 3 + 1 = 7"
assert buoc_4_5 == 9, "nới cạnh từ 4 lên 5 thì lát thêm 4 + 4 + 1 = 9 mét vuông"
assert buoc_5_6 == 11, "nới cạnh từ 5 lên 6 thì lát thêm 5 + 5 + 1 = 11 mét vuông"
assert buoc_cua_buoc == 2, "cột thứ tư đứng im ở 2 — đúng bằng hai cái cạnh phải lát thêm mỗi lần nới"
assert buoc_5_6 - buoc_4_5 == buoc_cua_buoc, "con số 2 ấy không phải may mắn ở một chỗ: mọi cặp ô liền nhau của cột thứ ba đều chênh nhau đúng bấy nhiêu"
assert san(-3) == 9, "cái máy nhận cả số âm: (-3) x (-3) lật thanh số hai lần nên quay về 9 — mảnh sân mới là thứ không có cạnh âm, không phải cái máy"
```

:::hints
- kind: attention
  body: Chỗ trống đầu tiên nằm sau `return`, và cái tên duy nhất mà hàm ấy nhận được là `n`. Ba chỗ trống tiếp theo hãy nhìn dòng `buoc_3_4` ngay phía trên chúng — nó đã viết sẵn đúng khuôn bạn cần chép lại với hai số khác. Chỗ trống cuối cùng thì không gọi `san` nữa: nguyên liệu của nó là hai cái tên vừa tính xong.
- kind: strategy
  body: Sân vuông thì hai cạnh bằng nhau, nên diện tích là cạnh nhân với chính cạnh — viết bằng cái tên `n`, đừng viết một con số cụ thể, vì hàm này phải chạy được với mọi cạnh. Ba ô của cột thứ ba đều theo một khuôn: dòng dưới trừ dòng trên. Ô của cột thứ tư cũng đúng khuôn ấy, chỉ khác là nó trừ hai ô của cột thứ ba cho nhau chứ không trừ hai diện tích.
- kind: one-line
  body: "Lần lượt là `n * n` (hoặc `n ** 2`), `san(5) - san(4)`, `san(6) - san(5)` và `buoc_4_5 - buoc_3_4`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hàm phải tính diện tích TỪ `n` (nhân `n` với chính nó, hay nâng `n` lên luỹ thừa hai — cách nào cũng được), ba ô của cột thứ ba phải GỌI hàm ấy, và ô của cột thứ tư phải trừ hai ô cột ba cho nhau — gõ thẳng con số vào là bạn tính hộ máy rồi, cái bảng mà bài này đi tìm không xuất hiện ở đâu cả
  requireAst:
  # Hàm phải ĐỌC `n`. Khung khởi đầu không đọc `n` lần nào, nên chỗ trống thứ
  # nhất là chỗ duy nhất sinh ra nó.
  #
  # Chỉ đòi ĐỌC `n`, không đòi dấu nhân. Bản cũ đòi `uses-operator *` và
  # `uses-name n min: 2`, tức là ngầm bắt phải viết `n * n` — và như thế đánh
  # trượt `return n ** 2`, một lời giải đúng và rất tự nhiên: người học có luỹ
  # thừa từ T2.1 bài 24, còn bài 30 ngay sau đây dùng chính `2 ** n`. Đánh
  # trượt một đáp án đúng thì tệ hơn cho người học nhiều so với bỏ lọt một đáp
  # án sai: họ làm đúng mà máy nói sai, và không có cách nào biết vì sao.
  - kind: uses-name, target: n, min: 1
  # Khung khởi đầu gọi `san` đúng 2 lần. Lời giải gọi 6, vì hai chỗ trống giữa
  # mỗi chỗ cần hai lần gọi. Điền số vào đó thì con số này tụt xuống.
  - kind: uses-call, target: san, min: 6
  # Bốn phép trừ: ba ô cột ba và một ô cột bốn. Khung khởi đầu mới có 1.
  - kind: uses-operator, target: -, min: 4
  # Ô cột bốn phải ĐỌC LẠI hai ô cột ba vừa tính, chứ không gọi lại hàm hay gõ
  # số. Khung khởi đầu đọc mỗi tên đúng 1 lần (trong `print`).
  - kind: uses-name, target: buoc_3_4, min: 2
  - kind: uses-name, target: buoc_4_5, min: 2
  forbidAst:
  # Lưới thứ hai: bốn con số này là KẾT QUẢ. Lời giải thật không chứa nguyên
  # văn cái nào trong chúng, nên luật này không cản ai làm thật.
  - kind: has-literal, target: 7
  - kind: has-literal, target: 9
  - kind: has-literal, target: 11
  # KHÔNG cấm số 2 nữa. Nó từng ở đây để chặn ai gõ thẳng `buoc_cua_buoc = 2`,
  # nhưng hai luật `uses-name buoc_3_4 / buoc_4_5 min: 2` đã chặn đúng chuyện
  # ấy rồi (gõ số thì hai cái tên chỉ được đọc một lần). Nó thừa với cái đích
  # của mình, mà lại chặn `n ** 2` — một lời giải đúng.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^7\n9\n11\n2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
7, 9, 11 — cột thứ ba không chịu đứng im. Nhưng cột thứ tư thì im re: đúng 2.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đường cong của mảnh sân đi lên mãi: cạnh càng dài, sân càng rộng, không có chỗ
dừng. Muốn sân rộng nhất thì cứ chọn cạnh lớn nhất — chẳng có gì phải nghĩ.

Nhưng ở xe bánh mì thì có một đường cong kiểu khác, và nó khó chịu hơn nhiều.

Byte đang bán 15 nghìn một ổ, mỗi ổ lãi 5 nghìn, mỗi ngày hết 60 ổ. Byte tính
tăng giá. Tăng giá thì **lãi mỗi ổ cao hơn** — nghe là muốn tăng ngay. Nhưng
khách thấy đắt sẽ bớt mua, nên **số ổ bán được lại ít đi**. Hai thứ kéo ngược
nhau.

Tăng ít quá thì phí. Tăng nhiều quá thì mất khách. Nghĩa là ở đâu đó giữa
chừng, đường lãi phải có một **chỗ cao nhất** — rồi từ đó đi xuống.

Chỗ ấy nằm đâu? Và nếu vẽ cả đường cong ra giấy, trên hình có gì giúp bạn chỉ
tay vào nó mà không phải dò từng số một?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
