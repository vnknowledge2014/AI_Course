---
id: toan.cam-nhan-so.boss-vuon-cua-byte
title: BOSS — Vườn của Byte
summary: Một câu hỏi duy nhất về cái vườn, và để trả lời nó phải đi lại gần như cả track — chặng nào cũng chốt bằng một dòng cho máy làm trọng tài.
locale: vi
track: toan
module: cam-nhan-so
order: 44
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [math.multiplication, math.multiply-distributive, math.zero-placeholder, math.additive-inverse, math.remainder, math.equivalent-fraction, math.terminating-decimal, math.percent-whole, math.division-quotative, math.unit-rate, math.order-of-operations, math.parentheses]
requires: [math.parentheses, math.order-of-operations, math.unit-rate, math.percent-whole, math.terminating-decimal, math.equivalent-fraction, math.division-quotative, math.remainder, math.additive-inverse, math.multiply-distributive, math.zero-placeholder, math.thuoc-do, core.arithmetic, core.division, core.float, core.variable, core.assignment, core.print-variable, core.fstring, core.number-literal, core.boolean, ctrl.comparison, core.output]
concepts: [math.vuon-cua-byte, math.don-vi, math.ky-hieu]
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
Hôm nay không có gì mới. Chỉ có cái vườn, một câu hỏi, và cả track để trả lời.
::::

::::explain{#mot-cau-hoi-duy-nhat}
Bốn mươi ba bài vừa rồi mỗi bài thêm đúng một thứ. Hôm nay không thêm gì cả — chỉ
cầm hết chỗ đồ nghề ấy lên một lần, trên đúng cái vườn đã chạy suốt track.

> Vườn của Byte có **6 luống**, mỗi luống gieo **15 hạt**. Đợt rét làm **20%** số
> hạt gieo không nảy. Số cây sống được đánh ra một bờ dài **18 mét**, chia thành
> các ô rộng **3/4 mét**.
>
> **Mỗi ô được mấy cây?**

Một câu hỏi. Một con số đi ra.

Trước khi tính, để ý cái vườn này khó ở đâu — không phải ở phép tính nào cả, mà ở
chỗ **các con số không cùng loại**. 6 là số *luống*. 15 là số *hạt*. 20 là *phần
trăm*, mà phần trăm thì luôn phải hỏi lại *của cái gì* (bài 39). 18 là số *mét*.
3/4 cũng là *mét*, nhưng nó đóng vai một **cái thước**, không phải một lượng đất
(bài 3, bài 27).

Bài 1 nói mọi con số đều là câu trả lời cho "mấy **cái** gì". Cả bài toán này
sống chết ở chỗ ấy: sai một đơn vị là hỏng cả con số cuối, dù mọi phép tính đều
đúng.

Nên ta đi từng chặng, và mỗi chặng chốt lại bằng một dòng cho máy làm trọng tài.
Máy không hiểu cái vườn — nó chỉ trả lời `True` hay `False` cho câu **bạn** phát
biểu ra.
::::

::::example{#di-het-mot-luot}
Chín chặng, theo đúng thứ tự mà bài toán bắt phải đi.

```python title=readonly
# 1 — Đơn vị và cái thước (bài 1, 3).
#     Bờ ấy dài 18 mét, mà cũng là 9 sải dây, mỗi sải 2 mét.
#     Đổi thước thì con số đổi, lượng đất thì không.
print(9 * 2 == 18)

# 2 — Mảng chữ nhật, và cắt mảng cho dễ đếm (bài 19, 20, 21).
#     6 luống × 15 hạt. Cắt 15 thành 10 với 5 thì nhẩm được bằng miệng.
print(6 * 15)
print(6 * 10 + 6 * 5 == 6 * 15)

# 3 — Cột và số 0 giữ chỗ (bài 6, 7, 8).
#     90 hạt là 9 bó chẵn — cột hạt lẻ rỗng, và `0` đứng giữ chỗ cho nó.
print(9 * 10 + 0 == 90)

# 4 — Phần trăm CỦA CÁI GÌ (bài 38, 39).
#     20% là cái thước "một phần trăm", đặt 20 lần — và ở đây nó đo
#     số hạt GIEO, không đo số cây sống.
print(20 / 100 == 1 / 5)
print(90 * 20 / 100)

# 5 — Trừ là cộng với số đối (bài 14, 16, 18).
print(90 - 18 == 90 + (-18))
print(90 - 18)

# 6 — Đóng bó và phần còn thừa (bài 6, 29).
#     72 cây là 7 bó mười và 2 cây lẻ; phần thừa nhỏ hơn cỡ bó.
print(7 * 10 + 2 == 72)

# 7 — Phân số tương đương và thập phân (bài 33, 37).
#     Ô rộng 3/4 mét, cũng đúng là 6/8 mét, cũng đúng là 0.75 mét.
print(3 / 4 == 6 / 8)
print(3 / 4)

# 8 — Chia ĐO: cái thước 3/4 mét đặt lên bờ 18 mét được mấy lần (bài 27, 43).
print(18 / (3 / 4))

# 9 — Quy về một đơn vị (bài 40, 41): mấy cây TRÊN MỘT ô.
print(72 / 24)
```

Máy in ra:

```text
True
90
True
True
True
18.0
True
72
True
True
0.75
24.0
3.0
```

Đọc lại theo lời:

- **90 hạt** được gieo. Đó là cái mảng 6 × 15, và nó cắt được thành 60 với 30.
- **18 hạt** không nảy — một phần năm của 90, vì 20/100 rút gọn về 1/5.
- **72 cây** sống. Ghi theo bó thì là 7 bó và 2 cây lẻ.
- Bờ 18 mét chứa được **24 ô** rộng 3/4 mét. Đây là chia **đo**, không phải chia
  đều: ta không bổ bờ thành mấy phần, ta đặt một cái thước lên nó và đếm số lần
  đặt được.
- **3 cây một ô** — 72 cây trên 24 ô, quy về một đơn vị.

Để ý con số 18 trong bài này xuất hiện hai lần mà là hai thứ khác hẳn nhau: 18
**hạt** không nảy, và bờ dài 18 **mét**. Cùng một chữ số, hai câu trả lời cho hai
câu hỏi *mấy cái gì* khác nhau — đúng chỗ bài 1 dặn.

Chặng 8 là chặng dễ hỏng nhất, và hỏng vì đúng thứ bài trước vừa nói: cái gạch
phân số của `3/4` là một cặp ngoặc ẩn. Viết `18 / 3 / 4` thì máy chia hai lần
liên tiếp, ra 1.5 — và cái ô 3/4 mét tan mất trước khi kịp thành một cái thước.
::::

::::predict{#doan-ba-dong commitOnce}
Trước khi tự viết, kiểm lại chặng 4 và chặng 5 — hai chặng mà một dấu nhân với
một dấu chia đứng chung dòng với một dấu trừ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
hat_gieo = 6 * 15
print(hat_gieo)
print(hat_gieo * 20 / 100)
print(hat_gieo - hat_gieo * 20 / 100)
```

:::opt{correct}
90, 18.0 rồi 72.0
:::

:::opt
90, 18.0 rồi 0.0
::why
Gần đúng ở chỗ bạn đọc từ trái sang phải, tới đâu tính tới đó. Cách đọc ấy đúng
với chữ viết, và đi từ trái sang phải thì không bao giờ sai với một dòng chỉ toàn
cộng với trừ — và bài 18 nói vì sao: đổi mọi phép trừ thành cộng số đối thì cả
dòng chỉ còn phép cộng, mà những đống của bài 10 thì gộp theo thứ tự nào cũng ra
một kết quả. Chưa đổi như thế thì vẫn phải đi từ trái sang phải: `10 − 3 + 2` ra
9, chứ không phải `10 − (3 + 2)` ra 5.

Ranh giới: dòng 3 có một dấu nhân và một dấu chia xen vào, và ba con số bên phải
dấu trừ đã dính thành **một khối** — "hai mươi phần trăm của 90 hạt". Bài 42 nói
đúng chuyện này: khối phải gói xong mới đem gộp hay bớt được. Lôi `hat_gieo` ra
khỏi khối để trừ trước là làm tan cái khối, và lúc ấy chẳng còn phần trăm của gì
nữa.
::
:::

:::opt
90, 18 rồi 72
::why
Gần đúng ở chỗ bạn giữ đúng bản chất của thứ đang đếm: hạt là **đơn vị rời** (bài
4), bẻ nhỏ là mất nghĩa. Không có 18,0 hạt nào khác 18 hạt cả, và nửa hạt thì
không phải hạt.

Ranh giới nằm ở chỗ cái đuôi `.0` ấy **không** nói rằng có nửa hạt. Nó nói con số
này vừa đi qua một **phép chia** — và phép chia thì luôn cho ra số có phần lẻ,
kể cả khi phần lẻ bằng không. Đó là điều Realm 0 đã dựng và bài 4 đã dùng lại.

Còn dòng 3: một khi phần lẻ đã vào trong dòng, nó ở lại. `90` trừ `18.0` cho ra
`72.0`, chứ không rơi ngược về số chẵn.
::
:::

:::opt
Máy báo lỗi ở dòng 3, vì `hat_gieo` xuất hiện hai lần trong cùng một dòng
::why
Gần đúng ở chỗ linh cảm ấy có gốc thật. Bài 13 đã dừng lại rất lâu ở dòng
`vi_tri = vi_tri + 1` — cùng một cái tên đứng hai bên, trông đúng là vòng tròn,
và trong vở toán thì câu ấy quả thật không số nào thoả.

Ranh giới: chỗ đáng ngờ là khi cái tên đứng ở **cả hai bên dấu `=`**. Dòng 3
không có dấu `=` nào cả. `hat_gieo` chỉ được **đọc**, hai lần — mà đọc thì không
làm gì tới nó. Máy lấy ra con số 90 ở lần thứ nhất, lấy ra đúng con số 90 ấy ở
lần thứ hai, rồi tính.
::
:::
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Ba chỗ trống, ba chặng. Hai chỗ hỏi số cây, một chỗ hỏi số ô.
::::

::::code{#hai-con-so-ban-le}
Cả bài toán bản lề ở hai con số: **số cây sống** và **số ô trên bờ**. Có hai con
số ấy thì chặng cuối chỉ còn một phép chia.

Ba chỗ trống, ba chặng khác nhau — và không chỗ nào chép được của chỗ kia:

1. `hat_chet` — **phần trăm của cái gì**: 20% ấy đo số hạt **gieo**, không đo số
   cây sống. Và nó đếm phần **không nảy**, chứ không phải phần còn nảy được —
   hai phần ấy ráp lại mới ra cả đống gieo.
2. `cay_song` — **bớt đi**: cả đống gieo bỏ phần không nảy.
3. `so_o` — **chia đo**: cái thước 3/4 mét đặt lên bờ 18 mét được mấy lần. Cái
   gạch của `3/4` là một cặp ngoặc ẩn, và ép xuống một dòng thì phải dựng nó lại
   bằng tay.

Đừng gõ thẳng con số đã đếm được — hãy viết ra phép tính, để dòng ấy còn đúng cả
khi vườn sang năm khác đi.

```python title=starter
# Cả vườn: 6 luống, mỗi luống 15 hạt.
hat_gieo = 6 * 15

# Đợt rét làm 20% SỐ HẠT GIEO không nảy.
hat_chet = ___
cay_song = ___

# Bờ dài 18 mét, mỗi ô rộng 3/4 mét. Đặt cái thước ấy lên bờ được mấy lần?
so_o = ___

print(cay_song)
print(so_o)
```

```python title=solution
# Cả vườn: 6 luống, mỗi luống 15 hạt.
hat_gieo = 6 * 15

# Đợt rét làm 20% SỐ HẠT GIEO không nảy.
hat_chet = hat_gieo * 20 / 100
cay_song = hat_gieo - hat_chet

# Bờ dài 18 mét, mỗi ô rộng 3/4 mét. Đặt cái thước ấy lên bờ được mấy lần?
so_o = 18 / (3 / 4)

print(cay_song)
print(so_o)
```

```python title=test
# Hai câu `!=` chốt hai cái bẫy mà bài đã nói thẳng ra, và chúng đứng TRƯỚC vì
# chương trình dừng ở câu vỡ đầu tiên — câu gọi tên một lỗi cụ thể mà xếp sau câu
# `==` bao trùm nó thì không bao giờ chạy tới.
assert hat_chet != 72, "72 là phần NẢY ĐƯỢC — 80% còn lại. Đề hỏi phần KHÔNG nảy: 20% của 90 hạt gieo"
assert so_o != 1.5, "thiếu cặp ngoặc thì `18 / 3 / 4` chia hai lần liên tiếp, và cái ô 3/4 mét tan mất"
# Bảy câu trên ba chặng khác nhau, nên một con số gõ cứng chỉ qua được nhiều
# nhất một câu.
assert hat_gieo == 90, "mảng 6 luống × 15 hạt là 90 hạt"
assert hat_chet == 18, "20% của 90 hạt gieo — một phần năm của 90"
assert cay_song == 72, "90 hạt gieo, bớt đi 18 hạt không nảy"
assert cay_song + hat_chet == hat_gieo, "phần chết và phần sống ráp lại phải đúng cả đống gieo ban đầu"
assert so_o == 24, "cái thước 3/4 mét đặt lên bờ 18 mét được 24 lần"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất và thứ hai đều nói về HẠT, và cả hai đều dựa vào cái tên `hat_gieo` ở dòng trên cùng. Chỗ trống thứ ba nói về MÉT, và hai con số của nó nằm ngay trong dòng chú thích phía trên nó.
- kind: strategy
  body: Phần trăm là cái thước "một phần trăm" đặt 20 lần lên số hạt gieo — nhân rồi chia cho một trăm. Số cây sống là số hạt gieo bớt đi phần vừa tính. Số ô là bờ 18 mét đem đo bằng cái thước 3/4 mét, mà muốn 3/4 còn là MỘT lượng thì phải rào nó lại bằng một cặp ngoặc.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `hat_gieo * 20 / 100`, `hat_gieo - hat_chet`, và `18 / (3 / 4)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: ba chỗ trống phải là ba phép tính viết ra thật — gõ thẳng 18, 72 hay 24 vào thì bài không kiểm được chặng nào cả
  requireAst:
  # Khung khởi đầu chỉ có sẵn đúng một dấu `*` (ở `6 * 15`) và không có dấu
  # chia hay dấu trừ nào, nên bộ luật này chặn được đáp án chép cứng ba con số.
  - kind: uses-operator, target: *, min: 2
  - kind: uses-operator, target: /, min: 3
  - kind: uses-operator, target: -, min: 1
  - kind: uses-name, target: hat_gieo, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^72\.0\n24\.0\s*$
:::
::::

::::explain{#gio-gop-lai-mot-dong}
Ba chặng vừa rồi mỗi chặng một cái tên. Cách viết ấy tốt cho người đọc, và trong
đời thực bạn nên giữ nó.

Nhưng còn một việc đáng làm đúng một lần, để thấy cả bài toán có **hình dạng**
gì: ép cả bốn chặng xuống **một dòng**.

Lúc ấy mọi cái tên biến mất, và thứ duy nhất còn giữ được cấu trúc là dấu ngoặc.
Đọc dòng ấy từ ngoài vào:

```text
( cả đống gieo  bớt  phần không nảy )   chia   ( bờ đo bằng cái thước 3/4 mét )
```

Cặp ngoặc bên trái là **số cây sống**. Cặp ngoặc bên phải là **số ô**. Phép chia
ở giữa là chặng quy về một đơn vị: mấy cây trên một ô.

Bên trong cặp ngoặc phải còn một cặp ngoặc nữa, quanh `3 / 4` — cái gạch phân số
đã mất khi ép xuống dòng, và đây là chỗ dựng nó lại. Gói từ trong ra ngoài: `3/4`
thành một lượng trước, rồi mới lấy 18 đo bằng lượng ấy.
::::

::::assemble{#ca-cai-vuon-trong-mot-dong}
Cả bài toán, một dòng. Hai chỗ trống nằm trong hai cặp ngoặc khác nhau.

Trong dòng này **không con số nào được là con số bạn đã tính ra ở bước trước**.
Chỉ được dùng đúng những con số của đề bài: 6, 15, 20, 100, 18, 3, 4.

```python title=starter
# 6 luống, mỗi luống 15 hạt. 20% số hạt gieo không nảy.
# Bờ dài 18 mét, mỗi ô rộng 3/4 mét. Mỗi ô được mấy cây?
cay_moi_o = (6 * 15 - ___) / (18 / ___)

print(f"Mỗi ô được {cay_moi_o} cây")
```

```python title=solution
# 6 luống, mỗi luống 15 hạt. 20% số hạt gieo không nảy.
# Bờ dài 18 mét, mỗi ô rộng 3/4 mét. Mỗi ô được mấy cây?
cay_moi_o = (6 * 15 - 6 * 15 * 20 / 100) / (18 / (3 / 4))

print(f"Mỗi ô được {cay_moi_o} cây")
```

```python title=test
# Cái bẫy của bài trước, đặt vào đúng chỗ nó hay sập: bỏ cặp ngoặc quanh 3/4.
# Câu này đứng trước hai câu `==` phía dưới, vì xếp sau thì nó không bao giờ chạy
# tới — `assert cay_moi_o == 3` đã vỡ trước và chương trình dừng ngay tại đó.
assert cay_moi_o != 48, "thiếu ngoặc thì `18 / 3 / 4` ra 1.5 mét, và mỗi ô bỗng được 48 cây"
assert cay_moi_o == 3, "72 cây sống chia cho 24 ô thì mỗi ô 3 cây"
# Ráp ngược lại: 24 ô, mỗi ô 3 cây, phải ra đúng số cây sống.
assert cay_moi_o * 24 == 72, "24 ô mỗi ô 3 cây thì ráp lại đúng 72 cây sống"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm sau dấu trừ, trong cặp ngoặc bên trái — nó là phần hạt KHÔNG NẢY. Chỗ trống thứ hai nằm sau dấu chia, trong cặp ngoặc bên phải — nó là chiều rộng của MỘT cái ô.
- kind: strategy
  body: Phần không nảy là 20% của cả đống gieo, mà cả đống gieo trong dòng này lại đang được viết ra bằng 6 nhân 15 — nên nó phải viết lại nguyên cụm ấy rồi mới nhân 20 chia 100. Chỗ trống thứ hai là ba phần tư mét; muốn nó còn là MỘT lượng chứ không thành hai lần chia liên tiếp thì phải rào nó lại.
- kind: one-line
  body: "Chỗ trống thứ nhất là `6 * 15 * 20 / 100`, chỗ trống thứ hai là `(3 / 4)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cả dòng phải viết lại từ những con số của đề bài — điền 18 hay 0.75 vào là bạn đã tính hộ máy mất một chặng
  requireAst:
  # Khung khởi đầu chỉ có một dấu `*` và hai dấu `/`. Lời giải có ba dấu `*`
  # và bốn dấu `/`, nên bộ luật này chặn cả đáp án điền số đã tính sẵn.
  - kind: uses-operator, target: *, min: 3
  - kind: uses-operator, target: /, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: Mỗi ô được 3.0 cây
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cây một ô. Cả cái vườn gói gọn trong một dòng, và không hạt nào đi lạc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang mạch sau.

Nhìn lại dòng vừa viết:

```text
(6 × 15 − 6 × 15 × 20 : 100) : (18 : (3/4))
```

Mọi con số trong đó đều là số **cụ thể**. Nó trả lời đúng một cái vườn — cái vườn
của mùa này, sáu luống, mười lăm hạt.

Nhưng sang năm Byte định trồng khác. Chưa biết mấy luống, chưa biết mỗi luống mấy
hạt — còn tuỳ trời và tuỳ số hạt giống để dành được. Byte vẫn muốn viết sẵn cái
dòng ấy ngay từ bây giờ, để mùa sau chỉ việc điền hai con số vào là ra đáp số.

Thử xem: cứ chỗ nào là số luống thì viết chữ **n**, chỗ nào là số hạt mỗi luống
thì viết chữ **h**.

```text
(n × h − n × h × 20 : 100) : (18 : (3/4))
```

Dòng ấy trông vẫn giống hệt dòng cũ, mà nó nói được nhiều hơn hẳn: nó đúng cho
**mọi** cái vườn, không riêng cái vườn mùa này.

Nhưng `n` với `h` là cái gì? Chúng không phải một lượng — chưa ai biết chúng bằng
bao nhiêu. Chúng cũng không phải đơn vị. Chúng là **tên của một con số chưa
biết**, và cả track này chưa có chỗ nào cho một thứ như thế.

Đặt tên cho một con số chưa biết thì viết ra sao, và tính toán với nó theo luật
gì? Mạch sau — **T2.2 — Đại số & hàm số** — nhận đúng câu hỏi này.
::::

::::checkpoint{mastery=0.85}
::::
