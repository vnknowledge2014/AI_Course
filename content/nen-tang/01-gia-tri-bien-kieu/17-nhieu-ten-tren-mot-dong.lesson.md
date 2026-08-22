---
id: nen-tang.gia-tri-bien-kieu.nhieu-ten-tren-mot-dong
title: Nhiều tên trên một dòng
summary: Một dòng gán dựng được nhiều cái tên cùng lúc — với điều kiện số tên bên trái khớp đúng số giá trị bên phải.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.multi-assign]
requires: [core.variable, core.assignment, core.string-literal, core.number-literal, core.fstring, core.value-error, core.none, core.constant]
concepts: [core.bien, core.gan]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Ba cái ô này sinh ra cùng lúc và đi với nhau. Mình muốn nhìn là thấy ngay điều đó.
::::

::::explain{#ba-o-cua-mot-dong-so}
Câu hỏi bỏ ngỏ của bài trước: ba dòng gán dựng ba ô của **một** dòng sổ, xếp
dọc thì mắt không thấy chúng thuộc về nhau.

```python title=readonly
ten = ""
tien = 0
ghi_chu = None
```

Nhìn cách người soát vé phát phiếu ở cửa rạp. Ba người xếp thành hàng, người
soát vé đưa ra ba tấm phiếu: tấm thứ nhất cho người thứ nhất, tấm thứ hai cho
người thứ hai, tấm thứ ba cho người thứ ba. Không ai gọi tên ai — việc ghép
hoàn toàn dựa vào **thứ tự đứng**. Và nếu có ba người mà trong tay chỉ còn hai
phiếu, người soát vé dừng lại hỏi chứ không phát bừa rồi để một người tay
không.

Python cho viết đúng cảnh ấy thành một dòng:

```python title=readonly
ten, tien, ghi_chu = "cà phê", 25000, None

print(ten)
print(tien)
print(ghi_chu)
```

```text
cà phê
25000
None
```

Bên trái dấu `=` là ba cái tên ngăn nhau bằng dấu phẩy. Bên phải là ba giá
trị, cũng ngăn bằng dấu phẩy. Máy ghép chúng theo **đúng thứ tự đứng**: tên
thứ nhất nhận giá trị thứ nhất, tên thứ hai nhận giá trị thứ hai, tên thứ ba
nhận giá trị thứ ba.

Cách viết này gọi là **gán nhiều** — tiếng Anh là *multiple assignment*. Nó
làm được một việc mà ba dòng riêng không làm được: nói ra rằng ba cái tên này
thuộc về nhau và sinh ra cùng lúc. Ba hằng ở đầu file thì vẫn nên đứng riêng
mỗi cái một dòng, vì chúng là ba chuyện không liên quan. Ba ô của một dòng sổ
thì ngược lại — chúng là một chuyện.
::::

::::example{#dem-truoc-khi-dan}
Luật cứng của dòng gán nhiều nằm ở **số lượng**. Thử đưa thiếu một giá trị:

```python title=readonly
ten, tien, ghi_chu = "cà phê", 25000
print(ten)
```

```text
Traceback (most recent call last):
  File "so_chi_tieu.py", line 1, in <module>
    ten, tien, ghi_chu = "cà phê", 25000
    ^^^^^^^^^^^^^^^^^^
ValueError: not enough values to unpack (expected 3, got 2)
```

`ValueError` — cái lỗi Realm 0 đã dạy: thứ bạn đưa đúng loại, nhưng nội dung
của nó dùng không được. Ở đây "dùng không được" nghĩa là **số lượng không
khớp**, và dòng chữ cuối nói thẳng ra: *expected 3, got 2* — chờ ba, nhận hai.

Thừa cũng bị chặn y như thiếu. `ten, tien = "cà phê", 25000, None` cho
`too many values to unpack (expected 2, got 3)`.

Để ý hai chuyện trong cú nổ trên.

**Máy đếm trước khi dán.** `print(ten)` ở dòng 2 không in ra gì cả — mà cả
`ten` cũng không hề được dán. Máy đếm hai bên trước, thấy lệch thì dừng ngay
tại dòng đó, chưa cái tên nào được động tới.

**Máy không tự bù.** Nó không lấy `None` điền vào ô còn thiếu, cũng không vứt
bớt giá trị thừa. Nếu nó tự bù, thì cái ngày bạn gõ sót một giá trị vì đãng
trí, sổ sẽ có một ô trống mà không ai biết.

Còn một chỗ máy **không** đỡ được cho bạn: nó đếm số lượng, nhưng nó không đọc
được ý nghĩa.

```python title=readonly
tien, ten, ghi_chu = "cà phê", 25000, None
print(f"Khoản {ten} hết {tien}đ")
```

```text
Khoản 25000 hết cà phêđ
```

Hai bên đều ba thứ nên số lượng khớp, dòng gán chạy êm — và bây giờ `tien`
đang giữ chữ "cà phê". Thứ tự bên trái phải khớp thứ tự bên phải, và người
giữ việc đó là bạn chứ không phải máy.
::::

::::predict{#doan-thieu-mot-gia-tri commitOnce}
Byte ghi khoản ăn trưa vào sổ nhưng quên mất ô ghi chú. **Trước khi bấm chạy**,
bạn đoán màn hình hiện ra gì?

```python title=readonly
ten, tien, ghi_chu = "ăn trưa", 60000
print(ten)
```

:::opt{correct}
Máy không in dòng nào, chỉ hiện một thông báo ValueError
:::

:::opt
ăn trưa
::why
Gần đúng ở chỗ bạn ghép trúng cặp đầu tiên: `ten` đứng thứ nhất bên trái,
`"ăn trưa"` đứng thứ nhất bên phải, nên đúng là hai thứ ấy đi với nhau. Đọc từ
trái sang, ghép từng cặp một, chính là cách dòng này được đọc.

Chỗ lệch nằm ở lúc máy đếm. Trước khi dán tên nào cả, nó đếm ba tên bên trái
và hai giá trị bên phải, thấy lệch thì dừng ngay tại dòng 1. Không cái tên nào
được dán, kể cả `ten` — nên dòng `print` bên dưới cũng không tới lượt chạy.
::
:::

:::opt
Máy in `ăn trưa`, và `ghi_chu` được máy tự để là None
::why
Gần đúng ở chỗ bạn nhớ rằng Python có sẵn một giá trị mang nghĩa "chưa có gì",
và đúng là `None` sinh ra cho đúng loại ô trống như thế này. Chọn `None` để
lấp vào là chọn đúng thứ.

Chỗ lệch: `None` là thứ **bạn** viết ra, không phải thứ máy tự điền. Máy tự bù
nghĩa là một dòng gõ sót cũng trót lọt, và sổ có một ô rỗng mà không ai biết
là mình đã sót. Đếm rồi dừng là cách nó bắt bạn nói ra ý mình.
::
:::

:::opt
Máy in `ăn trưa` xong rồi mới báo lỗi
::why
Gần đúng ở chỗ bạn hình dung máy chạy lần lượt từ trên xuống, dòng nào xong
thì tới dòng sau — cách hình dung ấy đúng cho phần lớn chương trình, và bài
`int = 0` vừa cho thấy máy in được mấy dòng rồi mới nổ.

Chỗ lệch: ở đây cú `ValueError` nổ ngay tại **dòng 1**, trước khi máy đọc tới
dòng `print`. Một dòng nổ là chương trình dừng hẳn, mọi dòng bên dưới không
được chạy — y như `TypeError` ở Realm 0.
::
:::
::::

::::code{#ghi-khoan-thu-hai}
Sổ chi tiêu chiều nay có hai khoản. Khoản thứ nhất đã dựng sẵn bằng một dòng
gán ba ô, và bạn thấy vế phải của một dòng như vậy trông ra sao.

Khoản thứ hai: tên khoản là **ăn trưa**, số tiền **60000** đồng, ghi chú là
**đi cùng Lan**.

Bên trái dấu `=` đã có sẵn ba cái tên. Chỗ trống là **vế phải** — điền đủ ba
giá trị, đúng thứ tự ba cái tên bên trái.

```python title=starter
ten, tien, ghi_chu = "cà phê", 25000, None
print(f"{ten} | {tien}đ | {ghi_chu}")

ten, tien, ghi_chu = ___
print(f"{ten} | {tien}đ | {ghi_chu}")
```

```python title=solution
ten, tien, ghi_chu = "cà phê", 25000, None
print(f"{ten} | {tien}đ | {ghi_chu}")

ten, tien, ghi_chu = "ăn trưa", 60000, "đi cùng Lan"
print(f"{ten} | {tien}đ | {ghi_chu}")
```

```python title=test
# Ba ô, ba giá trị khác loại nhau: một chuỗi, một số, một chuỗi khác. Đủ số
# lượng mà sai thứ tự thì dòng gán vẫn chạy — chỉ ba dòng assert này bắt được.
assert ten == "ăn trưa", "ô thứ nhất là tên khoản"
assert tien == 60000, "ô thứ hai là số tiền, viết trần không có nháy"
assert ghi_chu == "đi cùng Lan", "ô thứ ba là ghi chú"
```

:::hints
- kind: attention
  body: Nhìn bên trái dấu `=`: ba cái tên ngăn nhau bằng dấu phẩy — `ten`, rồi `tien`, rồi `ghi_chu`. Dòng gán ngay phía trên đã cho thấy vế phải của một dòng như vậy gồm những gì.
- kind: strategy
  body: Vế phải cũng là ba thứ ngăn nhau bằng dấu phẩy, xếp đúng thứ tự ba cái tên bên trái. Tên khoản là chữ nên phải bọc trong dấu nháy; số tiền là số nên viết trần; ghi chú lần này có nội dung nên nó cũng là chữ, khác khoản trước.
- kind: one-line
  body: "Viết `\"ăn trưa\", 60000, \"đi cùng Lan\"` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: ăn trưa | 60000đ | đi cùng Lan
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng, ba ô. Nhìn phát là biết chúng đi cùng nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sổ ghi xong thì phát hiện ghi ngược hai ngày: khoản 50 nghìn là của hôm nay,
còn 30 nghìn mới là của hôm qua. Hai cái tên cần đổi chỗ cho nhau.

Việc nghe rất gọn, nên bạn viết hai dòng, đọc lên xuôi tai:

```python
tien_hom_qua = tien_hom_nay
tien_hom_nay = tien_hom_qua
```

Chạy thử đi. Hai cái tên cùng thành **một** giá trị, và con số kia mất hẳn —
không còn đường nào gọi lại.

Chỗ hỏng không nằm ở chuyện gõ nhầm: hai dòng ấy viết đúng ý bạn định nói.
Nó nằm ở chỗ bạn đang hình dung dấu `=` làm một việc, còn máy thì làm việc
khác.

Vậy máy làm gì với một dòng `=`, và vì sao dòng gán nhiều bạn vừa học lại đổi
chỗ được hai cái tên trong khi hai dòng riêng thì không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
