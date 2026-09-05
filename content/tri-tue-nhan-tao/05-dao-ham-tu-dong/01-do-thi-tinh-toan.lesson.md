---
id: tri-tue-nhan-tao.dao-ham-tu-dong.do-thi-tinh-toan
title: "Đồ thị tính toán"
summary: "Biểu diễn f = (a·b + c)·d (a=2, b=-3, c=10, d=-2) thành một đồ thị tính toán: ba node trung gian n1=a·b=-6, n2=n1+c=4, f=n2·d=-8, mỗi node lưu giá trị và node cha (toán hạng tạo ra nó) bằng dict Python — chỉ cấu trúc dữ liệu, chưa đạo hàm gì cả."
locale: vi
track: tri-tue-nhan-tao
module: dao-ham-tu-dong
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.do-thi-tinh-toan]
requires: [ai.boss-neuron-va-mang-nhieu-tang]
concepts: [ai.do-thi-tinh-toan]
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
`Layer`/`MLP` của q8.2a dùng trọng số cho sẵn — chưa một lần tính đạo hàm.
Quest này xây bộ máy TỰ TÍNH đạo hàm. Bước đầu tiên không phải công thức —
mà là một cách VẼ ra chính biểu thức đang tính.
::::

::::explain{#bieu-thuc-nhieu-buoc-la-mot-do-thi}
Một biểu thức như `f = (a·b + c)·d` không phải MỘT phép tính — nó là BA
phép tính nối tiếp, mỗi phép tạo ra một giá trị trung gian:

> `n1 = a·b` — nhân hai đầu vào.
>
> `n2 = n1 + c` — cộng kết quả trên với `c`.
>
> `f = n2·d` — nhân kết quả đó với `d`.

Mỗi giá trị trung gian (`n1`, `n2`, `f`) là một **node**. Node nào cũng có
đúng hai toán hạng đã TẠO RA nó — quy ước của quest này gọi hai toán hạng đó
là **node cha** của nó (không phải nghĩa "cha" thường gặp trong cây thư
mục — ở đây "cha" là "thứ đã sinh ra node này", giống cách nói "phép nhân
`a·b` là cha của `n1`). `n1` có node cha là `a` và `b`; `n2` có node cha là
`n1` và `c`; `f` có node cha là `n2` và `d`. `a`, `b`, `c`, `d` không có node
cha nào — chúng là **lá** (leaf) của đồ thị, đầu vào gốc không được tính ra
từ đâu cả.

Toàn bộ cấu trúc này — node, giá trị của từng node, và node cha của từng
node — gọi là một **đồ thị tính toán** (computational graph). Nó có HƯỚNG:
mỗi cạnh đi từ một node cha tới node mà nó tạo ra, không bao giờ đi ngược
lại. Biểu diễn một biểu thức thành đồ thị — thay vì chỉ một dòng code tính
thẳng ra kết quả cuối — là bước NỀN TẢNG mà mọi phần còn lại của quest này
xây tiếp lên: bài sau tính đạo hàm bằng tay trên ĐÚNG đồ thị này; các bài kế
tiếp dạy một class Python tự XÂY đồ thị này mỗi khi bạn viết `a * b`.

Ở bài này, đồ thị được biểu diễn bằng cấu trúc dữ liệu đơn giản nhất có thể:
một `dict` Python, mỗi khoá là tên node, mỗi giá trị là một `dict` nhỏ chứa
`"gia_tri"` (giá trị đã tính) và `"cha"` (một tuple tên các node cha).
::::

::::example{#do_thi_bang_tay}
Đồ thị của `f = (a·b + c)·d` với `a=2, b=-3, c=10, d=-2`, liệt kê bằng tay
từng node một:

```python title=readonly
do_thi = {}
do_thi["a"] = {"gia_tri": 2, "cha": ()}
do_thi["b"] = {"gia_tri": -3, "cha": ()}
do_thi["c"] = {"gia_tri": 10, "cha": ()}
do_thi["d"] = {"gia_tri": -2, "cha": ()}

do_thi["n1"] = {
    "gia_tri": do_thi["a"]["gia_tri"] * do_thi["b"]["gia_tri"],
    "cha": ("a", "b"),
}
do_thi["n2"] = {
    "gia_tri": do_thi["n1"]["gia_tri"] + do_thi["c"]["gia_tri"],
    "cha": ("n1", "c"),
}
do_thi["f"] = {
    "gia_tri": do_thi["n2"]["gia_tri"] * do_thi["d"]["gia_tri"],
    "cha": ("n2", "d"),
}

for ten in ("a", "b", "c", "d", "n1", "n2", "f"):
    print(ten, do_thi[ten]["gia_tri"], do_thi[ten]["cha"])
```

```text title=readonly
a 2 ()
b -3 ()
c 10 ()
d -2 ()
n1 -6 ('a', 'b')
n2 4 ('n1', 'c')
f -8 ('n2', 'd')
```

Bốn lá (`a`, `b`, `c`, `d`) có `"cha": ()` — tuple rỗng, không node nào tạo
ra chúng. Ba node trung gian (`n1`, `n2`, `f`) có đúng hai node cha mỗi cái,
và giá trị của chúng được TÍNH RA từ giá trị của node cha, không gõ tay:
`n1` đọc `"gia_tri"` của `a` và `b` rồi nhân lại; `n2` đọc `"gia_tri"` của
`n1` và `c` rồi cộng; `f` đọc `"gia_tri"` của `n2` và `d` rồi nhân. Kết quả
cuối `f = -8` khớp với việc tính thẳng bằng tay: `2×(-3) = -6`, cộng thêm
`10` được `4`, rồi `4×(-2) = -8`.
::::

::::predict{#doan_node_cha_cua_f commitOnce}
Đồ thị trên có bốn lá (`a`, `b`, `c`, `d`) và ba node trung gian (`n1`,
`n2`, `f`), mỗi node trung gian có đúng hai node cha.

**Trước khi đọc lại**, bạn đoán: node `f` — node CUỐI CÙNG, giữ kết quả của
cả biểu thức — có node cha là những node nào?

:::opt{correct}
`n2` và `d` — vì bước cuối cùng của biểu thức là `f = n2·d`, nhân kết quả
của bước trước (`n2`) với `d`
:::

:::opt
`a`, `b`, `c`, `d` — vì `f` phụ thuộc vào CẢ BỐN giá trị gốc, nên cả bốn đều
phải là node cha trực tiếp của nó
::why
Gần đúng ở việc `f` đúng là PHỤ THUỘC vào cả bốn giá trị gốc — thay đổi bất
kỳ giá trị nào trong `a, b, c, d` cũng làm `f` đổi theo. Quan sát về sự phụ
thuộc đó không sai.

Chỗ lệch: "phụ thuộc" (qua nhiều bước) khác với "là node cha TRỰC TIẾP"
(phép toán cuối cùng dùng thẳng giá trị đó). `f` chỉ được TẠO RA trực tiếp
bởi đúng MỘT phép toán — `n2·d` — nên node cha trực tiếp của nó chỉ có hai:
`n2` và `d`. `a`, `b`, `c` ảnh hưởng tới `f` gián tiếp, qua `n1` và `n2`,
không phải trực tiếp.
::
:::

:::opt
`n1` và `c` — vì đó là node cha của `n2`, và `f` được tính ra ngay sau `n2`
::why
Gần đúng ở việc `n1` và `c` đúng là một cặp node cha THẬT SỰ tồn tại trong
đồ thị này — không sai chỗ đó.

Chỗ lệch: `n1` và `c` là node cha của `n2`, không phải của `f`. Mỗi node chỉ
có node cha là TOÁN HẠNG của phép toán tạo ra CHÍNH NÓ — `f = n2·d` được tạo
từ `n2` và `d`, không phải từ những gì đã tạo ra `n2`.
::
:::
::::

::::code{#hoan_thien_do_thi}
Đồ thị của bốn lá và node `n1` đã cho sẵn. Hoàn thiện `n2` (cộng `n1` với
`c`) và `f` (nhân `n2` với `d`) — mỗi giá trị phải TÍNH từ node cha của nó
qua `do_thi[...]["gia_tri"]`, không được gõ thẳng con số.

```python title=starter
do_thi = {}
do_thi["a"] = {"gia_tri": 2, "cha": ()}
do_thi["b"] = {"gia_tri": -3, "cha": ()}
do_thi["c"] = {"gia_tri": 10, "cha": ()}
do_thi["d"] = {"gia_tri": -2, "cha": ()}

do_thi["n1"] = {
    "gia_tri": do_thi["a"]["gia_tri"] * do_thi["b"]["gia_tri"],
    "cha": ("a", "b"),
}
do_thi["n2"] = {
    "gia_tri": ___,          # do_thi["n1"]["gia_tri"] + do_thi["c"]["gia_tri"]
    "cha": ("n1", "c"),
}
do_thi["f"] = {
    "gia_tri": ___,          # do_thi["n2"]["gia_tri"] * do_thi["d"]["gia_tri"]
    "cha": ("n2", "d"),
}

print(do_thi["n1"]["gia_tri"])
print(do_thi["n2"]["gia_tri"])
print(do_thi["f"]["gia_tri"])
print(do_thi["f"]["cha"])
```

```python title=solution
do_thi = {}
do_thi["a"] = {"gia_tri": 2, "cha": ()}
do_thi["b"] = {"gia_tri": -3, "cha": ()}
do_thi["c"] = {"gia_tri": 10, "cha": ()}
do_thi["d"] = {"gia_tri": -2, "cha": ()}

do_thi["n1"] = {
    "gia_tri": do_thi["a"]["gia_tri"] * do_thi["b"]["gia_tri"],
    "cha": ("a", "b"),
}
do_thi["n2"] = {
    "gia_tri": do_thi["n1"]["gia_tri"] + do_thi["c"]["gia_tri"],
    "cha": ("n1", "c"),
}
do_thi["f"] = {
    "gia_tri": do_thi["n2"]["gia_tri"] * do_thi["d"]["gia_tri"],
    "cha": ("n2", "d"),
}

print(do_thi["n1"]["gia_tri"])
print(do_thi["n2"]["gia_tri"])
print(do_thi["f"]["gia_tri"])
print(do_thi["f"]["cha"])
```

```python title=test
assert do_thi["n1"]["gia_tri"] == -6, f"n1 sai -- dang ra {do_thi['n1']['gia_tri']}"
assert do_thi["n2"]["gia_tri"] == 4, f"n2 sai -- dang ra {do_thi['n2']['gia_tri']}"
assert do_thi["f"]["gia_tri"] == -8, f"f sai -- dang ra {do_thi['f']['gia_tri']}"
assert do_thi["n2"]["cha"] == ("n1", "c"), f"cha cua n2 sai -- dang ra {do_thi['n2']['cha']}"
assert do_thi["f"]["cha"] == ("n2", "d"), f"cha cua f sai -- dang ra {do_thi['f']['cha']}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, đúng hai phép toán còn lại của biểu thức. `n2` là bước CỘNG — lấy `"gia_tri"` của node cha `n1` cộng `"gia_tri"` của node cha `c`, đọc qua `do_thi["n1"]["gia_tri"]` và `do_thi["c"]["gia_tri"]` (giống hệt cách `n1` đã đọc `a`, `b` ở trên). `f` là bước NHÂN cuối — lấy `"gia_tri"` của `n2` nhân với `"gia_tri"` của `d`.
- kind: strategy
  body: 'n2: `do_thi["n1"]["gia_tri"] + do_thi["c"]["gia_tri"]`. f: `do_thi["n2"]["gia_tri"] * do_thi["d"]["gia_tri"]`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `do_thi["n1"]["gia_tri"] + do_thi["c"]["gia_tri"]` và `do_thi["n2"]["gia_tri"] * do_thi["d"]["gia_tri"]`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: n2 phai tinh THAT bang phep CONG tu gia tri cua n1 va c (khong duoc chep san so 4); f phai tinh THAT bang phep NHAN tu gia tri cua n2 va d (khong duoc chep san so -8) -- ca hai deu phai doc qua do_thi, khong duoc go thang con so
  requireAst:
  - kind: uses-name, target: do_thi, min: 10
  - kind: uses-operator, target: "+", min: 1
  - kind: uses-operator, target: "*", min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai dung
  # dat=true, ca ba luat qua sach (do_thi doc 17 lan qua toan bo bay dong gan;
  # "+" chi xuat hien 1 lan, o dong n2; "*" xuat hien 2 lan, o dong n1 [cho
  # san] va dong f). Cheat "n2 chep san 4" lam "+" ve 0 -- bi chan rieng no.
  # Cheat "f chep san -8" lam "*" tut tu 2 xuong 1 -- bi chan rieng no. Ca hai
  # cheat deu bi bat DOC LAP voi nhau, va deu lam "do_thi" tut duoi nguong 10
  # (chi con 4 lan doc tu n1's dong, thay vi 17).
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^-6\\n4\\n-8\\n\\('n2', 'd'\\)\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`f = -8`, đúng ba node trung gian, mỗi node biết node cha của mình. Đồ thị
xong — nhưng nó mới lưu GIÁ TRỊ. Bài sau: đạo hàm của `f` theo từng lá, tính
bằng tay, trên ĐÚNG đồ thị này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đồ thị này có bốn lá (`a, b, c, d`) và ba node trung gian. Nếu biểu thức dài
ra — ví dụ thêm một bước `g = f + e` với `e` là một lá mới — số node cha mà
`g` cần lưu là bao nhiêu, và điều đó có phụ thuộc vào việc biểu thức đã có
BAO NHIÊU bước trước đó không?
::::

::::checkpoint{mastery=0.8}
::::
