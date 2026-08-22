---
id: nen-tang.gia-tri-bien-kieu.so-thuc-chi-la-so-xap-xi
title: Số thực chỉ là số gần đúng
summary: Máy ghi số thực bằng hệ hai, nên những con số như `0.1` chỉ được ghi gần đúng — cộng vài khoản là phần lệch trồi lên.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.float-precision]
requires: [core.float, core.division, core.arithmetic, core.bit, core.boolean, ctrl.comparison, core.type-fn]
concepts: [core.so-thap-phan, core.kieu-gia-tri]
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
Có những con số mình chép lại không bao giờ hết. Nên mình chép gần đúng.
::::

::::explain{#so-khong-viet-het-duoc}
Bài trước để lại một câu hỏi thẳng: sổ chợ ghi bằng nghìn, bánh mì `10.1` và
cà phê `20.2` — máy cộng ra đúng `30.3` chứ?

Trước khi chạy thử, hãy nhớ lại một con số bạn đã quen từ hồi tiểu học: một chia
ba. Viết bằng thập phân, nó là `0,3333…` và cái đuôi ấy không bao giờ dừng. Muốn
ghi nó xuống giấy thì phải cắt ở đâu đó — `0,33` chẳng hạn. Cắt xong thì ba lần
`0,33` cộng lại được `0,99`, không phải `1`.

Con số ấy lệch không phải vì bạn tính dở. Nó lệch vì **hệ mười không viết hết
được một phần ba**, và cái gì không viết hết được thì buộc phải cắt.

Máy gặp đúng chuyện đó, chỉ khác hệ đếm. Realm 0 đã nói: trong máy, mọi thứ cuối
cùng đều là **bit** — chỉ có `0` và `1`, không có tám chữ số còn lại. Số thực
trong máy cũng được ghi bằng một dãy bit, tức là ghi theo **hệ hai**.

Và trong hệ hai, một phần mười — con số `0.1` mà tay bạn viết ra gọn ghẽ — chính
là loại số không viết hết được, đúng y như một phần ba trong hệ mười. Máy buộc
phải cắt. Thứ nó cất vào là một con số **rất gần** `0.1`, không phải đúng `0.1`.

Cách ghi số này có tên: **số dấu chấm động** (tiếng Anh là *floating point*, và
`float` mà bạn gặp từ bài 1 chính là chữ viết tắt của nó). Điều cần nhớ về nó
gọn một câu: số thực trong máy là **xấp xỉ**, không phải chính xác.

Một con số đứng một mình thì phần lệch nhỏ tới mức máy in ra vẫn thấy `0.1`.
Cộng vài con số như thế lại với nhau thì phần lệch mới trồi lên chỗ mắt nhìn
thấy — và sổ chi tiêu thì toàn là cộng.
::::

::::example{#hoi-thang-cai-may}
Hỏi thẳng máy ba phép cộng:

```python title=readonly
tien_banh_mi = 10.1
tien_ca_phe = 20.2

print(tien_banh_mi + tien_ca_phe)
print(0.1 + 0.2)
print(0.25 + 0.25)
```

Máy in ra:

```text
30.299999999999997
0.30000000000000004
0.5
```

Dòng đầu là câu trả lời cho bài trước: không, máy **không** cho ra `30.3`. Nó
cho ra một con số kém `30.3` một chút xíu, và cái "một chút xíu" ấy nằm mãi tận
chữ số thứ mười lăm.

Dòng thứ hai là phiên bản nhỏ nhất của cùng câu chuyện: `0.1 + 0.2` không ra
`0.3`. Đây là ví dụ nổi tiếng nhất về số thực trong máy, và mọi ngôn ngữ lập
trình dùng số dấu chấm động đều cho ra đúng con số dài ngoằng ấy — không riêng
gì Python.

Dòng thứ ba mới là chỗ dễ bỏ qua nhất: `0.25 + 0.25` ra đúng `0.5`, khít khao,
không đuôi. Không phải số thực nào cũng lệch. `0.25` là một phần tư, và một phần
tư thì hệ hai viết hết được — nên máy cất nó vào nguyên vẹn, cộng ra cũng nguyên
vẹn.

Chỗ khó chịu nằm ở đây: nhìn vào `0.1` và `0.25`, mắt bạn không có cách nào biết
con số nào thuộc nhóm nào. Cả hai đều là hai chữ số sau dấu phẩy, cả hai đều
trông tròn trịa như nhau.
::::

::::predict{#doan-cau-tra-loi commitOnce}
Hỏi máy một câu có–không về đúng hai khoản chợ ban nãy.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
tien_banh_mi = 10.1
tien_ca_phe = 20.2
print(tien_banh_mi + tien_ca_phe == 30.3)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn cộng đúng: `10,1` cộng `20,2` bằng `30,3`, phép cộng ấy không
có chữ số nào sai, và nếu hai con số kia được cất nguyên vẹn thì bạn đã trúng
hoàn toàn.

Chỗ lệch nằm **trước** phép cộng. Hai con số máy đang giữ không phải đúng `10,1`
và `20,2` mà là hai con số rất gần chúng. Cộng hai con số hơi lệch thì được một
con số hơi lệch — và dấu `==` hỏi "có bằng đúng không", chứ không hỏi "có gần
bằng không".
::
:::

:::opt
30.3
::why
Gần đúng ở chỗ bạn đọc dấu `==` như dấu bằng trong vở toán, tức là "vế trái ra
bao nhiêu". Cách đọc ấy đúng ở mọi chỗ khác trong đời, và câu `10,1 + 20,2 =
30,3` viết trên giấy thì không ai bắt bẻ được.

Chỗ lệch: Realm 0 đã dạy `==` là một **câu hỏi có–không**, không phải một lời
tuyên bố. Thứ đi ra khỏi nó luôn là `True` hoặc `False`, chưa bao giờ là một con
số. Muốn thấy con số thì bỏ hẳn phần `== 30.3` đi.
::
:::

:::opt
30.299999999999997
::why
Gần đúng ở chỗ bạn nhớ chính xác con số thật mà máy đang giữ — nó vừa hiện ra ở
ví dụ phía trên, và bạn ghi lại không sai một chữ số nào. Đó là bằng chứng bạn
đã nắm được điều bài này muốn nói.

Chỗ lệch nằm ở việc dòng lệnh này in cái gì. Nó không in tổng; nó in **câu trả
lời cho một câu hỏi**. Con số bạn nhớ chính là lý do câu trả lời ấy là `False`,
nhưng bản thân con số thì không được in ra.
::
:::
::::

::::explain{#hai-so-trong-giong-nhau}
Rút ra một câu để mang theo: **hai con số in ra giống hệt nhau vẫn có thể khác
nhau với máy.**

Đây là chỗ nguy hơn mọi lỗi bạn đã gặp ở Realm 0. `TypeError` và `ValueError`
đều dừng chương trình lại và chỉ vào đúng dòng. Còn chuyện này thì máy không báo
gì cả — nó chỉ lặng lẽ trả lời `False` cho một câu mà bạn tin chắc là `True`,
rồi chương trình chạy tiếp như không có gì.

Cũng đừng nghĩ đây là chỗ hỏng của riêng Python. Mọi ngôn ngữ dùng số dấu chấm
động đều thế, và cả cái máy tính bỏ túi trên bàn cũng thế — nó chỉ giấu kỹ hơn,
bằng cách hiện ít chữ số hơn số nó đang giữ.
::::

::::code{#hoi-may-hai-cau}
Byte muốn tự tay xác nhận, trên đúng sổ chi tiêu ghi bằng nghìn của mình:

- **Sáng**: bánh mì `10.1` cộng cà phê `20.2` — trên giấy ra `30.3`.
- **Chiều**: trà đá `3.5` cộng gửi xe `5.25` — trên giấy ra `8.75`.

Mỗi buổi in hai dòng: dòng trên là con số thật máy đang giữ (đã viết sẵn), dòng
dưới là câu trả lời cho câu hỏi *"nó có bằng đúng con số tôi cộng trên giấy
không"*. Hai chỗ trống là hai câu hỏi ấy.

Bài chấm bằng cả hai buổi, và hai buổi này được chọn để cho ra hai câu trả lời
**khác nhau**: một buổi lệch, một buổi khít. Điền cứng `False` vào cả hai chỗ
thì buổi chiều sai; điền cứng `True` thì buổi sáng sai. Chỉ một câu hỏi viết
thật mới qua được cả hai.

```python title=starter
tong_sang = 10.1 + 20.2
tong_chieu = 3.5 + 5.25

print(tong_sang)
print(___)
print(tong_chieu)
print(___)
```

```python title=solution
tong_sang = 10.1 + 20.2
tong_chieu = 3.5 + 5.25

print(tong_sang)
print(tong_sang == 30.3)
print(tong_chieu)
print(tong_chieu == 8.75)
```

```python title=test
# Hai buổi cho ra hai câu trả lời ngược nhau (False rồi True), nên một chữ
# `True` hay `False` gõ cứng chỉ qua được nhiều nhất một dòng — và tầng chấm
# hình dạng bên dưới đòi có thật hai phép so sánh bằng trong bài.
#
# Ba assert này chốt lại chính điều bài vừa dạy, để nếu một ngày nào đó máy
# chạy bài học đổi cách lưu số thực thì cổng đỏ lên chứ không dạy sai lặng lẽ.
assert tong_sang != 30.3, "10.1 + 20.2 KHÔNG bằng đúng 30.3 — đó là cả nội dung bài này"
assert tong_chieu == 8.75, "3.5 + 5.25 thì bằng đúng 8.75 — không phải số thực nào cũng lệch"
assert 30.2999 < tong_sang < 30.3, "phần lệch phải cực nhỏ và nằm ở phía DƯỚI 30.3 — không phải một sai số nhìn thấy được"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều nằm trong `print`, ngay dưới một dòng đã in ra con số thật. Việc của chúng không phải in lại con số ấy lần nữa, mà là hỏi máy một câu chỉ có hai câu trả lời.
- kind: strategy
  body: Câu hỏi ấy gồm ba phần: cái tên giữ tổng, dấu so sánh bằng của Realm 0 (hai dấu bằng viết liền nhau), và con số bạn cộng ra trên giấy. Buổi sáng cộng trên giấy ra 30.3, buổi chiều ra 8.75 — hai con số khác nhau nên hai dòng không chép được cho nhau.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `tong_sang == 30.3` và `___` thứ hai bằng `tong_chieu == 8.75`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh bằng (`==`) giữa tổng và con số bạn mong đợi — gõ thẳng `True` hay `False` thì không hỏi máy điều gì cả
  requireAst:
  # `min: 2` vì có hai buổi, mỗi buổi một câu hỏi. Khung chưa có dấu `==` nào,
  # nên luật này chặn được đúng cái đáp án gõ cứng hai chữ True/False.
  - kind: uses-operator, target: ==, min: 2
- tier: output
  match: regex
  expect: ^30\.299999999999997\nFalse\n8\.75\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một buổi lệch, một buổi khít. Mình không giấu bạn con số thật đâu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Máy vừa nói thẳng vào mặt bạn: một tổng trông y hệt `30.3` mà **không** bằng
`30.3`.

Sổ chi tiêu thì đầy những câu hỏi có–không kiểu ấy. Khách đưa đủ tiền chưa. Hôm
nay tiêu đúng bằng hạn mức chưa. Hai người góp bằng nhau chưa. Viết chúng bằng
`==` trên những con số ghi bằng nghìn thì có ngày máy trả lời sai — mà không nổ
một dòng lỗi nào để bạn lần ra.

Nhưng nhìn lại bài 4 và bài 5 mà xem. Ở đó bạn đếm bằng **đồng**, mọi con số đều
tròn trịa, và `//` với `%` chưa sai một lần nào.

Vậy cùng một khoản tiền, ghi `25.5` nghìn và ghi `25500` đồng là hai lựa chọn
khác hẳn nhau, chứ không phải hai cách viết cùng một thứ. Có luật nào nói cho
bạn biết lúc nào chọn cách nào không?

Bài sau trả lời, và câu trả lời gói gọn trong hai chữ.
::::

::::checkpoint{mastery=0.8}
::::
