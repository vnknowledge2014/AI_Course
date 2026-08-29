---
id: nen-tang.gia-tri-bien-kieu.khoang-trang-vo-hinh
title: Khoảng trắng ở hai đầu
summary: Dấu cách cũng là một ký tự, và `.strip()` cắt phần thừa ở hai đầu chuỗi mà không đụng vào phần giữa.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.string-strip]
requires: [core.string-literal, core.input-returns-str]
concepts: [core.chuoi, core.chuan-hoa]
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
Chỗ khác nhau nằm ở hai đầu. Mắt bạn không thấy nó, nhưng mình đếm được.
::::

::::explain{#dau-cach-cung-la-mot-ky-tu}
Bài trước để lại một câu đố: hai bên đã cùng chữ thường rồi mà

```python
print(" cà phê ".lower() == "cà phê")
```

vẫn in ra `False`. Chúng khác nhau ở một chỗ mắt không nhìn thấy.

Chỗ ấy là **dấu cách**.

Bạn đã biết từ mấy bài trước: chuỗi là một dãy ký tự có thứ tự, và `==` so từng
ký tự một. Nhưng có một điều dễ quên trong luật đó — **dấu cách cũng là một ký
tự**, đứng xếp hàng ngang hàng với `c`, với `à`, với `p`. Nó chỉ có mỗi tội là
không để lại vết mực nào trên màn hình.

Đếm thì thấy ngay:

```python
print(len(" cà phê "))
print(len("cà phê"))
```

Máy in ra `8` rồi `6`. Hai ký tự chênh nhau ấy là hai dấu cách ở hai đầu, và
với `==` thì hai chuỗi này khác nhau y như `"cà phê"` khác `"trà đá"`.

Chuyện này không phải do bạn gõ ẩu mà ra. Nó đến từ `input()`. Người ngồi trước
màn hình gõ tên khoản rồi quen tay đập thêm một phát space trước khi bấm Enter;
người khác dán từ tin nhắn sang, dính luôn dấu cách của câu trước. Cái mà
`input()` trao lại cho bạn là **nguyên văn** những gì họ gõ, kể cả phần lề
trắng.

Hình dung tên khoản như chữ viết trên một mẩu giấy cắt ra từ tờ lớn. Chữ thì
đúng, nhưng hai đầu mẩu giấy còn chừa lề trắng. Bạn cần một cái kéo xén hai đầu
ấy đi mà không đụng tới chữ.

Cái kéo đó có tên: **`.strip()`**. Trong tiếng Anh *strip* là tước đi, lột đi —
ở đây là tước phần khoảng trắng thừa.
::::

::::example{#keo-xen-hai-dau}
Byte cho hai người gõ cùng một tên khoản, mỗi người thừa khoảng trắng một đầu:

```python title=readonly
lan_mot = "   cà phê"
lan_hai = "cà phê   "

print(len(lan_mot), len(lan_hai))
print(len(lan_mot.strip()), len(lan_hai.strip()))
print(lan_mot.strip() == lan_hai.strip())
```

Màn hình:

```text
9 9
6 6
True
```

Ba điều đáng ghi lại từ ba dòng này.

Thứ nhất, `.strip()` gọi giống hệt `.lower()` của bài trước: một dấu chấm sau
giá trị, rồi tên, rồi cặp ngoặc. Đó là **phương thức** — và luật của nó vẫn
nguyên: nó **trả về một chuỗi mới**, còn `lan_mot` sau đó vẫn dài đúng 9 ký tự
như cũ. Không hứng lấy kết quả thì coi như chưa làm gì.

Thứ hai, nó xén ở **cả hai đầu**, không phân biệt đầu nào. Người thứ nhất thừa
ở đầu trái, người thứ hai thừa ở đầu phải, cả hai đều về `6`.

Thứ ba — và đây là chỗ dễ hiểu nhầm nhất — nó **không đụng vào phần giữa**:

```python title=readonly
print(len("  cà phê sữa  "))
print(len("  cà phê sữa  ".strip()))
```

`14` rồi `10`. Bốn dấu cách hai đầu bị xén; hai dấu cách nằm giữa `cà`, `phê`
và `sữa` thì còn nguyên. `.strip()` ăn từ ngoài vào, và **dừng lại ngay khi
chạm ký tự đầu tiên không phải khoảng trắng**. Phần giữa nằm sau chốt chặn đó
nên nó không với tới.

Điều này đúng như bạn cần: dấu cách giữa hai chữ là nội dung thật của cái tên,
xén đi thì thành `càphêsữa`, không ai đọc nổi.
::::

::::predict{#doan-ba-dong commitOnce}
Ô nhập tên khoản nhận về một chuỗi thừa khoảng trắng ở cả hai đầu. Byte xén nó
rồi cất vào một cái tên khác.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
o_nhap = "  cà phê sữa  "
sach = o_nhap.strip()
print(len(sach))
print(sach)
```

:::opt{correct}
10, rồi dòng chữ: cà phê sữa
:::

:::opt
8, rồi dòng chữ: càphêsữa
::why
Gần đúng ở chỗ bạn hiểu `.strip()` là lệnh dọn khoảng trắng, và bạn áp nó cho
mọi khoảng trắng có trong chuỗi — cách hiểu ấy khớp trọn vẹn với chữ "xén
khoảng trắng".

Chỗ lệch nằm ở phạm vi. `.strip()` chỉ ăn từ **hai đầu** vào, và nó dừng ngay
khi gặp ký tự đầu tiên không phải khoảng trắng — ở đây là chữ `c` bên trái và
chữ `a` bên phải. Hai dấu cách nằm giữa `cà`, `phê`, `sữa` nấp phía sau hai
chốt chặn đó nên chúng an toàn. Còn 10 ký tự, không phải 8.
::
:::

:::opt
14, rồi dòng chữ có cả khoảng trắng hai đầu: (hai dấu cách) cà phê sữa (hai dấu cách)
::why
Gần đúng ở chỗ bạn nhớ đúng luật vừa học ở bài trước: một phương thức chuỗi
**không sửa chuỗi cũ**, nó trả về một chuỗi mới. Nếu dòng 2 viết trống trơn là
`o_nhap.strip()` rồi in `o_nhap` ra, bạn đã đoán trúng từng ký tự.

Chỗ lệch nằm ở dòng 2 của đoạn này: kết quả có được hứng lấy, và nó được đặt
tên là `sach`. Hai dòng `print` phía dưới hỏi `sach` chứ không hỏi `o_nhap`.
Chuỗi cũ vẫn dài 14 ký tự thật — chỉ là bài này không in nó ra.
::
:::

:::opt
12, rồi dòng chữ có một dấu cách ở mỗi đầu
::why
Gần đúng ở chỗ bạn hình dung `.strip()` làm việc như một nhát cắt: đi vào một
bước ở mỗi đầu rồi thôi. Với chuỗi thừa đúng một dấu cách mỗi bên thì cách hình
dung này cho ra kết quả đúng.

Chỗ lệch: nó không cắt một nhát mà **ăn liên tục**, hết dấu cách này tới dấu
cách kia, cho tới khi chạm ký tự không phải khoảng trắng mới dừng. Hai dấu cách
hay hai chục dấu cách thì kết quả cũng như nhau — đó chính là lý do dùng được
nó cho `input()`, nơi bạn không biết trước người ta gõ thừa mấy phát.
::
:::
::::

::::explain{#noi-hai-nhat-keo}
Còn một chuyện nhỏ mà tiện: `.strip()` trả về một **chuỗi**. Mà đã là chuỗi thì
nó gọi được phương thức của chuỗi.

Nên hai việc dọn dẹp viết được liền một dòng:

```python
o_nhap = "  Cà Phê  "
ten = o_nhap.strip().lower()
print(ten)
```

Máy in ra `cà phê`.

Đọc từ trái sang phải như đọc một dây chuyền: `o_nhap` đi qua `.strip()` ra một
chuỗi mới đã xén hai đầu, chuỗi mới ấy đi tiếp qua `.lower()` ra một chuỗi mới
nữa đã hạ hết chữ hoa. Cái tên `ten` hứng lấy đầu ra cuối cùng.

Không có luật mới nào ở đây cả — chỉ là luật cũ dùng hai lần. Nhưng nó là cách
viết bạn sẽ gặp ở mọi chỗ có `input()`: xén rồi hạ, một dòng, xong.
::::

::::code{#don-ba-lan-go}
Ba người khách gõ cùng một tên khoản vào sổ, mỗi người thừa khoảng trắng một
kiểu: người thứ nhất thừa ở đầu trái, người thứ hai thừa ở đầu phải, người thứ
ba thừa cả hai đầu. Byte cần cả ba về đúng một dạng để so với tên đã có trong
sổ.

Ba chỗ trống nằm ngay sau dấu chấm. Điền vào đó lời gọi xén hai đầu.

Ba kiểu thừa ở đây được chọn có chủ ý: một lời gọi chỉ dọn được đầu trái sẽ đưa
người thứ hai về `9` chứ không phải `6`, nên bài chỉ xanh khi phép xén ăn cả
hai đầu.

```python title=starter
lan_mot = "   cà phê"
lan_hai = "cà phê   "
lan_ba = "  cà phê  "

ten_mot = lan_mot.___
ten_hai = lan_hai.___
ten_ba = lan_ba.___

print(len(ten_mot), len(ten_hai), len(ten_ba))
print(ten_ba)
```

```python title=solution
lan_mot = "   cà phê"
lan_hai = "cà phê   "
lan_ba = "  cà phê  "

ten_mot = lan_mot.strip()
ten_hai = lan_hai.strip()
ten_ba = lan_ba.strip()

print(len(ten_mot), len(ten_hai), len(ten_ba))
print(ten_ba)
```

```python title=test
# Chấm trên BA tình huống chứ không phải một, vì một tình huống thì không phân
# biệt nổi đúng với gặp may:
#   thừa bên trái  (lan_mot) → phép chỉ dọn đầu phải để nguyên 9 ký tự;
#   thừa bên phải  (lan_hai) → phép chỉ dọn đầu trái để nguyên 9 ký tự;
#   thừa cả hai    (lan_ba)  → phép dọn một nhát mỗi đầu để lại 8 ký tự.
# Chỉ một phép ăn liên tục ở CẢ HAI đầu mới đưa được cả ba về đúng "cà phê".
assert ten_mot == "cà phê", "người thứ nhất quệt thừa khoảng trắng ở đầu trái, dọn xong phải còn đúng tên khoản cà phê"
assert ten_hai == "cà phê", "người thứ hai quệt thừa ở đầu phải, dọn xong cũng phải về đúng tên khoản ấy"
assert ten_ba == "cà phê", "người thứ ba thừa cả hai đầu, mà chỉ sót lại một dấu cách thôi là sổ đã coi đây là khoản khác"
# Và chuỗi gốc phải còn nguyên: phương thức trả về chuỗi mới, không sửa tại chỗ.
assert lan_ba == "  cà phê  ", "dọn là để đem so cho khớp; nguyên văn người thứ ba gõ vẫn còn khoảng trắng ở hai đầu"
```

:::hints
- kind: attention
  body: Nhìn kỹ ba dòng đầu — `lan_mot` thừa khoảng trắng ở đầu trái, `lan_hai` thừa ở đầu phải, `lan_ba` thừa cả hai. Cả ba chỗ trống đều đứng ngay sau một dấu chấm, đúng chỗ dành cho tên một phương thức.
- kind: strategy
  body: Bạn cần phép xén khoảng trắng ở hai đầu mà không đụng phần giữa, viết y như cách gọi `.lower()` của bài trước: tên phương thức rồi cặp ngoặc rỗng. Kết quả phải được hứng lấy bằng dấu `=` phía trước, vì chuỗi cũ không tự đổi.
- kind: one-line
  body: "Điền `strip()` vào cả ba chỗ trống, ví dụ dòng đầu thành `ten_mot = lan_mot.strip()`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^6 6 6\ncà phê\s*$
- tier: static
  onFail: ba chỗ trống cần một lời gọi phương thức xén khoảng trắng, không phải một chuỗi viết cứng
  requireAst:
  - kind: uses-call, target: strip, min: 3
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba kiểu gõ, một dạng duy nhất. Giờ mình so tên khoản nào cũng khớp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cột "tên khoản" trên sổ đã sạch: xén hai đầu, hạ chữ thường, vừa 12 chỗ. Bên
cạnh nó là cột "tiền", rộng 10 chỗ.

Muốn biết một khoản có vừa cột tiền không thì phải đếm xem nó chiếm mấy chỗ.
Bạn đã có sẵn cái thước đo: `len`. Vậy gõ thử `len(25000)` xem máy nói gì.

Nó không trả về `5`. Nó cũng không trả về `0`. Nó dừng lại và báo một lỗi.

Vì sao cái thước dùng được cho `"cà phê"` mà lại từ chối `25000`? Bài sau trả
lời, và câu trả lời hé ra một chuyện lớn hơn: con số với dãy chữ số viết ra con
số ấy là **hai thứ khác nhau**.
::::

::::checkpoint{mastery=0.8}
::::
