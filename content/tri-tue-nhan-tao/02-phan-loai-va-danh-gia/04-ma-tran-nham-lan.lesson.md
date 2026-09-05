---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.ma-tran-nham-lan
title: "Ma trận nhầm lẫn"
summary: "20 email (9 spam thật, 11 không spam thật) đối chiếu với dự đoán của một bộ lọc cụ thể: đếm bằng numpy (không suy luận) ra TP=7, FP=3, TN=8, FN=2 — tổng đúng bằng 20, accuracy=(7+8)/20=0.75 — bốn con số định nghĩa CHÍNH XÁC bốn tình huống khác nhau (đoán đúng spam, báo nhầm thư thường thành spam, đoán đúng thư thường, BỎ LỌT spam thật) mà một con số accuracy duy nhất gộp chung, không phân biệt được."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.ma-tran-nham-lan]
requires: [ai.softmax]
concepts: [ai.ma-tran-nham-lan]
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
"Đúng 75%" nghe ổn — cho tới khi hỏi thêm: đúng CÁI GÌ, sai CÁI GÌ? Một con
số accuracy không trả lời được câu đó.
::::

::::explain{#bon-tinh-huong}
Bài `softmax-da-lop` đo accuracy — tỉ lệ dự đoán đúng trên tổng số điểm.
Một con số duy nhất, gọn — nhưng gộp chung MỌI loại đúng/sai làm một, che
mất một sự thật quan trọng: với bài toán nhị phân, có tới BỐN tình huống
khác nhau có thể xảy ra ở mỗi điểm, không chỉ "đúng" hay "sai":

> **True Positive (TP)** — nhãn thật là `1`, mô hình đoán `1`. Đoán ĐÚNG,
> và đúng ở lớp "có" (spam thật, bắt được).
>
> **False Positive (FP)** — nhãn thật là `0`, mô hình đoán `1`. Đoán SAI,
> báo động GIẢ (thư thường, bị gắn nhầm spam).
>
> **True Negative (TN)** — nhãn thật là `0`, mô hình đoán `0`. Đoán ĐÚNG,
> đúng ở lớp "không" (thư thường, nhận đúng).
>
> **False Negative (FN)** — nhãn thật là `1`, mô hình đoán `0`. Đoán SAI,
> BỎ LỌT (spam thật, mô hình tưởng là thư thường).

Bốn con số này gọi chung là **ma trận nhầm lẫn** (confusion matrix) — một
bảng 2×2 đếm số điểm rơi vào MỖI trong bốn ô. `accuracy = (TP + TN) / n` —
chỉ là MỘT cách tổng hợp bốn con số đó thành một tỉ lệ duy nhất, và như
tên gọi "tổng hợp" đã ngầm nói, nó ĐÁNH MẤT thông tin về việc sai ở đâu:
hai mô hình có CÙNG accuracy có thể sai theo hai kiểu HOÀN TOÀN khác nhau
— một mô hình toàn báo động giả (FP cao), một mô hình toàn bỏ lọt (FN
cao) — và với nhiều bài toán thực tế (bỏ lọt một ca ung thư nguy hiểm hơn
nhiều so với báo động giả một ca lành), sự khác biệt đó QUAN TRỌNG hơn hẳn
con số accuracy gộp chung.

Đếm bốn ô này bằng `numpy` không cần vòng lặp: với `y_that` và `y_du_doan`
là hai mảng nhãn `0`/`1`, mỗi ô là một phép AND giữa hai điều kiện boolean:

> `TP = Σ[(y_that == 1) & (y_du_doan == 1)]`

và tương tự cho ba ô còn lại, chỉ đổi vế nào so với `1` hay `0`.
::::

::::example{#dem_bon_o_tren_20_email}
Hai mươi email tự bịa: nhãn thật (`y_that`) và dự đoán của một bộ lọc cụ
thể (`y_du_doan`), đếm cả bốn ô bằng `numpy`:

```python title=readonly
import numpy as np

y_that = np.array([1,0,1,1,0,0,1,0,1,0,1,0,0,1,0,0,1,0,1,0])
y_du_doan = np.array([1,0,0,1,1,0,1,0,1,0,0,0,1,1,0,0,1,1,1,0])
n = len(y_that)

TP = int(np.sum((y_that == 1) & (y_du_doan == 1)))
FP = int(np.sum((y_that == 0) & (y_du_doan == 1)))
TN = int(np.sum((y_that == 0) & (y_du_doan == 0)))
FN = int(np.sum((y_that == 1) & (y_du_doan == 0)))

print("TP =", TP, " FP =", FP, " TN =", TN, " FN =", FN)
print("tong bon o:", TP + FP + TN + FN, " so email:", n)
print("accuracy =", (TP + TN) / n)
```

```text title=readonly
TP = 7  FP = 3  TN = 8  FN = 2
tong bon o: 20  so email: 20
accuracy = 0.75
```

Tổng bốn ô đúng bằng `20` — đúng tổng số email, không thiếu không thừa: mỗi
email rơi vào ĐÚNG MỘT trong bốn ô, không hơn không kém. `TP=7`: bảy email
spam thật được bắt đúng. `FN=2`: hai email spam thật bị BỎ LỌT — nguy hiểm
hơn `FP=3` (ba email thường bị báo nhầm) tuỳ vào bài toán cụ thể, nhưng
`accuracy = 0.75` một mình KHÔNG hề nói cho biết tỉ lệ `7:3:8:2` này — nó
chỉ gộp `(7+8)/20`.
::::

::::predict{#doan-y-nghia-fn commitOnce}
Nhìn công thức `FN = Σ[(y_that == 1) & (y_du_doan == 0)]` — kết quả tính
được là `FN = 2`.

**Trước khi đọc lại**, bạn đoán: hai email rơi vào ô `FN` này có đặc điểm
gì?

:::opt{correct}
Cả hai đều LÀ spam thật (`y_that = 1`), nhưng bị mô hình dự đoán NHẦM là
không spam (`y_du_doan = 0`) — hai ca "bỏ lọt"
:::

:::opt
Cả hai đều KHÔNG phải spam (`y_that = 0`), nhưng bị mô hình dự đoán nhầm
là spam (`y_du_doan = 1`) — hai ca "báo động giả"
::why
Gần đúng ở việc bạn mô tả ĐÚNG một loại lỗi có thật trong bài — "báo động
giả" đúng là một tình huống xảy ra trong bảng này.

Chỗ lệch: tình huống bạn mô tả (`y_that=0`, `y_du_doan=1`) là định nghĩa
của `FP` (đã tính ra `3`, không phải `2`), không phải `FN`. Nhìn kỹ công
thức `FN` — vế đầu là `y_that == 1`, nghĩa là nhãn THẬT phải là `1` (spam
thật), không phải `0`. `FN` và `FP` là hai ô ĐỐI XỨNG nhưng khác hẳn nhau:
một ô nói về spam thật bị bỏ lọt, ô kia nói về thư thường bị báo nhầm.
::
:::

:::opt
Cả hai đều được mô hình dự đoán ĐÚNG (`y_du_doan` trùng `y_that`), chỉ là
thuộc lớp `0`
::why
Gần đúng ở việc bạn nhớ đúng: có NHỮNG ô trong bảng đại diện cho dự đoán
ĐÚNG — đó là `TP` và `TN`, không sai về sự tồn tại của khái niệm "đúng".

Chỗ lệch: `FN` — chữ `F` đứng đầu là viết tắt của "False", nghĩa là dự
đoán SAI, không phải đúng. Công thức `(y_that == 1) & (y_du_doan == 0)`
đòi hỏi nhãn thật và dự đoán KHÁC NHAU (`1` so với `0`) — đúng định nghĩa
của một lần đoán SAI, cụ thể là bỏ lọt một ca dương tính thật.
::
:::
::::

::::code{#tinh_bon_o_ma_tran_nham_lan}
Hoàn thiện việc đếm ba ô còn lại của ma trận nhầm lẫn (`FP`, `TN`, `FN` —
`TP` đã có sẵn làm ví dụ), rồi tính `accuracy` từ chính bốn ô đó.

```python title=starter
import numpy as np

y_that = np.array([1,0,1,1,0,0,1,0,1,0,1,0,0,1,0,0,1,0,1,0])
y_du_doan = np.array([1,0,0,1,1,0,1,0,1,0,0,0,1,1,0,0,1,1,1,0])
n = len(y_that)

TP = int(np.sum((y_that == 1) & (y_du_doan == 1)))
FP = ___                        # y_that == 0 va y_du_doan == 1
TN = ___                        # y_that == 0 va y_du_doan == 0
FN = ___                        # y_that == 1 va y_du_doan == 0

accuracy = (TP + TN) / n

print(TP, FP, TN, FN)
print(round(accuracy, 4))
```

```python title=solution
import numpy as np

y_that = np.array([1,0,1,1,0,0,1,0,1,0,1,0,0,1,0,0,1,0,1,0])
y_du_doan = np.array([1,0,0,1,1,0,1,0,1,0,0,0,1,1,0,0,1,1,1,0])
n = len(y_that)

TP = int(np.sum((y_that == 1) & (y_du_doan == 1)))
FP = int(np.sum((y_that == 0) & (y_du_doan == 1)))
TN = int(np.sum((y_that == 0) & (y_du_doan == 0)))
FN = int(np.sum((y_that == 1) & (y_du_doan == 0)))

accuracy = (TP + TN) / n

print(TP, FP, TN, FN)
print(round(accuracy, 4))
```

```python title=test
assert TP == 7, f"TP phai la 7 -- dang ra {TP}"
assert FP == 3, f"FP phai la 3 -- dang ra {FP}"
assert TN == 8, f"TN phai la 8 -- dang ra {TN}"
assert FN == 2, f"FN phai la 2 -- dang ra {FN}"
assert TP + FP + TN + FN == n, "tong bon o phai dung bang tong so email -- moi email chi thuoc DUNG MOT o"
assert round(accuracy, 4) == 0.75, f"accuracy phai la 0.75 -- dang ra {round(accuracy, 4)}"
```

:::hints
- kind: attention
  body: Ba chỗ trống đều có hình dạng GIỐNG `TP` ở trên — `int(np.sum((y_that == ...) & (y_du_doan == ...)))` — chỉ khác việc so với `0` hay `1` ở MỖI vế. Bám sát đúng định nghĩa: `FP` là nhãn thật `0` nhưng đoán `1`; `TN` là cả hai đều `0`; `FN` là nhãn thật `1` nhưng đoán `0`.
- kind: strategy
  body: 'FP: `int(np.sum((y_that == 0) & (y_du_doan == 1)))`. TN: `int(np.sum((y_that == 0) & (y_du_doan == 0)))`. FN: `int(np.sum((y_that == 1) & (y_du_doan == 0)))`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là các phép đếm cho FP (`0`,`1`), TN (`0`,`0`), và FN (`1`,`0`) trên cặp `(y_that, y_du_doan)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: ca ba cho trong phai THAT SU dem tu y_that/y_du_doan bang phep so sanh == va AND (&) -- khong duoc chep san con so 3/8/2, vi bai nay dang day CACH DEM, khong day hoc thuoc dap an
  requireAst:
  - kind: uses-name, target: y_that, min: 4
  - kind: uses-name, target: y_du_doan, min: 4
  - kind: uses-operator, target: "==", min: 8
  # Da thu that (goi kiemAst that tren code day du): chep hang so FP=3/TN=8/
  # FN=2 (khong tinh toan gi) cho y_that=1 (chi con o dong TP), y_du_doan=1,
  # "=="=2 -- duoi ca ba nguong (can 4/4/8), bi chan. Loi giai that (bon dong
  # cung hinh dang) cho dung y_that=4, y_du_doan=4, "=="=8, qua sach ca ba.
  # Rieng kich ban DOI CHO dieu kien (vd FN dung dung cong thuc cua FP) khong
  # bi static chan -- nhung se bi tier tests chan qua gia tri sai (FN=3 thay
  # vi 2), dung phan cong: static chan hang-so-hoa, test chan cong-thuc-sai.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^7 3 8 2\\n0\\.75\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
TP=7, FP=3, TN=8, FN=2 — bốn con số, không phải một. Accuracy chỉ là một
lát cắt của bức tranh này, và không phải lát cắt quan trọng nhất mọi lúc.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bốn ô của ma trận nhầm lẫn đã đếm xong. Nhưng tự bản thân bốn con số thô
(`7, 3, 8, 2`) vẫn chưa trả lời trực tiếp những câu hỏi thực tế hay gặp:
"trong số email bị gắn spam, bao nhiêu phần trăm thật sự LÀ spam?" hay
"trong số email spam thật, mô hình bắt được bao nhiêu phần trăm?"

Hai câu hỏi đó cần hai TỈ LỆ khác nhau, tính từ chính bốn ô này. Bài sau
đặt tên và tính chúng — rồi cho thấy một tình huống mà accuracy cao ngất
nhưng hai tỉ lệ đó tố cáo một mô hình gần như vô dụng.
::::

::::checkpoint{mastery=0.8}
::::
