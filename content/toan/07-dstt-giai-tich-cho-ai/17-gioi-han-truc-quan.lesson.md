---
id: toan.dstt-giai-tich-cho-ai.gioi-han-truc-quan
title: Giới hạn trực quan
summary: "Giới hạn — khi Δx CÀNG nhỏ, Δy/Δx CÀNG tiến GẦN một con số CỐ ĐỊNH (kiểm bằng CODE: thử Δx=0.1, 0.01, 0.001, thấy dãy SỐ hội tụ); KHÔNG chứng minh ε-δ, chỉ QUAN sát số."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.limit-intuitive]
requires: [math.average-rate-of-change]
concepts: [math.gioi-han]
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
Đo tốc độ lớn GIỮA ngày `1` VÀ ngày `1.001` — con số CÓ ổn định LẠI
một giá trị, hay cứ đổi MÃI khi khoảng cách càng NHỎ?
::::

::::explain{#gioi-han-truc-quan}
Ổn định LẠI. **Giới hạn** — khi `Δx` CÀNG nhỏ, `Δy/Δx` CÀNG tiến
GẦN một con số CỐ ĐỊNH (kiểm bằng CODE: thử `Δx=0.1, 0.01, 0.001`,
THẤY dãy số HỘI tụ; track NÀY KHÔNG chứng minh ε-δ, CHỈ quan sát
SỐ):

```python title=readonly
def f(x):
    return x ** 2

def xap_xi(f, x, dx):
    return (f(x + dx) - f(x)) / dx


for dx in [0.1, 0.01, 0.001, 0.0001]:
    print(dx, "->", round(xap_xi(f, 3, dx), 4))
```

```text title=readonly
0.1 -> 6.1
0.01 -> 6.01
0.001 -> 6.001
0.0001 -> 6.0001
```

`Δx` CÀNG nhỏ, kết quả CÀNG gần `6` — `6.1, 6.01, 6.001, 6.0001` LÀ
một dãy HỘI TỤ VỀ `6`. Con số `6` ĐÓ chính LÀ tốc độ thay đổi TỨC
THỜI tại `x=3` của `f(x)=x²`.
::::

::::example{#ham-tuyen-tinh-khong-can-gioi-han}
Với hàm TUYẾN tính, độ DỐC KHÔNG đổi — KHÔNG cần `Δx` nhỏ:

```python title=readonly
def g(x):
    return 3 * x + 1

def xap_xi(f, x, dx):
    return (f(x + dx) - f(x)) / dx


print(xap_xi(g, 5, 10))
```

```text title=readonly
3.0
```

`g(x)=3x+1` LÀ một đường THẲNG — độ dốc LUÔN LÀ `3`, DÙ `Δx` LỚN
đến `10`. HÀM đường thẳng KHÔNG cần "thu nhỏ `Δx`" để tìm độ dốc —
CHỈ hàm CONG (như `x²`) mới CẦN giới hạn.
::::

::::predict{#doan-dx-cang-nho commitOnce}
Byte thử `f(x)=x²` TẠI `x=5`, VỚI `Δx` NHỎ hơn NỮA:

```python
def f(x):
    return x ** 2

def xap_xi(f, x, dx):
    return (f(x + dx) - f(x)) / dx

print(round(xap_xi(f, 5, 0.001), 2))
```

Dòng cuối in ra gì (GỢI ý: quy tắc `2x` từ ví DỤ TRƯỚC, TẠI `x=3`
cho `6`)?

:::opt{correct}
`10.0`
:::

:::opt
`6.0` — vì `Δx=0.001` giống HỆT ví dụ TRƯỚC (`x=3` cho kết quả
`6`), NÊN bất kỳ điểm `x` NÀO cũng hội tụ VỀ CÙNG một con số
::why
Gần đúng ở việc bạn nhớ ĐÚNG kết quả VÍ dụ TRƯỚC (`6`, tại `x=3`) —
một quan sát chính xác VỀ bài học TRƯỚC.

Chỗ lệch: giới hạn HỘI tụ VỀ một con số PHỤ thuộc VÀO `x` (điểm
ĐANG xét), KHÔNG phải MỘT hằng số CỐ định cho MỌI điểm. Tại `x=3`,
giới hạn LÀ `6` (`=2×3`); tại `x=5`, giới hạn LÀ `10` (`=2×5`) —
QUY tắc `2x` (bài 18 sẽ đặt TÊN) áp dụng CHO từng điểm RIÊNG.
::
:::

:::opt
Máy báo lỗi khi chạy — `Δx=0.001` QUÁ nhỏ, Python giới hạn ĐỘ chính
xác của phép CHIA cho những số nhỏ hơn `0.01`
::why
Gần đúng ở việc bạn để ý ĐÚNG `0.001` khá NHỎ — một quan sát VỀ độ
LỚN.

Chỗ lệch: Python KHÔNG giới hạn phép CHIA theo ĐỘ nhỏ của số chia
(chỉ CHIA cho ĐÚNG `0` mới lỗi) — `0.001` hoàn TOÀN hợp lệ. Biên
dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_xap_xi}
Viết `xap_xi(f, x, dx)` — tính tốc độ thay đổi XẤP XỈ tại `x` VỚI
bước nhảy `dx`.

```python title=starter
def xap_xi(f, x, dx):
    return ___


def f(x):
    return x ** 2


print(round(xap_xi(f, 3, 0.0001), 2))
```

```python title=solution
def xap_xi(f, x, dx):
    return (f(x + dx) - f(x)) / dx


def f(x):
    return x ** 2


print(round(xap_xi(f, 3, 0.0001), 2))
```

```python title=test
def f(x):
    return x ** 2

def g(x):
    return 3 * x + 1

assert xap_xi(g, 5, 10) == 3.0, "ham tuyen tinh -- doc luon khong doi"
assert round(xap_xi(f, 3, 0.1), 1) == 6.1, "x=3, dx=0.1"
assert round(xap_xi(f, 3, 0.0001), 2) == 6.0, "x=3, dx nho -- hoi tu ve 6"
assert round(xap_xi(f, 5, 0.0001), 2) == 10.0, "x=5, dx nho -- hoi tu ve 10"
```

:::hints
- kind: attention
  body: "Ap dung dinh nghia toc do thay doi (bai 16) voi diem thu hai la x+dx."
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
Giới hạn — con số dãy XẤP XỈ hội tụ VỀ. Con số ĐÓ có TÊN riêng
không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`f(x)=x²` — tính `f'(x)` TẠI vài điểm bằng xấp xỉ SỐ (`Δx` nhỏ). Kết
quả CÓ khớp công thức `2x` không? Có QUY tắc TỔNG quát nào cho `xⁿ`
không?
::::

::::checkpoint{mastery=0.8}
::::
