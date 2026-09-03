---
id: toan.dstt-giai-tich-cho-ai.do-dai-vector
title: Độ dài vector
summary: "‖v‖ = √(v1²+v2²+...) — mở RỘNG định lý Pythagoras (T2.1) lên nhiều chiều; độ dài LÀ 'khoảng cách TỪ gốc'."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.vector-norm]
requires: [math.vector-scalar-multiplication]
concepts: [math.do-dai-vector]
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
So sánh vector `(4.0, 6.0)` VÀ vector `(2.0, 3.0)`. Đo "độ LỚN" một
vector — dùng con số NÀO?
::::

::::explain{#do-dai-vector}
**`‖v‖ = √(v1²+v2²+...)`** — mở RỘNG định lý Pythagoras (T2.1: cạnh
huyền² = cạnh1² + cạnh2²) lên NHIỀU chiều; độ dài LÀ "khoảng cách TỪ
gốc" (điểm `(0,0,...)`) TỚI điểm `v`:

```python title=readonly
def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5


print(do_dai((3.0, 4.0)))
```

```text title=readonly
5.0
```

`3²+4²=9+16=25`, `√25=5` — ĐÚNG tam giác vuông `3-4-5` kinh điển
(T2.1). Vector `(3.0, 4.0)` "dài" `5.0` đơn vị TỪ gốc.
::::

::::example{#do-dai-ba-chieu}
Y HỆT công thức, GIỜ VỚI ba chiều:

```python title=readonly
def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5


luong_1 = (2.0, 3.0, 6.0)
print(do_dai(luong_1))
```

```text title=readonly
7.0
```

`2²+3²+6²=4+9+36=49`, `√49=7` — CÔNG thức KHÔNG đổi GÌ khi thêm
CHIỀU, CHỈ cộng THÊM một số HẠNG bình PHƯƠNG.
::::

::::predict{#doan-do-dai-nhan-doi commitOnce}
Byte tính độ dài của `(6.0, 8.0)` — vector NÀY LÀ `(3.0, 4.0)` nhân
ĐÔI (bài 3):

```python
def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

print(do_dai((6.0, 8.0)))
```

Dòng cuối in ra gì?

:::opt{correct}
`10.0`
:::

:::opt
`25.0` — vì công thức BÌNH phương TỪNG thành phần, VÀ nhân ĐÔI mỗi
thành phần TRƯỚC khi bình phương SẼ làm kết quả TĂNG gấp BỐN lần
(`2²=4`), RỒI khai căn TRẢ về gấp HAI... KHÔNG, gấp `2×2.5=5×5=25.0`
::why
Gần đúng ở việc bạn để ý ĐÚNG phép BÌNH phương LÀM số tăng NHANH
hơn tuyến TÍNH — một quan sát VỀ bản chất của luỹ THỪA hai.

Chỗ lệch: TÍNH trực tiếp: `6²+8²=36+64=100`, `√100=10` — ĐÚNG gấp
đôi `5.0` (độ dài của `(3,4)`), KHÔNG phải `25`. Nhân MỖI thành
phần vector VỚI `k` làm BÌNH phương tăng `k²` lần, NHƯNG khai CĂN
(`√`) đúng LÚC "HUỶ" lại thành `k` lần — `‖k·v‖ = |k|·‖v‖`, TĂNG
TUYẾN tính THEO `k`, không phải `k²`.
::
:::

:::opt
Máy báo lỗi khi chạy — `x ** 2` VÀ `** 0.5` dùng CÙNG toán tử `**`
hai lần TRONG cùng một hàm, Python giới hạn số LẦN dùng MỘT toán tử
trong MỘT hàm
::why
Gần đúng ở việc bạn để ý ĐÚNG `**` xuất hiện HAI lần — một quan sát
VỀ hình thức đúng.

Chỗ lệch: Python KHÔNG giới hạn số LẦN dùng một TOÁN tử — `**` CÓ
thể xuất hiện BAO nhiêu lần TUỲ ý trong MỘT hàm. Biên dịch sạch,
chạy sạch.
::
:::
::::

::::code{#viet_do_dai}
Viết `do_dai(v)` — tính độ dài (Pythagoras mở rộng) của vector `v`.

```python title=starter
def do_dai(v):
    return ___


print(do_dai((3.0, 4.0)))
```

```python title=solution
def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5


print(do_dai((3.0, 4.0)))
```

```python title=test
assert do_dai(()) == 0.0, "vector rong -- do dai 0"
assert do_dai((0.0, 0.0)) == 0.0, "vector khong -- do dai 0"
assert do_dai((3.0, 4.0)) == 5.0, "tam giac 3-4-5"
assert do_dai((2.0, 3.0, 6.0)) == 7.0, "ba chieu"
assert do_dai((6.0, 8.0)) == 10.0, "nhan doi -- do dai cung nhan doi"
```

:::hints
- kind: attention
  body: "Binh phuong tung thanh phan, cong lai, roi khai can (mu 0.5)."
- kind: strategy
  body: "sum(x ** 2 for x in v) ** 0.5"
- kind: one-line
  body: "___ = sum(x ** 2 for x in v) ** 0.5"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai binh phuong tung thanh phan, cong lai, roi khai can
  requireAst:
  - kind: uses-call, target: sum, min: 1
  - kind: uses-operator, target: '**', min: 2
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^5\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Độ dài — khoảng cách TỪ gốc. Hai luống Ở vị trí `(3.0, 4.0)` VÀ
`(0.0, 0.0)` — khoảng cách GIỮA chúng chính LÀ gì?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai luống Ở vị trí `(3.0, 4.0)` VÀ `(0.0, 0.0)` (gốc) — khoảng CÁCH
giữa CHÚNG chính LÀ độ dài vector NÀO?
::::

::::checkpoint{mastery=0.8}
::::
