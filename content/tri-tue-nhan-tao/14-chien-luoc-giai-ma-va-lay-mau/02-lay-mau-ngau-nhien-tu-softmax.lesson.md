---
id: tri-tue-nhan-tao.chien-luoc-giai-ma-va-lay-mau.lay-mau-ngau-nhien-tu-softmax
title: "Lấy mẫu ngẫu nhiên từ softmax — ngẫu nhiên nhưng tất định"
summary: "Thay argmax bang lay mau co trong so tren CHINH phan phoi softmax THAT cua micro-transformer (khong mo phong). Dung numpy.random.Generator.choice(n, p=phan_phoi) voi seed CO DINH truyen tuong minh (vd np.random.default_rng(7)) -- khong dung random module chuan, khong dung global seed an. Do bang so: CUNG seed=7 goi sinh_van_ban_lay_mau HAI LAN doc lap tu CUNG diem bat dau -> CUNG DUNG mot chuoi 'transirng ems r' ca hai lan (tat dinh du la 'ngau nhien'). SEED KHAC (seed=8) -> chuoi khac 'transfyirqgnk'. Minh hoa dung y tuong cot loi: lay mau ma van cham diem duoc, vi tinh ngau nhien nam trong Generator co seed, khong phai global state an."
locale: vi
track: tri-tue-nhan-tao
module: chien-luoc-giai-ma-va-lay-mau
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.lay-mau-ngau-nhien-tu-softmax]
requires: [ai.on-lai-greedy-va-gioi-han]
concepts: [ai.lay-mau-ngau-nhien-tu-softmax]
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
Bài trước đo giới hạn của `argmax`: `0` đa dạng, luôn cùng một chuỗi. Bài này thay bước chọn token cuối cùng bằng một phép LẤY MẪU ngẫu nhiên — nhưng "ngẫu nhiên" ở đây vẫn phải CHẤM ĐIỂM được, nên nó phải tất định theo một hạt giống (seed) cố định.
::::

::::explain{#lay_mau_co_trong_so}
`argmax` chọn token có xác suất CAO NHẤT — bỏ qua hoàn toàn mọi token khác, dù xác suất của chúng gần bằng số một. **Lấy mẫu có trọng số** (weighted sampling) làm khác: mỗi token có CƠ HỘI được chọn TỈ LỆ THUẬN với xác suất của nó — token có xác suất `0,36` có cơ hội được chọn cao hơn nhiều token có xác suất `0,01`, nhưng token thứ hai vẫn có MỘT cơ hội, không phải `0`.

`numpy` cung cấp đúng công cụ này qua lớp `numpy.random.Generator`: gọi `rng.choice(n, p=phan_phoi)` trả về một chỉ số trong khoảng `[0, n)`, được chọn ngẫu nhiên với xác suất mỗi chỉ số bằng đúng `phan_phoi[chi_so]`. Điểm bắt buộc: `rng` phải được tạo bằng `np.random.default_rng(seed)` với một `seed` CỤ THỂ, TRUYỀN TƯỜNG MINH — không dùng module `random` chuẩn của Python (không seed hoá tường minh theo cùng cách), và không dựa vào một trạng thái ngẫu nhiên TOÀN CỤC ẩn (như `np.random.seed(...)` cũ, ảnh hưởng MỌI lời gọi ngẫu nhiên trong cả chương trình, kể cả những chỗ không liên quan). Một `Generator` được tạo với `seed` cố định là một đối tượng ĐỘC LẬP: gọi `rng.choice(...)` nhiều lần trên CÙNG một `rng` cho ra một CHUỖI kết quả xác định trước — và tạo lại một `rng` MỚI với CÙNG `seed` đó, từ đầu, sẽ phát lại ĐÚNG chuỗi kết quả ấy.

Đây là điều làm "ngẫu nhiên" trở thành CHẤM ĐIỂM ĐƯỢC: `seed` cố định, `Generator` độc lập với global state, thuật toán lấy mẫu tất định (cùng `seed` + cùng chuỗi lời gọi ⇒ cùng kết quả, mọi lần, mọi máy). Bài này ráp một hàm sinh văn bản mới, `sinh_van_ban_lay_mau`, giữ nguyên vòng lặp autoregressive của `sinh_van_ban` (forward, lấy logit vị trí cuối, nối token mới, lặp lại) — chỉ đổi bước chọn token từ `argmax` sang lấy mẫu có trọng số trên phân phối softmax THẬT của `logits.data[-1]`.
::::

::::example{#lay_mau_tat_dinh}
Huấn luyện lại micro-transformer (giống hệt bài trước), rồi sinh văn bản bằng lấy mẫu — hai lần với CÙNG seed, một lần với seed KHÁC:

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


def lay_mau_tu_phan_phoi(phan_phoi, rng):
    n = len(phan_phoi)
    return int(rng.choice(n, p=phan_phoi))


def sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, so_token_moi, rng):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        z = logits.data[-1]
        zs = z - np.max(z)
        e = np.exp(zs)
        p = e / np.sum(e)
        token_moi = lay_mau_tu_phan_phoi(p, rng)
        ids.append(token_moi)
    return ids


rng_a1 = np.random.default_rng(7)
chuoi_a1 = giai_ma(sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, 8, rng_a1), id_sang_token)

rng_a2 = np.random.default_rng(7)
chuoi_a2 = giai_ma(sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, 8, rng_a2), id_sang_token)

rng_b = np.random.default_rng(8)
chuoi_b = giai_ma(sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, 8, rng_b), id_sang_token)

print(chuoi_a1)
print(chuoi_a2)
print(chuoi_a1 == chuoi_a2)
print(chuoi_b)
print(chuoi_a1 == chuoi_b)
```

```text title=readonly
transirng ems r
transirng ems r
True
transfyirqgnk
False
```

Hai `Generator` khác nhau (`rng_a1`, `rng_a2`), cùng tạo bằng `np.random.default_rng(7)` — CÙNG `seed`, không liên quan gì tới nhau về mặt đối tượng Python — nhưng `sinh_van_ban_lay_mau` chạy trên chúng cho ra ĐÚNG cùng một chuỗi `"transirng ems r"`. Đổi `seed` thành `8` (`rng_b`): chuỗi khác hẳn, `"transfyirqgnk"`. Ngẫu nhiên, nhưng tất định theo `seed` — đúng tính chất cần để một bài học có phép "ngẫu nhiên" mà vẫn chấm điểm được.
::::

::::predict{#doan_seed_giong_nhau commitOnce}
`lay_mau_tu_phan_phoi` gọi `rng.choice(n, p=phan_phoi)` — `rng` là một `Generator` được tạo từ `np.random.default_rng(seed)`.

**Trước khi chạy thử**, bạn đoán: nếu tạo HAI `Generator` độc lập bằng CÙNG một `seed` (ví dụ cả hai đều `np.random.default_rng(7)`), rồi gọi `sinh_van_ban_lay_mau` với CÙNG điểm khởi đầu trên MỖI `Generator` đó — hai chuỗi kết quả có giống nhau không?

:::opt{correct}
Có — hai `Generator` được khởi tạo từ CÙNG `seed` phát ra CÙNG một chuỗi số ngẫu nhiên nội bộ theo đúng cùng thứ tự, nên mọi lời gọi `rng.choice(...)` ở CÙNG bước sẽ trả về CÙNG chỉ số, và hai chuỗi văn bản sinh ra giống hệt nhau
:::

:::opt
Không — mỗi lần gọi `np.random.default_rng(...)` tạo ra một luồng ngẫu nhiên MỚI dựa trên đồng hồ hệ thống tại thời điểm gọi, nên dù cùng `seed` cũng không thể trùng nhau
::why
Gần đúng ở việc để ý rằng có những cách sinh số ngẫu nhiên (như `random.random()` không seed, hay `uuid4()`) THẬT SỰ dựa vào nguồn entropy của hệ thống (đồng hồ, tiếng ồn phần cứng) — quan sát đó đúng cho NHỮNG nguồn ngẫu nhiên đó.

Chỗ lệch: `np.random.default_rng(seed)` với một `seed` SỐ CỤ THỂ không đọc đồng hồ hệ thống để khởi tạo — nó dùng CHÍNH con số `seed` làm điểm bắt đầu tất định cho thuật toán sinh số giả-ngẫu-nhiên (PCG64) bên trong. Hai `Generator` cùng `seed` có cùng điểm bắt đầu, nên phát ra cùng một chuỗi số, mãi mãi, không phụ thuộc đồng hồ hay thời điểm gọi.
::
:::

:::opt
Chỉ giống nhau ở TOKEN ĐẦU TIÊN được lấy mẫu — càng về sau chuỗi càng lệch nhau vì sai số cộng dồn qua nhiều bước forward pass
::why
Gần đúng ở việc lưu ý rằng sinh văn bản là một quá trình LẶP NHIỀU BƯỚC, và trực giác "sai số cộng dồn qua nhiều bước" đúng trong một số ngữ cảnh số học (ví dụ tích phân số học lặp).

Chỗ lệch: ở đây không có "sai số" nào cộng dồn giữa hai lần chạy — `forward` là một hàm xác định (cùng `mo_hinh`, cùng `ids` ⇒ cùng logit, luôn luôn), và `rng.choice(...)` trên hai `Generator` cùng `seed` luôn trả về CÙNG chỉ số ở CÙNG bước gọi thứ mấy. Vì bước `t+1` phụ thuộc token được chọn ở bước `t` (chuỗi `ids` được nối thêm), và hai chuỗi giống nhau ở bước `t` thì đầu vào bước `t+1` cũng giống nhau — tính giống nhau LAN TRUYỀN qua mọi bước, không suy giảm.
::
:::
::::

::::code{#viet_lay_mau_tu_phan_phoi}
Hoàn thiện `lay_mau_tu_phan_phoi`: gọi `rng.choice` với đúng SỐ LƯỢNG lựa chọn (`n`, số phần tử của `phan_phoi`) và đúng TRỌNG SỐ (từ khoá `p=`, chính `phan_phoi`).

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


def lay_mau_tu_phan_phoi(phan_phoi, rng):
    n = len(phan_phoi)
    return int(rng.choice(___, p=___))          # n, phan_phoi


def sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, so_token_moi, rng):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        z = logits.data[-1]
        zs = z - np.max(z)
        e = np.exp(zs)
        p = e / np.sum(e)
        token_moi = lay_mau_tu_phan_phoi(p, rng)
        ids.append(token_moi)
    return ids


rng_a1 = np.random.default_rng(7)
chuoi_a1 = giai_ma(sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, 8, rng_a1), id_sang_token)

rng_a2 = np.random.default_rng(7)
chuoi_a2 = giai_ma(sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, 8, rng_a2), id_sang_token)

rng_b = np.random.default_rng(8)
chuoi_b = giai_ma(sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, 8, rng_b), id_sang_token)

print(chuoi_a1)
print(chuoi_a2)
print(chuoi_a1 == chuoi_a2)
print(chuoi_b)
print(chuoi_a1 == chuoi_b)
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


def lay_mau_tu_phan_phoi(phan_phoi, rng):
    n = len(phan_phoi)
    return int(rng.choice(n, p=phan_phoi))


def sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, so_token_moi, rng):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        z = logits.data[-1]
        zs = z - np.max(z)
        e = np.exp(zs)
        p = e / np.sum(e)
        token_moi = lay_mau_tu_phan_phoi(p, rng)
        ids.append(token_moi)
    return ids


rng_a1 = np.random.default_rng(7)
chuoi_a1 = giai_ma(sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, 8, rng_a1), id_sang_token)

rng_a2 = np.random.default_rng(7)
chuoi_a2 = giai_ma(sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, 8, rng_a2), id_sang_token)

rng_b = np.random.default_rng(8)
chuoi_b = giai_ma(sinh_van_ban_lay_mau(mo_hinh, ids_mo_dau, dim, 8, rng_b), id_sang_token)

print(chuoi_a1)
print(chuoi_a2)
print(chuoi_a1 == chuoi_a2)
print(chuoi_b)
print(chuoi_a1 == chuoi_b)
```

```python title=test
assert chuoi_a1 == "transirng ems r", f"chuoi voi seed=7 (lan 1) sai -- dang ra {chuoi_a1!r}"
assert chuoi_a2 == "transirng ems r", f"chuoi voi seed=7 (lan 2) sai -- dang ra {chuoi_a2!r}"
assert chuoi_a1 == chuoi_a2, "cung seed=7 tren hai Generator doc lap phai cho DUNG cung mot chuoi"
assert chuoi_b == "transfyirqgnk", f"chuoi voi seed=8 sai -- dang ra {chuoi_b!r}"
assert chuoi_a1 != chuoi_b, "seed khac nhau (7 va 8) phai cho hai chuoi KHAC nhau"

# bien: phan phoi mot-diem (delta) -- xac suat 1.0 o dung mot vi tri, moi
# seed deu phai cho DUNG vi tri do (khong con ngau nhien nao ca)
p_delta = np.array([0.0, 1.0, 0.0, 0.0])
for hat_giong in [0, 1, 2, 100]:
    rng_delta = np.random.default_rng(hat_giong)
    assert lay_mau_tu_phan_phoi(p_delta, rng_delta) == 1, f"phan phoi delta phai luon cho vi tri 1 -- seed {hat_giong} cho {lay_mau_tu_phan_phoi(p_delta, np.random.default_rng(hat_giong))}"

# bien: phan phoi delta o vi tri KHAC (vi tri 3) -- xac nhan ham doc dung
# THAM SO phan_phoi, khong hardcode vi tri
p_delta_2 = np.array([0.0, 0.0, 0.0, 1.0])
rng_delta_2 = np.random.default_rng(0)
assert lay_mau_tu_phan_phoi(p_delta_2, rng_delta_2) == 3, f"phan phoi delta o vi tri 3 phai cho DUNG 3 -- dang ra {lay_mau_tu_phan_phoi(p_delta_2, np.random.default_rng(0))}"
```

:::hints
- kind: attention
  body: Một chỗ trống với hai đối số, cả hai đều bên trong `rng.choice(___, p=___)`. Đối số đầu là SỐ LƯỢNG lựa chọn có thể — biến `n` vừa được tính ở dòng trên bằng `len(phan_phoi)`. Đối số từ khoá `p=` là TRỌNG SỐ của từng lựa chọn — chính tham số `phan_phoi` của hàm.
- kind: strategy
  body: 'Điền `n` vào đối số đầu và `phan_phoi` vào đối số `p=` — hoàn thiện thành `rng.choice(n, p=phan_phoi)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `n` và `phan_phoi`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: 'rng.choice(...) phai duoc goi voi so luong LA n (bien vua tinh bang len(phan_phoi), khong duoc hardcode mot hang so vd 28) VA trong so LA phan_phoi (tham so cua ham, khong duoc bo qua hay thay bang mot mang khac)'
  requireAst:
  - kind: uses-name, target: "n", min: 8
  - kind: uses-name, target: phan_phoi, min: 2
  # Da thu THAT bang kiemAst (goi truc tiep tren code trich tu solution da
  # bien dich, khong doan tay).
  # "n"=8: TEN NAY TRUNG voi bien cuc bo "n" ben trong
  # mat_mat_du_doan_tiep_theo (n = len(nhan), doc lai nhieu lan trong ham
  # do) -- dem AST khong phan biet pham vi (scope), nen tong la 8 lan doc
  # TRONG CA FILE: 7 lan CO SAN rai rac + 1 lan trong blank (rng.choice(n,
  # ...)). Dien bua blank thanh "rng.choice(28, p=phan_phoi)" (hardcode
  # vocab_size, tinh co dung tren du lieu vi du nay nhung sai khi phan_phoi
  # co do dai khac -- xem bien "phan phoi delta 4 phan tu" trong test) lam
  # so nay tut xuong 7 -- duoi nguong min=8, bi chan boi static; dong thoi
  # bi chan boi tests vi cac truong hop p_delta (4 phan tu, khong phai 28).
  # phan_phoi=2: 1 lan CO SAN (n = len(phan_phoi)), 1 lan trong blank
  # (p=phan_phoi). Dien bua blank thanh mot mang khac hay bo tu khoa p= lam
  # so nay tut xuong 1 -- duoi nguong min=2, bi chan boi static; dong thoi
  # bi chan boi run (thieu tu khoa p= bat buoc cua rng.choice se nem loi
  # hoac doi phan phoi deu, sai voi moi assert so sanh chuoi chinh xac).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^transirng ems r\\ntransirng ems r\\nTrue\\ntransfyirqgnk\\nFalse\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`seed=7` hai lần độc lập: cùng một chuỗi `"transirng ems r"`. `seed=8`: chuỗi khác hẳn. Ngẫu nhiên có kiểm soát, chấm điểm được. Bài sau thêm một tham số điều khiển ĐỘ "nhọn" của phân phối trước khi lấy mẫu: nhiệt độ — chia logit cho một hằng số trước `softmax`, đo bằng entropy thật.
::::

::::reflect{#nghi-lai}
`argmax` và lấy mẫu có trọng số là hai đầu của MỘT trục: `argmax` luôn chọn token có xác suất cao nhất (đa dạng `0`), lấy mẫu cho MỌI token có xác suất khác `0` một cơ hội thực sự (đa dạng `>0`, tuỳ seed). Nhưng lấy mẫu THÔ trên softmax gốc (chưa qua bất kỳ điều chỉnh nào) vẫn có thể chọn phải một token có xác suất rất thấp — không phải lúc nào cũng mong muốn. Bài sau thêm một "núm vặn" giữa hai thái cực đó: NHIỆT ĐỘ — chia logit cho một hằng số `T` trước khi đưa vào `softmax`. `T` nhỏ kéo phân phối về gần giống `argmax` hơn (ít đa dạng hơn); `T` lớn kéo phân phối về gần đều hơn (đa dạng hơn). Đo bằng entropy thật — không suy luận.
::::

::::checkpoint{mastery=0.8}
::::
