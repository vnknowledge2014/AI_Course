---
id: toan.cam-nhan-so.bang-vi-tri-la-day-luy-thua
title: Bảng vị trí là một dãy luỹ thừa
summary: Mỗi bước sang trái trong bảng vị trí là đóng bó thêm một lần, nên các cột chính là 10⁰, 10¹, 10², … — và cột "một" đáng 1 vì hạt chưa đóng bó vẫn là hạt.
locale: vi
track: toan
module: cam-nhan-so
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.place-value-powers]
requires: [math.dong-goi, math.multiplication, core.arithmetic, core.number-literal, core.output, core.variable]
concepts: [math.bang-vi-tri, math.so-mu-khong, math.mo-bo]
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
Mấy cái cột trong bảng vị trí không phải bốn thứ rời nhau. Chúng là một dãy.
::::

::::explain{#cot-mot-la-muoi-mu-may}
Bài trước để lại một câu hỏi thẳng: `10¹ = 10`, `10² = 100`, `10³ = 1000` —
nhìn quen, vì đó đúng là mấy cái cột bạn đã dùng từ bài 7. Thế **cột hạt lẻ**,
cột "một", là 10 mũ mấy?

Trước khi trả lời, nhìn lại cái vườn. Byte đếm hạt bằng luật đóng bó ở bài 6:
đủ mười hạt thì buộc lại thành **một bó**; đủ mười bó thì lại buộc thành **một
bó to hơn**. Cùng một luật, lặp lại. Mỗi cột trong bảng vị trí là một mức bó:

| Cột | Một cái ở cột này là gì | Đáng mấy hạt |
|---|---|---|
| một | một hạt, chưa buộc vào đâu cả | 1 |
| chục | một bó mười hạt | 10 |
| trăm | một bó gồm mười bó | 100 |
| nghìn | một bó gồm mười bó-của-bó | 1000 |

Đọc bảng từ dưới lên: **mỗi bước sang trái là đóng bó thêm đúng một lần**, và
mỗi lần đóng bó thì giá của một cái ở cột đó **nhân thêm 10**.

Đặt cạnh luỹ thừa của bài trước thì hai chuyện khớp vào nhau: số mũ đếm *số
thừa số 10*, còn số lần đóng bó cũng đếm đúng chừng ấy lần nhân 10. Vậy **số mũ
chính là số lần đã đóng bó**:

- cột nghìn — đóng bó ba lần — `10³`
- cột trăm — đóng bó hai lần — `10²`
- cột chục — đóng bó một lần — `10¹`
- cột một — đóng bó **không lần nào** — `10⁰`

Bây giờ câu hỏi đầu bài trả lời được, và câu trả lời không cần ai cho phép: cột
"một" là `10⁰`. Còn giá của nó thì đi ngược lại đúng một bước là thấy. Từ cột
chục sang cột một, ta **mở** một bó ra: một bó mười hạt mở ra thành mười hạt
rời, mỗi hạt là một cái ở cột "một". Một hạt chưa buộc vào đâu vẫn là **một
hạt**.

> `10⁰ = 1` không phải một quy ước ai đó áp xuống cho gọn. Nó là câu "chưa đóng
> bó lần nào thì hạt vẫn là hạt", viết lại bằng ký hiệu luỹ thừa.

Có một bức tranh thứ hai nói cùng điều ấy, cho ai thấy thanh số dễ hình dung
hơn cái vườn. Ở bài 22, nhân 10 là **kéo giãn thanh số ra mười lần**. Sang trái
một cột là kéo giãn thêm một lần. Vậy `10⁰` là kéo giãn **không lần nào** — mọi
thứ nằm nguyên chỗ cũ. Và cái mốc "nguyên chỗ cũ" của phép nhân là số `1`, chứ
không phải số `0`: nhân với 1 thì không xê dịch gì, còn nhân với 0 thì kéo sập
cả thanh số về một điểm.
::::

::::example{#hoi-thang-cai-may}
Trong Python, `10 ** 3` là cách gõ `10³` — hai dấu sao viết liền nhau, thừa số
đứng trước, số mũ đứng sau. Hỏi thẳng máy bốn cột của bảng vị trí:

```python title=readonly
print(10 ** 3)
print(10 ** 2)
print(10 ** 1)
print(10 ** 0)
```

Máy in ra:

```text
1000
100
10
1
```

Bốn dòng ấy đọc từ dưới lên là đúng bốn ô trong cột phải của cái bảng phía
trên: `1`, `10`, `100`, `1000`. Không phải bốn con số rời nhau ai đó chép vào
sổ — chúng là **một dãy**, và mỗi dòng cách dòng dưới nó đúng một lần đóng bó.

Để ý riêng dòng cuối. Máy không kêu ca gì về `10 ** 0`, và nó cũng không trả về
`0`. Nó trả về `1` — cùng con số mà cái vườn vừa cho bạn.
::::

::::predict{#doan-cot-mot commitOnce}
Byte gõ lại đúng dòng cuối ấy, một mình nó.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
print(10 ** 0)
```

:::opt{correct}
1
:::

:::opt
0
::why
Gần đúng ở chỗ bạn đọc số mũ `0` là "không có lần nào", và cách đọc ấy chính
xác. Trong phép **cộng** thì "không lần nào" thật sự cho ra `0`: cộng thêm
không lần nào vào một cái sổ trống thì sổ vẫn ghi 0.

Chỗ lệch là luỹ thừa đếm thừa số của phép **nhân**, không phải số hạng của phép
cộng — và hai phép ấy xuất phát từ hai mốc khác nhau. Mốc của phép cộng là `0`
(cộng thêm 0 thì không đổi); mốc của phép nhân là `1` (nhân với 1 thì không
đổi). Nhân thêm không lần nào thì còn nguyên cái mốc `1`.

Cái vườn nói cùng điều ấy bằng hạt: cột "một" là cột chưa đóng bó lần nào, mà
một hạt chưa đóng bó vẫn là một hạt, không phải không có hạt nào.
::
:::

:::opt
10
::why
Gần đúng ở chỗ bạn giữ lại con số `10` đứng trước, và với `10¹` thì đọc như thế
là trúng: một thừa số 10 thì kết quả đúng bằng 10.

Chỗ lệch nằm ở vai của hai con số. Con số `10` viết trước chỉ là **tên của thừa
số** — nó nói "cái được nhân đi nhân lại là 10". Số mũ mới nói **có mấy thừa
số**. Số mũ `0` nghĩa là không lấy thừa số nào cả, nên con số `10` kia không
được đem vào phép nhân lần nào.

Trên bảng vị trí, đó là chỗ khác nhau giữa cột chục và cột một: cột chục giữ
những cái bó mười, cột một giữ những hạt chưa buộc.
::
:::

:::opt
Máy dừng lại và báo lỗi
::why
Gần đúng ở chỗ bạn nhớ máy có dừng hẳn lại và nói ra khi câu hỏi thật sự hỏng —
bạn đã gặp đúng chuyện đó với `TypeError` và `ValueError`. Cách nghĩ ấy còn
dùng lại được ở vài bài nữa, khi có một phép tính hỏng thật.

Chỗ lệch: câu `10 ** 0` không hỏng. Nó hỏi "lấy mấy thừa số 10", và "không lấy
cái nào" là một câu trả lời nói được — giống như "trong giỏ có 0 quả cam" là
một câu nói được, không phải một câu vô nghĩa.
::
:::
::::

::::explain{#doc-mot-con-so-thanh-may-cot}
Nắm được dãy `10⁰, 10¹, 10², 10³` thì một con số nhiều chữ số không còn là một
xâu ký hiệu phải học thuộc. Nó là một **lời khai** về cái vườn:

`247` = 2 bó-của-bó + 4 bó + 7 hạt lẻ

và viết bằng luỹ thừa thì đúng một dòng:

`247 = 2 × 10² + 4 × 10¹ + 7 × 10⁰`

Con số ở vị trí nào cho biết **lấy mấy cái**; còn cột nó đứng cho biết **mỗi
cái đáng bao nhiêu**. Đó là hai vai bạn đã gặp từ bài 19: một con số nói lô to
bao nhiêu, con số kia nói lấy mấy lô.
::::

::::code{#dung-lai-so-hat}
Byte kiểm kho hạt sau vụ vừa rồi và ghi: **2 bó-của-bó, 4 bó, 7 hạt lẻ**.

Ba chỗ trống dưới đây là **số mũ** của ba cột — tức số lần đã đóng bó ở mỗi
cột. Điền xong thì dòng cuối dựng lại tổng số hạt trong kho.

Bài chấm bằng cả bốn dòng in ra, và ba cột được chọn để **không cột nào giống
cột nào**: điền cùng một con số vào cả ba chỗ thì nhiều nhất chỉ đúng được một
cột, ba dòng còn lại sai ngay.

```python title=starter
gia_cot_mot = 10 ** ___
gia_cot_chuc = 10 ** ___
gia_cot_tram = 10 ** ___

so_hat = 2 * gia_cot_tram + 4 * gia_cot_chuc + 7 * gia_cot_mot

print(gia_cot_mot)
print(gia_cot_chuc)
print(gia_cot_tram)
print(so_hat)
```

```python title=solution
gia_cot_mot = 10 ** 0
gia_cot_chuc = 10 ** 1
gia_cot_tram = 10 ** 2

so_hat = 2 * gia_cot_tram + 4 * gia_cot_chuc + 7 * gia_cot_mot

print(gia_cot_mot)
print(gia_cot_chuc)
print(gia_cot_tram)
print(so_hat)
```

```python title=test
# Ba cột, ba giá khác nhau — nên một con số gõ cứng vào cả ba chỗ trống không
# thể qua nổi ba dòng đầu. Dòng thứ tư khoá lại bằng tổng thật của kho hạt.
assert gia_cot_mot == 1, "cột một là 10 mũ 0 — chưa đóng bó lần nào, hạt vẫn là hạt, nên đáng 1"
assert gia_cot_chuc == 10, "cột chục là 10 mũ 1 — đóng bó đúng một lần, mỗi bó mười hạt"
assert gia_cot_tram == 100, "cột trăm là 10 mũ 2 — đóng bó hai lần, mỗi cái là bó của mười bó"
assert so_hat == 247, "2 bó-của-bó + 4 bó + 7 hạt lẻ thì kho có 247 hạt"
assert gia_cot_tram == gia_cot_chuc * 10, "mỗi bước sang trái là đóng bó thêm một lần, tức nhân thêm 10"
assert gia_cot_chuc == gia_cot_mot * 10, "luật ấy đúng ở cả bước cuối cùng — chính chỗ này bảo đảm 10 mũ 0 bằng 1"
```

:::hints
- kind: attention
  body: Chỗ trống nằm ngay sau `10 **`, nên thứ điền vào là **số mũ**, không phải giá của cột. Bảng ở đầu bài đã nói mỗi cột đã đóng bó mấy lần.
- kind: strategy
  body: Đếm số lần đóng bó cho từng cột, bắt đầu từ cột "một". Hạt lẻ chưa buộc vào đâu, nên số lần đóng bó của nó là con số nhỏ nhất có thể đếm được. Cột chục hơn nó một lần, cột trăm hơn hai lần.
- kind: one-line
  body: "Ba chỗ trống lần lượt là `0`, `1` và `2`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^1\n10\n100\n247\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
247 hạt. Bốn cột, một cái luật — và mình chỉ đóng bó thêm một lần mỗi bước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Suốt bài này bạn đi từ trái sang phải bằng cách **mở bó**: bó-của-bó mở ra
thành mười bó, bó mở ra thành mười hạt. Mỗi lần mở là tụt xuống một cột, và số
mũ tụt xuống một bậc.

Mở tới cột "một" thì hết đường: một hạt rời không mở ra được nữa mà vẫn còn là
hạt — đó chính là bài 4, cái thước không bẻ được.

Nhưng hãy nghe kỹ lại cái động tác bạn vừa làm mấy chục lần. "Mở một bó mười ra
thành mười hạt rời" — nói cách khác là **đem một bó chia đều cho mười chỗ, mỗi
chỗ một hạt**.

Byte vừa dùng chữ "chia". Suốt 24 bài, Byte cộng, trừ, nhân, đóng bó, mở bó —
mà chưa hề chia lần nào. Vậy **chia là gì**?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
