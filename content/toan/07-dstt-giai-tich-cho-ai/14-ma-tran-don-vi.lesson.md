---
id: toan.dstt-giai-tich-cho-ai.ma-tran-don-vi
title: Ma trận đơn vị
summary: "Ma trận đơn vị I — đường CHÉO toàn 1, còn lại 0; AI = IA = A VỚI MỌI A — đúng LỐI phần tử đơn vị (T2.6 bài 23), GIỜ trên ma trận."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.identity-matrix]
requires: [math.matrix-multiplication, math.identity-element]
concepts: [math.ma-tran-don-vi]
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
Ma trận `A` CÓ "nghịch đảo" `A⁻¹` sao `AA⁻¹=I` không?
::::

::::explain{#ma-tran-don-vi}
CÓ MỘT `I` như VẬY, NHƯNG trước hết cần biết `I` LÀ gì. **Ma trận
đơn vị `I`** — đường CHÉO (`I[i][i]`) TOÀN `1`, còn LẠI `0`; `AI =
IA = A` VỚI MỌI `A` — đúng LỐI phần tử đơn vị (T2.6 bài 23), GIỜ
trên ma TRẬN:

```python title=readonly
def tao_don_vi(n):
    return [tuple(1.0 if i == j else 0.0 for j in range(n)) for i in range(n)]


print(tao_don_vi(3))
```

```text title=readonly
[(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)]
```

MỖI hàng `i` CÓ đúng MỘT số `1` (Ở cột `i`, đường chéo), CÒN lại
TOÀN `0`. Ma trận `3×3` này LÀ đơn vị CỠ `3`.
::::

::::example{#nhan-voi-don-vi-khong-doi}
Nhân MỘT ma trận VỚI đơn vị — kết quả KHÔNG đổi:

```python title=readonly
def tao_don_vi(n):
    return [tuple(1.0 if i == j else 0.0 for j in range(n)) for i in range(n)]

def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def cot(B, j):
    return tuple(hang[j] for hang in B)

def nhan_hai_ma_tran(A, B):
    so_cot_B = len(B[0]) if B else 0
    return [tuple(tich_vo_huong(hang_A, cot(B, j)) for j in range(so_cot_B)) for hang_A in A]


A = [(1.0, 2.0), (3.0, 4.0)]
I = tao_don_vi(2)

print(nhan_hai_ma_tran(A, I) == A)
```

```text title=readonly
True
```

`A × I = A` — ĐÚNG y hệt phần tử đơn vị của phép cộng SỐ (`0`) hay
nhân SỐ (`1`, T2.6 bài 23): ghép VỚI đơn vị KHÔNG đổi gì.
::::

::::predict{#doan-o-ngoai-cheo commitOnce}
Byte tạo ma trận đơn vị CỠ `3`, RỒI truy CẬP `I[0][2]` — hàng `0`,
cột `2` (NGOÀI đường chéo):

```python
def tao_don_vi(n):
    return [tuple(1.0 if i == j else 0.0 for j in range(n)) for i in range(n)]

I = tao_don_vi(3)
print(I[0][2])
```

Dòng cuối in ra gì?

:::opt{correct}
`0.0`
:::

:::opt
`1.0` — vì HÀNG `0` LÀ hàng ĐẦU tiên, VÀ hàng đầu CỦA ma trận đơn vị
LUÔN toàn SỐ `1`
::why
Gần đúng ở việc bạn nghĩ "hàng ĐẦU" CÓ vị trí ĐẶC biệt — một trực
giác dễ hiểu NHẦM.

Chỗ lệch: KHÔNG hàng nào (kể CẢ hàng đầu) TOÀN số `1` — CHỈ đúng
MỘT vị trí TRÊN mỗi hàng LÀ `1` (đường chéo, `I[i][i]`), CÒN lại
đều `0`. `I[0][2]` CÓ `i=0, j=2`, `i≠j` — NẰM ngoài đường chéo, kết
quả LÀ `0.0`.
::
:::

:::opt
Máy báo lỗi khi chạy — `I[0][2]` cố truy CẬP cột `2` của hàng `0`,
NHƯNG hàng ĐẦU của ma trận đơn vị chỉ CÓ đúng MỘT phần tử "thật"
(số `1`), các Ô khác KHÔNG tồn tại
::why
Gần đúng ở việc bạn nghĩ CÁC Ô số `0` "không THẬT SỰ tồn tại" — một
cách hiểu SAI lệch VỀ ma trận thưa (sparse), một khái NIỆM khác.

Chỗ lệch: MỌI Ô của ma trận (kể CẢ những Ô `0`) ĐỀU LÀ phần tử THẬT
SỰ trong `tuple` — hàng `0` LÀ `(1.0, 0.0, 0.0)`, CÓ đủ BA phần tử,
`I[0][2]` truy CẬP hợp lệ, trả VỀ `0.0`. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_tao_don_vi}
Viết `tao_don_vi(n)` — tạo ma trận đơn vị CỠ `n×n`.

```python title=starter
def tao_don_vi(n):
    return ___


print(tao_don_vi(3))
```

```python title=solution
def tao_don_vi(n):
    return [tuple(1.0 if i == j else 0.0 for j in range(n)) for i in range(n)]


print(tao_don_vi(3))
```

```python title=test
assert tao_don_vi(0) == [], "co 0 -- ma tran rong"
assert tao_don_vi(1) == [(1.0,)], "co 1"
assert tao_don_vi(2) == [(1.0, 0.0), (0.0, 1.0)], "co 2"
I3 = tao_don_vi(3)
assert I3 == [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)], "co 3"
assert I3[0][2] == 0.0, "ngoai duong cheo -- 0"
assert I3[1][1] == 1.0, "tren duong cheo -- 1"
```

:::hints
- kind: attention
  body: "O hang i, cot j: 1.0 neu i==j (duong cheo), nguoc lai 0.0."
- kind: strategy
  body: "[tuple(1.0 if i == j else 0.0 for j in range(n)) for i in range(n)]"
- kind: one-line
  body: "___ = [tuple(1.0 if i == j else 0.0 for j in range(n)) for i in range(n)]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dat 1.0 khi i==j, 0.0 khi khac, cho tung o
  requireAst:
  - kind: uses-operator, target: '==', min: 1
  - kind: has-literal, target: '1.0', min: 1
  - kind: has-literal, target: '0.0', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\(1\.0, 0\.0, 0\.0\), \(0\.0, 1\.0, 0\.0\), \(0\.0, 0\.0, 1\.0\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ma trận đơn vị — đường chéo toàn `1`. KHÔNG PHẢI ma trận nào CŨNG có
nghịch đảo (câu hỏi track NÀY để NGỎ). NHƯNG có một phép BIẾN đổi
ĐƠN giản hơn LUÔN áp dụng được CHO mọi ma trận — "xoay" hàng THÀNH
cột.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Xoay" một bảng số — đổi HÀNG thành CỘT — MÀ không đổi giá trị bất
kỳ Ô nào. Có cách NÀO làm điều ĐÓ không, VÀ nó CÓ LUÔN áp dụng được
cho MỌI ma trận không (KHÁC hẳn nghịch đảo, vốn KHÔNG phải ma trận
nào cũng có)?
::::

::::checkpoint{mastery=0.8}
::::
