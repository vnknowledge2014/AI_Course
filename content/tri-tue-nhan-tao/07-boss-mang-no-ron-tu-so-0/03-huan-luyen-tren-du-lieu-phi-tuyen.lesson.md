---
id: tri-tue-nhan-tao.boss-mang-no-ron-tu-so-0.huan-luyen-tren-du-lieu-phi-tuyen
title: "Mạng nơ-ron giải được điều logistic regression không giải được"
summary: "MLP 2→2→2 (tầng ẩn tanh, tầng ra softmax qua cross_entropy_qua_value), Adam lr=0,1, 20 epoch, huấn luyện trên ĐÚNG 8 điểm hai vòng tròn của bài 1 — accuracy đạt 8/8 (100%), loss cuối 0,179. So sánh trực tiếp: logistic regression tuyến tính chỉ đạt 4/8 (50%, đúng bằng đoán ngẫu nhiên) trên CÙNG dữ liệu này. Cùng framework, cùng dữ liệu, khác đúng MỘT thứ — một tầng ẩn tanh — biến một mô hình bất lực thành một mô hình hoàn hảo."
locale: vi
track: tri-tue-nhan-tao
module: boss-mang-no-ron-tu-so-0
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.huan-luyen-tren-du-lieu-phi-tuyen]
requires: [ai.rap-framework-mang-no-ron]
concepts: [ai.huan-luyen-tren-du-lieu-phi-tuyen]
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

::::byte{trigger=enter mood=happy pose=jump}
Framework đã xác nhận chạy đúng. Dữ liệu hai vòng tròn đã hạ gục logistic
regression. Giờ là lúc đem cả hai lại gần nhau.
::::

::::explain{#mot_tang_an_thay_doi_tat_ca}
Bài `vi-sao-can-phi-tuyen` (q8.2a) đã trả lời câu hỏi lý thuyết: ghép nhiều
neuron tuyến tính qua một hàm kích hoạt phi tuyến (`tanh`) tạo ra một mạng
có khả năng vẽ đường phân cách CONG, không chỉ đường thẳng. Mỗi neuron ẩn
tự học một đường thẳng CẮT NGANG mặt phẳng theo một hướng riêng; tầng ra
kết hợp các đường cắt đó lại — và tổ hợp của nhiều đường thẳng, qua một phi
tuyến, có thể XẤP XỈ một đường cong kín như một vòng tròn.

Với bài toán hai vòng tròn cụ thể ở đây: một MLP `2 → 2 → 2` (tầng ẩn `2`
neuron `tanh`, tầng ra `2` neuron không kích hoạt, đọc qua
`cross_entropy_qua_value` — đúng công thức softmax + cross-entropy đã học
ở q8.2c) có đủ neuron ẩn để mỗi neuron học một "lát cắt" thô của mặt
phẳng, và tầng ra tổ hợp hai lát cắt đó thành một ranh giới đóng kín quanh
vòng tròn trong.

Điểm quan trọng: KHÔNG có gì mới được thêm vào framework. Cùng `Value`,
cùng `Layer`/`MLP`, cùng `khoi_tao_mlp`, cùng `adam_step_l2` của bài trước
— chỉ khác kiến trúc (`kich_thuoc=[2, 2, 2]` thay vì một neuron đơn không
tầng ẩn) và số epoch huấn luyện. Nếu kết quả khác biệt rõ rệt, sự khác biệt
đó phải đến từ CHÍNH kiến trúc, không phải từ một công thức bí mật nào
khác.
::::

::::example{#mang_giai_duoc_hai_vong_tron}
Huấn luyện MLP `2 → 2 → 2` — Adam (`lr = 0,1`), `20` epoch — trên ĐÚNG `8`
điểm hai vòng tròn (`seed=424`, `r_trong=1,0`, `r_ngoai=2,5`, nhiễu `±0,25`)
của bài `du-lieu-phi-tuyen-that`:

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

def du_doan_lop(logits):
    return 0 if logits[0].data > logits[1].data else 1

def adam_step_l2(params, t, m_list, v_list, lr, lam=0.0, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps) + lr * lam * p.data
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


X, y = sinh_du_lieu_vong_tron(seed=424, so_diem_moi_lop=4, r_trong=1.0, r_ngoai=2.5, do_nhieu=0.25)

mlp = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params = mlp.params()
m_list = [0.0] * len(params)
v_list = [0.0] * len(params)

for epoch in range(1, 21):
    loss_tong = Value(0.0)
    for (x1v, x2v), yv in zip(X, y):
        logits = mlp.forward([Value(x1v), Value(x2v)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, int(yv))
    loss_tb = loss_tong * (1.0 / len(X))
    loss_tb.backward()
    adam_step_l2(params, epoch, m_list, v_list, lr=0.1)

so_dung = 0
for (x1v, x2v), yv in zip(X, y):
    logits = mlp.forward([Value(x1v), Value(x2v)])
    du_doan = du_doan_lop(logits)
    so_dung += (du_doan == yv)

print("loss cuoi:", round(loss_tb.data, 4))
print("MANG NO-RON  -- so dung:", so_dung, "/", len(y))
print("LOGISTIC REG -- so dung: 4 / 8  (bai du-lieu-phi-tuyen-that)")
```

```text title=readonly
loss cuoi: 0.179
MANG NO-RON  -- so dung: 8 / 8
LOGISTIC REG -- so dung: 4 / 8  (bai du-lieu-phi-tuyen-that)
```

`8/8` — mạng nơ-ron phân loại ĐÚNG TOÀN BỘ hai vòng tròn, loss cuối
`0,179` (gần `0`, so với mức khởi đầu `0,6931`). Trên CHÍNH dữ liệu mà
logistic regression chỉ đạt `4/8`, một tầng ẩn `2` neuron `tanh` — tổng
cộng `12` tham số, nhiều hơn neuron đơn (`3` tham số) — đủ để vẽ một ranh
giới CONG quanh vòng tròn trong. Không đổi dữ liệu, không đổi hàm mất mát
dạng, không đổi bộ tối ưu — chỉ đổi kiến trúc.
::::

::::predict{#doan_neu_bo_tang_an commitOnce}
MLP `2 → 2 → 2` (một tầng ẩn `tanh`) đạt `8/8`. Giả sử đổi kiến trúc
thành `kich_thuoc=[2, 2]` — bỏ hẳn tầng ẩn, chỉ còn MỘT tầng tuyến tính đi
thẳng từ `2` đầu vào ra `2` logit (không `tanh` ở đâu cả).

**Trước khi chạy thử**, bạn đoán: kiến trúc `[2, 2]` này (không tầng ẩn,
không phi tuyến) có khả năng đạt độ chính xác cao trên hai vòng tròn
không?

:::opt{correct}
Không — dù đầu ra giờ là `2` logit thay vì `1` neuron sigmoid, `kich_thuoc=[2,
2]` vẫn là một phép biến đổi TUYẾN TÍNH thuần (không hàm kích hoạt phi
tuyến nào ở giữa) — về bản chất hình học, nó vẫn chỉ vẽ được MỘT đường
thẳng phân cách, giống hệt giới hạn của logistic regression ở bài đầu, chỉ
khác cách viết
:::

:::opt
Có — vì đầu ra bây giờ có `2` logit (được xử lý bằng `softmax` qua
`cross_entropy_qua_value`) thay vì `1` xác suất sigmoid, softmax mạnh hơn
sigmoid nên có thể vẽ được đường cong
::why
Gần đúng ở việc `softmax` và `sigmoid` đúng là hai công thức khác nhau, và
`softmax` xử lý được nhiều lớp cùng lúc — một khác biệt có thật.

Chỗ lệch: khác biệt giữa softmax hai lớp và sigmoid là khác biệt về CÁCH
ĐỌC đầu ra (nhiều logit thay vì một xác suất), không phải khác biệt về khả
năng vẽ ranh giới. Cả hai đều được tính từ một tổ hợp TUYẾN TÍNH của đầu
vào (`z = Wx + b`, không qua `tanh` hay bất kỳ phi tuyến nào ở giữa) —
softmax áp lên một `z` tuyến tính vẫn chỉ tạo ra một ranh giới quyết định
là một đường THẲNG (hay chính xác hơn, một siêu phẳng), không phải đường
cong. Chính TẦNG ẨN PHI TUYẾN — không phải công thức đọc đầu ra — mới là
thứ tạo ra khả năng vẽ đường cong.
::
:::

:::opt
Không xác định được nếu không thử — kiến trúc `[2, 2]` chưa từng chạy qua
nên không có cơ sở để khẳng định trước
::why
Gần đúng ở tinh thần thận trọng khi gặp một kiến trúc CỤ THỂ chưa từng đo.

Chỗ lệch: câu hỏi ở đây không phải về một con số CỤ THỂ chưa biết — nó về
một giới hạn CẤU TRÚC đã được chứng minh từ bài `vi-sao-can-phi-tuyen`
(q8.2a): bất kỳ phép biến đổi tuyến tính nào, dù viết dưới dạng gì (một
neuron sigmoid, hay nhiều neuron output qua softmax), khi không có tầng ẩn
phi tuyến xen giữa, đều chỉ vẽ được ranh giới quyết định là một đường
thẳng. Đây là một khẳng định TOÁN HỌC có thể đưa ra TRƯỚC khi chạy, không
cần đo thử để biết HƯỚNG của kết quả.
::
:::
::::

::::code{#viet_huan_luyen_phi_tuyen}
Hoàn thiện bốn chỗ trống: forward pass tính `logits` trong vòng lặp huấn
luyện, cộng dồn `loss_tong` qua `cross_entropy_qua_value`, gọi
`adam_step_l2` để cập nhật, và tính `du_doan` từ `du_doan_lop`.

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

def du_doan_lop(logits):
    return 0 if logits[0].data > logits[1].data else 1

def adam_step_l2(params, t, m_list, v_list, lr, lam=0.0, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps) + lr * lam * p.data
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


X, y = sinh_du_lieu_vong_tron(seed=424, so_diem_moi_lop=4, r_trong=1.0, r_ngoai=2.5, do_nhieu=0.25)

mlp = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params = mlp.params()
m_list = [0.0] * len(params)
v_list = [0.0] * len(params)

for epoch in range(1, 21):
    loss_tong = Value(0.0)
    for (x1v, x2v), yv in zip(X, y):
        logits = ___                    # mlp.forward([Value(x1v), Value(x2v)])
        loss_tong = ___                 # loss_tong + cross_entropy_qua_value(logits, int(yv))
    loss_tb = loss_tong * (1.0 / len(X))
    loss_tb.backward()
    ___                                  # adam_step_l2(params, epoch, m_list, v_list, lr=0.1)

so_dung = 0
for (x1v, x2v), yv in zip(X, y):
    logits = mlp.forward([Value(x1v), Value(x2v)])
    du_doan = ___                        # du_doan_lop(logits)
    so_dung += (du_doan == yv)

print(round(loss_tb.data, 4))
print(so_dung, "/", len(y))
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

def du_doan_lop(logits):
    return 0 if logits[0].data > logits[1].data else 1

def adam_step_l2(params, t, m_list, v_list, lr, lam=0.0, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps) + lr * lam * p.data
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


X, y = sinh_du_lieu_vong_tron(seed=424, so_diem_moi_lop=4, r_trong=1.0, r_ngoai=2.5, do_nhieu=0.25)

mlp = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params = mlp.params()
m_list = [0.0] * len(params)
v_list = [0.0] * len(params)

for epoch in range(1, 21):
    loss_tong = Value(0.0)
    for (x1v, x2v), yv in zip(X, y):
        logits = mlp.forward([Value(x1v), Value(x2v)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, int(yv))
    loss_tb = loss_tong * (1.0 / len(X))
    loss_tb.backward()
    adam_step_l2(params, epoch, m_list, v_list, lr=0.1)

so_dung = 0
for (x1v, x2v), yv in zip(X, y):
    logits = mlp.forward([Value(x1v), Value(x2v)])
    du_doan = du_doan_lop(logits)
    so_dung += (du_doan == yv)

print(round(loss_tb.data, 4))
print(so_dung, "/", len(y))
```

```python title=test
assert round(loss_tb.data, 4) == 0.179, f"loss cuoi sai -- dang ra {round(loss_tb.data, 4)}"
assert so_dung == 8, f"phai phan loai DUNG CA 8 diem -- dang ra so_dung={so_dung}"
assert len(y) == 8, f"tong so diem phai la 8 -- dang ra {len(y)}"

# rieng kiem tra khong dut do thi: MOI tham so phai co grad khac 0 ngay
# sau backward() cuoi cung cua vong lap huan luyen (dau hieu khong dut o
# bat ky diem nao trong Layer/MLP vua duoc goi lai).
loss_tb.backward()
so_khac_0 = sum(1 for p in params if abs(p.grad) > 1e-9)
assert so_khac_0 == len(params), f"MOI tham so phai co grad khac 0 sau backward() -- co {len(params) - so_khac_0}/{len(params)} dang grad=0"

# rieng kiem tra BIEN cua du_doan_lop tai logit bang nhau (da tu kiem
# chung: khong logit nao trong qua trinh huan luyen that dung bang nhau
# tuyet doi, nen chi mot loi goi truc tiep moi ep di qua dung nhanh bien).
logit_hoa = [Value(1.5), Value(1.5)]
assert du_doan_lop(logit_hoa) == 1, f"khi hai logit bang nhau, du_doan_lop phai tra ve 1 (dung '>', nghieng ve lop sau) -- dang ra {du_doan_lop(logit_hoa)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `logits`: forward một điểm `(x1v, x2v)`, giữ nguyên danh sách hai logit (không lấy `[0]`) — `mlp.forward([Value(x1v), Value(x2v)])`. `loss_tong`: cộng dồn qua `cross_entropy_qua_value` — `loss_tong + cross_entropy_qua_value(logits, int(yv))`. Chỗ gọi Adam: đúng năm tham số vị trí cộng `lr` — `adam_step_l2(params, epoch, m_list, v_list, lr=0.1)`. `du_doan`: gọi hàm ĐÃ CÓ, không viết lại so sánh tay — `du_doan_lop(logits)`.
- kind: strategy
  body: 'logits: `mlp.forward([Value(x1v), Value(x2v)])`. loss_tong: `loss_tong + cross_entropy_qua_value(logits, int(yv))`. Adam: `adam_step_l2(params, epoch, m_list, v_list, lr=0.1)`. du_doan: `du_doan_lop(logits)`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `mlp.forward([Value(x1v), Value(x2v)])`, `loss_tong + cross_entropy_qua_value(logits, int(yv))`, `adam_step_l2(params, epoch, m_list, v_list, lr=0.1)`, và `du_doan_lop(logits)`.'
:::

:::validate
- tier: run
  timeoutMs: 12000
- tier: static
  onFail: logits phai goi THAT mlp.forward tren tung diem trong vong lap huan luyen; loss_tong phai CONG DON cross_entropy_qua_value; phai GOI THAT adam_step_l2 moi epoch; du_doan phai goi ham du_doan_lop co san (khong tu viet lai so sanh)
  requireAst:
  - kind: uses-call, target: forward, min: 3
  - kind: uses-call, target: cross_entropy_qua_value, min: 1
  - kind: uses-call, target: du_doan_lop, min: 1
  - kind: uses-call, target: adam_step_l2, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (forward=3: goi noi bo trong
  # MLP.forward "tang.forward(a)" [1], goi that trong vong lap huan luyen
  # cho logits [1], goi lai trong vong lap tinh so_dung [1] -- dinh nghia
  # Layer.forward/MLP.forward KHONG tinh vi la FunctionDef; cross_entropy_
  # qua_value=1: dinh nghia [khong tinh], dung mot lan trong loss_tong;
  # du_doan_lop=1: dinh nghia [khong tinh], dung mot lan; adam_step_l2=1:
  # dinh nghia [khong tinh], dung mot lan).
  #
  # Cheat "logits dummy khong goi forward trong vong lap huan luyen" lam
  # forward tut xuong 2 -- bi chan; da tu kiem chung: cheat nay cung doi
  # loss cuoi tu 0.179 thanh 0.6931 va so_dung tu 8 xuong 3 -- phong
  # thu kep. Cheat "loss_tong = loss_tong + Value(0.0)" (bo qua tin hieu
  # hoc) lam cross_entropy_qua_value ve 0 -- bi chan, doi loss cuoi thanh
  # dung 0.0 va so_dung xuong 3 (mang khong con hoc duoc gi ca, tham so
  # dung yen). Cheat "du_doan = 1" (chep san) lam du_doan_lop ve 0 -- bi
  # chan, doi so_dung xuong 4 (chi con lop 1 dung). Cheat "khong goi
  # adam_step_l2" lam tham so dung yen -- bi chan CA static
  # (adam_step_l2=0) LAN output (da tu kiem chung: loss cuoi thanh 0.7328,
  # so_dung xuong 3).
- tier: tests
  timeoutMs: 12000
- tier: output
  match: regex
  expect: "^0\\.179\\n8 / 8\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`8/8` — hoàn hảo. Cùng dữ liệu khiến logistic regression chỉ đạt `4/8`,
một tầng ẩn `tanh` giải quyết trọn vẹn. Đây là bằng chứng số học cho lý do
cả track T8.2 tồn tại.
::::

::::reflect{#nghi-lai}
Mạng nơ-ron vừa giải được điều logistic regression không giải được — bằng
số thật, trên cùng một tập dữ liệu. Nhưng `8/8` chỉ trả lời câu hỏi
"giải được hay không", chưa trả lời câu hỏi "giải NHANH tới đâu".

Ba bộ tối ưu đã học ở q8.2c — SGD thường, động lượng, Adam — đều CÓ THỂ
đạt cùng đích trên bài toán này. Chúng có tốn cùng số epoch không?
::::

::::checkpoint{mastery=0.85}
::::
