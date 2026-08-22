---
id: nen-tang.gia-tri-bien-kieu.phuong-thuc-di-lien-gia-tri
title: Cái hàm đi liền với giá trị
summary: Mỗi chuỗi mang sẵn một số việc gọi bằng dấu chấm. Việc ấy không sửa chuỗi cũ — nó đưa lại một chuỗi mới, và không hứng lấy là mất trắng.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.string-method]
requires: [core.string-immutable, core.string-slice, core.string-sequence, core.list-append, core.list, ctrl.for-each, ctrl.if, ctrl.comparison, core.string-literal, core.variable, core.output]
concepts: [core.chuoi, core.tra-ve, core.gia-tri]
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
Mình làm cho bạn bản chữ thường. Nhưng bản gốc mình trả lại nguyên đấy nhé.
::::

::::explain{#tiem-photocopy-dau-ngo}
Bài trước để lại một việc chưa xong: khách gõ `"Cà Phê"`, sổ ghi `"cà phê"`, và
bạn cần kéo cả hai về cùng một dạng trước khi đem so. Sửa từng ô thì chuỗi không
cho, cắt khúc thì không đổi được chữ nào.

Ra tiệm photocopy đầu ngõ một lát.

Bạn đưa cô chủ tiệm một tờ giấy và nói: *cho cháu bản in mờ đi một chút*. Cô
không cầm bút tô đè lên tờ của bạn. Cô đặt nó lên máy, và **đưa lại cho bạn một
tờ mới** — tờ gốc trả về nguyên vẹn, không thêm một vết mực.

Hai chi tiết trong chuyện này chính là cả bài học hôm nay:

1. Việc "in mờ" không phải việc bạn tự làm ở nhà. Nó là việc **đi kèm cái tiệm**
   — trên tường có bảng dịch vụ, mỗi dòng một việc tiệm nhận làm.
2. Ra về mà quên cầm tờ mới thì bạn về tay không. Tờ gốc vẫn còn, nhưng bản in
   mờ nằm lại trên máy.

Trong Python, mỗi giá trị cũng mang theo một bảng dịch vụ như vậy. Một việc nằm
trong bảng ấy gọi là **phương thức** — tiếng Anh là *method*. Gọi nó bằng cách
viết dấu chấm ngay sau giá trị:

```text
gia_tri.ten_viec()
```

Cách viết này bạn đã gõ từ Realm 0: `mon_da_goi.append("bún chả")` — `append` là
một phương thức của danh sách. Nên dấu chấm không phải thứ mới. Thứ mới nằm ở
chỗ khác, và nó là chỗ hai bên khác nhau một trời một vực.
::::

::::example{#lam-xong-roi-dua-lai}
Phương thức đưa chữ về thường tên là `lower` — tiếng Anh *lower case* nghĩa là
chữ thường. Gọi thử:

```python title=readonly
ten = "Cà Phê"
ten.lower()
print(ten)
```

```text title=readonly
Cà Phê
```

Không có gì đổi cả. Không lỗi, không báo, và cũng không kết quả.

Đây đúng là chỗ nhiều người mất cả buổi để tìm ra, nên nói cho thật rõ. Dòng
`ten.lower()` **có chạy thật**: máy đã dựng xong bản chữ thường. Rồi nó đưa bản
ấy ra — và không có ai đứng đó nhận. Bản mới rơi mất ngay tại dòng đó.

Bài trước đã dựng sẵn lý do: chuỗi là giá trị bất biến, `ten` không sửa được một
ô nào, kể cả bởi chính phương thức của nó. Cho nên `lower` **buộc phải** đi
đường vòng — dựng chuỗi mới rồi đưa lại. Đó là con đường duy nhất luật bất biến
còn để ngỏ.

Muốn nhận, làm một trong hai cách:

```python title=readonly
ten = "Cà Phê"
print(ten.lower())
print(ten)
```

```text title=readonly
cà phê
Cà Phê
```

```python title=readonly
ten = "Cà Phê"
ten_thuong = ten.lower()
print(ten_thuong == "cà phê")
```

```text title=readonly
True
```

Cách trên đưa thẳng bản mới cho `print`. Cách dưới hứng nó vào một cái tên để
còn dùng tiếp. Cả hai lần, `ten` vẫn là `"Cà Phê"` — bản gốc trả về nguyên vẹn,
đúng như tờ giấy ở tiệm photocopy.

Giờ mới thấy hết chỗ khác nhau với `append`:

- `mon_da_goi.append("bún chả")` **sửa thẳng chính cái danh sách ấy**, làm nó
  dài thêm một món, và gọi xong thì không đưa lại gì cả — nên không ai hứng kết
  quả của `append` vào một cái tên bao giờ.
- `ten.lower()` **không đụng được vào `ten`**, nên nó đưa lại một chuỗi mới. Bỏ
  không hứng thì mất trắng.

Một câu để nhớ cả hai: *danh sách thì sửa tại chỗ, chuỗi thì đưa lại bản mới.*
Và mọi phương thức của chuỗi bạn gặp từ đây về sau đều theo luật thứ hai.
::::

::::predict{#doan-goi-xong-con-gi commitOnce}
Sổ đang ghi khoản `"bún chả"`. Khách gõ vào `"Bún Chả"`. Byte viết đoạn dưới để
xem hai bên có phải cùng một khoản không. **Trước khi bấm chạy**, bạn đoán màn
hình hiện ra gì?

```python
mon = "Bún Chả"
mon.lower()
print(mon == "bún chả")
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn nhận ra `lower` đúng là thứ cần gọi ở đây, và ở chỗ bạn tin
rằng dòng `mon.lower()` có chạy — nó chạy thật, máy đã dựng xong `"bún chả"`.

Chỗ lệch nằm ở chỗ bản vừa dựng đi đâu. Dòng ấy không có dấu `=` nào, không nằm
trong `print` nào, nên không ai nhận bản mới và nó rơi mất ngay tại đó. Tới dòng
`print`, `mon` vẫn là `"Bún Chả"` với hai chữ hoa — và `==` so từng ký tự một,
nên `B` khác `b` là đủ để ra `False`.
::
:::

:::opt
Máy báo TypeError, vì chuỗi không sửa được
::why
Gần đúng ở chỗ bạn nhớ chắc luật bài trước nữa — một chuỗi đã dựng thì không sửa
được ô nào — và bạn đang chờ máy chặn bất cứ dòng nào định sửa nó. Nếu dòng giữa
là `mon[0] = "b"` thì bạn đã trúng.

Chỗ lệch: `mon.lower()` không hề định sửa `mon`. Nó dựng ra một chuỗi thứ hai
rồi đưa ra ngoài, còn `mon` thì để yên. Chính vì luật bất biến mà phương thức
chuỗi phải làm theo kiểu đó, nên đây là cách gọi hợp luật, không phải cách gọi
bị cấm.
::
:::

:::opt
Máy báo lỗi ở dòng `mon.lower()`, vì gọi xong mà không gán cho ai
::why
Gần đúng ở chỗ bạn thấy dòng ấy có gì đó không ổn — và cảm nhận ấy chính xác:
dòng đó đúng là chạy không để lại gì, đúng là dòng thừa, và nó chính là cái bẫy
của bài này.

Chỗ lệch nằm ở việc Python coi chuyện đó là hợp lệ. Một dòng chỉ gọi một việc
rồi bỏ kết quả là chuyện bình thường — `print(...)` cũng đưa lại một thứ mà
chẳng ai hứng bao giờ. Máy không có cách nào biết bạn *muốn* giữ hay *muốn* bỏ,
nên nó im lặng làm theo đúng chữ bạn viết. Bẫy không nằm ở chỗ máy báo; nó nằm ở
chỗ máy **không** báo.
::
:::
::::

::::explain{#ba-cau-dung-duoc-ngay}
Rút gọn thành ba câu:

- **Hình dạng**: `gia_tri.ten_viec()` — dấu chấm dán việc vào đúng cái giá trị
  sắp được làm. Cặp ngoặc tròn cuối là chỗ ra lệnh *làm đi*; thiếu nó thì bạn
  mới chỉ trỏ vào tên việc chứ chưa nhờ ai làm gì.
- **Với chuỗi**: phương thức không sửa bản gốc, nó **đưa lại** một chuỗi mới.
- **Hệ quả**: dòng `ten.lower()` đứng trơ một mình là dòng chết. Phải có chỗ
  nhận — hoặc một cái tên, hoặc đưa thẳng vào chỗ đang cần dùng.
::::

::::code{#doi-chieu-ten-khoan}
Bà chủ đưa cho Byte ba dòng khách gõ trong ngày. Sổ đang có sẵn khoản `cà phê`
viết bằng chữ thường. Byte cần biết dòng nào là khoản đã có, dòng nào là khoản
mới.

Điền vào chỗ trống sao cho hai bên dấu `==` mang cùng một dạng chữ. Để ý hai
dòng `ket_qua.append(...)`: chúng ghép thẳng `ten` vào câu trả lời, nên thứ hiện
ra phải là **nguyên văn khách gõ** — bản gốc không được đổi.

```python title=starter
so_tay = "cà phê"
khach_go = ["Cà Phê", "CÀ PHÊ", "Bún Chả"]
ket_qua = []

for ten in khach_go:
    if ___ == so_tay:
        ket_qua.append(ten + " → đúng khoản trong sổ")
    else:
        ket_qua.append(ten + " → khoản mới")

for dong in ket_qua:
    print(dong)
```

```python title=solution
so_tay = "cà phê"
khach_go = ["Cà Phê", "CÀ PHÊ", "Bún Chả"]
ket_qua = []

for ten in khach_go:
    if ten.lower() == so_tay:
        ket_qua.append(ten + " → đúng khoản trong sổ")
    else:
        ket_qua.append(ten + " → khoản mới")

for dong in ket_qua:
    print(dong)
```

```python title=test
# Ba dòng khách gõ là ba tình huống cố ý khác nhau:
#   "Cà Phê" — hoa lẫn thường, phải nhận ra là khoản đã có
#   "CÀ PHÊ" — hoa tất, cũng phải nhận ra (chép tay `"cà phê"` vào chỗ trống thì
#              ca này lộ ngay, vì lúc đó cả ba dòng đều thành "đúng khoản")
#   "Bún Chả" — khoản khác hẳn, phải rơi vào nhánh else
assert ket_qua == [
    "Cà Phê → đúng khoản trong sổ",
    "CÀ PHÊ → đúng khoản trong sổ",
    "Bún Chả → khoản mới",
]
# Phương thức chuỗi đưa lại bản mới; danh sách khách gõ phải nguyên si.
assert khach_go == ["Cà Phê", "CÀ PHÊ", "Bún Chả"]
```

:::hints
- kind: attention
  body: Nhìn hai bên dấu `==`. Vế phải là `so_tay`, đang giữ `"cà phê"` — chữ thường tuốt. Vế trái phải là một chuỗi ở **cùng dạng** ấy, dựng ra từ `ten` của vòng này. Và để ý `ten` còn được dùng lại ở hai dòng dưới, nên nó phải giữ nguyên bản khách gõ.
- kind: strategy
  body: Chuỗi mang sẵn một phương thức đưa mọi chữ về thường. Gọi nó bằng dấu chấm ngay sau `ten`, đừng quên cặp ngoặc tròn ở cuối. Nó không sửa `ten` mà đưa lại một chuỗi mới — ở đây bạn không cần cái tên nào hứng, vì đem so ngay tại chỗ cũng là một cách nhận.
- kind: one-line
  body: Viết `ten.lower()` vào chỗ trống.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: CÀ PHÊ → đúng khoản trong sổ
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai kiểu viết hoa, một khoản. Mà sổ vẫn giữ đúng chữ khách gõ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm sau bà chủ đưa thêm một dòng nữa, và lần này nó làm bạn cụt hứng:

```python title=readonly
print(" cà phê ".lower() == "cà phê")
```

```text title=readonly
False
```

Hai bên đã cùng chữ thường. Cùng đủ sáu chữ cái, đúng thứ tự, không thiếu dấu
nào. Đọc lên nghe y hệt nhau. Vậy mà `==` vẫn nói không.

Chỗ khác nhau có thật, nó nằm ngay trước mắt bạn — chỉ là mắt không bắt được,
vì nó không phải một chữ. Đếm lại số ô của hai chuỗi ấy thì con số sẽ tố cáo
ngay.

Mà chỗ ấy còn lẻn vào sổ thường xuyên hơn bạn tưởng: khách gõ xong hay quệt
thêm một cái trước khi bấm Enter.

Vậy thứ vô hình đó là gì, và có phương thức nào dọn nó đi không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
