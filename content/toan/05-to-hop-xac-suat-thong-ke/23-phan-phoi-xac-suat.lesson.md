---
id: toan.to-hop-xac-suat-thong-ke.phan-phoi-xac-suat
title: Phân phối xác suất
summary: "Bảng phân phối của biến ngẫu nhiên rời rạc — liệt kê MỌI giá trị X có thể nhận, kèm P(X=x) cho MỖI giá trị. Cột P(X=x) cộng lại LUÔN đúng 1."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.probability-distribution]
requires: [math.random-variable]
concepts: [math.phan-phoi-xac-suat]
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
Sáu trong mười sáu kết quả cho `X=2`. Có cách nào TÓM TẮT năm giá
trị `X` có thể — mỗi giá trị bao nhiêu phần trăm — thành một bảng?
::::

::::explain{#phan-phoi-la-gi}
Có. **Bảng phân phối** của biến ngẫu nhiên rời rạc — liệt kê MỌI giá
trị `X` có thể nhận, kèm `P(X=x)` cho MỖI giá trị (gộp MỌI kết quả
trong `Ω` cho ra CÙNG giá trị đó, bài 22 để ngỏ):

```python title=readonly
def X(ket_qua):
    return sum(ket_qua)

def phan_phoi(omega, X):
    cac_gia_tri = {X(ket) for ket in omega}
    return {x: sum(1 for ket in omega if X(ket) == x) / len(omega) for x in cac_gia_tri}


omega = {(a, b, c, d) for a in [True, False] for b in [True, False] for c in [True, False] for d in [True, False]}

print(sorted(phan_phoi(omega, X).items()))
```

```text title=readonly
[(0, 0.0625), (1, 0.25), (2, 0.375), (3, 0.25), (4, 0.0625)]
```

(In bằng `sorted(...)` VÌ `dict` KHÔNG cố định thứ tự khi lấy `.items()`
qua nhiều lần chạy — bài 1.) `P(X=2)=0.375` (sáu KẾT QUẢ trên mười
sáu, bài 22). CHÍNH LÀ hàng `1,4,6,4,1` của tam giác Pascal (bài 9)
chia cho `16` — MỖI SỐ trong hàng ĐẾM đúng số kết quả cho `X` đó.
::::

::::example{#tong-phan-phoi-la-mot}
Cột `P(X=x)` cộng lại LUÔN đúng `1` (bài 12 áp lên `X` thay vì lên
`Ω` trực tiếp):

```python title=readonly
def X(ket_qua):
    return sum(ket_qua)

def phan_phoi(omega, X):
    cac_gia_tri = {X(ket) for ket in omega}
    return {x: sum(1 for ket in omega if X(ket) == x) / len(omega) for x in cac_gia_tri}


omega = {(a, b, c, d) for a in [True, False] for b in [True, False] for c in [True, False] for d in [True, False]}
pp = phan_phoi(omega, X)

print(sum(pp.values()))
```

```text title=readonly
1.0
```

MỖI kết quả trong `Ω` rơi vào ĐÚNG một giá trị `X` (vì `X` LÀ một
hàm — mỗi input MỘT output, T2.4 bài 24) — gộp lại NĂM nhóm ĐÓ vẫn
là CẢ `Ω`, nên cộng `P(X=x)` qua NĂM giá trị LUÔN ra `1`.
::::

::::predict{#doan-doi-xung-phan-phoi commitOnce}
Byte so sánh `P(X=0)` VÀ `P(X=4)`, rồi `P(X=1)` VÀ `P(X=3)`:

```python
def X(ket_qua):
    return sum(ket_qua)

def phan_phoi(omega, X):
    cac_gia_tri = {X(ket) for ket in omega}
    return {x: sum(1 for ket in omega if X(ket) == x) / len(omega) for x in cac_gia_tri}

omega = {(a, b, c, d) for a in [True, False] for b in [True, False] for c in [True, False] for d in [True, False]}
pp = phan_phoi(omega, X)

print(pp[0] == pp[4])
print(pp[1] == pp[3])
```

Hai dòng cuối in ra gì?

:::opt{correct}
`True`, rồi `True`
:::

:::opt
`False`, rồi `False` — vì `X=0` (không hạt nào nảy) và `X=4` (cả
bốn hạt nảy) là hai tình huống RẤT khác nhau, xác suất phải khác
::why
Gần đúng ở việc bạn thấy `X=0` VÀ `X=4` LÀ hai tình huống Ý NGHĨA
khác hẳn nhau — một quan sát đúng VỀ Ý NGHĨA thực tế.

Chỗ lệch: `P(X=0)=C(4,0)/16=1/16` VÀ `P(X=4)=C(4,4)/16=1/16` — CÙNG
một CON SỐ, vì `C(4,0)=C(4,4)=1` (bài 8, đối xứng tổ hợp: `C(n,k)=
C(n,n−k)`). Ý NGHĨA hai tình huống khác nhau KHÔNG có nghĩa XÁC SUẤT
của chúng khác nhau — bảng phân phối này ĐỐI XỨNG vì CHÍNH tam giác
Pascal (bài 9) đối xứng.
::
:::

:::opt
Máy báo lỗi khi chạy — `pp[0]` VÀ `pp[4]` tra khoá `int` trên MỘT
`dict`, nhưng khoá của `pp` LÀ kết quả `set`-comprehension nên KHÔNG
tra được BẰNG số thường
::why
Gần đúng ở việc bạn để ý `pp` được DỰNG từ một set-comprehension
(`cac_gia_tri`) — một quan sát VỀ CÁCH `pp` được xây.

Chỗ lệch: `cac_gia_tri` CHỈ là bước TRUNG GIAN để xác định CÓ những
khoá nào — kết quả CUỐI (`pp`) LÀ một `dict` BÌNH THƯỜNG với khoá LÀ
số nguyên (`0` tới `4`), tra bằng `pp[0]` hoàn toàn hợp lệ, giống hệt
tra bất kỳ `dict` số-nguyên-làm-khoá nào khác.
::
:::
::::

::::code{#viet_phan_phoi}
Viết `phan_phoi(omega, X)` — trả về `dict` ánh xạ MỖI giá trị `X` có
thể nhận sang `P(X=x)`.

```python title=starter
def X(ket_qua):
    return sum(ket_qua)

def phan_phoi(omega, X):
    cac_gia_tri = {X(ket) for ket in omega}
    return ___


omega = {(a, b, c, d) for a in [True, False] for b in [True, False] for c in [True, False] for d in [True, False]}

print(sorted(phan_phoi(omega, X).items()))
```

```python title=solution
def X(ket_qua):
    return sum(ket_qua)

def phan_phoi(omega, X):
    cac_gia_tri = {X(ket) for ket in omega}
    return {x: sum(1 for ket in omega if X(ket) == x) / len(omega) for x in cac_gia_tri}


omega = {(a, b, c, d) for a in [True, False] for b in [True, False] for c in [True, False] for d in [True, False]}

print(sorted(phan_phoi(omega, X).items()))
```

```python title=test
omega = {(a, b, c, d) for a in [True, False] for b in [True, False] for c in [True, False] for d in [True, False]}
pp = phan_phoi(omega, X)
assert set(pp.keys()) == {0, 1, 2, 3, 4}, "X nhan du nam gia tri, tu 0 toi 4"
assert pp[2] == 0.375, "sau tren muoi sau -- 0.375"
assert pp[0] == pp[4], "doi xung -- C(4,0)=C(4,4)"
assert pp[1] == pp[3], "doi xung -- C(4,1)=C(4,3)"
assert sum(pp.values()) == 1.0, "tong ca bang phai dung 1"
```

:::hints
- kind: attention
  body: "Voi moi gia tri x trong cac_gia_tri, dem so ket qua co X(ket)==x roi chia cho len(omega)."
- kind: strategy
  body: "{x: sum(1 for ket in omega if X(ket) == x) / len(omega) for x in cac_gia_tri}"
- kind: one-line
  body: "return {x: sum(1 for ket in omega if X(ket) == x) / len(omega) for x in cac_gia_tri}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung dict comprehension dem so ket qua cho tung gia tri x roi chia cho len(omega)
  requireAst:
  - kind: comprehension, min: 4
  - kind: uses-call, target: sum, min: 2
  - kind: uses-call, target: X, min: 2
  - kind: uses-call, target: len, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\[\(0, 0\.0625\), \(1, 0\.25\), \(2, 0\.375\), \(3, 0\.25\), \(4, 0\.0625\)\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảng phân phối xong. Có MỘT con số DUY NHẤT tóm tắt cả bảng — trung
bình bao nhiêu hạt nảy mầm MỖI LẦN gieo — không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảng phân phối của "số hạt nảy trong 4" có năm dòng (`X=0` tới
`X=4`). Byte muốn MỘT con số DUY NHẤT tóm tắt cả bảng — "trung bình
bao nhiêu hạt nảy mầm MỖI LẦN gieo". Cộng thẳng năm giá trị `X` rồi
chia 5 có đúng không, hay phải tính khác vì các dòng KHÔNG đều xác
suất?
::::

::::checkpoint{mastery=0.8}
::::
