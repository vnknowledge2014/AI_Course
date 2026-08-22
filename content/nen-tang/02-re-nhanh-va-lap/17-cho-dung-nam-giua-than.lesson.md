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

Một vòng lặp chỉ được coi là viết đúng khi nó xử đúng **mọi** khách, chứ không
riêng khách sáng nay. Nên Byte đặt tên cho việc hỏi, đúng cách Realm 0 đã dạy,
rồi gọi nó với ba người khách:

- khách sáng bấm Enter suông hai lần rồi mới gõ `45000`;
- khách trưa gõ `120000` ngay lần đầu, không bấm suông lần nào;
- khách tối bấm Enter suông **bốn** lần rồi mới gõ `80000`.

Đoạn dưới thiếu hai chỗ, và hai chỗ ấy **không** điền giống nhau. Chúng đúng là
cặp câu hỏi ngược chiều vừa nói ở trên: chỗ ở đầu vòng giữ cho vòng **ở lại**,
chỗ trong `if` quyết định lúc nào **đi ra**.

Vì bạn không biết trước khách sẽ bấm Enter suông mấy lần, đừng viết số lượt vào
chỗ trống ở đầu vòng.

```python title=starter
# Byte gõ hộ bàn phím: mỗi phần tử là một lần khách bấm phím rồi Enter.
def hoi_toi_khi_co_so(khach_go):
    lan = 0
    while ___:
        tra_loi = khach_go[lan]
        lan = lan + 1
        if ___:
            break
    return tra_loi

khach_sang = ["", "", "45000"]
khach_trua = ["120000"]
khach_toi = ["", "", "", "", "80000"]

print(f"Khách sáng gõ {hoi_toi_khi_co_so(khach_sang)} đồng")
print(f"Khách trưa gõ {hoi_toi_khi_co_so(khach_trua)} đồng")
print(f"Khách tối gõ {hoi_toi_khi_co_so(khach_toi)} đồng")
```

```python title=solution
# Byte gõ hộ bàn phím: mỗi phần tử là một lần khách bấm phím rồi Enter.
def hoi_toi_khi_co_so(khach_go):
    lan = 0
    while True:
        tra_loi = khach_go[lan]
        lan = lan + 1
        if tra_loi != "":
            break
    return tra_loi

khach_sang = ["", "", "45000"]
khach_trua = ["120000"]
khach_toi = ["", "", "", "", "80000"]

print(f"Khách sáng gõ {hoi_toi_khi_co_so(khach_sang)} đồng")
print(f"Khách trưa gõ {hoi_toi_khi_co_so(khach_trua)} đồng")
print(f"Khách tối gõ {hoi_toi_khi_co_so(khach_toi)} đồng")
```

```python title=test
# Chấm trên BA khách, không phải một.
#
# Chấm bằng đúng một khách thì không phân biệt được đúng với gặp may: khách
# sáng bấm Enter suông hai lần, nên `while lan < 3:` cũng qua — mà câu ấy sai,
# nó đoán trước số lượt. Ba khách đây được chọn để mỗi câu trả lời hụt đều lộ:
#   `while lan < 3:`      → khách tối bấm suông bốn lần, vòng thoát ở đầu lượt
#                           khi chưa có số, hàm trả về chuỗi rỗng;
#   `if True:` ở chỗ hai  → thoát ngay lượt đầu, khách sáng ra chuỗi rỗng;
#   `while False:`        → thân không chạy lượt nào, `tra_loi` chưa hề có.
assert hoi_toi_khi_co_so(["", "", "45000"]) == "45000", "khách sáng bấm Enter suông hai lần rồi mới gõ 45000 — hai lần suông đó không phải câu trả lời, con số mới là"
assert hoi_toi_khi_co_so(["120000"]) == "120000", "khách trưa gõ số ngay lần hỏi đầu tiên, nên không được hỏi lại lần nào nữa"
assert hoi_toi_khi_co_so(["", "", "", "", "80000"]) == "80000", "khách tối bấm Enter suông bốn lần — khách còn bấm suông thì còn phải hỏi lại, không đoán trước được mấy lần"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở hai chỗ khác nhau và không điền giống nhau. Chỗ thứ nhất ở giữa `while` và dấu hai chấm; chỗ thứ hai ở giữa `if` và dấu hai chấm, ngay trên dòng `break`.
- kind: strategy
  body: Chỗ ở đầu vòng cần một điều kiện không bao giờ sai, để máy không tự thoát ra ở đầu lượt — nó không phải một câu so sánh, mà là một giá trị đúng-sai viết thẳng ra, loại bạn đã gặp từ bài "Đúng hay sai". Câu hỏi thật nằm ở chỗ thứ hai, và nó hỏi ngược lại: khách vừa gõ đã có gì chưa, tức `tra_loi` có khác chuỗi rỗng không.
- kind: one-line
  body: Viết `True` vào chỗ trống sau `while` — chữ T viết hoa, không dấu nháy — và viết `tra_loi != ""` vào chỗ trống sau `if`.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Khách tối gõ 80000 đồng
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
