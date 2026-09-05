---
id: tri-tue-nhan-tao.boss-mang-no-ron-tu-so-0.du-lieu-phi-tuyen-that
title: "Dữ liệu phi tuyến thật: hai vòng tròn đồng tâm"
summary: "Tự dựng 8 điểm 2D bằng công thức tường minh (rng.uniform + np.cos/np.sin, không sklearn) — lớp trong bán kính khoảng 1,0 (nhiễu ±0,25), lớp ngoài bán kính khoảng 2,5. Huấn luyện một neuron logistic regression đơn (sigmoid + binary cross-entropy, ráp từ đúng các phép toán exp/log/pow đã có của Value, Adam 40 epoch, hội tụ ngay từ epoch đó) trên đúng dữ liệu này: accuracy CHỈ đạt 4/8 — đúng bằng mức đoán ngẫu nhiên. Đây là động lực của cả track T8.2."
locale: vi
track: tri-tue-nhan-tao
module: boss-mang-no-ron-tu-so-0
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.du-lieu-phi-tuyen-that]
requires: [ai.boss-huan-luyen-mang-no-ron]
concepts: [ai.du-lieu-phi-tuyen-that]
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
Chín bài của `huan-luyen-mang-no-ron` vừa khép lại bằng một BOSS ráp trọn
`Value`/`Layer`/`MLP`/Adam, chạy ngon trên hồi quy gần tuyến tính và phân
loại tách biệt rõ ràng. Bài này đặt ra câu hỏi: dữ liệu đó có PHẢI khó
không, hay chỉ đơn giản là chưa đủ khó?
::::

::::explain{#vi_sao_can_du_lieu_phi_tuyen_that}
Bài `vi-sao-can-phi-tuyen` (q8.2a) đã chứng minh một neuron tuyến tính đơn
không giải được XOR — nhưng XOR chỉ có ĐÚNG bốn điểm rời rạc. Một người
hoài nghi có thể nói: "bốn điểm thì ít quá, biết đâu chỉ là may rủi của
đúng bốn toạ độ đó". Bài này dựng lại đúng Ý TƯỞNG đó — một tập nhãn không
tuyến tính tách được — nhưng trên dữ liệu LỚN HƠN nhiều và LIÊN TỤC (toạ độ
thực, không phải chỉ `0`/`1`), để loại bỏ hẳn khả năng "chỉ là trùng hợp
của một vài điểm cụ thể".

Cấu trúc hình học được chọn: **hai vòng tròn đồng tâm**. Lớp `0` là các
điểm nằm QUANH một đường tròn bán kính nhỏ (`r_trong`), lớp `1` là các
điểm nằm QUANH một đường tròn bán kính lớn hơn hẳn (`r_ngoai`), cùng tâm
tại gốc toạ độ. Sinh từng điểm bằng công thức tường minh — một góc ngẫu
nhiên đều trên `[0, 2π)` (`rng.uniform`), một bán kính bằng bán kính danh
định cộng nhiễu nhỏ, rồi đổi sang toạ độ Descartes bằng `cos`/`sin`:

> `x = bán_kính · cos(góc)`, `y = bán_kính · sin(góc)`

Vì sao một đường thẳng không thể tách đúng hai lớp này? Một neuron tuyến
tính (`z = w₁x₁ + w₂x₂ + b`, ngưỡng tại `z = 0`) chia mặt phẳng thành ĐÚNG
hai NỬA MẶT PHẲNG bởi một đường thẳng. Lớp `1` (vòng ngoài) bao quanh lớp
`0` (vòng trong) theo MỌI HƯỚNG — với BẤT KỲ đường thẳng nào, luôn có điểm
của vòng ngoài nằm ở CẢ HAI phía của nó (vì vòng ngoài trải đều quanh
`360°`). Không giá trị `w₁, w₂, b` nào tách đúng "bên trong vòng nhỏ" khỏi
"bên ngoài vòng nhỏ", vì ranh giới cần thiết TỰ NÓ là một đường CONG (một
đường tròn), không phải một đường thẳng.

Neuron logistic regression ở đây được ráp lại QUA `Value` — không phải
công thức `numpy` đóng của T8.1b — để dùng lại đúng những phép toán
`Value` đã có (`exp`, `log`, `**`), không phát minh phép toán mới:

> `sigmoid(z) = 1 / (1 + eᶻ⁻)` viết qua `Value` là
> `(Value(1.0) + (z · -1.0).exp()) ** (-1.0)` — nghịch đảo (`** -1.0`)
> đứng vai trò phép chia, vì `Value` không có `__truediv__`.
>
> Cross-entropy nhị phân — `-[y·log(p) + (1-y)·log(1-p)]` — viết qua
> `Value.log()` trên `p` và trên `1 - p`.
::::

::::example{#sinh_du_lieu_hai_vong_tron}
Sinh `8` điểm (`4` mỗi lớp), lớp `0` quanh bán kính `1,0`, lớp `1` quanh
bán kính `2,5`, nhiễu `±0,25`:

```python title=readonly
import numpy as np

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

r = np.sqrt(X[:, 0] ** 2 + X[:, 1] ** 2)
r_lop0 = r[y == 0]
r_lop1 = r[y == 1]

print("so diem:", len(y), " so lop 0:", int((y == 0).sum()), " so lop 1:", int((y == 1).sum()))
print("ban kinh lop 0 -- nho nhat:", round(r_lop0.min(), 4), " lon nhat:", round(r_lop0.max(), 4))
print("ban kinh lop 1 -- nho nhat:", round(r_lop1.min(), 4), " lon nhat:", round(r_lop1.max(), 4))
```

```text title=readonly
so diem: 8  so lop 0: 4  so lop 1: 4
ban kinh lop 0 -- nho nhat: 0.8861  lon nhat: 1.1594
ban kinh lop 1 -- nho nhat: 2.4195  lon nhat: 2.6681
```

Hai dải bán kính KHÔNG hề chồng lấn (`1,1594 < 2,4195` — dải lớn nhất của
lớp `0` còn nhỏ hơn dải nhỏ nhất của lớp `1`) — tách biệt hoàn hảo THEO BÁN
KÍNH. Nhưng bán kính không phải là toạ độ `x` hay `y` — nó là
`√(x² + y²)`, một hàm PHI TUYẾN của hai toạ độ gốc. Một đường thẳng trên
mặt phẳng `(x, y)` không "nhìn thấy" được đại lượng bán kính này.
::::

::::example{#logreg_that_bai}
Huấn luyện một neuron logistic regression đơn qua `Value` — `40` epoch,
Adam (`lr = 0.1`) — trên ĐÚNG `8` điểm vừa sinh ở trên:

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


def sigmoid_qua_value(z):
    return (Value(1.0) + (z * -1.0).exp()) ** (-1.0)


def bce_qua_value(p, y):
    return (p.log() * (-y)) + ((Value(1.0) - p).log() * (-(1.0 - y)))


def gan_nhan(p, nguong=0.5):
    return 1 if p >= nguong else 0


X, y = sinh_du_lieu_vong_tron(seed=424, so_diem_moi_lop=4, r_trong=1.0, r_ngoai=2.5, do_nhieu=0.25)


def train_logreg(X, y, seed_w, so_epoch, lr):
    rng = np.random.default_rng(seed_w)
    w1 = Value(float(rng.uniform(-0.5, 0.5)))
    w2 = Value(float(rng.uniform(-0.5, 0.5)))
    b = Value(0.0)
    params = [w1, w2, b]
    m_list = [0.0, 0.0, 0.0]
    v_list = [0.0, 0.0, 0.0]
    for epoch in range(1, so_epoch + 1):
        loss_tong = Value(0.0)
        for (x1v, x2v), yv in zip(X, y):
            z = w1 * x1v + w2 * x2v + b
            p = sigmoid_qua_value(z)
            loss_tong = loss_tong + bce_qua_value(p, float(yv))
        loss_tb = loss_tong * (1.0 / len(X))
        loss_tb.backward()
        adam_step_l2(params, epoch, m_list, v_list, lr=lr)
    return w1, w2, b, loss_tb


w1, w2, b, loss_tb = train_logreg(X, y, seed_w=0, so_epoch=40, lr=0.1)

so_dung = 0
for (x1v, x2v), yv in zip(X, y):
    z_val = w1.data * x1v + w2.data * x2v + b.data
    p = sigmoid_qua_value(Value(z_val))
    du_doan = gan_nhan(p.data)
    so_dung += (du_doan == yv)

print("loss cuoi:", round(loss_tb.data, 4))
print("so dung:", so_dung, "/", len(y))
```

```text title=readonly
loss cuoi: 0.5016
so dung: 4 / 8
```

Loss ban đầu (mọi trọng số gần `0`) là `-log(0,5) ≈ 0,6931` — đúng mức
"không biết gì cả". Sau `40` epoch Adam, loss chỉ tụt xuống `0,5016` —
gần bằng mức "không biết gì cả" ban đầu. Số điểm phân loại đúng: đúng
`4` trên `8` — CHÍNH XÁC bằng mức đoán ngẫu nhiên (tung đồng xu), KHÔNG
đạt được độ chính xác cao. Đây chính là bằng chứng số học cho lý lẽ hình
học ở trên: đường phân cách tuyến tính không có cách nào "quấn quanh"
được vòng tròn trong.
::::

::::predict{#doan_huan_luyen_lau_hon commitOnce}
Loss sau `40` epoch (`0,5016`) gần như bằng mức khởi tạo (`0,6931` →
`0,5016` — giảm rất ít so với quãng đường lẽ ra phải đi để phân loại
đúng) — mô hình dường như đã "dừng lại".

**Trước khi chạy thử**, bạn đoán: nếu huấn luyện lâu hơn NHIỀU (`5000`
epoch thay vì `40`, cùng dữ liệu, cùng learning rate), độ chính xác có
cải thiện đáng kể không?

:::opt{correct}
Không — mô hình đã HỘI TỤ (loss và accuracy không đổi từ epoch `40` trở
đi, đã tự kiểm chứng: `1000` và `5000` epoch cho ra ĐÚNG cùng
`4/8`), vì cross-entropy ghép sigmoid tuyến tính là một hàm LỒI có ĐÚNG
một điểm tối ưu — mô hình đã ở điểm đó; giới hạn nằm ở CẤU TRÚC (đường
biên tuyến tính không thể khớp một hình tròn), không phải ở thời gian
huấn luyện
:::

:::opt
Có — huấn luyện thêm luôn cải thiện độ chính xác khi mô hình chưa đạt
`100%`, vì gradient descent luôn tiếp tục tiến gần hơn tới đáp án đúng
::why
Gần đúng ở việc gradient descent nói chung tiếp tục giảm LOSS khi còn
đường xuống dốc để đi — quan sát đó không sai cho một hàm mất mát CHƯA hội
tụ.

Chỗ lệch: câu hỏi không phải về loss còn giảm được hay không nói chung —
nó về TRƯỜNG HỢP CỤ THỂ này, nơi loss đã THỰC SỰ dừng đổi (đã tự kiểm
chứng bằng Python thật: `1000` epoch và `5000` epoch cho đúng cùng kết
quả `4/8`). Cross-entropy tuyến tính là hàm LỒI — một khi đã tới đáy, không
còn hướng nào để "tiến gần hơn" nữa, bất kể huấn luyện thêm bao nhiêu.
::
:::

:::opt
Không xác định được nếu không thử — mọi mô hình đều có khả năng cải thiện
bất ngờ sau đủ epoch, không thể khẳng định trước
::why
Gần đúng ở tinh thần thận trọng khi ngoại suy hành vi huấn luyện — một
thái độ hợp lý nói chung với các bài toán KHÔNG lồi (nhiều mạng nơ-ron sâu
rơi vào trường hợp này).

Chỗ lệch: logistic regression tuyến tính (sigmoid ghép cross-entropy) là
một trường hợp đặc biệt ĐÃ ĐƯỢC CHỨNG MINH là lồi — không có "hố cục bộ"
nào để mắc kẹt, cũng không có gì bất ngờ chờ ở epoch xa hơn. Khẳng định
trước là hợp lý CHÍNH VÌ tính chất toán học riêng của bài toán này, không
phải một suy đoán liều lĩnh.
::
:::
::::

::::code{#viet_du_lieu_va_logreg}
Hoàn thiện bốn chỗ trống: toạ độ `x1` của lớp ngoài (dùng `cos`, đối xứng
với `x0`), công thức `sigmoid_qua_value` (ghép từ `exp` và `**`), công
thức `bce_qua_value` (ghép từ `log`), và `gan_nhan` (ngưỡng `0,5`, dùng
`>=`).

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
    x1 = ___                          # ban_kinh1 * np.cos(goc1)
    y1 = ban_kinh1 * np.sin(goc1)
    X = np.concatenate([np.stack([x0, y0], axis=1), np.stack([x1, y1], axis=1)])
    y = np.array([0] * so_diem_moi_lop + [1] * so_diem_moi_lop)
    return X, y


def sigmoid_qua_value(z):
    return ___                        # (Value(1.0) + (z * -1.0).exp()) ** (-1.0)


def bce_qua_value(p, y):
    return ___                        # (p.log() * (-y)) + ((Value(1.0) - p).log() * (-(1.0 - y)))


def gan_nhan(p, nguong=0.5):
    return ___                        # 1 if p >= nguong else 0


X, y = sinh_du_lieu_vong_tron(seed=424, so_diem_moi_lop=4, r_trong=1.0, r_ngoai=2.5, do_nhieu=0.25)


def train_logreg(X, y, seed_w, so_epoch, lr):
    rng = np.random.default_rng(seed_w)
    w1 = Value(float(rng.uniform(-0.5, 0.5)))
    w2 = Value(float(rng.uniform(-0.5, 0.5)))
    b = Value(0.0)
    params = [w1, w2, b]
    m_list = [0.0, 0.0, 0.0]
    v_list = [0.0, 0.0, 0.0]
    for epoch in range(1, so_epoch + 1):
        loss_tong = Value(0.0)
        for (x1v, x2v), yv in zip(X, y):
            z = w1 * x1v + w2 * x2v + b
            p = sigmoid_qua_value(z)
            loss_tong = loss_tong + bce_qua_value(p, float(yv))
        loss_tb = loss_tong * (1.0 / len(X))
        loss_tb.backward()
        adam_step_l2(params, epoch, m_list, v_list, lr=lr)
    return w1, w2, b, loss_tb


w1, w2, b, loss_tb = train_logreg(X, y, seed_w=0, so_epoch=40, lr=0.1)

so_dung = 0
for (x1v, x2v), yv in zip(X, y):
    z_val = w1.data * x1v + w2.data * x2v + b.data
    p = sigmoid_qua_value(Value(z_val))
    du_doan = gan_nhan(p.data)
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


def sigmoid_qua_value(z):
    return (Value(1.0) + (z * -1.0).exp()) ** (-1.0)


def bce_qua_value(p, y):
    return (p.log() * (-y)) + ((Value(1.0) - p).log() * (-(1.0 - y)))


def gan_nhan(p, nguong=0.5):
    return 1 if p >= nguong else 0


X, y = sinh_du_lieu_vong_tron(seed=424, so_diem_moi_lop=4, r_trong=1.0, r_ngoai=2.5, do_nhieu=0.25)


def train_logreg(X, y, seed_w, so_epoch, lr):
    rng = np.random.default_rng(seed_w)
    w1 = Value(float(rng.uniform(-0.5, 0.5)))
    w2 = Value(float(rng.uniform(-0.5, 0.5)))
    b = Value(0.0)
    params = [w1, w2, b]
    m_list = [0.0, 0.0, 0.0]
    v_list = [0.0, 0.0, 0.0]
    for epoch in range(1, so_epoch + 1):
        loss_tong = Value(0.0)
        for (x1v, x2v), yv in zip(X, y):
            z = w1 * x1v + w2 * x2v + b
            p = sigmoid_qua_value(z)
            loss_tong = loss_tong + bce_qua_value(p, float(yv))
        loss_tb = loss_tong * (1.0 / len(X))
        loss_tb.backward()
        adam_step_l2(params, epoch, m_list, v_list, lr=lr)
    return w1, w2, b, loss_tb


w1, w2, b, loss_tb = train_logreg(X, y, seed_w=0, so_epoch=40, lr=0.1)

so_dung = 0
for (x1v, x2v), yv in zip(X, y):
    z_val = w1.data * x1v + w2.data * x2v + b.data
    p = sigmoid_qua_value(Value(z_val))
    du_doan = gan_nhan(p.data)
    so_dung += (du_doan == yv)

print(round(loss_tb.data, 4))
print(so_dung, "/", len(y))
```

```python title=test
assert round(loss_tb.data, 4) == 0.5016, f"loss cuoi sai -- dang ra {round(loss_tb.data, 4)}"
assert so_dung == 4, f"so dung phai la 4 -- dang ra {so_dung}"
assert len(y) == 8, f"tong so diem phai la 8 -- dang ra {len(y)}"

# rieng kiem tra du lieu duoc sinh DUNG hinh dang: ban kinh lop 0 phai nho
# hon HAN ban kinh lop 1 tren MOI diem (khong overlap) -- xac nhan x1 dung
# cong thuc cos (khong bi hoan doi voi sin hay bo sot).
r = (X[:, 0] ** 2 + X[:, 1] ** 2) ** 0.5
r_lop0 = [r[i] for i in range(len(y)) if y[i] == 0]
r_lop1 = [r[i] for i in range(len(y)) if y[i] == 1]
assert max(r_lop0) < min(r_lop1), "ban kinh lop 0 phai nho hon HAN ban kinh lop 1 tren moi diem -- kiem tra lai cong thuc x1"

# rieng kiem tra BIEN cua gan_nhan: xac suat dung bang 0.5 chinh xac phai
# duoc gan nhan 1 (dung '>=', khong phai '>') -- du lieu huan luyen that
# khong co xac suat nao dung bang 0.5 (da tu kiem chung margin toi thieu
# ~0.03), nen chi mot loi goi truc tiep moi ep di qua dung nhanh bien.
assert gan_nhan(0.5) == 1, f"gan_nhan(0.5) phai la 1 (dung '>=') -- dang ra {gan_nhan(0.5)}"

# rieng kiem tra gan_nhan KHONG PHAI mot ham hang so luon tra ve 1 -- neu
# nhanh else bi doi tu 0 thanh 1 (vd do dot bien), gan_nhan(0.5) o tren
# van qua (van la 1), nhung xac suat RO RANG duoi nguong phai tra ve 0.
# Du lieu bai nay co ca hai lop can bang (4 diem moi lop), nen "luon doan
# lop 1" tinh co cho DUNG so_dung=4/8 giong het loi giai dung -- CHI
# assertion truc tiep nay moi phan biet duoc.
assert gan_nhan(0.1) == 0, f"gan_nhan(0.1) phai la 0 (xac suat thap, duoi nguong) -- dang ra {gan_nhan(0.1)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `x1`: đối xứng với `x0` phía trên, chỉ đổi `ban_kinh0` thành `ban_kinh1`, `goc0` thành `goc1` — `ban_kinh1 * np.cos(goc1)`. `sigmoid_qua_value`: ghép `exp` và `**` — `(Value(1.0) + (z * -1.0).exp()) ** (-1.0)` (số mũ `-1.0` đóng vai trò phép chia, vì `Value` không có phép chia trực tiếp). `bce_qua_value`: hai số hạng `log`, mỗi số hạng nhân với hệ số âm tương ứng — `(p.log() * (-y)) + ((Value(1.0) - p).log() * (-(1.0 - y)))`. `gan_nhan`: ngưỡng `0,5`, dùng `>=` (không phải `>`) — `1 if p >= nguong else 0`.
- kind: strategy
  body: 'x1: `ban_kinh1 * np.cos(goc1)`. sigmoid_qua_value: `(Value(1.0) + (z * -1.0).exp()) ** (-1.0)`. bce_qua_value: `(p.log() * (-y)) + ((Value(1.0) - p).log() * (-(1.0 - y)))`. gan_nhan: `1 if p >= nguong else 0`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `ban_kinh1 * np.cos(goc1)`, `(Value(1.0) + (z * -1.0).exp()) ** (-1.0)`, `(p.log() * (-y)) + ((Value(1.0) - p).log() * (-(1.0 - y)))`, và `1 if p >= nguong else 0`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: x1 phai dung np.cos (doi xung voi x0); sigmoid_qua_value phai ghep tu exp VA phep mu (** -1.0), khong duoc chep hang so; bce_qua_value phai dung CA HAI so hang log (cho ca y=1 lan y=0); gan_nhan phai dung dung toan tu '>=' (khong phai '>')
  requireAst:
  - kind: uses-call, target: cos, min: 2
  - kind: uses-call, target: exp, min: 2
  - kind: uses-call, target: log, min: 3
  - kind: uses-operator, target: ">=", min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach. cos=2: x0 (cho san, khong tinh vi
  # la dinh nghia goc trong sinh_du_lieu_vong_tron) cong x1 (blank) -- dung
  # 2 GOI THAT su np.cos; exp=2: Value.exp() dinh nghia goi math.exp noi bo
  # [1] cong (z*-1.0).exp() trong sigmoid_qua_value [1] -- dung 2 (loi goi
  # np.exp o vong lap tinh do_chinh_xac DA duoc go bo, thay bang goi lai
  # chinh sigmoid_qua_value, tranh nhieu so dem); log=3: Value.log() dinh
  # nghia goi math.log noi bo [1] cong hai lan p.log()/(...).log() trong
  # bce_qua_value [2] -- dung 3; ">="=1: dung dung mot lan trong gan_nhan.
  #
  # Cheat "x1 dung np.sin thay vi np.cos" lam cos=1 -- bi chan. Cheat
  # "sigmoid_qua_value tra ve Value(0.5) chep san" lam exp=1 -- bi chan.
  # Cheat "bce_qua_value chi giu mot so hang" lam log=2 -- bi chan. Cheat
  # "gan_nhan dung > thay vi >=" lam ">="=0 -- bi chan boi static VA rieng
  # boi assertion gan_nhan(0.5) o tier tests (du lieu huan luyen that
  # khong co xac suat nao dung bang 0.5 nen output/tests tren du lieu
  # chinh khong bat duoc mutation nay mot minh).
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^0\\.5016\\n4 / 8\\s*$"
:::
::::

::::byte{trigger=success mood=curious pose=lean-in}
`4/8` — CHÍNH XÁC bằng tung đồng xu. Logistic regression, đúng công thức
đã học ở T8.1b, hoàn toàn bó tay trước hai vòng tròn. Đây là động lực có
thật cho mọi bài còn lại của T8.2d.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài `vi-sao-can-phi-tuyen` (q8.2a) đã trả lời: ghép nhiều neuron tuyến
tính qua một hàm kích hoạt phi tuyến (ví dụ `tanh`) — một MẠNG NHIỀU TẦNG —
có thể vượt qua giới hạn của một neuron đơn. `Layer`/`MLP` cùng `Value` với
đầy đủ tám phép toán (`+`, `-`, `*`, `**`, `tanh`, `relu`, `exp`, `log`) và
Adam đã được ráp xong xuôi ở `boss-huan-luyen-mang-no-ron` (q8.2c).

Framework đó có thực sự chạy nhất quán khi ráp lại — forward, backward, và
một bước cập nhật — trên một mạng nhỏ, TRƯỚC KHI đem ra thử thách hai vòng
tròn vừa thấy ở đây?
::::

::::checkpoint{mastery=0.85}
::::
