---
id: tri-tue-nhan-tao.khoi-transformer-va-huan-luyen.huan-luyen-tren-corpus-that
title: "Vòng lặp huấn luyện trên corpus thật: 12 bước, loss đo được giảm dần"
summary: "Lap lai mot_buoc (forward->loss->backward->SGD->reset grad, bai truoc) DUNG 12 lan tren cua so 10 token dau cua corpus BPE that (khong phai du lieu do choi). Loss do THAT qua 12 buoc (lr=0,5): 3,351352 -> 2,989879 -> 2,739706 -> 2,53488 -> 2,361116 -> 2,210683 -> 2,077004 -> 1,954192 -> 1,838194 -> 1,727511 -> 1,62595 -> 1,562128 -- GIAM DON DIEU ca 12 buoc. GOTCHA THAT: SGD lr co dinh KHONG dam bao giam don dieu MAI MAI -- chay THEM buoc 13 cho loss 1,681022, LON HON buoc 12, xac nhan bang so that rang day la hien tuong that (vuot qua vung cuc tieu), khong phai loi cai dat."
locale: vi
track: tri-tue-nhan-tao
module: khoi-transformer-va-huan-luyen
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.huan-luyen-tren-corpus-that]
requires: [ai.mot-buoc-cap-nhat]
concepts: [ai.huan-luyen-tren-corpus-that]
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
Một bước cập nhật làm loss giảm từ `3,351352` xuống `2,989879`. Một bước không chứng minh được gì cả — lặp lại nó nhiều lần, trên corpus THẬT, mới thấy huấn luyện có thật sự hiệu quả hay không.
::::

::::explain{#vong_lap_huan_luyen_corpus_that}
Vòng lặp huấn luyện chỉ đơn giản là gọi `mot_buoc_co_reset` (bài trước) LIÊN TỤC, ghi lại `loss` mỗi bước:

> `lich_su_loss = [mot_buoc(...) for _ in range(so_buoc)]`

`corpus` ở đây là văn bản THẬT (không phải một tập số đồ chơi bịa ra): `"may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."` — huấn luyện BPE (`token-hoa-bpe`, q8.3a) trên đó để lấy `vocab`/`merges`, rồi lấy `10` token ĐẦU của chuỗi đã mã hoá làm cửa sổ huấn luyện. Số bước dùng ở đây là `12` — đủ để THẤY xu hướng giảm rõ ràng bằng số thật, không cần hàng nghìn bước như huấn luyện LLM thật (đây là một mô hình MINH HOẠ, `dim = 4`, không phải một mô hình sản xuất).

Một điều CẦN đo, không suy luận: SGD với `lr` cố định có đảm bảo loss giảm ở MỌI bước hay không? Câu trả lời (kiểm bằng số thật ở phần dưới) là KHÔNG — SGD chỉ đảm bảo di chuyển theo hướng làm giảm loss CỤC BỘ tại điểm hiện tại, dùng đúng MỘT bước nhảy kích thước `lr`; bước nhảy đó có thể "vọt qua" một vùng cực tiểu và làm loss tăng TẠM THỜI, trước khi lại giảm tiếp ở bước sau.
::::

::::example{#huan_luyen_12_buoc_that}
Chạy đúng `12` bước, ghi lại `loss` mỗi bước:

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


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)
dim, hidden, so_token = 4, 6, 10
vocab_size = len(vocab)
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
lich_su_loss = [mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5) for _ in range(12)]

print([round(l, 6) for l in lich_su_loss])
print(all(lich_su_loss[i + 1] < lich_su_loss[i] for i in range(len(lich_su_loss) - 1)))
```

```text title=readonly
[3.351352, 2.989879, 2.739706, 2.53488, 2.361116, 2.210683, 2.077004, 1.954192, 1.838194, 1.727511, 1.62595, 1.562128]
True
```

`12` bước, loss giảm ĐƠN ĐIỆU ở MỌI bước — từ `3,351352` (gần mức đoán ngẫu nhiên trên `20` token, `≈ 3,0`) xuống `1,562128`. Đây KHÔNG phải dữ liệu đồ chơi tự bịa: `corpus` là một câu tiếng Việt không dấu THẬT, đi qua ĐÚNG pipeline BPE đã xây ở `token-hoa-bpe` (q8.3a).
::::

::::example{#buoc_13_khong_don_dieu}
Chạy TIẾP một bước nữa (bước `13`, cùng mô hình vừa huấn luyện `12` bước ở trên, KHÔNG reset lại từ đầu):

```python title=readonly
loss_buoc_13 = mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5)
print(round(loss_buoc_13, 6))
print(round(lich_su_loss[-1], 6))
print(loss_buoc_13 < lich_su_loss[-1])
```

```text title=readonly
1.681022
1.562128
False
```

Bước `13` cho loss `1,681022` — LỚN HƠN bước `12` (`1,562128`). SGD với `lr` cố định không đảm bảo giảm đơn điệu MÃI MÃI — bước nhảy kích thước `lr = 0,5` đã "vọt qua" một vùng tốt hơn, làm loss tăng tạm thời. Đây là hiện tượng THẬT của tối ưu hoá bằng gradient descent, không phải lỗi cài đặt (bài `so-sanh-toi-uu-hoa`, q8.2d, đã nói tới hiện tượng tương tự khi so `lr` giữa các bộ tối ưu).
::::

::::predict{#doan_buoc_13 commitOnce}
Ví dụ trên xác nhận: sau `12` bước giảm đơn điệu, bước `13` lại cho loss LỚN HƠN bước `12`.

**Trước khi đọc lại**, bạn đoán: sự thật này (SGD không đảm bảo giảm đơn điệu mãi mãi) có mâu thuẫn với khẳng định "huấn luyện `12` bước làm loss giảm THẬT" ở ví dụ đầu bài hay không?

:::opt{correct}
Không mâu thuẫn — khẳng định "`12` bước làm loss giảm" là một quan sát THỰC NGHIỆM trên đúng `12` bước đó (đã đo, đúng), không phải một CAM KẾT rằng MỌI bước tiếp theo, mãi mãi, cũng phải giảm; SGD chỉ đảm bảo hướng đi CỤC BỘ tại mỗi bước, không đảm bảo một đường loss giảm đơn điệu vô hạn
:::

:::opt
Có mâu thuẫn — nếu huấn luyện THẬT SỰ hiệu quả, loss phải giảm ở MỌI bước, không có ngoại lệ nào
::why
Gần đúng ở việc mong đợi huấn luyện hiệu quả LÀM GIẢM loss theo một xu hướng chung — kỳ vọng đó không sai về TỔNG THỂ.

Chỗ lệch: "huấn luyện hiệu quả" không có nghĩa là "giảm ở MỌI bước không ngoại lệ" — SGD với `lr` cố định là một thuật toán CỤC BỘ, XẤP XỈ, có thể vọt qua một vùng tốt rồi loss tăng tạm thời trước khi tiếp tục giảm. `12` bước đầu giảm đơn điệu là một quan sát THẬT của riêng chuỗi `12` bước đó — không phải một định lý áp dụng cho MỌI bước tương lai.
::
:::

:::opt
Có, và nó cho thấy mô hình đã học SAI ở đâu đó trong `12` bước trước — cần kiểm tra lại code
::why
Gần đúng ở phản xạ NGHI NGỜ khi thấy một con số đi ngược kỳ vọng — phản xạ đó hữu ích khi debug nhiều tình huống khác.

Chỗ lệch: đây không phải dấu hiệu của một lỗi cài đặt — bài `so-sanh-toi-uu-hoa` (q8.2d) đã ghi nhận hiện tượng loss dao động khi `lr` không đủ nhỏ là một hành vi BÌNH THƯỜNG của gradient descent, không phải bằng chứng của một lỗi. `12` bước trước đã được kiểm bằng cả `output` LẪN `tests` (giá trị khớp chính xác) — không có gì gợi ý chúng sai.
::
:::
::::

::::code{#viet_vong_lap_huan_luyen}
Hoàn thiện hai chỗ trống: ghi lại `loss` mỗi bước vào lịch sử, và kiểm tra loss CUỐI có nhỏ hơn loss ĐẦU hay không.

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


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)
dim, hidden, so_token = 4, 6, 10
vocab_size = len(vocab)
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
lich_su_loss = []
for _ in range(12):
    lich_su_loss.append(___)                    # mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5)

giam_han = ___                                  # lich_su_loss[-1] < lich_su_loss[0]

print([round(l, 6) for l in lich_su_loss])
print(giam_han)
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


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)
dim, hidden, so_token = 4, 6, 10
vocab_size = len(vocab)
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh = khoi_tao(8, vocab_size, dim, hidden)
lich_su_loss = []
for _ in range(12):
    lich_su_loss.append(mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5))

giam_han = lich_su_loss[-1] < lich_su_loss[0]

print([round(l, 6) for l in lich_su_loss])
print(giam_han)
```

```python title=test
lich_su_lam_tron = [round(l, 6) for l in lich_su_loss]
assert lich_su_lam_tron == [3.351352, 2.989879, 2.739706, 2.53488, 2.361116, 2.210683, 2.077004, 1.954192, 1.838194, 1.727511, 1.62595, 1.562128], f"lich_su_loss sai -- dang ra {lich_su_lam_tron}"
assert giam_han == True, "giam_han phai la True -- loss cuoi phai nho hon loss dau"
assert len(lich_su_loss) == 12, f"phai chay dung 12 buoc -- dang ra {len(lich_su_loss)}"

# rieng kiem tra GIAM DON DIEU o ca 12 buoc (manh hon giam_han, vi giam_han
# chi so sanh dau/cuoi)
assert all(lich_su_loss[i + 1] < lich_su_loss[i] for i in range(len(lich_su_loss) - 1)), "loss phai giam DON DIEU qua tung buoc (khong buoc nao tang) trong 12 buoc dau"

# rieng kiem tra bien: mot chuoi CO cap ke nhau BANG NHAU phai cho ket qua
# KHAC voi mot chuoi giam nghiem ngat -- phan biet < voi <=
chuoi_bang_nhau = [3.0, 2.0, 2.0, 1.0]
giam_don_dieu_bang_nhau = all(chuoi_bang_nhau[i + 1] < chuoi_bang_nhau[i] for i in range(len(chuoi_bang_nhau) - 1))
assert giam_don_dieu_bang_nhau == False, "chuoi co cap ke nhau BANG NHAU khong duoc coi la giam don dieu NGHIEM NGAT"
giam_han_bang_nhau = chuoi_bang_nhau[-1] < chuoi_bang_nhau[0]
assert giam_han_bang_nhau == True, "so sanh dau/cuoi tren vi du bien phai la True (1.0 < 3.0)"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu ghi lại `loss` của MỖI bước vào `lich_su_loss` — gọi `mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5)`. Chỗ hai so sánh loss CUỐI CÙNG với loss ĐẦU TIÊN của lịch sử — `lich_su_loss[-1] < lich_su_loss[0]` (loss cuối phải NHỎ HƠN loss đầu để coi là "đã giảm").
- kind: strategy
  body: 'Chỗ đầu: `mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5)`. Chỗ hai: `lich_su_loss[-1] < lich_su_loss[0]`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `mot_buoc(mo_hinh, ids, dim, so_token, mask, 0.5)` và `lich_su_loss[-1] < lich_su_loss[0]`.'
:::

:::validate
- tier: run
  timeoutMs: 25000
- tier: static
  onFail: chỗ trống đầu phải GỌI THẬT mot_buoc(...) với đúng tham số (khong duoc chep san mot gia tri hay bo qua viec goi ham); chỗ trống hai phải là phép so sánh '<' giữa lich_su_loss[-1] và lich_su_loss[0] (khong duoc chep san True/False, khong duoc doi thanh '<=', khong duoc doi chi so 0 thanh chi so khac)
  requireAst:
  - kind: uses-call, target: mot_buoc, min: 1
  - kind: uses-operator, target: "<", min: 5
  - kind: uses-name, target: lich_su_loss, min: 3
  - kind: has-literal, target: "0", min: 7
  # SUA LAI (phat hien lo dot bien THAT qua tools/kiem_dot_bien.mjs, khong
  # phai doan tay): ban dau ghi "<"=1 va thieu han luat has-literal cho "0"
  # -- ca hai deu SAI vi khong tinh toi cac "<" va cac literal "0" co san
  # trong CAC HAM PHU TRO da dinh nghia lai o day (dem_cap_lien_ke, gop_cap,
  # huan_luyen_bpe -- vi du "if tan_suat < 2", "if i < len(danh_sach) - 1",
  # "i = 0" trong gop_cap, "0.0" trong xay_mat_na_nhan_qua). Da thu THAT
  # bang kiemAst that tren code day du cua solution (ca lop Tensor + moi ham
  # phu tro + harness): tong so "<" la 5 (bon lan trong cac ham BPE/mask co
  # san, MOT lan trong blank2 "lich_su_loss[-1] < lich_su_loss[0]") --
  # nguong dung la min=5, khong phai 1. Tong so literal "0" (has-literal) la
  # 7 (rai rac trong gop_cap/xay_mat_na_nhan_qua/harness, CONG mot lan trong
  # chi so "lich_su_loss[0]" cua blank2) -- nguong dung la min=7.
  #
  # Cheat "lich_su_loss.append(0.0)" (chep hang so, bo qua goi ham THAT) lam
  # "mot_buoc" tut xuong 0 -- bi chan RIENG, VA da tu kiem chung bang Python
  # that: lich_su_loss se la [0.0]*12, lam assert lich_su_lam_tron THAT BAI
  # ngay.
  # Cheat "giam_han = True" (chep san, bo qua so sanh THAT) lam "<" tut
  # xuong 4 (duoi nguong 5) VA "lich_su_loss" tut xuong 1 -- bi chan BOI CA
  # HAI luat.
  # Cheat "lich_su_loss[-1] <= lich_su_loss[0]" (doi '<' thanh '<=') KHONG
  # doi "lich_su_loss"/"mot_buoc" (chi doi toan tu) -- da tu kiem chung bang
  # kiemAst THAT: lam tong "<" tut tu 5 xuong 4, duoi nguong moi min=5, bi
  # chan RIENG boi luat "<". Day chinh la lo dot bien q8.3d tu phat hien (da
  # tu kiem chung: tren du lieu THAT cua bai nay, doi '<' thanh '<=' KHONG
  # doi ket qua boolean vi khong co cap lien ke nao bang nhau trong day loss
  # 12 buoc -- CHI static rieng moi bat duoc, dung bien the "chuoi so sanh
  # khong co cap bang nhau nhung < va <= van dong thuan" cua lo dot bien
  # bien).
  # Cheat "lich_su_loss[-1] < lich_su_loss[1]" (doi chi so 0 thanh 1 trong
  # blank2) KHONG doi "<"/"mot_buoc"/"lich_su_loss" -- da tu kiem chung bang
  # kiemAst THAT: lam has-literal "0" tut tu 7 xuong 6, duoi nguong moi
  # min=7, bi chan RIENG boi luat has-literal. Cheat nay nguy hiem vi day
  # loss GIAM DON DIEU nen so sanh voi chi so 0 hay chi so 1 deu cho CUNG
  # ket qua True -- chi static moi phan biet duoc, khong output/tests nao
  # lam duoc vi ca hai deu cho giam_han=True giong het.
- tier: tests
  timeoutMs: 25000
- tier: output
  match: regex
  expect: "^\\[3\\.351352, 2\\.989879, 2\\.739706, 2\\.53488, 2\\.361116, 2\\.210683, 2\\.077004, 1\\.954192, 1\\.838194, 1\\.727511, 1\\.62595, 1\\.562128\\]\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`12` bước, loss từ `3,351352` xuống `1,562128` — huấn luyện THẬT SỰ có hiệu quả, đo bằng số thật, trên văn bản thật. Bài sau: dùng mô hình vừa huấn luyện này để SINH ra văn bản mới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mô hình vừa huấn luyện `12` bước biết dự đoán token TIẾP THEO — với MỘT chuỗi đầu vào, nó cho ra một phân phối xác suất trên `vocab` token ở MỖI vị trí. Nhưng "biết dự đoán" và "sinh ra một đoạn văn bản MỚI" là hai việc khác nhau: sinh văn bản cần LẶP LẠI việc dự đoán nhiều lần, mỗi lần nối thêm MỘT token mới vào chuỗi. Ở mỗi lần lặp, nên lấy token nào từ phân phối xác suất vừa dự đoán — token có xác suất CAO NHẤT, hay một lựa chọn khác?
::::

::::checkpoint{mastery=0.85}
::::
