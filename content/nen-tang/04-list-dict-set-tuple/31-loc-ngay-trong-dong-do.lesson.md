---
id: nen-tang.list-dict-set-tuple.loc-ngay-trong-dong-do
title: Lọc ngay trong dòng đó
summary: Mẩu thứ ba của dòng gọn — `if` đặt ở cuối nói "chỉ lấy khi", còn đầu dòng vẫn giữ nguyên việc nói "lấy gì".
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 31
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.comprehension-filter]
practices: [core.list-comprehension, core.list-of-dicts, core.nested-index, core.len, core.fstring]
requires: [core.list-comprehension, core.list-of-dicts, core.nested-index, core.dict, core.list, core.list-append, core.len, ctrl.for-each, ctrl.if, core.fstring]
concepts: [core.danh-sach, ctrl.re-nhanh]
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
Dòng gọn của bạn có hai mẩu. Hôm nay nó mọc thêm mẩu thứ ba, ở cuối.
::::

::::explain{#mau-thu-ba}
Bài trước kết ở đúng một chỗ hụt: dòng gọn chỉ có hai mẩu — **lấy gì** và **từ
đâu** — nên nó lấy hết, không mẩu nào nói được "chỉ lấy khoản nào thoả điều
kiện". Mà Byte thì đang hỏi *cho mình xem tên những khoản trên 100 nghìn thôi*.

Bản bốn dòng có sẵn chỗ cho câu hỏi ấy. Chèn một `if` vào giữa vòng lặp là xong:

```python title=readonly
ten_dat = []
for khoan in so:
    if khoan["tien"] > 100000:
        ten_dat.append(khoan["ten"])
```

Bốn dòng thành năm, và vai thứ năm ấy có tên rất dễ gọi: **chỉ lấy khi**.

Dòng gọn cũng nhận được vai đó, và chỗ của nó là **cuối dòng**, ngay sau phần
`for`:

```text title=readonly
[  lấy gì   for  từng cái  in  từ đâu  if  chỉ lấy khi  ]
```

Ba mẩu, viết liền một hàng. Và vẫn đúng lời dặn của bài trước: thứ tự viết ngược
với thứ tự máy làm. Máy chạy phần `for` trước để có `khoan`, rồi hỏi `if`, và chỉ
những lượt trả lời `True` mới đi tiếp lên đầu dòng để lấy giá trị.
::::

::::example{#bon-cai-ten-bai-truoc-hua}
Vẫn cuốn sổ tháng mới của bài trước. Bản dài trước:

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
    if khoan["tien"] > 100000:
        cach_cu.append(khoan["ten"])

cach_moi = [khoan["ten"] for khoan in so if khoan["tien"] > 100000]

print(cach_cu)
print(cach_moi)
print(cach_cu == cach_moi)
```

Máy in ra:

```text title=readonly
['xăng', 'ăn trưa', 'sửa xe', 'học phí']
['xăng', 'ăn trưa', 'sửa xe', 'học phí']
True
```

Đúng bốn cái tên mà bài trước đã hứa, và hai cách cho ra hai danh sách **bằng**
nhau — dòng thứ ba hỏi thẳng máy chuyện đó.

Đặt hai bản cạnh nhau thì thấy dòng gọn dùng lại nguyên si từng mảnh của bản dài:

- `for khoan in so` — y hệt dòng `for`.
- `if khoan["tien"] > 100000` — y hệt dòng `if`, mất mỗi dấu hai chấm.
- `khoan["ten"]` — y hệt thứ nằm trong ngoặc của `.append`, chỉ khác là nó chuyển
  lên đứng đầu.

Năm dòng của bản dài không mất đi đâu cả. Chúng bị xếp lại thành một hàng, và duy
nhất **một** mảnh đổi chỗ: thứ được lấy ra chạy lên trước.
::::

::::explain{#dau-dong-va-cuoi-dong}
Một dòng gọn có lọc thì mang **hai** quyết định, và chúng không giẫm lên nhau:

- **Đầu dòng quyết định lấy cái gì.** `khoan["ten"]` cho ra tên,
  `khoan["tien"]` cho ra tiền, `khoan` trơ trọi cho ra nguyên cả dict.
- **Cuối dòng quyết định ai được lọt.** `if` không đụng gì tới giá trị; nó chỉ bỏ
  bớt phần tử. Danh sách ra ngắn hơn cuốn sổ, hoặc dài bằng, không bao giờ dài
  hơn.

Hai chuyện đó rời nhau, nên bạn đổi một bên mà bên kia đứng yên: muốn số tiền của
những khoản trên 100 nghìn thì đổi mỗi đầu dòng, muốn tên của những khoản thuộc
nhóm ăn uống thì đổi mỗi cuối dòng.

Còn một điều bài trước đã nói mà cái đuôi này **không** sửa được: dòng gọn vẫn
không bỏ trùng. Nó bỏ bớt phần tử theo điều kiện, thế thôi — hai khoản khác nhau
mà cùng tên thì cả hai vẫn đi ra, miễn là cả hai cùng lọt qua `if`.

> Chỗ dễ vấp: `if` ở đây **không có** dấu hai chấm và **không có** thân thụt vào.
> Nó không phải một câu lệnh rẽ nhánh đứng riêng, nó là một mảnh nằm trong cặp
> ngoặc vuông. Gõ thêm dấu hai chấm vào đó là `SyntaxError`.

Và cái đuôi ấy không bắt buộc. Bỏ `if` đi thì dòng quay về đúng dòng của bài
trước và lấy hết. Thứ hôm nay không thay thế thứ hôm qua — nó gắn thêm vào.
::::

::::predict{#doan-danh-sach-ten commitOnce}
Tháng chưa hết, và Byte ghi tiếp. **Cuốn sổ dưới đây không còn là cuốn năm khoản
nữa**: nó dài **tám** khoản, chạy tới ngày 22. Ba khoản mới nằm ở cuối, và hai
trong ba khoản ấy cố tình nằm sát ngưỡng — "vá lốp" tròn 100 nghìn, "xăng" ngày
22 được 105 nghìn. Khoản cuối cũng trùng tên với khoản ngày 5, vì tháng này Byte
đổ xăng hai lần.

Đoạn code thì không đổi một chữ nào.

**Trước khi bấm chạy**, bạn đoán màn hình in ra gì?

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

dat = [khoan["ten"] for khoan in so if khoan["tien"] > 100000]
print(dat)
```

:::opt{correct}
['xăng', 'ăn trưa', 'sửa xe', 'học phí', 'xăng']
:::

:::opt
['xăng', 'ăn trưa', 'sửa xe', 'học phí', 'vá lốp', 'xăng']
::why
Gần đúng ở chỗ khó nhất: bạn đọc ra rằng đầu dòng lấy **tên**, phần `if` chỉ chọn
ai được lọt, và bạn giữ đúng thứ tự ghi sổ — kể cả chuyện "xăng" xuất hiện hai
lần.

Chỗ lệch nằm ở đúng một khoản: "vá lốp", tròn 100 nghìn. Dấu `>` hỏi "có **lớn
hơn** không", mà một con số thì không lớn hơn chính nó — nên ở khoản ấy điều kiện
đọc ra `False` và nó không được thêm vào.

Muốn "vá lốp" lọt thì phải đổi dấu thành `>=`. Hai dấu ấy chỉ khác nhau ở đúng
những khoản rơi trúng con số ngưỡng, và ba khoản mới được thêm vào sổ chính là để
chỗ khác nhau đó lộ ra.
::
:::

:::opt
[240000, 620000, 500000, 300000, 105000]
::why
Gần đúng ở chỗ bạn theo dõi phần lọc rất chuẩn: đúng năm khoản ấy vượt 100 nghìn,
không thừa không thiếu một khoản nào, và thứ tự cũng đúng.

Chỗ lệch là ở chuyện **ai** quyết định thứ đi ra. Một dòng gọn có hai việc tách
bạch: phần `if` ở cuối chỉ nói *khoản này có được lọt hay không*, còn phần đứng
**đầu dòng** mới nói *lọt rồi thì lấy cái gì của nó*. Ở đây đầu dòng viết
`khoan["ten"]`, nên thứ đi ra là tên.

Muốn ra đúng năm con số bạn vừa kể thì đổi đầu dòng thành `khoan["tien"]`, còn
phần `if` giữ nguyên không đụng tới.
::
:::

:::opt
['xăng', 'ăn trưa', 'sửa xe', 'học phí']
::why
Gần đúng ở chỗ bạn lọc không sai một khoản nào: bốn cái tên này đều thuộc về
những khoản trên 100 nghìn, và khoản "vá lốp" tròn ngưỡng bị bạn loại ra rất
chuẩn. Đây cũng đúng là kết quả của cuốn sổ **năm khoản** lúc nãy.

Chỗ lệch là ở khoản thứ tám. Ba khoản mới đã vào sổ, và một trong ba là "xăng"
ngày 22 hết 105 nghìn — trên ngưỡng, nên nó phải đi ra.

Chữ "xăng" đã có sẵn trong danh sách rồi cũng không ngăn được nó: thứ **không**
nhận hai lần là cái rổ ở bài 26, còn danh sách thì nhận đủ mọi lần xuất hiện,
đúng như bài trước đã chỉ ra khi lấy trường `nhom`.
::
:::
::::

::::code{#loc-khoan-tren-mot-tram}
Byte cần một danh sách tên của những khoản tiêu **quá** 100 nghìn, giữ nguyên thứ
tự ghi trong sổ. Vẫn cuốn sổ tám khoản vừa rồi, với hai khoản nằm sát ngưỡng —
một điều kiện đặt hớ tay sẽ nhận nhầm khoản này hoặc bỏ sót khoản kia.

Con số ngưỡng đã được đặt sẵn vào một cái tên `nguong` ngay phía trên — tháng
sau Byte đổi ngưỡng thì chỉ phải sửa đúng dòng đó. Chỗ trống vì vậy chỉ còn phải
nói phần so sánh.

Điền điều kiện vào chỗ trống ở cuối dòng.

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

dat = [khoan["ten"] for khoan in so if ___]

print(dat)
print(f"Có {len(dat)} khoản trên 100 nghìn")
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

dat = [khoan["ten"] for khoan in so if khoan["tien"] > nguong]

print(dat)
print(f"Có {len(dat)} khoản trên 100 nghìn")
```

```python title=test
# So BẰNG cả danh sách chứ không kiểm mỗi số lượng: thứ tự cũng là một phần của
# câu trả lời, và hai khoản nằm ở hai phía con số ngưỡng sẽ tố giác ngay một
# điều kiện đặt hớ tay.
assert dat == ["xăng", "ăn trưa", "sửa xe", "học phí", "xăng"], "cuốn sổ tám khoản này cho ra năm cái tên theo đúng thứ tự ghi, và 'xăng' có mặt hai lần vì cả hai lần đổ xăng đều trên 100 nghìn — nếu danh sách của bạn có thêm 'vá lốp' thì điều kiện đang nhận cả khoản tròn 100 nghìn, còn nếu thiếu 'xăng' ở cuối thì ngưỡng đang bị đặt cao hơn 105 nghìn"
assert len(dat) == 5, "trong tám khoản của cuốn sổ này có đúng năm khoản vượt 100 nghìn — ba khoản không vượt là cà phê 90 nghìn, bánh mì 15 nghìn và vá lốp tròn 100 nghìn"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ở cuối dòng, ngay sau chữ `if` và trước dấu ngoặc vuông đóng. Ở chỗ đó máy đang cầm trong tay đúng **một** khoản của lượt này, và cái tên gọi nó là `khoan`.
- kind: strategy
  body: Câu cần hỏi ở mỗi khoản là "khoản này có tiêu quá ngưỡng không". Số tiền nằm trong trường `tien` của khoản, lấy ra bằng đúng cặp ngoặc vuông thứ hai mà bài 24 đã dạy; còn con số đem so thì đã có tên sẵn ở dòng `nguong`. Chú ý chữ **quá** trong đề: khoản tròn 100 nghìn thì chưa quá, nên dấu so sánh phải là dấu nghiêm ngặt chứ không phải dấu có gạch dưới.
- kind: one-line
  body: 'Viết `khoan["tien"] > nguong` vào chỗ trống, không thêm dấu hai chấm nào.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^\['xăng', 'ăn trưa', 'sửa xe', 'học phí', 'xăng'\]\nCó 5 khoản trên 100 nghìn\s*$
- tier: output
  expect: Có 5 khoản trên 100 nghìn
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một dòng: đầu dòng nói lấy gì, đuôi dòng nói lấy của ai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ra được một list. Nhưng thứ mình cần là một **sổ tra cứu** tên → tiền, tra bằng
khoá: gõ `tra_cuu["sửa xe"]` là ra ngay số tiền, không phải dò lại cả danh sách.

Danh sách vừa dựng chỉ có tên, mất sạch phần tiền. Mà cái khuôn thì đã rất gần
thứ mình muốn: `for` ở giữa, `if` ở cuối, chỉ thiếu chỗ để đặt khoá và giá trị.

Đổi ngoặc vuông thành ngoặc nhọn thì máy cho ra dict?
::::

::::checkpoint{mastery=0.8}
::::
