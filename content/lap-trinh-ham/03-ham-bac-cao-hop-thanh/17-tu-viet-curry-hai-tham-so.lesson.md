---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.tu-viet-curry-hai-tham-so
title: "Tự viết `curry` — biến `f(a, b)` thành `f(a)(b)`"
summary: "def curry2(f): return lambda a: lambda b: f(a, b) rồi cong = curry2(lambda a,b: a+b); cong(3)(4) == 7 — currying là 'partial application TỰ ĐỘNG', viết được từ đúng những mảnh track đã dạy."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.curry-handwritten]
requires: [fp.functools-partial]
concepts: [fp.curry-handwritten]
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
`partial` chốt sẵn một tham số bằng MỘT lời gọi tường minh. Hôm nay:
một cách viết hàm sao cho việc "chốt dần từng tham số" xảy ra TỰ NHIÊN,
không cần gọi `partial` mỗi lần.
::::

::::explain{#curry-la-gi}
```python
def curry2(f):
    return lambda a: lambda b: f(a, b)

cong = curry2(lambda a, b: a + b)
print(cong(3)(4))
```

```text
7
```

`curry2(f)` biến một hàm HAI tham số (`f(a, b)`) thành một hàm MỘT
tham số, TRẢ VỀ một hàm MỘT tham số khác — `cong(3)` không tính ngay,
nó TRẢ VỀ một closure (bài 12!) đã "nhớ" `a = 3`, chờ nhận `b`. Gọi
tiếp `(4)` lên closure đó mới thực sự tính `f(3, 4)`.

Đây gọi là **currying** — biến `f(a, b)` thành `f(a)(b)`. Không phải
phép màu: `curry2` chỉ là MỘT closure (`lambda a: ...`) TRẢ VỀ một
closure KHÁC (`lambda b: f(a, b)`) — đúng những mảnh bài 12-13 đã dạy,
ghép lại theo một cách dùng mới. Currying là "partial application TỰ
ĐỘNG": thay vì gọi `partial(f, a=3)` tường minh mỗi lần, hàm ĐÃ ĐƯỢC
VIẾT sẵn để nhận đối số MỘT-CÁI-MỘT-LẦN.
::::

::::example{#tach-hai-buoc-goi}
Hai bước gọi TÁCH RỜI được — bước đầu tạo một hàm mới, có thể DÙNG LẠI:

```python title=readonly
def curry2(f):
    return lambda a: lambda b: f(a, b)

cong = curry2(lambda a, b: a + b)

cong_voi_5 = cong(5)

print(cong_voi_5(1))
print(cong_voi_5(2))
print(cong_voi_5(100))
```

```text title=readonly
6
7
105
```

`cong(5)` gọi MỘT LẦN, trả về `cong_voi_5` — một hàm CHỈ CẦN `b`, đã
chốt sẵn `a = 5`. Gọi `cong_voi_5(...)` NHIỀU LẦN với `b` khác nhau,
mỗi lần tính `5 + b` — CHÍNH XÁC cùng khuôn với `partial(luy_thua,
so_mu=2)` ở bài 16 (một hàm "nhớ sẵn" một phần dữ liệu, gọi lại nhiều
lần).
::::

::::predict{#doan-curry-doc-lap commitOnce}
```python
def curry2(f):
    return lambda a: lambda b: f(a, b)

nhan = curry2(lambda a, b: a * b)

nhan_3 = nhan(3)
nhan_7 = nhan(7)

print(nhan_3(2))
print(nhan_7(2))
print(nhan_3(10))
```

Ba dòng in ra gì, theo đúng thứ tự?

:::opt{correct}
`6` rồi `14` rồi `30`
:::

:::opt
`6` rồi `14` rồi `14` — vì tạo `nhan_7` sau đó "cập nhật" giá trị `a`
mà `nhan_3` đang giữ
::why
Gần đúng ở việc bạn tính đúng hai kết quả đầu (`3*2=6`, `7*2=14`) — hai
phép nhân đó đúng.

Chỗ lệch: `nhan_3` và `nhan_7` là HAI closure ĐỘC LẬP (đúng cơ chế bài
12), mỗi cái "nhớ" `a` của riêng LẦN GỌI `nhan(...)` sinh ra nó — tạo
`nhan_7` (nhớ `a=7`) không hề đụng tới `a=3` mà `nhan_3` đang giữ.
`nhan_3(10)` tính `3 * 10 = 30`, không quay lại `14`.
::
:::

:::opt
Máy báo lỗi ở `nhan_3(2)` — `curry2` chỉ tạo được MỘT hàm dùng được,
không dùng lại được cho nhiều lời gọi
::why
Gần đúng ở việc bạn cảnh giác về "dùng lại" một hàm curry nhiều lần —
một mối lo hợp lý nếu chưa chắc closure hoạt động ra sao qua nhiều
tầng gọi.

Chỗ lệch: `nhan_3` (kết quả của `nhan(3)`) là một hàm HOÀN TOÀN BÌNH
THƯỜNG, gọi được bao nhiêu lần tuỳ ý — `nhan_3(2)`, rồi `nhan_3(10)`
sau đó, đều chạy được, mỗi lần tính `3 * b` với `b` mới.
::
:::
::::

::::code{#curry2}
Tự viết `curry2(f)` — biến một hàm hai tham số `f(a, b)` thành dạng
gọi `f(a)(b)`.

```python title=starter
def curry2(f):
    ___

cong = curry2(lambda a, b: a + b)
print(cong(3)(4))
```

```python title=solution
def curry2(f):
    return lambda a: lambda b: f(a, b)

cong = curry2(lambda a, b: a + b)
print(cong(3)(4))
```

```python title=test
nhan = curry2(lambda a, b: a * b)
assert nhan(3)(4) == 12, "curry2 phải áp dụng đúng hàm cho cả hai đối số"
assert cong(3)(4) == 7, "gọi lại curry2(f)(3)(4) phải ra cùng kết quả"
tru = curry2(lambda a, b: a - b)
assert tru(10)(3) == 7, "curry2 phải hoạt động đúng với hàm khác (trừ)"
b5 = cong(5)
assert b5(1) == 6, "cong(5) phải trả về một hàm CHỈ CẦN b, đã chốt sẵn a=5"
assert b5(2) == 7, "gọi lại cong(5) với b khác phải ra kết quả khác, không lặp lại kết quả trước"
```

:::hints
- kind: attention
  body: "curry2(f) phải trả về MỘT hàm nhận a, và hàm đó lại trả về MỘT hàm khác nhận b — hai lambda LỒNG NHAU, không phải một lambda hai tham số."
- kind: strategy
  body: 'return lambda a: lambda b: f(a, b) — lambda ngoài nhận a, TRẢ VỀ lambda trong (đã nhớ a), lambda trong nhận b rồi mới gọi f(a, b).'
- kind: one-line
  body: "return lambda a: lambda b: f(a, b)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: curry2 phải trả về HAI lambda LỒNG NHAU (một hàm nhận a trả về một hàm nhận b) — không phải một lambda hai tham số như lambda a, b, f(a, b)) — đó không phải currying, chỉ là gọi lại f.
  requireAst:
  - kind: lambda, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "7"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Currying không phải khái niệm lạ — nó chỉ là closure trả về closure,
ghép đúng những mảnh đã học, cho một cách dùng gọi hàm mới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm này đã đi từ closure (bài 12) tới `partial` (bài 16) tới `curry2`
(bài này) — đủ mảnh để "chốt sẵn" dữ liệu theo nhiều cách. Ghép TẤT CẢ
lại trong một chương trình nhỏ thì trông thế nào?

Bài sau chốt cụm.
::::

::::checkpoint{mastery=0.8}
::::
