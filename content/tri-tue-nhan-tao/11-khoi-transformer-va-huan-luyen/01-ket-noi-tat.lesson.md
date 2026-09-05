---
id: tri-tue-nhan-tao.khoi-transformer-va-huan-luyen.ket-noi-tat
title: "Kết nối tắt: một đường tắt cho gradient, bất kể SubLayer sâu bao nhiêu"
summary: "Residual connection Y = X + SubLayer(X) cộng thẳng đầu vào vào đầu ra một sublayer, dùng Tensor.__add__ có sẵn -- đạo hàm của phép cộng luôn là 1 ở CẢ HAI nhánh, tạo một đường tắt cho gradient lan qua, độc lập với SubLayer sâu bao nhiêu. Trên chuỗi 6 'tầng' giả lập (mỗi tầng co embedding lại 0,3 lần, cùng X đã dùng ở BOSS co-che-attention): KHÔNG residual, gradient trung bình tới X co còn 0,000729; CÓ residual, gradient trung bình là 4,826809 -- lớn hơn 6621,1372 lần, đối chiếu trực tiếp với vanishing gradient đã đo THẬT ở gradient-bien-mat-that (q8.2d)."
locale: vi
track: tri-tue-nhan-tao
module: khoi-transformer-va-huan-luyen
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.ket-noi-tat]
requires: [ai.boss-co-che-attention]
concepts: [ai.ket-noi-tat]
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
Chín bài của `co-che-attention` vừa đóng: một tầng attention đầy đủ, chạy được, đối chiếu đúng số tính tay. Nhưng một tầng chưa phải một mô hình — Transformer THẬT xếp chồng NHIỀU tầng biến đổi nối tiếp nhau. Quest này ráp một KHỐI hoàn chỉnh, rồi xếp chồng hai khối, rồi huấn luyện thật. Bước đầu tiên: một vấn đề đã đo được từ lâu.
::::

::::explain{#vi_sao_can_ket_noi_tat}
Bài `gradient-bien-mat-that` (q8.2d) đã đo THẬT: một mạng SÂU hơn (nhiều tầng `tanh` nối tiếp) có gradient tới tầng ĐẦU co lại RẤT MẠNH so với mạng NÔNG — vì backpropagation nhân LIÊN TIẾP nhiều đạo hàm cục bộ, và mỗi đạo hàm đó thường nhỏ hơn `1`. Càng nhiều tầng đứng giữa tham số và loss, gradient càng phải "lách qua" nhiều phép nhân co lại liên tiếp.

Một khối Transformer xếp chồng NHIỀU sublayer biến đổi nối tiếp (attention rồi feedforward, hai bài sau sẽ cụ thể hoá) — ĐÚNG cấu trúc gây vanishing gradient nếu không có gì can thiệp. **Kết nối tắt** (residual connection) là câu trả lời kiến trúc cho vấn đề này, không phải một mẹo tối ưu hoá:

> `Y = X + SubLayer(X)`

Cộng THẲNG đầu vào gốc `X` vào đầu ra của sublayer, dùng `Tensor.__add__` ĐÃ CÓ SẴN (q8.3b) — không cần công thức đạo hàm mới. Điều làm nên chuyện: đạo hàm cục bộ của phép cộng, ở CẢ HAI nhánh, LUÔN đúng bằng `1` (`self.grad += out.grad; other.grad += out.grad` — không nhân với bất kỳ hệ số nào). Gradient lan ngược qua `Y = X + SubLayer(X)` tới `X` có HAI đường: một đường xuyên THẲNG qua phép cộng (hệ số `1`, không suy giảm dù `SubLayer` sâu bao nhiêu), và một đường khác đi QUA `SubLayer` (có thể suy giảm, tuỳ `SubLayer` cụ thể). Vì đường thứ nhất luôn CỘNG THÊM một lượng không suy giảm, tổng gradient tới `X` không bao giờ tệ hơn nhiều so với việc hoàn toàn không có `SubLayer` nào — bất kể xếp chồng bao nhiêu tầng `SubLayer` nối tiếp.
::::

::::example{#khong_residual_that}
Dùng lại CHÍNH `X` đã tính ở BOSS `co-che-attention` (embedding + vị trí, đã làm tròn `6` chữ số). `SubLayer` ở đây là một phép chiếu tuyến tính CỐ ĐỊNH (không học, chỉ để đo hiệu ứng ĐỘ SÂU một cách sạch): `W = 0,3 × ma trận đơn vị 4×4` — mỗi tầng co embedding lại còn `0,3` lần. Xếp chồng `6` tầng, KHÔNG residual:

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


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
W = Tensor(0.3 * np.eye(4))

X = Tensor(X0)
Y = X
for tang in range(6):
    Y = Y.matmul(W)     # KHONG residual -- SubLayer(Y), khong cong lai Y
Y.backward()

print(round(float(np.mean(np.abs(X.grad))), 6))
```

```text title=readonly
0.000729
```

Sáu tầng, mỗi tầng co gradient lại còn `0,3` lần tầng trước — gradient trung bình tới `X` chỉ còn `0,000729` (`0,3` nhân với chính nó `6` lần). Đúng xu hướng đã đo ở `gradient-bien-mat-that`: càng nhiều tầng nối tiếp KHÔNG có đường tắt, gradient tới đầu vào càng co lại theo cấp số nhân.
::::

::::example{#co_residual_that}
Cùng `X`, cùng `W`, cùng `6` tầng — nhưng MỖI tầng giờ là `Y = Y + Y.matmul(W)` (kết nối tắt bao quanh sublayer):

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
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


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
W = Tensor(0.3 * np.eye(4))

X = Tensor(X0)
Y = X
for tang in range(6):
    Y = Y + Y.matmul(W)     # CO residual -- Y + SubLayer(Y)
Y.backward()

print(round(float(np.mean(np.abs(X.grad))), 6))
```

```text title=readonly
4.826809
```

Cùng độ sâu, cùng `SubLayer`, chỉ thêm phép cộng — gradient trung bình tới `X` giờ là `4,826809`, lớn hơn phiên bản không residual (`0,000729`) khoảng `6621,1372` lần. Không chỉ "ít suy giảm hơn" — ở đây gradient còn TĂNG (vì mỗi tầng nhân hiệu quả với `1 + 0,3 = 1,3`, lớn hơn `1`), nhưng điều quan trọng hơn con số cụ thể: đường tắt đảm bảo gradient tới `X` không bao giờ bị nhân với một hệ số suy giảm THUẦN, bất kể `SubLayer` co gradient lại bao nhiêu.
::::

::::predict{#doan_sau_hon_mot_tang commitOnce}
Ở độ sâu `6` tầng, tỉ lệ gradient CÓ residual so với KHÔNG residual là `6621,1372` lần.

**Trước khi tính**, bạn đoán: nếu xếp chồng THÊM một tầng nữa (độ sâu `7` thay vì `6`, vẫn cùng `W = 0,3 × I`), tỉ lệ đó sẽ RỘNG RA (lớn hơn `6621,1372`) hay HẸP LẠI (nhỏ hơn `6621,1372`)?

:::opt{correct}
Rộng ra — mỗi tầng thêm vào làm phiên bản KHÔNG residual co gradient lại thêm một hệ số `0,3` nữa (càng nhỏ), còn phiên bản CÓ residual nhân gradient lên thêm một hệ số `1,3` nữa (càng lớn); khoảng cách giữa "càng nhỏ" và "càng lớn" chỉ có thể RỘNG RA thêm khi tăng độ sâu, không thể hẹp lại
:::

:::opt
Hẹp lại — càng nhiều tầng thì cả hai phiên bản đều bị ảnh hưởng bởi độ sâu như nhau, nên tỉ lệ giữa chúng phải tiến về một hằng số cố định khi độ sâu tăng
::why
Gần đúng ở việc để ý CẢ HAI phiên bản đều chịu tác động của việc tăng độ sâu — quan sát đó không sai.

Chỗ lệch: "chịu tác động như nhau" không có nghĩa là tỉ lệ hội tụ về một hằng số. Phiên bản không residual nhân thêm `0,3` (một số NHỎ HƠN `1`, đẩy tích về `0`), phiên bản có residual nhân thêm `1,3` (một số LỚN HƠN `1`, đẩy tích ra xa `0`) — hai tích phân kỳ theo hai hướng NGƯỢC nhau, nên tỉ lệ giữa chúng (~`(1,3/0,3)` mỗi tầng thêm) tiếp tục NHÂN LÊN, không hội tụ.
::
:::

:::opt
Không xác định được nếu không tính lại từ đầu — không có quy luật chung nào giữa độ sâu `6` và độ sâu `7`
::why
Gần đúng ở tinh thần muốn đo thật trước khi kết luận — nguyên tắc xuyên suốt track này.

Chỗ lệch: cấu trúc của ví dụ này (`W` CỐ ĐỊNH, cùng một hệ số mỗi tầng) đủ đơn giản để suy luận trước HƯỚNG thay đổi mà không cần tính lại: mỗi tầng thêm luôn nhân phiên bản không-residual với đúng `0,3` và phiên bản có-residual với đúng `1,3` — hai hệ số CỐ ĐỊNH, không đổi theo tầng. Từ đó suy ra được HƯỚNG (rộng ra), dù giá trị CHÍNH XÁC vẫn cần đo thật để biết rộng ra bao nhiêu.
::
:::
::::

::::code{#viet_ket_noi_tat}
Hoàn thiện hai chỗ trống: một bước "không residual" (chỉ áp `SubLayer`), và một bước "có residual" (cộng thêm đầu vào của chính bước đó).

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
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


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
W = Tensor(0.3 * np.eye(4))
DO_SAU = 6

X_khong = Tensor(X0)
Y = X_khong
for tang in range(DO_SAU):
    Y = ___                          # Y.matmul(W)
Y.backward()
g_khong = float(np.mean(np.abs(X_khong.grad)))

X_co = Tensor(X0)
Y = X_co
for tang in range(DO_SAU):
    Y = ___                          # Y + Y.matmul(W)
Y.backward()
g_co = float(np.mean(np.abs(X_co.grad)))

ti_le = g_co / g_khong

print(round(g_khong, 6))
print(round(g_co, 6))
print(round(ti_le, 4))
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

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        def _backward():
            self.grad += out.grad @ other.data.T
            other.grad += self.data.T @ out.grad
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


X0 = [[1.0, 0.0, 1.0, 0.0], [-0.0, 1.0, 1.0, 1.0], [1.0, 1.0, -0.0, 1.0], [1.14112, -1.989992, 1.029996, -0.00045]]
W = Tensor(0.3 * np.eye(4))
DO_SAU = 6

X_khong = Tensor(X0)
Y = X_khong
for tang in range(DO_SAU):
    Y = Y.matmul(W)
Y.backward()
g_khong = float(np.mean(np.abs(X_khong.grad)))

X_co = Tensor(X0)
Y = X_co
for tang in range(DO_SAU):
    Y = Y + Y.matmul(W)
Y.backward()
g_co = float(np.mean(np.abs(X_co.grad)))

ti_le = g_co / g_khong

print(round(g_khong, 6))
print(round(g_co, 6))
print(round(ti_le, 4))
```

```python title=test
assert round(g_khong, 6) == 0.000729, f"g_khong sai -- dang ra {round(g_khong, 6)}"
assert round(g_co, 6) == 4.826809, f"g_co sai -- dang ra {round(g_co, 6)}"
assert round(ti_le, 4) == 6621.1372, f"ti_le sai -- dang ra {round(ti_le, 4)}"

# rieng kiem tra CO residual phai co gradient LON HON han KHONG residual
assert g_co > g_khong, f"co residual phai cho gradient lon hon khong residual -- dang ra g_co={g_co}, g_khong={g_khong}"

# rieng kiem tra o mot DO SAU KHAC (7 tang) -- chan cheat chi dung cho dung 6 tang
X_khong_7 = Tensor(X0)
Y = X_khong_7
for tang in range(7):
    Y = Y.matmul(W)
Y.backward()
g_khong_7 = float(np.mean(np.abs(X_khong_7.grad)))

X_co_7 = Tensor(X0)
Y = X_co_7
for tang in range(7):
    Y = Y + Y.matmul(W)
Y.backward()
g_co_7 = float(np.mean(np.abs(X_co_7.grad)))

assert round(g_khong_7, 6) == 0.000219, f"g_khong_7 sai -- dang ra {round(g_khong_7, 6)}"
assert round(g_co_7, 6) == 6.274852, f"g_co_7 sai -- dang ra {round(g_co_7, 6)}"
assert (g_co_7 / g_khong_7) > ti_le, f"ti le o do sau 7 phai RONG RA (lon hon) ti le o do sau 6 -- dang ra {g_co_7/g_khong_7} va {ti_le}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai bên trong một vòng lặp `for tang in range(DO_SAU)`. Chỗ đầu (KHÔNG residual) chỉ áp `SubLayer` lên `Y` hiện tại, KHÔNG cộng gì thêm — `Y.matmul(W)`. Chỗ hai (CÓ residual) cộng `Y` hiện tại vào chính đầu ra của `SubLayer` đó — `Y + Y.matmul(W)`. Đừng đảo ngược hai chỗ.
- kind: strategy
  body: 'Chỗ đầu: `Y.matmul(W)`. Chỗ hai: `Y + Y.matmul(W)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `Y.matmul(W)` và `Y + Y.matmul(W)`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: vong lap KHONG residual phai la Y.matmul(W) (khong cong gi them); vong lap CO residual phai la Y + Y.matmul(W) (cong Y hien tai vao dau ra SubLayer) -- khong duoc dao nguoc hay chep san gia tri
  requireAst:
  - kind: uses-call, target: matmul, min: 2
  - kind: uses-operator, target: "+", min: 1
  - kind: uses-name, target: Y, min: 5
  # Da thu THAT bang kiemAst that (goi truc tiep tren code trich tu
  # solution/starter da bien dich, khong doan tay): tren solution, matmul=2
  # (mot lan moi blank -- "Y.matmul(W)" o blank1 va "Y.matmul(W)" ben trong
  # blank2; dinh nghia "def matmul(self, other):" KHONG tinh la Call).
  # "+"=6 tren toan bo solution (1 trong __add__.data, 1 trong blank2, con
  # lai rai rac trong cac dong khac cua Tensor/harness) -- nguong min=1 an
  # toan vi khong the tut xuong duoi 1 neu hoc vien dien dung blank2. Y=5
  # tren toan bo solution (Load: hai vong lap "for tang in range(DO_SAU):
  # Y = ..." doc "Y" o ca hai ve, dinh danh trong "def" khong tinh -- kiem
  # tra that qua kiemAst xac nhan dung 5, KHONG phai 8 nhu uoc luong tay ban
  # dau). Dien bua "___" -> "True" cho Y tut xuong 2 -- duoi nguong min=5,
  # bi chan.
  # Cheat "Y = Y.matmul(W)" cho CA HAI blank (bo qua residual o blank2) lam
  # "+" tut xuong 1 (chi con trong __add__, khong con trong blank2) --
  # NHUNG bi bat DOC LAP boi assert gia tri g_co (se bang g_khong thay vi
  # lon hon han) -- "+"=1 van dat nguong min=1 nen static KHONG tu no bat
  # duoc cheat nay; day la ly do PHAI co assert "g_co > g_khong" rieng o
  # tier tests, khong chi dua vao static.
  # Cheat "Y = Y + W" (cong W thay vi Y.matmul(W), bo qua phep chieu) lam
  # matmul tut xuong 1 -- bi chan RIENG, VA da tu kiem chung: shape khong
  # khop (Y la (4,4), W la (4,4), phep cong van chay duoc ve mat shape
  # nhung gia tri SAI HOAN TOAN so voi 4.826809 dung) -- bi bat DOC LAP boi
  # assert g_co.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^0\\.000729\\n4\\.826809\\n6621\\.1372\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng độ sâu, cùng SubLayer — chỉ thêm một phép cộng mà gradient tới đầu vào lớn hơn hơn sáu nghìn lần. Bài sau: SubLayer thật đầu tiên của khối Transformer — mạng truyền thẳng áp riêng cho từng vị trí.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Kết nối tắt vừa cho thấy: gradient tới `X` LUÔN có một đường xuyên thẳng qua phép cộng, hệ số `1`, bất kể `SubLayer` là gì. Nhưng `Y = X + SubLayer(X)` chỉ hợp lệ khi `X` và `SubLayer(X)` có CÙNG shape — phép cộng element-wise của `Tensor` không tự động "khớp" hai shape khác nhau. `SubLayer` sắp xây (mạng truyền thẳng theo vị trí, bài sau) nhận `X` shape `(so_token, dim)` làm đầu vào — nó cần trả về ĐÚNG shape nào để phép cộng residual này hợp lệ?
::::

::::checkpoint{mastery=0.8}
::::
