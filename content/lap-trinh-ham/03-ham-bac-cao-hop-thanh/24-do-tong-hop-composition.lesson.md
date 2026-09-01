---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.do-tong-hop-composition
title: "Đo tổng hợp: composition"
summary: "Viết một pipe xử lý dữ liệu qua ít nhất bốn bước, kết hợp map/filter/reduce từ cụm 2 làm các hàm trong chuỗi pipe. Không khái niệm mới — đo khả năng GHÉP LẠI mọi cụm trước."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.review-composition]
requires: [fp.method-chaining]
concepts: [fp.review-composition]
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
`compose`, `pipe`, method chaining — ba cách viết CÙNG một Ý ghép hàm.
Hôm nay: ghép TẤT CẢ, cùng `map`/`filter`/`reduce` từ cụm 2.
::::

::::explain{#ghep-toan-bo-cum}
`pipe()` (bài 21-22) không chỉ nhận hàm đơn giản — mỗi "bước" trong
chuỗi CÓ THỂ là bất kỳ hàm nào, kể cả một hàm được XÂY từ `map`/
`filter`/`reduce`:

```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def xu_ly_diem(diem):
    loc_dat = lambda ds: list(filter(lambda d: d >= 5, ds))
    nhan_doi = lambda ds: list(map(lambda d: d * 2, ds))
    tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)
    dinh_dang = lambda x: f"Tổng: {x}"
    return pipe(diem, loc_dat, nhan_doi, tong, dinh_dang)

print(xu_ly_diem([3, 7, 5, 9, 2]))
```

```text
Tổng: 42
```

Bốn bước, mỗi bước MỘT việc: `loc_dat` LỌC (`filter` — giữ điểm `>=
5`: `7, 5, 9`), `nhan_doi` BIẾN ĐỔI (`map` — nhân đôi: `14, 10, 18`),
`tong` GỘP (`reduce` — cộng dồn: `42`), `dinh_dang` ĐỊNH DẠNG (một hàm
thuần đơn giản). `pipe` nối cả bốn lại, chạy TUẦN TỰ, ĐÚNG thứ tự viết
— không khái niệm mới, chỉ GHÉP những gì cụm 2 (`map`/`filter`/
`reduce`) và cụm này (`pipe`) đã dạy riêng lẻ.
::::

::::example{#tung-buoc-hien-ra}
Theo dõi TỪNG bước, thấy `pipe` biến đổi dữ liệu dần dần:

```python title=readonly
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

diem = [3, 7, 5, 9, 2]

loc_dat = lambda ds: list(filter(lambda d: d >= 5, ds))
nhan_doi = lambda ds: list(map(lambda d: d * 2, ds))
tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)

buoc_1 = pipe(diem, loc_dat)
buoc_2 = pipe(diem, loc_dat, nhan_doi)
buoc_3 = pipe(diem, loc_dat, nhan_doi, tong)

print(buoc_1)
print(buoc_2)
print(buoc_3)
```

```text title=readonly
[7, 5, 9]
[14, 10, 18]
42
```

Mỗi `pipe(...)` chạy CÀNG NHIỀU bước thì kết quả CÀNG "biến đổi sâu"
hơn — `buoc_1` chỉ lọc, `buoc_2` lọc rồi nhân đôi, `buoc_3` lọc, nhân
đôi, rồi cộng dồn. Thêm một hàm vào cuối chuỗi `pipe(...)` KHÔNG cần
sửa các bước trước — mỗi bước hoàn toàn ĐỘC LẬP, chỉ quan tâm "nhận gì,
trả gì".
::::

::::predict{#doan-ghep-bon-buoc commitOnce}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

gia = [12000, 45000, 8000, 30000]

loc_re = lambda ds: list(filter(lambda g: g < 40000, ds))
giam_10 = lambda ds: list(map(lambda g: g * 0.9, ds))
dem = lambda ds: len(ds)

print(pipe(gia, loc_re, giam_10, dem))
```

Dòng cuối in ra gì?

:::opt{correct}
`3`
:::

:::opt
`[10800.0, 7200.0, 27000.0]` — vì bước cuối (`dem`) không được áp
dụng, `pipe` dừng lại ở bước `giam_10`
::why
Gần đúng ở việc bạn tính đúng KẾT QUẢ CỦA HAI BƯỚC ĐẦU (`loc_re` giữ
`12000, 8000, 30000` — ba giá dưới `40000`; `giam_10` nhân `0.9`: ra
đúng ba số đó) — hai bước đó tính đúng.

Chỗ lệch: `pipe` chạy HẾT MỌI hàm được truyền, không dừng ở áp CHÓT —
`dem` (bước THỨ BA, cuối cùng) CHẮC CHẮN được gọi, đếm số phần tử còn
lại trong danh sách (`3` giá), không phải trả về danh sách đó nguyên
vẹn.
::
:::

:::opt
Máy báo lỗi — `pipe` không nhận được BA hàm liên tiếp nếu hàm GIỮA
(`giam_10`) và hàm CUỐI (`dem`) có kiểu dữ liệu trả về khác nhau (danh
sách rồi số nguyên)
::why
Gần đúng ở việc bạn cân nhắc kiểu dữ liệu TRUYỀN QUA từng bước có cần
khớp nhau không — một câu hỏi hợp lý khi ghép nhiều hàm.

Chỗ lệch: `pipe` không hề kiểm tra kiểu dữ liệu — nó chỉ đơn giản đưa
KẾT QUẢ của bước trước làm ĐẦU VÀO của bước sau, bất kể kiểu gì.
`giam_10` trả về một `list`, `dem` nhận `list` đó và trả về một `int`
— hoàn toàn hợp lệ, không lỗi.
::
:::
::::

::::code{#xu_ly_diem}
Viết `xu_ly_diem(diem)` — dùng `pipe()` ghép ÍT NHẤT bốn bước: lọc điểm
`>= 5` (`filter`), nhân đôi từng điểm còn lại (`map`), cộng dồn thành
tổng (`reduce`), rồi định dạng thành chuỗi `f"Tổng: {tong}"`.

```python title=starter
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def xu_ly_diem(diem):
    ___

print(xu_ly_diem([3, 7, 5, 9, 2]))
```

```python title=solution
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def xu_ly_diem(diem):
    loc_dat = lambda ds: list(filter(lambda d: d >= 5, ds))
    nhan_doi = lambda ds: list(map(lambda d: d * 2, ds))
    tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)
    dinh_dang = lambda x: f"Tổng: {x}"
    return pipe(diem, loc_dat, nhan_doi, tong, dinh_dang)

print(xu_ly_diem([3, 7, 5, 9, 2]))
```

```python title=test
assert xu_ly_diem([3, 7, 5, 9, 2]) == "Tổng: 42", "phải lọc >=5, nhân đôi, cộng dồn, định dạng đúng"
assert xu_ly_diem([1, 2, 3]) == "Tổng: 0", "không điểm nào đạt thì tổng phải là 0"
assert xu_ly_diem([]) == "Tổng: 0", "danh sách rỗng phải ra Tổng: 0"
assert xu_ly_diem([10]) == "Tổng: 20", "một điểm duy nhất đạt điều kiện phải nhân đôi đúng"
```

:::hints
- kind: attention
  body: "Dùng pipe() với BỐN hàm: filter() lọc >=5, map() nhân đôi, reduce() cộng dồn, rồi một lambda định dạng chuỗi f-string. Mỗi bước một lambda riêng, truyền hết vào pipe()."
- kind: strategy
  body: 'loc_dat/nhan_doi/tong/dinh_dang — bốn lambda riêng biệt, rồi return pipe(diem, loc_dat, nhan_doi, tong, dinh_dang). Xem lại bài 9-11 nếu quên khuôn filter/map/reduce.'
- kind: one-line
  body: "loc_dat = lambda ds: list(filter(lambda d: d >= 5, ds))\nnhan_doi = lambda ds: list(map(lambda d: d * 2, ds))\ntong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)\ndinh_dang = lambda x: f\"Tổng: {x}\"\nreturn pipe(diem, loc_dat, nhan_doi, tong, dinh_dang)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: xu_ly_diem phải dùng CẢ pipe(), filter(), map(), VÀ reduce() — đây là bài ghép lại cả track, không phải viết tắt bằng comprehension hay vòng lặp.
  requireAst:
  - kind: uses-call, target: pipe, min: 1
  - kind: uses-call, target: filter, min: 1
  - kind: uses-call, target: map, min: 1
  - kind: uses-call, target: reduce, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Tổng: 42"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn bước nhỏ, ghép qua `pipe`, thành một chuỗi xử lý rõ ràng. Mỗi bước
độc lập, dễ hiểu riêng — nhưng cùng nhau làm được việc lớn hơn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm này (compose/pipe/method chaining) chốt lại — bạn đã có ĐỦ công cụ
ghép hàm. Nhưng mọi ví dụ tới giờ đều dùng dữ liệu NHỎ, sạch sẽ. Dữ
liệu thật thường lộn xộn hơn — cần chuẩn hoá, xử lý lỗi giữa chừng.

Cụm sau đưa `pipe` vào một tình huống dữ liệu THẬT.
::::

::::checkpoint{mastery=0.8}
::::
