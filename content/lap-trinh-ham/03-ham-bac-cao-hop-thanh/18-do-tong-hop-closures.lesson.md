---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.do-tong-hop-closures
title: "Đo tổng hợp: closures & partial application"
summary: "Ghép closure thuần + partial + curry2 trong một chương trình nhỏ — dùng curry2 để tạo ra một hàm 'đã chốt sẵn' một tham số, dùng nó qua map() (nối cụm 2). Không khái niệm mới."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.review-closures]
requires: [fp.curry-handwritten]
concepts: [fp.review-closures]
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
Closure, `partial`, `curry2` — ba công cụ riêng lẻ đã học. Hôm nay ghép
lại, và nối cả với `map()` từ cụm trước.
::::

::::explain{#ghep-curry-va-map}
```python
def curry2(f):
    return lambda a: lambda b: f(a, b)

nhan = curry2(lambda a, b: a * b)
nhan_3 = nhan(3)

ds = [1, 2, 3, 4]
print(list(map(nhan_3, ds)))
```

```text
[3, 6, 9, 12]
```

`curry2` tạo `nhan`, gọi `nhan(3)` tạo `nhan_3` — một hàm MỘT tham số,
đã chốt sẵn `a = 3`. `map(nhan_3, ds)` truyền `nhan_3` (một hàm bình
thường, dùng được ở BẤT KỲ đâu cần một hàm — bài 1-2 của cụm này đã
dạy đúng điều đó: hàm là giá trị) làm hàm biến đổi cho `map()`. Không
khái niệm mới — chỉ ghép lại đúng những mảnh đã có: closure trả về
closure (bài 12, 17), rồi dùng kết quả đó CHỖ NÀO cũng nhận một hàm
(bài 6's `map()`).
::::

::::example{#toan-bo-chuoi-cong-cu}
Cả `partial` VÀ `curry2` đều tạo ra một hàm dùng được với `map()`:

```python title=readonly
from functools import partial

def cong(a, b):
    return a + b

def curry2(f):
    return lambda a: lambda b: f(a, b)

ds = [10, 20, 30]

cong_5_partial = partial(cong, 5)
cong_5_curry = curry2(cong)(5)

print(list(map(cong_5_partial, ds)))
print(list(map(cong_5_curry, ds)))
```

```text title=readonly
[15, 25, 35]
[15, 25, 35]
```

Hai công cụ KHÁC nhau (`partial` chốt bằng một lời gọi tường minh,
`curry2` chốt bằng gọi liên tiếp `f(a)(b)`), CÙNG kết quả cuối — cả
hai đều tạo ra một hàm MỘT tham số, dùng được với `map()` y hệt nhau.
Không có cách nào "đúng hơn" — chọn tuỳ ngữ cảnh (hàm đã có sẵn thì
`partial` gọn hơn; cần TỰ ĐỘNG chốt dần từng tham số thì `curry2` hợp
hơn).
::::

::::predict{#doan-ghep-toan-bo commitOnce}
```python
def curry2(f):
    return lambda a: lambda b: f(a, b)

tru = curry2(lambda a, b: a - b)
tru_100 = tru(100)

diem = [30, 45, 60]
print(list(map(tru_100, diem)))
```

Dòng cuối in ra gì?

:::opt{correct}
`[70, 55, 40]`
:::

:::opt
`[-70, -55, -40]` — vì `tru(100)` chốt `a=100` nghĩa là TỪNG phần tử bị
trừ CHO 100, không phải 100 trừ ĐI từng phần tử
::why
Gần đúng ở việc bạn để ý đúng THỨ TỰ trừ có thể gây nhầm — quan sát đó
đúng, `-` không giao hoán, thứ tự thật sự quan trọng.

Chỗ lệch: `curry2(lambda a, b: a - b)` rồi `tru(100)` chốt `a = 100` —
`tru_100(x)` tính `a - b = 100 - x`, tức 100 TRỪ ĐI `x`, không phải
ngược lại. `100 - 30 = 70`, `100 - 45 = 55`, `100 - 60 = 40` — đúng dấu
dương.
::
:::

:::opt
Máy báo lỗi — `map()` không dùng được với một hàm được tạo từ `curry2`,
chỉ dùng được với hàm khai bằng `def`/`lambda` trực tiếp
::why
Gần đúng ở việc bạn nghi ngờ `map()` có yêu cầu ĐẶC BIỆT về NGUỒN GỐC
của hàm truyền vào — một mối lo hợp lý nếu chưa chắc `map()` chấp nhận
những gì.

Chỗ lệch: `map()` chỉ cần một GIÁ TRỊ GỌI ĐƯỢC (callable) với đúng một
tham số — nó KHÔNG quan tâm hàm đó được tạo ra bằng `def`, `lambda`
trực tiếp, `partial`, hay `curry2`. `tru_100` là một hàm hợp lệ như bất
kỳ hàm nào khác.
::
:::
::::

::::code{#nhan_theo_gia}
Viết `nhan_theo_gia(dsach, he_so)` — dùng `curry2()` để tạo một hàm
nhân đã chốt sẵn `he_so`, rồi dùng `map()` để áp dụng lên TOÀN BỘ
`dsach`, trả về một `list`.

```python title=starter
def curry2(f):
    return lambda a: lambda b: f(a, b)

def nhan_theo_gia(dsach, he_so):
    ___

print(nhan_theo_gia([10, 20, 30], 2))
```

```python title=solution
def curry2(f):
    return lambda a: lambda b: f(a, b)

def nhan_theo_gia(dsach, he_so):
    nhan = curry2(lambda a, b: a * b)
    nhan_he_so = nhan(he_so)
    return list(map(nhan_he_so, dsach))

print(nhan_theo_gia([10, 20, 30], 2))
```

```python title=test
assert nhan_theo_gia([10, 20, 30], 2) == [20, 40, 60], "phải nhân từng phần tử với he_so"
assert nhan_theo_gia([1, 2, 3], 5) == [5, 10, 15], "phải đúng với he_so khác"
assert nhan_theo_gia([], 10) == [], "danh sách rỗng phải trả về danh sách rỗng"
```

:::hints
- kind: attention
  body: "Dùng curry2 để tạo một hàm nhân hai tham số, gọi nó với he_so để chốt sẵn, rồi truyền hàm đã chốt vào map() cùng dsach — không dùng vòng lặp hay comprehension viết tay."
- kind: strategy
  body: 'nhan = curry2(lambda a, b: a * b); nhan_he_so = nhan(he_so); return list(map(nhan_he_so, dsach)) — ba dòng: curry, chốt he_so, map lên dsach.'
- kind: one-line
  body: "nhan = curry2(lambda a, b: a * b)\nnhan_he_so = nhan(he_so)\nreturn list(map(nhan_he_so, dsach))"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: nhan_theo_gia phải dùng CẢ curry2() (để chốt sẵn he_so) LẪN map() (để áp dụng lên dsach) — đây là bài ghép lại cả cụm, không phải viết lại bằng vòng lặp hay comprehension.
  requireAst:
  - kind: uses-call, target: curry2, min: 1
  - kind: uses-call, target: map, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "[20, 40, 60]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Closure, `partial`, `curry2` — ba cách "chốt sẵn" dữ liệu, cùng một họ
Ý. Ghép với `map()` từ cụm trước, chúng phối hợp trơn tru, không xung
đột.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm này chốt lại: hàm là giá trị, `map`/`filter`/`reduce`, và giờ là
closure/`partial`/`curry`. Bước tiếp theo: GHÉP nhiều hàm nhỏ thành một
hàm LỚN, theo một trật tự rõ ràng — không chỉ "nhớ sẵn dữ liệu", mà
"nối các bước xử lý lại với nhau".

Cụm sau mở đầu bằng `compose`.
::::

::::checkpoint{mastery=0.8}
::::
