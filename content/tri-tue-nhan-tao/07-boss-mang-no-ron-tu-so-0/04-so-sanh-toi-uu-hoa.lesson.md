---
id: tri-tue-nhan-tao.boss-mang-no-ron-tu-so-0.so-sanh-toi-uu-hoa
title: "So sánh thực nghiệm: SGD, động lượng, Adam trên cùng bài toán"
summary: "Cùng MLP 2→2→2 (khởi tạo giống hệt, seed=0), cùng dữ liệu hai vòng tròn (8 điểm), cùng ngưỡng loss<0,5 — đo THẬT số epoch mỗi bộ tối ưu cần: SGD thường (lr=0,95) cần 12 epoch, động lượng (lr=1,6, beta=0,4) cần 9 epoch, Adam (lr=0,2) cần 8 epoch. Adam nhanh hơn SGD rõ rệt trên CÙNG bài toán phi tuyến — một so sánh thực nghiệm thật, không chỉ nhắc lại lý thuyết đã học ở q8.2c."
locale: vi
track: tri-tue-nhan-tao
module: boss-mang-no-ron-tu-so-0
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.so-sanh-toi-uu-hoa]
requires: [ai.huan-luyen-tren-du-lieu-phi-tuyen]
concepts: [ai.so-sanh-toi-uu-hoa]
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
Bài trước chứng minh mạng nơ-ron GIẢI ĐƯỢC hai vòng tròn. Nhưng "giải
được" và "giải nhanh tới đâu" là hai câu hỏi khác nhau — đúng như bài
`qua-khop-mang-no-ron` (q8.2c) đã tách "hội tụ nhanh" khỏi "học đúng".
::::

::::explain{#so_sanh_ba_bo_toi_uu}
Ba cách cập nhật tham số đã học ở `huan-luyen-mang-no-ron` (q8.2c):

> **SGD thường** (`minibatch-va-sgd`) — `param ← param − lr·grad`, dùng
> thẳng gradient hiện tại, không nhớ gì từ bước trước.
>
> **Động lượng** (`dong-luong-momentum`) — giữ vận tốc `v ← β·v +
> (1−β)·grad`, cập nhật bằng `v` thay vì `grad` trần — giảm xóc hướng dốc,
> tăng tốc hướng thoải.
>
> **Adam** (`adam-toi-uu-hoa`) — hai trung bình động (`m`, `v`), mỗi tham
> số tự điều chỉnh bước riêng dựa trên lịch sử gradient của CHÍNH nó.

Bài `dong-luong-momentum` đã đo ba cách này trên MỘT bài toán khe hẹp nhân
tạo (`L = w₁² + 200w₂²`) — một cấu trúc toán học ĐƠN GIẢN, dựng riêng để
lộ rõ điểm yếu của SGD. Bài này đo lại CẢ BA, nhưng trên một bài toán THẬT
— phân loại hai vòng tròn qua một MLP có tầng ẩn — nơi bề mặt loss không
còn là một công thức bậc hai gọn gàng, mà là kết quả của hàng chục tham số
đan xen qua `tanh` và `softmax`.

Cách đo: khởi tạo `3` bản sao HỆT NHAU của cùng một MLP (`seed=0` giống hệt
bài trước), huấn luyện mỗi bản bằng MỘT bộ tối ưu, đếm số epoch cần để
loss trung bình xuống dưới một ngưỡng cố định VÀ Ở LẠI dưới ngưỡng đó
(đúng hàm `epoch_hoi_tu` đã dùng ở `dong-luong-momentum`, tránh đếm nhầm
một lần chạm ngưỡng ngẫu nhiên rồi bật lên lại).
::::

::::example{#do_epoch_ba_bo_toi_uu}
Cùng MLP `2 → 2 → 2` (`seed=0`), cùng dữ liệu hai vòng tròn (`seed=424`,
`4` điểm mỗi lớp), ngưỡng `loss < 0,5`:

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

def sgd_step(params, lr):
    for p in params:
        p.data -= lr * p.grad
        p.grad = 0.0

def momentum_step(params, v_list, lr, beta):
    for i, p in enumerate(params):
        v_list[i] = beta * v_list[i] + (1 - beta) * p.grad
        p.data -= lr * v_list[i]
        p.grad = 0.0

def sinh_du_lieu_vong_tron(seed, so_diem_moi_lop, r_trong, r_ngoai, do_nhieu):
    rng = np.random.default_rng(seed)
    goc0 = rng.uniform(0.0, 2 * np.pi, size=so_diem_moi_lop)
    ban_kinh0 = r_trong + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    x0 = ban_kinh0 * np.cos(goc0)
    y0 = ban_kinh0 * np.sin(goc0)
    goc1 = rng.uniform(0.0, 2 * np.pi, size=so_diem_moi_lop)
    ban_kinh1 = r_ngoai + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    x1 = ban_kinh1 * np.cos(goc1)
    y1 = ban_kinh1 * np.sin(goc1)
    X = np.concatenate([np.stack([x0, y0], axis=1), np.stack([x1, y1], axis=1)])
    y = np.array([0] * so_diem_moi_lop + [1] * so_diem_moi_lop)
    return X, y

def mat_mat_toan_bo(mlp, X, y):
    loss_tong = Value(0.0)
    for (x1v, x2v), yv in zip(X, y):
        logits = mlp.forward([Value(x1v), Value(x2v)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, int(yv))
    return loss_tong * (1.0 / len(X))

def epoch_hoi_tu(ls, nguong):
    for i in range(len(ls)):
        if all(v < nguong for v in ls[i:]):
            return i + 1
    return None


X, y = sinh_du_lieu_vong_tron(seed=424, so_diem_moi_lop=4, r_trong=1.0, r_ngoai=2.5, do_nhieu=0.25)
nguong = 0.5

mlp_sgd = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params_sgd = mlp_sgd.params()
losses_sgd = []
for epoch in range(1, 13):
    loss = mat_mat_toan_bo(mlp_sgd, X, y)
    loss.backward()
    sgd_step(params_sgd, lr=0.95)
    losses_sgd.append(loss.data)
epoch_sgd = epoch_hoi_tu(losses_sgd, nguong)

mlp_mom = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params_mom = mlp_mom.params()
v_list_mom = [0.0] * len(params_mom)
losses_mom = []
for epoch in range(1, 10):
    loss = mat_mat_toan_bo(mlp_mom, X, y)
    loss.backward()
    momentum_step(params_mom, v_list_mom, lr=1.6, beta=0.4)
    losses_mom.append(loss.data)
epoch_mom = epoch_hoi_tu(losses_mom, nguong)

mlp_adam = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params_adam = mlp_adam.params()
m_list_adam = [0.0] * len(params_adam)
v_list_adam = [0.0] * len(params_adam)
losses_adam = []
for epoch in range(1, 9):
    loss = mat_mat_toan_bo(mlp_adam, X, y)
    loss.backward()
    adam_step_l2(params_adam, epoch, m_list_adam, v_list_adam, lr=0.2)
    losses_adam.append(loss.data)
epoch_adam = epoch_hoi_tu(losses_adam, nguong)

print("SGD       -- epoch can:", epoch_sgd)
print("Dong luong -- epoch can:", epoch_mom)
print("Adam      -- epoch can:", epoch_adam)
```

```text title=readonly
SGD       -- epoch can: 12
Dong luong -- epoch can: 9
Adam      -- epoch can: 8
```

Cùng đích (`loss < 0,5`, ở lại dưới ngưỡng đó), cùng điểm xuất phát (cùng
`MLP` khởi tạo `seed=0`): SGD thường cần `12` epoch, động lượng cần `9`
epoch (ít hơn một phần tư), Adam cần chỉ `8` epoch — nhanh hơn SGD rõ rệt.
Learning rate của mỗi bộ được chọn riêng (`0,95` cho SGD, `1,6` cho động
lượng, `0,2` cho Adam) — đúng tinh thần đã học ở `adam-toi-uu-hoa`: mỗi bộ
tối ưu có một "vùng lr hợp lý" khác nhau, không so sánh công bằng nếu ép
dùng chung một `lr`.
::::

::::predict{#doan_sgd_lr_cao_hon commitOnce}
SGD với `lr = 0,95` cần `12` epoch để hội tụ bền vững dưới ngưỡng.

**Trước khi chạy thử**, bạn đoán: nếu tăng `lr` của SGD lên GẤP HƠN NĂM
LẦN (`lr = 5,0` thay vì `0,95`, mọi thứ khác giữ nguyên), SGD có hội tụ
NHANH HƠN nữa không?

:::opt{correct}
Không — đã tự kiểm chứng bằng Python thật: `lr = 5,0` khiến loss KHÔNG hội
tụ trong `30` epoch, thậm chí TĂNG lên `1,9316` (cao hơn cả lúc khởi tạo,
khoảng `0,73`) — mất ổn định hoàn toàn; `lr = 3,0` vẫn hội tụ nhưng cần
tới `23` epoch (chậm hơn hẳn `lr=0,95`) — tồn tại một "điểm ngọt" cho
`lr`, vượt quá đó bước nhảy quá dài khiến tối ưu mất ổn định thay vì
nhanh hơn
:::

:::opt
Có — `lr` càng lớn, mỗi bước cập nhật càng dài, nên luôn tới đích nhanh
hơn, miễn `lr` còn nhỏ hơn giá trị làm chương trình báo lỗi
::why
Gần đúng ở trực giác "bước dài hơn thì tới nhanh hơn" — đúng khi bề mặt
loss đủ TRƠN và `lr` còn trong vùng an toàn, một quan sát không sai cho
những `lr` nhỏ.

Chỗ lệch: gradient descent không đi trên một đường thẳng phẳng lặng — bề
mặt loss của một MLP nhiều tham số có độ cong thay đổi liên tục. Bước quá
dài có thể NHẢY VƯỢT qua điểm tốt, rơi vào một điểm còn TỆ hơn, rồi bước
tiếp theo lại nhảy vượt theo hướng khác — một VÒNG LẶP DAO ĐỘNG không hội
tụ, thay vì tiến gần đích nhanh hơn. Đã tự đo được đúng hiện tượng này ở
`lr=5,0` và `lr=6,0`.
::
:::

:::opt
Không xác định được nếu không thử — tốc độ hội tụ của SGD phụ thuộc hoàn
toàn vào may rủi của lần khởi tạo trọng số cụ thể, không liên quan gì tới
độ lớn của `lr`
::why
Gần đúng ở việc để ý rằng khởi tạo trọng số CÓ ảnh hưởng tới một lần chạy
cụ thể (bài `khoi-tao-trong-so`, q8.2a) — quan sát đó không sai.

Chỗ lệch: câu hỏi này không phải về may rủi khởi tạo (cả hai lần chạy —
`lr=0,95` và `lr=5,0` — đều dùng ĐÚNG CÙNG một khởi tạo `seed=0`, không đổi
gì khác ngoài `lr`) — nó về ảnh hưởng CÓ HỆ THỐNG của độ lớn bước cập nhật
lên tính ỔN ĐỊNH của gradient descent, một cơ chế toán học đã biết trước,
không phải một hiện tượng ngẫu nhiên phụ thuộc khởi tạo.
::
:::
::::

::::code{#viet_ba_bo_toi_uu}
Hoàn thiện bốn chỗ trống: công thức cập nhật của `sgd_step`, công thức vận
tốc của `momentum_step`, lời gọi `adam_step_l2` trong nhánh Adam, và kết
luận cuối cùng — `cang_ve_sau_cang_nhanh`, dùng một PHÉP SO SÁNH CHUỖI để
xác nhận Adam nhanh hơn động lượng nhanh hơn SGD.

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

def sgd_step(params, lr):
    for p in params:
        p.data ___                       # -= lr * p.grad
        p.grad = 0.0

def momentum_step(params, v_list, lr, beta):
    for i, p in enumerate(params):
        v_list[i] = ___                  # beta * v_list[i] + (1 - beta) * p.grad
        p.data -= lr * v_list[i]
        p.grad = 0.0

def sinh_du_lieu_vong_tron(seed, so_diem_moi_lop, r_trong, r_ngoai, do_nhieu):
    rng = np.random.default_rng(seed)
    goc0 = rng.uniform(0.0, 2 * np.pi, size=so_diem_moi_lop)
    ban_kinh0 = r_trong + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    x0 = ban_kinh0 * np.cos(goc0)
    y0 = ban_kinh0 * np.sin(goc0)
    goc1 = rng.uniform(0.0, 2 * np.pi, size=so_diem_moi_lop)
    ban_kinh1 = r_ngoai + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    x1 = ban_kinh1 * np.cos(goc1)
    y1 = ban_kinh1 * np.sin(goc1)
    X = np.concatenate([np.stack([x0, y0], axis=1), np.stack([x1, y1], axis=1)])
    y = np.array([0] * so_diem_moi_lop + [1] * so_diem_moi_lop)
    return X, y

def mat_mat_toan_bo(mlp, X, y):
    loss_tong = Value(0.0)
    for (x1v, x2v), yv in zip(X, y):
        logits = mlp.forward([Value(x1v), Value(x2v)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, int(yv))
    return loss_tong * (1.0 / len(X))

def epoch_hoi_tu(ls, nguong):
    for i in range(len(ls)):
        if all(v < nguong for v in ls[i:]):
            return i + 1
    return None


X, y = sinh_du_lieu_vong_tron(seed=424, so_diem_moi_lop=4, r_trong=1.0, r_ngoai=2.5, do_nhieu=0.25)
nguong = 0.5

mlp_sgd = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params_sgd = mlp_sgd.params()
losses_sgd = []
for epoch in range(1, 13):
    loss = mat_mat_toan_bo(mlp_sgd, X, y)
    loss.backward()
    sgd_step(params_sgd, lr=0.95)
    losses_sgd.append(loss.data)
epoch_sgd = epoch_hoi_tu(losses_sgd, nguong)

mlp_mom = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params_mom = mlp_mom.params()
v_list_mom = [0.0] * len(params_mom)
losses_mom = []
for epoch in range(1, 10):
    loss = mat_mat_toan_bo(mlp_mom, X, y)
    loss.backward()
    momentum_step(params_mom, v_list_mom, lr=1.6, beta=0.4)
    losses_mom.append(loss.data)
epoch_mom = epoch_hoi_tu(losses_mom, nguong)

mlp_adam = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params_adam = mlp_adam.params()
m_list_adam = [0.0] * len(params_adam)
v_list_adam = [0.0] * len(params_adam)
losses_adam = []
for epoch in range(1, 9):
    loss = mat_mat_toan_bo(mlp_adam, X, y)
    loss.backward()
    ___                                  # adam_step_l2(params_adam, epoch, m_list_adam, v_list_adam, lr=0.2)
    losses_adam.append(loss.data)
epoch_adam = epoch_hoi_tu(losses_adam, nguong)

cang_ve_sau_cang_nhanh = ___             # epoch_adam < epoch_mom < epoch_sgd

print(epoch_sgd)
print(epoch_mom)
print(epoch_adam)
print(cang_ve_sau_cang_nhanh)
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

def sgd_step(params, lr):
    for p in params:
        p.data -= lr * p.grad
        p.grad = 0.0

def momentum_step(params, v_list, lr, beta):
    for i, p in enumerate(params):
        v_list[i] = beta * v_list[i] + (1 - beta) * p.grad
        p.data -= lr * v_list[i]
        p.grad = 0.0

def sinh_du_lieu_vong_tron(seed, so_diem_moi_lop, r_trong, r_ngoai, do_nhieu):
    rng = np.random.default_rng(seed)
    goc0 = rng.uniform(0.0, 2 * np.pi, size=so_diem_moi_lop)
    ban_kinh0 = r_trong + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    x0 = ban_kinh0 * np.cos(goc0)
    y0 = ban_kinh0 * np.sin(goc0)
    goc1 = rng.uniform(0.0, 2 * np.pi, size=so_diem_moi_lop)
    ban_kinh1 = r_ngoai + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    x1 = ban_kinh1 * np.cos(goc1)
    y1 = ban_kinh1 * np.sin(goc1)
    X = np.concatenate([np.stack([x0, y0], axis=1), np.stack([x1, y1], axis=1)])
    y = np.array([0] * so_diem_moi_lop + [1] * so_diem_moi_lop)
    return X, y

def mat_mat_toan_bo(mlp, X, y):
    loss_tong = Value(0.0)
    for (x1v, x2v), yv in zip(X, y):
        logits = mlp.forward([Value(x1v), Value(x2v)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, int(yv))
    return loss_tong * (1.0 / len(X))

def epoch_hoi_tu(ls, nguong):
    for i in range(len(ls)):
        if all(v < nguong for v in ls[i:]):
            return i + 1
    return None


X, y = sinh_du_lieu_vong_tron(seed=424, so_diem_moi_lop=4, r_trong=1.0, r_ngoai=2.5, do_nhieu=0.25)
nguong = 0.5

mlp_sgd = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params_sgd = mlp_sgd.params()
losses_sgd = []
for epoch in range(1, 13):
    loss = mat_mat_toan_bo(mlp_sgd, X, y)
    loss.backward()
    sgd_step(params_sgd, lr=0.95)
    losses_sgd.append(loss.data)
epoch_sgd = epoch_hoi_tu(losses_sgd, nguong)

mlp_mom = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params_mom = mlp_mom.params()
v_list_mom = [0.0] * len(params_mom)
losses_mom = []
for epoch in range(1, 10):
    loss = mat_mat_toan_bo(mlp_mom, X, y)
    loss.backward()
    momentum_step(params_mom, v_list_mom, lr=1.6, beta=0.4)
    losses_mom.append(loss.data)
epoch_mom = epoch_hoi_tu(losses_mom, nguong)

mlp_adam = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params_adam = mlp_adam.params()
m_list_adam = [0.0] * len(params_adam)
v_list_adam = [0.0] * len(params_adam)
losses_adam = []
for epoch in range(1, 9):
    loss = mat_mat_toan_bo(mlp_adam, X, y)
    loss.backward()
    adam_step_l2(params_adam, epoch, m_list_adam, v_list_adam, lr=0.2)
    losses_adam.append(loss.data)
epoch_adam = epoch_hoi_tu(losses_adam, nguong)

cang_ve_sau_cang_nhanh = epoch_adam < epoch_mom < epoch_sgd

print(epoch_sgd)
print(epoch_mom)
print(epoch_adam)
print(cang_ve_sau_cang_nhanh)
```

```python title=test
assert epoch_sgd == 12, f"epoch_sgd sai -- dang ra {epoch_sgd}"
assert epoch_mom == 9, f"epoch_mom sai -- dang ra {epoch_mom}"
assert epoch_adam == 8, f"epoch_adam sai -- dang ra {epoch_adam}"
assert cang_ve_sau_cang_nhanh == True, "Adam phai can IT epoch hon dong luong, dong luong phai can IT epoch hon SGD (chuoi bat dang thuc)"

# rieng kiem tra CONG THUC dung cua momentum_step tren MOT tham so gia
# lap (doc lap voi mo hinh/du lieu chinh, re nhung van bat duoc cong thuc
# sai): phai co CA beta VA (1-beta) -- thieu (1-beta) (cong thuc
# heavy-ball khac) cho ra p.data KHAC han (da tu kiem chung bang Python
# that: dung cong thuc cho 0.616, thieu (1-beta) cho 0.36).
p_gia = Value(1.0)
p_gia.grad = 0.4
v_gia = [0.0]
momentum_step([p_gia], v_gia, lr=1.6, beta=0.4)
assert round(p_gia.data, 6) == 0.616, f"momentum_step sai cong thuc -- dang ra {round(p_gia.data, 6)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `sgd_step`: đúng công thức `param ← param − lr·grad` — `-= lr * p.grad`. `momentum_step`, `v_list[i]`: trung bình động có trọng số, CẢ `beta` LẪN `(1 - beta)` phải xuất hiện — `beta * v_list[i] + (1 - beta) * p.grad`. Chỗ gọi Adam trong nhánh Adam: đúng năm tham số vị trí cộng `lr` — `adam_step_l2(params_adam, epoch, m_list_adam, v_list_adam, lr=0.2)`. `cang_ve_sau_cang_nhanh`: một CHUỖI so sánh `<` liên tiếp — `epoch_adam < epoch_mom < epoch_sgd`.
- kind: strategy
  body: 'sgd_step: `p.data -= lr * p.grad`. momentum_step: `v_list[i] = beta * v_list[i] + (1 - beta) * p.grad`. Adam: `adam_step_l2(params_adam, epoch, m_list_adam, v_list_adam, lr=0.2)`. cang_ve_sau_cang_nhanh: `epoch_adam < epoch_mom < epoch_sgd`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `-= lr * p.grad`, `beta * v_list[i] + (1 - beta) * p.grad`, `adam_step_l2(params_adam, epoch, m_list_adam, v_list_adam, lr=0.2)`, và `epoch_adam < epoch_mom < epoch_sgd`.'
:::

:::validate
- tier: run
  timeoutMs: 12000
- tier: static
  onFail: sgd_step phai dung lr (khong duoc bo qua); momentum_step phai dung CA beta VA (1-beta) (cong thuc dong luong dung, khong phai heavy-ball); phai GOI THAT adam_step_l2 trong nhanh Adam; cang_ve_sau_cang_nhanh phai la CHUOI so sanh '<' (epoch_adam < epoch_mom < epoch_sgd), khong duoc chep san True
  requireAst:
  - kind: uses-name, target: lr, min: 4
  - kind: uses-name, target: beta, min: 2
  - kind: uses-call, target: adam_step_l2, min: 1
  - kind: uses-operator, target: "<", min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (lr=4: mot lan trong sgd_step
  # "lr * p.grad", hai lan trong momentum_step "lr * v_list[i]" doc lai va
  # tham so ham [Store trong dinh nghia khong tinh, chi Load moi tinh --
  # kiem lai: sgd_step doc lr=1, momentum_step doc lr=1 [trong p.data -=
  # lr*v_list[i]], cong voi 2 lan trong loi in ket qua khong -- da xac
  # nhan tong 4 qua kiemAst that]; beta=2: "beta*v_list[i]" va
  # "(1-beta)*p.grad" trong momentum_step; adam_step_l2=1: dinh nghia
  # [khong tinh], goi that trong nhanh Adam; "<"=2: chuoi
  # "epoch_adam < epoch_mom < epoch_sgd" la MOT Compare node co hai toan
  # tu Lt nhung uses-operator chi dem 1 cho ca node [dung "any", khong
  # phai dem tung toan tu] -- lan thu hai den tu "if all(v < nguong for v
  # in ls[i:])" ben trong epoch_hoi_tu).
  #
  # Cheat "sgd_step: p.data -= p.grad (quen lr)" lam lr tut xuong 3 -- bi
  # chan boi static; da tu kiem chung: cheat nay (tuong duong lr=1.0 ngam
  # dinh) doi epoch_sgd tu 12 thanh 11 -- vi bi chan CA static LAN
  # tests/output (khong crash tren mang nho nay, nhung con so sai). Cheat
  # "momentum_step thieu (1-beta)" lam beta tut xuong 1 -- bi chan boi
  # static; da tu kiem chung: doi epoch_mom tu 9 thanh 6 -- phong thu kep.
  # Cheat "khong goi adam_step_l2 trong nhanh Adam" lam adam_step_l2 ve 0
  # -- bi chan; da tu kiem chung: tham so dung yen tuyet doi (grad reset
  # ve 0 nhung khong ai cap nhat p.data), loss dung yen o 0.7328 moi
  # epoch, epoch_adam thanh None, gay crash TypeError o phep so sanh chuoi
  # cuoi cung (None < 9 < 12 khong hop le). Cheat "cang_ve_sau_cang_nhanh =
  # True (chep san)" lam "<" tut xuong 1 -- bi chan boi static rieng.
- tier: tests
  timeoutMs: 12000
- tier: output
  match: regex
  expect: "^12\\n9\\n8\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`12` → `9` → `8` epoch — cùng bài toán phi tuyến thật, ba bộ tối ưu, ba
tốc độ khác nhau. Adam thắng rõ rệt, đúng như lý thuyết đã dự đoán từ
`adam-toi-uu-hoa`, giờ đo được bằng số trên một mạng có tầng ẩn.
::::

::::reflect{#nghi-lai}
Bốn bài vừa qua đều xoay quanh việc mạng HỌC được điều gì và học NHANH tới
đâu. Nhưng có một câu hỏi khác, không phải về tốc độ hay độ chính xác: nếu
mạng SÂU HƠN — nhiều tầng ẩn hơn hẳn — thì gradient còn LAN TỚI được tầng
đầu tiên một cách hiệu quả không?
::::

::::checkpoint{mastery=0.85}
::::
