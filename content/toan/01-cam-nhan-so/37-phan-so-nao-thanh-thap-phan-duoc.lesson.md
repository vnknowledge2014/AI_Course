---
id: toan.cam-nhan-so.phan-so-nao-thanh-thap-phan-duoc
title: Phân số nào viết được thành thập phân
summary: Thập phân chỉ có mẫu 10, 100, 1000 rồi xa hơn nữa. Phân số nào kéo được mẫu về một cột của bảng ấy thì viết ra rồi dừng; `1/3` thì không, và chỗ dư cứ quay lại y hệt.
locale: vi
track: toan
module: cam-nhan-so
order: 37
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.terminating-decimal]
requires: [math.fraction, math.remainder, core.arithmetic, core.variable, core.assignment, core.number-literal, core.print-variable, ctrl.comparison]
concepts: [math.phan-so, math.thap-phan, math.don-vi-do]
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
Có cái thước bẻ mãi cũng trúng cột. Có cái bẻ hoài không bao giờ trúng.
::::

::::explain{#cau-hoi-con-treo}
Bài trước để lại một câu hỏi. Byte đong gạo: cái ca `1/4` lon viết được thành
`0,25` — hai chữ số, xong. Còn cái ca `1/3` lon thì viết ra số nào?

Nhớ lại bài 36 đã dựng cái gì. Bảng giá trị vị trí kéo sang bên phải dấu phẩy
thì mỗi cột là một cái thước nhỏ dần: cột thứ nhất là `1/10`, cột thứ hai là
`1/100`, cột thứ ba là `1/1000`. Viết `0,25` tức là nói: **hai** cái thước
`1/10`, rồi **năm** cái thước `1/100`. Mẫu do vị trí lo, khỏi phải viết ra.

Chỗ quan trọng nằm ở đây: bảng ấy chỉ có **ba loại thước** — `1/10`, `1/100`,
`1/1000`, rồi nhỏ nữa cũng vẫn cùng một họ. Số thập phân không có cột nào tên
là "một phần ba" cả.

Nên câu hỏi "`1/3` viết thành thập phân được không" thật ra là câu hỏi khác,
gọn hơn nhiều:

> Cái thước `1/3` có ghép lại được từ những cái thước `1/10`, `1/100`,
> `1/1000` không?

Và bài 33 cho sẵn cách hỏi ngược lại. Nhân cả tử lẫn mẫu với cùng một số là
**đo lại đúng lượng ấy bằng thước nhỏ hơn** — lượng không đổi, chỉ tên đổi.
Vậy `1/4` viết được thành `0,25` là vì kéo được mẫu 4 lên đúng 100:

```text
1/4  =  (1 × 25) / (4 × 25)  =  25/100  =  0,25
```

Phép thử của cả bài này chỉ có thế: **kéo mẫu về được một cột của bảng — 10,
100, 1000, hay xa hơn nữa — thì viết được thành thập phân, và viết xong là
dừng.**
::::

::::predict{#doan-ca-mot-phan-tam commitOnce}
Byte cầm cái ca `1/8` lon. **Trước khi đọc tiếp**, bạn đoán viết nó thành thập
phân thì ra số nào?

:::opt{correct}
0,125 — viết ba chữ số sau dấu phẩy rồi dừng
:::

:::opt
0,8 — số 8 ở dưới gạch, kéo xuống làm chữ số sau dấu phẩy
::why
Gần đúng ở chỗ bạn nhớ đúng bài 36: cột đầu tiên sau dấu phẩy là cột **phần
mười**, nên một chữ số đứng ở đó thật sự nói tới những cái thước `1/10`.

Chỗ lệch nằm ở việc con số `8` đang giữ vai gì. `0,8` nghĩa là `8/10` — **tám**
cái thước cỡ một phần mười, tức là gần hết cả lon. Còn `1/8` là **một** cái
thước cỡ một phần tám, nhỏ hơn cả một phần năm. Bài 31 đã tách hai vai ấy ra:
mẫu nói *thước cỡ nào*, tử nói *lấy mấy cái*. Kéo số ở mẫu xuống làm số lượng
là đổi vai của nó.
::
:::

:::opt
Không viết được, vì 8 không phải 10, 100 hay 1000
::why
Gần đúng ở chỗ bạn dùng đúng phép thử của bài này: thập phân chỉ có mẫu 10,
100, 1000, nên mẫu 8 đúng là không nằm sẵn trong bảng.

Chỗ lệch: phép thử không đòi mẫu phải **sẵn** là 10, 100 hay 1000 — nó cho
phép kéo mẫu lên, đúng như bài 33 cho phép. Nhân cả tử lẫn mẫu với 125 thì
`1/8` thành `125/1000`, cùng một lượng gạo, chỉ đo bằng thước nhỏ hơn 125 lần.
Nếu đòi mẫu phải sẵn tròn thì `1/4` cũng trượt, mà `1/4 = 0,25` thì bài trước
đã viết ra rồi.
::
:::

:::opt
0,1 rồi một cái đuôi không bao giờ dừng, giống 1/3
::why
Gần đúng ở chỗ bạn cảnh giác đúng chỗ: có những mẫu chẳng bao giờ kéo về tròn
được, và bạn nhớ `1/3` là một trong số đó. Với mẫu 3, 6, 7 hay 9 thì cảnh giác
ấy trúng.

Chỗ lệch nằm ở việc 8 khác 3 ở đâu. Cứ thử kéo mẫu xem: `8 × 125 = 1000`,
trúng đúng một cột của bảng, nên `1/8` viết ra rồi dừng. Còn mẫu 3 thì nhân với
số nguyên nào cũng trượt qua cột — phần ngay sau đây sẽ chỉ ra vì sao nó trượt
mãi mãi. Hai mẫu trông đều "không phải số tròn", nhưng chỉ một trong hai thật
sự nằm ngoài.
::
:::
::::

::::example{#keo-mau-ve-tron}
Thử phép thử ấy trên vài cái ca đong gạo trong bếp nhà Byte — kể cả cái ca
vừa đoán.

```text
1/5   →  5 × 2   = 10     →  2/10   = 0,2       dừng
1/4   →  4 × 25  = 100    →  25/100 = 0,25      dừng
1/20  →  20 × 5  = 100    →  5/100  = 0,05      dừng
1/8   →  8 × 125 = 1000   →  125/1000 = 0,125   dừng
```

Bốn lần đều trúng. Giờ tới `1/3`, và lần này không trúng lần nào:

```text
3 × 3   = 9        thiếu 1 so với 10
3 × 4   = 12       vượt 10
3 × 33  = 99       thiếu 1 so với 100
3 × 34  = 102      vượt 100
3 × 333 = 999      thiếu 1 so với 1000
3 × 334 = 1002     vượt 1000
```

Ba lần đều cùng một hình: `1000` **lọt vào giữa** hai bước liền nhau của bảng
nhân 3 — `3 × 333 = 999` rồi `3 × 334 = 1002`. Nhân 3 thì nhảy từng bước 3 một,
mà `1000` rơi vào giữa một bước, nên không có chỗ nào cho nó đứng.

Vì sao mãi mãi như thế, chứ không phải "chưa tìm ra số đủ lớn"? Nhìn lại cách
`1000` được dựng lên, đúng bằng luật đóng bó của bài 6 và 7. Mười là **hai lần
năm**. Trăm là mười lần mười. Nghìn là trăm lần mười. Nên `1000` chỉ được ghép
từ hai loại bó: **bó đôi** và **bó năm**.

Bẻ `1000` ra thành các phần bằng nhau thì cũng chỉ đi qua đúng hai bước ấy —
bẻ đôi, hoặc bẻ năm. Bẻ đôi mãi ra `2, 4, 8`. Bẻ năm mãi ra `5, 25, 125`. Trộn
hai kiểu ra `10, 20, 40, 50, 100, 200, 250, 500`. Cả danh sách:

```text
2  4  5  8  10  20  25  40  50  100  125  200  250  500  1000
```

Danh sách này mới dừng ở `1000`. Bảng thì còn chạy tiếp — `10000`, `100000` —
nên bẻ đôi bốn lần ra `16` cũng trúng: `16 × 625 = 10000`, tức
`1/16 = 0,0625`. Thứ quyết định không phải con số `1000`, mà là mẫu có ghép
được **chỉ từ bó đôi với bó năm** hay không.

Trong đó không có `3`. Cũng không có `6`, `7`, `9`. Chúng không phải bó đôi,
không phải bó năm, và không ghép được từ hai thứ ấy — nên bảng có chạy xa tới
đâu, chúng vẫn nằm ngoài.

Còn nếu cứ chia bừa `1` cho `3` theo kiểu mở bó thì chuyện xảy ra thế này:
mở một lon thành 10 phần, chia cho 3 được 3 phần, **dư 1**. Mở cái dư 1 ấy
thành 10 phần nhỏ hơn, chia cho 3 được 3 phần, **dư 1**. Số dư quay về đúng
cái nó vừa là. Cùng một tình huống lặp lại thì cho ra cùng một kết quả, mãi
mãi — nên `1/3 = 0,333…` và cái đuôi ấy không có chỗ dừng.
::::

::::code{#hoi-may-xem-co-trung-khong}
Việc còn lại của bài: đem cái phép thử vừa dựng ra hỏi máy. Máy không biết gì
về gạo hay về thước — nó chỉ nhân hai số rồi cho bạn xem kết quả. Nhưng mấy kết
quả đặt cạnh nhau đủ để bạn thấy mẫu nào trúng cột và mẫu nào không.

Mỗi dòng là một lần chạy phép thử trên một cái ca: kéo mẫu về đúng `1000` bằng
phép nhân với một **số nguyên**, rồi xem có trúng hay không.

Ba cái ca cho ra hai loại kết quả **ngược nhau**: ca `1/8` và ca `1/40` trúng
đúng cột, còn ca `1/3` thì chỉ kẹp được `1000` vào giữa. Điền cứng một con số
vào cả ba chỗ trống thì ít nhất một dòng sai.

```python title=starter
# Ca 1/8 lon — bẻ đôi ba lần, thử số 125
tam_nhan = 8 * 125

# Ca 1/40 lon — vừa bó đôi vừa bó năm. Nhân 40 với số nào thì trúng đúng 1000?
bon_muoi_nhan = ___

# Ca 1/3 lon — không số nguyên nào trúng, chỉ kẹp được 1000 vào giữa:
# một phép nhân dừng lại ngay dưới 1000, một phép nhân vừa vượt qua nó.
ba_nhan_duoi = ___
ba_nhan_tren = ___

print(tam_nhan)
print(bon_muoi_nhan)
print(ba_nhan_duoi)
print(ba_nhan_tren)
```

```python title=solution
# Ca 1/8 lon — bẻ đôi ba lần, thử số 125
tam_nhan = 8 * 125

# Ca 1/40 lon — vừa bó đôi vừa bó năm. Nhân 40 với số nào thì trúng đúng 1000?
bon_muoi_nhan = 40 * 25

# Ca 1/3 lon — không số nguyên nào trúng, chỉ kẹp được 1000 vào giữa:
# một phép nhân dừng lại ngay dưới 1000, một phép nhân vừa vượt qua nó.
ba_nhan_duoi = 3 * 333
ba_nhan_tren = 3 * 334

print(tam_nhan)
print(bon_muoi_nhan)
print(ba_nhan_duoi)
print(ba_nhan_tren)
```

```python title=test
# Bốn phép thử trên ba cái ca. Hai ca kéo được mẫu về đúng cột 1000, một ca
# thì không số nguyên nào trúng — đó đúng là câu mà tiêu đề bài đang hỏi.
assert tam_nhan == 1000, "8 nhân 125 trúng đúng 1000 — nên 1/8 = 125/1000 = 0,125, viết xong là dừng"
assert bon_muoi_nhan == 1000, "40 nhân 25 cũng trúng đúng 1000 — nên 1/40 = 25/1000 = 0,025, cũng dừng"
assert ba_nhan_duoi == 999, "3 nhân 333 mới được 999, còn thiếu 1 nữa mới tới 1000"
assert ba_nhan_tren == 1002, "3 nhân 334 đã vọt lên 1002 — vượt qua 1000 mất rồi"
assert ba_nhan_duoi < 1000 < ba_nhan_tren, "999 và 1002 là hai bước liền nhau của bảng nhân 3, cách nhau đúng 3 — 1000 lọt vào giữa nên không số nguyên nào nhân với 3 ra đúng nó"
```

:::hints
- kind: attention
  body: Nhìn dòng đã viết sẵn ở trên. Nó có đúng ba mảnh: mẫu của cái ca, dấu nhân, và con số đem thử. Ba dòng của bạn cũng cần đúng ba mảnh ấy, chỉ khác con số.
- kind: strategy
  body: "Ca 1/40 thì hỏi ngược lại: 1000 gồm mấy lần 40? `40 * 20 = 800`, còn thiếu 200 nữa, mà 200 lại đúng bằng `40 * 5` — vậy con số cần thử là 20 cộng 5. Ca 1/3 thì bảng dò ở phần trước đã tìm sẵn hai con số: một số nhân 3 ra 999, số liền sau nó nhân 3 ra 1002."
- kind: one-line
  body: "Viết `40 * 25` vào chỗ trống thứ nhất, `3 * 333` vào chỗ thứ hai và `3 * 334` vào chỗ thứ ba."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một phép nhân thật giữa mẫu của cái ca và con số đem thử — chép sẵn kết quả vào thì máy không nhân hộ bạn lần nào
  requireAst:
  # `min: 4` vì có bốn phép thử: 8×125 đã viết sẵn, cộng ba phép của bạn.
  # Khung khởi đầu chỉ có một dấu nhân, nên luật này chặn được đáp án gõ cứng.
  - kind: uses-operator, target: *, min: 4
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^1000\n1000\n999\n1002\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
999 rồi 1002. Cái cột 1000 nằm đúng vào khe giữa, mình với tay không tới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mẫu 10, 100, 1000 tiện thật. Tiện nhất là chúng luôn **thẳng cột**: viết `0,25`
dưới `0,05` thì hai chữ số 2 và 0 nằm đúng một hàng, khỏi phải quy đồng lần
nào, vì bảng giá trị vị trí đã quy đồng sẵn hộ rồi.

Nhưng chú ý một chuyện. Sáng nay Byte đếm vườn nhà mình: 40 cây, sâu ăn mất 14
cây. Vườn nhà An to hơn nhiều: 250 cây, sâu ăn mất 80 cây. Byte muốn biết vườn
nào bị nặng hơn — mà `14/40` với `80/250` là hai cái thước khác nhau, bài 34 đã
nói rõ là chưa so được.

Chuyện lạ là ngoài chợ, ngoài đài, trên nhãn chai nước mắm, người ta gặp đúng
loại câu hỏi này suốt ngày. Và họ không dùng cả ba mẫu 10, 100, 1000. Họ dùng
đúng **một** mẫu duy nhất, lần nào cũng nó.

Mẫu nào, và vì sao lại là nó chứ không phải hai mẫu kia? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
