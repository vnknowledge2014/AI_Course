---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.pipe-dung-reduce-ben-trong
title: "`pipe` dùng `reduce` bên trong — reduce không chỉ gộp SỐ"
summary: "def pipe(value, *ham): return reduce(lambda acc, f: f(acc), ham, value) — reduce (bài 9-10) gộp một DÃY SỐ thành MỘT SỐ; ở đây nó gộp một DÃY HÀM, áp dụng LẦN LƯỢT lên MỘT GIÁ TRỊ. Cùng khuôn acc = f(acc, phần_tử)."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [fp.pipe-uses-reduce]
requires: [fp.pipe-basics]
concepts: [fp.pipe-uses-reduce]
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
Bài trước bạn viết `pipe` bằng một vòng lặp `for` — CHÍNH XÁC khuôn của
`reduce_tu_viet` ở bài 10. Nếu vậy, viết được bằng `reduce` THẬT không?
::::

::::explain{#pipe-la-mot-reduce}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

print(pipe(3, lambda x: x + 1, lambda x: x * 2, str))
```

```text
8
```

Đúng vậy — MỘT dòng, dùng `reduce()` thay cho ba dòng vòng lặp `for`.
Nhớ lại bài 9: `reduce(f, xs, khoi_dau)` gộp dần TỪNG phần tử của `xs`
vào một bộ tích luỹ, bắt đầu từ `khoi_dau`, theo khuôn `acc = f(acc,
phan_tu)`.

Ở bài 9-10, `xs` là một DÃY SỐ (`[1,2,3,4,5]`), `phan_tu` là một SỐ, và
`reduce` GỘP chúng thành MỘT SỐ (cộng, nhân...). Ở đây, `ham` là một
DÃY HÀM, `f` (phần tử của `ham`) là một HÀM, và
`lambda acc, f: f(acc)` KHÔNG "gộp" hai giá trị bằng phép toán — nó ÁP
DỤNG hàm `f` LÊN `acc`. CÙNG khuôn `acc = <biểu thức dùng acc và phần
tử>`, nhưng "phần tử" giờ là MỘT HÀM, không phải một số. `reduce` tổng
quát hơn "cộng dồn số" rất nhiều: nó gộp một DÃY BẤT KỲ (số, chuỗi,
hàm, bất cứ gì) thành MỘT GIÁ TRỊ, miễn có một quy tắc GỘP rõ ràng.
::::

::::example{#doi-chieu-hai-cach-viet}
Hai cách viết `pipe` — vòng lặp (bài 21) và `reduce` (bài này) — CÙNG
kết quả trên MỌI đầu vào:

```python title=readonly
from functools import reduce

def pipe_vong_lap(value, *ham):
    ket_qua = value
    for f in ham:
        ket_qua = f(ket_qua)
    return ket_qua

def pipe_reduce(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

cong_1 = lambda x: x + 1
nhan_doi = lambda x: x * 2

a = pipe_vong_lap(5, cong_1, nhan_doi)
b = pipe_reduce(5, cong_1, nhan_doi)

print(a)
print(a == b)
```

```text title=readonly
12
True
```

Giống hệt bài 10's phát hiện (`reduce_tu_viet` khớp `reduce` thật) —
đây LÀ bằng chứng `reduce(lambda acc, f: f(acc), ham, value)` làm ĐÚNG
việc vòng lặp `for` bài 21 làm, không phải một cách viết "gần đúng".
::::

::::predict{#doan-pipe-reduce-rong commitOnce}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

print(pipe(42))
```

Dòng cuối in ra gì?

:::opt{correct}
`42`
:::

:::opt
Máy báo lỗi — `reduce()` cần ÍT NHẤT một phần tử trong dãy để gộp,
`ham` rỗng (không hàm nào được truyền) làm nó không có gì để chạy
::why
Gần đúng ở việc bạn nhớ ĐÚNG một sự thật thật: `reduce(f, xs)` (KHÔNG
truyền `khoi_dau`) BÁO LỖI nếu `xs` rỗng — sự thật đó có thật, nhưng
không áp dụng ở đây.

Chỗ lệch: `reduce(lambda acc, f: f(acc), ham, value)` CÓ truyền
`khoi_dau` (chính là `value`) — với `ham` rỗng, `reduce` với
`khoi_dau` LUÔN trả về ĐÚNG `khoi_dau`, không chạy vòng lặp nào cả,
không lỗi gì. `pipe(42)` (không hàm nào) trả về `42` y nguyên.
::
:::

:::opt
`0` — vì không có hàm nào trong `ham`, giá trị tích luỹ mặc định của
`reduce` luôn là `0`
::why
Gần đúng ở việc bạn nhớ `0` là một `khoi_dau` PHỔ BIẾN cho `reduce`
(đúng ở bài 9's ví dụ cộng dồn) — con số đó có xuất hiện trong bài học,
nhưng không phải LUÔN LUÔN.

Chỗ lệch: `khoi_dau` của `reduce` ở ĐÂY là `value` (đối số ĐẦU của
`pipe`, ở đây là `42`), không phải hằng số `0` — `reduce` dùng ĐÚNG
`khoi_dau` được TRUYỀN VÀO, không tự đặt về `0`.
::
:::
::::

::::code{#pipe_reduce}
Viết lại `pipe(value, *ham)` — lần này dùng `reduce()` thay vì vòng
lặp `for`.

```python title=starter
from functools import reduce

def pipe(value, *ham):
    ___

print(pipe(3, lambda x: x + 1, lambda x: x * 2, str))
```

```python title=solution
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

print(pipe(3, lambda x: x + 1, lambda x: x * 2, str))
```

```python title=test
assert pipe(3, lambda x: x + 1, lambda x: x * 2, str) == "8", "phải chạy TRÁI SANG PHẢI: 3+1=4, 4*2=8, str(8)='8'"
assert pipe(5) == 5, "không hàm nào thì trả về value gốc, không đổi"
assert pipe(2, lambda x: x * 10) == 20, "một hàm duy nhất vẫn phải áp dụng đúng"
assert pipe(1, lambda x: x + 1, lambda x: x + 1, lambda x: x + 1, lambda x: x + 1) == 5, "phải áp dụng ĐỦ và ĐÚNG THỨ TỰ với bốn hàm"
```

:::hints
- kind: attention
  body: "Dùng reduce(lambda acc, f: f(acc), ham, value) — acc là giá trị đang tích luỹ, f là từng hàm trong ham, value là khoi_dau. Không viết vòng lặp for nữa."
- kind: strategy
  body: 'return reduce(lambda acc, f: f(acc), ham, value) — mỗi bước reduce gọi f(acc) thay vì cộng/nhân acc với phần tử, đúng khuôn acc = f(acc, phan_tu) với phan_tu LÀ MỘT HÀM.'
- kind: one-line
  body: "return reduce(lambda acc, f: f(acc), ham, value)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: pipe phải dùng reduce() (bài đang dạy đúng cách viết lại này), không phải vòng lặp for như bài trước.
  requireAst:
  - kind: uses-call, target: reduce, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "8"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`reduce` gộp được BẤT KỲ dãy gì — kể cả một dãy HÀM. "Gộp một dãy số"
chỉ là MỘT trường hợp riêng, không phải toàn bộ khả năng của nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track này đã dạy `pipe()` — ghép hàm bằng cách TRUYỀN chúng vào một
lời gọi. Nhiều thư viện Python thật (như pandas, Django QuerySet) ghép
các bước bằng CÚ PHÁP KHÁC hẳn — gọi `.method()` nối tiếp nhau
(`obj.loc(...).gioi_han(...)`). Có liên quan gì tới `pipe` không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
