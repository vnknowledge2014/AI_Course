---
id: toan.cam-nhan-so.doi-thuoc-khong-doi-luong
title: Đổi thước, không đổi lượng
summary: "Nhân cả tử lẫn mẫu với cùng một số là đo lại đúng lượng ấy bằng cái thước nhỏ hơn bấy nhiêu lần — đất không nhúc nhích."
locale: vi
track: toan
module: cam-nhan-so
order: 33
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.equivalent-fraction]
requires: [math.fraction, math.improper-fraction, math.unit-fraction, math.thuoc-do, math.thanh-so, math.multiplication, math.like-units, math.division-by-zero, core.output, core.arithmetic, core.division, core.boolean, ctrl.comparison]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Bạn đổi cái thước trong tay. Đất trong vườn mình không nhúc nhích đâu.
::::

::::explain{#hai-so-mot-luong}
Bài trước để lại một chuyện khó chịu: Byte ghi luống hẹ là `1/2` sải, An ghi
cùng cái luống ấy là `2/4` sải, và trên thanh số hai cách viết rơi trúng cùng
một chỗ.

Có ai đếm sai không? Không ai cả. Cả hai đều đếm đúng số miếng mình đặt xuống.

Chuyện này bạn đã gặp rồi, ở bài 3, chỉ là lúc đó nó nói bằng chữ khác:

> Đổi thước thì **con số** đổi, còn **lượng đất** thì không đổi tí nào.

Ở bài 3, Byte bảo luống dài `4`, An bảo `12`, và không ai sai — họ đo bằng hai
cái thước khác cỡ. Đây đúng là chuyện ấy, chỉ khác chỗ hai cái thước lần này
đều nhỏ hơn một sải, nên con số viết ra là phân số.

Giờ nhìn kỹ An làm gì để có cái thước của mình. An **bẻ đôi** cái thước `1/2`.
Một động tác, mà hai chuyện xảy ra cùng lúc:

- Cái sải dây trước bị bẻ làm 2, giờ mỗi nửa lại bẻ đôi nữa, thành 4 phần.
  **Mẫu nhân 2** — thước nhỏ đi hai lần.
- Cái miếng duy nhất của Byte giờ nằm đó dưới dạng hai miếng nhỏ. An đếm được
  2. **Tử nhân 2** — số miếng nhiều lên đúng hai lần.

Không ai thêm một tấc đất, không ai bớt một tấc đất. Cái luống nằm im từ đầu
tới cuối. Chỉ có cái thước đổi cỡ, và con số đổi theo đúng bấy nhiêu lần.

Đó là lý do `1/2` và `2/4` cùng một chỗ: chúng là **một lượng đất, đo hai lần
bằng hai cái thước**.
::::

::::example{#be-doi-tung-mieng}
Vẽ ra thì thấy ngay không có tấc đất nào đi đâu:

```text
một sải          ├───────────────────────────────┤
thước 1/2        ├───────────────┼───────────────┤
luống hẹ         ├───────────────┤                  1 miếng cỡ 1/2  →  1/2 sải

bẻ đôi từng miếng — cùng cái luống ấy, không xê dịch:
thước 1/4        ├───────┼───────┼───────┼───────┤
luống hẹ         ├───────┼───────┤                  2 miếng cỡ 1/4  →  2/4 sải
```

Hai hàng cuối cùng dài đúng bằng nhau. Cái vạch mới ở giữa không cắt bớt gì —
nó chỉ chia đôi chỗ đã có.

Làm lại một lần nữa với luống rau muống `3/4` sải của bài 31, lần này bẻ đôi
cái thước `1/4`:

- thước `1/4` bẻ đôi thành thước `1/8` — mẫu `4` nhân 2 thành `8`;
- mỗi miếng cũ hoá hai miếng mới, 3 miếng thành 6 — tử `3` nhân 2 thành `6`.

Nên `3/4` và `6/8` cũng là một chỗ. Hỏi máy làm trọng tài:

```python title=readonly
print(1/2 == 2/4)
print(3/4 == 6/8)
```

Máy in ra:

```text
True
True
```

Máy đồng ý cả hai lần. Nhưng lý do thật thì không nằm trong máy — nó nằm ở chỗ
cái luống chưa hề nhúc nhích.
::::

::::predict{#doan-khi-quen-mot-ve commitOnce}
Byte cũng muốn viết lại `3/4` bằng cái thước nhỏ đi hai lần. Byte nhân tử `3`
với 2 thành `6`, rồi bị gọi ra vườn, quên mất chưa động tới mẫu. Trong sổ còn
lại `6/4`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(3/4 == 6/4)
```

:::opt{correct}
False
:::

:::opt
True
::why
Gần đúng ở chỗ bạn dùng đúng cái luật vừa học: nhân lên thì vẫn là một lượng.
Luật ấy có thật, và bạn nhớ nó không sai chữ nào.

Chỗ lệch nằm ở **phạm vi** của nó: luật chỉ chạy khi **cả hai** con số cùng
nhân, vì hai con số ấy ghi lại hai chuyện xảy ra cùng lúc trong một động tác bẻ
đôi. Nhân mỗi tử là lấy gấp đôi số miếng mà **không** bẻ nhỏ miếng nào — sáu
miếng to bằng miếng cũ thì đất nhiều lên gấp đôi thật. Ngược lại, nhân mỗi mẫu
là bẻ nhỏ miếng mà giữ nguyên số miếng, thì đất ít đi.
::
:::

:::opt
Máy báo lỗi vì `6/4` có tử lớn hơn mẫu
::why
Gần đúng ở chỗ bạn thấy `6` lớn hơn `4` và dừng lại kiểm tra — thói quen đó
đúng ở nhiều chỗ.

Chỗ lệch: bài 32 vừa gỡ đúng cái bức tường ấy. `6/4` là sáu lần cái thước
`1/4`, một lượng đất có thật, và chỗ đứng của nó nằm giữa vạch `1` và vạch `2`.
Nó là một con số đàng hoàng — chỉ là **không phải** con số mà Byte định viết.
::
:::

:::opt
True, vì cả hai rút gọn về cùng một chỗ
::why
Gần đúng ở chỗ bạn dùng đúng chiều ngược của luật: chia cả tử lẫn mẫu của `6/4`
cho 2 thì được `3/2`, và phép rút gọn ấy bạn làm không sai bước nào — đó chính
là nửa sau của bài này.

Chỗ lệch: `3/2` không phải `3/4`. Rút gọn giữ nguyên chỗ đứng của `6/4`, mà chỗ
ấy nằm bên phải vạch `1` (sáu miếng thì đã quá bốn miếng), còn `3/4` thì nằm
bên trái. Hai chỗ khác nhau trên thanh số là hai số khác nhau, dù rút gọn xong
trông chúng có vẻ gần nhau hơn.
::
:::
::::

::::explain{#luat-doi-thuoc}
Giờ phát biểu được thành một câu dùng đi dùng lại suốt phần còn lại của track:

> `a/b` và `(a×k)/(b×k)` là **cùng một chỗ** trên thanh số, với mọi số đếm `k`.

Đi theo chiều **nhân** là bẻ nhỏ cái thước: mỗi miếng cũ hoá `k` miếng mới, nên
số miếng cũng nhân `k`. Đi theo chiều **chia** — chia cả tử lẫn mẫu cho cùng
một số — là gộp `k` miếng nhỏ lại thành một miếng to hơn. Chiều chia có sẵn một
cái tên bạn từng nghe: **rút gọn**. Nó không phải một mẹo làm bài; nó là đo lại
bằng cái thước to hơn.

Hai phân số ở cùng một chỗ như thế gọi là **phân số tương đương**.

Một điều kiện nhỏ nhưng phải nói: `k` không được là `0`. Bẻ một cái thước làm
`0` phần thì không còn cái thước nào để đo — đúng chỗ bài 28 đã chỉ ra rằng câu
hỏi ấy hỏng, chứ không phải máy cấm.

Và để ý một chuyện: luật này không cho bạn thêm đất, cũng không cho bớt đất. Nó
chỉ cho bạn **chọn cái thước tiện nhất** để nói về chừng ấy đất. Đó là toàn bộ
sức mạnh của nó — và bài sau sẽ tiêu đúng cái sức mạnh ấy.
::::

::::code{#do-lai-bang-thuoc-khac}
Luống rau muống dài `3/4` sải — ba miếng cỡ `1/4`. Byte muốn ghi lại nó bằng
thước `1/8`, và nhờ máy làm trọng tài cho hai cách ghi.

Hai chỗ trống cho ra hai câu trả lời **ngược nhau**: gõ cứng `True` vào cả hai
thì câu 2 sai, gõ cứng `False` thì câu 1 sai.

```python title=starter
# Câu 1: bẻ đôi từng miếng thì thước thành 1/8 VÀ số miếng gấp đôi.
#        Ghi lại cái luống bằng thước 1/8 — có còn đúng chừng ấy đất không?
print(___)

# Câu 2: nếu chỉ bẻ nhỏ thước mà quên nhân số miếng — vẫn ba miếng, nhưng là
#        ba miếng cỡ 1/8 — thì có còn đúng chừng ấy đất không?
print(___)

# Câu 3 — đã viết sẵn: còn đây là cái sổ Byte ghi dở lúc bị gọi ra vườn.
print(3/4 == 6/4)
```

```python title=solution
# Câu 1: bẻ đôi từng miếng thì thước thành 1/8 VÀ số miếng gấp đôi.
#        Ghi lại cái luống bằng thước 1/8 — có còn đúng chừng ấy đất không?
print(3/4 == 6/8)

# Câu 2: nếu chỉ bẻ nhỏ thước mà quên nhân số miếng — vẫn ba miếng, nhưng là
#        ba miếng cỡ 1/8 — thì có còn đúng chừng ấy đất không?
print(3/4 == 3/8)

# Câu 3 — đã viết sẵn: còn đây là cái sổ Byte ghi dở lúc bị gọi ra vườn.
print(3/4 == 6/4)
```

```python title=test
# Năm assert chốt lại đúng năm điều bài vừa dạy — hai chiều của luật, và hai
# kiểu quên một vế.
assert 3/4 == 6/8, "bẻ đôi từng miếng: thước nhỏ đi hai lần, số miếng gấp đôi, đất giữ nguyên"
assert 3/4 == 12/16, "bẻ mỗi miếng làm bốn cũng vẫn đúng chừng ấy đất"
assert 6/8 == 12/16, "luật chạy giữa hai phân số bất kỳ, không riêng gì với 3/4"
assert 3/4 != 3/8, "bẻ nhỏ thước mà giữ nguyên số miếng thì đất ít đi một nửa"
assert 3/4 != 6/4, "gấp đôi số miếng mà không bẻ nhỏ miếng nào thì đất nhiều lên gấp đôi"
```

:::hints
- kind: attention
  body: Cả hai chỗ trống đều là câu hỏi "có còn đúng chừng ấy đất không", nên vế trái của cả hai đều là cái luống lúc đầu, viết bằng thước cũ.
- kind: strategy
  body: Vế phải mới là chỗ hai câu khác nhau. Câu 1 làm đủ hai việc của một lần bẻ đôi: mẫu 4 thành 8, và tử 3 cũng phải nhân 2. Câu 2 cố tình chỉ làm một việc: mẫu đổi thành 8 mà tử vẫn để nguyên 3.
- kind: one-line
  body: "Chỗ trống thứ nhất là `3/4 == 6/8`, chỗ trống thứ hai là `3/4 == 3/8`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh bằng (`==`) giữa hai phân số — gõ thẳng `True` hay `False` thì không hỏi máy điều gì cả
  requireAst:
  # Dòng thứ ba viết sẵn đã có một `==` và hai dấu `/`. Đòi `==` từ 3 và `/`
  # từ 4 trở lên nghĩa là hai chỗ trống cũng phải là câu hỏi bằng viết bằng
  # phân số — gõ True/False hay chép sẵn một số thập phân đều không đạt.
  - kind: uses-operator, target: ==, min: 3
  - kind: uses-operator, target: /, min: 4
- tier: output
  match: regex
  expect: ^True\nFalse\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một luống, ba cách ghi. Chỉ một cách giữ nguyên được chừng ấy đất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`3/4` viết được thành `6/8`, thành `12/16`, thành `300/400` — vô số cách, tất
cả cùng một chỗ. Nghe như chuyện rắc rối, nhưng nó vừa cho bạn một quyền: bạn
được **chọn** cái thước tiện nhất cho việc đang làm.

Đem quyền ấy ra vườn thử ngay. Luống dưa dài `2/3` sải, luống bí dài `3/4` sải.
Luống nào dài hơn?

Nhìn tử thì `2` bé hơn `3`. Nhìn mẫu thì `3` bé hơn `4`. Hai cái nhìn ấy không
kết luận được gì, vì hai bên đang đo bằng hai cái thước khác cỡ — mà bài 11 đã
dặn: chỉ đem so với nhau được những thứ cùng một đơn vị.

Bạn đã được phép đổi thước rồi. Vậy phải đổi cả hai về cái thước nào để chúng
cùng đo được, và có luôn tìm ra được một cái thước như thế không? Bài sau trả
lời.
::::

::::checkpoint{mastery=0.8}
::::
