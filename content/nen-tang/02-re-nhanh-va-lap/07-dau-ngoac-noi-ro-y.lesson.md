---
id: nen-tang.re-nhanh-va-lap.dau-ngoac-noi-ro-y
title: Dấu ngoặc nói rõ ý bạn
summary: Cặp ngoặc gom mấy chữ thành một cụm và bắt máy tính cụm ấy trước — đè lên thứ tự mặc định.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [logic.parentheses]
requires: [logic.precedence, logic.not, logic.and, logic.or]
concepts: [logic.phep-logic, core.dung-sai]
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
Mình không tự đoán chỗ nào thuộc về chỗ nào. Bạn khoanh lại thì mình theo.
::::

::::explain{#cau-noi-bi-cat-doi}
Sáng nay Byte tính đi ăn phở. Đi được khi **vừa đủ tiền vừa kịp giờ** — thiếu
một trong hai là thôi. Byte muốn máy nhắc một câu vào đúng những sáng phải
nhịn:

> Không phải là vừa đủ tiền vừa kịp giờ.

Trong câu tiếng Việt đó, hai chữ *không phải* trùm lên **cả cụm** phía sau. Chữ
*không phải* của Python là `not` — nó lật một câu đúng thành sai, và một câu
sai thành đúng. Viết thẳng câu trên ra Python:

```python
not du_tien and kip_gio
```

Máy không đọc như bạn nói. Máy có một bảng thứ tự cố định, dùng cho mọi dòng nó
gặp: số học → so sánh → `not` → `and` → `or`. Trong bảng ấy `not` được tính
**trước** `and`, nghĩa là `not` chỉ ôm đúng cái tên đứng sát ngay sau nó —
`du_tien` — rồi mới tới lượt `and`. Câu máy đọc được là:

> Không đủ tiền, **mà lại** kịp giờ.

Hai câu khác hẳn nhau. Câu của bạn nói *thiếu ít nhất một trong hai*. Câu máy
đọc nói *chắc chắn thiếu tiền, và chắc chắn đủ giờ*.

Chỗ hụt không nằm ở thứ tự — thứ tự thì máy làm đúng như đã hứa. Chỗ hụt là bạn
chưa có cách nói *"mấy chữ này gom lại thành một cụm"*.

Cách nói ấy bạn đã dùng từ hồi học toán ở trường: `2 + 3 * 4` ra 14, còn
`(2 + 3) * 4` ra 20. Dấu ngoặc gom `2 + 3` thành một món, và món trong ngoặc
được tính trước.

Python dùng lại đúng dấu ngoặc đó, và dùng cho mọi loại giá trị chứ không riêng
số.
::::

::::predict{#hai-dong-mot-cap-ngoac commitOnce}
Sáng nay Byte đủ tiền, nhưng dậy muộn nên không kịp giờ — tức là một sáng phải
nhịn phở. Byte viết hai dòng chỉ khác nhau đúng một cặp ngoặc. **Trước khi bấm
chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
du_tien = True
kip_gio = False

print(not du_tien and kip_gio)
print(not (du_tien and kip_gio))
```

:::opt{correct}
`False` rồi `True`
:::

:::opt
`True` rồi `True` — hai dòng cùng một ý, cặp ngoặc chỉ để cho dễ đọc
::why
Gần đúng ở chỗ bạn đọc dòng thứ nhất y như câu tiếng Việt vừa viết ra: *không
phải là vừa đủ tiền vừa kịp giờ*. Sáng nay đúng là phải nhịn thật, nên `True`
là câu trả lời đúng cho **ý** ấy.

Chỗ lệch: máy không đọc câu tiếng Việt, nó đọc theo bảng thứ tự. Ở dòng không
có ngoặc, `not` chỉ ôm được `du_tien`, nên máy làm `not True` ra `False` trước
đã, rồi mới `False and kip_gio`. Cặp ngoặc ở dòng dưới không phải để cho dễ
đọc — nó đổi hẳn phép nào được tính trước, nên nó đổi luôn kết quả.
::
:::

:::opt
`False` rồi `False` — `not` bao giờ cũng tính trước, có ngoặc hay không cũng thế
::why
Gần đúng ở chỗ bạn nhớ đúng bảng thứ tự và áp thẳng nó vào cả hai dòng: `not`
đứng trên `and` nên `not` đi trước. Dòng thứ nhất bạn tính ra `False`, và
`False` đó đúng.

Chỗ lệch: bảng thứ tự chỉ phân xử khi mọi phép nằm chung một hàng. Cặp ngoặc ở
dòng thứ hai chen vào giữa `not` và `du_tien`, gom `du_tien and kip_gio` thành
một món riêng — `not` không còn cái tên nào sát bên để ôm, nó phải đợi món ấy
tính xong mới có thứ để lật.
::
:::

:::opt
Máy báo lỗi: dấu ngoặc là của `print` và `input`, không đặt giữa dòng được
::why
Gần đúng ở chỗ mọi dấu ngoặc bạn gặp cho tới hôm nay đều dính liền sau một cái
tên: `print(...)`, `input(...)`, `int(...)`. Thấy chúng đi cùng nhau suốt mấy
chục bài thì nghĩ chúng thuộc về nhau là chuyện tự nhiên.

Chỗ lệch: dấu ngoặc làm hai việc, và máy phân biệt bằng **thứ đứng ngay
trước nó**. Đi sau một cái tên gọi được — `print`, `input`, `int` — thì nó
nghĩa là *"gọi việc này"*. Đi sau một từ khoá hay một dấu phép, tức ở chỗ máy
đang chờ một giá trị — như cặp ngoặc sau `not` ở dòng thứ năm — thì nó nghĩa
là *"gom cụm này lại"*. Cả hai đều hợp lệ.

Đừng đo bằng dấu cách: `print (x)` có khoảng trắng mà vẫn là lời gọi, còn
`not(du_tien and kip_gio)` không có khoảng trắng mà vẫn là gom cụm. Cái quyết
định là ai đứng trước, không phải bao nhiêu chỗ trống.
::
:::
::::

::::example{#hai-cach-doc-mot-dong}
Bấm chạy hai dòng ấy:

```python title=readonly
du_tien = True
kip_gio = False

print(not du_tien and kip_gio)
print(not (du_tien and kip_gio))
```

Máy in ra:

```text
False
True
```

Hai dòng chỉ khác nhau một cặp ngoặc, và chúng cho hai câu trả lời ngược nhau.
Đi lại từng dòng:

- Dòng thứ tư: `not` ôm `du_tien` trước, `not True` cho `False`. Còn lại
  `False and kip_gio`, tức `False and False`, cho `False`.
- Dòng thứ năm: cụm trong ngoặc được tính trước, `True and False` cho `False`.
  Rồi `not` ôm cả kết quả ấy, `not False` cho `True`.

`True` ở dòng cuối chính là câu bạn muốn nói: sáng nay **không phải** vừa đủ
tiền vừa kịp giờ, nên đúng là phải nhịn phở thật.

Câu ngắn để nhớ: dấu ngoặc **đè lên** bảng thứ tự. Máy vẫn giữ nguyên bảng ấy ở
mọi chỗ khác; riêng cụm nằm trong ngoặc thì được tính trước tất cả.
::::

::::explain{#ngoac-de-len-thu-tu}
Hai điều dùng được ngay từ hôm nay:

- **Trong ngoặc tính trước.** Máy làm xong cụm bên trong, thu về đúng một giá
  trị, rồi mới đem giá trị ấy đi tính tiếp với phần còn lại.
- **Ngoặc thừa không sai.** `(tien > 200000) and da_ghi_so` cho kết quả y hệt
  khi bỏ cặp ngoặc đi, vì so sánh vốn đã được tính trước `and`. Cặp ngoặc ấy
  không đổi kết quả, nó chỉ đỡ cho người đọc khỏi phải nhớ bảng thứ tự. Viết
  ngoặc cho rõ ý là chuyện nên làm.

> Dễ nhầm: `not a and b` và `not (a and b)` là hai câu khác nhau, nhưng chúng
> cho cùng kết quả trong một nửa số trường hợp — đủ để bạn chạy thử đúng một lần
> thấy khớp rồi tin nhầm. Hễ trong đầu bạn có chữ *"không phải là **cả** hai
> điều này"* thì cặp ngoặc là bắt buộc.
::::

::::code{#noi-dung-y-ban}
Byte ghi lại bốn buổi sáng đầu tuần — đủ cả bốn cách mà tiền và giờ ghép được
với nhau:

- **Thứ hai** — đủ tiền, nhưng dậy muộn nên không kịp giờ.
- **Thứ ba** — vừa đủ tiền, vừa kịp giờ.
- **Thứ tư** — kịp giờ, nhưng ví đã hết tiền.
- **Thứ năm** — ví hết tiền, mà cũng dậy muộn.

Câu nhắc chỉ được hiện ra vào những sáng **không phải là vừa đủ tiền vừa kịp
giờ**. Cùng một điều kiện ấy xét cho cả bốn buổi sáng, nên **bốn chỗ trống điền
giống hệt nhau**.

Viết đúng thì màn hình hiện ra **ba** trong bốn dòng chữ.

```python title=starter
du_tien = True
kip_gio = False
if ___:
    print("Sáng thứ hai nhịn phở")

du_tien = True
kip_gio = True
if ___:
    print("Sáng thứ ba nhịn phở")

du_tien = False
kip_gio = True
if ___:
    print("Sáng thứ tư nhịn phở")

du_tien = False
kip_gio = False
if ___:
    print("Sáng thứ năm nhịn phở")
```

```python title=solution
du_tien = True
kip_gio = False
if not (du_tien and kip_gio):
    print("Sáng thứ hai nhịn phở")

du_tien = True
kip_gio = True
if not (du_tien and kip_gio):
    print("Sáng thứ ba nhịn phở")

du_tien = False
kip_gio = True
if not (du_tien and kip_gio):
    print("Sáng thứ tư nhịn phở")

du_tien = False
kip_gio = False
if not (du_tien and kip_gio):
    print("Sáng thứ năm nhịn phở")
```

```python title=test
# Chấm bằng TRỌN VẸN output, và chấm trên CẢ BỐN buổi sáng.
#
# Một buổi sáng thôi thì không đủ để phân biệt đúng với sai: điều kiện nào
# cũng chỉ cho ra `True` hoặc `False`, nên người gõ bừa có đúng hai lựa chọn
# và luôn trúng một cái. Bốn buổi sáng là đủ cả bốn cách ghép `True`/`False`,
# nên không còn chỗ nào cho may rủi trốn.
#
# - Điền một thứ luôn đúng: cả bốn dòng cùng hiện.
# - Điền một thứ luôn sai: không dòng nào hiện.
# - Điền `not kip_gio`: thứ hai và thứ năm lên tiếng, thứ tư mất tăm.
# - Điền `du_tien != kip_gio`: thứ hai và thứ tư khớp, nhưng thứ năm im —
#   đây đúng là đáp án chỉ lộ ra khi có buổi sáng thứ tư trong đề.
# - Điền `not du_tien and kip_gio` — đúng cái bẫy bài này dựng lên: thứ hai
#   im lặng dù đó là sáng phải nhịn thật, còn thứ tư thì lên tiếng. Chạy thử
#   một buổi sáng thấy khớp là tin nhầm ngay.
#
# Thứ hai và thứ năm là hai buổi mà cái bẫy trả lời khác điều kiện thật, thứ
# ba và thứ tư là hai buổi nó trả lời giống — đúng "một nửa số trường hợp" mà
# hộp Dễ nhầm phía trên đã báo.
pass
```

:::hints
- kind: attention
  body: Ý bạn có hai phần: một cụm "vừa… vừa…" gồm hai điều kiện, và chữ "không phải" trùm lên cả cụm ấy. Trong dòng code, cái gì đánh dấu ranh giới của cả cụm?
- kind: strategy
  body: Viết cụm "vừa đủ tiền vừa kịp giờ" ra trước bằng `and`, khoanh nó lại, rồi đặt `not` ở ngoài cùng. Đặt `not` sát vào một cái tên là nó chỉ lật đúng cái tên đó. Bốn buổi sáng chỉ khác nhau ở hai giá trị `True`/`False` phía trên, còn câu hỏi thì không đổi — nên bốn chỗ trống điền y hệt nhau.
- kind: one-line
  body: "Viết `not (du_tien and kip_gio)` vào cả bốn chỗ trống, giữ nguyên dấu hai chấm ở cuối mỗi dòng `if`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Sáng thứ hai nhịn phở\nSáng thứ tư nhịn phở\nSáng thứ năm nhịn phở$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn buổi sáng, một điều kiện, và nó đúng ở cả bốn. Cặp ngoặc đó là chỗ bạn chỉ ranh giới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bốn buổi sáng vừa rồi, bạn gõ cùng một điều kiện bốn lần. Gõ được là vì bạn
biết trước có đúng bốn buổi — đề bài đã đếm hộ bạn.

Ngoài quán thì không ai đếm hộ. Từ Realm 0, cách hỏi khách là `input`, và máy
hỏi **đúng một lần**: khách gõ nhầm một chữ thì chương trình vẫn cứ đi tiếp với
chữ gõ nhầm ấy. Muốn hỏi lại cho tới khi khách gõ đúng thì phải lặp.

Mà hai lối lặp bạn đang có đều đòi biết trước. `for ngay in range(n)` đòi con số
`n` ngay lúc bạn gõ code. `for mon in danh_sach` đòi cả danh sách phải có sẵn từ
trước khi vòng chạy lượt đầu.

Vậy `n` bằng bao nhiêu? Viết 3, gặp người gõ sai lần thứ tư thì sao? Viết 100
thì 99 lượt còn lại để làm gì?

Câu hỏi thật ra là: có lối lặp nào **không cần biết trước số lượt** không? Bài
sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
