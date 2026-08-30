---
id: onboarding.ra-lenh-cho-byte.dung-hay-sai
title: Đúng hay sai
summary: Có những câu hỏi chỉ có hai câu trả lời. Máy trả lời chúng bằng True và False.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [core.boolean, ctrl.comparison]
requires: [core.variable, core.type-of-value]
concepts: [core.dung-sai, core.so-sanh]
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
Có loại câu hỏi chỉ trả lời được hai cách: có, hoặc không. Máy rất giỏi loại đó.
::::

::::explain{#hai-cau-tra-loi}
Buổi sáng ở quán phở, bà chủ nhìn nồi nước dùng rồi hỏi một câu:

> Còn đủ cho mười tô nữa không?

Câu này không trả lời được bằng "hơi hơi". Chỉ có **còn** hoặc **không còn**.
Trả lời xong là biết ngay phải làm gì tiếp.

Máy tính cũng có đúng hai câu trả lời như vậy, và chúng có tên sẵn:

- `True` — đúng, có, thật
- `False` — sai, không, không phải

Hai từ này viết hoa chữ cái đầu, và **không có dấu nháy**. Bạn còn nhớ luật cũ
chứ: trong nháy là chữ để đọc nguyên văn, không nháy là thứ máy phải hiểu.
`True` và `False` nằm trong nhóm thứ hai — máy đã biết sẵn hai từ này từ trước
khi bạn viết dòng đầu tiên.

`"True"` (có nháy) chỉ là bốn chữ cái. `True` (không nháy) là **một câu trả
lời**. Hai thứ khác hẳn nhau.
::::

::::example{#hoi-may-mot-cau}
Bạn hỏi máy một câu, và máy trả lời bằng một trong hai từ đó:

```python title=readonly
gia_pho = 45000
print(gia_pho > 40000)
```

Máy in ra:

```text title=readonly
True
```

Đọc dòng giữa từ trái sang phải: *"cái tên `gia_pho` đang giữ giá trị 45000 —
nó có lớn hơn 40000 không?"* Máy so, thấy đúng, và đưa lại `True`.

Dấu `>` không tính toán gì cả. Nó **hỏi**. Còn đây là cả bộ dấu để hỏi:

| dấu | đọc là | ví dụ cho `True` |
|---|---|---|
| `>` | lớn hơn | `45000 > 40000` |
| `<` | nhỏ hơn | `25000 < 45000` |
| `>=` | lớn hơn hoặc bằng | `45000 >= 45000` |
| `<=` | nhỏ hơn hoặc bằng | `45000 <= 45000` |
| `==` | có bằng nhau không | `45000 == 45000` |
| `!=` | có khác nhau không | `45000 != 30000` |

Chú ý dòng áp chót. Một dấu `=` và hai dấu `=` là hai việc hoàn toàn khác nhau:

- `gia_pho = 45000` — **dán** cái tên `gia_pho` lên giá trị 45000. Đây là một
  mệnh lệnh.
- `gia_pho == 45000` — **hỏi** xem thứ mà `gia_pho` đang giữ có bằng 45000
  không. Đây là một câu hỏi, và nó trả về `True` hoặc `False`.

Gõ thiếu một dấu bằng là lỗi mà cả người đi làm mười năm vẫn mắc. Không phải vì
khó, mà vì hai câu đó nhìn gần giống nhau.
::::

::::predict{#du-tien-khong commitOnce}
Byte sắp chạy đoạn dưới. **Trước khi bấm chạy**, bạn đoán nó in ra gì?

```python title=readonly
tien_trong_vi = 30000
gia_pho = 45000
print(tien_trong_vi >= gia_pho)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn đã nhìn ra 45000 và 30000 chênh nhau, và một trong hai chiều
là đúng. Nhưng chiều nào mới là chiều máy đang hỏi?

Máy đọc từ trái sang phải, và vế **bên trái** là chủ ngữ của câu hỏi:
*"30000 có lớn hơn hoặc bằng 45000 không?"* Ba mươi nghìn không mua nổi tô phở
bốn mươi lăm nghìn, nên câu trả lời là `False`.

Đổi chỗ hai vế thành `gia_pho >= tien_trong_vi` thì mới ra `True` — nhưng lúc
đó bạn đang hỏi một câu khác hẳn.
::
:::

:::opt
-15000
::why
Gần đúng ở chỗ bạn nhận ra máy phải đem hai con số ra so với nhau, và đúng là
trong ruột máy việc so sánh có liên quan tới chuyện lấy số này trừ số kia.

Nhưng thứ máy **đưa lại cho bạn** không phải khoảng cách giữa hai số. Nó là câu
trả lời cho một câu hỏi có–không. Muốn biết thiếu bao nhiêu tiền thì phải hỏi
bằng dấu `-`, còn `>=` chỉ trả lời đúng hoặc sai.
::
:::

:::opt
Không in ra gì cả, vì đây là một câu hỏi chứ không phải một câu chữ
::why
Gần đúng ở chỗ bạn nhận ra dòng đó là một câu hỏi — nhận ra được điều này là đã
hiểu đúng phần khó nhất rồi.

Chỗ lệch nằm ở bước sau: máy trả lời câu hỏi đó bằng một **giá trị**, y như
`45000` là một giá trị. Mà `print` thì in ra được mọi giá trị đưa cho nó. Nên
màn hình vẫn hiện một dòng.
::
:::
::::

::::explain{#mot-kieu-moi}
Bạn vừa gặp một kiểu giá trị mới.

Trước đây bạn có chữ (`"Phở Thìn"`) và số (`45000`). Giờ có thêm loại thứ ba,
loại chỉ có đúng hai giá trị trong toàn bộ thế giới: `True` và `False`.

Thử hỏi máy xem nó gọi loại này là gì:

```python title=readonly
print(type(45000 > 40000))
```

Máy trả lời `<class 'bool'>`. Chữ `bool` là tên rút gọn của loại giá trị hai
trạng thái này. Bạn chưa cần nhớ tên đó. Cái cần nhớ là: **so sánh không cho ra
số, nó cho ra một câu trả lời.**
::::

::::code{#khach-du-tien}
Tô phở giá 45000. Ba người khách lần lượt bước vào và đưa ba số tiền khác nhau.
Hãy khiến máy trả lời cho **cả ba** cùng một câu hỏi: *tiền khách đưa có đủ trả
tô phở không?*

Người thứ ba đưa vừa đúng 45000. Ở quán phở, đưa vừa đủ thì vẫn là đủ — câu trả
lời cho người đó phải là `True`.

Ba chỗ trống điền **giống hệt nhau** — vẫn là một câu hỏi đó. Thứ thay đổi nằm ở
dòng ngay phía trên: cái tên `tien_khach_dua` được dán lại lên một số khác.

```python title=starter
gia_pho = 45000

tien_khach_dua = 50000
print(___)

tien_khach_dua = 30000
print(___)

tien_khach_dua = 45000
print(___)
```

```python title=solution
gia_pho = 45000

tien_khach_dua = 50000
print(tien_khach_dua >= gia_pho)

tien_khach_dua = 30000
print(tien_khach_dua >= gia_pho)

tien_khach_dua = 45000
print(tien_khach_dua >= gia_pho)
```

```python title=test
# Chấm bằng OUTPUT, và cố tình chấm trên BA tình huống chứ không một.
#
# Với một tình huống, câu trả lời đúng chỉ có thể là `True` hoặc `False` — nên
# người gõ bừa một trong hai từ đó vào chỗ trống cũng xanh mà không hiểu gì.
# Mỗi tình huống thêm vào lại loại đi một cách qua bài nhờ ăn may:
#
#   50000 → True   : loại `False`, `0`
#   30000 → False  : loại `True`, `1`, và mọi hằng số (ba dòng sẽ giống hệt nhau)
#   45000 → True   : loại `>` — chỗ duy nhất `>` và `>=` khác nhau là khi bằng
#
# Dòng cuối mới là dòng dạy được điều bài này muốn dạy. Thiếu nó thì `>` và
# `>=` không phân biệt được, mà `>` là đáp án SAI: đưa vừa đủ tiền vẫn là đủ.
#
# `match: regex` là kiểu chấm duy nhất viết ra được ràng buộc theo THỨ TỰ dòng;
# `contains` chỉ nhìn thấy một mẩu nên `<` viết ngược vẫn lọt.
pass
```

:::hints
- kind: attention
  body: Chỗ trống cần một **câu hỏi**, không phải một con số và cũng không phải chữ `True` gõ thẳng vào. Câu hỏi đó so hai cái tên có sẵn với nhau — và nó giống nhau ở cả ba chỗ trống.
- kind: strategy
  body: "Đủ tiền" nghĩa là tiền khách đưa lớn hơn giá, hoặc vừa đúng bằng giá. Người khách thứ ba rơi đúng vào vế thứ hai, nên dấu bạn chọn phải gộp được cả hai — trong bảng dấu ở trên, dấu nào làm được? Nhớ rằng vế bên trái là chủ ngữ của câu hỏi, nên bên trái phải là tiền khách đưa.
- kind: one-line
  body: "Viết `tien_khach_dua >= gia_pho` vào **cả ba** chỗ trống. Máy sẽ in `True`, `False`, rồi `True`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^True\nFalse\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một câu hỏi, ba tình huống, ba câu trả lời. Từ đây máy bắt đầu tự quyết định được.
::::

::::reflect{#nghi-lai}
Bạn đã có `True` và `False` trong tay. Nhưng để ý mà xem: máy mới chỉ **nói ra**
câu trả lời, chứ chưa **làm gì khác đi** vì câu trả lời đó.

Khách đủ tiền thì in "Mời vào", không đủ thì đừng in. Làm sao bảo máy: *chỉ chạy
dòng này khi câu trả lời là `True`*?

Đừng trả lời vội. Bài sau là đúng một câu lệnh để làm việc đó.
::::

::::checkpoint{mastery=0.8}
::::
