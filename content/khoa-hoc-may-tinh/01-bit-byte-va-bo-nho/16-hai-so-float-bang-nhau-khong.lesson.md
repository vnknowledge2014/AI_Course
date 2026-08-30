---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.hai-so-float-bang-nhau-khong
title: Đừng hỏi hai số lẻ có bằng nhau không
summary: "`==` hỏi hai số có trùng từng bit không — câu hỏi sai để hỏi với số thực. Hỏi 'chênh nhau ít hơn một ngưỡng' mới đáng hỏi, và nó có ích ngay từ bài toán tính tiền hôm sau."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.float-compare]
requires: [mem.float-layout, ctrl.comparison]
concepts: [mem.so-sanh-gan-dung, mem.nguong-sai-so]
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
`0,1 + 0,1 + 0,1 == 0,3` — mình đoán bạn đã nghi ngờ câu trả lời rồi.
::::

::::explain{#cau-hoi-sai-de-hoi}
Hỏi máy thẳng:

```python title=readonly
can_sang = 0.1 + 0.1 + 0.1
can_that = 0.3
print(can_sang)
print(can_sang == can_that)
```

```text title=readonly
0.30000000000000004
False
```

Đúng như bài trước đã báo trước: `0,1` không được cất đúng, cộng ba lần
thì phần lệch nhỏ xíu ấy trồi hẳn lên. `==` trả lời `False` — không sai,
máy chỉ đang trả lời ĐÚNG câu hỏi bạn hỏi nó.

Vấn đề nằm ở chính câu hỏi. `==` hỏi "hai dãy bit này có TRÙNG NHAU TUYỆT
ĐỐI không" — từng bit một, không sai một ly. Với số thực đã đi qua vài
bước cộng trừ, đó gần như luôn là câu hỏi sai để hỏi. Cân ba lần rồi cộng
dồn ba con số `0,1` không cần phải trùng bit-từng-bit với `0,3` mới đáng
được gọi là "gần như nhau" trong đời thật của một cân rau.

Câu hỏi đáng hỏi hơn: **"hai con số này chênh nhau ÍT HƠN một ngưỡng nhỏ
không?"** Viết bằng Python:

```python title=readonly
print(abs(can_sang - can_that) < 1e-9)
```

```text title=readonly
True
```

`abs(...)` là **giá trị tuyệt đối** — bỏ dấu âm nếu có, để "chênh lệch"
luôn là một số không âm dù bên nào lớn hơn. `1e-9` là cách Python viết số
rất nhỏ: một phần tỉ. Con số đó được chọn lớn hơn hẳn phần lệch do máy xấp
xỉ gây ra — cỡ một phần trăm nghìn tỉ — nhưng vẫn nhỏ hơn hẳn bất cứ chênh
lệch nào bạn thật sự quan tâm trong đời sống. Không có cân rau nào lệch
đúng một phần tỉ ký mà còn đáng để ý.
::::

::::example{#khong-phai-luc-nao-cung-lech}
Đừng rút ra kết luận "mọi phép cộng số thực đều lệch". Hai phép cộng sau
trông giống hệt nhau về hình dạng, mà một cái lệch, một cái không:

```python title=readonly
a = 0.1 + 0.1 + 0.1
b = 0.3
c = 0.25 + 0.25
d = 0.5

print(a == b)
print(c == d)
```

```text title=readonly
False
True
```

`0,25` và `0,5` đều là những phần tư, phần nửa — bài trước đã cho thấy
`(0,5).hex()` có phần định trị toàn số 0, nghĩa là hệ hai cất nó KHÍT,
không dư một bit. Cộng hai con số cất khít thì ra một con số cũng khít,
không có gì để mà lệch.

`0,1` thì khác: phần định trị của nó là một dãy tuần hoàn bị cắt — bài 14
đã chỉ đúng chỗ cắt ấy. Ba lần cộng ba con số ĐÃ xấp xỉ thì phần lệch có
cơ hội chồng lên nhau và trồi ra ngoài.

Không có cách nào NHÌN vào `0,1` với `0,25` mà biết ngay số nào thuộc
nhóm nào — cả hai đều trông tròn trịa như nhau trên màn hình. Đó chính là
lý do `==` không đáng tin cậy: bạn không biết trước lúc nào nó sẽ nói dối.
::::

::::predict{#doan-hai-dong commitOnce}
Đổi sang một cặp số khác, chưa chạy ở đâu trong bài này. **Trước khi
chạy**, bạn đoán hai dòng này in ra gì?

```python
e = 0.3 + 0.3 + 0.3
f = 0.9
g = 0.125 + 0.125
h = 0.25
print(e == f)
print(g == h)
```

:::opt{correct}
`False` rồi `True`
:::

:::opt
`True` rồi `True`
::why
Gần đúng ở chỗ bạn nắm chắc bài học chính của cả mạch này: cộng số thực có
thể lệch khỏi kết quả trên giấy, và bạn mang đúng nỗi ngờ vực ấy vào cả
hai dòng.

Chỗ lệch: KHÔNG PHẢI mọi phép cộng số thực đều lệch. `0,125` là một phần
tám — hệ hai cất nó khít không dư một bit, đúng như `0,25` và `0,5` ở
phần trên — nên `0,125 + 0,125` ra đúng khít `0,25`. Dòng hai phải là
`True`.
::
:::

:::opt
`False` rồi `False`
::why
Gần đúng ở chỗ dòng đầu bạn đoán trúng: ba lần cộng `0,3` đúng là không
khớp `0,9`.

Chỗ lệch nằm ở dòng hai. `0,125` không thuộc nhóm số bị cắt như `0,3` —
mẫu số của nó chỉ toàn thừa số 2 (`0,125 = 1/8`), khớp thẳng cơ số của hệ
hai, nên cộng lại ra khít, không lệch một bit.
::
:::

:::opt
`True` rồi `False`
::why
Gần đúng ở chỗ bạn đoán đúng NHỊP của câu hỏi: một dòng phải `True`, một
dòng phải `False` — không phải cả hai giống nhau.

Chỗ lệch là gán ngược dòng nào cho kết quả nào. Ba lần cộng `0,3` là phép
CHƯA TỪNG có cơ hội khít — mỗi lần cộng, phần lệch nhỏ lại chồng thêm một
lớp, nên dòng đó phải là `False`. Còn `0,125 + 0,125` khít tuyệt đối ngay
từ đầu, nên dòng đó phải là `True`.
::
:::
::::

::::code{#viet-ham-gan-bang}
Viết một hàm `gan_bang` trả lời đúng câu hỏi nên hỏi: hai số này chênh
nhau ít hơn một ngưỡng không — thay vì hỏi chúng có trùng bit tuyệt đối
không.

```python title=starter
def gan_bang(a, b, nguong=1e-9):
    return ___

can_sang = 0.1 + 0.1 + 0.1
can_that = 0.3

print(can_sang == can_that)
print(gan_bang(can_sang, can_that))
print(gan_bang(0.1, 0.2))
```

```python title=solution
def gan_bang(a, b, nguong=1e-9):
    return abs(a - b) < nguong

can_sang = 0.1 + 0.1 + 0.1
can_that = 0.3

print(can_sang == can_that)
print(gan_bang(can_sang, can_that))
print(gan_bang(0.1, 0.2))
```

```python title=test
assert gan_bang(0.1 + 0.1 + 0.1, 0.3) == True, "0,1 cộng ba lần phải được coi là gần bằng 0,3 — đây chính là ca mà == nói dối"
assert gan_bang(0.1, 0.2) == False, "0,1 và 0,2 khác nhau THẬT, không được coi là gần bằng"
assert gan_bang(0.2, 0.1) == False, "đổi chỗ hai tham số không được đổi kết quả"
assert gan_bang(1.0, 1.0000000001, nguong=1e-6) == True, "chênh lệch nhỏ hơn ngưỡng 1e-6 thì phải được coi là gần bằng"
assert gan_bang(1.0, 1.01, nguong=1e-6) == False, "chênh lệch 0,01 lớn hơn hẳn ngưỡng 1e-6 thì không được coi là gần bằng"
assert gan_bang(1.0, 1.000001) == False, "chênh lệch một phần triệu (1e-6), với ngưỡng MẶC ĐỊNH 1e-9, phải là False — nếu bạn lỡ dùng (a - b) ** 2 < nguong thay vì abs(a - b) < nguong, phần bình phương làm khoảng lệch trông nhỏ hơn thật, và ca này lộ ra ngay"
```

:::hints
- kind: attention
  body: Cả hàm chỉ có một dòng `return` — không cần vòng lặp, không cần `if`. Nhìn lại đúng câu hỏi ở phần giải thích — "chênh nhau ÍT HƠN một ngưỡng" — hai cụm viết hoa đó là hai việc phải làm.
- kind: strategy
  body: "'Chênh nhau' là hiệu của hai số, nhưng hiệu có thể âm nếu `a` nhỏ hơn `b` — bọc nó trong `abs(...)` để luôn dương, dù đưa vào theo thứ tự nào. 'Ít hơn một ngưỡng' là một phép so sánh `<` với tham số `nguong`. Ghép hai việc lại thành một biểu thức duy nhất."
- kind: one-line
  body: "`return abs(a - b) < nguong`"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^False\nTrue\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một ngưỡng nhỏ, một dấu `<` — và `==` không còn được quyền nói dối nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`gan_bang` giải quyết được việc SO SÁNH. Nhưng nó không giải quyết được
một chuyện khác: mỗi lần CỘNG DỒN số lẻ, một chút xíu lệch vẫn âm thầm
chất lên — cân rau muống mười lần, cộng dồn tiền mua giống hai chục
khoản, phần lệch nhỏ xíu ấy có ngày cũng trồi ra thấy được, và lúc đó bạn
phải nhớ gọi `gan_bang` ở đúng chỗ mới tránh được rắc rối.

Có cách nào tránh HẲN chuyện phải nhớ ấy không — né toàn bộ vấn đề, thay
vì xử lý nó mỗi lần nó xuất hiện?

Nhìn lại quán cô Bảy. Từ Realm 0 tới giờ, tô phở giá `40000`, không phải
`40.0` hay `40,000` nghìn. Đĩa quẩy `10000`. Trà đá `5000`. Không một tô
phở nào, không một ly trà nào, từng được ghi bằng một con số có dấu
chấm.

Đó có phải một sự tình cờ không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
