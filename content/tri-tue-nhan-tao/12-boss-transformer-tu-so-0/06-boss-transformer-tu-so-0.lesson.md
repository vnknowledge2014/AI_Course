---
id: tri-tue-nhan-tao.boss-transformer-tu-so-0.boss-transformer-tu-so-0
title: "BOSS — Đóng T8.3 tại 40/40: BPE → embedding → Transformer → huấn luyện → sinh văn bản"
summary: "Rap lai MOT LUOT ngan gon: BPE (28 token, 6 merge, corpus 80 ky tu 'transformer dung attention...') -> embedding+vi tri -> 2 khoi Transformer that (residual+layernorm+attention) -> 15 buoc SGD -> sinh van ban, TREN CUNG mot corpus. Cua so huan luyen giai ma DUNG thanh 'transformer'. Xac suat token dung tang 0,030955 -> 0,331933 (gap 10,72 lan), loss giam DON DIEU 3,503815 -> 0,922278, sinh van ban tu 'trans' truoc huan luyen ra 'transke d d dkye' (vo nghia), SAU huan luyen ra 'transformrsfo' (chua dung 'transform'). Thoi gian huan luyen that do bang time.perf_counter(): duoi 2 giay. Ket luan boolean tong hop CA BA bang chung (loss giam VA xac suat tang VA duoi 2 giay) deu True. Dong T8.3 (40/40): tong ket ca 5 quest, ban giao T8.4 'Dung LLM dung cach'."
locale: vi
track: tri-tue-nhan-tao
module: boss-transformer-tu-so-0
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-transformer-tu-so-0]
requires: [ai.gioi-han-that-cua-micro-transformer]
concepts: [ai.boss-transformer-tu-so-0]
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
Năm bài: residual chống vanishing, attention giải phụ thuộc xa, pipeline chạy đúng ở quy mô lớn, dưới `2` giây trên `5KB`, và giới hạn quá khớp đo được bằng số thật. Bài này ráp TẤT CẢ lại một lượt cuối cùng — và đóng cả T8.3 tại `40/40`.
::::

::::explain{#rap_toan_bo_lan_cuoi}
Pipeline đầy đủ, KHÔNG thêm gì mới — đúng những gì `khoi-transformer-va-huan-luyen` (q8.3d) đã xây và quest này vừa kiểm chứng ở quy mô lớn hơn:

> **BPE** (`huan_luyen_bpe`) — huấn luyện trên corpus, mã hoá thành token ID.
>
> **Embedding + vị trí** (`embedding_lookup` + `ma_hoa_vi_tri`).
>
> **Hai khối Transformer xếp chồng** (`khoi_transformer`/`xep_chong_2_tang`) — attention + residual + layernorm + feedforward, ĐÚNG kiến trúc đã chứng minh không vanishing gradient (bài `1` của quest này) nhờ residual.
>
> **Chiếu logit + mất mát dự đoán từ tiếp theo + vòng lặp huấn luyện** (`mot_buoc`, SGD).
>
> **Sinh văn bản** (`sinh_van_ban`, greedy decoding).

Corpus lần này: `"transformer dung attention de hoc tu du lieu, khong can de quy nhu mang hoi quy."` — một câu tổng kết chính chủ đề của track này. BPE mã hoá cửa sổ `10` token ĐẦU của câu — và nó giải mã ĐÚNG thành `"transformer"`. Huấn luyện `15` bước, đo BA con số tổng kết ĐỘC LẬP: loss có giảm không, xác suất token đúng có tăng không, và — quay lại đúng yêu cầu trung tâm MASTERPLAN — thời gian huấn luyện có dưới `2` giây không.
::::

::::example{#boss_truoc_huan_luyen}
Trước khi huấn luyện bước nào: BPE hoá corpus, sinh văn bản từ `"trans"`, đo xác suất token đúng tại vị trí `4` của cửa sổ huấn luyện:

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

mo_hinh = khoi_tao(9, vocab_size, dim, hidden)

p_truoc = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_truoc = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 8)

print(len(corpus), len(vocab), len(merges))
print(giai_ma(ids_huan_luyen, id_sang_token))
print(round(p_truoc, 6))
print(giai_ma(sinh_truoc, id_sang_token))
```

```text title=readonly
80 28 6
transformer
0.030955
transke d d dkye
```

Câu `80` ký tự → vocab BPE `28` token (`6` merge). Cửa sổ `10` token đầu giải mã ĐÚNG thành `"transformer"` — CHÍNH TỪ trung tâm của cả track này. Tham số CHƯA huấn luyện (`seed=9`): xác suất token đúng tại vị trí `4` chỉ `0,030955` — thấp hơn mức đoán ngẫu nhiên đều (`1/28 ≈ 0,0357`). Sinh văn bản từ `"trans"` cho `"transke d d dkye"` — vô nghĩa, đúng như một mô hình chưa học gì.
::::

::::example{#boss_sau_huan_luyen}
Huấn luyện đúng `15` bước (SGD, `lr = 0,5`), ĐO THỜI GIAN THẬT bằng `time.perf_counter()`, rồi đo lại đúng các con số trên TRÊN CÙNG MÔ HÌNH:

```python title=readonly
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


mask = xay_mat_na_nhan_qua(so_token)

t_bat_dau = time.perf_counter()
lich_su_loss = [mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5) for _ in range(15)]
t_ket_thuc = time.perf_counter()
thoi_gian = t_ket_thuc - t_bat_dau

p_sau = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_sau = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 8)

loss_giam = lich_su_loss[-1] < lich_su_loss[0]
xac_suat_tang = p_sau > p_truoc
huan_luyen_nhanh = thoi_gian < 2.0
ket_luan = loss_giam and xac_suat_tang and huan_luyen_nhanh

print([round(l, 6) for l in lich_su_loss])
print(round(p_sau, 6), round(p_sau / p_truoc, 4))
print(giai_ma(sinh_sau, id_sang_token))
print(loss_giam, xac_suat_tang, huan_luyen_nhanh)
print(ket_luan)
```

```text title=readonly
[3.503815, 3.071358, 2.729123, 2.437084, 2.186847, 1.971653, 1.795636, 1.648114, 1.548778, 1.470773, 1.341421, 1.21704, 1.09373, 1.002888, 0.922278]
0.331933 10.7231
transformrsfo
True True True
True
```

Bốn bằng chứng ĐỘC LẬP, đo bằng số thật, TẤT CẢ cùng một hướng:

> **Loss giảm ĐƠN ĐIỆU** — `3,503815 → 0,922278` qua `15` bước.
>
> **Xác suất token đúng tăng** — `0,030955 → 0,331933`, gấp `10,7231` lần.
>
> **Văn bản sinh ra thay đổi có ý nghĩa** — từ `"transke d d dkye"` (vô nghĩa) sang `"transformrsfo"` (chứa đúng `"transform"` — mô hình bắt đầu tái tạo lại chính từ nó vừa được huấn luyện trên).
>
> **Huấn luyện dưới `2` giây** — đúng yêu cầu MASTERPLAN, đo bằng `time.perf_counter()` thật, không suy luận.

`ket_luan = True` — tổng hợp CẢ BỐN bằng chứng vào một con số duy nhất.
::::

::::predict{#doan_tong_ket_ca_track commitOnce}
Ba bằng chứng độc lập (loss giảm, xác suất tăng, huấn luyện nhanh) đều `True` — `ket_luan = True`.

**Trước khi đọc phần reflect**, bạn đoán: nếu MỘT trong ba bằng chứng này là `False` (ví dụ giả sử `huan_luyen_nhanh = False` vì máy chạy chậm bất thường), điều đó có làm mất giá trị của HAI bằng chứng còn lại (`loss_giam`, `xac_suat_tang`) hay không?

:::opt{correct}
Không — ba bằng chứng đo BA khía cạnh ĐỘC LẬP (giá trị hàm mất mát, xác suất dự đoán, thời gian đồng hồ); một bằng chứng sai không thể "lây" sang làm sai các phép đo KHÁC, vì chúng được tính từ những công thức và dữ liệu hoàn toàn tách biệt
:::

:::opt
Có — nếu một phần của hệ thống có vấn đề, toàn bộ kết quả phải bị nghi ngờ
::why
Gần đúng ở sự thận trọng khi một phần của một hệ thống LỚN gặp vấn đề — thận trọng đó hợp lý trong nhiều ngữ cảnh gỡ lỗi phần mềm phức tạp, liên kết chặt.

Chỗ lệch: ở đây BA phép đo này được tính TỪ BA công thức riêng biệt, trên CÙNG một mô hình đã huấn luyện nhưng đo BA thứ khác nhau (giá trị số của loss, giá trị số của một xác suất, và một phép trừ hai lần gọi đồng hồ) — không có kết nối logic nào khiến sai số hay biến động ở PHÉP ĐO THỜI GIAN (vốn phụ thuộc tải hệ thống, không phụ thuộc đúng/sai của thuật toán) lây sang giá trị `loss` hay `xác suất` (những con số xác định hoàn toàn bởi phép tính toán học, không phụ thuộc đồng hồ).
::
:::

:::opt
Không xác định được — cần biết chi tiết NGUYÊN NHÂN gây ra sự sai lệch trước khi kết luận
::why
Gần đúng ở tinh thần thận trọng, muốn hiểu rõ nguyên nhân trước khi kết luận — nguyên tắc hữu ích trong debug.

Chỗ lệch: câu hỏi ở đây không phải "vì sao một phép đo sai" mà là "một phép đo CÓ THỂ ảnh hưởng phép đo khác hay không" — và cấu trúc TOÁN HỌC của ba công thức (loss từ cross-entropy, xác suất từ softmax, thời gian từ hiệu hai lần gọi đồng hồ) đã đủ để khẳng định TRƯỚC: chúng độc lập về mặt tính toán, không cần biết nguyên nhân cụ thể của bất kỳ sai lệch nào để biết điều đó.
::
:::
::::

::::code{#viet_boss_dong_track}
Hoàn thiện ba chỗ trống: chạy vòng lặp huấn luyện `15` bước, tính lại xác suất token đúng SAU huấn luyện, và tổng hợp CẢ BA bằng chứng thành một kết luận duy nhất.

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

p_truoc = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_truoc = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 8)

t_bat_dau = time.perf_counter()
lich_su_loss = ___                          # [mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5) for _ in range(15)]
t_ket_thuc = time.perf_counter()
thoi_gian = t_ket_thuc - t_bat_dau

p_sau = ___                                 # xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_sau = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 8)

loss_giam = lich_su_loss[-1] < lich_su_loss[0]
xac_suat_tang = p_sau > p_truoc
huan_luyen_nhanh = thoi_gian < 2.0
ket_luan = ___                              # loss_giam and xac_suat_tang and huan_luyen_nhanh

print(len(corpus), len(vocab), len(merges))
print(round(p_truoc, 6), round(p_sau, 6))
print(giai_ma(sinh_truoc, id_sang_token))
print(giai_ma(sinh_sau, id_sang_token))
print(loss_giam, xac_suat_tang, huan_luyen_nhanh)
print(ket_luan)
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

p_truoc = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_truoc = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 8)

t_bat_dau = time.perf_counter()
lich_su_loss = [mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5) for _ in range(15)]
t_ket_thuc = time.perf_counter()
thoi_gian = t_ket_thuc - t_bat_dau

p_sau = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)
sinh_sau = sinh_van_ban(mo_hinh, ids_mo_dau, dim, 8)

loss_giam = lich_su_loss[-1] < lich_su_loss[0]
xac_suat_tang = p_sau > p_truoc
huan_luyen_nhanh = thoi_gian < 2.0
ket_luan = loss_giam and xac_suat_tang and huan_luyen_nhanh

print(len(corpus), len(vocab), len(merges))
print(round(p_truoc, 6), round(p_sau, 6))
print(giai_ma(sinh_truoc, id_sang_token))
print(giai_ma(sinh_sau, id_sang_token))
print(loss_giam, xac_suat_tang, huan_luyen_nhanh)
print(ket_luan)
```

```python title=test
assert len(corpus) == 80, f"do dai corpus sai -- dang ra {len(corpus)}"
assert len(vocab) == 28, f"vocab sai -- dang ra {len(vocab)}"
assert len(merges) == 6, f"so merges sai -- dang ra {len(merges)}"
assert round(p_truoc, 6) == 0.030955, f"p_truoc sai -- dang ra {round(p_truoc, 6)}"
assert round(p_sau, 6) == 0.331933, f"p_sau sai -- dang ra {round(p_sau, 6)}"
lich_su_lam_tron = [round(l, 6) for l in lich_su_loss]
assert lich_su_lam_tron == [3.503815, 3.071358, 2.729123, 2.437084, 2.186847, 1.971653, 1.795636, 1.648114, 1.548778, 1.470773, 1.341421, 1.21704, 1.09373, 1.002888, 0.922278], f"lich_su_loss sai -- dang ra {lich_su_lam_tron}"
assert giai_ma(sinh_truoc, id_sang_token) == "transke d d dkye", f"van ban TRUOC huan luyen sai -- dang ra {giai_ma(sinh_truoc, id_sang_token)!r}"
assert giai_ma(sinh_sau, id_sang_token) == "transformrsfo", f"van ban SAU huan luyen sai -- dang ra {giai_ma(sinh_sau, id_sang_token)!r}"
assert loss_giam == True, "loss_giam phai la True"
assert xac_suat_tang == True, "xac_suat_tang phai la True"
assert huan_luyen_nhanh == True, f"huan_luyen_nhanh phai la True -- thoi gian do duoc la {thoi_gian}"
assert ket_luan == True, "ket_luan phai la True -- CA BA bang chung deu phai dung"

# BANG CHUNG TRUNG TAM cua BOSS: bon do luong doc lap (loss, xac suat, van
# ban, thoi gian) deu phai xac nhan huan luyen co hieu qua VA nhanh
assert p_sau / p_truoc > 5.0, f"xac suat dung phai tang it nhat gap 5 lan -- dang ra ti le {p_sau / p_truoc}"
assert 0.0 < thoi_gian < 2.0, f"thoi_gian phai la so THAT trong khoang (0, 2.0) -- dang ra {thoi_gian}"
assert giai_ma(ids_huan_luyen, id_sang_token) == "transformer", f"cua so huan luyen phai giai ma dung thanh 'transformer' -- dang ra {giai_ma(ids_huan_luyen, id_sang_token)!r}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu chạy `15` bước huấn luyện, ghi lại `loss` mỗi bước — `[mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5) for _ in range(15)]`. Chỗ hai tính LẠI xác suất token đúng SAU khi đã huấn luyện, gọi lại TRÊN mô hình vừa cập nhật — `xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)`. Chỗ ba tổng hợp CẢ BA điều kiện bằng `and` — `loss_giam and xac_suat_tang and huan_luyen_nhanh`.
- kind: strategy
  body: 'Chỗ đầu: `[mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5) for _ in range(15)]`. Chỗ hai: `xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)`. Chỗ ba: `loss_giam and xac_suat_tang and huan_luyen_nhanh`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là vòng lặp `mot_buoc` `15` lần, `xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4)`, và `loss_giam and xac_suat_tang and huan_luyen_nhanh`.'
:::

:::validate
- tier: run
  timeoutMs: 20000
- tier: static
  onFail: lich_su_loss phai goi THAT mot_buoc(...) 15 lan qua list comprehension (khong duoc chep san danh sach); p_sau phai duoc tinh THAT bang xac_suat_dung_tai(...) tren mo hinh DA huan luyen (khong duoc chep lai p_truoc hay mot hang so); ket_luan phai la phep 'and' THAT giua ca ba dieu kien (khong duoc chep san True, khong doi thanh 'or')
  requireAst:
  - kind: uses-call, target: mot_buoc, min: 1
  - kind: uses-call, target: xac_suat_dung_tai, min: 2
  - kind: uses-operator, target: and, min: 2
  # Da thu THAT bang kiemAst that (goi truc tiep tren code trich tu solution
  # da bien dich, khong doan tay).
  # mot_buoc=1: CHI mot lan goi trong blank1 (list comprehension) -- dinh
  # nghia ham (def mot_buoc(...):) khong tinh la Call. Dien bua
  # "lich_su_loss = [3.5] * 15" (chep san, khong goi ham THAT) lam so nay
  # tut xuong 0 -- duoi nguong min=1, bi chan.
  # xac_suat_dung_tai=2: 1 lan CO SAN cho p_truoc (dong "p_truoc =
  # xac_suat_dung_tai(...)", da co san trong starter, KHONG phai blank), 1
  # lan trong blank2. Dien bua blank2 thanh "p_sau = p_truoc" (chep lai,
  # bo qua tinh lai) lam so nay tut xuong 1 -- duoi nguong min=2, bi chan.
  # and=2: 1 lan CO SAN trong gop_cap ("if i < len(danh_sach)-1 and
  # danh_sach[i]==a and danh_sach[i+1]==b" -- ba toan hang noi tiep nhau
  # duoc AST goi la MOT node BoolOp duy nhat, dem la 1), 1 lan trong blank3
  # (cung la MOT BoolOp voi ba toan hang). Dien bua blank3 thanh
  # "ket_luan = True" (chep san) lam "and" tut xuong 1 -- duoi nguong
  # min=2, bi chan.
  # Cheat doi 'and' thanh 'or' o blank3 (bien the toan tu logic): tren du
  # lieu THAT cua bai nay, ca ba dieu kien (loss_giam, xac_suat_tang,
  # huan_luyen_nhanh) DEU la True nen 'and' va 'or' cho CUNG ket qua True
  # -- CHI static rieng moi bat duoc ("and" tut tu 2 xuong 1, vi "or" la
  # mot BoolOp KHAC, khong khop target "and"), khong output/tests nao bat
  # duoc vi ca hai cho cung ket_luan=True.
  # Cheat "p_sau = xac_suat_dung_tai(mo_hinh, ids_huan_luyen, dim, 4) * 1.0"
  # (goi ham THAT nhung nhan them 1.0 vo nghia) KHONG doi so dem AST nao,
  # NHUNG khong doi gia tri so hoc -- van dat moi kiem tra; day KHONG phai
  # mot cheat that su, chi la mot bien the toan hoc tuong duong.
- tier: tests
  timeoutMs: 20000
- tier: output
  match: regex
  expect: "^80 28 6\\n0\\.030955 0\\.331933\\ntranske d d dkye\\ntransformrsfo\\nTrue True True\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Loss `3,503815 → 0,922278`. Xác suất đúng `0,030955 → 0,331933`, gấp `10,7231` lần. Văn bản `"transke d d dkye" → "transformrsfo"` — chứa đúng `"transform"`. Thời gian huấn luyện dưới `2` giây, đúng MASTERPLAN. Bốn bằng chứng độc lập, một kết luận `True`. T8.3 "Transformer từ số 0" đóng tại `40/40`.
::::

::::reflect{#nghi-lai}
T8.3 "Transformer từ số 0" khép lại tại đây — `5` quest, `40` bài:

> **`token-hoa-bpe`** (q8.3a, `8` bài) — vì sao cần token hoá, thuật toán BPE tự viết từ đầu, mã hoá/giải mã round-trip.
>
> **`dong-co-tensor`** (q8.3b, `8` bài) — lớp `Tensor` bọc `numpy`, nhân ma trận, gradient qua ma trận, gradient-check.
>
> **`co-che-attention`** (q8.3c, `9` bài) — embedding, vị trí, Query/Key/Value, self-attention, causal mask, chạy trên `4` token đối chiếu byte-for-byte.
>
> **`khoi-transformer-va-huan-luyen`** (q8.3d, `9` bài) — kết nối tắt, mạng truyền thẳng theo vị trí, một khối Transformer hoàn chỉnh, xếp chồng `2` tầng, mất mát dự đoán từ tiếp theo, vòng lặp huấn luyện, sinh văn bản.
>
> **`boss-transformer-tu-so-0`** (q8.3e, `6` bài, quest này) — vì sao attention không vanishing (residual, không phải attention, là lý do), MLP thất bại có cấu trúc trên bài phụ thuộc xa mà Transformer giải được, pipeline chạy đúng ở quy mô lớn, huấn luyện dưới `2` giây trên corpus gần `5KB` (đúng MASTERPLAN), giới hạn quá khớp đo được bằng số thật, và BOSS này đóng track.

Toàn bộ track được xây trên đúng MỘT nguyên tắc xuyên suốt: đo THẬT bằng số, không suy luận công thức mà không kiểm chứng. Từ đạo hàm bằng số (`kiem-dao-ham-bang-so`, T8.2) tới gradient-check ma trận (q8.3b), từ so byte-for-byte với tính tay (q8.3c) tới đo thời gian đồng hồ thật (bài `4` quest này) — mọi khẳng định trong `40` bài đều có một con số thật đứng sau nó.

T8.4 "Dùng LLM đúng cách" là bước tiếp theo của MASTERPLAN — và nó đánh dấu một CHUYỂN HƯỚNG hoàn toàn: từ XÂY DỰNG một mô hình (những gì R8 làm từ T8.1 tới đây — hồi quy, mạng nơ-ron, Transformer, tất cả tự tay viết từ số `0`) sang SỬ DỤNG một mô hình đã có sẵn (LLM sản xuất thật, hàng trăm tỉ tham số, huấn luyện trên hàng nghìn tỉ token — đúng thứ mà `micro-transformer` `dim = 4` của quest này, dù đã chứng minh đúng NGUYÊN LÝ, không thể nào sánh được về QUY MÔ). Hiểu được BÊN TRONG một Transformer hoạt động thế nào — như `40` bài vừa qua đã làm — là nền tảng để dùng LLM một cách có ý thức, không phải một hộp đen.
::::

::::checkpoint{mastery=1.0}
::::
