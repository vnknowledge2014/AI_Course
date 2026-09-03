---
id: toan.do-thi-modular-dai-so-truu-tuong.phan-tu-don-vi
title: Phần tử đơn vị
summary: "Phần tử đơn vị e — e∗a = a∗e = a VỚI MỌI a (KHÔNG đổi gì khi ghép); 0 LÀ đơn vị của +, 1 LÀ đơn vị của ×; modular cộng CÓ đơn vị LÀ 0."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.identity-element]
requires: [math.associativity]
concepts: [math.phan-tu-don-vi]
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
Modular NHÂN trên `{0,...,n−1}` — đơn vị của NÓ LÀ số nào? Nhân số
ĐÓ với BẤT KỲ phần tử nào, kết quả có GIỮ nguyên không?
::::

::::explain{#phan-tu-don-vi}
LÀ số `1`. **Phần tử đơn vị `e`** — `e∗a = a∗e = a` VỚI MỌI `a`
(KHÔNG đổi GÌ khi GHÉP); `0` LÀ đơn vị của `+`, `1` LÀ đơn vị của
`×` (đã dùng TỪ T2.1, giờ đặt TÊN):

```python title=readonly
def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None


Z3 = {0, 1, 2}
print(tim_don_vi(Z3, lambda a, b: (a + b) % 3))
```

```text title=readonly
0
```

`0 + a = a` VỚI MỌI `a` trong `Z3` (VÀ `a + 0 = a`, cùng CHIỀU) —
`0` LÀ đơn vị của cộng modular. GHÉP `0` KHÔNG đổi GÌ.
::::

::::example{#don-vi-nhan}
Modular NHÂN — đơn vị LÀ `1`:

```python title=readonly
def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None


Z4 = {0, 1, 2, 3}
print(tim_don_vi(Z4, lambda a, b: (a * b) % 4))
```

```text title=readonly
1
```

`1 × a = a` VỚI MỌI `a` (mod `4`) — `1` LÀ đơn vị của nhân modular,
đúng LỐI `1` LÀ đơn vị của nhân THƯỜNG.
::::

::::predict{#doan-don-vi-cua-tru commitOnce}
Byte tìm đơn vị của phép TRỪ trên `{1,2,3}`:

```python
def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

so_nho = {1, 2, 3}
print(tim_don_vi(so_nho, lambda a, b: a - b))
```

Dòng cuối in ra gì?

:::opt{correct}
`None`
:::

:::opt
`0` — vì `0` LÀ đơn vị TỰ NHIÊN của phép TRỪ, giống hệt cộng (trừ
ĐI `0` không đổi gì)
::why
Gần đúng ở việc bạn nhớ ĐÚNG "trừ đi `0`" không đổi GÌ — `a-0=a` LÀ
sự thật ĐÚNG.

Chỗ lệch: đơn vị đòi CẢ HAI chiều — `e∗a=a` **VÀ** `a∗e=a`. `a-0=a`
đúng, NHƯNG `0-a=a` CHỈ đúng khi `a=0`, KHÔNG đúng VỚI MỌI `a` (vd
`0-2=-2≠2`). VÀ dù `0` CÓ thoả, nó CŨNG không nằm TRONG tập `{1,2,3}`
được xét — hàm CHỈ tìm `e` bên TRONG `tap`. Không `e` nào TRONG
`{1,2,3}` thoả CẢ hai chiều — trả VỀ `None`.
::
:::

:::opt
Máy báo lỗi khi chạy — vòng lặp `for e in tap` chạy HẾT `tap` mà
KHÔNG tìm thấy gì, hàm PHẢI trả VỀ một giá trị THUỘC `tap`, KHÔNG
được phép trả `None`
::why
Gần đúng ở việc bạn để ý ĐÚNG vòng lặp chạy HẾT mà KHÔNG "return"
sớm — một quan sát VỀ luồng chạy chính xác.

Chỗ lệch: `return None` (dòng CUỐI hàm, NGOÀI vòng lặp) LÀ một
GIÁ TRỊ trả về HOÀN TOÀN hợp lệ — hàm KHÔNG bị buộc phải trả VỀ một
phần tử CỦA `tap`; `None` chính LÀ CÁCH nói "không CÓ đơn vị nào".
Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_tim_don_vi}
Viết `tim_don_vi(tap, phep_toan)` — tìm phần tử đơn vị của `phep_toan`
trên `tap` (trả VỀ `None` nếu KHÔNG có).

```python title=starter
def tim_don_vi(tap, phep_toan):
    for e in tap:
        if ___:
            return e
    return None


Z3 = {0, 1, 2}
print(tim_don_vi(Z3, lambda a, b: (a + b) % 3))
```

```python title=solution
def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None


Z3 = {0, 1, 2}
print(tim_don_vi(Z3, lambda a, b: (a + b) % 3))
```

```python title=test
assert tim_don_vi(set(), lambda a, b: a + b) is None, "tap rong -- khong e nao ca"
Z3 = {0, 1, 2}
assert tim_don_vi(Z3, lambda a, b: (a + b) % 3) == 0, "don vi cong modular la 0"
Z4 = {0, 1, 2, 3}
assert tim_don_vi(Z4, lambda a, b: (a * b) % 4) == 1, "don vi nhan modular la 1"
so_nho = {1, 2, 3}
assert tim_don_vi(so_nho, lambda a, b: a - b) is None, "tru khong co don vi trong tap nay"
```

:::hints
- kind: attention
  body: "e la don vi khi phep_toan(e,a)==a VA phep_toan(a,e)==a voi MOI a trong tap."
- kind: strategy
  body: "all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap)"
- kind: one-line
  body: "if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem phep_toan(e,a)==a VA phep_toan(a,e)==a voi moi a
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-operator, target: 'and', min: 1
  - kind: uses-call, target: phep_toan, min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đơn vị — tính chất THỨ BA. Modular cộng CÓ nghịch đảo cho MỌI phần
tử không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Modular cộng CÓ nghịch đảo CHO mọi phần tử không (thử `a` bất kỳ
trong `{0,...,n−1}`, tìm `x` sao `(a+x) mod n = 0`)? So với modular
NHÂN (bài 17, CHỈ có nghịch đảo khi `gcd=1`) thì SAO?
::::

::::checkpoint{mastery=0.8}
::::
