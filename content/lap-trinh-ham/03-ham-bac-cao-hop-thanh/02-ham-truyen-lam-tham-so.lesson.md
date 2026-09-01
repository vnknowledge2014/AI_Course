---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.ham-truyen-lam-tham-so
title: "Hàm truyền được làm THAM SỐ cho hàm khác"
summary: "def ap_dung(xs, f): return [f(x) for x in xs] — một hàm NHẬN một hàm khác làm đối số, gọi nó BÊN TRONG. Nền tảng của mọi Higher-Order Function (HOF) sắp học."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [fp.function-as-parameter]
requires: [fp.function-is-value]
concepts: [fp.function-as-parameter]
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
Bài trước bạn LẤY một hàm ra khỏi `dict`. Hôm nay bạn TRUYỀN một hàm
thẳng vào một hàm khác — làm đối số, y hệt một con số.
::::

::::explain{#ham-nhan-ham}
Một hàm nhận một THAM SỐ tên `f`, và bên trong, gọi `f` như bất kỳ hàm
nào khác:

```python
def ap_dung(xs, f):
    return [f(x) for x in xs]

print(ap_dung([1, 2, 3], lambda x: x ** 2))
```

```text
[1, 4, 9]
```

`ap_dung` không biết TRƯỚC `f` sẽ là hàm gì — nó chỉ biết "có một tham
số tên `f`, gọi được như một hàm" (`f(x)`). Khi gọi `ap_dung([1,2,3],
lambda x: x**2)`, `f` bên trong THÀNH `lambda x: x**2`. Gọi lại
`ap_dung` với một hàm KHÁC, hành vi đổi theo, mà không sửa một dòng nào
trong `ap_dung`:

```python
print(ap_dung(["a", "bb", "ccc"], len))
```

```text
[1, 2, 3]
```

Lần này `f` là `len` — một hàm CÓ SẴN của Python, không phải `lambda` tự
viết. `len` cũng là một giá trị (bài trước), nên truyền được y hệt.
::::

::::example{#mot-ham-nhieu-cach-goi}
Cùng MỘT hàm `ap_dung`, gọi với BA hàm khác nhau — không sửa `ap_dung`
lần nào:

```python title=readonly
def ap_dung(xs, f):
    return [f(x) for x in xs]

so = [1, -2, 3, -4]

print(ap_dung(so, abs))
print(ap_dung(so, lambda x: x > 0))
print(ap_dung(so, str))
```

```text title=readonly
[1, 2, 3, 4]
[True, False, True, False]
['1', '-2', '3', '-4']
```

Ba lời gọi, ba KẾT QUẢ hoàn toàn khác hình dạng (số, `True`/`False`,
chuỗi) — vì `ap_dung` không hề biết `f` sẽ làm gì, nó chỉ CHUYỂN TIẾP
từng phần tử qua `f`. Đây là sức mạnh của "hàm nhận hàm": viết MỘT lần,
dùng lại cho MỌI phép biến đổi.
::::

::::predict{#doan-goi-voi-ham-khac commitOnce}
```python
def ap_dung(xs, f):
    return [f(x) for x in xs]

def la_so_chan(n):
    return n % 2 == 0

ket_qua = ap_dung([1, 2, 3, 4, 5], la_so_chan)
print(ket_qua)
```

Dòng cuối in ra gì?

:::opt{correct}
`[False, True, False, True, False]`
:::

:::opt
`[2, 4]` — vì `la_so_chan` lọc ra các số chẵn trong danh sách
::why
Gần đúng ở việc bạn hiểu ĐÚNG Ý của `la_so_chan` (kiểm số chẵn) — hiểu
đúng bản thân hàm.

Chỗ lệch: `ap_dung` KHÔNG LỌC — nó áp `f` lên TỪNG phần tử rồi TRẢ VỀ
list CÙNG ĐỘ DÀI với `xs` (5 phần tử vào, 5 kết quả ra), mỗi phần tử là
`True`/`False`. Lọc bỏ phần tử là việc của `filter()` — bài khác, chưa
học ở đây. `ap_dung` chỉ BIẾN ĐỔI, không LOẠI BỎ.
::
:::

:::opt
Máy báo lỗi — `la_so_chan` không định nghĩa được kiểu tham số như một
`lambda`
::why
Gần đúng ở việc bạn để ý `la_so_chan` được định nghĩa bằng `def`, không
phải `lambda` — đúng là chúng viết KHÁC nhau.

Chỗ lệch: khác cú pháp ĐỊNH NGHĨA không có nghĩa khác khả năng TRUYỀN —
một hàm viết bằng `def` là một giá trị y hệt một hàm viết bằng `lambda`
(bài trước). `ap_dung` gọi được `la_so_chan` giống hệt cách nó gọi một
`lambda`.
::
:::

:::opt
`5` — vì có 5 phần tử, và `ap_dung` đếm số phần tử thoả `la_so_chan`
::why
Gần đúng ở việc bạn để ý đúng danh sách có 5 phần tử.

Chỗ lệch: `ap_dung` không ĐẾM gì cả — nó TRẢ VỀ một `list` MỚI, không
phải một con số. Đề bài hỏi `ket_qua` (cả `list`), không hỏi độ dài của
nó.
::
:::
::::

::::code{#dem-thoa-dieu-kien}
Viết `tong_sau_bien_doi(xs, f)` — áp `f` lên TỪNG phần tử của `xs`, rồi
CỘNG DỒN tất cả kết quả lại thành một số.

```python title=starter
def tong_sau_bien_doi(xs, f):
    ___

print(tong_sau_bien_doi([1, 2, 3], lambda x: x * 2))
print(tong_sau_bien_doi([1, 2, 3], lambda x: x))
```

```python title=solution
def tong_sau_bien_doi(xs, f):
    return sum(f(x) for x in xs)

print(tong_sau_bien_doi([1, 2, 3], lambda x: x * 2))
print(tong_sau_bien_doi([1, 2, 3], lambda x: x))
```

```python title=test
assert tong_sau_bien_doi([1, 2, 3], lambda x: x * 2) == 12, "phải áp f RỒI cộng dồn, không chỉ cộng dồn xs"
assert tong_sau_bien_doi([5, 10], lambda x: x + 1) == 17, "phải đúng với một f và xs KHÁC ví dụ trên"
assert tong_sau_bien_doi([], lambda x: x * 100) == 0, "danh sách rỗng phải ra 0, không lỗi"
```

:::hints
- kind: attention
  body: Gọi f(x) cho TỪNG phần tử x trong xs, rồi cộng hết lại — dùng sum() với một generator hoặc list comprehension.
- kind: strategy
  body: 'sum(f(x) for x in xs) — sum() cộng dồn mọi giá trị trong dấu ngoặc, và f(x) for x in xs sinh ra từng kết quả đã áp f, từng phần tử một.'
- kind: one-line
  body: "return sum(f(x) for x in xs)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: tong_sau_bien_doi phải GỌI f(x) cho từng phần tử — không được bỏ qua f và chỉ cộng dồn xs.
  requireAst:
  - kind: uses-name, target: f, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "12"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàm nhận hàm — gọi nó, không cần biết trước nó sẽ làm gì. Đây chính
là viên gạch mà `map`/`filter`/`reduce` (bài sau) đều đứng trên đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa TRUYỀN một hàm VÀO một hàm khác. Chiều ngược lại thì sao — một
hàm có TRẢ VỀ được một hàm khác không, giống cách nó trả về một số hay
một chuỗi?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
