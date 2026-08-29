---
id: nen-tang.chuong-trinh-that.ghi-them-khong-xoa
title: Ghi thêm chứ không xoá
summary: "Chữ `\"w\"` dọn sạch file trước khi đặt xuống ký tự đầu tiên; chữ `\"a\"` giữ nguyên thứ đang có và ghi nối vào cuối."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.file-append]
requires: [core.file-read, core.file-write, core.file-close, core.file, core.string-literal, core.variable, core.assignment, core.output]
concepts: [core.file, core.chuoi]
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
Khoản hôm qua không đi đâu cả. Nó bị dọn đi, và người dọn là một chữ cái.
::::

::::explain{#hai-ngay-mot-cuon-so}
Bài trước để lại một cảnh khó chịu. Byte chạy chương trình ghi sổ hai ngày
liền: hôm thứ hai ghi `cà phê,25000`, hôm thứ ba ghi `bún bò,40000`. Mở
`so.txt` ra, trong đó chỉ có khoản của hôm nay.

Một cuốn sổ chi tiêu mà mỗi lần ghi lại xoá hết trang cũ thì không phải sổ. Nên
trước khi chữa, phải biết chính xác ai xoá.

Trong chương trình ấy có đúng ba chỗ có thể là thủ phạm: câu `open`, câu
`.write`, và việc đóng file. Bài này chỉ mặt một trong ba, và bước ngay dưới
đây là chỗ bạn tự chỉ.

Để dựng lại cảnh hai ngày mà không phải đợi tới mai, ta viết hai lần ghi vào
cùng một chương trình. Máy không biết giữa hai lần ấy có một đêm hay không —
với nó, đó vẫn là hai lần mở cùng một file để ghi.
::::

::::predict{#doan-ghi-hai-lan commitOnce}
Hai khối `with`, cùng một tên file, cùng một chữ `"w"`. Rồi mở ra đọc.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000")

with open("so.txt", "w") as f:
    f.write("bún bò,40000")

with open("so.txt", "r") as f:
    print(f.read())
```

:::opt{correct}
`bún bò,40000`
:::

:::opt
`cà phê,25000bún bò,40000`
::why
Gần đúng ở chỗ bạn suy ra từ một điều bài trước vừa chứng minh, và chứng minh
rất chắc: chữ ghi xuống thì nằm yên trên đĩa, đọc lại vẫn còn nguyên. Từ đó
nghĩ rằng lần ghi sau đặt tiếp vào sau lần ghi trước là một suy luận thẳng.

Chỗ lệch nằm ở chữ `"w"`. Nó không nói "hãy ghi thêm", nó nói "tôi mở file này
để ghi" — và trước khi cho ký tự đầu tiên rơi xuống, máy dọn sạch những gì đang
có. Bạn sẽ thấy đúng kết quả này ở phần dưới, nhưng với một chữ khác.
::
:::

:::opt
`cà phê,25000`
::why
Gần đúng ở chỗ bạn đang bảo vệ thứ đã ghi trước, và đó là bản năng của người
giữ sổ.

Chỗ lệch là chiều của việc đè. Khi hai lần ghi tranh nhau cùng một chỗ, thứ
đứng lại là thứ **ghi sau cùng** — y như dấu `=` ở R0 bài 12: dán lại cái tên
lên một giá trị mới thì giá trị mới thắng. Ở đây lần ghi thứ hai chạy sau, nên
`bún bò,40000` là thứ còn lại.
::
:::

:::opt
Máy dừng lại ở khối thứ hai, vì `so.txt` đã tồn tại rồi
::why
Gần đúng ở chỗ bạn nghĩ tới việc máy nên hỏi một câu trước khi đụng vào thứ đã
có. Nhiều phần mềm bạn dùng hằng ngày làm đúng như vậy khi lưu đè.

Chỗ lệch là `open` không hỏi câu đó. Chữ `"w"` đã là câu trả lời rồi: nó nhận
cả hai việc — chưa có file thì tạo mới, có rồi thì dọn sạch — và làm ngay,
không cảnh báo. Đó chính là lý do bài này tồn tại.
::
:::
::::

::::explain{#w-don-sach-truoc-khi-ghi}
Đoán xong rồi thì phát biểu lại cho gọn.

**`"w"` dọn sạch file trước khi ghi ký tự đầu tiên.** Việc dọn xảy ra ngay lúc
`open` chạy, chứ không đợi tới `.write` — nên một chương trình mở file bằng
`"w"` rồi không ghi gì cả cũng đủ làm cuốn sổ trắng tinh.

Cần nói cho đủ hai vế, vì `"w"` không phải kẻ phá hoại: nó cũng là thứ **tạo ra
file** khi file chưa hề tồn tại. Bài 1 dựng được `so.txt` từ chỗ không có gì
chính là nhờ nó. Một chữ, hai việc — và việc thứ hai là việc bạn cần đúng một
lần, ở lần đầu tiên.

Từ lần thứ hai trở đi bạn cần một chữ khác:

```python title=readonly
open("so.txt", "a")
```

`"a"` là chữ đầu của *append*, tiếng Anh nghĩa là **nối thêm vào cuối**. Kết nối
mở bằng `"a"` không đụng một ký tự nào đang có trong file; mọi thứ bạn `.write`
được đặt vào sau chữ cuối cùng.

Ba chữ, ba lời khai khác nhau với cùng một câu `open`:

| Chữ | Bạn khai với máy |
|---|---|
| `"w"` | mở để ghi, và dọn sạch những gì đang có |
| `"a"` | mở để ghi, giữ nguyên những gì đang có, đặt tiếp vào cuối |
| `"r"` | mở để đọc, không ghi gì cả |
::::

::::example{#ghi-noi-vao-cuoi}
Cùng hai lần ghi của phần đoán, đổi mỗi chữ chế độ ở khối thứ hai:

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000")

with open("so.txt", "a") as f:
    f.write("bún bò,40000")

with open("so.txt", "r") as f:
    print(f.read())
```

Máy in ra:

```text title=readonly
cà phê,25000bún bò,40000
```

Khoản hôm qua ở lại. Đó là toàn bộ việc bài này đi tìm.

Hai chỗ đáng ghi nhớ:

- **Khối đầu vẫn để `"w"`, và để có chủ ý.** Nó mở một cuốn sổ mới, sạch, cho
  lần chạy này. Nếu đổi luôn nó thành `"a"` thì mỗi lần bạn bấm chạy lại, hai
  khoản cũ vẫn nằm đó và hai khoản mới nối tiếp phía sau — chạy năm lần là
  mười khoản.
- **Có một chỗ gợn trong dòng in ra.** `cà phê,25000bún bò,40000` — hai khoản
  dính liền nhau thành một dòng, không có chỗ nào ngăn chúng. Giữ lấy chỗ gợn
  đó, cuối bài ta quay lại.
::::

::::code{#so-ba-khoan}
Ba khoản chi, ghi vào `so.txt` bằng ba lần mở file riêng biệt — đúng như ba
ngày Byte chạy chương trình ba lần.

Khối đầu tiên đã để sẵn `"w"`: nó cố ý mở một cuốn sổ mới cho lần chạy này.
Hai chỗ trống là chế độ của hai lần ghi sau, và cả hai đều phải giữ nguyên
những gì đang có trong sổ.

Giữa chừng, chương trình đọc sổ lên một lần để bạn nhìn thấy nó dài ra.

```python title=starter
with open("so.txt", "w") as f:
    f.write("cà phê,25000")

with open("so.txt", ___) as f:
    f.write("bún bò,40000")

with open("so.txt", "r") as f:
    sau_hai_khoan = f.read()

with open("so.txt", ___) as f:
    f.write("bánh mì,15000")

with open("so.txt", "r") as f:
    ca_cuon_so = f.read()

print(sau_hai_khoan)
print(ca_cuon_so)
```

```python title=solution
with open("so.txt", "w") as f:
    f.write("cà phê,25000")

with open("so.txt", "a") as f:
    f.write("bún bò,40000")

with open("so.txt", "r") as f:
    sau_hai_khoan = f.read()

with open("so.txt", "a") as f:
    f.write("bánh mì,15000")

with open("so.txt", "r") as f:
    ca_cuon_so = f.read()

print(sau_hai_khoan)
print(ca_cuon_so)
```

```python title=test
# Chỗ trống thứ nhất bị soi bởi câu này: sổ được đọc lên NGAY SAU lần ghi thứ
# hai, nên nếu lần ấy dọn sạch sổ thì khoản cà phê biến mất và câu này vỡ.
assert sau_hai_khoan == "cà phê,25000bún bò,40000", "đọc sổ ngay sau lần ghi thứ hai phải thấy cả hai khoản theo đúng thứ tự đã ghi: cà phê,25000 rồi tới bún bò,40000 — nếu chỉ còn bún bò,40000 thì lần ghi thứ hai đã dọn sạch sổ trước khi đặt chữ xuống"
# Chỗ trống thứ hai bị soi bởi câu này: nếu lần ghi thứ ba dọn sạch sổ thì
# trong file chỉ còn mỗi khoản bánh mì.
assert ca_cuon_so == "cà phê,25000bún bò,40000bánh mì,15000", "đọc sổ sau lần ghi thứ ba phải thấy đủ ba khoản theo đúng thứ tự đã ghi: cà phê,25000, bún bò,40000 rồi bánh mì,15000 — nếu chỉ còn bánh mì,15000 thì lần ghi thứ ba đã dọn sạch sổ trước khi đặt chữ xuống"
# Hai câu trên đọc qua biến; câu này hỏi thẳng cái file trên đĩa.
with open("so.txt", "r") as kiem:
    assert kiem.read() == "cà phê,25000bún bò,40000bánh mì,15000", "sau khi chương trình chạy xong, chính so.txt trên đĩa phải giữ đủ ba khoản cà phê,25000, bún bò,40000 và bánh mì,15000"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở cùng một vị trí trong hai khối giống nhau — đối số thứ hai của `open`, tức là chỗ khai với máy bạn định làm gì với file. Khối đầu tiên trong chương trình đang để một chữ ở đúng vị trí ấy, và hai khối của bạn cần khai một việc khác với nó.
- kind: strategy
  body: Khối đầu tiên mở một cuốn sổ mới nên nó dọn sạch, và đó là việc đúng cho khoản thứ nhất. Hai lần ghi sau thì ngược lại: chúng phải để nguyên mọi ký tự đang có rồi đặt tiếp vào sau chữ cuối cùng. Bảng ba chữ ở phần giải thích phía trên có đúng một dòng mô tả việc đó — chữ ấy cũng là một chữ cái, cũng đặt trong nháy.
- kind: one-line
  body: 'Cả hai chỗ trống đều điền `"a"`.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^cà phê,25000bún bò,40000\ncà phê,25000bún bò,40000bánh mì,15000\s*$
- tier: output
  expect: "cà phê,25000bún bò,40000bánh mì,15000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sổ dài ra thay vì ngắn lại. Đúng chiều rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cuốn sổ giờ giữ được mọi khoản: `"w"` mở sổ mới, `"a"` ghi thêm, và không lần
ghi nào xoá lần ghi trước nữa.

Nhưng nhìn lại thứ vừa in ra thì có chuyện.

Ba lần ghi, mở ra thấy `cà phê,25000bún bò,40000bánh mì,15000` dính liền một
dòng. Bảo máy xuống dòng bằng cách nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
