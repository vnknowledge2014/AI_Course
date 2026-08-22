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
estimatedMinutes: 13
teaches: [math.fraction-add]
requires: [math.fraction-compare, math.fraction, math.like-units, math.improper-fraction, math.subtraction-as-distance, core.arithmetic, core.variable, core.print-variable, core.boolean, ctrl.comparison]
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

print(sang + chieu)
```

Máy in ra:

```text
5
```

Năm **phần của thước `1/6`**, tức `5/6` sải dây.

Để ý một chuyện trong đoạn code trên: con số `6` không đi vào phép cộng lần
nào. Nó không xuất hiện ở bất kỳ dấu `+` nào cả. Nó chỉ nằm trong lời chú
thích, làm đúng một việc — nói cho bạn biết những con số 3, 2, 5 kia đang đếm
**cái gì**.

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

Bốn chỗ trống hỏi cùng một câu, chỉ khác con số: **lượng này đếm bằng thước
chung thì được mấy phần?** Máy sẽ cộng giúp bạn phần còn lại.

Hai luống này được chọn để cho ra hai cỡ khác nhau: một luống chưa tới một
sải, một luống vượt quá. Điền cùng một con số vào cả bốn chỗ thì không luống
nào ra đúng.

```python title=starter
# Luống A — sáng 1/2 sải, chiều thêm 1/3 sải. Thước chung: 1/6 sải.
a_sang = ___
a_chieu = ___
print(a_sang + a_chieu)

# Luống B — sáng 3/4 sải, chiều thêm 2/3 sải. Thước chung: 1/12 sải.
b_sang = ___
b_chieu = ___
print(b_sang + b_chieu)
print(b_sang + b_chieu > 12)
```

```python title=solution
# Luống A — sáng 1/2 sải, chiều thêm 1/3 sải. Thước chung: 1/6 sải.
a_sang = 3
a_chieu = 2
print(a_sang + a_chieu)

# Luống B — sáng 3/4 sải, chiều thêm 2/3 sải. Thước chung: 1/12 sải.
b_sang = 9
b_chieu = 8
print(b_sang + b_chieu)
print(b_sang + b_chieu > 12)
```

```python title=test
# Sáu assert trên hai bộ dữ liệu khác nhau: một số điền chung cho cả bốn chỗ
# trống không thể qua nổi, vì bốn chỗ ấy là bốn con số khác nhau.
assert a_sang == 3, "1/2 sải đếm bằng thước 1/6 thì được 3 phần, không phải 1"
assert a_chieu == 2, "1/3 sải đếm bằng thước 1/6 thì được 2 phần"
assert a_sang + a_chieu == 5, "1/2 + 1/3 là 5 phần của thước 1/6, tức 5/6 sải"
assert b_sang == 9, "3/4 sải đếm bằng thước 1/12 thì được 9 phần (3 x 3)"
assert b_chieu == 8, "2/3 sải đếm bằng thước 1/12 thì được 8 phần (2 x 4)"
assert b_sang + b_chieu == 17, "3/4 + 2/3 là 17 phần của thước 1/12 — vượt quá một sải, và điều đó không sao cả"
```

:::hints
- kind: attention
  body: Cả bốn chỗ trống hỏi đúng một kiểu câu, đó là câu bài 33 đã dạy trả lời — đổi sang thước nhỏ hơn mấy lần thì cùng lượng ấy đếm được nhiều phần hơn bấy nhiêu lần.
- kind: strategy
  body: Thước `1/6` nhỏ hơn thước `1/2` ba lần nên mỗi `1/2` gồm 3 phần; nhỏ hơn thước `1/3` hai lần nên mỗi `1/3` gồm 2 phần. Làm y hệt cho luống B với thước `1/12` — nó nhỏ hơn `1/4` ba lần và nhỏ hơn `1/3` bốn lần.
- kind: one-line
  body: "Điền lần lượt `3`, `2`, `9`, `8` vào bốn chỗ trống theo thứ tự từ trên xuống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^5\n17\nTrue\s*$
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
