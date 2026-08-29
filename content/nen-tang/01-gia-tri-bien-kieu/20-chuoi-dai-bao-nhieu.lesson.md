---
id: nen-tang.gia-tri-bien-kieu.chuoi-dai-bao-nhieu
title: Chuỗi dài bao nhiêu
summary: Máy đếm hộ bạn số ký tự của một chuỗi, kể cả những ký tự mắt bạn nhìn xuyên qua.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.len]
requires: [core.string-sequence, core.function-call, ctrl.comparison, ctrl.else]
concepts: [core.chuoi, core.do-dai]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Mắt bạn nhìn xuyên qua hạt trắng. Mình thì đếm cả nó.
::::

::::explain{#hoi-nguoi-ban-xau-hat}
Bài trước để lại một câu hỏi có vẻ dễ: cột "tên khoản" rộng 12 chỗ, còn
`"cà phê sữa đá"` thì dài bao nhiêu?

Bạn vừa nhẩm bằng mắt. Có thể ra 13, có thể ra 10 nếu bỏ sót ba hạt trắng. Và
đây không phải chuyện cẩn thận hay không: đếm bằng mắt hỏng đúng ở chỗ mà bài
trước vừa chỉ ra — những ký tự mắt nhìn xuyên qua.

Quay lại xâu hạt. Nếu bạn cầm một xâu thật và muốn biết nó có mấy hạt, bạn
không ngồi lần từng hạt. Bạn đưa cho người bán và hỏi: *xâu này mấy hạt?*

Python có sẵn một cái tên làm đúng việc ấy: **`len`**. Đọc là "len", viết tắt
của *length* — độ dài.

Cách gọi nó y hệt `print`, `int` hay `round` mà bạn đã gọi hàng chục lần: viết
tên, mở ngoặc, đặt giá trị vào trong, đóng ngoặc.

```python
len("cà phê")
```

Và nó **đưa lại một con số** — cũng y hệt `int` hay `round` đưa lại một con số
cho bạn dùng tiếp. Con số đó là số ký tự của chuỗi, một `int` sạch, không có
đuôi `.0` — vì đếm hạt thì không bao giờ ra nửa hạt.

Hai điều cần nhớ về cách nó đếm:

- Nó đếm **mọi** ký tự trong dãy, không loại trừ ai. Dấu cách được đếm. Dấu
  chấm, dấu phẩy, chữ số nằm trong chuỗi — đều được đếm.
- Chuỗi rỗng `""` không có hạt nào, nên `len("")` cho `0`.
::::

::::example{#dem-that}
Ba lần hỏi, ba câu trả lời:

```python title=readonly
ten_ngan = "cà phê"
ten_dai = "cà phê sữa đá"

print(len(ten_ngan))
print(len(ten_dai))
print(len(""))
```

Màn hình hiện ra:

```text
6
13
0
```

Con số 13 là con số bài trước hỏi. Xâu nó ra từng hạt để thấy nó đến từ đâu —
mỗi ô dưới đây là một ký tự, ô trống là một dấu cách:

```text
 c  à     p  h  ê     s  ữ  a     đ  á
 1  2  3  4  5  6  7  8  9 10 11 12 13
```

Mười chữ, ba dấu cách. Đếm bằng mắt mà quên ba hạt trắng thì ra 10, và bạn sẽ
tưởng cái tên này vừa cột 12 chỗ.

Một chi tiết nữa đáng nói: `ữ` được đếm là **một**. Trên giấy nó là chữ `u` đội
thêm hai dấu, nhưng trong bảng mã mà Realm 0 đã kể, `ữ` có một chỗ riêng của
nó — một mã số, một ký tự, một hạt. Chữ tiếng Việt có dấu không tốn hai chỗ.
::::

::::predict{#doan-do-dai commitOnce}
Byte sắp hỏi độ dài của một tên khoản khác trong sổ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
ten = "gửi xe máy"
print(len(ten))
```

:::opt{correct}
10
:::

:::opt
8
::why
Gần đúng ở chỗ bạn đếm đúng tám chữ cái: `g`, `ử`, `i`, `x`, `e`, `m`, `á`,
`y`. Không sót chữ nào, và cũng không tính chữ có dấu thành hai — cả hai đều
chính xác.

Chỗ lệch là hai hạt trắng. Giữa "gửi" và "xe" có một dấu cách, giữa "xe" và
"máy" có một dấu cách nữa. Chúng nằm trong dãy y như mọi ký tự khác, và bài
trước vừa cho thấy chỉ một dấu cách thừa cũng đủ làm phép so cho `False`. `len`
đếm chúng.
::
:::

:::opt
3
::why
Gần đúng ở chỗ bạn đếm như người Việt đọc câu này: "gửi", "xe", "máy" — ba
tiếng, ba mảnh có nghĩa. Đó là cách đếm đúng nếu câu hỏi là "tên khoản này gồm
mấy chữ".

Chỗ lệch: `len` không biết tiếng nào ra tiếng nào, và cũng không biết chỗ nào
là hết một từ. Nó chỉ thấy một dãy hạt và đếm hạt. Muốn đếm theo từ thì cần một
công cụ khác hẳn, và nó không có trong bài này.
::
:::

:::opt
Máy không in số nào, nó báo lỗi vì phải viết `ten.len()`
::why
Gần đúng ở chỗ bạn nhớ `danh_sach.append(...)` từ Realm 0: có những việc thật
sự được viết bằng một dấu chấm đặt sau giá trị, và cách viết ấy tồn tại.

Chỗ lệch: `len` không thuộc nhóm đó. Nó là một cái tên có sẵn đứng riêng một
mình, gọi y như `print`, `int` hay `round` — viết tên, mở ngoặc, đặt chuỗi vào
trong. Cách viết bằng dấu chấm có tên riêng của nó và có lý do riêng của nó;
bạn sẽ gặp lại nó ở mấy bài nữa.
::
:::
::::

::::explain{#con-so-de-lam-gi}
`len` đưa lại một con số, và con số thì **so sánh được**. Đó mới là chỗ nó có
ích, chứ không phải để in ra cho biết.

Cột "tên khoản" rộng 12 chỗ. Con số 12 ấy không đổi suốt chương trình nên nó
được khai thành một hằng, viết hoa, đặt ở đầu file:

```python
BE_RONG_COT = 12
```

Và câu hỏi "tên này có vừa cột không" viết được thành một câu điều kiện bình
thường, đúng loại câu bạn đã viết từ Realm 0:

```python
if len(ten) <= BE_RONG_COT:
```

Để ý một chuyện: bạn **không** gõ thẳng con số 6 hay 13 vào đó. Gõ tay thì đổi
tên khoản một cái là câu điều kiện nói dối, và nó nói dối im lặng — không báo
lỗi gì cả. Để `len` đếm thì con số luôn khớp với cái tên đang thật sự nằm đó.
::::

::::code{#do-ten-vao-cot}
Sổ chi tiêu in ra ba dòng, mỗi dòng một tên khoản. Byte cần biết tên nào vừa
cột 12 chỗ, tên nào tràn ra ngoài.

Ba chỗ trống, mỗi khối một chỗ, đều nằm ngay trước dấu `<=`.

Bài chấm bằng **cả ba** dòng chứ không riêng dòng nào. Chấm bằng một dòng thì
không phân biệt được: với `"cà phê"` thì gõ thẳng số `6`, gõ `0`, hay gõ
`len(ten_1)` đều cho ra y hệt câu "vừa cột". Hai tên đầu kẹp mốc 12 từ hai
phía — 6 và 13 — nên chỉ cách đếm thật mới xử đúng cả hai.

Tên thứ ba đứng ĐÚNG trên mốc: `"bạc xỉu nóng"` dài đúng 12 chỗ, không thừa
không thiếu. Nó ở đây để trả lời một câu mà hai tên kia không trả lời được:
vừa **khít** cột thì tính là vừa hay là tràn? Dấu `<=` nói vừa — và đó là chỗ
duy nhất trong cả bài phân biệt được `<=` với `<`.

```python title=starter
BE_RONG_COT = 12

ten_1 = "cà phê"
ten_2 = "cà phê sữa đá"
ten_3 = "bạc xỉu nóng"

if ___ <= BE_RONG_COT:
    print(ten_1, "— vừa cột")
else:
    print(ten_1, "— tràn cột")

if ___ <= BE_RONG_COT:
    print(ten_2, "— vừa cột")
else:
    print(ten_2, "— tràn cột")

if ___ <= BE_RONG_COT:
    print(ten_3, "— vừa cột")
else:
    print(ten_3, "— tràn cột")
```

```python title=solution
BE_RONG_COT = 12

ten_1 = "cà phê"
ten_2 = "cà phê sữa đá"
ten_3 = "bạc xỉu nóng"

if len(ten_1) <= BE_RONG_COT:
    print(ten_1, "— vừa cột")
else:
    print(ten_1, "— tràn cột")

if len(ten_2) <= BE_RONG_COT:
    print(ten_2, "— vừa cột")
else:
    print(ten_2, "— tràn cột")

if len(ten_3) <= BE_RONG_COT:
    print(ten_3, "— vừa cột")
else:
    print(ten_3, "— tràn cột")
```

```python title=test
# Chấm bằng TRỌN VẸN màn hình theo đúng thứ tự dòng (`match: regex`), trên BA
# tên chứ không phải một — cộng thêm một luật `static` đòi `len` xuất hiện ít
# nhất ba lần.
#
# Vì sao cần cả hai lớp:
#   `True` / `1` / `0`      → cả hai câu điều kiện thành đúng, dòng hai in
#                             "vừa cột" trong khi nó phải "tràn cột" → trượt.
#   chép `len(ten_1)` xuống → dòng hai lại in "vừa cột" → trượt.
#   gõ tay `6` và `13`      → OUTPUT đúng, nhưng luật static thấy không có lời
#                             gọi `len` nào → trượt. Đây chính là đáp án mà
#                             riêng output không bắt được, và cũng là đáp án
#                             hỏng ngay khi ai đó sửa tên khoản.
#
# 6 và 13 kẹp mốc 12 từ hai phía, nên không có một con số cố định nào điền vào
# cả hai chỗ trống mà qua được.
#
# Tên thứ ba dài ĐÚNG 12 — đúng cái mốc. Không có nó thì `<=` và `<` cho ra y
# hệt nhau trên cả hai tên kia (6 và 13 đều không chạm mốc), nên bài dạy dấu
# `<=` mà không hề kiểm được chỗ `<=` khác `<`. Cổng đột biến chỉ ra chỗ ấy:
# đổi mọi `<=` thành `<` mà vẫn qua sạch. Bây giờ thì không qua nữa.
#
# Người học chưa biết viết assert nên khối này không thêm phép kiểm nào; nó ở
# đây để nói rõ vì sao hai tên là hai, không phải một.
pass
```

:::hints
- kind: attention
  body: Vế bên phải dấu `<=` là `BE_RONG_COT`, tức là một con số. Vậy thứ điền vào bên trái cũng phải là một con số — số ký tự của cái tên nằm ngay phía trên khối đó.
- kind: strategy
  body: Đừng tự đếm rồi gõ con số vào: gõ tay thì đổi tên khoản một cái là câu điều kiện nói dối mà không báo lỗi. Có một cái tên có sẵn nhận vào một chuỗi và đưa lại số ký tự của nó. Để ý hai khối hỏi về hai cái tên khác nhau, nên hai chỗ trống không giống nhau.
- kind: one-line
  body: Chỗ trống thứ nhất là `len(ten_1)`, chỗ trống thứ hai là `len(ten_2)`.
:::

:::validate
- tier: static
  requireAst:
  - kind: uses-call, target: len, min: 3
  onFail: bài này đo độ dài bằng `len`, không bằng con số gõ tay
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^cà phê — vừa cột\ncà phê sữa đá — tràn cột\nbạc xỉu nóng — vừa cột\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu và mười ba. Mình đếm cả ba hạt trắng mà mắt bạn đi xuyên qua.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`len` đưa bạn một con số, và con số ấy đủ để so với bề rộng cột. Nhưng để ý nó
**không** đưa bạn ký tự nào cả. Nó nói xâu có mười ba hạt; nó không nói hạt nào
là hạt nào.

Mà có việc chỉ nhìn một hạt mới làm được. Một dòng sổ in xong trông thế này:

```text
cà phê 25000đ
```

Byte muốn kiểm xem mọi dòng có kết đúng bằng chữ `đ` không — tức là chỉ cần
nhìn **ký tự cuối cùng**, phần còn lại không quan tâm.

Chuỗi là một dãy có thứ tự, nên xin riêng một chỗ trong dãy thì bạn đã biết
cách: ở Realm 0 bạn gọi một món trong danh sách bằng chỗ đứng của nó, đếm từ 0.

Có điều "cuối cùng" là chỗ số mấy? Nó đổi theo từng dòng — dòng này mười ba ký
tự, dòng kia mười một. Chẳng lẽ mỗi lần lại gọi `len` rồi trừ đi một?

Bài sau trả lời, và câu trả lời ngắn hơn phép trừ ấy.
::::

::::checkpoint{mastery=0.8}
::::
