---
id: toan.cam-nhan-so.phan-so-nao-thanh-thap-phan-duoc
title: Phân số nào viết được thành thập phân
summary: Thập phân chỉ có mẫu 10, 100, 1000. Phân số nào kéo được mẫu về đó thì viết ra rồi dừng; `1/3` thì không, và chỗ dư cứ quay lại y hệt.
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

Phép thử của cả bài này chỉ có thế: **kéo mẫu về đúng 10, 100 hay 1000 được
thì viết được thành thập phân, và viết xong là dừng.**
::::

::::example{#keo-mau-ve-tron}
Thử phép thử ấy trên vài cái ca đong gạo trong bếp nhà Byte.

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

Ba lần đều cùng một hình: `1000` **lọt vào giữa** hai số nguyên liền nhau
`999` và `1002`, không có chỗ nào cho nó đứng.

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

Trong đó không có `3`. Cũng không có `6`, `7`, `9`. Chúng không phải bó đôi,
không phải bó năm, và không ghép được từ hai thứ ấy.

Còn nếu cứ chia bừa `1` cho `3` theo kiểu mở bó thì chuyện xảy ra thế này:
mở một lon thành 10 phần, chia cho 3 được 3 phần, **dư 1**. Mở cái dư 1 ấy
thành 10 phần nhỏ hơn, chia cho 3 được 3 phần, **dư 1**. Số dư quay về đúng
cái nó vừa là. Cùng một tình huống lặp lại thì cho ra cùng một kết quả, mãi
mãi — nên `1/3 = 0,333…` và cái đuôi ấy không có chỗ dừng.
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

Chỗ lệch nằm ở việc 8 khác 3 ở đâu. `8` là **bẻ đôi ba lần** — mà `1000` thì
bẻ đôi được. `3` không đến từ bẻ đôi, cũng không đến từ bẻ năm, nên nó nằm
ngoài danh sách. Hai mẫu trông đều "không phải số tròn", nhưng chỉ một trong
hai thật sự nằm ngoài.
::
:::
::::

::::explain{#viet-vao-may-thi-dung-dau-cham}
Một chỗ khác nhau nhỏ giữa vở toán và màn hình, nói trước để khỏi vướng:
trên giấy người Việt viết `0,25`, còn Python viết `0.25` bằng dấu chấm. Cùng
một con số, hai cách chấm câu. Bài này viết bằng dấu phẩy ở phần văn xuôi và
bằng dấu chấm trong code, và hai chỗ đó nói cùng một điều.

Việc còn lại của bài: đem cái phép thử vừa dựng ra hỏi máy. Máy không cần biết
gì về gạo hay về thước — nó chỉ nhân hai số rồi cho bạn xem kết quả. Nhưng ba
kết quả đặt cạnh nhau đủ để bạn thấy `1000` lọt vào giữa.
::::

::::code{#hoi-may-xem-co-trung-khong}
Kéo mẫu về đúng `1000` bằng phép nhân: mẫu nhân với một **số nguyên**, xem có
trúng đúng `1000` hay không.

Hai cái ca được chọn để cho ra hai câu trả lời **ngược nhau**: ca `1/8` trúng,
ca `1/3` thì kẹp `1000` vào giữa. Điền cứng một con số vào cả hai chỗ trống thì
ít nhất một dòng sai.

```python title=starter
# Ca 1/8 lon — thử số 125
tam_nhan_125 = 8 * 125

# Ca 1/3 lon — 333 là số lớn nhất còn dưới, 334 là số nhỏ nhất đã vượt
ba_nhan_333 = ___
ba_nhan_334 = ___

print(tam_nhan_125)
print(ba_nhan_333)
print(ba_nhan_334)
```

```python title=solution
# Ca 1/8 lon — thử số 125
tam_nhan_125 = 8 * 125

# Ca 1/3 lon — 333 là số lớn nhất còn dưới, 334 là số nhỏ nhất đã vượt
ba_nhan_333 = 3 * 333
ba_nhan_334 = 3 * 334

print(tam_nhan_125)
print(ba_nhan_333)
print(ba_nhan_334)
```

```python title=test
# Ba assert này chốt đúng ba câu của bài, trên hai cái ca khác nhau:
# một ca trúng cột, một ca không bao giờ trúng.
assert tam_nhan_125 == 1000, "8 nhân 125 trúng đúng 1000 — nên 1/8 = 125/1000 = 0,125, viết xong là dừng"
assert ba_nhan_333 == 999, "3 nhân 333 mới được 999, còn thiếu 1 nữa mới tới 1000"
assert ba_nhan_334 == 1002, "3 nhân 334 đã vọt lên 1002 — vượt qua 1000 mất rồi"
assert ba_nhan_333 < 1000 < ba_nhan_334, "1000 lọt vào GIỮA hai số nguyên liền nhau, nên không số nguyên nào nhân với 3 ra đúng nó"
```

:::hints
- kind: attention
  body: Nhìn dòng đã viết sẵn ở trên. Nó có đúng ba mảnh: mẫu của cái ca, dấu nhân, và con số đem thử. Hai dòng của bạn cũng cần đúng ba mảnh ấy, chỉ khác con số.
- kind: strategy
  body: Mẫu của ca thứ hai là 3, và hai con số đem thử đã ghi ngay trong lời chú thích — 333 rồi 334. Việc của bạn là để máy nhân hộ, chứ không phải tự nhân trong đầu rồi chép kết quả vào.
- kind: one-line
  body: "Viết `3 * 333` vào chỗ trống thứ nhất và `3 * 334` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mỗi chỗ trống phải là một phép nhân thật giữa mẫu 3 và con số đem thử — chép sẵn kết quả vào thì máy không nhân hộ bạn lần nào
  requireAst:
  # `min: 3` vì có ba phép thử: 8×125 đã viết sẵn, cộng hai phép của bạn.
  # Khung khởi đầu chỉ có một dấu nhân, nên luật này chặn được đáp án gõ cứng.
  - kind: uses-operator, target: *, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^1000\n999\n1002\s*$
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
