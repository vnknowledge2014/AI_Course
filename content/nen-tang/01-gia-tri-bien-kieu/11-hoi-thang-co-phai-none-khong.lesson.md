---
id: nen-tang.gia-tri-bien-kieu.hoi-thang-co-phai-none-khong
title: Hỏi thẳng — có phải `None` không
summary: Câu hỏi riêng dành cho None, và là chỗ duy nhất phân biệt được "chưa có gì" với "có mà rỗng".
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.is-none]
requires: [core.none, core.boolean, ctrl.if, ctrl.elif, ctrl.else, ctrl.for-each, core.list, core.list-append, core.variable, core.assignment]
concepts: [core.gia-tri, core.dung-sai, core.o-trong]
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
Hỏi "ô này có chữ không" thì hai ô trả lời giống nhau. Phải đổi câu hỏi.
::::

::::explain{#doi-cau-hoi-chu-khong-doi-o}
Bài trước để lại đúng một chỗ hụt: `0`, `""` và `None` là ba giá trị khác nhau,
nhưng `bool` của cả ba đều là `False`. Nên `if ghi_chu:` vẫn xếp "chưa hỏi được
khách" chung một nhánh với "đã hỏi, khách không ghi gì".

Nghĩ về nó như hai câu hỏi khác nhau hỏi về cùng một ô sổ.

Câu thứ nhất — `if ghi_chu:` — là hỏi **"ô này có chữ nào đọc được không?"**.
Đưa cả hai ô ra hỏi: ô trắng giấy trả lời "không", ô đã gạch một nét cho rỗng
cũng trả lời "không". Câu hỏi ấy không sai, nó chỉ không phân biệt được, vì cả
hai ô đúng là đều không có chữ để đọc.

Câu thứ hai là câu bạn cần: **"đã có ai chạm bút vào ô này chưa?"**. Ô trắng
giấy trả lời "chưa". Ô đã gạch cho rỗng trả lời "rồi" — có người đã cầm bút,
người ấy đã quyết định để nó rỗng.

Không có cách nào rút câu hỏi thứ hai ra từ câu hỏi thứ nhất. Phải hỏi thẳng.
Và Python có sẵn cách hỏi thẳng ấy:

```text
ghi_chu is None
```

Đọc lên gần như tiếng Việt: *`ghi_chu` có phải chính là `None` không*. Kết quả
là `True` hoặc `False`, nên nó đặt được thẳng vào `if` như mọi điều kiện khác.

Từ `is` trong tiếng Anh nghĩa là "là". Ở đây nó hỏi một câu chặt hơn dấu `==`:
không phải "hai bên có bằng nhau không", mà **"bên trái có đúng là chính cái thứ
bên phải không"**. Vì `None` chỉ tồn tại một cái duy nhất trong cả Python — như
bài trước đã nói, cả kiểu `NoneType` chỉ có mỗi một giá trị — nên `is None`
không bao giờ nhận nhầm ai khác.

Có cả cách hỏi ngược: `ghi_chu is not None`, nghĩa là *có thứ gì đó rồi*. Hai
chữ `is not` viết liền nhau như một cụm.

Bạn sẽ thấy người ta viết `ghi_chu == None` và nó cũng cho ra đúng câu trả lời.
Nhưng người viết Python luôn chọn `is None`, vì đó mới là câu hỏi bạn thật sự
muốn đặt: hỏi *có phải chính nó không*, chứ không hỏi *có bằng nó không*. Bài
này dùng `is None`, và từ đây về sau cũng vậy.
::::

::::example{#ba-o-ba-cau-tra-loi}
Ba ô ghi chú của ba dòng sổ, đem hỏi cả hai câu một lượt:

```python title=readonly
cac_o = [None, "", "khách quen"]

for o in cac_o:
    print(o, bool(o), o is None)
```

```text title=readonly
None False True
 False False
khách quen True False
```

Cột giữa là câu hỏi cũ, cột phải là câu hỏi mới. Đọc theo cột:

- **Cột giữa** (`bool(o)`): `False`, `False`, `True`. Hai ô đầu dính nhau — đúng
  cái hụt của bài trước.
- **Cột phải** (`o is None`): `True`, `False`, `False`. Lần này ô trắng giấy
  đứng riêng một mình, hai ô kia về chung phía.

Hai câu hỏi cắt tập hợp ba ô theo hai đường khác nhau. Chỉ đường thứ hai mới
tách được "chưa có gì" ra khỏi "có mà rỗng".

Để ý dòng thứ hai của kết quả bắt đầu bằng một khoảng trắng: đó là `print` in ô
`""` ra, không có ký tự nào, rồi mới tới dấu cách ngăn với `False`.

Đặt vào một câu `if` hoàn chỉnh thì nó thành ra thế này:

```python title=readonly
ghi_chu = ""

if ghi_chu is None:
    print("Chưa hỏi khách")
else:
    print("Đã hỏi rồi, khách không ghi gì")
```

```text title=readonly
Đã hỏi rồi, khách không ghi gì
```

Cùng cái ô `""` ấy, nếu viết `if ghi_chu:` thì bạn chỉ biết được "không có chữ".
Viết `if ghi_chu is None:` thì bạn biết thêm một chuyện mà cách kia không nói
được: **đã có người hỏi rồi**.
::::

::::predict{#doan-hai-cot commitOnce}
Byte để hai ô cạnh nhau: một ô đã hỏi mà nhập rỗng, một ô chưa ai chạm vào.
**Trước khi bấm chạy**, bạn đoán hai dòng in ra là gì?

```python title=readonly
o_trong = ""
o_chua_cham = None

print(bool(o_trong), bool(o_chua_cham))
print(o_trong is None, o_chua_cham is None)
```

:::opt{correct}
Dòng đầu `False False`, dòng sau `False True`
:::

:::opt
Dòng đầu `False False`, dòng sau `True True`
::why
Gần đúng ở chỗ bạn thấy hai ô này đều rỗng, nên nghĩ câu hỏi nào hỏi về sự rỗng
cũng phải trả lời giống nhau cho cả hai. Cách nhóm ấy chính xác với câu hỏi ở
dòng đầu — và dòng đầu bạn đoán đúng hoàn toàn.

Chỗ lệch: `is None` không hỏi "có rỗng không". Nó hỏi "có phải **chính** giá trị
`None` không". Ô `""` là một chuỗi — một chuỗi không chứa ký tự nào, nhưng vẫn
là chuỗi, vẫn có nhãn `str`. Nó không phải `None`, nên câu trả lời cho nó là
`False`.
::
:::

:::opt
Dòng đầu `False True`, dòng sau `False True`
::why
Gần đúng ở chỗ bạn nhớ bài trước nói `None` là một giá trị có thật, in ra được
hẳn hoi, có nhãn kiểu riêng. Điều đó đúng, và nó là chỗ nhiều người bỏ sót.

Chỗ lệch: "có thật" không kéo theo "sự thật của nó là `True`". Luật quy về đúng
sai xếp `None` về phía `False`, đứng cùng chỗ với `0` và `""` — và chính vì bị
xếp chung như vậy mà bài này mới phải tồn tại. `bool(None)` là `False`.
::
:::

:::opt
Máy dừng lại ở dòng `bool(o_chua_cham)` và báo lỗi, vì `None` không quy ra đúng
sai được
::why
Gần đúng ở chỗ bạn cảnh giác với một giá trị "rỗng" bị đem đi làm việc của giá
trị thường — và cảnh giác ấy có chỗ dùng thật: đem `None` đi cộng với một con số
thì máy dừng lại và báo lỗi kiểu ngay.

Chỗ lệch: quy về đúng/sai là việc **mọi** giá trị trong Python đều làm được,
không sót cái nào. Nhờ vậy `if` mới nhận được bất cứ thứ gì bạn đặt vào. `None`
làm được việc đó, và câu trả lời của nó là `False`.
::
:::
::::

::::explain{#hoi-none-truoc-roi-hoi-rong-sau}
Có một trật tự hỏi rất hay dùng khi một ô có ba tình cảnh. Hỏi câu hẹp nhất
trước, rồi mới tới câu rộng hơn:

```text
nếu ô là None        →  chưa ai chạm bút
còn nếu ô == ""      →  đã hỏi, để rỗng
còn lại              →  có nội dung thật
```

Thứ tự này quan trọng. Câu `is None` chỉ bắt đúng **một** giá trị, nên nó không
bao giờ nuốt mất trường hợp của câu đứng sau. Đổi nhánh đầu thành một câu hỏi
rộng — chẳng hạn hỏi xem ô có rỗng hay không, kiểu câu bắt cả `None` lẫn `""` —
thì nhánh `elif` phía dưới không còn ai để nhận nữa.

Còn một chỗ nữa `is None` cứu bạn, và nó là chỗ dễ mất tiền: cột **tiền**. Khoản
gửi xe `0` đồng là một con số hợp lệ, cộng vào tổng được. Khoản chưa hỏi giá là
`None`, cộng vào tổng thì máy dừng lại ngay. Với `if tien:` thì cả hai cùng rơi
vào nhánh "coi như không có". Với `if tien is None:` bạn tách được đúng khoản
chưa hỏi ra để đi hỏi lại — còn khoản 0 đồng vẫn nằm trong sổ như một khoản chi
đàng hoàng.
::::

::::code{#in-cot-ghi-chu}
Cuối buổi, Byte in cột ghi chú của ba dòng sổ. Ba dòng ba tình cảnh: một dòng có
nội dung thật, một dòng đã hỏi mà khách để rỗng, một dòng chưa hỏi được ai.

Byte muốn cột in ra như thế này:

- có nội dung thật thì in nguyên nội dung;
- đã hỏi mà rỗng thì in `(khách không ghi gì)`;
- chưa hỏi được thì in `(chưa hỏi khách)`.

Nhánh `elif` và nhánh `else` đã viết sẵn. Còn thiếu điều kiện của nhánh đầu —
nhánh hẹp nhất, nhánh chỉ dành cho đúng một giá trị.

```python title=starter
cac_o = ["cà phê cho anh Tuấn", "", None]
dong_in = []

for o in cac_o:
    if ___:
        nhan = "(chưa hỏi khách)"
    elif o == "":
        nhan = "(khách không ghi gì)"
    else:
        nhan = o
    dong_in.append(nhan)
    print(nhan)
```

```python title=solution
cac_o = ["cà phê cho anh Tuấn", "", None]
dong_in = []

for o in cac_o:
    if o is None:
        nhan = "(chưa hỏi khách)"
    elif o == "":
        nhan = "(khách không ghi gì)"
    else:
        nhan = o
    dong_in.append(nhan)
    print(nhan)
```

```python title=test
# Ba ô, ba nhãn khác nhau — một điều kiện đúng phải làm cả ba dòng cùng đúng.
# Điều kiện luôn đúng thì cả ba thành "(chưa hỏi khách)".
# Điều kiện luôn sai thì ô None tụt xuống nhánh else và lọt nguyên `None`.
assert dong_in == ["cà phê cho anh Tuấn", "(khách không ghi gì)", "(chưa hỏi khách)"]
```

:::hints
- kind: attention
  body: Nhìn nhánh `elif` ngay dưới chỗ trống — nó đã lo trường hợp ô rỗng bằng dấu `==`. Vậy nhánh trên nó phải lo một trường hợp khác hẳn, và trường hợp ấy chỉ có đúng một giá trị rơi vào.
- kind: strategy
  body: Câu hỏi bạn cần đặt là "ô này có phải chính là giá trị dành cho chưa-có-gì không". Hỏi bằng `bool` thì không được, vì cách hỏi ấy bắt luôn cả ô rỗng mà nhánh `elif` đang chờ.
- kind: one-line
  body: "Viết `o is None` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: (chưa hỏi khách)
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba dòng, ba câu khác nhau. Ô chưa hỏi giờ tự nói ra là nó chưa được hỏi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại dòng mở đầu của mọi ví dụ trong hai bài vừa rồi:

```text
ghi_chu = None
```

Đó là một **dòng gán** hẳn hoi. Có dấu `=`, có vế trái, có vế phải. Cái tên
`ghi_chu` có mặt kể từ dòng ấy, và nội dung nó giữ là "chưa có gì" — hai chuyện
khác nhau, và cả hai đều do bạn viết ra.

Bây giờ thử tưởng tượng bạn **quên hẳn** dòng ấy. Không có `ghi_chu = None` nào
cả, chỉ có một dòng `print(ghi_chu)` nằm giữa chương trình.

Máy có tự hiểu rằng cái tên ấy "chưa có gì" rồi lặng lẽ đưa cho bạn một `None`
không? Hay nó làm chuyện khác hẳn?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
