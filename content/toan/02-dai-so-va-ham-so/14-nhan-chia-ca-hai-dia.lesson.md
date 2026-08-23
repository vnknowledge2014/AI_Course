---
id: toan.dai-so-va-ham-so.nhan-chia-ca-hai-dia
title: Nhân, chia cả hai đĩa
summary: Nhân hay chia cả hai đĩa cho cùng một số khác 0 cũng giữ nguyên tập nghiệm — còn số 0 thì bóp cả thanh số về một điểm và cuốn theo cả câu hỏi.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.equation-multiply-both-sides]
requires: [math.equation-add-both-sides, math.equation, math.multiplication, math.multiply-as-scaling, math.multiply-by-negative, math.division-partitive, math.division-by-zero, math.thanh-so, math.compare-on-number-line, core.variable, core.reassign, core.arithmetic, core.division, core.float, core.number-literal, core.print-variable, core.output, core.boolean, ctrl.comparison]
concepts: [math.phuong-trinh-tuong-duong, math.giu-nguyen-nghiem, math.keo-gian-thanh-so]
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
Hai khay nặng đúng 12 ổ. Vậy một khay mấy ổ — cân nói hộ mình được không?
::::

::::explain{#nua-cua-hai-ben-bang-nhau}
Bài trước để Byte lại ở đây:

```text
    ┌───────────────┐       ┌───────────────┐
    │     n + n     │  ═══  │      12       │
    └───────────────┘       └───────────────┘
        đĩa trái                đĩa phải
```

`2n = 12`

Cộng và trừ đã hết việc. Bớt thêm nữa thì đĩa trái mất luôn cả khay, mà bớt cái
gì ở đĩa trái cũng phải bớt đúng chừng ấy ở đĩa phải — càng bớt càng rối.

Việc phải làm thì ai đứng trước cái cân cũng nhìn ra: đĩa trái đang có **hai**
khay giống hệt nhau, mà Byte chỉ muốn biết **một** khay. Vậy thì bỏ bớt đi một
khay, và bỏ đúng **một nửa** số ổ ở đĩa bên kia:

```text
    ┌───────────────┐       ┌───────────────┐
    │       n       │  ═══  │       6       │
    └───────────────┘       └───────────────┘
        đĩa trái                đĩa phải
```

`n = 6`

Ô trống đã đứng một mình. Mỗi khay 6 ổ.

Nhưng khoan mừng — bài 13 vừa dạy một thói quen đáng giữ: **hỏi lại xem cân có
thật sự còn thăng bằng không.** Lần trước thao tác là bớt đi cùng một *lượng*.
Lần này thao tác khác hẳn: bỏ đi cùng một *phần*, mỗi đĩa một nửa. Nó cũng an
toàn chứ?
::::

::::explain{#vi-sao-chia-doi-van-can-bang}
Cái cân trả lời được nửa câu hỏi. Hai đĩa đang nặng bằng nhau. Chia mỗi đĩa
thành hai phần bằng nhau rồi giữ lại một phần — đó đúng là phép **chia đều** ở
T2.1 bài 26. Hai lượng bằng nhau, chia đều cho cùng số phần, thì mỗi phần của
bên này bằng mỗi phần của bên kia. Cân vẫn thăng bằng.

Gấp lên cũng thế. Thay mỗi đĩa bằng **ba bản sao** của chính nó — đó là T2.1
bài 19, nhân là mấy lần một lô. Ba lô bằng nhau đặt bên trái, ba lô bằng nhau
đặt bên phải: vẫn cân.

Nhưng cân chỉ nói được nửa lời hứa — cái nửa "**không mất nghiệm**". Còn nửa
kia, "**không thêm nghiệm lạ**", phải hỏi bức tranh thứ hai: **thanh số**.

T2.1 bài 22 đã dựng sẵn: nhân với một số là **kéo giãn thanh số** quanh mốc 0.
Nhân 3 thì mọi chỗ dạt ra xa 0 gấp ba. Chia cho 3 thì mọi chỗ co lại về gần 0
ba lần. Nhân với −1 thì cả thanh **lật** sang phía bên kia (T2.1 bài 23).

Điều đáng nhìn là: kéo giãn hay lật thế nào thì **hai chỗ khác nhau vẫn là hai
chỗ khác nhau**. Chúng cùng dạt ra, cùng co vào, cùng lật — không cái nào đuổi
kịp cái nào.

```text
   nhân 3:   5 ────────────>  15
             6 ──────────────>  18      hai chỗ vẫn là hai chỗ

   nhân 0:   5 ──────>  0
             6 ──────>  0               hai chỗ hoá thành một chỗ
```

Nên nếu một số làm hai đĩa **lệch** nhau, thì sau khi nhân cả hai đĩa với cùng
một số, hai đĩa vẫn lệch. Không có số nào tự nhiên trở thành nghiệm.

Trừ đúng một chỗ, và bạn vừa nhìn thấy nó ở dòng cuối bức hình.
::::

::::example{#nhan-voi-khong-thi-hong}
Nhân với 0 không kéo giãn cũng không lật. Nó **bóp cả thanh số về đúng một
điểm**: mọi chỗ, gần xa gì, đều bị dí vào 0.

Đem phép ấy áp lên cái cân: nhân cả hai đĩa với 0 thì cả hai đĩa cùng rỗng.

```text
    ┌───────────────┐       ┌───────────────┐
    │       0       │  ═══  │       0       │
    └───────────────┘       └───────────────┘
        đĩa trái                đĩa phải
```

`0 = 0`

Cân thăng bằng — thăng bằng hoàn hảo, chưa bao giờ thăng bằng đến thế. Và đó
đúng là chỗ hỏng: nó thăng bằng **bất kể `n` bằng bao nhiêu**. Số 5 cũng được,
số 6 cũng được, số một triệu cũng được. Hai chỗ vốn khác nhau đã bị bóp vào
cùng một chỗ, nên không còn gì phân biệt số nào với số nào.

Hỏi máy cho chắc, ở hai số mà câu gốc trả lời khác nhau:

```python title=readonly
n = 5
print(2 * n == 12, 0 * (2 * n) == 0 * 12)

n = 6
print(2 * n == 12, 0 * (2 * n) == 0 * 12)
```

Máy in ra:

```text
False True
True True
```

Cột trái phân biệt được 5 với 6. Cột phải thì không: cả hai đều `True`.

Byte không nói dối câu nào cả — `0 = 0` là một câu **đúng**. Nó chỉ không còn
là câu hỏi cũ nữa. Tập nghiệm đã nở từ "đúng một số" ra "mọi số".

Còn **chia** cho 0 thì thậm chí không có phép ấy để mà hỏng: T2.1 bài 28 đã
chỉ ra rằng `12 : 0` không có kết quả vì bản thân câu hỏi hỏng — đặt đoạn dài
0 bao nhiêu lần cũng không lấp nổi 12.
::::

::::predict{#doan-nhan-khong commitOnce}
Byte lấy `n = 5` — một số **không** phải nghiệm của `2n = 12` — rồi hỏi máy hai
câu: câu gốc, và câu sau khi nhân cả hai đĩa với 0.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 5
print(2 * n == 12)
print(0 * (2 * n) == 0 * 12)
```

:::opt{correct}
False rồi True
:::

:::opt
False rồi False
::why
Gần đúng ở chỗ bạn đang giữ đúng cái luật bài 13: *làm cùng một việc ở cả hai
đĩa thì câu mới trả lời như câu cũ*. Câu gốc sai ở `n = 5`, nên bạn chờ câu mới
cũng sai. Đó là lối nghĩ đúng, và nó đúng với gần như mọi số bạn có thể nhân
vào.

Chỗ lệch là số 0 nằm ngoài cái "gần như" ấy. Nhân với 0 không giữ khoảng cách
giữa hai vế — nó xoá sạch khoảng cách. Vế trái `0 × (2 × 5)` ra 0; vế phải
`0 × 12` cũng ra 0. Máy hỏi "0 có bằng 0 không" và trả lời `True`. `n` đã biến
mất khỏi câu, nên câu chẳng còn phán gì về `n` nữa.
::
:::

:::opt
True rồi True
::why
Gần đúng ở chỗ bạn nhìn ra dòng thứ hai sẽ ra `True` — nó ra `True` thật, và đó
chính là chỗ đắt nhất của bài này.

Chỗ lệch nằm ở dòng thứ nhất. Có lẽ bạn đang đọc `2 * n == 12` như một lời sai
bảo "hãy làm cho hai bên bằng nhau", nên số nào cũng phải ra `True`. Nhưng bài
10 đã đặt tên khác cho dấu `==`: nó là một **câu hỏi Đ/S**, và câu trả lời phụ
thuộc số đang điền. Với `n = 5` thì vế trái là 10, vế phải là 12 — hai số khác
nhau, nên `False`.
::
:::

:::opt
Máy báo lỗi ở dòng thứ hai, vì nhân với 0 rồi so 0 với 0 là chuyện vô nghĩa
::why
Gần đúng ở chỗ bạn nhớ T2.1 bài 28: có những phép mà **bản thân câu hỏi hỏng**,
và máy không cứu được. Linh cảm ấy đúng — nhưng nó đúng với phép **chia** cho
0, không phải phép **nhân** với 0.

Chỗ lệch: nhân với 0 là một phép hoàn toàn làm được, kết quả là 0, không có gì
để máy phàn nàn. Cái hỏng ở đây không nằm trong phép tính mà nằm ở **thông tin
bị mất**: sau khi nhân, câu không còn nhắc gì tới `n` nữa. Máy vẫn vui vẻ trả
`True` — và đúng vì nó trả `True` với mọi `n`, bạn mới biết là mình vừa đánh
rơi câu hỏi.
::
:::
::::

::::explain{#dat-ten-cho-luat}
Đặt tên cho thứ vừa thấy:

> Nhân — hoặc chia — **cả hai vế** cho cùng một số **khác 0** cho ra một
> phương trình mới có **đúng cùng một tập nghiệm**.
>
> Với số **0** thì không: mọi phương trình đều biến thành `0 = 0`, và tập
> nghiệm nở ra thành mọi số.

Chữ "khác 0" ấy không phải một điều lệ ai đó gài vào cho chặt chẽ. Nó là chỗ
bức tranh thanh số đổi hẳn tính chất: mọi số khác 0 kéo giãn hoặc lật, và giãn
hay lật thì hai chỗ khác nhau vẫn khác nhau; riêng số 0 bóp tất cả về một điểm,
nên hai chỗ khác nhau hoá thành một.

Giờ Byte có **hai loại phép giữ nghiệm**, và chúng gỡ hai loại vướng khác nhau:

| Vướng gì ở đĩa trái | Gỡ bằng phép nào | Ví dụ |
|---|---|---|
| có một **lượng** cộng thêm hoặc bớt đi | cộng / trừ cả hai đĩa | `2n + 8 = 20` → `2n = 12` |
| ô trống bị nhân với một **số** | nhân / chia cả hai đĩa | `2n = 12` → `n = 6` |

Cả hai đều là cùng một câu: **làm gì cũng làm cả hai đĩa.** Chỗ khác nhau chỉ
là "làm gì".
::::

::::code{#hai-phep-mot-loi-hua}
Byte muốn máy làm chứng cho cả hai nửa của bài: phép chia đôi thì **giữ** được
câu trả lời, còn phép nhân 0 thì **không**.

Câu đang có là `2n = 12`. Điền bốn chỗ trống:

- Hai chỗ đầu: câu sau khi **chia cả hai đĩa cho 2**. Viết ra thành phép chia
  thật ở cả hai vế — `(2 × n) : 2` bên trái và `12 : 2` bên phải — chứ đừng thu
  gọn hộ máy, vì chính cái việc "chia cả hai" mới là điều bài này đi kiểm.
- Hai chỗ sau: câu sau khi **nhân cả hai đĩa với 0**, cũng viết ra ở cả hai vế.

Bài chấm bằng hai số thử khác nhau: `5` làm câu gốc sai, `6` làm câu gốc đúng.
Gõ cứng `True` hay `False` thì trượt ngay, vì hai số ấy phải cho hai kết quả
khác nhau ở cột "chia đôi" và cùng một kết quả ở cột "nhân 0".

```python title=starter
n = 5
truoc_5 = 2 * n == 12
sau_5 = ___

n = 6
truoc_6 = 2 * n == 12
sau_6 = ___

n = 5
nhan0_5 = ___

n = 6
nhan0_6 = ___

print(truoc_5, sau_5)
print(truoc_6, sau_6)
print(nhan0_5, nhan0_6)
```

```python title=solution
n = 5
truoc_5 = 2 * n == 12
sau_5 = 2 * n / 2 == 12 / 2

n = 6
truoc_6 = 2 * n == 12
sau_6 = 2 * n / 2 == 12 / 2

n = 5
nhan0_5 = 0 * (2 * n) == 0 * 12

n = 6
nhan0_6 = 0 * (2 * n) == 0 * 12

print(truoc_5, sau_5)
print(truoc_6, sau_6)
print(nhan0_5, nhan0_6)
```

```python title=test
# Hai câu `!=` đứng trước: chúng canh hai cái bẫy khác nhau. Xếp sau các câu
# `==` thì chúng không bao giờ chạy tới, và hai cái bẫy không bao giờ sập.
assert sau_5 != sau_6, "chia đôi hai đĩa phải GIỮ được chỗ khác nhau: n = 5 và n = 6 vẫn phải cho hai kết quả Đ/S khác nhau"
assert nhan0_5 != truoc_5, "n = 5 làm câu gốc SAI, mà nhân hai đĩa với 0 lại cho ĐÚNG — đó chính là chỗ nhân 0 làm hỏng, câu của bạn phải để lộ ra"
assert sau_5 == truoc_5, "n = 5: chia cả hai đĩa cho 2 không được đổi câu trả lời Đ/S"
assert sau_6 == truoc_6, "n = 6: chia cả hai đĩa cho 2 không được đổi câu trả lời Đ/S"
assert sau_6, "n = 6 là nghiệm, nên sau khi chia đôi hai đĩa nó vẫn phải làm câu mới ĐÚNG"
assert nhan0_5 and nhan0_6, "sau khi nhân hai đĩa với 0, câu phải ĐÚNG với cả n = 5 lẫn n = 6 — nó đúng với mọi số, và đó là lý do nó vô dụng"
```

:::hints
- kind: attention
  body: Dòng ngay trên mỗi chỗ trống viết câu gốc `2n = 12` thành một câu hỏi Đ/S. Chỗ trống cần đúng một câu như thế, chỉ khác ở chỗ hai vế đều đã bị đem chia cho 2 (hai chỗ đầu) hoặc đem nhân với 0 (hai chỗ sau). Hai chỗ đầu điền giống hệt nhau; hai chỗ sau cũng vậy.
- kind: strategy
  body: Đừng tính hộ máy. Vế trái của câu gốc là `2 * n`, vế phải là `12` — hãy để nguyên hai cụm ấy rồi mới treo phép chia (hoặc phép nhân) vào từng cụm. Cả bốn chỗ trống đều phải còn `n` ở trong, kể cả hai chỗ nhân 0: chính việc `n` vẫn có mặt mà câu vẫn ra `True` mới là điều đáng thấy.
- kind: one-line
  body: "Hai chỗ đầu là `2 * n / 2 == 12 / 2`; hai chỗ sau là `0 * (2 * n) == 0 * 12`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: bốn chỗ trống phải là bốn câu hỏi Đ/S còn `n` ở trong, và phải viết phép chia (hoặc phép nhân 0) ra ở CẢ HAI vế — thu gọn sẵn thành `n == 6` hay `0 == 0` thì cái việc "làm cả hai đĩa" không xuất hiện ở đâu cả
  requireAst:
  # Khung khởi đầu đã có hai dấu `==` (hai câu gốc). Bốn chỗ trống phải thêm
  # bốn dấu nữa. `sau_5 = truoc_5` hay `sau_5 = True` đều không có `==` nào.
  - kind: uses-operator, target: ==, min: 6
  # Phép chia phải xuất hiện ở CẢ HAI vế của hai chỗ trống đầu — bốn dấu `/`.
  # Khung khởi đầu không có dấu chia nào, nên luật này chặn đúng đáp án
  # `n == 6` (đã thu gọn hộ máy, không còn thấy việc chia cả hai đĩa).
  - kind: uses-operator, target: /, min: 4
  # Khung khởi đầu có hai dấu nhân. Hai chỗ nhân 0 phải thêm ít nhất bốn dấu
  # nữa, nên `0 == 0` viết tay không qua được.
  - kind: uses-operator, target: *, min: 6
  # Khung khởi đầu đọc `n` hai lần. `min: 6` buộc cả bốn chỗ trống mỗi chỗ
  # phải đọc `n` — kể cả hai chỗ nhân 0, nơi `n` vẫn phải có mặt dù kết quả
  # không còn phụ thuộc nó.
  - kind: uses-name, target: n, min: 6
  # Số 0 là nhân vật chính của nửa sau bài. Khung khởi đầu không có con số 0
  # nào, nên luật này buộc nó phải được viết ra thật.
  - kind: has-literal, target: 0, min: 2
  forbidAst:
  # Lưới thứ hai: lời giải thật không chứa `True`/`False` viết tay, nên hai
  # luật này chỉ cản đúng người gõ cứng câu trả lời Đ/S.
  - kind: has-literal, target: True
  - kind: has-literal, target: False
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^False False\nTrue True\nTrue True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chia đôi thì hai số vẫn cãi nhau như cũ. Nhân 0 thì chúng hết cãi — vì hết nói
luôn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte giờ có đủ đồ nghề: **hai loại phép** động vào cân mà không đánh mất câu
hỏi. Cộng/trừ cả hai đĩa. Nhân/chia cả hai đĩa cho một số khác 0.

Nhưng có đồ nghề chưa phải là biết làm. Chiều nay câu hỏi ở quán khó hơn hẳn
mấy câu đã gặp:

*Mỗi ổ bán 15 000 đồng. Mỗi buổi Byte trả 30 000 đồng tiền thuê chỗ. Dọn hàng
về, trong túi còn 195 000 đồng. Byte bán được bao nhiêu ổ?*

Viết ra thì đĩa trái có tận hai thứ bám vào ô trống: một phép **nhân** (`15000
×`) và một phép **trừ** (`− 30000`).

`15000 × n − 30000 = 195000`

Gỡ cái nào trước? Hai loại phép đều dùng được, nhưng chúng không đổi chỗ cho
nhau được đâu — thử gỡ phép nhân trước mà xem, con số 30 000 kia sẽ đi theo và
làm phiền bạn.

Vậy **thứ tự nào** thì gỡ được tới lúc ô trống còn đứng một mình trên một đĩa?

Bài sau trả lời — và câu trả lời hoá ra bạn đã biết từ T2.1, chỉ là biết theo
chiều xuôi.
::::

::::checkpoint{mastery=0.8}
::::
