---
id: tri-tue-nhan-tao.huan-luyen-mang-no-ron.qua-khop-mang-no-ron
title: "Quá khớp trên một mạng nơ-ron thật"
summary: "MLP 1-4-1 (13 tham số, tanh ẩn) huấn luyện bằng Adam (lr=0.25) trên 4 điểm train (x²+nhiễu nhỏ) và đo trên 3 điểm val (x² thật, không nhiễu). Epoch 20: train=1.1722, val=1.6477 — cả hai còn hợp lý. Epoch 30 (chỉ mười epoch sau): train=0.2771 (giảm hơn 4 lần) nhưng val=15.5632 (TĂNG gần 9,5 lần so với epoch 20) — chữ ký kinh điển của quá khớp, lộ ra RẤT NHANH trên một mạng nhỏ nhớ được từng điểm train, nối lại bias-variance của T8.1a."
locale: vi
track: tri-tue-nhan-tao
module: huan-luyen-mang-no-ron
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.qua-khop-mang-no-ron]
requires: [ai.adam-toi-uu-hoa]
concepts: [ai.qua-khop-mang-no-ron]
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
Ba bài vừa qua thi nhau tối ưu — SGD, động lượng, Adam, ai hội tụ nhanh
hơn. Nhưng "hội tụ nhanh" và "học được điều ĐÚNG" là hai câu hỏi khác nhau.
Bài này đo câu hỏi thứ hai, trên một mạng nơ-ron thật.
::::

::::explain{#qua_khop_mang_no_ron}
Bài `overfitting-va-do-phuc-tap` (T8.1a) đã chứng minh hiện tượng này trên
đa thức: một mô hình có QUÁ NHIỀU tự do so với lượng dữ liệu sẽ luồn qua
từng hạt nhiễu của tập train, thay vì học quy luật thật đứng sau nó. Mạng
nơ-ron không tránh khỏi hiện tượng đó — chỉ là "quá nhiều tự do" giờ nghĩa
là quá nhiều THAM SỐ (nhiều neuron, nhiều tầng) so với số điểm dữ liệu, và
"luồn qua nhiễu" giờ diễn ra qua nhiều epoch huấn luyện thay vì một phép
giải đóng.

Cách phát hiện GIỐNG HỆT T8.1a: tách riêng một tập **validation** (val) —
dữ liệu KHÔNG được dùng để tính gradient hay cập nhật bất kỳ tham số nào,
chỉ dùng để ĐO. Theo dõi CẢ hai loss (train VÀ val) qua nhiều epoch:

> Nếu mạng học đúng QUY LUẬT thật, cả train loss VÀ val loss cùng giảm,
> cùng ổn định ở mức gần nhau.
>
> Nếu mạng bắt đầu QUÁ KHỚP, train loss tiếp tục giảm (mạng ngày càng "nhớ"
> tốt hơn những điểm ĐÃ THẤY) nhưng val loss NGỪNG giảm rồi quay đầu TĂNG
> (mạng dự đoán ngày càng TỆ hơn trên những điểm CHƯA từng thấy).

Hai nguyên nhân cộng hưởng khiến quá khớp dễ xảy ra: mạng có NHIỀU tham số
hơn cần thiết so với lượng dữ liệu (giống "bậc cao" của đa thức), và huấn
luyện QUÁ NHIỀU epoch (mỗi epoch thêm là thêm một cơ hội để mạng "học
thuộc" chi tiết ngẫu nhiên của đúng những điểm train cụ thể). Khi mạng có
RẤT NHIỀU tham số so với RẤT ÍT điểm dữ liệu, cả hai nguyên nhân cộng
hưởng mạnh tới mức chữ ký quá khớp lộ ra chỉ sau VÀI CHỤC epoch, không cần
hàng trăm.
::::

::::example{#do_qua_khop_that}
Bốn điểm train (`x = -3, -1, 1, 3`, quan hệ thật là `y = x²`, cộng chút
nhiễu nhỏ lúc sinh dữ liệu), ba điểm val NẰM GIỮA các điểm train (`x = -2,
0, 2`, giá trị `y = x²` THẬT, không nhiễu) — một MLP `1 → 4 → 1` (`13`
tham số, tầng ẩn `tanh`, tầng ra không kích hoạt), huấn luyện bằng Adam
(`lr = 0.25`), đo tại epoch `20` và epoch `30`:

```python title=readonly
import math
import numpy as np

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


def tanh_kh(z): return z.tanh()
def dinh_danh_kh(z): return z

class Layer:
    def __init__(self, W, b, kich_hoat):
        self.W = W; self.b = b; self.kich_hoat = kich_hoat
    def forward(self, x):
        ra = []
        for j in range(len(self.b)):
            z = self.b[j]
            for i in range(len(x)):
                z = z + self.W[j][i] * x[i]
            ra.append(self.kich_hoat(z))
        return ra
    def params(self):
        return [p for hang in self.W for p in hang] + self.b

class MLP:
    def __init__(self, cac_tang):
        self.cac_tang = cac_tang
    def forward(self, x):
        a = x
        for tang in self.cac_tang:
            a = tang.forward(a)
        return a
    def params(self):
        ps = []
        for tang in self.cac_tang: ps += tang.params()
        return ps

def mse_qua_value(preds, targets):
    n = len(preds)
    tong = Value(0.0)
    for p, t in zip(preds, targets):
        err = p - t
        tong = tong + err ** 2
    return tong * (1.0 / n)

def mse_so(mlp, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        pred = mlp.forward([Value(x)])[0]
        tong += (pred.data - y) ** 2
    return tong / len(xs)

def khoi_tao_mlp(seed, so_neuron_an):
    rng = np.random.default_rng(seed)
    trong_so1 = rng.uniform(-0.5, 0.5, size=(so_neuron_an, 1))
    W1 = [[Value(float(w)) for w in hang] for hang in trong_so1]
    b1 = [Value(0.0) for _ in range(so_neuron_an)]
    trong_so2 = rng.uniform(-0.5, 0.5, size=(1, so_neuron_an))
    W2 = [[Value(float(w)) for w in hang] for hang in trong_so2]
    b2 = [Value(0.0)]
    return MLP([Layer(W1, b1, tanh_kh), Layer(W2, b2, dinh_danh_kh)])

def adam_step(params, t, m_list, v_list, lr, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps)
        p.grad = 0.0

x_train = [-3.0, -1.0, 1.0, 3.0]
y_train = [9.2, 0.7, 1.3, 8.6]
x_val = [-2.0, 0.0, 2.0]
y_val = [4.0, 0.0, 4.0]

mlp = khoi_tao_mlp(seed=56, so_neuron_an=4)
params = mlp.params()
m_list = [0.0] * len(params)
v_list = [0.0] * len(params)

ket_qua = {}
for epoch in range(1, 31):
    preds = [mlp.forward([Value(x)])[0] for x in x_train]
    loss = mse_qua_value(preds, y_train)
    loss.backward()
    adam_step(params, epoch, m_list, v_list, lr=0.25)
    if epoch in (20, 30):
        ket_qua[epoch] = (mse_so(mlp, x_train, y_train), mse_so(mlp, x_val, y_val))

for epoch in (20, 30):
    tl, vl = ket_qua[epoch]
    print(f"epoch {epoch}: train={round(tl, 4)}  val={round(vl, 4)}")
```

```text title=readonly
epoch 20: train=1.1722  val=1.6477
epoch 30: train=0.2771  val=15.5632
```

Từ epoch `20` tới epoch `30` — chỉ MƯỜI epoch sau: train loss giảm hơn `4`
lần (`1.1722 → 0.2771`) — mạng khớp bốn điểm train ngày càng sát. Nhưng val
loss KHÔNG giảm theo — nó TĂNG gần `9,5` lần (`1.6477 → 15.5632`). Mạng chỉ
có `13` tham số cho vỏn vẹn `4` điểm dữ liệu — dư sức "học thuộc lòng" từng
điểm, và mười epoch đó đủ để nó bắt đầu luồn qua nhiễu thay vì tiếp tục học
quy luật `y = x²`.
::::

::::predict{#doan_train_va_val_cung_huong commitOnce}
Cả `train loss` và `val loss` đều được tính bằng ĐÚNG một công thức
(`mse_so`, MSE) trên CÙNG một mạng, tại CÙNG một thời điểm.

**Trước khi chạy thử**, bạn đoán: vì dùng chung công thức, chung mạng, hai
loss này có BẮT BUỘC phải cùng tăng hoặc cùng giảm ở MỌI khoảng epoch
không?

:::opt{correct}
Không — bằng chứng cụ thể vừa đo được: từ epoch `20` tới `30`, train giảm
còn val tăng. Hai tập dữ liệu (train, val) là hai tập ĐIỂM khác nhau; "cùng
công thức" chỉ đảm bảo cách TÍNH giống nhau, không đảm bảo GIÁ TRỊ đo được
di chuyển cùng hướng, vì bản thân dữ liệu đưa vào công thức đó khác nhau
:::

:::opt
Có — nếu mạng đang học (tham số vẫn đang cập nhật), MỌI loss tính từ mạng
đó phải cùng xu hướng, vì chúng đều phản ánh "mạng đang tốt lên hay tệ đi"
::why
Gần đúng ở trực giác rằng loss phản ánh "mạng tốt lên hay tệ đi" — đúng,
nhưng CHỈ đúng cho ĐÚNG tập dữ liệu nó được tính trên.

Chỗ lệch: "mạng đang tốt lên" không phải một tính chất DUY NHẤT, cố định —
nó là "tốt lên ĐỐI VỚI tập dữ liệu X". Một mạng có thể đồng thời tốt lên
đối với train (nhớ chi tiết hơn) VÀ tệ đi đối với val (một tập điểm HOÀN
TOÀN khác) TRONG CÙNG một khoảng epoch — đó chính xác là điều số liệu ở
trên vừa cho thấy, không phải một nghịch lý, mà là bản chất của quá khớp.
::
:::

:::opt
Không, nhưng chỉ vì tập val có ÍT điểm hơn tập train (`3` so với `4`) —
nếu hai tập cùng kích thước, train và val chắc chắn cùng xu hướng
::why
Gần đúng ở việc để ý được sự khác biệt kích thước hai tập — một quan sát
đúng về mặt dữ kiện.

Chỗ lệch: kích thước tập không phải NGUYÊN NHÂN gây ra sự tách hướng —
nguyên nhân là VỊ TRÍ và NỘI DUNG của các điểm (val không hề được dùng để
cập nhật tham số, nên nó đo đúng khả năng TỔNG QUÁT HOÁ, còn train đo khả
năng NHỚ). Dù val có `3`, `30`, hay `300` điểm, hiện tượng quá khớp vẫn có
thể xảy ra — nó xuất phát từ việc mạng có quá nhiều tự do so với train,
không phải từ việc val "ít hơn" train bao nhiêu.
::
:::
::::

::::code{#do_qua_khop_that}
Hoàn thiện `mse_so` (tính dự đoán qua `mlp.forward`), vòng lặp ghi lại
`ket_qua` tại mỗi epoch kiểm tra, và kết luận cuối cùng: `dau_hieu_qua_khop`
— `True` khi train GIẢM mà val TĂNG giữa hai điểm kiểm tra.

```python title=starter
import math
import numpy as np

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


def tanh_kh(z): return z.tanh()
def dinh_danh_kh(z): return z

class Layer:
    def __init__(self, W, b, kich_hoat):
        self.W = W; self.b = b; self.kich_hoat = kich_hoat
    def forward(self, x):
        ra = []
        for j in range(len(self.b)):
            z = self.b[j]
            for i in range(len(x)):
                z = z + self.W[j][i] * x[i]
            ra.append(self.kich_hoat(z))
        return ra
    def params(self):
        return [p for hang in self.W for p in hang] + self.b

class MLP:
    def __init__(self, cac_tang):
        self.cac_tang = cac_tang
    def forward(self, x):
        a = x
        for tang in self.cac_tang:
            a = tang.forward(a)
        return a
    def params(self):
        ps = []
        for tang in self.cac_tang: ps += tang.params()
        return ps

def mse_qua_value(preds, targets):
    n = len(preds)
    tong = Value(0.0)
    for p, t in zip(preds, targets):
        err = p - t
        tong = tong + err ** 2
    return tong * (1.0 / n)

def mse_so(mlp, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        pred = ___                    # mlp.forward([Value(x)])[0]
        tong += (pred.data - y) ** 2
    return tong / len(xs)

def khoi_tao_mlp(seed, so_neuron_an):
    rng = np.random.default_rng(seed)
    trong_so1 = rng.uniform(-0.5, 0.5, size=(so_neuron_an, 1))
    W1 = [[Value(float(w)) for w in hang] for hang in trong_so1]
    b1 = [Value(0.0) for _ in range(so_neuron_an)]
    trong_so2 = rng.uniform(-0.5, 0.5, size=(1, so_neuron_an))
    W2 = [[Value(float(w)) for w in hang] for hang in trong_so2]
    b2 = [Value(0.0)]
    return MLP([Layer(W1, b1, tanh_kh), Layer(W2, b2, dinh_danh_kh)])

def adam_step(params, t, m_list, v_list, lr, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps)
        p.grad = 0.0

x_train = [-3.0, -1.0, 1.0, 3.0]
y_train = [9.2, 0.7, 1.3, 8.6]
x_val = [-2.0, 0.0, 2.0]
y_val = [4.0, 0.0, 4.0]

mlp = khoi_tao_mlp(seed=56, so_neuron_an=4)
params = mlp.params()
m_list = [0.0] * len(params)
v_list = [0.0] * len(params)

ket_qua = {}
for epoch in range(1, 31):
    preds = [mlp.forward([Value(x)])[0] for x in x_train]
    loss = mse_qua_value(preds, y_train)
    loss.backward()
    adam_step(params, epoch, m_list, v_list, lr=0.25)
    if epoch in (20, 30):
        ket_qua[epoch] = (___, ___)    # mse_so(mlp, x_train, y_train), mse_so(mlp, x_val, y_val)

train_20, val_20 = ket_qua[20]
train_30, val_30 = ket_qua[30]
train_giam = train_30 < train_20
val_tang = val_30 > val_20
dau_hieu_qua_khop = ___                 # train_giam and val_tang

print(round(train_20, 4), round(val_20, 4))
print(round(train_30, 4), round(val_30, 4))
print(dau_hieu_qua_khop)
```

```python title=solution
import math
import numpy as np

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


def tanh_kh(z): return z.tanh()
def dinh_danh_kh(z): return z

class Layer:
    def __init__(self, W, b, kich_hoat):
        self.W = W; self.b = b; self.kich_hoat = kich_hoat
    def forward(self, x):
        ra = []
        for j in range(len(self.b)):
            z = self.b[j]
            for i in range(len(x)):
                z = z + self.W[j][i] * x[i]
            ra.append(self.kich_hoat(z))
        return ra
    def params(self):
        return [p for hang in self.W for p in hang] + self.b

class MLP:
    def __init__(self, cac_tang):
        self.cac_tang = cac_tang
    def forward(self, x):
        a = x
        for tang in self.cac_tang:
            a = tang.forward(a)
        return a
    def params(self):
        ps = []
        for tang in self.cac_tang: ps += tang.params()
        return ps

def mse_qua_value(preds, targets):
    n = len(preds)
    tong = Value(0.0)
    for p, t in zip(preds, targets):
        err = p - t
        tong = tong + err ** 2
    return tong * (1.0 / n)

def mse_so(mlp, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        pred = mlp.forward([Value(x)])[0]
        tong += (pred.data - y) ** 2
    return tong / len(xs)

def khoi_tao_mlp(seed, so_neuron_an):
    rng = np.random.default_rng(seed)
    trong_so1 = rng.uniform(-0.5, 0.5, size=(so_neuron_an, 1))
    W1 = [[Value(float(w)) for w in hang] for hang in trong_so1]
    b1 = [Value(0.0) for _ in range(so_neuron_an)]
    trong_so2 = rng.uniform(-0.5, 0.5, size=(1, so_neuron_an))
    W2 = [[Value(float(w)) for w in hang] for hang in trong_so2]
    b2 = [Value(0.0)]
    return MLP([Layer(W1, b1, tanh_kh), Layer(W2, b2, dinh_danh_kh)])

def adam_step(params, t, m_list, v_list, lr, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps)
        p.grad = 0.0

x_train = [-3.0, -1.0, 1.0, 3.0]
y_train = [9.2, 0.7, 1.3, 8.6]
x_val = [-2.0, 0.0, 2.0]
y_val = [4.0, 0.0, 4.0]

mlp = khoi_tao_mlp(seed=56, so_neuron_an=4)
params = mlp.params()
m_list = [0.0] * len(params)
v_list = [0.0] * len(params)

ket_qua = {}
for epoch in range(1, 31):
    preds = [mlp.forward([Value(x)])[0] for x in x_train]
    loss = mse_qua_value(preds, y_train)
    loss.backward()
    adam_step(params, epoch, m_list, v_list, lr=0.25)
    if epoch in (20, 30):
        ket_qua[epoch] = (mse_so(mlp, x_train, y_train), mse_so(mlp, x_val, y_val))

train_20, val_20 = ket_qua[20]
train_30, val_30 = ket_qua[30]
train_giam = train_30 < train_20
val_tang = val_30 > val_20
dau_hieu_qua_khop = train_giam and val_tang

print(round(train_20, 4), round(val_20, 4))
print(round(train_30, 4), round(val_30, 4))
print(dau_hieu_qua_khop)
```

```python title=test
assert round(train_20, 4) == 1.1722, f"train epoch 20 sai -- dang ra {round(train_20, 4)}"
assert round(val_20, 4) == 1.6477, f"val epoch 20 sai -- dang ra {round(val_20, 4)}"
assert round(train_30, 4) == 0.2771, f"train epoch 30 sai -- dang ra {round(train_30, 4)}"
assert round(val_30, 4) == 15.5632, f"val epoch 30 sai -- dang ra {round(val_30, 4)}"

assert train_giam == True, "train loss PHAI giam tu epoch 20 den epoch 30 -- dau hieu mang van tiep tuc khop TOT HON tren du lieu train"
assert val_tang == True, "val loss PHAI tang tu epoch 20 den epoch 30 -- dau hieu qua khop, mang khop tot hon tren train nhung TE hon tren du lieu chua tung thay"
assert dau_hieu_qua_khop == True, "dau_hieu_qua_khop phai la True: train giam VA val tang CUNG LUC la chu ky kinh dien cua qua khop"

# rieng kiem tra ty le: val phai tang it nhat gap 5 lan (15.5632/1.6477 ~
# 9.45) -- khong chi tang mot chut xiu do nhieu so, ma tang RO RET, chi
# trong muoi epoch.
assert val_30 > val_20 * 5, f"val phai tang RAT RO RET (it nhat gap 5), khong chi mot chut nhieu so -- ty le do duoc {round(val_30/val_20, 4)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `mse_so`, `pred`: gọi `forward` trên mạng, bọc đầu vào trong `Value` — `mlp.forward([Value(x)])[0]` (lấy phần tử `[0]` vì tầng ra có đúng một neuron). Hai chỗ trong `ket_qua[epoch] = (___, ___)`: cặp giá trị `(mse_so(mlp, x_train, y_train), mse_so(mlp, x_val, y_val))` — train trước, val sau, đúng thứ tự đã dùng ở mọi nơi khác trong bài. `dau_hieu_qua_khop`: kết hợp HAI điều kiện đã tính sẵn bằng `and` — `train_giam and val_tang`.
- kind: strategy
  body: 'pred: `mlp.forward([Value(x)])[0]`. ket_qua[epoch]: `(mse_so(mlp, x_train, y_train), mse_so(mlp, x_val, y_val))`. dau_hieu_qua_khop: `train_giam and val_tang`.'
- kind: one-line
  body: 'Bốn chỗ trống: `mlp.forward([Value(x)])[0]`, `mse_so(mlp, x_train, y_train)`, `mse_so(mlp, x_val, y_val)`, và `train_giam and val_tang`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: mse_so phai goi THAT mlp.forward (khong duoc chep san du doan); ket_qua[epoch] phai goi THAT mse_so tren CA train LAN val (dung thu tu train truoc, val sau); dau_hieu_qua_khop phai la PHEP VA (and) cua CA HAI dieu kien train_giam, val_tang -- khong duoc chi dua vao mot dieu kien
  requireAst:
  - kind: uses-call, target: forward, min: 3
  - kind: uses-call, target: mse_so, min: 2
  - kind: uses-operator, target: and, min: 1
  - kind: uses-name, target: train_giam, min: 1
  - kind: uses-name, target: val_tang, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca nam luat qua sach (forward=3: goi that trong mse_so
  # "mlp.forward([Value(x)])", goi trong vong lap chinh huan luyen, goi ben
  # trong MLP.forward "tang.forward(a)" -- dinh nghia ham KHONG tinh vi la
  # uses-call chi dem ast.Call; mse_so=2: dinh nghia [khong tinh], hai lan
  # GOI trong tuple ket_qua[epoch] = (mse_so(...), mse_so(...)); and=1: dung
  # dung mot lan trong dau_hieu_qua_khop; train_giam=1, val_tang=1: uses-name
  # chi dem cho DOC [ctx=Load] -- dong gan "train_giam = ..." la Store,
  # khong tinh; dong doc lai trong "train_giam and val_tang" moi la 1 lan
  # Load duy nhat cho moi bien).
  #
  # Cheat "dau_hieu_qua_khop = val_tang" (bo qua train_giam, chi xet mot
  # dieu kien) lam "and" ve 0 VA "train_giam" ve dung 0 (khong con doc lai o
  # dau ca) -- bi chan boi static; ve mat logic day van la mot cheat NGUY
  # HIEM vi tinh cho ra dung True tren du lieu nay (ca hai dieu kien deu
  # True) nen tests/output khong bat duoc rieng le -- CHI static bat duoc,
  # dung ly do tang static ton tai.
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^1\\.1722 1\\.6477\\n0\\.2771 15\\.5632\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Train loss giảm hơn `4` lần. Val loss tăng gần `9,5` lần. Cùng một mạng,
cùng một lúc, chỉ sau mười epoch — chữ ký kinh điển của quá khớp, giờ đo
được trên một mạng nơ-ron thật, không chỉ trên đa thức.
::::

::::reflect{#nghi-lai}
Bài `overfitting-va-do-phuc-tap` (T8.1a) phát hiện quá khớp — nhưng KHÔNG
đưa ra cách CHỐNG nó ngoài "giảm bậc mô hình". Bài `regularization-l1-l2`
(cùng track đó) chỉ ra một cách khác: phạt trực tiếp các hệ số lớn, không
đụng vào kiến trúc mô hình. Mạng nơ-ron có phiên bản riêng của cả hai ý
tưởng đó không? Bài sau trả lời — bằng số thật, trên đúng mạng vừa quá
khớp ở đây.
::::

::::checkpoint{mastery=0.85}
::::
