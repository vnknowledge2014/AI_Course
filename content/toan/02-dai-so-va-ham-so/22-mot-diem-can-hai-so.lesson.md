---
id: toan.dai-so-va-ham-so.mot-diem-can-hai-so
title: Một điểm cần hai con số
summary: Một hàng số ghi được một con số, mà một dòng của bảng thì có tận hai — nên phải dựng thêm một trục nữa, và thứ tự trong cặp trở thành một phần của cái điểm.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.ordered-pair, math.coordinate-plane]
requires: [math.value-table, math.inequality, math.thanh-so, math.compare-on-number-line, math.multiplication, core.variable, core.assignment, core.arithmetic, core.number-literal, core.print-variable, core.list, core.list-index, core.boolean, ctrl.comparison]
concepts: [math.he-toa-do, math.cap-co-thu-tu, math.xe-banh-mi]
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
Một hàng số ghi được một con số. Còn một dòng của bảng thì có tận hai.
::::

::::explain{#mot-hang-khong-du}
Bài trước để lại một câu hỏi. Bạn vừa tô tập nghiệm của một bất phương trình
lên **trục số** — một tia chạy về bên trái, mút rỗng ở vạch 10. Trục số làm
được việc đó vì nó là **một hàng**: mỗi chỗ trên nó ghi đúng một con số.

Nhưng cái bảng đứng ngay cạnh thì có hai cột:

| số ổ hạ giá n | lãi = 60000 − 4000 × n |
| --- | --- |
| 9 | 24000 |
| 10 | 20000 |
| 12 | 12000 |

Đọc dòng đầu tiên: nó nói **hai** chuyện cùng một lúc — *chín ổ* và *hai mươi
tư nghìn đồng*. Hai chuyện ấy dính vào nhau, tách ra là mất nghĩa. Con số
`24000` đứng một mình không cho biết nó là lãi của bao nhiêu ổ; con số `9` đứng
một mình thì không nói được buổi ấy lãi bao nhiêu.

Thử ghi dòng ấy lên trục số xem:

```text
   0   1   2   3   4   5   6   7   8   9  10  11  12
   ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
                                       ▲
                     ghi được số 9 rồi — nhưng 24000 đi đâu?
```

Ghi 9 thì mất 24000. Ghi 24000 thì mất 9 — mà muốn ghi 24000 lên đúng cái trục
này còn phải đổi luôn cách đánh vạch. Ghi cả hai lên cùng một hàng thì được hai
vết nằm cạnh nhau, và sáng mai không ai còn biết vết nào là số ổ, vết nào là
tiền.

Một hàng không đủ. Phải có hàng thứ hai.
::::

::::example{#dung-hai-truc}
Lấy đúng cái trục số quen thuộc, đặt nằm ngang, và hẹn với nhau: **trục này ghi
số ổ**. Rồi dựng thêm một trục số nữa, **vuông góc** với nó, cắt nó ngay tại
mốc 0, và hẹn tiếp: **trục này ghi tiền**.

Thử ngay trên cái bảng gốc nhất của xe bánh mì — bảng tiền thu ở bài 5, `15000
× n`. Chấm dòng "3 ổ — 45000 đồng":

- Đi ngang tới vạch **3** trên trục nằm.
- Từ chỗ đó đi thẳng lên tới ngang vạch **45000** của trục đứng.
- Dừng lại, chấm một dấu.

```text
  tiền thu (đồng)
   ↑
 60000│
 45000│        ●
 30000│
 15000│
     0└──┴──┴──┴──┴──┴──┴──┴──→ số ổ (n)
      0  1  2  3  4  5  6  7
```

Một dấu chấm ấy giữ được **cả hai** con số cùng một lúc, mà không cần viết chữ
nào bên cạnh. Vì chính chỗ nó đứng đã nói ra cả hai: nó cách trục đứng đúng 3
bước, và cách trục nằm đúng 45000 bước.

Hai cái trục vuông góc chung mốc 0 ấy gọi là một **hệ toạ độ**. Còn hai con số
dùng để chấm một điểm thì viết gọn lại trong một dấu ngoặc:

> `(3; 45000)`

Đọc là: đi ngang 3, đi lên 45000.

Cặp `(9; 24000)` của bảng lãi cũng ghi được y như thế — chỉ cần đổi chữ ghi trên
trục đứng từ *tiền thu* thành *tiền lãi*, và đánh vạch cho vừa. Chỗ ghi thì
không đổi.
::::

::::explain{#thu-tu-la-mot-phan-cua-diem}
Thứ tự bên trong dấu ngoặc không phải chuyện trang trí. Nó là một phần của cái
điểm.

Bài 1 của track này mở đầu bằng đúng chỗ khó ấy: Byte viết lên bảng giá "▢ ổ
bánh mì và ▢ chai nước", hai ô trống nằm cạnh nhau, và ai đọc cũng phải biết ô
nào là ô nào. Cặp toạ độ gỡ chuyện đó bằng một quy ước, dài đúng một dòng:

> **Số viết trước đi ngang. Số viết sau đi lên.**

Có quy ước rồi thì thử ngay. Cuối mỗi buổi, Byte ghi vào sổ hai con số: bán
được mấy ổ, và có mấy khách ghé xe.

```text
  khách ghé xe
   ↑
  7│
  6│     ●
  5│
  4│
  3│
  2│                 ●
  1│
  0└──┴──┴──┴──┴──┴──┴──┴──→ ổ bánh mì bán được
   0  1  2  3  4  5  6  7
```

- `(2; 6)` — trưa nắng: 6 người ghé, mà chỉ 2 người mua; bốn người kia hỏi giá
  rồi đi.
- `(6; 2)` — chiều mát: đúng 2 khách, nhưng mỗi người ôm luôn 3 ổ về nhà.

Cùng hai con số 2 và 6, không thêm không bớt. Nhưng đó là hai buổi bán hàng
khác hẳn nhau — và trên lưới, hai dấu chấm rơi vào hai chỗ khác hẳn nhau, soi
gương nhau qua đường chéo.

Nên cặp toạ độ là một **cặp có thứ tự**: đổi chỗ hai con số **khác nhau** là
đổi luôn cái điểm. (Hai con số bằng nhau thì đổi chỗ chẳng đi đâu cả — `(4; 4)`
đảo lại vẫn là `(4; 4)`. Đó đúng là mấy điểm nằm ngay trên đường chéo, chỗ tấm
gương cắt qua.) Đây là chỗ nó khác hẳn phép cộng — `2 + 6` với `6 + 2` cho cùng một số,
còn `(2; 6)` với `(6; 2)` thì không cho cùng một chỗ đứng.
::::

::::predict{#doan-hai-cap commitOnce}
Byte đưa hai buổi bán ấy cho máy giữ. Mỗi cặp là một chỗ nhớ có **hai ngăn**,
xếp theo đúng thứ tự đã hẹn: ngăn số 0 giữ số ổ, ngăn số 1 giữ số khách.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
trua_nang = [2, 6]
chieu_mat = [6, 2]

print(trua_nang == chieu_mat)
print(trua_nang[0] == chieu_mat[1])
print(trua_nang[1] == chieu_mat[0])
```

:::opt{correct}
False, rồi True, rồi True
:::

:::opt
True, rồi True, rồi True
::why
Gần đúng ở chỗ bạn nhìn vào hai cặp và thấy chúng dùng **đúng hai con số ấy** —
2 và 6, không thiếu không thừa. Quy tắc bạn đang dùng, *hai đống gồm cùng chừng
ấy thứ thì là một đống*, đúng thật, và cả phép cộng dựng trên nó: đổ đống nào
vào trước cũng ra chừng ấy hạt.

Chỗ lệch là phạm vi. Quy tắc ấy nói về **đống**, mà một cặp toạ độ không phải
một đống. Đống thì không có ngăn; cặp thì có đúng hai ngăn, và mỗi ngăn trả lời
một câu hỏi khác nhau — "mấy ổ?" với "mấy khách?". Máy so hai danh sách theo
đúng từng ngăn một: ngăn 0 với ngăn 0, ngăn 1 với ngăn 1. Ngăn 0 bên này là 2,
bên kia là 6, lệch ngay từ đó, nên cả cặp khác nhau.
::
:::

:::opt
False, rồi False, rồi False
::why
Gần đúng ở chỗ bạn trả lời trúng câu đầu tiên, và trúng vì đúng lý do: hai cặp
ấy không phải một, đó chính là điều cả bài này muốn nói.

Chỗ lệch nằm ở bước suy tiếp. Từ "hai cặp khác nhau" bạn suy ra "vậy chẳng có
gì trong chúng khớp nhau" — suy luận ấy đúng khi hai vật khác nhau **ở mọi
thành phần**. Ở đây thì ngược hẳn: hai cặp này khác nhau **chỉ vì thứ tự**, còn
hai con số thì vẫn nguyên đó, chỉ đổi chỗ cho nhau. Nên ngăn 0 bên này (2) đúng
bằng ngăn 1 bên kia (2), và ngăn 1 bên này (6) đúng bằng ngăn 0 bên kia (6).
::
:::

:::opt
False, rồi máy báo lỗi
::why
Gần đúng ở chỗ bạn nhớ bài R0.16: ghép hai thứ khác kiểu, như `"5" + 5`, thì
máy dừng lại và báo `TypeError` chứ không đoán bừa hộ bạn. Cảnh giác ấy đặt
đúng chỗ trong rất nhiều bài.

Chỗ lệch là luật đó nói về những phép **tạo ra** một giá trị mới, như dấu `+`.
Còn `trua_nang[0]` không tạo ra gì cả — nó chỉ **mở đúng một ngăn ra xem**, mà
cặp nào cũng có ngăn số 0. Dấu `==` thì cũng chỉ **hỏi** "hai thứ này có giống
nhau không", và câu hỏi đó luôn trả lời được: cùng lắm là `False`.
::
:::
::::

::::explain{#hai-vai-cua-hai-o}
Máy vừa nói ra chỗ mà hình vẽ đã nói bằng cách khác: hai ô trong một cặp **không
thay nhau được**.

Đáng để ý là máy không hề biết gì về bánh mì hay khách hàng. Nó chỉ giữ hai ngăn
theo thứ tự và so từng ngăn một. Cái nghĩa — ngăn nào là số ổ, ngăn nào là số
khách — nằm ở quy ước của bạn, không nằm trong máy. Đổi quy ước giữa chừng thì
máy vẫn chạy trơn tru mà mọi câu trả lời đều sai.

Đó cũng là lý do một hệ toạ độ luôn phải nói rõ **trục nằm ghi cái gì, trục
đứng ghi cái gì**, trước khi chấm dấu đầu tiên.
::::

::::code{#cham-hai-buoi}
Xe bánh mì bán 15000 đồng một ổ. Sáng nay hai người bán ở hai đầu chợ:

- **Byte** bán được `3` ổ.
- **An** bán được `7` ổ.

Điền sáu chỗ trống để máy dựng ba cặp toạ độ: điểm của Byte, điểm của An, và —
cố ý — cặp của Byte **viết ngược thứ tự**, để xem hai cách viết ấy có ra cùng
một điểm không.

Bài chấm bằng cả ba cặp, và ba cặp ấy được chọn để không cặp nào giống cặp nào.
Gõ cứng `45000` vào là hỏng ở cặp của An; gõ cứng `105000` là hỏng ở cặp của
Byte. Chỉ lời giải viết bằng những cái tên có sẵn mới qua được cả ba.

```python title=starter
gia_mot_o = 15000

# Byte bán 3 ổ.
so_o_byte = 3
tien_byte = gia_mot_o * so_o_byte
diem_byte = [___, ___]

# An bán 7 ổ.
so_o_an = 7
tien_an = gia_mot_o * so_o_an
diem_an = [___, ___]

# Cùng hai con số của Byte, nhưng viết ngược thứ tự lại.
diem_nguoc = [___, ___]

print(diem_byte)
print(diem_an)
print(diem_byte == diem_nguoc)
```

```python title=solution
gia_mot_o = 15000

# Byte bán 3 ổ.
so_o_byte = 3
tien_byte = gia_mot_o * so_o_byte
diem_byte = [so_o_byte, tien_byte]

# An bán 7 ổ.
so_o_an = 7
tien_an = gia_mot_o * so_o_an
diem_an = [so_o_an, tien_an]

# Cùng hai con số của Byte, nhưng viết ngược thứ tự lại.
diem_nguoc = [tien_byte, so_o_byte]

print(diem_byte)
print(diem_an)
print(diem_byte == diem_nguoc)
```

```python title=test
# Hai câu `!=` đứng trước, vì chúng canh đúng chỗ dễ sập nhất: điền cùng một
# thứ vào mọi ngăn thì ba cặp hoá ra giống nhau, và mọi câu `==` phía sau sẽ
# không bao giờ chạy tới.
assert diem_byte != diem_an, "hai người bán số ổ khác nhau, nên phải ra hai điểm khác nhau"
assert diem_byte != diem_nguoc, "đổi chỗ hai con số là đổi luôn cái điểm — cặp toạ độ có thứ tự, không phải một đống"
assert diem_byte == [3, 45000], "Byte: đi ngang 3 ổ, đi lên 45000 đồng"
assert diem_an == [7, 105000], "An: đi ngang 7 ổ, đi lên 105000 đồng"
assert diem_nguoc == [45000, 3], "cặp viết ngược: 45000 đi ngang, 3 đi lên — một chỗ đứng khác hẳn"
```

:::hints
- kind: attention
  body: Nhìn hai dòng ngay phía trên mỗi chỗ trống. Một dòng cho biết số ổ, dòng kia cho biết số tiền. Rồi đọc lại quy ước một dòng ở phần trên bài: số nào đi ngang, số nào đi lên.
- kind: strategy
  body: Mỗi cặp có hai ngăn. Ngăn trước giữ thứ đo trên trục nằm, ngăn sau giữ thứ đo trên trục đứng. Viết chúng bằng những cái tên đã có sẵn ngay phía trên, đừng gõ thẳng con số — con số ấy chính là thứ bạn đang nhờ máy tính hộ. Riêng `diem_nguoc` thì cố ý xếp hai cái tên của Byte theo thứ tự ngược lại.
- kind: one-line
  body: "Ba cặp lần lượt là `[so_o_byte, tien_byte]`, `[so_o_an, tien_an]` và `[tien_byte, so_o_byte]`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi ngăn phải là một cái tên có sẵn ở ngay phía trên, không phải một con số gõ tay — gõ số nghĩa là bạn đã tra bảng hộ máy rồi
  requireAst:
  # `so_o_byte` và `tien_byte` phải được ĐỌC, không được gõ số thay.
  #
  # Trước đây đòi 3 và 2, tức ngầm bắt `diem_nguoc` phải viết lại hai cái tên
  # ấy. Như thế đánh trượt `diem_nguoc = [diem_byte[1], diem_byte[0]]` — thoả
  # đúng lời Byte dặn ("cùng hai con số ấy, viết ngược thứ tự lại"), và moi ra
  # từ chính cái điểm vừa dựng thì còn khít hơn. Hai con số KẾT QUẢ vẫn bị chặn
  # bởi `has-literal` phía dưới.
  - kind: uses-name, target: so_o_byte, min: 2
  - kind: uses-name, target: tien_byte, min: 1
  - kind: uses-name, target: so_o_an, min: 2
  - kind: uses-name, target: tien_an, min: 1
  forbidAst:
  # Lưới thứ hai, chặn đúng hai con số là KẾT QUẢ của phép nhân. Lời giải thật
  # không chứa nguyên văn chúng, nên luật này không cản ai làm thật.
  - kind: has-literal, target: 45000
  - kind: has-literal, target: 105000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^\[3, 45000\]\n\[7, 105000\]\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`[3, 45000]` với `[45000, 3]` — cùng hai con số, mà máy bảo không phải một.
Đúng rồi: thứ tự là một phần của chỗ đứng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay bạn chấm được hai điểm: một của Byte, một của An. Chấm hai điểm mà phải
dựng cả một hệ toạ độ thì hơi phí — hai dòng bảng đọc thẳng cũng xong.

Nhưng cái bảng ở bài 5 không có hai dòng. Nó chạy từ 0 ổ tới 8 ổ, không bỏ sót
số nào — chín dòng, chín cặp, chín dấu chấm.

Chấm cả chín lên cùng một tờ giấy thì được một nắm dấu chấm. Và đây là chỗ đáng
hỏi: nắm ấy nằm bừa bãi như hạt vãi xuống đất, hay chúng xếp thành một hình gì
đó?

Nếu quả thật có hình, thì cái hình ấy là hình của **cái gì** — của chín dòng
bảng, hay của đúng cái câu tính `15000 × n` đã đẻ ra cả chín dòng?

Bài sau chấm hết cả bảng lên một tờ giấy rồi nhìn.
::::

::::checkpoint{mastery=0.8}
::::
