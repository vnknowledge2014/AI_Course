---
id: toan.tap-hop-quan-he-anh-xa.tich-descartes
title: Tích Descartes
summary: "`A × B` — tập hợp MỌI cặp có thứ tự (a,b) với a∈A, b∈B — bảy luống × bảy ngày cho 7×7=49 cặp (đúng phép nhân mảng chữ nhật, T2.1), TOÀN BỘ khả năng, chưa nói cái nào THẬT xảy ra."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.cartesian-product]
requires: [math.ordered-pair, ctrl.nested-loop]
concepts: [math.tich-descartes, math.moi-kha-nang]
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
Byte có bảy luống, tuần có bảy ngày. Liệt kê TẤT CẢ cặp (luống, ngày)
CÓ THỂ có — kể cả cặp không tưới thật — thì được bao nhiêu cặp?
::::

::::explain{#tich-descartes-la-gi}
**`A × B`** (tích Descartes) — tập hợp MỌI cặp có thứ tự `(a, b)` với
`a ∈ A`, `b ∈ B`. TOÀN BỘ khả năng GHÉP — chưa nói cái nào THẬT xảy
ra.

```python title=readonly
luong = {"luong_1", "luong_2"}
ngay = {"Hai", "Ba"}

tich = set()
for x in luong:
    for y in ngay:
        tich.add((x, y))

print(len(tich))
```

```text title=readonly
4
```

Hai luống × hai ngày = BỐN cặp: mỗi luống GHÉP với MỖI ngày, KHÔNG
sót cặp NÀO. Vòng lặp NGOÀI (T1.2) chạy qua TỪNG luống, vòng lặp
TRONG chạy qua TỪNG ngày CHO MỖI luống — đúng cấu TRÚC "vòng trong
lòng vòng" đã học.
::::

::::example{#kich-thuoc-la-tich-so}
Kích THƯỚC của `A × B` LUÔN bằng `|A| × |B|` — đúng phép nhân mảng
chữ NHẬT (T2.1 bài 20):

```python title=readonly
luong7 = {"l1", "l2", "l3", "l4", "l5", "l6", "l7"}
ngay7 = {"Hai", "Ba", "Tu", "Nam", "Sau", "Bay", "CN"}

tich = set()
for x in luong7:
    for y in ngay7:
        tich.add((x, y))

print(len(tich))
print(len(luong7) * len(ngay7))
```

```text title=readonly
49
49
```

Bảy luống × bảy ngày = 49 cặp — KHỚP CHÍNH XÁC `7 × 7`, không cần
đếm THỦ CÔNG. Đây LÀ toàn bộ khả năng GHÉP luống VỚI ngày — luống 1
CÓ THỂ "tưới" (theo phép TÍNH này) VÀO bất kỳ ngày nào trong bảy
ngày, kể CẢ những ngày nó KHÔNG thật sự tưới.
::::

::::predict{#doan-tich-voi-tap-rong commitOnce}
Byte tính tích Descartes với MỘT tập hợp RỖNG:

```python
ngay7 = {"Hai", "Ba", "Tu", "Nam", "Sau", "Bay", "CN"}

tich = set()
for x in set():
    for y in ngay7:
        tich.add((x, y))

print(len(tich))
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`7` — vì vòng lặp TRONG (`for y in ngay7`) vẫn chạy đủ BẢY lượt cho
MỖI lần vòng NGOÀI cố GẮNG chạy, nên bảy CẶP `(None, y)` vẫn được
thêm vào, MỘT cho mỗi ngày
::why
Gần đúng ở việc bạn nhớ ĐÚNG `ngay7` có bảy phần tử, và hình dung
vòng TRONG "vẫn phải chạy đủ" — một trực GIÁC hợp lý nếu nghĩ hai
vòng lặp ĐỘC LẬP nhau.

Chỗ lệch: vòng lặp TRONG (`for y in ngay7`) CHỈ chạy khi vòng NGOÀI
(`for x in set()`) ĐÃ vào được MỘT lượt — mà `set()` RỖNG nghĩa LÀ
vòng ngoài chạy **`0` lượt** (đã quen từ T1.4: lặp qua chỗ chứa RỖNG
là hợp lệ, chỉ đơn giản KHÔNG chạy lượt nào). Vòng NGOÀI không VÀO
lượt nào thì vòng TRONG (nằm BÊN TRONG thân vòng ngoài) CŨNG KHÔNG
BAO GIỜ được gọi tới, DÙ `ngay7` có bao nhiêu phần tử. `tich` VẪN
rỗng, `len(tich)` LÀ `0`.
::
:::

:::opt
Máy báo lỗi biên dịch — `for x in set():` lặp qua một `set()` RỖNG
ngay TRONG vòng lặp NGOÀI của một cấu trúc lồng hai TẦNG là cú pháp
không hợp lệ, Python CẤM vòng ngoài của MỘT cấu trúc lồng nhận đầu
vào rỗng
::why
Gần đúng ở việc bạn để ý `set()` RỖNG nằm Ở vị trí vòng NGOÀI của
cấu trúc lồng — một quan sát đúng về VỊ TRÍ cú pháp.

Chỗ lệch: Python KHÔNG hề PHÂN biệt "vòng ngoài" VÀ "vòng trong" khi
xét chỗ chứa CÓ rỗng hay không — CẢ HAI đều CHỈ LÀ vòng `for` bình
thường, và lặp qua RỖNG luôn hợp lệ Ở BẤT KỲ tầng lồng NÀO. Biên dịch
sạch, chạy sạch — chỉ đơn giản KHÔNG có lượt NÀO thực thi.
::
:::
::::

::::code{#viet_tich_descartes}
Viết `tich_descartes(a, b)` — trả về tập hợp MỌI cặp `(x, y)` với `x
∈ a`, `y ∈ b`.

```python title=starter
def tich_descartes(a, b):
    ket_qua = set()
    for x in a:
        for y in b:
            ket_qua.add(___)
    return ket_qua


print(len(tich_descartes({"a", "b"}, {1, 2})))
```

```python title=solution
def tich_descartes(a, b):
    ket_qua = set()
    for x in a:
        for y in b:
            ket_qua.add((x, y))
    return ket_qua


print(len(tich_descartes({"a", "b"}, {1, 2})))
```

```python title=test
assert tich_descartes({"a", "b"}, {1, 2}) == {("a", 1), ("a", 2), ("b", 1), ("b", 2)}, "moi phan tu a ghep voi moi phan tu b"
assert tich_descartes(set(), {"x"}) == set(), "a rong -- tich rong"
assert tich_descartes({"x"}, set()) == set(), "b rong -- tich rong"
assert len(tich_descartes({"a", "b", "c"}, {1, 2})) == 6, "kich thuoc bang tich so 3 x 2"
```

:::hints
- kind: attention
  body: "Them mot cap CO THU TU (x, y) vao ket_qua -- dung ngoac tron, dung thu tu x truoc y sau."
- kind: strategy
  body: "(x, y)"
- kind: one-line
  body: "___ = (x, y)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai them DUNG tuple (x, y) -- dung thu tu x truoc, y sau (khop dinh nghia cap co thu tu bai 14)
  requireAst:
  - kind: uses-name, target: x, min: 1
  - kind: uses-name, target: y, min: 1
  - kind: uses-call, target: add, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^4\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tích Descartes = mọi khả năng ghép, chưa nói cái nào THẬT. Bài sau:
từ TOÀN BỘ khả năng đó, chọn ra đúng những cặp THẬT SỰ xảy ra.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

49 cặp là mọi khả năng. Nhưng luống 1 CHỈ tưới thứ Hai và thứ Năm
thật sự — hai cặp trong 49. Tập con "chỉ những cặp thật" ấy gọi là
gì?
::::

::::checkpoint{mastery=0.8}
::::
