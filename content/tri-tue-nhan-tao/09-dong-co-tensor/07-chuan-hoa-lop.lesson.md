---
id: tri-tue-nhan-tao.dong-co-tensor.chuan-hoa-lop
title: "Layer normalization qua Tensor"
summary: "Tensor.layernorm(): chuẩn hoá MỖI HÀNG (mean=0, var=1) qua std=sqrt(var+eps) -- eps=1e-5 bảo vệ chia cho phương sai gần 0, TÁI DÙNG đúng ý tưởng epsilon của on-dinh-so-hoc (khác chỗ áp dụng, cùng lý do). Trên X=[[1,2,3,6],[5,1,3,7]]: xhat=[[-1,069043; -0,534522; 0; 1,603565],[0,447213; -1,341639; -0,447213; 1,341639]]. Backward (công thức chuẩn, không affine) khớp finite-difference, sai số dưới 1e-8. Hàng hằng số (var=0): không eps cho nan; có eps cho [0,0,0,0] hữu hạn -- đã tự kiểm chứng."
locale: vi
track: tri-tue-nhan-tao
module: dong-co-tensor
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.chuan-hoa-lop]
requires: [ai.softmax-qua-tensor]
concepts: [ai.chuan-hoa-lop]
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
`softmax` chuẩn hoá để TỔNG bằng `1`. Một phép chuẩn hoá khác, dùng ở gần
như MỌI tầng Transformer thật: đưa TRUNG BÌNH về `0`, PHƯƠNG SAI về `1` —
`layer normalization`.
::::

::::explain{#layer_norm}
**Layer normalization** chuẩn hoá MỖI HÀNG của một mảng độc lập (giống
`softmax` — trục cuối, mỗi hàng riêng), nhưng theo một tiêu chí khác hẳn
"tổng bằng 1": đưa TRUNG BÌNH của hàng về `0` và PHƯƠNG SAI về `1`.

> `μ = mean(x)` (trung bình của hàng), `σ² = mean((x − μ)²)` (phương sai —
> trung bình BÌNH PHƯƠNG độ lệch so với trung bình), `x̂ = (x − μ) / √(σ² +
> ε)`.

`ε` (epsilon) ở đây đóng ĐÚNG vai trò epsilon của `on-dinh-so-hoc` (T8.1c):
nếu MỌI phần tử của một hàng bằng nhau tuyệt đối (ví dụ `[4, 4, 4, 4]`),
`σ² = 0` — chia cho `√0 = 0` cho `nan`, một giá trị VÔ DỤNG. Cộng thêm một
số CỰC NHỎ (`ε = 1e-5`) trước khi lấy căn giữ mẫu số LUÔN DƯƠNG, tránh
`nan`, gần như không đổi kết quả khi `σ²` đã đủ lớn — cùng chiêu bài, khác
CHỖ ÁP DỤNG (denominator của layer norm, thay vì `log(p)` của cross-entropy
hay tổng `exp` của softmax).

Bài này CHƯA thêm tham số học được `γ` (scale) và `β` (shift) — nhiều cài
đặt layer norm thật nhân thêm `x̂ · γ + β` sau bước chuẩn hoá (hai tham số
HỌC ĐƯỢC, không phải hằng số) để mô hình tự điều chỉnh lại độ lớn/độ lệch
nếu cần. Quest này giới hạn ở phần CHUẨN HOÁ THUẦN (không affine) — đủ để
gradient-check và để dùng trong khối Transformer tối giản của quest sau.

Công thức đạo hàm cục bộ (không affine, `N` là số phần tử một hàng, `dy` là
gradient đến `x̂`):

> `dvar = Σⱼ dyⱼ·(xⱼ − μ) · (−½)·(σ²+ε)^(−3/2)`
>
> `dmu = Σⱼ dyⱼ·(−1/√(σ²+ε)) + dvar · mean(−2·(x − μ))`
>
> `dx = dy/√(σ²+ε) + dvar·2·(x−μ)/N + dmu/N`

Ba số hạng của `dx` ứng với ba con đường `x` ảnh hưởng tới `x̂`: TRỰC TIẾP
(qua tử số), qua `μ` (mọi phần tử của hàng đều góp phần vào trung bình), và
qua `σ²` (mọi phần tử cũng góp phần vào phương sai) — không chỉ một con
đường như `__mul__`.
::::

::::example{#layernorm_forward_backward_that}
`X` hai hàng (`[1,2,3,6]` và `[5,1,3,7]`, phương sai khác nhau) — forward,
rồi gán tay một gradient đến và gọi `_backward()`:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

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


X = Tensor([[1., 2., 3., 6.], [5., 1., 3., 7.]])
Y = X.layernorm()

Y.grad = np.array([[1., 0., 0., 0.], [0.5, -0.5, 0.5, -0.5]])
Y._backward()

print(np.round(Y.data, 6).tolist())
print(np.round(X.grad, 6).tolist())
```

```text title=readonly
[[-1.069043, -0.534522, 0.0, 1.603565], [0.447213, -1.341639, -0.447213, 1.341639]]
[[0.248171, -0.20999, -0.13363, 0.09545], [0.223607, -0.223607, 0.223607, -0.223607]]
```

Cả hai hàng của `Y.data` đều có trung bình xấp xỉ `0` và phương sai xấp xỉ
`1` (kiểm bằng `np.mean`/`np.var` cho giá trị gần `0`/`1` — không đúng
TUYỆT ĐỐI vì `ε` làm lệch đi một lượng cực nhỏ, đúng như thiết kế). Hàng thứ
hai có phương sai LỚN HƠN hàng đầu (`5.0` so với `3.5`), nên với CÙNG một
độ lệch tuyệt đối, hàng đó co gradient lại (chia cho `√σ²` lớn hơn).
::::

::::example{#eps_bao_ve_layernorm}
Nếu MỌI phần tử một hàng bằng nhau (`σ² = 0`), `ε` là thứ DUY NHẤT ngăn
`nan`:

```python title=readonly
import numpy as np

Xc = np.array([[4.0, 4.0, 4.0, 4.0]])
mu_c = np.mean(Xc, axis=-1, keepdims=True)
var_c = np.mean((Xc - mu_c) ** 2, axis=-1, keepdims=True)
print("phuong sai:", var_c.tolist())

with np.errstate(invalid='ignore'):
    xhat_khong_eps = (Xc - mu_c) / np.sqrt(var_c)
print("KHONG eps:", xhat_khong_eps.tolist())

xhat_co_eps = (Xc - mu_c) / np.sqrt(var_c + 1e-5)
print("CO eps:", xhat_co_eps.tolist())
```

```text title=readonly
phuong sai: [0.0]
KHONG eps: [[nan, nan, nan, nan]]
CO eps: [[0.0, 0.0, 0.0, 0.0]]
```

`σ² = 0` đúng nghĩa đen (mọi phần tử `4.0`, không lệch trung bình chút
nào). Không `ε`: `0/0` cho `nan` ở CẢ BỐN phần tử. Có `ε`: mẫu số thành
`√(0 + 1e-5) ≈ 0,00316`, một số DƯƠNG nhỏ nhưng hữu hạn — kết quả `[0, 0, 0,
0]` (đúng về mặt Ý NGHĨA: một hàng không có biến thiên nào thì "chuẩn hoá"
của nó là toàn `0`, không phải `nan`).
::::

::::predict{#doan_hang_phuong_sai_lon commitOnce}
Hai hàng của `X` ở ví dụ đầu có phương sai khác nhau: hàng `1` là `3.5`,
hàng `2` là `5.0`.

**Trước khi tính**, bạn đoán: với CÙNG một độ lệch tuyệt đối so với trung
bình hàng (ví dụ đúng `+2` so với `μ` của hàng đó), hàng nào (phương sai
`3.5` hay phương sai `5.0`) sẽ cho `x̂` (giá trị đã chuẩn hoá) LỚN HƠN về độ
lớn?

:::opt{correct}
Hàng phương sai `3.5` (nhỏ hơn) — `x̂ = (x − μ)/√(σ² + ε)`; với TỬ SỐ giống
nhau (`+2`), MẪU SỐ nhỏ hơn (`√3.5` nhỏ hơn `√5.0`) cho kết quả CHIA lớn
hơn về độ lớn
:::

:::opt
Hàng phương sai `5.0` (lớn hơn) — phương sai lớn hơn nghĩa là dữ liệu "biến
thiên mạnh hơn", nên giá trị chuẩn hoá cũng phải lớn hơn theo
::why
Gần đúng ở việc phương sai LỚN hơn đúng là phản ánh dữ liệu "biến thiên
mạnh" hơn TRƯỚC KHI chuẩn hoá — quan sát về ý nghĩa của phương sai không
sai.

Chỗ lệch: layer normalization tồn tại CHÍNH XÁC để XOÁ bỏ sự khác biệt độ
biến thiên đó — chia cho `√σ²` lớn hơn sẽ CO giá trị chuẩn hoá lại NHỎ hơn,
không phải lớn hơn. Hàng biến thiên mạnh (phương sai lớn) bị "nén" nhiều
hơn, không phải "phóng đại" thêm.
::
:::

:::opt
Cả hai hàng cho `x̂` bằng nhau — vì `ε` đủ nhỏ để không ảnh hưởng gì tới kết
quả, bất kể phương sai gốc là bao nhiêu
::why
Gần đúng ở việc `ε` (`1e-5`) đúng là quá nhỏ để ảnh hưởng ĐÁNG KỂ — quan sát
đó không sai.

Chỗ lệch: câu hỏi không phải về ẢNH HƯỞNG CỦA `ε` — nó về ẢNH HƯỞNG CỦA
CHÍNH PHƯƠNG SAI GỐC (`σ²`, phần LỚN của mẫu số, không phải `ε` cực nhỏ).
Hai hàng có `σ²` khác nhau RÕ RỆT (`3.5` so với `5.0`), nên mẫu số
`√(σ²+ε)` của chúng khác nhau đáng kể, cho `x̂` khác nhau — `ε` không xoá bỏ
sự khác biệt đó.
::
:::
::::

::::code{#viet_layernorm}
Hoàn thiện `layernorm`: bảo vệ mẫu số bằng `eps` (forward), và số hạng cuối
của công thức backward (đóng góp qua `μ`).

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def layernorm(self, eps=1e-5):
        x = self.data
        mu = np.mean(x, axis=-1, keepdims=True)
        xm = x - mu
        var = np.mean(xm ** 2, axis=-1, keepdims=True)
        std = np.sqrt(var + ___)                    # eps
        xhat = xm / std
        out = Tensor(xhat, (self,), 'layernorm')
        N = x.shape[-1]
        def _backward():
            dy = out.grad
            dvar = np.sum(dy * xm, axis=-1, keepdims=True) * (-0.5) * std ** (-3)
            dmu = np.sum(dy * (-1.0 / std), axis=-1, keepdims=True) + dvar * np.mean(-2.0 * xm, axis=-1, keepdims=True)
            self.grad += dy / std + dvar * 2.0 * xm / N + ___    # dmu / N
        out._backward = _backward
        return out


X = Tensor([[1., 2., 3., 6.], [5., 1., 3., 7.]])
Y = X.layernorm()

Y.grad = np.array([[1., 0., 0., 0.], [0.5, -0.5, 0.5, -0.5]])
Y._backward()

print(np.round(Y.data, 6).tolist())
print(np.round(X.grad, 6).tolist())
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


X = Tensor([[1., 2., 3., 6.], [5., 1., 3., 7.]])
Y = X.layernorm()

Y.grad = np.array([[1., 0., 0., 0.], [0.5, -0.5, 0.5, -0.5]])
Y._backward()

print(np.round(Y.data, 6).tolist())
print(np.round(X.grad, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(Y.data, 6).tolist() == [[-1.069043, -0.534522, 0.0, 1.603565], [0.447213, -1.341639, -0.447213, 1.341639]], f"Y.data sai -- dang ra {np.round(Y.data, 6).tolist()}"
assert np.round(X.grad, 6).tolist() == [[0.248171, -0.20999, -0.13363, 0.09545], [0.223607, -0.223607, 0.223607, -0.223607]], f"X.grad sai -- dang ra {np.round(X.grad, 6).tolist()}"
assert np.allclose(np.mean(Y.data, axis=-1), 0.0, atol=1e-6), f"moi hang cua Y.data phai co trung binh xap xi 0 -- dang ra {np.mean(Y.data, axis=-1).tolist()}"

# rieng kiem tra BAO VE EPSILON tren mot HANG HANG SO THAT (phuong sai = 0
# dung nghia den) -- neu blank1 khong dung eps, ket qua se la nan.
X2 = Tensor([[4., 4., 4., 4.]])
Y2 = X2.layernorm()
assert np.all(np.isfinite(Y2.data)), f"layernorm tren hang hang so phai HUU HAN (eps bao ve), khong duoc la nan -- dang ra {Y2.data.tolist()}"
assert np.round(Y2.data, 6).tolist() == [[0.0, 0.0, 0.0, 0.0]], f"layernorm tren hang hang so phai ra toan 0 -- dang ra {np.round(Y2.data, 6).tolist()}"

# KIEM finite-difference THAT, doc lap voi Tensor
def layernorm_np(x, eps=1e-5):
    mu = np.mean(x, axis=-1, keepdims=True)
    var = np.mean((x - mu) ** 2, axis=-1, keepdims=True)
    return (x - mu) / np.sqrt(var + eps)

dy_hang0 = np.array([1.0, 0.0, 0.0, 0.0])

def L(x_hang):
    return np.sum(layernorm_np(x_hang.reshape(1, -1))[0] * dy_hang0)

h = 1e-5
x_hang0 = np.array([1.0, 2.0, 3.0, 6.0])
for k in range(4):
    xp = x_hang0.copy(); xp[k] += h
    xm_ = x_hang0.copy(); xm_[k] -= h
    so = (L(xp) - L(xm_)) / (2 * h)
    sai_so = abs(so - X.grad[0, k])
    assert sai_so < 1e-4, f"gradient hang dau, vi tri {k} lech qua nhieu voi finite-difference -- sai so {sai_so}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu (forward) cộng THÊM `eps` vào `var` trước khi lấy căn — dùng đúng tên tham số `eps` (không phải một số hằng chép tay), giữ mẫu số luôn dương. Chỗ hai (backward) là số hạng CUỐI của công thức `dx` — đóng góp gradient qua `μ`, đã tính sẵn ở dòng `dmu = ...` phía trên — chia cho `N` (số phần tử một hàng), `dmu / N`.
- kind: strategy
  body: 'Forward: `eps`. Backward: `dmu / N`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `eps` và `dmu / N`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: forward phai cong THAM SO eps (khong phai mot hang so chep tay) vao var truoc khi lay can; backward phai cong THEM so hang dmu/N (da tinh san o dong tren) vao cong thuc dx, khong duoc bo qua dong gop qua mu
  requireAst:
  - kind: uses-name, target: eps, min: 1
  - kind: uses-name, target: dmu, min: 1
  - kind: uses-name, target: self, min: 9
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # harness X/Y): loi giai dung dat=true. eps=1: tham so `eps` CHI duoc
  # Load dung MOT lan trong toan bo ham -- chinh la trong blank forward
  # "var + eps" (ten tham so trong chu ky ham `def layernorm(self,
  # eps=1e-5)` la mot ast.arg, KHONG phai ast.Name, nen khong tinh vao dem
  # nay). dmu=1: "dmu = ..." la gan ten (Store, khong dem), nen Load duy
  # nhat cua "dmu" la trong blank backward "dmu / N". self=9 tren toan bo
  # solution. Cheat "std = np.sqrt(var + 0.00001)" (chep hang so thay vi
  # dung ten tham so eps) lam "eps" tut xuong 0 -- bi chan RIENG. Cheat
  # "self.grad += dy/std + dvar*2.0*xm/N" (bo hoan toan so hang dmu/N) lam
  # "dmu" tut xuong 0 -- bi chan RIENG, VA doc lap boi test finite-
  # difference (thieu mot so hang se lam sai so vuot qua 1e-4 ro rang).
  # Cheat "std = np.sqrt(var - eps)" (doi dau eps) GIU NGUYEN ca ba so dem
  # (khong doi ten bien nao, chi doi toan tu + thanh -) nen KHONG bi static
  # bat -- nhung da tu kiem chung bang Python that: tren CHINH du lieu bai
  # nay, "var - eps" cho Y.data lech o CHU SO THAP PHAN THU SAU so voi
  # "var + eps" (vi du hang dau: -1.069046 so voi -1.069043 dung) -- bi bat
  # boi assert gia tri CHINH XAC tren Y.data/X.grad (so sanh danh sach TRON
  # VEN, khong phai mot nguong long leo). Rieng cheat "bo eps hoan toan"
  # (var khong cong gi) bi bat quyet doan hon: tren hang hang so (X2, bai
  # test), no cho nan ngay lap tuc -- bi chan boi assert np.isfinite.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^\\[\\[-1\\.069043, -0\\.534522, 0\\.0, 1\\.603565\\], \\[0\\.447213, -1\\.341639, -0\\.447213, 1\\.341639\\]\\]\\n\\[\\[0\\.248171, -0\\.20999, -0\\.13363, 0\\.09545\\], \\[0\\.223607, -0\\.223607, 0\\.223607, -0\\.223607\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`layernorm` xong — trung bình về `0`, phương sai về `1`, được `eps` bảo vệ,
backward khớp finite-difference. Ba phép toán mới của quest này (`matmul`,
`softmax`, `layernorm`) đều đã qua kiểm bằng số. Bài BOSS: ráp CẢ BA vào
MỘT biểu thức, kiểm toàn diện.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Quest này đã xây bốn phép toán MỚI trên `Tensor` (`matmul`, `transpose`/
`reshape`, `softmax`, `layernorm`), mỗi phép toán kiểm ĐỘC LẬP bằng finite-
difference. Nhưng một khối Transformer thật KHÔNG dùng riêng lẻ từng phép —
nó XÂU CHUỖI chúng (`matmul` rồi `softmax` rồi `layernorm`, ...). Nếu MỖI
phép toán riêng lẻ đã đúng, `.backward()` xâu chuỗi qua TẤT CẢ chúng có tự
động đúng theo, hay cần kiểm THÊM một lần nữa trên biểu thức GHÉP?
::::

::::checkpoint{mastery=0.9}
::::
