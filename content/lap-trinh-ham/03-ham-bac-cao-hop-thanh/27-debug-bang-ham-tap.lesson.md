---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.debug-bang-ham-tap
title: "Debug một pipeline bằng hàm `tap` — soi giá trị giữa chừng, không đổi kết quả"
summary: "def tap(nhan): return lambda x: (print(f'{nhan}: {x}'), x)[1] — chèn tap('sau loc') vào GIỮA chuỗi pipe để IN ra giá trị tại đúng bước đó, rồi TRẢ LẠI giá trị y hệt, không đổi luồng dữ liệu."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [fp.debug-with-tap]
requires: [fp.composability-needs-purity]
concepts: [fp.debug-with-tap]
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
Một `pipe` dài nhiều bước, ra kết quả sai — bước nào gây ra? Tách cả
chuỗi ra chạy từng phần rất phiền. Có cách nhanh hơn không?
::::

::::explain{#tap-la-gi}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def tap(nhan):
    return lambda x: (print(f"{nhan}: {x}"), x)[1]

cong_1 = lambda x: x + 1
nhan_2 = lambda x: x * 2

ket_qua = pipe(3, cong_1, tap("sau cong"), nhan_2, tap("sau nhan"))
print("kết quả cuối:", ket_qua)
```

```text
sau cong: 4
sau nhan: 8
kết quả cuối: 8
```

`tap(nhan)` trả về một hàm IN RA giá trị nó nhận (kèm nhãn `nhan` để
biết ĐANG Ở BƯỚC NÀO), rồi TRẢ LẠI giá trị đó Y NGUYÊN — không đổi gì
cả. Chèn `tap("sau cong")` NGAY SAU `cong_1` trong chuỗi `pipe`, bạn
thấy giá trị TẠI ĐÚNG điểm đó (`4`, sau khi `cong_1` chạy), mà không
cần tách `pipe` ra viết lại hay thêm `print()` thủ công vào TỪNG hàm.

`(print(...), x)[1]` là một mẹo nhỏ: `print(...)` LUÔN trả về `None`,
nên `(print(...), x)` tạo một tuple `(None, x)`, rồi `[1]` lấy phần tử
THỨ HAI (chính là `x`) — kết quả cuối cùng là `x`, không đổi, nhưng
`print(...)` đã CHẠY như một side effect dọc đường.
::::

::::example{#tap-khong-doi-luong-du-lieu}
Chèn hay bỏ `tap` KHÔNG làm đổi kết quả CUỐI của pipeline — chỉ đổi
những gì được IN RA màn hình:

```python title=readonly
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def tap(nhan):
    return lambda x: (print(f"{nhan}: {x}"), x)[1]

cong_1 = lambda x: x + 1
nhan_2 = lambda x: x * 2

khong_tap = pipe(3, cong_1, nhan_2)
co_tap = pipe(3, cong_1, tap("giua chung"), nhan_2)

print(khong_tap)
print(co_tap)
print(khong_tap == co_tap)
```

```text title=readonly
8
giua chung: 4
8
True
```

`khong_tap` và `co_tap` — CÙNG kết quả (`8`). `tap` chỉ THÊM một dòng
IN RA (`"giua chung: 4"`), không hề đổi giá trị chạy QUA nó. Đây là
điểm mấu chốt: `tap` AN TOÀN chèn vào BẤT KỲ đâu trong `pipe` để debug,
không lo làm hỏng logic — nó chỉ QUAN SÁT, không CAN THIỆP.
::::

::::predict{#doan-tap-khong-doi-ket-qua commitOnce}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def tap(nhan):
    return lambda x: (print(f"{nhan}: {x}"), x)[1]

nhan_3 = lambda x: x * 3
tru_1 = lambda x: x - 1

ket_qua = pipe(5, nhan_3, tap("A"), tru_1, tap("B"))
print(ket_qua)
```

Chương trình in ra MẤY dòng, và dòng CUỐI CÙNG là gì?

:::opt{correct}
Ba dòng: `A: 15`, `B: 14`, `14`
:::

:::opt
Một dòng duy nhất: `14` — vì `tap` chỉ TRẢ VỀ giá trị, không IN RA gì
thêm ngoài kết quả cuối
::why
Gần đúng ở việc bạn tính ĐÚNG kết quả CUỐI (`5*3=15`, rồi `15-1=14`) —
phép tính đó đúng.

Chỗ lệch: `tap(nhan)` LUÔN gọi `print(...)` MỖI LẦN hàm nó trả về được
gọi — không chỉ "trả về giá trị" mà còn IN RA một dòng CÓ SIDE EFFECT.
Hai lần chèn `tap("A")` và `tap("B")` in ra HAI dòng riêng, CỘNG THÊM
dòng `print(ket_qua)` ở cuối — BA dòng tổng cộng, không phải một.
::
:::

:::opt
Ba dòng, nhưng thứ tự KHÁC: `B: 14`, `A: 15`, `14` — vì `tap` được ĐỊNH
NGHĨA trước khi `pipe` chạy, nên IN RA theo thứ tự ĐỊNH NGHĨA, không
theo thứ tự CHẠY trong chuỗi
::why
Gần đúng ở việc bạn để ý CẢ HAI lời gọi `tap` đều xảy ra — đúng, có hai
dòng debug được in ra từ `tap`.

Chỗ lệch: `tap("A")` và `tap("B")` chỉ TẠO RA hai hàm khi được GỌI
trong lời gọi `pipe(...)` (dòng cuối) — chúng CHƯA in gì cho tới khi
`pipe` CHẠY và tới lượt TỪNG BƯỚC. Thứ tự IN RA đúng THỨ TỰ CHẠY trong
`pipe` (trái sang phải, bài 21): `A` trước (`nhan_3` vừa chạy xong),
rồi `B` sau (`tru_1` vừa chạy xong).
::
:::
::::

::::code{#tap}
Tự viết `tap(nhan)` — trả về một hàm IN RA `f"{nhan}: {x}"` khi nhận
`x`, rồi TRẢ LẠI `x` Y NGUYÊN, không đổi giá trị.

```python title=starter
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def tap(nhan):
    ___

cong_1 = lambda x: x + 1
nhan_2 = lambda x: x * 2
ket_qua = pipe(3, cong_1, tap("sau cong"), nhan_2)
print(ket_qua)
```

```python title=solution
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def tap(nhan):
    return lambda x: (print(f"{nhan}: {x}"), x)[1]

cong_1 = lambda x: x + 1
nhan_2 = lambda x: x * 2
ket_qua = pipe(3, cong_1, tap("sau cong"), nhan_2)
print(ket_qua)
```

```python title=test
assert tap("x")(5) == 5, "tap phải trả về ĐÚNG giá trị đưa vào, không đổi"
assert tap("y")(0) == 0, "tap phải không đổi giá trị ngay cả khi giá trị là 0"
assert tap("z")("chuoi") == "chuoi", "tap phải hoạt động với BẤT KỲ kiểu dữ liệu nào, không chỉ số"
assert ket_qua == 8, "chèn tap vào giữa pipe KHÔNG được làm đổi luồng dữ liệu, kết quả cuối vẫn đúng"
```

:::hints
- kind: attention
  body: "tap(nhan) phải trả về một lambda IN RA f\"{nhan}: {x}\" rồi TRẢ LẠI x — không phải chỉ trả về x (thiếu print thì mất tác dụng debug, dù test giá trị vẫn qua)."
- kind: strategy
  body: 'return lambda x: (print(f"{nhan}: {x}"), x)[1] — print(...) luôn trả None, tuple (None, x) rồi lấy [1] chính là x, không đổi.'
- kind: one-line
  body: "return lambda x: (print(f\"{nhan}: {x}\"), x)[1]"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: tap phải THẬT SỰ in ra giá trị (dùng print()) trước khi trả lại — chỉ trả về x không đổi (như một hàm identity) là thiếu tác dụng debug bài này đang dạy, dù test giá trị vẫn qua. (Đếm CẢ dòng print(ket_qua) có sẵn ở cuối bài — lời giải đúng phải có HAI lời gọi print trở lên, không chỉ một.)
  requireAst:
  - kind: uses-call, target: print, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "sau cong: 4"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`tap` soi được giá trị GIỮA chừng một pipeline mà không đổi kết quả —
một công cụ debug nhỏ, an toàn chèn vào bất kỳ đâu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tap` giúp NHÌN thấy giá trị giữa chừng — nhưng nếu một bước trong
pipeline không chỉ "giá trị sai", mà THỰC SỰ NÉM RA một lỗi (ví dụ
`int("abc")`)? Chuyện gì xảy ra với các bước SAU nó?

Bài sau đo đúng điều đó.
::::

::::checkpoint{mastery=0.8}
::::
