---
id: tri-tue-nhan-tao.chien-luoc-giai-ma-va-lay-mau.on-lai-greedy-va-gioi-han
title: "Ôn lại greedy decoding — và giới hạn: 0 đa dạng"
summary: "T8.4 vua dung mot LLM MO PHONG (q8.4a). Quest nay RE HUONG NGUOC LAI: dung THAT bo may Transformer da xay o T8.3 -- khong mo phong. Huan luyen lai DUNG mot micro-transformer (dim=4, hidden=6, corpus 80 ky tu, seed=9, 15 buoc SGD, giong het q8.3e) roi goi sinh_van_ban (greedy, da hoc o q8.3d) NHIEU LAN tu CUNG mot diem bat dau. Ca 5 lan deu ra dung 'transformrs' -- do bang tap hop (set) xac nhan CHINH XAC 1 chuoi duy nhat. Ham moi day_chuoi_khac_nhau tong quat hoa phep do do (5 lan -> 1, 0 lan -> 0, 1 lan -> 1) -- dat nen cho toan bo quest: greedy KHONG co ngau nhien nao de tao da dang, moi chien luoc sampling sau day deu can mot nguon ngau nhien THAT co kiem soat duoc."
locale: vi
track: tri-tue-nhan-tao
module: chien-luoc-giai-ma-va-lay-mau
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.on-lai-greedy-va-gioi-han]
requires: [ai.boss-prompt-hoan-chinh]
concepts: [ai.on-lai-greedy-va-gioi-han]
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
Quest trước dùng một LLM mô phỏng — tra bảng, không mạng nơ-ron nào cả. Quest này rẽ NGƯỢC LẠI: dùng THẬT bộ máy Transformer đã tự xây ở T8.3 — cùng lớp `Tensor`, cùng `khoi_transformer`, một mô hình huấn luyện thật, cho ra một phân phối xác suất CÓ THẬT. Bài đầu tiên: ôn lại `sinh_van_ban` (greedy decoding) — và đo bằng số một giới hạn của nó.
::::

::::explain{#chuyen_huong_va_greedy}
Suốt T8.4 tới giờ, mọi bài đều dùng một **LLM mô phỏng** — một hàm Python thuần, tra bảng hoặc áp luật cố định, vì mục tiêu là học CƠ CHẾ prompt (vai trò message, few-shot, CoT). Quest này ("chiến lược giải mã và lấy mẫu") học một chủ đề khác hẳn: LÀM SAO một mô hình đi TỪ một phân phối xác suất TỚI một chuỗi token cụ thể. Câu hỏi đó vô nghĩa nếu phân phối xác suất là giả — sampling cần một phân phối THẬT để áp thuật toán lên. Vì vậy từ đây, mọi bài dùng lại NGUYÊN VẸN bộ máy đã xây ở T8.3: lớp `Tensor` (`+`, `*`, `matmul`, `softmax`, `layernorm`, `backward`), `khoi_transformer`/`xep_chong_2_tang`, và pipeline BPE — không mô phỏng bất cứ điều gì.

Để có một phân phối xác suất thật, quest này huấn luyện lại — trong MỖI bài — đúng một micro-transformer nhỏ: corpus `80` ký tự (giống hệt `boss-transformer-tu-so-0`, q8.3e), BPE ra `28` token, `dim=4`, `hidden=6`, khởi tạo với `seed=9`, rồi chạy `15` bước SGD (`lr=0,5`). Đây KHÔNG phải một mô hình giỏi — mục tiêu chỉ là có MỘT phân phối xác suất không đồng đều, không ngẫu nhiên hoàn toàn, để các thuật toán lấy mẫu của quest này có cái THẬT để áp dụng lên.

Đã học ở `sinh-van-ban` (q8.3d): `sinh_van_ban` sinh văn bản bằng **greedy decoding** — ở MỖI bước, forward chuỗi hiện tại, lấy logit ở vị trí CUỐI, rồi chọn token có xác suất CAO NHẤT (`argmax`), nối vào chuỗi, lặp lại. Không có phép ngẫu nhiên nào trong toàn bộ quy trình đó — `argmax` trên cùng một mảng số luôn cho cùng một chỉ số. Bài này đo chính xác hệ quả của điều đó: chạy `sinh_van_ban` nhiều lần từ CÙNG một điểm khởi đầu, trên CÙNG một mô hình đã huấn luyện — kết quả có đa dạng không?
::::

::::example{#greedy_khong_da_dang}
Huấn luyện lại micro-transformer, rồi gọi `sinh_van_ban` ba lần từ cùng một điểm khởi đầu:

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

chuoi_1 = giai_ma(sinh_van_ban(mo_hinh, ids_mo_dau, dim, 6), id_sang_token)
chuoi_2 = giai_ma(sinh_van_ban(mo_hinh, ids_mo_dau, dim, 6), id_sang_token)
chuoi_3 = giai_ma(sinh_van_ban(mo_hinh, ids_mo_dau, dim, 6), id_sang_token)

print(chuoi_1)
print(chuoi_2)
print(chuoi_3)
print(chuoi_1 == chuoi_2 == chuoi_3)
print(len({chuoi_1, chuoi_2, chuoi_3}))
```

```text title=readonly
transformrs
transformrs
transformrs
True
1
```

Ba lần gọi `sinh_van_ban` — CÙNG mô hình vừa huấn luyện `15` bước, CÙNG điểm khởi đầu `ids_mo_dau` (giải mã là `"trans"`) — cho ra ĐÚNG cùng một chuỗi `"transformrs"` cả ba lần. `len({chuoi_1, chuoi_2, chuoi_3})` — đưa cả ba chuỗi vào một tập hợp (`set`, chỉ giữ giá trị KHÁC nhau) — bằng `1`: không có phép ngẫu nhiên nào trong `argmax`, nên không cách nào có chuỗi thứ hai.
::::

::::predict{#doan_da_dang_greedy commitOnce}
`sinh_van_ban` gọi `forward` rồi lấy `np.argmax(logits.data[-1])` ở mỗi bước — luôn chọn chỉ số có giá trị LỚN NHẤT trong một mảng số cố định.

**Trước khi chạy thử**, bạn đoán: nếu gọi `dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, so_lan=5)` — chạy `sinh_van_ban` `5` lần từ CÙNG một điểm khởi đầu, trên CÙNG một mô hình đã huấn luyện, rồi đếm số chuỗi KHÁC NHAU bằng `set()` — kết quả là bao nhiêu?

:::opt{correct}
`1` — cả `5` lần chạy cho ra CÙNG một chuỗi, vì `argmax` trên cùng một mảng logit luôn trả về cùng một chỉ số; không có nguồn ngẫu nhiên nào trong `forward` hay trong `sinh_van_ban` để tạo ra một kết quả khác ở lần chạy thứ hai trở đi
:::

:::opt
`5` — mỗi lần forward pass chạy lại từ đầu nên có thể tích luỹ sai số nổi (floating point) khác nhau, dẫn tới `argmax` chọn token khác nhau ở một vài lần chạy
::why
Gần đúng ở việc để ý rằng số học dấu phẩy động CÓ thể tích luỹ sai số nhỏ qua nhiều phép tính — quan sát đó có thật trong nhiều ngữ cảnh tính toán số.

Chỗ lệch: `forward(mo_hinh, ids, dim)` là một hàm THUẦN theo nghĩa nó chỉ đọc `mo_hinh` (không đổi giữa các lần gọi) và `ids` (giống hệt nhau ở bước đầu của mỗi lần chạy) — cùng một chuỗi phép toán `numpy` xác định (`matmul`, `softmax`, `layernorm`, ...) trên CÙNG một dữ liệu đầu vào luôn cho CÙNG một mảng số ra, không "trôi" giữa các lần gọi khác nhau trong cùng một tiến trình Python.
::
:::

:::opt
Không xác định được — phụ thuộc thứ tự các phép `numpy` được tối ưu hoá lúc chạy
::why
Gần đúng ở việc `numpy` CÓ tối ưu hoá cách thực thi phép toán ở tầng thấp (vectorization, đa luồng cho ma trận lớn) — quan tâm tới hiệu năng runtime là một trực giác hợp lý.

Chỗ lệch: tối ưu hoá runtime của `numpy` không đổi KẾT QUẢ TOÁN HỌC của một phép tính xác định — nó chỉ đổi tốc độ. `mo_hinh` không được cập nhật gì thêm giữa các lần gọi `sinh_van_ban` (không có `mot_buoc` nào chạy xen vào), nên đầu vào của `forward` giống hệt nhau ở mọi lần gọi, và đầu ra cũng phải giống hệt.
::
:::
::::

::::code{#viet_dem_chuoi_khac_nhau}
Hoàn thiện `dem_chuoi_khac_nhau`: mỗi lần lặp phải đưa chuỗi `ids` vừa sinh vào một tập hợp dưới dạng một giá trị BẤT BIẾN (`set` không nhận `list` trực tiếp), rồi trả về SỐ LƯỢNG giá trị khác nhau còn lại trong tập hợp đó.

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


def dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, so_token_moi, so_lan):
    ket_qua = set()
    for _ in range(so_lan):
        ids = sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi)
        ket_qua.add(___)                        # tuple(ids)
    return ___                                  # len(ket_qua)


print(dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 5))
print(dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 0))
print(dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 1))
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


def dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, so_token_moi, so_lan):
    ket_qua = set()
    for _ in range(so_lan):
        ids = sinh_van_ban(mo_hinh, ids_mo_dau, dim, so_token_moi)
        ket_qua.add(tuple(ids))
    return len(ket_qua)


print(dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 5))
print(dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 0))
print(dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 1))
```

```python title=test
assert dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 5) == 1, f"5 lan greedy tu cung diem bat dau phai cho DUNG 1 chuoi -- dang ra {dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 5)}"

# bien: 0 lan lap -- vong lap khong chay lan nao, tap rong, phai tra ve 0
assert dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 0) == 0, f"0 lan lap phai tra ve 0 -- dang ra {dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 0)}"

# bien: dung 1 lan lap -- tap co dung 1 phan tu
assert dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 1) == 1, f"1 lan lap phai tra ve 1 -- dang ra {dem_chuoi_khac_nhau(mo_hinh, ids_mo_dau, dim, 6, 1)}"

# bang chung trung tam: chuoi sinh ra la DUNG 'transformrs', khong phai
# mot chuoi ngau nhien nao khac -- xac nhan mo hinh huan luyen dung
assert giai_ma(sinh_van_ban(mo_hinh, ids_mo_dau, dim, 6), id_sang_token) == "transformrs", f"chuoi greedy sai -- dang ra {giai_ma(sinh_van_ban(mo_hinh, ids_mo_dau, dim, 6), id_sang_token)!r}"
```

:::hints
- kind: attention
  body: Hai chỗ trống trong `dem_chuoi_khac_nhau`. Chỗ đầu nằm trong `ket_qua.add(___)` — `ket_qua` là một `set`, và `set` KHÔNG nhận `list` (không băm được / unhashable) — phải bọc `ids` thành một kiểu bất biến trước. Chỗ hai là `return ___` — trả về SỐ LƯỢNG phần tử còn lại trong `ket_qua` sau vòng lặp.
- kind: strategy
  body: 'Chỗ đầu: `tuple(ids)` — biến `list` thành `tuple` (bất biến, băm được, `set` chấp nhận). Chỗ hai: `len(ket_qua)` — đếm số phần tử của tập hợp.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `tuple(ids)` và `len(ket_qua)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: 'ket_qua.add(...) phai boc ids thanh tuple(ids) (khong duoc them thang list truc tiep hay chuoi hoa kieu khac); return phai la len(ket_qua) THAT (khong duoc chep san mot hang so co dinh)'
  requireAst:
  - kind: uses-call, target: tuple, min: 1
  - kind: uses-call, target: len, min: 8
  # Da thu THAT bang kiemAst (goi truc tiep tren code trich tu solution da
  # bien dich, khong doan tay).
  # tuple=1: CHI mot lan xuat hien trong toan bo file, dung trong blank1
  # (ket_qua.add(tuple(ids))) -- khong co "tuple(" nao khac trong Tensor hay
  # cac ham BPE/transformer. Dien bua blank1 thanh "ket_qua.add(str(ids))"
  # hay "ket_qua.add(ids)" (loi runtime vi list khong bam duoc, bi chan boi
  # tier run) deu lam so nay tut xuong 0 -- duoi nguong min=1, bi chan boi
  # static (voi "str(ids)") hoac boi run (voi "ids" tran).
  # len=8: TONG THAT gom 7 lan CO SAN rai rac trong boilerplate (vd trong
  # dem_cap_lien_ke, gop_cap, huan_luyen_bpe, mat_mat_du_doan_tiep_theo,
  # forward) CONG 1 lan trong blank2 (return len(ket_qua)). Dien bua blank2
  # thanh "return 1" (chep san dung tinh co dung voi truong hop so_lan=5
  # trong test dau) lam so nay tut xuong 7 -- duoi nguong min=8, bi chan boi
  # static; dong thoi bi chan boi tests vi truong hop so_lan=0 doi hoi ket
  # qua 0, khong phai 1 (GOTCHA "boilerplate-threshold-masking": neu dat
  # min=1 ngay tho thi mutant "return 1" van qua duoc tang static, chi con
  # tests moi bat duoc no).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^1\\n0\\n1\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`5` lần chạy, `0` lần chạy, `1` lần chạy — greedy luôn cho ĐÚNG số chuỗi bằng `1` (trừ khi không chạy lần nào). `argmax` không có chỗ cho ngẫu nhiên. Bài sau thay `argmax` bằng một phép LẤY MẪU ngẫu nhiên có trọng số — trên CHÍNH phân phối softmax thật vừa dùng ở đây — và bắt đầu đo đa dạng tăng lên.
::::

::::reflect{#nghi-lai}
Greedy decoding không "sai" — nó vẫn là chiến lược giải mã hợp lý cho nhiều tác vụ (dịch máy, tóm tắt, nơi CHỈ MUỘN một câu trả lời "tốt nhất theo mô hình"). Nhưng khi cần NHIỀU phương án khác nhau từ CÙNG một prompt (viết sáng tạo, brainstorm, tạo dữ liệu tổng hợp đa dạng), greedy hoàn toàn bất lực — nó cấu trúc TOÁN HỌC không cho phép hai lần chạy khác nhau. Bài sau giữ nguyên bộ máy Transformer thật này, nhưng thay bước "chọn token" cuối cùng: thay vì luôn lấy xác suất cao nhất, LẤY MẪU ngẫu nhiên có trọng số theo đúng phân phối softmax — với một `numpy.random.Generator` seed cố định, để phép "ngẫu nhiên" đó vẫn tất định và chấm được.
::::

::::checkpoint{mastery=0.8}
::::
