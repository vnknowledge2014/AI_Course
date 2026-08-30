---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.mot-phan-muoi-trong-he-hai
title: 1/10 trong hệ hai là số vô hạn tuần hoàn
summary: "Đổi mỗi bước chia tay của bài trước từ nhân 10 sang nhân 2, và thấy `0,1` — con số bạn gõ mỗi ngày — không bao giờ viết hết. Đây là lời giải cho `0.1 + 0.2`."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [mem.binary-fraction]
requires: [mem.repeating-fraction]
concepts: [mem.phan-so-nhi-phan, mem.chuoi-hex-float]
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
Đổi mỗi bước từ "nhân 10" sang "nhân 2" xem `0,1` có viết hết không nhé.
::::

::::explain{#doi-co-so}
Thuật toán bài trước không hề nhắc tới số 10 ở bất cứ chỗ nào bắt buộc.
"Nhân dư lên 10" chỉ vì hệ mười CÓ MƯỜI chữ số — 0 tới 9 — nên mỗi bước
nhân dư lên đúng bằng số chữ số đó. Hệ hai chỉ có HAI chữ số — 0 và 1 —
nên ở hệ hai, mỗi bước nhân dư lên 2, không phải 10.

Ba bước còn lại y nguyên: nhân dư lên cơ số, thương là chữ số tiếp theo,
dư của phép chia đó là dư mới, dư về 0 thì dừng.

Thử với `1/10` — đúng con số `0,1` bạn gõ hằng ngày:

```text title=readonly
1 ÷ 10, viết trong hệ hai
 dư = 1
  1 × 2 = 2  ; 2 ÷ 10 = 0, dư 2
  2 × 2 = 4  ; 4 ÷ 10 = 0, dư 4
  4 × 2 = 8  ; 8 ÷ 10 = 0, dư 8
  8 × 2 = 16 ; 16 ÷ 10 = 1, dư 6
  6 × 2 = 12 ; 12 ÷ 10 = 1, dư 2
  2 × 2 = 4  ; 4 ÷ 10 = 0, dư 4   ← đã gặp dư 4 này ở bước hai
  ...
```

Dư từng gặp theo thứ tự: 2, rồi 4, rồi 8, rồi 6, rồi 2 — quay lại đúng dư
đã gặp ở bước đầu tiên. Từ đây dãy chữ số lặp lại y hệt, mãi mãi. Không có
bước nào dư về 0. `1/10` không viết hết trong hệ hai, cũng như `1/3` không
viết hết trong hệ mười.
::::

::::example{#chay-lai-ham-cu-doi-co-so}
Hàm bài trước thêm đúng MỘT tham số — cơ số — mặc định là 10 để mọi lời
gọi cũ vẫn chạy y nguyên, không phải sửa lại:

```python title=readonly
def chu_so_thap_phan(tu, mau, so_buoc, co_so=10):
    du = tu % mau
    chu_so = []
    for _ in range(so_buoc):
        du = du * co_so
        chu_so.append(du // mau)
        du = du % mau
    return chu_so, du

print(chu_so_thap_phan(1, 10, 8, co_so=2))
print(chu_so_thap_phan(1, 4, 8, co_so=2))
```

Máy in ra:

```text title=readonly
([0, 0, 0, 1, 1, 0, 0, 1], 6)
([0, 1, 0, 0, 0, 0, 0, 0], 0)
```

Dòng đầu là `1/10` trong hệ hai — tám chữ số đầu của một dãy sẽ còn lặp lại
`0011` mãi, và dư cuối là 6, khác 0. Đúng thứ bạn vừa chia bằng tay.

Dòng hai là `1/4`. Số 4 chỉ có đúng một thừa số là 2 — viết bằng `4 = 2 ×
2` — khớp thẳng với cơ số của hệ hai, nên nó dừng gọn: `0,01` rồi toàn số
0. `1/4` viết hết trong CẢ hai hệ, mười lẫn hai. Không phải mọi phân số
đều khó.
::::

::::predict{#doan-nhung-phan-so-nao-dung commitOnce}
Ở hệ mười, chỉ những phân số có mẫu số chỉ gồm thừa số 2 và 5 mới viết hết
— khớp với hai thừa số của cơ số hệ mười (`10 = 2 × 5`). Hệ hai chỉ có
đúng một thừa số để khớp: 2.

Trong bốn phân số `1/2`, `1/3`, `1/5`, `1/6`, bao nhiêu phân số viết hết
(dừng) khi viết trong hệ hai?

**Trước khi tính**, bạn đoán câu trả lời nào đúng?

:::opt{correct}
Chỉ một — `1/2`.
:::

:::opt
Hai — `1/2` và `1/5`.
::why
Gần đúng ở chỗ bạn nhớ đúng NGUYÊN TẮC chung: mẫu số phải chỉ gồm những
thừa số trùng với cơ số thì phân số mới dừng. Nguyên tắc ấy đúng, và nó
chính là điều cả hai bài — 13 và 14 — cùng dựng lên.

Chỗ lệch là đem đúng nguyên tắc ấy áp cho SAI cơ số. `2` và `5` là hai
thừa số của hệ MƯỜI (`10 = 2 × 5`) — đó là chuyện của bài trước. Hệ HAI
chỉ có mỗi số 2 làm cơ số, nên chỉ mẫu số nào toàn thừa số 2 mới dừng. Số
5 không có mặt trong hệ hai để mà khớp.
::
:::

:::opt
Cả bốn cái đều dừng.
::why
Gần đúng ở chỗ cả bốn phân số này đều "gọn" theo cách nhìn quen thuộc của
hệ mười — mẫu số nhỏ, không có gì trông rắc rối.

Chỗ lệch: "gọn trong hệ mười" không có nghĩa "gọn trong hệ hai". `1/3`,
`1/5`, `1/6` đều có một thừa số không phải 2 lẩn trong mẫu số (lần lượt là
3, 5, và 3), nên phần dư của chúng không bao giờ về 0 khi nhân lên 2 — y
hệt cách `1/3` không về 0 khi nhân lên 10.
::
:::

:::opt
Không cái nào dừng.
::why
Gần đúng ở chỗ bạn đã thấm đúng bài học: hệ hai "nghèo" hơn hệ mười, chỉ
có một thừa số để dùng, nên nhiều phân số quen thuộc bỗng không dừng nữa.

Chỗ lệch là đẩy điều đó đi quá xa. `1/2` chỉ có đúng MỘT thừa số 2, và
hệ hai có sẵn thừa số ấy — `1/2` viết trong hệ hai là `0,1`, dừng ngay ở
chữ số đầu tiên, gọn hơn cả cách viết trong hệ mười (`0,5`, cũng dừng,
nhưng bằng hai chữ số).
::
:::
::::

::::code{#kiem-tra-mot-phan-muoi}
Gọi hàm với `co_so=2` cho hai phân số — `1/10` và `1/4` — rồi hỏi máy
thẳng: phân số này có DỪNG không? Câu hỏi ấy chính là "dư cuối có bằng 0
không".

```python title=starter
def chu_so_thap_phan(tu, mau, so_buoc, co_so=10):
    du = tu % mau
    chu_so = []
    for _ in range(so_buoc):
        du = du * co_so
        chu_so.append(du // mau)
        du = du % mau
    return chu_so, du

chu_so_1_10, du_1_10 = chu_so_thap_phan(1, 10, 8, co_so=2)
chu_so_1_4, du_1_4 = chu_so_thap_phan(1, 4, 8, co_so=2)

print(chu_so_1_10)
print(___)
print(chu_so_1_4)
print(___)
```

```python title=solution
def chu_so_thap_phan(tu, mau, so_buoc, co_so=10):
    du = tu % mau
    chu_so = []
    for _ in range(so_buoc):
        du = du * co_so
        chu_so.append(du // mau)
        du = du % mau
    return chu_so, du

chu_so_1_10, du_1_10 = chu_so_thap_phan(1, 10, 8, co_so=2)
chu_so_1_4, du_1_4 = chu_so_thap_phan(1, 4, 8, co_so=2)

print(chu_so_1_10)
print(du_1_10 == 0)
print(chu_so_1_4)
print(du_1_4 == 0)
```

```python title=test
assert chu_so_1_10 == [0, 0, 0, 1, 1, 0, 0, 1], "tám chữ số đầu của 1/10 trong hệ hai phải đúng dãy này — kiểm lại phần cho sẵn, đừng sửa nó"
assert du_1_10 == 6, "dư cuối của 1/10 sau tám bước phải là 6, khác 0 — 1/10 KHÔNG dừng trong hệ hai"
assert chu_so_1_4 == [0, 1, 0, 0, 0, 0, 0, 0], "tám chữ số đầu của 1/4 trong hệ hai phải đúng dãy này"
assert du_1_4 == 0, "dư cuối của 1/4 phải là 0 — 1/4 CÓ dừng trong hệ hai"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trong `print(___)`, ngay dưới mỗi danh sách chữ số. Việc của chúng là trả lời câu "phân số này có dừng không" — mà câu trả lời ấy đọc thẳng ra được từ biến `du_1_10` hay `du_1_4` đứng ngay phía trên, không cần tính gì thêm.
- kind: strategy
  body: "Dừng nghĩa là dư cuối bằng 0. Viết thành một câu hỏi có–không bằng dấu `==`, đúng cách R1 đã dạy — so sánh biến dư với số 0. Đừng gõ thẳng `True` hay `False`: hai phân số này cho hai câu trả lời NGƯỢC nhau, nên một chữ gõ cứng chỉ đúng được nhiều nhất một trong hai dòng."
- kind: one-line
  body: "Chỗ trống thứ nhất là `du_1_10 == 0`, chỗ trống thứ hai là `du_1_4 == 0`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: mỗi chỗ trống phải hỏi máy DỰA VÀO chính con số dư nó vừa tính ra (biến `du_1_10` hay `du_1_4`) — gõ thẳng True/False là đoán, không phải hỏi
  requireAst:
  - kind: uses-name, target: du_1_10, min: 1
  - kind: uses-name, target: du_1_4, min: 1
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^\[0, 0, 0, 1, 1, 0, 0, 1\]\nFalse\n\[0, 1, 0, 0, 0, 0, 0, 0\]\nTrue\s*$
:::
::::

::::explain{#day-la-cau-tra-loi}
Giờ có thể trả lời thẳng câu hỏi R1 để lại: vì sao `0.1 + 0.2` không ra
`0.3`?

`0,1` viết trong hệ hai là một dãy vô hạn tuần hoàn — y như `1/3` viết
trong hệ mười. Máy không có chỗ chứa vô hạn. Nó phải CẮT dãy ấy lại ở một
chỗ nào đó, và giữ một con số RẤT GẦN `0,1`, không phải đúng `0,1`. `0,2`
cũng vậy — dãy nhị phân của nó cũng không dừng. Cộng hai con số đã bị cắt
thì ra một con số cũng bị cắt, và phần cắt ấy không nhất thiết khớp với
phần cắt của `0,3`.

Python cho xem đúng chỗ bị cắt, bằng một phương thức tên `.hex()`:

```python title=readonly
print((0.1).hex())
```

```text title=readonly
0x1.999999999999ap-4
```

Cái đuôi `999999999999a` không phải một dãy số ngẫu nhiên. Nó là dấu vết
của đúng dãy tuần hoàn bạn vừa chia bằng tay — chỉ viết gọn theo hệ
mười-sáu thay vì hệ hai, và bị cắt lại ở chữ số cuối cùng. Bài sau mổ xẻ
từng phần của chuỗi này.
::::

::::byte{trigger=success mood=happy pose=jump}
`1/10` không dừng trong hệ hai. Đó là lý do thật, không phải máy tính hư.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chuỗi `0x1.999999999999ap-4` mà Python vừa in ra có BA phần rõ rệt, ngăn
bởi ký tự `x`, dấu chấm, và ký tự `p`. Bạn đã đoán được phần nào là dãy
chữ số tuần hoàn bị cắt. Nhưng còn hai phần kia — nhất là con số `-4`
đứng sau chữ `p` — chúng nói lên điều gì?

Và vì sao cách ghi số này được gọi là "chấm ĐỘNG", trong khi số nguyên
đồng mà quán cô Bảy dùng suốt từ Realm 0 lại không cần một cái tên như
vậy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
