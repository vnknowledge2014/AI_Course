---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.functools-partial-chot-san-doi-so
title: "`functools.partial` — chốt sẵn MỘT PHẦN đối số, tạo hàm mới"
summary: "from functools import partial; def luy_thua(co_so, so_mu): return co_so ** so_mu; binh_phuong = partial(luy_thua, so_mu=2) rồi binh_phuong(5) == 25 — cách KHÁC để làm việc closure bài 12-13 đã làm, dùng thư viện có sẵn."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [fp.functools-partial]
requires: [fp.closure-impure-aware]
concepts: [fp.functools-partial]
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
Bài 12-13 tự tay viết `def lam_bang(k): return lambda x: x * k` để "chốt
sẵn" một tham số. Hôm nay: một công cụ có sẵn làm ĐÚNG việc đó, không
cần viết `lambda`/`def` lồng nhau.
::::

::::explain{#partial-la-gi}
```python
from functools import partial

def luy_thua(co_so, so_mu):
    return co_so ** so_mu

binh_phuong = partial(luy_thua, so_mu=2)
print(binh_phuong(5))
print(binh_phuong(7))
```

```text
25
49
```

`partial(luy_thua, so_mu=2)` tạo ra một hàm MỚI (`binh_phuong`) đã
"chốt sẵn" `so_mu=2` — gọi `binh_phuong(5)` tương đương gọi
`luy_thua(5, so_mu=2)`. `partial` không phải cú pháp lạ hay phép màu —
nó làm ĐÚNG việc closure bài 12-13 làm (tạo một hàm "nhớ sẵn" một phần
dữ liệu), chỉ khác CÔNG CỤ: dùng thư viện có sẵn (`functools.partial`)
thay vì tự viết `def lam_bang(k): return lambda x: x * k`.

So sánh hai cách viết CÙNG một Ý:

```python
def lam_binh_phuong(so_mu_co_dinh):
    return lambda co_so: co_so ** so_mu_co_dinh

cach_closure = lam_binh_phuong(2)
cach_partial = partial(luy_thua, so_mu=2)

print(cach_closure(5))
print(cach_partial(5))
```

```text
25
25
```

CÙNG kết quả, hai cách viết. `partial` gọn hơn khi hàm gốc (`luy_thua`)
ĐÃ TỒN TẠI SẴN — không cần bọc thêm một `def`/`lambda` mới, chỉ cần
"chốt" một đối số của hàm CÓ SẴN.
::::

::::example{#nhieu-partial-doc-lap}
Giống closure, mỗi lần gọi `partial()` tạo một hàm MỚI, ĐỘC LẬP:

```python title=readonly
from functools import partial

def luy_thua(co_so, so_mu):
    return co_so ** so_mu

binh_phuong = partial(luy_thua, so_mu=2)
lap_phuong = partial(luy_thua, so_mu=3)

print(lap_phuong(2))
print(binh_phuong(4))
```

```text title=readonly
8
16
```

Tạo `lap_phuong` (chốt `so_mu=3`) không hề đụng tới `binh_phuong` (chốt
`so_mu=2`) — `binh_phuong(4)` vẫn tính `4 ** 2 = 16` như bình thường,
dù `lap_phuong` được tạo SAU và dùng CÙNG hàm gốc `luy_thua`.
::::

::::predict{#doan-partial-doc-lap commitOnce}
```python
from functools import partial

def nhan(a, b):
    return a * b

nhan_3 = partial(nhan, b=3)
nhan_10 = partial(nhan, b=10)

print(nhan_3(5))
print(nhan_10(5))
print(nhan_3(2))
```

Ba dòng in ra gì, theo đúng thứ tự?

:::opt{correct}
`15` rồi `50` rồi `6`
:::

:::opt
`15` rồi `50` rồi `15` — vì `nhan_3(2)` dùng đối số ĐÃ GỌI lần trước
(`5`), không phải đối số mới (`2`)
::why
Gần đúng ở việc bạn tính đúng hai kết quả đầu (`5*3=15`, `5*10=50`) —
hai phép nhân đó đúng.

Chỗ lệch: mỗi LẦN GỌI `nhan_3(x)` dùng ĐÚNG đối số `x` được truyền NGAY
LẦN GỌI ĐÓ, không "nhớ" đối số của lần gọi trước — `nhan_3` chỉ chốt
sẵn `b=3` (cố định vĩnh viễn), còn `a` (`x`) LUÔN LÀ đối số mới.
`nhan_3(2)` tính `2 * 3 = 6`, không liên quan gì tới `nhan_3(5)` trước
đó.
::
:::

:::opt
Máy báo lỗi ở dòng `nhan_10 = partial(nhan, b=10)` — không tạo được
`partial` THỨ HAI từ CÙNG một hàm `nhan`
::why
Gần đúng ở việc bạn cảnh giác về việc tạo NHIỀU `partial` từ cùng một
hàm gốc — một mối lo hợp lý nếu chưa chắc cơ chế này hoạt động ra sao.

Chỗ lệch: tạo NHIỀU `partial` từ CÙNG một hàm là chuyện HOÀN TOÀN BÌNH
THƯỜNG — mỗi lần gọi `partial(...)` tạo một hàm MỚI, độc lập, không
giới hạn số lần (giống hệt closure ở bài 12).
::
:::
::::

::::code{#dung_partial}
Viết `tinh_gia_sau_giam(gia, phan_tram)` — hàm HAI tham số, tính giá
sau khi giảm. Rồi dùng `partial()` tạo `giam_20` — một hàm CHỈ CẦN
`gia`, đã chốt sẵn `phan_tram=20`.

```python title=starter
from functools import partial

def tinh_gia_sau_giam(gia, phan_tram):
    return gia * (1 - phan_tram / 100)

___

print(giam_20(100))
```

```python title=solution
from functools import partial

def tinh_gia_sau_giam(gia, phan_tram):
    return gia * (1 - phan_tram / 100)

giam_20 = partial(tinh_gia_sau_giam, phan_tram=20)

print(giam_20(100))
```

```python title=test
assert giam_20(100) == 80.0, "giảm 20% trên 100 phải ra 80.0"
assert giam_20(50) == 40.0, "giảm 20% trên 50 phải ra 40.0"
giam_50 = partial(tinh_gia_sau_giam, phan_tram=50)
assert giam_50(100) == 50.0, "giảm 50% trên 100 phải ra 50.0"
assert giam_20(100) == 80.0, "giam_20 không được bị ảnh hưởng bởi việc tạo giam_50 sau đó"
```

:::hints
- kind: attention
  body: "Dùng partial(tinh_gia_sau_giam, phan_tram=20) — không viết lại lambda hay def mới, partial tự tạo hàm mới từ tinh_gia_sau_giam đã có sẵn."
- kind: strategy
  body: 'giam_20 = partial(tinh_gia_sau_giam, phan_tram=20) — chốt sẵn phan_tram bằng từ khoá, chừa lại gia để truyền sau.'
- kind: one-line
  body: "giam_20 = partial(tinh_gia_sau_giam, phan_tram=20)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: giam_20 phải được tạo bằng functools.partial() (bài đang dạy đúng công cụ đó), không phải tự viết lại một hàm bọc bằng def/lambda.
  requireAst:
  - kind: uses-call, target: partial, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "80.0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`partial` chốt sẵn một phần đối số, trả về một hàm mới — đúng việc
closure làm, chỉ khác công cụ: dùng thư viện có sẵn, không tự viết.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`partial` chốt sẵn MỘT tham số bằng MỘT lần gọi. Nếu muốn chốt DẦN
TỪNG tham số một, mỗi lần MỘT cái — `f(a)(b)` thay vì `f(a, b)` — thì
làm sao?

Bài sau: tự viết một công cụ làm đúng điều đó.
::::

::::checkpoint{mastery=0.8}
::::
