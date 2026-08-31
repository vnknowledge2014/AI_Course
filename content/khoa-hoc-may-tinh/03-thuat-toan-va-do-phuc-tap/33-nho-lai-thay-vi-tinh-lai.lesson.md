---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.nho-lai-thay-vi-tinh-lai
title: "Nhớ lại kết quả cũ thay vì tính lại — quy hoạch động nhập môn"
summary: "Giữ một dict cất kết quả fib đã tính — mỗi n chỉ tính THẬT đúng một lần, những lần hỏi lại sau chỉ tra sổ. Đếm lại bằng đúng bộ đếm bài 6: fib(15) từng cần 1973 lượt tính, giờ chỉ còn 16 — đúng bằng số giá trị khác nhau cần biết."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 33
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.memoization]
requires: [alg.recompute-waste, alg.count-recursive-calls]
concepts: [alg.memoization]
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
`fib(2)` bị hỏi năm lần khi tính `fib(6)` — và năm lần đó luôn nhận đúng
CÙNG một câu trả lời. Vậy sao không chép lại câu trả lời đầu tiên, thay
vì tính lại từ đầu mỗi lần?
::::

::::explain{#giu-mot-cuon-so}
Bài 5 và bài 6 để lại một con số khó chịu: `fib(15)` cần tới 1973 lượt
gọi thật, chỉ để tính một dãy số mà bản thân nó chỉ có 16 giá trị khác
nhau — từ `fib(0)` tới `fib(15)`. Vấn đề không phải hàm SAI. Vấn đề là
hàm không có TRÍ NHỚ: mỗi lần được hỏi lại, nó tính lại từ đầu, y như
chưa từng gặp câu hỏi đó bao giờ.

Cách chữa gọi là **ghi nhớ** (memoization) — kỹ thuật mở đầu cho một
nhánh lớn hơn gọi là **quy hoạch động** (dynamic programming): giữ một
`dict` đóng vai trò một cuốn sổ, ghi lại mọi kết quả đã tính. Trước khi
tính bất cứ điều gì, tra sổ trước — có rồi thì LẤY RA dùng ngay, không có
mới thật sự bắt tay vào tính, rồi GHI kết quả vào sổ trước khi trả về,
để lần sau tra là thấy ngay.

```python
def fib_memo(n, bo_nho):
    if n in bo_nho:
        return bo_nho[n]
    if n == 0:
        ket_qua = 0
    elif n == 1:
        ket_qua = 1
    else:
        ket_qua = fib_memo(n - 1, bo_nho) + fib_memo(n - 2, bo_nho)
    bo_nho[n] = ket_qua
    return ket_qua
```

So với `fib` cũ, thân hàm không đổi PHÉP TÍNH — vẫn đúng công thức
`fib(n-1) + fib(n-2)`, vẫn đúng hai trường hợp cơ sở. Thứ thêm vào chỉ là
một cái CỔNG ở đầu (`if n in bo_nho: return bo_nho[n]`) và một dòng GHI
SỔ ở cuối (`bo_nho[n] = ket_qua`). Cổng ấy là chỗ chặn đứng việc tính lại
— gặp một `n` đã có trong sổ, hàm trả lời ngay mà không đi xuống tính
toán, không gọi đệ quy con nào cả.
::::

::::example{#dem-lai-bang-bo-dem-cu}
Byte gắn lại đúng bộ đếm sống của bài 6 — một biến toàn cục, tăng mỗi lần
hàm THẬT SỰ TÍNH (không tăng khi trả lời từ sổ):

```python title=readonly
so_lan_tinh_that = 0

def fib_memo(n, bo_nho):
    global so_lan_tinh_that
    if n in bo_nho:
        return bo_nho[n]
    so_lan_tinh_that += 1
    if n == 0:
        ket_qua = 0
    elif n == 1:
        ket_qua = 1
    else:
        ket_qua = fib_memo(n - 1, bo_nho) + fib_memo(n - 2, bo_nho)
    bo_nho[n] = ket_qua
    return ket_qua

bo_nho = {}
print("fib(6) =", fib_memo(6, bo_nho))
print("Số lần tính thật:", so_lan_tinh_that)
```

```text title=readonly
fib(6) = 8
Số lần tính thật: 7
```

Bài 5 đếm bằng mắt trên cây vẽ tay: `fib(6)` cần **hai mươi lăm** lượt
gọi khi không có sổ. Có sổ, con số tụt xuống còn **bảy** — đúng bằng số
giá trị khác nhau từ `fib(0)` tới `fib(6)` (bảy giá trị: 0, 1, 2, 3, 4,
5, 6). Mỗi giá trị chỉ còn bị tính đúng một lần, bất kể sau đó bị hỏi lại
bao nhiêu lượt — `fib(2)` từng bị hỏi năm lần thì giờ chỉ TÍNH một lần
duy nhất, bốn lần hỏi lại sau chỉ tra sổ.
::::

::::predict{#doan-fib10-tinh-that commitOnce}
Cũng `fib_memo` như trên, nhưng gọi `fib_memo(10, {})` thay vì `fib_memo(6, {})`:

```python
so_lan_tinh_that = 0

def fib_memo(n, bo_nho):
    global so_lan_tinh_that
    if n in bo_nho:
        return bo_nho[n]
    so_lan_tinh_that += 1
    if n == 0:
        ket_qua = 0
    elif n == 1:
        ket_qua = 1
    else:
        ket_qua = fib_memo(n - 1, bo_nho) + fib_memo(n - 2, bo_nho)
    bo_nho[n] = ket_qua
    return ket_qua

fib_memo(10, {})
print(so_lan_tinh_that)
```

**Trước khi chạy**, bạn đoán `so_lan_tinh_that` in ra con số nào?

:::opt{correct}
`11` — có sổ, mỗi giá trị từ `fib(0)` tới `fib(10)` chỉ tính thật đúng
một lần, và có đúng mười một giá trị khác nhau trong khoảng đó
:::

:::opt
`177` — đúng con số bài 6 đã đo cho `fib(10)` KHÔNG có sổ
::why
Gần đúng ở việc 177 là một con số có thật, đã đo đúng ở bài 6 — nhưng đó
là con số của bản KHÔNG có `bo_nho`, nơi mỗi lượt gọi đều tính lại từ
đầu, dù đã gặp câu hỏi đó trước hay chưa.

Chỗ lệch: `fib_memo` có cổng chặn ở đầu — gặp một `n` đã có trong sổ,
`so_lan_tinh_that += 1` KHÔNG chạy tới, hàm trả lời ngay và thoát. 177
là con số của một hàm không có cổng ấy.
::
:::

:::opt
`1` — vì có sổ rồi thì chỉ cần tính đúng MỘT LẦN, không hơn
::why
Gần đúng ở tinh thần "không tính lại" — chỗ đó không sai.

Chỗ lệch nằm ở việc hiểu nhầm "một lần" là gì. Sổ chặn được việc tính LẠI
CÙNG MỘT `n` — không chặn được việc phải tính MỖI `n` khác nhau đúng một
lần. Muốn biết `fib(10)`, hàm vẫn phải lần lượt biết `fib(0)`, `fib(1)`,
..., cho tới `fib(10)` — mười một giá trị riêng biệt, mỗi giá trị tính
thật đúng một lần, cộng lại là 11, không phải 1.
::
:::

:::opt
`21` — nhầm với con số của `fib(20)`
::why
Gần đúng ở việc 21 đúng là một con số thật, sẽ xuất hiện khi tính
`fib(20)` có sổ (hai mươi mốt giá trị khác nhau từ `fib(0)` tới
`fib(20)`).

Chỗ lệch: câu hỏi đang hỏi về `fib(10)`, không phải `fib(20)`. Số giá trị
khác nhau cần cho `fib(10)` là từ 0 tới 10 — mười một giá trị, không phải
hai mươi mốt.
::
:::
::::

::::code{#fib15-co-so}
Tính `fib(15)` bằng `fib_memo`, đo lại số lần tính thật, và so với con số
1973 bài 6 đã đo cho bản không sổ. Hai trường hợp cơ sở và lời gọi đệ quy
đã viết sẵn, không đổi — việc của bạn là hoàn thiện đúng CƠ CHẾ CUỐN SỔ:
tra sổ trước khi tính, ghi sổ sau khi tính xong.

```python title=starter
so_lan_tinh_that = 0

def fib_memo(n, bo_nho):
    global so_lan_tinh_that
    if n in ___:                       # đã có n này trong sổ chưa?
        return bo_nho[n]
    so_lan_tinh_that += 1
    if n == 0:
        ket_qua = 0
    elif n == 1:
        ket_qua = 1
    else:
        ket_qua = fib_memo(n - 1, bo_nho) + fib_memo(n - 2, bo_nho)
    bo_nho[___] = ket_qua               # ghi kết quả vào sổ, đúng vị trí n
    return ket_qua

bo_nho = {}
ket_qua_fib_15 = fib_memo(15, bo_nho)

print(f"fib(15) = {ket_qua_fib_15}")
print(f"Số lần tính thật: {so_lan_tinh_that}")
```

```python title=solution
so_lan_tinh_that = 0

def fib_memo(n, bo_nho):
    global so_lan_tinh_that
    if n in bo_nho:
        return bo_nho[n]
    so_lan_tinh_that += 1
    if n == 0:
        ket_qua = 0
    elif n == 1:
        ket_qua = 1
    else:
        ket_qua = fib_memo(n - 1, bo_nho) + fib_memo(n - 2, bo_nho)
    bo_nho[n] = ket_qua
    return ket_qua

bo_nho = {}
ket_qua_fib_15 = fib_memo(15, bo_nho)

print(f"fib(15) = {ket_qua_fib_15}")
print(f"Số lần tính thật: {so_lan_tinh_that}")
```

```python title=test
assert ket_qua_fib_15 == 610, f"fib(15) phải là 610 — đang ra {ket_qua_fib_15}"
assert so_lan_tinh_that == 16, f"phải tính thật đúng 16 lần (mười sáu giá trị khác nhau, từ fib(0) tới fib(15)) — đang ra {so_lan_tinh_that}"
assert so_lan_tinh_that < 1973, f"có sổ phải tính thật ÍT HƠN HẲN 1973 lần — con số của bản không có sổ ở bài 6 — đang ra {so_lan_tinh_that}"
```

:::hints
- kind: attention
  body: Chỗ trống 1 nằm trong câu hỏi "n đã có TRONG ĐÂU rồi?" — trả lời bằng chính cuốn sổ, không phải một con số. Chỗ trống 2 là VỊ TRÍ ghi vào sổ — phải đúng khớp với n đang được tính ở lượt này.
- kind: strategy
  body: 'Cuốn sổ chính là tham số bo_nho — "if n in bo_nho" hỏi đúng câu "n này đã ghi trong sổ chưa". Ghi kết quả vào đúng vị trí n bằng "bo_nho[n] = ket_qua" — dùng lại đúng n, không phải một biến khác, để lần sau tra đúng n này là thấy ngay.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là bo_nho (trong if n in ___) và n (trong bo_nho[___] = ket_qua).'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống 1 phải THẬT SỰ tra cứu trong bo_nho, chỗ trống 2 phải THẬT SỰ ghi vào đúng vị trí n — không phải một câu không làm gì như True, 1, 0
  requireAst:
  - kind: uses-name, target: bo_nho, min: 6
  - kind: uses-name, target: n, min: 7
  # min: 6 cho bo_nho — đếm thật trên solution bằng kiemAst (chỉ đếm Load,
  # gán "bo_nho = {}" không tính): "if n in bo_nho", "return bo_nho[n]",
  # hai lần trong "fib_memo(n - 1, bo_nho)" / "fib_memo(n - 2, bo_nho)" (đã
  # có sẵn trong khung), "bo_nho[n] = ket_qua" (chỗ trống 2, load bo_nho ở
  # vế trái subscript), và "fib_memo(15, bo_nho)" ở cuối = đúng 6. Điền
  # True/1/0 vào chỗ trống 1 làm mất đúng 1 lần Load này, tụt còn 5 — dưới
  # 6, bắt được. min: 7 cho n — đếm thật: "if n in bo_nho", "return
  # bo_nho[n]", "if n == 0", "elif n == 1", hai lần trong "fib_memo(n - 1,
  # ...)" / "fib_memo(n - 2, ...)", và "bo_nho[n] = ket_qua" (chỗ trống 2)
  # = đúng 7. Điền True/1/0 vào chỗ trống 2 làm mất đúng 1 lần Load n này,
  # tụt còn 6 — dưới 7, bắt được cả trường hợp CHỈ chỗ trống 2 sai dù chỗ
  # trống 1 đúng (bài học từ T3.1/T3.2: không được chỉ đếm một phía). ĐÃ
  # THỬ THẬT bằng kiemAst: điền True/1/0 vào CẢ HAI chỗ trống cùng lúc —
  # cả ba cách đều AN TOÀN, không chạy vô hạn — vì hai trường hợp cơ sở
  # (n == 0, n == 1) và hai lời gọi đệ quy fib_memo(n-1, ...)/fib_memo(n-2,
  # ...) vẫn giữ nguyên trong khung, đệ quy luôn lùi xuống 0 hoặc 1 bình
  # thường; "if n in True" / "if n in 1" / "if n in 0" nổ TypeError ngay
  # ở lượt gọi ĐẦU TIÊN (trước khi có bất kỳ đệ quy nào xảy ra) — bị run
  # tier bắt tức khắc. Cũng đã thử riêng trường hợp chỉ chỗ trống 2 điền
  # hụt (chỗ trống 1 đúng): không nổ lỗi, chạy trọn vẹn nhưng chậm hẳn
  # (so_lan_tinh_that lên tới hàng nghìn, vì sổ ghi sai vị trí nên gần như
  # mất tác dụng) — vẫn hữu hạn, không treo, và bị assert so_lan_tinh_that
  # == 16 bắt được ngay.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^fib\\(15\\) = 610\\nSố lần tính thật: 16\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một nghìn chín trăm bảy mươi ba, xuống còn mười sáu — không đổi công
thức, chỉ thêm một cuốn sổ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Túi công cụ giờ đã khá đầy: đếm bước, gọi tên bằng Big-O, tìm tuyến tính
và nhị phân, ba cách sắp xếp bậc hai với sắp xếp trộn nhanh hơn, BFS với
DFS, tham lam — và giờ thêm ghi nhớ. Mỗi công cụ giải đúng MỘT loại bài
toán, và dùng sai chỗ thì hỏng: tham lam sai ở hệ mệnh giá lạ (bài
trước), đệ quy trần trụi tính thừa hàng nghìn lần trong khi ghi nhớ chỉ
cần đúng số giá trị khác nhau (bài này).

Gặp một bài toán HOÀN TOÀN MỚI — chưa ai nói trước nên dùng công cụ nào —
làm sao biết nên rút đúng công cụ ra? Bài sau không dạy công cụ mới nữa.
Nó bắt tự chọn, tự viết, và tự ĐO để chứng minh mình chọn đúng.
::::

::::checkpoint{mastery=0.8}
::::
