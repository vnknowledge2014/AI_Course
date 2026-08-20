---
id: onboarding.may-tinh-noi-gi.dat-ten-cho-gia-tri
title: Đặt tên cho một giá trị
summary: Dấu bằng không có nghĩa là bằng nhau — nó dán một cái tên lên một giá trị.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.variable, core.assignment]
requires: [core.string-concat, core.name-lookup]
concepts: [core.bien, core.gan]
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
Bài 8 mình đi tìm một cái tên và không thấy. Hôm nay bạn cho mình cái để tìm.
::::

::::explain{#goi-tat-trong-bep}
Trong bếp quán phở, không ai đọc hết câu *"tô phở bò tái nạm nước trong hành
trần"* mỗi lần có khách gọi. Chị Hạnh viết lên bảng một dòng:

> MÓN 1 = phở bò tái nạm nước trong hành trần

Từ lúc đó, cả bếp chỉ nói "một món 1". Ba chữ ngắn thay cho cả câu dài.

Hãy để ý dòng trên bảng làm được ba việc cùng lúc:

- Câu dài chỉ phải viết ra **một lần**, nên cũng chỉ có một lần để viết nhầm.
- Ai nói "món 1" cũng đang nói về đúng cùng một thứ.
- Muốn đổi món 1 thành món khác thì sửa **một chỗ** trên bảng, không phải đi sửa
  từng lời từng người.

Python có đúng một dấu để làm việc dán tên đó. Nó là dấu `=`.
::::

::::example{#doc-tu-phai-sang}
Đây là dòng bảng của chị Hạnh, viết bằng Python:

```python title=readonly
mon_dac_biet = "Phở bò tái nạm"
```

Dòng này đừng đọc từ trái sang phải. Đọc **từ phải sang trái**:

1. Bên phải dấu `=` là **giá trị**: chuỗi `"Phở bò tái nạm"` — có nháy, nên là
   chữ, y như mọi bài trước.
2. Dấu `=` là một **hành động**: dán một cái tên lên giá trị bên phải.
3. Bên trái là **cái tên** do bạn tự đặt: `mon_dac_biet`. Nó viết **không nháy**
   — đúng luật bài 8: trong nháy là chữ, không nháy là tên.

Chỗ dễ vấp nhất của cả bài nằm ở dấu `=`. Trong toán, `x = 5` là một lời khẳng
định: hai vế bằng nhau. Ở đây thì không. Ở đây `=` là một **mệnh lệnh**, đọc
thành: *"lấy cái tên bên trái, dán lên giá trị bên phải"*.

Việc dán tên này có tên riêng: **gán**. Và một cái tên đã được gán như vậy gọi
là một **biến**.

Về cách đặt tên: dùng chữ cái, chữ số và dấu gạch dưới, không mở đầu bằng chữ
số, không có dấu cách ở giữa. Python cho phép viết tên có dấu tiếng Việt, nhưng
cả nghề quen viết không dấu và nối các chữ bằng gạch dưới — `mon_dac_biet` — nên
bạn theo lệ đó, sau này đọc code người khác sẽ thấy quen mắt.
::::

::::example{#may-lam-trong-im-lang}
Chạy thử hai dòng:

```python title=readonly
mon_dac_biet = "Phở bò tái nạm"
print("Đã ghi vào sổ")
```

Máy in ra đúng một dòng:

```text title=readonly
Đã ghi vào sổ
```

Dòng gán **không in ra gì cả**. Nó không phải một dòng nói, nó là một dòng ghi
nhớ. Máy nhận cái tên, cất giá trị lại, rồi im lặng đi tiếp.

Từ bài 1 tới giờ, mọi thứ hiện lên màn hình đều phải đi qua `print`. Dòng gán
không có `print` nào nên nó câm lặng — dù bên trong máy vừa có một thay đổi thật.

Còn một chuyện nữa, và Byte để lửng ở đây: cái tên `mon_dac_biet` đang giữ câu
chữ đó, nhưng bạn chưa có cách nào bảo máy đọc nó ra. Hai bài nữa bạn sẽ có.
::::

::::predict{#doan-may-noi-gi commitOnce}
Byte sắp chạy một chương trình dài đúng một dòng, không có gì thêm.

```python title=readonly
tien_mot_to = 45000
```

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

:::opt{correct}
Không hiện ra gì cả
:::

:::opt
45000
::why
Gần đúng ở chỗ bạn tin rằng máy có làm gì đó với con số 45000 — và nó làm thật:
nó cất con số ấy lại rồi dán cái tên `tien_mot_to` lên.

Chỗ lệch nằm ở hai chữ "hiện ra". Máy chỉ nói ra khi bạn bảo nó nói, và cách bảo
duy nhất bạn đã biết là `print`. Dòng này không có `print`, nên việc xảy ra hết
ở bên trong máy, không có gì lên màn hình.
::
:::

:::opt
Máy báo `NameError`, vì `tien_mot_to` chưa từng tồn tại
::why
Gần đúng, và đây là suy nghĩ sắc — bạn đang dùng đúng luật bài 8: gặp một cái
tên không nháy, máy đi tìm; không thấy thì `NameError`.

Chỗ lệch: luật đó chỉ áp cho cái tên mà máy phải **đi tìm**. Cái tên đứng bên
trái dấu `=` thì ngược lại — đó chính là dòng khai sinh ra nó. Máy không đi tìm,
máy đặt tên.

Và kể từ sau dòng này, nếu có chỗ nào cần đi tìm `tien_mot_to`, máy sẽ tìm thấy.
::
:::

:::opt
Máy báo lỗi vì `45000` không có dấu nháy
::why
Gần đúng ở chỗ bạn để mắt tới dấu nháy trước tiên — thói quen rất tốt, và nó sẽ
cứu bạn nhiều lần về sau.

Chỗ lệch: bài trước đã cho thấy chữ số viết không nháy là một con số đàng hoàng,
máy làm việc được với nó. Ở dòng này không có gì sai cả — máy nhận con số, dán
tên lên, rồi im lặng.
::
:::
::::

::::explain{#vi-sao-dat-ten}
Đặt tên nhìn thì nhỏ, nhưng nó đổi hẳn cách một chương trình được viết. Ba lý
do, xếp theo thứ tự quan trọng dần:

- **Gõ một lần.** Câu `"Phở bò tái nạm"` chỉ xuất hiện đúng một chỗ trong cả
  chương trình, nên cũng chỉ có đúng một chỗ để gõ nhầm.
- **Sửa một chỗ.** Quán tăng giá thì bạn sửa đúng dòng gán, cả chương trình theo
  đó mà đổi. Chuyện này chỉ thành thật khi bạn biết cách dùng lại cái tên — hai
  bài nữa.
- **Nói ra ý nghĩa.** Con số `45000` đứng trơ trọi giữa chương trình thì không ai
  biết nó là tiền phở, số nhà hay số điện thoại rút gọn. `tien_mot_to = 45000`
  thì đọc một cái là hiểu.

Lý do thứ ba mới là lý do chính. Người đọc chương trình của bạn nhiều nhất
không phải máy — mà là bạn, ba tháng sau.
::::

::::code{#ghi-gia-vao-so}
Quán vừa tăng giá tô đặc biệt lên 65.000đ. Hãy đặt cho con số đó cái tên
`gia_dac_biet`.

Dòng `print` bên dưới Byte viết sẵn, bạn không phải sửa. Nó chỉ để bạn biết
chương trình đã chạy tới cuối — vì như bạn vừa thấy, dòng gán không nói gì.

```python title=starter
___ = 65000
print("Đã ghi giá mới")
```

```python title=solution
gia_dac_biet = 65000
print("Đã ghi giá mới")
```

```python title=test
# Bài này chấm bằng TESTS chứ không chỉ bằng OUTPUT: thứ cần kiểm là cái tên có
# tồn tại và có đang giữ đúng con số hay không — mà điều đó không hiện lên màn
# hình. Người học không nhìn thấy khối này; nó hỏi máy một câu mà bài học chưa
# dạy cách hỏi.
assert gia_dac_biet == 65000
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên TRÁI dấu `=`. Bên trái dấu `=` luôn là một cái tên, không bao giờ là một giá trị.
- kind: strategy
  body: Cái tên cần dùng đã có sẵn trong đề bài. Viết đúng nó ra, và đừng bỏ vào dấu nháy — nháy là dành cho chữ, còn đây là tên.
- kind: one-line
  body: Viết `gia_dac_biet` vào chỗ trống, giữ nguyên phần còn lại của dòng.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Đã ghi giá mới
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giờ trong máy có một cái tên do bạn đặt. Mình tìm là thấy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái tên `gia_dac_biet` đang được dán lên con số 65000. Giả sử tuần sau quán hạ
giá, và bạn viết thêm một dòng nữa ngay bên dưới:

```text
gia_dac_biet = 60000
```

Máy có dừng lại kêu "cái tên này có rồi" không?

Nếu nó không kêu, thì con số 65000 lúc nãy đi đâu? Máy giữ cả hai con số dưới
cùng một cái tên, hay nó bỏ mất một con?

Đừng trả lời vội. Bài sau chạy đúng hai dòng này.
::::

::::checkpoint{mastery=0.8}
::::
