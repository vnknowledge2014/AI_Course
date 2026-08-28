---
id: toan.logic-va-chung-minh.kiem-nghin-lan-van-chua-du
title: Kiểm nghìn lần vẫn chưa đủ
summary: Máy chỉ nói được về những trường hợp nó đã xét — nên một câu "với mọi số" đúng 40 lần liền vẫn có thể sai ở lần thứ 41, và đó là chuyện đã xảy ra thật.
locale: vi
track: toan
module: logic-va-chung-minh
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [logic.finite-check-not-proof]
requires: [logic.quantifier-negation, logic.counterexample, logic.for-all, logic.exists, logic.not, math.remainder, math.multiplication, core.boolean, core.list, core.list-append, core.function-def, core.function-call, core.function-parameter, core.function-return, core.return-multiple, core.none, core.variable, core.accumulator, core.arithmetic, core.modulo, core.number-literal, ctrl.for-each, ctrl.for-range, ctrl.while, ctrl.if, ctrl.comparison]
concepts: [logic.kiem-huu-han, logic.mien-vo-han, logic.may-noi-duoc-gi]
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
Mình thử 40 số rồi, không sai lần nào. Nhưng mình vẫn chưa dám viết chữ "đúng"
lên bảng.
::::

::::explain{#cho-dau-bai-truoc}
Bài trước kết ở một chỗ gợn.

Với sáu thành viên, chữ "với mọi" chạy được tới cùng: dựng sáu giá trị Đ/S, gọi
`all` một lần, và máy trả lời dứt điểm. Lý do nó dứt điểm được nằm ở chỗ ít ai
để ý — **danh sách sáu tên có dòng cuối cùng**. `all` đi tới Khanh là hết
đường, và khi đã xét hết thì không còn ai để bất ngờ.

Byte đem đúng cách ấy sang một câu nói về **số**:

> Với mọi số tự nhiên `n`, số `n × n + n + 41` là số nguyên tố.

Số nguyên tố là thứ bạn gặp ở T2.1: số từ 2 trở lên mà chỉ chia hết cho 1 và
cho chính nó. Còn "số tự nhiên" thì là 0, 1, 2, 3, … — và chỗ khác nhau nằm
đúng ở dấu ba chấm ấy. **Danh sách số tự nhiên không có dòng cuối cùng.**

Máy vẫn chạy được, tất nhiên. Nó chỉ không chạy được *hết*. Nó chạy tới chỗ
bạn bảo nó dừng.
::::

::::example{#bon-muoi-lan-khong-sai}
Byte gõ vài số đầu ra giấy trước khi nhờ máy:

| `n` | `n × n + n + 41` | có phải số nguyên tố? |
|---|---|---|
| 0 | 41 | có |
| 1 | 43 | có |
| 2 | 47 | có |
| 3 | 53 | có |
| 4 | 61 | có |
| 5 | 71 | có |

Kiểm lại một dòng cho chắc tay — dòng `n = 5`: 25 + 5 + 41 = 71, và 71 không
chia hết cho số nào ngoài 1 và 71.

Byte cho máy chạy tiếp. Tới `n = 39` — tức là **40 số đầu tiên**, vì số tự nhiên
bắt đầu từ 0 chứ không từ 1 — vẫn không sai lần nào. Dòng cuối cùng nó xét:
1521 + 39 + 41 = 1601, và 1601 là số nguyên tố.

Bốn mươi lần liền không sai. Với một cái bảng nội quy sáu người thì bốn mươi lần
là thừa thãi. Ở đây thì nó là bao nhiêu phần của "mọi số tự nhiên"?
::::

::::predict{#doan-dong-thu-ba commitOnce}
Byte viết ba dòng để hỏi máy. Hàm `la_nguyen_to` là hàm kiểm số nguyên tố quen
thuộc: nó thử chia cho từng số từ 2 đi lên, gặp một phép chia hết là kết luận
"không phải".

`range(40)` cho ra 40 số: 0, 1, 2, …, 39 — bắt đầu từ 0 và **dừng trước** 40,
đúng như T1.2 đã dựng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python
def la_nguyen_to(n):
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d = d + 1
    return True

def cong_thuc(n):
    return n * n + n + 41

bang = []
for n in range(40):
    bang.append(la_nguyen_to(cong_thuc(n)))

print(len(bang))
print(all(bang))
print(la_nguyen_to(cong_thuc(40)))
```

:::opt{correct}
`40`, rồi `True`, rồi `False`
:::

:::opt
`40`, rồi `True`, rồi `True`
::why
Hai dòng đầu bạn đọc trúng: danh sách có 40 ô, và cả 40 ô đều `True` nên chuỗi
"và" kéo dài cho `True`.

Chỗ lệch ở dòng ba, và nó là cả nội dung bài hôm nay. Bốn mươi lần đúng liên
tiếp là một chuỗi rất thuyết phục — không có gì sai trong việc thấy nó thuyết
phục. Nhưng thuyết phục không phải là bảo đảm: 40 × 40 = 1600, cộng 40 rồi cộng
41 nữa thì được 1681, mà 1681 = 41 × 41. Một số viết được thành tích của hai số
đều khác 1 thì không phải số nguyên tố.

Con số 41 trong công thức quay lại cắn chính nó, đúng ở lần thứ 41.
::
:::

:::opt
`39`, rồi `True`, rồi `False`
::why
Hai dòng cuối bạn đọc trúng hẳn.

Chỗ lệch ở dòng đầu, và đó là chỗ ai cũng vấp ít nhất một lần: `range(40)` chạy
từ **0** tới 39. Đếm từ 0 tới 39 thì được 40 số chứ không phải 39 — con số 0
cũng chiếm một ô trong danh sách.

Chuyện này không phải chi tiết vặt trong bài hôm nay: chính vì có `n = 0` mà câu
"đúng 40 lần" mới là 40 chứ không phải 39, và cái mốc hỏng ở `n = 40` mới là lần
thứ 41.
::
:::

:::opt
`40`, rồi `False`, rồi `False`
::why
Dòng đầu và dòng cuối bạn đọc trúng.

Chỗ lệch ở dòng hai, và nó đến từ một suy nghĩ rất hợp lý: bài này đang định nói
rằng câu ấy sai, nên chắc máy cũng phải trả `False`.

Nhưng `all(bang)` không hề đọc câu tiếng Việt nào. Nó chỉ nhìn vào **40 ô đang
có trong `bang`**, và cả 40 ô ấy đều `True`, nên nó trả `True`. Máy không giấu
gì, cũng không biết trước gì — nó đang báo cáo trung thực về đúng những số nó đã
xét. Chỗ hụt không nằm ở câu trả lời của máy, mà nằm ở việc ta đọc câu trả lời
ấy rộng hơn phạm vi của nó.
::
:::
::::

::::explain{#may-noi-duoc-gi-va-khong-noi-duoc-gi}
Ba dòng vừa rồi nói ra một điều đáng viết lên bảng:

> **Kiểm hữu hạn không phải chứng minh.** Máy chỉ nói được về những trường hợp
> nó đã xét. Xét 40 số thì kết luận chỉ phủ 40 số ấy, không phủ số thứ 41.

Và đây là chỗ dễ đọc quá tay theo chiều ngược lại, nên nói cho rõ: **máy không
vô dụng.** Nó vẫn kết luận **dứt điểm** được hai việc, và cả hai đều là thứ bạn
vừa học ở hai bài trước:

- **Tìm ra phản ví dụ thì bác bỏ xong một câu "với mọi".** Bài 19: một trường
  hợp làm câu mở sai là đủ. `n = 40` là một số tự nhiên thật, và
  `40 × 40 + 40 + 41` thật sự không phải số nguyên tố. Nên câu "với mọi số tự
  nhiên n…" **sai**, dứt khoát, không cần bàn thêm. Máy vừa làm xong một việc
  trọn vẹn.
- **Tìm ra một nhân chứng thì khẳng định xong một câu "tồn tại".** Bài 18: một
  người là đủ. Câu "tồn tại một số tự nhiên n mà `n × n + n + 41` không phải số
  nguyên tố" **đúng**, và bằng chứng của nó chính là số 40.

Chỗ máy không với tới được chỉ có đúng một, và bài 20 vừa cho bạn cách gọi tên
nó: câu **"với mọi"** trên một miền **vô hạn**. Vì bài 20 nói rằng phủ định của
"với mọi" là "tồn tại một phản ví dụ", nên đi tìm phản ví dụ là cách bác bỏ; mà
**không tìm thấy** phản ví dụ trong 40 số đầu thì chỉ có nghĩa là *chưa thấy
trong 40 số đầu*.

Nói gọn lại thành một câu để mang đi:

> Máy nói được "**chưa thấy sai**". Máy không nói được "**đúng**" cho một câu
> "với mọi" trên miền vô hạn.

Hai câu ấy nghe rất giống nhau và cách nhau rất xa. Cả nửa sau của track này
sinh ra để đi từ câu thứ nhất tới câu thứ hai.
::::

::::code{#san-phan-vi-du-tren-so}
Bắt máy làm đúng cái việc nó làm được: **đi săn phản ví dụ, và nói luôn nó đã
phải xét bao nhiêu số** mới tìm ra.

Bạn viết thân của `san_phan_vi_du`. Nó nhận một mốc `den`, xét lần lượt các số
tự nhiên từ 0 tới `den`, và trả về hai thứ:

- **số đầu tiên** làm câu mở "`n × n + n + 41` là số nguyên tố" thành sai — hoặc
  `None` nếu đi hết mà không gặp số nào;
- **số lượt đã xét** cho tới lúc trả lời. Đây là cái giá thật, nên nó phải được
  đếm ra chứ không phải một con số bạn đã biết trước.

Hai chỗ trống:

1. Sau `da_thu =` — cộng thêm một lượt vào bộ đếm, theo đúng khuôn biến cộng dồn
   của T1.2.12: lấy giá trị cũ, cộng thêm một, gán lại.
2. Sau `if` — câu hỏi "số này có phải phản ví dụ không". Phản ví dụ là số làm
   câu mở **sai**, nên đây là chỗ chữ "không" phải xuất hiện.

Hai hàm `la_nguyen_to` và `cong_thuc` đã viết sẵn cho bạn; đừng sửa chúng.

Bài chấm bằng **hai mốc khác nhau**: mốc 39 (đi hết mà không gặp ai) và mốc 100
(gặp ở giữa đường rồi dừng). Gõ cứng một con số vào thì một trong hai mốc sai
ngay, nên phải viết ra phép đếm thật và câu hỏi thật.

```python title=starter
def la_nguyen_to(n):
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d = d + 1
    return True

def cong_thuc(n):
    return n * n + n + 41

def san_phan_vi_du(den):
    da_thu = 0
    for n in range(den + 1):
        da_thu = ___
        if ___:
            return n, da_thu
    return None, da_thu

pvd_39, so_lan_39 = san_phan_vi_du(39)
pvd_100, so_lan_100 = san_phan_vi_du(100)

print(pvd_39, so_lan_39)
print(pvd_100, so_lan_100)
```

```python title=solution
def la_nguyen_to(n):
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d = d + 1
    return True

def cong_thuc(n):
    return n * n + n + 41

def san_phan_vi_du(den):
    da_thu = 0
    for n in range(den + 1):
        da_thu = da_thu + 1
        if not la_nguyen_to(cong_thuc(n)):
            return n, da_thu
    return None, da_thu

pvd_39, so_lan_39 = san_phan_vi_du(39)
pvd_100, so_lan_100 = san_phan_vi_du(100)

print(pvd_39, so_lan_39)
print(pvd_100, so_lan_100)
```

```python title=test
# Hai câu đứng đầu canh hai cái bẫy lớn nhất của bài, nên chúng phải chạy
# TRƯỚC: nếu một câu `==` ở dưới trượt trước, hai bẫy này không bao giờ sập.
#
#   · bẫy 1 — trả bừa một con số: tới mốc 39 thì KHÔNG có phản ví dụ nào, nên
#     ô thứ nhất phải là `None`.
#   · bẫy 2 — đếm bừa: tới mốc 39 phải xét đúng 40 số, vì `n = 0` cũng là một
#     lượt.
assert san_phan_vi_du(39)[0] is None, "trong 40 số đầu (n từ 0 tới 39) không có số nào làm công thức hết nguyên tố, nên không có phản ví dụ nào để trả về"
assert san_phan_vi_du(39)[1] == 40, "xét từ n = 0 tới n = 39 là 40 lượt, vì số 0 cũng chiếm một lượt — đếm 39 là đã bỏ quên n = 0"
assert pvd_39 is None, "mốc 39 phải cho đúng kết quả như câu trên: đi hết mà không gặp phản ví dụ nào"
assert so_lan_39 == 40, "mốc 39 phải đếm đúng 40 lượt, y như câu trên"
assert pvd_100 == 40, "tới mốc 100 thì phản ví dụ ĐẦU TIÊN là n = 40, vì 40 × 40 + 40 + 41 = 1681 = 41 × 41 — xét tới số lớn hơn mà vẫn trả về nó nghĩa là chưa dừng đúng lúc"
assert so_lan_100 == 41, "gặp phản ví dụ ở n = 40 thì đã xét các số 0 tới 40, tức 41 lượt, rồi dừng ngay — đếm 40 là quên n = 0, đếm 101 là không dừng khi đã gặp"
assert san_phan_vi_du(45) == (40, 41), "đổi mốc từ 100 xuống 45 không đổi được câu trả lời: phản ví dụ đầu tiên vẫn là n = 40, và vẫn tốn 41 lượt để tới đó"
assert san_phan_vi_du(0) == (None, 1), "mốc 0 thì chỉ xét đúng một số là n = 0; công thức cho 41, một số nguyên tố, nên không có phản ví dụ"
assert la_nguyen_to(cong_thuc(41)) is False, "n = 41 cũng là phản ví dụ — nhưng nó không phải cái ĐẦU TIÊN, nên hàm săn không được trả về nó"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm ngay sau `da_thu =`, và nó phải làm con số ấy nhích lên MỘT sau mỗi lượt — đọc lại dòng `da_thu = 0` đứng trên để thấy nó bắt đầu từ đâu. Chỗ trống thứ hai là một câu hỏi Đ/S về đúng con số đang đứng trong tay, tên nó là `n`; hai hàm viết sẵn ở trên cho bạn cách biến `n` thành giá trị công thức, rồi hỏi giá trị ấy có nguyên tố hay không.
- kind: strategy
  body: "Bộ đếm cộng dồn viết theo khuôn của T1.2.12: lấy giá trị cũ rồi cộng thêm một, gán ngược lại vào chính cái tên đó. Còn câu hỏi trong `if`: `cong_thuc(n)` cho ra con số cần xét, `la_nguyen_to(...)` cho biết con số ấy CÓ nguyên tố hay không — mà phản ví dụ lại là số KHÔNG nguyên tố, nên bạn cần lật giá trị ấy bằng đúng công cụ của bài 3."
- kind: one-line
  body: "Chỗ thứ nhất là `da_thu + 1`; chỗ thứ hai là `not la_nguyen_to(cong_thuc(n))`."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: bộ đếm phải cộng thật, và câu hỏi phải chạy qua hai hàm viết sẵn — gõ cứng một con số thì bài đang chấm một thứ nó không hề tính
  requireAst:
  # Khung khởi đầu định nghĩa hai hàm này mà KHÔNG gọi lần nào. Hai luật này
  # một mình chặn mọi đáp án không hỏi tới công thức: `if True`, `if n == 40`,
  # `if da_thu == 41`.
  - kind: uses-call, target: la_nguyen_to, min: 1
  - kind: uses-call, target: cong_thuc, min: 1
  # Bộ đếm phải là một phép CỘNG thật. Khung khởi đầu có đúng 4 dấu cộng
  # (`d + 1`, hai dấu trong `n * n + n + 41`, và `den + 1`); lời giải có 5 —
  # dấu thứ năm nằm đúng trong phép cộng dồn. Gõ `da_thu = 41` thì không đạt.
  - kind: uses-operator, target: +, min: 5
  forbidAst:
  # 40 là ĐÁP ÁN của bài, không phải dữ liệu. Mọi cách viết hợp lệ đều dựng nó
  # ra từ vòng lặp, nên không cách nào chứa nguyên văn con số này.
  - kind: has-literal, target: 40
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: ^None 40\n40 41\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn mươi mốt lượt để bác bỏ xong một câu. Còn để khẳng định nó thì bao nhiêu
lượt cũng không đủ — mình phải tìm cách khác.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Máy chỉ nói được về những số nó đã thử. Muốn nói một câu đúng cho **mọi** số
chẵn mà không thử số nào, bạn phải làm việc với chữ **"chẵn"** chứ không với
từng con số.

Mà chữ "chẵn" thì tính toán không được. Bạn cộng được 4 với 6; bạn không cộng
được chữ "chẵn" với chữ "chẵn".

Vậy chữ "chẵn" viết ra thành **cái gì** mà tính toán được?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
