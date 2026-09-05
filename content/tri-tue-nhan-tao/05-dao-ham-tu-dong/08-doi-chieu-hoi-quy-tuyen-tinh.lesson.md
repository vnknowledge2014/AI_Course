---
id: tri-tue-nhan-tao.dao-ham-tu-dong.doi-chieu-hoi-quy-tuyen-tinh
title: "Đối chiếu hồi quy tuyến tính"
summary: "Lấy MSE loss của hoi-quy-tuyen-tinh-tu-so-0 (T8.1a) cho một điểm dữ liệu (diện tích 30m2, giá 780 triệu, w=20.2321, b=183.2143 đã khớp trước đó), tính gradient của loss theo w, b bằng Value.backward() thay vì công thức đạo hàm đóng viết tay: w.grad=610.638, b.grad=20.3546 — khớp CHÍNH XÁC với công thức đóng ∂MSE/∂w=2(ŷ-y)x, ∂MSE/∂b=2(ŷ-y) — bằng chứng cụ thể autograd đúng trên bài toán đã biết đáp số."
locale: vi
track: tri-tue-nhan-tao
module: dao-ham-tu-dong
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.doi-chieu-hoi-quy-tuyen-tinh]
requires: [ai.kiem-dao-ham-bang-so]
concepts: [ai.doi-chieu-hoi-quy-tuyen-tinh]
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
`Value.backward()` đã qua được phép kiểm bằng số. Giờ thử trên một bài toán
CŨ — một bài đã biết chính xác đáp án đúng, từ trước khi autograd tồn tại.
::::

::::explain{#mse_bang_value}
Bài `hoi-quy-tuyen-tinh-tu-so-0` (T8.1a) khớp một đường thẳng `y = w·x + b`
qua các điểm giá nhà, bằng công thức least-squares — một công thức ĐÓNG, suy
ra bằng đại số, không lặp. Với dữ liệu một điểm `(x, y_thật)`, sai số bình
phương trung bình (MSE — Mean Squared Error) cho một điểm chỉ còn:

> `MSE = (ŷ − y_thật)²`, với `ŷ = w·x + b` (dự đoán của mô hình).

Đạo hàm ĐÓNG của `MSE` theo `w` và `b` (công thức đại số, suy ra bằng chain
rule viết tay — đúng kiểu bài `dao-ham-nguoc-chuoi` đã làm):

> `∂MSE/∂w = 2(ŷ − y_thật)·x`
>
> `∂MSE/∂b = 2(ŷ − y_thật)`

Đây là những công thức đã BIẾT đáp án đúng từ TRƯỚC — không cần `Value`,
không cần autograd, chỉ cần đạo hàm bằng tay của một hàm bậc hai. Bài này
dùng ĐÚNG những công thức đó làm THƯỚC ĐO: tính gradient của `MSE` theo `w`,
`b` bằng `Value.backward()` (autograd), rồi so với công thức đóng — nếu
khớp, đó là bằng chứng CỤ THỂ rằng động cơ autograd đúng trên một bài toán
đã biết đáp số, không chỉ đúng trên những ví dụ nhỏ tự bịa.

Xây `MSE` bằng `Value` không khác gì xây bất kỳ biểu thức nào khác ở các
bài trước: `ŷ = w·x + b` dùng `__add__`/`__mul__`, `MSE = (ŷ − y_thật)²`
dùng `__sub__`/`__pow__` — không có phép toán MỚI nào, chỉ ráp lại những gì
đã học.
::::

::::example{#doi_chieu_mot_diem}
Điểm đầu tiên trong bảy điểm của `hoi-quy-tuyen-tinh-tu-so-0` — diện tích
`30`m², giá `780` triệu — với `w ≈ 20.2321`, `b ≈ 183.2143` đã khớp trước đó:

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

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += (other * self.data ** (other - 1)) * out.grad
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


def dw_dong(w, b, x, y_true):
    y_pred = w * x + b
    return 2 * (y_pred - y_true) * x

def db_dong(w, b, x, y_true):
    y_pred = w * x + b
    return 2 * (y_pred - y_true)


w_val, b_val = 20.2321, 183.2143
x_data, y_true = 30.0, 780.0

dw_closed = dw_dong(w_val, b_val, x_data, y_true)
db_closed = db_dong(w_val, b_val, x_data, y_true)

w = Value(w_val)
b = Value(b_val)
y_pred = w * x_data + b
loss = (y_pred - y_true) ** 2
loss.backward()

print(round(y_pred.data, 4))
print(round(loss.data, 4))
print(round(w.grad, 4), round(dw_closed, 4))
print(round(b.grad, 4), round(db_closed, 4))
```

```text title=readonly
790.1773
103.5774
610.638 610.638
20.3546 20.3546
```

`ŷ = 790.1773` (mô hình dự đoán căn nhà `30`m² giá khoảng `790.18` triệu —
gần đúng với giá thật `780`, sai số `MSE ≈ 103.58`). `w.grad` (từ
`Value.backward()`) khớp CHÍNH XÁC với `dw_closed` (công thức đóng viết
tay): cả hai đều `610.638`. `b.grad` khớp `db_closed`: cả hai đều `20.3546`.
Hai cách tính hoàn toàn độc lập — một dùng công thức đại số biết trước, một
dùng đồ thị tính toán và lan truyền ngược — cho ra CÙNG một số.
::::

::::predict{#doan_dau_gradient commitOnce}
`w.grad ≈ 610.638` (dương) và `b.grad ≈ 20.3546` (dương) — mô hình đang dự
đoán CAO HƠN giá thật (`790.18` so với `780`).

**Trước khi đọc lại**, bạn đoán: nếu điểm dữ liệu có giá THẬT cao HƠN dự
đoán của mô hình (thay vì thấp hơn như ở đây), dấu của `w.grad` và `b.grad`
sẽ đổi thế nào?

:::opt{correct}
Cả hai đổi dấu, thành ÂM — công thức `2(ŷ−y_thật)·x` và `2(ŷ−y_thật)` đều
mang dấu của `(ŷ−y_thật)`; khi dự đoán THẤP hơn giá thật, hiệu đó ÂM, nên cả
hai gradient đổi từ dương sang âm
:::

:::opt
Không đổi — dấu của gradient chỉ phụ thuộc vào `w` và `b` hiện tại, không
phụ thuộc việc dự đoán cao hay thấp hơn giá thật
::why
Gần đúng ở việc `w` và `b` đúng là CÓ ảnh hưởng tới gradient (qua `x`, qua
việc tính `ŷ`) — quan sát đó không sai hoàn toàn.

Chỗ lệch: cả hai công thức đóng đều có thừa số `(ŷ − y_thật)` — hiệu giữa dự
đoán và giá thật. Dấu của hiệu này (dự đoán cao hơn hay thấp hơn) QUYẾT ĐỊNH
trực tiếp dấu của cả `∂MSE/∂w` lẫn `∂MSE/∂b`, không phải một chi tiết phụ.
::
:::

:::opt
Chỉ `w.grad` đổi dấu, `b.grad` giữ nguyên — vì `w.grad` có thêm thừa số `x`
(có thể âm), còn `b.grad` không nhân với gì cả
::why
Gần đúng ở việc nhận ra hai công thức có HÌNH DẠNG khác nhau — `∂MSE/∂w` có
thêm thừa số `x`, `∂MSE/∂b` thì không. Quan sát về sự khác biệt hình dạng đó
không sai.

Chỗ lệch: `x = 30` (diện tích) LUÔN DƯƠNG trong bài toán này — nhân với một
số dương không đổi dấu của số đó. Cả hai công thức đều mang thừa số
`(ŷ−y_thật)` làm gốc dấu; `x` dương chỉ đổi ĐỘ LỚN của `w.grad`, không đổi
dấu của nó so với `b.grad`.
::
:::
::::

::::code{#hoan_thien_doi_chieu}
Hoàn thiện hai công thức đóng `dw_dong` và `db_dong` — đúng hai đạo hàm đã
nêu ở phần giải thích.

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


def dw_dong(w, b, x, y_true):
    y_pred = w * x + b
    return ___                  # 2 * (y_pred - y_true) * x

def db_dong(w, b, x, y_true):
    y_pred = w * x + b
    return ___                  # 2 * (y_pred - y_true)


w_val, b_val = 20.2321, 183.2143
x_data, y_true = 30.0, 780.0

dw_closed = dw_dong(w_val, b_val, x_data, y_true)
db_closed = db_dong(w_val, b_val, x_data, y_true)

w = Value(w_val)
b = Value(b_val)
y_pred = w * x_data + b
loss = (y_pred - y_true) ** 2
loss.backward()

print(round(y_pred.data, 4))
print(round(loss.data, 4))
print(round(w.grad, 4), round(dw_closed, 4))
print(round(b.grad, 4), round(db_closed, 4))
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

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += (other * self.data ** (other - 1)) * out.grad
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


def dw_dong(w, b, x, y_true):
    y_pred = w * x + b
    return 2 * (y_pred - y_true) * x

def db_dong(w, b, x, y_true):
    y_pred = w * x + b
    return 2 * (y_pred - y_true)


w_val, b_val = 20.2321, 183.2143
x_data, y_true = 30.0, 780.0

dw_closed = dw_dong(w_val, b_val, x_data, y_true)
db_closed = db_dong(w_val, b_val, x_data, y_true)

w = Value(w_val)
b = Value(b_val)
y_pred = w * x_data + b
loss = (y_pred - y_true) ** 2
loss.backward()

print(round(y_pred.data, 4))
print(round(loss.data, 4))
print(round(w.grad, 4), round(dw_closed, 4))
print(round(b.grad, 4), round(db_closed, 4))
```

```python title=test
assert round(y_pred.data, 4) == 790.1773, f"y_pred sai -- dang ra {round(y_pred.data, 4)}"
assert round(loss.data, 4) == 103.5774, f"loss sai -- dang ra {round(loss.data, 4)}"
assert round(w.grad, 4) == 610.638, f"w.grad sai -- dang ra {round(w.grad, 4)}"
assert round(dw_closed, 4) == 610.638, f"dw_closed sai -- dang ra {round(dw_closed, 4)}"
assert round(b.grad, 4) == 20.3546, f"b.grad sai -- dang ra {round(b.grad, 4)}"
assert round(db_closed, 4) == 20.3546, f"db_closed sai -- dang ra {round(db_closed, 4)}"

# ban chat cua bai nay: autograd va cong thuc dong phai khop nhau -- sai lech
# duoi 1e-6 (khong dung == vi ca hai deu la so thuc).
assert abs(w.grad - dw_closed) < 1e-6, f"w.grad va dw_closed phai khop -- {w.grad} vs {dw_closed}"
assert abs(b.grad - db_closed) < 1e-6, f"b.grad va db_closed phai khop -- {b.grad} vs {db_closed}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, hai công thức đóng đã nêu ở phần giải thích. `dw_dong`: `2·(ŷ−y_thật)·x` — nhân thêm `x` so với `db_dong`. `db_dong`: `2·(ŷ−y_thật)` — không nhân thêm gì. Cả hai đều dùng biến `y_pred` đã tính sẵn ở dòng trên, không tính lại `w*x+b`.
- kind: strategy
  body: 'dw_dong: `2 * (y_pred - y_true) * x`. db_dong: `2 * (y_pred - y_true)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `2 * (y_pred - y_true) * x` và `2 * (y_pred - y_true)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: dw_dong va db_dong phai tinh THAT tu y_pred/y_true/x (khong duoc chep san 610.638/20.3546) -- dw_dong phai nhan them x, db_dong thi khong
  requireAst:
  - kind: uses-name, target: y_pred, min: 4
  - kind: uses-operator, target: "*", min: 11
  - kind: uses-operator, target: "-", min: 5
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca ba luat qua sach (y_pred=4: doc trong dw_dong, trong
  # db_dong, trong print, va trong loss=(y_pred-y_true)**2; *=11: gom nhieu
  # phep nhan trong Value.__mul__/__pow__ va hai cho trong hai cong thuc
  # dong; -=5: gom (y_pred-y_true) o ca hai cong thuc dong, __sub__'s than,
  # loss's dong, va (other-1) trong __pow__). Cheat "chep san ca hai cong
  # thuc thanh hang so" lam "y_pred" tut ve 2, "*" tut duoi 11, "-" tut duoi
  # 5 -- bi chan boi ca ba luat cung luc. Doi chieu bang gia tri THAT (khong
  # phai chi so sanh voi mot hang so co dinh) la phong thu manh nhat cua bai
  # nay: autograd va cong thuc dong phai TU KHOP nhau, khong chi khop voi
  # mot dap an da biet truoc.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^790\\.1773\\n103\\.5774\\n610\\.638 610\\.638\\n20\\.3546 20\\.3546\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Value.backward()` khớp CHÍNH XÁC công thức đóng — cả `w.grad` lẫn
`b.grad`. Autograd đúng, không chỉ trên ví dụ tự bịa mà trên một bài toán
đã biết đáp số từ T8.1a.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài này chỉ dùng MỘT điểm dữ liệu. `hoi-quy-tuyen-tinh-tu-so-0` khớp đường
thẳng qua BẢY điểm cùng lúc, bằng công thức đóng cho CẢ BẢY. Nếu muốn dùng
`Value`/`.backward()` cho cả bảy điểm — tính `MSE` trung bình rồi lấy đạo
hàm theo `w`, `b` — đồ thị tính toán sẽ cần bao nhiêu node so với chỉ một
điểm, và điều đó có phải vấn đề cho autograd không?
::::

::::checkpoint{mastery=0.85}
::::
