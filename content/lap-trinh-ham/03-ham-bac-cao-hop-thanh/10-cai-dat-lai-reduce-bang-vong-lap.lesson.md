---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.cai-dat-lai-reduce-bang-vong-lap
title: "`reduce` chỉ là một vòng lặp tích luỹ, viết gọn lại"
summary: "Viết reduce_tu_viet(f, xs, khoi_dau) bằng for thường — chứng minh reduce không phải phép màu, chỉ là acc = khoi_dau; for x in xs: acc = f(acc, x); return acc đóng gói lại."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [fp.reduce-from-scratch]
requires: [fp.reduce-basics]
concepts: [fp.reduce-from-scratch]
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
`reduce()` trông bí ẩn — bạn tự viết được một bản y hệt, chỉ bằng `for`
đã biết từ Realm 1. Byte thách bạn chứng minh điều đó.
::::

::::explain{#reduce-la-mot-vong-lap}
`reduce(f, xs, khoi_dau)` KHÔNG có phép màu — nó chỉ là mẫu hình một
vòng lặp tích luỹ, đóng gói lại thành một hàm:

```python
acc = khoi_dau
for x in xs:
    acc = f(acc, x)
# acc bây giờ CHÍNH LÀ kết quả reduce(f, xs, khoi_dau)
```

Ba dòng này làm ĐÚNG việc `functools.reduce` làm. Gói nó vào một hàm tự
viết:

```python
def reduce_tu_viet(f, xs, khoi_dau):
    acc = khoi_dau
    for x in xs:
        acc = f(acc, x)
    return acc

print(reduce_tu_viet(lambda acc, x: acc + x, [1, 2, 3], 0))
```

```text
6
```

Không phải "giả lập gần đúng" — đây LÀ chính cơ chế `reduce` thật sự
làm bên trong (`functools.reduce` là bản viết bằng C, nhanh hơn, nhưng
CÙNG một Ý). Biết cách tự viết một công cụ nghĩa là hiểu THẬT nó làm gì,
không chỉ nhớ tên gọi.
::::

::::example{#doi-chieu-hai-ban}
Chạy CẢ HAI — bản tự viết và bản thật — trên CÙNG dữ liệu, so sánh:

```python title=readonly
from functools import reduce

def reduce_tu_viet(f, xs, khoi_dau):
    acc = khoi_dau
    for x in xs:
        acc = f(acc, x)
    return acc

xs = [2, 4, 6, 8]

a = reduce_tu_viet(lambda acc, x: acc * x, xs, 1)
b = reduce(lambda acc, x: acc * x, xs, 1)

print(a)
print(a == b)
```

```text title=readonly
384
True
```

Hai hàm, hai cách viết, CÙNG một kết quả — đúng bằng chứng `reduce_tu_viet`
không phải một bản "gần đúng", mà là ĐÚNG hệt cơ chế của `reduce` thật.
::::

::::predict{#doan-thu-tu-tich-luy commitOnce}
```python
def reduce_tu_viet(f, xs, khoi_dau):
    acc = khoi_dau
    for x in xs:
        acc = f(acc, x)
    return acc

noi_chuoi = reduce_tu_viet(lambda acc, s: acc + s, ["a", "b", "c"], "")
print(noi_chuoi)
```

Dòng cuối in ra gì?

:::opt{correct}
`abc`
:::

:::opt
`cba` — vì mỗi bước GHÉP phần tử MỚI vào ĐẦU chuỗi tích luỹ, không phải
cuối
::why
Gần đúng ở việc bạn nghĩ tới thứ tự ghép — đúng là thứ tự quan trọng ở
đây.

Chỗ lệch: `acc + s` ghép `s` vào SAU `acc` (`acc` đứng trước, `+` nối
tiếp), không phải trước. Bước 1: `"" + "a" = "a"`. Bước 2:
`"a" + "b" = "ab"`. Bước 3: `"ab" + "c" = "abc"` — đúng thứ tự phần tử
trong `xs`, không đảo ngược.
::
:::

:::opt
Máy báo lỗi — `khoi_dau=""` (chuỗi rỗng) không dùng được làm giá trị
tích luỹ ban đầu
::why
Gần đúng ở việc bạn nghi ngờ một giá trị "rỗng" có thể gây vấn đề — phản
xạ cẩn trọng hợp lý.

Chỗ lệch: `""` là một giá trị HOÀN TOÀN HỢP LỆ cho `khoi_dau` — nó là
"giá trị trung tính" của phép NỐI CHUỖI (`x + "" == x`, giống `0` cho
cộng số, `1` cho nhân số). `acc = ""` chạy bình thường qua từng bước.
::
:::
::::

::::code{#reduce_tu_viet}
Viết `reduce_tu_viet(f, xs, khoi_dau)` bằng vòng lặp `for` — KHÔNG dùng
`functools.reduce`. Kết quả phải khớp CHÍNH XÁC `reduce` thật.

```python title=starter
def reduce_tu_viet(f, xs, khoi_dau):
    ___

xs = [1, 2, 3, 4, 5]
print(reduce_tu_viet(lambda acc, x: acc + x, xs, 0))
```

```python title=solution
def reduce_tu_viet(f, xs, khoi_dau):
    acc = khoi_dau
    for x in xs:
        acc = f(acc, x)
    return acc

xs = [1, 2, 3, 4, 5]
print(reduce_tu_viet(lambda acc, x: acc + x, xs, 0))
```

```python title=test
from functools import reduce

xs2 = [2, 4, 6, 8]
assert reduce_tu_viet(lambda acc, x: acc + x, xs2, 0) == reduce(lambda acc, x: acc + x, xs2, 0), "phải khớp reduce thật khi cộng"
assert reduce_tu_viet(lambda acc, x: acc * x, xs2, 1) == reduce(lambda acc, x: acc * x, xs2, 1), "phải khớp reduce thật khi nhân"
assert reduce_tu_viet(lambda acc, x: acc + x, [], 99) == 99, "danh sách rỗng phải trả về ĐÚNG khoi_dau, không đổi"
assert reduce_tu_viet(lambda acc, s: acc + s, ["x", "y"], "") == "xy", "phải đúng thứ tự khi gộp chuỗi"
```

:::hints
- kind: attention
  body: Dùng một biến acc bắt đầu từ khoi_dau, vòng qua xs, mỗi bước cập nhật acc = f(acc, x) — không import functools ở đây.
- kind: strategy
  body: 'acc = khoi_dau; for x in xs: acc = f(acc, x); return acc — ba dòng, không hơn. acc PHẢI được gán lại kết quả f(acc, x) mỗi bước, không giữ nguyên khoi_dau.'
- kind: one-line
  body: "acc = khoi_dau\nfor x in xs:\n    acc = f(acc, x)\nreturn acc"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: reduce_tu_viet phải tự cộng dồn bằng vòng lặp (for), KHÔNG được gọi functools.reduce bên trong — đó là điều bài này đang chứng minh (tự viết được, không phải phép màu).
  requireAst:
  - kind: uses-name, target: acc, min: 2
  forbidAst:
  - kind: uses-call, target: reduce
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "15"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không phép màu nào cả — `reduce` là một vòng lặp quen thuộc, chỉ đóng
gói gọn lại thành một hàm dùng được nhiều nơi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba công cụ — `map`, `filter`, `reduce` — đứng riêng lẻ đã hữu ích. Ghép
CẢ BA lại trong một chuỗi xử lý thì trông thế nào?

Bài sau đo đúng điều đó.
::::

::::checkpoint{mastery=0.8}
::::
