---
id: toan.dstt-giai-tich-cho-ai.dao-ham-rieng
title: Đạo hàm riêng
summary: "Đạo hàm RIÊNG ∂f/∂x — đạo hàm THEO một biến, GIỮ các biến KHÁC cố định; hàm f(x,y)=x²+y² CÓ hai đạo hàm riêng, MỘT cho x, MỘT cho y."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.partial-derivative]
requires: [math.critical-point, math.vector]
concepts: [math.dao-ham-rieng]
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
Hàm CHI PHÍ tưới nước CÓ NHIỀU biến (không chỉ MỘT `x`) — "độ dốc"
theo TỪNG biến RIÊNG tính thế NÀO?
::::

::::explain{#dao-ham-rieng}
**Đạo hàm RIÊNG `∂f/∂x`** — đạo hàm THEO một biến, GIỮ các biến
KHÁC CỐ định; hàm `f(x,y)=x²+y²` CÓ hai đạo hàm riêng, MỘT cho `x`,
MỘT cho `y`:

```python title=readonly
def f(x, y):
    return x ** 2 + y ** 2

def dao_ham_rieng_x(f, x, y, dx=0.000001):
    return (f(x + dx, y) - f(x, y)) / dx


print(round(dao_ham_rieng_x(f, 3, 4), 2))
```

```text title=readonly
6.0
```

`∂f/∂x` TẠI `(3,4)` — CHỈ dịch `x` một chút (GIỮ `y=4` CỐ định),
đo TỐC độ thay đổi. Kết quả `6.0 = 2×3`, ĐÚNG quy tắc luỹ thừa (bài
19) áp DỤNG riêng cho `x`, xem `y` NHƯ một HẰNG số.
::::

::::example{#dao-ham-rieng-theo-y}
Đạo hàm riêng THEO `y` — GIỮ `x` cố định:

```python title=readonly
def f(x, y):
    return x ** 2 + y ** 2

def dao_ham_rieng_y(f, x, y, dy=0.000001):
    return (f(x, y + dy) - f(x, y)) / dy


print(round(dao_ham_rieng_y(f, 3, 4), 2))
```

```text title=readonly
8.0
```

`∂f/∂y` TẠI `(3,4)` — CHỈ dịch `y`, KẾT quả `8.0 = 2×4` — hai đạo
hàm RIÊNG (`6.0` cho `x`, `8.0` cho `y`) LÀ hai con số TÁCH biệt,
ĐO hai HƯỚNG khác nhau.
::::

::::predict{#doan-doi-y-khong-doi-x commitOnce}
Byte tính `∂f/∂x` TẠI `x=3`, NHƯNG đổi `y` thành `100` (RẤT khác
`4`):

```python
def f(x, y):
    return x ** 2 + y ** 2

def dao_ham_rieng_x(f, x, y, dx=0.000001):
    return (f(x + dx, y) - f(x, y)) / dx

print(round(dao_ham_rieng_x(f, 3, 100), 2))
```

Dòng cuối in ra gì?

:::opt{correct}
`6.0`
:::

:::opt
`206.0` — vì `y=100` LÀM tổng `f(x,y)` lớn hơn HẲN, VÀ đạo hàm
riêng theo `x` cũng PHẢI tăng THEO cho tương xứng
::why
Gần đúng ở việc bạn để ý ĐÚNG `y=100` làm GIÁ trị `f(x,y)` LỚN hơn
NHIỀU — một quan sát VỀ độ LỚN tổng thể.

Chỗ lệch: `∂f/∂x` đo TỐC độ thay đổi CHỈ khi `x` NHÍCH (`y` GIỮ cố
định, KHÔNG đổi TRONG suốt phép TÍNH), nên GIÁ trị TUYỆT đối của
`y` KHÔNG ảnh hưởng — CHỈ ảnh HƯỞNG nếu `f` có SỐ hạng "chéo" (như
`xy`, hàm NÀY không có). `f(x,y)=x²+y²`, phần `y²` LÀ HẰNG số khi
xét THEO `x`, biến MẤT khi lấy đạo hàm (đạo hàm của một HẰNG số LÀ
`0`) — kết quả VẪN LÀ `6.0`.
::
:::

:::opt
Máy báo lỗi khi chạy — `y=100` VÀ `x=3` chênh LỆCH quá LỚN (gấp Hơn
`33` lần), Python giới hạn TỈ lệ GIỮA hai đối số của MỘT hàm
::why
Gần đúng ở việc bạn để ý ĐÚNG `y` VÀ `x` chênh lệch NHIỀU — một
quan sát VỀ độ LỚN tương đối.

Chỗ lệch: Python KHÔNG hề giới hạn TỈ lệ giữa CÁC đối số — hai
tham số ĐỘC lập HOÀN toàn, chênh lệch BAO nhiêu cũng được. Biên
dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_dao_ham_rieng_x}
Viết `dao_ham_rieng_x(f, x, y, dx=0.000001)` — đạo hàm riêng THEO
`x`, GIỮ `y` cố định.

```python title=starter
def dao_ham_rieng_x(f, x, y, dx=0.000001):
    return ___


def f(x, y):
    return x ** 2 + y ** 2


print(round(dao_ham_rieng_x(f, 3, 4), 2))
```

```python title=solution
def dao_ham_rieng_x(f, x, y, dx=0.000001):
    return (f(x + dx, y) - f(x, y)) / dx


def f(x, y):
    return x ** 2 + y ** 2


print(round(dao_ham_rieng_x(f, 3, 4), 2))
```

```python title=test
def f(x, y):
    return x ** 2 + y ** 2

assert round(dao_ham_rieng_x(f, 3, 4), 2) == 6.0, "x=3, y=4"
assert round(dao_ham_rieng_x(f, 3, 100), 2) == 6.0, "doi y khong anh huong"
assert round(dao_ham_rieng_x(f, 0, 5), 2) == 0.0, "x=0"
assert round(dao_ham_rieng_x(f, 5, 0), 2) == 10.0, "x=5"
```

:::hints
- kind: attention
  body: "Ap dung dinh nghia dao ham (bai 18), CHI dich x, GIU y nguyen."
- kind: strategy
  body: "(f(x + dx, y) - f(x, y)) / dx"
- kind: one-line
  body: "___ = (f(x + dx, y) - f(x, y)) / dx"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tinh (f(x+dx, y) - f(x, y)) / dx, giu y khong doi
  requireAst:
  - kind: uses-call, target: f, min: 2
  - kind: uses-name, target: y, min: 2
  - kind: uses-operator, target: '/', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^6\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đạo hàm riêng — MỘT con số cho MỖI biến. Gộp CẢ hai đạo hàm riêng
lại thành MỘT cấu trúc DUY nhất — cấu trúc ĐÓ giống thứ GÌ đã học Ở
đầu track?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Gộp CẢ hai đạo hàm riêng (`∂f/∂x`, `∂f/∂y`) lại thành MỘT cấu trúc
DUY nhất — cấu trúc ĐÓ giống thứ GÌ đã học Ở đầu track?
::::

::::checkpoint{mastery=0.8}
::::
