---
id: toan.dai-so-va-ham-so.mo-dau-ngoac
title: Mở dấu ngoặc
summary: Nhân một số vào một tổng thì phải nhân vào TỪNG cụm — và luật ấy chắc chắn đúng với mọi giá trị điền vào, vì nó chỉ là hai cách đếm cùng một khay bánh.
locale: vi
track: toan
module: dai-so-va-ham-so
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.expand-brackets]
requires: [math.expression, math.substitution, math.letter-names-a-slot, math.multiply-distributive, math.parentheses, math.order-of-operations, math.multiplication, core.variable, core.arithmetic, core.print-variable, core.output]
concepts: [math.mo-hinh-vung, math.o-trong, math.bieu-thuc-tuong-duong]
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
Sợi dây cắt ngang khay bánh không cần biết bên trái có mấy ổ.
::::

::::explain{#cai-bang-het-cach}
Bài trước để lại hai câu tính và một chỗ bí.

Sáng nào dì Tư ở đầu ngõ cũng lấy sẵn **2 ổ** bánh mì, không cần hỏi. Ngoài
chỗ đó, Byte bán thêm `n` ổ nữa. Mỗi ổ 15 000 đồng.

Hai người ghi tiền thu theo hai kiểu:

- Byte ghi: *tiền của n ổ, cộng tiền của 2 ổ dì Tư* → `15000 × n + 15000 × 2`
- An ghi: *gộp lại thành `n + 2` ổ rồi mới nhân* → `15000 × (n + 2)`

Bảng của hai câu ấy trùng khít nhau ở mọi dòng đã thử. Nhưng bảng chỉ có vài
chục dòng, còn `n` thì nhận **mọi** số. Thử tới hết là chuyện không làm được —
lúc nào cũng còn một số chưa thử.

Nên câu hỏi đổi hình dạng: không phải *"hai câu này có bằng nhau ở số tiếp theo
không"*, mà *"có lý do nào bắt chúng phải bằng nhau ở **mọi** số không"*.

Lý do ấy có. Và nó không phải một quy tắc mới — bạn đã cầm nó từ T2.1 rồi.
::::

::::explain{#khay-banh-cat-lam-hai}
Ở T2.1, Byte cắt một mảng 7 hàng × 13 cột bằng một sợi dây và viết được:

```text
7 × 13  =  (7 × 10) + (7 × 3)
```

Sợi dây không nhổ cây nào, không trồng thêm cây nào, nên đếm hai miếng rồi cộng
lại thì bằng đúng mảng ban đầu. Và có một chỗ hay sập: cắt **một** cạnh thì cạnh
kia phải **chép lại cho cả hai miếng** — cả hai miếng đều còn đủ 7 hàng.

Bây giờ đúng cái khay ấy, chỉ khác một chỗ: một cạnh là **ô trống**.

Byte xếp bánh thành một khay dài. Mỗi ổ đáng 15 000 đồng. Bên trái là `n` ổ bán
lẻ, bên phải là 2 ổ của dì Tư, và sợi dây căng đúng giữa hai phần:

```text
        ←──── n ổ ───→ ←2 ổ→
       ┌──────────────┬─────┐
 15000 │              │     │
  đồng │  15000 × n   │15000│
 mỗi ổ │              │ × 2 │
       └──────────────┴─────┘
```

Đọc khay này theo hai đường:

- **Nhìn cả khay**: nó dài `n + 2` ổ, mỗi ổ 15 000 đồng → `15000 × (n + 2)`.
- **Nhìn hai miếng**: miếng trái `15000 × n`, miếng phải `15000 × 2`, cộng lại
  → `15000 × n + 15000 × 2`.

Cùng một khay bánh, đếm hai đường, nên hai câu tính phải ra cùng một số tiền.

Và đây là chỗ đáng giá nhất của bài: **sợi dây không cần biết `n` bằng bao
nhiêu.** Nó cắt được ở đó dù bên trái có 3 ổ, 40 ổ hay 1 000 ổ. Cái khay không
đổi hình dạng khi bạn điền số vào — nó chỉ dài ra hay ngắn lại. Nên lý do "đếm
hai đường ra cùng một số" đứng vững ở **mọi** cách điền, chứ không phải ở những
số bạn kịp thử.

Đó là thứ cái bảng ở bài 5 không bao giờ cho được: bảng chỉ nói về những dòng
nó có. Cái khay nói về tất cả cùng một lúc.
::::

::::example{#hoi-thang-cai-may}
Bắt máy làm trọng tài. Điền hai số rất khác nhau vào cùng hai câu tính:

```python title=readonly
n = 3
print(15000 * (n + 2))
print(15000 * n + 15000 * 2)

n = 40
print(15000 * (n + 2))
print(15000 * n + 15000 * 2)
```

Máy in ra:

```text
75000
75000
630000
630000
```

Máy vừa xác nhận hai dòng, không hơn: ở `n = 3` chúng bằng nhau, ở `n = 40`
chúng bằng nhau. Nó **không** nói được gì về `n = 41`, và cũng không định nói.

Thứ nói được về mọi `n` là cái khay bánh, chứ không phải cái máy. Máy ở đây chỉ
làm một việc: nếu bạn đọc khay sai, nó sẽ tố ngay.
::::

::::predict{#doan-ba-dong commitOnce}
Byte thử viết dòng thứ ba theo trí nhớ — mở ngoặc ra nhưng chỉ nhân vào cụm đầu.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
n = 4
print(15000 * (n + 2))
print(15000 * n + 15000 * 2)
print(15000 * n + 2)
```

:::opt{correct}
90000 rồi 90000 rồi 60002
:::

:::opt
90000 rồi 90000 rồi 90000
::why
Gần đúng ở chỗ bạn tin rằng mở dấu ngoặc thì giá trị không được đổi — đó chính
là điều bài này vừa dựng, và bạn giữ nó rất chắc.

Chỗ lệch nằm ở **cụm nào được nhân**. Mở ngoặc là cắt cái khay, mà cắt một cạnh
thì cạnh kia phải chép lại cho **cả hai** miếng — con số 15 000 phải xuất hiện
đủ hai lần, một lần cho `n` ổ và một lần cho 2 ổ. Dòng ba chỉ chép nó cho miếng
trái. Miếng phải của nó không còn là "2 ổ bánh" nữa mà thành "2 đồng", nên nó
ra 60 002 chứ không phải 90 000. Máy vẫn tính, vẫn cho ra một con số — chỉ là
con số của một cái khay khác.
::
:::

:::opt
60002 rồi 90000 rồi 60002
::why
Gần đúng ở chỗ bạn nhớ luật nhân trước cộng sau và áp nó rất kỷ luật — ở dòng
một bạn làm `15000 × n` trước rồi mới cộng 2. Với một câu tính **không có
ngoặc** thì cách đọc ấy đúng hoàn toàn.

Ranh giới nằm ở chính cái ngoặc. Thứ tự phép toán là luật đọc **mặc định**, còn
dấu ngoặc là chỗ người viết tự vẽ ra cái khối mình muốn — và khối vẽ tay thì
được làm trước. Ở dòng một, `n + 2` được đóng thành **một** cụm: đó là tổng số
ổ trên khay, và 15 000 nhân vào cả cụm ấy. Nên dòng một ra 90 000. Dòng ba mới
là dòng không có ngoặc, và ở đó cách đọc của bạn cho ra đúng 60 002.
::
:::

:::opt
Máy dừng lại ở dòng đầu, vì `n + 2` là cộng một chữ cái với một con số
::why
Gần đúng ở chỗ bạn nhớ đúng một chuyện đã xảy ra thật: khi hai thứ không cùng
kiểu, máy dừng lại và báo lỗi chứ không đoán bừa. Cảnh giác ấy là thói quen tốt.

Ranh giới nằm giữa **chữ trên giấy** và **cái tên trong máy**. Trên giấy, `n` là
một ô trống — nó chưa giữ số nào, và đúng là chưa cộng được với 2. Trong đoạn
mã này thì dòng `n = 4` đã đứng sẵn ở trên: `n` là một cái tên, và nó đang giữ
con số 4. Máy đọc tên ra 4 rồi mới cộng, nên `n + 2` là `4 + 2`.

Đó cũng là lý do cả track dùng Python theo kiểu này: mỗi lần chạy, máy làm đúng
**một** dòng của cái bảng — nó điền số vào ô trống hộ bạn rồi tính. Cái ô trống
thì vẫn nằm trên giấy.
::
:::
::::

::::explain{#luat-doc-theo-mot-chieu}
Đặt tên cho thứ vừa thấy, để mang đi được:

> **Mở dấu ngoặc**: `a × (b + c) = a × b + a × c`
>
> Số đứng ngoài ngoặc phải nhân vào **từng** cụm bên trong, không sót cụm nào.

Ba chữ `a`, `b`, `c` ở đây là ba ô trống. Với mọi cách điền bằng **số ổ đếm
được**, câu trên chỉ là một cái khay chữ nhật cắt làm hai — nhìn cái khay là
thấy ngay. Luật này còn đúng rộng hơn thế nhiều (số âm, phân số), nhưng cái
khay thì không vẽ ra được ở đó; phần ấy để dành khi đã có đủ đồ nghề.

Vài chỗ nên để ý:

- **Cụm nào cũng phải được chia phần.** `4 × (n + 5 + 2)` mở ra thành
  `4 × n + 4 × 5 + 4 × 2`. Cắt khay thành ba miếng thì cạnh không bị cắt vẫn
  chép cho cả ba.
- **Trong ngoặc là dấu trừ thì dấu trừ đi theo.** `15000 × (n − 2)` là khay `n`
  ổ bị lấy bớt 2 ổ, nên nó thành `15000 × n − 15000 × 2`.
- **Cách viết gọn.** Giữa một con số và một chữ, dấu nhân được phép bỏ:
  `15000n` chính là `15000 × n`, chỉ ngắn hơn. Bài này viết cả hai kiểu để bạn
  quen mắt. Riêng Python thì không cho bỏ — trong mã vẫn phải gõ `15000 * n`.

Và câu tính của quán, viết gọn lại, thành:

```text
15000 × (n + 2)  =  15000n + 30000
```

Con số 30 000 ở vế phải không rơi từ trên trời: nó là `15000 × 2`, tức tiền của
đúng 2 ổ dì Tư — miếng bên phải của cái khay, đã tính xong thành một số.
::::

::::code{#hai-so-cua-mot-buoi}
Byte ghi hai cuốn sổ, mỗi cuốn một mặt hàng, và cuốn nào cũng đang ở dạng có
ngoặc. Việc của bạn là viết lại **dạng đã mở ngoặc** cho từng cuốn.

- **Sổ bánh mì**: mỗi ổ 15 000 đồng. Dì Tư lấy sẵn **2 ổ**, ngoài ra bán thêm
  `n` ổ.
- **Sổ nước**: mỗi chai 8 000 đồng. Quán tạp hoá gửi sẵn **3 chai**, ngoài ra
  bán thêm `m` chai.

Bài chấm bằng **cả hai cuốn sổ**, và chúng cố ý khác nhau ở cả giá lẫn số hàng
gửi sẵn — nên không có con số nào chép được từ chỗ trống này sang chỗ trống kia.
Mỗi dòng bạn viết còn được đối chiếu với chính dạng có ngoặc ở ngay trên nó:
mở ngoặc mà đổi giá trị thì lệch ngay.

Một luật của bài: **viết ra cả hai phép nhân, đừng nhân sẵn cụm thứ hai.** Ở
trên bài có ghi `15000 × (n + 2) = 15000n + 30000` — đúng, nhưng đó là dạng đã
đi thêm một bước nữa. Chỗ trống ở đây muốn thấy đúng cái bước MỞ NGOẶC:
`15000 * n + 15000 * 2`. Viết thẳng `15000 * n + 30000` là bạn đã nhân hộ máy
cụm thứ hai, và cái phép nhân thứ hai — thứ mà cả bài này dựng lên để nói tới —
không xuất hiện ở đâu cả. Rút gọn nốt là việc của bài 8.

Nhớ chỗ hay sập: giá một món là cạnh **không** bị cắt, nên nó phải có mặt ở
**cả hai** miếng.

```python title=starter
# Sổ bánh mì
n = 6
co_ngoac_banh = 15000 * (n + 2)
mo_ngoac_banh = ___

# Sổ nước
m = 11
co_ngoac_nuoc = 8000 * (m + 3)
mo_ngoac_nuoc = ___

print(mo_ngoac_banh)
print(mo_ngoac_nuoc)
```

```python title=solution
# Sổ bánh mì
n = 6
co_ngoac_banh = 15000 * (n + 2)
mo_ngoac_banh = 15000 * n + 15000 * 2

# Sổ nước
m = 11
co_ngoac_nuoc = 8000 * (m + 3)
mo_ngoac_nuoc = 8000 * m + 8000 * 3

print(mo_ngoac_banh)
print(mo_ngoac_nuoc)
```

```python title=test
# Câu `!=` đứng trước: nó canh cái bẫy chép một đáp án cho cả hai sổ. Xếp nó
# sau hai câu `==` thì nó không bao giờ chạy tới, và cái bẫy không bao giờ sập.
assert mo_ngoac_banh != mo_ngoac_nuoc, "hai sổ khác giá và khác số hàng gửi sẵn — một con số dùng cho cả hai là không được"
assert mo_ngoac_banh == co_ngoac_banh, "mở ngoặc không được đổi giá trị: dạng mở phải bằng đúng 15000 * (n + 2)"
assert mo_ngoac_nuoc == co_ngoac_nuoc, "mở ngoặc không được đổi giá trị: dạng mở phải bằng đúng 8000 * (m + 3)"
assert mo_ngoac_banh == 120000, "6 ổ bán thêm cộng 2 ổ của dì Tư, mỗi ổ 15 000 đồng, là 120 000 đồng"
assert mo_ngoac_nuoc == 112000, "11 chai bán thêm cộng 3 chai gửi sẵn, mỗi chai 8 000 đồng, là 112 000 đồng"
```

:::hints
- kind: attention
  body: Nhìn dòng ngay trên mỗi chỗ trống. Trong ngoặc có hai cụm — một cụm là chữ, một cụm là con số hàng gửi sẵn. Đếm xem con số giá tiền phải xuất hiện mấy lần trong dòng bạn sắp viết.
- kind: strategy
  body: Cắt khay làm hai miếng. Miếng trái là giá nhân với chữ, miếng phải là giá nhân với số hàng gửi sẵn, rồi cộng hai miếng. Giá tiền là cạnh không bị cắt, nên nó viết lại đủ hai lần trong mỗi dòng.
- kind: one-line
  body: "Chỗ trống thứ nhất là `15000 * n + 15000 * 2`, chỗ trống thứ hai là `8000 * m + 8000 * 3`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là hai miếng cộng lại, và giá tiền phải có mặt ở cả hai miếng — chép lại dạng có ngoặc hoặc gõ thẳng số tiền thì cái luật bài này vừa dựng không xuất hiện ở đâu cả
  requireAst:
  # Dạng mở ngoặc có bốn phép nhân (hai miếng mỗi sổ); khung khởi đầu chỉ có
  # hai. `min: 6` vì hai phép nhân của khung vẫn còn nguyên trong lời giải.
  - kind: uses-operator, target: *, min: 6
  # Khung khởi đầu có đúng hai dấu cộng, nằm trong hai cặp ngoặc. Lời giải thêm
  # hai dấu cộng nữa — mỗi sổ một dấu nối hai miếng.
  - kind: uses-operator, target: +, min: 4
  # Mỗi chữ phải được ĐỌC thêm một lần nữa ở dòng mở ngoặc. Không có hai luật
  # này thì `mo_ngoac_banh = co_ngoac_banh` cũng lọt, mà đó là chép chứ không
  # phải mở.
  - kind: uses-name, target: n, min: 2
  - kind: uses-name, target: m, min: 2
  forbidAst:
  # Lưới thứ hai: chặn đúng hai con số KẾT QUẢ. Lời giải thật không chứa nguyên
  # văn chúng, nên luật này không cản ai làm thật.
  - kind: has-literal, target: 120000
  - kind: has-literal, target: 112000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^120000\n112000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
120 nghìn với 112 nghìn. Cắt khay làm hai miếng, đếm lại vẫn đủ từng ấy tiền.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay bạn đi một chiều: có ngoặc → bỏ ngoặc. `15000 × (n + 2)` mở ra thành
`15000n + 30000`.

Nhưng sợi dây thì không có chiều. Nó chỉ nằm đó chia khay làm hai miếng — và
người ta cắt được thì cũng gỡ dây ra, ghép hai miếng lại được.

Vậy thử đi ngược xem. Đưa bạn `15000n + 30000` — một câu tính **không có dấu
ngoặc nào** — bạn có dựng lại được cái khay không?

Chỗ vướng nằm ngay chỗ nhìn. Trong `15000 × n + 15000 × 2` thì con số 15 000
đứng chình ình hai lần, ai cũng thấy. Còn trong `15000n + 30000` thì nó chỉ còn
lại **một** lần: cụm thứ hai đã bị tính xong thành 30 000, và cái 15 000 trong
đó biến mất khỏi mặt giấy.

Muốn gấp lại thì phải lấy cái gì làm **cái chung** — và tìm nó ở đâu, khi nó
không còn hiện ra?

Bài sau đọc đúng luật hôm nay theo chiều ngược lại.
::::

::::checkpoint{mastery=0.8}
::::
