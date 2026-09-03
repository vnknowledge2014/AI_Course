---
id: toan.do-thi-modular-dai-so-truu-tuong.nhom-la-gi
title: Nhóm là gì
summary: "Nhóm (S, ∗) — MỘT tập S VỚI một phép toán ∗ thoả CẢ BỐN: đóng (bài 21), kết hợp (bài 22), có đơn vị (bài 23), MỌI phần tử có nghịch đảo (bài 24); GỘP bốn bài liền thành MỘT định nghĩa."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.group]
requires: [math.binary-operation, math.associativity, math.identity-element, math.inverse-element, math.equivalence-relation]
concepts: [math.nhom]
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
`(số nguyên, +)` LÀ một nhóm (đủ cả bốn tiên đề). `(số nguyên, ×)`
CÓ LÀ nhóm không — thiếu ĐÚNG tiên đề NÀO?
::::

::::explain{#nhom-la-gi}
Thiếu NGHỊCH đảo (bài 24) — MỌI số ĐỀU có nghịch đảo CỘNG, NHƯNG chỉ
`1`, `−1` MỚI có nghịch đảo NHÂN. **Nhóm `(S, ∗)`** — MỘT tập `S`
VỚI một phép toán `∗` thoả CẢ BỐN: **đóng** (bài 21), **kết hợp**
(bài 22), **có đơn vị** (bài 23), **MỌI phần tử có nghịch đảo** (bài
24) — GỘP bốn bài LIỀN thành MỘT định nghĩa:

```python title=readonly
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)


Z3 = {0, 1, 2}
print(la_nhom(Z3, lambda a, b: (a + b) % 3))
```

```text title=readonly
True
```

`Z3` VỚI cộng modular — ĐÓNG, kết HỢP, CÓ đơn vị (`0`), MỌI phần tử
CÓ nghịch đảo. ĐỦ cả bốn — `(Z3, +mod 3)` LÀ một nhóm.
::::

::::example{#khong-du-nghich-dao}
`Z4` VỚI nhân modular — THIẾU đúng MỘT tiên đề:

```python title=readonly
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)


Z4 = {0, 1, 2, 3}
print(la_nhom(Z4, lambda a, b: (a * b) % 4))
```

```text title=readonly
False
```

`Z4` VỚI nhân modular — ĐÓNG, kết HỢP, CÓ đơn vị (`1`), NHƯNG `0`
KHÔNG có nghịch đảo (`0 × x = 0`, KHÔNG BAO GIỜ ra `1`) — thiếu ĐÚNG
tiên đề CUỐI, KHÔNG PHẢI nhóm.
::::

::::predict{#doan-tap-nho-nhan-thuong commitOnce}
Byte kiểm `{−2,−1,0,1,2}` VỚI phép nhân THƯỜNG (KHÔNG modular):

```python
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)

so_nho = {-2, -1, 0, 1, 2}
print(la_nhom(so_nho, lambda a, b: a * b))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `−2,−1,1,2` ĐỀU có nghịch đảo nhân TRONG tập ĐÓ (`−1`
nghịch đảo CHÍNH nó, `1` nghịch đảo chính nó), CHỈ `0` THIẾU, NHƯNG
`0` là MỘT ngoại lệ ĐẶC biệt luôn được BỎ QUA
::why
Gần đúng ở việc bạn để ý ĐÚNG `−1` VÀ `1` LÀ nghịch đảo của CHÍNH
chúng — một quan sát chính xác VỀ hai phần tử ĐÓ.

Chỗ lệch: định nghĩa nhóm đòi **MỌI** phần tử CÓ nghịch đảo, KHÔNG
CÓ ngoại lệ "được bỏ qua" cho `0` — VÀ ngay CẢ `−2`, `2` CŨNG thiếu
nghịch đảo NHÂN TRONG tập NÀY (`2 × x = 1` không CÓ nghiệm nguyên
`x`). Tập `{−2,−1,0,1,2}` VỚI nhân THƯỜNG THẤT bại Ở NGHỊCH đảo cho
BA phần tử (`−2, 0, 2`), KHÔNG chỉ `0` — KHÔNG PHẢI nhóm.
::
:::

:::opt
Máy báo lỗi khi chạy — `so_nho` CÓ số ÂM (`−2, −1`), mà `la_dong`
VÀ các hàm KHÁC chỉ được THIẾT kế cho số KHÔNG âm (giống MỌI ví dụ
`Z3`, `Z4` TRƯỚC ĐÓ)
::why
Gần đúng ở việc bạn để ý ĐÚNG MỌI ví dụ TRƯỚC dùng số KHÔNG âm — một
quan sát VỀ MẪU hình dữ liệu.

Chỗ lệch: KHÔNG hàm nào Ở đây "chỉ thiết kế cho" số không âm — TẤT
CẢ dùng `in`, `==`, GỌI hàm CHUNG, hoạt động BÌNH thường VỚI BẤT KỲ
số nguyên nào, ÂM hay dương. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_nhom}
Viết PHẦN CÒN LẠI của `la_nhom(tap, phep_toan)` — kiểm MỌI phần tử
CÓ nghịch đảo hay KHÔNG (tiên đề cuối CÙNG).

```python title=starter
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return ___


Z3 = {0, 1, 2}
print(la_nhom(Z3, lambda a, b: (a + b) % 3))
```

```python title=solution
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)


Z3 = {0, 1, 2}
print(la_nhom(Z3, lambda a, b: (a + b) % 3))
```

```python title=test
assert la_nhom(set(), lambda a, b: a + b) is False, "tap rong -- tim_don_vi tra None ngay"
Z3 = {0, 1, 2}
assert la_nhom(Z3, lambda a, b: (a + b) % 3) is True, "Z3 cong modular la nhom"
Z4 = {0, 1, 2, 3}
assert la_nhom(Z4, lambda a, b: (a * b) % 4) is False, "Z4 nhan modular -- 0 khong co nghich dao"
so_nho = {-2, -1, 0, 1, 2}
assert la_nhom(so_nho, lambda a, b: a * b) is False, "nhan thuong tren tap nho -- khong phai nhom"
```

:::hints
- kind: attention
  body: "Voi MOI a trong tap, tim_nghich_dao(tap, phep_toan, e, a) phai KHAC None."
- kind: strategy
  body: "all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)"
- kind: one-line
  body: "___ = all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem tim_nghich_dao(tap, phep_toan, e, a) khac None voi MOI a
  requireAst:
  - kind: uses-call, target: tim_nghich_dao, min: 1
  - kind: uses-operator, target: 'is not', min: 1
  - kind: uses-call, target: all, min: 4
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhóm — gộp bốn tiên đề. Vườn của Byte (đồ thị, bài 1-12) có PHÉP
toán nào ĐÓNG-kết hợp-đơn vị-nghịch đảo GIỐNG vậy không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Lịch tưới BA ngày (bài 13) VÀ đồng hồ mười HAI giờ ĐỀU LÀ nhóm
modular, CHỈ khác `n`. Vườn của Byte (đồ thị, bài 1-12) có PHÉP toán
nào ĐÓNG-kết hợp-đơn vị-nghịch đảo GIỐNG vậy KHÔNG, hay đồ thị KHÔNG
có cấu trúc nhóm TỰ NHIÊN?
::::

::::checkpoint{mastery=0.8}
::::
