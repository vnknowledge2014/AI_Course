---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.roc-va-auc
title: "ROC và AUC"
summary: "10 điểm tự bịa (5 dương thật, 5 âm thật) với xác suất dự đoán liên tục: tại 5 ngưỡng cụ thể [0.9,0.7,0.5,0.3,0.1], TPR chạy [0.2,0.6,0.8,1.0,1.0] và FPR chạy [0.0,0.2,0.2,0.6,1.0] — ước lượng AUC bằng hình thang (np.trapezoid) trên 5 điểm này ra 0.84; dùng lưới ngưỡng ĐẦY ĐỦ hơn (mọi điểm chia cắt tự nhiên, 12 ngưỡng) ra đúng AUC=0.8, khớp với công thức chính xác (xác suất một điểm dương ngẫu nhiên có điểm số cao hơn một điểm âm ngẫu nhiên) — ít ngưỡng hơn cho ước lượng THÔ hơn, không phải sai một cách ngẫu nhiên mà LUÔN lệch về một phía."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.roc-auc]
requires: [ai.precision-recall-f1]
concepts: [ai.roc-auc]
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
Precision và recall đều cần CHỌN một ngưỡng trước. Có cách nào đánh giá mô
hình mà không phải chốt ngưỡng nào cả?
::::

::::explain{#duong-cong-khong-can-chon-nguong}
`precision-recall-f1` tính hai tỉ lệ tại ĐÚNG MỘT ngưỡng (`0.5`). Nhưng một
mô hình phân loại thường trả về XÁC SUẤT liên tục, và `ranh-gioi-quyet-dinh`
đã chỉ ra: đổi ngưỡng thì đổi cả kết quả. Vậy: mô hình có "tốt" hay không,
xét trên MỌI ngưỡng có thể chọn, chứ không chỉ một ngưỡng cụ thể?

**Đường ROC** (Receiver Operating Characteristic) trả lời câu đó bằng cách
vẽ MỘT điểm cho MỖI ngưỡng có thể: trục ngang là **FPR** (False Positive
Rate, `FP/(FP+TN)` — trong số điểm ÂM thật, bao nhiêu bị báo động giả),
trục dọc là **TPR** (True Positive Rate — chính là `recall` đã học,
`TP/(TP+FN)`). Khi ngưỡng giảm dần từ `1` xuống `0` (mô hình càng "xông
xáo", báo động ngày càng nhiều), cả TPR lẫn FPR đều TĂNG dần (đoán dương
nhiều hơn thì bắt được nhiều positive thật hơn, NHƯNG cũng báo nhầm nhiều
negative hơn) — nối các điểm đó lại theo thứ tự ngưỡng giảm dần, được một
đường đi từ góc `(0, 0)` tới góc `(1, 1)`.

Một mô hình ĐOÁN NGẪU NHIÊN (không học được gì) cho đường ROC là đúng
đường CHÉO `y = x` — TPR tăng đúng bằng tốc độ FPR tăng. Một mô hình TỐT
kéo đường cong lên GẦN góc trên-trái (`FPR` thấp, `TPR` cao) càng nhiều
càng tốt — nghĩa là bắt được nhiều positive thật mà không phải trả giá
bằng nhiều báo động giả.

**AUC** (Area Under the Curve) là DIỆN TÍCH dưới đường ROC — một con số
DUY NHẤT gộp cả đường cong đó lại, không phụ thuộc việc chọn ngưỡng nào.
`AUC = 1.0` là phân loại hoàn hảo; `AUC = 0.5` đúng bằng đoán ngẫu nhiên
(diện tích dưới đường chéo); `AUC` càng gần `1`, mô hình càng tách được hai
lớp tốt trên MỌI ngưỡng, không chỉ một ngưỡng may mắn.

Tính chính xác diện tích dưới một đường cong bất kỳ nói chung cần tích
phân — nhưng có thể ƯỚC LƯỢNG bằng **quy tắc hình thang** (trapezoidal
rule): chia đường cong thành nhiều đoạn thẳng nhỏ nối các điểm đã tính
được, mỗi đoạn tạo một hình thang với trục ngang, cộng diện tích các hình
thang đó lại. `numpy` có sẵn `np.trapezoid(y, x)` làm đúng việc này. Càng
dùng NHIỀU điểm (nhiều ngưỡng), các đoạn thẳng càng khớp sát đường cong
thật — ước lượng càng chính xác.
::::

::::example{#tpr-fpr-tai-5-nguong}
Mười điểm tự bịa: `5` điểm dương thật, `5` điểm âm thật, mỗi điểm có một
xác suất dự đoán liên tục (không phải nhãn cứng `0`/`1`):

```python title=readonly
import numpy as np

y_that = np.array([1, 1, 1, 1, 1, 0, 0, 0, 0, 0])
xac_suat = np.array([0.9, 0.8, 0.7, 0.6, 0.3, 0.75, 0.4, 0.35, 0.2, 0.1])

def tpr_fpr(y_that, xac_suat, nguong):
    y_pred = (xac_suat >= nguong).astype(int)
    TP = np.sum((y_that == 1) & (y_pred == 1))
    FN = np.sum((y_that == 1) & (y_pred == 0))
    FP = np.sum((y_that == 0) & (y_pred == 1))
    TN = np.sum((y_that == 0) & (y_pred == 0))
    tpr = TP / (TP + FN) if (TP + FN) > 0 else 0.0
    fpr = FP / (FP + TN) if (FP + TN) > 0 else 0.0
    return tpr, fpr

nguong_list = [0.9, 0.7, 0.5, 0.3, 0.1]
tprs, fprs = [], []
for ng in nguong_list:
    tpr, fpr = tpr_fpr(y_that, xac_suat, ng)
    tprs.append(tpr)
    fprs.append(fpr)
    print("nguong=", ng, " TPR=", round(tpr, 4), " FPR=", round(fpr, 4))

tprs, fprs = np.array(tprs), np.array(fprs)
auc_5_nguong = np.trapezoid(tprs, fprs)
print("AUC uoc luong (5 nguong):", round(auc_5_nguong, 4))
```

```text title=readonly
nguong= 0.9  TPR= 0.2  FPR= 0.0
nguong= 0.7  TPR= 0.6  FPR= 0.2
nguong= 0.5  TPR= 0.8  FPR= 0.2
nguong= 0.3  TPR= 1.0  FPR= 0.6
nguong= 0.1  TPR= 1.0  FPR= 1.0
AUC uoc luong (5 nguong): 0.84
```

Khi ngưỡng giảm dần (`0.9 → 0.1`), cả TPR lẫn FPR đều tăng dần, đúng như
giải thích — mô hình càng "xông xáo" thì bắt được nhiều positive hơn NHƯNG
cũng báo nhầm nhiều negative hơn. Ước lượng AUC bằng năm điểm này ra
`0.84`. Giờ thử lại với LƯỚI ngưỡng đầy đủ hơn — mọi điểm chia cắt tự
nhiên giữa các giá trị xác suất khác nhau (`12` ngưỡng thay vì `5`):

```python title=readonly
import numpy as np

y_that = np.array([1, 1, 1, 1, 1, 0, 0, 0, 0, 0])
xac_suat = np.array([0.9, 0.8, 0.7, 0.6, 0.3, 0.75, 0.4, 0.35, 0.2, 0.1])

def tpr_fpr(y_that, xac_suat, nguong):
    y_pred = (xac_suat >= nguong).astype(int)
    TP = np.sum((y_that == 1) & (y_pred == 1))
    FN = np.sum((y_that == 1) & (y_pred == 0))
    FP = np.sum((y_that == 0) & (y_pred == 1))
    TN = np.sum((y_that == 0) & (y_pred == 0))
    tpr = TP / (TP + FN) if (TP + FN) > 0 else 0.0
    fpr = FP / (FP + TN) if (FP + TN) > 0 else 0.0
    return tpr, fpr

nguong_day_du = np.sort(np.unique(np.concatenate([[1.01], xac_suat, [0.0]])))[::-1]
tprs2, fprs2 = [], []
for ng in nguong_day_du:
    tpr, fpr = tpr_fpr(y_that, xac_suat, ng)
    tprs2.append(tpr)
    fprs2.append(fpr)
tprs2, fprs2 = np.array(tprs2), np.array(fprs2)
auc_day_du = np.trapezoid(tprs2, fprs2)
print("so nguong dung:", len(nguong_day_du))
print("AUC uoc luong (luoi day du):", round(auc_day_du, 4))
```

```text title=readonly
so nguong dung: 12
AUC uoc luong (luoi day du): 0.8
```

Dùng `12` ngưỡng thay vì `5` cho `AUC = 0.8` — khác hẳn `0.84` của ước
lượng thô. Con số `0.8` này KHỚP với công thức tính AUC chính xác (xác
suất một điểm dương ngẫu nhiên có điểm số CAO HƠN một điểm âm ngẫu nhiên,
tính trên mọi cặp `5 × 5 = 25` cặp dương-âm có thể). Ước lượng `5` ngưỡng
không sai NGẪU NHIÊN — nó LUÔN lệch về một phía (ở đây là lệch CAO hơn giá
trị thật), vì quy tắc hình thang nối các điểm THƯA bằng đường THẲNG, trong
khi đường ROC thật là một bậc thang — đường thẳng nối các bậc thang đó bỏ
sót phần diện tích lồi lõm ở giữa.
::::

::::predict{#doan-uoc-luong-2-nguong commitOnce}
Vẫn dữ liệu trên. Giờ thử ước lượng AUC bằng vỏn vẹn HAI ngưỡng — chỉ `0.9`
và `0.1` (hai đầu cực, bỏ hẳn ba ngưỡng giữa).

**Trước khi tính**, bạn đoán: ước lượng AUC với hai ngưỡng này so với ước
lượng `0.84` (năm ngưỡng) và giá trị chính xác `0.8` sẽ ra sao?

:::opt{correct}
Còn lệch XA hơn nữa — khoảng `0.6`, thấp hơn cả `0.84` lẫn `0.8` — vì càng
ít ngưỡng, các đoạn thẳng nối càng dài, càng bỏ sót nhiều phần đường cong
thật ở giữa
:::

:::opt
Chính xác hơn cả `0.84` — vì chỉ dùng hai điểm CỰC TRỊ (ngưỡng cao nhất và
thấp nhất), tránh được nhiễu từ các ngưỡng ở giữa
::why
Gần đúng ở việc hai điểm `0.9` và `0.1` đúng là hai đầu mút của lưới ngưỡng
đã dùng — không sai về việc chúng LÀ hai điểm cực trị.

Chỗ lệch: "ít điểm hơn" không đồng nghĩa "chính xác hơn" khi ước lượng
diện tích dưới một đường CONG (không phải đường thẳng). Bỏ ba ngưỡng giữa
nghĩa là bỏ luôn thông tin về HÌNH DẠNG của đường ROC ở đoạn giữa — quy
tắc hình thang buộc phải nối thẳng từ `(0, 0.2)` tới `(1, 1.0)`, một đường
thẳng dài bỏ sót toàn bộ phần lồi của đường cong thật. Ước lượng này ra
khoảng `0.6` — lệch XA hơn `0.84`, không phải gần hơn.
::
:::

:::opt
Ra đúng `0.5` — vì chỉ còn hai điểm, quy tắc hình thang không đủ dữ liệu để
tính, mặc định trả về giá trị của một mô hình đoán ngẫu nhiên
::why
Gần đúng ở trực giác "quá ít điểm thì tính toán kém tin cậy" — lo ngại đó
có cơ sở.

Chỗ lệch: quy tắc hình thang VẪN tính được với chỉ hai điểm — nó không hề
"mặc định về 0.5" khi thiếu dữ liệu, nó vẫn thật sự tính diện tích hình
thang giữa hai điểm `(0, 0.2)` và `(1, 1.0)` đã có, cho ra một con số CỤ
THỂ (khoảng `0.6`), không phải một giá trị mặc định báo hiệu "không tính
được".
::
:::
::::

::::code{#tinh_tpr_fpr_va_auc}
Hoàn thiện `tpr_fpr` (công thức TPR và FPR đã học) và ước lượng AUC bằng
quy tắc hình thang trên năm ngưỡng.

```python title=starter
import numpy as np

y_that = np.array([1, 1, 1, 1, 1, 0, 0, 0, 0, 0])
xac_suat = np.array([0.9, 0.8, 0.7, 0.6, 0.3, 0.75, 0.4, 0.35, 0.2, 0.1])

def tpr_fpr(y_that, xac_suat, nguong):
    y_pred = (xac_suat >= nguong).astype(int)
    TP = np.sum((y_that == 1) & (y_pred == 1))
    FN = np.sum((y_that == 1) & (y_pred == 0))
    FP = np.sum((y_that == 0) & (y_pred == 1))
    TN = np.sum((y_that == 0) & (y_pred == 0))
    tpr = ___ if (TP + FN) > 0 else 0.0      # TP / (TP + FN)
    fpr = ___ if (FP + TN) > 0 else 0.0      # FP / (FP + TN)
    return tpr, fpr

nguong_list = [0.9, 0.7, 0.5, 0.3, 0.1]
tprs, fprs = [], []
for ng in nguong_list:
    tpr, fpr = tpr_fpr(y_that, xac_suat, ng)
    tprs.append(tpr)
    fprs.append(fpr)
tprs, fprs = np.array(tprs), np.array(fprs)

auc = ___                                    # np.trapezoid(tprs, fprs)

print(np.round(tprs, 4).tolist())
print(np.round(fprs, 4).tolist())
print(round(auc, 4))
```

```python title=solution
import numpy as np

y_that = np.array([1, 1, 1, 1, 1, 0, 0, 0, 0, 0])
xac_suat = np.array([0.9, 0.8, 0.7, 0.6, 0.3, 0.75, 0.4, 0.35, 0.2, 0.1])

def tpr_fpr(y_that, xac_suat, nguong):
    y_pred = (xac_suat >= nguong).astype(int)
    TP = np.sum((y_that == 1) & (y_pred == 1))
    FN = np.sum((y_that == 1) & (y_pred == 0))
    FP = np.sum((y_that == 0) & (y_pred == 1))
    TN = np.sum((y_that == 0) & (y_pred == 0))
    tpr = TP / (TP + FN) if (TP + FN) > 0 else 0.0
    fpr = FP / (FP + TN) if (FP + TN) > 0 else 0.0
    return tpr, fpr

nguong_list = [0.9, 0.7, 0.5, 0.3, 0.1]
tprs, fprs = [], []
for ng in nguong_list:
    tpr, fpr = tpr_fpr(y_that, xac_suat, ng)
    tprs.append(tpr)
    fprs.append(fpr)
tprs, fprs = np.array(tprs), np.array(fprs)

auc = np.trapezoid(tprs, fprs)

print(np.round(tprs, 4).tolist())
print(np.round(fprs, 4).tolist())
print(round(auc, 4))
```

```python title=test
assert np.round(tprs, 4).tolist() == [0.2, 0.6, 0.8, 1.0, 1.0], f"tprs sai -- dang ra {np.round(tprs, 4).tolist()}"
assert np.round(fprs, 4).tolist() == [0.0, 0.2, 0.2, 0.6, 1.0], f"fprs sai -- dang ra {np.round(fprs, 4).tolist()}"
assert round(auc, 4) == 0.84, f"AUC uoc luong (5 nguong) phai la 0.84 -- dang ra {round(auc, 4)}"

# Tren du lieu that cua bai, TP+FN va FP+TN khong bao gio dung bang 0 (luon
# co ca diem duong lan diem am) -- nen khong phan biet duoc nhanh du phong
# ">0" co THAT SU chan duoc mau so 0 hay khong. Goi truc tiep tpr_fpr voi
# du lieu KHONG co diem duong nao / KHONG co diem am nao de ep di qua nhanh
# du phong that su (thieu nhanh nay se chia 0/0 ra NaN, khong phai 0.0).
tpr_khong_duong, _ = tpr_fpr(np.array([0, 0, 0]), np.array([0.8, 0.2, 0.9]), 0.5)
assert tpr_khong_duong == 0.0, f"khong co diem duong nao (TP+FN=0): tpr phai la 0.0 (nhanh du phong), dang ra {tpr_khong_duong}"
_, fpr_khong_am = tpr_fpr(np.array([1, 1, 1]), np.array([0.8, 0.2, 0.9]), 0.5)
assert fpr_khong_am == 0.0, f"khong co diem am nao (FP+TN=0): fpr phai la 0.0 (nhanh du phong), dang ra {fpr_khong_am}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Hai chỗ đầu là công thức TPR (`TP/(TP+FN)`) và FPR (`FP/(FP+TN)`) đã học — chú ý KHÔNG dùng nhầm mẫu số của nhau. Chỗ cuối gọi `np.trapezoid(y, x)` với `y=tprs` (trục dọc) và `x=fprs` (trục ngang) — đúng thứ tự tham số, không đảo ngược.
- kind: strategy
  body: 'tpr: `TP / (TP + FN)`. fpr: `FP / (FP + TN)`. auc: `np.trapezoid(tprs, fprs)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `TP / (TP + FN)`, `FP / (FP + TN)`, và `np.trapezoid(tprs, fprs)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: tpr phai chia cho (TP+FN), fpr phai chia cho (FP+TN) -- khong duoc dung nham mau so cua nhau; auc phai goi THAT np.trapezoid tren tprs/fprs -- khong duoc thay bang trung binh cong hay chep hang so 0.84
  requireAst:
  - kind: uses-name, target: TP, min: 2
  - kind: uses-name, target: FN, min: 1
  - kind: uses-name, target: FP, min: 2
  - kind: uses-name, target: TN, min: 1
  - kind: uses-call, target: trapezoid, min: 1
  # Da thu that (goi kiemAst that tren code day du):
  # - loi giai dung: dat=true, ca nam luat qua sach.
  # - tpr nham dung (FP+TN) thay vi (TP+FN): FN tut tu 1 xuong 0 (khong con
  #   xuat hien o dau) -- duoi nguong 1, bi chan; TP tut tu 2 xuong 1 (chi
  #   con o tu so) -- duoi nguong 2, bi chan ca hai.
  # - auc = np.mean(tprs) (trung binh cong thay vi hinh thang): trapezoid
  #   tut tu 1 xuong 0 -- duoi nguong 1, bi chan.
  # - auc = 0.84 (hang so chep san): trapezoid cung tut xuong 0 -- bi chan
  #   tuong tu.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\[0\\.2, 0\\.6, 0\\.8, 1\\.0, 1\\.0\\]\\n\\[0\\.0, 0\\.2, 0\\.2, 0\\.6, 1\\.0\\]\\n0\\.84\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
0.84 với năm ngưỡng, 0.8 với mười hai. Cùng dữ liệu, cùng đường cong thật —
chỉ khác việc lấy mẫu đường cong đó thô hay mịn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mọi mô hình từ đầu track tới giờ — logistic regression, softmax — đều có
một PHA HUẤN LUYỆN rõ ràng: khởi tạo `w`, `b` (hay `W`, `B`) bằng `0`, rồi
lặp gradient descent để chúng hội tụ dần về giá trị tốt. Nhưng có một cách
phân loại HOÀN TOÀN khác: không hề có `w`, `b` nào để học cả — quyết định
dựa thẳng vào chính DỮ LIỆU train, mỗi lần dự đoán một điểm mới.

Bài sau giới thiệu cách đó.
::::

::::checkpoint{mastery=0.8}
::::
