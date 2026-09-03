---
id: toan.dstt-giai-tich-cho-ai.mot-buoc-gradient-descent
title: Một bước gradient descent
summary: "x_mới = x − α·∇f(x) — α (tốc độ HỌC) LÀ một bước NHỎ theo hướng NGƯỢC gradient (bài 25); MỘT bước ĐƯA x TỚI gần cực tiểu hơn, chưa TỚI hẳn."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.gradient-descent-step]
requires: [math.negative-gradient-direction, math.average-rate-of-change]
concepts: [math.mot-buoc-gradient-descent]
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
Đi một BƯỚC ngược gradient — bước ĐÓ nên DÀI bao nhiêu?
::::

::::explain{#mot-buoc-gradient-descent}
MỘT bước NHỎ, KHÔNG đi hết. **`x_mới = x − α·∇f(x)`** — `α` (TỐC
độ HỌC) LÀ hệ số điều chỉnh ĐỘ dài bước, đi THEO hướng NGƯỢC
gradient (bài 25); MỘT bước đưa `x` TỚI gần cực tiểu HƠN, CHƯA tới
HẲN:

```python title=readonly
def mot_buoc(x, y, gx, gy, alpha):
    return (x - alpha * gx, y - alpha * gy)


print(mot_buoc(3.0, 4.0, 6.0, 8.0, 0.1))
```

```text title=readonly
(2.4, 3.2)
```

TẠI `(3,4)`, gradient LÀ `(6,8)` (bài 24). VỚI `α=0.1`: `x_mới =
3−0.1×6=2.4`, `y_mới=4−0.1×8=3.2`. Điểm MỚI `(2.4, 3.2)` GẦN cực
tiểu `(0,0)` HƠN điểm CŨ.
::::

::::example{#buoc-dung-cho-truong-hop-nay}
VỚI `f(x,y)=x²+y²`, có MỘT `α` đưa THẲNG TỚI cực tiểu CHỈ sau MỘT
bước:

```python title=readonly
def mot_buoc(x, y, gx, gy, alpha):
    return (x - alpha * gx, y - alpha * gy)


print(mot_buoc(3.0, 4.0, 6.0, 8.0, 0.5))
```

```text title=readonly
(0.0, 0.0)
```

VỚI `α=0.5`, điểm MỚI CHÍNH LÀ `(0,0)` — cực tiểu THẬT sự, ĐẠT
được CHỈ sau MỘT bước DUY nhất! Đây LÀ MỘT trường hợp MAY mắn, đặc
THÙ của HÀM parabol ĐƠN giản NÀY, KHÔNG phải LUÔN xảy ra.
::::

::::predict{#doan-alpha-qua-lon commitOnce}
Byte thử `α=1.0` — LỚN hơn hẳn LẦN TRƯỚC:

```python
def mot_buoc(x, y, gx, gy, alpha):
    return (x - alpha * gx, y - alpha * gy)

def f(x, y):
    return x ** 2 + y ** 2

x1, y1 = mot_buoc(3.0, 4.0, 6.0, 8.0, 1.0)
print(f(3.0, 4.0))
print(f(x1, y1))
```

HAI dòng in ra CÓ giống nhau không?

:::opt{correct}
CÓ, CẢ hai đều LÀ `25.0`
:::

:::opt
KHÔNG, dòng THỨ hai NHỎ hơn (BƯỚC lớn hơn thì TIẾN gần cực tiểu
NHANH hơn, `f` PHẢI giảm NHIỀU hơn)
::why
Gần đúng ở việc bạn nghĩ "BƯỚC lớn hơn LUÔN tốt HƠN" — một trực
giác hợp LÝ khi CHƯA thấy hiện tượng "vọt QUA" (overshoot).

Chỗ lệch: `α=1.0` LỚN đến mức VỌT hẳn qua cực TIỂU — điểm MỚI LÀ
`(-3.0, -4.0)`, đối XỨNG QUA gốc VỚI điểm ĐẦU `(3.0, 4.0)`. VÌ
`f(x,y)=x²+y²` đối xứng (bình phương XOÁ dấu), `f(-3,-4) = f(3,4) =
25.0` — HOÀN toàn KHÔNG có tiến bộ NÀO, dù ĐÃ "đi" một bước LỚN.
Bước QUÁ lớn CÓ thể làm gradient DESCENT dậm CHÂN tại chỗ (hoặc TỆ
hơn, phân KỲ).
::
:::

:::opt
Máy báo lỗi khi chạy — `α=1.0` LÀM `x_mới` VÀ `y_mới` trở thành số
ÂM (`-3.0`, `-4.0`), VÀ `f(x,y)=x²+y²` KHÔNG nhận đối số âm
::why
Gần đúng ở việc bạn để ý ĐÚNG kết quả LÀ số ÂM — một quan sát VỀ
giá TRỊ chính xác.

Chỗ lệch: `f(x,y)=x²+y²` BÌNH phương CẢ `x` LẪN `y`, nhận đối số ÂM
HOÀN toàn BÌNH thường (bình phương XOÁ dấu). Biên dịch sạch, chạy
sạch.
::
:::
::::

::::code{#viet_mot_buoc}
Viết `mot_buoc(x, y, gx, gy, alpha)` — MỘT bước gradient DESCENT.

```python title=starter
def mot_buoc(x, y, gx, gy, alpha):
    return ___


print(mot_buoc(3.0, 4.0, 6.0, 8.0, 0.1))
```

```python title=solution
def mot_buoc(x, y, gx, gy, alpha):
    return (x - alpha * gx, y - alpha * gy)


print(mot_buoc(3.0, 4.0, 6.0, 8.0, 0.1))
```

```python title=test
assert mot_buoc(3.0, 4.0, 6.0, 8.0, 0.1) == (2.4, 3.2), "buoc nho"
assert mot_buoc(3.0, 4.0, 6.0, 8.0, 0.5) == (0.0, 0.0), "buoc vua dung -- toi cuc tieu"
assert mot_buoc(3.0, 4.0, 6.0, 8.0, 1.0) == (-3.0, -4.0), "buoc qua lon -- vot qua"
assert mot_buoc(3.0, 4.0, 0.0, 0.0, 0.5) == (3.0, 4.0), "gradient khong -- khong doi"
```

:::hints
- kind: attention
  body: "Tru alpha nhan gradient khoi tung toa do: x - alpha*gx, y - alpha*gy."
- kind: strategy
  body: "(x - alpha * gx, y - alpha * gy)"
- kind: one-line
  body: "___ = (x - alpha * gx, y - alpha * gy)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tinh (x - alpha*gx, y - alpha*gy)
  requireAst:
  - kind: uses-operator, target: '-', min: 2
  - kind: uses-operator, target: '*', min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(2\.4, 3\.2\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bước gradient descent — CÓ thể tới đích, CÓ thể vọt QUA. Lặp
LẠI nhiều bước — kết quả có HỘI tụ VỀ đúng cực tiểu không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

LẶP LẠI một bước NHIỀU lần — kết quả CÓ hội tụ VỀ đúng điểm CỰC tiểu
không? Thử `α` quá LỚN thì SAO?
::::

::::checkpoint{mastery=0.8}
::::
