---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.decision-tree-don-gian
title: "Decision tree đơn giản: một lần chia"
summary: "Trên tám email của bài logistic-regression-tu-so-0 (số từ viết hoa [0,1,2,3,5,6,7,9], nhãn spam [0,0,0,1,1,1,1,1]): entropy gốc H=0.9544 bit. Thử bảy ngưỡng chia khác nhau — ngưỡng x=4.0 chia CÂN BẰNG (4 email mỗi bên) nhưng chỉ đạt information gain=0.5488; ngưỡng x=2.5 chia LỆCH (3 và 5 email) nhưng CẢ HAI bên đều thuần một lớp (entropy=0.0 cả hai), đạt gain=0.9544 — đúng bằng entropy gốc, mức TỐI ĐA có thể — và trùng khớp với vùng ranh giới quyết định (~2.32) mà logistic regression đã tìm ra bằng một cơ chế hoàn toàn khác."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.decision-tree]
requires: [ai.k-nn]
concepts: [ai.decision-tree]
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
k-NN đo khoảng cách. Đây là cách khác hẳn: đặt ĐÚNG một câu hỏi ngưỡng,
rồi chia dữ liệu làm hai theo câu trả lời.
::::

::::explain{#do-do-hon-don}
Một **decision tree** (cây quyết định) học bằng cách đặt ra các câu hỏi
dạng "đặc trưng này có LỚN HƠN một ngưỡng nào đó không?", rồi chia dữ liệu
thành hai nhánh theo câu trả lời (có/không). Bài này chỉ dừng ở MỘT lần
chia duy nhất — gọi là **decision stump** (gốc cây, một cấp) — đủ để học
cơ chế CHỌN NGƯỠNG mà không cần đệ quy xây cả cây.

Câu hỏi cốt lõi: trong số MỌI ngưỡng có thể chọn, ngưỡng nào "tốt nhất"?
Cần một cách ĐO độ "thuần" (hay "hỗn độn") của một tập nhãn. **Entropy**
(lấy từ lý thuyết thông tin) đo đúng điều đó:

> `H = -Σₖ pₖ · log₂(pₖ)`

với `pₖ` là tỉ lệ nhãn lớp `k` trong tập. Nếu MỌI điểm trong tập cùng một
lớp (tập THUẦN), `H = 0` — không còn gì "bất ngờ" để hỏi. Nếu các lớp chia
đều 50/50, `H` đạt GIÁ TRỊ LỚN NHẤT có thể với hai lớp (`H = 1` bit) — hỗn
độn tối đa, đoán lớp nào cũng như tung đồng xu.

Chia dữ liệu tại một ngưỡng `t` (đặc trưng `< t` vào nhánh trái, `≥ t` vào
nhánh phải) làm giảm entropy TRUNG BÌNH (có trọng số theo kích thước mỗi
nhánh) so với entropy TRƯỚC khi chia. Mức giảm đó gọi là **information
gain**:

> `IG(t) = H_gốc − [ (n_trái/n)·H_trái + (n_phải/n)·H_phải ]`

Ngưỡng "tốt nhất" là ngưỡng có `IG` LỚN NHẤT — chia sao cho MỖI nhánh, xét
trung bình có trọng số, thuần nhất có thể. Chú ý: `IG` lớn không đòi hỏi
hai nhánh phải CÂN BẰNG về SỐ LƯỢNG điểm — một ngưỡng chia lệch (`3` điểm
một bên, `5` điểm bên kia) hoàn toàn có thể có `IG` cao HƠN một ngưỡng
chia đều (`4`/`4`), miễn là hai nhánh của ngưỡng lệch đó THUẦN hơn hẳn.
::::

::::example{#tim-nguong-chia-tot-nhat}
Tám email của bài `logistic-regression-tu-so-0` — số từ viết hoa và nhãn
spam — thử BẢY ngưỡng chia khác nhau (mỗi ngưỡng nằm giữa hai giá trị liên
tiếp của dữ liệu đã sắp xếp: `0, 1, 2, 3, 5, 6, 7, 9`):

```python title=readonly
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
la_spam = np.array([0, 0, 0, 1, 1, 1, 1, 1])
n = len(la_spam)

def entropy(nhan):
    if len(nhan) == 0:
        return 0.0
    ent = 0.0
    for lop in np.unique(nhan):
        p = np.sum(nhan == lop) / len(nhan)
        if p > 0:
            ent -= p * np.log2(p)
    return ent

H_goc = entropy(la_spam)
print("entropy goc:", round(H_goc, 4))

nguong_list = [0.5, 1.5, 2.5, 4.0, 5.5, 6.5, 8.0]
for ng in nguong_list:
    trai = la_spam[so_tu_hoa < ng]
    phai = la_spam[so_tu_hoa >= ng]
    H_trai, H_phai = entropy(trai), entropy(phai)
    H_sau = (len(trai) / n) * H_trai + (len(phai) / n) * H_phai
    gain = H_goc - H_sau
    print(f"nguong={ng}: n_trai={len(trai)} n_phai={len(phai)}  H_trai={round(H_trai,4)} H_phai={round(H_phai,4)}  gain={round(gain,4)}")
```

```text title=readonly
entropy goc: 0.9544
nguong=0.5: n_trai=1 n_phai=7  H_trai=0.0 H_phai=0.8631  gain=0.1992
nguong=1.5: n_trai=2 n_phai=6  H_trai=0.0 H_phai=0.65  gain=0.4669
nguong=2.5: n_trai=3 n_phai=5  H_trai=0.0 H_phai=0.0  gain=0.9544
nguong=4.0: n_trai=4 n_phai=4  H_trai=0.8113 H_phai=0.0  gain=0.5488
nguong=5.5: n_trai=5 n_phai=3  H_trai=0.971 H_phai=0.0  gain=0.3476
nguong=6.5: n_trai=6 n_phai=2  H_trai=1.0 H_phai=0.0  gain=0.2044
nguong=8.0: n_trai=7 n_phai=1  H_trai=0.9852 H_phai=0.0  gain=0.0924
```

Ngưỡng `x = 4.0` chia CÂN BẰNG nhất về số lượng (`4` email mỗi bên) —
nhưng nhánh trái vẫn còn LẪN LỘN (`H_trái = 0.8113`, ba nhãn `0` và một
nhãn `1`), cho `gain = 0.5488`. Ngưỡng `x = 2.5` chia LỆCH hẳn (`3` và `5`)
nhưng CẢ HAI nhánh đều THUẦN TUYỆT ĐỐI (`H_trái = H_phải = 0.0` — nhánh
trái toàn nhãn `0`, nhánh phải toàn nhãn `1`) — `gain = 0.9544`, ĐÚNG BẰNG
entropy gốc, mức tối đa lý thuyết có thể đạt (chia hoàn hảo thì không còn
entropy nào sau khi chia, `gain = H_gốc − 0 = H_gốc`). Ngưỡng `2.5` thắng
áp đảo, dù không phải ngưỡng chia CÂN BẰNG nhất về số lượng.

Điều thú vị: ngưỡng `2.5` này nằm ngay TRONG khoảng ranh giới quyết định
mà `logistic-regression-tu-so-0`/`ranh-gioi-quyet-dinh` đã tìm được (`x ≈
2.3239`) — bằng một cơ chế HOÀN TOÀN khác (gradient descent trên
cross-entropy, không phải entropy rời rạc trên vài ngưỡng ứng viên). Hai
phương pháp độc lập, cùng một kết luận về VÙNG phân tách của dữ liệu này.
::::

::::predict{#doan-nguong-thang commitOnce}
Nhìn lại hai ngưỡng: `x = 4.0` (chia CÂN BẰNG `4`/`4` điểm) và `x = 2.5`
(chia LỆCH `3`/`5` điểm).

**Trước khi đọc lại bảng**, bạn đoán: ngưỡng nào có `information gain` CAO
HƠN?

:::opt{correct}
`x = 2.5` — dù chia lệch về số lượng, cả hai nhánh của nó đều THUẦN TUYỆT
ĐỐI (entropy `0.0` cả hai bên), trong khi nhánh trái của `x = 4.0` vẫn còn
lẫn cả hai lớp
:::

:::opt
`x = 4.0` — chia cân bằng số lượng luôn là dấu hiệu của một ngưỡng chia
tốt, bất kể nội dung từng nhánh ra sao
::why
Gần đúng ở việc "chia cân bằng số lượng" NGHE có vẻ công bằng, và trực
giác đó không phải lúc nào cũng sai trong MỌI bài toán.

Chỗ lệch: information gain đo độ THUẦN của từng nhánh SAU khi chia, không
đo việc hai nhánh có bằng nhau về SỐ LƯỢNG điểm hay không. Ngưỡng `x = 4.0`
chia đúng `4`/`4` nhưng nhánh trái vẫn lẫn lộn (`H_trái = 0.8113`, ba nhãn
`0` lẫn một nhãn `1`) — số liệu thật: `gain(4.0) = 0.5488`, THẤP hơn hẳn
`gain(2.5) = 0.9544`, dù `2.5` chia lệch hẳn (`3`/`5`).
::
:::

:::opt
Cả hai bằng nhau — vì cả hai ngưỡng đều nằm trong cùng phạm vi dữ liệu và
đều tạo ra hai nhánh không rỗng
::why
Gần đúng ở việc CẢ HAI ngưỡng đều hợp lệ theo nghĩa tạo ra hai nhánh không
rỗng — quan sát đó đúng nhưng không đủ.

Chỗ lệch: "hợp lệ" (không rỗng) không có nghĩa "cho cùng gain". Gain phụ
thuộc NỘI DUNG nhãn bên trong mỗi nhánh — hai ngưỡng khác nhau chia dữ liệu
thành hai cặp nhánh có mức độ THUẦN khác hẳn nhau, và số liệu thật (`0.9544`
so với `0.5488`) cho thấy chênh lệch đó không hề nhỏ.
::
:::
::::

::::code{#tim_nguong_gain_cao_nhat}
Hoàn thiện `entropy` (công thức entropy đã học) và vòng lặp tính `gain`
cho từng ngưỡng, rồi chọn ngưỡng có `gain` LỚN NHẤT.

```python title=starter
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
la_spam = np.array([0, 0, 0, 1, 1, 1, 1, 1])
n = len(la_spam)

def entropy(nhan):
    if len(nhan) == 0:
        return 0.0
    ent = 0.0
    for lop in np.unique(nhan):
        p = np.sum(nhan == lop) / len(nhan)
        if p > 0:
            ___                          # ent -= p * log2(p)
    return ent

H_goc = entropy(la_spam)

nguong_list = [0.5, 1.5, 2.5, 4.0, 5.5, 6.5, 8.0]
gains = []
for ng in nguong_list:
    trai = la_spam[so_tu_hoa < ng]
    phai = la_spam[so_tu_hoa >= ng]
    H_sau = ___                          # trung binh co trong so cua entropy(trai) va entropy(phai)
    gains.append(H_goc - H_sau)

nguong_tot_nhat = ___                    # nguong co gain LON NHAT trong nguong_list

print(round(H_goc, 4))
print(round(max(gains), 4))
print(nguong_tot_nhat)
```

```python title=solution
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
la_spam = np.array([0, 0, 0, 1, 1, 1, 1, 1])
n = len(la_spam)

def entropy(nhan):
    if len(nhan) == 0:
        return 0.0
    ent = 0.0
    for lop in np.unique(nhan):
        p = np.sum(nhan == lop) / len(nhan)
        if p > 0:
            ent -= p * np.log2(p)
    return ent

H_goc = entropy(la_spam)

nguong_list = [0.5, 1.5, 2.5, 4.0, 5.5, 6.5, 8.0]
gains = []
for ng in nguong_list:
    trai = la_spam[so_tu_hoa < ng]
    phai = la_spam[so_tu_hoa >= ng]
    H_sau = (len(trai) / n) * entropy(trai) + (len(phai) / n) * entropy(phai)
    gains.append(H_goc - H_sau)

nguong_tot_nhat = nguong_list[np.argmax(gains)]

print(round(H_goc, 4))
print(round(max(gains), 4))
print(nguong_tot_nhat)
```

```python title=test
assert round(H_goc, 4) == 0.9544, f"entropy goc phai la 0.9544 -- dang ra {round(H_goc, 4)}"
assert round(max(gains), 4) == 0.9544, f"gain lon nhat phai la 0.9544 -- dang ra {round(max(gains), 4)}"
assert nguong_tot_nhat == 2.5, f"nguong tot nhat phai la 2.5 -- dang ra {nguong_tot_nhat}"
assert round(gains[3], 4) == 0.5488, f"gain cua nguong 4.0 (chia can bang 4/4) phai la 0.5488, THAP hon nguong 2.5 -- dang ra {round(gains[3], 4)}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu, bên trong `entropy`, là phép TRỪ TÍCH LUỸ đúng công thức entropy — `ent -= p * np.log2(p)`. Chỗ hai là `H_sau`, TRUNG BÌNH CÓ TRỌNG SỐ (theo tỉ lệ số điểm mỗi nhánh trên tổng `n`) của `entropy(trai)` và `entropy(phai)`. Chỗ ba chọn ngưỡng có `gain` (trong `gains`, đã tính từng ngưỡng) LỚN NHẤT — dùng `np.argmax` để tìm CHỈ SỐ, rồi tra ngược lại `nguong_list`.
- kind: strategy
  body: 'entropy: `ent -= p * np.log2(p)`. H_sau: `(len(trai) / n) * entropy(trai) + (len(phai) / n) * entropy(phai)`. nguong_tot_nhat: `nguong_list[np.argmax(gains)]`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `ent -= p * np.log2(p)`, `(len(trai) / n) * entropy(trai) + (len(phai) / n) * entropy(phai)`, và `nguong_list[np.argmax(gains)]`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: entropy phai THAT SU cong don -p*log2(p) cho tung lop; H_sau phai la trung binh CO TRONG SO theo n (khong duoc cong thang hai entropy khong nhan trong so, se lam sai lech gain khi hai nhanh khac kich thuoc); nguong_tot_nhat phai dung np.argmax tren gains that su, khong chep san 2.5
  requireAst:
  - kind: uses-name, target: H_goc, min: 2
  - kind: uses-name, target: n, min: 2
  - kind: uses-call, target: argmax, min: 1
  - kind: uses-name, target: gains, min: 3
  # Da thu that (goi kiemAst that tren code day du):
  # - loi giai dung: dat=true, ca bon luat qua sach.
  # - H_sau quen nhan trong so (= entropy(trai) + entropy(phai), khong chia
  #   cho n): n tut tu 2 xuong 1 (chi con o dong dinh nghia "n = len(...)")
  #   -- duoi nguong 2, bi chan.
  # - chon nguong bang np.argmin thay vi np.argmax: argmax tut xuong 0 --
  #   duoi nguong 1, bi chan.
  # - chep hang so nguong_tot_nhat = 2.5 (bo qua argmax hoan toan): argmax
  #   tut xuong 0, gains tut xuong 2 (mat lan dung trong dong chon nguong)
  #   -- duoi ca hai nguong, bi chan.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^0\\.9544\\n0\\.9544\\n2\\.5\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ngưỡng chia lệch hẳn (3/5) thắng ngưỡng chia đều (4/4) — vì thuần mới là
thứ information gain đo, không phải cân bằng số lượng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài này dừng lại ở MỘT lần chia — một decision stump. Một decision tree
THẬT SỰ lặp lại việc này: sau khi chia lần đầu, LẠI tìm ngưỡng tốt nhất
CHO TỪNG NHÁNH con, đệ quy, cho tới khi mỗi nhánh đủ thuần hoặc đạt một
điều kiện dừng nào đó. Với dữ liệu tám email này, một lần chia tại `x =
2.5` đã đủ THUẦN TUYỆT ĐỐI — không còn gì để chia thêm.

Mọi mô hình từ đầu track — logistic regression, softmax, k-NN, decision
stump — đều nhìn CHÍNH XÁC các điểm dữ liệu riêng lẻ để quyết định. Có một
cách tiếp cận khác hẳn: không hề có nhãn nào để học theo — chỉ có dữ liệu,
và mục tiêu là tự tìm ra CẤU TRÚC ẩn trong đó. Bài sau bước sang hướng đó.
::::

::::checkpoint{mastery=0.8}
::::
