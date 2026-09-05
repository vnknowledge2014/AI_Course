---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.precision-recall-f1
title: "Precision, Recall, F1"
summary: "100 điểm, lệch lớp mạnh (95 âm thật, chỉ 5 dương thật): mô hình 'luôn đoán 0' đạt accuracy=0.95 (cao ngất) nhưng precision=0.0, recall=0.0, F1=0.0 — bắt được đúng 0 trong 5 ca dương thật. Một mô hình khác, accuracy THẤP HƠN (0.93, vì có 6 báo động giả) nhưng precision=0.4, recall=0.8, F1=0.5333 — bắt được 4/5 ca dương thật, hữu dụng hơn hẳn dù accuracy thấp hơn: accuracy một mình đã NÓI DỐI về mô hình nào tốt hơn."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.precision-recall-f1]
requires: [ai.ma-tran-nham-lan]
concepts: [ai.precision-recall-f1]
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
Một mô hình đạt 95% accuracy. Trước khi khen nó — hỏi thêm một câu: 95% của
CÁI GÌ?
::::

::::explain{#khi-accuracy-noi-doi}
`ma-tran-nham-lan` đã đếm bốn ô: `TP`, `FP`, `TN`, `FN`. Từ bốn ô đó, hai tỉ
lệ trả lời hai câu hỏi CỤ THỂ mà accuracy gộp chung không trả lời được:

> **Precision** — `TP / (TP + FP)` — trong số các điểm mô hình ĐOÁN LÀ
> dương, bao nhiêu phần trăm đoán ĐÚNG? (Trả lời: "khi mô hình báo động,
> có đáng tin không?")
>
> **Recall** — `TP / (TP + FN)` — trong số các điểm THẬT SỰ dương, mô hình
> BẮT ĐƯỢC bao nhiêu phần trăm? (Trả lời: "trong số ca cần bắt, bỏ lọt bao
> nhiêu?")

Hai tỉ lệ này thường ĐÁNH ĐỔI nhau: một mô hình càng "rụt rè" (chỉ báo
động khi cực kỳ chắc chắn) thường có precision cao nhưng recall thấp (bỏ
lọt nhiều); một mô hình càng "xông xáo" (báo động cả khi hơi nghi ngờ)
thường có recall cao nhưng precision thấp (báo động giả nhiều). Cần MỘT
con số gộp cả hai lại để so sánh mô hình dễ hơn — nhưng gộp bằng TRUNG BÌNH
CỘNG thông thường có một lỗ hổng: một mô hình precision `1.0`, recall `0.0`
(hoàn toàn vô dụng) sẽ có trung bình cộng `0.5` — nghe không tệ, dù mô hình
đó thực chất chẳng bắt được gì. **F1-score** dùng **trung bình điều hoà**
(harmonic mean) thay vì trung bình cộng, để tránh đúng lỗ hổng đó:

> `F1 = 2 · (precision · recall) / (precision + recall)`

Trung bình điều hoà có tính chất: nếu MỘT trong hai số (`precision` hay
`recall`) rất thấp, F1 cũng bị kéo THẤP theo, dù số còn lại có cao tới đâu
— khác trung bình cộng, vốn "cứu vớt" một số thấp bằng một số cao. Ví dụ
trên vừa nêu (`precision=1.0, recall=0.0`) cho `F1 = 2·(1.0·0.0)/(1.0+0.0)
= 0`, đúng phản ánh mô hình vô dụng — không phải `0.5` như trung bình cộng
ngộ nhận.

Cả `precision` và `recall` đều có mẫu số có thể bằng `0` (nếu mô hình
KHÔNG BAO GIỜ đoán dương, `TP+FP=0`; nếu dữ liệu không có điểm dương nào,
`TP+FN=0`) — quy ước chuẩn là gán `0.0` cho trường hợp chia cho `0` này,
thay vì để chương trình vỡ.
::::

::::example{#accuracy-noi-doi-tren-du-lieu-lech-lop}
Một trăm điểm, LỆCH LỚP mạnh: chỉ `5` điểm dương thật (`5%`), `95` điểm âm
thật (`95%`). Hai mô hình: một mô hình "lười" luôn đoán `0`, một mô hình
"thực" bắt được phần lớn ca dương nhưng có vài báo động giả:

```python title=readonly
import numpy as np

n = 100
duong_that_idx = np.array([5, 25, 45, 65, 85])
y_that = np.zeros(n, dtype=int)
y_that[duong_that_idx] = 1

y_luoi = np.zeros(n, dtype=int)                 # luon doan 0, khong doan gi ca

y_thuc = np.zeros(n, dtype=int)
y_thuc[[5, 25, 45, 65]] = 1                     # bat dung 4/5 ca duong that
y_thuc[[10, 20, 30, 40, 50, 60]] = 1            # 6 bao dong gia

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP):
    return TP / (TP + FP) if (TP + FP) > 0 else 0.0

def recall(TP, FN):
    return TP / (TP + FN) if (TP + FN) > 0 else 0.0

def f1(prec, rec):
    return 2 * prec * rec / (prec + rec) if (prec + rec) > 0 else 0.0

for ten, y_pred in [("luoi (luon doan 0)", y_luoi), ("thuc", y_thuc)]:
    TP, FP, TN, FN = conf(y_that, y_pred)
    acc = (TP + TN) / n
    p, r = precision(TP, FP), recall(TP, FN)
    print(ten, ": TP,FP,TN,FN=", (TP, FP, TN, FN), " acc=", round(acc, 4),
          " precision=", round(p, 4), " recall=", round(r, 4), " F1=", round(f1(p, r), 4))
```

```text title=readonly
luoi (luon doan 0) : TP,FP,TN,FN= (0, 0, 95, 5)  acc= 0.95  precision= 0.0  recall= 0.0  F1= 0.0
thuc : TP,FP,TN,FN= (4, 6, 89, 1)  acc= 0.93  precision= 0.4  recall= 0.8  F1= 0.5333
```

Mô hình "lười" — KHÔNG hề nhìn dữ liệu, luôn đoán `0` — đạt accuracy
`0.95`, cao HƠN mô hình "thực" (`0.93`). Nhưng precision và recall của nó
đều là `0.0` — nó không bắt được đúng MỘT ca dương nào trong số năm ca có
thật (`recall = 0/5 = 0`), và bất kỳ ai tin vào accuracy một mình sẽ nghĩ
đây là mô hình TỐT HƠN. Mô hình "thực" tuy accuracy thấp hơn nhưng bắt được
`4/5` ca dương (`recall = 0.8`), và trong số các lần nó báo động, `40%` là
đúng (`precision = 0.4`) — `F1 = 0.5333`, thấp nhưng phản ánh một mô hình
CÓ LÀM VIỆC GÌ ĐÓ, khác hẳn `F1 = 0.0` của mô hình lười. Đây chính là cách
accuracy "nói dối" trên dữ liệu lệch lớp: một mô hình không làm gì cả vẫn
có thể có accuracy đẹp, chỉ vì lớp thiểu số quá hiếm.
::::

::::predict{#doan-so-ca-bat-duoc commitOnce}
Vẫn hai mô hình trên. Mô hình "lười" có `recall = 0.0`.

**Trước khi đọc lại bảng**, bạn đoán: trong số `5` điểm dương thật, mô
hình "lười" đã BẮT ĐÚNG bao nhiêu điểm?

:::opt{correct}
Đúng `0` điểm — `recall = 0.0` nghĩa là KHÔNG bắt được ca dương thật nào,
dù accuracy của nó vẫn cao vì lớp âm chiếm áp đảo
:::

:::opt
Khoảng `4` hoặc `5` điểm — vì accuracy của nó rất cao (`0.95`), phần lớn dự
đoán phải đúng, kể cả với lớp dương
::why
Gần đúng ở việc `accuracy = 0.95` đúng là một con số CAO — không sai về
mặt số học.

Chỗ lệch: accuracy cao ở đây đến từ việc mô hình đoán ĐÚNG lớp `0` (95 điểm
âm, đoán đúng cả `95`) — hoàn toàn KHÔNG liên quan tới việc nó có bắt được
điểm dương nào không. Mô hình "lười" đoán `0` cho MỌI điểm, kể cả năm điểm
dương thật — nên với cả năm điểm đó, nó đoán SAI hoàn toàn. Bắt đúng `0`
trong `5`, không phải `4` hay `5`.
::
:::

:::opt
Không xác định được nếu chỉ biết accuracy và recall — cần thêm thông tin
khác
::why
Gần đúng ở trực giác cẩn trọng "một con số không luôn đủ để suy ra mọi
thứ" — một thái độ hợp lý nói chung khi đọc số liệu.

Chỗ lệch: trong trường hợp NÀY, thông tin đã đủ. Định nghĩa của recall là
`TP / (TP + FN)`, và `TP + FN` LUÔN đúng bằng tổng số điểm dương THẬT (ở
đây là `5`, cố định, không đổi theo mô hình). `recall = 0.0` cho `TP/5 =
0`, chỉ có một nghiệm: `TP = 0`. Không cần thêm thông tin nào khác.
::
:::
::::

::::code{#tinh_precision_recall_f1}
Viết nốt `precision`, `recall`, `f1` (đều có nhánh chia-cho-`0` an toàn),
rồi so sánh mô hình "lười" với mô hình "thực" trên cùng một trăm điểm.

```python title=starter
import numpy as np

n = 100
duong_that_idx = np.array([5, 25, 45, 65, 85])
y_that = np.zeros(n, dtype=int)
y_that[duong_that_idx] = 1

y_luoi = np.zeros(n, dtype=int)

y_thuc = np.zeros(n, dtype=int)
y_thuc[[5, 25, 45, 65]] = 1
y_thuc[[10, 20, 30, 40, 50, 60]] = 1

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP):
    return ___ if (TP + FP) > 0 else 0.0      # TP / (TP + FP)

def recall(TP, FN):
    return ___ if (TP + FN) > 0 else 0.0       # TP / (TP + FN)

def f1(prec, rec):
    return ___ if (prec + rec) > 0 else 0.0    # 2 * prec * rec / (prec + rec)

TP1, FP1, TN1, FN1 = conf(y_that, y_luoi)
p1, r1 = precision(TP1, FP1), recall(TP1, FN1)

TP2, FP2, TN2, FN2 = conf(y_that, y_thuc)
p2, r2 = precision(TP2, FP2), recall(TP2, FN2)

print(round(p1, 4), round(r1, 4), round(f1(p1, r1), 4))
print(round(p2, 4), round(r2, 4), round(f1(p2, r2), 4))
```

```python title=solution
import numpy as np

n = 100
duong_that_idx = np.array([5, 25, 45, 65, 85])
y_that = np.zeros(n, dtype=int)
y_that[duong_that_idx] = 1

y_luoi = np.zeros(n, dtype=int)

y_thuc = np.zeros(n, dtype=int)
y_thuc[[5, 25, 45, 65]] = 1
y_thuc[[10, 20, 30, 40, 50, 60]] = 1

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP):
    return TP / (TP + FP) if (TP + FP) > 0 else 0.0

def recall(TP, FN):
    return TP / (TP + FN) if (TP + FN) > 0 else 0.0

def f1(prec, rec):
    return 2 * prec * rec / (prec + rec) if (prec + rec) > 0 else 0.0

TP1, FP1, TN1, FN1 = conf(y_that, y_luoi)
p1, r1 = precision(TP1, FP1), recall(TP1, FN1)

TP2, FP2, TN2, FN2 = conf(y_that, y_thuc)
p2, r2 = precision(TP2, FP2), recall(TP2, FN2)

print(round(p1, 4), round(r1, 4), round(f1(p1, r1), 4))
print(round(p2, 4), round(r2, 4), round(f1(p2, r2), 4))
```

```python title=test
assert (p1, r1) == (0.0, 0.0), f"mo hinh luoi phai co precision=0.0, recall=0.0 -- dang ra {(p1, r1)}"
assert round(f1(p1, r1), 4) == 0.0, "F1 cua mo hinh luoi phai la 0.0"
assert round(p2, 4) == 0.4, f"precision cua mo hinh thuc phai la 0.4 -- dang ra {round(p2, 4)}"
assert round(r2, 4) == 0.8, f"recall cua mo hinh thuc phai la 0.8 -- dang ra {round(r2, 4)}"
assert round(f1(p2, r2), 4) == 0.5333, f"F1 cua mo hinh thuc phai la 0.5333 -- dang ra {round(f1(p2, r2), 4)}"
acc_luoi = (TP1 + TN1) / n
acc_thuc = (TP2 + TN2) / n
assert acc_luoi > acc_thuc, "accuracy cua mo hinh luoi phai CAO HON accuracy cua mo hinh thuc -- dung trong tam bai: accuracy noi doi"
assert f1(p2, r2) > f1(p1, r1), "nhung F1 cua mo hinh thuc phai CAO HON han F1 cua mo hinh luoi -- F1 phan anh dung mo hinh nao huu dung"

# Tren du lieu that cua bai, TP+FP/TP+FN/prec+rec khong bao gio dung bang 1
# -- nen khong phan biet duoc nguong bao ve ">0" voi ">1". Goi truc tiep ba
# ham voi mau so DUNG bang 1 de ep di qua nhanh chia that su.
assert precision(1, 0) == 1.0, f"precision(1,0): mau so TP+FP=1 -- phai chia ra 1.0, dang ra {precision(1, 0)}"
assert recall(1, 0) == 1.0, f"recall(1,0): mau so TP+FN=1 -- phai chia ra 1.0, dang ra {recall(1, 0)}"
assert f1(0.5, 0.5) == 0.5, f"f1(0.5,0.5): mau so prec+rec=1 -- phai chia ra 0.5, dang ra {f1(0.5, 0.5)}"
```

:::hints
- kind: attention
  body: Ba chỗ trống là ba công thức đã học — `precision = TP/(TP+FP)`, `recall = TP/(TP+FN)`, `f1 = 2·prec·rec/(prec+rec)` — mỗi hàm đã có sẵn nhánh `if ... else 0.0` để tránh chia cho `0`, chỉ cần điền đúng phép chia ở nhánh còn lại.
- kind: strategy
  body: 'precision: `TP / (TP + FP)`. recall: `TP / (TP + FN)`. f1: `2 * prec * rec / (prec + rec)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `TP / (TP + FP)`, `TP / (TP + FN)`, và `2 * prec * rec / (prec + rec)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: precision phai chia cho (TP+FP), recall phai chia cho (TP+FN) -- KHONG duoc dung nham mau so cua nhau, va f1 phai la trung binh DIEU HOA (2*prec*rec/(prec+rec)), khong phai trung binh cong -- ca ba deu phai tinh THAT, khong chep hang so
  requireAst:
  - kind: uses-name, target: TP, min: 6
  - kind: uses-name, target: FP, min: 3
  - kind: uses-name, target: FN, min: 3
  - kind: uses-name, target: prec, min: 3
  - kind: uses-name, target: rec, min: 3
  # Da thu that (goi kiemAst that tren code day du):
  # - loi giai dung: dat=true, ca nam luat qua sach.
  # - recall nham dung (TP+FP) thay vi (TP+FN) (nhu precision): FN tut tu 3
  #   xuong 0 -- duoi nguong 3, bi chan.
  # - f1 = (prec+rec)/2 (trung binh CONG thay vi DIEU HOA): prec va rec deu
  #   tut tu 3 xuong 2 (mat mot lan dung moi ten trong cong thuc dieu hoa
  #   that) -- duoi nguong 3, bi chan ca hai.
  # - hardcode precision=0.4 / recall=0.8 (hang so, khong tinh tu TP/FP/FN):
  #   TP tut xuong 4, FP tut xuong 1, FN tut xuong 1 -- duoi ca ba nguong
  #   (can 6/3/3), bi chan.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^0\\.0 0\\.0 0\\.0\\n0\\.4 0\\.8 0\\.5333\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Accuracy 0.95 so với 0.93 — mô hình lười trông "tốt hơn". F1 0.0 so với
0.5333 — sự thật ngược lại hoàn toàn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Precision và recall đều được tính tại MỘT ngưỡng cố định — ngưỡng `0.5`
quen thuộc từ bài `ranh-gioi-quyet-dinh`. Nhưng nếu mô hình trả về XÁC SUẤT
(không phải nhãn `0`/`1` cứng), đổi ngưỡng sẽ đổi CẢ precision LẪN recall —
giống hệt cách đổi ngưỡng đã dịch chuyển ranh giới quyết định ở bài đó.

Có cách nào nhìn được TOÀN BỘ sự đánh đổi giữa hai tỉ lệ này, qua MỌI
ngưỡng có thể, chỉ bằng MỘT con số hay MỘT đường cong duy nhất — thay vì
phải chọn một ngưỡng cụ thể rồi tính riêng lẻ? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
