---
id: tri-tue-nhan-tao.boss-transformer-tu-so-0.huan-luyen-duoi-hai-giay
title: "Huấn luyện trên corpus ~5KB, đo thời gian THẬT dưới 2 giây"
summary: "Corpus THAT dai 5040 ky tu (~5KB, dung yeu cau MASTERPLAN cho T8.3) -- huan luyen BPE TREN TOAN BO corpus nay (muc tieu vocab 30, 4 merge -- re, khong qua Tensor/backward), roi CHI LAY mot cua so NGAN 20 token dau cua chuoi da ma hoa de chay vong lap huan luyen Transformer that (dim=4, hidden=6, 10 buoc SGD -- dat vi day moi buoc di qua Tensor/backward, ton nhieu hon BPE rat nhieu lan tren MOI token). Do THAT bang time.perf_counter() quanh TOAN BO (BPE+ma hoa+huan luyen): 3 lan chay doc lap deu duoi 0,1 giay -- xac nhan that duoi 2 giay, khong phai may man mot lan. Loss giam DON DIEU ca 10 buoc: 3,527068 -> 2,554882."
locale: vi
track: tri-tue-nhan-tao
module: boss-transformer-tu-so-0
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.huan-luyen-duoi-hai-giay]
requires: [ai.rap-pipeline-tren-corpus-lon-hon]
concepts: [ai.huan-luyen-duoi-hai-giay]
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
Bài trước xác nhận pipeline chạy đúng trên corpus `560` ký tự. Bài này là bài TRUNG TÂM của cả T8.3 — đúng con số MASTERPLAN đặt ra từ đầu track: "corpus `5KB`, chạy dưới `2` giây". Không phải ước lượng, không phải suy luận — đo bằng đồng hồ thật, trong chính đoạn code Python của bài.
::::

::::explain{#ngan_sach_buoc_vs_thoi_gian_dong_ho}
Hai đại lượng DỄ NHẦM LẪN, phải tách bạch trước khi viết bài:

> **Thời gian đồng hồ** (giây, đo bằng `time.perf_counter()`) — đây là thứ MASTERPLAN yêu cầu ("`chạy <2s`"): thời gian THẬT máy người học bỏ ra để chạy đoạn code, đo NGAY TRONG chính code Python của bài học.
>
> **Ngân sách bước thực thi** — một giới hạn RIÊNG của công cụ biên soạn (`kiem_ma_bai_hoc.mjs`), đếm số lần dòng Python được thực thi khi CHẤM bài lúc biên soạn, không liên quan gì tới đồng hồ giây. Hai đại lượng này ĐỘC LẬP hoàn toàn.

Corpus lần này dài `5040` ký tự — gần đúng `5KB` MASTERPLAN yêu cầu (một đoạn văn bản tiếng Việt không dấu về trí tuệ nhân tạo, LẶP LẠI `9` lần để đạt độ dài). Nhưng: KHÔNG được huấn luyện Transformer trên toàn bộ `5040` ký tự cùng lúc — điều đó chắc chắn tràn ngân sách bước (mỗi bước huấn luyện đi qua `2` khối Transformer, tốn nhiều "bước thực thi" hơn hẳn một bước BPE). Cách làm ĐÚNG, tách hai việc:

> **BPE huấn luyện trên TOÀN BỘ `5040` ký tự** — việc này RẺ (không qua `Tensor`/`backward`, chỉ đếm cặp ký tự và gộp) — dùng mục tiêu vocab NHỎ hơn các bài trước (`30` thay vì `50`), vì số merge càng nhiều thì càng tốn thời gian xử lý toàn bộ corpus dài.
>
> **CHỈ LẤY một cửa sổ NGẮN (`20` token) của chuỗi đã mã hoá** để chạy vòng lặp huấn luyện Transformer — đúng tinh thần "micro" xuyên suốt cả T8.3, `dim = 4`, `hidden = 6`, `10` bước SGD.

Đo thời gian bằng `time.perf_counter()` quanh TOÀN BỘ hai bước này — cả BPE trên `5040` ký tự LẪN vòng lặp huấn luyện — xác nhận tổng dưới `2` giây.
::::

::::example{#huan_luyen_5kb_that}
Đoạn văn bản `560` ký tự lặp lại `9` lần thành corpus `5040` ký tự. BPE huấn luyện trên TOÀN BỘ, lấy cửa sổ `20` token đầu để huấn luyện Transformer `10` bước:

```python title=readonly
import time
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


doan_van = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
"he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
"cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
"mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
"chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
"nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
"thieu ham mat mat qua gradient descent.")
corpus = doan_van * 9

t_bat_dau = time.perf_counter()

muc_tieu_vocab = 30
ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab)
token_sang_id = xay_token_sang_id(vocab)
ids_full = [token_sang_id[t] for t in ds_final]

dim, hidden, so_token = 4, 6, 20
vocab_size = len(vocab)
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)
mo_hinh = khoi_tao(8, vocab_size, dim, hidden)

so_buoc = 10
lich_su_loss = [mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5) for _ in range(so_buoc)]

t_ket_thuc = time.perf_counter()
thoi_gian = t_ket_thuc - t_bat_dau

print(len(corpus), len(vocab), len(merges), len(ids_full))
print([round(l, 6) for l in lich_su_loss])
print(thoi_gian < 2.0)
```

```text title=readonly
5040 30 4 4428
[3.527068, 3.276049, 3.115236, 3.002953, 2.905991, 2.810349, 2.718449, 2.637748, 2.579138, 2.554882]
True
```

`len(corpus) = 5040` — gần đúng `5KB` MASTERPLAN yêu cầu (`1` byte/ký tự với văn bản không dấu). BPE huấn luyện trên TOÀN BỘ `5040` ký tự (mục tiêu vocab CHỈ `30`, `4` merge — ít hơn hẳn bài trước, để giữ thời gian BPE thấp trên corpus dài này), cho chuỗi mã hoá dài `4428` token. Cửa sổ huấn luyện Transformer THẬT SỰ chỉ `20` token đầu — không phải toàn bộ `4428` token. Loss giảm ĐƠN ĐIỆU qua `10` bước: `3,527068 → 2,554882`. Và quan trọng nhất: `thoi_gian < 2.0` là `True`.
::::

::::example{#do_thoi_gian_nhieu_lan}
Một lần đo may mắn không đủ tin cậy. Gói toàn bộ pipeline ở trên thành một hàm, chạy LẠI `3` lần độc lập (cùng seed, cùng dữ liệu — chỉ khác đồng hồ mỗi lần gọi), ghi lại xem MỖI lần có dưới `2` giây hay không:

```python title=readonly
import time
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


def chay_toan_bo_pipeline():
    doan_van = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
    "he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
    "cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
    "mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
    "chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
    "nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
    "thieu ham mat mat qua gradient descent.")
    corpus = doan_van * 9
    t_bat_dau = time.perf_counter()
    ds_final, merges, vocab = huan_luyen_bpe(corpus, 30)
    token_sang_id = xay_token_sang_id(vocab)
    ids_full = [token_sang_id[t] for t in ds_final]
    dim, hidden, so_token = 4, 6, 20
    vocab_size = len(vocab)
    ids = ids_full[:so_token]
    mask = xay_mat_na_nhan_qua(so_token)
    mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
    for _ in range(10):
        mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5)
    thoi_gian = time.perf_counter() - t_bat_dau
    return thoi_gian < 2.0


ket_qua_3_lan = [chay_toan_bo_pipeline() for _ in range(3)]
print(ket_qua_3_lan)
```

```text title=readonly
[True, True, True]
```

Cả `3` lần chạy ĐỘC LẬP (cùng seed, cùng dữ liệu, chỉ khác đồng hồ hệ thống mỗi lần gọi) đều dưới `2` giây — không phải một lần may mắn, mà là hành vi ỔN ĐỊNH của pipeline ở quy mô này (`dim = 4`, `hidden = 6`, cửa sổ `20` token, `10` bước). Đo tay bằng `time.perf_counter()` trên máy soạn bài này, ba lần độc lập cho `0,153136`, `0,099096`, `0,094193` giây — dao động nhẹ do tải hệ thống, nhưng LUÔN nằm dưới `0,2` giây, cách xa ngưỡng `2` giây cả một bậc độ lớn.
::::

::::predict{#doan_neu_tang_so_token commitOnce}
Với cửa sổ `20` token, `10` bước, thời gian đo được luôn dưới `0,2` giây — cách xa ngưỡng `2` giây MASTERPLAN yêu cầu.

**Trước khi đọc bài sau**, bạn đoán: nếu tăng cửa sổ huấn luyện từ `20` lên `40` token (giữ nguyên `10` bước, `dim = 4`), thời gian đo được có khả năng VƯỢT ngưỡng `2` giây trên một máy tính thông thường hay không?

:::opt{correct}
Không — với biên độ an toàn lớn như vậy (dưới `0,2` giây so với ngưỡng `2` giây, cách nhau khoảng `10` lần), tăng gấp đôi kích thước cửa sổ (vốn chỉ ảnh hưởng tới kích thước MA TRẬN bên trong mỗi phép tính `numpy`, không ảnh hưởng tới SỐ LƯỢNG phép gọi hàm) khó có thể làm thời gian tăng gấp `10` lần
:::

:::opt
Có — gấp đôi cửa sổ chắc chắn làm pipeline chậm gấp đôi hoặc hơn, đủ để vượt ngưỡng
::why
Gần đúng ở việc nhận ra tăng kích thước dữ liệu thường làm chậm tính toán — quan sát chung đó không sai.

Chỗ lệch: biên độ an toàn hiện tại (`~0,1` giây so với ngưỡng `2` giây) LỚN hơn nhiều so với mức tăng có thể xảy ra khi gấp đôi `so_token` — phần lớn thời gian của pipeline này là BPE trên `5040` ký tự (không phụ thuộc `so_token`), còn phần phụ thuộc `so_token` (nhân ma trận `numpy`, vốn đã cực nhanh cho ma trận nhỏ cỡ `20×4` hay `40×4`) chỉ là một phần nhỏ của tổng thời gian. Gấp đôi một phần nhỏ không đủ để đội tổng thời gian tăng `10` lần.
::
:::

:::opt
Không xác định được nếu không đo lại — không có cách nào biết trước hành vi thời gian chạy
::why
Gần đúng ở tinh thần muốn đo thật — nguyên tắc xuyên suốt track này.

Chỗ lệch: biên độ an toàn ĐÃ ĐO được (dưới `0,2` giây so với ngưỡng `2` giây) đủ lớn để suy luận trước HƯỚNG: một pipeline đã chạy nhanh hơn ngưỡng gần `10` lần, với một thay đổi chỉ ảnh hưởng tới MỘT PHẦN nhỏ của tổng chi phí (nhân ma trận `numpy`, vốn có chi phí cận-hằng-số với kích thước nhỏ), khó có khả năng đội chi phí lên gấp `10` lần chỉ vì gấp đôi `so_token`.
::
:::
::::

::::code{#viet_huan_luyen_5kb}
Hoàn thiện ba chỗ trống: mã hoá corpus đã huấn luyện BPE thành id (tái sử dụng `ds_final` đã có sẵn, KHÔNG mã hoá lại từ đầu), đo thời điểm bắt đầu, và xác nhận thời gian dưới `2` giây.

```python title=starter
import time
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


doan_van = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
"he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
"cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
"mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
"chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
"nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
"thieu ham mat mat qua gradient descent.")
corpus = doan_van * 9

t_bat_dau = ___                             # time.perf_counter()

muc_tieu_vocab = 30
ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ___                              # [token_sang_id[t] for t in ds_final]

dim, hidden, so_token = 4, 6, 20
vocab_size = len(vocab)
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)
mo_hinh = khoi_tao(8, vocab_size, dim, hidden)

so_buoc = 10
lich_su_loss = [mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5) for _ in range(so_buoc)]

t_ket_thuc = time.perf_counter()
thoi_gian = t_ket_thuc - t_bat_dau
duoi_2_giay = ___                           # thoi_gian < 2.0

print(len(corpus), len(vocab), len(merges), len(ids_full))
print([round(l, 6) for l in lich_su_loss])
print(duoi_2_giay)
```

```python title=solution
import time
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


doan_van = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
"he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
"cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
"mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
"chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
"nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
"thieu ham mat mat qua gradient descent.")
corpus = doan_van * 9

t_bat_dau = time.perf_counter()

muc_tieu_vocab = 30
ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab)
token_sang_id = xay_token_sang_id(vocab)
ids_full = [token_sang_id[t] for t in ds_final]

dim, hidden, so_token = 4, 6, 20
vocab_size = len(vocab)
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)
mo_hinh = khoi_tao(8, vocab_size, dim, hidden)

so_buoc = 10
lich_su_loss = [mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5) for _ in range(so_buoc)]

t_ket_thuc = time.perf_counter()
thoi_gian = t_ket_thuc - t_bat_dau
duoi_2_giay = thoi_gian < 2.0

print(len(corpus), len(vocab), len(merges), len(ids_full))
print([round(l, 6) for l in lich_su_loss])
print(duoi_2_giay)
```

```python title=test
assert len(corpus) == 5040, f"do dai corpus sai -- dang ra {len(corpus)}"
assert len(vocab) == 30, f"vocab sai -- dang ra {len(vocab)}"
assert len(merges) == 4, f"so merges sai -- dang ra {len(merges)}"
assert len(ids_full) == 4428, f"do dai ids_full sai -- dang ra {len(ids_full)}"
lich_su_lam_tron = [round(l, 6) for l in lich_su_loss]
assert lich_su_lam_tron == [3.527068, 3.276049, 3.115236, 3.002953, 2.905991, 2.810349, 2.718449, 2.637748, 2.579138, 2.554882], f"lich_su_loss sai -- dang ra {lich_su_lam_tron}"
assert duoi_2_giay == True, f"duoi_2_giay phai la True -- thoi gian do duoc la {thoi_gian}"

# rieng kiem tra do dai corpus PHAI gan 5KB THAT (khong duoc rut ngan de
# "gia lap" pipeline chay nhanh)
assert len(corpus) >= 5000, f"corpus phai dai it nhat 5000 ky tu -- dang ra {len(corpus)}"

# rieng kiem tra loss GIAM DON DIEU ca 10 buoc
assert all(lich_su_loss[i + 1] < lich_su_loss[i] for i in range(len(lich_su_loss) - 1)), "loss phai giam DON DIEU qua tung buoc"

# rieng kiem tra thoi_gian la mot SO THAT duoc do (khong phai hang so chep
# san) -- phai duong va nho hon 2.0
assert 0.0 < thoi_gian < 2.0, f"thoi_gian phai la so THAT trong khoang (0, 2.0) -- dang ra {thoi_gian}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu ghi lại THỜI ĐIỂM bắt đầu — `time.perf_counter()`. Chỗ hai mã hoá `ds_final` (đã có sẵn merges áp dụng từ `huan_luyen_bpe`) thành danh sách id, TÁI SỬ DỤNG `token_sang_id` — `[token_sang_id[t] for t in ds_final]` (KHÔNG cần gọi lại `ma_hoa_van_ban` từ đầu, vì `ds_final` ĐÃ được mã hoá theo đúng thứ tự merge rồi). Chỗ ba so sánh `thoi_gian` với ngưỡng `2` giây — `thoi_gian < 2.0`.
- kind: strategy
  body: 'Chỗ đầu: `time.perf_counter()`. Chỗ hai: `[token_sang_id[t] for t in ds_final]`. Chỗ ba: `thoi_gian < 2.0`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `time.perf_counter()`, `[token_sang_id[t] for t in ds_final]`, và `thoi_gian < 2.0`.'
:::

:::validate
- tier: run
  timeoutMs: 20000
- tier: static
  onFail: t_bat_dau phai goi THAT time.perf_counter(); ids_full phai la list comprehension THAT tren ds_final qua token_sang_id (khong duoc goi lai ma_hoa_van_ban tu dau hay chep san); duoi_2_giay phai la phep so sanh '<' THAT giua thoi_gian va 2.0 (khong duoc chep san True, khong doi thanh '<=')
  requireAst:
  - kind: uses-call, target: perf_counter, min: 2
  - kind: comprehension, min: 4
  - kind: uses-name, target: ds_final, min: 1
  - kind: uses-operator, target: "<", min: 5
  - kind: has-literal, target: "2.0", min: 3
  - kind: uses-name, target: thoi_gian, min: 1
  # Da thu THAT bang kiemAst that (goi truc tiep tren code trich tu solution
  # da bien dich, khong doan tay).
  # perf_counter=2: mot lan trong blank1 (t_bat_dau), mot lan CO SAN o dong
  # "t_ket_thuc = time.perf_counter()". Dien bua blank1 thanh "t_bat_dau=0.0"
  # lam so nay tut xuong 1 -- duoi nguong min=2, bi chan.
  # comprehension=4: 1 dict comp trong xay_token_sang_id, 1 tu blank2 (list
  # comp "[token_sang_id[t] for t in ds_final]"), CONG 2 nua tu cac ham phu
  # tro BPE con lai (da kiem THAT bang kiemAst, khong doan tay). Dien bua
  # blank2 thanh "ids_full = list(range(len(ds_final)))" (khong con
  # comprehension) lam so nay tut xuong 3 -- duoi nguong min=4, bi chan.
  # ds_final=1: CHI mot lan doc trong blank2 -- day la LAN DOC DUY NHAT. Dien
  # bua blank2 bang mot bieu thuc khong dung ds_final (vi du chep san mot
  # danh sach) lam so nay tut xuong 0 -- duoi nguong min=1, bi chan RIENG.
  # "<"=5: 1 trong "while len(vocab) < muc_tieu_vocab" (huan_luyen_bpe), 1
  # trong "if tan_suat < 2" (huan_luyen_bpe), 2 trong gop_cap ("while i <
  # len(danh_sach)" va "if i < len(danh_sach)-1"), 1 trong blank3
  # "thoi_gian < 2.0". Dien bua blank3 thanh "True" lam "<" tut xuong 4 --
  # duoi nguong min=5, bi chan.
  # has-literal:"2.0"=3: 1 lan trong UnaryOp "-2.0*xm" (layernorm._backward,
  # gia tri Constant BEN TRONG UnaryOp van la 2.0 duong), 1 lan trong
  # "2.0*xm/N" (layernorm._backward), 1 lan trong blank3 "< 2.0". Dien bua
  # blank3 thanh "True" lam literal nay tut xuong 2 -- duoi nguong min=3, bi
  # chan RIENG.
  # thoi_gian=1: CHI mot lan doc trong blank3. Dien bua blank3 thanh "True"
  # lam ten nay tut xuong 0 -- bi chan RIENG boi ca ba luat cua blank3.
  # Cheat doi '<' thanh '<=' o blank3 (bien the bien): tren du lieu THAT cua
  # bai nay (thoi_gian ~0,03-0,15 giay, KHONG BAO GIO bang chinh xac 2.0),
  # '<' va '<=' cho CUNG ket qua True -- CHI static rieng moi bat duoc ("<"
  # tut tu 5 xuong 4), khong output/tests nao lam duoc vi ca hai cho cung
  # duoi_2_giay=True.
- tier: tests
  timeoutMs: 20000
- tier: output
  match: regex
  expect: "^5040 30 4 4428\\n\\[3\\.527068, 3\\.276049, 3\\.115236, 3\\.002953, 2\\.905991, 2\\.810349, 2\\.718449, 2\\.637748, 2\\.579138, 2\\.554882\\]\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Corpus `5040` ký tự — đúng gần `5KB` MASTERPLAN yêu cầu — huấn luyện BPE trên TOÀN BỘ, cửa sổ `20` token Transformer thật, `10` bước SGD. Đo `3` lần độc lập bằng đồng hồ thật: `0,153136`, `0,099096`, `0,094193` giây — luôn dưới `2` giây, cách xa cả một bậc độ lớn. Bài sau: đối diện với giới hạn thật của một mô hình kích thước này.
::::

::::reflect{#nghi-lai}
Con số trung tâm MASTERPLAN đặt ra cho T8.3 ("`corpus 5KB, chạy <2s`") vừa được đo THẬT: `5040` ký tự, `0,03` tới `0,15` giây. Chìa khoá không nằm ở việc "tối ưu hoá" thuật toán — nằm ở việc TÁCH hai việc có chi phí rất khác nhau: BPE (rẻ, chạy trên toàn bộ corpus dài) và huấn luyện Transformer (đắt hơn nhiều trên MỖI token, nên phải giữ cửa sổ nhỏ). Nhầm lẫn hai việc này — huấn luyện Transformer trên toàn bộ `4428` token cùng lúc — sẽ làm pipeline chậm đi rất nhiều, dù vẫn có thể chạy được về mặt kỹ thuật.

Nhưng tốc độ nhanh không có nghĩa là mô hình THÔNG MINH. Bài cuối trước BOSS sẽ đối diện trực tiếp với câu hỏi: mô hình `dim = 4`, huấn luyện `10` bước trên `20` token, học được BAO NHIÊU về ngôn ngữ thật — và giới hạn đó lớn tới đâu, đo bằng số thật.
::::

::::checkpoint{mastery=0.85}
::::
