---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.closure-thuan-chi-doc
title: "Closure THUẦN — chỉ ĐỌC biến ngoài, không sửa"
summary: "def lam_cong(n): return lambda x: x + n — n bị 'bắt' (captured) nhưng KHÔNG BAO GIỜ bị sửa, chỉ được ĐỌC. Closure này THUẦN đúng nghĩa T4.1 đã dạy — code chấm điểm sống, dùng lại pure-fn."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [fp.closure-pure]
requires: [fp.closure-basics]
concepts: [fp.closure-pure]
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
Bài trước bạn thấy closure "nhớ" được biến ngoài. Hôm nay câu hỏi khác:
closure đó có SỬA được biến nó nhớ không, hay chỉ ĐỌC?
::::

::::explain{#closure-chi-doc}
```python
def lam_cong(n):
    return lambda x: x + n

cong5 = lam_cong(5)
print(cong5(10))
print(cong5(10))
```

```text
15
15
```

`cong5(10)` gọi HAI LẦN, với CÙNG đối số (`10`), ra CÙNG kết quả (`15`)
cả hai lần. `n = 5` bị closure "bắt" (captured) — nhưng closure chỉ
ĐỌC `n`, KHÔNG BAO GIỜ gán lại hay sửa nó. Mỗi lần gọi `cong5(x)` chỉ
tính `x + n`, không đổi trạng thái gì cả.

Đây CHÍNH LÀ định nghĩa hàm thuần đã học ở T4.1 (`fp.pure-fn-def`):
kết quả CHỈ phụ thuộc tham số đầu vào, không phụ thuộc gì khác, không
gây side effect nào. Closure không phải một ngoại lệ của quy tắc đó —
nó CHỈ là một hàm có thêm một biến "đi kèm" từ môi trường sinh ra nó,
và biến đi kèm đó, NẾU chỉ bị đọc, không phá vỡ tính thuần.
::::

::::example{#nhieu-lan-goi-van-thuan}
Nhiều closure khác nhau, mỗi cái vẫn thuần theo đúng cách riêng của nó:

```python title=readonly
def lam_cong(n):
    return lambda x: x + n

cong5 = lam_cong(5)
cong100 = lam_cong(100)

print(cong5(1))
print(cong100(1))
print(cong5(1))
print(cong5(2))
```

```text title=readonly
6
101
6
7
```

`cong5(1)` gọi hai lần (dòng 1 và dòng 3) — cùng đối số, cùng kết quả
(`6`), dù ở GIỮA đã gọi `cong100(1)`. `cong5(2)` (đối số KHÁC) ra kết
quả KHÁC (`7`) — đúng bản chất "kết quả phụ thuộc ĐÚNG đối số đầu vào",
không phụ thuộc thứ tự gọi hay bất kỳ lần gọi nào khác.
::::

::::predict{#doan-closure-thuan commitOnce}
```python
def lam_nhan(k):
    return lambda x: x * k

nhan4 = lam_nhan(4)
a = nhan4(7)
b = nhan4(7)
print(a == b)
print(nhan4(2))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True` rồi `8`
:::

:::opt
`False` rồi `8` — vì mỗi lần gọi `nhan4(7)` tạo một closure MỚI, hai
kết quả không đảm bảo bằng nhau
::why
Gần đúng ở việc bạn để ý tới CHUYỆN TẠO closure — closure quả thật được
tạo ra khi gọi `lam_nhan(4)`.

Chỗ lệch: `nhan4` chỉ được tạo MỘT LẦN (dòng `nhan4 = lam_nhan(4)`) —
`a = nhan4(7)` và `b = nhan4(7)` là hai LẦN GỌI của CÙNG một closure đã
có sẵn, không tạo closure mới. Vì closure này chỉ ĐỌC `k` (không sửa),
gọi lại với cùng đối số (`7`) luôn ra cùng kết quả — `a == b` là `True`.
::
:::

:::opt
`True` rồi `28` — vì `nhan4` "nhớ" luôn kết quả `7*4=28` từ lần gọi
trước, không tính lại
::why
Gần đúng ở việc bạn tin `a == b` đúng là `True` — dòng đó đúng, closure
này thuần thật.

Chỗ lệch: closure KHÔNG "nhớ" kết quả của lần gọi trước — mỗi lần gọi
`nhan4(x)` TÍNH LẠI TỪ ĐẦU bằng `x * k`, chỉ đối số `x` là mới, không
tái dùng kết quả cũ. `nhan4(2)` tính `2 * 4 = 8`, không liên quan gì
tới `28` của lần gọi `nhan4(7)` trước đó.
::
:::
::::

::::code{#lam_giam_gia}
Viết `lam_giam_gia(phan_tram)` — trả về một closure THUẦN: nhận `gia`,
trả về giá SAU khi giảm đúng `phan_tram`%. Ví dụ `lam_giam_gia(10)(100)`
phải ra `90.0` (giảm 10% trên 100).

```python title=starter
def lam_giam_gia(phan_tram):
    ___

giam_10 = lam_giam_gia(10)
print(giam_10(100))
```

```python title=solution
def lam_giam_gia(phan_tram):
    return lambda gia: gia * (1 - phan_tram / 100)

giam_10 = lam_giam_gia(10)
print(giam_10(100))
```

```python title=test
giam_20 = lam_giam_gia(20)
assert giam_20(100) == 80.0, "giảm 20% trên 100 phải ra 80.0"
assert giam_20(100) == giam_20(100), "gọi lại CÙNG đối số phải ra CÙNG kết quả — closure này phải THUẦN"
assert lam_giam_gia(0)(50) == 50.0, "giảm 0% thì giá phải giữ nguyên"
giam_50 = lam_giam_gia(50)
assert giam_10(100) == 90.0, "closure giam_10 không được bị ảnh hưởng bởi việc tạo giam_20/giam_50 sau đó"
```

:::hints
- kind: attention
  body: "Trả về một lambda nhận gia, tính gia nhân với (1 trừ phan_tram/100) — không dùng vòng lặp, không sửa phan_tram, không print bên trong."
- kind: strategy
  body: 'return lambda gia: gia * (1 - phan_tram / 100) — closure chỉ ĐỌC phan_tram, không bao giờ gán lại nó.'
- kind: one-line
  body: "return lambda gia: gia * (1 - phan_tram / 100)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: lam_giam_gia phải là một closure THUẦN có dùng đến phan_tram — không print, không sửa biến toàn cục, không gọi hàm ngẫu nhiên bên trong, và không được bỏ qua tham số. Bài này đang chứng minh closure vẫn giữ được tính thuần nếu chỉ ĐỌC biến bắt được.
  requireAst:
  - kind: pure-fn, target: lam_giam_gia
  - kind: uses-name, target: phan_tram, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "90.0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Closure "nhớ" một biến không có nghĩa nó SỬA biến đó. Chỉ ĐỌC — closure
vẫn thuần, vẫn dự đoán được y hệt mọi hàm thuần khác.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu một closure KHÔNG chỉ đọc, mà còn SỬA được biến nó bắt được — mỗi
lần gọi lại ra một kết quả khác — chuyện gì xảy ra với tính thuần?

Bài sau đo đúng trường hợp đó.
::::

::::checkpoint{mastery=0.8}
::::
