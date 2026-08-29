---
id: nen-tang.list-dict-set-tuple.ngoac-tron-la-gi
title: Thứ nằm trong ngoặc tròn
summary: Cặp mà `.items()` đưa ra có tên là `tuple` — một dãy có thứ tự, viết trong `( )`, đọc từng ô bằng chỉ số y như danh sách.
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.tuple]
practices: [core.list-index, core.type-fn, core.fstring, ctrl.for-each]
requires: [core.dict, core.dict-items, core.list, core.list-index, core.type-fn, core.return-multiple, core.string-sequence, ctrl.for-each, core.fstring]
concepts: [core.gia-tri, core.danh-sach, core.bien]
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
Cặp ngoặc tròn ấy mình từng đưa cho bạn một lần rồi. Ở mạch Hàm.
::::

::::explain{#tam-the-hai-o}
Bài trước, `.items()` chịu đưa cả tên lẫn tiền một lượt. In thử một cặp ra thì
màn hình hiện đúng dòng này:

```text
('sửa xe', 500000)
```

Ngoặc **tròn**. Không phải `[ ]` của danh sách, cũng không phải `{ }` của sổ tra
cứu. Một hình dạng thứ ba, chưa bài nào trong mạch này gọi tên.

Nhưng bạn đã nhìn thấy nó rồi — ở mạch Hàm, bài hàm đưa ra hai con số. Lúc ấy
`return tien_hang, tien_thue` cho ra một **cái gói**, và in cái gói lên màn hình
thì máy vẽ nó ra bằng đúng cặp ngoặc tròn ấy. Hỏi `type()` thì máy đáp
`<class 'tuple'>`.

Vậy thứ `.items()` đưa ra và cái gói của `return` là **cùng một loại**. Nó có
tên: **tuple**.

Điều mạch Hàm chưa nói tới, và là việc của bài này: cái gói ấy không phải đặc
sản riêng của `return`. Nó là một chỗ chứa đàng hoàng, và bạn **tự viết ra
được**, không cần hàm nào cả.

Nghĩ về tấm thẻ giá treo trên kệ hàng. Trên thẻ in sẵn hai ô: ô trên ghi tên
món, ô dưới ghi số tiền. Hai ô ấy nằm trên **một** tấm thẻ, theo **một** thứ
tự đã in — ô tên luôn ở trên, ô tiền luôn ở dưới. Cầm tấm thẻ là cầm cả hai ô
cùng lúc.

Trong Python, tấm thẻ ấy viết bằng dấu ngoặc tròn, các ô cách nhau bằng dấu
phẩy:

```python
cap = ("sửa xe", 500000)
```

Và đọc từng ô thì y hệt cách bạn đọc một danh sách: một con số trong ngoặc
vuông, đếm từ 0.

```python
print(cap[0])
print(cap[1])
```

Nói gọn lại một câu, và đây là câu duy nhất của bài này: **tuple là một dãy có
thứ tự, viết trong `( )`, tra bằng chỉ số y như list.**
::::

::::example{#tu-tay-viet-mot-cap}
Viết một cặp bằng tay, rồi hỏi máy xem nó là gì và bên trong có gì:

```python title=readonly
cap = ("sửa xe", 500000)

print(cap)
print(type(cap))
print(cap[0])
print(cap[1])

chi = {"sửa xe": 500000, "cà phê": 25000}
for mot_cap in chi.items():
    print(mot_cap[0])
```

Máy in ra:

```text
('sửa xe', 500000)
<class 'tuple'>
sửa xe
500000
sửa xe
cà phê
```

Sáu dòng ấy nói ra ba chuyện:

- **Dòng đầu và dòng hai.** Cái tên `cap` đang giữ **một** giá trị, không phải
  hai. Giá trị ấy có kiểu riêng, tên là `tuple` — đứng ngang hàng với `int`,
  `str`, `list` mà bạn đã hỏi `type()` từ Realm 0. Cặp ngoặc tròn là cách máy
  vẽ nó ra khi in.
- **Dòng ba và dòng bốn.** `cap[0]` đưa ra chuỗi `sửa xe`, `cap[1]` đưa ra số
  `500000`. Đúng lối đọc danh sách: ô đầu tiên mang chỉ số 0. Để ý chuyện in
  ra không còn dấu nháy quanh `sửa xe` — vì lần này máy in **một chuỗi**, chứ
  không in cả cái gói.
- **Hai dòng cuối.** Cái mà `.items()` thả ra mỗi lượt cũng đọc được y như vậy.
  Vòng lặp đặt tên nó là `mot_cap`, và `mot_cap[0]` lấy ra tên khoản của lượt
  ấy.

Thứ tự các ô là thứ tự bạn viết ra lúc dựng cặp. `("sửa xe", 500000)` thì ô 0
là tên, ô 1 là tiền. Viết ngược lại thì hai chỉ số đổi vai, và máy vẫn chạy
bình thường — nó không biết ô nào đáng lẽ phải là tên.
::::

::::predict{#doan-hai-dong commitOnce}
Byte dựng một cặp mới cho khoản cà phê, in cả cặp rồi in ô đầu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
cap = ("cà phê", 25000)

print(cap)
print(cap[0])
```

:::opt{correct}
`('cà phê', 25000)` rồi `cà phê`
:::

:::opt
`('cà phê', 25000)` rồi `c`
::why
Gần đúng ở chỗ bạn nhớ một chuyện có thật và rất đáng nhớ từ mạch trước: với
một **chuỗi**, `ten[0]` lấy ra ký tự đầu tiên, nên `"cà phê"[0]` cho ra `c`.
Suy luận ấy chính xác — chỉ là nó đang áp vào nhầm thứ.

Chỗ lệch nằm ở thứ đứng trước ngoặc vuông. Ở đây `cap` không phải chuỗi; nó là
cái gói có hai ô, và ô số 0 của nó chứa **trọn** chuỗi `"cà phê"`. Muốn tới
từng ký tự thì phải bóc thêm một tầng nữa — hỏi ô trước, rồi mới hỏi ký tự
trong cái ô ấy.
::
:::

:::opt
`cà phê 25000` rồi `cà phê`
::why
Gần đúng ở chỗ bạn để ý một hành vi thật của `print`: in một chuỗi thì máy
không kèm dấu nháy, `print("cà phê")` cho ra `cà phê` trơn. Bạn đang mang đúng
hành vi ấy sang.

Chỗ lệch: lần này thứ đem đi in không phải một chuỗi mà là **cả cái gói**. Khi
in một cái gói, máy phải vẽ ra hình dạng của nó cho người đọc thấy có mấy ô và
mỗi ô là gì — nên nó viết đủ ngoặc tròn, dấu phẩy, và dấu nháy quanh ô nào là
chuỗi. Dấu nháy chỉ biến mất khi bạn lấy riêng ô ấy ra, như dòng thứ hai.
::
:::

:::opt
`('cà phê', 25000)` rồi `('cà phê')`
::why
Gần đúng ở chỗ bạn phân biệt được hai chuyện khác nhau thật: **lấy một ô** và
**lấy một đoạn**. Lấy một đoạn của danh sách thì đúng là cho về một chỗ chứa
nhỏ hơn chứ không phải một giá trị lẻ — bạn đã gặp chuyện đó ở bài lát cắt.

Chỗ lệch là ở cách viết. Lát cắt cần một dấu hai chấm giữa hai đầu; ở đây trong
ngoặc vuông chỉ có **một con số trơn**, và một con số trơn là **chỉ số**. Chỉ
số không cắt ra khúc nào cả — nó chỉ thẳng vào một ô và đưa ra chính giá trị
nằm trong ô đó.
::
:::
::::

::::code{#doc-hai-o-cua-cap}
Byte đưa bạn đúng một tấm thẻ và muốn in nó thành hai dòng báo cáo cho dễ đọc.

Hai chỗ trống, mỗi chỗ lấy ra một ô của cặp. Ô nào ra tên, ô nào ra tiền là
việc bạn quyết định bằng chỉ số.

```python title=starter
cap = ("sửa xe", 500000)

ten = ___
tien = ___

print(f"Khoản: {ten}")
print(f"Số tiền: {tien} đồng")
```

```python title=solution
cap = ("sửa xe", 500000)

ten = cap[0]
tien = cap[1]

print(f"Khoản: {ten}")
print(f"Số tiền: {tien} đồng")
```

```python title=test
# Mỗi chỗ trống có một câu canh riêng: điền nhầm chỉ số thì cả hai cùng vỡ,
# vì lúc đó `ten` đang giữ con số và `tien` đang giữ chuỗi.
assert ten == "sửa xe", "cặp ('sửa xe', 500000) có ô 0 là tên khoản — sau hai dòng điền, `ten` phải đang giữ chuỗi 'sửa xe'"
assert tien == 500000, "cặp ('sửa xe', 500000) có ô 1 là số tiền — sau hai dòng điền, `tien` phải đang giữ số 500000"
```

:::hints
- kind: attention
  body: Nhìn lại dòng dựng cặp. Trong ngoặc tròn có hai thứ, cách nhau một dấu phẩy, và chúng có thứ tự — thứ nào được viết ra trước?
- kind: strategy
  body: Cách đọc một ô của cặp giống hệt cách đọc một món trong danh sách ở Realm 0 — tên của chỗ chứa, rồi một con số trong ngoặc vuông. Ô đầu tiên mang số 0, nên ô thứ hai mang số 1. Cả hai chỗ trống đều lấy từ cùng cái tên `cap`, chỉ khác con số trong ngoặc.
- kind: one-line
  body: "Viết `cap[0]` vào chỗ trống thứ nhất và `cap[1]` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Khoản: sửa xe\nSố tiền: 500000 đồng\s*$
- tier: static
  onFail: hai chỗ trống phải LẤY RA từ chính cặp `cap`, không gõ lại giá trị bằng tay
  requireAst:
  # `min: 2` vì lời giải đọc `cap` đúng hai lần — `cap[0]` và `cap[1]`. Dòng
  # dựng cặp KHÔNG được tính: ở đó `cap` đang được đặt tên chứ chưa dùng tới.
  # Nhờ vậy đáp án gõ thẳng `"sửa xe"` vào chỗ trống sẽ không thoả.
  - kind: uses-name, target: cap, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tấm thẻ có hai ô, và bạn gọi đúng tên từng ô rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả bài này dựa trên một câu: tuple tra bằng chỉ số **y như list**. Câu ấy đúng
với việc **đọc** — `cap[0]`, `cap[1]` đều chạy, bạn vừa thấy tận mắt.

Nhưng ở Realm 0, danh sách còn làm được một việc nữa ngoài đọc: nó **sửa được**.
`.append` gắn thêm món vào cuối một danh sách đã có, và chính cái danh sách ấy
dài ra.

Vậy thử tới nốt chỗ đó. Quán sửa xe tính lại, khoản kia chỉ hết 450 nghìn thôi.
Bạn đang cầm `cap = ("sửa xe", 500000)`, và bạn gõ:

```python
cap[1] = 450000
```

Máy sẽ nói gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
