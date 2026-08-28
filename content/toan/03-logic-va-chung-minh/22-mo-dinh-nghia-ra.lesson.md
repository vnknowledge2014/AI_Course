---
id: toan.logic-va-chung-minh.mo-dinh-nghia-ra
title: Mở định nghĩa ra
summary: Chữ "chẵn" không tính toán được, nhưng dạng n = 2k thì có — và vì định nghĩa là một câu hai chiều nên bạn được đi cả chiều mở chữ ra lẫn chiều gói dạng lại.
locale: vi
track: toan
module: logic-va-chung-minh
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [logic.use-definition]
requires: [logic.finite-check-not-proof, logic.biconditional, logic.exists, logic.for-all, logic.open-sentence, logic.counterexample, logic.vacuous-truth, math.remainder, math.multiplication, math.letter-names-a-slot, core.boolean, core.variable, core.number-literal, core.arithmetic, core.modulo, core.floor-division, core.function-def, core.function-call, core.function-parameter, core.function-return, core.none, core.list, core.print-variable, ctrl.if, ctrl.for-range, ctrl.comparison]
concepts: [logic.mo-dinh-nghia, logic.goi-dinh-nghia, logic.nhan-chung-k]
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
Mình muốn nói một câu đúng cho mọi số chẵn — mà không thử con số nào. Nghe như đòi hỏi quá đáng.
::::

::::explain{#cho-tac-bai-truoc}
Bài trước đóng một cánh cửa.

Máy thử `n × n + n + 41` với n = 0, rồi 1, rồi 2… đúng 40 lần liền không sai lần
nào, rồi ở n = 40 thì hỏng. Kết luận rút ra không phải "máy tồi", mà là: **máy
chỉ nói được về những trường hợp nó đã xét**. Trên một danh sách 6 thành viên,
xét hết là xong. Trên số tự nhiên, xét hết là chuyện không bao giờ xong.

Nên nếu muốn nói *"tổng hai số chẵn luôn là số chẵn"* — một câu "với mọi" chạy
trên vô hạn số — thì phải thôi làm việc với từng con số, và bắt đầu làm việc
với **chữ "chẵn"**.

Chỗ khó nằm ngay đó. Tới lúc này, "n chẵn" mới chỉ là một **câu mở** của bài 16:
một câu có chỗ trống, chưa đúng chưa sai, và cách duy nhất bạn biết để trả lời
nó là điền một số cụ thể vào rồi đem chia. Mà điền từng số rồi chia thì lại rơi
đúng vào chỗ bài 21 vừa đóng.

Câu hỏi của cả bài hôm nay gọn thế này: **viết chữ "chẵn" ra thành cái gì mà
tính toán được, mà không phải đi hỏi từng con số một?**
::::

::::example{#xep-cap-dau}
CLB cờ vua lớp 6A có sáu thành viên: **Nam, Lan, Minh, Hoa, Tú, Khanh**. Chiều
thứ Sáu thầy phụ trách xếp cặp đấu — mỗi bàn đúng hai người.

```text
  bàn 1: Nam  – Lan
  bàn 2: Minh – Hoa
  bàn 3: Tú   – Khanh
```

Ba bàn, không ai ngồi không. Nếu CLB nhận thêm một bạn nữa thành bảy người, vẫn
ba bàn — và một người ngồi ngoài xem.

"Xếp hết thành cặp, không dư ai" chính là chữ "chẵn" nói bằng hiện vật. Và hiện
vật ấy viết thẳng ra được thành một câu tính:

> Số nguyên **n chẵn** **khi và chỉ khi** **có** một số nguyên `k` sao cho
> **n = 2k**.

Con số `k` ở đây không phải chữ trang trí — nó là **số bàn**. Sáu thành viên thì
k = 3, và câu tính đọc ra là 2 × 3 = 6.

Bài 15 đã chốt: một câu "khi và chỉ khi" khẳng định cả hai chiều cùng lúc, nên
mọi định nghĩa dùng được theo **cả hai hướng**. Đặt tên cho hai hướng ấy, vì cả
track còn lại sống bằng chúng:

- **Mở định nghĩa ra** — đi từ chữ sang dạng tính được. Biết "n chẵn" thì viết
  ngay ra "n = 2k, với `k` là một số nguyên nào đó". Bạn vừa đổi một chữ lấy một
  **câu tính**.
- **Gói định nghĩa lại** — đi ngược, từ dạng tính được sang chữ. Nhìn thấy một
  số viết được thành 2 nhân một số nguyên thì gọi thẳng tên nó là chẵn, khỏi
  chia. Chẳng hạn 2 × 17 = 34, nên 34 chẵn — bạn không phải đem 34 ra chia cho 2
  lần nào.
::::

::::explain{#hai-chi-tiet-dat-tien}
Hai chi tiết trong định nghĩa ấy nhỏ mà chịu lực.

**Chi tiết thứ nhất: chữ "có".** Đó đúng là chữ "tồn tại" của bài 18. Định nghĩa
không nói `k` bằng bao nhiêu; nó chỉ nói `k` **có mặt**. Nên "mở định nghĩa ra"
thực chất là: đặt cho cái `k` đang có mặt ấy một **cái tên**, rồi từ đó làm việc
với cái tên chứ không với con số. Chữ đặt tên cho một chỗ chưa biết là thứ T2.2
đã dựng suốt 36 bài; ở đây nó quay lại làm đúng việc cũ.

Và vì mỗi số chẵn có `k` của riêng nó — 6 có k = 3, còn 10 có k = 5 — nên cái
tên ấy **đi liền với số nào đang được nói tới**. Chuyện này sẽ đắt hơn nữa ở bài
sau.

**Chi tiết thứ hai: `k` phải là số nguyên.** Bỏ ba chữ "là số nguyên" đi thì
định nghĩa sập ngay: 2 × 3,5 = 7, nên số 7 cũng thành "chẵn", và cả bảy tỉ số
khác cũng thế. Một chữ mà số nào cũng mang thì chữ ấy không phân loại được gì
nữa. Ba chữ ấy chính là chỗ **bàn cờ không cắt đôi được**: không có nửa cái bàn,
nên không có nửa cặp đấu.

Cùng một cú, làm với chữ "lẻ":

> Số nguyên **n lẻ** **khi và chỉ khi** **có** một số nguyên `k` sao cho
> **n = 2k + 1**.

Đọc bằng hiện vật: `k` bàn đủ đôi, cộng đúng một người ngồi ngoài. Bảy thành
viên thì k = 3, và 2 × 3 + 1 = 7.

Còn con số 0? 0 = 2 × 0, mà 0 là số nguyên, nên **0 chẵn**. Không bàn nào, không
người nào, và cũng không ai ngồi ngoài — đúng cái cảm giác "chẳng có gì mà vẫn
đúng" mà bài 19 đã dựng bằng tổ trực nhật rỗng.

Một điều phải nói thẳng, kẻo nhầm: mở định nghĩa ra **chưa chứng minh được gì
cả**. Nó không phải một lập luận, nó là một phép đổi chữ lấy dạng. Nhưng nó đúng
là mẩu bài 21 còn thiếu: dạng `2k` viết theo chữ, không theo một con số, nên nó
nói về mọi số chẵn **cùng một lúc**.
::::

::::predict{#doan-tim-k commitOnce}
Byte viết chữ "tồn tại" của bài 18 ra thành máy, đúng từng chữ một: đi qua mọi
`k` từ 0 tới n, xem có `k` nào làm `n = 2k` không. Rồi đặt nó cạnh phép chia dư
mà bạn đã quen.

Nhắc lại: `range(0, n + 1)` đi qua các số nguyên từ 0 tới n, chặn trên hở nên n
vẫn được đi qua. Và `n % 2` là phần dư khi chia n cho 2.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def co_k_de_bang_hai_k(n):
    for k in range(0, n + 1):
        if n == 2 * k:
            return True
    return False

print(co_k_de_bang_hai_k(6), 6 % 2 == 0)
print(co_k_de_bang_hai_k(7), 7 % 2 == 0)
print(co_k_de_bang_hai_k(0), 0 % 2 == 0)
```

:::opt{correct}
`True True`, rồi `False False`, rồi `True True`
:::

:::opt
`True True`, rồi `False False`, rồi `False False`
::why
Gần đúng ở chỗ bạn đọc trúng hai dòng đầu, và ở chỗ bạn cẩn thận với số 0 — 0 là
con số hay làm hỏng mọi luật, nên dè chừng nó là phản xạ tốt.

Chỗ lệch: định nghĩa chỉ đòi **có một số nguyên `k`** để n = 2k, và với n = 0 thì
k = 0 có sẵn — 2 × 0 = 0. Số 0 là một số nguyên hợp lệ như mọi số nguyên khác.
Bên cột phải cũng thế: chia 0 cho 2 được 0 và không thừa gì, nên `0 % 2` bằng 0.

Đây là lần thứ hai trong track chỗ "chẳng có gì" làm trực giác trượt: bài 19 đã
gặp một lần với tổ trực nhật chưa xếp ai.
::
:::

:::opt
`True True`, rồi `False True`, rồi `True True`
::why
Gần đúng ở chỗ bạn nghĩ 7 vẫn "chia được cho 2" — và về mặt số học thì đúng,
7 chia 2 được 3,5. Nếu `k` được phép là 3,5 thì vòng lặp sẽ tìm thấy nó và cột
trái ra `True`.

Chỗ lệch nằm ở ba chữ mà định nghĩa không cho bỏ: `k` phải là **số nguyên**. Ở
đây `range` chỉ đi qua số nguyên, nên 3,5 không bao giờ được đem ra thử — nhưng
đó không phải hạn chế của máy, đó là định nghĩa đang được tôn trọng. Cho `k` lấy
giá trị lẻ thì mọi số đều thành chẵn, và chữ "chẵn" hết phân loại được gì.

Cột phải bạn đoán trúng: `7 % 2` bằng 1, khác 0, nên `False`.
::
:::

:::opt
`True True`, rồi `False False`, rồi `False True`
::why
Gần đúng ở hai dòng đầu, và ở chỗ bạn để ý tới `range` — đó là chỗ đáng để ý
thật, vì cả hàm này sống nhờ nó.

Chỗ lệch: với n = 0 thì `range(0, 0 + 1)` là `range(0, 1)`, và khoảng ấy **không
rỗng** — nó có đúng một số, là 0. Chặn dưới đóng, chặn trên hở: đi qua 0 rồi
dừng. Vòng chạy đúng một lượt, gặp `0 == 2 * 0`, và trả `True`.

Cột phải thì bạn đoán trúng, và đoán trúng vì đúng lý do: `0 % 2` bằng 0.
::
:::
::::

::::code{#hai-chieu-cua-dinh-nghia}
Giờ viết cả hai chiều của định nghĩa thành ba cái máy nhỏ, để đối chiếu chúng
trên những con số cụ thể.

Nói cho rõ máy đang làm gì ở đây, kẻo hiểu nhầm: nó **không** chứng minh gì hết
— bài 21 đã chặn đúng chuyện đó. Nó chỉ giữ hộ bạn cái hiện vật, để chiều "mở
ra" và chiều "gói lại" không trôi thành hai chuyện khác nhau trong đầu.

Ba chỗ trống, mỗi chỗ một chiều:

1. **`so_ban`** — chiều **mở ra**. Nhận n người, trả về số bàn `k` khi n chẵn, và
   `None` khi không xếp hết được thành cặp. Cần hai mẩu: câu hỏi "n có chẵn
   không", và con số `k` rút ra từ n.
2. **`so_nguoi_chan`** — chiều **gói lại**. Có `k` bàn, mỗi bàn hai người thì tất
   cả bao nhiêu người.
3. **`so_nguoi_le`** — cũng chiều gói lại, nhưng cho chữ "lẻ": `k` bàn đủ đôi
   cộng đúng một người ngồi ngoài.

Bài chấm bằng **nhiều con số khác nhau cho mỗi máy**, trong đó có cả 0 người và
0 bàn. Gõ cứng một con số vào thì con số bên cạnh sai ngay, nên phải viết ra
phép tính thật theo chữ `n` và chữ `k`.

```python title=starter
# Chiều MỞ RA: từ chữ "n chẵn" rút ra con số k.
def so_ban(n):
    if ___:
        return ___
    return None

# Chiều GÓI LẠI: có k bàn thì tất cả có mấy người.
def so_nguoi_chan(k):
    return ___

# Cũng chiều gói lại, cho chữ "lẻ": k bàn đủ đôi và một người ngồi ngoài.
def so_nguoi_le(k):
    return ___

print(so_ban(6), so_ban(7))
print(so_nguoi_chan(3), so_nguoi_le(3))
```

```python title=solution
# Chiều MỞ RA: từ chữ "n chẵn" rút ra con số k.
def so_ban(n):
    if n % 2 == 0:
        return n // 2
    return None

# Chiều GÓI LẠI: có k bàn thì tất cả có mấy người.
def so_nguoi_chan(k):
    return 2 * k

# Cũng chiều gói lại, cho chữ "lẻ": k bàn đủ đôi và một người ngồi ngoài.
def so_nguoi_le(k):
    return 2 * k + 1

print(so_ban(6), so_ban(7))
print(so_nguoi_chan(3), so_nguoi_le(3))
```

```python title=test
# Ba câu đầu canh ba cái bẫy lớn nhất của bài, nên chúng phải chạy TRƯỚC: nếu
# một câu `==` ở dưới trượt trước, ba bẫy này không bao giờ sập.
#
#   · bẫy 1 — trả bừa một con số cho mọi n: 7 phải cho `None`.
#   · bẫy 2 — gõ cứng số bàn: 10 người phải cho 5 bàn, không phải 3.
#   · bẫy 3 — quên mất người ngồi ngoài: hai máy gói lại không được trùng nhau.
assert so_ban(7) is None, "7 người thì xếp được 3 bàn và còn dư một người, nên 7 không viết được thành 2 nhân một số nguyên nào"
assert so_ban(10) == 5, "10 người xếp hết thành cặp thì được 5 bàn — số bàn phải ĐỔI theo n, không phải một con số đứng yên"
assert so_nguoi_chan(4) != so_nguoi_le(4), "cùng 4 bàn, một bên có người ngồi ngoài và một bên không, nên hai con số ấy không thể bằng nhau"
assert so_ban(6) == 3, "6 thành viên xếp hết thành cặp thì đúng 3 bàn, vì 2 × 3 = 6"
assert so_ban(0) == 0, "0 người thì 0 bàn, và 0 = 2 × 0 nên 0 vẫn là số chẵn"
assert so_ban(1) is None, "1 người thì không xếp nổi một cặp nào, nên không có số bàn để trả về"
assert so_nguoi_chan(3) == 6, "3 bàn, mỗi bàn hai người: 2 × 3 = 6"
assert so_nguoi_chan(5) == 10, "5 bàn thì 2 × 5 = 10 — số người phải đổi theo số bàn"
assert so_nguoi_chan(0) == 0, "0 bàn thì không có ai, và 0 đúng là 2 × 0"
assert so_nguoi_le(3) == 7, "3 bàn đủ đôi cộng một người ngồi ngoài: 2 × 3 + 1 = 7"
assert so_nguoi_le(0) == 1, "0 bàn mà vẫn có một người ngồi ngoài, nên tất cả là 2 × 0 + 1 = 1 người"
assert so_ban(so_nguoi_chan(4)) == 4, "gói 4 bàn lại thành 8 người rồi mở ra lại thì phải về đúng 4 bàn — hai chiều của một định nghĩa phải khớp nhau"
assert so_ban(so_nguoi_le(4)) is None, "9 người thì mở định nghĩa CHẴN ra không được, vì 9 lẻ"
```

:::hints
- kind: attention
  body: Chỗ trống đầu là một câu hỏi Đ/S về `n` — đọc lại ô định nghĩa trong bài: "n chẵn" nghĩa là n xếp hết thành cặp, tức chia cho 2 không còn thừa gì. Chỗ trống thứ hai trả về SỐ BÀN, mà mỗi bàn hai người. Hai chỗ trống cuối đi ngược chiều: chúng đã có `k` trong tay rồi và phải trả về số người.
- kind: strategy
  body: "Phần dư khi chia n cho 2 viết là `n % 2`; chia lấy phần nguyên viết là `n // 2` — cả hai bạn đã dùng từ T2.1. Hai máy gói lại thì chép thẳng từ hai ô định nghĩa: một cái là `2k`, cái kia là `2k + 1`, chỉ khác nhau đúng người ngồi ngoài. Đừng gõ sẵn con số: mỗi máy được chấm bằng nhiều giá trị khác nhau."
- kind: one-line
  body: "Bốn chỗ lần lượt là `n % 2 == 0`, `n // 2`, `2 * k` và `2 * k + 1`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cả bốn chỗ trống phải viết theo chữ `n` hoặc chữ `k` — gõ cứng một con số thì bài đang chấm một thứ nó không hề tính, và chiều "gói lại" của định nghĩa không được viết ra ở đâu cả
  requireAst:
  # Khung khởi đầu không có dấu `%` nào (chú thích không nằm trong cây cú
  # pháp). Luật này một mình chặn mọi đáp án không hỏi tới phần dư: `if True`,
  # `if n > 0`, `if n == 6`.
  - kind: uses-operator, target: '%', min: 1
  # Hai máy gói lại đều là "hai người một bàn", nên mỗi cái phải có một phép
  # nhân. Khung khởi đầu không có dấu nhân nào.
  - kind: uses-operator, target: '*', min: 2
  # Máy mở ra phải ĐỌC `n` (một lần trong câu hỏi, một lần trong con số trả
  # về). Khung khởi đầu đọc `n` 0 lần — ba dòng `def` là khai tham số, không
  # phải đọc.
  - kind: uses-name, target: n, min: 2
  # Hai máy gói lại phải ĐỌC `k`. Khung khởi đầu đọc `k` 0 lần.
  - kind: uses-name, target: k, min: 2
  forbidAst:
  # Lưới thứ hai, chặn đúng ba con số là KẾT QUẢ chứ không phải dữ liệu: 3 là
  # số bàn của 6 người, 5 là số bàn của 10 người, 8 là số người của 4 bàn. Mọi
  # cách viết hợp lệ đều dựng từ `n`, `k`, số 2 và số 1, nên không cách nào
  # chứa nguyên văn ba số dưới đây.
  - kind: has-literal, target: 5
  - kind: has-literal, target: 8
  - kind: has-literal, target: 9
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^3 None\n6 7\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chữ "chẵn" biến thành `2k` rồi. Giờ nó là một câu tính, mà câu tính thì cãi lại được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có dạng `n = 2k` trong tay, và bạn có quyền đi cả hai chiều của nó. Đem thử
ngay vào câu mà bài này mở đầu:

> *Tổng hai số chẵn bất kỳ là một số chẵn.*

Mở định nghĩa ra cho số thứ nhất: nó bằng `2k`. Mở cho số thứ hai: nó cũng
bằng… `2k`?

Chỗ ấy có gì đó không ổn, và bạn cảm thấy ngay: hai số chẵn khác nhau thì hai
cái `k` của chúng đâu có bằng nhau.

Mà rắc rối hơn thế. Đề bài nói **"hai số chẵn bất kỳ"** — vậy hai số ấy viết ra
là số nào? Viết 6 và 10 thì bạn chỉ vừa nói về 6 và 10, và bài 21 đã chặn đúng
lối đó. Không viết số nào thì lấy gì mà cộng.

"Bất kỳ" — làm sao đặt xuống giấy được một thứ như thế?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
