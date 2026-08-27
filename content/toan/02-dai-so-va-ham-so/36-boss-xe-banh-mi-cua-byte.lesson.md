---
id: toan.dai-so-va-ham-so.boss-xe-banh-mi-cua-byte
title: BOSS — Xe bánh mì của Byte
summary: Một buổi bán hàng, bốn câu hỏi rất đời — và để trả lời chúng phải cầm lên gần như cả track, từ câu tính còn ô trống tới cái ống nối hai máy.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 36
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [math.function-composition, math.composition-order, math.inverse-function, math.linear-function, math.quadratic-function, math.parabola-symmetry, math.vertex, math.exponential-function, math.exponential-beats-linear, math.equation, math.check-solution, math.inequality, math.solution-set, math.factor-common, math.expand-brackets, math.value-table, math.slope, math.identity, math.exponent, math.don-vi-roi-va-lien]
requires: [math.function-composition, math.composition-order, math.inverse-function, math.function, math.function-notation, math.slope, math.linear-function, math.quadratic-function, math.parabola-symmetry, math.vertex, math.exponential-function, math.exponential-beats-linear, math.equation, math.check-solution, math.inequality, math.solution-set, math.identity, math.value-table, math.expression, math.substitution, math.letter-names-a-slot, math.factor-common, math.expand-brackets, math.multiply-distributive, math.order-of-operations, math.parentheses, math.exponent, math.additive-inverse, math.negative-number, math.multiplication, math.don-vi-roi-va-lien, math.don-vi, core.function-def, core.function-call, core.function-parameter, core.function-return, core.variable, core.assignment, core.print-variable, core.arithmetic, core.fstring, core.boolean, ctrl.comparison, core.float, core.division]
concepts: [math.xe-banh-mi-cua-byte, math.o-trong, math.cai-may]
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
Hôm nay không có **đồ nghề** nào mới. Chỉ có cái xe, một buổi bán hàng khác, và
cả mạch để tả nó.
::::

::::explain{#mot-buoi-ban-hang}
Ba mươi lăm bài vừa rồi mỗi bài thêm đúng một thứ. Hôm nay không thêm gì cả —
chỉ cầm hết chỗ đồ nghề ấy lên một lần.

Vẫn cái xe bánh mì ấy, nhưng **một buổi khác, con số khác**: thuê chỗ đắt hơn,
lời lãi tính khác, khách khác. Cố ý như vậy. Chép đáp án của bài 17 hay bài 29
sang đây là trật hết — thứ mang sang được chỉ có **cách làm**. Nên mấy con số
dưới đây phải đọc như đọc một đề hoàn toàn mới.

> Xe bánh mì của Byte. Một ổ bán **15 000 đồng**. Mỗi ngày trả **100 000 đồng**
> tiền thuê chỗ.
>
> 1. Bán ít nhất **mấy ổ** thì hết lỗ?
> 2. Muốn lãi đúng **200 000** thì bán mấy ổ? Muốn lãi **350 000** thì mấy ổ?
> 3. Đang bán 15 nghìn một ổ, mỗi ngày được 40 ổ. Cứ tăng giá 1 nghìn thì bán
>    ít đi 2 ổ. **Tăng bao nhiêu thì thu được nhiều nhất?**
> 4. Thúng men để từ tối, mỗi giờ nở gấp đôi. **Bảy giờ sau được mấy phần?**

Bốn câu hỏi hỏi bằng tiếng Việt, không câu nào có sẵn một dấu `=` nào.

Trước khi tính, để ý cái khó nằm ở đâu — không nằm ở phép tính nào cả, mà ở chỗ
**phải dịch câu hỏi thành câu tính trước đã**. "Hết lỗ" là `≥ 0`, không phải
`= 0`. "Muốn lãi đúng 200 000" là chạy ngược một cái máy, không phải bấm máy
xuôi. "Thu được nhiều nhất" là hỏi cái đỉnh của một đường cong. "Gấp đôi mỗi
giờ" là phép nhân lặp, không phải phép cộng lặp.

Và một chỗ nữa: **số ổ bánh mì là thứ đếm được**, không bẻ nửa được — đúng cái
đơn vị rời mà mạch trước dựng từ bài 4. Câu 1 sẽ đâm thẳng vào chỗ ấy.

Nên ta đi từng chặng, và mỗi chặng chốt lại bằng một dòng cho máy làm trọng tài.
Máy không hiểu cái xe bánh mì — nó chỉ trả lời `True` hay `False` cho câu **bạn**
phát biểu ra.
::::

::::example{#di-het-mot-luot}
Chín chặng, theo đúng thứ tự mà cả mạch đã đi.

```python title=readonly
# 1 — Biểu thức: câu tính còn ô trống (bài 3–5).
#     Hai cái máy của quán, viết lại một lần cho đủ bộ.
def thu(n):
    return 15000 * n

def lai(t):
    return t - 100000

print(thu(20))
print(lai(thu(20)))

# 2 — Rút cái chung ra ngoài (bài 7–9).
#     15000n − 100000 và 5000 × (3n − 20) là HAI hình dạng, MỘT bảng.
print(15000 * 8 - 100000 == 5000 * (3 * 8 - 20))
print(15000 * 25 - 100000 == 5000 * (3 * 25 - 20))

# 3 — Phương trình, và thử lại ở CÂU GỐC (bài 10, 15, 16).
#     "Lãi đúng 200 000" → 15000n − 100000 = 200000 → n = 20.
print(lai(thu(20)) == 200000)

# 4 — Bất phương trình, và cái đơn vị rời (bài 20, 21; mạch trước bài 4).
#     "Hết lỗ" là ≥ 0 → n ≥ 100000 : 15000 → n ≥ 6,67 → mà ổ thì không bẻ nửa.
print(100000 / 15000)
print(lai(thu(6)))
print(lai(thu(7)))

# 5 — Hai con số tả trọn một đường thẳng (bài 26, 27).
#     Dốc: thêm một ổ thì lãi thêm bao nhiêu. Chỗ bắt đầu: lãi lúc bán 0 ổ.
print(lai(thu(1)) - lai(thu(0)))
print(lai(thu(0)))

# 6 — Đường cong, và hai bên cái gương (bài 28, 29).
#     Tăng giá x nghìn: giá thành (15 + x), bán được (40 − 2x) ổ.
print((15 + 2) * (40 - 2 * 2))
print((15 + 3) * (40 - 2 * 3))

# 7 — Mỗi bước NHÂN với cùng một số (bài 30).
print(2 ** 7)

# 8 — Chạy ngược cái máy (bài 32): muốn lãi 350 000 thì bán mấy ổ?
print((350000 + 100000) / 15000)

# 9 — Nối máy, và thứ tự nối (bài 34, 35).
print(lai(thu(20)))
print(thu(lai(20)))
```

Máy in ra:

```text
300000
200000
True
True
True
6.666666666666667
-10000
5000
15000
-100000
612
612
128
30.0
200000
-1499700000
```

Đọc lại theo lời:

- **Chặng 2** — hai câu tính khác hình dạng mà trùng kết quả ở cả `n = 8` lẫn
  `n = 25`. Đó là **biểu thức tương đương** của bài 6, và luật cho phép đổi hình
  dạng là luật phân phối của bài 7.
- **Chặng 4** — con số 6,67 không phải một câu trả lời. Không ai bán 6,67 ổ bánh
  mì. Nó chỉ nói *ranh giới nằm ở giữa 6 và 7*, và hai dòng sau đọc thẳng ranh
  giới ấy ra: bán 6 ổ còn **lỗ 10 000**, bán 7 ổ đã **lãi 5 000**. Đáp án là
  **7 ổ** — và nó là 7 vì cái ổ bánh mì, không phải vì phép chia.
- **Chặng 5** — `15000` và `−100000`. Hai con số ấy tả trọn cả đường lãi: dốc
  15 000 một ổ, và cắt trục dọc ở −100 000 (chưa bán ổ nào thì đã nợ tiền thuê
  chỗ).
- **Chặng 6** — **612 và 612**. Tăng 2 nghìn hay tăng 3 nghìn đều thu về đúng
  612 nghìn. Hai đầu vào khác nhau, một đầu ra: đó là hai bên tấm gương của bài
  29, và cái đỉnh nằm ở chính giữa chúng.
- **Chặng 9** — `lai(thu(20))` ra 200 000, còn `thu(lai(20))` ra một con số âm
  khổng lồ. Cùng hai cái máy, đổi thứ tự nối thì hỏng hẳn: dòng thứ hai đang trừ
  một trăm nghìn khỏi **hai mươi cái ổ bánh mì**.

Để ý con số 20 trong buổi này xuất hiện hai lần mà là hai thứ khác hẳn nhau: 20
**ổ** ở chặng 3, và số 20 trong `3n − 20` ở chặng 2 — chỗ ấy nó chẳng đếm cái gì
cả, nó chỉ là phần còn lại sau khi rút 5 000 ra ngoài.
::::

::::predict{#doan-thu-tu-noi commitOnce}
Trước khi tự viết, kiểm lại chặng 9 — chặng mà đổi thứ tự là đổi cả câu chuyện.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def thu(n):
    return 15000 * n

def lai(t):
    return t - 100000

print(lai(thu(6)))
print(thu(lai(6)))
```

:::opt{correct}
-10000 rồi -1499910000
:::

:::opt
-10000 rồi -10000
::why
Gần đúng ở chỗ bạn dùng một luật rất thật: đổi chỗ thì kết quả không đổi. Với
hai con số dưới một phép nhân hay một phép cộng thì nó đúng tuyệt đối — mạch
trước đã dựng nó bằng cái mảng chữ nhật xoay một góc.

Ranh giới đã được nói thẳng ở bài 35: luật ấy nói về hai **con số** đứng hai bên
**một** phép, không nói về hai **cái máy** nối đuôi nhau. Ở đây máy `thu` nhân
lên, máy `lai` bớt đi một lượng cố định — nhân sau khi bớt thì cái lượng bớt ấy
cũng bị nhân theo. Đó chính là lý do hai dòng lệch nhau xa đến thế.
::
:::

:::opt
-10000 rồi máy báo lỗi
::why
Gần đúng ở chỗ bạn đọc ra được điều quan trọng nhất của dòng thứ hai: nó **vô
nghĩa**. Trừ một trăm nghìn đồng khỏi sáu cái ổ bánh mì là một câu không tả nổi
việc gì ngoài đời, và người nào thấy chỗ đó là đang giữ đúng thói quen mà mạch
trước dựng từ bài 1 — luôn hỏi *"mấy cái gì"*.

Ranh giới: máy **không** gác đơn vị. Nó chỉ thấy con số 6, trừ đi 100 000, rồi
nhân với 15 000, không thắc mắc gì cả. Người gác đơn vị là bạn, không phải nó.
Con số khổng lồ hiện ra chính là cách duy nhất máy "kêu" — và biết đọc tiếng kêu
ấy là việc của người viết.
::
:::

:::opt
90000 rồi -1499910000
::why
Gần đúng ở chỗ dòng thứ hai bạn tính trọn vẹn, không sai một bước — và đó là
dòng khó hơn trong hai dòng.

Ranh giới nằm ở dòng thứ nhất, chỗ dừng hơi sớm. `thu(6)` ra 90 000 thật, nhưng
90 000 là con số đang **nằm trong ống**: nó mới ra khỏi máy đầu. Cặp ngoặc ngoài
còn máy `lai` đang há miệng chờ, và nó bớt tiếp 100 000 tiền thuê chỗ.
::
:::
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Bốn chỗ trống, bốn câu hỏi của buổi sáng. Không chỗ nào chép được của chỗ kia.
::::

::::code{#bon-cau-hoi-cua-buoi-sang}
Bốn chỗ trống, bốn chặng khác nhau của cùng một buổi:

1. `so_o_can_ban` — **chạy ngược cái máy**: muốn lãi đúng 200 000 thì bán mấy ổ?
   Gỡ tiền thuê chỗ trước, gỡ giá một ổ sau — đúng thứ tự gỡ của bài 15.
2. `lai_ban_6` và `lai_ban_7` — **ranh giới hết lỗ**: nối hai máy rồi hỏi hai số
   ổ kề nhau. Hai con số đi ra phải nằm hai bên số 0, và đó là cách đọc ra đáp
   án "7 ổ".
3. `men_sau_7_gio` — **mỗi giờ nhân đôi**, không phải mỗi giờ cộng thêm.

Đừng gõ thẳng con số đã đọc được ở trên — hãy viết ra phép tính, để dòng ấy còn
đúng cả khi mai Byte đổi giá hay đổi chỗ ngồi.

```python title=starter
def thu(n):
    return 15000 * n

def lai(t):
    return t - 100000

# 1 — Muốn lãi đúng 200 000 thì bán mấy ổ? Chạy NGƯỢC cái máy.
so_o_can_ban = ___

# 2 — Bán 6 ổ, rồi bán 7 ổ. Nối hai máy và hỏi cả hai lần.
lai_ban_6 = ___
lai_ban_7 = ___

# 3 — Thúng men: 1 phần lúc 0 giờ, mỗi giờ NHÂN ĐÔI. Sau 7 giờ được mấy phần?
men_sau_7_gio = ___

print(so_o_can_ban)
print(lai_ban_6)
print(lai_ban_7)
print(men_sau_7_gio)
```

```python title=solution
def thu(n):
    return 15000 * n

def lai(t):
    return t - 100000

# 1 — Muốn lãi đúng 200 000 thì bán mấy ổ? Chạy NGƯỢC cái máy.
so_o_can_ban = (200000 + 100000) / 15000

# 2 — Bán 6 ổ, rồi bán 7 ổ. Nối hai máy và hỏi cả hai lần.
lai_ban_6 = lai(thu(6))
lai_ban_7 = lai(thu(7))

# 3 — Thúng men: 1 phần lúc 0 giờ, mỗi giờ NHÂN ĐÔI. Sau 7 giờ được mấy phần?
men_sau_7_gio = 2 ** 7

print(so_o_can_ban)
print(lai_ban_6)
print(lai_ban_7)
print(men_sau_7_gio)
```

```python title=test
# Ba câu `!=` canh ba cái bẫy mà bài đã nói thẳng ra, và chúng đứng TRƯỚC vì
# chương trình dừng ở câu vỡ đầu tiên; xếp sau các câu `==` bao trùm chúng thì
# chúng không bao giờ chạy tới.
assert so_o_can_ban != 200000 / 15000, "quên gỡ tiền thuê chỗ: muốn LÃI 200 000 thì phải THU 300 000, vì 100 000 đã đi tiền thuê chỗ rồi"
assert lai_ban_6 != 90000, "90 000 là tiền THU khi bán 6 ổ — con số ấy còn nằm trong ống, chưa qua máy trừ tiền thuê chỗ"
assert men_sau_7_gio != 14, "14 là 2 × 7 — men NHÂN đôi mỗi giờ chứ không CỘNG thêm 2 mỗi giờ"
# Bốn chặng khác nhau, nên một con số gõ cứng chỉ qua được nhiều nhất một câu.
assert so_o_can_ban == 20, "thu phải là 300 000, chia cho 15 000 một ổ thì ra 20 ổ"
assert lai_ban_6 == -10000, "bán 6 ổ thu 90 000, trả 100 000 tiền thuê chỗ — còn thiếu 10 000"
assert lai_ban_7 == 5000, "bán 7 ổ thu 105 000 — vừa đủ trả tiền thuê chỗ và dư 5 000"
assert men_sau_7_gio == 128, "1 phần nhân đôi bảy lần: 2, 4, 8, 16, 32, 64, 128"
# Ranh giới hết lỗ nằm GIỮA hai con số này — đó là cách đọc ra đáp án "7 ổ".
assert lai_ban_6 < 0, "bán 6 ổ vẫn còn lỗ, nên số này phải âm"
assert lai_ban_7 > 0, "bán 7 ổ đã hết lỗ, nên số này phải dương"
```

:::hints
- kind: attention
  body: Chỗ trống 1 và chỗ trống 3 không gọi máy nào cả — chúng chỉ dùng những con số nằm trong chính dòng chú thích phía trên. Hai chỗ trống ở giữa thì ngược lại: cả hai đều phải cho số ổ đi qua CẢ HAI máy đã có sẵn ở đầu khung.
- kind: strategy
  body: Muốn lãi 200 000 thì tiền thu phải bù thêm cả tiền thuê chỗ, rồi mới chia cho giá một ổ — nhớ rào cái tổng lại bằng một cặp ngoặc, không thì phép chia chỉ ăn vào con số cuối. Hai chỗ trống giữa nối máy theo đúng thứ tự bài 34: máy nhận số ổ nằm bên trong. Còn men thì nhân đôi bảy lần, mà nhân lặp lại thì mạch trước đã có cách viết gọn.
- kind: one-line
  body: "Bốn chỗ trống là `(200000 + 100000) / 15000`, `lai(thu(6))`, `lai(thu(7))` và `2 ** 7`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: bốn chỗ trống phải là bốn phép tính viết ra thật — gõ thẳng 20, 5000 hay 128 vào thì bài không kiểm được chặng nào cả
  requireAst:
  # Khung khởi đầu không GỌI máy nào (hai dòng `def` là định nghĩa, không phải
  # lệnh gọi) và không có dấu `/` hay `**` nào, nên bốn luật này chặn được cả
  # đáp án điền bừa lẫn đáp án chép cứng bốn con số.
  - kind: uses-call, target: thu, min: 2
  - kind: uses-call, target: lai, min: 2
  - kind: uses-operator, target: /, min: 1
  - kind: uses-operator, target: **, min: 1
  forbidAst:
  # Lưới thứ hai, chặn đúng bốn con số KẾT QUẢ. Lời giải thật chỉ chứa 200000,
  # 100000, 15000, 6, 7 và 2, nên luật này không cản ai làm thật. `10000` chặn
  # cả `-10000`: trong cây cú pháp Python dấu trừ đứng riêng, hằng vẫn là 10000.
  - kind: has-literal, target: 20
  - kind: has-literal, target: 20.0
  - kind: has-literal, target: 10000
  - kind: has-literal, target: 5000
  - kind: has-literal, target: 128
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^20\.0\n-10000\n5000\n128\s*$
:::
::::

::::explain{#con-cau-hoi-thu-ba}
Ba câu hỏi của buổi sáng đã xong. Còn câu thứ ba, và nó là câu khác loại hẳn.

Câu 1 và câu 2 chạy trên một **đường thẳng**: mỗi ổ thêm vào thì lãi thêm đúng
15 000, không hơn không kém. Câu 4 thì đã rời đường thẳng rồi, nhưng rời theo lối
riêng của nó — **nhân đôi mỗi giờ**, đúng cái máy `2ⁿ` mà bài 30 dựng và bài 31
chỉ ra là không đường thẳng nào đuổi kịp. Còn câu thứ ba rời đường thẳng theo lối
thứ ba nữa, vì tăng giá làm **hai việc cùng lúc và ngược chiều nhau**:

- giá mỗi ổ **cao lên** — tốt;
- số ổ bán được **ít đi** — xấu.

Gọi `x` là số nghìn tăng thêm so với giá đang bán. Giá thành `15 + x` nghìn, và
số ổ còn `40 − 2x`. Tiền thu là tích của hai thứ ấy:

```text
   thu(x) = (15 + x) × (40 − 2x)     (đơn vị: nghìn đồng)
```

Đây là chỗ hai cụm cùng chứa `x` nhân với nhau — và mở ngoặc ra thì có `x × x`.
Đúng cái làm bảng tăng **không đều** và đồ thị **cong** của bài 28.

Bảng của nó, theo bài 5, ghi được như thế này:

```text
     x │  giá │  số ổ │  thu (nghìn)
───────┼──────┼───────┼──────────────
    −1 │   14 │    42 │          588
     0 │   15 │    40 │          600
     1 │   16 │    38 │          608
     2 │   17 │    36 │          612
     3 │   18 │    34 │          612
     4 │   19 │    32 │          608
     5 │   20 │    30 │          600
     6 │   21 │    28 │          588
```

Cột cuối đi lên rồi đi xuống, và nó **đối xứng**: `−1` với `6` cùng ra 588, `0`
với `5` cùng ra 600, `1` với `4` cùng ra 608, `2` với `3` cùng ra 612. Có một
tấm gương dựng ở chính giữa 2 và 3, và **cái đỉnh nằm trên tấm gương ấy**.

Chú ý `x = −1` vẫn có nghĩa: hạ giá 1 nghìn, bán 14 nghìn một ổ, được 42 ổ. Ô
trống này nhận cả số âm — nó không phải một cái sân, nó là một mức tăng giá.

Việc còn lại chỉ là chốt cái đối xứng ấy bằng một dòng: hai mức tăng khác nhau,
xem thu về có chênh nhau đồng nào không.
::::

::::assemble{#hai-ben-tam-guong}
Hai mức tăng giá — thêm 2 nghìn và thêm 3 nghìn — dựng lại từ đúng những con số
của đề bài, rồi so.

Trong hai chỗ trống này, **không được viết con số nào là kết quả**. Mỗi dòng
phải dựng từ những cái tên đã đặt ở trên, để dòng ấy còn đọc được là *"giá gốc
cộng phần tăng, nhân với số ổ đã bớt đi hai lần phần tăng"*.

```python title=starter
gia_goc = 15
o_ban_duoc = 40
tang_2 = 2
tang_3 = 3

# Tăng thêm `tang_2` nghìn: giá cao lên bấy nhiêu, số ổ bớt đi 2 lần bấy nhiêu.
doanh_thu_tang_2 = (gia_goc + tang_2) * (o_ban_duoc - 2 * ___)
# Tăng thêm `tang_3` nghìn.
doanh_thu_tang_3 = (gia_goc + ___) * (o_ban_duoc - 2 * tang_3)

chenh = doanh_thu_tang_2 - doanh_thu_tang_3

print(doanh_thu_tang_2)
print(doanh_thu_tang_3)
print(f"Hai mức tăng giá, chênh nhau {chenh} nghìn")
```

```python title=solution
gia_goc = 15
o_ban_duoc = 40
tang_2 = 2
tang_3 = 3

# Tăng thêm `tang_2` nghìn: giá cao lên bấy nhiêu, số ổ bớt đi 2 lần bấy nhiêu.
doanh_thu_tang_2 = (gia_goc + tang_2) * (o_ban_duoc - 2 * tang_2)
# Tăng thêm `tang_3` nghìn.
doanh_thu_tang_3 = (gia_goc + tang_3) * (o_ban_duoc - 2 * tang_3)

chenh = doanh_thu_tang_2 - doanh_thu_tang_3

print(doanh_thu_tang_2)
print(doanh_thu_tang_3)
print(f"Hai mức tăng giá, chênh nhau {chenh} nghìn")
```

```python title=test
# Hai câu `!=` canh hai cách hiểu sai "tăng giá", và chúng đứng TRƯỚC vì câu
# `==` phía dưới bao trùm chúng — xếp sau thì chúng không bao giờ chạy tới.
assert doanh_thu_tang_2 != 600, "600 là thu khi KHÔNG tăng giá: 15 × 40. Tăng giá thì cả giá lẫn số ổ đều đổi"
assert doanh_thu_tang_2 != 680, "680 là 17 × 40 — tăng giá mà quên rằng số ổ bán được ít đi"
assert doanh_thu_tang_2 == 612, "giá 17 nghìn, bán được 36 ổ"
assert doanh_thu_tang_3 == 612, "giá 18 nghìn, bán được 34 ổ"
# Chỗ này mới là cái đáng nhớ của cả chặng.
assert chenh == 0, "hai mức tăng giá KHÁC nhau cho đúng MỘT khoản thu — chúng là hai bên của tấm gương, và đỉnh nằm giữa chúng"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở hai dòng khác nhau, và mỗi dòng đã tự nói tên phần tăng của nó ngay trong chú thích phía trên. Nhìn xem trong cùng một dòng, cái tên ấy đã xuất hiện ở vế nào rồi và còn thiếu ở vế nào.
- kind: strategy
  body: Trong một dòng, phần tăng giá phải xuất hiện HAI lần: một lần cộng vào giá, một lần nhân với 2 rồi trừ khỏi số ổ. Cùng một mức tăng thì cùng một cái tên ở cả hai chỗ — đó chính là luật "mọi chỗ mang cùng một chữ là cùng một ô" của bài 2.
- kind: one-line
  body: "Chỗ trống thứ nhất là `tang_2`, chỗ trống thứ hai là `tang_3`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: mỗi dòng phải nhắc lại đúng cái tên phần tăng của chính nó ở CẢ HAI vế — điền một con số vào là ô trống ấy không còn là một ô nữa
  requireAst:
  # Khung khởi đầu đọc `tang_2` đúng một lần và `tang_3` đúng một lần, nên
  # `min: 2` là ranh giới phân biệt được lời giải với đáp án điền bừa.
  - kind: uses-name, target: tang_2, min: 2
  - kind: uses-name, target: tang_3, min: 2
  forbidAst:
  # Chặn hai con số KẾT QUẢ. Lời giải chỉ chứa 15, 40, 2, 3 nên không vướng.
  - kind: has-literal, target: 612
  - kind: has-literal, target: 36
  - kind: has-literal, target: 34
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^612\n612\nHai mức tăng giá, chênh nhau 0 nghìn\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảy ổ thì hết lỗ, hai mươi ổ thì lãi hai trăm nghìn, tăng hai hay ba nghìn cũng thu về như nhau. Cả buổi sáng nằm gọn trong mấy dòng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang mạch sau — và nó là câu bỏ ngỏ lâu nhất của cả mạch
này.

Nhìn lại chặng 2 hôm nay:

```text
15000n − 100000   và   5000 × (3n − 20)
```

Bạn thử hai số, `n = 8` và `n = 25`. Máy trả `True` cả hai lần. Bạn tin chúng
bằng nhau.

Nhưng "bằng nhau" ở bài 6 nghĩa là bằng nhau ở **mọi** giá trị điền vào — mà bạn
mới thử hai. Thử thêm một trăm số nữa cũng chỉ là một trăm lẻ hai. Giữa "đúng ở
mọi số tôi đã thử" và "đúng ở mọi số" còn một khoảng mà không cái bảng nào bắc
qua được.

Bài 19 đẩy chỗ hở ấy tới tận cùng: có những câu mà **mọi số đều là nghiệm**. Lúc
đó bạn tin nó vì đã thử vài số — hay vì bạn **chứng minh** được?

Đấy là câu hỏi mà mạch sau nhận: **T2.3 — Logic & chứng minh**. Nó hỏi bằng cách
nào một câu được biết là đúng, chứ không chỉ được thấy là đúng.

Còn hai món nữa bạn đang cầm mà chưa dùng hết, và chúng đi xa hơn Realm 2:

- **Cái ống nối máy** của bài 34 — nối hai mảnh mà không cần mở mảnh nào ra. Ở
  Realm 4, đúng phép nối ấy là cách người ta viết cả một chương trình: những
  mảnh nhỏ đóng kín, chỉ ráp miệng với nhau.
- **Cái máy nhân đôi** của bài 30 — 128 phần sau bảy giờ, và bài 31 cho thấy nó
  cuối cùng vượt mọi đường thẳng, dù đường ấy dốc tới đâu. Ở Realm 4, bạn sẽ gặp
  lại nó dưới một cái tên khác hẳn: một chương trình phải chạy `2ⁿ` bước. Lúc ấy
  "cuối cùng luôn vượt" không còn là một quan sát đẹp trên đồ thị nữa — nó là lý
  do một chương trình chạy xong trong một giây, hay không bao giờ chạy xong.

Cất cái xe bánh mì đi. Đồ nghề thì mang theo.
::::

::::checkpoint{mastery=0.85}
::::
