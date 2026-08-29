---
id: nen-tang.list-dict-set-tuple.mot-dong-thay-bon-dong
title: Một dòng thay bốn dòng
summary: Một dòng nói thẳng lấy gì và lấy từ đâu, thay cho bốn dòng của khuôn cũ — và cho ra đúng cùng một danh sách.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.list-comprehension]
requires: [core.set-difference, core.list-of-dicts, core.dict-nested-index, core.set, core.list, core.list-append, core.len, ctrl.for-each, core.fstring]
concepts: [core.danh-sach, core.so-tra-cuu]
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
Bốn dòng để nói một câu. Mình muốn nói thẳng câu đó thôi.
::::

::::explain{#bon-vai-trong-bon-dong}
Bài trước để lại một khuôn tay bạn đã gõ nhiều lần. Đây là nó, trên cuốn sổ của
bài 23 — một danh sách các khoản, mỗi khoản là một dict:

```python title=readonly
ten_cac_khoan = []
for khoan in so:
    ten = khoan["ten"]
    ten_cac_khoan.append(ten)
```

Bốn dòng, và mỗi dòng đóng đúng một vai:

1. `ten_cac_khoan = []` — dựng ra một **danh sách mới** để hứng kết quả.
2. `for khoan in so:` — nói **lấy từ đâu**: từng khoản một, trong cuốn sổ.
3. `ten = khoan["ten"]` — nói **lấy gì** ở mỗi khoản: cái tên của nó.
4. `.append(ten)` — **bỏ vào** danh sách mới.

Dòng 3 và dòng 4 gộp làm một cũng được, viết thẳng `ten_cac_khoan.append(khoan["ten"])`;
tách ra thế này để nhìn rõ bốn vai.

Nhìn kỹ thì vai 1 và vai 4 chẳng mang thông tin gì riêng của bài toán này: lần
nào cũng "dựng một danh sách rỗng" rồi "bỏ vào cuối". Chúng là thủ tục. Thông
tin thật chỉ nằm ở vai 2 và vai 3 — **lấy gì**, và **từ đâu**.

Python cho bạn viết thẳng hai vai ấy, và tự lo hai vai kia:

```python title=readonly
ten_cac_khoan = [khoan["ten"] for khoan in so]
```

Đọc từ ngoài vào: cặp ngoặc vuông `[ ]` là lời hứa "kết quả sẽ là một danh sách
mới" — đúng vai 1 và vai 4 gộp lại. Bên trong nó, `khoan["ten"]` là **lấy gì**,
và `for khoan in so` là **từ đâu**.

Đọc thành lời: *lấy `khoan["ten"]`, với mỗi `khoan` trong `so`.*

Lối viết này có tên riêng trong tiếng Anh của giới lập trình Python:
**comprehension**. Ở đây cứ gọi nó là **một dòng thay bốn dòng** cũng được — vì
đó đúng là việc nó làm.
::::

::::example{#hai-cach-mot-ket-qua}
Từ đây tới hết mạch, Byte mở một cuốn sổ **tháng mới** — vẫn năm khoản, vẫn
đúng bốn ô mỗi khoản như bài 23 đã dựng, nhưng số tiền lớn hơn hẳn vì tháng
này có khoản sửa xe. Chạy cả hai cách trên cuốn ấy rồi đem so.

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "học phí", "tien": 300000, "ngay": 15, "nhom": "học phí"},
]

cach_cu = []
for khoan in so:
    ten = khoan["ten"]
    cach_cu.append(ten)

cach_moi = [khoan["ten"] for khoan in so]

print(cach_cu)
print(cach_moi)
print(cach_cu == cach_moi)
```

Máy in ra:

```text title=readonly
['cà phê', 'xăng', 'ăn trưa', 'sửa xe', 'học phí']
['cà phê', 'xăng', 'ăn trưa', 'sửa xe', 'học phí']
True
```

Hai danh sách bằng nhau từng phần tử một, đúng thứ tự. Không phải "gần giống", mà
là **bằng** — dòng thứ ba hỏi thẳng máy chuyện đó.

Ba chuyện đọc ra được từ đây:

- **Kết quả là một danh sách**, không phải rổ. Nên nó có thứ tự, có `[0]`, và giữ
  nguyên thứ tự các khoản trong sổ.
- **Cái tên `khoan` do chính dòng ấy sinh ra**, y như `for khoan in so:` ở bản
  bốn dòng sinh ra nó. Bạn không phải gán `khoan` trước ở đâu cả.
- **Cuốn sổ `so` không bị đụng tới.** Một dòng ấy đọc sổ rồi dựng ra một danh
  sách mới — cùng nếp với `sorted` ở bài 19, với `&` ở bài 28 và `-` ở bài 29.

Đổi phần **lấy gì** thì ra một danh sách khác, mà phần **từ đâu** không phải sửa
một chữ:

```python title=readonly
so_tien = [khoan["tien"] for khoan in so]
print(so_tien)
print(sum(so_tien))
```

Máy in ra:

```text title=readonly
[90000, 240000, 620000, 500000, 300000]
1750000
```
::::

::::predict{#doan-mot-dong commitOnce}
Byte lấy trường `nhom` của cuốn sổ năm khoản ấy. Trong sổ, `cà phê` và `ăn trưa`
cùng thuộc nhóm `ăn uống`; `xăng` và `sửa xe` cùng thuộc nhóm `xăng xe`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "học phí", "tien": 300000, "ngay": 15, "nhom": "học phí"},
]

print([khoan["nhom"] for khoan in so])
```

:::opt{correct}
`['ăn uống', 'xăng xe', 'ăn uống', 'xăng xe', 'học phí']` — năm phần tử, hai cái tên nằm hai lần
:::

:::opt
`['ăn uống', 'xăng xe', 'học phí']` — ba phần tử, trùng bị bỏ
::why
Gần đúng ở chỗ bạn đang mang theo bốn bài về rổ vừa học, và ba nhóm ấy đúng là ba
nhóm khác nhau có trong sổ. Nếu câu hỏi là "sổ này chi vào những nhóm nào" thì
đáp án của bạn mới là đáp án đúng.

Chỗ lệch nằm ở cặp ngoặc. Ngoặc **vuông** dựng ra một danh sách, và danh sách
nhận đủ mọi lần xuất hiện. Bốn dòng mà một dòng này thay thế cũng làm y hệt:
`.append` không bao giờ hỏi "đã có chưa". Muốn bỏ trùng thì phải bọc thêm
`set(...)` — một việc riêng, không phải việc của cặp ngoặc vuông.
::
:::

:::opt
`['cà phê', 'xăng', 'ăn trưa', 'sửa xe', 'học phí']`
::why
Gần đúng ở phần khó: bạn đọc ra rằng kết quả có năm phần tử, đúng bằng số khoản
trong sổ, và giữ nguyên thứ tự ghi sổ. Phần suy luận về hình dạng kết quả của bạn
chính xác.

Chỗ lệch là ở **lấy gì**. Đoạn này viết `khoan["nhom"]`, tức là mỗi lượt lấy
trường `nhom`; còn danh sách bạn chọn là danh sách trường `ten`. Đổi một chữ
trong ngoặc là đổi cả cột được lấy ra — đó chính là chỗ duy nhất mang thông tin
riêng của bài toán trong cả dòng.
::
:::

:::opt
Máy báo lỗi, vì `khoan` chưa được gán ở dòng nào trước đó
::why
Gần đúng ở chỗ bạn nhớ một luật thật và nhớ đúng: dùng một cái tên chưa ai gán
thì máy dừng lại báo `NameError`, chuyện này Realm 0 đã cho bạn thấy tận mắt.

Chỗ lệch: cái tên `khoan` **được sinh ra ngay trong dòng ấy**, bởi chính phần
`for khoan in so`. Đúng như `for khoan in so:` ở bản bốn dòng sinh ra nó mà không
cần ai gán trước. Trong một dòng gọn, phần `for` vẫn giữ nguyên nhiệm vụ ấy.
::
:::
::::

::::explain{#mot-dong-nay-khong-loc-va-khong-bo-trung}
Đúng như bạn vừa đoán. Bốn bài vừa rồi toàn nói về rổ, nên chỗ này phải nói rõ
để khỏi lẫn.

**Dòng bạn vừa viết lấy hết, không bỏ sót phần tử nào.** Cuốn sổ có năm khoản thì
danh sách ra năm phần tử, không hơn không kém. Trong dòng ấy mới có đúng hai mẩu
— **lấy gì** và **từ đâu** — và không mẩu nào nói "chỉ lấy khoản nào thoả điều
kiện". Cũng phải thôi: bốn dòng mà nó đang thay thế cũng chẳng có câu `if` nào.

**Và nó không bỏ trùng.** Ngoặc vuông dựng ra một danh sách, mà danh sách thì
nhận đủ mọi lần xuất hiện — đúng như `.append` không bao giờ hỏi "đã có chưa".
Lấy trường `nhom` của cuốn sổ trên thì `ăn uống` hiện ra **hai** lần và `xăng xe`
cũng hai lần:

```python title=readonly
nhom_cac_khoan = [khoan["nhom"] for khoan in so]
print(nhom_cac_khoan)
print(len(nhom_cac_khoan))
```

Máy in ra:

```text title=readonly
['ăn uống', 'xăng xe', 'ăn uống', 'xăng xe', 'học phí']
5
```

Muốn bỏ trùng thì vẫn là việc của bài 26: bọc kết quả lại bằng `set(...)`, và năm
phần tử ấy rút xuống còn ba nhóm khác nhau.

> Chỗ dễ vấp: thứ tự **viết** trong dòng ngược với thứ tự **máy làm**. Máy chạy
> phần `for khoan in so` trước để có `khoan`, rồi mới tính `khoan["ten"]` — nhưng
> `khoan["ten"]` lại được viết trước. Lý do là dòng này viết theo lối kể cho
> người nghe: người ta hỏi "lấy gì" trước, "từ đâu" sau. Nếu đọc mà thấy rối, cứ
> đọc từ chữ `for` trở đi trước, rồi quay lại đầu dòng.
::::

::::code{#lay-hai-cot-ra-khoi-so}
Byte cần hai danh sách rút ra từ cuốn sổ: một danh sách **tên** các khoản, và một
danh sách **số tiền** của chúng — cùng thứ tự với sổ.

Hai chỗ trống, mỗi chỗ đúng một dòng. Đừng gõ lại bốn dòng cũ: bài này muốn đúng
lối viết một dòng.

```python title=starter
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "học phí", "tien": 300000, "ngay": 15, "nhom": "học phí"},
]

ten_cac_khoan = ___
so_tien = ___

print(f"Tên các khoản: {ten_cac_khoan}")
print(f"Số tiền từng khoản: {so_tien}")
print(f"Cả sổ hết: {sum(so_tien)} đồng")
```

```python title=solution
so = [
    {"ten": "cà phê", "tien": 90000, "ngay": 3, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 240000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "ăn trưa", "tien": 620000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "sửa xe", "tien": 500000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "học phí", "tien": 300000, "ngay": 15, "nhom": "học phí"},
]

ten_cac_khoan = [khoan["ten"] for khoan in so]
so_tien = [khoan["tien"] for khoan in so]

print(f"Tên các khoản: {ten_cac_khoan}")
print(f"Số tiền từng khoản: {so_tien}")
print(f"Cả sổ hết: {sum(so_tien)} đồng")
```

```python title=test
# So cả danh sách chứ không so mỗi `len` hay mỗi tổng: lấy nhầm trường `nhom`
# vẫn ra đủ năm phần tử, và một danh sách tiền xáo thứ tự vẫn cộng ra đúng
# con số tổng.
assert ten_cac_khoan == ["cà phê", "xăng", "ăn trưa", "sửa xe", "học phí"], "danh sách tên phải giữ nguyên thứ tự năm khoản trong sổ: cà phê, xăng, ăn trưa, sửa xe, học phí"
assert so_tien == [90000, 240000, 620000, 500000, 300000], "danh sách tiền phải là số tiền của năm khoản theo đúng thứ tự sổ, và là số nguyên chứ không phải chuỗi"
assert sum(so_tien) == 1750000, "năm khoản trong cuốn sổ này cộng lại hết 1 triệu 750 nghìn"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống cần đủ hai mẩu thông tin: **lấy gì** ở mỗi khoản, và **từ đâu**. Cái tên biến ở vế trái đã nói cho bạn biết mẩu thứ nhất; cuốn sổ ngay phía trên là mẩu thứ hai. Cặp ngoặc bao ngoài chính là thứ hứa rằng kết quả sẽ là một danh sách mới.
- kind: strategy
  body: Dựng lại theo bản bốn dòng rồi gói lại. Phần `for khoan in so` giống hệt nhau ở cả hai chỗ trống, chỉ mẩu **lấy gì** là khác: một chỗ lấy trường tên, một chỗ lấy trường tiền — mà trường của một khoản thì tra bằng ngoặc vuông kèm tên trường, đúng lối hai lớp ngoặc của bài 24.
- kind: one-line
  body: "Dòng thứ nhất viết `[khoan[\"ten\"] for khoan in so]`, dòng thứ hai viết `[khoan[\"tien\"] for khoan in so]`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Tên các khoản: \['cà phê', 'xăng', 'ăn trưa', 'sửa xe', 'học phí'\]\nSố tiền từng khoản: \[90000, 240000, 620000, 500000, 300000\]\nCả sổ hết: 1750000 đồng\s*$
- tier: output
  expect: Cả sổ hết: 1750000 đồng
- tier: static
  onFail: mỗi chỗ trống phải là một dòng rút từ cuốn sổ ra, không phải một danh sách gõ sẵn
  requireAst:
  - kind: comprehension, min: 2
  - kind: uses-name, target: so, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lấy gì, từ đâu. Nói xong câu là xong việc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một dòng ấy lấy **hết**: sổ có bao nhiêu khoản thì danh sách ra bấy nhiêu phần
tử. Đó vừa là chỗ mạnh vừa là chỗ hụt của nó.

Byte hỏi tiếp một câu rất đời thường: *cho mình xem tên những khoản trên 100
nghìn thôi.* Trong cuốn sổ năm khoản vừa rồi, đó là `xăng`, `ăn trưa`, `sửa xe`
và `học phí` — bốn khoản, không phải năm.

Bản bốn dòng có sẵn chỗ cho câu hỏi ấy: chèn một câu `if` vào giữa vòng lặp —
đúng câu `if` mà bài 25, bài 28 và bài 29 đều đã gõ — rồi chỉ những khoản lọt qua
mới được `.append`.

Nhưng dòng gọn thì chỉ có hai mẩu: **lấy gì**, và **từ đâu**. Không có mẩu nào
mang tên "chỉ lấy khi".

Nhét điều kiện ấy vào chỗ nào trong dòng ấy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
