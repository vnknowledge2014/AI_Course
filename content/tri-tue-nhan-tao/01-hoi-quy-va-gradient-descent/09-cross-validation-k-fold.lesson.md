---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.cross-validation-k-fold
title: "Cross-validation: k-fold"
summary: "20 điểm (y=2+5x+nhiễu), mô hình bậc 2 chuẩn hoá: năm lần chia train/val ĐƠN LẺ (80/20 ngẫu nhiên, 5 seed khác nhau) cho độ lệch chuẩn giữa các lần std=0.2053. Năm lần chạy 5-fold CV đầy đủ (mỗi lần dùng seed xáo trộn khác nhau rồi lấy TRUNG BÌNH 5 fold) cho độ lệch chuẩn giữa các CV-score chỉ còn std=0.0464 — thấp hơn khoảng 4.4 lần: lấy trung bình qua nhiều fold cho một ước lượng ổn định hơn hẳn một lần chia ngẫu nhiên."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.cross-validation]
requires: [ai.train-val-test-split]
concepts: [ai.cross-validation]
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
Bài trước để lại một nỗi lo: một lần chia val may rủi có thể đổi cả kết
quả. Nỗi lo đó đúng tới mức nào — đo được bằng số, không cần đoán.
::::

::::explain{#trung-binh-nhieu-lan-chia}
`train-val-test-split` tách dữ liệu MỘT lần: một phần cố định làm val, phần
còn lại làm train. Nhưng phép chia đó phụ thuộc vào chỗ RANH GIỚI rơi đúng
đâu — với cùng một bộ dữ liệu, chia theo cách khác có thể cho ra một tập
val "may mắn" (khớp tốt) hay "xui" (khớp tệ) khác hẳn, dù mô hình và dữ
liệu gốc không đổi gì. Điểm số validation vì vậy có thể dao động nhiều chỉ
vì MỘT lần chia ngẫu nhiên, không phản ánh đúng chất lượng thật của mô
hình.

**k-fold cross-validation** (kiểm định chéo k-phần) giải quyết việc đó bằng
cách không chỉ chia MỘT lần: chia dữ liệu thành `k` phần bằng nhau (gọi là
**fold**), rồi lặp `k` lần — mỗi lần giữ lại ĐÚNG MỘT fold làm val, `k − 1`
fold còn lại làm train, khớp mô hình, đo điểm số trên fold val đó. Sau `k`
lần, LẤY TRUNG BÌNH của `k` điểm số — đó là **CV score**, điểm số cuối cùng
đại diện cho mô hình.

Vì mọi điểm dữ liệu đều được làm val ĐÚNG MỘT LẦN (không điểm nào bị bỏ
sót, không điểm nào bị dùng hai lần làm val), CV score không phụ thuộc vào
việc "may mắn rơi đúng phần nào" theo cách mà một lần chia đơn lẻ gặp phải
— nó tổng hợp thông tin từ TOÀN BỘ dữ liệu, qua `k` góc nhìn khác nhau. Kỳ
vọng: CV score ổn định hơn — ít dao động hơn — so với điểm số của một lần
chia val đơn lẻ. Bài này đo trực tiếp mức độ ổn định đó bằng số thật.
::::

::::example{#do-do-on-dinh-bang-so}
Hai mươi điểm (`x = 1` tới `20`, quan hệ thật `y = 2 + 5x` cộng nhiễu), mô
hình đa thức bậc 2 chuẩn hoá. So sánh: năm lần chia val ĐƠN LẺ (mỗi lần một
seed ngẫu nhiên khác nhau, 80% train / 20% val) đối chiếu với năm lần chạy
5-fold CV ĐẦY ĐỦ (mỗi lần một cách xáo trộn khác nhau trước khi chia 5
fold):

```python title=readonly
import numpy as np

x = np.arange(1, 21, dtype=float)
noise = np.array([0.6,-0.4,0.9,-0.7,0.3,0.5,-0.8,0.4,-0.3,0.7,
                   -0.5,0.6,-0.6,0.2,-0.2,0.8,-0.9,0.5,-0.4,0.3])
y = 2 + 5*x + noise

def dac_trung(x, bac=2):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_and_mse(train_idx, val_idx):
    xt, yt = x[train_idx], y[train_idx]
    xv, yv = x[val_idx], y[val_idx]
    X_tho = dac_trung(xt)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(xt),1))])
    he_so, *_ = np.linalg.lstsq(A, yt, rcond=None)
    w, b = he_so[:-1], he_so[-1]
    Xv = (dac_trung(xv) - tb) / sd
    return np.mean((yv - (Xv @ w + b)) ** 2)

idx_all = np.arange(20)

def single_split(seed):
    rng = np.random.default_rng(seed)
    perm = rng.permutation(idx_all)
    return fit_and_mse(perm[4:], perm[:4])

def k_fold_cv(seed, K=5):
    rng = np.random.default_rng(seed)
    folds = np.array_split(rng.permutation(idx_all), K)
    diem = [fit_and_mse(np.concatenate([folds[j] for j in range(K) if j != i]), folds[i]) for i in range(K)]
    return np.mean(diem)

single_scores = np.array([single_split(seed) for seed in range(5)])
cv_scores = np.array([k_fold_cv(seed + 100) for seed in range(5)])

print("5 lan chia don le:", np.round(single_scores, 4).tolist())
print("  -> std giua 5 lan:", round(single_scores.std(), 4))
print("5 lan 5-fold CV:", np.round(cv_scores, 4).tolist())
print("  -> std giua 5 lan CV:", round(cv_scores.std(), 4))
```

```text title=readonly
5 lan chia don le: [0.4484, 0.7662, 0.3992, 0.765, 0.2544]
  -> std giua 5 lan: 0.2053
5 lan 5-fold CV: [0.4686, 0.3658, 0.3694, 0.3583, 0.447]
  -> std giua 5 lan CV: 0.0464
```

Năm lần chia val đơn lẻ cho ra điểm số dao động MẠNH — từ `0.2544` tới
`0.7662`, độ lệch chuẩn `0.2053`. Năm lần chạy 5-fold CV (mỗi lần đã LẤY
TRUNG BÌNH của 5 fold bên trong nó) cho ra điểm số ổn định hơn NHIỀU — chỉ
từ `0.3583` tới `0.4686`, độ lệch chuẩn `0.0464`, thấp hơn khoảng `4.4` lần.
Cùng dữ liệu, cùng mô hình — chỉ khác việc có LẤY TRUNG BÌNH qua nhiều fold
hay không. Trung bình qua `k` góc nhìn làm mượt đi phần may rủi của TỪNG
lần chia riêng lẻ.
::::

::::predict{#doan-anh-huong-cua-k commitOnce}
Vẫn hai mươi điểm ở trên. Giả sử đổi từ `5-fold` sang `20-fold` (mỗi fold
chỉ còn ĐÚNG một điểm — cách chia cực đoan nhất có thể, gọi là
leave-one-out).

**Trước khi đọc tiếp**, bạn đoán: so với `5-fold`, số LƯỢT khớp mô hình
(số lần phải chạy `np.linalg.lstsq`) trong MỘT vòng `20-fold` đầy đủ sẽ:

:::opt{correct}
Tăng lên, từ 5 lượt (5-fold) lên 20 lượt (20-fold) — vì số lượt khớp mô
hình trong một vòng k-fold LUÔN đúng bằng k, mỗi fold giữ lại làm val một
lần thì cần khớp lại trên phần còn lại đúng một lần
:::

:::opt
Giữ nguyên 5 lượt — vì tổng số điểm dữ liệu (20 điểm) không đổi, chỉ cách
CHIA lại thay đổi
::why
Gần đúng ở việc bạn để ý đúng: tổng số ĐIỂM DỮ LIỆU không hề đổi, vẫn là
20 điểm dù chia thành 5 fold hay 20 fold — quan sát đó không sai.

Chỗ lệch: số lượt KHỚP MÔ HÌNH không phụ thuộc số điểm dữ liệu — nó phụ
thuộc số FOLD, vì định nghĩa của k-fold là lặp đúng `k` lần, mỗi lần giữ
lại một fold khác làm val. Đổi từ `k=5` sang `k=20` giữ nguyên tổng số
điểm nhưng chia chúng thành nhiều fold NHỎ hơn — mà nhiều fold hơn nghĩa
là nhiều lượt lặp hơn, nên số lượt khớp mô hình tăng theo `k`, không đứng
yên.
::
:::

:::opt
Giảm xuống, vì mỗi fold giờ chỉ có một điểm nên khớp mô hình nhanh hơn,
tổng công sức ít đi
::why
Gần đúng ở việc mỗi LẦN khớp riêng lẻ trên `20-fold` đúng là dùng tập train
lớn hơn (19 điểm thay vì 16 điểm của `5-fold`) — một chi tiết có thật.

Chỗ lệch: câu hỏi hỏi về SỐ LƯỢT khớp (bao nhiêu lần phải gọi hàm khớp mô
hình), không hỏi về thời gian mỗi lượt nhanh hay chậm. Dù mỗi lượt riêng lẻ
có nhanh hơn hay chậm hơn, tổng SỐ LƯỢT vẫn đúng bằng `k` — `20-fold` cần
`20` lượt, nhiều hơn hẳn `5` lượt của `5-fold`, không phải ít hơn.
::
:::
::::

::::code{#viet_k_fold_cv}
Hoàn thiện `k_fold_cv`: với mỗi fold thứ `i`, `val_idx` là ĐÚNG fold đó,
`train_idx` là TẤT CẢ các fold còn lại gộp vào một mảng.

```python title=starter
import numpy as np

x = np.arange(1, 21, dtype=float)
noise = np.array([0.6,-0.4,0.9,-0.7,0.3,0.5,-0.8,0.4,-0.3,0.7,
                   -0.5,0.6,-0.6,0.2,-0.2,0.8,-0.9,0.5,-0.4,0.3])
y = 2 + 5*x + noise

def dac_trung(x, bac=2):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_and_mse(train_idx, val_idx):
    xt, yt = x[train_idx], y[train_idx]
    xv, yv = x[val_idx], y[val_idx]
    X_tho = dac_trung(xt)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(xt),1))])
    he_so, resid, rank, sv = np.linalg.lstsq(A, yt, rcond=None)
    w, b = he_so[:-1], he_so[-1]
    Xv = (dac_trung(xv) - tb) / sd
    return np.mean((yv - (Xv @ w + b)) ** 2)

idx_all = np.arange(20)
K = 5

def k_fold_cv(seed):
    rng = np.random.default_rng(seed)
    folds = np.array_split(rng.permutation(idx_all), K)
    diem_moi_fold = []
    for i in range(K):
        val_idx = ___                             # fold thu i
        train_idx = ___                            # tat ca fold KHAC gop lai
        diem_moi_fold.append(fit_and_mse(train_idx, val_idx))
    return np.mean(diem_moi_fold)

def single_split(seed):
    rng = np.random.default_rng(seed)
    perm = rng.permutation(idx_all)
    return fit_and_mse(perm[4:], perm[:4])

single_scores = np.array([single_split(seed) for seed in range(5)])
cv_scores = np.array([k_fold_cv(seed + 100) for seed in range(5)])

print(round(single_scores.std(), 4))
print(round(cv_scores.mean(), 4), round(cv_scores.std(), 4))
```

```python title=solution
import numpy as np

x = np.arange(1, 21, dtype=float)
noise = np.array([0.6,-0.4,0.9,-0.7,0.3,0.5,-0.8,0.4,-0.3,0.7,
                   -0.5,0.6,-0.6,0.2,-0.2,0.8,-0.9,0.5,-0.4,0.3])
y = 2 + 5*x + noise

def dac_trung(x, bac=2):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_and_mse(train_idx, val_idx):
    xt, yt = x[train_idx], y[train_idx]
    xv, yv = x[val_idx], y[val_idx]
    X_tho = dac_trung(xt)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(xt),1))])
    he_so, resid, rank, sv = np.linalg.lstsq(A, yt, rcond=None)
    w, b = he_so[:-1], he_so[-1]
    Xv = (dac_trung(xv) - tb) / sd
    return np.mean((yv - (Xv @ w + b)) ** 2)

idx_all = np.arange(20)
K = 5

def k_fold_cv(seed):
    rng = np.random.default_rng(seed)
    folds = np.array_split(rng.permutation(idx_all), K)
    diem_moi_fold = []
    for i in range(K):
        val_idx = folds[i]
        train_idx = np.concatenate([folds[j] for j in range(K) if j != i])
        diem_moi_fold.append(fit_and_mse(train_idx, val_idx))
    return np.mean(diem_moi_fold)

def single_split(seed):
    rng = np.random.default_rng(seed)
    perm = rng.permutation(idx_all)
    return fit_and_mse(perm[4:], perm[:4])

single_scores = np.array([single_split(seed) for seed in range(5)])
cv_scores = np.array([k_fold_cv(seed + 100) for seed in range(5)])

print(round(single_scores.std(), 4))
print(round(cv_scores.mean(), 4), round(cv_scores.std(), 4))
```

```python title=test
assert round(single_scores.std(), 4) == 0.2053, f"std cua 5 lan chia don le phai la 0.2053 -- dang ra {round(single_scores.std(), 4)}"
assert round(cv_scores.mean(), 4) == 0.4018, f"trung binh cua 5 lan CV phai la 0.4018 -- dang ra {round(cv_scores.mean(), 4)}"
assert round(cv_scores.std(), 4) == 0.0464, f"std cua 5 lan CV phai la 0.0464 -- dang ra {round(cv_scores.std(), 4)}"
assert cv_scores.std() < single_scores.std(), "std cua CV phai THAP HON han std cua chia don le -- CV on dinh hon"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trong vòng lặp `for i in range(K)`. `val_idx` chỉ đơn giản là `folds[i]` — fold thứ `i` của vòng lặp hiện tại. `train_idx` phải GỘP tất cả các fold KHÁC `i` — dùng `np.concatenate` trên một list comprehension chạy qua mọi `j` khác `i`.
- kind: strategy
  body: 'val_idx: `folds[i]`. train_idx: `np.concatenate([folds[j] for j in range(K) if j != i])`.'
- kind: one-line
  body: 'Hai chỗ trống: `folds[i]` và `np.concatenate([folds[j] for j in range(K) if j != i])`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: val_idx phai la folds[i] (dung chi so i cua vong lap, khong duoc co dinh mot fold), va train_idx phai GOM cac fold con lai bang np.concatenate -- ca hai deu phai phu thuoc dung vao i dang chay
  requireAst:
  - kind: uses-name, target: folds, min: 2
  - kind: uses-name, target: i, min: 2
  - kind: uses-call, target: concatenate, min: 1
  # Da thu that (ast.parse): co dinh val_idx = folds[0] (khong dung i cua
  # vong lap) cho folds=2, i=1 (chi con o train_idx), concatenate=1 -- i tut
  # duoi nguong 2, bi chan. Loi giai that cho dung folds=2, i=2, concat=1,
  # qua sach ca ba nguong.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^0\\.2053\\n0\\.4018 0\\.0464\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
4.4 lần ổn định hơn — không phải nhờ dữ liệu tốt hơn, chỉ nhờ nhìn nó qua
nhiều góc rồi lấy trung bình.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chín bài vừa qua đã ráp gần đủ mọi mảnh: khớp một đường thẳng (bài 1), đo
sai số đúng cách (bài 2), tìm hệ số bằng lặp dần thay vì công thức đóng
(bài 3), mở rộng sang đường cong (bài 4), làm cho việc lặp dần đó hội tụ
nhanh (bài 5), phát hiện khi mô hình học thuộc nhiễu (bài 6), trị nó bằng
cách phạt hệ số (bài 7), tách dữ liệu đúng kỷ luật để chọn tham số trung
thực (bài 8), và giờ, làm cho việc chọn đó ổn định hơn qua nhiều lần chia
(bài 9).

Còn thiếu đúng một việc: RÁP tất cả những mảnh đó lại thành MỘT quy trình
liền mạch, từ dữ liệu thô tới một con số cuối cùng đáng tin. Bài sau làm
đúng việc đó.
::::

::::checkpoint{mastery=0.8}
::::
