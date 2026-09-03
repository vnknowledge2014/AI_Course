---
id: toan.do-thi-modular-dai-so-truu-tuong.tinh-ket-hop
title: Tính kết hợp
summary: "(a∗b)∗c = a∗(b∗c) — nhóm NGOẶC KIỂU nào cũng ra CÙNG kết quả; + VÀ × số nguyên ĐỀU kết hợp, NHƯNG − THÌ KHÔNG (5−3−1 ≠ 5−(3−1)) — tính chất THỨ HAI của một phép toán 'tốt'."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.associativity]
requires: [math.binary-operation, math.relation-composition]
concepts: [math.tinh-ket-hop]
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
Hợp thành ánh xạ (T2.4 bài 27, `f∘g`) — ghép BA ánh xạ liên tiếp
theo HAI cách nhóm ngoặc KHÁC nhau, có RA cùng kết quả không?
::::

::::explain{#tinh-ket-hop}
CÓ. **`(a∗b)∗c = a∗(b∗c)`** — nhóm NGOẶC kiểu NÀO cũng RA cùng kết
quả. `+` VÀ `×` trên số nguyên ĐỀU kết hợp, NHƯNG `−` THÌ KHÔNG
(`(5−3)−1 ≠ 5−(3−1)`) — tính chất THỨ HAI của một phép toán "tốt":

```python title=readonly
def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)


Z3 = {0, 1, 2}
print(la_ket_hop(Z3, lambda a, b: (a + b) % 3))
```

```text title=readonly
True
```

MỌI cặp `(a,b,c)` trong `Z3` — `(a∗b)∗c` VÀ `a∗(b∗c)` LUÔN ra CÙNG
kết quả (nhóm NGOẶC bên TRÁI hay bên PHẢI trước ĐỀU không quan
trọng). Cộng modular kết HỢP.
::::

::::example{#tru-khong-ket-hop}
Trừ THÌ KHÔNG kết hợp:

```python title=readonly
def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)


so_nho = {1, 2, 3}
print(la_ket_hop(so_nho, lambda a, b: a - b))
```

```text title=readonly
False
```

`(1−2)−3 = −4`, NHƯNG `1−(2−3) = 2` — HAI cách nhóm ngoặc RA HAI kết
quả KHÁC nhau. Trừ KHÔNG kết hợp: THỨ TỰ trừ trước/SAU quan trọng.
::::

::::predict{#doan-nhan-modular-ket-hop commitOnce}
Byte kiểm tính kết hợp của NHÂN modular trên `{0,1,2,3}` (mod `4`):

```python
def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

Z4 = {0, 1, 2, 3}
print(la_ket_hop(Z4, lambda a, b: (a * b) % 4))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì trừ (bài ĐANG học) VỪA thấy KHÔNG kết hợp, VÀ nhân
CŨNG LÀ một phép toán "phức tạp" tương tự, nên NHIỀU khả năng CŨNG
không kết hợp
::why
Gần đúng ở việc bạn liên hệ TỚI ví dụ trừ VỪA thấy — một PHẢN xạ
hợp lý VỀ việc "không phải phép TOÁN nào cũng kết hợp".

Chỗ lệch: trừ KHÔNG kết hợp vì nó KHÔNG đối xứng (`a−b ≠ b−a` nói
chung) — NHÂN thì KHÁC hẳn, nhân số nguyên (VÀ nhân modular, hệ QUẢ
trực tiếp) LUÔN kết hợp, giống HỆT cộng. "Phức tạp" KHÔNG phải tiêu
chí quyết định kết hợp — CHỈ có kiểm TRỰC TIẾP (hoặc CHỨNG minh)
mới biết CHẮC.
::
:::

:::opt
Máy báo lỗi khi chạy — hàm `la_ket_hop` gọi `phep_toan` LỒNG nhau BA
lần TRONG một biểu thức, Python giới hạn ĐỘ sâu gọi hàm LỒNG
::why
Gần đúng ở việc bạn để ý ĐÚNG `phep_toan` được GỌI lồng NHIỀU lần
trong MỘT biểu thức — một quan sát VỀ hình thức đúng.

Chỗ lệch: Python KHÔNG giới hạn việc GỌI hàm lồng NHAU theo cách
THỰC tế hay gặp (VÀI lớp LÀ rất nông) — biểu thức tính ĐÚNG, KHÔNG
lỗi gì. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_ket_hop}
Viết `la_ket_hop(tap, phep_toan)` — kiểm PHÉP toán `phep_toan` có
KẾT HỢP trên tập `tap` không.

```python title=starter
def la_ket_hop(tap, phep_toan):
    return ___


Z3 = {0, 1, 2}
print(la_ket_hop(Z3, lambda a, b: (a + b) % 3))
```

```python title=solution
def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)


Z3 = {0, 1, 2}
print(la_ket_hop(Z3, lambda a, b: (a + b) % 3))
```

```python title=test
assert la_ket_hop(set(), lambda a, b: a + b) is True, "tap rong -- chan ly rong"
Z3 = {0, 1, 2}
assert la_ket_hop(Z3, lambda a, b: (a + b) % 3) is True, "cong modular ket hop"
Z4 = {0, 1, 2, 3}
assert la_ket_hop(Z4, lambda a, b: (a * b) % 4) is True, "nhan modular ket hop"
so_nho = {1, 2, 3}
assert la_ket_hop(so_nho, lambda a, b: a - b) is False, "tru khong ket hop"
```

:::hints
- kind: attention
  body: "Voi MOI bo ba (a, b, c), so sanh phep_toan(phep_toan(a,b), c) voi phep_toan(a, phep_toan(b,c))."
- kind: strategy
  body: "phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))"
- kind: one-line
  body: "all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c)) for a in tap for b in tap for c in tap)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh phep_toan((a,b),c) voi phep_toan(a,(b,c)) tren MOI bo ba
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-call, target: phep_toan, min: 4
  - kind: uses-operator, target: '==', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kết hợp — tính chất THỨ HAI. Modular NHÂN trên `{0,...,n−1}` — đơn
vị của NÓ LÀ số nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Modular NHÂN (bài 15, trên `{0,...,n−1}`) — đơn vị của NÓ LÀ số
NÀO? Kiểm LẠI: nhân số ĐÓ với BẤT KỲ phần tử nào, kết quả có GIỮ
nguyên không?
::::

::::checkpoint{mastery=0.8}
::::
