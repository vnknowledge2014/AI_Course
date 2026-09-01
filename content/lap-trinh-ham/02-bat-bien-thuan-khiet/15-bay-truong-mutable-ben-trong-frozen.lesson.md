---
id: lap-trinh-ham.bat-bien-thuan-khiet.bay-truong-mutable-ben-trong-frozen
title: "Bẫy: field bên trong `frozen=True` vẫn MUTABLE thì vẫn sửa được"
summary: "frozen=True chỉ chặn GÁN LẠI field — nếu field đó là list, sửa BÊN TRONG nó (.append) chạy trót lọt, không lỗi gì. Đổi field sang tuple mới thật sự chặn được cả hai đường."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.nested-mutable-trap]
requires: [fp.frozenset]
concepts: [fp.nested-mutable-trap]
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
Câu trả lời không phải điều bạn nghĩ: `frozen=True` chặn ÍT hơn bạn
tưởng. Đây là chỗ nhiều người viết Python thật sự vấp.
::::

::::explain{#chan-gan-lai-khong-chan-sua-ben-trong}
`frozen=True` chặn ĐÚNG MỘT việc: gán LẠI một field, tức là đổi field
đó sang TRỎ VÀO một giá trị khác (`gio.mon = [...]`). Nó không hề biết
— và không hề quan tâm — GIÁ TRỊ hiện tại field đang trỏ vào là gì, hay
giá trị đó có tự sửa được tại chỗ hay không.

Nếu field đó là `tuple`, chuyện không thành vấn đề: `tuple` tự nó đã
không sửa được tại chỗ (bài trước-trước đó), nên "không gán lại field"
cộng với "field không tự sửa được" ra một đối tượng đóng băng THẬT SỰ,
ở mọi tầng.

Nhưng nếu field đó là `list`? `list` VẪN sửa được tại chỗ — `.append()`
vẫn còn nguyên đó, không hề bị `frozen=True` động tới. Gọi
`gio.mon.append(...)` không phải GÁN LẠI field `mon` — nó gọi một
phương thức sửa TRÊN GIÁ TRỊ mà field đang trỏ tới. `frozen=True` không
can thiệp, vì việc đó chưa bao giờ chạm tới cơ chế nó canh gác.

Đã đo thật, đây là bẫy CÓ THẬT, không phải một trường hợp hiếm gặp:
```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Gio:
    mon: list

gio = Gio(["Phở", "Bún"])
gio.mon.append("Chả cá")   # CHẠY TRÓT LỌT, không lỗi gì
```

`gio` là "frozen" — nhưng `gio.mon` vẫn đổi được, vì `frozen=True` chỉ
đóng băng đúng lớp OUTER: cái tên `mon` đang trỏ vào đâu. Nó không đóng
băng những gì NẰM BÊN TRONG chỗ đó trỏ tới.
::::

::::example{#hai-duong-mot-frozen}
```python title=readonly
from dataclasses import dataclass

@dataclass(frozen=True)
class Gio:
    mon: list

gio = Gio(["Phở", "Bún"])
gio.mon.append("Chả cá")
print(f"mon: {gio.mon}")
```

```text title=readonly
mon: ['Phở', 'Bún', 'Chả cá']
```

Không lỗi nào cả — `gio.mon` giờ có ba món. Nhưng thử GÁN LẠI cả field,
thay vì sửa bên trong nó:

```python title=readonly
gio.mon = ["Phở mới"]
```

```text title=readonly
dataclasses.FrozenInstanceError: cannot assign to field 'mon'
```

Hai dòng trông giống nhau — cùng chạm tới `gio.mon` — nhưng một dòng là
GÁN LẠI field (bị chặn), một dòng là SỬA BÊN TRONG field (không bị chặn
gì cả). `frozen=True` chỉ canh gác đúng dòng đầu tiên.
::::

::::predict{#doan-dong-nao-chan-duoc commitOnce}
Một giỏ hàng đóng băng, field `mon` kiểu `list`.

**Trước khi chạy**, bạn đoán dòng cuối in ra gì?

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Gio:
    mon: list

gio = Gio(["Phở", "Bún"])
gio.mon.append("Chả cá")
print(gio.mon)
```

:::opt{correct}
Chạy trót lọt, in ra `['Phở', 'Bún', 'Chả cá']` — không lỗi gì
:::

:::opt
Máy dừng lại, báo `dataclasses.FrozenInstanceError: cannot assign to
field 'mon'`
::why
Gần đúng ở việc bạn nhớ đúng: `Gio` thật sự là `frozen=True`, và
`FrozenInstanceError` thật sự là lỗi có thể xảy ra với nó.

Chỗ lệch: `gio.mon.append(...)` không GÁN LẠI field `mon` — nó gọi một
phương thức sửa NGAY TRÊN giá trị `mon` đang trỏ tới. `frozen=True` chỉ
canh gác việc gán lại field, không canh gác những gì xảy ra bên trong
giá trị field đó đang giữ. Dòng này chạy trót lọt.
::
:::

:::opt
Máy báo `AttributeError: 'list' object has no attribute 'append'`
::why
Gần đúng ở việc bạn nhớ đúng có một kiểu dữ liệu trong track này không
có `.append()` — đó là `tuple`.

Chỗ lệch: field `mon` ở đây được khai `list`, không phải `tuple`. `list`
VẪN CÓ `.append()` — phương thức đó chưa từng bị lấy đi, dù `Gio` có
`frozen=True` hay không. Frozen chỉ ảnh hưởng tới FIELD của `Gio`, hoàn
toàn không ảnh hưởng tới các phương thức của GIÁ TRỊ nằm trong field
đó.
::
:::

:::opt
In ra `['Phở', 'Bún']`, không đổi — phép sửa bị âm thầm bỏ qua vì
`Gio` là frozen
::why
Gần đúng ở việc bạn tin `frozen=True` ảnh hưởng tới CẢ giỏ hàng, không
chỉ riêng field.

Chỗ lệch: khi `frozen=True` chặn một việc, nó chặn bằng cách NÉM LỖI
DỪNG CHƯƠNG TRÌNH — không bao giờ âm thầm bỏ qua. Ở đây `.append(...)`
không hề bị chặn (vì lý do đã giải thích ở trên), nên nó chạy, và
`gio.mon` THẬT SỰ có ba món, không phải hai.
::
:::
::::

::::code{#that-su-dong-bang}
`Gio` bên dưới vẫn còn bẫy: field `mon` khai là `list`. Sửa lời gọi
tạo `gio` để field đó thật sự là một `tuple` — kiểu dữ liệu không có
`.append()` — rồi kiểm tra: `.append(...)` giờ phải bị chặn bằng
`AttributeError`.

```python title=starter
from dataclasses import dataclass

@dataclass(frozen=True)
class Gio:
    mon: tuple

gio = Gio(___)

da_chan_dung = False
loai_loi = None
try:
    gio.mon.append("Chả cá")
except Exception as loi:
    da_chan_dung = True
    loai_loi = type(loi).__name__

print(f"mon: {gio.mon}")
print(f"Bị chặn: {da_chan_dung}")
print(f"Loại lỗi: {loai_loi}")
```

```python title=solution
from dataclasses import dataclass

@dataclass(frozen=True)
class Gio:
    mon: tuple

gio = Gio(("Phở", "Bún"))

da_chan_dung = False
loai_loi = None
try:
    gio.mon.append("Chả cá")
except Exception as loi:
    da_chan_dung = True
    loai_loi = type(loi).__name__

print(f"mon: {gio.mon}")
print(f"Bị chặn: {da_chan_dung}")
print(f"Loại lỗi: {loai_loi}")
```

```python title=test
assert isinstance(gio.mon, tuple), "gio.mon phải THẬT SỰ là một tuple — truyền một tuple vào Gio(...), không phải list, kể cả khi field đã khai kiểu tuple"
assert gio.mon == ("Phở", "Bún"), "gio.mon phải đúng hai món: Phở, Bún"
assert da_chan_dung is True, "gọi .append() trên gio.mon phải bị chặn — điều đó chỉ xảy ra khi mon thật sự là một tuple, không phải list"
assert loai_loi == "AttributeError", f"lỗi ném ra phải là AttributeError, đang là {loai_loi}"
```

:::hints
- kind: attention
  body: Field mon đã khai kiểu tuple trong class — nhưng khai KIỂU không tự đổi GIÁ TRỊ được truyền vào. Chỗ trống là chính giá trị đó.
- kind: strategy
  body: Python không tự kiểm tra field truyền vào có đúng khớp annotation hay không lúc chạy. Muốn .append() thật sự bị chặn, giá trị truyền vào Gio(...) phải LÀ một tuple, viết bằng dấu ngoặc tròn.
- kind: one-line
  body: 'Điền `("Phở", "Bún")` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^mon: \('Phở', 'Bún'\)\nBị chặn: True\nLoại lỗi: AttributeError\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Field khai `tuple`, giá trị THẬT SỰ là `tuple` — hai điều phải khớp
nhau. Giờ `.append()` không còn đường nào lách qua nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đổi kiểu dữ liệu sang `tuple` chặn triệt để — nhưng không phải lúc nào
cũng làm được. Đôi khi field đó CẦN là `list`, vì phần còn lại của
chương trình đang dựa vào các phương thức của `list`. Có cách nào lấy
một bản ĐỘC LẬP của toàn bộ dữ liệu — kể cả các lớp lồng bên trong —
mà không cần đổi kiểu field gì cả?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
