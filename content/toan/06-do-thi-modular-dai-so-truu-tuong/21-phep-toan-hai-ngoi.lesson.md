---
id: toan.do-thi-modular-dai-so-truu-tuong.phep-toan-hai-ngoi
title: Phép toán hai ngôi
summary: "Phép toán hai ngôi ∗ trên tập S — nhận HAI phần tử của S, TRẢ VỀ một phần tử CỦA S (TÍNH ĐÓNG — kết quả KHÔNG 'thoát ra ngoài' S); + trên số nguyên ĐÓNG, NHƯNG ÷ trên số nguyên KHÔNG đóng."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.binary-operation]
requires: [math.generalized-modular-arithmetic]
concepts: [math.tinh-dong]
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
Modular cộng (bài 15) trên tập `{0,1,...,n−1}` — kết quả CÓ LUÔN nằm
TRONG tập đó không, hay CÓ thể "chạy RA ngoài"?
::::

::::explain{#phep-toan-hai-ngoi}
LUÔN nằm trong — VÀ đây LÀ tính chất ĐẦU tiên của một phép toán
"tốt". **Phép toán hai ngôi `∗`** trên tập `S` — nhận HAI phần tử của
`S`, TRẢ VỀ một phần tử CỦA `S` (**tính ĐÓNG** — kết quả KHÔNG
"thoát ra ngoài" `S`). `+` trên số nguyên ĐÓNG, NHƯNG `÷` trên số
nguyên KHÔNG đóng (`7÷2` không phải số nguyên):

```python title=readonly
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)


Z3 = {0, 1, 2}
print(la_dong(Z3, lambda a, b: (a + b) % 3))
```

```text title=readonly
True
```

`Z3 = {0,1,2}` — MỌI cặp `(a,b)` trong `Z3`, kết quả `(a+b) mod 3`
LUÔN nằm TRONG `Z3` (đúng ĐỊNH nghĩa của `%`, bài 13). Cộng modular
ĐÓNG trên tập `{0,...,n−1}`.
::::

::::example{#chia-khong-dong}
Chia THƯỜNG trên `{1,2,3}` — KHÔNG đóng:

```python title=readonly
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)


so_nho = {1, 2, 3}
print(la_dong(so_nho, lambda a, b: a / b))
```

```text title=readonly
False
```

`1÷2 = 0.5`, KHÔNG nằm TRONG `{1,2,3}` — chia THƯỜNG trên tập số
nguyên KHÔNG đóng (kết quả "thoát RA" tập số nguyên, RA số THẬP
phân).
::::

::::predict{#doan-cong-thuong-tren-tap-nho commitOnce}
Byte kiểm phép CỘNG THƯỜNG (KHÔNG phải modular) trên CÙNG tập
`{1,2,3}`:

```python
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

so_nho = {1, 2, 3}
print(la_dong(so_nho, lambda a, b: a + b))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì phép CỘNG THƯỜNG ĐÃ chứng minh ĐÓNG trên số nguyên (T2.1),
VÀ `{1,2,3}` LÀ một tập TOÀN số nguyên
::why
Gần đúng ở việc bạn nhớ ĐÚNG: cộng trên TOÀN BỘ tập số nguyên LÀ
đóng — một sự thật TOÁN học chính xác.

Chỗ lệch: tính ĐÓNG LÀ tính chất của MỘT CẶP `(tập, phép toán)`
CÙNG nhau, KHÔNG PHẢI của riêng phép TOÁN. Cộng ĐÓNG trên TOÀN bộ số
nguyên (tập VÔ hạn), NHƯNG trên tập CON hữu hạn `{1,2,3}` thì
KHÔNG: `1+3=4`, KHÔNG nằm TRONG `{1,2,3}`. Cùng phép CỘNG, nhưng đổi
TẬP thì đổi tính ĐÓNG — modular cộng ĐÓNG trên `{0,...,n−1}` (bài
21's ví dụ chính) CHÍNH VÌ `%` luôn ÉP kết quả VỀ lại trong TẬP đó.
::
:::

:::opt
Máy báo lỗi khi chạy — `lambda a, b: a + b` KHÔNG có TÊN, mà hàm
`la_dong` cần GỌI một hàm CÓ tên qua tham số `phep_toan`
::why
Gần đúng ở việc bạn để ý ĐÚNG `lambda` tạo ra hàm KHÔNG có tên — một
quan sát VỀ hình thức đúng.

Chỗ lệch: Python HOÀN TOÀN cho phép TRUYỀN một hàm KHÔNG tên (lambda)
làm ĐỐI số — `la_dong` chỉ cần MỘT thứ GỌI ĐƯỢC (`phep_toan(a, b)`),
KHÔNG quan tâm nó CÓ tên hay không. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_dong}
Viết `la_dong(tap, phep_toan)` — kiểm PHÉP toán `phep_toan` có ĐÓNG
trên tập `tap` không (MỌI cặp, kết quả LUÔN nằm TRONG `tap`).

```python title=starter
def la_dong(tap, phep_toan):
    return ___


Z3 = {0, 1, 2}
print(la_dong(Z3, lambda a, b: (a + b) % 3))
```

```python title=solution
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)


Z3 = {0, 1, 2}
print(la_dong(Z3, lambda a, b: (a + b) % 3))
```

```python title=test
assert la_dong(set(), lambda a, b: a + b) is True, "tap rong -- chan ly rong"
Z3 = {0, 1, 2}
assert la_dong(Z3, lambda a, b: (a + b) % 3) is True, "cong modular dong"
Z4 = {0, 1, 2, 3}
assert la_dong(Z4, lambda a, b: (a * b) % 4) is True, "nhan modular dong"
so_nho = {1, 2, 3}
assert la_dong(so_nho, lambda a, b: a / b) is False, "chia thuong khong dong"
assert la_dong(so_nho, lambda a, b: a + b) is False, "cong thuong tren tap nho khong dong"
```

:::hints
- kind: attention
  body: "Voi MOI cap (a, b) trong tap, ket qua phep_toan(a, b) phai nam trong tap -- dung all va in."
- kind: strategy
  body: "all(phep_toan(a, b) in tap for a in tap for b in tap)"
- kind: one-line
  body: "___ = all(phep_toan(a, b) in tap for a in tap for b in tap)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem MOI cap (a,b) trong tap, ket qua phep_toan(a,b) co nam trong tap khong
  requireAst:
  - kind: uses-call, target: all, min: 1
  - kind: uses-operator, target: 'in', min: 1
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tính đóng — tính chất ĐẦU tiên. Ghép BA ánh xạ liên tiếp theo hai
cách NHÓM ngoặc khác nhau — có RA cùng kết quả không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hợp thành ánh xạ (T2.4 bài 27, `f∘g`) — ghép BA ánh xạ liên tiếp
theo HAI cách nhóm ngoặc KHÁC nhau, có RA cùng kết quả không?
::::

::::checkpoint{mastery=0.8}
::::
