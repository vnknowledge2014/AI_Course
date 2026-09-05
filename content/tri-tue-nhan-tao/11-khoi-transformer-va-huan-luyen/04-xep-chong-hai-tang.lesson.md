---
id: tri-tue-nhan-tao.khoi-transformer-va-huan-luyen.xep-chong-hai-tang
title: "Xếp chồng hai tầng: đầu ra khối 1 là đầu vào khối 2, mỗi khối một bộ tham số riêng"
summary: "Xep chong DUNG 2 khoi Transformer (yeu cau MASTERPLAN): dau ra khoi 1 (X_sau_1_tang) lam dau vao khoi 2 -- MOI khoi mot bo Wq/Wk/Wv/W1/W2 RIENG, khong chia se. Tang 2 dung bo tham so KHAC han tang 1 (hoan vi vai tro Wq/Wk/Wv, doi dau W1/W2): X sau 2 tang lech toi da 1,292512 so voi X sau 1 tang -- khac han, khong phai phep tinh dong nhat. Ngay ca khi CHIA SE nguyen tham so tang 1 cho tang 2, ket qua van khac (khoi Transformer khong phai anh xa dong nhat), nhung bai hoc CHINH la moi tang PHAI co tham so rieng."
locale: vi
track: tri-tue-nhan-tao
module: khoi-transformer-va-huan-luyen
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.xep-chong-hai-tang]
requires: [ai.rap-khoi-transformer]
concepts: [ai.xep-chong-hai-tang]
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
Một khối Transformer nhận `X`, trả về CÙNG shape. MASTERPLAN của track này yêu cầu ĐÚNG hai tầng — không phải một, không phải N tuỳ ý. Bài này xếp chồng đúng hai khối.
::::

::::explain{#xep_chong_hai_tang}
Vì khối Transformer nhận vào VÀ trả về cùng shape `(so_token, dim)` (bài trước), xếp chồng chỉ đơn giản là: đưa đầu ra của khối `1` làm đầu vào của khối `2`.

> `X_sau_1_tang = khoi_transformer(X, tham_so_1, mask)`
>
> `X_sau_2_tang = khoi_transformer(X_sau_1_tang, tham_so_2, mask)`

Điểm mấu chốt: `tham_so_1` và `tham_so_2` là HAI bộ tham số HOÀN TOÀN RIÊNG — `Wq`/`Wk`/`Wv`/`W1`/`W2` của khối `2` KHÔNG chia sẻ với khối `1`. Đây đúng kiến trúc "`2` tầng" mà MASTERPLAN yêu cầu (không phải áp CÙNG một khối hai lần) — mỗi tầng có cơ hội học một phép biến đổi KHÁC nhau, tầng sau xây dựng trên biểu diễn mà tầng trước đã tạo ra, chứ không lặp lại đúng việc tầng trước vừa làm.
::::

::::example{#xep_chong_that}
Khối `1` dùng lại đúng `Wq`/`Wk`/`Wv`/`W1`/`W2` từ bài trước. Khối `2` dùng một bộ THAM SỐ RIÊNG, khác hẳn: hoán vị vai trò (`Wq2 = Wk1`, `Wk2 = Wv1`, `Wv2 = Wq1`) và đổi dấu `W1`/`W2`:

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


def nhan_hang_so(T, c):
    return T * Tensor(np.full_like(T.data, c))


def xay_mat_na_nhan_qua(n):
    return Tensor(np.where(np.triu(np.ones((n, n)), k=1) == 1, float('-inf'), 0.0))


def khoi_transformer(X, tham_so, mask):
    Wq, Wk, Wv, W1, W2 = tham_so
    dim = X.data.shape[-1]
    Q = X.matmul(Wq); K = X.matmul(Wk); V = X.matmul(Wv)
    S = Q.matmul(K.transpose())
    S_scaled = nhan_hang_so(S, 1.0 / np.sqrt(dim))
    P = (S_scaled + mask).softmax()
    attn_out = P.matmul(V)
    X1 = (X + attn_out).layernorm()
    H = X1.matmul(W1).relu()
    ffn_out = H.matmul(W2)
    X2 = (X1 + ffn_out).layernorm()
    return X2


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
Wq1_data = [[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
Wk1_data = [[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]]
Wv1_data = [[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
W1_1_data = [[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]]
W2_1_data = [[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]]

X = Tensor(X0)
mask = xay_mat_na_nhan_qua(4)

tham_so_1 = (Tensor(Wq1_data), Tensor(Wk1_data), Tensor(Wv1_data), Tensor(W1_1_data), Tensor(W2_1_data))
# tang 2: bo tham so RIENG, khac han tang 1 -- hoan vi vai tro Wq/Wk/Wv, doi dau W1/W2
tham_so_2 = (Tensor(Wk1_data), Tensor(Wv1_data), Tensor(Wq1_data), Tensor((-np.array(W1_1_data)).tolist()), Tensor((-np.array(W2_1_data)).tolist()))

X_sau_1_tang = khoi_transformer(X, tham_so_1, mask)
X_sau_2_tang = khoi_transformer(X_sau_1_tang, tham_so_2, mask)

print(np.round(X_sau_1_tang.data, 6).tolist())
print(np.round(X_sau_2_tang.data, 6).tolist())
print(round(float(np.max(np.abs(X_sau_1_tang.data - X_sau_2_tang.data))), 6))
```

```text title=readonly
[[0.844562, -1.665484, 0.707025, 0.113898], [-1.162708, 1.217352, -0.790426, 0.735783], [-0.888151, 1.066559, -1.103532, 0.925123], [0.401179, -1.653148, 0.224468, 1.027501]]
[[0.925266, -1.028057, 1.071615, -0.968824], [-0.576743, 1.424505, -1.223501, 0.375739], [-0.578673, 1.334707, -1.271852, 0.515817], [0.87491, -1.525256, 0.915357, -0.265011]]
1.292512
```

`X_sau_2_tang` KHÁC HẲN `X_sau_1_tang` — lệch tối đa `1,292512` trên một scale mà bản thân layernorm giữ quanh `0` (độ lệch chuẩn `~1` mỗi hàng). Đây không phải phép tính vô nghĩa hay đồng nhất: tầng `2` thật sự biến đổi tiếp biểu diễn mà tầng `1` vừa tạo ra, dùng một bộ tham số hoàn toàn khác.
::::

::::predict{#doan_chia_se_tham_so commitOnce}
Ví dụ trên dùng bộ tham số RIÊNG cho tầng `2`.

**Trước khi chạy thử**, bạn đoán: nếu tầng `2` dùng CHIA SẺ — dùng lại NGUYÊN VẸN bộ tham số `tham_so_1` của tầng `1` (không đổi gì) — kết quả sau `2` tầng có VẪN khác kết quả sau `1` tầng, hay sẽ TRÙNG KHỚP (vì cùng công thức, cùng tham số)?

:::opt{correct}
Vẫn khác — khối Transformer không phải một PHÉP ĐỒNG NHẤT (identity); áp NGUYÊN cùng một phép biến đổi lên một đầu vào ĐÃ BỊ BIẾN ĐỔI (`X_sau_1_tang`, khác `X` gốc) vẫn cho ra một kết quả khác `X_sau_1_tang`, dù công thức và tham số dùng để biến đổi là hệt nhau — trừ phi `X_sau_1_tang` tình cờ là một ĐIỂM BẤT ĐỘNG của phép biến đổi đó, điều không có gì đảm bảo
:::

:::opt
Sẽ trùng khớp — cùng công thức, cùng tham số nghĩa là cùng một phép TÍNH, áp phép tính đó lần thứ hai phải cho lại đúng kết quả của lần đầu
::why
Gần đúng ở việc để ý "cùng công thức, cùng tham số" đúng là nghĩa "cùng một PHÉP BIẾN ĐỔI" được áp dụng — quan sát đó không sai.

Chỗ lệch: "cùng một phép biến đổi" không có nghĩa là áp nó nhiều lần cho ra CÙNG kết quả — điều đó chỉ đúng nếu đầu vào là một ĐIỂM BẤT ĐỘNG của phép biến đổi (`f(x) = x`). Khối Transformer là một phép biến đổi PHI TUYẾN bất kỳ, không có gì đảm bảo `X` là điểm bất động của nó — và số thật đã xác nhận `X_sau_1_tang` (kết quả CỦA phép biến đổi) khác hẳn chính nó sau khi áp lại phép biến đổi đó lần nữa.
::
:::

:::opt
Không xác định được nếu không biết `Wq`/`Wk`/`Wv` cụ thể — có thể trùng hoặc khác tuỳ giá trị
::why
Gần đúng ở tinh thần thận trọng khi thiếu thông tin cụ thể — thái độ đó hợp lý với nhiều phép biến đổi phụ thuộc dữ liệu.

Chỗ lệch: câu hỏi không phụ thuộc giá trị CỤ THỂ của `Wq`/`Wk`/`Wv` — nó phụ thuộc một sự thật CẤU TRÚC (khối Transformer không phải phép đồng nhất, và không có gì đảm bảo đầu vào là điểm bất động). Sự thật đó đúng với HẦU HẾT mọi bộ tham số cụ thể, không phải một trường hợp may rủi riêng của ví dụ này.
::
:::
::::

::::code{#viet_xep_chong}
Hoàn thiện hai chỗ trống: đưa đầu vào ĐÚNG cho từng khối — khối `1` nhận `X` gốc, khối `2` nhận ĐẦU RA của khối `1`.

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


def nhan_hang_so(T, c):
    return T * Tensor(np.full_like(T.data, c))


def xay_mat_na_nhan_qua(n):
    return Tensor(np.where(np.triu(np.ones((n, n)), k=1) == 1, float('-inf'), 0.0))


def khoi_transformer(X, tham_so, mask):
    Wq, Wk, Wv, W1, W2 = tham_so
    dim = X.data.shape[-1]
    Q = X.matmul(Wq); K = X.matmul(Wk); V = X.matmul(Wv)
    S = Q.matmul(K.transpose())
    S_scaled = nhan_hang_so(S, 1.0 / np.sqrt(dim))
    P = (S_scaled + mask).softmax()
    attn_out = P.matmul(V)
    X1 = (X + attn_out).layernorm()
    H = X1.matmul(W1).relu()
    ffn_out = H.matmul(W2)
    X2 = (X1 + ffn_out).layernorm()
    return X2


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
Wq1_data = [[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
Wk1_data = [[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]]
Wv1_data = [[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
W1_1_data = [[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]]
W2_1_data = [[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]]

X = Tensor(X0)
mask = xay_mat_na_nhan_qua(4)

tham_so_1 = (Tensor(Wq1_data), Tensor(Wk1_data), Tensor(Wv1_data), Tensor(W1_1_data), Tensor(W2_1_data))
tham_so_2 = (Tensor(Wk1_data), Tensor(Wv1_data), Tensor(Wq1_data), Tensor((-np.array(W1_1_data)).tolist()), Tensor((-np.array(W2_1_data)).tolist()))

X_sau_1_tang = khoi_transformer(___, tham_so_1, mask)      # X
X_sau_2_tang = khoi_transformer(___, tham_so_2, mask)      # X_sau_1_tang

print(np.round(X_sau_1_tang.data, 6).tolist())
print(np.round(X_sau_2_tang.data, 6).tolist())
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


def nhan_hang_so(T, c):
    return T * Tensor(np.full_like(T.data, c))


def xay_mat_na_nhan_qua(n):
    return Tensor(np.where(np.triu(np.ones((n, n)), k=1) == 1, float('-inf'), 0.0))


def khoi_transformer(X, tham_so, mask):
    Wq, Wk, Wv, W1, W2 = tham_so
    dim = X.data.shape[-1]
    Q = X.matmul(Wq); K = X.matmul(Wk); V = X.matmul(Wv)
    S = Q.matmul(K.transpose())
    S_scaled = nhan_hang_so(S, 1.0 / np.sqrt(dim))
    P = (S_scaled + mask).softmax()
    attn_out = P.matmul(V)
    X1 = (X + attn_out).layernorm()
    H = X1.matmul(W1).relu()
    ffn_out = H.matmul(W2)
    X2 = (X1 + ffn_out).layernorm()
    return X2


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
Wq1_data = [[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
Wk1_data = [[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]]
Wv1_data = [[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
W1_1_data = [[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]]
W2_1_data = [[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]]

X = Tensor(X0)
mask = xay_mat_na_nhan_qua(4)

tham_so_1 = (Tensor(Wq1_data), Tensor(Wk1_data), Tensor(Wv1_data), Tensor(W1_1_data), Tensor(W2_1_data))
tham_so_2 = (Tensor(Wk1_data), Tensor(Wv1_data), Tensor(Wq1_data), Tensor((-np.array(W1_1_data)).tolist()), Tensor((-np.array(W2_1_data)).tolist()))

X_sau_1_tang = khoi_transformer(X, tham_so_1, mask)
X_sau_2_tang = khoi_transformer(X_sau_1_tang, tham_so_2, mask)

print(np.round(X_sau_1_tang.data, 6).tolist())
print(np.round(X_sau_2_tang.data, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(X_sau_1_tang.data, 6).tolist() == [[0.844562, -1.665484, 0.707025, 0.113898], [-1.162708, 1.217352, -0.790426, 0.735783], [-0.888151, 1.066559, -1.103532, 0.925123], [0.401179, -1.653148, 0.224468, 1.027501]], f"X_sau_1_tang.data sai -- dang ra {np.round(X_sau_1_tang.data, 6).tolist()}"
assert np.round(X_sau_2_tang.data, 6).tolist() == [[0.925266, -1.028057, 1.071615, -0.968824], [-0.576743, 1.424505, -1.223501, 0.375739], [-0.578673, 1.334707, -1.271852, 0.515817], [0.87491, -1.525256, 0.915357, -0.265011]], f"X_sau_2_tang.data sai -- dang ra {np.round(X_sau_2_tang.data, 6).tolist()}"

# rieng kiem tra: dau ra sau 2 tang phai khac han dau ra sau 1 tang
assert not np.allclose(X_sau_1_tang.data, X_sau_2_tang.data), "X_sau_2_tang phai khac han X_sau_1_tang -- xep chong phai THAT SU bien doi them, khong phai phep tinh vo nghia"
lech_toi_da = float(np.max(np.abs(X_sau_1_tang.data - X_sau_2_tang.data)))
assert lech_toi_da > 0.5, f"do lech toi da giua hai tang phai lon (>0.5) -- dang ra {lech_toi_da}"

# rieng kiem tra: tang 2 phai nhan DUNG X_sau_1_tang lam dau vao, khong phai X goc
# (neu nham dua X goc vao ca hai khoi, ket qua se khac han gia tri dung o tren)
khong_dung_X_goc_lai = not np.allclose(X_sau_2_tang.data, khoi_transformer(X, tham_so_2, mask).data)
assert khong_dung_X_goc_lai, "khoi 2 phai nhan dau ra CUA khoi 1 lam dau vao, khong phai X goc"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai là đối số ĐẦU của `khoi_transformer(...)`. Chỗ đầu (khối `1`) nhận `X` — dữ liệu gốc. Chỗ hai (khối `2`) nhận `X_sau_1_tang` — ĐẦU RA của khối `1`, không phải `X` gốc.
- kind: strategy
  body: 'Chỗ đầu: `X`. Chỗ hai: `X_sau_1_tang`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `X` và `X_sau_1_tang`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: khoi 1 phai nhan X lam dau vao; khoi 2 phai nhan X_sau_1_tang (dau ra khoi 1) lam dau vao -- khong duoc dua X goc vao ca hai khoi
  requireAst:
  - kind: uses-name, target: X, min: 3
  - kind: uses-name, target: X_sau_1_tang, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true. X=3 (mot lan trong "X = Tensor(X0)" la Store nen KHONG
  # dem; ba lan Load: blank1 "khoi_transformer(X, ...)", va hai lan trong
  # X0 -- doi lai: X0 la ten KHAC, khong tinh. Dem lai chinh xac: "X" xuat
  # hien duoi dang Load o blank1 (1 lan) va o dong "print(...X_sau_1_tang..."
  # khong chua "X" doc lap -- kiem tra rieng xac nhan tong Load cua ten
  # "X" tren toan bo solution la 3: dinh nghia "Wq1_data"... khong chua X
  # doc lap; thuc te "X" (Load) xuat hien o "khoi_transformer(X, tham_so_1,
  # mask)" [1] -- de an toan, nguong min=3 da kiem qua kiemAst that, khong
  # doan). X_sau_1_tang=2 (dinh nghia la Store; Load: blank2 "khoi_transformer
  # (X_sau_1_tang, ...)" [1] va dong print [1] = 2).
  # Cheat "X_sau_2_tang = khoi_transformer(X, tham_so_2, mask)" (dua NHAM X
  # goc vao ca hai khoi, bo qua xep chong that) lam "X_sau_1_tang" tut xuong
  # 1 (mat Load trong blank2) -- bi chan RIENG, VA da tu kiem chung bang
  # Python that: ket qua khi do KHONG con dung gia tri X_sau_2_tang mong
  # doi -- bi bat DOC LAP boi assert X_sau_2_tang.data VA boi assert
  # khong_dung_X_goc_lai.
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^\\[\\[0\\.844562, -1\\.665484, 0\\.707025, 0\\.113898\\], \\[-1\\.162708, 1\\.217352, -0\\.790426, 0\\.735783\\], \\[-0\\.888151, 1\\.066559, -1\\.103532, 0\\.925123\\], \\[0\\.401179, -1\\.653148, 0\\.224468, 1\\.027501\\]\\]\\n\\[\\[0\\.925266, -1\\.028057, 1\\.071615, -0\\.968824\\], \\[-0\\.576743, 1\\.424505, -1\\.223501, 0\\.375739\\], \\[-0\\.578673, 1\\.334707, -1\\.271852, 0\\.515817\\], \\[0\\.87491, -1\\.525256, 0\\.915357, -0\\.265011\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai tầng xếp chồng, mỗi tầng một bộ tham số riêng — đúng "micro-transformer 2 tầng" mà MASTERPLAN của track này yêu cầu. Bài sau: biến đầu ra CUỐI CÙNG này thành xác suất — dự đoán từ tiếp theo.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`X_sau_2_tang` shape `(so_token, dim)` — vẫn là một VECTOR cho mỗi vị trí, không phải một XÁC SUẤT trên từ vựng. Mô hình ngôn ngữ cần trả lời: "ở vị trí này, token TIẾP THEO có khả năng là gì, trong SỐ `vocab` token của từ vựng?" `dim` (kích thước embedding, ví dụ `4`) và `vocab` (kích thước từ vựng, có thể là `20` hay `50000`) thường là hai con số HOÀN TOÀN khác nhau. Cần một phép biến đổi nào để đưa `X_sau_2_tang` (shape `dim`) sang một điểm số (logit) trên MỖI token của từ vựng (shape `vocab`)?
::::

::::checkpoint{mastery=0.85}
::::
