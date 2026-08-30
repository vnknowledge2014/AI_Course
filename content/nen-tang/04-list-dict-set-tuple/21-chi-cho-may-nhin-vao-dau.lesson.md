---
id: nen-tang.list-dict-set-tuple.chi-cho-may-nhin-vao-dau
title: Chỉ cho máy nhìn vào đâu
summary: '`key=` nhận một cái hàm; `sorted` gọi hàm đó trên từng phần tử để lấy ra đúng thứ đem so.'
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [core.sort-key]
requires: [core.tuple-compare, core.sorted, core.tuple, core.tuple-unpack, core.for-unpack, core.dict, core.dict-items, core.list, core.list-index, core.function-def, core.function-call, core.function-parameter, core.function-return, core.keyword-argument, core.docstring, ctrl.for-each, core.variable, core.assignment, core.fstring, core.output, err.type-error]
practices: [core.sorted, core.function-def, core.function-return, core.keyword-argument, core.for-unpack, core.fstring]
concepts: [core.danh-sach, core.ham, core.gia-tri]
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
Mình không tự đoán bạn muốn nhìn ô nào. Bạn chỉ chỗ, mình nhìn đúng chỗ đó.
::::

::::explain{#cat-ra-thi-mat-ten}
Bài trước để lại một thế kẹt rất cụ thể.

Giữ nguyên cặp thì `sorted` xếp theo tên — vì ô đầu của cặp là tên, và ô đầu
quyết định trước. Cắt riêng cột tiền ra như bài 19 thì xếp được theo tiền, nhưng
hàng số ấy đứng trơ một mình: nó nói `500000` mà không nói `500000` của khoản
nào. Cái tên rụng lại ở cuốn sổ cũ.

Hai đường đều hụt, và cả hai hụt vì cùng một lý do: bạn đang cố **đổi dữ liệu**
cho vừa với cách máy so, thay vì đổi **cách máy so** cho vừa với dữ liệu.

Thứ bạn thật sự muốn nói với `sorted` là một câu rất ngắn:

> Cứ giữ nguyên các cặp. Nhưng lúc so hai cặp với nhau, đừng nhìn ô đầu — nhìn
> ô thứ hai.

Python cho bạn nói đúng câu ấy.
::::

::::explain{#dua-cho-sorted-mot-cai-ham}
`sorted` nhận thêm một thứ nữa, gọi bằng tên: `key`. Cách gọi một hàm bằng tên
tham số bạn đã dùng ở mạch Hàm — `key=` chính là kiểu ấy.

Chỗ mới nằm ở **thứ bạn đưa cho nó**. `key` không nhận một con số, cũng không
nhận một chỗ đứng như `1`. Nó nhận **một cái hàm**.

Trước hết viết cái hàm ấy. Nó nhỏ tới mức gần như không có gì: nhận một cặp,
đưa ra ô tiền của cặp đó.

```python title=readonly
def lay_tien(cap):
    """Đưa ra ô tiền của một cặp (tên, tiền)."""
    return cap[1]
```

Rồi đưa tên hàm cho `sorted`:

```python title=readonly
chi = {
    "sửa xe": 500000,
    "cà phê": 25000,
    "biếu bà": 300000,
    "xăng xe": 60000,
    "mua sách": 40000,
}

cac_cap = list(chi.items())
theo_tien = sorted(cac_cap, key=lay_tien)

for ten, tien in theo_tien:
    print(f"{ten}: {tien} đồng")
```

```text title=readonly
cà phê: 25000 đồng
mua sách: 40000 đồng
xăng xe: 60000 đồng
biếu bà: 300000 đồng
sửa xe: 500000 đồng
```

Cột tiền đi từ nhỏ tới lớn, và mỗi con số vẫn còn nguyên cái tên đi kèm.

Cách `sorted` làm việc, nói cho hết: **trước khi xếp**, nó gọi hàm của bạn
**đúng một lần trên mỗi phần tử** để rút ra thứ đem so, rồi xếp hàng dựa trên
chính những thứ ấy. Không phải gọi lại mỗi lần so hai phần tử — sổ năm khoản
thì đúng năm lần gọi, đếm được. Với `lay_tien`, thứ rút ra là ô tiền — một con
số — nên máy so số với số. Cặp vẫn nguyên vẹn từ
đầu tới cuối; hàm `key` không sửa gì, nó chỉ **rút ra thứ đem so**.

Nói gọn: `key` là chỗ bạn chỉ tay cho máy biết nhìn vào đâu.
::::

::::explain{#dua-ten-ham-chu-khong-goi-no}
Có một chi tiết nhỏ trong dòng trên, và nó là chi tiết đáng nhìn nhất cả bài:

```python title=readonly
theo_tien = sorted(cac_cap, key=lay_tien)
```

Sau `lay_tien` **không có cặp ngoặc**.

Từ đầu khoá tới giờ, hễ viết tên một hàm là bạn viết kèm cặp ngoặc ngay sau —
`print(...)`, `len(...)`, `lay_tien(...)`. Cặp ngoặc ấy nghĩa là *gọi nó, chạy
nó, ngay bây giờ*. Ở đây bạn không muốn chạy `lay_tien` ngay bây giờ: bạn còn
chưa có cặp nào trong tay để đưa cho nó.

Thứ bạn muốn là **giao cái hàm cho `sorted` giữ**, để `sorted` gọi nó sau, lần
lượt trên từng cặp, đúng lúc nó cần. Nên bạn đưa đi chính cái tên trần.

Đây là lần đầu trong cả khoá một cái tên hàm được **đưa đi như một giá trị**,
y như đưa đi một con số hay một cuốn sổ. Hàm không phải một câu lệnh dính chặt
vào chỗ nó được viết ra — nó là một thứ có tên, và cái tên ấy trao tay được.

Viết nhầm `key=lay_tien()` thì bạn đang tự gọi nó ngay tại chỗ, mà không đưa cho
nó cặp nào — máy dừng lại và báo `TypeError`, kêu rằng thiếu mất phần đưa vào.
::::

::::predict{#key-tra-lai-cai-gi commitOnce}
Byte lấy ba khoản đầu sổ, xếp lại bằng `key=lay_tien`, rồi in **nguyên** danh
sách kết quả ra, không qua vòng lặp nào.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
cac_cap = [("sửa xe", 500000), ("cà phê", 25000), ("biếu bà", 300000)]

def lay_tien(cap):
    """Đưa ra ô tiền của một cặp (tên, tiền)."""
    return cap[1]

print(sorted(cac_cap, key=lay_tien))
```

:::opt{correct}
[('cà phê', 25000), ('biếu bà', 300000), ('sửa xe', 500000)]
:::

:::opt
[25000, 300000, 500000]
::why
Gần đúng ở chỗ bạn theo dõi rất sát việc `sorted` làm: nó gọi `lay_tien` trên
từng cặp, và thứ nhận về đúng là ba con số ấy, đúng theo thứ tự ấy. Phần đó bạn
đọc không sai chữ nào.

Chỗ lệch là ở việc mấy con số ấy dùng để làm gì. Chúng được dùng để **so**, rồi
bỏ. Thứ `sorted` xếp hàng và đưa lại vẫn là những phần tử bạn giao cho nó — tức
là các cặp. `key` đổi tiêu chuẩn so sánh, không đổi món hàng.
::
:::

:::opt
[('biếu bà', 300000), ('cà phê', 25000), ('sửa xe', 500000)]
::why
Gần đúng ở chỗ bạn nhớ chắc luật của bài trước: so hai cặp thì ô đầu quyết định,
nên danh sách cặp xếp theo tên. Luật ấy vẫn đúng nguyên vẹn — nó nói chuyện gì
xảy ra **khi máy đem hai cặp ra so với nhau**.

Chỗ lệch: có `key=` rồi thì máy không đem hai cặp ra so nữa. Nó so hai thứ mà
`lay_tien` đưa ra, và hai thứ ấy là hai con số, không phải hai cặp. Ô đầu của
cặp không còn được đụng tới, nên tên không tham gia quyết định thứ tự.
::
:::

:::opt
[('sửa xe', 500000), ('biếu bà', 300000), ('cà phê', 25000)]
::why
Gần đúng ở chỗ bạn nghĩ tới đúng thứ Byte đang cần: một bản báo cáo thì khoản
tốn nhất nên nằm trên đầu. Đó là cái đích, và bạn đang nhắm vào nó.

Chỗ lệch là ở chỗ `key` làm được gì. `sorted` xếp từ nhỏ tới lớn, và `key` không
đụng tới chiều ấy — nó nói cho máy biết nhìn vào đâu, chứ không nói nhìn theo
hướng nào. Nên kết quả vẫn là tiền tăng dần, và khoản tốn nhất nằm ở cuối hàng.
::
:::
::::

::::code{#xep-so-theo-tien}
Cuốn sổ của Byte, năm khoản, đã sẵn thành danh sách cặp theo thứ tự ghi sổ.

Hai chỗ trống nằm ở hai đầu của cùng một việc: một chỗ nói **rút ra thứ gì từ
mỗi cặp**, một chỗ **giao cái hàm ấy cho `sorted`**.

```python title=starter
chi = [("sửa xe", 500000), ("cà phê", 25000), ("biếu bà", 300000),
       ("xăng xe", 60000), ("mua sách", 40000)]

def lay_tien(cap):
    """Đưa ra ô tiền của một cặp (tên, tiền)."""
    return ___

theo_tien = sorted(chi, key=___)

for ten, tien in theo_tien:
    print(f"{ten}: {tien} đồng")
```

```python title=solution
chi = [("sửa xe", 500000), ("cà phê", 25000), ("biếu bà", 300000),
       ("xăng xe", 60000), ("mua sách", 40000)]

def lay_tien(cap):
    """Đưa ra ô tiền của một cặp (tên, tiền)."""
    return cap[1]

theo_tien = sorted(chi, key=lay_tien)

for ten, tien in theo_tien:
    print(f"{ten}: {tien} đồng")
```

```python title=test
# Câu đầu bắt cả hai chỗ trống một lượt: rút nhầm ô, hay giao nhầm thứ cho
# `key`, thì thứ tự năm cặp này không thể ra đúng như dưới đây.
assert theo_tien == [("cà phê", 25000), ("mua sách", 40000), ("xăng xe", 60000), ("biếu bà", 300000), ("sửa xe", 500000)], "năm cặp phải nằm theo tiền tăng dần: cà phê 25000, mua sách 40000, xăng xe 60000, biếu bà 300000, sửa xe 500000"
assert lay_tien(("cà phê", 25000)) == 25000, "gọi lay_tien trên cặp ('cà phê', 25000) phải đưa ra 25000 — hàm này rút ra ô tiền chứ không rút ô tên"
assert chi == [("sửa xe", 500000), ("cà phê", 25000), ("biếu bà", 300000), ("xăng xe", 60000), ("mua sách", 40000)], "sổ gốc phải còn nguyên thứ tự ghi: sửa xe, cà phê, biếu bà, xăng xe, mua sách"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm sau `return`, trong một hàm nhận vào một cặp tên là `cap`. Chỗ trống thứ hai nằm sau `key=`, và dòng mô tả ngay trên đã nói hàm kia làm gì — đó là cái hàm `sorted` cần mượn.
- kind: strategy
  body: Một cặp có hai ô, và bạn lấy ra một ô bằng chỗ đứng của nó y như với danh sách — ô tiền là ô thứ hai. Còn ở `key=`, thứ cần giao đi là chính cái hàm, nên viết tên nó trần, không kèm cặp ngoặc: kèm ngoặc là bạn tự gọi nó ngay lúc chưa có cặp nào trong tay.
- kind: one-line
  body: Viết `cap[1]` vào chỗ trống thứ nhất, và `lay_tien` vào chỗ trống thứ hai.
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: 'cà phê: 25000 đồng'
- tier: output
  expect: 'sửa xe: 500000 đồng'
- tier: static
  onFail: chỗ sau `key=` phải là tên của cái hàm vừa viết, để `sorted` gọi nó trên từng cặp
  requireAst:
  # Cái tên `lay_tien` chỉ được ĐỌC đúng một chỗ trong lời giải: ngay sau
  # `key=`. Dòng `def` đặt tên chứ không đọc tên, nên luật này hỏi trúng đúng
  # chỗ trống thứ hai và không hỏi gì khác.
  - kind: uses-name, target: lay_tien, min: 1
  - kind: uses-call, target: sorted, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bạn đưa mình cái hàm, mình gọi nó trên từng cặp. Nhìn đúng ô bạn chỉ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Xếp xong thì hàng chạy từ nhỏ tới lớn, nên khoản **rẻ nhất** nằm trên đầu và
khoản tốn nhất nằm tận cuối. Bản báo cáo của Byte thì cần ngược lại: tốn nhất
phải nằm trên đầu, vì đó là dòng người đọc nhìn thấy trước.

Bạn đang có trong tay một danh sách đã xếp đúng tiêu chuẩn, chỉ sai chiều.

Vậy làm sao? Xếp xong rồi tự tay lật ngược cả danh sách lại — duyệt từ cuối lên
đầu mà chép sang một danh sách khác? Hay `sorted` có sẵn một cái nút để bảo nó
xếp theo chiều kia ngay từ đầu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.85}
::::
