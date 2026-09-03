---
id: co-so-du-lieu.cay-cua-nguoi-thu-thu.tim-trong-mot-la
title: Tìm trong một lá
summary: "tim_trong_la quét qua khoá TRONG một node lá (danh sách NHỎ, đã sắp xếp) để tìm giá trị khớp — khoá không tồn tại trả về None, không lỗi gì. Đây LÀ quét tuyến tính (bài 1) nhưng thu NHỎ phạm vi lại còn ĐÚNG một node, không phải toàn bộ log."
locale: vi
track: co-so-du-lieu
module: cay-cua-nguoi-thu-thu
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.search-in-leaf]
requires: [db.node-format]
concepts: [db.search-in-leaf]
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
Một node LÁ chứa `3` khoá CÙNG lúc. LÀM sao tìm ĐÚNG giá trị của
MỘT khoá cụ THỂ trong SỐ đó?
::::

::::explain{#quet-trong-la}
`tim_trong_la` quét QUA `khoa` (danh sách nhỏ, ĐÃ sắp xếp), khớp
đúng CHỈ số thì lấy `gia_tri` Ở CHÍNH chỉ số đó:

```python title=readonly
def tim_trong_la(la, khoa_can_tim):
    for i, k in enumerate(la['khoa']):
        if k == khoa_can_tim:
            return la['gia_tri'][i]
    return None


la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
print(tim_trong_la(la, 7))
```

```text title=readonly
bay
```

`enumerate(la['khoa'])` cho CẢ chỉ số `i` LẪN giá trị `k` CÙNG lúc
— khớp `k == 7` Ở chỉ số `1`, LẤY `la['gia_tri'][1]` = `'bay'`. Đây
LÀ quét tuyến tính (bài 1) — NHƯNG thu NHỎ phạm vi lại còn ĐÚNG một
node, KHÔNG phải toàn bộ log.
::::

::::example{#khoa-dau-va-cuoi}
Tìm khoá ĐẦU (`3`) VÀ khoá CUỐI (`9`) — CẢ hai đều tìm ĐƯỢC, dù VỊ
trí khác nhau:

```python title=readonly
print(tim_trong_la(la, 3))
print(tim_trong_la(la, 9))
```

```text title=readonly
ba
chin
```

VÌ `la['khoa']` CHỈ có `3` phần TỬ (KHÔNG phải `10000` như bài 1),
quét HẾT cả node LUÔN rất RẺ, dù khoá NẰM Ở đâu.
::::

::::predict{#doan-khoa-khong-ton-tai commitOnce}
Byte tìm khoá `5` (KHÔNG hề CÓ trong `khoa=[3, 7, 9]`):

```python
print(tim_trong_la(la, 5))
```

Dòng cuối in ra gì?

:::opt{correct}
`None`
:::

:::opt
`chin` — vì hàm trả VỀ giá trị GẦN nhất khi khoá CHÍNH xác không có
::why
Gần đúng ở việc bạn nghĩ TỚI một cơ chế "TÌM gần đúng" hợp lý cho
một SỐ hệ thống tra cứu KHÁC (vd tìm kiếm mờ).

Chỗ lệch: `tim_trong_la` CHỈ so sánh CHÍNH xác bằng `==` — `5`
KHÁC hoàn toàn `3`, `7`, VÀ `9`, KHÔNG có khái niệm "gần NHẤT" NÀO
cả. Vòng lặp chạy HẾT mà KHÔNG khớp lần NÀO, rơi VỀ `return None`.
::
:::

:::opt
Máy báo lỗi — vì `5` KHÔNG tồn tại TRONG `khoa`
::why
Gần đúng ở việc bạn LO ngại đúng hướng — khoá KHÔNG tồn tại LÀ một
tình huống ĐÁNG lưu ý.

Chỗ lệch: `tim_trong_la` xử LÝ trường hợp NÀY hoàn toàn BÌNH
thường — vòng LẶP kết thúc KHÔNG khớp GÌ, hàm chỉ ĐƠN giản trả về
`None`, KHÔNG có `raise` NÀO cả.
::
:::
::::

::::code{#viet_tim_trong_la}
Hoàn thiện `tim_trong_la(la, khoa_can_tim)` — trả VỀ giá trị khớp,
`None` nếu KHÔNG tìm thấy.

```python title=starter
def tim_trong_la(la, khoa_can_tim):
    for i, k in enumerate(la['khoa']):
        ___
    return None


la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
print(tim_trong_la(la, 7))
```

```python title=solution
def tim_trong_la(la, khoa_can_tim):
    for i, k in enumerate(la['khoa']):
        if k == khoa_can_tim:
            return la['gia_tri'][i]
    return None


la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
print(tim_trong_la(la, 7))
```

```python title=test
la = {'loai': 'la', 'khoa': [3, 7, 9], 'gia_tri': ['ba', 'bay', 'chin'], 'la_tiep': None}
assert tim_trong_la(la, 3) == 'ba', "khoa dau"
assert tim_trong_la(la, 7) == 'bay', "khoa giua"
assert tim_trong_la(la, 9) == 'chin', "khoa cuoi"
assert tim_trong_la(la, 5) is None, "khoa khong ton tai"

la_rong = {'loai': 'la', 'khoa': [], 'gia_tri': [], 'la_tiep': None}
assert tim_trong_la(la_rong, 1) is None, "la rong"
```

:::hints
- kind: attention
  body: "Neu k == khoa_can_tim, return la['gia_tri'][i] -- mot dong."
- kind: strategy
  body: "if k == khoa_can_tim: return la['gia_tri'][i]"
- kind: one-line
  body: "if k == khoa_can_tim: return la['gia_tri'][i]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai return la['gia_tri'][i] khi k == khoa_can_tim
  requireAst:
  - kind: uses-name, target: k, min: 1
  - kind: uses-name, target: i, min: 1
  - kind: uses-name, target: la, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^bay\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tìm ĐƯỢC trong một lá — quét NHỎ, rẻ. NHƯNG một lá CHỈ chứa được
CÓ hạn — muốn THÊM một khoá MỚI vào, LÀM sao?
::::

::::reflect{#nghi-lai}
`tim_trong_la` quét QUA danh sách khoá NHỎ TRONG một node — chính
LÀ quét tuyến tính (bài 1), NHƯNG thu nhỏ phạm VI lại còn đúng MỘT
node. Node HIỆN có `3` khoá — MUỐN thêm một khoá MỚI vào (VD `5`),
CHÈN nó Ở đâu để `khoa` VẪN giữ ĐÚNG thứ tự đã sắp XẾP?
::::

::::checkpoint{mastery=0.8}
::::
