---
id: toan.dstt-giai-tich-cho-ai.vector-gradient
title: Vector gradient
summary: "∇f = (∂f/∂x, ∂f/∂y) — VECTOR (bài 1) gồm TẤT CẢ đạo hàm riêng; ∇f chỉ HƯỚNG mà f TĂNG nhanh NHẤT tại một điểm."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.gradient-vector]
requires: [math.partial-derivative]
concepts: [math.gradient]
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
Gộp CẢ hai đạo hàm riêng lại thành MỘT cấu trúc DUY nhất — cấu trúc
ĐÓ giống thứ GÌ đã học Ở đầu track?
::::

::::explain{#vector-gradient}
GIỐNG một VECTOR (bài 1). **`∇f = (∂f/∂x, ∂f/∂y)`** — VECTOR gồm
TẤT CẢ đạo hàm riêng; `∇f` chỉ HƯỚNG mà `f` TĂNG nhanh NHẤT tại một
điểm:

```python title=readonly
def dao_ham_rieng_x(f, x, y, d=0.000001):
    return (f(x + d, y) - f(x, y)) / d

def dao_ham_rieng_y(f, x, y, d=0.000001):
    return (f(x, y + d) - f(x, y)) / d

def gradient(f, x, y):
    return (dao_ham_rieng_x(f, x, y), dao_ham_rieng_y(f, x, y))

def f(x, y):
    return x ** 2 + y ** 2


g = gradient(f, 3, 4)
print(round(g[0], 2), round(g[1], 2))
```

```text title=readonly
6.0 8.0
```

`∇f(3,4) = (6.0, 8.0)` — GHÉP `∂f/∂x=6.0` (bài 23) VÀ `∂f/∂y=8.0`
LẠI thành MỘT vector HAI chiều (bài 1), ĐÚNG cấu TRÚC đã quen.
::::

::::example{#do-dai-gradient}
Độ dài (bài 4) của `∇f(3,4)` — MỘT con số QUEN thuộc:

```python title=readonly
def dao_ham_rieng_x(f, x, y, d=0.000001):
    return (f(x + d, y) - f(x, y)) / d

def dao_ham_rieng_y(f, x, y, d=0.000001):
    return (f(x, y + d) - f(x, y)) / d

def gradient(f, x, y):
    return (dao_ham_rieng_x(f, x, y), dao_ham_rieng_y(f, x, y))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def f(x, y):
    return x ** 2 + y ** 2


g = gradient(f, 3, 4)
print(round(do_dai(g), 2))
```

```text title=readonly
10.0
```

`‖∇f(3,4)‖ = ‖(6,8)‖ = 10` — ĐÚNG tam giác `6-8-10` (bài 4). Độ DÀI
gradient đo "TỐC độ tăng NHANH nhất" TẠI điểm ĐÓ — CÀNG lớn, `f`
CÀNG dốc.
::::

::::predict{#doan-gradient-tai-goc commitOnce}
Byte tính `∇f` TẠI điểm gốc `(0,0)`:

```python
def dao_ham_rieng_x(f, x, y, d=0.000001):
    return (f(x + d, y) - f(x, y)) / d

def dao_ham_rieng_y(f, x, y, d=0.000001):
    return (f(x, y + d) - f(x, y)) / d

def gradient(f, x, y):
    return (dao_ham_rieng_x(f, x, y), dao_ham_rieng_y(f, x, y))

def f(x, y):
    return x ** 2 + y ** 2

g0 = gradient(f, 0, 0)
print(round(g0[0], 2), round(g0[1], 2))
```

Dòng cuối in ra gì?

:::opt{correct}
`0.0 0.0`
:::

:::opt
Máy báo lỗi khi chạy — tại `x=0, y=0`, phép TÍNH `(f(0+d,0)-f(0,0))
/d` chia CHO một khoảng NGẮN NGAY sát GỐC, gây LỖI chia gần `0`
::why
Gần đúng ở việc bạn để ý ĐÚNG điểm gốc `(0,0)` LÀ một VỊ trí đặc
BIỆT (đáy CỦA `f(x,y)=x²+y²`) — một quan sát VỀ Ý nghĩa hình học.

Chỗ lệch: PHÉP chia LÀ `.../d` (chia CHO `d`, KHÔNG PHẢI chia cho
`0`) — `d` LUÔN LÀ `0.000001`, một số CỐ định KHÔNG đổi, DÙ `x, y`
LÀ gì. KHÔNG có phép CHIA nào gần `0` cả. Biên dịch sạch, chạy sạch.
::
:::

:::opt
`(0.0, 0.0)` in RA đúng nghĩa ĐEN VỚI dấu ngoặc `()`, KHÔNG phải hai
số CÁCH nhau bằng dấu CÁCH
::why
Gần đúng ở việc bạn để ý CÁCH `print` thường hiển thị `tuple` VỚI
dấu ngoặc — một quan sát ĐÚNG cho `print(g0)` (in NGUYÊN `tuple`).

Chỗ lệch: CODE ở đây in RA `g0[0]` VÀ `g0[1]` (HAI GIÁ trị TÁCH
riêng, cách NHAU bằng dấu PHẨY trong lệnh `print`), KHÔNG in
NGUYÊN `tuple` `g0` — `print` mặc ĐỊNH nối các đối SỐ bằng dấu
CÁCH, KHÔNG dấu ngoặc.
::
:::
::::

::::code{#viet_gradient}
Viết `gradient(f, x, y)` — vector gradient TẠI `(x,y)`.

```python title=starter
def dao_ham_rieng_x(f, x, y, d=0.000001):
    return (f(x + d, y) - f(x, y)) / d

def dao_ham_rieng_y(f, x, y, d=0.000001):
    return (f(x, y + d) - f(x, y)) / d

def gradient(f, x, y):
    return ___


def f(x, y):
    return x ** 2 + y ** 2


g = gradient(f, 3, 4)
print(round(g[0], 2), round(g[1], 2))
```

```python title=solution
def dao_ham_rieng_x(f, x, y, d=0.000001):
    return (f(x + d, y) - f(x, y)) / d

def dao_ham_rieng_y(f, x, y, d=0.000001):
    return (f(x, y + d) - f(x, y)) / d

def gradient(f, x, y):
    return (dao_ham_rieng_x(f, x, y), dao_ham_rieng_y(f, x, y))


def f(x, y):
    return x ** 2 + y ** 2


g = gradient(f, 3, 4)
print(round(g[0], 2), round(g[1], 2))
```

```python title=test
def f(x, y):
    return x ** 2 + y ** 2

g = gradient(f, 3, 4)
assert round(g[0], 2) == 6.0 and round(g[1], 2) == 8.0, "gradient tai (3,4)"
g0 = gradient(f, 0, 0)
assert round(g0[0], 2) == 0.0 and round(g0[1], 2) == 0.0, "gradient tai goc -- vector khong"
assert len(gradient(f, 1, 1)) == 2, "gradient la vector hai chieu"
```

:::hints
- kind: attention
  body: "Ghep hai dao ham rieng thanh mot tuple: (dao_ham_rieng_x(...), dao_ham_rieng_y(...))."
- kind: strategy
  body: "(dao_ham_rieng_x(f, x, y), dao_ham_rieng_y(f, x, y))"
- kind: one-line
  body: "___ = (dao_ham_rieng_x(f, x, y), dao_ham_rieng_y(f, x, y))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai ghep dao_ham_rieng_x va dao_ham_rieng_y thanh mot tuple
  requireAst:
  - kind: uses-call, target: dao_ham_rieng_x, min: 1
  - kind: uses-call, target: dao_ham_rieng_y, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^6\.0 8\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Gradient — vector chỉ hướng TĂNG nhanh nhất. Muốn hàm CHI PHÍ (bài
22) GIẢM nhanh NHẤT — nên đi THEO hướng gradient, hay hướng NGƯỢC
lại?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Muốn hàm CHI PHÍ (bài 22) GIẢM nhanh NHẤT — nên đi THEO hướng
gradient, hay hướng NGƯỢC LẠI?
::::

::::checkpoint{mastery=0.8}
::::
