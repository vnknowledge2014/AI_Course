---
id: toan.logic-va-chung-minh.chung-minh-cau-phan-dao-thay
title: Chứng minh câu phản đảo thay cho câu gốc
summary: Câu gốc và phản đảo của nó trùng khít một bảng, nên chúng thay được cho nhau ở mọi chỗ — kể cả ở chỗ bạn đang cầm bút chứng minh.
locale: vi
track: toan
module: logic-va-chung-minh
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.proof-by-contrapositive]
requires: [logic.direct-proof, logic.use-definition, logic.contrapositive, logic.converse, logic.inverse, logic.equivalence, logic.implication, logic.vacuous-truth, logic.negation, logic.truth-value, logic.counterexample, math.expand-brackets, math.factor-common, math.multiplication, math.multiply-commutative, math.exponent, math.remainder, core.boolean, core.variable, core.number-literal, core.arithmetic, core.modulo, core.function-def, core.function-call, core.function-parameter, core.function-return, core.none, core.list, core.list-comprehension, core.list-index, core.len, core.print-variable, ctrl.if, ctrl.for-range, ctrl.comparison, logic.not, logic.or]
concepts: [logic.chung-minh-phan-dao, logic.doi-cau-can-chung-minh, logic.dung-nham-cau-dao]
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

::::byte{trigger=enter mood=dizzy pose=lean-in}
Mình nhìn dòng `n² = 2k` cả buổi. Nó không chịu nhả ra chữ `n` nào cả.
::::

::::explain{#cho-tac-co-hinh-dang}
Bài trước tắc, và chỗ tắc có một hình dạng rất rõ.

Câu cần chứng minh: *cho `n` là số nguyên; nếu `n²` chẵn thì `n` chẵn.*

Nhịp 1 chạy được: `n²` chẵn nên có số nguyên `k` sao cho `n² = 2k`. Rồi hết
đường. Giả thiết nói về `n²`, kết luận đòi nói về `n`, và định nghĩa của bài 22
chỉ mở được chữ "chẵn" thành `2k` — nó không có cửa nào đi ngược từ `n²` về `n`.

Vấn đề không phải bạn nghĩ chưa đủ lâu. Vấn đề là **câu này bắt đầu ở đầu sai**.

Và bài 14 đã cất sẵn cho bạn cái chìa, từ mười bài trước.

Bài 14 dựng bảng cho câu gốc `P → Q` và cho phản đảo `¬Q → ¬P`, rồi thấy hai cột
**trùng khít nhau**, dòng nào cũng trùng. Bài 8 nói tiếp: hai câu trùng bảng thì
thay thế nhau ở **bất kỳ chỗ nào** mà không đổi giá trị.

"Bất kỳ chỗ nào" gồm cả chỗ bạn đang cầm bút. Nên:

> **Chứng minh xong câu phản đảo là chứng minh xong câu gốc.** Không phải "gần
> như", không phải "đủ dùng" — là xong hẳn, vì hai câu ấy có cùng một cột.

Viết phản đảo của câu đang tắc ra xem nó là câu gì:

| | câu |
|---|---|
| gốc | nếu `n²` chẵn thì `n` chẵn |
| phản đảo | nếu `n` **không** chẵn thì `n²` **không** chẵn |

Với số nguyên, "không chẵn" đọc gọn được thành "lẻ". Bài 22 viết "chẵn" thành
"phần dư khi chia cho 2 bằng 0", mà phần dư ấy chỉ có hai giá trị là 0 hoặc 1 —
không có cửa thứ ba, đúng tính lưỡng trị của bài 2. Nên "không chẵn" và "lẻ" là
một, và phản đảo đọc gọn thành:

> Nếu `n` lẻ thì `n²` lẻ.

Giờ so hai câu bằng một tiêu chí duy nhất — **bắt đầu ở đâu**:

- Câu gốc bắt đầu ở `n²`, chỗ bạn không cầm được.
- Phản đảo bắt đầu ở `n`, chỗ định nghĩa mở ra được ngay lập tức.

Đó là toàn bộ lý do đổi. Không phải mẹo, mà là chọn đầu nào để nắm.
::::

::::example{#chung-minh-cau-lat}
Chứng minh câu phản đảo, bằng đúng khuôn ba nhịp của bài 23. Không có công cụ
nào mới ở đây cả.

**Nhịp 1 — đặt tên, mở định nghĩa ra.**

Cho `n` là một số nguyên bất kỳ, và giả sử `n` lẻ. Theo bài 22, có một số nguyên
`k` sao cho `n = 2k + 1`.

**Nhịp 2 — biến đổi, chỉ bằng luật đã có.**

`n²` là `n × n`, mà cả hai thừa số đều là `n`, nên thay dạng vào cả hai:

```text
n² = (2k + 1) × (2k + 1)
```

Giờ mở ngoặc bên phải, coi cả cụm `(2k + 1)` bên trái như "một số" — đúng cái
luật T2.2 đã dựng bằng khay bánh: nhân một số vào một tổng thì nhân vào **từng**
cụm.

```text
(2k + 1) × (2k + 1) = (2k + 1) × 2k + (2k + 1) × 1
```

Mở tiếp từng cụm một, cũng bằng đúng luật ấy (T2.1 cho phép đổi chỗ hai thừa số,
nên số nhân đứng bên nào cũng được):

```text
(2k + 1) × 2k = 2k × 2k + 1 × 2k = 4k² + 2k
(2k + 1) × 1  = 2k + 1
```

Cộng hai mẩu lại:

```text
n² = 4k² + 2k + 2k + 1 = 4k² + 4k + 1
```

Còn một bước nữa: gom số 2 ra ngoài ở hai cụm đầu — luật phân phối đọc ngược,
đúng phép gom mà bài 23 vừa dùng. Vì `4k²` là `2 × 2k²` và `4k` là `2 × 2k`:

```text
4k² + 4k + 1 = 2 × (2k² + 2k) + 1
```

**Nhịp 3 — gói định nghĩa lại.**

Đặt `m = 2k² + 2k`. Vì `k` là số nguyên nên `2k² + 2k` là số nguyên, nên `m` là
số nguyên. Vậy

```text
n² = 2m + 1,  với m là số nguyên
```

và đó đúng là dạng mà chiều gói lại của bài 22 nhận cho chữ "lẻ". Nên `n²` lẻ.

Thử một con số cho chắc tay: k = 3 cho n = 7, và 7 × 7 = 49; mà m ở đây là
2 × 9 + 2 × 3 = 24, và 2 × 24 + 1 = 49. Khớp.

**Và thế là xong câu gốc.** Ta chưa viết dòng nào bắt đầu bằng "giả sử `n²`
chẵn", mà câu *"nếu `n²` chẵn thì `n` chẵn"* đã được chứng minh — vì nó và câu
vừa chứng minh có cùng một cột trong bảng của bài 14.

Cách viết ấy có tên:

> **Chứng minh bằng phản đảo** — thay câu cần chứng minh bằng phản đảo của nó,
> chứng minh câu thay thế bằng lối trực tiếp, và dừng lại ở đó. Quyền thay thế
> đến từ bài 14 (trùng bảng) cộng bài 8 (trùng bảng thì thay được cho nhau).
::::

::::explain{#dung-nham-voi-cau-dao}
Có một chỗ va rất dễ, và nó tốn kém, nên phải nói cho rõ.

Phản đảo là **đổi chỗ hai vế rồi phủ định cả hai** — làm cả hai việc. Làm mỗi
một việc thì ra câu khác:

- đổi chỗ thôi → **mệnh đề đảo** (bài 12);
- phủ định thôi → **mệnh đề phản** (bài 13).

Và bài 13 đã chốt: bảng của đảo trùng bảng của phản, chứ **không** trùng bảng
của câu gốc. Nên chứng minh câu đảo là chứng minh một câu khác hẳn.

Ở câu về `n²` thì chỗ hỏng này khuất mắt, vì đảo của nó — *nếu `n` chẵn thì `n²`
chẵn* — tình cờ cũng là một câu đúng. Muốn nhìn cho rõ thì phải lấy một bộ ba
khác. Bảng nội quy của CLB có một dòng về số ghế:

> Nếu số ghế chia hết cho 4 thì số ghế chẵn.

| | câu | đúng hay sai |
|---|---|---|
| gốc | nếu `n` chia hết cho 4 thì `n` chẵn | đúng |
| đảo | nếu `n` chẵn thì `n` chia hết cho 4 | **sai** |
| phản đảo | nếu `n` không chẵn thì `n` không chia hết cho 4 | đúng |

Câu đảo sai, và bác bỏ nó tốn đúng một phản ví dụ như bài 19 đã dạy: `n = 2`
chẵn mà không chia hết cho 4. (`n = 6` cũng làm được việc ấy — nhưng một cái là
đủ, và tìm thêm không làm câu sai hơn.)

Ba câu, hai giá trị. Gốc và phản đảo luôn đi cùng nhau; đảo thì đi đường riêng.
Đó là lý do bài 12, 13, 14 phải là ba bài chứ không phải một bảng đối chiếu để
thuộc lòng.
::::

::::predict{#doan-ba-cot commitOnce}
Byte bắt máy dựng cả ba cột cùng lúc, trên 100 số đầu tiên.

Hàm `keo_theo` là câu kéo theo của bài 10 viết thành máy: chỉ sai đúng ở dòng vế
trước đúng mà vế sau sai. Và `all` là chữ "và" kéo dài của bài 17 — nó hỏi "cột
này có Sai ở dòng nào không".

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def keo_theo(p, q):
    return (not p) or q

so = list(range(0, 100))

goc = all([keo_theo(n % 4 == 0, n % 2 == 0) for n in so])
dao = all([keo_theo(n % 2 == 0, n % 4 == 0) for n in so])
phan_dao = all([keo_theo(n % 2 != 0, n % 4 != 0) for n in so])

print(goc)
print(dao)
print(phan_dao)
```

:::opt{correct}
`True`, rồi `False`, rồi `True`
:::

:::opt
`True`, rồi `True`, rồi `True`
::why
Gần đúng ở hai dòng ngoài, và ở chỗ bạn tin rằng ba câu ấy "nói cùng một
chuyện" — chúng đúng là dựng từ cùng hai vế, chỉ khác cách xếp.

Chỗ lệch: bài 12 đã dựng riêng một bài để chỉ ra rằng **đổi chỗ hai vế cho ra
một câu khác**, và giá trị của câu mới có thể khác hẳn. Ở đây khác thật. Lấy
n = 2: nó chẵn nên vế trước của câu đảo đúng, mà 2 không chia hết cho 4 nên vế
sau sai — đúng cái dòng duy nhất làm câu kéo theo sai. Một phản ví dụ là đủ để
`all` trả `False`, đúng như bài 19 chốt.
::
:::

:::opt
`True`, rồi `False`, rồi `False`
::why
Gần đúng ở hai dòng đầu, và ở lối nghĩ đằng sau: phản đảo động vào câu gốc
**hai** lần — vừa đổi chỗ vừa phủ định — nên nó phải lệch đi xa hơn câu đảo chứ.

Chỗ lệch nằm đúng ở chữ "hai lần". Bài 12 cho thấy đổi chỗ thì hỏng, bài 13 cho
thấy phủ định cũng hỏng và hỏng y hệt kiểu ấy, còn bài 14 cho thấy làm **cả
hai** thì hai cái hỏng **bù nhau**: cột của phản đảo trùng khít cột của câu gốc,
dòng nào cũng trùng. Nên câu gốc đúng thì phản đảo đúng — không có cửa nào khác.
::
:::

:::opt
`False`, rồi `False`, rồi `False`
::why
Gần đúng ở chỗ bạn để ý rằng trong 100 số ấy, phần lớn số **không** chia hết cho
4 — và bạn đọc mỗi dòng như thế là một dòng "câu không nói đúng về nó".

Chỗ lệch là bài 11. Một câu "nếu… thì" có vế trước sai thì **cả câu đúng**, vì
không ai vi phạm được: nội quy nói về số chia hết cho 4, còn n = 3 thì nó không
nói tới, mà không nói tới thì không phá được. Nên mọi dòng có vế trước sai đều
ghi Đ vào cột, không phải ghi S.

Bỏ luật ấy đi thì không câu "nếu… thì" nào trên đời còn đúng nổi, kể cả câu gốc
— và đó là dấu hiệu cách đọc đang lệch chứ không phải ba câu cùng hỏng.
::
:::
::::

::::code{#dung-ba-cau-va-di-san}
Giờ tự tay dựng ba câu ấy, rồi thả máy đi tìm dòng đầu tiên mà mỗi cột ghi Sai.

Vai của máy vẫn không đổi từ bài 21: nó **không** chứng minh gì. Nó chỉ dò 100
dòng và chỉ ra chỗ hỏng nếu có. Chỗ chứng minh là ba nhịp bạn vừa đọc.

Bốn chỗ trống, hai việc:

1. **`m_theo_k`** — dòng cốt lõi của chứng minh. Từ `n = 2k + 1` bạn đã gom được
   `n² = 2 × (2k² + 2k) + 1`; hàm này nhận `k` và trả về đúng con số `m` nằm
   trong dạng `2m + 1`.
2. **`cau_dao` và `cau_phan_dao`** — hai câu họ hàng của `cau_goc` đã viết sẵn.
   Đảo là **đổi chỗ** hai vế. Phản đảo là đổi chỗ **rồi** phủ định cả hai, và
   phép phủ định đã có sẵn thành hàm `khong` ở ngay trên.

Một luật của bài: **viết phủ định bằng hàm `khong`**, đừng tự đổi `== 0` thành
`!= 0`. Hai cách cho cùng một giá trị, nhưng cả bài này nói về *hình dạng* của
ba câu, và chỉ có cách viết bằng `khong` mới đọc ra được "đây là câu gốc đã bị
phủ định hai vế".

Bài chấm bằng cả ba cột và bằng dòng cốt lõi ở nhiều giá trị `k`. Chép nhầm câu
này sang chỗ câu kia thì một trong ba cột sai ngay.

```python title=starter
# Bài 10 dựng câu kéo theo, bài 3 dựng phủ định. Hai hàm dưới chỉ là hai câu ấy
# viết thành máy, để dùng lại cho gọn.
def keo_theo(p, q):
    return (not p) or q

def khong(p):
    return not p

# Dòng cốt lõi: n = 2k + 1  ⟹  n² = 2 × (2k² + 2k) + 1.
def m_theo_k(k):
    return ___

# Câu gốc, viết sẵn: "nếu n chia hết cho 4 thì n chẵn".
def cau_goc(n):
    return keo_theo(n % 4 == 0, n % 2 == 0)

# Mệnh đề ĐẢO: đổi chỗ hai vế của câu gốc (bài 12).
def cau_dao(n):
    return keo_theo(___, ___)

# Mệnh đề PHẢN ĐẢO: đổi chỗ hai vế RỒI phủ định cả hai (bài 14).
def cau_phan_dao(n):
    return keo_theo(___, ___)

def san(cot):
    for n in range(0, len(cot)):
        if not cot[n]:
            return n
    return None

so = list(range(0, 100))

print(m_theo_k(3))
print(san([cau_goc(n) for n in so]))
print(san([cau_dao(n) for n in so]))
print(san([cau_phan_dao(n) for n in so]))
```

```python title=solution
# Bài 10 dựng câu kéo theo, bài 3 dựng phủ định. Hai hàm dưới chỉ là hai câu ấy
# viết thành máy, để dùng lại cho gọn.
def keo_theo(p, q):
    return (not p) or q

def khong(p):
    return not p

# Dòng cốt lõi: n = 2k + 1  ⟹  n² = 2 × (2k² + 2k) + 1.
def m_theo_k(k):
    return 2 * k * k + 2 * k

# Câu gốc, viết sẵn: "nếu n chia hết cho 4 thì n chẵn".
def cau_goc(n):
    return keo_theo(n % 4 == 0, n % 2 == 0)

# Mệnh đề ĐẢO: đổi chỗ hai vế của câu gốc (bài 12).
def cau_dao(n):
    return keo_theo(n % 2 == 0, n % 4 == 0)

# Mệnh đề PHẢN ĐẢO: đổi chỗ hai vế RỒI phủ định cả hai (bài 14).
def cau_phan_dao(n):
    return keo_theo(khong(n % 2 == 0), khong(n % 4 == 0))

def san(cot):
    for n in range(0, len(cot)):
        if not cot[n]:
            return n
    return None

so = list(range(0, 100))

print(m_theo_k(3))
print(san([cau_goc(n) for n in so]))
print(san([cau_dao(n) for n in so]))
print(san([cau_phan_dao(n) for n in so]))
```

```python title=test
# Hai câu về CỘT đứng đầu, vì chúng canh đúng cái bẫy của bài: chép câu gốc
# sang ô phản đảo, hoặc chép phản đảo sang ô đảo. Xếp chúng sau một câu `==`
# nào đó thì bẫy có thể không bao giờ sập.
assert [cau_dao(n) for n in so] != [cau_goc(n) for n in so], "cột của câu đảo KHÔNG được trùng cột của câu gốc — trong 100 số này, n = 2 là chỗ hai cột ấy tách ra"
assert [cau_phan_dao(n) for n in so] == [cau_goc(n) for n in so], "cột của phản đảo phải trùng cột của câu gốc ở cả 100 dòng — đó chính là bảng bài 14 dựng, chạy dài ra"
assert 2 * m_theo_k(7) + 1 == 15 * 15, "k = 7 cho n = 15, nên dạng 2m + 1 phải ra đúng 15 nhân 15"
assert 2 * m_theo_k(4) + 1 == 9 * 9, "k = 4 cho n = 9, nên dạng 2m + 1 phải ra đúng 9 nhân 9"
assert m_theo_k(3) == 24, "k = 3 cho n = 7, và 7 nhân 7 bằng 49; viết 49 thành 2m + 1 thì m bằng 24"
assert m_theo_k(0) == 0, "k = 0 cho n = 1, và 1 nhân 1 bằng 1; viết 1 thành 2m + 1 thì m bằng 0"
assert m_theo_k(1) == 4, "k = 1 cho n = 3, và 3 nhân 3 bằng 9; viết 9 thành 2m + 1 thì m bằng 4"
assert m_theo_k(2) == 12, "k = 2 cho n = 5, và 5 nhân 5 bằng 25; viết 25 thành 2m + 1 thì m bằng 12"
assert cau_dao(2) == False, "câu đảo nói n chẵn thì n chia hết cho 4; ở n = 2 vế trước đúng mà vế sau sai, nên câu đảo sai ở đúng chỗ này"
assert cau_dao(4) == True, "ở n = 4 thì hai vế của câu đảo đều đúng, nên câu đảo đúng ở chỗ này"
assert cau_dao(3) == True, "ở n = 3 vế trước của câu đảo sai, mà câu kéo theo có vế trước sai thì đúng — bài 11"
assert cau_phan_dao(3) == True, "phản đảo nói n không chẵn thì n không chia hết cho 4; ở n = 3 hai vế đều đúng"
assert cau_phan_dao(2) == True, "ở n = 2 vế trước của phản đảo sai, nên câu vẫn đúng"
assert cau_phan_dao(4) == True, "ở n = 4 vế trước của phản đảo cũng sai, nên câu vẫn đúng"
```

:::hints
- kind: attention
  body: Chỗ trống đầu trả về một CON SỐ, và chú thích ngay trên hàm đã viết sẵn cụm nằm trong ngoặc của dạng `2m + 1` — con số cần trả về chính là cụm ấy, không kèm số 2 ngoài ngoặc và không kèm số 1. Ba chỗ còn lại đều nhìn thẳng vào thân của `cau_goc` ở ngay trên: đảo dùng đúng hai vế ấy, đổi chỗ; phản đảo dùng hai vế đã đổi chỗ, mỗi vế bọc thêm một lớp `khong`.
- kind: strategy
  body: "Trong `2 × (2k² + 2k) + 1`, chữ `m` là cả cụm trong ngoặc, và `k²` viết thành `k * k` như T2.2 đã quen. Với hai câu họ hàng: chép thân `cau_goc` xuống, đổi chỗ hai đối số của `keo_theo` là xong câu đảo; rồi từ câu đảo ấy, bọc mỗi đối số vào `khong(...)` là xong câu phản đảo. Làm theo đúng thứ tự ấy thì không phải nhớ gì cả."
- kind: one-line
  body: "Bốn chỗ lần lượt là `2 * k * k + 2 * k`; rồi `n % 2 == 0` và `n % 4 == 0` cho câu đảo; rồi `khong(n % 2 == 0)` và `khong(n % 4 == 0)` cho câu phản đảo."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: dòng cốt lõi phải viết theo chữ `k`, và phản đảo phải bọc cả hai vế bằng `khong` — hai câu chỉ khác nhau ở hình dạng ấy, nên gõ cứng một con số hay bỏ lớp phủ định đi thì bài đang chấm một thứ nó không hề kiểm
  requireAst:
  # Khung khởi đầu KHÔNG gọi `khong` lần nào (dòng `def khong(p)` là khai báo,
  # không phải lời gọi). Luật này một mình chặn mọi đáp án viết phản đảo mà
  # không phủ định vế nào — kể cả đáp án chép nguyên câu gốc sang, thứ mà không
  # cột nào bắt được vì gốc và phản đảo vốn trùng cột.
  - kind: uses-call, target: khong, min: 2
  # Dòng cốt lõi có ba phép nhân: `2 * k * k` là hai, `2 * k` là một. Khung
  # khởi đầu không có dấu nhân nào.
  - kind: uses-operator, target: '*', min: 3
  # Khung khởi đầu đã có 2 dấu `%` (trong `cau_goc`). Lời giải có 6 — hai cho
  # câu đảo, hai cho câu phản đảo. Gõ bừa vào ba ô câu thì con số 6 không đạt.
  - kind: uses-operator, target: '%', min: 6
  # Dòng cốt lõi phải ĐỌC `k` ba lần. Khung khởi đầu đọc `k` 0 lần.
  - kind: uses-name, target: k, min: 3
  forbidAst:
  # Lưới thứ hai, chặn đúng con số là KẾT QUẢ: 24 là giá trị của `m` tại k = 3.
  # Mọi cách viết hợp lệ đều dựng từ `k`, số 2 và số 1, nên không cách nào chứa
  # nguyên văn con số ấy.
  - kind: has-literal, target: 24
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: ^24\nNone\n2\nNone\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cột phản đảo trùng cột gốc từ dòng đầu tới dòng cuối. Nên chứng minh một cái là xong cả hai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa có ba lối đi cho một câu "nếu… thì": đi thẳng, và nếu đầu này không nắm
được thì lật sang phản đảo mà nắm đầu kia.

Nhưng cả hai lối ấy đều bắt đầu bằng cùng một việc: **nhặt lấy vế trước làm giả
thiết**. Giờ thử câu này:

> Không có phân số nào mà đem nhân với chính nó lại ra đúng 2.

Đọc lại nó một lượt và tìm chữ "nếu". Không có. Tìm vế trước. Cũng không có —
câu này không nối hai vế, nó chỉ nói một điều về **mọi** phân số cùng lúc.

Không có vế trước thì không có gì để đổi chỗ, nên phản đảo không dùng được ở
đây. Mà cũng chẳng có giả thiết nào để nhặt lên, nên nhịp 1 của bài 23 cũng
không biết bắt đầu từ đâu.

Vậy một câu như thế thì bắt đầu từ chỗ nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
