---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.map-ap-len-moi-phan-tu
title: "`map()` — áp MỘT hàm lên MỌI phần tử"
summary: "map(lambda x: x*2, [1,2,3]) — TRẢ VỀ MỘT ITERATOR, không phải list. Cần list(map(...)) để ép ra hoặc lặp trực tiếp. Đã đo thật, chạy sạch."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [fp.map-basics]
requires: [fp.review-first-class]
concepts: [fp.map-basics]
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
Bạn tự viết `ap_dung(xs, f)` ở cụm trước. Python có sẵn một hàm làm y hệt
— tên nó là `map()`. Nhưng nó giấu một bẫy nhỏ.
::::

::::explain{#map-la-gi}
`map(f, xs)` áp hàm `f` lên TỪNG phần tử của `xs` — đúng việc
`ap_dung` đã làm, có sẵn trong Python, không cần tự viết:

```python
xs = [1, 2, 3]
ket_qua = map(lambda x: x * 2, xs)
print(ket_qua)
```

```text
<map object at 0x...>
```

ĐÃ ĐO THẬT, bẫy có thật: `map()` KHÔNG trả về một `list` — nó trả về
một **iterator**, một đối tượng "hứa sẽ sinh ra từng giá trị khi được
hỏi", không phải một `list` đã tính sẵn. In nó ra chỉ thấy
`<map object at 0x...>`, không thấy `[2, 4, 6]`.

Muốn LẤY RA các giá trị thật, bọc bằng `list(...)`:

```python
danh_sach = list(map(lambda x: x * 2, xs))
print(danh_sach)
```

```text
[2, 4, 6]
```

Hệ quả trực tiếp: `map(f, xs) == [2, 4, 6]` LUÔN LUÔN là `False`, dù giá
trị bên trong đúng — vì một `map object` không bao giờ BẰNG một `list`,
bất kể nó sẽ sinh ra gì. Phải `list(map(...))` TRƯỚC khi so sánh với
một `list`.
::::

::::example{#map-vs-vong-lap}
`map()` và một vòng lặp thường làm CÙNG một việc:

```python title=readonly
xs = [1, 2, 3, 4]

cach_1 = list(map(lambda x: x * x, xs))

cach_2 = []
for x in xs:
    cach_2.append(x * x)

print(cach_1)
print(cach_1 == cach_2)
```

```text title=readonly
[1, 4, 9, 16]
True
```

Không có phép màu nào — `map()` chỉ là một cách viết KHÁC cho đúng vòng
lặp "biến đổi từng phần tử, gom vào một danh sách mới" mà `for` cũng
làm được.
::::

::::predict{#doan-map-chua-list commitOnce}
```python
xs = [10, 20, 30]
ket_qua = map(lambda x: x // 10, xs)

print(ket_qua == [1, 2, 3])
print(list(ket_qua) == [1, 2, 3])
```

Hai dòng in ra gì?

:::opt{correct}
`False` rồi `True`
:::

:::opt
`True` rồi `True` — vì giá trị bên trong đều đúng `[1, 2, 3]` ở cả hai
lần so sánh
::why
Gần đúng ở việc bạn tính ĐÚNG các giá trị bên trong (`10//10=1`,
`20//10=2`, `30//10=3`) — phép tính đó không sai.

Chỗ lệch: dòng ĐẦU so sánh `ket_qua` (một `map object`, chưa ép ra) với
một `list` — một `map object` KHÔNG BAO GIỜ bằng một `list`, bất kể giá
trị bên trong là gì, đúng bẫy đã đo ở trên. Chỉ dòng THỨ HAI (có
`list(...)`) mới thật sự so sánh được giá trị, và mới ra `True`.
::
:::

:::opt
`False` rồi `False` — vì sau khi `ket_qua == [1,2,3]` đã "dùng"
`ket_qua`, `list(ket_qua)` ở dòng sau không còn gì để lấy
::why
Gần đúng ở việc bạn cảnh giác về việc một iterator có thể bị "dùng cạn"
— đây là một mối lo THẬT SỰ có với iterator (dùng một lần là hết), đáng
để ý.

Chỗ lệch: phép so sánh `ket_qua == [1,2,3]` KHÔNG duyệt qua `ket_qua` —
so sánh bằng giữa hai KIỂU KHÁC NHAU (`map` và `list`) trả `False` NGAY
LẬP TỨC, không cần lặp qua từng phần tử để kiểm tra. `ket_qua` vẫn còn
nguyên vẹn, chưa bị "dùng" gì cả, nên `list(ket_qua)` ở dòng sau vẫn lấy
đủ ba phần tử.
::
:::
::::

::::code{#tang_gia}
Viết `tang_gia(danh_sach, so_tien)` — CỘNG THÊM `so_tien` vào TỪNG giá
trong `danh_sach`, dùng `map()`. Trả về một `list` thật (không phải một
`map object`).

```python title=starter
def tang_gia(danh_sach, so_tien):
    ___

gia_ds = [40000, 55000, 60000]
print(tang_gia(gia_ds, 5000))
```

```python title=solution
def tang_gia(danh_sach, so_tien):
    return list(map(lambda g: g + so_tien, danh_sach))

gia_ds = [40000, 55000, 60000]
print(tang_gia(gia_ds, 5000))
```

```python title=test
assert tang_gia([40000, 55000, 60000], 5000) == [45000, 60000, 65000], "phải cộng đúng so_tien vào TỪNG phần tử"
assert tang_gia([1, 2], 100) == [101, 102], "phải đúng với một danh sách/số tiền KHÁC ví dụ trên"
assert tang_gia([1, 2, 3], 0) == [1, 2, 3], "cộng thêm 0 thì giữ nguyên"
assert isinstance(tang_gia([1], 1), list), "phải trả về list THẬT — không phải map object chưa ép ra"
```

:::hints
- kind: attention
  body: Dùng map() để cộng so_tien vào từng phần tử, rồi ĐỪNG QUÊN bọc list(...) — map() một mình trả về iterator, không phải list.
- kind: strategy
  body: 'list(map(lambda g: g + so_tien, danh_sach)) — map(...) áp lambda lên từng phần tử, list(...) ép kết quả thành một danh sách thật, so sánh được với một list khác.'
- kind: one-line
  body: "return list(map(lambda g: g + so_tien, danh_sach))"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: tang_gia phải dùng map() (bài đang dạy đúng công cụ đó) VÀ bọc list(...) quanh nó — map() một mình trả về iterator, không so sánh được với list.
  requireAst:
  - kind: uses-call, target: map, min: 1
  - kind: uses-call, target: list, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "45000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`map()` biến đổi từng phần tử — nhưng nhớ `list()` bọc lại, nếu không
bạn chỉ có một lời hứa, chưa có kết quả thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`map()` biến đổi MỌI phần tử, giữ nguyên số lượng. Nếu bạn chỉ muốn GIỮ
LẠI một số phần tử — bỏ bớt những cái không thoả điều kiện — thì dùng
công cụ nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
