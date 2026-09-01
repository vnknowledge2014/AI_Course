---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.mot-mat-xich-nem-loi-dung-cho
title: "Một mắt xích NÉM LỖI — pipeline dừng ĐÚNG CHỖ, không lặng lẽ tiếp tục"
summary: "Nếu một hàm giữa chuỗi pipe (ví dụ int('abc')) ném ValueError, TOÀN BỘ pipe dừng ngay tại đó — các hàm SAU không chạy, lỗi lộ ra rõ ràng, không có bước nào 'nuốt' lỗi âm thầm."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.pipeline-error-handling]
requires: [fp.debug-with-tap]
concepts: [fp.pipeline-error-handling]
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
`tap` giúp NHÌN giá trị giữa chừng. Nhưng nếu một bước không chỉ "ra
giá trị sai" mà THỰC SỰ ném ra một lỗi? Chuyện gì xảy ra với các bước
sau nó?
::::

::::explain{#loi-dung-vi-tri}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def xu_ly_so(vb):
    chuan_hoa = lambda s: s.strip()
    chuyen_so = lambda s: int(s)
    nhan_2 = lambda n: n * 2
    return pipe(vb, chuan_hoa, chuyen_so, nhan_2)

print(xu_ly_so("  42  "))

try:
    xu_ly_so("khong phai so")
except ValueError as e:
    print("bắt lỗi:", type(e).__name__)
```

```text
84
bắt lỗi: ValueError
```

`xu_ly_so("  42  ")` chạy hết BA bước, ra `84`. Nhưng `int("khong phai
so")` (bước `chuyen_so`, giữa chuỗi) NÉM RA `ValueError` — Python
không có cách nào biến `"khong phai so"` thành số nguyên. Lỗi này
KHÔNG bị `pipe` "nuốt" hay bỏ qua — nó BAY THẲNG ra ngoài, dừng cả
pipeline NGAY TẠI bước gây lỗi.

Nhớ lại `reduce` bên dưới `pipe`: nó chạy TUẦN TỰ, TỪNG bước MỘT (bài
22). Khi MỘT bước ném lỗi, `reduce` KHÔNG có cơ chế nào "bỏ qua rồi
chạy tiếp" — nó DỪNG NGAY, lỗi lan ra khỏi cả `pipe`. Đây không phải
thiếu sót — nó CHÍNH XÁC là hành vi ta MUỐN: nếu dữ liệu giữa chừng
"hỏng", tiếp tục chạy các bước SAU trên dữ liệu hỏng đó CHỈ tạo ra kết
quả sai một cách ÂM THẦM, khó phát hiện hơn NHIỀU so với dừng ngay và
báo lỗi rõ ràng.
::::

::::example{#cac-buoc-sau-khong-chay}
Chèn `tap` (bài 27) TRƯỚC và SAU bước gây lỗi, để thấy RÕ bước nào chạy,
bước nào KHÔNG:

```python title=readonly
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def tap(nhan):
    return lambda x: (print(f"{nhan}: {x}"), x)[1]

chuan_hoa = lambda s: s.strip()
chuyen_so = lambda s: int(s)
nhan_2 = lambda n: n * 2

try:
    pipe(
        "khong phai so",
        chuan_hoa,
        tap("sau chuẩn hoá"),
        chuyen_so,
        tap("sau chuyển số — KHÔNG BAO GIỜ tới đây"),
        nhan_2,
    )
except ValueError as e:
    print("dừng ở bước chuyen_so:", type(e).__name__)
```

```text title=readonly
sau chuẩn hoá: khong phai so
dừng ở bước chuyen_so: ValueError
```

`tap("sau chuẩn hoá")` IN RA — bước ĐÓ chạy bình thường (`chuan_hoa`
không gây lỗi). Nhưng `tap("sau chuyển số — KHÔNG BAO GIỜ tới đây")`
KHÔNG BAO GIỜ in ra — `chuyen_so` (bước NGAY TRƯỚC nó) đã ném lỗi, dừng
cả chuỗi TẠI ĐÓ, `nhan_2` cũng không bao giờ chạy.
::::

::::predict{#doan-dung-dung-cho commitOnce}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

chia_10 = lambda n: 10 / n
cong_1 = lambda n: n + 1
nhan_2 = lambda n: n * 2

try:
    ket_qua = pipe(0, chia_10, cong_1, nhan_2)
    print("khong loi:", ket_qua)
except ZeroDivisionError:
    print("bat duoc loi chia cho 0")
```

Chương trình in ra gì?

:::opt{correct}
`bat duoc loi chia cho 0`
:::

:::opt
`khong loi: 2.0` — vì `pipe` bỏ qua bước gây lỗi (`chia_10`), tiếp tục
với `cong_1` và `nhan_2` trên giá trị gốc (`0`)
::why
Gần đúng ở việc bạn nghĩ tới một CƠ CHẾ bỏ-qua-bước-lỗi — một số công
cụ xử lý dữ liệu THẬT có cơ chế đó (nhưng phải khai báo rõ ràng, không
mặc định).

Chỗ lệch: `pipe` (viết bằng `reduce`, bài 22) KHÔNG có cơ chế "bỏ qua"
nào — `chia_10(0)` ném `ZeroDivisionError` NGAY LẬP TỨC, dừng cả `pipe`
tại đó. `cong_1` và `nhan_2` KHÔNG BAO GIỜ chạy. Lỗi bay thẳng ra ngoài
`pipe(...)`, bị bắt bởi `except ZeroDivisionError`.
::
:::

:::opt
Máy báo lỗi cú pháp — `try/except` không dùng được BAO QUANH một lời
gọi `pipe(...)`, chỉ dùng được bao quanh từng hàm riêng lẻ
::why
Gần đúng ở việc bạn cân nhắc PHẠM VI của `try/except` — một câu hỏi hợp
lý khi mới thấy nó bọc quanh cả một lời gọi phức tạp.

Chỗ lệch: `try/except` bọc quanh BẤT KỲ đoạn mã nào, kể cả một lời gọi
`pipe(...)` chứa nhiều bước bên trong — không có giới hạn nào về việc
đó. Nếu BẤT KỲ bước nào bên trong ném lỗi, `except` sẽ bắt được, đúng
như mọi lời gọi hàm khác.
::
:::
::::

::::code{#xu_ly_so}
Viết `xu_ly_so(vb)` — dùng `pipe()` ghép BA bước: chuẩn hoá (bỏ khoảng
trắng), chuyển thành số nguyên (`int()`), rồi nhân đôi. KHÔNG bắt lỗi
bên trong hàm — để lỗi (nếu `vb` không chuyển được thành số) bay thẳng
ra ngoài.

```python title=starter
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def xu_ly_so(vb):
    ___

print(xu_ly_so("  42  "))
```

```python title=solution
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def xu_ly_so(vb):
    chuan_hoa = lambda s: s.strip()
    chuyen_so = lambda s: int(s)
    nhan_2 = lambda n: n * 2
    return pipe(vb, chuan_hoa, chuyen_so, nhan_2)

print(xu_ly_so("  42  "))
```

```python title=test
assert xu_ly_so("  42  ") == 84, "phải chuẩn hoá, chuyển số, nhân đôi đúng"
assert xu_ly_so("10") == 20, "phải đúng với chuỗi số khác"
try:
    xu_ly_so("khong phai so")
    assert False, "phải NÉM lỗi khi chuỗi không chuyển được thành số"
except ValueError:
    pass
except AssertionError:
    raise
except Exception as e:
    assert False, f"phải ném ValueError, không phải {type(e).__name__}"
```

:::hints
- kind: attention
  body: "Dùng pipe() với ba bước: strip(), int(), rồi nhân đôi — KHÔNG bọc try/except bên trong xu_ly_so, để lỗi ValueError từ int() tự bay ra ngoài."
- kind: strategy
  body: 'chuan_hoa/chuyen_so/nhan_2 — ba lambda, rồi return pipe(vb, chuan_hoa, chuyen_so, nhan_2). Không cần xử lý lỗi ở đây — đó là việc của NGƯỜI GỌI hàm, không phải hàm này.'
- kind: one-line
  body: "chuan_hoa = lambda s: s.strip()\nchuyen_so = lambda s: int(s)\nnhan_2 = lambda n: n * 2\nreturn pipe(vb, chuan_hoa, chuyen_so, nhan_2)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: xu_ly_so phải dùng pipe() để ghép các bước (bài đang dạy đúng cách lỗi lan ra qua pipe), không viết gọn thành một biểu thức duy nhất bỏ qua pipe.
  requireAst:
  - kind: uses-call, target: pipe, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "84"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một pipeline không "nuốt" lỗi — nó dừng ĐÚNG chỗ, báo RÕ ràng. Im lặng
tiếp tục trên dữ liệu hỏng NGUY HIỂM hơn dừng lại và báo lỗi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm này gần chốt — bạn có `map`/`filter`/`reduce` (cụm 2), closure/
`partial`/`curry` (cụm 3), `compose`/`pipe`/method chaining (cụm 4), và
giờ là pipeline dữ liệu thật với debug + xử lý lỗi (cụm 5). Ghép TẤT CẢ
lại trước khi vào BOSS trông thế nào?

Bài sau đo tổng hợp lần cuối, trước khi khép track.
::::

::::checkpoint{mastery=0.8}
::::
