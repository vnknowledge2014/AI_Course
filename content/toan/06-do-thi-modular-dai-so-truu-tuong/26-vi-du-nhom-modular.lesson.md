---
id: toan.do-thi-modular-dai-so-truu-tuong.vi-du-nhom-modular
title: "Ví dụ nhóm: modular cộng"
summary: "(ℤₙ, +mod n) LUÔN LÀ một nhóm, VỚI MỌI n — kiểm ĐỦ bốn tiên đề (bài 21-24) TRÊN modular cộng CỤ THỂ, bằng CHÍNH những hàm ĐÃ viết; 'đồng hồ modular' (bài 20) BÂY GIỜ có TÊN toán học CHÍNH XÁC: MỘT nhóm."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.modular-group-example]
requires: [math.group, math.generalized-modular-arithmetic]
concepts: [math.nhom-modular]
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
Vườn của Byte (đồ thị, bài 1-12) KHÔNG có phép toán nhóm TỰ NHIÊN.
Modular thì SAO — MỌI `n` đều cho ra một nhóm, hay CHỈ vài `n` đặc
biệt?
::::

::::explain{#vi-du-nhom-modular}
MỌI `n`. **`(ℤₙ, +mod n)` LUÔN LÀ một nhóm**, VỚI MỌI `n>0` — kiểm
ĐỦ bốn tiên đề (bài 21-24) TRÊN modular cộng CỤ THỂ, bằng CHÍNH
những hàm ĐÃ viết. "Đồng hồ modular" (bài 20) BÂY GIỜ có TÊN toán
học CHÍNH XÁC: MỘT nhóm:

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


print(la_nhom(set(range(3)), lambda a, b: (a + b) % 3))
print(la_nhom(set(range(5)), lambda a, b: (a + b) % 5))
```

```text title=readonly
True
True
```

Lịch tưới BA ngày (`n=3`) VÀ MỘT chu kỳ NĂM ngày (`n=5`) — CẢ hai
ĐỀU LÀ nhóm. KHÔNG cần kiểm TỪNG `n` MỘT — ĐÚNG cho MỌI `n`.
::::

::::example{#nhom-tam-thuong}
NGAY CẢ trường hợp "tầm thường" nhất — `n=1`, MỘT chu kỳ chỉ CÓ một
vị trí — VẪN LÀ nhóm:

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


print(la_nhom(set(range(1)), lambda a, b: (a + b) % 1))
```

```text title=readonly
True
```

`ℤ₁ = {0}` — MỘT phần tử DUY nhất, đóng vai TRÒ vừa LÀ đơn vị vừa
LÀ nghịch đảo của CHÍNH nó. NHÓM nhỏ nhất CÓ thể — VẪN đủ CẢ bốn
tiên đề.
::::

::::predict{#doan-nhom-mod-4 commitOnce}
Byte kiểm `(ℤ₄, +mod 4)` — `n=4` LÀ một HỢP số (`4=2×2`), KHÔNG phải
số nguyên TỐ:

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

print(la_nhom(set(range(4)), lambda a, b: (a + b) % 4))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì `4` LÀ hợp số, VÀ nhóm modular CỘNG CHỈ hoạt động khi
`n` LÀ số NGUYÊN tố, đúng LỐI nghịch đảo modular NHÂN (bài 17, 24)
CHỈ tồn tại khi `gcd=1`
::why
Gần đúng ở việc bạn liên hệ TỚI điều kiện nguyên TỐ đã học Ở nghịch
đảo modular NHÂN — một liên tưởng HỢP lý, DỰA trên bài học TRƯỚC.

Chỗ lệch: điều kiện "`n` nguyên TỐ" CHỈ áp dụng cho nhóm **NHÂN**
modular (bài 17, 24: `a⁻¹` tồn tại khi `gcd(a,n)=1`, VÀ khi `n`
nguyên TỐ thì MỌI `a≠0` đều nguyên tố CÙNG `n`). Nhóm **CỘNG**
modular (`ℤₙ, +mod n`) LUÔN LÀ nhóm VỚI MỌI `n`, KHÔNG phân biệt
nguyên tố hay hợp số — VÌ MỌI phần tử LUÔN có nghịch đảo CỘNG (`n−a`)
BẤT kể `n` LÀ gì. Cộng VÀ nhân modular tuân theo hai QUY tắc khác
nhau Ở đây.
::
:::

:::opt
Máy báo lỗi khi chạy — `set(range(4))` VÀ `lambda a, b: (a + b) % 4`
dùng CÙNG con số `4` Ở hai chỗ KHÁC nhau, Python từ CHỐI lặp LẠI
CÙNG một hằng số nhiều lần trong MỘT lệnh gọi
::why
Gần đúng ở việc bạn để ý ĐÚNG số `4` xuất hiện HAI lần trong CÙNG
một lệnh GỌI — một quan sát VỀ hình thức đúng.

Chỗ lệch: Python KHÔNG hề CẤM dùng LẶP LẠI cùng một GIÁ trị TRONG
một lệnh — THẬT ra ĐÂY chính LÀ điều BẮT buộc: `range(n)` VÀ `% n`
PHẢI dùng CÙNG một `n` để nhóm khớp ĐÚNG. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_zn_la_nhom}
Viết `zn_la_nhom(n)` — kiểm `(ℤₙ, +mod n)` CÓ LÀ nhóm không.

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
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)

def zn_la_nhom(n):
    return ___


print(zn_la_nhom(3))
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

def zn_la_nhom(n):
    return la_nhom(set(range(n)), lambda a, b: (a + b) % n)


print(zn_la_nhom(3))
```

```python title=test
assert zn_la_nhom(1) is True, "n=1 -- nhom tam thuong"
assert zn_la_nhom(3) is True, "n=3 -- lich tuoi"
assert zn_la_nhom(4) is True, "n=4 -- hop so, van la nhom"
assert zn_la_nhom(5) is True, "n=5"
```

:::hints
- kind: attention
  body: "Goi la_nhom voi tap set(range(n)) va phep cong modular lambda a, b: (a+b) % n."
- kind: strategy
  body: "la_nhom(set(range(n)), lambda a, b: (a + b) % n)"
- kind: one-line
  body: "___ = la_nhom(set(range(n)), lambda a, b: (a + b) % n)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi la_nhom voi set(range(n)) va phep cong modular
  requireAst:
  - kind: uses-call, target: la_nhom, min: 1
  - kind: uses-call, target: range, min: 1
  - kind: uses-call, target: set, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
MỌI `n` đều CHO một nhóm cộng modular. Hàm — CÓ thể GÁN, TRUYỀN,
TRẢ VỀ như một GIÁ trị bình THƯỜNG — ĐÃ dùng nhiều rồi, nhưng CHƯA
đặt TÊN toán học.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hợp thành ánh xạ (`f∘g`, T2.4 bài 27) — tập TẤT CẢ song ánh TỪ một
tập HỮU hạn VÀO chính nó, VỚI phép hợp thành — có phải MỘT nhóm
không (đóng, kết hợp, đơn vị, nghịch đảo)? Đơn vị LÀ ánh xạ NÀO?
::::

::::checkpoint{mastery=0.8}
::::
