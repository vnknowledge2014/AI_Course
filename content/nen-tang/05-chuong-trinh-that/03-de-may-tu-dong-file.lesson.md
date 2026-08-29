---
id: nen-tang.chuong-trinh-that.de-may-tu-dong-file
title: Để máy tự đóng
summary: "`with open(...) as f:` giao việc đóng file cho Python — hết khối là file đóng, kể cả khi bên trong vừa nổ ra một lỗi."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.with-open]
requires: [core.file-close, core.file-write, core.file, core.boolean, ctrl.block-indent, ctrl.break, ctrl.for-each, core.name-lookup, err.name-error, err.traceback, core.fstring, core.list-of-dicts, core.list-comprehension, core.list-index, core.nested-index, core.variable, core.assignment, core.output, core.builtin-function, core.string-method]
concepts: [core.file, core.khoi-lenh, core.dung-sai]
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
Nhớ đóng cửa thì được. Nhớ đóng cửa vào đúng cái ngày mọi thứ cháy mới là
chuyện khó.
::::

::::explain{#close-co-kip-chay-khong}
Câu hỏi bài trước để lại: nếu giữa `open` và `.close()` có một dòng gây lỗi,
`.close()` có kịp chạy không?

Đem thử. Byte gõ nhầm tên biến ở dòng 5 — định viết `so_khoan` mà ra
`dem_khoan`, một cái tên chưa bài nào đặt:

```python title=readonly
tong = 75000

f = open("so-thang-tam.txt", "w")
f.write("cà phê,25000")
print(f"Số khoản: {dem_khoan}")
f.close()
```

Máy in ra:

```text title=readonly
Traceback (most recent call last):
  File "so.py", line 5, in <module>
    print(f"Số khoản: {dem_khoan}")
                       ^^^^^^^^^
NameError: name 'dem_khoan' is not defined
```

Đọc từ dòng cuối lên, đúng như Realm 0 đã dạy: loại lỗi là `NameError`, chỗ
vấp là dòng 5. Và đây là chỗ đáng nhìn kỹ — **dòng 6 không bao giờ chạy**.
Chương trình dừng lại ở dòng 5 và không đi tiếp. Dòng 6 chính là `f.close()`.

Nên câu trả lời là: không kịp. Kết nối vẫn đang mở lúc chương trình chết, và
câu `cà phê,25000` vừa `.write` thì chưa chắc đã xuống tới đĩa — vì như bài
trước đã nói, thứ làm nó chắc chắn xuống là `.close()`.

Bạn có thể tự dặn mình cẩn thận hơn. Nhưng dòng gây lỗi ở đây là một cái tên
gõ nhầm, và không ai gõ nhầm một cách có kế hoạch.

Python có sẵn một cách giao hẳn việc đóng cho máy:

```python title=readonly
with open("so-thang-tam.txt", "w") as f:
    f.write("cà phê,25000")
```

Đọc từng mảnh:

- **`with`** mở đầu, và dòng ấy kết bằng dấu hai chấm, còn những dòng thuộc về
  nó thì thụt vào — đúng hình dạng khối lệnh bạn đã dùng với `if` và `for`.
- **`open(...)`** vẫn y nguyên bài 1: cùng tên file, cùng chữ `"w"`.
- **`as f`** là chỗ đặt tên cho kết nối, đứng thay cho dấu `=` của bài 1.
- **Không còn dòng `.close()` nào.** Hết khối là Python tự đóng, và nó đóng
  kể cả khi khối ấy vừa nổ ra một lỗi giữa chừng.
::::

::::example{#khoi-with}
Hỏi `f.closed` ở hai chỗ: một chỗ bên trong khối, một chỗ sau khi khối đã hết.

```python title=readonly
with open("nhap.txt", "w") as f:
    f.write("cà phê,25000")
    print(f"Đang trong khối — đã đóng chưa? {f.closed}")

print(f"Ra khỏi khối — đã đóng chưa? {f.closed}")
```

Máy in ra:

```text title=readonly
Đang trong khối — đã đóng chưa? False
Ra khỏi khối — đã đóng chưa? True
```

Không có dòng `.close()` nào trong đoạn trên, mà kết nối vẫn đóng. Thứ đóng nó
là **chỗ khối kết thúc** — chỗ chữ thụt vào hết, tức là dòng `print` cuối cùng
đã lùi ra sát lề trái.

Nhưng đoạn trên đi ra khỏi khối bằng cửa chính. Câu hỏi thật của bài này là
lúc đi ra bằng cửa khác. Thử một cửa khác mà bạn đã biết — `break`:

```python title=readonly
for lan in [1]:
    with open("nhap.txt", "w") as g:
        g.write("cà phê,25000")
        break

print(f"Nhảy ra giữa chừng — đã đóng chưa? {g.closed}")
```

Máy in ra:

```text title=readonly
Nhảy ra giữa chừng — đã đóng chưa? True
```

`break` nhảy thẳng ra ngoài, bỏ lại phần còn lại của khối `with`. Nếu ở đây có
một dòng `.close()` viết tay đặt sau `break` thì nó đã bị bỏ qua. Vậy mà kết
nối vẫn đóng.

Đó là điều `with` hứa: **đi ra bằng đường nào cũng đóng**. Cửa chính, `break`,
hay một dòng nổ lỗi như `dem_khoan` ở trên — Python đóng file trên đường ra,
trước khi để lỗi bay tiếp lên màn hình.
::::

::::predict{#doan-sau-khoi-with commitOnce}
Byte viết một khối `with` gọn ghẽ rồi hỏi `f` một câu ở ngoài khối.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra gì?

```python title=readonly
with open("nhap.txt", "w") as f:
    f.write("cà phê,25000")

print(f.closed)
```

:::opt{correct}
`True`
:::

:::opt
`False`, vì trong cả đoạn không có dòng `.close()` nào.
::why
Gần đúng ở chỗ bạn dò rất kỹ và dò đúng: trong đoạn này thật sự không có dòng
`.close()` nào cả. Bài trước dạy rằng thiếu dòng ấy thì `f.closed` vẫn là
`False`, và bạn áp đúng điều đã học.

Chỗ lệch là ai gọi `.close()`. Với `with`, người gọi không phải bạn mà là
Python, và nó gọi vào lúc khối kết thúc. Dòng `.close()` vẫn xảy ra — nó chỉ
không xuất hiện dưới dạng một dòng bạn phải gõ.
::
:::

:::opt
Máy báo lỗi, vì `f` chỉ sống bên trong khối `with` và ra ngoài là hết.
::why
Gần đúng ở chỗ bạn liên hệ tới một luật có thật và rất đáng nhớ: có những cái
tên chỉ sống trong một vùng nhất định, và ra khỏi vùng ấy thì gọi tên là
`NameError`.

Chỗ lệch là `with` không tạo ra một vùng như thế. Cái tên sau chữ `as` được
đặt ở ngay chỗ khối đang đứng, y như `for lan in [1]` để lại cái tên `lan`
dùng được sau vòng lặp. Thứ hết hiệu lực khi ra khỏi khối là **kết nối**, còn
cái tên thì vẫn còn — đúng như bài trước: đóng kết nối không xoá cái tên.
::
:::

:::opt
`nhap.txt`
::why
Gần đúng ở chỗ bạn nhớ rằng kết nối có giữ tên file, và nó giữ thật — hỏi
`f.name` thì đúng ra `nhap.txt`.

Chỗ lệch là chỗ trong câu lệnh: dòng cuối hỏi `f.closed`, không hỏi `f.name`.
`closed` là câu hỏi *còn mở không*, nên câu trả lời của nó là `True` hoặc
`False`, không bao giờ là một cái tên.
::
:::
::::

::::code{#hai-khoi-with-cho-hai-file}
Cùng hai file của bài trước, cùng cuốn sổ ba khoản, cùng những con số ấy —
`so-thang-tam.txt` giữ dòng đầu của sổ, `tom-tat-thang-tam.txt` giữ câu tổng
kết của ba khoản — nhưng lần này viết lại bằng `with`, và không được có dòng
`.close()` nào.

Hai chỗ trống là hai dòng mở khối. Mỗi dòng phải nói đủ ba điều: mở file nào,
định làm gì với nó, và gọi kết nối ấy là gì.

```python title=starter
so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "gửi xe", "nhom": "xăng xe", "tien": 10000},
]

tong = sum([khoan["tien"] for khoan in so])

___
    f_so.write(f"{so[0]['ten']},{so[0]['tien']}")

___
    f_tom.write(f"tổng,{tong}")

print(f"Sổ đã đóng chưa? {f_so.closed}")
print(f"Tóm tắt đã đóng chưa? {f_tom.closed}")
print(f"Hai file vừa ghi: {f_so.name} và {f_tom.name}")
```

```python title=solution
so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "gửi xe", "nhom": "xăng xe", "tien": 10000},
]

tong = sum([khoan["tien"] for khoan in so])

with open("so-thang-tam.txt", "w") as f_so:
    f_so.write(f"{so[0]['ten']},{so[0]['tien']}")

with open("tom-tat-thang-tam.txt", "w") as f_tom:
    f_tom.write(f"tổng,{tong}")

print(f"Sổ đã đóng chưa? {f_so.closed}")
print(f"Tóm tắt đã đóng chưa? {f_tom.closed}")
print(f"Hai file vừa ghi: {f_so.name} và {f_tom.name}")
```

```python title=test
# Chỗ trống thứ nhất bị soi bởi ba câu: khối phải đóng lấy kết nối, kết nối
# phải trỏ đúng file, và chữ trao cho open phải là chữ đi ghi.
assert f_so.closed, "ra khỏi khối rồi thì kết nối tới sổ phải tự đóng; f_so.closed đang là False, nghĩa là dòng bạn viết mở file bằng cách khác chứ không phải bằng with"
assert f_so.name == "so-thang-tam.txt", "khối thứ nhất phải mở file so-thang-tam.txt, vì dòng bên trong nó ghi dòng đầu của sổ"
assert f_so.mode == "w", "khối thứ nhất đi GHI, nên chữ trao cho open phải là 'w'"
# Chỗ trống thứ hai bị soi bởi ba câu cùng kiểu, và tên file thì khác hẳn —
# nên chép nguyên dòng thứ nhất xuống dưới là vỡ ngay ở câu tên file.
assert f_tom.closed, "ra khỏi khối rồi thì kết nối tới tóm tắt cũng phải tự đóng; f_tom.closed đang là False"
assert f_tom.name == "tom-tat-thang-tam.txt", "khối thứ hai phải mở file tom-tat-thang-tam.txt, không phải cùng file với khối thứ nhất"
assert f_tom.mode == "w", "khối thứ hai cũng đi GHI, nên chữ trao cho open phải là 'w'"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở hai dòng mở khối, và bạn nhận ra chúng qua chính những dòng ngay dưới — hai dòng ấy đã thụt vào, tức là chúng đang chờ một dòng đầu khối ở phía trên. Nhìn cái tên mà mỗi dòng thụt vào đang dùng: đó chính là cái tên mà dòng đầu khối phải đặt ra.
- kind: strategy
  body: Mỗi dòng đầu khối gồm bốn mảnh nối nhau - từ mở đầu, lời gọi open y như bài 1 với tên file và chữ đi ghi, chỗ đặt tên cho kết nối, rồi dấu hai chấm khép lại. Hai khối mở hai file khác nhau, nên hai dòng ấy khác nhau ở tên file và ở cái tên đặt cho kết nối.
- kind: one-line
  body: 'Dòng thứ nhất viết `with open("so-thang-tam.txt", "w") as f_so:`, dòng thứ hai viết `with open("tom-tat-thang-tam.txt", "w") as f_tom:`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Sổ đã đóng chưa\? True\nTóm tắt đã đóng chưa\? True\nHai file vừa ghi: so-thang-tam\.txt và tom-tat-thang-tam\.txt\s*$
- tier: output
  expect: "Hai file vừa ghi: so-thang-tam.txt và tom-tat-thang-tam.txt"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không còn dòng nào để quên. Máy đóng hộ, kể cả những hôm mọi thứ đổ vỡ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba bài vừa rồi đi hết một chiều: mở ra, đặt chữ vào, và đóng lại cho chắc. Giờ
thì tắt máy đi ngủ được rồi — chữ đã nằm yên trên đĩa, trong một file có tên,
đứng ở một chỗ.

Nhưng cả ba bài ấy, màn hình chưa một lần hiện ra thứ nằm trong file. Bạn tin
là nó có ở đó, và niềm tin ấy dựa vào biên nhận của `.write` chứ chưa dựa vào
mắt mình.

Chữ đã nằm yên trên đĩa. Mai mở chương trình lên, lấy nó ra bằng cách nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
