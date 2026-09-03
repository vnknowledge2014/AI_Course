---
id: toan.to-hop-xac-suat-thong-ke.hoan-vi-chon-k
title: Hoán vị chọn k
summary: "P(n,k) = n! / (n−k)! — số cách sắp xếp có thứ tự k phần tử được chọn từ n. Bài 5 (n!) là trường hợp riêng k=n, vì (n−n)! = 0! = 1."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.permutation-select-k]
requires: [math.factorial]
concepts: [math.hoan-vi-chon-k]
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
Bốn luống, nhưng Byte chỉ thu hoạch ĐÚNG hai luống sáng nay — xếp
thứ tự cho hai trong bốn luống được bao nhiêu cách?
::::

::::explain{#hoan-vi-chon-k-la-gi}
**`P(n,k) = n! / (n−k)!`** — số cách sắp xếp CÓ THỨ TỰ `k` phần tử
được chọn TỪ `n`. Bài 5 (`n!`) LÀ trường hợp riêng `k=n`, vì
`(n−n)! = 0! = 1` (bài 5), chia cho `1` không đổi gì:

```python title=readonly
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)


print(hoan_vi(4, 2))
```

```text title=readonly
12
```

`hoan_vi(4, 2) = 4! / 2! = 24 / 2 = 12`. Kiểm lại BẰNG cách khác
(bài 4): `day_khong_lap(4, 2) = [4, 3]`, nhân dồn `4×3 = 12` — khớp.
Chia `n!` cho `(n−k)!` chính LÀ "bỏ bớt" phần đuôi `(n−k)×...×1` mà
`day_khong_lap` KHÔNG hề nhân tới (nó dừng SAU đúng `k` số hạng).
::::

::::example{#hoan-vi-bang-giai-thua}
Khi `k = n`, `hoan_vi` khớp ĐÚNG `giai_thua`:

```python title=readonly
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)


print(hoan_vi(4, 4))
print(giai_thua(4))
```

```text title=readonly
24
24
```

`hoan_vi(4, 4) = 4! / (4−4)! = 4! / 0! = 24 / 1 = 24` — bằng ĐÚNG
`giai_thua(4)`. Bài 5 KHÔNG phải một công thức riêng; nó là bài này
với `k` cố định bằng `n`.
::::

::::predict{#doan-hoan-vi-chon-khong commitOnce}
Byte "thu hoạch ĐÚNG không luống nào" — chọn `k=0` trong bốn luống,
sắp xếp thứ tự CHO một tập RỖNG:

```python
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

print(hoan_vi(4, 0))
```

Dòng cuối in ra gì?

:::opt{correct}
`1`
:::

:::opt
`4` — vì túi VẪN có bốn luống để chọn, dù cuối cùng không chọn cái
nào, số "khả năng còn treo" phải là 4
::why
Gần đúng ở việc bạn nhớ ĐÚNG có bốn luống SẴN CÓ để chọn — quan sát
đó không sai.

Chỗ lệch: `hoan_vi(n, k)` đếm số CÁCH sắp xếp MỘT NHÓM đã chọn xong
gồm `k` phần tử — với `k=0`, nhóm đó LÀ tập rỗng, và CHỈ có ĐÚNG một
cách sắp xếp tập rỗng (bài 5: `0!=1`). Số luống CÒN LẠI trong túi
(`4`) không phải là con số hàm này trả về — nó tính `4!/(4−0)! =
4!/4! = 1`, đúng công thức, không liên quan tới "còn bao nhiêu luống
chưa chọn".
::
:::

:::opt
Máy báo lỗi khi chạy — `giai_thua(4 - 0)` gọi `giai_thua(4)`, trùng
với `giai_thua(n)` Ở tử số, và Python từ chối chia MỘT số cho CHÍNH
nó
::why
Gần đúng ở việc bạn để ý mẫu số VÀ tử số CÙNG gọi `giai_thua(4)` —
một quan sát đúng VỀ mặt CÔNG THỨC.

Chỗ lệch: Python KHÔNG hề cấm chia một số cho chính nó — `24 // 24`
chạy sạch, ra `1`, giống hệt phép chia BÌNH THƯỜNG bất kỳ. Không có
gì đặc biệt xảy ra khi tử số VÀ mẫu số trùng giá trị.
::
:::
::::

::::code{#viet_hoan_vi}
Viết `hoan_vi(n, k)` — số cách sắp xếp CÓ THỨ TỰ `k` phần tử chọn từ
`n`, dùng `giai_thua` đã có.

```python title=starter
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return ___


print(hoan_vi(4, 2))
```

```python title=solution
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)


print(hoan_vi(4, 2))
```

```python title=test
assert hoan_vi(4, 0) == 1, "chon 0 phan tu -- dung mot cach (tap rong)"
assert hoan_vi(4, 4) == giai_thua(4), "k=n phai khop giai thua"
assert hoan_vi(5, 3) == 60, "5!/(5-3)! = 120/2 = 60"
assert hoan_vi(4, 1) == 4, "chon 1 trong 4, sap xep -- dung 4 cach"
```

:::hints
- kind: attention
  body: "Chia giai_thua(n) cho giai_thua(n-k), dung phep chia nguyen // (khong lam tron sai)."
- kind: strategy
  body: "giai_thua(n) // giai_thua(n - k)"
- kind: one-line
  body: "___ = giai_thua(n) // giai_thua(n - k)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai chia giai_thua(n) cho giai_thua(n - k) bang phep chia nguyen //
  requireAst:
  - kind: uses-call, target: giai_thua, min: 2
  - kind: uses-operator, target: '//', min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^12\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sắp xếp CÓ thứ tự xong. Nhưng nếu Byte chỉ cần biết ĐÚNG hai luống
nào được chọn — không quan tâm luống nào trước, luống nào sau?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte lại chọn hai luống, NHƯNG lần này KHÔNG quan tâm thu hoạch luống
nào trước — chỉ cần biết ĐÚNG hai luống nào được chọn hôm nay.
`hoan_vi(4, 2) = 12` có còn là câu trả lời đúng không, hay đang ĐẾM
DƯ một thứ mà câu hỏi mới không cần?
::::

::::checkpoint{mastery=0.8}
::::
