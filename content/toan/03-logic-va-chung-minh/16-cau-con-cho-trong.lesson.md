---
id: toan.logic-va-chung-minh.cau-con-cho-trong
title: Câu còn một chỗ trống
summary: Câu "bạn ấy đeo thẻ" không đúng cũng không sai — nó còn một chỗ trống, và phải điền một cái tên vào thì mới có một mệnh đề để mà phân xử.
locale: vi
track: toan
module: logic-va-chung-minh
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [logic.open-sentence]
requires: [logic.biconditional, logic.implication, logic.proposition, logic.truth-value, logic.negation, core.boolean, core.dict, core.list, core.function-def, core.function-parameter, core.function-return, core.function-call, core.variable, core.print-variable, ctrl.comparison]
concepts: [logic.cau-mo, logic.cho-trong, logic.dien-ten-thanh-menh-de]
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
"Bạn ấy đeo thẻ." Mình đọc đi đọc lại rồi. Bạn ấy là ai?
::::

::::explain{#cau-khong-chiu-phan-xu}
Bài trước để lại đúng một câu, và nhờ bạn phân xử:

> **Bạn ấy đeo thẻ.**

Cách phân xử thì bài 1 đã dựng: mở sổ ra, đối chiếu với sự việc, rồi nói Đ hay S.

Byte mở sổ điểm danh sáng thứ Hai của CLB cờ vua lớp 6A. Sổ có sáu dòng, mỗi
dòng một cái tên — Nam, Lan, Minh, Hoa, Tú, Khanh. Không dòng nào tên "bạn ấy".

Nên câu ấy không phân xử được. Và đây là chỗ đáng dừng lại lâu hơn một chút: nó
không phân xử được vì một lý do **mới**, chưa bài nào nói tới.

Bài 1 đã loại ba loại câu ra khỏi hàng mệnh đề:

- câu hỏi — "Nam nộp quỹ chưa?";
- câu sai khiến — "Đeo thẻ vào!";
- câu nêu ý thích — "Cờ vua hay hơn cờ tướng".

"Bạn ấy đeo thẻ" không thuộc loại nào trong ba loại ấy. Nó là một câu kể. Nó nói
về chuyện đeo thẻ, mà chuyện đeo thẻ thì tra sổ một cái là ra. Nó không nhờ vả
ai và không khoe ý thích của ai.

Chỗ hỏng nằm ở chỗ khác hẳn: nó chưa nói về **ai**.
::::

::::explain{#dat-ten-cho-cho-trong}
Viết câu ấy ra cho lộ hẳn chỗ hỏng. Thay hai chữ "bạn ấy" bằng một khoảng trống:

```text
    ___ đeo thẻ.
```

Giờ thì thấy rõ. Câu này có một cái ô còn để không, và cái ô ấy đợi một cái tên.

Bỏ **Nam** vào ô: "Nam đeo thẻ." — một câu kể đầy đủ, tra sổ được, mang giá trị Đ.
Bỏ **Hoa** vào ô: "Hoa đeo thẻ." — cũng đầy đủ, cũng tra sổ được, và sáng thứ Hai
Hoa quên thẻ ở nhà nên nó mang giá trị S.

Thứ đang nằm trước mặt bạn có tên riêng:

> Câu có một **chỗ trống** — một ô dành cho một cái tên mà chưa cái tên nào đứng
> vào — gọi là **câu mở**.
>
> Câu mở **chưa** mang giá trị Đ, cũng chưa mang giá trị S. Điền một thành viên
> vào chỗ trống thì nó mới thành một **mệnh đề**, và mệnh đề ấy mới mang giá trị.

Một câu mở vì thế không phải "một câu sai", cũng không phải "một câu đúng". Nó
chưa tới lượt được hỏi câu ấy.
::::

::::explain{#dung-nham-voi-chua-biet}
Có một chỗ rất dễ va ở đây, và nó đáng được tách bạch ngay bây giờ.

Bài 2 đã chốt: mỗi mệnh đề mang **đúng một** trong hai giá trị, và "chưa biết" là
chuyện của người đang xét chứ không phải một giá trị thứ ba. Luật ấy còn nguyên;
bài hôm nay không đụng vào nó. Câu mở không phá luật của bài 2, vì luật ấy nói về
**mệnh đề**, mà câu mở thì chưa phải một mệnh đề.

Đặt hai câu cạnh nhau cho rõ:

- **"Sáng mai Khanh đến CLB."** Chưa ai biết câu này Đ hay S — sáng mai còn chưa
  tới. Nhưng nó là một mệnh đề đầy đủ: nói rõ ai, rõ chuyện gì. Nó đã mang sẵn
  một trong hai giá trị, chỉ là bạn chưa cầm được giá trị ấy trong tay.
- **"Bạn ấy đeo thẻ."** Câu này không thiếu tin tức. Nó thiếu một **chỗ**: không
  có ai để mà tra sổ, nên cũng không có giá trị nào để mà chưa biết.

Một bên thiếu tin, một bên thiếu người. Đó là hai chuyện khác nhau.
::::

::::example{#dien-tung-cai-ten}
CLB cờ vua lớp 6A có sáu thành viên. Sổ của CLB có ba cột, và cả ba cột đều được
ghi cho đủ sáu người:

| thành viên | đeo thẻ (sáng thứ Hai) | đã nộp quỹ tháng này | có mặt buổi họp |
|---|---|---|---|
| Nam | có | rồi | có |
| Lan | có | rồi | có |
| Minh | có | **chưa** | có |
| Hoa | **không** | rồi | có |
| Tú | có | rồi | có |
| Khanh | có | **chưa** | có |

Lấy câu mở `___ đeo thẻ` và **điền từng cái tên vào chỗ trống**, lần lượt hết cả
sáu người:

| điền vào chỗ trống | mệnh đề nhận được | giá trị |
|---|---|---|
| Nam | Nam đeo thẻ | Đ |
| Lan | Lan đeo thẻ | Đ |
| Minh | Minh đeo thẻ | Đ |
| Hoa | Hoa đeo thẻ | **S** |
| Tú | Tú đeo thẻ | Đ |
| Khanh | Khanh đeo thẻ | Đ |

Một câu mở, sáu cái tên, sáu mệnh đề, sáu giá trị.

Bảng nội quy còn dòng khác, và mỗi dòng là một câu mở khác. Lấy `___ đã nộp quỹ
tháng này` rồi làm y như thế:

| điền vào chỗ trống | mệnh đề nhận được | giá trị |
|---|---|---|
| Nam | Nam đã nộp quỹ tháng này | Đ |
| Lan | Lan đã nộp quỹ tháng này | Đ |
| Minh | Minh đã nộp quỹ tháng này | **S** |
| Hoa | Hoa đã nộp quỹ tháng này | Đ |
| Tú | Tú đã nộp quỹ tháng này | Đ |
| Khanh | Khanh đã nộp quỹ tháng này | **S** |

Đặt hai bảng cạnh nhau thì thấy một điều gọn gàng: giá trị nhận được phụ thuộc
**cả hai** thứ — câu mở nào, và cái tên nào.

Cứ nhìn riêng Hoa. Điền Hoa vào câu mở thứ nhất được một mệnh đề **sai**; điền
đúng cái tên ấy vào câu mở thứ hai được một mệnh đề **đúng**. Cái tên không tự nó
quyết định điều gì, và câu mở cũng vậy. Phải có đủ cả hai.
::::

::::explain{#cau-mo-viet-bang-python}
Python có sẵn một thứ mang đúng hình dạng này, và bạn đã dựng nó suốt Realm 1:
một **hàm**.

Nhớ lại cái máy của T2.2: bỏ một thứ vào, một thứ rơi ra. Cái máy ấy có một chỗ
để bỏ đồ vào — chính là **tham số** — và chừng nào chưa bỏ gì vào thì chưa có kết
quả nào rơi ra.

Chỗ để bỏ đồ vào của một cái máy, và chỗ trống của một câu mở, là cùng một
chuyện:

```text
    câu mở:   ___ đeo thẻ.          chỗ trống đợi một cái tên
    hàm:      cau_deo_the(ten)      tham số `ten` đợi một cái tên
```

Nên một câu mở viết bằng Python là một hàm **nhận một cái tên** và **trả về `True`
hoặc `False`**:

```python title=readonly
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}

def cau_deo_the(ten):
    return deo_the[ten]
```

Cuốn sổ `deo_the` là cột "đeo thẻ (sáng thứ Hai)" của cái bảng ở trên, chép sang
kiểu `dict` của Realm 1: tra một cái tên, nhận lại một giá trị Đ/S.

Còn `cau_deo_the` thì chưa nói về ai cả — nó là cái câu còn chỗ trống. Gọi
`cau_deo_the("Hoa")` mới là hành động điền tên vào, và chỉ từ lúc ấy mới có một
giá trị để in ra.
::::

::::predict{#doan-dien-ten commitOnce}
Byte gõ mấy dòng để nhìn tận mắt chuyện điền tên.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}

def cau_deo_the(ten):
    return deo_the[ten]

print(cau_deo_the("Nam"))
print(cau_deo_the("Hoa"))
print(cau_deo_the("Nam") == cau_deo_the("Lan"))
```

:::opt{correct}
`True`, rồi `False`, rồi `True`
:::

:::opt
`True`, rồi `False`, rồi `False`
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng đầu, và ở chỗ bạn bám vào một sự thật hoàn
toàn đúng: Nam và Lan là **hai người khác nhau**, hai cái tên khác nhau, không ai
lẫn với ai.

Chỗ lệch nằm ở chuyện dấu `==` đang so cái gì. Nó không so hai cái tên — hai cái
tên đã bị dùng hết ở bước điền vào chỗ trống rồi. Cái còn lại trong tay sau bước
ấy là hai **giá trị**: mệnh đề "Nam đeo thẻ" mang Đ, mệnh đề "Lan đeo thẻ" cũng
mang Đ. Dấu `==` so đúng hai giá trị ấy, và chúng bằng nhau.

Đây chính là điều bài 4 đã chốt bằng chữ khác: chuyện một câu ghép đúng hay sai
chỉ do **giá trị** của các vế quyết định, không do vế ấy nói về ai.
::
:::

:::opt
`True`, rồi `False`, rồi máy báo lỗi ở dòng cuối, vì `==` chỉ so được số với số
::why
Gần đúng ở chỗ bạn nhớ đúng nơi mình đã gặp dấu `==` lần đầu: Realm 0 dùng nó để
so hai con số, rồi so hai chuỗi chữ. Cẩn thận với kiểu giá trị là một thói quen
tốt, và ở nhiều chỗ khác nó cứu bạn thật.

Chỗ lệch: dấu `==` hỏi đúng một câu — "hai thứ này có phải cùng một giá trị
không" — và nó hỏi được câu ấy về bất kỳ loại giá trị nào, kể cả `True` và
`False`. Bài 7 đã cho hai giá trị Đ/S đứng cạnh nhau trong cùng một cột bảng và
so từng ô; ở đây cũng là chuyện ấy, chỉ khác là máy so hộ. Nên máy không báo lỗi,
nó in ra một giá trị Đ/S bình thường.
::
:::

:::opt
Máy báo lỗi ngay dòng `print(cau_deo_the("Nam"))`, vì câu "___ đeo thẻ" là câu mở nên chưa mang giá trị nào
::why
Gần đúng ở chỗ bạn nói ra đúng điều quan trọng nhất của cả bài này, và nói không
sai một chữ: **câu mở `___ đeo thẻ` chưa mang giá trị nào.** Giữ chặt câu đó.

Chỗ lệch: dòng ấy không in cái câu mở ra. Nó in `cau_deo_the("Nam")` — tức là câu
mở **sau khi đã điền** cái tên Nam vào chỗ trống. Điền xong thì chỗ trống không
còn trống nữa, câu đã thành một mệnh đề đầy đủ, và mệnh đề thì mang giá trị.

Cái chưa mang giá trị là `cau_deo_the` đứng một mình, không có ngoặc, không có
tên nào bỏ vào. Cái đó Python cũng in ra được, nhưng thứ nó in ra là một dòng mô
tả cái hàm chứ không phải `True` hay `False` — đúng như một câu còn chỗ trống thì
chưa có Đ hay S để mà khoe.
::
:::
::::

::::code{#ba-cau-mo}
Giờ bạn dựng lấy ba câu mở, mỗi câu tra một cột của cuốn sổ, rồi điền tên vào
từng câu.

Ba chỗ trống, đều nằm sau chữ `return`, và cả ba đều phải viết theo cái tên `ten`
— vì `ten` chính là chỗ trống của câu mở. Chú thích ngay trên mỗi `def` ghi rõ
câu mở ấy đọc là gì và tra cuốn sổ nào.

Bài chấm bằng **cả ba câu mở và nhiều cái tên khác nhau**, và ba cuốn sổ được
chọn để cãi nhau: riêng Hoa thì sổ thẻ ghi "không", hai sổ kia ghi "có". Gõ cứng
`True` hay `False` vào một chỗ trống thì một cái tên nào đó sẽ lật tẩy ngay, nên
phải tra sổ thật.

```python title=starter
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}
quy_thang_nay = {"Nam": True, "Lan": True, "Minh": False,
                 "Hoa": True, "Tú": True, "Khanh": False}
co_mat = {"Nam": True, "Lan": True, "Minh": True,
          "Hoa": True, "Tú": True, "Khanh": True}

# Câu mở "___ đeo thẻ" — tra cuốn sổ `deo_the`.
def cau_deo_the(ten):
    return ___

# Câu mở "___ đã nộp quỹ tháng này" — tra cuốn sổ `quy_thang_nay`.
def cau_da_nop_quy(ten):
    return ___

# Câu mở "___ có mặt buổi họp" — tra cuốn sổ `co_mat`.
def cau_co_mat(ten):
    return ___

# Cùng một cái tên, ba câu mở khác nhau.
print(cau_deo_the("Hoa"))
print(cau_da_nop_quy("Hoa"))
print(cau_co_mat("Hoa"))
# Cùng một câu mở, một cái tên khác.
print(cau_deo_the("Nam"))
```

```python title=solution
deo_the = {"Nam": True, "Lan": True, "Minh": True,
           "Hoa": False, "Tú": True, "Khanh": True}
quy_thang_nay = {"Nam": True, "Lan": True, "Minh": False,
                 "Hoa": True, "Tú": True, "Khanh": False}
co_mat = {"Nam": True, "Lan": True, "Minh": True,
          "Hoa": True, "Tú": True, "Khanh": True}

# Câu mở "___ đeo thẻ" — tra cuốn sổ `deo_the`.
def cau_deo_the(ten):
    return deo_the[ten]

# Câu mở "___ đã nộp quỹ tháng này" — tra cuốn sổ `quy_thang_nay`.
def cau_da_nop_quy(ten):
    return quy_thang_nay[ten]

# Câu mở "___ có mặt buổi họp" — tra cuốn sổ `co_mat`.
def cau_co_mat(ten):
    return co_mat[ten]

# Cùng một cái tên, ba câu mở khác nhau.
print(cau_deo_the("Hoa"))
print(cau_da_nop_quy("Hoa"))
print(cau_co_mat("Hoa"))
# Cùng một câu mở, một cái tên khác.
print(cau_deo_the("Nam"))
```

```python title=test
# Ba câu `!=` đứng ĐẦU. Chúng canh cái bẫy lớn nhất của bài: chép cùng một cuốn
# sổ vào cả ba câu mở. Xếp chúng xuống dưới các câu `==` thì một câu `==` trượt
# trước và ba cái bẫy không bao giờ sập.
assert cau_deo_the("Hoa") != cau_da_nop_quy("Hoa"), "sáng thứ Hai Hoa quên thẻ nhưng đã nộp quỹ tháng này, nên riêng cái tên Hoa phải cho hai giá trị ngược nhau ở hai câu mở này"
assert cau_deo_the("Hoa") != cau_co_mat("Hoa"), "Hoa không đeo thẻ mà vẫn có mặt buổi họp, nên hai câu mở này không thể cho cùng một giá trị khi điền tên Hoa"
assert cau_da_nop_quy("Khanh") != cau_co_mat("Khanh"), "Khanh có mặt buổi họp nhưng chưa nộp quỹ tháng này, nên hai câu mở này lệch nhau ở cái tên Khanh"
assert cau_deo_the("Nam") is True, "sổ sáng thứ Hai ghi Nam có đeo thẻ, nên mệnh đề `Nam đeo thẻ` mang giá trị Đ"
assert cau_deo_the("Hoa") is False, "sổ sáng thứ Hai ghi Hoa không đeo thẻ, nên riêng mệnh đề `Hoa đeo thẻ` mang giá trị S"
assert cau_deo_the("Khanh") is True, "sổ sáng thứ Hai ghi Khanh có đeo thẻ, nên mệnh đề `Khanh đeo thẻ` mang giá trị Đ"
assert cau_da_nop_quy("Minh") is False, "sổ quỹ ghi Minh chưa nộp tháng này, nên mệnh đề `Minh đã nộp quỹ tháng này` mang giá trị S"
assert cau_da_nop_quy("Khanh") is False, "sổ quỹ ghi Khanh chưa nộp tháng này, nên mệnh đề `Khanh đã nộp quỹ tháng này` mang giá trị S"
assert cau_da_nop_quy("Hoa") is True, "sổ quỹ ghi Hoa đã nộp tháng này, nên mệnh đề `Hoa đã nộp quỹ tháng này` mang giá trị Đ"
assert cau_co_mat("Minh") is True, "sổ họp ghi Minh có mặt, nên mệnh đề `Minh có mặt buổi họp` mang giá trị Đ"
assert cau_co_mat("Khanh") is True, "sổ họp ghi Khanh có mặt, nên mệnh đề `Khanh có mặt buổi họp` mang giá trị Đ"
```

:::hints
- kind: attention
  body: Cả ba chỗ trống đều đứng sau chữ `return`, và cả ba đều phải nhắc tới cái tên `ten` — vì `ten` đúng là chỗ trống của câu mở, còn cái đứng sau `return` là giá trị mà mệnh đề mang sau khi đã điền tên vào. Đọc chú thích ngay phía trên mỗi `def`: nó ghi sẵn câu mở ấy tra cuốn sổ nào trong ba cuốn.
- kind: strategy
  body: "Tra một cuốn sổ `dict` là chuyện Realm 1 đã dựng: viết tên cuốn sổ, rồi đặt cái cần tra vào trong cặp ngoặc vuông ngay sau nó. Ở đây cái cần tra là cái tên vừa được điền vào chỗ trống, tức là `ten`. Ba câu mở thì tra ba cuốn sổ khác nhau — chép cùng một cuốn vào cả ba chỗ thì riêng cái tên Hoa sẽ cho ba giá trị giống hệt nhau, mà sổ thì ghi không phải thế."
- kind: one-line
  body: "Ba chỗ lần lượt là `deo_the[ten]`, `quy_thang_nay[ten]` và `co_mat[ten]`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi câu mở phải TRA SỔ theo cái tên vừa điền vào — gõ cứng `True` hay `False` thì câu mở không còn chỗ trống nào, và cả bài học này nói về đúng cái chỗ trống ấy
  requireAst:
  # Ba cuốn sổ, mỗi cuốn phải được ĐỌC ít nhất một lần. Ba dòng `... = {...}` ở
  # đầu khung là gán, không phải đọc, nên khung khởi đầu đọc cả ba cuốn 0 lần.
  # Ba luật này một mình chặn mọi đáp án gõ cứng giá trị Đ/S vào chỗ trống.
  - kind: uses-name, target: deo_the, min: 1
  - kind: uses-name, target: quy_thang_nay, min: 1
  - kind: uses-name, target: co_mat, min: 1
  # Chỗ trống của câu mở phải thật sự được dùng. Khung khởi đầu đọc `ten` 0 lần
  # (`def cau_deo_the(ten)` là khai tham số, không phải đọc nó), lời giải đọc 3.
  # Thiếu luật này thì `return deo_the["Hoa"]` — một câu ĐÃ điền sẵn tên, tức
  # không còn chỗ trống nào — vẫn in ra đúng dòng đầu và lọt qua.
  #
  # (Không dùng `forbidAst: has-literal` cho ba cái tên để chặn chuyện ấy: luật
  # `has-literal` đếm trên CẢ tệp, mà ba cuốn sổ và bốn dòng `print` của khung
  # khởi đầu đã chứa sẵn chúng — nó sẽ đánh trượt cả lời giải đúng.)
  - kind: uses-name, target: ten, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^False\nTrue\nTrue\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một cái tên, ba câu mở, ba câu trả lời khác nhau. Giờ mình biết phải hỏi đủ hai
thứ mới có quyền nói Đ hay S.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa điền lần lượt sáu cái tên vào câu mở `___ đeo thẻ` và thu về sáu mệnh đề,
sáu giá trị Đ/S nằm thành một cột.

Nhưng dòng trên bảng nội quy CLB không viết sáu câu. Nó viết đúng **một** câu:

> Mọi thành viên đều đeo thẻ.

Một câu thì mang một giá trị. Vậy câu ấy thu sáu giá trị kia về thành **một** giá
trị bằng cách nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
