---
id: tri-tue-nhan-tao.dao-ham-tu-dong.tich-luy-gradient
title: "Tích luỹ gradient"
summary: "Gotcha trung tâm của autograd: một Value dùng LẶP LẠI trong đồ thị phải CỘNG DỒN (+=) gradient qua mọi đường, không GHI ĐÈ (=). Trên y=x*x+x (x=3.0, x dùng 2 lần): bản ĐÚNG (+=) cho x.grad=7.0, bản SAI (=) cho x.grad=3.0 — hai số khác nhau, tự kiểm chứng bằng Pyodide thật, không phải suy luận tay."
locale: vi
track: tri-tue-nhan-tao
module: dao-ham-tu-dong
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.tich-luy-gradient]
requires: [ai.sap-xep-to-po-va-lan-truyen-nguoc]
concepts: [ai.tich-luy-gradient]
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
Mọi đồ thị từ đầu quest tới giờ đều "hiền": mỗi lá chỉ xuất hiện đúng một
lần. Đời thật không hiền như vậy — và đây là chỗ autograd dễ sai nhất.
::::

::::explain{#gotcha_tich_luy}
Xét `y = x·x + x` — biến `x` xuất hiện HAI LẦN: một lần trong phép nhân
`x·x`, một lần nữa trong phép cộng `+ x`. Đồ thị của biểu thức này KHÔNG còn
là một cây (tree) — nó là một DAG thật sự, nơi node `x` có HAI cạnh đi ra
(một tới node nhân, một tới node cộng), không phải một.

Điều này thay đổi một điều quan trọng ở `_backward()`: gradient của `x`
không còn nhận đóng góp từ đúng MỘT đường — nó nhận đóng góp từ HAI đường
khác nhau (từ phép nhân, VÀ từ phép cộng), và HAI đóng góp đó phải **CỘNG
DỒN** lại với nhau, không được cái sau GHI ĐÈ lên cái trước.

Đây chính xác là lý do mọi `_backward()` viết từ bài `lop-value-cong-tru-
nhan` tới giờ đều dùng `self.grad += ...` (cộng dồn), KHÔNG dùng
`self.grad = ...` (ghi đè). Với một đồ thị "hiền" (mỗi lá dùng một lần) hai
cách viết cho CÙNG kết quả — không có gì để phân biệt. Với `x` dùng hai lần
như trên, hai cách viết cho HAI kết quả KHÁC NHAU:

> Bản **ĐÚNG** (`+=`): mỗi lần một node cha lan gradient vào `x`, giá trị đó
> được CỘNG THÊM vào `x.grad` đã có — không mất đóng góp nào.
>
> Bản **SAI** (`=`): mỗi lần một node cha lan gradient vào `x`, giá trị đó
> GHI ĐÈ lên `x.grad` — đóng góp của đường ĐI TRƯỚC bị xoá mất, chỉ đường đi
> SAU CÙNG còn lại.

Quy tắc chung, áp dụng cho MỌI đồ thị: gradient của một node là TỔNG đóng
góp từ MỌI đường nó tham gia tạo ra kết quả cuối — không phải giá trị của
đường GẦN NHẤT.
::::

::::example{#dung_vs_sai_bang_so}
Cùng một biểu thức `y = x·x + x` (`x = 3.0`), hai class chỉ khác đúng một
chỗ — `+=` so với `=` trong `_backward`:

```python title=readonly
class ValueDung:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        other = other if isinstance(other, ValueDung) else ValueDung(other)
        out = ValueDung(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad          # CONG DON
            other.grad += out.grad         # CONG DON
        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, ValueDung) else ValueDung(other)
        out = ValueDung(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad += other.data * out.grad     # CONG DON
            other.grad += self.data * out.grad      # CONG DON
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


class ValueSai:
    def __init__(self, data, _prev=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def __add__(self, other):
        other = other if isinstance(other, ValueSai) else ValueSai(other)
        out = ValueSai(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad = out.grad           # GHI DE
            other.grad = out.grad          # GHI DE
        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, ValueSai) else ValueSai(other)
        out = ValueSai(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad = other.data * out.grad      # GHI DE
            other.grad = self.data * out.grad       # GHI DE
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


x1 = ValueDung(3.0)
y1 = x1 * x1 + x1
y1.backward()
print("DUNG (+=):", y1.data, x1.grad)

x2 = ValueSai(3.0)
y2 = x2 * x2 + x2
y2.backward()
print("SAI (=):  ", y2.data, x2.grad)
```

```text title=readonly
DUNG (+=): 12.0 7.0
SAI (=):   12.0 3.0
```

Cùng biểu thức, cùng `x = 3.0`, cùng `y = 12.0` — nhưng `x.grad` khác hẳn:
bản ĐÚNG cho `7.0`, bản SAI cho `3.0`. Con số `7.0` khớp với đạo hàm thật của
`y = x² + x` (đạo hàm này, giống mọi đạo hàm khác trong quest, được XÁC NHẬN
bằng cách chạy code — bài `kiem-dao-ham-bang-so` ngay sau đây dạy đúng cách
xác nhận đó một cách có hệ thống). `3.0` chỉ là đóng góp của MỘT đường (từ
phép cộng cuối) — đóng góp từ phép nhân đã bị GHI ĐÈ mất.
::::

::::predict{#doan_neu_x_dung_ba_lan commitOnce}
`y = x·x + x` dùng `x` HAI lần, và bản SAI (ghi đè) cho `x.grad = 3.0` thay
vì `7.0` đúng.

**Trước khi đọc lại**, bạn đoán: nếu đổi biểu thức thành `y = x·x + x + x`
(dùng `x` BA lần), bản SAI (vẫn ghi đè, không cộng dồn) sẽ cho `x.grad` là
bao nhiêu?

:::opt{correct}
Vẫn `1.0` — dù `x` dùng bao nhiêu lần, bản GHI ĐÈ luôn chỉ giữ lại đóng góp
của lần lan gradient SAU CÙNG; với phép cộng, đóng góp mỗi lần luôn là
`out.grad = 1.0`, nên ghi đè bao nhiêu lần cũng dừng lại ở `1.0`
:::

:::opt
`4.0` — vì có bốn lượt lan gradient vào `x` (một từ phép nhân, ba từ ba
phép cộng), và bản sai cộng thiếu một cách nào đó nhưng không mất trắng
::why
Gần đúng ở việc ĐẾM đúng số lượt lan gradient vào `x` trong biểu thức mới —
quan sát về SỐ LƯỢT không sai.

Chỗ lệch: bản SAI không "cộng thiếu" — nó GHI ĐÈ hoàn toàn, nghĩa là mỗi
lượt XOÁ SẠCH lượt trước, không giữ lại một phần nào. Dù có bốn lượt hay
chỉ hai lượt, kết quả cuối luôn là giá trị của lượt CUỐI CÙNG, không phải
một phần tổng nào.
::
:::

:::opt
Không xác định được nếu không chạy thử — số lần dùng lại một biến càng
nhiều thì hành vi ghi đè càng khó đoán trước
::why
Gần đúng ở tinh thần cẩn trọng khi thay đổi biểu thức — kiểm tra lại bằng
cách chạy thử không phải một thói quen tồi.

Chỗ lệch: có một QUY LUẬT cố định ở đây, không phụ thuộc số lần dùng lại:
bản ghi đè luôn kết thúc bằng giá trị CỦA LƯỢT LAN CUỐI CÙNG, và với phép
`+` (đạo hàm cục bộ luôn là `1`), giá trị lan qua luôn là `out.grad` của
bước đó — với `y` là đầu ra, `out.grad` của MỌI bước cộng trực tiếp vào `y`
đều là `1.0`, bất kể có bao nhiêu bước như vậy.
::
:::
::::

::::code{#hoan_thien_tich_luy}
Hoàn thiện `_backward` của `__add__` và `__mul__`: cả bốn chỗ trống đều là
PHÉP CỘNG DỒN (`+=`), không phải phép gán (`=`).

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
            self.grad ___ out.grad         # +=, khong phai =
            other.grad ___ out.grad        # +=, khong phai =
        out._backward = _backward
        return out

    def __mul__(self, other):
        other = other if isinstance(other, Value) else Value(other)
        out = Value(self.data * other.data, (self, other), '*')
        def _backward():
            self.grad ___ other.data * out.grad     # +=
            other.grad ___ self.data * out.grad      # +=
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

x = Value(3.0)
m = x * x
y = m + x
y.backward()

print(round(y.data, 4))
print(round(x.grad, 4))
print(round(m.grad, 4))
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

x = Value(3.0)
m = x * x
y = m + x
y.backward()

print(round(y.data, 4))
print(round(x.grad, 4))
print(round(m.grad, 4))
```

```python title=test
assert round(y.data, 4) == 12.0, f"y.data sai -- dang ra {round(y.data, 4)}"
assert round(m.grad, 4) == 1.0, f"m.grad sai -- dang ra {round(m.grad, 4)}"

# gotcha trung tam: x dung LAP LAI hai lan (trong x*x va trong +x). Neu
# _backward ghi de (=) thay vi cong don (+=), x.grad se ra 3.0 thay vi 7.0
# dung -- da tu kiem chung bang Pyodide that (ban ghi de va ban cong don cho
# hai ket qua KHAC NHAU tren chinh bieu thuc nay).
assert round(x.grad, 4) == 7.0, f"x.grad phai la 7.0 (CONG DON qua ca hai duong, khong GHI DE) -- dang ra {round(x.grad, 4)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống, đều là MỘT toán tử — `+=`, không phải `=`. `x` trong `y = x*x + x` được dùng LẶP LẠI hai lần, nên gradient của nó phải nhận đóng góp từ CẢ HAI đường (từ phép nhân, và từ phép cộng) — `+=` giữ lại cả hai, `=` chỉ giữ lại đường lan SAU CÙNG.
- kind: strategy
  body: 'Cả bốn chỗ trống đều là `+=` (cộng dồn vào `self.grad`/`other.grad` đã có, không ghi đè).'
- kind: one-line
  body: 'Bốn chỗ trống đều là `+=`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: ca bon _backward phai CONG DON (+=) vao self.grad/other.grad, khong duoc GHI DE (=) -- x dung lap lai hai lan trong y=x*x+x, ghi de se lam mat dong gop tu mot trong hai duong
  requireAst:
  - kind: uses-operator, target: "+", min: 6
  - kind: uses-operator, target: "*", min: 4
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca hai luat qua sach ("+"=6: bon AugAssign += trong bon
  # closure, cong self.data+other.data trong __add__, cong y=m+x o duoi; "*"
  # =4: self.data*other.data trong __mul__, hai cong thuc trong closure cua
  # no, va x*x o duoi). Doi CA BON += thanh = (ban SAI) lam "+" tut tu 6
  # xuong 2 -- bi chan CHAC CHAN boi static. Doi RIENG LE mot trong bon +=
  # thanh = cung lam "+" tut xuong 5, duoi nguong 6 -- bi chan. Rieng gia tri
  # x.grad=7.0 (dung) vs 3.0 (sai neu ghi de toan bo) da tu kiem chung bang
  # Pyodide that, doc lap voi luat static nay.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^12\\.0\\n7\\.0\\n1\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`x.grad = 7.0` — cộng dồn qua cả hai đường mà `x` tham gia. Ghi đè cho
`3.0`, mất trắng một nửa đóng góp. Bài sau: một công cụ để PHÁT HIỆN lỗi
kiểu này mà không cần biết trước đáp án đúng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ví dụ ở bài này biết trước đáp án đúng (`7.0`) nhờ tính tay đạo hàm của
`y = x² + x`. Nhưng với một biểu thức PHỨC TẠP hơn nhiều — nhiều phép toán,
nhiều biến dùng lặp lại — tính tay để biết "đáp án đúng" không còn khả thi.
Làm sao kiểm tra được `Value.backward()` có đúng hay không, khi không còn
cách nào tính tay đối chiếu?
::::

::::checkpoint{mastery=0.85}
::::
