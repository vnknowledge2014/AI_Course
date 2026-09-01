---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.do-tong-hop-truoc-boss
title: "Đo tổng hợp trước BOSS"
summary: "Một pipe xử lý dữ liệu thật (map/filter/reduce làm các bước), có tap để debug, mọi hàm thành phần đều THUẦN. Không giới thiệu khái niệm mới — ghép lại mọi Ý cụm 2-5 trước khi vào BOSS."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.hof-gate-review]
requires: [fp.pipeline-error-handling]
concepts: [fp.hof-gate-review]
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
Track này đã dạy rất nhiều mảnh: `map`/`filter`/`reduce`, closure/
`partial`/`curry`, `compose`/`pipe`, `tap`, xử lý lỗi. Hôm nay ghép TẤT
CẢ lại, một lần cuối trước BOSS.
::::

::::explain{#ghep-toan-bo-track}
Một pipeline xử lý đơn hàng THẬT, dùng ĐỦ những công cụ track đã dạy:

```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def tap(nhan):
    return lambda x: (print(f"{nhan}: {x}"), x)[1]

def xu_ly_don_hang(gia_ds):
    loc_hop_le = lambda ds: list(filter(lambda g: g > 0, ds))
    giam_10 = lambda ds: list(map(lambda g: g * 0.9, ds))
    tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)
    dinh_dang = lambda t: f"Tổng đơn: {t}"
    return pipe(gia_ds, loc_hop_le, tap("sau lọc"), giam_10, tap("sau giảm"), tong, dinh_dang)

print(xu_ly_don_hang([100, -5, 200, 0, 50]))
```

```text
sau lọc: [100, 200, 50]
sau giảm: [90.0, 180.0, 45.0]
Tổng đơn: 315.0
```

Không CÓ khái niệm mới trong đoạn này — mọi mảnh đều đã học riêng:
`filter` LOẠI giá không hợp lệ (bài 7), `map` GIẢM GIÁ từng phần tử
(bài 6), `reduce` GỘP thành tổng (bài 9-10), `tap` SOI giá trị giữa
chừng mà không đổi luồng (bài 27), `pipe` GHÉP tất cả thành MỘT chuỗi
(bài 21-22), mọi hàm thành phần đều THUẦN (bài 13, 26). Bài này chỉ ĐO
khả năng GHÉP LẠI, không dạy gì mới.
::::

::::example{#thay-doi-mot-buoc-khong-anh-huong-cac-buoc-khac}
Đúng tinh thần "ghép được an toàn cần thuần" (bài 26): sửa MỘT bước
không cần hiểu HAY sửa các bước khác:

```python title=readonly
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def xu_ly_don_hang(gia_ds, ty_le_giam):
    loc_hop_le = lambda ds: list(filter(lambda g: g > 0, ds))
    giam_gia = lambda ds: list(map(lambda g: g * (1 - ty_le_giam), ds))
    tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)

    return pipe(gia_ds, loc_hop_le, giam_gia, tong)

don = [100, -5, 200, 50]
print(xu_ly_don_hang(don, 0.1))
print(xu_ly_don_hang(don, 0.5))
```

```text title=readonly
315.0
175.0
```

Chỉ ĐỔI `ty_le_giam` (0.1 → 0.5) — `loc_hop_le` và `tong` KHÔNG cần sửa
gì, vẫn hoạt động đúng. Mỗi bước ĐỘC LẬP, chỉ quan tâm "nhận gì, trả
gì" — đây là LỢI ÍCH THẬT của việc ghép từ những mảnh nhỏ, thuần: sửa
một phần không kéo theo sửa dây chuyền.
::::

::::predict{#doan-ghep-toan-bo-track commitOnce}
```python
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

diem = [3, 8, -2, 10, 0, 6]

loc_duong = lambda ds: list(filter(lambda d: d > 0, ds))
nhan_2 = lambda ds: list(map(lambda d: d * 2, ds))
dem = lambda ds: len(ds)

print(pipe(diem, loc_duong, nhan_2, dem))
```

Dòng cuối in ra gì?

:::opt{correct}
`4`
:::

:::opt
`[6, 16, 20, 12]` — vì `dem` (bước cuối) bị bỏ qua, `pipe` dừng lại ở
kết quả của `nhan_2`
::why
Gần đúng ở việc bạn tính đúng KẾT QUẢ CỦA `nhan_2` (`loc_duong` giữ
`3, 8, 10, 6` — bốn số dương; `nhan_2` nhân đôi: `6, 16, 20, 12`) — hai
bước đó tính đúng.

Chỗ lệch: `pipe` chạy HẾT mọi hàm được truyền — `dem` (hàm CUỐI CÙNG)
CHẮC CHẮN được gọi, đếm SỐ LƯỢNG phần tử còn lại (`4` phần tử), không
phải trả về chính danh sách đó.
::
:::

:::opt
Máy báo lỗi — `loc_duong` giữ lại số `0`, nhưng `0` không phải "dương"
đúng nghĩa toán học, gây mâu thuẫn logic
::why
Gần đúng ở việc bạn để ý ĐÚNG một chi tiết toán học tinh tế: `0` không
phải số dương — quan sát đó chính xác.

Chỗ lệch: điều kiện lọc là `d > 0` — SO SÁNH này tự động LOẠI `0` (vì
`0 > 0` là `False`), không giữ nó lại. Không có mâu thuẫn hay lỗi nào —
`loc_duong` hoạt động ĐÚNG như ý định: giữ đúng các số DƯƠNG THẬT SỰ.
::
:::
::::

::::code{#xu_ly_don_hang_review}
Viết `xu_ly_don_hang(gia_ds)` — dùng `pipe()` ghép: lọc giá `> 0`
(`filter`), giảm giá 10% (`map`), gộp thành tổng (`reduce`), CHÈN một
`tap()` để soi giá trị SAU bước lọc, rồi định dạng kết quả thành chuỗi
`f"Tổng đơn: {tong}"`.

```python title=starter
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def tap(nhan):
    return lambda x: (print(f"{nhan}: {x}"), x)[1]

def xu_ly_don_hang(gia_ds):
    ___

print(xu_ly_don_hang([100, -5, 200, 0, 50]))
```

```python title=solution
from functools import reduce

def pipe(value, *ham):
    return reduce(lambda acc, f: f(acc), ham, value)

def tap(nhan):
    return lambda x: (print(f"{nhan}: {x}"), x)[1]

def xu_ly_don_hang(gia_ds):
    loc_hop_le = lambda ds: list(filter(lambda g: g > 0, ds))
    giam_10 = lambda ds: list(map(lambda g: g * 0.9, ds))
    tong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)
    dinh_dang = lambda t: f"Tổng đơn: {t}"
    return pipe(gia_ds, loc_hop_le, tap("sau loc"), giam_10, tap("sau giam"), tong, dinh_dang)

print(xu_ly_don_hang([100, -5, 200, 0, 50]))
```

```python title=test
assert xu_ly_don_hang([100, -5, 200, 0, 50]) == "Tổng đơn: 315.0", "phải lọc, giảm giá, cộng dồn, định dạng đúng"
assert xu_ly_don_hang([]) == "Tổng đơn: 0", "danh sách rỗng phải ra 0"
assert xu_ly_don_hang([-1, -2]) == "Tổng đơn: 0", "không giá nào hợp lệ thì ra 0"
assert xu_ly_don_hang([10]) == "Tổng đơn: 9.0", "một giá duy nhất phải giảm đúng 10%"
```

:::hints
- kind: attention
  body: "Dùng pipe() ghép NĂM bước: filter() lọc >0, tap() soi giá trị, map() giảm 10%, tap() soi lần nữa, reduce() cộng dồn, rồi một lambda định dạng. Xem lại bài 29's ví dụ explain nếu cần."
- kind: strategy
  body: 'loc_hop_le/giam_10/tong/dinh_dang — bốn lambda, cộng hai lời gọi tap() chèn giữa chừng, tất cả truyền vào MỘT lời gọi pipe(gia_ds, ...).'
- kind: one-line
  body: "loc_hop_le = lambda ds: list(filter(lambda g: g > 0, ds))\ngiam_10 = lambda ds: list(map(lambda g: g * 0.9, ds))\ntong = lambda ds: reduce(lambda acc, x: acc + x, ds, 0)\ndinh_dang = lambda t: f\"Tổng đơn: {t}\"\nreturn pipe(gia_ds, loc_hop_le, tap(\"sau loc\"), giam_10, tap(\"sau giam\"), tong, dinh_dang)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: xu_ly_don_hang phải dùng ĐỦ pipe(), filter(), map(), reduce(), VÀ tap() — đây là bài ghép lại cả track trước BOSS, không phải viết tắt bằng comprehension.
  requireAst:
  - kind: uses-call, target: pipe, min: 1
  - kind: uses-call, target: filter, min: 1
  - kind: uses-call, target: map, min: 1
  - kind: uses-call, target: reduce, min: 1
  - kind: uses-call, target: tap, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Tổng đơn: 315.0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mọi mảnh của track — ghép lại trong một pipeline thật, không mảnh nào
xa lạ. Sẵn sàng cho BOSS.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã ghép được tất cả — nhưng luôn theo ĐÚNG khuôn ví dụ track đưa
ra. BOSS không đưa khuôn sẵn — chỉ đưa một bài toán, và đòi TỰ CHỌN
công cụ nào ghép với công cụ nào.

Sẵn sàng chưa?
::::

::::checkpoint{mastery=0.8}
::::
