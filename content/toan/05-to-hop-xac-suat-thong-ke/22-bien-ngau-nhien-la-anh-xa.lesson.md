---
id: toan.to-hop-xac-suat-thong-ke.bien-ngau-nhien-la-anh-xa
title: Biến ngẫu nhiên là một ánh xạ
summary: "Biến ngẫu nhiên X — một ánh xạ (T2.4 bài 24) từ không gian mẫu Ω sang một tập SỐ. \"Số hạt nảy mầm trong 4 hạt gieo\" là ánh xạ từ MỖI kết quả có thể sang một số nguyên từ 0 đến 4."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.random-variable]
requires: [math.bayes-theorem]
concepts: [math.bien-ngau-nhien]
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
Byte gieo bốn hạt, mỗi hạt nảy mầm hay không. Byte muốn hỏi "TRUNG
BÌNH mấy hạt nảy", không phải "hạt NÀY nảy hay không". Câu hỏi mới
cần một CÔNG CỤ gì?
::::

::::explain{#bien-ngau-nhien-la-gi}
Cần **biến ngẫu nhiên**. **Biến ngẫu nhiên `X`** — một ÁNH XẠ (T2.4
bài 24) từ không gian mẫu `Ω` sang MỘT tập SỐ. "Số hạt nảy mầm trong
4 hạt gieo" LÀ ánh xạ từ MỖI kết quả có thể (tổ hợp nảy/không nảy
của 4 hạt) sang một số nguyên TỪ `0` đến `4`:

```python title=readonly
def X(ket_qua):
    return sum(ket_qua)


omega = {(a, b, c, d) for a in [True, False] for b in [True, False] for c in [True, False] for d in [True, False]}

print(len(omega))
print(X((True, True, False, True)))
```

```text title=readonly
16
3
```

`omega` LÀ 16 kết quả có thể (T1.2, T2.4 bài 10: `2⁴=16`) — mỗi kết
quả MỘT dãy `True`/`False` (hạt 1 nảy? hạt 2 nảy?...). `X` NHẬN một
kết quả (một dãy CỤ THỂ), TRẢ VỀ số `True` trong đó (`sum` một dãy
`True`/`False` đếm số `True`, vì `True` được tính LÀ `1`).
::::

::::example{#nhieu-ket-qua-mot-gia-tri}
Ánh xạ `X` KHÔNG cần đơn ánh (T2.4 bài 25) — NHIỀU kết quả KHÁC nhau
có thể cùng gán RA một giá trị:

```python title=readonly
def X(ket_qua):
    return sum(ket_qua)


ket_qua_1 = (True, True, False, False)
ket_qua_2 = (False, False, True, True)

print(ket_qua_1 == ket_qua_2)
print(X(ket_qua_1))
print(X(ket_qua_2))
```

```text title=readonly
False
2
2
```

`ket_qua_1` VÀ `ket_qua_2` LÀ hai kết quả HOÀN TOÀN khác nhau (hạt
nào nảy KHÁC hẳn) — dòng đầu `False`. NHƯNG cả hai CÙNG cho `X=2`
(đều có ĐÚNG hai hạt nảy). `X` gộp NHIỀU kết quả có CÙNG "số lượng"
lại làm MỘT giá trị.
::::

::::predict{#doan-x-khong-don-anh commitOnce}
Byte đếm xem CÓ BAO NHIÊU kết quả (trong 16 kết quả) cùng cho
`X=2`:

```python
def X(ket_qua):
    return sum(ket_qua)

omega = {(a, b, c, d) for a in [True, False] for b in [True, False] for c in [True, False] for d in [True, False]}

so_ket_qua_bang_2 = sum(1 for ket in omega if X(ket) == 2)
print(so_ket_qua_bang_2)
```

Dòng cuối in ra gì?

:::opt{correct}
`6`
:::

:::opt
`1` — vì mỗi giá trị `X` (`0`, `1`, `2`, `3`, `4`) ứng với ĐÚNG một
kết quả DUY NHẤT, giống hệt một ánh xạ đơn ánh (T2.4 bài 25)
::why
Gần đúng ở việc bạn hình dung `X` như MỘT ánh xạ đơn ánh (mỗi giá
trị output ứng ĐÚNG một input) — một trực giác hợp lý nếu chưa thấy
ví dụ Ở TRÊN.

Chỗ lệch: bài này (Ở phần "example" phía TRÊN) ĐÃ chỉ ra CỤ THỂ hai
kết quả `(True,True,False,False)` VÀ `(False,False,True,True)` KHÁC
nhau nhưng CÙNG cho `X=2` — `X` KHÔNG đơn ánh. Có ĐÚNG `C(4,2)=6`
(bài 7, tổ hợp) cách CHỌN hai VỊ TRÍ trong bốn để đặt `True` — SÁU
kết quả khác nhau, không phải một.
::
:::

:::opt
Máy báo lỗi khi chạy — `omega` chứa TUPLE, không thể dùng làm phần
tử của một biểu thức đếm `sum(1 for ...)`
::why
Gần đúng ở việc bạn để ý `omega` chứa các TUPLE (không phải số hay
chuỗi đơn) — một quan sát VỀ kiểu dữ liệu.

Chỗ lệch: `tuple` HOÀN TOÀN hợp lệ làm phần tử của `set` (đã dùng từ
R1.T1.4) VÀ hoàn toàn dùng được trong vòng lặp `for ket in omega`
hay generator `sum(1 for ...)`. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_X}
Viết `X(ket_qua)` — trả về số lượng hạt nảy mầm (`True`) trong
`ket_qua`.

```python title=starter
def X(ket_qua):
    return ___


print(X((True, True, False, True)))
```

```python title=solution
def X(ket_qua):
    return sum(ket_qua)


print(X((True, True, False, True)))
```

```python title=test
assert X((False, False, False, False)) == 0, "khong hat nao nay -- X=0"
assert X((True, True, True, True)) == 4, "ca bon hat nay -- X=4"
assert X((True, False, False, False)) == 1, "dung mot hat nay"
assert X((True, True, False, False)) == X((False, False, True, True)), "hai ket qua khac nhau, cung X"
```

:::hints
- kind: attention
  body: "sum() tren mot day True/False dem so luong True (vi True duoc tinh la 1)."
- kind: strategy
  body: "sum(ket_qua)"
- kind: one-line
  body: "___ = sum(ket_qua)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung sum() tren ket_qua de dem so luong True
  requireAst:
  - kind: uses-call, target: sum, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^3\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`X` gộp 16 kết quả thành 5 giá trị (`0` tới `4`). Mỗi giá trị "phổ
biến" cỡ nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`X` gán MỖI kết quả trong `Ω` một số. NHIỀU kết quả khác nhau trong
`Ω` có thể cùng gán ra MỘT giá trị của `X` (vừa thấy: sáu kết quả
cùng cho `X=2`). Có cách nào TÓM TẮT — với MỖI giá trị `X` có thể
nhận, bao nhiêu PHẦN TRĂM kết quả rơi vào đó — thành MỘT bảng không?
::::

::::checkpoint{mastery=0.8}
::::
