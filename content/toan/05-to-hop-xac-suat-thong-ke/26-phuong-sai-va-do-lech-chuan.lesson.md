---
id: toan.to-hop-xac-suat-thong-ke.phuong-sai-va-do-lech-chuan
title: Phương sai và độ lệch chuẩn
summary: "Var(X) = E[(X−E[X])²] — trung bình BÌNH PHƯƠNG khoảng cách tới kỳ vọng. σ = √Var(X) — độ lệch chuẩn, cùng đơn vị với X."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 26
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.variance]
requires: [math.linearity-of-expectation]
concepts: [math.phuong-sai, math.do-lech-chuan]
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
Kỳ vọng đo trung tâm. Nhưng hai biến ngẫu nhiên có thể CÙNG kỳ vọng
mà "trải" ra RẤT khác nhau. Đo độ "trải" đó bằng con số nào?
::::

::::explain{#phuong-sai-la-gi}
**`Var(X) = E[(X−E[X])²]`** — trung bình BÌNH PHƯƠNG khoảng cách tới
kỳ vọng (bình phương để khoảng cách ÂM không TRIỆT TIÊU khoảng cách
DƯƠNG — một khoảng cách `-2` và một khoảng cách `+2` PHẢI cùng "nặng"
như nhau). **`σ = √Var(X)`** — độ lệch chuẩn, cùng ĐƠN VỊ với `X`
(phương sai thì đơn vị bị BÌNH PHƯƠNG theo):

```python title=readonly
def ky_vong(pp):
    return sum(x * p for x, p in pp.items())

def phuong_sai(pp, ev):
    return sum(((x - ev) ** 2) * p for x, p in pp.items())


pp = {0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}
ev = ky_vong(pp)

print(ev)
print(phuong_sai(pp, ev))
```

```text title=readonly
2.0
1.0
```

`E[X]=2.0`. `Var(X) = (0-2)²×0.0625 + (1-2)²×0.25 + (2-2)²×0.375 +
(3-2)²×0.25 + (4-2)²×0.0625 = 1.0`. Giá trị CÀNG XA `2.0` (như `0`
hay `4`, cách `2`) đóng góp BÌNH PHƯƠNG khoảng cách (`4`) NHÂN với
xác suất NHỎ của nó; giá trị GẦN (`2`, cách `0`) đóng góp `0`.
::::

::::example{#do-lech-chuan-cung-don-vi}
`σ` LÀ căn bậc hai của `Var(X)` — ĐƯA đơn vị TRỞ LẠI giống `X`:

```python title=readonly
def ky_vong(pp):
    return sum(x * p for x, p in pp.items())

def phuong_sai(pp, ev):
    return sum(((x - ev) ** 2) * p for x, p in pp.items())


pp = {0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}
ev = ky_vong(pp)
var = phuong_sai(pp, ev)

print(var ** 0.5)
```

```text title=readonly
1.0
```

`σ = √1.0 = 1.0` — "hạt" (đơn vị của `X`), KHÔNG PHẢI "hạt BÌNH
PHƯƠNG" (đơn vị của `Var(X)`). Nói "độ lệch chuẩn là 1 hạt" CÓ Ý
NGHĨA thực tế; nói "phương sai là 1 hạt bình phương" thì KHÔNG.
::::

::::predict{#doan-phuong-sai-hang-so commitOnce}
Byte có một biến ngẫu nhiên CHỈ nhận ĐÚNG một giá trị (`7`, xác suất
`1.0` — KHÔNG hề "ngẫu nhiên", luôn CHẮC CHẮN):

```python
def ky_vong(pp):
    return sum(x * p for x, p in pp.items())

def phuong_sai(pp, ev):
    return sum(((x - ev) ** 2) * p for x, p in pp.items())

pp = {7: 1.0}
ev = ky_vong(pp)
print(phuong_sai(pp, ev))
```

Dòng cuối in ra gì?

:::opt{correct}
`0.0`
:::

:::opt
`7.0` — vì `Var(X)` "kế thừa" GIÁ TRỊ của `X` khi chỉ có ĐÚNG một
giá trị khả dĩ, giống hệt kỳ vọng của một giá trị chắc chắn (bài 24)
::why
Gần đúng ở việc bạn nhớ ĐÚNG `E[X]=7.0` khi `X` chắc chắn LÀ `7`
(bài 24) — sự thật đó đúng.

Chỗ lệch: `Var(X)` KHÔNG "kế thừa" giá trị của `X` — nó đo KHOẢNG
CÁCH tới kỳ vọng. Khi `X` LUÔN bằng `7` (bằng CHÍNH `E[X]=7`),
khoảng cách `X−E[X]` LUÔN LÀ `0` — bình phương của `0` VẪN LÀ `0`,
nhân với xác suất nào cũng RA `0`. `Var(X)=0.0` nghĩa LÀ `X` KHÔNG
"trải" ra chút nào — đúng nghĩa CHẮC CHẮN, không ngẫu nhiên.
::
:::

:::opt
Máy báo lỗi khi chạy — `phuong_sai` cần `pp` có ÍT NHẤT hai giá trị
KHÁC nhau để tính "độ trải", một `dict` chỉ một cặp là không đủ
::why
Gần đúng ở việc bạn nghĩ TỚI "độ trải" như một khái niệm CẦN so
sánh NHIỀU giá trị — một trực giác hợp lý VỀ Ý NGHĨA của "trải ra".

Chỗ lệch: `sum(...)` trên MỘT phần tử hoàn toàn hợp lệ VỀ mặt kỹ
thuật (đã thấy Ở bài 24) — nó CHỈ đơn giản tính RA `0` (đúng), không
báo lỗi. Không cần "ít nhất hai" giá trị.
::
:::
::::

::::code{#viet_phuong_sai}
Viết `phuong_sai(pp, ev)` — tính `Var(X) = Σ (x−ev)²·P(X=x)` từ bảng
phân phối `pp` VÀ kỳ vọng `ev` đã biết.

```python title=starter
def phuong_sai(pp, ev):
    return ___


pp = {0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}
ev = 2.0

print(phuong_sai(pp, ev))
```

```python title=solution
def phuong_sai(pp, ev):
    return sum(((x - ev) ** 2) * p for x, p in pp.items())


pp = {0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}
ev = 2.0

print(phuong_sai(pp, ev))
```

```python title=test
assert phuong_sai({7: 1.0}, 7.0) == 0.0, "gia tri chac chan -- khong trai ra chut nao"
assert phuong_sai({0: 0.5, 10: 0.5}, 5.0) == 25.0, "hai gia tri cach deu 5, moi ben dong gop 25"
assert phuong_sai({0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}, 2.0) == 1.0, "phai khop vi du chinh"
```

:::hints
- kind: attention
  body: "Dung sum() voi generator: binh phuong khoang cach (x - ev), nhan voi p, cong lai."
- kind: strategy
  body: "sum(((x - ev) ** 2) * p for x, p in pp.items())"
- kind: one-line
  body: "___ = sum(((x - ev) ** 2) * p for x, p in pp.items())"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai binh phuong khoang cach (x - ev) roi nhan voi p, cong lai bang sum()
  requireAst:
  - kind: uses-call, target: sum, min: 1
  - kind: uses-operator, target: '**', min: 1
  - kind: uses-operator, target: '-', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^1\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Track đếm, xác suất, biến ngẫu nhiên đều là LÝ THUYẾT — tính TRƯỚC
khi gieo hạt thật. Gieo THẬT sáu mùa liền thì sao?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track đếm (1-10), xác suất (11-21), biến ngẫu nhiên (22-26) đều LÀ
LÝ THUYẾT — tính TRƯỚC khi gieo hạt thật. Byte GIEO THẬT sáu mùa
liền, GHI LẠI sáu con số kilôgam thu hoạch. `tong/dem` (R1 bài 27)
đã biết tính trung bình SÁU con số ấy — nó có phải `E[X]` của bài 24
không, hay là một thứ khác?
::::

::::checkpoint{mastery=0.8}
::::
