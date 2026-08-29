---
id: nen-tang.list-dict-set-tuple.hai-day-di-song-song
title: Hai dãy đi cạnh nhau
summary: Hai danh sách song song nối nhau bằng một chỉ số chung — và sợi dây ấy chỉ nằm trong đầu người viết, máy không hề biết nó có.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.parallel-lists]
requires: [core.list, core.list-index, core.list-append, core.list-remove, core.list-membership, core.len, ctrl.for-range, ctrl.for-each, core.fstring, core.output, err.type-error]
concepts: [core.danh-sach, core.chi-so, ctrl.lap]
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
Tên khoản mình vẫn đọc thành lời. Nhưng chỗ mình cất nó thì máy không đọc được.
::::

::::explain{#ten-khoan-dang-nam-o-dau}
Bài trước kết bằng một câu hỏi thật thà: cuốn sổ chỉ toàn số, vậy **tên** khoản
bạn đang ghi ở đâu?

Câu trả lời hơi khó chịu: trong đầu bạn. Bạn nhìn `so[2]` và đọc thành "khoản
sửa xe" vì bạn nhớ hôm ấy mình ghi gì. Máy nhìn `so[2]` và thấy con số 500000.
Không hơn.

Ý đầu tiên ai cũng nghĩ tới là nhét tên vào thẳng cuốn sổ, xen kẽ:

```python title=readonly
so = ["ăn sáng", 85000, "sửa xe", 500000]
```

Cuốn sổ này giữ được tên thật. Nhưng nó vừa làm hỏng mọi thứ đã dựng từ bài 1:

- Cộng tiền không cộng nổi. `so[0] + so[1]` là lấy một dãy chữ cộng với một con
  số, và máy trả lời bằng `TypeError` — đúng loại việc không làm được mà Realm 0
  đã cho bạn gặp.
- Muốn tính tổng, bạn phải nhớ luật ngầm "ô chẵn là tên, ô lẻ là tiền" và nhảy
  cách một ô một. Luật ấy không viết ở đâu cả.
- Và luật ấy vỡ ngay lần đầu có người xoá một khoản: bỏ `"sửa xe"` đi thì mọi ô
  phía sau dịch lên một chỗ, nên từ chỗ bị xoá trở về sau, luật ngầm đảo ngược —
  ô chẵn thành tiền, ô lẻ thành tên.

Nên cách người ta thật sự dùng là **hai cuốn sổ đi cạnh nhau**:

```python title=readonly
ten = ["ăn sáng", "sửa xe", "biếu bà", "đổ xăng"]
tien = [85000, 500000, 300000, 120000]
```

Hai danh sách, cùng độ dài, và một thoả thuận: **ô thứ `i` của dãy này đi với ô
thứ `i` của dãy kia**. `ten[1]` là `"sửa xe"`, `tien[1]` là `500000`, nên khoản
sửa xe hết 500 nghìn.

Sợi dây nối hai cuốn sổ, vì vậy, là **chỉ số chung**. Không phải một dòng lệnh,
không phải một cái tên — chỉ là một con số dùng chung cho cả hai bên.
::::

::::example{#di-hai-day-mot-luot}
Muốn đọc cả tên lẫn tiền trong cùng một lượt thì `for ten_khoan in ten` không đủ:
kiểu duyệt ấy đưa cho bạn giá trị của từng ô, mà không nói ô ấy là ô thứ mấy —
và chính con số thứ mấy mới là thứ mở được cuốn sổ bên kia.

Nên vòng lặp phải chạy trên **chỉ số**, đúng kiểu `range` mà Realm 0 đã dạy:

```python title=readonly
ten = ["ăn sáng", "sửa xe", "biếu bà", "đổ xăng"]
tien = [85000, 500000, 300000, 120000]

for i in range(len(ten)):
    print(f"{ten[i]}: {tien[i]} đồng")
```

Máy in ra:

```text title=readonly
ăn sáng: 85000 đồng
sửa xe: 500000 đồng
biếu bà: 300000 đồng
đổ xăng: 120000 đồng
```

Hai chỗ đáng nhìn kỹ:

- **`range(len(ten))` đếm theo dãy `ten`, không theo dãy `tien`.** Bạn chọn một
  trong hai dãy để đếm, và đó là một lựa chọn — nó chỉ vô hại chừng nào hai dãy
  còn dài bằng nhau.
- **`i` là thứ duy nhất biết cả hai bên.** Trong một lượt, `ten[i]` và `tien[i]`
  dùng chung đúng một con số ấy. Bỏ `i` ra khỏi một trong hai chỗ là mất luôn
  quan hệ giữa chúng.
::::

::::explain{#soi-day-chi-nam-trong-dau-ban}
Bây giờ tới phần quan trọng nhất của bài, và nó không phải một cú pháp mới.

Đọc lại đoạn code vừa rồi và tìm giúp: **dòng nào nói cho máy biết `ten` và
`tien` có liên quan tới nhau?**

Không có dòng nào cả.

Với máy, đó là hai danh sách độc lập, tình cờ nằm cạnh nhau trong cùng một file.
Nó không kiểm hai dãy có dài bằng nhau không. Nó không kiểm thứ tự hai bên có
khớp không. Thoả thuận "ô thứ `i` đi với ô thứ `i`" là thoả thuận giữa bạn với
chính bạn, và nó được giữ bằng đúng một thứ: sự cẩn thận của bạn mỗi lần đụng vào
một trong hai dãy.

Nghĩa là mọi thao tác từ nay phải làm **thành cặp**:

- Thêm một khoản thì phải `.append` vào cả hai dãy.
- Xoá một khoản thì phải xoá ở cả hai dãy, và xoá đúng ô tương ứng.

Quên một nửa thì hai dãy lệch nhau. Và đây là chỗ đau thật sự: bài 6 cho bạn thấy
một lỗi **ồn** — `.remove` không tìm thấy thì `ValueError` nổ ra, chương trình
dừng, bạn biết ngay. Còn hai dãy lệch nhau thì im lặng. Chương trình vẫn chạy,
báo cáo vẫn in ra đủ dòng, chỉ có điều mỗi dòng ghép sai tên với tiền.
::::

::::predict{#doan-khi-mot-day-ngan-di commitOnce}
Byte có ba khoản trong hai dãy song song. Byte xoá `"sửa xe"` khỏi dãy tên — và
quên mất dãy tiền.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python title=readonly
ten = ["ăn sáng", "sửa xe", "biếu bà"]
tien = [85000, 500000, 300000]

ten.remove("sửa xe")

for i in range(len(ten)):
    print(f"{ten[i]}: {tien[i]} đồng")
```

:::opt{correct}
Hai dòng: `ăn sáng: 85000 đồng` rồi `biếu bà: 500000 đồng`
:::

:::opt
Hai dòng: `ăn sáng: 85000 đồng` rồi `biếu bà: 300000 đồng`
::why
Gần đúng ở phần khó nhất: bạn thấy ngay dãy tên chỉ còn hai ô, nên vòng lặp chạy
hai lượt và màn hình có hai dòng. Chỗ đó bạn đọc chính xác.

Chỗ lệch là ở chỗ đứng của số 300000. Trong đầu bạn, 300000 dính với "biếu bà" —
đúng như lúc bạn ghi sổ. Nhưng dãy `tien` không hề biết chuyện ấy: với nó,
300000 chỉ là **ô số 2**. Sau khi `"biếu bà"` tụt từ ô số 2 lên ô số 1 ở dãy tên,
lượt thứ hai lấy `ten[1]` và `tien[1]` — và `tien[1]` vẫn đang là 500000, tiền
của khoản sửa xe.
::
:::

:::opt
Máy báo lỗi, vì hai dãy không còn dài bằng nhau
::why
Gần đúng ở chỗ bạn mong máy đứng về phía mình — hai dãy lệch nhau đúng là một
tình trạng hỏng, và một cái máy tử tế thì nên kêu lên.

Chỗ lệch: muốn kêu lên, máy phải biết hai dãy ấy có quan hệ với nhau đã. Mà
không dòng nào trong chương trình nói điều đó. Với máy, `ten` và `tien` là hai
danh sách rời, dài ngắn thế nào cũng là quyền của chúng. Vòng lặp chỉ chạy hai
lượt và hai lượt ấy đều lấy được ô hợp lệ ở cả hai bên, nên không có gì để báo.
::
:::

:::opt
Ba dòng, dòng cuối là `biếu bà: 300000 đồng`
::why
Gần đúng ở chỗ bạn nhớ rằng dãy tiền vẫn còn nguyên ba khoản — điều đó đúng, và
chính nó là gốc của rắc rối trong bài này.

Chỗ lệch là ở chỗ vòng lặp lấy số lượt từ đâu. `range(len(ten))` đếm theo dãy
**được gọi tên bên trong nó**, tức là `ten`, và dãy ấy vừa mất một ô nên chỉ còn
hai. Dãy `tien` dài bao nhiêu cũng không thêm được lượt nào. Nếu vòng lặp chạy
lượt thứ ba thật thì nó sẽ đòi `ten[2]` — một ô không còn tồn tại.
::
:::
::::

::::code{#bao-cao-tu-hai-day}
Byte cần một bản báo cáo: mỗi khoản một dòng, tên rồi tới tiền. Byte muốn dựng
sẵn cả bản báo cáo trong một danh sách chữ trước, in ra sau — như vậy sau này còn
đem đi làm việc khác được.

Đoạn dưới còn hở hai chỗ, và cả hai nằm trong cùng một câu chữ, cùng một lượt.

```python title=starter
ten = ["ăn sáng", "sửa xe", "biếu bà", "đổ xăng"]
tien = [85000, 500000, 300000, 120000]

dong_bao_cao = []
for i in range(len(ten)):
    dong_bao_cao.append(f"{___}: {___} đồng")

for dong in dong_bao_cao:
    print(dong)
```

```python title=solution
ten = ["ăn sáng", "sửa xe", "biếu bà", "đổ xăng"]
tien = [85000, 500000, 300000, 120000]

dong_bao_cao = []
for i in range(len(ten)):
    dong_bao_cao.append(f"{ten[i]}: {tien[i]} đồng")

for dong in dong_bao_cao:
    print(dong)
```

```python title=test
# Chấm trên cả bốn dòng chứ không chấm một dòng: ghép đúng ở dòng đầu mà lệch
# ở dòng sau là đúng cái lỗi bài này nói tới, và một phép so trên cả danh sách
# là cách duy nhất thấy được nó.
assert dong_bao_cao == [
    "ăn sáng: 85000 đồng",
    "sửa xe: 500000 đồng",
    "biếu bà: 300000 đồng",
    "đổ xăng: 120000 đồng",
], "mỗi dòng ghép ô thứ i của dãy ten với ô thứ i của dãy tien — bốn ô thì bốn dòng, và dòng thứ hai phải là sửa xe đi với 500000"
assert ten == ["ăn sáng", "sửa xe", "biếu bà", "đổ xăng"], "làm báo cáo là việc chỉ đọc: dãy tên phải còn nguyên bốn khoản, đúng thứ tự cũ"
assert tien == [85000, 500000, 300000, 120000], "dãy tiền cũng chỉ để đọc: bốn con số phải còn nguyên, đúng thứ tự cũ"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trong cùng một câu chữ và cùng một lượt vòng lặp. Ở lượt ấy chỉ có đúng một cái tên đang giữ con số "lượt này là lượt thứ mấy" — nhìn dòng `for` để tìm nó.
- kind: strategy
  body: Bên trái dấu hai chấm là tên khoản, bên phải là số tiền. Tên nằm trong dãy `ten`, tiền nằm trong dãy `tien`, và cả hai phải lấy ở **cùng một** chỗ đứng — nếu hai chỗ trống dùng hai chỗ đứng khác nhau thì báo cáo ghép sai tên với tiền mà máy không kêu gì cả.
- kind: one-line
  body: "Chỗ trống thứ nhất là `ten[i]`, chỗ trống thứ hai là `tien[i]`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: "sửa xe: 500000 đồng"
- tier: output
  expect: "đổ xăng: 120000 đồng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một chỉ số, hai cuốn sổ. Giờ mình đọc được cả tên lẫn tiền.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Báo cáo chạy ngon, và chừng nào bạn còn cẩn thận thì hai dãy còn khớp nhau.

Nhưng hãy hình dung một buổi chiều bạn vội. Bạn xoá `"sửa xe"` khỏi dãy tên mà
quên dãy tiền. Lượt tra sau, máy in tên một đằng tiền một nẻo — **mà không kêu
một tiếng**. Không `ValueError`, không dòng đỏ nào, chỉ có một bản báo cáo trông
hoàn toàn bình thường và nói sai.

Sợi dây nối tên với tiền đang nằm trong đầu bạn. Muốn **máy** cũng biết tên nào
dính tiền nào thì cất chúng thế nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
