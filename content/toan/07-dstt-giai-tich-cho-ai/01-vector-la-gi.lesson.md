---
id: toan.dstt-giai-tich-cho-ai.vector-la-gi
title: Vector là gì
summary: "Vector — một tuple số CÓ THỨ TỰ, MỖI luống giờ LÀ một điểm nhiều chiều: (dai, nuoc, nang); tổng quát hoá cặp có thứ tự (T2.4 bài 14) lên NHIỀU hơn hai thành phần."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.vector]
requires: [math.ordered-pair]
concepts: [math.vector]
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
Một luống giờ có NHIỀU con số đo được CÙNG lúc — dài `2m`, tưới `5L`
nước, nắng `6h`. Viết TẤT CẢ vào MỘT chỗ, được không?
::::

::::explain{#vector-la-gi}
ĐƯỢC. **Vector** — một `tuple` số CÓ THỨ TỰ: `(2.0, 5.0, 6.0)` nghĩa
LÀ dài `2m`, tưới `5L`, nắng `6h` — MỖI luống giờ LÀ một điểm NHIỀU
chiều, tổng quát hoá cặp có thứ tự (T2.4 bài 14, HAI thành phần) lên
`n` thành phần:

```python title=readonly
luong_1 = (2.0, 5.0, 6.0)

print(luong_1)
print(len(luong_1))
```

```text title=readonly
(2.0, 5.0, 6.0)
3
```

`luong_1` LÀ một vector BA chiều — `len()` (ĐÃ quen từ `list`/`tuple`
Ở R1) đếm SỐ thành phần, GỌI LÀ **số chiều**. Thành phần THỨ nhất
(chỉ số `0`) LÀ chiều dài, thứ HAI (chỉ số `1`) LÀ nước, thứ BA (chỉ
số `2`) LÀ nắng.
::::

::::example{#truy-cap-thanh-phan}
Truy CẬP từng thành phần bằng chỉ SỐ — ĐÚNG lối `list`/`tuple` đã
quen:

```python title=readonly
luong_1 = (2.0, 5.0, 6.0)

print(luong_1[0])
print(luong_1[1])
print(luong_1[2])
```

```text title=readonly
2.0
5.0
6.0
```

`luong_1[0]` LÀ chiều dài, `luong_1[1]` LÀ lượng nước, `luong_1[2]`
LÀ giờ nắng — VỊ TRÍ (KHÔNG phải tên) quyết định Ý nghĩa của MỖI
thành phần.
::::

::::predict{#doan-chi-so-vuot-qua commitOnce}
Byte thử truy CẬP `luong_1[3]` — thành phần THỨ tư, TRONG khi
`luong_1` chỉ CÓ ba thành phần (chỉ số `0`, `1`, `2`):

```python
luong_1 = (2.0, 5.0, 6.0)
print(luong_1[3])
```

Dòng cuối in ra gì?

:::opt{correct}
Máy báo lỗi (`IndexError`)
:::

:::opt
`0.0` — vì Python TỰ ĐỘNG điền `0.0` cho những chỉ số CÒN THIẾU
ngoài phạm vi vector
::why
Gần đúng ở việc bạn nghĩ tới một GIÁ trị "mặc định" khi chỉ số VƯỢT
quá — một trực giác quen VỚI những ngôn ngữ/công cụ khác CÓ điền
mặc định.

Chỗ lệch: `tuple` Python KHÔNG hề "điền THÊM" cho chỉ số THIẾU — nó
CHỈ có ĐÚNG `3` thành phần (chỉ số hợp LỆ: `0`, `1`, `2`), truy cập
CHỈ số `3` (KHÔNG tồn tại) LÀM Python DỪNG lại NGAY và báo lỗi
`IndexError`, KHÔNG âm thầm trả VỀ giá trị NÀO cả.
::
:::

:::opt
`6.0` — vì Python "QUAY VÒNG" chỉ số VƯỢT quá về ĐẦU vector, giống
hệt modular (T2.6) — chỉ số `3` quay VỀ chỉ số `0`... KHÔNG, VỀ chỉ
số CUỐI (`2`, giá trị `6.0`)
::why
Gần đúng ở việc bạn LIÊN hệ tới modular (T2.6, "quay VÒNG khi vượt
n") — một liên TƯỞNG thú vị GIỮA hai bài học.

Chỗ lệch: chỉ số `tuple`/`list` trong Python **KHÔNG** quay vòng —
CHỈ SỐ ÂM (`luong_1[-1]`) CÓ ý nghĩa RIÊNG (đếm từ CUỐI), NHƯNG chỉ
số DƯƠNG vượt quá độ DÀI KHÔNG hề "quay lại", nó LÀ lỗi NGAY LẬP
TỨC.
::
:::
::::

::::code{#viet_cung_so_chieu}
Viết `cung_so_chieu(u, v)` — kiểm HAI vector `u`, `v` CÓ cùng số
chiều không.

```python title=starter
def cung_so_chieu(u, v):
    return ___


luong_1 = (2.0, 5.0, 6.0)
luong_2 = (3.0, 4.0, 5.0)
print(cung_so_chieu(luong_1, luong_2))
```

```python title=solution
def cung_so_chieu(u, v):
    return len(u) == len(v)


luong_1 = (2.0, 5.0, 6.0)
luong_2 = (3.0, 4.0, 5.0)
print(cung_so_chieu(luong_1, luong_2))
```

```python title=test
assert cung_so_chieu((), ()) is True, "hai vector rong -- cung so chieu (0)"
luong_1 = (2.0, 5.0, 6.0)
luong_2 = (3.0, 4.0, 5.0)
assert cung_so_chieu(luong_1, luong_2) is True, "cung ba chieu"
luong_3 = (1.0, 2.0)
assert cung_so_chieu(luong_1, luong_3) is False, "ba chieu vs hai chieu -- khac"
assert cung_so_chieu((1.0,), (2.0,)) is True, "cung mot chieu"
```

:::hints
- kind: attention
  body: "So sanh len(u) voi len(v)."
- kind: strategy
  body: "len(u) == len(v)"
- kind: one-line
  body: "___ = len(u) == len(v)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh len(u) voi len(v)
  requireAst:
  - kind: uses-call, target: len, min: 2
  - kind: uses-operator, target: '==', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Vector — nhiều con số, MỘT chỗ. Gộp lượng nước của HAI luống lại —
cộng TỪNG cặp con số tương ứng, được không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luống 1: `(2.0, 5.0, 6.0)` (dài `2m`, tưới `5L`, nắng `6h`). Luống 2:
`(3.0, 4.0, 5.0)`. Gộp lượng nước CỦA cả hai luống LẠI — cộng TỪNG
cặp con số tương ứng, được không?
::::

::::checkpoint{mastery=0.8}
::::
