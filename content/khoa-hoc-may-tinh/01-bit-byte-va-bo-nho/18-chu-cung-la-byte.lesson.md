---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.chu-cung-la-byte
title: "Chữ cũng chỉ là byte, và tiếng Việt tốn ba"
summary: "Mỗi ký tự có một con số trong bảng mã, nhưng con số đó không luôn vừa một byte — chữ 'a' chiếm một byte, còn chữ 'ở' phải chiếm ba, và bạn tự đếm được vì sao."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.utf8-bytes]
requires: [mem.hex, core.bang-ma, core.ky-tu-la-so, core.string-method, core.len, core.string-literal, core.fstring, core.variable, core.assignment]
concepts: [mem.utf8-bytes]
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
Chữ `ở` mang con số 7903. Một byte chỉ đếm được tới 255. Vậy con số ấy
nhét vào đâu?
::::

::::explain{#on-lai-bang-ma}
Hồi mới học "vì sao máy chỉ hiểu số", bạn đã biết: mỗi ký tự có một con số
riêng, gán sẵn trong một bảng quy ước. Chữ `A` là 65, chữ `a` là 97,
khoảng trắng là 32. Bảng đó, phần mở rộng đủ cho cả tiếng Việt, gọi là
**Unicode** — và chữ `ở` nằm ở dòng số 7903 trong bảng ấy.

Bài đó dừng lại đúng ở chỗ này: một byte đếm được tới 255, mà 7903 lớn hơn
255 rất nhiều, nên "một ký tự tiếng Việt phải nằm trên nhiều byte đứng
cạnh nhau". Bài hôm nay trả đúng món nợ đó — cho bạn thấy tận mắt CHÍNH
XÁC bao nhiêu byte, và chúng là byte nào.

Công cụ để nhìn là một lệnh mới: `.encode("utf-8")`. Đưa cho nó một chuỗi,
nó trả về **dãy byte thật sự** mà chuỗi ấy chiếm trong máy — mỗi phần tử
của dãy là một số từ 0 đến 255, đúng phạm vi một byte đếm được (bài
`tam-o-la-mot-byte` đã tự tính ra con số 255 đó).
::::

::::example{#mot-byte-va-ba-byte}
Thử với chữ cái tiếng Anh trước, vì nó đơn giản nhất:

```python title=readonly
print("a".encode("utf-8"))
print(list("a".encode("utf-8")))
```

```text title=readonly
b'a'
[97]
```

Đúng một byte, mang giá trị 97 — không lệch một số nào so với con số bạn
đã biết từ bảng mã: `ord("a")` cũng là 97. Với chữ cái không dấu, con số
trong bảng mã và byte lưu trong máy là **một**.

Giờ thử với `ở`:

```python title=readonly
print("ở".encode("utf-8"))
print(list("ở".encode("utf-8")))
```

```text title=readonly
b'\xe1\xbb\x9f'
[225, 187, 159]
```

Ba con số: 225, 187, 159. Không phải 7903 nằm nguyên trong một ô nào cả —
UTF-8 (cách mã hoá đang dùng ở đây) chẻ con số 7903 ra thành ba mảnh, mỗi
mảnh nhét vừa một byte, rồi xếp ba byte ấy cạnh nhau. Đọc lại bằng hex —
đúng luật bài `bon-bit-mot-chu-so`, mỗi byte là hai chữ số hex:

```python title=readonly
print("ở".encode("utf-8").hex())
```

```text title=readonly
e1bb9f
```

`e1`, `bb`, `9f` — ba cặp, khớp đúng ba byte 225, 187, 159. Không byte nào
trong ba byte đó vượt quá 255; luật của bài 4 vẫn đứng vững, chỉ là bây
giờ cần **ba** ô thay vì một.
::::

::::explain{#dem-ca-tu}
Ghép lại thành một từ có cả chữ không dấu lẫn chữ có dấu: `Phở`.

```python title=readonly
tu = "Phở"
print(len(tu))
print(len(tu.encode("utf-8")))
```

```text title=readonly
3
5
```

`len(tu)` đếm **ký tự** — `P`, `h`, `ở`, đúng ba. `len(tu.encode("utf-8"))`
đếm **byte** — và ba ký tự đó không nặng bằng nhau: `P` một byte, `h` một
byte, `ở` ba byte. Cộng lại: 1 + 1 + 3 = 5.

Đây là chỗ trả nợ hẹn từ bài "vì sao máy chỉ hiểu số": một file chữ tiếng
Việt nặng hơn một file chữ tiếng Anh cùng số ký tự, và giờ bạn tự đếm ra
đúng nặng hơn **bao nhiêu**, không cần tin lời ai nữa.

Một điều nên nói rõ, kẻo hiểu lầm: không phải MỌI chữ cái tiếng Việt đều
tốn đúng ba byte. Số byte phụ thuộc dòng mà ký tự ấy nằm trong bảng
Unicode — `ở` tốn ba, nhưng có những chữ có dấu khác (như `à`, `đ`) chỉ
tốn hai. Điều chắc chắn với mọi ký tự tiếng Việt có dấu là: **nhiều hơn
một byte** — không có con số cố định chung cho tất cả.
::::

::::predict{#doan-so-byte-cua-mo commitOnce}
Byte gõ nhầm phím, màn hình hiện đúng một chữ: `mở`. Byte tò mò, đem chữ
đó tính thử — gộp hai con số vào một dòng in duy nhất, thay vì in riêng
từng dòng như ví dụ trên.

**Trước khi bấm chạy**, bạn đoán dòng dưới in ra gì?

```python
mon = "mở"
so_ky_tu = len(mon)
so_byte = len(mon.encode("utf-8"))
print(so_ky_tu, so_byte)
```

:::opt{correct}
`2`, rồi `4`
:::

:::opt
`2`, rồi `2`
::why
Gần đúng ở chỗ đầu: `mở` đúng là có hai ký tự, `m` và `ở`, không hơn
không kém.

Chỗ lệch nằm ở con số sau. Bạn đang tính như thể mỗi ký tự luôn vừa đúng
một byte — đúng với `m`, nhưng `ở` không nằm trong bảng mã 128 dòng đầu,
và ví dụ vừa rồi đã cho thấy nó chiếm tới ba byte, không phải một. Một
byte của `m` cộng ba byte của `ở` mới ra 4, không phải 2.
::
:::

:::opt
`4`, rồi `4`
::why
Gần đúng ở chỗ con số 4 có thật — đó đúng là số byte, và bạn không bịa ra
nó.

Chỗ lệch nằm ở con số đầu. `len(mon)` đếm trên chuỗi Python, và chuỗi
Python đếm theo **ký tự**, không phải byte — dù ký tự đó có dấu hay
không. Chỉ riêng `.encode("utf-8")` mới đổi sang đếm byte; chưa gọi
`.encode` thì `len` vẫn trả lời bằng ký tự, và `mở` có đúng hai.
::
:::

:::opt
`2`, rồi `5`
::why
Gần đúng ở chỗ bạn nhớ đúng: `ở` không nằm gọn trong một byte, nó phải
tràn sang byte khác. Suy luận ấy đúng hướng.

Chỗ lệch là con số tràn. UTF-8 có thể dùng tới bốn byte cho một số ký tự
hiếm gặp (như một số hình mặt cười), nhưng `ở` không rơi vào nhóm đó — nó
nằm ở vùng đúng ba byte, khớp với ví dụ `225, 187, 159` phía trên đã đếm
tận tay. Không phải "ngoài bảng 128 dòng thì luôn tốn tối đa"; số byte tuỳ
vào ký tự nằm ở dòng nào trong bảng.
::
:::
::::

::::code{#dem-byte-mot-mon}
Bảng thực đơn của cô Bảy có món **quẩy**. Byte muốn biết món này tốn bao
nhiêu ký tự và bao nhiêu byte khi lưu vào file thực đơn.

Dùng `.encode("utf-8")` để đếm byte thật — đừng đoán bằng cách cộng thêm
một con số vào `len(mon)`, vì không phải ký tự có dấu nào cũng tốn giống
ký tự có dấu nào.

```python title=starter
mon = "quẩy"

so_ky_tu = len(mon)
so_byte = len(___)

print(f"{mon}: {so_ky_tu} ký tự, {so_byte} byte")
```

```python title=solution
mon = "quẩy"

so_ky_tu = len(mon)
so_byte = len(mon.encode("utf-8"))

print(f"{mon}: {so_ky_tu} ký tự, {so_byte} byte")
```

```python title=test
assert so_ky_tu == 4, "'quẩy' có bốn ký tự: q, u, ẩ, y — đừng sửa dòng đếm ký tự, chỗ trống chỉ nằm ở dòng đếm byte"
assert so_byte == 6, "'quẩy' mã hoá UTF-8 ra sáu byte: q, u, y mỗi chữ một byte, còn ẩ chiếm ba byte — đếm bằng .encode('utf-8') thật, đừng cộng tay"
assert isinstance(so_byte, int), "so_byte phải là một số nguyên — kết quả của len(...) trên một dãy byte"
```

:::hints
- kind: attention
  body: Chỗ trống nằm trong `len(...)`, y hệt dòng ngay trên nó — chỉ khác thứ đưa vào `len`. Dòng trên đếm ký tự bằng cách đưa thẳng `mon` vào `len`. Dòng này cần đếm BYTE, nên trước khi đưa vào `len` phải đổi `mon` thành một dãy byte trước đã.
- kind: strategy
  body: Đổi một chuỗi thành dãy byte dùng đúng lệnh vừa học trong bài — `.encode("utf-8")` — gọi ngay trên `mon`. Đừng cộng thêm một con số đoán chừng vào `so_ky_tu`; mỗi ký tự có dấu tốn số byte khác nhau, cộng tay dễ đúng ăn may cho một từ mà sai cho từ khác.
- kind: one-line
  body: 'Điền `mon.encode("utf-8")` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải thật sự gọi .encode để lấy dãy byte, không phải đoán số byte bằng cách cộng thêm vào số ký tự
  requireAst:
  - kind: uses-call, target: encode, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^quẩy: 4 ký tự, 6 byte\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn ký tự, sáu byte — đếm tận tay, không cần tin lời ai nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba byte của chữ `ở` — 225, 187, 159 — không trôi nổi lơ lửng đâu đó. Chúng
phải **nằm ở đâu** trong bộ nhớ, xếp cạnh nhau theo đúng thứ tự, để lúc
cần đọc lại thì máy lấy đúng cả ba, không lấy nhầm byte của chữ khác.

Muốn tìm lại đúng ba ô ấy, máy cần biết điều gì về chúng?

Đừng trả lời vội. Bài sau chỉ thẳng vào chỗ ấy.
::::

::::checkpoint{mastery=0.8}
::::
