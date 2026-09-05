---
id: tri-tue-nhan-tao.huan-luyen-mang-no-ron.ham-mat-mat-qua-value
title: "Hàm mất mát qua Value"
summary: "MSE (hồi quy) và cross-entropy+softmax (phân loại) tính QUA Value — không công thức numpy đóng. MSE trên ba điểm cho loss=0.1267, gradient dự đoán khớp CHÍNH XÁC công thức chuẩn 2(pred-target)/n. Cross-entropy bọc log-sum-exp ổn định (tái dùng q8.1c, thêm hai method Value.exp()/Value.log() theo đúng khuôn _backward đã học) cho loss=0.4644 trên logits=[1.0,2.0,0.5], gradient autograd khớp khít công thức chuẩn softmax-trừ-one-hot. Trên logits LỚN [1000,1001,999], phiên bản không trừ max làm math.exp báo OverflowError ngay lập tức; phiên bản có log-sum-exp cho loss=0.4076, không lỗi."
locale: vi
track: tri-tue-nhan-tao
module: huan-luyen-mang-no-ron
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.ham-mat-mat-qua-value]
requires: [ai.neuron-tu-autograd]
concepts: [ai.ham-mat-mat-qua-value]
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
`Layer`/`MLP` giờ có gradient tới từng tham số — nhưng gradient của CÁI GÌ?
Bài trước gọi `out[0].backward()` thẳng lên đầu ra của mạng. Huấn luyện
thật cần gọi `backward()` lên một con số khác hẳn: hàm mất mát.
::::

::::explain{#mat_mat_qua_value}
`Value` (từ `q8.2b`) có đúng sáu phép toán: `+`, `-`, `*`, `**`, `tanh`,
`relu`. Hàm mất mát cho **hồi quy** — MSE, `mean((pred − target)²)` — dùng
được NGUYÊN VẸN sáu phép đó: trừ (`-`), bình phương (`**2`), cộng dồn
(`+`), và chia cho `n` chỉ là NHÂN với hằng số `1/n` (một số Python trần,
không phải phép chia `Value/Value`).

Hàm mất mát cho **phân loại** — softmax + cross-entropy — cần THÊM hai phép
mà `Value` chưa có: `exp` và `log`. Thêm chúng không phải viết lại động cơ
— chỉ là ráp thêm hai method theo ĐÚNG khuôn mẫu `_backward` đã học từ tám
bài của `q8.2b`:

> `Value.exp()`: `out = Value(e^self.data)`. Đạo hàm của `e^x` chính là
> `e^x`, nên `_backward` cộng dồn `e * out.grad` vào `self.grad` — `e` là
> giá trị `Value` VỪA tính ra, tái dùng lại, không tính `exp` hai lần.
>
> `Value.log()`: `out = Value(log(self.data))`. Đạo hàm của `log(x)` là
> `1/x`, nên `_backward` cộng dồn `(1.0 / self.data) * out.grad`.

Với hai phép mới đó, softmax + cross-entropy bọc lại ĐÚNG chiêu ổn định số
học đã học ở `on-dinh-so-hoc` (`q8.1c`) — trừ giá trị lớn nhất `m` (một số
Python trần, không lấy đạo hàm qua bước chọn max này) khỏi mọi logit trước
khi `exp`, cộng dồn `Value` để lấy tổng, rồi:

> `L = log(Σⱼ e^(zⱼ − m)) − (zᵧ − m)`

— với `y` là chỉ số lớp đúng. Công thức này CHÍNH XÁC bằng
`−log(softmax(z)ᵧ)` (cross-entropy chuẩn) khi khai triển đại số, nhưng
không bao giờ gọi `exp` trên một số lớn — vì sau khi trừ `m`, số lớn nhất
còn lại luôn là `0`.

Điểm khác biệt DUY NHẤT với cách viết trong `on-dinh-so-hoc` (nơi mọi phép
tính là `numpy` thuần, không có gradient): ở đây MỌI bước — trừ, `exp`,
cộng, `log`, trừ lần nữa — đều đi qua `Value`, nên gọi `.backward()` lên
`L` lan được gradient tới TỪNG logit `z` — chính là tới đầu ra của tầng
cuối một `MLP` phân loại.
::::

::::example{#mse_va_cross_entropy_that}
**MSE** trên ba dự đoán, so với ba giá trị thật:

```python title=readonly
import math

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

    def __sub__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
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

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += (other * self.data ** (other - 1)) * out.grad
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


def mse_qua_value(preds, targets):
    n = len(preds)
    tong = Value(0.0)
    for p, t in zip(preds, targets):
        err = p - t
        tong = tong + err ** 2
    return tong * (1.0 / n)


preds = [Value(2.3), Value(-0.5), Value(1.8)]
targets = [2.0, 0.0, 2.0]
mse = mse_qua_value(preds, targets)
mse.backward()

print(round(mse.data, 4))
print([round(p.grad, 4) for p in preds])
```

```text title=readonly
0.1267
[0.2, -0.3333, -0.1333]
```

Gradient `[0.2, -0.3333, -0.1333]` khớp CHÍNH XÁC công thức chuẩn của MSE
(`∂L/∂predᵢ = 2(predᵢ − targetᵢ)/n`) — không phải trùng hợp, vì `err ** 2`
và `tong * (1.0/n)` chỉ là `Value.__pow__` và `Value.__mul__` áp đúng công
thức đó, autograd không "biết" công thức MSE, nó chỉ lan chain rule qua
từng phép toán đã viết.

**Cross-entropy + softmax** trên ba logit, nhãn thật là lớp `1`:

```python title=readonly
import math

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

    def __sub__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
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


def exp_v(self):
    e = math.exp(self.data)
    out = Value(e, (self,), 'exp')
    def _backward():
        self.grad += e * out.grad
    out._backward = _backward
    return out

def log_v(self):
    l = math.log(self.data)
    out = Value(l, (self,), 'log')
    def _backward():
        self.grad += (1.0 / self.data) * out.grad
    out._backward = _backward
    return out

Value.exp = exp_v
Value.log = log_v

def cross_entropy_qua_value(logits, y_true_idx):
    m = max(v.data for v in logits)          # so PYTHON tran, khong lay dao ham qua day
    shifted = [v - m for v in logits]
    exps = [v.exp() for v in shifted]
    tong_exp = exps[0]
    for e in exps[1:]:
        tong_exp = tong_exp + e
    log_tong = tong_exp.log()
    return log_tong - shifted[y_true_idx]


logits = [Value(1.0), Value(2.0), Value(0.5)]
loss = cross_entropy_qua_value(logits, 1)
loss.backward()

print(round(loss.data, 4))
print([round(v.grad, 4) for v in logits])
```

```text title=readonly
0.4644
[0.2312, -0.3715, 0.1402]
```

Gradient `[0.2312, -0.3715, 0.1402]` khớp CHÍNH XÁC công thức chuẩn của
softmax+cross-entropy (`∂L/∂zᵢ = softmaxᵢ − 1{i=y}`) — tính riêng bằng
`math.exp`/`math.log` thuần, không đụng một dòng nào của `Value`, để đối
chiếu. Với logit `1000, 1001, 999` (đủ lớn để `math.exp` tràn số), phiên
bản KHÔNG trừ `m` báo `OverflowError` ngay khi gọi `exp` — dừng chương
trình hoàn toàn; phiên bản CÓ trừ `m` (đúng hàm `cross_entropy_qua_value`
ở trên) cho `loss = 0.4076`, không lỗi gì cả — cùng một công thức toán
học, chỉ khác cách tính có tràn số hay không, đúng bài học của
`on-dinh-so-hoc`.
::::

::::predict{#doan_gradient_tai_max commitOnce}
Trong `cross_entropy_qua_value`, `m = max(...)` là một số PYTHON TRẦN — nó
không đi qua bất kỳ operator nào của `Value`. Với `logits = [1.0, 2.0,
0.5]`, `m = 2.0`, đúng bằng giá trị của `logits[1]` — nên
`shifted[1] = logits[1] - m` có GIÁ TRỊ đúng bằng `0.0`.

**Trước khi chạy thử**, bạn đoán: `logits[1].grad` sau `backward()` — phần
tử VỪA bị trừ về đúng `0.0` — sẽ là bao nhiêu?

:::opt{correct}
Khác `0` (cụ thể là `-0.3715`, giống hệt kết quả đã thấy) — `shifted[1] =
logits[1] - m` vẫn là một phép trừ `Value` bình thường, với `logits[1]`
là node cha THẬT SỰ của nó; đạo hàm cục bộ của phép trừ (`∂(a-b)/∂a = 1`)
không phụ thuộc GIÁ TRỊ kết quả — dù kết quả bằng `0.0` hay bất kỳ số nào
khác, gradient vẫn lan ngược bình thường qua `logits[1]`
:::

:::opt
Đúng `0.0` — vì bất kỳ biểu thức nào có giá trị bằng `0` thì gradient đi
qua nó cũng bằng `0`
::why
Gần đúng ở việc bạn nhớ đúng MỘT trường hợp thật — `relu` (bài
`lop-value-mu-tanh-relu`, q8.2b) đúng là có tính chất "giá trị `0` thì
gradient cục bộ cũng `0`", vì cả hai đều dùng chung điều kiện
`self.data ≤ 0`.

Chỗ lệch: đó là tính chất RIÊNG của `relu`, không phải quy luật chung cho
mọi phép toán. Phép TRỪ (`__sub__`) có đạo hàm cục bộ CỐ ĐỊNH — luôn là `1`
với số bị trừ, luôn là `-1` với số trừ — không phụ thuộc giá trị kết quả
bằng bao nhiêu. `shifted[1]` có giá trị `0.0` chỉ là một sự trùng hợp SỐ
HỌC (vì `m` được chọn đúng bằng `logits[1]`), không làm đạo hàm của phép
trừ đổi thành `0`.
::
:::

:::opt
Không xác định được — vì `m` là số Python trần (không phải `Value`), phép
trừ `v - m` không tạo được liên kết đồ thị nào với `v`
::why
Gần đúng ở việc để ý `m` không phải một `Value` — quan sát đó đúng, và nó
QUAN TRỌNG (đúng lý do bài `neuron-tu-autograd` cảnh báo: lỡ dùng số trần ở
SAI chỗ làm đứt đồ thị).

Chỗ lệch: `Value.__sub__` LUÔN wrap `other` thành `Value` mới nếu nó chưa
phải (`other = other if isinstance(other, Value) else Value(other)`) —
nhưng `self` (ở đây là `v`, tức `logits[1]`) không cần wrap, nó ĐÃ là một
`Value` thật, và nó vẫn là node cha thật sự của kết quả. Số trần chỉ "mất
liên kết" khi nó đứng ở vị trí sẽ được dùng làm THAM SỐ cần học (như `W`,
`b` ở bài trước) — ở đây `m` chỉ là một hằng số ổn định số học, không cần
liên kết, và việc nó không có liên kết không hề ảnh hưởng tới liên kết của
`v`.
::
:::
::::

::::code{#viet_mse_va_cross_entropy}
Hoàn thiện `mse_qua_value` (hiệu số) và `cross_entropy_qua_value` (cộng dồn
tổng `exp`, và công thức log-sum-exp cuối cùng).

```python title=starter
import math

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

    def __sub__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
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

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += (other * self.data ** (other - 1)) * out.grad
        out._backward = _backward
        return out

    def exp(self):
        e = math.exp(self.data)
        out = Value(e, (self,), 'exp')
        def _backward():
            self.grad += e * out.grad
        out._backward = _backward
        return out

    def log(self):
        assert self.data > 0
        l = math.log(self.data)
        out = Value(l, (self,), 'log')
        def _backward():
            self.grad += (1.0 / self.data) * out.grad
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


def mse_qua_value(preds, targets):
    n = len(preds)
    tong = Value(0.0)
    for p, t in zip(preds, targets):
        err = ___                        # p - t
        tong = tong + err ** 2
    return tong * (1.0 / n)


def cross_entropy_qua_value(logits, y_true_idx):
    m = max(v.data for v in logits)
    shifted = [v - m for v in logits]
    exps = [v.exp() for v in shifted]
    tong_exp = exps[0]
    for e in exps[1:]:
        tong_exp = ___                   # tong_exp + e
    log_tong = tong_exp.log()
    return ___                            # log_tong - shifted[y_true_idx]


preds = [Value(2.3), Value(-0.5), Value(1.8)]
targets = [2.0, 0.0, 2.0]
mse = mse_qua_value(preds, targets)
mse.backward()

logits = [Value(1.0), Value(2.0), Value(0.5)]
loss = cross_entropy_qua_value(logits, 1)
loss.backward()

logits_lon = [Value(1000.0), Value(1001.0), Value(999.0)]
loss_lon = cross_entropy_qua_value(logits_lon, 1)

print(round(mse.data, 4))
print([round(p.grad, 4) for p in preds])
print(round(loss.data, 4))
print([round(l.grad, 4) for l in logits])
print(round(loss_lon.data, 4))
```

```python title=solution
import math

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

    def __sub__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data - other.data, (self, other), '-')
        def _backward():
            self.grad += out.grad
            other.grad += -out.grad
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

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += (other * self.data ** (other - 1)) * out.grad
        out._backward = _backward
        return out

    def exp(self):
        e = math.exp(self.data)
        out = Value(e, (self,), 'exp')
        def _backward():
            self.grad += e * out.grad
        out._backward = _backward
        return out

    def log(self):
        assert self.data > 0
        l = math.log(self.data)
        out = Value(l, (self,), 'log')
        def _backward():
            self.grad += (1.0 / self.data) * out.grad
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


def mse_qua_value(preds, targets):
    n = len(preds)
    tong = Value(0.0)
    for p, t in zip(preds, targets):
        err = p - t
        tong = tong + err ** 2
    return tong * (1.0 / n)


def cross_entropy_qua_value(logits, y_true_idx):
    m = max(v.data for v in logits)
    shifted = [v - m for v in logits]
    exps = [v.exp() for v in shifted]
    tong_exp = exps[0]
    for e in exps[1:]:
        tong_exp = tong_exp + e
    log_tong = tong_exp.log()
    return log_tong - shifted[y_true_idx]


preds = [Value(2.3), Value(-0.5), Value(1.8)]
targets = [2.0, 0.0, 2.0]
mse = mse_qua_value(preds, targets)
mse.backward()

logits = [Value(1.0), Value(2.0), Value(0.5)]
loss = cross_entropy_qua_value(logits, 1)
loss.backward()

logits_lon = [Value(1000.0), Value(1001.0), Value(999.0)]
loss_lon = cross_entropy_qua_value(logits_lon, 1)

print(round(mse.data, 4))
print([round(p.grad, 4) for p in preds])
print(round(loss.data, 4))
print([round(l.grad, 4) for l in logits])
print(round(loss_lon.data, 4))
```

```python title=test
assert round(mse.data, 4) == 0.1267, f"mse.data sai -- dang ra {round(mse.data, 4)}"
assert [round(p.grad, 4) for p in preds] == [0.2, -0.3333, -0.1333], f"grad cua preds sai -- dang ra {[round(p.grad, 4) for p in preds]}"

assert round(loss.data, 4) == 0.4644, f"loss.data sai -- dang ra {round(loss.data, 4)}"
assert [round(v.grad, 4) for v in logits] == [0.2312, -0.3715, 0.1402], f"grad cua logits sai -- dang ra {[round(v.grad, 4) for v in logits]}"

# rieng doi chieu gradient tu dong voi cong thuc chuan softmax-tru-one-hot,
# TU TAY tinh lai bang math thuan (khong dung lai mot dong nao cua Value)
mu = max(l.data for l in logits)
mu_e = [math.exp(l.data - mu) for l in logits]
tong_mu = sum(mu_e)
softmax_chuan = [v / tong_mu for v in mu_e]
grad_chuan = [softmax_chuan[i] - (1.0 if i == 1 else 0.0) for i in range(3)]
assert all(abs(logits[i].grad - grad_chuan[i]) < 1e-6 for i in range(3)), f"gradient autograd phai khop cong thuc chuan (softmax - one_hot) -- dang ra {[l.grad for l in logits]} vs chuan {grad_chuan}"

# gotcha on dinh so hoc: logits LON phai KHONG loi (da tu kiem chung: ban
# KHONG tru max bi math.exp bao OverflowError ngay lap tuc tren cung logits
# nay -- ham cross_entropy_qua_value o day PHAI di qua m=max(...) truoc khi
# goi .exp(), neu khong se crash ngay o dong nay, khong toi duoc dong test.
assert round(loss_lon.data, 4) == 0.4076, f"loss_lon.data sai -- dang ra {round(loss_lon.data, 4)}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. `mse_qua_value`, `err`: hiệu số dự đoán trừ mục tiêu — `p - t` (đúng thứ tự, `p` trước). `cross_entropy_qua_value`, `tong_exp`: cộng dồn từng `e` vào tổng đang có — `tong_exp + e`. Chỗ trống cuối, giá trị trả về: công thức log-sum-exp — `log_tong - shifted[y_true_idx]` (dùng logit ĐÃ TRỪ MAX của lớp đúng, không phải `logits[y_true_idx]` gốc).
- kind: strategy
  body: 'err: `p - t`. tong_exp: `tong_exp + e`. return: `log_tong - shifted[y_true_idx]`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `p - t`, `tong_exp + e`, và `log_tong - shifted[y_true_idx]`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: mse_qua_value phai tru p CHO t (khong duoc dao nguoc, se khong chay duoc vi Value khong co __rsub__); cross_entropy_qua_value phai CONG DON tong_exp qua vong lap (khong duoc bo qua bat ky exps[i] nao) va phai tra ve cong thuc dung shifted[y_true_idx] (logit DA TRU MAX cua lop dung), khong duoc dung logits[y_true_idx] goc
  requireAst:
  - kind: uses-call, target: exp, min: 2
  - kind: uses-call, target: log, min: 2
  - kind: uses-operator, target: "-", min: 5
  - kind: uses-name, target: shifted, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (exp=2: mot lan dinh nghia method,
  # mot lan goi trong list comp "v.exp() for v in shifted"; log=2: tuong
  # tu; "-"=5: p-t, out.grad tru trong AugAssign cua __sub__ [self.grad +=
  # -out.grad], v-m trong list comp, log_tong-shifted[...] o cho trong
  # cuoi; shifted=2: doc trong "for v in shifted" va doc trong
  # "shifted[y_true_idx]").
  # Cheat "return log_tong - logits[y_true_idx]" (dung logit GOC thay vi DA
  # TRU MAX) lam "shifted" tut xuong 1 -- bi chan; da tu kiem chung bang
  # Python that: cheat nay cho loss=-1.5356 (sai hoan toan so voi 0.4644) VA
  # loss_lon=-1000.5924 (sai hoan toan so voi 0.4076) -- bi chan CA static
  # LAN output/tests, doc lap voi nhau.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^0\\.1267\\n\\[0\\.2, -0\\.3333, -0\\.1333\\]\\n0\\.4644\\n\\[0\\.2312, -0\\.3715, 0\\.1402\\]\\n0\\.4076\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
MSE và cross-entropy, cả hai đều tính QUA `Value` — không công thức đóng.
Log-sum-exp giữ cho `exp` không bao giờ tràn số, kể cả trên logit `1000`.
::::

::::reflect{#nghi-lai}
`Value` giờ có tám phép toán — sáu cái cũ (`+`, `-`, `*`, `**`, `tanh`,
`relu`) cộng thêm `exp`, `log` vừa thêm ở bài này, đủ để tính MSE HOẶC
cross-entropy trên đầu ra của bất kỳ `MLP` nào đã ráp ở bài trước — và lan
gradient ngược thẳng vào từng logit, từng dự đoán.

Nhưng có gradient của hàm mất mát rồi vẫn chưa đủ để huấn luyện: gradient
đó phải được DÙNG để thực sự thay đổi `W`, `b` — và phải làm điều đó LẶP
ĐI LẶP LẠI, hàng chục hay hàng trăm lần, mỗi lần một chút. Bài sau ráp vòng
lặp đó — và chỉ ra một lỗi im lặng, nguy hiểm, dễ mắc phải nhất khi viết nó
lần đầu.
::::

::::checkpoint{mastery=0.85}
::::
