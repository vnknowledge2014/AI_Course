---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.hai-the-mot-noi
title: Hai tấm thẻ buộc vào một cái nồi
summary: "`=` không dựng nồi thứ hai — nó buộc thêm một tấm thẻ vào đúng địa chỉ cũ, và đó chính là điều làm bạn giật mình hồi còn học Realm 1."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [mem.aliasing-explained]
requires: [mem.name-is-reference]
concepts: [mem.dia-chi, mem.ten, mem.danh-sach]
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
Cùng một dòng `.append` không hề nhắc tên `giu_lai` — mà `giu_lai` vẫn đổi.
::::

::::explain{#vi-sao-mot-lan-doi-mot-lan-khong}
Bài trước để lại đúng một câu hỏi: `.append` làm `giu_lai` đổi theo `gio_rau`,
dù dòng lệnh chẳng hề nhắc tên `giu_lai`. Nhưng khi `gio_rau` bị **gán lại**,
`giu_lai` lại đứng yên. Cùng là "dòng lệnh không nhắc tới `giu_lai`", một lần
đổi, một lần không. Vì sao?

Câu trả lời nằm gọn trong một câu bài trước đã dựng: tấm thẻ không giữ cái
nồi, nó giữ **số nhà** của cái nồi. Từ câu đó, hai việc bạn vừa làm hoá ra
khác hẳn nhau:

- `.append` **sửa tại chỗ** — nó không đụng gì tới tấm thẻ cả, chỉ viết
  thêm vào đúng cái nồi mà tấm thẻ đang trỏ tới. Số nhà ghi trên tấm thẻ
  không đổi một chữ số nào. Bất kỳ tấm thẻ nào khác đang ghi CÙNG số nhà ấy
  cũng sẽ thấy, vì chúng đang mở đúng một cái nồi.
- Gán lại là **sửa tấm thẻ**, không đụng tới nồi. `gio_rau = ["mồng tơi"]`
  dựng một nồi mới, rồi đổi con số ghi trên tấm thẻ `gio_rau`. Tấm thẻ
  `giu_lai` không ai động tới, nên nó vẫn ghi nguyên con số cũ — vẫn trỏ
  đúng cái nồi cũ, y như trước.

Nói gọn: **`.append` đi qua tấm thẻ để tới nồi. Gán lại chỉ sửa mỗi tấm
thẻ.** Hai tấm thẻ cùng ghi một số nhà thì cùng thấy mọi lượt sửa tại chỗ —
không phải phép màu, chỉ là cả hai đang mở đúng một cái nồi.

Chuyện hai tấm thẻ cùng chung một số nhà có tên gọi: **trùng địa chỉ**. Bài
trước hỏi chuyện ấy bằng `id(giu_lai) == id(gio_rau)` — đúng, nhưng dài.
Python có một câu hỏi ngắn hơn, hỏi thẳng đúng điều đó: không phải "hai thứ
này có giống nhau không" (đó là `==`, so nội dung), mà "hai tấm thẻ này có
đang buộc vào cùng một chỗ không". Câu hỏi ấy viết bằng từ khoá `is`.
::::

::::example{#nhin-tan-mat}
Nồi nước dùng của chị Hạnh sáng nay ninh xương bò. Chị ghi ra một danh sách
nguyên liệu còn thiếu, rồi đưa cho phụ bếp tên Tâm cùng nhìn — không chép lại,
chỉ đưa đúng cái tên.

```python title=readonly
nguyen_lieu = ["xương bò", "hành tây", "gừng"]
phu_bep = nguyen_lieu

phu_bep.append("quế")

print(f"Chị Hạnh thấy: {nguyen_lieu}")
print(f"Phụ bếp thấy:  {phu_bep}")
print(f"Trùng địa chỉ: {nguyen_lieu is phu_bep}")
```

Máy in ra:

```text title=readonly
Chị Hạnh thấy: ['xương bò', 'hành tây', 'gừng', 'quế']
Phụ bếp thấy:  ['xương bò', 'hành tây', 'gừng', 'quế']
Trùng địa chỉ: True
```

Không có dòng nào viết `nguyen_lieu.append(...)`. Chỉ có `phu_bep.append(...)`
— vậy mà tên `nguyen_lieu` cũng thấy "quế". Dòng cuối nói huỵch toẹt lý do:
`nguyen_lieu is phu_bep` ra `True`. Hai tấm thẻ, một địa chỉ, một cái nồi.
Tâm không cần chạy sang hỏi chị Hạnh "chị ơi con vừa ghi thêm quế đấy nhé" —
chị Hạnh mở tấm thẻ của mình ra là thấy ngay, vì đó là đúng cái nồi ấy.
::::

::::predict{#doan-don-hang commitOnce}
Một bàn gọi món, ghi vào một `dict`. Người viết đơn ở bàn và người bếp nhận
đơn dùng hai cái tên khác nhau cho **cùng một** đơn — y như `so`/`so_moi` hồi
nãy, chỉ đổi từ `list` sang `dict`.

**Trước khi chạy**, bạn đoán hai dòng cuối in ra gì?

```python
don = {"mon": "phở bò", "so_luong": 1}
don_bep = don

don_bep["so_luong"] = 2

print(f"Bàn ghi: {don}")
print(f"Bếp ghi: {don_bep}")
```

:::opt{correct}
Cả hai dòng đều ghi `so_luong` là `2` — giống hệt nhau
:::

:::opt
Bàn ghi `so_luong` là `1`, Bếp ghi `so_luong` là `2`
::why
Gần đúng ở chỗ bạn nhớ đúng một luật có thật, chỉ vừa học ở R1: khi hai tên
KHÔNG trùng địa chỉ, sửa qua tên này không ai bên kia thấy.

Chỗ lệch là `don_bep = don` không tạo ra hai địa chỉ. Nó làm đúng việc
`phu_bep = nguyen_lieu` vừa làm: đọc `don` đang ghi số nhà nào, buộc `don_bep`
vào đúng số nhà ấy. Sửa một ô của `dict` qua tên nào cũng là sửa **cùng một**
`dict`, y như `.append` vào cùng một `list`.
::
:::

:::opt
Cả hai dòng đều ghi `so_luong` là `3`
::why
Gần đúng ở chỗ bạn tin đúng nửa quan trọng: hai tên cùng thấy một con số,
không đứa nào lệch đứa nào. Phần đó trúng.

Chỗ lệch nằm ở phép tính. Dòng `don_bep["so_luong"] = 2` là một phép **gán**
— nó đặt số `2` vào ô ấy, thay hẳn cho số cũ. Nó không phải `+=`, nên không
có chuyện `1` cộng thêm `2` thành `3`. Ô "so_luong" sau dòng ấy chỉ còn giữ
đúng con số vừa gán, không nhớ số cũ.
::
:::

:::opt
Dòng "Bàn ghi" không hiện gì, vì `don` đã "chuyển" hẳn sang `don_bep`
::why
Gần đúng ở chỗ bạn nhớ đúng luật gán lại MỘT tên: hồi chị Hạnh chuyển tấm
thẻ từ quai nồi bò sang quai nồi gà, tấm thẻ cũ gỡ ra khỏi nồi cũ, buộc sang
nồi mới — chỉ còn một tấm thẻ, đứng ở chỗ mới.

Chỗ lệch là ở đây có **hai** tên khác nhau, không phải một tên gán lại hai
lần. `don_bep = don` không gỡ tấm thẻ `don` ra khỏi đâu cả — nó buộc THÊM một
tấm thẻ mới vào đúng cái `dict` mà `don` đang buộc vào. Tấm thẻ `don` vẫn
đứng nguyên chỗ cũ, và chỗ cũ ấy chính là chỗ mà tấm thẻ mới cũng đang trỏ
tới.
::
:::
::::

::::code{#khach-cho-va-le-tan}
Chị Hạnh có một danh sách khách đang chờ bàn. Lễ tân cần một cái tên riêng để
gọi thêm khách mới vào — nhưng chỉ có **một** danh sách khách chờ, ghi qua
tên nào thì chị Hạnh cũng phải thấy ngay, không cần ai chạy ra báo.

Điền vế phải của dòng còn thiếu, sao cho tên `so_le_tan` trùng địa chỉ với
`khach_dang_cho`.

```python title=starter
khach_dang_cho = ["Lan", "Minh"]

# Lễ tân ghi thêm khách mới qua tên `so_le_tan`. Chỉ có MỘT danh sách chờ.
so_le_tan = ___

so_le_tan.append("chị Tư")

print(f"Chị Hạnh thấy: {khach_dang_cho}")
print(f"Lễ tân thấy:   {so_le_tan}")
print(f"Trùng địa chỉ: {khach_dang_cho is so_le_tan}")
```

```python title=solution
khach_dang_cho = ["Lan", "Minh"]

# Lễ tân ghi thêm khách mới qua tên `so_le_tan`. Chỉ có MỘT danh sách chờ.
so_le_tan = khach_dang_cho

so_le_tan.append("chị Tư")

print(f"Chị Hạnh thấy: {khach_dang_cho}")
print(f"Lễ tân thấy:   {so_le_tan}")
print(f"Trùng địa chỉ: {khach_dang_cho is so_le_tan}")
```

```python title=test
assert khach_dang_cho == ["Lan", "Minh", "chị Tư"], "ghi 'chị Tư' qua tên so_le_tan thì khach_dang_cho cũng phải thấy dòng ấy — hai tên phải đang buộc vào MỘT danh sách"
assert khach_dang_cho is so_le_tan, "so_le_tan phải trùng địa chỉ với khach_dang_cho, không phải một danh sách khác tình cờ giống nội dung"
# Ghi thêm một khách NỮA, sau khi đoạn trên đã chạy xong: hai tên phải dính
# vào nhau ở MỌI lần ghi, không chỉ ở lần ghi đầu tiên.
so_le_tan.append("anh Hải")
assert khach_dang_cho == ["Lan", "Minh", "chị Tư", "anh Hải"], "ghi thêm 'anh Hải' qua so_le_tan thì khach_dang_cho cũng phải có dòng ấy"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở vế phải một dấu bằng. Danh sách khách chờ đã có tên sẵn ở dòng đầu tiên rồi — việc cần làm là trỏ tới đúng nó, không phải dựng ra một danh sách nào khác.
- kind: strategy
  body: Đây đúng là hình dạng vừa gặp trong ví dụ, chỉ đổi tên gọi. Đừng gõ tay lại `["Lan", "Minh"]`, và cũng đừng cắt lát `[:]` — cả hai cách đó đều dựng ra một địa chỉ MỚI, còn bài này cần hai tên chung một địa chỉ CŨ.
- kind: one-line
  body: 'Viết `khach_dang_cho` vào chỗ trống, thành `so_le_tan = khach_dang_cho`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Chị Hạnh thấy: \['Lan', 'Minh', 'chị Tư'\]\nLễ tân thấy:   \['Lan', 'Minh', 'chị Tư'\]\nTrùng địa chỉ: True\s*$
- tier: output
  expect: "Trùng địa chỉ: True"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai tấm thẻ, một cái nồi. Giờ bạn đoán trước được, không còn giật mình nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay bạn chỉ thử với những nồi chứa toàn chữ — tên khách, tên nguyên liệu.
Nhưng nồi của chị Hạnh cũng có lúc chứa nồi con bên trong nó: một tô phở
không chỉ có bánh và thịt, nó còn có một danh sách "món kèm" nằm lồng bên
trong tô ấy.

Realm 1 đã cho bạn một cách tách hẳn một cuốn ra làm hai địa chỉ: `so[:]` —
cắt trọn cả cuốn. Bây giờ bạn biết vì sao nó tách được rồi: nó dựng ra một
địa chỉ mới, không phải buộc thêm thẻ vào địa chỉ cũ.

Nhưng nếu cuốn đó có một cuốn CON nằm bên trong, `so[:]` có tách luôn được
cả cuốn con hay không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
