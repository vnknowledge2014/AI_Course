---
id: tri-tue-nhan-tao.boss-transformer-tu-so-0.gioi-han-that-cua-micro-transformer
title: "Giới hạn thật của micro-transformer: tốt hơn ngẫu nhiên trên dữ liệu ĐÃ THẤY, tệ hơn trên dữ liệu MỚI"
summary: "Dung LAI dung mo hinh vua huan luyen o bai truoc (dim=4, 10 buoc, cua so 20 token dau cua corpus 5040 ky tu): loss tren du lieu DA HUAN LUYEN la 2,554882 -- THAP HON muc doan ngau nhien hoan hao log(30)=3,401197 (vocab_size=30). Nhung do loss TREN MOT CUA SO HOAN TOAN MOI (chua tung thay, trich tu vi tri 100 cua CHINH corpus do) duoc 3,495173 -- CAO HON muc ngau nhien. Ba so xep thang hang: 2,554882 < 3,401197 < 3,495173 -- mo hinh tot hon ngau nhien tren du lieu DA THAY, te hon ngau nhien tren du lieu CHUA THAY. Day la bang chung THAT cua qua khop (overfitting) tren mot mo hinh sieu nho, doi chieu bias-variance da hoc o T8.1/T8.2, KHONG phai suy luan."
locale: vi
track: tri-tue-nhan-tao
module: boss-transformer-tu-so-0
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.gioi-han-that-cua-micro-transformer]
requires: [ai.huan-luyen-duoi-hai-giay]
concepts: [ai.gioi-han-that-cua-micro-transformer]
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
Bài trước đo xong tốc độ: `5KB`, dưới `2` giây. Nhưng nhanh không có nghĩa là GIỎI. Trước khi bước vào BOSS đóng track, phải đối diện thẳng với câu hỏi khó chịu nhất: mô hình `dim = 4` này, huấn luyện `10` bước trên `20` token, thực sự "học" được gì — hay chỉ đang HỌC THUỘC LÒNG đúng `20` token đó?
::::

::::explain{#gioi_han_micro_transformer}
Một mô hình ngôn ngữ ĐOÁN NGẪU NHIÊN HOÀN HẢO (không học gì cả, cho xác suất đều nhau cho mọi token trong vocab) có cross-entropy loss đúng bằng `log(vocab_size)` — với `vocab_size = 30` (bài trước), con số đó là `log(30) ≈ 3,401197`. Đây là một MỐC THAM CHIẾU: bất kỳ mô hình nào có loss THẤP HƠN mốc này đang làm TỐT HƠN đoán ngẫu nhiên; loss CAO HƠN mốc này đang làm TỆ HƠN đoán ngẫu nhiên (một dấu hiệu bất thường, đáng ngờ).

Bài `huan-luyen-duoi-hai-giay` đã đo: sau `10` bước huấn luyện, loss trên CHÍNH cửa sổ `20` token đã huấn luyện là `2,554882` — THẤP HƠN mốc `3,401197`. Nhưng đây là loss trên dữ liệu mô hình ĐÃ THẤY và ĐÃ ĐƯỢC ĐIỀU CHỈNH THAM SỐ để khớp — không có gì đảm bảo mô hình học được QUY LUẬT CHUNG của ngôn ngữ, thay vì chỉ HỌC THUỘC LÒNG chính `20` token cụ thể đó.

Phép kiểm THẬT: đo loss của CÙNG mô hình đã huấn luyện đó, nhưng trên một cửa sổ `20` token KHÁC — trích từ một vị trí KHÁC trong CHÍNH corpus đó (văn bản THẬT, cùng chủ đề, cùng ngôn ngữ — không phải dữ liệu ngoài phân phối) — mà mô hình CHƯA TỪNG thấy trong `10` bước huấn luyện vừa rồi. Nếu mô hình học được QUY LUẬT CHUNG, loss trên cửa sổ mới này vẫn nên tốt hơn (hoặc ít nhất gần bằng) đoán ngẫu nhiên. Nếu mô hình chỉ HỌC THUỘC LÒNG, loss trên cửa sổ mới có thể TỆ HƠN cả đoán ngẫu nhiên — vì tham số đã bị "kéo lệch" để khớp CHÍNH XÁC với `20` token cũ, tại chính vị trí đó.
::::

::::example{#do_qua_khop_that}
Huấn luyện CHÍNH XÁC như bài trước (corpus `5040` ký tự, BPE vocab `30`, cửa sổ `20` token đầu, `10` bước SGD), rồi đo loss trên một cửa sổ MỚI — bắt đầu từ vị trí `100` của CHÍNH chuỗi đã mã hoá đó (chưa từng xuất hiện trong `20` token đầu dùng để huấn luyện):

```python title=readonly
import math
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


def tinh_loss(mo_hinh, ids, dim, so_token, mask):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    logits = X2.matmul(W_out)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    return float(loss.data)


doan_van = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
"he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
"cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
"mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
"chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
"nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
"thieu ham mat mat qua gradient descent.")
corpus = doan_van * 9

ds_final, merges, vocab = huan_luyen_bpe(corpus, 30)
token_sang_id = xay_token_sang_id(vocab)
ids_full = [token_sang_id[t] for t in ds_final]

dim, hidden, so_token = 4, 6, 20
vocab_size = len(vocab)
ids_huan_luyen = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)
mo_hinh = khoi_tao(8, vocab_size, dim, hidden)

lich_su_loss = [mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5) for _ in range(10)]

vi_tri_giu_lai = 100
ids_giu_lai = ids_full[vi_tri_giu_lai:vi_tri_giu_lai + so_token]
loss_giu_lai = tinh_loss(mo_hinh, ids_giu_lai, dim, so_token, mask)

loss_ngau_nhien = math.log(vocab_size)
loss_huan_luyen_cuoi = lich_su_loss[-1]

print(vocab_size, round(loss_ngau_nhien, 6))
print(round(loss_huan_luyen_cuoi, 6), round(loss_giu_lai, 6))
print(loss_huan_luyen_cuoi < loss_ngau_nhien, loss_giu_lai > loss_ngau_nhien)
```

```text title=readonly
30 3.401197
2.554882 3.495173
True True
```

Ba con số, xếp thẳng hàng: `2,554882 < 3,401197 < 3,495173`.

> **Trên dữ liệu ĐÃ huấn luyện**: loss `2,554882` — THẤP HƠN mốc ngẫu nhiên `3,401197`. Mô hình "giỏi hơn đoán mò" — nhưng CHỈ trên chính `20` token nó đã học thuộc.
>
> **Trên dữ liệu MỚI** (cùng corpus, cùng ngôn ngữ, khác vị trí): loss `3,495173` — CAO HƠN mốc ngẫu nhiên. Mô hình TỆ HƠN một bộ đoán ngẫu nhiên hoàn hảo khi gặp văn bản chưa từng thấy.

Đây không phải một trục trặc — đây là bằng chứng THẬT của quá khớp (overfitting): với `dim = 4` và chỉ `10` bước huấn luyện trên một cửa sổ `20` token, mô hình có ĐỦ tham số để "ghi nhớ" gần như chính xác `20` token đó, nhưng những gì nó ghi nhớ KHÔNG khái quát hoá được sang văn bản mới, dù văn bản mới đó THẬT SỰ liên quan (cùng chủ đề, cùng ngôn ngữ, thậm chí cùng corpus gốc).
::::

::::predict{#doan_neu_them_du_lieu commitOnce}
Loss trên dữ liệu đã huấn luyện: `2,554882` (dưới mốc ngẫu nhiên). Loss trên dữ liệu mới: `3,495173` (trên mốc ngẫu nhiên).

**Trước khi đọc bài BOSS**, bạn đoán: nếu huấn luyện mô hình này trên NHIỀU cửa sổ khác nhau (thay vì chỉ MỘT cửa sổ `20` token cố định), luân phiên qua nhiều đoạn của corpus `5040` ký tự, loss trên dữ liệu MỚI (chưa từng huấn luyện) có khả năng cải thiện (giảm gần về mức ngẫu nhiên hoặc thấp hơn) hay không?

:::opt{correct}
Có khả năng — vấn đề ở đây không phải kích thước `dim = 4` là quá nhỏ để học BẤT KỲ điều gì tổng quát, mà là mô hình chỉ được cho thấy MỘT cửa sổ `20` token duy nhất suốt `10` bước; huấn luyện trên NHIỀU cửa sổ đa dạng hơn (dù vẫn cùng kích thước mô hình nhỏ) sẽ buộc tham số phải khớp với NHIỀU mẫu khác nhau cùng lúc, giảm khả năng "học thuộc lòng" một cửa sổ đơn lẻ
:::

:::opt
Không — `dim = 4` là quá nhỏ để BAO GIỜ khái quát hoá được, bất kể huấn luyện bao nhiêu dữ liệu đa dạng
::why
Gần đúng ở việc nhận ra `dim = 4` là một giới hạn dung lượng THẬT (mô hình sản xuất dùng hàng trăm/hàng nghìn chiều) — quan sát đó không sai về giới hạn TUYỆT ĐỐI của mô hình này so với GPT thật.

Chỗ lệch: "quá khớp" và "quá nhỏ để khái quát hoá bất kỳ điều gì" là hai vấn đề KHÁC NHAU. Vấn đề đo được ở đây là quá khớp trên MỘT cửa sổ duy nhất (dữ liệu quá ÍT so với số tham số) — bài `data-leakage-chuan-hoa-truoc` và các bài bias-variance ở T8.1 đã chỉ ra: tăng ĐA DẠNG dữ liệu huấn luyện (không nhất thiết tăng KÍCH THƯỚC mô hình) là một cách hiệu quả để giảm quá khớp. `dim = 4` vẫn có GIỚI HẠN dung lượng riêng nó (không thể học được cấu trúc ngôn ngữ phức tạp như GPT thật, bài phần dưới sẽ đo tiếp), nhưng đó là giới hạn KHÁC với vấn đề quá khớp trên một cửa sổ đơn lẻ.
::
:::

:::opt
Không xác định được nếu không thử — quá khớp là hiện tượng khó đoán trước
::why
Gần đúng ở tinh thần đo thật trước khi kết luận — nguyên tắc xuyên suốt track này.

Chỗ lệch: cơ chế gây ra quá khớp ở đây (huấn luyện lặp lại NHIỀU bước trên CÙNG một cửa sổ duy nhất, không có dữ liệu nào khác để "cân bằng" lại) đã được hiểu rõ từ T8.1 (bias-variance) — đây là nguyên nhân KINH ĐIỂN của overfitting, không phải một bí ẩn riêng của Transformer. Tăng đa dạng dữ liệu huấn luyện là hướng giải quyết có thể suy luận trước từ nguyên lý đã học, dù mức độ cải thiện cụ thể vẫn cần đo thật.
::
:::
::::

::::code{#viet_gioi_han_micro_transformer}
Hoàn thiện ba chỗ trống: mốc loss ngẫu nhiên hoàn hảo, cửa sổ dữ liệu MỚI để kiểm tra, và kết luận "mô hình quá khớp".

```python title=starter
import math
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


def tinh_loss(mo_hinh, ids, dim, so_token, mask):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    logits = X2.matmul(W_out)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    return float(loss.data)


doan_van = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
"he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
"cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
"mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
"chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
"nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
"thieu ham mat mat qua gradient descent.")
corpus = doan_van * 9

ds_final, merges, vocab = huan_luyen_bpe(corpus, 30)
token_sang_id = xay_token_sang_id(vocab)
ids_full = [token_sang_id[t] for t in ds_final]

dim, hidden, so_token = 4, 6, 20
vocab_size = len(vocab)
ids_huan_luyen = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)
mo_hinh = khoi_tao(8, vocab_size, dim, hidden)

lich_su_loss = [mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5) for _ in range(10)]

loss_ngau_nhien = ___                       # math.log(vocab_size)

vi_tri_giu_lai = 100
ids_giu_lai = ___                           # ids_full[vi_tri_giu_lai:vi_tri_giu_lai + so_token]
loss_giu_lai = tinh_loss(mo_hinh, ids_giu_lai, dim, so_token, mask)

loss_huan_luyen_cuoi = lich_su_loss[-1]
qua_khop = ___                              # loss_giu_lai > loss_ngau_nhien

print(vocab_size, round(loss_ngau_nhien, 6))
print(round(loss_huan_luyen_cuoi, 6), round(loss_giu_lai, 6))
print(qua_khop)
```

```python title=solution
import math
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


def tinh_loss(mo_hinh, ids, dim, so_token, mask):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    logits = X2.matmul(W_out)
    loss = mat_mat_du_doan_tiep_theo(logits, ids)
    return float(loss.data)


doan_van = ("tri tue nhan tao la mot linh vuc cua khoa hoc may tinh nghien cuu cach xay dung "
"he thong co the hoc va suy luan tu du lieu. mang no ron nhan tao mo phong theo cau truc "
"cua nao bo con nguoi, gom nhieu lop neuron ket noi voi nhau. transformer la mot kien truc "
"mang no ron dung co che attention de xu ly du lieu tuan tu nhu van ban, cho phep mo hinh "
"chu y toi moi vi tri trong chuoi cung luc. huan luyen mot mo hinh ngon ngu lon can rat "
"nhieu du lieu van ban va rat nhieu buoc toi uu hoa, nhung nguyen ly co ban van la giam "
"thieu ham mat mat qua gradient descent.")
corpus = doan_van * 9

ds_final, merges, vocab = huan_luyen_bpe(corpus, 30)
token_sang_id = xay_token_sang_id(vocab)
ids_full = [token_sang_id[t] for t in ds_final]

dim, hidden, so_token = 4, 6, 20
vocab_size = len(vocab)
ids_huan_luyen = ids_full[:so_token]
mask = xay_mat_na_nhan_qua(so_token)
mo_hinh = khoi_tao(8, vocab_size, dim, hidden)

lich_su_loss = [mot_buoc(mo_hinh, ids_huan_luyen, dim, so_token, mask, 0.5) for _ in range(10)]

loss_ngau_nhien = math.log(vocab_size)

vi_tri_giu_lai = 100
ids_giu_lai = ids_full[vi_tri_giu_lai:vi_tri_giu_lai + so_token]
loss_giu_lai = tinh_loss(mo_hinh, ids_giu_lai, dim, so_token, mask)

loss_huan_luyen_cuoi = lich_su_loss[-1]
qua_khop = loss_giu_lai > loss_ngau_nhien

print(vocab_size, round(loss_ngau_nhien, 6))
print(round(loss_huan_luyen_cuoi, 6), round(loss_giu_lai, 6))
print(qua_khop)
```

```python title=test
assert vocab_size == 30, f"vocab_size sai -- dang ra {vocab_size}"
assert round(loss_ngau_nhien, 6) == 3.401197, f"loss_ngau_nhien sai -- dang ra {round(loss_ngau_nhien, 6)}"
assert round(loss_huan_luyen_cuoi, 6) == 2.554882, f"loss_huan_luyen_cuoi sai -- dang ra {round(loss_huan_luyen_cuoi, 6)}"
assert round(loss_giu_lai, 6) == 3.495173, f"loss_giu_lai sai -- dang ra {round(loss_giu_lai, 6)}"
assert qua_khop == True, "qua_khop phai la True -- loss tren du lieu MOI phai cao hon muc ngau nhien"

# rieng kiem tra ca hai chieu: tot hon ngau nhien tren du lieu DA huan
# luyen, VA te hon ngau nhien tren du lieu MOI -- day la bang chung KEP
assert loss_huan_luyen_cuoi < loss_ngau_nhien, "loss tren du lieu DA huan luyen phai THAP HON muc ngau nhien"
assert loss_giu_lai > loss_huan_luyen_cuoi, "loss tren du lieu MOI phai CAO HON loss tren du lieu da huan luyen"

# rieng kiem tra ids_giu_lai la mot cua so KHAC, khong trung voi ids_huan_luyen
assert ids_giu_lai != ids_huan_luyen, "ids_giu_lai phai la mot cua so KHAC voi ids_huan_luyen"
assert len(ids_giu_lai) == so_token, f"ids_giu_lai phai co dung so_token phan tu -- dang ra {len(ids_giu_lai)}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu tính mốc loss ngẫu nhiên hoàn hảo — `math.log(vocab_size)`. Chỗ hai lấy một cửa sổ `so_token` phần tử bắt đầu từ `vi_tri_giu_lai` của `ids_full` — `ids_full[vi_tri_giu_lai:vi_tri_giu_lai + so_token]`. Chỗ ba so sánh loss trên dữ liệu mới với mốc ngẫu nhiên — `loss_giu_lai > loss_ngau_nhien`.
- kind: strategy
  body: 'Chỗ đầu: `math.log(vocab_size)`. Chỗ hai: `ids_full[vi_tri_giu_lai:vi_tri_giu_lai + so_token]`. Chỗ ba: `loss_giu_lai > loss_ngau_nhien`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `math.log(vocab_size)`, `ids_full[vi_tri_giu_lai:vi_tri_giu_lai + so_token]`, và `loss_giu_lai > loss_ngau_nhien`.'
:::

:::validate
- tier: run
  timeoutMs: 20000
- tier: static
  onFail: loss_ngau_nhien phai goi THAT math.log(vocab_size); ids_giu_lai phai la phep cat lat THAT tren ids_full dung vi_tri_giu_lai va so_token (khong duoc chep san hay dung chi so khac); qua_khop phai la phep so sanh '>' THAT giua loss_giu_lai va loss_ngau_nhien (khong duoc chep san True, khong doi thanh '>=')
  requireAst:
  - kind: uses-call, target: log, min: 2
  - kind: uses-name, target: ids_full, min: 2
  - kind: uses-name, target: vi_tri_giu_lai, min: 2
  - kind: uses-operator, target: ">", min: 2
  - kind: uses-name, target: loss_ngau_nhien, min: 2
  - kind: uses-name, target: loss_giu_lai, min: 2
  # Da thu THAT bang kiemAst that (goi truc tiep tren code trich tu solution
  # da bien dich, khong doan tay).
  # log=2: 1 lan CO SAN trong mat_mat_du_doan_tiep_theo (np.log(p[...])), 1
  # lan trong blank1 (math.log(vocab_size)) -- ca hai deu khop target "log"
  # (kiemAst dem theo TEN ham, khong phan biet module). Dien bua blank1
  # thanh "loss_ngau_nhien = 3.401197" (chep san) lam so nay tut xuong 1 --
  # duoi nguong min=2, bi chan.
  # ids_full=2: 1 lan CO SAN o dong "ids_huan_luyen = ids_full[:so_token]",
  # 1 lan trong blank2. Dien bua blank2 lam bien mat tham chieu ids_full
  # (vi du chep san mot danh sach) lam so nay tut xuong 1 -- duoi nguong
  # min=2, bi chan.
  # vi_tri_giu_lai=2: CA HAI lan doc deu nam TRONG blank2 (ca chi so dau va
  # chi so cuoi cua slice deu dung bien nay). Dien bua blank2 lam ca hai
  # lan doc nay bien mat (tut xuong 0) -- duoi nguong min=2, bi chan RIENG,
  # manh hon ca luat ids_full.
  # ">"=2: 1 lan CO SAN trong relu()._backward ("self.data > 0"), 1 lan
  # trong blank3. Dien bua blank3 thanh "True" lam ">" tut xuong 1 -- duoi
  # nguong min=2, bi chan.
  # loss_ngau_nhien=2, loss_giu_lai=2: moi ten doc lai trong blank3 VA
  # trong dong print rieng. Dien bua blank3 thanh hang so lam ca hai ten
  # nay tut xuong 1 -- duoi nguong min=2, bi chan BOI CA HAI luat.
  # Cheat doi '>' thanh '>=' o blank3 (bien the bien): tren du lieu THAT
  # cua bai nay (loss_giu_lai=3,495173, loss_ngau_nhien=3,401197, khong
  # bao gio bang nhau), '>' va '>=' cho CUNG ket qua True -- CHI static
  # rieng moi bat duoc (">" tut tu 2 xuong 1), khong output/tests nao bat
  # duoc vi ca hai cho cung qua_khop=True.
- tier: tests
  timeoutMs: 20000
- tier: output
  match: regex
  expect: "^30 3\\.401197\\n2\\.554882 3\\.495173\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=curious pose=lean-in}
`2,554882 < 3,401197 < 3,495173` — mô hình tốt hơn đoán ngẫu nhiên trên dữ liệu ĐÃ THẤY, tệ hơn trên dữ liệu MỚI. Đây là giới hạn THẬT của một micro-transformer `dim = 4`, huấn luyện `10` bước trên một cửa sổ duy nhất — không phải suy luận, đo bằng số thật. Bài cuối: đóng cả T8.3 tại `40/40`.
::::

::::reflect{#nghi-lai}
Bốn bài đầu của quest này đã dựng một bức tranh đầy đủ: residual chống vanishing (bài `1`), attention giải phụ thuộc xa mà MLP không giải được có cấu trúc (bài `2`), pipeline chạy đúng ở quy mô lớn (bài `3`), chạy nhanh dưới `2` giây đúng yêu cầu MASTERPLAN (bài `4`). Bài này khép lại phần "đo giới hạn" bằng một sự thật không thoải mái nhưng cần thiết: một mô hình NHỎ, huấn luyện TRÊN MỘT cửa sổ đơn lẻ, có thể vừa "tốt hơn ngẫu nhiên" trên đúng dữ liệu nó thấy, vừa "tệ hơn ngẫu nhiên" trên dữ liệu mới — hai điều này không mâu thuẫn, mà là hai mặt của CÙNG MỘT hiện tượng quá khớp, đối chiếu trực tiếp với bias-variance đã học ở T8.1.

Đây KHÔNG phải một lỗi của Transformer hay của cách ráp pipeline — nó là hệ quả tất yếu của việc nhồi MỘT mô hình `dim = 4` học THẬT SÂU trên một lượng dữ liệu MINH HOẠ (không phải hàng tỉ token như GPT thật huấn luyện). Bài BOSS cuối cùng sẽ ráp lại toàn bộ pipeline một lần cuối, tổng kết các con số đã đo được xuyên suốt `5` bài, và đóng track T8.3 tại `40/40`.
::::

::::checkpoint{mastery=0.85}
::::
