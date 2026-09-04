---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.boss-hoi-quy-va-gradient-descent
title: "BOSS — Ráp pipeline hồi quy và gradient descent"
summary: "Pipeline đầy đủ trên 8 điểm train + 4 val + 3 test (đặc trưng đa thức bậc 7, chuẩn hoá theo train, gradient descent có L2, 50 bước): quét 3 giá trị lambda, val chọn lambda=0.0 (val MSE=0.6183, thấp nhất) — test MSE trung thực cuối cùng là 0.3618. Nếu lỡ nhìn thẳng vào test để chọn, lambda=0.05 có test MSE=0.1971 (nhìn 'đẹp' hơn) nhưng đó là rò rỉ, đúng bài học T8.1.8: pipeline này chỉ dùng val để quyết định."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-hoi-quy-va-gradient-descent]
requires: [ai.cross-validation]
concepts: [ai.boss-hoi-quy-va-gradient-descent]
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
Chín mảnh riêng lẻ. Giờ ráp lại thành một quy trình duy nhất, từ dữ liệu
thô tới một con số cuối cùng dám tin.
::::

::::explain{#rap-pipeline-day-du}
`huan_luyen_va_danh_gia` đi qua ĐÚNG năm trạm, THEO ĐÚNG THỨ TỰ đã học:

1. **Dữ liệu thô, có nhiễu** — tám điểm train, bốn điểm validation, ba điểm
   test, tất cả sinh từ quan hệ thật `y = 2 + 5x` cộng nhiễu (mỗi tập một
   dải `x` và một dải nhiễu riêng — val và test hoàn toàn KHÔNG trùng train).
2. **Đặc trưng đa thức bậc 7** (bài `dac-trung-da-thuc`) — tám điểm train
   thì bảy hệ số tự do gần như đủ khớp CHÍNH XÁC, đúng tình huống dễ
   overfitting của bài `overfitting-va-do-phuc-tap`.
3. **Chuẩn hoá** (bài `chuan-hoa-dac-trung`) — `trung_bình` và
   `độ_lệch_chuẩn` CHỈ tính từ TRAIN, rồi áp CÙNG hai con số đó lên val và
   test. Không tính lại chuẩn hoá riêng cho val/test — nếu làm vậy, val và
   test sẽ bị "nhìn thấy" ngay từ bước chuẩn hoá, trước cả khi mô hình chạm
   tới chúng.
4. **Gradient descent có phạt L2** (bài `gradient-descent-tu-so-0` +
   `regularization-l1-l2`) — với MỖI giá trị `λ` trong một danh sách ứng
   viên, chạy gradient descent riêng, được một cặp `(w, b)`.
5. **Chọn `λ` qua validation, báo cáo qua test** (bài `train-val-test-split`)
   — đo `val MSE` của từng `λ`, giữ `λ` có `val MSE` THẤP NHẤT; rồi, và CHỈ
   rồi, đo `test MSE` của MÔ HÌNH đã chốt — đúng một lần, không dùng test
   để chọn bất cứ điều gì.

Không có khái niệm MỚI nào ở bài này — mỗi trạm là một bài đã học, chỉ khác
là ĐẦU RA của trạm này trở thành ĐẦU VÀO của trạm kế tiếp, liền một mạch.
::::

::::example{#pipeline-chay-that}
Tám điểm train (`x = 1..8`), bốn điểm val (`x = 1.5, 3.5, 5.5, 7.5`), ba
điểm test (`x = 2.5, 4.5, 6.5`) — cả ba đều sinh từ `y = 2 + 5x` cộng nhiễu
riêng. Đặc trưng bậc 7, chuẩn hoá theo train, gradient descent L2 với ba
giá trị `λ` (một ngân sách bước cố định và khiêm tốn — `50` bước — đúng
tinh thần bài `gradient-descent-tu-so-0`: so sánh CÔNG BẰNG giữa các `λ`
quan trọng hơn việc chạy tới hội tụ tuyệt đối):

```python title=readonly
import numpy as np

x = np.array([1,2,3,4,5,6,7,8], dtype=float)
noise_train = np.array([0.3, -0.4, 0.2, -0.5, 0.6, -0.2, 0.4, -0.3])
y_train = 2 + 5*x + noise_train

x_val = np.array([1.5, 3.5, 5.5, 7.5])
noise_val = np.array([-0.3, 0.4, -0.5, 0.2])
y_val = 2 + 5*x_val + noise_val

x_test = np.array([2.5, 4.5, 6.5])
noise_test = np.array([0.5, -0.6, 0.3])
y_test = 2 + 5*x_test + noise_test

BAC = 7
def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

X_tho_train = dac_trung(x, BAC)
tb, sd = X_tho_train.mean(axis=0), X_tho_train.std(axis=0)   # chi tinh tu TRAIN

def chuan_hoa(xs):
    return (dac_trung(xs, BAC) - tb) / sd

X_train, X_val, X_test = chuan_hoa(x), chuan_hoa(x_val), chuan_hoa(x_test)

def mat_mat(X, y, w, b):
    return np.mean((y - (X @ w + b)) ** 2)

def huan_luyen_ridge(X, y, lam, lr, so_buoc):
    n, d = X.shape
    w, b = np.zeros(d), 0.0
    for _ in range(so_buoc):
        sai_so = (X @ w + b) - y
        dw = (2.0 / n) * (X.T @ sai_so) + 2.0 * lam * w
        db = (2.0 / n) * np.sum(sai_so)
        w -= lr * dw
        b -= lr * db
    return w, b

cac_lambda = [0.0, 0.05, 0.5]
lam_tot_nhat, val_tot_nhat, w_tot_nhat, b_tot_nhat = None, np.inf, None, None
for lam in cac_lambda:
    w, b = huan_luyen_ridge(X_train, y_train, lam, 0.1, 50)
    mv = mat_mat(X_val, y_val, w, b)
    mt = mat_mat(X_test, y_test, w, b)
    print(f"lam={lam}: val={round(mv,4)} test={round(mt,4)}")
    if mv < val_tot_nhat:
        val_tot_nhat, lam_tot_nhat, w_tot_nhat, b_tot_nhat = mv, lam, w, b

mse_test_cuoi = mat_mat(X_test, y_test, w_tot_nhat, b_tot_nhat)
print("lambda chon qua val:", lam_tot_nhat)
print("TEST MSE CUOI CUNG (trung thuc):", round(mse_test_cuoi, 4))
```

```text title=readonly
lam=0.0: val=0.6183 test=0.3618
lam=0.05: val=0.9424 test=0.1971
lam=0.5: val=7.4357 test=4.4995
lambda chon qua val: 0.0
TEST MSE CUOI CUNG (trung thuc): 0.3618
```

Pipeline chọn `λ = 0.0` — KHÔNG phạt gì — vì đó là giá trị cho `val MSE`
thấp nhất (`0.6183`). Điều đó không có nghĩa regularization vô dụng: nó có
nghĩa validation, chạy đúng kỷ luật, phát hiện ra RẰNG bài toán CỤ THỂ này
(tám điểm train, gradient descent giới hạn `50` bước) không cần phạt thêm
để tổng quát tốt — con số nói lên đúng SỰ THẬT của trường hợp này, không
phải một giả định áp đặt trước.

Nhìn kỹ thêm một điều thú vị, đúng bài học của `train-val-test-split`: nếu
lỡ nhìn thẳng vào cột `test` để chọn `λ`, sẽ thấy `λ = 0.05` có test MSE
`0.1971` — THẤP hơn cả `0.3618` của lựa chọn trung thực. Con số đó "đẹp"
hơn, nhưng đó CHÍNH LÀ rò rỉ: chọn `λ` để LÀM ĐẸP con số test không phải
điều pipeline này làm. Pipeline chỉ dùng `val` để quyết định, và chỉ chạm
vào `test` đúng MỘT lần, sau khi mọi quyết định đã chốt xong. `0.3618` —
không phải `0.1971` — là con số trung thực để báo cáo.
::::

::::predict{#doan-neu-doi-thu-tu-buoc commitOnce}
Giả sử ai đó viết lại pipeline này theo một thứ tự KHÁC: tính `trung_bình`
và `độ_lệch_chuẩn` từ TOÀN BỘ dữ liệu (train + val + test gộp chung), thay
vì chỉ từ train — rồi mới tách ra huấn luyện, chọn `λ`, và báo cáo như cũ.

**Trước khi đọc tiếp**, bạn đoán: cách làm này có vi phạm đúng nguyên tắc
mà pipeline gốc đang tuân thủ không?

:::opt{correct}
Có — dù `w`, `b` vẫn chỉ được khớp từ train, hai con số `trung_bình`/
`độ_lệch_chuẩn` đã "nhìn thấy" val và test TRƯỚC khi mô hình chạm vào
chúng, nên val/test không còn hoàn toàn "chưa từng chạm" nữa
:::

:::opt
Không — vì `trung_bình` và `độ_lệch_chuẩn` chỉ là hai con số thống kê đơn
giản, không phải "huấn luyện" theo nghĩa tìm `w`, `b`
::why
Gần đúng ở việc `trung_bình`/`độ_lệch_chuẩn` đúng là hai con số ĐƠN GIẢN
hơn nhiều so với việc tìm ra `w`, `b` bằng gradient descent — không sai về
độ phức tạp.

Chỗ lệch: "đơn giản" không đồng nghĩa "không rò rỉ thông tin". Hai con số
đó được TÍNH TỪ dữ liệu — nếu tính chúng từ CẢ val/test, thì val/test đã
âm thầm ảnh hưởng tới cách MỌI điểm dữ liệu (kể cả điểm train) được chuẩn
hoá, trước khi bước huấn luyện bắt đầu. Đây là một dạng rò rỉ tinh vi hơn
việc nhìn thẳng vào test để chọn `λ` (bài `train-val-test-split`), nhưng
cùng bản chất: một phần thông tin của val/test đã lọt vào quy trình trước
khi nó lẽ ra được phép.
::
:::

:::opt
Không — vì `trung_bình` và `độ_lệch_chuẩn` được tính lại MỖI lần thử một
`λ` mới, nên không có gì bị "học tủ" giữa các lần thử
::why
Gần đúng ở việc bạn để ý đúng: pipeline THẬT SỰ có lặp qua nhiều `λ`, và
mỗi vòng lặp đó không tính lại chuẩn hoá — nhưng đó không phải điều câu hỏi
đang hỏi.

Chỗ lệch: vấn đề không nằm ở việc chuẩn hoá có bị tính LẶP LẠI qua các `λ`
hay không — mà nằm ở việc chuẩn hoá được tính từ NGUỒN DỮ LIỆU nào, ngay
từ đầu. Dù chỉ tính MỘT lần duy nhất, nếu lần tính đó đã gộp cả val và
test vào, thông tin của chúng đã rò vào hai con số `trung_bình`/
`độ_lệch_chuẩn` — và mọi phép chuẩn hoá dùng lại hai con số đó về sau đều
mang theo phần rò rỉ ấy.
::
:::
::::

::::code{#hoan_thien_pipeline}
Hoàn thiện `huan_luyen_ridge` (thêm số hạng phạt L2 vào gradient của `w`)
và bước chọn `λ` (giữ `λ` có `val MSE` NHỎ NHẤT — không nhìn vào test).

```python title=starter
import numpy as np

x = np.array([1,2,3,4,5,6,7,8], dtype=float)
noise_train = np.array([0.3, -0.4, 0.2, -0.5, 0.6, -0.2, 0.4, -0.3])
y_train = 2 + 5*x + noise_train

x_val = np.array([1.5, 3.5, 5.5, 7.5])
noise_val = np.array([-0.3, 0.4, -0.5, 0.2])
y_val = 2 + 5*x_val + noise_val

x_test = np.array([2.5, 4.5, 6.5])
noise_test = np.array([0.5, -0.6, 0.3])
y_test = 2 + 5*x_test + noise_test

BAC = 7
def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

X_tho_train = dac_trung(x, BAC)
tb, sd = X_tho_train.mean(axis=0), X_tho_train.std(axis=0)

def chuan_hoa(xs):
    return (dac_trung(xs, BAC) - tb) / sd

X_train, X_val, X_test = chuan_hoa(x), chuan_hoa(x_val), chuan_hoa(x_test)

def mat_mat(X, y, w, b):
    return np.mean((y - (X @ w + b)) ** 2)

def huan_luyen_ridge(X, y, lam, lr, so_buoc):
    n, d = X.shape
    w, b = np.zeros(d), 0.0
    for _ in range(so_buoc):
        sai_so = (X @ w + b) - y
        dw = ___                       # dao ham MSE cong them phat L2: 2*lam*w
        db = (2.0 / n) * np.sum(sai_so)
        w -= lr * dw
        b -= lr * db
    return w, b

cac_lambda = [0.0, 0.05, 0.5]
lam_tot_nhat, val_tot_nhat, w_tot_nhat, b_tot_nhat = None, np.inf, None, None
for lam in cac_lambda:
    w, b = huan_luyen_ridge(X_train, y_train, lam, 0.1, 50)
    mv = mat_mat(X_val, y_val, w, b)
    if ___:                            # mv nho hon val_tot_nhat hien tai?
        val_tot_nhat, lam_tot_nhat, w_tot_nhat, b_tot_nhat = mv, lam, w, b

mse_test_cuoi = mat_mat(X_test, y_test, w_tot_nhat, b_tot_nhat)

print(lam_tot_nhat)
print(round(mse_test_cuoi, 4))
```

```python title=solution
import numpy as np

x = np.array([1,2,3,4,5,6,7,8], dtype=float)
noise_train = np.array([0.3, -0.4, 0.2, -0.5, 0.6, -0.2, 0.4, -0.3])
y_train = 2 + 5*x + noise_train

x_val = np.array([1.5, 3.5, 5.5, 7.5])
noise_val = np.array([-0.3, 0.4, -0.5, 0.2])
y_val = 2 + 5*x_val + noise_val

x_test = np.array([2.5, 4.5, 6.5])
noise_test = np.array([0.5, -0.6, 0.3])
y_test = 2 + 5*x_test + noise_test

BAC = 7
def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

X_tho_train = dac_trung(x, BAC)
tb, sd = X_tho_train.mean(axis=0), X_tho_train.std(axis=0)

def chuan_hoa(xs):
    return (dac_trung(xs, BAC) - tb) / sd

X_train, X_val, X_test = chuan_hoa(x), chuan_hoa(x_val), chuan_hoa(x_test)

def mat_mat(X, y, w, b):
    return np.mean((y - (X @ w + b)) ** 2)

def huan_luyen_ridge(X, y, lam, lr, so_buoc):
    n, d = X.shape
    w, b = np.zeros(d), 0.0
    for _ in range(so_buoc):
        sai_so = (X @ w + b) - y
        dw = (2.0 / n) * (X.T @ sai_so) + 2.0 * lam * w
        db = (2.0 / n) * np.sum(sai_so)
        w -= lr * dw
        b -= lr * db
    return w, b

cac_lambda = [0.0, 0.05, 0.5]
lam_tot_nhat, val_tot_nhat, w_tot_nhat, b_tot_nhat = None, np.inf, None, None
for lam in cac_lambda:
    w, b = huan_luyen_ridge(X_train, y_train, lam, 0.1, 50)
    mv = mat_mat(X_val, y_val, w, b)
    if mv < val_tot_nhat:
        val_tot_nhat, lam_tot_nhat, w_tot_nhat, b_tot_nhat = mv, lam, w, b

mse_test_cuoi = mat_mat(X_test, y_test, w_tot_nhat, b_tot_nhat)

print(lam_tot_nhat)
print(round(mse_test_cuoi, 4))
```

```python title=test
assert lam_tot_nhat == 0.0, f"lambda chon qua validation phai la 0.0 -- dang ra {lam_tot_nhat}"
assert round(val_tot_nhat, 4) == 0.6183, f"val mse cua lambda tot nhat phai la 0.6183 -- dang ra {round(val_tot_nhat, 4)}"
assert round(mse_test_cuoi, 4) == 0.3618, f"test mse cuoi cung phai la 0.3618 -- dang ra {round(mse_test_cuoi, 4)}"

# lam=0.0 (lambda THANG cuoc) lam so hang phat L2 bang 0 du dau cong/tru co
# dung hay khong -- rieng kiem tra ba assert tren KHONG the phan biet dau
# cua so hang phat. Goi rieng huan_luyen_ridge voi mot lambda KHAC 0 de ep
# so hang phat thuc su co tac dung, roi doi chieu voi so lieu da thay trong
# vi du (lam=0.05: val=0.9424).
w_lam005, b_lam005 = huan_luyen_ridge(X_train, y_train, 0.05, 0.1, 50)
assert round(mat_mat(X_val, y_val, w_lam005, b_lam005), 4) == 0.9424, f"val mse rieng voi lam=0.05 phai la 0.9424 (dau cong cua so hang phat L2 phai dung) -- dang ra {round(mat_mat(X_val, y_val, w_lam005, b_lam005), 4)}"
```

:::hints
- kind: attention
  body: Chỗ trống đầu là gradient của MSE (`(2.0/n) * (X.T @ sai_so)`, đã học ở bài `gradient-descent-tu-so-0`) CỘNG THÊM số hạng phạt L2 (`2.0 * lam * w`, đã học ở bài `regularization-l1-l2`) — thiếu số hạng phạt thì `lam` không hề có tác dụng gì. Chỗ trống hai là điều kiện "val MSE hiện tại có THẤP HƠN val MSE tốt nhất đã thấy trước đó không" — dùng đúng toán tử `<`.
- kind: strategy
  body: 'dw: `(2.0 / n) * (X.T @ sai_so) + 2.0 * lam * w`. Điều kiện chọn: `mv < val_tot_nhat`.'
- kind: one-line
  body: 'Hai chỗ trống: `(2.0 / n) * (X.T @ sai_so) + 2.0 * lam * w` và `mv < val_tot_nhat`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: dw phai cong them so hang phat L2 (2*lam*w) vao dao ham MSE -- thieu no thi lam khong co tac dung gi; dieu kien chon lambda phai so sanh mv voi val_tot_nhat bang toan tu < -- khong duoc luon dung (chon lambda cuoi cung) hay luon sai (khong bao gio cap nhat)
  requireAst:
  - kind: uses-name, target: lam, min: 3
  - kind: uses-name, target: w, min: 6
  - kind: uses-name, target: val_tot_nhat, min: 1
  - kind: uses-operator, target: "<", min: 1
  # Da thu that (ast.parse): bo phan phat L2 (dw chi con
  # (2.0/n)*(X.T@sai_so), khong cong 2*lam*w) cho lam=2 (duoi nguong 3), w=5
  # (duoi nguong 6) -- bi chan. Dieu kien luon True (khong so sanh gi) cho
  # val_tot_nhat=0 (duoi nguong 1) va "<"=0 (duoi nguong 1) -- bi chan doc
  # lap. Loi giai that cho dung lam=3, w=6, val_tot_nhat=1, "<"=1.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^0\\.0\\n0\\.3618\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ tám điểm nhiễu tới một con số test duy nhất, chưa từng bị chạm trước đó
— cả một quy trình, không một bước nào bỏ qua kỷ luật.
::::

::::reflect{#nghi-lai}
Mười bài, một hành trình: từ "khớp một đường thẳng qua vài điểm" (bài 1)
tới một PIPELINE máy học có kỷ luật — đo sai số đúng cách, tìm hệ số bằng
lặp dần, mở rộng sang đường cong, làm việc lặp đó hội tụ nhanh, phát hiện
và trị overfitting, tách dữ liệu tách bạch train/val/test, ổn định hoá bằng
cross-validation, và ráp tất cả lại thành một quy trình đầu-cuối vừa chạy
xong.

Mọi con số trong track này đều đến từ ĐÚNG một công cụ: gradient descent,
đạo hàm riêng viết TAY, cập nhật `w ← w − lr·∂L/∂w`. Track tiếp theo
(`phan-loai-va-danh-gia`) sẽ đổi bài toán — từ dự đoán một CON SỐ (hồi quy)
sang dự đoán một NHÃN (phân loại) — nhưng gradient descent, hàm mất mát,
regularization, và train/val/test vẫn là nền tảng y hệt, chỉ đổi công thức
cụ thể của hàm mất mát.

Xa hơn nữa, ở T8.2 (mạng nơ-ron), CHÍNH gradient descent này vẫn là động
cơ — nhưng đạo hàm sẽ không còn viết tay từng công thức `∂L/∂w` như mười
bài vừa qua nữa. Một kỹ thuật gọi là **autograd** sẽ tính đạo hàm đó TỰ
ĐỘNG, cho những hàm mất mát phức tạp hơn nhiều lần công thức `(y − wx − b)²`
đã quen thuộc ở đây. Nắm chắc CƠ CHẾ tay ở mười bài này — nó không biến
mất, nó được TỰ ĐỘNG HOÁ.
::::

::::checkpoint{mastery=0.85}
::::
