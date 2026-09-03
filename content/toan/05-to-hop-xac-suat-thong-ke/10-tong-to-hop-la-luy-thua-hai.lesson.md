---
id: toan.to-hop-xac-suat-thong-ke.tong-to-hop-la-luy-thua-hai
title: Tổng mọi tổ hợp là luỹ thừa của 2
summary: "C(n,0) + C(n,1) + ... + C(n,n) = 2ⁿ — cộng số cách chọn 0, 1, 2, ..., n phần tử chính là đếm mọi tập con có thể của một tập n phần tử. Đóng vòng thẳng về T2.4: mọi tập hợp có 2ⁿ tập con."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.subset-count-power-of-two]
requires: [math.pascal-recurrence]
concepts: [math.tong-to-hop-luy-thua-hai]
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
`1+4+6+4+1 = 16 = 2⁴`. Trùng hợp, hay cộng một hàng Pascal LUÔN ra
luỹ thừa của 2?
::::

::::explain{#tong-to-hop-la-gi}
Không trùng hợp. **`C(n,0) + C(n,1) + ... + C(n,n) = 2ⁿ`** — cộng số
cách chọn `0`, `1`, `2`, ..., `n` phần tử chính LÀ đếm MỌI tập con
có thể của một tập `n` phần tử (MỖI phần tử: có mặt hay không, hai
lựa chọn — quy tắc nhân, bài 3, áp `n` lần):

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


print(sum(to_hop(4, k) for k in range(5)))
print(2 ** 4)
```

```text title=readonly
16
16
```

Bốn luống, MỖI luống "có mặt trong tập con" hay "không" — hai lựa
chọn ĐỘC LẬP mỗi luống, quy tắc nhân cho `2×2×2×2=16`. Cộng dồn theo
KÍCH THƯỚC (`C(4,0)` tập con RỖNG, `C(4,1)` tập con MỘT phần tử,...)
ra CÙNG con số — hai cách ĐẾM, một câu trả lời.
::::

::::example{#khop-t2-4}
T2.4 (bài 1-13) đã DÙNG `set`/`⊆` không cần đếm — đây LÀ con số đó,
tính ra bằng công thức:

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


tap_rau = {"ca_chua", "xa_lach", "ca_rot"}
print(sum(to_hop(len(tap_rau), k) for k in range(len(tap_rau) + 1)))
print(2 ** len(tap_rau))
```

```text title=readonly
8
8
```

Ba loại rau — `2³=8` tập con (kể cả tập RỖNG `∅` bài 5 T2.4, và
CHÍNH `tap_rau` bài 7 T2.4). T2.4 KHÔNG BAO GIỜ đếm con số này —
track đó chỉ so sánh `⊆` từng CẶP tập cụ thể, chưa từng hỏi "TẤT CẢ
có bao nhiêu".
::::

::::predict{#doan-them-mot-luong commitOnce}
Byte thêm luống thứ năm vào bốn luống cũ (`n` từ `4` lên `5`):

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

def tong_to_hop(n):
    return sum(to_hop(n, k) for k in range(n + 1))

print(tong_to_hop(4))
print(tong_to_hop(5))
print(tong_to_hop(5) == 2 * tong_to_hop(4))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — thêm MỘT luống thì phải cộng thêm ĐÚNG một lượng CỐ ĐỊNH
(bằng `n+1`), không phải NHÂN ĐÔI toàn bộ
::why
Gần đúng ở việc bạn nghĩ TỚI kiểu tăng CỘNG THÊM — hợp lý với NHIỀU
đại lượng đời thường (thêm một luống thì diện tích vườn cộng thêm
MỘT khoảng cố định).

Chỗ lệch: số tập con KHÔNG tăng kiểu CỘNG THÊM — nó tăng kiểu NHÂN
ĐÔI, vì luống MỚI đem lại đúng MỘT lựa chọn nhị phân THÊM (có mặt
hay không trong tập con) áp lên MỌI tập con CŨ đã có (mỗi tập con cũ
giờ nhân đôi thành hai: một bản THÊM luống mới, một bản GIỮ nguyên).
`2⁵ = 2 × 2⁴`, đúng gấp đôi, không phải `2⁴ + 5`.
::
:::

:::opt
Còn tuỳ LUỐNG thứ năm đó cụ thể trồng gì — nếu nó trồng CÙNG loại
rau với một luống đã có, số tập con sẽ tăng ÍT hơn gấp đôi
::why
Gần đúng ở việc bạn nghĩ TỚI nội dung CỤ THỂ của luống mới (trồng
gì) như một yếu tố có thể ảnh hưởng — một trực giác hợp lý khi bàn
về DỮ LIỆU rau thật.

Chỗ lệch: `tong_to_hop(n)` CHỈ đếm CẤU TRÚC — "n phần tử, mỗi phần
tử có mặt hay không" — nó KHÔNG hề biết (và không cần biết) TỪNG
phần tử LÀ loại rau gì. Luống thứ năm được coi là một PHẦN TỬ mới,
KHÁC biệt về mặt đếm với bốn luống cũ, bất kể nó trồng loại rau
trùng hay khác — luôn nhân đôi, không phụ thuộc nội dung.
::
:::
::::

::::code{#viet_tong_to_hop}
Viết `tong_to_hop(n)` — cộng `C(n,k)` với `k` chạy từ `0` tới `n`.

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

def tong_to_hop(n):
    return ___


print(tong_to_hop(4))
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

def tong_to_hop(n):
    return sum(to_hop(n, k) for k in range(n + 1))


print(tong_to_hop(4))
```

```python title=test
assert tong_to_hop(0) == 1, "tap rong -- dung mot tap con: chinh no"
assert tong_to_hop(1) == 2, "mot phan tu -- hai tap con: rong va chinh no"
assert tong_to_hop(5) == 32, "phai khop 2**5"
assert tong_to_hop(5) == 2 * tong_to_hop(4), "them mot phan tu la nhan doi, khong phai cong them"
```

:::hints
- kind: attention
  body: "Dung sum() voi generator expression, cong to_hop(n, k) voi k chay tu 0 toi n."
- kind: strategy
  body: "sum(to_hop(n, k) for k in range(n + 1))"
- kind: one-line
  body: "___ = sum(to_hop(n, k) for k in range(n + 1))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung sum() voi generator tren to_hop(n, k), k chay tu 0 toi n
  requireAst:
  - kind: uses-call, target: sum, min: 1
  - kind: uses-call, target: to_hop, min: 1
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^16\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đếm CÁCH CHỌN xong — mười bài liền không có một "hạt" hay "luống"
ngẫu nhiên nào cả, chỉ đếm KHẢ NĂNG. Rút MỘT hạt THẬT thì sao?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đếm cách chọn xong — mười bài liền không có một "hạt" hay "luống"
ngẫu nhiên nào cả, chỉ đếm KHẢ NĂNG. Byte rút MỘT hạt THẬT từ túi
trộn — hạt nào ra là chuyện MAY RỦI, không còn đếm hết mọi khả năng
nữa. Có cách nào NÓI VỀ sự may rủi ấy bằng con số không?
::::

::::checkpoint{mastery=0.8}
::::
