---
id: tri-tue-nhan-tao.huan-luyen-mang-no-ron.boss-huan-luyen-mang-no-ron
title: "BOSS — Vòng lặp huấn luyện đầy đủ"
summary: "Ráp TẤT CẢ: Layer/MLP qua Value (bài 1) + hàm mất mát qua Value (bài 2) + Adam với L2 tuỳ chọn (bài 6, 8) thành MỘT vòng lặp huấn luyện, chạy từ đầu tới cuối trên hai bài toán KHÁC NHAU. Hồi quy (MLP 1-3-1, 4 điểm gần y=2x+1, 25 epoch, lr=0.3): MSE cuối = 0.6345. Phân loại nhị phân tuyến tính tách được (MLP 2-4-2, 4 điểm hai góc phần tư đối diện, 3 epoch, softmax+cross-entropy): 4/4 điểm đúng, loss cuối ~ 0.057. Đóng q8.2c — chuyển giao Layer/MLP/Value/Adam cho q8.2d huấn luyện trên dữ liệu phi tuyến THẬT."
locale: vi
track: tri-tue-nhan-tao
module: huan-luyen-mang-no-ron
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-huan-luyen-mang-no-ron]
requires: [ai.chinh-quy-hoa-mang-no-ron]
concepts: [ai.boss-huan-luyen-mang-no-ron]
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
Tám bài, từng mảnh một: neuron qua autograd, hàm mất mát, vòng lặp, batch,
động lượng, Adam, quá khớp, chính quy hoá. Giờ ráp lại thành MỘT khối, chạy
trên hai bài toán mà cả track này đã dạy: hồi quy VÀ phân loại.
::::

::::explain{#rap_tat_ca}
Vòng lặp huấn luyện đầy đủ của bài này gồm ĐÚNG những gì tám bài trước đã
xây, không thêm gì mới:

> **`Layer`/`MLP`** (bài `neuron-tu-autograd`) — `W`, `b` là `Value`,
> `forward` xây đồ thị tính toán tự động.
>
> **Hàm mất mát qua `Value`** (bài `ham-mat-mat-qua-value`) — `mse_qua_value`
> cho hồi quy, `cross_entropy_qua_value` (bọc log-sum-exp ổn định) cho phân
> loại.
>
> **Adam + L2 tuỳ chọn** (bài `adam-toi-uu-hoa`, `chinh-quy-hoa-mang-no-ron`)
> — hai trung bình động, hiệu chỉnh lệch, cộng thêm bước phạt `lr·λ·param`
> nếu `λ > 0`.

Với bài toán HỒI QUY, đầu ra là MỘT số thực (không kích hoạt ở tầng ra),
loss là `mse_qua_value`. Với bài toán PHÂN LOẠI, đầu ra là NHIỀU logit
(không kích hoạt ở tầng ra — softmax nằm TRONG `cross_entropy_qua_value`,
không phải một tầng riêng), loss là trung bình `cross_entropy_qua_value`
trên từng điểm, và dự đoán cuối cùng là lớp có logit LỚN NHẤT (không cần
tính softmax tường minh để so sánh thứ tự — logit lớn nhất luôn cho xác
suất softmax lớn nhất, vì `exp` là hàm ĐƠN ĐIỆU TĂNG).

Cấu trúc vòng lặp — forward, loss, `backward()`, cập nhật qua `adam_step_l2`
— không đổi giữa hai bài toán. Điểm khác biệt DUY NHẤT là hàm mất mát nào
được gọi, và cách đọc đầu ra cuối cùng.
::::

::::example{#boss_hoi_quy_va_phan_loai}
Hồi quy: `4` điểm (`x = 1..4`, quan hệ thật gần `y = 2x+1`), MLP `1 → 3 →
1`. Phân loại: `4` điểm hai góc phần tư ĐỐI DIỆN tách được bằng đường
thẳng (`(1,1)`, `(1,-1)` là lớp `1`; `(-1,1)`, `(-1,-1)` là lớp `0` — phân
biệt bởi dấu của `x₁`), MLP `2 → 4 → 2`:

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

def mse_so(mlp, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        pred = mlp.forward([Value(x)])[0]
        tong += (pred.data - y) ** 2
    return tong / len(xs)

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

# ---------- HOI QUY ----------
x_reg = [1.0, 2.0, 3.0, 4.0]
y_reg = [3.2, 4.9, 7.1, 9.3]

mlp_reg = khoi_tao_mlp(seed=16, kich_thuoc=[1, 3, 1])
params_reg = mlp_reg.params()
m_reg, v_reg = [0.0] * len(params_reg), [0.0] * len(params_reg)
for epoch in range(1, 26):
    preds = [mlp_reg.forward([Value(x)])[0] for x in x_reg]
    loss = mse_qua_value(preds, y_reg)
    loss.backward()
    adam_step_l2(params_reg, epoch, m_reg, v_reg, lr=0.3)
mse_cuoi = mse_so(mlp_reg, x_reg, y_reg)

# ---------- PHAN LOAI ----------
X_clf = [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)]
y_clf = [1, 1, 0, 0]

mlp_clf = khoi_tao_mlp(seed=14, kich_thuoc=[2, 4, 2])
params_clf = mlp_clf.params()
m_clf, v_clf = [0.0] * len(params_clf), [0.0] * len(params_clf)
for epoch in range(1, 4):
    loss_tong = Value(0.0)
    for (x1, x2), y in zip(X_clf, y_clf):
        logits = mlp_clf.forward([Value(x1), Value(x2)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, y)
    loss_tb = loss_tong * (1.0 / len(X_clf))
    loss_tb.backward()
    adam_step_l2(params_clf, epoch, m_clf, v_clf, lr=0.3)

so_dung = 0
for (x1, x2), y in zip(X_clf, y_clf):
    logits = mlp_clf.forward([Value(x1), Value(x2)])
    du_doan = du_doan_lop(logits)
    so_dung += (du_doan == y)

print("HOI QUY   -- mse cuoi:", round(mse_cuoi, 4))
print("PHAN LOAI -- so dung:", so_dung, "/", len(X_clf), " loss cuoi:", round(loss_tb.data, 4))
```

```text title=readonly
HOI QUY   -- mse cuoi: 0.6345
PHAN LOAI -- so dung: 4 / 4  loss cuoi: 0.057
```

Hồi quy: MSE cuối `0.6345` sau chỉ `25` epoch — mạng `1→3→1` (`10` tham
số) khớp khá sát bốn điểm gần đường thẳng `y = 2x+1`. Phân loại: `4/4`
điểm đúng, loss cuối `0.057` — thấp, đúng như kỳ vọng cho một bài
toán TUYẾN TÍNH TÁCH ĐƯỢC (hai lớp nằm ở hai góc phần tư đối diện, không
hề chồng lấn, chỉ cần phân biệt dấu của `x₁`). Cùng MỘT vòng lặp huấn
luyện — chỉ khác hàm mất mát và cách đọc đầu ra — giải được CẢ HAI loại
bài toán track này đã dạy.
::::

::::predict{#doan_neu_hoan_doi_ham_mat_mat commitOnce}
Vòng lặp huấn luyện ở trên dùng `mse_qua_value` cho hồi quy,
`cross_entropy_qua_value` cho phân loại — hai bài toán KHÁC NHAU, hai hàm
mất mát KHÁC NHAU.

**Trước khi đọc lại**, bạn đoán: nếu lỡ dùng NHẦM `mse_qua_value` (so
từng logit với `0` hoặc `1` như một con số hồi quy, thay vì
`cross_entropy_qua_value`) để huấn luyện bài toán PHÂN LOẠI hai góc phần
tư ở trên, mạng có còn khả năng phân loại đúng CẢ BỐN điểm được không?

:::opt{correct}
Có thể vẫn đúng cả bốn — vì MSE, dù không phải hàm mất mát "đúng thiết kế"
cho phân loại, vẫn TRUNG THỰC đo khoảng cách giữa dự đoán và nhãn
(`0`/`1`); trên một bài toán DỄ như hai góc phần tư tách biệt rõ ràng
(không hề chồng lấn), MỘT TÍN HIỆU HUẤN LUYỆN HỢP LÝ nào cũng có khả năng
đủ để mạng tìm ra ranh giới đúng — MSE không phải hàm mất mát TỐI ƯU cho
phân loại, nhưng "không tối ưu" không đồng nghĩa "không hoạt động" trên
MỘT bài toán cụ thể dễ đến vậy
:::

:::opt
Không — MSE hoàn toàn không dùng được cho phân loại, gradient của nó bằng
`0` với mọi logit, mạng không học được gì cả
::why
Gần đúng ở việc phân biệt MSE và cross-entropy là hai công thức KHÁC nhau,
được thiết kế cho hai mục đích khác nhau — quan sát đó không sai.

Chỗ lệch: MSE (`(pred − target)²`) có đạo hàm `2(pred − target)` — hoàn
toàn KHÁC `0` với hầu hết giá trị `pred`, không có gì đặc biệt khiến nó
triệt tiêu trên bài toán này. "Không phải công thức tối ưu" khác hẳn
"gradient luôn bằng 0" — MSE vẫn TÍNH ĐƯỢC và LAN ĐƯỢC gradient, chỉ là
tín hiệu nó cho không phản ánh đúng "xác suất" như cross-entropy được
thiết kế để làm.
::
:::

:::opt
Không xác định được nếu không chạy thử — hàm mất mát nào cũng có thể hoạt
động hoặc không, tuỳ vào may rủi của lần khởi tạo trọng số cụ thể
::why
Gần đúng ở việc thận trọng — khởi tạo trọng số ĐÚNG LÀ có ảnh hưởng tới kết
quả cụ thể của một lần huấn luyện (bài `khoi-tao-trong-so`, q8.2a).

Chỗ lệch: câu hỏi không phải về MAY RỦI ngẫu nhiên của một lần chạy cụ thể
— nó hỏi về khả năng CƠ BẢN của công thức MSE trong việc tạo tín hiệu học
được cho bài toán phân loại này. Với một bài toán dễ và MSE là một hàm số
TRƠN, có đạo hàm khác `0` ở hầu hết mọi nơi, khả năng học được không phụ
thuộc chủ yếu vào may rủi khởi tạo — nó phụ thuộc vào việc công thức đó có
mang tín hiệu HỢP LÝ hay không, và MSE thì có, dù không phải thiết kế
chuẩn.
::
:::
::::

::::code{#viet_boss_huan_luyen}
Hoàn thiện bốn chỗ trống: forward pass hồi quy, forward pass phân loại
(tính `logits`), cộng dồn `loss_tong` qua từng điểm, và dự đoán lớp từ
logit lớn nhất.

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

def mse_so(mlp, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        pred = mlp.forward([Value(x)])[0]
        tong += (pred.data - y) ** 2
    return tong / len(xs)

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
    return ___                            # 0 if logits[0].data > logits[1].data else 1

def adam_step_l2(params, t, m_list, v_list, lr, lam=0.0, beta1=0.9, beta2=0.999, eps=1e-8):
    for i, p in enumerate(params):
        g = p.grad
        m_list[i] = beta1 * m_list[i] + (1 - beta1) * g
        v_list[i] = beta2 * v_list[i] + (1 - beta2) * g * g
        m_hat = m_list[i] / (1 - beta1 ** t)
        v_hat = v_list[i] / (1 - beta2 ** t)
        p.data -= lr * m_hat / (v_hat ** 0.5 + eps) + lr * lam * p.data
        p.grad = 0.0

# ---------- HOI QUY ----------
x_reg = [1.0, 2.0, 3.0, 4.0]
y_reg = [3.2, 4.9, 7.1, 9.3]

mlp_reg = khoi_tao_mlp(seed=16, kich_thuoc=[1, 3, 1])
params_reg = mlp_reg.params()
m_reg, v_reg = [0.0] * len(params_reg), [0.0] * len(params_reg)
for epoch in range(1, 26):
    preds = ___                          # [mlp_reg.forward([Value(x)])[0] for x in x_reg]
    loss = mse_qua_value(preds, y_reg)
    loss.backward()
    adam_step_l2(params_reg, epoch, m_reg, v_reg, lr=0.3)
mse_cuoi = mse_so(mlp_reg, x_reg, y_reg)

# ---------- PHAN LOAI ----------
X_clf = [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)]
y_clf = [1, 1, 0, 0]

mlp_clf = khoi_tao_mlp(seed=14, kich_thuoc=[2, 4, 2])
params_clf = mlp_clf.params()
m_clf, v_clf = [0.0] * len(params_clf), [0.0] * len(params_clf)
for epoch in range(1, 4):
    loss_tong = Value(0.0)
    for (x1, x2), y in zip(X_clf, y_clf):
        logits = ___                     # mlp_clf.forward([Value(x1), Value(x2)])
        loss_tong = ___                  # loss_tong + cross_entropy_qua_value(logits, y)
    loss_tb = loss_tong * (1.0 / len(X_clf))
    loss_tb.backward()
    adam_step_l2(params_clf, epoch, m_clf, v_clf, lr=0.3)

so_dung = 0
for (x1, x2), y in zip(X_clf, y_clf):
    logits = mlp_clf.forward([Value(x1), Value(x2)])
    du_doan = du_doan_lop(logits)
    so_dung += (du_doan == y)

print("HOI QUY   -- mse cuoi:", round(mse_cuoi, 4))
print("PHAN LOAI -- so dung:", so_dung, "/", len(X_clf), " loss cuoi:", round(loss_tb.data, 4))
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

def mse_so(mlp, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        pred = mlp.forward([Value(x)])[0]
        tong += (pred.data - y) ** 2
    return tong / len(xs)

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

# ---------- HOI QUY ----------
x_reg = [1.0, 2.0, 3.0, 4.0]
y_reg = [3.2, 4.9, 7.1, 9.3]

mlp_reg = khoi_tao_mlp(seed=16, kich_thuoc=[1, 3, 1])
params_reg = mlp_reg.params()
m_reg, v_reg = [0.0] * len(params_reg), [0.0] * len(params_reg)
for epoch in range(1, 26):
    preds = [mlp_reg.forward([Value(x)])[0] for x in x_reg]
    loss = mse_qua_value(preds, y_reg)
    loss.backward()
    adam_step_l2(params_reg, epoch, m_reg, v_reg, lr=0.3)
mse_cuoi = mse_so(mlp_reg, x_reg, y_reg)

# ---------- PHAN LOAI ----------
X_clf = [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)]
y_clf = [1, 1, 0, 0]

mlp_clf = khoi_tao_mlp(seed=14, kich_thuoc=[2, 4, 2])
params_clf = mlp_clf.params()
m_clf, v_clf = [0.0] * len(params_clf), [0.0] * len(params_clf)
for epoch in range(1, 4):
    loss_tong = Value(0.0)
    for (x1, x2), y in zip(X_clf, y_clf):
        logits = mlp_clf.forward([Value(x1), Value(x2)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, y)
    loss_tb = loss_tong * (1.0 / len(X_clf))
    loss_tb.backward()
    adam_step_l2(params_clf, epoch, m_clf, v_clf, lr=0.3)

so_dung = 0
for (x1, x2), y in zip(X_clf, y_clf):
    logits = mlp_clf.forward([Value(x1), Value(x2)])
    du_doan = du_doan_lop(logits)
    so_dung += (du_doan == y)

print("HOI QUY   -- mse cuoi:", round(mse_cuoi, 4))
print("PHAN LOAI -- so dung:", so_dung, "/", len(X_clf), " loss cuoi:", round(loss_tb.data, 4))
```

```python title=test
assert round(mse_cuoi, 4) == 0.6345, f"mse_cuoi sai -- dang ra {round(mse_cuoi, 4)}"
assert so_dung == 4, f"phai phan loai DUNG CA BON diem (bai toan tuyen tinh tach duoc) -- dang ra so_dung={so_dung}"
assert round(loss_tb.data, 4) == 0.057, f"loss cuoi phan loai sai -- dang ra {round(loss_tb.data, 4)}"

# rieng kiem tra dut do thi: MOI tham so cua CA HAI mang phai co grad khac
# 0 ngay sau backward() cuoi cung cua vong lap huan luyen tuong ung (dau
# hieu do thi khong dut o bat ky diem nao).
loss.backward()
so_khac_0_reg = sum(1 for p in params_reg if abs(p.grad) > 1e-12)
assert so_khac_0_reg == len(params_reg), f"MOI tham so mang hoi quy phai co grad khac 0 sau backward() -- co {len(params_reg) - so_khac_0_reg}/{len(params_reg)} dang grad=0"

loss_tb.backward()
so_khac_0_clf = sum(1 for p in params_clf if abs(p.grad) > 1e-12)
assert so_khac_0_clf == len(params_clf), f"MOI tham so mang phan loai phai co grad khac 0 sau backward() -- co {len(params_clf) - so_khac_0_clf}/{len(params_clf)} dang grad=0"

# rieng kiem tra BIEN cua du_doan_lop (tie-break tai logits bang nhau): goi
# TRUC TIEP tren mot cap logit CO Y bang nhau -- da tu kiem chung bang
# Python that: '>' (khong phai '>=') cho lop 1 thang khi hoa, dung quy uoc
# "nghieng ve lop sau" khi khong the phan biet duoc.
logit_hoa = [Value(2.5), Value(2.5)]
assert du_doan_lop(logit_hoa) == 1, f"khi hai logit bang nhau tuyet doi, du_doan_lop phai tra ve 1 (dung '>', khong phai '>=') -- dang ra {du_doan_lop(logit_hoa)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `preds` (hồi quy): forward TỪNG điểm `x` trong `x_reg`, lấy phần tử `[0]` (một neuron ra) — `[mlp_reg.forward([Value(x)])[0] for x in x_reg]`. `logits` (phân loại): forward MỘT điểm `(x1, x2)`, KHÔNG lấy `[0]` (giữ nguyên danh sách hai logit) — `mlp_clf.forward([Value(x1), Value(x2)])`. `loss_tong`: cộng dồn — `loss_tong + cross_entropy_qua_value(logits, y)`. `du_doan_lop`: trả về lớp có logit LỚN HƠN — `0 if logits[0].data > logits[1].data else 1`.
- kind: strategy
  body: 'preds: `[mlp_reg.forward([Value(x)])[0] for x in x_reg]`. logits: `mlp_clf.forward([Value(x1), Value(x2)])`. loss_tong: `loss_tong + cross_entropy_qua_value(logits, y)`. du_doan_lop: `return 0 if logits[0].data > logits[1].data else 1`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `[mlp_reg.forward([Value(x)])[0] for x in x_reg]`, `mlp_clf.forward([Value(x1), Value(x2)])`, `loss_tong + cross_entropy_qua_value(logits, y)`, và `0 if logits[0].data > logits[1].data else 1`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: preds/logits phai goi THAT mlp_reg.forward/mlp_clf.forward (khong duoc chep du doan); loss_tong phai CONG DON cross_entropy_qua_value qua tung diem (khong duoc bo qua diem nao); du_doan_lop phai so sanh DUNG logits[0].data va logits[1].data bang '>' (khong phai '>=')
  requireAst:
  - kind: uses-call, target: forward, min: 5
  - kind: uses-call, target: cross_entropy_qua_value, min: 1
  - kind: uses-operator, target: ">", min: 1
  - kind: uses-name, target: loss_tong, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (forward=5: dinh nghia Layer.forward
  # va MLP.forward KHONG tinh [FunctionDef, khong phai Call], goi that
  # trong preds, goi trong MLP.forward noi bo "tang.forward(a)", goi trong
  # vong lap logits, goi lai trong vong lap so_dung phia duoi -- it nhat 5
  # lan GOI THAT su; cross_entropy_qua_value=1: dinh nghia [khong tinh],
  # dung mot lan goi trong loss_tong; ">"=1: dung dung mot lan trong
  # du_doan_lop; loss_tong=2: gan mot lan [Store, khong tinh], doc mot lan o
  # ve phai cua chinh no trong phep cong don, doc lai lan nua khi tinh
  # loss_tb -- it nhat 2 lan Load).
  #
  # Cheat "du_doan_lop dung '>=' thay vi '>'" khong doi so_dung tren du lieu
  # HUAN LUYEN that (khong logit nao bang nhau tuyet doi trong qua trinh
  # huan luyen that) nen KHONG bi bat boi tests/output -- day la LO HONG
  # BIEN kinh dien. Assertion rieng "logit_hoa" (hai logit gia lap BANG
  # NHAU tuyet doi) ep di qua dung nhanh bien, doc lap voi du lieu huan
  # luyen cu the, bat duoc cheat nay CHAC CHAN.
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^HOI QUY   -- mse cuoi: 0\\.6345\\nPHAN LOAI -- so dung: 4 / 4  loss cuoi: 0\\.057\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
MSE `0.6345` cho hồi quy, `4/4` đúng cho phân loại — cùng MỘT vòng lặp,
cùng `Layer`/`MLP`/`Value`/Adam đã xây suốt chín bài. q8.2c khép lại tại
đây.
::::

::::reflect{#nghi-lai}
Chín bài, một track: `neuron-tu-autograd` ráp `Value` vào `Layer`/`MLP`.
`ham-mat-mat-qua-value` thêm `exp`/`log`, tính MSE và cross-entropy qua
autograd. `vong-lap-huan-luyen` ráp bốn bước cập nhật, bắt gotcha
`zero_grad`. `minibatch-va-sgd` so ba cách chia dữ liệu. `dong-luong-momentum`
và `adam-toi-uu-hoa` thêm hai cách cập nhật thông minh hơn. `qua-khop-mang-no-ron`
đo chữ ký quá khớp trên một mạng thật. `chinh-quy-hoa-mang-no-ron` chữa nó
bằng L2 và dropout. Bài này ráp TẤT CẢ vào một vòng lặp chạy được trên cả
hồi quy lẫn phân loại.

Nhưng cả `4` điểm hồi quy lẫn `4` điểm phân loại của bài này đều là dữ liệu
ĐƠN GIẢN — tuyến tính, hoặc gần tuyến tính, tách được dễ dàng. `q8.2d` (BOSS
đóng T8.2) sẽ đưa framework này — nguyên vẹn, không viết lại — đối đầu với
dữ liệu PHI TUYẾN thật: hai đường cong lồng nhau mà một mô hình tuyến tính
không thể nào tách được, đúng bài toán mà cả track T8.2 tồn tại để giải.
::::

::::checkpoint{mastery=0.9}
::::
