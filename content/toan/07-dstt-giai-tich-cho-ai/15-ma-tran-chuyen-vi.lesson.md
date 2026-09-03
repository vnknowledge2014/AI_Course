---
id: toan.dstt-giai-tich-cho-ai.ma-tran-chuyen-vi
title: Ma trận chuyển vị
summary: "Aᵀ — đổi HÀNG thành cột (Aᵀ[i][j] = A[j][i]); 'xoay' bảng số MÀ không đổi giá trị — hữu ích khi cần đọc dữ liệu THEO chiều KHÁC."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.matrix-transpose]
requires: [math.matrix]
concepts: [math.ma-tran-chuyen-vi]
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
"Xoay" một bảng số — đổi HÀNG thành CỘT — MÀ không đổi giá trị bất
kỳ Ô nào. Có cách NÀO làm điều ĐÓ không?
::::

::::explain{#ma-tran-chuyen-vi}
CÓ — VÀ LUÔN áp dụng được cho MỌI ma trận (khác nghịch đảo, bài 14,
KHÔNG phải ma trận nào cũng có). **`Aᵀ`** — đổi HÀNG thành cột:
`Aᵀ[i][j] = A[j][i]`:

```python title=readonly
def chuyen_vi(A):
    if not A:
        return []
    so_hang, so_cot = len(A), len(A[0])
    return [tuple(A[i][j] for i in range(so_hang)) for j in range(so_cot)]


A = [(1.0, 2.0, 3.0), (4.0, 5.0, 6.0)]

print(chuyen_vi(A))
```

```text title=readonly
[(1.0, 4.0), (2.0, 5.0), (3.0, 6.0)]
```

`A` LÀ `2×3` (hai hàng, BA cột). `Aᵀ` LÀ `3×2` (BA hàng, hai cột) —
cột ĐẦU của `A` (`1,4`) trở thành HÀNG đầu của `Aᵀ`. KÍCH thước
"lật" NGƯỢC, giá TRỊ giữ nguyên.
::::

::::example{#chuyen-vi-hai-lan}
Chuyển vị HAI lần — TRỞ về ma trận GỐC:

```python title=readonly
def chuyen_vi(A):
    if not A:
        return []
    so_hang, so_cot = len(A), len(A[0])
    return [tuple(A[i][j] for i in range(so_hang)) for j in range(so_cot)]


A = [(1.0, 2.0, 3.0), (4.0, 5.0, 6.0)]

print(chuyen_vi(chuyen_vi(A)))
```

```text title=readonly
[(1.0, 2.0, 3.0), (4.0, 5.0, 6.0)]
```

`(Aᵀ)ᵀ = A` — "xoay" HAI lần TRẢ về đúng vị TRÍ ban đầu, đúng LỐI
`không hoi` KÉP (hai lần phủ định, T2.3) hay hai lần LẤY nghịch
đảo cộng (T2.6).
::::

::::predict{#doan-chuyen-vi-don-vi commitOnce}
Byte chuyển VỊ ma trận đơn vị `I3` (bài 14):

```python
def chuyen_vi(A):
    if not A:
        return []
    so_hang, so_cot = len(A), len(A[0])
    return [tuple(A[i][j] for i in range(so_hang)) for j in range(so_cot)]

I3 = [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)]
print(chuyen_vi(I3) == I3)
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì chuyển vị LUÔN "xoay" thành một ma trận KHÁC, VÀ hai
ma trận (dù nhìn GIỐNG nhau) không THỂ bằng nhau SAU khi qua một
phép biến ĐỔI
::why
Gần đúng ở việc bạn nghĩ "biến đổi" LUÔN cho RA kết quả khác — một
trực giác chung ĐÚNG cho NHIỀU phép toán.

Chỗ lệch: chuyển vị KHÔNG PHẢI lúc nào cũng đổi ma TRẬN — MỘT ma
trận CÓ `A[i][j] = A[j][i]` VỚI mọi `i,j` (GỌI LÀ **đối XỨNG**, y
hệt quan hệ đối xứng T2.4 bài 20) sẽ GIỮ nguyên khi chuyển vị. Ma
trận đơn vị LÀ ví dụ đơn giản NHẤT: đường chéo `1`, còn LẠI `0` —
đối xứng qua đường CHÉO, `I3ᵀ = I3`.
::
:::

:::opt
Máy báo lỗi khi chạy — `chuyen_vi(I3)` VÀ `I3` LÀ hai biểu THỨC
KHÁC nhau (một LÀ kết quả HÀM, một LÀ biến GỐC), Python KHÔNG cho
so SÁNH `==` giữa MỘT lời gọi hàm VÀ một biến TRỰC tiếp
::why
Gần đúng ở việc bạn để ý ĐÚNG hai VẾ CÓ nguồn gốc KHÁC nhau (một LÀ
kết quả TÍNH, một LÀ biến CÓ sẵn) — một quan sát VỀ hình THỨC.

Chỗ lệch: `==` so sánh GIÁ TRỊ, KHÔNG quan tâm giá trị ĐÓ đến TỪ
đâu (kết quả hàm HAY biến trực tiếp) — HOÀN toàn hợp lệ. Biên dịch
sạch, chạy sạch.
::
:::
::::

::::code{#viet_chuyen_vi}
Viết `chuyen_vi(A)` — đổi HÀNG thành cột của ma trận `A`.

```python title=starter
def chuyen_vi(A):
    if not A:
        return []
    so_hang, so_cot = len(A), len(A[0])
    return ___


A = [(1.0, 2.0, 3.0), (4.0, 5.0, 6.0)]
print(chuyen_vi(A))
```

```python title=solution
def chuyen_vi(A):
    if not A:
        return []
    so_hang, so_cot = len(A), len(A[0])
    return [tuple(A[i][j] for i in range(so_hang)) for j in range(so_cot)]


A = [(1.0, 2.0, 3.0), (4.0, 5.0, 6.0)]
print(chuyen_vi(A))
```

```python title=test
assert chuyen_vi([]) == [], "ma tran rong -- ket qua rong"
A = [(1.0, 2.0, 3.0), (4.0, 5.0, 6.0)]
assert chuyen_vi(A) == [(1.0, 4.0), (2.0, 5.0), (3.0, 6.0)], "2x3 thanh 3x2"
assert chuyen_vi(chuyen_vi(A)) == A, "chuyen vi hai lan -- ve goc"
I3 = [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)]
assert chuyen_vi(I3) == I3, "ma tran don vi doi xung -- khong doi"
```

:::hints
- kind: attention
  body: "O hang j, cot i cua ket qua chinh la A[i][j] -- lap qua i (giu j co dinh) cho tung j."
- kind: strategy
  body: "[tuple(A[i][j] for i in range(so_hang)) for j in range(so_cot)]"
- kind: one-line
  body: "___ = [tuple(A[i][j] for i in range(so_hang)) for j in range(so_cot)]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai doi cho i va j khi doc A -- A[i][j] lap qua i cho tung j
  requireAst:
  - kind: uses-name, target: so_hang, min: 1
  - kind: uses-name, target: so_cot, min: 1
  - kind: comprehension, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\(1\.0, 4\.0\), \(2\.0, 5\.0\), \(3\.0, 6\.0\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chuyển vị — đóng TRỌN cụm ma trận. Luống LỚN lên bao NHANH — đo TỐC
ĐỘ thay đổi thế NÀO?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vườn CỦA Byte đo được BAO NHIÊU (dài, nước, nắng) — nhưng CÁC con số
ĐÓ đo theo THỜI GIAN thì SAO? Luống LỚN lên bao NHANH — đo TỐC ĐỘ
thay đổi thế NÀO?
::::

::::checkpoint{mastery=0.8}
::::
