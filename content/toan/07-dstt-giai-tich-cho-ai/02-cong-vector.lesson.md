---
id: toan.dstt-giai-tich-cho-ai.cong-vector
title: Cộng vector
summary: "u + v — cộng TỪNG thành phần TƯƠNG ỨNG: (a1,a2)+(b1,b2)=(a1+b1,a2+b2); gộp hai đợt tưới LÀ cộng vector."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.vector-addition]
requires: [math.vector]
concepts: [math.cong-vector]
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
Luống 1: `(2.0, 5.0, 6.0)`. Luống 2: `(3.0, 4.0, 5.0)`. Gộp lượng
nước CỦA cả hai luống LẠI — cộng TỪNG cặp con số tương ứng, được
không?
::::

::::explain{#cong-vector}
ĐƯỢC. **`u + v`** — cộng TỪNG thành phần TƯƠNG ỨNG: `(a1,a2)+(b1,
b2)=(a1+b1, a2+b2)`; gộp hai đợt tưới LÀ cộng vector, TỪNG chỉ số
CỘNG RIÊNG với chỉ SỐ cùng vị trí:

```python title=readonly
def cong_vector(u, v):
    return tuple(a + b for a, b in zip(u, v))


luong_1 = (2.0, 5.0, 6.0)
luong_2 = (3.0, 4.0, 5.0)

print(cong_vector(luong_1, luong_2))
```

```text title=readonly
(5.0, 9.0, 11.0)
```

`2.0+3.0=5.0` (chiều dài), `5.0+4.0=9.0` (nước), `6.0+5.0=11.0`
(nắng) — MỖI thành phần cộng VỚI thành phần CÙNG vị trí, KHÔNG cộng
LẪN lộn giữa các chỉ số KHÁC nhau.
::::

::::example{#vector-rong}
Vector RỖNG cộng vector RỖNG — RA vector rỗng:

```python title=readonly
def cong_vector(u, v):
    return tuple(a + b for a, b in zip(u, v))


print(cong_vector((), ()))
```

```text title=readonly
()
```

KHÔNG thành phần nào để cộng — kết quả LÀ tuple RỖNG, đúng LỐI chân
LÝ rỗng (T2.3, T2.4) ĐÃ gặp nhiều lần: KHÔNG có gì để LÀM SAI thì
KHÔNG SAI.
::::

::::predict{#doan-khac-so-chieu commitOnce}
Byte cộng HAI vector KHÁC số chiều — `(1.0, 2.0, 3.0)` (BA chiều)
VÀ `(10.0, 20.0)` (HAI chiều):

```python
def cong_vector(u, v):
    return tuple(a + b for a, b in zip(u, v))

print(cong_vector((1.0, 2.0, 3.0), (10.0, 20.0)))
```

Dòng cuối in ra gì?

:::opt{correct}
`(11.0, 22.0)`
:::

:::opt
Máy báo lỗi khi chạy — hai vector KHÁC số chiều (BA VÀ hai) KHÔNG
thể cộng, `zip` sẽ TỪ chối ghép hai dãy có ĐỘ dài khác nhau
::why
Gần đúng ở việc bạn NGHĨ hai vector khác chiều "KHÔNG hợp lệ" để
cộng — VỀ mặt TOÁN học, đây LÀ một quan sát ĐÚNG (cộng vector chỉ
định NGHĨA khi cùng số chiều).

Chỗ lệch: `zip(u, v)` KHÔNG báo lỗi khi hai dãy khác ĐỘ dài — nó CHỈ
GHÉP tới khi dãy NGẮN hơn HẾT rồi DỪNG lặng lẽ, BỎ QUA phần THỪA của
dãy dài hơn. `zip((1,2,3),(10,20))` chỉ tạo RA hai cặp (`1↔10`,
`2↔20`), phần tử `3` bị LỜ đi HOÀN toàn, KHÔNG có cảnh BÁO nào.
::
:::

:::opt
`(11.0, 22.0, 3.0)` — vì `zip` GHÉP hết những gì CÓ thể, RỒI giữ
NGUYÊN phần tử THỪA của vector DÀI hơn Ở CUỐI kết quả
::why
Gần đúng ở việc bạn nghĩ Python "GIỮ LẠI" phần DƯ thay VÌ bỏ hẳn —
một cách xử LÝ hợp lý, GIỐNG một số ngôn NGỮ/hàm khác.

Chỗ lệch: `zip` Python KHÔNG "giữ LẠI" gì cả — nó DỪNG NGAY khi dãy
ngắn HƠN hết, phần tử `3.0` (thừa ra Ở vector đầu) BỊ BỎ HOÀN TOÀN,
KHÔNG xuất hiện Ở BẤT KỲ đâu trong kết quả. Kết quả CHỈ có đúng HAI
thành phần, không phải BA.
::
:::
::::

::::code{#viet_cong_vector}
Viết `cong_vector(u, v)` — cộng HAI vector CÙNG số chiều, TỪNG thành
phần TƯƠNG ứng.

```python title=starter
def cong_vector(u, v):
    return ___


luong_1 = (2.0, 5.0, 6.0)
luong_2 = (3.0, 4.0, 5.0)
print(cong_vector(luong_1, luong_2))
```

```python title=solution
def cong_vector(u, v):
    return tuple(a + b for a, b in zip(u, v))


luong_1 = (2.0, 5.0, 6.0)
luong_2 = (3.0, 4.0, 5.0)
print(cong_vector(luong_1, luong_2))
```

```python title=test
assert cong_vector((), ()) == (), "hai vector rong -- ket qua rong"
luong_1 = (2.0, 5.0, 6.0)
luong_2 = (3.0, 4.0, 5.0)
assert cong_vector(luong_1, luong_2) == (5.0, 9.0, 11.0), "cong tung thanh phan"
assert cong_vector((1.0, 2.0), (10.0, 20.0)) == (11.0, 22.0), "vector hai chieu"
assert cong_vector((0.0, 0.0, 0.0), (1.0, 2.0, 3.0)) == (1.0, 2.0, 3.0), "cong voi vector khong -- khong doi"
```

:::hints
- kind: attention
  body: "Dung zip(u, v) de ghep tung cap thanh phan tuong ung, roi cong tung cap."
- kind: strategy
  body: "tuple(a + b for a, b in zip(u, v))"
- kind: one-line
  body: "___ = tuple(a + b for a, b in zip(u, v))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung zip(u, v) va cong tung cap thanh phan
  requireAst:
  - kind: uses-call, target: tuple, min: 1
  - kind: uses-call, target: zip, min: 1
  - kind: uses-operator, target: '+', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(5\.0, 9\.0, 11\.0\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cộng vector — từng thành phần một. Nhân MỖI thành phần lên GẤP ĐÔI
(tăng khẩu phần tưới gấp đôi) — có phải phép TOÁN khác cộng vector
không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nhân MỖI thành phần của vector lên GẤP ĐÔI (tăng khẩu phần tưới gấp
đôi) — có phải phép TOÁN khác cộng vector không?
::::

::::checkpoint{mastery=0.8}
::::
