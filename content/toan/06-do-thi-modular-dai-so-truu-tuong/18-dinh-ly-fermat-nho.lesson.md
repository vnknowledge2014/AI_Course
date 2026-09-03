---
id: toan.do-thi-modular-dai-so-truu-tuong.dinh-ly-fermat-nho
title: Định lý Fermat nhỏ
summary: "aᵖ⁻¹ ≡ 1 (mod p) khi p NGUYÊN TỐ VÀ a KHÔNG chia hết cho p — một CÔNG THỨC RÚT GỌN cho luỹ thừa modular LỚN, KHÔNG cần nhân LẶP hàng nghìn lần."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.fermat-little-theorem]
requires: [math.modular-inverse, math.independence-chain]
concepts: [math.dinh-ly-fermat-nho]
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
`2¹⁰⁰ mod 101` — nhân LẶP một trăm lần thì CHẬM. Có công thức nào
TÍNH ra kết quả NGAY LẬP TỨC không?
::::

::::explain{#dinh-ly-fermat-nho}
CÓ. **Định lý Fermat nhỏ**: `aᵖ⁻¹ ≡ 1 (mod p)` khi `p` NGUYÊN TỐ VÀ
`a` KHÔNG chia hết cho `p` — một CÔNG THỨC RÚT GỌN cho luỹ thừa
modular LỚN, KHÔNG cần nhân LẶP hàng nghìn lần:

```python title=readonly
def luy_thua_modular(a, k, n):
    ket_qua = 1
    for _ in range(k):
        ket_qua = (ket_qua * a) % n
    return ket_qua


print(luy_thua_modular(3, 6, 7))
```

```text title=readonly
1
```

`7` LÀ số nguyên TỐ, `3` KHÔNG chia hết CHO `7` — Fermat nói
`3⁶ ≡ 1 (mod 7)` (`p−1=6`). Kiểm bằng CÁCH nhân LẶP THẬT — ĐÚNG `1`,
KHÔNG cần biết `3⁶=729` LÀ số bao nhiêu.
::::

::::example{#so-mu-lon}
ĐÚNG lời hứa Ở đầu bài — `2¹⁰⁰ mod 101` cũng RA `1`:

```python title=readonly
def luy_thua_modular(a, k, n):
    ket_qua = 1
    for _ in range(k):
        ket_qua = (ket_qua * a) % n
    return ket_qua


print(luy_thua_modular(2, 100, 101))
```

```text title=readonly
1
```

`101` LÀ số nguyên TỐ, `2` KHÔNG chia hết CHO `101` — Fermat nói
`2¹⁰⁰ ≡ 1 (mod 101)` (`p−1=100`) MÀ KHÔNG cần biết `2¹⁰⁰` LÀ một số
LỚN CỠ NÀO.
::::

::::predict{#doan-a-chia-het-cho-p commitOnce}
Byte thử `luy_thua_modular(7, 6, 7)` — LẦN NÀY `a=7` CHIA HẾT cho
`p=7` (VI PHẠM điều kiện của định LÝ):

```python
def luy_thua_modular(a, k, n):
    ket_qua = 1
    for _ in range(k):
        ket_qua = (ket_qua * a) % n
    return ket_qua

print(luy_thua_modular(7, 6, 7))
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì `p=7` VẪN LÀ số nguyên TỐ, VÀ `p−1=6` VẪN LÀ số MŨ đúng
CÔNG thức, nên định LÝ Fermat VẪN áp dụng được bình THƯỜNG
::why
Gần đúng ở việc bạn để ý ĐÚNG `p=7` VẪN nguyên tố VÀ số mũ VẪN LÀ
`p−1=6` — một quan sát ĐÚNG VỀ hai trong BA điều kiện.

Chỗ lệch: định lý Fermat đòi CẢ HAI: `p` nguyên tố **VÀ** `a` KHÔNG
chia hết cho `p`. Ở đây `a=7` CHIA HẾT cho `p=7` (`7 mod 7 = 0`) —
VI PHẠM điều kiện THỨ HAI. Khi `a` chia hết `p`, `a mod p = 0`, nên
MỌI luỹ thừa của `a` CŨNG chia hết `p` (`0×0×...×0=0`) — kết quả
LUÔN LÀ `0`, KHÔNG phải `1`.
::
:::

:::opt
Máy báo lỗi khi chạy — `a=7` VÀ `n=7` LÀ cùng MỘT giá trị, Python
KHÔNG cho phép hai THAM số của MỘT hàm nhận CÙNG một con số
::why
Gần đúng ở việc bạn để ý ĐÚNG `a` VÀ `n` cùng LÀ `7` — một quan sát
VỀ dữ liệu.

Chỗ lệch: KHÔNG có RÀNG buộc nào như VẬY — hai THAM số HOÀN TOÀN có
thể nhận CÙNG một GIÁ trị, Python KHÔNG hề kiểm TRA hay CẤM điều
NÀY. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_luy_thua_modular}
Viết PHẦN CÒN LẠI của `luy_thua_modular(a, k, n)` — MỖI bước, nhân
`ket_qua` với `a` RỒI lấy dư CHO `n`.

```python title=starter
def luy_thua_modular(a, k, n):
    ket_qua = 1
    for _ in range(k):
        ket_qua = ___
    return ket_qua


print(luy_thua_modular(3, 6, 7))
```

```python title=solution
def luy_thua_modular(a, k, n):
    ket_qua = 1
    for _ in range(k):
        ket_qua = (ket_qua * a) % n
    return ket_qua


print(luy_thua_modular(3, 6, 7))
```

```python title=test
assert luy_thua_modular(3, 6, 7) == 1, "fermat: 3^6 mod 7 = 1 (7 nguyen to, 3 khong chia het 7)"
assert luy_thua_modular(2, 100, 101) == 1, "fermat: 2^100 mod 101 = 1"
assert luy_thua_modular(7, 6, 7) == 0, "a chia het p -- ket qua luon 0, khong phai 1"
assert luy_thua_modular(5, 0, 11) == 1, "mu 0 -- luon la 1 (khong lap lan nao)"
assert luy_thua_modular(5, 10, 11) == 1, "fermat: 11 nguyen to, 5 khong chia het 11"
```

:::hints
- kind: attention
  body: "Nhan ket_qua voi a, roi lay du cho n -- dung * va %."
- kind: strategy
  body: "(ket_qua * a) % n"
- kind: one-line
  body: "ket_qua = (ket_qua * a) % n"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan ket_qua voi a roi lay du cho n
  requireAst:
  - kind: uses-operator, target: '*', min: 1
  - kind: uses-operator, target: '%', min: 1
  - kind: uses-name, target: a, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^1\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Fermat nhỏ RÚT gọn luỹ thừa modular. Mã hoá "A" dịch `3` vị trí RA
"D" — mã hoá "Z" (chữ CUỐI bảng) dịch `3` vị trí RA chữ NÀO?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mã hoá "A" dịch `3` vị trí RA "D". Mã hoá "Z" (chữ CUỐI bảng chữ
cái) dịch `3` vị trí RA chữ NÀO — có "chạy quá" bảng chữ CÁI không,
hay MODULAR tự đưa NÓ quay LẠI đầu?
::::

::::checkpoint{mastery=0.8}
::::
