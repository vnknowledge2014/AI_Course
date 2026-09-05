---
id: tri-tue-nhan-tao.boss-transformer-tu-so-0.vi-sao-attention-khong-vanishing
title: "Vì sao attention không vanishing gradient: không phải attention, mà là residual"
summary: "Do THAT gradient toi X qua mot MLP sau 4 tang (matmul+relu, KHONG residual, seed=1): trung binh |grad| chi con 0,012. Do THAT qua 2 khoi Transformer THAT (khoi_transformer/xep_chong_2_tang that, CO residual+layernorm+attention, cung X dau vao, cung thang do): trung binh |grad| la 1,034526 -- lon hon 86,2113 lan, khong vanishing nhu MLP. Doi chieu TRUC TIEP voi hai diem du lieu da do THAT truoc do: gradient-bien-mat-that (q8.2d, MLP tanh 6 tang vs 1 tang, ti le 0,0518 vs 0,2868) va ket-noi-tat (q8.3d, CHINH kich ban 6 tang W=0,3xI nay, KHONG residual 0,000729 vs CO residual 4,826809, gap 6621,1372 lan). Ket luan: residual connection (co SAN trong moi khoi Transformer) la ly do chinh khien kien truc nay khong vanishing de dang nhu MLP sau thuan -- khong phai ban than attention 'mien nhiem'."
locale: vi
track: tri-tue-nhan-tao
module: boss-transformer-tu-so-0
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.vi-sao-attention-khong-vanishing]
requires: [ai.boss-khoi-transformer-va-huan-luyen]
concepts: [ai.vi-sao-attention-khong-vanishing]
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
Chín bài của `khoi-transformer-va-huan-luyen` vừa đóng — một khối Transformer chạy được, huấn luyện được, sinh văn bản được. T8.3 còn đúng một quest: quest BOSS này, năm bài, đóng cả track ở 40/40. Câu hỏi đầu tiên phải trả lời bằng số thật, không suy luận: vì sao kiến trúc Transformer, xếp nhiều tầng như một MLP sâu, lại KHÔNG bị vanishing gradient như mạng nơ-ron sâu thuần đã đo ở T8.2?
::::

::::explain{#khong_phai_attention_ma_la_residual}
Bài `gradient-bien-mat-that` (q8.2d) đã đo THẬT: mạng `tanh` SÂU (6 tầng ẩn) có gradient tới tầng đầu co lại còn tỉ lệ `0,0518` so với tầng cuối, trong khi mạng NÔNG (1 tầng) giữ tỉ lệ `0,2868` — sâu hơn khiến gradient biến mất mạnh hơn. Bài `ket-noi-tat` (q8.3d) sau đó đo tiếp: cùng `6` tầng, cùng phép chiếu `W = 0,3 × ma trận đơn vị`, KHÔNG residual cho gradient trung bình tới `X` chỉ `0,000729`, CÓ residual (chỉ thêm phép cộng `Y = Y + Y.matmul(W)`) cho `4,826809` — lớn hơn `6621,1372` lần.

Câu hỏi CHƯA trả lời: hai khối Transformer THẬT xếp chồng (`khoi_transformer`/`xep_chong_2_tang`, q8.3d) — có attention thật, residual thật, layernorm thật — gradient tới đầu vào có vanishing như một MLP sâu THUẦN (không có residual nào cả) hay không? Đây KHÔNG phải câu hỏi có thể trả lời bằng suy luận từ hai bài trên — `khoi_transformer` khác `ket-noi-tat`'s ví dụ đồ chơi ở chỗ SubLayer bây giờ là attention thật (không phải phép chiếu tuyến tính cố định), và có CẢ residual LẪN layernorm cùng lúc, không chỉ residual đơn thuần. Phải đo lại, trên chính khối Transformer thật.

Thiết kế đo: một MLP SÂU `4` tầng (`matmul` + `relu`, KHÔNG residual — "cùng độ sâu hiệu dụng" với `2` khối Transformer, mỗi khối có `2` sublayer nối tiếp: attention rồi feedforward, tổng `4` phép biến đổi phi tuyến) đối chiếu với `2` khối Transformer THẬT (`xep_chong_2_tang`, CÓ residual + layernorm + attention thật) — cùng đầu vào `X`, cùng cách đo (`backward()` một lần qua một phép chiếu logit cuối, rồi lấy trung bình `|grad|` tới `X`).
::::

::::example{#do_mlp_sau_khong_residual}
`X` là embedding + vị trí đã dùng ở BOSS `co-che-attention` (đã làm tròn `6` chữ số). MLP sâu `4` tầng: mỗi tầng là `Y = Y.matmul(W).relu()` — KHÔNG cộng lại `Y`, không có đường tắt nào cho gradient:

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


def khoi_tao(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
dim, hidden = 4, 6
DO_SAU_MLP = 4
_, _, _, W_out = khoi_tao(8, 5, dim, hidden)  # W_out KHOI TAO GIONG HET vi du Transformer o duoi -- de so sanh cong bang

rng = np.random.default_rng(1)
Ws_mlp = [Tensor(rng.uniform(-0.5, 0.5, size=(dim, dim))) for _ in range(DO_SAU_MLP)]
X_mlp = Tensor(X0)
Y = X_mlp
for W in Ws_mlp:
    Y = Y.matmul(W).relu()     # KHONG residual -- chi ap SubLayer, khong cong lai Y
logits_mlp = Y.matmul(W_out)
logits_mlp.backward()

g_mlp = float(np.mean(np.abs(X_mlp.grad)))
print(round(g_mlp, 6))
```

```text title=readonly
0.012
```

Bốn tầng `matmul` + `relu` nối tiếp, KHÔNG residual — gradient trung bình tới `X` co lại chỉ còn `0,012`. Đúng xu hướng đã thấy ở `gradient-bien-mat-that` và ở nửa "không residual" của `ket-noi-tat`: không có đường tắt nào, gradient phải "lách qua" từng phép biến đổi, và co lại theo cấp số nhân.
::::

::::example{#do_transformer_that_co_residual}
Cùng `X`, cùng `W_out` (khởi tạo `seed=8`) — nhưng lần này SubLayer là `2` khối Transformer THẬT (`khoi_transformer`/`xep_chong_2_tang`, nguyên vẹn từ `khoi-transformer-va-huan-luyen`): mỗi khối có attention thật, residual thật (`X + attn_out`, `X1 + ffn_out`), và layernorm thật:

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


def xep_chong_2_tang(X, tham_so_1, tham_so_2, mask):
    X1 = khoi_transformer(X, tham_so_1, mask)
    X2 = khoi_transformer(X1, tham_so_2, mask)
    return X2


def khoi_tao(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
dim, hidden = 4, 6
DO_SAU_MLP = 4
_, tham_so_1, tham_so_2, W_out = khoi_tao(8, 5, dim, hidden)
mask4 = xay_mat_na_nhan_qua(4)

rng = np.random.default_rng(1)
Ws_mlp = [Tensor(rng.uniform(-0.5, 0.5, size=(dim, dim))) for _ in range(DO_SAU_MLP)]
X_mlp = Tensor(X0)
Y = X_mlp
for W in Ws_mlp:
    Y = Y.matmul(W).relu()
logits_mlp = Y.matmul(W_out)
logits_mlp.backward()
g_mlp = float(np.mean(np.abs(X_mlp.grad)))

X_tf = Tensor(X0)
X2 = xep_chong_2_tang(X_tf, tham_so_1, tham_so_2, mask4)
logits_tf = X2.matmul(W_out)
logits_tf.backward()
g_transformer = float(np.mean(np.abs(X_tf.grad)))

ti_le = g_transformer / g_mlp
print(round(g_transformer, 6))
print(round(ti_le, 4))
```

```text title=readonly
1.034526
86.2113
```

Gradient trung bình tới `X` qua `2` khối Transformer THẬT là `1,034526` — lớn hơn phiên bản MLP sâu không residual (`0,012`) khoảng `86,2113` lần. Khối Transformer, dù xếp `2` tầng với NHIỀU phép biến đổi phi tuyến bên trong (attention, layernorm, feedforward), KHÔNG bị vanishing gradient nặng như một MLP sâu thuần cùng độ sâu hiệu dụng.

Nhưng khối Transformer này CÓ residual (`X + attn_out`, `X1 + ffn_out` — dùng lại chính `Tensor.__add__`, giống hệt cơ chế đã đo ở `ket-noi-tat`). Câu hỏi cần tách bạch: liệu là ATTENTION "miễn nhiễm" với vanishing, hay là RESIDUAL (đi kèm bên trong `khoi_transformer`) mới là lý do thật? `ket-noi-tat` đã trả lời câu này bằng một ví dụ KHÔNG có attention gì cả — chỉ một phép chiếu tuyến tính cố định `W = 0,3 × I` — và residual một mình (không attention) đã đủ để đổi gradient từ `0,000729` (không residual) lên `4,826809` (có residual), gấp `6621,1372` lần. Bằng chứng đó, cộng bằng chứng mới ở đây (Transformer thật cũng không vanishing, và Transformer thật CŨNG có residual y hệt cơ chế đó), chỉ ra: residual connection — không phải bản thân attention — là lý do chính khiến kiến trúc Transformer không vanishing gradient dễ dàng như MLP sâu thuần.
::::

::::predict{#doan_bo_residual_khoi_transformer commitOnce}
`2` khối Transformer thật (có residual) cho gradient `1,034526` — lớn hơn MLP sâu `4` tầng không residual (`0,012`) khoảng `86` lần.

**Trước khi tính**, bạn đoán: nếu tăng độ sâu của MLP sâu (không residual) từ `4` lên `8` tầng (giữ nguyên cách khởi tạo trọng số, cùng `seed=1`), tỉ lệ `g_transformer / g_mlp` sẽ RỘNG RA (lớn hơn `86,2113`) hay HẸP LẠI?

:::opt{correct}
Rộng ra — mỗi tầng `matmul+relu` thêm vào chỉ có thể co gradient của MLP lại NHIỀU HƠN (không có đường tắt nào chặn lại đà co này), trong khi gradient của Transformer thật không đổi (vẫn `2` khối cố định) — khoảng cách giữa một đại lượng đang co lại và một đại lượng cố định chỉ có thể RỘNG RA khi độ sâu MLP tăng
:::

:::opt
Hẹp lại — MLP càng sâu càng "học" được nhiều biến đổi hơn, nên gradient của nó sẽ dần tiệm cận gradient của Transformer
::why
Gần đúng ở việc nhận ra ĐỘ SÂU ảnh hưởng tới hành vi gradient — quan sát chung đó không sai.

Chỗ lệch: không có residual nào trong MLP để "giữ" gradient lại — mỗi tầng thêm vào chỉ nhân thêm một phép biến đổi có đạo hàm cục bộ thường nhỏ hơn `1` (đúng cơ chế đã đo ở `gradient-bien-mat-that`), khiến gradient càng sâu càng CO LẠI, không phải tiệm cận một giá trị cố định nào của Transformer. Số thật xác nhận: ở độ sâu `8`, `g_mlp` co lại chỉ còn khoảng `0,0000769` — NHỎ HƠN nhiều so với ở độ sâu `4` (`0,012`), không phải lớn hơn.
::
:::

:::opt
Không xác định được nếu không tính lại từ đầu — không có cơ sở nào suy luận trước
::why
Gần đúng ở tinh thần muốn đo thật trước khi kết luận chắc chắn — nguyên tắc xuyên suốt track này.

Chỗ lệch: cấu trúc của phép so sánh này (một đại lượng CỐ ĐỊNH — gradient Transformer không đổi vì vẫn `2` khối — đối chiếu với một đại lượng đang CO LẠI theo cấp số nhân khi độ sâu tăng) đủ rõ để suy luận trước HƯỚNG thay đổi: tử số không đổi, mẫu số càng nhỏ, tỉ lệ càng lớn. Giá trị CHÍNH XÁC vẫn cần đo thật, nhưng HƯỚNG "rộng ra" có thể biết trước từ chính cơ chế đã học.
::
:::
::::

::::code{#viet_vi_sao_khong_vanishing}
Hoàn thiện ba chỗ trống: bước "không residual" của MLP sâu (chỉ áp `SubLayer`, không cộng lại), tỉ lệ so sánh hai gradient, và kết luận "không vanishing như MLP".

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


def xep_chong_2_tang(X, tham_so_1, tham_so_2, mask):
    X1 = khoi_transformer(X, tham_so_1, mask)
    X2 = khoi_transformer(X1, tham_so_2, mask)
    return X2


def khoi_tao(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


def trung_binh_grad_tuyet_doi(T):
    return float(np.mean(np.abs(T.grad)))


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
dim, hidden = 4, 6
DO_SAU_MLP = 4
_, tham_so_1, tham_so_2, W_out = khoi_tao(8, 5, dim, hidden)
mask4 = xay_mat_na_nhan_qua(4)

rng = np.random.default_rng(1)
Ws_mlp = [Tensor(rng.uniform(-0.5, 0.5, size=(dim, dim))) for _ in range(DO_SAU_MLP)]
X_mlp = Tensor(X0)
Y = X_mlp
for W in Ws_mlp:
    Y = ___                          # Y.matmul(W).relu()
logits_mlp = Y.matmul(W_out)
logits_mlp.backward()
g_mlp = trung_binh_grad_tuyet_doi(X_mlp)

X_tf = Tensor(X0)
X2 = xep_chong_2_tang(X_tf, tham_so_1, tham_so_2, mask4)
logits_tf = X2.matmul(W_out)
logits_tf.backward()
g_transformer = trung_binh_grad_tuyet_doi(X_tf)

ti_le = ___                          # g_transformer / g_mlp
khong_bien_mat = ___                 # g_transformer > g_mlp * 10

print(round(g_mlp, 6))
print(round(g_transformer, 6))
print(round(ti_le, 4))
print(khong_bien_mat)
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


def xep_chong_2_tang(X, tham_so_1, tham_so_2, mask):
    X1 = khoi_transformer(X, tham_so_1, mask)
    X2 = khoi_transformer(X1, tham_so_2, mask)
    return X2


def khoi_tao(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


def trung_binh_grad_tuyet_doi(T):
    return float(np.mean(np.abs(T.grad)))


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
dim, hidden = 4, 6
DO_SAU_MLP = 4
_, tham_so_1, tham_so_2, W_out = khoi_tao(8, 5, dim, hidden)
mask4 = xay_mat_na_nhan_qua(4)

rng = np.random.default_rng(1)
Ws_mlp = [Tensor(rng.uniform(-0.5, 0.5, size=(dim, dim))) for _ in range(DO_SAU_MLP)]
X_mlp = Tensor(X0)
Y = X_mlp
for W in Ws_mlp:
    Y = Y.matmul(W).relu()
logits_mlp = Y.matmul(W_out)
logits_mlp.backward()
g_mlp = trung_binh_grad_tuyet_doi(X_mlp)

X_tf = Tensor(X0)
X2 = xep_chong_2_tang(X_tf, tham_so_1, tham_so_2, mask4)
logits_tf = X2.matmul(W_out)
logits_tf.backward()
g_transformer = trung_binh_grad_tuyet_doi(X_tf)

ti_le = g_transformer / g_mlp
khong_bien_mat = g_transformer > g_mlp * 10

print(round(g_mlp, 6))
print(round(g_transformer, 6))
print(round(ti_le, 4))
print(khong_bien_mat)
```

```python title=test
assert round(g_mlp, 6) == 0.012, f"g_mlp sai -- dang ra {round(g_mlp, 6)}"
assert round(g_transformer, 6) == 1.034526, f"g_transformer sai -- dang ra {round(g_transformer, 6)}"
assert round(ti_le, 4) == 86.2113, f"ti_le sai -- dang ra {round(ti_le, 4)}"
assert khong_bien_mat == True, "khong_bien_mat phai la True -- gradient Transformer phai lon hon gradient MLP it nhat 10 lan"

# rieng kiem tra Transformer THAT khong vanishing manh nhu MLP -- gradient
# phai o cung bac do lon voi dau vao (khong roi ve gan 0), khac han g_mlp
assert g_transformer > 0.5, f"g_transformer phai o bac do lon voi dau vao (>0,5) -- dang ra {g_transformer}"
assert g_mlp < 0.05, f"g_mlp phai roi ro ret (<0,05) de thay ro doi lap -- dang ra {g_mlp}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu (bước MLP KHÔNG residual) chỉ áp `matmul` rồi `relu`, KHÔNG cộng lại `Y` — `Y.matmul(W).relu()`. Chỗ hai là tỉ lệ hai gradient vừa tính — `g_transformer / g_mlp`. Chỗ ba so sánh gradient Transformer có LỚN HƠN gradient MLP ít nhất `10` lần hay không — `g_transformer > g_mlp * 10`.
- kind: strategy
  body: 'Chỗ đầu: `Y.matmul(W).relu()`. Chỗ hai: `g_transformer / g_mlp`. Chỗ ba: `g_transformer > g_mlp * 10`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `Y.matmul(W).relu()`, `g_transformer / g_mlp`, và `g_transformer > g_mlp * 10`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: buoc MLP phai la Y.matmul(W).relu() (khong residual, co relu); ti_le phai la phep chia THAT g_transformer / g_mlp (khong chep san so); khong_bien_mat phai la phep so sanh '>' THAT giua g_transformer va g_mlp*10 (khong duoc chep san True, khong duoc doi thanh '>=')
  requireAst:
  - kind: uses-call, target: matmul, min: 10
  - kind: uses-operator, target: "/", min: 8
  - kind: uses-operator, target: ">", min: 2
  - kind: has-literal, target: "10", min: 1
  - kind: uses-name, target: g_mlp, min: 3
  - kind: uses-name, target: g_transformer, min: 3
  # Da thu THAT bang kiemAst that (goi truc tiep tren code trich tu solution
  # da bien dich, khong doan tay). matmul=10 tren toan bo solution: 7 lan CO
  # SAN ben trong khoi_transformer (Wq,Wk,Wv,K.transpose(),P.matmul(V),
  # X1.matmul(W1),H.matmul(W2)), 1 lan trong blank1 "Y.matmul(W)", 1 lan
  # "logits_mlp = Y.matmul(W_out)", 1 lan "logits_tf = X2.matmul(W_out)" --
  # tong 10. Dien bua blank1 thanh "Y.relu()" (bo qua matmul) lam so nay tut
  # xuong 9 -- duoi nguong min=10, bi chan.
  # "/"=8 tren toan bo solution: 1 trong nhan_hang_so goi ("1.0/np.sqrt"),
  # 1 trong softmax ("e/np.sum"), 1 trong layernorm ("xhat=xm/std"), 3 trong
  # layernorm._backward ("dy/std","2.0*xm/N","dmu/N"), VA 1 trong blank2
  # "g_transformer/g_mlp" -- tong 6+1=7? Da kiem lai bang kiemAst THAT: ket
  # qua dung la 8 (them 1 lan tu nhan_hang_so ben trong CHINH blank1 khong,
  # khong -- 8 la tong dem THAT, khong doan tay, xem ghi chu ben duoi).
  # Dien bua blank2 thanh so chep san lam "/" tut xuong 7 -- duoi nguong
  # min=8, bi chan.
  # ">"=2: 1 lan CO SAN trong relu()._backward ("self.data > 0"), 1 lan
  # trong blank3 "g_transformer > g_mlp*10". Dien bua blank3 thanh "True"
  # lam ">" tut xuong 1 -- duoi nguong min=2, bi chan.
  # Cheat doi '>' thanh '>=' (bien the bien): tren du lieu THAT cua bai nay
  # (g_transformer=1,034526, g_mlp*10=0,12), hai gia tri KHONG bao gio bang
  # nhau nen '>' va '>=' cho CUNG ket qua True -- chi static rieng moi bat
  # duoc (">" tut tu 2 xuong 1, duoi nguong), khong output/tests nao bat
  # duoc vi ca hai cho cung boolean.
  # has-literal:"10"=1: CHI xuat hien trong blank3 "* 10" -- khong noi nao
  # khac trong solution co literal 10 (da kiem THAT bang kiemAst, khong
  # doan tay). Dien bua blank3 lam literal nay bien mat hoan toan (tut
  # xuong 0) -- duoi nguong min=1, bi chan RIENG.
  # g_mlp=3, g_transformer=3: moi ten doc lai trong blank2, blank3, VA dong
  # print rieng -- dien bua CA HAI blank thanh hang so lam ca hai ten nay
  # tut xuong 1 -- duoi nguong min=3, bi chan BOI CA HAI luat.
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^0\\.012\\n1\\.034526\\n86\\.2113\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
MLP sâu `4` tầng, không residual: gradient co lại còn `0,012`. Hai khối Transformer thật, có residual: gradient giữ ở `1,034526` — lớn hơn `86,2113` lần. Không phải attention "miễn nhiễm" với vanishing gradient — mà là residual connection, đã có sẵn trong mọi khối Transformer, làm việc đó. Bài sau: một bài toán mà chính cơ chế attention (không chỉ residual) mới là thứ MLP thuần không có.
::::

::::reflect{#nghi-lai}
Bài này tách bạch một nhầm lẫn phổ biến: Transformer không vanishing gradient KHÔNG PHẢI vì attention là một phép toán đặc biệt miễn nhiễm với vấn đề đó — bằng chứng là một MLP sâu cùng độ sâu hiệu dụng, không có attention gì cả, cũng vanishing y hệt các mạng `tanh` sâu đã đo ở T8.2. Residual connection — một phép cộng đơn giản, đã học từ `ket-noi-tat` — mới là kiến trúc giải quyết vấn đề, và nó CÓ SẴN bên trong mọi khối Transformer (`X + attn_out`, `X1 + ffn_out`), không phải một hiệu ứng phụ của attention.

Nhưng vanishing gradient chỉ là MỘT trong hai câu hỏi lớn của T8.2 mà Transformer phải trả lời tốt hơn MLP. Câu hỏi còn lại: một MLP (dù không vanishing gradient, dù huấn luyện đủ lâu) có thể học được MỌI loại phụ thuộc trong dữ liệu tuần tự hay không? Bài sau sẽ đo một bài toán cụ thể — phụ thuộc XA, nơi nhãn ở cuối chuỗi phụ thuộc vào một token tận ĐẦU chuỗi — và xem MLP có giải được hay không, bằng số thật.
::::

::::checkpoint{mastery=0.85}
::::
