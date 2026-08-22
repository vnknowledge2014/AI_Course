---
id: toan.cam-nhan-so.thanh-so-moi-so-mot-cho
title: Thanh số — mỗi số một chỗ
summary: Một mốc 0 và một khoảng làm đơn vị là đủ dựng ra thanh số, nơi con số thôi là một đống và thành một chỗ đứng.
locale: vi
track: toan
module: cam-nhan-so
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.thanh-so]
requires: [math.don-vi-roi-va-lien, core.output, core.variable, core.print-variable, core.arithmetic, core.float, core.boolean, ctrl.comparison]
concepts: [math.thanh-so, math.vi-tri, math.don-vi]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Mình vẽ được cái chỗ ấy. Chỉ cần một cái mốc và một cái khoảng.
::::

::::explain{#doi-buc-tranh}
Bài trước kết ở một chỗ khó chịu: ai cũng gật đầu "giữa 3 và 4 còn chỗ", nhưng
chỉ tay vào chỗ ấy thì chịu.

Lý do không nằm ở bạn, nó nằm ở **bức tranh** bạn đang dùng. Tới giờ, `3` trong
đầu bạn vẫn là một *đống ba hạt*:

```text
đống 3:   ● ● ●
đống 4:   ● ● ● ●
```

Hai cái đống là hai cái đống. Chúng không đứng cạnh nhau ở đâu cả, nên "khoảng
giữa hai đống" là một câu không có nghĩa — không có khoảng nào để chỉ vào.

Byte đổi bức tranh. Ra vườn, căng sợi dây dọc luống, rồi làm đúng ba việc:

1. Chọn một điểm làm **mốc**, gọi nó là `0`. Byte chọn đầu luống.
2. Chọn một khoảng làm **một đơn vị**. Byte chọn một sải dây.
3. Đặt cái khoảng ấy nối tiếp nhau dọc sợi dây, mỗi lần đặt xong đánh một
   vạch và ghi tên: `1`, `2`, `3`, `4`…

Ra thế này:

```text
   0    1    2    3    4    5
   ●────┼────┼────┼────┼────┼───→
```

Cái hình ấy có tên: **thanh số**.

Và nó vừa làm một việc mà đống hạt không làm được. Trên thanh số, `3` không
còn là một đống nữa — nó là **một chỗ**, đúng một điểm, và bạn chỉ tay vào
được. Giữa chỗ của `3` với chỗ của `4` là một đoạn dây có thật, dài đúng một
sải. Cái "chỗ trống" mà bài 4 nói tới nằm gọn trong đoạn ấy:

```text
   0    1    2    3   3,5   4    5
   ●────┼────┼────┼────·────┼────┼───→
                       ↑
                  đây, chỉ được rồi
```

Thanh số có đúng **một luật** phải giữ, và nó là luật khắt khe:

> Hai vạch liền nhau cách nhau **đúng** một đơn vị. Không co, không giãn,
> không có chỗ nào thưa hơn chỗ nào.

Luật ấy nghe như chuyện vẽ cho đẹp, nhưng không phải. Nếu khoảng cách được
phép co giãn thì "chỗ" mất nghĩa: cùng một điểm trên dây có thể đọc thành `3`
hay thành `4` tuỳ người vẽ, và cả bức tranh hỏng.
::::

::::example{#do-lai-ba-cai-vach}
Byte đo lại chỗ đứng của ba cái vạch, tính từ mốc `0`, rồi bắt máy kiểm luật
khoảng cách:

```python title=readonly
vach_1 = 1
vach_2 = 2
vach_3 = 3

print(vach_2 - vach_1)
print(vach_3 - vach_2)
print(vach_2 - vach_1 == vach_3 - vach_2)
```

Máy in ra:

```text
1
1
True
```

Hai khoảng bằng nhau, nên sợi dây này là một thanh số đàng hoàng.

Để ý cách hỏi: Byte không hỏi "ba cái vạch có tên đẹp không". Byte hỏi **hai
cái khoảng** có bằng nhau không. Tên vạch là thứ người viết lên; chỗ đứng mới
là thứ sợi dây thật sự có.
::::

::::predict{#doan-soi-day-hong commitOnce}
Lần này Byte vội, đánh dấu bằng mắt chứ không đo. An mang thước ra đo lại ba
cái vạch ấy và ghi được: vạch mang tên `1` nằm ở 1 sải, vạch `2` ở 2 sải, còn
vạch `3` thì ở tận **4 sải**.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
vach_1 = 1
vach_2 = 2
vach_3 = 4

print(vach_2 - vach_1 == vach_3 - vach_2)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn đọc ba cái **tên** — `vach_1`, `vach_2`, `vach_3` — và thấy
chúng đi đều một nấc mỗi bước. Nếu cứ nhìn tên mà xét thì sợi dây này chuẩn
thật, và thói quen đọc tên ấy sẽ còn có ích ở bài 7.

Chỗ lệch: thanh số không nói về tên vạch, nó nói về **chỗ đứng**. Chỗ đứng thật
là ba con số bên phải dấu `=`: 1, 2 và **4**. Từ vạch 2 tới vạch 3 dài gấp đôi
từ vạch 1 tới vạch 2, nên cái vạch mang tên `3` đang đứng nhầm chỗ.
::
:::

:::opt
1
::why
Gần đúng ở chỗ bạn tính đúng vế bên trái: `2 - 1` cho ra `1`, không sai một
chút nào.

Chỗ lệch là ở dấu `==`. Realm 0 đã dạy nó là một **câu hỏi có–không**, không
phải một phép tính: thứ đi ra khỏi nó luôn là `True` hoặc `False`. Muốn thấy
con số `1` thì bỏ hẳn nửa sau của dòng đi, chỉ để lại `print(vach_2 - vach_1)`.
::
:::

:::opt
Máy báo lỗi, vì `==` chỉ đứng được giữa hai con số chứ không đứng giữa hai phép trừ
::why
Gần đúng ở chỗ bạn nhớ chắc một điều đúng: `==` cần có một **giá trị** ở mỗi
bên thì mới so được. Cảnh giác ấy sẽ cứu bạn ở nhiều chỗ khác.

Chỗ lệch là phạm vi của chữ "giá trị". Mỗi bên của `==` có thể là cả một phép
tính; máy tính xong vế trái ra `1`, tính xong vế phải ra `2`, rồi mới đem hai
con số ấy ra so. Phép trừ làm xong trước, việc so làm sau.
::
:::
::::

::::explain{#hai-buc-tranh-cung-mot-so}
Từ đây bạn có **hai bức tranh** cho cùng một con số, và cả hai đều đúng:

- **Đống** — `3` là ba hạt nằm trong tay. Bức tranh này trả lời "có bao nhiêu".
- **Chỗ** — `3` là một điểm trên thanh số, cách mốc `0` đúng ba sải. Bức tranh
  này trả lời "đứng ở đâu", và nó cho bạn cái mà bức kia không có: **khoảng
  cách giữa hai số**.

Người chỉ có bức tranh "đống" sẽ đứng hình ngay lần đầu gặp `3,5` — vì không có
đống nào ba hạt rưỡi. Người có thêm bức tranh "chỗ" thì chỉ tay vào `3,5` được
ngay, và điều đó đúng với cả những số mà về sau bạn còn chưa gặp.

Một điều nữa đáng cất đi: mốc `0` và cái khoảng đơn vị đều là do **người chọn**.
Byte chọn đầu luống làm `0` và một sải làm đơn vị; chọn khác thì cùng một luống
đất ra một con số khác — đúng như bài 3 đã nói. Nhưng chọn xong rồi thì phải
giữ nguyên suốt bức tranh, nếu không thì luật khoảng cách gãy.
::::

::::code{#dat-vach-cho-dung-cho}
Sợi dây căng dọc luống nhà An. Mốc `0` ở đầu luống, mỗi sải một vạch. An đo lại
và chỉ ghi được chỗ của hai cái vạch — vạch `3` bị mưa xoá mất.

Điền hai chỗ trống:

1. `vach_3` — chỗ mà vạch mang tên `3` phải đứng để sợi dây này còn là một
   thanh số.
2. `giua_3_va_4` — cái chỗ trống mà bài 4 nói tới: điểm nằm **chính giữa** vạch
   `3` và vạch `4`.

Bài chấm bằng cả hai chỗ và bằng bốn câu hỏi khác nhau, trong đó có câu đòi hai
bên phải cách đều. Một con số gõ cứng vào cả hai chỗ trống thì trượt ngay câu
đầu tiên.

```python title=starter
vach_2 = 2
vach_5 = 5

# 1) Vạch mang tên "3" phải đứng ở chỗ nào?
vach_3 = ___

# 2) Điểm nằm CHÍNH GIỮA vạch 3 và vạch 4.
giua_3_va_4 = ___

print(vach_3)
print(giua_3_va_4)
```

```python title=solution
vach_2 = 2
vach_5 = 5

# 1) Vạch mang tên "3" phải đứng ở chỗ nào?
vach_3 = 3

# 2) Điểm nằm CHÍNH GIỮA vạch 3 và vạch 4.
giua_3_va_4 = 3.5

print(vach_3)
print(giua_3_va_4)
```

```python title=test
# Hai câu đầu kiểm luật khoảng cách theo hai hướng khác nhau: một bước sang
# trái tới vạch 2, hai bước sang phải tới vạch 5. Một con số đặt sai chỗ khó
# lọt được cả hai.
assert vach_3 - vach_2 == 1, "hai vạch liền nhau cách nhau đúng một đơn vị — vạch 3 phải đứng cách vạch 2 đúng 1 sải"
assert vach_5 - vach_3 == 2, "từ vạch 3 tới vạch 5 là hai bước, mỗi bước một sải"
assert vach_3 < giua_3_va_4 < vach_3 + 1, "chỗ giữa phải nằm HẲN trong đoạn từ vạch 3 tới vạch 4"
assert giua_3_va_4 - vach_3 == (vach_3 + 1) - giua_3_va_4, "chính giữa nghĩa là hai bên cách đều — không lệch về vạch nào"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất bị kẹp giữa hai con số đã cho sẵn ở trên là `vach_2` và `vach_5`. Chỗ trống thứ hai không nằm trên vạch nào cả — nó nằm giữa hai vạch.
- kind: strategy
  body: Trên thanh số, vạch mang tên nào thì đứng cách mốc 0 đúng bấy nhiêu đơn vị, nên chỗ trống thứ nhất là một số nguyên. Chỗ trống thứ hai thì lấy điểm chính giữa của đoạn từ vạch 3 tới vạch 4, giống hệt cách bẻ đôi ở bài trước.
- kind: one-line
  body: "Viết `3` vào chỗ trống thứ nhất và `3.5` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giờ mình chỉ tay vào được. Số ba rưỡi ở ngay đây, giữa hai cái vạch.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Thanh số dựng xong rồi, và nó chỉ được đúng một chỗ cho mỗi con số. Bây giờ
Byte đổ ra một đống hạt to và muốn chỉ tay vào chỗ của nó trên thanh: **137
hạt**.

Muốn tới được chỗ ấy thì phải đi qua 137 cái vạch, mà muốn có 137 cái vạch thì
phải đặt cái khoảng một sải xuống 137 lần. Đặt tới lần thứ sáu mươi mấy là mắt
lạc, tay run, và đếm lại từ đầu.

Vậy có cách nào nói ra `137` mà **không** phải đếm 137 lần không? Ngoài chợ
người ta bán một trăm quả trứng suốt ngày mà chẳng ai đếm tới một trăm — họ
làm thế nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
