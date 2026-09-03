---
id: toan.dstt-giai-tich-cho-ai.dao-ham-la-gi
title: Đạo hàm là gì
summary: "Đạo hàm f'(x) — độ dốc TỨC THỜI tại x (giới hạn của Δy/Δx khi Δx→0, bài 17); đo TỐC ĐỘ thay đổi CHÍNH XÁC tại một điểm, KHÔNG phải trung bình."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.derivative]
requires: [math.limit-intuitive]
concepts: [math.dao-ham]
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
`f(x)=x²` — tính `f'(x)` TẠI vài điểm bằng xấp xỉ SỐ. Kết quả CÓ
khớp công thức `2x` không?
::::

::::explain{#dao-ham-la-gi}
CÓ. **Đạo hàm `f'(x)`** — độ dốc TỨC THỜI tại `x` (giới HẠN của
`Δy/Δx` khi `Δx→0`, bài 17); đo TỐC ĐỘ thay đổi CHÍNH XÁC tại MỘT
điểm, KHÔNG phải trung BÌNH. Dùng `Δx` CỰC nhỏ LÀM xấp xỉ THỰC dụng
cho giới hạn:

```python title=readonly
def dao_ham_xap_xi(f, x, dx=0.000001):
    return (f(x + dx) - f(x)) / dx

def f(x):
    return x ** 2


print(round(dao_ham_xap_xi(f, 3), 2))
```

```text title=readonly
6.0
```

TẠI `x=3`, `f'(3)=6` — ĐÚNG kết quả HỘI tụ Ở bài 17. `2×3=6` — khớp
CÔNG thức `2x` (bài 19 sẽ đặt TÊN chính THỨC cho quy tắc NÀY).
::::

::::example{#dao-ham-tai-0}
Tại `x=0` — độ dốc BẰNG `0` (đáy của PARABOL `x²`):

```python title=readonly
def dao_ham_xap_xi(f, x, dx=0.000001):
    return (f(x + dx) - f(x)) / dx

def f(x):
    return x ** 2


print(round(dao_ham_xap_xi(f, 0), 2))
```

```text title=readonly
0.0
```

TẠI `x=0`, ĐỒ thị `x²` NẰM Ở đáy (điểm THẤP nhất) — độ dốc BẰNG `0`,
KHÔNG tăng cũng KHÔNG giảm NGAY tại đó. `2×0=0` — vẫn khớp CÔNG
thức.
::::

::::predict{#doan-dao-ham-x-mu-3 commitOnce}
Byte tính đạo hàm của `h(x)=x³` TẠI `x=2`:

```python
def dao_ham_xap_xi(f, x, dx=0.000001):
    return (f(x + dx) - f(x)) / dx

def h(x):
    return x ** 3

print(round(dao_ham_xap_xi(h, 2), 2))
```

Dòng cuối in ra gì (GỢI ý: KHÔNG phải `2x`, VÌ đây LÀ `x³` chứ
KHÔNG phải `x²`)?

:::opt{correct}
`12.0`
:::

:::opt
`4.0` — vì QUY tắc `2x` (bài 18, cho `x²`) áp DỤNG chung cho MỌI
luỹ thừa, CHỈ cần thay `x=2` VÀO: `2×2=4`
::why
Gần đúng ở việc bạn áp DỤNG lại quy tắc VỪA học (`2x`) — một PHẢN
xạ hợp lý khi gặp bài TOÁN tương tự.

Chỗ lệch: `2x` LÀ đạo hàm RIÊNG của `x²` (LUỸ thừa `2`), KHÔNG áp
dụng CHUNG cho MỌI luỹ thừa. `h(x)=x³` LÀ luỹ thừa `3`, CÓ quy tắc
RIÊNG (bài 19 sẽ đặt TÊN: `3x²`) — TẠI `x=2`: `3×2²=3×4=12`, khớp
ĐÚNG kết quả xấp xỉ SỐ.
::
:::

:::opt
Máy báo lỗi khi chạy — `dao_ham_xap_xi` được ĐỊNH nghĩa VỚI tham số
MẶC định `dx=0.000001`, NHƯNG lời GỌI `dao_ham_xap_xi(h, 2)` KHÔNG
truyền `dx`, Python đòi PHẢI truyền ĐỦ mọi tham số
::why
Gần đúng ở việc bạn để ý ĐÚNG `dx` CÓ giá trị MẶC định — một quan
sát VỀ khai BÁO hàm chính xác.

Chỗ lệch: CHÍNH VÌ `dx` có giá trị MẶC định, việc KHÔNG truyền nó
LÀ hoàn TOÀN hợp lệ — Python TỰ động dùng `0.000001`. Biên dịch
sạch, chạy sạch.
::
:::
::::

::::code{#viet_dao_ham_xap_xi}
Viết `dao_ham_xap_xi(f, x, dx=0.000001)` — xấp xỉ đạo hàm của `f`
tại `x`.

```python title=starter
def dao_ham_xap_xi(f, x, dx=0.000001):
    return ___


def f(x):
    return x ** 2


print(round(dao_ham_xap_xi(f, 3), 2))
```

```python title=solution
def dao_ham_xap_xi(f, x, dx=0.000001):
    return (f(x + dx) - f(x)) / dx


def f(x):
    return x ** 2


print(round(dao_ham_xap_xi(f, 3), 2))
```

```python title=test
def f(x):
    return x ** 2

def h(x):
    return x ** 3

assert round(dao_ham_xap_xi(f, 3), 2) == 6.0, "x=3, f=x^2"
assert round(dao_ham_xap_xi(f, 0), 2) == 0.0, "x=0, day parabol"
assert round(dao_ham_xap_xi(f, 5), 2) == 10.0, "x=5, f=x^2"
assert round(dao_ham_xap_xi(h, 2), 2) == 12.0, "x=2, h=x^3"
```

:::hints
- kind: attention
  body: "Ap dung dinh nghia gioi han (bai 17) voi dx mac dinh cuc nho."
- kind: strategy
  body: "(f(x + dx) - f(x)) / dx"
- kind: one-line
  body: "___ = (f(x + dx) - f(x)) / dx"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tinh (f(x+dx) - f(x)) / dx
  requireAst:
  - kind: uses-call, target: f, min: 2
  - kind: uses-operator, target: '/', min: 1
  - kind: uses-operator, target: '+', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^6\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đạo hàm — độ dốc TỨC thời. Có QUY tắc TỔNG quát nào cho `xⁿ` không,
thay VÌ tính xấp xỉ SỐ mỗi LẦN?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàm `f(x) = x² + x³` (TỔNG hai luỹ thừa) — đạo hàm của TỔNG có phải
TỔNG của hai đạo hàm không?
::::

::::checkpoint{mastery=0.8}
::::
