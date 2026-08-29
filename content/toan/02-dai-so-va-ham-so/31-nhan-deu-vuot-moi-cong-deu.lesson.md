---
id: toan.dai-so-va-ham-so.nhan-deu-vuot-moi-cong-deu
title: Nhân đều rồi sẽ vượt mọi cộng đều
summary: Máy nhân đôi mỗi bước cuối cùng luôn vượt máy cộng đều mỗi bước — dốc tới đâu cũng chỉ dời được ngày bị vượt, không huỷ được nó.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 31
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.exponential-beats-linear]
requires: [math.exponential-function, math.function-notation, math.slope, math.exponent, math.multiplication, math.compare-on-number-line, math.subtraction-as-distance, math.negative-number, core.function-def, core.function-parameter, core.function-return, core.function-call, core.variable, core.arithmetic, core.print-variable]
concepts: [math.muc-them-moi-buoc, math.cho-hai-duong-cat-nhau, math.khoang-cach-doi-dau]
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
Nhà bên đang bỏ xa hũ men của mình bảy nghìn phần. Chờ thêm mấy giờ nữa?
::::

::::explain{#cau-hoi-con-treo}
Bài trước để lại một câu hỏi có số hẳn hoi, và bây giờ là lúc trả lời nó.

Hai cái làm bột, đứng cách nhau một bức tường.

- **Hũ men của Byte.** Mỗi giờ men **nhân đôi**. Bắt đầu từ 1 phần, sau `n` giờ
  hũ có `2ⁿ` phần — đúng cái máy `men` mà bài trước đã dựng.
- **Máy trộn nhà bên cạnh.** Nó chẳng nhân đôi gì cả, chỉ làm ra đều đặn `1000`
  phần bột mỗi giờ, không hơn không kém. Sau `n` giờ, nhà bên có `1000 × n`
  phần. Một hàm bậc nhất, độ dốc 1000.

Bảy giờ trôi qua. Hũ của Byte có `128` phần. Nhà bên có `7000` phần. Byte thua
đậm — thua hơn năm mươi lần.

Và đây là chỗ đáng nghi. Bài trước gọi phép nhân đôi là tăng trưởng nhân và nói
nó mạnh. Nhìn bảy giờ đầu thì chẳng thấy mạnh chỗ nào. Một trong hai câu sau
phải sai:

- *Máy nhân đôi mạnh hơn máy cộng đều* — nhưng nó đang thua.
- *Máy nào đang dẫn thì cứ dẫn mãi* — nhưng câu này chưa ai chứng minh.

Câu thứ hai mới là câu sai. Và chỗ nó sai không nằm ở con số nào cả; nó nằm ở
chỗ ta đang nhìn **sai cột**.
::::

::::example{#nhin-cot-muc-them}
Đừng nhìn cột *đang có bao nhiêu*. Nhìn cột **mỗi giờ thêm bao nhiêu** — đúng
cái cột mà bài 26 gọi là mức đổi của một bước.

Nhà bên dễ trước: mỗi giờ thêm đúng `1000` phần. Cột ấy là một hàng số giống
hệt nhau, kéo dài mãi mãi.

Hũ men thì khác. Từ giờ `n` sang giờ `n + 1`, men đi từ `2ⁿ` lên `2ⁿ⁺¹`.
Nhân đôi nghĩa là **thêm đúng bằng chỗ đang có**. Nên mức thêm của Byte ở giờ
`n` chính là `2ⁿ` — và cái mức thêm ấy tự nó cũng nhân đôi mỗi giờ.

```text
giờ           8      9     10     11     12     13     14
men thêm    256    512   1024   2048   4096   8192  16384
bột thêm   1000   1000   1000   1000   1000   1000   1000
```

Đọc hàng "men thêm" từ trái sang: `256`, `512`, rồi `1024`. Ngay tại giờ **10**,
mức thêm của men lần đầu vượt mức thêm của bột. Từ giây phút ấy trở đi, mỗi giờ
Byte **lấy lại** được một ít khoảng cách: mỗi giờ men thêm `2ⁿ` còn bột chỉ
thêm `1000`, nên phần gỡ được là `2ⁿ − 1000`. Số ấy chưa nhân đôi ngay, nhưng
`1000` thì đứng yên còn `2ⁿ` thì không, nên càng về sau nó càng sát với nhân
đôi.

Bảng khoảng cách xác nhận đúng điều đó. Cột cuối là `men − bột`: âm nghĩa là
Byte còn thua.

| giờ | men (`2ⁿ`) | bột (`1000n`) | men − bột |
|---|---|---|---|
| 9 | 512 | 9000 | −8488 |
| 10 | 1024 | 10000 | **−8976** ← thua đậm nhất |
| 11 | 2048 | 11000 | −8952 |
| 12 | 4096 | 12000 | −7904 |
| 13 | 8192 | 13000 | −4808 |
| 14 | 16384 | 14000 | **+2384** ← vượt |

Chỗ thua đậm nhất rơi đúng vào giờ 10 — đúng cái giờ mà mức thêm của men vượt
mức thêm của bột. Không phải trùng hợp: trước giờ đó Byte tụt lại thêm mỗi giờ,
sau giờ đó Byte gỡ lại mỗi giờ. Giờ 10 là chỗ quay đầu.

Dựng hai cái máy rồi hỏi thẳng chúng ở hai giờ quanh chỗ cắt nhau:

```python title=readonly
def men(n):
    return 2 ** n          # men nhân đôi mỗi giờ

def bot(n):
    return 1000 * n        # máy trộn làm đều 1000 phần mỗi giờ

print(men(13) - bot(13))
print(men(14) - bot(14))
```

Máy in ra:

```text
-4808
2384
```

Một dấu trừ, rồi một dấu cộng. Giữa hai giờ ấy, hai đường cắt nhau.
::::

::::predict{#doan-doc-gap-tram commitOnce}
Nhà bên thay máy trộn. Máy mới làm `100000` phần mỗi giờ — **dốc gấp 100 lần**
máy cũ. Đường thẳng của nhà bên dựng đứng hẳn lên.

Byte không đổi gì cả: men vẫn nhân đôi mỗi giờ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def men(n):
    return 2 ** n

def bot_manh(n):
    return 100000 * n      # máy mới, dốc gấp 100 lần máy cũ

print(men(21) - bot_manh(21))
print(men(22) - bot_manh(22))
```

:::opt{correct}
`-2848` rồi `1994304` — giờ 21 men còn thua một chút xíu, giờ 22 đã vượt hẳn
:::

:::opt
Hai số âm rất to. Máy mới dốc gấp 100 lần thì men không đuổi kịp nữa.
::why
Gần đúng ở chỗ bạn dùng một quy tắc thật và mạnh: *đường nào dốc hơn thì cuối
cùng đường ấy dẫn*. Với **hai đường thẳng** thì quy tắc ấy đúng tuyệt đối —
đường dốc 100000 bỏ xa đường dốc 1000 mãi mãi, không ngoại lệ. Bạn không nhớ
nhầm luật.

Ranh giới nằm ở chỗ luật ấy đem so **hai con số cố định** với nhau. Nhà bên có
một mức thêm cố định là `100000`. Byte thì không có mức thêm cố định nào để đem
ra so: mức thêm của Byte là `2ⁿ`, và nó tự lớn lên. Đặt một con số đứng yên cạnh
một con số đang nhân đôi thì con số đứng yên chỉ thắng được một lúc. `2¹⁷` đã là
`131072`, hơn `100000` rồi — từ giờ 17 Byte bắt đầu gỡ.
::
:::

:::opt
Cả hai đều dương. Bài vừa nói máy nhân đôi luôn vượt máy cộng đều mà.
::why
Gần đúng ở chỗ bạn đọc trúng kết luận của bài, và kết luận ấy đúng: máy nhân đôi
**sẽ** vượt, không máy cộng đều nào thoát.

Ranh giới nằm ở hai chữ *cuối cùng*. Câu "sẽ vượt" hứa rằng **có tồn tại** một
giờ mà men dẫn, chứ không hứa giờ ấy là giờ nào. Với máy cũ, giờ ấy là 14. Máy
mới dốc gấp 100 lần thì giờ ấy lùi ra sau. Ở giờ 21, `2²¹ = 2097152` còn
`100000 × 21 = 2100000` — men thua đúng `2848` phần. Sát sạt, nhưng vẫn là thua.
Một câu nói về *cuối cùng* không trả lời hộ bạn câu hỏi *bây giờ*.
::
:::

:::opt
Cả hai đều âm cỡ hai triệu — dốc gấp 100 lần thì phải tới quãng giờ 1400 men mới
vượt.
::why
Gần đúng ở chỗ bạn dùng đúng lối nghĩ tỉ lệ mà T2.1 đã dựng: *gấp 100 lần thứ
này thì gấp 100 lần thứ kia*. Và với **cột bột** thì nó đúng thật — ở bất kỳ giờ
nào, bột của máy mới đúng bằng 100 lần bột của máy cũ.

Ranh giới: tỉ lệ ấy áp được lên cột bột, nhưng không áp được lên **giờ bị vượt**.
Muốn đuổi kịp một con số lớn gấp 100 lần, men không cần gấp 100 lần số giờ — nó
chỉ cần thêm đủ số lần nhân đôi để tự lớn lên 100 lần. Bảy lần nhân đôi đã là
128 lần rồi. Nên ngày bị vượt lùi từ giờ 14 sang giờ 22: thêm 8 giờ, không phải
thêm gấp trăm.
::
:::
::::

::::explain{#phat-bieu}
Đặt tên cho thứ vừa thấy, để mang đi được:

> Một máy **nhân** với cùng một số lớn hơn 1 ở mỗi bước thì **cuối cùng luôn
> vượt** mọi máy **cộng** thêm cùng một lượng ở mỗi bước. Đường thẳng dốc tới
> đâu cũng chỉ dời được ngày bị vượt ra xa hơn, không huỷ được nó.

Và đây là lý do, gói trong ba câu — nhớ lý do thì không phải nhớ bảng:

1. Máy cộng đều có mức thêm **đứng yên** ở một con số. Gọi nó là `a`.
2. Máy nhân đôi có mức thêm **tự nhân đôi**. Nó bắt đầu bé hơn `a`, nhưng nhân
   đôi mãi thì có lúc phải vượt `a` — vì `a` chỉ là một con số, còn `2ⁿ` thì
   vượt lên trên mọi con số. (Nó không ghé qua mọi con số: nó nhảy 1, 2, 4, 8,
   bỏ qua 3, bỏ qua 5. Nhưng nhảy kiểu ấy thì con số nào rồi cũng bị bỏ lại
   phía dưới.)
3. Từ giờ đó trở đi, mỗi giờ máy nhân đôi gỡ lại được một ít, và số gỡ được còn
   lớn lên gấp bội. Khoảng cách phải đóng thì hữu hạn; số gỡ được thì không có
   trần. Nên nó đóng — rồi mở ra về phía ngược lại và không đóng lại nữa.

Điều `a` quyết định là **khi nào**, không phải **có hay không**. Bài trước hỏi
hai câu; giờ trả lời được cả hai:

| độ dốc `a` của máy trộn | giờ men vượt |
|---|---|
| 1000 | 14 |
| 100000 — gấp 100 lần | 22 |
| 1000000 — gấp 1000 lần | 25 |

*Sau 20 giờ thì ai hơn ai?* Với máy trộn thường, men đã dẫn từ giờ 14, và tới
giờ 20 men có `1048576` phần còn nhà bên mới `20000` — dẫn hơn năm mươi lần
(52,4). Ở giờ thứ bảy men từng **thua** cũng cỡ ấy (54,7 lần). Hai con số không
bằng nhau đâu, nhưng cùng một cỡ, và cái đáng nhìn là chiều đã lật hẳn.

*Máy trộn khoẻ hơn nữa thì có đổi câu trả lời không?* Không. Nhìn cái bảng trên:
độ dốc nhân lên **một nghìn lần** — từ 1000 lên một triệu — mà ngày bị vượt chỉ
lùi từ giờ 14 sang giờ 25, thêm mười một giờ. Cột trái nhân lên gấp bội, cột
phải bò thêm được vài bước. Đó chính là câu "chỉ đổi cái lúc nào" viết bằng số.
::::

::::code{#do-cho-quay-dau}
Cả bài này xoay quanh **giờ 10** — chỗ quay đầu. Giờ bắt máy tự chỉ ra nó.

Bạn dựng bốn cái máy: hai máy đo *đang có bao nhiêu*, và hai máy đo *mỗi giờ
thêm bao nhiêu*. Máy `men_them` đã viết sẵn làm mẫu — nó hỏi máy `men` hai lần
rồi lấy hiệu. Máy `bot_them` phải làm **đúng kiểu ấy**, đừng gõ thẳng con số
`1000` vào, vì cái đáng chứng minh chính là *bột thêm bao nhiêu cũng chẳng đổi*.

Bài chấm bằng **năm con số khác nhau**, và chúng được chọn để kể trọn câu
chuyện: hai mức thêm ôm lấy mốc 1000, rồi ba khoảng cách trong đó cái ở giữa là
cái tệ nhất. Gõ cứng `512` vào mọi chỗ thì bốn chỗ kia sai. Chỉ bốn cái máy viết
thật mới qua được cả năm.

```python title=starter
def men(n):
    return ___                     # men nhân đôi mỗi giờ, bắt đầu từ 1 phần

def bot(n):
    return ___                     # máy trộn làm đều 1000 phần mỗi giờ

# MỨC THÊM khi đi từ giờ n sang giờ n + 1.
def men_them(n):
    return men(n + 1) - men(n)

def bot_them(n):
    return ___

# Giờ 9 mức thêm của men còn thua 1000; giờ 10 đã vượt.
them_men_9 = men_them(9)
them_men_10 = ___

# Khoảng cách men − bột, đo ngay hai bên giờ 10.
cach_9 = men(9) - bot(9)
cach_10 = ___
cach_11 = ___

print(them_men_9)
print(them_men_10)
print(cach_9)
print(cach_10)
print(cach_11)
```

```python title=solution
def men(n):
    return 2 ** n                  # men nhân đôi mỗi giờ, bắt đầu từ 1 phần

def bot(n):
    return 1000 * n                # máy trộn làm đều 1000 phần mỗi giờ

# MỨC THÊM khi đi từ giờ n sang giờ n + 1.
def men_them(n):
    return men(n + 1) - men(n)

def bot_them(n):
    return bot(n + 1) - bot(n)

# Giờ 9 mức thêm của men còn thua 1000; giờ 10 đã vượt.
them_men_9 = men_them(9)
them_men_10 = men_them(10)

# Khoảng cách men − bột, đo ngay hai bên giờ 10.
cach_9 = men(9) - bot(9)
cach_10 = men(10) - bot(10)
cach_11 = men(11) - bot(11)

print(them_men_9)
print(them_men_10)
print(cach_9)
print(cach_10)
print(cach_11)
```

```python title=test
# Câu `!=` đứng TRƯỚC. Nó canh đúng cái bẫy của bài: nếu mức thêm của men hoá ra
# đứng yên thì máy bạn viết là máy cộng đều, và mọi câu `==` phía dưới có đúng
# cũng chẳng chứng minh được cái chỗ quay đầu nào. Xếp nó sau thì không bao giờ
# chạy tới, và bẫy không bao giờ sập.
assert them_men_9 != them_men_10, "mức thêm của men không đứng yên — nó tự nhân đôi mỗi giờ, nên hai giờ liền nhau phải cho hai con số khác nhau"
assert bot_them(9) == bot_them(10), "mức thêm của bột thì đứng yên thật: giờ nào cũng đúng bấy nhiêu, đó là cái làm nó thành máy cộng đều"
assert them_men_9 == 512, "từ giờ 9 sang giờ 10 men đi từ 512 lên 1024, tức thêm đúng 512 phần — đúng bằng chỗ đang có"
assert them_men_10 == 1024, "giờ sau men thêm 1024 phần, lần đầu vượt mức 1000 của bột"
assert bot_them(10) == 1000, "bột thêm 1000 phần ở giờ 10, y như ở mọi giờ khác"
assert cach_9 == -8488, "giờ 9: men 512 phần, bột 9000 phần, men thua 8488"
assert cach_10 == -8976, "giờ 10: men thua 8976 — đây là chỗ thua đậm nhất cả bảng"
assert cach_11 == -8952, "giờ 11: men vẫn thua, nhưng đã đỡ hơn giờ 10 một chút"
assert them_men_9 < bot_them(9), "ở giờ 9 men thêm ít hơn bột, nên khoảng cách còn nới rộng ra"
assert them_men_10 > bot_them(10), "ở giờ 10 men thêm nhiều hơn bột, nên từ đây khoảng cách bắt đầu thu lại"
assert cach_10 < cach_9 and cach_10 < cach_11, "chỗ thua đậm nhất rơi đúng vào giờ 10 — đúng cái giờ mức thêm của men vượt mức thêm của bột; hai chuyện đó là một"
```

:::hints
- kind: attention
  body: Đọc chú thích ở cuối hai dòng đầu. Một dòng nói men NHÂN ĐÔI mỗi giờ, dòng kia nói máy trộn THÊM 1000 phần mỗi giờ. Hai chữ khác nhau đó phải thành hai phép tính khác nhau. Còn chỗ trống trong `bot_them` thì nhìn thẳng lên dòng `men_them` ngay phía trên — nó đã viết sẵn khuôn bạn cần.
- kind: strategy
  body: Nhân đôi n lần từ 1 phần thì được 2 luỹ thừa n; trong Python luỹ thừa viết bằng hai dấu sao. Cộng đều thì là độ dốc nhân số giờ. Ba chỗ trống cuối đều là bản sao của dòng ngay trên chúng, chỉ đổi con số giờ — đừng gõ thẳng kết quả, vì chính mấy con số ấy là thứ bạn đang nhờ máy tìm hộ.
- kind: one-line
  body: "Năm chỗ lần lượt là `2 ** n`, `1000 * n`, `bot(n + 1) - bot(n)`, `men_them(10)`, rồi `men(10) - bot(10)` và `men(11) - bot(11)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: máy men phải là một luỹ thừa của 2 viết theo `n`, máy bột phải là độ dốc nhân `n`, và `bot_them` phải HỎI máy `bot` chứ đừng gõ thẳng 1000 — gõ cứng con số thì cái đáng chứng minh không được chứng minh ở đâu cả
  requireAst:
  # Khung khởi đầu không có `**` hay `*` nào, nên hai luật này một mình đã chặn
  # đáp án chép cứng hai cái máy đầu.
  - kind: uses-operator, target: **, min: 1
  - kind: uses-operator, target: *, min: 1
  # Hai cái máy phải viết theo `n`, không phải theo một con số. Khung khởi đầu
  # đọc `n` đúng 2 lần, cả hai nằm trong thân `men_them`; lời giải đọc 6 lần.
  - kind: uses-name, target: n, min: 5
  # `bot_them` phải HỎI máy `bot` chứ không gõ thẳng 1000. Khung khởi đầu gọi
  # `bot` đúng 1 lần (trong `cach_9`); lời giải gọi 5 lần.
  - kind: uses-call, target: bot, min: 5
  # Ba chỗ trống cuối phải chạy máy thật. Khung khởi đầu gọi `men` 3 lần và
  # `men_them` 1 lần; lời giải gọi 5 và 2.
  - kind: uses-call, target: men, min: 5
  - kind: uses-call, target: men_them, min: 2
  forbidAst:
  # `uses-operator` đếm trên cả file nên không chặn nổi đáp án chép cứng ĐÚNG
  # MỘT chỗ. Năm luật dưới chặn từng kết quả một. Chúng không cản cách viết hợp
  # lệ nào: `2 ** n`, `1000 * n`, `bot(n + 1) - bot(n)`, `men(10) - bot(10)` đều
  # không chứa nguyên văn con số nào trong danh sách.
  - kind: has-literal, target: 512
  - kind: has-literal, target: 1024
  - kind: has-literal, target: 8488
  - kind: has-literal, target: 8976
  - kind: has-literal, target: 8952
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^512\n1024\n-8488\n-8976\n-8952\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đúng giờ 10! Chỗ mình thua đậm nhất lại chính là chỗ mình bắt đầu gỡ. Mình cứ
tưởng hai chuyện đó khác nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Suốt bài này bạn chạy máy men theo đúng một chiều: bỏ **số giờ** vào, nó nhả ra
**số phần men**. Bỏ 13 vào ra 8192. Bỏ 22 vào ra hơn bốn triệu. Chiều nào cũng
trơn tru.

Nhưng chiều tối, đứng cạnh hũ men, câu hỏi trong đầu Byte đi ngược lại:

> *Sáng mai cần **1000 phần** men. Vậy phải ủ từ mấy giờ?*

Bảng nằm ngay đó: giờ 9 được `512` phần, giờ 10 được `1024` phần. Byte dò ngón
tay dọc **cột kết quả**, thấy `1024` thì gióng sang **cột giờ**, đọc ra `10`.

Chỉ có điều dò ngón tay thì **không phải một cái máy**. Máy `men` nhận số giờ;
nó không nhận số phần. Byte vừa làm một việc mà chưa cái máy nào trong tủ làm
được: đưa **đầu ra** vào, đòi lại **đầu vào**.

Vậy có cái máy nào chạy được chiều đó không — và nếu có thì dựng nó thế nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
