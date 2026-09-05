---
id: tri-tue-nhan-tao.dong-co-tensor.boss-dong-co-tensor
title: "BOSS — Động cơ Tensor, gradient-check trên biểu thức hỗn hợp"
summary: "Ráp matmul + softmax + layernorm + nhân element-wise thành MỘT biểu thức: Z = (X.matmul(W).softmax().layernorm()) * Wc, gọi MỘT lệnh Z.backward() duy nhất. X shape (2,3), W shape (3,4): X.grad=[[1,0165; -1,0165; 0,52678],[-0,314832; 0,314832; -1,162374]], W.grad khớp đủ 12 phần tử. Kiểm bằng finite-difference THẬT trên TOÀN BỘ biểu thức (không chỉ từng phép riêng lẻ): sai số tối đa đo được là 2,42e-10 -- dưới ngưỡng 1e-4. Đóng quest dong-co-tensor, bàn giao động cơ Tensor đầy đủ cho co-che-attention."
locale: vi
track: tri-tue-nhan-tao
module: dong-co-tensor
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-dong-co-tensor]
requires: [ai.chuan-hoa-lop]
concepts: [ai.boss-dong-co-tensor]
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
Bảy bài, bảy phép toán, mỗi phép kiểm RIÊNG bằng finite-difference. Bài này
ráp TẤT CẢ vào MỘT biểu thức — và kiểm lại từ đầu, trên CẢ CHUỖI.
::::

::::explain{#rap_toan_bo_dong_co}
Động cơ `Tensor` đầy đủ của quest này, KHÔNG thêm gì mới:

> **`__add__`/`__sub__`/`__mul__`** — element-wise, CÙNG shape (bài
> `lop-tensor-co-ban`).
>
> **`matmul`** — nhân ma trận thật, forward `self.data @ other.data`,
> backward `dA = dC @ Bᵀ`, `dB = Aᵀ @ dC` (bài `nhan-ma-tran-forward`/
> `nhan-ma-tran-backward`).
>
> **`transpose`/`reshape`** — biến đổi hình dạng, backward là phép NGƯỢC
> (`.T` lại, hoặc `reshape` về shape gốc) — bài `chuyen-vi-va-dinh-hinh-lai`.
>
> **`softmax`** — chuẩn hoá tổng mỗi hàng `= 1`, ỔN ĐỊNH SỐ HỌC (trừ max
> trước exp, TÁI DÙNG `on-dinh-so-hoc`), backward công thức gộp `s·(dy −
> dot)` — bài `softmax-qua-tensor`.
>
> **`layernorm`** — chuẩn hoá trung bình `= 0`, phương sai `= 1`, bảo vệ
> bằng `eps`, backward ba số hạng (trực tiếp, qua `μ`, qua `σ²`) — bài
> `chuan-hoa-lop`.
>
> **`backward()`** — sắp xếp tô-pô + duyệt ngược, TÁI DÙNG NGUYÊN VẸN từ
> `Value` (q8.2b), chỉ đổi seed `1.0` thành `np.ones_like(data)`.

BOSS này ráp MỘT biểu thức dùng CẢ BỐN phép toán mới (`matmul`, `softmax`,
`layernorm`, và một phép nhân element-wise cuối):

> `H = X.matmul(W)` — chiếu `X` (shape `(2, 3)`, "2 token, 3 chiều") qua
> `W` (shape `(3, 4)`, "chiếu sang 4 chiều") — giống hệt vai trò một phép
> chiếu Query/Key/Value mà attention (quest sau) sẽ dùng.
>
> `P = H.softmax()` — biến mỗi hàng của `H` thành một phân phối.
>
> `Y = P.layernorm()` — chuẩn hoá lại mỗi hàng của `P`.
>
> `Z = Y * Wc` — nhân element-wise với một trọng số CỐ ĐỊNH `Wc` (đóng vai
> trò một gradient đến TỪ MỘT TẦNG SAU nào đó, không phải `1` đồng loạt —
> để gradient-check không rơi vào trường hợp ĐẶC BIỆT quá đơn giản).

Gọi DUY NHẤT `Z.backward()` — không có gì phải viết thêm cho việc XÂU
CHUỖI: `_prev`/`_backward`/tô-pô đã xử lý đúng việc lan gradient qua NHIỀU
phép toán khác nhau liên tiếp, y hệt cách nó đã xử lý một `Value` dùng lặp
lại (q8.2b) hay một `Tensor` dùng lặp lại (`chuyen-vi-va-dinh-hinh-lai`).
::::

::::example{#boss_full_pipeline}
Toàn bộ động cơ `Tensor`, ráp một biểu thức `matmul → softmax → layernorm →
*`, gọi MỘT lệnh `.backward()`:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __sub__(self, other):
        out = Tensor(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        out = Tensor(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
        out._backward = _backward
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        def _backward():
            self.grad += out.grad.T
        out._backward = _backward
        return out

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        def _backward():
            self.grad += out.grad.reshape(self.data.shape)
        out._backward = _backward
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        def _backward():
            dy = out.grad
            dot = np.sum(dy * s, axis=-1, keepdims=True)
            self.grad += s * (dy - dot)
        out._backward = _backward
        return out

    def layernorm(self, eps=1e-5):
        x = self.data
        mu = np.mean(x, axis=-1, keepdims=True)
        xm = x - mu
        var = np.mean(xm ** 2, axis=-1, keepdims=True)
        std = np.sqrt(var + eps)
        xhat = xm / std
        out = Tensor(xhat, (self,), 'layernorm')
        N = x.shape[-1]
        def _backward():
            dy = out.grad
            dvar = np.sum(dy * xm, axis=-1, keepdims=True) * (-0.5) * std ** (-3)
            dmu = np.sum(dy * (-1.0 / std), axis=-1, keepdims=True) + dvar * np.mean(-2.0 * xm, axis=-1, keepdims=True)
            self.grad += dy / std + dvar * 2.0 * xm / N + dmu / N
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
        self.grad = np.ones_like(self.data)
        for node in reversed(topo):
            node._backward()


X = Tensor([[1., 0., -1.], [0.5, 2., 1.]])                          # shape (2, 3)
W = Tensor([[1., 0., 2., 1.], [0., 1., -1., 0.], [2., -1., 0., 1.]])  # shape (3, 4)
Wc = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.]])                    # shape (2, 4)

H = X.matmul(W)
P = H.softmax()
Y = P.layernorm()
Z = Y * Wc
Z.backward()

print(np.round(Z.data, 6).tolist())
print(np.round(X.grad, 6).tolist())
print(np.round(W.grad, 6).tolist())
```

```text title=readonly
[[-0.910064, -0.0, 1.64488, -0.0], [0.0, -0.500908, -0.0, -0.102898]]
[[1.0165, -1.0165, 0.52678], [-0.314832, 0.314832, -1.162374]]
[[-0.258707, -0.354461, 0.504623, 0.108545], [-1.392068, 0.520781, -0.108883, 0.980169], [-0.785344, 0.745047, -0.586285, 0.626582]]
```

Một lệnh `Z.backward()` DUY NHẤT lan gradient qua BỐN phép toán liên tiếp
(`matmul`, `softmax`, `layernorm`, `*`) — không cần gọi tay từng
`_backward()` theo thứ tự, không cần viết thêm bất kỳ dòng nào để XÂU
CHUỖI. Nhưng đây vẫn CHỈ là công thức chạy được — bằng chứng nó ĐÚNG nằm ở
ví dụ sau.
::::

::::example{#boss_finite_difference_toan_dien}
Kiểm bằng finite-difference trên TOÀN BỘ biểu thức (không phải riêng từng
phép) — một hàm `numpy` THUẦN, viết lại y hệt pipeline nhưng KHÔNG hề chạm
vào `Tensor`, rồi nhích TỪNG phần tử của `X` VÀ `W`:

```python title=readonly
import numpy as np

X_data = np.array([[1., 0., -1.], [0.5, 2., 1.]])
W_data = np.array([[1., 0., 2., 1.], [0., 1., -1., 0.], [2., -1., 0., 1.]])
Wc_data = np.array([[1., 0., 1., 0.], [0., 1., 0., 1.]])


def softmax_stable(z):
    zs = z - np.max(z, axis=-1, keepdims=True)
    e = np.exp(zs)
    return e / np.sum(e, axis=-1, keepdims=True)


def layernorm_np(x, eps=1e-5):
    mu = np.mean(x, axis=-1, keepdims=True)
    var = np.mean((x - mu) ** 2, axis=-1, keepdims=True)
    return (x - mu) / np.sqrt(var + eps)


def L_np(X_data, W_data, Wc_data):
    H_ = X_data @ W_data
    P_ = softmax_stable(H_)
    Y_ = layernorm_np(P_)
    Z_ = Y_ * Wc_data
    return np.sum(Z_)


X_grad_giai_tich = np.array([[1.0165, -1.0165, 0.52678], [-0.314832, 0.314832, -1.162374]])
W_grad_giai_tich = np.array([
    [-0.258707, -0.354461, 0.504623, 0.108545],
    [-1.392068, 0.520781, -0.108883, 0.980169],
    [-0.785344, 0.745047, -0.586285, 0.626582],
])

h = 1e-5
sai_so_toi_da = 0.0
for i in range(2):
    for j in range(3):
        Xp = X_data.copy(); Xp[i, j] += h
        Xm = X_data.copy(); Xm[i, j] -= h
        so = (L_np(Xp, W_data, Wc_data) - L_np(Xm, W_data, Wc_data)) / (2 * h)
        sai_so_toi_da = max(sai_so_toi_da, abs(so - X_grad_giai_tich[i, j]))
for i in range(3):
    for j in range(4):
        Wp = W_data.copy(); Wp[i, j] += h
        Wm = W_data.copy(); Wm[i, j] -= h
        so = (L_np(X_data, Wp, Wc_data) - L_np(X_data, Wm, Wc_data)) / (2 * h)
        sai_so_toi_da = max(sai_so_toi_da, abs(so - W_grad_giai_tich[i, j]))

print(f"{sai_so_toi_da:.2e}")
print(sai_so_toi_da < 1e-4)
```

```text title=readonly
2.42e-10
True
```

Sai số tối đa đo được, trên CẢ `6` phần tử của `X.grad` VÀ `12` phần tử của
`W.grad` (`18` phép kiểm riêng biệt), là `2,42e-10` — thấp hơn `1e-4` ở MỌI
phần tử. `L_np` không hề import hay gọi `Tensor` — nó là bằng chứng ĐỘC
LẬP HOÀN TOÀN rằng bốn phép toán mới của quest này, GHÉP LẠI thành một
chuỗi, lan gradient ĐÚNG.
::::

::::predict{#doan_kiem_tren_chuoi commitOnce}
Giả sử `matmul.backward` (một phép toán TRUNG GIAN trong chuỗi) có MỘT lỗi
nhỏ (ví dụ nhầm `Aᵀ @ dC` thành `A @ dC`, bỏ chuyển vị) — nhưng bạn KHÔNG
kiểm riêng từng phép, CHỈ chạy gradient-check trên TOÀN BỘ biểu thức
`Z = ...` như ví dụ trên.

**Trước khi trả lời**, bạn đoán: gradient-check trên TOÀN BỘ biểu thức có
còn phát hiện được lỗi đó không?

:::opt{correct}
Có — lỗi ở `matmul.backward` làm SAI gradient lan qua nó, và gradient sai
đó tiếp tục lan tới TẬN `X.grad`/`W.grad` (những gradient cuối cùng được so
sánh với finite-difference); sai số giữa `X.grad`/`W.grad` và finite-
difference sẽ VƯỢT xa ngưỡng `1e-4`, dù lỗi nằm ở một phép TRUNG GIAN, không
phải phép cuối
:::

:::opt
Không — chỉ kiểm RIÊNG từng phép toán (như các bài trước đã làm) mới phát
hiện được lỗi ở một phép cụ thể; kiểm trên CẢ chuỗi không đủ chi tiết
::why
Gần đúng ở việc kiểm RIÊNG từng phép đúng là hữu ích để KHOANH VÙNG chính
xác lỗi nằm ở đâu (matmul, softmax, hay layernorm) — một lợi thế thật của
cách kiểm từng phần.

Chỗ lệch: câu hỏi không phải "cách nào tốt hơn để TÌM RA lỗi nằm ở đâu" —
mà là "kiểm trên cả chuỗi có PHÁT HIỆN được lỗi tồn tại hay không". Chain
rule composition nghĩa là MỘT lỗi ở bất kỳ đâu trong chuỗi đều lan gradient
SAI tới tận đầu ra — gradient-check toàn diện VẪN thấy sai số vượt ngưỡng,
chỉ là nó không tự nói CHO BẠN BIẾT lỗi nằm ở bước nào.
::
:::

:::opt
Chỉ phát hiện được nếu lỗi nằm ở PHÉP CUỐI CÙNG của chuỗi (`__mul__` với
`Wc`); lỗi ở phép TRUNG GIAN sẽ bị các phép sau "sửa lại" một phần
::why
Gần đúng ở việc để ý VỊ TRÍ của một phép toán trong chuỗi có thể là một yếu
tố đáng chú ý khi GỠ LỖI — quan điểm đó không sai hoàn toàn trong thực hành
debug.

Chỗ lệch: không có phép toán nào "sửa lại" một gradient đã sai — chain rule
chỉ NHÂN/CỘNG DỒN các đạo hàm cục bộ liên tiếp, không có cơ chế nào triệt
tiêu một lỗi đã xảy ra ở một bước trước đó. Lỗi ở BẤT KỲ vị trí nào trong
chuỗi (đầu, giữa, cuối) đều lan gradient sai tới tận `X.grad`/`W.grad`.
::
:::
::::

::::code{#viet_boss_pipeline}
Hoàn thiện hai chỗ trống: ráp `P` bằng `softmax()` của `H` (dùng ĐỘNG CƠ
`Tensor`), và ráp `Z_` bằng phép nhân element-wise trong hàm kiểm bằng số
ĐỘC LẬP (`numpy` thuần).

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __sub__(self, other):
        out = Tensor(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        out = Tensor(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
        out._backward = _backward
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        def _backward():
            self.grad += out.grad.T
        out._backward = _backward
        return out

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        def _backward():
            self.grad += out.grad.reshape(self.data.shape)
        out._backward = _backward
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        def _backward():
            dy = out.grad
            dot = np.sum(dy * s, axis=-1, keepdims=True)
            self.grad += s * (dy - dot)
        out._backward = _backward
        return out

    def layernorm(self, eps=1e-5):
        x = self.data
        mu = np.mean(x, axis=-1, keepdims=True)
        xm = x - mu
        var = np.mean(xm ** 2, axis=-1, keepdims=True)
        std = np.sqrt(var + eps)
        xhat = xm / std
        out = Tensor(xhat, (self,), 'layernorm')
        N = x.shape[-1]
        def _backward():
            dy = out.grad
            dvar = np.sum(dy * xm, axis=-1, keepdims=True) * (-0.5) * std ** (-3)
            dmu = np.sum(dy * (-1.0 / std), axis=-1, keepdims=True) + dvar * np.mean(-2.0 * xm, axis=-1, keepdims=True)
            self.grad += dy / std + dvar * 2.0 * xm / N + dmu / N
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
        self.grad = np.ones_like(self.data)
        for node in reversed(topo):
            node._backward()


X = Tensor([[1., 0., -1.], [0.5, 2., 1.]])
W = Tensor([[1., 0., 2., 1.], [0., 1., -1., 0.], [2., -1., 0., 1.]])
Wc = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.]])

H = X.matmul(W)
P = ___                                    # H.softmax()
Y = P.layernorm()
Z = Y * Wc
Z.backward()

print(np.round(Z.data, 6).tolist())
print(np.round(X.grad, 6).tolist())
print(np.round(W.grad, 6).tolist())


def softmax_stable(z):
    zs = z - np.max(z, axis=-1, keepdims=True)
    e = np.exp(zs)
    return e / np.sum(e, axis=-1, keepdims=True)


def layernorm_np(x, eps=1e-5):
    mu = np.mean(x, axis=-1, keepdims=True)
    var = np.mean((x - mu) ** 2, axis=-1, keepdims=True)
    return (x - mu) / np.sqrt(var + eps)


def L_np(X_data, W_data, Wc_data):
    H_ = X_data @ W_data
    P_ = softmax_stable(H_)
    Y_ = layernorm_np(P_)
    Z_ = ___                               # Y_ * Wc_data
    return np.sum(Z_)


h = 1e-5
def fd_X(i, j):
    Xp = X.data.copy(); Xp[i, j] += h
    Xm = X.data.copy(); Xm[i, j] -= h
    return (L_np(Xp, W.data, Wc.data) - L_np(Xm, W.data, Wc.data)) / (2 * h)


def fd_W(i, j):
    Wp = W.data.copy(); Wp[i, j] += h
    Wm = W.data.copy(); Wm[i, j] -= h
    return (L_np(X.data, Wp, Wc.data) - L_np(X.data, Wm, Wc.data)) / (2 * h)


sai_so_toi_da = 0.0
for i in range(2):
    for j in range(3):
        sai_so_toi_da = max(sai_so_toi_da, abs(fd_X(i, j) - X.grad[i, j]))
for i in range(3):
    for j in range(4):
        sai_so_toi_da = max(sai_so_toi_da, abs(fd_W(i, j) - W.grad[i, j]))

print(f"{sai_so_toi_da:.2e}")
print(sai_so_toi_da < 1e-4)
```

```python title=solution
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __sub__(self, other):
        out = Tensor(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        out = Tensor(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
        out._backward = _backward
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        def _backward():
            self.grad += out.grad.T
        out._backward = _backward
        return out

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        def _backward():
            self.grad += out.grad.reshape(self.data.shape)
        out._backward = _backward
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        def _backward():
            dy = out.grad
            dot = np.sum(dy * s, axis=-1, keepdims=True)
            self.grad += s * (dy - dot)
        out._backward = _backward
        return out

    def layernorm(self, eps=1e-5):
        x = self.data
        mu = np.mean(x, axis=-1, keepdims=True)
        xm = x - mu
        var = np.mean(xm ** 2, axis=-1, keepdims=True)
        std = np.sqrt(var + eps)
        xhat = xm / std
        out = Tensor(xhat, (self,), 'layernorm')
        N = x.shape[-1]
        def _backward():
            dy = out.grad
            dvar = np.sum(dy * xm, axis=-1, keepdims=True) * (-0.5) * std ** (-3)
            dmu = np.sum(dy * (-1.0 / std), axis=-1, keepdims=True) + dvar * np.mean(-2.0 * xm, axis=-1, keepdims=True)
            self.grad += dy / std + dvar * 2.0 * xm / N + dmu / N
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
        self.grad = np.ones_like(self.data)
        for node in reversed(topo):
            node._backward()


X = Tensor([[1., 0., -1.], [0.5, 2., 1.]])
W = Tensor([[1., 0., 2., 1.], [0., 1., -1., 0.], [2., -1., 0., 1.]])
Wc = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.]])

H = X.matmul(W)
P = H.softmax()
Y = P.layernorm()
Z = Y * Wc
Z.backward()

print(np.round(Z.data, 6).tolist())
print(np.round(X.grad, 6).tolist())
print(np.round(W.grad, 6).tolist())


def softmax_stable(z):
    zs = z - np.max(z, axis=-1, keepdims=True)
    e = np.exp(zs)
    return e / np.sum(e, axis=-1, keepdims=True)


def layernorm_np(x, eps=1e-5):
    mu = np.mean(x, axis=-1, keepdims=True)
    var = np.mean((x - mu) ** 2, axis=-1, keepdims=True)
    return (x - mu) / np.sqrt(var + eps)


def L_np(X_data, W_data, Wc_data):
    H_ = X_data @ W_data
    P_ = softmax_stable(H_)
    Y_ = layernorm_np(P_)
    Z_ = Y_ * Wc_data
    return np.sum(Z_)


h = 1e-5
def fd_X(i, j):
    Xp = X.data.copy(); Xp[i, j] += h
    Xm = X.data.copy(); Xm[i, j] -= h
    return (L_np(Xp, W.data, Wc.data) - L_np(Xm, W.data, Wc.data)) / (2 * h)


def fd_W(i, j):
    Wp = W.data.copy(); Wp[i, j] += h
    Wm = W.data.copy(); Wm[i, j] -= h
    return (L_np(X.data, Wp, Wc.data) - L_np(X.data, Wm, Wc.data)) / (2 * h)


sai_so_toi_da = 0.0
for i in range(2):
    for j in range(3):
        sai_so_toi_da = max(sai_so_toi_da, abs(fd_X(i, j) - X.grad[i, j]))
for i in range(3):
    for j in range(4):
        sai_so_toi_da = max(sai_so_toi_da, abs(fd_W(i, j) - W.grad[i, j]))

print(f"{sai_so_toi_da:.2e}")
print(sai_so_toi_da < 1e-4)
```

```python title=test
import numpy as np

assert np.round(Z.data, 6).tolist() == [[-0.910064, -0.0, 1.64488, -0.0], [0.0, -0.500908, -0.0, -0.102898]], f"Z.data sai -- dang ra {np.round(Z.data, 6).tolist()}"
assert np.round(X.grad, 6).tolist() == [[1.0165, -1.0165, 0.52678], [-0.314832, 0.314832, -1.162374]], f"X.grad sai -- dang ra {np.round(X.grad, 6).tolist()}"
assert np.round(W.grad, 6).tolist() == [
    [-0.258707, -0.354461, 0.504623, 0.108545],
    [-1.392068, 0.520781, -0.108883, 0.980169],
    [-0.785344, 0.745047, -0.586285, 0.626582],
], f"W.grad sai -- dang ra {np.round(W.grad, 6).tolist()}"

# BANG CHUNG TRUNG TAM cua ca quest: gradient-check TOAN DIEN tren bieu
# thuc HON HOP phai dat, sai so duoi 1e-4.
assert sai_so_toi_da < 1e-4, f"sai so gradient-check toan dien phai duoi 1e-4 -- dang ra {sai_so_toi_da}"

# rieng kiem tra P THAT SU la ket qua cua softmax (tong moi hang = 1), khong
# phai mot phep chep khac (vi du gan thang P = H).
assert np.allclose(np.sum(P.data, axis=-1), 1.0), f"P phai la ket qua softmax (tong moi hang = 1) -- dang ra {np.sum(P.data, axis=-1).tolist()}"

# rieng kiem tra CONG THUC TONG QUAT cua L_np tren MOT BO THAM SO KHAC (Wc
# doi dau) -- chan cheat "Z_ = Y_" (bo qua Wc_data hoan toan).
Wc_khac = -Wc.data
gia_tri_khac = L_np(X.data, W.data, Wc_khac)
assert round(gia_tri_khac, 6) == -round(L_np(X.data, W.data, Wc.data), 6), f"L_np phai PHU THUOC Wc_data (doi dau Wc phai doi dau ket qua) -- dang ra {gia_tri_khac} va {L_np(X.data, W.data, Wc.data)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu ráp `P` từ `H` bằng phương thức `softmax()` CỦA CHÍNH `Tensor` (`H.softmax()`) — dùng động cơ đã xây, không tính tay. Chỗ hai, TRONG hàm `L_np` (hàm kiểm ĐỘC LẬP, không dùng `Tensor`), ráp `Z_` bằng phép nhân element-wise `numpy` thuần giữa `Y_` và `Wc_data` (`Y_ * Wc_data`) — PHẢI dùng đúng `Wc_data` (tham số của hàm), không phải một hằng số hay biến khác.
- kind: strategy
  body: 'Chỗ đầu: `H.softmax()`. Chỗ hai: `Y_ * Wc_data`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `H.softmax()` và `Y_ * Wc_data`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: P phai duoc rap bang H.softmax() (dung DONG CO Tensor, khong tinh tay hay gan thang H); Z_ trong L_np phai la Y_ * Wc_data (dung DUNG tham so Wc_data, khong duoc bo qua hay dung hang so khac)
  requireAst:
  - kind: uses-call, target: softmax, min: 1
  - kind: uses-name, target: Wc_data, min: 1
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # class Tensor + harness + L_np): loi giai dung dat=true. softmax=1:
  # CHI mot loi goi PHUONG THUC ten "softmax" tren toan bo file -- chinh
  # la blank dau (H.softmax()); dinh nghia "def softmax(self):" KHONG
  # tinh (khong phai Call). Wc_data=1: tham so `Wc_data` trong chu ky ham
  # `def L_np(X_data, W_data, Wc_data)` la mot ast.arg (khong dem); LAN
  # DOC duy nhat cua no la trong blank hai "Y_ * Wc_data". Cheat "P = H"
  # (gan thang, bo qua softmax hoan toan) lam "softmax" tut xuong 0 -- bi
  # chan RIENG, VA doc lap boi assert np.allclose(sum(P.data,axis=-1),
  # 1.0) trong tier tests (P=H se khong cong dung 1 tren tung hang tru khi
  # tinh co). Cheat "Z_ = Y_" (bo qua Wc_data trong L_np) lam "Wc_data" tut
  # xuong 0 -- bi chan RIENG, VA doc lap boi assert L_np PHU THUOC Wc_data
  # (doi dau Wc phai doi dau ket qua) trong tier tests -- neu Z_=Y_, doi
  # dau Wc khong anh huong gi toi L_np, lam assertion do THAT BAI ro rang.
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^\\[\\[-0\\.910064, -0\\.0, 1\\.64488, -0\\.0\\], \\[0\\.0, -0\\.500908, -0\\.0, -0\\.102898\\]\\]\\n\\[\\[1\\.0165, -1\\.0165, 0\\.52678\\], \\[-0\\.314832, 0\\.314832, -1\\.162374\\]\\]\\n\\[\\[-0\\.258707, -0\\.354461, 0\\.504623, 0\\.108545\\], \\[-1\\.392068, 0\\.520781, -0\\.108883, 0\\.980169\\], \\[-0\\.785344, 0\\.745047, -0\\.586285, 0\\.626582\\]\\]\\n2\\.42e-10\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sai số `2,42e-10` trên `18` phần tử của một biểu thức bốn phép toán xâu
chuỗi — động cơ `Tensor` của quest này ĐÚNG, không chỉ trông có lý. Quest
`dong-co-tensor` khép lại tại đây.
::::

::::reflect{#nghi-lai}
Quest `dong-co-tensor` khép lại tại đây, tám bài: vì sao cần `Tensor` (đếm
THẬT số `Value` một bảng embedding cần), lớp `Tensor` cơ bản (cộng/trừ/nhân
element-wise, giới hạn cùng shape), nhân ma trận forward rồi backward (công
thức `dA = dC·Bᵀ`, `dB = Aᵀ·dC`, kiểm bằng finite-difference), chuyển vị và
định hình lại (backward là phép ngược, tái hiện gotcha tích luỹ gradient
trên `Tensor`), softmax (công thức gộp `s·(dy−dot)`, không dựng Jacobian
đầy đủ), layer normalization (ba số hạng, `eps` bảo vệ), và BOSS này: ráp
CẢ BỐN phép toán mới thành một biểu thức, gradient-check TOÀN DIỆN — sai số
dưới `1e-4` trên toàn bộ `18` phần tử kiểm.

Động cơ `Tensor` giờ đã đủ: `matmul` (chiếu embedding sang không gian
khác), `softmax` (biến điểm số thành trọng số chú ý), `layernorm` (ổn định
mỗi tầng). Quest sau, `co-che-attention` (q8.3c), dùng CHÍNH ba phép toán
này để xây Query/Key/Value và điểm attention — không viết lại động cơ, chỉ
GHÉP chúng theo đúng công thức attention.
::::

::::checkpoint{mastery=0.9}
::::
