---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.boss-hof-composition
title: "BOSS — Khép track HOF & Composition"
summary: "Viết một chương trình nhỏ dùng ĐỦ: hàm là giá trị, map/filter/reduce, closure thuần + curry, pipe, một pipeline thật có xử lý lỗi. Khép track — không dạy khái niệm mới, chỉ đòi ghép lại."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.hof-gate-boss]
requires: [fp.hof-gate-review]
concepts: [fp.hof-gate-boss]
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
Hai mươi chín bài để tới đây. Một Ý gốc xuyên suốt tất cả: hàm không
chỉ chạy — nó còn là một GIÁ TRỊ, truyền được, ghép được. Giờ ghép hết
lại, trong một chương trình duy nhất.
::::

::::explain{#nam-manh-ghep}
Suốt track này, mọi bài đều là hệ quả của đúng MỘT Ý gốc — **hàm là
một giá trị bình thường: gán được vào biến, truyền được làm đối số,
trả về được từ hàm khác**. Năm mảnh ghép, tất cả cùng xoay quanh Ý đó:

1. **Hàm là giá trị** (cụm 1) — gán hàm vào biến, truyền hàm làm đối
   số cho hàm khác, không có gì "đặc biệt" về cú pháp.
2. **`map`/`filter`/`reduce`** (cụm 2) — ba công cụ biến đổi/lọc/gộp
   một dãy, nhận HÀM làm đối số để nói "làm gì với TỪNG phần tử".
3. **Closure thuần + `curry`** (cụm 3) — một hàm trả về hàm khác, "nhớ"
   được biến của hàm ngoài; `curry2(f)` biến `f(a,b)` thành `f(a)(b)`.
4. **`pipe`** (cụm 4) — ghép NHIỀU hàm thành một chuỗi, chạy tuần tự,
   đọc xuôi tự nhiên (trái sang phải).
5. **Pipeline thật, có xử lý lỗi** (cụm 5) — mỗi bước THUẦN, một mắt
   xích lỗi thì DỪNG NGAY, không lặng lẽ tiếp tục trên dữ liệu hỏng.

Năm mảnh đó là toàn bộ nguyên liệu để viết một chương trình xử lý đơn
hàng nhỏ: chuyển giá từ chuỗi thành số, lọc giá hợp lệ, áp dụng giảm
giá (dùng `curry2`), cộng tổng, định dạng kết quả.
::::

::::example{#chuong-trinh-ghep-du-nam-manh}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def curry2(f):
    return lambda a: lambda b: f(a, b)

def ap_dung_giam_gia(ty_le, gia):
    return gia * (1 - ty_le)

giam_theo_ty_le = curry2(ap_dung_giam_gia)

def xu_ly_don_hang(gia_chuoi_ds, ty_le_giam):
    chuyen_so = lambda ds: list(map(float, ds))
    loc_hop_le = lambda ds: list(filter(lambda g: g > 0, ds))
    giam_gia = lambda ds: list(map(giam_theo_ty_le(ty_le_giam), ds))
    tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)
    dinh_dang = lambda t: f"Tổng: {round(t, 2)}"

    return pipe(gia_chuoi_ds, chuyen_so, loc_hop_le, giam_gia, tong, dinh_dang)

print(xu_ly_don_hang(["100", "-5", "200", "50"], 0.1))
```

```text
Tổng: 315.0
```

Đọc ra năm mảnh: `float` được TRUYỀN THẲNG vào `map(float, ds)` như
một GIÁ TRỊ, không gọi `float(x)` cho từng phần tử tay (mảnh 1).
`chuyen_so`/`loc_hop_le`/`giam_gia`/`tong` đều xây từ `map`/`filter`/
`reduce` (mảnh 2). `giam_theo_ty_le = curry2(ap_dung_giam_gia)` rồi
`giam_theo_ty_le(ty_le_giam)` — một closure ĐÃ CHỐT SẴN `ty_le`, dùng
LÀM đối số cho `map()` (mảnh 3). `pipe(...)` ghép NĂM bước, chạy tuần
tự, đọc xuôi từ trái sang phải (mảnh 4). Nếu MỘT giá trong danh sách
không chuyển được thành số (`float("xyz")` ném `ValueError`), cả
`pipe` dừng NGAY — mảnh 5, đã đo ở bài 28.
::::

::::predict{#doan-loi-lan-trong-chuong-trinh-ghep commitOnce}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def curry2(f):
    return lambda a: lambda b: f(a, b)

def ap_dung_giam_gia(ty_le, gia):
    return gia * (1 - ty_le)

giam_theo_ty_le = curry2(ap_dung_giam_gia)

def xu_ly_don_hang(gia_chuoi_ds, ty_le_giam):
    chuyen_so = lambda ds: list(map(float, ds))
    loc_hop_le = lambda ds: list(filter(lambda g: g > 0, ds))
    giam_gia = lambda ds: list(map(giam_theo_ty_le(ty_le_giam), ds))
    tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)
    dinh_dang = lambda t: f"Tổng: {round(t, 2)}"
    return pipe(gia_chuoi_ds, chuyen_so, loc_hop_le, giam_gia, tong, dinh_dang)

gia_ds = ["50", "xyz", "100"]
try:
    print(xu_ly_don_hang(gia_ds, 0.2))
except ValueError:
    print("lỗi: không chuyển được số")
```

**Trước khi chạy**, chương trình in ra gì?

:::opt{correct}
`lỗi: không chuyển được số`
:::

:::opt
`Tổng: 120.0` — vì `chuyen_so` bỏ qua ÂM THẦM những giá trị không
chuyển được, chỉ giữ lại `"50"` và `"100"` (giảm 20%: `40 + 80 = 120`)
::why
Gần đúng ở việc bạn tính đúng KẾT QUẢ NẾU `"xyz"` bị bỏ qua
(`50*0.8 + 100*0.8 = 120`) — phép tính giả định đó đúng.

Chỗ lệch: `chuyen_so = lambda ds: list(map(float, ds))` KHÔNG có cơ chế
"bỏ qua âm thầm" nào — `float("xyz")` ném `ValueError` NGAY LẬP TỨC khi
`list(map(...))` cố ép TOÀN BỘ kết quả ra danh sách. Lỗi đó bay thẳng
ra khỏi `chuyen_so`, dừng cả `pipe` — đúng luật bài 26 và bài 28: một
mắt xích hỏng, dừng ngay, không lặng lẽ tiếp tục.
::
:::

:::opt
`lỗi: không chuyển được số`, nhưng chương trình CÒN in thêm một dòng
`Tổng: 0` phía SAU, vì `pipe` vẫn cố chạy hết các bước còn lại với
danh sách rỗng sau khi bắt lỗi
::why
Gần đúng ở việc bạn xác định ĐÚNG dòng lỗi sẽ in ra — dòng đó đúng.

Chỗ lệch: khối `except ValueError` chỉ IN RA MỘT dòng
(`"lỗi: không chuyển được số"`) rồi KẾT THÚC khối `try/except` —
không có mã nào SAU đó cố "chạy tiếp" `pipe` với dữ liệu rỗng.
Chương trình chỉ in ĐÚNG MỘT dòng, không có dòng `Tổng: 0` nào theo
sau.
::
:::
::::

::::code{#xu_ly_don_hang_boss}
Chương trình đã đủ bốn mảnh — `curry2`, `map`/`filter`/`reduce`,
`float` truyền như giá trị. Hoàn thành mảnh cuối: ghép TẤT CẢ các bước
lại thành một chuỗi `pipe()` duy nhất, ĐÚNG thứ tự (chuyển số → lọc hợp
lệ → giảm giá → cộng tổng → định dạng).

```python title=starter
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def curry2(f):
    return lambda a: lambda b: f(a, b)

def ap_dung_giam_gia(ty_le, gia):
    return gia * (1 - ty_le)

giam_theo_ty_le = curry2(ap_dung_giam_gia)

def xu_ly_don_hang(gia_chuoi_ds, ty_le_giam):
    chuyen_so = lambda ds: list(map(float, ds))
    loc_hop_le = lambda ds: list(filter(lambda g: g > 0, ds))
    giam_gia = lambda ds: list(map(giam_theo_ty_le(ty_le_giam), ds))
    tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)
    dinh_dang = lambda t: f"Tổng: {round(t, 2)}"

    ___

print(xu_ly_don_hang(["100", "-5", "200", "50"], 0.1))
```

```python title=solution
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def curry2(f):
    return lambda a: lambda b: f(a, b)

def ap_dung_giam_gia(ty_le, gia):
    return gia * (1 - ty_le)

giam_theo_ty_le = curry2(ap_dung_giam_gia)

def xu_ly_don_hang(gia_chuoi_ds, ty_le_giam):
    chuyen_so = lambda ds: list(map(float, ds))
    loc_hop_le = lambda ds: list(filter(lambda g: g > 0, ds))
    giam_gia = lambda ds: list(map(giam_theo_ty_le(ty_le_giam), ds))
    tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)
    dinh_dang = lambda t: f"Tổng: {round(t, 2)}"

    return pipe(gia_chuoi_ds, chuyen_so, loc_hop_le, giam_gia, tong, dinh_dang)

print(xu_ly_don_hang(["100", "-5", "200", "50"], 0.1))
```

```python title=test
assert xu_ly_don_hang(["100", "-5", "200", "50"], 0.1) == "Tổng: 315.0", "phải chuyển số, lọc, giảm giá, cộng dồn đúng"
assert xu_ly_don_hang([], 0.1) == "Tổng: 0", "danh sách rỗng phải ra 0"
assert xu_ly_don_hang(["10"], 0.5) == "Tổng: 5.0", "một giá duy nhất phải giảm đúng tỷ lệ"

da_nem = False
try:
    xu_ly_don_hang(["100", "khong phai so"], 0.1)
except ValueError:
    da_nem = True
assert da_nem, "phải ném lỗi khi có chuỗi không chuyển được thành số — KHÔNG được bắt lỗi bên trong hàm"
```

:::hints
- kind: attention
  body: "Chỗ trống là MỘT dòng return, ghép NĂM bước đã có sẵn (chuyen_so, loc_hop_le, giam_gia, tong, dinh_dang) vào một lời gọi pipe() duy nhất, ĐÚNG thứ tự đã liệt kê."
- kind: strategy
  body: 'return pipe(gia_chuoi_ds, chuyen_so, loc_hop_le, giam_gia, tong, dinh_dang) — gia_chuoi_ds là điểm khởi đầu, năm hàm còn lại chạy tuần tự, đúng thứ tự đã định nghĩa phía trên.'
- kind: one-line
  body: "return pipe(gia_chuoi_ds, chuyen_so, loc_hop_le, giam_gia, tong, dinh_dang)"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: xu_ly_don_hang phải ghép các bước bằng pipe(gia_chuoi_ds, ...) — không gọi lồng nhau tay (dinh_dang(tong(...))) hay tính tay lại bằng vòng lặp/comprehension.
  requireAst:
  - kind: uses-call, target: pipe, min: 1
  - kind: uses-name, target: gia_chuoi_ds, min: 1
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "Tổng: 315.0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hàm là giá trị, `map`/`filter`/`reduce`, closure/`curry`, `pipe`, một
pipeline thật biết dừng đúng chỗ khi lỗi — mọi mảnh ghép cùng đứng
trong một chương trình. Track HOF & Composition khép lại ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả track này.

Ba mươi bài, đi từ một hàm được gán vào một cái tên như bất kỳ giá trị
nào khác, tới một pipeline xử lý dữ liệu THẬT — ghép từ những mảnh nhỏ,
mỗi mảnh THUẦN, mỗi mảnh hiểu được riêng lẻ, nhưng cùng nhau làm được
việc lớn hơn nhiều so với tổng từng mảnh viết tay.

Bạn mang theo đúng MỘT cách NHÌN sang track sau: một chương trình không
phải một CHUỖI LỆNH sửa trạng thái từng bước — nó là một CHUỖI PHÉP
BIẾN ĐỔI DỮ LIỆU, mỗi bước nhận một giá trị, trả về một giá trị mới.
Track sau (T4.3, quay lại TypeScript) hỏi tiếp: nếu dữ liệu tự nó có
THỂ mang nhiều HÌNH DẠNG khác nhau (một đơn hàng có thể "đang chờ", "đã
giao", hay "đã huỷ" — không phải lúc nào cũng cùng một cấu trúc), làm
sao mô hình hoá điều đó một cách AN TOÀN, để trình biên dịch tự bắt
được những trường hợp bạn quên xử lý?
::::

::::checkpoint{mastery=0.85}
::::
