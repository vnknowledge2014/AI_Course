---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.mot-ham-goi-chinh-no-hai-lan
title: "Một hàm gọi chính nó — nhưng gọi HAI lần"
summary: "Fibonacci: fib(n) = fib(n-1) + fib(n-2). Hai lời gọi đệ quy trong một thân hàm khiến chồng lời gọi không còn là một cột thẳng — nó phân nhánh, và cần ĐỦ hai trường hợp cơ sở, không phải một, để chắc chắn dừng."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [alg.double-recursion]
requires: [alg.recursion-limit]
concepts: [alg.double-recursion]
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
Một cột thẳng là chuyện cũ. Hôm nay chồng lời gọi rẽ làm đôi.
::::

::::explain{#khi-mot-lan-khong-du}
Mọi hàm đệ quy đã gặp — đếm ngược, tính tổng, duyệt Trái-Gốc-Phải theo
một nhánh — đều gọi lại chính mình **đúng một lần** ở mỗi lượt. Chồng
lời gọi vì vậy luôn là một cột thẳng đứng: phiếu này nằm ngay trên
phiếu kia, một hàng duy nhất, đúng như đo ở bài "Đệ quy đụng trần".

Có bài toán cần gọi lại chính mình **hai lần** trong cùng một lượt.
Kinh điển nhất: dãy Fibonacci — số 0, 1, 1, 2, 3, 5, 8, 13... — mỗi số
(từ số thứ ba trở đi) là tổng của **hai** số ngay trước nó. Viết đúng
định nghĩa ấy thành hàm:

```python
def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)
```

Dòng cuối gọi `fib` **hai lần** — một cho `n - 1`, một cho `n - 2` —
trong cùng một lượt gọi. Cả hai lời gọi con đều phải chạy xong (đẩy
phiếu, rồi gỡ phiếu) trước khi dấu `+` cộng được hai kết quả lại.
Python tính vế trái của `+` trước — `fib(n - 1)` chạy trọn vẹn, đẩy
rồi gỡ hết mọi phiếu của NÓ — rồi mới tới lượt `fib(n - 2)`.

Có một chỗ dễ vấp, nghiêm trọng hơn hẳn so với đệ quy một nhánh: hàm
này cần **đủ hai** trường hợp cơ sở — cả `n == 0` lẫn `n == 1` — không
phải một. Thiếu mất `if n == 1: return 1`, lời gọi `fib(n - 1)` từ
một lượt `fib(2)` sẽ đi tới `fib(1)`, mà `fib(1)` lại không khớp
`n == 0` nên tiếp tục gọi `fib(0)` và `fib(-1)` — `-1` không bao giờ
bằng `0`, nên nhánh đó cứ lùi mãi: `-1, -2, -3, ...`, và chồng lời gọi
lớn dần cho tới khi chạm mức trần đã đo ở bài trước, rồi vỡ bằng
`RecursionError`. Đây vẫn là một lỗi CÓ báo — không phải loại nguy
hiểm nhất — nhưng nó cho thấy đệ quy đôi cần soát cẩn thận HƠN, vì có
hai nhánh dừng phải đúng, không phải một.
::::

::::example{#nhat-ky-phan-nhanh}
Ghi lại đúng lúc mỗi lời gọi `fib` được ĐẨY lên và LẤY ra, y hệt kỹ
thuật nhật ký ở bài "Đệ quy chính là một ngăn xếp":

```python title=readonly
nhat_ky = []

def fib(n):
    nhat_ky.append(f"ĐẨY fib({n})")
    if n == 0:
        nhat_ky.append(f"LẤY fib({n}) -> 0")
        return 0
    if n == 1:
        nhat_ky.append(f"LẤY fib({n}) -> 1")
        return 1
    ket_qua = fib(n - 1) + fib(n - 2)
    nhat_ky.append(f"LẤY fib({n}) -> {ket_qua}")
    return ket_qua

fib(4)
for dong in nhat_ky:
    print(dong)
```

```text title=readonly
ĐẨY fib(4)
ĐẨY fib(3)
ĐẨY fib(2)
ĐẨY fib(1)
LẤY fib(1) -> 1
ĐẨY fib(0)
LẤY fib(0) -> 0
LẤY fib(2) -> 1
ĐẨY fib(1)
LẤY fib(1) -> 1
LẤY fib(3) -> 2
ĐẨY fib(2)
ĐẨY fib(1)
LẤY fib(1) -> 1
ĐẨY fib(0)
LẤY fib(0) -> 0
LẤY fib(2) -> 1
LẤY fib(4) -> 3
```

Đọc kỹ bốn dòng đầu: `fib(4)` đẩy, rồi `fib(3)` đẩy, rồi `fib(2)` đẩy,
rồi `fib(1)` đẩy — một cột thẳng đi xuống, y hệt đệ quy một nhánh,
**cho tới khi** `fib(1)` chạm cơ sở và LẤY ra ngay. Chỉ từ đó chồng
mới bắt đầu lộ ra là nó phân nhánh: sau khi nhánh `fib(n-1)` của
`fib(2)` đã gỡ hết (`fib(1)` rồi `fib(0)`, dòng 4-7), `fib(2)` mới
LẤY (dòng 8) — và `fib(3)` vẫn CHƯA lấy, vì nó còn phải đợi nhánh thứ
hai của chính nó: một `fib(1)` MỚI (dòng 9), không phải tờ đã gỡ.

Chú ý dòng 4 và dòng 9: cả hai đều là `ĐẨY fib(1)` — hai lời gọi khác
nhau, tới từ hai nhánh khác nhau, hỏi đúng một câu. Chỗ đau này có tên
riêng, và bài sau sẽ vẽ nó ra cho rõ hơn.
::::

::::predict{#doan-so-nhanh-truc-tiep commitOnce}
Cũng `fib` như trên, gọi `fib(4)`:

```python
def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)

fib(4)
```

Khi lượt gọi `fib(4)` này chạy tới dòng
`return fib(n - 1) + fib(n - 2)`, nó tạo ra đúng MẤY lời gọi con
**trực tiếp** — tức những lời gọi được viết ngay trong thân của
chính lượt `fib(4)` này, không tính lời gọi cháu bên trong chúng?

:::opt{correct}
Đúng 2 — `fib(3)` và `fib(2)`, cả hai đều được viết ngay trong thân
của lượt `fib(4)` này
:::

:::opt
4 — vì để tính xong `fib(4)` cần tổng cộng bốn lời gọi `fib` khác nữa
::why
Gần đúng ở việc TỔNG số lời gọi xảy ra trong suốt quá trình tính
`fib(4)` đúng là nhiều hơn 2 — nhật ký ở ví dụ vừa rồi cho thấy tới
chín lượt gọi `fib` tất cả (kể cả chính `fib(4)`).

Chỗ lệch: câu hỏi chỉ hỏi về lời gọi con TRỰC TIẾP, tức được viết
ngay trong dòng `return` của một lượt `fib(4)` cụ thể — không tính
những lời gọi nằm sâu bên trong `fib(3)` hay `fib(2)`. Đúng một dòng
`return fib(n-1) + fib(n-2)` chỉ chứa đúng hai lời gọi hàm, dù mỗi
lời gọi đó sau này còn kéo theo bao nhiêu lời gọi cháu khác.
::
:::

:::opt
1 — chỉ `fib(3)` thật sự được gọi; `fib(2)` chỉ tính khi cần, không
phải một lời gọi thật
::why
Gần đúng ở trực giác rằng có thể có một kiểu "tính khi cần" nào đó —
trực giác này đúng với `and`/`or`, hai toán tử có "đoản mạch" (bỏ qua
vế sau nếu vế trước đã đủ quyết định kết quả).

Chỗ lệch: `+` không phải một toán tử như vậy. Python luôn tính CẢ HAI
vế của phép cộng, không có ngoại lệ — `fib(n - 1)` và `fib(n - 2)`
đều là những lời gọi hàm thật, đều chạy trọn vẹn, đều để lại phiếu
trên chồng lời gọi lúc đang chạy.
::
:::

:::opt
Không xác định được nếu không biết trước giá trị `fib(3)` và `fib(2)`
::why
Gần đúng ở sự thận trọng khi gặp một biểu thức có kết quả chưa biết
trước — thái độ đó hợp lý với nhiều đoạn mã khác.

Chỗ lệch: câu hỏi không hỏi KẾT QUẢ của `fib(3)` hay `fib(2)` là bao
nhiêu — nó hỏi có BAO NHIÊU LỜI GỌI được viết ra trong dòng
`return` đó. Con số ấy nằm ngay trong CÁCH VIẾT của hàm, cố định ở
2, không phụ thuộc `fib(3)` hay `fib(2)` tính ra số mấy.
::
:::
::::

::::code{#viet-fib-de-quy-doi}
Hoàn thiện hàm Fibonacci đệ quy đôi. Hai trường hợp cơ sở đã viết sẵn
— việc của bạn là viết đúng dòng gọi lại chính hàm này, hai lần.

```python title=starter
def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1
    return ___ + ___

print(fib(2))
print(fib(6))
print(fib(9))
```

```python title=solution
def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n - 1) + fib(n - 2)

print(fib(2))
print(fib(6))
print(fib(9))
```

```python title=test
assert fib(0) == 0, "fib(0) phải là 0 — trường hợp cơ sở đã viết sẵn, không được động vào"
assert fib(1) == 1, "fib(1) phải là 1 — trường hợp cơ sở đã viết sẵn, không được động vào"
assert fib(2) == 1, f"fib(2) = fib(1) + fib(0) = 1 + 0 = 1 — đang ra {fib(2)}; nếu ra 2 thì có thể cả hai chỗ trống đang gọi CÙNG một lời gọi (ví dụ fib(n-1) hai lần) thay vì hai lời gọi khác nhau"
assert fib(6) == 8, f"fib(6) phải là 8 (dãy 0,1,1,2,3,5,8...) — đang ra {fib(6)}"
assert fib(9) == 34, f"fib(9) phải là 34 — đang ra {fib(9)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống là hai LỜI GỌI LẠI CHÍNH fib, không phải hai con số. Đúng định nghĩa Fibonacci — số hiện tại bằng tổng của HAI số ngay trước nó.
- kind: strategy
  body: 'fib(n - 1) là số ngay trước n; fib(n - 2) là số trước đó nữa. Viết return fib(n - 1) + fib(n - 2) — đúng hai lời gọi, đúng hai đối số khác nhau. Đừng gọi cùng một đối số hai lần (như fib(n - 1) + fib(n - 1)) — con số ra sẽ sai ngay từ fib(2).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là fib(n - 1) và fib(n - 2).'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ gọi lại fib — đúng hai lần, một cho n - 1 và một cho n - 2 — không phải một câu không làm gì như True, 1, 0
  requireAst:
  - kind: uses-call, target: fib, min: 5
  # min: 5 — luật này đếm trên TOÀN BỘ mã nguồn, không chỉ dòng return, nên ba
  # lời gọi print(fib(...)) đã có sẵn trong khung CŨNG được tính (chúng là
  # uses-call target=fib thật sự). Đếm thật trên solution: 3 (ba lời print có
  # sẵn) + 2 (hai chỗ trống trong dòng return) = 5. ĐÃ THỬ THẬT bằng kiemAst
  # thật trên bản điền hụt: True/1/0 vào cả hai chỗ trống chỉ còn đúng 3 (ba
  # lời print, không thêm lời gọi fib nào từ dòng return) — dưới 5, luật này
  # chặn được (trước khi sửa, min từng đặt là 2 và bị chính ba lời print có
  # sẵn "che" mất — 3 vẫn ≥ 2 — nên bản điền hụt lọt qua static; sửa lại
  # thành 5 cho khớp đúng số lần solution thật sự có). Cả ba cách điền hụt
  # cũng dừng AN TOÀN — không lặp vô hạn, vì hai trường hợp cơ sở không phụ
  # thuộc chỗ trống — và cho fib(6) sai (2 hoặc 0, không phải 8), bị tests
  # bắt độc lập. Đã thử thêm một lời giải SAI-nhưng-hợp-lý khác — fib(n-1)
  # hai lần thay vì fib(n-1)+fib(n-2) — có đủ 5 lời gọi nên qua được static,
  # nhưng fib(2) ra 2 thay vì 1, bị assert fib(2) bắt riêng.
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^1\\n8\\n34\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai lời gọi, một dấu cộng, và chồng lời gọi giờ rẽ nhánh thật sự.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhìn lại nhật ký `fib(4)` ở ví dụ: dòng 4 ghi `ĐẨY fib(1)`, và dòng 9
ghi lại đúng `ĐẨY fib(1)` — hai lời gọi khác nhau, cùng hỏi một câu,
cùng nhận một câu trả lời (1). Với `fib(4)` — con số nhỏ — chuyện này
chỉ xảy ra một lần thừa.

Nếu tính không phải `fib(4)` mà là `fib(6)`, câu hỏi "fib(2) là bao
nhiêu?" bị lặp lại bao nhiêu lần? Vẽ cây cuộc gọi ra giấy và đếm thử
bằng mắt.

Bài sau làm đúng việc đó.
::::

::::checkpoint{mastery=0.8}
::::
