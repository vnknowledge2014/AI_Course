---
id: nen-tang.chuong-trinh-that.doc-lai-thu-da-ghi
title: Đọc lại thứ đã ghi
summary: "Chữ `\"r\"` mở file để đọc thay vì để ghi, và `.read()` đưa toàn bộ nội dung file về thành đúng một chuỗi."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.file-read]
requires: [core.file-close, core.file-write, core.file, core.string-literal, core.variable, core.assignment, core.fstring, core.output, core.type-fn, ctrl.comparison]
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
Chữ nằm yên trên đĩa rồi. Giờ mình đi lấy nó về.
::::

::::explain{#lay-ve-bang-cach-nao}
Bài trước khép lại bằng một cảnh yên tâm: `with open(...) as f:` — hết khối là
Python đóng file hộ bạn, kể cả khi bên trong vừa nổ lỗi. Chữ nằm yên trên đĩa,
tắt máy cũng không mất.

Và đúng lúc yên tâm ấy thì câu hỏi thật hiện ra: mai mở chương trình lên, **lấy
nó ra bằng cách nào**?

Cho tới giờ bạn mới dùng `open` theo đúng một kiểu:

```python title=readonly
open("so.txt", "w")
```

Chữ `"w"` trong ngoặc không phải trang trí. Nó là một lời khai báo với máy:
*tôi mở file này để **ghi***. Máy nghe lời khai ấy, dọn một kết nối cho bạn ghi
— và kết nối đó chỉ ghi được.

Muốn đọc thì khai một chữ khác:

```python title=readonly
open("so.txt", "r")
```

`"r"` là chữ đầu của *read*, tức là **đọc**. Kết nối mở bằng `"r"` không cho bạn
ghi; đổi lại, nó cho phép một động tác mà kết nối `"w"` từ chối:

```python title=readonly
noi_dung = f.read()
```

`.read()` đưa lại **toàn bộ** nội dung của file. Không phải khoản đầu tiên,
không phải dòng đầu tiên — toàn bộ, gom thành **một chuỗi** duy nhất.

Chỗ ấy đáng dừng lại một nhịp, vì nó quyết định mọi thứ bạn làm được sau đó.
Bên trong một file văn bản không có ngăn, không có ô, không có khoản nào tách
khỏi khoản nào. Chỉ có chữ nối tiếp chữ. `.read()` bê nguyên dãy chữ ấy về, và
thứ bạn cầm trong tay là một chuỗi — đúng loại giá trị bạn đã dùng từ Realm 0.
::::

::::example{#ghi-roi-doc-lai}
Ghi một khoản xuống, rồi mở lại chính file ấy để đọc:

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000")

with open("so.txt", "r") as f:
    ca_cuon_so = f.read()

print(ca_cuon_so)
print(type(ca_cuon_so))
```

Máy in ra:

```text title=readonly
cà phê,25000
<class 'str'>
```

Ba chỗ đáng nhìn kỹ:

- **Hai khối `with`, hai kết nối khác nhau.** Khối trên mở để ghi rồi đóng lại
  ngay khi hết khối. Khối dưới mở lại đúng file ấy, lần này để đọc. Chúng không
  dùng chung một kết nối, dù cái tên `f` được đặt lại cho cả hai.
- **Thứ `.read()` đưa về là một chuỗi bình thường.** `type` nói thẳng ra điều
  đó: `<class 'str'>` — cùng loại với `"Phở Thìn"` bạn viết ở R0. Nó ghép nối
  được, so sánh được, chèn vào f-string được.
- **Đọc không làm file suy suyển gì.** Sau đoạn này, `so.txt` vẫn còn nguyên
  dòng `cà phê,25000` trên đĩa. `.read()` chép một bản về cho bạn, nó không bê
  chữ đi khỏi đĩa.
::::

::::predict{#doan-doc-hai-lan commitOnce}
Byte mở cùng một file hai lần liên tiếp để đọc, mỗi lần một khối `with` riêng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000")

with open("so.txt", "r") as f:
    lan_dau = f.read()

with open("so.txt", "r") as f:
    lan_hai = f.read()

print(lan_dau)
print(lan_hai)
```

:::opt{correct}
`cà phê,25000` rồi `cà phê,25000`
:::

:::opt
`cà phê,25000` rồi một dòng trống
::why
Gần đúng ở chỗ bạn theo dõi rất sát chữ nghĩa: người ta hay nói "đọc **lấy**
nội dung ra", mà lấy ra thì chỗ cũ vơi đi. Với một cái hộp thật thì đúng như
vậy.

Chỗ lệch: `.read()` chép chứ không bê đi, nên `so.txt` sau lần đọc đầu vẫn còn
nguyên dòng chữ ấy. Và trong đoạn trên có **hai** khối `with` tách rời: khối
thứ hai mở một kết nối hoàn toàn mới, mà một kết nối mới thì bắt đầu từ đầu
file. Hai lần đọc ở hai kết nối khác nhau nên đều thấy đủ cả dòng.
::
:::

:::opt
Cả hai dòng đều trống
::why
Gần đúng ở chỗ bạn nhớ đúng bài 2, và nhớ một điều quan trọng: chữ chỉ chắc
chắn nằm trên đĩa **sau khi file được đóng**. Nghi ngờ chuyện chữ chưa kịp
xuống đĩa là một nghi ngờ có cơ sở.

Chỗ lệch nằm ở thời điểm. Bài 3 vừa dạy rằng hết khối `with` là Python đóng file
ngay lúc đó — nên tới lúc khối đọc đầu tiên chạy, việc đóng đã xong từ dòng
trước. Chữ đã nằm yên rồi mới có ai đi đọc.
::
:::

:::opt
Máy dừng lại ở khối `with` thứ hai, vì một file đã đóng thì không mở lại được
::why
Gần đúng ở chỗ bạn coi trọng động tác đóng file, và đó là thái độ đúng sau hai
bài vừa rồi.

Chỗ lệch là thứ bị đóng. `.close()` đóng **kết nối**, không đóng **file**. File
là cái hộp nằm trên đĩa; kết nối là sợi dây tạm bạn nối tới nó. Cắt sợi dây
xong thì cái hộp vẫn nằm đó, và nối một sợi dây mới tới nó là chuyện bình
thường — mỗi lần `open` là một sợi dây mới.
::
:::
::::

::::code{#tu-tay-doc-lai}
Đoạn dưới đã ghi sẵn một khoản xuống `so.txt`. Việc của bạn là mở lại đúng file
ấy để **đọc**, và lấy toàn bộ nội dung về cất vào `ca_cuon_so`.

Hai chỗ trống nằm ở hai câu bạn vừa đọc trong phần ví dụ, chỉ bị khoét đi.

```python title=starter
with open("so.txt", "w") as f:
    f.write("cà phê,25000")

with open("so.txt", ___) as f:
    ca_cuon_so = ___

print(ca_cuon_so)
print(f"Đọc lên có đúng thứ vừa ghi không? {ca_cuon_so == 'cà phê,25000'}")
```

```python title=solution
with open("so.txt", "w") as f:
    f.write("cà phê,25000")

with open("so.txt", "r") as f:
    ca_cuon_so = f.read()

print(ca_cuon_so)
print(f"Đọc lên có đúng thứ vừa ghi không? {ca_cuon_so == 'cà phê,25000'}")
```

```python title=test
# Chỗ trống thứ nhất là chế độ mở. Mở nhầm bằng "w" thì máy dọn sạch file rồi
# từ chối cho đọc, nên chương trình dừng trước khi tới câu này; mở đúng bằng
# "r" thì `ca_cuon_so` mới cầm được dòng chữ.
# Chỗ trống thứ hai là thứ được cất vào `ca_cuon_so`. Cất nhầm chính kết nối
# `f` vào đó thì phép so sánh dưới đây cho `False` và câu này vỡ.
assert ca_cuon_so == "cà phê,25000", "ca_cuon_so phải là đúng dòng chữ đang nằm trong so.txt, tức là chuỗi cà phê,25000 — không phải kết nối f, cũng không phải một mẩu của dòng ấy"
# Đọc là việc chỉ xem. Sau khi chương trình chạy xong, file phải còn nguyên.
with open("so.txt", "r") as kiem:
    assert kiem.read() == "cà phê,25000", "sau khi chương trình chạy xong, so.txt trên đĩa vẫn phải giữ đúng dòng cà phê,25000 — đọc một file không làm nội dung của nó vơi đi"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm trong ngoặc của `open`, ngay chỗ mà khối bên trên đang để chữ `"w"`. Chỗ trống thứ hai nằm sau dấu bằng, tức là thứ sẽ được cất vào `ca_cuon_so` — và ở dòng đó, cái tên `f` đang giữ kết nối vừa mở.
- kind: strategy
  body: Khối trên khai với máy rằng nó mở file để ghi. Khối dưới làm việc ngược lại, nên chữ khai báo cũng phải là chữ khác — một chữ cái, đặt trong nháy, đúng cái chữ mà phần giải thích phía trên đã gọi tên. Còn chỗ thứ hai, hãy viết `f.` rồi tên phương thức đưa toàn bộ nội dung về, và đừng bỏ quên cặp ngoặc gọi phía sau.
- kind: one-line
  body: 'Chỗ thứ nhất điền `"r"`, chỗ thứ hai điền `f.read()`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: "cà phê,25000"
- tier: output
  expect: "Đọc lên có đúng thứ vừa ghi không? True"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ghi được, đọc lại được. Cuốn sổ vừa có đủ hai đầu rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte đem đoạn code ghi sổ ấy dùng thật. Hôm thứ hai Byte uống cà phê, chạy
chương trình, ghi xuống `cà phê,25000`. Hôm thứ ba Byte ăn bún bò, chạy lại
đúng chương trình đó với khoản mới.

Bạn chạy chương trình ghi sổ hai ngày liền. Mở ra chỉ thấy khoản của hôm nay.
Khoản hôm qua đi đâu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
