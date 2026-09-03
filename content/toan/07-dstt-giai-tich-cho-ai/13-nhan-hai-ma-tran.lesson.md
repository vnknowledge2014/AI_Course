---
id: toan.dstt-giai-tich-cho-ai.nhan-hai-ma-tran
title: Nhân hai ma trận
summary: "AB — MỖI phần tử của kết quả LÀ tích vô hướng của MỘT hàng A VỚI MỘT cột B; nhân ma trận-vector (bài 12) LÀ trường hợp RIÊNG khi B chỉ CÓ một cột."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.matrix-multiplication]
requires: [math.matrix-vector-multiplication]
concepts: [math.nhan-hai-ma-tran]
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
`AB` VÀ `BA` — CÓ luôn bằng nhau không (giống câu hỏi Ở T2.6 VỀ hợp
thành ánh xạ, bài 27)?
::::

::::explain{#nhan-hai-ma-tran}
KHÔNG luôn bằng nhau — sẽ THẤY ngay. **`AB`** — MỖI phần tử của kết
quả LÀ **tích vô hướng** của MỘT hàng `A` VỚI MỘT cột `B`. Nhân ma
trận-vector (bài 12) LÀ trường HỢP RIÊNG khi `B` chỉ CÓ một cột:

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def cot(B, j):
    return tuple(hang[j] for hang in B)

def nhan_hai_ma_tran(A, B):
    so_cot_B = len(B[0]) if B else 0
    return [tuple(tich_vo_huong(hang_A, cot(B, j)) for j in range(so_cot_B)) for hang_A in A]


A = [(1.0, 2.0), (3.0, 4.0)]
B = [(5.0, 6.0), (7.0, 8.0)]

print(nhan_hai_ma_tran(A, B))
```

```text title=readonly
[(19.0, 22.0), (43.0, 50.0)]
```

`cot(B, 0)` LÀ `(5,7)` (cột ĐẦU của `B`) — hàng ĐẦU của `A` (`(1,2)`)
tích vô hướng VỚI NÓ: `1×5+2×7=19`. MỖI Ô CỦA kết quả LÀ MỘT tích
vô hướng "hàng-cột".
::::

::::example{#khong-giao-hoan}
ĐỔI thứ tự — `BA` RA kết quả KHÁC HẲN `AB`:

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def cot(B, j):
    return tuple(hang[j] for hang in B)

def nhan_hai_ma_tran(A, B):
    so_cot_B = len(B[0]) if B else 0
    return [tuple(tich_vo_huong(hang_A, cot(B, j)) for j in range(so_cot_B)) for hang_A in A]


A = [(1.0, 2.0), (3.0, 4.0)]
B = [(5.0, 6.0), (7.0, 8.0)]

print(nhan_hai_ma_tran(B, A))
```

```text title=readonly
[(23.0, 34.0), (31.0, 46.0)]
```

`BA = [(23,34),(31,46)]`, KHÁC HẲN `AB = [(19,22),(43,50)]` Ở TRÊN
— nhân ma trận **KHÔNG giao hoán**, đúng dự ĐOÁN Ở đầu bài.
::::

::::predict{#doan-tinh-o-cu-the commitOnce}
Byte tính TRỰC TIẾP `AB[0][1]` — hàng `0` của `A` LÀ `(1.0, 2.0)`,
cột `1` của `B` LÀ `(6.0, 8.0)`:

```python
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

print(tich_vo_huong((1.0, 2.0), (6.0, 8.0)))
```

Dòng cuối in ra gì?

:::opt{correct}
`22.0`
:::

:::opt
`14.0` — vì `AB[0][1]` NẰM Ở hàng `0`, cột `1` — CỘNG thẳng CÁC chỉ
số VỊ trí (`0+1=1`)... KHÔNG, cộng CÁC giá TRỊ Ở đúng VỊ trí đó:
`1+6=7`, `2+8=10`, TỔNG `17`... ĐÁP số GẦN đúng nhất LÀ `14.0`
::why
Gần đúng ở việc bạn thử NHIỀU cách kết HỢP số khác NHAU — một nỗ
lực TÌM quy luật hợp lý.

Chỗ lệch: `tich_vo_huong` LUÔN LÀ **nhân TỪNG cặp RỒI cộng**, KHÔNG
phải cộng TRỰC tiếp hay bất KỲ tổ hợp nào KHÁC: `1×6 + 2×8 = 6+16 =
22`. KHÔNG có phép TÍNH nào khác — LUÔN nhân từng CẶP trước.
::
:::

:::opt
Máy báo lỗi khi chạy — hai `tuple` `(1.0, 2.0)` VÀ `(6.0, 8.0)`
KHÔNG được PHÉP truyền TRỰC tiếp vào `tich_vo_huong` mà KHÔNG bọc
qua `nhan_hai_ma_tran` TRƯỚC
::why
Gần đúng ở việc bạn để ý ĐÚNG hai `tuple` NÀY thường xuất HIỆN bên
TRONG `nhan_hai_ma_tran` — một quan sát VỀ NGỮ cảnh sử dụng thông
thường.

Chỗ lệch: `tich_vo_huong` LÀ một hàm ĐỘC lập, GỌI được VỚI bất kỳ
cặp vector NÀO, KHÔNG cần đi qua `nhan_hai_ma_tran`. Biên dịch sạch,
chạy sạch.
::
:::
::::

::::code{#viet_nhan_hai_ma_tran}
Viết `nhan_hai_ma_tran(A, B)` — nhân HAI ma trận `A`, `B`.

```python title=starter
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def cot(B, j):
    return tuple(hang[j] for hang in B)

def nhan_hai_ma_tran(A, B):
    so_cot_B = len(B[0]) if B else 0
    return ___


A = [(1.0, 2.0), (3.0, 4.0)]
B = [(5.0, 6.0), (7.0, 8.0)]
print(nhan_hai_ma_tran(A, B))
```

```python title=solution
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def cot(B, j):
    return tuple(hang[j] for hang in B)

def nhan_hai_ma_tran(A, B):
    so_cot_B = len(B[0]) if B else 0
    return [tuple(tich_vo_huong(hang_A, cot(B, j)) for j in range(so_cot_B)) for hang_A in A]


A = [(1.0, 2.0), (3.0, 4.0)]
B = [(5.0, 6.0), (7.0, 8.0)]
print(nhan_hai_ma_tran(A, B))
```

```python title=test
assert nhan_hai_ma_tran([], []) == [], "hai ma tran rong -- ket qua rong"
A = [(1.0, 2.0), (3.0, 4.0)]
B = [(5.0, 6.0), (7.0, 8.0)]
assert nhan_hai_ma_tran(A, B) == [(19.0, 22.0), (43.0, 50.0)], "AB"
assert nhan_hai_ma_tran(B, A) == [(23.0, 34.0), (31.0, 46.0)], "BA -- khac AB"
I = [(1.0, 0.0), (0.0, 1.0)]
assert nhan_hai_ma_tran(A, I) == [(1.0, 2.0), (3.0, 4.0)], "nhan voi ma tran don vi -- khong doi"
```

:::hints
- kind: attention
  body: "Voi MOI hang cua A va MOI cot j cua B, tinh tich vo huong cua hang do voi cot(B, j)."
- kind: strategy
  body: "[tuple(tich_vo_huong(hang_A, cot(B, j)) for j in range(so_cot_B)) for hang_A in A]"
- kind: one-line
  body: "___ = [tuple(tich_vo_huong(hang_A, cot(B, j)) for j in range(so_cot_B)) for hang_A in A]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tinh tich_vo_huong cua tung hang A voi tung cot cua B
  requireAst:
  - kind: uses-call, target: tich_vo_huong, min: 1
  - kind: uses-call, target: cot, min: 1
  - kind: uses-call, target: range, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\(19\.0, 22\.0\), \(43\.0, 50\.0\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhân hai ma trận — KHÔNG giao hoán. Ma trận `A` CÓ "nghịch đảo"
`A⁻¹` sao `AA⁻¹=I` không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ma trận `A` CÓ "nghịch đảo" `A⁻¹` sao `AA⁻¹=I` không — LUÔN có, hay
CHỈ vài ma trận MỚI có (đúng LỐI T2.6 bài 24)?
::::

::::checkpoint{mastery=0.8}
::::
