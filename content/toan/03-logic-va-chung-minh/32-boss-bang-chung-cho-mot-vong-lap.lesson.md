---
id: toan.logic-va-chung-minh.boss-bang-chung-cho-mot-vong-lap
title: BOSS — Bằng chứng cho một vòng lặp
summary: Ba mẩu ghép lại thành một bằng chứng trọn vẹn cho vòng cộng quỹ: bất biến giữ suốt vòng, thước đo bắt vòng phải dừng, và điều kiện lặp SAI lúc thoát.
locale: vi
track: toan
module: logic-va-chung-minh
order: 32
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [logic.loop-invariant, logic.termination-measure, logic.induction, logic.direct-proof, logic.finite-check-not-proof, logic.negation, logic.implication, logic.vacuous-truth, logic.counterexample]
requires: [logic.termination-measure, logic.loop-invariant, logic.induction, logic.direct-proof, logic.implication, logic.negation, logic.not, logic.vacuous-truth, logic.finite-check-not-proof, logic.counterexample, ctrl.while, ctrl.while-check-timing, ctrl.loop-progress, ctrl.for-range, ctrl.comparison, core.accumulator, core.read-modify-write, core.variable, core.reassign, core.list, core.list-index, core.len, core.builtin-function, core.boolean, core.number-literal, core.arithmetic, core.function-def, core.function-call, core.function-parameter, core.function-return, core.print-variable]
concepts: [logic.bang-chung-cho-vong-lap, logic.dieu-kien-lap-sai-luc-thoat, logic.ba-manh-ghep]
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
Vòng dừng rồi. Nhưng nó dừng ở đâu — chỗ đó mình chưa hỏi bao giờ.
::::

::::explain{#manh-tin-chi-co-luc-thoat}
Bài trước để lại đúng một chỗ hở, và hở nhỏ tới mức dễ bỏ qua.

Bạn có hai mảnh, cả hai đều đã chứng minh:

- **Bất biến** — trước mỗi lượt, `tong` bằng tổng của `i` khoản đầu tiên trong
  sổ, và `i` nằm giữa 0 với số tháng đã ghi.
- **Thước đo dừng** — số tháng chưa đọc là số nguyên, không âm, giảm 1 mỗi
  lượt, nên vòng chạy nhiều nhất bằng số tháng đã ghi rồi thoát.

Ghép lại, cái bạn cầm trong tay lúc vòng vừa thoát là: *`tong` bằng tổng `i`
khoản đầu, với một `i` nào đó không vượt quá 6.*

"Một `i` nào đó không vượt quá 6" thì có thể là 4. Mà `tong` bằng tổng 4 khoản
đầu không phải điều Byte cần. Cần `i` **đúng bằng 6**.

Mẩu tin còn thiếu nằm ở chính cái cửa mà vòng vừa đi qua.

> Vòng `while` **không có `break`** thì thoát ra bằng đúng một cách: máy quay
> lên đọc điều kiện, và đọc thấy **sai**. Nên đứng ở dòng ngay dưới một vòng
> như thế, bạn luôn được cầm thêm một mệnh đề miễn phí: **điều kiện lặp là
> sai.**

T1.2.9 đã dạy máy xem điều kiện **trước** thân. Bài này chỉ đọc lại sự thật ấy
theo chiều của người đi chứng minh: cái chỗ đứng dưới vòng không phải chỗ trống
— nó mang sẵn một tin, và tin ấy là **phủ định** của điều kiện lặp.

Chữ "không có `break`" trong khung không phải câu rào đón. T1.2.16 dạy bạn
một lối thoát thứ hai: gặp `break` là máy nhảy thẳng ra ngoài, **không** quay
lên đọc điều kiện lần nào nữa. Đứng dưới một vòng có `break` thì mẩu tin miễn
phí kia biến mất — điều kiện lặp lúc ấy có thể vẫn đang đúng.

Đây là chỗ đáng ghi lại, vì nó là một mẫu chung của cả mạch này: **một luật
suy luận chỉ dùng được trong đúng cái phạm vi nó được phát biểu.** Vòng của
Byte dưới đây không có `break`, nên bạn dùng được. Gặp vòng có `break`, phải
đi tìm mẩu tin khác — thường là chính cái điều kiện bạn viết trong `if` ngay
trước `break`.
::::

::::example{#ba-manh-ghep-lai}
Đặt ba mảnh cạnh nhau trên đúng vòng cộng quỹ. Sổ quỹ CLB cờ vua lớp 6A vẫn là
cuốn sổ của bài trước, không đổi một con số nào:

| tháng | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|
| thu được (đồng) | 40 000 | 25 000 | 60 000 | 15 000 | 30 000 | 45 000 |

Tổng quỹ sáu tháng là 215 000 đồng.

```python title=readonly
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]
i = 0
tong = 0

while i < len(so_quy):
    tong = tong + so_quy[i]
    i = i + 1
```

**Mảnh 1 — bất biến (bài 30).** Gọi `T(k)` là tổng của `k` khoản đầu tiên trong
sổ. Câu cần giữ là: `0 ≤ i ≤ 6` **và** `tong = T(i)`.

- *Cơ sở.* Trước lượt đầu, `i` là 0 và `tong` là 0. Tổng của 0 khoản đầu là 0,
  và 0 nằm giữa 0 với 6. Đúng.
- *Bước.* Giả sử câu ấy đúng trước một lượt nào đó. Muốn vào lượt thì điều kiện
  phải đúng, tức `i < 6`, nên `so_quy[i]` là một ô có thật. Thân vòng cộng
  `so_quy[i]` vào `tong` rồi đẩy `i` lên `i + 1`. Sau lượt, `tong` bằng
  `T(i) + so_quy[i]`, đúng bằng `T(i + 1)`; và `i + 1` vẫn không vượt 6 vì
  trước lượt `i` còn nhỏ hơn 6. Vào đúng thì ra đúng.

**Mảnh 2 — thước đo dừng (bài 31).** Thước đo là `6 − i`: số nguyên, không âm
chừng nào vòng còn chạy, và tụt đúng 1 mỗi lượt vì thân vòng có đúng một dòng
`i = i + 1`. Nó xuất phát từ 6, mà số tự nhiên không giảm mãi được, nên vòng
chạy nhiều nhất 6 lượt rồi **thoát ra**.

**Mảnh 3 — điều kiện lặp sai lúc thoát (bài này).** Vòng thoát nghĩa là lần đọc
cuối cùng thấy `i < 6` **sai**. Phủ định của `i < 6` là `i ≥ 6` — đúng phép lật
dấu của bài 3, đọc trên trục số.

Bây giờ ghép, và ghép bằng đúng những phép của track này:

1. Từ mảnh 3: `i ≥ 6`.
2. Từ mảnh 1, vế `0 ≤ i ≤ 6`, lấy nửa phải: `i ≤ 6`.
3. Một số vừa không nhỏ hơn 6 vừa không lớn hơn 6 thì bằng 6. Vậy **`i = 6`**.
4. Từ mảnh 1, vế `tong = T(i)`, thay `i` bằng 6: **`tong = T(6)`** — tổng cả
   sáu khoản, tức 215 000 đồng.

Bốn dòng ấy là **bằng chứng**. Không có "chạy thử thấy đúng" ở dòng nào, và
không dòng nào nói về một con số cụ thể trong sổ — thay sáu khoản kia bằng sáu
khoản khác thì cả bốn dòng vẫn đứng nguyên.

Đáng để ý mảnh nào gánh việc gì, vì bỏ mảnh nào cũng đổ:

| bỏ mảnh nào | thì mất gì |
|---|---|
| bất biến | không biết `tong` liên quan gì tới `i`, nên biết `i = 6` cũng vô ích |
| thước đo | không biết có lúc nào "sau vòng" để mà đứng — vòng có thể chạy mãi |
| điều kiện sai lúc thoát | chỉ biết `i ≤ 6`, và `tong` có thể là tổng 4 khoản |

Và một ca đáng nhìn: cuốn sổ **chưa ghi tháng nào**. Vòng chạy 0 lượt. Bất biến
đúng nhờ mỗi cơ sở; thước đo bằng 0 ngay từ đầu nên vòng dừng ngay; điều kiện
`0 < 0` sai cho `i ≥ 0`, ghép với `i ≤ 0` ra `i = 0`; nên `tong = T(0) = 0`.
Bằng chứng chạy trơn qua ca ấy mà không cần thêm một câu nào — đúng như bài 11
đã dựng: một luật không có ai để vi phạm thì không ai phá được nó.
::::

::::predict{#doan-ba-cau-luc-thoat commitOnce}
Byte cho vòng chạy hết rồi hỏi máy ba câu về đúng cái khoảnh khắc vừa thoát ra.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]
i = 0
tong = 0

while i < len(so_quy):
    tong = tong + so_quy[i]
    i = i + 1

print(i < len(so_quy))
print(i == len(so_quy))
print(tong == 215000)
```

:::opt{correct}
`False`, rồi `True`, rồi `True`
:::

:::opt
`True`, rồi `True`, rồi `True`
::why
Gần đúng ở chỗ bạn đang giữ một thói quen rất tốt: điều kiện của một vòng là
cái **luật** của vòng ấy, và luật thì người ta mong nó đúng. Suốt sáu lượt vừa
rồi nó đúng thật, không lượt nào sai.

Chỗ lệch nằm ở lần đọc **cuối cùng** — lần thứ bảy. Sáu lần đọc đầu đều thấy
đúng và mỗi lần ấy mở ra một lượt. Lần thứ bảy thấy sai, và chính vì nó sai mà
máy mới đi xuống dòng `print`. Nếu nó vẫn đúng thì máy đã vào thân thêm lượt
nữa, và ba dòng `print` kia còn chưa tới lượt chạy. Nên đứng dưới vòng **này**
thì điều kiện lặp chắc chắn sai — đó là mẩu tin mà cả bài này đi tìm. (Chắc
chắn được là vì vòng này không có `break`; lý do đã nói ở mảnh 3.)
::
:::

:::opt
`False`, rồi `False`, rồi `True`
::why
Gần đúng ở chỗ bạn đọc trúng dòng đầu: thoát vòng thì `i < 6` phải sai, không
bàn cãi. Và cách nghĩ ở dòng hai cũng có gốc — "sai" ở đây nghĩa là `i` đã
**không còn nhỏ hơn** 6, mà "không nhỏ hơn" thì gồm cả chuyện lớn hơn.

Chỗ lệch: `i` không có cách nào nhảy quá 6. Thân vòng đẩy `i` lên đúng 1 mỗi
lượt, và lượt cuối cùng bắt đầu khi `i` còn bằng 5 — nên nó kết thúc ở 6, không
ở 7. Đây đúng chỗ cần **hai** mảnh chứ không phải một: điều kiện sai cho
`i ≥ 6`, còn bất biến giữ `i ≤ 6`. Ghép hai cái mới ra `i = 6`.
::
:::

:::opt
`False`, rồi `True`, rồi `False`
::why
Gần đúng ở chỗ hai dòng đầu bạn đọc trúng hết, kể cả chỗ khó là `i` dừng đúng
ở 6.

Chỗ lệch nằm ở dòng ba, và nó là chuyện thứ tự trong thân vòng. Ở lượt cuối,
dòng `tong = tong + so_quy[i]` chạy **trước** dòng `i = i + 1`, nên khoản 45 000
của tháng 6 kịp vào `tong` rồi `i` mới nhích lên 6.

Đổi chỗ hai dòng ấy thì không phải "hụt một tháng" — nó hỏng nặng hơn thế. Đẩy
`i` lên trước làm mất khoản tháng ĐẦU, rồi tới lượt sáu máy đi đọc `so_quy[6]`,
một ngăn không có, và chương trình nổ `IndexError`. Bất biến "trước mỗi lượt
`tong` bằng tổng `i` khoản đầu" chính là câu nói ra chuyện `i` và `tong` luôn
khớp nhau, không cái nào chạy trước cái nào — và khi nó gãy thì cả cái cớ để
tin vòng dừng đúng chỗ cũng gãy theo.
::
:::
::::

::::code{#viet-bang-chung-thanh-assert}
Bằng chứng đã xong bằng tiếng Việt. Việc cuối của cả track: viết nó thành mã để
máy **kiểm lại** — và chỉ kiểm lại, vì máy không chứng minh hộ ai được điều gì.

Hàm `tong_cua(so, k)` đã viết sẵn: nó trả về `T(k)`, tổng của `k` khoản đầu
tiên trong sổ. Sáu câu `assert` cũng đã viết sẵn, mỗi câu canh một mẩu của bằng
chứng — việc của bạn là đưa cho ba trong số ấy **con số mà bằng chứng nói nó
phải bằng**.

Ba chỗ trống:

1. `tong_dung_ra = ___` — theo bất biến, sau lượt này `tong` **đáng ra phải
   bằng** cái gì? (Câu `assert` ngay dưới sẽ đem nó so với `tong` thật.)
2. `thuoc_do_bay_gio = ___` — thước đo **ngay lúc này** là bao nhiêu? (Câu
   `assert` dưới đem nó so với `con_lai - 1`, tức thước đo đầu lượt đã tụt 1;
   dòng `con_lai =` đã ghi hộ giá trị đầu lượt.)
3. `so_thang_trong_so = ___` — cuốn sổ này có bao nhiêu tháng? (Câu `assert`
   cuối đem nó so với `i` lúc thoát.)

Bài chấm bằng **năm cuốn sổ**, trong đó có một cuốn rỗng, một cuốn một tháng, và
một cuốn có tháng thu 0 đồng. Ba chỗ trống phải dựng từ cuốn sổ đang cầm và từ
`i`, không được là con số nhớ sẵn của riêng sổ quỹ CLB.

```python title=starter
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]


def tong_cua(so, k):
    goi = 0
    for j in range(k):
        goi = goi + so[j]
    return goi


def cong_quy(so):
    i = 0
    tong = 0
    assert tong == tong_cua(so, i), "cơ sở hỏng: trước lượt đầu chưa cộng khoản nào, nên tong phải bằng tổng 0 khoản đầu"
    while i < len(so):
        con_lai = len(so) - i
        tong = tong + so[i]
        i = i + 1
        tong_dung_ra = ___
        thuoc_do_bay_gio = ___
        assert tong == tong_dung_ra, "bất biến gãy: sau lượt này tong không còn bằng tổng i khoản đầu của sổ"
        assert thuoc_do_bay_gio == con_lai - 1, "thước đo không tụt đúng 1 sau lượt này, nên chưa có gì bảo đảm vòng dừng"
        assert thuoc_do_bay_gio >= 0, "thước đo tụt xuống dưới 0 — nó phải là số nguyên không âm ở mọi lúc vòng còn chạy"
    assert not (i < len(so)), "vòng này không có `break`, nên thoát ra nghĩa là điều kiện lặp đã sai — nếu nó còn đúng thì máy đã vào thân thêm một lượt nữa chứ chưa xuống tới đây"
    so_thang_trong_so = ___
    assert i == so_thang_trong_so, "ghép lại: điều kiện lặp sai cho i không nhỏ hơn số tháng, bất biến giữ i không lớn hơn số tháng"
    return tong


print(cong_quy(so_quy))
print(cong_quy([]))
print(cong_quy([1, 2, 3]))
```

```python title=solution
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]


def tong_cua(so, k):
    goi = 0
    for j in range(k):
        goi = goi + so[j]
    return goi


def cong_quy(so):
    i = 0
    tong = 0
    assert tong == tong_cua(so, i), "cơ sở hỏng: trước lượt đầu chưa cộng khoản nào, nên tong phải bằng tổng 0 khoản đầu"
    while i < len(so):
        con_lai = len(so) - i
        tong = tong + so[i]
        i = i + 1
        tong_dung_ra = tong_cua(so, i)
        thuoc_do_bay_gio = len(so) - i
        assert tong == tong_dung_ra, "bất biến gãy: sau lượt này tong không còn bằng tổng i khoản đầu của sổ"
        assert thuoc_do_bay_gio == con_lai - 1, "thước đo không tụt đúng 1 sau lượt này, nên chưa có gì bảo đảm vòng dừng"
        assert thuoc_do_bay_gio >= 0, "thước đo tụt xuống dưới 0 — nó phải là số nguyên không âm ở mọi lúc vòng còn chạy"
    assert not (i < len(so)), "vòng này không có `break`, nên thoát ra nghĩa là điều kiện lặp đã sai — nếu nó còn đúng thì máy đã vào thân thêm một lượt nữa chứ chưa xuống tới đây"
    so_thang_trong_so = len(so)
    assert i == so_thang_trong_so, "ghép lại: điều kiện lặp sai cho i không nhỏ hơn số tháng, bất biến giữ i không lớn hơn số tháng"
    return tong


print(cong_quy(so_quy))
print(cong_quy([]))
print(cong_quy([1, 2, 3]))
```

```python title=test
# Hai cuốn sổ ở đầu là hai ca dễ trượt nhất, nên chúng chạy TRƯỚC: sổ rỗng
# (vòng 0 lượt, câu kết luận vẫn phải đúng dù không lượt nào chạy) và sổ có
# tháng thu 0 đồng (thước đo vẫn phải tụt 1 dù `tong` đứng yên). Xếp chúng sau
# thì một câu ở dưới trượt trước, và hai ca ấy không bao giờ được chạm tới.
assert cong_quy([]) == 0, "sổ chưa ghi tháng nào: vòng chạy 0 lượt, và lúc thoát i vẫn phải bằng số tháng trong sổ, tức bằng 0"
assert cong_quy([7, 0, 0, 5]) == 12, "7 + 0 + 0 + 5 = 12 — một tháng thu 0 đồng vẫn là một lượt, thước đo vẫn phải tụt 1"
assert cong_quy(so_quy) == 215000, "sổ quỹ sáu tháng của CLB cộng lại đúng 215000 đồng"
assert cong_quy([50000]) == 50000, "sổ một tháng: đúng một lượt, tong bằng khoản duy nhất"
assert cong_quy([1, 2, 3]) == 6, "1 + 2 + 3 = 6 — bằng chứng không dựa vào con số riêng nào của sổ quỹ"
assert tong_cua(so_quy, 0) == 0, "tổng 0 khoản đầu là 0 — đó là chỗ cơ sở của bất biến đứng"
assert tong_cua(so_quy, 6) == 215000, "tổng 6 khoản đầu là cả cuốn sổ: 215000 đồng"
```

:::hints
- kind: attention
  body: Cả ba chỗ trống đều là một GIÁ TRỊ, không phải một câu Đ/S — câu Đ/S đã nằm sẵn trong ba dòng `assert` ngay dưới. Đọc lại từng câu `assert` ấy để biết chỗ trống phải mang con số nào: câu đầu so với `tong`, câu giữa so với `con_lai - 1`, câu cuối so với `i`. Và để ý `i` vừa nhích lên ở dòng ngay trên hai chỗ trống đầu.
- kind: strategy
  body: "Chỗ thứ nhất: bất biến nói `tong` bằng tổng `i` khoản đầu, mà hàm tính tổng ấy đã có sẵn — đưa vào đúng hai thứ nó cần là cuốn sổ và số khoản đã cộng. Chỗ thứ hai: thước đo dựng y như bài 31, số tháng có trong sổ trừ số tháng đã đọc. Chỗ thứ ba: hỏi thẳng cuốn sổ xem nó dài bao nhiêu — đừng gõ số 6, vì trong năm cuốn sổ đem chấm chỉ có một cuốn sáu tháng."
- kind: one-line
  body: "Ba chỗ lần lượt là `tong_cua(so, i)`, `len(so) - i`, và `len(so)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: ba chỗ trống phải dựng từ cuốn sổ đang cầm và từ `i` — gõ một con số cố định thì bài đang cấp một tấm bằng chứng cho thứ nó chưa hề kiểm
  requireAst:
  # Khung khởi đầu gọi `tong_cua` đúng 1 lần (ở câu cơ sở trước vòng); lời giải
  # gọi 2. Luật này chặn mọi cách viết chỗ trống 1 không tra lại tổng thật:
  # `True`, `tong`, `0`.
  - kind: uses-call, target: tong_cua, min: 2
  # Khung khởi đầu gọi `len` 3 lần; lời giải gọi 5 — hai lần thêm nằm đúng ở
  # chỗ trống 2 và chỗ trống 3. Đây là chỗ chặn `so_thang_trong_so = 6` và
  # chặn cả `thuoc_do_bay_gio = con_lai - 1` (một câu tự so với chính nó).
  - kind: uses-call, target: len, min: 5
  # Khung khởi đầu đọc `i` 7 lần; lời giải đọc 9 — chỗ trống 1 và 2 mỗi chỗ
  # thêm một lần. Không có luật này thì `len(so) - 1` lọt qua luật `len` trên
  # mà vẫn không đo theo số tháng ĐÃ đọc.
  - kind: uses-name, target: i, min: 9
  forbidAst:
  # Lưới thứ hai, chặn đúng con số là ĐÁP ÁN của riêng cuốn sổ sáu tháng. Mọi
  # cách viết hợp lệ dựng từ `len(so)`, `i` và `con_lai`, nên không cách nào
  # chứa nguyên văn số 6.
  - kind: has-literal, target: 6
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^215000\n0\n6\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba câu, không câu nào nhắc tới 215 000. Vậy mà chúng nói được về cả cuốn sổ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp — câu cuối của cả track.

Nhìn lại chặng vừa qua. Bài 1 hỏi câu nào phân xử được. Bài 21 lấy đi cái máy
kiểm-hết-mọi-trường-hợp và bắt bạn tự đi bằng chân. Từ đó tới đây là các cách
đi: mở định nghĩa, xét một số bất kỳ, chứng minh phản đảo, phản chứng, quy nạp,
quy nạp mạnh, bất biến, thước đo. Hôm nay chúng ghép lại thành một bằng chứng
cho đúng cái vòng `while` mà bạn đã gõ từ T1.2, hồi chưa ai nói với bạn rằng nó
cần một bằng chứng.

Và bài 21 nói lại lần cuối, ở chỗ nó cắn đau nhất: khối test vừa xanh **không**
phải bằng chứng. Nó chỉ nói năm cuốn sổ ấy chưa thấy sai. Cuốn sổ thứ sáu thì
nó chưa từng nghe tới.

Bằng chứng bạn vừa viết bằng tiếng Việt, máy không đọc được — nên vẫn phải chạy
test mới yên tâm. Có cách nào nói cho máy nghe **một phần** bằng chứng, để nó
bắt lỗi trước cả khi chạy?

Có. Và mẩu đầu tiên nhỏ tới bất ngờ: nói cho máy biết một cái tên được phép
mang **loại giá trị nào** — số nguyên, chữ, hay Đ/S — là đã nhờ nó kiểm hộ một
câu trong bằng chứng, và kiểm mà không cần chạy dòng nào. Realm 3 bắt đầu từ
đúng chỗ đó: bit, byte, và kiểu dữ liệu.
::::

::::checkpoint{mastery=0.85}
::::
