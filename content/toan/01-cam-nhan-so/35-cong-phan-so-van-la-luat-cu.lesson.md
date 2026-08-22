---
id: toan.cam-nhan-so.cong-phan-so-van-la-luat-cu
title: Cộng phân số vẫn là luật cũ
summary: Không có luật cộng riêng cho phân số. Vẫn là luật bài 11 — chỉ gộp được thứ cùng đơn vị — nên phải cùng mẫu, rồi cộng tử và giữ nguyên mẫu.
locale: vi
track: toan
module: cam-nhan-so
order: 35
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.fraction-add]
requires: [math.fraction-compare, math.equivalent-fraction, math.fraction, math.like-units, math.improper-fraction, math.subtraction-as-distance, core.arithmetic, core.variable, core.print-variable, core.boolean, ctrl.comparison]
concepts: [math.phan-so, math.mau-chung, math.don-vi]
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

::::byte{trigger=enter mood=thinking pose=lean-in}
Mình chỉ cộng được số phần thôi. Cái tên của phần thì không đem cộng.
::::

::::explain{#khong-co-luat-moi-nao-ca}
Nói trước một điều, để bạn khỏi mất công đi tìm: **bài này không mang khái
niệm mới nào.** Nó đóng lại một cái vòng mở từ bài 11. Việc của bạn ở đây
không phải học thêm luật, mà là nhận ra luật cũ vẫn đang đứng nguyên chỗ.

Bài trước để lại luật dễ nghĩ ra nhất: cộng tử với tử, cộng mẫu với mẫu, nên
`1/2 + 1/3 = 2/5`. Và một chỗ hỏng của nó: theo luật ấy thì `1/2 + 1/2 = 2/4`,
mà `2/4` rút gọn lại đúng bằng `1/2`. Gộp nửa sải dây với nửa sải dây xong vẫn
còn nửa sải.

Vì sao luật ấy hỏng, thì bài 31 đã nói sẵn: **mẫu không phải một lượng, mẫu là
tên của cái thước.** `1/2` là *một cái thước cỡ nửa sải*. `1/3` là *một cái
thước cỡ một phần ba sải*. Đem cộng hai con số 2 và 3 với nhau là đem cộng hai
cái **tên** — mà cộng "nửa" với "một phần ba" thì không ra tên nào cả.

So nó với chuyện bạn đã làm suốt từ bài 6:

- 3 bó cộng 2 bó được **5 bó**. Cộng số bó; chữ "bó" giữ nguyên, không thành
  "bó bó".
- 3 hạt cộng 2 bó thì không cộng thẳng được — bài 11 gọi đó là *chỉ gộp được
  thứ cùng đơn vị*, và đó là lý do người ta viết thẳng cột.

Phân số không có ngoại lệ nào ở đây. `1/2` và `1/3` là hai đơn vị khác nhau,
nên phép cộng chưa có nghĩa. Muốn nó có nghĩa thì phải làm đúng việc bài 34
vừa làm: quy cả hai về **cùng một thước**. Xong bước đó thì hai bên thành hai
đống cùng loại phần, và cộng hai đống cùng loại phần thì bạn đã biết làm từ
lâu.
::::

::::example{#gop-hai-doan-day}
Sáng Byte căng `1/2` sải dây, chiều căng thêm `1/3` sải nữa. Cả luống dài bao
nhiêu?

Thước chung lấy như bài 34: mẫu này nhân mẫu kia, `2 × 3 = 6`. Mỗi `1/2` gồm 3
cái `1/6`; mỗi `1/3` gồm 2 cái `1/6`.

```python title=readonly
# Quy về thước 1/6 sải rồi mới cộng.
sang = 1 * 3      # 1/2 sải = 3 phần của thước 1/6
chieu = 1 * 2     # 1/3 sải = 2 phần của thước 1/6
mau = 6           # cộng xong thì vẫn đang đếm bằng thước 1/6

print(sang + chieu, mau)
```

Máy in ra:

```text
5 6
```

Năm **phần của thước `1/6`**, tức `5/6` sải dây.

Để ý con số `6` trong đoạn code trên: nó không đi vào dấu `+` nào cả. Nó đi
thẳng từ dòng trên xuống dòng kết quả, y nguyên — vì nó không phải một lượng
đem cộng, nó là **tên của cái thước** mà cả ba con số 3, 2, 5 kia đang đếm.

Đó là toàn bộ nội dung của câu "cộng tử, giữ nguyên mẫu": tử là số phần đếm
được nên đem cộng, mẫu là tên đơn vị nên đi theo mà không đổi.
::::

::::predict{#doan-hai-phan-nam-co-trung-khong commitOnce}
Bây giờ hỏi thẳng máy: `2/5` — kết quả của luật sai — có rơi trúng chỗ `5/6`
trên thanh số không?

Hai phân số này đo bằng hai thước khác nhau, nên theo bài 34 phải quy về cùng
thước trước. Thước chung là `1/30` (`5 × 6`): mỗi `1/5` gồm 6 phần, mỗi `1/6`
gồm 5 phần.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(2 * 6)              # 2/5 sải là mấy phần của thước 1/30
print(5 * 5)              # 5/6 sải là mấy phần của thước 1/30
print(2 * 6 == 5 * 5)     # hai bên có rơi trúng cùng một chỗ không
```

:::opt{correct}
12, rồi 25, rồi False
:::

:::opt
12, rồi 25, rồi True
::why
Gần đúng ở chỗ bạn giữ một suy nghĩ rất chắc: một phép cộng chỉ có **đúng
một** kết quả. Nếu `2/5` thật sự là tổng của `1/2` và `1/3`, thì nó buộc phải
đứng trùng chỗ với `5/6` trên thanh số, không thể khác.

Chỗ lệch là ở chỗ suy nghĩ ấy đang được dùng để bênh luật sai, trong khi nó
chính là thứ vạch mặt luật sai. Hai dòng trên vừa nói ra: một bên 12 phần, bên
kia 25 phần, cùng thước `1/30`. Hai số phần khác nhau thì hai chỗ khác nhau —
nên `2/5` không phải tổng, và luật "cộng cả tử lẫn mẫu" bị loại.
::
:::

:::opt
30, rồi 30, rồi True
::why
Gần đúng ở chỗ bạn nhận ra việc cần làm trước tiên là kéo cả hai bên về cùng
một cỡ thước, và `30` đúng là cỡ thước ấy. Bước đó bạn làm chuẩn.

Chỗ lệch: `30` là **tên đơn vị**, không phải thứ hai dòng `print` đang đếm.
`2 * 6` không tính ra mẫu chung — nó trả lời câu "2 cái thước `1/5` gồm bao
nhiêu cái thước `1/30`". Mẫu chung thì hai bên giống nhau thật, nhưng số phần
đếm được thì không, và số phần mới là thứ nói ai đứng ở đâu.
::
:::

:::opt
12, rồi 25, rồi 13
::why
Gần đúng ở chỗ bạn muốn biết hai bên lệch nhau **bao nhiêu**, chứ không chỉ
muốn biết có lệch hay không. Đó là câu hỏi hay hơn, và con số bạn đưa ra cũng
đúng: `25 − 12 = 13`, tức hai chỗ ấy cách nhau 13 phần của thước `1/30` — đúng
kiểu khoảng cách của bài 15.

Chỗ lệch nằm ở dấu bạn cần dùng để hỏi. `==` chỉ hỏi một câu có–không, và thứ
đi ra khỏi nó luôn là `True` hoặc `False`. Muốn thấy con số 13 thì phải viết
dấu trừ chứ không phải dấu bằng kép.
::
:::
::::

::::explain{#cong-tu-giu-mau}
Chốt lại thành một câu, và câu này bạn đã có từ bài 11, chỉ đổi tên đồ vật:

> Quy về cùng mẫu. Rồi cộng tử, giữ nguyên mẫu.

Vì sao nó đúng thì nhìn bằng bài 31 là thấy: `3/6` là 3 bản sao của thước
`1/6`, `2/6` là 2 bản sao của cùng cái thước ấy. Đặt 3 bản sao cạnh 2 bản sao
thì có 5 bản sao. Cái thước không tự to ra hay nhỏ đi trong lúc bạn đặt chúng
cạnh nhau — nên mẫu không có lý do gì để đổi.

Thêm một chuyện nữa, nối lại với bài 32: tổng hoàn toàn có quyền vượt quá 1.
`3/4 + 2/3` quy về thước `1/12` là `9/12 + 8/12 = 17/12`. Tử `17` lớn hơn mẫu
`12`, và đó không phải dấu hiệu sai — nó chỉ nói luống ấy dài hơn một sải dây,
đúng như hai đoạn dây bạn vừa nối lại ngoài vườn.
::::

::::code{#cong-hai-luong}
Byte đo hai luống, mỗi luống căng làm hai lần:

- **Luống A** — sáng `1/2` sải, chiều thêm `1/3` sải. Thước chung `1/6`
  (`2 × 3`).
- **Luống B** — sáng `3/4` sải, chiều thêm `2/3` sải. Thước chung `1/12`
  (`4 × 3`).

Mỗi luống có ba chỗ trống. Hai chỗ đầu hỏi câu quen thuộc: **lượng này đếm
bằng thước chung thì được mấy phần?** Chỗ thứ ba hỏi thứ máy không đoán hộ
được: **cộng xong rồi thì số phần ấy đang đếm bằng thước nào** — tức mẫu của
kết quả. Chỉ riêng phép cộng là máy làm giúp.

Hai luống này được chọn để cho ra hai cỡ khác nhau: một luống chưa tới một
sải, một luống vượt quá. Điền cùng một con số vào cả sáu chỗ thì không luống
nào ra đúng.

```python title=starter
# Luống A — sáng 1/2 sải, chiều thêm 1/3 sải. Thước chung: 1/6 sải.
a_sang = ___
a_chieu = ___
a_mau = ___
print(a_sang + a_chieu, a_mau)

# Luống B — sáng 3/4 sải, chiều thêm 2/3 sải. Thước chung: 1/12 sải.
b_sang = ___
b_chieu = ___
b_mau = ___
print(b_sang + b_chieu, b_mau)
print(b_sang + b_chieu > 12)
```

```python title=solution
# Luống A — sáng 1/2 sải, chiều thêm 1/3 sải. Thước chung: 1/6 sải.
a_sang = 3
a_chieu = 2
a_mau = 6
print(a_sang + a_chieu, a_mau)

# Luống B — sáng 3/4 sải, chiều thêm 2/3 sải. Thước chung: 1/12 sải.
b_sang = 9
b_chieu = 8
b_mau = 12
print(b_sang + b_chieu, b_mau)
print(b_sang + b_chieu > 12)
```

```python title=test
# Tám assert trên hai bộ dữ liệu khác nhau: một số điền chung cho cả sáu chỗ
# trống không thể qua nổi, vì sáu chỗ ấy là sáu con số khác nhau. Hai assert
# về `_mau` chấm đúng nửa sau của luật: cộng tử, GIỮ NGUYÊN mẫu.
assert a_sang == 3, "1/2 sải đếm bằng thước 1/6 thì được 3 phần, không phải 1"
assert a_chieu == 2, "1/3 sải đếm bằng thước 1/6 thì được 2 phần"
assert a_sang + a_chieu == 5, "1/2 + 1/3 là 5 phần của thước 1/6, tức 5/6 sải"
assert a_mau == 6, "đặt 3 bản sao của thước 1/6 cạnh 2 bản sao thì vẫn là thước 1/6 — mẫu không cộng vào nhau thành 12, cũng không nhân lên thành 36"
assert b_sang == 9, "3/4 sải đếm bằng thước 1/12 thì được 9 phần (3 x 3)"
assert b_chieu == 8, "2/3 sải đếm bằng thước 1/12 thì được 8 phần (2 x 4)"
assert b_sang + b_chieu == 17, "3/4 + 2/3 là 17 phần của thước 1/12 — vượt quá một sải, và điều đó không sao cả"
assert b_mau == 12, "cộng xong vẫn đang đếm bằng thước 1/12 — cái thước không tự to ra hay nhỏ đi trong lúc bạn đặt các phần cạnh nhau"
```

:::hints
- kind: attention
  body: Bốn chỗ trống ở dòng `_sang` và `_chieu` hỏi đúng một kiểu câu, đó là câu bài 33 đã dạy trả lời — đổi sang thước nhỏ hơn mấy lần thì cùng lượng ấy đếm được nhiều phần hơn bấy nhiêu lần. Hai chỗ `_mau` thì không phải tính gì cả: câu trả lời đã nằm sẵn trong dòng chú thích "Thước chung" ngay phía trên.
- kind: strategy
  body: Thước `1/6` nhỏ hơn thước `1/2` ba lần nên mỗi `1/2` gồm 3 phần; nhỏ hơn thước `1/3` hai lần nên mỗi `1/3` gồm 2 phần. Làm y hệt cho luống B với thước `1/12` — nó nhỏ hơn `1/4` ba lần và nhỏ hơn `1/3` bốn lần. Còn `a_mau` và `b_mau` là tên của chính cái thước chung ấy: cộng xong không ai đổi thước, nên mẫu vẫn là con số đứng dưới gạch ngang của thước chung.
- kind: one-line
  body: "Điền `3`, `2`, `6` cho luống A và `9`, `8`, `12` cho luống B, theo thứ tự từ trên xuống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^5 6\n17 12\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm phần sáu, rồi mười bảy phần mười hai. Mình chỉ cộng số phần thôi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại việc bạn vừa làm bốn lần liền: trước mỗi phép cộng đều phải dừng lại
đi tìm một cỡ thước chung, rồi quy cả hai bên về nó. `1/2` với `1/3` thì tìm
ra `1/6`; `3/4` với `2/3` thì tìm ra `1/12`. Mỗi cặp một cỡ thước riêng, và
lần nào cũng phải tìm lại từ đầu.

Ngoài chợ thì người ta không làm thế. Hỏi giá thì nghe "hai mươi lăm nghìn
rưỡi", đo vải thì nghe "một mét tư", cân gạo thì nghe "ba cân rưỡi" — cộng
chúng lại chẳng ai đi tìm mẫu chung lần nào.

Vậy có loại phân số nào mà mẫu **luôn sẵn giống nhau** không? Một loại thước
mà mọi lượng đều đã đo bằng nó từ trước, nên cộng là cộng thẳng?

Bài sau trả lời, và cái thước ấy bạn đã cầm trên tay từ bài 6.
::::

::::checkpoint{mastery=0.8}
::::
