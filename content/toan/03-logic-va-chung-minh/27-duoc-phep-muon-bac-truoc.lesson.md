---
id: toan.logic-va-chung-minh.duoc-phep-muon-bac-truoc
title: Được phép mượn bậc ngay trước
summary: "Giả sử P(k) đúng" không phải một lời khẳng định — nó là vế trước của một câu kéo theo. Nên bước quy nạp không mượn gì của điều cần chứng minh, và dây chuyền không khép thành vòng.
locale: vi
track: toan
module: logic-va-chung-minh
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.induction-hypothesis]
requires: [logic.induction, logic.proof-by-contradiction, logic.proof-by-contrapositive, logic.direct-proof, logic.finite-check-not-proof, logic.implication, logic.vacuous-truth, logic.truth-table, logic.truth-value, logic.for-all, logic.counterexample, logic.open-sentence, logic.negation, logic.not, logic.or, math.multiplication, math.letter-names-a-slot, core.boolean, core.variable, core.number-literal, core.arithmetic, core.floor-division, core.accumulator, core.reassign, core.list, core.list-comprehension, core.list-index, core.len, core.none, core.builtin-function, core.function-def, core.function-call, core.function-parameter, core.function-return, core.print-variable, ctrl.if, ctrl.while, ctrl.for-range, ctrl.comparison]
concepts: [logic.gia-thiet-quy-nap, logic.khong-vong-tron, logic.cay-cau-giua-hai-bac]
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
Mình làm xong bài trước rồi. Mình vẫn chưa tin bài mình vừa làm.
::::

::::explain{#noi-ro-cho-chuong}
Chỗ chướng ở cuối bài trước đáng được viết ra thật gọn, vì viết gọn xong là gần
gỡ xong:

> Muốn chứng minh: P(n) đúng với **mọi** n — trong đó có P(k).
> Mà việc thứ hai lại mở đầu bằng: **giả sử P(k) đúng**.
> Vậy chẳng phải ta đang dùng chính thứ mình đi tìm để tìm nó?

Đây không phải một hiểu nhầm cần dẹp cho nhanh. Đó là một nghi ngờ đúng chỗ, và
người nào không thấy nó thì đang đọc lướt. Nên gỡ cho hết.

Gỡ bằng cách hỏi lại một câu: **việc thứ hai chứng minh xong thì bạn cầm được
câu gì?**

Không phải câu "P(k) đúng". Câu bạn cầm được là:

> **nếu** P(k) đúng **thì** P(k+1) đúng.

Đó là một câu kéo theo, đúng thứ bài 10 dựng bằng bảng nội quy CLB. Và bài 10 đã
nói rõ một chuyện mà ở đây đáng giá cả bài: **một câu kéo theo không khẳng định
vế trước của nó.** Nội quy nói "thành viên thì đeo thẻ" — nó không nói ai là
thành viên. Nói "trời mưa thì sân ướt" không phải nói trời đang mưa.

Nên chữ "giả sử P(k) đúng" trong bước quy nạp không phải một lời khẳng định về
P(k). Nó là **cách gọi vế trước** của câu kéo theo bạn đang chứng minh. Nó có
tên riêng, và đây là khái niệm của cả bài hôm nay:

> **Giả thiết quy nạp** là vế trước của câu kéo theo trong bước quy nạp. Bạn
> được phép dùng nó **bên trong** phần chứng minh câu kéo theo ấy, và chỉ ở đó.
> Bước ra ngoài, bạn không sở hữu P(k) nào cả — bạn sở hữu một cây cầu.

Bài 11 đóng đinh chuyện này chặt hơn nữa: một câu kéo theo có **vế trước sai**
thì cả câu vẫn **đúng**. Nên hoàn toàn có thể xảy ra chuyện bạn chứng minh xong
bước quy nạp cho mọi k, mà chẳng bậc nào trong dãy đúng cả — mọi cây cầu đều
lành, mà không ai đứng trên bậc nào. Chứng minh xong bước quy nạp là chứng minh
xong **những cây cầu**, không phải chứng minh xong **những bậc**.
::::

::::example{#cay-cau-va-hai-dau-cua-no}
Nhìn tận mắt một cây cầu lành mà hai đầu đều sập.

Đặt Q(n) là câu: *1 + 2 + … + n nhỏ hơn 100.*

Vài bậc đầu tiên:

| bậc n | tổng | Q(n) |
|---|---|---|
| 12 | 78 | Đ |
| 13 | 91 | Đ |
| 14 | 105 | S |
| 15 | 120 | S |
| 16 | 136 | S |

(Kiểm nhanh bằng công thức của bài 26 — 13 × 14 / 2 = 91, rồi 14 × 15 / 2 = 105,
rồi 15 × 16 / 2 = 120, đúng ba dòng cuối của bảng.)

Giờ đọc ba cây cầu bằng bảng của bài 10 — cầu chỉ gãy đúng ở dòng "vế trước Đ,
vế sau S":

| cây cầu | vế trước | vế sau | câu kéo theo |
|---|---|---|---|
| Q(12) → Q(13) | Đ | Đ | **Đ** |
| Q(13) → Q(14) | Đ | S | **S** — gãy |
| Q(14) → Q(15) | S | S | **Đ** |

Cây cầu thứ ba là chỗ đáng nhìn lâu. Nó **đúng**, dù cả hai đầu của nó đều sai.
Đó là bài 11 nói lại một lần nữa: vế trước sai thì không ai phá được câu, nên
câu đúng — và một câu đúng kiểu ấy chẳng cho bạn cái gì.

Vậy nếu ai đó chứng minh được "Q(k) → Q(k+1) với mọi k", họ vẫn không được phép
kết luận Q(n) đúng với mọi n. Ở đây họ thậm chí không chứng minh được điều đó,
vì cây cầu thứ hai gãy thật. Nhưng chuyện quan trọng là: **có cầu không có nghĩa
là có bậc.**

Đó chính là câu trả lời cho nghi ngờ vòng tròn. Bước quy nạp không lấy trộm gì
của kết luận, bởi vì nó không kết luận gì về P(k) hết.
::::

::::explain{#so-muon-khong-khep-thanh-vong}
Còn một nửa của nghi ngờ chưa gỡ: nếu bậc k+1 mượn bậc k, mà bậc k lại mượn bậc
k−1, và cứ thế… thì có chắc chuỗi mượn ấy không quay về chỗ cũ không?

Mở sổ mượn ra xem. Với nguyên lý của bài 26:

```text
  bậc 1  mượn:  không ai        ← cơ sở, chứng minh thẳng
  bậc 2  mượn:  bậc 1
  bậc 3  mượn:  bậc 2
  bậc 4  mượn:  bậc 3
  …
  bậc n  mượn:  bậc n − 1
```

Hai dòng đầu là chỗ chịu lực. **Bậc 1 không mượn ai** — nó được chứng minh thẳng
bằng lối trực tiếp của bài 23, không giả thiết quy nạp nào tham gia. Và mọi bậc
từ 2 trở lên chỉ mượn một bậc **thấp hơn** nó.

Một vòng tròn thật thì trông thế này: *"A đúng vì B đúng, và B đúng vì A đúng."*
Ở đó không có chỗ nào bắt đầu — bỏ cả hai đi thì lập luận vẫn tự khớp, mà giữ cả
hai lại thì cũng chẳng ai chứng minh gì.

Sổ mượn ở trên không có hình dạng ấy. Muốn tới bậc 500, bạn đi 499 nhịp, và mỗi
nhịp mượn một bậc đã trả xong trước đó. Chuỗi mượn đi **một chiều xuống dưới**,
và nó dừng ở bậc 1 — chỗ chẳng mượn ai. Một chuỗi có điểm dừng thì không phải
một vòng.

> Không vòng tròn, vì hai lẽ đứng cùng nhau: **bậc đầu chẳng mượn ai**, và **mỗi
> bậc sau chỉ mượn bậc đã đổ.**

Bỏ một trong hai lẽ ấy đi thì lập luận hỏng — và bài sau sẽ cho thấy bỏ lẽ thứ
nhất thì hỏng tới mức nào.
::::

::::predict{#doan-cay-cau commitOnce}
Byte bắt máy đọc đúng ba cây cầu vừa dựng trong bảng.

Hàm `keo_theo` là câu kéo theo của bài 10 viết thành máy: chỉ sai đúng ở dòng vế
trước đúng mà vế sau sai.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def keo_theo(p, q):
    return (not p) or q

def tong_that(n):
    tong = 0
    i = 1
    while i <= n:
        tong = tong + i
        i = i + 1
    return tong

# Câu Q(n): "1 + 2 + … + n nhỏ hơn 100".
def duoi_tram(n):
    return tong_that(n) < 100

print(duoi_tram(13))
print(duoi_tram(14))
print(keo_theo(duoi_tram(13), duoi_tram(14)))
print(keo_theo(duoi_tram(14), duoi_tram(15)))
```

:::opt{correct}
`True`, rồi `False`, rồi `False`, rồi `True`
:::

:::opt
`True`, rồi `False`, rồi `False`, rồi `False`
::why
Gần đúng ở ba dòng đầu, và ở một lối nghĩ mà tiếng Việt đời thường ủng hộ: hai
đầu cầu đều sập thì cây cầu còn đứng làm gì nữa.

Chỗ lệch là bài 11. Câu kéo theo chỉ bị phá ở đúng một dòng — vế trước đúng mà
vế sau sai. Ở cây cầu thứ tư, vế trước là `duoi_tram(14)` và nó sai, nên không
ai phá được câu, nên câu đúng. Nó đúng theo kiểu chẳng nói gì cả, và đó chính là
điều bài hôm nay cần bạn nhìn thấy: **có cầu không có nghĩa là có bậc.**
::
:::

:::opt
`True`, rồi `True`, rồi `True`, rồi `True`
::why
Gần đúng ở dòng đầu, và ở cảm giác rằng 100 là một con số khá lớn nên tổng vài
chục số đầu vẫn còn nằm dưới nó.

Chỗ lệch nằm ở tốc độ của tổng. Công thức bài 26 cho biết ngay: ở bậc 13 tổng là
13 × 14 / 2 = 91, còn ở bậc 14 tổng đã là 14 × 15 / 2 = 105. Cộng thêm đúng một
số hạng mà vượt hẳn 100, nên `duoi_tram(14)` là `False`. Và vì vế trước đúng còn
vế sau sai, cây cầu thứ ba gãy.
::
:::

:::opt
`True`, rồi `False`, rồi `True`, rồi `True`
::why
Gần đúng ở hai dòng đầu, và ở một suy nghĩ hợp lý: bước quy nạp của bài 26 đã
chứng minh cầu nào cũng lành, nên hai dòng cuối phải cùng ra `True`.

Chỗ lệch: bài 26 chứng minh cầu lành cho **công thức tổng**, không phải cho câu
"tổng nhỏ hơn 100". Mỗi câu có bộ cầu riêng của nó, và ở đây máy không chứng minh
gì cả — nó chỉ đọc hai giá trị Đ/S rồi tra bảng bài 10. Vế trước đúng, vế sau sai
thì cầu gãy; cây cầu thứ ba rơi đúng vào dòng đó.
::
:::
::::

::::code{#dung-so-muon-thanh-may}
Giờ tự tay dựng cây cầu và cuốn sổ mượn, rồi thả máy đi dọc dây chuyền xem nó đổ
tới bậc nào.

Máy làm việc trên một **cột** Đ/S theo bậc — đúng cái cột mà bài 6 tới bài 9 đã
dựng bằng tay, chỉ dài hơn: `cot[0]` là bậc 1, `cot[1]` là bậc 2, và cứ thế.

Bốn chỗ trống, ba việc:

1. **`cau`** — cây cầu từ bậc k sang bậc k+1, đọc trên một cột cho sẵn. Nó là
   một câu **kéo theo**, không phải một lời khẳng định về bậc k. Nhớ rằng bậc k
   nằm ở ô `cot[k - 1]`, vì ô đầu tiên đánh số 0.
2. **`muon_cua`** — bậc n mượn bậc nào. Bậc 1 là cơ sở, chứng minh thẳng, nên nó
   không mượn ai; trong Python, "không có gì cả" viết là `None`.
3. **`do_toi_bac_may`** — đi từ cơ sở lên, mỗi lượt qua đúng một cây cầu, và
   dừng ngay khi gặp cầu gãy. Chỗ trống nằm ở đối số thứ hai của `cau`: đang
   đứng ở bậc nào thì bước lên cây cầu xuất phát từ bậc ấy.

Bài chấm bằng hai cột. `cot_dung` là cột của công thức bài 26 — đúng ở mọi bậc.
`cot_duoi_100` là cột của câu "tổng nhỏ hơn 100" — đúng tới bậc 13 rồi sai từ bậc
14, nên nó có đúng một cây cầu gãy, và cây cầu ngay sau chỗ gãy thì **lành mà vô
dụng**. Cột thứ hai có mặt để canh đúng chỗ ấy: viết cầu ngược chiều hay lệch một
ô thì nó báo ngay, còn cột thứ nhất thì không, vì ở đó cách viết nào cũng ra Đ.

```python title=starter
# Bài 10 dựng câu kéo theo. Hàm dưới là câu ấy viết thành máy.
def keo_theo(p, q):
    return (not p) or q

def tong_that(n):
    tong = 0
    i = 1
    while i <= n:
        tong = tong + i
        i = i + 1
    return tong

# Hai CỘT Đ/S theo bậc: cot[0] là bậc 1, cot[1] là bậc 2, ...
cot_dung = [tong_that(n) == n * (n + 1) // 2 for n in range(1, 21)]
cot_duoi_100 = [tong_that(n) < 100 for n in range(1, 21)]

# Cây cầu từ bậc k sang bậc k+1, đọc trên một cột cho sẵn.
def cau(cot, k):
    return keo_theo(___, ___)

# Bậc n mượn bậc nào? Bậc 1 là cơ sở nên không mượn ai.
def muon_cua(n):
    if n == 1:
        return ___
    return ___

# Đi từ cơ sở lên, mỗi lượt qua đúng một cây cầu. Trả về bậc cao nhất đổ được.
def do_toi_bac_may(cot):
    if not cot[0]:
        return 0
    da_do = 1
    while da_do < len(cot) and cau(cot, ___):
        da_do = da_do + 1
    return da_do

print(muon_cua(1))
print(muon_cua(9))
print(do_toi_bac_may(cot_dung))
print(do_toi_bac_may(cot_duoi_100))
```

```python title=solution
# Bài 10 dựng câu kéo theo. Hàm dưới là câu ấy viết thành máy.
def keo_theo(p, q):
    return (not p) or q

def tong_that(n):
    tong = 0
    i = 1
    while i <= n:
        tong = tong + i
        i = i + 1
    return tong

# Hai CỘT Đ/S theo bậc: cot[0] là bậc 1, cot[1] là bậc 2, ...
cot_dung = [tong_that(n) == n * (n + 1) // 2 for n in range(1, 21)]
cot_duoi_100 = [tong_that(n) < 100 for n in range(1, 21)]

# Cây cầu từ bậc k sang bậc k+1, đọc trên một cột cho sẵn.
def cau(cot, k):
    return keo_theo(cot[k - 1], cot[k])

# Bậc n mượn bậc nào? Bậc 1 là cơ sở nên không mượn ai.
def muon_cua(n):
    if n == 1:
        return None
    return n - 1

# Đi từ cơ sở lên, mỗi lượt qua đúng một cây cầu. Trả về bậc cao nhất đổ được.
def do_toi_bac_may(cot):
    if not cot[0]:
        return 0
    da_do = 1
    while da_do < len(cot) and cau(cot, da_do):
        da_do = da_do + 1
    return da_do

print(muon_cua(1))
print(muon_cua(9))
print(do_toi_bac_may(cot_dung))
print(do_toi_bac_may(cot_duoi_100))
```

```python title=test
assert cau(cot_dung, 1) == True, "trên cột của công thức, bậc 1 và bậc 2 đều đúng, nên cây cầu giữa chúng đúng"
assert cau(cot_dung, 19) == True, "trên cột của công thức, bậc 19 và bậc 20 đều đúng, nên cây cầu giữa chúng đúng"
assert cau(cot_duoi_100, 12) == True, "ở cột 'tổng nhỏ hơn 100', bậc 12 và bậc 13 đều đúng, nên cây cầu giữa chúng đúng"
assert cau(cot_duoi_100, 13) == False, "bậc 13 đúng vì tổng là 91, còn bậc 14 sai vì tổng đã là 105 — vế trước đúng mà vế sau sai thì cây cầu gãy"
assert cau(cot_duoi_100, 14) == True, "bậc 14 và bậc 15 đều sai, mà câu kéo theo có vế trước sai thì vẫn đúng — bài 11"
assert cau(cot_duoi_100, 15) == True, "bậc 15 và bậc 16 cũng đều sai, nên cây cầu giữa chúng vẫn đúng theo kiểu chẳng nói gì"
assert muon_cua(1) is None, "bậc 1 là cơ sở, được chứng minh thẳng, nên nó không mượn bậc nào"
assert muon_cua(2) == 1, "bậc 2 mượn bậc 1"
assert muon_cua(9) == 8, "bậc 9 mượn bậc 8"
assert muon_cua(500) == 499, "bậc 500 mượn bậc 499"
assert all([muon_cua(n) < n for n in range(2, 300)]), "mọi bậc từ 2 tới 299 chỉ được mượn một bậc THẤP hơn chính nó — đó là chỗ chuỗi mượn không khép lại thành vòng"
assert do_toi_bac_may(cot_dung) == 20, "trên cột của công thức không cây cầu nào gãy, nên đi tới bậc 20 là hết cột"
assert do_toi_bac_may(cot_duoi_100) == 13, "cột 'tổng nhỏ hơn 100' đúng tới bậc 13 rồi sai từ bậc 14, nên dây chuyền dừng lại ở bậc 13"
assert do_toi_bac_may([True, True, True]) == 3, "cột ba bậc đều đúng thì hai cây cầu đều lành, nên đổ hết cả ba bậc"
assert do_toi_bac_may([True, False, True]) == 1, "cột này đúng ở bậc 1 nhưng cây cầu từ bậc 1 sang bậc 2 gãy, nên chỉ đổ được tới bậc 1"
assert do_toi_bac_may([False, True, True]) == 0, "cột này sai ngay ở ô đầu, nên hàm dừng trước cả lượt đi đầu tiên và trả về 0"
```

:::hints
- kind: attention
  body: Ở `cau`, hai đối số của `keo_theo` là hai ô liền nhau của cùng một cột, và thứ tự của chúng là thứ tự đi lên — bậc k trước, bậc k+1 sau. Chú thích ngay trên cột đã nói bậc 1 nằm ở ô số 0, nên bậc k nằm ở ô số mấy? Ở `muon_cua`, nhánh đầu trả về thứ mà Python dùng để nói "không có gì", còn nhánh sau trả về bậc liền dưới. Ở `do_toi_bac_may`, biến `da_do` đang giữ bậc bạn đang đứng, và cây cầu cần bước lên là cây xuất phát từ chính bậc ấy.
- kind: strategy
  body: "Với `cau`: ô của bậc k là `cot[k - 1]`, nên ô của bậc k+1 là `cot[k]`. Đặt chúng vào `keo_theo` theo đúng chiều đi lên, đừng đổi chỗ — đổi chỗ là ra mệnh đề đảo của bài 12, một câu khác hẳn. Với `muon_cua`: `None` cho bậc 1, và `n - 1` cho mọi bậc còn lại. Với `do_toi_bac_may`: truyền chính `da_do` vào `cau`, vì lượt này bước từ bậc `da_do` lên bậc liền trên."
- kind: one-line
  body: "Bốn chỗ lần lượt là `cot[k - 1]` và `cot[k]`; rồi `None`; rồi `n - 1`; rồi `da_do`."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cây cầu phải đọc HAI ô liền nhau của cột theo chỉ số tính từ `k`, và sổ mượn phải tính ra từ `n` — gõ cứng một chỉ số hay một con số thì bài đang chấm một thứ nó không hề kiểm
  requireAst:
  # Khung khởi đầu không có dấu trừ nào. Lời giải có hai: `k - 1` và `n - 1`.
  - kind: uses-operator, target: '-', min: 2
  # `k` phải được ĐỌC hai lần, ở hai ô của cây cầu. Khung khởi đầu đọc 0 lần.
  - kind: uses-name, target: k, min: 2
  # Khung khởi đầu đọc `cot` 3 lần, tất cả nằm trong `do_toi_bac_may`. Lời giải
  # đọc 5 — hai lần thêm là hai ô của cây cầu.
  - kind: uses-name, target: cot, min: 5
  # `da_do` phải được truyền vào `cau`. Khung khởi đầu đọc nó 3 lần, lời giải 4.
  - kind: uses-name, target: da_do, min: 4
  # `n` phải được đọc trong `muon_cua`. Khung khởi đầu đọc `n` 5 lần (ở hai cột
  # dựng sẵn và ở điều kiện vòng `while`); lời giải đọc 6.
  - kind: uses-name, target: n, min: 6
  forbidAst:
  # Lưới thứ hai, chặn đúng những con số là KẾT QUẢ chứ không phải nguyên liệu:
  # 13 là chỗ dây chuyền dừng, 8 là bậc mà bậc 9 mượn.
  - kind: has-literal, target: 13
  - kind: has-literal, target: 8
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: ^None\n8\n20\n13\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sổ mượn đi một chiều xuống dưới rồi dừng ở bậc 1. Không có vòng nào cả — mình tin bài mình làm rồi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa thấy bước quy nạp không mượn gì của kết luận. Nên thử dùng nó thật thoải
mái xem sao. Lấy câu R(n): *n = n + 1.*

Chứng minh bước quy nạp cho câu ấy:

```text
  giả sử  R(k):   k = k + 1
  cộng 1 vào hai vế:  k + 1 = k + 2
  mà đó đúng là  R(k+1).
```

Không dòng nào sai. Phép cộng cùng một số vào hai vế của một đẳng thức là phép
T2.2 đã dựng bằng cái cân, và nó hợp lệ. Nên bước quy nạp của câu "n = n + 1"
**đúng**, cho mọi k.

Vậy mọi số đều bằng số liền sau nó?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
