---
id: toan.do-thi-modular-dai-so-truu-tuong.dong-du-modulo
title: Đồng dư modulo n
summary: "a ≡ b (mod n) — a VÀ b CÙNG số dư khi chia cho n (bài 13); ĐÚNG một quan hệ TƯƠNG ĐƯƠNG (T2.4: phản xạ, đối xứng, bắc cầu) trên tập số nguyên."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.congruence-modulo]
requires: [math.division-with-remainder, math.equivalence-relation]
concepts: [math.dong-du]
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
Ngày thứ `10` VÀ ngày thứ `13` (chu kỳ BA ngày) — CÙNG dư `1` khi
chia CHO `3`. Chúng CÓ "CÙNG một ngày" trong chu kỳ không?
::::

::::explain{#dong-du-la-gi}
CÓ. **`a ≡ b (mod n)`** — `a` VÀ `b` CÙNG số dư khi chia CHO `n`
(bài 13). Đây LÀ một quan hệ TƯƠNG ĐƯƠNG THẬT SỰ (T2.4: phản xạ,
đối xứng, bắc cầu) — kiểm lại BẰNG chính công thức T2.4 ĐÃ viết,
GIỜ áp lên số nguyên VÀ modular:

```python title=readonly
def dong_du(a, b, n):
    return a % n == b % n


print(dong_du(10, 13, 3))
```

```text title=readonly
True
```

`10 % 3 = 1` VÀ `13 % 3 = 1` — CÙNG dư — ĐỒNG DƯ modulo `3`. Ngày
thứ `10` VÀ ngày thứ `13` rơi VÀO CÙNG một vị trí trong chu kỳ ba
ngày ("Hai").
::::

::::example{#khong-dong-du}
Ngày thứ `10` VÀ ngày thứ `11` — KHÁC dư, KHÔNG đồng dư:

```python title=readonly
def dong_du(a, b, n):
    return a % n == b % n


print(dong_du(10, 11, 3))
```

```text title=readonly
False
```

`10 % 3 = 1` NHƯNG `11 % 3 = 2` — dư KHÁC nhau — KHÔNG đồng dư. Ngày
thứ `10` LÀ "Hai", ngày thứ `11` LÀ "Tư" — hai vị trí KHÁC nhau
trong chu kỳ.
::::

::::predict{#doan-bac-cau commitOnce}
Byte ĐÃ biết `dong_du(10, 13, 3)` VÀ `dong_du(13, 16, 3)` ĐỀU
`True` (CÙNG dư `1`). Byte tính TIẾP `dong_du(10, 16, 3)` — BẮC CẦU
QUA `13`:

```python
def dong_du(a, b, n):
    return a % n == b % n

print(dong_du(10, 13, 3))
print(dong_du(13, 16, 3))
print(dong_du(10, 16, 3))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì `10` VÀ `16` cách xa NHAU tới `6` đơn vị, MỘT khoảng
cách LỚN hơn hẳn `n=3`, nên KHÔNG THỂ còn đồng dư
::why
Gần đúng ở việc bạn để ý ĐÚNG `10` VÀ `16` cách NHAU khá XA — một
quan sát VỀ khoảng cách SỐ học.

Chỗ lệch: đồng dư KHÔNG phụ thuộc khoảng CÁCH tuyệt đối, CHỈ phụ
thuộc số DƯ khi chia CHO `n` — `16 % 3 = 1`, CÙNG dư VỚI `10 % 3 =
1`. Đây chính LÀ tính BẮC CẦU (T2.4): `10≡13` VÀ `13≡16` (CÙNG
modulo `3`) buộc `10≡16` — dù `10` VÀ `16` cách nhau BAO xa, MIỄN
chênh lệch LÀ MỘT bội số của `n` (`16−10=6=2×3`).
::
:::

:::opt
Máy báo lỗi khi chạy — hàm `dong_du` được GỌI liên tiếp BA lần
TRONG cùng một khối lệnh, Python giới hạn số lần gọi MỘT hàm trong
MỘT chương trình
::why
Gần đúng ở việc bạn để ý ĐÚNG `dong_du` được GỌI tới BA lần — một
quan sát VỀ hình thức đúng.

Chỗ lệch: Python KHÔNG hề giới hạn số LẦN gọi một hàm — gọi LẶP LẠI
bao nhiêu LẦN cũng được, KHÔNG có RÀO cản nào. Biên dịch sạch, chạy
sạch.
::
:::
::::

::::code{#viet_dong_du}
Viết `dong_du(a, b, n)` — kiểm `a` VÀ `b` có ĐỒNG DƯ modulo `n`
không.

```python title=starter
def dong_du(a, b, n):
    return ___


print(dong_du(10, 13, 3))
```

```python title=solution
def dong_du(a, b, n):
    return a % n == b % n


print(dong_du(10, 13, 3))
```

```python title=test
assert dong_du(10, 13, 3) is True, "cung du 1 -- dong du"
assert dong_du(10, 11, 3) is False, "du khac nhau -- khong dong du"
assert dong_du(7, 7, 4) is True, "phan xa -- a luon dong du voi chinh no"
assert dong_du(5, 9, 4) == dong_du(9, 5, 4), "doi xung -- thu tu khong quan trong"
assert dong_du(10, 13, 3) and dong_du(13, 16, 3) and dong_du(10, 16, 3), "bac cau"
```

:::hints
- kind: attention
  body: "Tinh du cua a va du cua b khi chia cho n, roi so sanh hai du do."
- kind: strategy
  body: "a % n == b % n"
- kind: one-line
  body: "___ = a % n == b % n"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh a % n voi b % n
  requireAst:
  - kind: uses-operator, target: '%', min: 2
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-name, target: b, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đồng dư — MỘT quan hệ tương đương THẬT trên số nguyên. Cộng hai
ngày trong chu kỳ LẠI với nhau — kết quả có CÒN đồng dư theo cách dự
đoán được không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte tính ngày thứ `1000` rơi VÀO ngày NÀO (chu kỳ BA ngày) — nhân
TRỰC TIẾP một số LỚN có TIỆN bằng lấy dư TRƯỚC rồi mới nhân KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
