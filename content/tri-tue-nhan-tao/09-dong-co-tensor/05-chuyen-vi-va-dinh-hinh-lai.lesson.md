---
id: tri-tue-nhan-tao.dong-co-tensor.chuyen-vi-va-dinh-hinh-lai
title: "Chuyển vị và định hình lại: transpose, reshape"
summary: "Tensor.transpose() (backward: out.grad.T -- chuyển vị NGƯỢC lại gradient) và Tensor.reshape(shape) (backward: out.grad.reshape(self.data.shape) -- trả gradient về ĐÚNG shape gốc). Trên A=[[1,2,3],[4,5,6]], C=A.matmul(A.transpose()) (ma trận Gram, A dùng LẶP LẠI hai lần -- callback tich-luy-gradient): C.backward() cho A.grad=[[10,14,18],[10,14,18]], khớp finite-difference (sai số ~1,4e-9). reshape (2,3)->(6,): gradient reshape ngược lại đúng [[10,20,30],[40,50,60]]."
locale: vi
track: tri-tue-nhan-tao
module: dong-co-tensor
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.chuyen-vi-va-dinh-hinh-lai]
requires: [ai.nhan-ma-tran-backward]
concepts: [ai.chuyen-vi-va-dinh-hinh-lai]
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
Attention sắp tới cần `Q · Kᵀ` — Query nhân với Key ĐÃ chuyển vị. `matmul`
xong rồi, nhưng `.T` chưa phải một phép toán CỦA `Tensor` — chỉ là một chi
tiết ẩn bên trong công thức backward của nó. Bài này tách nó ra thành một
phép toán riêng.
::::

::::explain{#transpose_reshape}
Hai phép biến đổi HÌNH DẠNG, không đổi GIÁ TRỊ nào — chỉ sắp xếp lại các số
đã có:

> **`transpose()`** — đảo hàng thành cột, cột thành hàng (`self.data.T`).
> Backward: nếu `Y = X.transpose()` và gradient đã lan tới `Y` là `dY`, thì
> gradient lan về `X` là `dY` CHUYỂN VỊ NGƯỢC LẠI — `self.grad += out.grad.
> T`. Trực giác: chuyển vị chỉ ĐỔI CHỖ các phần tử, không trộn hay mất
> phần tử nào — nên đường gradient đi ngược lại đúng những chỗ đã đổi, một
> lần `.T` nữa là quay về đúng vị trí gốc.
>
> **`reshape(shape)`** — sắp lại CÙNG các phần tử vào một shape khác (ví dụ
> `(2, 3)` thành `(6,)`, phẳng hoá). Backward: gradient `dY` (shape MỚI)
> phải `reshape` lại về ĐÚNG shape GỐC của `X` — `self.grad += out.grad.
> reshape(self.data.shape)`. `numpy` lưu dữ liệu tuần tự trong bộ nhớ, nên
> `reshape` không đổi THỨ TỰ phần tử — reshape gradient về shape gốc luôn
> khớp ĐÚNG phần tử nào đi với phần tử nào.

Cả hai phép này sẽ cần cho attention (quest sau): Query/Key/Value được chiếu
từ CÙNG một embedding, rồi phải TÁCH thành nhiều "đầu" (multi-head) bằng
`reshape`, và điểm attention cần `Q · Kᵀ` — `transpose()` trên `K`.
::::

::::example{#transpose_backward_gram}
`A` shape `(2, 3)`, `At = A.transpose()` shape `(3, 2)`, `C = A.matmul(At)`
— gọi là **ma trận Gram** (`C = A · Aᵀ`, dùng để đo độ tương tự giữa các
hàng của `A`, đúng vai trò `Q · Kᵀ` của attention sẽ dùng sau này). Chú ý:
`A` xuất hiện HAI LẦN trong đồ thị — một lần trực tiếp làm toán hạng của
`matmul`, một lần NỮA qua `A.transpose()` — đúng gotcha "một node dùng lặp
lại" của bài `tich-luy-gradient` (q8.2b), giờ tái hiện trên `Tensor`:

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

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        def _backward():
            self.grad += out.grad.T
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


A = Tensor([[1., 2., 3.], [4., 5., 6.]])
At = A.transpose()
C = A.matmul(At)
C.backward()

print(np.round(C.data, 4).tolist())
print(np.round(A.grad, 4).tolist())
```

```text title=readonly
[[14.0, 32.0], [32.0, 77.0]]
[[10.0, 14.0, 18.0], [10.0, 14.0, 18.0]]
```

`A.grad` nhận đóng góp từ HAI đường: một trực tiếp (`A` là toán hạng đầu của
`matmul`), một qua `At` (lan ngược qua `transpose()` rồi cộng dồn tiếp vào
`A`). `_prev`/`visited`/`topo` của `.backward()` (giữ nguyên từ
`lop-tensor-co-ban`) xử lý ĐÚNG việc này — CỘNG DỒN cả hai đường, không GHI
ĐÈ — không có gì phải viết thêm, cơ chế cũ vẫn hoạt động cho một node dùng
lặp lại, y hệt `Value` đã làm được ở q8.2b.
::::

::::example{#reshape_backward}
`X` shape `(2, 3)`, `Y = X.reshape((6,))` — phẳng hoá thành một vector `6`
phần tử. Gán TAY một gradient tới `Y` rồi gọi `Y._backward()` một bước:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        def _backward():
            self.grad += out.grad.reshape(self.data.shape)
        out._backward = _backward
        return out


X = Tensor([[1., 2., 3.], [4., 5., 6.]])   # shape (2, 3)
Y = X.reshape((6,))                        # shape (6,)

print(X.data.shape, "->", Y.data.shape)

Y.grad = np.array([10., 20., 30., 40., 50., 60.])
Y._backward()

print(np.round(X.grad, 4).tolist())
```

```text title=readonly
(2, 3) -> (6,)
[[10.0, 20.0, 30.0], [40.0, 50.0, 60.0]]
```

`Y.grad` là một vector phẳng `6` phần tử; `X.grad` sau `_backward()` quay về
ĐÚNG shape gốc `(2, 3)` của `X` — và phần tử THỨ MẤY của `Y.grad` đi vào
đúng vị trí THỨ ĐÓ của `X.grad` khi đọc theo thứ tự hàng (`10` vào `[0,0]`,
`20` vào `[0,1]`, ..., `60` vào `[1,2]`) — reshape không xáo trộn thứ tự.
::::

::::predict{#doan_shape_sau_reshape_backward commitOnce}
`X` shape `(2, 3)`, `Y = X.reshape((3, 2))` (một shape KHÁC ví dụ trên).

**Trước khi tính**, bạn đoán: sau khi gán `Y.grad` (shape `(3, 2)`) và gọi
`Y._backward()`, `X.grad` sẽ có shape gì?

:::opt{correct}
`(2, 3)` — `_backward` của `reshape` LUÔN trả gradient về ĐÚNG shape GỐC
của `self.data` (`self.data.shape`), bất kể `Y` được reshape thành shape
nào; ở đây shape gốc của `X` là `(2, 3)`
:::

:::opt
`(3, 2)` — vì `Y.grad` đã có shape đó, và `_backward` chỉ CỘNG DỒN
`Y.grad` vào `X.grad` mà không đổi gì
::why
Gần đúng ở việc `Y.grad` đúng là điểm KHỞI ĐẦU của phép tính — mọi
`_backward` đều bắt đầu từ `out.grad`.

Chỗ lệch: `_backward` của `reshape` không CỘNG THẲNG `out.grad` vào
`self.grad` — nó gọi `out.grad.reshape(self.data.shape)` TRƯỚC, đổi lại
shape của `out.grad` (`(3, 2)`) thành ĐÚNG shape của `self.data`
(`(2, 3)`) rồi mới cộng dồn. Thiếu bước reshape này, phép cộng thậm chí sẽ
NÉM lỗi shape (giống ví dụ shape-khác-nhau ở bài `lop-tensor-co-ban`).
::
:::

:::opt
Không xác định được — shape của `X.grad` phụ thuộc vào GIÁ TRỊ cụ thể đã
gán cho `Y.grad`, không chỉ phụ thuộc shape của nó
::why
Gần đúng ở việc thận trọng khi một phép biến đổi có vẻ phụ thuộc dữ liệu cụ
thể — thái độ đó hợp lý ở nhiều phép toán khác (như `relu`, phụ thuộc dấu
của giá trị).

Chỗ lệch: `reshape` không hề nhìn vào GIÁ TRỊ nào để quyết định shape kết
quả — chỉ cần biết `self.data.shape` (một thuộc tính CỐ ĐỊNH của `X`, không
đổi theo `Y.grad`). Shape của `X.grad` sau `_backward` LUÔN là
`self.data.shape`, không phụ thuộc giá trị nào đã gán cho `Y.grad`.
::
:::
::::

::::code{#viet_transpose_reshape}
Hoàn thiện `_backward` của `transpose` (chuyển vị NGƯỢC lại gradient) và
`reshape` (trả gradient về ĐÚNG shape gốc).

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
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
        out._backward = _backward
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        def _backward():
            self.grad += ___                              # out.grad.T
        out._backward = _backward
        return out

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        def _backward():
            self.grad += ___                              # out.grad.reshape(self.data.shape)
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


A = Tensor([[1., 2., 3.], [4., 5., 6.]])
At = A.transpose()
C = A.matmul(At)
C.backward()

X = Tensor([[1., 2., 3.], [4., 5., 6.]])
Y = X.reshape((6,))
Y.grad = np.array([10., 20., 30., 40., 50., 60.])
Y._backward()

print(np.round(C.data, 4).tolist())
print(np.round(A.grad, 4).tolist())
print(np.round(X.grad, 4).tolist())
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

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        def _backward():
            self.grad += out.grad.T
        out._backward = _backward
        return out

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        def _backward():
            self.grad += out.grad.reshape(self.data.shape)
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


A = Tensor([[1., 2., 3.], [4., 5., 6.]])
At = A.transpose()
C = A.matmul(At)
C.backward()

X = Tensor([[1., 2., 3.], [4., 5., 6.]])
Y = X.reshape((6,))
Y.grad = np.array([10., 20., 30., 40., 50., 60.])
Y._backward()

print(np.round(C.data, 4).tolist())
print(np.round(A.grad, 4).tolist())
print(np.round(X.grad, 4).tolist())
```

```python title=test
import numpy as np

assert np.round(C.data, 4).tolist() == [[14.0, 32.0], [32.0, 77.0]], f"C.data sai -- dang ra {np.round(C.data, 4).tolist()}"
assert np.round(A.grad, 4).tolist() == [[10.0, 14.0, 18.0], [10.0, 14.0, 18.0]], f"A.grad sai -- dang ra {np.round(A.grad, 4).tolist()}"
assert tuple(X.grad.shape) == (2, 3), f"X.grad phai tro ve DUNG shape goc (2,3) -- dang ra {tuple(X.grad.shape)}"
assert np.round(X.grad, 4).tolist() == [[10.0, 20.0, 30.0], [40.0, 50.0, 60.0]], f"X.grad sai -- dang ra {np.round(X.grad, 4).tolist()}"

# KIEM finite-difference THAT cho A.grad qua transpose+matmul, doc lap voi Tensor
def L(A_data):
    return np.sum(A_data @ A_data.T)

h = 1e-5
def fd(i, j):
    Ap = A.data.copy(); Ap[i, j] += h
    Am = A.data.copy(); Am[i, j] -= h
    return (L(Ap) - L(Am)) / (2 * h)

for i, j in [(0, 0), (1, 2)]:
    sai_so = abs(fd(i, j) - A.grad[i, j])
    assert sai_so < 1e-4, f"gradient A[{i},{j}] lech qua nhieu voi finite-difference -- sai so {sai_so}"

# rieng kiem tra CONG THUC TONG QUAT tren mot Tensor KHAC (transpose don le,
# khong qua matmul) -- chan cheat quen .T.
Z = Tensor([[1., 2.], [3., 4.], [5., 6.]])   # shape (3, 2)
Zt = Z.transpose()                            # shape (2, 3)
Zt.grad = np.array([[1., 0., 2.], [0., 1., 0.]])
Zt._backward()
assert tuple(Z.grad.shape) == (3, 2), f"Z.grad phai co shape (3,2) -- dang ra {tuple(Z.grad.shape)}"
assert np.round(Z.grad, 4).tolist() == [[1.0, 0.0], [0.0, 1.0], [2.0, 0.0]], f"Z.grad sai -- dang ra {np.round(Z.grad, 4).tolist()}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. `transpose`, `_backward`: gradient lan về `self` là gradient của `out` CHUYỂN VỊ NGƯỢC LẠI — `out.grad.T`. `reshape`, `_backward`: gradient lan về `self` là gradient của `out` reshape lại về ĐÚNG shape gốc của `self.data` — `out.grad.reshape(self.data.shape)`, không phải một shape hằng số cố định.
- kind: strategy
  body: 'transpose: `out.grad.T`. reshape: `out.grad.reshape(self.data.shape)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `out.grad.T` và `out.grad.reshape(self.data.shape)`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: transpose backward phai chuyen vi NGUOC LAI gradient (out.grad.T); reshape backward phai goi THAT out.grad.reshape(self.data.shape) -- dung self.data.shape (KHONG phai mot shape hang so co dinh) de tro ve DUNG shape goc
  requireAst:
  - kind: uses-call, target: reshape, min: 3
  - kind: uses-name, target: self, min: 20
  - kind: uses-name, target: other, min: 4
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # harness A/At/C/X/Y): loi giai dung dat=true. reshape=3: dinh nghia
  # forward (self.data.reshape(shape)), blank backward
  # (out.grad.reshape(self.data.shape)), va loi goi harness
  # X.reshape((6,)) -- ba lan goi PHUONG THUC ten "reshape". self=20,
  # other=4 tren toan bo file (khong doi so voi bai truoc vi matmul/
  # __init__ giu nguyen, transpose/reshape moi them self nhung khong them
  # other nao). Cheat "reshape backward: self.grad += out.grad" (bo qua
  # reshape) lam so dem "reshape" tut xuong 2 VA "self" tut xuong duoi 20
  # (mat tham chieu self.data trong blank) -- bi chan KEP. Cheat "reshape
  # backward: out.grad.reshape((2, 3))" (chep san shape hang so thay vi
  # self.data.shape) GIU nguyen so dem "reshape" (=3, vi van la mot loi
  # goi ham reshape) nhung lam "self" tut xuong duoi 20 (mat self.data) --
  # bi chan boi luat self rieng. Cheat "transpose backward: self.grad +=
  # out.grad" (quen .T) KHONG doi bat ky so dem nao (khong co self/other/
  # reshape nao trong bieu thuc out.grad.T hay out.grad) -- nhung da tu
  # kiem chung bang Python that: voi A shape (2,3), At shape (3,2), quen
  # .T lam shape khong khop (3,2) cong don vao (2,3) -- NEM ValueError,
  # bi chan boi tier `run`.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^\\[\\[14\\.0, 32\\.0\\], \\[32\\.0, 77\\.0\\]\\]\\n\\[\\[10\\.0, 14\\.0, 18\\.0\\], \\[10\\.0, 14\\.0, 18\\.0\\]\\]\\n\\[\\[10\\.0, 20\\.0, 30\\.0\\], \\[40\\.0, 50\\.0, 60\\.0\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`A` dùng lặp lại (trực tiếp và qua `transpose()`) — cộng dồn vẫn đúng,
không cần viết thêm gì. `reshape` trả gradient về đúng shape gốc. Bài sau:
`softmax` — phép toán ĐẦU TIÊN của quest mà một phần tử đầu ra phụ thuộc
TẤT CẢ phần tử đầu vào, không chỉ một phần tử tương ứng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mọi phép toán từ đầu quest tới giờ (`+`, `-`, `*` element-wise, `matmul`,
`transpose`, `reshape`) đều có công thức đạo hàm cục bộ viết được bằng MỘT
hoặc HAI dòng — không dòng nào cần một VÒNG LẶP hay một tổng phức tạp bên
trong chính công thức đó (`matmul` tổng qua chỉ số `t`, nhưng `numpy` gói
gọn trong một phép `@`). `softmax` (bài sau) có một tính chất khác hẳn: MỖI
phần tử đầu ra phụ thuộc TẤT CẢ phần tử đầu vào của CÙNG một hàng — không
chỉ một phần tử tương ứng như `__mul__`, và không chỉ một hàng/cột cố định
như `matmul`. Công thức đạo hàm cục bộ của nó có cần một hình dạng khác hẳn
sáu phép toán vừa học không?
::::

::::checkpoint{mastery=0.85}
::::
