---
id: onboarding.ra-lenh-cho-byte.hai-dieu-kien-cung-dung
title: Hai điều kiện cùng đúng
summary: Một từ nối buộc hai câu hỏi có–không lại, và chỉ gật khi cả hai cùng đúng.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [logic.and]
requires: [ctrl.if, core.boolean]
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
Một lối rẽ, hai điều kiện. Có đúng một từ để buộc chúng lại với nhau.
::::

::::explain{#to-giay-doi-hai-thu}
Đây là tờ giấy mà bài trước để lại cho bạn:

> Học sinh đến trước 8 giờ: giảm 5000 đồng.

Người bán hàng đọc tờ này rồi kiểm hai thứ, kiểm cái nào trước cũng được:

- Bạn này có phải học sinh không?
- Bây giờ có sớm hơn 8 giờ không?

Chỉ khi **cả hai** câu trả lời đều là "có" thì mới giảm. Học sinh mà 9 giờ mới
tới: không giảm. Đến từ 7 giờ nhưng không phải học sinh: cũng không giảm.

Nhớ lại bài *Đúng hay sai*: mỗi câu hỏi có–không cho ra một giá trị `True` hoặc
`False`. Bây giờ bạn có hai giá trị như thế trong tay, mà chỗ giữa `if` và dấu
hai chấm chỉ vừa cho **một**.

Việc cần làm là gộp hai câu trả lời thành một câu trả lời. Từ làm việc đó là
`and` — tiếng Anh nghĩa là "và".
::::

::::explain{#bang-bon-dong}
`and` đứng giữa hai giá trị đúng/sai và cho ra một giá trị đúng/sai. Nó chỉ có
bốn trường hợp, và bốn trường hợp này là toàn bộ luật của nó:

| vế trái | vế phải | `and` cho ra |
|---|---|---|
| `True` | `True` | `True` |
| `True` | `False` | `False` |
| `False` | `True` | `False` |
| `False` | `False` | `False` |

Nhìn cột cuối: trong bốn dòng chỉ có **một** dòng cho ra `True` — dòng mà cả hai
vế cùng đúng. Ba dòng còn lại đều `False`.

Nói theo kiểu quán phở: người bán chỉ gật khi bạn có đủ cả hai thứ. Thiếu một
thứ hay thiếu cả hai thì, với người bán, cũng như nhau — đều là không giảm.
::::

::::example{#giam-gia-buoi-sang}
Tờ giấy dán tường viết bằng Python:

```python title=readonly
la_hoc_sinh = True
gio_den = 7

if la_hoc_sinh and gio_den < 8:
    print("Giảm 5000 đồng")
else:
    print("Giá thường")
```

Máy in ra đúng một dòng: `Giảm 5000 đồng`.

Tập đọc dòng `if` này thành ba bước nhỏ, vì đó là cách gỡ rối nhanh nhất khi
một điều kiện chạy không như ý:

1. `la_hoc_sinh` — cái tên này đang giữ sẵn `True` từ dòng đầu. Nó **đã là** một
   câu trả lời có–không rồi, không cần so sánh gì thêm.
2. `gio_den < 8` — tức là `7 < 8`, cho ra `True`.
3. `True and True` — theo bảng bốn dòng ở trên, cho ra `True`.

Tới lúc này `if` chỉ còn nhìn thấy đúng một giá trị: `True`. Nó làm việc quen
thuộc của mình — chạy khối thụt vào bên dưới.

Máy làm gọn cả ba bước trong một lần đọc dòng. Nhưng khi bạn ngồi tìm xem điều
kiện sai ở đâu, tách ra như trên là cách để biết vế nào mới là vế hỏng.
::::

::::predict{#mot-ve-sai commitOnce}
Vẫn đoạn code đó, đổi đúng một con số: bạn học sinh này 9 giờ mới tới quán.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
la_hoc_sinh = True
gio_den = 9

if la_hoc_sinh and gio_den < 8:
    print("Giảm 5000 đồng")
else:
    print("Giá thường")
```

:::opt{correct}
Chỉ một dòng: Giá thường
:::

:::opt
Giảm 5000 đồng
::why
Gần đúng ở chỗ bạn kiểm vế trái và thấy nó đúng — bạn này là học sinh thật, một
nửa điều kiện đã đạt. Với một tờ giấy dán tường ngoài đời, nhiều người bán sẽ
gật đầu cho qua.

Chỗ lệch: `and` không nhân nhượng nửa nào. Vế phải là `9 < 8`, cho ra `False`.
Tra bảng bốn dòng: `True and False` cho ra `False`. Một vế sai là cả câu sai, và
máy đi thẳng xuống `else`.

Đây chính là chỗ đáng nhớ nhất của `and`: nó khắt khe. Muốn dễ tính hơn thì phải
dùng một từ khác — và đó là bài sau.
::
:::

:::opt
Cả hai dòng: Giảm 5000 đồng, rồi Giá thường
::why
Gần đúng ở thói quen đọc từ trên xuống: dòng nào có trên trang thì rồi cũng chạy
tới. Với mọi bài trước khi học `if`, điều đó luôn đúng.

Chỗ lệch: `if` và `else` là hai lối rẽ của **cùng một** ngã ba, không phải hai
việc xếp nối nhau. Máy đi đúng một lối rồi ra khỏi ngã ba, không bao giờ đi cả
hai. Muốn hai dòng cùng in ra thì chúng phải nằm trong cùng một khối.
::
:::

:::opt
Máy báo lỗi vì `la_hoc_sinh` đứng một mình, không có dấu so sánh nào
::why
Gần đúng ở chỗ bạn để ý một điều có thật: mọi điều kiện bạn từng thấy tới giờ
đều có một dấu so sánh — `>`, `<`, hay `==`. Nhận ra khuôn mẫu như vậy là thói
quen đọc code rất tốt.

Chỗ lệch: so sánh chỉ là một **cách tạo ra** `True` / `False`, không phải cách
duy nhất. Dòng `la_hoc_sinh = True` đã đặt sẵn giá trị `True` vào cái tên đó,
nên nó dùng thẳng làm điều kiện được, không cần chế biến thêm.

Viết `la_hoc_sinh == True` thì máy cũng chạy đúng, nhưng đó là đi hỏi lại một
câu đã có sẵn câu trả lời trong tay.
::
:::
::::

::::explain{#hai-cho-hay-vap}
Hai chỗ hay vấp khi mới dùng `and`:

- **Mỗi vế phải là một câu hỏi đủ đầu đủ đuôi.** Muốn nói "giờ nằm giữa 6 và 8",
  đừng viết `gio_den > 6 and < 8` — vế phải cụt mất bên trái, máy báo
  `SyntaxError`. Viết đủ hai lần: `gio_den > 6 and gio_den < 8`.
- **Nối ba vế cũng được.** `a and b and c` đòi cả ba cùng đúng. Cứ thêm một
  `and` là thêm một điều kiện bắt buộc, và mỗi lần thêm thì cái `if` đó lại khó
  vào hơn một chút.

> Mẹo đọc: gặp `and`, hãy đọc thành *"và phải"*. `if la_hoc_sinh and gio_den < 8`
> đọc là *"nếu là học sinh, **và phải** đến trước 8 giờ"*. Chữ "phải" nhắc bạn
> rằng vế sau là điều kiện bắt buộc, không phải điều kiện thêm cho vui.
::::

::::code{#dan-to-giay-len-tuong}
Đoạn dưới thiếu đúng một từ nối. Khách là học sinh và đến lúc 7 giờ, nên chương
trình phải in ra `Giảm 5000 đồng`.

Hãy điền vào chỗ trống để `if` chỉ gật khi **cả hai** vế cùng đúng.

```python title=starter
la_hoc_sinh = True
gio_den = 7

if la_hoc_sinh ___ gio_den < 8:
    print("Giảm 5000 đồng")
else:
    print("Giá thường")
```

```python title=solution
la_hoc_sinh = True
gio_den = 7

if la_hoc_sinh and gio_den < 8:
    print("Giảm 5000 đồng")
else:
    print("Giá thường")
```

```python title=test
# Chấm bằng OUTPUT: học sinh đến lúc 7 giờ thì phải nhận được lời giảm giá.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm **giữa** hai câu hỏi có–không, trong cùng một dòng `if`. Đó là chỗ của một từ nối, không phải chỗ của một điều kiện mới.
- kind: strategy
  body: Tờ giấy dán tường đòi cả hai điều cùng đúng thì mới giảm. Trong bảng bốn dòng ở trên, từ bạn cần là từ chỉ cho ra `True` ở đúng một dòng — dòng mà hai vế đều `True`.
- kind: one-line
  body: Thay `___` bằng `and`, giữ nguyên hai vế ở hai bên và dấu hai chấm cuối dòng.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Giảm 5000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=point-editor}
Một dòng `if`, hai điều kiện. Byte kiểm cả hai, không bỏ sót vế nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trưa nay quán gỡ tờ giấy cũ xuống và dán tờ mới:

> Học sinh **hoặc** người trên 65 tuổi: giảm 5000 đồng.

Chỉ đổi một chữ, mà luật đổi hẳn. Bác 70 tuổi không phải học sinh — vẫn được
giảm. Bạn học sinh 16 tuổi còn xa mới tới 65 — cũng được giảm. Lần này chỉ cần
**một** vế đúng là đủ.

Thử đưa tờ giấy mới này cho `and` làm xem: viết
`if la_hoc_sinh and tuoi_khach > 65:`, rồi để bác 70 tuổi bước vào quán. Theo
bảng bốn dòng hôm nay, `False and True` cho ra gì? Và bác sẽ nghe máy nói câu
nào?

Đừng trả lời vội. Bài sau là một từ nữa — ngắn hơn `and` một chữ, và dễ tính
hơn hẳn.
::::

::::checkpoint{mastery=0.8}
::::
