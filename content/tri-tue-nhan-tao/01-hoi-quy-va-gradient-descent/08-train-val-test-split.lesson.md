---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.train-val-test-split
title: "Train / validation / test split"
summary: "15 điểm (y=2+5x+nhiễu) tách thành train(9)/val(3)/test(3): quét bậc đa thức 1-8, chọn bậc qua val (đúng kỷ luật) ra bậc=2, test MSE trung thực=0.1003. Nếu thay vào đó chọn bậc bằng cách nhìn thẳng vào test (rò rỉ), quy trình chọn bậc=1 khác hẳn, với con số tự-báo-cáo 0.0911 — hai lựa chọn KHÁC NHAU chứng minh: để test set ảnh hưởng quyết định là làm hỏng vai trò của nó như một phép đo chưa từng bị chạm tới."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.train-val-test-split]
requires: [ai.regularization]
concepts: [ai.train-val-test-split]
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
Bài trước để lại một câu hỏi chưa trả lời: `λ` nào tốt nhất? Đo bằng test —
nhưng dùng CHÍNH test để CHỌN thì test còn đo được gì nữa?
::::

::::explain{#vi-sao-can-ba-tap}
`overfitting-va-do-phuc-tap` dùng một tập test để PHÁT HIỆN overfitting —
dùng đúng MỘT lần, chỉ để đo, không hề dùng để quyết định gì. Nhưng bài
`regularization-l1-l2` để lộ một nhu cầu khác: cần THỬ nhiều giá trị `λ`
(hay nhiều bậc đa thức, nhiều lựa chọn khác) rồi CHỌN cái tốt nhất. Nếu
dùng chính test set để chọn — thử từng `λ`, đo trên test, giữ `λ` nào cho
test MSE thấp nhất — thì test set không còn là dữ liệu "chưa từng chạm" nữa.
Việc CHỌN đã bị ảnh hưởng bởi chính test set, nên con số test cuối cùng
không còn đáng tin để nói mô hình sẽ làm tốt tới đâu trên dữ liệu THẬT SỰ
mới — đây gọi là **rò rỉ** (leakage).

Giải pháp: tách dữ liệu thành BA phần, không phải hai.

> **Train** — dùng để KHỚP mô hình (tìm `w`, `b` cho một `λ` cụ thể).
>
> **Validation** (val) — dùng để SO SÁNH và CHỌN giữa nhiều lựa chọn (nhiều
> `λ`, nhiều bậc, ...) — đây là "phòng thử", được nhìn NHIỀU lần trong lúc
> chọn.
>
> **Test** — dùng ĐÚNG MỘT LẦN, ở CUỐI CÙNG, sau khi mọi lựa chọn đã chốt
> — để báo cáo một con số trung thực về khả năng tổng quát hoá.

Val "bị nhìn nhiều lần" trong lúc chọn — nên nó cũng có thể bị "học tủ" một
chút, giống cách train có thể bị overfit. Test không hề tham gia vào bất kỳ
quyết định nào — nó là con số ĐÁNG TIN CUỐI CÙNG, chỉ vì chưa từng bị dùng
để chọn bất cứ điều gì.
::::

::::example{#chon-bac-qua-val-doi-chieu-ro-ri}
Mười lăm điểm (`x = 1` tới `15`, quan hệ thật `y = 2 + 5x` cộng nhiễu),
tách: `test` là ba điểm chỉ số chia hết cho 5, `val` là ba điểm chỉ số dư 1,
`train` là chín điểm còn lại. Quét bậc đa thức từ `1` tới `8`, đo MSE trên
cả ba tập:

```python title=readonly
import numpy as np

x = np.arange(1, 16, dtype=float)
noise = np.array([0.5,-0.3,0.8,-0.6,0.2,0.4,-0.7,0.3,-0.2,0.6,-0.4,0.5,-0.5,0.1,-0.1])
y = 2 + 5*x + noise

idx = np.arange(15)
test_idx = idx[idx % 5 == 0]
val_idx = idx[idx % 5 == 1]
train_idx = idx[(idx % 5 != 0) & (idx % 5 != 1)]

x_train, y_train = x[train_idx], y[train_idx]
x_val, y_val = x[val_idx], y[val_idx]
x_test, y_test = x[test_idx], y[test_idx]

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_bac(bac):
    X_tho = dac_trung(x_train, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(x_train),1))])
    he_so, *_ = np.linalg.lstsq(A, y_train, rcond=None)
    return he_so[:-1], he_so[-1], tb, sd

def mse_bac(bac, w, b, tb, sd, xs, ys):
    Xc = (dac_trung(xs, bac) - tb) / sd
    return np.mean((ys - (Xc @ w + b)) ** 2)

mse_val, mse_test = {}, {}
for bac in range(1, 9):
    w, b, tb, sd = fit_bac(bac)
    mse_val[bac] = mse_bac(bac, w, b, tb, sd, x_val, y_val)
    mse_test[bac] = mse_bac(bac, w, b, tb, sd, x_test, y_test)
    print("bac", bac, ": val =", round(mse_val[bac], 4), " test =", round(mse_test[bac], 4))

bac_qua_val = min(mse_val, key=mse_val.get)
bac_qua_test = min(mse_test, key=mse_test.get)
print("Chon qua VAL (dung ky luat):", bac_qua_val, "-> test THAT SU:", round(mse_test[bac_qua_val], 4))
print("Chon qua TEST truc tiep (RO RI):", bac_qua_test, "-> test 'dep':", round(mse_test[bac_qua_test], 4))
```

```text title=readonly
bac 1 : val = 0.4213  test = 0.0911
bac 2 : val = 0.4145  test = 0.1003
bac 3 : val = 0.491  test = 0.2283
bac 4 : val = 1.8933  test = 4.6408
bac 5 : val = 4.4139  test = 19.9846
bac 6 : val = 11.7284  test = 103.2425
bac 7 : val = 76.5117  test = 1387.7423
bac 8 : val = 31.3557  test = 3147.6157
Chon qua VAL (dung ky luat): 2 -> test THAT SU: 0.1003
Chon qua TEST truc tiep (RO RI): 1 -> test 'dep': 0.0911
```

Chọn qua val (đúng kỷ luật): bậc `2` có val MSE thấp nhất (`0.4145`) — chốt
bậc `2`, rồi ĐO test đúng MỘT lần: `0.1003`. Chọn trực tiếp qua test (rò
rỉ, chỉ làm ở đây để ĐỐI CHIẾU, không phải cách nên làm): bậc `1` có test
MSE thấp nhất (`0.0911`) — một con số "đẹp" hơn con số trung thực. Điều
đáng chú ý nhất không phải việc con số nào thấp hơn — mà việc HAI QUY TRÌNH
CHỌN RA HAI BẬC KHÁC NHAU (`2` so với `1`). Đó chính là bằng chứng của rò
rỉ: quyết định đã bị chi phối bởi việc có nhìn vào test hay không — và một
khi test đã ảnh hưởng tới quyết định, con số nó báo cáo không còn là một
phép đo trên dữ liệu "chưa từng chạm" nữa.
::::

::::predict{#doan-neu-bo-val commitOnce}
Giả sử Byte bỏ hẳn tập val, chỉ giữ train và test — rồi khớp CẢ TÁM bậc
đa thức trên train, đo MSE của cả tám bậc đó trực tiếp trên test, và chọn
bậc có test MSE thấp nhất.

**Trước khi đọc tiếp**, bạn đoán: cách làm này có gặp đúng vấn đề mà ví dụ
trên vừa chỉ ra không?

:::opt{correct}
Có — đây CHÍNH LÀ cột "chọn qua test trực tiếp" trong ví dụ trên; bỏ val
không phải giải pháp, nó chỉ là đặt tên khác cho đúng cách làm gây rò rỉ
:::

:::opt
Không — vì test chỉ được dùng để ĐO, không được dùng để khớp mô hình
(`w`, `b` vẫn khớp từ train), nên không có dữ liệu train nào "rò" sang test
::why
Gần đúng ở việc bạn phân biệt đúng HAI việc khác nhau: khớp hệ số (`w`,
`b`) và chọn siêu tham số (ở đây là bậc đa thức). Đúng là hệ số vẫn chỉ
được khớp từ train, không đụng tới test — không có dữ liệu SỐ nào rò rỉ.

Chỗ lệch: rò rỉ ở đây không phải chuyện dữ liệu SỐ bị lẫn vào nhau — mà là
chuyện QUYẾT ĐỊNH (chọn bậc nào) bị ảnh hưởng bởi test. Việc "thử cả tám
bậc rồi giữ bậc có test MSE thấp nhất" chính là dùng test để RA QUYẾT ĐỊNH
— sau bước đó, con số test MSE báo cáo không còn trung thực nữa, vì nó đã
được "chọn lọc" theo hướng có lợi cho chính test set đó, đúng cơ chế ví dụ
trên đã đo được: bậc `1` (chọn qua test) khác hẳn bậc `2` (chọn qua val).
::
:::

:::opt
Không — vì cả tám bậc đều được đo trên CÙNG một test set, nên so sánh giữa
chúng vẫn công bằng
::why
Gần đúng ở việc "công bằng giữa các bậc" — đúng là cả tám bậc được đo trên
CÙNG dữ liệu test, không có bậc nào bị ưu ái bởi một test set khác.

Chỗ lệch: vấn đề không nằm ở sự công bằng GIỮA CÁC BẬC — mà nằm ở việc con
số CUỐI CÙNG được báo cáo (test MSE của bậc được chọn) không còn là một
phép đo "chưa từng chạm" nữa. Trong tám lần đo, có NHIỀU cơ hội để MỘT
trong chúng tình cờ khớp tốt với đúng phần nhiễu riêng của test set đó — và
quy trình chọn ra "kẻ may mắn nhất trong tám lần thử" đó, chứ không còn báo
cáo trung thực khả năng tổng quát hoá thật.
::
:::
::::

::::code{#chon_bac_qua_val_khong_qua_test}
Chọn bậc đa thức TỐT NHẤT bằng cách tìm bậc có `mse_val` NHỎ NHẤT (không
nhìn vào `mse_test` để chọn) — rồi chỉ dùng `mse_test` để BÁO CÁO con số
cuối cùng, đúng một lần.

```python title=starter
import numpy as np

x = np.arange(1, 16, dtype=float)
noise = np.array([0.5,-0.3,0.8,-0.6,0.2,0.4,-0.7,0.3,-0.2,0.6,-0.4,0.5,-0.5,0.1,-0.1])
y = 2 + 5*x + noise

idx = np.arange(15)
test_idx = idx[idx % 5 == 0]
val_idx = idx[idx % 5 == 1]
train_idx = idx[(idx % 5 != 0) & (idx % 5 != 1)]

x_train, y_train = x[train_idx], y[train_idx]
x_val, y_val = x[val_idx], y[val_idx]
x_test, y_test = x[test_idx], y[test_idx]

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_bac(bac):
    X_tho = dac_trung(x_train, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(x_train),1))])
    he_so, resid, rank, sv = np.linalg.lstsq(A, y_train, rcond=None)
    return he_so[:-1], he_so[-1], tb, sd

def mse_bac(bac, w, b, tb, sd, xs, ys):
    Xc = (dac_trung(xs, bac) - tb) / sd
    return np.mean((ys - (Xc @ w + b)) ** 2)

mse_val_theo_bac, mse_test_theo_bac = {}, {}
for bac in range(1, 9):
    w, b, tb, sd = fit_bac(bac)
    mse_val_theo_bac[bac] = mse_bac(bac, w, b, tb, sd, x_val, y_val)
    mse_test_theo_bac[bac] = mse_bac(bac, w, b, tb, sd, x_test, y_test)

bac_chon = ___                          # bac co mse_val NHO NHAT (khong dung mse_test de chon)
mse_test_cuoi_cung = mse_test_theo_bac[bac_chon]

print(bac_chon, round(mse_test_cuoi_cung, 4))
```

```python title=solution
import numpy as np

x = np.arange(1, 16, dtype=float)
noise = np.array([0.5,-0.3,0.8,-0.6,0.2,0.4,-0.7,0.3,-0.2,0.6,-0.4,0.5,-0.5,0.1,-0.1])
y = 2 + 5*x + noise

idx = np.arange(15)
test_idx = idx[idx % 5 == 0]
val_idx = idx[idx % 5 == 1]
train_idx = idx[(idx % 5 != 0) & (idx % 5 != 1)]

x_train, y_train = x[train_idx], y[train_idx]
x_val, y_val = x[val_idx], y[val_idx]
x_test, y_test = x[test_idx], y[test_idx]

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_bac(bac):
    X_tho = dac_trung(x_train, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(x_train),1))])
    he_so, resid, rank, sv = np.linalg.lstsq(A, y_train, rcond=None)
    return he_so[:-1], he_so[-1], tb, sd

def mse_bac(bac, w, b, tb, sd, xs, ys):
    Xc = (dac_trung(xs, bac) - tb) / sd
    return np.mean((ys - (Xc @ w + b)) ** 2)

mse_val_theo_bac, mse_test_theo_bac = {}, {}
for bac in range(1, 9):
    w, b, tb, sd = fit_bac(bac)
    mse_val_theo_bac[bac] = mse_bac(bac, w, b, tb, sd, x_val, y_val)
    mse_test_theo_bac[bac] = mse_bac(bac, w, b, tb, sd, x_test, y_test)

bac_chon = min(mse_val_theo_bac, key=mse_val_theo_bac.get)
mse_test_cuoi_cung = mse_test_theo_bac[bac_chon]

print(bac_chon, round(mse_test_cuoi_cung, 4))
```

```python title=test
assert bac_chon == 2, f"chon qua val phai ra bac 2 -- dang ra {bac_chon}"
assert round(mse_test_cuoi_cung, 4) == 0.1003, f"mse test cuoi cung (trung thuc) phai la 0.1003 -- dang ra {round(mse_test_cuoi_cung, 4)}"
assert round(mse_val_theo_bac[bac_chon], 4) == 0.4145, "mse_val cua bac da chon phai la 0.4145 -- gia tri nho nhat trong toan bo mse_val_theo_bac"
assert all(mse_val_theo_bac[bac_chon] <= v for v in mse_val_theo_bac.values()), "bac_chon phai co mse_val NHO NHAT trong tat ca cac bac -- dung dinh nghia cua viec chon qua validation"
```

:::hints
- kind: attention
  body: Chỗ trống chọn bậc bằng cách tìm KHOÁ có GIÁ TRỊ nhỏ nhất trong dict `mse_val_theo_bac` — dùng hàm `min(...)` với tham số `key` để so theo giá trị thay vì so theo khoá. TUYỆT ĐỐI không dùng `mse_test_theo_bac` ở bước chọn này.
- kind: strategy
  body: '`min(mse_val_theo_bac, key=mse_val_theo_bac.get)` — `min` trên các KHOÁ của dict, so sánh bằng GIÁ TRỊ tương ứng (lấy qua `.get`), trả về khoá có giá trị nhỏ nhất.'
- kind: one-line
  body: 'Chỗ trống: `min(mse_val_theo_bac, key=mse_val_theo_bac.get)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: bac_chon phai duoc tinh THAT SU tu mse_val_theo_bac bang min(...) -- khong duoc chep san mot con so, va TUYET DOI khong duoc dung mse_test_theo_bac de chon (do la ro ri, dung trong tam bai nay)
  requireAst:
  - kind: uses-name, target: mse_val_theo_bac, min: 3
  - kind: uses-call, target: min, min: 1
  # Da thu that (ast.parse): dien "2" (chep san, khong tinh toan) cho
  # mse_val_theo_bac=1 (chi con o dong dinh nghia dict, khong con Load nao),
  # min_call=0 -- duoi ca hai nguong (can 3 va 1), bi chan. Loi giai that
  # (min(mse_val_theo_bac, key=mse_val_theo_bac.get)) cho dung
  # mse_val_theo_bac=3, min_call=1, qua sach ca hai nguong.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^2 0\\.1003\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bậc 2 khi chọn qua val, bậc 1 khi lỡ nhìn vào test — hai quy trình khác
nhau ra hai câu trả lời khác nhau. Đó chính là bằng chứng: test đã bị chạm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cách tách `train/val/test` ở bài này dùng đúng MỘT lần chia — chỉ số chia
hết cho 5 vào test, dư 1 vào val, còn lại vào train. Nhưng phép chia đó
hoàn toàn phụ thuộc vào MAY MẮN: nếu tình cờ ba điểm rơi vào val đều lệch
theo cùng một hướng (toàn số dương, hay toàn số âm so với xu hướng chung),
kết quả "bậc nào tốt nhất" của TOÀN BỘ bài này có thể đổi khác, dù dữ liệu
GỐC không hề đổi gì.

Có cách nào bớt phụ thuộc vào MỘT lần chia may rủi đó không?
::::

::::checkpoint{mastery=0.8}
::::
