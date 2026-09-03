---
id: toan.dstt-giai-tich-cho-ai.dao-ham-ham-da-thuc
title: Đạo hàm hàm đa thức
summary: "Quy tắc luỹ thừa: d/dx[xⁿ] = n·xⁿ⁻¹ — công thức ĐẠI SỐ thay cho tính xấp xỉ SỐ (bài 18); kiểm LẠI bằng xấp xỉ SỐ để THẤY công thức khớp."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.power-rule]
requires: [math.derivative]
concepts: [math.quy-tac-luy-thua]
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
Có QUY tắc TỔNG quát nào cho `xⁿ` không, thay VÌ tính xấp xỉ SỐ mỗi
lần?
::::

::::explain{#quy-tac-luy-thua}
CÓ. **Quy tắc luỹ thừa: `d/dx[xⁿ] = n·xⁿ⁻¹`** — công thức ĐẠI SỐ
thay CHO tính xấp xỉ SỐ (bài 18):

```python title=readonly
def dao_ham_luy_thua(n, x):
    return n * x ** (n - 1)


print(dao_ham_luy_thua(2, 3))
```

```text title=readonly
6
```

`n=2` (luỹ thừa `x²`), `x=3`: `d/dx[x²] = 2·x¹ = 2×3 = 6` — KHỚP
ĐÚNG kết quả xấp xỉ SỐ Ở bài 18 (`f'(3)=6` cho `f(x)=x²`).
::::

::::example{#kiem-lai-bang-xap-xi}
KIỂM lại công thức BẰNG xấp xỉ SỐ — cho `h(x)=x³`:

```python title=readonly
def dao_ham_luy_thua(n, x):
    return n * x ** (n - 1)

def dao_ham_xap_xi(f, x, dx=0.000001):
    return (f(x + dx) - f(x)) / dx

def h(x):
    return x ** 3


print(dao_ham_luy_thua(3, 2))
print(round(dao_ham_xap_xi(h, 2), 2))
```

```text title=readonly
12
12.0
```

CÔNG thức (`3×2²=12`) VÀ xấp xỉ SỐ (`≈12.0`) KHỚP nhau — công thức
`n·xⁿ⁻¹` KHÔNG phải một QUY tắc phải NHỚ, MÀ LÀ điều CODE tự XÁC
nhận.
::::

::::predict{#doan-n-bang-1 commitOnce}
Byte tính đạo hàm luỹ thừa `n=1` (TỨC LÀ `f(x)=x`, đường THẲNG) TẠI
NHIỀU điểm `x` khác nhau:

```python
def dao_ham_luy_thua(n, x):
    return n * x ** (n - 1)

print(dao_ham_luy_thua(1, 5))
print(dao_ham_luy_thua(1, 100))
print(dao_ham_luy_thua(1, -3))
```

BA dòng in ra CÓ giống nhau không?

:::opt{correct}
CÓ, CẢ BA đều LÀ `1`
:::

:::opt
KHÔNG, mỗi dòng RA một giá trị KHÁC nhau (TỈ lệ THEO `x`, giống LỐI
`n=2` cho `2x`)
::why
Gần đúng ở việc bạn ÁP dụng quan sát TỪ trường hợp `n=2` (đạo hàm
PHỤ thuộc `x`, ra `2x`) — một PHẢN xạ hợp lý.

Chỗ lệch: VỚI `n=1`, công thức LÀ `1·x⁰ = 1·1 = 1` — `x⁰` LUÔN LÀ
`1` (VỚI mọi `x≠0`, T2.1), NÊN kết quả LUÔN LÀ `1`, KHÔNG phụ thuộc
`x`. Đây LÀ Ý nghĩa HÌNH học: `f(x)=x` LÀ một đường THẲNG dốc
`45°`, độ DỐC LUÔN LÀ `1` Ở MỌI điểm — đúng LỐI ví dụ hàm tuyến
tính Ở bài 17.
::
:::

:::opt
Máy báo lỗi Ở dòng THỨ ba — `x=-3` LÀ số ÂM, VÀ luỹ thừa (`**`)
KHÔNG cho phép CƠ số ÂM
::why
Gần đúng ở việc bạn để ý ĐÚNG `-3` LÀ số ÂM — một quan sát VỀ dấu.

Chỗ lệch: Python HOÀN TOÀN cho phép cơ SỐ âm VỚI `**` (`(-3) ** 0 =
1`, ĐÚNG quy tắc "số MŨ 0 LUÔN ra 1"). Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_dao_ham_luy_thua}
Viết `dao_ham_luy_thua(n, x)` — đạo hàm của `xⁿ` tại `x`, theo quy
tắc luỹ thừa.

```python title=starter
def dao_ham_luy_thua(n, x):
    return ___


print(dao_ham_luy_thua(2, 3))
```

```python title=solution
def dao_ham_luy_thua(n, x):
    return n * x ** (n - 1)


print(dao_ham_luy_thua(2, 3))
```

```python title=test
assert dao_ham_luy_thua(2, 3) == 6, "x^2 tai x=3"
assert dao_ham_luy_thua(3, 2) == 12, "x^3 tai x=2"
assert dao_ham_luy_thua(1, 5) == 1, "n=1 -- luon la 1"
assert dao_ham_luy_thua(1, -3) == 1, "n=1, x am -- van la 1"
assert dao_ham_luy_thua(4, 2) == 32, "x^4 tai x=2"
```

:::hints
- kind: attention
  body: "Cong thuc: n nhan x mu (n-1)."
- kind: strategy
  body: "n * x ** (n - 1)"
- kind: one-line
  body: "___ = n * x ** (n - 1)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tinh n * x ** (n - 1)
  requireAst:
  - kind: uses-operator, target: '*', min: 1
  - kind: uses-operator, target: '**', min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^6\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Quy tắc luỹ thừa — công thức CHO `xⁿ`. Hàm `f(x) = x² + x³` — đạo
hàm của TỔNG có phải TỔNG của hai đạo hàm không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàm `f(x) = x² + x³` (TỔNG hai luỹ thừa) — đạo hàm của TỔNG có phải
TỔNG của hai đạo hàm không?
::::

::::checkpoint{mastery=0.8}
::::
