---
id: tri-tue-nhan-tao.dao-ham-tu-dong.lop-value-mu-tanh-relu
title: "Lớp Value: mũ, tanh, ReLU"
summary: "Mở rộng Value với __pow__ (số mũ hằng), tanh(), relu() — tái dùng đúng công thức đạo hàm đã học ở ham-kich-hoat (tanh'=1-tanh(z)², ReLU'=1 khi z>0 else 0). Trên h=a**2, t=h.tanh() (a=0.5): a.grad≈0.940015. Biên z=0 của relu xác nhận dùng đúng '>' không phải '>=' — relu_dao_ham(0.0) phải ra 0.0."
locale: vi
track: tri-tue-nhan-tao
module: dao-ham-tu-dong
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.lop-value-mu-tanh-relu]
requires: [ai.lop-value-cong-tru-nhan]
concepts: [ai.lop-value-mu-tanh-relu]
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
Ba phép toán số học chưa đủ để dựng một neuron. Còn thiếu luỹ thừa (cho các
công thức có bình phương) và hàm kích hoạt — `tanh`, `ReLU`.
::::

::::explain{#mu_tanh_relu}
Ba phương thức mới cho `Value`, mỗi cái tự biết đạo hàm cục bộ của MÌNH,
đúng cách `__add__`/`__sub__`/`__mul__` đã làm ở bài trước:

> **`__pow__`** (`out = self ** n`, `n` là một số mũ HẰNG, không phải một
> `Value` khác): đạo hàm cục bộ `∂out/∂self = n·self.data^(n-1)` — quy tắc
> luỹ thừa quen thuộc từ giải tích.
>
> **`tanh()`** (`out = tanh(self.data)`): đạo hàm cục bộ
> `∂out/∂self = 1 - tanh(self.data)²`. Đây KHÔNG phải công thức mới — bài
> `ham-kich-hoat` (q8.2a) đã đo chính xác công thức này (`tanh_dao_ham(z) =
> 1 - tanh(z)²`) khi khảo sát bão hoà. Ở đây chỉ TÁI DÙNG, không phát minh
> lại.
>
> **`relu()`** (`out = max(0, self.data)`): đạo hàm cục bộ là `1` khi
> `self.data > 0`, và `0` khi `self.data ≤ 0` — đúng công thức
> `relu_dao_ham` của bài `ham-kich-hoat`, kể cả điều kiện biên: dùng `>`
> (không phải `>=`), nên tại đúng `self.data = 0`, đạo hàm cục bộ là `0`.

Ba công thức này CHƯA được gọi qua một `.backward()` toàn cục (vẫn chưa xây
tới bài `sap-xep-to-po-va-lan-truyen-nguoc`) — ở đây, lan truyền ngược vẫn
gọi TAY, từng `_backward()` một, đúng thứ tự ngược của phép tính.
::::

::::example{#mu_va_tanh_tay}
`h = a**2`, `t = h.tanh()`, với `a = 0.5` — lan truyền ngược gọi tay:

```python title=readonly
import math

class Value:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += (other * self.data ** (other - 1)) * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
        out._backward = _backward
        return out

a = Value(0.5)
h = a ** 2
t = h.tanh()

t.grad = 1.0
t._backward()
h._backward()

print(round(h.data, 6), round(t.data, 6))
print(round(a.grad, 6))
```

```text title=readonly
0.25 0.244919
0.940015
```

`h = a**2` cho đúng `0.25` (`0.5` bình phương). `t = tanh(0.25) ≈ 0.244919`.
Sau khi gán `t.grad = 1.0` và lan ngược qua `t._backward()` rồi
`h._backward()`, `a.grad ≈ 0.940015` — kết quả của HAI đạo hàm cục bộ nhân
lại: đạo hàm của `tanh` tại `h=0.25` (khoảng `0.940015`, vì `1 − 0.244919²`
xấp xỉ đúng con số đó), nhân với đạo hàm của luỹ thừa tại `a=0.5` — với số mũ
`2`, quy tắc luỹ thừa cho hệ số nhân `2·a`, và tại `a=0.5` hệ số đó ĐÚNG
BẰNG `1`, không đổi độ lớn — nên `a.grad` khớp gần đúng với `h.grad` (chain
rule qua một bước có đạo hàm cục bộ bằng `1` thì không đổi giá trị lan qua).
::::

::::example{#relu_hai_nhanh}
`relu()` trên ba giá trị khác nhau — âm, dương, và ĐÚNG biên `0` — mỗi lần
gán `grad = 1.0` cho kết quả rồi gọi `_backward()` của chính nó:

```python title=readonly
class Value:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def relu(self):
        out = Value(0.0 if self.data < 0 else self.data, (self,), 'relu')
        def _backward():
            self.grad += (1.0 if self.data > 0 else 0.0) * out.grad
        out._backward = _backward
        return out

for x in (-3.0, 3.0, 0.0):
    a = Value(x)
    r = a.relu()
    r.grad = 1.0
    r._backward()
    print(x, "->", r.data, a.grad)
```

```text title=readonly
-3.0 -> 0.0 0.0
3.0 -> 3.0 1.0
0.0 -> 0.0 0.0
```

Nhánh ÂM (`x=-3.0`): `relu` cắt về `0.0`, và đạo hàm cục bộ cũng `0.0` —
không lan gradient nào ngược qua nhánh này. Nhánh DƯƠNG (`x=3.0`): `relu`
giữ nguyên giá trị, đạo hàm `1.0` — lan gradient nguyên vẹn. Nhánh BIÊN
(`x=0.0`, đúng ranh giới): đạo hàm ra `0.0`, vì điều kiện dùng `>` — nghiêm
ngặt, không tính `0.0` là "dương".
::::

::::predict{#doan_bien_relu commitOnce}
`relu_dao_ham` dùng điều kiện `self.data > 0` (nghiêm ngặt).

**Trước khi đọc lại**, bạn đoán: nếu đổi điều kiện đó thành `self.data >=
0`, kết quả `a.grad` tại đúng `x = 0.0` (ví dụ ở trên) sẽ đổi từ `0.0` thành
gì?

:::opt{correct}
Thành `1.0` — điều kiện `>= 0` coi `0.0` là "không âm", nên rơi vào nhánh
"đạo hàm bằng `1`" thay vì nhánh "đạo hàm bằng `0`"
:::

:::opt
Vẫn là `0.0` — đổi `>` thành `>=` không ảnh hưởng gì tới trường hợp
`self.data` đúng bằng `0`
::why
Gần đúng ở trực giác rằng nhiều thay đổi điều kiện KHÔNG ảnh hưởng gì tới
kết quả cuối — điều đó đúng ở NHIỀU tình huống khác.

Chỗ lệch: `>` và `>=` chỉ khác nhau đúng MỘT trường hợp — khi giá trị so
sánh bằng CHÍNH xÁC ranh giới (`0`). Đây chính là trường hợp đó: `self.data
= 0.0` đúng bằng ranh giới, nên đổi `>` thành `>=` đổi hẳn nhánh nào được
chọn, từ `0.0` sang `1.0`.
::
:::

:::opt
Không xác định được — kết quả phụ thuộc vào cách Python làm tròn số `0.0`
::why
Gần đúng ở việc số thực dấu phẩy động đôi khi có những bất ngờ khi so sánh —
sự cẩn trọng đó không sai ở nhiều ngữ cảnh khác.

Chỗ lệch: `0.0` là một giá trị CHÍNH XÁC, biểu diễn được tuyệt đối trong dấu
phẩy động (không như `0.1`) — không có sai số làm tròn nào ở đây. So sánh
`0.0 > 0` và `0.0 >= 0` cho kết quả HOÀN TOÀN xác định: `False` và `True`.
::
:::
::::

::::code{#hoan_thien_mu_tanh_relu}
Hoàn thiện ba closure `_backward`: `__pow__` (quy tắc luỹ thừa), `tanh`
(công thức đã học ở `ham-kich-hoat`), và `relu` (chú ý biên: dùng `>`, không
phải `>=`).

```python title=starter
import math

class Value:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += ___             # (other * self.data ** (other - 1)) * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += ___             # (1 - t ** 2) * out.grad
        out._backward = _backward
        return out

    def relu(self):
        out = Value(0.0 if self.data < 0 else self.data, (self,), 'relu')
        def _backward():
            self.grad += ___             # (1.0 if self.data > 0 else 0.0) * out.grad
        out._backward = _backward
        return out

a = Value(0.5)
h = a ** 2
t = h.tanh()
t.grad = 1.0
t._backward()
h._backward()

a2 = Value(-3.0)
r2 = a2.relu()
r2.grad = 1.0
r2._backward()

a3 = Value(3.0)
r3 = a3.relu()
r3.grad = 1.0
r3._backward()

a4 = Value(0.0)
r4 = a4.relu()
r4.grad = 1.0
r4._backward()

print(round(h.data, 6), round(t.data, 6))
print(round(a.grad, 6))
print(round(a2.grad, 4), round(a3.grad, 4), round(a4.grad, 4))
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

    def __pow__(self, other):
        assert isinstance(other, (int, float))
        out = Value(self.data ** other, (self,), f'**{other}')
        def _backward():
            self.grad += (other * self.data ** (other - 1)) * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
        out._backward = _backward
        return out

    def relu(self):
        out = Value(0.0 if self.data < 0 else self.data, (self,), 'relu')
        def _backward():
            self.grad += (1.0 if self.data > 0 else 0.0) * out.grad
        out._backward = _backward
        return out

a = Value(0.5)
h = a ** 2
t = h.tanh()
t.grad = 1.0
t._backward()
h._backward()

a2 = Value(-3.0)
r2 = a2.relu()
r2.grad = 1.0
r2._backward()

a3 = Value(3.0)
r3 = a3.relu()
r3.grad = 1.0
r3._backward()

a4 = Value(0.0)
r4 = a4.relu()
r4.grad = 1.0
r4._backward()

print(round(h.data, 6), round(t.data, 6))
print(round(a.grad, 6))
print(round(a2.grad, 4), round(a3.grad, 4), round(a4.grad, 4))
```

```python title=test
assert round(h.data, 6) == 0.25, f"h.data sai -- dang ra {round(h.data, 6)}"
assert round(t.data, 6) == 0.244919, f"t.data sai -- dang ra {round(t.data, 6)}"
assert round(a.grad, 6) == 0.940015, f"a.grad sai -- dang ra {round(a.grad, 6)}"
assert round(a2.grad, 4) == 0.0, f"a2.grad (nhanh am) sai -- dang ra {round(a2.grad, 4)}"
assert round(a3.grad, 4) == 1.0, f"a3.grad (nhanh duong) sai -- dang ra {round(a3.grad, 4)}"

# rieng kiem tra BIEN z=0: phai dung '>' khong phai '>=' -- neu doi thanh
# '>=', a4.grad se ra 1.0 thay vi 0.0. Da xac nhan bang Pyodide that.
assert round(a4.grad, 4) == 0.0, f"a4.grad (bien z=0) phai la 0.0 (dung '>', khong phai '>=') -- dang ra {round(a4.grad, 4)}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. `__pow__`: quy tắc luỹ thừa `n·x^(n-1)`, viết bằng `other * self.data ** (other - 1)`, nhân thêm `out.grad` để lan tiếp. `tanh`: công thức `1 - tanh(z)²` đã học ở `ham-kich-hoat`, dùng biến `t` đã tính sẵn ở dòng trên (không gọi lại `math.tanh`). `relu`: điều kiện `self.data > 0` (không phải `>=`) — giống hệt `relu_dao_ham` của `ham-kich-hoat`.
- kind: strategy
  body: '__pow__: `(other * self.data ** (other - 1)) * out.grad`. tanh: `(1 - t ** 2) * out.grad`. relu: `(1.0 if self.data > 0 else 0.0) * out.grad`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `(other * self.data ** (other - 1)) * out.grad`, `(1 - t ** 2) * out.grad`, và `(1.0 if self.data > 0 else 0.0) * out.grad`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: __pow__ phai dung THAT quy tac luy thua (** voi so mu other-1); tanh phai dung bien t da tinh san (1 - t**2); relu phai so sanh THAT self.data voi 0 bang toan tu > (khong duoc dung >= hay chep san mang ket qua)
  requireAst:
  - kind: uses-operator, target: "**", min: 4
  - kind: uses-operator, target: ">", min: 1
  - kind: uses-name, target: t, min: 5
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai dung
  # dat=true, ca ba luat qua sach ("**"=4: self.data**other khi tao out trong
  # __pow__, self.data**(other-1) trong backward, t**2 trong tanh, a**2 o
  # duoi; ">"=1: rieng trong relu's backward; t=5: Value(t,...), t**2, va ba
  # lan doc t o phan harness duoi). Cheat "__pow__ chep san 2*a.data thay vi
  # dung **" lam "**" tut xuong 3 -- bi chan. Cheat "relu dung >= thay vi >"
  # lam ">" ve 0 -- bi chan TINH (static) VA bi chan boi assert a4.grad rieng
  # (tier tests, doc lap voi static).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^0\\.25 0\\.244919\\n0\\.940015\\n0\\.0 1\\.0 0\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu phép toán, sáu công thức đạo hàm cục bộ — đủ để dựng bất kỳ biểu thức
nào một neuron cần. Còn thiếu đúng một mảnh: cách TỰ ĐỘNG tìm đúng thứ tự
gọi `_backward()`, thay vì gọi tay như hai bài vừa qua.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai bài vừa qua đều phải TỰ TAY liệt kê đúng thứ tự gọi `_backward()` (ví
dụ: `f` trước `e`, hay `t` trước `h`). Với một biểu thức chỉ vài node, việc
đó không khó nhớ. Cái THỨ TỰ đó — node được TÍNH SAU CÙNG (gần đầu ra nhất)
luôn lan gradient TRƯỚC — có tên gọi trong lý thuyết đồ thị không, và làm
sao một thuật toán có thể tự tìm ra nó cho một đồ thị BẤT KỲ, không cần một
người liệt kê tay?
::::

::::checkpoint{mastery=0.8}
::::
