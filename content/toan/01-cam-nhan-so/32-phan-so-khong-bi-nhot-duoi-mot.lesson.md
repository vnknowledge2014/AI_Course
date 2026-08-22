---
id: toan.cam-nhan-so.phan-so-khong-bi-nhot-duoi-mot
title: Phân số không bị nhốt dưới 1
summary: "Không có gì cấm tử lớn hơn mẫu. `7/4` là bảy lần cái thước `1/4`, và chỗ của nó nằm giữa 1 và 2."
locale: vi
track: toan
module: cam-nhan-so
order: 32
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.improper-fraction]
requires: [math.fraction, math.unit-fraction, math.thanh-so, math.number-line-add, math.like-units, math.addition-as-union, math.remainder, math.division-by-zero, core.output, core.arithmetic, core.division, core.boolean, ctrl.comparison]
concepts: [math.don-vi-va-thuoc, math.thanh-so, math.tu-va-mau]
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
Cái thước của bạn có bảo phải dừng ở vạch một đâu. Mình đặt tiếp thử.
::::

::::explain{#mieng-thu-nam}
Bài trước để lại câu hỏi: đặt cái thước `1/4` bảy lần thì được `7/4` — thanh số
có chỗ nào cho nó không?

Đừng trả lời bằng luật. Ra vườn nhìn tay mình làm gì đã.

Luống rau cải dài hơn luống rau muống. Byte đặt thước `1/4` sải xuống: miếng
một, miếng hai, miếng ba, miếng bốn — vừa hết đúng một sải, mà luống thì vẫn
còn đất. Không có ai đứng đó bảo phải dừng. Đặt tiếp: miếng năm, miếng sáu,
miếng bảy — hết luống.

Bảy lần cái thước `1/4`. Viết theo đúng luật bài trước: `7/4`.

Đây là chỗ hai cách đọc của bài trước rẽ hẳn sang hai đường.

- **"Ba cái bánh chia tư"** thì gãy ngay. Một cái bánh cắt tư thì lấy đâu ra
  miếng thứ năm? Muốn có bảy miếng phải đi mượn thêm bánh, mà "mượn thêm bánh"
  thì không nằm trong cách viết `7/4` — nhìn vào đó không thấy cái bánh thứ hai
  ở đâu cả.
- **"Bảy lần cái thước"** thì không vướng gì. Cái thước còn trong tay, đất còn
  dưới chân, cứ đặt tiếp.

Nên câu để nhớ của bài này ngắn thế này: **mẫu không phải cái bánh, mẫu chỉ là
cỡ thước.** Cỡ thước không quy định bạn được đặt mấy lần.
::::

::::example{#bay-mieng-tren-thanh-so}
Đặt bảy miếng ấy lên thanh số. Vẽ hai đường trên cùng một chỗ: đường trên đếm
bằng sải, đường dưới đếm bằng miếng `1/4`.

```text
0           1           2
├───────────┼───────────┤     đếm bằng sải: một sải, rồi một sải nữa
├──┼──┼──┼──┼──┼──┼──┼──┤     vẫn đường ấy, mỗi sải bẻ tư
0  1  2  3  4  5  6  7  8     đếm bằng miếng 1/4
                  ▲
                  bảy miếng — tức 7/4 sải — đứng giữa 1 và 2
```

Nhìn cái vạch số `4` ở hàng dưới: nó rơi trúng vạch `1` ở hàng trên. Bốn miếng
`1/4` ghép lại đúng bằng cái đơn vị đã bị bẻ ra để làm chúng — đó là câu bài 30
đã nói khi bẻ thước, chỉ giờ mới đem ra dùng:

> `4/4` và `1` là cùng một chỗ.

Có nó rồi thì bảy miếng tách ra được. Bảy miếng cùng cỡ nằm chung một đống,
gộp đống nào trước cũng thế (bài 10), nên gộp bốn miếng đầu lại thành một sải,
còn dư ba miếng:

`7/4 = 4/4 + 3/4 = 1 và 3/4`

Đọc thành tiếng: *một sải và ba phần tư sải*. Đây đúng là câu người bán vải hay
nói — "một thước rưỡi", "hai thước tư" — và giờ bạn viết được nó bằng số.

Hỏi máy làm trọng tài cho cả hai khẳng định:

```python title=readonly
print(4/4 == 1)
print(7/4 == 4/4 + 3/4)
```

Máy in ra:

```text
True
True
```
::::

::::predict{#doan-mot-sai-va-ba-mieng commitOnce}
Byte viết cùng một luống rau cải bằng hai cách: `7/4` sải, và "một sải cộng
`3/4` sải".

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(7/4 == 1 + 3/4)
```

:::opt{correct}
True
:::

:::opt
False
::why
Gần đúng ở chỗ bạn giữ đúng luật bài 11: chỉ gộp được thứ **cùng đơn vị**. Vế
phải trông như đang trộn một sải nguyên với ba miếng nhỏ — hai thứ khác cỡ —
và bạn từ chối gộp chúng. Sự cảnh giác ấy đúng chỗ, nó cứu bạn ở rất nhiều bài
khác.

Chỗ lệch: một sải nguyên **viết lại được** thành bốn miếng `1/4`, tức `4/4`.
Đổi xong thì hai vế cùng đếm một loại miếng: bốn miếng cộng ba miếng, ra bảy
miếng. Luật bài 11 không bị phá ở đây — nó vừa được dùng đúng.
::
:::

:::opt
Máy báo lỗi vì tử lớn hơn mẫu
::why
Gần đúng ở chỗ bạn nhớ rằng máy có dừng lại thật khi câu hỏi hỏng — bài 28 vừa
cho thấy `12 : 0` không có kết quả nào cả. Bạn đang áp đúng cái phản xạ ấy.

Chỗ lệch nằm ở chỗ câu hỏi này **không** hỏng. `12 : 0` hỏng vì đặt đoạn dài 0
mét thì bao nhiêu lần cũng không lấp nổi 12 mét. Còn `7/4` thì đặt được: bảy
lần một cái thước có thật, ra một lượng đất có thật. Với máy, `7/4` chỉ là phép
chia 7 cho 4 của bài 26 — chia bảy sải cho bốn người thì mỗi người vẫn có phần.
::
:::

:::opt
1 dư 3
::why
Gần đúng ở chỗ bạn nối được `7/4` với phép chia có dư ở bài 29: bảy chia bốn
được một, dư ba. Cách đọc ấy đúng, và "một dư ba" chính là "một sải và ba
miếng" — bạn đang nhìn thẳng vào nội dung bài này.

Có hai chỗ lệch nhỏ. Một: `==` là câu hỏi có–không, nên thứ đi ra khỏi nó luôn
là `True` hoặc `False`, không bao giờ là một cặp số. Hai: chữ "dư 3" bỏ mất tên
cái thước — ba **cái gì**? Ba sải hay ba miếng? Cách viết `3/4` giữ lại đúng
cái tên ấy, và đó là lý do phân số đi xa hơn số dư.
::
:::
::::

::::explain{#tu-lon-hon-mau-van-la-so}
Gom lại thành một câu: **không có gì cấm tử lớn hơn mẫu.**

Tử là số đếm — đếm xem lấy mấy cái thước. Mẫu là cỡ thước. Hai con số ấy trả
lời hai câu hỏi khác nhau, nên chúng không phải so kè với nhau, và cái này
không đặt trần cho cái kia.

Nhìn theo thanh số thì càng rõ chuyện gì đang xảy ra: mỗi lần tử tăng thêm 1 là
tay bạn dịch sang phải đúng một miếng — vẫn là phép cộng "bước sang phải" của
bài 13, chỉ khác bước chân ngắn hơn. Đi mãi thì phải qua vạch `1`, rồi qua vạch
`2`. Không có bức tường nào ở đó.

Và một hệ quả gọn: `a/b` bằng đúng `1` khi `a` bằng `b`, nhỏ hơn `1` khi tử nhỏ
hơn mẫu, lớn hơn `1` khi tử lớn hơn mẫu. Không phải luật mới cần học thuộc —
chỉ là "đếm miếng đã đủ một sải chưa" nói bằng chữ khác.
::::

::::code{#do-hai-luong}
Byte đo hai luống, cùng bằng một cái thước `1/4` sải:

- **Luống rau cải**: đặt vừa 7 lần → `7/4` sải.
- **Luống rau muống**: đặt vừa 3 lần → `3/4` sải.

Hai chỗ trống là cùng một câu hỏi hỏi về hai luống — và hai luống này được chọn
để cho ra hai câu trả lời **ngược nhau**. Gõ cứng `True` vào cả hai thì luống
rau muống sai; gõ cứng `False` thì luống rau cải sai.

```python title=starter
# Câu 1: luống rau CẢI có dài hơn một sải không?
print(___)

# Câu 2: luống rau MUỐNG có dài hơn một sải không?
print(___)

# Câu 3 — đã viết sẵn: bảy miếng có đúng bằng "một sải và ba miếng" không?
print(7/4 == 4/4 + 3/4)
```

```python title=solution
# Câu 1: luống rau CẢI có dài hơn một sải không?
print(7/4 > 1)

# Câu 2: luống rau MUỐNG có dài hơn một sải không?
print(3/4 > 1)

# Câu 3 — đã viết sẵn: bảy miếng có đúng bằng "một sải và ba miếng" không?
print(7/4 == 4/4 + 3/4)
```

```python title=test
# Bốn assert chốt lại bốn điều bài vừa dạy, để bài học tự canh lấy mình.
assert 7/4 > 1, "bảy miếng 1/4 dài hơn một sải — phân số không bị nhốt dưới 1"
assert 3/4 < 1, "ba miếng 1/4 thì chưa đủ một sải"
assert 4/4 == 1, "bốn lần thước 1/4 ghép lại đúng bằng cái đơn vị đã bẻ ra chúng"
assert 7/4 == 4/4 + 3/4, "bảy miếng = bốn miếng + ba miếng, và bốn miếng là một sải"
```

:::hints
- kind: attention
  body: Câu hỏi "có dài hơn một sải không" so cái luống với đúng một thứ, và thứ đó là một sải — trên thanh số nó là vạch số 1.
- kind: strategy
  body: "Bài 17 đã cho bạn dấu để hỏi câu ấy: lớn hơn nghĩa là đứng bên phải. Vế trái là cái luống viết bằng phân số, vế phải là số 1. Hai dòng khác nhau ở đúng một chỗ: số miếng đã đếm được."
- kind: one-line
  body: "Chỗ trống thứ nhất là `7/4 > 1`, chỗ trống thứ hai là `3/4 > 1`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu so sánh (`>`) giữa cái luống viết bằng phân số và số 1 — gõ thẳng `True` hay `False` thì không hỏi máy điều gì cả
  requireAst:
  # Khung chưa có dấu `>` nào, nên `min: 2` chặn đúng đáp án gõ cứng hai chữ
  # True/False. `/` min 4 buộc hai câu hỏi phải viết bằng phân số: dòng thứ ba
  # đã sẵn có hai dấu `/`, nên mức 4 chỉ đạt khi hai chỗ trống cũng là phân số.
  - kind: uses-operator, target: >, min: 2
  - kind: uses-operator, target: /, min: 4
- tier: output
  match: regex
  expect: ^True\nFalse\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Miếng thứ năm vẫn là một miếng. Vạch số một không phải bức tường.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte bẻ đôi sải dây, được cái thước `1/2`. Đo luống hẹ hết đúng **một** miếng,
nên Byte ghi vào sổ: `1/2` sải.

An không có mặt lúc đó. An bẻ tư sải dây, được thước `1/4`, đo đúng cái luống
hẹ ấy và hết **hai** miếng. An ghi: `2/4` sải.

Đem cả hai lên thanh số thì chúng rơi trúng cùng một chỗ. Cùng một luống đất,
mà sổ ghi hai cách viết khác hẳn nhau.

Bài 2 từng bảo: đếm thì ai đếm cũng ra một kết quả. Vậy ở đây, hai con số khác
nhau nghĩa là có người đếm sai — hay là có chuyện gì khác đang xảy ra? Bài sau
trả lời.
::::

::::checkpoint{mastery=0.8}
::::
