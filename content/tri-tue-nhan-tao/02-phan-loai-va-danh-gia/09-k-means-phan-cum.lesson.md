---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.k-means-phan-cum
title: "k-means: phân cụm không nhãn"
summary: "9 điểm 2D tự bịa, 3 cụm rõ rệt (quanh (1,1), (8,1), (4,8)), KHÔNG nhãn nào được đưa vào — khởi tạo tâm cố ý lệch [[1,1],[2,1],[1,2]]: vòng 1 gán nhầm điểm (1,2) vào cụm sai (trùng đúng toạ độ tâm khởi tạo), tâm dịch chuyển mạnh; vòng 2 gán ĐÚNG cả 9 điểm về ba cụm thật, tâm tiến sát giá trị trung bình thật của mỗi cụm; vòng 3 tâm KHÔNG đổi nữa (hội tụ) — [[1.3333,1.3333],[8.3333,1.3333],[4.3333,8.3333]], khớp đúng trung bình thật của từng cụm."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.k-means]
requires: [ai.decision-tree]
concepts: [ai.k-means]
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
Tám bài vừa qua đều có NHÃN đi kèm dữ liệu — mô hình học để khớp đúng nhãn
đó. Bài này không có nhãn nào cả. Không một cái nào.
::::

::::explain{#khong-nhan-van-hoc-duoc}
Mọi bài từ `logistic-regression-tu-so-0` tới `decision-tree-don-gian` đều
là **học có giám sát** (supervised learning): dữ liệu đi kèm nhãn ĐÚNG
(spam/không spam, lớp 0/1/2), và mô hình học để khớp đúng nhãn đó. **k-means**
thuộc nhóm hoàn toàn khác — **học không giám sát** (unsupervised learning):
chỉ có dữ liệu THÔ, không một nhãn nào cả. Mục tiêu không phải "dự đoán
đúng nhãn cho sẵn" — mà là tự tìm ra CẤU TRÚC ẩn trong dữ liệu: có bao
nhiêu NHÓM tự nhiên, và điểm nào thuộc nhóm nào.

k-means tìm `k` nhóm (cụm, chọn trước bởi người dùng) bằng cách lặp lại
đúng HAI bước, tới khi không còn gì đổi:

> **Bước gán-cụm** — với `k` **tâm cụm** (centroid) hiện tại, mỗi điểm dữ
> liệu được gán vào cụm có tâm GẦN NÓ NHẤT (khoảng cách Euclid, đúng công
> thức đã học ở `k-nn-phan-loai`).
>
> **Bước cập nhật-tâm** — với các điểm vừa gán xong, tâm MỚI của mỗi cụm
> là TRUNG BÌNH toạ độ của mọi điểm đã gán vào cụm đó.

Lặp lại hai bước này: gán lại theo tâm mới, cập nhật tâm theo cách gán
mới, gán lại theo tâm mới hơn nữa... Thuật toán DỪNG khi tâm không còn
DỊCH CHUYỂN nữa giữa hai vòng lặp liên tiếp — gọi là **hội tụ**. Không có
gradient, không có learning rate — chỉ có hai phép tính đơn giản lặp lại.

Tâm khởi tạo ban đầu ảnh hưởng tới việc bao nhiêu vòng lặp cần để hội tụ
(và trong một số trường hợp xấu, có thể ảnh hưởng tới việc hội tụ về nhóm
nào) — bài này dùng một tâm khởi tạo CỐ Ý không hoàn hảo, để thấy rõ quá
trình "dịch chuyển dần rồi ổn định" bằng số thật.
::::

::::example{#hoi-tu-qua-ba-vong}
Chín điểm 2D tự bịa, chia thành ba cụm rõ rệt trên mặt phẳng (quanh
`(1,1)`, quanh `(8,1)`, quanh `(4,8)`) — nhưng thuật toán KHÔNG được biết
trước cụm nào là cụm nào, chỉ nhận đúng toạ độ:

```python title=readonly
import numpy as np

X = np.array([
    [1, 1], [1, 2], [2, 1],
    [8, 1], [9, 1], [8, 2],
    [4, 8], [5, 8], [4, 9],
], dtype=float)
n, K = len(X), 3

def gan_cum(X, tam):
    kc = np.zeros((len(X), K))
    for k in range(K):
        kc[:, k] = np.sqrt(np.sum((X - tam[k]) ** 2, axis=1))
    return np.argmin(kc, axis=1)

def cap_nhat_tam(X, nhan):
    tam_moi = np.zeros((K, X.shape[1]))
    for k in range(K):
        tam_moi[k] = X[nhan == k].mean(axis=0)
    return tam_moi

tam = np.array([[1.0, 1.0], [2.0, 1.0], [1.0, 2.0]])   # khoi tao co y KHONG hoan hao
for vong in range(1, 4):
    nhan = gan_cum(X, tam)
    tam_moi = cap_nhat_tam(X, nhan)
    print(f"vong {vong}: nhan={nhan.tolist()}  tam_moi={np.round(tam_moi, 4).tolist()}  hoi_tu={np.allclose(tam_moi, tam)}")
    tam = tam_moi
```

```text title=readonly
vong 1: nhan=[0, 2, 1, 1, 1, 1, 2, 2, 2]  tam_moi=[[1.0, 1.0], [6.75, 1.25], [3.5, 6.75]]  hoi_tu=False
vong 2: nhan=[0, 0, 0, 1, 1, 1, 2, 2, 2]  tam_moi=[[1.3333, 1.3333], [8.3333, 1.3333], [4.3333, 8.3333]]  hoi_tu=False
vong 3: nhan=[0, 0, 0, 1, 1, 1, 2, 2, 2]  tam_moi=[[1.3333, 1.3333], [8.3333, 1.3333], [4.3333, 8.3333]]  hoi_tu=True
```

`vòng 1`: tâm khởi tạo `[[1,1],[2,1],[1,2]]` NẰM SÁT nhau (cả ba đều gần
góc dưới-trái) — điểm `(1,2)` (chỉ số `1`) bị gán NHẦM vào cụm `2`, đơn
giản vì tâm khởi tạo của cụm `2` TRÙNG ĐÚNG toạ độ của chính nó
(`[1.0, 2.0]`), khoảng cách `0`. Sau bước cập nhật, tâm dịch chuyển MẠNH —
từ `[2.0, 1.0]` nhảy tới `[6.75, 1.25]`, kéo về phía cụm bên phải. `vòng
2`: với tâm mới đã dịch xa hơn, TẤT CẢ chín điểm được gán ĐÚNG về ba cụm
thật — tâm tiến rất gần giá trị trung bình thật của mỗi cụm. `vòng 3`: gán
cụm KHÔNG đổi nữa so với vòng 2, tâm cũng KHÔNG đổi (`hội_tụ = True`) —
thuật toán dừng, `[[1.3333, 1.3333], [8.3333, 1.3333], [4.3333, 8.3333]]`
đúng bằng trung bình toạ độ thật của từng cụm ba điểm.
::::

::::predict{#doan-diem-nham-o-vong-2 commitOnce}
Ở `vòng 1`, điểm `(1, 2)` (chỉ số `1`) bị gán nhầm vào cụm `2` (đúng vì tâm
khởi tạo của cụm đó trùng khớp toạ độ điểm này).

**Trước khi đọc lại bảng**, bạn đoán: sang `vòng 2`, điểm này có được gán
LẠI đúng cụm `0` (cùng cụm với `(1,1)` và `(2,1)`) hay không?

:::opt{correct}
Có — sau bước cập nhật của vòng 1, tâm cụm `2` đã dịch xa hẳn (tới
`[3.5, 6.75]`), không còn ở gần `(1,2)` nữa, nên tới vòng 2, điểm này gần
tâm cụm `0` hơn và được gán lại đúng
:::

:::opt
Không — một khi một điểm đã bị gán vào một cụm SAI, k-means không có cơ
chế nào để SỬA lại gán đó ở vòng sau
::why
Gần đúng ở việc gán cụm ở `vòng 1` đúng là SAI (so với cấu trúc thật của
dữ liệu) — quan sát đó không sai.

Chỗ lệch: k-means không hề "khoá" một gán cụm lại vĩnh viễn — MỖI vòng lặp
đều tính lại khoảng cách từ ĐẦU, dựa trên tâm HIỆN TẠI (đã cập nhật), không
dựa trên gán cụm của vòng trước. Vì tâm cụm `2` đã dịch chuyển mạnh sau
vòng 1 (không còn trùng `(1,2)` nữa), khoảng cách từ điểm này tới tâm cụm
`0` (`[1.0, 1.0]`) giờ NHỎ hơn hẳn khoảng cách tới tâm cụm `2` đã dịch xa —
gán cụm ở vòng 2 hoàn toàn có thể (và thực sự) đổi khác.
::
:::

:::opt
Có, nhưng phải mất ÍT NHẤT ba vòng nữa mới sửa được, vì mỗi vòng tâm chỉ
dịch được một khoảng rất nhỏ
::why
Gần đúng ở việc bạn hiểu ĐÚNG rằng tâm dịch chuyển DẦN qua từng vòng, không
nhảy thẳng một bước tới đáp án cuối — cơ chế lặp đó có thật.

Chỗ lệch: không có gì đảm bảo mỗi bước chỉ dịch "một khoảng rất nhỏ" — số
liệu thật cho thấy tâm cụm `2` dịch từ `[1.0, 2.0]` (khởi tạo) sang
`[3.5, 6.75]` NGAY SAU vòng `1` — một bước dịch RẤT LỚN, đủ để điểm `(1,2)`
đổi cụm ngay ở vòng `2` kế tiếp, không cần đợi thêm ba vòng nào nữa.
::
:::
::::

::::code{#viet_vong_lap_k_means}
Hoàn thiện `gan_cum` (khoảng cách Euclid vector hoá, đã học ở
`k-nn-phan-loai`) và `cap_nhat_tam` (tâm mới là trung bình các điểm đã
gán), rồi kiểm tra hội tụ sau ba vòng lặp.

```python title=starter
import numpy as np

X = np.array([
    [1, 1], [1, 2], [2, 1],
    [8, 1], [9, 1], [8, 2],
    [4, 8], [5, 8], [4, 9],
], dtype=float)
n, K = len(X), 3

def gan_cum(X, tam):
    kc = np.zeros((len(X), K))
    for k in range(K):
        kc[:, k] = ___                  # khoang cach Euclid tu moi diem X toi tam[k]
    return np.argmin(kc, axis=1)

def cap_nhat_tam(X, nhan):
    tam_moi = np.zeros((K, X.shape[1]))
    for k in range(K):
        tam_moi[k] = ___                # trung binh toa do cac diem co nhan == k
    return tam_moi

tam = np.array([[1.0, 1.0], [2.0, 1.0], [1.0, 2.0]])
hoi_tu = False
for vong in range(1, 4):
    nhan = gan_cum(X, tam)
    tam_moi = cap_nhat_tam(X, nhan)
    hoi_tu = ___                        # tam_moi va tam co "gan bang nhau" khong (np.allclose)
    tam = tam_moi

print(nhan.tolist())
print(np.round(tam, 4).tolist())
print(hoi_tu)
```

```python title=solution
import numpy as np

X = np.array([
    [1, 1], [1, 2], [2, 1],
    [8, 1], [9, 1], [8, 2],
    [4, 8], [5, 8], [4, 9],
], dtype=float)
n, K = len(X), 3

def gan_cum(X, tam):
    kc = np.zeros((len(X), K))
    for k in range(K):
        kc[:, k] = np.sqrt(np.sum((X - tam[k]) ** 2, axis=1))
    return np.argmin(kc, axis=1)

def cap_nhat_tam(X, nhan):
    tam_moi = np.zeros((K, X.shape[1]))
    for k in range(K):
        tam_moi[k] = X[nhan == k].mean(axis=0)
    return tam_moi

tam = np.array([[1.0, 1.0], [2.0, 1.0], [1.0, 2.0]])
hoi_tu = False
for vong in range(1, 4):
    nhan = gan_cum(X, tam)
    tam_moi = cap_nhat_tam(X, nhan)
    hoi_tu = np.allclose(tam_moi, tam)
    tam = tam_moi

print(nhan.tolist())
print(np.round(tam, 4).tolist())
print(hoi_tu)
```

```python title=test
assert nhan.tolist() == [0, 0, 0, 1, 1, 1, 2, 2, 2], f"gan cum cuoi cung sai -- dang ra {nhan.tolist()}"
assert np.round(tam, 4).tolist() == [[1.3333, 1.3333], [8.3333, 1.3333], [4.3333, 8.3333]], f"tam cuoi cung sai -- dang ra {np.round(tam, 4).tolist()}"
assert hoi_tu == True, "sau ba vong, thuat toan phai da HOI TU (tam khong doi nua)"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu là khoảng cách Euclid vector hoá đã học — `np.sqrt(np.sum((X - tam[k]) ** 2, axis=1))` (trừ MỘT tâm `tam[k]` khỏi MỌI điểm của `X` cùng lúc). Chỗ hai là trung bình các điểm ĐÃ GÁN vào cụm `k` — lọc bằng `X[nhan == k]` rồi `.mean(axis=0)`. Chỗ ba so sánh `tam_moi` với `tam` có "gần bằng nhau" hay không, dùng `np.allclose`.
- kind: strategy
  body: 'gan_cum: `np.sqrt(np.sum((X - tam[k]) ** 2, axis=1))`. cap_nhat_tam: `X[nhan == k].mean(axis=0)`. hoi_tu: `np.allclose(tam_moi, tam)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `np.sqrt(np.sum((X - tam[k]) ** 2, axis=1))`, `X[nhan == k].mean(axis=0)`, và `np.allclose(tam_moi, tam)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: gan_cum phai THAT SU khai can bac hai (np.sqrt) -- binh phuong khong khai can van xep dung thu tu gan cum nen de "an toan qua test" ma khong tinh dung khoang cach Euclid; cap_nhat_tam phai THAT SU loc theo nhan roi lay trung binh, khong duoc tra ve tam cu (se khien thuat toan khong bao gio cap nhat); hoi_tu phai goi THAT np.allclose, khong duoc luon la True
  requireAst:
  - kind: uses-call, target: sqrt, min: 1
  - kind: uses-name, target: nhan, min: 3
  - kind: uses-call, target: allclose, min: 1
  - kind: uses-name, target: tam_moi, min: 3
  # Da thu that (goi kiemAst that tren code day du):
  # - loi giai dung: dat=true, ca bon luat qua sach.
  # - bo sqrt (dung binh phuong khoang cach truc tiep -- van xep dung thu
  #   tu gan cum vi khai can dong bien, se qua tier tests/output neu khong
  #   co luat nay): sqrt tut xuong 0 -- duoi nguong 1, bi chan.
  # - cap_nhat_tam tra ve tam[k] thay vi trung binh moi (khong bao gio cap
  #   nhat): nhan tut tu 3 xuong 2 (mat lan dung ben trong ham nay) -- duoi
  #   nguong 3, bi chan.
  # - hoi_tu = True (chep cung, khong goi allclose): tam tut xuong 3
  #   (khong con dem duoc tuy chinh nhu tren) va allclose tut xuong 0 --
  #   duoi nguong 1, bi chan.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\[0, 0, 0, 1, 1, 1, 2, 2, 2\\]\\n\\[\\[1\\.3333, 1\\.3333\\], \\[8\\.3333, 1\\.3333\\], \\[4\\.3333, 8\\.3333\\]\\]\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không nhãn, không gradient, không learning rate — chỉ hai phép tính lặp
lại. Ba vòng là đủ để chín điểm tự sắp về đúng ba cụm của chúng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chín bài vừa qua đã đi qua rất nhiều cách khác nhau để phân loại và đánh
giá: logistic regression, softmax, k-NN, decision stump, và giờ là k-means
— một cách tiếp cận HOÀN TOÀN không cần nhãn. Nhưng track này còn thiếu
một mảnh: RÁP một pipeline phân loại ĐẦY ĐỦ, huấn luyện NHIỀU mô hình trên
CÙNG một bài toán, rồi dùng đúng những công cụ đánh giá đã học (ma trận
nhầm lẫn, precision/recall/F1) để trả lời câu hỏi thực tế nhất: mô hình
nào PHÙ HỢP HƠN cho bài toán cụ thể này?

Bài sau — BOSS của track — làm đúng việc đó.
::::

::::checkpoint{mastery=0.8}
::::
