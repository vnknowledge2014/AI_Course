---
id: toan.dai-so-va-ham-so.cham-ca-bang-len-mat-phang
title: Chấm cả bảng lên mặt phẳng
summary: Chín dòng bảng thành chín dấu chấm, và chúng không rơi bừa bãi — cái hình hiện ra là khuôn mặt thứ ba của đúng một câu tính.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.graph-of-expression]
requires: [math.ordered-pair, math.coordinate-plane, math.value-table, math.substitution, math.expression, math.multiplication, math.order-of-operations, core.variable, core.assignment, core.arithmetic, core.number-literal, core.print-variable, core.boolean, ctrl.comparison]
concepts: [math.do-thi, math.bang-gia-tri, math.xe-banh-mi]
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
Chín dòng bảng, chín dấu chấm. Xem chúng rơi vào đâu đã.
::::

::::explain{#bay-dong-bay-diem}
Bài trước dựng xong hai trục và chấm được một điểm. Bài này chấm cả bảng.

Bảng ở bài 5 sinh ra từ đúng một câu tính — `15000 × n`. Mỗi lần điền một số ổ
vào chỗ trống là được một dòng, và bảng ấy chạy từ 0 ổ tới 8 ổ, chín dòng:

| số ổ (n) | tiền thu = 15000 × n | cặp toạ độ |
| --- | --- | --- |
| 0 | 0 | (0; 0) |
| 1 | 15000 | (1; 15000) |
| 2 | 30000 | (2; 30000) |
| 3 | 45000 | (3; 45000) |
| 4 | 60000 | (4; 60000) |
| 5 | 75000 | (5; 75000) |
| 6 | 90000 | (6; 90000) |
| 7 | 105000 | (7; 105000) |
| 8 | 120000 | (8; 120000) |

Cột thứ ba không thêm thông tin nào — nó chỉ chép lại hai cột kia theo đúng quy
ước của bài trước: số viết trước đi ngang, số viết sau đi lên. Mỗi dòng bảng
thành một cặp, và mỗi cặp thành một dấu chấm.

Chín dòng thì chấm chín lần.
::::

::::example{#cham-het-len-mot-to-giay}
```text
  tiền thu (đồng)
   ↑
120000│                       ●
105000│                    ●
 90000│                 ●
 75000│              ●
 60000│           ●
 45000│        ●
 30000│     ●
 15000│  ●
     0└──┴──┴──┴──┴──┴──┴──┴──┴──→ số ổ (n)
      0  1  2  3  4  5  6  7  8
```

Dòng đầu tiên của bảng — `(0; 0)`, bán 0 ổ thu 0 đồng — rơi đúng vào chỗ hai
trục cắt nhau, nên trên hình nó nằm lẫn trong cái góc ấy. Tám dấu chấm còn lại
thì thấy rõ.

Và chúng không rơi bừa bãi. Đặt cạnh một cây thước lên tờ giấy: cả chín dính
vào cùng một đường **thẳng**.

Chuyện đó không hiện ra ở đâu trong cái bảng. Bảng là một cột số xếp dọc; nhìn
mãi cũng chỉ thấy chúng "lớn dần". Còn tờ giấy thì nói ngay, chỉ bằng hình
dạng, rằng cách chúng lớn dần có một nếp rất chặt.

*(Vì sao lại thẳng — chứ không cong, không gãy khúc — thì để dành. Hôm nay chỉ
cần nhìn thấy nó thẳng đã.)*

Cái hình vừa hiện ra có tên: **đồ thị** của biểu thức `15000 × n`.

Chú ý mấy chữ "của biểu thức". Chín dấu chấm sinh ra từ chín dòng bảng, mà chín
dòng bảng sinh ra từ đúng một câu tính. Nên bây giờ bạn có ba thứ trong tay:

- **câu tính** `15000 × n` — gọn nhất, mang đi được, nhưng chưa ra số nào;
- **bảng** — nói rõ từng trường hợp một, nhưng chỉ những dòng đã viết ra;
- **hình** — nói hình dạng của cả bảng trong một cái liếc mắt.

Ba khuôn mặt của cùng một vật. Đổi một con số trong câu tính — chẳng hạn Byte
lên giá, `15000` thành `20000` — thì cả bảng lẫn hình đổi theo, không cách nào
giữ cái này mà bỏ cái kia.
::::

::::explain{#doc-hinh-thay-cho-tra-bang}
Đồ thị dùng để **tra**, y như bảng, nhưng bằng động tác chứ không bằng mắt dò
từng dòng:

> Đi ngang tới số ổ cần hỏi. Ngước thẳng lên tới khi gặp dấu chấm. Rẽ ngang
> sang trục đứng. Đọc con số ở đó.

Hỏi "bán 4 ổ thì thu bao nhiêu": đi ngang tới 4, ngước lên gặp dấu chấm, rẽ
sang trái, đọc 60000. Đúng bằng dòng thứ tư của bảng.

Có một chỗ đồ thị làm được mà bảng chín dòng thì không: nó gợi ra cả những chỗ
**giữa** hai dấu chấm — chỗ ứng với 2 ổ rưỡi chẳng hạn. Chỗ ấy có nghĩa gì hay
không thì còn tuỳ hàng: nửa ổ bánh mì thì Byte không bán, nhưng nửa cân bột thì
hoàn toàn cân được. Tờ giấy cứ mở ra chỗ đó; nhận hay không là việc của người
đọc, không phải của tờ giấy.
::::

::::predict{#doan-nam-chinh-giua commitOnce}
"Cả chín dính vào một đường thẳng" — nói bằng cây thước thì được, nhưng bảo máy
kiểm thì kiểm bằng cách nào? Máy không có thước.

Có một cách hỏi mà máy trả lời được. Lấy **ba** số ổ **cách đều nhau**. Nếu ba
dấu chấm của chúng nằm trên một đường thẳng thì dấu chấm giữa phải rơi đúng
**chính giữa** hai dấu chấm kia theo chiều đứng — tức là hai lần chiều cao của
nó phải bằng tổng hai chiều cao kia.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
gia = 15000   # đồng một ổ

# ba số ổ cách đều nhau: 2, 3, 4
print(gia * 3 * 2 == gia * 2 + gia * 4)

# ba số ổ cũng cách đều nhau, chỉ thưa hơn: 1, 4, 7
print(gia * 4 * 2 == gia * 1 + gia * 7)

# ba số ổ KHÔNG cách đều nhau: 1, 2, 6
print(gia * 2 * 2 == gia * 1 + gia * 6)
```

:::opt{correct}
True, rồi True, rồi False
:::

:::opt
True, rồi True, rồi True
::why
Gần đúng ở chỗ bạn đã nắm được điều quan trọng nhất của bài: cả chín dấu chấm
nằm trên một đường thẳng, nên **lấy ba cái nào ra cũng thẳng hàng** — kể cả bộ
ba 1, 2, 6. Điều đó đúng, và bạn không nghĩ sai về cái hình.

Chỗ lệch nằm ở chỗ ba dòng trên **không** kiểm "thẳng hàng". Chúng kiểm một
việc hẹp hơn: "dấu chấm giữa có rơi đúng chính giữa hai dấu chấm kia không".
Hai chuyện ấy chỉ trùng nhau khi ba số ổ cách đều nhau. Ở dòng ba, từ 1 tới 2
là một bước mà từ 2 tới 6 là bốn bước: dấu chấm ở n = 2 vẫn nằm trên đường
thẳng, nhưng nó lệch hẳn về bên trái chứ không ở chính giữa. Phép thử này là
một cái thước đo, và cái thước nào cũng có phạm vi dùng của nó.
::
:::

:::opt
True, rồi False, rồi False
::why
Gần đúng ở chỗ bạn thấy 2, 3, 4 nằm sát nhau nên tin dòng đầu, còn 1, 4, 7 cách
nhau xa nên đâm ngờ. Sự thận trọng ấy đúng ở ngoài đời: đo càng dài thì sai số
càng dồn, và ba cái cọc cách nhau ba cây số thì khó thẳng hàng hơn ba cái cọc
cách nhau ba mét.

Chỗ lệch là ở đây không có phép **đo** nào cả. Mỗi con số trong bảng không được
đo ra mà được **tính ra** từ đúng một câu tính `15000 × n`, nên không có sai số
nào để dồn lại. Điều duy nhất phép thử đòi hỏi là ba số ổ cách đều nhau — mà 1,
4, 7 cách đều thật (mỗi bước 3 ổ), y hệt 2, 3, 4 cách đều (mỗi bước 1 ổ). Xa
hay gần không đổi gì.
::
:::

:::opt
False, rồi False, rồi False
::why
Gần đúng ở chỗ bạn nhìn hai vế của mỗi dòng và thấy chúng là hai câu tính có
hình dạng khác hẳn nhau: một bên nhân ba con số, một bên cộng hai tích. Dè dặt
trước hai câu tính trông khác nhau là một thói quen tốt.

Chỗ lệch là bài 6 của track này đã dựng sẵn đúng chỗ ấy: hai biểu thức trông
khác nhau vẫn có thể ra cùng một số ở **mọi** giá trị điền vào. Thử tay dòng
đầu: `15000 × 3 × 2` là 90000, còn `15000 × 2 + 15000 × 4` là 30000 + 60000,
cũng 90000. Chúng không tình cờ bằng nhau — chúng là hai cách viết của cùng một
lượng, đúng theo luật mở ngoặc ở bài 7.
::
:::
::::

::::explain{#thang-la-mot-quan-sat}
Máy vừa xác nhận điều cây thước đã nói, trên hai bộ ba khác nhau. Đó chưa phải
một chứng minh cho **mọi** bộ ba — bạn mới thử hai. Nhưng nó đủ để bài này giữ
lại đúng một câu:

> Đồ thị của `15000 × n` là những dấu chấm nằm thẳng hàng.

Câu ấy nói về **hình dạng**, và hình dạng là thứ mà cả biểu thức lẫn bảng đều
giấu kín. Từ đây trở đi, mỗi khi gặp một câu tính lạ, bạn có thêm một việc để
làm ngoài chuyện điền số: chấm nó ra giấy rồi nhìn xem nó có hình gì.
::::

::::code{#do-ba-dau-cham}
Lấy ba dòng của bảng, chọn ba số ổ cách đều nhau: `2`, `5`, `8`.

Điền bốn chỗ trống để máy dựng ba chiều cao rồi tự kiểm xem dấu chấm giữa có
rơi đúng chính giữa hai dấu chấm kia không.

Bài chấm bằng cả ba chiều cao và câu trả lời cuối, nên gõ cứng `30000`, `75000`
hay `120000` vào là hỏng: những con số ấy chính là thứ bạn đang nhờ máy tính
hộ.

```python title=starter
gia_mot_o = 15000

# Ba số ổ cách đều nhau, mỗi bước 3 ổ.
so_o_dau = 2
so_o_giua = 5
so_o_cuoi = 8

tien_dau = ___
tien_giua = ___
tien_cuoi = ___

# Dấu chấm giữa có rơi đúng CHÍNH GIỮA hai dấu chấm kia không?
nam_chinh_giua = ___

print(tien_dau)
print(tien_giua)
print(tien_cuoi)
print(nam_chinh_giua)
```

```python title=solution
gia_mot_o = 15000

# Ba số ổ cách đều nhau, mỗi bước 3 ổ.
so_o_dau = 2
so_o_giua = 5
so_o_cuoi = 8

tien_dau = gia_mot_o * so_o_dau
tien_giua = gia_mot_o * so_o_giua
tien_cuoi = gia_mot_o * so_o_cuoi

# Dấu chấm giữa có rơi đúng CHÍNH GIỮA hai dấu chấm kia không?
nam_chinh_giua = tien_dau + tien_cuoi == tien_giua * 2

print(tien_dau)
print(tien_giua)
print(tien_cuoi)
print(nam_chinh_giua)
```

```python title=test
# Hai câu `!=` đứng trước: điền cùng một thứ vào cả bốn chỗ trống thì ba chiều
# cao hoá ra bằng nhau, và mọi câu `==` phía sau sẽ không bao giờ chạy tới.
assert tien_dau != tien_giua, "hai số ổ khác nhau phải cho hai chiều cao khác nhau — bằng nhau nghĩa là chỗ trống chưa đọc số ổ"
assert tien_giua != tien_cuoi, "5 ổ và 8 ổ không thể thu về cùng một số tiền"
assert tien_dau == 30000, "2 ổ × 15000 đồng = 30000 đồng"
assert tien_giua == 75000, "5 ổ × 15000 đồng = 75000 đồng"
assert tien_cuoi == 120000, "8 ổ × 15000 đồng = 120000 đồng"
assert nam_chinh_giua == True, "30000 + 120000 = 150000, đúng bằng hai lần 75000 — dấu chấm giữa rơi chính giữa, ba dấu chấm thẳng hàng"
```

:::hints
- kind: attention
  body: Ba chỗ trống đầu hỏi cùng một câu, chỉ khác số ổ: bán bấy nhiêu ổ thì thu bao nhiêu đồng. Dòng `gia_mot_o = 15000` ngay đầu bài đã cho một nửa câu trả lời. Chỗ trống thứ tư thì không hỏi tiền, nó hỏi một câu đúng hay sai.
- kind: strategy
  body: Ba chiều cao viết bằng phép nhân giữa giá một ổ và số ổ, mỗi dòng dùng đúng cái tên nằm ngay trên nó. Chỗ trống cuối viết lại nguyên câu trong phần đoán trước, chỉ thay ba câu tính dài bằng ba cái tên vừa đặt: tổng hai chiều cao ngoài, so với hai lần chiều cao giữa.
- kind: one-line
  body: "Ba dòng đầu là `gia_mot_o * so_o_dau`, `gia_mot_o * so_o_giua`, `gia_mot_o * so_o_cuoi`; dòng cuối là `tien_dau + tien_cuoi == tien_giua * 2`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: ba chiều cao phải được TÍNH ra từ giá một ổ và số ổ, còn dòng cuối phải là một phép so sánh thật — điền số đã tính sẵn thì cái bài này vừa dạy không xuất hiện ở đâu cả
  requireAst:
  # Bốn dấu nhân: ba chiều cao, cộng một lần `tien_giua * 2`. Khung khởi đầu
  # không có dấu nhân nào, nên luật này chặn được đúng đáp án chép cứng bốn giá
  # trị.
  - kind: uses-operator, target: *, min: 4
  # Dòng cuối phải là một câu SO SÁNH, không phải một con số hay một chữ `True`
  # gõ tay.
  - kind: uses-operator, target: ==, min: 1
  - kind: uses-operator, target: +, min: 1
  # Ba chiều cao phải đọc lại giá một ổ, thay vì tự nhân nhẩm rồi gõ kết quả.
  - kind: uses-name, target: gia_mot_o, min: 3
  - kind: uses-name, target: so_o_dau, min: 1
  - kind: uses-name, target: so_o_giua, min: 1
  - kind: uses-name, target: so_o_cuoi, min: 1
  # `tien_giua` được đọc hai lần: một lần trong câu so sánh, một lần ở `print`.
  - kind: uses-name, target: tien_giua, min: 2
  forbidAst:
  # Bốn con số là KẾT QUẢ. Lời giải thật không chứa nguyên văn cái nào.
  - kind: has-literal, target: 30000
  - kind: has-literal, target: 75000
  - kind: has-literal, target: 120000
  - kind: has-literal, target: 150000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^30000\n75000\n120000\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
30000, 75000, 120000 — và dấu chấm giữa rơi đúng chính giữa. Cây thước của mình
đặt lên đâu cũng dính.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái hình hôm nay dùng để **tra**: đi ngang tới một số ổ, ngước thẳng lên, gặp
dấu chấm, rẽ sang trục đứng đọc số tiền. Chín lần tra, chín câu trả lời, mỗi
lần đúng một câu.

Chiều nay Byte vẽ một hình khác, cũng hai trục, cũng chấm từ một cuốn sổ. Lần
này trục nằm ghi **số ổ bán được trong một giờ**, còn trục đứng ghi **giờ trên
đồng hồ**. Sổ ghi rõ: 8 giờ bán 12 ổ, và 16 giờ cũng bán đúng 12 ổ.

Tra thử cái hình ấy: đi ngang tới 12, ngước thẳng lên — và gặp **hai** dấu
chấm. Một cái ở giờ 8, một cái ở giờ 16.

Vẫn hai trục ấy, vẫn cách chấm ấy, vẫn cách tra ấy. Mà câu trả lời lại có hai.

Cái hình chiều nay còn dùng để tra được không? Và nếu không, thì thứ phân biệt
nó với cái hình sáng nay nằm ở chỗ nào — ở cuốn sổ, ở hai cái trục, hay ở một
điều gì đó mà tới giờ chưa ai nói ra thành lời?

Bài sau đi phân loại.
::::

::::checkpoint{mastery=0.8}
::::
