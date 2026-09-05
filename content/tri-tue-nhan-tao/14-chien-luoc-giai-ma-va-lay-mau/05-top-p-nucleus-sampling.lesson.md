---
id: tri-tue-nhan-tao.chien-luoc-giai-ma-va-lay-mau.top-p-nucleus-sampling
title: "Top-p (nucleus sampling) — ngưỡng xác suất, số token giữ lại ĐỘNG"
summary: "Top-p (nucleus sampling): sap xac suat giam dan, CONG DON (cumsum) toi khi DAT/VUOT nguong p, giu DUNG tap NHO NHAT do, chuan hoa, lay mau. Khac top-k (so luong token CO DINH bang k), top-p giu SO LUONG DONG theo tung phan phoi: tren phan phoi that cua micro-transformer o p=0,9, T=1,0 (17 buoc entropy trung binh) giu 17 token, T=2,0 (phang hon) giu 23 token -- HAI phan phoi khac nhau, HAI so luong khac nhau, do THAT khong phai hang so. Kiem bien tai DIEM CHAM NGUONG chinh xac: mang [0,5; 0,3; 0,2] voi p=0,8 giu DUNG 2 phan tu vi tong tich luy hai phan tu dau DUNG BANG 0,8 (cham nguong, khong vuot)."
locale: vi
track: tri-tue-nhan-tao
module: chien-luoc-giai-ma-va-lay-mau
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.top-p-nucleus-sampling]
requires: [ai.top-k-sampling]
concepts: [ai.top-p-nucleus-sampling]
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
Top-k giữ một SỐ LƯỢNG token cố định — nhưng cùng một `k` có thể thừa cho phân phối nhọn, thiếu cho phân phối phẳng. Bài này (top-p, hay nucleus sampling) cố định TỔNG XÁC SUẤT cần đạt thay vì số lượng token — số lượng giữ lại trở thành một đại lượng ĐỘNG.
::::

::::explain{#top_p_va_dong}
**Top-p sampling** (còn gọi **nucleus sampling**) làm ba bước, cũng theo đúng thứ tự, nhưng bước cắt khác hẳn top-k:

> **Sắp xếp** xác suất giảm dần — giống hệt top-k, `np.argsort(phan_phoi)[::-1]`.
>
> **Cộng dồn** (cumulative sum, `np.cumsum`) các xác suất đã sắp — tại vị trí `i`, giá trị cộng dồn là tổng của `i+1` xác suất LỚN NHẤT. Tìm vị trí ĐẦU TIÊN mà tổng cộng dồn ĐẠT HOẶC VƯỢT một ngưỡng `p_nguong` cho trước (ví dụ `0,9`) — giữ ĐÚNG tập nhỏ nhất đó, đặt phần còn lại về `0`.
>
> **Chuẩn hoá lại** — giống hệt top-k, chia phần giữ lại cho tổng của chính nó.

Khác biệt cốt lõi với top-k: `k` là một con số CỐ ĐỊNH, chọn trước, không đổi giữa các phân phối. Còn SỐ LƯỢNG token top-p giữ lại — gọi nó `so_luong` — phụ thuộc HOÀN TOÀN vào HÌNH DẠNG của phân phối đang xét: một phân phối "nhọn" (vài token chiếm gần hết xác suất) chỉ cần vài token để đạt `p_nguong=0,9`; một phân phối "phẳng" (xác suất trải rộng) cần NHIỀU token hơn để đạt cùng ngưỡng đó. Cùng một `p_nguong`, hai phân phối khác nhau, hai `so_luong` khác nhau — đo bằng số, không suy luận.

Tìm vị trí đạt ngưỡng dùng `np.searchsorted(tich_luy, p_nguong)`: hàm này trả về chỉ số ĐẦU TIÊN mà giá trị mảng (đã sắp tăng dần — `tich_luy` luôn tăng dần vì mọi xác suất không âm) LỚN HƠN HOẶC BẰNG `p_nguong`. Cộng thêm `1` để chuyển từ CHỈ SỐ (đếm từ `0`) sang SỐ LƯỢNG phần tử cần giữ.
::::

::::example{#top_p_dong_that}
Áp top-p (`p_nguong=0,9`) lên CÙNG một phân phối logit thật, ở hai nhiệt độ khác nhau — số lượng token giữ lại khác nhau:

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


def phan_phoi_nhiet_do(logits_vec, T):
    z = logits_vec / T
    z_shift = z - np.max(z)
    e = np.exp(z_shift)
    return e / np.sum(e)


def top_p_phan_phoi(phan_phoi, p_nguong):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    p_sap_xep = phan_phoi[idx_sap_xep]
    tich_luy = np.cumsum(p_sap_xep)
    so_luong = int(np.searchsorted(tich_luy, p_nguong)) + 1
    idx_giu = idx_sap_xep[:so_luong]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


logits_prompt = forward(mo_hinh, ids_mo_dau, dim)
z_cuoi = logits_prompt.data[-1]

p_T1 = phan_phoi_nhiet_do(z_cuoi, 1.0)
p_T2 = phan_phoi_nhiet_do(z_cuoi, 2.0)

for ten, p in [("T=1.0", p_T1), ("T=2.0", p_T2)]:
    pp = top_p_phan_phoi(p, 0.9)
    n_giu = int(np.sum(pp > 0))
    print(ten, n_giu, round(float(pp.sum()), 6))
```

```text title=readonly
T=1.0 17 1.0
T=2.0 23 1.0
```

CÙNG một logit thật (`z_cuoi`, vị trí cuối của prompt `"trans"`), CÙNG ngưỡng `p_nguong=0,9` — nhưng hai nhiệt độ khác nhau (bài trước) cho hai phân phối khác nhau, và top-p giữ lại HAI SỐ LƯỢNG token khác nhau: `17` token ở `T=1,0`, `23` token ở `T=2,0` (nhiệt độ cao làm phân phối phẳng hơn, cần NHIỀU token hơn mới đạt cùng một ngưỡng xác suất tích luỹ). Không phải một hằng số `k` cố định — số lượng THẬT SỰ đổi theo hình dạng phân phối, đo bằng số.
::::

::::predict{#doan_bien_gioi_han_top_p commitOnce}
Xét mảng xác suất nhỏ `[0,5; 0,3; 0,2]` (đã sắp giảm dần sẵn) và ngưỡng `p_nguong=0,8`. Tổng tích luỹ là `[0,5; 0,8; 1,0]` — phần tử thứ hai (`0,8`) CHẠM ĐÚNG ngưỡng, không vượt qua nó.

**Trước khi chạy thử**, bạn đoán: `top_p_phan_phoi` sẽ giữ lại BAO NHIÊU phần tử trong trường hợp này?

:::opt{correct}
`2` phần tử — quy tắc là giữ tập nhỏ nhất mà tổng tích luỹ ĐẠT HOẶC VƯỢT ngưỡng; tổng của `2` phần tử đầu (`0,5 + 0,3 = 0,8`) đã CHẠM đúng `0,8`, tức đã ĐẠT ngưỡng, nên không cần phần tử thứ ba
:::

:::opt
`1` phần tử — vì phần tử đầu tiên (`0,5`) đã là phần lớn nhất, và ngưỡng `0,8` không "vượt hẳn" qua nó nên chỉ cần giữ một phần tử
::why
Gần đúng ở việc phần tử ĐẦU TIÊN thực sự là phần tử LỚN NHẤT sau khi sắp xếp — quan sát đó đúng.

Chỗ lệch: điều kiện dừng không phải "phần tử đầu có lớn hay không" — nó là "tổng TÍCH LUỸ của các phần tử đã giữ có đạt ngưỡng `0,8` chưa". Tổng tích luỹ sau MỘT phần tử chỉ là `0,5` — nhỏ hơn `0,8` — nên CHƯA đạt ngưỡng, còn phải giữ thêm phần tử tiếp theo cho tới khi tổng đạt hoặc vượt `0,8`.
::
:::

:::opt
`3` phần tử (toàn bộ mảng) — vì `0,8` là một ngưỡng cao, gần với `1,0` (tổng của cả mảng), nên cần gần như MỌI phần tử mới đạt được
::why
Gần đúng ở trực giác "ngưỡng càng gần `1,0` thì càng cần nhiều phần tử" — trực giác đó ĐÚNG nói chung (so với một ngưỡng thấp hơn, ví dụ `0,5`, cần ít phần tử hơn).

Chỗ lệch: "gần `1,0`" không có nghĩa là CHƯA đạt được — phải kiểm tra CHÍNH XÁC bằng phép cộng: `0,5 + 0,3 = 0,8`, và `0,8 >= 0,8` là ĐÚNG (đạt ngưỡng, không cần "vượt hẳn" qua nó). Chỉ cần phần tử thứ ba (đưa tổng lên `1,0`) khi tổng SAU HAI phần tử vẫn còn NHỎ HƠN ngưỡng — ở đây không phải trường hợp đó.
::
:::
::::

::::code{#viet_top_p_phan_phoi}
Hoàn thiện `top_p_phan_phoi`: cộng dồn ĐÚNG mảng xác suất đã sắp xếp, rồi chuyển chỉ số tìm được thành SỐ LƯỢNG phần tử cần giữ (cộng thêm `1`).

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


def phan_phoi_nhiet_do(logits_vec, T):
    z = logits_vec / T
    z_shift = z - np.max(z)
    e = np.exp(z_shift)
    return e / np.sum(e)


def top_p_phan_phoi(phan_phoi, p_nguong):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    p_sap_xep = phan_phoi[idx_sap_xep]
    tich_luy = np.cumsum(___)                    # p_sap_xep
    so_luong = int(np.searchsorted(tich_luy, p_nguong)) ___    # + 1
    idx_giu = idx_sap_xep[:so_luong]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


logits_prompt = forward(mo_hinh, ids_mo_dau, dim)
z_cuoi = logits_prompt.data[-1]

p_T1 = phan_phoi_nhiet_do(z_cuoi, 1.0)
p_T2 = phan_phoi_nhiet_do(z_cuoi, 2.0)

for ten, p in [("T=1.0", p_T1), ("T=2.0", p_T2)]:
    pp = top_p_phan_phoi(p, 0.9)
    n_giu = int(np.sum(pp > 0))
    print(ten, n_giu, round(float(pp.sum()), 6))
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


def phan_phoi_nhiet_do(logits_vec, T):
    z = logits_vec / T
    z_shift = z - np.max(z)
    e = np.exp(z_shift)
    return e / np.sum(e)


def top_p_phan_phoi(phan_phoi, p_nguong):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    p_sap_xep = phan_phoi[idx_sap_xep]
    tich_luy = np.cumsum(p_sap_xep)
    so_luong = int(np.searchsorted(tich_luy, p_nguong)) + 1
    idx_giu = idx_sap_xep[:so_luong]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


logits_prompt = forward(mo_hinh, ids_mo_dau, dim)
z_cuoi = logits_prompt.data[-1]

p_T1 = phan_phoi_nhiet_do(z_cuoi, 1.0)
p_T2 = phan_phoi_nhiet_do(z_cuoi, 2.0)

for ten, p in [("T=1.0", p_T1), ("T=2.0", p_T2)]:
    pp = top_p_phan_phoi(p, 0.9)
    n_giu = int(np.sum(pp > 0))
    print(ten, n_giu, round(float(pp.sum()), 6))
```

```python title=test
assert int(np.sum(top_p_phan_phoi(p_T1, 0.9) > 0)) == 17, f"T=1.0, p=0.9 phai giu DUNG 17 token -- dang ra {int(np.sum(top_p_phan_phoi(p_T1, 0.9) > 0))}"
assert int(np.sum(top_p_phan_phoi(p_T2, 0.9) > 0)) == 23, f"T=2.0, p=0.9 phai giu DUNG 23 token -- dang ra {int(np.sum(top_p_phan_phoi(p_T2, 0.9) > 0))}"

# bang chung trung tam: HAI phan phoi khac nhau (T=1.0 va T=2.0) phai cho
# HAI SO LUONG token khac nhau -- khong phai mot hang so co dinh
assert int(np.sum(top_p_phan_phoi(p_T1, 0.9) > 0)) != int(np.sum(top_p_phan_phoi(p_T2, 0.9) > 0)), "top-p phai giu SO LUONG token KHAC NHAU tren hai phan phoi khac nhau"

# tong xac suat sau chuan hoa phai la 1.0
assert round(float(top_p_phan_phoi(p_T1, 0.9).sum()), 6) == 1.0, "tong xac suat sau chuan hoa (T=1.0) phai la 1.0"
assert round(float(top_p_phan_phoi(p_T2, 0.9).sum()), 6) == 1.0, "tong xac suat sau chuan hoa (T=2.0) phai la 1.0"

# bien bien gioi han QUAN TRONG NHAT: mang nho, nguong CHAM DUNG tai bien
# tich luy (0.5 + 0.3 = 0.8 dung bang nguong 0.8, khong vuot) -- phai giu
# DUNG 2 phan tu, khong phai 1 (chua du) hay 3 (thua)
p_nho = np.array([0.5, 0.3, 0.2])
pp_nho = top_p_phan_phoi(p_nho, 0.8)
assert int(np.sum(pp_nho > 0)) == 2, f"nguong cham dung tai bien tich luy phai giu DUNG 2 phan tu -- dang ra {int(np.sum(pp_nho > 0))}"
assert round(float(pp_nho[0]), 6) == 0.625, f"phan tu dau sau chuan hoa sai -- dang ra {round(float(pp_nho[0]), 6)}"
assert round(float(pp_nho[1]), 6) == 0.375, f"phan tu hai sau chuan hoa sai -- dang ra {round(float(pp_nho[1]), 6)}"
assert pp_nho[2] == 0.0, f"phan tu bi loai phai co xac suat DUNG 0 -- dang ra {pp_nho[2]}"

# bien: nguong RAT THAP (0.5) -- cham dung tai PHAN TU DAU TIEN, phai giu
# DUNG 1 phan tu (khong phai 0 hay 2)
pp_nho_thap = top_p_phan_phoi(p_nho, 0.5)
assert int(np.sum(pp_nho_thap > 0)) == 1, f"nguong 0.5 cham dung phan tu dau phai giu DUNG 1 -- dang ra {int(np.sum(pp_nho_thap > 0))}"

# bien: nguong RAT CAO (0.999, gan 1.0) -- can CA BA phan tu moi dat duoc
pp_nho_cao = top_p_phan_phoi(p_nho, 0.999)
assert int(np.sum(pp_nho_cao > 0)) == 3, f"nguong 0.999 phai can ca 3 phan tu -- dang ra {int(np.sum(pp_nho_cao > 0))}"
```

:::hints
- kind: attention
  body: Hai chỗ trống trong `top_p_phan_phoi`. Chỗ đầu — `tich_luy = np.cumsum(___)` — phải cộng dồn mảng xác suất ĐÃ SẮP XẾP giảm dần (`p_sap_xep`, không phải `phan_phoi` gốc chưa sắp). Chỗ hai — `so_luong = int(np.searchsorted(tich_luy, p_nguong)) ___` — `searchsorted` trả về một CHỈ SỐ (đếm từ `0`); cộng thêm `1` để có SỐ LƯỢNG phần tử cần giữ.
- kind: strategy
  body: 'Chỗ đầu: `p_sap_xep` — hoàn thiện thành `np.cumsum(p_sap_xep)`. Chỗ hai: `+ 1` — hoàn thiện thành `int(np.searchsorted(tich_luy, p_nguong)) + 1`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `p_sap_xep` và `+ 1`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: 'np.cumsum(...) phai duoc goi tren p_sap_xep (mang DA SAP XEP giam dan, khong duoc goi tren phan_phoi goc chua sap); so_luong phai CONG THEM 1 THAT sau searchsorted (khong duoc bo qua, se lam thieu dung 1 token o moi truong hop)'
  requireAst:
  - kind: uses-name, target: p_sap_xep, min: 1
  - kind: uses-operator, target: "+", min: 33
  # Da thu THAT bang kiemAst (goi truc tiep tren code trich tu solution da
  # bien dich, khong doan tay).
  # p_sap_xep=1: CHI mot lan xuat hien trong toan bo file, dung trong blank1
  # (np.cumsum(p_sap_xep)) -- ten nay khong trung voi bat ky bien nao khac
  # trong boilerplate. Dien bua blank1 thanh "np.cumsum(phan_phoi)" (cong
  # don tren mang CHUA sap xep -- sai logic, nhung van chay duoc khong loi)
  # lam so nay tut xuong 0 -- duoi nguong min=1, bi chan boi static; dong
  # thoi bi chan boi tests vi ket qua so_luong se sai tren moi phan phoi
  # khong doi xung.
  # "+"=33: TONG THAT gom 32 lan CO SAN rai rac trong boilerplate (rat
  # nhieu -- phep cong xuat hien khap Tensor, BPE, transformer) CONG 1 lan
  # trong blank2 (+ 1). Dien bua blank2 thanh bo het phan "+ 1" (chi con
  # "so_luong = int(np.searchsorted(tich_luy, p_nguong))") lam so nay tut
  # xuong 32 -- duoi nguong min=33, bi chan boi static (GOTCHA
  # "boilerplate-threshold-masking": "+" qua pho bien nen phai dat DUNG
  # TONG THAT 33, khong duoc mac dinh min=1); dong thoi bi chan boi tests
  # vi moi truong hop se thieu dung 1 token (vi du bien 0.8 se giu 1 thay
  # vi 2, T=1.0 se giu 16 thay vi 17).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^T=1\\.0 17 1\\.0\\nT=2\\.0 23 1\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`T=1,0` giữ `17` token, `T=2,0` giữ `23` token — CÙNG ngưỡng `0,9`, HAI số lượng khác nhau, đo thật. Trường hợp biên `[0,5; 0,3; 0,2]` với ngưỡng `0,8`: chạm ĐÚNG tại `2` phần tử, không thừa không thiếu. Bài cuối (BOSS) chạy CẢ BỐN chiến lược — greedy, lấy mẫu thô, top-k, top-p — trên CÙNG một mô hình, đếm số chuỗi khác nhau bằng `set()`.
::::

::::reflect{#nghi-lai}
Bốn bài vừa qua xây bốn CHIẾN LƯỢC chọn token, mỗi chiến lược sửa một vấn đề của chiến lược trước: `argmax` (bài `1`) không có đa dạng nào; lấy mẫu thô (bài `2`) có đa dạng nhưng có thể chọn phải token xác suất siêu thấp; nhiệt độ (bài `3`) co giãn TOÀN CỤC nhưng không loại hẳn phần đuôi; top-k (bài `4`) loại hẳn phần đuôi nhưng dùng một hằng số `k` không thích ứng với hình dạng phân phối; top-p (bài này) thích ứng ĐỘNG nhưng vẫn cần một ngưỡng `p_nguong` chọn trước. Bài cuối (BOSS) không thêm kỹ thuật mới — nó RÁP lại bốn chiến lược này, chạy trên CÙNG một mô hình thật, từ CÙNG một điểm khởi đầu, với các `seed` cố định liệt kê trước, rồi đếm bằng `set()` xem MỖI chiến lược thực sự tạo ra bao nhiêu chuỗi khác nhau — số liệu, không suy luận.
::::

::::checkpoint{mastery=0.85}
::::
