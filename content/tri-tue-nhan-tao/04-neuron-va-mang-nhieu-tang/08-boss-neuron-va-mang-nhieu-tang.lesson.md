---
id: tri-tue-nhan-tao.neuron-va-mang-nhieu-tang.boss-neuron-va-mang-nhieu-tang
title: "BOSS — Ráp Layer và MLP forward pass"
summary: "Ráp hai class tái dùng được — Layer (W, b, hàm kích hoạt) và MLP (danh sách Layer, forward nối tiếp) — rồi chạy CÙNG kiến trúc tầng ẩn (W1=[[20,20],[20,20]], b1=[-10,-30]) cho HAI bài toán: phân loại XOR (tầng ra sigmoid, w=[20,-20], b=-10) ra out=[0.0,1.0,1.0,0.0], đúng cả 4/4 điểm; và hồi quy (tầng ra identity, w=[3,-2], b=1) ra out=[1.0001,3.9998,3.9998,2.0001] — khớp byte-for-byte với giá trị đã tính tay ở các bài 4 và 7. Đóng q8.2a (8/8) — Layer.forward/MLP.forward là interface q8.2b (autograd) tái dùng."
locale: vi
track: tri-tue-nhan-tao
module: neuron-va-mang-nhieu-tang
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-neuron-va-mang-nhieu-tang]
requires: [ai.hoi-quy-vs-phan-loai-mang-no-ron]
concepts: [ai.boss-neuron-va-mang-nhieu-tang]
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
Bảy bài, mỗi bài viết lại `sigmoid(X@W+b)` bằng tay một lần nữa. Giờ ráp nó
thành hai class, dùng một lần cho mọi tầng, mọi mạng.
::::

::::explain{#layer-va-mlp}
Xuyên suốt q8.2a, mọi tầng — ẩn hay ra, `sigmoid` hay `identity` hay
`softmax` — đều tính ĐÚNG một công thức: `Z = X @ W + b`, rồi `A =
kich_hoat(Z)`. Và mọi mạng — dù một tầng hay nhiều tầng — đều là các tầng
đó XẾP NỐI TIẾP, đầu ra tầng này thành đầu vào tầng sau. Hai class gói
đúng hai sự thật đó:

> **`Layer`** — lưu `W`, `b`, và hàm kích hoạt `kich_hoat` của MỘT tầng.
> `forward(X)` tính `Z = X @ W + b` rồi trả `kich_hoat(Z)`. Một `Layer` duy
> nhất dùng được cho CẢ tầng ẩn (`kich_hoat = sigmoid`) LẪN tầng ra (`kich_
> hoat` là `sigmoid`, `softmax`, hay `identity`, tuỳ bài toán — đúng phân
> biệt đã học ở bài `hoi-quy-vs-phan-loai-mang-no-ron`).
>
> **`MLP`** — lưu một DANH SÁCH `Layer`, THEO ĐÚNG THỨ TỰ. `forward(X)` đưa
> `X` qua tầng đầu tiên, lấy kết quả đưa tiếp qua tầng thứ hai, cứ thế cho
> tới tầng cuối — đúng chuỗi đã chạy TAY qua từng bước ở bài
> `mang-nhieu-tang-forward-pass`.

Đây KHÔNG phải khái niệm mới — mọi phép tính bên trong `Layer.forward` và
`MLP.forward` đã chạy tay ở bảy bài trước. Cái mới là ĐÓNG GÓI: một cấu
trúc dữ liệu tái dùng được, không phải chép lại `sigmoid(X@W+b)` mỗi lần
cần thêm một tầng.

Trọng số của mọi `Layer` trong bài này vẫn CHO SẴN — chưa học được qua
huấn luyện. `Layer.forward`/`MLP.forward` chính là **interface** mà
`q8.2b` (autograd) và `q8.2c` (huấn luyện) sẽ TÁI DÙNG nguyên vẹn: autograd
thêm khả năng tính đạo hàm NGƯỢC qua đúng cấu trúc này, huấn luyện thêm
vòng lặp cập nhật `W`, `b` — nhưng `forward` không đổi.
::::

::::example{#mlp_giai_xor_va_hoi_quy}
Cùng MỘT tầng ẩn (`W1`, `b1` — đúng trọng số của bài `mang-nhieu-tang-
forward-pass`), ráp vào HAI `MLP` khác nhau: một cho phân loại XOR (tầng
ra `sigmoid`, trọng số của bài `mang-nhieu-tang-forward-pass`), một cho
hồi quy (tầng ra `identity`, trọng số của bài `hoi-quy-vs-phan-loai-mang-
no-ron`):

```python title=readonly
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def dinh_danh(z):
    return z

class Layer:
    def __init__(self, W, b, kich_hoat):
        self.W = W
        self.b = b
        self.kich_hoat = kich_hoat

    def forward(self, X):
        Z = X @ self.W + self.b
        return self.kich_hoat(Z)

class MLP:
    def __init__(self, cac_tang):
        self.cac_tang = cac_tang

    def forward(self, X):
        A = X
        for tang in self.cac_tang:
            A = tang.forward(A)
        return A

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
W1 = np.array([[20.0, 20.0],[20.0, 20.0]])
b1 = np.array([-10.0, -30.0])

# MLP PHAN LOAI: tang ra sigmoid
w_clf = np.array([[20.0], [-20.0]])
b_clf = np.array([-10.0])
mlp_xor = MLP([Layer(W1, b1, sigmoid), Layer(w_clf, b_clf, sigmoid)])
out_xor = mlp_xor.forward(X).ravel()
print("XOR:", np.round(out_xor, 4).tolist())

# MLP HOI QUY: cung tang an, tang ra identity
w_reg = np.array([[3.0], [-2.0]])
b_reg = np.array([1.0])
mlp_reg = MLP([Layer(W1, b1, sigmoid), Layer(w_reg, b_reg, dinh_danh)])
out_reg = mlp_reg.forward(X).ravel()
print("hoi quy:", np.round(out_reg, 4).tolist())
```

```text title=readonly
XOR: [0.0, 1.0, 1.0, 0.0]
hoi quy: [1.0001, 3.9998, 3.9998, 2.0001]
```

Hai kết quả này khớp BYTE-FOR-BYTE với hai bài đã tính tay trước đó:
`out_xor` khớp đúng `[0.0, 1.0, 1.0, 0.0]` của bài `mang-nhieu-tang-
forward-pass`, `out_reg` khớp đúng `[1.0001, 3.9998, 3.9998, 2.0001]` của
bài `hoi-quy-vs-phan-loai-mang-no-ron`. Cùng công thức, cùng con số — chỉ
khác cách VIẾT: không còn `sigmoid(H @ w + b)` chép tay, mà `Layer(...).
forward(...)` gọi lại được cho bất kỳ tầng nào, bất kỳ mạng nào.
::::

::::predict{#doan_doi_thu_tu_tang commitOnce}
`MLP.forward` đưa `X` qua CÁC tầng trong `self.cac_tang` THEO ĐÚNG THỨ TỰ
của danh sách.

**Trước khi đọc lại đoạn code**, bạn đoán: nếu đảo NGƯỢC thứ tự hai
`Layer` trong `mlp_xor` — đưa `Layer(w_clf, b_clf, sigmoid)` (vốn là tầng
RA) lên TRƯỚC `Layer(W1, b1, sigmoid)` (vốn là tầng ẨN) — mạng còn tính ra
`[0.0, 1.0, 1.0, 0.0]` nữa không?

:::opt{correct}
Không — hai tầng có hình dạng ma trận KHÁC nhau cho những vai trò khác
nhau (`W1` nhận `2` cột đầu vào, `w_clf` chỉ nhận `2` cột nhưng đúng nghĩa
là đầu ra CỦA tầng ẩn) — đảo thứ tự làm `X` (đầu vào gốc, `4×2`) đi qua
tầng vốn được thiết kế để nhận đầu ra tầng ẩn, cho kết quả sai hoàn toàn
(không còn liên quan gì tới XOR)
:::

:::opt
Vẫn ra đúng — cả hai tầng đều chỉ là `sigmoid(X@W+b)`, và phép nhân ma
trận không quan tâm thứ tự thực hiện
::why
Gần đúng ở việc CẢ HAI tầng đúng là dùng chung một CÔNG THỨC (`sigmoid(X@W
+b)`) — quan sát về CẤU TRÚC không sai.

Chỗ lệch: "cùng công thức" không có nghĩa "đổi thứ tự vô hại". `W1` và
`w_clf` là hai MA TRẬN TRỌNG SỐ khác nhau, được HUẤN LUYỆN (ở đây: được
CHỌN thủ công) cho đúng MỘT vị trí trong chuỗi — `W1` biến đổi đầu vào gốc
`(x1,x2)` thành biểu diễn ẩn, `w_clf` biến đổi biểu diễn ẩn ĐÓ thành xác
suất. Đưa `X` gốc qua `w_clf` trước là áp một phép biến đổi cho đúng loại
dữ liệu KHÁC với dữ liệu nó nhận được — kết quả không còn ý nghĩa gì với
bài toán XOR.
::
:::

:::opt
Không xác định được nếu không chạy thử — với những mạng nhỏ như thế này,
thứ tự tầng không có quy luật rõ ràng nào
::why
Gần đúng ở việc kết quả CỤ THỂ (con số chính xác) đúng là cần chạy để biết
— quan sát về việc "cần tính mới ra số" không sai.

Chỗ lệch: câu hỏi không hỏi CON SỐ chính xác, mà hỏi liệu kết quả có còn
ĐÚNG XOR không — và với câu đó có một quy luật chắc chắn, không cần chạy
thử: `MLP.forward` là một chuỗi phép biến đổi PHỤ THUỘC THỨ TỰ (tầng sau
nhận đầu ra CỦA tầng trước làm đầu vào), nên đảo thứ tự hai tầng có vai trò
khác nhau luôn phá vỡ chuỗi tính toán đó, không phụ thuộc mạng lớn hay nhỏ.
::
:::
::::

::::code{#rap_layer_va_mlp}
Hoàn thiện `Layer.forward` (tính `Z`, áp hàm kích hoạt), `MLP.forward`
(đưa `A` qua TỪNG tầng, theo đúng thứ tự), và `gan_nhan` (ngưỡng `0.5`,
dùng `>=`). Chạy cả hai mạng — phân loại XOR và hồi quy — từ CÙNG hai
class này.

```python title=starter
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def dinh_danh(z):
    return z

class Layer:
    def __init__(self, W, b, kich_hoat):
        self.W = W
        self.b = b
        self.kich_hoat = kich_hoat

    def forward(self, X):
        Z = ___                        # X @ self.W + self.b
        return ___                     # self.kich_hoat(Z)

class MLP:
    def __init__(self, cac_tang):
        self.cac_tang = cac_tang

    def forward(self, X):
        A = X
        for tang in self.cac_tang:
            A = ___                    # tang.forward(A)
        return A

def gan_nhan(p, nguong=0.5):
    return (p ___ nguong).astype(int)  # >=, khong phai >

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y_xor = np.array([0,1,1,0])

W1 = np.array([[20.0, 20.0],[20.0, 20.0]])
b1 = np.array([-10.0, -30.0])
w_clf = np.array([[20.0], [-20.0]])
b_clf = np.array([-10.0])

mlp_xor = MLP([Layer(W1, b1, sigmoid), Layer(w_clf, b_clf, sigmoid)])
out_xor = mlp_xor.forward(X).ravel()
nhan_xor = gan_nhan(out_xor)
so_dung_xor = int(np.sum(nhan_xor == y_xor))

w_reg = np.array([[3.0], [-2.0]])
b_reg = np.array([1.0])
mlp_reg = MLP([Layer(W1, b1, sigmoid), Layer(w_reg, b_reg, dinh_danh)])
out_reg = mlp_reg.forward(X).ravel()

print(np.round(out_xor, 4).tolist())
print(nhan_xor.tolist())
print(so_dung_xor)
print(np.round(out_reg, 4).tolist())
```

```python title=solution
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def dinh_danh(z):
    return z

class Layer:
    def __init__(self, W, b, kich_hoat):
        self.W = W
        self.b = b
        self.kich_hoat = kich_hoat

    def forward(self, X):
        Z = X @ self.W + self.b
        return self.kich_hoat(Z)

class MLP:
    def __init__(self, cac_tang):
        self.cac_tang = cac_tang

    def forward(self, X):
        A = X
        for tang in self.cac_tang:
            A = tang.forward(A)
        return A

def gan_nhan(p, nguong=0.5):
    return (p >= nguong).astype(int)

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y_xor = np.array([0,1,1,0])

W1 = np.array([[20.0, 20.0],[20.0, 20.0]])
b1 = np.array([-10.0, -30.0])
w_clf = np.array([[20.0], [-20.0]])
b_clf = np.array([-10.0])

mlp_xor = MLP([Layer(W1, b1, sigmoid), Layer(w_clf, b_clf, sigmoid)])
out_xor = mlp_xor.forward(X).ravel()
nhan_xor = gan_nhan(out_xor)
so_dung_xor = int(np.sum(nhan_xor == y_xor))

w_reg = np.array([[3.0], [-2.0]])
b_reg = np.array([1.0])
mlp_reg = MLP([Layer(W1, b1, sigmoid), Layer(w_reg, b_reg, dinh_danh)])
out_reg = mlp_reg.forward(X).ravel()

print(np.round(out_xor, 4).tolist())
print(nhan_xor.tolist())
print(so_dung_xor)
print(np.round(out_reg, 4).tolist())
```

```python title=test
assert np.round(out_xor, 4).tolist() == [0.0, 1.0, 1.0, 0.0], f"out_xor sai -- dang ra {np.round(out_xor, 4).tolist()}"
assert nhan_xor.tolist() == [0, 1, 1, 0], f"nhan_xor sai -- dang ra {nhan_xor.tolist()}"
assert so_dung_xor == 4, f"MLP phai phan loai DUNG CA BON diem XOR -- dang ra so_dung_xor={so_dung_xor}"
assert np.round(out_reg, 4).tolist() == [1.0001, 3.9998, 3.9998, 2.0001], f"out_reg sai -- dang ra {np.round(out_reg, 4).tolist()}"
assert out_reg.max() > 1.0, "MLP hoi quy phai du doan duoc gia tri VUOT NGOAI (0,1) -- xac nhan tang ra dinh_danh khong nen ket qua"

# rieng kiem tra BIEN cua nguong trong gan_nhan: tren du lieu XOR that (bon
# xac suat cua out_xor), khong xac suat nao dung bang 0.5 chinh xac -- da
# kiem tra that. Goi truc tiep gan_nhan voi xac suat DUNG BANG nguong de ep
# di qua dung nhanh bien: '>=' phai gan nhan 1.
assert int(gan_nhan(np.array([0.5]))[0]) == 1, f"gan_nhan(0.5) phai la 1 (dung '>=', khong phai '>') -- dang ra {int(gan_nhan(np.array([0.5]))[0])}"

# rieng kiem tra Layer/MLP TAI DUNG duoc cho MOT tang don (khong chi hai
# tang) -- MLP voi dung MOT Layer phai chay duoc, xac nhan interface khong
# gia dinh cung so tang.
mlp_mot_tang = MLP([Layer(w_clf, b_clf, sigmoid)])
ra_mot_tang = mlp_mot_tang.forward(np.array([[1.0, 0.0], [0.0, 1.0]])).ravel()
assert ra_mot_tang.shape == (2,), f"MLP mot tang phai tra ve dung 2 gia tri -- dang ra shape {ra_mot_tang.shape}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `Layer.forward`: `Z` là tổng có trọng số `X @ self.W + self.b` (chú ý dùng `self.W`, `self.b` — thuộc tính đã lưu ở `__init__`, không phải tham số rời); trả về `self.kich_hoat(Z)`, GỌI hàm kích hoạt đã lưu, không viết cứng `sigmoid(Z)` (một `Layer` phải dùng được với BẤT KỲ hàm kích hoạt nào, kể cả `dinh_danh`). `MLP.forward`: `A` ở bước sau là kết quả `forward` của TẦNG hiện tại, áp lên `A` của bước TRƯỚC — `tang.forward(A)`. `gan_nhan`: toán tử `>=` (không phải `>`), đúng quy ước đã dùng xuyên suốt track.
- kind: strategy
  body: 'Z: `X @ self.W + self.b`. return: `self.kich_hoat(Z)`. A (trong vòng lặp MLP): `tang.forward(A)`. Toán tử trong gan_nhan: `>=`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `X @ self.W + self.b`, `self.kich_hoat(Z)`, `tang.forward(A)`, và `>=`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: Layer.forward phai tinh Z tu self.W/self.b va goi THAT self.kich_hoat (khong duoc viet cung sigmoid hay chep san mang ket qua); MLP.forward phai goi THAT tang.forward trong vong lap (khong duoc bo qua cac tang giua); gan_nhan phai dung dung toan tu >= (khong phai >)
  requireAst:
  - kind: uses-name, target: self, min: 5
  - kind: uses-call, target: kich_hoat, min: 1
  - kind: uses-call, target: forward, min: 3
  - kind: uses-name, target: tang, min: 1
  - kind: uses-operator, target: ">=", min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca nam luat qua sach (self=5+: hai lan trong Layer.
  # forward (self.W, self.b) cong cac lan khac trong __init__ va thuoc tinh
  # khac; forward=3: tang.forward(A) trong MLP.forward, cong mlp_xor.forward
  # va mlp_reg.forward goi o duoi). Cheat "Layer.forward tra ve mang 0 chep
  # san, bo qua kich_hoat" lam "kich_hoat" ve 0 -- bi chan. Cheat "MLP.
  # forward chi goi tang cuoi, khong lap qua tung tang" lam "tang" ve 0 --
  # bi chan. Cheat "gan_nhan dung > thay vi >=" lam ">=" ve 0 -- bi chan.
  # Ca ba cheat bi bat DOC LAP voi nhau.
  #
  # MUTATION-TESTING KIEU BIEN: gan_nhan dung > thay vi >= CHI lo ra tren du
  # lieu neu co xac suat dung bang 0.5 chinh xac -- da kiem tra that tren
  # out_xor (bon gia tri, khong gia tri nao dung bang 0.5, vi trong so tang
  # an rat doc nen bao hoa gan 0/1). Mutation nay se qua SACH tier tests/
  # output neu chi dua vao out_xor. Da them assert GOI TRUC TIEP gan_nhan(np.
  # array([0.5])) trong tier tests de ep di qua dung nhanh bien, doc lap voi
  # du lieu XOR cu the.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^\\[0\\.0, 1\\.0, 1\\.0, 0\\.0\\]\\n\\[0, 1, 1, 0\\]\\n4\\n\\[1\\.0001, 3\\.9998, 3\\.9998, 2\\.0001\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Layer` và `MLP` — hai class nhỏ, ráp lại đúng bảy bài vừa học. XOR đúng
`4/4`, hồi quy khớp byte-for-byte với tính tay. q8.2a khép lại tại đây.
::::

::::reflect{#nghi-lai}
Tám bài, một track: `neuron-don` nối logistic regression đã quen thành
khái niệm "neuron". `ham-kich-hoat` mở rộng ra bốn lựa chọn cho `σ`, đo
bão hoà bằng số. `vi-sao-can-phi-tuyen` chứng minh một neuron tuyến tính
đơn không giải nổi XOR — thử thật, sai thật. `mang-nhieu-tang-forward-pass`
thêm một tầng ẩn, giải đúng chính bài toán đó, bằng cách đổi không gian
biểu diễn. `khoi-tao-trong-so` đo hai cái bẫy lúc khởi đầu — đối xứng
không vỡ, bão hoà vì khởi tạo quá lớn. `do-sau-va-do-rong` đếm tham số,
nối lại bias-variance của T8.1c. `hoi-quy-vs-phan-loai-mang-no-ron` tách
rõ vai trò tầng ra. Bài này ráp tất cả thành `Layer`/`MLP` — hai class sẽ
KHÔNG đổi tên, KHÔNG đổi interface khi `q8.2b` thêm autograd vào `forward`,
và `q8.2c` thêm vòng lặp huấn luyện cập nhật `W`, `b` của từng `Layer`.

Trọng số của mọi mạng trong track này đều CHO SẴN — chưa một lần học được
qua dữ liệu. Câu hỏi còn để ngỏ, xuyên suốt cả q8.2a: điều gì sẽ TỰ ĐỘNG
tính đúng `∂L/∂W` cho một mạng NHIỀU tầng — nơi lỗi ở tầng cuối phải
"truyền ngược" qua từng tầng để tới được tầng đầu? `q8.2b` trả lời câu hỏi
đó.
::::

::::checkpoint{mastery=0.85}
::::
