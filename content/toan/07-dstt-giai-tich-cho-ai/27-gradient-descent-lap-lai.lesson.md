---
id: toan.dstt-giai-tich-cho-ai.gradient-descent-lap-lai
title: Gradient descent lặp lại
summary: "Lặp LẠI x ← x − α∇f(x) cho tới khi ∇f(x) GẦN 0 (bài 22) — CHÍNH LÀ cách phần LỚN mô hình AI 'học': LẶP hàng NGHÌN bước NHỎ để GIẢM một hàm chi phí."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.iterative-gradient-descent]
requires: [math.gradient-descent-step, logic.loop-invariant]
concepts: [math.gradient-descent-lap-lai]
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
LẶP LẠI một bước NHIỀU lần — kết quả CÓ hội tụ VỀ đúng điểm CỰC
tiểu không?
::::

::::explain{#gradient-descent-lap-lai}
CÓ, hội tụ. **Lặp LẠI `x ← x − α∇f(x)`** cho tới khi `∇f(x)` GẦN
`0` (bài 22) — CHÍNH LÀ cách phần LỚN mô hình AI "học": LẶP hàng
NGHÌN bước NHỎ để GIẢM một hàm chi PHÍ:

```python title=readonly
def gradient_descent(x, y, alpha, so_lan_lap):
    for _ in range(so_lan_lap):
        gx = 2 * x
        gy = 2 * y
        x = x - alpha * gx
        y = y - alpha * gy
    return (x, y)


ket_qua = gradient_descent(3.0, 4.0, 0.1, 50)
print(round(ket_qua[0], 4), round(ket_qua[1], 4))
```

```text title=readonly
0.0 0.0001
```

XUẤT phát TỪ `(3,4)`, LẶP `50` bước NHỎ (`α=0.1`) — điểm HỘI tụ RẤT
gần `(0,0)`, cực TIỂU thật SỰ của `f(x,y)=x²+y²`.
::::

::::example{#lap-nhieu-hon-hoi-tu-hon}
LẶP NHIỀU hơn — hội tụ CÀNG sát:

```python title=readonly
def gradient_descent(x, y, alpha, so_lan_lap):
    for _ in range(so_lan_lap):
        gx = 2 * x
        gy = 2 * y
        x = x - alpha * gx
        y = y - alpha * gy
    return (x, y)

def f(x, y):
    return x ** 2 + y ** 2


ket_qua = gradient_descent(3.0, 4.0, 0.1, 100)
print(round(ket_qua[0], 6), round(ket_qua[1], 6))
print(round(f(*ket_qua), 6))
```

```text title=readonly
0.0 0.0
0.0
```

VỚI `100` bước, điểm HỘI tụ LÀM TRÒN CHÍNH XÁC về `(0,0)`, VÀ hàm
chi PHÍ `f` cũng VỀ `0` — đây CHÍNH LÀ Ý tưởng cốt LÕI: LẶP nhiều
bước NHỎ, hội tụ dần VỀ cực tiểu.
::::

::::predict{#doan-alpha-qua-lon-phan-ky commitOnce}
Byte thử `α=1.2` (LỚN hơn `1.0`, đã THẤY LÀ "vọt qua" Ở bài 26) —
LẶP `5` bước:

```python
def gradient_descent(x, y, alpha, so_lan_lap):
    for _ in range(so_lan_lap):
        gx = 2 * x
        gy = 2 * y
        x = x - alpha * gx
        y = y - alpha * gy
    return (x, y)

print(gradient_descent(3.0, 4.0, 1.2, 5))
```

Dòng cuối CÓ in ra một điểm GẦN `(0,0)` không?

:::opt{correct}
KHÔNG — điểm XA rời gốc NGÀY càng lớn (PHÂN kỳ)
:::

:::opt
CÓ — vì gradient DESCENT LUÔN hội tụ VỀ cực tiểu, KHÔNG quan tâm
`α` lớn HAY nhỏ, CHỈ cần lặp ĐỦ nhiều lần
::why
Gần đúng ở việc bạn tin TƯỞNG vào SỨC mạnh của việc "lặp nhiều lần"
— MỘT trực giác đúng KHI `α` đủ NHỎ.

Chỗ lệch: `α=1.2` LỚN hơn `1.0` (đã thấy Ở bài 26 LÀ "vọt qua đúng
điểm đối XỨNG") — MỖI bước GIỜ "vọt quá xa" theo chiều NGƯỢC lại,
RỒI lại "vọt quá xa" tiếp Ở bước SAU, CÀNG lặp CÀNG XA gốc — đây LÀ
hiện tượng **phân kỳ**. LẶP nhiều LẦN chỉ giúp khi `α` đủ NHỎ để
MỖI bước tiến GẦN hơn (bài 26); `α` quá LỚN thì lặp CÀNG nhiều CÀNG
tệ.
::
:::

:::opt
Máy báo lỗi khi chạy — `α=1.2` LỚN hơn `1.0`, Python giới hạn `α`
trong khoảng `[0, 1]` cho các hàm liên QUAN tới tối ưu HOÁ
::why
Gần đúng ở việc bạn để ý ĐÚNG `α=1.2` VƯỢT quá `1.0` — một quan sát
VỀ độ LỚN.

Chỗ lệch: Python KHÔNG hề giới hạn GIÁ trị `alpha` — hàm nhận BẤT
kỳ số nào, TÍNH toán BÌNH thường (dù kết quả TOÁN học có PHÂN kỳ).
Biên dịch sạch, chạy sạch — CHỈ LÀ kết quả "tệ" VỀ mặt tối ưu hoá.
::
:::
::::

::::code{#viet_gradient_descent}
Viết `gradient_descent(x, y, alpha, so_lan_lap)` — lặp `so_lan_lap`
bước gradient descent CHO `f(x,y)=x²+y²`.

```python title=starter
def gradient_descent(x, y, alpha, so_lan_lap):
    for _ in range(so_lan_lap):
        gx = 2 * x
        gy = 2 * y
        ___
    return (x, y)


print(gradient_descent(3.0, 4.0, 0.1, 1))
```

```python title=solution
def gradient_descent(x, y, alpha, so_lan_lap):
    for _ in range(so_lan_lap):
        gx = 2 * x
        gy = 2 * y
        x = x - alpha * gx
        y = y - alpha * gy
    return (x, y)


print(gradient_descent(3.0, 4.0, 0.1, 1))
```

```python title=test
assert gradient_descent(3.0, 4.0, 0.1, 0) == (3.0, 4.0), "khong lap lan nao -- khong doi"
assert gradient_descent(3.0, 4.0, 0.1, 1) == (2.4, 3.2), "mot lan lap"
ket_qua = gradient_descent(3.0, 4.0, 0.1, 100)
assert round(ket_qua[0], 6) == 0.0 and round(ket_qua[1], 6) == 0.0, "hoi tu ve cuc tieu"
xa, ya = gradient_descent(3.0, 4.0, 1.2, 5)
assert xa ** 2 + ya ** 2 > 100.0, "alpha qua lon -- phan ky, cang xa goc"
```

:::hints
- kind: attention
  body: "Cap nhat CA x va y: x = x - alpha*gx, y = y - alpha*gy (dung ca hai dong)."
- kind: strategy
  body: "x = x - alpha * gx; y = y - alpha * gy"
- kind: one-line
  body: "x = x - alpha * gx\n        y = y - alpha * gy"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cap nhat ca x va y bang buoc gradient descent (bai 26)
  requireAst:
  - kind: uses-name, target: gx, min: 1
  - kind: uses-name, target: gy, min: 1
  - kind: gan-ten, target: x, min: 1
  - kind: gan-ten, target: y, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(2\.4, 3\.2\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Track NÀY đóng bằng gradient descent — thuật toán CỐT LÕI của AI
hiện đại. VECTOR, tích vô hướng, đạo hàm, gradient — TẤT CẢ GẶP
nhau Ở đâu trên MỘT bài toán DUY nhất?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track NÀY đóng bằng gradient descent — thuật toán CỐT LÕI của AI
hiện đại. VECTOR, tích vô hướng, đạo hàm, gradient — TẤT CẢ GẶP
nhau Ở đâu trên MỘT bài toán DUY nhất?
::::

::::checkpoint{mastery=0.8}
::::
