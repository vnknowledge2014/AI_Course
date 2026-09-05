---
id: tri-tue-nhan-tao.huan-luyen-mang-no-ron.adam-toi-uu-hoa
title: "Adam: bền vững hơn động lượng"
summary: "Adam giữ HAI trung bình động — m (bậc 1, giống động lượng) và v (bậc 2, bình phương gradient) — có hiệu chỉnh lệch (bias correction) rồi cập nhật param -= lr*m_hat/(sqrt(v_hat)+eps). Trên bài toán khe hẹp của bài trước, tại CÙNG lr=0.1: Momentum (beta=0.7, đã tốt ở lr=0.025) PHÂN KỲ hoàn toàn; Adam (beta1=0.9, beta2=0.999 — mặc định chuẩn, không dò) hội tụ bền vững sau 92 bước, loss=0.008. Adam epoch 1 (có bias correction) cho loss=162.81 — bỏ hiệu chỉnh lệch đổi hẳn quỹ đạo những bước đầu (93.98) dù cùng hội tụ về sau."
locale: vi
track: tri-tue-nhan-tao
module: huan-luyen-mang-no-ron
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.adam-toi-uu-hoa]
requires: [ai.dong-luong-momentum]
concepts: [ai.adam-toi-uu-hoa]
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
Bài trước cho động lượng thắng SGD gấp `6` lần — nhưng chỉ ở đúng cặp
`lr=0.025, beta=0.7` đã dò kỹ. Tăng `lr` lên `0.1` (gấp `4` lần) thôi, điều
gì xảy ra?
::::

::::explain{#adam_hai_trung_binh_dong}
Bài `dong-luong-momentum` đã đo: động lượng CẦN `lr` được chọn đúng một
khoảng hẹp — quá khoảng đó, nó phân kỳ hoàn toàn (không phải chậm lại, mà
loss bùng nổ ra vô cùng). Đây là chi phí của việc chỉ giữ MỘT trung bình
động (`v`, bậc một, của chính gradient).

**Adam** giữ THÊM một trung bình động thứ hai — bình phương của gradient —
rồi dùng nó để tự động CO NHỎ bước cập nhật ở những hướng có gradient LỚN
(ồn ào), và giữ bước LỚN hơn ở những hướng có gradient nhỏ (ổn định):

> `m ← β₁·m + (1−β₁)·grad` (bậc một — giống hệt `v` của động lượng)
>
> `v ← β₂·v + (1−β₂)·grad²` (bậc hai — trung bình động của BÌNH PHƯƠNG
> gradient, luôn không âm)

`m`, `v` đều khởi tạo `0`, nên ở những bước ĐẦU TIÊN (`t` nhỏ), cả hai bị
lệch về phía `0` (thiên kiến khởi tạo). **Hiệu chỉnh lệch** (bias
correction) sửa đúng thiên kiến đó, chia cho `1 − βᵗ` (càng nhỏ khi `t`
càng nhỏ, đẩy `m`, `v` lên đúng tỷ lệ mất mát vì khởi tạo `0`):

> `m̂ = m / (1 − β₁ᵗ)`, `v̂ = v / (1 − β₂ᵗ)`

Cập nhật cuối cùng chia cho CĂN BẬC HAI của `v̂` (cộng thêm `ε` — epsilon,
một số cực nhỏ như `1e-8`, để không bao giờ chia cho đúng `0`):

> `param ← param − lr · m̂ / (√v̂ + ε)`

Chia cho `√v̂` chính là điểm khác biệt cốt lõi với động lượng: một tham số
có gradient LUÔN LỚN (dốc mạnh) sẽ có `v̂` lớn, làm mẫu số lớn, bước cập
nhật tự động CO LẠI; một tham số có gradient LUÔN NHỎ (thoải) sẽ có `v̂`
nhỏ, bước cập nhật tự động LỚN hơn tương đối. Việc CO GIÃN này diễn ra
RIÊNG cho từng tham số, không cần chọn `lr` khác nhau bằng tay như động
lượng phải làm.

`m`, `v`, `m̂`, `v̂` đều là số Python trần — như `v` của động lượng, chúng
không phải `Value`, chỉ là bộ nhớ phụ trợ quyết định bước cập nhật.
::::

::::example{#adam_ben_vung_hon_momentum}
Cùng bài toán khe hẹp `L(w1, w2) = w1² + 200·w2²` của bài trước — nhưng
lần này thử CÙNG một `lr = 0.1` cho cả Adam (`β₁=0.9, β₂=0.999` — hai giá
trị MẶC ĐỊNH chuẩn, không dò riêng cho bài toán này) và Động lượng
(`beta=0.7` — đúng giá trị đã tốt ở `lr=0.025` của bài trước):

```python title=readonly
class Value:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
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
        self.grad = 1.0
        for node in reversed(topo):
            node._backward()


K = 200.0
BETA1, BETA2, EPS = 0.9, 0.999, 1e-8

def toi_uu_adam(so_buoc, lr, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    m1, v1, m2, v2 = 0.0, 0.0, 0.0, 0.0
    ls = []
    for t in range(1, so_buoc + 1):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        g1, g2 = w1.grad, w2.grad

        m1 = BETA1 * m1 + (1 - BETA1) * g1
        v1 = BETA2 * v1 + (1 - BETA2) * g1 * g1
        m1_hat = m1 / (1 - BETA1 ** t)
        v1_hat = v1 / (1 - BETA2 ** t)
        w1.data -= lr * m1_hat / (v1_hat ** 0.5 + EPS)

        m2 = BETA1 * m2 + (1 - BETA1) * g2
        v2 = BETA2 * v2 + (1 - BETA2) * g2 * g2
        m2_hat = m2 / (1 - BETA1 ** t)
        v2_hat = v2 / (1 - BETA2 ** t)
        w2.data -= lr * m2_hat / (v2_hat ** 0.5 + EPS)

        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

def toi_uu_momentum_cung_lr(so_buoc, lr, beta):
    w1, w2 = Value(1.0), Value(1.0)
    v1, v2 = 0.0, 0.0
    for _ in range(so_buoc):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        v1 = beta * v1 + (1 - beta) * w1.grad
        v2 = beta * v2 + (1 - beta) * w2.grad
        w1.data -= lr * v1
        w2.data -= lr * v2
        w1.grad = 0.0
        w2.grad = 0.0
        gia_tri = w1.data ** 2 + K * w2.data ** 2
        if gia_tri != gia_tri or gia_tri > 1e6:
            return True    # PHAN KY
    return False

def epoch_hoi_tu(ls, nguong):
    for i in range(len(ls)):
        if all(v < nguong for v in ls[i:]):
            return i + 1
    return None

ls_adam = toi_uu_adam(150, 0.1)
epoch_adam = epoch_hoi_tu(ls_adam, 0.01)
mom_phan_ky = toi_uu_momentum_cung_lr(50, 0.1, 0.7)

print(round(ls_adam[0], 4))
print(epoch_adam)
print(round(ls_adam[epoch_adam - 1], 4))
print(mom_phan_ky)
```

```text title=readonly
162.81
92
0.008
True
```

Cùng `lr = 0.1`: Động lượng (giá trị `beta` đã TỐT ở `lr` khác) PHÂN KỲ
hoàn toàn (`True`). Adam (không dò riêng cho bài toán này, dùng hai giá
trị mặc định chuẩn) hội tụ bền vững, xuống dưới `0.01` sau `92` bước và ở
lại đó. Adam không nhanh hơn động lượng đã-được-dò-kỹ của bài trước (`42`
bước) — nhưng nó KHÔNG CẦN dò: `lr = 0.1` chỉ là một trong một khoảng RẤT
RỘNG giá trị mà Adam vẫn hội tụ ổn định, trong khi động lượng có một
khoảng "an toàn" hẹp hơn nhiều.
::::

::::predict{#doan_bo_hieu_chinh_lech commitOnce}
`ls_adam[0]` (loss sau bước ĐẦU TIÊN, `t=1`) ở trên là `162.81`. Hệ số hiệu
chỉnh lệch ở bước `t=1` là `1 − β₂¹ = 1 − 0,999 = 0,001` — một số CỰC NHỎ,
nên chia `v` cho nó (`v̂ = v / 0.001`) làm `v̂` LỚN HƠN `v` gốc tới `1000`
lần.

**Trước khi chạy thử**, bạn đoán: nếu bỏ hẳn bước hiệu chỉnh lệch (dùng
thẳng `m`, `v` thay vì `m̂`, `v̂` trong công thức cập nhật, MỌI thứ khác giữ
nguyên), `loss` sau bước ĐẦU TIÊN sẽ ra sao so với `162.81`?

:::opt{correct}
Khác hẳn — cụ thể là `93.98` (đã tự kiểm chứng bằng Python thật) — vì
thiếu hệ số chia cho `1 − βᵗ` làm `m`, `v` ở bước đầu bị lệch RẤT NẶNG về
phía `0` (do khởi tạo `0`), khiến bước cập nhật đầu tiên đi theo một hướng
và độ lớn khác hẳn phiên bản CÓ hiệu chỉnh — dù cả hai phiên bản cuối cùng
vẫn cùng hội tụ về sau
:::

:::opt
Vẫn `162.81`, không đổi — vì hiệu chỉnh lệch chỉ là một chi tiết kỹ thuật
phụ, không ảnh hưởng tới bước cập nhật cụ thể nào
::why
Gần đúng ở việc "hiệu chỉnh lệch" nghe như một điều chỉnh NHỎ so với hai
công thức trung bình động chính — dễ đoán nó không quan trọng bằng.

Chỗ lệch: hệ số hiệu chỉnh ở bước `t=1` (`1 − 0,001 = 0,999` cho `β₁`,
`1 − 0,999 = 0,001` cho `β₂`) là một trong những hệ số THAY ĐỔI MẠNH NHẤT
trong toàn bộ quá trình huấn luyện — càng về sau, `βᵗ` càng gần `0`, hệ số
càng gần `1` (gần như không đổi gì); nhưng CÀNG SỚM, hệ số càng khác `1`
CÀNG NHIỀU. Bước đầu tiên chính là lúc hiệu chỉnh lệch có tác động LỚN
NHẤT, không phải nhỏ nhất.
::
:::

:::opt
Sẽ nhỏ hơn `162.81` — bỏ hiệu chỉnh lệch làm bước cập nhật CẨN THẬN hơn (đi
chậm hơn), nên loss giảm ít hơn, tức bước đầu loss vẫn cao nhưng theo chiều
ngược lại đã đoán
::why
Gần đúng ở việc bạn nhận ra bỏ hiệu chỉnh SẼ đổi độ lớn bước đi — quan sát
về việc CÓ thay đổi là đúng hướng.

Chỗ lệch: chiều thay đổi không cố định theo một hướng đơn giản như "luôn
chậm hơn" — nó phụ thuộc cả `m` LẪN `v` cùng bị lệch, và hai lệch đó không
triệt tiêu nhau gọn gàng. Số liệu THẬT (`93.98`, thấp hơn `162.81`, nghĩa
là bước ĐẦU của bản KHÔNG hiệu chỉnh giảm loss NHIỀU hơn, không phải ít
hơn) cho thấy trực giác "cẩn thận hơn nên đi chậm hơn" không khớp với cách
hai thiên kiến của `m` và `v` tương tác trong công thức chia `m̂/√v̂` cụ
thể này.
::
:::
::::

::::code{#viet_adam_optimizer}
Hoàn thiện bốn dòng của Adam cho `w1`: cập nhật `v1` (bậc hai, dùng
`g1 * g1`), hai hệ số hiệu chỉnh lệch `m1_hat`, `v1_hat`, và bước cập nhật
cuối cùng (chia cho căn bậc hai của `v1_hat`, cộng `EPS`).

```python title=starter
class Value:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
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
        self.grad = 1.0
        for node in reversed(topo):
            node._backward()


K = 200.0
BETA1, BETA2, EPS = 0.9, 0.999, 1e-8

def toi_uu_adam(so_buoc, lr, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    m1, v1, m2, v2 = 0.0, 0.0, 0.0, 0.0
    ls = []
    for t in range(1, so_buoc + 1):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        g1, g2 = w1.grad, w2.grad

        m1 = BETA1 * m1 + (1 - BETA1) * g1
        v1 = ___                              # BETA2 * v1 + (1 - BETA2) * g1 * g1
        m1_hat = ___                           # m1 / (1 - BETA1 ** t)
        v1_hat = ___                           # v1 / (1 - BETA2 ** t)
        w1.data -= ___                         # lr * m1_hat / (v1_hat ** 0.5 + EPS)

        m2 = BETA1 * m2 + (1 - BETA1) * g2
        v2 = BETA2 * v2 + (1 - BETA2) * g2 * g2
        m2_hat = m2 / (1 - BETA1 ** t)
        v2_hat = v2 / (1 - BETA2 ** t)
        w2.data -= lr * m2_hat / (v2_hat ** 0.5 + EPS)

        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

def toi_uu_momentum_cung_lr(so_buoc, lr, beta):
    w1, w2 = Value(1.0), Value(1.0)
    v1, v2 = 0.0, 0.0
    for _ in range(so_buoc):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        v1 = beta * v1 + (1 - beta) * w1.grad
        v2 = beta * v2 + (1 - beta) * w2.grad
        w1.data -= lr * v1
        w2.data -= lr * v2
        w1.grad = 0.0
        w2.grad = 0.0
        gia_tri = w1.data ** 2 + K * w2.data ** 2
        if gia_tri != gia_tri or gia_tri > 1e6:
            return True
    return False

def epoch_hoi_tu(ls, nguong):
    for i in range(len(ls)):
        if all(v < nguong for v in ls[i:]):
            return i + 1
    return None

ls_adam = toi_uu_adam(150, 0.1)
epoch_adam = epoch_hoi_tu(ls_adam, 0.01)
mom_phan_ky = toi_uu_momentum_cung_lr(50, 0.1, 0.7)

print(round(ls_adam[0], 4))
print(epoch_adam)
print(round(ls_adam[epoch_adam - 1], 4))
print(mom_phan_ky)
```

```python title=solution
class Value:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
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
        self.grad = 1.0
        for node in reversed(topo):
            node._backward()


K = 200.0
BETA1, BETA2, EPS = 0.9, 0.999, 1e-8

def toi_uu_adam(so_buoc, lr, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    m1, v1, m2, v2 = 0.0, 0.0, 0.0, 0.0
    ls = []
    for t in range(1, so_buoc + 1):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        g1, g2 = w1.grad, w2.grad

        m1 = BETA1 * m1 + (1 - BETA1) * g1
        v1 = BETA2 * v1 + (1 - BETA2) * g1 * g1
        m1_hat = m1 / (1 - BETA1 ** t)
        v1_hat = v1 / (1 - BETA2 ** t)
        w1.data -= lr * m1_hat / (v1_hat ** 0.5 + EPS)

        m2 = BETA1 * m2 + (1 - BETA1) * g2
        v2 = BETA2 * v2 + (1 - BETA2) * g2 * g2
        m2_hat = m2 / (1 - BETA1 ** t)
        v2_hat = v2 / (1 - BETA2 ** t)
        w2.data -= lr * m2_hat / (v2_hat ** 0.5 + EPS)

        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

def toi_uu_momentum_cung_lr(so_buoc, lr, beta):
    w1, w2 = Value(1.0), Value(1.0)
    v1, v2 = 0.0, 0.0
    for _ in range(so_buoc):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        v1 = beta * v1 + (1 - beta) * w1.grad
        v2 = beta * v2 + (1 - beta) * w2.grad
        w1.data -= lr * v1
        w2.data -= lr * v2
        w1.grad = 0.0
        w2.grad = 0.0
        gia_tri = w1.data ** 2 + K * w2.data ** 2
        if gia_tri != gia_tri or gia_tri > 1e6:
            return True
    return False

def epoch_hoi_tu(ls, nguong):
    for i in range(len(ls)):
        if all(v < nguong for v in ls[i:]):
            return i + 1
    return None

ls_adam = toi_uu_adam(150, 0.1)
epoch_adam = epoch_hoi_tu(ls_adam, 0.01)
mom_phan_ky = toi_uu_momentum_cung_lr(50, 0.1, 0.7)

print(round(ls_adam[0], 4))
print(epoch_adam)
print(round(ls_adam[epoch_adam - 1], 4))
print(mom_phan_ky)
```

```python title=test
assert round(ls_adam[0], 4) == 162.81, f"loss buoc dau tien sai -- dang ra {round(ls_adam[0], 4)}"
assert epoch_adam == 92, f"epoch_adam sai -- dang ra {epoch_adam}"
assert round(ls_adam[epoch_adam - 1], 4) == 0.008, f"loss tai epoch hoi tu sai -- dang ra {round(ls_adam[epoch_adam - 1], 4)}"
assert mom_phan_ky == True, "dong luong (beta=0.7) PHAI phan ky tai lr=0.1 -- day chinh la doi lap voi do ben vung cua Adam ma bai nay muon chi ra"

# rieng doi chieu: BO hieu chinh lech (dung thang m1, v1 -- khong chia cho
# 1-beta**t) phai cho loss buoc dau KHAC han 162.81 -- da tu kiem chung
# bang Python that ca hai chieu truoc khi viet assertion nay.
def toi_uu_adam_khong_bc(so_buoc, lr, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    m1, v1, m2, v2 = 0.0, 0.0, 0.0, 0.0
    ls = []
    for t in range(1, so_buoc + 1):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        g1, g2 = w1.grad, w2.grad
        m1 = BETA1 * m1 + (1 - BETA1) * g1
        v1 = BETA2 * v1 + (1 - BETA2) * g1 * g1
        w1.data -= lr * m1 / (v1 ** 0.5 + EPS)
        m2 = BETA1 * m2 + (1 - BETA1) * g2
        v2 = BETA2 * v2 + (1 - BETA2) * g2 * g2
        w2.data -= lr * m2 / (v2 ** 0.5 + EPS)
        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

ls_khong_bc = toi_uu_adam_khong_bc(1, 0.1)
assert round(ls_khong_bc[0], 4) != round(ls_adam[0], 4), "loss buoc dau CO va KHONG hieu chinh lech phai KHAC nhau -- neu giong nhau, m1_hat/v1_hat dang khong duoc dung dung cho trong buoc cap nhat"
assert round(ls_khong_bc[0], 4) == 93.9764, f"loss buoc dau KHONG hieu chinh lech phai la 93.9764 -- dang ra {round(ls_khong_bc[0], 4)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống, cùng cấu trúc với bốn dòng ĐÃ VIẾT SẴN cho `w2` ngay bên dưới — soi sang là thấy công thức. `v1`: trung bình động BÌNH PHƯƠNG gradient — `BETA2 * v1 + (1 - BETA2) * g1 * g1`. `m1_hat`: chia `m1` cho hệ số hiệu chỉnh — `m1 / (1 - BETA1 ** t)`. `v1_hat`: tương tự với `BETA2` — `v1 / (1 - BETA2 ** t)`. Bước cập nhật cuối: `lr * m1_hat / (v1_hat ** 0.5 + EPS)` — chia cho CĂN BẬC HAI của `v1_hat`, cộng `EPS` để không chia cho `0`.
- kind: strategy
  body: 'v1: `BETA2 * v1 + (1 - BETA2) * g1 * g1`. m1_hat: `m1 / (1 - BETA1 ** t)`. v1_hat: `v1 / (1 - BETA2 ** t)`. w1.data -=: `lr * m1_hat / (v1_hat ** 0.5 + EPS)`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `BETA2 * v1 + (1 - BETA2) * g1 * g1`, `m1 / (1 - BETA1 ** t)`, `v1 / (1 - BETA2 ** t)`, và `lr * m1_hat / (v1_hat ** 0.5 + EPS)`.'
:::

:::validate
- tier: run
  timeoutMs: 12000
- tier: static
  onFail: v1 phai la trung binh dong cua BINH PHUONG gradient (g1*g1, khong phai g1); m1_hat/v1_hat phai CHIA cho he so hieu chinh lech (1 - BETA**t), khong duoc bo qua; buoc cap nhat cuoi PHAI dung m1_hat VA v1_hat (khong duoc dung thang m1/v1, thieu hieu chinh lech se doi han quy dao nhung buoc dau)
  requireAst:
  - kind: uses-name, target: BETA1, min: 6
  - kind: uses-name, target: BETA2, min: 6
  - kind: uses-name, target: EPS, min: 2
  - kind: uses-name, target: m1_hat, min: 1
  - kind: uses-name, target: v1_hat, min: 1
  - kind: uses-operator, target: "**", min: 10
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca sau luat qua sach (BETA1=6, BETA2=6: moi bien xuat
  # hien dung 3 lan trong khoi w1 [m1 cap nhat, m1_hat, v1_hat khong -- that
  # ra BETA1 xuat hien trong m1 va m1_hat, BETA2 trong v1 va v1_hat] cong 3
  # lan trong khoi w2 tuong tu -- dem duoc dung 6 moi bien; EPS=2: mot lan
  # moi khoi w1/w2; m1_hat=1: doc trong buoc cap nhat w1.data; v1_hat=1:
  # tuong tu; "**"=10: BETA1**t, BETA2**t [x2, w1+w2], g1*g1/g2*g2 khong
  # tinh vi la "*" khong phai "**", v1_hat**0.5/v2_hat**0.5, cong 2*(v1_hat
  # trong test rieng) -- solution chinh dem duoc 10 tu 4 cho **t va 2 cho
  # **0.5 nhan hai khoi w1/w2 = 4+2=6, CONG them cac ** trong ham
  # toi_uu_momentum_cung_lr o duoi khong co -- thuc te xac nhan lai bang
  # kiemAst that o buoc tu-verify cuoi bai).
  #
  # Cheat "v1 = BETA2*v1 + (1-BETA2)*g1" (thieu binh phuong, dung thang g1)
  # se cho loss KHAC han 162.81/92/0.008 -- bi chan boi tests, doc lap voi
  # static. Cheat "bo hieu chinh lech" (dung thang m1, v1) lam "m1_hat" va
  # "v1_hat" ve 0 -- bi chan boi static; da tu kiem chung: cheat nay cho
  # ls_adam[0]=93.9764, KHAC han 162.81 -- bi chan doc lap boi tests/output.
- tier: tests
  timeoutMs: 12000
- tier: output
  match: regex
  expect: "^162\\.81\\n92\\n0\\.008\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng `lr=0.1`: động lượng bùng nổ, Adam hội tụ về `0.008` sau `92` bước.
Không phải vì Adam luôn nhanh nhất — mà vì nó không cần dò `lr` cẩn thận
như động lượng.
::::

::::reflect{#nghi-lai}
SGD, động lượng, Adam — ba cách cập nhật tham số, mỗi cách một đánh đổi:
SGD đơn giản nhưng chậm, động lượng nhanh nhưng cần dò kỹ, Adam bền vững
trên một khoảng `lr` rộng hơn hẳn. Tất cả đều dùng CHUNG một thứ: gradient
đã tính được từ `loss.backward()`.

Nhưng có nhiều tham số hơn, huấn luyện lâu hơn, không phải lúc nào cũng
tốt hơn — một mạng có thể học THUỘC LÒNG dữ liệu huấn luyện thay vì học
QUY LUẬT đứng sau nó. Hai bài cuối trước BOSS sẽ đo đúng hiện tượng đó
trên một mạng nơ-ron thật.
::::

::::checkpoint{mastery=0.85}
::::
