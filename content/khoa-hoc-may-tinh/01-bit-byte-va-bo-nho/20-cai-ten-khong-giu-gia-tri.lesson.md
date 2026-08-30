---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.cai-ten-khong-giu-gia-tri
title: "Cái tên không giữ giá trị — nó chỉ tới giá trị"
summary: "Một cái tên không phải là giá trị nó mang — nó chỉ giữ một con số, địa chỉ của giá trị ấy; và con số đó có thể đổi trong khi giá trị cũ vẫn còn nguyên."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [mem.name-is-reference]
requires: [mem.address, core.list-aliasing, core.reassign, core.list, core.list-append, core.variable, core.assignment, core.builtin-function, core.fstring, ctrl.comparison, core.boolean]
concepts: [mem.name-is-reference]
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
Cái tên `gio_1` không PHẢI là cái giỏ. Bài này nói ra nó thật sự đang giữ
cái gì.
::::

::::explain{#tam-the-giu-mot-con-so}
Nhớ lại chị Hạnh và hai nồi nước dùng: một tấm thẻ gỗ buộc vào quai nồi
đang bán, và chuyển thẻ sang nồi khác không hề đổ nồi cũ đi — nồi cũ vẫn
đứng nguyên chỗ, chỉ là không còn ai gọi tới nó qua tấm thẻ ấy nữa. Cái
tên bạn đặt trong Python — `gia_dac_biet`, `so_to_da_ban`, `gio_1` — chính
là tấm thẻ đó.

Bài trước cho bạn công cụ đọc **số nhà** của một giá trị: `id(...)`. Giờ
ghép hai điều lại thành một câu duy nhất, và đây là bản lề của cả cụm bài
bạn đang học:

> **Tấm thẻ không giữ cái nồi. Nó giữ số nhà của cái nồi.**

Viết `gio_1 = [10, 20, 5]` không phải "nhét cái giỏ vào cái tên `gio_1`".
Máy dựng cái giỏ ở đâu đó trong bộ nhớ, giỏ ấy có một địa chỉ, và
`gio_1` chỉ ghi nhớ đúng CON SỐ địa chỉ đó. Đọc tên `gio_1` nghĩa là "đi
tới địa chỉ đang ghi trong tấm thẻ, rồi lấy thứ nằm ở đó".

Điều này giải thích được HAI chuyện bạn đã từng thấy mà chưa ai nói vì
sao:

- **Gán lại** (bài "đổi giá trị của một cái tên") không sửa gì trong nồi
  cũ — nó chỉ ghi một con số ĐỊA CHỈ MỚI vào tấm thẻ.
- **Hai tên trỏ chung một nồi** (bài "hai cái tên, một cuốn sổ") không
  phải phép màu — hai tấm thẻ đơn giản là đang ghi CHUNG một con số địa
  chỉ, nên sửa qua thẻ nào cũng đụng vào đúng một nồi.
::::

::::example{#sua-nong-doi-the}
Byte dựng một giỏ rau, rồi làm hai việc khác hẳn nhau vào nó: trước tiên
**sửa tại chỗ**, sau đó **gán lại**.

```python title=readonly
a = [10, 20, 30]
dia_chi_truoc = id(a)

a.append(40)                    # SỬA tại chỗ — không cởi thẻ ra
dia_chi_sau_khi_sua = id(a)

print(dia_chi_truoc == dia_chi_sau_khi_sua)
```

```text title=readonly
True
```

`.append` viết thêm vào ĐÚNG cái giỏ mà `a` đang buộc tới — không dựng giỏ
mới, nên địa chỉ ghi trong tấm thẻ `a` không hề đổi. Sửa nồi không đổi số
nhà.

Giờ mượn tạm một tấm thẻ thứ hai, `b`, buộc vào đúng giỏ đó — để giỏ khỏi
"biến mất" khi `a` bỏ đi (bài sau sẽ mổ xẻ kỹ chuyện hai tấm thẻ dùng
chung một nồi; ở đây chỉ mượn tạm nó để làm chứng):

```python title=readonly
b = a
print(id(a) == id(b))

a = [99]                        # GÁN LẠI — cởi thẻ `a`, buộc sang nồi khác hẳn
print(id(a) == id(b))
print(b)
```

```text title=readonly
True
False
[10, 20, 30, 40]
```

Đọc từng dòng: `b = a` không tạo giỏ mới, nó đọc con số địa chỉ đang ghi
trong `a`, rồi chép ĐÚNG con số đó vào `b` — nên `id(a) == id(b)` là
`True`, hai tấm thẻ cùng buộc một nồi. Rồi `a = [99]` dựng một giỏ HOÀN
TOÀN MỚI ở một địa chỉ khác, và ghi địa chỉ mới ấy vào tấm thẻ `a` — chỉ
tấm thẻ `a` đổi chỗ, tấm thẻ `b` không ai đụng tới, nên nó vẫn buộc
nguyên chỗ cũ. Vì vậy `id(a) == id(b)` giờ là `False`, và `b` vẫn in ra
`[10, 20, 30, 40]` — cái giỏ cũ, còn nguyên luôn cả lượt sửa hồi nãy.

Một tên đổi giá trị, tên kia vẫn đứng yên — vì "đổi giá trị của một tên"
chưa bao giờ có nghĩa là "sửa cái giá trị đó". Nó có nghĩa là ghi một con
số địa chỉ khác vào đúng một tấm thẻ.
::::

::::predict{#doan-thuc-don-doi commitOnce}
Cô Bảy giữ một cái tên `thuc_don` cho bảng đang treo lên tường, mượn từ
cái tên `mon` đang có sẵn.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra gì?

```python
mon = ["phở bò", "phở gà"]
thuc_don = mon

mon = ["phở bò", "phở gà", "phở tái"]

print(thuc_don)
```

:::opt{correct}
`['phở bò', 'phở gà']`
:::

:::opt
`['phở bò', 'phở gà', 'phở tái']`
::why
Gần đúng ở chỗ ba món đó đều có thật trong chương trình — bạn không bịa
ra món nào, và cụm `["phở bò", "phở gà", "phở tái"]` đúng là xuất hiện
nguyên văn.

Chỗ lệch nằm ở dòng `mon = [...]` thứ hai. Đó là GÁN LẠI, không phải SỬA:
nó dựng một bảng thực đơn hoàn toàn mới, ghi địa chỉ mới vào tấm thẻ
`mon`, và không đụng gì tới cái bảng cũ. `thuc_don` đã tự giữ địa chỉ của
bảng CŨ từ dòng `thuc_don = mon` — nó không đi theo `mon` tới nồi mới.
::
:::

:::opt
Không in được gì, vì `mon` đã đổi nên `thuc_don` không còn trỏ vào đâu
::why
Gần đúng ở chỗ bạn cảm thấy đúng có gì đó "đứt" khi `mon` đổi — phản xạ
cảnh giác ấy không sai chút nào cho một bài đang học về tấm thẻ và địa
chỉ.

Chỗ lệch là `thuc_don` không phụ thuộc vào việc `mon` sau đó còn trỏ tới
đâu. Ngay từ dòng `thuc_don = mon`, tấm thẻ `thuc_don` đã tự ghi một con
số địa chỉ RIÊNG của nó — con số đó không tự xoá chỉ vì tấm thẻ `mon` sau
này đổi ý buộc sang chỗ khác.
::
:::

:::opt
Máy dừng lại báo lỗi, vì `mon` đã bị `thuc_don` "mượn" nên không gán lại được
::why
Gần đúng ở chỗ bạn nhớ đúng luật cũ: một tấm thẻ, tại một lúc, chỉ buộc
vào đúng một giá trị — bạn không quên điều đó.

Chỗ lệch là "bị mượn" không phải một trạng thái có thật trong Python. Gán
lại một cái tên lúc nào cũng hợp lệ, bất kể có bao nhiêu tấm thẻ khác
đang trỏ chung địa chỉ cũ của nó. Không có gì bị khoá, không tiếng báo lỗi
nào — máy chỉ đơn giản ghi một con số mới vào đúng tấm thẻ được nhắc tên.
::
:::
::::

::::code{#gio-rau-doi-hang}
Vườn rau có một giỏ rau. Byte giữ thêm một tấm thẻ `giu_lai` buộc vào
đúng giỏ ấy — rồi tỉa thêm lá vào giỏ (sửa tại chỗ), sau đó đổi hẳn sang
một giỏ MỚI để chở ra chợ (gán lại).

Dùng `id(...)` để kiểm tra: sau tất cả, `giu_lai` và `gio_rau` còn buộc
chung một giỏ không?

```python title=starter
gio_rau = ["cải", "muống", "ngót"]
giu_lai = gio_rau

gio_rau.append("dền")
gio_rau = ["mồng tơi"]

dia_chi_giu_lai_va_gio_rau_giong_nhau = ___

print(f"giu_lai = {giu_lai}")
print(f"gio_rau = {gio_rau}")
print(f"Còn trỏ chung giỏ không: {dia_chi_giu_lai_va_gio_rau_giong_nhau}")
```

```python title=solution
gio_rau = ["cải", "muống", "ngót"]
giu_lai = gio_rau

gio_rau.append("dền")
gio_rau = ["mồng tơi"]

dia_chi_giu_lai_va_gio_rau_giong_nhau = id(giu_lai) == id(gio_rau)

print(f"giu_lai = {giu_lai}")
print(f"gio_rau = {gio_rau}")
print(f"Còn trỏ chung giỏ không: {dia_chi_giu_lai_va_gio_rau_giong_nhau}")
```

```python title=test
assert giu_lai == ["cải", "muống", "ngót", "dền"], "giu_lai buộc vào giỏ CŨ suốt từ đầu — lượt .append xảy ra TRƯỚC khi gio_rau gán lại, nên giỏ cũ (và giu_lai) phải thấy dòng 'dền'"
assert gio_rau == ["mồng tơi"], "gio_rau đã được GÁN LẠI sang một giỏ hoàn toàn mới — đừng sửa lại dòng này"
assert dia_chi_giu_lai_va_gio_rau_giong_nhau is False, "sau khi gio_rau gán lại, hai tấm thẻ không còn buộc chung một giỏ nữa — so sánh bằng id(...) phải ra False"
```

:::hints
- kind: attention
  body: Chỗ trống phải trả lời đúng câu hỏi ở dòng in cuối cùng — "còn trỏ CHUNG giỏ không". Đó là câu hỏi về ĐỊA CHỈ, không phải về nội dung hai bên đang chứa gì.
- kind: strategy
  body: Lấy địa chỉ của từng tấm thẻ bằng id(giu_lai) và id(gio_rau), rồi so sánh hai con số đó bằng ==. Đừng so sánh nội dung (giu_lai == gio_rau) — hai giỏ có thể chứa khác nhau hoàn toàn mà câu hỏi vẫn đang hỏi về ĐỊA CHỈ, không phải về những gì đang nằm trong giỏ.
- kind: one-line
  body: "Điền `id(giu_lai) == id(gio_rau)` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải thật sự gọi id(...) trên cả hai tấm thẻ giu_lai và gio_rau rồi so sánh hai con số địa chỉ đó, không phải so sánh nội dung, gõ thẳng True/False, hay gọi id trên thứ khác
  requireAst:
  - kind: uses-call, target: id, min: 2
  - kind: uses-name, target: giu_lai, min: 2
  # `gio_rau` cũng phải bị đòi riêng, cùng lý do với `giu_lai` ở trên: thiếu
  # dòng này thì đổi MỌI chỗ đọc `gio_rau` trong chỗ trống thành `id` (cho ra
  # `id(giu_lai) == id(id)`) vẫn qua — `uses-call id min:2` không xét ĐỐI SỐ.
  # `min: 4`, không phải 2: đếm thật bằng `ast` cho thấy lời giải đúng đọc
  # `gio_rau` NGUYÊN VĂN 4 lần trong khối solution — dòng `giu_lai = gio_rau`,
  # dòng `gio_rau.append("dền")`, chỗ trống `id(gio_rau)`, và dòng print — nên
  # `min: 2` vẫn còn dư 3 lần đọc kể cả sau khi đột biến xoá đúng lần đọc ở
  # chỗ trống, không hề lộ ra. `min: 4` mới chạm đúng ngưỡng: xoá một lần đọc
  # (còn 3) là trượt. Không siết gì thêm lên người học — cả bốn lần đọc này
  # đều nằm trong phần KHUNG cố định, không phải phần họ gõ.
  - kind: uses-name, target: gio_rau, min: 4
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^giu_lai = \['cải', 'muống', 'ngót', 'dền'\]\ngio_rau = \['mồng tơi'\]\nCòn trỏ chung giỏ không: False\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một lượt sửa thấy ở cả hai tên, một lượt gán lại chỉ thấy ở một tên. Giờ
bạn biết chính xác vì sao.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy `.append` làm `giu_lai` đổi theo — dù dòng `.append` chỉ nhắc
tên `gio_rau`. Nhưng khi `gio_rau` bị gán lại, `giu_lai` lại KHÔNG đổi
theo. Cùng là "dòng lệnh không nhắc tới `giu_lai`", mà một lần nó đổi,
một lần nó không.

Vì sao lại khác nhau đến thế?

Đừng trả lời vội. Bài sau mổ xẻ đúng chuyện này — chuyện hai tấm thẻ buộc
chung một nồi.
::::

::::checkpoint{mastery=0.8}
::::
