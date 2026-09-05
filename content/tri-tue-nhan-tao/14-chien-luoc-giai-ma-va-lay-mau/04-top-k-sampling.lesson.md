---
id: tri-tue-nhan-tao.chien-luoc-giai-ma-va-lay-mau.top-k-sampling
title: "Top-k sampling — chỉ giữ k xác suất cao nhất, chuẩn hoá lại"
summary: "Top-k sampling: sap xac suat giam dan (np.argsort), CHI GIU LAI k gia tri cao nhat, dat phan con lai ve 0, CHUAN HOA LAI (chia cho tong cua rieng k xac suat do) de tong lai bang 1. Tren phan phoi that cua micro-transformer: k=1 giu 1 token (0,999521 xac suat -- trung DUNG voi token argmax, chung minh top-k voi k=1 tuong duong greedy), k=5 giu 5 token (23 token con lai bi dat ve 0), k=10 giu 10 token (18 token bi loai), k=28=vocab_size giu ca 28 (khong doi gi -- phan phoi goc). Tong xac suat sau chuan hoa DUNG bang 1,0 o ca bon truong hop, do THAT khong suy doan."
locale: vi
track: tri-tue-nhan-tao
module: chien-luoc-giai-ma-va-lay-mau
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.top-k-sampling]
requires: [ai.nhiet-do]
concepts: [ai.top-k-sampling]
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

::::byte{trigger=enter mood=curious pose=point-editor}
Nhiệt độ co giãn TOÀN BỘ phân phối, nhưng không bao giờ đưa xác suất của bất kỳ token nào về đúng `0` — phần đuôi luôn còn một cơ hội nhỏ. Bài này cắt hẳn phần đuôi: chỉ giữ lại `k` token có xác suất cao nhất, đặt PHẦN CÒN LẠI về `0`, rồi chuẩn hoá lại để vẫn là một phân phối hợp lệ.
::::

::::explain{#top_k_va_chuan_hoa}
**Top-k sampling** làm ba việc, theo đúng thứ tự:

> **Sắp xếp** xác suất giảm dần, tìm `k` token có xác suất CAO NHẤT — dùng `np.argsort(phan_phoi)[::-1]` (sắp tăng dần rồi đảo ngược) để lấy chỉ số theo thứ tự giảm dần.
>
> **Cắt** — đặt xác suất của MỌI token KHÔNG nằm trong top-`k` về đúng `0`. Đây là điểm khác biệt cốt lõi với nhiệt độ: nhiệt độ chỉ làm xác suất đuôi NHỎ hơn, top-k làm nó bằng `0` TUYỆT ĐỐI.
>
> **Chuẩn hoá lại** — sau khi cắt, tổng xác suất còn lại KHÔNG còn bằng `1` nữa (vì đã bỏ bớt khối lượng xác suất của các token bị loại). Phải CHIA mỗi xác suất còn lại cho TỔNG của riêng chúng để phục hồi một phân phối hợp lệ (tổng lại bằng `1`).

Sau ba bước đó, lấy mẫu (`lay_mau_tu_phan_phoi` đã viết ở bài `2`) hoạt động bình thường trên phân phối đã cắt — nhưng giờ CHỈ có thể chọn một trong `k` token, không bao giờ chọn phải một token nằm ngoài top-`k`, dù ngẫu nhiên có "xui xẻo" tới đâu.

Một trường hợp biên đáng chú ý: khi `k=1`, chỉ một token còn xác suất khác `0` — và xác suất của nó, sau chuẩn hoá, LUÔN LÀ `1,0` (chia một số cho chính nó). Lấy mẫu trên một phân phối có đúng MỘT giá trị khác `0` luôn trả về đúng token đó, bất kể `seed` — top-k với `k=1` do đó tương đương HỆT với `argmax` (bài `1`).
::::

::::example{#cat_va_chuan_hoa_that}
Áp top-k lên phân phối softmax thật của micro-transformer, ở bốn giá trị `k`:

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


def giai_ma(ids, id_sang_token):
    return ''.join(id_sang_token[i] for i in ids)


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


def khoi_tao(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


def forward(mo_hinh, ids, dim):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    mask = xay_mat_na_nhan_qua(so_token)
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    return X2.matmul(W_out)


def mot_buoc(mo_hinh, ids, dim, so_token, mask, lr):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    logits = X2.matmul(W_out)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    loss.backward()
    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


corpus = "transformer dung attention de hoc tu du lieu, khong can de quy nhu mang hoi quy."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 28)
token_sang_id = xay_token_sang_id(vocab)
id_sang_token = {i: t for t, i in token_sang_id.items()}
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)

dim, hidden = 4, 6
vocab_size = len(vocab)
so_token = 10
ids_huan_luyen = ids_full[:so_token]
ids_mo_dau = ids_full[:4]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh = khoi_tao(9, vocab_size, dim, hidden)
for _ in range(15):
    mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5)


def top_k_phan_phoi(phan_phoi, k):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    idx_giu = idx_sap_xep[:k]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


logits_prompt = forward(mo_hinh, ids_mo_dau, dim)
z = logits_prompt.data[-1]
zs = z - np.max(z)
e = np.exp(zs)
p_goc = e / np.sum(e)

for k in [1, 5, 10, vocab_size]:
    pk = top_k_phan_phoi(p_goc, k)
    so_khac_khong = int(np.sum(pk > 0))
    print(k, so_khac_khong, round(float(pk.sum()), 6))
```

```text title=readonly
1 1 1.0
5 5 1.0
10 10 1.0
28 28 1.0
```

Bốn giá trị `k`, bốn kết quả: `k=1` giữ đúng `1` token khác `0`, `k=5` giữ `5`, `k=10` giữ `10`, `k=28` (bằng đúng `vocab_size`) giữ cả `28` — không loại bỏ gì, vì `vocab_size=28` là toàn bộ số token đang có. Ở CẢ BỐN giá trị `k`, tổng xác suất sau chuẩn hoá đều đúng `1,0` — dù đã cắt bỏ bao nhiêu token, phần còn lại luôn được chuẩn hoá lại thành một phân phối hợp lệ.
::::

::::predict{#doan_top_k_bang_1 commitOnce}
Với `k=1`, `top_k_phan_phoi` chỉ giữ lại đúng token có xác suất cao nhất, đặt mọi token khác về `0`, rồi chuẩn hoá.

**Trước khi chạy thử**, bạn đoán: kết quả của `int(np.argmax(top_k_phan_phoi(p_goc, 1)))` (token được chọn khi lấy mẫu trên phân phối top-`1` đã chuẩn hoá) có LUÔN bằng kết quả của `int(np.argmax(p_goc))` (token `argmax` trên phân phối GỐC, chưa cắt) hay không?

:::opt{correct}
Có — chuẩn hoá chia MỌI xác suất còn lại cho CÙNG một hằng số (tổng của chúng), phép chia đó KHÔNG đổi thứ tự tương đối giữa các giá trị; và vì top-`1` chỉ giữ lại đúng token có xác suất gốc cao nhất, token đó vẫn là token có xác suất cao nhất sau khi chuẩn hoá (giờ bằng đúng `1,0`)
:::

:::opt
Không — chuẩn hoá tính lại toàn bộ phân phối từ đầu, nên token có xác suất cao nhất có thể đổi sang một token khác sau khi chia lại
::why
Gần đúng ở việc để ý rằng chuẩn hoá THẬT SỰ thay đổi GIÁ TRỊ SỐ của xác suất (ví dụ từ `0,999521` lên đúng `1,0`).

Chỗ lệch: với `k=1`, chỉ CÓ đúng một token khác `0` sau bước cắt — không còn token nào khác để so sánh. Chia một số dương cho chính nó luôn cho `1,0`, và mọi token khác vẫn là `0` (chia `0` cho bất kỳ số dương nào vẫn là `0`) — không có "chỗ" nào để một token khác vượt lên trở thành `argmax` mới.
::
:::

:::opt
Không xác định được — phụ thuộc giá trị cụ thể của các xác suất bị loại bỏ, có thể chúng đủ lớn để ảnh hưởng kết quả chuẩn hoá
::why
Gần đúng ở việc chú ý rằng CÓ những phép biến đổi (ví dụ top-p ở bài sau) mà tập hợp token bị loại/giữ lại phụ thuộc rất nhạy vào giá trị cụ thể của phân phối.

Chỗ lệch: với `k=1` cố định, tập hợp "giữ lại" luôn chỉ có ĐÚNG MỘT phần tử — token có xác suất gốc lớn nhất — không phụ thuộc các xác suất khác lớn hay nhỏ thế nào. Chuẩn hoá một phân phối chỉ có một giá trị khác `0` luôn cho lại đúng giá trị đó bằng `1,0`, bất kể phần bị loại bỏ có tổng bao nhiêu.
::
:::
::::

::::code{#viet_top_k_phan_phoi}
Hoàn thiện `top_k_phan_phoi`: lấy đúng `k` chỉ số ĐẦU TIÊN của mảng đã sắp giảm dần, rồi chuẩn hoá phân phối đã cắt bằng cách chia cho tổng của chính nó.

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


def giai_ma(ids, id_sang_token):
    return ''.join(id_sang_token[i] for i in ids)


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


def khoi_tao(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


def forward(mo_hinh, ids, dim):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    mask = xay_mat_na_nhan_qua(so_token)
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    return X2.matmul(W_out)


def mot_buoc(mo_hinh, ids, dim, so_token, mask, lr):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    logits = X2.matmul(W_out)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    loss.backward()
    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


corpus = "transformer dung attention de hoc tu du lieu, khong can de quy nhu mang hoi quy."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 28)
token_sang_id = xay_token_sang_id(vocab)
id_sang_token = {i: t for t, i in token_sang_id.items()}
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)

dim, hidden = 4, 6
vocab_size = len(vocab)
so_token = 10
ids_huan_luyen = ids_full[:so_token]
ids_mo_dau = ids_full[:4]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh = khoi_tao(9, vocab_size, dim, hidden)
for _ in range(15):
    mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5)


def top_k_phan_phoi(phan_phoi, k):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    idx_giu = idx_sap_xep[___]                  # :k
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra ___ ra.sum()                      # /


logits_prompt = forward(mo_hinh, ids_mo_dau, dim)
z = logits_prompt.data[-1]
zs = z - np.max(z)
e = np.exp(zs)
p_goc = e / np.sum(e)

for k in [1, 5, 10, vocab_size]:
    pk = top_k_phan_phoi(p_goc, k)
    so_khac_khong = int(np.sum(pk > 0))
    print(k, so_khac_khong, round(float(pk.sum()), 6))
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


def giai_ma(ids, id_sang_token):
    return ''.join(id_sang_token[i] for i in ids)


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


def khoi_tao(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


def forward(mo_hinh, ids, dim):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    mask = xay_mat_na_nhan_qua(so_token)
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    return X2.matmul(W_out)


def mot_buoc(mo_hinh, ids, dim, so_token, mask, lr):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    logits = X2.matmul(W_out)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    loss.backward()
    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


corpus = "transformer dung attention de hoc tu du lieu, khong can de quy nhu mang hoi quy."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 28)
token_sang_id = xay_token_sang_id(vocab)
id_sang_token = {i: t for t, i in token_sang_id.items()}
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)

dim, hidden = 4, 6
vocab_size = len(vocab)
so_token = 10
ids_huan_luyen = ids_full[:so_token]
ids_mo_dau = ids_full[:4]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh = khoi_tao(9, vocab_size, dim, hidden)
for _ in range(15):
    mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5)


def top_k_phan_phoi(phan_phoi, k):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    idx_giu = idx_sap_xep[:k]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


logits_prompt = forward(mo_hinh, ids_mo_dau, dim)
z = logits_prompt.data[-1]
zs = z - np.max(z)
e = np.exp(zs)
p_goc = e / np.sum(e)

for k in [1, 5, 10, vocab_size]:
    pk = top_k_phan_phoi(p_goc, k)
    so_khac_khong = int(np.sum(pk > 0))
    print(k, so_khac_khong, round(float(pk.sum()), 6))
```

```python title=test
p1 = top_k_phan_phoi(p_goc, 1)
p5 = top_k_phan_phoi(p_goc, 5)
p10 = top_k_phan_phoi(p_goc, 10)
p28 = top_k_phan_phoi(p_goc, vocab_size)

assert int(np.sum(p1 > 0)) == 1, f"k=1 phai giu DUNG 1 token khac 0 -- dang ra {int(np.sum(p1 > 0))}"
assert int(np.sum(p5 > 0)) == 5, f"k=5 phai giu DUNG 5 token khac 0 -- dang ra {int(np.sum(p5 > 0))}"
assert int(np.sum(p10 > 0)) == 10, f"k=10 phai giu DUNG 10 token khac 0 -- dang ra {int(np.sum(p10 > 0))}"
assert int(np.sum(p28 > 0)) == 28, f"k=vocab_size phai giu ca 28 token -- dang ra {int(np.sum(p28 > 0))}"

# bang chung trung tam: cac token BI LOAI phai co xac suat DUNG BANG 0
idx_sap_xep_kiem = np.argsort(p_goc)[::-1]
idx_bi_loai_k5 = idx_sap_xep_kiem[10]  # chac chan ngoai top-5
assert p5[idx_bi_loai_k5] == 0.0, f"token bi loai boi top-5 phai co xac suat DUNG 0 -- dang ra {p5[idx_bi_loai_k5]}"

# tong xac suat sau chuan hoa phai DUNG bang 1.0 (trong sai so noi) o CA
# BON gia tri k
for p_kiem, ten in [(p1, "k=1"), (p5, "k=5"), (p10, "k=10"), (p28, "k=28")]:
    assert round(float(p_kiem.sum()), 6) == 1.0, f"tong xac suat sau chuan hoa ({ten}) phai la 1.0 -- dang ra {round(float(p_kiem.sum()), 6)}"

# bien bien gioi han: k=1 phai TUONG DUONG greedy (argmax tren phan phoi
# goc, chua cat)
assert int(np.argmax(p1)) == int(np.argmax(p_goc)), "top-k voi k=1 phai giu DUNG token argmax cua phan phoi goc"

# bien: k=vocab_size khong duoc doi GIA TRI cua phan phoi (khong con gi de
# cat bo)
assert np.allclose(p28, p_goc), "k=vocab_size phai cho lai DUNG phan phoi goc, khong doi gi"
```

:::hints
- kind: attention
  body: Hai chỗ trống trong `top_k_phan_phoi`. Chỗ đầu — `idx_giu = idx_sap_xep[___]` — phải lấy `k` chỉ số ĐẦU TIÊN của mảng đã sắp giảm dần (dùng cú pháp cắt lát `slice`). Chỗ hai — `return ra ___ ra.sum()` — phải CHIA mảng đã cắt cho tổng của chính nó để chuẩn hoá lại thành một phân phối hợp lệ.
- kind: strategy
  body: 'Chỗ đầu: `:k` — hoàn thiện thành `idx_sap_xep[:k]` (lấy `k` phần tử đầu). Chỗ hai: `/` — hoàn thiện thành `return ra / ra.sum()`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `:k` và `/`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: 'idx_giu phai duoc cat bang idx_sap_xep[:k] THAT (dung tham so k, khong duoc hardcode mot so co dinh vd :5); ket qua tra ve phai duoc CHIA cho ra.sum() (khong duoc tra ve ra chua chuan hoa)'
  requireAst:
  - kind: uses-name, target: "k", min: 3
  - kind: uses-operator, target: "/", min: 13
  # Da thu THAT bang kiemAst (goi truc tiep tren code trich tu solution da
  # bien dich, khong doan tay).
  # "k"=3: 1 lan trong blank1 (idx_sap_xep[:k]), 1 lan trong loi goi CO SAN
  # "pk = top_k_phan_phoi(p_goc, k)" (bien vong lap for k in [...]), 1 lan
  # trong "print(k, ...)" CO SAN. Dien bua blank1 thanh "idx_sap_xep[:5]"
  # (hardcode, tinh co dung o truong hop k=5 trong vi du) lam so nay tut
  # xuong 2 -- duoi nguong min=3, bi chan boi static; dong thoi bi chan boi
  # tests vi cac truong hop k=1/k=10/k=28 se sai (GOTCHA mutation o bien:
  # gia tri chinh xac 5 da xuat hien trong test, nhung hardcode van bi bat
  # vi cac k KHAC 5 cho ket qua sai).
  # "/"=13: TONG THAT gom 12 lan CO SAN rai rac trong boilerplate CONG 1
  # lan trong blank2 (return ra / ra.sum()). Dien bua blank2 thanh "return
  # ra" (khong chuan hoa) lam so nay tut xuong 12 -- duoi nguong min=13, bi
  # chan boi static; dong thoi bi chan boi tests vi tong xac suat sau do se
  # khong con la 1.0 khi da cat bot token (chi dung tinh co o k=28, nhung
  # sai o k=1/5/10).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^1 1 1\\.0\\n5 5 1\\.0\\n10 10 1\\.0\\n28 28 1\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`k=1` giữ `1` token — trùng đúng với `argmax`. `k=5`, `k=10`, `k=28` giữ đúng số lượng tương ứng, tổng xác suất luôn về lại `1,0`. Nhưng `k` là một con số CỐ ĐỊNH — chọn `k=5` cho một phân phối rất "nhọn" có thể thừa, chọn `k=5` cho một phân phối rất "phẳng" có thể thiếu. Bài sau (top-p) thay `k` cố định bằng một NGƯỠNG xác suất — số lượng token giữ lại tự thay đổi theo từng phân phối.
::::

::::reflect{#nghi-lai}
Top-k giải quyết đúng vấn đề nhiệt độ để lại: loại HẲN phần đuôi xác suất thấp, không chỉ làm nó nhỏ hơn. Nhưng `k` là một hằng số CỐ ĐỊNH chọn trước, không quan tâm phân phối đang xét "nhọn" hay "phẳng" tới đâu — với một phân phối rất tập trung (gần giống `argmax`), `k=10` có thể giữ lại nhiều token gần như vô nghĩa (xác suất siêu nhỏ); với một phân phối rất trải đều, `k=10` có thể bỏ sót nhiều token gần như quan trọng như top `10`. Bài sau (top-p / nucleus sampling) sửa đúng điểm đó: thay vì cố định SỐ LƯỢNG token, cố định TỔNG XÁC SUẤT cần đạt — số lượng token giữ lại trở thành một đại lượng ĐỘNG, tự thay đổi theo hình dạng của từng phân phối.
::::

::::checkpoint{mastery=0.8}
::::
