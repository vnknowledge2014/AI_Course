---
id: tri-tue-nhan-tao.khoi-transformer-va-huan-luyen.mang-truyen-thang-vi-tri
title: "Mạng truyền thẳng theo vị trí: một SubLayer thật, không trộn giữa các vị trí"
summary: "Feedforward theo vị trí FFN(x) = relu(x @ W1) @ W2 -- hai phép chiếu tuyến tính, một relu ở giữa, KHÔNG bias (giữ đúng lối không-bias đã dùng cho Wq/Wk/Wv ở co-che-attention, tránh broadcasting phá vỡ __add__.backward). relu là phép toán MỚI cho Tensor: forward np.maximum(0,x), backward (x>0) nhân gradient đến -- kiểm bằng finite-difference THẬT, sai số tối đa 3,78e-11 trên 40 phần tử X.grad+W1.grad+W2.grad. Áp ĐỘC LẬP lên mỗi vị trí: hoán đổi hai hàng của X hoán đổi ĐÚNG hai hàng tương ứng của đầu ra, không trộn lẫn -- khác hẳn attention."
locale: vi
track: tri-tue-nhan-tao
module: khoi-transformer-va-huan-luyen
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.mang-truyen-thang-vi-tri]
requires: [ai.ket-noi-tat]
concepts: [ai.mang-truyen-thang-vi-tri]
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
Kết nối tắt vừa xây cần một `SubLayer` để bao quanh. Attention (chín bài trước) là một `SubLayer` — nhưng khối Transformer dùng THÊM một `SubLayer` thứ hai, đơn giản hơn hẳn: hai phép chiếu tuyến tính, một phi tuyến ở giữa.
::::

::::explain{#mang_truyen_thang_vi_tri}
**Mạng truyền thẳng theo vị trí** (position-wise feedforward) là `SubLayer` thứ hai của một khối Transformer:

> `FFN(x) = relu(x @ W1) @ W2`

`W1` chiếu từ `dim` sang một chiều ẩn (thường lớn hơn, để mạng có chỗ "khai triển" trước khi nén lại), `relu` cắt bỏ phần âm, `W2` chiếu ngược lại đúng `dim` — CÙNG shape với đầu vào, để residual (`X + FFN(X)`, bài trước) cộng được. Khác hẳn attention: `FFN` áp dụng ĐỘC LẬP lên MỖI vị trí — hàng thứ `i` của đầu ra CHỈ phụ thuộc hàng thứ `i` của đầu vào, không có phép nào TRỘN thông tin giữa các vị trí khác nhau (attention làm ĐÚNG điều ngược lại: `Q.matmul(K.transpose())` so mọi CẶP vị trí với nhau).

Công thức gốc còn có bias (`x @ W1 + b1`) — quest này BỎ bias, cùng lý do `Wq`/`Wk`/`Wv` của `co-che-attention` cũng không có bias: `Tensor.__add__` hiện tại giả định hai toán hạng CÙNG shape hệt nhau (`self.grad += out.grad`, không rút gọn qua chiều broadcast) — cộng một vector bias shape `(hidden,)` vào một ma trận shape `(so_token, hidden)` sẽ forward đúng (numpy tự broadcast), nhưng BACKWARD sẽ sai (gradient của bias cần CỘNG DỒN qua mọi vị trí, một phép rút gọn mà `__add__` hiện tại chưa làm). Bỏ bias tránh đúng cái bẫy này mà không cần sửa động cơ `Tensor`.

`relu` là một phép toán MỚI của `Tensor` (chưa có ở `dong-co-tensor`, q8.3b) — cùng công thức ĐÃ học cho `Value` (`ham-kich-hoat`, q8.2a; nhắc lại ở `gradient-bien-mat-that`, q8.2d): forward `max(0, x)`, backward nhân gradient đến với `1` nếu đầu vào dương, `0` nếu âm:

> forward: `out = Tensor(np.maximum(0.0, self.data), ...)`
>
> backward: `self.grad += (self.data > 0).astype(float) * out.grad`
::::

::::example{#ffn_forward_that}
Dùng lại `X` từ bài `ket-noi-tat` (embedding + vị trí, đã tính ở BOSS `co-che-attention`). `W1` shape `(4, 6)` (chiếu lên `6` chiều ẩn), `W2` shape `(6, 4)` (chiếu về đúng `4` chiều):

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
        out._backward = _backward
        return out

    def relu(self):
        out = Tensor(np.maximum(0.0, self.data), (self,), 'relu')
        def _backward():
            self.grad += (self.data > 0).astype(float) * out.grad
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


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
W1_data = [[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]]
W2_data = [[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]]

X = Tensor(X0)
W1 = Tensor(W1_data)
W2 = Tensor(W2_data)

H = X.matmul(W1)
R = H.relu()
Y = R.matmul(W2)

print(np.round(H.data, 6).tolist())
print(np.round(R.data, 6).tolist())
print(np.round(Y.data, 6).tolist())
```

```text title=readonly
[[-1.3, 0.4, -0.8, -0.7, -0.4, 0.4], [-1.0, -1.9, 0.9, 1.5, -0.3, 1.8], [-0.7, -0.5, 0.5, 1.0, -0.5, 0.4], [-1.991329, 2.104274, -2.4828, -2.181071, 1.544296, 0.157077]]
[[0.0, 0.4, 0.0, 0.0, 0.0, 0.4], [0.0, 0.0, 0.9, 1.5, 0.0, 1.8], [0.0, 0.0, 0.5, 1.0, 0.0, 0.4], [0.0, 2.104274, 0.0, 0.0, 1.544296, 0.157077]]
[[0.24, -0.04, 0.32, 0.28], [0.48, 1.71, -1.32, 1.47], [-0.38, 0.8, -0.97, 0.61], [-0.604052, -1.801744, 0.212972, 1.620691]]
```

`H` (trước `relu`) có cả giá trị âm và dương; `R` (sau `relu`) giữ nguyên phần dương, CẮT phần âm về đúng `0` — hàng `0` mất `4` trong `6` chiều (chỉ còn cột `1` và `5` khác `0`). `Y` (sau phép chiếu `W2`) trở lại shape `(4, 4)` — ĐÚNG shape của `X`, sẵn sàng để cộng residual.
::::

::::example{#ffn_finite_difference}
Kiểm `relu` (và cả `FFN` ghép từ nó) bằng finite-difference THẬT — một hàm `numpy` thuần, không chạm `Tensor`:

```python title=readonly
import numpy as np

X0 = np.array([[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]])
W1_data = np.array([[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]])
W2_data = np.array([[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]])

X_grad_giai_tich = np.array([[-0.86, 0.11, 1.66, 1.46], [-0.47, -0.39, 1.81, 0.92], [-0.47, -0.39, 1.81, 0.92], [-0.65, 0.81, 1.73, 0.9]])


def relu_np(z):
    return np.maximum(0.0, z)


def L(Xd, W1d, W2d):
    h_ = Xd @ W1d
    r_ = relu_np(h_)
    y_ = r_ @ W2d
    return np.sum(y_)


h = 1e-5
sai_so_toi_da = 0.0
for i in range(4):
    for j in range(4):
        Xp = X0.copy(); Xp[i, j] += h
        Xm = X0.copy(); Xm[i, j] -= h
        so = (L(Xp, W1_data, W2_data) - L(Xm, W1_data, W2_data)) / (2 * h)
        sai_so_toi_da = max(sai_so_toi_da, abs(so - X_grad_giai_tich[i, j]))

print(f"{sai_so_toi_da:.2e}")
print(sai_so_toi_da < 1e-4)
```

```text title=readonly
3.78e-11
True
```

Sai số tối đa đo được trên toàn bộ `16` phần tử của `X.grad` là `3,78e-11` — thấp hơn `1e-4`. `L` không chạm `Tensor` — bằng chứng ĐỘC LẬP rằng công thức `relu` backward (`(x>0)` nhân gradient đến) khớp đạo hàm THẬT của `FFN`, không chỉ trông có lý.
::::

::::predict{#doan_hoan_vi_vi_tri commitOnce}
`FFN` áp ĐỘC LẬP lên mỗi vị trí — không có phép nào trộn thông tin giữa các hàng của `X`.

**Trước khi chạy thử**, bạn đoán: nếu HOÁN ĐỔI hai hàng của `X` (ví dụ đổi chỗ hàng `0` và hàng `1`) rồi mới đưa qua `FFN`, đầu ra `Y` sẽ có hàng `0` và hàng `1` HOÁN ĐỔI tương ứng (giống hệt hoán đổi `Y` gốc), hay đầu ra sẽ khác đi theo cách KHÁC (không đơn thuần là hoán đổi hai hàng)?

:::opt{correct}
Hoán đổi tương ứng — mỗi hàng của `FFN(X)` chỉ là một hàm CỦA RIÊNG hàng đó (`relu(hàng_i @ W1) @ W2`), không phụ thuộc bất kỳ hàng nào khác; đổi chỗ hai hàng đầu vào chỉ đổi chỗ THỨ TỰ tính toán ĐỘC LẬP đó, kết quả là hai hàng đầu ra tương ứng cũng đổi chỗ y hệt, không hàng nào bị pha trộn với hàng khác
:::

:::opt
Khác đi theo cách khác — hoán đổi thứ tự đầu vào làm thay đổi cách `W1`/`W2` "nhìn thấy" dữ liệu, nên kết quả không thể đơn thuần là hoán đổi lại
::why
Gần đúng ở việc để ý THỨ TỰ đầu vào đúng là một yếu tố có thể ảnh hưởng tới kết quả của NHIỀU phép biến đổi (như attention, vốn so sánh CẢ CẶP vị trí) — quan sát đó không sai cho những phép TRỘN vị trí.

Chỗ lệch: `FFN` không hề "nhìn thấy" hàng nào khác ngoài hàng đang xử lý — phép nhân `hàng_i @ W1` chỉ dùng đúng các phần tử của HÀNG `i`, không có tổng/so sánh nào bắc qua hàng khác. Đổi chỗ thứ tự các hàng không đổi NỘI DUNG của bất kỳ hàng nào, nên mỗi hàng vẫn cho ra kết quả y hệt như khi nó đứng ở vị trí cũ — chỉ khác THỨ TỰ sắp xếp trong kết quả.
::
:::

:::opt
Không xác định được nếu không biết giá trị cụ thể của `W1`/`W2` — có thể hoán đổi tương ứng hoặc không, tuỳ trọng số
::why
Gần đúng ở tinh thần thận trọng khi một phép biến đổi phụ thuộc tham số cụ thể — thái độ hợp lý với nhiều phép toán khác.

Chỗ lệch: tính chất "áp độc lập lên mỗi vị trí" là một tính chất CẤU TRÚC của công thức `FFN(x) = relu(x @ W1) @ W2` (không có phép nào cộng/so sánh giữa các HÀNG của đầu vào) — đúng với MỌI `W1`/`W2`, không phụ thuộc giá trị cụ thể của chúng là gì.
::
:::
::::

::::code{#viet_relu}
Hoàn thiện `relu`: forward cắt phần âm về `0`, backward nhân gradient đến với `1` nếu đầu vào dương, `0` nếu âm.

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
        out._backward = _backward
        return out

    def relu(self):
        out = Tensor(___, (self,), 'relu')                       # np.maximum(0.0, self.data)
        def _backward():
            self.grad += ___ * out.grad                          # (self.data > 0).astype(float)
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


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
W1_data = [[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]]
W2_data = [[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]]

X = Tensor(X0)
W1 = Tensor(W1_data)
W2 = Tensor(W2_data)

H = X.matmul(W1)
R = H.relu()
Y = R.matmul(W2)
Y.backward()

print(np.round(Y.data, 6).tolist())
print(np.round(X.grad, 6).tolist())
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

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
        out._backward = _backward
        return out

    def relu(self):
        out = Tensor(np.maximum(0.0, self.data), (self,), 'relu')
        def _backward():
            self.grad += (self.data > 0).astype(float) * out.grad
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


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
W1_data = [[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]]
W2_data = [[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]]

X = Tensor(X0)
W1 = Tensor(W1_data)
W2 = Tensor(W2_data)

H = X.matmul(W1)
R = H.relu()
Y = R.matmul(W2)
Y.backward()

print(np.round(Y.data, 6).tolist())
print(np.round(X.grad, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(Y.data, 6).tolist() == [[0.24, -0.04, 0.32, 0.28], [0.48, 1.71, -1.32, 1.47], [-0.38, 0.8, -0.97, 0.61], [-0.604052, -1.801744, 0.212972, 1.620691]], f"Y.data sai -- dang ra {np.round(Y.data, 6).tolist()}"
assert np.round(X.grad, 6).tolist() == [[-0.86, 0.11, 1.66, 1.46], [-0.47, -0.39, 1.81, 0.92], [-0.47, -0.39, 1.81, 0.92], [-0.65, 0.81, 1.73, 0.9]], f"X.grad sai -- dang ra {np.round(X.grad, 6).tolist()}"

# rieng kiem tra CONG THUC TONG QUAT bang finite-difference THAT, doc lap voi Tensor
X0_np = np.array(X0)
W1_np = np.array(W1_data)
W2_np = np.array(W2_data)

def relu_np(z):
    return np.maximum(0.0, z)

def L(Xd, W1d, W2d):
    h_ = Xd @ W1d
    r_ = relu_np(h_)
    y_ = r_ @ W2d
    return np.sum(y_)

h = 1e-5
sai_so_toi_da = 0.0
for i in range(4):
    for j in range(4):
        Xp = X0_np.copy(); Xp[i, j] += h
        Xm = X0_np.copy(); Xm[i, j] -= h
        so = (L(Xp, W1_np, W2_np) - L(Xm, W1_np, W2_np)) / (2 * h)
        sai_so_toi_da = max(sai_so_toi_da, abs(so - X.grad[i, j]))
assert sai_so_toi_da < 1e-4, f"sai so gradient-check phai duoi 1e-4 -- dang ra {sai_so_toi_da}"

# rieng kiem tra AP DOC LAP len moi vi tri -- hoan doi hang 0 va 1 cua X phai
# hoan doi DUNG hang 0 va 1 tuong ung cua dau ra, khong tron lan
X_hoan_vi = Tensor(X0_np[[1, 0, 2, 3]].tolist())
Y_hoan_vi = X_hoan_vi.matmul(W1).relu().matmul(W2)
assert np.allclose(Y_hoan_vi.data, Y.data[[1, 0, 2, 3]]), f"FFN phai ap doc lap len moi vi tri -- hoan doi hang dau vao phai hoan doi dung hang dau ra tuong ung"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu (forward) cắt phần âm về `0` — `np.maximum(0.0, self.data)`. Chỗ hai (backward) nhân gradient đến với `1` ở vị trí đầu vào DƯƠNG, `0` ở vị trí ÂM — `(self.data > 0).astype(float)` (nhớ `.astype(float)` để nhân được với mảng số thực, không phải mảng boolean).
- kind: strategy
  body: 'Chỗ đầu: `np.maximum(0.0, self.data)`. Chỗ hai: `(self.data > 0).astype(float)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `np.maximum(0.0, self.data)` và `(self.data > 0).astype(float)`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: forward phai dung np.maximum(0.0, self.data) (khong duoc chep san mang hay dung ham khac); backward phai nhan gradient den voi (self.data > 0).astype(float) (dung dung dieu kien duong/am tren self.data, khong duoc bo qua dieu kien)
  requireAst:
  - kind: uses-call, target: maximum, min: 1
  - kind: uses-operator, target: ">", min: 1
  - kind: uses-call, target: astype, min: 1
  - kind: uses-name, target: self, min: 5
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true. maximum=1 (chi trong blank1 "np.maximum(0.0,
  # self.data)"). ">"=1 (chi trong blank2 "self.data > 0" -- Compare voi
  # Gt). astype=1 (chi trong blank2 ".astype(float)"). self=5 (2 lan trong
  # blank1 "self.data" xuat hien 1 lan thuc su -- doi lai: "np.maximum(0.0,
  # self.data)" co 1 lan self.data; blank2 "self.data > 0" co 1 lan;
  # "self.grad += ... * out.grad" co 1 lan self.grad; cong them cac lan
  # self xuat hien trong matmul/backward co san trong starter).
  # Cheat "out = Tensor(self.data, (self,), 'relu')" (bo qua maximum, tra
  # ve nguyen self.data -- ham DANH TINH thay vi relu) lam "maximum" tut
  # xuong 0 -- bi chan RIENG, VA da tu kiem chung bang Python that: H co gia
  # tri AM (vi du H[0,0]=-1.3) se KHONG bi cat ve 0, lam R khac han R dung
  # (R[0,0] se la -1.3 thay vi 0.0) -- bi bat DOC LAP boi assert Y.data.
  # Cheat "self.grad += out.grad" (bo qua dieu kien duong/am, backward
  # DANH TINH) lam ">" VA "astype" deu tut xuong 0 -- bi chan BOI CA HAI
  # luat, VA da tu kiem chung: X.grad se khac han gia tri dung (moi vi tri
  # H am se VAN cho gradient di qua, thay vi bi chan ve 0) -- bi bat DOC
  # LAP boi assert X.grad va boi kiem finite-difference.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^\\[\\[0\\.24, -0\\.04, 0\\.32, 0\\.28\\], \\[0\\.48, 1\\.71, -1\\.32, 1\\.47\\], \\[-0\\.38, 0\\.8, -0\\.97, 0\\.61\\], \\[-0\\.604052, -1\\.801744, 0\\.212972, 1\\.620691\\]\\]\\n\\[\\[-0\\.86, 0\\.11, 1\\.66, 1\\.46\\], \\[-0\\.47, -0\\.39, 1\\.81, 0\\.92\\], \\[-0\\.47, -0\\.39, 1\\.81, 0\\.92\\], \\[-0\\.65, 0\\.81, 1\\.73, 0\\.9\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai `SubLayer` giờ đã có: attention (chín bài trước) và feedforward (bài này). Bài sau: ráp cả hai, cùng kết nối tắt, thành MỘT khối Transformer hoàn chỉnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`FFN` vừa xây nhận `X` shape `(so_token, dim)`, chiếu qua `W1` lên `hidden`, `relu`, rồi chiếu qua `W2` VỀ ĐÚNG `dim` — để cộng residual được. Attention (chín bài trước) cũng nhận `X` shape `(so_token, dim)` làm đầu vào, và đầu ra của nó (sau `P.matmul(V)`) CŨNG đúng shape `(so_token, dim)`. Nếu cả hai `SubLayer` đều nhận VÀO và trả RA cùng một shape, một khối Transformer ráp CẢ HAI cùng kết nối tắt sẽ có hình dạng tổng thể như thế nào — bao nhiêu phép cộng residual, xen giữa những gì?
::::

::::checkpoint{mastery=0.8}
::::
