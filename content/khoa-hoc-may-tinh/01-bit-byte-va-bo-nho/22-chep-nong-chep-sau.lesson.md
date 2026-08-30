---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.chep-nong-chep-sau
title: Chép cái nồi hay chép tấm thẻ
summary: "`a[:]` dựng một nồi mới thật, nhưng chỉ tách được lớp ngoài — nồi con nằm bên trong thì hai bản vẫn dùng chung, và đây là chỗ phải cho bạn vấp thật."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [mem.shallow-copy]
requires: [mem.aliasing-explained, core.slice-copy]
concepts: [mem.dia-chi, mem.sao-chep, mem.danh-sach]
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
Hôm nay bạn tách nồi ra thật — gần hết. Có một góc nồi vẫn còn dính chung.
::::

::::explain{#cat-ca-noi-la-dung-noi-moi}
Bài trước để lại một câu hỏi: `so[:]` — lát cắt cả cuốn, hồi Realm 1 đã dạy —
có tách được hẳn thành một địa chỉ khác không?

Có. Và giờ bạn giải thích được vì sao, bằng đúng chữ vừa học: `so[:]` không
đọc số nhà của `so` rồi buộc thêm thẻ vào đó — nó **dựng một cuốn hoàn toàn
mới**, rồi chép nội dung từng ô sang cuốn mới ấy. Cuốn mới có địa chỉ riêng.
Việc này có tên: **chép nông** — chép, nhưng chỉ chép đúng MỘT lớp.

"Một lớp" là chữ đáng dừng lại. Nếu mọi ô trong cuốn đều là số hay chữ, một
lớp là đủ tách hẳn hai cuốn ra, không dây dưa gì với nhau nữa. Nhưng nếu MỘT
ô trong cuốn lại là một cuốn CON — một `list` nằm trong `list` — thì chuyện
khác hẳn.

Nhớ lại bài trước: cái nằm trong một ô của `list` không phải chính giá trị,
mà là một **địa chỉ** dẫn tới giá trị. Với ô chứa số 5, "chép nông" chép
thẳng con số 5 sang — không có gì để dây dưa, vì số không có địa chỉ riêng
để mà chia sẻ. Nhưng với ô chứa một cuốn con, thứ nằm trong ô đó vốn dĩ là
MỘT ĐỊA CHỈ. Chép nông thì chép luôn cả cái địa chỉ ấy — y nguyên, không đổi.

Kết quả: cuốn ngoài thì tách hẳn, có hai địa chỉ khác nhau thật. Còn cuốn con
nằm bên trong thì cả hai cuốn ngoài vẫn đang chỉ vào **đúng một** cuốn con —
y như hai tấm thẻ ở bài trước.
::::

::::example{#mot-to-hai-lop}
Một tô phở của quán chị Hạnh không chỉ có bánh với thịt. Nó còn có một danh
sách "món kèm" — hành, ngò — nằm lồng ngay bên trong tô, ở ô thứ ba.

```python title=readonly
to1 = ["bánh phở", "thịt bò", ["hành", "ngò"]]
to2 = to1[:]

to2.append("giá")
print(f"to1 = {to1}")
print(f"to2 = {to2}")
```

Máy in ra:

```text title=readonly
to1 = ['bánh phở', 'thịt bò', ['hành', 'ngò']]
to2 = ['bánh phở', 'thịt bò', ['hành', 'ngò'], 'giá']
```

Đúng như hẹn: thêm "giá" vào `to2` thì `to1` không hề hay biết. Lớp ngoài đã
tách thật. Nhưng nếm thử món kèm — sửa vào chính cuốn con, ô thứ ba:

```python title=readonly
to2[2].append("rau thơm")
print(f"to1 = {to1}")
print(f"to2 = {to2}")
```

```text title=readonly
to1 = ['bánh phở', 'thịt bò', ['hành', 'ngò', 'rau thơm']]
to2 = ['bánh phở', 'thịt bò', ['hành', 'ngò', 'rau thơm'], 'giá']
```

`to1` không hề được nhắc tên ở dòng `to2[2].append(...)`. Vậy mà "rau thơm"
lại xuất hiện trong cả `to1` lẫn `to2`. Lý do đúng như đoạn giải thích ở
trên: `to1[2]` và `to2[2]` là hai ô, nhưng cả hai ô ấy đang giữ **cùng một
địa chỉ** — `to1[2] is to2[2]` ra `True`. Chép nông tách được `to1` khỏi
`to2`, nhưng không tách nổi `to1[2]` khỏi `to2[2]`, vì bản thân phép chép
chưa từng đụng tới lớp đó — nó chỉ chép cái địa chỉ trỏ vào lớp đó, y nguyên.

Đây chính là chỗ vấp thật của chép nông, không phải một trường hợp hiếm gặp.
Danh sách trong danh sách là chuyện rất thường, và mỗi lần như vậy, `[:]`
chỉ hứa tách được đúng lớp ngoài cùng.
::::

::::predict{#doan-mon-kem commitOnce}
Quán có một danh sách món chính, ô cuối là một danh sách "món kèm" của món
đầu tiên. Byte cắt ra một bản riêng để in tờ rơi, rồi sửa cả hai lớp.

**Trước khi chạy**, bạn đoán hai dòng cuối in ra gì?

```python
mon_chinh = ["phở bò", "phở gà", ["nạm", "gầu"]]
mon_rieng = mon_chinh[:]

mon_rieng.append("phở tái")
mon_rieng[2].append("vè giòn")

print(f"mon_chinh: {mon_chinh}")
print(f"mon_rieng: {mon_rieng}")
```

:::opt{correct}
`mon_chinh: ['phở bò', 'phở gà', ['nạm', 'gầu', 'vè giòn']]` và
`mon_rieng: ['phở bò', 'phở gà', ['nạm', 'gầu', 'vè giòn'], 'phở tái']`
:::

:::opt
Cả hai dòng đều có thêm `'phở tái'` VÀ `'vè giòn'`, giống hệt nhau
::why
Gần đúng ở chỗ bạn nhớ đúng có một sự dùng chung nào đó giữa `mon_chinh` và
`mon_rieng` — bài này quả thật đang nói về chuyện dùng chung.

Chỗ lệch là `[:]` KHÔNG dùng chung mọi thứ. Nó dựng ra một cuốn ngoài mới
thật sự, nên `mon_rieng.append("phở tái")` chỉ đổi `mon_rieng`, không đổi
`mon_chinh`. Chỉ có lớp CON — ô thứ ba, `["nạm", "gầu"]` — mới là chỗ hai
cuốn còn dính nhau.
::
:::

:::opt
`mon_chinh` không đổi gì cả, vẫn `["phở bò", "phở gà", ["nạm", "gầu"]]`
::why
Gần đúng ở chỗ bạn nhớ đúng: `[:]` dựng ra một cuốn MỚI, đúng như Realm 1
đã dạy — và với lớp ngoài, điều đó hoàn toàn đúng.

Chỗ lệch là bạn cho rằng "cuốn mới" nghĩa là mọi thứ bên trong cũng mới
theo. Ô thứ ba của `mon_rieng` không phải một cuốn con MỚI — nó là cùng
cuốn con của `mon_chinh`, chỉ được chép địa chỉ sang. `mon_rieng[2].append(...)`
sửa đúng cuốn con ấy, nên `mon_chinh[2]` cũng phải thấy "vè giòn".
::
:::

:::opt
Ngược lại: `mon_chinh` có thêm `'phở tái'` nhưng không có `'vè giòn'`
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra hai lớp của bài này cư xử KHÁC nhau —
đó chính là điều bài học nói tới.

Chỗ lệch là bạn gán ngược vai hai lớp. Lớp NGOÀI (thêm hẳn một món "phở tái"
vào cuối danh sách) là lớp đã tách hẳn, nên `mon_chinh` không thấy nó. Lớp
TRONG (nêm thêm "vè giòn" vào món kèm có sẵn) mới là lớp còn dùng chung, nên
`mon_chinh` phải thấy đúng "vè giòn", không phải "phở tái".
::
:::
::::

::::explain{#chep-nong-khong-phai-loi}
Cần nói rõ một điều trước khi đi tiếp: chép nông không phải một cách chép
"làm ẩu". Nó làm đúng cái tên của nó hứa — chép đúng MỘT lớp, không hơn
không kém. Với một danh sách toàn số hay toàn chữ, một lớp là đủ dùng, và
bạn đã dùng nó suốt từ Realm 1 mà chưa từng gặp rắc rối nào.

Rắc rối chỉ lộ ra khi có lớp lồng nhau, và lúc đó cái cần nhớ không phải
"đừng dùng `[:]`" — mà là: **`[:]` tách được đúng lớp ngoài cùng, không hơn.**
Biết trước điều này thì bạn không còn bị bất ngờ nữa, đúng như bài trước đã
cho bạn đoán trước thay vì giật mình.
::::

::::code{#gio-rau-cua-bac-sau}
Bác Sáu ở vườn rau sau nhà là người đưa rau cho quán chị Hạnh mỗi sáng.
Sáng nay bác hái được một giỏ, trong đó có một túi hạt giống để dành lại,
nằm lồng ngay trong giỏ.

Byte muốn thử nêm thêm một loại rau mới vào **bản nháp**, mà giỏ thật của
bác Sáu không được đụng tới ở lớp ngoài. Hãy điền vế phải của dòng còn
thiếu, sao cho `ban_nhap` là chép NÔNG từ `gio_rau_goc`.

```python title=starter
gio_rau_goc = ["rau muống", "cải ngọt", ["hạt bí đỏ", "hạt mướp"]]

# Bản nháp để thử nêm thêm. Chép NÔNG — đúng cách Realm 1 dạy cho lát cắt
# trọn cuốn.
ban_nhap = ___

ban_nhap.append("hành")

print(f"Bản nháp:   {ban_nhap}")
print(f"Giỏ gốc:    {gio_rau_goc}")
```

```python title=solution
gio_rau_goc = ["rau muống", "cải ngọt", ["hạt bí đỏ", "hạt mướp"]]

# Bản nháp để thử nêm thêm. Chép NÔNG — đúng cách Realm 1 dạy cho lát cắt
# trọn cuốn.
ban_nhap = gio_rau_goc[:]

ban_nhap.append("hành")

print(f"Bản nháp:   {ban_nhap}")
print(f"Giỏ gốc:    {gio_rau_goc}")
```

```python title=test
assert ban_nhap == ["rau muống", "cải ngọt", ["hạt bí đỏ", "hạt mướp"], "hành"], "bản nháp phải mang đủ rau của giỏ gốc, cộng thêm 'hành' vừa thử nêm"
assert gio_rau_goc == ["rau muống", "cải ngọt", ["hạt bí đỏ", "hạt mướp"]], "giỏ gốc không được đổi ở LỚP NGOÀI — 'hành' chỉ được thêm vào bản nháp"
assert ban_nhap is not gio_rau_goc, "bản nháp phải là một giỏ RIÊNG (địa chỉ khác) — chép nông dựng cuốn ngoài mới, không buộc thêm thẻ vào giỏ cũ"
# Túi hạt giống (ô thứ ba) là một danh sách CON. Chép nông chỉ tách lớp
# ngoài — lớp trong vẫn là MỘT túi, dùng chung giữa bản nháp và giỏ gốc.
ban_nhap[2].append("hạt dưa leo")
assert gio_rau_goc[2] == ["hạt bí đỏ", "hạt mướp", "hạt dưa leo"], "túi hạt giống bên trong vẫn phải là MỘT túi dùng chung — thêm hạt vào túi của bản nháp thì giỏ gốc cũng phải thấy, đó chính là chỗ chép NÔNG dừng lại"
assert ban_nhap[2] is gio_rau_goc[2], "túi hạt giống (ô thứ ba) của bản nháp và của giỏ gốc phải là CÙNG một danh sách con"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở vế phải một dấu bằng, và giỏ cần chép đã có tên sẵn ở dòng trên. Bài trước đã chỉ ra rằng viết trần cái tên ấy vào đây chỉ cho bạn thêm một tấm thẻ, không cho bạn một giỏ mới.
- kind: strategy
  body: Thứ dựng ra một địa chỉ mới ở lớp ngoài là lát cắt trọn cuốn — cặp ngoặc vuông bỏ trống cả hai đầu, gắn ngay sau tên giỏ. Đây đúng là công cụ Realm 1 đã dạy cho việc này.
- kind: one-line
  body: 'Viết `gio_rau_goc[:]` vào chỗ trống, thành `ban_nhap = gio_rau_goc[:]`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Bản nháp:   \['rau muống', 'cải ngọt', \['hạt bí đỏ', 'hạt mướp'\], 'hành'\]\nGiỏ gốc:    \['rau muống', 'cải ngọt', \['hạt bí đỏ', 'hạt mướp'\]\]\s*$
- tier: output
  expect: "Giỏ gốc:    ['rau muống', 'cải ngọt', ['hạt bí đỏ', 'hạt mướp']]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lớp ngoài tách rồi. Túi hạt giống thì mình biết trước là còn dùng chung.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chuyện túi hạt giống dùng chung chỉ đáng lo vì túi ấy là một `list` — một
thứ **sửa được tại chỗ**. Nếu ai đó nêm thêm hạt vào túi, mọi tấm thẻ trỏ
tới túi ấy đều thấy ngay, đúng như bài này vừa cho bạn thấy.

Nhưng không phải giá trị nào cũng sửa được tại chỗ. Hồi Realm 1, bạn đã thử
sửa một ký tự của chuỗi và bị máy từ chối thẳng; thử sửa một ô của `tuple`
cũng bị từ chối y như vậy.

Nếu hai tấm thẻ cùng trỏ vào một thứ **không sửa được tại chỗ**, chuyện
dùng chung ấy còn đáng lo như với `list` không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
