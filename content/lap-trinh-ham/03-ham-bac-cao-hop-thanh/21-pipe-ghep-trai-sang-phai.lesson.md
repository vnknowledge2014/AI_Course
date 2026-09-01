---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.pipe-ghep-trai-sang-phai
title: "`pipe(value, *fns)` — ghép TRÁI-SANG-PHẢI, đọc xuôi tự nhiên"
summary: "def pipe(value, *ham): ...; pipe(3, lambda x: x+1, lambda x: x*2, str) == '8' — CHẠY và ĐỌC cùng một chiều (trái sang phải), giải quyết đúng vấn đề đọc-ngược của compose."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [fp.pipe-basics]
requires: [fp.compose-nested]
concepts: [fp.pipe-basics]
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
Bài trước: `compose` lồng nhau chạy NGƯỢC thứ tự viết. Hôm nay một
công cụ khác — ghép hàm sao cho ĐỌC và CHẠY cùng một chiều.
::::

::::explain{#pipe-la-gi}
```python
def pipe(value, *ham):
    ket_qua = value
    for f in ham:
        ket_qua = f(ket_qua)
    return ket_qua

print(pipe(3, lambda x: x + 1, lambda x: x * 2, str))
```

```text
8
```

`pipe(value, *ham)` nhận một GIÁ TRỊ ban đầu, rồi BAO NHIÊU hàm tuỳ ý
(`*ham` gom hết vào một tuple). Chạy TỪNG hàm THEO ĐÚNG THỨ TỰ VIẾT,
mỗi hàm nhận kết quả của hàm TRƯỚC nó: `pipe(3, +1, *2, str)` — `3 + 1
= 4`, rồi `4 * 2 = 8`, rồi `str(8) = "8"`. Đọc từ TRÁI sang PHẢI, thấy
ĐÚNG thứ tự chạy — không phải lật ngược như bài 20's `compose` lồng
nhau.

`pipe` không phải phép màu — nó chỉ là MỘT vòng lặp `for` quen thuộc,
chạy qua TỪNG hàm trong `ham`, cập nhật `ket_qua` sau mỗi bước.
::::

::::example{#doc-xuoi-tu-nhien}
So sánh trực tiếp: CÙNG một chuỗi xử lý, hai cách viết:

```python title=readonly
def compose(f, g):
    return lambda x: f(g(x))

def pipe(value, *ham):
    ket_qua = value
    for f in ham:
        ket_qua = f(ket_qua)
    return ket_qua

cong_1 = lambda x: x + 1
nhan_doi = lambda x: x * 2

cach_compose = compose(str, compose(nhan_doi, cong_1))(3)
cach_pipe = pipe(3, cong_1, nhan_doi, str)

print(cach_compose)
print(cach_pipe)
print(cach_compose == cach_pipe)
```

```text title=readonly
8
8
True
```

CÙNG kết quả (`"8"`) — nhưng `pipe(3, cong_1, nhan_doi, str)` ĐỌC ĐÚNG
thứ tự chạy (`cong_1` trước, `nhan_doi` sau, `str` cuối), trong khi
`compose(str, compose(nhan_doi, cong_1))` phải ĐỌC TỪ TRONG RA NGOÀI,
NGƯỢC thứ tự viết, để thấy đúng thứ tự chạy y hệt.
::::

::::predict{#doan-pipe-nhieu-buoc commitOnce}
```python
def pipe(value, *ham):
    ket_qua = value
    for f in ham:
        ket_qua = f(ket_qua)
    return ket_qua

tru_3 = lambda x: x - 3
nhan_5 = lambda x: x * 5
cong_1 = lambda x: x + 1

print(pipe(10, tru_3, nhan_5, cong_1))
```

Dòng cuối in ra gì?

:::opt{correct}
`36`
:::

:::opt
`48` — vì `pipe` chạy hàm CUỐI CÙNG (`cong_1`) TRƯỚC, giống thứ tự
chạy của `compose`
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng THỨ TỰ chạy quan trọng (bài 20 đã dạy
điều đó) — cảnh giác đó có lý.

Chỗ lệch: `pipe` KHÔNG chạy như `compose` — nó chạy đúng THỨ TỰ VIẾT,
TRÁI sang PHẢI: `tru_3(10) = 7` trước, rồi `nhan_5(7) = 35`, rồi
`cong_1(35) = 36` cuối cùng. Đây CHÍNH LÀ điểm khác biệt `pipe` được
tạo ra để giải quyết — không cần lật ngược thứ tự khi đọc.
::
:::

:::opt
Máy báo lỗi — `pipe` chỉ nhận được TỐI ĐA hai hàm sau `value`, ở đây
truyền tới BA hàm
::why
Gần đúng ở việc bạn cảnh giác về giới hạn SỐ LƯỢNG đối số — một mối lo
hợp lý với nhiều hàm Python có giới hạn tham số cố định.

Chỗ lệch: `*ham` trong định nghĩa `pipe(value, *ham)` gom TẤT CẢ đối số
CÒN LẠI vào một tuple, không giới hạn số lượng — truyền `3`, `10`, hay
`100` hàm đều chạy được, vòng lặp `for f in ham` xử lý đúng bao nhiêu
hàm cũng được.
::
:::
::::

::::code{#pipe}
Tự viết `pipe(value, *ham)` — áp dụng LẦN LƯỢT từng hàm trong `ham` lên
`value`, ĐÚNG thứ tự viết (trái sang phải).

```python title=starter
def pipe(value, *ham):
    ___

print(pipe(3, lambda x: x + 1, lambda x: x * 2, str))
```

```python title=solution
def pipe(value, *ham):
    ket_qua = value
    for f in ham:
        ket_qua = f(ket_qua)
    return ket_qua

print(pipe(3, lambda x: x + 1, lambda x: x * 2, str))
```

```python title=test
assert pipe(3, lambda x: x + 1, lambda x: x * 2, str) == "8", "phải chạy TRÁI SANG PHẢI: 3+1=4, 4*2=8, str(8)='8'"
assert pipe(5) == 5, "không hàm nào thì trả về value gốc, không đổi"
assert pipe(2, lambda x: x * 10) == 20, "một hàm duy nhất vẫn phải áp dụng đúng"
assert pipe(1, lambda x: x + 1, lambda x: x + 1, lambda x: x + 1, lambda x: x + 1) == 5, "phải áp dụng ĐỦ và ĐÚNG THỨ TỰ với bốn hàm"
```

:::hints
- kind: attention
  body: "Dùng một biến ket_qua bắt đầu từ value, vòng qua từng hàm trong ham (dùng for), mỗi bước gán lại ket_qua = f(ket_qua) — không đổi thứ tự, không bỏ sót hàm nào."
- kind: strategy
  body: 'ket_qua = value; for f in ham: ket_qua = f(ket_qua); return ket_qua — ba dòng, giống hệt khuôn reduce_tu_viet bài 10, chỉ khác dữ liệu chạy qua là HÀM, không phải số.'
- kind: one-line
  body: "ket_qua = value\nfor f in ham:\n    ket_qua = f(ket_qua)\nreturn ket_qua"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: pipe phải chạy qua TỪNG hàm trong ham (dùng for hoặc tương đương) và áp dụng nó lên giá trị đang có — không được bỏ qua ham, chỉ trả lại value gốc.
  requireAst:
  - kind: uses-name, target: ham, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "8"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`pipe` ghép hàm ĐÚNG thứ tự viết — đọc mã và hiểu luồng chạy giờ là
CÙNG một việc, không cần lật ngược trong đầu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa viết `pipe` bằng một vòng lặp `for` — CHÍNH XÁC khuôn của
`reduce_tu_viet` ở bài 10 (một biến tích luỹ, cập nhật qua từng bước).
Nếu `reduce` (bài 9) gộp được một DÃY SỐ thành một số, nó có gộp được
một DÃY HÀM thành một giá trị không?

Bài sau viết lại `pipe` bằng `reduce`, nối thẳng cụm 2.
::::

::::checkpoint{mastery=0.8}
::::
