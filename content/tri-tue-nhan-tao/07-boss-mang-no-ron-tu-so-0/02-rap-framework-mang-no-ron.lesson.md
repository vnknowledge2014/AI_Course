---
id: tri-tue-nhan-tao.boss-mang-no-ron-tu-so-0.rap-framework-mang-no-ron
title: "Ráp framework: forward + backward + một bước Adam"
summary: "Không phát minh gì mới — chỉ XÁC NHẬN Value (8 phép toán) + Layer/MLP + khoi_tao_mlp + adam_step_l2 (nguyên vẹn từ q8.2c) chạy nhất quán trên một MLP nhỏ (2→3→2, 17 tham số). Forward + cross_entropy_qua_value + backward(): cả 17 tham số có grad khác 0 (loss=0,6673). Một bước Adam (lr=0,1): cả 17 tham số thay đổi giá trị VÀ grad reset về 0 — không đứt đồ thị ở bất kỳ điểm nào. Hai cách chép-đáp-số (hardcode so_khac_0/so_con_grad bằng đúng con số đúng) qua lọt output/tests, CHỈ static bắt được."
locale: vi
track: tri-tue-nhan-tao
module: boss-mang-no-ron-tu-so-0
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.rap-framework-mang-no-ron]
requires: [ai.du-lieu-phi-tuyen-that]
concepts: [ai.rap-framework-mang-no-ron]
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
Bài trước dùng một neuron ĐƠN, thất bại trước hai vòng tròn. Trước khi đem
`Layer`/`MLP` — mạng NHIỀU tầng — ra thử thách đó, một bước dừng lại: framework
này có thực sự ráp đúng không?
::::

::::explain{#khong_phat_minh_gi_moi}
Bài này KHÔNG thêm bất kỳ công thức toán mới nào. Mọi mảnh đã có sẵn từ
`huan-luyen-mang-no-ron` (q8.2c):

> **`Value`** — tám phép toán (`+`, `-`, `*`, `**`, `tanh`, `relu`, `exp`,
> `log`), mỗi phép tự biết đạo hàm cục bộ của mình.
>
> **`Layer`/`MLP`** — `W`, `b` là `Value`, `forward` cộng dồn qua vòng lặp
> tường minh (không `numpy`/`@`), xây đồ thị tính toán tự động.
>
> **`khoi_tao_mlp`** — khởi tạo `W` ngẫu nhiên nhỏ (`rng.uniform(-0.5,
> 0.5)`), `b` bằng `0`, tầng ẩn `tanh`, tầng ra không kích hoạt.
>
> **`adam_step_l2`** — hai trung bình động (`m`, `v`), hiệu chỉnh lệch,
> cộng thêm phạt L2 tuỳ chọn (`lam`, mặc định `0`).

Mục đích duy nhất của bài này: XÁC NHẬN bốn mảnh đó ráp lại thành một khối
chạy được — forward tạo ra một con số, `backward()` lan gradient tới TỪNG
tham số, và `adam_step_l2` THỰC SỰ thay đổi từng tham số đó. "Xác nhận"
nghĩa là đo được BẰNG SỐ, không phải chỉ đọc lại code và tin rằng nó đúng.

Gotcha trung tâm nhắc lại từ bài `neuron-tu-autograd` (q8.2c): **đúng SỐ
không có nghĩa là đúng ĐỒ THỊ**. Nếu forward pass lỡ dùng số Python trần
(ví dụ `Value(z.data)`) ở một điểm bất kỳ thay vì đi tiếp qua các operator
đã overload của `Value`, con số in ra vẫn có thể ĐÚNG — nhưng gradient của
mọi tham số PHÍA TRƯỚC điểm đó sẽ tuyệt đối bằng `0` sau `backward()`.
Kiểm tra "in ra đúng số" là chưa đủ; phải kiểm TỪNG tham số.
::::

::::example{#xac_nhan_khong_dut_do_thi}
MLP `2 → 3 → 2` (`17` tham số: `6` `W` + `3` `b` ở tầng ẩn, `6` `W` + `2`
`b` ở tầng ra), một điểm dữ liệu, một bước Adam:

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

    def relu(self):
        out = Value(max(0.0, self.data), (self,), 'relu')
        def _backward():
            self.grad += (1.0 if self.data > 0 else 0.0) * out.grad
        out._backward = _backward
        return out

    def exp(self):
        e = math.exp(self.data)
        out = Value(e, (self,), 'exp')
        def _backward():
            self.grad += e * out.grad
        out._backward = _backward
        return out

    def log(self):
        l = math.log(self.data)
        out = Value(l, (self,), 'log')
        def _backward():
            self.grad += (1.0 / self.data) * out.grad
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

def khoi_tao_mlp(seed, kich_thuoc):
    rng = np.random.default_rng(seed)
    tang = []
    dau_vao = kich_thuoc[0]
    for so_ra in kich_thuoc[1:-1]:
        tho = rng.uniform(-0.5, 0.5, size=(so_ra, dau_vao))
        W = [[Value(float(w)) for w in hang] for hang in tho]
        b = [Value(0.0) for _ in range(so_ra)]
        tang.append(Layer(W, b, tanh_kh))
        dau_vao = so_ra
    so_ra = kich_thuoc[-1]
    tho = rng.uniform(-0.5, 0.5, size=(so_ra, dau_vao))
    W = [[Value(float(w)) for w in hang] for hang in tho]
    b = [Value(0.0) for _ in range(so_ra)]
    tang.append(Layer(W, b, dinh_danh_kh))
    return MLP(tang)

def cross_entropy_qua_value(logits, y_true_idx):
    m = max(v.data for v in logits)
    shifted = [v - m for v in logits]
    exps = [v.exp() for v in shifted]
    tong_exp = exps[0]
    for e in exps[1:]:
        tong_exp = tong_exp + e
    log_tong = tong_exp.log()
    return log_tong - shifted[y_true_idx]

def adam_step_l2(params, t, m_list, v_list, lr, lam=0.0, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps) + lr * lam * p.data
        p.grad = 0.0


mlp = khoi_tao_mlp(seed=5, kich_thuoc=[2, 3, 2])
params = mlp.params()
truoc = [p.data for p in params]

x = [Value(0.6), Value(-0.4)]
logits = mlp.forward(x)
loss = cross_entropy_qua_value(logits, 1)
loss.backward()

so_khac_0 = sum(1 for p in params if abs(p.grad) > 1e-9)

m_list = [0.0] * len(params)
v_list = [0.0] * len(params)
adam_step_l2(params, 1, m_list, v_list, lr=0.1)

sau = [p.data for p in params]
so_doi = sum(1 for a, b in zip(truoc, sau) if abs(a - b) > 1e-9)
so_con_grad = sum(1 for p in params if abs(p.grad) > 1e-9)

print("so tham so:", len(params))
print("loss:", round(loss.data, 4))
print("moi tham so co grad khac 0:", so_khac_0 == len(params))
print("moi tham so thay doi sau Adam:", so_doi == len(params))
print("grad da reset ve 0:", so_con_grad == 0)
```

```text title=readonly
so tham so: 17
loss: 0.6673
moi tham so co grad khac 0: True
moi tham so thay doi sau Adam: True
grad da reset ve 0: True
```

Ba dấu hiệu cùng lúc: `17` tham số ĐỀU có gradient khác `0` sau MỘT
`backward()` (không đứt đồ thị ở bất kỳ nút nào), cả `17` tham số ĐỀU thay
đổi giá trị sau MỘT bước Adam (bộ tối ưu thực sự chạm được tới mọi tham
số), và gradient ĐÃ reset về `0` (sẵn sàng cho vòng lặp epoch tiếp theo,
không tích luỹ nhầm — gotcha đã học ở `vong-lap-huan-luyen`, q8.2c).
Framework sẵn sàng cho một bài toán thật.
::::

::::predict{#doan_dut_do_thi commitOnce}
Giả sử `Layer.forward` bị viết SAI ở một chỗ — thay vì `z = z +
self.W[j][i] * x[i]`, ai đó lỡ viết `z = Value(z.data + self.W[j][i].data
* x[i].data)` (tính đúng con số bằng `.data` trần, rồi mới bọc lại thành
`Value` MỚI, không nối vào đồ thị cũ).

**Trước khi đọc lại**, bạn đoán: forward pass (giá trị in ra của `logits`)
có còn ĐÚNG số không? Gradient của những tham số PHÍA TRƯỚC điểm lỗi đó
(ví dụ `W` của tầng ẩn) thì sao?

:::opt{correct}
Forward pass vẫn ra ĐÚNG số (`.data` trần vẫn cộng/nhân đúng — số học
không đổi), nhưng gradient của tham số phía trước sẽ tuyệt đối bằng `0`
sau `backward()`, vì `Value(...)` mới không có `_prev` trỏ về node cũ —
`backward()` không có đường nào lan ngược tới đó
:::

:::opt
Forward pass sẽ báo lỗi ngay lập tức, vì trộn `.data` (số Python trần) với
`Value` trong cùng biểu thức là một phép toán không hợp lệ
::why
Gần đúng ở việc nhận ra đây là một cách viết "trộn" khác thường — quan sát
đó không sai.

Chỗ lệch: `.data` của một `Value` chỉ là một số `float` Python bình
thường; cộng/nhân các số `float` với nhau rồi bọc kết quả vào `Value(...)`
là hợp lệ về mặt CÚ PHÁP và SỐ HỌC — Python không báo lỗi gì cả. Vấn đề
không nằm ở "chạy được hay không" mà ở việc `Value` MỚI đó không mang theo
lịch sử tính toán (`_prev`) của các `Value` cũ, nên đồ thị bị CẮT tại
đúng điểm này — một lỗi ÂM THẦM, không phải một lỗi runtime ồn ào.
::
:::

:::opt
Cả forward VÀ gradient đều sai, vì bất kỳ chỗ nào phá vỡ quy ước dùng
`Value` cũng lan lỗi ra toàn bộ chương trình
::why
Gần đúng ở việc coi trọng tính NHẤT QUÁN khi dùng `Value` xuyên suốt — một
nguyên tắc đúng.

Chỗ lệch: đây không phải một lỗi lan TOÀN BỘ theo kiểu "một chỗ sai, mọi
thứ sai" — nó là một lỗi CỤC BỘ, tại ĐÚNG một điểm. Forward pass (tính
XUÔI) không hề bị ảnh hưởng, vì phép cộng/nhân trên `.data` vẫn cho đúng
con số. Chỉ backward pass (tính NGƯỢC, dựa vào `_prev`) mới bị cắt đứt tại
đúng điểm đó — và chỉ những tham số NẰM TRƯỚC điểm cắt (theo chiều lan
gradient ngược) mới có `grad = 0`, không phải TOÀN BỘ chương trình.
::
:::
::::

::::code{#xac_nhan_khung_khong_dut}
Hoàn thiện bốn chỗ trống: đếm số tham số có gradient khác `0` sau
`backward()`, gọi `adam_step_l2` với đúng bốn tham số vị trí cộng `lr`,
đếm số tham số ĐÃ thay đổi sau bước Adam, và đếm số tham số CÒN gradient
khác `0` sau bước đó (phải là `0`, vì `adam_step_l2` tự reset).

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

    def relu(self):
        out = Value(max(0.0, self.data), (self,), 'relu')
        def _backward():
            self.grad += (1.0 if self.data > 0 else 0.0) * out.grad
        out._backward = _backward
        return out

    def exp(self):
        e = math.exp(self.data)
        out = Value(e, (self,), 'exp')
        def _backward():
            self.grad += e * out.grad
        out._backward = _backward
        return out

    def log(self):
        l = math.log(self.data)
        out = Value(l, (self,), 'log')
        def _backward():
            self.grad += (1.0 / self.data) * out.grad
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

def khoi_tao_mlp(seed, kich_thuoc):
    rng = np.random.default_rng(seed)
    tang = []
    dau_vao = kich_thuoc[0]
    for so_ra in kich_thuoc[1:-1]:
        tho = rng.uniform(-0.5, 0.5, size=(so_ra, dau_vao))
        W = [[Value(float(w)) for w in hang] for hang in tho]
        b = [Value(0.0) for _ in range(so_ra)]
        tang.append(Layer(W, b, tanh_kh))
        dau_vao = so_ra
    so_ra = kich_thuoc[-1]
    tho = rng.uniform(-0.5, 0.5, size=(so_ra, dau_vao))
    W = [[Value(float(w)) for w in hang] for hang in tho]
    b = [Value(0.0) for _ in range(so_ra)]
    tang.append(Layer(W, b, dinh_danh_kh))
    return MLP(tang)

def cross_entropy_qua_value(logits, y_true_idx):
    m = max(v.data for v in logits)
    shifted = [v - m for v in logits]
    exps = [v.exp() for v in shifted]
    tong_exp = exps[0]
    for e in exps[1:]:
        tong_exp = tong_exp + e
    log_tong = tong_exp.log()
    return log_tong - shifted[y_true_idx]

def adam_step_l2(params, t, m_list, v_list, lr, lam=0.0, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps) + lr * lam * p.data
        p.grad = 0.0


mlp = khoi_tao_mlp(seed=5, kich_thuoc=[2, 3, 2])
params = mlp.params()
truoc = [p.data for p in params]

x = [Value(0.6), Value(-0.4)]
logits = mlp.forward(x)
loss = cross_entropy_qua_value(logits, 1)
loss.backward()

so_khac_0 = ___                       # sum(1 for p in params if abs(p.grad) > 1e-9)

m_list = [0.0] * len(params)
v_list = [0.0] * len(params)
___                                    # adam_step_l2(params, 1, m_list, v_list, lr=0.1)

sau = [p.data for p in params]
so_doi = ___                          # sum(1 for a, b in zip(truoc, sau) if abs(a - b) > 1e-9)
so_con_grad = ___                     # sum(1 for p in params if abs(p.grad) > 1e-9)

print(len(params))
print(round(loss.data, 4))
print(so_khac_0 == len(params))
print(so_doi == len(params))
print(so_con_grad == 0)
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

    def relu(self):
        out = Value(max(0.0, self.data), (self,), 'relu')
        def _backward():
            self.grad += (1.0 if self.data > 0 else 0.0) * out.grad
        out._backward = _backward
        return out

    def exp(self):
        e = math.exp(self.data)
        out = Value(e, (self,), 'exp')
        def _backward():
            self.grad += e * out.grad
        out._backward = _backward
        return out

    def log(self):
        l = math.log(self.data)
        out = Value(l, (self,), 'log')
        def _backward():
            self.grad += (1.0 / self.data) * out.grad
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

def khoi_tao_mlp(seed, kich_thuoc):
    rng = np.random.default_rng(seed)
    tang = []
    dau_vao = kich_thuoc[0]
    for so_ra in kich_thuoc[1:-1]:
        tho = rng.uniform(-0.5, 0.5, size=(so_ra, dau_vao))
        W = [[Value(float(w)) for w in hang] for hang in tho]
        b = [Value(0.0) for _ in range(so_ra)]
        tang.append(Layer(W, b, tanh_kh))
        dau_vao = so_ra
    so_ra = kich_thuoc[-1]
    tho = rng.uniform(-0.5, 0.5, size=(so_ra, dau_vao))
    W = [[Value(float(w)) for w in hang] for hang in tho]
    b = [Value(0.0) for _ in range(so_ra)]
    tang.append(Layer(W, b, dinh_danh_kh))
    return MLP(tang)

def cross_entropy_qua_value(logits, y_true_idx):
    m = max(v.data for v in logits)
    shifted = [v - m for v in logits]
    exps = [v.exp() for v in shifted]
    tong_exp = exps[0]
    for e in exps[1:]:
        tong_exp = tong_exp + e
    log_tong = tong_exp.log()
    return log_tong - shifted[y_true_idx]

def adam_step_l2(params, t, m_list, v_list, lr, lam=0.0, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps) + lr * lam * p.data
        p.grad = 0.0


mlp = khoi_tao_mlp(seed=5, kich_thuoc=[2, 3, 2])
params = mlp.params()
truoc = [p.data for p in params]

x = [Value(0.6), Value(-0.4)]
logits = mlp.forward(x)
loss = cross_entropy_qua_value(logits, 1)
loss.backward()

so_khac_0 = sum(1 for p in params if abs(p.grad) > 1e-9)

m_list = [0.0] * len(params)
v_list = [0.0] * len(params)
adam_step_l2(params, 1, m_list, v_list, lr=0.1)

sau = [p.data for p in params]
so_doi = sum(1 for a, b in zip(truoc, sau) if abs(a - b) > 1e-9)
so_con_grad = sum(1 for p in params if abs(p.grad) > 1e-9)

print(len(params))
print(round(loss.data, 4))
print(so_khac_0 == len(params))
print(so_doi == len(params))
print(so_con_grad == 0)
```

```python title=test
assert len(params) == 17, f"MLP 2-3-2 phai co dung 17 tham so -- dang ra {len(params)}"
assert round(loss.data, 4) == 0.6673, f"loss sai -- dang ra {round(loss.data, 4)}"
assert so_khac_0 == 17, f"CA 17 tham so phai co grad khac 0 sau backward() -- dang ra so_khac_0={so_khac_0}"
assert so_doi == 17, f"CA 17 tham so phai thay doi gia tri sau mot buoc Adam -- dang ra so_doi={so_doi}"
assert so_con_grad == 0, f"grad phai duoc RESET ve 0 sau adam_step_l2 -- dang ra so_con_grad={so_con_grad}"

# rieng kiem tra TUNG tham so mot, khong chi dem tong -- neu co dung MOT
# tham so khong doi (vd bi bo sot khoi params() do loi khac), tong so_doi
# se tut xuong 16, van co the bi hieu nham la "gan dung" -- vong lap nay
# doi hoi CHINH XAC tuyet doi.
for i, (a, b) in enumerate(zip(truoc, sau)):
    assert abs(a - b) > 1e-9, f"tham so thu {i} khong thay doi sau buoc Adam (truoc={a}, sau={b})"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `so_khac_0`: đếm số `p` trong `params` có `abs(p.grad) > 1e-9` — `sum(1 for p in params if abs(p.grad) > 1e-9)`. Chỗ gọi Adam: đúng bốn tham số vị trí cộng `lr` — `adam_step_l2(params, 1, m_list, v_list, lr=0.1)` (đối số thứ hai là `t=1`, epoch đầu tiên). `so_doi`: đếm số cặp `(a, b)` trong `zip(truoc, sau)` có `abs(a - b) > 1e-9` — `sum(1 for a, b in zip(truoc, sau) if abs(a - b) > 1e-9)`. `so_con_grad`: giống hệt công thức `so_khac_0`, gọi LẠI sau bước Adam — `sum(1 for p in params if abs(p.grad) > 1e-9)`.
- kind: strategy
  body: 'so_khac_0: `sum(1 for p in params if abs(p.grad) > 1e-9)`. gọi Adam: `adam_step_l2(params, 1, m_list, v_list, lr=0.1)`. so_doi: `sum(1 for a, b in zip(truoc, sau) if abs(a - b) > 1e-9)`. so_con_grad: `sum(1 for p in params if abs(p.grad) > 1e-9)`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `sum(1 for p in params if abs(p.grad) > 1e-9)`, `adam_step_l2(params, 1, m_list, v_list, lr=0.1)`, `sum(1 for a, b in zip(truoc, sau) if abs(a - b) > 1e-9)`, và `sum(1 for p in params if abs(p.grad) > 1e-9)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: so_khac_0/so_con_grad phai dem THAT bang abs(p.grad) (khong duoc chep san dung bang so tham so); phai GOI THAT adam_step_l2 (khong duoc bo qua buoc cap nhat); so_doi phai dem THAT qua zip(truoc, sau) (khong duoc chep san)
  requireAst:
  - kind: uses-call, target: abs, min: 3
  - kind: uses-call, target: adam_step_l2, min: 1
  - kind: uses-name, target: truoc, min: 1
  - kind: uses-name, target: sau, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (abs=3: mot lan trong so_khac_0,
  # mot lan trong so_doi, mot lan trong so_con_grad; adam_step_l2=1: dinh
  # nghia [khong tinh, la FunctionDef], dung mot lan GOI THAT; truoc=1:
  # doc trong zip(truoc, sau) [dong gan "truoc = [...]" la Store, khong
  # tinh]; sau=1: tuong tu).
  #
  # BA CHEAT NGUY HIEM: hardcode so_khac_0 = len(params), so_doi =
  # len(params), hoac so_con_grad = 0 (thay vi tinh THAT) déu cho DUNG
  # con so ma mot khung dung dan se cho ra -- da tu kiem chung bang Python
  # that: ca ba cheat nay qua SACH tier output/tests (in ra dung
  # "True True True" giong het loi giai dung), CHI static (dem so lan
  # goi abs va uses-name cho truoc/sau) moi bat duoc. Day la ly do CHINH
  # bai nay can tier static: mot khung dung se cho dung con so 17, nhung
  # mot ho so gia (khong he kiem tra gi ca) cung cho dung con so 17 -- chi
  # nhin vao CACH TINH moi phan biet duoc.
  # Cheat rieng "khong goi adam_step_l2 (pass)" bi chan CA static (dem ve
  # 0) LAN output (so_doi/so_con_grad ve False vi tham so khong doi, grad
  # khong reset) -- phong thu kep.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^17\\n0\\.6673\\nTrue\\nTrue\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`17/17` tham số có gradient, `17/17` tham số đổi sau Adam, grad reset về
`0`. Framework nguyên vẹn, sẵn sàng đối đầu hai vòng tròn.
::::

::::reflect{#nghi-lai}
Framework đã xác nhận chạy nhất quán — không đứt đồ thị, không tham số nào
bị bỏ sót. Nhưng bài kiểm tra vừa rồi dùng một MLP CHƯA HUẤN LUYỆN (một
bước Adam duy nhất), trên một điểm dữ liệu DUY NHẤT.

Bài sau đem đúng framework này — nguyên vẹn, không viết lại thêm gì — huấn
luyện thật, nhiều epoch, trên đúng `8` điểm hai vòng tròn mà logistic
regression đã thất bại ở bài đầu. Đây là câu trả lời thật cho câu hỏi track
T8.2 tồn tại để trả lời.
::::

::::checkpoint{mastery=0.85}
::::
