---
id: nen-tang.list-dict-set-tuple.hai-lop-ngoac
title: Hai lớp ngoặc
summary: Đọc `so[2]["tien"]` từ trái sang phải — cặp ngoặc thứ nhất lấy ra một khoản, cặp thứ hai hỏi khoản ấy một trường.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.nested-index, core.dict-nested-index]
requires: [core.list-of-dicts, core.nested-list-dict, core.dict, core.list, core.list-index, core.list-negative-index, core.len, ctrl.for-each, core.fstring, core.output, err.key-error, err.type-error]
concepts: [core.danh-sach, core.so-tra-cuu, core.long-nhau]
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
Hai câu hỏi liền nhau thôi. Chỉ là mình không đặt tên cho câu trả lời ở giữa.
::::

::::explain{#hai-buoc-truoc-da}
Sếp hỏi số tiền của khoản thứ ba. Bạn có `so[2]`, và nó đưa ra nguyên một dict.

Làm hai bước thì xong ngay, vì cả hai bước đều là thứ bạn đã dùng nhiều lần:

```python title=readonly
khoan = so[2]
tien = khoan["tien"]
print(tien)
```

```text title=readonly
40000
```

Đọc lại từng dòng cho kỹ, vì cả bài này nằm ở đây:

- Dòng 1 hỏi **danh sách** `so`: "ô số 2 của mày là gì?" Danh sách trả lời bằng
  một dict, và cái tên `khoan` giữ lấy nó.
- Dòng 2 hỏi **dict** `khoan`: "trường `tien` của mày là gì?" Dict trả lời bằng
  một con số.

Hai câu hỏi khác nhau, hỏi hai chỗ chứa khác nhau. Cái tên `khoan` chỉ làm mỗi
việc bắc cầu giữa chúng — nó cầm câu trả lời của câu hỏi thứ nhất đúng một dòng
rồi không ai dùng tới nữa.
::::

::::explain{#bo-cai-ten-o-giua}
Cái tên bắc cầu ấy bỏ được. Chỗ nào đang viết `khoan`, thay bằng chính thứ mà
`khoan` đang giữ — tức là `so[2]`:

```python title=readonly
tien = so[2]["tien"]
```

Đây là câu trả lời cho câu hỏi cuối bài trước, và cách đọc nó là **từ trái sang
phải**, mỗi cặp ngoặc một bước:

- `so` — cuốn sổ, một danh sách.
- `so[2]` — hỏi danh sách ấy ô số 2. Ra một dict.
- `so[2]["tien"]` — hỏi cái dict vừa ra ấy trường `tien`. Ra một con số.

Máy làm đúng ba bước đó, đúng thứ tự đó. Cặp ngoặc thứ hai không bao giờ được
chạy trước cặp thứ nhất, vì trước khi cặp thứ nhất trả lời xong thì chưa có dict
nào để mà hỏi.

Hai cặp ngoặc trông giống hệt nhau nhưng **hỏi hai loại câu khác nhau**, và thứ
nằm bên trong ngoặc nói cho bạn biết loại nào:

- ngoặc đặt sau một **list** chứa một **chỗ đứng** — `[2]`, `[0]`, `[-1]`;
- ngoặc đặt sau một **dict** chứa một **khoá** — `["tien"]`, `["nhom"]`.

Nên khi đọc `so[2]["tien"]`, bạn không phải đoán: `[2]` là số nên nó đang hỏi
một danh sách, `["tien"]` là chữ nên nó đang hỏi một dict.

Muốn giữ cái tên `khoan` cũng không sai chút nào. Khi phải hỏi cùng một khoản
tới ba bốn trường, đặt tên cho nó ra một dòng riêng thì dễ đọc hơn hẳn. Bỏ tên
đi chỉ đáng làm khi bạn hỏi đúng một trường, đúng một lần.
::::

::::example{#doc-tu-trai-sang-phai}
Ba câu hỏi trên cùng cuốn sổ tháng này, mỗi câu một dòng:

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
]

print(so[2]["tien"])
print(so[0]["nhom"])
print(so[-1]["ngay"])

print("Cả sổ:")
for khoan in so:
    print(f"ngày {khoan['ngay']}: {khoan['ten']} — {khoan['tien']} đồng ({khoan['nhom']})")
```

Máy in ra:

```text title=readonly
40000
ăn uống
14
Cả sổ:
ngày 2: cà phê — 25000 đồng (ăn uống)
ngày 5: xăng — 60000 đồng (xăng xe)
ngày 8: bún bò — 40000 đồng (ăn uống)
ngày 11: vá lốp — 30000 đồng (xăng xe)
ngày 14: bánh mì — 15000 đồng (ăn uống)
```

`so[-1]["ngay"]` cho `14` — cặp ngoặc thứ nhất vẫn nhận chỗ đứng âm như mọi list
từ bài 1, chuyện ô ấy chứa một dict không đổi gì cả.

Còn khối `for` ở cuối là hình dạng cũ chứ không phải hình dạng mới: `khoan` là
một cái tên thật, giữ trọn một dict ở mỗi lượt, nên `khoan['ngay']` chỉ có
**một** cặp ngoặc. Vòng lặp đã làm hộ bước thứ nhất rồi.

Trong f-string thì dấu nháy bên trong ngoặc phải khác dấu nháy bọc ngoài, nên ở
đây là `{khoan['ngay']}` — nháy đơn bên trong, nháy kép bọc cả câu.
::::

::::predict{#doan-doi-cho-hai-ngoac commitOnce}
Byte gõ vội và đổi chỗ hai cặp ngoặc: `so["tien"][2]` thay vì `so[2]["tien"]`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
]

print(so["tien"][2])
```

:::opt{correct}
Không in ra con số nào. Máy dừng lại và báo `TypeError`.
:::

:::opt
Vẫn in ra `40000`, vì hai cặp ngoặc đã hỏi đủ cả hai câu.
::why
Gần đúng ở chỗ bạn nhìn thấy đúng hai mẩu thông tin cần có: khoá `"tien"` và chỗ
đứng `2`. Đủ hai mẩu ấy thì người đọc hiểu ngay ý định.

Chỗ lệch là máy không đọc ý định, nó đọc **thứ tự**. Nó làm cặp ngoặc bên trái
trước, mà bên trái là `so["tien"]` — hỏi một danh sách bằng một chữ. Tới đó nó
đã dừng rồi; cặp ngoặc `[2]` bên phải không bao giờ tới lượt được chạy. Máy
không có quy ước nào cho phép nó đổi chỗ hai câu hỏi giúp bạn, y như nó không tự
biến `"45000" + 5000` thành phép cộng ở Realm 0.
::
:::

:::opt
Máy báo `KeyError`, vì `so` không có khoá `"tien"`.
::why
Gần đúng ở chỗ bạn nhớ chính xác `KeyError` là gì và nó xảy ra khi nào: hỏi một
**dict** một khoá mà dict ấy không có. Nếu `so` là một dict thì bạn đã đúng.

Chỗ lệch: `so` không phải dict, nó là **list**. Một list không có khoá nào cả —
không phải nó thiếu khoá `"tien"`, mà nó không có chỗ để chứa khoá. Nên lời than
của máy không phải "thiếu khoá này" mà là "thứ mày đặt trong ngoặc sai loại":
`TypeError: list indices must be integers or slices, not str` — chỗ đứng của
list phải là số nguyên hoặc lát cắt, không phải chữ.
::
:::

:::opt
Máy báo `IndexError`, vì `"tien"` không phải một chỗ đứng có thật trong sổ.
::why
Gần đúng ở chỗ bạn khoanh trúng chỗ hỏng: chuyện rắc rối đúng là nằm ở cặp ngoặc
đặt sau `so`, và đúng là nằm ở chỗ đứng.

Chỗ lệch nằm ở việc phân biệt hai loại than phiền — đúng cặp `ValueError` với
`TypeError` mà Realm 0 đã dựng. `IndexError` là "đúng loại thứ, sai nội dung":
bạn đưa vào một con số thật, chỉ là sổ có năm ô mà bạn hỏi ô số 9. Còn ở đây bạn
đưa vào một **chữ**, và chữ thì không bao giờ là chỗ đứng của bất kỳ list nào —
sai ngay từ loại, nên máy nói `TypeError`.
::
:::
::::

::::code{#ba-cau-tra-loi-cho-sep}
Sếp hỏi ba câu, mỗi câu về một khoản khác nhau và một trường khác nhau:

1. Khoản **thứ ba** tiêu hết bao nhiêu **tiền**?
2. Khoản **đầu sổ** thuộc **nhóm** nào?
3. Khoản **cuối sổ** chi vào **ngày** nào?

Ba chỗ trống dưới đây, mỗi chỗ một câu trả lời. Đọc chúng ra từ cuốn sổ, đừng gõ
thẳng con số bạn nhìn thấy — tháng sau sổ đổi, mà mã đọc từ sổ thì vẫn đúng.

```python title=starter
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
]

tien_khoan_ba = ___
nhom_khoan_dau = ___
ngay_khoan_cuoi = ___

print(f"Khoản thứ ba tiêu hết {tien_khoan_ba} đồng")
print(f"Khoản đầu sổ thuộc nhóm {nhom_khoan_dau}")
print(f"Khoản cuối sổ chi vào ngày {ngay_khoan_cuoi}")
```

```python title=solution
so = [
    {"ten": "cà phê", "tien": 25000, "ngay": 2, "nhom": "ăn uống"},
    {"ten": "xăng", "tien": 60000, "ngay": 5, "nhom": "xăng xe"},
    {"ten": "bún bò", "tien": 40000, "ngay": 8, "nhom": "ăn uống"},
    {"ten": "vá lốp", "tien": 30000, "ngay": 11, "nhom": "xăng xe"},
    {"ten": "bánh mì", "tien": 15000, "ngay": 14, "nhom": "ăn uống"},
]

tien_khoan_ba = so[2]["tien"]
nhom_khoan_dau = so[0]["nhom"]
ngay_khoan_cuoi = so[-1]["ngay"]

print(f"Khoản thứ ba tiêu hết {tien_khoan_ba} đồng")
print(f"Khoản đầu sổ thuộc nhóm {nhom_khoan_dau}")
print(f"Khoản cuối sổ chi vào ngày {ngay_khoan_cuoi}")
```

```python title=test
# Ba phép kiểm cho ba chỗ trống, mỗi phép chạm đúng một chỗ. Ba khoản được hỏi
# nằm ở ba chỗ khác nhau trong sổ (giữa, đầu, cuối) và ba trường được hỏi cũng
# khác nhau, nên một chỗ điền nhầm không thể núp sau hai chỗ kia.
assert tien_khoan_ba == 40000, "ô số 2 của cuốn sổ này là bát bún bò, và trường tien của nó là 40000"
assert nhom_khoan_dau == "ăn uống", "ô số 0 của cuốn sổ này là cốc cà phê, và trường nhom của nó là 'ăn uống'"
assert ngay_khoan_cuoi == 14, "ô cuối cuốn sổ này là ổ bánh mì, và trường ngay của nó là 14"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống cần hai câu hỏi liền nhau, không phải một. Câu thứ nhất hỏi cuốn sổ `so` một **chỗ đứng**, câu thứ hai hỏi thứ vừa nhận được một **khoá**. Ba chỗ đứng cần dùng lần lượt là ô giữa, ô đầu và ô cuối.
- kind: strategy
  body: Viết câu hỏi thứ nhất trước rồi nối câu thứ hai vào ngay sau, không xuống dòng, không đặt tên trung gian. Khoản thứ ba là ô số 2 vì chỗ đứng đếm từ 0; khoản đầu sổ là ô số 0; khoản cuối sổ thì dùng chỗ đứng âm của bài 1 để khỏi phải biết sổ dài bao nhiêu. Ba khoá cần hỏi lần lượt là `"tien"`, `"nhom"`, `"ngay"`.
- kind: one-line
  body: 'Ba dòng lần lượt là `so[2]["tien"]`, `so[0]["nhom"]`, `so[-1]["ngay"]`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Khoản thứ ba tiêu hết 40000 đồng\nKhoản đầu sổ thuộc nhóm ăn uống\nKhoản cuối sổ chi vào ngày 14\s*$
- tier: output
  expect: Khoản cuối sổ chi vào ngày 14
- tier: static
  onFail: ba câu trả lời phải được ĐỌC RA từ cuốn sổ, không phải gõ thẳng giá trị bạn nhìn thấy
  requireAst:
  # `uses-name` chỉ đếm chỗ ĐỌC tên, còn dòng `so = [...]` là chỗ gán nên không
  # tính. Vậy ba lần đọc `so` chỉ có thể tới từ ba chỗ trống; chép cứng một giá
  # trị là mất một lần đọc, và luật vỡ ngay.
  - kind: uses-name, target: so, min: 3
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Trái trước, phải sau. Mỗi cặp ngoặc bóc đúng một tầng, không hơn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cuối tháng sếp thôi hỏi từng khoản. Sếp hỏi tổng **theo từng nhóm**: ăn uống hết
bao nhiêu, xăng xe hết bao nhiêu.

Bây giờ bạn đọc được cả `khoan["nhom"]` lẫn `khoan["tien"]`, nên cách làm hiện
ra ngay: mỗi nhóm một cái tên riêng, sinh ra từ `0` trước vòng lặp.

```python title=readonly
tong_an_uong = 0
tong_xang_xe = 0
```

Rồi duyệt cuốn sổ, mỗi khoản hỏi xem nó thuộc nhóm nào và cộng vào đúng cái tên
ấy — một chuỗi `if` mấy nhánh. Chạy được, và với cuốn sổ này thì ra đúng số.

Rồi Byte mua vở đi học, và cuốn sổ mọc thêm một khoản thuộc nhóm
`"học phí"`. Chương trình của bạn không có nhánh nào cho nhóm ấy: khoản đó đi
qua hết mọi câu hỏi, không nhánh nào nhận, rồi biến mất khỏi bản tổng kết — không
một tiếng kêu, đúng loại lỗi im lặng mà bài 8 đã cho bạn thấy.

Vậy là mỗi nhóm mới lại phải mở chương trình ra, thêm một cái tên, thêm một
nhánh. Cuốn sổ thì tự mọc thêm nhóm được, còn chương trình thì không.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
