---
id: toan.dstt-giai-tich-cho-ai.ma-tran-la-gi
title: Ma trận là gì
summary: "Ma trận — một BẢNG số, list CÁC vector hàng; TOÀN bộ vườn (MỖI luống MỘT hàng, MỖI cột MỘT chỉ số đo) LÀ một ma trận DUY nhất."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.matrix]
requires: [math.cosine-similarity, math.vector]
concepts: [math.ma-tran]
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
Một VƯỜN CÓ nhiều luống — LÀM sao viết GỌN toàn bộ dữ liệu (nhiều
vector) thành MỘT cấu trúc DUY nhất?
::::

::::explain{#ma-tran-la-gi}
**Ma trận** — một BẢNG số: một `list` CÁC vector HÀNG (bài 1). TOÀN
bộ vườn (MỖI luống MỘT hàng, MỖI cột MỘT chỉ số ĐO — dài/nước/nắng)
LÀ một ma trận DUY nhất:

```python title=readonly
vuon = [
    (2.0, 5.0, 6.0),
    (3.0, 4.0, 5.0),
    (1.0, 3.0, 4.0),
]

print(len(vuon))
print(len(vuon[0]))
```

```text title=readonly
3
3
```

`vuon` LÀ một ma trận `3×3` — `len(vuon)` đếm SỐ hàng (BA luống),
`len(vuon[0])` đếm SỐ cột của hàng đầu (BA chỉ số đo). MỖI hàng
`vuon[i]` LÀ vector hồ sơ của MỘT luống.
::::

::::example{#truy-cap-o}
Truy CẬP một Ô CỤ THỂ — hàng TRƯỚC, cột SAU:

```python title=readonly
vuon = [
    (2.0, 5.0, 6.0),
    (3.0, 4.0, 5.0),
    (1.0, 3.0, 4.0),
]

print(vuon[1])
print(vuon[1][2])
```

```text title=readonly
(3.0, 4.0, 5.0)
5.0
```

`vuon[1]` LÀ hàng THỨ hai (luống `2`, chỉ số `1` VÌ đếm từ `0`) —
`(3.0, 4.0, 5.0)`. `vuon[1][2]` LÀ Ô Ở hàng `1`, cột `2` — thành
phần THỨ ba (nắng) của LUỐNG THỨ hai — `5.0`.
::::

::::predict{#doan-truy-cap-hang-cot commitOnce}
Byte truy CẬP `vuon[2][1]`:

```python
vuon = [
    (2.0, 5.0, 6.0),
    (3.0, 4.0, 5.0),
    (1.0, 3.0, 4.0),
]
print(vuon[2][1])
```

Dòng cuối in ra gì?

:::opt{correct}
`3.0`
:::

:::opt
`4.0` — vì chỉ số ĐẦU (`2`) LÀ CỘT, chỉ số SAU (`1`) LÀ HÀNG (đọc
NGƯỢC lại so VỚI cách hiểu "hàng TRƯỚC, cột SAU")
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng đọc chỉ số theo THỨ tự ngược
— MỘT quy ước hợp LÝ (một VÀI ngữ CẢNH toán học dùng thứ tự "cột,
hàng").

Chỗ lệch: TRONG cách biểu diễn `list`-lồng-`list` (HAY `list`-lồng-
`tuple`) NÀY, `vuon[i][j]` LUÔN LÀ "lấy PHẦN TỬ thứ `i` của
`vuon` (MỘT hàng), RỒI lấy PHẦN tử thứ `j` của hàng ĐÓ" — Ý nghĩa
HOÀN TOÀN do CÁCH `[]` áp DỤNG lần LƯỢT quyết định, KHÔNG có "cột
trước". `vuon[2]` LÀ hàng `(1.0, 3.0, 4.0)`, `vuon[2][1]` LÀ phần
tử THỨ hai CỦA hàng ĐÓ — `3.0`.
::
:::

:::opt
Máy báo lỗi khi chạy — `vuon[2][1]` LỒNG hai dấu ngoặc VUÔNG liên
tiếp, Python CHỈ cho phép MỘT cặp dấu ngoặc VUÔNG trên MỖI biến
::why
Gần đúng ở việc bạn để ý ĐÚNG `[2][1]` CÓ hai cặp dấu ngoặc VUÔNG
liên tiếp — một quan sát VỀ cú pháp.

Chỗ lệch: Python CHO PHÉP lồng chỉ số THOẢI mái — `vuon[2]` trả VỀ
MỘT `tuple`, RỒI `[1]` LẤY tiếp phần tử CỦA `tuple` ĐÓ. Biên dịch
sạch, chạy sạch.
::
:::
::::

::::code{#viet_kich_thuoc}
Viết `kich_thuoc(m)` — trả VỀ `(so_hang, so_cot)` của ma trận `m`.

```python title=starter
def kich_thuoc(m):
    return ___


vuon = [
    (2.0, 5.0, 6.0),
    (3.0, 4.0, 5.0),
    (1.0, 3.0, 4.0),
]
print(kich_thuoc(vuon))
```

```python title=solution
def kich_thuoc(m):
    return (len(m), len(m[0]) if m else 0)


vuon = [
    (2.0, 5.0, 6.0),
    (3.0, 4.0, 5.0),
    (1.0, 3.0, 4.0),
]
print(kich_thuoc(vuon))
```

```python title=test
assert kich_thuoc([]) == (0, 0), "ma tran rong -- 0 hang, 0 cot"
vuon = [
    (2.0, 5.0, 6.0),
    (3.0, 4.0, 5.0),
    (1.0, 3.0, 4.0),
]
assert kich_thuoc(vuon) == (3, 3), "ba hang ba cot"
assert kich_thuoc([(1.0,)]) == (1, 1), "mot hang mot cot"
assert kich_thuoc([(1.0, 2.0), (3.0, 4.0)]) == (2, 2), "hai hang hai cot"
```

:::hints
- kind: attention
  body: "So hang la len(m), so cot la len cua hang dau tien (m[0]) neu m khong rong."
- kind: strategy
  body: "(len(m), len(m[0]) if m else 0)"
- kind: one-line
  body: "___ = (len(m), len(m[0]) if m else 0)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tra ve (len(m), len(m[0]) if m else 0)
  requireAst:
  - kind: uses-call, target: len, min: 2
  - kind: uses-name, target: m, min: 3
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(3, 3\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ma trận — một bảng số. Cộng HAI ma trận (hai lần ĐO của cùng khu
vườn) — cộng NHƯ thế nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cộng HAI ma trận (hai lần ĐO của cùng khu vườn) — cộng NHƯ thế nào,
đúng lối cộng vector (bài 2) không?
::::

::::checkpoint{mastery=0.8}
::::
