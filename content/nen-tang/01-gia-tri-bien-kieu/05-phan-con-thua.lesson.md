---
id: nen-tang.gia-tri-bien-kieu.phan-con-thua
title: Phần còn thừa
summary: Dấu `%` đưa thẳng ra phần dư của một phép chia — nửa còn lại mà `//` bỏ lại phía sau.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.modulo]
requires: [core.floor-division, core.division, core.arithmetic, core.variable, core.fstring]
concepts: [core.so, core.phep-tinh]
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
Phần mà phép chia bỏ lại, mình vẫn giữ. Chỉ cần bạn hỏi bằng đúng dấu.
::::

::::explain{#nua-con-lai-cua-phep-chia}
Bài trước dừng lại ở một đồng lơ lửng: hoá đơn `100000`, ba người, mỗi người
`33333`, cộng lại `99999`. Bạn đã có cách hỏi phần thừa — `100000 - 33333 * 3` —
nhưng cách ấy bắt bạn cầm lại kết quả của phép chia trước rồi mới hỏi được, và
đổi số người thì phải sửa con số ở hai chỗ.

Ngoài đời không ai làm hai bước như thế. Bà chia mười cái kẹo cho ba đứa cháu và
nói ngay một câu: *mỗi đứa ba cái, còn thừa một cái trong túi*. Hai con số ấy đi
liền nhau, và con số thứ hai không phải tính lại từ con số thứ nhất — bà nhìn cái
kẹo còn trong túi là biết.

Trong toán, hai con số ấy có hai cái tên: **thương** và **số dư**. `//` của bài
trước đưa bạn cái thương. Python có một dấu riêng cho cái còn lại: `%` — **chia
lấy dư** (tiếng Anh là *modulo*).

Một lời dặn ngay từ đầu, vì cái phím này quen mắt theo một nghĩa khác: `%` trong
Python **không phải phần trăm**. Python không có dấu phần trăm. Muốn tính sáu
phần trăm của hai trăm năm mươi nghìn thì bạn vẫn viết `250000 * 6 / 100` như
mọi khi. Còn `250000 % 6` hỏi một câu hoàn toàn khác: *chia cho sáu thì còn thừa
lại bao nhiêu*.
::::

::::example{#thuong-va-so-du}
Cùng một cặp số, hỏi hai câu:

```python title=readonly
hoa_don = 100000

print(hoa_don // 3)
print(hoa_don % 3)
print(hoa_don // 3 * 3 + hoa_don % 3)
```

Máy in ra:

```text
33333
1
100000
```

Dòng đầu là phần mỗi người: `33333`. Dòng thứ hai là đúng cái đồng lơ lửng của
bài trước: `1`. Bạn không phải nhân, không phải trừ, không phải nhớ lại con số
nào — chỉ đổi hai gạch chéo thành một dấu `%` trên cùng cặp số ấy.

Dòng thứ ba mới là chỗ đáng nhớ nhất. Nó lắp hai mảnh lại: lấy phần mỗi người
nhân với số người, rồi cộng phần thừa vào — và ra lại **đúng** hoá đơn ban đầu.

Đó không phải may mắn của riêng con số này. Với hai số nguyên bất kỳ, `//` và
`%` luôn khớp nhau như vậy: phần chia trọn cộng phần còn thừa thì bằng con số
lúc đầu, không rơi rớt một đơn vị nào. Nên `%` không phải một mẹo vặt đứng riêng
— nó là nửa còn lại của chính phép chia mà bạn vừa học.
::::

::::predict{#doan-phan-du commitOnce}
Vẫn khoản lẩu `250000` chia cho sáu người của bài trước, lần này hỏi phần thừa.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra con số nào?

```python
tien_lau = 250000
so_nguoi = 6
print(tien_lau % so_nguoi)
```

:::opt{correct}
4
:::

:::opt
41666
::why
Gần đúng ở chỗ bạn nhận ra dòng này vẫn là một phép chia giữa `250000` và `6`,
và `41666` đúng là kết quả của phép chia ấy — bạn không tính sai một con số nào,
bài trước vừa cho ra chính nó.

Chỗ lệch nằm ở chỗ hai dấu lấy về hai mảnh khác nhau của cùng một phép chia.
`//` lấy phần chia trọn được — mỗi người bao nhiêu. `%` lấy phần còn lại sau khi
đã chia xong — mảnh không đủ để chia thêm cho ai nữa.
::
:::

:::opt
0
::why
Gần đúng ở chỗ bạn để ý cả `250000` lẫn `6` đều là số chẵn, nên đoán chúng chia
hết cho nhau. Suy luận "chia hết thì dư 0" thì đúng luật, và cách bạn tìm dấu
hiệu chia hết cũng là cách người ta vẫn làm.

Chỗ lệch: cùng chẵn chỉ bảo đảm chia hết cho `2`, mà `6` là `2` nhân `3`.
`250000` chia `2` được `125000`, nhưng `125000` thì không chia hết cho `3` — nó
còn dư `2`. Nhân ngược lại với `2`, phần thừa của phép chia cho `6` là `4`.
::
:::

:::opt
15000.0
::why
Gần đúng ở chỗ bạn đọc dấu `%` đúng như nó vẫn được đọc ngoài đời — phần trăm —
rồi tính `6%` của `250000` ra `15000`. Phép tính ấy bạn làm không sai, và đây là
cách hiểu tự nhiên nhất khi mới thấy cái phím này trong code.

Chỗ lệch: Python mượn phím `%` cho một việc khác hẳn, và nó không có dấu nào
mang nghĩa phần trăm cả. Muốn sáu phần trăm thì viết `250000 * 6 / 100`. Còn
`%` đặt giữa hai số nguyên thì luôn hỏi về phần dư — và phần dư của một phép
chia cho `6` thì không bao giờ lớn hơn `5`.
::
:::
::::

::::explain{#hai-dau-di-thanh-cap}
Từ giờ hãy nhớ `//` và `%` như một cặp đi liền: cùng một cặp số, một dấu lấy
phần chia trọn, một dấu lấy phần còn thừa.

Cặp ấy dùng được ở nhiều chỗ ngoài chia tiền, và chỗ nào cũng theo đúng cái
khuôn "mấy phần trọn vẹn, còn dư bao nhiêu":

- Đổi `145` phút sang giờ: `145 // 60` cho `2` giờ, `145 % 60` cho `25` phút.
- Hỏi một số có chẵn không: `n % 2` bằng `0` thì chẵn, bằng `1` thì lẻ.
- Đổi `100000` đồng ra tờ hai chục nghìn: `100000 // 20000` được `5` tờ,
  `100000 % 20000` còn `0` đồng lẻ.

Cả ba đều là một câu hỏi duy nhất, hỏi trên ba đơn vị khác nhau.
::::

::::code{#dong-le-cua-hai-khoan}
Vẫn hai khoản ăn chung của bài trước, và lần này Byte muốn dòng sổ nói đủ cả
hai vế: mỗi người đưa bao nhiêu, và còn thừa mấy đồng chưa ai trả.

- Phở `100000đ`, chia cho **3** người.
- Lẩu `250000đ`, chia cho **6** người.

Phần "mỗi người" đã viết sẵn bằng `//` của bài trước. Việc của bạn là hai chỗ
trống còn lại: phần thừa của từng khoản.

Bài chấm bằng cả hai khoản, vì hai phần thừa ấy là hai con số khác nhau — `1` và
`4`. Điền một con số cố định vào cả hai chỗ thì một trong hai dòng sẽ sai ngay.

```python title=starter
tien_bun = 100000
tien_lau = 250000

thua_bun = ___
thua_lau = ___

print(f"Bún chả chia 3 người: mỗi người {tien_bun // 3}đ, còn thừa {thua_bun}đ")
print(f"Lẩu chia 6 người: mỗi người {tien_lau // 6}đ, còn thừa {thua_lau}đ")
```

```python title=solution
tien_bun = 100000
tien_lau = 250000

thua_bun = tien_bun % 3
thua_lau = tien_lau % 6

print(f"Bún chả chia 3 người: mỗi người {tien_bun // 3}đ, còn thừa {thua_bun}đ")
print(f"Lẩu chia 6 người: mỗi người {tien_lau // 6}đ, còn thừa {thua_lau}đ")
```

```python title=test
# Hai khoản cho ra hai phần thừa KHÁC nhau (1 và 4), nên một con số gõ cứng
# chỉ qua được nhiều nhất một dòng. Hai assert cuối kiểm cái luật của bài:
# phần chia trọn cộng phần còn thừa phải dựng lại đúng hoá đơn ban đầu — thứ
# mà một đáp án đoán mò không tự nhiên thoả được.
assert thua_bun == 1, "100000 chia cho 3 người thì còn thừa 1đ"
assert thua_lau == 4, "250000 chia cho 6 người thì còn thừa 4đ"
assert tien_bun // 3 * 3 + thua_bun == tien_bun, "phần mỗi người cộng phần thừa phải bằng đúng hoá đơn"
assert tien_lau // 6 * 6 + thua_lau == tien_lau, "phần mỗi người cộng phần thừa phải bằng đúng hoá đơn"
# Kiểm "không có đuôi .0" bằng cách in ra chữ chứ không bằng `isinstance`: bài
# 13 của chính track này dạy người học đè lên cái tên `int`, nên trong bộ chấm
# cái tên ấy không đáng tin.
assert f"{thua_bun}" == "1" and f"{thua_lau}" == "4", "phần thừa đếm bằng đồng nên phải là số nguyên, không mang đuôi .0"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở vế phải hai dòng gán, và hai dòng `print` bên dưới đã cho bạn xem sẵn phần chia trọn được viết thế nào. Phần thừa hỏi trên đúng cặp số ấy, chỉ đổi dấu.
- kind: strategy
  body: Dấu bạn cần là dấu lấy phần dư, đặt giữa tên khoản tiền và số người — cùng chỗ đứng với hai gạch chéo trong dòng `print`. Số người của hai khoản khác nhau, bún chả 3 và lẩu 6, nên hai dòng không chép được cho nhau.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `tien_bun % 3` và `___` thứ hai bằng `tien_lau % 6`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: bài này dạy dấu lấy dư `%` — hãy dùng chính nó ở cả hai dòng, đừng dựng lại phần thừa bằng phép nhân rồi trừ
  requireAst:
  # `min: 2` vì có hai khoản. Khung đã sẵn hai dấu `//` trong hai dòng `print`
  # nên không hỏi được bằng `//`; `%` thì khung chưa nhắc lần nào.
  - kind: uses-operator, target: %, min: 2
  # Mỗi khoản phải lấy dư TRÊN CHÍNH KHOẢN ẤY.
  #
  # `100000 % 3` và `250000 % 3` cùng bằng 1; `100000 % 6` và `250000 % 6` cùng
  # bằng 4. Nên đổi `tien_bun` thành `tien_lau` ở chỗ trống — lấy dư của khoản
  # kia — vẫn ra đúng cả hai con số, qua sạch mọi assert, kể cả đẳng thức
  # `phần chia trọn + phần thừa = hoá đơn`. Cổng đột biến bắt được, và bản
  # trước đây khai miễn trừ hai đột biến ấy vì đổi con số thì kéo theo bài 4.
  #
  # Luật dưới đây đóng lỗ mà không đụng tới con số nào: khung đã đọc mỗi tên
  # đúng một lần trong dòng `print` của nó, nên lời giải thật đẩy lên 2, còn
  # đáp án lấy dư nhầm khoản tụt về 1. Sửa dòng `print` thì phải sửa cả đây.
  - kind: uses-name, target: tien_bun, min: 2
  - kind: uses-name, target: tien_lau, min: 2
- tier: output
  match: regex
  expect: ^Bún chả chia 3 người: mỗi người 33333đ, còn thừa 1đ\nLẩu chia 6 người: mỗi người 41666đ, còn thừa 4đ\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một đồng và bốn đồng. Giờ dòng sổ nói đủ cả hai vế, không bỏ sót đồng nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại bốn con số bạn vừa lấy ra: `33333` và `1`, `41666` và `4`. Cả bốn đều
tròn trịa, không đuôi, không lệch. Ghép lại thì đúng bằng hoá đơn, không rơi
rớt đồng nào.

Chúng sạch được như thế vì bạn đang đếm bằng **đồng** — mà đồng thì không có
phần lẻ để mà lệch.

Nhưng sổ chợ ngoài đời ít ai ghi bằng đồng. Người ta ghi bằng **nghìn**: trứng
`10.1`, thịt gà `20.2`. Tay bạn cộng hai con số ấy trên giấy, ra `30.3`, và bạn
chắc chắn về nó như chắc chắn về `1 + 1`.

Máy có cho ra đúng `30.3` không? Bài sau hỏi thẳng máy, và câu trả lời của nó
đáng để bạn ngồi lại một lúc.
::::

::::checkpoint{mastery=0.8}
::::
