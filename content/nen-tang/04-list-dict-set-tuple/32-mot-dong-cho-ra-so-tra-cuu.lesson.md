---
id: nen-tang.list-dict-set-tuple.mot-dong-cho-ra-so-tra-cuu
title: Một dòng cho ra sổ tra cứu
summary: Cùng cái khuôn ấy, đổi sang ngoặc nhọn và thêm dấu hai chấm thì thứ đi ra là một dict tra được bằng tên.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 32
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.dict-comprehension]
practices: [core.comprehension-filter, core.list-comprehension, core.dict-items, core.for-unpack, core.dict, core.len, core.fstring]
requires: [core.comprehension-filter, core.list-comprehension, core.list-of-dicts, core.dict, core.dict-assign, core.dict-items, core.for-unpack, core.nested-index, core.len, ctrl.for-each, core.fstring]
concepts: [core.danh-sach, core.bien]
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
Đổi cặp ngoặc thôi thì chưa đủ. Sổ tra cứu cần hai thứ mỗi dòng, không phải một.
::::

::::explain{#mot-cai-ten-thi-chua-tra-duoc}
Danh sách tên của bài trước trả lời được câu *có những khoản nào*. Nó không trả
lời được câu *sửa xe hết bao nhiêu* — muốn biết thì lại phải quay về cuốn sổ và
dò từng khoản.

Thứ trả lời được câu ấy là cái bạn đã có từ bài 9: một **sổ tra cứu**, gõ
`tra_cuu["sửa xe"]` là ra ngay con số.

Vậy dựng nó bằng cái khuôn một dòng được không? Được, nhưng đổi cặp ngoặc thôi
thì chưa đủ, và lý do rất đời thường: một danh sách chỉ cần **một** thứ cho mỗi
phần tử, còn một sổ tra cứu cần **hai** — cái để tra, và cái tra ra được.

Dấu hai chấm là chỗ nói rõ đâu là cái nào. Đúng dấu hai chấm bạn vẫn viết khi gõ
một dict bằng tay:

```text title=readonly
{  khoá  :  giá trị   for  từng cái  in  từ đâu  }
```

Phần từ chữ `for` trở đi không đổi một chữ nào so với bài trước. Chỗ duy nhất
khác là đầu dòng: thay vì một biểu thức, giờ có hai, ngăn nhau bằng dấu hai chấm.
::::

::::example{#ba-khoan-mot-so-tra-cuu}
Lấy ba khoản đầu cuốn sổ cho dễ nhìn. Bản dài trước:

```python title=readonly
so_ngan = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
]

tra_cuu = {}
for khoan in so_ngan:
    tra_cuu[khoan["ten"]] = khoan["tien"]

print(tra_cuu)
print(tra_cuu["xăng"])
```

Máy in ra:

```text title=readonly
{'cà phê': 90000, 'xăng': 240000, 'ăn trưa': 620000}
240000
```

Bản gọn, chạy trên đúng ba khoản ấy:

```python title=readonly
tra_cuu = {khoan["ten"]: khoan["tien"] for khoan in so_ngan}
print(tra_cuu)
print(tra_cuu["xăng"])
```

Vẫn hai dòng in ấy. Và vẫn đúng cách ghép mảnh của bài trước: dòng `for` giữ
nguyên, còn dòng gán `tra_cuu[khoan["ten"]] = khoan["tien"]` thì tách làm đôi.
Cái khoá — thứ nằm trong cặp ngoặc vuông bên trái dấu bằng — chuyển sang đứng
**trước** dấu hai chấm; cái giá trị bên phải dấu bằng chuyển sang đứng **sau**.
Bản thân cái tên `tra_cuu` không còn xuất hiện trong ngoặc nhọn nữa: sổ mới
chưa có tên cho tới lúc nó được gán ở đầu dòng.
::::

::::predict{#doan-so-khoa commitOnce}
Giờ chạy đúng dòng gọn ấy trên **cả** cuốn sổ tám khoản của bài trước — cuốn có
hai khoản cùng tên "xăng", một khoản ngày 5 hết 240 nghìn và một khoản ngày 22
hết 105 nghìn.

**Trước khi bấm chạy**, bạn đoán hai dòng in ra những con số nào?

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "học phí", "tien": 300000, "ngay": 15, "nhom": "học phí"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xăng xe"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xăng xe"},
]

tra_cuu = {khoan["ten"]: khoan["tien"] for khoan in so}
print(len(tra_cuu))
print(tra_cuu["xăng"])
```

:::opt{correct}
7 rồi 105000
:::

:::opt
8 rồi 240000
::why
Gần đúng ở chỗ bạn đọc dòng gọn theo đúng nhịp của nó: một lượt `for` cho một
cặp khoá–giá trị, tám khoản thì tám lượt. Cách đếm lượt của bạn chính xác.

Chỗ lệch là ở chuyện **hai lượt rơi trúng cùng một khoá**. Khoá ở đây là cái
tên, mà "xăng" thì chỉ có một cái tên duy nhất dù được ghi hai lần. Lượt thứ hai
gặp lại đúng khoá ấy, và nó làm đúng luật của bài 12: có khoá thì sửa, chưa có
mới thêm. Nên sổ tra cứu không dài thêm, còn con số cũ thì bị con số mới đè lên.

Số 240000 của ngày 5 biến mất khỏi sổ tra cứu, không một tiếng báo.
::
:::

:::opt
8 rồi 105000
::why
Gần đúng ở nửa khó: bạn nhận ra lượt sau đè lên lượt trước, nên tra "xăng" ra
105000 — đúng.

Chỗ lệch nằm ở con số đầu. `len` của một dict đếm số **khoá**, không đếm số lượt
gán. Khi lượt thứ hai chỉ sửa lại một khoá đã có, sổ không mọc thêm ô nào, nên
tám khoản chỉ để lại bảy khoá.

Đây chính là chỗ một dict khác một list: list nhận đủ tám phần tử, dict thì gộp
hai khoản cùng tên lại làm một.
::
:::

:::opt
7 rồi 240000
::why
Gần đúng ở con số đầu, và nó là con số khó hơn: bạn đếm đúng bảy khoá, tức là
bạn đã thấy hai khoản "xăng" chỉ chiếm một chỗ trong sổ tra cứu.

Chỗ lệch là ở chuyện lượt nào thắng. Ở đây có cảm giác rất tự nhiên rằng khoá đã
có rồi thì lượt sau bị bỏ qua — nhưng phép gán không hỏi cái khoá đang giữ gì,
nó chỉ ghi đè. Lượt ngày 5 ghi 240000, lượt ngày 22 ghi 105000 lên chỗ ấy, và
lượt ngày 22 là lượt cuối cùng chạm vào khoá "xăng".

Nên thứ còn lại trong sổ là con số của lần ghi **sau cùng**.
::
:::
::::

::::explain{#khi-trong-tay-da-la-cac-cap}
Cuốn sổ tra cứu vừa dựng có một chỗ đau đáng nhìn thẳng vào: nó nuốt mất khoản
đổ xăng 240 nghìn. Không lỗi, không cảnh báo, chỉ là con số ấy không còn ở đâu
nữa. Đó là cái giá của việc chọn tên làm khoá khi tên có thể trùng — nhớ lấy
chỗ này, bài sau sẽ gọi tên nó.

Còn bây giờ là chuyện nguồn. Ở ví dụ trên, thứ đi vào là **các dict**, nên phải
tự tay bóc ra hai trường. Nhưng nhiều khi thứ bạn đang cầm đã là các **cặp** rồi
— `.items()` của bài 15 cho ra đúng những cặp như vậy. Lúc đó phần `for` mở luôn
cặp ra thành hai cái tên, y như bài 18:

```python title=readonly
dang_ke = {ten: tien for ten, tien in tra_cuu.items() if tien > 100000}
```

Đọc thành lời: *lấy từng cặp trong sổ tra cứu, gọi ô đầu là `ten`, ô sau là
`tien`; khoản nào trên 100 nghìn thì đặt vào sổ mới với `ten` làm khoá và `tien`
làm giá trị.*

Ba mảnh trong một dòng, và cả ba đều là mảnh cũ:

- `ten: tien` — dấu hai chấm của hôm nay.
- `for ten, tien in tra_cuu.items()` — cặp và cách mở cặp, bài 15 và bài 18.
- `if tien > 100000` — cái đuôi lọc của bài trước, đứng đúng chỗ cũ.

Sổ mới là một sổ **khác**; `tra_cuu` không suy suyển, y như `sorted` ở bài 19
không đụng tới sổ gốc.
::::

::::code{#dung-hai-cuon-so-tra-cuu}
Byte cần hai cuốn sổ tra cứu từ cùng một cuốn sổ chi tiêu tám khoản:

- `tra_cuu` — tra bằng **tên** khoản, ra **số tiền** của khoản đó.
- `dang_ke` — cũng tra bằng tên, nhưng chỉ giữ những khoản **trên** 100 nghìn.

Ba chỗ trống: hai chỗ hai bên dấu hai chấm ở dòng thứ nhất, một chỗ sau chữ `if`
ở dòng thứ hai. Con số ngưỡng đã có tên sẵn là `nguong`, nên chỗ trống cuối chỉ
còn phải nói phần so sánh.

```python title=starter
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "học phí", "tien": 300000, "ngay": 15, "nhom": "học phí"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xăng xe"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xăng xe"},
]

nguong = 100000

tra_cuu = {___: ___ for khoan in so}

dang_ke = {ten: tien for ten, tien in tra_cuu.items() if ___}

print(f"Sổ tra cứu có {len(tra_cuu)} khoá")
print(f"Xăng: {tra_cuu['xăng']} đồng")
print(f"Đáng kể: {len(dang_ke)} khoản")
```

```python title=solution
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "học phí", "tien": 300000, "ngay": 15, "nhom": "học phí"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xăng xe"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xăng xe"},
]

nguong = 100000

tra_cuu = {khoan["ten"]: khoan["tien"] for khoan in so}

dang_ke = {ten: tien for ten, tien in tra_cuu.items() if tien > nguong}

print(f"Sổ tra cứu có {len(tra_cuu)} khoá")
print(f"Xăng: {tra_cuu['xăng']} đồng")
print(f"Đáng kể: {len(dang_ke)} khoản")
```

```python title=test
# Bốn phép kiểm cho ba chỗ trống. Phép đầu bắt số khoá — đảo hai vế quanh dấu
# hai chấm là số này lệch ngay. Hai phép giữa hỏi thẳng giá trị tra ra được.
# Phép cuối chỉ đúng khi cái đuôi lọc đặt đúng ngưỡng và đúng dấu so sánh.
assert len(tra_cuu) == 7, "tám khoản nhưng chỉ có bảy cái TÊN khác nhau, vì 'xăng' được ghi hai lần và hai lần ấy dùng chung một khoá"
assert tra_cuu["ăn trưa"] == 620000, "khoá phải là tên khoản và giá trị phải là số tiền — tra 'ăn trưa' ra 620000; nếu chỗ này báo KeyError thì hai vế quanh dấu hai chấm đang bị đảo chỗ"
assert tra_cuu["xăng"] == 105000, "khoản xăng ngày 22 hết 105 nghìn là lần ghi SAU, nên nó đè lên con số của lần ghi ngày 5; khoá 'xăng' giữ lại 105000"
assert dang_ke == {"xăng": 105000, "ăn trưa": 620000, "sửa xe": 500000, "học phí": 300000}, "trong bảy khoá của sổ tra cứu, đúng bốn khoá có giá trị trên 100 nghìn là xăng, ăn trưa, sửa xe và học phí — cà phê 90 nghìn, bánh mì 15 nghìn và vá lốp tròn 100 nghìn đều không lọt"
```

:::hints
- kind: attention
  body: Hai chỗ trống đầu nằm hai bên dấu hai chấm, và chúng nói về cùng một khoản mà `for` vừa lấy ra — cái khoản tên `khoan`. Chỗ trống thứ ba nằm ở cuối dòng dưới, nơi phần `for` đã kịp mở cặp ra thành hai cái tên `ten` và `tien`.
- kind: strategy
  body: Sổ tra cứu phải tra được bằng **tên** và cho ra **số tiền**, nên bên trái dấu hai chấm là trường `ten` của khoản, bên phải là trường `tien`. Ở dòng dưới thì không còn `khoan` nào nữa: cặp đã mở rồi, số tiền đang nằm sẵn trong cái tên `tien`, còn con số đem so đã có tên là `nguong`; câu hỏi vẫn là câu hỏi quen thuộc của bài trước — trên 100 nghìn thì giữ, tròn 100 nghìn thì không.
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `khoan["ten"]`, `khoan["tien"]` và `tien > nguong`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Sổ tra cứu có 7 khoá\nXăng: 105000 đồng\nĐáng kể: 4 khoản\s*$
- tier: output
  expect: Xăng: 105000 đồng
- tier: output
  expect: Đáng kể: 4 khoản
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một khuôn. Ngoặc vuông cho dãy, ngoặc nhọn kèm dấu hai chấm cho sổ tra cứu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đang cầm bốn chỗ chứa: list, dict, set, tuple. Cùng một cuốn sổ chi tiêu —
lúc nào dùng cái nào, và **chọn sai thì trả giá ở đâu**?

Câu sau không phải câu hỏi cho vui. Bài này vừa đưa ra một cái giá cụ thể: cùng
cuốn sổ tám khoản ấy, danh sách của bài trước giữ đủ cả hai lần đổ xăng, còn sổ
tra cứu của bài này chỉ giữ lần sau. Không ai báo cho bạn biết 240 nghìn đã đi
đâu — chính bạn chọn cái tên làm khoá, và cái giá đi kèm lựa chọn đó.

Bài sau đặt tên cho những cái giá ấy.
::::

::::checkpoint{mastery=0.8}
::::
