---
id: nen-tang.gia-tri-bien-kieu.de-len-ten-co-san
title: Đè lên cái tên máy đã có
summary: Máy dọn sẵn hơn trăm cái tên trước khi bạn viết dòng đầu — và không khoá cái nào. Đè lên một cái thì nó im lặng nhận, rồi nổ ở chỗ khác.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.builtin-shadowing]
requires: [core.name-defined-order, core.name-lookup, core.assignment, core.reassign, core.int-cast, core.function-call, core.fstring, err.type-error, err.name-error]
concepts: [core.ten-co-san, core.loi-khi-chay]
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
`print` với `int` là mình dọn sẵn, trước khi bạn viết dòng đầu tiên. Dọn sẵn thôi — không khoá.
::::

::::explain{#ai-gan-print}
Bài trước để lại một câu hỏi: `print` — cái tên bạn gõ từ bài đầu Realm 0 —
**ai** gán nó?

Trả lời thẳng: **máy gán**, và gán xong trước khi nó đọc tới dòng đầu tiên
của bạn.

Hình dung quán phở lúc vừa mở cửa. Chưa có khách nào ngồi, nhưng trên mỗi bàn
đã có lọ tương, ống đũa, lọ ớt, hộp giấy ăn. Không ai bê chúng ra lúc bạn kéo
ghế — chúng nằm đó từ trước.

Bảng tên của chương trình cũng thế. Bài trước dạy rằng một cái tên chỉ có mặt
từ dòng gán trở xuống, nên dễ hình dung là lúc chạy dòng đầu thì bảng tên rỗng
trơn. Thật ra lúc đó nó đã có sẵn hơn một trăm dòng:

| tên | máy dán nó lên cái gì |
|---|---|
| `print` | công cụ in ra màn hình |
| `input` | công cụ hỏi người ngồi trước máy |
| `int` | công cụ đổi một giá trị thành số nguyên |
| `float` | công cụ đổi một giá trị thành số thực |
| `round` | công cụ làm tròn |
| `type` | công cụ cho biết nhãn kiểu |

Người ta gọi chúng là **tên có sẵn** — tiếng Anh là *built-in*, nghĩa đen là
"dựng sẵn bên trong". Đó là lý do `int("3")` chạy được ngay từ Realm 0 mà bạn
chưa bao giờ phải viết một dòng gán nào cho `int`.

Và đây là chỗ cả bài này xoay quanh: **dọn sẵn không có nghĩa là khoá lại.**
`int` nằm trên bảng tên đúng như `tien_ca_phe` bạn tự gán. Nên `int = 0` là
một dòng gán hợp lệ, đúng cái luật bạn đã học từ Realm 0: cái tên là mảnh giấy
dán, gỡ ra dán sang giá trị khác bao nhiêu lần cũng được. Máy không hỏi ai dán
mảnh giấy ấy trước.

Nên câu hỏi thứ hai của bài trước — viết `print = 5` thì sao — có một câu trả
lời ngắn:

```python title=readonly
print("Sổ tháng 8")
print = 5
print("Cà phê 25000đ")
```

```text
Sổ tháng 8
Traceback (most recent call last):
  File "so_chi_tieu.py", line 3, in <module>
    print("Cà phê 25000đ")
    ~~~~~^^^^^^^^^^^^^^^^^
TypeError: 'int' object is not callable
```

Dòng 1 in ra bình thường. Dòng 2 chạy êm, không một tiếng cảnh báo. Dòng 3 mới
nổ — và từ giây phút ấy, chương trình mất luôn đường in ra màn hình.

Đó là hình dạng chung của cả bài này, nên hãy nhìn kỹ nó một lần nữa với một
cái tên bạn dùng nhiều hơn.
::::

::::example{#may-khong-keu-mot-tieng}
Byte đang ghi sổ chi tiêu. Byte muốn một cái tên giữ **số khoản đã ghi từ đầu
tháng**, và trong lúc nghĩ tên thì gõ đại một từ ngắn:

```python title=readonly
tien_ca_phe = 25000
int = 0

print(f"Cà phê {tien_ca_phe}đ")
print(f"Đã ghi {int} khoản trước đó")

o_so_nguoi = "3"
so_nguoi = int(o_so_nguoi)
print(f"Chia {so_nguoi} người")
```

Chạy lên:

```text
Cà phê 25000đ
Đã ghi 0 khoản trước đó
Traceback (most recent call last):
  File "so_chi_tieu.py", line 8, in <module>
    so_nguoi = int(o_so_nguoi)
TypeError: 'int' object is not callable
```

Đọc chậm ba chỗ.

**Chỗ thứ nhất — dòng 2 chạy êm.** `int = 0` không bị kêu một tiếng nào. Không
cảnh báo, không gạch chân, không gì cả. Máy làm đúng việc bạn nhờ: gỡ nhãn
`int` khỏi công cụ đổi kiểu, dán nó lên số `0`.

**Chỗ thứ hai — hai dòng đầu vẫn in ra.** Chương trình sống khoẻ qua bốn dòng
nữa. Nếu đây là bài nộp và bạn liếc màn hình thấy hai dòng đúng, bạn sẽ tưởng
mọi thứ ổn.

**Chỗ thứ ba — nó nổ ở dòng 8.** `TypeError` bạn đã quen từ Realm 0: hai kiểu
không đi cùng nhau được trong việc này. Ở đây việc ấy là **gọi**. Dấu ngoặc
đặt sau một cái tên có nghĩa "chạy công cụ này giúp tôi", mà `int` bây giờ
đang giữ số `0`. Số `0` thì không chạy được. Chữ *callable* nghĩa là "gọi
được"; `'int' object is not callable` dịch sát là *cái mà `int` đang giữ thì
không phải thứ gọi được*.

Cái đắt của bài này nằm ở **khoảng cách**: dòng gây ra chuyện là dòng 2, dòng
nổ là dòng 8. Sáu dòng ở giữa hoàn toàn vô can, và thông báo lỗi chỉ tay vào
dòng 8. Với một chương trình hai trăm dòng, khoảng cách ấy là hai trăm dòng.
::::

::::predict{#doan-no-o-dong-nao commitOnce}
Byte tính trung bình mỗi ngày tiêu bao nhiêu, và cũng đặt tên biến bằng một từ
ngắn. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
tien = 45000
float = 0.0
print("Đang tính trung bình")
print(float(tien))
```

:::opt{correct}
In ra `Đang tính trung bình`, rồi mới báo TypeError ở dòng cuối
:::

:::opt
Máy báo lỗi ngay tại dòng `float = 0.0`, vì đó là tên của máy
::why
Gần đúng ở chỗ bạn nhận ra `float` không phải một cái tên bình thường: nó là
tên máy dán sẵn, và một dòng đè lên nó thì đáng bị chặn thật.

Chỗ lệch nằm ở chỗ Python không giữ danh sách nào để chặn. Với nó, `float = 0.0`
có hình dạng y hệt `tien = 45000`: một cái tên bên trái, một giá trị bên phải.
Dòng ấy chạy trót lọt, và cả chương trình đi tiếp như chưa có chuyện gì.
::
:::

:::opt
In ra `Đang tính trung bình` rồi `45000.0`
::why
Gần đúng ở chỗ bạn nhớ chính xác việc `float()` làm: nhận một giá trị và trả
về số thực của nó, nên `float(45000)` cho `45000.0`. Nếu dòng 2 không có mặt,
bạn đã đoán trúng cả hai dòng.

Chỗ lệch: dòng 2 có mặt. Sau nó, cái tên `float` không còn dán trên công cụ
đổi kiểu nữa — nó đang dán trên số `0.0`. Máy đọc `float(tien)` thì tra bảng
tên, thấy `0.0`, và số `0.0` không phải thứ gọi được.
::
:::

:::opt
Máy báo NameError ở dòng cuối, vì công cụ `float` đã bị xoá mất
::why
Gần đúng ở chỗ bạn thấy đúng thiệt hại: sau dòng 2, chương trình này không còn
đường nào gọi tới công cụ đổi số thực nữa.

Chỗ lệch nằm ở loại lỗi. `NameError` dành cho cái tên **chưa từng được gán** —
đúng thứ bài trước vừa dạy. Ở đây `float` có mặt trên bảng tên hẳn hoi, chỉ là
nó đang giữ `0.0`. Cái tên còn, thứ nó giữ mới là cái đã đổi — nên máy trả lời
bằng `TypeError` chứ không phải `NameError`.
::
:::
::::

::::explain{#nhin-ra-truoc-khi-no}
Hai bài liền nhau, hai mức nghiêm khắc khác hẳn:

- Tên **chưa từng gán** mà đem dùng: máy **báo ngay**, đúng dòng, đúng lúc.
- Tên **có sẵn** mà bạn đè lên: máy **im lặng**, rồi nổ ở một chỗ khác, có khi
  cách đó hai trăm dòng.

Cái im lặng ấy không phải máy sơ suất. Có những chương trình thật sự cần dán
lại một cái tên có sẵn, và Python để cửa mở cho việc đó. Cái giá của cửa mở là
bạn phải tự trông.

Trông bằng hai thói quen nhỏ:

**Thử tên trước khi lấy.** Trước khi lấy một từ tiếng Anh ngắn làm tên biến,
gõ thử `print(<từ đó>)` ở một dòng riêng rồi chạy.

```python title=readonly
print(round)
print(data)
```

```text
<built-in function round>
Traceback (most recent call last):
  File "thu_ten.py", line 2, in <module>
    print(data)
NameError: name 'data' is not defined
```

`round` in ra một dòng mô tả công cụ — nghĩa là chỗ đó đã có người ở, lấy tên
ấy là đè. `data` cho `NameError` — nghĩa là chỗ đó trống, tên ấy dùng được.

**Nhận ra chữ *callable* trong thông báo.** Hễ thấy `... object is not callable`
thì đọc luôn cái tên đứng ngay trước nó, rồi tìm ngược lên xem trong file có
dòng nào gán cho cái tên ấy không. Chín trên mười lần, thủ phạm nằm ở đó.

Và cách gỡ luôn là cách rẻ nhất: **đổi tên biến của bạn**, đừng cố giành lại
tên của máy.
::::

::::code{#tra-lai-ten-cho-may}
Byte gõ hai ô trên phiếu chi vào máy: ô tiền phở và ô tiền cà phê, cả hai đều
là chữ vì chúng vừa được gõ từ bàn phím. Byte đổi cả hai sang số rồi cộng lại.

Chỗ trống là **tên của cái biến giữ tiền phở**. Nó xuất hiện hai lần: một lần
bên trái dấu `=`, một lần trong f-string ở dòng cuối. Cả hai chỗ điền cùng một
tên.

Byte định gõ `int` vào cả hai chỗ cho ngắn gọn. **Cứ thử đúng như vậy trước,
bấm chạy và đọc thông báo** — rồi thay bằng một cái tên nói ra nội dung nó
giữ: `tien_pho`.

```python title=starter
o_tien_pho = "45000"
o_tien_ca_phe = "25000"

___ = int(o_tien_pho)
tien_ca_phe = int(o_tien_ca_phe)

print(f"Tổng: {___ + tien_ca_phe}đ")
```

```python title=solution
o_tien_pho = "45000"
o_tien_ca_phe = "25000"

tien_pho = int(o_tien_pho)
tien_ca_phe = int(o_tien_ca_phe)

print(f"Tổng: {tien_pho + tien_ca_phe}đ")
```

```python title=test
# Ba câu hỏi cho ba chuyện khác nhau.
# Hai câu đầu: hai ô trên phiếu đã thành số đúng và nằm trong hai cái tên riêng
# — ai xoá bớt một dòng cho hết lỗi sẽ trượt ở đây.
# Câu thứ ba: cái tên `int` phải còn nguyên là công cụ đổi kiểu. Ai điền `int`
# vào chỗ trống thì đã nổ từ dòng 5; ai lách qua được dòng đó bằng một đường
# vòng nào khác mà vẫn đè lên `int` thì trượt đúng câu này.
assert tien_pho == 45000, "ô tiền phở trên phiếu ghi 45000, và con số ấy phải nằm ở cái tên của riêng nó"
assert tien_ca_phe == 25000, "ô tiền cà phê ghi 25000, và nó là một khoản riêng chứ không dồn chung với tiền phở"
assert int("7") == 7, "int vẫn phải là công cụ đổi chữ thành số — ai lấy cái tên ấy đặt cho tiền của mình thì máy hết đường đọc ô tiếp theo"
```

:::hints
- kind: attention
  body: Điền `int` vào rồi chạy thử đi đã. Máy chỉ tay vào dòng 5, nhưng dòng 5 không có gì sai — cái tên bạn vừa đặt ở dòng 4 và công cụ mà dòng 5 đang gọi là cùng một cái tên.
- kind: strategy
  body: Một cái tên chỉ giữ được một thứ tại một lúc. Sau dòng 4, `int` giữ số 45000 nên nó thôi làm công cụ, mà dòng 5 thì vẫn cần công cụ ấy. Đừng giành tên của máy — chọn cho giá trị của bạn một cái tên còn trống, và tên ấy nên nói ra thứ nó giữ.
- kind: one-line
  body: "Viết `tien_pho` vào cả hai chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Tổng: 70000đ
:::
::::

::::byte{trigger=success mood=happy pose=jump}
70000. Tên của bạn trả về cho bạn, tên của mình trả về cho mình — ai làm việc nấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Máy để bạn đè lên `int` mà không kêu một tiếng. Nó cũng để bạn đè lên `print`,
lên `round`, lên `type` — hơn một trăm cái tên, không cái nào được rào lại.

Nghe như máy nhận mọi cái tên bạn nghĩ ra. Vậy thử hỏi ngược: nó **có** từ
chối cái tên nào không?

Thử một dòng này xem:

```python
class = "ăn ngoài"
```

`class` cũng chỉ là năm chữ cái tiếng Anh, y như `print`. Nhưng lần này máy
không im lặng — và nó cũng không đợi tới lúc chạy tới dòng đó mới nói.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
