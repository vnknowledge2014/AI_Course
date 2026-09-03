---
id: toan.to-hop-xac-suat-thong-ke.to-hop-chon-k
title: Tổ hợp chọn k
summary: "C(n,k) = n! / (k!(n−k)!) — số cách chọn k phần tử từ n, KHÔNG quan tâm thứ tự. Mỗi nhóm k phần tử bị P(n,k) đếm lặp lại đúng k! lần, nên chia cho k! để đếm mỗi nhóm đúng một lần."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.combination-select-k]
requires: [math.permutation-select-k]
concepts: [math.to-hop-chon-k]
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
`hoan_vi(4, 2) = 12` — đếm CẢ thứ tự. Nếu Byte chỉ cần biết ĐÚNG hai
luống nào được chọn, không quan tâm luống nào trước?
::::

::::explain{#to-hop-la-gi}
**Tổ hợp `C(n,k) = n! / (k! (n−k)!)`** — số cách chọn `k` phần tử từ
`n`, KHÔNG quan tâm thứ tự. Mỗi NHÓM `k` phần tử bị `hoan_vi(n,k)`
(bài 6) đếm LẶP LẠI đúng `k!` lần (mọi cách sắp xếp lại CÙNG nhóm
ấy), nên chia cho `k!` để đếm mỗi nhóm ĐÚNG một lần:

```python title=readonly
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

def to_hop(n, k):
    return hoan_vi(n, k) // giai_thua(k)


print(to_hop(4, 2))
```

```text title=readonly
6
```

`hoan_vi(4,2) = 12` đếm CẢ hai thứ tự của MỖI cặp luống (luống 1
trước-luống 3 sau, VÀ luống 3 trước-luống 1 sau — hai cách xếp CÙNG
một cặp). `12 / 2! = 12/2 = 6` — chia cho `2!` gộp mỗi cặp lại thành
MỘT nhóm duy nhất.
::::

::::example{#chon-het-hay-khong-chon}
Hai đầu MÚT: chọn HẾT, hoặc chọn KHÔNG gì — luôn ĐÚNG một cách:

```python title=readonly
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

def to_hop(n, k):
    return hoan_vi(n, k) // giai_thua(k)


print(to_hop(4, 4))
print(to_hop(4, 0))
```

```text title=readonly
1
1
```

Chọn CẢ bốn luống — chỉ ĐÚNG một NHÓM khả dĩ (chính bốn luống đó),
dù `hoan_vi(4,4)=24` cách SẮP XẾP chúng. Chọn KHÔNG luống nào — cũng
ĐÚNG một nhóm (nhóm rỗng, bài 5).
::::

::::predict{#doan-to-hop-vs-hoan-vi commitOnce}
Byte so sánh `to_hop(4, 4)` VÀ `hoan_vi(4, 4)` — CÙNG chọn cả bốn
luống:

```python
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

def to_hop(n, k):
    return hoan_vi(n, k) // giai_thua(k)

print(hoan_vi(4, 4))
print(to_hop(4, 4))
```

Hai con số này có BẰNG nhau không?

:::opt{correct}
Không — `hoan_vi(4,4) = 24` (mọi thứ tự xếp khác nhau đều được đếm
riêng), `to_hop(4,4) = 1` (chỉ ĐÚNG một nhóm gồm cả bốn, thứ tự
KHÔNG quan trọng)
:::

:::opt
Có — cả hai đều `= 24`, vì chọn HẾT `n` phần tử thì thứ tự và không
thứ tự phải cho CÙNG một câu trả lời
::why
Gần đúng ở việc bạn nghĩ "chọn hết thì còn gì để phân biệt thứ tự" —
một cảm giác dễ hiểu nhầm khi `k=n`.

Chỗ lệch: DÙ chọn hết bốn luống, VẪN có `4!=24` cách khác nhau để
XẾP THỨ TỰ bốn luống đó (luống nào thu hoạch trước, luống nào sau).
`hoan_vi` đếm HẾT 24 cách xếp ấy; `to_hop` chỉ hỏi "nhóm luống nào
được chọn" — mà chỉ CÓ một nhóm khả dĩ khi chọn hết — nên `to_hop`
LUÔN là `1`, bất kể `n` lớn cỡ nào, miễn `k=n`.
::
:::

:::opt
Có — cả hai đều `= 1`, vì "chọn hết" nghĩa là không còn LỰA CHỌN
nào khác, nên số cách phải là 1 CHO CẢ HAI phép đếm
::why
Gần đúng ở việc bạn thấy ĐÚNG `to_hop(4,4)=1` — con số đó đúng.

Chỗ lệch: "không còn lựa chọn NHÓM nào khác" (đúng cho tổ hợp) KHÁC
hẳn "không còn lựa chọn THỨ TỰ nào khác" (sai cho hoán vị). Chọn hết
bốn luống rồi vẫn PHẢI quyết định xếp chúng theo thứ tự NÀO — bốn vị
trí, `4!=24` cách gán — đó chính LÀ việc `hoan_vi` đếm, và nó không
hề rút gọn về `1` chỉ vì `k=n`.
::
:::
::::

::::code{#viet_to_hop}
Viết `to_hop(n, k)` — số cách chọn `k` phần tử từ `n`, KHÔNG quan
tâm thứ tự, dùng `hoan_vi` và `giai_thua` đã có.

```python title=starter
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

def to_hop(n, k):
    return ___


print(to_hop(4, 2))
```

```python title=solution
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

def to_hop(n, k):
    return hoan_vi(n, k) // giai_thua(k)


print(to_hop(4, 2))
```

```python title=test
assert to_hop(4, 0) == 1, "chon 0 phan tu -- dung mot nhom (rong)"
assert to_hop(4, 4) == 1, "chon het -- dung mot nhom"
assert to_hop(5, 3) == 10, "5!/(3!2!) = 120/12 = 10"
assert to_hop(4, 1) == 4, "chon 1 trong 4 -- 4 nhom kha di"
assert to_hop(4, 4) != hoan_vi(4, 4), "to hop va hoan vi phai KHAC nhau tai k=n (1 khac 24)"
```

:::hints
- kind: attention
  body: "Chia hoan_vi(n, k) cho giai_thua(k) -- moi nhom bi hoan vi dem lap k! lan."
- kind: strategy
  body: "hoan_vi(n, k) // giai_thua(k)"
- kind: one-line
  body: "___ = hoan_vi(n, k) // giai_thua(k)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai chia hoan_vi(n, k) cho giai_thua(k) bang phep chia nguyen //
  requireAst:
  - kind: uses-call, target: hoan_vi, min: 1
  - kind: uses-call, target: giai_thua, min: 1
  - kind: uses-operator, target: '//', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^6\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`C(4,2) = 6` — không quan tâm thứ tự. Chọn NHỮNG luống GIỮ LẠI hay
chọn những luống BỎ RA — có phải CÙNG một câu hỏi không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte tính `C(4,2) = 6`, rồi tính tiếp `C(4,2)` theo cách chọn "hai
luống KHÔNG được chọn" thay vì "hai luống ĐƯỢC chọn" — hai câu hỏi
nghe khác hẳn nhau. Kết quả có trùng nhau không, và nếu có thì VÌ
SAO?
::::

::::checkpoint{mastery=0.8}
::::
