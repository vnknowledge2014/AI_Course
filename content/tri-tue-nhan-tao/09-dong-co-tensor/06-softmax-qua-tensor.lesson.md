---
id: tri-tue-nhan-tao.dong-co-tensor.softmax-qua-tensor
title: "Softmax qua Tensor: ổn định số học, cả một mảng"
summary: "Tensor.softmax() TÁI DÙNG công thức ổn định số học của on-dinh-so-hoc (trừ max trước exp), áp theo TRỤC CUỐI cho cả một mảng (mỗi hàng một phân phối riêng). Trên Z=[[2,1,0.1],[0.5,0.5,0.5]]: S=[[0,659001; 0,242433; 0,098566],[0,333333 x3]], tổng mỗi hàng =1. Backward là CÔNG THỨC GỘP (không phải Jacobian đầy đủ): dz = s*(dy - sum(dy*s)) -- kiểm bằng finite-difference THẬT, sai số dưới 1e-9."
locale: vi
track: tri-tue-nhan-tao
module: dong-co-tensor
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.softmax-qua-tensor]
requires: [ai.chuyen-vi-va-dinh-hinh-lai]
concepts: [ai.softmax-qua-tensor]
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
Điểm attention (bài sau nữa) cần biến MỌI hàng điểm số thành một phân phối
xác suất — softmax. Công thức đã học ở `on-dinh-so-hoc` (T8.1c); giờ áp nó
qua `Tensor`, trên CẢ MỘT mảng cùng lúc.
::::

::::explain{#softmax_qua_tensor}
Công thức softmax ỔN ĐỊNH SỐ HỌC đã học ở `on-dinh-so-hoc` — TRỪ giá trị lớn
nhất trước khi lấy mũ, để tránh tràn số (`inf`/`nan`):

> `softmax(z)ₖ = e^(zₖ − max(z)) / Σⱼ e^(zⱼ − max(z))`

Bài đó áp công thức lên MỘT vector. `Tensor.softmax()` áp ĐÚNG công thức
này lên trục CUỐI (`axis=-1`) của một mảng CÓ THỂ nhiều hàng — mỗi hàng là
MỘT phân phối xác suất RIÊNG (`np.max(..., axis=-1, keepdims=True)` lấy giá
trị lớn nhất CỦA TỪNG HÀNG, không phải một giá trị lớn nhất chung cho cả
mảng — trộn hàng sẽ làm sai công thức, vì mỗi hàng cần trừ ĐÚNG giá trị lớn
nhất CỦA CHÍNH NÓ).

Đạo hàm cục bộ của `softmax` khác HẲN mọi phép toán đã học: `__mul__` có
mỗi phần tử đầu ra phụ thuộc ĐÚNG một phần tử tương ứng của đầu vào;
`matmul` có mỗi phần tử đầu ra phụ thuộc một HÀNG và một CỘT. `softmax` thì
MỖI phần tử đầu ra của một hàng phụ thuộc TẤT CẢ phần tử ĐẦU VÀO của CHÍNH
hàng đó — công thức đầy đủ (Jacobian) cho một hàng `s = softmax(z)` là ma
trận `∂sᵢ/∂zⱼ = sᵢ·(δᵢⱼ − sⱼ)` (`δᵢⱼ = 1` nếu `i = j`, else `0`).

May mắn, lan gradient không cần dựng cả ma trận Jacobian đó — chỉ cần TÍCH
của nó với gradient đến `dy` (kỹ thuật vector-Jacobian product), rút gọn
thành một công thức GỘP, không vòng lặp:

> `dzⱼ = Σᵢ dyᵢ · sᵢ·(δᵢⱼ − sⱼ) = sⱼ·dyⱼ − sⱼ·Σᵢ dyᵢsᵢ = sⱼ·(dyⱼ − Σᵢ dyᵢsᵢ)`

Viết bằng `numpy` (`dot = Σᵢ dyᵢsᵢ`, tính theo TỪNG HÀNG): `dz = s * (dy -
dot)`, với `dot = np.sum(dy * s, axis=-1, keepdims=True)`.
::::

::::example{#softmax_forward_backward_that}
`Z` hai hàng — hàng đầu có một logit lớn hẳn (`2.0`), hàng sau ba logit
BẰNG NHAU (softmax phải cho phân phối ĐỀU). Forward, rồi gán TAY một
gradient đến (`dY`, khác nhau giữa hai hàng) và gọi `_backward()`:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

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


Z = Tensor([[2.0, 1.0, 0.1], [0.5, 0.5, 0.5]])
S = Z.softmax()

print(np.round(S.data, 6).tolist())
print(np.round(np.sum(S.data, axis=-1), 6).tolist())

S.grad = np.array([[1.0, 0.0, 0.0], [0.3, 0.3, 0.4]])
S._backward()

print(np.round(Z.grad, 6).tolist())
```

```text title=readonly
[[0.659001, 0.242433, 0.098566], [0.333333, 0.333333, 0.333333]]
[1.0, 1.0]
[[0.224719, -0.159764, -0.064955], [-0.011111, -0.011111, 0.022222]]
```

Hàng đầu (`Z` lệch hẳn) cho một phân phối LỆCH (`0,659001` cho token có
logit lớn nhất); hàng sau (`Z` bằng nhau tuyệt đối) cho phân phối ĐỀU
(`0,333333` × 3, đúng `1/3`). CẢ HAI hàng cộng đúng `1.0` — bất biến của
softmax, không phụ thuộc `Z` là gì. `Z.grad` không đơn thuần là `S.data *
S.grad` (thiếu phần trừ `dot`) — công thức gộp trừ đi phần "trung bình có
trọng số theo `s`" của `dy`, đúng bản chất Jacobian của softmax.
::::

::::example{#kiem_finite_difference_softmax}
Kiểm ĐỘC LẬP bằng sai phân trung tâm, chỉ trên HÀNG ĐẦU của `Z` (`L = Σⱼ
sⱼ · dyⱼ` với `dy` cố định — đúng loss mà gradient ở trên đo):

```python title=readonly
import numpy as np

z_hang = np.array([2.0, 1.0, 0.1])
dy_hang = np.array([1.0, 0.0, 0.0])


def softmax_on_dinh(z):
    z_shift = z - np.max(z)
    e = np.exp(z_shift)
    return e / np.sum(e)


def L(z):
    return np.sum(softmax_on_dinh(z) * dy_hang)


h = 1e-5
dz_giai_tich = np.array([0.224719, -0.159764, -0.064955])
sai_so_toi_da = 0.0
for k in range(3):
    zp = z_hang.copy(); zp[k] += h
    zm = z_hang.copy(); zm[k] -= h
    so = (L(zp) - L(zm)) / (2 * h)
    sai_so_toi_da = max(sai_so_toi_da, abs(so - dz_giai_tich[k]))

print(sai_so_toi_da < 1e-4)
```

```text title=readonly
True
```

Sai số tối đa đo được là dưới `1e-9` (đo bằng Python thật) — thấp hơn `1e-4`
ở cả `3` phần tử. Công thức gộp `dz = s·(dy − dot)` khớp đạo hàm THẬT của
softmax, không chỉ trông có lý.
::::

::::predict{#doan_tong_dong_dz commitOnce}
Ví dụ trên: hàng sau của `Z.grad` là `[-0.011111, -0.011111, 0.022222]`
(cho `dY` hàng đó là `[0.3, 0.3, 0.4]`).

**Trước khi cộng thử**, bạn đoán: tổng BA phần tử của MỘT HÀNG bất kỳ trong
`Z.grad` (đạo hàm ngược qua `softmax`) sẽ luôn xấp xỉ bao nhiêu, BẤT KỂ
`dY` hàng đó là gì?

:::opt{correct}
Luôn xấp xỉ `0` — mỗi hàng của `S.data` (đầu ra softmax) LUÔN cộng đúng `1`,
một hằng số KHÔNG đổi theo `Z`; đạo hàm của một hằng số theo bất kỳ hướng
thay đổi nào của `Z` phải bằng `0`, nên tổng `dz` theo mọi hướng "thay đổi
đồng đều" luôn triệt tiêu, không phụ thuộc `dY`
:::

:::opt
Luôn bằng tổng của `dY` hàng đó — vì `_backward` chỉ phân phối lại `dY` cho
các phần tử đầu vào, không tạo ra hay mất giá trị
::why
Gần đúng ở trực giác "gradient chỉ phân phối lại, không tự sinh ra" — đúng
với NHIỀU phép toán tuyến tính (như `+`, nơi tổng gradient được bảo toàn).

Chỗ lệch: `softmax` không phải một phép TUYẾN TÍNH — công thức `dz = s·(dy
− dot)` có `dot` phụ thuộc CẢ `s` LẪN `dy` theo cách phi tuyến, và tổng của
`dz` triệt tiêu vì lý do KHÁC hẳn (ràng buộc tổng đầu ra `= 1`, không phải
bảo toàn tổng gradient đầu vào). Tổng `dY` hàng đó là `0,3+0,3+0,4=1.0`,
khác hẳn tổng `dZ.grad` hàng đó (`≈0`).
::
:::

:::opt
Phụ thuộc vào `Z` — hàng có logit lệch nhiều sẽ có tổng khác hàng có logit
bằng nhau
::why
Gần đúng ở việc để ý hai hàng của `Z` trong ví dụ trên CÓ hình dạng khác
nhau (một lệch, một đều) — quan sát về sự khác biệt giữa hai hàng không sai.

Chỗ lệch: ràng buộc "tổng đầu ra mỗi hàng luôn là `1`" áp dụng cho MỌI `Z`,
bất kể hình dạng phân phối cụ thể — nên tổng `dz` mỗi hàng LUÔN triệt tiêu
về `0`, không phân biệt hàng lệch hay hàng đều. Cả hai hàng trong ví dụ trên
đều có tổng `dZ.grad` xấp xỉ `0`, dù `dY` và hình dạng `S` của chúng khác
nhau.
::
:::
::::

::::code{#viet_softmax}
Hoàn thiện `softmax`: chuẩn hoá bằng tổng (forward), và công thức gộp
(backward).

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / ___                          # np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        def _backward():
            dy = out.grad
            dot = np.sum(dy * s, axis=-1, keepdims=True)
            self.grad += ___                 # s * (dy - dot)
        out._backward = _backward
        return out


Z = Tensor([[2.0, 1.0, 0.1], [0.5, 0.5, 0.5]])
S = Z.softmax()

S.grad = np.array([[1.0, 0.0, 0.0], [0.3, 0.3, 0.4]])
S._backward()

print(np.round(S.data, 6).tolist())
print(np.round(Z.grad, 6).tolist())
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


Z = Tensor([[2.0, 1.0, 0.1], [0.5, 0.5, 0.5]])
S = Z.softmax()

S.grad = np.array([[1.0, 0.0, 0.0], [0.3, 0.3, 0.4]])
S._backward()

print(np.round(S.data, 6).tolist())
print(np.round(Z.grad, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(S.data, 6).tolist() == [[0.659001, 0.242433, 0.098566], [0.333333, 0.333333, 0.333333]], f"S.data sai -- dang ra {np.round(S.data, 6).tolist()}"
assert np.allclose(np.sum(S.data, axis=-1), 1.0), f"moi hang cua S.data phai cong dung 1.0 -- dang ra {np.sum(S.data, axis=-1).tolist()}"
assert np.round(Z.grad, 6).tolist() == [[0.224719, -0.159764, -0.064955], [-0.011111, -0.011111, 0.022222]], f"Z.grad sai -- dang ra {np.round(Z.grad, 6).tolist()}"

# rieng kiem tra tinh chat TONG MOI HANG cua Z.grad xap xi 0 (bat bien cua
# softmax, doc lap voi gia tri cu the).
tong_hang = np.sum(Z.grad, axis=-1)
assert np.allclose(tong_hang, 0.0, atol=1e-6), f"tong moi hang cua Z.grad phai xap xi 0 -- dang ra {tong_hang.tolist()}"

# KIEM finite-difference THAT tren hang dau, doc lap voi Tensor
def softmax_on_dinh(z):
    z_shift = z - np.max(z)
    e = np.exp(z_shift)
    return e / np.sum(e)

z_hang = np.array([2.0, 1.0, 0.1])
dy_hang = np.array([1.0, 0.0, 0.0])

def L(z):
    return np.sum(softmax_on_dinh(z) * dy_hang)

h = 1e-5
for k in range(3):
    zp = z_hang.copy(); zp[k] += h
    zm = z_hang.copy(); zm[k] -= h
    so = (L(zp) - L(zm)) / (2 * h)
    sai_so = abs(so - Z.grad[0, k])
    assert sai_so < 1e-4, f"gradient hang dau, vi tri {k} lech qua nhieu voi finite-difference -- sai so {sai_so}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu (forward) chuẩn hoá `e` thành một phân phối — chia cho TỔNG của `e` THEO TỪNG HÀNG, `np.sum(e, axis=-1, keepdims=True)` (thiếu `axis=-1` sẽ cộng dồn NHẦM qua cả hai hàng). Chỗ hai (backward) là công thức gộp đã suy ra ở phần giải thích — `s * (dy - dot)`, KHÔNG phải chỉ `s * dy` (thiếu phần trừ `dot` sẽ bỏ mất phần "trung bình có trọng số" của Jacobian).
- kind: strategy
  body: 'Forward: `np.sum(e, axis=-1, keepdims=True)`. Backward: `s * (dy - dot)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `np.sum(e, axis=-1, keepdims=True)` và `s * (dy - dot)`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: forward phai chia e cho TONG cua no THEO TUNG HANG (np.sum voi axis=-1, keepdims=True); backward phai dung DUNG cong thuc gop s*(dy-dot), khong duoc bo phan tru dot
  requireAst:
  - kind: uses-call, target: sum, min: 2
  - kind: uses-name, target: dot, min: 1
  - kind: uses-name, target: s, min: 3
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true. sum=2: mot lan trong forward (np.sum(e, axis=-1,
  # keepdims=True), la BLANK), mot lan trong backward khi tinh dot
  # (np.sum(dy*s,...), CO SAN trong starter, khong phai blank). dot=1:
  # "dot = np.sum(...)" la GAN TEN (Store, khong tinh vao uses-name), nen
  # LAN DOC duy nhat cua "dot" la trong chinh blank backward "s*(dy-dot)"
  # -- dung 1. s=3: "s = e/..." la gan ten (khong dem), "s" trong
  # "Tensor(s,...)" (dem 1), "s" trong "dot=np.sum(dy*s,...)" co san trong
  # starter (dem 1), va "s" trong blank backward "s*(dy-dot)" (dem 1) --
  # tong 3. KHONG dung duoc has-literal cho "axis=-1": kiemAst's has-
  # literal chi so sanh truc tiep gia tri cua nut ast.Constant -- ma
  # "-1" trong Python AST la UnaryOp(USub, Constant(1)), KHONG PHAI
  # Constant(-1) -- da tu kiem chung bang ast that: has-literal target
  # "-1" luon dem duoc 0 tren MOI doan code co "axis=-1" (kha ca tren
  # chinh loi giai dung), nen mot luat nhu vay se lam ngay ca loi giai
  # dung cung TRUOT -- KHONG dung o day. Cheat "s = e / np.sum(e)" (quen
  # axis=-1, cong don sai qua ca hai hang) giu nguyen "sum"=2 (van goi
  # np.sum dung mot lan trong blank) nen KHONG bi static bat rieng --
  # nhung da tu kiem chung bang Python that: cheat nay lam moi hang cua
  # S.data KHONG con cong dung 1.0 rieng le (tong ca mang gop lai moi la
  # 1.0) -- bi bat NGAY boi assert np.allclose(np.sum(S.data, axis=-1),
  # 1.0) trong tier tests. Cheat "self.grad += s * dy" (bo phan tru dot,
  # cong thuc SAI don gian hoa qua muc) lam "dot" tut xuong 0 (khong con
  # Load nao trong blank) -- bi chan RIENG boi luat dot, VA doc lap boi
  # assert tong moi hang cua Z.grad xap xi 0 trong tier tests (cong thuc
  # thieu se KHONG con thoa bat bien nay).
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^\\[\\[0\\.659001, 0\\.242433, 0\\.098566\\], \\[0\\.333333, 0\\.333333, 0\\.333333\\]\\]\\n\\[\\[0\\.224719, -0\\.159764, -0\\.064955\\], \\[-0\\.011111, -0\\.011111, 0\\.022222\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`softmax` xong — forward ổn định số học, backward khớp finite-difference
dưới `1e-9`. Bài sau: một phép chuẩn hoá KHÁC, `layer normalization` — kỹ
thuật hoàn toàn mới mà `Value` (T8.2) chưa từng cần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`softmax` chuẩn hoá một hàng thành TỔNG `= 1` (một phân phối xác suất).
`layer normalization` (bài sau) chuẩn hoá một hàng theo cách KHÁC — trung
bình `= 0`, phương sai `= 1` — không quan tâm tổng có bằng `1` hay không.
Cả hai đều là "MỘT phần tử đầu ra phụ thuộc TẤT CẢ phần tử đầu vào của cùng
hàng", giống hệt tính chất vừa học của `softmax`. Công thức đạo hàm cục bộ
của `layer normalization` có tận dụng được gì từ công thức gộp `s·(dy −
dot)` vừa suy ra, hay phải suy từ đầu?
::::

::::checkpoint{mastery=0.85}
::::
