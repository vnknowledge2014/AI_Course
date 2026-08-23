---
id: toan.dai-so-va-ham-so.khi-khong-so-nao-dung
title: Khi không số nào làm nó đúng
summary: Có phương trình mà mọi cách điền đều sai — tập nghiệm rỗng; và đó là chỗ ý nghĩ "chữ là một con số bị giấu" hết đường đứng.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.no-solution]
requires: [math.variable-both-sides, math.check-solution, math.solution-set, math.equation-add-both-sides, math.placeholder-many-values, math.equation, math.multiplication, math.order-of-operations, core.variable, core.assignment, core.arithmetic, core.number-literal, core.print-variable, core.output]
concepts: [math.tap-rong, math.giu-nghiem, math.o-trong, math.bang-gia-tri]
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
Mình dồn chữ về một bên. Rồi chữ biến mất luôn, chỉ còn 0 = 30000.
::::

::::explain{#chu-bien-mat}
Bài trước để lại một câu hỏi. Byte hạ giá bánh mì xuống **9 000 đồng một ổ** —
đúng bằng tiền bột, thịt và than cho một ổ. Tiền thuê chỗ vẫn 30 000 đồng.

Câu hỏi cũ của quán, viết với giá mới:

```text
   9000 × n  =  9000 × n + 30000
```

Làm đúng việc bài trước dạy — bớt cụm `9000 × n` ở cả hai đĩa:

```text
   0  =  30000
```

Không còn chữ nào. Không còn ô trống nào để điền. Mà cũng chưa ai điền gì cả —
suốt chuỗi biến đổi, `n` chưa một lần nhận giá trị nào.

Nếu bạn thấy chỗ này khó chịu thì cảm giác ấy đúng chỗ. Suốt mấy bài vừa rồi,
gỡ một phương trình luôn kết thúc bằng dòng `n = một số`. Lần này dòng cuối
không có chữ `n`, nên nó không phải một đáp số. Nó là một **lời khẳng định**, và
lời khẳng định thì hoặc đúng hoặc sai.

`0 = 30000` là một lời khẳng định **sai**. Sai hẳn, không cần biết `n` bằng bao
nhiêu — vì trong nó không còn `n` nữa.
::::

::::example{#thu-het-moi-so}
Chưa vội tin. Cứ thử.

Câu gốc là `9000 × n = 9000 × n + 30000`. Lập bảng: mỗi dòng một cách điền, hai
cột giữa là hai đĩa, cột cuối đo xem đĩa phải nặng hơn đĩa trái bao nhiêu.

```text
      số ổ n         thu         chi    chi − thu
   ──────────────────────────────────────────────
           0           0       30000        30000
           5       45000       75000        30000
         100      900000      930000        30000
        1000     9000000     9030000        30000
```

Cột cuối không nhúc nhích. Bán 0 ổ hay bán 1000 ổ, đĩa phải luôn nặng hơn đĩa
trái **đúng 30 000**.

Nhìn kỹ thì thấy vì sao, và lý do nằm ngay trong cách viết. Hai đĩa có chung
một mảnh `9000 × n` — mảnh này lớn lên theo số ổ, nhưng lớn **giống hệt nhau ở
cả hai bên**, nên nó không kéo bên nào nặng hơn bên nào. Sau khi nó tự triệt,
cái còn lại là số 30 000 đứng trơ, và số 30 000 thì không đi theo `n`.

Đời thật của cái xe bánh mì nói y hệt: bán một ổ **lãi đúng 0 đồng**. Bán thêm
một ổ nữa cũng lãi 0 đồng. Cộng bao nhiêu số 0 lại thì vẫn là 0, nên 30 000
tiền thuê chỗ không bao giờ được bù.

```python title=readonly
# Thử một con số to hẳn, cho chắc.
n = 1000000
print(9000 * n)
print(9000 * n + 30000)
```

Máy in ra:

```text
9000000000
9000030000
```

Chín tỉ đồng tiền thu, mà vẫn thiếu đúng 30 000. Bảng không bỏ sót gì.
::::

::::predict{#doan-khoang-cach commitOnce}
Byte hỏi thẳng cái khoảng cách giữa hai đĩa, ở một ngày ế và một ngày đông
khách chưa từng có.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n_it = 5
n_nhieu = 1000
print((9000 * n_it + 30000) - 9000 * n_it)
print((9000 * n_nhieu + 30000) - 9000 * n_nhieu)
```

:::opt{correct}
30000 rồi 30000
:::

:::opt
30000 rồi 0
::why
Gần đúng ở chỗ bạn đang dùng một quy tắc rất thật của buôn bán: *bán đủ nhiều
thì bù được khoản chi cố định*. Với xe bánh mì bán 15 000 một ổ thì đúng — bài
trước cho thấy 5 ổ là đủ bù 30 000 thuê chỗ.

Chỗ lệch là điều kiện ngầm của quy tắc ấy: nó chỉ chạy khi **mỗi ổ còn lãi một
chút**. Ở đây giá bán bằng đúng giá vốn, nên phần lãi mỗi ổ là 0. Số ổ nhân với
0 vẫn là 0, nên bán nhiều tới đâu cũng không góp thêm được đồng nào vào việc bù
30 000. Ranh giới nằm ở chỗ phần lãi mỗi ổ có khác 0 hay không.
::
:::

:::opt
30000 rồi 6000000
::why
Gần đúng ở chỗ bạn thấy `n_nhieu` gấp 200 lần `n_it` và cho rằng mọi khoản
trong câu tính lớn lên theo. Quy tắc ấy đúng với phần **đi theo số ổ**: tiền
thu và tiền vốn đều gấp 200 lần thật.

Chỗ lệch: khoảng cách giữa hai đĩa không phải một khoản đi theo số ổ. Phần đi
theo số ổ là `9000 × n`, và nó có mặt **giống hệt nhau ở cả hai đĩa** nên nó tự
triệt khi ta lấy hiệu. Thứ sống sót là 30 000 thuê chỗ — khoản duy nhất không
nhân với `n`.
::
:::

:::opt
0 rồi 0
::why
Gần đúng ở chỗ bạn nhìn ra `9000 × n` xuất hiện hai lần trong mỗi dòng và triệt
lẫn nhau. Quan sát ấy chính xác, và nó chính là điều bài trước dạy.

Chỗ lệch: ngoài `9000 × n` ra, trong ngoặc còn số **30000** nữa, và số ấy không
có ai bên kia để triệt cùng. Ranh giới: phép triệt chỉ xoá được những mảnh **có
đôi**; mảnh lẻ thì ở lại nguyên vẹn.
::
:::
::::

::::explain{#dat-ten}
Đặt tên cho thứ vừa gặp:

> **Vô nghiệm** — có những phương trình mà **không con số nào** điền vào làm
> chúng đúng. Tập nghiệm của chúng **rỗng**: nó không chứa số nào cả.

Ba điều đi kèm:

- **`0 = 30000` là câu trả lời, không phải chỗ bí.** Nó không có nghĩa "chưa gỡ
  xong" hay "đề sai". Nó nói trọn một điều: mọi cách điền đều làm câu gốc hoá
  sai.
- **Bạn đọc được nó như thế chỉ vì mọi bước vừa đi đều giữ nguyên tập nghiệm.**
  Bớt cùng một lượng ở hai đĩa là phép giữ nghiệm, nên câu cuối và câu gốc có
  **cùng** tập nghiệm; câu cuối rỗng thì câu gốc cũng rỗng. Lỡ dùng một phép
  **không** giữ nghiệm ở giữa chừng thì dòng cuối chẳng nói gì về câu gốc hết —
  đúng như bài 14 đã cảnh báo với phép nhân hai vế cho 0.
- **Kiểm nghiệm vẫn dùng được như thường.** Thay số nào vào câu gốc cũng ra hai
  vế lệch nhau đúng 30 000, không bao giờ khớp. Cái bảng ở trên chính là phép
  kiểm nghiệm của bài 16 làm liên tiếp bốn lần.

Và đây là chỗ đáng dừng lâu nhất trong cả mạch này.

Nếu chữ `n` là "một con số bị giấu mà việc của mình là tìm ra", thì câu
`9000 × n = 9000 × n + 30000` đang nói: *có một con số bằng chính nó cộng thêm
30 000*. Đọc kiểu ấy thì câu này thành một câu đố mẹo không lối ra, và bạn sẽ
đi tìm mãi một thứ không có.

Còn nếu chữ `n` là một **ô trống nhận nhiều giá trị** — đúng như bài 1 dựng lên
— thì chẳng có gì bí ẩn. Câu hỏi là *"điền gì thì câu này đúng?"*, và câu trả
lời là *"không gì cả"*. Trọn vẹn, dứt khoát, và có ích: nó cho Byte biết phương
án hạ giá xuống 9 000 không bao giờ huề vốn, khỏi phải thử ngoài đời.

Một tập rỗng vẫn là một tập. "Không có số nào" là một đáp án, không phải một
thất bại.
::::

::::code{#do-khoang-cach-hai-dia}
Byte đo khoảng cách giữa hai đĩa ở hai ngày rất khác nhau: một ngày ế bán 5 ổ,
một ngày đông khách bán 1000 ổ.

Bốn chỗ trống là bốn cái đĩa — hai đĩa của ngày ế, hai đĩa của ngày đông khách.
Bốn con số ấy khác nhau hết (`45000`, `75000`, `9000000`, `9030000`), nên gõ
cứng một con số vào cả bốn thì hỏng ngay dòng in thứ hai.

Hai dòng cuối đã viết sẵn: chúng lấy hiệu hai đĩa. Việc của bạn là dựng đúng
bốn cái đĩa để hai con số ấy nói lên sự thật.

```python title=starter
# Câu của phương án hạ giá:   9000 × n  =  9000 × n + 30000

n_it = 5         # một ngày ế
n_nhieu = 1000   # một ngày đông khách chưa từng có

thu_it = ___     # đĩa trái ngày ế
chi_it = ___     # đĩa phải ngày ế

thu_nhieu = ___  # đĩa trái ngày đông khách
chi_nhieu = ___  # đĩa phải ngày đông khách

thieu_it = chi_it - thu_it
thieu_nhieu = chi_nhieu - thu_nhieu

print(thu_it)
print(chi_it)
print(thu_nhieu)
print(chi_nhieu)
print(thieu_it)
print(thieu_nhieu)
```

```python title=solution
# Câu của phương án hạ giá:   9000 × n  =  9000 × n + 30000

n_it = 5         # một ngày ế
n_nhieu = 1000   # một ngày đông khách chưa từng có

thu_it = 9000 * n_it     # đĩa trái ngày ế
chi_it = 9000 * n_it + 30000     # đĩa phải ngày ế

thu_nhieu = 9000 * n_nhieu  # đĩa trái ngày đông khách
chi_nhieu = 9000 * n_nhieu + 30000  # đĩa phải ngày đông khách

thieu_it = chi_it - thu_it
thieu_nhieu = chi_nhieu - thu_nhieu

print(thu_it)
print(chi_it)
print(thu_nhieu)
print(chi_nhieu)
print(thieu_it)
print(thieu_nhieu)
```

```python title=test
# Ba câu `!=` đứng trước: chúng canh chính điều bài này nói — hai đĩa không bao
# giờ khớp, và khoảng cách không bao giờ co về 0. Xếp chúng sau các câu `==`
# thì chúng không bao giờ chạy tới.
assert thu_it != chi_it, "ngày ế: hai đĩa không khớp, nên 5 không phải nghiệm"
assert thu_nhieu != chi_nhieu, "ngày bán 1000 ổ: hai đĩa vẫn không khớp, nên 1000 cũng không phải nghiệm"
assert thieu_nhieu != 0, "bán gấp 200 lần mà khoảng cách vẫn chưa khép — không số ổ nào khép được nó"
assert thieu_it == thieu_nhieu, "khoảng cách hai đĩa không đổi theo số ổ: phần `9000 × n` có mặt ở cả hai bên nên tự triệt"
assert thieu_it == 30000, "thứ sống sót là 30 000 thuê chỗ — khoản duy nhất không đi theo số ổ"
assert thu_it == 45000, "9000 × 5 = 45000 đồng tiền thu ngày ế"
assert chi_it == 75000, "9000 × 5 + 30000 = 75000 đồng tiền chi ngày ế"
assert thu_nhieu == 9000000, "9000 × 1000 = 9000000 đồng tiền thu ngày đông khách"
assert chi_nhieu == 9030000, "9000 × 1000 + 30000 = 9030000 đồng tiền chi ngày đông khách"
```

:::hints
- kind: attention
  body: Dòng chú thích trên cùng đã viết sẵn cả hai đĩa của câu gốc. Đĩa trái là phần bên trái dấu `=`, đĩa phải là phần bên phải. Bốn chỗ trống chỉ là hai đĩa ấy viết hai lần — một lần với `n_it`, một lần với `n_nhieu`.
- kind: strategy
  body: Chép lại từng đĩa, chỗ nào có chữ `n` thì đặt tên của ngày hôm đó vào — đừng tính nhẩm rồi gõ con số, vì bốn con số ấy chính là thứ đang nhờ máy tìm hộ. Hai chỗ trống của đĩa trái không có 30000; hai chỗ của đĩa phải đều có.
- kind: one-line
  body: "Bốn chỗ trống lần lượt là `9000 * n_it`, `9000 * n_it + 30000`, `9000 * n_nhieu` và `9000 * n_nhieu + 30000`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: bốn chỗ trống phải dựng từ tên của ngày hôm đó, không phải từ con số đã tính sẵn — gõ cứng bốn con số thì cái bảng này không còn chứng minh được gì
  requireAst:
  # Bốn cái đĩa, bốn phép nhân với số ổ. Khung khởi đầu không có dấu `*` nào.
  - kind: uses-operator, target: *, min: 4
  # Hai đĩa phải đều có 30 000 thuê chỗ cộng vào. Thiếu dấu cộng nghĩa là hai
  # đĩa bị dựng giống hệt nhau, và lúc đó khoảng cách ra 0 vì một lý do sai.
  - kind: uses-operator, target: +, min: 2
  # Mỗi ngày phải được ĐỌC từ cái tên của nó, hai lần — một lần cho mỗi đĩa.
  # Gõ `9000 * 5` cũng ra 45000, nhưng lúc đó ô trống đã bị thay bằng tay.
  - kind: uses-name, target: n_it, min: 2
  - kind: uses-name, target: n_nhieu, min: 2
  forbidAst:
  # Lưới thứ hai: bốn con số KẾT QUẢ. Lời giải thật dựng chúng từ hai cái tên
  # nên không chứa nguyên văn cái nào; đáp án chép cứng thì chứa đủ bốn.
  - kind: has-literal, target: 45000
  - kind: has-literal, target: 75000
  - kind: has-literal, target: 9000000
  - kind: has-literal, target: 9030000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^45000\n75000\n9000000\n9030000\n30000\n30000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bán bao nhiêu ổ cũng thiếu đúng 30 000. Không có số nào cứu được câu này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa gặp một câu mà **không** cách điền nào làm nó đúng: gỡ tới cuối thì còn
`0 = 30000`, một lời khẳng định sai, nên tập nghiệm rỗng.

Câu hỏi ngược lại thì sao — có câu nào **điền số nào vào cũng đúng** không?

Nghĩ về chỗ nó có thể mọc ra. Sáng mai xe bánh mì có hai người cùng đứng bán,
và mỗi người tính tiền một kiểu: người gõ từng món rồi cộng lại, người gõ gộp
cả suất cho nhanh. Hai cách gõ khác hẳn nhau trên giấy, mà cả hai đều tính đúng
tiền của cùng một buổi.

Đặt dấu `=` vào giữa hai cách gõ ấy — như bài 10 đã dạy, dấu `=` là một câu hỏi
— rồi gỡ nó bằng đúng những phép vừa dùng hôm nay. Chữ sẽ lại biến mất sạch.
Nhưng lần này thứ còn lại **không** phải một lời khẳng định sai.

Nếu dòng cuối là một lời khẳng định **đúng** thì nó nói gì về câu gốc? Có bao
nhiêu số điền vào được — không số nào, một số, hay nhiều hơn thế?

Bài sau nhận đúng câu hỏi này.
::::

::::checkpoint{mastery=0.8}
::::
