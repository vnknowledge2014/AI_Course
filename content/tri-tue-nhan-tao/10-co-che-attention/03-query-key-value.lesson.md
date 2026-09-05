---
id: tri-tue-nhan-tao.co-che-attention.query-key-value
title: "Query, Key, Value: ba phép chiếu từ cùng một embedding"
summary: "Q=X.matmul(Wq), K=X.matmul(Wk), V=X.matmul(Wv) -- ba phép chiếu tuyến tính TỪ CÙNG một X (embedding+vị trí), qua Tensor.matmul đã có sẵn (q8.3b), KHÔNG viết lại. Vai trò: Query 'tôi đang tìm gì', Key 'tôi đại diện cho gì' (so khớp với Query), Value 'tôi mang thông tin gì' (thứ thực sự được lấy ra). Trên X shape (4,4), ba ma trận trọng số RIÊNG biệt (4,4): Q,K,V đều shape (4,4) nhưng GIÁ TRỊ khác hẳn nhau dù CÙNG đầu vào X -- vì Wq≠Wk≠Wv."
locale: vi
track: tri-tue-nhan-tao
module: co-che-attention
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.query-key-value]
requires: [ai.ma-hoa-vi-tri]
concepts: [ai.query-key-value]
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
`X` (embedding cộng vị trí) đã sẵn sàng cho MỖI token. Nhưng attention không dùng thẳng `X` để so sánh các token — nó chiếu `X` qua BA ma trận riêng biệt trước. Vì sao lại cần tới ba?
::::

::::explain{#query_key_value}
Attention đo "token nào nên chú ý tới token nào" bằng cách so khớp MỘT phiên bản chiếu của token này với MỘT phiên bản chiếu KHÁC của token kia. Ba phép chiếu tuyến tính, từ CÙNG một `X` (shape `(so_token, dim)`), qua ba ma trận trọng số RIÊNG biệt (`Wq`, `Wk`, `Wv`, mỗi cái shape `(dim, dim)`):

> `Q = X.matmul(Wq)` — **Query** ("tôi đang tìm gì"): mỗi hàng của `Q` là câu hỏi mà token đó đặt ra khi nhìn các token khác.
>
> `K = X.matmul(Wk)` — **Key** ("tôi đại diện cho gì"): mỗi hàng của `K` là "nhãn" mà token đó đưa ra để được SO KHỚP với Query của token khác.
>
> `V = X.matmul(Wv)` — **Value** ("tôi mang thông tin gì"): mỗi hàng của `V` là nội dung THỰC SỰ được lấy ra, một khi Query và Key đã xác định token nào đáng chú ý.

Trực giác: nếu coi attention như một cuộc tra cứu (giống một từ điển) — `Query` là CÂU HỎI tra cứu, `Key` là NHÃN của từng mục trong từ điển (so khớp câu hỏi với nhãn nào gần nhất), và `Value` là NỘI DUNG của mục đó (thứ thực sự lấy ra sau khi đã chọn được mục khớp nhất). Ba vai trò khác nhau, nên cần BA ma trận trọng số khác nhau — nếu `Wq = Wk = Wv`, `Q`, `K`, `V` sẽ giống hệt `X` chiếu qua CÙNG một phép biến đổi, mất khả năng đóng ba vai trò riêng.

Cả ba phép chiếu đều là `Tensor.matmul` — ĐÃ CÓ SẴN, ĐÃ ĐÚNG cả forward lẫn backward (`nhan-ma-tran-forward`/`nhan-ma-tran-backward`, q8.3b) — không viết lại một dòng công thức nào.
::::

::::example{#qkv_that}
`X` shape `(4, 4)` — kết quả embedding cộng vị trí của một câu `4` token (đã xây ở hai bài trước) — chiếu qua ba ma trận trọng số `Wq`, `Wk`, `Wv`, mỗi cái shape `(4, 4)`:

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


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])   # shape (4, 4)

Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
V = X.matmul(Wv)

print(np.round(Q.data, 6).tolist())
print(np.round(K.data, 6).tolist())
print(np.round(V.data, 6).tolist())
```

```text title=readonly
[[2.0, 1.0, 1.0, 0.0], [1.0, 2.0, 1.0, 2.0], [1.0, 1.0, 2.0, 2.0], [2.171116, -0.959996, 1.14067, -1.990442]]
[[2.0, 1.0, 0.0, 1.0], [1.0, 1.0, 2.0, 2.0], [1.0, 2.0, 2.0, 1.0], [2.171116, 1.14067, -1.990442, -0.959996]]
[[2.0, 1.0, 0.0, 1.0], [1.0, 2.0, 2.0, 1.0], [1.0, 1.0, 2.0, 2.0], [2.171116, -0.959996, -1.990442, 1.14067]]
```

`Q`, `K`, `V` đều shape `(4, 4)` — CÙNG shape với `X` (vì `Wq`, `Wk`, `Wv` đều vuông `(4, 4)` trong ví dụ này) — nhưng GIÁ TRỊ khác hẳn nhau, dù cả ba đều chiếu từ ĐÚNG cùng một `X`. Hàng `0` của `Q` là `[2, 1, 1, 0]`, hàng `0` của `K` là `[2, 1, 0, 1]` — khác nhau ở CHÍNH những chiều lẽ ra phải giống nếu `Wq = Wk`. Ba ma trận trọng số khác nhau tạo ra ba "cách nhìn" khác nhau về CÙNG một token.
::::

::::predict{#doan_shape_v_khac commitOnce}
Ví dụ trên dùng `Wq`, `Wk`, `Wv` đều shape `(4, 4)` — vuông, cùng kích thước — nên `Q`, `K`, `V` đều shape `(4, 4)`.

**Trước khi trả lời**, bạn đoán: nếu đổi `Wv` thành shape `(4, 6)` (chiếu `V` sang một không gian `6` chiều thay vì `4`, GIỮ NGUYÊN `Wq`/`Wk` shape `(4, 4)`), phép nhân `X.matmul(Wq)` và `K = X.matmul(Wk)` (không đổi) — công thức attention `Q.matmul(K.transpose())` (bài sau) có còn CHẠY ĐƯỢC không?

:::opt{correct}
Có — `Q.matmul(K.transpose())` chỉ phụ thuộc shape của `Q` và `K` (cả hai đều `(4, 4)`, không đổi), hoàn toàn KHÔNG liên quan gì tới shape của `V`; `matmul` đòi CHIỀU TRONG của hai toán hạng khớp nhau (`nhan-ma-tran-forward`), và `V` không xuất hiện trong phép nhân đó — chỉ tới bước `attn.matmul(V)` (bài `dau-ra-attention`) shape của `V` mới bắt đầu quan trọng
:::

:::opt
Không — ba phép chiếu `Q`, `K`, `V` phải LUÔN cùng shape với nhau, vì cả ba đều chiếu từ CÙNG một `X`
::why
Gần đúng ở việc để ý cả ba đều chiếu từ CHUNG một `X` — quan sát về nguồn gốc chung không sai.

Chỗ lệch: "chiếu từ chung một nguồn" không bắt buộc "cho ra cùng shape". Shape kết quả của `X.matmul(W)` phụ thuộc HOÀN TOÀN vào shape CỦA `W` (số cột của `W` quyết định số cột kết quả, theo đúng luật `matmul` đã học) — ba ma trận `Wq`, `Wk`, `Wv` là BA THAM SỐ ĐỘC LẬP, không có ràng buộc nào bắt chúng phải cùng shape.
::
:::

:::opt
Không — `Q.matmul(K.transpose())` cần biết shape của `V` trước để xác định chiều CUỐI của kết quả attention
::why
Gần đúng ở việc `V` đúng là XUẤT HIỆN ở đâu đó trong toàn bộ pipeline attention — quan sát về sự có mặt của `V` không sai.

Chỗ lệch: `Q.matmul(K.transpose())` là MỘT phép nhân ma trận cụ thể, chỉ liên quan tới HAI toán hạng của chính nó (`Q` và `K.transpose()`) — theo luật `matmul` đã học (`nhan-ma-tran-forward`), shape kết quả CHỈ phụ thuộc shape của hai toán hạng ĐÓ, không phụ thuộc bất kỳ biến nào khác chưa tham gia phép tính. `V` chỉ bắt đầu liên quan ở một phép `matmul` KHÁC, diễn ra SAU đó.
::
:::
::::

::::code{#viet_qkv}
Hoàn thiện hai phép chiếu còn thiếu: `K` và `V`, dùng `Tensor.matmul` đã có sẵn.

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


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])

Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = ___                # X.matmul(Wk)
V = ___                # X.matmul(Wv)

print(np.round(Q.data, 6).tolist())
print(np.round(K.data, 6).tolist())
print(np.round(V.data, 6).tolist())
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


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])

Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
V = X.matmul(Wv)

print(np.round(Q.data, 6).tolist())
print(np.round(K.data, 6).tolist())
print(np.round(V.data, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(Q.data, 6).tolist() == [[2.0, 1.0, 1.0, 0.0], [1.0, 2.0, 1.0, 2.0], [1.0, 1.0, 2.0, 2.0], [2.171116, -0.959996, 1.14067, -1.990442]], f"Q.data sai -- dang ra {np.round(Q.data, 6).tolist()}"
assert np.round(K.data, 6).tolist() == [[2.0, 1.0, 0.0, 1.0], [1.0, 1.0, 2.0, 2.0], [1.0, 2.0, 2.0, 1.0], [2.171116, 1.14067, -1.990442, -0.959996]], f"K.data sai -- dang ra {np.round(K.data, 6).tolist()}"
assert np.round(V.data, 6).tolist() == [[2.0, 1.0, 0.0, 1.0], [1.0, 2.0, 2.0, 1.0], [1.0, 1.0, 2.0, 2.0], [2.171116, -0.959996, -1.990442, 1.14067]], f"V.data sai -- dang ra {np.round(V.data, 6).tolist()}"

# rieng kiem tra BA ket qua KHONG duoc trung nhau (chan cheat gan K=Q hay V=Q)
assert not np.allclose(Q.data, K.data), "Q va K khong duoc trung nhau -- kiem tra da dung dung Wk chua"
assert not np.allclose(Q.data, V.data), "Q va V khong duoc trung nhau -- kiem tra da dung dung Wv chua"
assert not np.allclose(K.data, V.data), "K va V khong duoc trung nhau -- kiem tra da dung dung ma tran trong so rieng cho tung cai chua"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai đều là một lời gọi `matmul` từ `X`, chiếu qua ĐÚNG ma trận trọng số tương ứng. `K` dùng `Wk` — `X.matmul(Wk)`. `V` dùng `Wv` — `X.matmul(Wv)`. Đừng nhầm lẫn dùng lại `Wq` cho cả ba.
- kind: strategy
  body: 'Chỗ đầu (`K`): `X.matmul(Wk)`. Chỗ hai (`V`): `X.matmul(Wv)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `X.matmul(Wk)` và `X.matmul(Wv)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: K phai duoc rap bang X.matmul(Wk) (dung DUNG Wk, khong duoc dung lai Wq hay Wv); V phai duoc rap bang X.matmul(Wv) (dung DUNG Wv)
  requireAst:
  - kind: uses-name, target: Wk, min: 1
  - kind: uses-name, target: Wv, min: 1
  - kind: uses-call, target: matmul, min: 3
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # class Tensor + harness): loi giai dung dat=true. Wk=1 (chi trong blank1
  # "X.matmul(Wk)" -- dinh nghia "Wk = Tensor(...)" la Store, khong dem).
  # Wv=1 (chi trong blank2). matmul=3 (Q=X.matmul(Wq) co san trong starter,
  # cong hai blank con lai -- dinh nghia "def matmul(self, other):" khong
  # tinh vi khong phai Call). Cheat "K = X.matmul(Wq)" (dung nham Wq) lam
  # "Wk" tut xuong 0 -- bi chan RIENG, VA da tu kiem chung bang Python that:
  # K.data khi do se TRUNG HET voi Q.data -- bi bat DOC LAP boi assert not
  # np.allclose(Q.data, K.data). Cheat "V = X.matmul(Wq)" tuong tu lam "Wv"
  # tut xuong 0 va bi bat boi assert not np.allclose(Q.data, V.data). Cheat
  # "K = X.matmul(Wv)" (hoan doi Wk/Wv) lam "Wk" tut xuong 0 -- bi chan
  # RIENG, va da tu kiem chung: K.data se bang gia tri dung cua V.data (hai
  # so lieu hoan doi cho nhau) -- van bi bat vi gia tri K.data KHONG con
  # khop assert K.data dung o tren (K.data phai la gia tri rieng cua no,
  # khong phai gia tri cua V).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[\\[2\\.0, 1\\.0, 1\\.0, 0\\.0\\], \\[1\\.0, 2\\.0, 1\\.0, 2\\.0\\], \\[1\\.0, 1\\.0, 2\\.0, 2\\.0\\], \\[2\\.171116, -0\\.959996, 1\\.14067, -1\\.990442\\]\\]\\n\\[\\[2\\.0, 1\\.0, 0\\.0, 1\\.0\\], \\[1\\.0, 1\\.0, 2\\.0, 2\\.0\\], \\[1\\.0, 2\\.0, 2\\.0, 1\\.0\\], \\[2\\.171116, 1\\.14067, -1\\.990442, -0\\.959996\\]\\]\\n\\[\\[2\\.0, 1\\.0, 0\\.0, 1\\.0\\], \\[1\\.0, 2\\.0, 2\\.0, 1\\.0\\], \\[1\\.0, 1\\.0, 2\\.0, 2\\.0\\], \\[2\\.171116, -0\\.959996, -1\\.990442, 1\\.14067\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Query, Key, Value — ba cách nhìn khác nhau về cùng một token. Bài sau: TÍNH TAY điểm attention giữa hai vị trí cụ thể, đối chiếu với `Tensor.matmul` thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Query` của một token đóng vai "câu hỏi tra cứu", `Key` của MỌI token khác đóng vai "nhãn để so khớp". So khớp một Query với một Key — về mặt số học — chính là một phép TÍNH TƯƠNG TỰ giữa hai vector. Phép toán nào đã học (từ `Value` vô hướng ở T8.2 tới `Tensor` ở q8.3b) đo được "độ giống nhau" giữa hai vector bằng CÁCH NHÂN TỪNG CẶP CHIỀU TƯƠNG ỨNG rồi CỘNG LẠI?
::::

::::checkpoint{mastery=0.8}
::::
