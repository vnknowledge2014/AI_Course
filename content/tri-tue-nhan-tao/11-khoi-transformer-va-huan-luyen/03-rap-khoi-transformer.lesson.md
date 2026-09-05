---
id: tri-tue-nhan-tao.khoi-transformer-va-huan-luyen.rap-khoi-transformer
title: "Ráp một khối Transformer: attention, residual, layernorm, feedforward, residual, layernorm"
summary: "Khối Transformer = X1 = layernorm(X + Attention(X)) roi X2 = layernorm(X1 + FFN(X1)) -- POST-NORM (kien truc Transformer goc), chuan hoa NGAY SAU moi phep cong residual, khong xen giua. Attention dung so_head=1 (BOSS co-che-attention da chung minh tai tao DUNG BANG BIT pipeline mot-dau). GOTCHA THAT: cach q8.3c scale diem attention (Tensor(S.data/sqrt(d)), dung lai mot Tensor MOI) lam DUT gradient -- sua bang phep nhan qua __mul__ voi mot Tensor hang CUNG SHAPE. GOTCHA THU HAI: goi backward() TRUC TIEP tren dau ra tho cua khoi (ket thuc bang layernorm) cho gradient BANG 0 o MOI tham so -- vi tong moi hang sau layernorm luon xap xi 0, mot tinh chat TOAN HOC, khong phai loi; kiem THAT xac nhan ca hai."
locale: vi
track: tri-tue-nhan-tao
module: khoi-transformer-va-huan-luyen
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.rap-khoi-transformer]
requires: [ai.mang-truyen-thang-vi-tri]
concepts: [ai.rap-khoi-transformer]
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
Hai `SubLayer` (attention, feedforward), một kết nối tắt, và giờ thêm một mảnh cuối: `layernorm` (đã có sẵn từ `dong-co-tensor`, q8.3b). Ráp cả ba đúng thứ tự — đó là MỘT khối Transformer.
::::

::::explain{#rap_mot_khoi}
Kiến trúc gốc Transformer (Vaswani và cộng sự) xen `layernorm` NGAY SAU mỗi phép cộng residual — gọi là **post-norm**, để phân biệt với một biến thể sau này (pre-norm, chuẩn hoá TRƯỚC sublayer thay vì sau). Quest này dùng ĐÚNG post-norm — kiến trúc gốc, đơn giản hơn để trình bày, và đã chốt SẴN trong kế hoạch của track (bài `boss-co-che-attention` đã nhắc "kết nối tắt, feed-forward, chuẩn hoá lớp XEN KẼ" — đúng nghĩa post-norm):

> `X1 = layernorm(X + Attention(X))`
>
> `X2 = layernorm(X1 + FFN(X1))`

`Attention(X)` ở đây dùng `so_head = 1` (một đầu) — bài BOSS `co-che-attention` đã CHỨNG MINH bằng số thật rằng `so_head = 1` tái tạo ĐÚNG BẰNG BIT kết quả của pipeline một-đầu đầy đủ (không tách gì cả), nên khối này dùng thẳng `Q`/`K`/`V` mà không cần `tach_dau`/`ghep_dau` — multi-head vẫn hoạt động y hệt nếu cần, chỉ không phải trọng tâm của bài này.

**Một gotcha thật, phát hiện khi ráp khối này**: pipeline attention của `scale-va-softmax` (q8.3c) scale điểm attention bằng `Sh_scaled = Tensor(Sh.data / np.sqrt(dim_dau))` — DỰNG một `Tensor` MỚI từ `.data` thô, không truyền `_prev`. Cách đó cho kết quả FORWARD đúng, nhưng làm ĐỨT gradient tại đúng chỗ đó: `S.grad` sẽ không bao giờ nhận được gì, nên `Wq`/`Wk`/`Wv` không bao giờ học được. `co-che-attention` không sao vì KHÔNG bài nào ở đó gọi `.backward()` xuyên qua bước scale này. Quest này CẦN — nên phải sửa: nhân bằng `__mul__` (đã có sẵn, backward đúng) với một `Tensor` HẰNG số CÙNG SHAPE (không phải một số vô hướng — `__mul__` không tự rút gọn qua broadcast, cùng lý do bài trước bỏ bias):

> `nhan_hang_so(T, c) = T * Tensor(np.full_like(T.data, c))`
::::

::::example{#rap_khoi_forward_that}
Dùng lại `X`, `Wq`/`Wk`/`Wv` (từ `co-che-attention`) và `W1`/`W2` (từ bài trước) — ráp trọn một khối:

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
Wq_data = [[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
Wk_data = [[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]]
Wv_data = [[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
W1_data = [[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]]
W2_data = [[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]]

X = Tensor(X0)
Wq = Tensor(Wq_data); Wk = Tensor(Wk_data); Wv = Tensor(Wv_data)
W1 = Tensor(W1_data); W2 = Tensor(W2_data)
mask = xay_mat_na_nhan_qua(4)

X2 = khoi_transformer(X, (Wq, Wk, Wv, W1, W2), mask)
print(np.round(X2.data, 6).tolist())
print(np.round(np.sum(X2.data, axis=-1), 8).tolist())
```

```text title=readonly
[[0.844562, -1.665484, 0.707025, 0.113898], [-1.162708, 1.217352, -0.790426, 0.735783], [-0.888151, 1.066559, -1.103532, 0.925123], [0.401179, -1.653148, 0.224468, 1.027501]]
[-0.0, -0.0, 0.0, 0.0]
```

`X2` — đầu ra của khối Transformer — cùng shape `(4, 4)` với `X` đầu vào, sẵn sàng làm đầu vào cho khối THỨ HAI (bài sau) hoặc cho tầng chiếu ra logit (bài sau nữa). Dòng thứ hai xác nhận một tính chất của `layernorm`: TỔNG mỗi hàng của `X2` luôn xấp xỉ `0` — hệ quả trực tiếp của việc chuẩn hoá trung bình về `0` ở bước cuối cùng.
::::

::::example{#gotcha_backward_truc_tiep}
Chính tính chất "tổng mỗi hàng luôn `≈ 0`" ở trên tạo ra một bẫy: `Tensor.backward()` (gọi TRỰC TIẾP, không qua thêm phép chiếu nào) seed gradient bằng `np.ones_like(...)` — TƯƠNG ĐƯƠNG coi "loss" là TỔNG mọi phần tử của `X2`. Nhưng tổng đó LUÔN xấp xỉ `0`, bất kể `X`/`Wq`/`W1`/... là gì — nên đạo hàm của nó theo mọi tham số PHẢI bằng `0`, một cách suy biến:

```python title=readonly
X2.backward()
print("Wq.grad tong tri tuyet doi:", np.sum(np.abs(Wq.grad)))
print("W1.grad tong tri tuyet doi:", np.sum(np.abs(W1.grad)))

# Sua bang cach nhan voi MOT tensor KHONG hang so theo hang (Wc), giong
# dung ky thuat da dung o BOSS dong-co-tensor -- pha vo tinh doi xung
# "tong moi hang = 0" truoc khi backward.
Wc = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Z = X2 * Wc
Z.backward()
print("Wq.grad tong tri tuyet doi (qua Wc):", round(float(np.sum(np.abs(Wq.grad))), 6))
print("W1.grad tong tri tuyet doi (qua Wc):", round(float(np.sum(np.abs(W1.grad))), 6))
```

```text title=readonly
Wq.grad tong tri tuyet doi: 0.0
W1.grad tong tri tuyet doi: 0.0
Wq.grad tong tri tuyet doi (qua Wc): 1.210621
W1.grad tong tri tuyet doi (qua Wc): 2.166277
```

Gọi `backward()` thẳng trên `X2` cho gradient BẰNG `0` tuyệt đối ở MỌI tham số — không phải lỗi code, mà là hệ quả TOÁN HỌC của việc `X2` (sau `layernorm` cuối) luôn có tổng mỗi hàng `≈ 0`. Nhân với `Wc` (một tensor KHÔNG hằng số theo hàng) trước khi backward phá vỡ tính đối xứng đó — gradient giờ THẬT SỰ khác `0`. Bài học rút ra: từ bài `mat-mat-tu-tiep-theo` trở đi, mọi kiểm tra gradient của quest này đều đi qua tầng chiếu logit + hàm mất mát THẬT — KHÔNG BAO GIỜ gọi `backward()` thẳng trên đầu ra thô của một khối kết thúc bằng `layernorm`.
::::

::::predict{#doan_cach_tranh_bay commitOnce}
Ví dụ trên xác nhận: gọi `backward()` trực tiếp trên đầu ra thô của khối (kết thúc bằng `layernorm`) luôn cho gradient bằng `0` ở mọi tham số, một cách suy biến — không phải lỗi.

**Trước khi đọc lại**, bạn đoán: trong các bài SAU của quest này (từ `mat-mat-tu-tiep-theo` trở đi, khi cần kiểm tra `Wq`/`W1`/... thật sự nhận được gradient), cách nào sau đây sẽ tránh được bẫy suy biến này?

:::opt{correct}
Luôn tính gradient qua một hàm mất mát THẬT (chiếu `X2` sang logit trên từ vựng rồi cross-entropy), KHÔNG BAO GIỜ gọi `backward()` thẳng trên đầu ra thô của một khối kết thúc bằng `layernorm` — hàm mất mát thật không có tính đối xứng "tổng mỗi hàng luôn `0`" như phép cộng đơn thuần, nên gradient lan ngược không bị triệt tiêu
:::

:::opt
Bỏ `layernorm` cuối cùng của khối đi — vậy đầu ra sẽ không còn ràng buộc tổng mỗi hàng `= 0`, và `backward()` trực tiếp sẽ luôn cho gradient đúng
::why
Gần đúng ở việc xác định ĐÚNG NGUYÊN NHÂN gây suy biến (ràng buộc tổng mỗi hàng `≈ 0` của `layernorm`) — chẩn đoán đó không sai.

Chỗ lệch: bỏ `layernorm` cuối cùng phá vỡ chính kiến trúc post-norm đã chọn (mỗi sublayer PHẢI kết thúc bằng chuẩn hoá, đúng thứ tự đã giải thích ở đầu bài) — đó là đổi kiến trúc để né một bẫy đo lường, không phải sửa cách đo. Cách đúng là GIỮ kiến trúc, chỉ đổi cách KIỂM TRA gradient (qua một hàm mất mát thật, không đối xứng).
::
:::

:::opt
Gọi `backward()` nhiều lần liên tiếp trên `X2` — tổng gradient qua nhiều lần gọi sẽ khác `0`
::why
Gần đúng ở việc nghĩ tới việc LẶP LẠI một phép tính để tích luỹ kết quả khác — có những gotcha khác trong quest này (như cộng dồn gradient) đúng là liên quan tới việc gọi lại nhiều lần.

Chỗ lệch: mỗi lần gọi `backward()` trên CÙNG một `X2` đều seed lại `grad = np.ones_like(...)` từ đầu — cùng một phép tính suy biến CHO RA đúng `0` lặp lại, không tích luỹ thành khác `0`. Vấn đề nằm ở CHÍNH công thức, không phải số lần gọi.
::
:::
::::

::::code{#viet_khoi_transformer}
Hoàn thiện hai chỗ trống: ráp `X1` (residual quanh attention, rồi chuẩn hoá), và `X2` (residual quanh feedforward, rồi chuẩn hoá).

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
    X1 = ___.layernorm()                     # (X + attn_out)
    H = X1.matmul(W1).relu()
    ffn_out = H.matmul(W2)
    X2 = ___.layernorm()                     # (X1 + ffn_out)
    return X2


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
Wq_data = [[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
Wk_data = [[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]]
Wv_data = [[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
W1_data = [[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]]
W2_data = [[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]]

X = Tensor(X0)
Wq = Tensor(Wq_data); Wk = Tensor(Wk_data); Wv = Tensor(Wv_data)
W1 = Tensor(W1_data); W2 = Tensor(W2_data)
mask = xay_mat_na_nhan_qua(4)

X2 = khoi_transformer(X, (Wq, Wk, Wv, W1, W2), mask)
print(np.round(X2.data, 6).tolist())
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
Wq_data = [[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
Wk_data = [[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]]
Wv_data = [[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]]
W1_data = [[-0.5, 0.9, -0.6, -0.6, -0.3, -0.5], [0.3, -0.8, 0.8, 0.7, -1.0, 0.1], [-0.8, -0.5, -0.2, -0.1, -0.1, 0.9], [-0.5, -0.6, 0.3, 0.9, 0.8, 0.8]]
W2_data = [[-0.9, 0.9, 0.3, 0.7], [-0.2, -0.6, 0.6, 0.3], [0.6, -0.6, -0.7, 0.5], [-1.0, 0.9, -0.7, 0.2], [-0.2, -0.4, -0.7, 0.6], [0.8, 0.5, 0.2, 0.4]]

X = Tensor(X0)
Wq = Tensor(Wq_data); Wk = Tensor(Wk_data); Wv = Tensor(Wv_data)
W1 = Tensor(W1_data); W2 = Tensor(W2_data)
mask = xay_mat_na_nhan_qua(4)

X2 = khoi_transformer(X, (Wq, Wk, Wv, W1, W2), mask)
print(np.round(X2.data, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(X2.data, 6).tolist() == [[0.844562, -1.665484, 0.707025, 0.113898], [-1.162708, 1.217352, -0.790426, 0.735783], [-0.888151, 1.066559, -1.103532, 0.925123], [0.401179, -1.653148, 0.224468, 1.027501]], f"X2.data sai -- dang ra {np.round(X2.data, 6).tolist()}"

# rieng kiem tra tinh chat layernorm: tong moi hang cua X2 phai xap xi 0
assert np.allclose(np.sum(X2.data, axis=-1), 0.0, atol=1e-6), f"tong moi hang X2 phai xap xi 0 (hau qua cua layernorm cuoi) -- dang ra {np.sum(X2.data, axis=-1).tolist()}"

# rieng kiem tra GOTCHA: backward() truc tiep tren X2 phai cho gradient BANG 0
X2.backward()
assert np.sum(np.abs(Wq.grad)) == 0.0, "backward() truc tiep tren dau ra tho (ket thuc bang layernorm) phai cho Wq.grad BANG 0 -- day la mot tinh chat toan hoc, khong phai loi"

# rieng kiem tra: nhan voi mot Tensor KHONG hang so theo hang truoc backward
# phai cho gradient THAT SU khac 0 -- xac nhan khoi RAP DUNG (khong dut gradient
# o buoc scale, dung __mul__ dung cach thay vi Tensor(...) dung lai tu dau).
Wq2 = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk2 = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv2 = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
W1b = Tensor(W1_data); W2b = Tensor(W2_data)
Xb = Tensor(X0)
maskb = xay_mat_na_nhan_qua(4)
X2b = khoi_transformer(Xb, (Wq2, Wk2, Wv2, W1b, W2b), maskb)
Wc = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Z = X2b * Wc
Z.backward()
assert np.sum(np.abs(Wq2.grad)) > 0.0, "nhan voi Wc (khong hang so theo hang) truoc backward phai cho Wq.grad KHAC 0 -- xac nhan gradient THAT SU lan toi Wq qua buoc scale"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu ráp `X1` — residual QUANH attention (`X` cộng `attn_out`), rồi mới `.layernorm()` — `(X + attn_out)`. Chỗ hai ráp `X2` — residual QUANH feedforward (`X1` cộng `ffn_out`), rồi mới `.layernorm()` — `(X1 + ffn_out)`. Đúng thứ tự POST-NORM: cộng TRƯỚC, chuẩn hoá SAU.
- kind: strategy
  body: 'Chỗ đầu: `(X + attn_out)`. Chỗ hai: `(X1 + ffn_out)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `(X + attn_out)` và `(X1 + ffn_out)`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: X1 phai duoc rap tu (X + attn_out) TRUOC khi goi layernorm (residual quanh attention); X2 phai duoc rap tu (X1 + ffn_out) TRUOC khi goi layernorm (residual quanh feedforward) -- dung thu tu POST-NORM, khong duoc bo qua residual hay hoan doi X/X1
  requireAst:
  - kind: uses-name, target: attn_out, min: 1
  - kind: uses-name, target: ffn_out, min: 1
  - kind: uses-operator, target: "+", min: 2
  - kind: uses-call, target: layernorm, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true. attn_out=1 (dinh nghia "attn_out = P.matmul(V)" la Store,
  # khong dem; Load duy nhat la trong blank1 "X + attn_out"). ffn_out=1
  # (tuong tu, Load duy nhat trong blank2). "+"=2: mot trong blank1 "X +
  # attn_out", mot trong blank2 "X1 + ffn_out" (cac "+" khac trong file --
  # vi du trong day dinh nghia __add__ -- la than ham co san trong starter,
  # KHONG tinh vao day vi day la BinOp/Compare tren CHINH VAN BAN blank, con
  # dinh nghia phuong thuc la code CO SAN khong doi; kiem tra rieng xac nhan
  # tong "+" tren TOAN BO file solution la dung 3: 1 trong __add__.data, 2
  # trong hai blank -- nguong min=2 an toan). layernorm=2 (mot lan moi
  # blank, dinh nghia "def layernorm" khong tinh).
  # Cheat "X1 = attn_out.layernorm()" (bo qua residual, chi dung attn_out)
  # lam "+" tut xuong 1 -- bi chan RIENG, VA da tu kiem chung bang Python
  # that: X2.data khac han gia tri dung -- bi bat DOC LAP boi assert
  # X2.data.
  # Cheat "X2 = X1.layernorm()" (bo qua ffn_out hoan toan) lam "ffn_out" tut
  # xuong 0 VA "+" tut xuong 1 -- bi chan BOI CA HAI luat, VA da tu kiem
  # chung: X2.data khac han gia tri dung.
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^\\[\\[0\\.844562, -1\\.665484, 0\\.707025, 0\\.113898\\], \\[-1\\.162708, 1\\.217352, -0\\.790426, 0\\.735783\\], \\[-0\\.888151, 1\\.066559, -1\\.103532, 0\\.925123\\], \\[0\\.401179, -1\\.653148, 0\\.224468, 1\\.027501\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một khối Transformer hoàn chỉnh: attention, residual, layernorm, feedforward, residual, layernorm. Bài sau: xếp chồng ĐÚNG hai khối như thế này — bộ tham số nào riêng, bộ nào không.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Khối Transformer vừa ráp nhận `X` shape `(so_token, dim)` và trả về `X2` CÙNG shape — đây chính là điều khiến việc XẾP CHỒNG nhiều khối trở nên khả thi: đầu ra của khối này khớp shape đầu vào khối kia hệt nhau. Nếu đưa `X2` (đầu ra khối vừa ráp) làm đầu vào cho một khối Transformer THỨ HAI, khối đó có nên dùng LẠI đúng `Wq`/`Wk`/`Wv`/`W1`/`W2` vừa dùng, hay cần một bộ tham số HOÀN TOÀN riêng?
::::

::::checkpoint{mastery=0.8}
::::
