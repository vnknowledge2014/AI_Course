---
id: nen-tang.list-dict-set-tuple.ro-khong-chua-hai-lan
title: Cái rổ không chứa hai lần
summary: "`set` là chỗ chứa mà mỗi giá trị chỉ nằm được đúng một lần — bỏ vào thứ đã có thì không có gì đổi, và `set(danh_sach)` lọc trùng trong một bước."
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.set]
requires: [core.dict-accumulator, core.list-membership, core.dict-nested-index, core.list-of-dicts, core.list-append, core.list, core.len, ctrl.for-each, ctrl.if, core.fstring]
concepts: [core.cho-chua, core.tap-hop, ctrl.lap]
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
Có loại chỗ chứa mà bạn không phải canh trùng. Nó tự lo.
::::

::::explain{#nguoi-gac-cua-lam-tay}
Câu hỏi bài trước để lại: có chỗ chứa nào **tự nó** không nhận hai lần không?

Trước khi trả lời, hãy nhìn kỹ đoạn lọc trùng làm tay một lần nữa:

```python title=readonly
ds = []
for khoan in so:
    nhom = khoan["nhom"]
    if nhom not in ds:
        ds.append(nhom)
```

Danh sách `ds` không hề biết nó phải giữ mình khỏi trùng. Nó nhận mọi thứ được
`.append` vào, không hỏi han gì. Thứ giữ cho nó sạch là **câu `if` bạn viết
ra** — một người gác cửa đứng bên ngoài chỗ chứa.

Người gác cửa ấy tốn công. Câu `nhom not in ds` bắt máy dò từng ô của `ds` từ
đầu cho tới khi gặp, hoặc tới hết danh sách nếu không gặp — đúng sự thật bạn
đã cất đi từ bài hỏi `in` trên một danh sách. Sổ bốn mươi dòng là bốn mươi
lượt dò, mỗi lượt lại dò trên một danh sách đang dài dần ra.

Và người gác cửa ấy có thể quên. Ai đó sửa chương trình về sau, thêm một dòng
`ds.append(...)` ở chỗ khác mà không chép theo câu `if`, thì danh sách có hai
tên giống nhau — không lỗi, không cảnh báo. Luật "không trùng" chỉ nằm trong
câu `if`, không nằm trong bản thân chỗ chứa.

Python có sẵn một chỗ chứa mang luật ấy **trong chính nó**. Tên nó là `set`.
Từ đây gọi nó là **cái rổ**, cho dễ hình dung: bạn bỏ đồ vào rổ, và cái rổ
không có ô nào để xếp hai quả cam giống hệt nhau nằm cạnh nhau — bỏ thêm quả
cam thứ hai y hệt thì rổ vẫn đúng chừng ấy đồ.
::::

::::example{#tao-mot-cai-ro}
Rổ rỗng viết là `set()`, và bỏ một thứ vào rổ là `.add(...)`:

```python title=readonly
ro = set()

ro.add("ăn uống")
ro.add("xăng xe")
ro.add("ăn uống")
ro.add("học phí")

print(len(ro))
print(ro)
```

Máy in ra:

```text title=readonly
3
{'xăng xe', 'ăn uống', 'học phí'}
```

Bốn lần `.add`, rổ giữ ba tên. Lần gọi thứ ba bỏ vào `"ăn uống"` — thứ rổ đã
có — và nó không báo lỗi, không kêu ca, cũng không thêm gì. Sau lời gọi ấy rổ
vẫn y như trước.

Bốn chi tiết cần nói rõ ngay, vì cả bốn đều dễ vấp:

- **Rổ rỗng phải viết `set()`, không viết `{}`.** Cặp ngoặc nhọn rỗng đã có
  chủ: nó là sổ tra cứu rỗng của bài trước.
- **Rổ có sẵn đồ thì viết được thẳng trong ngoặc nhọn**, ví dụ
  `{"ăn uống", "xăng xe"}` — bên trong chỉ có các giá trị, không có dấu hai chấm
  nào, nên máy không nhầm nó với sổ tra cứu. Đó cũng là cách máy in một cái rổ
  ra màn hình, như dòng thứ hai ở trên.
- **`.add` sửa thẳng cái rổ và không đưa lại gì.** Nó cư xử như `.append` trên
  danh sách, chứ không như các phương thức chuỗi — chuỗi không sửa được nên
  phương thức của nó phải trả về một chuỗi mới, còn rổ thì sửa được tại chỗ.
  Viết `ro = ro.add("cà phê")` là mất trắng cái rổ.
- **Dòng thứ hai in ra không theo thứ tự bạn bỏ vào.** Ở đây `"xăng xe"` hiện
  lên trước `"ăn uống"`, dù nó vào rổ sau. Chạy trên máy bạn, ba cái tên ấy
  còn có thể xếp theo một thứ tự khác nữa. Giữ chi tiết này trong đầu — cuối
  bài ta quay lại.
::::

::::predict{#bo-vao-thu-da-co commitOnce}
Byte bỏ vào rổ ba lần, trong đó có một tên lặp lại.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra con số nào?

```python title=readonly
ro = set()

ro.add("ăn uống")
ro.add("xăng xe")
ro.add("ăn uống")

print(len(ro))
```

:::opt{correct}
2
:::

:::opt
3
::why
Gần đúng ở chỗ bạn đếm không sai một lời gọi nào: `.add` được gọi đúng ba lần,
và với `.append` trên danh sách thì ba lần gọi cho ra ba ô — mỗi lời gọi một
chỗ mới, kể cả khi hai ô mang cùng một giá trị.

Chỗ lệch là cái rổ không cư xử như danh sách. Nó không có ô nào được đánh số
để mà xếp thêm. Lần gọi thứ ba đưa vào một giá trị rổ đang có, nên sau lời gọi
ấy rổ không khác gì trước đó: ba lời gọi, hai giá trị khác nhau, `len` ra 2.
::
:::

:::opt
Máy báo lỗi ở lời gọi thứ ba, vì rổ không nhận hai lần
::why
Gần đúng ở chỗ bạn suy ra rất hợp lý từ đúng tính chất của cái rổ: đã "không
chứa hai lần" thì bỏ vào lần hai phải bị chặn, mà bị chặn thì máy hay kêu lên
— y như `KeyError` khi tra một khoá lạ.

Chỗ lệch nằm ở chữ *chặn*. `.add` một giá trị đã có là chuyện **hợp lệ**, chỉ
là nó không làm gì cả. Cái rổ được dựng ra để bạn khỏi phải hỏi trước; nếu nó
báo lỗi thì bạn lại phải viết đúng câu `if` gác cửa mà bài này đang bỏ đi.
::
:::

:::opt
0, vì `.add` không đưa lại gì mà mình cũng không gán lại
::why
Gần đúng ở chỗ bạn nhớ một luật thật và nhớ chính xác: có những phương thức
đứng một mình thì mất trắng, phải gán lại mới giữ được kết quả. Chuỗi đúng là
như vậy.

Chỗ lệch nằm ở lý do vì sao chuỗi phải như vậy: chuỗi **không sửa được**, nên
phương thức của nó buộc phải nặn ra một chuỗi mới rồi đưa cho bạn. Cái rổ thì
sửa được tại chỗ, y như danh sách — `.add` đi thẳng vào rổ, chẳng có gì để đưa
lại. Đó cũng là lý do `ro = ro.add(...)` là một dòng làm hỏng việc.
::
:::
::::

::::explain{#loc-trung-trong-mot-buoc}
Cái rổ còn cho bạn một đường tắt. Đưa cho `set(...)` một danh sách, nó dựng
ngay một cái rổ chứa đúng những giá trị khác nhau có trong danh sách ấy:

```python title=readonly
nhom_moi_dong = ["ăn uống", "xăng xe", "ăn uống", "học phí", "ăn uống", "xăng xe"]

ro = set(nhom_moi_dong)

print(len(ro))
```

Sáu tên đi vào, `3` đi ra. Trong danh sách, `"ăn uống"` xuất hiện ba lần,
`"xăng xe"` hai lần, `"học phí"` một lần; cộng lại 3 + 2 + 1 = 6 chỗ, nhưng
chỉ có ba giá trị khác nhau. Cả đoạn lọc trùng năm dòng của bài trước rút
xuống còn một dòng, và câu `if` gác cửa biến mất — vì luật "không trùng" giờ nằm trong chỗ chứa chứ không nằm trong tay bạn.

Còn "giống nhau" ở đây nghĩa là gì? Đúng nghĩa mà `==` đã có từ đầu, không có
nghĩa nào mới. Hai chuỗi bằng nhau thì rổ coi là một; `"Ăn uống"` và
`"ăn uống"` **khác** nhau — chữ hoa với chữ thường là hai ký tự khác nhau, nên
rổ giữ cả hai. Cái rổ không dọn dẹp dữ liệu bẩn hộ bạn; nó chỉ hứa đúng một
việc, là không giữ hai bản của cùng một giá trị.

> Chỗ dễ vấp: `set(nhom_moi_dong)` **không** đụng gì tới `nhom_moi_dong`. Nó
> dựng một chỗ chứa mới và để danh sách cũ nguyên vẹn, vẫn đủ sáu ô, vẫn đúng
> thứ tự cũ — giống hệt cách lát cắt cho ra một danh sách mới ở đầu mạch này.
::::

::::code{#nhung-nhom-da-chi}
Cuốn sổ sáu khoản của bài trước, và câu hỏi của sếp: **tháng này chi vào những
nhóm nào?**

Khung dưới đã có sẵn cuốn sổ, một cái rổ rỗng, vòng duyệt và dòng báo cáo. Còn
thiếu đúng dòng bỏ tên nhóm vào rổ.

Đường tắt `set(danh_sach)` không dùng thẳng được ở đây: mỗi phần tử của `so`
là cả một khoản nhiều trường, còn thứ ta muốn gom chỉ là một trường trong đó.

```python title=starter
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
    {"ten": "vở", "tien": 20000, "ngay": 17, "nhom": "học phí"},
]

ro = set()

for khoan in so:
    ___

print(f"Tháng này chi vào {len(ro)} nhóm")
```

```python title=solution
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
    {"ten": "vở", "tien": 20000, "ngay": 17, "nhom": "học phí"},
]

ro = set()

for khoan in so:
    ro.add(khoan["nhom"])

print(f"Tháng này chi vào {len(ro)} nhóm")
```

```python title=test
# Dòng đầu vỡ khi rổ không nhận được gì (còn rỗng), và cũng vỡ khi bạn bỏ
# nhầm trường khác vào: sáu cái `ten` trong sổ này đều khác nhau, nên rổ sẽ
# giữ sáu tên chứ không phải ba.
# Ba dòng sau đọc đích danh từng nhóm, để một cái rổ tình cờ có ba thứ gì
# đó khác vẫn không qua được.
assert len(ro) == 3, "sáu khoản trong cuốn sổ này chỉ thuộc ba nhóm khác nhau, nên cái rổ phải giữ đúng ba tên"
assert "ăn uống" in ro, "ba khoản cà phê, bún bò và bánh mì trong sổ này đều mang nhóm ăn uống, nên tên nhóm ấy phải có trong rổ"
assert "xăng xe" in ro, "hai khoản xăng và vá lốp trong sổ này mang nhóm xăng xe, nên tên nhóm ấy phải có trong rổ"
assert "học phí" in ro, "khoản mua vở ở dòng cuối sổ mang nhóm học phí, nên tên nhóm ấy phải có trong rổ"
```

:::hints
- kind: attention
  body: Chỗ trống là dòng duy nhất nằm trong thân vòng, nên nó cũng là dòng duy nhất chạm được vào `ro`. Ở mỗi lượt bạn đang cầm `khoan` — cả một khoản nhiều trường — mà thứ cần bỏ vào rổ chỉ là một trường trong đó.
- kind: strategy
  body: Hai việc ghép lại thành một dòng. Việc thứ nhất là bóc đúng trường nhóm ra khỏi khoản đang xét, bằng đúng lối hai lớp ngoặc bạn đã dùng ở bài trước nữa. Việc thứ hai là bỏ giá trị vừa bóc vào rổ — cái rổ tự lo phần trùng, nên không cần câu `if` nào canh cửa cả.
- kind: one-line
  body: 'Viết `ro.add(khoan["nhom"])` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với thân vòng.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  expect: Tháng này chi vào 3 nhóm
- tier: static
  onFail: dòng bạn điền phải lấy tên nhóm ra TỪ khoản đang duyệt, không được gõ sẵn ba tên nhóm vào rổ
  requireAst:
  # Khung không ĐỌC `khoan` ở đâu cả — dòng `for` chỉ đặt tên ấy — nên chỉ
  # cần một lần đọc là đủ để phân biệt lời giải thật với một cái rổ gõ cứng.
  - kind: uses-name, target: khoan, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba tên, sáu lần bỏ vào. Mình không phải hỏi trước lần nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Quay lại chi tiết đã hẹn ở đầu bài. Bỏ vào rổ theo thứ tự "ăn uống", "xăng xe",
"học phí", rồi in rổ ra, thì thứ tự hiện lên **khác** lúc bỏ vào. Sổ tra cứu
ở bài trước thì không như thế: duyệt một `dict` là đi đúng theo thứ tự khoá
được thêm vào, lần nào cũng vậy.

Nghĩa là cái rổ đang thiếu một thứ mà mọi chỗ chứa bạn từng gặp đều có.

Thử hỏi thẳng nó xem. Danh sách có `ds[0]` cho ô đầu, chuỗi có `ten[0]` cho ký
tự đầu, tuple có `cap[0]` cho ô đầu.

Gõ `ro[0]` để lấy phần tử đầu của rổ — máy sẽ nói gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
