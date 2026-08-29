---
id: nen-tang.list-dict-set-tuple.boss-so-chi-tieu-thang
title: BOSS — Sổ chi tiêu tháng này
summary: Bốn chỗ chứa, một cuốn sổ, năm câu trả lời — cuốn sổ tự tổng kết tháng của Byte.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 34
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [core.list-of-dicts, core.dict-accumulator, core.sorted, core.sorted-reverse, core.comprehension-filter, core.tuple, core.dict-get-default, core.dict-items, core.set-unique, core.sorted-key, core.list-slice, core.list-comprehension, core.nested-index, core.tuple-unpack, core.pure-function, core.docstring, core.fstring]
requires: [core.list-of-dicts, core.dict-accumulator, core.sorted, core.sorted-reverse, core.comprehension-filter, core.tuple, core.list, core.list-index, core.len, core.accumulator, core.tuple-unpack, core.function-def, core.function-return, core.function-parameter, core.docstring, core.pure-function, core.one-job-name, core.fstring, core.reassign, ctrl.for-each, ctrl.if]
concepts: [core.danh-sach, core.chi-so, core.ham]
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
Hôm nay không có chỗ chứa nào mới. Chỉ có bốn cái cũ và một cuốn sổ phải tự
tổng kết lấy.
::::

::::explain{#chon-xong-roi-thi-sao}
Bài trước kết bằng một câu nghe như lời thách: chọn được chỗ chứa rồi, nhưng
ghép cả bốn thứ thành một cuốn sổ tự tổng kết tháng thì có làm nổi không?

Chọn và ghép là hai việc khác nhau. Bốn câu hỏi quyết định của bài trước trả lời
được *nên dùng cái nào*; chúng không tự viết ra chương trình. Bài này là chỗ trả
lời phần còn lại, và nó không mang theo công cụ nào mới — mọi thứ dùng ở đây đều
đã có tên và đã có một bài của riêng nó.

Byte đưa bạn cuốn sổ chi tiêu tháng này. Mỗi khoản chi là **một dict ba trường**:

```python title=readonly
{"ten": "cà phê", "nhom": "ăn uống", "tien": 25000}
```

Cả cuốn sổ là một **list các dict** như thế — đúng hình dạng bạn dựng ở bài 23.
Cuối tháng Byte muốn năm câu trả lời:

1. Cả tháng tiêu hết bao nhiêu?
2. Tháng này chi vào **mấy nhóm** khác nhau?
3. Mỗi nhóm hết bao nhiêu tiền?
4. **Ba khoản** tốn nhất là những khoản nào?
5. Những khoản nào tốn trên 100 nghìn?

Năm câu, và mỗi câu gọi đúng một chỗ chứa mà bạn vừa học cách chọn:

- Câu 1 — cần **mọi** số tiền gom lại một dãy để cộng: một list, dựng bằng
  comprehension.
- Câu 2 — chỉ hỏi *có những nhóm nào*, không quan tâm thứ tự, không đếm hai lần
  cùng một nhóm: một **set**.
- Câu 3 — mỗi nhóm phải dính chặt vào tổng tiền của nó và được tra bằng tên
  nhóm: một **dict**, dùng làm bộ cộng dồn.
- Câu 4 — cần sổ được xếp lại theo tiền, từ lớn xuống nhỏ, rồi cắt lấy khúc đầu:
  `sorted` với `key=`, `reverse=True`, và một lát cắt.
- Câu 5 — cần một list các tên, lọc theo điều kiện: comprehension có `if`.

Câu 4 cần một thứ đáng nhắc lại: `key=` nhận **một cái hàm**. Sổ này là list các
dict, nên hàm ấy phải nhận vào một khoản và đưa ra số tiền của khoản đó — đúng
loại hàm thuần khiết bạn viết suốt T1.3.
::::

::::example{#ca-cuon-so-nam-khoan}
Bắt đầu bằng một cuốn sổ ngắn: năm khoản đầu tháng, đủ ngắn để bạn dò lại bằng
mắt.

```python title=readonly
def lay_tien(khoan):
    """Đưa ra số tiền của một khoản, để sorted biết phải nhìn vào ô nào."""
    return khoan["tien"]


so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "sửa xe", "nhom": "đi lại", "tien": 500000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "đổ xăng", "nhom": "đi lại", "tien": 100000},
    {"ten": "vở ghi", "nhom": "học hành", "tien": 15000},
]

tong_thang = sum([khoan["tien"] for khoan in so])
so_nhom = len(set([khoan["nhom"] for khoan in so]))

tong_nhom = {}
for khoan in so:
    nhom = khoan["nhom"]
    tong_nhom[nhom] = tong_nhom.get(nhom, 0) + khoan["tien"]

khoan_theo_tien = sorted(so, key=lay_tien, reverse=True)
ba_khoan_ton_nhat = [khoan["ten"] for khoan in khoan_theo_tien[:3]]
khoan_lon = [khoan["ten"] for khoan in so if khoan["tien"] > 100000]

print("SỔ CHI TIÊU — năm khoản đầu tháng")
print(f"Tổng chi: {tong_thang} đồng")
print(f"Đã chi vào {so_nhom} nhóm")
for nhom, tien in tong_nhom.items():
    print(f"  {nhom}: {tien} đồng")
print(f"Ba khoản tốn nhất: {ba_khoan_ton_nhat}")
print(f"Khoản trên 100000 đồng: {khoan_lon}")
```

Máy in ra:

```text title=readonly
SỔ CHI TIÊU — năm khoản đầu tháng
Tổng chi: 680000 đồng
Đã chi vào 3 nhóm
  ăn uống: 65000 đồng
  đi lại: 600000 đồng
  học hành: 15000 đồng
Ba khoản tốn nhất: ['sửa xe', 'đổ xăng', 'bún bò']
Khoản trên 100000 đồng: ['sửa xe']
```

Bốn chỗ đáng dừng lại nhìn kỹ:

- **`sum([khoan["tien"] for khoan in so])`** đọc từ trong ra: comprehension dựng
  một list toàn số tiền, `sum` cộng cái list ấy. Cuốn sổ gốc không bị đụng tới.
- **`len(set([...]))`** cũng đọc từ trong ra: comprehension lấy ra năm tên nhóm
  (có trùng), `set` bỏ trùng còn ba, `len` đếm. Rổ không xếp hàng, nhưng câu hỏi
  ở đây chỉ là *mấy nhóm*, và con số ấy không phụ thuộc thứ tự.
- **`tong_nhom.get(nhom, 0)`** là chỗ khoá tự mọc. Lần đầu gặp `"ăn uống"`, khoá
  ấy chưa có trong sổ tra cứu, `.get` đưa về `0`, và `0 + 25000` thành mục mới.
  Lần sau gặp lại, `.get` đưa về `25000` thật.
- **`khoan_theo_tien[:3]`** cắt ba phần tử đầu của **danh sách đã xếp**, không
  phải của cuốn sổ gốc. `sorted` đưa ra một danh sách mới, nên `so` vẫn còn
  nguyên thứ tự ghi — và câu 5 phía dưới vẫn duyệt `so` theo đúng thứ tự ấy.

Ba khoản tốn nhất của cuốn sổ này là `['sửa xe', 'đổ xăng', 'bún bò']`, còn khoản
trên 100 nghìn thì chỉ có một. Hai câu trả lời khác nhau vì chúng hỏi hai chuyện
khác nhau: một câu hỏi *ba đứng đầu*, một câu hỏi *vượt một ngưỡng*. Khoản đổ
xăng nói rõ điều đó: nó tốn đúng 100000 đồng, đủ để lọt vào ba khoản đứng đầu,
mà vẫn không lọt qua `> 100000` — vì đúng ngưỡng thì chưa phải vượt ngưỡng, đúng
như bài 31 đã dặn.

Một chuyện nhỏ về cái tên `lay_tien`. Ở bài 22 bạn cũng viết một hàm tên như vậy,
nhưng nó nhận vào một **cặp** và lấy ô `cap[1]`, vì lúc ấy thứ đem xếp là danh
sách cặp lấy từ `.items()`. Ở đây thứ đem xếp là list các dict, nên hàm cùng tên
ấy phải lấy ô `khoan["tien"]`. Cùng một việc — *chỉ ra chỗ cho `sorted` nhìn
vào* — nhưng hình dạng thứ đưa vào đã đổi, nên thân hàm đổi theo.
::::

::::predict{#doan-thieu-get commitOnce}
Trong lúc chép lại đoạn cộng dồn, Byte viết gọn dòng giữa: bỏ hẳn `.get`, gán
thẳng số tiền của khoản vào khoá.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra gì?

```python title=readonly
so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "đổ xăng", "nhom": "đi lại", "tien": 100000},
]

tong_nhom = {}
for khoan in so:
    tong_nhom[khoan["nhom"]] = khoan["tien"]

print(tong_nhom)
```

:::opt{correct}
`{'ăn uống': 40000, 'đi lại': 100000}`
:::

:::opt
`{'ăn uống': 65000, 'đi lại': 100000}`
::why
Gần đúng ở chỗ bạn đọc đúng **ý định** của đoạn code: nhóm ăn uống có hai khoản,
và tổng của chúng là 25000 + 40000 = 65000. Con số ấy là con số Byte muốn, và
bạn tính không sai một đồng.

Chỗ lệch nằm ở dấu `=`. Nó gán, nó không cộng. Ở bài 12 bạn đã thấy đúng chuyện
này: `chi[khoa] = gia_tri` có khoá thì **sửa**, chưa có thì thêm mới — nó không
bao giờ hỏi khoá ấy đang giữ gì. Muốn cộng thì phải tự lấy giá trị cũ ra rồi
cộng vào, và `.get(khoa, 0)` chính là cái lấy ra ấy, kèm sẵn con số `0` cho lần
đầu tiên chưa có gì để lấy.
::
:::

:::opt
`{'ăn uống': 25000, 'đi lại': 100000}`
::why
Gần đúng ở chỗ bạn nhớ một tính chất có thật và nhớ đúng: **set** không nhận
cùng một giá trị hai lần. Bỏ vào lần thứ hai thì rổ không đổi.

Chỗ lệch là dict không cư xử như set. Khoá thì đúng là không trùng — sổ tra cứu
này chỉ có một mục `"ăn uống"` — nhưng cái không trùng là **khoá**, còn **giá
trị** của khoá ấy thì lần gán sau đè lên lần gán trước. Lượt hai gặp `"ăn uống"`
với số tiền 40000, và nó viết đè 40000 lên chỗ đang giữ 25000.
::
:::

:::opt
`{'cà phê': 25000, 'bún bò': 40000, 'đổ xăng': 100000}`
::why
Gần đúng ở chỗ bạn đọc ra đúng hình dạng của kết quả: một sổ tra cứu tên → tiền,
và cả ba khoản đều có mặt. Nếu dòng trong vòng lặp viết
`tong_nhom[khoan["ten"]] = khoan["tien"]` thì đây chính là đáp án.

Chỗ lệch là ô nào được lấy ra làm khoá. Dòng ấy viết `khoan["nhom"]` ở bên trái
dấu bằng, nên khoá là **tên nhóm** chứ không phải tên khoản. Ba khoản chỉ thuộc
hai nhóm, nên sổ tra cứu chỉ mọc ra hai khoá.
::
:::
::::

::::explain{#cong-don-can-hai-manh}
Đoán xong rồi thì phát biểu lại cho gọn: một bộ cộng dồn bằng dict cần **hai**
mảnh, và thiếu mảnh nào cũng hỏng theo một kiểu riêng.

- Mảnh **lấy ra**: `tong_nhom.get(nhom, 0)`. Nó trả lời câu "nhóm này đã cộng
  được bao nhiêu rồi", và trả lời được cả khi nhóm ấy chưa từng xuất hiện — đó
  là việc của con số `0` đứng thứ hai. Không có mảnh này thì lượt đầu tiên của
  mỗi nhóm sẽ hỏi một khoá chưa có và chương trình dừng vì `KeyError`.
- Mảnh **ghi lại**: `tong_nhom[nhom] = ...`. Nó cất kết quả vào đúng khoá ấy.
  Không có mảnh này thì phép cộng vẫn chạy nhưng không ai giữ lại, y hệt chuyện
  gọi một phương thức chuỗi mà quên gán.

Ghép hai mảnh:

```python title=readonly
tong_nhom[nhom] = tong_nhom.get(nhom, 0) + khoan["tien"]
```

Đọc câu đó thành tiếng Việt: *lấy con số nhóm này đang có — chưa có thì coi như
không — cộng thêm tiền của khoản này, rồi cất lại vào đúng nhóm ấy.*

Và đây là chỗ dễ tự lừa mình nhất trong cả bài: nếu cuốn sổ bạn thử **mỗi nhóm
chỉ có đúng một khoản**, thì bản viết thiếu `.get` cho ra kết quả **giống hệt**
bản đúng. Không có gì cộng dồn cả, nhưng cũng không có gì để lộ ra. Một cuốn sổ
như vậy không nói thật được, nên bước sau đưa bạn hai cuốn.
::::

::::code{#cong-don-tren-hai-cuon-so}
Hai cuốn sổ, cùng một câu hỏi: **mỗi nhóm hết bao nhiêu tiền**.

Sổ A có ba khoản thuộc ba nhóm khác nhau — mỗi nhóm đúng một khoản. Sổ B có bốn
khoản, trong đó ba khoản cùng thuộc nhóm ăn uống. Vì vậy một lời giải chỉ ghi đè
mà không cộng dồn vẫn cho ra đúng kết quả ở sổ A, và lộ ra ngay ở sổ B.

Hai chỗ trống nằm ở cùng một vị trí trong hai vòng lặp giống nhau, chỉ khác tên
cuốn sổ tra cứu.

```python title=starter
so_a = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "sửa xe", "nhom": "đi lại", "tien": 500000},
    {"ten": "vở ghi", "nhom": "học hành", "tien": 15000},
]

tong_a = {}
for khoan in so_a:
    nhom = khoan["nhom"]
    tong_a[nhom] = tong_a.get(nhom, 0) + khoan["tien"]

print("Sổ A")
for nhom, tien in tong_a.items():
    print(f"  {nhom}: {tien} đồng")

so_b = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "gửi xe", "nhom": "đi lại", "tien": 10000},
    {"ten": "cơm trưa", "nhom": "ăn uống", "tien": 35000},
]

tong_b = {}
for khoan in so_b:
    nhom = khoan["nhom"]
    tong_b[nhom] = ___

print("Sổ B")
for nhom, tien in tong_b.items():
    print(f"  {nhom}: {tien} đồng")
```

```python title=solution
so_a = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "sửa xe", "nhom": "đi lại", "tien": 500000},
    {"ten": "vở ghi", "nhom": "học hành", "tien": 15000},
]

tong_a = {}
for khoan in so_a:
    nhom = khoan["nhom"]
    tong_a[nhom] = tong_a.get(nhom, 0) + khoan["tien"]

print("Sổ A")
for nhom, tien in tong_a.items():
    print(f"  {nhom}: {tien} đồng")

so_b = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "gửi xe", "nhom": "đi lại", "tien": 10000},
    {"ten": "cơm trưa", "nhom": "ăn uống", "tien": 35000},
]

tong_b = {}
for khoan in so_b:
    nhom = khoan["nhom"]
    tong_b[nhom] = tong_b.get(nhom, 0) + khoan["tien"]

print("Sổ B")
for nhom, tien in tong_b.items():
    print(f"  {nhom}: {tien} đồng")
```

```python title=test
# Sổ A KHÔNG có chỗ trống, và đó là chủ ý.
#
# Sổ A mỗi nhóm đúng một khoản, nên bản viết thiếu `.get` —
# `tong_a[nhom] = khoan["tien"]` — cho ra kết quả giống hệt bản đúng. Đặt một
# chỗ trống chấm điểm lên đó là chấm mà không nhìn: mọi cách viết đều qua.
#
# Nên dòng của sổ A viết sẵn, làm bản mẫu, và ba assert dưới đây chỉ để bảo
# đảm người học không sửa hỏng nó. Chỗ trống duy nhất nằm ở sổ B — nơi nhóm
# `ăn uống` có ba khoản, và chỉ ở đó thì ghi đè mới lộ ra.
assert tong_a["đi lại"] == 500000, "sổ A có đúng một khoản đi lại là sửa xe 500 nghìn, nên nhóm đi lại của sổ A phải giữ 500000"
assert tong_a["ăn uống"] == 25000, "sổ A có đúng một khoản ăn uống là cà phê 25 nghìn, nên nhóm ăn uống của sổ A phải giữ 25000"
assert len(tong_a) == 3, "ba khoản của sổ A thuộc ba nhóm khác nhau, nên sổ tra cứu của sổ A phải có đúng ba khoá"
# Chỗ trống thứ hai bị soi bởi dòng `tong_b["ăn uống"]`: chỉ lời giải có cộng
# dồn mới ra 100000. Bản ghi đè cho ra 35000 — tiền của khoản cuối cùng.
assert tong_b["ăn uống"] == 100000, "sổ B có ba khoản ăn uống là 25000, 40000 và 35000, cộng lại thành 100000"
assert tong_b["đi lại"] == 10000, "sổ B có đúng một khoản đi lại là gửi xe 10 nghìn, nên nhóm đi lại của sổ B phải giữ 10000"
assert len(tong_b) == 2, "bốn khoản của sổ B chỉ thuộc hai nhóm, nên sổ tra cứu của sổ B phải có đúng hai khoá"
```

:::hints
- kind: attention
  body: Dòng cộng dồn của sổ A đã viết sẵn — chép nó xuống thì xong. Nhưng đọc kỹ đã: sổ A mỗi nhóm đúng một khoản, nên ở đó viết thiếu hay viết đủ đều ra cùng một kết quả. Sổ B mới là chỗ phân biệt được, vì nhóm `ăn uống` của nó có tới ba khoản.
- kind: strategy
  body: Lượt đầu tiên gặp một nhóm, khoá của nhóm ấy chưa tồn tại — nên phép lấy giá trị cũ phải là phép lấy có kèm mặc định, và mặc định đúng cho một bộ cộng dồn là số không. Lấy được giá trị cũ rồi thì cộng thêm ô `"tien"` của khoản trong lượt này.
- kind: one-line
  body: 'Viết `tong_b.get(nhom, 0) + khoan["tien"]` vào chỗ trống — đúng dòng của sổ A ở trên, chỉ đổi tên cuốn sổ tra cứu.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Sổ A\n  ăn uống: 25000 đồng\n  đi lại: 500000 đồng\n  học hành: 15000 đồng\nSổ B\n  ăn uống: 100000 đồng\n  đi lại: 10000 đồng\s*$
- tier: output
  expect: "ăn uống: 100000 đồng"
:::
::::

::::explain{#doi-so-truoc-khi-ghep}
Một chuyện phải nói thẳng ra trước khi bạn ghép cả cuốn sổ: **số liệu đổi**.

Cuốn sổ ở bước ghép **không phải** cuốn năm khoản của phần ví dụ. Nó là sổ cả
tháng: **mười khoản**, có thêm sách, cơm trưa, gửi xe, khoá tiếng Anh, trà sữa.
Vì thế mọi con số ở đó đều khác: tổng khác, tổng từng nhóm khác, và danh sách ba
khoản tốn nhất không còn là danh sách cũ — chỉ mỗi khoản sửa xe giữ được chỗ
đứng đầu. Chép đáp án của phần ví dụ sang là trật, và bộ test sẽ nói đúng chỗ
trật.

Ba chỗ trống trong cuốn sổ tháng nằm ở ba câu hỏi khác nhau, và cả ba đều là thứ
bạn đã viết ít nhất một lần rồi:

- một comprehension gom mọi số tiền lại để `sum` cộng;
- cái hàm mà `sorted` sẽ gọi trên từng khoản để biết nhìn vào ô nào;
- điều kiện lọc đặt ở cuối một comprehension.

Riêng chỗ thứ hai đáng nhắc lại một lần nữa, vì nó là chỗ dễ viết thừa nhất:
`key=` nhận **cái hàm**, không nhận **kết quả gọi hàm**. Viết tên hàm trần, không
có cặp ngoặc theo sau — cặp ngoặc là lệnh gọi ngay lập tức, mà lúc ấy chưa có
khoản nào để gọi lên cả.
::::

::::assemble{#ghep-ca-cuon-so-thang}
Mười khoản, năm câu trả lời, một chương trình. Ba chỗ trống, ba câu hỏi khác
nhau.

```python title=starter
def lay_tien(khoan):
    """Đưa ra số tiền của một khoản, để sorted biết phải nhìn vào ô nào."""
    return khoan["tien"]


so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "sửa xe", "nhom": "đi lại", "tien": 500000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "đổ xăng", "nhom": "đi lại", "tien": 100000},
    {"ten": "vở ghi", "nhom": "học hành", "tien": 15000},
    {"ten": "sách", "nhom": "học hành", "tien": 120000},
    {"ten": "cơm trưa", "nhom": "ăn uống", "tien": 35000},
    {"ten": "gửi xe", "nhom": "đi lại", "tien": 10000},
    {"ten": "khoá tiếng Anh", "nhom": "học hành", "tien": 300000},
    {"ten": "trà sữa", "nhom": "ăn uống", "tien": 45000},
]

tong_thang = sum(___)
so_nhom = len(set([khoan["nhom"] for khoan in so]))

tong_nhom = {}
for khoan in so:
    nhom = khoan["nhom"]
    tong_nhom[nhom] = tong_nhom.get(nhom, 0) + khoan["tien"]

khoan_theo_tien = sorted(so, key=___, reverse=True)
ba_khoan_ton_nhat = [khoan["ten"] for khoan in khoan_theo_tien[:3]]
khoan_lon = [khoan["ten"] for khoan in so if ___]

print("SỔ CHI TIÊU — cả tháng, mười khoản")
print(f"Tổng chi: {tong_thang} đồng")
print(f"Đã chi vào {so_nhom} nhóm")
for nhom, tien in tong_nhom.items():
    print(f"  {nhom}: {tien} đồng")
print(f"Ba khoản tốn nhất: {ba_khoan_ton_nhat}")
print(f"Khoản trên 100000 đồng: {khoan_lon}")
```

```python title=solution
def lay_tien(khoan):
    """Đưa ra số tiền của một khoản, để sorted biết phải nhìn vào ô nào."""
    return khoan["tien"]


so = [
    {"ten": "cà phê", "nhom": "ăn uống", "tien": 25000},
    {"ten": "sửa xe", "nhom": "đi lại", "tien": 500000},
    {"ten": "bún bò", "nhom": "ăn uống", "tien": 40000},
    {"ten": "đổ xăng", "nhom": "đi lại", "tien": 100000},
    {"ten": "vở ghi", "nhom": "học hành", "tien": 15000},
    {"ten": "sách", "nhom": "học hành", "tien": 120000},
    {"ten": "cơm trưa", "nhom": "ăn uống", "tien": 35000},
    {"ten": "gửi xe", "nhom": "đi lại", "tien": 10000},
    {"ten": "khoá tiếng Anh", "nhom": "học hành", "tien": 300000},
    {"ten": "trà sữa", "nhom": "ăn uống", "tien": 45000},
]

tong_thang = sum([khoan["tien"] for khoan in so])
so_nhom = len(set([khoan["nhom"] for khoan in so]))

tong_nhom = {}
for khoan in so:
    nhom = khoan["nhom"]
    tong_nhom[nhom] = tong_nhom.get(nhom, 0) + khoan["tien"]

khoan_theo_tien = sorted(so, key=lay_tien, reverse=True)
ba_khoan_ton_nhat = [khoan["ten"] for khoan in khoan_theo_tien[:3]]
khoan_lon = [khoan["ten"] for khoan in so if khoan["tien"] > 100000]

print("SỔ CHI TIÊU — cả tháng, mười khoản")
print(f"Tổng chi: {tong_thang} đồng")
print(f"Đã chi vào {so_nhom} nhóm")
for nhom, tien in tong_nhom.items():
    print(f"  {nhom}: {tien} đồng")
print(f"Ba khoản tốn nhất: {ba_khoan_ton_nhat}")
print(f"Khoản trên 100000 đồng: {khoan_lon}")
```

```python title=test
# Ba chỗ trống, ba nhóm câu kiểm — mỗi chỗ có ít nhất một câu vỡ nếu điền sai.
#
# Chỗ 1 (bên trong `sum`): tổng mười khoản của cuốn sổ tháng.
assert tong_thang == 1190000, "mười khoản của sổ tháng này cộng lại hết 1 triệu 190 nghìn — cuốn năm khoản ở phần ví dụ là cuốn khác, đừng lấy con số của nó"
# Chỗ 2 (`key=`): xếp theo TIỀN, lớn xuống nhỏ. Xếp nhầm ô, hoặc quên `key=`,
# thì ba cái tên dưới đây đổi chỗ hoặc chương trình dừng vì không so nổi dict.
assert ba_khoan_ton_nhat == ["sửa xe", "khoá tiếng Anh", "sách"], "ba khoản tốn nhất của sổ tháng này, xếp từ lớn xuống nhỏ, là sửa xe 500000, khoá tiếng Anh 300000 rồi sách 120000"
# Chỗ 3 (`if` cuối comprehension): lọc theo ngưỡng 100000, giữ nguyên thứ tự
# ghi sổ. Ba cái tên này KHÔNG cùng thứ tự với ba khoản tốn nhất ở trên.
assert khoan_lon == ["sửa xe", "sách", "khoá tiếng Anh"], "ba khoản trên 100 nghìn của sổ tháng này, theo đúng thứ tự đã ghi trong sổ, là sửa xe, sách rồi khoá tiếng Anh — nếu danh sách của bạn có thêm 'đổ xăng' thì điều kiện đang nhận cả khoản đúng 100000 đồng, mà đúng ngưỡng thì chưa phải vượt ngưỡng"
# Hai câu còn lại không có chỗ trống, nhưng vẫn phải đúng — chúng là hai trong
# năm câu trả lời mà Byte đặt hàng.
assert so_nhom == 3, "mười khoản của sổ tháng này chỉ thuộc ba nhóm: ăn uống, đi lại và học hành"
assert tong_nhom["học hành"] == 435000, "nhóm học hành của sổ tháng này gồm vở ghi 15000, sách 120000 và khoá tiếng Anh 300000, cộng lại thành 435000"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm trong cặp ngoặc của `sum`, nên thứ điền vào phải là một **dãy số**. Chỗ thứ hai nằm ngay sau `key=`, nên thứ điền vào phải là thứ `sorted` gọi được. Chỗ thứ ba nằm sau chữ `if` ở cuối một comprehension, nên thứ điền vào phải là một câu hỏi đúng-sai về `khoan`.
- kind: strategy
  body: 'Chỗ một: viết một comprehension chạy qua cả cuốn sổ và nhặt ra ô `"tien"` của từng khoản. Chỗ hai: cái hàm đã được định nghĩa sẵn ở đầu chương trình, và tên nó nói đúng việc nó làm — viết tên trần, đừng thêm cặp ngoặc gọi. Chỗ ba: so ô `"tien"` của khoản đang xét với ngưỡng một trăm nghìn, và ngưỡng ấy đã có sẵn trong câu print ở cuối bài.'
- kind: one-line
  body: 'Lần lượt ba chỗ trống là `[khoan["tien"] for khoan in so]`, `lay_tien`, và `khoan["tien"] > 100000`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^SỔ CHI TIÊU — cả tháng, mười khoản\nTổng chi: 1190000 đồng\nĐã chi vào 3 nhóm\n  ăn uống: 145000 đồng\n  đi lại: 610000 đồng\n  học hành: 435000 đồng\nBa khoản tốn nhất: \['sửa xe', 'khoá tiếng Anh', 'sách'\]\nKhoản trên 100000 đồng: \['sửa xe', 'sách', 'khoá tiếng Anh'\]\s*$
- tier: output
  expect: "Ba khoản tốn nhất: ['sửa xe', 'khoá tiếng Anh', 'sách']"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn chỗ chứa, một cuốn sổ. Mỗi câu hỏi được trả lời bằng đúng thứ sinh ra để trả
lời nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang mạch sau — và lần này là câu bạn đã mang theo từ rất
lâu rồi.

Sổ tổng kết xong thì tắt máy là mất sạch. Đúng câu hỏi bạn mang từ cuối Realm 0:
chương trình quên hết mọi thứ khi tắt. Suốt cả track vừa rồi, cuốn sổ chi tiêu
luôn được gõ thẳng vào giữa chương trình — mười khoản ấy nằm trong chính file
code. Thêm một khoản là sửa code. Chạy lại lần sau, sổ vẫn đúng mười khoản cũ.

Vậy làm sao ghi cả cuốn sổ xuống **file** rồi mai mở lại đọc? Và làm sao chạy
chương trình này thẳng từ **dòng lệnh**, đưa cho nó tên cuốn sổ muốn tổng kết,
thay vì mở file code ra sửa mỗi lần?

Mạch sau (T1.5 — Chương trình thật: file, module, CLI, debugger) nhận đúng hai
câu hỏi này. Cuốn sổ sẽ rời khỏi file code và nằm trên đĩa; và cái list các dict
bạn vừa dựng sẽ được dựng lại từ những dòng chữ đọc lên từ đó.
::::

::::checkpoint{mastery=0.85}
::::
