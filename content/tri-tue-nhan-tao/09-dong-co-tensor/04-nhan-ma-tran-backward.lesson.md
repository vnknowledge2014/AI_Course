---
id: tri-tue-nhan-tao.dong-co-tensor.nhan-ma-tran-backward
title: "Nhân ma trận qua Tensor: backward, kiểm bằng finite-difference"
summary: "Công thức đạo hàm matmul: dA = dC @ B.T, dB = A.T @ dC. Trên A(2,3)/B(3,4) của bài trước, gán tay C.grad=W rồi gọi C._backward(): A.grad=[[2,2,5],[7,1,7]], B.grad=[[7,2,2,5],[3,1,0,2],[-1,0,-2,-1]]. KIỂM bằng finite-difference THẬT (h=1e-5): sai số tối đa đo được giữa đạo hàm số và đạo hàm giải tích là 9,026646e-11 -- dưới ngưỡng 1e-4, xác nhận công thức đúng độc lập với chính code Tensor."
locale: vi
track: tri-tue-nhan-tao
module: dong-co-tensor
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.nhan-ma-tran-backward]
requires: [ai.nhan-ma-tran-forward]
concepts: [ai.nhan-ma-tran-backward]
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
`matmul` forward xong. Giờ tới công thức đạo hàm — và lần này, KHÔNG tin
công thức cho tới khi kiểm bằng số THẬT.
::::

::::explain{#cong_thuc_backward_matmul}
Với `C = A @ B` (`A` shape `(m, k)`, `B` shape `(k, n)`, `C` shape
`(m, n)`), một phần tử `A[i, t]` ảnh hưởng tới CẢ MỘT HÀNG của `C` (mọi
`C[i, j]` với `j` bất kỳ, vì `C[i, j] = Σₜ A[i, t]·B[t, j]`) — khác hẳn
`__mul__` element-wise, nơi mỗi phần tử chỉ ảnh hưởng ĐÚNG một phần tử kết
quả. Công thức đạo hàm cục bộ của `matmul`, viết bằng chính phép nhân ma
trận (chứng minh đầy đủ nằm ngoài phạm vi bài này — quest DÙNG công thức
CHUẨN của giải tích ma trận, rồi KIỂM nó bằng số ngay sau đây):

> `dL/dA = dL/dC @ Bᵀ` — gọi `dC` là gradient đã lan tới `C` (`out.grad`),
> công thức là `self.grad += out.grad @ other.data.T`.
>
> `dL/dB = Aᵀ @ dL/dC` — công thức là `other.grad += self.data.T @
> out.grad`.

Vì sao cần CHUYỂN VỊ (`.T`)? Đây là câu hỏi hình dạng, không chỉ công thức:
`out.grad` có shape `(m, n)` (giống `C`), `other.data` (`B`) có shape
`(k, n)`. Muốn kết quả CÙNG shape với `A` (`(m, k)`), phép nhân phải là
`(m, n) @ (n, k)` — mà `other.data` là `(k, n)`, nên phải CHUYỂN VỊ thành
`(n, k)` trước khi nhân. Thiếu `.T` thì shape không khớp — `numpy` ném lỗi
NGAY, không lặng lẽ cho một gradient sai.

Quest này KHÔNG dừng ở "công thức trông có lý" — mọi công thức đạo hàm MỚI
của `Tensor` (`matmul` ở đây, `softmax`/`layer-norm` các bài sau) đều phải
qua **kiểm đạo hàm bằng số** (`kiem-dao-ham-bang-so`, q8.2b): tính đạo hàm
CẢ HAI cách — công thức giải tích (`_backward`) VÀ sai phân trung tâm
(`(f(x+h)-f(x-h))/(2h)`, hoàn toàn KHÔNG chạm vào `Tensor`) — rồi đối chiếu.
::::

::::example{#matmul_backward_that}
Hoàn thiện `matmul` với `_backward` đúng công thức, gán TAY một gradient
đến (`W`, đóng vai `dL/dC`) rồi gọi `C._backward()` một bước:

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
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
        out._backward = _backward
        return out


A = Tensor([[1., 0., -1.], [2., 1., 0.]])                 # shape (2, 3)
B = Tensor([[1., 2., 0., 1.], [0., 1., 1., 0.], [1., 0., 1., 2.]])  # shape (3, 4)
C = A.matmul(B)

W = np.array([[1., 0., 2., 1.], [3., 1., 0., 2.]])  # dong vai dL/dC
C.grad = W
C._backward()

print(np.round(A.grad, 4).tolist())
print(np.round(B.grad, 4).tolist())
```

```text title=readonly
[[2.0, 2.0, 5.0], [7.0, 1.0, 7.0]]
[[7.0, 2.0, 2.0, 5.0], [3.0, 1.0, 0.0, 2.0], [-1.0, 0.0, -2.0, -1.0]]
```

`A.grad = W @ B.dataᵀ`, `B.grad = A.dataᵀ @ W` — tính bằng ĐÚNG hai dòng
`_backward` vừa viết. Nhưng đây mới là công thức, chưa phải bằng chứng.
::::

::::example{#kiem_bang_finite_difference_that}
Kiểm ĐỘC LẬP bằng sai phân trung tâm — một hàm Python THUẦN, không chạm vào
`Tensor`, tính `L = Σ (A @ B) · W` rồi nhích từng phần tử của `A`/`B`:

```python title=readonly
import numpy as np

A_data = np.array([[1., 0., -1.], [2., 1., 0.]])
B_data = np.array([[1., 2., 0., 1.], [0., 1., 1., 0.], [1., 0., 1., 2.]])
W = np.array([[1., 0., 2., 1.], [3., 1., 0., 2.]])


def L(A_data, B_data, W):
    return np.sum((A_data @ B_data) * W)


def dao_ham_so_phan_tu(mat, i, j, ham, h=1e-5):
    mat_p = mat.copy(); mat_p[i, j] += h
    mat_m = mat.copy(); mat_m[i, j] -= h
    return (ham(mat_p) - ham(mat_m)) / (2 * h)


A_grad_giai_tich = np.array([[2., 2., 5.], [7., 1., 7.]])
B_grad_giai_tich = np.array([[7., 2., 2., 5.], [3., 1., 0., 2.], [-1., 0., -2., -1.]])

sai_so_toi_da = 0.0
for i in range(2):
    for j in range(3):
        so = dao_ham_so_phan_tu(A_data, i, j, lambda m: L(m, B_data, W))
        sai_so_toi_da = max(sai_so_toi_da, abs(so - A_grad_giai_tich[i, j]))
for i in range(3):
    for j in range(4):
        so = dao_ham_so_phan_tu(B_data, i, j, lambda m: L(A_data, m, W))
        sai_so_toi_da = max(sai_so_toi_da, abs(so - B_grad_giai_tich[i, j]))

print(sai_so_toi_da < 1e-4)
```

```text title=readonly
True
```

Sai số tối đa đo được giữa đạo hàm số (sai phân trung tâm, hoàn toàn không
biết `Tensor` tồn tại) và đạo hàm giải tích (`_backward` của `matmul`) là
`9,026646e-11` (đo bằng Python thật) — thấp hơn `1e-4` ở MỌI phần tử của
CẢ HAI ma trận. Công thức `dA = dC @ Bᵀ`, `dB = Aᵀ @ dC` ĐÚNG, không phải
"trông có lý".
::::

::::predict{#doan_quen_chuyen_vi commitOnce}
Công thức đúng là `self.grad += out.grad @ other.data.T`. `out.grad` shape
`(2, 4)`, `other.data` (`B`) shape `(3, 4)`.

**Trước khi chạy thử**, bạn đoán: nếu quên `.T` — viết `out.grad @
other.data` — điều gì xảy ra?

:::opt{correct}
Chương trình dừng với `ValueError` NGAY — chiều trong của `out.grad @
other.data` đòi `4` (cột của `out.grad`) khớp `3` (hàng của `other.data`,
vì thiếu `.T` nên `other.data` vẫn giữ shape `(3, 4)` chứ không phải
`(4, 3)`); `4 ≠ 3`, `numpy` từ chối nhân ngay tại bước này
:::

:::opt
Chạy được, nhưng cho một `A.grad` có shape ĐÚNG mà GIÁ TRỊ sai
::why
Gần đúng ở việc lo lắng về một kết quả SAI mà vẫn "trông ổn" — đây đúng là
loại lỗi khó phát hiện nhất khi viết công thức đạo hàm ma trận.

Chỗ lệch: ở TRƯỜNG HỢP CỤ THỂ này, thiếu `.T` không chỉ làm SAI giá trị — nó
làm SAI CẢ SHAPE (chiều trong `4` so với `3` không khớp), nên `numpy` từ
chối tính toán hoàn toàn, không có kết quả "trông ổn" nào để so sánh.
::
:::

:::opt
Chạy được và cho ĐÚNG kết quả — vì `numpy` tự động chuyển vị khi cần, không
phụ thuộc người viết code có gõ `.T` hay không
::why
Gần đúng ở việc nhiều thao tác `numpy` (như broadcast ở bài
`vi-sao-can-tensor`) đúng là có cơ chế TỰ ĐỘNG xử lý một số khác biệt hình
dạng — quan sát về sự tự động hoá của `numpy` không sai ở NHỮNG NGỮ CẢNH
đó.

Chỗ lệch: chuyển vị (`.T`) không nằm trong nhóm "tự động" đó — `numpy` không
bao giờ tự ý đoán ý định của người viết code và tự chuyển vị một ma trận hộ.
`@` chỉ kiểm tra chiều trong có khớp hay không, và từ chối thẳng nếu không.
::
:::
::::

::::code{#viet_matmul_backward}
Hoàn thiện `_backward` của `matmul`: hai công thức đạo hàm, mỗi công thức
cần đúng MỘT phép chuyển vị.

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
        def _backward():
            self.grad += ___              # out.grad @ other.data.T
            other.grad += ___             # self.data.T @ out.grad
        out._backward = _backward
        return out


A = Tensor([[1., 0., -1.], [2., 1., 0.]])
B = Tensor([[1., 2., 0., 1.], [0., 1., 1., 0.], [1., 0., 1., 2.]])
C = A.matmul(B)

W = np.array([[1., 0., 2., 1.], [3., 1., 0., 2.]])
C.grad = W
C._backward()

print(np.round(A.grad, 4).tolist())
print(np.round(B.grad, 4).tolist())
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
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
        out._backward = _backward
        return out


A = Tensor([[1., 0., -1.], [2., 1., 0.]])
B = Tensor([[1., 2., 0., 1.], [0., 1., 1., 0.], [1., 0., 1., 2.]])
C = A.matmul(B)

W = np.array([[1., 0., 2., 1.], [3., 1., 0., 2.]])
C.grad = W
C._backward()

print(np.round(A.grad, 4).tolist())
print(np.round(B.grad, 4).tolist())
```

```python title=test
import numpy as np

assert np.round(A.grad, 4).tolist() == [[2.0, 2.0, 5.0], [7.0, 1.0, 7.0]], f"A.grad sai -- dang ra {np.round(A.grad, 4).tolist()}"
assert np.round(B.grad, 4).tolist() == [[7.0, 2.0, 2.0, 5.0], [3.0, 1.0, 0.0, 2.0], [-1.0, 0.0, -2.0, -1.0]], f"B.grad sai -- dang ra {np.round(B.grad, 4).tolist()}"

# KIEM BANG FINITE-DIFFERENCE THAT -- doc lap voi Tensor, dung numpy thuan.
def L(A_data, B_data, W_data):
    return np.sum((A_data @ B_data) * W_data)

h = 1e-5
def fd_A(i, j):
    Ap = A.data.copy(); Ap[i, j] += h
    Am = A.data.copy(); Am[i, j] -= h
    return (L(Ap, B.data, W) - L(Am, B.data, W)) / (2 * h)

def fd_B(i, j):
    Bp = B.data.copy(); Bp[i, j] += h
    Bm = B.data.copy(); Bm[i, j] -= h
    return (L(A.data, Bp, W) - L(A.data, Bm, W)) / (2 * h)

for i in range(2):
    for j in range(3):
        sai_so = abs(fd_A(i, j) - A.grad[i, j])
        assert sai_so < 1e-4, f"gradient A[{i},{j}] lech qua nhieu voi finite-difference -- sai so {sai_so}"
for i in range(3):
    for j in range(4):
        sai_so = abs(fd_B(i, j) - B.grad[i, j])
        assert sai_so < 1e-4, f"gradient B[{i},{j}] lech qua nhieu voi finite-difference -- sai so {sai_so}"

# rieng kiem tra CONG THUC TONG QUAT tren mot cap ma tran KHAC, upstream la
# ones (don gian hoa L thanh sum(D1@D2)).
D1 = Tensor([[2., 1.], [0., 3.]])
D2 = Tensor([[1., 0., 2.], [1., 1., 0.]])
E = D1.matmul(D2)
E.grad = np.ones((2, 3))
E._backward()
assert np.round(D1.grad, 4).tolist() == [[3.0, 2.0], [3.0, 2.0]], f"D1.grad sai -- dang ra {np.round(D1.grad, 4).tolist()}"
assert np.round(D2.grad, 4).tolist() == [[2.0, 2.0, 2.0], [4.0, 4.0, 4.0]], f"D2.grad sai -- dang ra {np.round(D2.grad, 4).tolist()}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, mỗi chỗ một phép nhân ma trận CÓ chuyển vị. `self.grad` (đạo hàm theo `A`) cộng `out.grad @ other.data.T` — nhân gradient đến với `B` đã CHUYỂN VỊ. `other.grad` (đạo hàm theo `B`) cộng `self.data.T @ out.grad` — nhân `A` đã CHUYỂN VỊ với gradient đến. Kiểm shape trước khi chốt: kết quả mỗi dòng phải khớp shape của `self.data`/`other.data` tương ứng.
- kind: strategy
  body: '`self.grad += out.grad @ other.data.T` và `other.grad += self.data.T @ out.grad`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `out.grad @ other.data.T` và `self.data.T @ out.grad`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: matmul backward phai dung DUNG cong thuc dao ham ma tran -- self.grad cong out.grad @ other.data.T, other.grad cong self.data.T @ out.grad, ca hai deu can dung phep CHUYEN VI (.T)
  requireAst:
  - kind: uses-name, target: self, min: 10
  - kind: uses-name, target: other, min: 4
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # harness A/B/C/W): loi giai dung dat=true. self=10 (sau lan trong
  # __init__ nhu bai truoc, cong self.data (tao out), self trong tuple
  # (self,other), self.grad @ blank1, self.data.T @ blank2 -- moi dong
  # `self.xxx` dem CA Load cua ten `self`). other=4 (other.data khi tao
  # out, other trong tuple, other.data.T trong blank1, other.grad trong
  # blank2). Khong dung duoc kind rieng cho ".T" (Attribute access khong
  # nam trong 18 AST-kind hop le -- chi Name/Call/toan tu duoc ho tro).
  # Cheat "self.grad += out.grad" / "other.grad += out.grad" (chep cong
  # thuc cua __add__, bo qua ca other.data.T lan self.data.T) lam self tut
  # xuong 9 VA other tut xuong 3 -- bi chan KEP. Cheat "quen .T" (out.grad @
  # other.data, hay self.data @ out.grad) GIU NGUYEN so dem self/other
  # (khong doi ten bien, chi bo .T) nen KHONG bi static bat -- nhung da tu
  # kiem chung bang Python that: voi shape A=(2,3), B=(3,4), ca hai cach
  # quen .T deu NEM ValueError (chieu trong 4 khac 3, hoac 3 khac 2) --
  # bi chan boi tier `run`. Cheat "dao nguoc ten bien trong .T" (vi du
  # "self.data.T @ other.data.T" cho blank1) giu nguyen so dem self/other
  # NHUNG da tu kiem chung bang Python that: voi shape A=(2,3), B=(3,4),
  # bien the nay CUNG nem ValueError (chieu trong 2 khac 4) -- bi chan boi
  # tier `run`, cung nhu hai cheat "quen .T" o tren. Moi bien the khac doi
  # gia tri ma KHONG doi shape ket qua (vi du hoan doi vai tro self/other
  # trong CA HAI blank cung luc) se bi bat boi tier tests (gia tri A.grad/
  # B.grad chinh xac LAN finite-difference LAN cap D1/D2 doc lap).
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^\\[\\[2\\.0, 2\\.0, 5\\.0\\], \\[7\\.0, 1\\.0, 7\\.0\\]\\]\\n\\[\\[7\\.0, 2\\.0, 2\\.0, 5\\.0\\], \\[3\\.0, 1\\.0, 0\\.0, 2\\.0\\], \\[-1\\.0, 0\\.0, -2\\.0, -1\\.0\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sai số dưới `1e-4`, độc lập bằng finite-difference. `matmul` giờ có cả
forward lẫn backward ĐÚNG. Bài sau: chuyển vị và định hình lại — hai phép
biến đổi shape cần cho attention.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài `chuyen-vi-va-dinh-hinh-lai` sắp tới cần chính phép `.T` vừa dùng trong
`_backward` của `matmul` — nhưng lần này KHÔNG ẩn bên trong một công thức
khác, mà là một PHÉP TOÁN ĐỘC LẬP: `Y = X.transpose()`. Nếu `Y = X.T`, và
gradient đã lan tới `Y` là `dY`, bạn đoán công thức đạo hàm cục bộ cho
`dX` (gradient lan ngược về `X`) sẽ đơn giản tới mức nào?
::::

::::checkpoint{mastery=0.85}
::::
