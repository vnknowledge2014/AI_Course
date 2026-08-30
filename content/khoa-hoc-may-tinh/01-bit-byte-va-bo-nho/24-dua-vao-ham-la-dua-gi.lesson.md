---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.dua-vao-ham-la-dua-gi
title: Đưa vào hàm là đưa cái gì
summary: "Gọi hàm cũng chỉ là buộc thêm một tấm thẻ — tham số trùng địa chỉ với đối số — nên hàm sửa được nồi hay không tuỳ đúng vào luật của bài trước: nồi đó có sửa được tại chỗ không."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.pass-by-reference]
requires: [mem.mutability, core.argument-not-copy]
concepts: [mem.dia-chi, mem.ham, mem.tham-so]
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
Tấm thẻ mới vừa đứng bên trong hàm. Nó buộc vào đâu mới là chuyện đáng hỏi.
::::

::::explain{#goi-ham-cung-la-buoc-the}
Bài trước để lại một câu hỏi thẳng: đối số truyền vào hàm "không phải bản
sao" — vậy nó LÀ cái gì?

Nó là một tấm thẻ. Đúng nghĩa đen, không phải cách nói bóng bẩy.

Nhớ lại ví dụ hồi Realm 1: `them_mot_mon(so_cua_ban_ba)`. Bên trong hàm, cái
tên `so_goi_mon` đọc được, `.append` được, và cuốn sổ ngoài kia đổi theo —
dù trong thân hàm không hề có dòng `so_goi_mon = so_cua_ban_ba` nào cả.

Không có dòng đó thì cái gì đã buộc `so_goi_mon` vào đúng cuốn sổ ấy? Chính
lời gọi hàm. Viết `them_mot_mon(so_cua_ban_ba)` khiến máy làm đúng một việc,
âm thầm, ngay trước khi thân hàm chạy dòng đầu tiên: đọc xem `so_cua_ban_ba`
đang ghi số nhà nào, rồi buộc tham số `so_goi_mon` vào **đúng số nhà ấy**.
Y hệt một dòng `so_goi_mon = so_cua_ban_ba`, chỉ có điều dòng ấy do chính
lời gọi hàm viết ra, không phải do bạn gõ.

Đây là bài `hai-the-mot-noi` — chỉ khác chỗ tấm thẻ thứ hai này được buộc
bởi máy, tại đúng lúc hàm được gọi, và nó sống đúng trong một lượt gọi rồi
biến mất. Cuốn sổ mà nó từng trỏ tới thì không biến mất theo — cuốn sổ ấy
vẫn còn nguyên đó, có tấm thẻ khác đang trỏ vào.
::::

::::explain{#tuy-vao-noi-do-sua-duoc-khong}
Vậy hàm có sửa được thứ bạn đưa vào hay không? Câu trả lời không nằm ở hàm.
Nó nằm đúng ở luật bài trước: **nồi đó có sửa được tại chỗ hay không.**

- Đối số là `list` hay `dict`: tham số trùng địa chỉ với nó, và `.append()`
  hay gán vào một ô đều là sửa tại chỗ — nên chạm ra ngoài được, đúng như
  Realm 1 đã cho bạn thấy.
- Đối số là `tuple`, `str`, hay một con số: tham số cũng trùng địa chỉ với
  nó thật, nhưng không có phép sửa tại chỗ nào để mà viết ra. Thứ duy nhất
  hàm làm được là gán LẠI cái tên tham số sang một giá trị khác — và gán
  lại một tấm thẻ bên trong hàm chỉ dịch chuyển đúng tấm thẻ ấy. Tấm thẻ
  ngoài kia, cái đưa vào lúc gọi hàm, không hề hay biết.

Hàm không có phép màu riêng nào cả. Nó tuân theo đúng hai luật bạn đã học:
bài `hai-the-mot-noi` quyết định tham số có trùng địa chỉ với đối số
không (luôn luôn có — đó là cách gọi hàm hoạt động), còn bài
`sua-duoc-va-khong-sua-duoc` quyết định trùng địa chỉ đó có đáng lo hay
không.
::::

::::example{#hai-ham-hai-so-phan}
Hai hàm dưới đây đều chỉ nhận một đối số rồi thử "đổi" nó. Một hàm sửa tại
chỗ, một hàm gán lại tham số.

```python title=readonly
def them_mot_mon(so_goi_mon):
    print(f"  trong hàm, trùng địa chỉ: {so_goi_mon is so_cua_ban_ba}")
    so_goi_mon.append("nạm")

so_cua_ban_ba = ["tái", "chín"]
print(f"Trước khi gọi hàm: {so_cua_ban_ba}")
them_mot_mon(so_cua_ban_ba)
print(f"Sau khi gọi hàm:   {so_cua_ban_ba}")
```

Máy in ra:

```text title=readonly
Trước khi gọi hàm: ['tái', 'chín']
  trong hàm, trùng địa chỉ: True
Sau khi gọi hàm:   ['tái', 'chín', 'nạm']
```

Dòng `trùng địa chỉ: True` là bằng chứng: `so_goi_mon` và `so_cua_ban_ba`
đang buộc vào đúng một cuốn sổ, ngay từ trước khi thân hàm chạy dòng đầu
tiên. `.append("nạm")` sửa tại chỗ, nên cả hai tên đều thấy.

Giờ đổi đối số sang một chuỗi — thứ không sửa được tại chỗ:

```python title=readonly
def doi_ten(mon):
    mon = "bún bò"   # gán LẠI tham số, không sửa tại chỗ
    return mon

ten = "phở"
tra_ve = doi_ten(ten)
print(f"ten bên ngoài: {ten}")
print(f"tra_ve:        {tra_ve}")
```

```text title=readonly
ten bên ngoài: phở
tra_ve:        bún bò
```

Ngay khi hàm được gọi, `mon` cũng trùng địa chỉ với `ten` — y hệt bài trên.
Nhưng dòng `mon = "bún bò"` không sửa gì tại chỗ cả, vì chuỗi không có phép
sửa tại chỗ nào. Nó gán lại tấm thẻ `mon`, và tấm thẻ ấy chỉ sống trong lượt
gọi này. Tấm thẻ `ten` ngoài kia không hề bị đụng tới, nên nó vẫn buộc vào
đúng chuỗi `"phở"` như trước. Muốn cái tên ngoài kia thấy chuỗi mới thì phải
`return` nó ra, rồi gán lại ở ngoài — đúng như dòng `tra_ve = doi_ten(ten)`.
::::

::::predict{#doan-phuc-vu commitOnce}
Một hàm nhận hai đối số: giá món (`tuple`) và danh sách khách chờ (`list`).
Nó thử đổi cả hai — giảm giá bằng cách gán lại tham số, và đánh dấu khách
đầu tiên bằng cách sửa thẳng vào ô đầu của danh sách.

**Trước khi chạy**, bạn đoán hai dòng cuối in ra gì?

```python
def phuc_vu(gia, khach_cho):
    gia = (gia[0], gia[1] - 5000)
    khach_cho[0] = "đã phục vụ"
    return None

gia_goc = ("phở gà", 40000)
danh_sach = ["Lan", "Minh"]

phuc_vu(gia_goc, danh_sach)

print(f"gia_goc:   {gia_goc}")
print(f"danh_sach: {danh_sach}")
```

:::opt{correct}
`gia_goc: ('phở gà', 40000)` — không đổi, và
`danh_sach: ['đã phục vụ', 'Minh']` — có đổi
:::

:::opt
Cả hai đều không đổi: `gia_goc` như cũ, `danh_sach` vẫn `['Lan', 'Minh']`
::why
Gần đúng ở chỗ bạn đọc đúng nửa dòng `gia = (gia[0], gia[1] - 5000)`: đó là
một phép gán lại tham số, và phép ấy đúng là không chạm ra ngoài. Nửa đó
bạn trúng.

Chỗ lệch nằm ở dòng `khach_cho[0] = "đã phục vụ"`. Đó không phải gán lại cái
tên `khach_cho` — không có dấu `=` nào đứng ngay sau `khach_cho`, mà đứng
sau `khach_cho[0]`. Đây là sửa vào MỘT Ô của danh sách, y hệt `.append()`:
sửa tại chỗ, trên đúng cuốn sổ mà `khach_cho` đang trùng địa chỉ — nên
`danh_sach` ngoài kia phải thấy.
::
:::

:::opt
Cả hai đều đổi: `gia_goc` giảm 5000, `danh_sach` có "đã phục vụ"
::why
Gần đúng ở chỗ bạn nhớ đúng luật chung: tham số trùng địa chỉ với đối số,
nên hàm CÓ khả năng chạm ra ngoài. Phản xạ ấy đúng hướng.

Chỗ lệch là bạn quên hỏi tiếp: nồi đó có sửa được tại chỗ không. `gia` là
một `tuple`, và dòng `gia = (gia[0], gia[1] - 5000)` không sửa tại chỗ —
nó dựng một tuple MỚI rồi gán lại đúng tấm thẻ `gia` bên trong hàm. Tấm thẻ
`gia_goc` ngoài kia không hề bị đụng tới.
::
:::

:::opt
Ngược lại: `gia_goc` giảm 5000, còn `danh_sach` vẫn `['Lan', 'Minh']`
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra hai dòng trong hàm cư xử KHÁC nhau —
đúng là điều bài này đang dạy.

Chỗ lệch là bạn gán ngược vai hai dòng. `gia = (...)` là gán lại tham số
(không chạm ra ngoài, vì `tuple` không sửa được tại chỗ). `khach_cho[0] =
"đã phục vụ"` là sửa một ô (có chạm ra ngoài, vì `list` sửa được tại chỗ).
Đổi lại đúng chỗ: `gia_goc` phải giữ nguyên, còn `danh_sach` mới là bên đổi.
::
:::
::::

::::code{#don-hang-cua-lan}
Byte viết một hàm nhận giỏ hàng của khách và tên khách, rồi ghi thêm một ly
trà đá vào giỏ. Hàm cũng thử đổi tên khách ngay trong sổ — dòng đó đã viết
sẵn, cứ để nguyên, vì đây chính là chỗ chuyện KHÔNG chạm ra ngoài.

Điền dòng còn thiếu: thêm `"trà đá"` vào giỏ, sửa TẠI CHỖ trên đúng giỏ được
đưa vào.

```python title=starter
def don_hang_moi(gio, ten_khach):
    """Thêm 'trà đá' vào giỏ, và thử đổi tên khách trong sổ."""
    ___
    ten_khach = "khách lạ"
    return None

gio_cua_lan = ["phở bò"]
ten_trong_so = "Lan"

don_hang_moi(gio_cua_lan, ten_trong_so)

print(f"Giỏ của Lan sau khi gọi hàm:   {gio_cua_lan}")
print(f"Tên trong sổ sau khi gọi hàm:  {ten_trong_so}")
```

```python title=solution
def don_hang_moi(gio, ten_khach):
    """Thêm 'trà đá' vào giỏ, và thử đổi tên khách trong sổ."""
    gio.append("trà đá")
    ten_khach = "khách lạ"
    return None

gio_cua_lan = ["phở bò"]
ten_trong_so = "Lan"

don_hang_moi(gio_cua_lan, ten_trong_so)

print(f"Giỏ của Lan sau khi gọi hàm:   {gio_cua_lan}")
print(f"Tên trong sổ sau khi gọi hàm:  {ten_trong_so}")
```

```python title=test
assert gio_cua_lan == ["phở bò", "trà đá"], "gọi hàm xong, giỏ của Lan phải có thêm 'trà đá' — hàm phải sửa TẠI CHỖ trên đúng giỏ được đưa vào"
assert ten_trong_so == "Lan", "tham số ten_khach chỉ được GÁN LẠI bên trong hàm, và chuỗi không sửa được tại chỗ — nên tên ngoài sổ phải còn nguyên 'Lan'"
# Gọi lại hàm với một giỏ và một tên KHÁC — cái mà đoạn code trên không hề
# nhắc tên. Một lời giải lỡ ghi thẳng vào `gio_cua_lan` (cái tên nằm ngoài,
# hàm vẫn đọc được nó) sẽ qua được hai câu trên, nhưng cuốn thứ hai này thì
# nó không đụng tới — và đó là chỗ phân biệt.
gio_khac = ["bún bò"]
ten_khac = "Minh"
don_hang_moi(gio_khac, ten_khac)
assert gio_khac == ["bún bò", "trà đá"], "hàm phải sửa đúng giỏ ĐƯỢC ĐƯA VÀO ở lần gọi này — đưa gio_khac vào thì 'trà đá' phải nằm trong gio_khac, không phải trong gio_cua_lan"
assert ten_khac == "Minh", "tên khác cũng vậy — gán lại tham số bên trong hàm không được chạm tới ten_khac ở ngoài"
```

:::hints
- kind: attention
  body: Hàm có hai tham số. Việc cần làm là thêm một phần tử vào cuối tham số `gio` — không phải dán lại tấm nhãn nào, và không phải đụng vào `gio_cua_lan` hay bất kỳ tên nào bên ngoài hàm.
- kind: strategy
  body: Thêm một phần tử vào cuối một danh sách là việc bạn đã làm từ Realm 0, bằng một lệnh gắn sau tên danh sách và dấu chấm. Dùng đúng tên tham số `gio` — hàm chỉ nên đụng vào thứ nó nhận được qua tham số, để lần sau gọi với giỏ khác cũng chạy đúng.
- kind: one-line
  body: 'Viết `gio.append("trà đá")` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng mô tả ngay trên nó.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Giỏ của Lan sau khi gọi hàm:   \['phở bò', 'trà đá'\]\nTên trong sổ sau khi gọi hàm:  Lan\s*$
- tier: output
  expect: "Tên trong sổ sau khi gọi hàm:  Lan"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giỏ đổi, tên không đổi. Hàm không có phép màu — nó chỉ theo đúng hai luật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàm `don_hang_moi` chạy xong, tấm thẻ tham số `gio` bị gỡ bỏ — nó chỉ sống
đúng trong một lượt gọi, đúng như Realm 1 đã nói. Nhưng cái GIỎ mà nó từng
trỏ tới thì không biến mất theo tấm thẻ ấy: `gio_cua_lan` ở ngoài vẫn đang
trỏ vào đúng cái giỏ đó.

Vậy câu hỏi ngược lại: nếu một ngày, MỌI tấm thẻ trỏ tới một cái nồi đều bị
gỡ bỏ hết — không còn cái tên nào, ở đâu trong chương trình, dẫn tới được
nó nữa — thì cái nồi ấy đi đâu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
