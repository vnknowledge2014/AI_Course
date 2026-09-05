---
id: tri-tue-nhan-tao.dong-co-tensor.nhan-ma-tran-forward
title: "Nhân ma trận qua Tensor: forward"
summary: "Tensor.matmul dùng np.matmul (self.data @ other.data) -- KHÁC hẳn __mul__ element-wise: matmul đòi hai shape TƯƠNG THÍCH (m,k)@(k,n)=(m,n), không đòi CÙNG shape. Trên A shape (2,3), B shape (3,4): C=A@B shape (2,4) = [[0,2,-1,-1],[2,5,1,2]], khớp đúng np.matmul thật. Bài này CHỈ forward -- _backward giữ mặc định lambda:None, bài sau hoàn thiện công thức đạo hàm."
locale: vi
track: tri-tue-nhan-tao
module: dong-co-tensor
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.nhan-ma-tran-forward]
requires: [ai.lop-tensor-co-ban]
concepts: [ai.nhan-ma-tran-forward]
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
`__mul__` nhân từng cặp phần tử tương ứng. Attention cần một phép nhân khác
hẳn — trộn thông tin CHÉO giữa hàng và cột. Đó là nhân ma trận.
::::

::::explain{#nhan_ma_tran_forward}
`__mul__` của bài trước đòi hai `Tensor` CÙNG shape, và nhân ĐỘC LẬP từng
cặp phần tử tương ứng — phần tử `(i, j)` của kết quả chỉ phụ thuộc phần tử
`(i, j)` của hai toán hạng, không phụ thuộc bất kỳ phần tử nào khác.

**Nhân ma trận** (`matmul`, ký hiệu `A @ B` trong Python/`numpy`) hoạt động
HOÀN TOÀN khác: phần tử `(i, j)` của kết quả là TỔNG của nhiều tích, trộn cả
MỘT HÀNG của `A` với MỘT CỘT của `B`:

> Với `A` shape `(m, k)` và `B` shape `(k, n)`, kết quả `C = A @ B` có shape
> `(m, n)`, và `C[i, j] = Σₜ A[i, t] · B[t, j]` (tổng qua `k` số hạng).

Điều kiện shape cũng khác hẳn `__mul__`: `matmul` KHÔNG đòi hai shape BẰNG
NHAU — nó đòi **chiều trong** khớp nhau (cột của `A` phải bằng hàng của
`B`, cùng là `k`), còn chiều ngoài (`m` và `n`) có thể khác nhau tuỳ ý. Đây
chính xác là lý do `matmul` phải là một PHƯƠNG THỨC RIÊNG (`Tensor.matmul`),
không thể ghép vào `__mul__` — hai phép toán có luật shape khác nhau, và
công thức tính từng phần tử cũng khác nhau (tích ĐỘC LẬP so với tích rồi
TỔNG CHÉO).

Bài này CHỈ dạy phần FORWARD — tính đúng `C = A @ B` bằng `numpy` (`self.data
@ other.data`, tương đương `np.matmul(self.data, other.data)`). `_backward`
của `out` giữ nguyên giá trị MẶC ĐỊNH từ `Tensor.__init__` — `lambda: None`
— nghĩa là gọi `.backward()` xuyên qua một `matmul` ở bài này CHƯA lan
gradient nào cả. Công thức đạo hàm cục bộ ĐÚNG của `matmul` (khác hẳn công
thức của `__mul__`) là việc của bài sau.
::::

::::example{#matmul_forward_that}
`A` shape `(2, 3)`, `B` shape `(3, 4)` — `C = A.matmul(B)` shape `(2, 4)`:

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
        # Bai nay CHI forward -- _backward giu mac dinh (lambda: None),
        # chua lan gradient nao qua matmul.
        return out


A = Tensor([[1., 0., -1.], [2., 1., 0.]])                 # shape (2, 3)
B = Tensor([[1., 2., 0., 1.], [0., 1., 1., 0.], [1., 0., 1., 2.]])  # shape (3, 4)
C = A.matmul(B)

print(A.data.shape, B.data.shape, C.data.shape)
print(np.round(C.data, 4).tolist())
```

```text title=readonly
(2, 3) (3, 4) (2, 4)
[[0.0, 2.0, -1.0, -1.0], [2.0, 5.0, 1.0, 2.0]]
```

Chiều TRONG của cả hai shape đều là `3` (cột của `A`, hàng của `B`) — khớp,
nên phép nhân hợp lệ. Kết quả `C` giữ lại chiều NGOÀI của mỗi bên: `2` hàng
(từ `A`) và `4` cột (từ `B`) — shape `(2, 4)`, KHÁC hẳn shape của `A` VÀ
shape của `B`. `C[0, 0] = 0.0` đến từ `1·1 + 0·0 + (-1)·1 = 0` — tổng của
`3` tích, đúng công thức `Σₜ A[0, t] · B[t, 0]`, không phải một tích đơn lẻ
như `__mul__` từng làm.
::::

::::example{#matmul_shape_khong_khop}
`matmul` đòi CHIỀU TRONG khớp nhau — không phải CÙNG shape như `__mul__`.
Thử nhân `A` (shape `(2, 3)`) với một ma trận có chiều trong SAI (`(4, 2)`
thay vì `(3, ?)`):

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


A = Tensor([[1., 0., -1.], [2., 1., 0.]])              # shape (2, 3)
D = Tensor([[1., 2.], [3., 4.], [5., 6.], [7., 8.]])   # shape (4, 2)

try:
    A.matmul(D)
    print("khong loi")
except ValueError:
    print("ValueError: chieu trong khong khop (3 != 4)")
```

```text title=readonly
ValueError: chieu trong khong khop (3 != 4)
```

`A` có `3` cột, `D` có `4` hàng — hai con số này PHẢI khớp cho `matmul`
(cột của toán hạng trái = hàng của toán hạng phải), và `3 ≠ 4`. `numpy` ném
`ValueError` NGAY tại bước forward — khác hẳn ví dụ shape-khác-nhau của
`__mul__` (bài trước), nơi forward chạy được (nhờ broadcast) nhưng
`_backward` mới vỡ. Với `matmul`, luật shape RIÊNG của chính nó chặn ngay từ
đầu.
::::

::::predict{#doan_shape_matmul commitOnce}
Giả sử `X` shape `(5, 7)` và `Y` shape `(7, 3)`.

**Trước khi tính**, bạn đoán: `X.matmul(Y)` cho kết quả shape gì?

:::opt{correct}
`(5, 3)` — chiều trong (`7`) của cả hai khớp nhau nên phép nhân hợp lệ; kết
quả giữ lại chiều ngoài của `X` (`5`, số hàng) làm số hàng, và chiều ngoài
của `Y` (`3`, số cột) làm số cột
:::

:::opt
`(7, 7)` — vì chiều TRONG (`7`) là con số chung của cả hai shape, kết quả
phải mang chính con số đó
::why
Gần đúng ở việc để ý đúng con số `7` là chiều QUYẾT ĐỊNH phép nhân có hợp lệ
hay không — quan sát về vai trò của nó không sai.

Chỗ lệch: chiều `7` chỉ là điều kiện KHỚP NHAU giữa hai toán hạng — nó bị
"tiêu thụ" trong phép tính tổng (`Σₜ`, tổng qua đúng `t=1..7`) và KHÔNG xuất
hiện trong shape kết quả. Shape kết quả giữ lại hai chiều NGOÀI (`5` từ `X`,
`3` từ `Y`), không phải chiều trong.
::
:::

:::opt
`(5, 7, 3)` — kết quả matmul giữ lại CẢ BA con số liên quan (`5`, `7`, `3`),
không bỏ con số nào
::why
Gần đúng ở việc cả ba con số `5`, `7`, `3` đều THAM GIA vào phép tính — quan
sát đó không sai.

Chỗ lệch: "tham gia vào phép tính" không có nghĩa là "xuất hiện trong shape
kết quả". Con số `7` (chiều trong) chỉ dùng để XÁC ĐỊNH số hạng cần cộng ở
mỗi phần tử kết quả — nó không phải một CHIỀU của kết quả, nên kết quả chỉ
có `2` chiều (`5` và `3`), không phải `3` chiều.
::
:::
::::

::::code{#viet_matmul_forward}
Hoàn thiện `matmul`: tính `out` bằng phép nhân ma trận `numpy` giữa
`self.data` và `other.data`.

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
        out = Tensor(___, (self, other), '@')     # self.data @ other.data
        return out


A = Tensor([[1., 0., -1.], [2., 1., 0.]])
B = Tensor([[1., 2., 0., 1.], [0., 1., 1., 0.], [1., 0., 1., 2.]])
C = A.matmul(B)

print(C.data.shape)
print(np.round(C.data, 4).tolist())
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


A = Tensor([[1., 0., -1.], [2., 1., 0.]])
B = Tensor([[1., 2., 0., 1.], [0., 1., 1., 0.], [1., 0., 1., 2.]])
C = A.matmul(B)

print(C.data.shape)
print(np.round(C.data, 4).tolist())
```

```python title=test
import numpy as np

assert tuple(C.data.shape) == (2, 4), f"C.data.shape sai -- dang ra {tuple(C.data.shape)}"
assert np.round(C.data, 4).tolist() == [[0.0, 2.0, -1.0, -1.0], [2.0, 5.0, 1.0, 2.0]], f"C.data sai -- dang ra {np.round(C.data, 4).tolist()}"

# rieng kiem tra CONG THUC THAT (khong chep san), tren mot cap ma tran KHAC
D1 = Tensor([[2., 1.], [0., 3.]])          # shape (2, 2)
D2 = Tensor([[1., 0., 2.], [1., 1., 0.]])  # shape (2, 3)
E = D1.matmul(D2)
assert tuple(E.data.shape) == (2, 3), f"shape tren cap ma tran khac sai -- dang ra {tuple(E.data.shape)}"
assert np.round(E.data, 4).tolist() == [[3.0, 1.0, 4.0], [3.0, 3.0, 0.0]], f"gia tri tren cap ma tran khac sai -- dang ra {np.round(E.data, 4).tolist()}"
```

:::hints
- kind: attention
  body: Một chỗ trống. `out` phải mang giá trị của phép NHÂN MA TRẬN giữa `self.data` và `other.data` — dùng toán tử `@` của `numpy` (`self.data @ other.data`), KHÔNG phải `*` (đó là element-wise, bài trước).
- kind: strategy
  body: '`self.data @ other.data`.'
- kind: one-line
  body: 'Chỗ trống là `self.data @ other.data`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: matmul phai dung toan tu @ (nhan ma tran THAT) giua self.data va other.data, khong duoc dung * (element-wise) hay chep san gia tri
  requireAst:
  - kind: uses-name, target: self, min: 8
  - kind: uses-name, target: other, min: 2
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # harness A/B/C): loi giai dung dat=true. self=8: sau lan trong __init__
  # (moi dong `self.xxx = ...` dem CA self cua Load lan cho attribute do,
  # vi Python AST luon Load `self` truoc khi Store vao thuoc tinh cua no),
  # cong hai lan trong `matmul` (self.data, va self trong tuple (self,
  # other)). other=2: other.data va other trong tuple (self, other), cả hai
  # trong `matmul` (khong co "other" nao trong __init__). Khong dung kind
  # "uses-operator target=@" duoc vi @ (MatMult) khong nam trong bang
  # _TOAN_TU cua kiem-ast.ts (chi co +,-,*,/,//,%,**, so sanh, is/in) --
  # kiemAst se NEM loi "truy van AST khong co that" neu dung sai target
  # nay, nen luat o day dua vao uses-name self/other thay the. Cheat "return
  # Tensor(np.zeros((2, 4)), (self, other), '@')" (chep san mot mang 0
  # dung shape, bo ca self.data lan other.data) lam self tut xuong 7 VA
  # other tut xuong 1 -- bi chan KEP boi ca hai luat. Cheat "self.data *
  # other.data" (dung element-wise thay vi matmul that) giu NGUYEN so dem
  # self/other (khong doi ten bien nao, chi doi toan tu) nen KHONG bi static
  # bat rieng -- nhung da tu kiem chung bang Python that: voi A shape (2,3)
  # va B shape (3,4), "self.data * other.data" NEM ValueError ngay (khong
  # broadcast duoc, 2x3 khac 3x4) -- bi chan boi tier `run`, truoc ca khi
  # kip toi tier tests hay output. Rieng cheat gia tri sai (vi du hoan doi
  # A/B) da tu kiem chung se bi bat boi assert gia tri tren cap D1/D2 khac
  # trong tier tests, doc lap voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\(2, 4\\)\\n\\[\\[0\\.0, 2\\.0, -1\\.0, -1\\.0\\], \\[2\\.0, 5\\.0, 1\\.0, 2\\.0\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Forward xong — `C = A @ B` đúng shape, đúng giá trị. Nhưng `.backward()`
qua nó vẫn chưa lan gradient nào. Bài sau: công thức đạo hàm THẬT của
`matmul`, kiểm bằng finite-difference trước khi tin.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`__mul__` (bài trước) có công thức đạo hàm cục bộ ĐƠN GIẢN: `self.grad +=
other.data * out.grad` — vì mỗi phần tử của kết quả chỉ phụ thuộc ĐÚNG một
phần tử tương ứng của mỗi toán hạng. Với `matmul`, MỘT phần tử của `A` (ví
dụ `A[0, 0]`) ảnh hưởng tới BAO NHIÊU phần tử của `C`? Công thức đạo hàm cục
bộ của `matmul` có thể đơn giản như `__mul__` được không?
::::

::::checkpoint{mastery=0.8}
::::
