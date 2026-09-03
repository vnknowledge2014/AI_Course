---
id: toan.to-hop-xac-suat-thong-ke.cong-thuc-bayes
title: Công thức Bayes
summary: "P(A|B) = P(B|A)·P(A) / P(B) — đảo chiều điều kiện, suy P(A|B) từ P(B|A) đã biết. Rút RA từ chính công thức bài 18 viết theo hai chiều, không phải một công thức mới độc lập."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.bayes-theorem]
requires: [math.independence-via-conditional]
concepts: [math.cong-thuc-bayes]
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
Byte biết `P(nảy mầm|giống mới)`. Một hạt ĐÃ nảy mầm — xác suất NGƯỢC
lại, nó LÀ giống mới, tính bằng cách nào?
::::

::::explain{#cong-thuc-bayes-la-gi}
**`P(A|B) = P(B|A) · P(A) / P(B)`** — đảo chiều điều kiện. Rút RA từ
CHÍNH công thức bài 18 viết theo HAI chiều: `P(A∩B) = P(A|B)P(B) =
P(B|A)P(A)`, chia hai vế SAU cho `P(B)` là ra công thức này — KHÔNG
PHẢI một công thức MỚI độc lập:

```python title=readonly
def bayes(p_b_cho_a, p_a, p_b):
    return p_b_cho_a * p_a / p_b


p_nay_moi = 0.2
p_moi = 0.6
p_nay = 0.16

print(bayes(p_nay_moi, p_moi, p_nay))
```

```text title=readonly
0.75
```

`P(\text{nảy mầm}|\text{giống mới})=0.2`, `P(\text{giống mới})=0.6`,
VÀ `P(\text{nảy mầm})=0.16` (đã tính GỘP từ cả giống mới lẫn giống
cũ). Bayes ĐẢO chiều: biết TRƯỚC hạt ĐÃ nảy mầm, xác suất nó LÀ
giống mới TĂNG từ `0.6` (trước khi biết) lên `0.75` (sau khi biết).
::::

::::example{#p-nay-gop-tu-dau}
`P(\text{nảy mầm})` (mẫu số Ở TRÊN) GỘP từ CẢ HAI loại giống — quy
tắc cộng (bài 1) trên hai nhánh RỜI NHAU (giống mới HOẶC giống cũ,
không thể cả hai):

```python title=readonly
p_nay_moi = 0.2
p_nay_cu = 0.1
p_moi = 0.6
p_cu = 0.4

p_nay = p_nay_moi * p_moi + p_nay_cu * p_cu
print(p_nay)
```

```text title=readonly
0.16
```

`0.2×0.6=0.12` (đường "giống mới VÀ nảy") CỘNG `0.1×0.4=0.04`
(đường "giống cũ VÀ nảy") — hai đường RỜI NHAU, cộng thẳng được (bài
1) — ra `0.16`, đúng mẫu số Bayes cần.
::::

::::predict{#doan-tong-hai-hau-nghiem commitOnce}
Byte tính XÁC SUẤT SAU (posterior) của CẢ HAI khả năng — "LÀ giống
mới" VÀ "LÀ giống cũ" — rồi cộng lại:

```python
def bayes(p_b_cho_a, p_a, p_b):
    return p_b_cho_a * p_a / p_b

p_nay_moi = 0.2
p_nay_cu = 0.1
p_moi = 0.6
p_cu = 0.4
p_nay = p_nay_moi * p_moi + p_nay_cu * p_cu

hau_nghiem_moi = bayes(p_nay_moi, p_moi, p_nay)
hau_nghiem_cu = 1 - hau_nghiem_moi

print(hau_nghiem_moi + hau_nghiem_cu)
```

Dòng cuối in ra gì?

:::opt{correct}
`1.0`
:::

:::opt
`0.16` — bằng ĐÚNG `p_nay`, vì cả hai xác suất SAU đều được TÍNH từ
CÙNG mẫu số `p_nay`, nên cộng lại phải RA LẠI chính mẫu số đó
::why
Gần đúng ở việc bạn để ý `p_nay` xuất hiện Ở CẢ HAI phép tính Bayes
(cùng làm mẫu số) — một quan sát đúng VỀ CÔNG THỨC.

Chỗ lệch: `hau_nghiem_moi` VÀ `hau_nghiem_cu` LÀ hai xác suất SAU KHI
đã CHIA cho `p_nay` — chúng LÀ tỉ lệ (giữa `0` và `1`), không còn
mang GIÁ TRỊ của `p_nay` nữa. Biết CHẮC hạt đã nảy mầm, nó PHẢI hoặc
LÀ giống mới hoặc LÀ giống cũ (bài 12: xác suất mọi khả năng cộng
đúng `1`) — hai xác suất SAU luôn cộng lại đúng `1.0`, không phải
`p_nay`.
::
:::

:::opt
Máy báo lỗi khi chạy — `hau_nghiem_cu` được TÍNH bằng `1 -
hau_nghiem_moi` thay vì gọi lại `bayes(...)`, hai cách tính KHÔNG
tương thích để CỘNG chung
::why
Gần đúng ở việc bạn để ý `hau_nghiem_cu` dùng CÁCH tính KHÁC
(`1 - ...`) so với `hau_nghiem_moi` (gọi `bayes(...)`) — một quan
sát đúng VỀ CÁCH viết code.

Chỗ lệch: CẢ HAI đều LÀ số thực (`float`) bình thường sau khi tính
xong — Python CỘNG hai số thực với nhau LUÔN hợp lệ, bất kể chúng
được tính RA bằng công thức nào. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_bayes}
Viết `bayes(p_b_cho_a, p_a, p_b)` — tính `P(a|b) = P(b|a)·P(a) /
P(b)`.

```python title=starter
def bayes(p_b_cho_a, p_a, p_b):
    return ___


p_nay_moi = 0.2
p_moi = 0.6
p_nay = 0.16

print(bayes(p_nay_moi, p_moi, p_nay))
```

```python title=solution
def bayes(p_b_cho_a, p_a, p_b):
    return p_b_cho_a * p_a / p_b


p_nay_moi = 0.2
p_moi = 0.6
p_nay = 0.16

print(bayes(p_nay_moi, p_moi, p_nay))
```

```python title=test
assert bayes(1.0, 0.5, 1.0) == 0.5, "biet chac B luon xay ra thi cong thuc rut ve P(a)"
assert bayes(0.2, 0.6, 0.16) == 0.75, "phai khop vi du chinh"
assert bayes(0.5, 1.0, 0.5) == 1.0, "P(a)=1 (chac chan xay ra) thi hau nghiem cung chac chan"
```

:::hints
- kind: attention
  body: "Nhan p_b_cho_a voi p_a, roi chia cho p_b."
- kind: strategy
  body: "p_b_cho_a * p_a / p_b"
- kind: one-line
  body: "___ = p_b_cho_a * p_a / p_b"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan p_b_cho_a voi p_a roi chia cho p_b
  requireAst:
  - kind: uses-operator, target: '*', min: 1
  - kind: uses-operator, target: '/', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\.75\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bayes đảo chiều được MỘT bước. Nhiều bước liên tiếp — có công cụ nào
NHÌN THẤY cả chuỗi cùng lúc không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chín bài liền (11-19) đều xoay quanh CÔNG THỨC — viết ra, tính bằng
tay từng bước. Có cách nào VẼ RA cả bài toán (giống mới hay cũ, nảy
hay không) thành một HÌNH, để nhìn một cái là thấy hết MỌI đường đi
có thể, không phải nhớ công thức nào dùng LÚC nào?
::::

::::checkpoint{mastery=0.8}
::::
