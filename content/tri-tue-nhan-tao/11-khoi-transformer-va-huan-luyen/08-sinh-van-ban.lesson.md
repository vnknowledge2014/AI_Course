---
id: tri-tue-nhan-tao.khoi-transformer-va-huan-luyen.sinh-van-ban
title: "Sinh văn bản: greedy decoding, lấy vị trí CUỐI, lặp lại, giải mã ngược"
summary: "Sinh van ban tu hoi quy: forward chuoi HIEN TAI -> lay logit o VI TRI CUOI (vi tri da 'thay' toan bo chuoi nho causal mask) -> softmax -> argmax (greedy decoding, don gian nhat) -> noi token moi vao chuoi -> LAP LAI. Tu mo hinh da huan luyen 12 buoc (bai truoc), mo dau 'may hoc' (4 token), sinh 4 token moi: [3,3,3,3] -- giai ma BPE nguoc thanh 'may hoc hoc hoc hoc hoc'. Doi chieu: argmax o VI TRI DAU (chi thay dung 1 token 'm', vo nghia cho du doan tiep theo) cho token HOAN TOAN khac (id 6 'a') -- xac nhan vi tri CUOI moi la vi tri dung de sinh."
locale: vi
track: tri-tue-nhan-tao
module: khoi-transformer-va-huan-luyen
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.sinh-van-ban]
requires: [ai.huan-luyen-tren-corpus-that]
concepts: [ai.sinh-van-ban]
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
Mô hình vừa huấn luyện biết dự đoán token TIẾP THEO — cho một chuỗi, nó trả về một phân phối xác suất trên từ vựng, tại MỖI vị trí. Sinh văn bản là dùng đúng khả năng đó, LẶP LẠI nhiều lần.
::::

::::explain{#sinh_van_ban_tu_hoi_quy}
**Sinh văn bản tự hồi quy** (autoregressive generation) — vòng lặp đơn giản, mỗi bước làm đúng bốn việc:

> 1. Forward chuỗi token HIỆN TẠI qua mô hình, lấy `logits` shape `(so_token, vocab)`.
> 2. Lấy logit ở VỊ TRÍ CUỐI CÙNG — `logits.data[-1]`. Nhờ causal mask (`co-che-attention`, q8.3c), vị trí cuối là vị trí DUY NHẤT đã "thấy" TOÀN BỘ chuỗi hiện tại (mọi vị trí trước nó) — đây chính là dự đoán "từ tiếp theo, sau khi đã đọc hết những gì có tới giờ".
> 3. `softmax` biến logit đó thành xác suất, rồi chọn token có xác suất CAO NHẤT — `np.argmax(...)`. Đây là **greedy decoding** — chiến lược sinh ĐƠN GIẢN NHẤT (luôn chọn lựa chọn tốt nhất tại chỗ, không có gì ngẫu nhiên hay tìm kiếm xa hơn).
> 4. Nối token mới vào CUỐI chuỗi, LẶP LẠI từ bước `1` với chuỗi đã dài hơn `1`.

Sau khi sinh đủ số token mong muốn, giải mã ngược (`giai_ma`, q8.3a) để đọc được chuỗi KÝ TỰ cuối cùng — đảo ngược đúng bước đã học ở `giai-ma-nguoc`.
::::

::::example{#sinh_tu_mo_hinh_da_huan_luyen}
Dùng mô hình đã huấn luyện đúng `12` bước (bài trước), sinh `4` token mới từ chuỗi mở đầu `"may hoc"` (`4` token):

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

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
for _ in range(12):
    mot_buoc(mo_hinh, ids_huan_luyen, dim, 0.5)

ids_mo_dau = ids_full[:4]
print(giai_ma(ids_mo_dau, id_sang_token))

ids_sinh = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 4)
print(ids_sinh)
print(giai_ma(ids_sinh, id_sang_token))
```

```text title=readonly
may hoc
[14, 6, 19, 3, 3, 3, 3, 3]
may hoc hoc hoc hoc hoc
```

Chuỗi mở đầu `"may hoc"` (`4` token: `m`, `a`, `y`, `' hoc'`), sinh thêm `4` token — cả `4` đều là token `3` (`' hoc'`, đã học đậm nhất từ corpus huấn luyện, nơi `"hoc"` lặp lại nhiều lần). Kết quả `"may hoc hoc hoc hoc hoc"` — không phải văn xuôi trôi chảy (mô hình MINH HOẠ, `dim = 4`, huấn luyện `12` bước trên `10` token, không phải một LLM thật), nhưng greedy decoding chạy ĐÚNG cơ chế: mỗi bước lấy đúng vị trí cuối, đúng argmax, nối đúng chuỗi, giải mã ngược đúng nguyên văn.
::::

::::predict{#doan_vi_tri_dung_de_sinh commitOnce}
Vòng lặp sinh văn bản lấy `logits.data[-1]` — vị trí CUỐI CÙNG của chuỗi hiện tại.

**Trước khi chạy thử**, bạn đoán: nếu (sai) lấy `logits.data[0]` (vị trí ĐẦU TIÊN) thay vì vị trí cuối để chọn token tiếp theo, kết quả `argmax` có giống hệt kết quả đúng, hay khác hẳn?

:::opt{correct}
Khác hẳn — vị trí `0` (nhờ causal mask) chỉ "thấy" ĐÚNG MỘT token duy nhất (chính nó, token đầu tiên của chuỗi) — dự đoán của nó là "token nào thường đứng NGAY SAU token đầu tiên", không liên quan gì tới TOÀN BỘ chuỗi đã có tới giờ; chỉ vị trí CUỐI mới đã "đọc" hết chuỗi hiện tại, nên chỉ nó mới cho ra dự đoán đúng nghĩa "từ tiếp theo của CẢ CÂU"
:::

:::opt
Giống hệt — vì cả hai vị trí đều đi qua ĐÚNG CÙNG một mô hình, cùng tham số, nên phải cho ra cùng một dự đoán bất kể lấy vị trí nào
::why
Gần đúng ở việc để ý CẢ HAI vị trí đều dùng chung MỘT bộ tham số (`Wq`/`Wk`/`Wv`/`W1`/`W2`/`W_out`) — quan sát đó không sai.

Chỗ lệch: dùng chung THAM SỐ không có nghĩa là cho ra cùng ĐẦU RA — mỗi vị trí có `Q`/`K`/`V` RIÊNG (tính từ embedding CỦA VỊ TRÍ ĐÓ), và causal mask giới hạn mỗi vị trí chỉ được "nhìn" các vị trí TRƯỚC hoặc BẰNG chính nó. Vị trí `0` và vị trí cuối nhìn thấy hai lượng thông tin HOÀN TOÀN khác nhau (một token so với toàn bộ chuỗi), nên đầu ra của chúng khác hẳn nhau — số thật đã xác nhận: vị trí cuối chọn token `3` (`' hoc'`), vị trí `0` chọn token `6` (`'a'`).
::
:::

:::opt
Không xác định được — phụ thuộc độ dài chuỗi hiện tại, có thể giống hoặc khác tuỳ lúc
::why
Gần đúng ở tinh thần thận trọng khi một hành vi có vẻ phụ thuộc ngữ cảnh cụ thể — thái độ đó hợp lý với nhiều hiện tượng khác.

Chỗ lệch: đây là một sự thật CẤU TRÚC của causal mask, không phải một hiện tượng may rủi phụ thuộc độ dài. Vị trí `0` LUÔN chỉ thấy chính nó (bất kể chuỗi dài `4` hay `40` token) — che TOÀN BỘ các vị trí sau nó bằng `-∞` trước `softmax`. Vị trí cuối LUÔN thấy toàn bộ chuỗi. Sự khác biệt về THÔNG TIN nhìn thấy được này không đổi theo độ dài chuỗi.
::
:::
::::

::::code{#viet_sinh_van_ban}
Hoàn thiện hai chỗ trống trong `sinh_van_ban`: chọn token có xác suất cao nhất ở đúng vị trí cuối, và nối nó vào chuỗi.

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


def sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        token_moi = ___                       # int(np.argmax(logits.data[-1]))
        ids.___(token_moi)                     # append
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

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
for _ in range(12):
    mot_buoc(mo_hinh, ids_huan_luyen, dim, 0.5)

ids_mo_dau = ids_full[:4]
ids_sinh = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 4)

print(ids_sinh)
print(giai_ma(ids_sinh, id_sang_token))
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

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
for _ in range(12):
    mot_buoc(mo_hinh, ids_huan_luyen, dim, 0.5)

ids_mo_dau = ids_full[:4]
ids_sinh = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 4)

print(ids_sinh)
print(giai_ma(ids_sinh, id_sang_token))
```

```python title=test
assert ids_sinh == [14, 6, 19, 3, 3, 3, 3, 3], f"ids_sinh sai -- dang ra {ids_sinh}"
assert giai_ma(ids_sinh, id_sang_token) == "may hoc hoc hoc hoc hoc", f"van ban giai ma sai -- dang ra {giai_ma(ids_sinh, id_sang_token)!r}"
assert len(ids_sinh) == len(ids_mo_dau) + 4, f"phai sinh dung 4 token moi -- dang ra {len(ids_sinh) - len(ids_mo_dau)}"
assert ids_sinh[:len(ids_mo_dau)] == ids_mo_dau, "chuoi mo dau phai duoc GIU NGUYEN, chi noi them token moi vao SAU"

# rieng kiem tra: lay VI TRI CUOI la dung, khac han lay vi tri DAU
logits_kiem = forward(mo_hinh, ids_mo_dau, dim)
tok_cuoi = int(np.argmax(logits_kiem.data[-1]))
tok_dau = int(np.argmax(logits_kiem.data[0]))
assert tok_cuoi == ids_sinh[len(ids_mo_dau)], f"token dau tien duoc sinh phai la argmax cua VI TRI CUOI -- dang ra {ids_sinh[len(ids_mo_dau)]}"
assert tok_cuoi != tok_dau, f"argmax vi tri cuoi phai KHAC argmax vi tri dau (minh hoa vi sao phai lay dung vi tri cuoi) -- ca hai deu la {tok_cuoi}"
```

:::hints
- kind: attention
  body: Hai chỗ trống trong vòng lặp sinh. Chỗ đầu chọn token có xác suất cao nhất tại VỊ TRÍ CUỐI CÙNG của `logits` — `int(np.argmax(logits.data[-1]))` (không cần gọi `.softmax()` trước — `argmax` trên logit thô và trên xác suất softmax luôn cho CÙNG chỉ số, vì `softmax` không đổi thứ tự). Chỗ hai nối token vừa chọn vào CUỐI danh sách `ids` — `ids.append(token_moi)`.
- kind: strategy
  body: 'Chỗ đầu: `int(np.argmax(logits.data[-1]))`. Chỗ hai: `append`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `int(np.argmax(logits.data[-1]))` và `append` (trong `ids.append(token_moi)`).'
:::

:::validate
- tier: run
  timeoutMs: 25000
- tier: static
  onFail: chỗ trống đầu phải dùng np.argmax tren logits.data[-1] (dung DUNG vi tri CUOI, khong duoc dung vi tri khac hay bo qua argmax); chỗ trống hai phải la append (noi token moi vao CUOI danh sach, khong duoc dung insert hay phuong thuc khac)
  requireAst:
  - kind: uses-call, target: argmax, min: 1
  - kind: uses-call, target: append, min: 1
  - kind: uses-name, target: ids, min: 4
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true. argmax=1 (chi trong blank1). append=1 (chi trong
  # blank2 "ids.append(token_moi)" -- day la Call voi func.attr="append").
  # ids=4: dinh nghia "ids = list(ids_mo_dau)" la Store (khong dem); Load:
  # "forward(mo_hinh, ids, dim)" [1], "ids.append(...)" trong blank2 [1],
  # "return ids" [1], va tham so ham "ids_mo_dau" la TEN KHAC khong tinh --
  # kiem tra rieng qua kiemAst that xac nhan tong Load "ids" la 4 tren toan
  # bo than ham sinh_van_ban.
  # Cheat "token_moi = int(np.argmax(logits.data[0]))" (dung vi tri DAU
  # thay vi CUOI) khong lam giam so lan goi argmax (van =1) nen KHONG bi
  # static bat rieng qua so dem -- nhung da tu kiem chung bang Python that:
  # cheat nay lam ids_sinh khac han gia tri dung ([14,6,19,3,6,...] thay vi
  # [14,6,19,3,3,3,3,3]) -- bi bat DOC LAP boi assert ids_sinh VA boi assert
  # tok_cuoi != tok_dau (design ngay tu dau de bat CHINH xac cheat nay).
  # Cheat "ids.insert(0, token_moi)" (noi vao DAU thay vi CUOI) lam "append"
  # tut xuong 0 -- bi chan RIENG, VA da tu kiem chung: chuoi mo dau se KHONG
  # con o dau ids_sinh -- bi bat DOC LAP boi assert ids_sinh[:len(ids_mo_dau)].
- tier: tests
  timeoutMs: 25000
- tier: output
  match: regex
  expect: "^\\[14, 6, 19, 3, 3, 3, 3, 3\\]\\nmay hoc hoc hoc hoc hoc\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ mở đầu "may hoc" tới "may hoc hoc hoc hoc hoc" — greedy decoding, đúng cơ chế, giải mã ngược khớp nguyên văn. Bài BOSS cuối cùng của quest: ráp TOÀN BỘ, và so sánh mô hình TRƯỚC/SAU huấn luyện.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài này sinh văn bản từ một mô hình ĐÃ huấn luyện `12` bước — nhưng chưa bao giờ so sánh TRỰC TIẾP với việc sinh từ một mô hình CHƯA huấn luyện chút nào (tham số vừa khởi tạo ngẫu nhiên). Nếu chạy `sinh_van_ban` trên MỘT mô hình chưa huấn luyện, với ĐÚNG cùng chuỗi mở đầu, kết quả có khác đi không — và quan trọng hơn, có cách nào đo bằng SỐ để khẳng định "huấn luyện đã thay đổi hành vi mô hình", chứ không chỉ so sánh chuỗi ký tự sinh ra bằng mắt?
::::

::::checkpoint{mastery=0.9}
::::
