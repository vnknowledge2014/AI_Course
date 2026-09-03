---
id: toan.dstt-giai-tich-cho-ai.khoang-cach-hai-vector
title: Khoảng cách giữa hai vector
summary: "d(u,v) = ‖u−v‖ — khoảng cách Euclid LÀ độ dài của vector HIỆU; 'hai luống GIỐNG nhau bao nhiêu' đo bằng khoảng CÁCH hồ sơ của chúng."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.vector-distance]
requires: [math.vector-norm]
concepts: [math.khoang-cach-vector]
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
Hai luống Ở vị trí `(3.0, 4.0)` VÀ `(0.0, 0.0)` (gốc) — khoảng CÁCH
giữa CHÚNG chính LÀ độ dài vector NÀO?
::::

::::explain{#khoang-cach-hai-vector}
LÀ độ dài của vector HIỆU. **`d(u,v) = ‖u−v‖`** — khoảng cách Euclid
LÀ độ dài (bài 4) của `u−v` (trừ TỪNG thành phần TƯƠNG ứng, đúng lối
cộng vector, bài 2, CHỈ đổi dấu):

```python title=readonly
def tru_vector(u, v):
    return tuple(a - b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def khoang_cach(u, v):
    return do_dai(tru_vector(u, v))


print(khoang_cach((3.0, 4.0), (0.0, 0.0)))
```

```text title=readonly
5.0
```

`(3.0,4.0) − (0.0,0.0) = (3.0,4.0)` — HIỆU CHÍNH LÀ vector gốc, độ
dài của NÓ (bài 4) LÀ `5.0` — ĐÚNG "khoảng cách từ (3,4) tới gốc".
::::

::::example{#khoang-cach-doi-xung}
Khoảng cách KHÔNG phụ thuộc THỨ TỰ — `d(u,v) = d(v,u)`:

```python title=readonly
def tru_vector(u, v):
    return tuple(a - b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def khoang_cach(u, v):
    return do_dai(tru_vector(u, v))


print(khoang_cach((0.0, 0.0), (3.0, 4.0)))
```

```text title=readonly
5.0
```

ĐỔI thứ tự — `(0,0)−(3,4)=(−3,−4)`, VẪN cùng độ dài `5.0` (bình
phương "xoá" dấu ÂM). Khoảng cách "TỚI" VÀ "TỪ" LÀ cùng một con số.
::::

::::predict{#doan-khong-qua-goc commitOnce}
Byte tính khoảng cách GIỮA `(5.0, 7.0)` VÀ `(2.0, 3.0)` — LẦN NÀY
KHÔNG qua gốc:

```python
def tru_vector(u, v):
    return tuple(a - b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def khoang_cach(u, v):
    return do_dai(tru_vector(u, v))

print(khoang_cach((5.0, 7.0), (2.0, 3.0)))
```

Dòng cuối in ra gì?

:::opt{correct}
`5.0`
:::

:::opt
`7.0` — vì khoảng CÁCH đơn giản LÀ tổng CHÊNH lệch từng tọa ĐỘ:
`|5−2|+|7−3|=3+4=7`
::why
Gần đúng ở việc bạn TÍNH đúng chênh LỆCH từng toạ độ (`3` VÀ `4`) —
VÀ CÁCH cộng TRỰC tiếp CHÊNH lệch (`3+4=7`) LÀ một cách đo khoảng
cách CÓ THẬT, gọi LÀ **khoảng cách Manhattan** (đi theo LƯỚI ô
vuông, KHÔNG đi thẳng).

Chỗ lệch: `khoang_cach` Ở ĐÂY tính khoảng cách **Euclid** (đường
thẳng NGẮN nhất, đúng Pythagoras) — CẦN bình phương RỒI khai căn:
`√(3²+4²)=√25=5`, KHÔNG cộng thẳng CHÊNH lệch. Hai cách đo NÀY LÀ
hai khái niệm KHÁC nhau, cùng tồn tại NHƯNG cho kết quả khác.
::
:::

:::opt
Máy báo lỗi khi chạy — `khoang_cach` gọi `tru_vector` RỒI `do_dai`
LỒNG hai hàm VÀO nhau (`do_dai(tru_vector(u, v))`), Python KHÔNG cho
GỌI kết quả của một hàm LÀM đối số cho hàm KHÁC
::why
Gần đúng ở việc bạn để ý ĐÚNG hai hàm được GỌI lồng NHAU — một quan
sát VỀ cấu TRÚC mã chính XÁC.

Chỗ lệch: GỌI hàm LỒNG nhau (kết quả hàm NÀY LÀM đầu VÀO hàm khác)
LÀ điều CỰC kỳ bình thường trong Python — KHÔNG có RÀNG buộc nào cấm.
Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_khoang_cach}
Viết `khoang_cach(u, v)` — tính khoảng cách Euclid giữa hai vector.

```python title=starter
def tru_vector(u, v):
    return tuple(a - b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def khoang_cach(u, v):
    return ___


print(khoang_cach((3.0, 4.0), (0.0, 0.0)))
```

```python title=solution
def tru_vector(u, v):
    return tuple(a - b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def khoang_cach(u, v):
    return do_dai(tru_vector(u, v))


print(khoang_cach((3.0, 4.0), (0.0, 0.0)))
```

```python title=test
assert khoang_cach((), ()) == 0.0, "hai vector rong -- khoang cach 0"
assert khoang_cach((5.0, 5.0), (5.0, 5.0)) == 0.0, "cung mot diem -- khoang cach 0"
assert khoang_cach((3.0, 4.0), (0.0, 0.0)) == 5.0, "toi goc"
assert khoang_cach((0.0, 0.0), (3.0, 4.0)) == 5.0, "doi xung"
assert khoang_cach((5.0, 7.0), (2.0, 3.0)) == 5.0, "khong qua goc"
assert khoang_cach((2.0, 3.0, 6.0), (0.0, 0.0, 0.0)) == 7.0, "ba chieu"
```

:::hints
- kind: attention
  body: "Tinh vector hieu roi lay do dai cua no: do_dai(tru_vector(u, v))."
- kind: strategy
  body: "do_dai(tru_vector(u, v))"
- kind: one-line
  body: "___ = do_dai(tru_vector(u, v))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi do_dai(tru_vector(u, v))
  requireAst:
  - kind: uses-call, target: do_dai, min: 1
  - kind: uses-call, target: tru_vector, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^5\.0\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khoảng cách — độ dài của HIỆU. Có cách nào đo "GIỐNG nhau" mà KHÔNG
quan tâm ĐỘ LỚN tuyệt đối, chỉ quan tâm HƯỚNG?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai luống hồ sơ GẦN NHAU (khoảng cách NHỎ) — điều ĐÓ nói lên gì VỀ
chúng? Có cách nào đo "giống nhau" mà KHÔNG quan tâm ĐỘ LỚN tuyệt
đối, chỉ quan tâm HƯỚNG?
::::

::::checkpoint{mastery=0.8}
::::
