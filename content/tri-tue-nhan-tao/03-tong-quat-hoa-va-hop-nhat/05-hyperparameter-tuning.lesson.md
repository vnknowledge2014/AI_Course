---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.hyperparameter-tuning
title: "Hyperparameter tuning: grid search"
summary: "8 điểm train + 4 điểm val (y=3+2x+0.4x²+nhiễu): quét lưới 3×3 (bậc đa thức {1,2,3} × lambda L2 {0.0,1.0,10.0} = 9 tổ hợp), đo val MSE từng tổ hợp, chọn tổ hợp NHỎ NHẤT ra (bậc=3, lambda=1.0, val_mse=6.9789). Đối chiếu: chọn ngẫu nhiên (seed cố định) ra (bậc=3, lambda=10.0, val_mse=12.5032) và chọn theo trực giác 'phức tạp hơn luôn tốt' (bậc=3, lambda=0.0, val_mse=10.1448) — cả hai đều TỆ HƠN grid search, chứng minh quét có hệ thống thắng cả may rủi lẫn cảm tính."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.hyperparameter-tuning]
requires: [ai.boosting]
concepts: [ai.hyperparameter-tuning]
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
Boosting có `lr` và số vòng. Ridge có `λ`. Đặc trưng đa thức có bậc. Track
này đã dùng CẢ NÚI siêu tham số — nhưng luôn chỉ quét MỘT tham số một lúc.
Nếu có HAI cùng lúc thì sao?
::::

::::explain{#quet-luoi-nhieu-tham-so}
`train-val-test-split` (track hồi quy) đã dạy đúng kỷ luật: dùng **val** để
CHỌN, dùng **test** để BÁO CÁO đúng một lần. Bài đó chỉ quét MỘT siêu tham
số (bậc đa thức). Nhưng một mô hình thường có NHIỀU siêu tham số cùng lúc —
ở đây: bậc đa thức VÀ `λ` (mức phạt L2). Hai tham số không độc lập với
nhau: bậc cao cần phạt nhiều hơn để tránh overfit, bậc thấp có thể không
cần phạt gì — nên không thể quét từng cái riêng rồi ghép kết quả lại, cần
thử MỌI TỔ HỢP của cả hai.

**Grid search** (quét lưới) làm đúng việc đó: liệt kê một danh sách giá trị
ứng viên cho MỖI siêu tham số, tạo TẤT CẢ tổ hợp có thể (tích Descartes của
các danh sách), huấn luyện một mô hình cho MỖI tổ hợp, đo val MSE của từng
tổ hợp, rồi giữ tổ hợp có val MSE THẤP NHẤT — đúng kỷ luật val/test đã học,
chỉ mở rộng từ "quét một chiều" sang "quét nhiều chiều cùng lúc".

Cách làm này đối lập với hai cách chọn phổ biến ngoài đời: chọn NGẪU NHIÊN
(thử một tổ hợp bất kỳ, hy vọng may mắn) và chọn theo TRỰC GIÁC (ví dụ: "mô
hình phức tạp hơn luôn tốt hơn", nên cứ chọn bậc cao nhất). Bài này đo cả ba cách trên CÙNG dữ liệu,
để xem quét có hệ thống hơn được bao nhiêu so với may rủi và cảm tính.
::::

::::example{#grid-search-doi-dau-ngau-nhien-va-truc-giac}
Tám điểm train, bốn điểm val (`y = 3 + 2x + 0.4x²` cộng nhiễu, val là bốn
điểm KHÁC train). Lưới `3 × 3`: bậc đa thức `{1, 2, 3}`, `λ ∈ {0.0, 1.0,
10.0}` — chín tổ hợp, mỗi tổ hợp một ridge fit (chuẩn hoá theo train):

```python title=readonly
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def ridge_fit(X, y, lam):
    n, d = X.shape
    Xb = np.hstack([X, np.ones((n, 1))])
    I = np.eye(d + 1); I[-1, -1] = 0.0
    he_so = np.linalg.solve(Xb.T @ Xb + lam * I, Xb.T @ y)
    return he_so[:-1], he_so[-1]

x_train = np.arange(1, 9, dtype=float)
rng = np.random.default_rng(3)
y_train = true_fn(x_train) + rng.normal(0, 3.0, size=8)

x_val = np.array([1.5, 3.5, 5.5, 7.5])
rng_val = np.random.default_rng(30)
y_val = true_fn(x_val) + rng_val.normal(0, 3.0, size=4)

bac_list = [1, 2, 3]
lam_list = [0.0, 1.0, 10.0]
to_hop = [(bac, lam) for bac in bac_list for lam in lam_list]

val_mse_theo_to_hop = {}
for (bac, lam) in to_hop:
    X_tho = dac_trung(x_train, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    X_train_c = (X_tho - tb) / sd
    X_val_c = (dac_trung(x_val, bac) - tb) / sd
    w, b = ridge_fit(X_train_c, y_train, lam)
    val_mse_theo_to_hop[(bac, lam)] = np.mean((y_val - (X_val_c @ w + b)) ** 2)
    print(f"bac={bac} lam={lam}: val_mse={round(val_mse_theo_to_hop[(bac, lam)], 4)}")

to_hop_tot_nhat = min(val_mse_theo_to_hop, key=val_mse_theo_to_hop.get)
print("grid search chon:", to_hop_tot_nhat, "val_mse=", round(val_mse_theo_to_hop[to_hop_tot_nhat], 4))

rng_ngau_nhien = np.random.default_rng(7)
to_hop_ngau_nhien = to_hop[rng_ngau_nhien.integers(0, len(to_hop))]
print("chon ngau nhien:", to_hop_ngau_nhien, "val_mse=", round(val_mse_theo_to_hop[to_hop_ngau_nhien], 4))

to_hop_truc_giac = (3, 0.0)   # "bac cao nhat, khong phat gi" -- truc giac "phuc tap hon = tot hon"
print("chon theo truc giac:", to_hop_truc_giac, "val_mse=", round(val_mse_theo_to_hop[to_hop_truc_giac], 4))
```

```text title=readonly
bac=1 lam=0.0: val_mse=14.83
bac=1 lam=1.0: val_mse=13.461
bac=1 lam=10.0: val_mse=39.6404
bac=2 lam=0.0: val_mse=7.8002
bac=2 lam=1.0: val_mse=8.6835
bac=2 lam=10.0: val_mse=19.8525
bac=3 lam=0.0: val_mse=10.1448
bac=3 lam=1.0: val_mse=6.9789
bac=3 lam=10.0: val_mse=12.5032
grid search chon: (3, 1.0) val_mse= 6.9789
chon ngau nhien: (3, 10.0) val_mse= 12.5032
chon theo truc giac: (3, 0.0) val_mse= 10.1448
```

Chín tổ hợp, chín val MSE khác nhau, dao động từ `6.9789` tới `39.6404` —
gấp hơn `5` lần. Grid search quét hết cả chín, giữ đúng tổ hợp thấp nhất:
`bậc=3, λ=1.0` (`6.9789`). Chọn ngẫu nhiên (một seed cố định, để có thể
lặp lại) rơi trúng `bậc=3, λ=10.0` (`12.5032`) — tệ hơn gần gấp đôi. Chọn
theo trực giác "phức tạp hơn luôn tốt hơn" (`bậc=3, λ=0.0`, không phạt gì)
cho `10.1448` — cũng tệ hơn grid search, dù trực giác đó ĐÚNG một phần
(bậc `3` thật sự là bậc tốt nhất ở đây) nhưng SAI ở phần còn lại (bỏ qua
việc cần phạt để tránh overfit trên bậc cao). Quét có hệ thống không dựa
vào may mắn hay cảm tính — nó nhìn thấy CẢ CHÍN con số cùng lúc.
::::

::::predict{#doan-tang-luoi commitOnce}
Vẫn lưới trên. Giả sử mở rộng `lam_list` thêm giá trị `100.0` (từ `3` giá
trị lên `4`), giữ nguyên `bac_list` (`3` giá trị).

**Trước khi tính**, bạn đoán: tổng số tổ hợp cần huấn luyện trong lưới MỚI
là bao nhiêu?

:::opt{correct}
`12` — số tổ hợp LUÔN bằng tích số giá trị của từng danh sách (`3` bậc ×
`4` λ = `12`), không phải tổng (`3 + 4 = 7`)
:::

:::opt
`7` — cộng số giá trị của hai danh sách lại (`3` bậc cộng `4` λ)
::why
Gần đúng ở việc bạn để ý ĐÚNG có hai danh sách, mỗi danh sách một số lượng
giá trị riêng — quan sát về CẤU TRÚC đó không sai.

Chỗ lệch: grid search thử MỌI CẶP `(bậc, λ)` có thể — với `bậc` bất kỳ
trong `3` giá trị, GHÉP với `λ` bất kỳ trong `4` giá trị, mỗi bậc phải được
thử với CẢ BỐN giá trị λ, không phải chỉ một. Đó là phép NHÂN (`3 × 4 =
12`), không phải phép cộng — danh sách dài thêm một chút ở MỘT tham số làm
số tổ hợp nhân lên, không chỉ cộng thêm.
::
:::

:::opt
Vẫn `9` — thêm giá trị vào MỘT danh sách không ảnh hưởng tới số tổ hợp,
miễn danh sách kia không đổi
::why
Gần đúng ở việc bạn giữ đúng `bac_list` không đổi (`3` giá trị) — quan sát
đó chính xác.

Chỗ lệch: số tổ hợp phụ thuộc vào CẢ HAI danh sách cùng lúc, không chỉ danh
sách không đổi. Thêm MỘT giá trị vào `lam_list` (từ `3` lên `4` giá trị)
nghĩa là MỖI bậc trong `bac_list` giờ phải ghép thêm với giá trị λ mới đó —
tổng số tổ hợp tăng từ `3 × 3 = 9` lên `3 × 4 = 12`, không giữ nguyên.
::
:::
::::

::::code{#quet_luoi_chon_to_hop_tot_nhat}
Hoàn thiện việc tạo lưới tổ hợp (`to_hop`, mọi cặp `bậc, λ`), tính val MSE
cho từng tổ hợp, rồi chọn tổ hợp có val MSE NHỎ NHẤT.

```python title=starter
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def ridge_fit(X, y, lam):
    n, d = X.shape
    Xb = np.hstack([X, np.ones((n, 1))])
    I = np.eye(d + 1); I[-1, -1] = 0.0
    he_so = np.linalg.solve(Xb.T @ Xb + lam * I, Xb.T @ y)
    return he_so[:-1], he_so[-1]

x_train = np.arange(1, 9, dtype=float)
rng = np.random.default_rng(3)
y_train = true_fn(x_train) + rng.normal(0, 3.0, size=8)

x_val = np.array([1.5, 3.5, 5.5, 7.5])
rng_val = np.random.default_rng(30)
y_val = true_fn(x_val) + rng_val.normal(0, 3.0, size=4)

bac_list = [1, 2, 3]
lam_list = [0.0, 1.0, 10.0]
to_hop = ___                              # moi cap (bac, lam): [(bac, lam) for bac in bac_list for lam in lam_list]

val_mse_theo_to_hop = {}
for (bac, lam) in to_hop:
    X_tho = dac_trung(x_train, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    X_train_c = (X_tho - tb) / sd
    X_val_c = (dac_trung(x_val, bac) - tb) / sd
    w, b = ridge_fit(X_train_c, y_train, lam)
    val_mse_theo_to_hop[(bac, lam)] = ___  # np.mean((y_val - (X_val_c @ w + b)) ** 2)

to_hop_tot_nhat = ___                     # to hop co val MSE NHO NHAT: min(val_mse_theo_to_hop, key=val_mse_theo_to_hop.get)

print(to_hop_tot_nhat[0], to_hop_tot_nhat[1], round(val_mse_theo_to_hop[to_hop_tot_nhat], 4))
```

```python title=solution
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def ridge_fit(X, y, lam):
    n, d = X.shape
    Xb = np.hstack([X, np.ones((n, 1))])
    I = np.eye(d + 1); I[-1, -1] = 0.0
    he_so = np.linalg.solve(Xb.T @ Xb + lam * I, Xb.T @ y)
    return he_so[:-1], he_so[-1]

x_train = np.arange(1, 9, dtype=float)
rng = np.random.default_rng(3)
y_train = true_fn(x_train) + rng.normal(0, 3.0, size=8)

x_val = np.array([1.5, 3.5, 5.5, 7.5])
rng_val = np.random.default_rng(30)
y_val = true_fn(x_val) + rng_val.normal(0, 3.0, size=4)

bac_list = [1, 2, 3]
lam_list = [0.0, 1.0, 10.0]
to_hop = [(bac, lam) for bac in bac_list for lam in lam_list]

val_mse_theo_to_hop = {}
for (bac, lam) in to_hop:
    X_tho = dac_trung(x_train, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    X_train_c = (X_tho - tb) / sd
    X_val_c = (dac_trung(x_val, bac) - tb) / sd
    w, b = ridge_fit(X_train_c, y_train, lam)
    val_mse_theo_to_hop[(bac, lam)] = np.mean((y_val - (X_val_c @ w + b)) ** 2)

to_hop_tot_nhat = min(val_mse_theo_to_hop, key=val_mse_theo_to_hop.get)

print(to_hop_tot_nhat[0], to_hop_tot_nhat[1], round(val_mse_theo_to_hop[to_hop_tot_nhat], 4))
```

```python title=test
assert len(to_hop) == 9, f"phai co dung 9 to hop (3 bac x 3 lambda) -- dang ra {len(to_hop)}"
assert set(to_hop) == {(b, l) for b in [1,2,3] for l in [0.0,1.0,10.0]}, "to_hop phai la TICH DESCARTES day du cua bac_list va lam_list"
assert to_hop_tot_nhat == (3, 1.0), f"to hop tot nhat phai la (3, 1.0) -- dang ra {to_hop_tot_nhat}"
assert round(val_mse_theo_to_hop[to_hop_tot_nhat], 4) == 6.9789, f"val mse cua to hop tot nhat phai la 6.9789 -- dang ra {round(val_mse_theo_to_hop[to_hop_tot_nhat], 4)}"
assert round(val_mse_theo_to_hop[(3, 0.0)], 4) == 10.1448, "val mse cua (3, 0.0), tuong ung 'chon theo truc giac', phai la 10.1448"
assert round(val_mse_theo_to_hop[(3, 10.0)], 4) == 12.5032, "val mse cua (3, 10.0), tuong ung 'chon ngau nhien' trong bai, phai la 12.5032"
assert all(val_mse_theo_to_hop[to_hop_tot_nhat] <= v for v in val_mse_theo_to_hop.values()), "to_hop_tot_nhat phai co val MSE NHO NHAT trong TAT CA chin to hop"
```

:::hints
- kind: attention
  body: Ba chỗ trống. `to_hop` là một list comprehension LỒNG hai vòng lặp (`for bac in bac_list for lam in lam_list`), tạo mọi cặp `(bac, lam)` có thể. `val_mse_theo_to_hop[(bac, lam)]` là công thức MSE quen thuộc, tính từ `y_val`, `X_val_c`, `w`, `b` đã có sẵn. `to_hop_tot_nhat` tìm KHOÁ có GIÁ TRỊ nhỏ nhất trong dict — cùng khuôn `min(dict, key=dict.get)` đã dùng ở bài `train-val-test-split`.
- kind: strategy
  body: 'to_hop: `[(bac, lam) for bac in bac_list for lam in lam_list]`. val_mse_theo_to_hop[(bac, lam)]: `np.mean((y_val - (X_val_c @ w + b)) ** 2)`. to_hop_tot_nhat: `min(val_mse_theo_to_hop, key=val_mse_theo_to_hop.get)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `[(bac, lam) for bac in bac_list for lam in lam_list]`, `np.mean((y_val - (X_val_c @ w + b)) ** 2)`, và `min(val_mse_theo_to_hop, key=val_mse_theo_to_hop.get)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: to_hop phai duoc tinh THAT tu bac_list va lam_list (khong duoc chep san danh sach 9 cap), val_mse_theo_to_hop phai tinh THAT tu y_val/X_val_c/w/b, va to_hop_tot_nhat phai dung min(...) THAT tren val_mse_theo_to_hop
  requireAst:
  - kind: uses-name, target: bac_list, min: 1
  - kind: uses-name, target: lam_list, min: 1
  - kind: uses-call, target: min, min: 1
  - kind: uses-name, target: val_mse_theo_to_hop, min: 3
  # Da thu that (goi kiemAst that tren code day du): loi giai dung dat=true,
  # ca bon luat qua sach. Cheat chep san to_hop = [(1,0.0),(1,1.0),...] (danh
  # sach 9 cap viet tay, khong con dung bac_list/lam_list) VA
  # to_hop_tot_nhat = (3, 1.0) (chep san, khong goi min) lam CA BON luat cung
  # roi xuong duoi nguong (bac_list/lam_list mat het Load, min bien mat, va
  # val_mse_theo_to_hop mat hai lan xuat hien trong dong min(...)) -- bi chan
  # boi ca bon.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^3 1\\.0 6\\.9789\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chín tổ hợp, một con số nhỏ nhất — quét hết còn hơn đoán một cái. Grid
search thắng cả may rủi lẫn cảm tính, không phải vì nó "thông minh", mà vì
nó KHÔNG BỎ SÓT tổ hợp nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Grid search vừa chọn tổ hợp `(bậc, λ)` bằng cách nhìn vào val MSE — dữ liệu
train và val ở bài này đều đã SẠCH, chuẩn hoá đúng cách. Nhưng nếu bước
CHUẨN HOÁ đó, ngay từ đầu, đã lỡ tính từ dữ liệu SAI (ví dụ: tính từ cả
train lẫn test gộp lại, trước khi tách) — thì MỌI con số val MSE mà grid
search vừa quét, dù quét đúng cách, có còn đáng tin không?

Bài sau chỉ ra một lỗi kinh điển đúng chỗ đó.
::::

::::checkpoint{mastery=0.8}
::::
