---
id: onboarding.may-tinh-noi-gi.in-gia-tri-cua-ten
title: In ra thứ mà cái tên đang giữ
summary: Đặt một cái tên vào print, không nháy. Máy đi tìm, và nói ra thứ nó tìm được.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [core.print-variable]
requires: [core.variable, core.output]
concepts: [core.bien, core.chuoi]
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
Hai bài vừa rồi bạn tin lời mình. Bài này bạn tự nhìn thấy.
::::

::::explain{#lan-nay-tim-thay}
Bài 8 cho bạn một luật để đọc mọi thứ nằm trong ngoặc của `print`:

- Có dấu nháy → đó là **chữ**. Máy đọc nguyên văn, không hỏi han gì thêm.
- Không có dấu nháy → đó là một **cái tên**. Máy phải đi tìm xem cái tên đó đang giữ gì.

Hồi bài 8, mọi lần đi tìm đều về tay không: bạn chưa từng đặt cái tên nào, nên
máy tìm không ra và dừng lại báo `NameError`.

Hai bài vừa rồi đã đổi chuyện đó. Bạn đã biết dán một tấm thẻ tên lên một giá
trị, và biết chuyển tấm thẻ ấy sang chỗ khác. Nghĩa là bây giờ trong máy **có**
những cái tên để tìm.

Vậy khi máy đi tìm và tìm **thấy** thì chuyện gì xảy ra?
::::

::::example{#print-mot-cai-ten}
Đây là đoạn bạn để lại cuối bài trước. Byte chạy nó:

```python title=readonly
mon_an = "Phở bò tái nạm"
print(mon_an)
```

Máy in ra:

```text
Phở bò tái nạm
```

Không có chữ `mon_an` nào hiện ra cả.

Đi chậm qua ba việc máy làm ở dòng thứ hai — chậm, vì cả bài này nằm gọn trong ba
việc đó:

1. Máy thấy trong ngoặc là `mon_an`, không có dấu nháy. Theo luật: đây là một cái tên, không phải chữ.
2. Máy đi tìm tấm thẻ mang tên `mon_an`, lần theo dây, thấy đầu kia là câu chữ `"Phở bò tái nạm"`.
3. Máy đưa **câu chữ đó** cho `print`. Cái tên đã làm xong việc của nó và biến khỏi câu chuyện.

Chỗ thứ ba đáng nhớ hơn cả: `print` không bao giờ biết cái tên. Lúc nó nhận được
hàng thì trong tay nó chỉ còn giá trị. Cái tên chỉ là đường đi tới giá trị, không
phải thứ được chở đi.
::::

::::predict{#co-nhay-thi-sao commitOnce}
Byte sắp chạy đoạn dưới. Nó giống hệt đoạn trên, chỉ thêm hai dấu nháy quanh
`mon_an`. **Trước khi bấm chạy**, bạn đoán nó in ra gì?

```python
mon_an = "Phở bò tái nạm"
print("mon_an")
```

:::opt{correct}
mon_an
:::

:::opt
Phở bò tái nạm
::why
Gần đúng ở chỗ bạn vừa nắm được luật mới và áp dụng ngay: đặt `mon_an` vào
`print` thì ra thứ cái tên đang giữ. Ở đoạn trên, đúng là như vậy.

Chỗ lệch là hai dấu nháy. Chúng đổi hẳn loại câu hỏi máy tự đặt ra. Có nháy thì
máy không đi tìm gì cả — nó đọc nguyên văn phần nằm giữa hai dấu, y như `"2 + 3"`
ở bài đầu tiên chỉ là ba ký tự chứ không phải phép cộng.

Dòng `mon_an = "Phở bò tái nạm"` vẫn chạy, tấm thẻ vẫn được buộc. Chỉ là dòng
dưới không thèm hỏi tới nó.
::
:::

:::opt
"mon_an" — hiện ra kèm cả hai dấu nháy
::why
Gần đúng, và gần nhất trong các đáp án: bạn nhận ra máy sẽ đọc nguyên văn phần
trong nháy. Phần suy luận đó chính xác.

Chỗ lệch chỉ ở chỗ hai dấu nháy **không thuộc về câu chữ**. Chúng là dấu hiệu bạn
để lại cho máy, nghĩa là "phần này là chữ, đọc nguyên văn". Máy đọc xong dấu hiệu
thì bỏ nó đi, giống như bóc giấy gói trước khi ăn bánh. Thứ hiện lên màn hình là
ruột bánh, không có giấy gói.
::
:::

:::opt
Máy báo `NameError`, vì cái tên bị kẹt trong dấu nháy nên tìm không ra
::why
Gần đúng ở chỗ bạn nhớ rất kỹ bài 8 — `NameError` đúng là chuyện xảy ra khi máy
đi tìm một cái tên mà không thấy. Nhớ được lỗi nào sinh ra từ đâu là một thói
quen tốt.

Chỗ lệch: máy chưa hề đi tìm. Hai dấu nháy làm nó dừng ngay ở bước một — "à, chữ"
— và không có cuộc tìm kiếm nào diễn ra. Không tìm thì không có gì để mà thất
bại.
::
:::
::::

::::explain{#hai-dong-dat-canh-nhau}
Đặt hai dòng cạnh nhau, khác đúng hai ký tự, mà ra hai kết quả khác hẳn:

| Bạn viết | Máy làm gì | Hiện ra |
|---|---|---|
| `print("mon_an")` | đọc nguyên văn phần trong nháy | mon_an |
| `print(mon_an)` | đi tìm cái tên, lấy thứ nó đang giữ | Phở bò tái nạm |

Bảng này là cả Module 1 thu lại thành ba dòng. Từ bài 8 tới giờ, mọi bài đều xoay
quanh đúng một câu hỏi: *máy phân biệt CHỮ với TÊN bằng cách nào*. Câu trả lời
vẫn là hai dấu nháy, không có gì khác.

Có một chỗ hay nhầm, và nhầm thì hay ngồi nhìn màn hình rất lâu: đặt tên biến rồi
lại quen tay bọc nháy vào lúc in. Chương trình chạy ngon lành, không báo lỗi
dòng nào — chỉ có màn hình hiện ra `gia_pho` thay vì `45000`. Gặp cảnh đó, hãy
nhìn ngay vào dấu nháy trước tiên.
::::

::::code{#in-bang-gia-quan}
Chị Hạnh muốn máy nói ra hai thứ: tên quán, rồi giá một tô phở.

Dòng `print` thứ nhất đã viết sẵn cho bạn xem. Hãy điền nốt dòng thứ hai để màn
hình hiện ra **con số giá**, chứ không phải cái tên.

```python title=starter
ten_quan = "Phở Thìn"
gia_pho = 45000
print(ten_quan)
print(___)
```

```python title=solution
ten_quan = "Phở Thìn"
gia_pho = 45000
print(ten_quan)
print(gia_pho)
```

```python title=test
# Bài này chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa hai dấu ngoặc của `print` ở dòng cuối. Thứ cần hiện ra ở đó là **giá một tô phở** — mà giá ấy đang được một cái tên giữ hộ ở dòng thứ hai.
- kind: strategy
  body: Dòng `print` ngay bên trên đã làm mẫu đúng việc bạn cần làm: đặt một cái tên vào trong ngoặc, không kèm dấu nháy. Việc còn lại là chọn đúng cái tên đang giữ con số.
- kind: one-line
  body: "Viết `gia_pho` vào chỗ trống — không dấu nháy."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Phở Thìn\n45000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Máy vừa nói ra thứ nó đang giữ, chứ không nói lại cái tên bạn đặt.
::::

::::reflect{#nghi-lai}
Nhìn lại hai dòng đầu của đoạn bạn vừa chạy:

```python
ten_quan = "Phở Thìn"
gia_pho = 45000
```

Về hình thức, hai dòng này giống nhau như đúc: một cái tên, một dấu `=`, một giá
trị. Cả hai tấm thẻ đều buộc xong xuôi, và `print` lấy được cả hai.

Nhưng hai giá trị ở đầu kia sợi dây thì khác nhau. Một bên là câu chữ nằm giữa
hai dấu nháy. Một bên là con số viết trần, không nháy — và theo bài 9, máy tính
toán được với nó.

Mắt bạn thấy ngay chỗ khác nhau đó.

Còn máy thì sao? Nó có phân biệt được như bạn không — hay với nó, cái gì cũng chỉ
là "thứ nằm ở đầu kia tấm thẻ", giống nhau cả?

Đừng trả lời vội. Bài sau bạn sẽ hỏi thẳng máy câu này, và máy trả lời được.
::::

::::checkpoint{mastery=0.8}
::::
