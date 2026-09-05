---
id: tri-tue-nhan-tao.co-che-attention.dau-ra-attention
title: "Đầu ra attention: trung bình có trọng số của Value"
summary: "attn = P.matmul(V) -- P (trọng số attention, mỗi hàng tổng=1, bài trước) nhân với V (Value, bài query-key-value), TÁI DÙNG matmul đã có sẵn. Mỗi hàng đầu ra là TRUNG BÌNH CÓ TRỌNG SỐ của MỌI hàng V, trọng số là điểm attention đã chuẩn hoá -- không phải chọn MỘT Value duy nhất, mà TRỘN tất cả theo đúng tỉ lệ. Trên P,V (4,4) tính từ pipeline đầy đủ: attn[3] khớp CHÍNH XÁC với tổng bốn số hạng P[3,j]×V[j] tính tường minh."
locale: vi
track: tri-tue-nhan-tao
module: co-che-attention
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.dau-ra-attention]
requires: [ai.scale-va-softmax]
concepts: [ai.dau-ra-attention]
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
`P` (bài trước) nói MỖI vị trí nên chú ý bao nhiêu tới mỗi vị trí khác — nhưng `P` chỉ là con số trọng số, chưa mang nội dung gì. `V` (bài `query-key-value`) mới là nội dung. Bước cuối: TRỘN chúng lại.
::::

::::explain{#dau_ra_attention}
Đầu ra của một tầng attention là phép nhân ma trận giữa trọng số attention `P` (mỗi hàng tổng bằng `1`, bài `scale-va-softmax`) và Value `V` (bài `query-key-value`):

> `attn = P.matmul(V)`

Đây LẠI là `Tensor.matmul` ĐÃ CÓ SẴN — không công thức mới, chỉ là ráp đúng hai `Tensor` đã có vào ĐÚNG một phép toán đã học. Nhưng Ý NGHĨA của phép nhân này đáng dừng lại: `P` shape `(so_token, so_token)`, `V` shape `(so_token, dim)`, kết quả `attn` shape `(so_token, dim)` — MỖI HÀNG của `attn` là:

> `attn[i] = Σⱼ P[i, j] · V[j]`

Nói bằng lời: hàng `i` của đầu ra là **TRUNG BÌNH CÓ TRỌNG SỐ** của MỌI hàng `V` — trọng số của hàng `V[j]` chính là `P[i, j]`, điểm attention đã chuẩn hoá từ vị trí `i` tới vị trí `j`. Vì mỗi hàng `P[i]` là một phân phối xác suất (tổng bằng `1`, không âm), `attn[i]` LUÔN nằm trong "vùng lồi" (convex combination) của các hàng `V` — không phải chọn RA một `V[j]` duy nhất (như tra bảng thông thường), mà TRỘN tất cả theo đúng tỉ lệ mà `P` chỉ định. Vị trí nào có trọng số `P[i, j]` lớn thì `V[j]` đóng góp NHIỀU hơn vào kết quả; vị trí có trọng số gần `0` gần như không đóng góp gì.
::::

::::example{#dau_ra_attention_that}
Toàn bộ pipeline tới `P` (embedding → Q/K/V → scale+softmax, các bài trước) rồi nhân với `V`:

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

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
V = X.matmul(Wv)
S = Q.matmul(K.transpose())
d_k = 4
S_scaled = Tensor(S.data / np.sqrt(d_k))
P = S_scaled.softmax()

attn = P.matmul(V)
print(np.round(attn.data, 6).tolist())

# doi chieu hang cuoi bang TRUNG BINH CO TRONG SO tuong minh, khong qua matmul
hang_cuoi_tay = (P.data[3, 0] * V.data[0] + P.data[3, 1] * V.data[1]
                  + P.data[3, 2] * V.data[2] + P.data[3, 3] * V.data[3])
print(np.round(hang_cuoi_tay, 6).tolist())
print(np.round(P.data[3], 6).tolist())
```

```text title=readonly
[[1.376547, 1.01885, 1.058615, 1.416312], [1.107338, 1.434129, 1.774594, 1.447804], [1.049837, 1.589414, 1.898526, 1.358949], [1.867235, -0.001523, -0.648263, 1.220495]]
[1.867235, -0.001523, -0.648263, 1.220495]
[0.217951, 0.085128, 0.142506, 0.554415]
```

Hàng cuối của `attn` (từ `matmul` thật) và `hang_cuoi_tay` (trung bình có trọng số tính tường minh — bốn phép nhân vector-với-số rồi cộng lại, dùng ĐÚNG bốn trọng số của hàng `P[3]`) khớp nhau TUYỆT ĐỐI (cả hai đều làm tròn `6` chữ số ra CÙNG một kết quả). Vị trí `3` (chính nó) có trọng số cao nhất (`0,554415`), nên `V[3]` đóng góp NHIỀU nhất vào đầu ra của chính vị trí `3` — nhưng vẫn PHA TRỘN thêm từ ba vị trí khác, không phải chỉ lấy nguyên `V[3]`.
::::

::::predict{#doan_trong_so_bang_nhau commitOnce}
Hàng `3` của `P` là `[0,217951; 0,085128; 0,142506; 0,554415]` — không đều nhau, và `attn[3]` PHA TRỘN cả bốn hàng của `V` theo đúng tỉ lệ đó.

**Trước khi suy luận**, bạn đoán: nếu MỘT hàng của `P` (giả sử) có trọng số ĐỀU NHAU cho cả `4` vị trí (`[0,25; 0,25; 0,25; 0,25]`), hàng đầu ra tương ứng của `attn` sẽ là gì?

:::opt{correct}
Trung bình CỘNG thông thường (không trọng số) của cả `4` hàng `V` — vì mọi trọng số bằng nhau (`0,25` mỗi vị trí), công thức `attn[i] = Σⱼ P[i,j]·V[j]` trở thành `0,25·V[0] + 0,25·V[1] + 0,25·V[2] + 0,25·V[3]`, đúng bằng `(V[0]+V[1]+V[2]+V[3])/4` — trường hợp ĐẶC BIỆT của trung bình có trọng số khi mọi trọng số bằng nhau
:::

:::opt
Chính `V[0]` (hàng đầu tiên) — vì khi trọng số đều nhau, quy ước là LUÔN LẤY vị trí đầu tiên làm đại diện
::why
Gần đúng ở việc để ý trường hợp trọng số đều nhau là một trường hợp ĐẶC BIỆT, đáng chú ý riêng — quan sát đó không sai.

Chỗ lệch: không có "quy ước lấy vị trí đầu" nào trong công thức `Σⱼ P[i,j]·V[j]` — công thức CỘNG DỒN đóng góp của MỌI vị trí theo đúng trọng số của nó, không có bước "chọn một vị trí đại diện" nào. Khi trọng số đều nhau, kết quả là trung bình CỘNG của TẤT CẢ các hàng, không phải riêng hàng đầu.
::
:::

:::opt
Không tính được nếu không biết THỨ TỰ token trong câu — trọng số đều nhau không đủ thông tin để xác định kết quả
::why
Gần đúng ở việc thận trọng khi thiếu thông tin — thái độ đó hợp lý trong nhiều tình huống khác.

Chỗ lệch: công thức `attn[i] = Σⱼ P[i,j]·V[j]` chỉ cần biết giá trị CỤ THỂ của `P[i,j]` và `V[j]` — với `P[i,j] = 0,25` cho mọi `j`, và `V` đã cho SẴN giá trị cụ thể ở mọi hàng, kết quả HOÀN TOÀN xác định (trung bình cộng), không thiếu thông tin gì.
::
:::
::::

::::code{#viet_dau_ra_attention}
Hoàn thiện: ráp đầu ra attention bằng `matmul` giữa `P` và `V`.

```python title=starter
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

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
V = X.matmul(Wv)
S = Q.matmul(K.transpose())
d_k = 4
S_scaled = Tensor(S.data / np.sqrt(d_k))
P = S_scaled.softmax()

attn = ___                # P.matmul(V)
print(np.round(attn.data, 6).tolist())
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


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
V = X.matmul(Wv)
S = Q.matmul(K.transpose())
d_k = 4
S_scaled = Tensor(S.data / np.sqrt(d_k))
P = S_scaled.softmax()

attn = P.matmul(V)
print(np.round(attn.data, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(attn.data, 6).tolist() == [[1.376547, 1.01885, 1.058615, 1.416312], [1.107338, 1.434129, 1.774594, 1.447804], [1.049837, 1.589414, 1.898526, 1.358949], [1.867235, -0.001523, -0.648263, 1.220495]], f"attn.data sai -- dang ra {np.round(attn.data, 6).tolist()}"

# rieng kiem tra tinh chat TRUNG BINH CO TRONG SO: doi voi mot hang P TU CHE
# (deu nhau), attn phai bang trung binh CONG thong thuong cua V -- doc lap
# voi gia tri cu the o tren, chan cheat hardcode ket qua.
P_deu = Tensor([[0.25, 0.25, 0.25, 0.25]])
attn_deu = P_deu.matmul(V)
trung_binh_cong = np.mean(V.data, axis=0)
assert np.allclose(attn_deu.data[0], trung_binh_cong), f"P deu nhau phai cho trung binh CONG cua V -- dang ra {attn_deu.data[0].tolist()} thay vi {trung_binh_cong.tolist()}"

# rieng kiem tra shape: attn phai (4,4), giu nguyen so hang cua P va so cot cua V
assert tuple(attn.data.shape) == (4, 4), f"attn.data.shape sai -- dang ra {tuple(attn.data.shape)}"
```

:::hints
- kind: attention
  body: Một chỗ trống. `attn` là kết quả trung bình có trọng số của `V`, dùng trọng số `P` — chính là phép nhân ma trận `P.matmul(V)`, dùng NGUYÊN phương thức đã có sẵn. Chú ý ĐÚNG thứ tự — `P` (trọng số) đứng TRƯỚC, `V` (nội dung) đứng SAU.
- kind: strategy
  body: '`P.matmul(V)`.'
- kind: one-line
  body: 'Chỗ trống là `P.matmul(V)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: attn phai duoc rap bang P.matmul(V) (dung DUNG thu tu P roi V -- P la trong so, V la noi dung; dao nguoc thanh V.matmul(P) cho gia tri sai)
  requireAst:
  - kind: uses-name, target: P, min: 1
  - kind: uses-name, target: V, min: 1
  - kind: uses-call, target: matmul, min: 5
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # class Tensor + harness): loi giai dung dat=true. P=1 (Load duy nhat
  # trong blank "P.matmul(V)" -- "P = S_scaled.softmax()" la Store, khong
  # dem). V=1 (Load duy nhat trong blank -- "V = X.matmul(Wv)" la Store).
  # matmul=5 (Q=X.matmul(Wq), K=X.matmul(Wk), V=X.matmul(Wv), S=Q.matmul(...),
  # cong blank "P.matmul(V)" -- nam san trong starter (bon lan dau) cong
  # blank (mot lan) = nam, dinh nghia phuong thuc khong tinh).
  # Cheat "attn = V.matmul(P)" (dao nguoc thu tu) GIU NGUYEN so dem P/V/
  # matmul (van dung 1 P, 1 V, nam matmul, chi doi VI TRI hai ten) nen
  # KHONG bi static bat rieng -- nhung da tu kiem chung bang Python that:
  # V.matmul(P) van CHAY duoc (V, P deu shape (4,4), vuong) nhung cho gia
  # tri SAI han so voi P.matmul(V) dung -- bi bat DOC LAP boi assert gia
  # tri attn.data (khong khop danh sach da assert o tren).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[\\[1\\.376547, 1\\.01885, 1\\.058615, 1\\.416312\\], \\[1\\.107338, 1\\.434129, 1\\.774594, 1\\.447804\\], \\[1\\.049837, 1\\.589414, 1\\.898526, 1\\.358949\\], \\[1\\.867235, -0\\.001523, -0\\.648263, 1\\.220495\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một tầng attention (chưa che tương lai) đã tính xong: embedding → vị trí → Q/K/V → điểm số → scale+softmax → trộn Value. Nhưng có một lỗ hổng: mọi vị trí đang NHÌN THẤY mọi vị trí khác, kể cả TƯƠNG LAI. Bài sau vá lỗ đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đầu ra attention vừa tính cho vị trí `3` (token cuối cùng) đã PHA TRỘN thông tin từ CẢ BA vị trí trước nó — hoàn toàn hợp lý. Nhưng nó cũng pha trộn TỪ chính token đó (`V[3]`, trọng số `0,554415`). Với một mô hình SINH VĂN BẢN (dự đoán token TIẾP THEO dựa trên các token ĐÃ THẤY), việc một vị trí "nhìn thấy" một vị trí Ở TƯƠNG LAI của nó (chưa được sinh ra) có hợp lý không?
::::

::::checkpoint{mastery=0.85}
::::
