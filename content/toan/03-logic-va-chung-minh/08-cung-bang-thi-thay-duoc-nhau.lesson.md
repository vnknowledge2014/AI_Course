---
id: toan.logic-va-chung-minh.cung-bang-thi-thay-duoc-nhau
title: Cùng một bảng thì thay được cho nhau
summary: Hai câu có bảng chân lý trùng khít ở mọi dòng thì đặt câu nào vào chỗ nào cũng không làm đổi giá trị của câu bao ngoài — còn hai câu chỉ tình cờ cùng giá trị hôm nay thì không.
locale: vi
track: toan
module: logic-va-chung-minh
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.equivalence]
requires: [logic.tautology, logic.contradiction, logic.truth-table, logic.negation, logic.conjunction, logic.disjunction, logic.and, logic.or, logic.not, logic.parentheses, core.boolean, core.variable, core.list, core.list-append, core.print-variable, ctrl.for-each, ctrl.nested-loop]
concepts: [logic.tuong-duong, logic.trung-khit-bang, logic.thay-vao-cho-trong]
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
Hai câu viết khác chữ hẳn nhau, mà cột dựng ra khớp từng ô một. Mình đổi chỗ chúng được không?
::::

::::explain{#hai-cot-trung-khit}
Bài trước để lại một cột lạ.

Bạn dựng bảng cho câu **"KHÔNG PHẢI là Nam chưa nộp quỹ"** — câu lật giá trị của
Nam một lần rồi lật thêm một lần nữa. Cột ra được không toàn Đ, cũng không toàn
S, nên nó không phải hằng đúng và cũng không phải hằng sai. Nhưng nó khớp từng ô
với cột của một câu khác:

| Nam đã nộp quỹ | Nam chưa nộp quỹ | KHÔNG PHẢI là Nam chưa nộp quỹ |
|---|---|---|
| Đ | S | Đ |
| S | Đ | S |

Cột thứ nhất và cột thứ ba: ô nào cũng khớp ô nấy.

Gọi tên hai câu ấy cho gọn, vì cả bài sẽ nhắc tới chúng:

- **Câu A** — "Nam đã nộp quỹ".
- **Câu B** — "KHÔNG PHẢI là Nam chưa nộp quỹ".

Câu hỏi để lại: hai câu viết khác chữ hẳn nhau mà cùng một cột thì có thay được
cho nhau không?

Trước hết, đặt tên cho chuyện "cùng một cột":

> Hai câu là **tương đương logic** khi bảng chân lý của chúng **trùng khít** —
> ở **mọi** dòng của bảng, hai cột mang cùng một giá trị.

Chữ "mọi dòng" gánh cả sức nặng của định nghĩa. Trùng ở ba dòng trên bốn thì
chưa phải tương đương; đó chỉ là hai câu hay đi cùng nhau.
::::

::::explain{#vi-sao-thay-duoc}
Tương đương mới là chuyện hai cột. Thay được cho nhau lại là chuyện khác — nó
nói về những câu **dài hơn** có chứa câu A hay câu B bên trong.

Dựng thử. Lấy một câu bao ngoài, chừa một chỗ trống ở giữa:

> **"( … ) VÀ Lan đã nộp quỹ."**

Đặt câu A vào chỗ trống thì được *"Nam đã nộp quỹ VÀ Lan đã nộp quỹ"*. Đặt câu B
vào đúng chỗ ấy thì được *"KHÔNG PHẢI là Nam chưa nộp quỹ VÀ Lan đã nộp quỹ"*.
Hai câu bao ngoài này nói về hai người, nên bảng của chúng có bốn dòng.

| Nam đã nộp | Lan đã nộp | câu A | câu B | câu A **VÀ** Lan đã nộp | câu B **VÀ** Lan đã nộp |
|---|---|---|---|---|---|
| Đ | Đ | Đ | Đ | Đ | Đ |
| Đ | S | Đ | Đ | S | S |
| S | Đ | S | S | S | S |
| S | S | S | S | S | S |

Hai cột cuối trùng khít nhau. Và điều đáng nói là **vì sao** chúng phải trùng,
chứ không phải chuyện chúng tình cờ trùng lần này.

Bài 4 đã chốt một luật mà lúc ấy nghe như một sự mất mát: giá trị của câu ghép
chỉ do **giá trị Đ/S của hai vế** quyết định, không do nội dung, không do cách
viết. Chữ "và" nhìn vào hai ô Đ/S rồi trả lời, nó không đọc chữ.

Giờ luật ấy trả công. Câu A và câu B mang cùng giá trị ở mọi dòng. Chữ "và" chỉ
nhìn thấy giá trị. Nên ở mỗi dòng, nó nhận vào đúng cùng một cặp Đ/S và trả ra
đúng cùng một câu trả lời. Không có dòng nào để hai cột lệch nhau.

Và lập luận vừa rồi chẳng nhắc gì tới chữ "và" ngoài chuyện nó chỉ nhìn giá trị.
Nên nó chạy y hệt với "hoặc", với câu ngược, với bất kỳ câu bao ngoài nào dựng
bằng những chữ nối ấy:

> Hai câu tương đương thì ở **bất kỳ** chỗ nào một câu đang đứng, đặt câu kia
> vào cũng không làm đổi giá trị của câu bao ngoài.

Đó là chỗ chữ "tương đương" đáng giá. Bảng trùng khít là thứ bạn **kiểm** được;
thay được cho nhau ở mọi chỗ là thứ bạn **dùng** được.
::::

::::example{#cung-dung-hom-nay-thi-chua-du}
Chỗ dễ va nằm ở chữ "mọi dòng". Thử một câu thứ ba:

- **Câu C** — "Lan đã nộp quỹ".

Sáng nay Nam đã nộp và Lan cũng đã nộp. Câu A đúng, câu C đúng. Hai câu cùng giá
trị — thay được cho nhau chưa?

Chưa. Dựng cả bảng ra mới thấy:

| Nam đã nộp | Lan đã nộp | câu A | câu C |
|---|---|---|---|
| Đ | Đ | Đ | Đ |
| Đ | S | Đ | S |
| S | Đ | S | Đ |
| S | S | S | S |

Hai cột khớp ở dòng đầu và dòng cuối, lệch ở dòng hai và dòng ba. Trùng hai dòng
trên bốn thì không phải trùng khít, nên câu A và câu C **không** tương đương.
Sáng nay chúng cùng đúng là chuyện của sáng nay — đó là dòng đầu của bảng, một
dòng trong bốn.

Đem câu C thay vào đúng chỗ trống lúc nãy thì được *"Lan đã nộp quỹ VÀ Lan đã
nộp quỹ"*. Câu ấy nghe kỳ, nhưng nó đúng là thứ phép thay nhả ra, và bảng của nó
vẫn dựng được như mọi câu khác:

| Nam đã nộp | Lan đã nộp | câu A **VÀ** Lan đã nộp | câu C **VÀ** Lan đã nộp |
|---|---|---|---|
| Đ | Đ | Đ | Đ |
| Đ | S | S | S |
| S | Đ | S | **Đ** |
| S | S | S | S |

Hai cột lệch nhau, và chúng lệch ở đúng **một** dòng: dòng ba, buổi mà Nam chưa
nộp còn Lan đã nộp.

Một dòng là đủ. Muốn nói "hai câu này thay được cho nhau ở mọi chỗ", bạn phải
không tìm ra dòng lệch nào; tìm ra một dòng là câu ấy hỏng.

Để ý một chuyện nữa, vì nó ngăn một kết luận đi quá xa. Câu A và câu C lệch nhau
ở **hai** dòng (dòng hai và dòng ba), nhưng hai câu bao ngoài chỉ lệch ở **một**
dòng. Ở dòng hai, Lan chưa nộp, nên chữ "và" cho cả hai câu bao ngoài cùng S —
chỗ lệch bên trong bị nuốt mất. Vậy lệch bên trong không bắt buộc kéo theo lệch
bên ngoài ở đúng những dòng ấy. Điều bài này khẳng định chỉ là chiều kia, và nó
là chiều chắc chắn: **trùng khít bên trong thì không bao giờ lệch bên ngoài.**
::::

::::predict{#doan-hai-buoi commitOnce}
Byte đem ba câu ấy cho máy, chạy ở **hai buổi** khác nhau. Trong Python, "Nam đã
nộp quỹ" là giá trị Đ/S giữ trong cái tên `p`, "Lan đã nộp quỹ" giữ trong `q`.
Câu B viết bằng hai chữ `not` chồng lên nhau, đúng như câu tiếng Việt có hai lần
lật.

Buổi đầu: cả hai đã nộp. Buổi sau: Nam đã nộp, Lan chưa.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
p = True
q = True
print(not (not p))
print(q)

p = True
q = False
print(not (not p))
print(q)
```

:::opt{correct}
True, True, True, False
:::

:::opt
False, True, False, False
::why
Gần đúng ở chỗ bạn đọc trúng cả hai dòng về Lan, và ở chỗ bạn xử lý chữ `not`
đúng luật của bài 3: nó lật giá trị đang có.

Chỗ lệch nằm ở **số lần lật**. Câu tiếng Việt "KHÔNG PHẢI là Nam chưa nộp quỹ"
mang hai lần phủ định: một lần trong chữ "chưa nộp", một lần trong chữ "không
phải" đứng ngoài. Đoạn mã cũng có hai chữ `not`. Máy làm chữ `not` bên trong
trước, `True` thành `False`; rồi chữ `not` bên ngoài lật `False` về lại `True`.
Lật hai lần thì về chỗ cũ, nên hai dòng ấy in ra `True`.
::
:::

:::opt
True, True, True, True
::why
Gần đúng ở chỗ bạn nắm được nửa quan trọng của bài: câu B luôn đi cùng câu A, hai
lần lật đưa giá trị về đúng chỗ nó xuất phát. Ba dòng đầu bạn đọc trúng.

Chỗ lệch nằm ở dòng thứ tư, và nó chính là điều bài này dựng lên để nói. Dòng ấy
in `q`, mà `q` vừa được đặt lại thành `False` ngay trên đó — buổi sau Lan chưa
nộp. Ở buổi đầu câu B và câu C cùng cho `True`, nên nhìn hai dòng đầu thì chúng
giống hệt nhau; buổi sau thì chúng tách ra. Cùng giá trị ở một buổi chưa đủ để
gọi là trùng khít.
::
:::

:::opt
Máy báo lỗi ở dòng `print(not (not p))`, vì không đặt được hai chữ `not` liền nhau
::why
Gần đúng ở chỗ bạn đọc mã bằng mắt của người viết tiếng Việt: hai chữ phủ định
đứng sát nhau nghe nặng, và trong nhiều câu tiếng Việt thì đúng là nên viết lại
cho nhẹ.

Chỗ lệch: `not` nhận vào một giá trị Đ/S và trả ra một giá trị Đ/S. Sau khi chữ
`not` bên trong chạy xong, thứ nằm trong tay máy đã là một giá trị Đ/S bình
thường — đúng loại thứ mà `not` nhận được. Nên đặt thêm một chữ `not` nữa trước
nó là hợp lệ, y như đặt `not` trước `p`. Máy trả lời và đi tiếp, không báo gì.
::
:::
::::

::::code{#thay-vao-cho-trong}
Bắt máy dựng bốn cột cạnh nhau, rồi để chính chúng nói ra câu nào thay được cho
câu nào.

`p` là "Nam đã nộp quỹ", `q` là "Lan đã nộp quỹ". Bảng bốn dòng của bài 6 đi hết
mọi kiểu sự việc, không sót không lặp.

Câu bao ngoài dùng chung cho ba cột cuối là:

> **"( … ) VÀ Lan đã nộp quỹ."**

Bốn chỗ cần điền:

- **`cau_b`** — câu B: "KHÔNG PHẢI là Nam chưa nộp quỹ". Viết ra cả hai lần lật,
  đừng viết tắt thành câu A: cả bài dựng lên để so hai **cách viết**, nên cách
  viết phải có mặt.
- **`ngoai_a`** — câu bao ngoài, chỗ trống đặt **câu A**.
- **`ngoai_b`** — cùng câu bao ngoài ấy, chỗ trống đặt **câu B**.
- **`ngoai_c`** — cùng câu bao ngoài ấy, chỗ trống đặt **câu C** ("Lan đã nộp
  quỹ"). Thay vào thì được "Lan đã nộp quỹ VÀ Lan đã nộp quỹ" — nghe kỳ, nhưng
  đó đúng là thứ phép thay nhả ra, và bảng của nó vẫn phải dựng.

Ba cột bao ngoài được chọn để cư xử khác nhau: hai cột trùng khít, cột thứ ba
lệch một dòng. Gõ cứng một giá trị Đ/S vào các chỗ trống thì ba cột giống hệt
nhau và chỗ lệch ấy biến mất.

```python title=starter
cau_a = []     # câu A: "Nam đã nộp quỹ"
cau_b = []     # câu B: "KHÔNG PHẢI là Nam chưa nộp quỹ"
ngoai_a = []   # "( câu A ) VÀ Lan đã nộp quỹ"
ngoai_b = []   # "( câu B ) VÀ Lan đã nộp quỹ"
ngoai_c = []   # "( câu C: Lan đã nộp quỹ ) VÀ Lan đã nộp quỹ"

for p in [True, False]:
    for q in [True, False]:
        cau_a.append(p)
        cau_b.append(___)
        ngoai_a.append(___)
        ngoai_b.append(___)
        ngoai_c.append(___)

print(cau_a == cau_b)
print(ngoai_a == ngoai_b)
print(ngoai_a)
print(ngoai_c)
```

```python title=solution
cau_a = []     # câu A: "Nam đã nộp quỹ"
cau_b = []     # câu B: "KHÔNG PHẢI là Nam chưa nộp quỹ"
ngoai_a = []   # "( câu A ) VÀ Lan đã nộp quỹ"
ngoai_b = []   # "( câu B ) VÀ Lan đã nộp quỹ"
ngoai_c = []   # "( câu C: Lan đã nộp quỹ ) VÀ Lan đã nộp quỹ"

for p in [True, False]:
    for q in [True, False]:
        cau_a.append(p)
        cau_b.append(not (not p))
        ngoai_a.append(p and q)
        ngoai_b.append(not (not p) and q)
        ngoai_c.append(q and q)

print(cau_a == cau_b)
print(ngoai_a == ngoai_b)
print(ngoai_a)
print(ngoai_c)
```

```python title=test
# Hai câu đầu canh đúng hai cái bẫy của bài, nên chúng chạy TRƯỚC. Xếp chúng
# sau các câu `==` thì một câu `==` trượt trước, và bẫy không bao giờ sập.
#
#   · bẫy 1 — chép cùng một thứ vào `ngoai_a` và `ngoai_c`: chỗ lệch mà cả
#     bài dựng lên để chỉ vào sẽ biến mất.
#   · bẫy 2 — chép `ngoai_a` sang `ngoai_b`: hai cột vẫn trùng, nhưng bài
#     không còn chứng minh được gì, vì câu B chưa từng được viết ra.
assert ngoai_a != ngoai_c, "đặt câu A và đặt câu C vào cùng chỗ trống ấy cho ra hai cột KHÁC nhau — chúng lệch ở dòng Nam chưa nộp mà Lan đã nộp; trùng nhau nghĩa là bạn đã chép một câu vào cả hai chỗ"
assert cau_a == cau_b, "câu A và câu B phải cho ra cùng một cột ở cả bốn dòng — lệch một ô nghĩa là câu B chưa lật đủ hai lần"
assert len(cau_b) == 4, "hai vế tự do, mỗi vế hai giá trị: bảng có 4 dòng, nên mỗi cột phải đủ 4 ô"
assert cau_b == [True, True, False, False], "câu B lật giá trị của Nam hai lần nên nó về đúng giá trị ấy: Đ ở hai dòng Nam đã nộp, S ở hai dòng Nam chưa nộp"
assert ngoai_a == [True, False, False, False], "'Nam đã nộp VÀ Lan đã nộp' chỉ đúng ở dòng đầu, dòng duy nhất cả hai cùng đã nộp"
assert ngoai_b == [True, False, False, False], "thay câu B vào đúng chỗ câu A đứng thì cột bao ngoài không đổi một ô nào — vì chữ VÀ chỉ nhìn giá trị hai vế, mà câu A với câu B luôn cùng giá trị"
assert ngoai_c == [True, False, True, False], "'Lan đã nộp VÀ Lan đã nộp' đòi hai vế cùng đúng, mà hai vế ấy là một câu, nên cột này lặp lại đúng cột của Lan: Đ ở dòng một và dòng ba, S ở dòng hai và dòng bốn"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống là **một câu ghép** viết theo `p` và `q`, không phải một giá trị Đ/S gõ sẵn. Đọc lại chú thích bên phải mỗi cột — nó ghi đủ chỗ trống của câu bao ngoài và câu nào được đặt vào. Và đếm xem câu B có mấy lần "không" trong tiếng Việt.
- kind: strategy
  body: "Dịch từng chữ một. Chữ VÀ là `and`; \"Nam chưa nộp quỹ\" là câu ngược của `p` nên viết bằng `not` đặt trước `p`; câu B lại phủ định câu ấy thêm một lần nữa, nên nó có hai chữ `not` chồng lên nhau. Ba cột bao ngoài đều có dạng `( … ) and q`, khác nhau ở đúng thứ điền vào chỗ trống — lần lượt là câu A, câu B, câu C."
- kind: one-line
  body: "Bốn chỗ lần lượt là `not (not p)`, `p and q`, `not (not p) and q`, và `q and q`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: mỗi chỗ trống phải là một câu ghép viết theo `p` và `q` — và câu B phải được viết ra với đủ hai lần lật, cả ở cột `cau_b` lẫn ở chỗ trống của `ngoai_b`; chép câu A sang thì hai cột vẫn trùng, nhưng bài không còn chứng minh được điều gì
  requireAst:
  # Câu B xuất hiện hai lần (ở `cau_b` và trong `ngoai_b`), mỗi lần hai chữ
  # `not`. Khung khởi đầu không có `not` nào, nên luật này một mình chặn cả
  # đáp án gõ cứng Đ/S lẫn đáp án chép câu A vào chỗ của câu B.
  - kind: uses-operator, target: not, min: 4
  # Cả hai lần ấy phải là `not` LỒNG TRONG `not`, không phải hai chữ `not`
  # rải ở hai chỗ khác nhau — đó mới là hình dạng của câu B.
  - kind: nesting, target: not/not, min: 2
  # Ba cột bao ngoài đều nối bằng VÀ. Khung khởi đầu không có `and` nào.
  - kind: uses-operator, target: and, min: 3
  # Nam được nhắc ở bốn chỗ: cột `cau_a` (có sẵn trong khung), cột `cau_b`,
  # và hai cột bao ngoài đặt câu A hay câu B vào. Khung khởi đầu đọc `p`
  # đúng 1 lần, nên con số 4 đo đúng phần người học viết ra.
  - kind: uses-name, target: p, min: 4
  # Lan được nhắc bốn lần: một lần ở mỗi cột bao ngoài, và hai lần ở `ngoai_c`
  # vì câu C bị thay vào chính chỗ trống của một câu ghép nói về Lan.
  - kind: uses-name, target: q, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\nTrue\n\[True, False, False, False\]\n\[True, False, True, False\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cột bao ngoài khớp từng ô. Còn cột thứ ba lệch đúng một dòng — và một dòng là đủ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Thủ quỹ CLB nhìn sổ rồi nói:

> **"Không phải cả Nam và Lan đều đã nộp quỹ."**

Nhiều người nghe xong hiểu ngay thành:

> **"Cả hai đều chưa nộp."**

Hai câu ấy có trùng khít không? Bạn đã có đủ đồ nghề để trả lời mà không cần cãi
nhau bằng cảm giác: dựng hai bảng bốn dòng, đặt hai cột cạnh nhau, tìm một dòng
lệch.

Nếu chúng không trùng khít, thì câu thứ hai không phải là phủ định của "cả hai
đều đã nộp". Vậy câu nào mới đúng là phủ định?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
