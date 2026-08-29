---
id: nen-tang.chuong-trinh-that.ky-tu-xuong-dong
title: Chỗ xuống dòng cũng là một ký tự
summary: "Chỗ ngắt dòng trong file là một ký tự thật, Python viết nó là `\\n`, nó chiếm đúng một chỗ — và nếu bạn không tự ghi nó vào thì không ai ghi hộ."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.newline-char]
requires: [core.file-append, core.file-read, core.file-write, core.file-close, core.bang-ma, core.len, core.string-concat, core.string-literal, core.variable, core.assignment, core.output]
concepts: [core.file, core.chuoi, core.bang-ma]
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
Mình chưa bao giờ xuống dòng hộ bạn. Mình không biết bạn muốn xuống ở đâu.
::::

::::explain{#ai-xuong-dong-cho-ban}
Bài trước kết ở một chỗ gợn. Ba lần ghi, mở sổ ra thấy:

```text title=readonly
cà phê,25000bún bò,40000bánh mì,15000
```

Ba khoản dính liền một dòng. Không phải sổ hỏng — sổ giữ đúng từng ký tự bạn
đưa cho nó, không thiếu một cái nào. Vấn đề là bạn chưa hề đưa cho nó cái ký tự
ngăn giữa hai khoản.

Câu đó nghe lạ, vì trong đầu ta chỗ xuống dòng không giống một ký tự. Chữ `c`
là một ký tự — nhìn thấy được, đếm được. Còn chỗ ngắt dòng thì như một khoảng
trống, một chỗ giấy trắng.

Realm 0 bài 6 đã dọn sẵn chỗ để trả lời chuyện này. Bên trong máy không có chữ,
chỉ có số, và chữ đọc ra được là nhờ một **bảng quy ước**: mỗi ký tự ứng với một
con số. Trong bảng ấy có một dòng dành cho khoảng trắng — con số của nó là 32 —
và khoảng trắng cũng là thứ mắt bạn nhìn xuyên qua.

Chỗ xuống dòng có một dòng của riêng nó trong đúng cái bảng đó. Con số của nó
là 10. Nó nằm cùng bảng với `A`, với dấu phẩy, với khoảng trắng, và nó chiếm
một chỗ y như chúng.

Nghĩa là: một file có ba dòng không phải là ba thứ nằm cạnh nhau. Nó là **một**
dãy chữ, trong đó những ký tự xuống dòng được cắm vào để chia dãy ấy ra. Người
viết dãy chữ ấy là bạn, nên những ký tự kia cũng phải do bạn đặt vào.
::::

::::explain{#go-hai-phim-cat-mot-cho}
Còn một chuyện phải nói trước khi gõ: viết ký tự ấy ra bằng cách nào?

Gõ phím Enter ngay giữa một dòng code thì bạn không viết được ký tự xuống dòng
vào chuỗi — bạn cắt đôi câu lệnh, và Python báo lỗi cú pháp. Nên Python cho
một lối viết riêng:

```python title=readonly
"\n"
```

Trên bàn phím bạn gõ **hai** phím: dấu gạch chéo ngược `\` rồi chữ `n`. Nhưng
lúc Python đọc chuỗi ấy, nó thấy cặp `\n` và hiểu thành **một** ký tự — đúng
cái ký tự mang số 10 trong bảng quy ước.

Hai phím gõ vào, một ô chữ đi ra. Chuyện này dễ trượt tay, nên bước ngay dưới
đây hỏi bạn trước khi nói tiếp.
::::

::::predict{#doan-dai-bao-nhieu commitOnce}
Byte đo hai chuỗi rất ngắn rồi in một chuỗi ra.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
print(len("ab"))
print(len("a\nb"))
print("a\nb")
```

:::opt{correct}
`2`, rồi `3`, rồi `a` và `b` nằm trên hai dòng
:::

:::opt
`2`, rồi `4`, rồi `a` và `b` nằm trên hai dòng
::why
Gần đúng ở chỗ bạn đọc đúng phần khó nhất: `\n` đúng là bắt máy xuống dòng, nên
hai chữ cái tách ra hai dòng. Bạn đọc trúng ý nghĩa của nó.

Chỗ lệch nằm ở phép đếm. Mắt bạn thấy bốn ký tự trên màn hình soạn thảo — `a`,
`\`, `n`, `b` — và đếm cả bốn. Nhưng `\` ở đây không phải một ký tự của chuỗi;
nó là dấu hiệu báo cho Python biết ký tự đứng ngay sau nó phải hiểu theo lối
khác. Python nuốt cả cặp `\n` và cất vào chuỗi đúng một ô. Nên chuỗi ấy có ba
ô: `a`, ký tự xuống dòng, `b`.
::
:::

:::opt
`2`, rồi `4`, rồi `a\nb` nằm gọn trên một dòng
::why
Gần đúng ở chỗ bạn giữ một nguyên tắc rất đáng giữ: thứ trong nháy thì máy đọc
nguyên văn, không tự diễn giải. Từ R0 bài 8 tới giờ nguyên tắc ấy đúng.

Chỗ lệch là dấu `\` — nó là một ngoại lệ được thoả thuận sẵn của nguyên tắc ấy.
Trong một chuỗi, gặp `\` thì Python không đọc nguyên văn nữa mà nhìn sang ký tự
kế bên để biết phải hiểu thành gì. Gặp `\n` nó hiểu thành ký tự xuống dòng, nên
lúc in ra bạn thấy dòng bị ngắt chứ không thấy hai ký tự `\` và `n`.
::
:::

:::opt
`2`, rồi `2`, rồi `a` và `b` nằm trên hai dòng
::why
Gần đúng ở chỗ bạn nhận ra ký tự ấy không hiện lên thành hình — và đúng là nó
không hiện lên thành hình.

Chỗ lệch là "không nhìn thấy" với "không tồn tại" là hai chuyện khác nhau.
Khoảng trắng cũng không nhìn thấy, mà `len(" ")` vẫn cho về một chỗ, đúng như
bài đo độ dài chuỗi ở T1.1 đã chỉ ra. Ký tự xuống dòng cũng vậy: nó có số
riêng trong bảng quy ước, nó chiếm một ô trong chuỗi, nên chuỗi `"a\nb"` dài
hơn chuỗi `"ab"` đúng một ô.
::
:::
::::

::::example{#ghi-so-co-ngat-dong}
Giờ đem ký tự ấy vào cuốn sổ. Mỗi khoản ghi xong thì đặt thêm một dấu xuống
dòng vào cuối:

```python title=readonly
with open("so.txt", "w") as f:
    f.write("cà phê,25000\n")

with open("so.txt", "a") as f:
    f.write("bún bò,40000\n")

with open("so.txt", "r") as f:
    ca_cuon_so = f.read()

print("[" + ca_cuon_so + "]")
```

Máy in ra:

```text title=readonly
[cà phê,25000
bún bò,40000
]
```

Hai dấu ngoặc vuông là mẹo nhìn, không phải nội dung của sổ: chúng được ghép
vào lúc in để bạn thấy chuỗi bắt đầu ở đâu và kết thúc ở đâu.

Và chúng cho xem một chuyện mà mắt thường bỏ sót: **dấu `]` không nằm cạnh chữ
`bún bò,40000`, nó rơi xuống dòng dưới.** Vì khoản thứ hai cũng kết thúc bằng
một ký tự xuống dòng, nên sau nó máy đã sang dòng mới rồi mới in `]`.

Ký tự xuống dòng cuối cùng ấy không thừa. Nó là thứ khiến khoản ghi sau — ngày
mai, lần chạy sau — bắt đầu ở một dòng sạch thay vì dính vào đuôi khoản hôm
nay.
::::

::::code{#moi-khoan-mot-dong}
Ba khoản, ba lần mở file, đúng như bài trước — nhưng lần này mỗi khoản phải nằm
riêng một dòng.

Nội dung ba khoản đã viết sẵn. Ba chỗ trống nằm ngay sau dấu `+`, tức là thứ
được ghép vào **sau** phần chữ của mỗi khoản.

Giữa chừng, chương trình đọc sổ lên một lần ngay sau khoản đầu tiên, để bạn
thấy dấu ngắt dòng đã có mặt từ khoản ấy chứ không phải chờ tới khoản cuối.

```python title=starter
with open("so.txt", "w") as f:
    f.write("cà phê,25000" + ___)

with open("so.txt", "r") as f:
    sau_khoan_dau = f.read()

with open("so.txt", "a") as f:
    f.write("bún bò,40000" + ___)

with open("so.txt", "a") as f:
    f.write("bánh mì,15000" + ___)

with open("so.txt", "r") as f:
    ca_cuon_so = f.read()

print("[" + sau_khoan_dau + "]")
print("[" + ca_cuon_so + "]")
```

```python title=solution
with open("so.txt", "w") as f:
    f.write("cà phê,25000" + "\n")

with open("so.txt", "r") as f:
    sau_khoan_dau = f.read()

with open("so.txt", "a") as f:
    f.write("bún bò,40000" + "\n")

with open("so.txt", "a") as f:
    f.write("bánh mì,15000" + "\n")

with open("so.txt", "r") as f:
    ca_cuon_so = f.read()

print("[" + sau_khoan_dau + "]")
print("[" + ca_cuon_so + "]")
```

```python title=test
# Chỗ trống thứ nhất bị soi bởi câu này: sổ được đọc lên ngay sau khoản đầu,
# nên thiếu dấu ngắt dòng ở đó là câu này vỡ, không đợi tới khoản cuối.
assert sau_khoan_dau == "cà phê,25000\n", "đọc sổ ngay sau khoản đầu tiên phải thấy đúng chuỗi cà phê,25000 rồi một ký tự xuống dòng — nếu ký tự ấy vắng mặt thì khoản ghi kế tiếp sẽ dính vào đuôi khoản này"
# Chỗ trống thứ hai và thứ ba bị soi bởi câu này: thiếu dấu ngắt dòng ở khoản
# nào thì khoản ấy dính liền với khoản sau nó, và chuỗi so ra khác ngay.
assert ca_cuon_so == "cà phê,25000\nbún bò,40000\nbánh mì,15000\n", "cả cuốn sổ phải là ba khoản cà phê,25000, bún bò,40000 và bánh mì,15000, mỗi khoản theo sau bởi đúng một ký tự xuống dòng"
# Hai câu trên đọc qua biến; câu này hỏi thẳng cái file trên đĩa.
with open("so.txt", "r") as kiem:
    assert kiem.read() == "cà phê,25000\nbún bò,40000\nbánh mì,15000\n", "chính so.txt trên đĩa phải giữ ba khoản cà phê,25000, bún bò,40000 và bánh mì,15000, mỗi khoản kết thúc bằng một ký tự xuống dòng"
```

:::hints
- kind: attention
  body: Ba chỗ trống đều nằm sau một dấu `+`, nên thứ điền vào phải ghép nối được với phần chữ đứng trước — tức là một chuỗi, và chuỗi thì viết trong nháy. Việc nó phải làm là đẩy khoản ghi tiếp theo xuống dòng mới.
- kind: strategy
  body: Thứ bạn cần là đúng một ký tự, cái ký tự mang số 10 trong bảng quy ước ở phần giải thích đầu bài. Trên bàn phím nó được gõ bằng hai phím — một dấu gạch chéo ngược rồi một chữ cái — và cả cặp ấy phải nằm gọn trong một cặp nháy để trở thành một chuỗi.
- kind: one-line
  body: 'Cả ba chỗ trống đều điền `"\n"`.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^\[cà phê,25000\n\]\n\[cà phê,25000\nbún bò,40000\nbánh mì,15000\n\]\s*$
- tier: output
  expect: "bánh mì,15000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba khoản, ba dòng. Cuốn sổ bắt đầu trông giống một cuốn sổ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mở `so.txt` bằng bất cứ trình soạn thảo nào, bạn thấy ba dòng ngay ngắn. Nhưng
đem nó về chương trình thì chuyện khác hẳn.

File có ba dòng rồi, nhưng `.read()` vẫn trả về đúng một chuỗi dài. Muốn xử lý
riêng từng khoản thì làm sao?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
