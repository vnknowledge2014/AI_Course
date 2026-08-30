---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.khi-so-tran-ra-ngoai
title: Khi con số tràn ra khỏi ô
summary: "Tám ô chỉ đếm được 256 kiểu; khi một phép tính cần thêm một kiểu nữa, phần dư bị cắt và giá trị vòng lại từ đầu — Python tự tránh chuyện này, nhưng byte giả mà bạn tự dựng bằng `& 0xff` thì không."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.overflow]
requires: [mem.twos-complement]
concepts: [mem.tran-so, mem.byte-gia, mem.vong-lai-tu-dau]
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
Tám ô của mình chỉ đếm tới một mức. Nhét thêm chút nữa thì sao nhỉ?
::::

::::explain{#tam-o-day-roi-thi-sao}
Bài trước, con số 251 đứng vào chỗ của −5 trong tám cái ô: `-5 & 0xff` cho
ra `251`, tức `0b11111011`. Cộng nó với 5 — cộng số đối với chính số gốc —
ra đúng 256. Nhưng khoan: trong tám ô, kết quả bạn nhìn thấy lại là `0`,
không phải `256`.

Chuyện gì xảy ra với con số 256? Nó không biến mất bằng phép màu nào cả. Nó
rớt ra ngoài, vì tám cái ô chỉ đếm được đúng 256 KIỂU — đánh số từ `0` tới
`255` — mà `256` cần một kiểu thứ 257, một ô thứ chín mà tám ô kia không
có.

Đây gọi là **tràn** (tiếng Anh: *overflow*): khi kết quả một phép tính cần
nhiều chỗ hơn số ô đang có, phần vượt ra ngoài bị cắt mất, và giá trị còn
lại trong khung là phần dư sau khi cắt — không phải kết quả toán học thật.
::::

::::example{#dong-ho-cua-quan}
Cạnh cửa quán cô Bảy có một cái máy bấm đếm khách, loại vặn tay cũ, chỉ
hiện được BA chữ số. Khách thứ 998 bước vào, máy hiện `998`. Khách thứ 999,
máy hiện `999`. Khách thứ 1000 bước vào — máy không hiện `1000`, vì bánh
răng của nó không có chỗ cho chữ số thứ tư. Nó vặn về `000`.

Cái máy không hỏng. Nó làm đúng thứ nó được xây để làm: đếm hết vòng của nó
rồi quay lại đầu. Chỉ có điều con số nó hiện ra sau khách thứ 1000 không
còn nói thật NGƯỜI THỨ MẤY đang đứng ở cửa nữa.

Tám cái ô của một byte cư xử giống hệt cái máy đó, chỉ khác chỗ nó đếm tới
`255` rồi vòng, không phải `999`. Dựng lại chuyện ấy bằng Python:

```python title=readonly
def cong_trong_mot_byte(a, b):
    """Cộng hai số, coi như kết quả chỉ có đúng một byte (8 ô) để đứng."""
    tong_that = a + b
    return tong_that & 0xff


print(cong_trong_mot_byte(250, 4))
print(cong_trong_mot_byte(255, 1))
print(cong_trong_mot_byte(250, 10))
```

Máy in ra:

```text title=readonly
254
0
4
```

`250 + 4 = 254` — còn dư chỗ, tám ô chứa thoải mái, không có gì lạ.

`255 + 1 = 256` — hết chỗ đúng khít. Ô thứ chín mà `256` cần không có, phần
đó rớt mất, còn lại `0`. Giống hệt cái máy bấm ở khách thứ 1000.

`250 + 10 = 260` — tràn nhiều hơn một chút. Phần vượt ra ngoài tám ô rớt
đi, phần còn nằm gọn trong tám ô là `4`.

Phép `& 0xff` chính là cách CHỦ Ý dựng lại cái khung tám-ô đó trong Python:
`0xff` là `0b11111111`, tám bit toàn số 1, nên `&` với nó chỉ giữ lại đúng
tám bit thấp nhất của kết quả — mọi bit cao hơn, tức mọi thứ vượt ra ngoài
tám ô, bị cắt bỏ.
::::

::::predict{#doan-tran-tiep commitOnce}
Hàm `cong_trong_mot_byte` ở trên vẫn còn nguyên. Byte gọi nó với hai số
mới:

```python
print(cong_trong_mot_byte(200, 100))
```

**Trước khi bấm chạy**, bạn đoán màn hình hiện số nào?

:::opt{correct}
44
:::

:::opt
300
::why
Gần đúng ở chỗ phép cộng: `200 + 100 = 300` đúng không sai một đơn vị nào.

Chỗ lệch là bạn dừng lại ở đó, quên mất dòng `return` không trả `tong_that`
suông — nó còn đi qua `& 0xff`. Tám ô không có chỗ cho `300`, nên phần vượt
ra ngoài bị cắt, và con số còn lại trong khung không phải `300` mà là `44`.
::
:::

:::opt
45
::why
Gần đúng ở chỗ bạn nhớ đúng: có một phép cắt xảy ra, và bạn còn nhớ đúng
tinh thần "quá mốc thì trừ mốc". Vấn đề chỉ nằm ở CÁI MỐC.

Tám ô đếm được 256 kiểu — từ `0` tới `255` — không phải 255 kiểu. Trừ nhầm
theo mốc `255` (`300 − 255 = 45`) là lặp lại đúng lỗi bài "tám ô là một
byte" đã cảnh: đếm số kiểu mà quên tính luôn kiểu `0`. Mốc đúng là `256`,
và `300 − 256 = 44`.
::
:::

:::opt
0
::why
Gần đúng ở chỗ bạn nhớ đúng: vừa nãy `255 + 1` tràn thì về `0` thật.

Chỗ lệch là bạn coi `0` như đích đến CỐ ĐỊNH của mọi lần tràn, trong khi nó
chỉ là kết quả của đúng MỘT cảnh — cảnh `256` khít đúng ranh giới. Tràn
không phải "về lại điểm xuất phát"; nó là "giữ lại phần dư sau khi cắt bớt
phần vượt". `300` tràn thì phần dư còn lại là `44`, không phải `0`.
::
:::
::::

::::code{#tu-viet-phep-cong-tran}
Byte muốn chính bạn viết hàm `cong_trong_mot_byte` — không xem lời giải
nữa, tự dựng lấy khung tám-ô.

Dòng `tong_that = a + b` đã tính đúng tổng thật. Việc còn lại của bạn là ép
tổng đó vào khung tám ô: chỉ giữ lại phần nằm gọn trong 256 kiểu, cắt bỏ
phần vượt ra ngoài.

```python title=starter
def cong_trong_mot_byte(a, b):
    """Cộng hai số, coi như kết quả chỉ có đúng một byte (8 ô) để đứng."""
    tong_that = a + b
    return ___


print(cong_trong_mot_byte(250, 4))
print(cong_trong_mot_byte(255, 1))
print(cong_trong_mot_byte(250, 10))
```

```python title=solution
def cong_trong_mot_byte(a, b):
    """Cộng hai số, coi như kết quả chỉ có đúng một byte (8 ô) để đứng."""
    tong_that = a + b
    return tong_that & 0xff


print(cong_trong_mot_byte(250, 4))
print(cong_trong_mot_byte(255, 1))
print(cong_trong_mot_byte(250, 10))
```

```python title=test
# Năm cảnh, cố ý trải khắp ranh giới của tám ô — chỉ một cảnh không giữ nổi
# cửa: quên khung thì cảnh không-tràn vẫn đúng, chỉ cảnh CHẠM hoặc VƯỢT
# ranh giới mới lộ ra chỗ hỏng.
assert cong_trong_mot_byte(10, 20) == 30, "10 + 20 = 30, còn dư chỗ trong tám ô, không có gì để cắt"
assert cong_trong_mot_byte(250, 4) == 254, "250 + 4 = 254, vẫn nằm gọn trong tám ô — 254 là kiểu áp chót"
assert cong_trong_mot_byte(200, 55) == 255, "200 + 55 = 255, đúng kiểu CUỐI CÙNG mà tám ô còn chứa được — chưa tràn"
assert cong_trong_mot_byte(255, 1) == 0, "255 + 1 = 256, khít đúng ranh giới — ô thứ chín không có, phần dư sau khi cắt là 0"
assert cong_trong_mot_byte(250, 10) == 4, "250 + 10 = 260, vượt ranh giới 4 đơn vị — phần dư sau khi cắt phải là 4, không phải 260 hay 0"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ dòng `return`. Đề đã tính sẵn `tong_that`; việc của bạn là ép nó vào khung tám ô, không phải tính lại tổng.
- kind: strategy
  body: Tám ô đếm được 256 kiểu, và `0xff` (tức `0b11111111`, tám số 1 liền nhau) đúng là tấm khuôn của khung đó. Ép một số vào khung bằng cách giữ lại đúng tám bit thấp nhất của nó — dùng đúng phép mà bài trước dùng để nhốt số âm vào tám ô.
- kind: one-line
  body: 'Điền `tong_that & 0xff`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^254\n0\n4\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
255 cộng 1 ra 0. Không phải máy hỏng — chỉ là tám ô đã hết chỗ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái `& 0xff` trong hàm của bạn là DO BẠN TỰ THÊM VÀO — không ai bắt Python
phải cắt gọt gì cả. Bỏ nó đi, để phép cộng chạy tự nhiên:

```python
print(255 + 1)
```

Track T2.2 đã cho bạn cộng và nhân những con số có hàng chục chữ số mà
không hề gặp lỗi tràn nào. Dòng trên cũng vậy: nó in ra `256`, không phải
`0`. Không tràn, không cắt, không vòng lại.

Vậy câu hỏi là: bộ nhớ của máy tính vẫn chỉ có từng ô một, đúng như tám ô
bạn vừa thấy hết chỗ. Vì sao con số PYTHON THƯỜNG — không mask, không tự
viết gì thêm — lại không bao giờ hết chỗ như vậy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
