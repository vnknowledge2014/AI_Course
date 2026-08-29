---
id: toan.dai-so-va-ham-so.chu-o-ca-hai-dia
title: Chữ đứng ở cả hai đĩa
summary: Một cụm mang chữ cũng là một lượng — nên bớt nó ở cả hai đĩa vẫn giữ nguyên thăng bằng, và đó là cách dồn hết chữ về một bên.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.variable-both-sides]
requires: [math.check-solution, math.equation-add-both-sides, math.equation-multiply-both-sides, math.letter-names-a-slot, math.substitution, math.factor-common, math.equation, math.multiplication, math.order-of-operations, math.like-units, core.variable, core.assignment, core.arithmetic, core.number-literal, core.print-variable, core.output]
concepts: [math.cai-can, math.giu-nghiem, math.hang-tu-dong-dang]
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
Lần này cả hai đĩa đều có chữ n. Gỡ bên nào trước bây giờ?
::::

::::explain{#hai-dia-deu-co-chu}
Bài trước để lại một câu hỏi: nếu tiền vốn cũng đi theo số ổ thì đĩa phải cũng
mang chữ. Viết ra thì thấy ngay.

Xe bánh mì của Byte, đếm bằng đồng:

- Bán một ổ được **15 000**. Bán `n` ổ thì thu `15000 × n`.
- Mỗi ổ tốn **9 000** tiền bột, tiền thịt, tiền than. Bán `n` ổ thì tốn
  `9000 × n`. Cộng thêm **30 000** thuê chỗ, không đổi dù bán mấy ổ.

Câu hỏi của quán — *bán mấy ổ thì tiền thu vừa đủ bù tiền chi?* — viết thành:

```text
   15000 × n  =  9000 × n + 30000
```

Đây là câu đầu tiên trong mạch có **ô trống ở cả hai đĩa**. Mọi cách gỡ tới giờ
đều bắt đầu bằng việc dọn mấy con số đứng cạnh chữ, để cuối cùng chữ còn đứng
một mình. Nhưng lần này dọn xong đĩa trái thì đĩa phải vẫn còn chữ, dọn xong
đĩa phải thì đĩa trái vẫn còn chữ. Đuổi vòng quanh.

Chỗ mắc không nằm ở kỹ thuật. Nó nằm ở một câu hỏi thật:

> Luật hai đĩa nói *bớt cùng một **lượng** ở cả hai đĩa*. Cụm `9000 × n` có
> được tính là một lượng không, khi chưa ai biết nó bằng bao nhiêu?
::::

::::example{#mot-luong-chua-biet-van-la-mot-luong}
Nghĩ về cái cân thật một lúc.

Byte có một gói hàng dán kín, không mở ra được, không biết nặng bao nhiêu. Byte
đặt **một gói y hệt** lên mỗi đĩa. Cân đang thăng bằng thì sau khi đặt xong nó
vẫn thăng bằng — hai đĩa vừa nặng thêm đúng bằng nhau.

Bây giờ nhấc **cả hai gói** ra. Cân có lệch không?

Không. Và bạn kết luận được điều đó **mà không cần biết gói ấy nặng bao nhiêu**.
Cái giữ cho cân thăng bằng không phải con số cân nặng, mà là chuyện hai đĩa bị
đối xử **giống hệt nhau**.

Cụm `9000 × n` đúng là một gói như thế. Nó chưa ra số — nhưng chữ `n` ở đĩa
trái và chữ `n` ở đĩa phải là **cùng một ô trống**, nên hễ điền vào thì cả hai
chỗ nhận cùng một con số, và `9000 × n` ở hai bên luôn là **cùng một lượng**.
Chưa biết bằng bao nhiêu, mà vẫn chắc chắn bằng nhau.

Nên bớt nó đi ở cả hai đĩa là hợp luật:

```text
                           đĩa trái     đĩa phải
   trước khi bớt          15000 × n  =  9000 × n + 30000
   bớt ở hai đĩa         − 9000 × n  =  − 9000 × n
   ─────────────────────────────────────────────────────
   sau khi bớt             6000 × n  =             30000
```

Hai chỗ đáng dừng lại:

- **Đĩa trái**: `15000 × n − 9000 × n` gộp lại thành `6000 × n`. Đó là luật rút
  cái chung ra ngoài đọc theo chiều quen thuộc — cả hai cụm đều đếm cùng một
  thứ là `n`, nên gộp được.
- **Đĩa phải**: `9000 × n + 30000 − 9000 × n` chỉ còn `30000`. Chữ biến khỏi
  đĩa phải, và đó chính là thứ ta muốn.

Từ `6000 × n = 30000`, phần còn lại là chuyện cũ: chia cả hai đĩa cho 6000, ra
`n = 5`.

Kiểm nghiệm ở **câu gốc**, không phải ở dòng vừa viết: thu là
`15000 × 5 = 75000`, chi là `9000 × 5 + 30000 = 45000 + 30000 = 75000`. Hai vế
cùng ra 75 000. Bán 5 ổ thì huề vốn.
::::

::::predict{#doan-hai-dia commitOnce}
Byte muốn nhìn hai đĩa của **câu gốc** bằng con số vừa gỡ ra, `n = 5`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 5
print(15000 * n)          # đĩa trái: tiền thu khi bán n ổ
print(9000 * n + 30000)   # đĩa phải: tiền vốn n ổ, cộng 30 000 thuê chỗ
```

:::opt{correct}
75000 rồi 75000
:::

:::opt
75000 rồi 45000
::why
Gần đúng ở chỗ bạn tính `9000 × 5` không sai một li, và bạn đọc trúng phần khó
nhất của đĩa phải — phần tiền vốn đi theo số ổ.

Chỗ lệch: đĩa phải còn một cục **30 000** nữa. Nó không đi theo số ổ nên trông
như thể không thuộc về câu tính, nhưng tiền thuê chỗ vẫn phải trả dù bán được
mấy ổ. Ranh giới nằm ở đó: một đĩa được phép chứa cả phần đi theo ô trống lẫn
phần đứng yên, và bỏ phần đứng yên đi là bỏ mất chính lý do câu này có nghiệm.
::
:::

:::opt
75000 rồi 39000
::why
Gần đúng ở chỗ bạn cộng đúng hai khoản của đĩa phải và không bỏ sót khoản nào —
`9000 + 30000` đúng bằng 39 000.

Chỗ lệch: chỉ một trong hai khoản ấy đi theo số ổ. Tiền vốn phải nhân với `n`
trước rồi mới cộng, còn tiền thuê chỗ thì không nhân với gì cả. Ranh giới đúng
là dấu nhân: chỗ nào có `× n` thì lớn lên theo số ổ, chỗ nào không có thì đứng
yên — và trong `9000 × n + 30000` chỉ có một chỗ mang dấu ấy.
::
:::

:::opt
Máy báo lỗi, vì `n` chưa phải một con số
::why
Gần đúng ở chỗ bạn đang giữ đúng ý cốt lõi của mạch này: `15000 × n` là một câu
tính **đang chờ**, không phải một con số, chừng nào ô trống chưa được điền.
Trong câu `15000 × n = 9000 × n + 30000` thì đúng là chưa có số nào cả.

Chỗ lệch là dòng `n = 5` đứng ngay trên. Nó chính là lần điền ấy. Từ chỗ đó trở
xuống, `n` không còn là ô trống nữa mà là một cái tên đang giữ con số 5, nên
hai dòng dưới tính ra số bình thường. Ranh giới: ô trống nằm ở **câu tính**,
còn ở đây bạn đang xem một **lần thử** của câu tính ấy.
::
:::
::::

::::explain{#dat-ten}
Đặt tên cho thứ vừa làm:

> Khi chữ đứng ở cả hai đĩa, **bớt cùng một cụm mang chữ ở hai đĩa** để dồn hết
> chữ về một bên. Cụm ấy cũng là một lượng, dù chưa biết nó bằng bao nhiêu.

Ba điều đi kèm:

- **Chọn bên nào cũng được.** Bớt `9000 × n` thì chữ dồn về trái, ra
  `6000 × n = 30000`. Bớt `15000 × n` thì chữ dồn về phải, ra
  `0 = −6000 × n + 30000`. Hai đường đều đúng và cùng ra `n = 5` — chỉ là đường
  thứ hai bắt bạn làm việc với số âm. Bớt cụm **nhỏ hơn** thì phần còn lại
  dương, đỡ mệt hơn.
- **Bớt cả cụm, không bớt nửa cụm.** `9000 × n` đi thì cả `9000` lẫn `n` cùng
  đi. Bớt riêng số 9000, hay bớt riêng chữ `n`, không phải phép nào cả.
- **Chỉ gộp được cái cùng loại.** `9000 × n` gặp `15000 × n` nên gộp được. Nếu
  đĩa trái chỉ có `15000` trần thì bớt xong nó thành `15000 − 9000 × n`, chẳng
  gộp thêm gì — và đó ĐÃ là dạng cuối của nó, đúng như `8n + 6` ở bài 9: hai cụm
  khác loại thì dừng, mà dừng không phải là chưa xong.

Và chỗ đáng nhớ nhất: bạn vừa dùng luật hai đĩa lên một thứ **chưa biết giá
trị**. Nếu chữ là "một con số bị giấu" thì việc này đáng ngờ — làm sao nhấc đi
một thứ mình không biết nặng bao nhiêu? Nhưng chữ là một **ô trống**, và hai ô
trống mang cùng một chữ luôn nhận cùng một con số. Đó là toàn bộ lý do phép bớt
này hợp lệ.
::::

::::code{#bot-cum-mang-chu}
Byte muốn xem tận mắt: bớt cụm `9000 × n` ở hai đĩa thì cân có còn thăng bằng
không, tại con số vừa gỡ ra là `n = 5`.

Ba chỗ trống: hai đĩa của **câu gốc**, rồi **cụm mang chữ** được bớt đi. Cụm ấy
viết đúng một lần và dùng cho cả hai đĩa — đó chính là chỗ "cùng một lượng"
hiện ra thành mã.

Ba chỗ ấy cho ra `75000`, `75000` rồi `45000`. Hai số đầu bằng nhau **đúng như
câu gốc hứa** — ở `n = 5` thì hai đĩa cân nhau, đó là cả điểm của bài. Số thứ ba
khác, nên gõ cứng một con số vào cả ba thì hỏng ngay ở dòng in thứ ba.

```python title=starter
n = 5   # con số vừa gỡ ra được, đang đem đi thử

# Hai đĩa của CÂU GỐC:   15000 × n  =  9000 × n + 30000

thu = ___    # đĩa trái: tiền thu khi bán n ổ
chi = ___    # đĩa phải: tiền vốn n ổ, cộng 30 000 thuê chỗ

bot = ___    # cụm MANG CHỮ được bớt đi ở CẢ HAI đĩa

trai_sau = thu - bot
phai_sau = chi - bot

print(thu)
print(chi)
print(bot)
print(trai_sau)
print(phai_sau)
```

```python title=solution
n = 5   # con số vừa gỡ ra được, đang đem đi thử

# Hai đĩa của CÂU GỐC:   15000 × n  =  9000 × n + 30000

thu = 15000 * n    # đĩa trái: tiền thu khi bán n ổ
chi = 9000 * n + 30000    # đĩa phải: tiền vốn n ổ, cộng 30 000 thuê chỗ

bot = 9000 * n    # cụm MANG CHỮ được bớt đi ở CẢ HAI đĩa

trai_sau = thu - bot
phai_sau = chi - bot

print(thu)
print(chi)
print(bot)
print(trai_sau)
print(phai_sau)
```

```python title=test
# Ba câu `!=` đứng trước: chúng canh những chỗ mà một đáp án điền bừa hay một
# con số gõ cứng sẽ làm sập. Xếp chúng sau các câu `==` thì chúng không bao giờ
# chạy tới.
assert thu != bot, "cụm bị bớt chỉ là MỘT PHẦN của đĩa trái, không phải cả đĩa — 15000 × n khác 9000 × n"
assert trai_sau != thu, "bớt thật thì đĩa trái phải nhẹ đi; còn nguyên nghĩa là chưa bớt gì"
assert phai_sau != chi, "đĩa phải cũng phải nhẹ đi đúng chừng ấy, nếu không thì hai đĩa bị đối xử khác nhau"
assert trai_sau == phai_sau, "bớt CÙNG MỘT LƯỢNG ở hai đĩa thì cân vẫn thăng bằng — đây là điều bài này nói"
assert thu == chi, "n = 5 làm hai vế câu gốc ra cùng một số, nên 5 là nghiệm — đúng phép kiểm nghiệm của bài 16"
assert thu == 75000, "15000 × 5 = 75000 đồng tiền thu"
assert bot == 45000, "9000 × 5 = 45000 đồng tiền vốn cho 5 ổ — cụm mang chữ, không gồm 30 000 thuê chỗ"
assert trai_sau == 30000, "sau khi bớt, đĩa trái còn 6000 × n tức 30000, và đĩa phải còn đúng 30 000 thuê chỗ"
```

:::hints
- kind: attention
  body: Dòng chú thích `15000 × n  =  9000 × n + 30000` đã viết sẵn cả hai đĩa. Chỗ trống thứ nhất là vế bên trái dấu `=`, chỗ thứ hai là vế bên phải. Chỗ thứ ba không phải cả một đĩa — nó là cụm xuất hiện ở **cả hai** đĩa.
- kind: strategy
  body: Chép lại từng vế, chỗ nào có chữ `n` thì đặt tên `n` vào đúng chỗ đó — đừng tính nhẩm rồi gõ con số, vì con số ấy chính là thứ đang nhờ máy tìm hộ. Cho chỗ trống thứ ba, đọc lại đĩa phải và tìm phần đi theo số ổ; khoản 30 000 đứng yên không thuộc về nó.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `15000 * n`, `9000 * n + 30000` và `9000 * n`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả ba chỗ trống phải viết bằng cái tên `n`, không phải bằng con số đã tính sẵn — tự tính hộ máy thì cụm mang chữ biến mất và bài này không còn gì để chỉ
  requireAst:
  # Ba chỗ trống, ba phép nhân với `n`. Khung khởi đầu không có dấu `*` nào.
  - kind: uses-operator, target: *, min: 3
  # Cụm mang chữ phải được ĐỌC từ cái tên `n`, ba lần — một lần cho mỗi chỗ
  # trống. Gõ `15000 * 5` cũng ra 75000, nhưng lúc đó ô trống đã bị thay tay.
  - kind: uses-name, target: n, min: 3
  # Đĩa phải có hai mảnh: phần đi theo số ổ và 30 000 đứng yên. Thiếu dấu cộng
  # nghĩa là một trong hai mảnh bị bỏ.
  - kind: uses-operator, target: +, min: 1
  # Số 15 000 chỉ được xuất hiện ĐÚNG MỘT lần — ở đĩa trái.
  #
  # Không có trần này thì `bot = 15000 * n - 30000` qua sạch bốn tầng: ở `n = 5`
  # nó ra đúng 45 000, bằng hệt `9000 * n`. Mà đó lại là đúng cái sai bài này
  # cảnh báo — `bot` phải là cụm CÓ MẶT Ở CẢ HAI ĐĨA, không phải đĩa trái trừ
  # tiền thuê. Hai bên bằng nhau ở n = 5 vì n = 5 chính là nghiệm, nên không
  # cách chấm nào dựa vào giá trị phân biệt nổi; chỉ đếm số 15 000 mới thấy.
  - kind: has-literal, target: 15000, max: 1
  forbidAst:
  # Lưới thứ hai: hai con số KẾT QUẢ. Lời giải thật dựng chúng từ `n` nên không
  # chứa nguyên văn cái nào; đáp án chép cứng thì chứa.
  - kind: has-literal, target: 75000
  - kind: has-literal, target: 45000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^75000\n75000\n45000\n30000\n30000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhấc cùng một gói khỏi hai đĩa. Cân vẫn thăng bằng, mà chữ đã dồn về một bên.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bán 5 ổ là huề vốn. Có người bảo Byte: *"Hạ giá đi cho đông khách."* Byte thử
hạ hẳn xuống **9 000 đồng một ổ** — đúng bằng tiền bột, thịt và than cho một ổ.

Viết lại câu hỏi cũ với giá mới. Tiền thu thành `9000 × n`, tiền chi vẫn là
`9000 × n + 30000`:

```text
   9000 × n  =  9000 × n + 30000
```

Giờ làm đúng việc vừa học: bớt cụm `9000 × n` ở cả hai đĩa. Đĩa trái còn `0`.
Đĩa phải còn `30000`. Câu ấy thành:

```text
   0 = 30000
```

Chữ biến mất sạch. Không còn ô trống nào để điền nữa, mà cũng chưa ai điền gì
cả. Chuỗi biến đổi thì hợp luật từ đầu tới cuối — mỗi bước đều là một phép giữ
nghiệm.

Vậy câu `9000 × n = 9000 × n + 30000` có nghiệm bằng bao nhiêu? Và nếu chữ thật
sự là "một con số bị giấu", thì con số bị giấu ấy vừa đi đâu mất?

Bài sau nhận đúng câu hỏi này.
::::

::::checkpoint{mastery=0.8}
::::
