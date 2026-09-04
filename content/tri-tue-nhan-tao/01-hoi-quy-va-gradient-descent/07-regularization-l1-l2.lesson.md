---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.regularization-l1-l2
title: "Regularization: L1 và L2"
summary: "Trên đặc trưng đa thức bậc 5 chuẩn hoá (8 điểm, quan hệ thật là bậc 1): không phạt gì, hệ số dao động dữ dội [-2.32, 69.08, -139.27, 128.84, -45.07]. Ridge (L2, lambda=1) chỉ CO nhỏ chúng lại còn [5.85, 3.49, 1.68, 0.39, -0.53] — cả năm vẫn khác 0. Lasso (L1, lambda=0.5) đẩy bốn trong năm hệ số về ĐÚNG 0.0, chỉ giữ lại một: [11.2, 0, 0, 0, 0] — sparsity thấy rõ bằng số thật, không phải chỉ 'nhỏ dần'."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.regularization]
requires: [ai.overfitting]
concepts: [ai.regularization]
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
Bài trước để lại một mô hình có hệ số dao động dữ dội. Không đổi bậc, không
bớt đặc trưng — chỉ PHẠT các hệ số lớn. Hai cách phạt, hai kết quả khác hẳn.
::::

::::explain{#phat-trong-so-lon}
Bài `overfitting-va-do-phuc-tap` cho thấy: một mô hình có QUÁ NHIỀU tự do so
với dữ liệu sẽ luồn qua từng hạt nhiễu — và dấu hiệu rõ nhất là các hệ số
`w` dao động cực mạnh, đổi dấu liên tục, giá trị tuyệt đối rất lớn. Đó
không phải trùng hợp: để một đường cong LUỒN chính xác qua từng điểm nhiễu,
các hệ số thường phải "kéo qua kéo lại" cực mạnh.

**Regularization** (chính quy hoá) là một cách chống overfitting KHÔNG cần
đụng tới bậc của mô hình: cộng thêm vào hàm mất mát một số hạng PHẠT các
hệ số lớn. Có hai cách phạt phổ biến:

> **Ridge (L2)**: `L = MSE + λ · Σ wⱼ²` — phạt TỔNG BÌNH PHƯƠNG các hệ số
>
> **Lasso (L1)**: `L = MSE + λ · Σ |wⱼ|` — phạt TỔNG TRỊ TUYỆT ĐỐI các hệ số

`λ` (lambda) là một số dương điều khiển mức độ phạt: `λ = 0` là không phạt
gì (mô hình gốc); `λ` càng lớn, hệ số càng bị ép nhỏ lại. Cả hai đều làm
hệ số nhỏ đi khi `λ` tăng — nhưng theo hai CÁCH khác hẳn nhau. Bình phương
(Ridge) phạt NẶNG hệ số lớn nhưng phạt NHẸ hệ số đã gần `0` — nó co MỌI hệ
số lại gần `0` nhưng hiếm khi đẩy hẳn một hệ số nào về ĐÚNG `0`. Trị tuyệt
đối (Lasso) phạt như nhau bất kể hệ số đang lớn hay nhỏ — kết quả là nó có
xu hướng đẩy hẳn một số hệ số về CHÍNH XÁC `0`, loại bỏ hoàn toàn đặc trưng
đó khỏi mô hình. Tính chất này của Lasso gọi là **sparsity** (thưa) — mô
hình cuối cùng chỉ còn dùng MỘT PHẦN các đặc trưng đưa vào, phần còn lại bị
loại hẳn chứ không chỉ bị co nhỏ.

`Σ wⱼ²` không có công thức đạo hàm gãy khúc (đạo hàm là `2wⱼ`, mượt) nên
Ridge có một **công thức đóng**: `w = (XᵀX + λI)⁻¹Xᵀy` (phần bù trừ vào
đường chéo của `XᵀX`) — giải trực tiếp bằng đại số tuyến tính, không cần
lặp. `Σ |wⱼ|` có đạo hàm gãy tại `wⱼ = 0` (giống MAE ở bài `ham-mat-mat-mse-mae`)
nên Lasso cần một thuật toán lặp riêng (không trình bày chi tiết ở bài
intro này) — điều quan trọng cần nhớ là KẾT QUẢ khác nhau giữa hai cách
phạt, không phải cách giải.
::::

::::example{#ridge-vs-lasso-so-that}
Cùng bài toán bậc 5 của bài trước (8 điểm, quan hệ thật là `y = 2 + 5x`,
đặc trưng đã chuẩn hoá) — không phạt gì, rồi Ridge, rồi Lasso:

```python title=readonly
import numpy as np

x_train = np.array([1,2,3,4,5,6,7,8], dtype=float)
noise = np.array([0.3, -0.4, 0.2, -0.5, 0.6, -0.2, 0.4, -0.3])
y_train = 2 + 5*x_train + noise

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

X_tho = dac_trung(x_train, 5)
tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
X = (X_tho - tb) / sd
n, d = X.shape

# khong phat gi: giai binh thuong bang lstsq
A = np.hstack([X, np.ones((n,1))])
he_so_0, *_ = np.linalg.lstsq(A, y_train, rcond=None)
print("khong phat:", np.round(he_so_0[:-1], 2).tolist())

# Ridge (L2): cong thuc dong, khong phat rieng phan hang so
def ridge_fit(X, y, lam):
    Xb = np.hstack([X, np.ones((n, 1))])
    I = np.eye(d + 1); I[-1, -1] = 0.0
    he_so = np.linalg.solve(Xb.T @ Xb + lam * I, Xb.T @ y)
    return he_so[:-1]

print("Ridge lam=1: ", np.round(ridge_fit(X, y_train, 1), 3).tolist())
print("Ridge lam=20:", np.round(ridge_fit(X, y_train, 20), 3).tolist())
```

```text title=readonly
khong phat: [-2.32, 69.08, -139.27, 128.84, -45.07]
Ridge lam=1:  [5.851, 3.488, 1.677, 0.388, -0.527]
Ridge lam=20: [1.849, 1.631, 1.423, 1.253, 1.118]
```

Không phạt: năm hệ số dao động dữ dội, đổi dấu liên tục, có hệ số lên tới
`±139`. Ridge `λ=1`: co lại nhiều, nhưng vẫn CẢ NĂM hệ số khác `0`. Ridge
`λ=20` (phạt mạnh hơn): co lại NHIỀU HƠN NỮA — nhưng vẫn cả năm hệ số khác
`0`, chỉ là nhỏ dần đều. Ridge co MỌI hệ số lại gần nhau, không loại bỏ
hệ số nào.

Lasso cần một thuật toán lặp riêng — không trình bày chi tiết từng bước ở
bài intro này, chỉ chạy nó và đọc KẾT QUẢ — trên CHÍNH bài toán này:

```python title=readonly
import numpy as np

x_train = np.array([1,2,3,4,5,6,7,8], dtype=float)
noise = np.array([0.3, -0.4, 0.2, -0.5, 0.6, -0.2, 0.4, -0.3])
y_train = 2 + 5*x_train + noise

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

X_tho = dac_trung(x_train, 5)
tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
X = (X_tho - tb) / sd
n, d = X.shape

def lasso_fit(X, y, lam, lr, so_buoc):
    w, b = np.zeros(d), 0.0
    for _ in range(so_buoc):
        sai_so = (X @ w + b) - y
        dw = (2.0 / n) * (X.T @ sai_so)
        db = (2.0 / n) * np.sum(sai_so)
        w -= lr * dw
        b -= lr * db
        # buoc rieng cua Lasso: keo tung he so ve 0 mot khoang lr*lam,
        # nhung khong cho no VUOT qua 0 sang phia doi dau (np.sign giu dau)
        w = np.sign(w) * np.maximum(np.abs(w) - lr * lam, 0.0)
    return w

for lam in [0.5, 5]:
    w = lasso_fit(X, y_train, lam, lr=0.05, so_buoc=2000)
    so_khac_0 = int(np.sum(np.abs(w) > 1e-9))
    print(f"Lasso lam={lam}: w={np.round(w, 3).tolist()}  ({so_khac_0} he so khac 0 trong 5)")
```

```text title=readonly
Lasso lam=0.5: w=[11.198, 0.0, 0.0, 0.0, 0.0]  (1 he so khac 0 trong 5)
Lasso lam=5: w=[8.948, 0.0, 0.0, 0.0, 0.0]  (1 he so khac 0 trong 5)
```

Ngay ở mức phạt YẾU NHẤT đã thử (`λ=0.5`), Lasso đẩy BỐN trong NĂM hệ số về
ĐÚNG `0.0` — không phải một số rất nhỏ, mà là số `0` tuyệt đối — chỉ giữ
lại một hệ số duy nhất (ứng với `x¹`, đặc trưng gần đúng nhất với quan hệ
thật `y = 2 + 5x`). Tăng `λ` lên `5`, hệ số còn lại co nhỏ dần (`11.2 →
8.95`) nhưng KHÔNG có thêm hệ số nào khác `0` được "hồi sinh" — mô hình vẫn
thưa, chỉ dùng một đặc trưng. Đây chính là sparsity: Ridge co đều tất cả,
Lasso chọn LỌC, giữ vài đặc trưng và loại hẳn phần còn lại.
::::

::::predict{#doan-so-he-so-khac-0 commitOnce}
Vẫn bài toán bậc 5 ở trên (5 đặc trưng: `x, x², x³, x⁴, x⁵`). Thử Ridge với
`λ = 100` — một mức phạt RẤT mạnh, mạnh hơn nhiều so với `λ = 20` đã thấy.

**Trước khi tính**, bạn đoán: với `λ = 100`, có bao nhiêu hệ số trong số
NĂM hệ số Ridge sẽ bằng ĐÚNG `0`?

:::opt{correct}
Không hệ số nào — cả năm vẫn khác `0` (dù rất nhỏ), vì phạt bình phương của
Ridge chỉ CO hệ số lại gần `0`, không có cơ chế nào đẩy hẳn một hệ số về
ĐÚNG `0`
:::

:::opt
Cả năm hệ số đều về đúng `0` — vì `λ = 100` đủ lớn để "xoá sạch" mọi hệ số,
bất kể dùng Ridge hay Lasso
::why
Gần đúng ở việc bạn nắm đúng HƯỚNG: `λ` càng lớn, mọi hệ số Ridge càng bị
ép co nhỏ lại gần `0` — quan sát đó đúng.

Chỗ lệch: "co gần `0`" không phải "bằng ĐÚNG `0`". Đạo hàm của số hạng phạt
Ridge (`λ·Σwⱼ²`) là `2λwⱼ` — một hàm TUYẾN TÍNH của `wⱼ`, càng gần `0` thì
lực kéo về `0` càng YẾU đi theo tỷ lệ thuận, không bao giờ "húc" hẳn `wⱼ`
qua `0`. Dù `λ = 100` hay `λ = 1 000 000`, hệ số Ridge vẫn chỉ tiệm cận
`0`, không chạm hẳn — khác hẳn cơ chế của Lasso đã thấy trong bài.
::
:::

:::opt
Đúng một hệ số bằng `0`, giống hệt kết quả của Lasso đã thấy ở trên
::why
Gần đúng ở việc bạn nhớ đúng KẾT QUẢ cụ thể của Lasso trong bài (một hệ số
khác `0`, bốn hệ số bằng `0`) — con số đó không sai CHO LASSO.

Chỗ lệch: câu hỏi hỏi về RIDGE, không phải Lasso — và đây chính là điểm
khác biệt cốt lõi giữa hai cách phạt mà bài này muốn nhấn mạnh. Ridge và
Lasso không hội tụ về cùng một dạng kết quả dù `λ` lớn cỡ nào — Ridge co
đều, không tạo số `0` tuyệt đối nào; chỉ Lasso mới có cơ chế đẩy hẳn hệ số
về `0`.
::
:::
::::

::::code{#viet_ridge_fit}
Viết `ridge_fit`: giải phương trình chuẩn có phạt L2 bằng `np.linalg.solve`,
KHÔNG phạt riêng phần hằng số (hàng/cột cuối của ma trận phạt `I` phải là
`0`, đã có sẵn trong khung).

```python title=starter
import numpy as np

x_train = np.array([1,2,3,4,5,6,7,8], dtype=float)
noise = np.array([0.3, -0.4, 0.2, -0.5, 0.6, -0.2, 0.4, -0.3])
y_train = 2 + 5*x_train + noise

BAC = 5
def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

X_tho = dac_trung(x_train, BAC)
tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
X = (X_tho - tb) / sd
n, d = X.shape

def ridge_fit(X, y, lam):
    Xb = np.hstack([X, np.ones((n, 1))])
    I = np.eye(d + 1)
    I[-1, -1] = 0.0
    he_so = np.linalg.solve(___, ___)   # (Xb^T Xb + lam*I) , Xb^T y
    return he_so[:-1], he_so[-1]

w1, b1v = ridge_fit(X, y_train, 1)
w20, b20 = ridge_fit(X, y_train, 20)

print(np.round(w1, 3).tolist())
print(np.round(w20, 3).tolist())
```

```python title=solution
import numpy as np

x_train = np.array([1,2,3,4,5,6,7,8], dtype=float)
noise = np.array([0.3, -0.4, 0.2, -0.5, 0.6, -0.2, 0.4, -0.3])
y_train = 2 + 5*x_train + noise

BAC = 5
def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

X_tho = dac_trung(x_train, BAC)
tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
X = (X_tho - tb) / sd
n, d = X.shape

def ridge_fit(X, y, lam):
    Xb = np.hstack([X, np.ones((n, 1))])
    I = np.eye(d + 1)
    I[-1, -1] = 0.0
    he_so = np.linalg.solve(Xb.T @ Xb + lam * I, Xb.T @ y)
    return he_so[:-1], he_so[-1]

w1, b1v = ridge_fit(X, y_train, 1)
w20, b20 = ridge_fit(X, y_train, 20)

print(np.round(w1, 3).tolist())
print(np.round(w20, 3).tolist())
```

```python title=test
assert np.round(w1, 3).tolist() == [5.851, 3.488, 1.677, 0.388, -0.527], f"w voi lam=1 sai -- dang ra {np.round(w1, 3).tolist()}"
assert np.round(w20, 3).tolist() == [1.849, 1.631, 1.423, 1.253, 1.118], f"w voi lam=20 sai -- dang ra {np.round(w20, 3).tolist()}"
assert np.sum(np.abs(w1)) > np.sum(np.abs(w20)), "phat MANH hon (lam=20) phai cho tong tri tuyet doi he so NHO hon phat yeu (lam=1)"
assert np.all(np.abs(w20) > 1e-6), "Ridge khong duoc dua he so nao ve dung 0 -- day la diem khac Lasso"
```

:::hints
- kind: attention
  body: Hai chỗ trống là hai đối số của `np.linalg.solve(A, b)` — giải hệ `A·he_so = b`. Vế `A` là ma trận hệ số CÓ CỘNG THÊM phạt (`Xb.T @ Xb + lam * I`); vế `b` là vế phải CHƯA có phạt (`Xb.T @ y`) — phạt chỉ tác động lên phía hệ số, không tác động lên phía dữ liệu quan sát.
- kind: strategy
  body: 'Chỗ trống 1 (ma trận A): `Xb.T @ Xb + lam * I`. Chỗ trống 2 (vế phải b): `Xb.T @ y`.'
- kind: one-line
  body: 'Hai chỗ trống: `Xb.T @ Xb + lam * I` và `Xb.T @ y`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: ve A phai la Xb.T @ Xb CONG THEM lam*I (thieu lam*I la giai khong phat gi), ve b phai la Xb.T @ y -- ca hai deu phai tinh tu Xb/I/lam/y that su, khong duoc chep hang so
  requireAst:
  - kind: uses-name, target: Xb, min: 3
  - kind: uses-name, target: I, min: 2
  - kind: uses-name, target: lam, min: 1
  - kind: uses-name, target: y, min: 1
  # Da thu that (ast.parse): bo lam*I (chi giai Xb.T@Xb , Xb.T@y -- khong
  # phat gi) cho [Xb=3, I=1, lam=0, y=1] -- I va lam tut duoi nguong (can
  # 2 va 1... lam=0 duoi 1, I=1 duoi 2), bi chan. Loi giai that cho dung
  # [3,2,1,1], qua sach.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[5\\.851, 3\\.488, 1\\.677, 0\\.388, -0\\.527\\]\\n\\[1\\.849, 1\\.631, 1\\.423, 1\\.253, 1\\.118\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ridge co đều, giữ mọi đặc trưng. Lasso chọn lọc, đẩy hẳn phần thừa về 0.
Cùng mục tiêu — trị hệ số dao động dữ dội — hai con đường khác hẳn nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`λ = 1` và `λ = 20` cho hai kết quả Ridge rất khác nhau. Nhưng bài này chưa
hề trả lời: `λ` NÀO là lựa chọn tốt nhất cho một bài toán cụ thể? Thử vài
giá trị rồi NHÌN vào hệ số đẹp hay xấu bằng mắt không phải một quy trình
đáng tin cậy — cần một cách ĐO khách quan.

Muốn đo, cần dữ liệu chưa từng dùng để khớp mô hình — đúng ý tưởng "tập
test" của bài `overfitting-va-do-phuc-tap`. Nhưng dùng CHÍNH tập test để
chọn `λ` có ổn không? Bài sau chỉ ra vì sao KHÔNG, và cần một tập dữ liệu
thứ ba.
::::

::::checkpoint{mastery=0.8}
::::
