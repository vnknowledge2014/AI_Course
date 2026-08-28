---
id: toan.dai-so-va-ham-so.khi-dau-phai-quay-nguoc
title: Khi dấu phải quay ngược
summary: Nhân hay chia hai vế của một bất phương trình cho số âm thì phải đảo chiều dấu — vì nhân với số âm là lật cả trục số, còn cộng trừ chỉ là dời chỗ.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.inequality-flip]
requires: [math.multiply-by-negative, math.negative-number, math.compare-on-number-line, math.thanh-so, math.number-line-add, math.number-line-subtract, math.additive-inverse, math.order-of-operations, core.variable, core.reassign, core.print-variable, core.arithmetic, core.number-literal, core.boolean, ctrl.comparison]
concepts: [math.dao-chieu-dau, math.phep-giu-nghiem, math.lat-truc-so, math.kiem-nghiem]
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

::::byte{trigger=enter mood=dizzy pose=lean-in}
Mình làm y hệt cho cả hai vế mà nghiệm thật lại hoá sai. Cái cân hỏng à?
::::

::::explain{#can-khong-do-duoc-ben-nao-nang-hon}
Bài trước để lại một chỗ gãy. Bạn có `n ≥ 5`, một câu đúng. Nhân hai vế với
`−1` — phép mà bài 14 nói là hợp lệ, vì `−1` khác 0 — rồi giữ nguyên dấu:

```text
n ≥ 5   →   −n ≥ −5
```

Kiểm nghiệm bằng `n = 6`: `−6 ≥ −5`? Sai. Một nghiệm thật vừa bị biến thành
không phải nghiệm.

Chỗ gãy nằm ở chính hình ảnh cái cân, chứ không ở phép tính. Cái cân chỉ nói
được đúng một chuyện: **hai đĩa có thăng bằng hay không**. Nó chưa bao giờ nói
được "đĩa nào nặng hơn thì đứng ở đâu", và nó cũng không cân nổi một lượng âm —
bạn không đặt được `−6` cân bánh mì lên đĩa. Với dấu `=` thì cái cân đủ dùng.
Với dấu `≥` thì nó hết dùng được, vì `≥` không nói về thăng bằng, nó nói về
**thứ tự**.

Cái nói được về thứ tự là **trục số**: số nào đứng bên phải thì lớn hơn. Nên từ
đây, mọi phép biến đổi bất phương trình phải hỏi một câu duy nhất — *phép này
làm gì với thứ tự trên trục số?*
::::

::::example{#doi-cho-thi-giu-lat-thi-dao}
Lấy hai con số cụ thể để nhìn, `3` và `7`. Trên trục số, 3 đứng bên trái 7, nên
`3 < 7`.

**Cộng hoặc trừ cùng một số ở hai vế: DỜI.** Trừ 100 ở cả hai vế thì 3 thành
−97, 7 thành −93. Cả hai cùng trượt sang trái đúng 100 vạch. Khoảng cách giữa
chúng vẫn đúng 4 vạch, và đứa nào bên trái vẫn ở bên trái. `−97 < −93` — dấu giữ
nguyên. Điều này đúng với **mọi** số bạn cộng hay trừ, âm hay dương, vì cộng là
bước sang phải và trừ là bước sang trái: cả hai vế bước cùng nhau, cùng một
quãng.

**Nhân với số dương: KÉO GIÃN.** Nhân hai vế với 2 thì 3 thành 6, 7 thành 14.
Trục số bị kéo giãn ra từ mốc 0, mọi khoảng cách gấp đôi, nhưng bên trái vẫn ở
bên trái. `6 < 14` — dấu giữ nguyên.

**Nhân với số âm: LẬT.** Đây là chỗ T2.1 bài 23 đã dựng sẵn: nhân với số âm là
**lật cả trục số quanh mốc 0**. Nhân hai vế với `−2` thì 3 thành −6, 7 thành
−14. Xem chúng rơi vào đâu:

```text
  -16 -14 -12 -10  -8  -6  -4  -2   0   2   4   6   8
    ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼─▶
        ○               ○                 ●       ●
     7×(−2)          3×(−2)               3       7
```

Trước khi lật, 3 ở bên **trái** 7. Sau khi lật, ảnh của 3 (là −6) nằm bên
**phải** ảnh của 7 (là −14). Cả hai cùng bay sang phần âm, nhưng đứa ở xa mốc 0
hơn thì bay xa hơn, nên nó vượt qua đứa kia. Thứ tự đảo:

```text
3 < 7          →   3×(−2) > 7×(−2)
                   −6 > −14
```

Nên luật là:

> Nhân hoặc chia **hai vế** của một bất phương trình cho một số **âm** thì phải
> **đảo chiều dấu**: `<` thành `>`, `≤` thành `≥`, và ngược lại.
> Cộng hoặc trừ thì **không** đảo, dù số cộng vào là âm hay dương.
> Nhân hoặc chia cho số **dương** cũng **không** đảo.
> (Nhân hai vế với 0 vẫn hỏng, đúng như bài 14: với `≥` thì mọi thứ thành
> `0 ≥ 0` — câu nào cũng đúng; với `>` thì thành `0 > 0` — câu nào cũng sai.
> Hỏng theo hai kiểu ngược nhau, mà kiểu nào cũng mất sạch tập nghiệm.)

Thử lại chỗ gãy ở đầu bài: `n ≥ 5`, nhân hai vế với `−1`, **đảo** dấu → `−n ≤
−5`. Kiểm với `n = 6`: `−6 ≤ −5`? Đúng. Nghiệm thật vẫn còn là nghiệm.
::::

::::predict{#doan-bon-dong commitOnce}
Byte bắt máy chấm bốn câu, tất cả đều xuất phát từ đúng một cặp số.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
a = 3
b = 7
print(a < b)
print(a * -2 < b * -2)
print(a - 100 < b - 100)
print(a * 2 < b * 2)
```

:::opt{correct}
True, False, True, True
:::

:::opt
True, False, True, False
::why
Gần đúng ở chỗ bạn vừa nhìn thấy phép nhân bẻ được thứ tự — dòng thứ hai chứng
minh điều đó, và bạn đọc trúng nó. Quy tắc bạn rút ra — *nhân hai vế thì phải
coi chừng dấu* — là quy tắc rất đáng có.

Chỗ lệch là phạm vi của nó: thủ phạm không phải phép nhân, mà là **dấu âm**.
Nhân với 2 chỉ kéo giãn trục số ra từ mốc 0; 3 thành 6, 7 thành 14, và cái nào
bên trái vẫn ở bên trái. Chỉ khi số nhân âm thì trục mới bị lật, và lật mới đảo
thứ tự. Nhân với số dương thì `6 < 14` — dấu y nguyên.
::
:::

:::opt
True, True, True, True
::why
Gần đúng ở chỗ bạn đang dùng luật của cái cân: *làm y hệt cho cả hai vế thì
quan hệ giữa hai vế không đổi*. Với dấu `=` thì luật ấy đúng trăm phần trăm —
`3 = 3` nhân hai vế với −2 vẫn ra `−6 = −6`.

Chỗ lệch là dấu `<` không nói về **bằng nhau**, nó nói về **thứ tự**. Nhân với
−2 giữ được chuyện "bằng nhau", nhưng nó lật cả trục số, nên nó **không** giữ
được chuyện "bên nào đứng trước". 3 nhỏ hơn 7, mà −6 lại lớn hơn −14. Đó đúng
là chỗ cái cân hết kể được chuyện.
::
:::

:::opt
True, False, False, True
::why
Gần đúng ở chỗ bạn để ý rằng trừ 100 đẩy cả hai số sang phần âm — đúng thật,
chúng thành −97 và −93 — và bạn nhớ rằng vùng số âm là nơi thứ tự hay làm người
ta hụt chân.

Chỗ lệch là **cách** chúng sang phần âm. Trừ là bước sang trái: cả hai cùng
bước, cùng đúng 100 vạch, nên khoảng cách giữa chúng vẫn đúng 4 vạch và đứa nào
đi trước vẫn đi trước. Đó là **dời**, không phải **lật**. Chỉ phép nhân với số
âm mới hất cái ở xa mốc 0 nhất bay xa nhất và vượt qua cái kia.
::
:::
::::

::::explain{#hai-duong-di-phai-gap-nhau}
Có một cách kiểm luật đảo dấu mà không cần tin ai: **đi hai đường, xem có gặp
nhau không.**

Cuối buổi Byte hạ giá cho ế hàng. Mỗi ổ hạ giá làm lãi tụt đúng 4 000 đ, và
trước khi hạ, lãi đang là 60 000 đ. Byte muốn cuối buổi lãi vẫn còn **hơn**
20 000 đ. Hạ giá được nhiều nhất mấy ổ?

```text
60000 − 4000 × n > 20000
```

**Đường một — chỉ dùng cộng trừ, không đụng phép nhân chia nào.** Cộng
`4000 × n` vào hai vế, rồi bớt 20 000 ở hai vế. Cả hai đều là phép "dời", không
bao giờ đảo dấu:

```text
60000 − 4000×n > 20000
60000 > 20000 + 4000×n
40000 > 4000×n
```

Chia hai vế cho 4 000 — số **dương**, không đảo:

```text
10 > n     tức là     n < 10
```

**Đường hai — bớt 60 000 trước, rồi phải chia cho số âm.**

```text
60000 − 4000×n > 20000
−4000×n > −40000
```

Chia hai vế cho `−4000`. Số âm, nên **đảo** dấu:

```text
n < 10
```

Hai đường đi hoàn toàn khác nhau, cùng về một chỗ: `n < 10`. Đó không phải trùng
hợp — đó là bằng chứng. Đường một không dùng luật đảo dấu lần nào, nên nó là một
nhân chứng độc lập. Nếu luật đảo dấu sai thì hai đường đã phải cãi nhau.

Và nếu **quên** đảo? Đường hai sẽ cho `n > 10`. Kiểm nghiệm bằng một con số, như
bài 16 dạy: lấy `n = 12`, tức hạ giá 12 ổ. Lãi còn `60000 − 4000×12 = 12000`. Có
hơn 20 000 không? Không. Vậy `n > 10` sai — và một con số cụ thể là đủ để bắt nó.

```text
  số ổ hạ giá n │ lãi = 60000 − 4000×n │ lãi > 20000 │ n < 10 │ n > 10
  ──────────────┼──────────────────────┼─────────────┼────────┼───────
              8 │                28000 │        True │   True │  False
              9 │                24000 │        True │   True │  False
             10 │                20000 │       False │  False │  False
             11 │                16000 │       False │  False │   True
             12 │                12000 │       False │  False │   True
```

Cột `n < 10` khớp cột `lãi > 20000` ở mọi dòng. Cột `n > 10` lệch ở bốn trên
năm dòng. Để ý dòng `n = 10`: ở đúng cái mốc thì **cả hai** cột đều `False`, nên
riêng dòng ấy không phân biệt được luật nào đúng. Ngoài nó ra thì **số nào cũng
bắt được lỗi** — `n = 9` sát ngay mốc cũng bắt được, đúng như "bốn trên năm dòng"
vừa nói. Chỗ duy nhất phải tránh khi kiểm là **chính cái mốc**, chứ không phải
mấy số gần nó.
::::

::::code{#hai-luat-tranh-nhau}
Dựng ba dòng của cái bảng trên, để chính con số phân xử giữa `n < 10` và
`n > 10`.

Ba giá trị `n` được chọn để kẹp cái mốc: một dưới mốc, một **đúng** mốc, một
trên mốc. Với mỗi giá trị, tính lãi còn lại rồi hỏi lãi có hơn 20 000 không.

```python title=starter
n = 9
lai_9 = ___
con_hon_9 = ___

n = 10
lai_10 = ___
con_hon_10 = ___

n = 12
lai_12 = ___
con_hon_12 = ___

print(lai_9, con_hon_9)
print(lai_10, con_hon_10)
print(lai_12, con_hon_12)
```

```python title=solution
n = 9
lai_9 = 60000 - 4000 * n
con_hon_9 = lai_9 > 20000

n = 10
lai_10 = 60000 - 4000 * n
con_hon_10 = lai_10 > 20000

n = 12
lai_12 = 60000 - 4000 * n
con_hon_12 = lai_12 > 20000

print(lai_9, con_hon_9)
print(lai_10, con_hon_10)
print(lai_12, con_hon_12)
```

```python title=test
# Hai câu `!=` đứng trước, và chúng là hai câu quan trọng nhất của bài: chúng
# đối chiếu kết quả thật với luật KHÔNG đảo dấu (`n > 10`) và bắt nó sai. Xếp
# chúng xuống dưới thì một câu `==` trượt trước, và cái bẫy này không bao giờ
# sập.
assert con_hon_9 != (9 > 10), "hạ giá 9 ổ vẫn còn lãi 24000, tức vẫn hơn 20000 — mà luật quên-đảo-dấu `n > 10` lại bảo 9 không thoả. Luật ấy sai ngay ở đây."
assert con_hon_12 != (12 > 10), "hạ giá 12 ổ chỉ còn 12000, KHÔNG hơn 20000 — mà luật quên-đảo-dấu `n > 10` lại bảo 12 thoả. Sai lần thứ hai, ở phía bên kia mốc."
assert con_hon_9 == (9 < 10), "luật đảo dấu cho `n < 10`; ở n = 9 nó phải khớp với kết quả tính thật"
assert con_hon_10 == (10 < 10), "ở đúng mốc n = 10 thì lãi bằng chằn chặn 20000, chưa HƠN — nên cả hai vế đều False"
assert con_hon_12 == (12 < 10), "ở n = 12 luật đảo dấu vẫn khớp — nó khớp ở cả ba lần thử, không phải may"
assert lai_9 == 24000, "hạ giá 9 ổ: 60000 trừ đi 4000 chín lần, còn 24000"
assert lai_10 == 20000, "hạ giá 10 ổ: lãi tụt đúng về 20000 — đây là cái mốc"
assert lai_12 == 12000, "hạ giá 12 ổ: lãi chỉ còn 12000"
```

:::hints
- kind: attention
  body: Mỗi dòng cần hai việc tách rời. Việc thứ nhất là tính lãi còn lại — nhìn công thức trong bảng ở phần trên, cột thứ hai. Việc thứ hai là so lãi ấy với 20 000 bằng dấu của câu hỏi "còn HƠN 20 000 không". Và để ý ba dòng `n = 9`, `n = 10`, `n = 12` đứng trên: hai ô dưới mỗi dòng dùng đúng cái `n` của dòng ấy.
- kind: strategy
  body: "Ô `lai_...` là một phép tính ra số: bắt đầu từ 60 000 rồi bớt đi phần đã tụt, mà phần đã tụt là 4 000 nhân số ổ hạ giá. Ô `con_hon_...` là một câu so sánh ra đúng/sai: lấy chính cái tên `lai_...` vừa đặt ở dòng trên rồi so với 20 000. Ba dòng dùng chung một khuôn, viết khuôn ra là xong cả ba. Đừng nhẩm sẵn con số rồi gõ vào — cái bài đang hỏi là bạn dựng được khuôn hay không."
- kind: one-line
  body: "Ô lãi là `60000 - 4000 * n`; ô so sánh là `lai_9 > 20000` (rồi `lai_10 > 20000`, `lai_12 > 20000`)."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: ô lãi phải là một phép tính dựng trên `n` (60 000 bớt đi 4 000 nhân số ổ), và ô so sánh phải là một câu so thật với 20 000 — gõ sẵn con số hay gõ thẳng True/False thì máy không tính và không so gì cả
  requireAst:
  # Ba ô lãi, mỗi ô đọc `n` một lần. Khung khởi đầu không đọc `n` lần nào (ba
  # dòng `n = 9/10/12` là gán, không phải đọc), nên luật này chặn đúng đáp án
  # chép cứng ba con số lãi.
  - kind: uses-name, target: n, min: 3
  # Phần lãi đã tụt là một phép NHÂN (4000 lần số ổ) đem TRỪ khỏi 60000. Thiếu
  # một trong hai nghĩa là người viết đã tự tính hộ máy.
  - kind: uses-operator, target: *, min: 3
  - kind: uses-operator, target: -, min: 3
  # Ba ô so sánh phải dùng đúng dấu CHẶT `>` — câu hỏi là "còn HƠN 20 000
  # không", và cả bài xoay quanh chuyện dấu nào đứng ở đâu.
  - kind: uses-operator, target: >, min: 3
  forbidAst:
  # Lưới thứ hai. Ba ô so sánh nhận giá trị đúng/sai, nên đáp án chép cứng ở đó
  # là chính chữ `True`/`False`; hai ô lãi 24000 và 12000 thì chép cứng bằng
  # con số. Lời giải thật dựng cả năm thứ ấy từ 60000, 4000, 20000 và `n`, nên
  # không chứa nguyên văn cái nào — luật không cản ai làm thật.
  - kind: has-literal, target: True
  - kind: has-literal, target: False
  - kind: has-literal, target: 24000
  - kind: has-literal, target: 12000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^24000 True\n20000 False\n12000 False\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai đường đi khác hẳn nhau mà về cùng một chỗ. Giờ mình tin cái luật ấy rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Câu trả lời của bạn bây giờ là cả một **khoảng**: `n < 10`, tô lên trục số thành
một tia chạy về bên trái với mút rỗng ở vạch 10. Đẹp và gọn.

Nhưng để ý cái bạn vừa tô nó lên. Trục số chỉ có **một hàng số** — mỗi chỗ trên
đó ghi được đúng một con số, là giá trị của `n`. Còn cái bảng ở bài 5 thì có tận
**hai** cột: một cột ghi *điền gì*, một cột ghi *ra gì*.

Nhìn lại bảng vừa xong mà xem:

```text
  số ổ hạ giá n │ lãi = 60000 − 4000×n
  ──────────────┼─────────────────────
              9 │                24000
             10 │                20000
             12 │                12000
```

Dòng đầu tiên mang **hai** con số — `9` và `24000` — và chúng đi thành một cặp,
tách ra là mất nghĩa. Trên cái trục số một hàng của bạn, `9` ghi được, `24000`
ghi được, nhưng **cặp** `9` với `24000` thì ghi vào đâu?

Và nếu ghi được cả ba dòng lên cùng một chỗ, ba cái dấu ấy sẽ nằm như thế nào?

Bài sau dựng chỗ để ghi.
::::

::::checkpoint{mastery=0.8}
::::
