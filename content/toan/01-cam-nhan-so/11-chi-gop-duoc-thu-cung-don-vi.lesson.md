---
id: toan.cam-nhan-so.chi-gop-duoc-thu-cung-don-vi
title: Chỉ gộp được thứ cùng đơn vị
summary: Phép cộng chỉ có nghĩa khi hai lượng đo bằng cùng một thước — và đó mới là lý do người ta viết thẳng cột, chứ không phải cho đẹp mắt.
locale: vi
track: toan
module: cam-nhan-so
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.like-units]
requires: [math.addition-as-union, math.don-vi, math.dong-goi, core.arithmetic, core.variable, core.print-variable, core.string-concat, err.type-error]
concepts: [math.don-vi, math.thang-cot, math.doi-thuoc]
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
Máy trả lời mình số 5 rất nhanh. Nhưng 5 cái gì thì máy không nói.
::::

::::explain{#nam-cai-gi}
Bài trước dừng lại ở một chỗ khó chịu: 3 hạt lẻ, 2 cái bó, gõ `3 + 2` thì ra
`5` — mà 5 **cái gì** thì không ai trả lời được.

Câu trả lời thẳng thắn: **không trả lời được, vì phép cộng ấy không có nghĩa.**

Nhớ lại hai bài cũ ghép vào nhau thì thấy ngay vì sao:

- Bài 1: một con số luôn là câu trả lời cho "mấy **cái** gì". Bỏ cái đơn vị đi
  thì nó chưa nói được điều gì.
- Bài 10: cộng là **gộp** — đổ hai lượng vào làm một rồi đo cái lượng chung ấy.

Ghép lại: muốn đo cái lượng chung thì phải **đo bằng một cái thước** (bài 3).
Nếu đống này đo bằng thước "một hạt" còn đống kia đo bằng thước "một bó", thì
con số đi ra đang đếm theo thước nào?

```text
3 hạt lẻ:   o  o  o

2 bó:       [==========]  [==========]

Đổ chung rồi đếm "vật":   o  o  o  [==========]  [==========]
                          1  2  3       4              5
```

Ra `5` thật. Nhưng năm cái vật ấy không bằng nhau, nên con số 5 không đo được
lượng hạt nào cả. Nó chỉ đếm số **món đồ nằm trên bàn** — mà số món đồ thì
thay đổi ngay khi Byte cởi một cái bó ra, dù không hạt nào rời khỏi bàn.

Một con số dùng được thì không được phép nhảy khi ta chưa động gì tới lượng.
Nên đây là luật của cả track, và nó có tên:

> **Luật cùng đơn vị.** Chỉ gộp được hai lượng khi chúng đo bằng **cùng một
> thước**. Khác thước thì phải đổi về một thước trước, rồi mới gộp.
::::

::::explain{#thang-cot-la-luat-nay-viet-ra}
Có hai đường ra khỏi chỗ tắc, và cả hai đều là luật trên:

**Cách 1 — giữ hai thước riêng, gộp từng thước một.** Bó gộp với bó, hạt lẻ
gộp với hạt lẻ. Không bao giờ trộn hai hàng vào nhau:

```text
        bó    hạt lẻ
Byte     4       3
An       2       5
       ----    ------
         6       8       →  6 bó và 8 hạt lẻ
```

**Cách 2 — đổi hết về một thước rồi mới gộp.** Cởi bó ra thành hạt: 4 bó 3 hạt
là 43 hạt, 2 bó 5 hạt là 25 hạt. Bây giờ hai lượng cùng thước "một hạt", gộp
thoải mái.

Và bây giờ tới chỗ đáng tiền của bài này. Bài 7 nói **chỗ đứng nói giá trị** —
cột bên phải là hạt lẻ, cột kế bên là bó. Nên khi bạn viết hai con số thẳng cột
với nhau:

```text
   4 3
 + 2 5
 -----
   6 8
```

thì cái bạn vừa làm **không phải** là xếp cho ngay ngắn. Bạn đang đặt hạt lẻ
đúng trên hạt lẻ và bó đúng trên bó — tức là bạn đang cưỡng chế luật cùng đơn
vị bằng cách bố trí trên giấy. Thẳng cột chính là luật này viết ra thành hình.

Đó cũng là lý do một dãy chữ số lệch cột thì hỏng hẳn chứ không chỉ xấu: lệch
một cột nghĩa là bạn đang gộp bó với hạt lẻ.
::::

::::example{#gop-tung-cot-mot}
Cho máy làm đúng cách 1 — hai cột, hai phép gộp riêng, không trộn:

```python title=readonly
byte_bo = 4
byte_hat = 3
an_bo = 2
an_hat = 5

print(byte_bo + an_bo)
print(byte_hat + an_hat)
```

Máy in ra:

```text
6
8
```

Sáu bó và tám hạt lẻ. Để ý là máy phải chạy **hai** phép cộng chứ không phải
một: hai đơn vị thì hai lần gộp, và không có lần nào nhìn sang lần kia.
::::

::::predict{#doan-may-co-can-khong commitOnce}
Bây giờ cố tình phá luật, xem máy có chặn không. Hai cái tên dưới đây đo bằng
hai thước khác nhau — một cái đếm bó, một cái đếm hạt lẻ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
bo = 3
hat_le = 2
print(bo + hat_le)
```

:::opt{correct}
5
:::

:::opt
Một thông báo TypeError
::why
Gần đúng ở chỗ bạn nhớ đúng một việc Python thật sự làm: khi hai bên của dấu
`+` không đi cùng nhau được, máy dừng hẳn lại và nói ra thay vì đoán bừa. Realm
0 đã cho bạn thấy nó làm thế với một câu chữ cộng một con số.

Chỗ lệch nằm ở việc máy nhìn thấy cái gì. Nó nhìn **kiểu** — chữ hay số — chứ
không nhìn **đơn vị**. Ở đây cả `3` lẫn `2` đều là số nguyên, hai bên hợp nhau
hoàn hảo dưới mắt máy. Chữ "bó" và "hạt lẻ" chỉ nằm trong tên biến và trong
đầu bạn; máy không đọc được ý nghĩa ấy. Nên luật cùng đơn vị là việc của bạn,
không phải việc của máy — và đó chính là lý do bài này tồn tại.
::
:::

:::opt
32
::why
Gần đúng ở chỗ bạn làm đúng thứ bài này đang dạy: đổi về một thước trước đã. 3
bó là 30 hạt, thêm 2 hạt lẻ nữa là 32 hạt — phép toán ấy không sai chữ số nào,
và nó chính là cách 2 ở phần trên.

Chỗ lệch là câu hỏi đang hỏi cái khác: **máy in ra gì**, chứ không phải lượng
hạt thật là bao nhiêu. Muốn máy ra 32 thì bạn phải tự viết phép đổi thước ra
cho nó thấy. Chừng nào bạn chưa viết, máy chỉ có hai con số trần `3` và `2`.
::
:::

:::opt
5 bó
::why
Gần đúng ở chỗ bạn thấy con số trần chưa đủ và tự động tìm cái đơn vị đi kèm —
đúng thói quen mà bài 1 dựng lên. Và cách chọn "lấy đơn vị của số đứng trước"
là một luật thật sự dùng được, miễn hai bên **cùng** đơn vị: 4 bó gộp 2 bó thì
đúng là 6 bó.

Chỗ lệch là ở đây hai bên **khác** đơn vị, nên không có đơn vị nào để lấy. Còn
về phía máy: máy chưa bao giờ in kèm đơn vị, vì nó không giữ đơn vị nào cả. Nó
in đúng một con số trần.
::
:::
::::

::::explain{#don-vi-nam-trong-dau-ban}
Rút ra hai câu, và câu thứ hai mới là câu khó:

1. Cộng hai lượng khác đơn vị thì con số đi ra vẫn đếm được một thứ — số
   **món đồ trên bàn** — nhưng nó không đo **lượng** nào cả, vì nó nhảy ngay
   khi ta cởi một cái bó ra mà chưa hạt nào rời khỏi bàn.
2. **Máy không hề biết chuyện đó.** Nó cộng hai con số trần và trả về một con
   số trần, không một lời cảnh báo.

Ở Realm 0 bạn quen với một cái máy hay dừng lại hỏi. Lần này nó không hỏi, vì
lỗi không nằm trong máy: hai con số ấy cộng được, chỉ có **ý nghĩa** của kết
quả là không tồn tại. Máy giữ kiểu, còn đơn vị thì bạn giữ.

Nên từ đây trở đi, mỗi lần viết một dấu `+`, có đúng một câu phải tự hỏi trước:
*hai bên có đang đo bằng cùng một thước không?*
::::

::::code{#gop-dung-cot}
Byte và An dồn hạt vào chung một sọt để mai đem gieo.

- **Byte**: 4 bó và 3 hạt lẻ.
- **An**: 2 bó và 5 hạt lẻ.

Mỗi bó đúng mười hạt (bài 6). Hai chỗ trống đầu là **cách 1**: một cột bó, một
cột hạt lẻ. Luật cùng đơn vị nói mỗi chỗ trống chỉ được gộp hai cái tên **cùng
một cột** — trộn cột là câu hỏi mất nghĩa.

Bài chấm bằng cả hai cột, và hai cột cho ra hai con số khác nhau (6 và 8). Gõ
cứng một con số vào cả hai chỗ thì hỏng một cột; gộp lệch cột thì con số cũng
lệch ngay — 4 bó gộp với 5 hạt lẻ ra `9`, không phải `6`.

Chỗ trống thứ ba là **cách 2**, đường mà phần trên mới nói tới chứ chưa bắt bạn
đi: cởi hết bó ra thành hạt rồi mới gộp. Bài 9 đã viết đúng phép đổi thước ấy
một lần — số bó nhân với cỡ bó, rồi cộng hạt lẻ. Ở đây cỡ bó là mười, và bạn
phải đổi cho **cả hai người** trước khi gộp.

Hai cách phải rơi trúng cùng một đống hạt. Câu chấm cuối đặt chúng cạnh nhau,
nên lệch một cột là lệch luôn cả câu ấy.

```python title=starter
# Mỗi bó đúng mười hạt.
byte_bo = 4
byte_hat = 3
an_bo = 2
an_hat = 5

# Cách 1: giữ hai cột. Bó chỉ gộp được với bó, hạt lẻ chỉ gộp với hạt lẻ.
bo_chung = ___
hat_chung = ___

# Cách 2: cởi hết bó ra thành hạt rồi mới gộp — cả hai người, cùng thước "một
# hạt". Dùng lại bốn cái tên phía trên, đừng gõ thẳng 43 hay 25 vào đây.
tong_hat = ___

print(bo_chung)
print(hat_chung)
print(tong_hat)
```

```python title=solution
# Mỗi bó đúng mười hạt.
byte_bo = 4
byte_hat = 3
an_bo = 2
an_hat = 5

# Cách 1: giữ hai cột. Bó chỉ gộp được với bó, hạt lẻ chỉ gộp với hạt lẻ.
bo_chung = byte_bo + an_bo
hat_chung = byte_hat + an_hat

# Cách 2: cởi hết bó ra thành hạt rồi mới gộp — cả hai người, cùng thước "một
# hạt". Dùng lại bốn cái tên phía trên, đừng gõ thẳng 43 hay 25 vào đây.
tong_hat = byte_bo * 10 + byte_hat + an_bo * 10 + an_hat

print(bo_chung)
print(hat_chung)
print(tong_hat)
```

```python title=test
assert bo_chung == 6, "4 bó gộp với 2 bó thì được 6 bó"
assert hat_chung == 8, "3 hạt lẻ gộp với 5 hạt lẻ thì được 8 hạt lẻ"

# Hai câu dưới là chỗ luật cùng đơn vị được kiểm thật: gộp lệch cột cho ra
# những con số KHÁC hẳn (4 + 5 = 9 và 3 + 2 = 5), nên chúng phân biệt được
# một lời giải gộp đúng cột với một lời giải cộng bừa hai cái tên.
assert bo_chung != byte_bo + an_hat, "bó không gộp được với hạt lẻ"
assert hat_chung != byte_hat + an_bo, "hạt lẻ không gộp được với bó"

assert tong_hat == 68, "43 hạt gộp với 25 hạt thì được 68 hạt"

# Câu chốt nối hai cách lại. Vế trái do bạn viết theo cách 2; vế phải dựng lại
# từ hai cột của cách 1 — hai đường khác nhau đi tới cùng một đống hạt, nên
# lệch một cột là lệch luôn cả câu này.
assert tong_hat == bo_chung * 10 + hat_chung, "gộp từng cột rồi cởi bó, hay cởi bó rồi gộp — vẫn cùng một đống"
```

:::hints
- kind: attention
  body: Bốn cái tên phía trên chia làm hai cặp, không phải bốn thứ rời. Đọc kỹ đuôi mỗi tên — cái nào đếm bó, cái nào đếm hạt lẻ. Chỗ trống thứ ba cần cả bốn cái tên ấy, vì nó đổi cả hai đống về cùng một thước.
- kind: strategy
  body: Hai chỗ trống đầu, mỗi chỗ là một phép gộp của bài 10 nhưng chỉ được lấy hai cái tên trong cùng một cột. Chỗ trống thứ ba đi đường khác, đúng lối bài 9 đã làm: số bó nhân với mười cho ra số hạt đang nằm trong bó, cộng thêm hạt lẻ là ra cả đống của một người — làm thế cho từng người rồi gộp hai kết quả.
- kind: one-line
  body: "Thay `___` thứ nhất bằng `byte_bo + an_bo`, `___` thứ hai bằng `byte_hat + an_hat`, và `___` thứ ba bằng `byte_bo * 10 + byte_hat + an_bo * 10 + an_hat`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: hai chỗ trống đầu phải gộp hai cái tên CÙNG một cột bằng dấu `+`, còn chỗ trống thứ ba phải đổi cả hai đống về thước "một hạt" rồi mới gộp — chép sẵn con số kết quả thì bài không kiểm được gì
  requireAst:
  # Hai cột thì hai phép gộp, cộng thêm ba dấu `+` của cách 2. Khung khởi đầu
  # chưa có dấu `+` nào, nên luật này chặn đúng đáp án chép cứng `6`, `8`, `68`.
  - kind: uses-operator, target: +, min: 5
  # Cách 2 phải tự viết ra phép đổi thước, không mượn lại `bo_chung`.
  - kind: uses-operator, target: *, min: 2
  # `min: 2` cho cả bốn cái tên: mỗi tên xuất hiện một lần ở cách 1 và một lần
  # ở cách 2. Thiếu luật này thì `bo_chung = an_bo + 4` — chép cứng đúng một
  # nửa mỗi phép gộp — vẫn qua sạch mọi tầng chấm.
  - kind: uses-name, target: byte_bo, min: 2
  - kind: uses-name, target: an_bo, min: 2
  - kind: uses-name, target: byte_hat, min: 2
  - kind: uses-name, target: an_hat, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^6\n8\n68\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu bó, tám hạt lẻ. Mỗi con số nói rõ nó đếm cái gì.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hôm nay hai cột đều vừa vặn: cột hạt lẻ gộp 3 với 5 ra 8, vẫn còn chỗ.

Ngày mai thì khác. Byte có 7 hạt lẻ, An mang tới 5 hạt lẻ. Luật cùng đơn vị
vẫn chạy trơn tru — hạt lẻ gộp với hạt lẻ — và ra **12 hạt lẻ**.

Nhưng cột hạt lẻ trong bảng vị trí (bài 7) chỉ có đúng một chỗ, và một chỗ thì
chỉ chứa nổi tới `9`. Số 12 không nhét vào đó được.

Vậy cái hạt thứ mười đi đâu? Nó biến mất à — hay nó vẫn nằm trên bàn, chỉ là
không còn chỗ để viết? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
