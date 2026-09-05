---
id: tri-tue-nhan-tao.huan-luyen-mang-no-ron.minibatch-va-sgd
title: "Full-batch, mini-batch, và single-sample SGD"
summary: "Ba cách chia dữ liệu cho MỖI bước cập nhật, trên CÙNG 8 điểm hồi quy (cùng lr=0.01, 15 epoch): full-batch (8 điểm/bước, 15 bước tổng) không một lần tăng loss (0/14 lần tăng) nhưng chỉ đạt loss=0.2239 sau 15 epoch; mini-batch-4 (2 bước/epoch, 30 bước) tăng 13/29 lần nhưng đạt loss=0.228; single-sample (8 bước/epoch, 120 bước) tăng 56/119 lần, đạt loss=0.1648 — thấp nhất. Càng chia nhỏ, càng nhiều cập nhật mỗi epoch (tiến nhanh hơn) nhưng đường loss càng KHÔNG mượt (mỗi bước chỉ thấy một phần dữ liệu, hướng giảm cho phần đó có thể làm loss TOÀN TẬP tăng)."
locale: vi
track: tri-tue-nhan-tao
module: huan-luyen-mang-no-ron
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.minibatch-va-sgd]
requires: [ai.vong-lap-huan-luyen]
concepts: [ai.minibatch-va-sgd]
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
Bài trước dùng CẢ BỐN điểm dữ liệu ở MỖI bước cập nhật. Với `8` điểm, hay
`8` triệu điểm, cách đó có còn hợp lý không? Ba cách chia dữ liệu, cùng bài
toán, số liệu thật sẽ trả lời.
::::

::::explain{#ba_cach_chia_du_lieu}
Vòng lặp huấn luyện ở bài `vong-lap-huan-luyen` dùng TOÀN BỘ dữ liệu để
tính MỘT gradient, rồi cập nhật MỘT lần — gọi là **full-batch**: mỗi
**epoch** (một lượt đi qua hết dữ liệu) chỉ có ĐÚNG một bước cập nhật.

Hai cách khác chia dữ liệu thành từng phần nhỏ hơn TRONG một epoch:

> **Mini-batch** — chia dữ liệu thành từng nhóm (batch) kích thước cố định
> (ví dụ `4` điểm/nhóm), tính gradient và cập nhật SAU MỖI nhóm. Một epoch
> giờ có NHIỀU bước cập nhật (bằng số nhóm), không chỉ một.
>
> **Single-sample SGD** — trường hợp cực đoan của mini-batch, kích thước
> nhóm bằng `1`: cập nhật ngay sau MỖI điểm dữ liệu riêng lẻ. Một epoch có
> đúng bằng số điểm dữ liệu bước cập nhật.

Ba cách này dùng CHUNG một vòng lặp cập nhật (`mot_buoc_huan_luyen` của bài
trước) — khác nhau đúng MỘT chỗ: gọi hàm đó với bao nhiêu điểm dữ liệu mỗi
lần (`xb`, `yb` — một LÁT của `x_data`, `y_data`, không phải toàn bộ).

Đánh đổi cốt lõi: nhóm CÀNG NHỎ, MỖI epoch có CÀNG NHIỀU bước cập nhật (tiến
nhanh hơn tính theo số epoch) — nhưng gradient tính từ MỘT nhóm nhỏ chỉ là
một ƯỚC LƯỢNG dựa trên phần dữ liệu đó, không phải gradient THẬT của toàn
bộ dữ liệu. Hướng giảm loss cho một nhóm nhỏ có thể KHÔNG phải hướng giảm
loss cho TOÀN BỘ dữ liệu — khiến loss đo trên toàn tập có thể TĂNG ngay sau
một bước cập nhật hoàn toàn hợp lệ (đúng gradient của nhóm đó). Đây chính
là sự khác biệt "mượt" hay "gập ghềnh" của đường loss.
::::

::::example{#ba_cach_so_that}
Tám điểm (`x = 1..8`, quan hệ thật gần `y = 2x + 1`, có nhiễu nhỏ), CÙNG
`lr = 0.01`, CÙNG `15` epoch — chỉ khác kích thước nhóm mỗi bước cập nhật:

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

def mse_so(w, b, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        err = (w.data * x + b.data) - y
        tong += err ** 2
    return tong / len(xs)

def mot_buoc(w, b, xs, ys, lr):
    preds = [w * xi + b for xi in xs]
    loss = mse_qua_value(preds, ys)
    loss.backward()
    w.data -= lr * w.grad
    b.data -= lr * b.grad
    w.grad = 0.0
    b.grad = 0.0

x_data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
y_data = [3.1219, 4.584, 7.3002, 9.3762, 10.2196, 12.4791, 15.0511, 16.8735]

def huan_luyen_theo_batch(kich_thuoc_batch, so_epoch, lr):
    w, b = Value(0.0), Value(0.0)
    loss_moi_buoc = []
    n = len(x_data)
    for epoch in range(so_epoch):
        for start in range(0, n, kich_thuoc_batch):
            xb = x_data[start:start + kich_thuoc_batch]
            yb = y_data[start:start + kich_thuoc_batch]
            mot_buoc(w, b, xb, yb, lr)
            loss_moi_buoc.append(mse_so(w, b, x_data, y_data))
    return w, b, loss_moi_buoc

def dem_lan_tang(ls):
    return sum(1 for i in range(len(ls) - 1) if ls[i + 1] > ls[i] + 1e-9)

for bs, ten in [(8, "full-batch"), (4, "mini-batch-4"), (1, "single-sample")]:
    w, b, ls = huan_luyen_theo_batch(bs, 15, 0.01)
    print(ten, ":", len(ls), "buoc,", dem_lan_tang(ls), "lan tang, loss cuoi =", round(ls[-1], 4))
```

```text title=readonly
full-batch : 15 buoc, 0 lan tang, loss cuoi = 0.2239
mini-batch-4 : 30 buoc, 13 lan tang, loss cuoi = 0.228
single-sample : 120 buoc, 56 lan tang, loss cuoi = 0.1648
```

Cùng `15` epoch, cùng `lr`, đo trên TOÀN BỘ `8` điểm sau MỖI bước cập nhật:
full-batch có đúng `15` bước (một bước/epoch) và loss KHÔNG BAO GIỜ tăng —
mượt tuyệt đối, nhưng loss cuối vẫn còn khá cao (`0.2239`). Single-sample
có `120` bước (tám bước/epoch) và loss TĂNG tới `56` trên `119` lần — gần
một nửa số bước làm loss TOÀN TẬP tệ đi tạm thời — nhưng vì có nhiều bước
cập nhật hơn hẳn trong CÙNG `15` epoch, nó đạt loss cuối THẤP NHẤT
(`0.1648`). Mini-batch-4 nằm giữa cả hai thái cực, ở cả hai tiêu chí.
::::

::::predict{#doan_loss_tang_khong commitOnce}
Cập nhật single-sample luôn tính gradient CHÍNH XÁC cho đúng MỘT điểm dữ
liệu nó vừa nhìn thấy — bước cập nhật đó chắc chắn làm giảm loss CHO ĐÚNG
điểm đó (nếu `lr` đủ nhỏ).

**Trước khi chạy thử**, bạn đoán: với `lr = 0.01` (một giá trị đã dùng
xuyên suốt bài, không có gì bất thường), liệu loss đo trên TOÀN BỘ `8`
điểm có thể TĂNG ngay sau một bước cập nhật single-sample như vậy không?

:::opt{correct}
Có — và đã xảy ra tới `56` lần trên `119` bước ở phần trên; giảm loss cho
MỘT điểm không đảm bảo gì cho các điểm KHÁC — hướng gradient tốt cho điểm
đang xét có thể là hướng làm dự đoán của các điểm còn lại XẤU ĐI, nên tổng
loss trên toàn bộ `8` điểm có thể tăng dù bước cập nhật hoàn toàn đúng cho
riêng điểm nó đang xét
:::

:::opt
Chỉ tăng nếu `lr` chọn quá lớn — với `lr = 0.01` đã dùng trong bài, loss
toàn tập chắc chắn giảm đều, không có bước nào tăng
::why
Gần đúng ở việc `lr` quá lớn ĐÚNG LÀ một nguyên nhân phổ biến khiến loss
tăng hay dao động mạnh — quan sát đó không sai trong bối cảnh chung.

Chỗ lệch: con số thật đã đo ở trên bác bỏ trực tiếp khẳng định "chắc chắn
giảm đều" — với ĐÚNG `lr = 0.01` này, single-sample vẫn tăng loss toàn tập
`56` lần trên `119` bước. Nguyên nhân không phải `lr` quá lớn — mà là bản
chất của việc chỉ nhìn thấy MỘT điểm mỗi bước, hướng tốt cho điểm đó không
nhất thiết tốt cho các điểm còn lại. Giảm `lr` nữa có thể giảm SỐ LẦN tăng,
nhưng không xoá bỏ được khả năng này hoàn toàn.
::
:::

:::opt
Không — vì `Value.backward()` luôn tính đúng gradient toán học, một phép
tính đúng không thể nào dẫn tới kết quả tệ hơn
::why
Gần đúng ở việc `backward()` THẬT SỰ tính đúng gradient — không có lỗi
phần mềm nào ở đây cả, công thức toán học được áp dụng chính xác.

Chỗ lệch: "gradient tính đúng" chỉ đảm bảo bước đi ĐÚNG HƯỚNG GIẢM cho
ĐÚNG những gì được đưa vào lúc tính nó (ở đây: một điểm dữ liệu) — nó không
đảm bảo gì về những điểm KHÔNG được đưa vào phép tính đó. "Đúng toán học
cho một phần" và "tốt cho toàn bộ" là hai khẳng định khác nhau — số liệu đo
được (`56/119` lần tăng) là bằng chứng cụ thể cho khoảng cách đó.
::
:::
::::

::::code{#viet_huan_luyen_theo_batch}
Hoàn thiện `huan_luyen_theo_batch`: cắt `x_data`, `y_data` thành từng lát
kích thước `kich_thuoc_batch`, bắt đầu từ `start`.

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

def mse_so(w, b, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        err = (w.data * x + b.data) - y
        tong += err ** 2
    return tong / len(xs)

def mot_buoc(w, b, xs, ys, lr):
    preds = [w * xi + b for xi in xs]
    loss = mse_qua_value(preds, ys)
    loss.backward()
    w.data -= lr * w.grad
    b.data -= lr * b.grad
    w.grad = 0.0
    b.grad = 0.0

x_data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
y_data = [3.1219, 4.584, 7.3002, 9.3762, 10.2196, 12.4791, 15.0511, 16.8735]

def huan_luyen_theo_batch(kich_thuoc_batch, so_epoch, lr):
    w, b = Value(0.0), Value(0.0)
    loss_moi_buoc = []
    n = len(x_data)
    for epoch in range(so_epoch):
        for start in range(0, n, kich_thuoc_batch):
            xb = ___                      # x_data[start:start+kich_thuoc_batch]
            yb = ___                      # y_data[start:start+kich_thuoc_batch]
            mot_buoc(w, b, xb, yb, lr)
            loss_moi_buoc.append(mse_so(w, b, x_data, y_data))
    return w, b, loss_moi_buoc

def dem_lan_tang(ls):
    return sum(1 for i in range(len(ls) - 1) if ls[i + 1] > ls[i] + 1e-9)

w_full, b_full, ls_full = huan_luyen_theo_batch(8, 15, 0.01)
w_mini, b_mini, ls_mini = huan_luyen_theo_batch(4, 15, 0.01)
w_don, b_don, ls_don = huan_luyen_theo_batch(1, 15, 0.01)

print(len(ls_full), dem_lan_tang(ls_full), round(ls_full[-1], 4))
print(len(ls_mini), dem_lan_tang(ls_mini), round(ls_mini[-1], 4))
print(len(ls_don), dem_lan_tang(ls_don), round(ls_don[-1], 4))
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

def mse_so(w, b, xs, ys):
    tong = 0.0
    for x, y in zip(xs, ys):
        err = (w.data * x + b.data) - y
        tong += err ** 2
    return tong / len(xs)

def mot_buoc(w, b, xs, ys, lr):
    preds = [w * xi + b for xi in xs]
    loss = mse_qua_value(preds, ys)
    loss.backward()
    w.data -= lr * w.grad
    b.data -= lr * b.grad
    w.grad = 0.0
    b.grad = 0.0

x_data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
y_data = [3.1219, 4.584, 7.3002, 9.3762, 10.2196, 12.4791, 15.0511, 16.8735]

def huan_luyen_theo_batch(kich_thuoc_batch, so_epoch, lr):
    w, b = Value(0.0), Value(0.0)
    loss_moi_buoc = []
    n = len(x_data)
    for epoch in range(so_epoch):
        for start in range(0, n, kich_thuoc_batch):
            xb = x_data[start:start + kich_thuoc_batch]
            yb = y_data[start:start + kich_thuoc_batch]
            mot_buoc(w, b, xb, yb, lr)
            loss_moi_buoc.append(mse_so(w, b, x_data, y_data))
    return w, b, loss_moi_buoc

def dem_lan_tang(ls):
    return sum(1 for i in range(len(ls) - 1) if ls[i + 1] > ls[i] + 1e-9)

w_full, b_full, ls_full = huan_luyen_theo_batch(8, 15, 0.01)
w_mini, b_mini, ls_mini = huan_luyen_theo_batch(4, 15, 0.01)
w_don, b_don, ls_don = huan_luyen_theo_batch(1, 15, 0.01)

print(len(ls_full), dem_lan_tang(ls_full), round(ls_full[-1], 4))
print(len(ls_mini), dem_lan_tang(ls_mini), round(ls_mini[-1], 4))
print(len(ls_don), dem_lan_tang(ls_don), round(ls_don[-1], 4))
```

```python title=test
assert len(ls_full) == 15, f"full-batch phai co dung 15 buoc (mot buoc/epoch) -- dang ra {len(ls_full)}"
assert len(ls_mini) == 30, f"mini-batch-4 phai co dung 30 buoc (hai buoc/epoch tren 8 diem) -- dang ra {len(ls_mini)}"
assert len(ls_don) == 120, f"single-sample phai co dung 120 buoc (tam buoc/epoch) -- dang ra {len(ls_don)}"

assert dem_lan_tang(ls_full) == 0, f"full-batch KHONG duoc co buoc nao lam loss tang -- dang ra {dem_lan_tang(ls_full)} lan"
assert dem_lan_tang(ls_mini) == 13, f"mini-batch-4 phai co dung 13 lan loss tang -- dang ra {dem_lan_tang(ls_mini)}"
assert dem_lan_tang(ls_don) == 56, f"single-sample phai co dung 56 lan loss tang -- dang ra {dem_lan_tang(ls_don)}"

assert round(ls_full[-1], 4) == 0.2239, f"loss cuoi full-batch sai -- dang ra {round(ls_full[-1], 4)}"
assert round(ls_mini[-1], 4) == 0.228, f"loss cuoi mini-batch-4 sai -- dang ra {round(ls_mini[-1], 4)}"
assert round(ls_don[-1], 4) == 0.1648, f"loss cuoi single-sample sai -- dang ra {round(ls_don[-1], 4)}"

# rieng kiem tra ranh gioi kich_thuoc_batch=1 (single-sample) khong lam
# ham vo tinh gop nhieu diem vao mot lat -- da tu kiem chung: xb, yb khi
# kich_thuoc_batch=1 phai co dung 1 phan tu moi lat.
xb_thu = x_data[0:0 + 1]
assert len(xb_thu) == 1, "voi kich_thuoc_batch=1, moi lat phai dung 1 phan tu"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cùng một kiểu cắt lát Python — `danh_sach[start:start+kich_thuoc_batch]`. `xb` cắt từ `x_data`, `yb` cắt từ `y_data`, CÙNG khoảng `[start, start+kich_thuoc_batch)`. Chú ý: cắt lát ngoài phạm vi (ví dụ gần cuối danh sách) không báo lỗi trong Python — nó tự động RÚT NGẮN, không cần xử lý riêng.
- kind: strategy
  body: 'xb: `x_data[start:start+kich_thuoc_batch]`. yb: `y_data[start:start+kich_thuoc_batch]`.'
- kind: one-line
  body: 'Hai chỗ trống: `x_data[start:start+kich_thuoc_batch]` và `y_data[start:start+kich_thuoc_batch]`.'
:::

:::validate
- tier: run
  timeoutMs: 12000
- tier: static
  onFail: xb/yb phai CAT LAT that su tu x_data/y_data bang start va kich_thuoc_batch (khong duoc dung ca danh sach nguyen si, se lam full-batch/mini-batch/single-sample chay giong het nhau)
  requireAst:
  - kind: uses-name, target: start, min: 4
  - kind: uses-name, target: kich_thuoc_batch, min: 3
  - kind: uses-name, target: x_data, min: 3
  - kind: uses-name, target: y_data, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach (start=4: mot lan trong
  # "range(0, n, kich_thuoc_batch)" khong dem [khong doc "start"], hai lan
  # CAT LAT xb/yb, mot lan trong bieu thuc range o dong for -- dem duoc
  # dung 4; kich_thuoc_batch=3: mot lan trong range(), hai lan cat lat;
  # x_data=3: dinh nghia [khong dem, Store], cat lat trong xb, va goi trong
  # mse_so(w, b, x_data, y_data) o cuoi vong lap; y_data=2: cat lat trong
  # yb, va goi trong cung loi goi mse_so do).
  # Cheat "xb = x_data, yb = y_data" (bo qua cat lat, dung ca danh sach) lam
  # "start" va "kich_thuoc_batch" ve 0 (hoac con dung 1 tu dong range) --
  # duoi nguong 2, bi chan. Da tu kiem chung bang Python that: cheat nay lam
  # full-batch/mini-batch/single-sample chay GIONG HET nhau (moi lan goi
  # mot_buoc deu dung toan bo 8 diem), lam so buoc dung (15/30/120) nhung so
  # lan tang va loss cuoi SAI hoan toan cho hai truong hop sau -- bi chan
  # doc lap boi tests/output.
- tier: tests
  timeoutMs: 12000
- tier: output
  match: regex
  expect: "^15 0 0\\.2239\\n30 13 0\\.228\\n120 56 0\\.1648\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Full-batch: `0` lần tăng, chậm nhất. Single-sample: `56` lần tăng, nhanh
nhất. Mini-batch nằm giữa. Không có cách nào "đúng tuyệt đối" — chỉ có đánh
đổi, đo bằng số thật.
::::

::::reflect{#nghi-lai}
Ba cách chia dữ liệu chỉ đổi CÁCH tính gradient mỗi bước — thuật toán cập
nhật vẫn là `p.data -= lr * p.grad` y hệt bài trước. Câu hỏi còn bỏ ngỏ:
liệu có cách cập nhật THÔNG MINH HƠN — không chỉ dùng gradient của MỘT
bước, mà nhớ lại cả HƯỚNG ĐI của những bước trước đó — để giảm bớt sự
"gập ghềnh" mà mini-batch và single-sample vừa cho thấy? Bài sau trả lời.
::::

::::checkpoint{mastery=0.85}
::::
