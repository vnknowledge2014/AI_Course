---
id: nen-tang.list-dict-set-tuple.co-trong-so-khong
title: Có nằm trong sổ không
summary: "`gia_tri in danh_sach` hỏi thẳng một câu và nhận về `True`/`False` — thay cho bốn dòng cờ, `for` và `if` tự dựng."
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.list-membership]
requires: [core.list, core.list-index, core.list-remove, core.len, core.flag-variable, core.boolean, core.value-error, ctrl.for-each, ctrl.if, ctrl.else, ctrl.comparison, core.fstring, core.output]
concepts: [core.danh-sach, core.dung-sai, ctrl.tim-kiem]
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
Câu "sổ có khoản này không" — mình trả lời được bằng đúng một chữ.
::::

::::explain{#bon-dong-cho-mot-cau-hoi}
Bài trước để lại một chỗ đau cụ thể. `.remove(500000)` xoá theo nội dung, và
nếu cuốn sổ chưa từng ghi khoản ấy thì máy ném `ValueError` — chương trình dừng
giữa chừng, những dòng phía sau không bao giờ chạy.

Cách chữa thì rõ: hỏi trước, xoá sau. Câu hỏi là *"sổ có khoản 500000 không"*.

Với những gì bạn đang có, câu hỏi ấy viết ra thành thế này:

```python title=readonly
so = [85000, 240000, 500000, 120000]

co_khoan = False
for khoan in so:
    if khoan == 500000:
        co_khoan = True

if co_khoan:
    so.remove(500000)
    print("Đã xoá khoản 500000 đồng")
else:
    print("Sổ không có khoản 500000 đồng")
```

Đoạn ấy chạy đúng. Nhưng đếm lại xem: bốn dòng — một biến cờ, một vòng lặp, một
phép so sánh, một phép gán — chỉ để lấy về **một** câu trả lời có hoặc không. Và
lần sau muốn hỏi về khoản 300000, bạn chép lại cả bốn dòng ấy.

Python có sẵn đúng câu hỏi đó, viết thành một mẩu:

```python title=readonly
500000 in so
```

Đọc thẳng như tiếng Việt: *"500000 nằm trong so"*. Mẩu ấy cho ra một giá trị, và
giá trị đó chỉ có hai khả năng: `True` khi sổ có, `False` khi sổ không có.

Đúng hai giá trị mà Realm 0 đã đưa cho bạn qua các phép so sánh như `>` và `==`.
Nghĩa là `if` nuốt được nó ngay, không cần biến trung gian nào.
::::

::::explain{#may-tra-loi-bang-cach-nao}
Một câu hỏi ngắn không có nghĩa là máy làm ít việc hơn.

Để trả lời `500000 in so` trên một **danh sách**, máy làm đúng thứ vòng lặp của
bạn vừa làm: đứng ở ô đầu tiên, so ô ấy với 500000; chưa khớp thì bước sang ô kế;
cứ thế đi tới.

- Gặp một ô bằng đúng giá trị đang hỏi thì máy **dừng ngay tại đó** và cho ra
  `True` — nó không cần xem những ô còn lại.
- Không ô nào khớp thì máy chỉ kết luận `False` sau khi đã đi qua **cả** cuốn sổ.
  Muốn nói "không có" thì phải xem hết, không có đường tắt.

Với cuốn sổ bốn khoản thì chuyện đó không đáng kể. Cứ giữ lấy sự thật này đã —
"trên một danh sách, `in` phải dò từng ô" — vì cuối track bạn sẽ gặp một chỗ chứa
trả lời cùng câu hỏi ấy theo kiểu khác hẳn, và lúc đó câu này là thứ để đối chiếu.

Câu hỏi ngược cũng viết bằng đúng phép ấy, chỉ thêm một chữ: `999000 not in so`
đọc là *"999000 không nằm trong so"*, và nó cho ra `True` đúng ở những lúc
`999000 in so` cho ra `False`. Vẫn một phép hỏi, vẫn một lượt dò, chỉ đảo câu trả
lời.

Còn một chuyện nữa về cách máy so: nó so **trọn ô với trọn giá trị**, không so
một phần. Hỏi `85 in so` trên cuốn sổ trên thì câu trả lời là `False`, dù trong sổ
có khoản 85000 và hai con số nhìn có vẻ dính dáng nhau. Với máy, 85 và 85000 là
hai giá trị khác nhau, thế thôi.
::::

::::example{#hoi-thang-mot-cau}
Cùng cuốn sổ ấy, hỏi thẳng và in câu trả lời ra xem mặt mũi nó thế nào:

```python title=readonly
so = [85000, 240000, 500000, 120000]

print(500000 in so)
print(999000 in so)

if 500000 in so:
    so.remove(500000)

print(so)
```

Máy in ra:

```text title=readonly
True
False
[85000, 240000, 120000]
```

Ba chỗ đáng dừng lại nhìn:

- **Hai dòng đầu in ra `True` và `False`** — không phải chỗ đứng của khoản tiền,
  không phải chính khoản tiền. `in` trả lời một câu hỏi có/không, nên thứ nó đưa
  ra là một giá trị đúng/sai.
- **Dòng `if 500000 in so:` không cần dấu `== True`.** Mẩu `500000 in so` tự nó
  đã là một giá trị đúng/sai rồi, viết thêm phép so sánh nữa chỉ là hỏi lại một
  câu đã có câu trả lời.
- **Cuốn sổ cuối cùng còn ba khoản.** Cái cổng `if` đứng trước đã bảo đảm
  `.remove` chỉ chạy khi có thứ để xoá, nên không có `ValueError` nào.
::::

::::predict{#doan-in-ra-gi commitOnce}
Vẫn cuốn sổ bốn khoản, và lần này Byte hỏi về khoản 120000 — khoản đứng cuối sổ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
so = [85000, 240000, 500000, 120000]

print(120000 in so)
```

:::opt{correct}
True
:::

:::opt
3
::why
Gần đúng ở chỗ con số 3 có thật và bạn đếm không sai: sổ đếm chỗ đứng từ 0, nên
khoản 120000 đứng ở ô số 3. Sáu bài vừa rồi toàn nói về chỗ đứng, nên đây là chỗ
mắt bạn nhìn tới trước là chuyện tự nhiên.

Chỗ lệch nằm ở câu hỏi mà `in` đặt ra. Nó hỏi *có hay không*, chứ không hỏi *ở
đâu*. Thử nghĩ tới ca ngược lại: nếu sổ **không** có khoản đang hỏi thì một câu
trả lời dạng chỗ đứng sẽ phải đưa ra con số nào? Không có con số nào đúng cả.
`True` và `False` thì che được cả hai ca bằng cùng một hình dạng.
::
:::

:::opt
120000
::why
Gần đúng ở chỗ bạn đọc `in` như một phép lấy ra: hỏi về 120000 và nhận lại
120000, nghe như một lời xác nhận.

Chỗ lệch: câu trả lời ấy không nói thêm điều gì bạn chưa biết — chính bạn vừa gõ
con số đó ra. Và nếu sổ không có khoản đang hỏi thì máy lấy gì để đưa lại? `in`
sinh ra để trả lời cả hai ca, nên nó cho ra thứ chỉ có hai trạng thái: `True` khi
tìm thấy, `False` khi không.
::
:::

:::opt
Máy báo lỗi, vì `in` chỉ dùng được trong dòng `for`
::why
Gần đúng ở chỗ bạn nhớ rất chính xác chỗ mình đã gặp chữ `in` từ Realm 0 tới giờ:
`for khoan in so`. Ở đó nó nghĩa là "lấy lần lượt từng ô ra", và đúng là cho tới
bài này bạn chưa gặp nó ở chỗ nào khác.

Chỗ lệch: cùng một chữ, hai công việc, và máy phân biệt hai công việc ấy bằng
chỗ chữ ấy đứng. Đứng sau `for` cùng một cái tên thì nó là lệnh đi dạo qua từng
ô. Đứng một mình giữa hai giá trị, không có `for` nào phía trước, thì nó là một
câu hỏi — và câu hỏi thì có câu trả lời, `True` hoặc `False`.
::
:::
::::

::::code{#hoi-truoc-xoa-sau}
Byte đưa cuốn sổ bốn khoản, kèm một danh sách những khoản cần xoá. Ba khoản trong
danh sách ấy: có khoản nằm trong sổ thật, có khoản Byte nhớ nhầm và sổ chưa từng
ghi.

Mỗi lượt, chương trình phải hỏi trước rồi mới quyết định: xoá được thì xoá và báo
lại, không có thì nói ra là không có — và **không** được để `ValueError` làm
chương trình dừng giữa chừng.

Hãy điền điều kiện còn thiếu vào chỗ trống.

```python title=starter
so = [85000, 240000, 500000, 120000]
can_xoa = [500000, 999000, 85000]

for khoan in can_xoa:
    if ___:
        so.remove(khoan)
        print(f"Đã xoá khoản {khoan} đồng")
    else:
        print(f"Sổ không có khoản {khoan} đồng")

print(f"Sổ còn: {so}")
```

```python title=solution
so = [85000, 240000, 500000, 120000]
can_xoa = [500000, 999000, 85000]

for khoan in can_xoa:
    if khoan in so:
        so.remove(khoan)
        print(f"Đã xoá khoản {khoan} đồng")
    else:
        print(f"Sổ không có khoản {khoan} đồng")

print(f"Sổ còn: {so}")
```

```python title=test
# Ba lượt hỏi, và cuốn sổ đổi sau mỗi lượt xoá được. Một điều kiện lúc nào
# cũng đúng sẽ nổ ở lượt 999000; một điều kiện lúc nào cũng sai thì không xoá
# nổi khoản nào, và cuốn sổ vẫn còn nguyên bốn khoản.
assert so == [240000, 120000], "sổ mở đầu có bốn khoản: 500000 và 85000 nằm trong sổ nên xoá được, còn 999000 thì sổ chưa từng ghi — cuối cùng sổ phải còn đúng 240000 và 120000, giữ nguyên thứ tự cũ"
assert can_xoa == [500000, 999000, 85000], "danh sách cần xoá chỉ để đọc: sau khi chạy nó vẫn phải còn đủ ba khoản 500000, 999000 và 85000"
```

:::hints
- kind: attention
  body: Chỗ trống là điều kiện của `if`, nên thứ điền vào phải là một câu hỏi có câu trả lời đúng/sai. Câu hỏi ấy nhắc tới hai thứ, và cả hai đều đã có tên sẵn trong đoạn: khoản đang xét ở lượt này, và cuốn sổ.
- kind: strategy
  body: Mỗi lượt bạn hỏi đúng một câu — "cuốn sổ có khoản này không". Viết nó bằng chữ `in`: bên trái là giá trị đem đi hỏi, bên phải là danh sách bị hỏi. Giá trị đem đi hỏi đổi theo từng lượt, nên đừng gõ một con số cố định vào đó.
- kind: one-line
  body: "Viết `khoan in so` vào chỗ trống, giữ nguyên dấu hai chấm ở cuối dòng."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: static
  onFail: câu hỏi của `if` phải hỏi chính cuốn sổ xem có khoản này không, chứ không so khoản đang xét với một con số gõ sẵn
  requireAst:
  # Hỏi THẲNG chữ `in`, không lách bằng cách đếm số lần đọc tên `so`.
  #
  # Bản trước phải đếm tên vì bảng toán tử của `kiem-ast.ts` chưa có `in` —
  # mà một con số như thế ăn theo số lệnh `print` trong khung, nên sửa khung
  # là luật chấm hỏng lặng lẽ. Nay bảng đã có `in`, `not in`, `is`, `is not`.
  #
  # Không có luật này thì `khoan != 999000` qua sạch cả bốn tầng — chạy được,
  # hai assert đạt, regex khớp từng chữ — ở đúng bài mà `in` là khái niệm mới
  # duy nhất. Người học không gõ chữ `in` lần nào vẫn được khen.
  - kind: uses-operator, target: in, min: 1
- tier: output
  match: regex
  expect: ^Đã xoá khoản 500000 đồng\nSổ không có khoản 999000 đồng\nĐã xoá khoản 85000 đồng\nSổ còn: \[240000, 120000\]\s*$
- tier: output
  expect: Sổ không có khoản 999000 đồng
- tier: output
  expect: "Sổ còn: [240000, 120000]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hỏi một câu trước, khỏi phải đỡ một cú `ValueError` sau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`in` nói với bạn: "sổ có 500000". Câu trả lời ấy đúng, và nó vừa cứu chương trình
khỏi dừng giữa chừng.

Nhưng ngồi nhìn lâu hơn một chút thì nó hơi trống. 500000 là tiền gì — sửa xe hay
biếu bà? Hai khoản ấy khác nhau một trời một vực với người giữ sổ, mà trong sổ
chúng chỉ là cùng một dãy chữ số.

Cuốn sổ của bạn từ bài 1 tới giờ chỉ toàn số. **Tên** khoản — thứ bạn vẫn đọc
thành lời mỗi lần nhắc tới nó — bạn đang ghi ở đâu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
