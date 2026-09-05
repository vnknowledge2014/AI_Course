---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.k-nn-phan-loai
title: "k-NN: phân loại theo hàng xóm"
summary: "11 điểm 2D tự bịa (5 lớp 0 gần gốc toạ độ, 5 lớp 1 ở xa, cộng 1 điểm NHIỄU tại (2.1,2.4) mang nhãn 1 dù nằm giữa cụm lớp 0) — một điểm mới (2.0, 2.5) có hàng xóm GẦN NHẤT chính là điểm nhiễu đó (khoảng cách 0.1414): k=1 bị nhiễu đánh lừa, dự đoán SAI thành lớp 1; k=3 và k=5 nhìn thêm hàng xóm thật của lớp 0, dự đoán ĐÚNG lớp 0 — k-NN không hề có bước huấn luyện nào, chỉ tính khoảng cách Euclid vector hoá (không vòng lặp) rồi bỏ phiếu đa số."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.k-nn]
requires: [ai.roc-auc]
concepts: [ai.k-nn]
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
Mọi mô hình từ đầu track đều có `w`, `b` học dần qua gradient descent. Đây
là một cách phân loại KHÔNG có bước đó — không `w`, không `b`, không lặp.
::::

::::explain{#hoc-khong-can-huan-luyen}
`logistic-regression-tu-so-0` tới `roc-va-auc` đều theo đúng một khuôn: khởi
tạo tham số, lặp gradient descent, HỘI TỤ về một bộ `w`, `b` cố định — rồi
dùng bộ đó để dự đoán MỌI điểm mới, không cần nhìn lại dữ liệu train nữa.

**k-nearest neighbors** (k-NN) làm khác hẳn: KHÔNG có `w`, `b` nào để học,
KHÔNG có vòng lặp gradient descent nào cả. Toàn bộ "mô hình" chỉ là chính
tập dữ liệu train, giữ nguyên. Muốn dự đoán một điểm MỚI: tính khoảng cách
từ điểm đó tới MỌI điểm train, tìm ra `k` điểm GẦN NHẤT, rồi để `k` điểm đó
BỎ PHIẾU — nhãn nào xuất hiện nhiều nhất trong `k` hàng xóm đó, dự đoán
điểm mới thuộc nhãn đó.

Khoảng cách chuẩn dùng ở đây là **khoảng cách Euclid** — với hai điểm
`(x₁, y₁)` và `(x₂, y₂)`, khoảng cách là `√((x₁−x₂)² + (y₁−y₂)²)`, đúng
định lý Pythagoras mở rộng. Với `numpy`, tính khoảng cách từ MỘT điểm mới
tới TOÀN BỘ tập train cùng lúc, không cần vòng lặp: `X_train - diem_moi`
trừ `diem_moi` khỏi MỌI hàng của `X_train` đồng loạt (numpy tự "phát sóng"
— broadcast — điểm mới ra khớp mọi hàng), rồi bình phương, cộng theo hàng,
rồi khai căn.

k-NN không có khái niệm "hội tụ" hay "overfitting theo tham số" như các
bài trước — nhưng nó có một tham số riêng, `k`, quyết định việc dự đoán
NHẠY tới mức nào với từng điểm dữ liệu riêng lẻ. `k` nhỏ (đặc biệt `k=1`)
để MỘT điểm duy nhất — có thể là một điểm NHIỄU hay bị gán nhãn sai —
quyết định TOÀN BỘ dự đoán. `k` lớn hơn cho nhiều điểm cùng bỏ phiếu, làm
mượt bớt ảnh hưởng của một điểm nhiễu đơn lẻ. Bài này đo trực tiếp sự khác
biệt đó bằng số thật.
::::

::::example{#nhieu-danh-lua-k-nho}
Mười một điểm 2D tự bịa: năm điểm lớp `0` gần gốc toạ độ, năm điểm lớp `1`
ở xa, và MỘT điểm nhiễu — toạ độ `(2.1, 2.4)`, nằm ngay GIỮA cụm lớp `0`,
nhưng lại mang nhãn `1` (một lỗi gán nhãn, hay một trường hợp ngoại lệ có
thật):

```python title=readonly
import numpy as np

X_train = np.array([
    [1, 1], [1, 2], [2, 1], [2, 2], [1, 3],   # lop 0 -- gan goc toa do
    [2.1, 2.4],                                # DIEM NHIEU -- nam giua lop 0, nhan la 1
    [8, 8], [8, 9], [9, 8], [9, 9], [8, 7],   # lop 1 -- xa goc toa do
], dtype=float)
y_train = np.array([0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1])

diem_moi = np.array([2.0, 2.5])

khoang_cach = np.sqrt(np.sum((X_train - diem_moi) ** 2, axis=1))
print("khoang cach toi tung diem train:", np.round(khoang_cach, 4).tolist())

thu_tu_gan = np.argsort(khoang_cach)
print("thu tu gan nhat (chi so):", thu_tu_gan.tolist())
print("nhan theo thu tu gan dan:", y_train[thu_tu_gan].tolist())

def knn_du_doan(k):
    idx = np.argsort(khoang_cach)[:k]
    nhan_gan = y_train[idx]
    vote0 = np.sum(nhan_gan == 0)
    vote1 = np.sum(nhan_gan == 1)
    return 1 if vote1 > vote0 else 0

for k in [1, 3, 5]:
    print("k =", k, "-> du doan =", knn_du_doan(k))
```

```text title=readonly
khoang cach toi tung diem train: [1.8028, 1.118, 1.5, 0.5, 1.118, 0.1414, 8.1394, 8.8459, 8.9022, 9.5525, 7.5]
thu tu gan nhat (chi so): [5, 3, 1, 4, 2, 0, 10, 6, 7, 8, 9]
nhan theo thu tu gan dan: [1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1]
k = 1 -> du doan = 1
k = 3 -> du doan = 0
k = 5 -> du doan = 0
```

Hàng xóm GẦN NHẤT của `diem_moi` (khoảng cách `0.1414`, chỉ số `5`) chính
là điểm NHIỄU, mang nhãn `1`. Với `k=1`, CHỈ mỗi hàng xóm đó được bỏ phiếu
— dự đoán ra `1`, dù `diem_moi` thật ra nằm ngay giữa cụm lớp `0`. Với
`k=3`, thêm hai hàng xóm THẬT của lớp `0` vào bỏ phiếu (chỉ số `3` và `1`,
khoảng cách `0.5` và `1.118`) — phiếu bầu thành `2` chọi `1`, lớp `0`
thắng, dự đoán đổi lại thành `0`. `k=5` thêm một hàng xóm lớp `0` nữa (chỉ
số `4`), lớp `0` càng thắng áp đảo hơn (`4` chọi `1`). Một điểm nhiễu đơn
lẻ đủ sức quyết định TOÀN BỘ dự đoán khi `k=1`, nhưng mất dần sức nặng khi
có thêm hàng xóm thật lên tiếng.
::::

::::predict{#doan-anh-huong-diem-nhieu commitOnce}
Vẫn dữ liệu trên. Giả sử XOÁ hẳn điểm nhiễu `(2.1, 2.4)` khỏi tập train
(chỉ còn `10` điểm, năm mỗi lớp, không còn điểm gán nhãn sai nào).

**Trước khi tính**, bạn đoán: với `k=1`, k-NN dự đoán nhãn của `diem_moi =
(2.0, 2.5)` là gì?

:::opt{correct}
Lớp `0` — sau khi bỏ điểm nhiễu, hàng xóm GẦN NHẤT thật sự là `(2, 2)`
(khoảng cách `0.5`, lớp `0`), nên dự đoán đổi lại đúng
:::

:::opt
Vẫn là lớp `1` — vì `diem_moi` nằm khá gần vùng có thể lẫn giữa hai cụm,
bất kể điểm nhiễu còn hay mất
::why
Gần đúng ở việc `diem_moi` đúng là nằm gần RANH GIỚI giữa hai cụm — không
sai về vị trí hình học của nó.

Chỗ lệch: dự đoán của k-NN chỉ phụ thuộc vào CHÍNH các điểm CÒN LẠI trong
tập train, không phụ thuộc "cảm giác nó gần ranh giới". Xoá điểm nhiễu
(chỉ số `5`, khoảng cách `0.1414` — hàng xóm gần nhất CŨ) để lại hàng xóm
gần nhất MỚI là `(2,2)` ở khoảng cách `0.5`, mang nhãn `0` thật. Với `k=1`,
dự đoán đổi hẳn từ `1` sang `0` — không còn điểm nào kéo nó về lớp `1` nữa.
::
:::

:::opt
Không xác định được — xoá một điểm khỏi tập train làm phép tính khoảng
cách không còn hợp lệ nữa
::why
Gần đúng ở việc bạn để ý XOÁ dữ liệu là một thay đổi thật, ảnh hưởng tới
tập train — quan sát đó không sai.

Chỗ lệch: k-NN không đòi hỏi một SỐ LƯỢNG điểm cố định — công thức khoảng
cách Euclid và phép bỏ phiếu đa số vẫn chạy bình thường với `10` điểm thay
vì `11`. "Không xác định được" không đúng — kết quả HOÀN TOÀN xác định,
chỉ là khác với khi còn `11` điểm.
::
:::
::::

::::code{#viet_knn_vector_hoa}
Viết nốt: khoảng cách Euclid vector hoá (không vòng lặp), thứ tự hàng xóm
gần nhất, và bỏ phiếu đa số trong `knn_du_doan`.

```python title=starter
import numpy as np

X_train = np.array([
    [1, 1], [1, 2], [2, 1], [2, 2], [1, 3],
    [2.1, 2.4],
    [8, 8], [8, 9], [9, 8], [9, 9], [8, 7],
], dtype=float)
y_train = np.array([0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1])

diem_moi = np.array([2.0, 2.5])

khoang_cach = ___                    # can bac hai cua tong binh phuong hieu, theo tung hang (axis=1)

def knn_du_doan(k):
    idx = ___                        # k chi so co khoang_cach NHO NHAT
    nhan_gan = y_train[idx]
    vote0 = np.sum(nhan_gan == 0)
    vote1 = np.sum(nhan_gan == 1)
    return ___                       # 1 neu vote1 > vote0, nguoc lai 0

ket_qua = [knn_du_doan(k) for k in [1, 3, 5]]
print(ket_qua)
```

```python title=solution
import numpy as np

X_train = np.array([
    [1, 1], [1, 2], [2, 1], [2, 2], [1, 3],
    [2.1, 2.4],
    [8, 8], [8, 9], [9, 8], [9, 9], [8, 7],
], dtype=float)
y_train = np.array([0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1])

diem_moi = np.array([2.0, 2.5])

khoang_cach = np.sqrt(np.sum((X_train - diem_moi) ** 2, axis=1))

def knn_du_doan(k):
    idx = np.argsort(khoang_cach)[:k]
    nhan_gan = y_train[idx]
    vote0 = np.sum(nhan_gan == 0)
    vote1 = np.sum(nhan_gan == 1)
    return 1 if vote1 > vote0 else 0

ket_qua = [knn_du_doan(k) for k in [1, 3, 5]]
print(ket_qua)
```

```python title=test
assert round(khoang_cach[5], 4) == 0.1414, f"khoang cach toi diem nhieu (chi so 5) phai la 0.1414 -- dang ra {round(khoang_cach[5], 4)}"
assert round(khoang_cach[3], 4) == 0.5, f"khoang cach toi diem (2,2) (chi so 3) phai la 0.5 -- dang ra {round(khoang_cach[3], 4)}"
assert ket_qua == [1, 0, 0], f"du doan cho k=1,3,5 phai la [1, 0, 0] -- dang ra {ket_qua}"

# k=1,3,5 khong bao gio HOA phieu (moi lan vote0/vote1 luon lech nhau) nen
# khong phan biet duoc dau '>' voi '>='. k=2 THAT SU hoa: hai hang xom gan
# nhat cua diem_moi la diem (2,2) (nhan 0) va diem NHIEU (nhan 1) -- vote0=
# vote1=1. Dung '>' phai nghieng ve 0 (vote1 KHONG lon hon vote0).
assert knn_du_doan(2) == 0, f"voi k=2, vote0=vote1=1 (hoa) -- phai tra ve 0 (vote1 khong LON HON vote0), dang ra {knn_du_doan(2)}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. `khoang_cach` là công thức Euclid vector hoá đã học — `np.sqrt(np.sum((X_train - diem_moi) ** 2, axis=1))`. `idx` lấy `k` chỉ số ĐẦU TIÊN của mảng đã SẮP XẾP theo khoảng cách tăng dần (`np.argsort`). Chỗ cuối là biểu thức điều kiện Python (`gia_tri_neu_dung if dieu_kien else gia_tri_neu_sai`) so sánh `vote1` với `vote0`.
- kind: strategy
  body: 'khoang_cach: `np.sqrt(np.sum((X_train - diem_moi) ** 2, axis=1))`. idx: `np.argsort(khoang_cach)[:k]`. return: `1 if vote1 > vote0 else 0`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `np.sqrt(np.sum((X_train - diem_moi) ** 2, axis=1))`, `np.argsort(khoang_cach)[:k]`, và `1 if vote1 > vote0 else 0`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: khoang_cach phai THAT SU khai can bac hai (np.sqrt) -- binh phuong khong khai can van xep dung THU TU (vi khai can la ham dong bien) nen de "an toan qua test" ma khong tinh dung khoang cach Euclid, dung trong tam bai nay; idx phai sap xep THAT tu khoang_cach; return phai so sanh vote1/vote0 that su, khong duoc dao nguoc hay chep hang so
  requireAst:
  - kind: uses-call, target: sqrt, min: 1
  - kind: uses-name, target: khoang_cach, min: 1
  - kind: uses-name, target: vote0, min: 1
  - kind: uses-name, target: vote1, min: 1
  - kind: uses-name, target: nhan_gan, min: 2
  # Da thu that (goi kiemAst that tren code day du):
  # - loi giai dung: dat=true, ca nam luat qua sach.
  # - bo sqrt (dung binh phuong khoang cach truc tiep -- VAN xep dung thu
  #   tu vi khai can dong bien, nen se qua tier tests/output neu khong co
  #   luat nay): sqrt tut xuong 0 -- duoi nguong 1, bi chan boi static.
  # - dao nguoc dieu kien (return 0 if vote1>vote0 else 1): khong bi static
  #   chan (van dung du ten), nhung se bi tier tests chan qua gia tri sai
  #   (ket_qua thanh [0,1,1] thay vi [1,0,0]) -- dung phan cong hai tang.
  # - chep hang so ket_qua = [1,0,0] (bo qua toan bo ham): khoang_cach,
  #   vote0, vote1, nhan_gan deu tut ve 0 -- duoi moi nguong, bi chan.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\[1, 0, 0\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một điểm nhiễu duy nhất lật ngược dự đoán của `k=1`. Thêm vài hàng xóm bỏ
phiếu cùng, và điểm nhiễu đó không còn đủ sức nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

k-NN quyết định bằng KHOẢNG CÁCH — không quan tâm gì tới CẤU TRÚC bên
trong của dữ liệu, chỉ đo "gần" hay "xa". Có một cách phân loại khác hẳn:
thay vì đo khoảng cách, đặt ra từng CÂU HỎI về TỪNG đặc trưng một — "đặc
trưng này có lớn hơn một ngưỡng nào đó không?" — và chia dữ liệu theo câu
trả lời.

Cách đặt câu hỏi "tốt nhất" để chia là gì? Bài sau trả lời, bằng một khái
niệm mượn từ lý thuyết thông tin.
::::

::::checkpoint{mastery=0.8}
::::
