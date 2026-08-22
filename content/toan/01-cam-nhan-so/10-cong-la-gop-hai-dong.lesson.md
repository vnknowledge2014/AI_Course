---
id: toan.cam-nhan-so.cong-la-gop-hai-dong
title: Cộng là gộp hai đống
summary: Cộng không phải một phép tính phải học thuộc — nó là việc đổ hai đống vào làm một, và chính vì thế đổ đống nào trước cũng ra đúng một đống ấy.
locale: vi
track: toan
module: cam-nhan-so
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.addition-as-union]
requires: [math.don-vi, math.dong-goi, core.arithmetic, core.variable, core.print-variable, core.boolean, ctrl.comparison, core.string-concat]
concepts: [math.gop-hai-luong, math.doi-cho-hai-dong, math.don-vi]
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
An đổ đống hạt của bạn ấy vào đống của mình. Mình có phải đếm lại từ 1 không?
::::

::::explain{#do-chung-thi-goi-la-gi}
Bài trước để lại một cảnh: Byte hái thêm một đống hạt nữa và **đổ chung** vào
đống cũ. Câu hỏi bỏ ngỏ là "đổ chung" trong toán gọi là gì.

Trước khi đặt tên cho nó, hãy hỏi một câu thực dụng hơn — câu mà Byte đang lo
thật: **đổ chung xong thì có phải đếm lại từ 1 không?**

Mấy bài vừa rồi Byte không đếm chơi. Byte đã gom từng mười hạt thành một bó
(bài 6), đã xếp bó và hạt lẻ vào đúng cột (bài 7), đã ghi lại thành một dãy
chữ số gọn gàng. Đếm lại từ 1 nghĩa là vứt sạch chỗ công ấy đi.

Điều may mắn — và đây mới là nội dung của bài này — là **không phải đếm lại**.
Hai con số đã đếm xong vẫn còn dùng được. Việc đổ chung ấy có tên: **phép
cộng**, viết bằng dấu `+`.

Nhưng cái tên không phải thứ đáng nhớ. Thứ đáng nhớ là nghĩa của nó:

> **Cộng là gộp.** `a + b` là con số đo cái đống mà bạn được sau khi đổ đống
> `a` và đống `b` vào làm một.

Để ý một chuyện: câu trên không hề mô tả bạn phải **làm** gì. Nó mô tả một
**tình huống** — hai đống, đổ chung, được một đống. Con số đi ra là số đo của
cái đống mới ấy. Cách tính ra con số đó là chuyện tính sau; nó là gì thì đã
xong ngay ở đây.
::::

::::explain{#cung-mot-y-ve-thanh-hai-dai}
Đống hạt không phải bức tranh duy nhất. Vẽ mỗi lượng thành một **dải** — dải
dài bao nhiêu thì lượng lớn bấy nhiêu — thì gộp là nối hai dải đầu với đuôi:

```text
Đống của Byte            Đống của An
[------- 12 -------]     [--- 7 ---]

Nối lại thành một dải:
[------- 12 -------][--- 7 ---]
[--------- 19 ----------------]
```

Bức tranh dải làm lộ ra một thứ mà đống hạt giấu kín. Thử đổi chỗ hai đoạn:

```text
[--- 7 ---][------- 12 -------]
[--------- 19 ----------------]
```

Dải vẫn dài đúng bấy nhiêu. Không phải vì có một quy tắc nào bảo thế, mà vì
**độ dài của dải không phụ thuộc vào việc bạn đặt đoạn nào trước**.

Nói lại bằng đống hạt cho khớp: đổ đống của An vào đống của Byte, hay đổ đống
của Byte vào đống của An, thì cuối cùng cũng đúng một đống nằm trên bàn. Cái
đống ấy không nhớ ai được đổ trước.

Đây là câu trả lời cho "vì sao đúng", không phải cho "làm thế nào". Và nó đắt
hơn một quy tắc học thuộc: quy tắc thì phải nhớ, còn bức tranh thì mỗi lần
nghi ngờ bạn vẽ lại được.
::::

::::example{#hoi-thu-cai-may}
Bắt máy nói lại đúng chuyện ấy bằng hai con số của buổi sáng:

```python title=readonly
byte_hai = 12
an_hai = 7

print(byte_hai + an_hai)
print(an_hai + byte_hai)
```

Máy in ra:

```text
19
19
```

Máy xác nhận **con số**, và chỉ con số. Nó không biết vì sao hai dòng phải
giống nhau — lý do nằm ở bức tranh dải phía trên, không nằm trong máy. Ở track
này máy chỉ làm một việc: làm trọng tài cho những khẳng định bạn tự phát biểu
ra.
::::

::::predict{#doan-ba-dong commitOnce}
Lần này đổi cặp số: Byte chỉ hái được 3 hạt, còn An mang tới 12 hạt. Dòng cuối
hỏi máy một câu có–không.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
byte_hai = 3
an_hai = 12

print(byte_hai + an_hai)
print(an_hai + byte_hai)
print(byte_hai + an_hai == an_hai + byte_hai)
```

:::opt{correct}
15, rồi 15, rồi True
:::

:::opt
15, rồi 15, rồi False
::why
Gần đúng ở chỗ bạn nghĩ hai dòng lệnh ấy mô tả hai **việc khác nhau**, và với
phần lớn việc trên đời thì đúng là thứ tự có ý nghĩa thật: vo gạo rồi mới đổ
nước cho ra cơm, đổ nước rồi mới vo gạo thì ra một nồi khác hẳn.

Chỗ lệch nằm ở chỗ phép cộng không mô tả một **việc**, nó mô tả một **kết
quả**. Thứ đi ra không phải cách bạn đổ, mà là cái đống nằm trên bàn sau khi
đổ xong. Đống ấy chỉ có một, dù bạn đổ theo chiều nào.
::
:::

:::opt
312, rồi 123, rồi False
::why
Gần đúng ở chỗ bạn nhớ đúng một việc mà dấu `+` thật sự làm được: đặt hai mảnh
cạnh nhau thành một mảnh dài hơn. Realm 0 đã dùng nó như vậy để nối hai câu
chữ, và ở đó nghĩ thế là chính xác.

Chỗ lệch nằm ở phạm vi. Dấu `+` chỉ ghép khi **cả hai bên là chữ** — có dấu
nháy bao quanh. Ở đây `3` và `12` viết trần, nên chúng là hai con **số**, và
với hai con số thì `+` gộp lượng chứ không xếp chữ cạnh nhau.
::
:::

:::opt
15, rồi 15, rồi 15
::why
Gần đúng ở chỗ bạn đọc dấu `==` như dấu bằng trong vở toán, tức là "vế này ra
bao nhiêu". Trên giấy thì cách đọc ấy không sai chỗ nào.

Chỗ lệch: Realm 0 đã dạy `==` là một **câu hỏi có–không**, không phải một lời
tuyên bố. Thứ đi ra khỏi nó luôn là `True` hoặc `False`. Muốn thấy con số thì
bỏ hẳn phần `== an_hai + byte_hai` đi.
::
:::
::::

::::explain{#doi-cho-hai-dong}
Chốt lại thành một câu để mang theo cả track:

> **Đổi chỗ hai đống thì đống chung không đổi.** `a + b` và `b + a` là số đo
> của đúng một cái đống.

Đây không phải một luật rơi từ trên trời xuống, và cũng không phải một mẹo tính
nhẩm. Nó là **hệ quả** của việc cộng nghĩa là gộp: bạn không thể đổ hai đống
vào nhau theo hai cách rồi được hai kết quả, vì "đổ chung" chỉ có một kết quả.

Một chỗ cần cẩn thận, để sau này không hụt chân: điều vừa nói đúng cho **gộp**.
Đừng vội mang nó sang mọi thứ có hai con số hai bên. Có những phép mà đổi chỗ
là đổi hẳn câu hỏi, và bạn sẽ gặp một phép như vậy chỉ vài bài nữa thôi.
::::

::::code{#hai-buoi-trong-vuon}
Byte ghi sổ vườn hai buổi trong ngày. Mỗi buổi hai người mang hạt tới rồi đổ
chung vào một đống.

- **Sáng**: Byte hái 12 hạt, An mang tới 7 hạt.
- **Chiều**: Byte hái 5 hạt, An mang tới 20 hạt.

Hai chỗ trống là số đo của hai đống chung. Đừng chép sẵn con số tổng vào đó —
hãy viết ra phép **gộp** hai cái tên đứng ngay phía trên, để câu bạn viết còn
đúng cả khi Byte hái được nhiều hơn.

Bài chấm bằng cả hai buổi, và hai buổi cho ra hai con số khác nhau (một đống
19 hạt, một đống 25 hạt). Điền cứng một con số vào cả hai chỗ thì nhiều nhất
chỉ đúng được một buổi.

```python title=starter
# Sáng: Byte hái 12 hạt, An mang tới 7 hạt, đổ chung.
sang_byte = 12
sang_an = 7
tong_sang = ___

# Chiều: Byte hái 5 hạt, An mang tới 20 hạt, đổ chung.
chieu_byte = 5
chieu_an = 20
tong_chieu = ___

print(tong_sang)
print(tong_chieu)
```

```python title=solution
# Sáng: Byte hái 12 hạt, An mang tới 7 hạt, đổ chung.
sang_byte = 12
sang_an = 7
tong_sang = sang_byte + sang_an

# Chiều: Byte hái 5 hạt, An mang tới 20 hạt, đổ chung.
chieu_byte = 5
chieu_an = 20
tong_chieu = chieu_byte + chieu_an

print(tong_sang)
print(tong_chieu)
```

```python title=test
# Hai buổi ra hai con số khác nhau, nên một con số gõ cứng vào cả hai chỗ
# trống chỉ qua được nhiều nhất một buổi.
assert tong_sang == 19, "12 hạt đổ chung với 7 hạt thì đống chung có 19 hạt"
assert tong_chieu == 25, "5 hạt đổ chung với 20 hạt thì đống chung có 25 hạt"

# Hai câu dưới chốt lại đúng điều bài này nói, viết ra thành thứ máy kiểm
# được: đổ đống nào trước cũng ra một đống ấy. Buổi chiều được chọn cố ý —
# đống của An lớn hơn đống của Byte, nên nếu "số lớn phải đứng trước" là một
# luật thật thì dòng cuối đã đỏ.
assert tong_sang == sang_an + sang_byte, "đổi chỗ hai đống thì đống chung không đổi"
assert tong_chieu == chieu_an + chieu_byte, "đổi chỗ hai đống thì đống chung không đổi"
```

:::hints
- kind: attention
  body: Nhìn hai dòng ngay phía trên mỗi chỗ trống. Ở đó đã có sẵn hai cái tên giữ số hạt của hai người, và đống chung được làm từ đúng hai đống ấy.
- kind: strategy
  body: Gộp trong toán viết bằng dấu cộng. Đặt hai cái tên ở hai bên dấu ấy, đừng đặt con số kết quả — con số kết quả là thứ máy phải tự tính ra, không phải thứ bạn chép vào.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `sang_byte + sang_an`, và `___` thứ hai bằng `chieu_byte + chieu_an`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải GỘP hai cái tên đứng ngay trên nó bằng dấu `+` — chép sẵn con số tổng thì bài không còn kiểm được gì
  requireAst:
  # `min: 2` vì có hai buổi, mỗi buổi một phép gộp. Khung khởi đầu chưa có dấu
  # `+` nào, nên luật này chặn đúng cái đáp án chép cứng 19 với 25.
  - kind: uses-operator, target: +, min: 2
  - kind: uses-name, target: sang_an, min: 1
  - kind: uses-name, target: chieu_an, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^19\n25\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
19 và 25. Mình không phải đếm lại một hạt nào cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte đang quen tay. Trên bàn còn 3 hạt lẻ và 2 cái bó — bó thì mỗi bó đúng mười
hạt (bài 6). Byte gõ `3 + 2`, và máy trả lời ngay: `5`.

Năm. Nhưng năm **cái gì**?

Không phải 5 hạt: hai cái bó kia một mình đã hai chục hạt rồi. Cũng không phải
5 bó: ba hạt lẻ chưa đủ làm nổi một bó nào.

Bài 1 nói mọi con số đều là câu trả lời cho "mấy **cái** gì". Lần này câu hỏi
ấy có trả lời được không — hay con số `5` mà máy vừa in ra thật ra không đo cái
gì cả? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
