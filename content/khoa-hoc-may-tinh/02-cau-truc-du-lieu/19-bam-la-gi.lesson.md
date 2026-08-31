---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.bam-la-gi
title: "Băm một cái tên thành một con số"
summary: "Một hàm băm đổi một chuỗi thành một con số bằng cách cộng dồn mã byte UTF-8 rồi lấy dư cho một cỡ mảng — luật duy nhất nó phải giữ: cùng một chuỗi luôn băm ra cùng một số."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ds.hash-function]
requires: [mem.utf8-bytes, core.modulo, core.function-def, core.function-return, core.function-call, core.function-parameter, core.variable, core.assignment, core.fstring, core.builtin-function]
concepts: [ds.hash-function]
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
Suốt mấy bài vừa qua, tìm một thứ trong danh sách liên kết luôn phải ĐI BỘ
từ đầu. Hôm nay Byte thử một trò khác hẳn: biến một cái TÊN thành một con
số, xem chuyện gì xảy ra.
::::

::::explain{#cong-thuc-bam}
Trong danh sách liên kết (cụm vừa xong), muốn biết "Lan" có trong danh sách
không, bạn phải đi qua từng nút, so tên từng cái một, cho tới khi thấy hoặc
hết danh sách. Mảng (cụm 1) thì khác: có ĐỊA CHỈ, bạn nhảy thẳng tới ô số
`i` bằng một phép tính (bài 2), không cần dò.

Câu hỏi hôm nay: có cách nào biến một cái TÊN — "Lan", "An", "Bình" — thành
một CON SỐ, để rồi dùng con số đó nhảy thẳng vào một mảng, giống hệt cách
mảng nhảy thẳng tới ô `i`? Nếu có, tra một cái tên sẽ không cần đi bộ nữa.

Việc biến một chuỗi thành một con số như vậy gọi là **băm** (hash). Một
**hàm băm** nhận vào một chuỗi, trả về một con số. Công thức băm đơn giản
nhất — đủ để hiểu ý tưởng — làm đúng ba bước:

1. Mã hoá chuỗi thành byte UTF-8 (T3.1 bài 18: mỗi ký tự chiếm 1 byte trở
   lên, ký tự tiếng Việt có dấu thường chiếm nhiều hơn 1 byte).
2. Cộng dồn giá trị của TẤT CẢ các byte đó lại thành một tổng.
3. Lấy dư (`%`, `core.modulo`) của tổng đó cho một con số — thường là cỡ
   của mảng bạn định dùng. Phép `%` đảm bảo kết quả luôn nằm trong khoảng
   `[0, cỡ - 1]`, vừa khít làm chỉ số mảng, dù chuỗi ngắn hay dài bao nhiêu.

Viết thành hàm Python:

```python title=readonly
def bam(ten, co_bang):
    ma_byte = ten.encode("utf-8")
    tong = sum(ma_byte)
    return tong % co_bang
```

Đây là một công thức băm ĐƠN GIẢN, dựng để học ý tưởng — hàm `hash()` thật
của Python phức tạp hơn nhiều (bài 24 sẽ nói tới). Nhưng ý tưởng cốt lõi
giống hệt: biến một chuỗi thành một con số, và luật duy nhất bắt buộc phải
giữ là:

> **Cùng một chuỗi, gọi hàm băm bao nhiêu lần cũng ra đúng cùng một số.**

Không có luật nào bắt hai chuỗi KHÁC NHAU phải băm ra hai số khác nhau —
chuyện đó (bài 21 gọi là "đụng độ") vẫn được phép xảy ra. Luật duy nhất là
về MỘT chuỗi với chính nó: hôm nay băm ra sao thì lát nữa, ngày mai, năm
sau, băm lại vẫn phải ra y hệt.
::::

::::example{#bam-vai-cai-ten}
Byte định làm một cuốn danh bạ điện thoại — tra tên ra số, không cần dò
từng trang. Trước khi dựng danh bạ (bài sau), thử công thức băm trên vài
cái tên, với cỡ bảng `co_bang = 10`:

```python title=readonly
def bam(ten, co_bang):
    ma_byte = ten.encode("utf-8")
    tong = sum(ma_byte)
    return tong % co_bang

print(bam("An", 10))
print(bam("Hoa", 10))
print(bam("Lan", 10))
print(bam("An", 10))
```

```text title=readonly
5
0
3
5
```

`"An"` mã hoá UTF-8 ra đúng 2 byte, `65` (`A`) và `110` (`n`) — tổng
`175`, dư cho 10 là `5`. `"Hoa"` và `"Lan"` theo đúng ba bước ấy ra `0` và
`3`. Dòng cuối gọi lại `bam("An", 10)` — không đổi gì cả — và kết quả vẫn
là `5`, y hệt lần đầu. Đó chính là luật bắt buộc: cùng chuỗi, cùng cỡ bảng,
luôn ra cùng một số.

Chú ý: `"An"` băm ra `5`, khác hẳn `0` của `"Hoa"` và `3` của `"Lan"` — ba
cái tên, ba con số khác nhau. Nhưng đó là MAY MẮN của bộ ba này, không phải
luật. Bài 21 sẽ cho thấy hai tên khác nhau vẫn có thể băm trùng một số.
::::

::::predict{#bam-ten-co-dau commitOnce}
Byte băm tên "Sơn" — có dấu — với `co_bang = 10`, đúng ba bước vừa học:
mã hoá UTF-8, cộng dồn byte, lấy dư cho 10.

**Trước khi chạy**, bạn đoán `bam("Sơn", 10)` bằng bao nhiêu? Gợi ý: nhớ
lại T3.1 bài 18 — chữ có dấu tiếng Việt chiếm NHIỀU HƠN một byte khi mã hoá
UTF-8.

:::opt{correct}
`2`
:::

:::opt
`0`
::why
Gần đúng ở chỗ bạn có cộng dồn đúng BA con số, một cho mỗi ký tự trong
"Sơn" — không bỏ sót ký tự nào.

Chỗ lệch là bạn cộng mã ĐIỂM UNICODE của từng ký tự (`ord('S')`, `ord('ơ')`,
`ord('n')`), không phải BYTE thật sau khi mã hoá UTF-8. Chữ `ơ` một mình
chiếm HAI byte khi mã hoá UTF-8 (giống "ở" ở T3.1 bài 18 chiếm 3 byte,
không phải 1) — cộng theo ký tự bỏ sót đúng chỗ lệch này, tổng ra `610`
thay vì `552`, dư cho 10 ra `0` thay vì `2`.
::
:::

:::opt
`3`
::why
Gần đúng ở chỗ `3` là một con số có THẬT trong bài toán này — "Sơn" có
đúng 3 KÝ TỰ (`S`, `ơ`, `n`).

Chỗ lệch là câu hỏi đòi TỔNG GIÁ TRỊ của các byte rồi lấy dư, không phải
ĐẾM có bao nhiêu ký tự. Đếm ký tự và cộng dồn byte là hai việc khác hẳn
nhau, dù cùng làm trên một chuỗi.
::
:::

:::opt
`4`
::why
Gần đúng ở chỗ `4` cũng có THẬT — "Sơn" mã hoá UTF-8 ra đúng 4 byte (`S`,
hai byte của `ơ`, `n`).

Chỗ lệch là câu hỏi đòi TỔNG GIÁ TRỊ của 4 byte đó cộng lại rồi lấy dư,
không phải ĐẾM có bao nhiêu byte. Đếm số byte và cộng giá trị các byte lại
là hai việc khác nhau.
::
:::
::::

::::code{#viet-ham-bam}
Byte gõ sẵn khung hàm `bam`, chừa trống đúng một dòng — dòng cộng dồn giá
trị các byte lại thành một tổng. Bạn điền vào, rồi kiểm bằng ba cái tên
khác nhau: "An", "Hoa", "Nam".

```python title=starter
def bam(ten, co_bang):
    ma_byte = ten.encode("utf-8")
    tong = ___
    return tong % co_bang

print(bam("An", 10))
print(bam("Hoa", 10))
print(bam("Nam", 10))
```

```python title=solution
def bam(ten, co_bang):
    ma_byte = ten.encode("utf-8")
    tong = sum(ma_byte)
    return tong % co_bang

print(bam("An", 10))
print(bam("Hoa", 10))
print(bam("Nam", 10))
```

```python title=test
assert bam("An", 10) == 5, "bam('An', 10) phải ra 5 — 'A' là byte 65, 'n' là byte 110, tổng 175, dư cho 10 là 5"
assert bam("Hoa", 10) == 0, "bam('Hoa', 10) phải ra 0"
assert bam("Nam", 10) == 4, "bam('Nam', 10) phải ra 4"
assert bam("Lan", 10) == 3, "bam('Lan', 10) phải ra 3 — kiểm thêm một cái tên khác hẳn"
assert bam("An", 10) == bam("An", 10), "gọi bam trên đúng cùng một chuỗi hai lần phải ra đúng cùng một số"
assert bam("Sơn", 10) == 2, "bam('Sơn', 10) phải ra 2 — 'Sơn' mã hoá UTF-8 ra 4 byte ([83, 198, 161, 110], vì 'ơ' chiếm 2 byte), tổng 552, dư cho 10 là 2. Nếu chỗ trống cộng theo ord() từng KÝ TỰ thay vì từng BYTE UTF-8 (đúng chỗ lệch dự đoán ở predict phía trên), 'An', 'Hoa', 'Nam', 'Lan' đều thuần ASCII nên hai cách tính trùng nhau — chỉ 'Sơn' có dấu mới lộ ra chỗ sai"
```

:::hints
- kind: attention
  body: Dòng ngay phía trên chỗ trống đã tạo `ma_byte` — một đối tượng `bytes`, tức một dãy các số nguyên (mỗi số là giá trị một byte). Chỗ trống cần CỘNG DỒN cả dãy đó lại thành một con số duy nhất.
- kind: strategy
  body: Python có sẵn một hàm cộng dồn mọi phần tử của một dãy số lại với nhau — bạn đã dùng nó để cộng một danh sách số ở R1. Gọi đúng hàm đó trên `ma_byte`.
- kind: one-line
  body: "Điền `sum(ma_byte)` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^5\\n0\\n4\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cái tên, ba con số, và gọi lại "An" bao nhiêu lần cũng ra đúng 5. Bạn
vừa có công cụ biến một cái tên thành một con số — ổn định, lặp lại được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn giờ có một hàm nhận một cái TÊN, trả về một CON SỐ. Nhưng con số đó,
tự nó, chưa làm được gì cả — nó chỉ nằm đó, một con số suông.

Mảng (cụm 1) thì khác: số `i` của nó ĐI THẲNG vào ô thứ `i`. Nếu con số
`bam("Lan", 10)` cũng dùng làm CHỈ SỐ vào một mảng — giống hệt cách mảng
vẫn dùng chỉ số — thì chuyện gì sẽ xảy ra?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
