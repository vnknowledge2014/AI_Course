---
id: toan.cam-nhan-so.phan-tram-cua-cai-gi
title: Phần trăm của cái gì
summary: Mỗi con số phần trăm bám vào một cái toàn thể = 100%. Đổi cái toàn thể thì cùng một con số mang lượng khác — nên tăng 50% rồi giảm 50% không về chỗ cũ.
locale: vi
track: toan
module: cam-nhan-so
order: 39
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.percent-whole]
requires: [math.percent, math.multiplication, math.division-partitive, math.multiply-as-scaling, core.arithmetic, core.division, core.float, core.variable, core.print-variable, ctrl.comparison]
concepts: [math.phan-so, math.so-sanh, math.don-vi-do]
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
Ba mươi phần trăm. Của cái gì? Không nói ra thì mình chưa đếm được cây nào.
::::

::::explain{#con-so-30-con-thieu-gi}
Bài trước để lại một câu hỏi: cả hai vườn cùng mất **30%**, mà vườn Byte mất 12
cây còn vườn An mất 75 cây. Con số 30 ấy còn thiếu gì?

Thiếu **cái toàn thể**.

Nhớ lại cách bài 38 dựng phần trăm lên: `1%` là một cái thước cỡ `1/100`. Nhưng
`1/100` của **cái gì**? Bài 30 đã nói rõ từ đầu: bẻ một đơn vị thành `b` phần
bằng nhau thì mới có thước `1/b`. Có một cái đơn vị bị bẻ, thì mới có cái thước.

Với phần trăm, cái đơn vị bị bẻ ấy chính là **cái toàn thể**, và người ta gọi
nó là **100%**.

```text
vườn Byte:  100% = 40 cây    →  1% = 0,4 cây   →  30% = 12 cây
vườn An:    100% = 250 cây   →  1% = 2,5 cây   →  30% = 75 cây
```

Cùng chữ `30%`, hai cái thước khác cỡ hẳn nhau, nên hai lượng khác hẳn nhau.

Có một cách tính gọn hơn cho cùng chuyện đó, và nó chỉ dùng lại hai bài cũ.
`30% = 30/100`, rút gọn (bài 33) thành `3/10`. Lấy `3/10` của một lượng thì:
**chia lượng ấy cho 10** (bài 26 — chia đều thành 10 phần), rồi **lấy 3 lần**
(bài 19 — lặp lại một lượng mấy lần).

```text
40 chia 10 = 4,     4 lấy 3 lần = 12
250 chia 10 = 25,   25 lấy 3 lần = 75
```

Không có mẹo mới nào ở đây cả. "Lấy `p%` của một lượng" là bẻ lượng ấy ra 100
phần rồi nhặt `p` phần — đúng nghĩa đen của phân số, áp lên một cái toàn thể
không phải số 1.

Nên câu đầy đủ luôn phải có ba mảnh, thiếu một mảnh là chưa nói được gì:

> **30%** — của **cái vườn 40 cây** — là **12 cây**.
::::

::::predict{#doan-gia-gao commitOnce}
Chợ đầu làng bán gạo **20.000 đồng một ký**. Tháng trước chợ tăng giá **50%**.
Tháng này chợ giảm **50%**.

**Trước khi tính ra giấy**, bạn đoán một ký gạo bây giờ bao nhiêu?

:::opt{correct}
15.000 đồng
:::

:::opt
20.000 đồng — tăng bao nhiêu rồi giảm bấy nhiêu thì về chỗ cũ
::why
Gần đúng ở chỗ bạn dùng một luật hoàn toàn thật: cộng thêm một lượng rồi trừ đi
**đúng lượng ấy** thì quay về chỗ cũ. Đó chính là bài 18 — cộng số đối thì về
mốc. Với số thì luật này không hỏng lần nào.

Chỗ lệch nằm ở chữ `50%` không phải một **lượng**. Nó là một chỉ dẫn: "lấy một
nửa của cái đang có". Lần tăng, cái đang có là 20.000 nên lượng thêm là 10.000.
Lần giảm, cái đang có đã thành 30.000 nên lượng bớt là 15.000. Cộng 10.000 rồi
trừ 15.000 thì không về chỗ cũ được. Luật bài 18 vẫn đúng — chỉ là hai lượng ở
đây không bằng nhau như bạn tưởng.
::
:::

:::opt
10.000 đồng — giảm 50% thì còn một nửa của 20.000
::why
Gần đúng ở chỗ bạn đọc "giảm 50%" thành "còn lại một nửa", và cách đọc ấy chính
xác: bớt đi 50 trên 100 phần thì đúng là còn 50 trên 100 phần.

Chỗ lệch: một nửa **của cái gì**. Bạn lấy một nửa của giá gốc 20.000, nhưng lúc
chợ giảm giá thì giá đang là 30.000 — cái toàn thể mà chữ `50%` bám vào luôn là
cái đang có **tại thời điểm ấy**, không phải cái hồi đầu. Một nửa của 30.000 là
15.000, nên giá cuối là 15.000.
::
:::

:::opt
Chưa tính được — thiếu mất cái toàn thể
::why
Gần đúng ở chỗ bạn nắm đúng điều bài này dạy, và nắm chắc: con số phần trăm nào
cũng phải bám vào một cái toàn thể, không có nó thì con số chưa mang lượng.
Phản xạ ấy sẽ cứu bạn nhiều lần về sau.

Chỗ lệch: ở đây cái toàn thể có đủ cả hai lần, chỉ là nó **không được nhắc
tên**. Lần tăng bám vào giá gốc 20.000 đã cho sẵn; lần giảm bám vào giá vừa
tăng, mà giá ấy tự bạn tính ra được. Đề không thiếu dữ kiện — nó chỉ thay cái
toàn thể giữa chừng mà không báo.
::
:::
::::

::::example{#cai-toan-the-doi-giua-chung}
Đáp án là `15.000` — không về chỗ cũ. Vì sao thì phải đi từng bước lại chuyện
bao gạo ấy, và ở mỗi bước hỏi thẳng "50% của cái gì":

```text
Giá gốc:            20.000 đ        ← đây là 100% lúc tăng
Tăng 50% của nó:    +10.000 đ
Giá mới:            30.000 đ        ← đây là 100% lúc giảm
Giảm 50% của nó:    −15.000 đ
Giá cuối:           15.000 đ
```

`15.000` chứ không phải `20.000`. Rẻ hơn giá gốc một phần tư.

Chỗ trượt chân nằm ở dòng thứ ba. Sau khi tăng, cái toàn thể **không còn là**
20.000 nữa — nó là 30.000. Chữ `50%` ở lần hai bám vào một cái toàn thể khác
cái nó bám ở lần một. Cùng một con số 50, hai lượng khác nhau: 10.000 và
15.000.

Bức tranh thứ hai cho cùng chuyện này, vẽ trên **thanh số** như bài 22 đã dựng.
Tăng 50% là **kéo giãn** đoạn ra một lần rưỡi. Giảm 50% là **co lại** còn một
nửa. Kéo giãn một lần rưỡi rồi co còn một nửa thì rốt cuộc đoạn dài bao nhiêu?

```text
1,5 lần rồi × một nửa  =  0,75 lần  →  còn ba phần tư đoạn ban đầu
20.000 × 0,75 = 15.000
```

Hai bức tranh — sơ đồ dải từng bước, và thanh số co giãn — cho cùng một con số.
Bức thứ nhất cho thấy **vì sao** hụt: cái toàn thể đã đổi. Bức thứ hai cho thấy
**hụt bao nhiêu**: còn `3/4`, và con số `3/4` ấy không phụ thuộc vào giá gạo
là 20.000 hay bao nhiêu đi nữa.

Và đây không phải chuyện chợ búa vặt vãnh. Cùng một cái bẫy nằm trong: "giảm
giá 50%, hôm nay giảm thêm 20%", "lương tăng 10% rồi công ty cắt 10%", "cửa
hàng tăng giá 30% rồi treo biển sale 30%".
::::

::::explain{#hoi-may-tren-hai-tinh-huong}
Hai tình huống trong bài này nói cùng một điều từ hai phía:

- **Cùng một con số phần trăm, hai cái toàn thể** → hai lượng khác nhau
  (30% của 40 cây và 30% của 250 cây).
- **Cùng một con số phần trăm, cái toàn thể đổi giữa chừng** → tăng rồi giảm
  không về chỗ cũ.

Đem cả hai ra hỏi máy. Máy không biết cái nào là toàn thể — chính bạn phải nói
ra, bằng cách viết đúng con số ấy vào phép tính. Và đó cũng là chỗ dễ viết
nhầm nhất.
::::

::::code{#viet-ro-cai-toan-the}
Điền hai chỗ trống. Mỗi chỗ trống là một câu hỏi "phần trăm **của cái gì**", và
hai chỗ trả lời hai cái toàn thể khác nhau.

Bài chấm bằng cả hai tình huống. Ghi cứng một con số vào thì tình huống kia
sai, và nếu chỗ trống thứ hai bám nhầm vào `gia_goc` thì giá cuối ra `10.000`
— cổng bắt được ngay.

```python title=starter
# Tình huống 1 — cùng "30%", hai cái vườn khác cỡ
ca_vuon_byte = 40
ca_vuon_an = 250
ba_muoi_byte = ca_vuon_byte * 30 / 100   # 30% của CẢ VƯỜN Byte
ba_muoi_an = ___                         # 30% của CẢ VƯỜN An

# Tình huống 2 — cùng "50%", nhưng cái toàn thể đổi giữa chừng
gia_goc = 20000
tang_them = gia_goc * 50 / 100
gia_moi = gia_goc + tang_them
bot_di = ___                     # giảm 50% — 50% của giá NÀO?
gia_cuoi = gia_moi - bot_di

print(ba_muoi_byte)
print(ba_muoi_an)
print(gia_moi)
print(gia_cuoi)
```

```python title=solution
# Tình huống 1 — cùng "30%", hai cái vườn khác cỡ
ca_vuon_byte = 40
ca_vuon_an = 250
ba_muoi_byte = ca_vuon_byte * 30 / 100   # 30% của CẢ VƯỜN Byte
ba_muoi_an = ca_vuon_an * 30 / 100       # 30% của CẢ VƯỜN An

# Tình huống 2 — cùng "50%", nhưng cái toàn thể đổi giữa chừng
gia_goc = 20000
tang_them = gia_goc * 50 / 100
gia_moi = gia_goc + tang_them
bot_di = gia_moi * 50 / 100      # giảm 50% — 50% của giá NÀO?
gia_cuoi = gia_moi - bot_di

print(ba_muoi_byte)
print(ba_muoi_an)
print(gia_moi)
print(gia_cuoi)
```

```python title=test
# Tình huống 1: cùng 30%, hai lượng khác hẳn nhau.
assert ba_muoi_byte == 12, "30% của ca_vuon_byte, tức 40 cây, là 12 cây"
assert ba_muoi_an == 75, "30% của ca_vuon_an, tức 250 cây, là 75 cây — cùng con số 30, mà hơn sáu lần"
assert ba_muoi_an != ba_muoi_byte, "cùng 30% mà ra hai lượng khác nhau: đó là cả nội dung bài này"

# Tình huống 2: cái toàn thể lúc giảm là giá MỚI, không phải giá gốc.
assert bot_di == 15000, "lúc giảm, 100% là 30.000 chứ không còn là 20.000"
assert gia_cuoi == 15000, "tăng 50% rồi giảm 50% thì còn 15.000"
assert gia_cuoi != gia_goc, "KHÔNG quay về chỗ cũ — đây là chỗ nhiều người tính nhầm cả đời"
assert gia_cuoi == gia_goc * 75 / 100, "còn đúng ba phần tư giá gốc: kéo giãn 1,5 lần rồi co còn một nửa"
```

:::hints
- kind: attention
  body: Chỗ trống thứ hai nằm ngay dưới dòng đã tính ra `gia_moi`. Hỏi lại chính mình câu trong lời chú thích — lúc chợ giảm giá, cái giá đang treo trên bảng là giá nào?
- kind: strategy
  body: Cả hai chỗ trống dùng chung một khuôn với dòng đã viết sẵn ở trên nó — cái toàn thể, nhân với con số phần trăm, rồi chia cho 100. Việc duy nhất bạn phải quyết là điền cái toàn thể nào vào đầu khuôn ấy. Dùng tên biến chứ đừng chép con số, vì đọc lên là đọc ra ngay "30% của cái gì".
- kind: one-line
  body: "Viết `ca_vuon_an * 30 / 100` vào chỗ trống thứ nhất, và `gia_moi * 50 / 100` vào chỗ thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một phép tính phần trăm thật (cái toàn thể × con số phần trăm ÷ 100) — chép sẵn kết quả vào thì bài không hỏi bạn cái toàn thể nào cả
  requireAst:
  # Lời giải có bốn phép chia cho 100; khung khởi đầu chỉ có hai. `min: 4`
  # buộc mỗi chỗ trống phải tự mang một phép chia của riêng nó.
  - kind: uses-operator, target: /, min: 4
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^12\.0\n75\.0\n30000\.0\n15000\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giá cuối 15.000. Cái toàn thể đổi giữa chừng, mà không ai nói cho mình biết.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mọi phần trăm trong bài này đều so **một phần với cái toàn thể chứa nó**: cây
chết nằm trong vườn, tiền tăng nằm trong giá. Cái nhỏ luôn nằm gọn bên trong
cái lớn, và đó là lý do `100%` có nghĩa.

Nhưng sáng nay Byte gieo hạt và ghi vào sổ: vườn có **90 hạt** và **6 luống**.

Hạt không nằm trong luống theo kiểu cây nằm trong vườn — 6 luống không phải một
cái toàn thể mà 90 hạt là một phần của nó. Chúng là hai thứ **khác loại**, đếm
bằng hai đơn vị khác nhau, chẳng cái nào chứa cái nào. Hỏi "90 hạt là bao nhiêu
phần trăm của 6 luống" thì câu hỏi không có nghĩa gì.

Vậy mà rõ ràng hai con số ấy có liên quan tới nhau: gieo 90 hạt vào 6 luống thì
dày, gieo 90 hạt vào 30 luống thì thưa. Có một mối quan hệ thật ở đó.

So hai thứ mà không cái nào nằm trong cái nào — thì so kiểu gì? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
