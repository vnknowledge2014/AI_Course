---
id: toan.cam-nhan-so.chia-deu-cho-may-phan
title: Chia đều cho mấy phần
summary: Chia đều là biết số phần rồi đi tìm cỡ mỗi phần — tức đi tìm con số còn thiếu trong một phép nhân đã biết trước tổng.
locale: vi
track: toan
module: cam-nhan-so
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.division-partitive]
requires: [math.multiplication, math.multiply-commutative, core.arithmetic, core.division, core.output, core.variable, ctrl.for-range, ctrl.if, ctrl.comparison]
concepts: [math.chia-deu, math.thua-so-con-thieu, math.so-do-dai]
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
Mình biết có mấy luống. Cái mình chưa biết là mỗi luống được bao nhiêu.
::::

::::explain{#phat-vong-quanh}
Bài trước để lại câu hỏi: mở một bó mười ra thành mười hạt rời — đó là **đem
một bó chia đều cho mười chỗ**. Vậy chia là gì?

Bắt đầu bằng đúng động tác Byte làm ngoài vườn, không bằng một công thức.

Byte có **12 hạt** trong lòng bàn tay và **3 luống** đã cuốc sẵn. Byte muốn ba
luống nhận bằng nhau, không luống nào hơn. Cách chắc ăn nhất là đi vòng quanh
và phát từng hạt một:

```text
vòng 1:  luống A ●    luống B ●    luống C ●        (còn 9 hạt)
vòng 2:  luống A ●●   luống B ●●   luống C ●●       (còn 6 hạt)
vòng 3:  luống A ●●●  luống B ●●●  luống C ●●●      (còn 3 hạt)
vòng 4:  luống A ●●●● luống B ●●●● luống C ●●●●     (còn 0 hạt)
```

Hết hạt. Mỗi luống được **4 hạt**.

Hai điều xảy ra ở đây, và cả hai đều đáng gọi tên:

- Byte **biết trước** có mấy phần: ba luống. Con số 3 đã có sẵn từ đầu.
- Byte **đi tìm** cỡ mỗi phần: mỗi luống mấy hạt. Con số 4 là thứ cuộc phát
  hạt vừa sinh ra.

Đó là **chia đều**. Trong sổ, người ta viết `12 : 3 = 4`, đọc là "mười hai chia
ba bằng bốn". Bài này không dạy bạn cách đặt tính chia — nó dạy phép chia **là
gì** trước đã, vì người thuộc cách đặt tính mà không biết mình đang tìm cái gì
sẽ đứng hình ngay lần đầu bài toán đổi hình dạng.
::::

::::explain{#vi-sao-bon-la-dung}
Vì sao 4 là con số đúng, chứ không phải "vì phát ra thì thấy thế"?

Nhìn lại cuộc phát hạt bằng **mảng chữ nhật** của bài 20. Mỗi vòng phát là một
hàng ba hạt; phát bốn vòng thì được một mảng **4 hàng × 3 cột**, và mảng ấy
chứa đúng 12 hạt, không thừa không thiếu:

```text
● ● ●
● ● ●
● ● ●
● ● ●
```

Xoay mảng ấy 90° — bài 20 nói xoay không thêm bớt hạt nào — thì nó thành **3
hàng × 4 cột**: ba luống, mỗi luống bốn hạt.

Nên câu "mỗi luống 4 hạt" nói cùng một điều với câu `3 × 4 = 12`. Và đó là chỗ
phép chia thật sự nằm:

> `12 : 3` là đi tìm **con số còn thiếu** trong `3 × ? = 12`.

Bài 19 nói phép nhân có hai vai: một con số nói **lô to bao nhiêu**, con số kia
nói **lấy mấy lô**. Chia đều là biết trước "lấy mấy lô" (3 luống) và đi tìm "lô
to bao nhiêu" (mỗi luống mấy hạt).

Bức tranh thứ ba, cho ai quen nhìn dải hơn nhìn hạt — **sơ đồ dải**. Một dải
dài 12 ô, gấp làm ba khúc bằng nhau:

```text
├────────────┼────────────┼────────────┤
      4            4            4
└──────────────── 12 ────────────────┘
```

Số khúc là thứ bạn quyết định trước khi gấp; độ dài mỗi khúc là thứ phép gấp
trả lại cho bạn.
::::

::::example{#hoi-thang-cai-may}
Máy có sẵn dấu chia: dấu gạch chéo `/` mà bạn đã gặp ở Realm 0.

```python title=readonly
so_hat = 12
so_luong = 3

print(so_hat / so_luong)
print(so_luong * 4)
```

Máy in ra:

```text
4.0
12
```

Dòng đầu là `4.0` chứ không phải `4`, và đó là chuyện Realm 0 đã chốt: dấu `/`
**luôn** cho ra một con số mang phần lẻ, kể cả khi chia khít. `4.0` và `4` là
cùng một chỗ trên thanh số của bài 5, chỉ khác cách máy ghi lại.

Dòng thứ hai mới là dòng đáng giá. Nó gộp ba luống về lại và ra đúng `12` — cái
đống ban đầu. Chia đều xong mà nhân ngược lại không về được chỗ cũ thì con số
tìm được là con số sai.
::::

::::predict{#doan-vong-thu commitOnce}
Thay vì phát hạt bằng tay, Byte nhờ máy **thử lần lượt từng cỡ**: mỗi luống 1
hạt thì ba luống hết 3 hạt — chưa hết đống; mỗi luống 2 hạt thì hết 6 — chưa
hết; cứ thế cho tới cỡ nào gộp lại vừa khít 12.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
tong_hat = 12
so_luong = 3

for co in range(1, 13):
    if so_luong * co == tong_hat:
        print(co)
```

:::opt{correct}
4
:::

:::opt
3
::why
Gần đúng ở chỗ bạn nhận ra 3 và 4 là một cặp đi liền nhau trong `3 × 4 = 12`,
và cặp ấy đúng — nắm được nó là nắm được cả bài.

Chỗ lệch nằm ở chỗ trong hai con số ấy, con nào là thứ **đã biết** và con nào
là thứ **đi tìm**. Số 3 đã nằm sẵn ở dòng `so_luong = 3` từ trước khi vòng thử
bắt đầu: nó là **số luống**, thứ Byte quyết định lúc cuốc đất. Dòng `print` in
ra `co` — cái đang được đem thử, tức **cỡ mỗi luống**.

Chia đều là đi tìm cái chưa biết, nên thứ được in ra phải là cái chưa biết.
::
:::

:::opt
Mười hai dòng, từ 1 tới 12
::why
Gần đúng ở chỗ bạn nhớ chính xác `for` chạy qua **mọi** giá trị trong dãy chứ
không dừng lại giữa chừng — `co` thật sự lần lượt mang cả mười hai giá trị từ 1
tới 12.

Chỗ lệch nằm ở chỗ `print` thụt vào **bên trong** `if`. Vòng lặp ghé qua đủ
mười hai lần, nhưng mỗi lần ghé nó lại hỏi "gộp lại có vừa khít 12 chưa"; chỉ
lần nào câu trả lời là đúng thì dòng `print` mới chạy.
::
:::

:::opt
Không in gì cả
::why
Gần đúng ở chỗ bạn theo dõi đúng mấy lần thử đầu tiên: `3 × 1 = 3` chưa tới 12,
`3 × 2 = 6` cũng chưa. Nếu cuộc thử dừng lại ở lần trượt đầu tiên thì thật sự
không có gì được in ra, và có những kiểu lặp làm đúng như vậy.

Chỗ lệch: `for ... in range(...)` không dừng khi trượt. Nó đi hết dãy, và trong
dãy 1…12 có đúng một cỡ làm phép nhân khít.
::
:::
::::

::::explain{#hai-vuon-hai-co}
Cỡ mỗi phần không phải một con số cố định của phép chia; nó **đổi theo số phần**.

Cùng 12 hạt ấy mà Byte cuốc 4 luống thì mỗi luống được 3 hạt; cuốc 6 luống thì
mỗi luống được 2. Đống hạt không đổi tí nào — thứ đổi là bạn cắt nó thành mấy
phần. Càng nhiều phần thì mỗi phần càng nhỏ.

Đó là lý do câu "chia làm số nhỏ đi" nghe thì xuôi tai nhưng chưa phải một luật:
nó chỉ mô tả một chuyện đang xảy ra, chứ chưa nói **vì sao**. Vì sao thì cái sơ
đồ dải đã nói: một dải dài chừng ấy mà gấp thành nhiều khúc hơn thì mỗi khúc
buộc phải ngắn lại.
::::

::::code{#chia-deu-hai-vuon}
Hai vườn, hai lần chia đều:

- **Vườn của Byte**: 12 hạt, chia đều cho **3** luống.
- **Vườn của An**: 20 hạt, chia đều cho **4** luống.

Máy thử lần lượt từng cỡ, y như ở phần đoán. Hai chỗ trống là **cái tổng phải
gộp về được** — con số nói cho máy biết lúc nào thì vừa khít.

Bài chấm bằng cả hai vườn, và hai vườn được chọn để cho ra **hai cỡ khác nhau**
(4 và 5): một con số gõ cứng vào cả hai chỗ trống thì nhiều nhất chỉ đúng được
một vườn.

```python title=starter
# Vườn của Byte: 12 hạt, chia đều cho 3 luống.
moi_luong_byte = 0
for co in range(1, 13):
    if 3 * co == ___:
        moi_luong_byte = co

# Vườn của An: 20 hạt, chia đều cho 4 luống.
moi_luong_an = 0
for co in range(1, 21):
    if 4 * co == ___:
        moi_luong_an = co

print(moi_luong_byte)
print(moi_luong_an)
```

```python title=solution
# Vườn của Byte: 12 hạt, chia đều cho 3 luống.
moi_luong_byte = 0
for co in range(1, 13):
    if 3 * co == 12:
        moi_luong_byte = co

# Vườn của An: 20 hạt, chia đều cho 4 luống.
moi_luong_an = 0
for co in range(1, 21):
    if 4 * co == 20:
        moi_luong_an = co

print(moi_luong_byte)
print(moi_luong_an)
```

```python title=test
# Hai vườn cho hai cỡ khác nhau, nên một con số điền bừa vào cả hai chỗ trống
# không thể qua nổi cả hai dòng đầu. Hai assert cuối khoá lại điều bài dạy:
# cỡ tìm được phải gộp ngược về đúng cái đống ban đầu.
assert moi_luong_byte == 4, "12 hạt vào 3 luống thì mỗi luống 4 hạt — không cỡ nào khác gộp lại vừa khít 12"
assert moi_luong_an == 5, "20 hạt vào 4 luống thì mỗi luống 5 hạt"
assert 3 * moi_luong_byte == 12, "gộp 3 luống về phải ra đúng đống hạt ban đầu, không thừa không thiếu"
assert 4 * moi_luong_an == 20, "gộp 4 luống về cũng phải ra đúng đống ban đầu"
assert moi_luong_byte != moi_luong_an, "hai vườn khác nhau thì cỡ mỗi luống khác nhau — nếu bằng nhau là có chỗ điền cứng"
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên phải dấu `==`, tức là ở vế mà phép nhân phải gộp về cho khít. Bên trái đã có số luống nhân với cỡ đang thử.
- kind: strategy
  body: Vòng thử dừng lại đúng lúc "số luống nhân cỡ" bằng cả đống hạt ban đầu. Mỗi vườn có một đống hạt riêng, và hai con số ấy đã ghi ngay trong dòng chú thích phía trên mỗi khối.
- kind: one-line
  body: "Chỗ trống thứ nhất là `12`, chỗ trống thứ hai là `20`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^4\n5\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn hạt một luống, năm hạt một luống. Gộp ngược lại là về đúng đống cũ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chiều nay An sang chơi, mang theo một cuộn dây **12 mét** và bảo: "Cắt hộ mình
thành từng đoạn **3 mét** để buộc giàn."

Byte cầm bút, định ghi vào sổ. Và Byte viết ra đúng cái dòng của bài này:

```text
12 : 3
```

Cùng hai con số, cùng một dấu chia, cùng một kết quả là 4. Nhưng thử đọc kỹ con
số **3** trong hai chuyện:

- "12 hạt chia cho **3** luống" — số 3 là **số phần**, và số 4 tìm được là cỡ
  mỗi phần: bốn hạt.
- "12 mét cắt thành từng đoạn **3** mét" — số 3 là **cỡ mỗi phần**, còn số 4
  tìm được lại là **số phần**: bốn đoạn dây.

Hai vai đổi chỗ cho nhau, mà dòng chữ trong sổ thì y hệt. Chuyện thứ hai không
phát vòng quanh được — làm sao phát dây cho những cái đoạn còn chưa tồn tại?

Vậy chuyện thứ hai là phép chia kiểu gì, và Byte đi tìm con số 4 ấy bằng động
tác nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
