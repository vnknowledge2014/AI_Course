---
id: tri-tue-nhan-tao.dong-co-tensor.lop-tensor-co-ban
title: "Lớp Tensor cơ bản: cộng, trừ, nhân"
summary: "Tensor bọc một mảng numpy (self.data), giữ NGUYÊN _prev/_backward/.grad của Value -- chỉ đổi self.grad=0.0 (số) thành self.grad=np.zeros_like(data) (mảng). backward() TÁI DÙNG y hệt thuật toán tô-pô của Value, seed self.grad=np.ones_like(data). Trên e=a*b, f=e-c (ma trận 2x2): a.grad=[[-1,2],[0,5]], b.grad=[[4,-2],[1,3]], c.grad=[[-1,-1],[-1,-1]]. Quest này giới hạn +/-/* CHỈ hai Tensor CÙNG SHAPE -- trộn shape khác nhau làm .backward() NÉM ValueError thật, đã tự kiểm chứng."
locale: vi
track: tri-tue-nhan-tao
module: dong-co-tensor
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.lop-tensor-co-ban]
requires: [ai.vi-sao-can-tensor]
concepts: [ai.lop-tensor-co-ban]
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
`Value` bọc một số. Giờ bọc một MẢNG — cùng ý tưởng `_prev`/`_backward`,
nhưng `self.data` là cả một ma trận `numpy`, không phải một `float`.
::::

::::explain{#lop_tensor}
`Tensor` giữ ĐÚNG bốn thứ mà `Value` đã giữ, chỉ đổi KIỂU DỮ LIỆU bên trong:

> **`self.data`** — không còn là một `float`, mà là một mảng `numpy`
> (`np.array(...)`) — có thể là một vector, một ma trận, bất kỳ hình dạng
> nào.
>
> **`self.grad`** — không còn khởi tạo `0.0`, mà là `np.zeros_like(self.
> data)` — một mảng TOÀN SỐ `0`, CÙNG HÌNH DẠNG với `self.data`. Gradient
> của một mảng phải là một mảng cùng shape — mỗi phần tử của `data` có một
> đạo hàm riêng.
>
> **`self._prev`** — node cha, giống hệt `Value`.
>
> **`self._backward`** — một closure, giống hệt `Value` — chỉ khác công
> thức bên trong giờ thao tác trên MẢNG thay vì một số.

`__add__`, `__sub__`, `__mul__` viết THEO ĐÚNG khuôn của `Value`
(`lop-value-cong-tru-nhan`) — chỉ khác một điều QUAN TRỌNG: **quest này giới
hạn CẢ BA phép toán chỉ hoạt động giữa hai `Tensor` CÙNG SHAPE**. `Value`
không cần lo chuyện này (một số luôn "cùng hình dạng" với một số khác).
`numpy` có cơ chế **broadcast** tự động khớp các shape KHÁC NHAU (ví dụ cộng
một hàng `(1, 3)` với một ma trận `(4, 3)`), nhưng làm `_backward` đúng cho
trường hợp broadcast đòi một bước CỘNG DỒN gradient theo đúng trục đã broadcast
— một kỹ thuật riêng, quest này CHƯA cần tới (mọi phép cộng/trừ/nhân
element-wise trong suốt quest đều diễn ra giữa các `Tensor` đã CÙNG shape sẵn
từ thiết kế). Các phép toán THẬT SỰ cần shape khác nhau — `softmax` theo
trục cuối, `layer normalization` theo trục cuối — sẽ là PHƯƠNG THỨC RIÊNG
(bài `softmax-qua-tensor`, `chuan-hoa-lop`), mỗi cái tự viết công thức đạo
hàm ĐÚNG cho đúng phép biến đổi đó, không dựa vào `__add__`/`__sub__` chung.

`backward()` TÁI DÙNG chính xác thuật toán sắp-xếp-tô-pô của `Value`
(`sap-xep-to-po-va-lan-truyen-nguoc`) — không đổi một dòng logic nào, chỉ
đổi giá trị seed: `self.grad = np.ones_like(self.data)` thay vì `self.grad =
1.0`. Seed toàn số `1` cho MỌI phần tử tương đương với việc coi đầu ra là
`sum(self.data)` (tổng mọi phần tử) rồi lan gradient từ đó — đây là quy ước
`Tensor.backward()` dùng xuyên suốt quest này: gọi trực tiếp trên một
`Tensor` bất kỳ shape nào cũng lan gradient NHƯ THỂ đã cộng hết các phần tử
của nó lại thành một số trước.
::::

::::example{#tensor_cong_tru_nhan}
`e = a * b`, `f = e - c` — ba ma trận `2×2`, lan truyền qua `.backward()`
đầy đủ (không còn gọi tay từng `_backward()` như `Value` đã làm ở bài đầu
tiên của nó — `Tensor` được thừa hưởng bản `.backward()` đã HOÀN CHỈNH):

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __sub__(self, other):
        out = Tensor(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        out = Tensor(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def backward(self):
        topo, visited = [], set()
        def build(v):
            if v not in visited:
                visited.add(v)
                for con in v._prev:
                    build(con)
                topo.append(v)
        build(self)
        self.grad = np.ones_like(self.data)
        for node in reversed(topo):
            node._backward()


a = Tensor([[4., -2.], [1., 3.]])
b = Tensor([[-1., 2.], [0., 5.]])
c = Tensor([[2., 2.], [2., 2.]])

e = a * b
f = e - c
f.backward()

print(np.round(e.data, 4).tolist())
print(np.round(f.data, 4).tolist())
print(np.round(a.grad, 4).tolist())
print(np.round(b.grad, 4).tolist())
print(np.round(c.grad, 4).tolist())
```

```text title=readonly
[[-4.0, -4.0], [0.0, 15.0]]
[[-6.0, -6.0], [-2.0, 13.0]]
[[-1.0, 2.0], [0.0, 5.0]]
[[4.0, -2.0], [1.0, 3.0]]
[[-1.0, -1.0], [-1.0, -1.0]]
```

`a.grad` khớp CHÍNH XÁC `b.data` (đúng công thức tích: `∂(a*b)/∂a = b`, mỗi
phần tử ĐỘC LẬP — element-wise, không có tổng chéo giữa các phần tử khác
hàng/cột), `b.grad` khớp `a.data`, và `c.grad` toàn `-1` (đúng dấu âm của
`__sub__`, giống hệt `Value`). Một lời gọi `.backward()` DUY NHẤT — không
còn phải tự gọi tay từng `_backward()` theo thứ tự như `Value` đã phải làm ở
bài đầu tiên của nó, vì `Tensor` thừa hưởng thẳng bản `.backward()` đã
trưởng thành.
::::

::::example{#vi_sao_gioi_han_cung_shape}
Nếu bỏ qua giới hạn CÙNG SHAPE — cộng một `Tensor` `(1, 2)` với một `Tensor`
`(2, 2)` — `numpy` broadcast được ở bước FORWARD (`self.data + other.data`
chạy không lỗi), nhưng `_backward` thì KHÔNG:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def backward(self):
        topo, visited = [], set()
        def build(v):
            if v not in visited:
                visited.add(v)
                for con in v._prev:
                    build(con)
                topo.append(v)
        build(self)
        self.grad = np.ones_like(self.data)
        for node in reversed(topo):
            node._backward()


a = Tensor([[1., 2.]])            # shape (1, 2)
b = Tensor([[3., 4.], [5., 6.]])  # shape (2, 2)
c = a + b

print(c.data.shape)
print(np.round(c.data, 4).tolist())

try:
    c.backward()
    print("khong loi")
except ValueError:
    print("ValueError: khong cong don duoc gradient vao mot shape nho hon")
```

```text title=readonly
(2, 2)
[[4.0, 6.0], [6.0, 8.0]]
ValueError: khong cong don duoc gradient vao mot shape nho hon
```

Bước FORWARD (`c = a + b`) chạy trót lọt — `numpy` tự broadcast `a` (shape
`(1, 2)`) lên `(2, 2)` để cộng, cho `c` đúng shape `(2, 2)`. Nhưng
`c.backward()` THẤT BẠI: `_backward` của `__add__` viết `self.grad +=
out.grad` — với `self` là `a` (shape `(1, 2)`, `self.grad` cũng shape
`(1, 2)`) và `out.grad` shape `(2, 2)` (shape của `c`), phép `+=` này đòi hỏi
vế phải PHẢI broadcast được VÀO shape của vế trái — mà `(2, 2)` LỚN HƠN
`(1, 2)`, không thu nhỏ lại được. `numpy` ném `ValueError` thật, không âm
thầm cho một gradient sai. Đây CHÍNH XÁC là lý do quest này giới hạn CÙNG
SHAPE: viết đúng bước "cộng dồn gradient ngược qua broadcast" là một kỹ
thuật riêng, quest chưa cần tới.
::::

::::predict{#doan_shape_khac_nhau commitOnce}
Ví dụ trên: `a` shape `(1, 2)`, `b` shape `(2, 2)`, `c = a + b` chạy được ở
bước FORWARD nhưng `c.backward()` ném `ValueError`.

**Trước khi đọc lại**, bạn đoán: nếu đổi `a` thành shape `(2, 2)` — CÙNG
shape với `b` — thì `c.backward()` còn ném lỗi đó không?

:::opt{correct}
Không — với `a` và `b` CÙNG shape `(2, 2)`, `out.grad` (shape `(2, 2)`)
cộng dồn thẳng vào `self.grad` (cũng shape `(2, 2)`) mà không cần broadcast
gì cả; `_backward` của `__add__` chỉ thất bại khi shape của `self`/`other`
NHỎ HƠN shape của `out` — đúng vấn đề mà giới hạn "cùng shape" của quest này
né tránh
:::

:::opt
Vẫn ném lỗi — vì `numpy` luôn đòi shape khớp tuyệt đối cho MỌI phép `+=`,
kể cả khi hai bên đã cùng shape
::why
Gần đúng ở việc `numpy` đúng là NGHIÊM NGẶT với `+=` khi shape lệch nhau —
quan sát đó không sai ở trường hợp `(1,2)` cộng `(2,2)`.

Chỗ lệch: "cùng shape" chính là trường hợp KHÔNG lệch — `self.grad` và
`out.grad` khi đó có ĐÚNG cùng số chiều VÀ cùng kích thước từng chiều, nên
`+=` chạy bình thường như mọi phép cộng mảng cùng shape khác. Lỗi chỉ xảy ra
khi có SỰ CHÊNH LỆCH shape cần broadcast lại — không phải một luật cấm mọi
`+=` giữa hai mảng.
::
:::

:::opt
Không xác định được nếu không chạy thử — hành vi broadcast của `numpy` phụ
thuộc phiên bản cài đặt, không có quy luật cố định
::why
Gần đúng ở tinh thần cẩn trọng với sự khác biệt giữa các phiên bản thư viện
— một số hành vi CÓ thể đổi giữa các bản `numpy`.

Chỗ lệch: quy tắc broadcast cơ bản của `numpy` (hai mảng CÙNG shape luôn
cộng được trực tiếp, không cần broadcast gì cả) là một phần CHUẨN, ổn định
của ngôn ngữ đặc tả mảng — không phải một chi tiết dễ đổi giữa các phiên
bản. Trường hợp cùng shape không hề mơ hồ.
::
:::
::::

::::code{#hoan_thien_tensor_co_ban}
Hoàn thiện `_backward` của `__sub__` (dấu ÂM cho `other.grad`) và `__mul__`
(quy tắc tích, mỗi vế nhân với GIÁ TRỊ mảng của vế kia).

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __sub__(self, other):
        out = Tensor(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += ___              # -out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        out = Tensor(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += ___               # other.data * out.grad
            other.grad += ___              # self.data * out.grad
        out._backward = _backward
        return out

    def backward(self):
        topo, visited = [], set()
        def build(v):
            if v not in visited:
                visited.add(v)
                for con in v._prev:
                    build(con)
                topo.append(v)
        build(self)
        self.grad = np.ones_like(self.data)
        for node in reversed(topo):
            node._backward()


a = Tensor([[4., -2.], [1., 3.]])
b = Tensor([[-1., 2.], [0., 5.]])
c = Tensor([[2., 2.], [2., 2.]])

e = a * b
f = e - c
f.backward()

print(np.round(e.data, 4).tolist())
print(np.round(f.data, 4).tolist())
print(np.round(a.grad, 4).tolist())
print(np.round(b.grad, 4).tolist())
print(np.round(c.grad, 4).tolist())
```

```python title=solution
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __sub__(self, other):
        out = Tensor(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        out = Tensor(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def backward(self):
        topo, visited = [], set()
        def build(v):
            if v not in visited:
                visited.add(v)
                for con in v._prev:
                    build(con)
                topo.append(v)
        build(self)
        self.grad = np.ones_like(self.data)
        for node in reversed(topo):
            node._backward()


a = Tensor([[4., -2.], [1., 3.]])
b = Tensor([[-1., 2.], [0., 5.]])
c = Tensor([[2., 2.], [2., 2.]])

e = a * b
f = e - c
f.backward()

print(np.round(e.data, 4).tolist())
print(np.round(f.data, 4).tolist())
print(np.round(a.grad, 4).tolist())
print(np.round(b.grad, 4).tolist())
print(np.round(c.grad, 4).tolist())
```

```python title=test
import numpy as np

assert np.round(e.data, 4).tolist() == [[-4.0, -4.0], [0.0, 15.0]], f"e.data sai -- dang ra {np.round(e.data, 4).tolist()}"
assert np.round(f.data, 4).tolist() == [[-6.0, -6.0], [-2.0, 13.0]], f"f.data sai -- dang ra {np.round(f.data, 4).tolist()}"
assert np.round(a.grad, 4).tolist() == [[-1.0, 2.0], [0.0, 5.0]], f"a.grad sai -- dang ra {np.round(a.grad, 4).tolist()}"
assert np.round(b.grad, 4).tolist() == [[4.0, -2.0], [1.0, 3.0]], f"b.grad sai -- dang ra {np.round(b.grad, 4).tolist()}"
assert np.round(c.grad, 4).tolist() == [[-1.0, -1.0], [-1.0, -1.0]], f"c.grad sai -- dang ra {np.round(c.grad, 4).tolist()}"

# rieng kiem tra khong bi DAO NGUOC hai ve cua phep nhan: a.data va b.data
# khac nhau nen dao ve se cho a.grad/b.grad SAI, khac han hai gia tri dung.
assert not np.array_equal(a.grad, b.grad), "a.grad va b.grad khong duoc trung nhau -- kiem tra lai cong thuc tich co bi dao ve khong"
```

:::hints
- kind: attention
  body: Ba chỗ trống. `__sub__`, vế `other`: đạo hàm cục bộ của phép trừ theo TOÁN HẠNG THỨ HAI là `-1`, cộng dồn `-out.grad` (một mảng ÂM, element-wise). `__mul__`, cả hai vế: quy tắc tích — vế `self` nhân với MẢNG `other.data`, vế `other` nhân với MẢNG `self.data`; cả hai nhân THÊM `out.grad` (element-wise, `numpy` tự làm đúng theo từng phần tử vì hai mảng CÙNG shape).
- kind: strategy
  body: '__sub__: `-out.grad`. __mul__: `other.data * out.grad` và `self.data * out.grad`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `-out.grad`, `other.data * out.grad`, và `self.data * out.grad`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: __sub__ phai cong don DAU AM (-out.grad) cho other.grad; __mul__ phai dung DUNG quy tac tich -- self.grad cong other.data*out.grad, other.grad cong self.data*out.grad, khong duoc dao nguoc hay chep hang so
  requireAst:
  - kind: uses-operator, target: "*", min: 4
  - kind: uses-operator, target: neg, min: 3
  - kind: uses-name, target: self, min: 19
  - kind: uses-name, target: other, min: 10
  # Da thu that (goi kiemAst that tren code day du cua solution, gom ca
  # phan harness a/b/c/e/f o duoi): loi giai dung dat=true, ca bon
  # requireAst qua sach. "*"=4: self.data*other.data khi tao out trong
  # __mul__, hai lan trong closure cua no (other.data*out.grad,
  # self.data*out.grad), va rieng "e = a * b" trong harness -- BON, khong
  # phai BA, vi harness cung gop vao dem. "neg"=3: mot lan "-out.grad" (that
  # su la UnaryOp USub) trong __sub__, CONG hai literal am trong harness
  # (`-2.` trong Tensor cua a, `-1.` trong Tensor cua b) -- ast.parse KHONG
  # gop hang so am thanh Constant(-2.0), no la UnaryOp(USub, Constant(2.0)),
  # nen ca hai literal nay CUNG duoc dem. self=19, other=10 tren toan bo
  # file (dinh nghia lop + harness). Cheat "other.grad += out.grad" (quen
  # dau am trong __sub__) lam "neg" tut xuong 2 (chi con hai literal am,
  # mat -out.grad) -- bi chan. Cheat "__mul__ ca hai closure chep lai cong
  # thuc cua __add__ (self.grad += out.grad, other.grad += out.grad, bo het
  # phep nhan)" lam "*" tut xuong 2 (mat ca hai phep nhan trong closure,
  # chi con self.data*other.data luc tao out va e=a*b trong harness) VA lam
  # self/other tut xuong duoi nguong -- bi chan KEP. Cheat "dao nguoc hai ve
  # tich" (swap self.data/other.data giua hai closure) KHONG doi ca bon so
  # dem static (van dung dung 4 phep nhan, 19 self, 10 other -- chi doi VI
  # TRI cua self.data/other.data, khong doi SO LUONG) -- luat static KHONG
  # bat duoc cheat nay, nhung tier tests bat DOC LAP qua assert not
  # array_equal(a.grad, b.grad) (a.data != b.data nen dao ve cho ket qua
  # THAT SU khac nhau).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[\\[-4\\.0, -4\\.0\\], \\[0\\.0, 15\\.0\\]\\]\\n\\[\\[-6\\.0, -6\\.0\\], \\[-2\\.0, 13\\.0\\]\\]\\n\\[\\[-1\\.0, 2\\.0\\], \\[0\\.0, 5\\.0\\]\\]\\n\\[\\[4\\.0, -2\\.0\\], \\[1\\.0, 3\\.0\\]\\]\\n\\[\\[-1\\.0, -1\\.0\\], \\[-1\\.0, -1\\.0\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba phép toán, y hệt công thức của `Value` — chỉ khác `numpy` làm việc
element-wise trên cả một ma trận thay vì một số. Bài sau: phép toán ma trận
THẬT SỰ khác — nhân ma trận, `A @ B`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`__mul__` của `Tensor` nhân ELEMENT-WISE — mỗi phần tử của `a` nhân với
ĐÚNG phần tử tương ứng của `b`, không trộn lẫn phần tử khác hàng/cột. Nhưng
attention cần một phép nhân KHÁC hẳn: nhân ma trận thật (`A @ B`), nơi MỖI
phần tử kết quả là TỔNG của nhiều tích — trộn thông tin CHÉO giữa các hàng
và cột. Công thức đạo hàm cục bộ của `__mul__` (`self.grad += other.data *
out.grad`) có còn đúng cho phép nhân ma trận thật không, hay cần một công
thức khác hẳn?
::::

::::checkpoint{mastery=0.8}
::::
