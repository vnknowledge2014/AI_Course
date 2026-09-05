---
id: tri-tue-nhan-tao.neuron-va-mang-nhieu-tang.mang-nhieu-tang-forward-pass
title: "Mạng nhiều tầng: forward pass"
summary: "MLP 2 tầng (1 tầng ẩn 2 neuron + 1 tầng ra) VỚI TRỌNG SỐ CHO SẴN (W1=[[20,20],[20,20]], b1=[-10,-30], w2=[20,-20], b2=-10) giải ĐÚNG cả 4 điểm XOR: out=[0.0,1.0,1.0,0.0] khớp nhãn thật [0,1,1,0]. Tầng ẩn biến 4 điểm XOR gốc ((0,0),(0,1),(1,0),(1,1)) thành 4 điểm mới trong không gian (h1,h2): (0,0), (1,0), (1,0), (1,1) — hai điểm nhãn 1 giờ trùng nhau tại (1,0), tách tuyến tính được khỏi hai điểm nhãn 0."
locale: vi
track: tri-tue-nhan-tao
module: neuron-va-mang-nhieu-tang
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.mang-nhieu-tang-forward-pass]
requires: [ai.vi-sao-can-phi-tuyen]
concepts: [ai.mang-nhieu-tang-forward-pass]
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
Một neuron không giải nổi XOR — thử ba bộ trọng số, cả ba đều sai ít nhất
một điểm. Bài này thêm đúng MỘT tầng nữa, và bốn điểm đó bỗng tách được.
::::

::::explain{#hai-tang-mot-mang}
Một **mạng nơ-ron nhiều tầng** (MLP — multilayer perceptron) ghép nhiều
neuron thành các **tầng** (layer), xếp NỐI TIẾP: đầu ra của tầng này là đầu
vào của tầng kế tiếp.

> **Tầng ẩn** (hidden layer) — một nhóm neuron KHÔNG trực tiếp nhìn thấy
> bởi bên ngoài (không phải đầu vào, không phải đầu ra cuối cùng). Mỗi
> neuron trong tầng ẩn nhận TOÀN BỘ vector đầu vào, tính `z` và `a` RIÊNG
> của nó — tầng ẩn `k` neuron có ma trận trọng số `W` hình `(số_đầu_vào,
> k)`, một cột cho mỗi neuron.
>
> **Tầng ra** (output layer) — nhận đầu ra CỦA TẦNG ẨN (không phải đầu vào
> gốc nữa) làm đầu vào của chính nó, tính `z` và `a` theo đúng công thức
> neuron đã học.

Mạng đơn giản nhất giải được XOR: `2` đầu vào → tầng ẩn `2` neuron → tầng
ra `1` neuron. Với trọng số CHO SẴN dưới đây (chưa học được qua huấn luyện
— các quest SAU của T8.2 (autograd, rồi huấn luyện) mới nói chuyện HỌC ra
trọng số; ở đây chỉ ráp cấu trúc và chạy forward pass):

> Tầng ẩn: `W1 = [[20,20],[20,20]]`, `b1 = [-10,-30]`
>
> Tầng ra: `w2 = [20,-20]`, `b2 = -10`
::::

::::example{#forward_pass_xor}
Chạy forward pass đầy đủ qua cả hai tầng, cho cả bốn điểm XOR cùng lúc:

```python title=readonly
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0,1,1,0])

W1 = np.array([[20.0, 20.0],
               [20.0, 20.0]])
b1 = np.array([-10.0, -30.0])
w2 = np.array([20.0, -20.0])
b2 = -10.0

def tang_an(X, W1, b1):
    return sigmoid(X @ W1 + b1)

def tang_ra(H, w2, b2):
    return sigmoid(H @ w2 + b2)

H = tang_an(X, W1, b1)
out = tang_ra(H, w2, b2)

print("H:", np.round(H, 4).tolist())
print("out:", np.round(out, 4).tolist())
print("y:  ", y.tolist())
```

```text title=readonly
H: [[0.0, 0.0], [1.0, 0.0], [1.0, 0.0], [1.0, 1.0]]
out: [0.0, 1.0, 1.0, 0.0]
y:   [0, 1, 1, 0]
```

`out` khớp CHÍNH XÁC với `y` ở cả bốn điểm — mạng hai tầng này giải đúng
XOR, điều không MỘT neuron tuyến tính nào ở bài trước làm được. Nhìn kỹ
`H` (đầu ra của tầng ẩn, TRƯỚC khi qua tầng ra): với bốn đầu vào gốc
`(0,0)`, `(0,1)`, `(1,0)`, `(1,1)`, tầng ẩn biến chúng thành bốn điểm MỚI:
`(0,0)`, `(1,0)`, `(1,0)`, `(1,1)`.
::::

::::example{#khong_gian_bieu_dien_moi}
So sánh trực tiếp: bốn điểm XOR gốc (không tách tuyến tính được — bài
trước) đối chiếu với bốn điểm SAU tầng ẩn (`H`, cột `h1`/`h2`):

```python title=readonly
import numpy as np

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0,1,1,0])
H = np.array([[0.,0.],[1.,0.],[1.,0.],[1.,1.]])

for i in range(4):
    print(f"x={X[i].tolist()} (nhan {y[i]})  ->  h={H[i].tolist()}")
```

```text title=readonly
x=[0.0, 0.0] (nhan 0)  ->  h=[0.0, 0.0]
x=[0.0, 1.0] (nhan 1)  ->  h=[1.0, 0.0]
x=[1.0, 0.0] (nhan 1)  ->  h=[1.0, 0.0]
x=[1.0, 1.0] (nhan 0)  ->  h=[1.0, 1.0]
```

Trong không gian `(x1, x2)` gốc, hai điểm nhãn `1` nằm trên một đường CHÉO,
không tách được khỏi hai điểm nhãn `0` trên đường chéo kia bằng một đường
thẳng. Trong không gian `(h1, h2)` MỚI (sau tầng ẩn): hai điểm nhãn `1` giờ
trùng NHAU, cùng rơi vào đúng điểm `(1, 0)` — còn hai điểm nhãn `0` nằm ở
`(0,0)` và `(1,1)`. Ba điểm khác nhau, và một đường thẳng NHƯ `h1 - h2 =
0.5` tách gọn `(1,0)` (nhãn `1`, có `h1-h2=1 > 0.5`) khỏi `(0,0)` và
`(1,1)` (nhãn `0`, có `h1-h2` bằng `0` cả hai) — đúng việc tầng RA (một
neuron tuyến tính, y hệt bài trước) làm được. Tầng ẩn không "giải" XOR —
nó biến đổi dữ liệu sang một không gian KHÁC, nơi bài toán trở thành tuyến
tính tách được, để tầng ra (chỉ là một neuron tuyến tính bình thường) giải
nốt phần còn lại.
::::

::::predict{#doan_tang_an_lam_gi commitOnce}
Tầng ra của mạng này CHỈ LÀ một neuron tuyến tính đơn — đúng loại neuron đã
chứng minh KHÔNG giải được XOR ở bài trước.

**Trước khi đọc lại phần trên**, bạn đoán: vì sao thêm tầng ẩn lại giúp
tầng ra (vẫn tuyến tính) giải đúng được XOR?

:::opt{correct}
Vì tầng ẩn biến đổi dữ liệu gốc sang một không gian biểu diễn MỚI, nơi hai
điểm nhãn `1` (vốn nằm trên đường chéo, không tách tuyến tính được) giờ
trùng nhau tại một điểm — bài toán trong không gian mới trở thành tuyến
tính tách được, nên tầng ra (vẫn chỉ là một neuron tuyến tính) giải được
:::

:::opt
Vì tầng ẩn có `2` neuron thay vì `1`, và nhiều neuron hơn thì luôn mạnh hơn
một neuron, bất kể chúng làm gì
::why
Gần đúng ở việc SỐ LƯỢNG neuron đúng là tăng lên (từ `1` neuron tuyến tính
trước đó lên `2` neuron ở tầng ẩn cộng `1` ở tầng ra) — quan sát về số
lượng không sai.

Chỗ lệch: "nhiều neuron hơn" không tự động nghĩa "mạnh hơn" nếu tất cả vẫn
tuyến tính — hai neuron tuyến tính ghép nối tiếp (không hàm kích hoạt phi
tuyến ở giữa) vẫn thu gọn về được đúng MỘT phép biến đổi tuyến tính, không
mạnh hơn một neuron đơn. Sức mạnh thật ở đây tới từ hàm kích hoạt PHI TUYẾN
(`sigmoid`) ở tầng ẩn — nó là thứ khiến phép biến đổi từ `(x1,x2)` sang
`(h1,h2)` không phải một đường thẳng, để không gian mới có hình dạng KHÁC
hẳn không gian gốc.
::
:::

:::opt
Vì tầng ẩn "ghi nhớ" bốn điểm dữ liệu, giống như một bảng tra cứu — với chỉ
bốn điểm, bất kỳ mạng đủ lớn nào cũng làm được điều này
::why
Gần đúng ở việc mạng CÓ nhìn thấy cả bốn điểm train và tính đúng đầu ra cho
từng điểm — không sai ở mức QUAN SÁT.

Chỗ lệch: đây không phải một bảng tra cứu ghi nhớ RIÊNG từng điểm — cùng
một CÔNG THỨC `sigmoid(X@W1+b1)` rồi `sigmoid(H@w2+b2)` áp dụng cho MỌI
điểm, kể cả những điểm CHƯA từng thấy (thử `x=[0.5, 0.5]` sẽ vẫn ra một số
hợp lý, không phải lỗi). Điều làm nó đúng không phải "nhớ bốn đáp án", mà
là phép BIẾN ĐỔI không gian vừa nêu ở trên.
::
:::
::::

::::code{#hoan_thien_forward_pass_mlp}
Hoàn thiện `tang_an` (áp `sigmoid` lên tổng có trọng số của tầng ẩn) và
`tang_ra` (áp `sigmoid` lên tổng có trọng số của tầng ra, nhận `H` — đầu ra
tầng ẩn — làm đầu vào).

```python title=starter
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0,1,1,0])

W1 = np.array([[20.0, 20.0],
               [20.0, 20.0]])
b1 = np.array([-10.0, -30.0])
w2 = np.array([20.0, -20.0])
b2 = -10.0

def tang_an(X, W1, b1):
    return ___          # sigmoid(X @ W1 + b1)

def tang_ra(H, w2, b2):
    return ___          # sigmoid(H @ w2 + b2)

H = tang_an(X, W1, b1)
out = tang_ra(H, w2, b2)

nhan = (out >= 0.5).astype(int)
so_dung = int(np.sum(nhan == y))

print(np.round(H, 4).tolist())
print(np.round(out, 4).tolist())
print(nhan.tolist())
print(so_dung)
```

```python title=solution
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0,1,1,0])

W1 = np.array([[20.0, 20.0],
               [20.0, 20.0]])
b1 = np.array([-10.0, -30.0])
w2 = np.array([20.0, -20.0])
b2 = -10.0

def tang_an(X, W1, b1):
    return sigmoid(X @ W1 + b1)

def tang_ra(H, w2, b2):
    return sigmoid(H @ w2 + b2)

H = tang_an(X, W1, b1)
out = tang_ra(H, w2, b2)

nhan = (out >= 0.5).astype(int)
so_dung = int(np.sum(nhan == y))

print(np.round(H, 4).tolist())
print(np.round(out, 4).tolist())
print(nhan.tolist())
print(so_dung)
```

```python title=test
assert np.round(H, 4).tolist() == [[0.0, 0.0], [1.0, 0.0], [1.0, 0.0], [1.0, 1.0]], f"H sai -- dang ra {np.round(H, 4).tolist()}"
assert np.round(out, 4).tolist() == [0.0, 1.0, 1.0, 0.0], f"out sai -- dang ra {np.round(out, 4).tolist()}"
assert nhan.tolist() == [0, 1, 1, 0], f"nhan sai -- dang ra {nhan.tolist()}"
assert so_dung == 4, f"mang phai du doan DUNG CA BON diem XOR -- dang ra so_dung={so_dung}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cùng một khuôn `sigmoid(tong_co_trong_so)`. `tang_an` nhận `X` (đầu vào GỐC) — trả `sigmoid(X @ W1 + b1)`. `tang_ra` nhận `H` (đầu ra CỦA TẦNG ẨN, không phải `X`) — trả `sigmoid(H @ w2 + b2)`. Nhầm lẫn phổ biến nhất: dùng `X` thay vì `H` trong `tang_ra` — tầng ra phải nhận đầu vào từ tầng NGAY TRƯỚC nó, không phải đầu vào gốc của cả mạng.
- kind: strategy
  body: 'tang_an: `sigmoid(X @ W1 + b1)`. tang_ra: `sigmoid(H @ w2 + b2)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `sigmoid(X @ W1 + b1)` và `sigmoid(H @ w2 + b2)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: tang_an va tang_ra deu phai goi THAT sigmoid tren tong co trong so tinh tu chinh cac tham so W1/b1 va w2/b2 -- khong duoc chep san gia tri H hay out
  requireAst:
  - kind: uses-call, target: sigmoid, min: 2
  - kind: uses-name, target: W1, min: 2
  - kind: uses-name, target: b1, min: 2
  - kind: uses-name, target: w2, min: 2
  - kind: uses-name, target: b2, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca nam luat qua sach (moi ten tham so xuat hien 2 lan --
  # 1 lan trong dinh nghia ham, 1 lan trong loi goi tang_an/tang_ra o duoi).
  # Cheat "tang_an tra ve mang H chep san" lam sigmoid tut ve 1 (chi con
  # tang_ra goi), va W1/b1 ve 0 -- bi chan boi ca ba luat. Cheat "tang_ra tra
  # ve mang out chep san" lam sigmoid tut ve 1, w2/b2 ve 0 -- bi chan boi ca
  # ba luat con lai.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\[\\[0\\.0, 0\\.0\\], \\[1\\.0, 0\\.0\\], \\[1\\.0, 0\\.0\\], \\[1\\.0, 1\\.0\\]\\]\\n\\[0\\.0, 1\\.0, 1\\.0, 0\\.0\\]\\n\\[0, 1, 1, 0\\]\\n4\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`out = [0.0, 1.0, 1.0, 0.0]` — khớp `y` cả bốn điểm. Một neuron không làm
được; hai tầng thì làm được, nhờ tầng ẩn đổi hẳn không gian biểu diễn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Trọng số của mạng này (`W1`, `b1`, `w2`, `b2`) đều được CHO SẴN — không hề
học qua gradient descent. Nếu phải TỰ CHỌN giá trị khởi đầu cho những trọng
số này trước khi huấn luyện (thay vì được cho sẵn như ở đây), việc chọn
TOÀN BỘ chúng bằng `0` có ổn không?
::::

::::checkpoint{mastery=0.8}
::::
