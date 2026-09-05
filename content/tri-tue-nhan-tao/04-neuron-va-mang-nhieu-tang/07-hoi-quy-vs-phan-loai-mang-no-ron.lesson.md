---
id: tri-tue-nhan-tao.neuron-va-mang-nhieu-tang.hoi-quy-vs-phan-loai-mang-no-ron
title: "Hồi quy vs phân loại: mạng nơ-ron"
summary: "Cùng MỘT tầng ẩn (H, tái dùng từ bài mang-nhieu-tang-forward-pass) gắn BA loại tầng ra khác nhau: hồi quy (identity, không hàm kích hoạt) cho y_reg_pred=[1.0001, 3.9998, 3.9998, 2.0001] — vượt ngoài khoảng (0,1); phân loại nhị phân (sigmoid) cho p_clf=[0.0,1.0,1.0,0.0]; phân loại 3 lớp (softmax) cho ma trận xác suất có TỔNG mỗi hàng đúng bằng 1.0. Cùng kiến trúc tầng ẩn, khác đúng lớp đọc kết quả ở tầng ra."
locale: vi
track: tri-tue-nhan-tao
module: neuron-va-mang-nhieu-tang
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.hoi-quy-vs-phan-loai-mang-no-ron]
requires: [ai.do-sau-va-do-rong]
concepts: [ai.hoi-quy-vs-phan-loai-mang-no-ron]
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
Track T8.1 có hai bài toán khác nhau: dự đoán MỘT CON SỐ, và dự đoán MỘT
NHÃN. Mạng nơ-ron xử lý cả hai — chỉ khác đúng tầng cuối cùng.
::::

::::explain{#khac-nhau-o-tang-ra}
Bài `mang-nhieu-tang-forward-pass` xây một tầng ẩn biến đổi đầu vào sang
một không gian biểu diễn MỚI (`H`). Tầng ẩn đó dùng CHUNG được cho nhiều
bài toán khác nhau — điểm khác biệt nằm ở TẦNG RA:

> **Hồi quy** (regression — dự đoán một số liên tục, như
> `hoi-quy-tuyen-tinh-tu-so-0` T8.1a) — tầng ra KHÔNG có hàm kích hoạt
> (hay nói cách khác, hàm kích hoạt là **identity**, `f(z) = z`). Đầu ra có
> thể là BẤT KỲ số thực nào — âm, dương, lớn hơn `1`, nhỏ hơn `0` — đúng
> như một giá phòng, một điểm số, một khoảng cách không có giới hạn tự
> nhiên nào.
>
> **Phân loại nhị phân** (như `logistic-regression-tu-so-0` T8.1b) — tầng
> ra dùng `sigmoid`, nén về `(0, 1)`, đọc như xác suất của lớp `1`.
>
> **Phân loại đa lớp** (như `softmax-da-lop` T8.1b) — tầng ra dùng
> `softmax`, biến `k` số thực bất kỳ thành `k` xác suất KHÔNG ÂM, CỘNG LẠI
> đúng bằng `1` — một phân phối xác suất trên `k` lớp.

Nếu gắn `sigmoid` vào tầng ra của một bài toán HỒI QUY, đầu ra sẽ bị nén
cứng về `(0, 1)` — không bao giờ dự đoán được một giá trị như `4.0` hay
`-2.0`, dù bài toán thật cần điều đó. Chọn SAI hàm kích hoạt tầng ra không
phải một lỗi nhỏ — nó giới hạn cứng miền giá trị mà mô hình CÓ THỂ dự đoán,
bất kể huấn luyện tốt tới đâu.
::::

::::example{#ba_dau_ra_cung_mot_tang_an}
Cùng một tầng ẩn `H` (đúng trọng số `W1`, `b1` của bài
`mang-nhieu-tang-forward-pass`), gắn BA tầng ra khác nhau:

```python title=readonly
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def softmax(Z):
    Z = Z - Z.max(axis=1, keepdims=True)
    e = np.exp(Z)
    return e / e.sum(axis=1, keepdims=True)

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
W1 = np.array([[20.0, 20.0],[20.0, 20.0]])
b1 = np.array([-10.0, -30.0])
H = sigmoid(X @ W1 + b1)
print("H:", np.round(H, 4).tolist())

# tang ra HOI QUY -- khong ham kich hoat
w_reg, b_reg = np.array([3.0, -2.0]), 1.0
y_reg_pred = H @ w_reg + b_reg
print("hoi quy (identity):", np.round(y_reg_pred, 4).tolist())

# tang ra PHAN LOAI NHI PHAN -- sigmoid
w_clf, b_clf = np.array([20.0, -20.0]), -10.0
p_clf = sigmoid(H @ w_clf + b_clf)
print("phan loai nhi phan (sigmoid):", np.round(p_clf, 4).tolist())

# tang ra PHAN LOAI DA LOP -- softmax, 3 lop
W_sm = np.array([[2.0, -1.0, 0.0],[-1.0, 2.0, 1.0]])
b_sm = np.array([0.0, 0.0, 0.5])
P_sm = softmax(H @ W_sm + b_sm)
print("phan loai da lop (softmax):", np.round(P_sm, 4).tolist())
print("tong moi hang:", np.round(P_sm.sum(axis=1), 6).tolist())
```

```text title=readonly
H: [[0.0, 0.0], [1.0, 0.0], [1.0, 0.0], [1.0, 1.0]]
hoi quy (identity): [1.0001, 3.9998, 3.9998, 2.0001]
phan loai nhi phan (sigmoid): [0.0, 1.0, 1.0, 0.0]
phan loai da lop (softmax): [[0.2741, 0.2741, 0.4519], [0.7856, 0.0391, 0.1753], [0.7856, 0.0391, 0.1753], [0.2741, 0.2741, 0.4519]]
tong moi hang: [1.0, 1.0, 1.0, 1.0]
```

Ba tầng ra, CÙNG một `H`: hồi quy cho những con số như `3.9998` — VƯỢT NGOÀI
khoảng `(0,1)`, đúng vì tầng ra không hề nén gì cả. Phân loại nhị phân cho
đúng hai giá trị cực `0.0`/`1.0` (vì `H` ở đây gần như nhị phân sẵn, do
trọng số tầng ẩn rất dốc). Phân loại đa lớp cho MA TRẬN `4×3` — mỗi hàng
một điểm, mỗi cột một lớp — và mỗi hàng CỘNG LẠI đúng bằng `1.0`, xác nhận
`softmax` tạo ra một phân phối xác suất hợp lệ trên `3` lớp.
::::

::::predict{#doan_neu_doi_sigmoid_cho_hoi_quy commitOnce}
Giá trị hồi quy vừa tính có một điểm là `3.9998` — một số lớn hơn `1` khá
nhiều.

**Trước khi đọc lại**, bạn đoán: nếu ai đó (nhầm lẫn) gắn `sigmoid` vào
tầng ra HỒI QUY này thay vì để nó là `identity`, giá trị dự đoán cho điểm
đó sẽ là bao nhiêu, và mô hình còn dự đoán ĐÚNG được giá trị thật (gần `4`)
nữa không?

:::opt{correct}
Giá trị sẽ bị nén về gần `1.0` (`sigmoid` của một số dương lớn tiến gần
`1`) — và mô hình KHÔNG BAO GIỜ dự đoán được một giá trị gần `4` nữa, vì
`sigmoid` giới hạn cứng đầu ra trong `(0, 1)`, bất kể huấn luyện thế nào
:::

:::opt
Giá trị vẫn ra gần `4` — `sigmoid` chỉ đổi CÁCH đọc kết quả, không đổi giá
trị số học thật sự được tính ra
::why
Gần đúng ở việc `sigmoid` đúng là một phép biến đổi có thể "diễn giải lại"
— nhưng đây không phải diễn giải, nó THAY ĐỔI con số cụ thể được trả về.

Chỗ lệch: `sigmoid(z) = 1/(1+e^-z)` là một PHÉP TÍNH THẬT, không phải một
"nhãn dán" lên con số đã có sẵn. Đưa `z ≈ 4` (giá trị `w·H+b` trước khi
nén) qua `sigmoid` cho ra một số HOÀN TOÀN KHÁC (`sigmoid(4) ≈ 0.982`), rồi
làm tròn về gần `1.0` với những `z` lớn hơn — không còn giữ được thông tin
"khoảng cách bao xa" mà bài toán hồi quy cần.
::
:::

:::opt
Không đổi gì — hàm kích hoạt tầng ra chỉ ảnh hưởng tới TỐC ĐỘ huấn luyện
(gradient descent hội tụ nhanh hay chậm), không ảnh hưởng tới GIÁ TRỊ cuối
cùng mô hình có thể dự đoán
::why
Gần đúng ở việc hàm kích hoạt (qua đạo hàm của nó) đúng là có ảnh hưởng tới
tốc độ hội tụ — bài `ham-kich-hoat` đã đo hiện tượng bão hoà làm gradient
nhỏ đi.

Chỗ lệch: ảnh hưởng tới TỐC ĐỘ hội tụ không phải ảnh hưởng DUY NHẤT. `sig
moid` còn giới hạn CỨNG miền giá trị đầu ra — `(0,1)`, không hơn không kém
— nên dù huấn luyện bao lâu, mô hình gắn `sigmoid` ở tầng ra không BAO GIỜ
trả về được một số như `4.0`, bất kể tốc độ hội tụ nhanh hay chậm.
::
:::
::::

::::code{#hoan_thien_ba_tang_ra}
Hoàn thiện ba hàm tầng ra: `dau_ra_hoi_quy` (không hàm kích hoạt),
`dau_ra_phan_loai_nhi_phan` (`sigmoid`), và `dau_ra_phan_loai_da_lop`
(`softmax`).

```python title=starter
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def softmax(Z):
    Z = Z - Z.max(axis=1, keepdims=True)
    e = np.exp(Z)
    return e / e.sum(axis=1, keepdims=True)

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])

W1 = np.array([[20.0, 20.0],[20.0, 20.0]])
b1 = np.array([-10.0, -30.0])
H = sigmoid(X @ W1 + b1)

def dau_ra_hoi_quy(H, w, b):
    return ___                        # H @ w + b (KHONG ham kich hoat)

def dau_ra_phan_loai_nhi_phan(H, w, b):
    return ___                        # sigmoid(H @ w + b)

def dau_ra_phan_loai_da_lop(H, W, b):
    return ___                        # softmax(H @ W + b)

w_reg = np.array([3.0, -2.0])
b_reg = 1.0
y_reg_pred = dau_ra_hoi_quy(H, w_reg, b_reg)

w_clf = np.array([20.0, -20.0])
b_clf = -10.0
p_clf = dau_ra_phan_loai_nhi_phan(H, w_clf, b_clf)

W_sm = np.array([[2.0, -1.0, 0.0],[-1.0, 2.0, 1.0]])
b_sm = np.array([0.0, 0.0, 0.5])
P_sm = dau_ra_phan_loai_da_lop(H, W_sm, b_sm)

print(np.round(y_reg_pred, 4).tolist())
print(np.round(p_clf, 4).tolist())
print(np.round(P_sm, 4).tolist())
print(np.round(P_sm.sum(axis=1), 6).tolist())
```

```python title=solution
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def softmax(Z):
    Z = Z - Z.max(axis=1, keepdims=True)
    e = np.exp(Z)
    return e / e.sum(axis=1, keepdims=True)

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])

W1 = np.array([[20.0, 20.0],[20.0, 20.0]])
b1 = np.array([-10.0, -30.0])
H = sigmoid(X @ W1 + b1)

def dau_ra_hoi_quy(H, w, b):
    return H @ w + b

def dau_ra_phan_loai_nhi_phan(H, w, b):
    return sigmoid(H @ w + b)

def dau_ra_phan_loai_da_lop(H, W, b):
    return softmax(H @ W + b)

w_reg = np.array([3.0, -2.0])
b_reg = 1.0
y_reg_pred = dau_ra_hoi_quy(H, w_reg, b_reg)

w_clf = np.array([20.0, -20.0])
b_clf = -10.0
p_clf = dau_ra_phan_loai_nhi_phan(H, w_clf, b_clf)

W_sm = np.array([[2.0, -1.0, 0.0],[-1.0, 2.0, 1.0]])
b_sm = np.array([0.0, 0.0, 0.5])
P_sm = dau_ra_phan_loai_da_lop(H, W_sm, b_sm)

print(np.round(y_reg_pred, 4).tolist())
print(np.round(p_clf, 4).tolist())
print(np.round(P_sm, 4).tolist())
print(np.round(P_sm.sum(axis=1), 6).tolist())
```

```python title=test
assert np.round(y_reg_pred, 4).tolist() == [1.0001, 3.9998, 3.9998, 2.0001], f"y_reg_pred sai -- dang ra {np.round(y_reg_pred, 4).tolist()}"
assert np.round(p_clf, 4).tolist() == [0.0, 1.0, 1.0, 0.0], f"p_clf sai -- dang ra {np.round(p_clf, 4).tolist()}"
assert np.round(P_sm, 4).tolist() == [[0.2741, 0.2741, 0.4519], [0.7856, 0.0391, 0.1753], [0.7856, 0.0391, 0.1753], [0.2741, 0.2741, 0.4519]], f"P_sm sai -- dang ra {np.round(P_sm, 4).tolist()}"
assert np.round(P_sm.sum(axis=1), 6).tolist() == [1.0, 1.0, 1.0, 1.0], "moi hang cua P_sm phai cong lai dung bang 1.0"
assert y_reg_pred.max() > 1.0, "dau ra HOI QUY phai du doan duoc gia tri VUOT NGOAI khoang (0,1) -- xac nhan KHONG bi nen boi ham kich hoat nao"
```

:::hints
- kind: attention
  body: Ba chỗ trống, ba tầng ra đã nêu ở phần giải thích. `dau_ra_hoi_quy`: KHÔNG hàm kích hoạt — chỉ `H @ w + b` trần trụi. `dau_ra_phan_loai_nhi_phan`: bọc thêm `sigmoid(...)` quanh đúng công thức đó. `dau_ra_phan_loai_da_lop`: bọc `softmax(...)` quanh `H @ W + b` (chú ý `W` viết hoa — một MA TRẬN, không phải vector, vì có `3` lớp).
- kind: strategy
  body: 'dau_ra_hoi_quy: `H @ w + b`. dau_ra_phan_loai_nhi_phan: `sigmoid(H @ w + b)`. dau_ra_phan_loai_da_lop: `softmax(H @ W + b)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `H @ w + b`, `sigmoid(H @ w + b)`, và `softmax(H @ W + b)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: ca ba tang ra phai tinh THAT tu H/w/b (khong duoc chep san mang ket qua); dau_ra_phan_loai_nhi_phan phai goi sigmoid; dau_ra_phan_loai_da_lop phai goi softmax
  requireAst:
  - kind: uses-name, target: H, min: 6
  - kind: uses-name, target: w, min: 2
  - kind: uses-name, target: b, min: 3
  - kind: uses-call, target: sigmoid, min: 2
  - kind: uses-call, target: softmax, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca nam luat qua sach (H=6: 3 lan trong than ham, 3 lan
  # tai cac loi goi ham o duoi; sigmoid=2: 1 lan dinh nghia H o dau, 1 lan
  # trong dau_ra_phan_loai_nhi_phan; softmax=1: trong dau_ra_phan_loai_da_
  # lop). Cheat "dau_ra_hoi_quy chep san mang ket qua" lam H/w/b deu tut
  # xuong duoi nguong -- bi chan boi ca ba. Cheat "dau_ra_phan_loai_nhi_phan
  # chep san" lam H/w/b tut VA rieng sigmoid tut ve 1 -- bi chan boi bon
  # luat. Cheat "dau_ra_phan_loai_da_lop chep san" lam H/b tut VA softmax ve
  # 0 -- bi chan boi ba luat.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\[1\\.0001, 3\\.9998, 3\\.9998, 2\\.0001\\]\\n\\[0\\.0, 1\\.0, 1\\.0, 0\\.0\\]\\n\\[\\[0\\.2741, 0\\.2741, 0\\.4519\\], \\[0\\.7856, 0\\.0391, 0\\.1753\\], \\[0\\.7856, 0\\.0391, 0\\.1753\\], \\[0\\.2741, 0\\.2741, 0\\.4519\\]\\]\\n\\[1\\.0, 1\\.0, 1\\.0, 1\\.0\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một `H` — ba tầng ra, ba loại bài toán. Cấu trúc chung: tầng ẩn học
biểu diễn, tầng ra QUYẾT ĐỊNH cách đọc kết quả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảy bài vừa qua đều dùng trọng số CHO SẴN, viết tay từng con số. Bài BOSS
sắp ráp lại tất cả thành hai class tái dùng được — `Layer` và `MLP` — để
không phải viết lại `sigmoid(X@W+b)` mỗi lần cần một tầng mới. Một class
`Layer` cần lưu những gì để `forward` của nó tính đúng một tầng bất kỳ,
dùng được cho CẢ tầng ẩn lẫn tầng ra?
::::

::::checkpoint{mastery=0.8}
::::
