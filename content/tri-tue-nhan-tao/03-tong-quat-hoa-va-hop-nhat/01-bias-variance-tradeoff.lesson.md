---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.bias-variance-tradeoff
title: "Bias-variance tradeoff"
summary: "Trên CÙNG một dữ liệu (10 điểm, quan hệ thật y=3+2x+0.4x², nhiễu ngẫu nhiên): bậc 1 (underfit) cho train=6.5512 và test=18.2286 — cả hai đều cao. Bậc 2 (đúng dạng) cho train=3.3783 và test=10.087 — cả hai đều thấp. Bậc 9 (overfit, 9 hệ số cho 10 điểm) cho train=0.0 nhưng test=19.6273 — train thấp nhưng test cao. Lặp lại qua 30 seed khác nhau: độ lệch chuẩn của test MSE là 2.9907 (bậc 1), 2.5883 (bậc 2), nhưng tới 97.3183 (bậc 9) — mô hình overfit dao động mạnh hơn hẳn theo từng lần chia dữ liệu."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.bias-variance-tradeoff]
requires: [ai.boss-phan-loai-va-danh-gia]
concepts: [ai.bias-variance-tradeoff]
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
Track trước ráp xong một pipeline phân loại hoàn chỉnh. Track này quay lại
đào sâu chính những công cụ đã có — bắt đầu bằng một câu hỏi cũ, giờ mới đo
được bằng số: một mô hình "vừa đủ tốt" khác một mô hình quá đơn giản hay quá
phức tạp CHÍNH XÁC ở đâu?
::::

::::explain{#hai-nguon-sai-so}
`overfitting-va-do-phuc-tap` (track hồi quy) đã cho thấy: một đa thức bậc
CAO có thể khớp train gần như hoàn hảo nhưng lại tệ trên test. Bài này đặt
tên chính xác cho HAI nguồn sai số làm nên hiện tượng đó, và đo cả hai bằng
số thật trên CÙNG một dữ liệu.

> **Bias** (thiên lệch) — sai số đến từ việc mô hình QUÁ ĐƠN GIẢN để nắm
> bắt quy luật thật. Một đường thẳng cố khớp một đường cong sẽ luôn lệch đi
> theo cùng một kiểu, dù có bao nhiêu dữ liệu train đi nữa — dấu hiệu: train
> error VÀ test error đều CAO, và gần bằng nhau.
>
> **Variance** (phương sai) — sai số đến từ việc mô hình QUÁ NHẠY với đúng
> tập dữ liệu train cụ thể nó thấy. Đổi một chút dữ liệu train (nhiễu khác,
> vài điểm khác), mô hình bậc cao cho ra một hàm số khác hẳn — dấu hiệu:
> train error THẤP nhưng test error CAO, và bản thân test error dao động
> mạnh nếu lặp lại với dữ liệu train khác.

Ba mô hình, cùng một dữ liệu, cho thấy cả ba trạng thái:

> **Underfit** (bậc quá THẤP) — bias cao: train error cao, test error cũng
> cao, hai con số gần nhau.
>
> **Vừa đủ** (bậc ĐÚNG dạng quan hệ thật) — cả bias lẫn variance đều thấp:
> train error thấp, test error cũng thấp.
>
> **Overfit** (bậc quá CAO) — variance cao: train error cực thấp (mô hình
> đủ tự do để luồn qua từng điểm) nhưng test error cao — VÀ, khác overfit đã
> thấy ở track trước, con số test error đó còn DAO ĐỘNG MẠNH nếu đổi seed
> nhiễu của dữ liệu train, vì mô hình bậc cao rất nhạy với đúng tập train nó
> thấy.

Đây chính là **tradeoff**: giảm bậc để giảm variance thường làm bias tăng
lên; tăng bậc để giảm bias thường làm variance tăng lên. Không có bậc nào
xoá sạch cả hai cùng lúc — mục tiêu là tìm điểm cân bằng, không phải điểm
cực trị của một trong hai.
::::

::::example{#ba-bac-mot-lan-chia}
Mười điểm train (`x = 1..10`), quan hệ thật là `y = 3 + 2x + 0.4x²` (một
đường CONG nhẹ, không phải đường thẳng) cộng nhiễu ngẫu nhiên. Chín điểm
test riêng biệt (`x = 1.5, 2.5, ..., 9.5`, nhiễu khác). Khớp ba bậc đa thức
— `1` (thẳng, quá đơn giản cho quan hệ có độ cong), `2` (đúng dạng), `9`
(chín hệ số cho mười điểm, gần như đủ luồn qua từng điểm):

```python title=readonly
import numpy as np

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_bac(x_tr, y_tr, bac):
    X_tho = dac_trung(x_tr, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(x_tr),1))])
    he_so, *_ = np.linalg.lstsq(A, y_tr, rcond=None)
    return he_so[:-1], he_so[-1], tb, sd

def mse_bac(bac, w, b, tb, sd, xs, ys):
    Xc = (dac_trung(xs, bac) - tb) / sd
    return np.mean((ys - (Xc @ w + b)) ** 2)

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

x_test = np.array([1.5,2.5,3.5,4.5,5.5,6.5,7.5,8.5,9.5])
rng_test = np.random.default_rng(999)
y_test = true_fn(x_test) + rng_test.normal(0, 3.0, size=len(x_test))

x_train = np.arange(1, 11, dtype=float)
rng0 = np.random.default_rng(0)
y_train = true_fn(x_train) + rng0.normal(0, 3.0, size=10)

for bac in [1, 2, 9]:
    w, b, tb, sd = fit_bac(x_train, y_train, bac)
    mse_train = mse_bac(bac, w, b, tb, sd, x_train, y_train)
    mse_test = mse_bac(bac, w, b, tb, sd, x_test, y_test)
    print(f"bac={bac}: train={round(mse_train,4)} test={round(mse_test,4)}")
```

```text title=readonly
bac=1: train=6.5512 test=18.2286
bac=2: train=3.3783 test=10.087
bac=9: train=0.0 test=19.6273
```

Bậc `1`: train `6.5512`, test `18.2286` — cả hai đều CAO, và ở cùng bậc độ
lớn (một đường thẳng không nắm được độ cong của quan hệ thật, nên sai lệch
xuất hiện đều trên cả hai tập — bias). Bậc `2`: train `3.3783`, test
`10.087` — cả hai đều THẤP hơn hẳn bậc `1` (đúng dạng quan hệ thật, khớp
tốt cả trên dữ liệu đã thấy lẫn chưa thấy). Bậc `9`: train `0.0` (gần như
tuyệt đối — chín hệ số cho mười điểm) nhưng test `19.6273` — CAO hơn cả bậc
`1`, dù train của nó thấp nhất trong ba mô hình. Đây đúng dấu hiệu variance:
mô hình luồn qua từng điểm train (kể cả phần nhiễu), nhưng cách luồn đó
không hề khớp quy luật thật, nên tệ hẳn trên dữ liệu chưa thấy.
::::

::::example{#dao-dong-qua-nhieu-lan-chia}
Bậc `9` có test error cao hơn bậc `1` ở LẦN CHIA cụ thể này — nhưng con số
đó có ỔN ĐỊNH không, nếu lặp lại với một tập train KHÁC (cùng quy luật
sinh dữ liệu, chỉ đổi nhiễu ngẫu nhiên)? Lặp lại toàn bộ quy trình khớp +
đo test MSE qua `30` seed nhiễu khác nhau, với CÙNG một tập test cố định ở
trên:

```python title=readonly
import numpy as np

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_bac(x_tr, y_tr, bac):
    X_tho = dac_trung(x_tr, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(x_tr),1))])
    he_so, *_ = np.linalg.lstsq(A, y_tr, rcond=None)
    return he_so[:-1], he_so[-1], tb, sd

def mse_bac(bac, w, b, tb, sd, xs, ys):
    Xc = (dac_trung(xs, bac) - tb) / sd
    return np.mean((ys - (Xc @ w + b)) ** 2)

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

x_test = np.array([1.5,2.5,3.5,4.5,5.5,6.5,7.5,8.5,9.5])
rng_test = np.random.default_rng(999)
y_test = true_fn(x_test) + rng_test.normal(0, 3.0, size=len(x_test))

x_train = np.arange(1, 11, dtype=float)

def test_mse_qua_cac_seed(bac, so_seed=30):
    ds = []
    for seed in range(so_seed):
        rng = np.random.default_rng(seed)
        y_tr = true_fn(x_train) + rng.normal(0, 3.0, size=10)
        w, b, tb, sd = fit_bac(x_train, y_tr, bac)
        ds.append(mse_bac(bac, w, b, tb, sd, x_test, y_test))
    return np.array(ds)

for bac in [1, 2, 9]:
    diem = test_mse_qua_cac_seed(bac)
    print(f"bac={bac}: trung_binh={round(diem.mean(),4)} std={round(diem.std(),4)}")
```

```text title=readonly
bac=1: trung_binh=18.8058 std=2.9907
bac=2: trung_binh=8.7107 std=2.5883
bac=9: trung_binh=100.3702 std=97.3183
```

Bậc `2` không chỉ có test error TRUNG BÌNH thấp nhất (`8.7107`) — độ lệch
chuẩn của nó (`2.5883`) cũng nằm cùng mức với bậc `1` (`2.9907`). Bậc `9` có
độ lệch chuẩn `97.3183` — cao gấp khoảng `33` lần bậc `2`, gần bằng chính
giá trị trung bình của nó (`100.3702`). Nói cách khác: kết quả của bậc `9`
có lần ra rất tệ, có lần đỡ hơn nhiều, tuỳ HOÀN TOÀN vào đúng nhiễu ngẫu
nhiên rơi vào tập train — một mô hình mà kết quả cuối cùng phụ thuộc nặng
vào may rủi của dữ liệu train là một mô hình có variance cao, bất kể trung
bình của nó trông ra sao.
::::

::::predict{#doan-mo-hinh-on-dinh-nhat commitOnce}
Nhìn lại ba độ lệch chuẩn: bậc `1` là `2.9907`, bậc `2` là `2.5883`, bậc `9`
là `97.3183`.

**Trước khi đọc lại**, bạn đoán: nếu thêm một bậc `THỨ TƯ` vào phép so
sánh — bậc `5` (không quá thấp như bậc `1`, không cực đoan như bậc `9`) —
độ lệch chuẩn của nó nhiều khả năng sẽ nằm ở đâu, so với ba con số đã có?

:::opt{correct}
Ở giữa — cao hơn bậc `1` và bậc `2` (có nhiều hệ số hơn, nhạy hơn với dữ
liệu train), nhưng thấp hơn hẳn bậc `9` (chưa "no" hệ số tới mức luồn qua
gần hết từng điểm train)
:::

:::opt
Thấp hơn cả bậc `1` — bậc càng cao thì mô hình càng "mạnh", nên hẳn phải ổn
định hơn các bậc thấp
::why
Gần đúng ở việc bậc cao hơn đúng là có nhiều hệ số tự do hơn — một quan sát
không sai về mặt CẤU TRÚC.

Chỗ lệch: "nhiều hệ số tự do hơn" đi cùng NHIỀU khả năng luồn theo nhiễu
hơn, không phải ổn định hơn. Chiều đúng, đo được bằng số của track này: bậc
càng cao (gần số điểm dữ liệu), độ lệch chuẩn của test error càng có xu
hướng TĂNG, không giảm — đúng hướng ngược với "mạnh hơn nên ổn định hơn".
::
:::

:::opt
Bằng đúng bậc `9` — vì cả hai đều là "đa thức bậc cao" nên phải dao động
tương đương nhau
::why
Gần đúng ở việc bậc `5` và bậc `9` cùng thuộc nhóm "có nhiều hệ số hơn bậc
`1`, `2`" — một cách phân loại thô không sai hoàn toàn.

Chỗ lệch: mức độ variance không nhảy vọt đột ngột ở một ranh giới cố định —
nó tăng DẦN theo số hệ số tự do so với số điểm dữ liệu. Bậc `9` (chín hệ số
cho mười điểm, gần bão hoà hoàn toàn) là một trường hợp CỰC ĐOAN hơn hẳn
bậc `5` (còn nhiều điểm dữ liệu "thừa" hơn so với số hệ số) — hai bậc không
đứng chung một mức.
::
:::
::::

::::code{#do_bias_variance_ba_bac}
Hoàn thiện phần đo test MSE tại một lần chia (`mse_test`), phần đo độ dao
động qua nhiều seed (`std_theo_bac`), rồi xác định bậc có độ lệch chuẩn LỚN
NHẤT trong ba bậc.

```python title=starter
import numpy as np

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_bac(x_tr, y_tr, bac):
    X_tho = dac_trung(x_tr, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(x_tr),1))])
    he_so, *_ = np.linalg.lstsq(A, y_tr, rcond=None)
    return he_so[:-1], he_so[-1], tb, sd

def mse_bac(bac, w, b, tb, sd, xs, ys):
    Xc = (dac_trung(xs, bac) - tb) / sd
    return np.mean((ys - (Xc @ w + b)) ** 2)

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

x_test = np.array([1.5,2.5,3.5,4.5,5.5,6.5,7.5,8.5,9.5])
rng_test = np.random.default_rng(999)
y_test = true_fn(x_test) + rng_test.normal(0, 3.0, size=len(x_test))

x_train = np.arange(1, 11, dtype=float)
rng0 = np.random.default_rng(0)
y_train = true_fn(x_train) + rng0.normal(0, 3.0, size=10)

ket_qua_mot_lan = {}
for bac in [1, 2, 9]:
    w, b, tb, sd = fit_bac(x_train, y_train, bac)
    mse_train = mse_bac(bac, w, b, tb, sd, x_train, y_train)
    mse_test = ___                          # mse_bac(bac, w, b, tb, sd, x_test, y_test)
    ket_qua_mot_lan[bac] = (mse_train, mse_test)

def test_mse_qua_cac_seed(bac, so_seed=30):
    ds = []
    for seed in range(so_seed):
        rng = np.random.default_rng(seed)
        y_tr = true_fn(x_train) + rng.normal(0, 3.0, size=10)
        w, b, tb, sd = fit_bac(x_train, y_tr, bac)
        ds.append(mse_bac(bac, w, b, tb, sd, x_test, y_test))
    return np.array(ds)

std_theo_bac = {}
for bac in [1, 2, 9]:
    diem = test_mse_qua_cac_seed(bac)
    std_theo_bac[bac] = ___                 # diem.std()

bac_dao_dong_manh_nhat = ___                 # bac co std LON NHAT trong std_theo_bac

print(round(ket_qua_mot_lan[1][0], 4), round(ket_qua_mot_lan[1][1], 4))
print(round(ket_qua_mot_lan[2][0], 4), round(ket_qua_mot_lan[2][1], 4))
print(round(ket_qua_mot_lan[9][0], 4), round(ket_qua_mot_lan[9][1], 4))
print(round(std_theo_bac[1], 4), round(std_theo_bac[2], 4), round(std_theo_bac[9], 4))
print(bac_dao_dong_manh_nhat)
```

```python title=solution
import numpy as np

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def fit_bac(x_tr, y_tr, bac):
    X_tho = dac_trung(x_tr, bac)
    tb, sd = X_tho.mean(axis=0), X_tho.std(axis=0)
    A = np.hstack([(X_tho - tb) / sd, np.ones((len(x_tr),1))])
    he_so, *_ = np.linalg.lstsq(A, y_tr, rcond=None)
    return he_so[:-1], he_so[-1], tb, sd

def mse_bac(bac, w, b, tb, sd, xs, ys):
    Xc = (dac_trung(xs, bac) - tb) / sd
    return np.mean((ys - (Xc @ w + b)) ** 2)

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

x_test = np.array([1.5,2.5,3.5,4.5,5.5,6.5,7.5,8.5,9.5])
rng_test = np.random.default_rng(999)
y_test = true_fn(x_test) + rng_test.normal(0, 3.0, size=len(x_test))

x_train = np.arange(1, 11, dtype=float)
rng0 = np.random.default_rng(0)
y_train = true_fn(x_train) + rng0.normal(0, 3.0, size=10)

ket_qua_mot_lan = {}
for bac in [1, 2, 9]:
    w, b, tb, sd = fit_bac(x_train, y_train, bac)
    mse_train = mse_bac(bac, w, b, tb, sd, x_train, y_train)
    mse_test = mse_bac(bac, w, b, tb, sd, x_test, y_test)
    ket_qua_mot_lan[bac] = (mse_train, mse_test)

def test_mse_qua_cac_seed(bac, so_seed=30):
    ds = []
    for seed in range(so_seed):
        rng = np.random.default_rng(seed)
        y_tr = true_fn(x_train) + rng.normal(0, 3.0, size=10)
        w, b, tb, sd = fit_bac(x_train, y_tr, bac)
        ds.append(mse_bac(bac, w, b, tb, sd, x_test, y_test))
    return np.array(ds)

std_theo_bac = {}
for bac in [1, 2, 9]:
    diem = test_mse_qua_cac_seed(bac)
    std_theo_bac[bac] = diem.std()

bac_dao_dong_manh_nhat = max(std_theo_bac, key=std_theo_bac.get)

print(round(ket_qua_mot_lan[1][0], 4), round(ket_qua_mot_lan[1][1], 4))
print(round(ket_qua_mot_lan[2][0], 4), round(ket_qua_mot_lan[2][1], 4))
print(round(ket_qua_mot_lan[9][0], 4), round(ket_qua_mot_lan[9][1], 4))
print(round(std_theo_bac[1], 4), round(std_theo_bac[2], 4), round(std_theo_bac[9], 4))
print(bac_dao_dong_manh_nhat)
```

```python title=test
assert round(ket_qua_mot_lan[1][0], 4) == 6.5512, f"train mse bac 1 phai la 6.5512 -- dang ra {round(ket_qua_mot_lan[1][0], 4)}"
assert round(ket_qua_mot_lan[1][1], 4) == 18.2286, f"test mse bac 1 phai la 18.2286 -- dang ra {round(ket_qua_mot_lan[1][1], 4)}"
assert round(ket_qua_mot_lan[2][0], 4) == 3.3783, f"train mse bac 2 phai la 3.3783 -- dang ra {round(ket_qua_mot_lan[2][0], 4)}"
assert round(ket_qua_mot_lan[2][1], 4) == 10.087, f"test mse bac 2 phai la 10.087 -- dang ra {round(ket_qua_mot_lan[2][1], 4)}"
assert round(ket_qua_mot_lan[9][0], 4) == 0.0, f"train mse bac 9 phai gan bang 0 -- dang ra {round(ket_qua_mot_lan[9][0], 4)}"
assert round(ket_qua_mot_lan[9][1], 4) == 19.6273, f"test mse bac 9 phai la 19.6273 -- dang ra {round(ket_qua_mot_lan[9][1], 4)}"
assert round(std_theo_bac[1], 4) == 2.9907, f"std bac 1 phai la 2.9907 -- dang ra {round(std_theo_bac[1], 4)}"
assert round(std_theo_bac[2], 4) == 2.5883, f"std bac 2 phai la 2.5883 -- dang ra {round(std_theo_bac[2], 4)}"
assert round(std_theo_bac[9], 4) == 97.3183, f"std bac 9 phai la 97.3183 -- dang ra {round(std_theo_bac[9], 4)}"
assert bac_dao_dong_manh_nhat == 9, f"bac dao dong manh nhat phai la 9 -- dang ra {bac_dao_dong_manh_nhat}"
assert std_theo_bac[9] > std_theo_bac[1] and std_theo_bac[9] > std_theo_bac[2], "std cua bac 9 phai CAO HON han ca bac 1 lan bac 2"
```

:::hints
- kind: attention
  body: Ba chỗ trống. `mse_test` gọi lại đúng hàm `mse_bac` đã dùng để tính `mse_train` ngay dòng trên, chỉ đổi `xs, ys` thành `x_test, y_test`. `std_theo_bac[bac]` là `diem.std()` — độ lệch chuẩn của mảng `diem` (mảng `30` test MSE, một cho mỗi seed). `bac_dao_dong_manh_nhat` tìm KHOÁ có GIÁ TRỊ lớn nhất trong dict `std_theo_bac` — dùng `max(...)` với tham số `key`, cùng khuôn với cách bài `train-val-test-split` từng chọn bậc qua `min(...)`.
- kind: strategy
  body: 'mse_test: `mse_bac(bac, w, b, tb, sd, x_test, y_test)`. std_theo_bac[bac]: `diem.std()`. bac_dao_dong_manh_nhat: `max(std_theo_bac, key=std_theo_bac.get)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `mse_bac(bac, w, b, tb, sd, x_test, y_test)`, `diem.std()`, và `max(std_theo_bac, key=std_theo_bac.get)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: mse_test phai goi THAT mse_bac tren x_test/y_test (khong duoc chep lai mse_train hay mot con so co san); std_theo_bac[bac] phai goi THAT .std() tren diem; bac_dao_dong_manh_nhat phai tinh THAT bang max(...) tren std_theo_bac, khong duoc chep san 9
  requireAst:
  - kind: uses-call, target: mse_bac, min: 3
  - kind: uses-call, target: std, min: 2
  - kind: uses-call, target: max, min: 1
  - kind: uses-name, target: std_theo_bac, min: 3
  # Da thu that (goi kiemAst that tren code day du, trich tu chinh khoi
  # solution): loi giai dung dat=true, ca bon luat qua sach. Cheat (mse_test
  # = mse_train, std_theo_bac[bac] = 1.0, bac_dao_dong_manh_nhat = 9 chep
  # san) lam CA BON luat cung roi xuong duoi nguong (mse_bac mat 1 lan goi,
  # std mat 1 lan goi, max bien mat hoan toan, std_theo_bac mat hai lan xuat
  # hien trong dong max(...)) -- bi chan boi ca bon.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^6\\.5512 18\\.2286\\n3\\.3783 10\\.087\\n0\\.0 19\\.6273\\n2\\.9907 2\\.5883 97\\.3183\\n9\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Độ lệch chuẩn `97.3183` so với `2.5883` — không phải một con số trừu tượng.
Đó là bằng chứng bằng số: mô hình overfit không chỉ "tệ hơn" trên trung
bình, nó còn KHÔNG ĐÁNG TIN, vì kết quả của nó đổi mạnh theo đúng may rủi
của dữ liệu train.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bậc `9` dao động mạnh vì nó quá NHẠY với đúng tập train nó thấy — đổi nhiễu
là đổi cả mô hình. Nhưng nếu, thay vì chỉ khớp MỘT mô hình bậc `9` trên một
tập train, ta khớp NHIỀU mô hình bậc `9` (mỗi cái trên một PHIÊN BẢN hơi
khác của cùng dữ liệu train), rồi lấy TRUNG BÌNH dự đoán của tất cả chúng —
liệu variance của kết quả trung bình đó có thấp hơn variance của một mô
hình đơn lẻ không?

Bài sau bắt đầu từ đúng câu hỏi "tạo ra nhiều phiên bản hơi khác của cùng
dữ liệu" đó.
::::

::::checkpoint{mastery=0.8}
::::
