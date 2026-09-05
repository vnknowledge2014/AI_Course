---
id: tri-tue-nhan-tao.boss-mang-no-ron-tu-so-0.gradient-bien-mat-that
title: "Gradient biến mất thật: mạng càng sâu, tầng đầu càng mù"
summary: "Cùng một đầu vào, cùng seed khởi tạo (seed=7): mạng SÂU (6 tầng ẩn tanh, width=4) có gradient trung bình tầng đầu 0,0212 so với tầng cuối 0,4084 (tỉ lệ 0,0518); mạng NÔNG (1 tầng ẩn) có gradient tầng đầu 0,2065 so với tầng cuối 0,7201 (tỉ lệ 0,2868). Mạng sâu có tỉ lệ NHỎ HƠN mạng nông khoảng 5,5 lần, VÀ gradient tuyệt đối ở tầng đầu nhỏ hơn khoảng 10 lần — vanishing gradient đo được bằng số thật, không suy luận, bắc cầu tới T8.3."
locale: vi
track: tri-tue-nhan-tao
module: boss-mang-no-ron-tu-so-0
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.gradient-bien-mat-that]
requires: [ai.so-sanh-toi-uu-hoa]
concepts: [ai.gradient-bien-mat-that]
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
Ba bộ tối ưu vừa thi đấu tốc độ trên một mạng NÔNG (một tầng ẩn). Nếu mạng
sâu hơn hẳn — nhiều tầng ẩn nối tiếp nhau — bộ tối ưu nhanh nhất cũng không
cứu được một tầng gradient không tới nổi.
::::

::::explain{#vi_sao_gradient_bien_mat}
Bài `ham-kich-hoat` (q8.2a) đã chỉ ra `tanh` là một hàm BÃO HOÀ — đạo hàm
cục bộ của nó, `1 − tanh(z)²`, luôn nằm trong khoảng `(0, 1]`, và tiến về
`0` khi `|z|` lớn (tanh "bão hoà" ở gần `-1` hoặc `1`). Với backpropagation
(bài `dao-ham-nguoc-chuoi`, q8.2b), gradient lan từ đầu ra NGƯỢC VỀ đầu
vào bằng cách NHÂN liên tiếp các đạo hàm cục bộ của từng tầng đã đi qua —
quy tắc chuỗi.

Hệ quả: nếu MỖI tầng `tanh` chỉ truyền qua một PHẦN của gradient (vì đạo
hàm cục bộ luôn `≤ 1`, thường NHỎ HƠN `1` đáng kể), thì một mạng có NHIỀU
tầng nối tiếp khiến gradient bị NHÂN với nhiều số nhỏ hơn `1` liên tục —
càng lan qua nhiều tầng, gradient đến tầng ĐẦU (gần input) càng NHỎ. Đây
gọi là **vanishing gradient** (gradient biến mất/tiêu biến): tầng gần đầu
vào của một mạng sâu học CHẬM HƠN NHIỀU so với tầng gần đầu ra, chỉ vì
khoảng cách nó phải "vượt qua" để nhận được tín hiệu học là xa hơn, và mỗi
bước trên khoảng cách đó lại co gradient lại một chút.

Đo hiện tượng này: dựng một mạng cùng chiều rộng ở mỗi tầng ẩn nhưng khác
ĐỘ SÂU (số tầng ẩn nối tiếp), forward một điểm, `backward()` MỘT LẦN, rồi
so sánh độ lớn TUYỆT ĐỐI trung bình của gradient (`|p.grad|`, lấy trung
bình trên mọi tham số) ở tầng ĐẦU tiên (gần input nhất) với tầng CUỐI
(gần loss nhất).
::::

::::example{#do_vanishing_gradient_that}
Hai mạng, cùng chiều rộng tầng ẩn (`4` neuron `tanh`), cùng seed khởi tạo
(`seed=7`), cùng đầu vào (`0,6`, `-0,4`), cùng mục tiêu hồi quy (`1,0`, MSE)
— chỉ khác ĐỘ SÂU: mạng SÂU có `6` tầng ẩn liên tiếp, mạng NÔNG chỉ có `1`:

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

def mse_qua_value(preds, targets):
    n = len(preds)
    tong = Value(0.0)
    for p, t in zip(preds, targets):
        err = p - t
        tong = tong + err ** 2
    return tong * (1.0 / n)

def trung_binh_grad_tuyet_doi(layer):
    ps = layer.params()
    return sum(abs(p.grad) for p in ps) / len(ps)


mlp_sau = khoi_tao_mlp(seed=7, kich_thuoc=[2, 4, 4, 4, 4, 4, 4, 1])
pred_sau = mlp_sau.forward([Value(0.6), Value(-0.4)])
loss_sau = mse_qua_value(pred_sau, [1.0])
loss_sau.backward()
g_dau_sau = trung_binh_grad_tuyet_doi(mlp_sau.cac_tang[0])
g_cuoi_sau = trung_binh_grad_tuyet_doi(mlp_sau.cac_tang[-1])
ti_le_sau = g_dau_sau / g_cuoi_sau

mlp_nong = khoi_tao_mlp(seed=7, kich_thuoc=[2, 4, 1])
pred_nong = mlp_nong.forward([Value(0.6), Value(-0.4)])
loss_nong = mse_qua_value(pred_nong, [1.0])
loss_nong.backward()
g_dau_nong = trung_binh_grad_tuyet_doi(mlp_nong.cac_tang[0])
g_cuoi_nong = trung_binh_grad_tuyet_doi(mlp_nong.cac_tang[-1])
ti_le_nong = g_dau_nong / g_cuoi_nong

print("mang SAU  -- so tang:", len(mlp_sau.cac_tang), " g_dau:", round(g_dau_sau, 4), " g_cuoi:", round(g_cuoi_sau, 4), " ti_le:", round(ti_le_sau, 4))
print("mang NONG -- so tang:", len(mlp_nong.cac_tang), " g_dau:", round(g_dau_nong, 4), " g_cuoi:", round(g_cuoi_nong, 4), " ti_le:", round(ti_le_nong, 4))
```

```text title=readonly
mang SAU  -- so tang: 7  g_dau: 0.0212  g_cuoi: 0.4084  ti_le: 0.0518
mang NONG -- so tang: 2  g_dau: 0.2065  g_cuoi: 0.7201  ti_le: 0.2868
```

Mạng sâu (`7` tầng, `6` tầng ẩn `tanh` cộng một tầng ra): gradient trung
bình tầng đầu (`0,0212`) chỉ bằng khoảng một phần `19` gradient tầng cuối
(`0,4084`) — tỉ lệ `0,0518`. Mạng nông (`2` tầng, `1` tầng ẩn): gradient
tầng đầu (`0,2065`) bằng gần một phần `3,5` gradient tầng cuối (`0,7201`)
— tỉ lệ `0,2868`, cao hơn mạng sâu khoảng `5,5` lần. Đáng chú ý hơn: xét
GIÁ TRỊ TUYỆT ĐỐI, gradient tầng đầu của mạng sâu (`0,0212`) nhỏ hơn gradient
tầng đầu của mạng nông (`0,2065`) khoảng `10` lần — dù CẢ HAI mạng đều có
tầng ẩn ĐẦU TIÊN cùng kiến trúc hệt nhau (`4` neuron `tanh`, cùng
`seed=7` khởi tạo giống hệt phần trùng lặp). Sự khác biệt DUY NHẤT là có
bao nhiêu tầng đứng GIỮA tầng đầu và loss — càng nhiều tầng, gradient càng
phải "lách qua" nhiều đạo hàm cục bộ `< 1` liên tiếp trước khi tới được
tầng đầu.
::::

::::predict{#doan_neu_dung_relu commitOnce}
Cả hai mạng ở trên đều dùng `tanh` cho tầng ẩn — một hàm BÃO HOÀ (đạo hàm
cục bộ luôn `≤ 1`).

**Trước khi đọc lại**, bạn đoán: nếu đổi TẤT CẢ tầng ẩn của mạng SÂU sang
dùng `relu` thay vì `tanh` (đạo hàm cục bộ của `relu` là ĐÚNG `1` khi đầu
vào dương, `0` khi âm — không bao giờ nằm strictly giữa `0` và `1`), hiện
tượng gradient càng lan xa càng nhỏ có còn xảy ra HỆT như vậy không?

:::opt{correct}
Không hệt như vậy — khi đầu vào của một neuron `relu` dương, đạo hàm cục
bộ của nó là ĐÚNG `1` (không co gradient lại chút nào ở neuron đó), khác
hẳn `tanh` (đạo hàm luôn NHỎ HƠN `1` một cách nghiêm ngặt trừ khi đầu vào
đúng bằng `0`) — nhưng `relu` có vấn đề RIÊNG của nó (neuron "chết" khi đầu
vào âm, đạo hàm cục bộ đúng bằng `0`, cắt đứt gradient hoàn toàn tại neuron
đó) — đổi hàm kích hoạt đổi CƠ CHẾ gây khó khăn, không xoá bỏ mọi khó khăn
khi mạng sâu
:::

:::opt
Có — mọi hàm kích hoạt phi tuyến đều gây ra vanishing gradient như nhau
khi mạng đủ sâu, không phụ thuộc vào công thức cụ thể của hàm đó
::why
Gần đúng ở việc nhận ra ĐỘ SÂU là một yếu tố chung ảnh hưởng tới việc
gradient lan xa tới đâu — quan sát đó không sai xét riêng phần "độ sâu".

Chỗ lệch: công thức CỤ THỂ của đạo hàm cục bộ MỚI là thứ quyết định
gradient bị co lại BAO NHIÊU ở mỗi tầng, không phải chỉ có việc hàm đó có
phi tuyến hay không. `tanh` có đạo hàm cục bộ luôn nhỏ hơn `1` (trừ đúng
tại `z=0`), gây co gradient MỌI nơi; `relu` có đạo hàm cục bộ đúng `1` ở
vùng dương — không co gradient ở đó. Hai công thức khác nhau tạo ra hai
kiểu khó khăn khác nhau khi mạng sâu, không phải cùng một hiện tượng với
cùng mức độ.
::
:::

:::opt
Không xác định được nếu không đo — không có cách nào suy luận trước hành
vi của `relu` từ công thức đạo hàm của nó
::why
Gần đúng ở tinh thần muốn đo THẬT trước khi kết luận — một nguyên tắc
xuyên suốt track này.

Chỗ lệch: công thức đạo hàm cục bộ của `relu` (`1` khi dương, `0` khi âm
— bài `lop-value-mu-tanh-relu`, q8.2b) đã được HỌC và XÁC NHẬN từ trước,
không cần đo lại từ đầu để biết HƯỚNG khác biệt so với `tanh`: một hàm có
đạo hàm cục bộ đúng bằng `1` (không nhỏ hơn) ở một vùng rõ ràng KHÔNG co
gradient theo cùng cách một hàm có đạo hàm LUÔN nhỏ hơn `1` mọi nơi. Đây
là một suy luận có thể đưa ra TRƯỚC từ công thức đã biết, dù mức độ chính
xác của "cải thiện bao nhiêu" vẫn cần đo thật.
::
:::
::::

::::code{#do_vanishing_gradient}
Hoàn thiện bốn chỗ trống: hàm `trung_binh_grad_tuyet_doi` (trung bình
`|grad|` trên mọi tham số của một tầng), hai tỉ lệ `ti_le_sau`/`ti_le_nong`,
và kết luận cuối cùng — mạng sâu có bị "tiêu biến" MẠNH HƠN mạng nông
không.

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

def mse_qua_value(preds, targets):
    n = len(preds)
    tong = Value(0.0)
    for p, t in zip(preds, targets):
        err = p - t
        tong = tong + err ** 2
    return tong * (1.0 / n)

def trung_binh_grad_tuyet_doi(layer):
    ps = layer.params()
    return ___                        # sum(abs(p.grad) for p in ps) / len(ps)


mlp_sau = khoi_tao_mlp(seed=7, kich_thuoc=[2, 4, 4, 4, 4, 4, 4, 1])
pred_sau = mlp_sau.forward([Value(0.6), Value(-0.4)])
loss_sau = mse_qua_value(pred_sau, [1.0])
loss_sau.backward()
g_dau_sau = trung_binh_grad_tuyet_doi(mlp_sau.cac_tang[0])
g_cuoi_sau = trung_binh_grad_tuyet_doi(mlp_sau.cac_tang[-1])
ti_le_sau = ___                       # g_dau_sau / g_cuoi_sau

mlp_nong = khoi_tao_mlp(seed=7, kich_thuoc=[2, 4, 1])
pred_nong = mlp_nong.forward([Value(0.6), Value(-0.4)])
loss_nong = mse_qua_value(pred_nong, [1.0])
loss_nong.backward()
g_dau_nong = trung_binh_grad_tuyet_doi(mlp_nong.cac_tang[0])
g_cuoi_nong = trung_binh_grad_tuyet_doi(mlp_nong.cac_tang[-1])
ti_le_nong = ___                      # g_dau_nong / g_cuoi_nong

sau_bi_trieu_tieu_manh_hon = ___       # ti_le_sau < ti_le_nong

print(len(mlp_sau.cac_tang), len(mlp_nong.cac_tang))
print(round(g_dau_sau, 4), round(g_cuoi_sau, 4), round(ti_le_sau, 4))
print(round(g_dau_nong, 4), round(g_cuoi_nong, 4), round(ti_le_nong, 4))
print(sau_bi_trieu_tieu_manh_hon)
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

def mse_qua_value(preds, targets):
    n = len(preds)
    tong = Value(0.0)
    for p, t in zip(preds, targets):
        err = p - t
        tong = tong + err ** 2
    return tong * (1.0 / n)

def trung_binh_grad_tuyet_doi(layer):
    ps = layer.params()
    return sum(abs(p.grad) for p in ps) / len(ps)


mlp_sau = khoi_tao_mlp(seed=7, kich_thuoc=[2, 4, 4, 4, 4, 4, 4, 1])
pred_sau = mlp_sau.forward([Value(0.6), Value(-0.4)])
loss_sau = mse_qua_value(pred_sau, [1.0])
loss_sau.backward()
g_dau_sau = trung_binh_grad_tuyet_doi(mlp_sau.cac_tang[0])
g_cuoi_sau = trung_binh_grad_tuyet_doi(mlp_sau.cac_tang[-1])
ti_le_sau = g_dau_sau / g_cuoi_sau

mlp_nong = khoi_tao_mlp(seed=7, kich_thuoc=[2, 4, 1])
pred_nong = mlp_nong.forward([Value(0.6), Value(-0.4)])
loss_nong = mse_qua_value(pred_nong, [1.0])
loss_nong.backward()
g_dau_nong = trung_binh_grad_tuyet_doi(mlp_nong.cac_tang[0])
g_cuoi_nong = trung_binh_grad_tuyet_doi(mlp_nong.cac_tang[-1])
ti_le_nong = g_dau_nong / g_cuoi_nong

sau_bi_trieu_tieu_manh_hon = ti_le_sau < ti_le_nong

print(len(mlp_sau.cac_tang), len(mlp_nong.cac_tang))
print(round(g_dau_sau, 4), round(g_cuoi_sau, 4), round(ti_le_sau, 4))
print(round(g_dau_nong, 4), round(g_cuoi_nong, 4), round(ti_le_nong, 4))
print(sau_bi_trieu_tieu_manh_hon)
```

```python title=test
assert len(mlp_sau.cac_tang) == 7, f"mang sau phai co 7 tang -- dang ra {len(mlp_sau.cac_tang)}"
assert len(mlp_nong.cac_tang) == 2, f"mang nong phai co 2 tang -- dang ra {len(mlp_nong.cac_tang)}"
assert round(g_dau_sau, 4) == 0.0212, f"g_dau_sau sai -- dang ra {round(g_dau_sau, 4)}"
assert round(g_cuoi_sau, 4) == 0.4084, f"g_cuoi_sau sai -- dang ra {round(g_cuoi_sau, 4)}"
assert round(ti_le_sau, 4) == 0.0518, f"ti_le_sau sai -- dang ra {round(ti_le_sau, 4)}"
assert round(g_dau_nong, 4) == 0.2065, f"g_dau_nong sai -- dang ra {round(g_dau_nong, 4)}"
assert round(g_cuoi_nong, 4) == 0.7201, f"g_cuoi_nong sai -- dang ra {round(g_cuoi_nong, 4)}"
assert round(ti_le_nong, 4) == 0.2868, f"ti_le_nong sai -- dang ra {round(ti_le_nong, 4)}"
assert sau_bi_trieu_tieu_manh_hon == True, "mang sau phai co ti le g_dau/g_cuoi NHO HON mang nong (bien mat manh hon)"

# rieng kiem tra gradient tuyet doi tang dau cua mang sau phai NHO HON han
# gradient tuyet doi tang dau cua mang nong -- so sanh TUYET DOI, khong
# chi ti le, cung la mot bang chung doc lap cho vanishing gradient.
assert g_dau_sau < g_dau_nong, "gradient tuyet doi tang dau cua mang SAU phai nho hon mang NONG"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `trung_binh_grad_tuyet_doi`: tổng `|grad|` chia số tham số — `sum(abs(p.grad) for p in ps) / len(ps)`. `ti_le_sau`, `ti_le_nong`: chia gradient tầng đầu cho gradient tầng cuối — `g_dau_sau / g_cuoi_sau` và `g_dau_nong / g_cuoi_nong`. `sau_bi_trieu_tieu_manh_hon`: so sánh hai tỉ lệ vừa tính — `ti_le_sau < ti_le_nong`.
- kind: strategy
  body: 'trung_binh_grad_tuyet_doi: `sum(abs(p.grad) for p in ps) / len(ps)`. ti_le_sau: `g_dau_sau / g_cuoi_sau`. ti_le_nong: `g_dau_nong / g_cuoi_nong`. sau_bi_trieu_tieu_manh_hon: `ti_le_sau < ti_le_nong`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `sum(abs(p.grad) for p in ps) / len(ps)`, `g_dau_sau / g_cuoi_sau`, `g_dau_nong / g_cuoi_nong`, và `ti_le_sau < ti_le_nong`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: trung_binh_grad_tuyet_doi phai dung abs() THAT SU tren p.grad (khong duoc chep san hay bo qua gia tri tuyet doi); ti_le_sau VA ti_le_nong deu phai tinh THAT bang phep chia (khong duoc chep hang so); sau_bi_trieu_tieu_manh_hon phai la PHEP SO SANH '<' (khong duoc chep san True)
  requireAst:
  - kind: uses-call, target: abs, min: 1
  - kind: uses-operator, target: "/", min: 5
  - kind: uses-operator, target: "<", min: 1
  - kind: uses-name, target: ti_le_sau, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (abs=1: dung mot lan trong
  # trung_binh_grad_tuyet_doi; "/"=5: sum(...)/len(ps) trong dinh nghia
  # trung_binh_grad_tuyet_doi [1], "1.0/n" trong mse_qua_value [1],
  # "1.0/self.data" trong Value.log()._backward [1, luon co san], VA hai
  # phep chia ti_le_sau/ti_le_nong [2] -- tong 5, can CA HAI ti_le deu
  # dung phep chia moi dat nguong nay; ti_le_sau=2: doc lai trong
  # sau_bi_trieu_tieu_manh_hon va trong dong in ket qua).
  #
  # Cheat "trung_binh_grad_tuyet_doi tra ve ps[0].grad (khong dung abs,
  # khong chia trung binh)" lam abs=0 VA "/" tut xuong 4 -- bi chan boi
  # ca hai luat. Cheat "ti_le_sau = 0.05 (chep gan dung)" hoac "ti_le_nong
  # = 0.29 (chep gan dung)" deu lam "/" tut xuong 4 -- bi chan; da tu
  # kiem chung: ca hai deu doi so o vi tri thap phan thu 3-4 so voi loi
  # giai dung nen CUNG bi bat boi output/tests, static la phong thu THEM.
  # Cheat "sau_bi_trieu_tieu_manh_hon = True (chep san)" lam "<" tut
  # xuong 0 (chi con "<" ben trong ham epoch_hoi_tu... khong, bai nay
  # khong dung epoch_hoi_tu, nen "<" ve dung 0) -- bi chan boi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^7 2\\n0\\.0212 0\\.4084 0\\.0518\\n0\\.2065 0\\.7201 0\\.2868\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=curious pose=lean-in}
Mạng sâu `6` tầng: gradient tầng đầu chỉ bằng một phần `19` gradient tầng
cuối. Mạng nông `1` tầng: chỉ một phần `3,5`. Vanishing gradient — không
còn là lý thuyết, mà là một con số đo được.
::::

::::reflect{#nghi-lai}
Ba mươi hai khái niệm, bốn quest, một track: T8.2 "Mạng nơ-ron từ số 0"
gần khép lại. q8.2a xây neuron và mạng nhiều tầng. q8.2b xây autograd. q8.2c
ráp thành một vòng lặp huấn luyện đầy đủ. q8.2d (track này) đối đầu dữ
liệu phi tuyến thật, đo tốc độ ba bộ tối ưu, và giờ vừa đo được giới hạn
của chính kiến trúc `tanh`-nhiều-tầng khi mạng đủ sâu.

Vanishing gradient không phải một lỗi có thể sửa bằng cách chọn `lr` khéo
hơn hay huấn luyện lâu hơn — nó là hệ quả CẤU TRÚC của việc nhân nhiều đạo
hàm cục bộ nhỏ hơn `1` liên tiếp. T8.3 "Transformer từ số 0" sẽ giới thiệu
những kiến trúc được thiết kế RIÊNG để giải quyết đúng vấn đề này (kết nối
tắt, chuẩn hoá theo tầng) — nhưng trước khi tới đó, một bài BOSS cuối cùng
sẽ ráp TOÀN BỘ framework của T8.2 lại, đóng track tại `32/32`.
::::

::::checkpoint{mastery=0.85}
::::
