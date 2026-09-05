---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.feature-importance
title: "Feature importance: permutation"
summary: "Hồi quy tuyến tính ba biến (x1 thang đo lớn [0,50] thật sự quan trọng, x2 thang đo cực nhỏ [0,0.02] KHÔNG quan trọng, x3 thang đo vừa [0,10] quan trọng vừa phải), hệ số thô w=[0.8094, -66.7726, 0.3028] — nhìn |w| thô, x2 trông NHƯ quan trọng nhất (66.7726, lớn hơn hẳn). Permutation importance (xáo trộn từng cột trên test, đo test MSE tăng bao nhiêu): x1 tăng 236.0749 (áp đảo), x3 tăng 0.883, x2 chỉ tăng 0.1132 — permutation vạch trần: x2 gần như vô dụng, đúng với cách nó được tạo ra (hệ số thật bằng 0), còn |w| thô chỉ phản ánh thang đo cực nhỏ của x2, không phản ánh mức độ quan trọng thật."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.feature-importance]
requires: [ai.on-dinh-so-hoc]
concepts: [ai.feature-importance]
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
Một mô hình đã khớp xong, ba hệ số `w`. Câu hỏi tự nhiên: cột nào QUAN
TRỌNG nhất? Nhìn con số lớn nhất trong `w` có phải câu trả lời đúng không?
::::

::::explain{#nhin-he-so-tho-co-du-tin}
`chuan-hoa-dac-trung` (track hồi quy) đã chỉ ra: các đặc trưng có thang đo
khác nhau khiến gradient descent hội tụ khó khăn khác nhau. Bài này chỉ ra
một hậu quả KHÁC của cùng vấn đề: nhìn TRỰC TIẾP vào độ lớn của hệ số `w`
(chưa chuẩn hoá) để đoán đặc trưng nào "quan trọng" có thể ĐÁNH LỪA hoàn
toàn.

Lý do: để giải thích CÙNG một lượng biến thiên trong `y`, một đặc trưng có
thang đo NHỎ (giá trị chỉ trải trong một khoảng bé) cần một hệ số LỚN; một
đặc trưng có thang đo LỚN chỉ cần một hệ số NHỎ. Hệ số lớn có thể chỉ là
dấu hiệu của "đặc trưng này có thang đo bé", không phải "đặc trưng này ảnh
hưởng mạnh tới `y`".

**Permutation importance** đo mức độ quan trọng theo một cách KHÔNG phụ
thuộc thang đo: với một mô hình ĐÃ khớp xong (hệ số `w`, `b` cố định), lấy
một tập dữ liệu (thường là test, chưa dùng để huấn luyện):

1. Đo sai số (MSE) của mô hình trên dữ liệu đó — gọi là **MSE gốc**.
2. Với MỘT cột đặc trưng, XÁO TRỘN NGẪU NHIÊN thứ tự các giá trị của CHÍNH
   cột đó (các cột KHÁC giữ nguyên) — hành động này PHÁ VỠ mối liên hệ giữa
   cột đó và nhãn thật, trong khi vẫn giữ nguyên PHÂN PHỐI giá trị của
   riêng cột đó (không đổi trung bình, không đổi độ lệch chuẩn).
3. Đo lại MSE trên dữ liệu đã xáo trộn — MỨC TĂNG so với MSE gốc là thước
   đo mức độ quan trọng của cột đó: nếu xáo trộn một cột làm MSE tăng RẤT
   NHIỀU, mô hình đang dựa dẫm nặng vào đúng cột đó; nếu MSE gần như không
   đổi, cột đó gần như không đóng góp gì cho dự đoán, bất kể hệ số `w`
   tương ứng của nó lớn hay nhỏ.

Lặp lại bước `2`–`3` cho MỖI cột, riêng biệt (mỗi lần chỉ xáo trộn ĐÚNG một
cột), rồi so sánh mức tăng MSE giữa các cột — cột có mức tăng LỚN NHẤT là
cột quan trọng nhất.
::::

::::example{#w-tho-noi-doi-permutation-noi-that}
Hồi quy tuyến tính ba biến: `x1` (thang đo `[0, 50]`, thật sự có ảnh hưởng
tới `y`), `x2` (thang đo `[0, 0.02]` — CỰC NHỎ, KHÔNG hề có ảnh hưởng thật
tới `y`), `x3` (thang đo `[0, 10]`, ảnh hưởng vừa phải):

```python title=readonly
import numpy as np

rng_train = np.random.default_rng(5)
n = 40
x1 = rng_train.uniform(0, 50, n)
x2 = rng_train.uniform(0, 0.02, n)
x3 = rng_train.uniform(0, 10, n)
y = 5 + 0.8*x1 + 0.0*x2 + 0.3*x3 + rng_train.normal(0, 2.0, n)

X = np.vstack([x1, x2, x3]).T
Xb = np.hstack([X, np.ones((n, 1))])
he_so, *_ = np.linalg.lstsq(Xb, y, rcond=None)
w = he_so[:-1]
b = he_so[-1]
print("w (he so THO, chua chuan hoa):", np.round(w, 4).tolist())

rng_test = np.random.default_rng(22)
n_te = 30
x1_te = rng_test.uniform(0, 50, n_te)
x2_te = rng_test.uniform(0, 0.02, n_te)
x3_te = rng_test.uniform(0, 10, n_te)
y_te = 5 + 0.8*x1_te + 0.0*x2_te + 0.3*x3_te + rng_test.normal(0, 2.0, n_te)
X_te = np.vstack([x1_te, x2_te, x3_te]).T

def mse(X_, y_, w_, b_):
    return np.mean((y_ - (X_ @ w_ + b_)) ** 2)

mse_goc = mse(X_te, y_te, w, b)
print("mse goc (test):", round(mse_goc, 4))

rng_perm = np.random.default_rng(1)
for i, ten in enumerate(["x1", "x2", "x3"]):
    X_perm = X_te.copy()
    X_perm[:, i] = rng_perm.permutation(X_perm[:, i])
    mse_perm = mse(X_perm, y_te, w, b)
    print(f"xao tron {ten}: mse={round(mse_perm,4)} tang={round(mse_perm - mse_goc, 4)}")
```

```text title=readonly
w (he so THO, chua chuan hoa): [0.8094, -66.7726, 0.3028]
mse goc (test): 2.7364
xao tron x1: mse=238.8113 tang=236.0749
xao tron x2: mse=2.8496 tang=0.1132
xao tron x3: mse=3.6194 tang=0.883
```

Nhìn `w` thô: `x2` có hệ số `-66.7726` — trị tuyệt đối LỚN HƠN HẲN cả `x1`
(`0.8094`) lẫn `x3` (`0.3028`). Ai chỉ nhìn con số này sẽ kết luận `x2` là
đặc trưng quan trọng NHẤT. Nhưng `x2` được tạo ra với hệ số THẬT bằng
`0.0` — nó KHÔNG hề ảnh hưởng tới `y`. Hệ số `-66.7726` chỉ là hậu quả của
thang đo CỰC NHỎ (`x2` chỉ trải từ `0` tới `0.02`) — cần một hệ số RẤT LỚN
để tạo ra bất kỳ đóng góp nào, kể cả đóng góp từ nhiễu ngẫu nhiên trong dữ
liệu train.

Permutation importance nói một câu chuyện HOÀN TOÀN khác: xáo trộn `x1`
làm MSE tăng `236.0749` — áp đảo hẳn hai cột còn lại. Xáo trộn `x2` chỉ làm
MSE tăng `0.1132` — gần như không đáng kể. `x1` — không phải `x2` — mới là
đặc trưng mô hình thật sự dựa vào để dự đoán.
::::

::::predict{#doan-neu-xao-tron-x3 commitOnce}
Nhìn lại ba mức tăng MSE: `x1` là `236.0749`, `x2` là `0.1132`, `x3` là
`0.883`.

**Trước khi đọc lại**, bạn đoán: nếu chỉ so sánh `x2` với `x3` (bỏ qua
`x1`), permutation importance có đồng ý với thứ tự mà `|w|` thô gợi ý
không? (`|w|` thô: `x2` là `66.7726`, `x3` là `0.3028` — `|w|` thô nói `x2`
quan trọng hơn `x3` nhiều lần)

:::opt{correct}
Không — permutation importance đảo ngược hoàn toàn thứ tự đó: `x3` (tăng
`0.883`) quan trọng hơn `x2` (tăng `0.1132`) khoảng `7-8` lần, dù `|w|` thô
của `x2` lớn hơn `|w|` thô của `x3` hơn `200` lần
:::

:::opt
Có — cả hai cách đo đều phải đồng ý về THỨ TỰ tương đối giữa các cột, chỉ
khác nhau về ĐỘ LỚN tuyệt đối của con số
::why
Gần đúng ở việc bạn kỳ vọng "hai cách đo cùng một khái niệm phải cho ra kết
luận nhất quán" — một kỳ vọng hợp lý NẾU cả hai cách đo cùng nhìn vào MỘT
đại lượng.

Chỗ lệch: `|w|` thô và permutation importance KHÔNG đo cùng một đại lượng
— `|w|` thô lẫn cả ảnh hưởng của THANG ĐO (bao nhiêu hệ số cần để tạo ra
MỘT đơn vị thay đổi ở `y`) vào con số của nó, trong khi permutation đo
TRỰC TIẾP ảnh hưởng lên sai số dự đoán, không quan tâm thang đo. Vì `x2` có
thang đo cực nhỏ, hai cách đo này có thể (và ở đây, THẬT SỰ) cho ra hai thứ
tự NGƯỢC NHAU, không chỉ khác độ lớn.
::
:::

:::opt
Không xác định được — cần biết thêm công thức chính xác của mô hình mới so
sánh được `x2` với `x3`
::why
Gần đúng ở việc bạn cẩn trọng, muốn có đủ thông tin trước khi kết luận —
một thái độ hợp lý nói chung khi so sánh số liệu.

Chỗ lệch: mọi thông tin cần thiết đã có sẵn trong ví dụ — bốn con số cụ
thể (`|w|` của `x2` và `x3`, mức tăng MSE khi xáo trộn `x2` và `x3`) đã đủ
để so sánh trực tiếp, không cần biết thêm công thức nào khác. Permutation
importance của `x3` (`0.883`) lớn hơn của `x2` (`0.1132`) là một con số đã
đo được, không phải một suy luận còn thiếu dữ kiện.
::
:::
::::

::::code{#permutation_importance_vs_w_tho}
Hoàn thiện vòng lặp permutation importance (xáo trộn từng cột, đo MSE
tăng), rồi xác định đặc trưng "quan trọng nhất" theo hai cách: `|w|` thô và
permutation.

```python title=starter
import numpy as np

rng_train = np.random.default_rng(5)
n = 40
x1 = rng_train.uniform(0, 50, n)
x2 = rng_train.uniform(0, 0.02, n)
x3 = rng_train.uniform(0, 10, n)
y = 5 + 0.8*x1 + 0.0*x2 + 0.3*x3 + rng_train.normal(0, 2.0, n)

X = np.vstack([x1, x2, x3]).T
Xb = np.hstack([X, np.ones((n, 1))])
he_so, *_ = np.linalg.lstsq(Xb, y, rcond=None)
w = he_so[:-1]
b = he_so[-1]

rng_test = np.random.default_rng(22)
n_te = 30
x1_te = rng_test.uniform(0, 50, n_te)
x2_te = rng_test.uniform(0, 0.02, n_te)
x3_te = rng_test.uniform(0, 10, n_te)
y_te = 5 + 0.8*x1_te + 0.0*x2_te + 0.3*x3_te + rng_test.normal(0, 2.0, n_te)
X_te = np.vstack([x1_te, x2_te, x3_te]).T

def mse(X_, y_, w_, b_):
    return np.mean((y_ - (X_ @ w_ + b_)) ** 2)

mse_goc = mse(X_te, y_te, w, b)

rng_perm = np.random.default_rng(1)
tang_theo_cot = {}
for i, ten in enumerate(["x1", "x2", "x3"]):
    X_perm = X_te.copy()
    X_perm[:, i] = ___                    # rng_perm.permutation(X_perm[:, i])
    mse_perm = ___                        # mse(X_perm, y_te, w, b)
    tang_theo_cot[ten] = mse_perm - mse_goc

quan_trong_nhat_theo_w = ___               # ["x1","x2","x3"][int(np.argmax(np.abs(w)))]
quan_trong_nhat_theo_perm = ___             # cot co tang MSE LON NHAT trong tang_theo_cot

print(np.round(w, 4).tolist())
print(round(mse_goc, 4))
print(quan_trong_nhat_theo_w)
print(quan_trong_nhat_theo_perm)
```

```python title=solution
import numpy as np

rng_train = np.random.default_rng(5)
n = 40
x1 = rng_train.uniform(0, 50, n)
x2 = rng_train.uniform(0, 0.02, n)
x3 = rng_train.uniform(0, 10, n)
y = 5 + 0.8*x1 + 0.0*x2 + 0.3*x3 + rng_train.normal(0, 2.0, n)

X = np.vstack([x1, x2, x3]).T
Xb = np.hstack([X, np.ones((n, 1))])
he_so, *_ = np.linalg.lstsq(Xb, y, rcond=None)
w = he_so[:-1]
b = he_so[-1]

rng_test = np.random.default_rng(22)
n_te = 30
x1_te = rng_test.uniform(0, 50, n_te)
x2_te = rng_test.uniform(0, 0.02, n_te)
x3_te = rng_test.uniform(0, 10, n_te)
y_te = 5 + 0.8*x1_te + 0.0*x2_te + 0.3*x3_te + rng_test.normal(0, 2.0, n_te)
X_te = np.vstack([x1_te, x2_te, x3_te]).T

def mse(X_, y_, w_, b_):
    return np.mean((y_ - (X_ @ w_ + b_)) ** 2)

mse_goc = mse(X_te, y_te, w, b)

rng_perm = np.random.default_rng(1)
tang_theo_cot = {}
for i, ten in enumerate(["x1", "x2", "x3"]):
    X_perm = X_te.copy()
    X_perm[:, i] = rng_perm.permutation(X_perm[:, i])
    mse_perm = mse(X_perm, y_te, w, b)
    tang_theo_cot[ten] = mse_perm - mse_goc

quan_trong_nhat_theo_w = ["x1","x2","x3"][int(np.argmax(np.abs(w)))]
quan_trong_nhat_theo_perm = max(tang_theo_cot, key=tang_theo_cot.get)

print(np.round(w, 4).tolist())
print(round(mse_goc, 4))
print(quan_trong_nhat_theo_w)
print(quan_trong_nhat_theo_perm)
```

```python title=test
assert np.round(w, 4).tolist() == [0.8094, -66.7726, 0.3028], f"w sai -- dang ra {np.round(w, 4).tolist()}"
assert round(mse_goc, 4) == 2.7364, f"mse_goc phai la 2.7364 -- dang ra {round(mse_goc, 4)}"
assert round(tang_theo_cot["x1"], 4) == 236.0749, f"tang MSE khi xao tron x1 phai la 236.0749 -- dang ra {round(tang_theo_cot['x1'], 4)}"
assert round(tang_theo_cot["x2"], 4) == 0.1132, f"tang MSE khi xao tron x2 phai la 0.1132 -- dang ra {round(tang_theo_cot['x2'], 4)}"
assert round(tang_theo_cot["x3"], 4) == 0.883, f"tang MSE khi xao tron x3 phai la 0.883 -- dang ra {round(tang_theo_cot['x3'], 4)}"
assert quan_trong_nhat_theo_w == "x2", f"theo |w| tho, cot quan trong nhat (SAI, gay hieu lam) phai la x2 -- dang ra {quan_trong_nhat_theo_w}"
assert quan_trong_nhat_theo_perm == "x1", f"theo permutation (DUNG), cot quan trong nhat phai la x1 -- dang ra {quan_trong_nhat_theo_perm}"
assert quan_trong_nhat_theo_w != quan_trong_nhat_theo_perm, "hai cach do phai cho ra HAI cot khac nhau -- dung trong tam bai: he so tho co the noi doi"
assert tang_theo_cot["x3"] > tang_theo_cot["x2"], "permutation importance cua x3 phai CAO HON x2, nguoc voi thu tu ma |w| tho goi y"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `X_perm[:, i]` được GÁN LẠI bằng chính nó đã XÁO TRỘN — `rng_perm.permutation(X_perm[:, i])`. `mse_perm` gọi lại hàm `mse` đã có sẵn, trên `X_perm` (không phải `X_te` gốc). `quan_trong_nhat_theo_w` tìm chỉ số của `|w|` LỚN NHẤT (`np.argmax(np.abs(w))`) rồi tra vào danh sách tên cột. `quan_trong_nhat_theo_perm` tìm khoá có GIÁ TRỊ lớn nhất trong dict `tang_theo_cot` — cùng khuôn `max(dict, key=dict.get)` đã dùng ở các bài trước.
- kind: strategy
  body: 'X_perm[:, i]: `rng_perm.permutation(X_perm[:, i])`. mse_perm: `mse(X_perm, y_te, w, b)`. quan_trong_nhat_theo_w: `["x1","x2","x3"][int(np.argmax(np.abs(w)))]`. quan_trong_nhat_theo_perm: `max(tang_theo_cot, key=tang_theo_cot.get)`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `rng_perm.permutation(X_perm[:, i])`, `mse(X_perm, y_te, w, b)`, `["x1","x2","x3"][int(np.argmax(np.abs(w)))]`, và `max(tang_theo_cot, key=tang_theo_cot.get)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: X_perm[:, i] phai THAT SU xao tron bang rng_perm.permutation (khong duoc giu nguyen cot cu); mse_perm phai goi THAT ham mse tren X_perm; quan_trong_nhat_theo_w phai dung np.argmax THAT tren np.abs(w); quan_trong_nhat_theo_perm phai dung max(...) THAT tren tang_theo_cot
  requireAst:
  - kind: uses-call, target: permutation, min: 1
  - kind: uses-call, target: mse, min: 2
  - kind: uses-call, target: argmax, min: 1
  - kind: uses-call, target: max, min: 1
  - kind: uses-name, target: tang_theo_cot, min: 3
  # Da thu that (goi kiemAst that tren code day du): loi giai dung dat=true,
  # ca nam luat qua sach. Cheat khong xao tron gi (X_perm[:,i]=X_perm[:,i]),
  # mse_perm=mse_goc (khong goi lai ham mse), va ca hai bien
  # quan_trong_nhat_* chep san chuoi "x1" (khong dung argmax/max) lam CA NAM
  # luat cung roi xuong duoi nguong -- bi chan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[0\\.8094, -66\\.7726, 0\\.3028\\]\\n2\\.7364\\nx2\\nx1\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`|w|` thô hét lên "x2 quan trọng nhất". Permutation importance, đo trực
tiếp trên sai số, trả lời "không — x1". Một con số dựa vào thang đo, một
con số không — chỉ một trong hai đáng tin.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chín bài vừa qua đào sâu từng mảnh riêng: bias-variance, bootstrap,
bagging, boosting, tuning, data leakage, xử lý lệch lớp, ổn định số học,
và giờ feature importance. Còn thiếu đúng một việc — RÁP tất cả những mảnh
đó, cùng với mọi mảnh của hai track trước, thành MỘT pipeline duy nhất, từ
dữ liệu thô tới một mô hình đã giải thích được. Bài sau, BOSS cuối cùng của
T8.1, làm đúng việc đó.
::::

::::checkpoint{mastery=0.8}
::::
