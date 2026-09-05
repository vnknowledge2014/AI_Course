---
id: tri-tue-nhan-tao.boss-mang-no-ron-tu-so-0.boss-mang-no-ron-tu-so-0
title: "BOSS — Mạng nơ-ron từ số 0, đóng T8.2 tại 32/32"
summary: "Ráp CUỐI CÙNG: Value (8 phép toán) + Layer/MLP + Adam + L2 tuỳ chọn, huấn luyện đầu-cuối trên hai hình trăng lồng nhau (8 điểm, công thức tường minh, KHÁC hai vòng tròn của bài 1 — đa dạng dữ liệu phi tuyến). MLP 2→2→2, 15 epoch, lr=0,15, lam=0,001: loss cuối 0,0108, accuracy 8/8 (100%). Đóng T8.2 'Mạng nơ-ron từ số 0' tại 32/32 khái niệm — bắc cầu sang T8.3 'Transformer từ số 0'."
locale: vi
track: tri-tue-nhan-tao
module: boss-mang-no-ron-tu-so-0
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-mang-no-ron-tu-so-0]
requires: [ai.gradient-bien-mat-that]
concepts: [ai.boss-mang-no-ron-tu-so-0]
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
Ba mươi mốt bài, bốn quest, một track. Bài cuối cùng này ráp TẤT CẢ lại —
autograd, framework, tối ưu hoá, và một mảnh dữ liệu phi tuyến MỚI, chưa
từng thấy — để đóng T8.2 tại `32/32`.
::::

::::explain{#rap_tat_ca_dong_track}
Vòng lặp huấn luyện của bài này gồm ĐÚNG những gì T8.2 đã xây, không thêm
gì mới:

> **`Value`** — tám phép toán (`+`, `-`, `*`, `**`, `tanh`, `relu`, `exp`,
> `log`), mỗi phép tự biết đạo hàm cục bộ của mình (q8.2b).
>
> **`Layer`/`MLP`** — `W`, `b` là `Value`, forward xây đồ thị tính toán tự
> động; `khoi_tao_mlp` khởi tạo ngẫu nhiên nhỏ, tầng ẩn `tanh`, tầng ra
> không kích hoạt (q8.2a ráp qua `Value` ở q8.2c).
>
> **`cross_entropy_qua_value`** — softmax + cross-entropy ổn định số học,
> bọc log-sum-exp (q8.2c).
>
> **`adam_step_l2`** — Adam với phạt L2 tuỳ chọn (`lam`, mặc định `0`) —
> cả hai kỹ thuật chống quá khớp đã học ở `chinh-quy-hoa-mang-no-ron`
> (q8.2c) đều có thể bật lên chỉ bằng một tham số.

Dữ liệu lần này KHÁC hai vòng tròn của bài `du-lieu-phi-tuyen-that` — một
biến thể khác của "không tuyến tính tách được": **hai hình trăng lồng vào
nhau**. Mỗi hình trăng là một nửa đường tròn (`góc` chạy trên `[0, π]`,
không phải trọn `[0, 2π)` như vòng tròn), một hình trăng bị LẬT NGƯỢC và
DỊCH sang để mắc cài vào khoảng lõm của hình trăng kia. Công thức tường
minh, dùng lại đúng `rng.uniform`/`np.cos`/`np.sin` như bài `du-lieu-
phi-tuyen-that`, không `sklearn`:

> Hình trăng `0`: `x = cos(góc) + nhiễu`, `y = sin(góc) + nhiễu`, `góc ∈
> [0, π]`.
>
> Hình trăng `1`: `x = 1 − cos(góc) + nhiễu`, `y = −sin(góc) + 0,5 +
> nhiễu`, `góc ∈ [0, π]` — số hạng `1 − cos` lật và dịch NGANG, số hạng
> `−sin + 0,5` lật và dịch DỌC, mắc hai hình trăng vào nhau như hai bàn
> tay đan lồng.

Framework không đổi. Dữ liệu đổi để chứng minh mạng nơ-ron giải được
KHÔNG chỉ một hình dạng phi tuyến cụ thể (vòng tròn), mà bất kỳ hình dạng
nào không tuyến tính tách được.
::::

::::example{#boss_hai_hinh_trang}
Sinh `8` điểm hai hình trăng (`4` mỗi lớp, nhiễu `±0,12`), huấn luyện MLP
`2 → 2 → 2` — Adam VỚI phạt L2 nhẹ (`lam = 0,001`) — `15` epoch:

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

def sinh_du_lieu_hai_trang(seed, so_diem_moi_lop, do_nhieu):
    rng = np.random.default_rng(seed)
    theta0 = rng.uniform(0.0, np.pi, size=so_diem_moi_lop)
    x0 = np.cos(theta0) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    y0 = np.sin(theta0) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    theta1 = rng.uniform(0.0, np.pi, size=so_diem_moi_lop)
    x1 = 1.0 - np.cos(theta1) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    y1 = -np.sin(theta1) + 0.5 + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    X = np.concatenate([np.stack([x0, y0], axis=1), np.stack([x1, y1], axis=1)])
    y = np.array([0] * so_diem_moi_lop + [1] * so_diem_moi_lop)
    return X, y


X, y = sinh_du_lieu_hai_trang(seed=3, so_diem_moi_lop=4, do_nhieu=0.12)

mlp = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params = mlp.params()
m_list = [0.0] * len(params)
v_list = [0.0] * len(params)

for epoch in range(1, 16):
    loss_tong = Value(0.0)
    for (x1v, x2v), yv in zip(X, y):
        logits = mlp.forward([Value(x1v), Value(x2v)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, int(yv))
    loss_tb = loss_tong * (1.0 / len(X))
    loss_tb.backward()
    adam_step_l2(params, epoch, m_list, v_list, lr=0.15, lam=0.001)

so_dung = 0
for (x1v, x2v), yv in zip(X, y):
    logits = mlp.forward([Value(x1v), Value(x2v)])
    du_doan = du_doan_lop(logits)
    so_dung += (du_doan == yv)

do_chinh_xac = so_dung / len(y)

print("loss cuoi:", round(loss_tb.data, 4))
print("so dung:", so_dung, "/", len(y))
print("do chinh xac:", round(do_chinh_xac, 4))
```

```text title=readonly
loss cuoi: 0.0108
so dung: 8 / 8
do chinh xac: 1.0
```

`8/8` — mạng phân loại ĐÚNG TOÀN BỘ hai hình trăng lồng nhau, một hình
dạng phi tuyến HOÀN TOÀN KHÁC hai vòng tròn của bài đầu track này. Cùng
framework — không đổi một dòng — giải quyết được một bài toán phi tuyến
MỚI, chưa từng thấy trong suốt track. Đây không phải một kết quả trùng
hợp của đúng một hình dạng cụ thể: nó là bằng chứng cho việc framework này
tổng quát, không "học thuộc" hình vòng tròn.
::::

::::predict{#doan_tong_ket_track commitOnce}
Suốt track T8.2, mọi con số đều được đo bằng Python thật — không suy luận
tay. Bài `du-lieu-phi-tuyen-that` (bài `1`) đo logistic regression thất
bại (`4/8`); bài này đo mạng nơ-ron thành công (`8/8`) trên một dữ
liệu phi tuyến KHÁC.

**Trước khi đọc tiếp**, bạn đoán: điều gì làm cho một mô hình "giải được"
một bài toán phân loại phi tuyến — độ chính xác cao trên dữ liệu ĐÃ THẤY
(train), hay khả năng đạt độ chính xác cao trên dữ liệu CHƯA TỪNG THẤY?

:::opt{correct}
Khả năng đạt độ chính xác cao trên dữ liệu CHƯA TỪNG THẤY — đúng bài học
xuyên suốt `qua-khop-mang-no-ron` (q8.2c): một mạng NHỚ THUỘC train data
(khớp `100%` trên chính những điểm nó đã thấy) không đảm bảo gì về khả
năng TỔNG QUÁT HOÁ; `8/8` ở bài này chỉ là bằng chứng mạng CÓ KHẢ NĂNG
biểu diễn ranh giới cong đúng, phải kết hợp với `val`/`test` riêng (đã học
từ T8.1a) để khẳng định nó tổng quát hoá tốt, không chỉ học thuộc
:::

:::opt
Độ chính xác cao trên dữ liệu train — nếu một mô hình khớp được toàn bộ
dữ liệu nó thấy, nó đã "giải được" bài toán, không cần đo gì thêm
::why
Gần đúng ở việc độ chính xác train cao LÀ một điều kiện CẦN — một mô hình
không khớp nổi cả dữ liệu nó đã thấy chắc chắn không giải được bài toán.

Chỗ lệch: "cần" khác hẳn "đủ". Bài `qua-khop-mang-no-ron` (q8.2c) đã đo
THẬT một mạng khớp train GẦN HOÀN HẢO nhưng val loss lại TĂNG gần `9,5`
lần — độ chính xác train cao không loại trừ khả năng mạng chỉ đang "học
thuộc lòng" từng điểm cụ thể, thay vì học đúng quy luật đứng sau dữ liệu.
`8/8` ở bài này là một CON SỐ TỐT, nhưng để khẳng định "giải được" theo
đúng nghĩa, cần một tập kiểm tra riêng, độc lập với dữ liệu huấn luyện.
::
:::

:::opt
Không có tiêu chí chung nào cả — mỗi bài toán phân loại phi tuyến cần một
định nghĩa "giải được" riêng, không thể so sánh giữa các bài toán khác
nhau
::why
Gần đúng ở việc thận trọng khi so sánh CÁC BÀI TOÁN khác nhau — chi tiết
cụ thể (ngưỡng độ chính xác nào là "đủ tốt") ĐÚNG LÀ phụ thuộc vào bài
toán và ứng dụng.

Chỗ lệch: câu hỏi ở đây không phải về NGƯỠNG cụ thể (`90%` hay `99%`) — nó
về TIÊU CHÍ chung để đánh giá "mô hình học đúng hay chỉ học thuộc": tách
riêng dữ liệu ĐÃ DÙNG để cập nhật tham số khỏi dữ liệu CHƯA TỪNG DÙNG là
một nguyên tắc chung, áp dụng cho MỌI bài toán học có giám sát, từ hồi quy
tuyến tính (T8.1a) tới mạng nơ-ron sâu — không phải một tiêu chí đặc thù
của riêng bài toán này.
::
:::
::::

::::code{#viet_boss_dong_track}
Hoàn thiện bốn chỗ trống: toạ độ `x1` của hình trăng thứ hai (lật và dịch
từ `cos`), forward pass tính `logits` trong vòng lặp huấn luyện, gọi
`adam_step_l2` với phạt L2 (`lam=0,001`), và tính `do_chinh_xac` từ
`so_dung`.

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

def sinh_du_lieu_hai_trang(seed, so_diem_moi_lop, do_nhieu):
    rng = np.random.default_rng(seed)
    theta0 = rng.uniform(0.0, np.pi, size=so_diem_moi_lop)
    x0 = np.cos(theta0) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    y0 = np.sin(theta0) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    theta1 = rng.uniform(0.0, np.pi, size=so_diem_moi_lop)
    x1 = ___                          # 1.0 - np.cos(theta1) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    y1 = -np.sin(theta1) + 0.5 + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    X = np.concatenate([np.stack([x0, y0], axis=1), np.stack([x1, y1], axis=1)])
    y = np.array([0] * so_diem_moi_lop + [1] * so_diem_moi_lop)
    return X, y


X, y = sinh_du_lieu_hai_trang(seed=3, so_diem_moi_lop=4, do_nhieu=0.12)

mlp = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params = mlp.params()
m_list = [0.0] * len(params)
v_list = [0.0] * len(params)

for epoch in range(1, 16):
    loss_tong = Value(0.0)
    for (x1v, x2v), yv in zip(X, y):
        logits = ___                    # mlp.forward([Value(x1v), Value(x2v)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, int(yv))
    loss_tb = loss_tong * (1.0 / len(X))
    loss_tb.backward()
    ___                                  # adam_step_l2(params, epoch, m_list, v_list, lr=0.15, lam=0.001)

so_dung = 0
for (x1v, x2v), yv in zip(X, y):
    logits = mlp.forward([Value(x1v), Value(x2v)])
    du_doan = du_doan_lop(logits)
    so_dung += (du_doan == yv)

do_chinh_xac = ___                       # so_dung / len(y)

print(round(loss_tb.data, 4))
print(so_dung, "/", len(y))
print(round(do_chinh_xac, 4))
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

def sinh_du_lieu_hai_trang(seed, so_diem_moi_lop, do_nhieu):
    rng = np.random.default_rng(seed)
    theta0 = rng.uniform(0.0, np.pi, size=so_diem_moi_lop)
    x0 = np.cos(theta0) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    y0 = np.sin(theta0) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    theta1 = rng.uniform(0.0, np.pi, size=so_diem_moi_lop)
    x1 = 1.0 - np.cos(theta1) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    y1 = -np.sin(theta1) + 0.5 + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)
    X = np.concatenate([np.stack([x0, y0], axis=1), np.stack([x1, y1], axis=1)])
    y = np.array([0] * so_diem_moi_lop + [1] * so_diem_moi_lop)
    return X, y


X, y = sinh_du_lieu_hai_trang(seed=3, so_diem_moi_lop=4, do_nhieu=0.12)

mlp = khoi_tao_mlp(seed=0, kich_thuoc=[2, 2, 2])
params = mlp.params()
m_list = [0.0] * len(params)
v_list = [0.0] * len(params)

for epoch in range(1, 16):
    loss_tong = Value(0.0)
    for (x1v, x2v), yv in zip(X, y):
        logits = mlp.forward([Value(x1v), Value(x2v)])
        loss_tong = loss_tong + cross_entropy_qua_value(logits, int(yv))
    loss_tb = loss_tong * (1.0 / len(X))
    loss_tb.backward()
    adam_step_l2(params, epoch, m_list, v_list, lr=0.15, lam=0.001)

so_dung = 0
for (x1v, x2v), yv in zip(X, y):
    logits = mlp.forward([Value(x1v), Value(x2v)])
    du_doan = du_doan_lop(logits)
    so_dung += (du_doan == yv)

do_chinh_xac = so_dung / len(y)

print(round(loss_tb.data, 4))
print(so_dung, "/", len(y))
print(round(do_chinh_xac, 4))
```

```python title=test
assert round(loss_tb.data, 4) == 0.0108, f"loss cuoi sai -- dang ra {round(loss_tb.data, 4)}"
assert so_dung == 8, f"phai phan loai DUNG CA 8 diem -- dang ra so_dung={so_dung}"
assert len(y) == 8, f"tong so diem phai la 8 -- dang ra {len(y)}"
assert round(do_chinh_xac, 4) == 1.0, f"do_chinh_xac sai -- dang ra {round(do_chinh_xac, 4)}"

# rieng kiem tra du lieu duoc sinh DUNG hinh dang: hai hinh trang phai
# KHONG chong lan hoan toan (mot phep kiem tra tho: tam hai lop phai cach
# nhau mot khoang ro rang) -- xac nhan x1 dung cong thuc lat "1 - cos"
# (khong bi nham thanh "cos" tran, se lam hai hinh trang chong len nhau
# gan het thay vi mac cai).
tam_x0 = sum(X[i][0] for i in range(len(y)) if y[i] == 0) / 4
tam_x1 = sum(X[i][0] for i in range(len(y)) if y[i] == 1) / 4
assert abs(tam_x1 - tam_x0) > 0.3, "tam hai hinh trang theo truc x phai cach nhau ro ret -- kiem tra lai cong thuc x1"

# rieng kiem tra khong dut do thi: MOI tham so phai co grad khac 0 ngay
# sau backward() cuoi cung.
loss_tb.backward()
so_khac_0 = sum(1 for p in params if abs(p.grad) > 1e-9)
assert so_khac_0 == len(params), f"MOI tham so phai co grad khac 0 sau backward() -- co {len(params) - so_khac_0}/{len(params)} dang grad=0"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `x1`: LẬT `cos` (trừ từ `1,0`) rồi cộng nhiễu, đối xứng với công thức `y1` phía dưới nó — `1.0 - np.cos(theta1) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)`. `logits`: forward một điểm, giữ nguyên danh sách hai logit — `mlp.forward([Value(x1v), Value(x2v)])`. Chỗ gọi Adam: đúng sáu tham số vị trí cộng `lr` VÀ `lam` — `adam_step_l2(params, epoch, m_list, v_list, lr=0.15, lam=0.001)`. `do_chinh_xac`: tỉ lệ số điểm đúng trên tổng số điểm — `so_dung / len(y)`.
- kind: strategy
  body: 'x1: `1.0 - np.cos(theta1) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)`. logits: `mlp.forward([Value(x1v), Value(x2v)])`. Adam: `adam_step_l2(params, epoch, m_list, v_list, lr=0.15, lam=0.001)`. do_chinh_xac: `so_dung / len(y)`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `1.0 - np.cos(theta1) + rng.uniform(-do_nhieu, do_nhieu, size=so_diem_moi_lop)`, `mlp.forward([Value(x1v), Value(x2v)])`, `adam_step_l2(params, epoch, m_list, v_list, lr=0.15, lam=0.001)`, và `so_dung / len(y)`.'
:::

:::validate
- tier: run
  timeoutMs: 12000
- tier: static
  onFail: x1 phai dung np.cos VOI phep lat (1.0 - cos), khong duoc chep y het x0; logits phai goi THAT mlp.forward trong vong lap huan luyen; phai goi THAT adam_step_l2 VOI ca lam=0.001; do_chinh_xac phai tinh THAT bang phep chia so_dung/len(y)
  requireAst:
  - kind: uses-call, target: cos, min: 2
  - kind: uses-call, target: forward, min: 3
  - kind: has-literal, target: "0.001", min: 1
  - kind: uses-name, target: so_dung, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (cos=2: x0 [cho san, dinh nghia
  # khong tinh] cong x1 [blank] -- dung 2 GOI THAT su np.cos; forward=3:
  # goi noi bo trong MLP.forward "tang.forward(a)" [1], goi that trong
  # vong lap huan luyen [1], goi lai trong vong lap tinh so_dung [1];
  # has-literal "0.001"=1: xuat hien dung mot lan trong loi goi
  # adam_step_l2(..., lam=0.001) -- kiem tra tham so tu khoa CO duoc
  # truyen, vi keyword argument khong phai ast.Name nen uses-name "lam"
  # khong phan biet duoc co truyen hay khong; so_dung=2: doc lai trong
  # "do_chinh_xac = so_dung / len(y)" VA trong dong in ket qua cuoi).
  #
  # Cheat "x1 khong dung cos (vd chi = rng.uniform(...))" lam cos tut
  # xuong 1 -- bi chan boi static; da tu kiem chung: cheat nay VAN cho
  # so_dung=8/8 tren du lieu nho nay (mang du manh de tach qua dac trung
  # con lai), NHUNG loss doi han tu 0.0108 thanh 0.144 -- bi bat boi
  # output. Rieng assertion tam_x0/tam_x1 (kiem tra CACH tinh x1, khong
  # chi ket qua cuoi) bat duoc DOC LAP: da tu kiem chung, cheat nay lam
  # khoang cach tam hai lop tut tu 1,32 xuong chi con 0,11 (duoi nguong
  # 0,3) -- day la ly do assertion nay ton tai, khong chi dua vao
  # so_dung/loss. Cheat "logits dummy trong vong lap huan luyen" lam
  # forward tut xuong 2 -- bi chan; da tu kiem chung: doi loss tu 0.0108
  # thanh 0.6931, so_dung tu 8 xuong 3. Cheat "goi adam_step_l2 nhung BO
  # qua lam=0.001 (dung mac dinh lam=0.0)" lam has-literal "0.001" ve 0 --
  # bi chan boi static RIENG (day la mutation NGUY HIEM: da tu kiem chung
  # ca hai loss lam=0.001 va lam=0.0 deu LAM TRON ve DUNG 0.0108 -- giong
  # het nhau sau round(x,4), so_dung=8 ca hai -- output/tests KHONG bat
  # duoc mutation nay mot minh, CHI static moi bat duoc, dung ly do tang
  # static ton tai). Cheat "do_chinh_xac = 1.0 (chep san)" lam so_dung tut
  # xuong 1 -- bi chan.
- tier: tests
  timeoutMs: 12000
- tier: output
  match: regex
  expect: "^0\\.0108\\n8 / 8\\n1\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`8/8` — hai hình trăng, hoàn hảo. `32/32` khái niệm của T8.2 — HOÀN TẤT.
::::

::::reflect{#nghi-lai}
T8.2 "Mạng nơ-ron từ số 0" khép lại tại đây, `32/32` khái niệm, bốn quest:

> **q8.2a** (`neuron-va-mang-nhieu-tang`, `9` khái niệm — tính cả BOSS
> của nó) — neuron đơn, hàm kích hoạt, vì sao cần phi tuyến (XOR), forward
> pass nhiều tầng, khởi tạo trọng số, độ sâu và độ rộng, hồi quy đối phân
> loại.
>
> **q8.2b** (`dao-ham-tu-dong`, `9` khái niệm) — đồ thị tính toán, đạo hàm
> ngược chuỗi, `Value` cộng/trừ/nhân, `Value` mũ/tanh/relu, sắp xếp tô-pô
> và lan truyền ngược, tích luỹ gradient, kiểm đạo hàm bằng số, đối chiếu
> hồi quy tuyến tính.
>
> **q8.2c** (`huan-luyen-mang-no-ron`, `9` khái niệm) — neuron qua
> autograd, hàm mất mát qua `Value`, vòng lặp huấn luyện, minibatch/SGD,
> động lượng, Adam, quá khớp trên mạng thật, L2/dropout.
>
> **q8.2d** (`boss-mang-no-ron-tu-so-0`, `6` khái niệm — track BẠN vừa
> hoàn thành) — dữ liệu phi tuyến thật (hai vòng tròn), ráp framework,
> huấn luyện trên dữ liệu phi tuyến, so sánh ba bộ tối ưu, gradient biến
> mất, và BOSS đóng track này trên một dữ liệu phi tuyến thứ hai (hai hình
> trăng).

Framework xây suốt track này — `Value`, `Layer`/`MLP`, hàm mất mát, Adam —
KHÔNG phải một bài tập lý thuyết. Nó vừa giải được một bài toán mà mô hình
tuyến tính (logistic regression, cả track T8.1) không có cách nào giải
được, đo bằng số thật, hai lần, trên hai hình dạng dữ liệu khác nhau.

Nhưng bài `gradient-bien-mat-that` để lại một giới hạn CHƯA giải quyết:
mạng càng sâu, tầng đầu càng khó nhận được tín hiệu học. Kiến trúc
`Layer`/`MLP` xây tuần tự (mỗi tầng CHỈ nhận từ tầng liền trước) không có
cách nào tự sửa lỗi này. T8.3 "Transformer từ số 0" sẽ giới thiệu một kiến
trúc khác hẳn — không xây tuần tự tầng-nối-tầng, mà để MỌI vị trí trong
một chuỗi "chú ý" trực tiếp tới MỌI vị trí khác, một cách giải quyết vấn đề
gradient biến mất mà track này chưa chạm tới.
::::

::::checkpoint{mastery=0.9}
::::
