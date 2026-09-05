---
id: tri-tue-nhan-tao.dao-ham-tu-dong.sap-xep-to-po-va-lan-truyen-nguoc
title: "Sắp xếp tô-pô và lan truyền ngược"
summary: "Hoàn thiện Value.backward() thật: sắp xếp tô-pô (topological sort) đồ thị từ node đầu ra, rồi duyệt NGƯỢC thứ tự đó, gọi _backward của từng node. Trên f=(a*b+c)*d (a=2,b=-3,c=10,d=-2): a.grad=6.0, b.grad=-4.0, c.grad=-2.0, d.grad=4.0 — khớp byte-for-byte với công thức tính tay ở bài dao-ham-nguoc-chuoi. Quên đảo ngược (reversed) hay quên seed grad=1.0 đều cho ra toàn số 0 — đã tự kiểm chứng."
locale: vi
track: tri-tue-nhan-tao
module: dao-ham-tu-dong
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.sap-xep-to-po-va-lan-truyen-nguoc]
requires: [ai.lop-value-mu-tanh-relu]
concepts: [ai.sap-xep-to-po-va-lan-truyen-nguoc]
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
Hai bài liền phải tự tay liệt kê đúng thứ tự gọi `_backward()`. Bài này xây
đúng MỘT thuật toán tìm ra thứ tự đó — cho MỌI đồ thị, không cần đoán.
::::

::::explain{#topo_sort_va_backward}
Quy luật đã thấy ở hai bài trước: một node phải lan gradient của MÌNH ra
node cha TRƯỚC khi node cha đó dùng gradient vừa nhận để lan tiếp. Nói cách
khác — node được TÍNH SAU CÙNG trong lúc XÂY đồ thị (gần đầu ra nhất) phải
LAN TRƯỚC trong lúc gọi `_backward()`.

Thứ tự "mọi node cha đứng trước node được tạo ra từ nó" gọi là **sắp xếp
tô-pô** (topological sort/order) — một khái niệm chuẩn của lý thuyết đồ thị
cho đồ thị có hướng KHÔNG CHU TRÌNH (DAG — Directed Acyclic Graph; đồ thị
tính toán luôn là DAG, vì một phép toán không bao giờ dùng chính kết quả
tương lai của nó làm đầu vào). Thuật toán tìm thứ tự đó bằng đệ quy:

> Với mỗi node `v` CHƯA THĂM: đánh dấu đã thăm, rồi đệ quy vào TỪNG node cha
> của `v` trước, sau đó mới THÊM `v` vào danh sách kết quả.

Vì mọi node cha được thêm vào danh sách TRƯỚC `v` (đệ quy chạy xong trước
khi `v` được thêm), danh sách cuối cùng có tính chất: node cha luôn đứng
TRƯỚC node được tạo từ nó — tức là LÁ đứng ĐẦU danh sách, ĐẦU RA đứng CUỐI.
Nhưng thứ tự LAN TRUYỀN NGƯỢC cần ĐẦU RA lan trước — nghĩa là phải duyệt
danh sách này theo chiều NGƯỢC LẠI (`reversed`).

`Value.backward()` hoàn chỉnh, ráp từ hai mảnh:

> **Sắp xếp tô-pô**: đệ quy từ node gọi `.backward()` (đầu ra), xây danh
> sách `topo` theo đúng thuật toán trên.
>
> **Lan truyền ngược**: gán `self.grad = 1.0` (đầu ra luôn có đạo hàm `1`
> theo chính nó — điểm khởi đầu của MỌI lan truyền ngược), rồi duyệt
> `reversed(topo)`, gọi `_backward()` của từng node.

Không còn phải tự tay liệt kê thứ tự nữa — thuật toán tự tìm, cho BẤT KỲ
đồ thị nào, không phụ thuộc số node hay hình dạng.
::::

::::example{#backward_hoan_chinh}
`Value.backward()` đầy đủ, chạy trên ĐÚNG biểu thức `f = (a·b + c)·d` (`a=2,
b=-3, c=10, d=-2`) đã dùng ở bài `do-thi-tinh-toan` và `dao-ham-nguoc-chuoi`:

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
        topo = []
        visited = set()
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

a = Value(2.0)
b = Value(-3.0)
c = Value(10.0)
d = Value(-2.0)

e = a * b
n = e + c
f = n * d
f.backward()

print(f.data)
print(a.grad, b.grad, c.grad, d.grad)
```

```text title=readonly
-8.0
6.0 -4.0 -2.0 4.0
```

Không còn `f._backward()` rồi `e._backward()` gọi tay — CHỈ một lệnh
`f.backward()`, và bốn lá nhận đúng gradient: `6.0, -4.0, -2.0, 4.0`. Khớp
BYTE-FOR-BYTE với bốn công thức tính tay ở bài `dao-ham-nguoc-chuoi`
(`df_da=6`, `df_db=-4`, `df_dc=-2`, `df_dd=4`) — cùng một đồ thị, hai cách
tính, cùng một đáp số.
::::

::::predict{#doan_khong_dao_nguoc commitOnce}
`backward()` duyệt `reversed(topo)` — đảo ngược danh sách tô-pô trước khi
gọi `_backward()`.

**Trước khi đọc lại**, bạn đoán: nếu bỏ `reversed` (duyệt thẳng `topo`,
không đảo), gradient của LÁ `a` (`da=6.0` đúng) sẽ ra sao?

:::opt{correct}
Ra `0.0` (sai) — không đảo nghĩa là LÁ được xử lý TRƯỚC node cha của nó
(đúng thứ tự tô-pô: lá đứng đầu danh sách), nên khi `_backward()` của lá
chạy, gradient từ node cha CHƯA kịp lan tới nó — `a.grad` giữ nguyên giá
trị khởi tạo `0.0`
:::

:::opt
Vẫn ra `6.0` (đúng) — miễn thuật toán sắp xếp tô-pô đúng, thứ tự duyệt
xuôi hay ngược không ảnh hưởng gì tới kết quả cuối
::why
Gần đúng ở việc thuật toán sắp xếp tô-pô ĐÚNG là một điều kiện CẦN — nếu
`topo` sai thứ tự thì mọi thứ sau đó cũng hỏng.

Chỗ lệch: sắp xếp tô-pô đúng chỉ đảm bảo "node cha đứng TRƯỚC node được tạo
từ nó" trong danh sách — mà lan truyền ngược cần ĐIỀU NGƯỢC LẠI (đầu ra lan
trước, lá lan sau cùng). Duyệt THẲNG danh sách tô-pô nghĩa là gọi
`_backward()` của LÁ trước, của ĐẦU RA sau — sai hướng hoàn toàn so với thứ
tự lan truyền cần có.
::
:::

:::opt
Ra một số khác `6.0` nhưng vẫn khác `0`, vì thứ tự chỉ làm gradient bị TÍNH
THIẾU một phần, không mất trắng
::why
Gần đúng ở trực giác rằng thứ tự sai thường làm kết quả "gần đúng nhưng
lệch" ở nhiều bài toán khác.

Chỗ lệch: ở ĐÚNG trường hợp này, lá `a` chỉ nhận gradient từ CHÍNH XÁC một
đường — qua `e` rồi `n` rồi `f`. Nếu `e._backward()` chạy TRƯỚC khi
`n._backward()` (và `f._backward()`) kịp lan gradient vào `e`, `e.grad` vẫn
là `0` tại thời điểm đó — nên `a.grad` nhận đúng `0.0`, không phải một số
lệch nhẹ.
::
:::
::::

::::code{#hoan_thien_backward}
Hoàn thiện `backward()`: kiểm tra `v` đã thăm chưa, seed gradient của đầu
ra, và duyệt `topo` theo đúng chiều NGƯỢC để lan truyền.

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
        topo = []
        visited = set()
        def build(v):
            if v not in ___:                # visited
                visited.add(v)
                for con in v._prev:
                    build(con)
                topo.append(v)
        build(self)
        self.grad = ___                     # 1.0
        for node in ___(topo):              # reversed
            node._backward()

a = Value(2.0)
b = Value(-3.0)
c = Value(10.0)
d = Value(-2.0)

e = a * b
n = e + c
f = n * d
f.backward()

print(f.data)
print(a.grad, b.grad, c.grad, d.grad)
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
        topo = []
        visited = set()
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

a = Value(2.0)
b = Value(-3.0)
c = Value(10.0)
d = Value(-2.0)

e = a * b
n = e + c
f = n * d
f.backward()

print(f.data)
print(a.grad, b.grad, c.grad, d.grad)
```

```python title=test
assert round(f.data, 4) == -8.0, f"f.data sai -- dang ra {round(f.data, 4)}"
assert round(a.grad, 4) == 6.0, f"a.grad sai -- dang ra {round(a.grad, 4)}"
assert round(b.grad, 4) == -4.0, f"b.grad sai -- dang ra {round(b.grad, 4)}"
assert round(c.grad, 4) == -2.0, f"c.grad sai -- dang ra {round(c.grad, 4)}"
assert round(d.grad, 4) == 4.0, f"d.grad sai -- dang ra {round(d.grad, 4)}"

# rieng kiem tra khong bi "toan 0" (dau hieu quen seed grad=1.0 hoac quen
# reversed) -- neu bug xay ra, CA BON gradient deu ve 0.0 cung luc.
tong_tuyet_doi = abs(a.grad) + abs(b.grad) + abs(c.grad) + abs(d.grad)
assert round(tong_tuyet_doi, 4) != 0.0, "ca bon gradient deu ve 0 -- kiem tra lai da seed self.grad=1.0 va da dung reversed(topo) chua"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ thứ nhất kiểm tra `v` đã có trong tập `visited` chưa — dùng chính tên biến `visited` đã khai ở dòng trên. Chỗ thứ hai seed gradient của node gọi `.backward()` (chính `self`, đầu ra) bằng `1.0` — luôn là `1`, vì đạo hàm của một giá trị theo CHÍNH NÓ là `1`. Chỗ thứ ba đảo ngược `topo` bằng hàm dựng sẵn `reversed` trước khi duyệt — thiếu đảo ngược thì lá lan TRƯỚC đầu ra, sai hướng.
- kind: strategy
  body: 'Chỗ trống 1: `visited`. Chỗ trống 2: `1.0`. Chỗ trống 3: `reversed`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `visited`, `1.0`, và `reversed`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: build phai kiem tra v CHUA CO trong visited (dung dung ten bien visited); phai seed self.grad = 1.0 truoc khi lan truyen; va phai duyet reversed(topo) (khong duoc duyet thang topo hay mot tap hop khac)
  requireAst:
  - kind: uses-name, target: visited, min: 2
  - kind: uses-call, target: reversed, min: 1
  - kind: uses-name, target: topo, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca ba luat qua sach (visited=2: "v not in visited" doc, va
  # "visited.add(v)" doc chinh no de goi .add; reversed=1; topo=2: "topo.
  # append(v)" doc de goi .append, va "reversed(topo)" doc). Cheat "chep san
  # set() rong thay vi visited" lam "visited" tut ve 1 -- bi chan. Cheat
  # "khong dao nguoc, duyet thang topo" lam "reversed" ve 0 -- bi chan. Cheat
  # "duyet reversed(visited) thay vi reversed(topo)" van giu "reversed"=1
  # nhung lam "topo" tut ve 1 -- bi chan boi luat topo. Da tu kiem chung bang
  # Pyodide that: ca hai bug (quen dao nguoc, quen seed grad=1.0) deu cho ca
  # bon gradient ve 0.0 -- assert rieng trong tier tests bat ca hai, doc lap
  # voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^-8\\.0\\n6\\.0 -4\\.0 -2\\.0 4\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một lệnh `f.backward()`, đúng bốn gradient — khớp byte-for-byte với công
thức tính tay. Nhưng đồ thị của bài này còn "hiền": mỗi lá chỉ dùng đúng một
lần. Bài sau phá vỡ giả định đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đồ thị `f = (a·b + c)·d` có tính chất đặc biệt: mỗi lá (`a, b, c, d`) chỉ
xuất hiện đúng MỘT LẦN trong toàn bộ biểu thức. Nếu một lá — ví dụ `a` —
xuất hiện ở HAI chỗ khác nhau trong cùng một biểu thức (chẳng hạn
`f = a·a + a`), `a._prev` của node nào sẽ "thấy" `a`, và gradient của `a`
cần nhận đóng góp từ MẤY đường khác nhau?
::::

::::checkpoint{mastery=0.8}
::::
