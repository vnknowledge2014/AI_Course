---
id: toan.tap-hop-quan-he-anh-xa.tap-hop-rong
title: Tập hợp rỗng
summary: "`∅` (Python `set()`, KHÔNG PHẢI `{}` — `{}` là dict rỗng, một bẫy cú pháp thật) — tập hợp không có phần tử nào VẪN là một tập hợp hợp lệ, không phải lỗi hay chưa có gì."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [math.set-empty]
requires: [math.set-builder, core.dict, ctrl.for-each]
concepts: [math.tap-rong, math.bay-ngoac-nhon-rong]
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
Luống 5 chưa gieo gì cả. Tập hợp loại rau của nó — có phải một lỗi, hay
vẫn là một tập hợp đàng hoàng?
::::

::::explain{#tap-rong-la-gi}
Vẫn đàng hoàng. **`∅`** (ký hiệu tập rỗng) — tập hợp KHÔNG có phần tử
nào — là một tập hợp HỢP LỆ y hệt bất kỳ tập hợp nào khác, không phải
"lỗi" hay "chưa xác định". Luống chưa gieo có tập loại-rau đúng là `∅`.

Python viết `∅` bằng `set()` — **KHÔNG PHẢI** `{}`. Đây là một bẫy cú
pháp CÓ THẬT:

```python title=readonly
o_rong = {}
tap_rong = set()

print(type(o_rong))
print(type(tap_rong))
```

```text title=readonly
<class 'dict'>
<class 'set'>
```

`{}` (ngoặc nhọn TRẦN, không gì bên trong) LUÔN LÀ một `dict` rỗng —
KHÔNG phải tập hợp rỗng. `{1, 2, 3}` (ngoặc nhọn CÓ phần tử) là `set`,
nhưng bỏ hết phần tử đi thì Python phải chọn MỘT nghĩa cho `{}` trơ trọi,
và nó chọn `dict` (lý do lịch sử: `dict` xuất hiện trước `set` trong
Python). Muốn `∅` thật, phải gọi `set()`.
::::

::::example{#luong-rong-van-la-tap-hop}
Luống chưa gieo có tập rỗng — `len` đếm ra `0`, không phải lỗi:

```python title=readonly
luong_5 = set()
luong_6 = {"bí đỏ"}

print(luong_5 == set())
print(len(luong_5))
print(len(luong_6))
```

```text title=readonly
True
0
1
```

`luong_5` là tập rỗng THẬT SỰ (so `==` với `set()` khớp), `len` đếm ra
`0` — KHÔNG có ngoại lệ nào bị ném ra, KHÔNG có thông báo lỗi nào. Một
luống trống là một luống có tập hợp `0` phần tử, đơn giản vậy thôi.
::::

::::predict{#doan-bay-ngoac-nhon commitOnce}
Byte gõ nhanh tay, viết `{}` thay vì `set()`:

```python
o_trong = {}

print(type(o_trong) == set)
print(type(o_trong) == dict)
```

Hai dòng cuối in ra gì?

:::opt{correct}
`False`, rồi `True`
:::

:::opt
`True`, rồi `False` — vì `{}` là cách viết TRỐNG của dấu ngoặc nhọn, và
`set` CŨNG dùng dấu ngoặc nhọn (`{1, 2, 3}`), nên `{}` phải mặc định LÀ
một `set` rỗng, không phải `dict`
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng `set` dùng dấu ngoặc nhọn — `{1, 2,
3}` đúng thật là một `set`, quan sát ĐÓ chính xác.

Chỗ lệch: quy tắc ĐÓ chỉ áp dụng khi ngoặc nhọn CÓ phần tử bên trong.
Khi ngoặc nhọn TRỐNG TRƠN, Python KHÔNG có manh mối nào để biết bạn
muốn `set` hay `dict` — và nó chọn `dict` (đúng chọn LỊCH SỬ: `dict`
ra đời trước `set` trong Python, `{}` đã "thuộc về" `dict` từ trước khi
`set` tồn tại). `type(o_trong)` là `dict`, KHÔNG phải `set`.
::
:::

:::opt
Máy báo lỗi biên dịch — gán một `dict` rỗng (`{}`) cho biến `o_trong`
rồi đem so `type()` với `set` VÀ `dict` liên tiếp là một thao tác không
hợp lệ, Python cấm so sánh kiểu của MỘT biến với HAI kiểu khác nhau
trong CÙNG một đoạn code
::why
Gần đúng ở việc bạn để ý có HAI phép so sánh `type()` liên tiếp trên
CÙNG một biến — một quan sát đúng về CẤU TRÚC đoạn code.

Chỗ lệch: Python không hề giới hạn số lần bạn được so sánh `type()` của
một biến — gọi bao nhiêu lần, với bao nhiêu kiểu khác nhau, đều hợp lệ.
Biên dịch sạch, chạy sạch — mỗi dòng chỉ đơn giản trả về `True` hoặc
`False` theo đúng kiểu THẬT của `o_trong`.
::
:::
::::

::::code{#viet_nhung_luong_rong}
Viết `nhung_luong_rong(vuon)` — trả về danh sách tên những luống có tập
hợp loại rau LÀ TẬP RỖNG (chưa gieo gì).

```python title=starter
def nhung_luong_rong(vuon):
    ket_qua = []
    for ten_luong in vuon:
        if ___:
            ket_qua.append(ten_luong)
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_5": set(),
    "luong_6": {"bí đỏ"},
    "luong_7": set(),
}

print(nhung_luong_rong(vuon))
```

```python title=solution
def nhung_luong_rong(vuon):
    ket_qua = []
    for ten_luong in vuon:
        if vuon[ten_luong] == set():
            ket_qua.append(ten_luong)
    return ket_qua


vuon = {
    "luong_1": {"cà chua", "xà lách"},
    "luong_5": set(),
    "luong_6": {"bí đỏ"},
    "luong_7": set(),
}

print(nhung_luong_rong(vuon))
```

```python title=test
assert nhung_luong_rong({}) == [], "vuon rong -- khong luong nao de xet"
assert nhung_luong_rong({"luong_1": {"a"}}) == [], "luong co rau -- khong rong"
assert nhung_luong_rong({"luong_1": set()}) == ["luong_1"], "mot luong rong duy nhat"
assert nhung_luong_rong({"a": set(), "b": set(), "c": {"x"}}) == ["a", "b"], "hai luong rong, mot luong co rau"
```

:::hints
- kind: attention
  body: "Dieu kien if phai kiem tap hop cua luong (vuon[ten_luong]) co BANG set() khong -- dung ==, khong dung {}."
- kind: strategy
  body: "vuon[ten_luong] == set()"
- kind: one-line
  body: "___ = vuon[ten_luong] == set()"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dieu kien phai so sanh tap hop cua tung luong (vuon[ten_luong]) voi set() bang toan tu == -- dung {} se so sanh voi dict rong chu khong phai tap rong
  requireAst:
  - kind: uses-call, target: set, min: 1
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-name, target: ten_luong, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\['luong_5', 'luong_7'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`∅` là `set()`, không phải `{}` — một bẫy nhỏ, ghi nhớ chắc. Bài sau:
so sánh HAI tập hợp KHÔNG bằng nhau — tập nào "nằm trong" tập nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luống 5 chưa gieo gì — tập của nó là `∅`. Luống 6 trồng đúng "bí đỏ".
Tập nào "nhỏ hơn" tập kia — và "nhỏ hơn" nghĩa là gì cho tập hợp, khi
tập hợp không phải một CON SỐ?
::::

::::checkpoint{mastery=0.8}
::::
