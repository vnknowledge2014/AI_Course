---
id: toan.dstt-giai-tich-cho-ai.gradient-nguoc-huong-giam
title: Gradient ngược hướng giảm
summary: "−∇f chỉ hướng f GIẢM nhanh NHẤT — đi ngược GRADIENT LÀ cách nhanh nhất để TỚI cực tiểu; nền tảng của thuật TOÁN học máy quan trọng nhất."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 25
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.negative-gradient-direction]
requires: [math.gradient-vector]
concepts: [math.gradient-am]
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
Muốn hàm CHI PHÍ (bài 22) GIẢM nhanh NHẤT — nên đi THEO hướng
gradient, hay hướng NGƯỢC LẠI?
::::

::::explain{#gradient-am}
NGƯỢC lại. **`−∇f`** chỉ hướng `f` GIẢM nhanh NHẤT — đi ngược
GRADIENT LÀ cách nhanh nhất để TỚI cực tiểu; nền TẢNG của thuật
toán học MÁY quan trọng NHẤT:

```python title=readonly
def gradient_am(g):
    return tuple(-x for x in g)


g = (6.0, 8.0)
print(gradient_am(g))
```

```text title=readonly
(-6.0, -8.0)
```

`∇f(3,4) = (6.0, 8.0)` (bài 24) chỉ hướng `f` TĂNG nhanh NHẤT — đảo
NGƯỢC MỖI thành phần cho `−∇f = (-6.0, -8.0)`, hướng `f` GIẢM nhanh
NHẤT.
::::

::::example{#di-nguoc-gradient-giam-f}
Đi một BƯỚC NGƯỢC gradient — `f` THẬT sự GIẢM:

```python title=readonly
def f(x, y):
    return x ** 2 + y ** 2


print(f(3, 4))
print(f(3 - 0.1 * 6, 4 - 0.1 * 8))
print(f(3 + 0.1 * 6, 4 + 0.1 * 8))
```

```text title=readonly
25
16.0
36.0
```

TẠI `(3,4)`, `f=25`. Đi NGƯỢC gradient (TRỪ MỘT phần nhỏ CỦA
`(6,8)`) — `f` GIẢM xuống `16.0`. Đi THEO gradient (CỘNG THÊM) — `f`
TĂNG lên `36.0`. Đúng LỐI `−∇f` LÀ hướng GIẢM.
::::

::::predict{#doan-gradient-dau-hon-hop commitOnce}
Byte đảo NGƯỢC một gradient CÓ dấu HỖN hợp — `(-3.0, 5.0)`:

```python
def gradient_am(g):
    return tuple(-x for x in g)

print(gradient_am((-3.0, 5.0)))
```

Dòng cuối in ra gì?

:::opt{correct}
`(3.0, -5.0)`
:::

:::opt
`(-3.0, -5.0)` — vì "đảo NGƯỢC" nghĩa LÀ TRỪ CẢ hai thành phần cho
CÙNG một hằng số, LÀM cả hai đều ÂM
::why
Gần đúng ở việc bạn nghĩ TỚI "đảo ngược" NHƯ một phép DỊCH chuyển
(trừ HẰNG số) — một cách hiểu hợp LÝ về từ "đảo".

Chỗ lệch: `gradient_am` KHÔNG "trừ" gì — nó ĐỔI DẤU từng THÀNH
phần RIÊNG BIỆT (`-x` cho MỖI `x`). `-3.0` đổi dấu thành `3.0`
(dương); `5.0` đổi DẤU thành `-5.0` (âm) — MỖI thành phần đổi dấu
ĐỘC lập, KHÔNG phụ THUỘC dấu của thành phần KIA.
::
:::

:::opt
Máy báo lỗi khi chạy — `(-3.0, 5.0)` chứa MỘT thành phần ÂM VÀ một
DƯƠNG, hàm `gradient_am` chỉ được thiết KẾ cho vector TOÀN cùng dấu
::why
Gần đúng ở việc bạn để ý ĐÚNG `(-3.0, 5.0)` có dấu HỖN hợp — một
quan sát VỀ dữ liệu.

Chỗ lệch: KHÔNG hàm nào Ở đây giới hạn dấu của THÀNH phần — `-x`
hoạt động BÌNH thường VỚI cả số âm LẪN dương. Biên dịch sạch, chạy
sạch.
::
:::
::::

::::code{#viet_gradient_am}
Viết `gradient_am(g)` — đảo NGƯỢC dấu của MỖI thành phần vector `g`.

```python title=starter
def gradient_am(g):
    return ___


g = (6.0, 8.0)
print(gradient_am(g))
```

```python title=solution
def gradient_am(g):
    return tuple(-x for x in g)


g = (6.0, 8.0)
print(gradient_am(g))
```

```python title=test
assert gradient_am(()) == (), "vector rong"
assert gradient_am((6.0, 8.0)) == (-6.0, -8.0), "gradient duong"
assert gradient_am((-3.0, 5.0)) == (3.0, -5.0), "dau hon hop"
assert gradient_am((0.0, 0.0)) == (0.0, 0.0), "gradient khong -- van khong"
```

:::hints
- kind: attention
  body: "Doi dau tung thanh phan x cua g."
- kind: strategy
  body: "tuple(-x for x in g)"
- kind: one-line
  body: "___ = tuple(-x for x in g)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai doi dau tung thanh phan cua g
  requireAst:
  - kind: uses-call, target: tuple, min: 1
  - kind: uses-operator, target: neg, min: 1
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(-6\.0, -8\.0\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ngược gradient — hướng GIẢM nhanh nhất. Đi MỘT bước ngược gradient —
bước ĐÓ nên DÀI bao nhiêu?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đi một BƯỚC ngược gradient — bước ĐÓ nên DÀI bao nhiêu? Đi CẢ quãng
MỘT lần có ổn không, hay CẦN đi từng bước NHỎ?
::::

::::checkpoint{mastery=0.8}
::::
