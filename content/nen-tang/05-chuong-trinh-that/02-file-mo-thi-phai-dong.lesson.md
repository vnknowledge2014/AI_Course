---
id: nen-tang.chuong-trinh-that.file-mo-thi-phai-dong
title: File đang mở thì phải đóng
summary: "Thứ `open` đưa lại là một kết nối đang mở tới file; `.close()` đóng nó lại, và chỉ khi đó chữ mới chắc chắn nằm trên đĩa."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.file-close]
requires: [core.file-write, core.file, core.boolean, core.string-method, core.fstring, core.list-of-dicts, core.list-comprehension, core.list-index, core.nested-index, core.variable, core.assignment, core.output, core.builtin-function, core.value-error, err.traceback]
concepts: [core.file, core.dung-sai, core.bien]
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
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Thứ mình đưa lại cho bạn không phải chữ. Nó là một cánh cửa đang mở — và cửa
mở thì có lúc phải đóng.
::::

::::explain{#cai-open-dua-lai-la-gi}
Bài trước kết ở một câu hỏi: thứ `open` đưa lại là gì, và nó tồn tại tới bao
giờ?

Câu trả lời: đó là một **kết nối đang mở** giữa chương trình của bạn và file
nằm ngoài kia. Không phải cái file. Không phải chữ trong file. Là đường đi
giữa hai bên, và nó đang mở.

Hình dung theo lối cũ của cô Bảy: `open` là động tác kéo ngăn kéo ra, lấy cuốn
sổ đặt lên bàn, mở đúng trang, kê sẵn cây bút. Từ lúc ấy tới lúc cô gấp sổ cất
đi, ngăn kéo đứng nguyên ở trạng thái *đang mở*. `.write` là nét bút. Còn động
tác gấp sổ cất vào thì Python viết thế này:

```python title=readonly
f.close()
```

Trong ngoặc không có gì cả — `.close()` đóng đúng cái kết nối đang đứng trước
dấu chấm, không cần bảo nó đóng cái nào.

Bài trước bạn đã hỏi kết nối hai điều nó nhớ: `f.name` và `f.mode`. Nó còn nhớ
một điều thứ ba, và đây là điều của bài này:

```python title=readonly
f.closed
```

Đó là một câu trả lời đúng-sai, đúng loại `True` / `False` bạn học từ Realm 0.
Ngay sau `open` nó là `False` — chưa đóng. Sau `.close()` nó là `True`.

Và đây là chỗ `.close()` đáng giá hơn một thói quen gọn gàng. Máy **không**
chạy ra tận đĩa mỗi lần bạn `.write` một mẩu chữ ngắn — xăng xe như thế tốn quá.
Nó gom các mẩu ấy lại trong bộ nhớ, rồi đổ xuống đĩa một lượt. `.close()` chính
là lệnh *đổ hết những gì còn đang gom xuống, rồi đóng đường xăng xe*. Nên chỉ
sau khi đóng, chữ mới **chắc chắn** đã nằm trên đĩa.

Vậy nó tồn tại tới bao giờ? Từ `open` cho tới `.close()`. Đó là cả tuổi đời
của một kết nối.
::::

::::example{#mo-roi-dong}
Ba dòng, và một câu hỏi được hỏi hai lần — trước và sau khi đóng.

```python title=readonly
f = open("nhap.txt", "w")
print(f"Vừa mở xong — đã đóng chưa? {f.closed}")

f.write("cà phê,25000")
f.close()

print(f"Sau .close() — đã đóng chưa? {f.closed}")
```

Máy in ra:

```text title=readonly
Vừa mở xong — đã đóng chưa? False
Sau .close() — đã đóng chưa? True
```

Ba chỗ đáng dừng lại nhìn:

- **`f.closed` đổi từ `False` sang `True`**, và thứ làm nó đổi là đúng một
  dòng: `f.close()`. Không có dòng ấy thì tới cuối chương trình nó vẫn là
  `False`.
- **Cái tên `f` vẫn còn sau khi đóng.** Đóng kết nối không xoá cái tên; `f`
  vẫn trả lời được `f.name`, `f.mode`, `f.closed`. Thứ hết hiệu lực là đường
  đi tới file, không phải cái tên.
- **`.close()` không đưa lại biên nhận nào**, khác `.write` ở bài trước. Nó
  không có gì để báo — việc của nó là dọn dẹp, không phải chuyển hàng.
::::

::::predict{#doan-ghi-sau-khi-dong commitOnce}
Byte đóng sổ xong mới sực nhớ còn một khoản chưa ghi, nên gọi thêm một
`.write` nữa ở dòng cuối.

**Trước khi bấm chạy**, bạn đoán chuyện gì xảy ra?

```python title=readonly
f = open("nhap.txt", "w")
f.write("cà phê,25000")
f.close()
f.write("bún bò,40000")
```

:::opt{correct}
Máy dừng lại ở dòng cuối và in ra một traceback: `ValueError: I/O operation on closed file.`
:::

:::opt
File có cả hai khoản. `.close()` chỉ đánh dấu là đã ghi xong, ghi thêm thì vẫn được.
::why
Gần đúng ở chỗ bạn hình dung `.close()` như một cái dấu đóng lên tờ giấy — làm
xong thì đóng dấu, mà đóng dấu rồi thì tờ giấy vẫn nằm đó. Cách nghĩ ấy hợp lý
với rất nhiều thứ trong đời.

Chỗ lệch: `.close()` không đánh dấu, nó **cắt đường đi**. Sau khi cắt, `f` vẫn
còn tên nhưng không còn nối tới file nữa, nên `.write` chẳng có lối nào để đi.
Muốn ghi thêm thì phải `open` lại một lần nữa — mở lại là một kết nối mới, chứ
kết nối cũ không sống lại được.
::
:::

:::opt
Dòng cuối chạy nhưng không làm gì cả, chương trình vẫn im lặng chạy tới hết.
::why
Gần đúng ở chỗ bạn đoán đúng phần *file không có thêm gì*: khoản `bún bò` quả
thật không vào được tới file.

Chỗ lệch là ở chữ **im lặng**. Python không lặng lẽ bỏ qua một việc bạn đã bảo
nó làm — im lặng như thế là kiểu hỏng tệ nhất, vì bạn tưởng đã ghi rồi. Nó nói
thẳng ra bằng một traceback, và đọc từ dòng cuối lên đúng như Realm 0 đã dạy
thì dòng ấy ghi rõ: thao tác vào-ra trên một file đã đóng.
::
:::

:::opt
Python tự mở lại file rồi ghi tiếp, vì nó biết `f` trỏ tới file nào.
::why
Gần đúng ở chỗ bạn nhớ đúng một điều có thật: `f` vẫn nhớ tên file, `f.name`
vẫn trả lời được `nhap.txt` kể cả sau khi đóng.

Chỗ lệch là *nhớ tên* với *đang nối tới* là hai chuyện. Bạn nhớ số nhà một
người bạn không có nghĩa là cửa nhà họ đang mở. Python cũng không tự ý mở lại
hộ bạn: mở file là việc có hậu quả thật ngoài đĩa, nên nó chỉ làm khi bạn viết
ra lệnh `open`.
::
:::
::::

::::explain{#mot-open-mot-close}
Phát biểu lại cho gọn: **mỗi lần `open` sinh ra một kết nối, và mỗi kết nối
phải được đóng lấy một lần.**

Chuyện này chỉ thành thói quen khi chương trình mở hơn một file. Đóng cuốn sổ
không đóng hộ cuốn tóm tắt — chúng là hai kết nối riêng, mỗi cái có `f.closed`
riêng, và Python không suy ra hộ bạn cái nào đi với cái nào.

Bước sau đưa bạn đúng tình huống đó: hai file, hai kết nối, và hai chỗ trống.
::::

::::code{#dong-ca-hai-ket-noi}
Byte ghi hai file trong một lần chạy: `so-thang-tam.txt` giữ dòng đầu của sổ,
`tom-tat-thang-tam.txt` giữ câu tổng kết của **ba khoản** trong sổ rút gọn này.

Hai `open` đã viết sẵn, hai `.write` cũng vậy. Việc còn lại là đóng.

```python title=starter
so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "gửi xe", "nhom": "xăng xe", "tien": 10000},
]

tong = sum([khoan["tien"] for khoan in so])

f_so = open("so-thang-tam.txt", "w")
f_tom = open("tom-tat-thang-tam.txt", "w")

f_so.write(f"{so[0]['ten']},{so[0]['tien']}")
f_tom.write(f"tổng,{tong}")

___
___

print(f"Sổ đã đóng chưa? {f_so.closed}")
print(f"Tóm tắt đã đóng chưa? {f_tom.closed}")
```

```python title=solution
so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "gửi xe", "nhom": "xăng xe", "tien": 10000},
]

tong = sum([khoan["tien"] for khoan in so])

f_so = open("so-thang-tam.txt", "w")
f_tom = open("tom-tat-thang-tam.txt", "w")

f_so.write(f"{so[0]['ten']},{so[0]['tien']}")
f_tom.write(f"tổng,{tong}")

f_so.close()
f_tom.close()

print(f"Sổ đã đóng chưa? {f_so.closed}")
print(f"Tóm tắt đã đóng chưa? {f_tom.closed}")
```

```python title=test
# Hai chỗ trống, hai câu chấm — mỗi kết nối có `closed` riêng, nên đóng một
# cái không làm câu kia xanh theo.
assert f_so.closed, "kết nối tới so-thang-tam.txt vẫn đang mở: f_so.closed đang là False, nghĩa là chưa có dòng nào đóng nó lại"
assert f_tom.closed, "kết nối tới tom-tat-thang-tam.txt vẫn đang mở: đóng cuốn sổ không đóng hộ cuốn tóm tắt, mỗi kết nối phải tự đóng lấy một lần"
# Hai kết nối phải trỏ đúng hai file khác nhau — đóng nhầm cái này hai lần thì
# câu trên đã vỡ, còn câu dưới canh cho hai cái tên không bị lẫn.
assert f_so.name == "so-thang-tam.txt", "f_so phải là kết nối tới so-thang-tam.txt"
assert f_tom.name == "tom-tat-thang-tam.txt", "f_tom phải là kết nối tới tom-tat-thang-tam.txt"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm ở hai dòng liền nhau, sau khi cả hai `.write` đã xong và trước hai dòng `print`. Mỗi chỗ là một câu lệnh trọn vẹn đứng riêng một dòng, không gán vào tên nào cả. Nhìn lại hai cái tên mà hai dòng `open` phía trên đã đặt.
- kind: strategy
  body: Việc cần làm là gấp sổ cất đi, và động tác ấy viết dưới hình dạng quen thuộc: một cái tên, dấu chấm, rồi tên việc, rồi cặp ngoặc rỗng vì nó không cần biết thêm gì. Có hai kết nối thì phải làm hai lần, mỗi lần với một cái tên khác nhau.
- kind: one-line
  body: 'Dòng thứ nhất viết `f_so.close()`, dòng thứ hai viết `f_tom.close()`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Sổ đã đóng chưa\? True\nTóm tắt đã đóng chưa\? True\s*$
- tier: output
  expect: "Tóm tắt đã đóng chưa? True"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai ngăn kéo, hai lần gấp sổ. Giờ thì chữ nằm yên trên đĩa thật rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chương trình vừa rồi chạy trơn tru nên `.close()` chạy được. Nhưng chương trình
thật thì hay vấp: một cái tên gõ nhầm, một phép tính không hợp kiểu — Realm 0
đã cho bạn xem cả một chùm traceback như vậy.

Nếu giữa `open` và `.close()` có một dòng gây lỗi, chương trình dừng ngay tại
đó. `.close()` có kịp chạy không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
