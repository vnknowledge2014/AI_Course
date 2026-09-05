---
id: tri-tue-nhan-tao.dao-ham-tu-dong.kiem-dao-ham-bang-so
title: "Kiểm đạo hàm bằng số"
summary: "Gradient checking bằng finite difference: (f(x+h)-f(x-h))/(2h) xấp xỉ đạo hàm CHỈ CẦN forward pass lặp lại, không cần đồ thị/backward. Trên y=tanh(x*x+x) tại x=0.6 (x dùng lại 2 lần): đạo hàm số ≈0.9813, đạo hàm Value.backward() ≈0.9813, lệch dưới 1e-3 — công cụ soát lỗi chuẩn cho MỌI autograd, không cần biết trước đáp án đúng."
locale: vi
track: tri-tue-nhan-tao
module: dao-ham-tu-dong
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.kiem-dao-ham-bang-so]
requires: [ai.tich-luy-gradient]
concepts: [ai.kiem-dao-ham-bang-so]
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
Bài trước biết trước đáp án đúng vì tự tính tay được. Với một biểu thức
phức tạp hơn, tính tay không còn khả thi — cần một cách khác để soát lỗi.
::::

::::explain{#finite_difference}
Định nghĩa đạo hàm (từ giải tích): `f'(x)` đo `f` thay đổi nhanh tới đâu
quanh `x` — xấp xỉ bằng cách nhích `x` một khoảng RẤT NHỎ `h` rồi xem `f`
đổi bao nhiêu. Cách xấp xỉ ổn định nhất, dùng CẢ HAI phía của `x`, gọi là
**sai phân trung tâm** (central finite difference):

> `f'(x) ≈ (f(x + h) − f(x − h)) / (2h)`

với `h` một số RẤT NHỎ (thường `1e-4` tới `1e-6` — đủ nhỏ để xấp xỉ chính
xác, nhưng không quá nhỏ tới mức sai số làm tròn của số thực (floating
point) làm hỏng phép trừ).

Điểm mấu chốt: công thức này CHỈ CẦN gọi `f` — một hàm PYTHON THUẦN, không
cần đồ thị, không cần `Value`, không cần `_backward` hay `.backward()` gì
cả. Chỉ hai lần forward pass (`f(x+h)` và `f(x-h)`), rồi một phép trừ và
một phép chia. Đây chính là kỹ thuật **kiểm đạo hàm bằng số** (gradient
checking / numerical gradient check): tính đạo hàm hai cách ĐỘC LẬP — một
cách bằng `Value.backward()` (autograd, nhanh nhưng có thể có lỗi trong
_LOGIC_ code), một cách bằng sai phân số (chậm hơn nhiều lần, nhưng KHÔNG hề
dùng lại bất kỳ dòng code nào của `Value` — nên nếu `Value` có lỗi, kiểm
bằng số vẫn cho đáp án đúng độc lập) — rồi ĐỐI CHIẾU hai kết quả. Nếu lệch
nhau QUÁ một ngưỡng nhỏ (ví dụ `1e-3`), autograd có lỗi ở đâu đó.

Đây là công cụ soát lỗi CHUẨN cho mọi thư viện autograd thật (PyTorch,
TensorFlow đều có `gradcheck` tương tự) — vì khi biểu thức đủ phức tạp,
không ai còn tính tay đối chiếu được nữa.
::::

::::example{#kiem_tren_bieu_thuc_tai_su_dung}
`y = tanh(x·x + x)` tại `x = 0.6` — biểu thức có TÁI SỬ DỤNG biến (`x` xuất
hiện hai lần, đúng kiểu gotcha của bài `tich-luy-gradient`), kiểm bằng CẢ HAI
cách:

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

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
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


def f_thuan(x):
    return math.tanh(x * x + x)


def dao_ham_so(f, x, h=1e-4):
    return (f(x + h) - f(x - h)) / (2 * h)


x0 = 0.6
grad_so = dao_ham_so(f_thuan, x0)

xv = Value(x0)
y = (xv * xv + xv).tanh()
y.backward()
grad_tu_dong = xv.grad

print(round(y.data, 6))
print(round(grad_so, 4), round(grad_tu_dong, 4))
print(abs(grad_so - grad_tu_dong) < 1e-3)
```

```text title=readonly
0.744277
0.9813 0.9813
True
```

`dao_ham_so` không hề chạm vào `Value` — nó chỉ gọi `f_thuan` (một hàm
`math.tanh` thuần) hai lần, tại `x+h` và `x-h`. `grad_tu_dong` đến từ
`Value.backward()` — một con đường tính TOÀN KHÁC. Hai con đường độc lập cho
CÙNG một số, làm tròn `4` chữ số thập phân: `0.9813`. Đây là bằng chứng cụ
thể rằng `Value` (kể cả phần cộng dồn gradient khi `x` dùng lại) đang tính
đúng, không chỉ "trông có vẻ đúng".
::::

::::predict{#doan_neu_h_qua_lon commitOnce}
Ví dụ trên dùng `h = 1e-4` — một số rất nhỏ.

**Trước khi đọc lại**, bạn đoán: nếu đổi `h` thành một số LỚN hơn nhiều
(ví dụ `1.0`, không còn "rất nhỏ" nữa), `dao_ham_so` có còn cho kết quả gần
`0.9813` không?

:::opt{correct}
Không chắc — công thức sai phân trung tâm chỉ XẤP XỈ đạo hàm THẬT khi `h`
đủ nhỏ; với `h` lớn, `f(x+h)` và `f(x-h)` đo độ dốc TRUNG BÌNH trên một
khoảng rộng, có thể khác khá xa độ dốc TẠI ĐÚNG điểm `x`
:::

:::opt
Có — công thức `(f(x+h)-f(x-h))/(2h)` luôn cho đúng đạo hàm tại `x`, bất kể
`h` lớn hay nhỏ, vì đó là một công thức TOÁN chính xác
::why
Gần đúng ở việc công thức này đúng là một công thức toán CÓ THẬT, không
phải bịa ra — quan sát về nguồn gốc công thức không sai.

Chỗ lệch: công thức sai phân trung tâm là một PHÉP XẤP XỈ, chính xác tuyệt
đối chỉ khi `h → 0` (giới hạn toán học). Với `h` hữu hạn, nó đo độ dốc TRUNG
BÌNH của `f` trên đoạn `[x-h, x+h]` — càng gần đúng đạo hàm TẠI `x` khi đoạn
đó càng hẹp (h càng nhỏ). `h = 1.0` không còn "rất nhỏ" so với biểu thức
đang xét, nên xấp xỉ có thể lệch đáng kể.
::
:::

:::opt
Không xác định được nếu không chạy thử — độ chính xác của `h` phụ thuộc
hoàn toàn vào hàm `f` cụ thể, không có quy luật chung
::why
Gần đúng ở việc GIÁ TRỊ CHÍNH XÁC của độ lệch (bao nhiêu phần trăm) đúng là
phụ thuộc vào hình dạng cụ thể của `f` — quan sát đó không sai.

Chỗ lệch: dù độ lệch CHÍNH XÁC phụ thuộc vào `f`, có một quy luật CHUNG áp
dụng cho MỌI hàm trơn (smooth): sai phân trung tâm hội tụ về đạo hàm thật
khi `h` tiến về `0`, và sai số của nó tăng theo `h` (thường theo bậc `h²`).
Không cần biết `f` cụ thể để biết `h` LỚN luôn RỦI RO xấp xỉ kém hơn `h`
nhỏ — chỉ không biết trước LỚN tới đâu thì mới đáng lo.
::
:::
::::

::::code{#hoan_thien_dao_ham_so}
Hoàn thiện `dao_ham_so`: công thức sai phân trung tâm.

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

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
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


def f_thuan(x):
    return math.tanh(x * x + x)


def dao_ham_so(f, x, h=1e-4):
    return ___                       # (f(x + h) - f(x - h)) / (2 * h)


x0 = 0.6
grad_so = dao_ham_so(f_thuan, x0)

xv = Value(x0)
y = (xv * xv + xv).tanh()
y.backward()
grad_tu_dong = xv.grad

print(round(y.data, 6))
print(round(grad_so, 4), round(grad_tu_dong, 4))
print(abs(grad_so - grad_tu_dong) < 1e-3)
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

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
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


def f_thuan(x):
    return math.tanh(x * x + x)


def dao_ham_so(f, x, h=1e-4):
    return (f(x + h) - f(x - h)) / (2 * h)


x0 = 0.6
grad_so = dao_ham_so(f_thuan, x0)

xv = Value(x0)
y = (xv * xv + xv).tanh()
y.backward()
grad_tu_dong = xv.grad

print(round(y.data, 6))
print(round(grad_so, 4), round(grad_tu_dong, 4))
print(abs(grad_so - grad_tu_dong) < 1e-3)
```

```python title=test
assert round(y.data, 6) == 0.744277, f"y.data sai -- dang ra {round(y.data, 6)}"
assert round(grad_so, 4) == 0.9813, f"grad_so sai -- dang ra {round(grad_so, 4)}"
assert round(grad_tu_dong, 4) == 0.9813, f"grad_tu_dong sai -- dang ra {round(grad_tu_dong, 4)}"
assert abs(grad_so - grad_tu_dong) < 1e-3, f"hai cach tinh dao ham lech qua nhieu -- {grad_so} vs {grad_tu_dong}"

# rieng kiem tra cong thuc dao_ham_so tren mot ham KHAC (f(x)=x**3 tai x=2.0,
# dao ham that = 3*x**2 = 12.0) -- chan cheat "chep san cong thuc rieng cho
# f_thuan" hay dao nguoc dau/sai he so.
assert round(dao_ham_so(lambda x: x ** 3, 2.0), 4) == 12.0, f"dao_ham_so tren x**3 tai x=2.0 sai -- dang ra {round(dao_ham_so(lambda x: x ** 3, 2.0), 4)}"
```

:::hints
- kind: attention
  body: Một chỗ trống, đúng công thức sai phân trung tâm đã nêu ở phần giải thích — `(f(x+h) - f(x-h)) / (2*h)`. Gọi `f` (tham số của hàm, không phải `f_thuan` trực tiếp) hai lần, tại `x+h` và `x-h`.
- kind: strategy
  body: '`(f(x + h) - f(x - h)) / (2 * h)`.'
- kind: one-line
  body: 'Chỗ trống là `(f(x + h) - f(x - h)) / (2 * h)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: dao_ham_so phai goi THAT f hai lan (tai x+h va x-h), roi tru va chia cho 2h -- khong duoc chep san cong thuc rieng cho f_thuan hay tra ve hang so
  requireAst:
  - kind: uses-call, target: f, min: 2
  - kind: uses-name, target: h, min: 3
  - kind: uses-operator, target: "/", min: 1
  - kind: uses-operator, target: "-", min: 3
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (f=2: f(x+h) va f(x-h); h=3: trong
  # x+h, x-h, va 2*h; "/"=1: rieng phep chia ngoai cung; "-"=3: gom ca "1 -
  # t**2" trong tanh's backward va (other-1) trong ... khong, o day khong co
  # pow -- gom tu f(x+h)-f(x-h) [1] va x-h [1] va "1 - t**2" trong tanh
  # [1] = 3). Cheat "dao_ham_so chi goi f mot lan roi nhan he so" lam "f" tut
  # ve 1 -- bi chan. Cheat "chep san cong thuc rieng, khong dung h" lam "h"
  # tut duoi 3 -- bi chan. Da tu kiem chung bang Pyodide/python that: bon
  # dot bien co the (doi + thanh -, doi - thanh +, doi * thanh +, doi / thanh
  # *) deu cho ket qua CACH XA gia tri dung (0.0, 7443.7, ~0.0000981,
  # ~0.0000000392 so voi dung 0.9813) -- bi chan boi ca assert gia tri lan
  # assert doi chieu abs(grad_so - grad_tu_dong) < 1e-3.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^0\\.744277\\n0\\.9813 0\\.9813\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cách tính độc lập, cùng ra `0.9813`. `dao_ham_so` không cần biết
`Value` tồn tại — và chính vì thế nó soát lỗi được `Value`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`dao_ham_so` cần gọi `f` hai lần CHO MỖI biến cần đạo hàm — với một biểu
thức có `1000` tham số (một mạng nơ-ron thật có thể có hàng triệu), kiểm
bằng số cho TOÀN BỘ tham số sẽ tốn `2000` lần forward pass. Autograd
(`Value.backward()`) có tốn tương tự vậy không, hay rẻ hơn nhiều — và điều
đó có ảnh hưởng gì tới việc dùng công cụ nào để HUẤN LUYỆN, so với dùng công
cụ nào để CHỈ SOÁT LỖI một lần?
::::

::::checkpoint{mastery=0.8}
::::
