---
id: toan.dstt-giai-tich-cho-ai.goc-giua-hai-vector
title: Góc giữa hai vector
summary: "cos θ = (u·v)/(‖u‖‖v‖) — tích vô hướng CHIA cho tích hai độ dài RA cosin của GÓC giữa chúng; cos θ = 0 nghĩa LÀ VUÔNG góc."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.vector-angle]
requires: [math.dot-product, math.vector-norm]
concepts: [math.goc-vector]
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
`u·v` VÀ `v·u` — có bằng NHAU không? Con số `u·v` LỚN nghĩa LÀ gì VỀ
mặt HÌNH học?
::::

::::explain{#goc-giua-hai-vector}
`u·v` GIAO hoán (đổi thứ tự KHÔNG đổi kết quả — TỪNG số hạng nhân
VẪN vậy). Con số ĐÓ đo GÓC: **`cos θ = (u·v)/(‖u‖‖v‖)`** — tích vô
hướng CHIA cho tích hai độ DÀI ra cosin của GÓC `θ` giữa chúng:

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def cos_goc(u, v):
    return tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))


print(cos_goc((3.0, 4.0), (3.0, 4.0)))
```

```text title=readonly
1.0
```

MỘT vector SO với CHÍNH nó — GÓC LÀ `0°`, `cos 0° = 1`. `cos θ = 1`
nghĩa LÀ HAI vector CÙNG hướng HOÀN toàn.
::::

::::example{#vuong-goc-va-nguoc-huong}
`cos θ = 0` VUÔNG góc; `cos θ = -1` NGƯỢC hướng HOÀN toàn:

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def cos_goc(u, v):
    return tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))


print(cos_goc((1.0, 0.0), (0.0, 1.0)))
print(cos_goc((3.0, 4.0), (-3.0, -4.0)))
```

```text title=readonly
0.0
-1.0
```

`(1,0)` VÀ `(0,1)` VUÔNG góc (`90°`, `cos 90°=0`, ĐÚNG tích vô hướng
`0` — bài 6). `(3,4)` VÀ `(-3,-4)` NGƯỢC hướng HOÀN toàn (`180°`,
`cos 180°=-1`).
::::

::::predict{#doan-goc-45-do commitOnce}
Byte tính góc giữa `(1.0, 0.0)` VÀ `(1.0, 1.0)` — MỘT góc `45°`:

```python
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def cos_goc(u, v):
    return tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))

print(round(cos_goc((1.0, 0.0), (1.0, 1.0)), 4))
```

Dòng cuối in ra gì?

:::opt{correct}
`0.7071`
:::

:::opt
`0.5` — vì `45°` LÀ đúng NỬA của `90°`, VÀ `cos 90°=0`, `cos 0°=1`,
nên `cos 45°` phải LÀ TRUNG bình cộng của HAI giá trị ĐÓ: `0.5`
::why
Gần đúng ở việc bạn nghĩ TỚI phép NỘI suy tuyến TÍNH (giữa hai mốc
đã BIẾT) — một CÁCH ước lượng hợp LÝ cho nhiều đại lượng.

Chỗ lệch: `cos` KHÔNG biến thiên TUYẾN tính THEO góc — nó LÀ một
đường CONG (sẽ học kỹ hơn Ở giải TÍCH, cụm sau). `cos 45°` = `√2/2
≈ 0.7071`, KHÔNG phải trung bình CỘNG đơn giản của `cos 0°` VÀ
`cos 90°`.
::
:::

:::opt
Máy báo lỗi khi chạy — `round(cos_goc(...), 4)` cố LÀM TRÒN một kết
quả PHÉP chia, mà `round()` chỉ hoạt ĐỘNG trên số nguyên
::why
Gần đúng ở việc bạn để ý ĐÚNG `cos_goc` trả VỀ kết quả TỪ phép chia
(`/`) — một quan sát VỀ nguồn GỐC của giá trị.

Chỗ lệch: `round()` HOẠT động HOÀN TOÀN bình thường trên số THẬP
phân (kết quả CỦA `/`), KHÔNG chỉ số nguyên — đây chính LÀ công
DỤNG chính của `round()`. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_cos_goc}
Viết `cos_goc(u, v)` — tính cosin của GÓC giữa hai vector.

```python title=starter
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def cos_goc(u, v):
    return ___


print(cos_goc((3.0, 4.0), (3.0, 4.0)))
```

```python title=solution
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def cos_goc(u, v):
    return tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))


print(cos_goc((3.0, 4.0), (3.0, 4.0)))
```

```python title=test
assert cos_goc((3.0, 4.0), (3.0, 4.0)) == 1.0, "cung huong -- cos 0"
assert cos_goc((1.0, 0.0), (0.0, 1.0)) == 0.0, "vuong goc -- cos 0"
assert cos_goc((3.0, 4.0), (-3.0, -4.0)) == -1.0, "nguoc huong -- cos -1"
assert cos_goc((3.0, 4.0), (6.0, 8.0)) == 1.0, "cung huong, khac do dai -- van cos 1"
assert round(cos_goc((1.0, 0.0), (1.0, 1.0)), 4) == 0.7071, "goc 45 do"
```

:::hints
- kind: attention
  body: "Chia tich vo huong cho tich hai do dai: tich_vo_huong(u,v) / (do_dai(u) * do_dai(v))."
- kind: strategy
  body: "tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))"
- kind: one-line
  body: "___ = tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai chia tich_vo_huong(u,v) cho do_dai(u)*do_dai(v)
  requireAst:
  - kind: uses-call, target: tich_vo_huong, min: 1
  - kind: uses-call, target: do_dai, min: 2
  - kind: uses-operator, target: '/', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^1\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Góc giữa hai vector — đo BẰNG cosin. Có cách nào dùng `cos θ` LÀM
thước đo "giống nhau" CHUẨN hoá, không phụ thuộc ĐỘ LỚN?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai luống hồ sơ VUÔNG góc (`cos θ = 0`) — chúng "khác nhau HOÀN
TOÀN" theo nghĩa NÀO? Có cách NÀO dùng `cos θ` làm thước đo "GIỐNG
nhau" chuẩn HOÁ, không phụ thuộc ĐỘ LỚN?
::::

::::checkpoint{mastery=0.8}
::::
