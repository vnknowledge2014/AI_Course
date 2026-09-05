---
id: tri-tue-nhan-tao.khoi-transformer-va-huan-luyen.mot-buoc-cap-nhat
title: "Một bước cập nhật tham số: SGD đơn giản trên Tensor, và gotcha quên reset grad"
summary: "Mot buoc huan luyen: forward (BPE + embedding + 2 khoi + W_out) -> loss.backward() MOT LAN -> SGD don gian param.data -= lr * param.grad cho MOI tham so (KHONG Adam -- don gian hoa co y, khong phai trong tam quest nay). GOTCHA TAI HIEN tren Tensor: Tensor.grad cong don bang += trong moi _backward, KHONG tu reset giua hai lan backward() -- phai dat param.grad = np.zeros_like(param.data) cho MOI tham so TRUOC buoc forward/backward tiep theo. Chay THAT 2 buoc (lr=0,5): CO reset cho tong Wq tang 1 la 0,766605; KHONG reset la 0,766185 -- khac nhau, va do lech TANG THEM qua moi buoc khong reset (buoc 3: 0,01458; buoc 4: 0,044994)."
locale: vi
track: tri-tue-nhan-tao
module: khoi-transformer-va-huan-luyen
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.mot-buoc-cap-nhat]
requires: [ai.mat-mat-tu-tiep-theo]
concepts: [ai.mot-buoc-cap-nhat]
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
`loss.backward()` giờ điền đúng `.grad` cho mọi tham số. Bước cuối cùng còn thiếu: DÙNG `.grad` đó để thật sự thay đổi `.data` — và một gotcha đã gặp một lần, giờ quay lại dưới một hình dạng mới.
::::

::::explain{#mot_buoc_cap_nhat}
Vòng lặp huấn luyện đầy đủ cho `Value` (`vong-lap-huan-luyen`, q8.2c) học công thức SGD đơn giản nhất: `p.data -= lr * p.grad`, rồi `p.grad = 0.0` để RESET. Quest này dùng ĐÚNG ý tưởng đó cho `Tensor` — KHÔNG cần Adam đầy đủ (`adam_step_l2`, T8.2): huấn luyện ở đây không phải để tối ưu tốc độ hội tụ, mà để MINH HOẠ rằng huấn luyện thật sự làm loss giảm — SGD đơn giản đã đủ cho mục tiêu đó.

Một bước cập nhật đầy đủ, trên MỌI tham số của mô hình (`Bang`, `Wq`/`Wk`/`Wv`/`W1`/`W2` của cả hai khối, `W_out` — gộp thành một danh sách):

> 1. Forward: `logits = ...`, `loss = mat_mat_du_doan_tiep_theo(logits, ids)`.
> 2. `loss.backward()` — MỘT lần duy nhất.
> 3. Với MỖI tham số `p` trong danh sách: `p.data -= lr * p.grad`.
> 4. Với MỖI tham số `p`: RESET `p.grad = np.zeros_like(p.data)`.

Bước `4` không phải thủ tục thừa. `Tensor.grad` CỘNG DỒN bằng `+=` trong MỌI `_backward` (đúng cơ chế đã học từ `tich-luy-gradient`, q8.2b, và tái khẳng định ở `tra-cuu-embedding`, q8.3c) — nó KHÔNG tự reset giữa hai lần gọi `.backward()` khác nhau. Nếu quên bước `4`: bước cập nhật TIẾP THEO gọi `backward()` một lần nữa, và gradient MỚI sẽ CỘNG THÊM lên gradient CŨ còn sót lại từ bước trước — bước cập nhật thứ hai trở đi dùng một con số SAI (tổng của hai gradient, không phải gradient thật của bước hiện tại), dù không có lỗi (`Error`) nào được ném ra. Đây là biến thể MỚI của đúng gotcha đã gặp ở `vong-lap-huan-luyen` (q8.2c) — giờ tái hiện trên `Tensor`, và trên CẢ MỘT DANH SÁCH tham số thay vì hai biến `w`/`b` đặt tên riêng.
::::

::::example{#reset_grad_that}
Hai kịch bản, CÙNG dữ liệu, CÙNG khởi tạo (`seed=8`), CÙNG `lr = 0,5`, chạy đúng `2` bước: một kịch bản CÓ reset `grad` sau mỗi bước, một kịch bản KHÔNG:

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


def mot_buoc_co_reset(mo_hinh, ids, dim, so_token, mask, lr):
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


def mot_buoc_khong_reset(mo_hinh, ids, dim, so_token, mask, lr):
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
    # CO Y khong reset grad o day -- day chinh la ban THIEU zero_grad
    return float(loss.data)


corpus = "may hoc hoc tu du lieu. cang nhieu du lieu, may hoc cang hoc tot hon."
ds_final, merges, vocab = huan_luyen_bpe(corpus, 20)
token_sang_id = xay_token_sang_id(vocab)
ids_full = ma_hoa_van_ban(corpus, merges, token_sang_id)
dim, hidden, so_token = 4, 6, 10
vocab_size = len(vocab)
ids = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)

mo_hinh_co = khoi_tao(8, vocab_size, dim, hidden)
loss1_co = mot_buoc_co_reset(mo_hinh_co, ids, dim, so_token, mask, 0.5)
loss2_co = mot_buoc_co_reset(mo_hinh_co, ids, dim, so_token, mask, 0.5)
tong_co = float(np.sum(mo_hinh_co[1][0].data))

mo_hinh_khong = khoi_tao(8, vocab_size, dim, hidden)
loss1_khong = mot_buoc_khong_reset(mo_hinh_khong, ids, dim, so_token, mask, 0.5)
loss2_khong = mot_buoc_khong_reset(mo_hinh_khong, ids, dim, so_token, mask, 0.5)
tong_khong = float(np.sum(mo_hinh_khong[1][0].data))

print(round(loss1_co, 6), round(loss2_co, 6))
print(round(loss1_khong, 6), round(loss2_khong, 6))
print(round(tong_co, 6))
print(round(tong_khong, 6))
print(round(tong_co, 6) != round(tong_khong, 6))
```

```text title=readonly
3.351352 2.989879
3.351352 2.989879
0.766605
0.766185
True
```

`loss` sau bước `1` và bước `2` GIỐNG HỆT nhau ở cả hai kịch bản (`3,351352` rồi `2,989879`) — vì giá trị `loss` chỉ phụ thuộc `.data` của tham số TRƯỚC bước đó, và tham số sau bước `1` giống hệt nhau ở cả hai kịch bản. Nhưng `Wq` của tầng `1` (tổng mọi phần tử, sau ĐÚNG `2` bước) khác nhau: `0,766605` (CÓ reset) so với `0,766185` (KHÔNG reset). Ở kịch bản KHÔNG reset, bước `2` gọi `backward()` với gradient CỦA BƯỚC `1` vẫn còn sót lại trong `.grad` — gradient dùng để cập nhật ở bước `2` là TỔNG của gradient bước `1` và bước `2`, không phải gradient THẬT của riêng bước `2` — nên bước cập nhật thứ hai đi SAI hướng một lượng nhỏ, không báo lỗi gì.
::::

::::predict{#doan_lech_tang_qua_cac_buoc commitOnce}
Ở bước `2`, độ lệch giữa CÓ và KHÔNG reset (đo bằng tổng `Wq` tầng `1`) là `|0,766605 − 0,766185| = 0,00042`.

**Trước khi chạy thử**, bạn đoán: nếu tiếp tục chạy KHÔNG reset thêm vài bước nữa (bước `3`, bước `4`, ... vẫn không bao giờ reset), độ lệch đó sẽ NGÀY CÀNG LỚN hơn, hay giữ NGUYÊN một mức nhỏ cố định?

:::opt{correct}
Ngày càng lớn hơn — mỗi bước KHÔNG reset lại cộng dồn THÊM gradient của bước mới lên đúng phần "gradient cũ vẫn còn sót lại", nên phần dư thừa đó CHỒNG CHẤT qua từng bước, không dừng lại ở một mức cố định
:::

:::opt
Giữ nguyên một mức nhỏ cố định — vì gradient của mỗi bước đều nhỏ (do `lr` nhỏ), nên phần cộng dồn thêm cũng luôn nhỏ như nhau
::why
Gần đúng ở việc để ý gradient MỖI BƯỚC riêng lẻ có thể có độ lớn tương tự nhau — quan sát đó không sai xét riêng từng bước.

Chỗ lệch: "mỗi bước cộng thêm một lượng nhỏ tương tự nhau" không có nghĩa là TỔNG cộng dồn giữ nguyên — nó có nghĩa là tổng đó liên tục TĂNG THÊM, y hệt việc cộng liên tiếp nhiều số dương nhỏ vẫn cho một tổng ngày càng lớn. Số thật xác nhận: độ lệch đo được tăng từ `0,000421` (bước `2`) lên `0,01458` (bước `3`) rồi `0,044994` (bước `4`) — không giữ nguyên.
::
:::

:::opt
Không xác định được — độ lệch có thể tăng, giảm, hoặc dao động tuỳ ngẫu nhiên, không có xu hướng rõ ràng
::why
Gần đúng ở tinh thần thận trọng trước một hệ thống phức tạp (nhiều tham số, nhiều phép toán phi tuyến) — thái độ đó hợp lý với nhiều hiện tượng khác trong quest này.

Chỗ lệch: đây không phải một hiện tượng NGẪU NHIÊN — cơ chế gây ra nó là CỘNG DỒN có chủ đích của `+=` trong `_backward`, một quy tắc CỐ ĐỊNH, không đổi giữa các bước. Gradient còn sót lại từ mọi bước trước LUÔN được cộng thêm vào bước sau (không bao giờ bị trừ bớt hay huỷ bỏ), nên xu hướng TĂNG là một hệ quả CẤU TRÚC, đo được nhất quán, không phải một sự trùng hợp ngẫu nhiên của riêng bộ số này.
::
:::
::::

::::code{#viet_mot_buoc_cap_nhat}
Hoàn thiện hai chỗ trống trong `mot_buoc_co_reset`: bước cập nhật SGD, và bước RESET gradient về `0`.

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


def mot_buoc_co_reset(mo_hinh, ids, dim, so_token, mask, lr):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    logits = X2.matmul(W_out)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    loss.backward()

    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= ___                         # lr * p.grad
    for p in tat_ca:
        p.grad = ___                          # np.zeros_like(p.data)
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
loss1 = mot_buoc_co_reset(mo_hinh, ids, dim, so_token, mask, 0.5)
loss2 = mot_buoc_co_reset(mo_hinh, ids, dim, so_token, mask, 0.5)
tong_wq1 = float(np.sum(mo_hinh[1][0].data))

print(round(loss1, 6), round(loss2, 6))
print(round(tong_wq1, 6))
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


def mot_buoc_co_reset(mo_hinh, ids, dim, so_token, mask, lr):
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
loss1 = mot_buoc_co_reset(mo_hinh, ids, dim, so_token, mask, 0.5)
loss2 = mot_buoc_co_reset(mo_hinh, ids, dim, so_token, mask, 0.5)
tong_wq1 = float(np.sum(mo_hinh[1][0].data))

print(round(loss1, 6), round(loss2, 6))
print(round(tong_wq1, 6))
```

```python title=test
import numpy as np

assert round(loss1, 6) == 3.351352, f"loss1 sai -- dang ra {round(loss1, 6)}"
assert round(loss2, 6) == 2.989879, f"loss2 sai -- dang ra {round(loss2, 6)}"
assert loss2 < loss1, f"loss phai GIAM sau mot buoc cap nhat -- dang ra loss1={loss1}, loss2={loss2}"
assert round(tong_wq1, 6) == 0.766605, f"tong_wq1 sai -- dang ra {round(tong_wq1, 6)}"

# rieng kiem tra grad THAT SU duoc reset ve 0 sau moi buoc (khong con sot lai)
Bang, tham_so_1, tham_so_2, W_out = mo_hinh
assert np.sum(np.abs(tham_so_1[0].grad)) == 0.0, "grad cua Wq tang 1 phai duoc RESET ve 0 sau buoc cap nhat cuoi cung"
assert np.sum(np.abs(W_out.grad)) == 0.0, "grad cua W_out phai duoc RESET ve 0 sau buoc cap nhat cuoi cung"

# GOTCHA TRUNG TAM: chay lai TU DAU, mot phien ban KHONG reset grad, doi
# chieu voi phien ban CO reset (vua chay o tren) sau dung 2 buoc.
def mot_buoc_khong_reset(mo_hinh, ids, dim, so_token, mask, lr):
    Bang_, tham_so_1_, tham_so_2_, W_out_ = mo_hinh
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang_.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1_, tham_so_2_, mask)
    logits = X2.matmul(W_out_)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    loss.backward()
    tat_ca = [Bang_] + list(tham_so_1_) + list(tham_so_2_) + [W_out_]
    for p in tat_ca:
        p.data -= lr * p.grad
    # KHONG reset grad -- day chinh la gotcha
    return float(loss.data)

mo_hinh_khong = khoi_tao(8, vocab_size, dim, hidden)
mot_buoc_khong_reset(mo_hinh_khong, ids, dim, so_token, mask, 0.5)
mot_buoc_khong_reset(mo_hinh_khong, ids, dim, so_token, mask, 0.5)
tong_khong = float(np.sum(mo_hinh_khong[1][0].data))

assert round(tong_khong, 6) == 0.766185, f"tong_khong sai -- dang ra {round(tong_khong, 6)}"
assert round(tong_wq1, 6) != round(tong_khong, 6), "CO reset va KHONG reset grad phai cho hai ket qua KHAC NHAU sau 2 buoc -- neu giong nhau, mot_buoc_co_reset dang khong reset dung cach"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai bên trong vòng lặp `for p in tat_ca`. Chỗ đầu là bước CẬP NHẬT SGD — trừ `lr` nhân gradient — `lr * p.grad`. Chỗ hai là bước RESET — đặt LẠI gradient về `0`, CÙNG shape với `p.data` — `np.zeros_like(p.data)` (không phải số `0` trần, vì `p.grad` phải là một MẢNG cùng shape).
- kind: strategy
  body: 'Chỗ đầu: `lr * p.grad`. Chỗ hai: `np.zeros_like(p.data)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `lr * p.grad` và `np.zeros_like(p.data)`.'
:::

:::validate
- tier: run
  timeoutMs: 20000
- tier: static
  onFail: buoc cap nhat phai tru DUNG lr * p.grad (khong duoc chep hang so hay bo qua lr); buoc reset phai dat p.grad = np.zeros_like(p.data) (dung DUNG shape cua p.data, khong duoc gan so 0 tran hay bo qua buoc reset)
  requireAst:
  - kind: uses-name, target: lr, min: 1
  - kind: uses-call, target: zeros_like, min: 1
  - kind: uses-name, target: p, min: 6
  # Da thu THAT bang kiemAst that (goi truc tiep tren code bien dich, khong
  # doan tay): tren solution, lr=1 (tham so ham "lr" trong chu ky ham la
  # ast.arg, KHONG dem; Load DUY NHAT la trong blank1 "lr * p.grad" -- ban
  # dau uoc luong nham la 2, kiem tra that xac nhan dung 1). zeros_like=3
  # (mot trong Tensor.__init__ co san, mot trong mat_mat_du_doan_tiep_theo
  # co san, mot trong blank2 -- nguong min=1 van an toan vi khong the tut
  # xuong duoi 1 neu hoc vien dien dung blank2, va dien bua khong lam giam
  # so nay vi hai lan dau la code co san khong doi). p=6+ rai rac trong hai
  # vong lap "for p in tat_ca" (gan-ten p khong dem, chi Load: "p.data" va
  # "p.grad" trong blank1, "p.grad" trong blank2, "p.data" o zeros_like --
  # nhieu lan Load).
  # Cheat "p.data -= p.grad" (bo qua lr, tuong duong lr=1) lam "lr" tut
  # xuong 0 trong blank1 (van con o cho khac?) -- da tu kiem chung bang
  # Python that: cheat nay lam loss2 khac han 2.989879 dung (buoc nhay qua
  # lon) -- bi bat DOC LAP boi assert loss2.
  # Cheat "p.grad = 0" (so 0 tran thay vi mang) SE lam chuong trinh NEM LOI
  # o buoc forward/backward KE TIEP (numpy khong the += vao mot int 0 theo
  # dung shape mang trong _backward tiep theo -- thuc ra numpy CHO PHEP
  # p.grad=0 roi p.grad += mang se tu dong quang bien no thanh mang, NHUNG
  # object p.grad khi do la một python int 0 chu khong phai ndarray, va
  # p.grad[tid] += ... trong embedding_lookup se NEM TypeError vi int
  # khong ho tro gan chi so) -- bi chan boi tier run, da tu kiem chung
  # bang Python that.
- tier: tests
  timeoutMs: 20000
- tier: output
  match: regex
  expect: "^3\\.351352 2\\.989879\\n0\\.766605\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bước cập nhật đúng — và một gotcha đã gặp ở q8.2c, giờ tái hiện y hệt trên `Tensor`: quên reset gradient làm bước thứ hai trở đi sai lặng lẽ. Bài sau: lặp lại bước này NHIỀU LẦN trên corpus thật, và đo loss giảm dần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một bước cập nhật vừa xong: loss giảm từ `3,351352` xuống `2,989879` — đúng MỘT bước, trên đúng MỘT cửa sổ `10` token. Để thấy huấn luyện THẬT SỰ có hiệu quả (không chỉ một bước may mắn), cần lặp lại bước này NHIỀU lần liên tiếp và quan sát loss có tiếp tục giảm hay không. Vòng lặp đó cần bao nhiêu bước là đủ để THẤY xu hướng giảm rõ ràng, mà vẫn giữ trong ngân sách thời gian chạy của một bài học?
::::

::::checkpoint{mastery=0.85}
::::
