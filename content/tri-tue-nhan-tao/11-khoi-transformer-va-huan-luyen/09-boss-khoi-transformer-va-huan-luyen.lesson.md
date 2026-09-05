---
id: tri-tue-nhan-tao.khoi-transformer-va-huan-luyen.boss-khoi-transformer-va-huan-luyen
title: "BOSS — Huấn luyện Transformer 2 tầng, đo hành vi THAY ĐỔI thật trước/sau"
summary: "Rap TOAN BO: BPE hoa corpus that -> Transformer 2 tang (moi tang tham so rieng) -> loss du doan tu tiep theo -> 12 buoc SGD tren cua so 10 token, loss 3,351352 -> 1,562128 GIAM DON DIEU -- sinh van ban TRUOC huan luyen tu 'may hoc' ra 'may hoct,,o' (vo nghia), SAU huan luyen ra 'may hoc hoc hoc hoc hoc' (lap dung mau da hoc). Do THAT xac suat mo hinh gan cho token DUNG tai mot vi tri kiem tra: 0,027688 TRUOC huan luyen (duoi muc ngau nhien 1/20=0,05) len 0,084085 SAU 12 buoc (tren muc ngau nhien) -- tang 3,036844 lan, bang chung so THAT rang huan luyen thay doi hanh vi mo hinh. Dong quest khoi-transformer-va-huan-luyen (9/9), T8.3 con lai dung boss-transformer-tu-so-0 (q8.3e)."
locale: vi
track: tri-tue-nhan-tao
module: khoi-transformer-va-huan-luyen
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-khoi-transformer-va-huan-luyen]
requires: [ai.sinh-van-ban]
concepts: [ai.boss-khoi-transformer-va-huan-luyen]
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
Tám bài, tám mảnh: kết nối tắt, feedforward theo vị trí, một khối Transformer, xếp chồng hai tầng, mất mát dự đoán từ tiếp theo, một bước cập nhật, vòng lặp huấn luyện, sinh văn bản. Bài này ráp TẤT CẢ vào một pipeline — và chứng minh huấn luyện THẬT SỰ thay đổi hành vi mô hình, không chỉ chạy cho có.
::::

::::explain{#rap_toan_bo_pipeline}
Pipeline đầy đủ của quest này, KHÔNG thêm gì mới — ĐÚNG những gì tám bài trước đã xây:

> **BPE** (`token-hoa-bpe`, q8.3a) — huấn luyện trên corpus thật, mã hoá thành token ID.
>
> **Embedding + vị trí** (`co-che-attention`, q8.3c) — `Bang.embedding_lookup(ids) + PE`.
>
> **Hai khối Transformer xếp chồng** (`rap-khoi-transformer`, `xep-chong-hai-tang`) — mỗi khối: attention một-đầu + residual + layernorm, rồi feedforward + residual + layernorm; MỖI khối một bộ tham số RIÊNG.
>
> **Chiếu logit + mất mát dự đoán từ tiếp theo** (`mat-mat-tu-tiep-theo`) — `W_out`, `mat_mat_du_doan_tiep_theo`.
>
> **Vòng lặp huấn luyện** (`mot-buoc-cap-nhat`, `huan-luyen-tren-corpus-that`) — SGD đơn giản, RESET gradient mỗi bước, `12` bước.
>
> **Sinh văn bản** (`sinh-van-ban`) — greedy decoding, vị trí cuối, giải mã ngược.

Bằng chứng trung tâm của BOSS này: chạy `sinh_van_ban` và đo xác suất token đúng ở một vị trí kiểm tra, TRƯỚC và SAU `12` bước huấn luyện, trên CÙNG một mô hình (cùng khởi tạo, cùng dữ liệu) — nếu huấn luyện thật sự có tác dụng, cả hai con số này phải khác nhau THEO ĐÚNG HƯỚNG (xác suất đúng tăng lên, văn bản sinh ra bớt vô nghĩa hơn).
::::

::::example{#boss_truoc_huan_luyen}
Trước khi huấn luyện bước nào: sinh văn bản từ `"may hoc"`, và đo xác suất mô hình gán cho token ĐÚNG tại vị trí `4` của cửa sổ huấn luyện:

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


def xac_suat_dung_tai(mo_hinh, ids, dim, vi_tri):
    logits = forward(mo_hinh, ids, dim)
    z = logits.data[vi_tri]
    zs = z - np.max(z)
    e = np.exp(zs)
    p = e / np.sum(e)
    return float(p[ids[vi_tri + 1]])


def sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        token_moi = int(np.argmax(logits.data[-1]))
        ids.append(token_moi)
    return ids


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
id_sang_token = {i: t for t, i in token_sang_id.items()}
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)
dim, hidden = 4, 6
vocab_size = len(vocab)
so_token = 10
ids_huan_luyen = ids_full[:so_token]
ids_mo_dau = ids_full[:4]

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)

p_truoc = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_truoc = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 4)

print(round(p_truoc, 6))
print(sinh_truoc, repr(giai_ma(sinh_truoc, id_sang_token)))
```

```text title=readonly
0.027688
[14, 6, 19, 3, 17, 4, 4, 16] 'may hoct,,o'
```

Tham số CHƯA huấn luyện (vừa khởi tạo ngẫu nhiên, `seed=8`): xác suất mô hình gán cho token ĐÚNG tại vị trí `4` chỉ `0,027688` — THẤP HƠN mức đoán ngẫu nhiên đều (`1/20 = 0,05`). Sinh văn bản từ `"may hoc"` cho `"may hoct,,o"` — vô nghĩa, đúng như kỳ vọng của một mô hình chưa học gì.
::::

::::example{#boss_sau_huan_luyen}
Huấn luyện đúng `12` bước (SGD, `lr = 0,5`, reset gradient mỗi bước — bài `mot-buoc-cap-nhat`/`huan-luyen-tren-corpus-that`), rồi đo LẠI đúng hai con số trên, TRÊN CÙNG MÔ HÌNH:

```python title=readonly
def mot_buoc(mo_hinh, ids, dim, lr):
    logits = forward(mo_hinh, ids, dim)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    loss.backward()
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


lich_su_loss = [mot_buoc(mo_hinh, ids_huan_luyen, dim, 0.5) for _ in range(12)]

p_sau = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_sau = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 4)

print([round(l, 6) for l in lich_su_loss])
print(round(p_sau, 6))
print(sinh_sau, repr(giai_ma(sinh_sau, id_sang_token)))
print(p_sau > p_truoc, round(p_sau / p_truoc, 6))
```

```text title=readonly
[3.351352, 2.989879, 2.739706, 2.53488, 2.361116, 2.210683, 2.077004, 1.954192, 1.838194, 1.727511, 1.62595, 1.562128]
0.084085
[14, 6, 19, 3, 3, 3, 3, 3] 'may hoc hoc hoc hoc hoc'
True 3.036844
```

Ba bằng chứng ĐỘC LẬP, đo bằng số thật, đều cùng một hướng:

> **Loss giảm đơn điệu** — `3,351352 → 1,562128` qua `12` bước.
>
> **Xác suất token đúng tăng** — `0,027688 → 0,084085`, gấp `3,036844` lần, từ DƯỚI mức ngẫu nhiên lên TRÊN mức ngẫu nhiên (`0,05`).
>
> **Văn bản sinh ra thay đổi có ý nghĩa** — từ `"may hoct,,o"` (vô nghĩa) sang `"may hoc hoc hoc hoc hoc"` (lặp lại đúng mẫu `"hoc"` xuất hiện dày đặc nhất trong corpus huấn luyện).

Huấn luyện `12` bước SGD đơn giản — không Adam, không hàng nghìn bước — đã THẬT SỰ thay đổi hành vi của khối Transformer `2` tầng, đo được bằng ba con số độc lập, không phải suy luận.
::::

::::predict{#doan_them_buoc_huan_luyen commitOnce}
`12` bước cho `p_sau = 0,084085` (tăng `3,036844` lần so với `p_truoc`).

**Trước khi tính**, bạn đoán: nếu huấn luyện TIẾP tới tổng `20` bước (thay vì dừng ở `12`), xác suất token đúng tại đúng vị trí kiểm tra này sẽ tiếp tục CAO HƠN `0,084085`, hay sẽ THẤP HƠN (vì mô hình bắt đầu "học quá đà" trên một cửa sổ chỉ `10` token)?

:::opt{correct}
Cao hơn — xu hướng TỔNG THỂ của tối ưu hoá gradient trên bài toán này vẫn là giảm loss/tăng xác suất đúng, dù CÓ THỂ có những bước cục bộ dao động (bài `huan-luyen-tren-corpus-that` đã đo bước `13` không giảm) — dao động cục bộ không phủ định xu hướng chung khi nhìn xa hơn vài bước
:::

:::opt
Thấp hơn — huấn luyện thêm trên một cửa sổ CHỈ `10` token chắc chắn gây quá khớp (overfitting), làm mô hình học những đặc điểm KHÔNG liên quan rồi mất khả năng dự đoán đúng
::why
Gần đúng ở việc nêu đúng MỐI LO overfitting khi dữ liệu ít — đây là một lo ngại THẬT, có cơ sở, và track này (T8.3 nói chung, `boss-transformer-tu-so-0` sắp tới) có bàn kỹ vấn đề này.

Chỗ lệch: overfitting làm mô hình khớp TỐT HƠN với dữ liệu huấn luyện (chính là điều đang đo ở đây — xác suất TRÊN CỬA SỔ HUẤN LUYỆN), không phải khớp TỆ hơn. "Quá khớp" là vấn đề khi đo trên dữ liệu MỚI chưa từng huấn luyện — ở đây đang đo lại chính vị trí ĐÃ huấn luyện, nên xác suất tiếp tục tăng đúng là dấu hiệu dự kiến, không phải bằng chứng của một lỗi. Số thật xác nhận: `20` bước cho `p = 0,255909`, cao hơn hẳn `12` bước.
::
:::

:::opt
Không xác định được nếu không chạy — không có cơ sở nào để đoán trước xu hướng
::why
Gần đúng ở tinh thần muốn đo thật trước khi kết luận chắc chắn — nguyên tắc xuyên suốt track này.

Chỗ lệch: dù GIÁ TRỊ chính xác cần đo thật, HƯỚNG đi có thể suy luận trước từ cơ chế đã biết: SGD di chuyển tham số theo hướng làm GIẢM loss tại mỗi bước (đúng định nghĩa của gradient descent), và loss thấp hơn tương ứng với xác suất token đúng CAO hơn (định nghĩa của cross-entropy) — nên xu hướng chung (nhiều bước hơn thường cho xác suất cao hơn, dù có thể không đơn điệu tuyệt đối từng bước) là một suy luận có cơ sở, không phải đoán mò.
::
:::
::::

::::code{#viet_boss_pipeline}
Hoàn thiện hai chỗ trống: tính xác suất token đúng SAU huấn luyện, và xác nhận nó lớn hơn xác suất TRƯỚC huấn luyện.

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


def xac_suat_dung_tai(mo_hinh, ids, dim, vi_tri):
    logits = forward(mo_hinh, ids, dim)
    z = logits.data[vi_tri]
    zs = z - np.max(z)
    e = np.exp(zs)
    p = e / np.sum(e)
    return float(p[ids[vi_tri + 1]])


def sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        token_moi = int(np.argmax(logits.data[-1]))
        ids.append(token_moi)
    return ids


def mot_buoc(mo_hinh, ids, dim, lr):
    logits = forward(mo_hinh, ids, dim)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    loss.backward()
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
id_sang_token = {i: t for t, i in token_sang_id.items()}
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)
dim, hidden = 4, 6
vocab_size = len(vocab)
so_token = 10
ids_huan_luyen = ids_full[:so_token]
ids_mo_dau = ids_full[:4]

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
p_truoc = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_truoc = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 4)

lich_su_loss = [mot_buoc(mo_hinh, ids_huan_luyen, dim, 0.5) for _ in range(12)]

p_sau = ___                                    # xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_sau = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 4)
huan_luyen_co_hieu_qua = ___                   # p_sau > p_truoc

print(round(p_truoc, 6), round(p_sau, 6))
print(giai_ma(sinh_truoc, id_sang_token))
print(giai_ma(sinh_sau, id_sang_token))
print(huan_luyen_co_hieu_qua)
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


def xac_suat_dung_tai(mo_hinh, ids, dim, vi_tri):
    logits = forward(mo_hinh, ids, dim)
    z = logits.data[vi_tri]
    zs = z - np.max(z)
    e = np.exp(zs)
    p = e / np.sum(e)
    return float(p[ids[vi_tri + 1]])


def sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        token_moi = int(np.argmax(logits.data[-1]))
        ids.append(token_moi)
    return ids


def mot_buoc(mo_hinh, ids, dim, lr):
    logits = forward(mo_hinh, ids, dim)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    loss.backward()
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
id_sang_token = {i: t for t, i in token_sang_id.items()}
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)
dim, hidden = 4, 6
vocab_size = len(vocab)
so_token = 10
ids_huan_luyen = ids_full[:so_token]
ids_mo_dau = ids_full[:4]

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
p_truoc = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_truoc = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 4)

lich_su_loss = [mot_buoc(mo_hinh, ids_huan_luyen, dim, 0.5) for _ in range(12)]

p_sau = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_sau = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 4)
huan_luyen_co_hieu_qua = p_sau > p_truoc

print(round(p_truoc, 6), round(p_sau, 6))
print(giai_ma(sinh_truoc, id_sang_token))
print(giai_ma(sinh_sau, id_sang_token))
print(huan_luyen_co_hieu_qua)
```

```python title=test
assert round(p_truoc, 6) == 0.027688, f"p_truoc sai -- dang ra {round(p_truoc, 6)}"
assert round(p_sau, 6) == 0.084085, f"p_sau sai -- dang ra {round(p_sau, 6)}"
assert huan_luyen_co_hieu_qua == True, "huan_luyen_co_hieu_qua phai la True -- xac suat token dung phai TANG sau huan luyen"
assert giai_ma(sinh_truoc, id_sang_token) == "may hoct,,o", f"van ban TRUOC huan luyen sai -- dang ra {giai_ma(sinh_truoc, id_sang_token)!r}"
assert giai_ma(sinh_sau, id_sang_token) == "may hoc hoc hoc hoc hoc", f"van ban SAU huan luyen sai -- dang ra {giai_ma(sinh_sau, id_sang_token)!r}"

# BANG CHUNG TRUNG TAM cua BOSS: ca ba do luong (loss, xac suat, van ban)
# deu phai xac nhan huan luyen co hieu qua, DOC LAP voi nhau.
assert round(lich_su_loss[-1], 6) < round(lich_su_loss[0], 6), "loss cuoi phai nho hon loss dau"
assert round(lich_su_loss[-1], 6) == 1.562128, f"loss cuoi cung sai -- dang ra {round(lich_su_loss[-1], 6)}"
assert p_sau / p_truoc > 2.0, f"xac suat dung phai tang it nhat gap doi -- dang ra ti le {p_sau / p_truoc}"
assert giai_ma(sinh_truoc, id_sang_token) != giai_ma(sinh_sau, id_sang_token), "van ban sinh TRUOC va SAU huan luyen phai khac nhau"

# rieng kiem tra p_sau duoc tinh THAT (khong chep gia tri p_truoc)
assert round(p_sau, 6) != round(p_truoc, 6), "p_sau phai duoc TINH LAI sau huan luyen, khong duoc chep lai p_truoc"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu tính LẠI xác suất token đúng, SAU khi đã huấn luyện `12` bước (dùng lại đúng hàm đã dùng cho `p_truoc`, gọi lại TRÊN mô hình vừa cập nhật) — `xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)`. Chỗ hai so sánh xác suất SAU với TRƯỚC — `p_sau > p_truoc`.
- kind: strategy
  body: 'Chỗ đầu: `xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)`. Chỗ hai: `p_sau > p_truoc`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)` và `p_sau > p_truoc`.'
:::

:::validate
- tier: run
  timeoutMs: 30000
- tier: static
  onFail: p_sau phai duoc tinh THAT bang xac_suat_dung_tai(...) tren mo hinh DA huan luyen (khong duoc chep lai p_truoc hay mot hang so); huan_luyen_co_hieu_qua phai la phep so sanh '>' THAT giua p_sau va p_truoc (khong duoc chep san True)
  requireAst:
  - kind: uses-call, target: xac_suat_dung_tai, min: 2
  - kind: uses-operator, target: ">", min: 2
  - kind: uses-name, target: p_sau, min: 2
  # Da thu THAT bang kiemAst that (goi truc tiep tren code bien dich, khong
  # doan tay). xac_suat_dung_tai=2 tren solution (dinh nghia ham khong
  # tinh; goi THAT: mot lan cho p_truoc [co san trong starter, KHONG phai
  # blank], mot lan trong blank1 -- tong 2). Dien bua ca hai blank thanh
  # "True" lam so nay tut xuong 1 (chi con loi goi cho p_truoc) -- duoi
  # nguong min=2, bi chan.
  # p_sau=2 tren solution: dinh nghia "p_sau = ..." la Store (khong dem);
  # Load: blank2 "p_sau > p_truoc" [1], dong in ket qua
  # "print(round(p_truoc, 6), round(p_sau, 6))" [1] (dong nay CO SAN, khong
  # phai blank) = 2. Dien bua "___" -> "True" o CA HAI blank lam p_sau tut
  # xuong 1 (chi con lai lan Load trong dong in, mat lan Load trong blank2)
  # -- duoi nguong min=2, bi chan.
  #
  # SUA LAI (phat hien lo dot bien THAT qua tools/kiem_dot_bien.mjs): ban
  # dau ghi ">"=1, dua tren nhan dinh SAI rang nguong nay "khong the tu no
  # chan bua vi relu con giu 1 '>' co san". Nhan dinh do dung cho nguong
  # min=1 nhung khong con dung khi nang len min=2: tong ">" that su tren
  # solution la 2 (mot lan CO SAN trong Tensor.relu()._backward, "self.data
  # > 0", khong dinh gi den blank; mot lan trong blank2 "p_sau > p_truoc").
  # Cheat "huan_luyen_co_hieu_qua = p_sau >= p_truoc" (doi '>' thanh '>=',
  # GIU NGUYEN ten p_sau nen KHONG cham gi den luat p_sau) da tu kiem chung
  # bang kiemAst THAT: lam tong ">" tut tu 2 xuong 1 (chi con lai cua relu)
  # -- duoi nguong moi min=2, bi chan RIENG boi luat ">". Day la lo dot
  # bien q8.3d tu phat hien: tren du lieu THAT cua bai nay (p_sau=0,084085,
  # p_truoc=0,027688, khong bao gio bang nhau), doi '>' thanh '>=' KHONG
  # doi ket qua boolean -- chi static rieng moi bat duoc, khong output/tests
  # nao lam duoc vi ca hai deu cho huan_luyen_co_hieu_qua=True giong het.
  # Cheat "p_sau = p_truoc" (chep lai, bo qua tinh lai) lam
  # "xac_suat_dung_tai" tut xuong 1 -- bi chan RIENG, VA da tu kiem chung
  # bang Python that: assert "p_sau != p_truoc" (tier tests) se THAT BAI
  # ngay lap tuc -- bi bat DOC LAP.
  # Cheat "huan_luyen_co_hieu_qua = True" (chep san, bo qua so sanh THAT)
  # lam ">" tut xuong 1 (van dat nguong min=1, KHONG tu no bi chan) VA
  # "p_sau" tut xuong 1 (duoi nguong min=2, BI CHAN) -- chan boi luat p_sau.
- tier: tests
  timeoutMs: 30000
- tier: output
  match: regex
  expect: "^0\\.027688 0\\.084085\\nmay hoct,,o\\nmay hoc hoc hoc hoc hoc\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Loss `3,351352 → 1,562128`. Xác suất đúng `0,027688 → 0,084085`, gấp `3,036844` lần. Văn bản `"may hoct,,o" → "may hoc hoc hoc hoc hoc"`. Ba con số độc lập, cùng một hướng — huấn luyện Transformer `2` tầng THẬT SỰ có tác dụng. Quest `khoi-transformer-va-huan-luyen` khép lại tại đây.
::::

::::reflect{#nghi-lai}
Quest `khoi-transformer-va-huan-luyen` khép lại tại đây, chín bài: kết nối tắt (đường tắt cho gradient, hệ số `1`, đối chiếu trực tiếp với vanishing gradient của `gradient-bien-mat-that`, q8.2d — sai khác hơn `6621` lần chỉ sau `6` tầng), mạng truyền thẳng theo vị trí (`relu` mới cho `Tensor`, áp độc lập từng vị trí — khác hẳn attention), ráp một khối Transformer (post-norm, và một gotcha thật: gọi `backward()` trực tiếp trên đầu ra kết thúc bằng `layernorm` cho gradient `0` suy biến), xếp chồng hai tầng (tham số RIÊNG cho mỗi tầng), mất mát dự đoán từ tiếp theo (dịch nhãn một vị trí, tái dùng log-sum-exp), một bước cập nhật (SGD đơn giản, và gotcha reset gradient tái hiện trên `Tensor`), vòng lặp huấn luyện trên corpus thật (`12` bước, loss giảm đơn điệu, và một sự thật thật hơn cả lý thuyết: SGD không đảm bảo giảm mãi mãi), sinh văn bản (greedy decoding, đúng vị trí cuối), và BOSS này: huấn luyện thật, đo hành vi thay đổi thật qua BA con số độc lập.

T8.3 "Transformer từ số 0" còn lại đúng MỘT quest: `boss-transformer-tu-so-0` (q8.3e, BOSS đóng cả track) — vì sao attention không bị vanishing gradient (đối chiếu trực tiếp với `gradient-bien-mat-that`), so sánh MLP với Transformer trên một bài toán phụ thuộc xa, ráp pipeline đầy đủ trên corpus lớn hơn (gần `5KB`, đúng yêu cầu MASTERPLAN), và giới hạn thật của một micro-transformer kích thước này. Khối Transformer vừa ráp ở quest này — attention, residual, layernorm, feedforward, xếp chồng, huấn luyện, sinh văn bản — chính là nền tảng quest cuối cùng đó sẽ dùng lại, không viết thêm gì mới.
::::

::::checkpoint{mastery=0.95}
::::
