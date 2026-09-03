---
id: toan.dstt-giai-tich-cho-ai.tich-vo-huong
title: Tích vô hướng
summary: "u·v = u1v1+u2v2+... — tổng CÁC tích từng cặp thành phần TƯƠNG ứng; MỘT con số DUY NHẤT tóm tắt 'hai vector khớp NHAU bao nhiêu'."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.dot-product]
requires: [math.vector-distance]
concepts: [math.tich-vo-huong]
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
Có cách nào đo "GIỐNG nhau" mà KHÔNG quan tâm ĐỘ LỚN tuyệt đối, chỉ
quan tâm HƯỚNG?
::::

::::explain{#tich-vo-huong}
CÓ. **`u·v = u1v1+u2v2+...`** — tổng CÁC tích từng cặp thành phần
TƯƠNG ứng; MỘT con số DUY NHẤT tóm TẮT "hai vector khớp NHAU bao
nhiêu":

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))


print(tich_vo_huong((2.0, 3.0), (4.0, 5.0)))
```

```text title=readonly
23.0
```

`2×4 + 3×5 = 8+15 = 23` — MỖI cặp thành phần TƯƠNG ứng NHÂN riêng,
RỒI cộng TẤT CẢ lại thành MỘT con số DUY nhất.
::::

::::example{#tich-vo-huong-voi-chinh-no}
Tích vô hướng của MỘT vector VỚI chính NÓ — LIÊN quan trực TIẾP tới
độ dài (bài 4):

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))


v = (3.0, 4.0)
print(tich_vo_huong(v, v))
```

```text title=readonly
25.0
```

`3×3+4×4=9+16=25` — CHÍNH LÀ bình phương độ dài (`‖v‖²`, bài 4:
`‖(3,4)‖=5`, `5²=25`). Tích vô hướng của một vector VỚI chính nó
LUÔN LÀ bình phương độ dài của NÓ.
::::

::::predict{#doan-vuong-goc commitOnce}
Byte tính tích vô hướng của `(2.0, 3.0)` VÀ `(-3.0, 2.0)` — HAI
vector NÀY VUÔNG góc NHAU (xoay `90°`):

```python
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

print(tich_vo_huong((2.0, 3.0), (-3.0, 2.0)))
```

Dòng cuối in ra gì?

:::opt{correct}
`0.0`
:::

:::opt
`-6.0` — vì `2×(-3)=-6`, VÀ số HẠNG thứ hai (`3×2=6`) KHÔNG đủ LỚN
để "huỷ" hoàn TOÀN số hạng ÂM đầu tiên
::why
Gần đúng ở việc bạn TÍNH đúng số hạng đầu (`2×(-3)=-6`) — một phép
NHÂN chính xác.

Chỗ lệch: `tich_vo_huong` CỘNG **CẢ HAI** số hạng, KHÔNG dừng Ở số
hạng ĐẦU — số hạng đầu LÀ `-6` (`2×(-3)`), số hạng SAU LÀ `6` (`3×2`),
CỘNG lại đúng `0`. Số hạng THỨ hai ĐỦ để "huỷ" CHÍNH XÁC số hạng đầu,
KHÔNG phải "không đủ" — kết quả LÀ `0`, đúng ĐẶC trưng của hai vector
VUÔNG góc.
::
:::

:::opt
Máy báo lỗi khi chạy — `(-3.0, 2.0)` CÓ một thành phần ÂM, mà
`tich_vo_huong` chỉ được THIẾT kế cho vector TOÀN số KHÔNG âm
::why
Gần đúng ở việc bạn để ý ĐÚNG `-3.0` LÀ số âm — một quan sát VỀ dấu.

Chỗ lệch: KHÔNG hàm nào Ở đây giới hạn dấu của THÀNH phần — phép
NHÂN VÀ cộng HOẠT động bình thường VỚI số âm. Biên dịch sạch, chạy
sạch.
::
:::
::::

::::code{#viet_tich_vo_huong}
Viết `tich_vo_huong(u, v)` — tính tích vô hướng của HAI vector.

```python title=starter
def tich_vo_huong(u, v):
    return ___


print(tich_vo_huong((2.0, 3.0), (4.0, 5.0)))
```

```python title=solution
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))


print(tich_vo_huong((2.0, 3.0), (4.0, 5.0)))
```

```python title=test
assert tich_vo_huong((), ()) == 0, "hai vector rong -- tich 0"
assert tich_vo_huong((2.0, 3.0), (4.0, 5.0)) == 23.0, "vi du chinh"
assert tich_vo_huong((4.0, 5.0), (2.0, 3.0)) == 23.0, "giao hoan -- doi thu tu van bang"
assert tich_vo_huong((1.0, 0.0), (0.0, 1.0)) == 0.0, "vuong goc don vi -- tich 0"
v = (3.0, 4.0)
assert tich_vo_huong(v, v) == 25.0, "tich voi chinh no -- binh phuong do dai"
```

:::hints
- kind: attention
  body: "Nhan tung cap thanh phan tuong ung (dung zip), roi cong tat ca lai."
- kind: strategy
  body: "sum(a * b for a, b in zip(u, v))"
- kind: one-line
  body: "___ = sum(a * b for a, b in zip(u, v))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai nhan tung cap thanh phan (zip) roi cong lai (sum)
  requireAst:
  - kind: uses-call, target: sum, min: 1
  - kind: uses-call, target: zip, min: 1
  - kind: uses-operator, target: '*', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^23\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tích vô hướng — MỘT con số tóm tắt "khớp nhau bao nhiêu". `u·v` VÀ
`v·u` — có bằng NHAU không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`u·v` VÀ `v·u` — có bằng NHAU không (giống hệt kiểm tra Ở T2.6, phép
TOÁN nào giao hoán)? Con số `u·v` LỚN nghĩa LÀ gì VỀ mặt HÌNH học?
::::

::::checkpoint{mastery=0.8}
::::
