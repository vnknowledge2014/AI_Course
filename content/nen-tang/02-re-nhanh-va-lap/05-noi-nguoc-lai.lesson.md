---
id: nen-tang.re-nhanh-va-lap.noi-nguoc-lai
title: Nói ngược lại một câu
summary: Một từ đặt trước câu trả lời và lật nó — đúng thành sai, sai thành đúng.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [logic.not]
requires: [core.boolean, ctrl.if, logic.and]
concepts: [logic.phep-logic, core.dung-sai]
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
Mình có sẵn một từ để lật ngược câu trả lời. Đúng thành sai, sai thành đúng.
::::

::::explain{#chu-khong-cua-python}
Bài trước để lại một dòng đọc lên nghe vòng vèo:

```python
if co_trong_so == False:
    print("Ngày này chưa ghi sổ")
```

Đọc thành tiếng: *"nếu có-trong-sổ bằng sai"*. Ba khái niệm chồng lên nhau —
*có*, *bằng*, *sai* — để nói đúng một chuyện mà tiếng Việt gói trong hai chữ:
**không có**.

Để ý cách tiếng Việt làm chuyện đó. Nó không sửa câu gốc. Nó giữ nguyên câu
*"có trong sổ"* rồi dán thêm chữ **không** vào **đằng trước**:

> có trong sổ → **không** có trong sổ

Python có đúng một từ làm việc dán ấy: `not` — tiếng Anh nghĩa là "không".

`not` đứng **trước** một câu trả lời đúng/sai, và nó lật câu ấy. Toàn bộ luật
của `not` chỉ có hai dòng:

| câu gốc | `not` câu đó |
|---|---|
| `True` | `False` |
| `False` | `True` |

Nhớ lại `and` ở Realm 0: nó ngồi **giữa** hai câu trả lời và cần đủ hai bên mới
làm việc được. `not` khác hẳn — nó chỉ cần **một** thứ, và thứ ấy nằm bên phải
nó.
::::

::::example{#lat-thu-mot-cau}
Bấm chạy hai dòng ngắn nhất có thể, để thấy `not` làm gì:

```python title=readonly
print(not True)
print(not False)
```

Máy in ra:

```text
False
True
```

Đúng hai dòng trong bảng trên, không hơn.

Giờ đem `not` vào đúng chỗ nó sinh ra để làm. Ngày hôm nay Byte chưa ghi vào
sổ, nên cái tên `co_trong_so` đang giữ `False`:

```python title=readonly
co_trong_so = False

if not co_trong_so:
    print("Ngày này chưa ghi sổ")
```

Máy in ra:

```text
Ngày này chưa ghi sổ
```

Đi lại đường máy đi. `co_trong_so` giữ `False`. `not` lật `False` thành `True`.
Chỗ giữa `if` và dấu hai chấm nhận được `True`, nên dòng in bên dưới được chạy.

Và đọc dòng `if` ấy thành tiếng: *"nếu không có trong sổ"*. Đúng câu tiếng Việt
bạn muốn nói, không thừa một chữ nào.
::::

::::predict{#not-co-sua-cai-ten-khong commitOnce}
Hôm nay Byte **đã** ghi vào sổ, nên `co_trong_so` giữ `True`. Byte in ra hai
dòng: dòng đầu có `not`, dòng sau thì không.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
co_trong_so = True

print(not co_trong_so)
print(co_trong_so)
```

:::opt{correct}
`False` rồi `True`
:::

:::opt
`False` rồi `False`
::why
Gần đúng ở chỗ bạn nhận ra dòng thứ ba đã lật `True` thành `False` — phần ấy
bạn tính chính xác, và `False` đúng là thứ hiện ra đầu tiên.

Chỗ lệch nằm ở chuyện `not` **để lại** cái gì phía sau nó. `not co_trong_so`
không đụng vào kho chứa của cái tên; nó chỉ lấy giá trị ra xem rồi đưa lại một
câu trả lời mới. Giống hệt `2 + 3`: phép cộng ấy cho ra `5` chứ không biến số
`2` thành số khác. Muốn cái tên đổi thứ nó giữ thì phải có dấu `=` —
`co_trong_so = not co_trong_so` — mà dòng đó thì không có trong đoạn này.
::
:::

:::opt
`True` rồi `True`
::why
Gần đúng ở chỗ mọi lần bạn gặp một điều kiện cho tới hôm nay, nó đều nằm giữa
`if` và dấu hai chấm. Nghĩ rằng những từ như `not` chỉ có nghĩa **bên trong**
một lối rẽ là suy nghĩ tự nhiên khi bạn mới thấy chúng ở đúng một chỗ.

Chỗ lệch: `not co_trong_so` là một **giá trị**, y như `45000` hay `"phở"` là
một giá trị. Nó dùng được ở bất cứ đâu mà một giá trị dùng được — đưa vào
`print` để xem, hoặc đặt sau `if` để rẽ nhánh. `if` không phải điều kiện sống
của nó.
::
:::

:::opt
Máy báo lỗi: `not` cần hai vế như `and`, mà đây chỉ có một
::why
Gần đúng ở chỗ bạn nhớ đúng hình dạng của `and`: nó phải có đủ hai bên, thiếu
một bên là máy dừng lại hỏi. Từ nối nào cũng cần hai đầu để nối — đó là luật
đúng cho `and` và `or`.

Chỗ lệch: `not` không phải từ **nối**. Nó không buộc hai câu lại với nhau, nó
lật một câu. Một câu thì chỉ cần một chỗ để đặt, và chỗ ấy là bên phải nó. Vì
vậy `not True` viết đủ, còn `True not` thì mới là dòng máy không đọc được.
::
:::
::::

::::explain{#not-dung-o-dau}
Ba điều dùng được ngay từ hôm nay:

- **`not` đứng trước, cách một khoảng trắng.** `not co_trong_so` — có dấu cách.
  Gõ dính thành `notco_trong_so` thì máy đọc ra một **cái tên** hoàn toàn khác,
  và vì chưa ai đặt tên đó nên nó dừng lại với `NameError`.
- **`not` lật được mọi câu trả lời đúng/sai, không riêng gì cái tên.** Kể cả
  câu trả lời bạn vừa dựng ra từ một phép so sánh rồi đặt tên cho nó:

  ```python title=readonly
  tien = 150000
  vuot_nguong = tien > 200000

  print(vuot_nguong)
  print(not vuot_nguong)
  ```

  Máy in ra `False` rồi `True`. Dòng `tien > 200000` cho ra `False`, cái tên
  `vuot_nguong` giữ lấy `False` ấy, và `not` lật nó lên.
- **`not x` thay được cho `x == False`.** Hai cách cho cùng kết quả, nhưng cách
  thứ nhất là cách một người nói. Từ hôm nay, hễ trong đầu bạn có chữ *"không"*
  thì trên màn hình là chữ `not`.

> Dễ nhầm: `not` **không** sửa thứ mà cái tên đang giữ. Sau dòng
> `print(not co_trong_so)`, cái tên `co_trong_so` vẫn giữ nguyên giá trị cũ.
> `not` sinh ra một câu trả lời mới rồi đưa cho bạn — bạn dùng ngay hay bỏ đi
> là chuyện của bạn.
::::

::::code{#nhac-nhung-ngay-chua-ghi}
Sổ chi tiêu của Byte có cột *đã ghi sổ chưa*, mỗi ngày một giá trị `True` hoặc
`False`. Byte muốn nhắc đúng những ngày **chưa** ghi.

Đây là bốn ngày đầu tuần:

- **Thứ hai** — đã ghi sổ.
- **Thứ ba** — chưa ghi.
- **Thứ tư** — chưa ghi.
- **Thứ năm** — đã ghi sổ.

Cùng một câu hỏi ấy đặt cho cả bốn ngày, nên **bốn chỗ trống điền giống hệt
nhau**. Dòng `co_trong_so = ...` phía trên mỗi khối gán lại cái tên cũ: từ đó
trở xuống, `co_trong_so` là ngày kế tiếp.

Viết đúng thì màn hình hiện ra **hai** trong bốn dòng chữ.

```python title=starter
co_trong_so = True
if ___:
    print("Thứ hai chưa ghi sổ")

co_trong_so = False
if ___:
    print("Thứ ba chưa ghi sổ")

co_trong_so = False
if ___:
    print("Thứ tư chưa ghi sổ")

co_trong_so = True
if ___:
    print("Thứ năm chưa ghi sổ")
```

```python title=solution
co_trong_so = True
if not co_trong_so:
    print("Thứ hai chưa ghi sổ")

co_trong_so = False
if not co_trong_so:
    print("Thứ ba chưa ghi sổ")

co_trong_so = False
if not co_trong_so:
    print("Thứ tư chưa ghi sổ")

co_trong_so = True
if not co_trong_so:
    print("Thứ năm chưa ghi sổ")
```

```python title=test
# Chấm bằng TRỌN VẸN output, trên CẢ BỐN ngày.
#
# Một ngày thôi thì không phân biệt được đúng với sai: điều kiện nào cũng chỉ
# cho ra `True` hoặc `False`, nên người gõ bừa có đúng hai lựa chọn và luôn
# trúng một cái. Bốn ngày gồm cả hai trạng thái của cột "đã ghi sổ chưa", nên
# mỗi cách điền sai để lại một dấu vết khác nhau trên màn hình:
#
# - Điền một thứ luôn đúng (`True`, `1`): cả bốn dòng cùng hiện.
# - Điền một thứ luôn sai (`0`): không dòng nào hiện.
# - Điền `co_trong_so`, tức quên mất chữ `not`: thứ hai và thứ năm lên tiếng —
#   đúng hai ngày ĐÃ ghi sổ, ngược hẳn ý bài. Đây là cách sai duy nhất vẫn cho
#   ra hai dòng, và nó vẫn trượt vì hai dòng ấy sai tên ngày.
# - Điền `co_trong_so == True`: ra y hệt cách trên, cũng trượt.
pass
```

:::hints
- kind: attention
  body: Ý bạn muốn nói là "chưa ghi sổ", còn cái tên trong đoạn code lại giữ câu trả lời cho "đã ghi sổ". Hai câu ấy ngược nhau — bạn cần một từ đặt vào trước để lật.
- kind: strategy
  body: Viết cái tên `co_trong_so` ra trước, đọc nó thành tiếng là "có trong sổ", rồi dán chữ "không" của Python vào đằng trước, nhớ chừa một khoảng trắng. Bốn ngày chỉ khác nhau ở giá trị `True`/`False` phía trên, còn câu hỏi thì không đổi — nên bốn chỗ trống điền y hệt nhau.
- kind: one-line
  body: "Viết `not co_trong_so` vào cả bốn chỗ trống, giữ nguyên dấu hai chấm ở cuối mỗi dòng `if`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai câu hỏi phải lật bằng chữ `not`, không phải bằng `== False` — cả hai chạy đúng, nhưng `not` là thứ duy nhất bài này dạy
  requireAst:
  # `co_trong_so == False` cho ra y hệt hai dòng ấy và khớp regex, nên không
  # có luật này thì người học xong bài `teaches: [logic.not]` mà chưa gõ chữ
  # `not` lần nào.
  - kind: uses-operator, target: not, min: 2
- tier: output
  match: regex
  expect: ^Thứ ba chưa ghi sổ\nThứ tư chưa ghi sổ\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đúng hai ngày lên tiếng. Một chữ `not` là đủ để hỏi ngược lại cả cột sổ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sổ chi tiêu còn một cột nữa: ngày đó có vượt ngưỡng 200 nghìn không. Gọi cột ấy
là `vuot_nguong`. Bây giờ Byte muốn lọc ra những ngày đáng để ý, và viết:

```python
not co_trong_so and vuot_nguong
```

Đọc dòng đó lên. Bạn đọc được **hai** câu hoàn toàn khác nhau:

> Ngày **không** có trong sổ, **mà lại** vượt ngưỡng.

hoặc:

> **Không phải** là ngày vừa có trong sổ vừa vượt ngưỡng.

Câu thứ nhất chỉ khớp với những ngày chưa ghi sổ. Câu thứ hai khớp với gần như
mọi ngày — chỉ trừ đúng loại ngày có đủ cả hai. Hai câu, hai tập ngày khác hẳn
nhau, cùng sinh ra từ một dòng chữ.

Chỗ mập mờ nằm ở chỗ đặt chữ: `not` viết **trước** một câu, còn `and` viết
**giữa** hai câu. Nên trên cùng một dòng, không có gì nói cho bạn biết `not`
lật đến đâu thì dừng.

Vậy máy chọn cách đọc nào? Nó lật mỗi `co_trong_so`, hay lật cả cụm
`co_trong_so and vuot_nguong`? Và nó dựa vào đâu để chọn — hay nó cũng phân vân
như bạn? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
