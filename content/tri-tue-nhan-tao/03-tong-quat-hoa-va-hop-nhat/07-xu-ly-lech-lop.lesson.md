---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.xu-ly-lech-lop
title: "Xử lý lệch lớp: class weight"
summary: "25 điểm (20 lớp 0, 5 lớp 1 — lệch 4:1), logistic regression: KHÔNG trọng số cho conf=(TP=3,FP=0,TN=20,FN=2), precision=1.0, recall=0.6, F1=0.75. Nhân trọng số lớp thiểu số trong gradient lên đúng tỷ lệ lệch lớp (4.0 lần) rồi huấn luyện lại: conf=(TP=4,FP=0,TN=20,FN=1), precision VẪN 1.0 (không thêm báo động giả nào) nhưng recall tăng lên 0.8 và F1 lên 0.8889 — bắt thêm được một ca dương thật mà không mất gì ở precision."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.xu-ly-lech-lop]
requires: [ai.data-leakage]
concepts: [ai.xu-ly-lech-lop]
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
`precision-recall-f1` (track trước) đã CHỈ RA vấn đề: dữ liệu lệch lớp làm
accuracy nói dối, và F1 phản ánh đúng hơn. Nhưng bài đó chỉ dừng ở việc PHÁT
HIỆN. Lần này, sửa nó.
::::

::::explain{#trong-so-lop}
Vì sao một mô hình huấn luyện bình thường trên dữ liệu lệch lớp thường có
recall thấp cho lớp thiểu số? Hàm mất mát cross-entropy (bài
`logistic-regression-tu-so-0`) TRUNG BÌNH sai số qua MỌI điểm — với `20`
điểm lớp `0` và chỉ `5` điểm lớp `1`, phần đóng góp của lớp `1` vào tổng sai
số chỉ chiếm một phần nhỏ. Gradient descent, để giảm loss TỔNG nhanh nhất,
tự nhiên "ưu tiên" khớp tốt cho lớp CHIẾM ĐA SỐ — vì đó là nơi có nhiều
điểm nhất để giảm sai số.

**Class weight** (trọng số lớp) sửa đúng chỗ mất cân bằng đó: thay vì TRUNG
BÌNH ĐỀU mọi điểm, gán một TRỌNG SỐ lớn hơn cho các điểm thuộc lớp thiểu
số khi tính gradient — buộc mô hình phải "chú ý" nhiều hơn tới từng điểm
hiếm đó, bù lại việc chúng ít về SỐ LƯỢNG.

> `dw = (1/Σw) · Σᵢ wᵢ·xᵢ·(pᵢ − yᵢ)`
>
> `db = (1/Σw) · Σᵢ wᵢ·(pᵢ − yᵢ)`

Với `wᵢ` là trọng số của điểm `i` — `1.0` cho lớp đa số, và một số LỚN HƠN
`1.0` cho lớp thiểu số. Một lựa chọn trọng số phổ biến: TỶ LỆ lệch lớp
chính nó — nếu lớp đa số nhiều gấp `k` lần lớp thiểu số, gán trọng số `k`
cho MỖI điểm thiểu số, giữ trọng số `1` cho điểm đa số. Cách này cân bằng
lại TỔNG trọng số của hai lớp: `20` điểm đa số × `1` = `20`, và `5` điểm
thiểu số × `4` = `20` — bằng nhau, dù số LƯỢNG điểm chênh lệch `4` lần.

(Một kỹ thuật khác, không dùng trong bài này nhưng cùng mục tiêu:
**oversampling** — LẶP LẠI các điểm thiểu số nhiều lần trong dữ liệu train,
để chúng xuất hiện thường xuyên hơn một cách trực tiếp, thay vì chỉ đổi
trọng số trong công thức gradient. Cả hai đều nhắm cùng một đích: buộc mô
hình không lãng quên lớp hiếm.)
::::

::::example{#trong-so-cai-thien-recall}
Hai mươi lăm điểm, lệch lớp `4:1` (`20` lớp `0`, `5` lớp `1`). Huấn luyện
logistic regression hai lần — KHÔNG trọng số, rồi CÓ trọng số lớp thiểu số
(bằng đúng tỷ lệ lệch `4.0`):

```python title=readonly
import numpy as np

rng = np.random.default_rng(4)
n0, n1 = 20, 5
x = np.concatenate([rng.uniform(0, 6, n0), rng.uniform(3, 10, n1)])
y = np.concatenate([np.zeros(n0), np.ones(n1)])
n = len(x)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP): return TP / (TP + FP) if (TP + FP) > 0 else 0.0
def recall(TP, FN): return TP / (TP + FN) if (TP + FN) > 0 else 0.0
def f1(p, r): return 2 * p * r / (p + r) if (p + r) > 0 else 0.0

def huan_luyen(trong_so, lr, so_buoc):
    w, b = 0.0, 0.0
    tong_w = np.sum(trong_so)
    for _ in range(so_buoc):
        p = sigmoid(w * x + b)
        dw = np.sum(trong_so * x * (p - y)) / tong_w
        db = np.sum(trong_so * (p - y)) / tong_w
        w -= lr * dw
        b -= lr * db
    return w, b

# KHONG trong so -- moi diem dong gop nhu nhau
trong_so_deu = np.ones(n)
w1, b1 = huan_luyen(trong_so_deu, 0.5, 1000)
pred1 = (sigmoid(w1 * x + b1) >= 0.5).astype(int)
TP1, FP1, TN1, FN1 = conf(y, pred1)
p1, r1 = precision(TP1, FP1), recall(TP1, FN1)
print("KHONG trong so: conf=", (TP1,FP1,TN1,FN1), " prec=", round(p1,4), " rec=", round(r1,4), " f1=", round(f1(p1,r1),4))

# CO trong so -- lop thieu so duoc nhan trong so bang dung ty le lech lop
ty_le = n0 / n1
trong_so_can_bang = np.where(y == 1, ty_le, 1.0)
w2, b2 = huan_luyen(trong_so_can_bang, 0.5, 1000)
pred2 = (sigmoid(w2 * x + b2) >= 0.5).astype(int)
TP2, FP2, TN2, FN2 = conf(y, pred2)
p2, r2 = precision(TP2, FP2), recall(TP2, FN2)
print("CO trong so   : conf=", (TP2,FP2,TN2,FN2), " prec=", round(p2,4), " rec=", round(r2,4), " f1=", round(f1(p2,r2),4))
```

```text title=readonly
KHONG trong so: conf= (3, 0, 20, 2)  prec= 1.0  rec= 0.6  f1= 0.75
CO trong so   : conf= (4, 0, 20, 1)  prec= 1.0  rec= 0.8  f1= 0.8889
```

KHÔNG trọng số: precision TUYỆT ĐỐI (`1.0` — không báo động giả nào) nhưng
recall chỉ `0.6` — bỏ lọt `2` trong `5` ca lớp `1` thật. CÓ trọng số: precision
VẪN `1.0` (không hề tăng thêm báo động giả nào, `FP` vẫn là `0`), nhưng
recall tăng lên `0.8` — bắt thêm được đúng MỘT ca lớp `1` mà trước đó bị bỏ
lọt, `FN` giảm từ `2` xuống `1`. `F1` tăng từ `0.75` lên `0.8889`. Đây là
kết quả TỐT NHẤT có thể xảy ra khi thêm class weight: cải thiện recall MÀ
KHÔNG phải đánh đổi bằng precision — không phải lúc nào cũng xảy ra như
vậy (thường phải đánh đổi ít nhiều), nhưng khi xảy ra, nó chứng minh mô
hình trước đó thật sự đã bỏ sót một ca lớp `1` hoàn toàn có thể bắt được,
chỉ vì trọng số huấn luyện bất công.
::::

::::predict{#doan-neu-trong-so-qua-lon commitOnce}
Vẫn hai mươi lăm điểm trên. Giả sử, thay vì dùng đúng tỷ lệ lệch lớp
(`4.0`), Byte gán một trọng số CỰC LỚN cho lớp thiểu số — ví dụ `1000.0`.

**Trước khi chạy**, bạn đoán: điều gì nhiều khả năng xảy ra với precision
của mô hình, so với dùng trọng số `4.0`?

:::opt{correct}
Precision NHIỀU KHẢ NĂNG sẽ GIẢM — trọng số quá lớn có thể khiến mô hình
"quá sợ" bỏ lọt lớp thiểu số, dự đoán lớp `1` cho cả những điểm THẬT SỰ
thuộc lớp `0`, làm tăng báo động giả (`FP`)
:::

:::opt
Precision sẽ không đổi gì — trọng số chỉ ảnh hưởng tới recall, không bao
giờ ảnh hưởng tới precision
::why
Gần đúng ở việc bài này đúng cho thấy MỘT trường hợp precision không đổi
(`1.0` cả hai lần) — quan sát đó có căn cứ TRONG chính ví dụ vừa xem.

Chỗ lệch: đó là một KẾT QUẢ CỤ THỂ của mức trọng số `4.0` trên dữ liệu này,
không phải một QUY LUẬT chung. Class weight thay đổi CHÍNH đường ranh giới
quyết định (`w`, `b`) — đẩy nó dịch chuyển để ưu tiên lớp thiểu số hơn.
Đẩy đủ mạnh (trọng số `1000.0`, gấp `250` lần trọng số đã dùng) hoàn toàn
có thể đẩy đường ranh giới đó QUÁ ĐÀ, khiến một số điểm lớp `0` thật bị gọi
nhầm thành lớp `1` — làm `FP` tăng và precision giảm theo.
::
:::

:::opt
Precision sẽ tăng thêm nữa — trọng số càng lớn thì mô hình càng "cẩn thận"
hơn, nên càng ít sai
::why
Gần đúng ở trực giác "trọng số lớn hơn thể hiện một dạng chú ý nhiều hơn"
— một liên tưởng hợp lý về mặt ngôn ngữ.

Chỗ lệch: "chú ý nhiều hơn tới lớp thiểu số" và "cẩn thận hơn nói chung" là
hai điều khác nhau. Trọng số lớn ưu tiên MỘT HƯỚNG cụ thể — đừng bỏ lọt lớp
thiểu số — mà không hề đảm bảo tổng thể mô hình "cẩn thận" hơn. Đẩy quá
mạnh về một hướng thường kéo theo đánh đổi ở hướng còn lại (ở đây là
precision), đúng bản chất đánh đổi mà `precision-recall-f1` đã dạy.
::
:::
::::

::::code{#trong_so_lop_cai_thien_recall}
Hoàn thiện gradient CÓ TRỌNG SỐ (`dw`, `db` nhân với `trong_so`), rồi tính
`ty_le` lệch lớp và tạo `trong_so_can_bang` — trọng số `ty_le` cho lớp `1`,
trọng số `1.0` cho lớp `0`.

```python title=starter
import numpy as np

rng = np.random.default_rng(4)
n0, n1 = 20, 5
x = np.concatenate([rng.uniform(0, 6, n0), rng.uniform(3, 10, n1)])
y = np.concatenate([np.zeros(n0), np.ones(n1)])
n = len(x)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP): return TP / (TP + FP) if (TP + FP) > 0 else 0.0
def recall(TP, FN): return TP / (TP + FN) if (TP + FN) > 0 else 0.0
def f1(p, r): return 2 * p * r / (p + r) if (p + r) > 0 else 0.0

def huan_luyen(trong_so, lr, so_buoc):
    w, b = 0.0, 0.0
    tong_w = np.sum(trong_so)
    for _ in range(so_buoc):
        p = sigmoid(w * x + b)
        dw = ___                        # np.sum(trong_so * x * (p - y)) / tong_w
        db = ___                        # np.sum(trong_so * (p - y)) / tong_w
        w -= lr * dw
        b -= lr * db
    return w, b

trong_so_deu = np.ones(n)
w1, b1 = huan_luyen(trong_so_deu, 0.5, 1000)
pred1 = (sigmoid(w1 * x + b1) >= 0.5).astype(int)
TP1, FP1, TN1, FN1 = conf(y, pred1)
p1, r1 = precision(TP1, FP1), recall(TP1, FN1)

ty_le = ___                              # n0 / n1
trong_so_can_bang = ___                  # np.where(y == 1, ty_le, 1.0)
w2, b2 = huan_luyen(trong_so_can_bang, 0.5, 1000)
pred2 = (sigmoid(w2 * x + b2) >= 0.5).astype(int)
TP2, FP2, TN2, FN2 = conf(y, pred2)
p2, r2 = precision(TP2, FP2), recall(TP2, FN2)

print(TP1, FP1, TN1, FN1, round(p1, 4), round(r1, 4), round(f1(p1, r1), 4))
print(TP2, FP2, TN2, FN2, round(p2, 4), round(r2, 4), round(f1(p2, r2), 4))
```

```python title=solution
import numpy as np

rng = np.random.default_rng(4)
n0, n1 = 20, 5
x = np.concatenate([rng.uniform(0, 6, n0), rng.uniform(3, 10, n1)])
y = np.concatenate([np.zeros(n0), np.ones(n1)])
n = len(x)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def conf(y_that, y_pred):
    TP = int(np.sum((y_that == 1) & (y_pred == 1)))
    FP = int(np.sum((y_that == 0) & (y_pred == 1)))
    TN = int(np.sum((y_that == 0) & (y_pred == 0)))
    FN = int(np.sum((y_that == 1) & (y_pred == 0)))
    return TP, FP, TN, FN

def precision(TP, FP): return TP / (TP + FP) if (TP + FP) > 0 else 0.0
def recall(TP, FN): return TP / (TP + FN) if (TP + FN) > 0 else 0.0
def f1(p, r): return 2 * p * r / (p + r) if (p + r) > 0 else 0.0

def huan_luyen(trong_so, lr, so_buoc):
    w, b = 0.0, 0.0
    tong_w = np.sum(trong_so)
    for _ in range(so_buoc):
        p = sigmoid(w * x + b)
        dw = np.sum(trong_so * x * (p - y)) / tong_w
        db = np.sum(trong_so * (p - y)) / tong_w
        w -= lr * dw
        b -= lr * db
    return w, b

trong_so_deu = np.ones(n)
w1, b1 = huan_luyen(trong_so_deu, 0.5, 1000)
pred1 = (sigmoid(w1 * x + b1) >= 0.5).astype(int)
TP1, FP1, TN1, FN1 = conf(y, pred1)
p1, r1 = precision(TP1, FP1), recall(TP1, FN1)

ty_le = n0 / n1
trong_so_can_bang = np.where(y == 1, ty_le, 1.0)
w2, b2 = huan_luyen(trong_so_can_bang, 0.5, 1000)
pred2 = (sigmoid(w2 * x + b2) >= 0.5).astype(int)
TP2, FP2, TN2, FN2 = conf(y, pred2)
p2, r2 = precision(TP2, FP2), recall(TP2, FN2)

print(TP1, FP1, TN1, FN1, round(p1, 4), round(r1, 4), round(f1(p1, r1), 4))
print(TP2, FP2, TN2, FN2, round(p2, 4), round(r2, 4), round(f1(p2, r2), 4))
```

```python title=test
assert (TP1, FP1, TN1, FN1) == (3, 0, 20, 2), f"confusion matrix KHONG trong so sai -- dang ra {(TP1, FP1, TN1, FN1)}"
assert round(p1, 4) == 1.0 and round(r1, 4) == 0.6, f"precision/recall KHONG trong so phai la 1.0/0.6 -- dang ra {round(p1,4)}/{round(r1,4)}"
assert round(f1(p1, r1), 4) == 0.75, "F1 KHONG trong so phai la 0.75"
assert ty_le == 4.0, f"ty_le (n0/n1) phai la 4.0 -- dang ra {ty_le}"
assert (TP2, FP2, TN2, FN2) == (4, 0, 20, 1), f"confusion matrix CO trong so sai -- dang ra {(TP2, FP2, TN2, FN2)}"
assert round(p2, 4) == 1.0 and round(r2, 4) == 0.8, f"precision/recall CO trong so phai la 1.0/0.8 -- dang ra {round(p2,4)}/{round(r2,4)}"
assert round(f1(p2, r2), 4) == 0.8889, "F1 CO trong so phai la 0.8889"
assert r2 > r1, "recall SAU khi them trong so phai CAO HON recall TRUOC do"
assert f1(p2, r2) > f1(p1, r1), "F1 SAU khi them trong so phai CAO HON F1 TRUOC do"

# Tren du lieu that cua bai, trong_so_can_bang[y==1] luon dung bang ty_le va
# trong_so_can_bang[y==0] luon dung bang 1.0 -- kiem tra rieng hai gia tri
# nay de dam bao np.where khong bi dao nguoc (gan ty_le cho lop 0 thay vi
# lop 1), diem loi ma cac assert tren (chi nhin ket qua cuoi qua huan_luyen)
# co the khong phan biet duoc neu tinh co lech.
assert np.all(trong_so_can_bang[y == 1] == ty_le), "moi diem lop 1 phai duoc gan dung trong so ty_le"
assert np.all(trong_so_can_bang[y == 0] == 1.0), "moi diem lop 0 phai duoc gan dung trong so 1.0 (khong doi)"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `dw`, `db` là gradient CÓ TRỌNG SỐ — công thức giống hệt `logistic-regression-tu-so-0`, chỉ thêm nhân `trong_so` vào TỪNG số hạng trước khi cộng, rồi chia cho `tong_w` (tổng trọng số) thay vì chia cho `n`. `ty_le` là tỷ lệ lớp đa số trên lớp thiểu số (`n0 / n1`). `trong_so_can_bang` gán `ty_le` cho MỌI điểm lớp `1`, và `1.0` cho MỌI điểm lớp `0` — dùng `np.where(điều_kiện, giá_trị_nếu_đúng, giá_trị_nếu_sai)`.
- kind: strategy
  body: 'dw: `np.sum(trong_so * x * (p - y)) / tong_w`. db: `np.sum(trong_so * (p - y)) / tong_w`. ty_le: `n0 / n1`. trong_so_can_bang: `np.where(y == 1, ty_le, 1.0)`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `np.sum(trong_so * x * (p - y)) / tong_w`, `np.sum(trong_so * (p - y)) / tong_w`, `n0 / n1`, và `np.where(y == 1, ty_le, 1.0)`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: dw/db phai nhan THAT voi trong_so (thieu no thi trong so khong co tac dung gi, giong het gradient khong trong so); ty_le phai tinh THAT tu n0/n1; trong_so_can_bang phai dung np.where THAT de gan ty_le cho lop 1 va 1.0 cho lop 0
  requireAst:
  - kind: uses-name, target: trong_so, min: 3
  - kind: uses-name, target: ty_le, min: 1
  - kind: uses-call, target: where, min: 1
  - kind: uses-name, target: n0, min: 2
  - kind: uses-name, target: n1, min: 2
  # Da thu that (goi kiemAst that tren code day du): loi giai dung dat=true,
  # ca nam luat qua sach. Cheat bo trong_so khoi dw/db (dung cong thuc khong
  # trong so nhu logistic-regression-tu-so-0, dw=np.mean(x*(p-y))) VA chep
  # san ty_le=1.0, trong_so_can_bang=np.ones(n) (khong dung n0/n1/where) lam
  # CA NAM luat cung roi xuong duoi nguong -- bi chan. Rieng mutation doi
  # np.where(y == 1, ...) thanh np.where(y != 1, ...) (dao nguoc dieu kien)
  # KHONG bi static bat (van dung du ten/ham) nhung da chay THAT xac nhan no
  # lam sai HANG LOAT gia tri (TP giam tu 4 xuong 2, recall giam tu 0.8
  # xuong 0.4) -- bi chan chac chan boi tier tests, vi day la dao nguoc TOAN
  # BO logic tren moi diem du lieu (khong phai mot diem bien hiem gap ma du
  # lieu tu nhien khong bao gio cham toi) -- KHAC voi mau ">/>=" tren du lieu
  # so thuc lien tuc da gay lo o quest truoc.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^3 0 20 2 1\\.0 0\\.6 0\\.75\\n4 0 20 1 1\\.0 0\\.8 0\\.8889\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Recall từ `0.6` lên `0.8`, precision vẫn nguyên `1.0` — không đánh đổi gì,
chỉ vì trọng số huấn luyện công bằng hơn với lớp hiếm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Class weight sửa một VẤN ĐỀ VỀ SỐ LƯỢNG dữ liệu (lớp thiểu số quá ít điểm).
Nhưng có một loại vấn đề KHÁC, không liên quan gì tới việc lớp nào nhiều
hay ít — mà tới chính PHÉP TÍNH bên trong công thức mất mát: điều gì xảy ra
nếu mô hình dự đoán một xác suất tuyệt đối `0` cho đúng nhãn thật, và công
thức cross-entropy phải tính `log(0)`?

Bài sau chỉ ra chỗ đó.
::::

::::checkpoint{mastery=0.8}
::::
