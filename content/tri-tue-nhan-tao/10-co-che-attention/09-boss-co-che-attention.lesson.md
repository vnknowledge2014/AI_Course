---
id: tri-tue-nhan-tao.co-che-attention.boss-co-che-attention
title: "BOSS — Một tầng attention đầy đủ, đối chiếu số tính tay"
summary: "Ráp TOÀN BỘ pipeline: embedding_lookup + positional encoding (làm tròn 6 chữ số để khớp đúng độ chính xác đã dùng xuyên suốt quest) -> Q/K/V -> điểm attention -> scale+causal mask+softmax -> đầu ra -> multi-head. Trên câu 4 token (ID 0 lặp lại), S[0,2] khớp CHÍNH XÁC điểm tính tay 6 của bài diem-attention-tinh-tay. Multi-head với so_head=1 (trường hợp suy biến, không thực sự tách) tái tạo ĐÚNG BẰNG BIT kết quả một-đầu của các bài trước; so_head=4 (tách thật) cho kết quả KHÁC hẳn -- xác nhận multi-head tổng quát đúng ở cả hai đầu mút. Đóng quest co-che-attention (9/9)."
locale: vi
track: tri-tue-nhan-tao
module: co-che-attention
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-co-che-attention]
requires: [ai.da-dau-attention]
concepts: [ai.boss-co-che-attention]
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
Tám bài, tám mảnh: tra cứu embedding, vị trí, Query/Key/Value, điểm attention tính tay, scale+softmax, đầu ra, causal mask, multi-head. Bài này ráp TẤT CẢ vào MỘT pipeline, trên một câu 4 token — và đối chiếu lại với con số đã tính TAY từ bài `diem-attention-tinh-tay`.
::::

::::explain{#rap_toan_bo_attention}
Một tầng attention ĐẦY ĐỦ, KHÔNG thêm gì mới — ĐÚNG những gì tám bài trước đã xây:

> **`embedding_lookup`** — token ID sang vector, gradient CỘNG DỒN qua ID lặp lại (bài `tra-cuu-embedding`).
>
> **`ma_hoa_vi_tri` + `__add__`** — bơm vị trí vào embedding (bài `ma-hoa-vi-tri`).
>
> **`Q = X.matmul(Wq)`, `K = X.matmul(Wk)`, `V = X.matmul(Wv)`** — ba phép chiếu (bài `query-key-value`).
>
> **`S = Q.matmul(K.transpose())`** — điểm attention, ĐÃ tính tay `S[0, 2] = 6` (bài `diem-attention-tinh-tay`).
>
> **`S_scaled = S / √d_k`, rồi `softmax`** — chuẩn hoá thành trọng số (bài `scale-va-softmax`).
>
> **causal mask** — cộng `-∞` vào các vị trí `j > i` TRƯỚC `softmax` (bài `mat-na-nhan-qua`).
>
> **`attn = P.matmul(V)`** — trộn Value theo trọng số (bài `dau-ra-attention`).
>
> **`tach_dau`/`ghep_dau`** — chạy nhiều đầu song song rồi nối lại (bài `da-dau-attention`).

Một chi tiết kỹ thuật đáng nói rõ: `embedding_lookup(ids) + PE` cho một `X` với độ chính xác ĐẦY ĐỦ của `numpy` (nhiều chữ số hơn `6` chữ số thập phân đã dùng để HIỂN THỊ xuyên suốt quest). Để `X` khớp ĐÚNG với các con số đã in ra ở những bài trước (vốn đều làm tròn `6` chữ số), BOSS làm tròn `X` về `6` chữ số NGAY sau bước cộng vị trí — trước khi đưa vào `Q`/`K`/`V`. Đây không phải một mẹo che giấu sai số: đó là NEO độ chính xác của pipeline THẬT khớp với độ chính xác đã dùng để so sánh trong suốt quest, y hệt việc mọi bài trước đều `round(..., 6)` trước khi in.
::::

::::example{#boss_full_pipeline}
Câu `4` token, ID `0` xuất hiện HAI LẦN (vị trí `0` và `3`) — chạy trọn pipeline một-đầu, đối chiếu `S[0, 2]` với điểm tính tay từ bài `diem-attention-tinh-tay`:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def embedding_lookup(self, ids):
        out = Tensor(self.data[ids], (self,), 'embedding_lookup')
        return out

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        return out

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = np.sin(goc[:, 0::2])
    pe[:, 1::2] = np.cos(goc[:, 1::2])
    return pe


Bang = Tensor([[1.0, -1.0, 1.0, -1.0], [-0.841471, 0.459698, 0.99, 0.00005], [0.090703, 1.416147, -0.019999, 0.0002]])
ids = [0, 1, 2, 0]   # 4 token, ID 0 xuat hien o vi tri 0 VA vi tri 3

PE = Tensor(ma_hoa_vi_tri(4, 4))
X = Tensor(np.round((Bang.embedding_lookup(ids) + PE).data, 6))
print(X.data.tolist())

Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
V = X.matmul(Wv)

diem_tay_bai_4 = 2.0 * 1.0 + 1.0 * 2.0 + 1.0 * 2.0 + 0.0 * 1.0   # tinh tay tu bai diem-attention-tinh-tay

S = Q.matmul(K.transpose())
print(round(float(S.data[0, 2]), 6), "==", diem_tay_bai_4)
print(round(float(S.data[0, 2]), 6) == diem_tay_bai_4)
```

```text title=readonly
[[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
6.0 == 6.0
True
```

`X` tái tạo ĐÚNG các giá trị đã dùng xuyên suốt quest — ba hàng đầu (vị trí `0`, `1`, `2`) là các vector "sạch" đã thiết kế, hàng cuối (vị trí `3`, dùng LẠI embedding của ID `0` nhưng cộng vị trí `3`) là vector "thật" (có cả `sin`/`cos`). `S[0, 2]` từ `Tensor.matmul` THẬT khớp CHÍNH XÁC với `diem_tay_bai_4` — con số đã tính tay ở bài `diem-attention-tinh-tay`, không lệch một chữ số nào.
::::

::::example{#boss_multi_head_bien}
Chạy multi-head với `so_head = 1` (trường hợp suy biến — "một đầu" nghĩa là KHÔNG thực sự tách gì) và `so_head = 4` (tách thật, mỗi đầu chỉ còn `1` chiều), rồi so với pipeline MỘT-ĐẦU đầy đủ (scale + causal mask + softmax + `matmul V`, các bài `scale-va-softmax`/`mat-na-nhan-qua`/`dau-ra-attention`):

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        return out

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        return out


def tach_dau(T, so_dau):
    n_tok, d = T.data.shape
    dd = d // so_dau
    T3 = T.reshape((n_tok, so_dau, dd))
    return [Tensor(T3.data[:, h, :]) for h in range(so_dau)]


def ghep_dau(danh_sach_dau):
    return Tensor(np.concatenate([t.data for t in danh_sach_dau], axis=-1))


Q = Tensor([[2.0, 1.0, 1.0, 0.0], [1.0, 2.0, 1.0, 2.0], [1.0, 1.0, 2.0, 2.0], [2.171116, -0.959996, 1.14067, -1.990442]])
K = Tensor([[2.0, 1.0, 0.0, 1.0], [1.0, 1.0, 2.0, 2.0], [1.0, 2.0, 2.0, 1.0], [2.171116, 1.14067, -1.990442, -0.959996]])
V = Tensor([[2.0, 1.0, 0.0, 1.0], [1.0, 2.0, 2.0, 1.0], [1.0, 1.0, 2.0, 2.0], [2.171116, -0.959996, -1.990442, 1.14067]])

mask = Tensor(np.where(np.triu(np.ones((4, 4)), k=1) == 1, float('-inf'), 0.0))


def chay_attention_da_dau(so_head):
    dim_dau = 4 // so_head
    Qh, Kh, Vh = tach_dau(Q, so_head), tach_dau(K, so_head), tach_dau(V, so_head)
    dau_ra = []
    for h in range(so_head):
        Sh = Qh[h].matmul(Kh[h].transpose())
        Sh_scaled = Tensor(Sh.data / np.sqrt(dim_dau))
        Ph = (Sh_scaled + mask).softmax()
        dau_ra.append(Ph.matmul(Vh[h]))
    return ghep_dau(dau_ra)


# pipeline MOT-DAU day du (khong tach), tham chieu tu cac bai truoc
S = Q.matmul(K.transpose())
P_mot_dau = (Tensor(S.data / np.sqrt(4)) + mask).softmax()
attn_mot_dau = P_mot_dau.matmul(V)

out_so_head_1 = chay_attention_da_dau(1)
out_so_head_4 = chay_attention_da_dau(4)

print(np.round(attn_mot_dau.data, 6).tolist())
print(np.round(out_so_head_1.data, 6).tolist())
print(np.allclose(out_so_head_1.data, attn_mot_dau.data))
print(np.allclose(out_so_head_4.data, attn_mot_dau.data))
```

```text title=readonly
[[2.0, 1.0, 0.0, 1.0], [1.182426, 1.817574, 1.635149, 1.0], [1.048611, 1.592201, 1.902778, 1.359188], [1.867235, -0.001523, -0.648263, 1.220495]]
[[2.0, 1.0, 0.0, 1.0], [1.182426, 1.817574, 1.635149, 1.0], [1.048611, 1.592201, 1.902778, 1.359188], [1.867235, -0.001523, -0.648263, 1.220495]]
True
False
```

`so_head = 1` (không thực sự tách — `tach_dau` với `1` đầu trả về đúng NGUYÊN `Q`/`K`/`V`, không cắt bớt chiều nào) tái tạo ĐÚNG TUYỆT ĐỐI kết quả của pipeline một-đầu đầy đủ — mã multi-head TỔNG QUÁT vẫn cho ra ĐÚNG kết quả ở trường hợp suy biến của nó. `so_head = 4` (tách THẬT, mỗi đầu chỉ còn `1` chiều) cho một kết quả KHÁC HẲN — đúng như kỳ vọng, vì mỗi đầu giờ chỉ "nhìn" được MỘT chiều duy nhất của `Q`/`K`, tính điểm attention trên một không gian con hẹp hơn nhiều.
::::

::::predict{#doan_so_head_4_khac commitOnce}
Ví dụ trên xác nhận: `so_head = 1` khớp TUYỆT ĐỐI với pipeline một-đầu, còn `so_head = 4` cho kết quả KHÁC.

**Trước khi tổng quát hoá**, bạn đoán: với `so_head = 2` (giữa hai thái cực `1` và `4`, mỗi đầu còn `2` chiều — đúng cấu hình đã dùng ở bài `da-dau-attention`), kết quả có khớp TUYỆT ĐỐI với pipeline một-đầu như trường hợp `so_head = 1`, hay khác như trường hợp `so_head = 4`?

:::opt{correct}
Khác — CHỈ `so_head = 1` mới là trường hợp SUY BIẾN (không cắt bớt chiều nào, `tach_dau` trả về nguyên `Q`/`K`/`V`); bất kỳ `so_head > 1` nào (kể cả `2`) đều THỰC SỰ cắt `Q`/`K`/`V` thành các không gian con HẸP HƠN trước khi tính điểm attention — số điểm attention tính trên một không gian con hẹp hơn hầu như luôn khác số điểm attention tính trên không gian ĐẦY ĐỦ, nên kết quả `so_head = 2` cũng khác pipeline một-đầu, giống hệt lý do `so_head = 4` khác
:::

:::opt
Khớp tuyệt đối, giống `so_head = 1` — vì `so_head = 2` vẫn còn "khá gần" với một đầu duy nhất so với `so_head = 4` (tách nhỏ hơn nhiều)
::why
Gần đúng ở việc để ý `so_head = 2` đúng là "ít bị chia nhỏ hơn" `so_head = 4` — quan sát về mức độ chia nhỏ không sai.

Chỗ lệch: không có ngưỡng "đủ gần một đầu thì khớp" nào trong công thức — CHỈ `so_head = 1` mới thực sự KHÔNG cắt chiều nào (`dim/so_head = dim/1 = dim`, giữ nguyên toàn bộ). Với `so_head = 2`, mỗi đầu đã chỉ còn MỘT NỬA số chiều (`dim/2`) — điểm attention tính trên một nửa số chiều đó gần như CHẮC CHẮN khác điểm tính trên đầy đủ `dim` chiều, dù mức độ "cắt" có ít hơn `so_head = 4` hay không.
::
:::

:::opt
Không xác định được nếu không có thêm thông tin về `Q`/`K` cụ thể — có thể khớp hoặc khác tuỳ dữ liệu
::why
Gần đúng ở việc thận trọng khi thiếu ràng buộc cụ thể — thái độ đó hợp lý trong nhiều tình huống khác.

Chỗ lệch: đây KHÔNG phải một câu hỏi phụ thuộc dữ liệu ngẫu nhiên — nó là một sự thật về CẤU TRÚC của phép `reshape`+cắt lát. `so_head = 1` LUÔN giữ nguyên toàn bộ `Q`/`K`/`V` (không phụ thuộc giá trị cụ thể của chúng là gì), còn `so_head ≥ 2` LUÔN cắt bớt chiều — sự khác biệt này đúng cho MỌI `Q`/`K`/`V`, không phải một trường hợp may rủi của riêng ví dụ này.
::
:::
::::

::::code{#viet_boss_pipeline}
Hoàn thiện hai chỗ trống: dựng `X` bằng `embedding_lookup` + vị trí (làm tròn `6` chữ số để khớp đúng độ chính xác đã dùng xuyên suốt quest), và xác nhận `so_head = 1` khớp TUYỆT ĐỐI với pipeline một-đầu.

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def embedding_lookup(self, ids):
        out = Tensor(self.data[ids], (self,), 'embedding_lookup')
        return out

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        return out

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        return out

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = np.sin(goc[:, 0::2])
    pe[:, 1::2] = np.cos(goc[:, 1::2])
    return pe


def tach_dau(T, so_dau):
    n_tok, d = T.data.shape
    dd = d // so_dau
    T3 = T.reshape((n_tok, so_dau, dd))
    return [Tensor(T3.data[:, h, :]) for h in range(so_dau)]


def ghep_dau(danh_sach_dau):
    return Tensor(np.concatenate([t.data for t in danh_sach_dau], axis=-1))


Bang = Tensor([[1.0, -1.0, 1.0, -1.0], [-0.841471, 0.459698, 0.99, 0.00005], [0.090703, 1.416147, -0.019999, 0.0002]])
ids = [0, 1, 2, 0]
PE = Tensor(ma_hoa_vi_tri(4, 4))
X = Tensor(np.round(___, 6))                      # (Bang.embedding_lookup(ids) + PE).data

Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
V = X.matmul(Wv)

S = Q.matmul(K.transpose())
d_k = 4
mask = Tensor(np.where(np.triu(np.ones((4, 4)), k=1) == 1, float('-inf'), 0.0))
P_mot_dau = (Tensor(S.data / np.sqrt(d_k)) + mask).softmax()
attn_mot_dau = P_mot_dau.matmul(V)


def chay_attention_da_dau(so_head):
    dim_dau = 4 // so_head
    Qh, Kh, Vh = tach_dau(Q, so_head), tach_dau(K, so_head), tach_dau(V, so_head)
    dau_ra = []
    for h in range(so_head):
        Sh = Qh[h].matmul(Kh[h].transpose())
        Sh_scaled = Tensor(Sh.data / np.sqrt(dim_dau))
        Ph = (Sh_scaled + mask).softmax()
        dau_ra.append(Ph.matmul(Vh[h]))
    return ghep_dau(dau_ra)


out_so_head_1 = chay_attention_da_dau(1)
khop_suy_bien = ___                                # np.allclose(out_so_head_1.data, attn_mot_dau.data)

print(round(float(S.data[0, 2]), 6))
print(khop_suy_bien)
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

    def embedding_lookup(self, ids):
        out = Tensor(self.data[ids], (self,), 'embedding_lookup')
        return out

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        return out

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        return out

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = np.sin(goc[:, 0::2])
    pe[:, 1::2] = np.cos(goc[:, 1::2])
    return pe


def tach_dau(T, so_dau):
    n_tok, d = T.data.shape
    dd = d // so_dau
    T3 = T.reshape((n_tok, so_dau, dd))
    return [Tensor(T3.data[:, h, :]) for h in range(so_dau)]


def ghep_dau(danh_sach_dau):
    return Tensor(np.concatenate([t.data for t in danh_sach_dau], axis=-1))


Bang = Tensor([[1.0, -1.0, 1.0, -1.0], [-0.841471, 0.459698, 0.99, 0.00005], [0.090703, 1.416147, -0.019999, 0.0002]])
ids = [0, 1, 2, 0]
PE = Tensor(ma_hoa_vi_tri(4, 4))
X = Tensor(np.round((Bang.embedding_lookup(ids) + PE).data, 6))

Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
V = X.matmul(Wv)

S = Q.matmul(K.transpose())
d_k = 4
mask = Tensor(np.where(np.triu(np.ones((4, 4)), k=1) == 1, float('-inf'), 0.0))
P_mot_dau = (Tensor(S.data / np.sqrt(d_k)) + mask).softmax()
attn_mot_dau = P_mot_dau.matmul(V)


def chay_attention_da_dau(so_head):
    dim_dau = 4 // so_head
    Qh, Kh, Vh = tach_dau(Q, so_head), tach_dau(K, so_head), tach_dau(V, so_head)
    dau_ra = []
    for h in range(so_head):
        Sh = Qh[h].matmul(Kh[h].transpose())
        Sh_scaled = Tensor(Sh.data / np.sqrt(dim_dau))
        Ph = (Sh_scaled + mask).softmax()
        dau_ra.append(Ph.matmul(Vh[h]))
    return ghep_dau(dau_ra)


out_so_head_1 = chay_attention_da_dau(1)
khop_suy_bien = np.allclose(out_so_head_1.data, attn_mot_dau.data)

print(round(float(S.data[0, 2]), 6))
print(khop_suy_bien)
```

```python title=test
import numpy as np

assert round(float(S.data[0, 2]), 6) == 6.0, f"S.data[0,2] sai -- dang ra {round(float(S.data[0, 2]), 6)}"
assert khop_suy_bien == True, "so_head=1 phai khop TUYET DOI voi pipeline mot-dau"

# BANG CHUNG TRUNG TAM cua BOSS: doi chieu TOAN BO cac gia tri chinh voi
# so lieu da tinh/xac nhan xuyen suot quest.
assert np.round(X.data, 6).tolist() == [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]], f"X.data sai -- dang ra {np.round(X.data, 6).tolist()}"
assert np.round(Q.data, 6).tolist() == [[2.0, 1.0, 1.0, 0.0], [1.0, 2.0, 1.0, 2.0], [1.0, 1.0, 2.0, 2.0], [2.171116, -0.959996, 1.14067, -1.990442]], f"Q.data sai -- dang ra {np.round(Q.data, 6).tolist()}"
assert np.round(attn_mot_dau.data, 6).tolist() == [[2.0, 1.0, 0.0, 1.0], [1.182426, 1.817574, 1.635149, 1.0], [1.048611, 1.592201, 1.902778, 1.359188], [1.867235, -0.001523, -0.648263, 1.220495]], f"attn_mot_dau.data sai -- dang ra {np.round(attn_mot_dau.data, 6).tolist()}"

# rieng kiem tra so_head=4 (tach THAT) KHONG khop pipeline mot-dau -- chan
# cheat "khop_suy_bien" luon tra ve True bat ke so_head nao.
out_so_head_4 = chay_attention_da_dau(4)
assert not np.allclose(out_so_head_4.data, attn_mot_dau.data), "so_head=4 (tach THAT) khong duoc khop voi pipeline mot-dau -- day la bang chung multi-head THUC SU chia nho khong gian"

# rieng kiem tra multi-head so_head=2 (cau hinh da dung o bai da-dau-attention) cho dung gia tri
out_so_head_2 = chay_attention_da_dau(2)
assert np.round(out_so_head_2.data, 6).tolist() == [[2.0, 1.0, 0.0, 1.0], [1.669762, 1.330238, 1.888386, 1.0], [1.401112, 1.197776, 1.977146, 1.193335], [1.951166, 0.161714, 0.594117, 1.524645]], f"out_so_head_2.data sai -- dang ra {np.round(out_so_head_2.data, 6).tolist()}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu ráp `X` từ TỔNG `embedding_lookup(ids)` và `PE`, lấy `.data` của nó để đưa vào `np.round(..., 6)` — `(Bang.embedding_lookup(ids) + PE).data`. Chỗ hai xác nhận `so_head = 1` khớp TUYỆT ĐỐI với pipeline một-đầu — dùng `np.allclose` giữa hai mảng `.data` tương ứng — `np.allclose(out_so_head_1.data, attn_mot_dau.data)`.
- kind: strategy
  body: 'Chỗ đầu: `(Bang.embedding_lookup(ids) + PE).data`. Chỗ hai: `np.allclose(out_so_head_1.data, attn_mot_dau.data)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `(Bang.embedding_lookup(ids) + PE).data` và `np.allclose(out_so_head_1.data, attn_mot_dau.data)`.'
:::

:::validate
- tier: run
  timeoutMs: 15000
- tier: static
  onFail: X phai duoc rap tu (Bang.embedding_lookup(ids) + PE).data (dung DUNG embedding_lookup va PE, khong duoc chep san mang so hay bo qua mot trong hai); khop_suy_bien phai goi THAT np.allclose giua out_so_head_1.data va attn_mot_dau.data
  requireAst:
  - kind: uses-call, target: embedding_lookup, min: 1
  - kind: uses-name, target: PE, min: 1
  - kind: uses-call, target: allclose, min: 1
  - kind: uses-name, target: out_so_head_1, min: 1
  - kind: uses-name, target: attn_mot_dau, min: 1
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # class Tensor + ham + harness): loi giai dung dat=true. embedding_lookup=1
  # (chi trong blank1 -- dinh nghia phuong thuc khong tinh). PE=1 (dinh
  # nghia "PE = Tensor(...)" la Store, khong dem; Load duy nhat la trong
  # blank1). allclose=1 (chi trong blank2). out_so_head_1=1 (dinh nghia la
  # Store, khong dem; Load duy nhat la trong blank2). attn_mot_dau=1
  # (dinh nghia la Store; Load duy nhat la trong blank2).
  # Cheat "X = Tensor(np.round(PE.data, 6))" (bo qua embedding_lookup hoan
  # toan, chi dung PE) lam "embedding_lookup" tut xuong 0 -- bi chan RIENG,
  # VA da tu kiem chung bang Python that: X.data khi do KHONG con dung
  # (thieu phan embedding, chi con vi tri) -- bi bat DOC LAP boi assert
  # X.data.
  # Cheat "khop_suy_bien = True" (chep san, bo qua so sanh THAT) lam
  # "allclose"/"out_so_head_1"/"attn_mot_dau" deu tut xuong 0 -- bi chan
  # BA LAN, VA se KHONG con bi bat boi test gia tri khop_suy_bien (vi True
  # tinh co dung voi truong hop nay) -- nhung day chinh la ly do CAN static
  # rieng: neu khong co static, cheat nay se qua tier tests ma khong thuc
  # su SO SANH gi ca -- static bat no truoc khi kip toi do.
- tier: tests
  timeoutMs: 15000
- tier: output
  match: regex
  expect: "^6\\.0\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Điểm attention tính tay ở bài `diem-attention-tinh-tay` khớp CHÍNH XÁC với pipeline `Tensor` đầy đủ — embedding, vị trí, Query/Key/Value, scale, causal mask, softmax, multi-head, tất cả ráp lại đúng. Quest `co-che-attention` khép lại tại đây.
::::

::::reflect{#nghi-lai}
Quest `co-che-attention` khép lại tại đây, chín bài: tra cứu embedding (gradient CỘNG DỒN qua ID lặp lại, không GHI ĐÈ), positional encoding (sin/cos xen kẽ, cùng token khác vị trí cho vector khác nhau), Query/Key/Value (ba phép chiếu, ba vai trò), điểm attention TÍNH TAY (`Q[0]·K[2] = 6`, khớp chính xác `Tensor.matmul` thật, và KHÔNG đối xứng), scale + softmax (chia `√d_k` chống bão hoà), đầu ra attention (trung bình có trọng số của Value), causal mask (`-∞` thật, dựng bằng `np.where` chứ không phải phép nhân — tránh đúng gotcha `0 × (-∞) = nan`), multi-head (tách bằng `reshape`, nối bằng `concatenate`, mỗi đầu một không gian con riêng), và BOSS này: ráp toàn bộ, đối chiếu số tính tay, xác nhận multi-head tổng quát đúng ở CẢ hai đầu mút (`so_head=1` suy biến khớp tuyệt đối, `so_head=4` tách thật cho kết quả khác).

Một tầng attention đầy đủ giờ đã có — quest sau, `khoi-transformer-va-huan-luyen` (q8.3d), ráp nó vào một KHỐI Transformer hoàn chỉnh (kết nối tắt, feed-forward, chuẩn hoá lớp xen kẽ) và huấn luyện thật.
::::

::::checkpoint{mastery=0.95}
::::
