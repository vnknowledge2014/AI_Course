---
id: tri-tue-nhan-tao.chien-luoc-giai-ma-va-lay-mau.boss-so-sanh-bon-chien-luoc
title: "BOSS — So sánh bốn chiến lược, đo đa dạng bằng set()"
summary: "Chay CA BON chien luoc giai ma (greedy, lay mau tho, top-k=5, top-p=0,9) TREN CUNG mot micro-transformer that da huan luyen 15 buoc SGD, tu CUNG mot diem bat dau ('trans'), voi 5 seed CO DINH liet ke truoc [0,1,2,3,4]. Dem SO CHUOI KHAC NHAU moi chien luoc tao ra bang set() tren van ban da giai ma: greedy cho DUNG 1 chuoi duy nhat (0 dang dang, dung nhu bai 1 da chi ra); lay mau tho, top-k, top-p deu cho DUNG 5 chuoi khac nhau (da dang toi da tren 5 seed, khong seed nao trung nhau). Bon con so do THAT, khong suy doan, dong quest chien-luoc-giai-ma-va-lay-mau (q8.4b, 6/6 bai) tren CHINH bo may Transformer that cua T8.3 -- khong mo phong."
locale: vi
track: tri-tue-nhan-tao
module: chien-luoc-giai-ma-va-lay-mau
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.boss-so-sanh-bon-chien-luoc]
requires: [ai.top-p-nucleus-sampling]
concepts: [ai.boss-so-sanh-bon-chien-luoc]
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
Bốn bài, bốn chiến lược, cùng một bộ máy Transformer thật. Bài BOSS này ráp CẢ BỐN lại — greedy, lấy mẫu thô, top-k, top-p — chạy trên CÙNG một mô hình, từ CÙNG một điểm khởi đầu, với các `seed` cố định liệt kê trước, rồi đếm bằng `set()` xem MỖI chiến lược thực sự đa dạng tới đâu.
::::

::::explain{#rap_bon_chien_luoc}
Không có kỹ thuật MỚI nào trong bài này — đúng bốn hàm đã viết ở bốn bài trước, RÁP lại thành một phép so sánh có kiểm soát:

> **Greedy** (`sinh_van_ban`, bài `1`) — `argmax` mỗi bước, không đọc `seed` nào cả.
>
> **Lấy mẫu thô** (`sinh_van_ban_ngau_nhien`, dùng `lay_mau_tu_phan_phoi` của bài `2`) — lấy mẫu có trọng số trực tiếp trên softmax gốc, không cắt gì cả.
>
> **Top-k** (`sinh_van_ban_top_k`, dùng `top_k_phan_phoi` của bài `4`, `k=5`) — cắt còn `5` token trước khi lấy mẫu.
>
> **Top-p** (`sinh_van_ban_top_p`, dùng `top_p_phan_phoi` của bài `5`, `p_nguong=0,9`) — cắt theo ngưỡng tích luỹ trước khi lấy mẫu.

Phép đo: chạy MỖI chiến lược `5` lần, với `5` giá trị `seed` CỐ ĐỊNH và LIỆT KÊ TRƯỚC — `[0, 1, 2, 3, 4]` (ba chiến lược có lấy mẫu dùng `seed` này để tạo `Generator`; greedy bỏ qua nó vì `argmax` không đọc `seed`). Với mỗi lần chạy, giải mã chuỗi token ra văn bản, đưa vào một `set()`. Số phần tử còn lại trong `set()` sau `5` lần chạy chính là SỐ CHUỖI KHÁC NHAU mà chiến lược đó tạo ra — một con số đo được, không suy luận từ "trực giác về ngẫu nhiên".

Dự đoán có cấu trúc rõ ràng: greedy phải luôn cho ĐÚNG `1` (đã chứng minh ở bài `1` — không có ngẫu nhiên nào để tạo chuỗi thứ hai). Ba chiến lược còn lại đều CÓ đọc `seed` để lấy mẫu, nên với `5` seed khác nhau, chúng CÓ THỂ (không chắc chắn TUYỆT ĐỐI, nhưng đo được cụ thể) cho ra nhiều hơn `1` chuỗi.
::::

::::example{#boss_bon_chien_luoc_that}
Huấn luyện lại micro-transformer (giống hệt bốn bài trước), rồi chạy cả bốn chiến lược qua `5` seed cố định:

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


def sinh_van_ban_ngau_nhien(mo_hinh, ids_mo_dau, dim, so_token_moi, rng):
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


def top_k_phan_phoi(phan_phoi, k):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    idx_giu = idx_sap_xep[:k]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


def sinh_van_ban_top_k(mo_hinh, ids_mo_dau, dim, so_token_moi, k, rng):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        z = logits.data[-1]
        zs = z - np.max(z)
        e = np.exp(zs)
        p = e / np.sum(e)
        pk = top_k_phan_phoi(p, k)
        token_moi = lay_mau_tu_phan_phoi(pk, rng)
        ids.append(token_moi)
    return ids


def top_p_phan_phoi(phan_phoi, p_nguong):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    p_sap_xep = phan_phoi[idx_sap_xep]
    tich_luy = np.cumsum(p_sap_xep)
    so_luong = int(np.searchsorted(tich_luy, p_nguong)) + 1
    idx_giu = idx_sap_xep[:so_luong]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


def sinh_van_ban_top_p(mo_hinh, ids_mo_dau, dim, so_token_moi, p_nguong, rng):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        z = logits.data[-1]
        zs = z - np.max(z)
        e = np.exp(zs)
        p = e / np.sum(e)
        pp = top_p_phan_phoi(p, p_nguong)
        token_moi = lay_mau_tu_phan_phoi(pp, rng)
        ids.append(token_moi)
    return ids


def so_sanh_bon_chien_luoc(mo_hinh, ids_mo_dau, dim, so_token_moi, seeds, k, p_nguong):
    tap_greedy = set()
    tap_ngau_nhien = set()
    tap_top_k = set()
    tap_top_p = set()
    for seed in seeds:
        tap_greedy.add(giai_ma(sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi), id_sang_token))
        rng1 = np.random.default_rng(seed)
        tap_ngau_nhien.add(giai_ma(sinh_van_ban_ngau_nhien(mo_hinh, ids_mo_dau, dim, so_token_moi, rng1), id_sang_token))
        rng2 = np.random.default_rng(seed)
        tap_top_k.add(giai_ma(sinh_van_ban_top_k(mo_hinh, ids_mo_dau, dim, so_token_moi, k, rng2), id_sang_token))
        rng3 = np.random.default_rng(seed)
        tap_top_p.add(giai_ma(sinh_van_ban_top_p(mo_hinh, ids_mo_dau, dim, so_token_moi, p_nguong, rng3), id_sang_token))
    return {
        "greedy": len(tap_greedy),
        "ngau_nhien": len(tap_ngau_nhien),
        "top_k": len(tap_top_k),
        "top_p": len(tap_top_p),
    }


SEEDS = [0, 1, 2, 3, 4]
ket_qua = so_sanh_bon_chien_luoc(mo_hinh, ids_mo_dau, dim, 4, SEEDS, 5, 0.9)
print(ket_qua)
```

```text title=readonly
{'greedy': 1, 'ngau_nhien': 5, 'top_k': 5, 'top_p': 5}
```

Bốn chiến lược, cùng `5` seed `[0, 1, 2, 3, 4]`, cùng mô hình, cùng điểm khởi đầu: `greedy` cho ĐÚNG `1` chuỗi (đúng như bài `1` đã chứng minh — không có ngẫu nhiên nào để tạo chuỗi thứ hai). Cả ba chiến lược CÓ lấy mẫu — `ngau_nhien`, `top_k`, `top_p` — đều cho ĐÚNG `5` chuỗi khác nhau: cả `5` seed đều tạo ra một chuỗi RIÊNG, không seed nào trùng nhau. Đa dạng đo được bằng số thật, không suy luận.
::::

::::predict{#doan_boss_seed_khong_anh_huong_greedy commitOnce}
`so_sanh_bon_chien_luoc` tạo một `Generator` mới từ `seed` cho MỖI chiến lược có lấy mẫu, nhưng gọi `sinh_van_ban` (greedy) mà KHÔNG truyền `seed` nào vào nó cả — `sinh_van_ban` không nhận tham số `rng`.

**Trước khi chạy thử**, bạn đoán: nếu đổi `SEEDS` thành một danh sách khác, ví dụ `[100, 200, 300, 400, 500]` thay vì `[0, 1, 2, 3, 4]`, kết quả `ket_qua["greedy"]` có đổi không?

:::opt{correct}
Không — `sinh_van_ban` không đọc `seed` hay bất kỳ `Generator` nào, nó chỉ gọi `argmax` trên logit thật; đổi danh sách `SEEDS` chỉ đổi những gì được truyền cho `np.random.default_rng(seed)` ở BA nhánh lấy mẫu, không ảnh hưởng gì tới nhánh `greedy` — `ket_qua["greedy"]` vẫn luôn là `1`, bất kể `SEEDS` chứa giá trị gì hay bao nhiêu phần tử (miễn còn ít nhất một phần tử)
:::

:::opt
Có — vì vòng lặp `for seed in seeds:` chạy `5` lần cho MỌI nhánh trong thân vòng lặp, kể cả nhánh `greedy`, nên đổi `seeds` cũng đổi số lần `sinh_van_ban` được gọi, và điều đó có thể tạo thêm chuỗi mới
::why
Gần đúng ở việc để ý ĐÚNG rằng `tap_greedy.add(...)` nằm TRONG cùng một vòng lặp `for seed in seeds:` như ba nhánh lấy mẫu — quan sát cấu trúc code đó chính xác.

Chỗ lệch: SỐ LẦN gọi `sinh_van_ban` phụ thuộc ĐỘ DÀI của `seeds` (bao nhiêu phần tử), không phụ thuộc GIÁ TRỊ cụ thể bên trong nó — và quan trọng hơn, `sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi)` không hề nhận biến `seed` làm tham số, nên gọi nó bao nhiêu lần đi nữa, nó vẫn luôn tính `argmax` trên CÙNG logit và trả về CÙNG một chuỗi mỗi lần — `set()` của các chuỗi giống hệt nhau vẫn chỉ có `1` phần tử.
::
:::

:::opt
Không xác định được — cần biết cụ thể các số trong danh sách `SEEDS` mới đó có gây xung đột gì với mô hình đã huấn luyện hay không
::why
Gần đúng ở tinh thần thận trọng "cần dữ liệu cụ thể mới kết luận được" — nguyên tắc hữu ích khi kết quả THẬT SỰ phụ thuộc giá trị đầu vào.

Chỗ lệch: ở đây không có "xung đột" nào có thể xảy ra, vì nhánh `greedy` không hề dùng tới giá trị của `seed` — nó hoàn toàn tách biệt khỏi luồng dữ liệu `seed → Generator → lay_mau_tu_phan_phoi`. Không cần biết `SEEDS` chứa số gì để khẳng định `ket_qua["greedy"] == 1` — điều đó đúng với MỌI danh sách `SEEDS` không rỗng, vì lý do cấu trúc (greedy không đọc `seed`), không phải vì các số cụ thể "may mắn" không xung đột.
::
:::
::::

::::code{#viet_so_sanh_bon_chien_luoc}
Hoàn thiện `so_sanh_bon_chien_luoc`: nhánh top-p phải tạo `Generator` từ ĐÚNG biến `seed` của vòng lặp hiện tại (giống hai nhánh trên), và kết quả trả về phải đếm ĐÚNG tập hợp `tap_top_p`.

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
    return int(rng.choice(n, p=phan_phoi))


def sinh_van_ban_ngau_nhien(mo_hinh, ids_mo_dau, dim, so_token_moi, rng):
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


def top_k_phan_phoi(phan_phoi, k):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    idx_giu = idx_sap_xep[:k]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


def sinh_van_ban_top_k(mo_hinh, ids_mo_dau, dim, so_token_moi, k, rng):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        z = logits.data[-1]
        zs = z - np.max(z)
        e = np.exp(zs)
        p = e / np.sum(e)
        pk = top_k_phan_phoi(p, k)
        token_moi = lay_mau_tu_phan_phoi(pk, rng)
        ids.append(token_moi)
    return ids


def top_p_phan_phoi(phan_phoi, p_nguong):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    p_sap_xep = phan_phoi[idx_sap_xep]
    tich_luy = np.cumsum(p_sap_xep)
    so_luong = int(np.searchsorted(tich_luy, p_nguong)) + 1
    idx_giu = idx_sap_xep[:so_luong]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


def sinh_van_ban_top_p(mo_hinh, ids_mo_dau, dim, so_token_moi, p_nguong, rng):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        z = logits.data[-1]
        zs = z - np.max(z)
        e = np.exp(zs)
        p = e / np.sum(e)
        pp = top_p_phan_phoi(p, p_nguong)
        token_moi = lay_mau_tu_phan_phoi(pp, rng)
        ids.append(token_moi)
    return ids


def so_sanh_bon_chien_luoc(mo_hinh, ids_mo_dau, dim, so_token_moi, seeds, k, p_nguong):
    tap_greedy = set()
    tap_ngau_nhien = set()
    tap_top_k = set()
    tap_top_p = set()
    for seed in seeds:
        tap_greedy.add(giai_ma(sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi), id_sang_token))
        rng1 = np.random.default_rng(seed)
        tap_ngau_nhien.add(giai_ma(sinh_van_ban_ngau_nhien(mo_hinh, ids_mo_dau, dim, so_token_moi, rng1), id_sang_token))
        rng2 = np.random.default_rng(seed)
        tap_top_k.add(giai_ma(sinh_van_ban_top_k(mo_hinh, ids_mo_dau, dim, so_token_moi, k, rng2), id_sang_token))
        rng3 = np.random.default_rng(___)                # seed
        tap_top_p.add(giai_ma(sinh_van_ban_top_p(mo_hinh, ids_mo_dau, dim, so_token_moi, p_nguong, rng3), id_sang_token))
    return {
        "greedy": len(tap_greedy),
        "ngau_nhien": len(tap_ngau_nhien),
        "top_k": len(tap_top_k),
        "top_p": len(___),                               # tap_top_p
    }


SEEDS = [0, 1, 2, 3, 4]
ket_qua = so_sanh_bon_chien_luoc(mo_hinh, ids_mo_dau, dim, 4, SEEDS, 5, 0.9)
print(ket_qua)
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


def sinh_van_ban_ngau_nhien(mo_hinh, ids_mo_dau, dim, so_token_moi, rng):
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


def top_k_phan_phoi(phan_phoi, k):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    idx_giu = idx_sap_xep[:k]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


def sinh_van_ban_top_k(mo_hinh, ids_mo_dau, dim, so_token_moi, k, rng):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        z = logits.data[-1]
        zs = z - np.max(z)
        e = np.exp(zs)
        p = e / np.sum(e)
        pk = top_k_phan_phoi(p, k)
        token_moi = lay_mau_tu_phan_phoi(pk, rng)
        ids.append(token_moi)
    return ids


def top_p_phan_phoi(phan_phoi, p_nguong):
    idx_sap_xep = np.argsort(phan_phoi)[::-1]
    p_sap_xep = phan_phoi[idx_sap_xep]
    tich_luy = np.cumsum(p_sap_xep)
    so_luong = int(np.searchsorted(tich_luy, p_nguong)) + 1
    idx_giu = idx_sap_xep[:so_luong]
    ra = np.zeros_like(phan_phoi)
    ra[idx_giu] = phan_phoi[idx_giu]
    return ra / ra.sum()


def sinh_van_ban_top_p(mo_hinh, ids_mo_dau, dim, so_token_moi, p_nguong, rng):
    ids = list(ids_mo_dau)
    for _ in range(so_token_moi):
        logits = forward(mo_hinh, ids, dim)
        z = logits.data[-1]
        zs = z - np.max(z)
        e = np.exp(zs)
        p = e / np.sum(e)
        pp = top_p_phan_phoi(p, p_nguong)
        token_moi = lay_mau_tu_phan_phoi(pp, rng)
        ids.append(token_moi)
    return ids


def so_sanh_bon_chien_luoc(mo_hinh, ids_mo_dau, dim, so_token_moi, seeds, k, p_nguong):
    tap_greedy = set()
    tap_ngau_nhien = set()
    tap_top_k = set()
    tap_top_p = set()
    for seed in seeds:
        tap_greedy.add(giai_ma(sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi), id_sang_token))
        rng1 = np.random.default_rng(seed)
        tap_ngau_nhien.add(giai_ma(sinh_van_ban_ngau_nhien(mo_hinh, ids_mo_dau, dim, so_token_moi, rng1), id_sang_token))
        rng2 = np.random.default_rng(seed)
        tap_top_k.add(giai_ma(sinh_van_ban_top_k(mo_hinh, ids_mo_dau, dim, so_token_moi, k, rng2), id_sang_token))
        rng3 = np.random.default_rng(seed)
        tap_top_p.add(giai_ma(sinh_van_ban_top_p(mo_hinh, ids_mo_dau, dim, so_token_moi, p_nguong, rng3), id_sang_token))
    return {
        "greedy": len(tap_greedy),
        "ngau_nhien": len(tap_ngau_nhien),
        "top_k": len(tap_top_k),
        "top_p": len(tap_top_p),
    }


SEEDS = [0, 1, 2, 3, 4]
ket_qua = so_sanh_bon_chien_luoc(mo_hinh, ids_mo_dau, dim, 4, SEEDS, 5, 0.9)
print(ket_qua)
```

```python title=test
assert ket_qua["greedy"] == 1, f"greedy phai cho DUNG 1 chuoi duy nhat -- dang ra {ket_qua['greedy']}"
assert ket_qua["ngau_nhien"] == 5, f"lay mau tho phai cho DUNG 5 chuoi khac nhau -- dang ra {ket_qua['ngau_nhien']}"
assert ket_qua["top_k"] == 5, f"top-k phai cho DUNG 5 chuoi khac nhau -- dang ra {ket_qua['top_k']}"
assert ket_qua["top_p"] == 5, f"top-p phai cho DUNG 5 chuoi khac nhau -- dang ra {ket_qua['top_p']}"

# bang chung trung tam: greedy phai la CHIEN LUOC DUY NHAT co da dang bang
# 1 -- ca ba chien luoc con lai deu phai co da dang LON HON greedy
assert ket_qua["greedy"] < ket_qua["ngau_nhien"], "greedy phai co it da dang HON lay mau tho"
assert ket_qua["greedy"] < ket_qua["top_k"], "greedy phai co it da dang HON top-k"
assert ket_qua["greedy"] < ket_qua["top_p"], "greedy phai co it da dang HON top-p"

# bien: danh sach seeds HOAN TOAN KHAC ([7], chi 1 phan tu, khong lien
# quan gi toi [0,1,2,3,4] cua lan chay chinh) -- greedy VAN phai la 1
# (khong doc seed nao ca, dung nhu du doan o phan predict), va vi seeds
# chi co 1 phan tu nen CA BON chien luoc deu phai cho DUNG 1 chuoi (khong
# the co nhieu chuoi hon so lan chay)
ket_qua_mot_seed = so_sanh_bon_chien_luoc(mo_hinh, ids_mo_dau, dim, 4, [7], 5, 0.9)
assert ket_qua_mot_seed == {"greedy": 1, "ngau_nhien": 1, "top_k": 1, "top_p": 1}, f"chi 1 seed (khac hoan toan SEEDS chinh) thi moi chien luoc phai cho DUNG 1 chuoi -- dang ra {ket_qua_mot_seed}"
```

:::hints
- kind: attention
  body: Hai chỗ trống trong `so_sanh_bon_chien_luoc`. Chỗ đầu — `rng1 = np.random.default_rng(___)` — nhìn hai dòng NGAY TRÊN nó (`rng1 = ...`, `rng2 = ...`) để thấy đúng biến nào được truyền vào `np.random.default_rng(...)` ở CẢ HAI dòng đó. Chỗ hai — `"top_p": len(___)` — nhìn ba dòng NGAY TRÊN trong cùng `return` (`"greedy": len(tap_greedy)`, `"ngau_nhien": len(tap_ngau_nhien)`, `"top_k": len(tap_top_k)`) để thấy đúng KHUÔN MẪU: mỗi khoá đếm ĐÚNG tập hợp cùng tên của nó.
- kind: strategy
  body: 'Chỗ đầu: `seed` — hoàn thiện thành `rng3 = np.random.default_rng(seed)`, giống hệt `rng1`/`rng2` phía trên. Chỗ hai: `tap_top_p` — hoàn thiện thành `"top_p": len(tap_top_p)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `seed` và `tap_top_p`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: 'rng3 phai duoc tao tu DUNG bien seed cua vong lap (khong duoc hardcode mot so co dinh hay dung sai bien); khoa "top_p" trong ket qua tra ve phai dem DUNG tap_top_p (khong duoc dem nham mot tap hop khac)'
  requireAst:
  - kind: uses-name, target: seed, min: 4
  - kind: uses-name, target: tap_top_p, min: 2
  # Da thu THAT bang kiemAst (goi truc tiep tren code trich tu solution da
  # bien dich, khong doan tay).
  # seed=4: TEN NAY TRUNG voi tham so cua khoi_tao (rng = np.random.
  # default_rng(seed) trong khoi_tao) -- 1 lan CO SAN o do, CONG 3 lan trong
  # than vong lap "for seed in seeds:" cua so_sanh_bon_chien_luoc (rng1=...,
  # rng2=..., va blank1 rng3=...(seed)). Dien bua blank1 thanh mot so co
  # dinh (vd "np.random.default_rng(0)", lam MOI lan lap deu dung CUNG mot
  # seed=0 cho nhanh top-p, bo qua bien seed dang chay) lam so nay tut
  # xuong 3 -- duoi nguong min=4, bi chan boi static; dong thoi bi chan boi
  # tests vi ket_qua["top_p"] se tut xuong 1 (moi lan lap deu dung seed=0
  # giong het nhau, cho CUNG mot chuoi) thay vi 5.
  # tap_top_p=2: 1 lan CO SAN (tap_top_p.add(...) trong than vong lap), 1
  # lan trong blank2 (len(tap_top_p)). Dien bua blank2 thanh "len(tap_top_k)"
  # (dem nham tap hop, loi copy-paste de xay ra khi bon nhanh giong cau
  # truc nhau) lam so nay tut xuong 1 -- duoi nguong min=2, bi chan boi
  # static; dong thoi bi chan boi tests vi ket_qua["top_p"] se BANG
  # ket_qua["top_k"] thay vi duoc dem doc lap.
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^\\{'greedy': 1, 'ngau_nhien': 5, 'top_k': 5, 'top_p': 5\\}\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`greedy=1`, `ngau_nhien=5`, `top_k=5`, `top_p=5` — bốn con số đo bằng `set()` thật, trên CHÍNH bộ máy Transformer đã tự xây ở T8.3, không mô phỏng gì cả. Quest `chien-luoc-giai-ma-va-lay-mau` đóng tại `6/6` bài.
::::

::::reflect{#nghi-lai}
Quest này khép lại bằng đúng CHỦ ĐỀ đã mở ra ở bài `1`: một chuỗi số logit thật không tự nó quyết định "chuỗi văn bản nào sẽ được sinh ra" — QUYẾT ĐỊNH đó nằm ở CHIẾN LƯỢC chọn token từ phân phối, và bốn chiến lược khác nhau cho bốn hành vi khác nhau, đo được bằng chính một con số: số chuỗi khác nhau qua `5` lần chạy.

> **Greedy** — `0` tự do, `1` chuỗi duy nhất, tất định tuyệt đối. Phù hợp khi cần một câu trả lời "tốt nhất theo mô hình", không cần đa dạng.
>
> **Lấy mẫu thô** — tự do tối đa trên toàn bộ phân phối, có thể chọn phải token xác suất cực thấp.
>
> **Nhiệt độ** — không đổi tập hợp token có thể chọn, chỉ đổi ĐỘ TẬP TRUNG (đo bằng entropy).
>
> **Top-k / top-p** — thu hẹp tập hợp CÓ THỂ chọn trước khi lấy mẫu, loại hẳn phần đuôi xác suất thấp; top-k dùng một số lượng cố định, top-p dùng một ngưỡng xác suất khiến số lượng đó tự thích ứng.

Toàn bộ quest này KHÔNG mô phỏng bất cứ điều gì — mọi phân phối xác suất, mọi logit, mọi chuỗi được sinh ra đều là output THẬT của chính lớp `Tensor` và `khoi_transformer` đã tự viết ở T8.3, chạy forward pass thật trên một mô hình đã huấn luyện thật. Quest tiếp theo của T8.4 quay lại dùng LLM mô phỏng (như q8.4a) cho một chủ đề khác hẳn: đầu ra có CẤU TRÚC (JSON máy phân tích được) và gọi công cụ.
::::

::::checkpoint{mastery=1.0}
::::
