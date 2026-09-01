---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.reduce-gop-mot-day-thanh-mot-gia-tri
title: "`reduce()` — gộp một dãy thành MỘT giá trị"
summary: "from functools import reduce; reduce(lambda acc, x: acc + x, [1,2,3,4,5], 0) == 15 — acc mang giá trị TỪ BƯỚC TRƯỚC sang bước sau. Khác map/filter, reduce gộp NHIỀU thành MỘT."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [fp.reduce-basics]
requires: [fp.map-filter-vs-comprehension]
concepts: [fp.reduce-basics]
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
`map()`/`filter()` luôn trả về một dãy (dài hơn, ngắn hơn, hoặc bằng).
Hôm nay: một công cụ gộp CẢ dãy lại thành đúng MỘT giá trị.
::::

::::explain{#reduce-la-gi}
`reduce(f, xs, khoi_dau)` gộp dần TỪNG phần tử của `xs` vào một bộ
TÍCH LUỸ, bắt đầu từ `khoi_dau`:

```python
from functools import reduce

xs = [1, 2, 3, 4, 5]
tong = reduce(lambda acc, x: acc + x, xs, 0)
print(tong)
```

```text
15
```

Cách đọc: bắt đầu `acc = 0` (giá trị `khoi_dau`). Với TỪNG `x` trong
`xs`, tính `acc = acc + x`, rồi DÙNG kết quả đó cho bước tiếp theo. Sau
khi hết `xs`, `acc` là kết quả cuối cùng.

`reduce` không CHỈ cộng — bất kỳ phép GỘP nào cũng viết được:

```python
tich = reduce(lambda acc, x: acc * x, xs, 1)
print(tich)
```

```text
120
```

`khoi_dau` là `1` (không phải `0`) vì `0` sẽ làm MỌI phép nhân ra `0` —
`khoi_dau` phải là "giá trị trung tính" của phép gộp (`0` cho cộng, `1`
cho nhân).
::::

::::example{#tung-buoc-reduce}
Theo dõi `acc` qua TỪNG bước, để thấy `reduce` thật sự làm gì:

```python title=readonly
from functools import reduce

def cong_co_in(acc, x):
    ket_qua = acc + x
    print(f"acc={acc}, x={x} -> {ket_qua}")
    return ket_qua

reduce(cong_co_in, [1, 2, 3, 4, 5], 0)
```

```text title=readonly
acc=0, x=1 -> 1
acc=1, x=2 -> 3
acc=3, x=3 -> 6
acc=6, x=4 -> 10
acc=10, x=5 -> 15
```

Năm bước, mỗi bước lấy `acc` của bước TRƯỚC làm điểm bắt đầu. `x` chạy
qua từng phần tử của `[1,2,3,4,5]`. Kết quả CUỐI (`15`) là giá trị
`reduce()` trả về.
::::

::::predict{#doan-khoi-dau-sai commitOnce}
```python
from functools import reduce

xs = [2, 4, 6]

tong_dung = reduce(lambda acc, x: acc + x, xs, 0)
tong_sai = reduce(lambda acc, x: acc + x, xs, 100)

print(tong_dung)
print(tong_sai)
```

Hai dòng cuối in ra gì?

:::opt{correct}
`12` rồi `112`
:::

:::opt
`12` rồi `12` — vì phép cộng CUỐI CÙNG cho ra kết quả giống nhau, bất kể
`khoi_dau` là gì
::why
Gần đúng ở việc bạn tính đúng `2+4+6=12` cho `tong_dung` — phép cộng đó
đúng.

Chỗ lệch: `khoi_dau` KHÔNG bị "gạt bỏ" ở cuối — nó là điểm XUẤT PHÁT của
`acc`, cộng dồn CÙNG với mọi phần tử. `tong_sai` bắt đầu từ `100`, cộng
thêm `2+4+6=12`, ra `112` — không quay lại `12`.
::
:::

:::opt
Máy báo lỗi — `khoi_dau=100` không hợp lệ vì không khớp giá trị nào
trong `xs`
::why
Gần đúng ở việc bạn nghi ngờ có gì đó "không khớp" với `100` — có sự
khác biệt thật, nhưng không phải lỗi.

Chỗ lệch: `khoi_dau` KHÔNG cần khớp bất kỳ phần tử nào của `xs` — nó chỉ
là điểm XUẤT PHÁT của `acc`, một giá trị hoàn toàn độc lập, chọn tuỳ ý
theo Ý của phép gộp (`0` cho cộng, `1` cho nhân, hay bất kỳ số nào nếu
có lý do).
::
:::
::::

::::code{#tong_don_hang}
Viết `tong_don_hang(gia_ds)` — cộng dồn TOÀN BỘ giá trong `gia_ds`
thành một tổng, dùng `reduce()`.

```python title=starter
from functools import reduce

def tong_don_hang(gia_ds):
    ___

print(tong_don_hang([10000, 20000, 5000]))
```

```python title=solution
from functools import reduce

def tong_don_hang(gia_ds):
    return reduce(lambda acc, g: acc + g, gia_ds, 0)

print(tong_don_hang([10000, 20000, 5000]))
```

```python title=test
assert tong_don_hang([10000, 20000, 5000]) == 35000, "phải cộng dồn đúng tổng ba giá"
assert tong_don_hang([1, 2, 3]) == 6, "phải đúng với một danh sách KHÁC ví dụ trên"
assert tong_don_hang([]) == 0, "danh sách rỗng phải ra 0 — đúng nghĩa khoi_dau"
```

:::hints
- kind: attention
  body: Dùng reduce() với khoi_dau = 0 — reduce yêu cầu import từ functools trước (đã có sẵn ở đầu file, không cần thêm).
- kind: strategy
  body: 'reduce(lambda acc, g: acc + g, gia_ds, 0) — acc tích luỹ tổng, g chạy qua từng giá, 0 là điểm khởi đầu (danh sách rỗng phải ra 0).'
- kind: one-line
  body: "return reduce(lambda acc, g: acc + g, gia_ds, 0)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: tong_don_hang phải dùng reduce() (bài đang dạy đúng công cụ đó), không phải sum() hay vòng lặp viết tay.
  requireAst:
  - kind: uses-call, target: reduce, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "35000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một dãy, gộp dần từng bước, ra đúng MỘT con số. `khoi_dau` không phải
chi tiết phụ — nó quyết định cả kết quả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`reduce()` trông như phép màu — nhưng nó có THẬT SỰ là một công cụ mới,
hay chỉ là một vòng lặp quen thuộc, viết gọn lại?

Bài sau trả lời — bằng cách TỰ VIẾT một bản `reduce` từ số 0.
::::

::::checkpoint{mastery=0.8}
::::
