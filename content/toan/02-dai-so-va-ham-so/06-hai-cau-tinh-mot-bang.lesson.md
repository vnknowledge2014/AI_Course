---
id: toan.dai-so-va-ham-so.hai-cau-tinh-mot-bang
title: Hai câu tính, một bảng
summary: Hai câu tính trông khác nhau vẫn có thể là một, và cách biết là so cả hai bảng — trùng ở mọi dòng mới là tương đương, trùng ở vài dòng thì chỉ là tình cờ.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.equivalent-expression]
requires: [math.value-table, math.substitution, math.expression, math.letter-names-a-slot, math.parentheses, math.order-of-operations, math.multiplication, ctrl.for-range, ctrl.if, ctrl.comparison, core.boolean, core.variable, core.fstring, core.arithmetic]
concepts: [math.o-trong, math.bieu-thuc, math.bang-gia-tri, math.tuong-duong]
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
Hai dòng chữ khác hẳn nhau. Sao hai cái bảng lại giống nhau từng dòng?
::::

::::explain{#hai-loi-tinh-mot-buoi-ban}
Bài trước để lại hai dòng chữ. Chúng cùng nói về đúng một buổi bán hàng — Byte
bán `n` ổ, nhà lấy sẵn 2 ổ ăn trưa và vẫn tính tiền như đã bán:

```text
   An viết:     15000 × (n + 2)
   Byte viết:   15000 × n + 30000
```

Nhìn thì khác nhau đủ đường. An có dấu ngoặc, Byte không. An có con số 2, Byte
có con số 30000. Nếu chỉ nhìn hình dạng thì đây là hai câu tính khác nhau, và
cãi nhau kiểu ấy thì không ai thắng.

Nhưng bài trước vừa đưa cho bạn một thứ để không phải cãi. Một câu tính có ô
trống **là cái bảng của nó**. Vậy thì đừng so hai dòng chữ — **so hai cái bảng.**

Dựng cả hai bảng lên, cùng một cột trái, mỗi bên một cột phải:

```text
   n    15000 × n + 30000      15000 × (n + 2)
   0                30000                30000
   1                45000                45000
   2                60000                60000
   3                75000                75000
   4                90000                90000
   5               105000               105000
```

Sáu dòng, không dòng nào lệch. Và đây không phải chuyện may: hai người đang đếm
cùng một mớ tiền, chỉ đếm theo hai lối. An gom hai ổ nhà ăn vào chung với số ổ
bán rồi mới nhân giá. Byte nhân giá cho số ổ bán trước, rồi cộng riêng tiền hai
ổ kia vào — mà `15000 × 2` chính là 30000.

Hai câu tính như thế có tên riêng:

> Hai câu tính là **tương đương** khi hai bảng của chúng trùng nhau ở **mọi**
> dòng — không sót dòng nào, kể cả những dòng chưa ai in ra.

Chữ **mọi** trong câu ấy gánh toàn bộ sức nặng. Bài này tồn tại vì nó.
::::

::::example{#trung-vai-dong-khong-du}
Vì sao chữ "mọi" lại quan trọng đến thế? Vì có những cặp câu tính trùng nhau ở
vài dòng rồi thôi — và nếu bạn dừng lại đúng ở dòng ấy, bạn sẽ kết luận sai.

Byte có hai khay bánh mì và hai lối đếm:

- **Khay của Byte:** `n` ổ mới nướng, cộng thêm 6 ổ để dành từ hôm qua →  `n + 6`
- **Khay của lò:** lò giao gấp ba số ổ Byte nướng →  `3 × n`

Sáng nay Byte nướng 3 ổ. Đếm cả hai khay: khay nào cũng 9 ổ. Byte reo lên: "Hai
lối đếm này là một!"

Dựng bảng ra thì thấy ngay chuyện gì đã xảy ra:

```text
   n     n + 6     3 × n
   0         6         0
   1         7         3
   2         8         6
   3         9         9   ← trùng
   4        10        12
   5        11        15
```

Đúng **một** dòng trùng. Chỉ một. Byte nướng 3 ổ nên rơi trúng dòng ấy, và tưởng
mình vừa phát hiện ra một luật.

Đặt hai cặp cạnh nhau thì ranh giới hiện rõ:

- Cặp của An và Byte: trùng ở **mọi** dòng → **tương đương**. Chúng là một câu
  tính, chỉ viết theo hai lối.
- Cặp hai cái khay: trùng ở **một** dòng → **không** tương đương. Chúng là hai
  câu tính khác nhau, tình cờ gặp nhau một chỗ.

Cùng là "hai câu tính cho cùng một số", nhưng một bên đúng luôn luôn, một bên
đúng đúng một lần. Đó là hai chuyện hoàn toàn khác nhau, và cách duy nhất phân
biệt được là **nhìn cả bảng, không nhìn một dòng**.
::::

::::predict{#doan-hai-dong commitOnce}
Byte đưa hai lối đếm khay cho máy phân xử, ở hai dòng khác nhau của bảng. Dấu
`==` bạn đã gặp ở Realm 0: nó là một câu **hỏi** — "hai bên có ra cùng một số
không?"

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 3
print(n + 6 == 3 * n)

n = 4
print(n + 6 == 3 * n)
```

:::opt{correct}
True rồi False
:::

:::opt
True rồi True
::why
Gần đúng ở chỗ bạn đọc trúng dòng đầu: ở `n` bằng 3 thì hai bên đúng là cùng ra
9, nên máy trả `True`. Và cái phản xạ đứng sau lựa chọn này — *thử một trường
hợp thấy khớp thì tin* — là phản xạ hoàn toàn hợp lý; nó là cách người ta kiểm
tra mọi thứ trong đời.

Chỗ lệch là phạm vi của nó. Một lần khớp chỉ nói được **đúng một dòng** của hai
bảng trùng nhau, không nói gì về những dòng còn lại. Mà máy thì không nhớ kết
luận của bạn: ở dòng dưới nó điền 4 vào và tính lại từ đầu — `4 + 6` ra 10,
`3 × 4` ra 12. Hai số khác nhau, nên câu trả lời là `False`.

Đây đúng là cái bẫy mà cặp hai khay dựng ra: trùng một dòng thì chưa phải tương
đương.
::
:::

:::opt
False rồi False
::why
Gần đúng ở chỗ bạn nhìn ra hai câu tính có **hình dạng khác nhau** — một bên
cộng, một bên nhân — và bạn cảnh giác với chuyện hai thứ khác hình dạng lại cho
cùng một số. Sự cảnh giác ấy đúng, và ở dòng dưới nó cho bạn câu trả lời đúng.

Chỗ lệch là dấu `==` không so **hình dạng** hai câu tính. Nó chờ mỗi bên tự thu
lại thành một con số rồi mới so hai con số ấy. Ở dòng `n` bằng 3, hai bên cùng
thu lại thành 9, nên máy trả `True` — dù hai dòng chữ vẫn khác nhau y như cũ.
Hình dạng khác chỉ khiến hai câu **thường** ra số khác, chứ không cấm chúng gặp
nhau ở đôi ba dòng.
::
:::

:::opt
9 == 9 rồi 10 == 12
::why
Gần đúng ở chỗ bạn tính không sai một con số nào: ở `n` bằng 3 hai bên đúng là 9
với 9, ở `n` bằng 4 là 10 với 12. Bạn đã làm xong đúng phần việc khó. Và trong
vở toán, viết `9` với `9` hai bên một dấu bằng là một câu hoàn chỉnh, nên chờ
thấy nó hiện ra là chờ có căn cứ.

Chỗ lệch nằm ở việc `==` trong Python trả về cái gì. Nó không chép lại câu hỏi;
nó **trả lời** câu hỏi, và câu trả lời chỉ có hai chữ: `True` hoặc `False` — đúng
như bài "đúng hay sai" ở Realm 0. Hai con số bạn vừa tính ra là thứ máy dùng để
trả lời, không phải thứ nó in ra.
::
:::
::::

::::explain{#bang-manh-den-dau}
Bảng vừa làm được một việc lớn: nó phân xử được cuộc cãi nhau mà nhìn hình dạng
thì chịu. Đáng nhớ hai điều, và điều thứ hai là chỗ hụt.

**Điều thứ nhất — bảng là trọng tài duy nhất bạn có lúc này.** Muốn biết hai câu
tính có phải một không, dựng hai bảng rồi soi từng dòng. Thấy **một** dòng lệch
là xong: hai câu **không** tương đương, và bạn không cần soi thêm dòng nào nữa.
Một dòng lệch đủ để bác bỏ.

**Điều thứ hai — chiều ngược lại thì bảng không kết luận nổi.** Soi hết trăm
dòng không thấy lệch, bạn nói được gì? Chỉ nói được: "trăm dòng ấy không lệch".
Bảng có vô số dòng. Trăm dòng, nghìn dòng, triệu dòng — vẫn còn lại vô số dòng
chưa ai soi.

Cặp hai cái khay là lời cảnh cáo có thật cho chuyện đó. Nếu Byte chỉ soi đúng
dòng `n` bằng 3, Byte kết luận hai lối đếm là một — và Byte sai. Số dòng bạn soi
càng nhiều thì bạn càng tin, nhưng **tin** không phải **chắc**.

Vậy nên cặp của An và Byte lúc này mới đứng ở mức: *soi bao nhiêu dòng cũng chưa
thấy lệch*. Câu chuyện "hai ổ nhà ăn đáng 30 000 đồng" nghe rất xuôi, và nó là
một lý do tốt — nhưng nó là một **câu chuyện về xe bánh mì**, không phải một
luật dùng được cho câu tính bất kỳ.
::::

::::code{#soi-mot-tram-dong}
Bắt máy soi một trăm dòng đầu của hai bảng, và chỉ in ra dòng nào **lệch**. Nếu
màn hình không hiện dòng lệch nào, nghĩa là trong một trăm dòng ấy hai bảng
trùng khít.

Ba chỗ trống:

- Hai chỗ đầu nằm trong vòng lặp: **chép đúng câu tính của mỗi người**, giữ
  nguyên dấu ngoặc của An và con số 30000 của Byte. Chép cả hai thành một dòng
  chữ giống nhau thì phép so này chẳng so gì cả.
- Chỗ thứ ba lấy riêng **một dòng** của bảng Byte, dòng `n` bằng 20 — in ra để
  thấy cột phải của bảng chứa **số tiền**, không chứa `True`/`False`.

Cặp hai cái khay đã viết sẵn ở cuối, không phải điền. Nó in ra dòng nào **trùng**
— và cả trăm dòng chỉ có đúng một.

```python title=starter
# An viết:    15000 × (n + 2)
# Byte viết:  15000 × n + 30000
for n in range(100):
    cua_byte = ___
    cua_an = ___
    if cua_byte != cua_an:
        print(f"lệch ở dòng {n} — {cua_byte} khác {cua_an}")

# Một dòng cụ thể của bảng Byte.
n = 20
print(___)

# Cặp hai cái khay — viết sẵn. In ra dòng nào TRÙNG.
for n in range(100):
    if n + 6 == 3 * n:
        print(f"hai khay trùng ở dòng {n}")
```

```python title=solution
# An viết:    15000 × (n + 2)
# Byte viết:  15000 × n + 30000
for n in range(100):
    cua_byte = 15000 * n + 30000
    cua_an = 15000 * (n + 2)
    if cua_byte != cua_an:
        print(f"lệch ở dòng {n} — {cua_byte} khác {cua_an}")

# Một dòng cụ thể của bảng Byte.
n = 20
print(15000 * n + 30000)

# Cặp hai cái khay — viết sẵn. In ra dòng nào TRÙNG.
for n in range(100):
    if n + 6 == 3 * n:
        print(f"hai khay trùng ở dòng {n}")
```

```python title=test
# Sau khi cả hai vòng chạy xong, `n` giữ 99 còn `cua_byte`, `cua_an` giữ dòng
# cuối của vòng thứ nhất. Ba câu `!=` đứng trước để canh ba cái bẫy: chép cứng
# một con số vào chỗ trống, nhầm cột phải với cột trái, và chép hai chỗ trống
# thành hai dòng chữ khác nghĩa. Xếp sau `==` thì chúng không bao giờ chạy tới.
assert cua_byte != 30000, "30000 là dòng đầu tiên của bảng, dòng n bằng 0 — chỗ trống phải là cả câu tính, không phải một dòng của nó"
assert cua_an != n, "cột phải của bảng ghi số tiền, không ghi lại con số vừa điền vào"
assert cua_an != 0, "câu tính của An không bao giờ ra 0: dù không bán ổ nào thì vẫn còn hai ổ nhà ăn"
assert cua_byte == cua_an, "hai câu tính tương đương thì trùng nhau ở MỌI dòng, kể cả dòng cuối vòng vừa soi"
assert cua_byte == 1515000, "dòng n bằng 99: 15000 × 99 + 30000 ra 1515000, và 15000 × (99 + 2) cũng ra đúng số ấy"
```

:::hints
- kind: attention
  body: Hai dòng chú thích ngay đầu khung đã ghi sẵn câu tính của mỗi người. Việc của bạn là chép chúng sang Python, không phải nghĩ ra câu mới. Chú ý dấu ngoặc của An — bỏ nó đi là đổi hẳn câu tính, vì nhân làm trước cộng.
- kind: strategy
  body: Trong Python, dấu nhân viết là `*` và dấu ngoặc giữ nguyên như trong vở. Câu của Byte có hai phần nối bằng dấu cộng; câu của An có đúng một phép nhân, với cả cụm trong ngoặc. Chỗ trống thứ ba lặp lại đúng câu của Byte, vì nó lấy một dòng của chính bảng ấy.
- kind: one-line
  body: "Lần lượt là `15000 * n + 30000`, `15000 * (n + 2)`, rồi `15000 * n + 30000` lần nữa."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: ba chỗ trống phải là ba câu tính viết bằng chính ô trống `n` — câu của Byte có 30000, câu của An có dấu ngoặc quanh `n + 2`; chép cứng con số hay chép hai chỗ thành một dòng chữ thì phép so hai bảng không so gì cả
  requireAst:
  # Ba chỗ trống, mỗi chỗ phải ĐỌC `n`. Khung khởi đầu đã đọc `n` bốn lần (một
  # lần trong chuỗi f của vòng đầu, hai lần ở `n + 6` và `3 * n`, một lần trong
  # chuỗi f của vòng sau); ba dòng `for n in ...` và `n = 20` là chỗ ĐẶT tên nên
  # không tính. Vậy điền bừa cho ra 4, còn lời giải cho 7.
  - kind: uses-name, target: n, min: 6
  # Mỗi câu tính phải có một phép nhân. Khung khởi đầu chỉ có đúng một (`3 * n`).
  - kind: uses-operator, target: *, min: 3
  # Câu của Byte cộng thêm tiền hai ổ, câu của An cộng 2 vào trong ngoặc — cả ba
  # chỗ trống đều có dấu cộng. Khung khởi đầu chỉ có đúng một (`n + 6`).
  - kind: uses-operator, target: +, min: 3
  # Hai con số này tách hai câu tính ra khỏi nhau. Thiếu 30000 nghĩa là câu của
  # Byte không được chép; thiếu 2 nghĩa là câu của An không được chép — và lúc
  # đó hai chỗ trống thành một dòng chữ, phép so hoá vô nghĩa. Khung khởi đầu
  # không chứa con số nào trong hai con số này.
  - kind: has-literal, target: 30000
  - kind: has-literal, target: 2
  forbidAst:
  # Lưới thứ hai, chặn hai con số KẾT QUẢ hay bị chép cứng: dòng n bằng 20 và
  # dòng n bằng 99. Mọi cách viết hợp lệ đều không chứa nguyên văn chúng.
  - kind: has-literal, target: 330000
  - kind: has-literal, target: 1515000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^330000\nhai khay trùng ở dòng 3\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một trăm dòng, không dòng nào lệch. Còn hai cái khay thì gặp nhau đúng một lần —
mình suýt tin nhầm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Máy vừa soi một trăm dòng và không tìm ra dòng lệch nào. Nhưng bảng thì không có
dòng cuối. Soi thêm một trăm dòng nữa cũng chỉ đẩy ranh giới ra xa thêm một chút
— **"mọi số" thì thử không bao giờ hết.**

Nhìn lại hai câu tính, chúng khác nhau đúng một chỗ:

```text
   An viết:     15000 × (n + 2)
   Byte viết:   15000 × n + 30000
```

An có một dấu ngoặc bao lấy cả cụm `n + 2`. Byte tháo cái ngoặc ấy ra: nhân
15000 vào `n` trước, rồi nhân 15000 vào 2 thành 30000, rồi cộng lại.

Câu hỏi là: **cái việc tháo ngoặc kia có phải một luật không?**

Nếu nó là luật, thì nó phải đúng với mọi con số điền vào, không cần thử dòng nào
cả — và cũng phải đúng khi đổi 15000 thành số khác, đổi 2 thành số khác. Nếu nó
chỉ là một câu chuyện về hai ổ bánh mì nhà ăn, thì lần sau gặp cặp câu tính
khác, bạn lại phải dựng bảng lên soi từ đầu.

Có luật nào cho phép tháo dấu ngoặc mà **chắc chắn** đúng với mọi số không — và
nếu có, làm sao nhìn thấy nó mà không phải thử?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
