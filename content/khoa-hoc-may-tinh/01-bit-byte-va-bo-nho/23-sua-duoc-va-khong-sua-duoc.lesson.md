---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.sua-duoc-va-khong-sua-duoc
title: Thứ sửa được và thứ không
summary: Hai tấm thẻ trùng địa chỉ chỉ đáng lo khi cái nồi đó SỬA ĐƯỢC tại chỗ — tuple và str chặn hẳn việc đó, nên chia sẻ chúng luôn an toàn.
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [mem.mutability]
requires: [mem.shallow-copy, core.tuple-immutable, core.string-immutable]
concepts: [mem.sua-duoc, mem.bat-bien, mem.dia-chi]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Chia sẻ một cái nồi là chuyện an toàn hay nguy hiểm? Câu trả lời chỉ
tuỳ một điều.
::::

::::explain{#cau-tra-loi-tuy-mot-dieu}
Hai bài vừa rồi cho bạn thấy hai lần dùng chung gây bất ngờ: hai tên cùng
trỏ vào một `list` thì sửa qua tên nào cũng lộ ra ở tên kia; chép nông rồi mà
cuốn con bên trong vẫn còn dùng chung cũng vậy. Cả hai lần đều xoay quanh
một hành động: `.append` — sửa cái nồi **tại chỗ**, không dựng nồi mới.

Nhưng Realm 1 từng cho bạn gặp hai loại giá trị từ chối thẳng việc đó. Sửa
một ký tự của chuỗi: máy báo `TypeError`. Sửa một ô của `tuple`: cũng
`TypeError`. Hai loại ấy có tên: giá trị **bất biến** — dựng ra rồi thì
không có cách nào sửa một mẩu bên trong nó nữa. Còn `list` và `dict` thì
ngược lại: **sửa được tại chỗ**, đó chính là lý do `.append` tồn tại.

Đặt hai điều này cạnh nhau thì ra một câu trả lời rất gọn cho câu hỏi mở đầu
bài: hai tấm thẻ trùng địa chỉ có đáng lo hay không, phụ thuộc đúng vào việc
cái nồi đó **sửa được tại chỗ hay không**.

- Nếu nồi **sửa được tại chỗ** (`list`, `dict`): một tấm thẻ đổi nồi, tấm
  thẻ kia thấy ngay. Chia sẻ nó là một quyết định cần cân nhắc.
- Nếu nồi **không sửa được tại chỗ** (`tuple`, `str`): không có cách nào để
  "đổi nồi tại chỗ" cả — máy chặn ngay từ đầu. Vậy thì trùng địa chỉ với nó
  chẳng có gì đáng lo. Cách duy nhất để "đổi" là gán lại cái TÊN sang một
  giá trị khác — và gán lại tên chỉ dịch chuyển ĐÚNG tấm thẻ vừa gán, tấm
  thẻ kia không hề hay biết.

Nói cách khác: bất biến không phải một luật tuỳ tiện của Python đặt ra cho
vui. Nó là thứ quyết định ai được phép dùng chung một địa chỉ mà không cần
lo lắng gì cả.
::::

::::example{#an-toan-va-can-than}
Chị Hạnh niêm yết giá một món bằng `tuple`, và giỏ hàng của một khách bằng
`list`. Cả hai đều được một tên thứ hai trỏ vào, giống hệt cách bài trước
đã làm.

```python title=readonly
gia_mon = ("phở bò", 40000)
gia_hien_thi = gia_mon
print(f"Trùng địa chỉ (tuple): {gia_mon is gia_hien_thi}")

gio_hang = ["phở bò"]
gio_hien_thi = gio_hang
gio_hien_thi.append("trà đá")
print(f"gio_hang:      {gio_hang}")
print(f"gio_hien_thi:  {gio_hien_thi}")
```

Máy in ra:

```text title=readonly
Trùng địa chỉ (tuple): True
gio_hang:      ['phở bò', 'trà đá']
gio_hien_thi:  ['phở bò', 'trà đá']
```

`gia_mon` và `gia_hien_thi` cũng trùng địa chỉ y hệt `gio_hang` với
`gio_hien_thi`. Chỉ khác một việc: không có dòng nào trong đoạn trên **thử**
sửa `gia_mon` tại chỗ — vì không có cách nào viết ra được. `gia_mon[1] = 0`
sẽ nổ `TypeError` ngay, đúng như Realm 1 đã cho bạn thấy với cả `tuple` lẫn
`str`. Nên dù `gia_mon` và `gia_hien_thi` trùng địa chỉ, chẳng có phép sửa
tại chỗ nào để mà lo bị lộ sang tên kia cả. An toàn không phải vì Python
"thương" hai cái tên này — mà vì con đường gây rắc rối (sửa tại chỗ) đã bị
chặn từ trước, ngay tại chính giá trị đó.
::::

::::predict{#doan-ghi-chu commitOnce}
Bảng ghi chú của quán đang viết `"còn 3 tô"`. Byte lưu thêm một tên nữa để
đối chiếu, rồi cập nhật bảng.

**Trước khi chạy**, bạn đoán hai dòng cuối in ra gì?

```python
ghi_chu = "còn 3 tô"
ghi_chu_kho = ghi_chu

ghi_chu = "còn 2 tô"

print(f"ghi_chu:     {ghi_chu}")
print(f"ghi_chu_kho: {ghi_chu_kho}")
```

:::opt{correct}
`ghi_chu: còn 2 tô` và `ghi_chu_kho: còn 3 tô` — hai dòng khác nhau
:::

:::opt
Cả hai đều in ra `còn 2 tô`
::why
Gần đúng ở chỗ bạn đang áp đúng bài học của hai bài trước: hai tên trùng
địa chỉ thì sửa qua tên nào cũng lộ ra ở tên kia. Phản xạ đó đúng với `list`.

Chỗ lệch là dòng `ghi_chu = "còn 2 tô"` không phải một phép **sửa tại chỗ**
— chuỗi không có phép sửa tại chỗ nào để mà viết ra. Đó là một phép **gán
lại**: gỡ tấm thẻ `ghi_chu` khỏi chuỗi cũ, buộc sang chuỗi mới. Tấm thẻ
`ghi_chu_kho` không hề bị đụng tới, nên nó vẫn đứng nguyên ở chuỗi cũ.
::
:::

:::opt
Cả hai đều in ra `còn 3 tô`
::why
Gần đúng ở chỗ bạn nhớ đúng: chuỗi bất biến, không sửa được. Đó là sự thật.

Chỗ lệch là bạn hiểu "không sửa được" thành "không gán lại được". Bất biến
chỉ cấm một việc: sửa một MẨU bên trong giá trị đã dựng. Nó không hề cấm
việc gán cái tên `ghi_chu` sang một giá trị hoàn toàn khác — dòng
`ghi_chu = "còn 2 tô"` chạy trót lọt, và sau dòng ấy `ghi_chu` thật sự đang
buộc vào chuỗi mới.
::
:::

:::opt
Máy báo lỗi, vì cái tên `ghi_chu` đã dùng ở dòng trên rồi
::why
Gần đúng ở chỗ bạn đang đọc dấu `=` như một lời tuyên bố kiểu sách toán —
tuyên bố hai lần với hai giá trị khác nhau thì nghe đúng là vô lý.

Chỗ lệch: `=` ở đây là một mệnh lệnh — "buộc tên bên trái vào giá trị bên
phải" — chứ không phải một lời khẳng định. Ra lệnh lần thứ hai thì máy chỉ
việc buộc lại. Cái bị cấm là sửa MỘT MẨU bên trong chuỗi cũ, ví dụ
`ghi_chu[0] = "C"`; còn gán lại cả cái tên thì luôn được phép, với bất kỳ
giá trị nào.
::
:::
::::

::::code{#doi-gia-dac-biet}
Đầu giờ, quán niêm yết giá đặc biệt bằng một `tuple`, và bảng điện tử ngoài
cửa lấy đúng tên ấy — hai tên, một địa chỉ. Trưa, chị Hạnh đổi giá đặc biệt
thêm 5000 đồng, nhưng CHỈ đổi thực đơn trong quầy; bảng điện ngoài cửa chưa
kịp cập nhật nên vẫn phải giữ giá cũ.

Vì `tuple` không sửa được tại chỗ, cách duy nhất để đổi giá là gán lại tên
`gia_hom_nay` sang một cặp mới. Điền dòng còn thiếu.

```python title=starter
gia_hom_nay = ("phở đặc biệt", 60000)
gia_bang_dien = gia_hom_nay

# Trưa, quán đổi giá đặc biệt thêm 5000 — CHỈ thực đơn trong quầy đổi.
___

print(f"Thực đơn quầy:  {gia_hom_nay}")
print(f"Bảng điện cửa:  {gia_bang_dien}")
```

```python title=solution
gia_hom_nay = ("phở đặc biệt", 60000)
gia_bang_dien = gia_hom_nay

# Trưa, quán đổi giá đặc biệt thêm 5000 — CHỈ thực đơn trong quầy đổi.
gia_hom_nay = ("phở đặc biệt", 65000)

print(f"Thực đơn quầy:  {gia_hom_nay}")
print(f"Bảng điện cửa:  {gia_bang_dien}")
```

```python title=test
assert gia_hom_nay == ("phở đặc biệt", 65000), "thực đơn trong quầy phải ghi giá mới: 60000 cộng thêm 5000 là 65000"
assert gia_bang_dien == ("phở đặc biệt", 60000), "bảng điện ngoài cửa CHƯA cập nhật — nó phải còn giữ đúng giá cũ 60000, vì gán lại một tên không đụng gì tới tên kia"
assert gia_hom_nay is not gia_bang_dien, "sau khi gán lại, hai tên phải KHÔNG còn trùng địa chỉ nữa — gia_hom_nay giờ trỏ tới một tuple khác hẳn"
```

:::hints
- kind: attention
  body: Chỗ trống là một dòng lệnh trọn vẹn, không phải một biểu thức chèn vào chỗ khác. Việc cần làm là gán LẠI cái tên `gia_hom_nay`, không phải sửa một ô của cặp đang có — cặp đang có là một tuple, không sửa được tại chỗ.
- kind: strategy
  body: Dựng một tuple MỚI với đúng hai phần — tên món giữ nguyên, còn tiền thì lấy tiền cũ cộng thêm 5000 — rồi gán tuple ấy cho `gia_hom_nay`. Tên `gia_bang_dien` không được nhắc tới ở dòng này, vì nó không cần đổi.
- kind: one-line
  body: 'Viết `gia_hom_nay = ("phở đặc biệt", 65000)` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Thực đơn quầy:  \('phở đặc biệt', 65000\)\nBảng điện cửa:  \('phở đặc biệt', 60000\)\s*$
- tier: output
  expect: "Bảng điện cửa:  ('phở đặc biệt', 60000)"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Quầy đổi giá, bảng ngoài cửa không hay biết gì. Không ai sửa lén được cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba bài vừa rồi đều xoay quanh hai tên cùng trỏ vào một chỗ — do bạn tự tay
viết dấu `=`. Nhưng có một chỗ khác cũng dán thêm một tấm thẻ mà bạn chưa hề
gõ dấu `=` nào ở đó: **gọi một hàm**.

Hồi Realm 1, bạn đã thấy `.append()` bên trong một hàm sửa luôn cuốn sổ nằm
ngoài kia, dù chẳng có `global` nào cả. Bài ấy chốt lại đúng một câu: đối số
truyền vào hàm "không phải bản sao".

Giờ bạn có đủ chữ để hỏi tiếp: nếu không phải bản sao, thì nó LÀ cái gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
