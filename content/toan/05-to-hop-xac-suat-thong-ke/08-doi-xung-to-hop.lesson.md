---
id: toan.to-hop-xac-suat-thong-ke.doi-xung-to-hop
title: Đối xứng của tổ hợp
summary: "C(n,k) = C(n,n−k) — chọn k phần tử GIỮ LẠI cũng chính là chọn n−k phần tử BỎ RA, hai câu hỏi đếm CÙNG một tập kết quả nhìn từ hai phía."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.combination-symmetry]
requires: [math.combination-select-k]
concepts: [math.doi-xung-to-hop]
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
`to_hop(4,2)=6` — chọn HAI luống GIỮ LẠI. Chọn HAI luống BỎ RA (cũng
để lại đúng hai luống khác) — có phải CÙNG một câu hỏi không?
::::

::::explain{#doi-xung-to-hop-la-gi}
Có. **`C(n,k) = C(n,n−k)`** — chọn `k` phần tử GIỮ LẠI cũng CHÍNH LÀ
chọn `n−k` phần tử BỎ RA: MỖI cách chọn hai luống giữ lại ĐI CÙNG
đúng một cách chọn hai luống bỏ ra (phần còn lại), một-một khớp nhau:

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
print(to_hop(4, 4 - 2))
```

```text title=readonly
6
6
```

`to_hop(4,2)` (chọn 2 giữ) VÀ `to_hop(4,2)` (chọn `4−2=2` bỏ) — TRÙNG
NHAU vì `k` và `n−k` cùng là `2` khi `n=4,k=2`. Thử cặp KHÔNG bằng
nhau ở ví dụ sau để thấy rõ hơn.
::::

::::example{#doi-xung-khong-bang-nhau}
`k` và `n−k` KHÁC nhau — vẫn cho CÙNG kết quả:

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


print(to_hop(5, 1))
print(to_hop(5, 4))
```

```text title=readonly
5
5
```

Chọn `1` GIỮ LẠI trong `5` — cũng LÀ chọn `4` BỎ RA (phần còn lại).
`to_hop(5,1)=5` VÀ `to_hop(5,4)=5` — bằng nhau, dù `k=1` và `k=4` là
hai con số KHÁC hẳn.
::::

::::predict{#doan-doi-xung-tai-giua commitOnce}
Byte thử `n=4, k=2` — trường hợp `k` ĐÚNG BẰNG `n−k` (điểm GIỮA của
đối xứng):

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

print(to_hop(6, 3) == to_hop(6, 6 - 3))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì `k` và `n−k` CÙNG bằng `3` (điểm giữa CHÍNH XÁC), tính
hai lần CÙNG một biểu thức chỉ để so sánh THƯỜNG cho `False` do sai
số làm tròn
::why
Gần đúng ở việc bạn để ý `k=3` VÀ `n−k=6−3=3` LÀ cùng một con số —
quan sát đó đúng.

Chỗ lệch: `to_hop` dùng TOÀN phép chia nguyên `//` (bài 6, 7) — KHÔNG
có số thực, KHÔNG có sai số làm tròn nào cả. Hai lời gọi
`to_hop(6,3)` VÀ `to_hop(6,3)` (vì `6−3=3`) tính RA đúng CÙNG một số
nguyên, và `==` so sánh hai số nguyên GIỐNG hệt nhau LUÔN cho `True`.
::
:::

:::opt
Máy báo lỗi khi chạy — `to_hop(6, 6 - 3)` gọi `to_hop` với đối số
TRÙNG kết quả của lời gọi TRƯỚC nó, Python phát hiện trùng lặp và từ
chối tính lại
::why
Gần đúng ở việc bạn để ý hai lời gọi `to_hop(6,3)` VÀ `to_hop(6,
6-3)` nhìn khá GIỐNG nhau — một quan sát về HÌNH THỨC dòng code.

Chỗ lệch: Python KHÔNG hề "nhớ" các lời gọi hàm trước đó để so trùng
lặp — MỖI lời gọi `to_hop(...)` chạy ĐỘC LẬP, tính lại từ đầu, dù
đối số có trùng lời gọi khác hay không. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_doi_xung}
Viết `la_doi_xung(n, k)` — kiểm `to_hop(n, k)` có bằng `to_hop(n,
n−k)` hay không (LUÔN đúng, nhưng viết ra để KIỂM LẠI bằng máy).

```python title=starter
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

def to_hop(n, k):
    return hoan_vi(n, k) // giai_thua(k)

def la_doi_xung(n, k):
    return ___


print(la_doi_xung(4, 2))
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

def la_doi_xung(n, k):
    return to_hop(n, k) == to_hop(n, n - k)


print(la_doi_xung(4, 2))
```

```python title=test
assert la_doi_xung(4, 2) is True, "k=2, n-k=2 -- trung nhau"
assert la_doi_xung(5, 1) is True, "k=1, n-k=4 -- khac nhau nhung to_hop bang nhau"
assert la_doi_xung(6, 0) is True, "k=0, n-k=6 -- chon khong gi vs chon het, van doi xung"
assert la_doi_xung(7, 3) is True, "k=3, n-k=4"
```

:::hints
- kind: attention
  body: "So sanh to_hop(n, k) voi to_hop(n, n - k) bang toan tu ==."
- kind: strategy
  body: "to_hop(n, k) == to_hop(n, n - k)"
- kind: one-line
  body: "___ = to_hop(n, k) == to_hop(n, n - k)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh to_hop(n, k) voi to_hop(n, n - k) bang ==
  requireAst:
  - kind: uses-call, target: to_hop, min: 2
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`C(4,2)=6` đối xứng quanh `k=2`. Xếp `C(4,0)..C(4,4)` thành một hàng
— có cách nào TÍNH hàng tiếp theo mà không tính lại giai thừa không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`C(4,2)=6` đối xứng quanh `k=2`. Xếp `C(4,0), C(4,1), C(4,2), C(4,3),
C(4,4)` thành một hàng — `1, 4, 6, 4, 1`. Có cách nào TÍNH hàng tiếp
theo (`n=5`) TỪ hàng NÀY, không cần tính lại giai thừa từ đầu?
::::

::::checkpoint{mastery=0.8}
::::
