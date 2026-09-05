---
id: tri-tue-nhan-tao.khoi-transformer-va-huan-luyen.mat-mat-tu-tiep-theo
title: "Đầu ra & mất mát: chiếu sang từ vựng, dự đoán đúng token TIẾP THEO"
summary: "W_out (dim, vocab) chieu dau ra khoi Transformer cuoi sang logit tren TOAN BO tu vung, tai vi tri; nhan la CHINH chuoi token dich di mot vi tri (ids[1:]) -- dinh nghia cot loi cua mo hinh ngon ngu tu hoi quy. mat_mat_du_doan_tiep_theo() tai dung log-sum-exp on dinh so hoc (cross_entropy_qua_value, q8.2c) tren Tensor, giam bot dung hang CUOI (khong co 'tu tiep theo' that trong cua so ngan). Tren corpus BPE that (20 vocab, 10 token cua so): loss = 3,351352. Kiem THAT: hang cuoi cua logits.grad dung 0 tuyet doi (khong giam sat); Bang.grad va W_out.grad deu khac 0 -- gradient THAT SU lan toi ca embedding lan hai khoi Transformer, khong con la 0 suy bien nhu bai truoc."
locale: vi
track: tri-tue-nhan-tao
module: khoi-transformer-va-huan-luyen
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.mat-mat-tu-tiep-theo]
requires: [ai.xep-chong-hai-tang]
concepts: [ai.mat-mat-tu-tiep-theo]
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
Hai tầng Transformer xếp chồng, đầu ra shape `(so_token, dim)`. Nhưng một mô hình ngôn ngữ phải trả lời câu hỏi cụ thể: "token tiếp theo là gì, trong TOÀN BỘ từ vựng?" Cần thêm một phép chiếu, và một hàm mất mát.
::::

::::explain{#dau_ra_va_mat_mat}
**Tầng chiếu cuối**: nhân đầu ra khối Transformer thứ hai (`X_sau_2_tang`, shape `(so_token, dim)`) với `W_out` (shape `(dim, vocab)`) — một phép `matmul` NGUYÊN VẸN, không gì mới:

> `logits = X_sau_2_tang.matmul(W_out)` — shape `(so_token, vocab)`, MỖI vị trí có một điểm số cho MỖI token trong từ vựng.

**Dự đoán token tiếp theo**: ý tưởng cốt lõi của một mô hình ngôn ngữ TỰ HỒI QUY — ở vị trí `i`, mô hình phải đoán ĐÚNG token ở vị trí `i + 1` (không phải token ở CHÍNH vị trí `i`, cái đó nó đã thấy). Nhãn cho toàn chuỗi vì vậy là chính chuỗi token DỊCH đi một vị trí: `nhan = ids[1:]`. Vị trí CUỐI CÙNG của chuỗi không có "từ tiếp theo" thật để đối chiếu (không còn token nào sau nó trong cửa sổ đang xét) — nên hàm mất mát chỉ tính trên `n = so_token - 1` vị trí ĐẦU, bỏ qua vị trí cuối.

Công thức là CROSS-ENTROPY, tái dùng ĐÚNG kỹ thuật log-sum-exp ổn định số học đã học (`on-dinh-so-hoc`, T8.1c; `cross_entropy_qua_value`, q8.2c) — giờ áp trên `Tensor` (mỗi hàng một phân phối, `Tensor.softmax()` đã có công thức ổn định này sẵn):

> `L = -mean(log(softmax(logits)[i, nhan[i]]))`, trung bình trên `n` vị trí có nhãn.

Backward của cặp "softmax + cross-entropy" gộp lại thành một công thức NỔI TIẾNG, đơn giản hơn hẳn đi qua từng bước riêng: gradient tại logit ĐÚNG trừ đi `1`, chia đều cho `n`:

> `dlogits[i, nhan[i]] -= 1`, rồi chia cho `n` — CHÍNH XÁC công thức `softmaxᵢ − 1{i=y}` đã gặp ở `cross_entropy_qua_value` (q8.2c), giờ áp trên cả một MẢNG thay vì từng vô hướng. Hàng CUỐI (không có nhãn) không xuất hiện trong `dlogits` — gradient của nó giữ nguyên `0`, không nhận được tín hiệu học nào từ hàm mất mát này.
::::

::::example{#loss_that_tren_corpus}
Dùng bộ tham số đã ráp (`Bang`, hai khối Transformer, `W_out`) trên một CỬA SỔ `10` token đầu của corpus BPE (`may hoc hoc tu d`):

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

    def embedding_lookup(self, ids):
        out = Tensor(self.data[ids], (self,), 'embedding_lookup')
        def _backward():
            for vi_tri, tid in enumerate(ids):
                self.grad[tid] += out.grad[vi_tri]
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


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = np.sin(goc[:, 0::2])
    pe[:, 1::2] = np.cos(goc[:, 1::2])
    return pe


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


def dem_cap_lien_ke(danh_sach):
    dem = {}
    for i in range(len(danh_sach) - 1):
        cap = (danh_sach[i], danh_sach[i + 1])
        dem[cap] = dem.get(cap, 0) + 1
    return dem


def gop_cap(danh_sach, cap):
    a, b = cap
    token_moi = a + b
    ra = []
    i = 0
    while i < len(danh_sach):
        if i < len(danh_sach) - 1 and danh_sach[i] == a and danh_sach[i + 1] == b:
            ra.append(token_moi)
            i += 2
        else:
            ra.append(danh_sach[i])
            i += 1
    return ra


def huan_luyen_bpe(corpus, muc_tieu_vocab):
    ds = list(corpus)
    vocab = set(ds)
    merges = []
    while len(vocab) < muc_tieu_vocab:
        dem = dem_cap_lien_ke(ds)
        if not dem:
            break
        cap_pho_bien = max(dem, key=dem.get)
        tan_suat = dem[cap_pho_bien]
        if tan_suat < 2:
            break
        ds = gop_cap(ds, cap_pho_bien)
        vocab.add(cap_pho_bien[0] + cap_pho_bien[1])
        merges.append(cap_pho_bien)
    return ds, merges, vocab


def xay_token_sang_id(vocab):
    return {t: i for i, t in enumerate(sorted(vocab))}


def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]


def mat_mat_du_doan_tiep_theo(logits, ids):
    nhan = ids[1:]
    n = len(nhan)
    z = logits.data[:n]
    z_shift = z - np.max(z, axis=-1, keepdims=True)
    e = np.exp(z_shift)
    p = e / np.sum(e, axis=-1, keepdims=True)
    log_p_dung = np.log(p[np.arange(n), nhan])
    loss = -np.mean(log_p_dung)
    out = Tensor(loss, (logits,), 'mat_mat_du_doan_tiep_theo')
    def _backward():
        dlogits = np.zeros_like(logits.data)
        dz = p.copy()
        dz[np.arange(n), nhan] -= 1.0
        dz /= n
        dlogits[:n] = dz
        logits.grad += dlogits * out.grad
    out._backward = _backward
    return out


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)

dim, hidden, so_token = 4, 6, 10
vocab_size = len(vocab)
ids = ids_full[:so_token]

rng = np.random.default_rng(8)
def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
Bang = W((vocab_size, dim))
tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
W_out = W((dim, vocab_size))

PE = Tensor(ma_hoa_vi_tri(so_token, dim))
X = Bang.embedding_lookup(ids) + PE
mask = xay_mat_na_nhan_qua(so_token)
X_sau_2_tang = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
logits = X_sau_2_tang.matmul(W_out)

loss = mat_mat_du_doan_tiep_theo(logits, ids)
print(round(float(loss.data), 6))
print(logits.data.shape)
```

```text title=readonly
3.351352
(10, 20)
```

`vocab_size = 20`, nên nếu mô hình đoán HOÀN TOÀN ngẫu nhiên (đều nhau trên `20` token), loss kỳ vọng là `-log(1/20) ≈ 3,0`. Loss đo được (`3,351352`, trên tham số CHƯA huấn luyện) ở gần mức đó — đúng như kỳ vọng của một mô hình chưa học gì.
::::

::::example{#gradient_lan_toi_tham_so}
Gọi `loss.backward()` MỘT lần — khác hẳn bài trước (gọi thẳng trên đầu ra thô của một khối, cho gradient `0` suy biến), lần này gradient đi qua MỘT hàm mất mát THẬT:

```python title=readonly
loss.backward()

print(round(float(np.sum(np.abs(logits.grad[-1]))), 6))     # hang CUOI -- khong co nhan
print(round(float(np.sum(np.abs(logits.grad[0]))), 6))       # hang 0 -- CO nhan
print(round(float(np.sum(np.abs(Bang.grad))), 6))
print(round(float(np.sum(np.abs(W_out.grad))), 6))
```

```text title=readonly
0.0
0.208509
1.212335
4.15824
```

Hàng CUỐI của `logits.grad` (vị trí `9`, không có "từ tiếp theo" thật để đối chiếu) là `0` TUYỆT ĐỐI — đúng như công thức: `dlogits[:n]` chỉ điền `n = 9` hàng đầu, hàng thứ `10` (chỉ số `9`) không bao giờ được gán. Hàng `0` (CÓ nhãn) nhận gradient khác `0`. Và quan trọng nhất: `Bang.grad` (bảng embedding) và `W_out.grad` (tầng chiếu cuối) đều khác `0` — gradient đã lan XUYÊN SUỐT toàn bộ pipeline, từ loss ngược về tới tận bảng embedding, qua CẢ HAI khối Transformer.
::::

::::predict{#doan_hang_cuoi_khong_giam_sat commitOnce}
Cửa sổ huấn luyện có `10` token — hàm mất mát chỉ tính trên `n = 9` vị trí đầu (dịch nhãn đi một vị trí, bỏ vị trí cuối).

**Trước khi đọc lại**, bạn đoán: nếu tăng cửa sổ lên `20` token (thay vì `10`), hàng CUỐI CÙNG của `logits.grad` — vị trí thứ `20` — vẫn sẽ là `0` tuyệt đối, hay sẽ khác `0`?

:::opt{correct}
Vẫn `0` tuyệt đối — bất kể cửa sổ dài bao nhiêu, VỊ TRÍ CUỐI CÙNG của bất kỳ cửa sổ nào cũng không có "từ tiếp theo" thật để đối chiếu (không còn token nào đứng sau nó TRONG cửa sổ đang xét); công thức `mat_mat_du_doan_tiep_theo` luôn bỏ qua đúng hàng cuối, không phụ thuộc độ dài `so_token`
:::

:::opt
Sẽ khác `0` — cửa sổ dài hơn cho mô hình nhiều ngữ cảnh hơn, nên MỌI vị trí (kể cả vị trí cuối) đều nhận được gradient nhiều hơn
::why
Gần đúng ở việc để ý cửa sổ DÀI HƠN đúng là cho mô hình nhiều thông tin hơn ở các vị trí CÓ nhãn — quan sát đó không sai cho những vị trí đó.

Chỗ lệch: "nhiều ngữ cảnh hơn" không tạo ra được một NHÃN cho vị trí cuối cùng — vấn đề không phải THIẾU thông tin, mà là KHÔNG CÓ token thật nào đứng sau vị trí cuối trong cửa sổ đang xét, bất kể cửa sổ dài `10` hay `20`. Hàng cuối luôn bị `dlogits[:n]` bỏ qua, với `n = so_token - 1` luôn nhỏ hơn `so_token` đúng `1`.
::
:::

:::opt
Phụ thuộc `vocab_size` — vocab lớn hơn thì gradient hàng cuối cũng lớn hơn theo
::why
Gần đúng ở việc `vocab_size` đúng là ảnh hưởng tới ĐỘ LỚN gradient ở các hàng CÓ nhãn (phân phối softmax trải trên nhiều lớp hơn) — quan sát đó không sai cho những hàng đó.

Chỗ lệch: hàng CUỐI không nằm trong phạm vi `dlogits[:n]` chút nào — nó KHÔNG được gán giá trị gì từ hàm mất mát này, nên vẫn giữ nguyên `0` từ lúc khởi tạo (`np.zeros_like`), bất kể `vocab_size` là bao nhiêu. `vocab_size` chỉ ảnh hưởng tới CÁC HÀNG CÒN LẠI, không tạo ra một giá trị nào cho hàng cuối.
::
:::
::::

::::code{#viet_mat_mat_tu_tiep_theo}
Hoàn thiện hai chỗ trống trong `mat_mat_du_doan_tiep_theo`: dịch nhãn đi một vị trí (forward), và trừ `1` tại logit đúng (backward).

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

    def embedding_lookup(self, ids):
        out = Tensor(self.data[ids], (self,), 'embedding_lookup')
        def _backward():
            for vi_tri, tid in enumerate(ids):
                self.grad[tid] += out.grad[vi_tri]
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


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = np.sin(goc[:, 0::2])
    pe[:, 1::2] = np.cos(goc[:, 1::2])
    return pe


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


def dem_cap_lien_ke(danh_sach):
    dem = {}
    for i in range(len(danh_sach) - 1):
        cap = (danh_sach[i], danh_sach[i + 1])
        dem[cap] = dem.get(cap, 0) + 1
    return dem


def gop_cap(danh_sach, cap):
    a, b = cap
    token_moi = a + b
    ra = []
    i = 0
    while i < len(danh_sach):
        if i < len(danh_sach) - 1 and danh_sach[i] == a and danh_sach[i + 1] == b:
            ra.append(token_moi)
            i += 2
        else:
            ra.append(danh_sach[i])
            i += 1
    return ra


def huan_luyen_bpe(corpus, muc_tieu_vocab):
    ds = list(corpus)
    vocab = set(ds)
    merges = []
    while len(vocab) < muc_tieu_vocab:
        dem = dem_cap_lien_ke(ds)
        if not dem:
            break
        cap_pho_bien = max(dem, key=dem.get)
        tan_suat = dem[cap_pho_bien]
        if tan_suat < 2:
            break
        ds = gop_cap(ds, cap_pho_bien)
        vocab.add(cap_pho_bien[0] + cap_pho_bien[1])
        merges.append(cap_pho_bien)
    return ds, merges, vocab


def xay_token_sang_id(vocab):
    return {t: i for i, t in enumerate(sorted(vocab))}


def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]


def mat_mat_du_doan_tiep_theo(logits, ids):
    nhan = ids[___]                            # 1:
    n = len(nhan)
    z = logits.data[:n]
    z_shift = z - np.max(z, axis=-1, keepdims=True)
    e = np.exp(z_shift)
    p = e / np.sum(e, axis=-1, keepdims=True)
    log_p_dung = np.log(p[np.arange(n), nhan])
    loss = -np.mean(log_p_dung)
    out = Tensor(loss, (logits,), 'mat_mat_du_doan_tiep_theo')
    def _backward():
        dlogits = np.zeros_like(logits.data)
        dz = p.copy()
        dz[np.arange(n), nhan] -= ___           # 1.0
        dz /= n
        dlogits[:n] = dz
        logits.grad += dlogits * out.grad
    out._backward = _backward
    return out


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)

dim, hidden, so_token = 4, 6, 10
vocab_size = len(vocab)
ids = ids_full[:so_token]

rng = np.random.default_rng(8)
def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
Bang = W((vocab_size, dim))
tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
W_out = W((dim, vocab_size))

PE = Tensor(ma_hoa_vi_tri(so_token, dim))
X = Bang.embedding_lookup(ids) + PE
mask = xay_mat_na_nhan_qua(so_token)
X_sau_2_tang = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
logits = X_sau_2_tang.matmul(W_out)

loss = mat_mat_du_doan_tiep_theo(logits, ids)
loss.backward()

print(round(float(loss.data), 6))
print(round(float(np.sum(np.abs(logits.grad[-1]))), 6))
print(round(float(np.sum(np.abs(Bang.grad))), 6))
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

    def embedding_lookup(self, ids):
        out = Tensor(self.data[ids], (self,), 'embedding_lookup')
        def _backward():
            for vi_tri, tid in enumerate(ids):
                self.grad[tid] += out.grad[vi_tri]
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


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = np.sin(goc[:, 0::2])
    pe[:, 1::2] = np.cos(goc[:, 1::2])
    return pe


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


def dem_cap_lien_ke(danh_sach):
    dem = {}
    for i in range(len(danh_sach) - 1):
        cap = (danh_sach[i], danh_sach[i + 1])
        dem[cap] = dem.get(cap, 0) + 1
    return dem


def gop_cap(danh_sach, cap):
    a, b = cap
    token_moi = a + b
    ra = []
    i = 0
    while i < len(danh_sach):
        if i < len(danh_sach) - 1 and danh_sach[i] == a and danh_sach[i + 1] == b:
            ra.append(token_moi)
            i += 2
        else:
            ra.append(danh_sach[i])
            i += 1
    return ra


def huan_luyen_bpe(corpus, muc_tieu_vocab):
    ds = list(corpus)
    vocab = set(ds)
    merges = []
    while len(vocab) < muc_tieu_vocab:
        dem = dem_cap_lien_ke(ds)
        if not dem:
            break
        cap_pho_bien = max(dem, key=dem.get)
        tan_suat = dem[cap_pho_bien]
        if tan_suat < 2:
            break
        ds = gop_cap(ds, cap_pho_bien)
        vocab.add(cap_pho_bien[0] + cap_pho_bien[1])
        merges.append(cap_pho_bien)
    return ds, merges, vocab


def xay_token_sang_id(vocab):
    return {t: i for i, t in enumerate(sorted(vocab))}


def ma_hoa_van_ban(van_ban, danh_sach_merge, token_sang_id):
    ds = list(van_ban)
    for cap in danh_sach_merge:
        ds = gop_cap(ds, cap)
    return [token_sang_id[t] for t in ds]


def mat_mat_du_doan_tiep_theo(logits, ids):
    nhan = ids[1:]
    n = len(nhan)
    z = logits.data[:n]
    z_shift = z - np.max(z, axis=-1, keepdims=True)
    e = np.exp(z_shift)
    p = e / np.sum(e, axis=-1, keepdims=True)
    log_p_dung = np.log(p[np.arange(n), nhan])
    loss = -np.mean(log_p_dung)
    out = Tensor(loss, (logits,), 'mat_mat_du_doan_tiep_theo')
    def _backward():
        dlogits = np.zeros_like(logits.data)
        dz = p.copy()
        dz[np.arange(n), nhan] -= 1.0
        dz /= n
        dlogits[:n] = dz
        logits.grad += dlogits * out.grad
    out._backward = _backward
    return out


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)

dim, hidden, so_token = 4, 6, 10
vocab_size = len(vocab)
ids = ids_full[:so_token]

rng = np.random.default_rng(8)
def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
Bang = W((vocab_size, dim))
tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
W_out = W((dim, vocab_size))

PE = Tensor(ma_hoa_vi_tri(so_token, dim))
X = Bang.embedding_lookup(ids) + PE
mask = xay_mat_na_nhan_qua(so_token)
X_sau_2_tang = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
logits = X_sau_2_tang.matmul(W_out)

loss = mat_mat_du_doan_tiep_theo(logits, ids)
loss.backward()

print(round(float(loss.data), 6))
print(round(float(np.sum(np.abs(logits.grad[-1]))), 6))
print(round(float(np.sum(np.abs(Bang.grad))), 6))
```

```python title=test
import numpy as np

assert round(float(loss.data), 6) == 3.351352, f"loss sai -- dang ra {round(float(loss.data), 6)}"
assert round(float(np.sum(np.abs(logits.grad[-1]))), 6) == 0.0, f"hang cuoi cua logits.grad phai bang 0 tuyet doi (khong co nhan) -- dang ra {round(float(np.sum(np.abs(logits.grad[-1]))), 6)}"
assert round(float(np.sum(np.abs(Bang.grad))), 6) == 1.212335, f"Bang.grad sum abs sai -- dang ra {round(float(np.sum(np.abs(Bang.grad))), 6)}"

# rieng kiem tra gradient THAT SU lan toi W_out va den ca hai khoi Transformer
assert np.sum(np.abs(W_out.grad)) > 0.0, "W_out.grad phai khac 0 -- gradient phai lan toi tang chieu cuoi"
assert np.sum(np.abs(tham_so_1[0].grad)) > 0.0, "Wq cua khoi 1 phai nhan gradient khac 0"
assert np.sum(np.abs(tham_so_2[0].grad)) > 0.0, "Wq cua khoi 2 phai nhan gradient khac 0"

# rieng kiem tra hang CO nhan (hang 0) phai khac 0 -- phan biet voi hang cuoi
assert np.sum(np.abs(logits.grad[0])) > 0.0, "hang 0 (co nhan) phai nhan gradient khac 0"

# kiem finite-difference THAT tren gia tri loss, doc lap voi Tensor
def softmax_np(z):
    zs = z - np.max(z, axis=-1, keepdims=True)
    e = np.exp(zs)
    return e / np.sum(e, axis=-1, keepdims=True)

nhan_doc_lap = ids[1:]
n_doc_lap = len(nhan_doc_lap)
def L_doc_lap(logits_data):
    z = logits_data[:n_doc_lap]
    p = softmax_np(z)
    return -np.mean(np.log(p[np.arange(n_doc_lap), nhan_doc_lap]))

logits_data_goc = logits.data.copy()
assert abs(L_doc_lap(logits_data_goc) - float(loss.data)) < 1e-9, "gia tri loss phai khop CONG THUC cross-entropy doc lap"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu dịch chuỗi token đi MỘT vị trí để làm nhãn — bỏ token ĐẦU TIÊN, giữ mọi token còn lại — `ids[1:]`. Chỗ hai (backward) trừ đúng `1` tại logit ĐÚNG (công thức `softmax − one-hot` đã học ở `cross_entropy_qua_value`, q8.2c) — `1.0`.
- kind: strategy
  body: 'Chỗ đầu: `1:`. Chỗ hai: `1.0`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `1:` (bên trong `ids[1:]`) và `1.0`.'
:::

:::validate
- tier: run
  timeoutMs: 20000
- tier: static
  onFail: nhan phai duoc rap bang ids[1:] (dich di DUNG mot vi tri, khong duoc dung ids nguyen ven hay mot chi so khac); backward phai tru DUNG 1.0 tai logit dung (khong duoc chep hang so khac hay bo qua buoc tru)
  requireAst:
  - kind: uses-name, target: ids, min: 2
  - kind: uses-operator, target: "-", min: 1
  - kind: has-literal, target: "1.0", min: 3
  # Da thu THAT bang kiemAst that (goi truc tiep tren code bien dich, khong
  # doan tay). ids=5 tren toan bo solution (Load rai rac, khong tut du dien
  # gi vao blank1 vi "ids[___]"/"ids[1:]" deu la Subscript doc "ids" dung
  # mot lan tai do -- luat nay xac nhan CO dung bien ids, khong xac nhan
  # DUNG cach cat lat, nen chi la mot rao chan phu). "-"=7 tren toan bo
  # solution (AugAssign Sub trong blank2 CONG rai rac cac phep tru khac co
  # san trong Tensor/layernorm -- so nay KHONG doi du blank2 dien gi, vi
  # node AugAssign(Sub) ton tai bat ke gia tri ben phai, nen day chi la mot
  # kiem tra sanity, KHONG phai cho chinh chan bua).
  # has-literal "1.0"=3 tren solution: HAI lan da co san trong boilerplate
  # (layernorm backward "-1.0/std", va "1.0 / np.sqrt(dim)" trong
  # nhan_hang_so) CONG mot lan trong blank2 "-= 1.0". Dien bua "___" ->
  # "True" o CA HAI blank lam has-literal "1.0" tut xuong DUNG 2 (mat dung
  # dong gop cua blank2) -- duoi nguong min=3, bi chan. Neu chi dat
  # min=1 (nhu ban truoc), hai lan co san trong boilerplate da du de qua
  # ngay ca khi dien bua -- day la loi da tu phat hien qua chinh
  # kiem_ma_bai_hoc.mjs (khong doan tay), sua bang cach nang nguong len 3.
  # Cheat "nhan = ids" (bo qua dich vi tri, du doan CHINH no thay vi tu tiep
  # theo) khong doi so dem cac luat static (van co "ids" o Load) NHUNG da
  # tu kiem chung bang Python that: loss.data se khac han 3.351352 dung (vi
  # nhan sai chuoi, VA n=len(ids)=10 thay vi 9, lam logits.data[:10] khac
  # shape mong doi so voi n=9) -- bi bat DOC LAP boi assert gia tri loss.
  # Cheat "dz[np.arange(n), nhan] -= 2.0" (sai hang so) lam has-literal
  # "1.0" tut xuong 2 -- duoi nguong min=3, bi chan RIENG, VA da tu kiem
  # chung: gradient sai gap doi, bi bat DOC LAP boi kiem finite-difference
  # doc lap.
- tier: tests
  timeoutMs: 20000
- tier: output
  match: regex
  expect: "^3\\.351352\\n0\\.0\\n1\\.212335\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Loss `3,351352` trên tham số CHƯA huấn luyện, gần đúng mức đoán ngẫu nhiên trên `20` token — và gradient giờ lan xuyên suốt, tới tận bảng embedding. Bài sau: dùng gradient đó để cập nhật tham số — MỘT bước huấn luyện.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`loss.backward()` giờ đã điền đúng `.grad` cho MỌI tham số — `Bang`, `Wq`/`Wk`/`Wv`/`W1`/`W2` của cả hai khối, `W_out`. Nhưng `.grad` MỚI chỉ là "hướng và độ lớn cần thay đổi" — bản thân `param.data` chưa hề nhúc nhích. Vòng lặp huấn luyện `vong-lap-huan-luyen` (q8.2c) đã học công thức cập nhật đơn giản nhất cho `Value`: `p.data -= lr * p.grad`. Công thức đó có cần đổi gì khi `p` giờ là một `Tensor` (mảng numpy) thay vì một số vô hướng không?
::::

::::checkpoint{mastery=0.85}
::::
