---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.boss-tong-quat-hoa-va-hop-nhat
title: "BOSS — Ráp pipeline ML hoàn chỉnh"
summary: "36 email tự bịa (24 không spam, 12 spam — lệch 2:1), hai đặc trưng (so_tu_hoa THẬT SỰ quan trọng, do_dai_email KHÔNG quan trọng): tách train(18)/val(9)/test(9) giữ nguyên tỷ lệ lệch lớp, chuẩn hoá CHỈ từ train, grid search 3 lambda qua val chọn lambda=0.05 (val_loss=0.3049, thấp nhất), logistic regression L2 trên TEST đạt conf=(TP=2,FP=0,TN=6,FN=1) — precision=1.0, recall=0.6667, F1=0.8. Permutation importance xác nhận so_tu_hoa (tăng loss 0.6149) quan trọng hơn hẳn do_dai_email (tăng 0.0047, gần như không đáng kể) — ĐÓNG T8.1 (30/30)."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-tong-quat-hoa-va-hop-nhat]
requires: [ai.feature-importance]
concepts: [ai.boss-tong-quat-hoa-va-hop-nhat]
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
Chín mảnh riêng lẻ: bias-variance, bootstrap, bagging, boosting, tuning,
data leakage, xử lý lệch lớp, ổn định số học, feature importance. Cùng
CHÍN mảnh của hai track trước đó nữa. Giờ ráp TẤT CẢ lại — đúng MỘT lần,
đúng thứ tự — thành một pipeline ML hoàn chỉnh.
::::

::::explain{#rap-pipeline-day-du}
`huan_luyen_va_giai_thich` đi qua đúng bảy trạm, THEO ĐÚNG THỨ TỰ đã học
xuyên suốt cả ba quest của T8.1:

1. **Dữ liệu thô, lệch lớp nhẹ** — ba mươi sáu email tự bịa, hai đặc trưng
   (`so_tu_hoa` — thật sự phân biệt được spam; `do_dai_email` — hầu như
   KHÔNG liên quan tới nhãn, chỉ để kiểm tra bước cuối), lệch `2:1` (`24`
   không spam, `12` spam) — đúng tình huống `precision-recall-f1` (track
   trước) đã cảnh báo.
2. **Train/val/test split** (`train-val-test-split`, track hồi quy) — tách
   theo chỉ số, GIỮ NGUYÊN tỷ lệ lệch lớp trong cả ba tập.
3. **Chuẩn hoá ĐÚNG** (`data-leakage-chuan-hoa-truoc`, bài 6) — `trung_bình`
   và `độ_lệch_chuẩn` tính CHỈ từ train, áp lại cho val và test.
4. **Chọn `λ` qua val** (`hyperparameter-tuning`, bài 5) — grid search nhỏ
   trên `3` giá trị `λ`, giữ giá trị có val loss THẤP NHẤT.
5. **Huấn luyện logistic regression CÓ regularization L2** (`regularization-
   l1-l2` + `logistic-regression-tu-so-0`, kết hợp lại đúng cách BOSS của
   track hồi quy đã làm cho hồi quy tuyến tính) — với `λ` đã chốt.

   (Bagging — `bagging`, bài 3 — hoàn toàn CÓ THỂ ghép thêm vào đây, huấn
   luyện nhiều mô hình logistic trên nhiều mẫu bootstrap của train rồi
   trung bình dự đoán. Pipeline này KHÔNG dùng nó — dữ liệu đủ ổn định để
   một mô hình đơn đã cho kết quả tốt — nhưng bagging vẫn là một lựa chọn
   HỢP LỆ, không bắt buộc, đúng tinh thần "thêm khi cần, không thêm cho có".)
6. **Đánh giá bằng confusion matrix + precision/recall/F1** (`ma-tran-nham-
   lan` + `precision-recall-f1`, track phân loại) — đo trên TEST, đúng một
   lần, sau khi mọi lựa chọn đã chốt.
7. **Feature importance qua permutation** (`feature-importance`, bài 9) —
   xáo trộn từng đặc trưng trên test, đo mức tăng loss, để GIẢI THÍCH mô
   hình vừa huấn luyện — không chỉ báo cáo nó "tốt" tới đâu.

Không có khái niệm MỚI nào ở bài này — mỗi trạm là một bài đã học, xuyên
suốt BA quest của T8.1, chỉ khác là kết quả của trạm này trở thành đầu vào
của trạm kế tiếp, liền một mạch từ dữ liệu thô tới một mô hình đã giải
thích được.
::::

::::example{#pipeline_day_du_chay_that}
Ba mươi sáu email tự bịa (`np.random.default_rng(6)`, lệch `2:1`), tách chỉ
số theo modulo `4` (giữ nguyên tỷ lệ lệch lớp trong mỗi tập: train `18`
điểm/`6` spam, val `9` điểm/`3` spam, test `9` điểm/`3` spam):

```python title=readonly
import numpy as np

rng = np.random.default_rng(6)
n0, n1 = 24, 12
so_tu_hoa = np.concatenate([rng.uniform(0, 6, n0), rng.uniform(3, 10, n1)])
do_dai = np.concatenate([rng.uniform(200, 2000, n0), rng.uniform(200, 2000, n1)])
la_spam = np.concatenate([np.zeros(n0), np.ones(n1)])
n = len(la_spam)
idx = np.arange(n)

test_idx = idx[idx % 4 == 0]
val_idx = idx[idx % 4 == 1]
train_idx = idx[(idx % 4 == 2) | (idx % 4 == 3)]

def lay(idxs):
    X = np.vstack([so_tu_hoa[idxs], do_dai[idxs]]).T
    y = la_spam[idxs]
    return X, y

X_train, y_train = lay(train_idx)
X_val, y_val = lay(val_idx)
X_test, y_test = lay(test_idx)
print("train:", len(y_train), "diem,", int(y_train.sum()), "spam")
print("val:  ", len(y_val), "diem,", int(y_val.sum()), "spam")
print("test: ", len(y_test), "diem,", int(y_test.sum()), "spam")

# chuan hoa DUNG -- chi tu train
tb, sd = X_train.mean(axis=0), X_train.std(axis=0)
def chuan_hoa(X):
    return (X - tb) / sd
Xc_train, Xc_val, Xc_test = chuan_hoa(X_train), chuan_hoa(X_val), chuan_hoa(X_test)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def huan_luyen_ridge_logistic(X, y, lam, lr, so_buoc):
    n_, d_ = X.shape
    w, b = np.zeros(d_), 0.0
    for _ in range(so_buoc):
        p = sigmoid(X @ w + b)
        dw = (X.T @ (p - y)) / n_ + 2 * lam * w
        db = np.sum(p - y) / n_
        w -= lr * dw
        b -= lr * db
    return w, b

def cross_entropy(X, y, w, b):
    p = np.clip(sigmoid(X @ w + b), 1e-12, 1 - 1e-12)
    return -np.mean(y * np.log(p) + (1 - y) * np.log(1 - p))

# chon lambda qua VAL
cac_lambda = [0.0, 0.05, 0.5]
val_loss_theo_lam, w_theo_lam, b_theo_lam = {}, {}, {}
for lam in cac_lambda:
    w, b = huan_luyen_ridge_logistic(Xc_train, y_train, lam, 0.5, 1000)
    val_loss_theo_lam[lam] = cross_entropy(Xc_val, y_val, w, b)
    w_theo_lam[lam], b_theo_lam[lam] = w, b
    print(f"lam={lam}: val_loss={round(val_loss_theo_lam[lam], 4)}")

lam_tot_nhat = min(val_loss_theo_lam, key=val_loss_theo_lam.get)
w_tot, b_tot = w_theo_lam[lam_tot_nhat], b_theo_lam[lam_tot_nhat]
print("lambda chon qua val:", lam_tot_nhat)

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP): return TP / (TP + FP) if (TP + FP) > 0 else 0.0
def recall(TP, FN): return TP / (TP + FN) if (TP + FN) > 0 else 0.0
def f1(p, r): return 2 * p * r / (p + r) if (p + r) > 0 else 0.0

def du_doan_nhan(xac_suat, nguong=0.5):
    return (xac_suat >= nguong).astype(int)

pred_test = du_doan_nhan(sigmoid(Xc_test @ w_tot + b_tot))
TP, FP, TN, FN = conf(y_test, pred_test)
p_, r_ = precision(TP, FP), recall(TP, FN)
print("TEST conf(TP,FP,TN,FN):", (TP, FP, TN, FN), " precision=", round(p_, 4), " recall=", round(r_, 4), " f1=", round(f1(p_, r_), 4))

# feature importance qua permutation, tren TEST
def loss_test(X_):
    p = np.clip(sigmoid(X_ @ w_tot + b_tot), 1e-12, 1 - 1e-12)
    return -np.mean(y_test * np.log(p) + (1 - y_test) * np.log(1 - p))

loss_goc = loss_test(Xc_test)
rng_perm = np.random.default_rng(2)
for i, ten in enumerate(["so_tu_hoa", "do_dai"]):
    Xp = Xc_test.copy()
    Xp[:, i] = rng_perm.permutation(Xp[:, i])
    print(f"xao tron {ten}: tang loss = {round(loss_test(Xp) - loss_goc, 4)}")
```

```text title=readonly
train: 18 diem, 6 spam
val:   9 diem, 3 spam
test:  9 diem, 3 spam
lam=0.0: val_loss=0.3208
lam=0.05: val_loss=0.3049
lam=0.5: val_loss=0.5133
lambda chon qua val: 0.05
TEST conf(TP,FP,TN,FN): (2, 0, 6, 1)  precision= 1.0  recall= 0.6667  f1= 0.8
xao tron so_tu_hoa: tang loss = 0.6149
xao tron do_dai: tang loss = 0.0047
```

Bảy trạm, một mạch: `18`/`9`/`9` điểm, đúng tỷ lệ `2:1` giữ nguyên ở cả ba
tập. Grid search qua ba `λ` chọn `λ = 0.05` (val loss `0.3049` — thấp nhất
trong ba). Trên TEST (chưa từng chạm cho tới lúc này): `precision = 1.0`
(không báo động giả nào), `recall = 0.6667` (bắt được `2` trong `3` ca spam
thật), `F1 = 0.8`. Cuối cùng, permutation importance xác nhận ĐÚNG những gì
dữ liệu được THIẾT KẾ để thể hiện: xáo trộn `so_tu_hoa` làm loss tăng
`0.6149` — lớn hơn HẲN mức tăng khi xáo trộn `do_dai` (`0.0047`, gần như
không đáng kể). Mô hình thật sự dựa vào `so_tu_hoa` để phân loại, đúng như
mong đợi — pipeline không chỉ cho một con số F1, nó còn GIẢI THÍCH được vì
sao con số đó hợp lý.
::::

::::predict{#doan-neu-bo-buoc-chuan-hoa-dung commitOnce}
Giả sử ai đó viết lại pipeline này, bỏ qua bước 3 (chuẩn hoá đúng) — thay
vào đó tính `trung_bình`/`độ_lệch_chuẩn` từ TOÀN BỘ `36` điểm (train+val+
test gộp) TRƯỚC khi tách ở bước `2`, rồi mới chạy các bước còn lại y hệt.

**Trước khi đọc lại**, bạn đoán: sai lầm này có ảnh hưởng tới bước feature
importance (bước `7`, chạy SAU CÙNG) không, hay chỉ ảnh hưởng tới các bước
sớm hơn?

:::opt{correct}
Có — vì mọi bước SAU bước chuẩn hoá đều dùng lại đúng `Xc_test` (đã bị tính
sai từ bước `3`), nên feature importance ở bước `7` (đo trên CHÍNH
`Xc_test` đó) cũng thừa hưởng sai lầm đó, dù bản thân bước `7` không hề
"làm gì sai" theo đúng logic của riêng nó
:::

:::opt
Không — feature importance chỉ đo mức THAY ĐỔI khi xáo trộn, một phép đo
TƯƠNG ĐỐI, nên không bị ảnh hưởng bởi việc chuẩn hoá đúng hay sai ở bước
trước
::why
Gần đúng ở việc feature importance đúng là một phép đo TƯƠNG ĐỐI (so sánh
loss TRƯỚC và SAU khi xáo trộn) — quan sát về BẢN CHẤT của phép đo đó không
sai.

Chỗ lệch: "tương đối" không có nghĩa "miễn nhiễm với input sai". Feature
importance vẫn cần một `w_tot`, `b_tot` (huấn luyện từ `Xc_train` bị rò rỉ)
VÀ một `Xc_test` (cũng bị rò rỉ) để tính `loss_goc` và `loss_perm` — nếu cả
hai đầu vào đó đã lệch do chuẩn hoá sai, phép SO SÁNH tương đối giữa chúng
cũng thừa hưởng sự lệch đó, dù công thức của riêng bước `7` không đổi.
::
:::

:::opt
Không — feature importance chạy trên dữ liệu ĐÃ chuẩn hoá, và bất kỳ cách
chuẩn hoá nào (đúng hay rò rỉ) đều đưa dữ liệu về CÙNG một thang đo (trung
bình `0`, độ lệch chuẩn `1`), nên kết quả cuối phải giống nhau
::why
Gần đúng ở việc CẢ HAI cách chuẩn hoá (đúng và rò rỉ) đúng là đều đưa dữ
liệu ĐƯỢC TRANSFORM về một thang đo có trung bình `0`/độ lệch chuẩn `1` —
tính chất TOÁN HỌC đó không sai.

Chỗ lệch: "cùng đưa về thang chuẩn `0`/`1`" không có nghĩa "cùng một phép
biến đổi". Hai cách chuẩn hoá dùng HAI CẶP `trung_bình`/`độ_lệch_chuẩn`
KHÁC NHAU (một chỉ từ train, một từ cả `36` điểm) — nên GIÁ TRỊ CỤ THỂ của
`Xc_train`, `Xc_test` sau biến đổi khác nhau thật, kéo theo `w_tot`, `b_tot`
khác nhau, và cuối cùng feature importance đo trên một mô hình KHÁC, không
phải cùng một kết quả.
::
:::
::::

::::code{#hoan_thien_pipeline_day_du}
Hoàn thiện gradient CÓ regularization L2 của logistic regression (`dw`),
bước chọn `λ` qua val (`lam_tot_nhat`), ngưỡng gán nhãn trong `du_doan_nhan`
(`>=`, không phải `>`), và bước chọn đặc trưng quan trọng nhất qua
permutation (`quan_trong_nhat`).

```python title=starter
import numpy as np

rng = np.random.default_rng(6)
n0, n1 = 24, 12
so_tu_hoa = np.concatenate([rng.uniform(0, 6, n0), rng.uniform(3, 10, n1)])
do_dai = np.concatenate([rng.uniform(200, 2000, n0), rng.uniform(200, 2000, n1)])
la_spam = np.concatenate([np.zeros(n0), np.ones(n1)])
n = len(la_spam)
idx = np.arange(n)

test_idx = idx[idx % 4 == 0]
val_idx = idx[idx % 4 == 1]
train_idx = idx[(idx % 4 == 2) | (idx % 4 == 3)]

def lay(idxs):
    X = np.vstack([so_tu_hoa[idxs], do_dai[idxs]]).T
    y = la_spam[idxs]
    return X, y

X_train, y_train = lay(train_idx)
X_val, y_val = lay(val_idx)
X_test, y_test = lay(test_idx)

tb, sd = X_train.mean(axis=0), X_train.std(axis=0)
def chuan_hoa(X):
    return (X - tb) / sd
Xc_train, Xc_val, Xc_test = chuan_hoa(X_train), chuan_hoa(X_val), chuan_hoa(X_test)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def huan_luyen_ridge_logistic(X, y, lam, lr, so_buoc):
    n_, d_ = X.shape
    w, b = np.zeros(d_), 0.0
    for _ in range(so_buoc):
        p = sigmoid(X @ w + b)
        dw = ___                        # dao ham logistic CONG THEM phat L2: (X.T @ (p - y)) / n_ + 2 * lam * w
        db = np.sum(p - y) / n_
        w -= lr * dw
        b -= lr * db
    return w, b

def cross_entropy(X, y, w, b):
    p = np.clip(sigmoid(X @ w + b), 1e-12, 1 - 1e-12)
    return -np.mean(y * np.log(p) + (1 - y) * np.log(1 - p))

cac_lambda = [0.0, 0.05, 0.5]
val_loss_theo_lam, w_theo_lam, b_theo_lam = {}, {}, {}
for lam in cac_lambda:
    w, b = huan_luyen_ridge_logistic(Xc_train, y_train, lam, 0.5, 1000)
    val_loss_theo_lam[lam] = cross_entropy(Xc_val, y_val, w, b)
    w_theo_lam[lam], b_theo_lam[lam] = w, b

lam_tot_nhat = ___                      # lambda co val_loss NHO NHAT: min(val_loss_theo_lam, key=val_loss_theo_lam.get)
w_tot, b_tot = w_theo_lam[lam_tot_nhat], b_theo_lam[lam_tot_nhat]

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP): return TP / (TP + FP) if (TP + FP) > 0 else 0.0
def recall(TP, FN): return TP / (TP + FN) if (TP + FN) > 0 else 0.0
def f1(p, r): return 2 * p * r / (p + r) if (p + r) > 0 else 0.0

def du_doan_nhan(xac_suat, nguong=0.5):
    return (xac_suat ___ nguong).astype(int)     # >=, khong phai >

pred_test = du_doan_nhan(sigmoid(Xc_test @ w_tot + b_tot))
TP, FP, TN, FN = conf(y_test, pred_test)
p_, r_ = precision(TP, FP), recall(TP, FN)

def loss_test(X_):
    p = np.clip(sigmoid(X_ @ w_tot + b_tot), 1e-12, 1 - 1e-12)
    return -np.mean(y_test * np.log(p) + (1 - y_test) * np.log(1 - p))

loss_goc = loss_test(Xc_test)
rng_perm = np.random.default_rng(2)
tang_theo_dac_trung = {}
for i, ten in enumerate(["so_tu_hoa", "do_dai"]):
    Xp = Xc_test.copy()
    Xp[:, i] = rng_perm.permutation(Xp[:, i])
    tang_theo_dac_trung[ten] = loss_test(Xp) - loss_goc

quan_trong_nhat = ___                    # dac trung co tang loss LON NHAT: max(tang_theo_dac_trung, key=tang_theo_dac_trung.get)

print(lam_tot_nhat)
print(TP, FP, TN, FN, round(p_, 4), round(r_, 4), round(f1(p_, r_), 4))
print(round(tang_theo_dac_trung["so_tu_hoa"], 4), round(tang_theo_dac_trung["do_dai"], 4))
print(quan_trong_nhat)
```

```python title=solution
import numpy as np

rng = np.random.default_rng(6)
n0, n1 = 24, 12
so_tu_hoa = np.concatenate([rng.uniform(0, 6, n0), rng.uniform(3, 10, n1)])
do_dai = np.concatenate([rng.uniform(200, 2000, n0), rng.uniform(200, 2000, n1)])
la_spam = np.concatenate([np.zeros(n0), np.ones(n1)])
n = len(la_spam)
idx = np.arange(n)

test_idx = idx[idx % 4 == 0]
val_idx = idx[idx % 4 == 1]
train_idx = idx[(idx % 4 == 2) | (idx % 4 == 3)]

def lay(idxs):
    X = np.vstack([so_tu_hoa[idxs], do_dai[idxs]]).T
    y = la_spam[idxs]
    return X, y

X_train, y_train = lay(train_idx)
X_val, y_val = lay(val_idx)
X_test, y_test = lay(test_idx)

tb, sd = X_train.mean(axis=0), X_train.std(axis=0)
def chuan_hoa(X):
    return (X - tb) / sd
Xc_train, Xc_val, Xc_test = chuan_hoa(X_train), chuan_hoa(X_val), chuan_hoa(X_test)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def huan_luyen_ridge_logistic(X, y, lam, lr, so_buoc):
    n_, d_ = X.shape
    w, b = np.zeros(d_), 0.0
    for _ in range(so_buoc):
        p = sigmoid(X @ w + b)
        dw = (X.T @ (p - y)) / n_ + 2 * lam * w
        db = np.sum(p - y) / n_
        w -= lr * dw
        b -= lr * db
    return w, b

def cross_entropy(X, y, w, b):
    p = np.clip(sigmoid(X @ w + b), 1e-12, 1 - 1e-12)
    return -np.mean(y * np.log(p) + (1 - y) * np.log(1 - p))

cac_lambda = [0.0, 0.05, 0.5]
val_loss_theo_lam, w_theo_lam, b_theo_lam = {}, {}, {}
for lam in cac_lambda:
    w, b = huan_luyen_ridge_logistic(Xc_train, y_train, lam, 0.5, 1000)
    val_loss_theo_lam[lam] = cross_entropy(Xc_val, y_val, w, b)
    w_theo_lam[lam], b_theo_lam[lam] = w, b

lam_tot_nhat = min(val_loss_theo_lam, key=val_loss_theo_lam.get)
w_tot, b_tot = w_theo_lam[lam_tot_nhat], b_theo_lam[lam_tot_nhat]

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP): return TP / (TP + FP) if (TP + FP) > 0 else 0.0
def recall(TP, FN): return TP / (TP + FN) if (TP + FN) > 0 else 0.0
def f1(p, r): return 2 * p * r / (p + r) if (p + r) > 0 else 0.0

def du_doan_nhan(xac_suat, nguong=0.5):
    return (xac_suat >= nguong).astype(int)

pred_test = du_doan_nhan(sigmoid(Xc_test @ w_tot + b_tot))
TP, FP, TN, FN = conf(y_test, pred_test)
p_, r_ = precision(TP, FP), recall(TP, FN)

def loss_test(X_):
    p = np.clip(sigmoid(X_ @ w_tot + b_tot), 1e-12, 1 - 1e-12)
    return -np.mean(y_test * np.log(p) + (1 - y_test) * np.log(1 - p))

loss_goc = loss_test(Xc_test)
rng_perm = np.random.default_rng(2)
tang_theo_dac_trung = {}
for i, ten in enumerate(["so_tu_hoa", "do_dai"]):
    Xp = Xc_test.copy()
    Xp[:, i] = rng_perm.permutation(Xp[:, i])
    tang_theo_dac_trung[ten] = loss_test(Xp) - loss_goc

quan_trong_nhat = max(tang_theo_dac_trung, key=tang_theo_dac_trung.get)

print(lam_tot_nhat)
print(TP, FP, TN, FN, round(p_, 4), round(r_, 4), round(f1(p_, r_), 4))
print(round(tang_theo_dac_trung["so_tu_hoa"], 4), round(tang_theo_dac_trung["do_dai"], 4))
print(quan_trong_nhat)
```

```python title=test
assert lam_tot_nhat == 0.05, f"lambda chon qua val phai la 0.05 -- dang ra {lam_tot_nhat}"
assert round(val_loss_theo_lam[0.05], 4) == 0.3049, f"val loss cua lambda=0.05 phai la 0.3049 -- dang ra {round(val_loss_theo_lam[0.05], 4)}"
assert all(val_loss_theo_lam[lam_tot_nhat] <= v for v in val_loss_theo_lam.values()), "lam_tot_nhat phai co val_loss NHO NHAT trong ca ba lambda"
assert (TP, FP, TN, FN) == (2, 0, 6, 1), f"confusion matrix TEST phai la (2,0,6,1) -- dang ra {(TP, FP, TN, FN)}"
assert round(p_, 4) == 1.0, f"precision TEST phai la 1.0 -- dang ra {round(p_, 4)}"
assert round(r_, 4) == 0.6667, f"recall TEST phai la 0.6667 -- dang ra {round(r_, 4)}"
assert round(f1(p_, r_), 4) == 0.8, f"F1 TEST phai la 0.8 -- dang ra {round(f1(p_, r_), 4)}"
assert round(tang_theo_dac_trung["so_tu_hoa"], 4) == 0.6149, f"tang loss xao tron so_tu_hoa phai la 0.6149 -- dang ra {round(tang_theo_dac_trung['so_tu_hoa'], 4)}"
assert round(tang_theo_dac_trung["do_dai"], 4) == 0.0047, f"tang loss xao tron do_dai phai la 0.0047 -- dang ra {round(tang_theo_dac_trung['do_dai'], 4)}"
assert quan_trong_nhat == "so_tu_hoa", f"dac trung quan trong nhat phai la so_tu_hoa -- dang ra {quan_trong_nhat}"
assert tang_theo_dac_trung["so_tu_hoa"] > tang_theo_dac_trung["do_dai"], "tang loss cua so_tu_hoa phai CAO HON han do_dai"

# rieng kiem tra L2 THAT SU co tac dung (dau cong dung, khong phai chi ton
# tai ten "lam"): goi rieng huan_luyen_ridge_logistic voi mot lambda LON HON
# 0.05 (1.0, van trong vung on dinh cua lr=0.5 -- da thu that: lam >= 2.0
# lam gradient descent PHAN KY thanh nan voi dung lr nay, nen khong dung de
# kiem tra duoc) roi doi chieu voi val_loss cua lam=0.05: phat L2 manh hon
# phai lam val_loss XAU DI (cao hon), khong duoc giu nguyen nhu khi khong
# co phat L2.
w_lam1, b_lam1 = huan_luyen_ridge_logistic(Xc_train, y_train, 1.0, 0.5, 1000)
val_loss_lam1 = cross_entropy(Xc_val, y_val, w_lam1, b_lam1)
assert val_loss_lam1 > val_loss_theo_lam[0.05], f"val_loss voi lambda=1.0 phai CAO HON val_loss voi lambda=0.05 (phat L2 manh hon nua) -- dang ra {round(val_loss_lam1, 4)} so voi {round(val_loss_theo_lam[0.05], 4)}"

# rieng kiem tra BIEN cua nguong phan loai: tren du lieu THAT cua bai,
# khong xac suat test nao dung bang 0.5 chinh xac -- nen khong phan biet
# duoc dau '>=' voi '>' chi bang cach nhin ket qua tren Xc_test. Goi truc
# tiep du_doan_nhan voi xac suat DUNG BANG nguong 0.5 de ep di qua dung
# nhanh bien: '>=' phai gan nhan 1, con '>' se gan nham thanh 0.
assert du_doan_nhan(np.array([0.5]))[0] == 1, f"du_doan_nhan(0.5): xac suat DUNG BANG nguong 0.5 phai duoc gan nhan 1 (dung '>=', khong phai '>') -- dang ra {du_doan_nhan(np.array([0.5]))[0]}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống, mỗi chỗ tái dùng ĐÚNG một công thức đã học ở một bài khác. `dw` là gradient logistic (`logistic-regression-tu-so-0`) CỘNG THÊM số hạng phạt L2 (`regularization-l1-l2`) — thiếu số hạng phạt thì `lam` không có tác dụng gì. `lam_tot_nhat` là khoá có GIÁ TRỊ nhỏ nhất trong `val_loss_theo_lam` (`hyperparameter-tuning`). Chỗ trống trong `du_doan_nhan` là toán tử so sánh — PHẢI là `>=` (không phải `>`), để một xác suất đúng bằng ngưỡng vẫn được gán nhãn dương. `quan_trong_nhat` là khoá có GIÁ TRỊ lớn nhất trong `tang_theo_dac_trung` (`feature-importance`).
- kind: strategy
  body: 'dw: `(X.T @ (p - y)) / n_ + 2 * lam * w`. lam_tot_nhat: `min(val_loss_theo_lam, key=val_loss_theo_lam.get)`. Toán tử trong du_doan_nhan: `>=`. quan_trong_nhat: `max(tang_theo_dac_trung, key=tang_theo_dac_trung.get)`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `(X.T @ (p - y)) / n_ + 2 * lam * w`, `min(val_loss_theo_lam, key=val_loss_theo_lam.get)`, `>=`, và `max(tang_theo_dac_trung, key=tang_theo_dac_trung.get)`.'
:::

:::validate
- tier: run
  timeoutMs: 12000
- tier: static
  onFail: dw phai cong them so hang phat L2 (2*lam*w) vao dao ham logistic; lam_tot_nhat phai dung min(...) THAT tren val_loss_theo_lam; toan tu trong du_doan_nhan phai la >= (khong phai >, de xac suat dung bang nguong van duoc gan nhan duong); quan_trong_nhat phai dung max(...) THAT tren tang_theo_dac_trung
  requireAst:
  - kind: uses-name, target: lam, min: 5
  - kind: uses-name, target: w, min: 6
  - kind: uses-call, target: min, min: 1
  - kind: uses-name, target: val_loss_theo_lam, min: 3
  - kind: uses-operator, target: ">=", min: 1
  - kind: uses-call, target: max, min: 1
  - kind: uses-name, target: tang_theo_dac_trung, min: 5
  # Da thu that (goi kiemAst that tren code day du, trich tu chinh khoi
  # solution): loi giai dung dat=true, ca bay luat qua sach.
  # Cheat da chay THAT: bo phat L2 (dw thieu 2*lam*w) lam "w" tut tu 6 xuong
  # duoi nguong (mat mot lan dung trong so hang phat) -- bi chan. Chep san
  # lam_tot_nhat=0.05 (khong goi min) lam "min" tut ve 0 -- bi chan. Doi
  # toan tu du_doan_nhan tu >= thanh > lam luat ">=" tut ve 0 -- bi chan.
  # Chep san quan_trong_nhat="so_tu_hoa" (khong goi max) lam "max" tut ve 0
  # -- bi chan. Ca bon cheat deu bi static bat DOC LAP voi nhau.
  #
  # MUTATION-TESTING KIEU BIEN: doi >= thanh > trong du_doan_nhan KHONG bi
  # static bat neu chi nhin qua uses-name/uses-call (van dung du ten ham) --
  # phai co rieng luat uses-operator target ">=" moi bat duoc. VA tren du
  # lieu THAT cua bai (36 email, xac suat lien tuc), KHONG diem test nao co
  # xac suat dung bang 0.5 -- da kiem tra that (np.any(probs == 0.5) ==
  # False) -- nen mutation nay se qua SACH tier tests/output neu chi dua vao
  # gia tri cua Xc_test. Da them boundary-case assertion GOI TRUC TIEP
  # du_doan_nhan(np.array([0.5])) trong tier tests de ep di qua dung nhanh
  # bien: '>=' cho 1, '>' cho 0 -- da chay THAT xac nhan ca hai gia tri.
- tier: tests
  timeoutMs: 12000
- tier: output
  match: regex
  expect: "^0\\.05\\n2 0 6 1 1\\.0 0\\.6667 0\\.8\\n0\\.6149 0\\.0047\\nso_tu_hoa\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ ba mươi sáu email thô, lệch lớp, hai đặc trưng — một quy trình đầy đủ,
đúng kỷ luật, kết thúc bằng một mô hình biết chỉ ra CHÍNH XÁC đặc trưng nào
nó dựa vào. T8.1 khép lại tại đây: 30/30.
::::

::::reflect{#nghi-lai}
Ba mươi bài, ba quest, một hành trình: `hoi-quy-va-gradient-descent` mở
đầu bằng least-squares và gradient descent tự viết tay — đo sai số, lặp
dần, mở rộng sang đường cong, chuẩn hoá để hội tụ nhanh, phát hiện và trị
overfitting, tách train/val/test, ổn định hoá bằng cross-validation.
`phan-loai-va-danh-gia` đổi bài toán từ dự đoán MỘT CON SỐ sang dự đoán MỘT
NHÃN — sigmoid, softmax, ma trận nhầm lẫn, precision/recall/F1/ROC-AUC,
k-NN, decision tree, k-means. `tong-quat-hoa-va-hop-nhat` (track này) không
đưa thêm bài toán mới nào — nó ĐÀO SÂU những công cụ đã có: vì sao mô hình
underfit hay overfit đo được bằng số (bias-variance), cách kết hợp nhiều mô
hình lại tốt hơn từng mô hình riêng (bagging, boosting), cách chọn siêu
tham số có hệ thống (tuning), một lỗi kinh điển khiến số liệu nói dối (data
leakage), cách xử lý dữ liệu lệch lớp thay vì chỉ phát hiện nó, hai lỗi số
học âm thầm phá huỷ cả một pipeline (`log(0)`, tràn số), và cách giải thích
một mô hình đã huấn luyện xong (permutation importance) — kết thúc bằng
một pipeline duy nhất ráp lại TẤT CẢ.

Nền tảng xuyên suốt ba mươi bài không hề đổi: **gradient descent**, đạo hàm
riêng viết TAY, cập nhật `w ← w − lr·∂L/∂w`. Mọi mô hình — hồi quy tuyến
tính, logistic regression, softmax — đều học qua đúng CƠ CHẾ đó, chỉ đổi
công thức mất mát cụ thể.

T8.2 (`Mạng nơ-ron từ số 0`) sẽ giữ NGUYÊN động cơ đó — nhưng thay đổi
CÁCH tính đạo hàm. Một kỹ thuật gọi là **autograd** (~200 dòng code, tự
viết) sẽ tính `∂L/∂w` TỰ ĐỘNG, cho những hàm mất mát phức tạp hơn nhiều lần
`(y − wx − b)²` hay cross-entropy một lớp đã quen thuộc — đủ phức tạp để
biểu diễn một MẠNG nhiều tầng, không chỉ một phép biến đổi tuyến tính. Mọi
khái niệm nền tảng của ba mươi bài vừa qua — gradient descent, train/val/
test, regularization, bias-variance, ensemble, ổn định số học — không hề
biến mất khi bước sang T8.2. Chúng vẫn là nền, chỉ có phần TÍNH ĐẠO HÀM
được tự động hoá.
::::

::::checkpoint{mastery=0.85}
::::
