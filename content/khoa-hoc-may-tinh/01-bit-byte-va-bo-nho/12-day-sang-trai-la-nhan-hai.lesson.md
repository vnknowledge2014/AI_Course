---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.day-sang-trai-la-nhan-hai
title: Đẩy sang trái là nhân hai
summary: "Đẩy cả dãy bit sang trái một cột nhân giá trị lên gấp đôi — đúng luật 'kéo bảng vị trí' mà hệ mười đã dạy, chỉ khác con số nhân vì hệ hai đếm bằng luỹ thừa của 2."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.bit-shift]
requires: [mem.bitwise-and-or]
concepts: [mem.day-bit-trai, mem.day-bit-phai, mem.nhan-doi-qua-cot]
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
Đẩy cả dãy số sang một bên, không đổi chữ số — mà giá trị vẫn đổi.
::::

::::explain{#keo-bang-vi-tri-o-he-hai}
Track T2.1, bài "bảng vị trí kéo sang phải" dạy một luật: kéo cả bảng vị
trí của hệ MƯỜI sang phải một cột thì giá trị CHIA cho mười — mỗi bó mười
hạt mở ra thành mười phần lẻ hơn.

Bài "đọc một dãy bit ra số" ở đầu track này đã chốt: trong hệ HAI, mỗi cột
cũng đứng ở một luỹ thừa, chỉ không phải luỹ thừa của 10 mà là luỹ thừa của
**2** — `1, 2, 4, 8, 16, ...`. Cột này luôn gấp ĐÔI cột ngay bên phải nó,
không gấp mười.

Vậy nếu kéo cả bảng vị trí của hệ hai — không phải sang phải, mà sang
TRÁI — mỗi chữ số dịch qua đúng một cột đáng giá gấp đôi. Giá trị của cả
dãy phải nhân hai.

Python có một phép làm đúng việc "đẩy cả dãy bit sang trái N cột": `<<`,
gọi là **phép đẩy bit**. `x << n` đẩy mọi bit của `x` sang trái `n` cột,
độn thêm `n` số 0 vào bên phải để lấp chỗ trống. Đẩy một cột thì nhân hai
một lần; đẩy `n` cột thì nhân hai LIÊN TIẾP `n` lần, tức nhân `2**n`.

Chiều ngược lại là `>>`, đẩy sang PHẢI — đúng luật "kéo bảng vị trí sang
phải" của T2.1, chỉ đổi mẫu số từ mười thành hai: `x >> n` chia `x` cho
`2**n`, bỏ luôn phần dư nếu có.
::::

::::example{#nhan-hai-bang-day-bit}
```python title=readonly
x = 5

print(bin(x))
print(bin(x << 1), x << 1)
print(bin(x << 3), x << 3)
```

Máy in ra:

```text title=readonly
0b101
0b1010 10
0b101000 40
```

`5` là `0b101`. Đẩy sang trái một cột, mỗi chữ số dịch qua một cột đáng
giá gấp đôi, thêm một số `0` giữ chỗ ở cột thấp nhất: `0b1010`, tức `10` —
đúng `5 × 2`.

Đẩy ba cột thì nhân hai ba lần liên tiếp: `5 × 2 × 2 × 2 = 40`, khớp với
`0b101000`.

Chiều ngược lại cũng đúng luật, chỉ có một chỗ khác hẳn hệ mười: chia cho
hai đôi khi MẤT một phần, vì hệ hai không có "cột lẻ hơn" nào để giữ phần
dư lại như dấu phẩy của T2.1.

```python title=readonly
print(20 >> 1)
print(7 >> 1)
```

```text title=readonly
10
3
```

`20 >> 1` chia hai vừa khít, ra `10`. Nhưng `7 >> 1` phải là `3,5` theo
toán thường — mà `>>` chỉ trả `3`. Bit thấp nhất của `7` (`0b111`) là `1`;
đẩy phải thì bit đó rớt hẳn ra ngoài dãy, mất luôn, không có chỗ nào giữ nó
lại. Đó là phần dư của phép chia, và nó không quay lại được.
::::

::::predict{#day-may-cot commitOnce}
```python
x = 9

print(x << 2)
print(x * 4)
print((x << 2) == (x * 4))
```

**Trước khi bấm chạy**, bạn đoán ba dòng trên in ra gì?

:::opt{correct}
36, rồi 36, rồi True
:::

:::opt
18, rồi 36, rồi False
::why
Gần đúng ở chỗ luật gốc bạn nhớ đúng: đẩy trái đúng là nhân hai, và
`9 × 2 = 18` không sai phép tính nào.

Chỗ lệch là con số `2` sau dấu `<<` không phải "nhân hai một lần" — nó là
SỐ CỘT cần đẩy. Đẩy 2 cột nghĩa là nhân hai hai LẦN LIÊN TIẾP:
`9 × 2 × 2 = 36`, khớp đúng `9 × 4`. Dừng lại sau một lần nhân là mới đẩy
được một cột, chưa đủ hai.
::
:::

:::opt
2, rồi 36, rồi False
::why
Gần đúng ở chỗ có một phép tính lặp lại hai lần theo con số `2` sau dấu
`<<` — cách bạn tính lặp cũng đúng kiểu, chỉ là bạn lặp bằng phép chia lấy
phần nguyên thay vì phép nhân.

Chỗ lệch là HƯỚNG. Dấu `<<` đẩy sang TRÁI, và bài này vừa chốt: trái là
NHÂN, không phải chia. Chia liên tiếp là việc của `>>`, dấu ngược lại.
::
:::

:::opt
11, rồi 36, rồi False
::why
Gần đúng ở chỗ bạn thấy đúng con số `2` xuất hiện trong phép tính và đem
nó vào — không bỏ qua chi tiết nào của đề bài cả.

Chỗ lệch là bạn đọc `2` như một LƯỢNG CỘNG THẲNG vào `9` (`9 + 2 = 11`),
trong khi nó là SỐ CỘT cần đẩy dãy bit sang, không phải một số hạng để
cộng. `<<` không hề cộng gì cả.
::
:::
::::

::::code{#day-trong-mot-byte}
Nối lại với khung tám-ô của hai bài trước: nếu một số CHỈ CÓ đúng một byte
để đứng, đẩy trái nó có tràn không?

```python title=starter
def day_trai_byte_gia(x, n):
    """Đẩy X sang trái N cột, coi như X chỉ có đúng một byte (8 ô) để đứng."""
    da_day = x << n
    return ___


print(day_trai_byte_gia(5, 3))
print(day_trai_byte_gia(1, 8))
```

```python title=solution
def day_trai_byte_gia(x, n):
    """Đẩy X sang trái N cột, coi như X chỉ có đúng một byte (8 ô) để đứng."""
    da_day = x << n
    return da_day & 0xff


print(day_trai_byte_gia(5, 3))
print(day_trai_byte_gia(1, 8))
```

```python title=test
# Sáu cảnh: hai cảnh không tràn, một cảnh tràn vừa qua ranh giới, một cảnh
# TRÀN HẾT SẠCH (bit duy nhất bị đẩy ra khỏi cả tám ô), một cảnh 0 làm đối
# chứng, và một cảnh TRÀN NHIỀU HƠN MỘT VÒNG — nơi "trừ 256 một lần nếu vượt"
# (đúng nửa việc, sai nửa còn lại) vẫn lộ ra: 200 << 3 = 1600, trừ 256 một
# lần còn 1344, vẫn lớn hơn 255 — phải cắt đúng tám bit thấp nhất (& 0xff),
# ra 64, không phải dừng lại ở 1344.
assert day_trai_byte_gia(5, 1) == 10, "5 << 1 = 10, còn dư chỗ trong tám ô, chưa chạm ranh giới"
assert day_trai_byte_gia(5, 3) == 40, "5 << 3 = 40, vẫn nằm gọn trong tám ô"
assert day_trai_byte_gia(200, 1) == 144, "200 << 1 = 400, vượt tám ô — phần dư sau khi cắt phải là 144, không phải 400"
assert day_trai_byte_gia(1, 8) == 0, "1 << 8 = 256, đúng một bit bị đẩy ra khỏi tám ô — không còn bit nào ở lại, kết quả là 0"
assert day_trai_byte_gia(0, 5) == 0, "0 đẩy đi đâu cũng vẫn là 0"
assert day_trai_byte_gia(200, 3) == 64, "200 << 3 = 1600 — tràn nhiều hơn một vòng 256. Trừ 256 đúng một lần chỉ còn 1344, vẫn chưa nằm gọn trong tám ô; phải giữ đúng tám bit thấp nhất, ra 64"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ dòng `return`. Biến `da_day` đã đẩy xong; việc còn lại là ép nó vào khung tám ô — đúng phép hai bài trước đã dùng cho phép cộng.
- kind: strategy
  body: Đẩy trái có thể đẩy bit ra khỏi cả tám ô — lúc đó `da_day` lớn hơn 255 thật sự, không còn nằm gọn trong một byte nữa. Khung tám ô chỉ giữ lại đúng tám bit thấp nhất của nó.
- kind: one-line
  body: 'Điền `da_day & 0xff`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^40\n0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bit đẩy ra khỏi tám ô là mất hẳn, không đường lùi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài này vừa cho bạn thấy `>>` cắt mất phần dư mỗi khi số bị đẩy là số lẻ:
`7 >> 1` mất đứt nửa đơn vị, không có cách nào lấy lại. Chia cho 2 trong hệ
hai mà còn có lúc không viết hết.

Vậy đặt câu hỏi này sang một hệ khác hẳn, một hệ bạn đã quen thuộc từ nhỏ:
chia `1` cho `3` trong hệ MƯỜI. Viết `1 ÷ 3` ra bằng những chữ số quen
thuộc — `0,333...` — bạn có bao giờ viết HẾT được nó không, hay luôn còn
một phần dư chưa chỗ nào chứa hết?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
