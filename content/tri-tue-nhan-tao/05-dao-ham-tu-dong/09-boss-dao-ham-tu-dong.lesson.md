---
id: tri-tue-nhan-tao.dao-ham-tu-dong.boss-dao-ham-tu-dong
title: "BOSS — Động cơ đạo hàm tự động"
summary: "Ráp động cơ Value ĐẦY ĐỦ (+, -, *, **, tanh, relu, backward với sắp xếp tô-pô và cộng dồn gradient) rồi chạy gradient-check trên MỘT biểu thức phức hợp MỚI, tái sử dụng biến — L=tanh(relu(a·b+c)·a - c²), a=2.0,b=1.5,c=-1.3: L≈0.9366, gradient (a,b,c)≈(0.5766,0.4908,0.5644), khớp đạo hàm số trong sai số 1e-6. Đóng q8.2b — động cơ này q8.2c tái dùng để huấn luyện Layer/MLP thật."
locale: vi
track: tri-tue-nhan-tao
module: dao-ham-tu-dong
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-dao-ham-tu-dong]
requires: [ai.doi-chieu-hoi-quy-tuyen-tinh]
concepts: [ai.boss-dao-ham-tu-dong]
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
Tám bài, từng mảnh một: đồ thị, đạo hàm tay, `Value`, mũ/`tanh`/`ReLU`, sắp
xếp tô-pô, cộng dồn, kiểm bằng số, đối chiếu hồi quy. Giờ ráp lại thành MỘT
động cơ, và thử nó trên một biểu thức chưa từng thấy.
::::

::::explain{#dong_co_day_du}
`Value` đầy đủ gồm ĐÚNG những gì tám bài trước đã xây, không thêm gì mới:

> **Bốn phép toán số học** (`__add__`, `__sub__`, `__mul__`, `__pow__`) —
> mỗi cái với `_backward` riêng, CỘNG DỒN (`+=`) vào gradient đã có (bài
> `lop-value-cong-tru-nhan`, `lop-value-mu-tanh-relu`, `tich-luy-gradient`).
>
> **Hai hàm kích hoạt** (`tanh`, `relu`) — tái dùng công thức đạo hàm đã học
> ở `ham-kich-hoat` (q8.2a), với `relu` dùng đúng biên `self.data > 0`
> (bài `lop-value-mu-tanh-relu`).
>
> **`backward()`** — sắp xếp tô-pô từ node đầu ra, seed `grad = 1.0`, rồi
> duyệt NGƯỢC thứ tự đó (bài `sap-xep-to-po-va-lan-truyen-nguoc`).

Đây là NGUYÊN VẸN "micrograd" — một động cơ autograd vô hướng đủ để tính
đạo hàm của BẤT KỲ biểu thức nào ráp từ sáu phép toán trên, dù ngắn hay dài,
dù biến dùng một lần hay lặp lại nhiều lần. Không có phép toán "ma trận",
không có "tensor nhiều chiều", không có gì phình to hơn đúng những gì tám
bài vừa học cần — track này CỐ Ý giữ nó nhỏ.

Phép kiểm cuối cùng, đúng kỹ thuật của bài `kiem-dao-ham-bang-so`: chạy
`Value.backward()` VÀ đạo hàm số (finite difference) trên MỘT biểu thức
PHỨC HỢP CHƯA TỪNG DÙNG trong track này — nhiều phép toán khác nhau, biến
tái sử dụng ít nhất một lần — rồi đối chiếu. Khớp nghĩa là mọi mảnh (bốn
phép số học, hai hàm kích hoạt, sắp xếp tô-pô, cộng dồn) hoạt động ĐÚNG và
NHẤT QUÁN với nhau, không chỉ đúng riêng lẻ từng mảnh.
::::

::::example{#gradient_check_bieu_thuc_moi}
Biểu thức MỚI: `e = a·b`, `f = e + c`, `g = relu(f)`, `h = g·a − c²`,
`L = tanh(h)` — với `a=2.0, b=1.5, c=-1.3` (`a` xuất hiện HAI lần: trong
`a·b` và trong `g·a`), chạy CẢ động cơ lẫn đạo hàm số:

```python title=readonly
import math

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

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += (other * self.data ** (other - 1)) * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
        out._backward = _backward
        return out

    def relu(self):
        out = Value(0.0 if self.data < 0 else self.data, (self,), 'relu')
        def _backward():
            self.grad += (1.0 if self.data > 0 else 0.0) * out.grad
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
        self.grad = 1.0
        for node in reversed(topo):
            node._backward()


def bieu_thuc_thuan(a, b, c):
    e = a * b
    f = e + c
    g = max(0.0, f)
    h = g * a - c ** 2
    return math.tanh(h)


def dao_ham_so(f, tham_so, i, h=1e-4):
    cong = list(tham_so); cong[i] += h
    tru = list(tham_so); tru[i] -= h
    return (f(*cong) - f(*tru)) / (2 * h)


a0, b0, c0 = 2.0, 1.5, -1.3

a = Value(a0)
b = Value(b0)
c = Value(c0)
e = a * b
f = e + c
g = f.relu()
h = g * a - c ** 2
L = h.tanh()
L.backward()

grad_tu_dong = [a.grad, b.grad, c.grad]
grad_so = [dao_ham_so(bieu_thuc_thuan, (a0, b0, c0), i) for i in range(3)]

print(round(L.data, 4))
print([round(x, 4) for x in grad_tu_dong])
print([round(x, 4) for x in grad_so])
```

```text title=readonly
0.9366
[0.5766, 0.4908, 0.5644]
[0.5766, 0.4908, 0.5644]
```

`L ≈ 0.9366`. Gradient theo `a`, `b`, `c` — tính bằng `Value.backward()`
(đi qua ĐÚNG `_backward` của `*`, `+`, `relu`, `-`, `**`, `tanh`, và cộng dồn
đúng hai đóng góp của `a`) — khớp với đạo hàm số (không hề dùng lại một dòng
code nào của `Value`), làm tròn `4` chữ số: cả hai đều `[0.5766, 0.4908,
0.5644]`. Đây là một biểu thức CHƯA từng xuất hiện ở tám bài trước, với
`relu` ở giữa chuỗi (không chỉ ở lá) và `a` tái sử dụng — đúng những tình
huống mà TỪNG bài riêng lẻ đã kiểm, giờ kiểm CÙNG LÚC.
::::

::::predict{#doan_neu_doi_c commitOnce}
Biểu thức trên có `f = a·b + c = 2.0×1.5 + (-1.3) = 1.7` — DƯƠNG, nên
`g = relu(f) = f` (nhánh dương, không bị cắt).

**Trước khi đọc lại**, bạn đoán: nếu đổi `c` thành một số RẤT ÂM (ví dụ
`c = -10.0`, khiến `f = a·b + c` thành ÂM), gradient của `b` (`∂L/∂b`) sẽ
thay đổi thế nào so với `0.4908` ở trên?

:::opt{correct}
`∂L/∂b` sẽ trở thành `0.0` — khi `f` âm, `g = relu(f) = 0` VÀ đạo hàm cục bộ
của `relu` cũng là `0` tại nhánh đó; vì `b` CHỈ ảnh hưởng tới `L` thông qua
`e → f → g` (không có đường nào khác từ `b` tới `L`), toàn bộ đường đó bị
CẮT, nên gradient của `b` triệt tiêu hoàn toàn
:::

:::opt
`∂L/∂b` sẽ đổi dấu (thành số âm), vì `f` chuyển từ dương sang âm
::why
Gần đúng ở việc nhận ra `f` đổi dấu ảnh hưởng tới `g` — quan sát về NGUYÊN
NHÂN không sai.

Chỗ lệch: khi `relu` cắt một nhánh (đầu vào âm), nó không "đảo dấu" gradient
đi qua — nó CHẶN ĐỨNG hoàn toàn (đạo hàm cục bộ bằng `0`, không phải `-1`).
`b` chỉ ảnh hưởng `L` qua đúng con đường bị chặn đó, nên gradient của nó về
`0`, không đổi dấu.
::
:::

:::opt
Không đổi — `relu` chỉ ảnh hưởng tới GIÁ TRỊ của `g`, không ảnh hưởng tới
gradient lan qua nó
::why
Gần đúng ở việc phân biệt "giá trị" và "gradient" là hai thứ khác nhau —
sự phân biệt đó không sai ở nhiều ngữ cảnh.

Chỗ lệch: với `relu` cụ thể, GIÁ TRỊ và GRADIENT của nhánh âm đi CÙNG một
điều kiện (`self.data ≤ 0`) — cả hai đều về `0` khi đầu vào không dương.
`relu` không phải một phép toán "trong suốt" với gradient; nó là lý do
chính khiến một neuron ReLU có thể "chết" (ngừng nhận gradient) khi đầu vào
luôn âm — đúng khái niệm đã đo ở bài `ham-kich-hoat` (q8.2a).
::
:::
::::

::::code{#hoan_thien_dong_co_boss}
Hoàn thiện bốn mảnh cuối của động cơ: công thức đạo hàm cục bộ của `__pow__`
và của `relu`, chiều duyệt của `backward()`, và công thức sai phân trung
tâm của `dao_ham_so`.

```python title=starter
import math

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

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += ___             # (other * self.data ** (other - 1)) * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
        out._backward = _backward
        return out

    def relu(self):
        out = Value(0.0 if self.data < 0 else self.data, (self,), 'relu')
        def _backward():
            self.grad += ___             # (1.0 if self.data > 0 else 0.0) * out.grad
        out._backward = _backward
        return out

    def backward(self):
        topo = []
        visited = set()
        def build(v):
            if v not in visited:
                visited.add(v)
                for con in v._prev:
                    build(con)
                topo.append(v)
        build(self)
        self.grad = 1.0
        for node in ___(topo):           # reversed
            node._backward()


def bieu_thuc_thuan(a, b, c):
    e = a * b
    f = e + c
    g = max(0.0, f)
    h = g * a - c ** 2
    return math.tanh(h)


def dao_ham_so(f, tham_so, i, h=1e-4):
    cong = list(tham_so); cong[i] += h
    tru = list(tham_so); tru[i] -= h
    return ___                            # (f(*cong) - f(*tru)) / (2 * h)


a0, b0, c0 = 2.0, 1.5, -1.3

a = Value(a0)
b = Value(b0)
c = Value(c0)
e = a * b
f = e + c
g = f.relu()
h = g * a - c ** 2
L = h.tanh()
L.backward()

grad_tu_dong = [a.grad, b.grad, c.grad]
grad_so = [dao_ham_so(bieu_thuc_thuan, (a0, b0, c0), i) for i in range(3)]
khop = [abs(grad_tu_dong[i] - grad_so[i]) < 1e-3 for i in range(3)]

a4 = Value(0.0)
r4 = a4.relu()
r4.grad = 1.0
r4._backward()

print(round(L.data, 4))
print([round(x, 4) for x in grad_tu_dong])
print([round(x, 4) for x in grad_so])
print(all(khop))
print(round(a4.grad, 4))
```

```python title=solution
import math

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

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += (other * self.data ** (other - 1)) * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
        out._backward = _backward
        return out

    def relu(self):
        out = Value(0.0 if self.data < 0 else self.data, (self,), 'relu')
        def _backward():
            self.grad += (1.0 if self.data > 0 else 0.0) * out.grad
        out._backward = _backward
        return out

    def backward(self):
        topo = []
        visited = set()
        def build(v):
            if v not in visited:
                visited.add(v)
                for con in v._prev:
                    build(con)
                topo.append(v)
        build(self)
        self.grad = 1.0
        for node in reversed(topo):
            node._backward()


def bieu_thuc_thuan(a, b, c):
    e = a * b
    f = e + c
    g = max(0.0, f)
    h = g * a - c ** 2
    return math.tanh(h)


def dao_ham_so(f, tham_so, i, h=1e-4):
    cong = list(tham_so); cong[i] += h
    tru = list(tham_so); tru[i] -= h
    return (f(*cong) - f(*tru)) / (2 * h)


a0, b0, c0 = 2.0, 1.5, -1.3

a = Value(a0)
b = Value(b0)
c = Value(c0)
e = a * b
f = e + c
g = f.relu()
h = g * a - c ** 2
L = h.tanh()
L.backward()

grad_tu_dong = [a.grad, b.grad, c.grad]
grad_so = [dao_ham_so(bieu_thuc_thuan, (a0, b0, c0), i) for i in range(3)]
khop = [abs(grad_tu_dong[i] - grad_so[i]) < 1e-3 for i in range(3)]

a4 = Value(0.0)
r4 = a4.relu()
r4.grad = 1.0
r4._backward()

print(round(L.data, 4))
print([round(x, 4) for x in grad_tu_dong])
print([round(x, 4) for x in grad_so])
print(all(khop))
print(round(a4.grad, 4))
```

```python title=test
assert round(L.data, 4) == 0.9366, f"L.data sai -- dang ra {round(L.data, 4)}"
assert [round(x, 4) for x in grad_tu_dong] == [0.5766, 0.4908, 0.5644], f"grad_tu_dong sai -- dang ra {[round(x, 4) for x in grad_tu_dong]}"
assert [round(x, 4) for x in grad_so] == [0.5766, 0.4908, 0.5644], f"grad_so sai -- dang ra {[round(x, 4) for x in grad_so]}"
assert all(khop), f"autograd va dao ham so phai khop trong sai so 1e-3 -- khop={khop}"

# rieng kiem tra BIEN z=0 cua relu (dung '>' khong phai '>='): neu doi thanh
# '>=', a4.grad se ra 1.0 thay vi 0.0.
assert round(a4.grad, 4) == 0.0, f"a4.grad (bien z=0 cua relu) phai la 0.0 -- dang ra {round(a4.grad, 4)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống, mỗi cái đã xuất hiện ở một bài trước trong quest. `__pow__` (bài `lop-value-mu-tanh-relu`): quy tắc luỹ thừa `other * self.data ** (other - 1)`. `relu` (cùng bài đó): điều kiện `self.data > 0` — nghiêm ngặt. `backward` (bài `sap-xep-to-po-va-lan-truyen-nguoc`): duyệt `reversed(topo)`. `dao_ham_so` (bài `kiem-dao-ham-bang-so`): `(f(*cong) - f(*tru)) / (2 * h)`.
- kind: strategy
  body: '__pow__: `(other * self.data ** (other - 1)) * out.grad`. relu: `(1.0 if self.data > 0 else 0.0) * out.grad`. backward: `reversed`. dao_ham_so: `(f(*cong) - f(*tru)) / (2 * h)`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là công thức luỹ thừa của `__pow__`, điều kiện `self.data > 0` của `relu`, `reversed` trong `backward`, và công thức sai phân trung tâm của `dao_ham_so`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: __pow__ phai dung quy tac luy thua that (**); relu phai so sanh self.data voi 0 bang '>'; backward phai duyet REVERSED(topo); dao_ham_so phai goi THAT f hai lan roi tru/chia -- khong duoc chep san bat ky ket qua nao
  requireAst:
  - kind: uses-operator, target: "**", min: 5
  - kind: uses-operator, target: ">", min: 1
  - kind: uses-call, target: reversed, min: 1
  - kind: uses-name, target: topo, min: 2
  - kind: uses-call, target: f, min: 2
  - kind: uses-operator, target: "/", min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, tat ca sau luat qua sach ("**"=5: self.data**other [tao
  # out] va self.data**(other-1) trong __pow__, t**2 trong tanh, c**2 va
  # a**2-tuong-duong o phan harness -- thuc te la c**2 trong h=g*a-c**2;
  # ">"=1: rieng trong relu; reversed=1; topo=2; f=2: f(*cong), f(*tru);
  # "/"=1). Cheat "__pow__ chep san 2*self.data thay vi dung **" lam "**"
  # tut duoi 5 -- bi chan. Cheat "relu dung >= thay vi >" lam ">" ve 0 -- bi
  # chan CA static LAN test rieng (assert a4.grad, doc lap). Cheat "backward
  # khong dao nguoc" lam "reversed" ve 0 -- bi chan, VA tu kiem chung bang
  # Pyodide that: bo dao nguoc lam ca ba gradient ve gan 0, khac han [0.5766,
  # 0.4908, 0.5644] -- bi chan doc lap boi test. Cheat "dao_ham_so chi goi f
  # mot lan" lam "f" tut ve 1 -- bi chan.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^0\\.9366\\n\\[0\\.5766, 0\\.4908, 0\\.5644\\]\\n\\[0\\.5766, 0\\.4908, 0\\.5644\\]\\nTrue\\n0\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Value` đầy đủ — bốn phép số học, hai hàm kích hoạt, sắp xếp tô-pô, cộng dồn
gradient — vượt qua gradient-check trên một biểu thức chưa từng thấy. q8.2b
khép lại tại đây.
::::

::::reflect{#nghi-lai}
Chín bài, một động cơ: `do-thi-tinh-toan` biểu diễn biểu thức thành node có
node cha. `dao-ham-nguoc-chuoi` tính tay bằng chain rule, thấy nó không
scale. `lop-value-cong-tru-nhan` gói công thức đạo hàm cục bộ vào `_backward`
của từng phép toán. `lop-value-mu-tanh-relu` thêm mũ và hai hàm kích hoạt.
`sap-xep-to-po-va-lan-truyen-nguoc` tự động hoá thứ tự gọi, thay vì tự tay
liệt kê. `tich-luy-gradient` sửa đúng lỗ hổng nguy hiểm nhất — biến dùng lặp
lại. `kiem-dao-ham-bang-so` cho một cách soát lỗi không cần biết trước đáp
án. `doi-chieu-hoi-quy-tuyen-tinh` xác nhận autograd khớp một bài toán đã
biết đáp số. Bài này ráp tất cả thành một động cơ, kiểm nó trên một biểu
thức mới.

`Value`/`.backward()` xong — nhưng nó vẫn tách biệt hoàn toàn với `Layer`/
`MLP` của `q8.2a`. Trọng số của mọi `Layer` vẫn là số Python trần, không
phải `Value`. Câu hỏi còn để ngỏ: điều gì xảy ra khi `W`, `b` của một
`Layer` được bọc thành `Value`, và `Layer.forward` chạy xuyên qua động cơ
này — cập nhật trọng số bằng ĐÚNG gradient nó vừa tính ra? `q8.2c` trả lời
câu hỏi đó.
::::

::::checkpoint{mastery=0.85}
::::
