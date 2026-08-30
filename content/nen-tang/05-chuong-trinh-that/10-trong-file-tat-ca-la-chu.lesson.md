---
id: nen-tang.chuong-trinh-that.trong-file-tat-ca-la-chu
title: Trong file, tất cả đều là chữ
summary: "Một file văn bản chỉ giữ được chữ. Con số bạn ghi xuống, đọc lên vẫn là chữ — nên muốn cộng thì phải `int()` lại, đúng như với `input()` ở Realm 0."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [io.text-is-str]
requires: [core.string-split, core.strip-newline, io.readlines, core.with-open, core.file-read, core.file-write, core.int-cast, core.input-returns-str, core.string-concat, core.string-strip, core.type-fn, core.accumulator, core.list, core.list-index, core.len, core.fstring, core.variable, core.assignment, core.output, ctrl.for-each]
concepts: [core.file, core.chuoi, core.kieu-gia-tri]
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
Cái đĩa chỉ biết cất chữ. Con số của bạn đi xuống đó là thành chữ hết.
::::

::::explain{#hai-khoan-dinh-thanh-mot-day}
Bài trước đã cắt được một dòng thành từng mảnh. `"cà phê,25000".split(",")` cho
về `["cà phê", "25000"]`, và `manh[1]` là mảnh giữ tiền.

Rồi bạn cộng hai khoản đầu lại, và máy in ra `2500040000` — không lỗi, không
traceback, chỉ là hai khoản dính liền nhau thành một dãy mười chữ số. Câu hỏi
cuối bài trước là đúng câu này: `"25000" + "40000"` ra `"2500040000"`. Vì sao?

Câu trả lời không nằm ở `.split()`. Nó nằm ở chỗ **file văn bản chỉ cất được
chữ**.

Nghĩ lại cái file bạn vừa ghi ra ở những bài trước. Trên đĩa, dòng cà phê nằm
đó dưới dạng một chuỗi ký tự nối đuôi nhau: `c`, `à`, ` `, `p`, `h`, `ê`, `,`,
rồi `2`, `5`, `0`, `0`, `0`, rồi ký tự xuống dòng. Năm ký tự chữ số ấy **không
phải** một con số — chúng là năm chữ, y hệt như chữ `c` đứng đầu dòng là một
chữ. Một file văn bản không có chỗ nào để cất một con số cả; nó chỉ có chỗ cất ký
tự. (Sâu xuống nữa thì R0 đã dạy rồi: bản thân mỗi ký tự ấy cũng được ghi
xuống bằng một con số của bảng quy ước. Nhưng con số ấy là số của **chữ** `2`,
không phải số hai — hai chuyện khác nhau, và chỗ này đang nói chuyện thứ hai.)

Nên khi `.readlines()` mang nội dung ấy về, thứ mang về là chữ. `.strip()` cắt
chữ. `.split(",")` cắt chữ ra thành những mẩu chữ nhỏ hơn. Từ đầu tới cuối
không có bước nào biến chữ thành số, nên `manh[1]` là chuỗi `"25000"`, không
phải số 25000.

Và với hai chuỗi thì dấu `+` có một nghĩa khác hẳn: **nối chúng lại**. Realm 0
đã dạy đúng chuyện này ở bài nối chuỗi — `"Xin " + "chào"` cho `"Xin chào"`.
Máy không nhầm gì cả; nó làm đúng việc bạn bảo, chỉ là bạn tưởng mình đang bảo
nó cộng tiền.

Chuyện này bạn cũng đã gặp một lần rồi, ở một cái cửa khác. Realm 0 nói: thứ
`input()` đưa về **luôn** là chữ, kể cả khi người ta gõ toàn chữ số. Bây giờ
thêm một cái cửa nữa cùng tính chất: thứ đọc lên từ file **luôn** là chữ, kể cả
khi trên đĩa toàn chữ số. Hai cửa khác nhau, cùng một luật, và cùng một cách
chữa — công cụ `int()` mà bạn đã dùng cho `input()`:

```python title=readonly
int("25000")
```

`int` nhận một chuỗi toàn chữ số và đưa lại con số thật mà nó viết ra. Từ chỗ
đó trở đi thì cộng trừ được bình thường.
::::

::::example{#doc-len-van-la-chu}
Ghi hai dòng xuống, đọc lên, rồi soi thẳng vào mảnh tiền:

```python title=readonly
with open("so_vi_du.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")

with open("so_vi_du.txt", "r") as f:
    cac_dong = f.readlines()

dong_dau = cac_dong[0].strip()
manh = dong_dau.split(",")

print(manh)
print(type(manh[1]))
print(manh[1] + manh[1])
print(int(manh[1]) + int(manh[1]))
```

Máy in ra:

```text title=readonly
['cà phê', '25000']
<class 'str'>
2500025000
50000
```

Bốn dòng ấy kể trọn câu chuyện:

- **`['cà phê', '25000']`** — hai mảnh, và cả hai đều nằm trong dấu nháy. Dấu
  nháy là cách Python nói "cái này là chữ". Mảnh tiền có nháy y như mảnh tên.
- **`<class 'str'>`** — hỏi thẳng máy xem mảnh tiền thuộc kiểu gì, máy trả lời
  `str`, tức chuỗi. Không phải `int`.
- **`2500025000`** — `+` giữa hai chuỗi là nối. Nó đặt `25000` rồi lại `25000`
  cạnh nhau, không chèn gì vào giữa, và ra một dãy mười chữ số vô nghĩa.
- **`50000`** — bọc mỗi mảnh trong `int()` trước, rồi mới `+`. Lúc này hai vế
  đều là số thật, và `+` quay về nghĩa cộng.

Một chỗ đáng để ý: dòng thứ ba và dòng thứ tư viết gần giống nhau, khác đúng
hai lời gọi `int`. Một dãy vô nghĩa và một con số đúng chỉ cách nhau chừng ấy.
::::

::::predict{#doan-cong-hai-cot-tien commitOnce}
Byte ghi hai khoản xuống file, đọc lên, lấy mảnh tiền của từng dòng, rồi cộng
hai mảnh ấy lại.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
with open("thu_hai_dong.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")

with open("thu_hai_dong.txt", "r") as f:
    cac_dong = f.readlines()

a = cac_dong[0].strip().split(",")[1]
b = cac_dong[1].strip().split(",")[1]

print(a + b)
```

:::opt{correct}
`2500040000`
:::

:::opt
`65000`
::why
Gần đúng ở chỗ bạn tính không sai một đồng: hai khoản ấy cộng lại đúng là sáu
mươi lăm nghìn, và đó chính là con số Byte muốn có.

Chỗ lệch nằm ở nghĩa của dấu `+`, mà nghĩa ấy phụ thuộc vào **kiểu** của hai
vế. Cả `a` lẫn `b` đều vừa đi qua ba bước — đọc lên từ file, `.strip()`,
`.split(",")` — và không bước nào biến chữ thành số. Nên hai vế đều là chuỗi,
và `+` giữa hai chuỗi là nối, đúng như Realm 0 đã dạy. Muốn có 65000 thì phải
bọc mỗi vế trong `int()` trước.
::
:::

:::opt
Máy dừng và báo `TypeError`
::why
Gần đúng ở chỗ bạn nhớ đúng một cái bẫy có thật: Realm 0 dựng hẳn một bài cho
`TypeError`, và nó nổ đúng khi bạn bảo máy cộng một câu chữ với một con số.

Chỗ lệch là ở đây **không có** con số nào. Cả hai vế đều là chữ, nên máy không
gặp hai kiểu chỏi nhau; nó có sẵn một quy ước cho chữ cộng chữ, và quy ước đó
là nối. `TypeError` chỉ nổ khi hai kiểu không đi cùng nhau được — chữ với chữ
thì đi cùng nhau rất êm, chỉ là êm theo cái nghĩa bạn không muốn.
::
:::

:::opt
`25000 40000`
::why
Gần đúng ở chỗ bạn đoán trúng phần khó nhất: hai mảnh sẽ đứng cạnh nhau chứ
không cộng lại thành một con số.

Chỗ lệch chỉ còn là dấu cách ở giữa. Dấu cách ấy là việc của `print` khi bạn
đưa cho nó **nhiều** thứ cách nhau bởi dấu phẩy — `print(a, b)` mới chèn một
dấu cách. Ở đây `print` chỉ nhận đúng một thứ: cái chuỗi mà `a + b` vừa nối ra,
và phép nối không chèn gì vào giữa cả.
::
:::
::::

::::code{#cong-cot-tien-cua-ca-so}
Ba khoản trong sổ, và một câu hỏi: cả ba cộng lại hết bao nhiêu.

Chương trình đã ghi sổ xuống file, đọc lên thành từng dòng, cắt mỗi dòng thành
hai mảnh. Chỗ trống nằm ở đúng bước cuối cùng còn thiếu.

```python title=starter
with open("so_bai_muoi.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("vở ghi,15000\n")

with open("so_bai_muoi.txt", "r") as f:
    cac_dong = f.readlines()

tong = 0
for dong in cac_dong:
    manh = dong.strip().split(",")
    tong = tong + ___

print(f"Ba khoản trong sổ cộng lại: {tong} đồng")
```

```python title=solution
with open("so_bai_muoi.txt", "w") as f:
    f.write("cà phê,25000\n")
    f.write("bún bò,40000\n")
    f.write("vở ghi,15000\n")

with open("so_bai_muoi.txt", "r") as f:
    cac_dong = f.readlines()

tong = 0
for dong in cac_dong:
    manh = dong.strip().split(",")
    tong = tong + int(manh[1])

print(f"Ba khoản trong sổ cộng lại: {tong} đồng")
```

```python title=test
# Chỗ trống là chỗ DUY NHẤT quyết định `tong`, nên mọi cách điền sai đều lộ ra
# ở câu đầu tiên:
#
#   `manh[1]` mà quên `int`  → `0 + "25000"` nổ TypeError, chương trình dừng
#   `manh[0]`                → `int("cà phê")` nổ ValueError, chương trình dừng
#   một hằng số bất kỳ       → ba lượt cộng cùng một con số, mà 80000 không
#                              chia hết cho 3 nên không hằng số nguyên nào trúng
assert tong == 80000, "ba khoản trong sổ là 25000, 40000 và 15000; cộng lại phải ra 80000"
assert cac_dong[2].strip() == "vở ghi,15000", "dòng thứ ba đọc lên từ file phải còn nguyên là 'vở ghi,15000' — vòng lặp này chỉ đọc sổ, không sửa sổ"
assert len(cac_dong) == 3, "sổ có ba dòng, và cả ba đều phải được đọc lên rồi duyệt qua"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay sau dấu `+`, tức là chỗ đáng ra phải là số tiền của dòng đang xét. Ở lượt này bạn đang cầm `manh` — hai mảnh vừa cắt ra từ một dòng, và cả hai đều là chữ.
- kind: strategy
  body: Mảnh đứng sau dấu phẩy là mảnh giữ tiền, nhưng nó giữ dưới dạng chữ, nên đem cộng thẳng thì `+` sẽ nối chứ không cộng. Đổi nó thành số thật trước đã, bằng đúng công cụ bạn dùng cho `input()` ở Realm 0.
- kind: one-line
  body: "Viết `int(manh[1])` vào chỗ trống."
:::

:::validate
- tier: static
  onFail: chỗ trống phải ĐỔI mảnh tiền của dòng đang xét thành số, không chép sẵn một con số nào
  requireAst:
  # CHỈ đòi gọi `int`. Bản trước đòi thêm `uses-name target: manh, min: 1` và
  # luật ấy đánh trượt một lời giải đúng: người học bỏ biến bắc cầu, viết
  # thẳng `int(dong.strip().split(",")[1])`, thì `manh` không được đọc lần nào.
  #
  # Việc chặn đáp án chép cứng để `int` và mấy `assert` lo — chúng chấm trên
  # ba dòng sổ khác nhau nên một hằng số gõ sẵn không qua nổi.
  - kind: uses-call, target: int, min: 1
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: "Ba khoản trong sổ cộng lại: 80000 đồng"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chữ nằm trên đĩa, số nằm trong chương trình. `int` là cái cửa giữa hai bên.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cuốn sổ thật của bạn không dừng ở ba dòng. Cuối tháng nó dài sáu chục dòng, và
một hôm nào đó bạn gõ vội, dòng thứ 41 thành ra thế này:

```text title=readonly
cà phê,hai lăm nghìn
```

`int("hai lăm nghìn")` không có cách nào đưa ra một con số — đó đúng là
`ValueError` mà Realm 0 đã đặt tên. Chương trình chết ngay tại dòng đó. Bốn
mươi khoản trước nó đã cộng xong xuôi, nằm gọn trong `tong`, và cũng mất trắng
theo: `print` ở cuối không bao giờ chạy tới.

Một dòng gõ nhầm giết cả cuốn sổ. Bỏ qua dòng hỏng mà đi tiếp được không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
