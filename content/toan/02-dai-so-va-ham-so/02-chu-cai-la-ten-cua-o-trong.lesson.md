---
id: toan.dai-so-va-ham-so.chu-cai-la-ten-cua-o-trong
title: Chữ cái chỉ là cái tên dán lên ô trống
summary: Dán một chữ lên mỗi ô trống thì đọc được ngay ô nào là ô nào — và luật đi kèm là cùng một chữ thì phải điền cùng một số.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.letter-names-a-slot]
requires: [math.placeholder-many-values, math.multiplication, math.order-of-operations, core.variable, core.assignment, core.print-variable, core.number-literal, core.arithmetic, core.output]
concepts: [math.xe-banh-mi, math.o-trong, math.ky-hieu]
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
Hai cái ô trống trông giống hệt nhau. Mình phải dán nhãn thôi.
::::

::::explain{#hai-o-giong-het-nhau}
Tờ giấy mới của Byte có hai ô trống, một cho bánh mì, một cho nước:

```text
15 000 × ▢   +   8 000 × ▢
```

Câu hỏi để lại từ bài trước: hai ô này có buộc phải điền cùng một số không?

Không. Khách mua 3 ổ bánh mì và 1 chai nước là chuyện thường ngày, và tờ giấy
phải nhận được cả hai con số ấy cùng lúc. Hai ô trống này là **hai chỗ để dành
khác nhau**, mỗi chỗ chờ một con số riêng.

Nhưng nói được như thế là vì bạn đang nhìn thấy `15 000` và `8 000` đứng cạnh
chúng. Đưa tờ giấy cho người khác, hoặc để chính Byte đọc lại vào một buổi khác,
thì hai cái ô ấy trông giống hệt nhau — không cái nào tự nói ra nó là ô của cái
gì. Và khi dòng dài ra thì chuyện càng tệ:

```text
15 000 × ▢   +   8 000 × ▢   −   9 000 × ▢   −   ▢
```

Bốn ô trống trên một dòng. Ô nào là số ổ bán ra, ô nào là số chai, ô nào là số ổ
đã lấy nguyên liệu, ô nào là tiền thuê chỗ? Đếm từ trái sang mà nhớ thì được vài
lần rồi thôi.

Cách chữa cũng chính là cách người ta chữa mọi chuyện tương tự ngoài đời: **dán
nhãn**. Mỗi ô một cái tên riêng, và cái tên ngắn nhất có thể là một **chữ cái**.
::::

::::example{#dan-nhan-cho-tung-o}
Byte dán nhãn: chữ **b** cho ô số ổ bánh mì, chữ **c** cho ô số chai nước.

```text
   trước khi dán nhãn        15 000 × ▢   +   8 000 × ▢
   sau khi dán nhãn          15 000 × b   +   8 000 × c
```

Dòng dưới nói được nhiều hơn hẳn dòng trên, mà không thêm một con số nào. Giờ
đọc nó là biết ngay: `b` là số ổ, `c` là số chai.

Chữ `b` **không phải một con số**. Nó là cái nhãn dán lên một chỗ để dành — đúng
cái ô trống của bài trước, chỉ khác là giờ nó có tên. Mọi thứ đã nói về ô trống
vẫn nguyên vẹn: `b` nhận được nhiều số khác nhau, mỗi lần điền là một lần riêng,
và không có con số nào bị giấu sau chữ `b` cả.

Cái nhãn trả công ngay lập tức khi một chữ có mặt ở **nhiều chỗ**. Byte tính tiền
lãi: mỗi ổ bán 15 000, mỗi ổ tốn 9 000 tiền nguyên liệu.

```text
   tiền thu       15 000 × n
   tiền vốn        9 000 × n
   tiền lãi       15 000 × n  −  9 000 × n
```

Chữ `n` xuất hiện **hai lần** trên dòng cuối, và hai lần ấy là **cùng một ô**.
Byte bán bao nhiêu ổ thì cũng lấy nguyên liệu đúng bấy nhiêu ổ — không có cách
nào bán 4 ổ mà mua nguyên liệu cho 6 ổ trong cùng một buổi. Nên khi điền, cả hai
chỗ phải nhận **cùng một con số**.

So với hai chữ khác nhau ở tờ giấy trên: `b` và `c` là hai ô riêng, điền hai số
khác nhau là chuyện bình thường. Vậy có luật:

> Cùng một chữ = cùng một ô = **phải** điền cùng một số.
> Chữ khác nhau = ô khác nhau = **không buộc** phải điền số khác nhau, cũng
> không buộc phải giống nhau.

Vế thứ hai có một chỗ dễ đọc hụt. "Ô khác nhau" **không** có nghĩa là hai số phải
khác. Khách mua 2 ổ và 2 chai thì `b` và `c` cùng nhận số 2 — hoàn toàn hợp lệ,
chỉ là **tình cờ** bằng nhau lần đó, chứ không phải luật bắt chúng bằng nhau.
::::

::::predict{#doan-dien-lech commitOnce}
Byte nhờ máy tính tiền lãi khi bán **4 ổ**, tức là điền 4 vào chữ `n`.

Dòng đầu Byte điền đúng: cả hai chỗ mang chữ `n` đều nhận số 4.

Dòng sau Byte gõ vội, điền 4 vào chỗ trước và **6** vào chỗ sau — hai chỗ cùng
mang chữ `n` mà nhận hai số khác nhau.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(15000 * 4 - 9000 * 4)
print(15000 * 4 - 9000 * 6)
```

:::opt{correct}
24000, rồi 6000
:::

:::opt
24000, rồi máy báo lỗi
::why
Gần đúng ở chỗ bạn thấy dòng thứ hai **sai** — và nó sai thật. Luật bạn đang
dùng là luật vừa học, dùng đúng chỗ: hai chỗ mang chữ `n` là cùng một ô, điền
lệch nhau là kể chuyện Byte bán 4 ổ mà mua nguyên liệu cho 6 ổ, chuyện ấy không
có thật ở cái xe.

Chỗ lệch là **ai** giữ luật ấy. Nhìn kỹ dòng máy nhận được: nó chỉ thấy `15000`,
`4`, `9000`, `6`. Chữ `n` đã biến mất trước khi máy đọc tới — nó nằm trên tờ
giấy, không nằm trong dòng lệnh. Máy không có cách nào biết hai con số ấy đáng lẽ
phải bằng nhau, nên nó tính bình thường và in ra một con số đẹp đẽ.

Đó là chỗ đáng nhớ nhất của bài này: một con số **sai chuyện** vẫn trông y hệt
một con số đúng. Luật "cùng chữ thì cùng số" là luật của người đọc tờ giấy, và
người đọc phải tự giữ nó.
::
:::

:::opt
24000, rồi 24000
::why
Gần đúng ở chỗ bạn đang dùng một quy tắc rất chắc: cùng một câu tính thì cho cùng
một kết quả. Quy tắc ấy đúng — bài trước vừa dựng nó, và nó là lý do khách thứ tư
mua 2 ổ thì lại trả đúng 30 000.

Chỗ lệch là hai dòng này **không** phải cùng một câu tính. Chúng chỉ giống nhau ở
hình dạng thôi; con số điền vào chỗ cuối đã đổi từ 4 sang 6. Đổi một số điền vào
là đổi câu tính, nên kết quả đổi theo. Cái làm hai dòng khác nhau nhỏ tới mức dễ
lướt qua — và đó chính là lý do người ta dán chữ lên ô trống: có chữ rồi thì hai
chỗ ấy **nhìn** là thấy phải giống nhau.
::
:::

:::opt
204000, rồi 306000
::why
Gần đúng ở chỗ bạn đọc dòng lệnh theo đúng thứ tự nó được viết ra, trái sang
phải — và với một câu chữ thì đọc như thế là đúng.

Chỗ lệch là câu tính có thứ tự riêng của nó, thứ tự bạn đã dựng ở mạch trước:
nhân và chia làm trước, cộng và trừ làm sau. Nên `15000 * 4 - 9000 * 4` không
phải "15000 nhân 4, trừ 9000, rồi nhân 4" (ra 204 000) mà là "hai tích tính
xong rồi mới trừ nhau": 60 000 trừ 36 000. Cũng chính vì thế mà dòng
`15 000 × n − 9 000 × n` đọc được thành *tiền thu trừ tiền vốn* — hai cụm, mỗi
cụm một phép nhân trọn vẹn.
::
:::
::::

::::explain{#chu-cai-chua-phai-con-so}
Đặt tên cho thứ vừa dựng:

> Một **chữ cái** trong câu tính là **tên của một ô trống**. Mọi chỗ mang cùng
> một chữ là cùng một ô, nên phải điền cùng một số. Chữ khác nhau là ô khác
> nhau.

Ba việc cái nhãn ấy làm được, mà một ô trống trơn không làm được:

- **Đọc ra được ô nào là ô nào**, kể cả khi dòng có năm bảy ô.
- **Buộc được nhiều chỗ vào nhau.** `15 000 × n − 9 000 × n` nói ra một sự thật
  về cái xe — bán bao nhiêu thì tốn nguyên liệu bấy nhiêu — mà hai ô trống trơn
  không nói nổi.
- **Gọi tên được khi bàn bạc.** Byte nói "n là số ổ", người kia hiểu ngay, không
  phải chỉ tay vào tờ giấy.

Còn một chỗ dễ nhầm, và nó đủ quan trọng để tách riêng ra: chữ `n` **chưa** giữ
con số nào. Nó là cái nhãn, không phải cái hộp. Dán nhãn "gạo" lên một cái lọ
rỗng thì cái lọ vẫn rỗng — cái nhãn chỉ cho biết sau này ai đổ gì vào đó.
::::

::::code{#hai-to-giay-cua-byte}
Hai tờ giấy đang dán trên xe. Mỗi tờ đã được điền sẵn — việc của bạn là chép
đúng câu tính của tờ ấy, dùng đúng những chữ đã dán.

- **Tờ lãi:** `15 000 × n − 9 000 × n`. Buổi sáng Byte bán `4` ổ, nên `n` nhận
  số 4 — và nhận ở **cả hai** chỗ.
- **Tờ tính tiền khách:** `15 000 × b + 8 000 × c`. Một khách mua `3` ổ bánh mì
  và `1` chai nước, nên `b` nhận 3 còn `c` nhận 1 — hai ô khác nhau, hai số khác
  nhau.

Viết cả hai câu tính bằng **chữ**, không bằng con số điền vào. Gõ thẳng `4` hay
`3` vào chỗ trống thì cái nhãn không còn tác dụng gì, và tờ giấy chỉ dùng được
đúng một lần.

```python title=starter
# Tờ lãi:  15 000 × n  −  9 000 × n
# Sáng nay bán 4 ổ, nên điền 4 vào chữ n.
n = 4
lai_buoi_sang = ___

# Tờ tính tiền khách:  15 000 × b  +  8 000 × c
# Khách mua 3 ổ bánh mì và 1 chai nước.
b = 3
c = 1
tien_khach = ___

print(lai_buoi_sang)
print(tien_khach)
```

```python title=solution
# Tờ lãi:  15 000 × n  −  9 000 × n
# Sáng nay bán 4 ổ, nên điền 4 vào chữ n.
n = 4
lai_buoi_sang = 15000 * n - 9000 * n

# Tờ tính tiền khách:  15 000 × b  +  8 000 × c
# Khách mua 3 ổ bánh mì và 1 chai nước.
b = 3
c = 1
tien_khach = 15000 * b + 8000 * c

print(lai_buoi_sang)
print(tien_khach)
```

```python title=test
# Ba câu `!=` canh ba cách điền sai mà bài đã gọi tên thẳng ra. Chúng đứng TRƯỚC
# các câu `==` vì chương trình dừng ở câu vỡ đầu tiên; xếp sau một câu `==` bao
# trùm thì chúng không bao giờ chạy tới, và cái bẫy không bao giờ sập.
assert lai_buoi_sang != 6000, "hai chỗ mang chữ n là CÙNG một ô — điền 4 vào chỗ này rồi 6 vào chỗ kia là kể chuyện Byte bán 4 ổ mà mua nguyên liệu cho 6 ổ"
assert tien_khach != 69000, "b và c là hai ô khác nhau — điền cả hai bằng 3 là bắt khách ôm về 3 chai nước trong khi họ chỉ lấy 1"
assert 15000 * c + 8000 * b != tien_khach, "đổi chỗ hai chữ là đổi hẳn câu chuyện: 1 ổ bánh mì với 3 chai nước không ra cùng số tiền"
# Hai tờ giấy, hai con số khác nhau — nên một số gõ cứng chỉ qua được nhiều nhất
# một câu.
assert lai_buoi_sang == 24000, "bán 4 ổ: thu 60 000, vốn 36 000, lãi 24 000"
assert tien_khach == 53000, "3 ổ bánh mì 45 000 cộng 1 chai nước 8 000"
```

:::hints
- kind: attention
  body: Đọc lại dòng chú thích ngay trên mỗi chỗ trống — nó chép sẵn nguyên văn tờ giấy. Việc của bạn là viết lại đúng dòng ấy bằng ký hiệu của Python, giữ nguyên mọi chữ cái đang đứng trong đó. Đếm xem tờ thứ nhất có mấy chỗ mang chữ `n`.
- kind: strategy
  body: Dấu × trên tờ giấy viết trong Python là dấu `*`. Tờ thứ nhất có hai cụm nhân rồi mới trừ nhau, và chữ `n` phải xuất hiện ở cả hai cụm. Tờ thứ hai có hai cụm nhân rồi cộng lại, mỗi cụm một chữ riêng. Đừng thay chữ bằng con số — máy đã có sẵn `n`, `b`, `c` ở mấy dòng trên rồi.
- kind: one-line
  body: "Chỗ trống thứ nhất là `15000 * n - 9000 * n`, chỗ trống thứ hai là `15000 * b + 8000 * c`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai câu tính phải viết bằng CHỮ — `n` phải có mặt hai lần ở tờ lãi, còn `b` và `c` mỗi chữ một lần ở tờ khách; thay chữ bằng số điền vào là bỏ mất cái nhãn
  requireAst:
  # Khung khởi đầu không có dấu `*` nào và không ĐỌC cái tên nào (`n = 4` là gán,
  # không tính), nên bộ luật này chặn được đáp án chép cứng hai con số.
  # `n` phải đọc ĐÚNG HAI lần: đó chính là luật "cùng một chữ, hai chỗ".
  - kind: uses-operator, target: *, min: 4
  - kind: uses-operator, target: -, min: 1
  - kind: uses-operator, target: +, min: 1
  - kind: uses-name, target: n, min: 2
  - kind: uses-name, target: b, min: 1
  - kind: uses-name, target: c, min: 1
  forbidAst:
  # Lưới thứ hai, chặn hai con số KẾT QUẢ. `uses-operator` đếm trên cả file nên
  # một chỗ gõ cứng vẫn có thể lọt qua bộ luật trên; hai luật này chặn đúng chỗ
  # đó. Không cách viết hợp lệ nào chứa nguyên văn 24000 hay 53000.
  - kind: has-literal, target: 24000
  - kind: has-literal, target: 53000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^24000\n53000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giờ tờ giấy tự nói ra ô nào là ô nào. Mình khỏi phải nhớ nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trong bài này, mỗi lần muốn ra một con số thì bạn đều phải **điền** trước: cho
`n` số 4, cho `b` số 3, cho `c` số 1. Điền xong máy mới nhả ra 24 000 và 53 000.

Nhưng lúc tờ giấy vừa dán lên xe, sáu giờ sáng, chưa khách nào tới — thì trên đó
đang có cái gì?

```text
15 000 × n
```

Trong Python, chuyện này rõ ràng. `n = 5` là một cái tên đang giữ đúng một con
số, và `15000 * n` lập tức **là** một con số. Chưa gán gì cho `n` mà đã gọi tới
nó thì máy dừng lại luôn — bạn đã gặp cái dừng ấy từ Realm 0.

Còn tờ giấy dán ngoài xe thì không dừng lại. Nó vẫn nằm đó cả buổi, chưa ai điền
gì, và không ai coi nó là hỏng cả.

Vậy `15 000 × n` **đã là một con số chưa**?

Nếu chưa — thì nó là **cái gì**? Nó viết ra được, đọc thành lời được, Byte chép
lại được sang tờ khác. Một thứ vừa cầm được như thế mà lại chưa phải một con số
thì gọi là gì?

Bài sau đặt tên cho nó.
::::

::::checkpoint{mastery=0.8}
::::
