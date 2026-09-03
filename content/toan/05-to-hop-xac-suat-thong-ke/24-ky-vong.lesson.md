---
id: toan.to-hop-xac-suat-thong-ke.ky-vong
title: Kỳ vọng
summary: "E[X] = Σ x · P(X=x) — trung bình CÓ TRỌNG SỐ theo xác suất, không phải trung bình thường của các giá trị x."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.expectation]
requires: [math.probability-distribution]
concepts: [math.ky-vong]
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
Bảng phân phối có năm dòng, KHÔNG đều xác suất. Cộng thẳng năm giá
trị `X` rồi chia 5 — có đúng "trung bình" không?
::::

::::explain{#ky-vong-la-gi}
Không đúng — cần TRỌNG SỐ. **`E[X] = Σ x · P(X=x)`** — trung bình CÓ
TRỌNG SỐ theo xác suất, KHÔNG phải trung bình THƯỜNG của các giá trị
`x`; giá trị `x` càng có xác suất CAO càng "kéo" `E[X]` VỀ phía nó:

```python title=readonly
def ky_vong(pp):
    return sum(x * p for x, p in pp.items())


pp = {0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}

print(ky_vong(pp))
```

```text title=readonly
2.0
```

`E[X]=0×(1/16)+1×(4/16)+2×(6/16)+3×(4/16)+4×(1/16)=2.0`. Cộng THƯỜNG năm
giá trị (`0+1+2+3+4=10`) rồi chia `5` CŨNG ra `2.0` — TRÙNG hợp vì
bảng này ĐỐI XỨNG (bài 23); NÓI CHUNG hai cách KHÔNG trùng nhau (bài
27 sẽ chỉ ra một ca lệch hẳn).
::::

::::example{#trong-so-keo-lech}
Khi phân phối KHÔNG đối xứng — hai cách (trọng số VÀ trung bình
thường) cho HAI con số khác nhau:

```python title=readonly
def ky_vong(pp):
    return sum(x * p for x, p in pp.items())


pp_lech = {0: 0.5, 10: 0.5}

print(ky_vong(pp_lech))
print((0 + 10) / 2)
```

```text title=readonly
5.0
5.0
```

(Vẫn TRÙNG Ở ví dụ này VÌ hai xác suất bằng nhau — `0.5` VÀ `0.5`.)
Đổi trọng số LỆCH hẳn (giá trị `0` có xác suất CAO hơn NHIỀU) thì
`E[X]` bị KÉO VỀ phía `0`, TRONG KHI trung bình thường `(0+10)/2=5`
KHÔNG hề đổi — hai cách tính KHÁC NHAU, chỉ MỘT trong hai đúng nghĩa
"kỳ vọng".
::::

::::predict{#doan-mot-gia-tri-chac-chan commitOnce}
Byte có một biến ngẫu nhiên CHỈ nhận ĐÚNG MỘT giá trị (`7`), với xác
suất `1.0` (chắc chắn):

```python
def ky_vong(pp):
    return sum(x * p for x, p in pp.items())

pp = {7: 1.0}
print(ky_vong(pp))
```

Dòng cuối in ra gì?

:::opt{correct}
`7.0`
:::

:::opt
Máy báo lỗi khi chạy — `sum(x * p for x, p in pp.items())` cần ÍT
NHẤT hai cặp `(x, p)` để tính "trung bình", một `dict` chỉ MỘT cặp
là không đủ
::why
Gần đúng ở việc bạn nghĩ TỚI "trung bình" như một phép tính CẦN
NHIỀU giá trị để so sánh — một trực giác hợp lý VỚI trung bình
THƯỜNG.

Chỗ lệch: `sum(...)` trên một generator CHỈ có MỘT phần tử hoàn toàn
hợp lệ — nó CHỈ đơn giản LÀ giá trị của phần tử ĐÓ (`7×1.0=7.0`).
Không cần "ít nhất hai" gì cả; một biến ngẫu nhiên hoàn toàn có thể
CHỈ nhận một giá trị DUY NHẤT (biến cố chắc chắn, `P=1`, bài 12).
::
:::

:::opt
`1.0` — vì xác suất Ở TRONG `pp` LÀ `1.0`, và kỳ vọng "trả về" ngay
xác suất khi CHỈ có một giá trị duy nhất, không nhân với `x`
::why
Gần đúng ở việc bạn để ý xác suất `1.0` xuất hiện Ở TRONG `pp` — một
quan sát đúng VỀ DỮ LIỆU.

Chỗ lệch: công thức `E[X] = Σ x·P(X=x)` LUÔN nhân `x` VỚI `P(X=x)`
trước khi cộng — KHÔNG có trường hợp ĐẶC BIỆT nào "bỏ qua" phép
nhân. `7 × 1.0 = 7.0`, không phải trả thẳng xác suất.
::
:::
::::

::::code{#viet_ky_vong}
Viết `ky_vong(pp)` — tính `E[X] = Σ x·P(X=x)` từ bảng phân phối
`pp`.

```python title=starter
def ky_vong(pp):
    return ___


pp = {0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}

print(ky_vong(pp))
```

```python title=solution
def ky_vong(pp):
    return sum(x * p for x, p in pp.items())


pp = {0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}

print(ky_vong(pp))
```

```python title=test
assert ky_vong({7: 1.0}) == 7.0, "gia tri chac chan -- ky vong bang chinh no"
assert ky_vong({0: 0.5, 10: 0.5}) == 5.0, "hai gia tri bang xac suat -- ky vong o giua"
assert ky_vong({0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}) == 2.0, "phai khop bang phan phoi chinh"
```

:::hints
- kind: attention
  body: "Dung sum() voi generator, nhan x voi p cho tung cap trong pp.items()."
- kind: strategy
  body: "sum(x * p for x, p in pp.items())"
- kind: one-line
  body: "___ = sum(x * p for x, p in pp.items())"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dung sum() voi generator, nhan x voi p cho tung cap
  requireAst:
  - kind: uses-call, target: sum, min: 1
  - kind: uses-operator, target: '*', min: 1
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^2\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`E[X]=2.0` — không nguyên, dù `X` chỉ nhận giá trị nguyên. Byte gieo
ĐÚNG một lần — có lần nào `X` thật sự bằng `2.0` không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`E[X] = 2.0` — một con số KHÔNG nguyên, dù `X` chỉ nhận giá trị
nguyên (`0` tới `4`). Byte gieo đúng MỘT lần — có lần nào `X` thật
sự bằng `2.0` không? Vậy `E[X]` đang nói về CÁI GÌ, nếu không phải
kết quả một lần gieo cụ thể?
::::

::::checkpoint{mastery=0.8}
::::
