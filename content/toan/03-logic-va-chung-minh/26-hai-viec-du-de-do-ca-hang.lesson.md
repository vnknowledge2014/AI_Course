---
id: toan.logic-va-chung-minh.hai-viec-du-de-do-ca-hang
title: Hai việc là đủ để đổ cả hàng
summary: Một dãy vô hạn câu không thể chứng minh từng câu một — nhưng nếu câu đầu đúng và mỗi câu đẩy được câu ngay sau nó, thì cả hàng đổ, và bạn chỉ phải làm đúng hai việc.
locale: vi
track: toan
module: logic-va-chung-minh
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.induction]
requires: [logic.proof-by-contradiction, logic.proof-by-contrapositive, logic.direct-proof, logic.use-definition, logic.finite-check-not-proof, logic.implication, logic.vacuous-truth, logic.contradiction, logic.for-all, logic.exists, logic.counterexample, logic.open-sentence, logic.truth-value, logic.negation, logic.not, math.multiplication, math.multiply-commutative, math.factor-common, math.expand-brackets, math.letter-names-a-slot, math.equivalent-expression, core.boolean, core.variable, core.number-literal, core.arithmetic, core.floor-division, core.accumulator, core.reassign, core.list, core.list-comprehension, core.builtin-function, core.function-def, core.function-call, core.function-parameter, core.function-return, core.print-variable, ctrl.while, ctrl.for-range, ctrl.comparison]
concepts: [logic.quy-nap, logic.hang-domino, logic.hai-viec-du]
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
Vô hạn câu thì chứng minh từng câu một là hết đời. Nhưng mình đâu có phải đẩy từng quân domino.
::::

::::explain{#mot-day-khong-co-dong-cuoi}
Bài trước để lại một dãy, không phải một câu:

> 1 + 2 + … + n = n(n+1)/2, với mọi số nguyên dương n.

Viết vài dòng đầu của dãy ấy ra cho thấy mặt:

| bậc | câu | vế trái | vế phải |
|---|---|---|---|
| 1 | 1 = 1 × 2 / 2 | 1 | 1 |
| 2 | 1 + 2 = 2 × 3 / 2 | 3 | 3 |
| 3 | 1 + 2 + 3 = 3 × 4 / 2 | 6 | 6 |
| 4 | 1 + 2 + 3 + 4 = 4 × 5 / 2 | 10 | 10 |
| 5 | 1 + 2 + 3 + 4 + 5 = 5 × 6 / 2 | 15 | 15 |

Mỗi dòng là **một mệnh đề riêng**, phân xử được riêng. Bảng này có năm dòng, và
nó không bao giờ có dòng cuối.

Ba lối đã có đều chốt được **một** câu: trực tiếp (bài 23), phản đảo (bài 24),
phản chứng (bài 25). Đem lối nào vào đây thì cũng chốt xong một dòng rồi phải
bắt đầu lại từ đầu ở dòng sau. Chốt vô hạn lần là chuyện không xong.

Và bài 21 đã đóng cửa còn lại: kiểm 40 dòng, kiểm 4 000 dòng, vẫn không phải
chứng minh.

Nhưng cuối bài trước có một chỗ gợn mà ta để dở. Đọc lại cột "vế trái":

```text
  bậc 4:  1 + 2 + 3 + 4      = 10
  bậc 5:  1 + 2 + 3 + 4 + 5  = 15
```

Vế trái của bậc 5 **là vế trái của bậc 4, cộng thêm đúng số 5**. Không phải một
tổng mới toanh — là tổng cũ, thêm một mẩu. Và 10 + 5 = 15, đúng bằng vế phải của
bậc 5.

Chỗ ấy không phải chuyện riêng của bậc 4 với bậc 5. Hai bậc **nào** cạnh nhau
cũng dính nhau kiểu ấy: bậc k+1 chỉ hơn bậc k một số hạng, và số hạng ấy là
chính k+1.

Vậy thay vì chứng minh từng dòng, ta đổi sang chứng minh **mối dính** giữa hai
dòng liền nhau.
::::

::::example{#hang-domino-cua-clb}
Chiều thứ Sáu, CLB cờ vua lớp 6A dựng một hàng domino dọc theo mép bàn. Rất dài
— dài tới mức từ đầu này không nhìn thấy đầu kia.

Thầy phụ trách hỏi: *muốn cả hàng đổ, các em phải bảo đảm mấy điều?*

Nam đề nghị đi kiểm từng quân xem quân nào đổ được. Cách đó đúng, và nó không
bao giờ xong.

Lan nói hai điều là đủ:

```text
  việc 1:  đẩy đổ quân ĐẦU TIÊN
  việc 2:  xếp sao cho HỄ một quân đổ THÌ nó đụng ngã quân ngay sau nó
```

Xong hai việc ấy thì cả hàng đổ, không cần chạm vào quân thứ ba nghìn.

Để ý kỹ việc 2. Nó **không** nói quân nào đang đổ. Nó nói một chuyện có dạng
"nếu… thì" — đúng câu kéo theo của bài 10 — và nó nói câu ấy cho **mọi** vị trí
trong hàng cùng lúc.

Dịch sang một dãy mệnh đề, đặt tên từng bậc là P(1), P(2), P(3), … (chữ P kèm
một chỗ trống, đúng câu mở của bài 16 — điền số vào chỗ trống là ra một mệnh đề
phân xử được):

> **Nguyên lý quy nạp.** Muốn khẳng định P(n) đúng với **mọi** số nguyên dương
> n, chỉ cần hai việc:
>
> - **Cơ sở:** P(1) đúng.
> - **Bước quy nạp:** với **mọi** k, câu *"nếu P(k) đúng thì P(k+1) đúng"* là
>   một câu đúng.
>
> Xong hai việc ấy thì P(n) đúng với mọi n.

Hai việc, và cả hai đều **hữu hạn**: một câu để kiểm, và một câu kéo theo để
chứng minh. Không có việc thứ ba nào chạy tới vô hạn.

Vì sao hai việc là đủ? Đi thử vài bậc bằng tay:

```text
  cơ sở cho P(1).
  bước quy nạp ở k = 1  cho:  P(1) → P(2).   Có P(1) rồi, nên có P(2).
  bước quy nạp ở k = 2  cho:  P(2) → P(3).   Có P(2) rồi, nên có P(3).
  bước quy nạp ở k = 3  cho:  P(3) → P(4).   Có P(3) rồi, nên có P(4).
```

Muốn tới bậc 500 thì đi 499 nhịp như thế. Đường đi có thể dài, nhưng nó **luôn
có**, cho mọi bậc — và đó đúng là điều "với mọi n" đòi hỏi.
::::

::::example{#chung-minh-cong-thuc-tong}
Áp nguyên lý ấy vào đúng câu đang treo.

Đặt P(n) là câu: *1 + 2 + … + n = n(n+1)/2.*

**Việc 1 — cơ sở.**

P(1) nói: 1 = 1 × 2 / 2. Vế trái là 1. Vế phải: 1 × 2 / 2 = 1. Hai bên bằng nhau,
nên P(1) đúng. Việc thứ nhất xong, và nó tốn đúng một dòng.

**Việc 2 — bước quy nạp.**

Phải chứng minh câu kéo theo: *nếu P(k) đúng thì P(k+1) đúng*, cho mọi k.

Đây là một câu "nếu… thì", nên dùng đúng khuôn ba nhịp của bài 23: nhặt vế trước
lên làm giả thiết, rồi đi tới vế sau.

*Nhặt giả thiết.* Giả sử P(k) đúng, tức là:

```text
  1 + 2 + … + k = k(k+1)/2
```

*Đi tới đích.* Cần tới được P(k+1), tức là cần vế trái của bậc k+1 bằng
(k+1)(k+2)/2 — vì công thức ở bậc k+1 là n(n+1)/2 với n thay bằng k+1.

Bắt đầu từ vế trái của bậc k+1, và dùng đúng chỗ gợn đã tìm ra ở đầu bài: nó là
vế trái của bậc k, cộng thêm số hạng k+1.

```text
  1 + 2 + … + k + (k+1)
= ( 1 + 2 + … + k ) + (k+1)
= k(k+1)/2 + (k+1)              ← thay bằng giả thiết
```

Giờ gom (k+1) ra ngoài làm nhân tử chung — đúng phép gom mà bài 23 và bài 24 đã
dùng:

```text
  k(k+1)/2 + (k+1)
= (k+1) × ( k/2 + 1 )
= (k+1) × ( (k+2)/2 )           ← vì 1 = 2/2, nên k/2 + 1 = (k+2)/2
= (k+1)(k+2)/2
```

Đó đúng là vế phải của P(k+1). Vậy P(k) → P(k+1), cho mọi k.

Thử một con số cho chắc tay, ở k = 4: giả thiết cho vế trái bậc 4 bằng
4 × 5 / 2 = 10; cộng số hạng mới là 5 được 10 + 5 = 15; còn công thức ở bậc 5
cho 5 × 6 / 2 = 15. Khớp.

**Kết.** Hai việc xong, nên P(n) đúng với mọi số nguyên dương n.

Nhìn lại xem bạn vừa mua được gì bằng bao nhiêu. Bạn chốt được **vô hạn** mệnh
đề, trong đó có cả P(1000000) mà không ai trên đời từng cộng thử — bằng một dòng
kiểm và một đoạn biến đổi dài sáu dòng. Đó là chỗ đắt nhất của cả track.
::::

::::predict{#doan-hai-viec commitOnce}
Byte bắt máy làm hộ hai việc ấy ở dạng nhỏ nhất: kiểm cơ sở, và kiểm mối dính
giữa bậc 4 với bậc 5.

Máy vẫn giữ đúng vai của bài 21 — nó kiểm vài bậc cụ thể, nó không chứng minh gì.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
# Tổng thật: cộng dồn từng số một, đúng vòng `while` của T1.2.
def tong_that(n):
    tong = 0
    i = 1
    while i <= n:
        tong = tong + i
        i = i + 1
    return tong

# Việc thứ nhất, ở đúng bậc 1: vế trái có bằng vế phải không?
print(tong_that(1) == 1 * 2 // 2)

# Mối dính: mượn TỔNG ở bậc 4 rồi cộng thêm số hạng mới.
print(tong_that(4) + 5)

# Công thức ở bậc 5.
print(5 * 6 // 2)
```

:::opt{correct}
`True`, rồi `15`, rồi `15`
:::

:::opt
`True`, rồi `14`, rồi `15`
::why
Gần đúng ở hai dòng ngoài, và ở chỗ bạn nhớ đúng rằng bậc k+1 chỉ hơn bậc k một
số hạng — đó chính là ý cả bài đứng trên.

Chỗ lệch là **số hạng ấy bằng mấy**. Đi từ bậc 4 sang bậc 5 thì mẩu thêm vào
không phải số 4 mà là số 5: tổng bậc 4 là 1 + 2 + 3 + 4, còn tổng bậc 5 là
1 + 2 + 3 + 4 + 5. Nói chung, mẩu thêm vào khi đi từ bậc k sang bậc k+1 chính là
k+1. Nhớ nhầm chỗ này là bước quy nạp trượt ngay dòng đầu.
::
:::

:::opt
`True`, rồi `15`, rồi `30`
::why
Gần đúng ở hai dòng đầu, và ở chỗ bạn đọc đúng phần `5 * 6` của công thức.

Chỗ lệch là phép chia đôi ở cuối bị bỏ quên. Công thức là n(n+1)/2, và số 2 ấy
không phải trang trí. Viết tổng ra **hai lượt**, một lượt xuôi và một lượt ngược,
rồi cộng theo từng cột:

```text
   1   2   3   4   5
   5   4   3   2   1
  ───────────────────
   6   6   6   6   6
```

Năm cột, cột nào cũng ra 6, nên 5 × 6 = 30 chính là **hai lần** cái tổng cần tìm.
Chia đôi mới ra tổng thật. Máy làm đúng phép ấy bằng `// 2`, nên nó in ra 15 chứ
không phải 30.
::
:::

:::opt
`False`, rồi `15`, rồi `15`
::why
Gần đúng ở hai dòng sau, và ở một dè dặt rất hợp lý: một câu "với mọi n" thì
không thể được xác nhận bằng một bậc duy nhất, nên dòng đầu không được phép ra
`True`.

Chỗ lệch: dòng ấy **không** hỏi về mọi n. Nó hỏi đúng một câu — bậc 1 — và ở bậc
1 thì vế trái là 1, vế phải cũng là 1, nên nó ra `True`. Cơ sở vốn chỉ là một
mệnh đề, và chính chỗ rẻ ấy là điều làm nguyên lý quy nạp dùng được: nếu cơ sở
cũng đòi vô hạn công thì chẳng đổi được gì.
::
:::
::::

::::code{#viet-hai-viec-thanh-may}
Giờ tự tay viết hai việc ấy thành máy, rồi thả máy dò xem có bậc nào lệch không.

Nhắc lại vai của Python ở track này, vì bài hôm nay là chỗ dễ quên nhất: máy
**không** chứng minh gì. Nó dò được tới bậc 60 rồi hết hơi, và bài 21 đã chốt
rằng dò 60 lần không phải chứng minh. Chỗ chứng minh là hai việc bạn vừa đọc.

Ba chỗ trống:

1. **`cong_thuc`** — vế phải của câu, viết theo `n`. Nhớ phép chia đôi, và dùng
   `//` để kết quả là số nguyên chứ không phải số lẻ.
2. **`so_hang_moi`** — mẩu mà bậc k+1 có thêm so với bậc k.
3. **`ve_trai_buoc`** — vế trái của bước quy nạp: tổng tới bậc k+1, tính bằng
   cách **mượn công thức ở bậc k** rồi cộng mẩu mới. Dòng này là bước quy nạp
   viết thành máy, nên đừng gọi `tong_that` ở đây — gọi `tong_that` là đi cộng
   lại từ đầu, tức là không mượn gì cả.

Bài chấm bằng nhiều bậc, và bằng chính đẳng thức của bước quy nạp:
`ve_trai_buoc(k)` phải bằng `cong_thuc(k + 1)` ở mọi k trong tầm dò. Lệch một
đơn vị ở bất kỳ chỗ trống nào thì đẳng thức ấy vỡ ngay.

```python title=starter
# Tổng thật: cộng dồn từng số một, đúng vòng `while` của T1.2.
def tong_that(n):
    tong = 0
    i = 1
    while i <= n:
        tong = tong + i
        i = i + 1
    return tong

# CÔNG THỨC — vế phải của câu, viết theo n.
def cong_thuc(n):
    return ___

# Mẩu mà bậc k+1 có thêm so với bậc k.
def so_hang_moi(k):
    return ___

# Bước quy nạp viết thành máy: MƯỢN công thức ở bậc k, cộng mẩu mới.
def ve_trai_buoc(k):
    return ___

# Máy chỉ dò tới `den`. Nó nói "chưa thấy sai", không nói "đúng" — bài 21.
def chua_thay_sai(den):
    return all([tong_that(n) == cong_thuc(n) for n in range(1, den + 1)])

print(cong_thuc(1))
print(cong_thuc(10))
print(so_hang_moi(9))
print(ve_trai_buoc(9))
print(chua_thay_sai(60))
```

```python title=solution
# Tổng thật: cộng dồn từng số một, đúng vòng `while` của T1.2.
def tong_that(n):
    tong = 0
    i = 1
    while i <= n:
        tong = tong + i
        i = i + 1
    return tong

# CÔNG THỨC — vế phải của câu, viết theo n.
def cong_thuc(n):
    return n * (n + 1) // 2

# Mẩu mà bậc k+1 có thêm so với bậc k.
def so_hang_moi(k):
    return k + 1

# Bước quy nạp viết thành máy: MƯỢN công thức ở bậc k, cộng mẩu mới.
def ve_trai_buoc(k):
    return cong_thuc(k) + so_hang_moi(k)

# Máy chỉ dò tới `den`. Nó nói "chưa thấy sai", không nói "đúng" — bài 21.
def chua_thay_sai(den):
    return all([tong_that(n) == cong_thuc(n) for n in range(1, den + 1)])

print(cong_thuc(1))
print(cong_thuc(10))
print(so_hang_moi(9))
print(ve_trai_buoc(9))
print(chua_thay_sai(60))
```

```python title=test
assert cong_thuc(1) == 1, "cơ sở: ở bậc 1, vế phải phải ra đúng 1, bằng vế trái là chính số 1"
assert cong_thuc(2) == 3, "ở bậc 2, tổng 1 cộng 2 bằng 3, nên công thức cũng phải ra 3"
assert cong_thuc(3) == 6, "ở bậc 3, tổng 1 cộng 2 cộng 3 bằng 6"
assert cong_thuc(4) == 10, "ở bậc 4, tổng 1 cộng 2 cộng 3 cộng 4 bằng 10"
assert cong_thuc(5) == 15, "ở bậc 5, tổng năm số đầu bằng 15"
assert cong_thuc(10) == 55, "ở bậc 10, tổng mười số đầu bằng 55"
assert cong_thuc(100) == 5050, "ở bậc 100, tổng một trăm số đầu bằng 5050"
assert so_hang_moi(1) == 2, "đi từ bậc 1 sang bậc 2 thì mẩu thêm vào là chính số 2"
assert so_hang_moi(4) == 5, "đi từ bậc 4 sang bậc 5 thì mẩu thêm vào là chính số 5"
assert so_hang_moi(9) == 10, "đi từ bậc 9 sang bậc 10 thì mẩu thêm vào là chính số 10"
assert so_hang_moi(99) == 100, "đi từ bậc 99 sang bậc 100 thì mẩu thêm vào là chính số 100"
assert ve_trai_buoc(1) == 3, "mượn công thức ở bậc 1 được 1, cộng mẩu mới là 2, ra 3"
assert ve_trai_buoc(4) == 15, "mượn công thức ở bậc 4 được 10, cộng mẩu mới là 5, ra 15"
assert ve_trai_buoc(9) == 55, "mượn công thức ở bậc 9 được 45, cộng mẩu mới là 10, ra 55"
assert all([ve_trai_buoc(k) == cong_thuc(k + 1) for k in range(1, 200)]), "bước quy nạp: mượn bậc k rồi cộng mẩu mới thì phải ra ĐÚNG công thức ở bậc k+1, ở mọi k từ 1 tới 199"
assert all([tong_that(n) == cong_thuc(n) for n in range(1, 200)]), "công thức phải khớp tổng thật ở mọi bậc từ 1 tới 199 — máy chỉ dò được tới đó, và dò không phải chứng minh"
assert chua_thay_sai(60) == True, "trong 60 bậc đầu, máy không được thấy bậc nào lệch"
```

:::hints
- kind: attention
  body: Chỗ trống đầu chép lại đúng vế phải đã viết trong phần chứng minh, chỉ đổi cách gõ: chữ n đứng cạnh một ngoặc trong toán nghĩa là NHÂN, và gạch phân số nghĩa là chia — mà ở đây chia cho 2 luôn ra số nguyên nên dùng `//`. Chỗ trống thứ hai đã có câu trả lời nằm trong chính chú thích trên nó. Chỗ trống thứ ba ghép hai hàm vừa viết lại, đúng theo dòng "vế trái bậc k, cộng thêm số hạng k+1" trong đoạn biến đổi.
- kind: strategy
  body: "Viết `cong_thuc` bằng đúng ba mảnh của n(n+1)/2: `n`, rồi `(n + 1)`, rồi chia đôi. Viết `so_hang_moi(k)` bằng đúng cái mẩu mà bảng ở đầu bài chỉ ra — từ bậc 4 sang bậc 5 thì mẩu là 5, tức là k cộng 1. Rồi `ve_trai_buoc(k)` chỉ việc gọi hai hàm ấy và cộng lại; đừng cộng dồn lại từ đầu, vì cả điểm của bước quy nạp là được phép mượn kết quả ở bậc k."
- kind: one-line
  body: "Ba chỗ lần lượt là `n * (n + 1) // 2`; rồi `k + 1`; rồi `cong_thuc(k) + so_hang_moi(k)`."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: công thức phải TÍNH ra từ `n`, mẩu mới phải tính ra từ `k`, và bước quy nạp phải MƯỢN hai hàm ấy chứ không cộng lại từ đầu — gõ cứng một con số thì bài đang chấm một thứ nó không hề kiểm
  requireAst:
  # Khung khởi đầu không có dấu nhân nào. Công thức có đúng một.
  - kind: uses-operator, target: '*', min: 1
  # Chia đôi phải là phép chia lấy nguyên. Khung khởi đầu không có `//` nào.
  - kind: uses-operator, target: '//', min: 1
  # Khung khởi đầu gọi `cong_thuc` 3 lần (hai lệnh `print` và một lần trong
  # `chua_thay_sai`). Lời giải gọi 4 — lần thứ tư nằm trong `ve_trai_buoc`,
  # đúng chỗ bước quy nạp mượn bậc k. Ai cộng dồn lại từ đầu thì hụt con số này.
  - kind: uses-call, target: cong_thuc, min: 4
  # Khung khởi đầu gọi `so_hang_moi` 1 lần (trong `print`). Lời giải gọi 2.
  - kind: uses-call, target: so_hang_moi, min: 2
  # `k` phải được ĐỌC ở hai chỗ trống sau. Khung khởi đầu đọc `k` 0 lần.
  - kind: uses-name, target: k, min: 3
  forbidAst:
  # Lưới thứ hai, chặn đúng những con số là KẾT QUẢ. Mọi cách viết hợp lệ đều
  # dựng từ `n`, `k`, số 1 và số 2.
  - kind: has-literal, target: 55
  - kind: has-literal, target: 5050
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: ^1\n55\n10\n55\nTrue\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai việc, và cả hàng đổ. Kể cả quân thứ một triệu mà không ai chạm tới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đọc lại việc thứ hai, chậm thôi:

> Với mọi k: **giả sử P(k) đúng**, rồi suy ra P(k+1) đúng.

Có một chỗ nghe rất chướng ở đó, và nếu bạn thấy chướng thì bạn đang đọc đúng.

Điều cần chứng minh là *P(n) đúng với mọi n* — tức là gồm cả P(k). Mà việc thứ
hai lại mở đầu bằng "giả sử P(k) đúng". Tức là giả sử chính cái mình đang muốn
chứng minh? Nghe như đi vòng tròn.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
