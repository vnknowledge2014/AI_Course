---
id: nen-tang.list-dict-set-tuple.may-nhin-o-dau-truoc
title: Máy nhìn ô nào trước
summary: So hai cặp, máy nhìn ô đầu trước; hoà rồi mới xét tới ô sau — nên một danh sách cặp bị xếp theo tên.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.tuple-compare]
requires: [core.sorted, core.tuple, core.tuple-immutable, core.tuple-unpack, core.for-unpack, core.dict, core.dict-items, core.slice-copy, core.list, core.string-sequence, ctrl.comparison, ctrl.for-each, core.variable, core.assignment, core.fstring, core.output]
practices: [core.sorted, core.for-unpack, core.dict-items, ctrl.comparison, core.fstring]
concepts: [core.danh-sach, core.so-sanh, core.gia-tri]
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
Muốn xếp thì mình phải so hai thứ với nhau. Và mình so từ ô đầu tiên.
::::

::::explain{#chay-thu-tren-danh-sach-cap}
Câu hỏi bỏ ngỏ của bài trước: đem `sorted` chạy thẳng trên danh sách các cặp
`(tên, tiền)` thì nó xếp theo tên hay theo tiền?

Chạy thử. `chi.items()` cho ra từng cặp, và `list(...)` gói chúng lại thành một
danh sách thật — đúng cách viết mà bài 5 đã nhắc tới khi lấy ra một cuốn sổ
riêng. Thứ tự trong đó vẫn là thứ tự ghi sổ.

```python title=readonly
chi = {
    "sửa xe": 500000,
    "cà phê": 25000,
    "biếu bà": 300000,
    "xăng xe": 60000,
    "mua sách": 40000,
}

cac_cap = list(chi.items())
da_xep = sorted(cac_cap)

for ten, tien in da_xep:
    print(f"{ten}: {tien} đồng")
```

Máy in ra:

```text title=readonly
biếu bà: 300000 đồng
cà phê: 25000 đồng
mua sách: 40000 đồng
sửa xe: 500000 đồng
xăng xe: 60000 đồng
```

Nó xếp **theo tên**: b, c, m, s, x. Cột tiền nhảy lung tung — 300 nghìn rồi 25
nghìn rồi 40 nghìn — nghĩa là tiền không hề tham gia vào việc quyết định thứ tự.

Kết quả này không phải một sự tuỳ tiện của máy. Nó là hệ quả của một luật rất
gọn, và biết luật ấy thì bạn đoán trước được mọi lần xếp cặp về sau.
::::

::::explain{#so-hai-cap-thi-so-o-nao}
Muốn xếp một hàng, máy phải làm được một việc nhỏ hơn nhiều: cầm **hai** thứ lên
và trả lời "thứ nào đứng trước". Xếp cả hàng là làm việc nhỏ ấy rất nhiều lần.

Với hai con số thì câu trả lời có sẵn: số nhỏ đứng trước. Còn hai **cặp** thì
mỗi cặp có tới hai ô. Máy làm thế này:

1. Nhìn **ô đầu** của cả hai cặp. Khác nhau thì xong — ô đầu quyết định, ô sau
   không ai đọc tới.
2. Hai ô đầu bằng nhau thì mới xét tiếp **ô thứ hai**.

Giống hệt cách người ta dò một cuốn danh bạ: so họ trước; hai người trùng họ thì
mới đọc tới tên.

Trong một cặp `(tên, tiền)`, ô đầu là **tên**. Nên lần nào máy cũng đi so tên
trước, và năm cái tên trong sổ thì khác nhau ngay từ đầu — không lần nào phải
xét tới ô tiền. Đó là toàn bộ lý do bản báo cáo vừa rồi xếp theo tên.

Nhìn luật ấy chạy trên hai cặp lẻ:

```python title=readonly
print(("cà phê", 40000) < ("bún bò", 55000))
print(("cà phê", 25000) < ("cà phê", 40000))
```

```text title=readonly
False
True
```

- Dòng đầu: hai ô đầu khác nhau — `"cà phê"` với `"bún bò"`. Máy so hai chuỗi ấy
  theo từng ký tự, và ký tự đầu là `c` với `b`. `b` đứng trước, nên cặp
  `"bún bò"` đứng trước, nên câu hỏi "cà phê có đứng trước không" cho `False`.
  Hai con số 40000 và 55000 nằm đó nhưng không ai đọc tới chúng.
- Dòng thứ hai: hai ô đầu **giống hệt**. Lúc này máy mới xét tới ô thứ hai, và ở
  đó 25000 nhỏ hơn 40000. Nên `True`.

> Một chỗ cần nói thẳng: năm cái tên trong cuốn sổ này khác nhau ngay ở ký tự
> đầu — b, c, m, s, x — nên thứ tự máy cho ra trùng với thứ tự bảng chữ cái mà
> bạn quen. Với những ký tự có dấu thì máy có bảng xếp riêng của nó, không phải
> bảng chữ cái tiếng Việt. Sổ trong bài cố ý chọn năm cái tên khác nhau ngay từ
> chữ đầu để chuyện đó không xen vào.
::::

::::predict{#hai-cap-trung-ten commitOnce}
Byte ghi ba lần mua trong một ngày, và hai trong ba lần trùng tên.

**Trước khi bấm chạy**, bạn đoán máy in ra danh sách nào?

```python title=readonly
mua = [("cà phê", 40000), ("bún bò", 55000), ("cà phê", 25000)]

print(sorted(mua))
```

:::opt{correct}
[('bún bò', 55000), ('cà phê', 25000), ('cà phê', 40000)]
:::

:::opt
[('cà phê', 25000), ('cà phê', 40000), ('bún bò', 55000)]
::why
Gần đúng ở chỗ bạn đọc rất kỹ ô tiền: 25000, rồi 40000, rồi 55000 — đúng là một
hàng xếp từ nhỏ tới lớn, không sai một chỗ nào.

Chỗ lệch là ở ô mà máy nhìn vào. Bạn xếp theo ô **thứ hai**, còn máy bắt đầu từ
ô **đầu**. Ô đầu của ba cặp này là `"cà phê"`, `"bún bò"`, `"cà phê"` — chúng
không bằng nhau hết, nên máy giải quyết xong ngay tại đó và không đọc tới tiền,
trừ đúng một chỗ: hai cặp cùng tên `"cà phê"`.
::
:::

:::opt
[('bún bò', 55000), ('cà phê', 40000), ('cà phê', 25000)]
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra ô đầu quyết định trước, nên `"bún bò"` lên
đầu và hai cặp `"cà phê"` xuống dưới. Nửa đầu bài này bạn đọc chính xác.

Chỗ lệch nằm ở hai cặp trùng tên. Bạn cho rằng khi ô đầu hoà thì máy giữ nguyên
thứ tự cũ — nghe rất hợp lý, vì cặp `("cà phê", 40000)` được ghi vào trước.
Nhưng hoà ở ô đầu không phải là hết chuyện: máy đi tiếp sang ô thứ hai và so
25000 với 40000. Còn một ô nữa để so thì nó so, chứ không bỏ dở.
::
:::

:::opt
Máy dừng và báo `TypeError`, vì không so được một cái tên với một con số
::why
Gần đúng ở chỗ bạn nhớ một luật có thật và nhớ đúng: đem một chuỗi ra so lớn bé
với một con số thì Python từ chối, y như chuyện cộng chuỗi với số. Máy thật sự
không làm việc đó.

Chỗ lệch: máy không bao giờ đặt tên cạnh tiền để so. Nó so **ô đầu với ô đầu**
(tên với tên) rồi mới tới **ô sau với ô sau** (tiền với tiền) — luôn luôn cùng
loại với cùng loại. Ba cặp ở đây đều là `(chuỗi, số)` nên không có phép so nào
lệch loại xảy ra.
::
:::
::::

::::code{#hoi-thang-may-cap-nao-truoc}
Ba cặp của bài đoán vừa rồi, giờ đặt tên hẳn hoi để hỏi thẳng máy.

Dấu `<` bạn đã dùng cho số từ Realm 0; đem nguyên nó ra dùng cho **cặp** cũng
được, và câu hỏi nó trả lời là: *cặp bên trái có đứng trước cặp bên phải không.*

Hãy điền hai chỗ trống bằng hai phép so sánh. Hai cặp trong dòng đầu khác tên
nhau; hai cặp trong dòng sau trùng tên — nên một phép cho `False`, một phép cho
`True`.

```python title=starter
mot = ("cà phê", 40000)
hai = ("bún bò", 55000)
ba = ("cà phê", 25000)

mot_truoc_hai = ___
ba_truoc_mot = ___

print(f"cặp một đứng trước cặp hai: {mot_truoc_hai}")
print(f"cặp ba đứng trước cặp một: {ba_truoc_mot}")
```

```python title=solution
mot = ("cà phê", 40000)
hai = ("bún bò", 55000)
ba = ("cà phê", 25000)

mot_truoc_hai = mot < hai
ba_truoc_mot = ba < mot

print(f"cặp một đứng trước cặp hai: {mot_truoc_hai}")
print(f"cặp ba đứng trước cặp một: {ba_truoc_mot}")
```

```python title=test
# Hai ca cố ý ngược nhau: một ca dừng ở ô đầu, một ca phải đi tiếp sang ô sau.
# Điền cùng một thứ vào cả hai chỗ thì luôn có một câu vỡ.
assert mot_truoc_hai == False, 'cặp một mở đầu bằng "cà phê", cặp hai bằng "bún bò" — máy so hai cái tên ấy trước, mà b đứng trước c, nên cặp một KHÔNG đứng trước cặp hai'
assert ba_truoc_mot == True, 'cặp ba và cặp một cùng mở đầu bằng "cà phê", nên máy mới xét tới ô tiền — 25000 nhỏ hơn 40000, cặp ba đứng trước'
```

:::hints
- kind: attention
  body: Hai dòng cần điền đều đang hứng một câu trả lời **đúng hay sai** — cứ nhìn hai dòng `print` bên dưới là thấy chúng in ra `True` hay `False`. Tên biến nói sẵn thứ tự hai vế: `mot_truoc_hai` hỏi về cặp `mot` so với cặp `hai`.
- kind: strategy
  body: Câu hỏi "cái này có đứng trước cái kia không" chính là dấu bé hơn mà bạn đã dùng cho số, đặt cặp bên trái ở trước dấu và cặp bên phải ở sau. Đừng gõ thẳng `True` hay `False` vào — bài này muốn chính máy trả lời, để bạn đối chiếu với điều mình vừa đoán.
- kind: one-line
  body: Viết `mot < hai` vào chỗ trống thứ nhất, và `ba < mot` vào chỗ trống thứ hai.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^cặp một đứng trước cặp hai: False\ncặp ba đứng trước cặp một: True\s*$
- tier: output
  expect: 'cặp một đứng trước cặp hai: False'
- tier: static
  onFail: hai dòng này phải để máy so hai cặp, không phải gõ thẳng True/False vào
  requireAst:
  # `mot` bị đọc hai lần trong lời giải (một lần ở mỗi phép so). Hỏi `uses-name`
  # chứ không hỏi riêng dấu `<`: người viết `hai > mot` cũng đang trả lời đúng
  # câu hỏi ấy, và luật chấm không nên đánh trượt một lời giải đúng.
  - kind: uses-name, target: mot, min: 2
  - kind: uses-name, target: ba, min: 1
  forbidAst:
  - kind: has-literal, target: 'True'
  - kind: has-literal, target: 'False'
:::
::::

::::byte{trigger=success mood=happy pose=jump}
So hai cặp thì ô đầu quyết định. Hoà, mình mới xét tới ô sau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luật đã rõ, và nó giải thích trọn vẹn bản báo cáo xếp theo tên. Nhưng thứ Byte
cần lại là xếp theo **tiền**.

Đường vòng thì có: cắt danh sách thành toàn tiền như bài 19 rồi xếp. Chỉ có điều
bạn đã thấy cái giá của nó — hàng số đã xếp nói được "năm trăm nghìn", mà không
nói được năm trăm nghìn ấy là khoản gì.

Vậy phải giữ nguyên cặp, và bảo `sorted` đừng nhìn ô đầu nữa mà nhìn **ô thứ
hai**. Nói với nó câu đó bằng cách nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
