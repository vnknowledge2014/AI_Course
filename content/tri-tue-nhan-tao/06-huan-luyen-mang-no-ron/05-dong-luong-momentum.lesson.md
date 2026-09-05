---
id: tri-tue-nhan-tao.huan-luyen-mang-no-ron.dong-luong-momentum
title: "Động lượng: vượt qua khe hẹp"
summary: "Bài toán khe hẹp L(w1,w2)=w1²+200w2² (K=200, xuất phát w1=w2=1.0) mô phỏng một hướng RẤT dốc và một hướng RẤT thoải — chính hình dạng khiến plain SGD phải dùng lr cực nhỏ. SGD (lr=0.0045) cần 255 bước để loss xuống dưới 0.01 và Ở LẠI dưới đó. Động lượng v=beta*v+(1-beta)*grad, cập nhật bằng v thay vì grad (lr=0.025, beta=0.7) chỉ cần 42 bước — nhanh hơn khoảng 6 lần, cùng một bài toán, cùng đích đến."
locale: vi
track: tri-tue-nhan-tao
module: huan-luyen-mang-no-ron
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.dong-luong-momentum]
requires: [ai.minibatch-va-sgd]
concepts: [ai.dong-luong-momentum]
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
Mọi bước cập nhật từ đầu quest tới giờ đều CHỈ dùng gradient của bước HIỆN
TẠI, quên sạch mọi bước trước đó. Nếu nhớ lại một chút "đà" từ những bước
vừa đi qua thì sao?
::::

::::explain{#dong_luong}
Xét một bài toán tối ưu có hình dạng KHE HẸP: một hướng RẤT dốc (đi lệch
một chút là loss vọt lên rất nhanh) và một hướng RẤT thoải (đi bao xa cũng
chỉ đổi loss một chút). Gradient descent thường phải chọn `lr` đủ NHỎ để
không "văng" ra khỏi hướng dốc — nhưng `lr` nhỏ đó cũng làm hướng thoải tiến
CỰC KỲ chậm, vì cùng một `lr` áp dụng cho cả hai hướng.

**Động lượng (momentum)** giữ một bộ nhớ — vận tốc `v`, khởi tạo `0` — và
cập nhật nó bằng trung bình động của gradient, thay vì dùng thẳng gradient:

> `v ← β·v + (1−β)·grad`
>
> `param ← param − lr·v`

`β` (beta, thường `0.7`–`0.9`) là hệ số "nhớ" — bao nhiêu phần vận tốc CŨ
được giữ lại. Nếu gradient đổi HƯỚNG liên tục qua từng bước (đúng đặc điểm
của hướng DỐC trong khe hẹp — bước này lệch sang trái, bước sau lệch sang
phải), các đóng góp trái dấu dần TRIỆT TIÊU nhau trong `v`, làm bước đi
theo hướng đó bị GIẢM XÓC. Nếu gradient giữ NGUYÊN hướng qua nhiều bước
(đúng đặc điểm của hướng THOẢI), các đóng góp CÙNG dấu CỘNG DỒN lại trong
`v`, làm bước đi theo hướng đó được TĂNG TỐC. Cùng một `lr`, động lượng để
mỗi hướng "tự" đi nhanh hay chậm theo đúng đặc điểm riêng của nó — không
cần chọn `lr` khác nhau cho từng hướng.

Trạng thái `v` không phải một `Value` — nó là một số Python trần, đi kèm
với từng tham số, y hệt vai trò của `w.grad`: một con số phụ trợ để QUYẾT
ĐỊNH bước cập nhật, không phải một phần của đồ thị tính đạo hàm.
::::

::::example{#khe_hep_sgd_vs_momentum}
Bài toán khe hẹp `L(w1, w2) = w1² + 200·w2²` (hướng `w2` dốc gấp `200` lần
hướng `w1`), xuất phát từ `w1 = w2 = 1.0`. Đếm số bước để loss xuống dưới
`0.01` VÀ Ở LẠI dưới đó (không bật lên lại):

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

def toi_uu_sgd(so_buoc, lr, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    ls = []
    for _ in range(so_buoc):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        w1.data -= lr * w1.grad
        w2.data -= lr * w2.grad
        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

def toi_uu_momentum(so_buoc, lr, beta, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    v1, v2 = 0.0, 0.0
    ls = []
    for _ in range(so_buoc):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        v1 = beta * v1 + (1 - beta) * w1.grad
        v2 = beta * v2 + (1 - beta) * w2.grad
        w1.data -= lr * v1
        w2.data -= lr * v2
        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

def epoch_hoi_tu(ls, nguong):
    for i in range(len(ls)):
        if all(v < nguong for v in ls[i:]):
            return i + 1
    return None

nguong = 0.01
ls_sgd = toi_uu_sgd(260, 0.0045)
ls_mom = toi_uu_momentum(45, 0.025, 0.7)

print(epoch_hoi_tu(ls_sgd, nguong))
print(epoch_hoi_tu(ls_mom, nguong))
```

```text title=readonly
255
42
```

SGD (`lr = 0.0045` — đủ nhỏ để hướng `w2` dốc không phân kỳ) cần `255`
bước. Động lượng (`lr = 0.025` — gấp hơn `5` lần, `beta = 0.7`) chỉ cần
`42` bước — nhanh hơn khoảng `6` lần, TRÊN CÙNG bài toán, TỚI CÙNG ngưỡng.
Đường loss của SGD giảm tuyệt đối đơn điệu (`0` lần tăng trên `259` bước) —
nhưng động lượng thì KHÔNG: nó có những bước loss tăng tạm thời (đà từ
hướng dốc "vọt" qua trước khi tự điều chỉnh) trước khi hội tụ nhanh hơn
hẳn về sau.
::::

::::predict{#doan_loss_muot_khong commitOnce}
SGD ở trên giảm loss đơn điệu tuyệt đối — `259` bước, `0` lần tăng.

**Trước khi chạy thử**, bạn đoán: đường loss của động lượng (dùng vận tốc
tích luỹ `v`, không dùng thẳng gradient) có giữ được tính chất "không bao
giờ tăng" đó không?

:::opt{correct}
Không — động lượng CÓ THỂ làm loss tăng tạm thời một vài bước, vì `v` mang
theo "đà" từ những bước trước, có thể đẩy tham số VỌT QUA điểm tốt nhất
theo một hướng trước khi tự điều chỉnh lại ở bước sau; nhưng nó vẫn hội tụ
— và hội tụ nhanh hơn hẳn — về sau
:::

:::opt
Có — động lượng vẫn luôn đi theo đúng hướng giảm dốc của gradient tại mỗi
bước, y hệt SGD, chỉ là bước dài hơn nên mượt hơn nữa
::why
Gần đúng ở việc "bước dài hơn" đúng là một phần bản chất của động lượng —
`v` tích luỹ qua nhiều bước có thể lớn hơn một gradient đơn lẻ.

Chỗ lệch: động lượng KHÔNG đi theo hướng gradient TỨC THỜI tại mỗi bước —
nó đi theo hướng của `v`, một pha trộn giữa gradient hiện tại và vận tốc
CŨ. Hai hướng đó có thể LỆCH nhau, đặc biệt ngay sau khi gradient vừa đổi
hướng (ở khúc cua của khe hẹp) — đó chính xác là lúc `v` còn mang đà từ
hướng CŨ, đẩy tham số VỌT QUA điểm tốt, không "mượt hơn" mà "có thể vọt
qua".
::
:::

:::opt
Không xác định được nếu không biết trước `beta` chính xác — với `beta`
đủ nhỏ, động lượng luôn mượt y như SGD, không có bước nào tăng
::why
Gần đúng ở việc `beta` THẬT SỰ ảnh hưởng tới mức độ "đà" được giữ lại —
`beta` càng nhỏ, `v` càng gần với gradient tức thời (ít đà hơn), một quan
sát đúng hướng.

Chỗ lệch: với `beta = 0.7` cụ thể ở bài này (không phải một giá trị cực
đoan) — kết quả đã đo THẬT ở trên vẫn cho một vài bước loss tăng trước khi
hội tụ hẳn. Việc "beta nhỏ hơn nữa có mượt tuyệt đối như SGD không" là một
câu hỏi khác, không được trả lời chỉ bằng suy luận — nó cần đo THẬT như
mọi số liệu khác trong bài này.
::
:::
::::

::::code{#viet_cap_nhat_dong_luong}
Hoàn thiện `toi_uu_momentum`: cập nhật vận tốc `v1`, `v2` bằng đúng công
thức `beta·v + (1−beta)·grad`.

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

def toi_uu_sgd(so_buoc, lr, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    ls = []
    for _ in range(so_buoc):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        w1.data -= lr * w1.grad
        w2.data -= lr * w2.grad
        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

def toi_uu_momentum(so_buoc, lr, beta, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    v1, v2 = 0.0, 0.0
    ls = []
    for _ in range(so_buoc):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        v1 = ___                          # beta * v1 + (1 - beta) * w1.grad
        v2 = ___                          # beta * v2 + (1 - beta) * w2.grad
        w1.data -= lr * v1
        w2.data -= lr * v2
        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

def epoch_hoi_tu(ls, nguong):
    for i in range(len(ls)):
        if all(v < nguong for v in ls[i:]):
            return i + 1
    return None

nguong = 0.01
ls_sgd = toi_uu_sgd(260, 0.0045)
ls_mom = toi_uu_momentum(45, 0.025, 0.7)

epoch_sgd = epoch_hoi_tu(ls_sgd, nguong)
epoch_mom = epoch_hoi_tu(ls_mom, nguong)

print(epoch_sgd)
print(epoch_mom)
print(epoch_mom < epoch_sgd)
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

def toi_uu_sgd(so_buoc, lr, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    ls = []
    for _ in range(so_buoc):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        w1.data -= lr * w1.grad
        w2.data -= lr * w2.grad
        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

def toi_uu_momentum(so_buoc, lr, beta, w1_0=1.0, w2_0=1.0):
    w1, w2 = Value(w1_0), Value(w2_0)
    v1, v2 = 0.0, 0.0
    ls = []
    for _ in range(so_buoc):
        L = w1 * w1 + Value(K) * (w2 * w2)
        L.backward()
        v1 = beta * v1 + (1 - beta) * w1.grad
        v2 = beta * v2 + (1 - beta) * w2.grad
        w1.data -= lr * v1
        w2.data -= lr * v2
        w1.grad = 0.0
        w2.grad = 0.0
        ls.append(w1.data ** 2 + K * w2.data ** 2)
    return ls

def epoch_hoi_tu(ls, nguong):
    for i in range(len(ls)):
        if all(v < nguong for v in ls[i:]):
            return i + 1
    return None

nguong = 0.01
ls_sgd = toi_uu_sgd(260, 0.0045)
ls_mom = toi_uu_momentum(45, 0.025, 0.7)

epoch_sgd = epoch_hoi_tu(ls_sgd, nguong)
epoch_mom = epoch_hoi_tu(ls_mom, nguong)

print(epoch_sgd)
print(epoch_mom)
print(epoch_mom < epoch_sgd)
```

```python title=test
assert epoch_sgd == 255, f"epoch_sgd sai -- dang ra {epoch_sgd}"
assert epoch_mom == 42, f"epoch_mom sai -- dang ra {epoch_mom}"
assert epoch_mom < epoch_sgd, "dong luong phai hoi tu ben vung NHANH HON SGD tren cung bai toan khe hep nay"

# rieng kiem tra CONG THUC dung: beta phai xuat hien o CA HAI so hang (beta*v
# VA he so (1-beta) nhan grad) -- neu dung cong thuc "heavy-ball" khac (v =
# beta*v + grad, thieu he so (1-beta)) thi VOI DUNG sieu tham so lr=0.025,
# beta=0.7 cua bai nay, ket qua se KHAC han so voi da tu kiem chung -- da tu
# chay that: cong thuc thieu (1-beta) cho epoch_mom khac 42 (hoac khong hoi
# tu trong 45 buoc).
ls_mom_kiem = toi_uu_momentum(45, 0.025, 0.7)
assert ls_mom_kiem == ls_mom, "toi_uu_momentum phai tat dinh -- goi lai voi cung tham so phai ra CUNG ket qua"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cùng một công thức — trung bình động có trọng số. `v1` MỚI là `beta` phần của `v1` CŨ, cộng `(1 - beta)` phần của gradient MỚI: `beta * v1 + (1 - beta) * w1.grad`. `v2` làm y hệt với `w2.grad`. Chú ý cả hai hệ số `beta` và `(1 - beta)` đều phải xuất hiện — thiếu `(1 - beta)` (chỉ viết `beta * v1 + w1.grad`) là một công thức động lượng KHÁC, cho kết quả khác hẳn với đúng bộ `lr`, `beta` của bài này.
- kind: strategy
  body: 'v1: `beta * v1 + (1 - beta) * w1.grad`. v2: `beta * v2 + (1 - beta) * w2.grad`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `beta * v1 + (1 - beta) * w1.grad` và `beta * v2 + (1 - beta) * w2.grad`.'
:::

:::validate
- tier: run
  timeoutMs: 12000
- tier: static
  onFail: v1/v2 phai cap nhat DUNG cong thuc dong luong beta*v + (1-beta)*grad -- ca beta VA (1-beta) deu phai xuat hien, thieu (1-beta) la mot cong thuc dong luong khac (heavy-ball), cho ket qua khac han voi dung sieu tham so cua bai nay
  requireAst:
  - kind: uses-name, target: beta, min: 4
  - kind: uses-name, target: v1, min: 2
  - kind: uses-name, target: v2, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca ba luat qua sach (beta=4: hai lan moi dong "beta*v1"/
  # "(1-beta)*w1.grad" va tuong tu cho v2 -- dung 4; v1=2: doc trong chinh
  # cong thuc cap nhat cua no VA doc lai trong "w1.data -= lr*v1"; v2 tuong
  # tu). Cheat "v1 = beta*v1 + w1.grad" (thieu he so (1-beta), cong thuc
  # heavy-ball) lam "beta" tut tu 4 xuong 2 -- bi chan boi static; da tu
  # kiem chung bang Python that: cheat nay VOI DUNG lr=0.025, beta=0.7 cua
  # bai lam optimizer PHAN KY (loss vuot qua nguong on dinh, khong con hoi
  # tu ben vung trong 45 buoc) -- bi chan doc lap boi tests (epoch_mom se
  # khong con la 42, co the la None neu khong tim thay epoch hoi tu).
- tier: tests
  timeoutMs: 12000
- tier: output
  match: regex
  expect: "^255\\n42\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`255` bước cho SGD, `42` bước cho động lượng — cùng bài toán, cùng ngưỡng.
Vận tốc tích luỹ giảm xóc hướng dốc, tăng tốc hướng thoải — không cần đổi
`lr` theo từng hướng.
::::

::::reflect{#nghi-lai}
Động lượng nhớ được HƯỚNG đi qua các bước trước — nhưng nó dùng CHUNG một
`lr` cho mọi tham số, không phân biệt tham số nào cần bước lớn, tham số nào
cần bước nhỏ. Bài sau: một bộ tối ưu tự động điều chỉnh `lr` RIÊNG cho từng
tham số, dựa trên chính lịch sử gradient của tham số đó.
::::

::::checkpoint{mastery=0.85}
::::
