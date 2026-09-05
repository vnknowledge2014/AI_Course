---
id: tri-tue-nhan-tao.huan-luyen-mang-no-ron.neuron-tu-autograd
title: "Neuron chạy qua autograd"
summary: "Ráp lại Layer/MLP của q8.2a nhưng W, b giờ là Value (không phải số Python trần), và Layer.forward gọi các operator ĐÃ OVERLOAD của Value — forward pass tự xây đồ thị tính toán. Neuron 2 đầu vào (w1=0.5, w2=-0.5, b=0.1, tanh) cho out=-0.3799, backward() cho w1.grad=0.8556, w2.grad=1.7113, b.grad=0.8556 — không số nào bằng 0 (test đứt-đồ-thị). MLP 2-2-1 xác nhận CẢ CHÍN tham số (sáu W, ba b) đều có grad khác 0 sau một backward() duy nhất."
locale: vi
track: tri-tue-nhan-tao
module: huan-luyen-mang-no-ron
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.neuron-tu-autograd]
requires: [ai.boss-dao-ham-tu-dong]
concepts: [ai.neuron-tu-autograd]
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
Hai quest đã xây xong hai nửa: `Layer`/`MLP` biết forward pass (q8.2a), `Value`
biết tính đạo hàm (q8.2b). Trọng số của `Layer` xưa nay vẫn là số Python trần
— chưa một lần được `Value` chạm tới. Bài này ráp hai nửa lại làm một.
::::

::::explain{#rap-value-vao-layer}
`Layer`/`MLP` của `q8.2a` tính `Z = X @ W + b` bằng `numpy`, trên `W`, `b` là
những mảng số thực trần — không có gì để gọi `.backward()` lên cả, vì không
có đồ thị tính toán nào được xây trong lúc `forward`. `Value` của `q8.2b`
biết xây đồ thị VÀ tính đạo hàm ngược — nhưng nó chỉ là một con số duy nhất,
chưa từng ráp vào cấu trúc nhiều tầng nào.

Ráp hai thứ lại chỉ cần MỘT thay đổi, nhưng thay đổi đó phải NHẤT QUÁN ở
MỌI nơi:

> **Mọi `W`, mọi `b` của một `Layer` giờ là các đối tượng `Value`** (không
> phải số Python trần), và **`Layer.forward` phải tính bằng các operator ĐÃ
> OVERLOAD của `Value`** (`+`, `*` — những gì `Value.__add__`, `Value.__mul__`
> đã định nghĩa từ `q8.2b`), **không phải `numpy`/`@`**.

Vì `Value` không có phép nhân ma trận, `forward` của một `Layer` giờ viết
bằng một vòng lặp tường minh trên TỪNG neuron, TỪNG trọng số — không còn
`X @ W + b` gọn một dòng nữa: với neuron thứ `j`, `z = b[j]`, rồi CỘNG DẦN
`W[j][i] * x[i]` cho từng đầu vào `i`. Đây không phải một bước lùi — đây
chính là cách duy nhất để MỌI phép nhân, MỌI phép cộng đều đi QUA `Value`,
để `.backward()` sau đó lan gradient được tới từng `W[j][i]`, từng `b[j]`.

Hệ quả trực tiếp: **một phép tính lỡ dùng số Python trần xen giữa (thay vì
gọi qua operator của `Value`) sẽ làm đồ thị "đứt" tại đúng điểm đó** —
gradient không lan qua được, và tham số ở phía trước điểm đứt sẽ có
`grad = 0.0` sau `backward()`, dù forward pass vẫn ra đúng số. Đây là gotcha
lớn nhất của bài này: **đúng SỐ không có nghĩa là đúng ĐỒ THỊ**.
::::

::::example{#neuron_qua_value}
Một neuron duy nhất, hai đầu vào, kích hoạt `tanh` — MỌI thứ (`w1`, `w2`,
`b`, và cả `x1`, `x2`) đều là `Value`, và phép tính chỉ dùng `+`, `*`,
`.tanh()` đã có sẵn:

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

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
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


w1 = Value(0.5)
w2 = Value(-0.5)
b = Value(0.1)
x1 = Value(1.0)
x2 = Value(2.0)

z = w1 * x1 + w2 * x2 + b
out = z.tanh()
out.backward()

print(round(z.data, 4))
print(round(out.data, 4))
print(round(w1.grad, 4), round(w2.grad, 4), round(b.grad, 4))
```

```text title=readonly
-0.4
-0.3799
0.8556 1.7113 0.8556
```

`z = w1*x1 + w2*x2 + b` — ba phép toán (`*`, `*`, `+`, `+`), MỖI phép đi
qua đúng operator đã overload của `Value`. Gọi `out.backward()` MỘT LẦN
DUY NHẤT, và cả ba tham số (`w1`, `w2`, `b`) đều nhận được gradient khác
`0`. Đây chính là test "đứt đồ thị": nếu `Layer.forward` viết sai — lỡ dùng
`self.W[j][i].data` (số trần) thay vì `self.W[j][i]` (Value) ở đâu đó —
forward pass vẫn ra `-0.3799` (numpy hay Value tính ra cùng con số), nhưng
gradient của tham số đó sẽ tuyệt đối bằng `0.0` sau `backward()`, vì
`backward()` không có đường nào để tới được nó.
::::

::::predict{#doan_goi_lai_khong_reset commitOnce}
`w1`, `w2`, `b` ở trên là những đối tượng `Value` TỒN TẠI LÂU DÀI — không bị
tạo mới mỗi lần gọi `forward`. Sau đoạn code trên, `w1.grad = 0.8556`.

**Trước khi chạy thử**, bạn đoán: nếu gọi LẠI đúng hai dòng `z = w1*x1 +
w2*x2 + b` và `out = z.tanh()` rồi `out.backward()` một lần NỮA — dùng lại
CHÍNH `w1`, `w2`, `b`, `x1`, `x2` đó, KHÔNG đổi giá trị nào, không reset
`grad` — `w1.grad` sẽ là bao nhiêu?

:::opt{correct}
`1.7113` — đúng gấp đôi `0.8556` — vì đồ thị lần thứ hai giống HỆT đồ thị
lần thứ nhất (không tham số nào đổi giá trị), nên gradient cục bộ tính ra
cũng giống hệt lần trước; `Value.grad` cộng dồn (`+=`), không tự reset, nên
hai đóng góp giống hệt nhau cộng lại thành đúng gấp đôi
:::

:::opt
Vẫn `0.8556` — vì `backward()` luôn tính ra đúng gradient CỦA đồ thị hiện
tại, không quan tâm nó đã được gọi bao nhiêu lần trước đó
::why
Gần đúng ở việc `backward()` THẬT SỰ tính đúng gradient của đồ thị vừa xây
— mỗi lần gọi, `_backward()` của từng node vẫn cộng đúng công thức đạo hàm
cục bộ, không có gì "hỏng" ở phép tính đó.

Chỗ lệch: "tính đúng gradient của đồ thị hiện tại" và "kết quả cuối cùng
lưu trong `w1.grad`" là hai thứ khác nhau. `self.grad += ...` (bài
`tich-luy-gradient`, q8.2b) CỘNG DỒN vào bất cứ gì đã có sẵn trong
`w1.grad` — mà sau lần gọi thứ nhất, `w1.grad` đã là `0.8556`, không phải
`0.0`. Lần gọi thứ hai cộng thêm đúng `0.8556` nữa vào đó.
::
:::

:::opt
Không xác định được nếu không biết `Value` xử lý việc gọi `backward()`
nhiều lần trên cùng một tập node ra sao — có thể là lỗi runtime
::why
Gần đúng ở sự thận trọng — gọi lại một cấu trúc dữ liệu đã dùng rồi đúng là
một thói quen cần kiểm tra kỹ ở nhiều thư viện.

Chỗ lệch: `Value` không có gì đặc biệt để "khoá" một node lại sau khi
`backward()` chạy xong — `w1`, `w2`, `b`, `x1`, `x2` vẫn là những `Value`
bình thường, dùng lại được ở bất kỳ phép toán nào, kể cả lặp lại đúng phép
toán cũ. Không có ngoại lệ hay lỗi nào xảy ra — chỉ có `self.grad += ...`
cộng dồn đúng như mọi lần khác.
::
:::
::::

::::code{#rap_layer_mlp_qua_value}
Hoàn thiện `Layer.forward`: `z` bắt đầu từ `self.b[j]`, CỘNG DẦN từng
`self.W[j][i] * x[i]` (không phải `numpy`/`@`); `MLP.forward` đưa `a` qua
từng tầng. Chạy một MLP `2-2-1` (tầng ẩn `2` neuron, tầng ra `1` neuron,
đều `tanh`), gọi `backward()`, xác nhận CẢ CHÍN tham số có gradient khác
`0`.

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

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
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


def tanh_kh(z):
    return z.tanh()

class Layer:
    def __init__(self, W, b, kich_hoat):
        self.W = W              # danh sach CAC danh sach Value: W[j][i]
        self.b = b               # danh sach Value, do dai = so neuron ra
        self.kich_hoat = kich_hoat

    def forward(self, x):
        ra = []
        for j in range(len(self.b)):
            z = self.b[j]
            for i in range(len(x)):
                z = ___                   # z + self.W[j][i] * x[i]
            ra.append(___)                # self.kich_hoat(z)
        return ra

class MLP:
    def __init__(self, cac_tang):
        self.cac_tang = cac_tang

    def forward(self, x):
        a = x
        for tang in self.cac_tang:
            a = ___                       # tang.forward(a)
        return a

W1 = [[Value(0.5), Value(-0.3)], [Value(0.2), Value(0.4)]]
b1 = [Value(0.1), Value(-0.1)]
W2 = [[Value(0.6), Value(-0.7)]]
b2 = [Value(0.05)]

mlp = MLP([Layer(W1, b1, tanh_kh), Layer(W2, b2, tanh_kh)])
x = [Value(1.0), Value(-2.0)]
out = mlp.forward(x)
out[0].backward()

tat_ca_tham_so = [p for hang in W1 for p in hang] + b1 + [p for hang in W2 for p in hang] + b2
so_khac_0 = sum(1 for p in tat_ca_tham_so if abs(p.grad) > 1e-9)

print(round(out[0].data, 4))
print(so_khac_0 == len(tat_ca_tham_so))
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

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

    def tanh(self):
        t = math.tanh(self.data)
        out = Value(t, (self,), 'tanh')
        def _backward():
            self.grad += (1 - t ** 2) * out.grad
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


def tanh_kh(z):
    return z.tanh()

class Layer:
    def __init__(self, W, b, kich_hoat):
        self.W = W
        self.b = b
        self.kich_hoat = kich_hoat

    def forward(self, x):
        ra = []
        for j in range(len(self.b)):
            z = self.b[j]
            for i in range(len(x)):
                z = z + self.W[j][i] * x[i]
            ra.append(self.kich_hoat(z))
        return ra

class MLP:
    def __init__(self, cac_tang):
        self.cac_tang = cac_tang

    def forward(self, x):
        a = x
        for tang in self.cac_tang:
            a = tang.forward(a)
        return a

W1 = [[Value(0.5), Value(-0.3)], [Value(0.2), Value(0.4)]]
b1 = [Value(0.1), Value(-0.1)]
W2 = [[Value(0.6), Value(-0.7)]]
b2 = [Value(0.05)]

mlp = MLP([Layer(W1, b1, tanh_kh), Layer(W2, b2, tanh_kh)])
x = [Value(1.0), Value(-2.0)]
out = mlp.forward(x)
out[0].backward()

tat_ca_tham_so = [p for hang in W1 for p in hang] + b1 + [p for hang in W2 for p in hang] + b2
so_khac_0 = sum(1 for p in tat_ca_tham_so if abs(p.grad) > 1e-9)

print(round(out[0].data, 4))
print(so_khac_0 == len(tat_ca_tham_so))
```

```python title=test
assert round(out[0].data, 4) == 0.7501, f"out[0].data sai -- dang ra {round(out[0].data, 4)}"
assert len(tat_ca_tham_so) == 9, f"MLP 2-2-1 phai co dung 9 tham so (4 W1 + 2 b1 + 2 W2 + 1 b2) -- dang ra {len(tat_ca_tham_so)}"

# gotcha trung tam cua bai: DUT DO THI. Neu Layer.forward lo dung so tran
# (vd tao lai Value(z.data) o giua, hoac nhan bang .data) o bat ky diem nao,
# tham so PHIA TRUOC diem do se co grad=0.0 dung tuyet doi, du out[0].data
# van dung 0.7501 het suc binh thuong -- da tu kiem chung bang Python that
# (cheat "z = Value(z.data)" truoc khi goi kich_hoat cho CA CHIN grad ve
# dung 0.0, trong khi out[0].data khong doi).
assert so_khac_0 == len(tat_ca_tham_so), f"MOI tham so phai co gradient KHAC 0 sau MOT backward() duy nhat (dau hieu dut do thi neu khong) -- co {len(tat_ca_tham_so) - so_khac_0}/{len(tat_ca_tham_so)} tham so dang grad=0"

# rieng kiem tra TUNG tham so mot, khong chi dem tong -- cu the hoa gotcha
cac_ten = ["W1[0][0]", "W1[0][1]", "W1[1][0]", "W1[1][1]", "b1[0]", "b1[1]", "W2[0][0]", "W2[0][1]", "b2[0]"]
for ten, p in zip(cac_ten, tat_ca_tham_so):
    assert abs(p.grad) > 1e-9, f"{ten}.grad phai KHAC 0 sau backward() -- dang ra {p.grad} (dau hieu dut do thi tai tham so nay)"
```

:::hints
- kind: attention
  body: Ba chỗ trống. `Layer.forward`, `z`: cộng DẦN, mỗi vòng lặp `i` cộng thêm đúng MỘT số hạng — `z + self.W[j][i] * x[i]` (chú ý thứ tự nhân: `self.W[j][i]` — một `Value` — đứng TRƯỚC `x[i]`, vì `Value` không có `__rmul__`). `ra.append`: gọi hàm kích hoạt ĐÃ LƯU, `self.kich_hoat(z)`, không viết cứng `z.tanh()`. `MLP.forward`, `a`: kết quả `forward` của tầng HIỆN TẠI áp lên `a` — `tang.forward(a)`.
- kind: strategy
  body: 'z: `z + self.W[j][i] * x[i]`. append: `self.kich_hoat(z)`. a (trong vòng lặp MLP): `tang.forward(a)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `z + self.W[j][i] * x[i]`, `self.kich_hoat(z)`, và `tang.forward(a)`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: Layer.forward phai cong DAN tung self.W[j][i] * x[i] vao z (khong duoc bo qua trong so, khong duoc dung numpy/@); phai goi THAT self.kich_hoat (khong viet cung tanh); MLP.forward phai goi THAT tang.forward trong vong lap
  requireAst:
  - kind: uses-operator, target: "*", min: 5
  - kind: uses-call, target: kich_hoat, min: 1
  - kind: uses-call, target: forward, min: 2
  - kind: uses-name, target: tang, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach ("*"=5: self.data*other.data trong
  # __mul__ [dinh nghia], hai cong thuc trong _backward cua __mul__, va
  # self.W[j][i] * x[i] trong Layer.forward -- xac nhan lai bang kiemAst
  # that o buoc tu-verify cuoi bai).
  # Cheat "Layer.forward bo qua trong so, chi dung bias" (z = self.b[j] roi
  # goi kich_hoat ngay, khong cong W[j][i]*x[i]) lam "*" tut xuong 4 -- bi
  # chan. Cheat "ra.append(z) bo qua kich_hoat" lam "kich_hoat" ve 0 -- bi
  # chan CA static LAN output (gia tri sai vi thieu tanh).
  #
  # GOTCHA DUT DO THI: KHONG bat duoc boi static (AST khong biet "z =
  # Value(z.data)" la mot phep cat do thi hop le ve mat cu phap) -- day la
  # ly do PHAI co assertion rieng o tier tests, da tu kiem chung bang Python
  # that: cheat "z = Value(z.data) truoc khi goi kich_hoat" van cho
  # out[0].data = 0.7501 (dung nguyen, static/output khong phan biet duoc)
  # nhung ca CHIN grad deu ve dung 0.0 -- CHI assertion so_khac_0 ==
  # len(tat_ca_tham_so) (va vong lap tung tham so ben duoi) bat duoc.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^0\\.7501\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chín tham số, chín gradient khác `0`, từ đúng MỘT lần gọi `backward()`.
`Layer`/`MLP` giờ chạy xuyên qua `Value` — forward pass tự xây đồ thị,
không cần viết tay `_backward` cho từng mạng nữa.
::::

::::reflect{#nghi-lai}
`Layer`/`MLP` giờ tính bằng `Value` thay vì `numpy` — cùng cấu trúc hai
class của `q8.2a`, chỉ khác NGUYÊN LIỆU bên trong. Forward pass tự động
xây đồ thị tính toán; `backward()` (nguyên vẹn từ `q8.2b`, không đổi một
dòng) tự động lan gradient tới TỪNG tham số.

Nhưng `W`, `b` trong bài này vẫn là những con số CHO SẴN — `0.5`, `-0.3`,
`0.1`, ... viết tay, không phải học được từ dữ liệu. Có gradient rồi không
có nghĩa là đã HUẤN LUYỆN: gradient mới chỉ nói "tham số này cần thay đổi
theo hướng nào", chưa có gì THỰC SỰ thay đổi tham số đó cả. Bài sau: hàm
mất mát — thứ mà gradient này thực ra đang lan ngược TỪ, và điều kiện cần
trước khi có thể cập nhật bất kỳ tham số nào.
::::

::::checkpoint{mastery=0.85}
::::
