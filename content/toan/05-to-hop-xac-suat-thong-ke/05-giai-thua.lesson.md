---
id: toan.to-hop-xac-suat-thong-ke.giai-thua
title: Giai thừa
summary: "n! = n × (n−1) × ... × 2 × 1 — số cách sắp xếp toàn bộ n phần tử không lặp. 0! = 1 (một cách sắp xếp \"không có gì\", đúng một cách, không phải không cách nào)."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.factorial]
requires: [math.permutation-vs-repetition]
concepts: [math.giai-thua]
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
`day_khong_lap(4, 4)` cho `[4,3,2,1]`, nhân dồn ra `24`. Phép nhân
một dãy giảm dần TỪ `n` XUỐNG `1` như vậy có tên riêng không?
::::

::::explain{#giai-thua-la-gi}
Có. **Giai thừa `n!`** — `n! = n × (n−1) × ... × 2 × 1`, đúng phép
nhân dãy `day_khong_lap(n, n)` (bài 4, trường hợp CHỌN HẾT, `k=n`):

```python title=readonly
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua


print(giai_thua(4))
```

```text title=readonly
24
```

`giai_thua(4) = 4×3×2×1 = 24` — đúng số cách xếp thứ tự thu hoạch
TOÀN BỘ bốn luống (bài 4). `range(1, n+1)` đi từ `1` tới `n`, KHÔNG
đi ngược như `day_khong_lap` — thứ tự nhân không đổi kết quả (T2.1
bài 20, phép nhân giao hoán).
::::

::::example{#giai-thua-nho}
Giai thừa của những số NHỎ — mỗi số thêm MỘT thừa số:

```python title=readonly
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua


print(giai_thua(1))
print(giai_thua(2))
print(giai_thua(3))
```

```text title=readonly
1
2
6
```

`1! = 1` (một phần tử, đúng MỘT cách xếp — chính nó). `2! = 2×1 = 2`
(hai phần tử, hai cách: trước-sau hoặc sau-trước). `3! = 3×2×1 = 6`.
::::

::::predict{#doan-giai-thua-khong commitOnce}
Byte có CHƯA luống nào để thu hoạch (không luống nào, `n=0`) — "sắp
xếp thứ tự" của MỘT TẬP RỖNG:

```python
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

print(giai_thua(0))
```

Dòng cuối in ra gì?

:::opt{correct}
`1`
:::

:::opt
`0` — không có gì để sắp xếp thì SỐ CÁCH sắp xếp phải là `0`, không
phải `1`
::why
Gần đúng ở việc bạn nghĩ "không có gì" nghe GẦN như "không có cách
nào" — một trực giác dễ hiểu nhầm, rất phổ biến với `0!`.

Chỗ lệch: "không có luống nào để sắp" và "không có CÁCH nào sắp" là
HAI câu KHÁC hẳn nhau. Có ĐÚNG MỘT cách sắp xếp một tập RỖNG — cái
"không làm gì cả" — giống hệt chân lý rỗng (T2.3 bài 11: một luật
không có ai để vi phạm thì KHÔNG ai phá được nó, luật vẫn ĐÚNG chứ
không phải "vô nghĩa"). `range(1, 1)` rỗng, vòng lặp KHÔNG chạy lần
nào, `ket_qua` giữ nguyên giá trị bắt đầu — `1`.
::
:::

:::opt
Máy báo lỗi khi chạy — `range(1, 1)` rỗng khiến vòng lặp KHÔNG thực
thi lần nào, và hàm KHÔNG CÓ gì để `return`
::why
Gần đúng ở việc bạn đọc ĐÚNG `range(1, 0+1) = range(1, 1)` rỗng —
vòng lặp thật sự không chạy lần nào, quan sát đó chính xác.

Chỗ lệch: `ket_qua = 1` đã được gán TRƯỚC vòng lặp — dòng `return
ket_qua` LUÔN có một giá trị để trả, DÙ vòng lặp chạy `0` lần hay
`100` lần (đúng cấu trúc cộng dồn R1 bài 12: giá trị khởi tạo LUÔN
sẵn sàng, vòng lặp chỉ CẬP NHẬT nó). Không có dòng lệnh nào ở đây bị
bỏ trống.
::
:::
::::

::::code{#viet_giai_thua}
Viết `giai_thua(n)` — nhân dồn từ `1` tới `n`.

```python title=starter
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ___
    return ket_qua


print(giai_thua(4))
```

```python title=solution
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua


print(giai_thua(4))
```

```python title=test
assert giai_thua(0) == 1, "0! = 1"
assert giai_thua(1) == 1, "1! = 1"
assert giai_thua(2) == 2, "2! = 2"
assert giai_thua(3) == 6, "3! = 6"
assert giai_thua(5) == 120, "5! = 120"
```

:::hints
- kind: attention
  body: "Nhan don ket_qua voi i moi luot, dung vong lap da co."
- kind: strategy
  body: "ket_qua * i"
- kind: one-line
  body: "ket_qua = ket_qua * i"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan don ket_qua voi i moi luot
  requireAst:
  - kind: uses-operator, target: '*', min: 1
  - kind: uses-name, target: ket_qua, min: 1
  - kind: uses-name, target: i, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^24\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`4! = 24` — xếp toàn bộ bốn luống. Nhưng nếu Byte chỉ có thời gian
thu hoạch HAI luống sáng nay, không phải cả bốn?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bốn luống, `4! = 24` cách xếp thứ tự thu hoạch TOÀN BỘ. Nhưng Byte
chỉ có thời gian thu hoạch ĐÚNG hai luống sáng nay (không phải cả
bốn) — xếp thứ tự cho HAI trong bốn luống được bao nhiêu cách, và
`4!` còn dùng thẳng được không?
::::

::::checkpoint{mastery=0.8}
::::
