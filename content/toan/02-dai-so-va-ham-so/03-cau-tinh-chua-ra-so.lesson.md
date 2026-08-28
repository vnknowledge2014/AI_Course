---
id: toan.dai-so-va-ham-so.cau-tinh-chua-ra-so
title: Câu tính chưa ra số
summary: Một câu tính còn ô trống thì chưa phải một con số — nhưng nó là một vật thật: có hình dạng, đọc được, chép được, so với vật khác được.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.expression]
requires: [math.letter-names-a-slot, math.placeholder-many-values, math.multiplication, math.order-of-operations, math.parentheses, core.name-lookup, err.name-error, core.variable, core.assignment, core.print-variable, core.number-literal, core.arithmetic, core.output]
concepts: [math.xe-banh-mi, math.o-trong, math.ky-hieu]
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
Sáu giờ sáng, tờ giấy dán rồi, chưa khách nào tới. Trên đó đang có cái gì?
::::

::::explain{#chua-phai-mot-con-so}
Câu hỏi để lại từ bài trước: `15 000 × n` đã là một con số chưa?

**Chưa.** Một con số thì chỉ ra được một chỗ trên thanh số. Thử chỉ xem
`15 000 × n` nằm ở vạch nào — không chỉ được, vì nó chưa nhận số nào. Điền 4 thì
nó là 60 000, điền 9 thì nó là 135 000. Một con số không đứng ở hai chỗ.

Nhưng "chưa phải một con số" không có nghĩa là **không có gì**. Tờ giấy vẫn đang
dán trên xe, cả buổi, và không ai coi nó là hỏng. Nó là một cái gì đó thật —
việc còn lại là nói cho ra nó là cái gì.
::::

::::example{#cam-len-duoc}
Cách nhanh nhất để biết một thứ có thật hay không: thử **làm gì đó với nó**.

Chưa điền một con số nào vào `15 000 × n`, Byte vẫn làm được cả bốn việc sau.

**Đọc nó thành lời.** *"Mười lăm nghìn nhân với n."* Đọc lên là người kia hiểu,
không cần biết n bằng bao nhiêu.

**Chép nó sang tờ khác.** Byte chép nguyên dòng ấy vào sổ, và hôm sau đọc lại
vẫn đúng dòng ấy. Chép được nghĩa là có cái để chép.

**Đếm bên trong nó có gì.** Một ô trống, một phép nhân, một con số cụ thể.
`15 000 × n − 9 000 × n` thì khác: vẫn một ô trống, nhưng ô ấy có mặt ở hai chỗ,
và có ba phép tính.

**So nó với một vật khác.** Đây là việc trả công nhiều nhất. Hai dòng sau chỉ
khác nhau một cặp ngoặc:

```text
   15 000 × (n + 2)      gói "n cộng hai" thành MỘT cụm, rồi mới nhân
   15 000 × n + 2        "15 000 nhân n" là một cụm, rồi cộng thêm hai
```

Trên xe của Byte, hai dòng ấy kể hai chuyện khác hẳn nhau. Dòng trên: bán n ổ,
cộng thêm **hai ổ** để phần nhà, tất cả đều tính tiền. Dòng dưới: bán n ổ, rồi
cộng thêm **hai đồng** — hai đồng, không phải hai ổ.

Và bạn phân biệt được hai chuyện ấy **trước khi** biết n bằng bao nhiêu. Không
phải nhờ tính ra số; nhờ nhìn vào **hình dạng**.

Bốn việc trên đều là việc làm với một cái **vật**. Nên gọi thẳng nó ra:

> Một câu tính còn ô trống gọi là một **biểu thức**. Nó chưa phải một con số,
> nhưng nó là một **vật**: có hình dạng, đọc được thành lời, chép lại được, và
> so với biểu thức khác được.
::::

::::explain{#may-thi-doi-mot-con-so}
Có một chỗ máy Python và tờ giấy dán ngoài xe không giống nhau, và biết trước
chỗ ấy thì đỡ mất công về sau.

Máy Python **không** cầm được biểu thức. Gõ `15000 * n` cho nó thì nó không cất
dòng ấy đi đợi sau; nó đi tra ngay xem `n` là cái tên đang giữ số nào. Tra không
thấy thì nó dừng lại — đúng cái dừng bạn đã gặp ở Realm 0, cái dừng có chữ
`NameError`.

Tờ giấy thì không dừng. Nó nằm trên xe suốt buổi, chưa ai điền, và vẫn là một tờ
giấy dùng được.

Nói cho gọn: **máy đòi một con số ngay; tờ giấy thì chờ được.** Cái dừng của máy
không phải bằng chứng rằng `15 000 × n` là thứ vô nghĩa — nó chỉ là bằng chứng
rằng máy Python được làm ra để tính, không phải để cầm biểu thức.
::::

::::predict{#doan-khi-chua-dien commitOnce}
Byte gõ đúng một dòng, và cố ý **không** cho `n` con số nào trước đó — đúng cảnh
sáu giờ sáng, tờ giấy dán rồi, chưa khách nào tới.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(15000 * n)
```

:::opt{correct}
Máy dừng lại và báo lỗi: `NameError: name 'n' is not defined`
:::

:::opt
Máy in ra đúng dòng chữ `15000 * n`
::why
Gần đúng ở chỗ bạn đang giữ chắc ý chính của bài này: `15 000 × n` là một vật
chép lại được, và một vật chép được thì hiện ra được. Trên tờ giấy thì đúng như
thế thật.

Chỗ lệch là ranh giới giữa **chữ** và **tên**. `print` chỉ in nguyên văn khi thứ
bên trong ngoặc là một chuỗi nằm giữa hai dấu nháy — `print("15000 * n")` thì máy
in ra đúng chín ký tự ấy. Không có nháy thì `n` là một cái **tên**, và gặp tên là
máy đi tra chứ không chép lại. Cái vật ấy sống trên tờ giấy, không sống trong
dòng lệnh này.
::
:::

:::opt
Máy in ra `0`
::why
Gần đúng ở chỗ bạn dùng một quy tắc rất chắc khi đếm: chưa có gì thì là không có,
mà không có thì viết là 0. Ngoài đời quy tắc ấy dùng suốt — giỏ chưa bỏ ổ nào vào
thì trong giỏ có 0 ổ.

Chỗ lệch là "ô chưa điền" khác hẳn "ô điền số 0". Bài đầu mạch này đã tách đúng
hai thứ đó: khách ghé xem rồi đi, không mua ổ nào — đó là một lần điền **thật**,
điền bằng số 0, và `15 000 × 0` cho ra 0 đồng. Còn sáu giờ sáng thì chưa có lần
điền nào cả. Máy cũng phân biệt đúng như vậy: nó không tự điền 0 vào chỗ chưa ai
điền, nó dừng lại và nói ra rằng chưa có gì.
::
:::

:::opt
Máy in ra `15000`
::why
Gần đúng ở chỗ bạn đang dùng một quy tắc thật của các tờ khai ngoài đời: chỗ nào
bỏ trống thì coi như không có, phần còn lại vẫn đọc bình thường. Nhiều biểu mẫu
được thiết kế đúng như thế.

Chỗ lệch là máy không bỏ qua chỗ trống nào cả. Với nó, mỗi cái tên là một câu hỏi
bắt buộc phải có câu trả lời; hỏi mà không có thì cả dòng dừng lại chứ không chạy
tiếp với phần còn lại. Đây chính là chỗ máy chặt hơn tờ giấy — và cũng là lý do
`15 000 × n` phải sống trên giấy trước, rồi mới xuống tới máy sau khi đã điền.
::
:::
::::

::::explain{#hinh-dang-la-thu-de-lam-viec}
Nếu biểu thức đã là một vật, thì việc đáng làm với nó là **nhìn ra hình dạng**.

Hình dạng của một biểu thức là câu trả lời cho câu hỏi: *cụm nào tính trước, và
phép nào là phép cuối cùng gộp mọi thứ lại?* Bạn đã có sẵn công cụ đọc nó — thứ
tự phép toán và dấu ngoặc, từ mạch trước.

Ba tờ giấy sau đây cùng nói về một buổi bán, cùng đúng một chữ `n`, mà là ba
vật khác nhau — khác ở chỗ **phép nào gộp mọi thứ lại sau cùng**:

```text
   15 000 × n + 20 000     phép cuối là CỘNG:  một cụm nhân, cộng thêm 20 000
   15 000 × (n + 2)        phép cuối là NHÂN:  cụm (n + 2) đem nhân 15 000
   15 000 × (n − 2)        phép cuối là NHÂN:  cụm (n − 2) đem nhân 15 000
```

Đọc được như thế thì chép lại không sai — mà chép sai một cặp ngoặc là kể một câu
chuyện khác về cái xe, chứ không phải "lỗi nhỏ về hình thức".
::::

::::code{#chep-lai-ba-to-giay}
Byte đang ở chợ, đọc ba tờ giấy cho bạn qua điện thoại. Bạn chép lại thành ba
câu tính.

- **Tờ A** — *"mười lăm nghìn nhân n, rồi cộng hai mươi nghìn tiền nước cả
  buổi."*
- **Tờ B** — *"n ổ cộng thêm hai ổ để phần nhà, tất cả đem nhân mười lăm nghìn."*
- **Tờ C** — *"n ổ bớt hai ổ bị cháy, còn lại đem nhân mười lăm nghìn."*

Máy không đọc được hình dạng giúp bạn. Nên ta nhờ nó làm **trọng tài**: đưa cho
cả ba tờ cùng một số ổ, rồi xem ba con số nhả ra có đúng ba con số Byte đã đọc
không. Nếu ba tờ cho ra **ba** con số khác nhau thì chắc chắn bạn đã chép ra ba
vật khác nhau, chứ không phải ba bản của cùng một vật.

Để ý chiều của câu ấy: ba số khác nhau *thì* ba tờ khác nhau. Chiều ngược lại
chưa chắc đâu — hai tờ trông khác hẳn nhau mà vẫn nhả ra cùng một số ở **mọi**
chỗ trống là chuyện có thật, và bài 6 dựng riêng để bạn gặp nó.

Cả ba chỗ trống phải viết bằng **chữ `n`**, không bằng con số 8.

```python title=starter
# Sáng nay bán 8 ổ. Đây là con số đưa cho cả ba tờ giấy.
n = 8

# Tờ A: mười lăm nghìn nhân n, rồi cộng hai mươi nghìn tiền nước
to_a = ___

# Tờ B: n ổ cộng thêm hai ổ để phần nhà, tất cả đem nhân mười lăm nghìn
to_b = ___

# Tờ C: n ổ bớt hai ổ bị cháy, còn lại đem nhân mười lăm nghìn
to_c = ___

print(to_a)
print(to_b)
print(to_c)
```

```python title=solution
# Sáng nay bán 8 ổ. Đây là con số đưa cho cả ba tờ giấy.
n = 8

# Tờ A: mười lăm nghìn nhân n, rồi cộng hai mươi nghìn tiền nước
to_a = 15000 * n + 20000

# Tờ B: n ổ cộng thêm hai ổ để phần nhà, tất cả đem nhân mười lăm nghìn
to_b = 15000 * (n + 2)

# Tờ C: n ổ bớt hai ổ bị cháy, còn lại đem nhân mười lăm nghìn
to_c = 15000 * (n - 2)

print(to_a)
print(to_b)
print(to_c)
```

```python title=test
# Năm câu `!=` canh năm cách chép sai hình dạng. Chúng đứng TRƯỚC các câu `==`
# vì chương trình dừng ở câu vỡ đầu tiên — xếp sau một câu `==` bao trùm thì
# chúng không bao giờ chạy tới, và cái bẫy không bao giờ sập.
assert to_b != 15000 * n + 2, "tờ B nói CẢ CỤM `n cộng hai` mới đem nhân — bỏ cặp ngoặc thì hai ổ phần nhà tụt xuống thành hai đồng"
assert to_c != 15000 * n - 2, "tờ C nói bớt hai Ổ rồi mới nhân — bỏ cặp ngoặc thì hai ổ cháy tụt xuống thành hai đồng"
assert to_a != to_b, "tờ A cộng 20 000 tiền nước, tờ B cộng thêm hai ổ tức 30 000 — hai khoản cộng khác nhau thì cùng 8 ổ không thể ra cùng một số"
assert to_b != to_c, "tờ B cộng thêm hai ổ, tờ C bớt đi hai ổ — hai vật khác nhau"
assert to_a != to_c, "tờ A cộng thêm tiền, tờ C bớt đi ổ — hai vật khác nhau"
# Ba tờ, ba con số — nên một con số gõ cứng chỉ qua được nhiều nhất một câu.
assert to_a == 140000, "8 ổ thu 120 000, cộng 20 000 tiền nước"
assert to_b == 150000, "8 ổ cộng 2 ổ phần nhà là 10 ổ, mỗi ổ 15 000"
assert to_c == 90000, "8 ổ bớt 2 ổ cháy còn 6 ổ, mỗi ổ 15 000"
```

:::hints
- kind: attention
  body: Với mỗi tờ, hỏi một câu duy nhất — phép nào là phép LÀM SAU CÙNG? Tờ A đọc tới "rồi cộng hai mươi nghìn" mới hết, còn tờ B và tờ C đều kết bằng "đem nhân mười lăm nghìn". Phép làm sau cùng cho biết cụm nào phải gói lại trước.
- kind: strategy
  body: Tờ nào có một cụm phải tính xong trước khi đem nhân thì cụm ấy cần một cặp ngoặc rào lại; không rào thì phép nhân chạy trước và câu chuyện đổi hẳn. Tờ A không cần rào gì, vì nhân vốn đã làm trước cộng. Cả ba tờ đều viết bằng chữ `n`, đừng thay bằng số 8.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `15000 * n + 20000`, `15000 * (n + 2)`, `15000 * (n - 2)`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: ba chỗ trống phải chép lại ba HÌNH DẠNG, mỗi chỗ dựng trên chữ `n` — gõ thẳng 140000, 150000 hay 90000 vào thì không còn tờ giấy nào để so hình dạng nữa
  requireAst:
  # Khung khởi đầu không có dấu `*` nào và không ĐỌC cái tên nào (`n = 8` là gán,
  # không tính), nên bộ luật này chặn được đáp án chép cứng ba con số. `n` phải
  # có mặt ở cả ba tờ, nên `min: 3`.
  - kind: uses-operator, target: *, min: 3
  - kind: uses-operator, target: +, min: 2
  - kind: uses-operator, target: -, min: 1
  - kind: uses-name, target: n, min: 3
  forbidAst:
  # Lưới thứ hai, chặn ba con số KẾT QUẢ. `uses-operator` đếm trên cả file nên
  # đáp án gõ cứng đúng MỘT tờ vẫn có thể lọt bộ luật trên; ba luật này chặn
  # đúng chỗ đó, và không cản cách viết hợp lệ nào — `15000 * (n + 2)` không
  # chứa nguyên văn 150000.
  - kind: has-literal, target: 140000
  - kind: has-literal, target: 150000
  - kind: has-literal, target: 90000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^140000\n150000\n90000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba tờ, ba hình dạng, ba con số. Cặp ngoặc nhỏ xíu mà đổi cả câu chuyện.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả bài này gọi `15 000 × n` là một **vật đang chờ**. Chờ thì chờ cái gì?

Chờ một con số. Và lúc nãy Byte đưa cho nó số 8 thật — máy nhả ra 140 000,
150 000, 90 000.

Nhưng nhìn kỹ chuyện vừa xảy ra đi. Trước khi đưa số 8, trên tờ giấy là một cái
vật có hình dạng: có ô trống, có phép nhân, có cặp ngoặc. Sau khi đưa số 8, còn
lại đúng **một con số** — 150 000, trơ trọi, không còn ngoặc, không còn ô trống,
không còn dấu vết nào của tờ giấy nó sinh ra từ đó.

Vậy cái vật ấy đi đâu mất? Nó **thu lại** thành con số, hay nó vẫn nằm đó và con
số chỉ là một bản in của nó?

Và câu này còn sắc hơn. Tờ giấy tính lãi có chữ `n` ở **hai** chỗ:

```text
15 000 × n − 9 000 × n
```

Byte đưa cho tờ ấy đúng một con số: 20. Con số 20 đi vào chỗ nào — chỗ đầu, chỗ
sau, hay cả hai? Và nếu chỉ đi vào một chỗ thì chỗ còn lại vẫn trống, tức là tờ
giấy vẫn chưa ra được số nào.

Bài sau nói rõ số 20 đi vào đâu, và chuyện gì xảy ra sau đó.
::::

::::checkpoint{mastery=0.8}
::::
