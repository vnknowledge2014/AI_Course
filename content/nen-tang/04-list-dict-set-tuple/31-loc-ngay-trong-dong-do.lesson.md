---
id: nen-tang.list-dict-set-tuple.loc-ngay-trong-dong-do
title: Lọc ngay trong dòng đó
summary: Một chữ `if` đặt ở cuối dòng gọn quyết định khoản nào được lọt ra; phần đầu dòng vẫn quyết định thứ lọt ra là gì.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 31
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.comprehension-filter]
practices: [core.list-comprehension, core.list-of-dicts, core.dict-lookup, core.len, core.fstring]
requires: [core.list-comprehension, core.list-of-dicts, core.dict, core.list, core.list-append, core.len, ctrl.for-each, ctrl.if, core.fstring]
concepts: [core.danh-sach, ctrl.re-nhanh]
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
Dòng gọn của bạn còn thừa một chỗ ở cuối. Mình để dành nó đúng cho hôm nay.
::::

::::explain{#mot-dong-lay-het}
Bài trước bạn gói cả bốn dòng vào một: nói thẳng **lấy gì**, rồi **từ đâu**, và
máy dựng ra một danh sách mới.

Chỉ có điều dòng ấy lấy **hết**. Cuốn sổ có bao nhiêu khoản thì danh sách ra bấy
nhiêu tên, kể cả những khoản lẻ tẻ vài chục nghìn.

Byte thì đang cần một câu hỏi hẹp hơn: *những khoản nào tiêu quá 100 nghìn?*

Viết theo lối dài — cái lối bạn đã gõ bằng tay suốt mấy bài — thì chỗ đặt câu
hỏi ấy rõ mồn một: một `if` bọc lấy dòng `.append`. Khoản nào lọt qua `if` mới
được thêm vào.

Dòng gọn cũng có đúng chỗ ấy, và nó nằm ở **cuối dòng**, ngay sau phần `for`:

```text title=readonly
[  lấy gì   for  từng cái  in  từ đâu  if  điều kiện  ]
```

Đọc từ trái sang phải thì thứ tự nghe hơi ngược. Nhưng đọc theo thứ tự **máy làm
việc** thì nó đúng y lối dài: đi từ đâu trước, hỏi điều kiện, rồi mới lấy.
::::

::::example{#hai-ban-cung-mot-cau}
Vẫn cuốn sổ của bài 23 — một list các dict, mỗi khoản bốn trường `ten`, `tien`,
`ngay`, `nhom`. Có điều Byte đã ghi tiếp: cuốn sổ ở bài 23 dừng ở ngày 15 với
năm khoản, còn cuốn dưới đây chạy tới ngày 22 và dài **tám** khoản. Ba khoản mới
nằm ở cuối, và khoản cuối cùng trùng tên với khoản ngày 5 — tháng này Byte đổ
xăng hai lần.

Bản dài trước, bản bạn đã biết đọc:

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xe cộ"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xe cộ"},
    {"ten": "biếu bà", "tien": 300000, "ngay": 15, "nhom": "biếu tặng"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xe cộ"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xe cộ"},
]

dat = []
for khoan in so:
    if khoan["tien"] > 100000:
        dat.append(khoan["ten"])

print(dat)
```

Máy in ra:

```text title=readonly
['xăng', 'ăn trưa', 'sửa xe', 'biếu bà', 'xăng']
```

Giờ là bản gọn, chạy trên đúng cuốn sổ ấy:

```python title=readonly
dat = [khoan["ten"] for khoan in so if khoan["tien"] > 100000]
print(dat)
```

Cùng một danh sách đi ra. Đặt hai bản cạnh nhau thì thấy chúng dùng lại nguyên si
từng mảnh:

- `for khoan in so` — y hệt dòng `for` của bản dài.
- `if khoan["tien"] > 100000` — y hệt dòng `if`, mất mỗi dấu hai chấm.
- `khoan["ten"]` — y hệt thứ nằm trong ngoặc của `.append`, chỉ khác là nó
  chuyển lên đứng đầu.

Ba dòng của bản dài không mất đi đâu cả. Chúng chỉ bị xếp lại thành một hàng, và
duy nhất **một** mảnh đổi chỗ: thứ được lấy ra chạy lên trước.
::::

::::predict{#doan-danh-sach-ten commitOnce}
Vẫn cuốn sổ tám khoản đó, chạy bản gọn.

**Trước khi bấm chạy**, bạn đoán màn hình in ra gì?

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xe cộ"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xe cộ"},
    {"ten": "biếu bà", "tien": 300000, "ngay": 15, "nhom": "biếu tặng"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xe cộ"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xe cộ"},
]

dat = [khoan["ten"] for khoan in so if khoan["tien"] > 100000]
print(dat)
```

:::opt{correct}
['xăng', 'ăn trưa', 'sửa xe', 'biếu bà', 'xăng']
:::

:::opt
['xăng', 'ăn trưa', 'sửa xe', 'biếu bà', 'vá lốp', 'xăng']
::why
Gần đúng ở chỗ khó nhất: bạn đọc ra rằng đầu dòng lấy **tên**, phần `if` chỉ
chọn ai được lọt, và bạn giữ đúng thứ tự ghi sổ — kể cả chuyện "xăng" xuất hiện
hai lần.

Chỗ lệch nằm ở đúng một khoản: "vá lốp", tròn 100 nghìn. Dấu `>` hỏi "có **lớn
hơn** không", mà một con số thì không lớn hơn chính nó — nên ở khoản ấy điều
kiện đọc ra `False` và nó không được thêm vào.

Muốn "vá lốp" lọt thì phải đổi dấu thành `>=`. Hai dấu ấy chỉ khác nhau ở đúng
những khoản rơi trúng con số ngưỡng, và cuốn sổ này cố tình có một khoản như vậy
để chỗ khác nhau đó lộ ra.
::
:::

:::opt
[240000, 620000, 500000, 300000, 105000]
::why
Gần đúng ở chỗ bạn theo dõi phần lọc rất chuẩn: đúng năm khoản ấy vượt 100
nghìn, không thừa không thiếu một khoản nào, và thứ tự cũng đúng.

Chỗ lệch là ở chuyện **ai** quyết định thứ đi ra. Một dòng gọn có hai việc tách
bạch: phần `if` ở cuối chỉ nói *khoản này có được lọt hay không*, còn phần đứng
**đầu dòng** mới nói *lọt rồi thì lấy cái gì của nó*. Ở đây đầu dòng viết
`khoan["ten"]`, nên thứ đi ra là tên.

Muốn ra đúng năm con số bạn vừa kể thì đổi đầu dòng thành `khoan["tien"]`, còn
phần `if` giữ nguyên không đụng tới.
::
:::

:::opt
['xăng', 'ăn trưa', 'sửa xe', 'biếu bà']
::why
Gần đúng ở chỗ bạn lọc không sai một khoản nào: bốn cái tên này đều thuộc về
những khoản trên 100 nghìn, và khoản "vá lốp" tròn ngưỡng bị bạn loại ra rất
chuẩn.

Chỗ lệch là ở chuyện chữ "xăng" đã có sẵn trong danh sách rồi thì lần sau có
được ghi nữa không. Thứ **không** nhận hai lần là cái rổ ở bài 26. List thì
nhận: nó chỉ chép lại đúng những gì từng lượt `for` đưa cho, theo đúng thứ tự.

Cuốn sổ có hai khoản tên "xăng" (ngày 5 và ngày 22), cả hai đều trên 100 nghìn,
nên cả hai đều đi ra — thành hai phần tử riêng, nằm ở hai đầu danh sách.
::
:::
::::

::::explain{#dau-dong-va-cuoi-dong}
Một dòng gọn có lọc thì mang **hai** quyết định, và chúng không giẫm lên nhau:

- **Đầu dòng quyết định lấy cái gì.** `khoan["ten"]` cho ra tên,
  `khoan["tien"]` cho ra tiền, `khoan` trơ trọi cho ra nguyên cả dict.
- **Cuối dòng quyết định ai được lọt.** `if` không đụng gì tới giá trị; nó chỉ
  bỏ bớt phần tử. Danh sách ra ngắn hơn cuốn sổ, hoặc dài bằng, không bao giờ
  dài hơn.

Hai chuyện đó rời nhau, nên bạn đổi một bên mà bên kia đứng yên: muốn số tiền
của những khoản trên 100 nghìn thì đổi mỗi đầu dòng, muốn tên của những khoản
thuộc nhóm ăn uống thì đổi mỗi cuối dòng.

> Chỗ dễ vấp: `if` ở đây **không có** dấu hai chấm và **không có** thân thụt vào.
> Nó không phải một câu lệnh rẽ nhánh đứng riêng, nó là một mảnh nằm trong cặp
> ngoặc vuông. Gõ thêm dấu hai chấm vào đó là `SyntaxError`.

Còn một chuyện đáng nói về cái đuôi này: nó không bắt buộc. Bỏ `if` đi thì dòng
quay về đúng dòng của bài trước và lấy hết. Nói cách khác, thứ hôm nay không
thay thế thứ hôm qua — nó gắn thêm vào.
::::

::::code{#loc-khoan-tren-mot-tram}
Byte cần một danh sách tên của những khoản tiêu **quá** 100 nghìn, giữ nguyên
thứ tự ghi trong sổ.

Cuốn sổ vẫn là tám khoản quen thuộc, và nó cố tình có hai khoản nằm sát ngưỡng:
"vá lốp" tròn 100 nghìn, "xăng" ngày 22 được 105 nghìn. Một điều kiện đặt hớ tay
sẽ nhận nhầm khoản này hoặc bỏ sót khoản kia.

Điền điều kiện vào chỗ trống ở cuối dòng.

```python title=starter
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xe cộ"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xe cộ"},
    {"ten": "biếu bà", "tien": 300000, "ngay": 15, "nhom": "biếu tặng"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xe cộ"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xe cộ"},
]

dat = [khoan["ten"] for khoan in so if ___]

print(dat)
print(f"Có {len(dat)} khoản trên 100 nghìn")
```

```python title=solution
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xe cộ"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xe cộ"},
    {"ten": "biếu bà", "tien": 300000, "ngay": 15, "nhom": "biếu tặng"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xe cộ"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xe cộ"},
]

dat = [khoan["ten"] for khoan in so if khoan["tien"] > 100000]

print(dat)
print(f"Có {len(dat)} khoản trên 100 nghìn")
```

```python title=test
# So BẰNG cả danh sách chứ không kiểm từng phần: thứ tự cũng là một phần của
# câu trả lời, và hai khoản sát ngưỡng nằm ở hai phía của dấu so sánh sẽ tố
# giác ngay một điều kiện đặt hớ tay.
assert dat == ["xăng", "ăn trưa", "sửa xe", "biếu bà", "xăng"], "cuốn sổ này cho ra năm cái tên theo đúng thứ tự ghi, và 'xăng' có mặt hai lần vì tháng này Byte đổ xăng hai lần, cả hai lần đều trên 100 nghìn — nếu danh sách của bạn có thêm 'vá lốp' thì điều kiện đang nhận cả khoản tròn 100 nghìn, còn nếu thiếu 'xăng' ở cuối thì ngưỡng đang bị đặt cao hơn 105 nghìn"
assert len(dat) == 5, "trong tám khoản của cuốn sổ này có đúng năm khoản vượt 100 nghìn — ba khoản không vượt là cà phê 90 nghìn, bánh mì 15 nghìn và vá lốp tròn 100 nghìn"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở cuối dòng, ngay sau chữ `if` và trước dấu ngoặc vuông đóng. Ở chỗ đó máy đang cầm trong tay đúng **một** khoản của lượt này, và cái tên gọi nó là `khoan`.
- kind: strategy
  body: Câu cần hỏi ở mỗi khoản là "khoản này có tiêu quá 100 nghìn không". Số tiền nằm trong trường `tien` của khoản, lấy ra bằng đúng cặp ngoặc vuông thứ hai mà bạn đã dùng để bóc một tầng. Chú ý chữ **quá** trong đề: khoản tròn 100 nghìn thì chưa quá, nên dấu so sánh phải là dấu nghiêm ngặt chứ không phải dấu có gạch dưới.
- kind: one-line
  body: 'Viết `khoan["tien"] > 100000` vào chỗ trống, không thêm dấu hai chấm nào.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: ['xăng', 'ăn trưa', 'sửa xe', 'biếu bà', 'xăng']
- tier: output
  expect: Có 5 khoản trên 100 nghìn
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một dòng: đầu dòng nói lấy gì, đuôi dòng nói lấy của ai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ra được một list. Nhưng thứ mình cần là một **sổ tra cứu** tên → tiền, tra bằng
khoá: gõ `tra_cuu["sửa xe"]` là ra ngay số tiền, không phải dò lại cả danh sách.

Danh sách vừa dựng chỉ có tên, mất sạch phần tiền. Mà cái khuôn thì đã rất gần
thứ mình muốn: `for` ở giữa, `if` ở cuối, chỉ thiếu chỗ để đặt khoá và giá trị.

Đổi ngoặc vuông thành ngoặc nhọn thì máy cho ra dict?
::::

::::checkpoint{mastery=0.8}
::::
