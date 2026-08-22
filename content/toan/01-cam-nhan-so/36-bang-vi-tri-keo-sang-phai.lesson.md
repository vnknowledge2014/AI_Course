---
id: toan.cam-nhan-so.bang-vi-tri-keo-sang-phai
title: Bảng vị trí kéo sang phải
summary: "Kéo bảng giá trị vị trí sang bên phải dấu phẩy thì các cột thành 1/10, 1/100 — nên số thập phân là phân số có mẫu sẵn cùng một họ 10, 100, 1000: quy đồng chỉ còn là thêm chữ số 0, không phải đi tìm cỡ thước."
locale: vi
track: toan
module: cam-nhan-so
order: 36
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.decimal-place-value]
requires: [math.fraction-add, math.fraction-compare, math.equivalent-fraction, math.dong-goi, math.place-value-powers, math.exponent, math.unit-fraction, math.remainder, core.arithmetic, core.variable, core.print-variable, core.boolean, ctrl.comparison]
concepts: [math.thap-phan, math.gia-tri-vi-tri, math.phan-so]
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
Bảng cột của mình còn chạy tiếp sang bên phải. Ở bên đó mẫu có sẵn rồi.
::::

::::explain{#mo-bo-thay-vi-gom-bo}
Bài trước để lại một chỗ mỏi tay: mỗi lần cộng phân số lại phải đi tìm một cỡ
thước chung mới. Câu hỏi bỏ ngỏ là có loại thước nào mọi lượng đều đã đo sẵn
bằng nó chưa.

Có, và bạn đã cầm nó từ bài 6. Chỉ cần cho cái luật đóng bó chạy **ngược
chiều**.

Bài 6 nói: đủ mười hạt thì gom thành một bó, và bó ấy sang đứng ở cột bên
trái. Cứ thế, mỗi bước sang trái là gom mười thành một, nên cột bên trái to
gấp mười cột bên phải nó.

Chạy ngược lại: mỗi bước sang **phải** là **mở một bó ra thành mười phần**.
Từ cột bó-của-bó mở ra được cột bó; từ cột bó mở ra được cột hạt lẻ — cột
"một".

Suốt 29 bài vừa rồi, bảng vị trí dừng ở đó. Nhưng không có gì bắt nó phải
dừng. Mở tiếp cột "một" ra thì được mười phần bằng nhau, mỗi phần là `1/10` —
đúng cái thước mà bài 30 đã dạy bẻ. Mở tiếp một cái `1/10` thì được mười cái
`1/100`.

Bảng vị trí giờ chạy được cả hai chiều, và người ta cắm một cái mốc để đánh
dấu chỗ cột "một" đứng: **dấu phẩy**.

Đọc `0,25` mét theo bảng ấy: chữ số `2` đứng ở cột `1/10`, chữ số `5` đứng ở
cột `1/100`. Nghĩa là 2 cái `1/10` mét và 5 cái `1/100` mét. Bài 35 vừa dạy
cách gộp hai lượng ấy — quy về cùng thước `1/100`: `20/100 + 5/100 = 25/100`.

`0,25` **là** `25/100`. Không phải một loại số mới; nó là phân số, viết theo
cách để **vị trí lo giùm cái mẫu**.

Và đây là chỗ trả lời câu hỏi bài trước: mẫu của mọi số viết kiểu này luôn là
10, hoặc 100, hoặc 1000 — cùng một họ. Muốn hai bên cùng mẫu thì chỉ việc mở
thêm bó, tức thêm chữ số `0` vào bên phải, chứ không phải đi tìm cỡ thước nào.
::::

::::example{#doc-hai-so-thap-phan}
Byte đo hai luống: một luống `0,25` mét, một luống `0,4` mét. Đếm cả hai bằng
thước `1/100` mét.

```python title=readonly
# 0,25 mét = 2 cái 1/10 mét + 5 cái 1/100 mét.
# Mỗi cái 1/10 mở ra thành 10 cái 1/100.
print(2 * 10 + 5 * 1)

# 0,4 mét = 4 cái 1/10 mét, cột 1/100 rỗng.
print(4 * 10 + 0 * 1)
```

Máy in ra:

```text
25
40
```

`0,25` mét là `25/100` mét; `0,4` mét là `40/100` mét. Hai mẫu bằng nhau mà
không ai phải quy đồng — chỉ cần mở bó thêm một bước ở bên có ít cột hơn.

Cùng mẫu rồi thì bài 34 trả lời ngay: 25 phần ít hơn 40 phần, nên `0,25` mét
**ngắn hơn** `0,4` mét. Nhiều chữ số hơn không có nghĩa là dài hơn.

Đi ngược lại cũng được, và vẫn là việc bài 33 đã dạy — chỉ chạy theo chiều
gom thay vì chiều bẻ: chia **số phần** cho 25, rồi chia luôn **số phần trong
một đơn vị** cho 25. Tức là 25 phần của thước `1/100` gom lại vừa đúng 1 phần
của thước `1/4`, không thừa mẩu nào. Vậy `1/4` và `0,25` là cùng một chỗ trên
thanh số, viết bằng hai cách.
::::

::::predict{#doan-luong-nao-dai-hon commitOnce}
Bây giờ hai luống chỉ khác nhau một chữ số `0`: `0,25` mét và `0,205` mét.

Đếm cả hai bằng **cùng một** thước — thước `1/1000` mét. Ở thước đó, mỗi cái
`1/10` gồm 100 phần, mỗi cái `1/100` gồm 10 phần, mỗi cái `1/1000` gồm 1 phần.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
a = 2 * 100 + 5 * 10 + 0 * 1      # 0,25  mét
b = 2 * 100 + 0 * 10 + 5 * 1      # 0,205 mét

print(a > b)
print(a == b)
```

:::opt{correct}
True, rồi False
:::

:::opt
False, rồi False
::why
Gần đúng ở chỗ bạn dùng một luật thật sự đúng: số nào **nhiều chữ số hơn** thì
lớn hơn. `0,205` có ba chữ số sau dấu phẩy, `0,25` chỉ có hai — và với số đếm
ở bên trái dấu phẩy thì luật ấy không sai lần nào, bài 7 dựng ra bảng vị trí
đúng để nó đúng.

Chỗ lệch là phạm vi của luật ấy. Bên **trái** dấu phẩy, thêm một cột là gom
mười thành một, tức nhân mười — nên thêm chữ số làm con số to lên. Bên
**phải**, thêm một cột là mở một ra thành mười, tức chia mười — nên chữ số
thêm vào chỉ góp được một lượng **nhỏ hơn một phần** của cột đứng ngay trước
nó. Nó làm con số mịn hơn, và có làm to lên chút ít — `0,25` đúng là nhỏ hơn
`0,251` — nhưng không bao giờ bù nổi chỗ đã thua ở một cột to hơn: `0,205`
thua `0,25` ngay tại cột `1/100`, 0 phần so với 5 phần, nên chữ số `5` nằm mãi
ở cột `1/1000` có thêm vào cũng không gỡ lại được.

Luật thật là thế này: **so số thập phân là so từ cột to nhất chạy sang phải;
cột nào hơn trước thì hơn hẳn, bao nhiêu cột phía sau cũng không cứu được.**
Quy về thước `1/1000` thì `0,25` được 250 phần, còn `0,205` được 205 phần.
::
:::

:::opt
False, rồi True
::why
Gần đúng ở chỗ bạn nhớ điều bài 8 nói về chữ số `0`: nó không mang theo lượng
nào cả. Cột `1/100` của `0,205` đúng là rỗng thật, chẳng có phần nào đứng ở
đó — chỗ ấy bạn đọc chuẩn.

Chỗ lệch: bỏ chữ số `0` đi thì chữ số `5` **dịch sang trái một cột**, từ cột
`1/1000` sang cột `1/100`, tức lớn lên mười lần. Đó chính là việc mà bài 8
dựng ra số `0` để ngăn: nó không mang lượng, nhưng nó **giữ chỗ**, và nhờ giữ
chỗ mà `0,205` không lẫn thành `0,25`.
::
:::

:::opt
1, rồi 0
::why
Gần đúng ở chỗ bạn nghĩ đúng/sai trong máy vẫn ghi được bằng `1` và `0` — điều
đó có thật, Realm 0 đã nói máy bên dưới chỉ có hai chữ số ấy, và nhiều nơi
đúng là hiện `1` với `0`.

Chỗ lệch nằm ở việc Python chọn hiện ra cái gì cho bạn đọc. Nó in đúng hai chữ
`True` và `False`, viết hoa chữ đầu, như bạn đã gặp ở Realm 0. Chuyện `1` và
`0` là chuyện bên dưới, không phải thứ `print` đưa lên màn hình.
::
:::
::::

::::explain{#ten-cua-cac-cot-moi}
Mấy cái cột mới bên phải dấu phẩy cũng có tên riêng — dãy số mũ của bài 25
viết tiếp xuống dưới `0` thì thành **luỹ thừa âm**. Bài này không cần tới cách
viết ấy, ta cứ đếm bằng **số phần**; cứ biết là mấy cột mới không phải đồ vô
danh, và để dành luỹ thừa âm cho lúc nó có việc thật.

Còn một chuyện nhỏ nhưng dễ vấp khi bạn gõ vào máy. Trên giấy tiếng Việt ta
ngăn phần nguyên với phần lẻ bằng **dấu phẩy** — `0,25`. Python ngăn bằng
**dấu chấm** — `0.25`. Trong bài này ta cố ý đếm bằng **số phần** (toàn số
nguyên) nên chưa cần tới cách viết ấy; cứ nhớ hai ký hiệu là hai thói quen ghi
chép của hai nơi, không phải hai con số khác nhau.
::::

::::code{#hai-luong-mot-cai-thuoc-san}
Byte nối hai luống lại và muốn biết luống nào dài hơn:

- **Luống A** dài `0,25` mét.
- **Luống B** dài `0,4` mét.

Đếm cả hai bằng thước `1/100` mét. Hai chỗ trống hỏi cùng một câu: *lượng này
đếm bằng thước `1/100` mét thì được mấy phần?*

Bài chấm bằng **hai** dòng ra khác nhau: một dòng là tổng số phần, một dòng là
câu trả lời có–không. Điền cùng một con số vào cả hai chỗ trống thì tổng sai;
điền số ở cột `1/10` mà quên mở bó thì tổng cũng sai.

```python title=starter
# Mỗi cái 1/10 mét mở ra thành 10 cái 1/100 mét.
a = ___
b = ___

print(a + b)
print(a > b)
```

```python title=solution
# Mỗi cái 1/10 mét mở ra thành 10 cái 1/100 mét.
a = 25
b = 40

print(a + b)
print(a > b)
```

```python title=test
# Bốn assert trên hai lượng khác nhau. Chỗ bẫy nằm ở luống B: chữ số 4 đứng ở
# cột 1/10, nên đếm bằng thước 1/100 thì nó thành 40 phần chứ không phải 4.
assert a == 25, "0,25 mét là 2 cái 1/10 và 5 cái 1/100 — đếm bằng thước 1/100 thì được 25 phần"
assert b == 40, "0,4 mét là 4 cái 1/10, mỗi cái mở ra 10 cái 1/100 — nên được 40 phần, không phải 4"
assert a + b == 65, "25 phần cộng 40 phần được 65 phần của thước 1/100, tức 0,65 mét"
assert a < b, "0,25 mét NGẮN hơn 0,4 mét, dù nó có nhiều chữ số hơn"
```

:::hints
- kind: attention
  body: Đừng chép thẳng các chữ số sau dấu phẩy vào chỗ trống. Hỏi từng chữ số một xem nó đang đứng ở cột nào — cột `1/10` hay cột `1/100`.
- kind: strategy
  body: Tách mỗi số theo cột. `0,25` có `2` ở cột `1/10` và `5` ở cột `1/100`. `0,4` có `4` ở cột `1/10`, còn cột `1/100` rỗng. Mỗi cái ở cột `1/10` mở ra thành 10 phần của thước `1/100`, rồi cộng lại với phần đã sẵn ở cột `1/100`.
- kind: one-line
  body: "Điền `25` vào chỗ trống thứ nhất và `40` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^65\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
65 phần của thước 1/100 mét. Bên phải dấu phẩy, mẫu có sẵn khỏi phải tìm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy `1/4` và `0,25` là cùng một chỗ trên thanh số. Chuyện đó chạy
được vì `1/4` đổi thước lên được thành `25/100` — bẻ mỗi `1/4` thành 25 phần
nhỏ thì vừa khít cột `1/100`, không thừa mẩu nào.

Bây giờ thử với `1/3`. Bẻ một đơn vị — một mét dây, hay một lon gạo — ra ba
phần bằng nhau, rồi hỏi: một phần ấy viết theo bảng vị trí thì ra số nào?

Cột `1/10` chứa được 3 phần, còn dư. Mở tiếp sang cột `1/100` cũng chứa thêm
được vài phần, vẫn dư. Mỗi lần mở bó ra chia cho 3, bài 29 đã cảnh báo sẵn:
**vẫn còn thừa**.

Vậy `1/3` viết thành thập phân thì ra con số nào — và chuyện gì xảy ra nếu
lần nào mở bó cũng thừa? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
