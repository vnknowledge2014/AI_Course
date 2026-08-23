---
id: toan.dai-so-va-ham-so.bang-cua-moi-lan-dien
title: Bảng của mọi lần điền
summary: Một câu tính có ô trống không sinh ra một con số mà sinh ra cả một bảng — hai cột, mỗi lần điền một dòng, và bảng ấy mới là chân dung đầy đủ của nó.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.value-table]
requires: [math.substitution, math.expression, math.letter-names-a-slot, math.multiplication, ctrl.for-range, ctrl.loop-variable, core.variable, core.fstring, core.print-variable, core.arithmetic]
concepts: [math.o-trong, math.bieu-thuc, math.bang-gia-tri]
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
Bảy ngày, bảy lần điền, bảy con số. Mình vừa ghi ra thứ gì thế này?
::::

::::explain{#cuon-so-bay-ngay}
Byte làm đúng như đã nói: bán suốt tuần, mỗi ngày điền số ổ bán được vào câu tính
tiền thu `15000 × n`, rồi ghi kết quả xuống sổ.

Bài này đổi sang câu tính đơn giản nhất của cái xe — **tiền thu**, `15000 × n` —
chứ không dùng câu tiền lãi của bài trước. Không phải vì câu kia khó; mà vì thứ
đáng nhìn trong bài này **không phải câu tính**, mà là thứ nó sinh ra. Câu càng
gọn thì thứ ấy càng dễ thấy.

Đây là cuốn sổ sau bảy ngày:

```text
ngày        số ổ bán    tiền thu
thứ Hai            8      120000
thứ Ba            20      300000
thứ Tư            12      180000
thứ Năm           12      180000
thứ Sáu            5       75000
thứ Bảy           31      465000
Chủ nhật          24      360000
```

Nhìn cuốn sổ này kỹ một chút thì thấy ba điều.

**Một:** cột "ngày" thật ra chẳng làm gì cả. Thứ Tư và thứ Năm cùng bán 12 ổ, và
cùng thu 180000. Cái tên "thứ Tư" không hề tham gia vào phép tính. Bỏ cột ấy đi,
cuốn sổ không mất gì.

**Hai:** cột "số ổ bán" là **thứ được điền vào**, cột "tiền thu" là **thứ ra**.
Hai cột, và cột phải hoàn toàn do cột trái quyết định.

**Ba:** cuốn sổ này lộn xộn. Nó đi theo thứ tự các ngày trôi qua, nên cột trái
nhảy 8, 20, 12, 12, 5, 31, 24 — có chỗ lặp, có chỗ nhảy cóc. Nó ghi lại **những
lần điền đã xảy ra**, không phải **những lần điền có thể xảy ra**.
::::

::::example{#tu-cuon-so-thanh-bang}
Byte chép lại cuốn sổ, lần này bỏ cột ngày và xếp cột trái theo thứ tự, không bỏ
sót số nào, bắt đầu từ 0:

```text
    n     15000 × n
    0             0
    1         15000
    2         30000
    3         45000
    4         60000
    5         75000
    6         90000
    7        105000
    8        120000
```

Thứ vừa hiện ra không còn là cuốn sổ của một tuần cụ thể nữa. Nó là **bảng của
câu tính** — và nó có tên: **bảng giá trị**.

> **Bảng giá trị** của một câu tính có ô trống: mỗi lần điền là **một dòng**,
> hai cột — cột trái ghi **điền gì**, cột phải ghi **ra gì**.

Bảng này khác cuốn sổ ở một điểm quyết định. Cuốn sổ phụ thuộc vào chuyện đã xảy
ra ngoài đời: tuần sau Byte bán khác đi thì sổ khác đi. Còn bảng thì không. Bảng
chỉ phụ thuộc vào **câu tính**. Nó có sẵn dòng cho 5 ổ, cho 31 ổ, cho 100 ổ, cho
cả những số ổ Byte chưa bao giờ bán tới.

Và đây là câu trả lời cho câu hỏi bỏ ngỏ bài trước: **một câu tính có ô trống
không sinh ra một con số — nó sinh ra cả một bảng.** Mỗi con số bạn từng lấy ra
được ở bài 4 chỉ là **một dòng** của cái bảng này.

```text
   một con số     →  một điểm dừng
   một câu tính   →  cả cái bảng
```

Bảng còn dài mãi xuống dưới: 9, 10, 11… không có dòng cuối. Nhưng in ra vài dòng
đầu là đã đủ để nhìn ra dáng của nó.

Bắt máy in hộ, vì việc này lặp đi lặp lại đúng một kiểu:

```python title=readonly
for so_o in range(9):
    tien_thu = 15000 * so_o
    print(f"{so_o} ổ thu {tien_thu} đồng")
```

Ba dòng ấy làm đúng ba việc của bảng: lấy lần lượt từng giá trị để điền, điền
vào câu tính, ghi lại thành một dòng.
::::

::::predict{#doan-bang commitOnce}
Byte rút ngắn lại, chỉ in bốn dòng đầu của bảng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
for so_o in range(4):
    tien_thu = 15000 * so_o
    print(f"{so_o} ổ thu {tien_thu} đồng")
```

:::opt{correct}
Bốn dòng, mở đầu bằng `0 ổ thu 0 đồng` và kết thúc bằng `3 ổ thu 45000 đồng`
:::

:::opt
Bốn dòng, mở đầu bằng `1 ổ thu 15000 đồng` và kết thúc bằng `4 ổ thu 60000 đồng`
::why
Gần đúng ở chỗ bạn đếm không sai một dòng nào: `range(4)` phát ra **bốn** giá
trị, và bạn ghi ra đúng bốn dòng. Quy tắc bạn dùng — *đếm bắt đầu từ 1* — là
quy tắc đúng của đời thường: ổ thứ nhất, ngày thứ nhất, người xếp hàng thứ nhất.

Chỗ lệch là `range` đếm theo lối của máy, bắt đầu từ **0**, đúng như chỗ đứng
đầu tiên của một danh sách. Bốn giá trị nó phát ra là 0, 1, 2, 3. Và với cái
bảng này thì mốc 0 không thừa chút nào: dòng "0 ổ thu 0 đồng" là một dòng thật,
tả một buổi Byte không bán được ổ nào.
::
:::

:::opt
Đúng một dòng, `3 ổ thu 45000 đồng`
::why
Gần đúng ở chỗ bạn theo dõi được cái tên `tien_thu` qua từng lượt và thấy nó bị
đặt lại mỗi vòng — lượt sau đè lên lượt trước. Điều đó **đúng**: sau khi vòng
chạy xong, `tien_thu` chỉ còn giữ giá trị của lượt cuối.

Chỗ lệch là `print` không nằm ngoài vòng, nó nằm **trong** vòng — nên nó chạy
mỗi lượt một lần, và chữ đã hiện lên màn hình rồi thì không ai đè lên được nữa.
Cái tên chỉ giữ được một giá trị, còn màn hình thì giữ cả bốn dòng. Đó chính là
lý do bảng ghi được nhiều dòng trong khi một cái tên thì không.
::
:::

:::opt
Bốn dòng, dòng nào cũng `thu 15000 đồng`, chỉ đổi số ổ ở đầu dòng
::why
Gần đúng ở chỗ bạn bám vào một sự thật của cái xe: **giá một ổ không đổi**, luôn
là 15 000 đ. Điều đó đúng suốt cả bài, và nó là lý do câu tính chỉ có đúng một
ô trống.

Chỗ lệch là cột phải không ghi *giá một ổ*, nó ghi *tiền thu của cả `so_o` ổ*.
Con số 15000 đứng yên, nhưng nó đang được **nhân** với một thứ đổi mỗi lượt, nên
tích đổi theo: 0, rồi 15000, rồi 30000, rồi 45000. Cái đứng yên trong câu tính
là 15000; cái đổi là ô trống.
::
:::
::::

::::explain{#bang-la-chan-dung}
Đặt lại cho gọn, để mang đi:

> Một câu tính có ô trống **là** cái bảng của nó. Muốn biết câu tính ấy làm gì,
> đừng hỏi "nó bằng bao nhiêu" — hỏi "bảng của nó trông thế nào".

Vì sao cách nhìn này đáng đổi? Vì nó trả lời được những câu mà một con số lẻ
không trả lời nổi:

- **Bán gấp đôi số ổ thì thu gấp đôi tiền phải không?** Nhìn một dòng thì chịu.
  Nhìn bảng thì thấy ngay: dòng 2 là 30000, dòng 4 là 60000 — đúng gấp đôi.
- **Mỗi ổ bán thêm thì tiền thu nhích lên bao nhiêu?** Bảng cho thấy từ dòng nào
  sang dòng kế cũng nhích đúng 15000. Đều tăm tắp, không dòng nào lệch.
- **Có dòng nào cột phải bằng 0 không?** Có, đúng một dòng: dòng `n` bằng 0.

Cả ba câu trên đều là câu hỏi về **cái bảng**, không phải về một lần điền. Và
đây là chỗ cách nhìn "chữ là số bí ẩn cần tìm" bắt đầu vướng: nếu `n` là một con
số đang bị giấu thì cái bảng này không có lý do gì tồn tại — nó phải chỉ có đúng
một dòng. Bảng có vô số dòng, vì `n` là một **ô trống**, và ô trống thì nhận
được nhiều giá trị.
::::

::::code{#in-bang-cua-xe-banh-mi}
Dựng bảng tiền thu của xe bánh mì cho sáu lần điền đầu tiên, rồi lấy riêng một
dòng ở xa hơn.

Có hai chỗ trống, và **cả hai đều là cùng một câu tính** `15000 × so_o`:

- Chỗ thứ nhất nằm trong vòng lặp — nó dựng sáu dòng đầu của bảng.
- Chỗ thứ hai nằm sau vòng lặp. Byte cần đúng **một dòng** của bảng, dòng ứng
  với 12 ổ. Vẫn phải viết bằng cái tên `so_o`, không được viết thẳng con số
  kết quả — vì con số ấy chính là thứ bạn đang nhờ máy tìm hộ.

Bài chấm bằng cả bảy dòng in ra. Sáu dòng đầu khác nhau từng dòng một, nên một
con số gõ cứng không thể làm đúng quá một dòng.

```python title=starter
# Sáu dòng đầu của bảng: cột trái điền gì, cột phải ra gì.
for so_o in range(6):
    tien_thu = ___
    print(f"{so_o} ổ thu {tien_thu} đồng")

# Bảng còn dài mãi. Byte chỉ cần đúng một dòng của nó.
so_o = 12
print(___)
```

```python title=solution
# Sáu dòng đầu của bảng: cột trái điền gì, cột phải ra gì.
for so_o in range(6):
    tien_thu = 15000 * so_o
    print(f"{so_o} ổ thu {tien_thu} đồng")

# Bảng còn dài mãi. Byte chỉ cần đúng một dòng của nó.
so_o = 12
print(15000 * so_o)
```

```python title=test
# Sau khi vòng chạy xong, `so_o` giữ 12 và `tien_thu` giữ dòng cuối của vòng
# (5 ổ). Hai câu `!=` đứng trước để canh đúng hai cái bẫy: chép cứng một con số
# vào chỗ trống, và nhầm cột trái với cột phải. Xếp sau `==` thì chúng không bao
# giờ chạy tới.
assert tien_thu != so_o, "cột phải là tiền thu, không phải số ổ — hai cột của một bảng không bao giờ là một"
assert tien_thu != 15000, "cột phải không ghi giá một ổ; nó ghi tiền thu của cả so_o ổ, nên nó đổi theo từng dòng"
assert tien_thu == 75000, "dòng cuối của vòng là 5 ổ: 15000 × 5 ra 75000"
assert so_o == 12, "sau vòng lặp, so_o được điền lại bằng 12 để lấy riêng một dòng ở xa hơn"
```

:::hints
- kind: attention
  body: Hai chỗ trống nhận cùng một câu tính, vì cả hai đều hỏi đúng một chuyện — bán bấy nhiêu ổ thì thu bao nhiêu tiền. Nhìn dòng `print` ngay dưới chỗ trống thứ nhất để biết cái tên nào đang giữ số ổ.
- kind: strategy
  body: Cột phải của bảng là giá một ổ nhân với số ổ. Viết nó bằng con số 15000 và cái tên giữ số ổ, đừng viết con số kết quả — con số kết quả đổi theo từng dòng, mà bạn chỉ được viết đúng một dòng chữ cho cả sáu dòng bảng.
- kind: one-line
  body: "Chỗ trống thứ nhất là `15000 * so_o`; chỗ trống thứ hai là `15000 * so_o` y hệt."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống phải là câu tính `15000 * so_o`, viết bằng chính cái tên — chép cứng con số kết quả thì bảng chỉ còn đúng một dòng đúng, và cái bảng mà bài này nói tới không hiện ra ở đâu cả
  requireAst:
  # Hai chỗ trống, mỗi chỗ phải ĐỌC `so_o`. Khung khởi đầu đã đọc `so_o` một lần
  # trong chuỗi f (`{so_o}`), và hai dòng `so_o = ...` là chỗ ĐẶT tên nên không
  # tính — vậy điền bừa cho ra đúng 1, còn lời giải cho 3.
  - kind: uses-name, target: so_o, min: 3
  # Đọc tên thôi chưa đủ: `so_o` trơ một mình cũng đọc tên. Mỗi chỗ trống phải
  # có một phép nhân, vì cột phải là giá một ổ NHÂN số ổ. Khung khởi đầu không
  # có dấu nhân nào.
  - kind: uses-operator, target: *, min: 2
  forbidAst:
  # Lưới thứ hai, chặn ba con số KẾT QUẢ hay bị chép cứng nhất: dòng cuối của
  # vòng, dòng 12 ổ, và dòng ngay trước dòng cuối. Lời giải chỉ viết 15000 nên
  # luật này không cản ai làm thật.
  - kind: has-literal, target: 75000
  - kind: has-literal, target: 180000
  - kind: has-literal, target: 60000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^0 ổ thu 0 đồng\n1 ổ thu 15000 đồng\n2 ổ thu 30000 đồng\n3 ổ thu 45000 đồng\n4 ổ thu 60000 đồng\n5 ổ thu 75000 đồng\n180000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu dòng hiện ra từ một dòng chữ. Cái bảng nằm sẵn trong câu tính, mình chỉ chép
nó ra thôi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáng nay An sang phụ bán, và hai người tính tiền theo hai lối khác nhau.

Nhà Byte sáng nào cũng lấy sẵn **2 ổ** ăn trưa, và Byte vẫn tính tiền hai ổ ấy
như đã bán. Cho nên:

- **An** viết: *"Coi như bán được `n + 2` ổ, mỗi ổ 15 000 đ"* →  `15000 × (n + 2)`
- **Byte** viết: *"Bán `n` ổ được `15000 × n`, cộng thêm 30 000 của hai ổ nhà ăn"*
  →  `15000 × n + 30000`

Hai dòng chữ ấy trông chẳng giống nhau chút nào. Một bên có dấu ngoặc, một bên
không. Một bên có con số 2, một bên có con số 30000.

Bây giờ bạn đã có một cách so hai câu tính mà không cần cãi nhau: **so hai
bảng của chúng.**

Hai bảng ấy giống hay khác? Và nếu chúng giống nhau ở mấy dòng đầu, thế đã đủ để
nói hai câu tính là một chưa?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
