---
id: tri-tue-nhan-tao.huan-luyen-mang-no-ron.chinh-quy-hoa-mang-no-ron
title: "Chính quy hoá mạng nơ-ron: L2 và dropout"
summary: "Trên đúng mạng vừa quá khớp ở bài trước (25 epoch, không regularization: train=0.3718, val=4.6324): L2 weight decay (lambda=0.2, phạt lr*lambda*w cộng thẳng vào bước cập nhật mỗi tham số) cho train=3.1154 (tệ hơn nhiều — đánh đổi có chủ đích) nhưng val=1.6002 (giảm khoảng 65%). Dropout (tắt ngẫu nhiên 10% activation tầng ẩn lúc huấn luyện, có scale 1/0.9, tắt hẳn lúc đánh giá) cho train=6.0692, val=1.7337 (giảm khoảng 63%). Cả hai đều hy sinh khả năng khớp train để đổi lấy val tốt hơn — số liệu thật, không chỉ lý thuyết."
locale: vi
track: tri-tue-nhan-tao
module: huan-luyen-mang-no-ron
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.chinh-quy-hoa-mang-no-ron]
requires: [ai.qua-khop-mang-no-ron]
concepts: [ai.chinh-quy-hoa-mang-no-ron]
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
Bài trước để lại một mạng quá khớp rõ ràng — chỉ mười epoch mà val loss
tăng gần `9,5` lần. Hai cách chống lại, không cần đổi kiến trúc mạng,
không cần bớt epoch.
::::

::::explain{#l2_va_dropout}
Track hồi quy (T8.1a) đã dạy **regularization**: cộng thêm vào loss một số
hạng PHẠT các hệ số lớn — `λ·Σwⱼ²` (L2/Ridge). Mạng nơ-ron dùng lại Ý TƯỞNG
đó, nhưng cách viết CỤ THỂ khác đi một chút khi kết hợp với Adam: thay vì
cộng phạt vào loss rồi để nó lan qua `backward()`, ta trừ thẳng một lượng
tỷ lệ với chính trọng số vào bước cập nhật:

> `param ← param − lr·(bước_Adam + λ·param)`

Đây gọi là **weight decay** (rã trọng số) — mỗi bước, MỌI trọng số bị kéo
nhẹ về phía `0` một lượng tỷ lệ thuận với chính độ lớn của nó, CỘNG THÊM
vào bước cập nhật thường của Adam. Trọng số càng lớn, lực kéo về `0` càng
mạnh — đúng tinh thần L2 đã học, chỉ khác chỗ áp dụng (trực tiếp vào bước
cập nhật, không qua gradient của loss) để tránh xung đột với cách Adam tự
CO GIÃN bước theo `√v̂` (bài `adam-toi-uu-hoa`).

**Dropout** là một ý tưởng hoàn toàn khác: trong lúc huấn luyện, ngẫu
nhiên chọn một phần activation của tầng ẩn và NHÂN chúng với `0` — coi như
những neuron đó "vắng mặt" ở đúng bước này. Ở bước sau, một tập neuron
KHÁC (chọn lại ngẫu nhiên) có thể bị tắt. Mạng buộc phải học một biểu diễn
KHÔNG phụ thuộc quá nhiều vào bất kỳ một neuron cụ thể nào — vì neuron đó
có thể vắng mặt bất cứ lúc nào. Lúc ĐÁNH GIÁ (tính `val loss`), dropout
TẮT HẲN — mọi neuron đều có mặt, dùng đúng năng lực đầy đủ của mạng.

Vì lúc huấn luyện một phần activation bị nhân `0`, tổng tín hiệu truyền
qua tầng đó giảm đi so với lúc đánh giá — cách sửa chuẩn là NHÂN THÊM các
activation còn giữ lại với `1/(1 − tỷ_lệ_drop)`, để tổng tín hiệu TRUNG
BÌNH không đổi giữa hai chế độ.

Cả hai kỹ thuật đều CỐ Ý làm train loss TỆ ĐI (mạng không còn tự do luồn
qua nhiễu như trước) — đổi lại val loss tốt hơn. Đây không phải một sự cải
thiện "miễn phí".
::::

::::example{#l2_va_dropout_that}
Trên ĐÚNG mạng (`1 → 4 → 1`, `13` tham số) và dữ liệu của bài
`qua-khop-mang-no-ron` (`seed=56`), sau `25` epoch:

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

def adam_step_l2(params, t, m_list, v_list, lr, lam, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        buoc_adam = lr * m_hat / (v_hat ** 0.5 + eps)
        buoc_l2 = lr * lam * p.data
        p.data -= buoc_adam + buoc_l2
        p.grad = 0.0

x_train = [-3.0, -1.0, 1.0, 3.0]
y_train = [9.2, 0.7, 1.3, 8.6]
x_val = [-2.0, 0.0, 2.0]
y_val = [4.0, 0.0, 4.0]

def train_l2(seed, so_epoch, lr, lam):
    mlp = khoi_tao_mlp(seed, 4)
    params = mlp.params()
    m_list = [0.0] * len(params)
    v_list = [0.0] * len(params)
    for epoch in range(1, so_epoch + 1):
        preds = [mlp.forward([Value(x)])[0] for x in x_train]
        loss = mse_qua_value(preds, y_train)
        loss.backward()
        adam_step_l2(params, epoch, m_list, v_list, lr, lam)
    return mse_so(mlp, x_train, y_train), mse_so(mlp, x_val, y_val)

tl0, vl0 = train_l2(56, 25, 0.25, 0.0)      # khong regularization -- dung bai truoc
tl1, vl1 = train_l2(56, 25, 0.25, 0.2)      # L2, lambda=0.2

print("khong L2:  train=%.4f  val=%.4f" % (tl0, vl0))
print("co L2:     train=%.4f  val=%.4f" % (tl1, vl1))
```

```text title=readonly
khong L2:  train=0.3718  val=4.6324
co L2:     train=3.1154  val=1.6002
```

L2 (`λ=0.2`) làm train loss TỆ HƠN gần `8,4` lần (`0.3718 → 3.1154`) — đúng
như dự đoán, mạng không còn tự do khớp nhiễu như trước. Nhưng val loss
giảm khoảng `65%` (`4.6324 → 1.6002`) — mạng tổng quát hoá TỐT HƠN HẲN, dù
khớp train KÉM hơn nhiều.

Dropout (tỷ lệ `10%` trên tầng ẩn, có scale `1/0.9`, tắt hẳn lúc đánh giá)
trên CÙNG mạng, CÙNG số epoch — khối này TỰ CHỨA lại toàn bộ `Value`,
`Layer`, `MLP`, không dùng chung gì với khối phía trên:

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

def forward_dropout(mlp, x, ty_le_drop, rng, dang_train):
    a = mlp.cac_tang[0].forward([Value(x)])          # activation tang an
    if dang_train and ty_le_drop > 0:
        giu = 1.0 - ty_le_drop
        a = [act * Value(0.0) if rng.random() < ty_le_drop else act * Value(1.0 / giu) for act in a]
    return mlp.cac_tang[1].forward(a)[0]              # tang ra

def mse_dropout(mlp, xs, ys, ty_le_drop, rng, dang_train):
    tong = Value(0.0)
    for x, y in zip(xs, ys):
        pred = forward_dropout(mlp, x, ty_le_drop, rng, dang_train)
        tong = tong + (pred - y) ** 2
    return tong * (1.0 / len(xs))

def mse_so_dropout(mlp, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        pred = forward_dropout(mlp, x, 0.0, None, dang_train=False)
        tong += (pred.data - y) ** 2
    return tong / len(xs)

def train_dropout(seed, so_epoch, lr, ty_le_drop):
    mlp = khoi_tao_mlp(seed, 4)
    params = mlp.params()
    m_list = [0.0] * len(params)
    v_list = [0.0] * len(params)
    rng = np.random.default_rng(123)
    for epoch in range(1, so_epoch + 1):
        loss = mse_dropout(mlp, x_train, y_train, ty_le_drop, rng, dang_train=True)
        loss.backward()
        adam_step(params, epoch, m_list, v_list, lr=lr)
    return mse_so_dropout(mlp, x_train, y_train), mse_so_dropout(mlp, x_val, y_val)

tl2, vl2 = train_dropout(56, 25, 0.25, ty_le_drop=0.1)
print("dropout 10%%: train=%.4f  val=%.4f" % (tl2, vl2))
```

```text title=readonly
dropout 10%: train=6.0692  val=1.7337
```

Dropout `10%` cũng cho cùng chữ ký: train TỆ hơn hẳn (`0.3718 → 6.0692` —
tệ hơn cả L2, vì việc tắt ngẫu nhiên neuron gây nhiễu loạn mạnh trên một
mạng chỉ có `4` neuron ẩn), val giảm khoảng `63%` (`4.6324 → 1.7337`) —
gần bằng mức cải thiện của L2, bằng một cơ chế hoàn toàn khác.
::::

::::predict{#doan_l2_lon_hon_nua commitOnce}
`λ = 0.2` cho val giảm từ `4.6324` xuống `1.6002` — một cải thiện rõ rệt.

**Trước khi chạy thử**, bạn đoán: nếu tăng `λ` lên RẤT lớn (ví dụ `λ = 2.0`
— gấp `10` lần), val loss sẽ tiếp tục cải thiện hơn nữa, hay đổi chiều?

:::opt{correct}
Đổi chiều — `λ` quá lớn kéo MỌI trọng số về gần `0` mạnh tới mức mạng mất
luôn khả năng khớp cả QUY LUẬT thật (không chỉ mất khả năng khớp nhiễu) —
cả train LẪN val cùng tệ đi, một dạng "chưa khớp" (underfitting) mới,
ngược hẳn với quá khớp mà `λ` được thêm vào để chữa
:::

:::opt
Không — `λ` càng lớn, phạt càng mạnh, val loss càng thấp, không có giới
hạn nào cả — chỉ cần đủ kiên nhẫn tăng `λ`
::why
Gần đúng ở việc nhận ra `λ` lớn hơn tạo phạt mạnh hơn — quan sát đó đúng
về CƠ CHẾ.

Chỗ lệch: "phạt mạnh hơn" không đồng nghĩa "tổng quát hoá tốt hơn mãi mãi".
Có một điểm mà phạt quá mạnh không còn ép mạng bỏ NHIỄU nữa — nó ép mạng
bỏ luôn cả TÍN HIỆU thật, vì trọng số bị kéo về `0` xa hơn mức cần thiết để
biểu diễn đúng `y = x²`. Regularization có một "lượng vừa đủ" chứ không
phải "càng nhiều càng tốt" — đúng bài học đã thấy với Ridge/`λ` ở track hồi
quy.
::
:::

:::opt
Không xác định được — L2 và mạng nơ-ron là hai thứ khác nhau, không thể
suy ra hành vi của cái này từ cái kia
::why
Gần đúng ở việc thận trọng khi chuyển một trực giác từ bối cảnh này sang
bối cảnh khác — một thái độ hợp lý nói chung.

Chỗ lệch: bản chất TOÁN HỌC của phạt L2 (`λ·Σw²` kéo trọng số về `0`) không
đổi dù áp dụng cho hồi quy tuyến tính hay mạng nơ-ron — cùng MỘT cơ chế,
cùng một điểm yếu khi `λ` quá lớn. Không cần chạy thử mù quáng để biết
HƯỚNG của hiện tượng này — cơ chế đã được hiểu rõ từ track hồi quy, chỉ có
CON SỐ cụ thể (val loss chính xác bằng bao nhiêu) là cần đo thật.
::
:::
::::

::::code{#viet_l2_weight_decay}
Hoàn thiện `adam_step_l2`: bước phạt `buoc_l2` (`lr * lam * p.data`), rồi
cập nhật `p.data` bằng TỔNG của bước Adam và bước phạt.

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

def adam_step_l2(params, t, m_list, v_list, lr, lam, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        buoc_adam = lr * m_hat / (v_hat ** 0.5 + eps)
        buoc_l2 = ___                    # lr * lam * p.data
        p.data -= ___                    # buoc_adam + buoc_l2
        p.grad = 0.0

def train_l2(seed, so_epoch, lr, lam):
    mlp = khoi_tao_mlp(seed, 4)
    params = mlp.params()
    m_list = [0.0] * len(params)
    v_list = [0.0] * len(params)
    for epoch in range(1, so_epoch + 1):
        preds = [mlp.forward([Value(x)])[0] for x in x_train]
        loss = mse_qua_value(preds, y_train)
        loss.backward()
        adam_step_l2(params, epoch, m_list, v_list, lr, lam)
    return mse_so(mlp, x_train, y_train), mse_so(mlp, x_val, y_val)

x_train = [-3.0, -1.0, 1.0, 3.0]
y_train = [9.2, 0.7, 1.3, 8.6]
x_val = [-2.0, 0.0, 2.0]
y_val = [4.0, 0.0, 4.0]

tl0, vl0 = train_l2(56, 25, 0.25, 0.0)
tl1, vl1 = train_l2(56, 25, 0.25, 0.2)

print(round(tl0, 4), round(vl0, 4))
print(round(tl1, 4), round(vl1, 4))
print(vl1 < vl0)
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

def adam_step_l2(params, t, m_list, v_list, lr, lam, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        buoc_adam = lr * m_hat / (v_hat ** 0.5 + eps)
        buoc_l2 = lr * lam * p.data
        p.data -= buoc_adam + buoc_l2
        p.grad = 0.0

def train_l2(seed, so_epoch, lr, lam):
    mlp = khoi_tao_mlp(seed, 4)
    params = mlp.params()
    m_list = [0.0] * len(params)
    v_list = [0.0] * len(params)
    for epoch in range(1, so_epoch + 1):
        preds = [mlp.forward([Value(x)])[0] for x in x_train]
        loss = mse_qua_value(preds, y_train)
        loss.backward()
        adam_step_l2(params, epoch, m_list, v_list, lr, lam)
    return mse_so(mlp, x_train, y_train), mse_so(mlp, x_val, y_val)

x_train = [-3.0, -1.0, 1.0, 3.0]
y_train = [9.2, 0.7, 1.3, 8.6]
x_val = [-2.0, 0.0, 2.0]
y_val = [4.0, 0.0, 4.0]

tl0, vl0 = train_l2(56, 25, 0.25, 0.0)
tl1, vl1 = train_l2(56, 25, 0.25, 0.2)

print(round(tl0, 4), round(vl0, 4))
print(round(tl1, 4), round(vl1, 4))
print(vl1 < vl0)
```

```python title=test
assert round(tl0, 4) == 0.3718, f"train khong L2 sai -- dang ra {round(tl0, 4)}"
assert round(vl0, 4) == 4.6324, f"val khong L2 sai -- dang ra {round(vl0, 4)}"
assert round(tl1, 4) == 3.1154, f"train CO L2 sai -- dang ra {round(tl1, 4)}"
assert round(vl1, 4) == 1.6002, f"val CO L2 sai -- dang ra {round(vl1, 4)}"

assert vl1 < vl0, "L2 (lambda=0.2) PHAI cho val loss THAP HON khong L2"
assert tl1 > tl0, "L2 phai HY SINH mot phan kha nang khop train (train loss cao hon) -- day la danh doi co chu y, khong phai loi"

# rieng kiem tra bien: lam=0.0 phai cho buoc_l2 dung bang 0 (khong con anh
# huong gi den cap nhat) -- kiem tra TRUC TIEP tren mot tham so gia lap,
# doc lap voi ket qua huan luyen day du o tren.
p_gia = Value(2.0)
p_gia.grad = 0.5
m_gia = [0.0]; v_gia = [0.0]
adam_step_l2([p_gia], 1, m_gia, v_gia, lr=0.1, lam=0.0)
gia_tri_sau_lam0 = p_gia.data
p_gia2 = Value(2.0)
p_gia2.grad = 0.5
m_gia2 = [0.0]; v_gia2 = [0.0]
adam_step_l2([p_gia2], 1, m_gia2, v_gia2, lr=0.1, lam=0.3)
gia_tri_sau_lam03 = p_gia2.data
assert abs(gia_tri_sau_lam0 - gia_tri_sau_lam03) > 1e-6, "buoc_l2 phai THAT SU thay doi cap nhat khi lam khac 0 -- lam=0.0 va lam=0.3 phai cho hai gia tri p.data KHAC nhau tren cung mot tham so gia lap"
```

:::hints
- kind: attention
  body: Hai chỗ trống. `buoc_l2`: đúng công thức weight decay — `lr * lam * p.data` (tỷ lệ thuận với CHÍNH giá trị tham số, không phải gradient của nó). `p.data -=`: TRỪ ĐI TỔNG của bước Adam thường VÀ bước phạt — `buoc_adam + buoc_l2` (không phải chỉ một trong hai).
- kind: strategy
  body: 'buoc_l2: `lr * lam * p.data`. p.data -=: `buoc_adam + buoc_l2`.'
- kind: one-line
  body: 'Hai chỗ trống: `lr * lam * p.data` và `buoc_adam + buoc_l2`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: buoc_l2 phai tinh tu lr, lam, VA p.data (khong duoc chep hang so, se khong tat dong khi lam=0); p.data phai tru DI TONG ca buoc_adam LAN buoc_l2 (khong duoc bo qua buoc_adam hay buoc_l2)
  requireAst:
  - kind: uses-name, target: lam, min: 2
  - kind: uses-name, target: buoc_adam, min: 1
  - kind: uses-name, target: buoc_l2, min: 1
  - kind: gan-ten, target: p, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (lam=2: mot lan trong bieu thuc
  # buoc_l2 = lr*lam*p.data, mot lan la tham so trong dinh nghia ham [Store,
  # khong tinh boi uses-name -- kiem tra lai xac nhan dung 2 lan Load thuc
  # su]; buoc_adam=1: doc trong p.data -=; buoc_l2=1: doc trong p.data -=;
  # gan-ten p=2: "p.data -= ..." [AugAssign, Name(p) trong dich Attribute]
  # VA "p.grad = 0.0" [Assign, Name(p) trong dich Attribute] -- vong lap
  # "for i, p in enumerate(...)" la ast.For, KHONG duoc gan-ten dem [chi
  # dem Assign/AugAssign/AnnAssign]).
  #
  # Cheat "buoc_l2 = 0.0" (chep hang so, bo qua regularization hoan toan)
  # lam "lam" ve 0 (khong con doc lam o dau) -- bi chan boi static; da tu
  # kiem chung bang Python that: cheat nay cho tl1/vl1 GIONG HET tl0/vl0
  # (vi lam=0.2 khong con anh huong gi), that bai ro rang o assert
  # "vl1 < vl0" va "tl1 > tl0" -- bi chan doc lap boi tests. Rieng test tren
  # tham so gia lap (p_gia/p_gia2) ep di qua dung bien lam=0.0 doc lap voi
  # ket qua huan luyen day du, phong khi hai gia tri tl0/tl1 tinh co trung
  # nhau vi ly do khac.
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^0\\.3718 4\\.6324\\n3\\.1154 1\\.6002\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
L2 giảm val loss `65%`, dropout giảm `63%` — cả hai đều hy sinh khả năng
khớp train để đổi lấy tổng quát hoá tốt hơn. Không có gì miễn phí, nhưng
cả hai đều THẬT.
::::

::::reflect{#nghi-lai}
Tám khái niệm, một chuỗi: `Layer`/`MLP` chạy qua `Value`, hàm mất mát tính
qua `Value`, vòng lặp huấn luyện đầy đủ, ba cách chia dữ liệu, động lượng,
Adam, quá khớp, và giờ hai cách chống quá khớp — L2, dropout. Mọi mảnh đều
đã lắp vào đúng vị trí của nó, đo bằng số thật, không suy luận tay.

Bài cuối ráp TẤT CẢ lại: một vòng lặp huấn luyện đầy đủ (Adam + tuỳ chọn
L2) chạy từ đầu tới cuối trên MỘT bài toán hồi quy VÀ MỘT bài toán phân
loại nhị phân — số liệu cuối cùng phải khớp với chính những gì track này
đã dạy.
::::

::::checkpoint{mastery=0.85}
::::
