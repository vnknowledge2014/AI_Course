---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.cung-mot-so-ba-cach-viet
title: Cùng một con số, ba cách viết
summary: Python đọc thẳng số viết theo hệ hai, hệ tám hay hệ mười sáu qua ba tiền tố `0b`, `0o`, `0x` — đọc xong thì cách viết ấy bị quên ngay, chỉ còn lại giá trị.
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.radix-literals]
requires: [mem.hex, core.number-literal, core.variable, ctrl.comparison, core.boolean, core.print-variable]
concepts: [mem.radix-literals, mem.tien-to-co-so]
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
Bài trước bạn tự tay đổi một byte ra hex. Giờ mình chỉ bạn cách gõ THẲNG
con số hex đó vào code, khỏi cần gọi hàm nào.
::::

::::explain{#tien-to-cho-may-biet-he-nao}
Bài `so-khong-can-nhay` của Onboarding đã cho bạn gõ số thẳng vào code —
`218`, không dấu nháy, không hàm nào bọc quanh. Python đọc nó theo hệ
mười, vì hệ mười là mặc định.

Python cũng đọc được số viết theo hệ khác, miễn là bạn nói rõ hệ nào
bằng một **tiền tố** — vài ký tự đứng ngay trước con số:

```text title=readonly
0b   →  hệ hai       (binary)
0o   →  hệ tám       (octal)
0x   →  hệ mười sáu  (hex)
```

Số `0` đứng đầu không phải chữ số "không" của bản thân con số — nó cùng
chữ cái theo sau làm thành một CẶP báo hiệu, đọc gộp lại thành tiền tố.
Bỏ tiền tố ấy đi thì phần còn lại phải là chữ số hợp lệ của đúng hệ đó:
sau `0b` chỉ được `0` và `1`; sau `0x` được cả `0`-`9` lẫn `a`-`f`.
::::

::::example{#bon-cach-viet-218}
Bài trước, byte `218` đổi ra hex là `da`, ra bit là `11011010`. Tính
thêm hệ tám (nhóm BA bit một, không phải bốn — hệ tám dùng tám ký hiệu
`0`-`7`, và 2³ = 8): `218` là `332` trong hệ tám.

Bốn cách viết dưới đây, gõ vào Python:

```python title=readonly
print(0xda)
print(0b11011010)
print(0o332)
print(218)
```

```text title=readonly
218
218
218
218
```

Bốn dòng in ra giống hệt nhau. Không phải "giống nhau tình cờ" — đó là
CÙNG một giá trị, chỉ khác ở cách bạn đã GÕ nó ra trong mã nguồn. Máy
đọc xong tiền tố, tính ra giá trị, rồi tiền tố hết việc — Python không
giữ lại một mẩu ghi chú nào nói "con số này từng viết theo hệ mười sáu".

```python title=readonly
tuoi_hex = 0xda
print(tuoi_hex)
print(type(tuoi_hex))
```

```text title=readonly
218
<class 'int'>
```

Gán `0xda` vào một cái tên rồi in cái tên ra, màn hình vẫn hiện `218` —
không phải `0xda`. Và kiểu của nó là `int` bình thường, y hệt kiểu của
`218` gõ trực tiếp. Không có kiểu riêng nào tên là "số hex".
::::

::::predict{#may-dong-khac-nhau commitOnce}
Byte in bốn dòng:

```python title=readonly
print(0xda)
print(0b11011010)
print(218)
print(0o332)
```

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra MẤY dòng KHÁC NHAU
(không tính số lần lặp)?

:::opt{correct}
Một dòng khác nhau duy nhất — `218` — lặp lại bốn lần.
:::

:::opt
Bốn dòng khác nhau: `0xda`, `0b11011010`, `218`, `0o332`.
::why
Gần đúng ở chỗ trong MÃ NGUỒN đúng là có bốn cách gõ khác hẳn nhau — bạn
đọc mã không sai chữ nào.

Chỗ lệch là `print` không in lại nguyên văn thứ bạn đã gõ. Máy tính GIÁ
TRỊ của biểu thức rồi mới in giá trị đó ra — mà bốn biểu thức này tính
ra cùng một giá trị. Tiền tố chỉ là hướng dẫn lúc ĐỌC, không phải một
phần được giữ lại trong kết quả.
::
:::

:::opt
Hai dòng khác nhau: `218` (từ `0xda` và `218`), một dòng khác cho
`0b11011010`, một dòng khác nữa cho `0o332`.
::why
Gần đúng ở nửa đầu — `0xda` và `218` đúng là cùng in ra `218`, bạn ghép
đúng cặp đó.

Chỗ lệch: bạn dừng lại sớm, coi tiền tố `0b` và `0o` là "khác loại" nên
đoán chúng in ra khác. Nhưng cả ba tiền tố `0b`, `0o`, `0x` đều làm ĐÚNG
MỘT việc — báo hệ đếm rồi biến mất — không tiền tố nào đặc biệt hơn
tiền tố nào. `0b11011010` và `0o332` cũng tính ra `218` y như hai cái
kia.
::
:::

:::opt
Hai dòng: `218` lặp ba lần (từ `0xda`, `0b11011010`, `218`), và `332`
một lần (từ `0o332`).
::why
Gần đúng ở chỗ bạn nhận đúng ba cách viết đầu quy về cùng một giá trị —
đúng là như vậy.

Chỗ lệch nằm ở phép tính hệ tám. `332` chỉ đúng nếu đọc ba chữ số đó
theo hệ MƯỜI — nhưng đây là hệ TÁM, mỗi cột mang một luỹ thừa của 8,
không phải 10. Đọc đúng: 3×64 + 3×8 + 2×1 = 192 + 24 + 2 = 218, không
phải 332. Bài 1 của track này đã nói: hệ mười chỉ là một thoả thuận, hệ
nào cũng có bảng vị trí riêng của nó.
::
:::
::::

::::explain{#the-khong-nho-cach-goi}
Bài `doi-gia-tri-cua-ten` của Onboarding đã dạy một luật cho cái TÊN: nó
chỉ buộc vào một giá trị, và không giữ lại quá khứ — gán lại là chuyển
tấm thẻ sang chỗ khác, giá trị cũ coi như không còn.

Con số cũng có một luật giống vậy, chỉ đổi chủ ngữ. Bản thân MỘT GIÁ TRỊ
không giữ lại "nó từng được gõ ra sao". `0xda`, `0b11011010`, `0o332` và
`218` không phải bốn giá trị bằng nhau một cách tình cờ — sau khi Python
đọc xong, chúng LÀ một giá trị, không hơn không kém. Khác biệt duy nhất
nằm ở BẢN THẢO mã nguồn, chỗ mà công việc của tiền tố đã xong từ lâu.
::::

::::code{#kiem-ba-cach-viet-bang-nhau}
Byte viết cùng một byte theo ba cách khác nhau. Hai dòng dưới đã so sẵn một
phía cho bạn — dòng đầu so với `bang_hex`, dòng sau so với `bang_thap_phan`.
Việc của bạn là điền cái TÊN còn thiếu vào phía trống của mỗi dòng, để mỗi
phép so sánh thật sự đối chiếu đúng cặp cách viết bài này đang hỏi.

```python title=starter
bang_hex = 0xda
bang_nhi_phan = 0b11011010
bang_thap_phan = 218

hex_va_nhi_phan_bang_nhau = bang_hex == ___
nhi_phan_va_thap_phan_bang_nhau = ___ == bang_thap_phan

print(hex_va_nhi_phan_bang_nhau)
print(nhi_phan_va_thap_phan_bang_nhau)
```

```python title=solution
bang_hex = 0xda
bang_nhi_phan = 0b11011010
bang_thap_phan = 218

hex_va_nhi_phan_bang_nhau = bang_hex == bang_nhi_phan
nhi_phan_va_thap_phan_bang_nhau = bang_nhi_phan == bang_thap_phan

print(hex_va_nhi_phan_bang_nhau)
print(nhi_phan_va_thap_phan_bang_nhau)
```

```python title=test
assert hex_va_nhi_phan_bang_nhau is True, "hex_va_nhi_phan_bang_nhau phải là True — bang_hex và bang_nhi_phan cùng là 218, chỉ viết khác cách"
assert nhi_phan_va_thap_phan_bang_nhau is True, "nhi_phan_va_thap_phan_bang_nhau phải là True — bang_nhi_phan và bang_thap_phan cùng một giá trị 218"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống chỉ là MỘT CÁI TÊN, không phải cả phép so sánh — dòng đầu đã có sẵn `bang_hex ==`, dòng sau đã có sẵn `== bang_thap_phan`. Bạn chỉ điền tên biến còn thiếu vào đúng phía trống.
- kind: strategy
  body: Cả hai chỗ trống đều cần `bang_nhi_phan` — dòng đầu so nó với `bang_hex`, dòng sau so nó với `bang_thap_phan`. `bang_nhi_phan` đứng ở giữa, bắc cầu qua cả hai cách viết còn lại.
- kind: one-line
  body: "Cả hai chỗ trống đều là `bang_nhi_phan`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là chính cái tên `bang_nhi_phan` — dòng đầu đã so sẵn với `bang_hex`, dòng sau đã so sẵn với `bang_thap_phan`. Gõ một tên khác hay một con số vào đó thì phép so sánh không còn đối chiếu đúng cặp cách viết bài này đang hỏi nữa, dù kết quả cuối cùng vẫn có thể tình cờ ra `True` — vì ba biến này đều cùng bằng 218, MỌI cách ghép đôi giữa chúng đều so ra bằng nhau.
  requireAst:
  - kind: uses-operator, target: ==, min: 2
  - kind: uses-name, target: bang_nhi_phan, min: 2
  # Hai luật dưới không siết gì thêm lên NGƯỜI HỌC — `bang_hex ==` và
  # `== bang_thap_phan` đã có sẵn trong khung, không ai gõ lại chúng. Chúng
  # chặn đúng lỗ mà cổng đột biến bắt được: sửa MỘT phía đã điền sẵn (không
  # phải chỗ trống) thành `bang_nhi_phan` vẫn qua `uses-name bang_nhi_phan
  # min:2` vì luật cũ chỉ đếm TỔNG số lần đọc, không xét đọc ĐÚNG CẶP với ai.
  - kind: uses-name, target: bang_hex, min: 1
  - kind: uses-name, target: bang_thap_phan, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^True\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba cách viết, một giá trị — Python đọc xong là quên ngay cách bạn đã gõ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mọi con số bạn viết từ đầu track tới giờ đều KHÔNG ÂM — 0 tới 255, đúng
tầm một byte. Nhưng số âm thì sao?

Một cái ô chỉ chứa được `0` hoặc `1`. Không có ký tự nào trong tám cái ô
ấy trông giống dấu trừ cả. Vậy nếu quán cần ghi một khoản LỖ, một byte
làm cách nào để nói "đây là số âm"?

Bài sau trả lời — và câu trả lời đầu tiên bạn nghĩ ra rất có thể sẽ
hỏng.
::::

::::checkpoint{mastery=0.8}
::::
