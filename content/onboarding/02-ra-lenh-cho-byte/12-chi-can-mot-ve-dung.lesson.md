---
id: onboarding.ra-lenh-cho-byte.chi-can-mot-ve-dung
title: Chỉ cần một vế đúng
summary: Từ nối thứ hai, dễ tính hơn: một vế đúng là đủ để cả câu thành đúng.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [logic.or]
requires: [ctrl.if, core.boolean, logic.and]
concepts: [core.dung-sai, logic.phep-logic]
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
`and` bắt cả hai vế cùng đúng. Tờ giấy mới của quán chỉ đòi một vế thôi.
::::

::::example{#and-lam-sai-viec}
Bài trước để lại cho bạn một câu hỏi: đưa tờ giấy mới cho `and` làm thì sao?

> Học sinh **hoặc** người trên 65 tuổi: giảm 5000 đồng.

Bác 70 tuổi bước vào quán. Bác không phải học sinh.

```python title=readonly
la_hoc_sinh = False
tuoi_khach = 70

if la_hoc_sinh and tuoi_khach > 65:
    print("Giảm 5000 đồng")
else:
    print("Giá thường")
```

Máy in ra: `Giá thường`.

Bác bị tính giá đầy đủ, dù tờ giấy trên tường ghi rành rành rằng bác được giảm.

Máy không hỏng, và nó cũng không hiểu nhầm. Nó làm đúng thứ bạn viết: vế trái là
`False`, vế phải là `True`, và theo bảng bốn dòng hôm qua, `False and True` cho
ra `False`. Luật đó là luật đúng — nhưng đúng cho tờ giấy **cũ**, tờ đòi cả hai
điều kiện.

Tờ giấy mới cần một luật khác hẳn: một vế đúng cũng đủ gật. Python có sẵn một từ
cho luật đó, và nó ngắn hơn `and` một chữ: `or` — tiếng Anh nghĩa là "hoặc".
::::

::::explain{#bang-bon-dong-cua-or}
`or` đứng ở đúng chỗ mà `and` từng đứng: giữa hai giá trị đúng/sai, và cho ra
một giá trị đúng/sai. Nó cũng có đúng bốn trường hợp:

| vế trái | vế phải | `or` cho ra |
|---|---|---|
| `True` | `True` | `True` |
| `True` | `False` | `True` |
| `False` | `True` | `True` |
| `False` | `False` | `False` |

Đặt bảng này cạnh bảng của `and` hôm qua thì thấy chúng là hai mặt của một tờ
giấy:

- `and` cho `True` ở **đúng một** dòng — dòng cả hai vế cùng đúng.
- `or` cho `False` ở **đúng một** dòng — dòng cả hai vế cùng sai.

Nói theo kiểu quán phở: người bán cầm tờ giấy `or` chỉ lắc đầu khi bạn không có
thứ nào trong hai thứ. Có một thứ là gật. Có cả hai thì càng gật.
::::

::::predict{#hai-ve-cung-dung commitOnce}
Sáng nay có một khách đặc biệt: bác 70 tuổi, đang đi học lớp xoá mù chữ buổi
tối, nên bác **vừa** là học sinh **vừa** trên 65 tuổi. Cả hai vế đều đúng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra dòng nào?

```python
la_hoc_sinh = True
tuoi_khach = 70

if la_hoc_sinh or tuoi_khach > 65:
    print("Giảm 5000 đồng")
else:
    print("Giá thường")
```

:::opt{correct}
Giảm 5000 đồng
:::

:::opt
Giá thường
::why
Gần đúng ở chỗ bạn đang đọc chữ `or` theo đúng nghĩa tiếng Việt hằng ngày. Khi
người ta nói *"ăn phở hoặc ăn bún"*, ý là chọn một trong hai, lấy cả hai thì
không còn là "hoặc" nữa. Hiểu như thế trong đời sống là hiểu đúng.

Chỗ lệch: `or` của Python không hỏi *"bạn chọn vế nào?"*. Nó hỏi đúng một câu:
*"có ít nhất một vế đúng không?"*. Hai vế cùng đúng thì câu trả lời vẫn là có —
thậm chí còn chắc chắn hơn.

Nhìn lại bảng bốn dòng: dòng `True or True` cho ra `True`. Chỉ một dòng duy nhất
cho ra `False`, và đó là dòng hai vế cùng sai.
::
:::

:::opt
Giảm 10000 đồng
::why
Gần đúng ở chỗ bạn nghĩ hai lý do giảm giá thì cộng dồn lại. Ngoài đời có quán
làm đúng như vậy thật, nên đây là suy nghĩ rất hợp lẽ thường.

Chỗ lệch: `or` không đếm xem có bao nhiêu vế đúng. Nó chỉ trả về `True` hoặc
`False`, một trong hai, không có mức độ ở giữa. Và máy thì chỉ in ra đúng dòng
chữ bạn đã gõ sẵn trong khối — trong đoạn này không có dòng nào ghi 10000 cả.

Muốn cộng dồn khuyến mãi, bạn phải tự viết ra thành các nhánh riêng, chẳng hạn
một chuỗi `if / elif / else` với một nhánh dành cho người trúng cả hai điều kiện.
::
:::

:::opt
Máy báo lỗi vì nó không biết chọn vế nào
::why
Gần đúng ở chỗ bạn cảm thấy chữ "hoặc" có gì đó lấp lửng — và đúng là trong
tiếng Việt nó lấp lửng thật, nên cảnh giác ở đây là phản xạ tốt.

Chỗ lệch: `or` không phải chỗ để chọn. Nó không lấy vế nào ra dùng cả. Việc duy
nhất của nó là nhìn hai câu trả lời có–không rồi gộp thành **một** câu trả lời
có–không, theo đúng bốn dòng trong bảng. Máy không phải quyết định gì mập mờ,
nên cũng không có gì để báo lỗi.
::
:::
::::

::::explain{#dat-canh-nhau}
Bây giờ bạn có hai từ nối, và chọn nhầm từ là một lỗi máy không bao giờ báo cho
bạn biết — chương trình vẫn chạy, chỉ tính sai tiền của khách.

Cách chọn: đọc lại tờ giấy dán tường bằng tiếng Việt, rồi tìm chữ bản lề.

- Thấy chữ **"và"**, **"vừa... vừa..."**, **"phải có đủ"** → dùng `and`.
- Thấy chữ **"hoặc"**, **"một trong hai"**, **"chỉ cần"** → dùng `or`.

Hai câu dưới đây khác nhau đúng một từ, và khác nhau cả một chính sách bán hàng:

- `la_hoc_sinh and tuoi_khach > 65` — chỉ giảm cho học sinh trên 65 tuổi. Quán
  này gần như không giảm cho ai.
- `la_hoc_sinh or tuoi_khach > 65` — giảm cho học sinh, và giảm cho cả người
  trên 65. Đông khách được giảm hơn nhiều.

> Dễ nhầm: giống `and`, mỗi vế của `or` phải là một câu hỏi đủ đầu đủ đuôi. Viết
> `tuoi_khach > 65 or > 80` là `SyntaxError`, vì vế sau cụt mất bên trái. Phải
> viết lại đủ: `tuoi_khach > 65 or tuoi_khach > 80`.
::::

::::code{#tam-bien-khuyen-mai}
Đoạn dưới thiếu đúng một từ nối. Khách là bác 70 tuổi, không phải học sinh —
theo tờ giấy mới, bác vẫn phải được nghe `Giảm 5000 đồng`.

Hãy điền vào chỗ trống để `if` gật khi **ít nhất một** vế đúng.

```python title=starter
la_hoc_sinh = False
tuoi_khach = 70

if la_hoc_sinh ___ tuoi_khach > 65:
    print("Giảm 5000 đồng")
else:
    print("Giá thường")
```

```python title=solution
la_hoc_sinh = False
tuoi_khach = 70

if la_hoc_sinh or tuoi_khach > 65:
    print("Giảm 5000 đồng")
else:
    print("Giá thường")
```

```python title=test
# Chấm bằng OUTPUT: bác 70 tuổi tuy không phải học sinh vẫn phải được giảm giá.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm **giữa** hai câu hỏi có–không, đúng vị trí mà bài trước bạn đã điền một từ nối. Vế trái đang là `False`, vế phải đang là `True`.
- kind: strategy
  body: Tờ giấy dán tường lần này ghi chữ "hoặc" — một vế đúng là đủ giảm. Trong hai từ nối bạn đã biết, hãy chọn từ có bảng bốn dòng chỉ cho ra `False` ở đúng dòng cuối.
- kind: one-line
  body: Thay `___` bằng `or`, giữ nguyên hai vế ở hai bên và dấu hai chấm cuối dòng.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Giảm 5000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bác 70 tuổi được giảm rồi. Đổi một từ là đổi cả chính sách của quán.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã dạy được máy rẽ nhánh khá thành thạo: một nhánh, hai nhánh, nhiều nhánh,
và ghép hai điều kiện bằng `and` hay `or`.

Chiều nay chủ quán nhờ bạn một việc rất khác. Quán có mười món, bà muốn in ra
màn hình mười dòng để dán lên tường:

> Món số 1
>
> Món số 2
>
> Món số 3

...và cứ thế tới `Món số 10`.

Cách bạn đang có là gõ mười dòng `print`, gần giống hệt nhau, khác nhau đúng một
con số. Gõ xong bạn sẽ thấy ba chuyện: mỏi tay, dễ gõ nhầm một số ở giữa mà
không ai phát hiện, và hôm sau quán thêm món thứ mười một thì phải mở file ra
sửa tiếp.

Máy tính vốn giỏi nhất đúng cái việc lặp lại không mệt. Vậy có cách nào nói với
nó **một lần** rằng *"làm việc này mười lượt"*, thay vì chép tay mười dòng?

Đừng trả lời vội. Bài sau là một câu lệnh thay được cả mười dòng ấy.
::::

::::checkpoint{mastery=0.8}
::::
