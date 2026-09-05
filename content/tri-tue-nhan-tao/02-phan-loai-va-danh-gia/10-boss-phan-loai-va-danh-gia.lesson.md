---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.boss-phan-loai-va-danh-gia
title: "BOSS — Ráp pipeline phân loại và đánh giá"
summary: "25 email tự bịa (18 không spam, 7 spam — lệch lớp, sinh bằng np.random.default_rng(7)), MỘT đặc trưng số (so_tu_hoa): logistic regression (gradient descent, 1000 bước) đạt confusion matrix (TP=3,FP=0,TN=18,FN=4) — precision=1.0 tuyệt đối nhưng recall chỉ 0.4286, bỏ lọt hơn nửa số spam thật. k-NN (k=1, leave-one-out) đạt (TP=4,FP=2,TN=16,FN=3) — precision=0.6667 thấp hơn nhưng recall=0.5714 cao hơn, F1=0.6154 nhỉnh hơn F1=0.6 của logistic regression. Không mô hình nào thắng tuyệt đối trên MỌI chỉ số — mô hình 'phù hợp hơn' phụ thuộc câu hỏi bài toán thật sự ưu tiên gì."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-phan-loai-va-danh-gia]
requires: [ai.k-means]
concepts: [ai.boss-phan-loai-va-danh-gia]
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
Chín mảnh riêng lẻ: sigmoid, softmax, ma trận nhầm lẫn, precision/recall,
ROC, k-NN, decision tree, k-means. Giờ ráp hai mô hình lại, đo bằng cùng
một bộ thước, và hỏi: cái nào PHÙ HỢP hơn?
::::

::::explain{#rap-pipeline-phan-loai}
`huan_luyen_va_so_sanh` đi qua đúng bốn trạm, THEO ĐÚNG THỨ TỰ đã học:

1. **Dữ liệu lệch lớp** — hai mươi lăm email tự bịa, một đặc trưng số (số
   từ viết hoa), nhưng LỆCH hẳn về nhãn: `18` không spam, chỉ `7` spam
   (`72%`/`28%`) — đúng tình huống `precision-recall-f1` đã cảnh báo:
   accuracy một mình không đủ để đánh giá công bằng.
2. **Huấn luyện HAI mô hình khác bản chất, trên CÙNG dữ liệu** —
   `logistic-regression-tu-so-0` (gradient descent, có pha huấn luyện) VÀ
   `k-nn-phan-loai` (không pha huấn luyện nào, chỉ đo khoảng cách). Với
   k-NN, mỗi điểm được dự đoán bằng **leave-one-out**: tìm hàng xóm gần
   nhất trong số các điểm KHÁC nó (loại chính nó ra trước khi đo khoảng
   cách) — thiếu bước loại trừ này, một điểm sẽ luôn "tìm thấy chính nó"
   ở khoảng cách `0` và tự cho ra dự đoán đúng tuyệt đối, một con số giả
   tạo không nói lên gì về khả năng phân loại thật.
3. **Ma trận nhầm lẫn cho CẢ HAI** (`ma-tran-nham-lan`) — đếm riêng
   `TP`/`FP`/`TN`/`FN` của từng mô hình trên CHÍNH hai mươi lăm điểm đó.
4. **Precision, recall, F1 cho CẢ HAI** (`precision-recall-f1`) — so sánh
   không chỉ MỘT con số accuracy, mà cả ba góc nhìn, rồi hỏi: mô hình nào
   phù hợp hơn cho bài toán CỤ THỂ này?

Không có khái niệm MỚI nào ở bài này — mỗi trạm là một bài đã học, chỉ
khác là kết quả của HAI mô hình được đặt CẠNH NHAU, đo bằng CÙNG một bộ
thước.
::::

::::example{#hai-mo-hinh-mot-bo-du-lieu}
Hai mươi lăm email tự bịa (sinh bằng `np.random.default_rng(7)`, lệch lớp
`18`/`7`), huấn luyện logistic regression VÀ k-NN, đo cả hai bằng cùng bộ
ma trận nhầm lẫn và precision/recall/F1:

```python title=readonly
import numpy as np

rng = np.random.default_rng(7)
n0, n1 = 18, 7
so_tu_hoa = np.concatenate([rng.uniform(0, 6, n0), rng.uniform(3, 10, n1)])
la_spam = np.concatenate([np.zeros(n0), np.ones(n1)])
n = len(so_tu_hoa)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def gradient(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    dw = np.mean(so_tu_hoa * (p - la_spam))
    db = np.mean(p - la_spam)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w -= lr * dw
        b -= lr * db
    return w, b

w, b = huan_luyen(0.1, 1000)
pred_lr = (sigmoid(w * so_tu_hoa + b) >= 0.5).astype(int)

def knn_leave_one_out(k):
    preds = np.zeros(n, dtype=int)
    for i in range(n):
        d = np.abs(so_tu_hoa - so_tu_hoa[i])
        d[i] = np.inf                     # loai chinh no, dung leave-one-out
        idx = np.argsort(d)[:k]
        nb = la_spam[idx]
        preds[i] = 1 if np.sum(nb == 1) > np.sum(nb == 0) else 0
    return preds

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP): return TP / (TP + FP) if (TP + FP) > 0 else 0.0
def recall(TP, FN): return TP / (TP + FN) if (TP + FN) > 0 else 0.0
def f1(p, r): return 2 * p * r / (p + r) if (p + r) > 0 else 0.0

for ten, k in [("k-NN (k=1)", 1), ("k-NN (k=3)", 3), ("k-NN (k=5)", 5)]:
    TP, FP, TN, FN = conf(la_spam, knn_leave_one_out(k))
    p_, r_ = precision(TP, FP), recall(TP, FN)
    print(f"{ten}: conf=({TP},{FP},{TN},{FN})  prec={round(p_,4)} rec={round(r_,4)} f1={round(f1(p_,r_),4)}")

TPl, FPl, TNl, FNl = conf(la_spam, pred_lr)
pl, rl = precision(TPl, FPl), recall(TPl, FNl)
print(f"logistic regression: conf=({TPl},{FPl},{TNl},{FNl})  prec={round(pl,4)} rec={round(rl,4)} f1={round(f1(pl,rl),4)}")
```

```text title=readonly
k-NN (k=1): conf=(4,2,16,3)  prec=0.6667 rec=0.5714 f1=0.6154
k-NN (k=3): conf=(1,3,15,6)  prec=0.25 rec=0.1429 f1=0.1818
k-NN (k=5): conf=(0,1,17,7)  prec=0.0 rec=0.0 f1=0.0
logistic regression: conf=(3,0,18,4)  prec=1.0 rec=0.4286 f1=0.6
```

Nhìn `k-NN (k=5)` trước: `prec=0.0, rec=0.0` — ĐÚNG hệt kiểu thất bại của
mô hình "luôn đoán 0" ở bài `precision-recall-f1`. Với chỉ `7` điểm spam
trong `25` điểm, một hàng xóm-`5` gần một điểm spam gần như LUÔN bị át bởi
đa số không-spam xung quanh — `k` lớn, kết hợp với lớp thiểu số, xoá sạch
khả năng bắt spam. `k=3` đỡ hơn chút nhưng vẫn tệ (`f1=0.1818`). Chỉ
`k=1` — nhạy nhất với điểm gần nhất, đúng bài học của `k-nn-phan-loai` —
mới giữ được khả năng bắt spam hợp lý (`rec=0.5714`).

So `k-NN (k=1)` với `logistic regression`: **logistic regression có
precision TUYỆT ĐỐI** (`1.0` — mọi email nó gắn nhãn spam ĐỀU thật sự là
spam, không báo động giả nào) nhưng **recall thấp hơn** (`0.4286` — bỏ lọt
`4` trong `7` spam thật). **k-NN (k=1) có recall CAO HƠN** (`0.5714` — bắt
được nhiều spam hơn) nhưng **precision thấp hơn** (`0.6667` — một phần ba
số cảnh báo của nó là báo động giả). `F1` của k-NN (`0.6154`) nhỉnh hơn
`F1` của logistic regression (`0.6`) — nhưng KHÔNG mô hình nào thắng trên
MỌI chỉ số. Mô hình "phù hợp hơn" phụ thuộc câu hỏi thật: nếu báo động giả
gây phiền toái lớn (email quan trọng bị chặn nhầm), logistic regression
phù hợp hơn; nếu bỏ lọt spam nguy hiểm hơn báo động giả, k-NN (k=1) phù
hợp hơn.
::::

::::predict{#doan-mo-hinh-precision-cao-hon commitOnce}
Nhìn lại hai mô hình: logistic regression và k-NN (`k=1`).

**Trước khi đọc lại bảng**, bạn đoán: mô hình nào có **precision** CAO
HƠN?

:::opt{correct}
Logistic regression — precision của nó là `1.0` TUYỆT ĐỐI (không báo động
giả nào), cao hơn hẳn `0.6667` của k-NN (`k=1`)
:::

:::opt
k-NN (`k=1`) — vì nó có `F1` cao hơn (`0.6154` so với `0.6`), nên hẳn nó
phải thắng ở MỌI chỉ số thành phần, kể cả precision
::why
Gần đúng ở việc bạn nhớ đúng: `F1` của k-NN (`k=1`) ĐÚNG là cao hơn — con
số đó không sai.

Chỗ lệch: `F1` cao hơn không có nghĩa THẮNG ở MỌI chỉ số thành phần. `F1`
là một cách GỘP precision và recall lại — một mô hình có thể thắng ở `F1`
dù THUA ở CHÍNH XÁC MỘT trong hai chỉ số cấu thành, miễn là thắng đủ nhiều
ở chỉ số còn lại để bù lại. Số liệu thật: k-NN thắng recall (`0.5714` so
với `0.4286`) NHƯNG THUA precision (`0.6667` so với `1.0` của logistic
regression) — đúng kiểu đánh đổi mà `precision-recall-f1` đã cảnh báo.
::
:::

:::opt
Cả hai bằng nhau — vì cả hai đều được huấn luyện và đánh giá trên CÙNG một
bộ hai mươi lăm điểm dữ liệu
::why
Gần đúng ở việc CẢ HAI đúng là dùng chung một bộ dữ liệu — không có mô
hình nào được ưu ái bởi một tập điểm khác biệt.

Chỗ lệch: dùng chung dữ liệu không có nghĩa cho ra CÙNG kết quả — hai mô
hình học theo hai CƠ CHẾ khác hẳn nhau (một khớp `w`, `b` toàn cục qua
gradient descent; một chỉ nhìn hàng xóm gần nhất cục bộ), nên hoàn toàn có
thể (và thực sự) cho ra hai bộ dự đoán khác nhau, dẫn tới hai precision
khác nhau: `1.0` so với `0.6667`.
::
:::
::::

::::code{#hoan_thien_pipeline_phan_loai}
Hoàn thiện gradient của logistic regression (`dw`, công thức đã học ở
`logistic-regression-tu-so-0`), bước bỏ phiếu của k-NN leave-one-out (đã
học ở `k-nn-phan-loai`), rồi xác định mô hình nào có **recall** cao hơn.

```python title=starter
import numpy as np

rng = np.random.default_rng(7)
n0, n1 = 18, 7
so_tu_hoa = np.concatenate([rng.uniform(0, 6, n0), rng.uniform(3, 10, n1)])
la_spam = np.concatenate([np.zeros(n0), np.ones(n1)])
n = len(so_tu_hoa)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def gradient(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    dw = ___                          # (1/n) * tong(so_tu_hoa * (p - la_spam)), dung np.mean
    db = np.mean(p - la_spam)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w -= lr * dw
        b -= lr * db
    return w, b

w, b = huan_luyen(0.1, 1000)
pred_lr = (sigmoid(w * so_tu_hoa + b) >= 0.5).astype(int)

def knn_leave_one_out(k):
    preds = np.zeros(n, dtype=int)
    for i in range(n):
        d = np.abs(so_tu_hoa - so_tu_hoa[i])
        d[i] = np.inf
        idx = np.argsort(d)[:k]
        nb = la_spam[idx]
        preds[i] = ___                # 1 neu so phieu lop 1 > so phieu lop 0, nguoc lai 0
    return preds

pred_knn = knn_leave_one_out(1)

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP): return TP / (TP + FP) if (TP + FP) > 0 else 0.0
def recall(TP, FN): return TP / (TP + FN) if (TP + FN) > 0 else 0.0
def f1(p, r): return 2 * p * r / (p + r) if (p + r) > 0 else 0.0

TPl, FPl, TNl, FNl = conf(la_spam, pred_lr)
TPk, FPk, TNk, FNk = conf(la_spam, pred_knn)
pl, rl = precision(TPl, FPl), recall(TPl, FNl)
pk, rk = precision(TPk, FPk), recall(TPk, FNk)

def chon_mo_hinh(rk, rl):
    return ___                        # "k-NN" neu rk > rl, nguoc lai "logistic regression"

mo_hinh_recall_cao_hon = chon_mo_hinh(rk, rl)

print(TPl, FPl, TNl, FNl)
print(TPk, FPk, TNk, FNk)
print(mo_hinh_recall_cao_hon)
```

```python title=solution
import numpy as np

rng = np.random.default_rng(7)
n0, n1 = 18, 7
so_tu_hoa = np.concatenate([rng.uniform(0, 6, n0), rng.uniform(3, 10, n1)])
la_spam = np.concatenate([np.zeros(n0), np.ones(n1)])
n = len(so_tu_hoa)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def gradient(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    dw = np.mean(so_tu_hoa * (p - la_spam))
    db = np.mean(p - la_spam)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w -= lr * dw
        b -= lr * db
    return w, b

w, b = huan_luyen(0.1, 1000)
pred_lr = (sigmoid(w * so_tu_hoa + b) >= 0.5).astype(int)

def knn_leave_one_out(k):
    preds = np.zeros(n, dtype=int)
    for i in range(n):
        d = np.abs(so_tu_hoa - so_tu_hoa[i])
        d[i] = np.inf
        idx = np.argsort(d)[:k]
        nb = la_spam[idx]
        preds[i] = 1 if np.sum(nb == 1) > np.sum(nb == 0) else 0
    return preds

pred_knn = knn_leave_one_out(1)

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP): return TP / (TP + FP) if (TP + FP) > 0 else 0.0
def recall(TP, FN): return TP / (TP + FN) if (TP + FN) > 0 else 0.0
def f1(p, r): return 2 * p * r / (p + r) if (p + r) > 0 else 0.0

TPl, FPl, TNl, FNl = conf(la_spam, pred_lr)
TPk, FPk, TNk, FNk = conf(la_spam, pred_knn)
pl, rl = precision(TPl, FPl), recall(TPl, FNl)
pk, rk = precision(TPk, FPk), recall(TPk, FNk)

def chon_mo_hinh(rk, rl):
    return "k-NN" if rk > rl else "logistic regression"

mo_hinh_recall_cao_hon = chon_mo_hinh(rk, rl)

print(TPl, FPl, TNl, FNl)
print(TPk, FPk, TNk, FNk)
print(mo_hinh_recall_cao_hon)
```

```python title=test
assert (TPl, FPl, TNl, FNl) == (3, 0, 18, 4), f"confusion matrix cua logistic regression sai -- dang ra {(TPl, FPl, TNl, FNl)}"
assert (TPk, FPk, TNk, FNk) == (4, 2, 16, 3), f"confusion matrix cua k-NN sai -- dang ra {(TPk, FPk, TNk, FNk)}"
assert round(pl, 4) == 1.0, f"precision cua logistic regression phai la 1.0 -- dang ra {round(pl, 4)}"
assert round(rl, 4) == 0.4286, f"recall cua logistic regression phai la 0.4286 -- dang ra {round(rl, 4)}"
assert round(pk, 4) == 0.6667, f"precision cua k-NN phai la 0.6667 -- dang ra {round(pk, 4)}"
assert round(rk, 4) == 0.5714, f"recall cua k-NN phai la 0.5714 -- dang ra {round(rk, 4)}"
assert round(f1(pl, rl), 4) == 0.6, f"F1 cua logistic regression phai la 0.6 -- dang ra {round(f1(pl, rl), 4)}"
assert round(f1(pk, rk), 4) == 0.6154, f"F1 cua k-NN phai la 0.6154 -- dang ra {round(f1(pk, rk), 4)}"
assert mo_hinh_recall_cao_hon == "k-NN", f"mo hinh co recall cao hon phai la k-NN -- dang ra {mo_hinh_recall_cao_hon}"
# rieng kiem tra CHIEU NGUOC: precision cao hon lai la logistic regression --
# dam bao bai tap phan biet DUOC ca hai nhanh, khong chi mot nhanh "thang
# san" duoc hardcode
assert pl > pk, "precision cua logistic regression phai CAO HON precision cua k-NN"
assert rk > rl, "recall cua k-NN phai CAO HON recall cua logistic regression"

# rk (0.5714) khong bao gio dung bang rl (0.4286) tren du lieu THAT cua bai
# -- khong phan biet duoc dau '>' voi '>=', va khong phan biet duoc chuoi
# "logistic regression" co THAT SU bi so sanh hay bi hardcode (ca hai nhanh
# tra ve cung "k-NN"). Goi chon_mo_hinh TRUC TIEP voi cac cap gia tri tong
# hop de ep di qua CA HAI nhanh, KEM CA diem HOA (rk==rl).
assert chon_mo_hinh(0.9, 0.1) == "k-NN", f"chon_mo_hinh(0.9,0.1): rk>rl phai ra k-NN, dang ra {chon_mo_hinh(0.9, 0.1)}"
assert chon_mo_hinh(0.1, 0.9) == "logistic regression", f"chon_mo_hinh(0.1,0.9): rk<rl phai ra logistic regression, dang ra {chon_mo_hinh(0.1, 0.9)}"
assert chon_mo_hinh(0.5, 0.5) == "logistic regression", f"chon_mo_hinh(0.5,0.5): rk BANG rl (hoa) khong duoc goi la k-NN (rk KHONG lon hon rl), dang ra {chon_mo_hinh(0.5, 0.5)}"

# k=1 (dung trong pred_knn) moi lan CHi co DUNG MOT hang xom -- vote0/vote1
# khong bao gio hoa nen khong phan biet duoc dau '>' voi '>=' o buoc bo
# phieu. Goi rieng knn_leave_one_out(2): tai chi so 0, hai hang xom gan
# nhat HOA phieu (1 nhan 0, 1 nhan 1) -- dung '>' phai nghieng ve 0.
assert knn_leave_one_out(2)[0] == 0, f"knn_leave_one_out(2) tai chi so 0: hai hang xom HOA phieu (1-1), phai tra ve 0 (vote1 khong LON HON vote0), dang ra {knn_leave_one_out(2)[0]}"
```

:::hints
- kind: attention
  body: Ba chỗ trống, mỗi chỗ tái dùng ĐÚNG một công thức đã học ở bài khác. `dw` là gradient của cross-entropy (`logistic-regression-tu-so-0`) — `np.mean(so_tu_hoa * (p - la_spam))`. `preds[i]` là bỏ phiếu đa số của k-NN (`k-nn-phan-loai`) — `1 if np.sum(nb == 1) > np.sum(nb == 0) else 0`. `mo_hinh_recall_cao_hon` so sánh `rk` với `rl` — mô hình nào có recall LỚN HƠN.
- kind: strategy
  body: 'dw: `np.mean(so_tu_hoa * (p - la_spam))`. preds[i]: `1 if np.sum(nb == 1) > np.sum(nb == 0) else 0`. mo_hinh_recall_cao_hon: `"k-NN" if rk > rl else "logistic regression"`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `np.mean(so_tu_hoa * (p - la_spam))`, `1 if np.sum(nb == 1) > np.sum(nb == 0) else 0`, và `"k-NN" if rk > rl else "logistic regression"`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: dw phai nhan THAT voi so_tu_hoa (thieu no se giong het cong thuc cua db, khong con phan biet duoc dac trung); preds[i] phai so sanh THAT so phieu cua nb (khong duoc dao nguoc hay chep hang so); mo_hinh_recall_cao_hon phai so sanh THAT rk voi rl, khong duoc chep san chuoi ket qua
  requireAst:
  - kind: uses-name, target: so_tu_hoa, min: 6
  - kind: uses-name, target: nb, min: 2
  - kind: uses-name, target: rk, min: 1
  - kind: uses-name, target: rl, min: 1
  # Da thu that (goi kiemAst that tren DUNG code cua khoi solution, trich
  # thang tu file .lesson.md, khong phai ban rut gon):
  # - loi giai dung: dat=true, ca bon luat qua sach.
  # - dw quen nhan so_tu_hoa (dw = np.mean(p - la_spam), giong het db): so_tu_hoa
  #   tut tu 6 xuong 5 -- duoi nguong 6, bi chan boi static (RIENG kich ban
  #   nay khong bi phan biet boi gia tri cuoi cung neu chi nhin ket qua --
  #   day chinh la loai "dien bua ma van co the qua" can luat static).
  # - dao nguoc bo phieu k-NN (preds[i] = 1 if sum(nb==1) < sum(nb==0) else 0):
  #   KHONG bi static chan (van dung du ten nb/so_tu_hoa), nhung se bi tier
  #   tests chan qua confusion matrix sai han (da kiem that: (3,16,2,4) thay
  #   vi (4,2,16,3)) -- dung phan cong hai tang.
  # - chep hang so mo_hinh_recall_cao_hon = "k-NN" (dung gia tri, khong so
  #   sanh gi): trong THAN khoi solution, rk va rl chi con xuat hien o dong
  #   GAN (khong con o day) -- ca hai tut ve 0 -- bi chan boi static, DU gia
  #   tri chep san co THAT SU dung (day la diem mau chot: khong the chi dua
  #   vao test/output de bat loai "dien bua" nay, vi chuoi ket qua trung hop
  #   dung -- phai dua vao static de bat viec KHONG so sanh gi ca).
  #
  # HAI LO MUTATION-TESTING THAT tim thay Ở cong.sh, ca hai deu khong bi
  # static chan (van dung du ten rk/rl/nb/so_tu_hoa) VA khong tu nhien lo ra
  # qua du lieu that cua bai (rk=0.5714 khong bao gio dung bang rl=0.4286;
  # k=1 moi lan chi co DUNG MOT hang xom nen vote0/vote1 khong bao gio hoa)
  # -- ca hai da sua bang cach BOC lai thanh ham (chon_mo_hinh) roi goi RIENG
  # voi cap gia tri tong hop (0.9,0.1)/(0.1,0.9)/(0.5,0.5) va goi rieng
  # knn_leave_one_out(2) (k chan, THAT SU hoa tai chi so 0) trong tier tests:
  #   1. doi "logistic regression" thanh "k-NN" (ca hai nhanh cung tra ve
  #      "k-NN") -- vẫn qua sạch moi tang chấm CU.
  #   2. doi dau '>' thanh '>=' o CA hai cho (rk>=rl VA vote1>=vote0) -- vẫn
  #      qua sạch moi tang chấm CU.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^3 0 18 4\\n4 2 16 3\\nk-NN\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Precision tuyệt đối đối đầu recall cao hơn — không mô hình nào thắng mọi
mặt. Chọn mô hình nào phụ thuộc câu hỏi bài toán, không phụ thuộc một con
số duy nhất.
::::

::::reflect{#nghi-lai}
Mười bài, một hành trình: từ dự đoán một CON SỐ (hồi quy, track trước)
sang dự đoán một NHÃN (phân loại, track này) — sigmoid thay cho đường
thẳng trần trụi, softmax mở rộng sang nhiều lớp, ma trận nhầm lẫn và
precision/recall/F1/ROC-AUC đo "đúng-sai" theo nhiều góc thay vì một
accuracy duy nhất, k-NN và decision tree cho thấy có nhiều CƠ CHẾ học khác
hẳn gradient descent, và k-means cho thấy có thể học mà không cần nhãn
nào cả.

Nhưng nền tảng KHÔNG hề đổi: logistic regression và softmax vẫn dùng ĐÚNG
gradient descent đã học ở track trước — chỉ đổi công thức mất mát cụ thể
(cross-entropy thay vì MSE). Train/val/test split và cross-validation (đã
học ở track trước) áp dụng y hệt cho bài toán phân loại — chọn ngưỡng, chọn
`k` của k-NN, hay chọn bất kỳ siêu tham số nào khác đều cần đúng kỷ luật
đó, không có ngoại lệ. Regularization (L1/L2) cũng ghép thẳng vào hàm mất
mát cross-entropy y hệt cách nó ghép vào MSE, không cần công thức mới. Từ
hồi quy sang phân loại là một bước đổi BÀI TOÁN, không phải đổi CÔNG CỤ.

Track tiếp theo (`tong-quat-hoa-va-hop-nhat`) sẽ không giới thiệu bài toán
mới nào nữa — mà quay lại đào sâu chính những công cụ đã có: vì sao một mô
hình underfit hay overfit (bias-variance, đo bằng số chứ không chỉ bằng
lời), cách kết hợp NHIỀU mô hình lại cho tốt hơn từng mô hình riêng lẻ
(bagging, boosting), cách chọn siêu tham số một cách có hệ thống (tuning),
một lỗi kinh điển khiến số liệu đánh giá nói dối (data leakage — chuẩn hoá
trước khi tách train/test), và những chỗ số học có thể âm thầm hỏng
(`log(0)`, tràn số trong softmax) nếu không cẩn thận đúng chỗ.
::::

::::checkpoint{mastery=0.85}
::::
