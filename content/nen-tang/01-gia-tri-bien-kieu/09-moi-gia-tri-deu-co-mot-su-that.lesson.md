---
id: nen-tang.gia-tri-bien-kieu.moi-gia-tri-deu-co-mot-su-that
title: Mọi giá trị đều có một sự thật
summary: Đặt trần một giá trị vào `if` thì máy vẫn có câu trả lời đúng/sai — vì giá trị nào cũng quy được về `True` hoặc `False`.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.truthiness]
requires: [core.float-cast, core.don-vi-nho-nhat, core.boolean, ctrl.if, ctrl.else, ctrl.comparison, core.input-returns-str, core.value-error, core.function-def, core.function-parameter, core.function-return, core.function-call, ctrl.block-indent]
concepts: [core.dung-sai, core.kieu-gia-tri]
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
Bạn đưa mình thứ gì mình cũng hỏi được nó một câu: có, hay không?
::::

::::explain{#nguoi-gac-cong-chi-hoi-mot-cau}
Bài trước để lại một dòng lạ:

```python
if o_tien:
```

Sau chữ `if` không có phép so sánh nào. Chỉ có một cái tên đang giữ một chuỗi.
Vậy mà dòng ấy chạy được — không lỗi, không cảnh báo. Máy lấy đâu ra câu trả
lời đúng/sai?

Nghĩ tới bác bảo vệ ở cổng trường. Bác không đọc hết tờ giấy bạn đưa. Bác chỉ
cần trả lời **một** câu cho chính mình: *tờ này có gì không, hay trắng trơn?*
Có chữ thì cho vào, trắng trơn thì mời quay ra. Cái tờ giấy có thể ghi bất cứ
thứ gì — tên, số, một dấu chấm — nhưng câu bác tự hỏi chỉ có hai đáp án.

`if` làm đúng như bác bảo vệ. Nó không đọc giá trị bạn đưa, nó **hỏi giá trị
ấy một câu duy nhất**: có, hay không? Và trong Python, giá trị nào cũng có sẵn
một câu trả lời cho câu hỏi đó.

Người ta gọi câu trả lời ấy là **sự thật** của giá trị — tiếng Anh là
*truthiness*. Muốn xem tận mắt sự thật của một giá trị, gọi `bool()` lên nó:

```python title=readonly
print(bool("cà phê"))
print(bool(""))
print(bool(25500))
print(bool(0))
```

```text
True
False
True
False
```

`bool()` đứng cùng họ với `int()` và `float()` mà bạn đã dùng: đưa vào một giá
trị, nhận về một giá trị thuộc kiểu mang đúng cái tên ấy — ở đây là `bool`,
kiểu đúng/sai bạn đã biết từ Realm 0.
::::

::::example{#luat-cua-su-that}
Luật ngắn đến bất ngờ. Chỉ có một nhóm nhỏ giá trị mang sự thật `False`:

| Giá trị | `bool()` cho ra | Vì sao |
|---|---|---|
| `0` | `False` | số không, không đếm được gì |
| `0.0` | `False` | vẫn là số không, chỉ khác kiểu |
| `""` | `False` | chuỗi rỗng, không một ký tự nào |
| `[]` | `False` | danh sách rỗng, không một phần tử nào |
| `False` | `False` | chính nó đã là câu trả lời "không" rồi |

Còn lại, mọi giá trị bạn đã gặp tới giờ đều là `True`. Cách nhớ: rỗng, bằng
không, hoặc chính chữ `False` thì `False`; có gì trong tay thì `True`.

Cột bên trái còn dài thêm khi bạn gặp kiểu mới — bài sau bạn sẽ gặp thêm đúng
một cái nữa. Còn bốn dòng dưới đây là chỗ hay bị đọc nhầm nhất, và cả bốn đều
theo đúng luật trên:

```python title=readonly
print(bool("0"))
print(bool(" "))
print(bool("False"))
print(bool(0.0))
```

```text
True
True
True
False
```

`"0"` có dấu nháy nên nó là một **chuỗi dài một ký tự** — trong tay có một ký
tự, vậy là `True`. `" "` là chuỗi chứa một dấu cách: mắt không thấy gì, nhưng
vẫn là một ký tự. `"False"` là năm ký tự chữ cái, không phải giá trị `False`.
Còn `0.0` thì đúng là số không, nên `False`.

Ba dòng đầu có chung một điều: **cứ có dấu nháy là chuỗi, và chuỗi chỉ rỗng khi
không còn ký tự nào bên trong.** Nội dung viết gì không quan trọng.
::::

::::predict{#doan-o-so-khong commitOnce}
Sáng nay Byte gửi xe ở chỗ quen, bác giữ xe không lấy tiền. Byte vẫn ghi khoản
ấy vào sổ, và ở ô tiền Byte gõ đúng một ký tự: `0`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
o_tien = "0"

if o_tien:
    print(f"Ô tiền có chữ: {o_tien}")
else:
    print("Ô tiền để trống")
```

:::opt{correct}
Ô tiền có chữ: 0
:::

:::opt
Ô tiền để trống
::why
Gần đúng ở chỗ bạn nhớ đúng nửa đầu của luật: số `0` mang sự thật `False`, và
điều đó chính xác — `bool(0)` đúng là `False` thật.

Chỗ lệch nằm ở hai dấu nháy quanh nó. `"0"` không phải số không, nó là một
chuỗi có **một ký tự** bên trong. Chuỗi chỉ rỗng khi không còn ký tự nào, mà ở
đây còn nguyên một ký tự, nên sự thật của nó là `True`. Máy đọc dấu nháy chứ
không đọc hình dáng của ký tự.
::
:::

:::opt
Máy báo lỗi, vì `if` cần một câu đúng/sai chứ không nhận chuỗi
::why
Gần đúng ở chỗ bạn đang giữ đúng thói quen mà Realm 0 dựng lên: sau `if` là một
câu hỏi có–không, kiểu `tien > 0` hay `ten == "phở"`. Viết như thế bao giờ cũng
đúng, và trong nhiều tình huống nó còn dễ đọc hơn.

Chỗ lệch: đó là thói quen của người viết, không phải luật của máy. `if` nhận
được giá trị nào cũng hỏi nó câu "có hay không", và giá trị nào cũng có sẵn câu
trả lời. Không có gì để báo lỗi cả.
::
:::

:::opt
In cả hai dòng
::why
Gần đúng ở chỗ bạn thấy hai lệnh `print` nằm trong cùng một đoạn code và nghĩ
cả hai đều tới lượt — nếu chúng viết sát lề trái thì đúng là như vậy thật.

Chỗ lệch nằm ở chỗ chúng thụt vào, mỗi dòng thuộc một khối khác nhau. `if` và
`else` là hai lối rẽ của **một** câu hỏi: đi lối này thì không đi lối kia. Máy
chỉ chạy đúng một trong hai khối, không bao giờ cả hai.
::
:::
::::

::::explain{#dung-de-canh-cong}
Giờ dòng `if o_tien:` của bài trước đọc được rồi, và nó làm đúng việc bạn cần:

```python title=readonly
o_tien = ""                       # người ghi sổ bấm Enter suông

if o_tien:
    tien = round(float(o_tien) * 1000)
else:
    tien = 0

print(tien)
```

```text
0
```

Ô rỗng mang sự thật `False`, nên máy không bước vào khối có `float()`. Cú nổ
`ValueError` của bài trước không xảy ra — không phải vì bạn bắt được lỗi, mà vì
bạn **không để nó xảy ra**. Hỏi trước, đổi kiểu sau.

Viết theo lối cũ thì dòng ấy là `if o_tien != "":`, cũng chạy đúng y hệt. Lối
mới ngắn hơn, và ngắn hơn thì đọc nhanh hơn — nhưng chỉ khi bạn thuộc luật sự
thật. Người không thuộc luật sẽ đọc `if o_tien:` thành "nếu có tiền", và đó là
chỗ họ hiểu sai.

> Chỗ dễ vấp: `if so_tien:` với `so_tien` là một **số** thì hoàn toàn khác. Ô
> chữ rỗng và số 0 cùng cho `False`, nên câu `if so_tien:` sẽ bỏ qua cả những
> khoản chi bằng 0 đồng. Với chuỗi thì "rỗng" nghĩa là chưa nhập; với số thì
> `0` là một con số đàng hoàng, đã nhập hẳn hoi.
::::

::::code{#o-tien-de-trong}
Byte cần một hàm đọc ô tiền cho mọi trường hợp: ô có chữ thì đổi ra đồng như
bài trước, ô để trống thì trả về `0` chứ không được để máy nổ.

Hãy điền điều kiện còn thiếu.

```python title=starter
def doc_o_tien(o_tien):
    if ___:
        return round(float(o_tien) * 1000)
    return 0

print(doc_o_tien("32.3"))
print(doc_o_tien(""))
```

```python title=solution
def doc_o_tien(o_tien):
    if o_tien:
        return round(float(o_tien) * 1000)
    return 0

print(doc_o_tien("32.3"))
print(doc_o_tien(""))
```

```python title=test
# Chấm trên BỐN ô, vì một ô thì không phân biệt được gì. Điều kiện lúc nào
# cũng đúng thì nổ ngay ở ô rỗng; điều kiện lúc nào cũng sai thì qua được ô
# rỗng nhưng trượt ba ô còn lại.
assert doc_o_tien("32.3") == 32300, "ô có chữ thì vẫn phải đọc ra tiền như bài trước: 32.3 nghìn là 32 nghìn 3 trăm đồng"
assert doc_o_tien("220") == 220000, "ô ghi 220 nghìn là ô có nội dung, nên nó phải được đi tiếp vào chỗ đổi kiểu"
assert doc_o_tien("") == 0, "người ghi sổ bấm Enter suông thì ô ấy tính là 0 đồng, chứ không được để máy dừng giữa chừng"
# Ô có gõ hẳn số không: đã nhập, nên vẫn phải đi qua nhánh đổi kiểu.
assert doc_o_tien("0") == 0, "ô gõ hẳn một số không là ô đã nhập — nó có một ký tự nên không phải ô để trống"
```

:::hints
- kind: attention
  body: Nhìn dòng ngay dưới chỗ trống: nó gọi `float(o_tien)`, và lời gọi ấy chỉ chạy được khi trong ô có chữ. Vậy câu hỏi của `if` là "ô này có chữ không".
- kind: strategy
  body: Bạn không cần so sánh gì cả. Đặt trần cái tên đang giữ ô chữ vào sau `if` là đủ — máy sẽ tự hỏi nó câu "có hay không", và chuỗi rỗng trả lời là không.
- kind: one-line
  body: "Viết `o_tien` vào chỗ trống, thành `if o_tien:`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^32300\n0\s*$
- tier: static
  onFail: điều kiện phải hỏi về chính ô chữ `o_tien`, không phải một giá trị cố định
  requireAst:
  # `o_tien` phải được ĐỌC ít nhất hai lần: một lần trong điều kiện, một lần
  # trong `float(o_tien)`. Điền bừa một giá trị cố định chỉ còn một lần.
  - kind: uses-name, target: o_tien, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ô trống mình trả về 0, ô có chữ mình đọc. Không lần nào phải dừng giữa chừng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàm bạn vừa viết chạy đúng cho cả bốn ô. Nhưng hãy nhìn kỹ hai câu cuối trong
khối test:

```python
assert doc_o_tien("") == 0     # ô để trống, chưa gõ gì cả
assert doc_o_tien("0") == 0    # ô có gõ hẳn một số không
```

Hai chuyện khác nhau một trời một vực, mà hàm trả về **cùng một** con số `0`.

Đem `0` ấy đi thì lẫn thật: một dòng sổ ghi 0 đồng có thể là *gửi xe miễn phí,
đã biết chắc là không mất đồng nào*, mà cũng có thể là *chưa hỏi ai, chưa
biết bao nhiêu*. Cuối tháng cộng sổ, hai loại ấy phải cư xử khác nhau: khoản
miễn phí thì cộng vào bình thường, khoản chưa biết thì phải đi hỏi cho ra chứ
không được coi như không tốn gì.

Số `0` không nói được "chưa biết". Chuỗi `""` cũng không, vì `""` chỉ có nghĩa
là ô ấy rỗng chứ không có nghĩa là bạn chưa từng hỏi.

Vậy máy có sẵn một giá trị nào mang đúng nghĩa **chưa có gì** không, để bạn cất
vào chỗ ấy thay cho số 0?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
