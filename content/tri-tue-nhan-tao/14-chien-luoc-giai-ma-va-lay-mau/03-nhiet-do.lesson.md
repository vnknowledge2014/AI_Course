---
id: tri-tue-nhan-tao.chien-luoc-giai-ma-va-lay-mau.nhiet-do
title: "Nhiệt độ (temperature) — chia logit trước softmax, đo bằng entropy"
summary: "Chia logit cho hang so nhiet do T TRUOC khi dua vao softmax (logits/T), tren CHINH phan phoi that cua micro-transformer da huan luyen. Do entropy THAT bang cong thuc -sum(p*log(p)): T=0,1 cho entropy 0,004142 (gan giong argmax, xac suat toi da 0,999521); T=1,0 (khong doi) cho entropy 2,462297 (xac suat toi da 0,364263); T=2,0 cho entropy 3,15695 (phang hon, xac suat toi da 0,139247). Ba entropy tang DON DIEU theo T, do THAT khong suy doan. Nhiet do KHONG doi token co xac suat cao nhat (argmax giu nguyen o ca ba T) -- chi doi DO TAP TRUNG cua phan phoi quanh no."
locale: vi
track: tri-tue-nhan-tao
module: chien-luoc-giai-ma-va-lay-mau
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.nhiet-do]
requires: [ai.lay-mau-ngau-nhien-tu-softmax]
concepts: [ai.nhiet-do]
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
Lấy mẫu thô trên softmax gốc vẫn có thể chọn phải một token xác suất rất thấp. Bài này thêm một núm vặn TRƯỚC bước lấy mẫu: nhiệt độ — chia logit cho một hằng số trước khi đưa vào `softmax` — và đo bằng ENTROPY thật xem núm vặn đó thay đổi phân phối thế nào.
::::

::::explain{#nhiet_do_va_entropy}
**Nhiệt độ** (temperature, ký hiệu `T`) là một hằng số dương chia vào logit TRƯỚC khi đưa vào `softmax`: thay vì `softmax(z)`, tính `softmax(z/T)`. Ba vùng giá trị của `T`:

> **`T = 1,0`** — không đổi gì cả; `z/1,0 = z`, phân phối y hệt `softmax(z)` gốc.
>
> **`T < 1,0`** (ví dụ `0,1`) — chia cho một số NHỎ HƠN `1` khuếch đại KHOẢNG CÁCH tương đối giữa các logit trước khi mũ hoá, làm phân phối "nhọn" hơn — xác suất dồn mạnh hơn vào token có logit cao nhất, gần giống `argmax` hơn.
>
> **`T > 1,0`** (ví dụ `2,0`) — chia cho một số LỚN HƠN `1` thu hẹp khoảng cách tương đối, làm phân phối "phẳng" hơn — xác suất trải đều hơn giữa nhiều token.

Để ĐO "nhọn" hay "phẳng" bằng một con số, dùng **entropy** — công thức kinh điển từ lý thuyết thông tin: `H(p) = -sum(p_i * log(p_i))`. Entropy thấp nghĩa là phân phối tập trung (gần một token chắc chắn, `H → 0` khi một `p_i → 1`); entropy cao nghĩa là phân phối trải đều (`H` đạt giá trị lớn nhất khi mọi `p_i` bằng nhau). Bài này tính entropy THẬT — không suy luận công thức — trên phân phối softmax thật của micro-transformer đã huấn luyện, ở ba giá trị `T`.

Quan trọng: chia logit cho `T` là một phép biến đổi ĐƠN ĐIỆU (không đổi thứ tự tương đối giữa các logit — nếu `z_i > z_j` thì `z_i/T > z_j/T` với mọi `T > 0`). Vì vậy nhiệt độ KHÔNG BAO GIỜ đổi token nào có xác suất cao nhất — nó chỉ đổi ĐỘ CHÊNH LỆCH giữa token đó và phần còn lại.
::::

::::example{#entropy_ba_nhiet_do}
Lấy logit thật ở vị trí cuối của prompt `"trans"` (từ micro-transformer đã huấn luyện `15` bước), rồi tính entropy ở ba nhiệt độ:

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


def sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        token_moi = int(np.argmax(logits.data[-1]))
        ids.append(token_moi)
    return ids


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


def tinh_entropy(phan_phoi):
    return float(-np.sum(phan_phoi * np.log(phan_phoi)))


def phan_phoi_nhiet_do(logits_vec, T):
    z = logits_vec / T
    z_shift = z - np.max(z)
    e = np.exp(z_shift)
    return e / np.sum(e)


logits_prompt = forward(mo_hinh, ids_mo_dau, dim)
z_cuoi = logits_prompt.data[-1]

for T in [0.1, 1.0, 2.0]:
    p_T = phan_phoi_nhiet_do(z_cuoi, T)
    print(T, round(tinh_entropy(p_T), 6), round(float(p_T.sum()), 6), round(float(p_T.max()), 6))
```

```text title=readonly
0.1 0.004142 1.0 0.999521
1.0 2.462297 1.0 0.364263
2.0 3.15695 1.0 0.139247
```

Cùng MỘT phân phối gốc (cùng `z_cuoi`, logit thật của micro-transformer), ba nhiệt độ khác nhau: `T=0,1` cho entropy `0,004142` — cực thấp, xác suất tối đa `0,999521` gần như chắc chắn, gần giống `argmax`. `T=1,0` (không đổi) cho entropy `2,462297`. `T=2,0` cho entropy `3,15695` — cao nhất, xác suất tối đa chỉ còn `0,139247`, trải đều hơn nhiều. Ba entropy tăng ĐƠN ĐIỆU theo `T` — đúng như lý thuyết dự đoán, đo bằng số thật.
::::

::::predict{#doan_entropy_cao_thap commitOnce}
Entropy đo mức độ "không chắc chắn" của một phân phối xác suất — càng gần đều, entropy càng cao; càng tập trung vào một giá trị, entropy càng thấp.

**Trước khi chạy thử**, bạn đoán: so giữa `T=0,1` và `T=2,0` trên CÙNG một phân phối gốc, nhiệt độ nào cho entropy CAO HƠN?

:::opt{correct}
`T=2,0` cho entropy cao hơn — chia logit cho một số LỚN thu hẹp khoảng cách tương đối giữa chúng, khiến `softmax` cho ra xác suất gần đều hơn giữa các token, và một phân phối gần đều luôn có entropy cao hơn một phân phối tập trung
:::

:::opt
`T=0,1` cho entropy cao hơn — chia cho một số NHỎ khuếch đại độ lớn của mọi logit lên nhiều lần, và giá trị lớn hơn thì mang nhiều "thông tin" hơn, nên entropy phải cao hơn
::why
Gần đúng ở việc quan sát đúng: chia cho `T=0,1` (tức nhân `10`) THẬT SỰ làm độ lớn tuyệt đối của logit tăng lên.

Chỗ lệch: entropy không đo ĐỘ LỚN của các con số logit — nó đo mức ĐỀU hay LỆCH của phân phối XÁC SUẤT sau `softmax`. Khuếch đại khoảng cách tương đối giữa các logit làm `softmax` (một hàm mũ) đẩy xác suất dồn MẠNH HƠN vào token có logit cao nhất — kết quả là một phân phối TẬP TRUNG hơn (gần một điểm chắc chắn), tức entropy THẤP hơn, không phải cao hơn.
::
:::

:::opt
Cả hai bằng nhau — nhiệt độ chỉ nhân/chia đều lên mọi giá trị, không đổi HÌNH DẠNG tương đối của phân phối nên entropy phải không đổi
::why
Gần đúng ở việc để ý rằng chia cho `T` là một phép biến đổi ĐƠN ĐIỆU — nó THẬT SỰ không đổi token nào có logit cao nhất (thứ tự tương đối được giữ nguyên, đã nói ở phần giải thích).

Chỗ lệch: giữ nguyên THỨ TỰ không có nghĩa giữ nguyên KHOẢNG CÁCH tương đối — và `softmax` (một hàm mũ phi tuyến) rất nhạy với khoảng cách đó. Chia cho `T` khác `1,0` co giãn khoảng cách giữa các logit KHÔNG đều (phi tuyến), nên hình dạng của phân phối xác suất sau `softmax` thực sự đổi — bằng số đã đo được ở trên: `0,004142` so với `3,15695` khác nhau rất xa, không hề bằng nhau.
::
:::
::::

::::code{#viet_phan_phoi_nhiet_do}
Hoàn thiện `phan_phoi_nhiet_do`: chia vector logit cho nhiệt độ `T` TRƯỚC khi trừ max, rồi chuẩn hoá kết quả `exp` thành một phân phối xác suất hợp lệ (tổng bằng `1`).

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


def sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        token_moi = int(np.argmax(logits.data[-1]))
        ids.append(token_moi)
    return ids


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


def tinh_entropy(phan_phoi):
    return float(-np.sum(phan_phoi * np.log(phan_phoi)))


def phan_phoi_nhiet_do(logits_vec, T):
    z = logits_vec ___                          # / T
    z_shift = z - np.max(z)
    e = np.exp(z_shift)
    return e ___ np.sum(e)                       # /


logits_prompt = forward(mo_hinh, ids_mo_dau, dim)
z_cuoi = logits_prompt.data[-1]

for T in [0.1, 1.0, 2.0]:
    p_T = phan_phoi_nhiet_do(z_cuoi, T)
    print(T, round(tinh_entropy(p_T), 6), round(float(p_T.sum()), 6), round(float(p_T.max()), 6))
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


def sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        token_moi = int(np.argmax(logits.data[-1]))
        ids.append(token_moi)
    return ids


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


def tinh_entropy(phan_phoi):
    return float(-np.sum(phan_phoi * np.log(phan_phoi)))


def phan_phoi_nhiet_do(logits_vec, T):
    z = logits_vec / T
    z_shift = z - np.max(z)
    e = np.exp(z_shift)
    return e / np.sum(e)


logits_prompt = forward(mo_hinh, ids_mo_dau, dim)
z_cuoi = logits_prompt.data[-1]

for T in [0.1, 1.0, 2.0]:
    p_T = phan_phoi_nhiet_do(z_cuoi, T)
    print(T, round(tinh_entropy(p_T), 6), round(float(p_T.sum()), 6), round(float(p_T.max()), 6))
```

```python title=test
p_01 = phan_phoi_nhiet_do(z_cuoi, 0.1)
p_10 = phan_phoi_nhiet_do(z_cuoi, 1.0)
p_20 = phan_phoi_nhiet_do(z_cuoi, 2.0)

assert round(tinh_entropy(p_01), 6) == 0.004142, f"entropy T=0.1 sai -- dang ra {round(tinh_entropy(p_01), 6)}"
assert round(tinh_entropy(p_10), 6) == 2.462297, f"entropy T=1.0 sai -- dang ra {round(tinh_entropy(p_10), 6)}"
assert round(tinh_entropy(p_20), 6) == 3.15695, f"entropy T=2.0 sai -- dang ra {round(tinh_entropy(p_20), 6)}"

# bang chung trung tam: entropy TANG DON DIEU theo T
assert tinh_entropy(p_01) < tinh_entropy(p_10) < tinh_entropy(p_20), "entropy phai tang don dieu theo T (T=0.1 < T=1.0 < T=2.0)"

# bien: tong xac suat phai la 1.0 o CA BA nhiet do (van la mot phan phoi
# hop le sau khi chia logit cho T)
for p_kiem in [p_01, p_10, p_20]:
    assert round(float(p_kiem.sum()), 6) == 1.0, f"tong xac suat phai la 1.0 -- dang ra {round(float(p_kiem.sum()), 6)}"

# bien bien gioi han: nhiet do KHONG doi token co xac suat cao nhat (phep
# bien doi don dieu, argmax phai giu nguyen o ca ba T)
assert int(np.argmax(p_01)) == int(np.argmax(p_10)) == int(np.argmax(p_20)), "argmax phai giu nguyen bat ke nhiet do"

# bien: T=1.0 phai giong het softmax KHONG doi (khong chia gi ca)
zs_goc = z_cuoi - np.max(z_cuoi)
e_goc = np.exp(zs_goc)
p_goc = e_goc / np.sum(e_goc)
assert np.allclose(p_10, p_goc), "T=1.0 phai cho DUNG phan phoi softmax goc, khong doi gi ca"
```

:::hints
- kind: attention
  body: Hai chỗ trống trong `phan_phoi_nhiet_do`. Chỗ đầu — `z = logits_vec ___` — phải CHIA vector logit cho nhiệt độ `T` (tham số thứ hai của hàm), TRƯỚC khi trừ max. Chỗ hai — `return e ___ np.sum(e)` — phải CHIA `e` cho tổng của nó để chuẩn hoá thành một phân phối xác suất hợp lệ (giống hệt bước cuối của `Tensor.softmax`).
- kind: strategy
  body: 'Chỗ đầu: `/ T` — hoàn thiện thành `z = logits_vec / T`. Chỗ hai: `/` — hoàn thiện thành `return e / np.sum(e)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `/ T` và `/`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: 'phan_phoi_nhiet_do phai CHIA logits_vec cho T (khong duoc bo qua nhiet do hay hardcode T=1) VA phai CHUAN HOA e bang phep CHIA cho np.sum(e) (khong duoc tra ve e chua chuan hoa)'
  requireAst:
  - kind: uses-operator, target: "/", min: 13
  # Da thu THAT bang kiemAst (goi truc tiep tren code trich tu solution da
  # bien dich, khong doan tay).
  # "/"=13: TONG THAT gom 11 lan CO SAN rai rac trong boilerplate (Tensor.
  # softmax, Tensor.layernorm, ma_hoa_vi_tri, khoi_transformer,
  # mat_mat_du_doan_tiep_theo, forward...) CONG 2 lan trong hai blank cua
  # phan_phoi_nhiet_do (logits_vec / T, e / np.sum(e)). Dien bua MOT trong
  # hai blank thanh mot toan tu khac (vd "z = logits_vec * T" hay "return
  # e - np.sum(e)") lam so nay tut xuong 12 -- duoi nguong min=13, bi chan
  # boi static; dong thoi bi chan boi tests vi entropy/tong xac suat sai
  # (GOTCHA "boilerplate-threshold-masking": vi "/" xuat hien rat nhieu
  # trong boilerplate, dat min=1 ngay tho se KHONG bat duoc mutant xoa MOT
  # trong hai blank -- phai dat dung TONG THAT 13 moi bat duoc).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^0\\.1 0\\.004142 1\\.0 0\\.999521\\n1\\.0 2\\.462297 1\\.0 0\\.364263\\n2\\.0 3\\.15695 1\\.0 0\\.139247\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Entropy `0,004142 → 2,462297 → 3,15695` khi `T` tăng `0,1 → 1,0 → 2,0` — đo thật, tăng đơn điệu. `argmax` không đổi ở cả ba nhiệt độ — nhiệt độ chỉ đổi độ TẬP TRUNG, không đổi token "tốt nhất". Bài sau cắt phân phối theo một cách khác hẳn: chỉ GIỮ LẠI k xác suất cao nhất, đặt phần còn lại về `0`.
::::

::::reflect{#nghi-lai}
Nhiệt độ là một núm vặn TOÀN CỤC — nó co giãn MỌI xác suất theo cùng một quy luật hàm mũ, không phân biệt token nào. Nhưng có một vấn đề nó không giải quyết được: dù `T` cao hay thấp, phần ĐUÔI của phân phối (những token xác suất rất nhỏ) vẫn luôn có một cơ hội khác `0` để bị chọn — với `T` cao, cơ hội đó thậm chí còn TĂNG lên. Bài sau (top-k) và bài sau nữa (top-p) giải quyết đúng vấn đề đó theo một hướng khác: CẮT BỎ hẳn phần đuôi, chỉ giữ lại một tập con các token có xác suất cao nhất, rồi mới lấy mẫu trên tập con đã chuẩn hoá lại đó.
::::

::::checkpoint{mastery=0.8}
::::
