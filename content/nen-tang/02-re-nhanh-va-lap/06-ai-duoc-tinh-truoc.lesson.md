---
id: nen-tang.re-nhanh-va-lap.ai-duoc-tinh-truoc
title: Ai được tính trước
summary: Một dòng có nhiều phép thì máy luôn theo cùng một bậc thang — số học, so sánh, not, and, or.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [logic.precedence]
requires: [logic.not, logic.and, logic.or, core.boolean, ctrl.if]
concepts: [logic.phep-logic, core.dung-sai]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Dòng nào mình cũng đọc theo đúng một bậc thang. Không bao giờ đổi, không bao giờ phân vân.
::::

::::explain{#mot-bac-thang-co-dinh}
Bài trước để bạn lại với một dòng đọc được hai cách:

```python
not co_trong_so and vuot_nguong
```

Bạn phân vân là phải. Máy thì không. Nó có sẵn một **bậc thang** cố định, dùng
cho mọi dòng nó gặp, từ dòng đầu tiên bạn viết cho tới dòng cuối cùng.

Bậc thang ấy không phải chuyện mới toanh. Hồi học toán ở trường bạn đã dùng một
cái y hệt:

> `2 + 3 * 4` bằng bao nhiêu?

Bằng 14, không phải 20. Vì có luật: **nhân chia trước, cộng trừ sau**. Không ai
hỏi lại người viết xem ý họ là gì — cả lớp cùng đọc theo một luật, nên cả lớp ra
cùng một số.

Python dùng đúng ý tưởng đó, chỉ là bậc thang của nó dài hơn. Đây là toàn bộ
phần bạn cần cho tới hết Realm này, xếp từ **được tính trước nhất** xuống:

| bậc | phép | ví dụ |
|---|---|---|
| 1 — trước nhất | số học | `+` `-` `*` `/` |
| 2 | so sánh | `>` `<` `>=` `<=` `==` `!=` |
| 3 | `not` | `not co_trong_so` |
| 4 | `and` | `a and b` |
| 5 — sau cùng | `or` | `a or b` |

Đọc bậc thang này theo nghĩa: phép ở bậc **trên** gom lấy giá trị của nó
**trước**, rồi phép ở bậc dưới mới có thứ để làm việc.

Và đây là câu trả lời cho dòng bỏ ngỏ hôm qua. `not` ở bậc 3, `and` ở bậc 4 —
`not` trên `and`. Nên `not` gom lấy phần của nó trước, mà phần sát bên phải nó
chỉ có đúng một cái tên:

> **Không** có trong sổ, **mà lại** vượt ngưỡng.

Cách đọc thứ hai — *không phải là vừa có trong sổ vừa vượt ngưỡng* — không phải
cách máy chọn.
::::

::::example{#doc-mot-dong-theo-bac-thang}
Bậc thang chỉ đáng nhớ khi bạn dùng được nó để đọc một dòng thật. Đây là một
dòng dùng tới bốn bậc cùng lúc, chép từ sổ chi tiêu của Byte.

Hôm nay Byte tiêu 260 nghìn, được hoàn lại 50 nghìn tiền trả hàng, và chưa ghi
vào sổ:

```python title=readonly
tien = 260000
hoan = 50000
co_trong_so = False

print(tien - hoan > 200000 and not co_trong_so)
```

Máy in ra:

```text
True
```

Đi lại đúng đường máy đi, leo bậc thang từ trên xuống:

- **Bậc 1 — số học.** `tien - hoan` cho `210000`. Dòng bây giờ là
  `210000 > 200000 and not co_trong_so`.
- **Bậc 2 — so sánh.** `210000 > 200000` cho `True`. Dòng còn
  `True and not co_trong_so`.
- **Bậc 3 — `not`.** `not False` cho `True`. Dòng còn `True and True`.
- **Bậc 4 — `and`.** `True and True` cho `True`. Hết dòng.

Bốn bậc, không một dấu hiệu nào thêm để chỉ chỗ. Bạn viết dòng ấy y như đọc lên
thành tiếng — *"tiền trừ hoàn mà còn quá 200 nghìn, và chưa ghi sổ"* — và máy
hiểu đúng, vì bậc thang của nó trùng với thứ tự bạn nghĩ.

Đây cũng là lý do bốn bài đầu track chạy trơn mà bạn chưa từng phải để ý tới
chuyện này. `tien > 200000 and co_trong_so` chạy đúng vì so sánh ở bậc 2, `and`
ở bậc 4 — phép so sánh xong xuôi rồi `and` mới tới lượt. Bậc thang đã làm việc
cho bạn từ lâu, hôm nay bạn mới nhìn thấy nó.
::::

::::predict{#not-om-toi-dau commitOnce}
Ngày hôm nay Byte **đã** ghi vào sổ (`co_trong_so` giữ `True`) và **không** vượt
ngưỡng (`vuot_nguong` giữ `False`).

Byte in ra hai dòng. Hai dòng dùng đúng những chữ như nhau — chỉ khác chỗ: cái
tên nào được đặt ngay sau `not`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai dòng nào?

```python title=readonly
co_trong_so = True
vuot_nguong = False

print(not co_trong_so and vuot_nguong)
print(not vuot_nguong and co_trong_so)
```

:::opt{correct}
`False` rồi `True`
:::

:::opt
`True` rồi `True`
::why
Gần đúng ở chỗ bạn đọc `not` như tiếng Việt đọc chữ *không phải* — trùm lên cả
câu đứng sau nó. Trong câu nói hằng ngày thì đúng là như vậy, và cách đọc ấy
cho ra `True` ở cả hai dòng thật: hôm nay không phải ngày vừa-có-trong-sổ
vừa-vượt-ngưỡng, mà cũng không phải ngày vừa-vượt-ngưỡng vừa-có-trong-sổ.

Chỗ lệch: máy không đọc tiếng Việt, nó leo bậc thang. `not` ở bậc 3, `and` ở
bậc 4, nên `not` gom phần của nó xong thì `and` mới tới lượt. Mà phần `not` gom
được chỉ là thứ nằm sát ngay bên phải nó ở bậc trên — ở dòng này là một cái tên — chứ không phải cả cụm.
::
:::

:::opt
`False` rồi `False`
::why
Gần đúng ở chỗ bạn tính chính xác dòng thứ tư: `not co_trong_so` cho `False`,
rồi `False and vuot_nguong` cho `False`. Bậc thang bạn leo đúng.

Chỗ lệch nằm ở dòng thứ năm. Hai dòng nhìn na ná nhau nên rất dễ tin chúng cho
cùng một kết quả, nhưng hai cái tên đã đổi chỗ cho nhau. Ở dòng thứ năm, thứ
nằm sát bên phải `not` là `vuot_nguong`, tức `False`, nên `not` lật nó thành
`True`. Còn lại `True and co_trong_so`, tức `True and True`, cho `True`.

Bài học đọc code: khi thấy `not`, việc đầu tiên là nhìn xem **nó ôm tới
đâu** — hết cả phép so sánh sát bên phải, nhưng dừng lại trước `and`.
::
:::

:::opt
Máy báo lỗi: một dòng có cả `not` lẫn `and` thì phải nói rõ cái nào tính trước
::why
Gần đúng ở chỗ bạn nhớ đúng tính cách của Python từ Realm 0: gặp chuyện không
có một câu trả lời đúng duy nhất thì nó dừng lại hỏi chứ không đoán bừa —
`TypeError` sinh ra đúng từ tinh thần ấy.

Chỗ lệch: ở đây **có** một câu trả lời duy nhất, vì bậc thang đã quyết sẵn từ
trước khi bạn gõ dòng đầu tiên. Máy không phân vân nên không có gì để hỏi. Chỗ
mập mờ chỉ nằm trong đầu người đọc, không nằm trong máy — và đó chính là loại
sai nguy hiểm: chương trình chạy trơn tru mà trả lời câu hỏi khác câu bạn định
hỏi.
::
:::
::::

::::explain{#hai-he-qua-cua-bac-thang}
Bậc thang có hai hệ quả bạn dùng được ngay, và cả hai đều tiết kiệm chữ cho
bạn.

**Một: `not` ôm được cả một phép so sánh.** Vì so sánh ở bậc 2, còn `not` ở bậc
3 — so sánh xong trước thì `not` mới có thứ để lật:

```python title=readonly
tien = 150000
print(not tien > 200000)
```

Máy in ra `True`. Nó làm `tien > 200000` trước, được `False`, rồi `not` lật
thành `True`. Đọc lên: *"không phải là tiêu quá 200 nghìn"* — đúng với ngày 150
nghìn.

**Hai: `and` được tính trước `or`.** Đây là chỗ bậc thang cắn mạnh nhất, vì hai
từ này nhìn cùng cỡ nhau nên người ta hay tưởng chúng ngang hàng:

```python title=readonly
cuoi_tuan = True
tien = 100000
hoan = 50000
co_trong_so = True

print(cuoi_tuan or tien - hoan > 200000 and not co_trong_so)
```

Máy in ra `True`. Bậc thang gom dòng ấy thành
`cuoi_tuan or ((tien - hoan > 200000) and (not co_trong_so))` — `and` buộc
chặt hơn nên nó gom trước, `or` đứng ngoài cùng. Soi cụm trong ngoặc:
`50000 > 200000` cho `False`, `not True` cho `False`, `and` ghép hai cái ấy
thành `False`. Còn `or` thì `True or False` cho `True`.

(Thật ra máy còn lười hơn thế: `or` thấy vế trái đã `True` là nó thôi, không
thèm tính cụm bên phải lần nào. Nhưng chuyện lười ấy là bài khác — ở đây ta
đang đọc **hình dạng** của dòng, và hình dạng thì không đổi.)

Nói gọn lại: `and` buộc chặt hơn `or`, nên một dòng có cả hai luôn đọc thành
*"cái này, **hoặc là** cụm kia"* — chứ không phải ngược lại.

> Dễ nhầm: hai phép **cùng một bậc** thì máy làm từ trái sang phải, như
> `a and b and c`. Bậc thang chỉ phân xử giữa các bậc **khác nhau** — nó không
> phải luật "đọc từ đâu tới đâu", nó là luật "ai gom giá trị trước".
::::

::::code{#mot-dong-bon-bac}
Cuối tuần Byte soát lại sổ. Ngày **cần xem lại** là ngày thoả cả hai chuyện:

- Sau khi trừ khoản được hoàn 50 nghìn, số tiền còn lại **vẫn quá 200 nghìn**.
- Ngày đó **chưa** ghi vào sổ.

Bốn ngày trong sổ:

- **Thứ hai** — tiêu 300 nghìn, chưa ghi sổ.
- **Thứ ba** — tiêu 300 nghìn, đã ghi sổ.
- **Thứ tư** — tiêu 220 nghìn, chưa ghi sổ.
- **Thứ năm** — tiêu 260 nghìn, chưa ghi sổ.

Cả hai chuyện viết được vào **một dòng duy nhất**, dùng tới bốn bậc của bậc
thang, và không cần một dấu hiệu nào để chỉ chỗ. Cùng câu hỏi ấy đặt cho bốn
ngày, nên **bốn chỗ trống điền giống hệt nhau**.

Viết đúng thì màn hình hiện ra **hai** trong bốn dòng chữ.

```python title=starter
hoan = 50000

tien = 300000
co_trong_so = False
if ___:
    print("Thứ hai cần xem lại")

tien = 300000
co_trong_so = True
if ___:
    print("Thứ ba cần xem lại")

tien = 220000
co_trong_so = False
if ___:
    print("Thứ tư cần xem lại")

tien = 260000
co_trong_so = False
if ___:
    print("Thứ năm cần xem lại")
```

```python title=solution
hoan = 50000

tien = 300000
co_trong_so = False
if tien - hoan > 200000 and not co_trong_so:
    print("Thứ hai cần xem lại")

tien = 300000
co_trong_so = True
if tien - hoan > 200000 and not co_trong_so:
    print("Thứ ba cần xem lại")

tien = 220000
co_trong_so = False
if tien - hoan > 200000 and not co_trong_so:
    print("Thứ tư cần xem lại")

tien = 260000
co_trong_so = False
if tien - hoan > 200000 and not co_trong_so:
    print("Thứ năm cần xem lại")
```

```python title=test
# Chấm bằng TRỌN VẸN output, trên CẢ BỐN ngày.
#
# Một ngày thôi thì không phân biệt được đúng với sai: điều kiện nào cũng chỉ
# cho ra `True` hoặc `False`, nên người gõ bừa có đúng hai lựa chọn và luôn
# trúng một cái. Bốn ngày được chọn sao cho mỗi bậc bị bỏ sót để lại một dấu
# vết khác nhau:
#
# - Điền một thứ luôn đúng (`True`, `1`): cả bốn dòng cùng hiện.
# - Điền một thứ luôn sai (`0`): không dòng nào hiện.
# - Quên bậc 1, viết `tien > 200000 and not co_trong_so`: thứ tư lọt vào, vì
#   220 nghìn vượt ngưỡng khi chưa trừ khoản hoàn nhưng không vượt sau khi
#   trừ. Ra ba dòng.
# - Quên bậc 3, viết `tien - hoan > 200000 and co_trong_so`: chỉ thứ ba lên
#   tiếng — đúng một dòng, và là dòng ngược hẳn ý bài.
# - Đổi `and` thành `or`: cả bốn dòng cùng hiện.
#
# Thứ ba tách được cách quên `not`; thứ tư tách được cách quên phép trừ. Bỏ
# một trong hai ngày ấy đi là mở đường cho một cách sai lọt lưới.
pass
```

:::hints
- kind: attention
  body: Ý bạn có hai chuyện nối bằng chữ "và", và chuyện thứ nhất còn phải tính một phép trừ trước khi đem đi so sánh. Bậc thang trong bài xếp phép trừ, phép so sánh, `not` và `and` ở bốn bậc khác nhau — nên viết thẳng theo thứ tự bạn nghĩ là được.
- kind: strategy
  body: Viết từng chuyện ra riêng trước. Chuyện thứ nhất là "tiền trừ hoàn còn quá 200 nghìn". Chuyện thứ hai là "chưa ghi sổ" — dùng chữ lật của bài trước. Rồi nối hai chuyện bằng `and`, giữ nguyên thứ tự vừa viết, không thêm dấu hiệu nào khác.
- kind: one-line
  body: "Viết `tien - hoan > 200000 and not co_trong_so` vào cả bốn chỗ trống, giữ nguyên dấu hai chấm ở cuối mỗi dòng `if`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Thứ hai cần xem lại\nThứ năm cần xem lại\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn bậc trong một dòng, không cần đánh dấu chỗ nào. Bậc thang lo hết.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bậc thang trả lời được câu hỏi hôm qua: `not` ôm phần sát bên phải nó ở các
bậc trên — một cái tên, hoặc trọn một phép so sánh — rồi dừng lại trước `and`.
Giờ bạn đọc được mọi dòng máy viết ra.

Nhưng còn chiều ngược lại. Sáng nay Byte tính đi ăn phở, đi được khi **vừa đủ
tiền vừa kịp giờ**. Byte muốn máy nhắc vào đúng những sáng phải nhịn — tức là
những sáng **không phải** là vừa đủ tiền vừa kịp giờ.

Đó chính là cách đọc mà bài này vừa nói là máy **không** chọn. Nhưng lần này nó
đúng là ý bạn:

```python
not du_tien and kip_gio
```

Máy sẽ đọc thành *"không đủ tiền, mà lại kịp giờ"* — một câu khác hẳn. Nó không
đọc sai; nó đọc đúng bậc thang, và bậc thang thì không biết bạn đang nghĩ gì.

Bạn cũng không sửa được bậc thang, vì mọi dòng khác trong chương trình đang
sống nhờ nó.

Vậy còn cách nào để nói với máy rằng *mấy chữ này gom lại thành một cụm, tính
cụm ấy trước đã*? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
