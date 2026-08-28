---
id: toan.logic-va-chung-minh.dieu-dung-lai-sau-moi-luot
title: Điều đúng lại sau mỗi lượt
summary: Một câu đúng trước lượt đầu và hễ đúng trước một lượt thì đúng sau lượt đó — chứng minh nó chính là quy nạp, chỉ đổi tên "bậc thứ n" thành "sau lượt thứ n".
locale: vi
track: toan
module: logic-va-chung-minh
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [logic.loop-invariant]
requires: [logic.strong-induction, logic.induction-base-case, logic.induction-hypothesis, logic.induction, logic.implication, logic.direct-proof, logic.finite-check-not-proof, logic.counterexample, logic.for-all, math.multiplication, core.boolean, core.variable, core.reassign, core.number-literal, core.arithmetic, core.accumulator, core.read-modify-write, core.function-def, core.function-call, core.function-parameter, core.function-return, core.list, core.list-index, core.len, core.builtin-function, core.print-variable, ctrl.while, ctrl.while-check-timing, ctrl.loop-progress, ctrl.comparison]
concepts: [logic.bat-bien-vong-lap, logic.quy-nap-theo-so-luot, logic.viet-bat-bien-thanh-assert]
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
Vòng này mình gõ từ hồi T1.2. Gõ thì chạy, mà chưa lần nào mình chứng minh được gì về nó.
::::

::::explain{#bac-doi-ten-thanh-luot}
Bài trước dựng một cái bảng từ dưới lên, và nó chạy theo bậc: bậc thứ nhất, bậc
thứ hai, bậc thứ ba… Câu hỏi để lại là: vòng `while` cũng chạy theo đúng một
thứ tự như thế — lượt 1, lượt 2, lượt 3 — thì sau mỗi lượt có câu nào cứ đúng đi
đúng lại không?

Lấy đúng cuốn sổ quỹ của CLB cờ vua lớp 6A. Sổ ghi sáu tháng, mỗi tháng một
khoản:

| tháng | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|
| thu được (đồng) | 40 000 | 25 000 | 60 000 | 15 000 | 30 000 | 45 000 |

Và đúng cái vòng cộng dồn của T1.2.12:

```python title=readonly
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]
i = 0
tong = 0

while i < len(so_quy):
    tong = tong + so_quy[i]
    i = i + 1
```

Chạy tay từng lượt, ghi lại hai cái tên đang mang giá trị gì **trước** mỗi lượt:

| trước lượt | `i` | `tong` |
|---|---|---|
| lượt 1 | 0 | 0 |
| lượt 2 | 1 | 40 000 |
| lượt 3 | 2 | 65 000 |
| lượt 4 | 3 | 125 000 |
| lượt 5 | 4 | 140 000 |
| lượt 6 | 5 | 170 000 |
| không còn lượt nào | 6 | 215 000 |

Bảy dòng, bảy cặp số khác nhau. Không con số nào đứng yên. Nhưng có một **câu**
đứng yên, và nó đúng ở cả bảy dòng:

> `tong` đúng bằng tổng của `i` khoản **đầu tiên** trong sổ.

Kiểm thử vài dòng: dòng đầu, `i` là 0 và `tong` là 0 — tổng của 0 khoản đầu là
0, khớp. Dòng thứ ba, `i` là 2 và `tong` là 65 000, mà 40 000 + 25 000 = 65 000
nên khớp. Dòng cuối, `i` là 6 và `tong` là 215 000, đúng bằng **tổng quỹ sáu tháng
là 215 000 đồng**.

Câu ấy có tên:

> **Bất biến vòng lặp** là một câu về các biến của vòng, thoả hai điều:
>
> 1. nó **đúng trước lượt đầu tiên**;
> 2. hễ nó **đúng trước một lượt** thì nó vẫn **đúng sau lượt đó**.
>
> Từ hai điều ấy suy ra: nó đúng trước và sau **mọi** lượt, bao nhiêu lượt cũng
> vậy.
::::

::::explain{#hai-viec-quen-mat-doi-ten}
Đọc lại hai điều vừa nêu và so với hai việc của bài 26 và 28.

| quy nạp | bất biến vòng lặp |
|---|---|
| cơ sở: bậc đầu tiên đúng | câu đúng **trước lượt đầu** |
| bước: bậc `k` đúng thì bậc `k + 1` đúng | câu đúng trước một lượt thì **đúng sau lượt đó** |
| kết luận: đúng ở mọi bậc | kết luận: đúng ở **mọi lượt** |

Trùng khít. Không phải "giống", mà là **cùng một thứ**: bất biến vòng lặp chính
là quy nạp, với chữ "bậc thứ `n`" đổi tên thành "sau lượt thứ `n`".

Nên hôm nay không có nguyên lý nào mới cả. Cái mới là chỗ đem nó tới: một vòng
`while` bạn đã gõ từ lâu.

Bây giờ chứng minh cho tử tế. Gọi `T(k)` là tổng của `k` khoản đầu tiên trong
sổ. Câu cần giữ, viết đủ, gồm hai vế nối bằng chữ "và" của bài 4:

> `tong = T(i)` **và** `0 ≤ i ≤ 6`.

Vế thứ hai trông như thừa. Nó không thừa: `T(i)` chỉ có nghĩa khi `i` không
vượt ra ngoài cuốn sổ, và bài sau sẽ cần đúng vế ấy.

**Việc 1 — trước lượt đầu.** `i` là 0 và `tong` là 0. Tổng của 0 khoản đầu là 0,
nên `tong = T(0)`. Và 0 nằm giữa 0 với 6. Đúng.

**Việc 2 — một lượt bất kỳ.** Giả sử câu ấy đúng trước một lượt nào đó — đây
đúng là **giả thiết quy nạp** của bài 27, không phải một điều đã có sẵn. Vì máy
vào được thân vòng nên điều kiện `i < 6` đúng, do đó `so_quy[i]` là một ô có
thật. Thân vòng làm hai việc, theo đúng thứ tự này:

- `tong` thành `T(i) + so_quy[i]`, mà đó chính là `T(i + 1)`;
- `i` thành `i + 1`.

Sau lượt, cái tên `i` mang giá trị `i + 1`, và `tong` đúng bằng `T` của cái giá
trị mới ấy. Còn vế thứ hai: trước lượt `i` nhỏ hơn 6, nên sau lượt `i + 1`
nhiều nhất là 6. Vào lượt đúng thì ra lượt cũng đúng.

Hai việc xong. Nên câu ấy đúng trước và sau mọi lượt, khỏi cần chạy thử dòng
nào.

Đáng để ý: **thứ tự hai dòng trong thân vòng là chỗ chứng minh này sống hay
chết.** Đảo chúng lại — đẩy `i` lên trước rồi mới cộng — thì sau lượt `tong` chỉ
bằng `T(i)` của giá trị `i` cũ, mà `i` đã nhích, nên hai bên lệch nhau đúng một
tháng. Bất biến gãy ngay lượt đầu.
::::

::::predict{#doan-tong-k-khoan-dau commitOnce}
Trước khi bắt máy canh bất biến, Byte thử riêng cái hàm phát biểu nó: `T(k)`,
tổng của `k` khoản **đầu tiên**.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]

def tong_k_khoan_dau(so, k):
    tong = 0
    j = 0
    while j < k:
        tong = tong + so[j]
        j = j + 1
    return tong

print(tong_k_khoan_dau(so_quy, 0))
print(tong_k_khoan_dau(so_quy, 3))
print(tong_k_khoan_dau(so_quy, 6))
print(tong_k_khoan_dau(so_quy, 2) + so_quy[2] == tong_k_khoan_dau(so_quy, 3))
```

:::opt{correct}
`0`, `125000`, `215000`, `True`
:::

:::opt
`40000`, `125000`, `215000`, `True`
::why
Gần đúng ở ba dòng, và ở một lối đọc rất hợp lý: bạn hiểu tham số `k` là "khoản
thứ mấy", nên `k` bằng 0 được đọc thành "khoản mở đầu" và cho ra 40 000.

Chỗ lệch là `k` ở đây đếm **số lượng khoản**, không đánh số khoản nào cả. Nhìn
vòng `while j < k` mà xem: với `k` bằng 0 nó không chạy lượt nào, nên `tong` giữ
nguyên giá trị 0 lúc khai sinh. Và chuyện đó là chỗ dựa của cả bài: trước lượt
đầu tiên `i` là 0 và `tong` là 0, nên bất biến đứng được ngay từ đầu chính nhờ
tổng của 0 khoản bằng 0.
::
:::

:::opt
`0`, `65000`, `170000`, `False`
::why
Gần đúng ở dòng đầu, và ở chỗ bạn cẩn thận với đánh số từ 0 — một cẩn thận rất
đáng có, vì `so_quy[0]` đúng là khoản tháng đầu tiên.

Chỗ lệch là bạn mang cách đánh số ấy sang cho `k`, thành ra cộng thiếu một
khoản mỗi lần. Vòng chạy với `j` bằng 0, 1, 2 khi `k` bằng 3 — tức **ba** ô đầu,
cộng lại 125 000 chứ không phải 65 000. Con số 65 000 là tổng của hai khoản đầu,
tức `T(2)`. Cũng vì thế `T(6)` là cả sáu khoản, 215 000, chứ không phải 170 000.
::
:::

:::opt
`0`, `125000`, `215000`, `False`
::why
Gần đúng ở cả ba dòng đầu — bạn đọc `T(k)` chính xác hoàn toàn.

Chỗ lệch nằm ở dòng cuối, tại chỗ `so_quy[2]`. Nhiều người đọc nó là "khoản
tháng 2", tức 25 000, và khi ấy 65 000 cộng 25 000 ra 90 000, khác `T(3)` nên
`False`. Nhưng `so_quy[2]` là ô **chỉ số 2**, mà đếm từ 0 thì ô ấy là khoản
tháng 3, tức 60 000. Cộng vào 65 000 được đúng 125 000. Dòng cuối chính là việc
thứ hai của chứng minh viết thành một câu hỏi: `T(i)` cộng khoản thứ `i` ra
đúng `T(i + 1)`.
::
:::
::::

::::code{#viet-bat-bien-thanh-assert}
Bất biến bạn vừa chứng minh nằm trên giấy. Giờ viết nó thành mã, và đặt vào
**trong thân vòng**, để mỗi lượt máy tự kiểm lại một lần.

Vai của máy vẫn không đổi từ bài 21: một câu `assert` không đạt là bằng chứng
rằng có chỗ sai; một câu `assert` đạt suốt sáu lượt chỉ nói **"chưa thấy sai
trên sáu lượt ấy"**. Chỗ chứng minh vẫn là hai việc bạn vừa đọc.

Hàm `bat_bien` phát biểu câu cần giữ, và nó có hai chỗ trống — đúng hai vế của
câu:

1. `tong == ___` — vế thứ nhất: `tong` phải bằng tổng của bao nhiêu khoản đầu?
2. `0 <= i <= ___` — vế thứ hai: `i` không được vượt qua đâu?

Hai câu `assert` gọi `bat_bien` đã đặt sẵn ở hai chỗ, đúng hai việc của chứng
minh: một câu đứng **trước vòng** canh cơ sở, một câu đứng **cuối thân vòng**
canh bước.

Một luật của bài: **vế thứ hai phải hỏi chính cuốn sổ đang cầm xem nó dài bao
nhiêu**, đừng gõ một con số. Bài chấm bằng bốn cuốn sổ dài ngắn khác nhau,
trong đó có một cuốn rỗng và một cuốn **tám** tháng — một con số nhớ sẵn thì
đúng với đúng một cuốn.

```python title=starter
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]


# T(k) — tổng của k khoản ĐẦU TIÊN trong sổ.
def tong_k_khoan_dau(so, k):
    tong = 0
    j = 0
    while j < k:
        tong = tong + so[j]
        j = j + 1
    return tong


# Câu cần giữ đúng trước mỗi lượt và sau mỗi lượt.
def bat_bien(so, i, tong):
    return tong == ___ and 0 <= i <= ___


def cong_quy(so):
    i = 0
    tong = 0
    assert bat_bien(so, i, tong), "cơ sở gãy: trước lượt đầu tiên i là 0 và tong là 0, mà câu bạn viết không nhận đúng ca đó"
    while i < len(so):
        tong = tong + so[i]
        i = i + 1
        assert bat_bien(so, i, tong), "bước gãy: vào lượt này câu còn đúng, ra khỏi lượt thì hết đúng"
    return tong


print(cong_quy(so_quy))
print(cong_quy([]))
print(bat_bien(so_quy, 3, 125000))
print(bat_bien(so_quy, 3, 999))
```

```python title=solution
so_quy = [40000, 25000, 60000, 15000, 30000, 45000]


# T(k) — tổng của k khoản ĐẦU TIÊN trong sổ.
def tong_k_khoan_dau(so, k):
    tong = 0
    j = 0
    while j < k:
        tong = tong + so[j]
        j = j + 1
    return tong


# Câu cần giữ đúng trước mỗi lượt và sau mỗi lượt.
def bat_bien(so, i, tong):
    return tong == tong_k_khoan_dau(so, i) and 0 <= i <= len(so)


def cong_quy(so):
    i = 0
    tong = 0
    assert bat_bien(so, i, tong), "cơ sở gãy: trước lượt đầu tiên i là 0 và tong là 0, mà câu bạn viết không nhận đúng ca đó"
    while i < len(so):
        tong = tong + so[i]
        i = i + 1
        assert bat_bien(so, i, tong), "bước gãy: vào lượt này câu còn đúng, ra khỏi lượt thì hết đúng"
    return tong


print(cong_quy(so_quy))
print(cong_quy([]))
print(bat_bien(so_quy, 3, 125000))
print(bat_bien(so_quy, 3, 999))
```

```python title=test
# Hai câu đầu canh đúng hai cái bẫy của bài, nên chúng đứng trước. Bẫy thứ
# nhất: một câu luôn đúng thì không canh được gì, nên phải có một ca mà nó
# BUỘC phải trả False. Bẫy thứ hai: cuốn sổ tám tháng, thứ mà một con số nhớ
# sẵn không đi qua nổi.
assert bat_bien(so_quy, 3, 999) == False, "999 không phải tổng ba khoản đầu của sổ này, nên câu phải trả False ở đúng ca đó"
assert cong_quy([10, 20, 30, 40, 50, 60, 70, 80]) == 360, "sổ tám tháng vẫn phải chạy trót lọt: cộng tám khoản ấy được 360"
assert bat_bien(so_quy, 3, 125000) == True, "125000 đúng bằng tổng ba khoản đầu của sổ, và 3 nằm giữa 0 với sáu tháng đã ghi"
assert bat_bien(so_quy, 0, 0) == True, "đây là chính ca cơ sở: chưa cộng khoản nào thì tổng bằng 0"
assert cong_quy(so_quy) == 215000, "sổ quỹ sáu tháng cộng lại được 215000 đồng, và bất biến phải đứng suốt cả sáu lượt"
assert cong_quy([]) == 0, "sổ chưa ghi tháng nào: vòng chạy 0 lượt, và câu cơ sở vẫn phải đứng"
assert cong_quy([7]) == 7, "sổ đúng một tháng: một lượt duy nhất, và bất biến phải đúng cả trước lẫn sau lượt ấy"
assert tong_k_khoan_dau(so_quy, 0) == 0, "tổng của 0 khoản đầu là 0 — đó là chỗ cơ sở của bất biến đứng"
assert tong_k_khoan_dau(so_quy, 6) == 215000, "tổng của cả sáu khoản đầu đúng bằng tổng quỹ sáu tháng"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trên cùng một dòng, nối nhau bằng chữ `and` — đó là hai vế của câu bạn vừa chứng minh trên giấy. Đọc lại đúng câu ấy trong phần giải thích: vế thứ nhất nói `tong` bằng `T` của một cái gì đó, vế thứ hai nói `i` bị kẹp giữa hai mốc. Và để ý ba cái tên mà `bat_bien` nhận vào — `so` là cuốn sổ, `i` là số khoản đã cộng, `tong` là số tiền đang cộng dồn.
- kind: strategy
  body: "Chỗ thứ nhất: hàm `tong_k_khoan_dau` ở ngay trên chính là `T`, và nó cần hai thứ — cuốn sổ nào, và bao nhiêu khoản đầu. Số khoản đã cộng vào `tong` chính là cái tên `i` đang mang. Chỗ thứ hai: mốc trên của `i` là số tháng CÓ TRONG cuốn sổ đang cầm, mà độ dài một danh sách thì hỏi thẳng nó là ra — đừng gõ con số, vì trong bốn cuốn sổ đem chấm chỉ có một cuốn sáu tháng."
- kind: one-line
  body: "Chỗ thứ nhất là `tong_k_khoan_dau(so, i)`; chỗ thứ hai là `len(so)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: vế thứ nhất phải TÍNH LẠI tổng i khoản đầu bằng `tong_k_khoan_dau`, và vế thứ hai phải hỏi độ dài cuốn sổ đang cầm — gõ cứng hay so `tong` với chính nó thì bài đang chấm một thứ nó không hề kiểm
  requireAst:
  # Khung khởi đầu KHÔNG gọi `tong_k_khoan_dau` lần nào (dòng `def` là khai
  # báo, không phải lời gọi). Luật này một mình chặn đáp án `tong == tong`,
  # thứ luôn đúng nên không khối test nào bắt được.
  - kind: uses-call, target: tong_k_khoan_dau, min: 1
  # Khung khởi đầu gọi `len` 1 lần (ở điều kiện `while`); lời giải gọi 2. Luật
  # này chặn đáp án `0 <= i <= i`, thứ cũng luôn đúng.
  - kind: uses-call, target: len, min: 2
  # Bất biến phải nhắc tới chính cuốn sổ được đưa vào. Khung khởi đầu đọc `so`
  # 5 lần; lời giải đọc 7.
  - kind: uses-name, target: so, min: 7
  # …và phải nhắc tới số khoản ĐÃ cộng. Khung khởi đầu đọc `i` 5 lần; lời giải
  # đọc 7.
  - kind: uses-name, target: i, min: 7
  forbidAst:
  # Lưới thứ hai. Con số 6 chạy đúng với cuốn sổ sáu tháng, nhưng nó nói về
  # một cuốn sổ nhớ sẵn chứ không phải cuốn đang cầm. Mọi cách viết hợp lệ đều
  # dựng từ `so`, `i` và `tong_k_khoan_dau`, nên không cách nào chứa nguyên văn
  # con số ấy.
  - kind: has-literal, target: 6
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^215000\n0\nTrue\nFalse\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu lượt, sáu lần máy gật đầu. Mà cái làm mình yên tâm là hai việc trên giấy kia.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bất biến đúng suốt vòng — đúng trước lượt đầu, đúng sau mọi lượt, và bạn có
chứng minh cho cả hai chứ không chỉ có sáu lần máy gật đầu.

Nhưng đọc kỹ xem nó hứa gì. Nó nói: *sau lượt nào cũng vậy*. Nếu vòng cứ chạy
mãi thì nó cứ đúng mãi — và bạn chẳng bao giờ có kết quả trong tay để mà đọc.

Ở T1.2 bạn đã gặp một vòng không chịu dừng, và cách xử lý lúc ấy là bấm Dừng
rồi đi tìm dòng thiếu. Giờ câu hỏi khác hẳn: lấy gì **bảo đảm** vòng này dừng?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
