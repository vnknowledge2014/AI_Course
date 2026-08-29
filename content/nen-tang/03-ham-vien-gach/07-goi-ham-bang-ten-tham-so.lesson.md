---
id: nen-tang.ham-vien-gach.goi-ham-bang-ten-tham-so
title: Gọi bằng tên, không bằng chỗ
summary: Mang tên tham số xuống ngay chỗ gọi — `tinh_tien(so_to=2, gia=45000)`; đã gọi theo tên thì thứ tự hết quan trọng.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.keyword-argument]
practices: [core.function-call, core.function-argument, core.fstring]
requires: [core.function-def, core.function-call, core.function-parameter, core.function-argument, core.function-arg-count, core.docstring, core.fstring, err.type-error, err.syntax-error]
concepts: [core.ham, core.doi-so]
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
Bốn con số xếp hàng. Mình đọc được thứ tự, nhưng không đọc ra ý nghĩa.
::::

::::explain{#bon-con-so-khong-ai-doc-ra}
Bài trước để lại một dòng trông rất vô hại:

```python
tien = tinh_tien(2, 45000, 1, 0)
```

Máy chạy nó không vấp một nhịp nào. Con số hiện ra đúng. Vấn đề nằm ở chỗ
khác: một tuần sau mở lại file này, bạn phải cuộn ngược lên tìm dòng `def` mới
biết con số `1` là gì và con số `0` là gì.

Dòng `def` ấy đây:

```python
def tinh_tien(so_to, gia, so_tra_da, giam_gia):
    """Tiền một bàn: phở, trà đá 5000 đồng một cốc, rồi trừ tiền giảm."""
    return so_to * gia + so_tra_da * 5000 - giam_gia
```

Bốn cái tên `so_to`, `gia`, `so_tra_da`, `giam_gia` nói rất rõ mỗi chỗ trống
dùng để làm gì. Nhưng chúng nằm ở dòng `def` — có khi cách chỗ gọi vài trăm
dòng, có khi nằm hẳn trong một file khác do người khác viết. Ở chỗ gọi, thứ còn
lại chỉ là bốn con số trần.

Python cho phép mang bốn cái tên ấy xuống ngay chỗ gọi:

```python
tien = tinh_tien(so_to=2, gia=45000, so_tra_da=1, giam_gia=0)
```

Cùng một hàm, cùng một kết quả, cùng bốn giá trị. Khác nhau ở chỗ dòng thứ hai
tự nó nói ra hết: hai tô, giá bốn mươi lăm nghìn một tô, một cốc trà đá, không
giảm đồng nào. Không phải cuộn đi đâu tìm.

Giới lập trình gọi lối viết này là **đối số theo tên**: bên trong ngoặc, mỗi
giá trị đi kèm tên của cái chỗ trống mà nó sẽ rơi vào, nối bằng dấu `=`.

> Dấu `=` trong ngoặc không phải phép gán biến. `so_to=2` không tạo ra một cái
> tên `so_to` ở ngoài hàm — nó nói với máy: "giá trị `2` này dành cho tham số
> tên `so_to`". Hết lượt gọi, cái tên ấy không còn ở đâu ngoài hàm cả.
::::

::::example{#doi-cho-cung-khong-sao}
Khi mỗi giá trị đã tự khai tên chỗ đứng của mình, máy không cần đếm chỗ nữa.
Nên thứ tự trong ngoặc hết quan trọng — đảo thế nào cũng ra cùng một kết quả:

```python title=readonly
def tinh_tien(so_to, gia, so_tra_da, giam_gia):
    """Tiền một bàn: phở, trà đá 5000 đồng một cốc, rồi trừ tiền giảm."""
    return so_to * gia + so_tra_da * 5000 - giam_gia

print(tinh_tien(so_to=2, gia=45000, so_tra_da=1, giam_gia=0))
print(tinh_tien(giam_gia=0, so_tra_da=1, gia=45000, so_to=2))
```

Máy in ra:

```text
95000
95000
```

Hai lời gọi viết khác hẳn nhau mà cho cùng một số, vì cả hai nói cùng một điều.
Hai tô phở bốn mươi lăm nghìn là 90000, thêm một cốc trà đá 5000 là 95000, giảm
0 đồng nên giữ nguyên 95000.

So lại với bài trước cho thấy rõ hai lối gọi khác nhau ở chỗ nào:

- **Theo vị trí** — máy đếm chỗ. Đối số thứ nhất rơi vào tham số thứ nhất, thứ
  hai vào thứ hai. Đảo chỗ hai con số là đổi luôn ý nghĩa của chúng, mà máy
  không kêu một tiếng.
- **Theo tên** — máy đọc tên. Chỗ đứng không còn nói gì nữa, nên đảo chỗ chẳng
  đổi gì cả.

Có một chỗ máy vẫn kêu: gõ sai tên. `tinh_tien(so_toooo=2, ...)` không có tham
số nào tên như vậy, và máy dừng ngay bằng `TypeError` thay vì lặng lẽ tính ra
một con số sai. Đó là món lời của lối gọi theo tên: cái tên gõ nhầm thì máy bắt
được, còn cái chỗ đứng nhầm thì không.
::::

::::predict{#tron-hai-loi commitOnce}
Hai lối gọi ấy trộn chung trong một cặp ngoặc được không? Byte thử viết `gia`
theo tên, còn số tô thì để trần.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những gì?

```python
def tinh_tien(so_to, gia):
    """Tiền phở của một bàn."""
    return so_to * gia

print("Bắt đầu tính tiền")
print(tinh_tien(gia=45000, 2))
```

:::opt{correct}
Màn hình trống trơn — máy báo `SyntaxError` trước khi chạy dòng nào
:::

:::opt
In `Bắt đầu tính tiền`, rồi máy báo lỗi ở dòng cuối
::why
Gần đúng ở chỗ quan trọng nhất: bạn nhận ra dòng cuối có gì đó không ổn, và
đúng là máy sẽ từ chối nó. Phần đọc lời gọi của bạn chính xác.

Chỗ lệch nằm ở **lúc nào** máy phát hiện ra. `SyntaxError` là loại lỗi máy tìm
thấy lúc **đọc** cả file, trước khi chạy dòng đầu tiên — giống hệt lúc bạn quên
dấu hai chấm cuối dòng `if` ở Realm 0: cả chương trình không chạy, chứ không
phải chạy được một nửa rồi mới dừng. Nên dòng `print("Bắt đầu tính tiền")` nằm
trên cũng không kịp in ra.
::
:::

:::opt
In `Bắt đầu tính tiền`, rồi in `90000`
::why
Gần đúng ở chỗ bạn suy luận rất hợp lý: `gia` đã nhận 45000 rồi, vậy trong hai
tham số còn đúng một chỗ trống, nên con số `2` chỉ có thể là `so_to`. Người đọc
nào cũng hiểu như vậy.

Chỗ lệch là máy không suy luận theo kiểu "chỉ còn một chỗ nên chắc là chỗ đó".
Nó đọc trong ngoặc từ trái sang phải, và luật của nó rất cứng: **đối số theo vị
trí phải đứng trước, đối số theo tên đứng sau**. Một khi đã gặp `gia=45000`,
mọi thứ đứng sau đều phải mang tên. Viết ngược lại thì máy dừng ngay ở khâu đọc
chứ không cố đoán ý bạn.
::
:::

:::opt
Máy báo `TypeError` vì `tinh_tien` nhận thiếu `so_to`
::why
Gần đúng ở chỗ bạn nhớ đúng một loại lỗi có thật: đưa thiếu đối số thì máy dừng
bằng `TypeError`, đúng như bài về đưa thiếu đưa thừa đã dựng.

Chỗ lệch là ở đây `so_to` không hề thiếu — con số `2` vẫn nằm trong ngoặc. Cái
máy phản đối là **chỗ đứng** của nó, và phản đối ấy xảy ra sớm hơn hẳn: lúc đọc
chữ, chứ không phải lúc gọi hàm. `TypeError` là lỗi khi máy đã chạy tới lời gọi
và thấy số đối số không khớp; `SyntaxError` là lỗi khi máy còn chưa hiểu nổi
dòng ấy viết cái gì.
::
:::
::::

::::explain{#luat-tron-hai-loi}
Vậy luật khi trộn hai lối gọi trong cùng một cặp ngoặc chỉ có một câu: **đối số
theo vị trí đứng trước, đối số theo tên đứng sau**.

Đúng luật thì trộn thoải mái:

```python
tinh_tien(2, 45000, so_tra_da=1, giam_gia=0)
```

Hai con số đầu đi theo chỗ đứng, hai giá trị sau đi theo tên. Máy đếm chỗ cho
tới khi gặp cái tên đầu tiên, rồi từ đó trở đi nó chuyển hẳn sang đọc tên.

Sai luật thì máy dừng ngay:

```python
tinh_tien(so_to=2, 45000, 1, 0)
```

Vì sao luật lại cứng như vậy? Vì nếu cho phép viết ngược, máy phải tự nghĩ ra
con số trần đứng sau `so_to=2` thuộc về chỗ nào — chỗ thứ hai, hay chỗ đầu tiên
còn trống? Hai cách hiểu, và cả hai đều nghe lọt tai. Một dòng code có hai cách
hiểu thì hai người đọc ra hai chương trình khác nhau, nên Python cắt phăng chỗ
mơ hồ đó bằng cách từ chối luôn.

Trong thực tế người ta hay dùng đúng cách trộn ấy: một hai đối số đầu quá rõ
nghĩa nên để trần, còn những đối số dễ lẫn thì gọi theo tên.

> Chỗ dễ vấp: gọi theo tên thì tên phải **trùng khít** với tên trong dòng `def`,
> kể cả dấu gạch dưới. Đổi tên tham số trong `def` mà quên sửa những chỗ gọi
> theo tên là một lỗi có thật, và nó hiện ra dưới dạng `TypeError` nói rằng
> hàm không có tham số nào tên như vậy.
::::

::::code{#goi-ban-ba-bang-ten}
Bàn 3 vừa đứng dậy. Bồi bàn đọc số cho Byte theo đúng thứ tự họ nhớ ra, không
theo thứ tự tham số: **3 tô**, **2 cốc trà đá**, **giá một tô 40000**, và bàn
này được **giảm 5000**.

Lời gọi dưới đây đã viết theo tên cho ba giá trị, còn một chỗ trống. Hãy điền
nốt giá trị còn lại — cũng theo tên, vì nó đang đứng ở chỗ mà lối vị trí không
với tới được.

```python title=starter
def tinh_tien(so_to, gia, so_tra_da, giam_gia):
    """Tiền một bàn: phở, trà đá 5000 đồng một cốc, rồi trừ tiền giảm."""
    return so_to * gia + so_tra_da * 5000 - giam_gia

tien_ban_3 = tinh_tien(so_to=3, so_tra_da=2, ___, giam_gia=5000)

print(f"Bàn 3 phải trả {tien_ban_3} đồng")
```

```python title=solution
def tinh_tien(so_to, gia, so_tra_da, giam_gia):
    """Tiền một bàn: phở, trà đá 5000 đồng một cốc, rồi trừ tiền giảm."""
    return so_to * gia + so_tra_da * 5000 - giam_gia

tien_ban_3 = tinh_tien(so_to=3, so_tra_da=2, gia=40000, giam_gia=5000)

print(f"Bàn 3 phải trả {tien_ban_3} đồng")
```

```python title=test
# Ba tô 40000 là 120000, hai cốc trà đá là 10000, trừ 5000 tiền giảm.
# Con số này chỉ ra đúng khi giá một tô rơi vào tham số `gia`: đưa nhầm nó
# sang chỗ khác thì phép nhân đổi luôn ý nghĩa và tổng lệch hẳn.
assert tien_ban_3 == 125000, "bàn 3 phải trả 125000 đồng — ba tô 40000 cộng hai cốc trà đá rồi trừ 5000 tiền giảm; con số khác nghĩa là 40000 đang được đưa vào một tham số khác"
# Gọi lại chính hàm ấy với đúng bốn giá trị của bàn 3, lần này theo vị trí:
# hai lời gọi cùng nói một điều nên phải cùng ra một số.
assert tien_ban_3 == tinh_tien(3, 40000, 2, 5000), "cùng bốn giá trị ấy, gọi theo tên và gọi theo vị trí phải cho cùng một kết quả — lệch nhau nghĩa là có giá trị đang rơi nhầm chỗ"
```

:::hints
- kind: attention
  body: Chỗ trống nằm lọt giữa hai đối số đã có tên. Đối chiếu lời gọi với dòng `def` xem bốn tham số ở đó, cái nào chưa được nhắc tới trong ngoặc.
- kind: strategy
  body: Cái tên còn thiếu là cái giữ giá một tô phở, và giá một tô của bàn 3 là 40000. Viết theo đúng lối mà ba đối số kia đang dùng — tên, dấu bằng, rồi con số. Vì cả bốn đều mang tên nên chuyện nó đứng thứ ba trong ngoặc không ảnh hưởng gì.
- kind: one-line
  body: 'Viết `gia=40000` vào chỗ trống, không thêm dấu cách quanh dấu bằng.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Bàn 3 phải trả 125000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giờ đọc dòng gọi là biết bàn 3 ăn gì. Không phải cuộn lên tìm chữ ký nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Từ tháng này quán của Byte xuất hoá đơn có thuế, nên hàm tính tiền có thêm một
chỗ trống nữa:

```python
def tinh_hoa_don(so_to, gia, thue):
    """Tiền một bàn, đã cộng thuế."""
    tien_hang = so_to * gia
    return tien_hang + int(tien_hang * thue)
```

Chiều nay Byte gọi nó mười lần cho mười bàn. Chín bàn tính thuế mức thường
`0.1`; đúng một bàn xin hoá đơn đỏ nên tính `0.08`.

Nhờ bài hôm nay, mười dòng gọi ấy đọc ra được hết — không dòng nào còn con số
trần nào khó hiểu. Nhưng nhìn cả mười dòng xếp cạnh nhau thì thấy một chuyện
khác: chín trong mười dòng có đúng một mẩu giống hệt nhau, chép đi chép lại
không sai một ký tự.

Mười lần gọi thì chín lần `thue=0.1`. Vẫn phải gõ lại đủ mười lần sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
