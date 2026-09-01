---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.map-filter-doi-chieu-comprehension
title: "`map`/`filter` so với list comprehension — cùng Ý, khác cú pháp"
summary: "[x*2 for x in xs] và list(map(lambda x: x*2, xs)) cho CÙNG kết quả. Python thường ưu tiên comprehension — track vẫn dạy map/filter vì reduce và compose/pipe cần hàm ở DẠNG GIÁ TRỊ."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [fp.map-filter-vs-comprehension]
requires: [fp.filter-basics]
concepts: [fp.map-filter-vs-comprehension]
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
Bạn đã biết list comprehension từ Realm 1. Hai bài vừa qua học thêm
`map`/`filter`. Chúng có phải BA cách viết cho CÙNG một việc không?
::::

::::explain{#hai-cach-viet-mot-y}
`map`/`filter` và list comprehension làm CÙNG Ý, khác cú pháp:

```python
xs = [1, 2, 3, 4]

# map
cach_map = list(map(lambda x: x * 2, xs))

# comprehension
cach_comp = [x * 2 for x in xs]

print(cach_map == cach_comp)
```

```text
True
```

```python
# filter
loc_map = list(filter(lambda x: x % 2 == 0, xs))

# comprehension có điều kiện
loc_comp = [x for x in xs if x % 2 == 0]

print(loc_map == loc_comp)
```

```text
True
```

Python THƯỜNG ưu tiên comprehension — nó đọc gần với tiếng Việt/tiếng
Anh hơn ("lấy `x` nhân đôi, cho mỗi `x` trong `xs`") so với
`map(lambda x: x*2, xs)` (đọc từ trong ra ngoài, ngược thứ tự thao tác).
Nhiều hướng dẫn Python khuyên "dùng comprehension thay vì map/filter khi
có thể" — lời khuyên đó ĐÚNG cho việc VIẾT MỘT DÒNG độc lập.

Track này vẫn dạy `map`/`filter` vì hai lý do cụ thể, không phải sở
thích:

1. **`reduce()` (bài sau) không có dạng comprehension tương đương** —
   "gộp một dãy thành một giá trị" không viết được bằng `[... for ...]`.
2. **Ghép hàm (`compose`/`pipe`, cụm sau) cần hàm ở DẠNG GIÁ TRỊ, truyền
   được** — `map`/`filter` LÀ những hàm, gán được vào biến, truyền được
   vào hàm khác. Một list comprehension KHÔNG PHẢI một giá trị truyền
   được theo cách đó — nó là một BIỂU THỨC, tính ra kết quả ngay tại
   chỗ viết, không tách rời được "phép biến đổi" khỏi "danh sách cụ
   thể" như một hàm độc lập.
::::

::::predict{#doan-hai-cach-cung-ket-qua commitOnce}
```python
diem = [65, 92, 48, 77, 55]

qua_1 = [d for d in diem if d >= 60]
qua_2 = list(filter(lambda d: d >= 60, diem))

print(qua_1 == qua_2)
print(qua_1)
```

Hai dòng in ra gì?

:::opt{correct}
`True` rồi `[65, 92, 77]`
:::

:::opt
`False` rồi `[65, 92, 77]` — vì `filter()` trả về một `filter object`,
không thể nào bằng một `list` (bẫy đã học ở bài 6-7)
::why
Gần đúng ở việc bạn nhớ ĐÚNG bẫy `filter object` khác `list` — bẫy đó có
thật, và bạn nhớ đúng.

Chỗ lệch: `qua_2` KHÔNG còn là `filter object` nữa — nó đã bị
`list(...)` ép ra thành một `list` THẬT ở chính dòng định nghĩa. Bẫy chỉ
xảy ra nếu QUÊN `list(...)`; ở đây có rồi, nên `qua_1 == qua_2` so sánh
hai `list`, và chúng bằng nhau.
::
:::

:::opt
`True` rồi `[65, 92, 48, 77, 55]` — cả hai cách đều lấy TOÀN BỘ danh
sách gốc
::why
Gần đúng ở việc bạn tin `qua_1 == qua_2` đúng là `True` — dòng đó đúng.

Chỗ lệch: `qua_1` không phải TOÀN BỘ `diem` — cả `[d for d in diem if d
>= 60]` lẫn `filter(lambda d: d >= 60, diem)` đều LỌC, chỉ giữ điểm
`>= 60` (`65`, `92`, `77` — ba điểm, không phải cả năm điểm gốc).
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cách viết, một Ý — không có cách nào "đúng hơn" cách nào. Track này
dùng `map`/`filter` vì lý do CỤ THỂ sắp tới, không phải vì nó "chuẩn FP
hơn".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`map`/`filter` biến đổi/lọc từng phần tử, giữ nguyên SỐ LƯỢNG (map) hay
GIẢM số lượng (filter) — nhưng luôn cho ra một `list`/iterator. Nếu bạn
muốn gộp HẲN một danh sách thành MỘT con số duy nhất — như cộng dồn tất
cả lại — thì dùng công cụ nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
