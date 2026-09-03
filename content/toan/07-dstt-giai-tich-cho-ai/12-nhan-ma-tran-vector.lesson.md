---
id: toan.dstt-giai-tich-cho-ai.nhan-ma-tran-vector
title: Nhân ma trận với vector
summary: "Av — MỖI thành phần của kết quả LÀ tích vô hướng (bài 6) của MỘT hàng ma trận VỚI v; đây LÀ biến đổi TUYẾN TÍNH — cách AI 'biến' một vector đầu VÀO thành một vector ĐẦU ra."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.matrix-vector-multiplication]
requires: [math.matrix-scalar-multiplication, math.dot-product]
concepts: [math.nhan-ma-tran-vector]
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
Ma trận NHÂN VỚI một VECTOR (không phải MỘT số) — kết quả LÀ GÌ, VÀ
tính THẾ nào?
::::

::::explain{#nhan-ma-tran-vector}
**`Av`** — MỖI thành phần của kết quả LÀ **tích vô hướng** (bài 6)
của MỘT hàng ma trận VỚI `v`. Đây LÀ **biến đổi TUYẾN TÍNH** — cách
AI "biến" một vector ĐẦU VÀO thành một vector ĐẦU ra:

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def nhan_ma_tran_vector(A, v):
    return tuple(tich_vo_huong(hang, v) for hang in A)


vuon = [(2.0, 5.0, 6.0), (3.0, 4.0, 5.0), (1.0, 3.0, 4.0)]
w = (1.0, 1.0, 1.0)

print(nhan_ma_tran_vector(vuon, w))
```

```text title=readonly
(13.0, 12.0, 8.0)
```

MỖI HÀNG của `vuon` (một hồ sơ luống) nhân TÍCH vô hướng VỚI `w =
(1,1,1)` — CHÍNH LÀ CỘNG DỒN cả BA chỉ số đo: luống `1` cho
`2+5+6=13`, luống `2` cho `3+4+5=12`, luống `3` cho `1+3+4=8`.
::::

::::example{#chon-cot}
Chọn vector TRỌNG số KHÁC — CHỈ giữ LẠI một chỉ số đo:

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def nhan_ma_tran_vector(A, v):
    return tuple(tich_vo_huong(hang, v) for hang in A)


vuon = [(2.0, 5.0, 6.0), (3.0, 4.0, 5.0), (1.0, 3.0, 4.0)]

print(nhan_ma_tran_vector(vuon, (1.0, 0.0, 0.0)))
```

```text title=readonly
(2.0, 3.0, 1.0)
```

Trọng số `(1,0,0)` "TẮT" hai chỉ số SAU (nhân VỚI `0`), CHỈ giữ lại
chiều DÀI của MỖI luống — kết quả LÀ cột ĐẦU tiên của `vuon`.
::::

::::predict{#doan-ma-tran-don-vi commitOnce}
Byte nhân ma trận `I = [(1.0,0.0), (0.0,1.0)]` VỚI vector `(7.0,
9.0)`:

```python
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def nhan_ma_tran_vector(A, v):
    return tuple(tich_vo_huong(hang, v) for hang in A)

I = [(1.0, 0.0), (0.0, 1.0)]
print(nhan_ma_tran_vector(I, (7.0, 9.0)))
```

Dòng cuối in ra gì?

:::opt{correct}
`(7.0, 9.0)`
:::

:::opt
`(0.0, 0.0)` — vì `I` chứa TOÀN số `0` VÀ `1` xen KẼ, VÀ nhân bất
kỳ vector NÀO với một ma trận chứa NHIỀU số `0` THƯỜNG cho ra kết
quả TOÀN `0`
::why
Gần đúng ở việc bạn để ý ĐÚNG `I` chứa NHIỀU số `0` — một quan sát
VỀ dữ liệu.

Chỗ lệch: VỊ trí của các số `0` VÀ `1` mới QUYẾT định, KHÔNG phải
"nhiều số 0 THÌ ra 0". Hàng ĐẦU `(1,0)` tích vô hướng VỚI `(7,9)`
LÀ `1×7+0×9=7`; hàng HAI `(0,1)` cho `0×7+1×9=9` — kết quả GIỮ
NGUYÊN vector gốc, KHÔNG đổi gì.
::
:::

:::opt
Máy báo lỗi khi chạy — `I` LÀ một ma trận `2×2`, VÀ vector `(7.0,
9.0)` CŨNG có `2` thành phần, nhưng Python KHÔNG cho nhân ma trận
VUÔNG với vector CÙNG kích thước
::why
Gần đúng ở việc bạn để ý ĐÚNG `I` VUÔNG (`2×2`) VÀ `v` cũng CÓ `2`
thành phần — một quan sát VỀ KÍCH thước chính xác.

Chỗ lệch: KÍCH thước KHỚP nhau (số cột của `I` bằng số thành phần
của `v`) chính LÀ điều kiện CẦN để phép nhân HỢP LỆ — KHÔNG phải lý
do bị TỪ chối. Biên dịch sạch, chạy sạch, kết quả HOÀN toàn đúng.
::
:::
::::

::::code{#viet_nhan_ma_tran_vector}
Viết `nhan_ma_tran_vector(A, v)` — nhân ma trận `A` VỚI vector `v`.

```python title=starter
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def nhan_ma_tran_vector(A, v):
    return ___


vuon = [(2.0, 5.0, 6.0), (3.0, 4.0, 5.0), (1.0, 3.0, 4.0)]
w = (1.0, 1.0, 1.0)
print(nhan_ma_tran_vector(vuon, w))
```

```python title=solution
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def nhan_ma_tran_vector(A, v):
    return tuple(tich_vo_huong(hang, v) for hang in A)


vuon = [(2.0, 5.0, 6.0), (3.0, 4.0, 5.0), (1.0, 3.0, 4.0)]
w = (1.0, 1.0, 1.0)
print(nhan_ma_tran_vector(vuon, w))
```

```python title=test
assert nhan_ma_tran_vector([], (1.0,)) == (), "ma tran rong -- ket qua rong"
vuon = [(2.0, 5.0, 6.0), (3.0, 4.0, 5.0), (1.0, 3.0, 4.0)]
assert nhan_ma_tran_vector(vuon, (1.0, 1.0, 1.0)) == (13.0, 12.0, 8.0), "cong don ba chi so"
assert nhan_ma_tran_vector(vuon, (1.0, 0.0, 0.0)) == (2.0, 3.0, 1.0), "chi giu chieu dai"
I = [(1.0, 0.0), (0.0, 1.0)]
assert nhan_ma_tran_vector(I, (7.0, 9.0)) == (7.0, 9.0), "ma tran don vi -- khong doi vector"
```

:::hints
- kind: attention
  body: "Voi MOI hang cua A, tinh tich vo huong cua hang do voi v."
- kind: strategy
  body: "tuple(tich_vo_huong(hang, v) for hang in A)"
- kind: one-line
  body: "___ = tuple(tich_vo_huong(hang, v) for hang in A)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tinh tich_vo_huong cua tung hang voi v
  requireAst:
  - kind: uses-call, target: tuple, min: 1
  - kind: uses-call, target: tich_vo_huong, min: 1
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(13\.0, 12\.0, 8\.0\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhân ma trận-vector — biến đổi TUYẾN tính. `AB` VÀ `BA` — có luôn
bằng nhau không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`AB` VÀ `BA` — CÓ luôn bằng nhau không (giống câu hỏi Ở T2.6 VỀ hợp
thành ánh xạ, bài 27)?
::::

::::checkpoint{mastery=0.8}
::::
