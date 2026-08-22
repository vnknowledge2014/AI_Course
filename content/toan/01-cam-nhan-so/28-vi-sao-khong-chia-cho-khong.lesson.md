---
id: toan.cam-nhan-so.vi-sao-khong-chia-cho-khong
title: Vì sao không chia được cho 0
summary: "`12 : 0` không có kết quả vì câu hỏi hỏng, chứ không phải vì máy cấm — đặt cái thước 0 mét bao nhiêu lần cũng không lấp nổi 12 mét."
locale: vi
track: toan
module: cam-nhan-so
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.division-by-zero]
requires: [math.division-quotative, math.division-partitive, core.arithmetic, core.division, core.boolean, ctrl.comparison, err.traceback]
concepts: [math.chia-do, math.don-vi, math.cau-hoi-hong]
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
Có câu hỏi mình không trả lời được. Không phải vì khó — vì nó chưa hỏi gì cả.
::::

::::explain{#hoi-lai-cau-hoi-cua-bai-truoc}
Bài trước để lại đúng một câu: **"0 lọt vào 12 mấy lần?"**

Trước khi trả lời, hãy đọc lại cho kỹ cái câu mà bài trước dạy bạn đặt. Khi
Byte cầm sợi dây 12 mét và cái thước 3 mét, câu hỏi thật của phép chia là:

> Đặt cái thước ấy nối tiếp nhau, **mấy lần** thì phủ hết 12 mét?

Đặt 1 lần được 3 mét. Hai lần được 6 mét. Ba lần được 9 mét. Bốn lần được 12
mét — vừa khít. Nên `12 : 3 = 4`, và con số 4 là câu trả lời cho chữ **mấy lần**.

Giờ thay cái thước 3 mét bằng một cái thước **0 mét**.

Đặt 1 lần: được 0 mét. Đặt 4 lần: vẫn 0 mét. Đặt một nghìn lần: 0 mét. Đặt một
triệu lần: vẫn 0 mét, không nhích lên một xíu nào. Cái thước ấy không dài thêm
được, nên chồng bao nhiêu cái cũng nằm nguyên tại chỗ.

Vậy "mấy lần thì phủ hết 12 mét?" — **không có lần nào cả**. Không phải câu trả
lời là một con số nào đó lớn lắm. Là **không có con số nào**, dù bạn đi hết dãy
số cũng không gặp.
::::

::::explain{#nhin-tu-phia-phep-nhan}
Còn một đường nữa dẫn tới cùng chỗ đó, và nó ngắn hơn.

Phép chia là câu hỏi ngược của phép nhân. Viết `12 : 3 = 4` nghĩa là *`4 × 3`
cho ra `12`*. Muốn kiểm một phép chia, bạn nhân ngược lại là biết ngay.

Vậy `12 : 0` đang hỏi: **số nào nhân với 0 thì ra 12?**

Nhưng nhân với 0 là lấy **không lần nào** cái lượng ấy. Không lần nào thì không
có gì, nên số nào nhân với 0 cũng ra 0: `7 × 0 = 0`, `500 × 0 = 0`,
`999999 × 0 = 0`. Cả dãy số, không con nào chệch ra khỏi con số 0 ấy.

Một câu hỏi mà không có lời đáp nào đúng thì nó không phải câu hỏi khó — nó là
câu hỏi **hỏng**. `12 : 0` hỏng đúng kiểu như câu "sợi dây này nặng mấy mét".

Một chỗ dễ lẫn cần nói cho rõ: con số `0` đứng riêng một mình ở đây mang nghĩa
**một lượng rỗng** — thước không dài tí nào. Nó khác hẳn chữ số `0` trong dãy
`307`, nơi `0` làm việc giữ chỗ cho một cột rỗng. Cùng một ký hiệu, hai việc.
::::

::::example{#hoi-thang-cai-may}
Hỏi thẳng máy. Ba dòng đầu đặt cái thước 0 mét lần lượt 4 lần, một nghìn lần,
một triệu lần. Dòng thứ tư hỏi một câu có–không: chồng ấy đã phủ hết 12 mét chưa?

```python title=readonly
print(0 * 4)
print(0 * 1000)
print(0 * 1000000)
print(0 * 1000000 == 12)
```

Máy in ra:

```text
0
0
0
False
```

Ba lần đặt thước, ba lần vẫn ở nguyên vạch 0. Và câu trả lời cuối cùng là
`False` — chưa phủ được, sẽ không bao giờ phủ được.

Bây giờ tới chỗ đáng xem nhất: bảo thẳng máy chia 12 cho 0. Bạn đoán nó làm gì?
::::

::::predict{#doan-may-lam-gi commitOnce}
Byte sắp chạy đúng một dòng. **Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(12 / 0)
```

:::opt{correct}
Máy không in con số nào, chỉ hiện một thông báo lỗi có chữ ZeroDivisionError
:::

:::opt
0
::why
Gần đúng ở chỗ bạn đang dùng một luật có thật: dính tới số 0 thì kết quả thường
về 0. Luật ấy đúng cho phép nhân (`7 × 0 = 0`) và đúng cho cả phép chia khi số 0
đứng ở **vế trái**: `0 : 12 = 0`, vì chia 0 mét dây cho 12 người thì mỗi người
nhận 0 mét.

Chỗ ra ngoài phạm vi là hai con số đã đổi vai cho nhau. `0 : 12` hỏi "chia 0 mét
ra", còn `12 : 0` hỏi "cái thước 0 mét lọt vào 12 mét mấy lần". Câu đầu có đáp
án; câu sau thì không có lần nào.
::
:::

:::opt
12
::why
Gần đúng ở chỗ bạn nhớ đúng một luật khác: làm gì với số 0 thì lượng cũ giữ
nguyên. `12 + 0 = 12` và `12 − 0 = 12` — cả hai đều chuẩn, vì gộp thêm không có
gì thì đống dây vẫn thế, bớt đi không có gì thì cũng vẫn thế.

Chỗ ra ngoài phạm vi: cái số "không đổi gì" của phép chia là **1**, không phải
0. `12 : 1 = 12` nghĩa là cái thước 1 mét lọt vào 12 mét đúng 12 lần. Mỗi phép
toán có riêng một con số vô hại của nó, và của phép chia thì con số ấy là 1.
::
:::

:::opt
Một con số rất lớn, kiểu như vô cùng
::why
Gần đúng ở chỗ bạn đã nhìn thấy một quy luật thật và nhìn rất tinh: thước càng
ngắn thì lọt vào 12 mét càng nhiều lần. Thước 3 mét lọt 4 lần, thước 1 mét lọt
12 lần, thước nửa mét lọt 24 lần — cứ ngắn đi thì con số cứ phình ra.

Chỗ ra ngoài phạm vi: dãy ấy phình mãi mà **không dừng ở đâu cả**, nên nó không
chỉ vào một con số nào để làm đáp án. Và cái thước 0 mét cũng không nằm cuối dãy
ấy: mọi cái thước trong dãy đều dài hơn 0 một chút và đều phủ được 12 mét nếu
đặt đủ nhiều, còn thước 0 mét thì đặt bao nhiêu lần cũng phủ 0 mét. Nó khác loại,
không phải khác cỡ.
::
:::
::::

::::explain{#may-khong-cam-ban}
Chạy lên, máy in ra thế này:

```text
Traceback (most recent call last):
  File "vuon.py", line 1, in <module>
    print(12 / 0)
          ~~~^~~
ZeroDivisionError: division by zero
```

Chữ ở đầu dòng dưới cùng ghép từ *zero* (số không), *division* (phép chia) và
*error* (lỗi). Dịch sát: **lỗi chia cho không**.

Đây là chỗ dễ hiểu lầm nhất của cả bài, nên nói cho thẳng: **máy không cấm bạn.**
Người viết ra Python có đặt sẵn một chỗ chặn ở đây thật — nhưng họ chặn vì **câu
hỏi hỏng**, chứ không phải vì họ ngại con số 0. Nếu `12 : 0` có đáp án thì họ đã
cho máy trả lời rồi, y như nó vẫn trả lời `12 / 0.0001` ngay dưới đây.
Bỏ hết máy tính trên đời đi thì `12 : 0` vẫn không có kết quả.

Và để thấy máy chia được cho mọi cái thước khác: `12 / 3` cho `4.0`, `12 / 1` cho
`12.0`, `12 / 0.5` cho `24.0`, `12 / 0.0001` cho `120000.0`. Cái thước cuối mỏng
như sợi tóc, máy vẫn trả lời gọn ghẽ. (Dấu chấm với số `0` phía sau là do phép
`/` luôn cho ra số **có phần thập phân**, kể cả khi chia hết — R0 bài 15 đã nói.)
Chỉ đúng một cái thước làm nó dừng, và đó là cái thước không dài tí nào.

Còn một kiểu hỏng nữa, hỏng ngược lại: `0 : 0` hỏi "thước 0 mét lọt vào 0 mét
mấy lần?" — đặt 5 lần cũng được 0 mét, đặt 100 lần cũng được 0 mét, lần nào cũng
"đúng". Câu đầu không có đáp án nào, câu này thì đáp án nào cũng lọt. Cả hai đều
không chỉ ra được **một** con số, nên cả hai đều là câu hỏi hỏng.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Bạn tự bắt mình tin nhé. Hai phép chia, hai câu trả lời khác nhau.
::::

::::code{#tu-tay-kiem-hai-phep-chia}
Byte muốn tự tay xác nhận, bằng đúng cái cầu vừa dựng ở trên: **chia là câu hỏi
ngược của nhân**. Muốn biết một phép chia có đáp án hay không thì đem thử một con
số rồi nhân ngược lại xem có về đúng chỗ cũ không.

- **`12 : 3`** hỏi *"số nào nhân 3 thì ra 12?"* — thử số **4**.
- **`12 : 0`** hỏi *"số nào nhân 0 thì ra 12?"* — thử luôn **một triệu**.

Mỗi phép chia in hai dòng: dòng trên là kết quả nhân ngược (đã viết sẵn), dòng
dưới là câu trả lời cho câu hỏi *"nhân ngược có về đúng 12 chưa"*. Hai chỗ trống
là hai câu hỏi ấy.

Bài chấm bằng **cả hai** phép chia, và hai phép này cho ra hai câu trả lời ngược
nhau. Gõ cứng `True` vào cả hai chỗ thì phép `12 : 0` sai; gõ cứng `False` thì
phép `12 : 3` sai. Và chép thẳng con số cũng không qua: câu hỏi phải so **cái
tên** đang giữ kết quả nhân ngược với 12.

```python title=starter
# 12 : 3 hỏi "số nào nhân 3 thì ra 12?" — thử số 4
thu_bon = 4 * 3
print(thu_bon)
print(___)

# 12 : 0 hỏi "số nào nhân 0 thì ra 12?" — thử một triệu
thu_mot_trieu = 1000000 * 0
print(thu_mot_trieu)
print(___)
```

```python title=solution
# 12 : 3 hỏi "số nào nhân 3 thì ra 12?" — thử số 4
thu_bon = 4 * 3
print(thu_bon)
print(thu_bon == 12)

# 12 : 0 hỏi "số nào nhân 0 thì ra 12?" — thử một triệu
thu_mot_trieu = 1000000 * 0
print(thu_mot_trieu)
print(thu_mot_trieu == 12)
```

```python title=test
# Chốt lại chính điều bài vừa nói, trên hai phép chia khác hẳn nhau. Nếu một
# ngày nào đó máy chạy bài học tính sai chỗ này thì cổng đỏ lên, chứ không dạy
# sai lặng lẽ.
assert thu_bon == 12, "4 nhân 3 về đúng 12 — nên 4 là đáp án của 12 : 3"
assert thu_mot_trieu == 0, "một triệu nhân 0 vẫn ra 0, không nhích lên tí nào"
assert thu_mot_trieu != 12, "đây là cả nội dung bài: không con số nào nhân 0 mà về được 12"
assert 7 * 0 != 12, "7 không phải đáp án của 12 : 0"
assert 999999999 * 0 != 12, "một con số khổng lồ cũng không phải đáp án của 12 : 0"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều nằm trong `print`, ngay dưới một dòng đã in ra kết quả nhân ngược. Việc của chúng không phải in lại con số ấy lần nữa, mà là hỏi máy một câu chỉ có hai câu trả lời.
- kind: strategy
  body: Câu hỏi ấy gồm ba phần: cái tên đang giữ kết quả nhân ngược, dấu so sánh bằng của R0 bài 24 (hai dấu bằng viết liền nhau), và con số 12 — cái số bị chia. Hai chỗ trống dùng hai cái tên khác nhau, nên không chép được của nhau.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `thu_bon == 12` và `___` thứ hai bằng `thu_mot_trieu == 12`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh bằng (`==`) giữa kết quả nhân ngược và con số 12 — và phải so cái TÊN đang giữ kết quả ấy với 12, không phải chép lại con số
  requireAst:
  # `min: 2` vì có hai phép chia, mỗi phép một câu hỏi. Khung chưa có dấu `==`
  # nào, nên luật này chặn được đúng cái đáp án gõ cứng hai chữ True/False.
  - kind: uses-operator, target: ==, min: 2
  # Khung đã ĐỌC mỗi cái tên đúng một lần (`print(thu_bon)`), nên `min: 2` ép
  # chỗ trống phải nhắc lại tên ấy — chặn đáp án `print(12 == 12)` chép cứng
  # con số mà vẫn khớp output lẫn dấu `==`.
  - kind: uses-name, target: thu_bon, min: 2
  - kind: uses-name, target: thu_mot_trieu, min: 2
- tier: output
  match: regex
  expect: ^12\nTrue\n0\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn nhân 3 về đúng 12. Một triệu nhân 0 thì vẫn nằm nguyên ở số 0.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái thước 0 mét là ca hỏng. Nhưng còn những cái thước bình thường mà **không
chia hết** thì sao?

Byte có 13 mét dây, vẫn cắt thành từng đoạn 3 mét. Đặt 4 lần thì được 12 mét —
còn đúng 1 mét nữa. Đặt thêm một lần thứ năm thì thành 15 mét, dài quá sợi dây.

Vậy Byte cắt được 4 đoạn, và **thừa 1 mét**. Chỗ thừa ấy có thật, sờ được, cầm
được — nhưng nó không phải một đoạn, cũng không phải không có gì.

Gọi nó bằng một con số được không, và con số ấy đứng ở chỗ nào trong phép chia
vừa làm? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
