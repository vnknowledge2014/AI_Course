---
id: nen-tang.re-nhanh-va-lap.cho-dung-nam-giua-than
title: Chỗ dừng nằm giữa thân vòng
summary: Khi lối ra thật sự nằm giữa thân, điều kiện ở đầu vòng viết thẳng là True và việc dừng giao hết cho break.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ctrl.while-true]
requires: [ctrl.break, ctrl.while, core.boolean]
concepts: [ctrl.lap, core.dung-sai]
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
Vòng này mình không định dừng. Chỗ dừng nằm ở giữa, do bạn đặt.
::::

::::explain{#dong-moi-phai-viet-hai-lan}
Bài trước để lại câu hỏi: điều kiện ở đầu `while` còn để làm gì, khi lối ra thật
sự là dòng `break` nằm giữa thân?

Hãy nhìn một đoạn code mà câu hỏi ấy làm phiền thật sự. Byte hỏi bà chủ số tiền
tiêu hôm nay, và phải hỏi lại chừng nào bà còn bấm Enter suông:

```python title=readonly
tra_loi = input("Số tiền hôm nay: ")

while tra_loi == "":
    tra_loi = input("Số tiền hôm nay: ")

print(f"Đã ghi: {tra_loi}")
```

Đoạn này chạy đúng. Nhưng dòng `input` bị gõ **hai lần**, và hai lần ấy giống
nhau từng chữ.

Vì sao lại phải thế? Vì bài "Điều kiện được xem lại lúc nào" đã nói rõ: máy đọc
điều kiện `tra_loi == ""` ở **đầu** lượt, tức là trước khi thân vòng chạy dòng
nào. Muốn đọc được, cái tên `tra_loi` phải có sẵn giá trị từ trước. Dòng `input`
đứng trên vòng lặp tồn tại chỉ để làm việc đó: mồi sẵn một giá trị cho điều kiện
có thứ mà nhìn.

Người ta gọi nó là **dòng mồi**. Và nó phải trả giá:

- Đổi câu hỏi thành `"Tiêu bao nhiêu: "` là phải sửa hai chỗ. Sửa một chỗ thì
  lần hỏi đầu và lần hỏi lại thành hai câu khác nhau.
- Đọc code, mắt bạn thấy hai lần hỏi và phải tự nhủ *"thật ra chỉ có một việc
  hỏi thôi"*.

Vấn đề gốc: việc hỏi phải xảy ra **trước** khi kiểm, mà điều kiện thì luôn nằm
trước thân. Hai thứ ấy giành nhau chỗ đứng đầu tiên.
::::

::::example{#while-true}
Cách gỡ: thôi không dùng cửa trước nữa. Cho vòng lặp một điều kiện **không bao
giờ sai**, rồi giao hẳn việc dừng cho `break` ở giữa thân.

Điều kiện không bao giờ sai thì viết thế nào? Nó có sẵn từ Realm 0. `True` là
một giá trị đàng hoàng, y như `45000` hay `"phở"` — và một điều kiện viết thẳng
là `True` thì lần nào đọc lại cũng ra `True`.

```python title=readonly
while True:
    tra_loi = input("Số tiền hôm nay: ")
    if tra_loi != "":
        break

print(f"Đã ghi: {tra_loi}")
```

Dòng `input` giờ chỉ còn **một**. Dòng mồi biến mất, vì không còn điều kiện nào
ở đầu cần được mồi.

Đọc `while True:` cho đúng: *"chừng nào `True` còn đúng thì lặp"*. `True` luôn
đúng, nên câu ấy có nghĩa là **lặp mãi**. Máy sẽ không bao giờ tự thoát ra ở đầu
lượt — chuyện thoát ra bây giờ hoàn toàn nằm trong tay dòng `break`.

Để ý điều kiện đã **đảo chiều** khi nó chuyển chỗ:

- Ở đầu vòng, câu hỏi là *"có phải hỏi lại không?"* — `tra_loi == ""`.
- Ở giữa thân, câu hỏi là *"đủ để đi tiếp chưa?"* — `tra_loi != ""`.

Cùng một ý, hai cách hỏi ngược nhau. Cửa trước hỏi để **ở lại**, `break` hỏi để
**đi ra**.

Và một điều bắt buộc phải nhớ: `while True` mà trong thân không có `break` nào
thì chính là vòng lặp vô hạn ở bài "Vòng lặp không chịu dừng". Ở đó, bước tiến
trong thân là thứ đẩy điều kiện dần tới sai. Ở đây không còn điều kiện nào để
đẩy, nên `break` chính là bước tiến — thiếu nó là máy chạy mãi, và bạn phải bấm
Dừng.
::::

::::predict{#doan-vong-khong-dieu-kien commitOnce}
Đoạn dưới có `while True` và một `break`. **Trước khi bấm chạy**, bạn đoán màn
hình hiện ra những dòng nào?

```python title=readonly
lan = 0

while True:
    lan = lan + 1
    print(f"Lượt {lan}")
    if lan == 3:
        break

print(f"Dừng ở lượt {lan}")
```

:::opt{correct}
Lượt 1, Lượt 2, Lượt 3, rồi Dừng ở lượt 3
:::

:::opt
Máy in Lượt 1, Lượt 2, Lượt 3, Lượt 4… không bao giờ dừng, phải bấm Dừng
::why
Gần đúng ở chỗ quan trọng nhất, và đây là phản xạ đúng cần giữ: `while True`
thật sự không bao giờ tự sai, nên nhìn riêng dòng đầu thì đây đúng là một vòng
lặp vô hạn.

Chỗ lệch là bài này đã đặt sẵn một lối ra khác. Xuống ba dòng nữa có `if lan ==
3:` và `break` — tới lượt thứ ba, `lan` bằng 3, máy đi ra. `while True` chỉ vô
hạn khi trong thân **không có** `break` nào chạy tới được.
::
:::

:::opt
Không in dòng nào, vì `True` không phải một câu so sánh nên chưa phải điều kiện
::why
Gần đúng ở chỗ mọi điều kiện bạn từng viết cho tới giờ đều có dấu so sánh ở
giữa: `lan < 5`, `tien > 500000`, `tra_loi == ""`. Trông chúng giống nhau tới
mức dễ tưởng đó là hình dạng bắt buộc.

Chỗ lệch: cái `while` cần không phải một **dấu so sánh**, mà một **giá trị
`True` hoặc `False`**. Dấu so sánh chỉ là một cách làm ra giá trị ấy. Từ bài
"Đúng hay sai" ở Realm 0, `True` đã là một giá trị viết thẳng ra được — nên đưa
thẳng nó cho `while` là hợp lệ, và máy đọc được ngay.
::
:::

:::opt
Lượt 1, Lượt 2, Lượt 3, rồi máy báo lỗi vì điều kiện không bao giờ thành sai
::why
Gần đúng ở chỗ bạn cảnh giác với vòng lặp không có đường ra — đúng thứ bài
trước dặn phải để ý.

Chỗ lệch là Python không kiểm chuyện đó. Nó không đọc trước thân vòng để đoán
xem bạn có thoát ra được hay không; nó chỉ chạy. `while True` là câu hợp lệ, và
lỗi duy nhất có thể xảy ra là chương trình chạy mãi — mà ở đây thì `break` đã
lo xong.
::
:::
::::

::::explain{#dung-luc-nao-thi-hop}
Từ giờ bạn có hai kiểu vòng `while`, và mỗi kiểu hợp với một loại việc:

- **`while <điều kiện>`** khi thứ cần kiểm đã có sẵn trước lượt đầu tiên: số dư
  trong ví, số ngày còn lại, biến đếm bạn tự đặt.
- **`while True` kèm `break`** khi thứ cần kiểm **chỉ ra đời giữa thân vòng** —
  câu người dùng vừa gõ, con số vừa tính xong. Viết cách này thì bạn khỏi phải
  mồi sẵn nó một lần ở ngoài.

Cả hai đều là vòng lặp bình thường, không cái nào "cao cấp" hơn cái nào. Chọn
cái nào là chọn xem lối ra tự nhiên nằm ở đầu vòng hay giữa thân.
::::

::::code{#hoi-lai-cho-toi-khi-co-so}
Byte hỏi số tiền tiêu hôm nay, và phải hỏi lại chừng nào khách còn bấm Enter
suông.

Ô chấm điểm không có ai ngồi trước bàn phím, nên Byte đóng vai khách: mấy câu
khách gõ được ghi sẵn trong một danh sách, mỗi lượt lấy ra một câu.

Đoạn dưới chỉ còn thiếu điều kiện của vòng lặp. Nhớ rằng bạn đang viết cho **mọi
khách**, không riêng khách hôm nay — nên đừng viết số lượt vào đó, vì bạn không
biết trước khách sẽ bấm Enter suông mấy lần.

```python title=starter
# Byte gõ hộ bàn phím: hai lần bấm Enter suông, lần thứ ba mới gõ số.
khach_go = ["", "", "45000"]
lan = 0

while ___:
    tra_loi = khach_go[lan]
    lan = lan + 1
    if tra_loi != "":
        break

print(f"Khách gõ {tra_loi} đồng")
```

```python title=solution
# Byte gõ hộ bàn phím: hai lần bấm Enter suông, lần thứ ba mới gõ số.
khach_go = ["", "", "45000"]
lan = 0

while True:
    tra_loi = khach_go[lan]
    lan = lan + 1
    if tra_loi != "":
        break

print(f"Khách gõ {tra_loi} đồng")
```

```python title=test
# Vòng phải chạy đủ ba lượt: bỏ qua hai câu rỗng, giữ lại câu thứ ba.
assert lan == 3
assert tra_loi == "45000"
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa `while` và dấu hai chấm. Cả việc dừng đã được dòng `break` phía dưới lo trọn rồi — vậy chỗ này còn cần nói điều gì?
- kind: strategy
  body: Bạn cần một điều kiện không bao giờ sai, để máy không tự thoát ra ở đầu lượt. Điều kiện ấy không phải một câu so sánh, mà là một giá trị đúng-sai viết thẳng ra — loại giá trị bạn đã gặp từ bài "Đúng hay sai".
- kind: one-line
  body: "Viết `True` vào chỗ trống — chữ T viết hoa, không dấu nháy."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Khách gõ 45000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dòng hỏi, không còn dòng mồi. Chỗ dừng nằm đúng chỗ bạn biết đủ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Dừng hẳn thì bạn đã có `break`. Nhưng có loại việc khác: chỉ muốn **bỏ qua một
lượt** rồi đi tiếp.

Sổ chi tiêu có những ngày bà chủ quên không ghi, để trống. Duyệt tới ngày như
vậy thì không cộng, không đếm, không in gì cả — nhưng vẫn phải duyệt tiếp những
ngày sau.

Cách bạn làm được ngay hôm nay là bọc **toàn bộ** phần còn lại của thân vòng vào
một `if ngày này có ghi:`. Nó chạy đúng. Chỉ có điều mọi dòng việc bị đẩy thụt
vào thêm một tầng nữa — và bài "Lối rẽ nằm trong lối rẽ" đã cho bạn thấy đọc một
dòng nằm sâu ba tầng thụt lề mệt tới đâu.

Có cách nào nói thẳng *"lượt này bỏ, đi tiếp"* mà không phải bọc thêm một tầng
không? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
