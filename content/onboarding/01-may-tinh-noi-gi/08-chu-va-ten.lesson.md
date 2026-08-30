---
id: onboarding.may-tinh-noi-gi.chu-va-ten
title: Chữ và tên là hai thứ khác nhau
summary: Trong nháy là chữ để đọc nguyên văn. Ngoài nháy là một cái tên máy phải đi tìm.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.name-lookup, err.name-error]
requires: [core.string-literal, core.terminal]
concepts: [core.ten-va-chu, core.chuoi]
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
Trong nháy thì mình đọc. Ngoài nháy thì mình đi tìm. Hai việc khác hẳn nhau.
::::

::::explain{#to-giay-cua-chi-hanh-lan-hai}
Quay lại tờ giấy dán tường trong bếp chị Hạnh. Hôm nay trên đó có hai dòng:

```text
Gọi cho bà Tư
Viết lên bảng: "Hôm nay nghỉ bán"
```

Cậu em phụ bếp đọc hai dòng này và làm hai việc rất khác nhau.

Ở dòng đầu, cậu **không** đứng giữa bếp đọc to ba tiếng "bà Tư". Cậu hiểu đó là
một người, nên cậu đi tìm: mở sổ điện thoại ra, dò tới chữ *Tư*, bấm số.

Ở dòng sau, cậu **không** đi tìm ai tên "Hôm nay nghỉ bán" cả. Cậu cầm phấn và
chép đúng năm chữ ấy lên bảng, nguyên văn, không thêm không bớt.

Cả hai dòng đều là chữ viết trên cùng một tờ giấy. Vậy cái gì bảo cậu em biết
dòng nào phải **đi tìm**, dòng nào phải **chép nguyên văn**?

Hai dấu nháy.

Máy tính dùng đúng dấu hiệu đó. Chỉ khác là nó theo dấu hiệu ấy một cách tuyệt
đối máy móc, không đoán thêm bao giờ.
::::

::::explain{#hai-cuon-so}
Bài 6 cho bạn cuốn sổ thứ nhất: **bảng ký tự**. Chữ nằm giữa hai dấu nháy được
máy tra bảng đó, đổi ra số, rồi giữ nguyên văn. Máy không cố hiểu nghĩa của nó —
`"Phở"` với máy chỉ là ba con số xếp hàng.

Hôm nay bạn gặp cuốn sổ thứ hai: **sổ tên**.

Khi máy gặp một từ **không có nháy**, nó không tra bảng ký tự. Nó mở sổ tên ra
và hỏi: *có ai tên như thế này không?*

- Có → máy lấy ra thứ mà cái tên ấy đang trỏ tới, rồi làm tiếp.
- Không có → máy dừng lại và nói thẳng ra rằng nó không tìm thấy.

Lúc này sổ tên của bạn đang trống trơn — bạn chưa đặt tên cho thứ gì cả. Nên mọi
từ không nháy đều rơi vào trường hợp thứ hai.

Đó chính là câu trả lời cho câu hỏi treo từ bài 1.
::::

::::example{#thu-ngay-o-terminal}
Mở terminal, gõ hai dòng, mỗi dòng bấm **Enter** một lần:

```text title=readonly
>>> print("Phở")
Phở
>>> print(Phở)
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
NameError: name 'Phở' is not defined
>>>
```

Hai câu lệnh bạn gõ chỉ khác nhau đúng **hai dấu nháy**. Kết quả thì khác nhau
hoàn toàn.

Nhìn dòng cuối cùng của phần máy trả lời:

```text
NameError: name 'Phở' is not defined
```

Dịch từng mảnh:

| Mảnh | Nghĩa |
|---|---|
| `NameError` | Lỗi Tên — loại lỗi này liên quan tới một cái tên |
| `name 'Phở'` | cái tên gây chuyện là `Phở` |
| `is not defined` | chưa được định nghĩa, tức là **sổ tên không có dòng nào tên như vậy** |

Ghép lại: *"Bạn bảo tôi đi tìm một thứ tên Phở. Tôi mở sổ ra rồi, không có."*

Để ý giọng của câu ấy. Máy không nói bạn dốt, cũng không nói bạn sai. Nó **báo
cáo** một sự việc: nó đã tìm, và nó không thấy. Cả khoá học này, mọi thông báo
lỗi bạn gặp đều là loại câu như vậy — một bản báo cáo, không phải một lời chê.

Còn dòng ở giữa (`File "<stdin>", line 1...`) trên máy bạn có thể trông hơi
khác. Cả cụm này có tên riêng và có cách đọc riêng; bài 17 sẽ dành trọn cho nó.
Bây giờ bạn chỉ cần một thói quen: **đọc dòng cuối cùng trước**.

Và một chi tiết nhỏ mà dễ bỏ qua: sau khi báo lỗi, dấu nhắc `>>>` **hiện lại**.
Máy không giận, không đóng cửa sổ, không bắt bạn làm lại từ đầu. Nó chờ câu tiếp
theo của bạn, y như bài trước.
::::

::::predict{#doan-hai-dong commitOnce}
Byte sắp chạy đoạn dưới. Hai dòng, và chúng chỉ khác nhau hai dấu nháy.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
print("quẩy")
print(quẩy)
```

:::opt{correct}
Dòng chữ `quẩy` hiện ra một lần, rồi máy dừng lại ở dòng hai với `NameError`.
:::

:::opt
Chữ `quẩy` hiện ra hai lần.
::why
Gần đúng ở chỗ mắt bạn đọc hai dòng ấy gần như giống hệt nhau — và đúng là
chúng chỉ lệch nhau hai ký tự bé xíu.

Chỗ lệch: hai ký tự bé xíu đó lại đổi hẳn công việc của máy. Dòng đầu, máy tra
bảng ký tự rồi in nguyên văn. Dòng sau, máy mở sổ tên và đi tìm một thứ tên
`quẩy`. Hai cuốn sổ khác nhau, hai việc khác nhau.

Đây là lý do dấu nháy đáng để bạn nhìn kỹ, dù nó nhỏ tới đâu.
::
:::

:::opt
Máy báo lỗi ngay từ đầu, không in ra chữ nào cả.
::why
Gần đúng, và bạn suy luận trúng phần khó: trong đoạn này có một chỗ máy làm
không nổi, và nó sẽ dừng ở đó.

Chỗ lệch là về **thứ tự**. Máy đọc từ trên xuống, làm xong dòng nào mới sang
dòng dưới. Dòng đầu nó làm được trọn vẹn nên nó cứ in ra. Chỉ tới dòng hai nó
mới vấp.

Nói cách khác: một chương trình gặp lỗi giữa chừng thì **phần trước chỗ vấp đã
thật sự chạy rồi**. Ghi nhớ điều này, nó sẽ giúp bạn tìm lỗi rất nhiều về sau.
::
:::

:::opt
Máy in ra chữ `quẩy` rồi in tiếp chữ `print`.
::why
Gần đúng ở chỗ bạn để ý rằng `print` cũng là một từ không có nháy — quan sát này
rất tinh, và nó đúng.

Chỗ lệch: `print` là một từ không nháy, nên máy cũng đi tra sổ tên với nó. Khác
biệt là lần này máy **tìm thấy**: `print` đã có sẵn trong sổ từ trước, do Python
ghi vào giùm bạn, và nó trỏ tới việc "nói ra". Tìm thấy thì máy làm việc đó,
chứ không in tên ra.

Vậy là bạn vừa gặp cả hai nhánh trong cùng một dòng: `print` tìm thấy, `quẩy`
tìm không ra.
::
:::
::::

::::code{#sua-lai-bang-hieu}
Quán chè đầu ngõ nhờ Byte in hộ tên món lên bảng hiệu. Byte gõ dòng dưới và máy
báo `NameError: name 'Chè' is not defined`.

Hãy sửa lại để máy nói ra đúng chữ **Chè**.

```python title=starter
print(Chè)
```

```python title=solution
print("Chè")
```

```python title=test
# Bài này chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Nhìn kỹ chữ `Chè` bên trong hai dấu ngoặc. Nó đang nằm trần, không có gì bọc quanh — nên máy coi nó là một cái tên và đi tra sổ.
- kind: strategy
  body: Bạn muốn máy đọc nguyên văn chữ đó chứ không đi tìm ai cả. Nhớ lại bài 1 và bảng ký tự ở bài 6: cái gì báo cho máy biết "phần này là chữ"?
- kind: one-line
  body: "Viết `print(\"Chè\")` — thêm một dấu nháy kép trước chữ C và một dấu nữa sau chữ è."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Chè
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai dấu nháy bé xíu. Chúng quyết định mình đọc nguyên văn hay mình đi tìm.
::::

::::explain{#so-ten-de-lam-gi}
Có thể bạn đang nghĩ: nếu sổ tên lúc nào cũng trống và từ không nháy lúc nào
cũng gây lỗi, thì viết từ không nháy làm gì cho khổ?

Sổ tên trống vì bạn chưa ghi gì vào đó. Nó không phải lúc nào cũng trống.

Tới bài 11 bạn sẽ ghi dòng đầu tiên vào sổ — đặt một cái tên cho một giá trị. Từ
lúc ấy, `print(mon_an)` không còn là lỗi nữa, mà là cách bạn bảo máy: *lấy thứ
đang đứng tên `mon_an` ra và nói nó ra*.

Nói cách khác, `NameError` hôm nay không phải một cái bẫy. Nó là một cánh cửa
chưa mở: máy đã có sẵn chỗ để bạn đặt tên, chỉ là bạn chưa đặt tên nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài hôm nay chia mọi thứ bạn viết ra làm hai loại: có nháy thì máy **đọc**,
không nháy thì máy **đi tìm trong sổ tên**.

Bây giờ nhìn dòng này:

```text
print(2 + 3)
```

`2` và `3` không có dấu nháy. Theo luật hôm nay, máy phải mở sổ tên ra tìm một
thứ tên là `2`. Nhưng chẳng ai đặt tên cho cái gì là `2` bao giờ.

Vậy máy sẽ báo `NameError` chứ? Hay con số là một chuyện khác hẳn, nằm ngoài cả
hai loại?

Đừng trả lời vội. Bài sau gõ đúng dòng đó và cho bạn xem.
::::

::::checkpoint{mastery=0.8}
::::
