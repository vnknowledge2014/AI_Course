---
id: tri-tue-nhan-tao.boss-transformer-tu-so-0.rap-pipeline-tren-corpus-lon-hon
title: "Ráp pipeline đầy đủ trên corpus lớn hơn: BPE 50 token, gradient xuyên suốt"
summary: "Rap TOAN BO BPE (huan luyen vocab 50, 24 merge, tren corpus 560 ky tu THAT ve tri tue nhan tao) + embedding + 2 khoi Transformer that (residual+layernorm+attention) + chieu logit, tren cua so 16 token dau. Loss tinh duoc: 4,052583 (huu han, khong NaN). backward() MOT LAN roi kiem TAT CA 12 nhom tham so (Bang, ca hai bo Wq/Wk/Wv/W1/W2, W_out): moi nhom deu co grad TRUNG BINH TUYET DOI > 0 -- gradient lan XUYEN SUOT toan bo pipeline, khong bi ket o dau. Xac nhan pipeline chay DUNG truoc khi do thoi gian tren corpus 5KB o bai sau."
locale: vi
track: tri-tue-nhan-tao
module: boss-transformer-tu-so-0
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.rap-pipeline-tren-corpus-lon-hon]
requires: [ai.mlp-that-bai-transformer-thanh-cong]
concepts: [ai.rap-pipeline-tren-corpus-lon-hon]
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
Hai bài đầu đã trả lời XONG câu hỏi kiến trúc: residual chống vanishing, attention giải phụ thuộc xa. Ba bài còn lại của quest chuyển hướng hoàn toàn — không còn so sánh kiến trúc, mà RÁP một pipeline đầy đủ và đẩy nó lên quy mô lớn hơn hẳn mọi thứ track này từng dùng. Bước đầu: một corpus LỚN HƠN nhiều so với `70` ký tự của `khoi-transformer-va-huan-luyen`, và một vocab BPE lớn hơn.
::::

::::explain{#pipeline_day_du_corpus_lon}
Mọi hàm dùng ở đây đã xây xong, KHÔNG viết lại gì mới:

> **BPE** (`huan_luyen_bpe`, `token-hoa-bpe` q8.3a) — huấn luyện trên corpus THẬT (không phải câu ví dụ đồ chơi), lấy `vocab` lớn hơn nhiều so với các bài trước (`50` thay vì `20`).
>
> **Embedding + vị trí** (`embedding_lookup` + `ma_hoa_vi_tri`, `co-che-attention` q8.3c).
>
> **Hai khối Transformer xếp chồng** (`khoi_transformer`/`xep_chong_2_tang`, `khoi-transformer-va-huan-luyen` q8.3d) — attention thật, residual thật, layernorm thật.
>
> **Chiếu logit + mất mát dự đoán từ tiếp theo** (`mat_mat_du_doan_tiep_theo`).

Corpus lần này: một đoạn văn bản THẬT về trí tuệ nhân tạo (không phải câu ví dụ ngắn), huấn luyện BPE với mục tiêu `50` token trong vocab (thay vì `20` như các bài trước) — nhiều merge hơn, vocab phong phú hơn. Trước khi lo về THỜI GIAN chạy (bài sau, trên corpus còn lớn hơn nữa — gần `5KB`), phải xác nhận điều cơ bản hơn: pipeline có CHẠY ĐÚNG không? `loss` có tính được một số hữu hạn (không phải `NaN` hay lỗi) không? Và quan trọng nhất — gradient có thật sự LAN TỚI mọi nhóm tham số hay không, hay có nhóm nào đó bị "kẹt" ở gradient `0` do một lỗi nối dây đâu đó trong pipeline?

Cách kiểm tra: sau `backward()`, tính `|grad|` trung bình của TỪNG nhóm tham số riêng biệt (`Bang`, cả `5` tham số của khối `1`, cả `5` tham số của khối `2`, `W_out`) — `12` nhóm tất cả — và xác nhận MỌI nhóm đều có gradient khác `0`.
::::

::::example{#rap_pipeline_corpus_that}
Corpus thật (`560` ký tự), huấn luyện BPE với mục tiêu `50` token, lấy cửa sổ `16` token đầu, ráp `2` khối Transformer, tính loss và gradient:

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


def khoi_tao(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


corpus = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
"he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
"cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
"mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
"chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
"nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
"thieu ham mat mat qua gradient descent.")

muc_tieu_vocab = 50
ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)

dim, hidden = 4, 6
vocab_size = len(vocab)
so_token = 16
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
Bang, tham_so_1, tham_so_2, W_out = mo_hinh
PE = Tensor(ma_hoa_vi_tri(so_token, dim))
X = Bang.embedding_lookup(ids) + PE
X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
logits = X2.matmul(W_out)
loss = mat_mat_du_doan_tiep_theo(logits, ids)
loss.backward()

print(len(corpus), len(vocab), len(merges), len(ids_full))
print(round(float(loss.data), 6))

tat_ca_tham_so = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
for i, p in enumerate(tat_ca_tham_so):
    print(i, round(float(np.mean(np.abs(p.grad))), 6))
```

```text title=readonly
560 50 24 356
4.052583
0 0.008104
1 0.005578
2 0.007804
3 0.04764
4 0.009548
5 0.018775
6 0.00689
7 0.005899
8 0.058196
9 0.007977
10 0.018412
11 0.023517
```

Corpus `560` ký tự → vocab BPE `50` token (`24` merge, nhiều hơn hẳn `20` token/vài merge của các bài trước) → chuỗi mã hoá dài `356` token. Cửa sổ `16` token đầu đi qua ĐẦY ĐỦ `2` khối Transformer, loss ra một số HỮU HẠN: `4,052583`. Pipeline chạy được trên quy mô lớn hơn hẳn.

Mười hai nhóm tham số (`Bang`, `5` tham số của khối `1`, `5` tham số của khối `2`, `W_out`) — TẤT CẢ đều có gradient khác `0`. Không có nhóm nào "kẹt" ở gradient `0` — chứng tỏ gradient lan XUYÊN SUỐT toàn bộ pipeline, từ `loss` ngược về tới tận bảng embedding, qua cả `2` khối Transformer, không rơi rớt ở đâu.
::::

::::predict{#doan_neu_tang_vocab commitOnce}
Cửa sổ `16` token, vocab `50`: pipeline chạy được, gradient lan xuyên suốt tới cả `12` nhóm tham số.

**Trước khi đọc bài sau**, bạn đoán: nếu tăng CORPUS lên gần `5KB` (gấp gần `10` lần corpus `560` ký tự này) nhưng GIỮ NGUYÊN cửa sổ huấn luyện chỉ `16-20` token (không huấn luyện trên TOÀN BỘ corpus dài cùng lúc), việc RÁP pipeline (embedding, attention, layernorm) có cần thay đổi gì không?

:::opt{correct}
Không cần đổi gì trong cách ráp pipeline — `khoi_transformer`/`xep_chong_2_tang` chỉ phụ thuộc vào SỐ TOKEN của cửa sổ đưa vào (`so_token`, ở đây là `16-20`, không phải độ dài corpus gốc); corpus dài hơn chỉ ảnh hưởng tới bước BPE (huấn luyện vocab) và việc CHỌN cửa sổ nào để huấn luyện, không ảnh hưởng cấu trúc của khối Transformer
:::

:::opt
Cần đổi — corpus dài hơn bắt buộc phải tăng `so_token` tương ứng để mô hình "nhìn thấy" toàn bộ corpus
::why
Gần đúng ở việc nghĩ rằng mô hình cần "nhìn thấy" nhiều dữ liệu hơn khi corpus lớn hơn — trực giác đó có lý trong một số ngữ cảnh huấn luyện.

Chỗ lệch: `so_token` (kích thước cửa sổ mỗi lần huấn luyện) và độ dài TOÀN BỘ corpus là HAI đại lượng độc lập — một mô hình ngôn ngữ thật cũng không "nhìn" toàn bộ dữ liệu huấn luyện cùng lúc, mà xử lý từng cửa sổ (batch) nhỏ, LẶP LẠI qua nhiều cửa sổ khác nhau trích từ corpus lớn. Tăng corpus không bắt buộc tăng `so_token` — thực ra vì lý do ngân sách tính toán, `so_token` thường phải giữ NHỎ dù corpus có lớn tới đâu.
::
:::

:::opt
Không xác định được nếu không thử — quy mô lớn hơn luôn có nguy cơ phát sinh lỗi không lường trước
::why
Gần đúng ở sự thận trọng khi thay đổi quy mô — thận trọng đó không sai.

Chỗ lệch: cấu trúc của `khoi_transformer` (nhận `X` shape `(so_token, dim)`, không có bước nào tham chiếu tới "độ dài corpus gốc") đã đủ rõ để khẳng định trước: hàm này không có lý do cấu trúc nào để hỏng khi corpus dài hơn, MIỄN LÀ cửa sổ đưa vào (`so_token`) vẫn nhỏ như cũ. Bài sau sẽ đo THẬT để xác nhận, nhưng hướng suy luận có thể đưa ra trước từ việc đọc code.
::
:::
::::

::::code{#viet_rap_pipeline}
Hoàn thiện ba chỗ trống: chiếu đầu ra Transformer thành logit, tính mất mát, và xác nhận gradient khác `0` ở MỌI nhóm tham số.

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


corpus = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
"he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
"cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
"mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
"chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
"nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
"thieu ham mat mat qua gradient descent.")

muc_tieu_vocab = 50
ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)

dim, hidden = 4, 6
vocab_size = len(vocab)
so_token = 16
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
Bang, tham_so_1, tham_so_2, W_out = mo_hinh
PE = Tensor(ma_hoa_vi_tri(so_token, dim))
X = Bang.embedding_lookup(ids) + PE
X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
logits = ___                                # X2.matmul(W_out)
loss = ___                                  # mat_mat_du_doan_tiep_theo(logits, ids)
loss.backward()

tat_ca_tham_so = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
grad_khac_khong = ___                       # all(float(np.mean(np.abs(p.grad))) > 0 for p in tat_ca_tham_so)

print(len(corpus), len(vocab), len(merges), len(ids_full))
print(round(float(loss.data), 6))
print(grad_khac_khong)
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


def khoi_tao(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


corpus = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
"he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
"cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
"mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
"chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
"nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
"thieu ham mat mat qua gradient descent.")

muc_tieu_vocab = 50
ds_final, merges, vocab = huan_luyen_bpe(corpus, muc_tieu_vocab)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)

dim, hidden = 4, 6
vocab_size = len(vocab)
so_token = 16
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
Bang, tham_so_1, tham_so_2, W_out = mo_hinh
PE = Tensor(ma_hoa_vi_tri(so_token, dim))
X = Bang.embedding_lookup(ids) + PE
X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
logits = X2.matmul(W_out)
loss = mat_mat_du_doan_tiep_theo(logits, ids)
loss.backward()

tat_ca_tham_so = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
grad_khac_khong = all(float(np.mean(np.abs(p.grad))) > 0 for p in tat_ca_tham_so)

print(len(corpus), len(vocab), len(merges), len(ids_full))
print(round(float(loss.data), 6))
print(grad_khac_khong)
```

```python title=test
assert len(corpus) == 560, f"do dai corpus sai -- dang ra {len(corpus)}"
assert len(vocab) == 50, f"vocab sai -- dang ra {len(vocab)}"
assert len(merges) == 24, f"so merges sai -- dang ra {len(merges)}"
assert len(ids_full) == 356, f"do dai ids_full sai -- dang ra {len(ids_full)}"
assert round(float(loss.data), 6) == 4.052583, f"loss sai -- dang ra {round(float(loss.data), 6)}"
assert grad_khac_khong == True, "grad_khac_khong phai la True -- gradient phai lan toi MOI nhom tham so"

# rieng kiem tra tung nhom tham so mot -- khong duoc chi kiem mot phan roi
# tra ve True cho ca 12 nhom
for i, p in enumerate(tat_ca_tham_so):
    g = float(np.mean(np.abs(p.grad)))
    assert g > 0, f"nhom tham so thu {i} co gradient bang 0 -- pipeline bi ket o day"
assert len(tat_ca_tham_so) == 12, f"phai co dung 12 nhom tham so -- dang ra {len(tat_ca_tham_so)}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu chiếu đầu ra Transformer thành logit trên vocab — `X2.matmul(W_out)`. Chỗ hai tính mất mát dự đoán từ tiếp theo — `mat_mat_du_doan_tiep_theo(logits, ids)`. Chỗ ba xác nhận MỌI nhóm tham số đều có gradient trung bình tuyệt đối lớn hơn `0` — `all(float(np.mean(np.abs(p.grad))) > 0 for p in tat_ca_tham_so)`.
- kind: strategy
  body: 'Chỗ đầu: `X2.matmul(W_out)`. Chỗ hai: `mat_mat_du_doan_tiep_theo(logits, ids)`. Chỗ ba: `all(float(np.mean(np.abs(p.grad))) > 0 for p in tat_ca_tham_so)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `X2.matmul(W_out)`, `mat_mat_du_doan_tiep_theo(logits, ids)`, và `all(float(np.mean(np.abs(p.grad))) > 0 for p in tat_ca_tham_so)`.'
:::

:::validate
- tier: run
  timeoutMs: 20000
- tier: static
  onFail: logits phai la X2.matmul(W_out) THAT (khong duoc chep san hay bo qua phep chieu); loss phai goi THAT mat_mat_du_doan_tiep_theo(logits, ids); grad_khac_khong phai dung all(...) THAT tren MOI nhom tham so, so sanh '>' voi 0 (khong duoc chep san True, khong doi thanh '>=')
  requireAst:
  - kind: uses-call, target: matmul, min: 8
  - kind: uses-call, target: mat_mat_du_doan_tiep_theo, min: 1
  - kind: uses-call, target: all, min: 1
  - kind: comprehension, min: 3
  - kind: uses-operator, target: ">", min: 2
  # Da thu THAT bang kiemAst that (goi truc tiep tren code trich tu solution
  # da bien dich, khong doan tay).
  # matmul=8 tren toan bo solution: 7 lan CO SAN ben trong khoi_transformer
  # (Wq,Wk,Wv,K.transpose(),P.matmul(V),X1.matmul(W1),H.matmul(W2)), 1 lan
  # trong blank1 "X2.matmul(W_out)". Dien bua blank1 thanh mot bien khac
  # (khong goi matmul) lam so nay tut xuong 7 -- duoi nguong min=8, bi chan.
  # mat_mat_du_doan_tiep_theo=1: CHI mot lan goi trong blank2 -- dinh nghia
  # ham (def mat_mat_du_doan_tiep_theo(...):) khong tinh la Call. Dien bua
  # "loss = Tensor(0.0)" (bo qua goi ham that) lam so nay tut xuong 0 --
  # duoi nguong min=1, bi chan.
  # all=1, comprehension=3: "all=1" CHI mot lan goi trong blank3. Dien bua
  # "grad_khac_khong = True" (chep san) lam "all" tut xuong 0 -- bi chan
  # RIENG. comprehension=3 tren toan bo solution: 1 dict comp trong
  # xay_token_sang_id, 1 tu blank3 (genexp "... for p in tat_ca_tham_so"),
  # CONG 1 nua rai rac trong ham phu tro con lai -- da kiem THAT bang
  # kiemAst, tong dung la 3 (khong phai 2 nhu uoc luong tay ban dau). Dien
  # bua blank3 thanh hang so lam comprehension tut xuong 2 -- duoi nguong
  # min=3, bi chan RIENG doc lap voi luat "all".
  # ">"=2: 1 lan CO SAN trong relu()._backward ("self.data > 0"), 1 lan
  # trong blank3 (">0" ben trong genexp). Dien bua blank3 thanh
  # "grad_khac_khong = True" lam ">" tut xuong 1 -- duoi nguong min=2, bi
  # chan RIENG boi luat nay -- ba luat DOC LAP deu chan cung mot cheat nay.
  # Cheat doi '>' thanh '>=' (bien the bien): tren du lieu THAT cua bai nay,
  # moi gradient trung binh tuyet doi deu la mot so THUC DUONG (khong bao
  # gio dung bang 0 chinh xac trong thuc te tinh toan nay), nen '>' va '>='
  # cho CUNG ket qua True cho ca 12 nhom -- CHI static rieng moi bat duoc
  # (">" tut tu 2 xuong 1), khong output/tests nao lam duoc vi ca hai cho
  # cung grad_khac_khong=True.
- tier: tests
  timeoutMs: 20000
- tier: output
  match: regex
  expect: "^560 50 24 356\\n4\\.052583\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Corpus `560` ký tự, vocab `50` token, cửa sổ `16` token — loss `4,052583`, gradient khác `0` ở CẢ `12` nhóm tham số. Pipeline chạy đúng trên quy mô lớn hơn hẳn các bài trước. Bài sau: đẩy corpus lên gần `5KB` — đúng con số MASTERPLAN yêu cầu — và đo THỜI GIAN thật.
::::

::::reflect{#nghi-lai}
Bài này không có gì MỚI về mặt lý thuyết — mọi hàm đều tái dùng nguyên vẹn từ `token-hoa-bpe`, `co-che-attention`, `khoi-transformer-va-huan-luyen`. Điều bài này xác nhận là một sự thật KỸ THUẬT quan trọng: pipeline không có giả định ẩn nào về quy mô nhỏ — BPE huấn luyện được trên corpus `560` ký tự (gấp `8` lần corpus các bài trước), và gradient vẫn lan xuyên suốt `12` nhóm tham số không sai khác gì so với quy mô nhỏ.

Bài tiếp theo đẩy quy mô lên xa hơn nữa — corpus gần `5KB`, đúng con số MASTERPLAN yêu cầu cho T8.3 — và lần đầu tiên trong track này, đo THỜI GIAN CHẠY THẬT bằng đồng hồ, không chỉ đo số bước hay giá trị loss.
::::

::::checkpoint{mastery=0.8}
::::
