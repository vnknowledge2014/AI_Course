---
id: toan.logic-va-chung-minh.khong-co-bac-dau-thi-khong-do
title: Không có bậc đầu thì không đổ
summary: Bước quy nạp chỉ chuyển sự đúng từ bậc này sang bậc sau — nó không tạo ra sự đúng ở bậc nào cả, nên thiếu cơ sở thì cả dây chuyền không bao giờ bắt đầu.
locale: vi
track: toan
module: logic-va-chung-minh
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.induction-base-case]
requires: [logic.induction-hypothesis, logic.induction, logic.implication, logic.vacuous-truth, logic.direct-proof, logic.counterexample, logic.finite-check-not-proof, logic.negation, logic.truth-value, logic.for-all, math.multiplication, math.remainder, core.boolean, core.variable, core.reassign, core.number-literal, core.arithmetic, core.floor-division, core.function-def, core.function-call, core.function-parameter, core.function-return, core.accumulator, core.list, core.list-comprehension, core.builtin-function, core.print-variable, ctrl.while, ctrl.for-range, ctrl.comparison]
concepts: [logic.co-so-quy-nap, logic.buoc-chi-chuyen-tiep, logic.day-chuyen-khong-khoi-dong]
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
Mình kiểm đi kiểm lại cái bước ấy. Nó không sai chỗ nào. Mà kết luận thì sai bét.
::::

::::explain{#buoc-dung-ma-ket-luan-sai}
Bài trước để lại một câu khó chịu, và nó khó chịu vì nó đúng.

Câu đem ra doạ là: **mọi số tự nhiên đều bằng số liền sau nó** — tức `n = n + 1`
với mọi `n`. Gọi câu ở bậc thứ `n` là `P(n)`.

Giờ làm việc thứ hai của quy nạp, đúng khuôn bài 26 và 27: **giả sử `P(k)` đúng,
chứng minh `P(k + 1)` đúng.**

Giả sử `k = k + 1`. Cộng thêm 1 vào hai vế — phép cộng cùng một lượng vào hai
đĩa cân, T2.2 đã dựng. Vế trái thành `k + 1`, vế phải thành `k + 2`. Vậy
`k + 1 = k + 2`, mà đó chính là `P(k + 1)`.

Đọc lại đoạn ấy và tìm chỗ sai. Không có chỗ nào sai. Bước quy nạp của câu này
**đúng thật**, đúng ở mọi `k`, không sứt mẻ một chữ.

Vậy mà kết luận thì sai đến mức không cần cãi: 3 không bằng 4.

Chỗ hổng không nằm trong bước. Nó nằm ở chỗ ta chưa hề nhìn tới.
::::

::::explain{#khong-ai-day-quan-dau-tien}
Bài 26 dựng nguyên lý quy nạp bằng hàng domino, và nó nói **hai** việc:

1. **Cơ sở** — quân đầu tiên đổ.
2. **Bước** — hễ một quân đổ thì nó xô đổ quân ngay sau.

Việc thứ hai của câu `n = n + 1` làm xong rồi: mỗi quân đứng đủ gần quân sau để
xô đổ được nó. Còn việc thứ nhất thì chưa ai làm.

Thử làm xem. Cơ sở là `P(0)`, tức câu **0 = 1**. Câu ấy sai.

Thế là cả hàng domino đứng nguyên. Mỗi quân sẵn sàng xô đổ quân sau nó, và
không quân nào đổ, bởi vì không có ai đẩy quân đầu tiên.

Đây là chỗ đáng chép vào sổ:

> **Bước quy nạp là một câu kéo theo.** Nó nói: *nếu* bậc `k` đúng *thì* bậc
> `k + 1` đúng. Bài 11 đã chốt rằng một câu kéo theo có vế trước sai thì cả câu
> vẫn đúng — nên bước có thể đúng ở mọi bậc mà chẳng bậc nào thật sự đúng cả.
> Bước **chuyển** sự đúng đi; nó không **tạo** ra sự đúng ở đâu hết.
>
> Muốn có cái để chuyển, phải có một bậc đúng sẵn. Bậc ấy là **cơ sở**, và nó
> phải được chứng minh riêng, bằng cách kiểm thẳng.

Nói cách khác: hai việc của quy nạp không thay thế cho nhau được. Việc thứ hai
làm hoàn hảo cũng không gánh nổi việc thứ nhất, dù chỉ một nửa.
::::

::::example{#mot-cong-thuc-lech-di-nam-don-vi}
Câu `n = n + 1` dễ nghi vì nó vô lý ngay từ lúc đọc. Nên phải lấy một ca nữa,
ca mà cái sai không lộ ra mặt chữ.

Bài 25 và 26 đã làm việc với tổng `1 + 2 + ... + n`. Gọi tổng thật ấy là `S(n)`,
và đem ba công thức ra ứng cử:

| công thức | viết ra | ở bậc 1 nó cho | `S(1)` là |
|---|---|---|---|
| đúng | `n × (n + 1) / 2` | 1 | 1 |
| lệch | `n × (n + 1) / 2 + 5` | 6 | 1 |
| hỏng | `n × n` | 1 | 1 |

Cột thứ ba đã đủ nói: công thức **lệch** trượt cơ sở, hai công thức kia thì
không.

Bây giờ làm việc thứ hai cho công thức **lệch**, và làm cho tử tế.

Giả sử ở bậc `k` nó cho ra một số nào đó. Ở bậc `k + 1` nó cho ra số ấy cộng
thêm `k + 1` hay không? Viết ra:

```text
lệch(k + 1) − lệch(k) = [ (k+1) × (k+2) / 2 + 5 ] − [ k × (k+1) / 2 + 5 ]
```

Hai số 5 triệt tiêu nhau — cộng cùng một lượng vào hai vế thì hiệu không đổi.
Còn lại đúng cái hiệu của công thức đúng, mà công thức đúng thì nối được:

```text
(k+1) × (k+2) / 2 − k × (k+1) / 2 = k + 1
```

Vậy **bước của công thức lệch cũng đúng, ở mọi `k`**. Thử một con số cho chắc
tay: ở bậc 7 nó cho 33, ở bậc 8 nó cho 41, và 33 + 8 = 41. Khớp.

Đem hai việc xếp cạnh nhau:

| công thức | cơ sở (bậc 1) | bước (mọi `k`) | có phải `S(n)` không |
|---|---|---|---|
| đúng | đạt | đạt | **phải** |
| lệch | **trượt** | đạt | không |
| hỏng | đạt | **trượt** | không |

Ba dòng, ba câu chuyện khác nhau, và chúng nói cùng một điều: **thiếu một trong
hai việc là hỏng, thiếu việc nào cũng hỏng.**

Công thức lệch là ca đáng sợ hơn, vì nó lệch **đều** — lệch đúng 5 ở mọi bậc,
không bao giờ lệch nhiều hơn, không bao giờ tự sửa. Ở bậc 8 nó cho 41 còn tổng
thật là 36. Ai chỉ kiểm bước rồi tuyên bố đã chứng minh xong sẽ mang con số 41
ấy đi dùng.
::::

::::predict{#doan-cong-thuc-lech commitOnce}
Byte bắt máy hỏi công thức lệch bốn câu: hai câu về **bước**, hai câu về **cơ
sở**.

Hàm `tong_that` cộng dồn từng số một, đúng cách T1.2 đã dạy — nó là `S(n)` viết
thành máy.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def tong_that(n):
    tong = 0
    i = 1
    while i <= n:
        tong = tong + i
        i = i + 1
    return tong

def cong_thuc_lech(n):
    return n * (n + 1) // 2 + 5

print(cong_thuc_lech(8) == cong_thuc_lech(7) + 8)
print(cong_thuc_lech(30) == cong_thuc_lech(29) + 30)
print(cong_thuc_lech(1) == tong_that(1))
print(cong_thuc_lech(8) == tong_that(8))
```

:::opt{correct}
`True`, `True`, `False`, `False`
:::

:::opt
`True`, `True`, `True`, `True`
::why
Gần đúng ở hai dòng đầu, và ở một lối nghĩ rất tự nhiên: bạn vừa thấy bước của
công thức lệch nối được ở mọi bậc, nên bạn kết luận nó là tổng thật.

Chỗ lệch nằm đúng ở chỗ cả bài hôm nay đứng. Bước chỉ hứa *nếu bậc này đúng thì
bậc sau đúng*. Nó không hứa bậc nào đúng. Ở bậc 1, công thức lệch cho 6 còn
tổng thật là 1, nên hai dòng cuối trả `False`. Cái lệch 5 ấy đi theo suốt mọi
bậc mà bước chẳng bao giờ kêu lên tiếng nào.
::
:::

:::opt
`False`, `False`, `False`, `False`
::why
Gần đúng ở chỗ bạn tin rằng một công thức sai thì sai ở khắp nơi — và với công
thức này, nó sai ở khắp nơi thật: không bậc nào nó cho ra đúng tổng thật cả.

Chỗ lệch là hai dòng đầu hỏi một câu khác hẳn. Chúng không hỏi "công thức có
đúng không". Chúng hỏi "từ bậc này sang bậc sau, công thức có tự nối được
không" — và nó nối được, vì hai lần cộng thêm 5 triệt tiêu nhau khi lấy hiệu.
Một công thức sai vẫn có thể có bước hoàn hảo; đó chính là điều làm cơ sở trở
thành bắt buộc.
::
:::

:::opt
`True`, `True`, `False`, `True`
::why
Gần đúng ở chỗ bạn bắt trúng dòng thứ ba: ở bậc 1 công thức lệch cho 6 còn tổng
thật là 1, nên nó trả `False`. Bạn đã nhìn thấy chỗ cơ sở gãy.

Chỗ lệch là suy nghĩ rằng bậc càng cao thì cái lệch càng nhỏ đi so với con số
lớn, tới lúc nào đó thì hết lệch. Nó không hết. Hiệu giữa hai công thức luôn
đúng bằng 5, ở mọi bậc, vì cả hai cùng nối thêm đúng một lượng như nhau mỗi
bậc. Ở bậc 8 công thức lệch cho 41 còn tổng thật là 36, nên dòng cuối cũng
`False`.
::
:::
::::

::::code{#kiem-rieng-hai-viec}
Giờ viết **hai việc của quy nạp** thành hai hàm riêng, rồi chấm cả ba công thức
bằng cùng một cái thước.

- `co_so(f)` — công thức `f` có đúng ở **bậc đầu tiên**, tức bậc 1, không?
- `buoc(f, k)` — từ bậc `k` sang bậc `k + 1`, công thức `f` có **tự nối được**
  không? Nối được nghĩa là giá trị ở bậc `k + 1` đúng bằng giá trị ở bậc `k`
  cộng thêm `k + 1`, vì đó chính là số hạng mới của tổng.

Hàm `moi_buoc` đã viết sẵn: nó chạy `buoc` trên 50 bậc rồi thu về một giá trị
bằng chữ "và" kéo dài của bài 17. Nhớ bài 21: máy thử 50 bậc chỉ nói được
"chưa thấy sai" — chỗ chứng minh vẫn là mấy dòng bạn vừa đọc.

Một luật của bài: **cơ sở phải so với tổng thật, viết bằng `tong_that`**, đừng
gõ thẳng con số. Cả bài hôm nay nói về việc *kiểm* bậc đầu tiên chứ không phải
*nhớ* đáp số của nó, và hai chuyện ấy khác nhau đúng ở chỗ đó.

Bài chấm bằng cả ba công thức, và ba công thức được chọn để cư xử khác hẳn
nhau: một cái đạt cả hai việc, một cái trượt cơ sở mà đạt bước, một cái đạt cơ
sở mà trượt bước. Điền bừa một giá trị vào chỗ trống thì hỏng ngay một trong ba.

```python title=starter
# Tổng thật: 1 + 2 + ... + n, cộng dồn từng số một (T1.2.12).
def tong_that(n):
    tong = 0
    i = 1
    while i <= n:
        tong = tong + i
        i = i + 1
    return tong


# Ba công thức ứng cử. Chỉ một trong ba là tổng thật.
def cong_thuc_dung(n):
    return n * (n + 1) // 2


def cong_thuc_lech(n):
    return n * (n + 1) // 2 + 5


def cong_thuc_hong(n):
    return n * n


# VIỆC 1 — cơ sở: công thức có đúng ở bậc đầu tiên, tức bậc 1, không?
def co_so(f):
    return ___


# VIỆC 2 — bước: từ bậc k sang bậc k + 1, công thức có tự nối được không?
def buoc(f, k):
    return ___


# Chữ "và" kéo dài của bài 17, chạy trên 50 bậc đầu.
def moi_buoc(f):
    return all([buoc(f, k) for k in range(0, 50)])


print(co_so(cong_thuc_dung), moi_buoc(cong_thuc_dung))
print(co_so(cong_thuc_lech), moi_buoc(cong_thuc_lech))
print(co_so(cong_thuc_hong), moi_buoc(cong_thuc_hong))
```

```python title=solution
# Tổng thật: 1 + 2 + ... + n, cộng dồn từng số một (T1.2.12).
def tong_that(n):
    tong = 0
    i = 1
    while i <= n:
        tong = tong + i
        i = i + 1
    return tong


# Ba công thức ứng cử. Chỉ một trong ba là tổng thật.
def cong_thuc_dung(n):
    return n * (n + 1) // 2


def cong_thuc_lech(n):
    return n * (n + 1) // 2 + 5


def cong_thuc_hong(n):
    return n * n


# VIỆC 1 — cơ sở: công thức có đúng ở bậc đầu tiên, tức bậc 1, không?
def co_so(f):
    return f(1) == tong_that(1)


# VIỆC 2 — bước: từ bậc k sang bậc k + 1, công thức có tự nối được không?
def buoc(f, k):
    return f(k + 1) == f(k) + (k + 1)


# Chữ "và" kéo dài của bài 17, chạy trên 50 bậc đầu.
def moi_buoc(f):
    return all([buoc(f, k) for k in range(0, 50)])


print(co_so(cong_thuc_dung), moi_buoc(cong_thuc_dung))
print(co_so(cong_thuc_lech), moi_buoc(cong_thuc_lech))
print(co_so(cong_thuc_hong), moi_buoc(cong_thuc_hong))
```

```python title=test
# Ba câu về CƠ SỞ đứng trước, vì chúng canh đúng cái bẫy của bài: một cơ sở
# viết bừa vẫn cho ra cột `bước` đẹp đẽ, và người ta đọc cột đẹp ấy rồi tin cả
# bảng. Xếp chúng sau thì bẫy có thể không bao giờ sập.
assert co_so(cong_thuc_lech) == False, "công thức lệch cho 6 ở bậc 1, còn tổng thật ở bậc 1 là 1 — cơ sở của RIÊNG công thức này không đứng"
assert co_so(cong_thuc_dung) == True, "công thức đúng cho 1 ở bậc 1, khớp tổng thật ở bậc 1 — cơ sở của nó đứng"
assert co_so(cong_thuc_hong) == True, "công thức n nhân n cho 1 ở bậc 1, cũng khớp tổng thật ở bậc 1 — cơ sở của nó cũng đứng"
assert buoc(cong_thuc_lech, 7) == True, "ở k = 7: công thức lệch cho 33 ở bậc 7, và 33 cộng 8 đúng bằng 41 là giá trị của nó ở bậc 8"
assert moi_buoc(cong_thuc_lech) == True, "công thức lệch nối được ở cả 50 bậc máy vừa thử — bước của nó không gãy chỗ nào"
assert buoc(cong_thuc_hong, 1) == False, "ở k = 1: công thức n nhân n cho 4 ở bậc 2, mà 1 cộng 2 chỉ là 3 — bước gãy đúng chỗ này"
assert buoc(cong_thuc_hong, 0) == True, "ở k = 0: công thức n nhân n cho 1 ở bậc 1, và 0 cộng 1 cũng là 1 — một bậc trót lọt không cứu nổi cả dây chuyền"
assert moi_buoc(cong_thuc_dung) == True, "công thức đúng nối được ở cả 50 bậc máy vừa thử"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều trả về một giá trị Đúng/Sai, và mỗi chỗ là một câu so sánh bằng dấu `==`. Đọc lại hai gạch đầu dòng ngay trên khung mã: gạch thứ nhất nêu rõ cơ sở hỏi về bậc nào, gạch thứ hai nêu rõ "nối được" nghĩa là hai bên nào phải bằng nhau. Và để ý `f` là một HÀM được đưa vào, nên `f(1)` là giá trị của công thức ấy ở bậc 1.
- kind: strategy
  body: "Chỗ thứ nhất so hai số: giá trị của công thức ở bậc đầu tiên, và tổng thật ở đúng bậc ấy — hàm `tong_that` ở ngay trên tính hộ vế sau. Chỗ thứ hai cũng so hai số: giá trị của công thức ở bậc `k + 1`, và giá trị của chính nó ở bậc `k` cộng thêm số hạng mới. Số hạng mới của tổng khi đi từ bậc `k` sang bậc kế tiếp chính là `k + 1`, đúng con số vừa được thêm vào."
- kind: one-line
  body: "Chỗ thứ nhất là `f(1) == tong_that(1)`; chỗ thứ hai là `f(k + 1) == f(k) + (k + 1)`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cơ sở phải TÍNH LẠI tổng thật bằng `tong_that` rồi so, chứ không gõ sẵn con số; và bước phải viết theo `f` với `k` chứ không theo một công thức cụ thể nào — gõ cứng thì bài đang chấm một thứ nó không hề kiểm
  requireAst:
  # Khung khởi đầu KHÔNG gọi `tong_that` lần nào (dòng `def tong_that` là khai
  # báo, không phải lời gọi). Luật này một mình chặn đáp án `f(1) == 1`, thứ mà
  # cả ba công thức lẫn khối test đều không phân biệt nổi với lời giải thật.
  - kind: uses-call, target: tong_that, min: 1
  # `f` là hàm được đưa vào, và cả hai chỗ trống đều phải GỌI nó: một lần ở cơ
  # sở, hai lần ở bước. Khung khởi đầu gọi `f` 0 lần.
  - kind: uses-call, target: f, min: 3
  # Bước phải viết theo `k`, không theo một con số. Khung khởi đầu đọc `k` 1
  # lần (trong `moi_buoc`); lời giải đọc 4.
  - kind: uses-name, target: k, min: 4
  # Hai chỗ trống đều là một phép so sánh BẰNG. Khung khởi đầu có 0 dấu `==`.
  - kind: uses-operator, target: '==', min: 2
  forbidAst:
  # Lưới thứ hai, chặn hai con số là KẾT QUẢ chứ không phải nguyên liệu: 6 là
  # giá trị của công thức lệch ở bậc 1, và 36 là tổng thật ở bậc 8. Mọi cách
  # viết hợp lệ đều dựng từ `f`, `k`, `tong_that` và số 1, nên không cách nào
  # chứa nguyên văn hai số ấy.
  - kind: has-literal, target: 6
  - kind: has-literal, target: 36
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True True\nFalse True\nTrue False\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cột bước đẹp long lanh cả ba dòng lận. Mà cột cơ sở mới là cột nói thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn có đủ hai việc, và biết vì sao thiếu việc nào cũng hỏng. Đem bộ đôi ấy
ra dùng thử trên một câu mới:

> Mọi số nguyên lớn hơn hoặc bằng 2 đều viết được thành một tích các số nguyên
> tố.

Cơ sở dễ: 2 là số nguyên tố, nên nó đã là một tích gồm đúng một thừa số.

Bước thì tắc ngay. Muốn tách 12, cách tự nhiên nhất là viết 12 = 3 × 4, rồi
tách tiếp con 4. Nghĩa là bạn cần biết **4** tách được. Mà bước quy nạp chỉ cho
mượn đúng bậc liền trước, tức bậc 11 — và 11 là số nguyên tố, nó chẳng nói gì
về 4 cả.

Bậc được phép mượn thì vô dụng, bậc cần mượn thì không được phép. Thế thì tắc?

Bài sau gỡ đúng chỗ ấy.
::::

::::checkpoint{mastery=0.8}
::::
