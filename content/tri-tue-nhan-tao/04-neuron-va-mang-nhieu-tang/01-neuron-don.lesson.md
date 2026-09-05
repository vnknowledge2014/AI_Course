---
id: tri-tue-nhan-tao.neuron-va-mang-nhieu-tang.neuron-don
title: "Neuron đơn"
summary: "Một neuron — z = w·x + b rồi a = σ(z) — chính là logistic regression đã học ở T8.1b, đổi tên khung nhìn. Neuron một đầu vào đã huấn luyện trước đó (w≈1.3885, b≈-3.2266) tính lại bằng công thức vector tổng quát z=X@w+b: a≈[0.0382, 0.1373, 0.3895, 0.7189, 0.9762, 0.994, 0.9985, 0.9999] trên tám điểm train cũ. Một neuron ba đầu vào tự bịa (w=[0.5,-1.0,2.0], b=0.3) xác nhận công thức ĐÚNG cho mọi số chiều: a([1,1,0]) = 0.4502."
locale: vi
track: tri-tue-nhan-tao
module: neuron-va-mang-nhieu-tang
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.neuron-don]
requires: [ai.boss-tong-quat-hoa-va-hop-nhat]
concepts: [ai.neuron-don]
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
T8.1 vừa khép lại. Realm mới, track mới — nhưng công thức đầu tiên của nó
lại là công thức bạn đã viết tay hàng chục lần rồi.
::::

::::explain{#tu-logistic-regression-sang-neuron}
`logistic-regression-tu-so-0` (T8.1b) định nghĩa: `z = w·x + b`, rồi
`p = sigmoid(z)`. Mạng nơ-ron gọi đúng cấu trúc đó bằng một cái tên khác —
**neuron** — và đặt tên riêng cho từng mảnh:

> **đầu vào** (input) `x₁, x₂, ..., xₙ` — các đặc trưng, giống hệt cột dữ
> liệu đã dùng suốt T8.1.
>
> **trọng số** (weight) `w₁, w₂, ..., wₙ` — một số cho mỗi đầu vào, đo mức
> đầu vào đó ảnh hưởng tới kết quả bao nhiêu.
>
> **độ lệch** (bias) `b` — một số cộng thêm, không phụ thuộc đầu vào nào.
>
> **tổng có trọng số** (weighted sum, hay net input) `z = Σᵢ wᵢxᵢ + b` —
> đúng biểu thức `w·x + b` đã quen, viết bằng tích vô hướng.
>
> **hàm kích hoạt** (activation function) `σ` — một hàm phi tuyến nén `z`
> về một khoảng cụ thể. `sigmoid` là MỘT lựa chọn cho `σ` (bài sau giới
> thiệu thêm vài lựa chọn khác).
>
> **đầu ra / kích hoạt** (output / activation) `a = σ(z)` — đúng vị trí của
> `p` trong logistic regression.

Một **neuron** là đúng năm mảnh đó ghép lại: nhận `n` số, tính một tổng có
trọng số, nén qua một hàm kích hoạt, trả về một số. Logistic regression BẠN
ĐÃ VIẾT ở T8.1b — không có gì đổi về công thức — chính là MỘT neuron duy
nhất, với `σ = sigmoid`. Cái mới trong track này không phải công thức, mà
là những gì bài `mang-nhieu-tang-forward-pass` sắp làm với nó: ghép NHIỀU
neuron thành một **tầng** (layer), rồi ghép nhiều tầng thành một **mạng**
(network) — nơi đầu ra của tầng này là đầu vào của tầng kế tiếp.

Một điểm khác về kỹ thuật viết code: `w·x + b` cho một điểm dữ liệu đơn lẻ
tổng quát hoá thành `X @ w + b` cho CẢ MỘT MA TRẬN điểm dữ liệu `X` (mỗi
hàng một điểm, mỗi cột một đầu vào) — `numpy` tính tích ma trận-vector cho
toàn bộ `X` trong một lệnh, không cần vòng lặp qua từng hàng.
::::

::::example{#tai-dung-neuron-da-hoc}
Neuron một đầu vào đã huấn luyện ở `logistic-regression-tu-so-0`
(`w ≈ 1.3885`, `b ≈ -3.2266`, huấn luyện trên tám email, đặc trưng số từ
viết hoa) — viết lại bằng công thức vector tổng quát `z = X@w + b`:

```python title=readonly
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def neuron_forward(X, w, b):
    z = X @ w + b
    a = sigmoid(z)
    return z, a

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
X_spam = so_tu_hoa.reshape(-1, 1)
w_spam = np.array([1.3885])
b_spam = -3.2266

z_spam, a_spam = neuron_forward(X_spam, w_spam, b_spam)
print("z:", np.round(z_spam, 4).tolist())
print("a:", np.round(a_spam, 4).tolist())
```

```text title=readonly
z: [-3.2266, -1.8381, -0.4496, 0.9389, 3.7159, 5.1044, 6.4929, 9.2699]
a: [0.0382, 0.1373, 0.3895, 0.7189, 0.9762, 0.994, 0.9985, 0.9999]
```

`X_spam` là một ma trận `8×1` — tám điểm, một đầu vào mỗi điểm (`reshape(-1,
1)` biến mảng phẳng `so_tu_hoa` thành đúng hình dạng cột mà `X @ w` cần).
Kết quả `a` khớp (xấp xỉ, sai khác ở chữ số thứ tư do `w`, `b` đã bị làm
tròn trước khi đưa vào đây) với xác suất đã tính ở T8.1b — vì đó ĐÚNG LÀ
cùng một phép tính, chỉ viết bằng cú pháp ma trận thay vì nhân vô hướng
từng phần tử.
::::

::::example{#neuron-nhieu-dau-vao}
Công thức `z = X@w + b` không hề đổi khi số đầu vào tăng lên — chỉ `w` dài
ra thêm. Một neuron BA đầu vào, tự bịa hoàn toàn (chưa gắn với bài toán cụ
thể nào), `w = [0.5, -1.0, 2.0]`, `b = 0.3`, áp cho bốn điểm dữ liệu:

```python title=readonly
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def neuron_forward(X, w, b):
    z = X @ w + b
    a = sigmoid(z)
    return z, a

w_moi = np.array([0.5, -1.0, 2.0])
b_moi = 0.3
X_moi = np.array([
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [2.0, -1.0, 0.5],
])

z_moi, a_moi = neuron_forward(X_moi, w_moi, b_moi)
print("z:", np.round(z_moi, 4).tolist())
print("a:", np.round(a_moi, 4).tolist())
```

```text title=readonly
z: [0.8, -0.7, 1.8, 3.3]
a: [0.69, 0.3318, 0.8581, 0.9644]
```

Điểm đầu `[1.0, 0.0, 0.0]`: `z = 0.5×1 - 1×0 + 2×0 + 0.3 = 0.8`, `a =
sigmoid(0.8) ≈ 0.69`. Cùng một hàm `neuron_forward`, không sửa một dòng
nào, chạy đúng cho cả neuron một đầu vào (bài trên) lẫn neuron ba đầu vào
(bài này) — vì `X @ w + b` không quan tâm `X` có bao nhiêu cột, miễn `w` có
đúng số phần tử khớp với số cột đó.
::::

::::predict{#doan-dau-ra-neuron-moi commitOnce}
Neuron ba đầu vào ở trên (`w = [0.5, -1.0, 2.0]`, `b = 0.3`) chưa từng tính
cho điểm `x = [1.0, 1.0, 0.0]` — con số này không nằm trong bốn điểm vừa
chạy.

**Trước khi tính**, bạn đoán: đầu ra kích hoạt `a` của neuron này cho điểm
`x = [1.0, 1.0, 0.0]` khoảng bao nhiêu?

:::opt{correct}
Khoảng `0.4502` — `z = 0.5×1 - 1×1 + 2×0 + 0.3 = -0.2`, `a = sigmoid(-0.2)
≈ 0.4502`
:::

:::opt
Khoảng `-0.2` — đó là kết quả của `w·x + b`, và neuron dừng lại ở đó
::why
Gần đúng ở phép tính `z = 0.5×1 - 1×1 + 2×0 + 0.3 = -0.2` — con số đó không
sai, và nó đúng là bước ĐẦU của một neuron.

Chỗ lệch: `z` chỉ là tổng có trọng số, chưa qua bước NÉN. Một neuron LUÔN
có đúng hai bước — tính `z`, rồi `a = σ(z)` — thiếu bước sau thì `-0.2` chỉ
là một số thực trung gian, không phải "đầu ra" mà câu hỏi đang hỏi tới.
::
:::

:::opt
Khoảng `0.5498` — lấy `1` trừ đi giá trị `sigmoid(-0.2)`
::why
Gần đúng ở việc bạn có tính đúng `sigmoid(-0.2) ≈ 0.4502` trước — phép tính
đó không sai.

Chỗ lệch: không có lý do nào để trừ kết quả cho `1` ở đây. Phép trừ đó chỉ
có nghĩa khi hỏi "xác suất của lớp ĐỐI LẬP" trong một bài toán phân loại
nhị phân — nhưng câu hỏi ở đây chỉ hỏi đúng một con số, đầu ra `a` của
neuron, không có lớp đối lập nào cần tính thêm.
::
:::
::::

::::code{#hoan_thien_neuron_forward}
Hoàn thiện `neuron_forward(X, w, b)`: tính tổng có trọng số `z`, rồi đầu ra
kích hoạt `a`. Áp dụng ĐÚNG một hàm này cho cả neuron một đầu vào (`X_spam`,
`8×1`) lẫn neuron ba đầu vào (`X_moi`, `4×3`).

```python title=starter
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def neuron_forward(X, w, b):
    z = ___                    # X @ w + b
    a = ___                    # sigmoid(z)
    return z, a

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
X_spam = so_tu_hoa.reshape(-1, 1)
w_spam = np.array([1.3885])
b_spam = -3.2266
z_spam, a_spam = neuron_forward(X_spam, w_spam, b_spam)

w_moi = np.array([0.5, -1.0, 2.0])
b_moi = 0.3
X_moi = np.array([
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [2.0, -1.0, 0.5],
])
z_moi, a_moi = neuron_forward(X_moi, w_moi, b_moi)

print(np.round(a_spam, 4).tolist())
print(np.round(z_moi, 4).tolist())
print(np.round(a_moi, 4).tolist())
```

```python title=solution
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def neuron_forward(X, w, b):
    z = X @ w + b
    a = sigmoid(z)
    return z, a

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
X_spam = so_tu_hoa.reshape(-1, 1)
w_spam = np.array([1.3885])
b_spam = -3.2266
z_spam, a_spam = neuron_forward(X_spam, w_spam, b_spam)

w_moi = np.array([0.5, -1.0, 2.0])
b_moi = 0.3
X_moi = np.array([
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [2.0, -1.0, 0.5],
])
z_moi, a_moi = neuron_forward(X_moi, w_moi, b_moi)

print(np.round(a_spam, 4).tolist())
print(np.round(z_moi, 4).tolist())
print(np.round(a_moi, 4).tolist())
```

```python title=test
assert np.round(a_spam, 4).tolist() == [0.0382, 0.1373, 0.3895, 0.7189, 0.9762, 0.994, 0.9985, 0.9999], f"a_spam sai -- dang ra {np.round(a_spam, 4).tolist()}"
assert np.round(z_moi, 4).tolist() == [0.8, -0.7, 1.8, 3.3], f"z_moi sai -- dang ra {np.round(z_moi, 4).tolist()}"
assert np.round(a_moi, 4).tolist() == [0.69, 0.3318, 0.8581, 0.9644], f"a_moi sai -- dang ra {np.round(a_moi, 4).tolist()}"
assert a_spam.shape == (8,), f"a_spam phai co 8 phan tu -- dang ra shape {a_spam.shape}"
assert a_moi.shape == (4,), f"a_moi phai co 4 phan tu -- dang ra shape {a_moi.shape}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, đúng hai bước của một neuron. `z` là tổng có trọng số — công thức `X @ w + b` (tích ma trận `X` với vector `w`, cộng `b`), y hệt `w·x + b` đã viết ở T8.1b nhưng cho cả một ma trận điểm một lúc. `a` là đầu ra kích hoạt — gọi `sigmoid(z)` đã có sẵn ở trên, không viết lại công thức của nó.
- kind: strategy
  body: 'z: `X @ w + b`. a: `sigmoid(z)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `X @ w + b` và `sigmoid(z)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: neuron_forward phai tinh z bang PHEP TOAN THAT tren w va b (khong duoc chep san mang xac suat), va a phai goi THAT sigmoid(z) -- thieu loi goi sigmoid la dau hieu ro nhat cua mot ham chep san ket qua
  requireAst:
  - kind: uses-call, target: sigmoid, min: 1
  - kind: uses-name, target: w, min: 1
  - kind: uses-name, target: b, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca ba luat qua sach. Cheat "chep san mang xac suat theo
  # shape cua X, khong goi sigmoid" lam "sigmoid" ve 0 -- bi chan rieng no.
  # Cheat "quen cong b" (z = X @ w) lam "b" ve 0 -- bi chan. Cheat "quen
  # nhan w" (z = b, phat tan gia tri b cho moi hang) lam "w" ve 0 -- bi
  # chan. Ca ba cheat deu bi bat DOC LAP voi nhau.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\[0\\.0382, 0\\.1373, 0\\.3895, 0\\.7189, 0\\.9762, 0\\.994, 0\\.9985, 0\\.9999\\]\\n\\[0\\.8, -0\\.7, 1\\.8, 3\\.3\\]\\n\\[0\\.69, 0\\.3318, 0\\.8581, 0\\.9644\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng đúng một hàm — `z = X@w + b`, `a = sigmoid(z)` — chạy cho neuron một
đầu vào lẫn ba đầu vào. Bài sau: nhiều lựa chọn khác cho `σ`, không chỉ
`sigmoid`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`sigmoid` không phải lựa chọn DUY NHẤT cho hàm kích hoạt `σ` — nó nén mọi
`z` về khoảng `(0, 1)`, hợp cho việc đọc kết quả như một xác suất. Nhưng
không phải mọi neuron đều cần đọc kết quả kiểu đó.

Nếu một neuron không nằm ở tầng cuối (không cần trả về xác suất), mà nằm ở
một tầng TRUNG GIAN — liệu `σ` của nó có cần nén về đúng khoảng `(0, 1)`
không, hay có thể là một hàm khác?
::::

::::checkpoint{mastery=0.8}
::::
