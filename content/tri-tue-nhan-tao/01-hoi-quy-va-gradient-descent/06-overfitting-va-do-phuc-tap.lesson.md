---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.overfitting-va-do-phuc-tap
title: "Overfitting và độ phức tạp"
summary: "Tám điểm train (x=1..8, quan hệ thật là y=2+5x cộng nhiễu nhỏ): đa thức bậc 1 (đúng dạng) cho train MSE=0.1485, test MSE=0.0002 trên bảy điểm chưa từng thấy. Đa thức bậc 7 (8 hệ số khớp đúng 8 điểm) cho train MSE gần như 0 tuyệt đối (~2.6e-22) nhưng test MSE=1.7556 — cao gấp khoảng 8800 lần test MSE của bậc 1: train gần như hoàn hảo không đảm bảo gì về dữ liệu chưa từng thấy."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.overfitting]
requires: [ai.chuan-hoa-dac-trung]
concepts: [ai.overfitting]
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
Một mô hình khớp train gần như HOÀN HẢO. Vậy nó có phải mô hình TỐT NHẤT?
Không nhất thiết.
::::

::::explain{#khop-diem-hay-khop-quy-luat}
Bài `dac-trung-da-thuc` cho thấy: thêm đặc trưng đa thức (`x²`, `x³`, ...)
làm MSE trên chính dữ liệu đã khớp GIẢM. Câu hỏi tự nhiên tiếp theo: nếu
thêm CÀNG NHIỀU bậc, MSE có càng giảm nữa không? Có — và đây chính là vấn
đề.

Với `n` điểm dữ liệu, một đa thức bậc `n − 1` có ĐÚNG `n` hệ số tự do — đủ
để đi QUA CHÍNH XÁC từng điểm, không chừa sai số nào. Train MSE về gần `0`
không phải vì mô hình đã "hiểu" quy luật sinh ra dữ liệu — mà vì nó có đủ
tự do để LUỒN qua từng điểm, kể cả những điểm bị lệch bởi nhiễu ngẫu nhiên.
Đây gọi là **overfitting** (khớp quá đà): mô hình học thuộc luôn cả phần
NHIỄU của dữ liệu train, không chỉ học quy luật thật đứng sau nó.

Cách duy nhất phát hiện overfitting: có một bộ dữ liệu THỨ HAI, không hề
được dùng lúc khớp mô hình — gọi là **tập kiểm tra** (test set) — rồi đo
sai số trên đó. Một mô hình khớp quy luật thật (không chỉ khớp nhiễu) sẽ có
sai số trên test GẦN với sai số trên train. Một mô hình overfit sẽ có sai
số trên test cao hơn HẲN sai số trên train — nó "giỏi" trên dữ liệu đã
thấy, nhưng "kém" trên dữ liệu chưa từng thấy.
::::

::::example{#bac-1-doi-dau-bac-7}
Tám điểm train (`x = 1` tới `8`), quan hệ thật đứng sau chúng là
`y = 2 + 5x` cộng một chút nhiễu nhỏ. Bảy điểm test (`x = 1.5, 2.5, ..., 7.5`
— nằm GIỮA các điểm train, chưa từng được đưa vào lúc khớp) đối chiếu với
giá trị THẬT của `y = 2 + 5x`, không nhiễu:

```python title=readonly
import numpy as np

x_train = np.array([1,2,3,4,5,6,7,8], dtype=float)
y_train = np.array([7.3, 11.6, 17.2, 21.5, 27.6, 31.8, 37.4, 41.7])
x_test = np.array([1.5,2.5,3.5,4.5,5.5,6.5,7.5])
y_test_true = np.array([9.5, 14.5, 19.5, 24.5, 29.5, 34.5, 39.5])

def dac_trung(x, bac):
    cot = [x ** k for k in range(bac, 0, -1)]
    cot.append(np.ones_like(x))
    return np.vstack(cot).T

def fit_va_danh_gia(bac):
    A_train = dac_trung(x_train, bac)
    he_so, *_ = np.linalg.lstsq(A_train, y_train, rcond=None)
    mse_train = np.mean((y_train - A_train @ he_so) ** 2)
    A_test = dac_trung(x_test, bac)
    mse_test = np.mean((y_test_true - A_test @ he_so) ** 2)
    return mse_train, mse_test

for bac in [1, 7]:
    mse_train, mse_test = fit_va_danh_gia(bac)
    print("bac", bac, ": train =", round(mse_train, 6), " test =", round(mse_test, 4))
```

```text title=readonly
bac 1 : train = 0.148527  test = 0.0002
bac 7 : train = 0.0  test = 1.7556
```

Bậc 1 (đúng dạng của quan hệ thật `y = 2 + 5x`): train MSE `0.1485` (khác
`0` vì dữ liệu train có nhiễu), test MSE chỉ `0.0002` — cực nhỏ, mô hình
tổng quát hoá RẤT tốt sang dữ liệu chưa từng thấy. Bậc 7 (tám hệ số cho
tám điểm — đủ tự do để đi qua chính xác từng điểm): train MSE gần như
tuyệt đối `0` — TỐT HƠN bậc 1 trên chính dữ liệu train. Nhưng test MSE lại
là `1.7556` — cao hơn test MSE của bậc 1 tới khoảng `8800` lần. Con số train
đẹp của bậc 7 là một ẢO GIÁC: nó không phản ánh mô hình khớp quy luật tốt
hơn, chỉ phản ánh việc nó có đủ tự do để luồn qua nhiễu của tám điểm cụ
thể đó.
::::

::::predict{#doan-mse-train-bac-8 commitOnce}
Vẫn tám điểm train ở trên. Giờ thử đa thức bậc `8` — tức CHÍN hệ số tự do
cho tám điểm dữ liệu (nhiều hệ số HƠN cả số điểm).

**Trước khi tính**, bạn đoán: train MSE của mô hình bậc `8` sẽ ra sao, so
với train MSE của mô hình bậc `7` (đã gần như tuyệt đối `0`)?

:::opt{correct}
Vẫn gần như tuyệt đối `0` — bậc `7` đã đủ hệ số để đi qua CHÍNH XÁC cả tám
điểm rồi, thêm bậc nữa không còn gì để cải thiện trên chính tập train
:::

:::opt
Sẽ ÂM — vì mô hình "quá mạnh" nên sai số phải lật dấu, thấp hơn cả `0`
::why
Gần đúng ở việc bạn cảm nhận đúng mô hình bậc `8` "mạnh" hơn hẳn bậc `7`
về số hệ số tự do — quan sát đó không sai.

Chỗ lệch: MSE là TRUNG BÌNH của các số hạng đã bị BÌNH PHƯƠNG (`sai_số²`)
— một tổng các số không âm chia cho số điểm, nên MSE không thể nào ra số
âm, bất kể mô hình mạnh cỡ nào. "Mạnh hơn" ở đây có trần: một khi đã đi
qua CHÍNH XÁC mọi điểm (sai số bằng `0` ở từng điểm), MSE đã chạm đáy `0`
— không có "thấp hơn 0" để lật xuống.
::
:::

:::opt
Sẽ tăng trở lại, cao hơn cả bậc `7` — vì thêm quá nhiều hệ số làm công thức
rối loạn, khó tính đúng
::why
Gần đúng ở trực giác "càng phức tạp càng dễ hỏng" — một lo ngại hợp lý về
mặt số học khi có QUÁ NHIỀU tham số so với dữ liệu.

Chỗ lệch: nỗi lo đó đúng cho một vấn đề KHÁC — sự ổn định số học của phép
giải (không nằm trong phạm vi bài này) — chứ không phải cho GIÁ TRỊ train
MSE. Về mặt lý thuyết, một khi mô hình đã có đủ hệ số để khớp chính xác
tuyệt đối mọi điểm train (bậc `7` đã làm được điều đó), thêm hệ số tự do
nữa không thể làm train MSE tệ hơn — nó chỉ có thể giữ nguyên ở mức tối
thiểu `0`, không có hướng nào để tăng lên.
::
:::
::::

::::code{#do_train_test_bac_1_va_7}
Hoàn thiện `fit_va_danh_gia`: tính dự đoán trên train VÀ trên test từ CÙNG
một bộ hệ số `he_so` (khớp chỉ trên train), rồi so sánh bậc `1` với bậc `7`.

```python title=starter
import numpy as np

x_train = np.array([1,2,3,4,5,6,7,8], dtype=float)
y_train = np.array([7.3, 11.6, 17.2, 21.5, 27.6, 31.8, 37.4, 41.7])
x_test = np.array([1.5,2.5,3.5,4.5,5.5,6.5,7.5])
y_test_true = np.array([9.5, 14.5, 19.5, 24.5, 29.5, 34.5, 39.5])

def dac_trung(x, bac):
    cot = [x ** k for k in range(bac, 0, -1)]
    cot.append(np.ones_like(x))
    return np.vstack(cot).T

def fit_va_danh_gia(bac):
    A_train = dac_trung(x_train, bac)
    he_so, resid, rank, sv = np.linalg.lstsq(A_train, y_train, rcond=None)
    du_doan_train = ___                 # A_train nhan he_so
    mse_train = np.mean((y_train - du_doan_train) ** 2)

    A_test = dac_trung(x_test, bac)
    du_doan_test = ___                  # A_test nhan he_so (CUNG he_so, khong khop lai)
    mse_test = np.mean((y_test_true - du_doan_test) ** 2)
    return mse_train, mse_test

mse_train_1, mse_test_1 = fit_va_danh_gia(1)
mse_train_7, mse_test_7 = fit_va_danh_gia(7)

print(round(mse_train_1, 4), round(mse_test_1, 4))
print(round(mse_train_7, 4), round(mse_test_7, 4))
```

```python title=solution
import numpy as np

x_train = np.array([1,2,3,4,5,6,7,8], dtype=float)
y_train = np.array([7.3, 11.6, 17.2, 21.5, 27.6, 31.8, 37.4, 41.7])
x_test = np.array([1.5,2.5,3.5,4.5,5.5,6.5,7.5])
y_test_true = np.array([9.5, 14.5, 19.5, 24.5, 29.5, 34.5, 39.5])

def dac_trung(x, bac):
    cot = [x ** k for k in range(bac, 0, -1)]
    cot.append(np.ones_like(x))
    return np.vstack(cot).T

def fit_va_danh_gia(bac):
    A_train = dac_trung(x_train, bac)
    he_so, resid, rank, sv = np.linalg.lstsq(A_train, y_train, rcond=None)
    du_doan_train = A_train @ he_so
    mse_train = np.mean((y_train - du_doan_train) ** 2)

    A_test = dac_trung(x_test, bac)
    du_doan_test = A_test @ he_so
    mse_test = np.mean((y_test_true - du_doan_test) ** 2)
    return mse_train, mse_test

mse_train_1, mse_test_1 = fit_va_danh_gia(1)
mse_train_7, mse_test_7 = fit_va_danh_gia(7)

print(round(mse_train_1, 4), round(mse_test_1, 4))
print(round(mse_train_7, 4), round(mse_test_7, 4))
```

```python title=test
assert round(mse_train_1, 4) == 0.1485, f"train mse bac 1 phai la 0.1485 -- dang ra {round(mse_train_1, 4)}"
assert round(mse_test_1, 4) == 0.0002, f"test mse bac 1 phai la 0.0002 -- dang ra {round(mse_test_1, 4)}"
assert round(mse_train_7, 4) == 0.0, f"train mse bac 7 phai gan bang 0 -- dang ra {round(mse_train_7, 4)}"
assert round(mse_test_7, 4) == 1.7556, f"test mse bac 7 phai la 1.7556 -- dang ra {round(mse_test_7, 4)}"
assert mse_test_7 > mse_test_1 * 1000, "test mse bac 7 phai cao hon han (hang nghin lan) test mse bac 1 -- dau hieu overfitting"
assert mse_train_7 < mse_train_1, "train mse bac 7 phai THAP HON train mse bac 1 -- bac 7 khop diem train tot hon"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều là phép NHÂN MA TRẬN với hệ số `he_so` — một để dự đoán trên `A_train`, một để dự đoán trên `A_test`. Chú ý cả hai đều dùng CHÍNH `he_so` đã khớp từ train — test KHÔNG được khớp lại, chỉ được dùng để ĐO.
- kind: strategy
  body: 'du_doan_train: `A_train @ he_so`. du_doan_test: `A_test @ he_so`.'
- kind: one-line
  body: 'Hai chỗ trống: `A_train @ he_so` và `A_test @ he_so`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: ca hai cho trong phai THAT SU nhan ma tran voi he_so (khong duoc chep du_doan bang y_train/y_test_true, vi lam vay se cho mse bang 0 gia tao ca tren train lan test)
  requireAst:
  - kind: uses-name, target: he_so, min: 2
  # Da thu that (ast.parse): "cheat" bang du_doan_train=y_train,
  # du_doan_test=y_test_true cho he_so=0 -- duoi nguong 2, bi chan. Loi giai
  # that (A_train @ he_so, A_test @ he_so) cho he_so=2, qua nguong.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^0\\.1485 0\\.0002\\n0\\.0 1\\.7556\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Train MSE gần bằng 0 nghe như một chiến thắng — cho tới khi nhìn sang test
MSE, cao gấp khoảng 8800 lần đối thủ khiêm tốn hơn của nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Overfitting xảy ra vì mô hình bậc `7` có QUÁ NHIỀU tự do so với lượng dữ
liệu — tám hệ số cho tám điểm. Một cách chống overfitting là giảm bậc (ít
hệ số hơn) — nhưng cách đó đòi phải BIẾT trước bậc nào là "vừa đủ", điều
không phải lúc nào cũng rõ ràng.

Có cách nào GIỮ NGUYÊN số hệ số (vẫn cho phép mô hình đủ linh hoạt để khớp
đường cong thật), nhưng vẫn NGĂN nó luồn qua từng hạt nhiễu một cách quá
đà? Bài sau trả lời — bằng cách phạt thẳng vào chính các hệ số, không đụng
tới bậc của mô hình.
::::

::::checkpoint{mastery=0.8}
::::
