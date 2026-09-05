---
id: tri-tue-nhan-tao.huan-luyen-mang-no-ron.vong-lap-huan-luyen
title: "Vòng lặp huấn luyện: forward, backward, cập nhật, zero_grad"
summary: "Vòng lặp đầy đủ: forward -> mse_qua_value -> loss.backward() -> w.data -= lr*w.grad -> zero_grad. Trên 4 điểm hồi quy tuyến tính (x=1..4, y~2x+1), 30 epoch (lr=0.02) đưa loss từ 41.0 xuống 0.0097, giảm ĐƠN ĐIỆU mọi bước, w,b hội tụ về 2.0814/0.7606. Gotcha trung tâm: quên zero_grad khiến gradient CỘNG DỒN xuyên epoch — chạy đúng 2 bước, CÓ zero_grad cho w,b=(1.166, 0.4004), KHÔNG zero_grad cho (1.866, 0.6404) — hai kết quả khác nhau, tự kiểm chứng bằng Python thật."
locale: vi
track: tri-tue-nhan-tao
module: huan-luyen-mang-no-ron
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.vong-lap-huan-luyen]
requires: [ai.ham-mat-mat-qua-value]
concepts: [ai.vong-lap-huan-luyen]
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
Hai bài trước xây xong hai nửa: forward pass tự động qua `Value`, hàm mất
mát tính được gradient tới từng tham số. Còn thiếu đúng một mảnh: DÙNG
gradient đó để thực sự đổi `W`, `b` — lặp đi lặp lại.
::::

::::explain{#vong_lap_day_du}
Một bước huấn luyện có ĐÚNG bốn việc, theo ĐÚNG thứ tự này, không đảo được:

> **1. Forward** — đưa dữ liệu qua mạng, tính dự đoán (`Layer`/`MLP` của
> bài `neuron-tu-autograd`, giờ chạy qua `Value`).
>
> **2. Loss** — so dự đoán với nhãn thật, ra MỘT con số duy nhất
> (`mse_qua_value` hoặc `cross_entropy_qua_value` của bài
> `ham-mat-mat-qua-value`).
>
> **3. Backward** — `loss.backward()`, lan gradient ngược tới MỌI tham số
> đã tham gia tính `loss`.
>
> **4. Cập nhật** — với MỖI tham số `p`: `p.data -= lr * p.grad` (`lr` —
> learning rate — một số dương nhỏ, điều khiển bước đi bao xa theo hướng
> NGƯỢC gradient, vì gradient chỉ ra hướng làm loss TĂNG nhanh nhất).

Lặp lại bốn bước đó nhiều lần — mỗi lần gọi là một **epoch** — loss sẽ
giảm dần, nếu `lr` chọn hợp lý.

Nhưng có một bước THỨ NĂM, dễ quên nhất, và quên nó không hề báo lỗi gì:
**`zero_grad`** — sau khi đã DÙNG `p.grad` để cập nhật `p.data`, phải RESET
`p.grad` về lại `0.0` trước khi bước tiếp theo bắt đầu. Lý do: `Value.grad`
KHÔNG BAO GIỜ tự reset giữa hai lần gọi `.backward()` (bài
`tich-luy-gradient`, q8.2b) — nó luôn CỘNG DỒN (`+=`). Nếu quên
`zero_grad`, gradient của bước 2 sẽ cộng CHỒNG lên gradient CÒN SÓT LẠI của
bước 1 (chưa bị xoá), khiến bước cập nhật ở epoch 2 dùng một con số gradient
LỚN HƠN gradient THẬT của riêng epoch đó — càng nhiều epoch trôi qua, phần
"sót lại" càng chồng chất, huấn luyện càng sai lệch so với đúng thuật toán
gradient descent.

Vòng lặp đầy đủ, viết thành code:

```python
for epoch in range(so_epoch):
    preds = [...]                    # 1. forward
    loss = mse_qua_value(preds, y)   # 2. loss
    loss.backward()                  # 3. backward
    for p in params:
        p.data -= lr * p.grad        # 4. cap nhat
        p.grad = 0.0                 # 5. zero_grad -- KHONG duoc quen
```
::::

::::example{#loss_giam_that}
Bốn điểm (`x = 1, 2, 3, 4`), quan hệ thật gần với `y = 2x + 1`. Một neuron
tuyến tính (`w`, `b`, không kích hoạt) huấn luyện `30` epoch, `lr = 0.02`:

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


x_data = [1.0, 2.0, 3.0, 4.0]
y_data = [3.0, 5.0, 7.0, 9.0]

w = Value(0.0)
b = Value(0.0)
lich_su_loss = []
for epoch in range(30):
    preds = [w * xi + b for xi in x_data]
    loss = mse_qua_value(preds, y_data)
    loss.backward()
    w.data -= 0.02 * w.grad
    b.data -= 0.02 * b.grad
    w.grad = 0.0
    b.grad = 0.0
    lich_su_loss.append(loss.data)

print(round(lich_su_loss[0], 4))
print(round(lich_su_loss[-1], 4))
print(round(w.data, 4), round(b.data, 4))
```

```text title=readonly
41.0
0.0097
2.0814 0.7606
```

Loss đi từ `41.0` (epoch đầu, `w=b=0` còn rất xa) xuống `0.0097` (epoch
`30`) — giảm ĐƠN ĐIỆU ở MỌI epoch, không một lần tăng (đã tự kiểm chứng
bằng Python thật). `w`, `b` hội tụ về gần `2`, `1` — đúng hướng quan hệ
thật `y = 2x + 1` mà bốn điểm dữ liệu (có nhiễu) gợi ý.
::::

::::predict{#doan_quen_zero_grad commitOnce}
Xét vòng lặp trên nhưng KHÔNG có hai dòng cuối (`w.grad = 0.0`,
`b.grad = 0.0`) — tức QUÊN `zero_grad`. Ở epoch `1`, `w.grad` sau
`backward()` là `-35.0` (tính từ `w=0, b=0`). Vì `w`, `b` đã đổi sau epoch
`1`, epoch `2` xây một đồ thị MỚI — nếu được reset về `0` trước đó, gradient
"riêng" của MỖI epoch `2` (tính từ đúng `w`, `b` sau epoch `1`) sẽ là
`-23.3`.

**Trước khi chạy thử**, bạn đoán: nếu QUÊN reset, `w.grad` sau
`backward()` ở epoch `2` sẽ là bao nhiêu?

:::opt{correct}
`-58.3` — đúng bằng `-35.0 + (-23.3)` — vì gradient "sót lại" từ epoch `1`
(`-35.0`, chưa bị xoá) CỘNG DỒN với gradient "riêng" của epoch `2`
(`-23.3`, tính từ đồ thị MỚI của epoch này); `backward()` không hề biết
gradient cũ có nên bị xoá hay không, nó chỉ trung thành cộng dồn
(`+=`) vào bất cứ gì đang có sẵn trong `w.grad`
:::

:::opt
`w.grad` sẽ hoàn toàn sai/vô nghĩa — vì quên reset làm hỏng luôn cả phép
tính `backward()` của chính epoch `2`, không chỉ ảnh hưởng epoch sau
::why
Gần đúng ở việc nhận ra quên reset LÀ một lỗi nghiêm trọng, ảnh hưởng thật
tới kết quả — quan sát về mức độ nghiêm trọng không sai.

Chỗ lệch: `backward()` của epoch `2` vẫn tính ĐÚNG gradient cục bộ của đồ
thị epoch `2` (đúng công thức đạo hàm, không "hỏng" phép tính nào) — nó chỉ
CỘNG kết quả đó vào một `w.grad` chưa được dọn sạch. Đây là lỗi CỘNG DỒN
NHẦM, không phải lỗi TÍNH SAI công thức.
::
:::

:::opt
`w.grad` sẽ gấp đôi `-35.0`, tức `-70.0` — giống hệt kịch bản gọi
`backward()` hai lần liên tiếp trên CÙNG một đồ thị không đổi
::why
Gần đúng ở việc bạn nhớ đúng một kịch bản THẬT (bài `neuron-tu-autograd`):
gọi `backward()` hai lần trên đồ thị GIỐNG HỆT nhau (không tham số nào đổi
giữa hai lần) cho đúng gấp đôi.

Chỗ lệch: ở ĐÂY, `w`, `b` đã bị CẬP NHẬT giữa epoch `1` và epoch `2` (dòng
`w.data -= 0.02 * w.grad` đã chạy) — nên epoch `2` xây một đồ thị KHÁC
(cùng cấu trúc, khác giá trị `w`), cho gradient cục bộ riêng KHÁC `-35.0`.
Tổng cuối cùng là CỘNG hai gradient khác nhau (`-35.0` và `-23.3`), không
phải nhân đôi một gradient giống hệt.
::
:::
::::

::::code{#viet_mot_buoc_huan_luyen}
Hoàn thiện `mot_buoc_huan_luyen`: cập nhật `w.data`, `b.data` bằng
`lr * grad`, rồi RESET cả hai gradient về `0.0`.

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


x_data = [1.0, 2.0, 3.0, 4.0]
y_data = [3.0, 5.0, 7.0, 9.0]

def mot_buoc_huan_luyen(w, b, x_data, y_data, lr):
    preds = [w * xi + b for xi in x_data]
    loss = mse_qua_value(preds, y_data)
    loss.backward()
    w.data -= ___                     # lr * w.grad
    b.data -= ___                     # lr * b.grad
    w.grad = ___                      # 0.0
    b.grad = ___                      # 0.0
    return loss.data

w = Value(0.0)
b = Value(0.0)
lich_su_loss = []
for epoch in range(30):
    l = mot_buoc_huan_luyen(w, b, x_data, y_data, lr=0.02)
    lich_su_loss.append(l)

print(round(lich_su_loss[0], 4))
print(round(lich_su_loss[-1], 4))
print(round(w.data, 4), round(b.data, 4))
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


x_data = [1.0, 2.0, 3.0, 4.0]
y_data = [3.0, 5.0, 7.0, 9.0]

def mot_buoc_huan_luyen(w, b, x_data, y_data, lr):
    preds = [w * xi + b for xi in x_data]
    loss = mse_qua_value(preds, y_data)
    loss.backward()
    w.data -= lr * w.grad
    b.data -= lr * b.grad
    w.grad = 0.0
    b.grad = 0.0
    return loss.data

w = Value(0.0)
b = Value(0.0)
lich_su_loss = []
for epoch in range(30):
    l = mot_buoc_huan_luyen(w, b, x_data, y_data, lr=0.02)
    lich_su_loss.append(l)

print(round(lich_su_loss[0], 4))
print(round(lich_su_loss[-1], 4))
print(round(w.data, 4), round(b.data, 4))
```

```python title=test
assert round(lich_su_loss[0], 4) == 41.0, f"loss epoch dau sai -- dang ra {round(lich_su_loss[0], 4)}"
assert round(lich_su_loss[-1], 4) == 0.0097, f"loss epoch cuoi sai -- dang ra {round(lich_su_loss[-1], 4)}"
assert round(w.data, 4) == 2.0814, f"w cuoi sai -- dang ra {round(w.data, 4)}"
assert round(b.data, 4) == 0.7606, f"b cuoi sai -- dang ra {round(b.data, 4)}"

# rieng kiem tra BIEN: loss phai GIAM (hoac giu nguyen trong sai so) o MOI
# buoc, khong duoc co buoc nao TANG -- da tu kiem chung bang Python that
# tren chinh 30 epoch nay, dung dung '<=' voi dung sai 1e-9 de tranh lo hong
# bien do so thuc.
assert all(lich_su_loss[i + 1] <= lich_su_loss[i] + 1e-9 for i in range(len(lich_su_loss) - 1)), "loss phai giam don dieu qua tung epoch voi vong lap dung -- neu co buoc tang, kiem tra lai zero_grad"

# GOTCHA TRUNG TAM: quen zero_grad. Chay dung 2 buoc goi mot_buoc_huan_luyen
# (CO zero_grad, dung ham vua viet) doi chieu voi 2 buoc TU TAY KHONG reset
# gradient -- hai ket qua PHAI khac nhau. Da tu kiem chung bang Python that
# ca hai chieu truoc khi viet assertion nay.
w_co, b_co = Value(0.0), Value(0.0)
mot_buoc_huan_luyen(w_co, b_co, x_data, y_data, lr=0.02)
mot_buoc_huan_luyen(w_co, b_co, x_data, y_data, lr=0.02)

w_khong, b_khong = Value(0.0), Value(0.0)
for _ in range(2):
    preds_tmp = [w_khong * xi + b_khong for xi in x_data]
    loss_tmp = mse_qua_value(preds_tmp, y_data)
    loss_tmp.backward()
    w_khong.data -= 0.02 * w_khong.grad
    b_khong.data -= 0.02 * b_khong.grad
    # CO Y khong reset grad o day -- day chinh la ban THIEU zero_grad

assert (round(w_co.data, 6), round(b_co.data, 6)) != (round(w_khong.data, 6), round(b_khong.data, 6)), "CO zero_grad va KHONG zero_grad phai cho hai ket qua KHAC NHAU sau 2 buoc -- neu giong nhau, mot_buoc_huan_luyen dang khong reset gradient dung cach"
assert round(w_co.data, 6) == 1.166, f"w sau 2 buoc CO zero_grad phai la 1.166 -- dang ra {round(w_co.data, 6)}"
assert round(b_co.data, 6) == 0.4004, f"b sau 2 buoc CO zero_grad phai la 0.4004 -- dang ra {round(b_co.data, 6)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. Hai chỗ đầu (`w.data`, `b.data`) là bước CẬP NHẬT — trừ đi `lr` NHÂN gradient tương ứng: `lr * w.grad`, `lr * b.grad`. Hai chỗ sau (`w.grad`, `b.grad`) là bước `zero_grad` — RESET thẳng về `0.0`, không phải trừ hay nhân gì cả. Thiếu hai chỗ SAU sẽ làm gradient cộng dồn xuyên epoch, sai hoàn toàn kết quả huấn luyện dù không báo lỗi gì.
- kind: strategy
  body: 'w.data -= : `lr * w.grad`. b.data -= : `lr * b.grad`. w.grad = : `0.0`. b.grad = : `0.0`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `lr * w.grad`, `lr * b.grad`, `0.0`, `0.0`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: mot_buoc_huan_luyen phai cap nhat w.data/b.data bang dung lr*grad (khong duoc chep lai hang so), VA phai RESET w.grad/b.grad ve dung 0.0 sau moi buoc (thieu buoc nay la gotcha zero_grad -- gradient se cong don xuyen epoch)
  requireAst:
  - kind: gan-ten, target: w, min: 3
  - kind: gan-ten, target: b, min: 3
  - kind: uses-operator, target: "-", min: 5
  - kind: uses-name, target: lr, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution, CHI phan
  # solution -- khong tinh phan test noi them): loi giai dung dat=true, ca
  # bon luat qua sach (gan-ten w=3: "w = Value(0.0)" [gan ten moi], "w.data
  # -= ..." [AugAssign, Name(w) xuat hien trong dich], "w.grad = 0.0"
  # [Assign, Name(w) trong dich] -- tuong tu cho b; "-"=5: hai AugAssign tru
  # trong buoc cap nhat w.data/b.data, cong out.grad tru trong __sub__ [tinh
  # ca 2 nhanh cua no], cong p-t trong mse_qua_value).
  #
  # GOTCHA TRUNG TAM: neu bo hai dong "w.grad = 0.0" / "b.grad = 0.0" (quen
  # zero_grad), gan-ten w/b tut tu 3 xuong 2 -- bi chan boi static. Da tu
  # kiem chung bang Python that: cheat nay con lam SAI CA loss epoch cuoi
  # (0.6875 thay vi 0.0097), SAI w/b cuoi, VA lam mat tinh don dieu cua loss
  # (co buoc TANG) -- bi chan doc lap boi CA BON tier (static, run khong,
  # nhung tests/output chac chan).
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^41\\.0\\n0\\.0097\\n2\\.0814 0\\.7606\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Loss từ `41.0` xuống `0.0097` — giảm đơn điệu suốt `30` epoch. Quên
`zero_grad` không báo lỗi gì, chỉ lặng lẽ cho ra một mô hình sai — hai bước
kiểm tra khác nhau đã bắt được nó.
::::

::::reflect{#nghi-lai}
Vòng lặp huấn luyện đầy đủ — forward, loss, backward, cập nhật, `zero_grad`
— giờ chạy được trên BẤT KỲ tham số nào bọc trong `Value`, BẤT KỲ hàm mất
mát nào (MSE hay cross-entropy). Bốn điểm dữ liệu này chỉ là ví dụ nhỏ nhất
có thể chạy tay đối chiếu; cấu trúc vòng lặp không đổi dù dữ liệu lớn hay
nhỏ.

Nhưng vòng lặp ở đây dùng CẢ BỐN điểm dữ liệu ở MỌI epoch — gọi là
**full-batch**. Với dữ liệu lớn hơn, tính gradient trên toàn bộ dữ liệu mỗi
bước có thể chậm. Bài sau so sánh full-batch với hai cách khác: chia thành
từng nhóm nhỏ, hoặc dùng đúng MỘT điểm mỗi bước.
::::

::::checkpoint{mastery=0.85}
::::
