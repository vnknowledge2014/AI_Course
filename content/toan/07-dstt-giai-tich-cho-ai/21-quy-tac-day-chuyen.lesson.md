---
id: toan.dstt-giai-tich-cho-ai.quy-tac-day-chuyen
title: Quy tắc dây chuyền
summary: "(f∘g)'(x) = f'(g(x))·g'(x) — đạo hàm hàm HỢP LÀ tích của 'đạo hàm NGOÀI tại điểm trong' VÀ 'đạo hàm TRONG'; kiểm bằng xấp xỉ số CHO một ca cụ thể."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.chain-rule]
requires: [math.sum-rule-derivative]
concepts: [math.quy-tac-day-chuyen]
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
Hàm HỢP `f(g(x))` — đạo hàm của NÓ có ĐƠN giản là `f'(g(x))` không,
hay CẦN thêm gì?
::::

::::explain{#quy-tac-day-chuyen}
CẦN thêm — nhân VỚI đạo hàm CỦA hàm TRONG. **`(f∘g)'(x) = f'(g(x))
·g'(x)`** — đạo hàm hàm HỢP LÀ tích của "đạo hàm NGOÀI tại điểm
TRONG" VÀ "đạo hàm TRONG":

```python title=readonly
def g(x):
    return x ** 2

def f_dao_ham(u):
    return 2 * u

def g_dao_ham(x):
    return 2 * x

def day_chuyen(x):
    return f_dao_ham(g(x)) * g_dao_ham(x)


print(day_chuyen(2))
```

```text title=readonly
32
```

`h(x) = (x²)² = f(g(x))` VỚI `f(u)=u², g(x)=x²`. TẠI `x=2`:
`g(2)=4`, `f'(g(2)) = f'(4) = 2×4 = 8`; `g'(2) = 2×2 = 4`. NHÂN LẠI:
`8×4 = 32`.
::::

::::example{#kiem-lai-bang-xap-xi}
KIỂM lại BẰNG xấp xỉ SỐ TRÊN hàm HỢP `(x²)²`:

```python title=readonly
def g(x):
    return x ** 2

def f_dao_ham(u):
    return 2 * u

def g_dao_ham(x):
    return 2 * x

def day_chuyen(x):
    return f_dao_ham(g(x)) * g_dao_ham(x)

def dao_ham_xap_xi(f, x, dx=0.000001):
    return (f(x + dx) - f(x)) / dx

def h(x):
    return (x ** 2) ** 2


print(day_chuyen(2))
print(round(dao_ham_xap_xi(h, 2), 2))
```

```text title=readonly
32
32.0
```

CÔNG thức dây CHUYỀN (`32`) VÀ xấp xỉ SỐ trên `h` GỘP (`32.0`) —
KHỚP nhau.
::::

::::predict{#doan-diem-dao-ham-bang-0 commitOnce}
Byte tính `day_chuyen(0)` — hàm `h(x)=(x²)² CÓ đúng MỘT điểm mà đạo
hàm bằng `0`:

```python
def g(x):
    return x ** 2

def f_dao_ham(u):
    return 2 * u

def g_dao_ham(x):
    return 2 * x

def day_chuyen(x):
    return f_dao_ham(g(x)) * g_dao_ham(x)

print(day_chuyen(0))
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
Máy báo lỗi khi chạy — `x=0` LÀM `g_dao_ham(0) = 2×0 = 0`, VÀ nhân
VỚI `0` trong công thức dây CHUYỀN gây ra LỖI CHIA-cho-0 ẩn BÊN
trong
::why
Gần đúng ở việc bạn để ý ĐÚNG `g_dao_ham(0)=0` sẽ xuất HIỆN trong
phép NHÂN — một quan sát VỀ giá TRỊ trung gian chính xác.

Chỗ lệch: `day_chuyen` KHÔNG hề chia CHO bất kỳ giá trị nào — NÓ
CHỈ nhân (`f_dao_ham(g(x)) * g_dao_ham(x)`), VÀ nhân VỚI `0` HOÀN
TOÀN hợp lệ, chỉ đơn giản CHO kết quả `0`. KHÔNG có phép chia NÀO
ẩn bên trong. Biên dịch sạch, chạy sạch.
::
:::

:::opt
`4` — vì `f_dao_ham(g(0)) = f_dao_ham(0) = 2×0 = 0`, NHƯNG hàm
"TỰ SỬA" giá trị NHÂN thành đạo hàm TẠI điểm GẦN `0` nhất thay VÌ
trả VỀ `0` thẳng
::why
Gần đúng ở việc bạn nghĩ TỚI một CƠ chế "tự sửa" khi gặp GIÁ trị
`0` — một trực GIÁC AN toàn nhưng KHÔNG khớp CÁCH Python hoạt động.

Chỗ lệch: Python KHÔNG "tự sửa" gì cả — TÍNH ĐÚNG NHƯ công thức
viết RA: `f_dao_ham(g(0)) = f_dao_ham(0) = 0`, `g_dao_ham(0) = 0`,
NHÂN LẠI: `0×0 = 0`. Kết quả LÀ `0` — điểm ĐÓ (`x=0`) chính LÀ nơi
HÀM `(x²)²` chạm ĐÁY (bài 22 sẽ đặt tên: điểm cực TIỂU).
::
:::
::::

::::code{#viet_day_chuyen}
Viết `day_chuyen(x)` — đạo hàm của `(x²)²` tại `x`, theo quy tắc
dây chuyền.

```python title=starter
def g(x):
    return x ** 2

def f_dao_ham(u):
    return 2 * u

def g_dao_ham(x):
    return 2 * x

def day_chuyen(x):
    return ___


print(day_chuyen(2))
```

```python title=solution
def g(x):
    return x ** 2

def f_dao_ham(u):
    return 2 * u

def g_dao_ham(x):
    return 2 * x

def day_chuyen(x):
    return f_dao_ham(g(x)) * g_dao_ham(x)


print(day_chuyen(2))
```

```python title=test
assert day_chuyen(2) == 32, "x=2"
assert day_chuyen(1) == 4, "x=1"
assert day_chuyen(0) == 0, "x=0 -- diem cuc tri"
assert day_chuyen(3) == 108, "x=3"
```

:::hints
- kind: attention
  body: "Nhan f_dao_ham(g(x)) voi g_dao_ham(x) -- dao ham ngoai tai diem trong, nhan dao ham trong."
- kind: strategy
  body: "f_dao_ham(g(x)) * g_dao_ham(x)"
- kind: one-line
  body: "___ = f_dao_ham(g(x)) * g_dao_ham(x)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan f_dao_ham(g(x)) voi g_dao_ham(x)
  requireAst:
  - kind: uses-call, target: f_dao_ham, min: 1
  - kind: uses-call, target: g_dao_ham, min: 1
  - kind: uses-operator, target: '*', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^32\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Quy tắc dây chuyền — nhân đạo hàm TRONG lẫn NGOÀI. Điểm `x=0` LÀ
điểm gì — cực đại, cực tiểu, hay KHÔNG đổi?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàm `f(x) = (x²)²` CÓ đúng MỘT điểm mà đạo hàm BẰNG `0` — điểm ĐÓ có
Ý nghĩa GÌ (lớn nhất? nhỏ nhất? không đổi?)
::::

::::checkpoint{mastery=0.8}
::::
