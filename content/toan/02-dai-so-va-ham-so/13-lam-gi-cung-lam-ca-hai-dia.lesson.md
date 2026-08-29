---
id: toan.dai-so-va-ham-so.lam-gi-cung-lam-ca-hai-dia
title: Làm gì cũng phải làm cả hai đĩa
summary: Bớt cùng một lượng ở CẢ HAI đĩa cho ra một câu trông khác hẳn — nhưng nó đúng ở đúng những số cũ và sai ở đúng những số cũ.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.equation-add-both-sides]
requires: [math.equation, math.additive-inverse, math.subtraction-as-distance, math.like-units, math.multiplication, core.variable, core.reassign, core.arithmetic, core.number-literal, core.print-variable, core.output, core.boolean, ctrl.comparison]
concepts: [math.phuong-trinh-tuong-duong, math.giu-nguyen-nghiem, math.hai-dia-can]
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
Bốc 8 ổ khỏi một đĩa thì cân đổ nghiêng. Bốc khỏi cả hai thì mình được gì?
::::

::::explain{#cau-hoi-con-treo}
Bài trước dựng cái cân: một phương trình là hai đĩa **đang thăng bằng**, và ô
trống là gói hàng chưa ai biết nặng bao nhiêu. Câu hỏi để lại là câu hỏi của
người đang cầm cái cân trong tay: *được phép động vào hai cái đĩa theo kiểu
nào?*

Cảnh của bài này là buổi sáng ở xe bánh mì. Byte xếp hàng vào **hai khay giống
hệt nhau**, mỗi khay đúng `n` ổ — chưa đếm, nên chưa ai biết `n` là mấy. Cạnh
đó còn **8 ổ nguội** để rời trong túi. Đếm cả xe thì được **20 ổ**.

Viết ra:

`n + n + 8 = 20`

Hai khay giống hệt nhau nên `n + n` gộp lại thành `2n` — đó là bài 9, gộp thứ
cùng loại. Câu ngắn lại:

`2n + 8 = 20`

Đặt lên cân:

```text
    ┌───────────────┐       ┌───────────────┐
    │   n + n + 8   │  ═══  │      20       │
    └───────────────┘       └───────────────┘
        đĩa trái                đĩa phải
```

Byte muốn biết một khay mấy ổ. Nhìn cái cân thì thấy ngay chỗ vướng: đĩa trái
có **8 ổ lẻ** đứng chen vào, không dính gì tới khay. Bốc 8 ổ ấy đi thì đĩa trái
sạch, chỉ còn hai khay.

Nhưng bốc xong thì đĩa trái nhẹ đi 8 ổ, còn đĩa phải vẫn nguyên 20. Cân đổ
nghiêng:

```text
    ┌───────────────┐       ┌───────────────┐
    │     n + n     │   ?   │      20       │
    └───────────────┘       └───────────────┘
        đĩa trái                đĩa phải
```

Cân lệch nghĩa là câu vừa viết ra **không còn hỏi cùng một chuyện nữa**. Byte
không đơn giản hoá cái gì cả — Byte vừa đổi sang một câu KHÁC. Câu mới ấy tự nó
chẳng sai: điền 10 vào thì `2n = 20` đúng hẳn hoi. Nó chỉ không còn là câu của
Byte, vì nghiệm của nó khác nghiệm câu gốc.

Cách chữa chỉ có một: bốc 8 ổ ở **cả hai** đĩa.
::::

::::example{#bot-tam-o-ca-hai-dia}
Bốc 8 ổ khỏi đĩa trái, rồi bốc tiếp 8 ổ khỏi đĩa phải. Đĩa phải còn `20 − 8`,
tức 12 ổ:

```text
    ┌───────────────┐       ┌───────────────┐
    │     n + n     │  ═══  │      12       │
    └───────────────┘       └───────────────┘
        đĩa trái                đĩa phải
```

`2n = 12`

Đây là chỗ dễ đọc nhầm nhất của cả bài, nên nói thẳng ra: **`2n = 12` không
phải là `2n + 8 = 20` viết gọn.** Hai câu ấy khác nhau thật — chữ trên hai đĩa
đã khác, số trên đĩa phải đã khác. Chúng là hai lời khẳng định riêng biệt.

Thứ được giữ nguyên không phải hình dạng câu. Thứ được giữ nguyên là **danh
sách những số làm nó đúng**.

Bài 11 đã cho cách nhìn danh sách ấy: điền một số vào, câu hoá ra đúng hoặc
sai. Vậy thì đem cả hai câu ra hỏi cùng một loạt số, rồi so hai cột kết quả:

```python title=readonly
n = 6
print(2 * n + 8 == 20, 2 * n == 12)

n = 5
print(2 * n + 8 == 20, 2 * n == 12)

n = 9
print(2 * n + 8 == 20, 2 * n == 12)
```

Máy in ra:

```text
True True
False False
False False
```

Ba lần điền, ba lần hai cột trùng khít. Ở `n = 6` cả hai cùng đúng; ở `n = 5`
và `n = 9` cả hai cùng sai. Cân không nói cho bạn `n` bằng mấy — cân chỉ hứa
một điều: **câu mới trả lời giống hệt câu cũ ở mọi số bạn đem tới thử.**
::::

::::predict{#doan-bot-mot-dia commitOnce}
Giờ thử đúng cái việc hỏng: bốc 8 ổ khỏi **một** đĩa thôi. Byte lấy `n = 6` —
con số vừa làm câu gốc đúng — rồi hỏi máy hai câu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 6
print(2 * n + 8 == 20)
print(2 * n == 20)
```

:::opt{correct}
True rồi False
:::

:::opt
True rồi True
::why
Gần đúng ở chỗ bạn đang dùng đúng cái luật vừa học: *bớt đi rồi thì câu vẫn trả
lời như cũ*. Luật ấy có thật, và nó chính là thứ cả bài này đi dựng.

Chỗ lệch nằm ở phạm vi của nó. Luật ấy chỉ nói về việc bớt ở **cả hai** đĩa.
Dòng thứ hai chỉ bớt ở đĩa trái: đĩa trái nhẹ đi 8 ổ, đĩa phải vẫn 20. Cân đổ
nghiêng, và một cái cân đang nghiêng thì lời khẳng định "hai bên bằng nhau" là
lời sai. Với `n = 6` thì đĩa trái còn 12 ổ, đĩa phải 20 ổ — máy trả `False`.
::
:::

:::opt
False rồi False
::why
Gần đúng ở chỗ bạn thấy dòng thứ hai hỏng — nó hỏng thật, và nhận ra điều đó là
nửa bài học.

Chỗ lệch: quy tắc *một bước sai thì cả chuỗi hỏng* đúng khi bạn đang đi một
chuỗi biến đổi trên giấy, vì dòng sau viết ra từ dòng trước. Nhưng ở đây máy
đang chạy hai lệnh `print` **độc lập**. Mỗi lệnh tự đọc lại `n`, tự tính lại
hai vế từ đầu. Dòng thứ hai có sai cách mấy cũng không với tay ngược lên bôi
bẩn dòng thứ nhất được. Dòng thứ nhất vẫn là `2 × 6 + 8 == 20`, tức `20 == 20`.
::
:::

:::opt
Máy báo lỗi ở dòng thứ hai, vì `2 * n == 20` là một câu chưa giải
::why
Gần đúng ở chỗ bạn nhìn `2n = 20` ra một **bài chưa xong** — trong vở toán thì
đúng là thế, viết ra dòng ấy nghĩa là còn phải làm tiếp.

Chỗ lệch là dấu `==` không phải một lời sai bảo. Bài 10 đã đặt tên cho nó: đó
là một **câu hỏi Đ/S**. Máy không đi tìm `n`; máy tính vế trái ra một số, tính
vế phải ra một số, rồi trả lời "hai số này có bằng nhau không". Với `n = 6` thì
vế trái là 12, vế phải là 20 — câu trả lời là `False`, và `False` là một câu
trả lời đàng hoàng, không phải một lỗi.
::
:::
::::

::::explain{#dat-ten-cho-luat}
Đặt tên cho thứ vừa thấy, để mang đi được:

> Thêm — hoặc bớt — **cùng một lượng** vào **cả hai vế** cho ra một phương
> trình **mới**, và phương trình mới ấy có **đúng cùng một tập nghiệm** với
> phương trình cũ.

Vì sao chắc chắn đúng, chứ không phải "thử vài số thấy trùng"? Lời hứa ấy có
hai nửa, và cân cho thấy cả hai:

- **Không mất nghiệm.** Lấy một số làm câu cũ đúng — hai đĩa đang nặng bằng
  nhau. Bớt 8 ở cả hai thì hai đĩa cùng nhẹ đi đúng 8. Bằng nhau trừ đi cùng
  một lượng thì vẫn bằng nhau, nên số ấy làm câu mới cũng đúng.
- **Không thêm nghiệm lạ.** Lấy một số làm câu cũ **sai** — hai đĩa lệch, một
  bên nặng hơn bên kia đúng một khoảng nào đó. Bớt 8 ở cả hai thì cái khoảng
  lệch ấy không đổi tí nào (đó là bài 15 của T2.1: hiệu hai số là khoảng cách
  giữa chúng, dời cả hai cùng một quãng thì khoảng cách giữ nguyên). Lệch vẫn
  hoàn lệch, nên số ấy làm câu mới cũng sai.

Hai nửa cộng lại thành một câu: câu mới và câu cũ **luôn trả lời giống nhau**,
ở mọi số, chứ không riêng vài số bạn kịp thử.

Và để ý thứ bạn vừa đổi được. Chín bài đầu track chỉ đổi được **hình dạng một
biểu thức** — mở ngoặc, rút cái chung, gộp cái cùng loại. Giờ bạn đổi được cả
một **lời khẳng định** mà không làm nó nói khác đi. Đó là cách thoát khỏi ngõ
cụt của bài 11: không phải thử từng số nữa, mà kéo chính câu hỏi về một hình
dạng dễ đọc hơn.
::::

::::code{#hai-cau-mot-danh-sach}
Byte muốn máy làm chứng cho lời hứa ấy — không phải ở một số, mà ở ba số cùng
lúc.

Câu gốc là `2n + 8 = 20`. Câu mới, sau khi bốc 8 ổ khỏi cả hai đĩa, là
`2n = 12`.

Điền ba chỗ trống để mỗi lần thử có đủ **hai** câu trả lời Đ/S đem so với nhau.

Bài chấm bằng cả ba lần thử, và ba con số được chọn để không giống nhau: `6`
làm cả hai câu đúng, `5` và `9` làm cả hai câu sai. Gõ cứng `True` vào mọi chỗ
thì hai lần sau trượt; gõ cứng `False` thì lần đầu trượt. Chỉ một câu hỏi Đ/S
viết thật, có `n` ở trong, mới qua được cả ba.

```python title=starter
n = 6
goc_6 = 2 * n + 8 == 20
moi_6 = ___

n = 5
goc_5 = 2 * n + 8 == 20
moi_5 = ___

n = 9
goc_9 = 2 * n + 8 == 20
moi_9 = ___

print(goc_6, moi_6)
print(goc_5, moi_5)
print(goc_9, moi_9)
```

```python title=solution
n = 6
goc_6 = 2 * n + 8 == 20
moi_6 = 2 * n == 12

n = 5
goc_5 = 2 * n + 8 == 20
moi_5 = 2 * n == 12

n = 9
goc_9 = 2 * n + 8 == 20
moi_9 = 2 * n == 12

print(goc_6, moi_6)
print(goc_5, moi_5)
print(goc_9, moi_9)
```

```python title=test
# Câu `!=` đứng trước: nó canh cái bẫy "gõ cứng một giá trị Đ/S vào cả ba chỗ".
# Xếp nó sau ba câu `==` thì không bao giờ chạy tới, và cái bẫy không bao giờ
# sập.
assert moi_6 != moi_5, "n = 6 và n = 5 phải cho hai kết quả Đ/S khác nhau — nếu giống nhau thì câu mới của bạn không hỏi gì về n cả"
assert moi_6 == goc_6, "n = 6 làm câu gốc đúng, nên nó phải làm câu mới đúng — bớt 8 ở hai đĩa không được làm mất nghiệm"
assert moi_5 == goc_5, "n = 5 làm câu gốc sai, nên nó phải làm câu mới sai — bớt 8 ở hai đĩa không được thêm nghiệm lạ"
assert moi_9 == goc_9, "n = 9 cũng vậy: hai câu phải trả lời giống nhau ở MỌI số đem thử, không riêng số nào"
assert moi_6, "riêng n = 6 thì câu mới phải ra True — hai khay đúng 12 ổ"
```

:::hints
- kind: attention
  body: Nhìn dòng ngay trên mỗi chỗ trống. Nó viết câu GỐC ra thành một câu hỏi Đ/S. Chỗ trống cần đúng một câu như thế, chỉ khác ở chỗ nó là câu SAU KHI đã bốc 8 ổ khỏi cả hai đĩa. Cả ba chỗ trống điền giống hệt nhau.
- kind: strategy
  body: Sau khi bốc 8 ổ khỏi cả hai đĩa, đĩa trái còn hai khay, đĩa phải còn 20 trừ 8. Viết lại nguyên câu ấy thành một câu hỏi Đ/S — vẫn phải có `n` ở trong, vì không có `n` thì câu chẳng hỏi gì về số ổ trong khay nữa.
- kind: one-line
  body: "Cả ba chỗ trống đều là `2 * n == 12`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi Đ/S có `n` ở trong — `2 * n == 12`; gõ thẳng `True` hay `False`, hay chép lại tên `goc_6`, thì máy không kiểm gì cả, bạn tự trả lời hộ nó rồi
  requireAst:
  # Khung khởi đầu đã có ba dấu `==` (ba câu gốc). Ba chỗ trống phải thêm ba
  # dấu nữa. `moi_6 = goc_6` hay `moi_6 = True` đều không có dấu `==` nào, nên
  # luật này phân biệt được đúng với sai.
  - kind: uses-operator, target: ==, min: 6
  # Có dấu `==` thôi chưa đủ: `12 == 12` cũng là một câu hỏi Đ/S mà không nhắc
  # tới `n`, và nó ra `True` ở cả ba lần thử. Khung khởi đầu đọc `n` ba lần
  # (trong ba câu gốc), nên `min: 6` buộc ba chỗ trống mỗi chỗ phải đọc `n`.
  - kind: uses-name, target: n, min: 6
  # Con số 12 là thứ cả bước biến đổi này sinh ra — 20 bớt đi 8. Lời giải thật
  # có nó; mọi kiểu điền bừa thì không.
  # KHÔNG đòi nguyên văn con số 12.
  #
  # Bài viết ở trên: "Đĩa phải còn `20 − 8`, tức 12 ổ" — nên `2 * n == 20 - 8`
  # cũng là chép đúng lời bài, thậm chí còn cho thấy phép bớt ở đĩa phải rõ hơn.
  # Đòi nguyên văn 12 là đánh trượt đúng một trong hai dạng mà chính bài bày ra.
  # Chỗ chấm thật nằm ở `==` sáu lần, `n` sáu lần và regex output.
  forbidAst:
  # Lưới thứ hai. Lời giải thật không chứa `True` hay `False` viết tay, nên hai
  # luật này chỉ cản đúng người gõ cứng câu trả lời.
  - kind: has-literal, target: True
  - kind: has-literal, target: False
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^True True\nFalse False\nFalse False\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cột trùng khít cả ba lần. Câu mình vừa viết ra là câu khác, mà vẫn hỏi đúng
chuyện cũ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte đang cầm `2n = 12`. So với `2n + 8 = 20` thì gọn hơn hẳn — 8 ổ lẻ đã biến
khỏi đĩa trái.

Nhưng nhìn kỹ: **ô trống vẫn chưa đứng một mình.** Trên đĩa trái còn hai khay,
không phải một. Câu chưa có dạng `n = một số`, nên chưa đọc ra được số ổ.

Cộng và trừ đã hết việc ở đây: bớt thêm nữa thì đĩa trái mất luôn cả khay.

Vậy còn phép nào khác? Ngoài đời thì việc phải làm rõ mồn một — hai khay nặng
12 ổ, thì **chia đôi** ra, mỗi khay 6 ổ. Nhưng "chia đôi" là một thao tác mới
trên cái cân, và bài này chưa cấp phép cho nó.

- Chia đôi **cả hai** đĩa — cân có còn thăng bằng không?
- Còn nhân cả hai đĩa lên **gấp ba** thì sao?
- Và nếu nhân cả hai đĩa với **0** — hai đĩa cùng rỗng, chắc chắn thăng bằng —
  thì có phải bạn vừa được một phương trình đúng mà chẳng tốn công gì không?

Bài sau trả lời cả ba, và câu thứ ba là câu đáng ngờ nhất.
::::

::::checkpoint{mastery=0.8}
::::
