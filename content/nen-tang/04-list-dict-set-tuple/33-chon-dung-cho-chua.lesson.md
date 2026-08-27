---
id: nen-tang.list-dict-set-tuple.chon-dung-cho-chua
title: Chọn đúng chỗ chứa
summary: Bốn câu hỏi quyết định — thứ tự, cách tra, chuyện trùng, chuyện sửa — và cái giá cụ thể của mỗi lần chọn sai.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 33
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [core.choose-container, core.set-union]
practices: [core.set, core.set-intersection, core.set-difference, core.list-comprehension, core.dict-comprehension, core.list-of-dicts, core.tuple, core.len, core.fstring]
requires: [core.list, core.list-index, core.list-of-dicts, core.dict, core.dict, core.dict-comprehension, core.list-comprehension, core.comprehension-filter, core.set, core.set-unordered, core.set-intersection, core.set-difference, core.tuple, core.tuple-immutable, core.parallel-lists, core.len, core.fstring]
concepts: [core.danh-sach, core.chi-so]
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
Bốn chỗ chứa, và mỗi cái đã từng làm bạn đau đúng một lần. Đó là bảng chọn rồi.
::::

::::explain{#bon-cau-hoi}
Bài trước kết bằng một câu hỏi thật: cầm bốn chỗ chứa trong tay, lúc nào dùng
cái nào?

Không có cái nào tốt hơn cái nào. Nhưng mỗi cái **từ chối** một chuyện, và chỗ
nó từ chối chính là chỗ bạn đã bị đau — nên bạn nhớ được nó mà không phải học
thuộc gì cả. Bốn câu hỏi dưới đây chỉ là bốn vết đau ấy được đặt tên.

**Câu 1 — Thứ tự bỏ vào có phải là một phần của câu trả lời không?**

Sổ chi tiêu ghi theo ngày, nên thứ tự có nghĩa: `list` và `tuple` giữ nguyên
thứ tự. Còn câu hỏi "tháng này chi vào những nhóm nào" thì thứ tự chẳng nghĩa
gì.

*Giá phải trả nếu chọn sai:* bài 27, rổ không xếp hàng. In ra thì thứ tự khác
lúc bỏ vào, và `ro[0]` là `TypeError` — trong rổ không có ô nào mang số 0 cả.

**Câu 2 — Tra bằng chỗ đứng hay tra bằng tên?**

Tra bằng chỗ đứng thì dùng `list`, `tuple`. Tra bằng tên thì `dict`.

*Giá phải trả nếu chọn sai:* bài 8, hai dãy song song. Sợi dây nối tên với tiền
chỉ nằm trong đầu bạn; xoá bên này quên bên kia là máy in tên một đằng tiền một
nẻo, mà không kêu một tiếng.

**Câu 3 — Cùng một thứ bỏ vào hai lần thì để nguyên hai, hay gộp thành một?**

`list` để nguyên hai. `set` gộp thành một. `dict` gộp **khoá** thành một, và
giá trị của lần sau đè lên lần trước.

*Giá phải trả nếu chọn sai:* bài trước, ngay trước mắt bạn. Cùng cuốn sổ ấy,
danh sách tên giữ đủ cả hai lần đổ xăng, còn sổ tra cứu chỉ giữ lần sau — khoản
240 nghìn ngày 5 biến mất không một lời báo.

**Câu 4 — Dựng xong rồi còn phải sửa nữa không?**

`list` và `dict` sửa được. `tuple` thì không, và đó là lý do người ta chọn nó:
một cặp tên–tiền đã đóng cứng thì không lệch được như hai dãy song song.

*Giá phải trả nếu chọn sai:* bài 17, gán vào một ô của tuple là `TypeError`.
Chọn tuple cho thứ còn phải thêm bớt là tự khoá tay mình.
::::

::::example{#mot-cuon-so-bon-chiec-hop}
Cùng cuốn sổ tám khoản của hai bài trước, đổ vào bốn chỗ chứa khác nhau. Nhìn
riêng ba con số đầu là đủ thấy chúng không phải bốn cách viết của cùng một thứ.

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xe cộ"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xe cộ"},
    {"ten": "biếu bà", "tien": 300000, "ngay": 15, "nhom": "biếu tặng"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xe cộ"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xe cộ"},
]

ten_khoan = [khoan["ten"] for khoan in so]
tra_cuu = {khoan["ten"]: khoan["tien"] for khoan in so}
nhom = set([khoan["nhom"] for khoan in so])
khoan_ton_nhat = ("ăn trưa", 620000)

print(len(ten_khoan))
print(len(tra_cuu))
print(len(nhom))
print(khoan_ton_nhat[0], khoan_ton_nhat[1])
print("xe cộ" in nhom)
```

Máy in ra:

```text title=readonly
8
7
3
ăn trưa 620000
True
```

Cùng một cuốn sổ, ba con số khác nhau — và không con số nào sai cả:

- **8** — dãy tên giữ đủ tám khoản, kể cả hai khoản cùng tên "xăng". Nó trả lời
  được câu *khoản thứ tư là gì*, vì mỗi khoản còn chỗ đứng riêng.
- **7** — sổ tra cứu gộp hai khoản "xăng" vào một khoá. Nó trả lời được câu
  *xăng hết bao nhiêu* ngay lập tức, và trả bằng cách quên mất một lần đổ.
- **3** — rổ nhóm chỉ còn ba nhóm cho tám khoản. Nó không trả lời nổi câu nào về
  tiền, nhưng câu *tháng này có chi vào xe cộ không* thì nó đáp ngay, không dò.
- Cặp `("ăn trưa", 620000)` giữ tên dính chặt vào tiền. Đọc bằng `[0]` và `[1]`
  như list, chỉ khác một điều: không ai gán đè lên ô nào được.

Bảng chọn nhanh, đọc theo bốn câu hỏi ở trên:

| Chỗ chứa | Giữ thứ tự bỏ vào | Tra bằng gì | Bỏ vào hai lần | Sửa được sau khi dựng |
|---|---|---|---|---|
| `list` | Có | chỗ đứng — `so[0]`, `so[-1]` | Giữ nguyên cả hai | Có — `.append`, `.remove` |
| `dict` | Bạn không bao giờ hỏi "khoá thứ mấy" | tên khoá — `tra_cuu["xăng"]` | Khoá gộp làm một, giá trị sau đè lên trước | Có — `chi[khoa] = ...` |
| `set` | Không, bài 27 | không tra từng ô; chỉ hỏi `in` | Gộp làm một, bài 26 | Chưa học cách sửa — dựng bằng `set(...)` rồi dùng |
| `tuple` | Có | chỗ đứng — `cap[0]` | Giữ nguyên cả hai | Không, bài 17 |
::::

::::explain{#con-mot-ky-hieu-nua}
Riêng cái rổ còn một chuyện chưa nói hết.

Bài 28 cho bạn dấu `&`: lấy những gì **cả hai** rổ cùng có. Bài 29 cho dấu `-`:
lấy những gì bên này có mà bên kia không.

Hai phép ấy trả lời hai câu trong ba câu người ta hay hỏi khi cầm hai cái rổ.
Câu thứ ba là *gộp cả hai lại thì có những gì* — và Python có sẵn một ký hiệu
cho nó, ký hiệu `|`, gõ bằng phím dấu gạch đứng.

Bài này không giảng trước nó làm gì. Bạn đã đủ để đoán, và đoán rồi mới xem thì
nhớ lâu hơn.
::::

::::predict{#doan-ba-con-so commitOnce}
Hai cái rổ nhóm chi: tháng này ba nhóm, tháng trước bốn nhóm. Hai nhóm có mặt ở
cả hai rổ.

**Trước khi bấm chạy**, bạn đoán ba dòng in ra ba con số nào?

```python title=readonly
thang_nay = {"ăn uống", "xe cộ", "biếu tặng"}
thang_truoc = {"ăn uống", "xe cộ", "điện nước", "học phí"}

print(len(thang_nay & thang_truoc))
print(len(thang_nay - thang_truoc))
print(len(thang_nay | thang_truoc))
```

:::opt{correct}
2 rồi 1 rồi 5
:::

:::opt
2 rồi 1 rồi 7
::why
Gần đúng ở hai dòng đầu, và chúng là hai dòng dùng đúng thứ đã học: `&` cho hai
nhóm chung, `-` cho một nhóm riêng của tháng này. Bạn đọc cả hai không sai.

Chỗ lệch ở dòng thứ ba là phép cộng cỡ hai rổ: `3 + 4 = 7`. Phép ấy chỉ đúng khi
hai rổ không có gì chung — mà ở đây "ăn uống" và "xe cộ" nằm trong cả hai.

Kết quả của một phép trên rổ vẫn là một cái **rổ**, và rổ thì không chứa hai
lần, đúng luật bài 26. Hai nhóm chung đi vào một lần thôi, nên bảy tụt xuống
còn năm.
::
:::

:::opt
2 rồi 3 rồi 5
::why
Gần đúng ở dòng đầu và dòng cuối, kể cả dòng cuối là dòng chưa ai dạy bạn — bạn
đoán ra `|` gộp hai rổ và đếm đúng năm nhóm.

Chỗ lệch ở dòng giữa. Con số 3 là số nhóm **khác nhau ở hai phía**: "biếu tặng"
của tháng này, cộng "điện nước" và "học phí" của tháng trước. Nhưng dấu `-` của
bài 29 không hỏi câu đó — nó chỉ lấy từ rổ đứng **trước** dấu trừ.

`thang_nay - thang_truoc` vì vậy chỉ nhặt trong ba nhóm của tháng này, và chỉ
"biếu tặng" là nhóm tháng trước không có.
::
:::

:::opt
2 rồi 1 rồi 4
::why
Gần đúng ở hai dòng đầu — bạn tách phần chung và phần riêng chính xác.

Chỗ lệch ở dòng thứ ba là chỗ đặt kết quả. Con số 4 là cỡ của rổ tháng trước,
nghe như "đổ tháng này vào tháng trước thì được rổ tháng trước". Nhưng "biếu
tặng" là nhóm mà tháng trước chưa từng có, nên nó phải làm rổ dài thêm một chỗ.

Bốn nhóm cũ cộng thêm "biếu tặng" là năm. Phép gộp lấy hết những gì **một
trong hai** rổ có, không bỏ sót bên nào.
::
:::
::::

::::explain{#dau-gach-doc-la-phep-hop}
Đúng như bạn đoán: `|` là phép **hợp** — lấy những gì có ở rổ này **hoặc** rổ
kia, mỗi thứ đúng một lần.

Ba phép đứng cạnh nhau thì đọc thành ba câu tiếng Việt rất gọn:

- `a & b` — **cả hai** cùng có.
- `a | b` — **một trong hai** có.
- `a - b` — `a` có mà `b` không.

Hai phép đầu không phân biệt bên nào đứng trước; phép thứ ba thì có, và bài 29
đã bắt bạn trả giá cho chỗ đó rồi.

Cả ba đều **trả về một rổ mới**, không đụng tới hai rổ nguồn — cùng cái nết của
`sorted` ở bài 19 và của lát cắt ở bài 5. Chạy xong `thang_nay | thang_truoc`
thì `thang_nay` vẫn đủ ba nhóm như cũ.

Và đây là chỗ bốn câu hỏi ở đầu bài trở nên dùng được: bạn chọn `set` cho nhóm
chi **vì** câu hỏi là "có hay không", chứ không phải vì set nghe hay. Chọn đúng
rồi thì ba ký hiệu này tự nhiên là thứ bạn cần; chọn thành list thì cả ba đều
không dùng được, và bạn quay về viết vòng lặp bằng tay như bài 28.
::::

::::code{#chon-cho-chua-cho-bon-cau-hoi}
Byte cần bốn câu trả lời từ cuốn sổ tám khoản, và tất cả đều nói về **nhóm** chi
chứ không nói về tiền.

Bốn chỗ trống: một chỗ chọn **chỗ chứa** cho nhóm chi của tháng này, ba chỗ còn
lại chọn **ký hiệu** cho ba câu hỏi trên hai cái rổ. Ba ký hiệu ấy khác nhau
từng cái một.

```python title=starter
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xe cộ"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xe cộ"},
    {"ten": "biếu bà", "tien": 300000, "ngay": 15, "nhom": "biếu tặng"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xe cộ"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xe cộ"},
]

# Tháng trước Byte đã tổng kết xong, chỉ còn giữ lại cái rổ nhóm.
thang_truoc = {"ăn uống", "xe cộ", "điện nước", "học phí"}

# Nhóm chi tháng này: chỉ cần biết CÓ những nhóm nào. Không cần thứ tự, và
# "ăn uống" nằm trong sổ ba lần thì cũng chỉ tính một.
thang_nay = ___([khoan["nhom"] for khoan in so])

thang_nao_cung_chi = thang_nay ___ thang_truoc
moi_nhom_tung_chi = thang_nay ___ thang_truoc
thang_truoc_chua_co = thang_nay ___ thang_truoc

print(f"Tháng này chi vào {len(thang_nay)} nhóm")
print(f"Tháng nào cũng chi: {len(thang_nao_cung_chi)} nhóm")
print(f"Mọi nhóm từng chi: {len(moi_nhom_tung_chi)} nhóm")
print(f"Tháng trước chưa có: {len(thang_truoc_chua_co)} nhóm")
```

```python title=solution
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xe cộ"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xe cộ"},
    {"ten": "biếu bà", "tien": 300000, "ngay": 15, "nhom": "biếu tặng"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 17, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 100000, "ngay": 19, "nhom": "xe cộ"},
    {"ten": "xăng", "tien": 105000, "ngay": 22, "nhom": "xe cộ"},
]

# Tháng trước Byte đã tổng kết xong, chỉ còn giữ lại cái rổ nhóm.
thang_truoc = {"ăn uống", "xe cộ", "điện nước", "học phí"}

# Nhóm chi tháng này: chỉ cần biết CÓ những nhóm nào. Không cần thứ tự, và
# "ăn uống" nằm trong sổ ba lần thì cũng chỉ tính một.
thang_nay = set([khoan["nhom"] for khoan in so])

thang_nao_cung_chi = thang_nay & thang_truoc
moi_nhom_tung_chi = thang_nay | thang_truoc
thang_truoc_chua_co = thang_nay - thang_truoc

print(f"Tháng này chi vào {len(thang_nay)} nhóm")
print(f"Tháng nào cũng chi: {len(thang_nao_cung_chi)} nhóm")
print(f"Mọi nhóm từng chi: {len(moi_nhom_tung_chi)} nhóm")
print(f"Tháng trước chưa có: {len(thang_truoc_chua_co)} nhóm")
```

```python title=test
# Bốn phép kiểm cho bốn chỗ trống, mỗi phép nhìn vào đúng một chỗ. Kiểm bằng
# phép so BẰNG cả rổ chứ không kiểm mỗi số lượng: ba ký hiệu trên hai cái rổ
# này có thể tình cờ cho ra cùng một con số đếm, nhưng không thể tình cờ cho
# ra cùng những nhóm.
assert thang_nay == {"ăn uống", "xe cộ", "biếu tặng"}, "tám khoản của cuốn sổ này chỉ rơi vào ba nhóm: ăn uống, xe cộ và biếu tặng — 'ăn uống' được ghi ba lần và 'xe cộ' bốn lần, mà chỗ chứa ở dòng này phải gộp mỗi nhóm về đúng một lần"
assert thang_nao_cung_chi == {"ăn uống", "xe cộ"}, "hai tháng cùng chi vào ăn uống và xe cộ; 'biếu tặng' chỉ tháng này có, còn 'điện nước' với 'học phí' chỉ tháng trước có"
assert moi_nhom_tung_chi == {"ăn uống", "xe cộ", "biếu tặng", "điện nước", "học phí"}, "gộp hai rổ lại được năm nhóm — ba nhóm của tháng này cộng thêm điện nước và học phí của tháng trước, còn ăn uống với xe cộ có ở cả hai rổ nhưng chỉ vào một lần"
assert thang_truoc_chua_co == {"biếu tặng"}, "chỉ 'biếu tặng' là nhóm tháng này có mà tháng trước chưa có; nếu bạn nhận về điện nước hay học phí thì hai cái rổ đang đứng nhầm chỗ ở hai bên ký hiệu"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất đứng ngay trước dấu ngoặc đơn, nên nó là tên của một chỗ chứa được gọi như gọi một hàm — đọc lại hai dòng ghi chú ngay trên nó, chúng nói thẳng chỗ chứa ấy phải có tính chất gì. Ba chỗ còn lại đều nằm giữa hai cái rổ, nên mỗi chỗ chỉ nhận đúng một ký hiệu.
- kind: strategy
  body: Dòng đầu cần thứ gộp mỗi nhóm về một lần và không quan tâm thứ tự — bài 26 có đúng một chỗ chứa như vậy, và nó bọc ngoài một danh sách. Ba dòng sau đọc thẳng từ tên biến: "tháng nào cũng chi" là phần cả hai rổ cùng có, "mọi nhóm từng chi" là phần một trong hai rổ có, "tháng trước chưa có" là phần rổ tháng này có mà rổ tháng trước không. Ba câu ấy ứng với ba ký hiệu khác nhau, và ở câu thứ ba thì thứ tự hai cái rổ có ý nghĩa.
- kind: one-line
  body: Bốn chỗ trống lần lượt là `set`, `&`, `|` và `-`.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Tháng này chi vào 3 nhóm
- tier: output
  expect: Mọi nhóm từng chi: 5 nhóm
- tier: output
  expect: Tháng trước chưa có: 1 nhóm
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn chỗ chứa, bốn câu hỏi. Chọn xong thì cú pháp tự đến.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang bài cuối của mạch.

Chọn được rồi. Ghép cả bốn thứ thành một cuốn sổ chi tiêu tự tổng kết tháng —
làm nổi không?

Nghĩ thử xem một cuốn sổ như thế phải trả lời những gì: cả tháng hết bao nhiêu,
chi vào mấy nhóm, mỗi nhóm hết bao nhiêu, khoản nào tốn nhất, khoản nào trên
100 nghìn. Năm câu, và không câu nào cần thêm một công cụ mới — mỗi câu chỉ cần
bạn chọn đúng một trong bốn chỗ chứa vừa xếp thành bảng.

Chỗ khó không còn nằm ở chỗ nhớ cú pháp nữa. Nó nằm ở chỗ: năm câu ấy chồng lên
nhau trên **một** cuốn sổ, và mỗi lựa chọn sai sẽ lặng lẽ cho ra một con số
trông rất giống câu trả lời đúng.
::::

::::checkpoint{mastery=0.85}
::::
