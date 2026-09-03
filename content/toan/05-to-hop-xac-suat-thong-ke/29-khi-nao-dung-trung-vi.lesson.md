---
id: toan.to-hop-xac-suat-thong-ke.khi-nao-dung-trung-vi
title: Khi nào dùng trung vị thay trung bình
summary: "Trung vị BỀN trước ngoại lệ (đổi một giá trị cực đoan gần như không đổi trung vị), trung bình NHẠY (mọi giá trị, kể cả cực đoan, đều góp phần trực tiếp vào tổng); chọn công cụ nào tuỳ CÂU HỎI."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.mean-vs-median-robustness]
requires: [math.median]
concepts: [math.chon-cong-cu-thong-ke]
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
Trung vị GẦN năm mùa bình thường hơn hẳn trung bình. Trung vị LUÔN
"đáng tin cậy hơn", hay mỗi công cụ có chỗ RIÊNG?
::::

::::explain{#khi-nao-dung-cong-cu-nao}
Mỗi công cụ có chỗ RIÊNG. **Trung vị BỀN trước ngoại lệ** (đổi một
giá trị CỰC ĐOAN gần như KHÔNG đổi trung vị, bài 28 đã thấy). **Trung
bình NHẠY** (MỌI giá trị, kể cả cực đoan, đều góp phần TRỰC TIẾP vào
tổng, bài 27 đã thấy). Chọn công cụ tuỳ CÂU HỎI:

```python title=readonly
def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)

def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def do_lech(du_lieu):
    return abs(trung_binh(du_lieu) - trung_vi(du_lieu))


mua_byte = [40, 42, 41, 39, 43, 5]

print(do_lech(mua_byte))
```

```text title=readonly
5.5
```

`x̄=35`, trung vị`=40.5` — LỆCH `5.5` kg. Muốn biết "TỔNG cả năm thu
hoạch bao nhiêu" — CẦN `x̄` (nhân LẠI `x̄×n` ra đúng tổng, bài 27
đã dùng phép cộng dồn TRỰC TIẾP). Muốn biết "mùa ĐIỂN HÌNH" — trung
vị đáng tin HƠN, vì nó KHÔNG bị mùa `5` kg kéo.
::::

::::example{#du-lieu-khong-co-ngoai-le}
Khi dữ liệu KHÔNG có ngoại lệ — `x̄` VÀ trung vị GẦN NHAU, thậm chí
TRÙNG:

```python title=readonly
def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)

def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def do_lech(du_lieu):
    return abs(trung_binh(du_lieu) - trung_vi(du_lieu))


mua_lan = [40, 40, 38, 41, 39, 42]

print(trung_binh(mua_lan))
print(trung_vi(mua_lan))
print(do_lech(mua_lan))
```

```text title=readonly
40.0
40.0
0.0
```

Sáu mùa của Lan (KHÔNG mùa nào bất thường) — `x̄` VÀ trung vị CÙNG
LÀ `40.0`, `do_lech=0.0`. KHÔNG có ngoại lệ, hai công cụ CHO cùng
một câu trả lời — chỉ khi có ngoại lệ chúng mới TÁCH ra.
::::

::::predict{#doan-du-lieu-doi-xung commitOnce}
Byte thử một dãy dữ liệu ĐỐI XỨNG hoàn hảo quanh một tâm — `[10, 20,
30, 40, 50]`:

```python
def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)

def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

print(trung_binh([10, 20, 30, 40, 50]))
print(trung_vi([10, 20, 30, 40, 50]))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`30.0`, rồi `30`
:::

:::opt
`30.0`, rồi `25` — trung vị LUÔN LÀ chỉ số Ở GIỮA danh sách (vị trí
`n//2`), và với năm phần tử thì `giua=5//2=2`, TRỎ tới GIÁ TRỊ `25`
::why
Gần đúng ở việc bạn tính ĐÚNG `giua = 5//2 = 2` — phép chia nguyên
đó chính xác.

Chỗ lệch: `giua=2` LÀ CHỈ SỐ (bắt đầu từ `0`), KHÔNG PHẢI giá trị.
Danh sách ĐÃ sắp `[10,20,30,40,50]` — phần tử Ở CHỈ SỐ `2` (đếm từ
`0`: `10` là chỉ số `0`, `20` là `1`, `30` là `2`) LÀ `30`, không
phải `25`. `s[2]` đọc RA `30`.
::
:::

:::opt
Máy báo lỗi khi chạy — `trung_binh` VÀ `trung_vi` cho CÙNG một số
(`30.0`), và Python từ chối in HAI kết quả GIỐNG hệt nhau liên tiếp
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng hai lời gọi `print` cho ra
CÙNG kết quả — đúng LÀ chúng SẼ trùng nhau ở ví dụ này.

Chỗ lệch: Python KHÔNG hề "phát hiện trùng lặp" giữa hai lời gọi
`print` rồi từ chối in — MỖI `print` chạy ĐỘC LẬP, in RA đúng giá
trị nó nhận, dù giá trị đó CÓ trùng với dòng trước hay không. Biên
dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_do_lech}
Viết `do_lech(du_lieu)` — tính khoảng cách TUYỆT ĐỐI giữa `trung_binh`
VÀ `trung_vi` của `du_lieu`.

```python title=starter
def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)

def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def do_lech(du_lieu):
    return ___


mua_byte = [40, 42, 41, 39, 43, 5]

print(do_lech(mua_byte))
```

```python title=solution
def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)

def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def do_lech(du_lieu):
    return abs(trung_binh(du_lieu) - trung_vi(du_lieu))


mua_byte = [40, 42, 41, 39, 43, 5]

print(do_lech(mua_byte))
```

```python title=test
mua_lan = [40, 40, 38, 41, 39, 42]
assert do_lech(mua_lan) == 0.0, "khong ngoai le -- hai cong cu trung nhau"
assert do_lech([10, 20, 30, 40, 50]) == 0, "doi xung hoan hao -- van trung nhau"
assert do_lech([10]) == 0, "mot phan tu -- ca hai cong cu deu bang chinh no"
assert do_lech(mua_byte) == 5.5, "phai khop vi du chinh"
```

:::hints
- kind: attention
  body: "Dung abs() de lay khoang cach tuyet doi giua trung_binh(du_lieu) va trung_vi(du_lieu)."
- kind: strategy
  body: "abs(trung_binh(du_lieu) - trung_vi(du_lieu))"
- kind: one-line
  body: "___ = abs(trung_binh(du_lieu) - trung_vi(du_lieu))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung abs() tren hieu cua trung_binh(du_lieu) va trung_vi(du_lieu)
  requireAst:
  - kind: uses-call, target: abs, min: 1
  - kind: uses-call, target: trung_binh, min: 1
  - kind: uses-call, target: trung_vi, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^5\.5\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu mùa của Byte không mùa nào TRÙNG kilôgam. Nếu HAI mùa cùng cho
đúng một con số thì sao?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sáu mùa của Byte không có mùa nào TRÙNG kilôgam với mùa khác. Lan
(người làm vườn khác, T2.4) có SÁU mùa mà HAI mùa CÙNG cho đúng `40`
kg — số nào xuất hiện NHIỀU LẦN nhất trong một tập dữ liệu, và nó
khác trung vị Ở CHỖ nào?
::::

::::checkpoint{mastery=0.8}
::::
