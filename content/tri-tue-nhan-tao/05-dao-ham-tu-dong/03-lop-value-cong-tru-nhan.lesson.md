---
id: tri-tue-nhan-tao.dao-ham-tu-dong.lop-value-cong-tru-nhan
title: "Lớp Value: cộng, trừ, nhân"
summary: "Class Value đầu tiên: bọc một số vô hướng (self.data), lưu self.grad=0, node cha (self._prev), và một closure _backward RIÊNG cho mỗi phép toán (__add__, __sub__, __mul__). Trên e=a*b, f=e-c (a=4.0,b=-2.0,c=5.0): gọi tay f._backward() rồi e._backward() cho a.grad=-2.0, b.grad=4.0, c.grad=-1.0 — chưa có .backward() toàn cục, đó là bài 5."
locale: vi
track: tri-tue-nhan-tao
module: dao-ham-tu-dong
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.lop-value-cong-tru-nhan]
requires: [ai.dao-ham-nguoc-chuoi]
concepts: [ai.lop-value-cong-tru-nhan]
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
Viết tay bốn công thức cho bốn lá đã đủ mệt. Giờ để một CLASS tự nhớ hộ:
mỗi phép toán tự biết đạo hàm cục bộ của chính nó, ngay lúc được gọi.
::::

::::explain{#class-value}
Ý tưởng: thay vì `a`, `b`, `c` là số Python trần (`int`/`float`), bọc chúng
trong một class `Value`. Mỗi `Value` lưu bốn thứ:

> **`self.data`** — giá trị số thật (đúng thứ `a=2` từng là).
>
> **`self.grad`** — đạo hàm của node CUỐI đồ thị theo node này, khởi tạo
> `0` (chưa biết gì, sẽ được điền khi lan truyền ngược chạy).
>
> **`self._prev`** — node cha của nó (đúng khái niệm bài `do-thi-tinh-toan`,
> giờ lưu THẲNG trong object thay vì một `dict` rời).
>
> **`self._backward`** — một HÀM, RIÊNG cho node này, biết cách LAN gradient
> của chính nó ngược về node cha. Mỗi phép toán (`+`, `-`, `*`) tự định
> nghĩa hàm này theo đúng công thức đạo hàm cục bộ của phép toán đó.

Khi viết `e = a * b` (với `a`, `b` là `Value`), Python gọi
`a.__mul__(b)` — và hàm này KHÔNG chỉ trả về `a.data * b.data`. Nó tạo một
`Value` mới (`e`), gán `e._prev = (a, b)` (đúng node cha), và gắn cho `e`
một `_backward` riêng: một hàm ghi nhớ ĐÚNG công thức "nếu biết `e.grad`,
`a.grad` phải cộng thêm bao nhiêu, `b.grad` phải cộng thêm bao nhiêu". Công
thức đó chính là đạo hàm cục bộ của phép nhân: `∂e/∂a = b`, `∂e/∂b = a` —
đã suy ra ở bài `dao-ham-nguoc-chuoi`, giờ gói vào code thay vì viết tay.

Ba phép đầu tiên — `__add__`, `__sub__`, `__mul__` — mỗi cái có công thức
đạo hàm cục bộ RIÊNG:

> `__add__` (`out = self + other`): `∂out/∂self = 1`, `∂out/∂other = 1`.
>
> `__sub__` (`out = self - other`): `∂out/∂self = 1`, `∂out/∂other = -1`
> (KHÁC `__add__` — vế `other` mang dấu âm).
>
> `__mul__` (`out = self * other`): `∂out/∂self = other.data`,
> `∂out/∂other = self.data` (đạo hàm cục bộ của phép nhân — mỗi vế nhân
> với GIÁ TRỊ của vế còn lại, không phải với chính nó).

Mỗi `_backward` CỘNG DỒN (`+=`) vào `grad` đã có, không GHI ĐÈ — quy ước này
sẽ trở nên quan trọng khi một `Value` được dùng nhiều lần (bài
`tich-luy-gradient` mổ xẻ kỹ lý do). Ở bài này, mỗi lá chỉ xuất hiện đúng
một lần trong đồ thị, nên `+=` và `=` cho cùng kết quả — chưa thấy khác biệt
ngay, nhưng thói quen viết `+=` bắt đầu từ đây.

Bài này CHƯA có một hàm `.backward()` toàn cục tự đi khắp đồ thị — đó là
việc của bài `sap-xep-to-po-va-lan-truyen-nguoc`. Ở đây, lan truyền ngược
được gọi TAY: gán `grad = 1` cho node cuối, rồi tự gọi `_backward()` của
từng node, ĐÚNG THỨ TỰ ngược (node được tính sau cùng lan trước).
::::

::::example{#lan_truyen_tay}
`e = a * b`, `f = e - c`, với `a=4.0, b=-2.0, c=5.0` — lan truyền ngược
GỌI TAY, đúng thứ tự `f` trước rồi `e`:

```python title=readonly
class Value:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __sub__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

a = Value(4.0)
b = Value(-2.0)
c = Value(5.0)

e = a * b
f = e - c

f.grad = 1.0
f._backward()
e._backward()

print(e.data, f.data)
print(a.grad, b.grad, c.grad)
```

```text title=readonly
-8.0 -13.0
-2.0 4.0 -1.0
```

`e = a*b = -8.0`, `f = e-c = -13.0` — đúng giá trị thuần Python. Sau khi gán
`f.grad = 1.0` (node cuối luôn có đạo hàm `1` theo chính nó) và gọi
`f._backward()` rồi `e._backward()` THEO ĐÚNG THỨ TỰ ĐÓ (không thể đảo — gọi
`e._backward()` trước sẽ lan gradient khi `e.grad` còn là `0`, vì `f` chưa
kịp cộng dồn vào nó), ba lá nhận đúng gradient: `a.grad = -2.0` (bằng
`b.data`, đúng công thức `∂e/∂a = b` của phép nhân), `b.grad = 4.0` (bằng
`a.data`), `c.grad = -1.0` (dấu âm, đúng công thức `∂f/∂c = -1` của
`__sub__`).
::::

::::predict{#doan_dao_nguoc_thu_tu commitOnce}
Ví dụ trên gọi `f._backward()` TRƯỚC, rồi mới `e._backward()`.

**Trước khi đọc lại**, bạn đoán: nếu đảo thứ tự — gọi `e._backward()` trước,
`f._backward()` sau — `a.grad` và `b.grad` cuối cùng có còn đúng `-2.0` và
`4.0` không?

:::opt{correct}
Không — `e._backward()` đọc `e.grad` để tính phần cộng dồn cho `a.grad` và
`b.grad`; nếu gọi trước khi `f._backward()` kịp cộng vào `e.grad`, `e.grad`
vẫn còn `0` (giá trị khởi tạo), nên `a.grad` và `b.grad` đều nhận `0`, không
phải `-2.0`/`4.0`
:::

:::opt
Có — `_backward` của mỗi node chỉ phụ thuộc vào công thức của CHÍNH node đó,
không phụ thuộc thứ tự gọi
::why
Gần đúng ở việc công thức BÊN TRONG mỗi `_backward` (ví dụ `self.grad +=
other.data * out.grad`) đúng là cố định, không đổi theo thứ tự gọi — quan
sát về công thức không sai.

Chỗ lệch: công thức đó ĐỌC `out.grad` — một giá trị PHỤ THUỘC vào việc node
CHA của `out` (ở đây, `f` là "cha" theo nghĩa nó lan gradient VÀO `e`) đã lan
gradient vào `out` hay chưa. Gọi `e._backward()` trước khi `f._backward()`
chạy nghĩa là đọc `e.grad` lúc nó vẫn là `0` — công thức không đổi, nhưng
GIÁ TRỊ nó đọc vào thì sai.
::
:::

:::opt
Không xác định được nếu không chạy thử — với đồ thị nhỏ như thế này, thứ tự
gọi không theo quy luật nào rõ ràng
::why
Gần đúng ở tinh thần cẩn trọng khi không chắc — kiểm tra lại bằng cách chạy
thử không phải một thói quen tồi.

Chỗ lệch: có một quy luật CHẮC CHẮN áp dụng ở đây, không cần đoán mò: một
node phải lan gradient của CHÍNH NÓ ra ngoài (cộng dồn vào node cha) TRƯỚC
khi node cha đó dùng gradient vừa nhận để lan tiếp. `f` phải chạy trước `e`
vì `e` là node cha của `f` (toán hạng của phép trừ tạo ra `f`) — thứ tự
ngược lại luôn sai, không phụ thuộc đồ thị lớn hay nhỏ.
::
:::
::::

::::code{#hoan_thien_value_cong_tru_nhan}
Hoàn thiện ba closure `_backward`: `__sub__` còn thiếu công thức cho
`other.grad` (dấu ÂM); `__mul__` còn thiếu CẢ HAI công thức (quy tắc tích:
mỗi vế nhân với GIÁ TRỊ của vế kia).

```python title=starter
class Value:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __sub__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += ___          # -out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += ___           # other.data * out.grad
            other.grad += ___          # self.data * out.grad
        out._backward = _backward
        return out

a = Value(4.0)
b = Value(-2.0)
c = Value(5.0)

e = a * b
f = e - c

f.grad = 1.0
f._backward()
e._backward()

print(round(e.data, 4), round(f.data, 4))
print(round(a.grad, 4), round(b.grad, 4), round(c.grad, 4))
```

```python title=solution
class Value:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __sub__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

a = Value(4.0)
b = Value(-2.0)
c = Value(5.0)

e = a * b
f = e - c

f.grad = 1.0
f._backward()
e._backward()

print(round(e.data, 4), round(f.data, 4))
print(round(a.grad, 4), round(b.grad, 4), round(c.grad, 4))
```

```python title=test
assert round(e.data, 4) == -8.0, f"e.data sai -- dang ra {round(e.data, 4)}"
assert round(f.data, 4) == -13.0, f"f.data sai -- dang ra {round(f.data, 4)}"
assert round(a.grad, 4) == -2.0, f"a.grad sai -- dang ra {round(a.grad, 4)}"
assert round(b.grad, 4) == 4.0, f"b.grad sai -- dang ra {round(b.grad, 4)}"
assert round(c.grad, 4) == -1.0, f"c.grad sai -- dang ra {round(c.grad, 4)}"

# rieng kiem tra khong bi DAO NGUOC hai ve cua phep nhan: a.data=4.0 va
# b.data=-2.0 khac nhau, nen neu cong thuc dao (self.grad dung self.data
# thay vi other.data) se cho a.grad=4.0/b.grad=-2.0 -- khac han ket qua
# dung, bi bat qua chinh hai assert tren.
assert round(a.grad, 4) != round(b.grad, 4), "a.grad va b.grad khong duoc trung nhau -- kiem tra lai cong thuc tich co bi dao ve khong"
```

:::hints
- kind: attention
  body: Ba chỗ trống. `__sub__`, vế `other`: đạo hàm cục bộ của phép trừ theo TOÁN HẠNG THỨ HAI là `-1`, nên cộng dồn `-out.grad` (không phải `out.grad` như vế `self`). `__mul__`, cả hai vế: quy tắc tích — vế `self` nhân với GIÁ TRỊ của `other` (`other.data`), vế `other` nhân với GIÁ TRỊ của `self` (`self.data`); cả hai đều nhân thêm với `out.grad` để lan tiếp.
- kind: strategy
  body: '__sub__: `other.grad += -out.grad`. __mul__: `self.grad += other.data * out.grad` và `other.grad += self.data * out.grad`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `-out.grad`, `other.data * out.grad`, và `self.data * out.grad`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: __sub__ phai cong don DAU AM (-out.grad) cho other.grad; __mul__ phai dung DUNG quy tac tich -- self.grad cong other.data*out.grad, other.grad cong self.data*out.grad, khong duoc dao nguoc hay chep hang so
  requireAst:
  - kind: uses-operator, target: "*", min: 4
  - kind: uses-operator, target: neg, min: 2
  - kind: uses-name, target: self, min: 12
  - kind: uses-name, target: other, min: 15
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach ("*"=4: self.data*other.data trong
  # __mul__, hai lan trong cac closure, va a*b o cuoi; neg=2: -2.0 khi tao
  # Value(b), va -out.grad trong __sub__; self=15, other=19 tren toan bo
  # file). Cheat "other.grad chep 0 trong __sub__" lam "neg" tut ve 1 -- bi
  # chan. Cheat "__mul__ ca hai vong tra ve hang so chep san" lam "*" tut
  # xuong 2 -- bi chan. Ca hai deu bi bat DOC LAP; tier tests con bat rieng
  # phep DAO hai ve tich qua assert a.grad != b.grad (4.0 != -2.0 that su
  # khac nhau vi a.data != b.data).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^-8\\.0 -13\\.0\\n-2\\.0 4\\.0 -1\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba phép toán, ba công thức đạo hàm cục bộ, gọi tay đúng thứ tự — và gradient
ra khớp với công thức viết tay ở bài trước. Bài sau: thêm luỹ thừa, `tanh`,
`ReLU`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ví dụ trên phải gọi `f._backward()` rồi `e._backward()` — TỰ TAY, đúng thứ
tự. Với đồ thị `3` node như thế này thì tự nhớ thứ tự không khó. Với một
biểu thức có `50` node trung gian, việc tự nhớ đúng thứ tự gọi bằng tay có
còn khả thi không — và cái gì có thể tự động hoá việc "tìm đúng thứ tự" đó?
::::

::::checkpoint{mastery=0.8}
::::
