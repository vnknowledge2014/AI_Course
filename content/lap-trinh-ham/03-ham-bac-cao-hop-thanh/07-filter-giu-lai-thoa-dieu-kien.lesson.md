---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.filter-giu-lai-thoa-dieu-kien
title: "`filter()` — giữ lại phần tử THOẢ điều kiện"
summary: "filter(lambda x: x % 2 == 0, xs) — cùng bẫy iterator như map() (cần list()). Khác map() (biến đổi giá trị), filter() chỉ QUYẾT ĐỊNH giữ hay bỏ."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [fp.filter-basics]
requires: [fp.map-basics]
concepts: [fp.filter-basics]
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
`map()` biến đổi MỌI phần tử, không bỏ cái nào. Hôm nay: một công cụ chỉ
GIỮ LẠI phần tử thoả điều kiện, bỏ phần còn lại.
::::

::::explain{#filter-la-gi}
`filter(dieu_kien, xs)` giữ lại CHỈ những phần tử mà `dieu_kien(x)` trả
`True`:

```python
xs = [1, 2, 3, 4, 5]
r = filter(lambda x: x % 2 == 0, xs)
print(r)
print(list(r))
```

```text
<filter object at 0x...>
[2, 4]
```

Cùng bẫy đã đo ở `map()`: `filter()` cũng trả về một **iterator**, chưa
phải `list` — cần `list(filter(...))`. Khác biệt CỐT LÕI với `map()`:

- `map(f, xs)`: BIẾN ĐỔI từng phần tử — số lượng phần tử ĐẦU RA bằng số
  lượng phần tử ĐẦU VÀO.
- `filter(dieu_kien, xs)`: LOẠI BỎ một số phần tử — số lượng đầu ra
  THƯỜNG ÍT HƠN đầu vào (hoặc bằng, nếu mọi phần tử đều thoả).

Hàm truyền cho `map()` TRẢ VỀ giá trị mới. Hàm truyền cho `filter()`
TRẢ VỀ `True`/`False` (giữ hay bỏ) — bản thân giá trị KHÔNG đổi.
::::

::::example{#map-va-filter-doi-chieu}
Cùng một danh sách, chạy qua CẢ HAI, thấy rõ khác biệt:

```python title=readonly
xs = [1, 2, 3, 4, 5]

qua_map = list(map(lambda x: x % 2 == 0, xs))
qua_filter = list(filter(lambda x: x % 2 == 0, xs))

print(qua_map)
print(qua_filter)
```

```text title=readonly
[False, True, False, True, False]
[2, 4]
```

`qua_map` có ĐỦ 5 phần tử (mỗi cái là `True`/`False`) — `map()` KHÔNG bỏ
gì cả, chỉ đổi giá trị. `qua_filter` chỉ có 2 phần tử (`2` và `4`) —
`filter()` LOẠI BỎ những cái không thoả, giữ nguyên GIÁ TRỊ GỐC của
những cái còn lại (không phải `True`/`False`, mà chính `2` và `4`).
::::

::::predict{#doan-nham-map-filter commitOnce}
```python
diem = [3, 8, 5, 9, 2, 10]

qua_mon = filter(lambda d: d >= 5, diem)

print(list(qua_mon))
```

Dòng cuối in ra gì?

:::opt{correct}
`[8, 5, 9, 10]`
:::

:::opt
`[True, False, True, True, False, True]`
::why
Gần đúng ở việc bạn tính ĐÚNG điểm nào `>= 5` — phép so sánh đó không
sai.

Chỗ lệch: đây là kết quả của `map()`, không phải `filter()`. `filter()`
KHÔNG trả về `True`/`False` cho từng phần tử — nó dùng `True`/`False`
để QUYẾT ĐỊNH giữ hay bỏ, rồi trả về CHÍNH giá trị gốc của những phần tử
được giữ (`8`, `5`, `9`, `10` — đúng bốn điểm `>= 5`).
::
:::

:::opt
`4` — vì có 4 điểm thoả điều kiện `>= 5`, và `filter()` đếm số lượng
::why
Gần đúng ở việc bạn đếm ĐÚNG số phần tử thoả điều kiện (4 điểm: 8, 5, 9,
10) — con số đó đúng.

Chỗ lệch: `filter()` không ĐẾM — nó TRẢ VỀ chính các phần tử đó (một
iterator, ép ra là một `list` bốn phần tử), không phải một con số đếm
được. Muốn ĐẾM, phải gọi thêm `len(list(qua_mon))` — một bước KHÁC,
chưa có ở đây.
::
:::
::::

::::code{#loc_du_hang}
Viết `loc_du_hang(ds_ton_kho, muc_toi_thieu)` — GIỮ LẠI những số lượng
tồn kho `>= muc_toi_thieu`, dùng `filter()`.

```python title=starter
def loc_du_hang(ds_ton_kho, muc_toi_thieu):
    ___

print(loc_du_hang([2, 10, 5, 20, 1], 5))
```

```python title=solution
def loc_du_hang(ds_ton_kho, muc_toi_thieu):
    return list(filter(lambda sl: sl >= muc_toi_thieu, ds_ton_kho))

print(loc_du_hang([2, 10, 5, 20, 1], 5))
```

```python title=test
assert loc_du_hang([2, 10, 5, 20, 1], 5) == [10, 5, 20], "phải GIỮ giá trị gốc của các phần tử >= muc_toi_thieu, đúng thứ tự"
assert loc_du_hang([1, 2, 3], 100) == [], "không phần tử nào thoả thì trả về danh sách rỗng"
assert loc_du_hang([5, 5, 5], 5) == [5, 5, 5], "mọi phần tử thoả thì giữ lại HẾT"
```

:::hints
- kind: attention
  body: Dùng filter() để giữ lại các số lượng >= muc_toi_thieu, rồi bọc list(...) — filter() một mình trả về iterator, không phải list.
- kind: strategy
  body: 'list(filter(lambda sl: sl >= muc_toi_thieu, ds_ton_kho)) — filter(...) giữ lại phần tử mà điều kiện trả True, list(...) ép kết quả thành danh sách thật.'
- kind: one-line
  body: "return list(filter(lambda sl: sl >= muc_toi_thieu, ds_ton_kho))"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: loc_du_hang phải dùng filter() (bài đang dạy đúng công cụ đó), không phải map() — map() không LOẠI BỎ phần tử nào, chỉ đổi giá trị.
  requireAst:
  - kind: uses-call, target: filter, min: 1
  - kind: uses-call, target: list, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "[10, 5, 20]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`map()` đổi giá trị, giữ nguyên số lượng. `filter()` giữ nguyên giá trị,
đổi số lượng. Hai công cụ, hai việc khác hẳn nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã có `map()`, `filter()`, và trước đó là list comprehension (Realm
1). Cả ba làm gì đó khá giống nhau — vậy khi nào dùng cái nào?

Bài sau so sánh trực tiếp.
::::

::::checkpoint{mastery=0.8}
::::
