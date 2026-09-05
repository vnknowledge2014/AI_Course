---
id: tri-tue-nhan-tao.boss-transformer-tu-so-0.mlp-that-bai-transformer-thanh-cong
title: "Bài toán phụ thuộc xa: MLP thất bại có cấu trúc, Transformer thành công"
summary: "Thiet ke mot bai toan phu thuoc XA: chuoi 6 token, token CUOI phai bang token DAU, giua la nhieu (khong doi). Mot MLP theo-vi-tri thuan (embedding+PE roi feedforward AP DOC LAP tung vi tri, khong attention) cho logit tai vi tri kiem tra GIONG HET nhau (np.array_equal=True) du dau vao A(bit=0) hay B(bit=1) khac nhau -- CHUNG MINH duoc bang toan hoc (khong phai do choi), khong the vuot qua accuracy 0,5 tren 2 vi du du huan luyen bao nhieu buoc. Transformer 2 tang THAT (attention nhin duoc token dau qua causal mask), sau 20 buoc SGD tren CA HAI vi du, dat accuracy 1,0 -- doan dung CA HAI, loss giam that (2,013202 -> 0,285035). Do accuracy/loss THAT, khong suy luan."
locale: vi
track: tri-tue-nhan-tao
module: boss-transformer-tu-so-0
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.mlp-that-bai-transformer-thanh-cong]
requires: [ai.vi-sao-attention-khong-vanishing]
concepts: [ai.mlp-that-bai-transformer-thanh-cong]
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
Bài trước đo xong CHUYỆN GRADIENT: residual khiến Transformer không vanishing như MLP sâu. Nhưng gradient lan tới được không có nghĩa mô hình HỌC được mọi thứ. Bài này hỏi câu khác hẳn: có một bài toán mà một MLP, dù gradient lan tốt, dù huấn luyện bao lâu, KHÔNG BAO GIỜ giải được — trong khi Transformer giải được — hay không?
::::

::::explain{#bai_toan_phu_thuoc_xa}
**Bài toán phụ thuộc xa**: một chuỗi `6` token — vị trí `0` là "bit" (`0` hoặc `1`), các vị trí `1..4` là token NHIỄU giống hệt nhau (không mang thông tin gì), vị trí `5` PHẢI BẰNG vị trí `0`. Hai ví dụ:

> `A = [0, 2, 2, 2, 2, 0]` (bit `= 0`, token nhiễu `= 2`)
>
> `B = [1, 2, 2, 2, 2, 1]` (bit `= 1`, token nhiễu `= 2`)

Để dự đoán ĐÚNG token ở vị trí `5`, mô hình phải "nhớ" thông tin từ vị trí `0` — cách xa `5` vị trí, xuyên qua `4` token nhiễu không liên quan.

**MLP theo-vị-trí thuần** (position-wise feedforward — CHÍNH LÀ mạng truyền thẳng đã xây ở `khoi-transformer-va-huan-luyen`, nhưng dùng MỘT MÌNH, không có attention đứng trước): `embedding + vị trí` rồi `matmul → relu → matmul` áp ĐỘC LẬP cho MỖI vị trí, không có bước nào trộn thông tin GIỮA các vị trí. Đầu ra tại vị trí `4` (dùng để dự đoán token ở vị trí `5`) chỉ phụ thuộc vào: token TẠI vị trí `4` (luôn là token nhiễu `2`, GIỐNG HỆT nhau ở cả `A` lẫn `B`) và vị trí `4` (một con số cố định, không đổi). Cả hai đầu vào này GIỐNG HỆT nhau giữa `A` và `B` — nên đầu ra tại vị trí `4` cũng phải GIỐNG HỆT nhau, BẤT KỂ trọng số của MLP là gì. Đây là một sự thật CẤU TRÚC (chứng minh được bằng đại số, không cần chạy thử) — không phải một quan sát thực nghiệm có thể đổi bằng cách huấn luyện lâu hơn.

**Transformer** (attention có causal mask, `khoi-transformer-va-huan-luyen`): tại vị trí `4`, self-attention có thể "nhìn" TRỰC TIẾP về vị trí `0` (mask chỉ chặn nhìn về TƯƠNG LAI, không chặn nhìn về QUÁ KHỨ) — trọng số attention (Query/Key) phụ thuộc vào NỘI DUNG token, nên attention CÓ THỂ học cách chú ý nhiều hơn vào vị trí `0` khi cần. Không có rào cản cấu trúc nào ngăn thông tin từ vị trí `0` "chảy" tới vị trí `4`.
::::

::::example{#mlp_giong_het_truoc_huan_luyen}
Trước khi huấn luyện bước nào — MLP theo-vị-trí (dim `4`, hidden `6`, seed `3`) — so hai logit tại vị trí kiểm tra (`4`) giữa `A` và `B`:

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

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
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


def khoi_tao_mlp(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    W1 = W((dim, hidden)); W2 = W((hidden, dim)); W_out = W((dim, vocab_size))
    return Bang, W1, W2, W_out


def forward_mlp(mo_hinh, ids, dim):
    Bang, W1, W2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    H = X.matmul(W1).relu()
    Y = H.matmul(W2)
    return Y.matmul(W_out)


vocab_size, dim, hidden = 3, 4, 6
ids_A = [0, 2, 2, 2, 2, 0]
ids_B = [1, 2, 2, 2, 2, 1]
vi_tri_kiem_tra = 4

mo_hinh_mlp = khoi_tao_mlp(3, vocab_size, dim, hidden)
logits_mlp_A = forward_mlp(mo_hinh_mlp, ids_A, dim)
logits_mlp_B = forward_mlp(mo_hinh_mlp, ids_B, dim)

print(logits_mlp_A.data[vi_tri_kiem_tra])
print(logits_mlp_B.data[vi_tri_kiem_tra])
print(np.array_equal(logits_mlp_A.data[vi_tri_kiem_tra], logits_mlp_B.data[vi_tri_kiem_tra]))
```

```text title=readonly
[ 0.06495954 -0.07601054 -0.05940757]
[ 0.06495954 -0.07601054 -0.05940757]
True
```

Hai logit GIỐNG HỆT NHAU từng chữ số — dù `A` và `B` khác nhau ở CHÍNH bit cần nhớ (`0` với `1`). Đây KHÔNG phải trùng hợp của một seed cụ thể: MLP theo-vị-trí áp CÙNG một hàm lên MỖI vị trí một cách độc lập, nên với CÙNG token ở vị trí `4` (token nhiễu `2`, giống hệt ở cả `A` và `B`), đầu ra PHẢI giống hệt nhau, bất kể trọng số `Bang`, `W1`, `W2`, `W_out` là gì.
::::

::::example{#huan_luyen_20_buoc_ket_qua}
Huấn luyện CẢ HAI mô hình `20` bước (SGD, `lr = 0,5`, mất mát TỔNG trên cả `A` và `B` mỗi bước — reset gradient mỗi bước), rồi đo lại logit/accuracy tại vị trí `4`:

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


def khoi_tao_transformer(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


def khoi_tao_mlp(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    W1 = W((dim, hidden)); W2 = W((hidden, dim)); W_out = W((dim, vocab_size))
    return Bang, W1, W2, W_out


def forward_transformer(mo_hinh, ids, dim, mask):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    return X2.matmul(W_out)


def forward_mlp(mo_hinh, ids, dim):
    Bang, W1, W2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    H = X.matmul(W1).relu()
    Y = H.matmul(W2)
    return Y.matmul(W_out)


def mot_buoc_tf(mo_hinh, dim, mask, lr, ids_A, ids_B):
    logits_A = forward_transformer(mo_hinh, ids_A, dim, mask)
    logits_B = forward_transformer(mo_hinh, ids_B, dim, mask)
    loss = mat_mat_du_doan_tiep_theo(logits_A, ids_A) + mat_mat_du_doan_tiep_theo(logits_B, ids_B)
    loss.backward()
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


def mot_buoc_mlp(mo_hinh, dim, lr, ids_A, ids_B):
    logits_A = forward_mlp(mo_hinh, ids_A, dim)
    logits_B = forward_mlp(mo_hinh, ids_B, dim)
    loss = mat_mat_du_doan_tiep_theo(logits_A, ids_A) + mat_mat_du_doan_tiep_theo(logits_B, ids_B)
    loss.backward()
    tat_ca = list(mo_hinh)
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


vocab_size, dim, hidden = 3, 4, 6
ids_A = [0, 2, 2, 2, 2, 0]
ids_B = [1, 2, 2, 2, 2, 1]
vi_tri_kiem_tra = 4
mask6 = xay_mat_na_nhan_qua(6)

mo_hinh_mlp = khoi_tao_mlp(3, vocab_size, dim, hidden)
mo_hinh_tf = khoi_tao_transformer(7, vocab_size, dim, hidden)

lich_su_mlp = [mot_buoc_mlp(mo_hinh_mlp, dim, 0.5, ids_A, ids_B) for _ in range(20)]
lich_su_tf = [mot_buoc_tf(mo_hinh_tf, dim, mask6, 0.5, ids_A, ids_B) for _ in range(20)]

logits_mlp_A = forward_mlp(mo_hinh_mlp, ids_A, dim)
logits_mlp_B = forward_mlp(mo_hinh_mlp, ids_B, dim)
pred_mlp_A = int(np.argmax(logits_mlp_A.data[vi_tri_kiem_tra]))
pred_mlp_B = int(np.argmax(logits_mlp_B.data[vi_tri_kiem_tra]))
acc_mlp = ((pred_mlp_A == ids_A[vi_tri_kiem_tra + 1]) + (pred_mlp_B == ids_B[vi_tri_kiem_tra + 1])) / 2

logits_tf_A = forward_transformer(mo_hinh_tf, ids_A, dim, mask6)
logits_tf_B = forward_transformer(mo_hinh_tf, ids_B, dim, mask6)
pred_tf_A = int(np.argmax(logits_tf_A.data[vi_tri_kiem_tra]))
pred_tf_B = int(np.argmax(logits_tf_B.data[vi_tri_kiem_tra]))
acc_tf = ((pred_tf_A == ids_A[vi_tri_kiem_tra + 1]) + (pred_tf_B == ids_B[vi_tri_kiem_tra + 1])) / 2

print(np.array_equal(logits_mlp_A.data[vi_tri_kiem_tra], logits_mlp_B.data[vi_tri_kiem_tra]))
print(acc_mlp, acc_tf)
print(round(lich_su_mlp[0], 6), round(lich_su_mlp[-1], 6))
print(round(lich_su_tf[0], 6), round(lich_su_tf[-1], 6))
```

```text title=readonly
True
0.5 1.0
2.256497 0.453143
2.013202 0.285035
```

Bốn bằng chứng ĐỘC LẬP:

> **Loss giảm cho CẢ HAI mô hình** — `2,256497 → 0,453143` (MLP), `2,013202 → 0,285035` (Transformer). Cả hai đều huấn luyện THẬT, không phải đứng yên.
>
> **Logit của MLP vẫn GIỐNG HỆT nhau** giữa `A` và `B`, ngay cả SAU `20` bước huấn luyện — đúng như dự đoán cấu trúc: huấn luyện đổi trọng số, nhưng KHÔNG thể làm đầu vào tại vị trí `4` (giống hệt nhau) tạo ra hai đầu ra khác nhau.
>
> **`acc_mlp = 0,5`** — MLP đoán ĐÚNG chính xác một trong hai ví dụ (đoán cùng một lớp cho cả `A` lẫn `B`, trùng với nhãn của đúng MỘT ví dụ) — đây là TRẦN accuracy có thể đạt, không phải kết quả của huấn luyện chưa đủ.
>
> **`acc_tf = 1,0`** — Transformer đoán ĐÚNG CẢ HAI, sau khi attention học được cách "nhìn" về vị trí `0`.
::::

::::predict{#doan_huan_luyen_them_mlp commitOnce}
`acc_mlp = 0,5` sau `20` bước huấn luyện, logit của MLP tại vị trí `4` vẫn giống hệt nhau giữa `A` và `B`.

**Trước khi đọc lại phần giải thích**, bạn đoán: nếu huấn luyện MLP này THÊM `1000` bước nữa (không đổi kiến trúc, không đổi dữ liệu), `acc_mlp` có bao giờ vượt qua `0,5` hay không?

:::opt{correct}
Không bao giờ — đầu vào của MLP tại vị trí `4` (token nhiễu `2` cộng vị trí `4`) giống hệt nhau giữa `A` và `B` bất kể trọng số nào, nên đầu ra tại đó LUÔN giống hệt nhau; với `2` ví dụ có nhãn KHÁC NHAU, một đầu ra giống hệt nhau chỉ có thể khớp ĐÚNG một trong hai, không bao giờ khớp cả hai
:::

:::opt
Có, nếu học đủ lâu — huấn luyện đủ bước luôn giúp mọi mạng nơ-ron học được mọi hàm số, kể cả hàm này
::why
Gần đúng ở việc tin tưởng sức mạnh của huấn luyện lâu dài — điều đó ĐÚNG cho nhiều bài toán khi kiến trúc đủ biểu đạt.

Chỗ lệch: đây không phải vấn đề "học chưa đủ lâu" mà là vấn đề GIỚI HẠN BIỂU ĐẠT của kiến trúc — MLP theo-vị-trí về mặt TOÁN HỌC không có cách nào để đầu ra tại vị trí `4` phụ thuộc vào giá trị tại vị trí `0`, vì phép tính tại mỗi vị trí hoàn toàn ĐỘC LẬP với các vị trí khác. Không có số bước huấn luyện nào thay đổi được sự thật cấu trúc này — đây là một GIỚI HẠN của kiến trúc, không phải của quá trình tối ưu hoá.
::
:::

:::opt
Không xác định được nếu không huấn luyện thử — không có cách nào biết trước giới hạn của một mạng nơ-ron
::why
Gần đúng ở tinh thần đo thật trước khi kết luận — nguyên tắc xuyên suốt track này.

Chỗ lệch: ở đây KHÔNG cần chạy `1000` bước để biết trước — lập luận cấu trúc (đầu vào giống hệt nhau tại vị trí `4` giữa `A` và `B`, với BẤT KỲ trọng số nào) là một chứng minh TOÁN HỌC, đúng với MỌI giá trị trọng số có thể có, không chỉ với những gì đã quan sát được sau `20` bước cụ thể. Đây là trường hợp suy luận trước hoàn toàn có cơ sở, không cần đo thêm để biết HƯỚNG kết quả.
::
:::
::::

::::code{#viet_phu_thuoc_xa}
Hoàn thiện ba chỗ trống: kiểm tra logit của MLP tại vị trí kiểm tra có giống hệt nhau giữa `A` và `B` không, và tính accuracy của mỗi mô hình.

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


def khoi_tao_transformer(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


def khoi_tao_mlp(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    W1 = W((dim, hidden)); W2 = W((hidden, dim)); W_out = W((dim, vocab_size))
    return Bang, W1, W2, W_out


def forward_transformer(mo_hinh, ids, dim, mask):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    return X2.matmul(W_out)


def forward_mlp(mo_hinh, ids, dim):
    Bang, W1, W2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    H = X.matmul(W1).relu()
    Y = H.matmul(W2)
    return Y.matmul(W_out)


def mot_buoc_tf(mo_hinh, dim, mask, lr, ids_A, ids_B):
    logits_A = forward_transformer(mo_hinh, ids_A, dim, mask)
    logits_B = forward_transformer(mo_hinh, ids_B, dim, mask)
    loss = mat_mat_du_doan_tiep_theo(logits_A, ids_A) + mat_mat_du_doan_tiep_theo(logits_B, ids_B)
    loss.backward()
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


def mot_buoc_mlp(mo_hinh, dim, lr, ids_A, ids_B):
    logits_A = forward_mlp(mo_hinh, ids_A, dim)
    logits_B = forward_mlp(mo_hinh, ids_B, dim)
    loss = mat_mat_du_doan_tiep_theo(logits_A, ids_A) + mat_mat_du_doan_tiep_theo(logits_B, ids_B)
    loss.backward()
    tat_ca = list(mo_hinh)
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


vocab_size, dim, hidden = 3, 4, 6
ids_A = [0, 2, 2, 2, 2, 0]
ids_B = [1, 2, 2, 2, 2, 1]
vi_tri_kiem_tra = 4
mask6 = xay_mat_na_nhan_qua(6)

mo_hinh_mlp = khoi_tao_mlp(3, vocab_size, dim, hidden)
mo_hinh_tf = khoi_tao_transformer(7, vocab_size, dim, hidden)

lich_su_mlp = [mot_buoc_mlp(mo_hinh_mlp, dim, 0.5, ids_A, ids_B) for _ in range(20)]
lich_su_tf = [mot_buoc_tf(mo_hinh_tf, dim, mask6, 0.5, ids_A, ids_B) for _ in range(20)]

logits_mlp_A = forward_mlp(mo_hinh_mlp, ids_A, dim)
logits_mlp_B = forward_mlp(mo_hinh_mlp, ids_B, dim)
mlp_giong_het = ___                    # np.array_equal(logits_mlp_A.data[vi_tri_kiem_tra], logits_mlp_B.data[vi_tri_kiem_tra])
pred_mlp_A = int(np.argmax(logits_mlp_A.data[vi_tri_kiem_tra]))
pred_mlp_B = int(np.argmax(logits_mlp_B.data[vi_tri_kiem_tra]))
acc_mlp = ___                           # ((pred_mlp_A == ids_A[vi_tri_kiem_tra + 1]) + (pred_mlp_B == ids_B[vi_tri_kiem_tra + 1])) / 2

logits_tf_A = forward_transformer(mo_hinh_tf, ids_A, dim, mask6)
logits_tf_B = forward_transformer(mo_hinh_tf, ids_B, dim, mask6)
pred_tf_A = int(np.argmax(logits_tf_A.data[vi_tri_kiem_tra]))
pred_tf_B = int(np.argmax(logits_tf_B.data[vi_tri_kiem_tra]))
acc_tf = ___                            # ((pred_tf_A == ids_A[vi_tri_kiem_tra + 1]) + (pred_tf_B == ids_B[vi_tri_kiem_tra + 1])) / 2

print(mlp_giong_het)
print(acc_mlp, acc_tf)
print(round(lich_su_mlp[0], 6), round(lich_su_mlp[-1], 6))
print(round(lich_su_tf[0], 6), round(lich_su_tf[-1], 6))
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


def khoi_tao_transformer(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    tham_so_1 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    tham_so_2 = (W((dim, dim)), W((dim, dim)), W((dim, dim)), W((dim, hidden)), W((hidden, dim)))
    W_out = W((dim, vocab_size))
    return Bang, tham_so_1, tham_so_2, W_out


def khoi_tao_mlp(seed, vocab_size, dim, hidden):
    rng = np.random.default_rng(seed)
    def W(shape): return Tensor(rng.uniform(-0.5, 0.5, size=shape))
    Bang = W((vocab_size, dim))
    W1 = W((dim, hidden)); W2 = W((hidden, dim)); W_out = W((dim, vocab_size))
    return Bang, W1, W2, W_out


def forward_transformer(mo_hinh, ids, dim, mask):
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    X2 = xep_chong_2_tang(X, tham_so_1, tham_so_2, mask)
    return X2.matmul(W_out)


def forward_mlp(mo_hinh, ids, dim):
    Bang, W1, W2, W_out = mo_hinh
    so_token = len(ids)
    PE = Tensor(ma_hoa_vi_tri(so_token, dim))
    X = Bang.embedding_lookup(ids) + PE
    H = X.matmul(W1).relu()
    Y = H.matmul(W2)
    return Y.matmul(W_out)


def mot_buoc_tf(mo_hinh, dim, mask, lr, ids_A, ids_B):
    logits_A = forward_transformer(mo_hinh, ids_A, dim, mask)
    logits_B = forward_transformer(mo_hinh, ids_B, dim, mask)
    loss = mat_mat_du_doan_tiep_theo(logits_A, ids_A) + mat_mat_du_doan_tiep_theo(logits_B, ids_B)
    loss.backward()
    Bang, tham_so_1, tham_so_2, W_out = mo_hinh
    tat_ca = [Bang] + list(tham_so_1) + list(tham_so_2) + [W_out]
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


def mot_buoc_mlp(mo_hinh, dim, lr, ids_A, ids_B):
    logits_A = forward_mlp(mo_hinh, ids_A, dim)
    logits_B = forward_mlp(mo_hinh, ids_B, dim)
    loss = mat_mat_du_doan_tiep_theo(logits_A, ids_A) + mat_mat_du_doan_tiep_theo(logits_B, ids_B)
    loss.backward()
    tat_ca = list(mo_hinh)
    for p in tat_ca:
        p.data -= lr * p.grad
    for p in tat_ca:
        p.grad = np.zeros_like(p.data)
    return float(loss.data)


vocab_size, dim, hidden = 3, 4, 6
ids_A = [0, 2, 2, 2, 2, 0]
ids_B = [1, 2, 2, 2, 2, 1]
vi_tri_kiem_tra = 4
mask6 = xay_mat_na_nhan_qua(6)

mo_hinh_mlp = khoi_tao_mlp(3, vocab_size, dim, hidden)
mo_hinh_tf = khoi_tao_transformer(7, vocab_size, dim, hidden)

lich_su_mlp = [mot_buoc_mlp(mo_hinh_mlp, dim, 0.5, ids_A, ids_B) for _ in range(20)]
lich_su_tf = [mot_buoc_tf(mo_hinh_tf, dim, mask6, 0.5, ids_A, ids_B) for _ in range(20)]

logits_mlp_A = forward_mlp(mo_hinh_mlp, ids_A, dim)
logits_mlp_B = forward_mlp(mo_hinh_mlp, ids_B, dim)
mlp_giong_het = np.array_equal(logits_mlp_A.data[vi_tri_kiem_tra], logits_mlp_B.data[vi_tri_kiem_tra])
pred_mlp_A = int(np.argmax(logits_mlp_A.data[vi_tri_kiem_tra]))
pred_mlp_B = int(np.argmax(logits_mlp_B.data[vi_tri_kiem_tra]))
acc_mlp = ((pred_mlp_A == ids_A[vi_tri_kiem_tra + 1]) + (pred_mlp_B == ids_B[vi_tri_kiem_tra + 1])) / 2

logits_tf_A = forward_transformer(mo_hinh_tf, ids_A, dim, mask6)
logits_tf_B = forward_transformer(mo_hinh_tf, ids_B, dim, mask6)
pred_tf_A = int(np.argmax(logits_tf_A.data[vi_tri_kiem_tra]))
pred_tf_B = int(np.argmax(logits_tf_B.data[vi_tri_kiem_tra]))
acc_tf = ((pred_tf_A == ids_A[vi_tri_kiem_tra + 1]) + (pred_tf_B == ids_B[vi_tri_kiem_tra + 1])) / 2

print(mlp_giong_het)
print(acc_mlp, acc_tf)
print(round(lich_su_mlp[0], 6), round(lich_su_mlp[-1], 6))
print(round(lich_su_tf[0], 6), round(lich_su_tf[-1], 6))
```

```python title=test
assert mlp_giong_het == True, "mlp_giong_het phai la True -- logit MLP tai vi tri kiem tra phai giong het nhau giua A va B"
assert acc_mlp == 0.5, f"acc_mlp sai -- dang ra {acc_mlp}"
assert acc_tf == 1.0, f"acc_tf sai -- dang ra {acc_tf}"
assert round(lich_su_mlp[0], 6) == 2.256497, f"lich_su_mlp dau sai -- dang ra {round(lich_su_mlp[0], 6)}"
assert round(lich_su_mlp[-1], 6) == 0.453143, f"lich_su_mlp cuoi sai -- dang ra {round(lich_su_mlp[-1], 6)}"
assert round(lich_su_tf[0], 6) == 2.013202, f"lich_su_tf dau sai -- dang ra {round(lich_su_tf[0], 6)}"
assert round(lich_su_tf[-1], 6) == 0.285035, f"lich_su_tf cuoi sai -- dang ra {round(lich_su_tf[-1], 6)}"

# rieng kiem tra Transformer PHAI vuot troi MLP tren bai toan phu thuoc xa nay
assert acc_tf > acc_mlp, f"acc_tf phai lon hon acc_mlp -- dang ra acc_tf={acc_tf}, acc_mlp={acc_mlp}"

# rieng kiem tra ca hai mo hinh DEU duoc huan luyen that (loss giam), khong
# phai acc_mlp thap vi MLP khong duoc huan luyen gi ca
assert lich_su_mlp[-1] < lich_su_mlp[0], "MLP cung phai duoc huan luyen that (loss phai giam)"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu kiểm tra hai mảng logit có giống hệt nhau từng phần tử không — `np.array_equal(logits_mlp_A.data[vi_tri_kiem_tra], logits_mlp_B.data[vi_tri_kiem_tra])`. Chỗ hai/ba tính accuracy trên `2` ví dụ — đếm số dự đoán ĐÚNG (so với token thật ở vị trí kế tiếp) rồi chia cho `2` — `((pred_..._A == ids_A[vi_tri_kiem_tra + 1]) + (pred_..._B == ids_B[vi_tri_kiem_tra + 1])) / 2`, thay `...` bằng `mlp` hoặc `tf` tương ứng.
- kind: strategy
  body: 'Chỗ đầu: `np.array_equal(logits_mlp_A.data[vi_tri_kiem_tra], logits_mlp_B.data[vi_tri_kiem_tra])`. Chỗ hai: `((pred_mlp_A == ids_A[vi_tri_kiem_tra + 1]) + (pred_mlp_B == ids_B[vi_tri_kiem_tra + 1])) / 2`. Chỗ ba: `((pred_tf_A == ids_A[vi_tri_kiem_tra + 1]) + (pred_tf_B == ids_B[vi_tri_kiem_tra + 1])) / 2`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `np.array_equal(...)`, `((pred_mlp_A == ...) + (pred_mlp_B == ...)) / 2`, và cùng công thức đó với `pred_tf_A`/`pred_tf_B`.'
:::

:::validate
- tier: run
  timeoutMs: 20000
- tier: static
  onFail: mlp_giong_het phai goi THAT np.array_equal (khong duoc chep san True); acc_mlp phai dung THAT pred_mlp_A va pred_mlp_B (khong duoc chep san so); acc_tf phai dung THAT pred_tf_A va pred_tf_B (khong duoc chep san so)
  requireAst:
  - kind: uses-call, target: array_equal, min: 1
  - kind: uses-name, target: logits_mlp_A, min: 2
  - kind: uses-name, target: logits_mlp_B, min: 2
  - kind: uses-name, target: pred_mlp_A, min: 1
  - kind: uses-name, target: pred_mlp_B, min: 1
  - kind: uses-name, target: pred_tf_A, min: 1
  - kind: uses-name, target: pred_tf_B, min: 1
  # Da thu THAT bang kiemAst that (goi truc tiep tren code trich tu solution
  # da bien dich, khong doan tay).
  # array_equal=1: CHI mot lan goi trong blank1 -- khong noi nao khac trong
  # solution goi np.array_equal. Dien bua "mlp_giong_het = True" lam so nay
  # tut xuong 0 -- duoi nguong min=1, bi chan.
  # logits_mlp_A=2, logits_mlp_B=2: moi ten doc lai o blank1 (trong
  # array_equal) VA o dong tinh pred_mlp_A/pred_mlp_B (dong CO SAN, khong
  # phai blank). Dien bua blank1 lam moi ten tut xuong 1 -- duoi nguong
  # min=2, bi chan RIENG boi hai luat nay (doc lap voi luat array_equal).
  # pred_mlp_A=1, pred_mlp_B=1: CHI doc lai duy nhat trong blank2 (acc_mlp)
  # -- day la LAN DOC DUY NHAT trong toan bo solution. Dien bua
  # "acc_mlp = 0.5" (chep san) lam ca hai ten nay tut xuong 0 -- duoi
  # nguong min=1, bi chan.
  # pred_tf_A=1, pred_tf_B=1: tuong tu, CHI doc lai duy nhat trong blank3
  # (acc_tf). Dien bua "acc_tf = 1.0" (chep san) lam ca hai ten nay tut
  # xuong 0 -- bi chan.
  # Cheat "acc_mlp = (pred_mlp_A == ids_A[vi_tri_kiem_tra+1]) +
  # (pred_mlp_B == ids_B[vi_tri_kiem_tra+1])" (quen chia 2, van doc du
  # pred_mlp_A/pred_mlp_B nen KHONG bi static bat) -- da tu kiem chung: cho
  # gia tri 1 (khong phai 0.5), bi bat DOC LAP boi tier tests
  # ("assert acc_mlp == 0.5").
- tier: tests
  timeoutMs: 20000
- tier: output
  match: regex
  expect: "^True\\n0\\.5 1\\.0\\n2\\.256497 0\\.453143\\n2\\.013202 0\\.285035\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
MLP theo-vị-trí: `acc_mlp = 0,5`, mãi mãi — một trần accuracy có thể CHỨNG MINH bằng đại số, không phải một con số xui xẻo. Transformer: `acc_tf = 1,0` sau `20` bước — attention "nhìn" được xuyên qua khoảng cách xa mà MLP không thể. Bài sau: ráp lại toàn bộ pipeline BPE + embedding + Transformer, lần này trên một corpus LỚN HƠN hẳn.
::::

::::reflect{#nghi-lai}
Bài trước chứng minh Transformer không vanishing gradient (nhờ residual). Bài này chứng minh một điều KHÁC: ngay cả khi gradient lan tốt, một kiến trúc không có cơ chế TRỘN THÔNG TIN giữa các vị trí (MLP theo-vị-trí) vẫn có một TRẦN biểu đạt không thể vượt qua bằng huấn luyện — `acc_mlp = 0,5` là giới hạn CẤU TRÚC, chứng minh được bằng đại số, không phải giới hạn của quá trình tối ưu hoá. Attention giải quyết đúng vấn đề này: nó là cơ chế DUY NHẤT trong khối Transformer cho phép thông tin ở một vị trí ảnh hưởng tới đầu ra ở vị trí KHÁC.

Hai bài đầu của quest này đã trả lời xong câu hỏi "vì sao Transformer tốt hơn MLP" bằng số thật (residual chống vanishing, attention giải phụ thuộc xa). Ba bài còn lại chuyển hướng: ráp TOÀN BỘ pipeline (BPE, embedding, Transformer, huấn luyện, sinh văn bản) trên dữ liệu LỚN HƠN hẳn những gì track này đã dùng — đúng yêu cầu MASTERPLAN về corpus `~5KB`, đo thời gian chạy thật, và cuối cùng đối diện với GIỚI HẠN thật của một mô hình kích thước `dim = 4-8` này.
::::

::::checkpoint{mastery=0.85}
::::
