---
id: toan.logic-va-chung-minh.vi-sao-vong-chac-chan-dung
title: Vì sao vòng chắc chắn dừng
summary: Bất biến nói vòng luôn đúng, nó không nói vòng có xong — muốn biết vòng dừng thì phải chỉ ra một thước đo nguyên, không âm, giảm ít nhất 1 sau mỗi lượt.
locale: vi
track: toan
module: logic-va-chung-minh
order: 31
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.termination-measure]
requires: [logic.loop-invariant, logic.induction, logic.direct-proof, logic.counterexample, logic.negation, logic.not, ctrl.while, ctrl.while-check-timing, ctrl.loop-progress, ctrl.comparison, core.accumulator, core.read-modify-write, core.variable, core.reassign, core.list, core.list-index, core.len, core.builtin-function, core.boolean, core.number-literal, core.arithmetic, core.function-def, core.function-call, core.function-parameter, core.function-return, core.return-multiple, core.print-variable]
concepts: [logic.thuoc-do-dung, logic.giam-dan-tren-so-tu-nhien, logic.bat-bien-khong-hua-dung]
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
Bất biến của mình đúng ở mọi lượt. Kể cả ở một vòng không bao giờ hết lượt.
::::

::::explain{#cho-bat-bien-khong-voi-toi}
Bài trước để lại một câu hỏi, và nó khó chịu đúng kiểu một câu hỏi hay.

Bạn có bất biến của vòng cộng quỹ: **trước mỗi lượt, `tong` bằng tổng của `i`
khoản đầu tiên trong sổ, và `i` nằm giữa 0 với số tháng đã ghi.** Bạn chứng
minh nó bằng quy nạp theo số lượt: đúng trước lượt đầu, và hễ đúng trước một
lượt thì đúng sau lượt đó.

Đọc kỹ câu ấy xem nó hứa gì. Nó hứa: *nếu* vòng chạy tới lượt thứ n thì trước
lượt ấy điều kia đúng. Nó không hứa có lượt cuối cùng nào cả.

Nghe như bắt bẻ chữ nghĩa. Nó không phải bắt bẻ chữ nghĩa. Có những vòng lặp
giữ bất biến hoàn hảo suốt đời mà chẳng bao giờ trả về cái gì cho ai.
::::

::::example{#vong-rut-quy-khong-bao-gio-xong}
Quỹ CLB cờ vua lớp 6A: sổ ghi sáu tháng, mỗi tháng một khoản.

| tháng | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|
| thu được (đồng) | 40 000 | 25 000 | 60 000 | 15 000 | 30 000 | 45 000 |

Cộng dồn sáu khoản ấy lại thì được **tổng quỹ sáu tháng là 215 000 đồng**. Lát
nữa bạn sẽ thấy con số đó hiện ra từng bước một, cùng với thứ mà cả bài hôm nay
đi tìm.

Giờ thầy phụ trách giao Byte một việc khác: *hễ quỹ còn tiền thì rút ra mua
phấn*. Byte viết khung vòng lặp trước, còn số tiền mỗi lần rút thì để tạm 0 —
định bụng lát nữa hỏi thầy mua bao nhiêu rồi sửa sau:

```python title=readonly
con_trong_quy = 215000
da_rut = 0

while con_trong_quy > 0:
    rut = 0
    con_trong_quy = con_trong_quy - rut
    da_rut = da_rut + rut
    print("Rút thêm một lần")
```

Đặt bất biến cho nó: **`da_rut` cộng `con_trong_quy` luôn bằng 215 000.**

Kiểm hai việc của quy nạp, đúng như bài 30:

- **Trước lượt đầu.** `da_rut` là 0, `con_trong_quy` là 215 000, cộng lại đúng
  215 000. Đúng.
- **Một lượt bất kỳ.** Lượt này lấy `rut` ra khỏi quỹ rồi cộng đúng `rut` ấy
  vào `da_rut`. Một bên bớt bao nhiêu thì bên kia thêm bấy nhiêu, nên tổng hai
  bên không đổi. Vào lượt đúng thì ra lượt cũng đúng.

Bất biến này **đúng**, và nó đúng mãi mãi. Vấn đề là "mãi mãi" ở đây theo nghĩa
đen: mỗi lượt Byte rút 0 đồng, nên `con_trong_quy` không nhúc nhích, nên
`215000 > 0` vẫn là `True` ở mọi lượt. Màn hình in "Rút thêm một lần" cho tới
lúc bạn tắt máy.

Đây đúng cái vòng lặp vô hạn của T1.2.10, chỉ khác một chỗ: lần này nó có hẳn
một bất biến được chứng minh tử tế. Nên kết luận phải nói thẳng ra:

> Bất biến nói về **những lượt có xảy ra**. Nó không nói có bao nhiêu lượt, và
> nó không nói có lượt cuối. Muốn biết vòng dừng thì phải chứng minh riêng.
::::

::::explain{#thuoc-do-dung}
Vậy chứng minh riêng ấy bám vào đâu?

T1.2.10 đã dặn một câu: mỗi `while` cần một **bước tiến** — một dòng trong thân
đẩy cái tên trong điều kiện về phía làm điều kiện sai. Lời dặn ấy đúng và có
ích, nhưng nó là lời dặn, chưa phải bằng chứng: "đẩy về phía đó" thì đẩy bao
lâu mới tới nơi? Vòng rút quỹ ở trên cũng có một dòng động vào
`con_trong_quy` đấy thôi.

Chỗ biến lời dặn thành bằng chứng nằm ở một thứ mà bài 26 đã dựng sẵn: **số tự
nhiên không giảm mãi được.**

Đó là quy nạp nhìn ngược lại. Bậc thang của bài 26 xếp lên trên và không có
đỉnh, nên leo bao nhiêu bậc cũng được. Nhưng đi **xuống** thì có đáy: dưới 0
không còn số tự nhiên nào để đứng.

Đặt tên cho thứ mình cần:

> **Thước đo dừng** của một vòng lặp là một đại lượng thoả cả ba điều:
>
> 1. nó là một **số nguyên**;
> 2. nó **không âm** ở mọi lúc vòng còn chạy;
> 3. mỗi lượt thân vòng làm nó **nhỏ đi**.
>
> Chỉ ra được một thước đo như thế là đã chứng minh vòng dừng.

Vì sao ba điều ấy đủ:

- Điều 1 gánh một việc lặng lẽ mà không ai thay được: giữa hai số nguyên liền
  nhau không có số nào chen vào, nên với số nguyên, "nhỏ đi" nghĩa là **nhỏ đi
  ít nhất 1**.
- Gọi `d` là giá trị của thước đo ngay trước lượt đầu. Sau một lượt nó còn
  nhiều nhất `d − 1`; sau hai lượt còn nhiều nhất `d − 2`; sau `d` lượt còn
  nhiều nhất 0.
- Nếu vòng còn chạy thêm một lượt nữa, lượt ấy kéo nó xuống dưới 0. Mà điều 2
  cấm chuyện đó.
- Nên vòng chạy **nhiều nhất `d` lượt**. Một con số hữu hạn, biết trước, đếm
  được.

Và thiếu điều nào cũng hỏng, mỗi điều hỏng một kiểu khác nhau:

- Bỏ điều 1: lấy thước đo là 1, rồi một nửa, rồi một phần tư, rồi một phần
  tám… Lượt nào nó cũng nhỏ đi thật, và nó không âm lúc nào cả — mà chia đôi
  mãi thì không bao giờ tới 0. Vòng chạy mãi.
- Bỏ điều 2: một số nguyên cứ nhỏ đi 1 mỗi lượt mà không có đáy nào chặn thì đi
  xuống mãi, chẳng chỗ nào bắt nó dừng lại.
- Bỏ điều 3: đó đúng là vòng rút quỹ phía trên — `con_trong_quy` là số nguyên,
  không âm, nhưng nó không nhỏ đi.

Để ý điều 3 chỉ đòi "nhỏ đi", không đòi "nhỏ đi đúng 1". Một thước đo tụt 3 mỗi
lượt thì càng chóng hết, lập luận trên không đổi một chữ nào.

Bây giờ quay lại vòng cộng quỹ. Điều kiện lặp là `i < 6`. Cái đẩy nó tới chỗ
sai là chuyện `i` lớn dần, mà thước đo phải **giảm** — nên lấy khoảng cách còn
lại: **`6 − i`**, số tháng chưa đọc.

| trước lượt | `i` | thước đo `6 − i` | `tong` |
|---|---|---|---|
| lượt 1 | 0 | 6 | 0 |
| lượt 2 | 1 | 5 | 40 000 |
| lượt 3 | 2 | 4 | 65 000 |
| lượt 4 | 3 | 3 | 125 000 |
| lượt 5 | 4 | 2 | 140 000 |
| lượt 6 | 5 | 1 | 170 000 |
| không còn lượt nào | 6 | 0 | 215 000 |

Ba điều đều đạt: `6 − i` là số nguyên; nó không âm chừng nào `i` chưa vượt 6;
và mỗi lượt có đúng một dòng `i = i + 1` nên nó tụt đúng 1. Thước đo xuất phát
từ 6, nên vòng chạy nhiều nhất 6 lượt.

Đó là bằng chứng cho lời dặn của T1.2.10, chứ không phải một lời dặn mới.
::::

::::predict{#doan-luc-thoat commitOnce}
Byte cho vòng cộng quỹ chạy hết, rồi hỏi ba câu về đúng lúc nó vừa thoát ra.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]
i = 0
tong = 0

while i < len(so_quy):
    tong = tong + so_quy[i]
    i = i + 1

print(i)
print(len(so_quy) - i)
print(tong)
```

:::opt{correct}
`6`, rồi `0`, rồi `215000`
:::

:::opt
`5`, rồi `1`, rồi `170000`
::why
Gần đúng ở chỗ bạn nhớ một sự thật thật: ô cuối cùng của một sổ sáu tháng mang
chỉ số **5**, không phải 6. Đếm từ 0 thì `so_quy[5]` đúng là khoản tháng cuối,
và bạn không nhầm chỗ đó.

Chỗ lệch nằm ở lúc `i` bằng 5. Khi ấy vòng chưa xong việc: `5 < 6` vẫn là
`True`, nên máy vào thân thêm một lượt nữa — lượt ấy cộng khoản tháng 6 vào
`tong` rồi đẩy `i` lên 6. Chỉ tới lúc `i` bằng 6 thì `6 < 6` mới sai và vòng
mới thoát. Nên `i` **dừng ở một bậc cao hơn chỉ số cuối cùng đúng một bậc**, và
thước đo `6 − i` vừa vặn chạm 0.
::
:::

:::opt
`6`, rồi `0`, rồi `170000`
::why
Gần đúng ở chỗ hai dòng đầu bạn đọc trúng hoàn toàn: `i` dừng ở 6, thước đo
chạm 0, đúng như bảng vừa dựng.

Chỗ lệch nằm ở thứ tự hai dòng trong thân vòng. Ở lượt cuối, `tong = tong +
so_quy[i]` chạy **trước** rồi `i = i + 1` mới chạy — nên khoản 45 000 của tháng
6 đã kịp vào `tong` rồi `i` mới nhích lên 6.

Con số 170 000 đúng là tổng khi thiếu khoản 45 000 của tháng cuối. Nhưng muốn
thiếu nó thì vòng phải **dừng sớm một lượt** — chẳng hạn điều kiện viết nhầm
thành `i < 5`. Đổi chỗ hai dòng thì không ra 170 000: đẩy `i` lên trước làm mất
khoản tháng ĐẦU, và tới lượt sáu máy đi đọc `so_quy[6]` rồi nổ `IndexError`.
Đây là chỗ đáng dừng lại:
bất biến "trước mỗi lượt `tong` bằng tổng `i` khoản đầu" nói đúng chuyện ấy —
`i` và `tong` luôn khớp nhau, không cái nào chạy trước cái nào một tháng.
::
:::

:::opt
`7`, rồi `-1`, rồi `215000`
::why
Gần đúng ở chỗ bạn cẩn thận với chuyện máy phải **đọc lại điều kiện thêm một
lần nữa** mới biết đường thoát — điều đó có thật, và nhiều người quên hẳn lần
đọc cuối cùng ấy.

Chỗ lệch: đọc lại điều kiện không phải là chạy thêm một lượt. T1.2.9 đã chốt
rằng `while` xem điều kiện **trước** thân, nên lần đọc cuối cùng thấy `6 < 6`
là sai và máy đi thẳng xuống dưới vòng, không đụng vào `i` nữa. Đó cũng chính
là lý do thước đo dừng lại đúng ở 0 chứ không tụt xuống âm: thân vòng — chỗ
duy nhất làm nó giảm — không được chạy thêm lần nào.
::
:::
::::

::::code{#do-thuoc-do-tung-luot}
Giờ viết thước đo ra thành mã, và bắt máy **kiểm** nó ở từng lượt.

Hàm `cong_quy` nhận một cuốn sổ (danh sách các khoản), cộng dồn, và trả về hai
thứ: tổng, cùng **số lượt đã chạy**. Hai câu `assert` trong thân đã viết sẵn —
chúng canh đúng hai trong ba điều của thước đo: không âm, và nhỏ đi. Điều "nhỏ
đi" viết thành `sau <= truoc - 1` chứ không viết `sau < truoc`, và hai cách viết
ấy nói cùng một chuyện **vì đây là số nguyên** — đúng chỗ điều 1 làm việc.

Hai chỗ trống, cả hai đều là **thước đo tại một thời điểm**:

1. `truoc = ___` — thước đo ngay **trước** lượt đầu tiên.
2. `sau = ___` — thước đo ngay **sau** khi `i` vừa nhích lên trong lượt này.

Chú ý: cuốn sổ đưa vào không phải lúc nào cũng sáu tháng. Bài chấm bằng **năm
cuốn sổ dài ngắn khác nhau**, trong đó có một cuốn rỗng và một cuốn chỉ ghi một
tháng — nên thước đo phải viết theo chính cuốn sổ đang cầm, không viết theo một
con số nhớ sẵn.

```python title=starter
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]


def cong_quy(so):
    i = 0
    tong = 0
    so_luot = 0
    truoc = ___
    while i < len(so):
        tong = tong + so[i]
        i = i + 1
        sau = ___
        assert sau >= 0, "thước đo vừa tụt xuống dưới 0 — nó phải là số nguyên KHÔNG ÂM ở mọi lúc vòng còn chạy"
        assert sau <= truoc - 1, "sau lượt này thước đo không giảm nổi 1 — một vòng như thế có thể chạy mãi"
        truoc = sau
        so_luot = so_luot + 1
    return tong, so_luot


tong_quy, luot_quy = cong_quy(so_quy)
print(tong_quy, luot_quy)
tong_rong, luot_rong = cong_quy([])
print(tong_rong, luot_rong)
```

```python title=solution
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]


def cong_quy(so):
    i = 0
    tong = 0
    so_luot = 0
    truoc = len(so) - i
    while i < len(so):
        tong = tong + so[i]
        i = i + 1
        sau = len(so) - i
        assert sau >= 0, "thước đo vừa tụt xuống dưới 0 — nó phải là số nguyên KHÔNG ÂM ở mọi lúc vòng còn chạy"
        assert sau <= truoc - 1, "sau lượt này thước đo không giảm nổi 1 — một vòng như thế có thể chạy mãi"
        truoc = sau
        so_luot = so_luot + 1
    return tong, so_luot


tong_quy, luot_quy = cong_quy(so_quy)
print(tong_quy, luot_quy)
tong_rong, luot_rong = cong_quy([])
print(tong_rong, luot_rong)
```

```python title=test
# Ba câu về SỐ LƯỢT đứng trước. Chúng canh đúng cái bẫy của bài: một thước đo
# viết bừa vẫn cho ra tổng tiền đúng, vì tổng tiền do dòng cộng dồn quyết định
# chứ không do thước đo. Chỉ số lượt mới nói được thước đo có thật sự đo cuốn
# sổ đang cầm hay không.
assert cong_quy([])[1] == 0, "sổ rỗng: thước đo đã bằng 0 ngay trước lượt đầu, nên vòng không được chạy lượt nào"
assert cong_quy([50000])[1] == 1, "sổ ghi đúng một tháng: thước đo đi từ 1 xuống 0, vừa đủ một lượt"
assert cong_quy([1, 2, 3])[1] == 3, "sổ ba khoản thì ba lượt — số lượt do ĐỘ DÀI sổ quyết định, không do con số ghi trong sổ"
assert cong_quy(so_quy) == (215000, 6), "sổ quỹ sáu tháng: cộng ra 215000 đồng qua đúng 6 lượt"
assert cong_quy([1, 2, 3])[0] == 6, "1 + 2 + 3 = 6 — thước đo không được đụng vào số tiền đang cộng dồn"
assert cong_quy([0, 0])[1] == 2, "hai tháng chẳng thu được đồng nào vẫn là hai lượt: thước đo đếm THÁNG chứ không đếm tiền"
```

:::hints
- kind: attention
  body: Hai chỗ trống hỏi cùng một câu, chỉ khác thời điểm — dòng `truoc =` hỏi lúc chưa chạy lượt nào, dòng `sau =` hỏi ngay sau khi `i` vừa nhích. Đọc lại bảng thước đo trong phần giải thích: cột `6 − i` được dựng từ hai thứ, số tháng CÓ TRONG SỔ và số tháng ĐÃ ĐỌC. Trong hàm này, số tháng đã đọc chính là `i`; còn số tháng có trong sổ thì cuốn sổ tự nói ra được.
- kind: strategy
  body: "Thước đo là phần còn lại: lấy độ dài cuốn sổ trừ đi số tháng đã đọc. Đừng gõ một con số cố định — cuốn sổ đưa vào có thể rỗng, có thể một tháng, có thể ba; một con số nhớ sẵn thì đúng với đúng một cuốn và sai với bốn cuốn còn lại. Cùng một câu ấy viết được cho cả hai chỗ trống, vì hai chỗ chỉ khác nhau ở giá trị mà `i` đang mang lúc chạy tới."
- kind: one-line
  body: "Cả hai chỗ trống đều là `len(so) - i`."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: thước đo phải dựng từ ĐỘ DÀI cuốn sổ đang cầm trừ đi `i` — gõ một con số cố định hay một biểu thức không nhắc tới cuốn sổ thì bài đang chấm một thứ nó không hề đo
  requireAst:
  # Khung khởi đầu gọi `len` đúng 1 lần (ở điều kiện `while`); lời giải gọi 3.
  # Luật này một mình chặn mọi đáp án gõ cứng: `truoc = 6`, `truoc = 0`,
  # `truoc = True` đều dừng ở 1.
  - kind: uses-call, target: len, min: 3
  # Thước đo phải nhắc tới chính cuốn sổ. Khung khởi đầu đọc `so` 2 lần
  # (`len(so)` và `so[i]`); lời giải đọc 4.
  - kind: uses-name, target: so, min: 4
  # …và phải nhắc tới số tháng ĐÃ đọc. Khung khởi đầu đọc `i` 3 lần.
  #
  # `min: 4`, không phải 5. Chỗ trống thứ nhất nằm TRƯỚC vòng, lúc `i` còn
  # đúng bằng 0 — nên `truoc = len(so)` là một lời giải ĐÚNG, dựng từ chính
  # cuốn sổ đang cầm, không phải "một con số nhớ sẵn". Đặt 5 là đo theo hình
  # dạng lời giải mẫu và đánh trượt nó oan, với một câu `onFail` còn đổ tội
  # nhầm ("không nhắc tới cuốn sổ") trong khi họ vừa gõ `len(so)`.
  #
  # 4 vẫn chặn đúng ca luật này sinh ra để chặn — `len(so)` ở CẢ HAI chỗ (chỉ
  # đọc `i` 3 lần). Ca ấy tầng `run` cũng bắt được, nhưng `truoc = 6` thì run
  # KHÔNG bắt (đã thử: nó chạy êm trên cả năm cuốn sổ), nên luật này vẫn cần.
  - kind: uses-name, target: i, min: 4
  # Thước đo là một phép TRỪ. Khung khởi đầu có đúng 1 dấu trừ (trong câu
  # `assert sau <= truoc - 1`); lời giải có 3.
  - kind: uses-operator, target: -, min: 3
  forbidAst:
  # Lưới thứ hai. `6 - i` chạy đúng với cuốn sổ sáu tháng và cũng là một thước
  # đo hợp lệ cho nó — nhưng nó đo một cuốn sổ tưởng tượng, không đo cuốn đang
  # cầm. Mọi cách viết hợp lệ dựng từ `len(so)` và `i`, không cách nào chứa
  # nguyên văn con số 6.
  - kind: has-literal, target: 6
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: ^215000 6\n0 0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu, năm, bốn, ba, hai, một, không. Đếm ngược thì kiểu gì cũng tới nơi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đang cầm hai mảnh, và cả hai đều đã chứng minh xong:

- **Bất biến** — trước mỗi lượt, `tong` bằng tổng `i` khoản đầu tiên của sổ, và
  `i` không vượt quá số tháng đã ghi.
- **Thước đo dừng** — `6 − i` là số nguyên, không âm, giảm 1 mỗi lượt, nên vòng
  chạy nhiều nhất 6 lượt rồi thoát.

Vòng dừng rồi, và bất biến vẫn đúng. Ghép hai điều đó lại đã đủ để nói `tong`
bằng đúng tổng quỹ sáu tháng — 215 000 đồng — chưa, hay còn thiếu một mẩu tin
nữa, mẩu mà **chỉ lúc thoát** mới có?

Thử tự trả lời trước: bất biến nói `tong` bằng tổng `i` khoản đầu. Muốn đọc ra
"tổng cả sáu khoản" thì phải biết `i` **đúng bằng 6** lúc ấy. Bất biến một mình
chỉ nói `i` không vượt quá 6. Vậy ai nói cho bạn biết `i` không dừng ở 4?

Bài sau là bài cuối của cả track, và nó ghép ba mảnh lại thành một bằng chứng.
::::

::::checkpoint{mastery=0.8}
::::
