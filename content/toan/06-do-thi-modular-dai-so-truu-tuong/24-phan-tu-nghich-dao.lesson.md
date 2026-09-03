---
id: toan.do-thi-modular-dai-so-truu-tuong.phan-tu-nghich-dao
title: Phần tử nghịch đảo
summary: "Nghịch đảo của a — phần tử a⁻¹ sao cho a∗a⁻¹ = a⁻¹∗a = e (bài 23); MỌI số nguyên có nghịch đảo CỘNG, NHƯNG KHÔNG PHẢI mọi số có nghịch đảo NHÂN — nghịch đảo modular (bài 17) LÀ trường hợp RIÊNG của khái niệm NÀY."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.inverse-element]
requires: [math.identity-element, math.modular-inverse]
concepts: [math.phan-tu-nghich-dao]
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
Modular cộng CÓ nghịch đảo CHO mọi phần tử không? So với modular
NHÂN (bài 17, CHỈ có nghịch đảo khi `gcd=1`) thì SAO?
::::

::::explain{#phan-tu-nghich-dao}
CÓ, CHO mọi phần tử. **Nghịch đảo của `a`** — phần tử `a⁻¹` sao cho
`a∗a⁻¹ = a⁻¹∗a = e` (đơn vị, bài 23); MỌI số nguyên CÓ nghịch đảo
CỘNG (`−a`), NHƯNG KHÔNG PHẢI mọi số CÓ nghịch đảo NHÂN (chỉ `1`,
`−1`) — **nghịch đảo modular** (bài 17) LÀ trường hợp RIÊNG của
khái niệm NÀY:

```python title=readonly
def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None


Z3 = {0, 1, 2}
print(tim_nghich_dao(Z3, lambda a, b: (a + b) % 3, 0, 1))
```

```text title=readonly
2
```

`1 + 2 = 3`, `3 mod 3 = 0` (đơn vị của cộng) — `2` LÀ nghịch đảo
CỘNG của `1` modulo `3`. MỌI phần tử trong `Z3` ĐỀU CÓ nghịch đảo
cộng — kiểm được từng cái MỘT.
::::

::::example{#nghich-dao-nhan-lai-bai-17}
Áp DỤNG lên nhân modular — CHÍNH LÀ bài 17, giờ NHÌN qua khái niệm
CHUNG:

```python title=readonly
def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None


Z7 = {0, 1, 2, 3, 4, 5, 6}
print(tim_nghich_dao(Z7, lambda a, b: (a * b) % 7, 1, 3))
```

```text title=readonly
5
```

`3 × 5 = 15`, `15 mod 7 = 1` (đơn vị của nhân) — Y HỆT kết quả bài
17 (`nghich_dao_modular(3,7)=5`), giờ tính LẠI qua `tim_nghich_dao`
CHUNG — nghịch đảo NHÂN modular CHỈ LÀ MỘT trường hợp CỦA "nghịch
đảo" tổng quát.
::::

::::predict{#doan-nghich-dao-khong-ton-tai commitOnce}
Byte tìm nghịch đảo NHÂN của `4` modulo `8` (`gcd(4,8)=4≠1`):

```python
def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

Z8 = {0, 1, 2, 3, 4, 5, 6, 7}
print(tim_nghich_dao(Z8, lambda a, b: (a * b) % 8, 1, 4))
```

Dòng cuối in ra gì?

:::opt{correct}
`None`
:::

:::opt
`2` — vì `4 × 2 = 8`, VÀ `8` LÀ chính "MODULO" đang xét, nên `x=2`
PHẢI LÀ nghịch đảo hợp lý CỦA `4`
::why
Gần đúng ở việc bạn TÍNH đúng `4×2=8` — một phép NHÂN chính xác.

Chỗ lệch: nghịch đảo NHÂN đòi kết quả BẰNG đơn vị (`e=1`), KHÔNG
phải bằng `n` (`8`). `8 mod 8 = 0`, KHÔNG phải `1` — `2` KHÔNG PHẢI
nghịch đảo của `4`. THẬT ra, KHÔNG `x` nào trong `Z8` cho `4×x ≡ 1
(mod 8)` — đúng như bài 17 ĐÃ dạy, `gcd(4,8)=4≠1` nên nghịch đảo
KHÔNG tồn tại.
::
:::

:::opt
Máy báo lỗi khi chạy — tham số `a` của hàm TRÙNG TÊN với biến vòng
lặp bên TRONG comprehension của `tim_don_vi` (bài TRƯỚC), gây xung
đột TÊN biến
::why
Gần đúng ở việc bạn để ý ĐÚNG tên `a` xuất hiện Ở CẢ hai bài — một
quan sát VỀ đặt TÊN.

Chỗ lệch: `tim_nghich_dao` (bài NÀY) VÀ `tim_don_vi` (bài TRƯỚC) LÀ
hai hàm HOÀN TOÀN riêng biệt, MỖI hàm có PHẠM VI biến RIÊNG — dùng
LẠI tên `a` Ở hai hàm KHÁC nhau KHÔNG hề xung đột. Biên dịch sạch,
chạy sạch.
::
:::
::::

::::code{#viet_tim_nghich_dao}
Viết `tim_nghich_dao(tap, phep_toan, e, a)` — tìm nghịch đảo của
`a` (trả VỀ `None` nếu KHÔNG có).

```python title=starter
def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if ___:
            return x
    return None


Z3 = {0, 1, 2}
print(tim_nghich_dao(Z3, lambda a, b: (a + b) % 3, 0, 1))
```

```python title=solution
def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None


Z3 = {0, 1, 2}
print(tim_nghich_dao(Z3, lambda a, b: (a + b) % 3, 0, 1))
```

```python title=test
assert tim_nghich_dao(set(), lambda a, b: a + b, 0, 1) is None, "tap rong -- khong x nao"
Z3 = {0, 1, 2}
assert tim_nghich_dao(Z3, lambda a, b: (a + b) % 3, 0, 1) == 2, "nghich dao cong cua 1 mod 3 la 2"
assert tim_nghich_dao(Z3, lambda a, b: (a + b) % 3, 0, 0) == 0, "nghich dao cong cua 0 la chinh no"
Z7 = {0, 1, 2, 3, 4, 5, 6}
assert tim_nghich_dao(Z7, lambda a, b: (a * b) % 7, 1, 3) == 5, "nghich dao nhan cua 3 mod 7 la 5"
Z8 = {0, 1, 2, 3, 4, 5, 6, 7}
assert tim_nghich_dao(Z8, lambda a, b: (a * b) % 8, 1, 4) is None, "gcd(4,8)=4 -- khong co nghich dao"
```

:::hints
- kind: attention
  body: "x la nghich dao cua a khi phep_toan(a,x)==e VA phep_toan(x,a)==e."
- kind: strategy
  body: "phep_toan(a, x) == e and phep_toan(x, a) == e"
- kind: one-line
  body: "if phep_toan(a, x) == e and phep_toan(x, a) == e:"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem phep_toan(a,x)==e VA phep_toan(x,a)==e
  requireAst:
  - kind: uses-operator, target: 'and', min: 1
  - kind: uses-call, target: phep_toan, min: 2
  - kind: uses-name, target: e, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đóng, kết hợp, đơn vị, nghịch đảo — bốn tính chất. `(số nguyên, +)`
CÓ đủ CẢ bốn không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`(số nguyên, +)` LÀ một nhóm (đủ cả bốn tiên đề). `(số nguyên, ×)`
CÓ LÀ nhóm không — thiếu ĐÚNG tiên đề NÀO?
::::

::::checkpoint{mastery=0.8}
::::
