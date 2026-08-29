---
id: nen-tang.ham-vien-gach.ham-goi-ham-khac
title: Hàm gọi hàm
summary: Trong thân một hàm gọi được hàm khác — và kết quả hàm này đưa thẳng vào làm đối số cho hàm kia.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.nested-call]
requires: [core.guard-clause, core.builtin-function, core.docstring, core.implicit-return-none, core.function-def, core.function-parameter, core.function-argument, core.function-return, core.function-call, ctrl.if, ctrl.comparison, core.floor-division, core.fstring]
concepts: [core.ham, core.tra-ve, core.gia-tri]
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
Mình không phải tự làm hết. Việc nào có người làm rồi thì mình gọi người đó.
::::

::::explain{#cat-ra-roi-ghep-lai}
Bài trước làm phẳng được cái hình thang, nhưng hàm vẫn dài. Hôm nay quán đổi
cách ghi đơn: khách chỉ nói cỡ tô, giá thì hàm tự tra; và hàm phải đưa ra sẵn
một câu chữ để dán lên hoá đơn. Viết thẳng tuột, tất cả trong một hàm, thì ra
thế này:

```python title=readonly
def tinh_hoa_don(co):
    """Dựng câu hoá đơn cho một tô, theo cỡ tô."""
    if co == "nhỏ":
        gia = 40000
    elif co == "vừa":
        gia = 45000
    else:
        gia = 55000
    thue = gia // 10
    return f"Tô {co}: {gia} đồng, thuế {thue} đồng"
```

Nó chạy đúng, và cái dài của nó có lý do rõ ràng: **nó làm ba việc khác hẳn
nhau**. Tra giá theo cỡ tô là một việc. Tính thuế trên một số tiền là việc thứ
hai. Dựng câu chữ để đưa ra cho người đọc là việc thứ ba. Ba việc ấy không
liên quan gì nhau ngoài chuyện tình cờ bị viết chung một chỗ.

Cắt ra thì gọn ngay. Mỗi việc một hàm, mỗi hàm một cái tên nói đúng việc nó
làm:

```python
def tra_gia(co):
    """Trả về giá một tô theo cỡ: nhỏ, vừa hay lớn."""
    if co == "nhỏ":
        return 40000
    if co == "vừa":
        return 45000
    return 55000


def tinh_thue(tien):
    """Trả về tiền thuế của một số tiền: một phần mười số ấy."""
    return tien // 10
```

Hai hàm này nhỏ tới mức đọc một lượt là hiểu, và mỗi hàm thử riêng được. Nhưng
cắt xong thì lại hở ra đúng câu hỏi bài trước bỏ ngỏ: **ghép lại bằng cách
nào?**

Câu trả lời ngắn tới mức dễ bỏ qua: *ở đâu viết được một lời gọi hàm thì ở đó
gọi được*. Kể cả bên trong thân một hàm khác.

Từ trước tới giờ bạn luôn gọi hàm ở ngoài cùng, sát lề trái, kiểu
`print(tra_gia("vừa"))`. Nhưng dòng `gia = tra_gia(co)` viết trong thân một
hàm cũng là một dòng gán bình thường: vế phải được tính ra một giá trị, rồi
giá trị ấy được dán lên cái tên bên trái. Máy không hề hỏi dòng ấy đang nằm
trong hàm hay ngoài hàm.

Nên hàm dựng hoá đơn chỉ còn ba dòng, và ba dòng đó đọc như một câu tiếng
Việt: tra giá, tính thuế, dựng câu.
::::

::::example{#ba-vien-gach}
Ba hàm nhỏ, một hàm gọi hai hàm kia:

```python title=readonly
def tra_gia(co):
    """Trả về giá một tô theo cỡ: nhỏ, vừa hay lớn."""
    if co == "nhỏ":
        return 40000
    if co == "vừa":
        return 45000
    return 55000


def tinh_thue(tien):
    """Trả về tiền thuế của một số tiền: một phần mười số ấy."""
    return tien // 10


def tinh_hoa_don(co):
    """Dựng câu hoá đơn cho một tô, dựa vào hai hàm ở trên."""
    gia = tra_gia(co)
    thue = tinh_thue(gia)
    return f"Tô {co}: {gia} đồng, thuế {thue} đồng"


print(tinh_hoa_don("vừa"))
print(tinh_hoa_don("nhỏ"))
```

Máy in ra:

```text
Tô vừa: 45000 đồng, thuế 4500 đồng
Tô nhỏ: 40000 đồng, thuế 4000 đồng
```

Nhìn kỹ dòng `thue = tinh_thue(gia)`. Cái tên `gia` ở đó không rơi từ trên
trời xuống: nó vừa được dòng ngay trên tạo ra, và thứ nó đang giữ chính là
**kết quả** mà `tra_gia` đưa ra. Vậy là kết quả của hàm này đã trở thành đối
số của hàm kia — chỉ có điều nó đi qua một cái tên trung gian.

Cái tên trung gian ấy bỏ được. Chỗ nào cần một giá trị thì chỗ đó nhận được
một lời gọi hàm, vì lời gọi hàm cũng cho ra một giá trị y như mọi thứ khác:

```python
thue = tinh_thue(tra_gia(co))
```

Một dòng thay hai. Đây là chỗ đáng dừng lại: `tra_gia(co)` đang nằm ở vị trí
của một **đối số**, đúng cái chỗ mà trước nay bạn chỉ viết vào đó một con số
hay một cái tên.

Giữ hay bỏ cái tên trung gian là chuyện của người đọc, không phải của máy.
Trong hàm trên thì `gia` được giữ lại, vì dòng cuối cần in nó ra. Còn khi một
giá trị chỉ đi ngang qua để vào ngay hàm kế tiếp, đặt tên cho nó nhiều khi chỉ
làm dài thêm.
::::

::::predict{#return-cua-ai commitOnce}
Byte gọi `tinh_thue` từ trong thân một hàm khác, rồi tự hỏi không biết dòng
`print` phía dưới có chạy không.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def tinh_thue(tien):
    """Trả về tiền thuế của một số tiền: một phần mười số ấy."""
    return tien // 10


def tong_phai_tra(gia):
    """Cộng thuế vào giá tô rồi đưa ra tổng phải trả."""
    thue = tinh_thue(gia)
    print("Đã tính xong thuế")
    return gia + thue


print(tong_phai_tra(45000))
```

:::opt{correct}
Hai dòng: `Đã tính xong thuế` rồi `49500`
:::

:::opt
Chỉ một dòng: `4500`
::why
Gần đúng ở chỗ bạn nhớ rất chắc câu của bài 12: `return` là dấu chấm hết, mọi
dòng nằm sau nó không bao giờ chạy. Câu ấy đúng, và bạn đang áp nó đúng lúc.

Chỗ lệch nằm ở chữ "hàm nào". `return tien // 10` chấm hết lượt chạy của
**chính hàm chứa nó**, tức là của `tinh_thue`, chứ không chạm gì tới
`tong_phai_tra`. Xong lượt của mình, `tinh_thue` giao lại giá trị cho đúng chỗ
đã gọi nó — dòng `thue = ...` — rồi biến mất. `tong_phai_tra` chạy tiếp từ
dòng ngay dưới, in câu báo, rồi mới `return` phần của nó.

Mỗi hàm có dấu chấm hết của riêng mình.
::
:::

:::opt
Chỉ một dòng: `49500` — câu `Đã tính xong thuế` nằm trong hàm nên không hiện ra
::why
Gần đúng ở chỗ bạn tính ra con số cuối cùng chính xác, và bạn cũng đang giữ
một phân biệt thật: `return` đưa giá trị về chỗ gọi, còn `print` hiện chữ lên
màn hình — hai chuyện khác nhau, đúng như bài 11 đã tách bạch.

Chỗ lệch: `print` không quan tâm nó đứng ở đâu. Dòng `print` nằm trong thân
hàm thì nó chạy vào lúc thân hàm chạy tới đó, và chữ hiện lên màn hình ngay
lúc ấy. Thân hàm không phải một cái hộp kín tiếng.

Vì lời gọi `tong_phai_tra(45000)` chạy trước, còn `print` ngoài cùng chỉ in
được sau khi đã có giá trị để in, nên câu báo bên trong xuất hiện **trước** con
số.
::
:::

:::opt
Máy báo lỗi, vì `tinh_thue` được gọi bên trong một hàm khác
::why
Gần đúng ở chỗ bạn thấy đây là một chuyện mới: từ đầu khoá tới giờ mọi lời gọi
hàm đều nằm ở ngoài cùng, sát lề trái. Thấy nó dời vào trong thân một hàm mà
sinh nghi là một phản xạ tốt.

Chỗ lệch: máy không có luật nào cấm chuyện đó. Với máy, `tinh_thue(gia)` chỉ
là một biểu thức cho ra một giá trị, y hệt `gia * 2`, và biểu thức thì viết
được ở bất cứ chỗ nào cần một giá trị — trong hàm, ngoài hàm, trong `if`, hay
ngay trong ngoặc của một lời gọi khác.
::
:::
::::

::::explain{#gach-va-tuong}
Ba hàm nhỏ ghép thành một hàm lớn, và cách ghép chỉ có hai lối, cả hai bạn vừa
thấy:

- **Qua một cái tên trung gian.** `gia = tra_gia(co)` rồi dùng `gia` ở dòng
  sau. Lối này đặt tên được cho từng bước, nên khi đoạn việc dài và mỗi bước
  đáng có một cái tên thì nó dễ đọc hơn.
- **Đưa thẳng vào làm đối số.** `tinh_thue(tra_gia(co))`. Lối này gọn, hợp với
  lúc giá trị chỉ đi ngang qua.

Cả hai lối đều dựa trên đúng một sự thật: **một lời gọi hàm là một giá trị.**
Bạn đã đối xử với `len("phở")` như một con số ngay từ bài 1 mà không nghĩ ngợi
gì; giờ thì mọi hàm bạn tự viết cũng vậy.

Và có một chỗ cần cẩn thận khi ghép: hàm được gọi phải thật sự **đưa ra** một
giá trị bằng `return`. Nếu `tra_gia` chỉ `print` giá ra màn hình rồi thôi, thì
dòng `gia = tra_gia(co)` vẫn chạy, không báo lỗi gì, và cái tên `gia` nhận về
`None` — đúng chỗ trũng mà bài 11 đã chỉ ra. Viên gạch nào cũng phải chìa ra
một mặt cho viên kế tiếp bám vào.

> Chỗ dễ vấp: gọi hàm mà quên hứng kết quả. Viết `tinh_thue(gia)` đứng một
> mình thành một dòng thì hàm vẫn chạy, vẫn tính ra con số — rồi con số ấy
> không được dán lên cái tên nào cả và mất luôn. Máy không kêu một tiếng.
::::

::::code{#ghep-ba-vien-gach}
Byte đã cắt sẵn hai viên gạch: `tra_gia` tra giá theo cỡ tô, `tinh_thue` tính
thuế trên một số tiền. Còn `tinh_hoa_don` thì đang hở hai dòng.

Hãy điền hai dòng ấy sao cho `tinh_hoa_don` dùng lại đúng hai hàm ở trên, thay
vì tự chép lại việc của chúng.

```python title=starter
def tra_gia(co):
    """Trả về giá một tô theo cỡ: nhỏ, vừa hay lớn."""
    if co == "nhỏ":
        return 40000
    if co == "vừa":
        return 45000
    return 55000


def tinh_thue(tien):
    """Trả về tiền thuế của một số tiền: một phần mười số ấy."""
    return tien // 10


def tinh_hoa_don(co):
    """Dựng câu hoá đơn cho một tô, dựa vào hai hàm ở trên."""
    gia = ___
    thue = ___
    return f"Tô {co}: {gia} đồng, thuế {thue} đồng"


print(tinh_hoa_don("vừa"))
print(tinh_hoa_don("nhỏ"))
print(tinh_hoa_don("lớn"))
```

```python title=solution
def tra_gia(co):
    """Trả về giá một tô theo cỡ: nhỏ, vừa hay lớn."""
    if co == "nhỏ":
        return 40000
    if co == "vừa":
        return 45000
    return 55000


def tinh_thue(tien):
    """Trả về tiền thuế của một số tiền: một phần mười số ấy."""
    return tien // 10


def tinh_hoa_don(co):
    """Dựng câu hoá đơn cho một tô, dựa vào hai hàm ở trên."""
    gia = tra_gia(co)
    thue = tinh_thue(gia)
    return f"Tô {co}: {gia} đồng, thuế {thue} đồng"


print(tinh_hoa_don("vừa"))
print(tinh_hoa_don("nhỏ"))
print(tinh_hoa_don("lớn"))
```

```python title=test
# Chấm trên cả ba cỡ tô, vì một cỡ thì chưa phân biệt được gì: điền một con
# số cứng vào chỗ trống cũng ra đúng câu của cỡ ấy. Ba cỡ cho ba cặp số khác
# nhau, nên chỉ hai lời gọi hàm thật mới trả lời trọn cả ba.
assert tinh_hoa_don("vừa") == "Tô vừa: 45000 đồng, thuế 4500 đồng", "tô vừa giá 45 nghìn, thuế bằng một phần mười giá ấy — hai con số này phải do tra_gia và tinh_thue đưa ra"
assert tinh_hoa_don("nhỏ") == "Tô nhỏ: 40000 đồng, thuế 4000 đồng", "tô nhỏ có giá riêng, nên dòng thứ nhất phải hỏi tra_gia theo đúng cỡ đang nhận vào chứ không giữ một con số cố định"
assert tinh_hoa_don("lớn") == "Tô lớn: 55000 đồng, thuế 5500 đồng", "tô lớn rơi vào dòng return cuối của tra_gia; thuế của nó vẫn phải là một phần mười giá, do tinh_thue tính"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở vế phải của hai dòng gán, nên mỗi chỗ cần một thứ cho ra GIÁ TRỊ. Nhìn lại hai hàm ở trên: mỗi hàm cần được đưa vào cái gì, và đưa ra cái gì?
- kind: strategy
  body: Dòng thứ nhất cần giá của tô, mà giá thì `tra_gia` biết — nó cần được đưa vào cỡ tô, và cỡ tô đang nằm sẵn trong tham số của `tinh_hoa_don`. Dòng thứ hai cần tiền thuế, mà `tinh_thue` tính thuế từ một số tiền — số tiền ấy chính là thứ dòng thứ nhất vừa đặt tên xong.
- kind: one-line
  body: Dòng thứ nhất là `gia = tra_gia(co)`, dòng thứ hai là `thue = tinh_thue(gia)`.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tô vừa: 45000 đồng, thuế 4500 đồng\nTô nhỏ: 40000 đồng, thuế 4000 đồng\nTô lớn: 55000 đồng, thuế 5500 đồng\s*$
- tier: output
  expect: Tô lớn: 55000 đồng, thuế 5500 đồng
- tier: static
  onFail: thân `tinh_hoa_don` phải GỌI hai hàm ở trên chứ không chép lại việc của chúng
  requireAst:
  - kind: uses-call, target: tra_gia, min: 1
  - kind: uses-call, target: tinh_thue, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba viên gạch, một bức tường. Viên nào hỏng thì sửa đúng viên đó thôi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte cắt nốt phần dựng câu ra thành viên gạch thứ ba:

```python
def dung_cau(thue):
    """Dựng câu báo thuế từ số tiền thuế đã tính."""
    return f"Thuế phải nộp: {thue} đồng"
```

Rồi Byte bỏ luôn cả hai cái tên trung gian, đưa thẳng kết quả hàm này vào làm
đối số hàm kia, và viết cả ba viên gạch trên đúng một dòng:

`dung_cau(tinh_thue(tra_gia("vừa")))` — ba lớp ngoặc lồng nhau.

Máy chạy hàm nào trước?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
