---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.viet-mot-so-ra-bit
title: Viết một số thành dãy bit
summary: "Chia liên tiếp cho hai, giữ lại từng số dư — đó là dãy bit, đọc từ dưới lên. Lần đầu tiên `%` và `//` của Realm 1 làm việc cùng nhau cho một mục đích thật."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [mem.int-to-binary]
requires: [mem.binary-to-int, core.modulo, core.floor-division]
concepts: [mem.int-to-binary]
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
22 khách ghé quán tối nay. Đèn cần hiện đúng con số ấy — nhưng đèn chỉ biết
bật với tắt, chưa từng biết số 22 là gì.
::::

::::explain{#chia-doi-lien-tiep}
Bài trước bạn đọc một dãy bit **có sẵn** ra thành số — cộng đáng giá những
cột đang bật. Bây giờ làm chiều ngược lại: có sẵn con số, cần dựng dãy bit.

Đáng giá mỗi cột là một luỹ thừa của hai: 1, 2, 4, 8, 16... Muốn biết một
con số cần cột nào, hỏi nó theo kiểu khác: **con số này có phải số chẵn
không** — tức chia hết cho hai không. Chẵn thì cột phải cùng tắt (`0`); lẻ
thì cột phải cùng bật (`1`), vì nó còn dư ra đúng 1 sau khi đã chia hết phần
chẵn.

Hai câu hỏi ấy bạn đã có công cụ sẵn từ Realm 1: bài **Phần còn thừa** cho
`%`, hỏi đúng "còn dư mấy sau khi chia"; bài **Phép chia không có phần lẻ**
cho `//`, hỏi "chia hết được bao nhiêu lần". Ở đây `%` cho biết cột phải
cùng, còn `//` bỏ cột đó đi và đẩy phần còn lại sang **hỏi tiếp về cột kế
tiếp** — đúng việc bạn vẫn làm khi đóng bó: bó xong một lớp thì đem phần còn
lại đi bó tiếp lớp sau, không đụng tới phần đã bó.

Lặp lại: chia cho hai, ghi số dư, lấy phần nguyên, chia tiếp — cho tới khi
phần nguyên về 0, tức không còn gì để hỏi thêm nữa.
::::

::::example{#viet-13-ra-bit}
Viết số 13 ra bit. Chia liên tiếp cho hai, ghi lại từng số dư:

```text title=readonly
13 // 2 = 6,  dư 1
 6 // 2 = 3,  dư 0
 3 // 2 = 1,  dư 1
 1 // 2 = 0,  dư 1     (phần nguyên về 0 — dừng)
```

Bốn số dư, đọc theo thứ tự vừa ghi: `1, 0, 1, 1`. Đó **chưa phải** đáp án —
số dư đầu tiên (1) là cột phải cùng, đáng giá nhỏ nhất; số dư cuối cùng (1)
là cột trái cùng, đáng giá lớn nhất. Muốn viết đúng chiều (trái cùng trước,
phải cùng sau, như mọi con số bạn từng viết), phải **đảo ngược** thứ tự vừa
ghi: `1, 1, 0, 1`.

Bằng Python:

```python title=readonly
n = 13
bit_list = []
while n > 0:
    du = n % 2
    bit_list.append(str(du))
    n = n // 2

bit_list.reverse()
day_bit = "".join(bit_list)
print(day_bit)
```

Máy in ra:

```text title=readonly
1101
```

Đọc lại bằng bài trước để kiểm: `1101` là 8 + 4 + 0 + 1 = 13. Khớp — chia rồi
đảo ngược đưa bạn về đúng chỗ xuất phát.
::::

::::predict{#doan-bit-cua-20 commitOnce}
Byte muốn viết số 20 ra bit, dùng đúng cách vừa học: chia liên tiếp cho hai,
ghi số dư, rồi đảo ngược.

**Trước khi chia tay**, bạn đoán kết quả là dãy bit nào?

:::opt{correct}
10100
:::

:::opt
00101 — chia đúng từng bước, chỉ là không đảo ngược lại.
::why
Gần đúng ở chỗ năm số dư bạn tính — `0, 0, 1, 0, 1` — đúng là năm số dư thật
của phép chia 20 liên tiếp cho hai, không sai số nào.

Chỗ lệch là dãy số dư, đọc theo đúng thứ tự tính ra, đi từ cột phải cùng tới
cột trái cùng — tức NGƯỢC chiều với cách viết một con số. Bỏ bước đảo ngược
thì cột trái cùng bị đặt nhầm sang bên phải, và ngược lại.
::
:::

:::opt
0100 — bốn số dư, dừng lại khi phần nguyên vừa còn 1.
::why
Gần đúng ở chỗ ba số dư đầu — `0, 1, 0` — bạn tính đúng, không lệch bước
nào.

Chỗ lệch là dừng sớm mất một bước. Vòng lặp phải chạy tới khi phần nguyên về
**0**, không phải dừng ngay khi nó **còn 1** — vì `1 // 2` vẫn còn một câu
hỏi phải hỏi: 1 có chia hết cho hai không (không, dư 1), và phần nguyên lúc
đó mới thật sự về 0. Thiếu bước cuối này thì mất luôn chữ số cao giá nhất,
`10100` hụt còn `0100` — một dãy ngắn hơn cho một con số y hệt.
::
:::

:::opt
10101 — cách làm đúng, chỉ là tính trên số 21.
::why
Gần đúng ở chỗ cách làm không sai một bước nào: chia cho hai, ghi dư, đảo
ngược — làm đúng hệt bài vừa học.

Chỗ lệch nằm ở con số đem chia. `10101` là dãy bit của 21 (16 + 4 + 1),
không phải của 20. Đúng thuật toán trên sai đầu vào vẫn ra sai đáp án — thuật
toán không tự sửa được một con số bị đọc nhầm.
::
:::
::::

::::code{#viet-22-ra-bit}
Máy đếm khách của cô Bảy cần tự viết con số **22** ra bit để thắp đúng đèn.
Byte đã gõ sẵn khung vòng lặp, chỉ bỏ trống hai chỗ: chỗ lấy SỐ DƯ, và chỗ
lấy PHẦN NGUYÊN để đi tiếp.

```python title=starter
so = 22

bit_list = []
n = so
while n > 0:
    du = ___
    bit_list.append(str(du))
    n = ___

bit_list.reverse()
day_bit = "".join(bit_list)
print(day_bit)
```

```python title=solution
so = 22

bit_list = []
n = so
while n > 0:
    du = n % 2
    bit_list.append(str(du))
    n = n // 2

bit_list.reverse()
day_bit = "".join(bit_list)
print(day_bit)
```

```python title=test
# 22 = 16 + 4 + 2 -> 10110. Năm chữ số, và tổng đọc lại đúng bằng 22.
assert so == 22, "con số đem viết ra bit là 22 — chỗ trống không được sửa nó"
assert n == 0, "vòng lặp phải chạy tới khi phần nguyên về hẳn 0, không dừng sớm hơn"
assert len(bit_list) == 5, "22 cần đúng năm cột (16, 8, 4, 2, 1) mới đủ chỗ chứa"
assert day_bit == "10110", "22 chia liên tiếp cho hai, đảo ngược số dư lại, phải ra đúng 10110"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm ngay dòng nói về `du` — nó hỏi "còn dư mấy sau khi chia hai". Chỗ trống thứ hai nằm ở dòng cập nhật `n` cho lượt kế tiếp — nó hỏi "chia hết được bao nhiêu lần", tức bỏ luôn phần dư vừa lấy.
- kind: strategy
  body: "Hai câu hỏi ấy có tên riêng từ Realm 1: \"còn dư mấy\" là dấu `%`, \"chia hết bao nhiêu lần\" là dấu `//`. Cả hai đều chia `n` cho 2 — chỉ khác đằng nào giữ phần dư, đằng nào giữ phần nguyên."
- kind: one-line
  body: "Chỗ trống thứ nhất là `n % 2`, chỗ trống thứ hai là `n // 2`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^10110\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
10110. Đèn máy đếm khách vừa học cách tự viết ra chính con số nó đang giữ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Máy đếm khách của cô Bảy có đúng **tám đèn** — không hơn, không kém, y hệt
bảng công tắc bạn gặp từ những bài đầu tiên. Tám đèn ấy, đọc và viết đúng
cách bạn vừa học hai bài liền, chứa được những con số nào?

Chắc chắn có một con số lớn nhất mà tám đèn còn thắp nổi — quá con số đó thì
hết chỗ. Con số ấy là bao nhiêu, và vì sao?

Bài sau trả lời — và lần này bạn tự tính ra, không ai nói trước.
::::

::::checkpoint{mastery=0.8}
::::
