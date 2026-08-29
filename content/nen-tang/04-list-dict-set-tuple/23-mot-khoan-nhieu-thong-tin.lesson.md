---
id: nen-tang.list-dict-set-tuple.mot-khoan-nhieu-thong-tin
title: Một khoản, nhiều thông tin
summary: Mỗi khoản chi thành một dict nhiều trường, và cả cuốn sổ thành một danh sách các dict.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.list-of-dicts, core.nested-list-dict]
requires: [core.sorted-reverse, core.dict, core.tuple, core.tuple-immutable, core.list, core.list-index, core.list-negative-index, core.len, ctrl.for-each, core.fstring, core.output]
concepts: [core.danh-sach, core.so-tra-cuu, core.long-nhau]
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
Một khoản chi có bốn thứ đáng nhớ. Cái cặp của mình thì chỉ có hai chỗ ngồi.
::::

::::explain{#bon-thu-cho-mot-khoan}
Sếp muốn biết mỗi khoản chi vào ngày nào và thuộc nhóm gì. Thứ bạn đang cầm là
những cặp kiểu `('sửa xe', 500000)` — hai ô, hết chỗ.

Cách nhét thêm dễ nghĩ nhất là kéo dài cái cặp ra:

```python title=readonly
khoan = ("sửa xe", 500000, 11, "xăng xe")
```

Viết ra thì gọn. Đọc lại mới là chỗ khổ. Sáu tháng sau bạn mở đúng dòng này lên
và phải tự trả lời: ô số 2 là ngày hay là nhóm? `khoan[2]` đưa ra `11` — số ấy
là ngày mười một, hay mười một nghìn, hay khoản thứ mười một trong tháng? Bài 17
đã nói trước chuyện gì xảy ra khi bạn nhớ nhầm một ô: máy đưa ra đúng ô bạn hỏi,
đúng như bạn hỏi, và không kêu một tiếng nào.

Vấn đề không nằm ở chỗ cặp có ít ô. Nó nằm ở chỗ **ô thì không có tên**.

Mà chỗ chứa tra bằng tên thì bạn có từ bài 9. Cho mỗi thứ đáng nhớ một khoá.

Từ đây trở đi ta làm việc trên **một cuốn sổ khác**, không phải cuốn của hai bài
vừa rồi: đây là sổ chi tiêu tháng này của Byte, và mọi con số trong nó đều khác.
Đừng mang đáp án của bài 22 sang — cuốn sổ ấy đã đóng lại.

```python title=readonly
khoan = {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"}

print(khoan["tien"])
print(khoan["nhom"])
```

```text title=readonly
40000
ăn uống
```

`khoan["nhom"]` đọc lên là "nhóm của khoản này". Không phải nhớ, không phải đếm.
Một khoản chi bây giờ là **một dict bốn trường**.
::::

::::explain{#ca-cuon-so-thi-sao}
Một khoản đã yên chỗ. Nhưng cuốn sổ có năm khoản, và tháng sau sẽ có bốn mươi.

Chỗ chứa nhiều thứ **có thứ tự**, thêm được vào cuối, đếm được bằng `len` —
chính là `list`, thứ bạn dùng từ Realm 0. Mọi lần trước, ô của list chứa một con
số, một câu chữ, hoặc một cặp. Không có luật nào nói nó chỉ được chứa ba loại ấy.

Ô của một list chứa được **một giá trị bất kỳ** — kể cả khi giá trị ấy là cả một
dict.

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
]
```

Đọc từ ngoài vào: cặp `[ ]` ngoài cùng nói "đây là một danh sách"; mỗi dòng bên
trong là **một ô** của danh sách ấy; và mỗi ô mở ra bằng `{ }` nên nó là một
dict.

Cách gọi tên hình dạng này: **một list các dict**. Nó là chỗ chứa mà gần như mọi
chương trình có dữ liệu thật đều dùng tới — một bảng, mỗi dòng một bản ghi, mỗi
bản ghi vài trường có tên.

Và nó vá đúng vết thương của bài 8: hai dãy song song thì máy không biết chúng
dính nhau, còn ở đây tên với tiền với ngày với nhóm nằm chung **một** dict, đi
đâu cũng đi cùng nhau.
::::

::::example{#so-nam-khoan}
Cuốn sổ tháng này của Byte, tính tới hôm nay là năm khoản — tháng chưa hết nên
nó còn dài thêm được:

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
]

print(f"Sổ đang có {len(so)} khoản")
print("Khoản đầu sổ:", so[0])
print("Khoản cuối sổ:", so[-1])

print("Cả sổ, mỗi khoản một dòng:")
for khoan in so:
    print(khoan)
```

Máy in ra:

```text title=readonly
Sổ đang có 5 khoản
Khoản đầu sổ: {'ten': 'cà phê', 'tien': 25000, 'ngay': 2, 'nhom': 'ăn uống'}
Khoản cuối sổ: {'ten': 'bánh mì', 'tien': 15000, 'ngay': 14, 'nhom': 'ăn uống'}
Cả sổ, mỗi khoản một dòng:
{'ten': 'cà phê', 'tien': 25000, 'ngay': 2, 'nhom': 'ăn uống'}
{'ten': 'xăng', 'tien': 60000, 'ngay': 5, 'nhom': 'xăng xe'}
{'ten': 'bún bò', 'tien': 40000, 'ngay': 8, 'nhom': 'ăn uống'}
{'ten': 'vá lốp', 'tien': 30000, 'ngay': 11, 'nhom': 'xăng xe'}
{'ten': 'bánh mì', 'tien': 15000, 'ngay': 14, 'nhom': 'ăn uống'}
```

Ba thứ đáng để ý:

- `len(so)` cho `5` chứ không cho `20`. Cuốn sổ có năm **ô**, và mỗi ô là một
  khoản — chuyện bên trong mỗi khoản có bốn trường không liên quan gì tới độ dài
  của danh sách.
- `so[0]` và `so[-1]` vẫn chạy y như mọi list bạn từng dùng: chỗ đứng đếm từ 0,
  `-1` là ô cuối. Thứ chúng đưa ra bây giờ là cả một dict.
- `for khoan in so:` vẫn lấy lần lượt từng ô, và mỗi lượt cái tên `khoan` giữ
  trọn một dict — không phải một con số, không phải một cái tên món.

Máy in dict ra kèm cả cặp `{ }` lẫn dấu nháy. Đó là cách Python **cho bạn xem**
một dict, không phải cách bạn phải gõ nó vào: bạn vẫn viết nháy kép như thường,
máy in lại bằng nháy đơn.
::::

::::predict{#doan-o-thu-ba commitOnce}
Vẫn cuốn sổ năm khoản ở trên. Bây giờ hỏi nó đúng một câu.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra gì?

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
]

print(so[2])
```

:::opt{correct}
`{'ten': 'bún bò', 'tien': 40000, 'ngay': 8, 'nhom': 'ăn uống'}`
:::

:::opt
`40000`
::why
Gần đúng ở chỗ bạn tìm đúng khoản: ô số 2 của cuốn sổ này quả thật là bát bún bò
bốn mươi nghìn, và bạn đếm từ 0 không sai một nhịp.

Chỗ lệch là ở chuyện **ô chứa gì**. Trong mọi cuốn sổ trước bài này, một ô của
list chứa đúng một con số hoặc đúng một cặp, nên `so[2]` cho ra một con số là
phản xạ rất tự nhiên. Từ bài này trở đi ô của sổ chứa cả một dict bốn trường,
nên `so[2]` đưa ra trọn cái dict ấy. Muốn riêng con số tiền thì còn phải nói
thêm một câu nữa — và đó đúng là câu hỏi bỏ ngỏ cuối bài này.
::
:::

:::opt
`bún bò`
::why
Gần đúng ở chỗ bạn cũng chỉ trúng khoản thứ ba, và bạn chọn ra thứ dễ đọc nhất
trong khoản ấy — cái tên.

Chỗ lệch: `print` không chọn giúp bạn trường nào cả. Nó in ra **nguyên** thứ mà
`so[2]` đưa cho nó, và thứ ấy là cả dict. Trong dict, `"bún bò"` chỉ là giá trị
của một khoá trong bốn khoá; ba khoá kia không có lý do gì để bị bỏ lại.
::
:::

:::opt
`{'ten': 'xăng', 'tien': 60000, 'ngay': 5, 'nhom': 'xăng xe'}`
::why
Gần đúng ở chỗ bạn nắm chắc phần khó hơn: `so[2]` đưa ra **cả một dict**, không
phải một trường lẻ. Hình dạng câu trả lời của bạn hoàn toàn đúng.

Chỗ lệch chỉ là một ô. Bạn đếm khoản xăng là khoản số 2 vì nó là khoản thứ hai
trong sổ — cách đếm của đời thường. Máy đếm từ 0, nên cà phê là 0, xăng là 1, và
bún bò mới là 2. Đây là chỗ vấp cũ từ Realm 0, và nó không biến mất khi ô của
list đổi từ con số sang dict.
::
:::
::::

::::code{#viet-not-mot-khoan}
Byte đang chép cuốn sổ sang hình dạng mới thì bị gọi đi mất, và cuốn sổ hụt đúng
một khoản ở giữa. Khoản còn thiếu là:

> Bát **bún bò** ngày **8**, hết **40000** đồng, thuộc nhóm **ăn uống**.

Viết nó thành một ô của danh sách, đủ bốn trường, dùng đúng bốn khoá mà những
khoản khác đang dùng: `ten`, `tien`, `ngay`, `nhom`.

```python title=starter
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    ___,
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
]

print(f"Sổ đang có {len(so)} khoản")
print("Khoản thứ ba:", so[2])
print("Khoản cuối sổ:", so[-1])
```

```python title=solution
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
]

print(f"Sổ đang có {len(so)} khoản")
print("Khoản thứ ba:", so[2])
print("Khoản cuối sổ:", so[-1])
```

```python title=test
# Ba phép kiểm hỏi ba chuyện khác nhau. Cái đầu hỏi cuốn sổ có đúng năm ô không.
# Cái giữa hỏi thẳng vào ô vừa điền: đủ bốn khoá, và giá trị từng khoá đúng như
# đề bài ghi — so bằng dấu `==` nên thứ tự bạn viết bốn khoá ra sao cũng được.
# Cái cuối canh chừng ô kế bên không bị xô lệch.
assert len(so) == 5, "cuốn sổ tháng này ghi năm khoản: cà phê, xăng, bún bò, vá lốp, bánh mì — ô ở giữa phải là một khoản chứ không phải chỗ trống"
assert so[2] == {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"}, "ô số 2 phải là bát bún bò: ten là 'bún bò', tien là 40000, ngay là 8, nhom là 'ăn uống' — thiếu một khoá hoặc gõ sai một khoá thì ô này không khớp"
assert so[3] == {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"}, "ô số 3 vẫn phải là khoản vá lốp 30000 đồng ngày 11 — nếu nó trượt sang khoản khác thì ô vừa điền đã chiếm nhầm chỗ"
```

:::hints
- kind: attention
  body: Chỗ trống là một **ô** của danh sách, nên thứ điền vào phải cùng hình dạng với bốn ô đang nằm quanh nó — nhìn ô ngay trên và ô ngay dưới rồi làm theo đúng khuôn ấy. Dấu phẩy cuối dòng đã có sẵn, bạn không phải thêm.
- kind: strategy
  body: Một khoản là một dict bốn trường, mở bằng `{` và đóng bằng `}`. Bốn khoá viết trong nháy kép và phải trùng từng chữ với các ô khác — `ten`, `tien`, `ngay`, `nhom`. Giá trị của `tien` và `ngay` là số nên viết không nháy; giá trị của `ten` và `nhom` là chữ nên viết trong nháy.
- kind: one-line
  body: 'Viết `{"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"}` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  # Dòng GIỮA không neo thứ tự bốn khoá, vì `print` in dict theo thứ tự CHÈN
  # còn khối test lại hứa thẳng "thứ tự bạn viết bốn khoá ra sao cũng được".
  # Neo cứng nó là đánh trượt một lời giải đúng — và đánh trượt bằng đúng lời
  # hứa bài vừa đưa ra. Hai dòng ngoài không do người học sinh ra nên neo được.
  expect: ^Sổ đang có 5 khoản\n.*\nKhoản cuối sổ: \{'ten': 'bánh mì', 'tien': 15000, 'ngay': 14, 'nhom': 'ăn uống'\}\s*$
- tier: output
  expect: Khoản cuối sổ: {'ten': 'bánh mì', 'tien': 15000, 'ngay': 14, 'nhom': 'ăn uống'}
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn thứ trong một khoản, thứ nào cũng có tên riêng. Không phải đếm ô nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cuốn sổ đã đúng hình dạng, và bạn vừa in `so[2]` ra để xem khoản thứ ba. Nhưng
câu sếp hỏi không phải "cho xem cả khoản" — sếp hỏi **số tiền** của khoản thứ ba.

Lấy số tiền của khoản thứ ba: `so[2]` cho ra nguyên một dict.

```text title=readonly
{'ten': 'bún bò', 'tien': 40000, 'ngay': 8, 'nhom': 'ăn uống'}
```

Rồi sao nữa? Bạn biết cách hỏi một dict lấy một trường — `khoan["tien"]`, làm từ
bài 9. Nhưng ở đây không có cái tên `khoan` nào cả; thứ bạn có là `so[2]`.

Đặt thêm một cái tên trung gian rồi hỏi làm hai bước, hay hỏi thẳng trong một
câu? Và nếu hỏi thẳng thì viết ra sao — hai cặp ngoặc đứng cạnh nhau ư?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
