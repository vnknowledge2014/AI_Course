---
id: nen-tang.chuong-trinh-that.tach-mot-dong-thanh-manh
title: Tách một dòng thành nhiều mảnh
summary: "`.split(\",\")` cắt chuỗi tại mỗi dấu phân cách và đưa về một list các mảnh — nên tên khoản dài ngắn thế nào cũng lấy được riêng phần tiền."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.string-split]
requires: [core.strip-newline, io.readlines, core.with-open, core.file-write, core.string-strip, core.string-slice, core.string-method, core.string-literal, core.string-concat, core.len, core.list, core.list-index, core.list-append, core.variable, core.assignment, core.output, ctrl.for-each]
concepts: [core.chuoi, core.danh-sach, core.dong-van-ban]
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
Dấu phẩy đứng chỗ nào cũng được. Miễn là bạn bảo máy cắt tại dấu phẩy.
::::

::::explain{#cat-theo-cho-dung-thi-truot}
Bài trước để lại một cuốn sổ sạch: bốn chuỗi, không thừa một ký tự. Và để lại
một việc chưa xong — lấy riêng phần tiền, cái phần nằm sau dấu phẩy.

Ở T1.1 bạn đã có một cách cắt chuỗi: nói cho máy biết cắt từ **chỗ đứng** nào.
Thử ngay trên dòng đầu:

```python title=readonly
print("cà phê,25000"[7:])
```

Ra đúng `25000`. Dấu phẩy của dòng ấy đứng ở ô số 6, nên từ ô số 7 trở đi là
phần tiền. Xong một dòng.

Nhưng cuốn sổ có bốn dòng, và cùng một lệnh ấy đem sang dòng khác thì hỏng:

```python title=readonly
print("bánh mì,15000"[7:])
```

Ra `,15000` — dính luôn dấu phẩy. Lý do không có gì bí ẩn: `"cà phê"` dài sáu
ký tự còn `"bánh mì"` dài bảy, nên dấu phẩy của hai dòng không đứng cùng một
chỗ. Và tên khoản thì Byte gõ tay, mỗi khoản một độ dài; ngày mai thêm một
khoản tên `"gửi xe"` hay `"sửa điện thoại"` là con số trong ngoặc lại sai.

Chỗ hụt nằm ở chỗ này: cắt theo **chỗ đứng** đòi bạn biết trước dấu phẩy nằm ở
ô số mấy. Nhưng thứ bạn thật sự biết không phải con số ấy — thứ bạn biết là
*cắt tại dấu phẩy*. Bạn cần một cách nói thẳng ra điều mình biết.

Chuỗi có sẵn một phương thức làm đúng vậy:

```python title=readonly
manh = "cà phê,25000".split(",")
```

*Split* trong tiếng Anh là chẻ, tách ra. Bạn đưa vào **dấu phân cách** — ở đây
là chuỗi một ký tự `","` — và máy đi dọc chuỗi, gặp dấu ấy thì chẻ.

Ba điều đáng nhớ về thứ nhận được:

- Nó là một **list các mảnh**, nên `len`, ngoặc vuông và `for` đều dùng được
  ngay, y như với list dòng của bài 7.
- **Dấu phân cách không đi theo mảnh nào.** Nó là chỗ cắt, và chỗ cắt thì bị
  tiêu đi, không nằm lại trong mảnh trái cũng không nằm trong mảnh phải.
- Số mảnh không phụ thuộc độ dài tên khoản. Một dòng có một dấu phẩy thì chẻ ra
  hai mảnh, dù tên khoản dài bao nhiêu.
::::

::::example{#hai-manh-moi-dong}
Cùng một lệnh chẻ, đem chạy trên ba dòng có độ dài tên khác nhau:

```python title=readonly
dong = "cà phê,25000"
manh = dong.split(",")

print(manh)
print(len(manh))
print(manh[0])
print(manh[1])

print("bánh mì,15000".split(","))
print("vá lốp,100000".split(","))
```

Màn hình:

```text title=readonly
['cà phê', '25000']
2
cà phê
25000
['bánh mì', '15000']
['vá lốp', '100000']
```

Bốn chỗ đáng dừng lại nhìn:

- **Hai mảnh, không phải ba.** Một dấu phẩy chẻ chuỗi làm hai khúc. Dấu phẩy
  không tự nó thành một mảnh — nó biến mất, và chỗ nó đứng thành ranh giới.
- **`manh[0]` là tên, `manh[1]` là tiền.** Đếm từ 0 như mọi list khác, nên mảnh
  đầu tiên là mảnh mang chỉ số 0.
- **Ba dòng, cùng một lệnh, đều ra hai mảnh đúng chỗ.** `"cà phê"` sáu ký tự,
  `"bánh mì"` bảy, `"vá lốp"` sáu — không con số nào của bạn phải đổi theo. Đây
  đúng là chỗ cách cắt cũ chịu thua.
- **Dòng đầu in ra có dấu nháy quanh cả hai mảnh.** Đó là cách Python cho bạn
  xem một list các chuỗi. Ghi nhớ chi tiết ấy; cuối bài ta quay lại.
::::

::::predict{#doan-hai-manh commitOnce}
Byte chẻ dòng cuối sổ rồi hỏi máy hai câu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
manh = "bánh mì,15000".split(",")

print(len(manh))
print(manh[1])
```

:::opt{correct}
`2` rồi `15000`
:::

:::opt
`2` rồi `bánh mì`
::why
Gần đúng ở chỗ bạn đếm số mảnh chính xác: một dấu phẩy chẻ chuỗi làm hai khúc,
và `2` là con số đúng.

Chỗ lệch nằm ở chỗ đứng. Ngoặc vuông đếm từ `0`, đúng như R0·34 đã dặn, nên mảnh
đầu tiên — `"bánh mì"` — mang chỉ số `0`, còn `manh[1]` là mảnh **thứ hai**, tức
phần tiền. Chỗ này đáng để dừng lại một nhịp: cả bài sau này sẽ lấy tiền bằng
`manh[1]`, nên nhớ nhầm một ô là mọi khoản đều ra tên thay vì ra số.
::
:::

:::opt
`2` rồi `,15000`
::why
Gần đúng ở chỗ bạn cảnh giác đúng thứ vừa gây phiền ngay đầu bài: cắt theo chỗ
đứng đã để dấu phẩy dính lại trong `",15000"`, nên nghi ngờ nó dính thêm lần nữa
là phản xạ hợp lý.

Chỗ lệch nằm ở vai của dấu phẩy trong hai cách cắt. Cắt theo chỗ đứng thì dấu
phẩy chỉ là một ký tự bình thường như mọi ký tự khác, nằm ở đâu thì ở lại đó.
Còn khi chẻ, dấu phẩy là **chỗ cắt** — nó bị tiêu đi cùng nhát cắt, nên không
mảnh nào giữ nó lại.
::
:::

:::opt
`3` rồi `15000`
::why
Gần đúng ở chỗ bạn đang đếm rất thật thà những gì có trong chuỗi: một tên khoản,
một dấu phẩy, một số tiền — ba thứ, nên đoán ba mảnh là suy luận thẳng.

Chỗ lệch: dấu phẩy không được đếm là một mảnh, vì nó là nhát cắt chứ không phải
thứ bị cắt. Hình dung một sợi dây bị cắt một nhát ở giữa: bạn được hai khúc dây,
còn nhát cắt thì không phải khúc thứ ba. Vế thứ hai bạn đoán trúng — `manh[1]`
đúng là `15000`.
::
:::
::::

::::code{#tach-ca-cuon-so}
Cuốn sổ bốn khoản của Byte, dòng thứ hai vẫn dính dấu cách gõ vội ở đầu như bài
trước. Việc của bạn: đi qua từng dòng, xén sạch hai đầu rồi **chẻ tại dấu
phẩy**, cất mỗi khoản thành một `dict` hai khoá `"ten"` và `"tien"`, tất cả vào **cùng
một** list — đúng cách T1.4 bài 8 đã chốt, để tên và tiền không bao giờ lệch
nhau được.

Bốn dòng `f.write` ở đầu chỉ để dựng lại cuốn sổ cho bài chạy được một mình.

```python title=starter
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write(" bún bò,40000\n")
    f.write("bánh mì,15000\n")
    f.write("vá lốp,100000\n")

with open("so.txt", "r") as f:
    cac_dong = f.readlines()

cac_khoan = []
for dong in cac_dong:
    manh = dong.strip().___(",")
    cac_khoan.append({"ten": manh[0], "tien": manh[1]})

print(cac_khoan[0])
print(cac_khoan[2]["tien"])
print(cac_khoan[0]["tien"] + cac_khoan[1]["tien"])
```

```python title=solution
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write(" bún bò,40000\n")
    f.write("bánh mì,15000\n")
    f.write("vá lốp,100000\n")

with open("so.txt", "r") as f:
    cac_dong = f.readlines()

cac_khoan = []
for dong in cac_dong:
    manh = dong.strip().split(",")
    cac_khoan.append({"ten": manh[0], "tien": manh[1]})

print(cac_khoan[0])
print(cac_khoan[2]["tien"])
print(cac_khoan[0]["tien"] + cac_khoan[1]["tien"])
```

```python title=test
# Bốn tên khoản dài ngắn khác nhau (6, 6, 7, 6 ký tự), nên một cách cắt theo
# chỗ đứng cố định sẽ đúng ở vài dòng và sai ở dòng "bánh mì" — câu kiểm đầu
# tiên bắt được ngay.
assert cac_khoan == [
    {"ten": "cà phê", "tien": "25000"},
    {"ten": "bún bò", "tien": "40000"},
    {"ten": "bánh mì", "tien": "15000"},
    {"ten": "vá lốp", "tien": "100000"},
], f"mỗi khoản phải thành một dict hai khoá, tên là mảnh TRƯỚC dấu phẩy và tiền là mảnh SAU, và dấu phẩy không đi theo mảnh nào; đang có {cac_khoan}"
assert cac_khoan[0]["tien"] + cac_khoan[1]["tien"] == "2500040000", "hai mảnh tiền của cà phê và bún bò vẫn đang là chữ, nên dấu + dán chúng thành chuỗi '2500040000' — nếu câu này trượt thì hai mảnh ấy chưa phải thứ lấy ra từ sổ"
```

:::hints
- kind: attention
  body: Chỗ trống đứng sau `dong.strip()`, và ngay sau chỗ trống đã có sẵn `(",")` — tức là dấu phân cách bạn được cho trước, phần phải gọi tên là **việc** đem dấu ấy đi làm. Việc ấy phải cho ra một thứ mà dòng dưới lấy được `manh[0]` và `manh[1]`.
- kind: strategy
  body: Bạn cần chẻ chuỗi tại mỗi dấu phẩy và nhận về một list các mảnh. Tên phương thức là từ tiếng Anh nghĩa là chẻ, tách ra — viết thường, không dấu gạch. Nối nó vào ngay sau `.strip()` được, vì `.strip()` đã trả về một chuỗi và chuỗi thì chẻ được.
- kind: one-line
  body: "Viết `split` vào chỗ trống, để dòng ấy thành `manh = dong.strip().split(\",\")`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^\{'ten': 'cà phê', 'tien': '25000'\}\n15000\n2500040000\s*$
- tier: output
  expect: "{'ten': 'cà phê', 'tien': '25000'}"
:::
::::

::::byte{trigger=success mood=curious pose=point-editor}
Mỗi khoản một dòng sổ gọn ghẽ. Nhưng dòng in cuối cùng trông không giống tổng tiền.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cuốn sổ đã tách xong: `manh[0]` là tên khoản, `manh[1]` là phần tiền. Với dòng
đầu, `manh[1]` là `"25000"` — đúng con số Byte đã ghi.

Nên Byte làm chuyện tự nhiên nhất: cộng hai khoản đầu lại.

```python title=readonly
print(cac_khoan[0]["tien"] + cac_khoan[1]["tien"])
```

Máy in ra `2500040000`.

Không lỗi, không traceback, không một lời phàn nàn nào. Chỉ là một con số dài
mười chữ số mà không ai tiêu ngần ấy tiền cho cà phê với bún bò.

Nhìn lại dòng sổ vừa in ra thì thấy một chi tiết đã đứng đó từ đầu: cả
`'cà phê'` lẫn `'25000'` đều nằm trong dấu nháy như nhau. Máy đang xếp chúng vào
cùng một loại.

`"25000" + "40000"` ra `"2500040000"`. Vì sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
