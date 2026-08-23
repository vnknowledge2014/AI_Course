---
id: toan.dai-so-va-ham-so.dau-bang-la-mot-cau-hoi
title: Dấu bằng đặt ra một câu hỏi
summary: Hai câu tính nối bằng dấu `=` không ra lệnh tính gì cả — nó nêu một lời khẳng định, và mỗi lần điền số vào ô trống thì lời ấy hoá đúng hoặc hoá sai.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.equation]
requires: [math.expression, math.substitution, math.letter-names-a-slot, math.placeholder-many-values, math.value-table, math.multiplication, math.order-of-operations, core.boolean, ctrl.comparison, core.variable, core.assignment, core.print-variable, core.arithmetic, core.number-literal, core.output]
concepts: [math.o-trong, math.bieu-thuc, math.phuong-trinh, math.xe-banh-mi]
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
Chín bài rồi mà chưa ai hỏi mình bán được mấy ổ. Trưa nay thì phải hỏi.
::::

::::explain{#quan-hoi-mot-cau}
Chín bài vừa rồi có một điều lạ mà bài trước đã chỉ ra: **chưa ai hỏi `n` bằng
bao nhiêu.** Bạn viết `15000 × n`, bạn điền số vào, bạn lập bảng, bạn mở ngoặc,
bạn rút cái chung ra, bạn gộp hai cụm cùng chữ. Cách nào thì câu tính cũng vẫn
còn ô trống, và ô trống thì vẫn nhận được nhiều số.

Trưa nay quán hỏi một câu khác hẳn.

Byte dọn xe về. Trong túi tiền có đúng **300 000 đồng**, toàn tiền bánh mì, không
lẫn thứ gì khác. Mỗi ổ bán **15 000 đồng**. Byte không đếm số ổ đã trao tay —
nhưng bây giờ thì muốn biết.

Trước khi tính, phải **viết câu hỏi của quán thành một câu tính**. Đây là một
việc riêng, làm xong rồi mới tới lượt toán:

- Thứ chưa biết là **số ổ đã bán**. Dán cho nó một cái tên: `n`.
- Bán `n` ổ, mỗi ổ 15 000 đồng thì thu về `15000 × n` đồng. Đây vẫn là một câu
  tính đang chờ — chưa phải một con số, vì `n` chưa được điền.
- Còn `300000` là một con số đàng hoàng: đếm được, nằm trong túi, không chờ ai.

Câu hỏi của quán, viết ra cho gọn, là: **`15000 × n` có đúng bằng `300000`
không?**

```text
15000 × n = 300000
```

Và ở đúng dòng ấy, dấu `=` làm một việc mà nó chưa từng làm trong track này.
::::

::::explain{#dau-bang-khong-ra-lenh}
Từ bé tới giờ, dấu `=` gần như lúc nào cũng đứng ở **cuối** một phép tính:

```text
7 × 8 = 56
```

Đọc quen tay thì nó thành một cái nút bấm: bên trái là việc phải làm, bên phải
là chỗ ghi kết quả. Nhưng đem cách đọc ấy áp vào dòng của Byte thì hỏng ngay:

```text
15000 × n = 300000
```

Bên trái **chưa làm được** — còn ô trống. Bên phải thì **đã có sẵn** kết quả rồi,
chẳng ai bảo tính nó ra. Nếu `=` là nút bấm thì dòng này vô nghĩa. Mà nó không
vô nghĩa: nó đúng là câu Byte đang hỏi.

Vậy `=` ở đây đóng vai gì? Nó nối hai câu tính và **khẳng định rằng hai bên bằng
nhau**. Nó không sai khiến ai làm gì. Nó *nói một điều*.

Chỗ khác nhau ấy quan trọng, vì một lời nói thì có thể **đúng**, có thể **sai** —
còn một mệnh lệnh thì không. "Tính 7 × 8 đi" không đúng cũng không sai. "Hai bên
này bằng nhau" thì đúng hoặc sai, tuỳ.

Tuỳ **cái gì**? Tuỳ số điền vào ô trống.
::::

::::example{#moi-lan-dien-mot-loi-phan}
Byte thử vài số. Mỗi dòng là một lần điền, và mỗi lần điền cho một **lời phán**
về dòng `15000 × n = 300000`:

| điền `n` | `15000 × n` ra | so với `300000` | lời khẳng định |
|---|---|---|---|
| 10 | 150 000 | còn thiếu | SAI |
| 19 | 285 000 | còn thiếu | SAI |
| 20 | 300 000 | đúng bằng | **ĐÚNG** |
| 21 | 315 000 | quá tay | SAI |

Bảng này trông giống bảng giá trị ở bài 5, nhưng cột cuối là một thứ khác hẳn.
Bảng bài 5 có cột kết quả — những **con số**. Bảng này có cột ĐÚNG / SAI.

Đặt hai dòng cạnh nhau cho thấy rõ:

| dòng | điền 21 vào thì ra gì | hỏi "dòng này đúng hay sai" |
|---|---|---|
| `15000 × n` | ra số 315 000 | hỏi hỏng — một con số không đúng cũng không sai |
| `15000 × n = 300000` | ra một lời phán: SAI | hỏi được, và trả lời được |

Một câu tính còn ô trống thì điền vào sẽ ra **một con số**. Hai câu tính nối bằng
`=` thì điền vào sẽ ra **một lời phán**. Cùng một ô trống, cùng một cách điền,
mà thứ rơi ra ở cuối thuộc hai loại khác nhau.

Bạn đã gặp loại "đúng hay sai" ấy rồi. Ở Realm 0, Byte hỏi máy `7 > 5` và máy trả
lời `True`. Python cẩn thận tới mức dùng **hai dấu khác nhau** cho hai vai mà
toán gộp chung vào một dấu `=`:

```python title=readonly
# một dấu bằng: ĐẶT TÊN. Đây là một mệnh lệnh, không đúng cũng không sai.
gia_mot_o = 15000

# hai dấu bằng: HỎI. Đây là một câu hỏi, và máy trả lời đúng hoặc sai.
print(gia_mot_o * 20 == 300000)
print(gia_mot_o * 21 == 300000)
```

Máy in ra:

```text
True
False
```

Trong vở toán chỉ có một dấu `=`, gánh cả hai vai — nên người đọc phải tự nhìn
chỗ nó đứng mà biết nó đang làm vai nào. `15000 × n = 300000` là vai thứ hai.
::::

::::predict{#doan-hai-dau-bang commitOnce}
Byte muốn máy phán về hai con số cùng lúc. Cả hai dòng đều dùng **hai** dấu bằng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
gia_mot_o = 15000
print(gia_mot_o * 20 == 300000)
print(gia_mot_o * 21 == 300000)
```

:::opt{correct}
`True` rồi `False`
:::

:::opt
`300000` rồi `315000`
::why
Gần đúng ở chỗ hai con số bạn viết ra không sai một đồng nào: `15000 × 20` đúng
là 300 000, `15000 × 21` đúng là 315 000. Bạn tính trúng cả hai. Và quy tắc bạn
đang dùng — *dấu bằng là chỗ ra kết quả* — là quy tắc đúng suốt từ hồi học phép
nhân, và nó vẫn đúng ở dòng đầu tiên: `gia_mot_o = 15000` đặt tên cho con số
15 000.

Ranh giới nằm ở chỗ có **một** hay **hai** dấu bằng. Một dấu là đặt tên, và nó
không nhả ra thứ gì để in. Hai dấu là một câu hỏi so sánh, và thứ nó nhả ra
không phải con số mà là lời phán về con số: `True` hoặc `False`. Bạn đã tính
xong phần khó rồi — chỉ còn bước cuối: đem 300 000 vừa tính so với 300 000 thì
được gì, đem 315 000 so với 300 000 thì được gì.
::
:::

:::opt
`True` rồi `True`
::why
Gần đúng ở chỗ bạn đọc `==` trúng vai của nó: nó hỏi, và câu trả lời là `True`
hoặc `False`. Bạn đã qua được chỗ khó nhất của bài này.

Chỗ lệch: có lẽ bạn đang đọc `==` thành "hai bên này có **cùng kiểu** không" hay
"câu này có viết đúng luật không". Cả hai dòng đúng là viết đúng luật, và cả hai
vế đều là số — theo cách đọc ấy thì `True` cả hai là hợp lý. Nhưng `==` hỏi một
câu hẹp hơn nhiều: *hai bên có đúng cùng một giá trị không*. 315 000 và 300 000
đều là số, đều hợp lệ, và vẫn khác nhau 15 000 đồng — đúng một ổ bánh mì. Nên
dòng thứ hai phải là `False`.
::
:::

:::opt
`False` rồi `False`
::why
Gần đúng ở chỗ bạn thấy hai vế **trông** chẳng giống nhau: một bên là một phép
nhân, một bên là con số `300000` trơ trọi. Cảnh giác ấy chính là thứ chín bài
vừa rồi dựng lên — một câu tính còn ô trống thì chưa phải một con số, nên đừng
vội coi nó bằng với một con số.

Ranh giới: `n` **đã được điền** rồi. `gia_mot_o` đang giữ 15 000, và chỗ trống
đã nhận con số 20 — nên `gia_mot_o * 20` thu lại thành đúng một con số trước
khi `==` kịp hỏi gì. `==` so hai **giá trị** sau khi mỗi bên tính xong, chứ
không so mặt chữ. Chưa điền mới là chưa ra số; điền rồi thì so được.
::
:::
::::

::::explain{#dat-ten-phuong-trinh}
Đặt tên cho thứ vừa dựng, để mang đi được:

> **Phương trình** là hai câu tính nối với nhau bằng dấu `=`. Nó không phải một
> phép tính phải làm, nó là một **lời khẳng định**: hai bên bằng nhau. Mỗi lần
> điền một số vào ô trống, lời khẳng định ấy hoá **đúng** hoặc hoá **sai**.

Ba thứ đã có từ trước và không đổi tí nào:

- **Ô trống vẫn là ô trống** (bài 1). Nó vẫn nhận được nhiều số. Không có gì
  trong dấu `=` biến `n` thành một con số bị giấu.
- **Điền là điền vào mọi chỗ có chữ ấy** (bài 4).
- **Hai vế vẫn là hai câu tính** (bài 3), nên mọi luật đổi hình dạng ở bài 7–9
  vẫn dùng được trên từng vế.

Thứ mới đúng một: cái **kết cục**. Trước bài này, điền số vào thì ra một con số.
Từ bài này, điền số vào một phương trình thì ra một lời phán.

Và vì là lời phán nên có chỗ để hỏi tiếp: *lời phán ấy phụ thuộc số nào?* Với
`15000 × n = 300000`, điền 20 thì ĐÚNG, điền 19 hay 21 thì SAI. Con số 20 không
"bị giấu" ở đâu cả — nó chỉ là số **làm cho lời khẳng định ấy đúng**.
::::

::::byte{trigger=enter mood=thinking pose=point-editor}
Mình có ba con số nghi ngờ. Nhờ máy phán hộ từng cái, đừng phán hộ mình.
::::

::::code{#may-phan-ba-loi-khang-dinh}
Byte không muốn tự tính rồi tự kết luận — dễ nhầm, và nhầm thì không biết mình
nhầm. Byte muốn **máy làm trọng tài**: đưa cho nó ba lời khẳng định, nó phán từng
cái.

Ba lời khẳng định, cùng nói về một túi tiền 300 000 đồng:

- "Trưa nay mình bán **19** ổ."
- "Trưa nay mình bán **20** ổ."
- "Trưa nay mình bán **21** ổ."

Điền ba chỗ trống. Mỗi chỗ phải là một **câu hỏi so sánh** viết bằng `==`, dựng
từ hai cái tên đã có sẵn phía trên — chứ không phải một con số bạn tự tính rồi
gõ vào, cũng không phải chữ `True`/`False` bạn tự phán rồi gõ vào. Gõ tay thì
trọng tài là bạn, và cái bạn đang muốn kiểm lại chính là mình.

Bài chấm bằng **cả ba** lời khẳng định, và chúng cố ý cho ra hai kết cục khác
nhau: một cái ĐÚNG, hai cái SAI. Một câu trả lời chép cứng chỉ qua nổi nhiều
nhất một dòng.

```python title=starter
gia_mot_o = 15000
tien_dem_duoc = 300000

# "Trưa nay mình bán 19 ổ" — đúng hay sai?
loi_khang_dinh_19 = ___

# "Trưa nay mình bán 20 ổ" — đúng hay sai?
loi_khang_dinh_20 = ___

# "Trưa nay mình bán 21 ổ" — đúng hay sai?
loi_khang_dinh_21 = ___

print(loi_khang_dinh_19)
print(loi_khang_dinh_20)
print(loi_khang_dinh_21)
```

```python title=solution
gia_mot_o = 15000
tien_dem_duoc = 300000

# "Trưa nay mình bán 19 ổ" — đúng hay sai?
loi_khang_dinh_19 = gia_mot_o * 19 == tien_dem_duoc

# "Trưa nay mình bán 20 ổ" — đúng hay sai?
loi_khang_dinh_20 = gia_mot_o * 20 == tien_dem_duoc

# "Trưa nay mình bán 21 ổ" — đúng hay sai?
loi_khang_dinh_21 = gia_mot_o * 21 == tien_dem_duoc

print(loi_khang_dinh_19)
print(loi_khang_dinh_20)
print(loi_khang_dinh_21)
```

```python title=test
# Hai câu `!=` đứng TRƯỚC. Chúng canh cái bẫy "ba dòng giống hệt nhau": nếu ba
# lời phán ra như một thì bẫy sập ngay ở đây, trước khi mấy câu `==` phía dưới
# kịp che nó đi.
assert loi_khang_dinh_19 != loi_khang_dinh_20, "bán 19 ổ và bán 20 ổ không thể cùng một lời phán — 285 000 đồng khác 300 000 đồng"
assert loi_khang_dinh_21 != loi_khang_dinh_20, "bán 21 ổ và bán 20 ổ không thể cùng một lời phán — 315 000 đồng khác 300 000 đồng"
assert loi_khang_dinh_20 == True, "15 000 × 20 đúng bằng 300 000, nên lời khẳng định 'bán 20 ổ' phải ĐÚNG"
assert loi_khang_dinh_19 == False, "15 000 × 19 = 285 000, còn thiếu 15 000 đồng, nên lời khẳng định này SAI"
assert loi_khang_dinh_21 == False, "15 000 × 21 = 315 000, quá 15 000 đồng, nên lời khẳng định này SAI"
```

:::hints
- kind: attention
  body: Nhìn hai dòng đầu tiên. Ở đó có hai cái tên đã giữ sẵn số: một cái giữ giá một ổ, một cái giữ số tiền trong túi. Mỗi chỗ trống cần cả hai cái tên ấy, cộng thêm con số ổ ghi trong câu ngay phía trên nó. Và đếm lại xem một chỗ trống cần mấy dấu bằng — một hay hai.
- kind: strategy
  body: Mỗi chỗ trống là một phương trình đã điền sẵn số, đem hỏi máy. Vế trái là tiền thu khi bán bấy nhiêu ổ, dựng bằng giá một ổ nhân với số ổ. Vế phải là số tiền đếm được. Nối hai vế bằng `==` để hỏi "hai bên có đúng bằng nhau không" — đừng tự nhân ra rồi gõ kết quả, vì lúc đó máy chẳng còn gì để phán.
- kind: one-line
  body: "Dòng đầu là `gia_mot_o * 19 == tien_dem_duoc`; hai dòng sau y hệt, chỉ thay 19 bằng 20 rồi 21."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một câu hỏi so sánh viết bằng `==`, dựng từ `gia_mot_o` và `tien_dem_duoc` — gõ thẳng `True`/`False` hay gõ thẳng con số thì bạn đã tự phán hộ máy rồi
  requireAst:
  # Ba chỗ trống, ba câu hỏi so sánh. Khung khởi đầu không có dấu `==` nào, nên
  # luật này chặn được đúng cái đáp án tự phán rồi gõ vào.
  - kind: uses-operator, target: ==, min: 3
  # Dấu `==` thôi thì chưa đủ: `285000 == 300000` cũng là một câu hỏi so sánh
  # mà chẳng đọc cái tên nào — người viết nó đã tự nhân hộ máy. Hai luật dưới
  # buộc cả hai vế phải dựng từ tên. Khung khởi đầu không ĐỌC tên nào trong hai
  # tên này (dòng gán không tính là đọc), nên `min: 3` phân biệt được.
  - kind: uses-name, target: gia_mot_o, min: 3
  - kind: uses-name, target: tien_dem_duoc, min: 3
  # Phép nhân phải có mặt: số ổ nhân giá một ổ mới ra tiền thu.
  - kind: uses-operator, target: *, min: 3
  forbidAst:
  # Lưới thứ hai, chặn đúng ba thứ có thể chép cứng: hai kết quả nhân sẵn và
  # lời phán gõ tay. Lời giải thật không chứa nguyên văn cái nào.
  - kind: has-literal, target: 285000
  - kind: has-literal, target: 315000
  - kind: has-literal, target: True
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^False\nTrue\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sai, đúng, sai. Mình không đoán nữa — mình có ba lời phán hẳn hoi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa làm hai việc khác nhau, và việc thứ nhất dễ bị bỏ qua vì nó không có
phép tính nào: **dịch câu hỏi của quán thành một câu tính**. "Túi có 300 000
đồng, mỗi ổ 15 000, bán mấy ổ?" → `15000 × n = 300000`. Không có bước ấy thì
chẳng có gì để giải.

Bây giờ tới lượt bạn dịch. Hôm hội chợ, Byte bán chạy hơn hẳn: cuối buổi đếm
được **1 200 000 đồng**, vẫn giá 15 000 đồng một ổ.

- Câu hỏi ấy viết thành phương trình nào?
- Và câu hỏi thật sự khó: với dòng của trưa nay, `15000 × n = 300000`, bạn đã tìm
  ra 20 làm nó đúng. **Còn số nào khác cũng làm nó đúng nữa không?** Byte mới thử
  bốn con số trong bảng — 10, 19, 20, 21. Còn cả một dãy số chưa ai đụng tới.

Nói "đáp án là 20" nghe rất chắc chắn. Nhưng bạn mới kiểm bốn số. Có **bao
nhiêu** số làm dòng ấy đúng, và làm sao biết mình đã tìm đủ?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
