---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.lambda-bieu-thuc-khong-ten
title: "`lambda` — một hàm không tên, đúng MỘT biểu thức"
summary: "lambda x, y: x + y — không def, không tên, không return (giá trị biểu thức TỰ LÀ kết quả), và CHỈ được đúng một biểu thức. Đã đo thật trên Pyodide."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [fp.lambda-basics]
requires: [fp.function-as-return]
concepts: [fp.lambda-basics]
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
Ba bài vừa qua dùng `lambda` mà chưa hỏi kỹ nó LÀ GÌ. Hôm nay Byte đặt nó
cạnh `def`, chỉ đúng chỗ khác nhau.
::::

::::explain{#lambda-la-gi}
`lambda` định nghĩa một hàm KHÔNG TÊN, gói trong đúng MỘT biểu thức:

```python
cong = lambda x, y: x + y
print(cong(3, 4))
```

```text
7
```

So với `def`:

```python
def cong(x, y):
    return x + y
```

Hai cách viết CÙNG một hàm — nhưng `lambda` KHÔNG có từ khoá `return`.
Giá trị của biểu thức sau dấu `:` TỰ ĐỘNG là kết quả trả về — không viết
`return`, không viết được `return` (cú pháp `lambda` không cho phép).

`lambda` có tham số y hệt `def` (nhiều tham số, cách nhau dấu phẩy), và
dùng được biểu thức điều kiện (`a if ... else b`) — vẫn là MỘT biểu
thức, dù trông phức tạp:

```python
lon_hon = lambda a, b: a if a > b else b
print(lon_hon(5, 9))
```

```text
9
```

Giới hạn thật của `lambda`: KHÔNG được nhiều dòng, KHÔNG được câu lệnh
kiểu `if`/`for` (dạng đầy đủ, có `:` xuống dòng), KHÔNG được nhiều biểu
thức tách bằng `;`. Cần logic phức tạp hơn một biểu thức — dùng `def`.
`lambda` không phải "hàm yếu hơn `def`" — nó là công cụ cho đúng một
tình huống: một phép tính NGẮN, dùng MỘT LẦN, không cần đặt tên riêng
(ví dụ: truyền thẳng cho `ap_dung` ở bài 2, không cần `def` một hàm phụ
chỉ dùng đúng một chỗ).
::::

::::example{#lambda-ngay-tai-cho}
Chỗ `lambda` toả sáng nhất: truyền THẲNG vào một hàm khác, không cần đặt
tên riêng:

```python title=readonly
def ap_dung(xs, f):
    return [f(x) for x in xs]

print(ap_dung([1, 2, 3], lambda x: x * x))
```

```text title=readonly
[1, 4, 9]
```

Không cần viết `def binh_phuong(x): return x * x` rồi mới truyền
`binh_phuong` — nếu phép tính chỉ dùng ĐÚNG một chỗ, `lambda` viết gọn
ngay tại vị trí gọi, không tốn một cái tên riêng cho một hàm chỉ sống
đúng một dòng.
::::

::::predict{#doan-gioi-han-lambda commitOnce}
Byte thử viết một `lambda` có `if` ĐẦY ĐỦ (không phải biểu thức điều
kiện) bên trong:

```python
xep_loai = lambda diem: (
    if diem >= 8:
        return "Giỏi"
    else:
        return "Khá"
)
```

Chạy đoạn này thì sao?

:::opt{correct}
Máy báo lỗi cú pháp (`SyntaxError`) — `lambda` không cho phép câu lệnh
`if`/`return` đầy đủ bên trong
:::

:::opt
Chạy được bình thường, `xep_loai(9)` ra `"Giỏi"`
::why
Gần đúng ở việc bạn tin `lambda` là một hàm đầy đủ, làm được mọi thứ
`def` làm được — phản xạ hợp lý nếu chỉ nhìn tên gọi "hàm".

Chỗ lệch: `lambda` CHỈ nhận đúng MỘT BIỂU THỨC — `if`/`return` ở đây là
CÂU LỆNH (statement), không phải biểu thức (expression), và cú pháp
`lambda` không có chỗ cho câu lệnh. Muốn rẽ nhánh trong `lambda`, phải
dùng biểu thức điều kiện (`"Giỏi" if diem >= 8 else "Khá"`), như ví dụ
`lon_hon` ở trên.
::
:::

:::opt
Chạy được, nhưng `return` bên trong bị bỏ qua, `xep_loai(9)` ra `None`
::why
Gần đúng ở việc bạn nghi ngờ có gì đó không ổn với `return` ở đây —
đúng, có vấn đề thật.

Chỗ lệch: vấn đề không phải "return bị bỏ qua" — mã này KHÔNG PARSE
ĐƯỢC, dừng lại ngay ở bước đọc cú pháp, trước khi có cơ hội chạy dòng
nào. Không có `xep_loai(9)` nào chạy tới nơi.
::
:::
::::

::::code{#lambda-kiem-tra-diem}
Viết `la_diem_dat` — một `lambda` nhận `diem`, trả về `True` nếu
`diem >= 50`, ngược lại `False`.

```python title=starter
la_diem_dat = ___

print(la_diem_dat(75))
print(la_diem_dat(40))
```

```python title=solution
la_diem_dat = lambda diem: diem >= 50

print(la_diem_dat(75))
print(la_diem_dat(40))
```

```python title=test
assert la_diem_dat(50) == True, "diem == 50 phải tính là đạt"
assert la_diem_dat(49) == False, "diem == 49 phải tính là KHÔNG đạt"
assert la_diem_dat(100) == True, "phải đúng với một điểm KHÁC ví dụ trên"
```

:::hints
- kind: attention
  body: Viết một lambda, không phải def — chỗ trống là cả biểu thức lambda gán cho la_diem_dat.
- kind: strategy
  body: 'lambda diem: diem >= 50 — nhận một tham số diem, biểu thức so sánh diem >= 50 TỰ LÀ kết quả trả về (True hoặc False), không cần return.'
- kind: one-line
  body: "lambda diem: diem >= 50"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: la_diem_dat phải được viết bằng lambda (bài này đang dạy đúng cú pháp đó) — không phải def, dù cùng hành vi.
  requireAst:
  - kind: lambda, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "True"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một biểu thức, không tên, không `return` — đúng cỡ vừa đủ cho một phép
tính ngắn, dùng ngay tại chỗ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bốn bài vừa qua đã ghép đủ: hàm gán được vào tên, truyền được vào hàm
khác, trả về được từ hàm khác, và viết gọn được bằng `lambda`. Ghép cả
bốn lại trong MỘT chương trình thì trông thế nào?

Bài sau đo đúng điều đó.
::::

::::checkpoint{mastery=0.8}
::::
