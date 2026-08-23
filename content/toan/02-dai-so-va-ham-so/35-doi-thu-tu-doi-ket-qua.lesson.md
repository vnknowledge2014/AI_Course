---
id: toan.dai-so-va-ham-so.doi-thu-tu-doi-ket-qua
title: Đổi thứ tự nối, đổi luôn kết quả
summary: Cùng hai cái máy ấy, nối theo hai thứ tự thì ra hai cái máy khác nhau — thứ tự nối là một phần của cái máy, không phải chuyện phụ.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 35
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.composition-order]
requires: [math.function-composition, math.inverse-function, math.function-notation, math.expand-brackets, math.multiply-distributive, math.percent-whole, math.percent, math.multiply-commutative, math.parentheses, math.order-of-operations, core.function-def, core.function-call, core.function-parameter, core.function-return, core.variable, core.assignment, core.print-variable, core.arithmetic, core.float, core.division]
concepts: [math.thu-tu-noi-may, math.khong-doi-cho-duoc, math.hai-bien-giam-gia]
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
Hai tấm biển, một bác khách đang chờ. Nối tấm nào trước cũng được — nhưng có ra cùng một số tiền không?
::::

::::explain{#hai-tam-bien}
Bài trước, cái ống chỉ bắc được một chiều: máy `thu` nhả ra tiền, máy `lai` nhận
vào tiền, nên `lai(thu(n))` là cách nối duy nhất có nghĩa. Thứ tự không phải là
một lựa chọn — nó bị hai đầu ống ép cho.

Chiều nay thì khác. Quán treo hai tấm biển:

- **Máy `bot_phieu`** — bỏ **số tiền** vào, bớt 10 nghìn, nhả **số tiền** ra.
  `bot_phieu(t) = t − 10 000`.
- **Máy `giam20`** — bỏ **số tiền** vào, giảm 20%, nhả **số tiền** ra. Giảm 20%
  nghĩa là còn lại 80%, nên `giam20(t) = t × 80 : 100`.

Cả hai máy đều **nhận tiền và nhả tiền**. Hai đầu khớp nhau ở cả hai chiều, nên
lần này nối kiểu nào cũng bắc được ống — và Byte phải chọn.

```text
                       ┌──►  [ bớt phiếu ]  ──►  [ giảm 20% ]  ──►  152 000
   hoá đơn 200 000  ───┤
                       └──►  [ giảm 20% ]  ──►  [ bớt phiếu ]  ──►  150 000
```

Hai con số. Không bằng nhau.

Hoá đơn thì vẫn một, hai tấm biển thì vẫn hai, không tấm nào bị bỏ đi. Chỉ có
**thứ tự** đổi — và số tiền bác khách phải trả đổi theo.
::::

::::example{#lech-bao-nhieu}
Đi chậm lại từng cách, trên hoá đơn 200 nghìn:

- **Cách A — bớt phiếu trước.** 200 000 bớt 10 000 còn 190 000. Giảm 20% của
  190 000, còn lại 80%: `190 000 × 80 : 100 = 152 000`.
- **Cách B — giảm 20% trước.** 200 000 còn lại 80% là 160 000. Rồi bớt phiếu:
  `160 000 − 10 000 = 150 000`.

Bác khách trả cách B thì rẻ hơn 2 000 đồng.

Bây giờ mới là chỗ đáng hỏi: **2 000 ấy ở đâu ra, và nó có đổi theo hoá đơn
không?** Thử thêm mấy hoá đơn nữa:

```text
  hoá đơn │  A: phiếu trước │  B: 20% trước │  chênh
──────────┼─────────────────┼───────────────┼───────
   100000 │           72000 │         70000 │   2000
   200000 │          152000 │        150000 │   2000
   250000 │          192000 │        190000 │   2000
   500000 │          392000 │        390000 │   2000
```

Cột chênh không nhúc nhích. Hoá đơn to gấp năm lần mà khoảng lệch vẫn đúng
2 000 đồng.

Bảng thì thấy, nhưng bảng chỉ thử được mấy hoá đơn. Muốn chắc với **mọi** hoá
đơn thì phải mở dấu ngoặc ra — đúng cái luật phân phối của bài 7:

```text
Cách A:  (t − 10 000) × 80 : 100  =  t × 80 : 100  −  8 000
Cách B:   t × 80 : 100 − 10 000
```

Hai câu tính ấy có **chung hệt** phần `t × 80 : 100`. Khác nhau đúng ở cái đuôi:
cách A bớt đi 8 000, cách B bớt đi 10 000. Nên cách B luôn rẻ hơn đúng
`10 000 − 8 000 = 2 000` đồng, ở mọi hoá đơn, kể cả hoá đơn chưa ai viết ra.

Và 8 000 ấy chính là **80% của tấm phiếu**: đưa phiếu vào trước thì bản thân
tấm phiếu cũng bị giảm 20% theo. Mất 2 000 của tấm phiếu — đúng bằng khoảng
lệch.

Hỏi thẳng cái máy:

```python title=readonly
def bot_phieu(t):
    return t - 10000

def giam20(t):
    return t * 80 / 100

# Cách A: bớt phiếu trước, rồi mới giảm 20%.
print(giam20(bot_phieu(200000)))

# Cách B: giảm 20% trước, rồi mới bớt phiếu.
print(bot_phieu(giam20(200000)))
```

Máy in ra:

```text
152000.0
150000.0
```

Cái đuôi `.0` là dấu vết của phép chia cho 100, y như Realm 0 đã dựng — nó
không nói rằng có nửa đồng bạc nào.
::::

::::predict{#doan-hai-cach commitOnce}
Bác khách sau đưa hoá đơn 300 nghìn, vẫn hai tấm biển ấy.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def bot_phieu(t):
    return t - 10000

def giam20(t):
    return t * 80 / 100

hoa_don = 300000
print(giam20(bot_phieu(hoa_don)))
print(bot_phieu(giam20(hoa_don)))
```

:::opt{correct}
232000.0 rồi 230000.0
:::

:::opt
232000.0 rồi 232000.0
::why
Gần đúng ở chỗ bạn đang dùng một luật rất thật và rất mạnh: **đổi chỗ thì kết
quả không đổi**. Với hai con số dưới một phép cộng hay một phép nhân thì luật ấy
đúng tuyệt đối, mạch trước đã dựng nó ở bài 20 bằng cái mảng chữ nhật xoay một
góc — `3 × 5` và `5 × 3` là cùng một mảng nhìn từ hai phía.

Ranh giới nằm ở chỗ **cái gì đang đổi chỗ**. Luật ấy nói về hai **con số** đứng
hai bên **một** phép. Ở đây đổi chỗ là hai **cái máy** nối đuôi nhau, mà hai máy
này làm hai việc khác loại: một cái **bớt đi một lượng cố định**, một cái **thu
nhỏ cả con số lại**. Thu nhỏ sau khi bớt thì tấm phiếu bị thu nhỏ theo; bớt sau
khi thu nhỏ thì nó không bị. Cái mảng chữ nhật không kể được câu chuyện đó, nên
nó không phủ tới đây.
::
:::

:::opt
230000.0 rồi 232000.0
::why
Gần đúng ở hai chỗ, và cả hai đều là chỗ khó: bạn thấy hai kết quả **phải khác
nhau**, và bạn tính ra đúng hai con số ấy. Chỉ còn việc ghép mỗi con số vào đúng
dòng của nó.

Ranh giới: bên trong một dòng, máy chạy theo thứ tự **trong ra ngoài** chứ không
theo thứ tự chữ viết. Dòng thứ nhất là `giam20(bot_phieu(...))` — `bot_phieu`
nằm trong cùng nên nó chạy trước, tức là cách A, bớt phiếu trước, ra 232 000.
Chữ `giam20` viết trước nhưng chạy sau.
::
:::

:::opt
232000 rồi 230000
::why
Gần đúng ở chỗ bạn giữ đúng bản chất của thứ đang đếm: tiền Việt không có nửa
đồng, và một hoá đơn thì luôn là số nguyên đồng. Linh cảm ấy đúng, và nó đã cứu
bạn ở những bài đếm hạt.

Ranh giới: cái đuôi `.0` không nói rằng có nửa đồng. Nó chỉ nói con số này vừa
đi qua một **phép chia** — ở đây là chia cho 100 khi tính 80%. Máy nhớ điều đó
và giữ phần lẻ lại, kể cả khi phần lẻ bằng không. Và một khi phần lẻ đã vào
trong dòng, nó ở lại: `160000.0` bớt `10000` vẫn ra `150000.0`.
::
:::
::::

::::explain{#thu-tu-la-mot-phan-cua-may}
Đặt tên cho thứ vừa thấy:

> Hợp hai máy **không đổi chỗ được**. Nói chung, `f(g(n))` khác `g(f(n))`.
> **Thứ tự nối là một phần của cái máy**, chứ không phải một chi tiết bên ngoài
> có thể bỏ qua.

"Nói chung" ở đây là một chữ được cân đo. Nó không có nghĩa "luôn luôn khác" —
nó có nghĩa: **không được phép giả định là giống, trừ khi bạn kiểm được**.

Và bạn đã biết đúng **một** cặp kiểm được rồi: cặp máy xuôi và máy ngược của bài
32. Chạy máy xuôi rồi máy ngược, hay máy ngược rồi máy xuôi, đều trả về đúng con
số đã bỏ vào. Đó là chuyện cũ, gọi lại để so — chứ không phải điều mới ở bài
này. Nó là ngoại lệ, và nó là ngoại lệ **vì hai máy ấy được dựng ra để triệt
nhau**, chứ không phải vì đổi chỗ nói chung thì được.

Chỗ này còn trả lời một câu hỏi thực tế của quán: **treo biển theo thứ tự nào là
có lợi cho khách?** Cách B — giảm phần trăm trước, bớt phiếu sau. Không phải vì
ai tử tế hơn ai, mà vì đưa tấm phiếu vào trước thì chính tấm phiếu cũng bị giảm
20% theo. Một câu chuyện có thật ngoài chợ, và nó nằm gọn trong một dòng ký hiệu.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Hoá đơn 250 nghìn. Hai cách nối, rồi so xem lệch bao nhiêu.
::::

::::code{#hai-cach-noi}
Bác khách tiếp theo đưa hoá đơn **250 nghìn** và một tấm phiếu.

Ba chỗ trống: một cho **cách A**, một cho **cách B**, một cho **khoảng lệch**.

Ba con số đi ra phải khác nhau, nên gõ cứng một con số thì hai chỗ kia sai ngay.
Và chỗ trống thứ ba phải dựng từ **hai cái tên phía trên nó**, không phải từ con
số 2 000 mà bạn đã đọc được ở bảng — bảng ấy chỉ thử bốn hoá đơn, còn dòng bạn
viết thì phải đúng cả với hoá đơn của ngày mai.

```python title=starter
def bot_phieu(t):
    return t - 10000

def giam20(t):
    return t * 80 / 100

hoa_don = 250000

# Cách A: bớt phiếu TRƯỚC, rồi mới giảm 20%.
cach_a = ___
# Cách B: giảm 20% TRƯỚC, rồi mới bớt phiếu.
cach_b = ___
# Vào xe của Byte (cách B) thì bác khách đỡ được bao nhiêu?
khach_do = ___

print(cach_a)
print(cach_b)
print(khach_do)
```

```python title=solution
def bot_phieu(t):
    return t - 10000

def giam20(t):
    return t * 80 / 100

hoa_don = 250000

# Cách A: bớt phiếu TRƯỚC, rồi mới giảm 20%.
cach_a = giam20(bot_phieu(hoa_don))
# Cách B: giảm 20% TRƯỚC, rồi mới bớt phiếu.
cach_b = bot_phieu(giam20(hoa_don))
# Vào xe của Byte (cách B) thì bác khách đỡ được bao nhiêu?
khach_do = cach_a - cach_b

print(cach_a)
print(cach_b)
print(khach_do)
```

```python title=test
# Câu `!=` này canh đúng cái bẫy của bài — nối cùng một thứ tự hai lần — và nó
# đứng TRƯỚC vì chương trình dừng ở câu vỡ đầu tiên; xếp sau các câu `==` thì
# nó không bao giờ chạy tới.
assert cach_a != cach_b, "hai cách nối phải cho hai số khác nhau; bằng nhau nghĩa là bạn đã nối cùng một thứ tự cả hai lần"
assert cach_a != 200000, "200 000 là 80% của hoá đơn, chưa có tấm phiếu nào trong đó"
# Ba con số khác nhau, nên một con số gõ cứng chỉ qua được nhiều nhất một câu.
assert cach_a == 192000, "bớt phiếu trước còn 240 000, rồi giảm 20% còn 192 000"
assert cach_b == 190000, "giảm 20% trước còn 200 000, rồi bớt phiếu còn 190 000"
assert khach_do == 2000, "cách B rẻ hơn cách A đúng 2 000 đồng"
# Vì sao đúng 2 000: đưa phiếu vào trước thì chính tấm phiếu cũng bị giảm 20%.
assert khach_do == 10000 * 20 / 100, "khoảng lệch đúng bằng 20% của tấm phiếu 10 000 — phần tấm phiếu bị giảm theo"
```

:::hints
- kind: attention
  body: Đọc lại hai dòng `def` ở đầu khung, rồi đọc hai dòng chú thích ngay trên hai chỗ trống đầu. Mỗi chú thích nói rõ máy nào chạy TRƯỚC. Chỗ trống thứ ba không nhắc tới máy nào cả — nó chỉ nhắc tới hai dòng vừa viết.
- kind: strategy
  body: Máy chạy trước là máy nhận thẳng hoá đơn, nên tên nó nằm BÊN TRONG cặp ngoặc của máy chạy sau. Hai chỗ trống đầu dùng đúng hai cái tên máy ấy, chỉ khác nhau chỗ tên nào nằm trong. Chỗ trống thứ ba là hiệu của hai cái tên vừa đặt — cách A trừ đi cách B, vì cách A đắt hơn.
- kind: one-line
  body: "Ba chỗ trống là `giam20(bot_phieu(hoa_don))`, `bot_phieu(giam20(hoa_don))` và `cach_a - cach_b`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống đầu phải nối HAI máy theo HAI thứ tự khác nhau, và chỗ trống thứ ba phải dựng từ hai cái tên vừa đặt — gõ thẳng con số kết quả thì dòng ấy chỉ đúng với đúng một hoá đơn
  requireAst:
  # Khung khởi đầu không GỌI máy nào (hai dòng `def` là định nghĩa, không phải
  # lệnh gọi) và không ĐỌC `hoa_don` lần nào, nên ba luật này chặn được cả đáp
  # án điền bừa lẫn đáp án chỉ nối một thứ tự rồi chép sang chỗ kia.
  - kind: uses-call, target: bot_phieu, min: 2
  - kind: uses-call, target: giam20, min: 2
  - kind: uses-name, target: hoa_don, min: 2
  # Khung đã đọc `cach_a` và `cach_b` mỗi tên một lần ở `print`, nên `min: 2`
  # mới là thứ buộc chỗ trống thứ ba phải dựng từ hai cái tên ấy.
  - kind: uses-name, target: cach_a, min: 2
  - kind: uses-name, target: cach_b, min: 2
  forbidAst:
  # Lưới thứ hai, chặn đúng ba con số KẾT QUẢ, cả dạng nguyên lẫn dạng có phần
  # lẻ. Lời giải thật chỉ chứa 10000, 80, 100 và 250000 nên không vướng.
  - kind: has-literal, target: 192000
  - kind: has-literal, target: 192000.0
  - kind: has-literal, target: 190000
  - kind: has-literal, target: 190000.0
  - kind: has-literal, target: 2000
  - kind: has-literal, target: 2000.0
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^192000\.0\n190000\.0\n2000\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai nghìn đồng, và nó không nằm ở tấm biển nào cả — nó nằm ở thứ tự treo biển.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang bài cuối.

Nhìn lại cả mạch này, từ bài 1 tới đây, chỉ có đúng **một** ý chạy suốt: chữ cái
là một **ô trống**, và một cái máy là một **quy tắc điền ô ấy**.

- Một biểu thức là ô trống chưa được điền.
- Một phương trình là câu hỏi *"điền gì thì câu này đúng"*.
- Một bảng là mọi lần điền, xếp thành hàng.
- Một đồ thị là cái bảng ấy chấm lên mặt phẳng.
- Một hàm số là quy tắc: điền vào một chỗ, đúng một chỗ khác hiện ra.
- Và hai bài vừa rồi: hai quy tắc nối đuôi nhau vẫn là **một** quy tắc.

Byte có trong tay đủ đồ nghề: câu tính còn ô trống, cái cân hai đĩa, trục số tô
khoảng, mặt phẳng toạ độ, đường thẳng, đường cong có đỉnh, đường nhân đôi, máy
chạy ngược, và cái ống nối máy.

Ngày mai xe bánh mì mở hàng. Byte cần biết: bán mấy ổ thì hết lỗ, bán mấy ổ thì
lãi hai trăm nghìn, tăng giá bao nhiêu thì thu được nhiều nhất, thúng men sáng
mai nở tới đâu.

Bốn câu hỏi rất đời, hỏi bằng tiếng Việt. Ghép hết chỗ đồ nghề kia lại, có tả
trọn được **một buổi bán hàng** không?
::::

::::checkpoint{mastery=0.8}
::::
