---
id: toan.dstt-giai-tich-cho-ai.dao-ham-tong-hieu
title: Đạo hàm tổng, hiệu
summary: "(f+g)' = f'+g', (f−g)' = f'−g' — đạo hàm PHÂN PHỐI qua tổng/hiệu; TÍNH đạo hàm một hàm PHỨC bằng cách TÁCH thành TỪNG số hạng."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.sum-rule-derivative]
requires: [math.power-rule]
concepts: [math.dao-ham-tong-hieu]
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
Hàm `f(x) = x² + x³` — đạo hàm của TỔNG có phải TỔNG của hai đạo
hàm không?
::::

::::explain{#dao-ham-tong-hieu}
ĐÚNG, PHẢI. **`(f+g)' = f'+g'`**, `(f−g)' = f'−g'` — đạo hàm PHÂN
PHỐI qua TỔNG/hiệu; TÍNH đạo hàm một hàm PHỨC bằng cách TÁCH thành
TỪNG số hạng:

```python title=readonly
def dao_ham_luy_thua(n, x):
    return n * x ** (n - 1)

def dao_ham_tong(n1, n2, x):
    return dao_ham_luy_thua(n1, x) + dao_ham_luy_thua(n2, x)


print(dao_ham_tong(2, 3, 2))
```

```text title=readonly
16
```

`f(x)=x²+x³` TẠI `x=2`: đạo hàm CỦA `x²` LÀ `4` (bài 19: `2×2`),
đạo hàm CỦA `x³` LÀ `12` (`3×2²`) — CỘNG LẠI: `4+12=16`.
::::

::::example{#kiem-lai-bang-xap-xi}
KIỂM lại BẰNG xấp xỉ SỐ — đúng công thức:

```python title=readonly
def dao_ham_luy_thua(n, x):
    return n * x ** (n - 1)

def dao_ham_tong(n1, n2, x):
    return dao_ham_luy_thua(n1, x) + dao_ham_luy_thua(n2, x)

def dao_ham_xap_xi(f, x, dx=0.000001):
    return (f(x + dx) - f(x)) / dx

def f(x):
    return x ** 2 + x ** 3


print(dao_ham_tong(2, 3, 2))
print(round(dao_ham_xap_xi(f, 2), 2))
```

```text title=readonly
16
16.0
```

CÔNG thức TÁCH riêng (`16`) VÀ xấp xỉ SỐ trên hàm GỘP (`16.0`) —
KHỚP nhau. Quy tắc TỔNG hoạt động ĐÚNG NHƯ mong đợi.
::::

::::predict{#doan-thu-tu-hieu commitOnce}
Byte tính đạo hàm HIỆU, NHƯNG đổi THỨ tự — `x²−x³` (KHÔNG phải
`x³−x²` như thường thấy) TẠI `x=2`:

```python
def dao_ham_luy_thua(n, x):
    return n * x ** (n - 1)

def dao_ham_hieu(n1, n2, x):
    return dao_ham_luy_thua(n1, x) - dao_ham_luy_thua(n2, x)

print(dao_ham_hieu(2, 3, 2))
```

Dòng cuối in ra gì?

:::opt{correct}
`-8`
:::

:::opt
`8` — vì phép TRỪ hai đạo hàm LUÔN cho MỘT giá trị "độ CHÊNH lệch"
KHÔNG âm, DÙ đổi thứ tự tham SỐ như thế nào
::why
Gần đúng ở việc bạn nghĩ TỚI "độ chênh LỆCH" như một khái niệm
KHÔNG âm — MỘT trực giác quen từ khoảng CÁCH (bài 5), vốn LUÔN
không âm.

Chỗ lệch: `dao_ham_hieu` KHÔNG tính khoảng CÁCH — nó tính HIỆU trực
tiếp `f'−g'`, GIỮ nguyên dấu. `dao_ham_luy_thua(2,2)=4` (đạo hàm
`x²`), `dao_ham_luy_thua(3,2)=12` (đạo hàm `x³`) — `4−12=-8`, MỘT
số ÂM HOÀN toàn hợp lệ, ĐÚNG THỨ tự trừ tham số TRUYỀN vào.
::
:::

:::opt
Máy báo lỗi khi chạy — GỌI `dao_ham_hieu(2, 3, 2)` VỚI `n1=2 < n2=3`
LÀ SAI thứ tự, hàm YÊU cầu `n1` PHẢI lớn hơn `n2`
::why
Gần đúng ở việc bạn để ý ĐÚNG `n1=2` nhỏ hơn `n2=3` — một quan sát
VỀ THỨ tự tham số.

Chỗ lệch: `dao_ham_hieu` KHÔNG hề YÊU cầu `n1 > n2` — hàm nhận BẤT
kỳ hai số nào, tính hiệu ĐÚNG thứ tự TRUYỀN vào, KHÔNG kiểm tra
GÌ thêm. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_dao_ham_tong}
Viết `dao_ham_tong(n1, n2, x)` — đạo hàm của `xⁿ¹ + xⁿ²` tại `x`.

```python title=starter
def dao_ham_luy_thua(n, x):
    return n * x ** (n - 1)

def dao_ham_tong(n1, n2, x):
    return ___


print(dao_ham_tong(2, 3, 2))
```

```python title=solution
def dao_ham_luy_thua(n, x):
    return n * x ** (n - 1)

def dao_ham_tong(n1, n2, x):
    return dao_ham_luy_thua(n1, x) + dao_ham_luy_thua(n2, x)


print(dao_ham_tong(2, 3, 2))
```

```python title=test
assert dao_ham_tong(2, 3, 2) == 16, "x^2+x^3 tai x=2"
assert dao_ham_tong(1, 1, 5) == 2, "x+x tai x=5 -- 1+1"
assert dao_ham_tong(2, 2, 3) == 12, "x^2+x^2 tai x=3 -- 6+6"
assert dao_ham_tong(3, 2, 2) == 16, "doi thu tu -- cong giao hoan, van 16"
```

:::hints
- kind: attention
  body: "Cong hai dao ham luy thua rieng le: dao_ham_luy_thua(n1,x) + dao_ham_luy_thua(n2,x)."
- kind: strategy
  body: "dao_ham_luy_thua(n1, x) + dao_ham_luy_thua(n2, x)"
- kind: one-line
  body: "___ = dao_ham_luy_thua(n1, x) + dao_ham_luy_thua(n2, x)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cong dao_ham_luy_thua(n1,x) voi dao_ham_luy_thua(n2,x)
  requireAst:
  - kind: uses-call, target: dao_ham_luy_thua, min: 2
  - kind: uses-operator, target: '+', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^16\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đạo hàm tổng/hiệu — tách riêng RỒI cộng lại. Hàm HỢP `f(g(x))` — đạo
hàm của NÓ có ĐƠN giản là `f'(g'(x))` không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàm HỢP `f(g(x))` (một hàm LỒNG trong hàm khác, T2.4 khái niệm hợp
THÀNH) — đạo hàm của NÓ có ĐƠN giản là `f'(g'(x))` không, hay cần
THÊM gì?
::::

::::checkpoint{mastery=0.8}
::::
