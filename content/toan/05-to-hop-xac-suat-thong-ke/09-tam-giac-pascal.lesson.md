---
id: toan.to-hop-xac-suat-thong-ke.tam-giac-pascal
title: Tam giác Pascal
summary: "C(n,k) = C(n−1,k−1) + C(n−1,k) — công thức truy hồi: một nhóm k phần tử từ n HOẶC chứa một phần tử cố định (chọn thêm k−1 từ n−1 còn lại) HOẶC không chứa nó (chọn đủ k từ n−1 còn lại) — hai trường hợp rời nhau cộng lại vừa khít."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.pascal-recurrence]
requires: [math.combination-symmetry, func.recursion]
concepts: [math.tam-giac-pascal]
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
`C(4,2)=6` tính bằng giai thừa. Có cách nào tính `C(n,k)` KHÔNG cần
giai thừa — chỉ dùng những `C` NHỎ hơn đã biết?
::::

::::explain{#cong-thuc-truy-hoi}
Có. **`C(n,k) = C(n−1,k−1) + C(n−1,k)`** — cố định MỘT luống bất kỳ,
mỗi nhóm `k` luống HOẶC chứa nó (chọn thêm `k−1` từ `n−1` luống còn
lại) HOẶC không chứa nó (chọn đủ `k` từ `n−1` luống còn lại). Hai
trường hợp RỜI NHAU (bài 1 — không nhóm nào vừa chứa vừa không chứa
luống đó), cộng lại vừa khít MỌI nhóm:

```python title=readonly
def to_hop_pascal(n, k):
    if k == 0 or k == n:
        return 1
    return to_hop_pascal(n - 1, k - 1) + to_hop_pascal(n - 1, k)


print(to_hop_pascal(4, 2))
```

```text title=readonly
6
```

`to_hop_pascal(4,2)` KHÔNG hề gọi `giai_thua` — nó tự GỌI LẠI CHÍNH
NÓ (đệ quy, R1 T1.3 bài 17) trên `n` NHỎ dần, dừng khi `k=0` (chọn
không gì, luôn một cách — bài 5) HOẶC `k=n` (chọn hết, luôn một cách
— bài 7).
::::

::::example{#hang-pascal}
Xếp một HÀNG các `C(4,k)` — chính là hàng `n=4` trong tam giác Pascal:

```python title=readonly
def to_hop_pascal(n, k):
    if k == 0 or k == n:
        return 1
    return to_hop_pascal(n - 1, k - 1) + to_hop_pascal(n - 1, k)


print([to_hop_pascal(4, k) for k in range(5)])
```

```text title=readonly
[1, 4, 6, 4, 1]
```

`1, 4, 6, 4, 1` — đối xứng (bài 8: `C(4,k)=C(4,4−k)`, hàng ĐỌC XUÔI
và ĐỌC NGƯỢC giống hệt nhau). Mỗi số (trừ hai đầu `1`) LÀ tổng của
hai số Ở HÀNG TRÊN (`n=3`: `1,3,3,1`) — `4 = 1+3`, `6 = 3+3`, đúng
công thức truy hồi vừa học.
::::

::::predict{#doan-hai-cach-tinh-khop-nhau commitOnce}
Byte tính `C(5,3)` bằng HAI cách hoàn toàn khác nhau — công thức
truy hồi (bài này, cộng dồn từ hàng nhỏ) VÀ công thức giai thừa
(bài 7, chia trực tiếp):

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

def to_hop_pascal(n, k):
    if k == 0 or k == n:
        return 1
    return to_hop_pascal(n - 1, k - 1) + to_hop_pascal(n - 1, k)

print(to_hop_pascal(5, 3) == to_hop(5, 3))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — công thức truy hồi CHỈ đúng cho những hàng NHỎ (gần đỉnh
tam giác); càng đi sâu, sai số CỘNG DỒN từ nhiều bước đệ quy càng
LỆCH xa kết quả giai thừa
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng "tính qua nhiều bước thì dễ
lệch" — một trực giác hợp lý với NHIỀU phép tính số THỰC (dấu phẩy
động).

Chỗ lệch: MỌI phép tính Ở ĐÂY — cộng, trừ, nhân, chia nguyên `//` —
đều LÀ số NGUYÊN, KHÔNG có phần thập phân, KHÔNG có sai số làm tròn
nào để mà cộng dồn. Dù đệ quy qua BAO NHIÊU bước, mỗi bước cộng hai
số nguyên CHÍNH XÁC — kết quả cuối LUÔN khớp tuyệt đối với công thức
giai thừa, không "gần đúng".
::
:::

:::opt
Không so sánh được — `to_hop_pascal` dùng đệ quy còn `to_hop` dùng
vòng lặp VÀ chia giai thừa, hai CÁCH tính khác nhau trả về hai KIỂU
dữ liệu không thể đem so `==`
::why
Gần đúng ở việc bạn để ý ĐÚNG hai hàm dùng CƠ CHẾ tính khác hẳn nhau
(đệ quy cộng dồn VÀ chia giai thừa trực tiếp) — quan sát đó đúng VỀ
CÁCH viết code.

Chỗ lệch: CƠ CHẾ tính bên trong một hàm KHÔNG ảnh hưởng tới KIỂU của
giá trị nó trả về — cả `to_hop_pascal(5,3)` LẪN `to_hop(5,3)` đều
trả về một số nguyên bình thường (`int`), và `==` so sánh hai số
nguyên luôn hợp lệ, bất kể chúng được TÍNH ra bằng cách nào.
::
:::
::::

::::code{#viet_to_hop_pascal}
Viết `to_hop_pascal(n, k)` — tính `C(n,k)` bằng công thức truy hồi
Pascal, KHÔNG dùng giai thừa.

```python title=starter
def to_hop_pascal(n, k):
    if k == 0 or k == n:
        return 1
    return ___


print(to_hop_pascal(4, 2))
```

```python title=solution
def to_hop_pascal(n, k):
    if k == 0 or k == n:
        return 1
    return to_hop_pascal(n - 1, k - 1) + to_hop_pascal(n - 1, k)


print(to_hop_pascal(4, 2))
```

```python title=test
assert to_hop_pascal(0, 0) == 1, "hang dinh -- dung mot cach"
assert to_hop_pascal(6, 0) == 1, "chon 0 -- luon mot cach du n lon co nao"
assert to_hop_pascal(6, 6) == 1, "chon het -- luon mot cach"
assert [to_hop_pascal(4, k) for k in range(5)] == [1, 4, 6, 4, 1], "hang n=4 dung tam giac Pascal"
assert to_hop_pascal(5, 3) == 10, "phai khop cong thuc giai thua: 5!/(3!2!)=10"
```

:::hints
- kind: attention
  body: "Goi lai chinh to_hop_pascal voi n-1, cong hai truong hop: chua phan tu co dinh va khong chua."
- kind: strategy
  body: "to_hop_pascal(n - 1, k - 1) + to_hop_pascal(n - 1, k)"
- kind: one-line
  body: "return to_hop_pascal(n - 1, k - 1) + to_hop_pascal(n - 1, k)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi lai chinh to_hop_pascal hai lan tren n-1, cong lai bang +
  requireAst:
  - kind: recursion, min: 2
  - kind: uses-operator, target: '+', min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^6\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cộng cả hàng `n=4`: `1+4+6+4+1 = 16`. Trùng hợp, hay CỘNG một hàng
Pascal LUÔN ra luỹ thừa của 2?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cộng CẢ HÀNG `n=4`: `1+4+6+4+1 = 16 = 2⁴`. Trùng hợp, hay CỘNG một
hàng Pascal LUÔN ra luỹ thừa của 2 — và nếu luôn đúng, nó đang ĐẾM
cái gì?
::::

::::checkpoint{mastery=0.8}
::::
