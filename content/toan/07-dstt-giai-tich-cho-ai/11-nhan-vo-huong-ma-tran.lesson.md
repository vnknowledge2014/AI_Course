---
id: toan.dstt-giai-tich-cho-ai.nhan-vo-huong-ma-tran
title: Nhân vô hướng ma trận
summary: "k·A — nhân MỖI phần tử VỚI k; đúng lối nhân vô hướng vector (bài 3)."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.matrix-scalar-multiplication]
requires: [math.matrix-addition]
concepts: [math.nhan-vo-huong-ma-tran]
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
Nhân CẢ ma trận VỚI một số `k` (tăng đồng LOẠT mọi chỉ số đo LÊN `k`
lần) — phép toán NÀY LÀ gì?
::::

::::explain{#nhan-vo-huong-ma-tran}
**`k·A`** — nhân MỖI phần tử VỚI `k`; đúng LỐI nhân vô hướng vector
(bài 3), CHỈ áp DỤNG lên TỪNG hàng:

```python title=readonly
def nhan_vo_huong_ma_tran(k, A):
    return [tuple(k * x for x in hang) for hang in A]


A = [(1.0, 2.0), (3.0, 4.0)]

print(nhan_vo_huong_ma_tran(2, A))
```

```text title=readonly
[(2.0, 4.0), (6.0, 8.0)]
```

MỖI phần tử NHÂN VỚI `2` — ĐÚNG lối nhân vô hướng vector, ÁP dụng
LÊN từng hàng RIÊNG.
::::

::::example{#nhan-voi-0-va-am}
Nhân VỚI `0` — RA ma trận TOÀN số `0`; nhân VỚI số ÂM — ĐỔI dấu MỌI
phần tử:

```python title=readonly
def nhan_vo_huong_ma_tran(k, A):
    return [tuple(k * x for x in hang) for hang in A]


A = [(1.0, 2.0), (3.0, 4.0)]

print(nhan_vo_huong_ma_tran(0, A))
print(nhan_vo_huong_ma_tran(-1, A))
```

```text title=readonly
[(0.0, 0.0), (0.0, 0.0)]
[(-1.0, -2.0), (-3.0, -4.0)]
```

Y HỆT nhân vô hướng vector (bài 3): `k=0` cho ma trận KHÔNG, `k=-1`
đổi dấu MỌI phần tử NGAY (KHÔNG cần nhân hai LẦN).
::::

::::predict{#doan-phan-phoi commitOnce}
Byte kiểm: `3·(A+B)` VÀ `3·A + 3·B` — hai cách TÍNH khác nhau, CÙNG
đích:

```python
def nhan_vo_huong_ma_tran(k, A):
    return [tuple(k * x for x in hang) for hang in A]

def cong_ma_tran(A, B):
    return [tuple(a + b for a, b in zip(hang_a, hang_b)) for hang_a, hang_b in zip(A, B)]

A = [(1.0, 2.0), (3.0, 4.0)]
B = [(5.0, 6.0), (7.0, 8.0)]
trai = nhan_vo_huong_ma_tran(3, cong_ma_tran(A, B))
phai = cong_ma_tran(nhan_vo_huong_ma_tran(3, A), nhan_vo_huong_ma_tran(3, B))
print(trai == phai)
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì `3·(A+B)` NHÂN sau khi CỘNG, còn `3·A + 3·B` NHÂN
TRƯỚC rồi CỘNG sau — hai THỨ tự thao TÁC khác nhau KHÔNG đảm bảo ra
cùng kết quả
::why
Gần đúng ở việc bạn để ý ĐÚNG hai vế THỰC hiện phép TOÁN theo THỨ
tự khác nhau — một quan sát VỀ cấu trúc BIỂU thức chính xác.

Chỗ lệch: nhân vô hướng ma trận PHÂN PHỐI qua phép cộng — `k·(A+B)
= k·A + k·B`, giống HỆT phân phối SỐ học thường (`k×(a+b)=k×a+k×b`,
T2.1). Đổi THỨ tự "cộng RỒI nhân" hay "nhân RỒI cộng" KHÔNG đổi kết
quả CUỐI, MIỄN áp dụng ĐÚNG quy tắc phân phối.
::
:::

:::opt
Máy báo lỗi khi chạy — `trai` VÀ `phai` LÀ hai `list` LỒNG `tuple`,
Python KHÔNG cho SO sánh HAI cấu trúc lồng bằng `==`
::why
Gần đúng ở việc bạn để ý ĐÚNG `trai`, `phai` LÀ cấu trúc LỒNG (list
của tuple) — một quan sát VỀ kiểu DỮ liệu.

Chỗ lệch: Python so SÁNH `==` trên cấu TRÚC lồng HOÀN TOÀN bình
thường — so SÁNH từng phần tử ĐỆ quy, TRẢ về `True`/`False` NHƯ
mong đợi. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_nhan_vo_huong_ma_tran}
Viết `nhan_vo_huong_ma_tran(k, A)` — nhân MỖI phần tử của `A` VỚI
`k`.

```python title=starter
def nhan_vo_huong_ma_tran(k, A):
    return ___


A = [(1.0, 2.0), (3.0, 4.0)]
print(nhan_vo_huong_ma_tran(2, A))
```

```python title=solution
def nhan_vo_huong_ma_tran(k, A):
    return [tuple(k * x for x in hang) for hang in A]


A = [(1.0, 2.0), (3.0, 4.0)]
print(nhan_vo_huong_ma_tran(2, A))
```

```python title=test
assert nhan_vo_huong_ma_tran(2, []) == [], "ma tran rong -- ket qua rong"
A = [(1.0, 2.0), (3.0, 4.0)]
assert nhan_vo_huong_ma_tran(2, A) == [(2.0, 4.0), (6.0, 8.0)], "nhan gap doi"
assert nhan_vo_huong_ma_tran(0, A) == [(0.0, 0.0), (0.0, 0.0)], "nhan voi 0"
assert nhan_vo_huong_ma_tran(-1, A) == [(-1.0, -2.0), (-3.0, -4.0)], "nhan voi -1"
```

:::hints
- kind: attention
  body: "Voi MOI hang trong A, nhan k voi TUNG phan tu cua hang do."
- kind: strategy
  body: "[tuple(k * x for x in hang) for hang in A]"
- kind: one-line
  body: "___ = [tuple(k * x for x in hang) for hang in A]"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan k voi tung phan tu cua tung hang trong A
  requireAst:
  - kind: uses-call, target: tuple, min: 1
  - kind: uses-operator, target: '*', min: 1
  - kind: comprehension, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\(2\.0, 4\.0\), \(6\.0, 8\.0\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhân vô hướng ma trận — phóng to CẢ bảng số. Ma trận NHÂN VỚI một
VECTOR (không phải MỘT số) — kết quả LÀ GÌ?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ma trận NHÂN VỚI một VECTOR (không phải MỘT số) — kết quả LÀ GÌ, VÀ
tính THẾ nào?
::::

::::checkpoint{mastery=0.8}
::::
