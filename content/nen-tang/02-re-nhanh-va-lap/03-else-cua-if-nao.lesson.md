---
id: nen-tang.re-nhanh-va-lap.else-cua-if-nao
title: "`else` này của `if` nào"
summary: Mỗi `else` thuộc về `if` cùng mức thụt lề với nó — và đó đúng là chỗ `and` không thay được cho lồng.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ctrl.else-binding]
requires: [ctrl.else, ctrl.if-nested, ctrl.condition-path, ctrl.block-indent, logic.and]
concepts: [ctrl.re-nhanh, core.khoi-lenh]
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
Thêm một chữ `else` vào khối lồng, phép gộp `and` của bài trước hỏng ngay.
::::

::::explain{#cau-noi-voi-nguoi-la}
Bài trước để lại cho bạn một phép thử. Đây là đoạn lồng, có `else` ở tầng
trong:

```python
la_khach_quen = False
tien = 30000

if la_khach_quen:
    if tien >= 45000:
        print("Tính giá quen cho bác.")
    else:
        print("Bác quen mà thiếu tiền, ghi sổ nợ nhé.")
```

Một người lạ cầm 30 nghìn bước vào. Màn hình trống trơn: máy **không in ra
dòng nào cả** — và đó đúng là điều quán muốn. Câu "ghi sổ nợ" chỉ dành cho
người quen; với người lạ thì chưa có gì để nói.

Bây giờ làm phẳng đúng như bài trước dạy: gộp hai điều kiện bằng `and`, giữ
nguyên `else` bên dưới.

```python
la_khach_quen = False
tien = 30000

if la_khach_quen and tien >= 45000:
    print("Tính giá quen cho bác.")
else:
    print("Bác quen mà thiếu tiền, ghi sổ nợ nhé.")
```

Cũng người lạ ấy, cũng 30 nghìn ấy. Máy in ra:

```text
Bác quen mà thiếu tiền, ghi sổ nợ nhé.
```

Quán vừa gọi một người chưa từng tới đây là "bác quen", rồi mời họ ghi sổ nợ.

Không có lỗi nào hiện lên. Đoạn code chạy trơn tru từ đầu tới cuối. Nó chỉ nói
sai một câu với đúng một loại khách — và loại khách ấy có thể cả tuần mới ghé
một lần.

Vì sao? `else` **không** gắn vào một điều kiện. Nó gắn vào một `if`, và nó gom
mọi trường hợp còn lại của chính `if` đó. Khi bạn gộp hai câu hỏi thành một
`if`, thì "phần còn lại" của nó phình ra: khách quen thiếu tiền, khách lạ đủ
tiền, khách lạ thiếu tiền — cả ba loại rơi chung vào một rọ.

Còn khi lồng, `else` bên trong chỉ gom phần còn lại **của câu hỏi bên trong**.
Mà câu hỏi bên trong chỉ được hỏi với người đã qua cửa ngoài. Người lạ chưa
từng đi tới đó.

Đây là chỗ `and` không thay được cho lồng, và bài này nói về đúng chỗ ấy.
::::

::::example{#else-nhin-ngang-hang}
Vậy làm sao máy biết một `else` thuộc về `if` nào? Nó nhìn **cột lề**.

```python title=readonly
la_khach_quen = True
tien = 30000

if la_khach_quen:
    if tien >= 45000:
        print("Tính giá quen cho bác.")
    else:
        print("Bác quen mà thiếu tiền, ghi sổ nợ nhé.")
```

```text
Bác quen mà thiếu tiền, ghi sổ nợ nhé.
```

Xếp bốn dòng lệnh theo cột lề, sẽ thấy ngay từng cặp:

- Cột 0: `if la_khach_quen:` — câu hỏi ngoài. Ngay sau thân của nó không có
  dòng `else:` nào ở cột 0, nên câu hỏi ngoài **không có** `else`. (Một dòng
  cột 0 khác — chẳng hạn một `print` như câu "Đã ghi vào sổ." ở hai bài trước —
  thì chỉ là việc làm tiếp theo, không phải nhánh của nó.)
- Cột 4: `if tien >= 45000:` và `else:`. Hai dòng này ngang hàng nhau, nên
  chúng là hai lối của cùng một ngã ba — ngã ba về tiền.
- Cột 8: hai dòng `print`, mỗi dòng nằm trong một lối.

Luật gói gọn trong một câu: **`else` thuộc về `if` gần nhất phía trên nó có
cùng mức thụt lề.** Tiếng Anh gọi chuyện này là *else binding* — từ để bạn tra
cứu.

Đi lại đường máy đi với khách quen cầm 30 nghìn. `la_khach_quen` là `True`, máy
bước vào thân tầng ngoài. Trong thân ấy có một ngã ba: `30000 >= 45000` cho
`False`, nên máy rẽ sang lối `else` của ngã ba đó và in câu ghi sổ nợ.

Và nhớ đoạn đầu bài: nếu `la_khach_quen` là `False` thì máy không vào thân tầng
ngoài, nên nó không gặp cả ngã ba lẫn `else` của ngã ba ấy. Cái `else` nằm ở
cột 4 — nó nằm **bên trong** cửa ngoài, y như mọi dòng cột 4 khác.
::::

::::predict{#day-else-ra-le-trai commitOnce}
Vẫn khách quen ấy, vẫn 30 nghìn ấy. Đổi đúng một thứ: đẩy `else` ra sát lề
trái, cột 0. Câu chữ bên trong cũng sửa lại cho hợp với chỗ đứng mới.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
la_khach_quen = True
tien = 30000

if la_khach_quen:
    if tien >= 45000:
        print("Tính giá quen cho bác.")
else:
    print("Khách lạ, tính theo giá bảng.")
```

:::opt{correct}
Không in ra dòng nào cả
:::

:::opt
Khách lạ, tính theo giá bảng.
::why
Gần đúng ở chỗ bạn thấy trong đoạn này có một `if` vừa cho `False`, và nghĩ
rằng hễ có `if` nào sai thì `else` phải chạy. Đúng là `30000 >= 45000` cho
`False` thật.

Chỗ lệch nằm ở cột lề của `else`. Nó viết sát lề trái, ngang hàng với
`if la_khach_quen:` chứ không ngang hàng với `if tien >= 45000:`. Nên nó là nửa
còn lại của **câu hỏi ngoài**, và câu hỏi ngoài đang cho `True` — máy đã rẽ vào
lối kia rồi, `else` của nó bị bỏ qua.

Câu hỏi về tiền thì lần này chẳng có `else` nào của riêng mình cả.
::
:::

:::opt
Tính giá quen cho bác.
::why
Gần đúng ở chỗ bạn dò đúng đường đi như bài trước: cửa ngoài `la_khach_quen`
đang mở, nên máy có bước được vào thân tầng ngoài thật.

Chỗ lệch nằm ở lớp cửa thứ hai. `30000 >= 45000` cho `False`, nên dòng `print`
ở cột 8 không chạy. Đường đi tới một dòng đòi mọi cửa trên đường cùng mở.
::
:::

:::opt
Cả hai dòng, vì mỗi `if` chạy một lối
::why
Gần đúng ở chỗ bạn đếm ra hai câu hỏi trong đoạn và nghĩ mỗi câu hỏi phải cho
ra một câu trả lời. Cách đếm ấy có lý.

Chỗ lệch: hai câu hỏi này không đứng ngang hàng nhau. Câu hỏi về tiền nằm
**trong** thân của câu hỏi về khách quen, nên nó chỉ là một việc bên trong một
lối. Và một ngã ba `if` – `else` thì chỉ có đúng một lối được chạy, không bao
giờ cả hai. Ở đây câu hỏi ngoài đúng nên máy đi lối trên; trong lối trên, câu
hỏi tiền sai và không có `else` để rẽ sang.
::
:::

:::opt
Máy báo lỗi vì `else` nằm cách `if` của nó tận hai dòng
::why
Gần đúng ở chỗ bạn đọc ra rằng dòng `else` này đứng cách xa cái `if` mà mắt bạn
ghép cặp cho nó — giữa chúng còn chen mấy dòng thụt vào. Khoảng cách ấy có
thật.

Chỗ lệch: máy không ghép cặp bằng khoảng cách dòng, nó ghép bằng **cột lề**.
Tính theo cột lề thì `else` ở cột 0 nối thẳng với `if la_khach_quen:` ở cột 0,
và giữa hai dòng ấy có bao nhiêu dòng thụt vào cũng không sao. Thêm nữa, một
`if` không bắt buộc phải có `else`. Đoạn này hợp lệ trọn vẹn — nó chỉ không làm
điều bạn nghĩ.
::
:::
::::

::::explain{#lech-bon-dau-cach-doi-y-nghia}
Hai đoạn code trong bài này gồm gần như cùng những chữ giống nhau. Thứ phân
biệt chúng là cột lề của một dòng `else`:

- `else` ở **cột 4** — ngang hàng `if tien >= 45000:` — thì nó trả lời cho câu
  hỏi về tiền, và chỉ những khách quen mới nghe được câu ấy.
- `else` ở **cột 0** — ngang hàng `if la_khach_quen:` — thì nó trả lời cho câu
  hỏi về khách quen, và chỉ người lạ mới nghe được.

Cùng một chữ `else`, hai ý nghĩa cách nhau rất xa, và khoảng cách giữa chúng là
bốn dấu cách.

Nên khi một nhánh `else` chạy vào lúc bạn không ngờ tới, việc đầu tiên đáng làm
không phải là đọc lại điều kiện. Việc đầu tiên là **dò lên theo cột lề** xem
cái `else` ấy đang trả lời cho câu hỏi nào.
::::

::::code{#dat-else-dung-cot}
Sổ chi tiêu chỉ xét những ngày đã ghi sổ. Trong những ngày ấy: tiêu quá 200
nghìn thì nhắc một câu, còn không thì khen một câu.

Hôm nay có ghi sổ, tiêu 120 nghìn. Hai dòng `print` đã nằm sẵn ở cột 8. Hãy
viết dòng còn thiếu ở cột 4 sao cho câu khen là **phần còn lại của câu hỏi về
tiền**, chứ không phải phần còn lại của câu hỏi về sổ.

```python title=starter
co_ghi_so = True
tien = 260000

if co_ghi_so:
    if tien > 200000:
        print("Ngày này tiêu quá tay.")
    ___
        print("Ngày này tiêu vừa phải.")

tien = 120000

if co_ghi_so:
    if tien > 200000:
        print("Ngày này tiêu quá tay.")
    ___
        print("Ngày này tiêu vừa phải.")
```

```python title=solution
co_ghi_so = True
tien = 260000

if co_ghi_so:
    if tien > 200000:
        print("Ngày này tiêu quá tay.")
    else:
        print("Ngày này tiêu vừa phải.")

tien = 120000

if co_ghi_so:
    if tien > 200000:
        print("Ngày này tiêu quá tay.")
    else:
        print("Ngày này tiêu vừa phải.")
```

```python title=test
# Chấm bằng OUTPUT, và chấm bằng HAI ngày chứ không một.
#
# Một ngày thì không đủ. Với riêng ngày 120 nghìn, mọi câu dưới đây đều in ra
# đúng câu khen và đều lọt: `if True:`, `if co_ghi_so:`, `if tien < 500000:` —
# không câu nào là `else`, ở đúng bài dạy `else` thuộc về `if` nào.
#
# Hai ngày thì chúng lộ hết. Ngày 260 nghìn đi vào nhánh trên; một dòng `if`
# điền bừa ở cột 4 sẽ chạy THÊM nhánh dưới, nên màn hình ra ba dòng thay vì
# hai. Chỉ `else` mới loại trừ được nhánh kia.
#
# Đặt `else` ở cột 0 thì nó thành phần còn lại của câu hỏi về sổ — câu hỏi ấy
# đang đúng ở cả hai ngày, nên nhánh dưới không chạy lần nào, và màn hình chỉ
# còn một dòng.
pass
```

:::hints
- kind: attention
  body: Hai chỗ trống nhận cùng một câu trả lời — cùng một ngã ba, thử trên hai ngày khác nhau. Cả hai đều nằm ở cột 4, ngang hàng với dòng `if tien > 200000:`. Ngang hàng nghĩa là hai dòng ấy sẽ thành hai lối của cùng một ngã ba.
- kind: strategy
  body: Bạn cần từ khoá mở ra lối "còn không thì" mà Realm 0 đã dạy. Nó đứng một mình, phía sau không kèm câu hỏi nào, và kết thúc bằng dấu hai chấm vì nó mở ra một khối lệnh.
- kind: one-line
  body: "Thay cả hai `___` bằng `else:`, giữ nguyên bốn dấu cách phía trước nó."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Ngày này tiêu quá tay\.\nNgày này tiêu vừa phải\.\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cột lề nói cho mình biết `else` ấy là của ai. Bạn vừa nói đúng ý mình muốn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả bài này là đi tìm **chủ** của một dòng `else`: nó gom phần còn lại của `if`
nào. Cột lề luôn chỉ ra được đúng một cái tên.

Realm 0 thì đã cho bạn luật của chuỗi nhánh: `if` – `elif` – `elif` hỏi từ trên
xuống và dừng ở nhánh đúng **đầu tiên**.

Hôm nay bạn hỏi một dòng `else`: *nó thuộc về `if` nào?* Bây giờ mang đúng câu
hỏi ấy sang một chuỗi `elif`, nhưng hỏi từ phía ngược lại: **những ngày nào
thuộc về dòng này?** Sổ chi tiêu xếp mức tiêu, nhánh "quá 100 nghìn" đứng trước
nhánh "quá 500 nghìn":

```python
if tien > 100000:
    print("Ngày tiêu nhiều")
elif tien > 500000:
    print("Ngày tiêu rất nhiều")
```

Lấy giấy bút và tìm giúp mình **một** số tiền in ra được câu "Ngày tiêu rất
nhiều". Thử 600 nghìn. Thử một triệu. Thử số nào bạn thích.

Nếu tìm mãi không ra số nào, thì dòng `elif` ấy nằm trong file để làm gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
