---
id: tri-tue-nhan-tao.neuron-va-mang-nhieu-tang.khoi-tao-trong-so
title: "Khởi tạo trọng số"
summary: "Khởi tạo TOÀN BỘ trọng số bằng 0 cho một MLP 2 tầng (huấn luyện qua 5 bước bằng đạo hàm số/finite-difference trên dữ liệu OR): hai neuron ẩn ra CÙNG giá trị ở MỌI bước (dong_bo_moi_buoc = [True]*5) — đối xứng không bao giờ vỡ. Khởi tạo z ~ N(0,10) (lớn) cho độ lớn đạo hàm sigmoid trung bình 0.0415 — nhỏ hơn khởi tạo z ~ N(0,1) (0.2078) gần 5 lần, do bão hoà. Xavier: phương sai = 2/(n_vào+n_ra) = 0.3333 cho (4,2); He: phương sai = 2/n_vào = 0.5 cho n_vào=4."
locale: vi
track: tri-tue-nhan-tao
module: neuron-va-mang-nhieu-tang
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.khoi-tao-trong-so]
requires: [ai.mang-nhieu-tang-forward-pass]
concepts: [ai.khoi-tao-trong-so]
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
Bài trước dùng trọng số CHO SẴN. Nhưng huấn luyện thật phải bắt đầu từ đâu
đó — và điểm bắt đầu đó hoá ra quan trọng hơn tưởng tượng.
::::

::::explain{#hai-cai-bay-khoi-tao}
Trước khi huấn luyện, mọi trọng số của một mạng cần một giá trị khởi đầu.
Hai lựa chọn TƯỞNG như hợp lý lại đều có vấn đề:

> **Khởi tạo bằng `0` (hoặc bất kỳ hằng số NÀO giống nhau cho mọi neuron
> cùng tầng)** — mọi neuron trong CÙNG một tầng nhận ĐÚNG cùng trọng số,
> nên tính ra ĐÚNG cùng đầu ra cho MỌI đầu vào, và (điểm quan trọng hơn)
> nhận ĐÚNG cùng gradient ở mọi bước huấn luyện sau đó. Không có gì phân
> biệt được hai neuron — chúng học y hệt nhau MÃI MÃI, dù tầng có bao
> nhiêu neuron đi nữa. Đây gọi là vấn đề **đối xứng** (symmetry breaking
> thất bại): khởi tạo phải NGẪU NHIÊN và KHÁC NHAU giữa các neuron cùng
> tầng để chúng có cơ hội học những đặc trưng khác nhau.
>
> **Khởi tạo quá LỚN** (ví dụ lấy mẫu từ phân phối có độ lệch chuẩn lớn) —
> tổng có trọng số `z = w·x + b` có xu hướng ra giá trị `|z|` lớn ngay từ
> đầu. Bài `ham-kich-hoat` đã đo: `sigmoid` và `tanh` BÃO HOÀ khi `|z|`
> lớn — đạo hàm gần `0`. Gradient descent cập nhật theo đạo hàm; đạo hàm
> gần `0` nghĩa là gần như KHÔNG cập nhật gì, dù mô hình còn cách xa lời
> giải tốt.

Heuristic phổ biến để tránh cả hai cực đoan: chọn trọng số ngẫu nhiên,
NHỎ, với độ lớn tỉ lệ nghịch với số đầu vào của neuron đó — nhiều đầu vào
hơn thì mỗi trọng số nên nhỏ hơn, để tổng `z` không phình to.

> **Xavier/Glorot** (thường dùng với `sigmoid`/`tanh`): phương sai mỗi
> trọng số = `2 / (n_vào + n_ra)`, với `n_vào`/`n_ra` là số đầu vào/đầu ra
> của tầng đó.
>
> **He** (thường dùng với `ReLU`/`Leaky ReLU`): phương sai = `2 / n_vào`.

Track này KHÔNG chứng minh vì sao đúng hai công thức này — chỉ ghi nhận
chúng như quy ước đã được kiểm chứng rộng rãi, và đo được HAI cái bẫy ở
trên bằng số thật, ngay dưới đây.
::::

::::example{#doi_xung_khong_bao_gio_vo}
Một MLP nhỏ (`2` đầu vào, tầng ẩn `2` neuron, tầng ra `1` neuron), khởi tạo
TOÀN BỘ trọng số bằng `0`, huấn luyện trên dữ liệu **OR** (không đối xứng
như XOR, để phép huấn luyện thực sự di chuyển trọng số — đo bằng **đạo hàm
số** — finite difference — một cách xấp xỉ đạo hàm chỉ cần forward pass
lặp lại, KHÔNG cần backprop; q8.2b sẽ giới thiệu cách tính đạo hàm nhanh
hơn nhiều, nhưng phép xấp xỉ này đủ để quan sát ở đây):

```python title=readonly
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0.,1.,1.,1.])  # OR

def forward(X, W1, b1, w2, b2):
    H = sigmoid(X @ W1 + b1)
    out = sigmoid(H @ w2 + b2)
    return H, out

def mat_mat(flat):
    W1 = flat[:4].reshape(2, 2); b1 = flat[4:6]; w2 = flat[6:8]; b2 = flat[8]
    _, p = forward(X, W1, b1, w2, b2)
    eps = 1e-12
    return -np.mean(y * np.log(p + eps) + (1 - y) * np.log(1 - p + eps))

def dao_ham_so(flat, eps=1e-4):
    g = np.zeros_like(flat)
    for i in range(len(flat)):
        cong = flat.copy(); cong[i] += eps
        tru = flat.copy(); tru[i] -= eps
        g[i] = (mat_mat(cong) - mat_mat(tru)) / (2 * eps)
    return g

def gop(W1, b1, w2, b2):
    return np.concatenate([W1.ravel(), b1.ravel(), w2.ravel(), np.array([b2])])

def tach(flat):
    return flat[:4].reshape(2, 2), flat[4:6], flat[6:8], flat[8]

flat = gop(np.zeros((2, 2)), np.zeros(2), np.zeros(2), 0.0)
for buoc in range(5):
    g = dao_ham_so(flat)
    flat = flat - 2.0 * g
    W1, b1, w2, b2 = tach(flat)
    H, _ = forward(X, W1, b1, w2, b2)
    print(f"buoc {buoc}: h1={np.round(H[:,0],4).tolist()} h2={np.round(H[:,1],4).tolist()} giong_het_nhau={np.allclose(H[:,0], H[:,1])}")
```

```text title=readonly
buoc 0: h1=[0.5, 0.5, 0.5, 0.5] h2=[0.5, 0.5, 0.5, 0.5] giong_het_nhau=True
buoc 1: h1=[0.5022, 0.5072, 0.5072, 0.5122] h2=[0.5022, 0.5072, 0.5072, 0.5122] giong_het_nhau=True
buoc 2: h1=[0.5032, 0.5138, 0.5138, 0.5243] h2=[0.5032, 0.5138, 0.5138, 0.5243] giong_het_nhau=True
buoc 3: h1=[0.5036, 0.5198, 0.5198, 0.5359] h2=[0.5036, 0.5198, 0.5198, 0.5359] giong_het_nhau=True
buoc 4: h1=[0.5037, 0.5255, 0.5255, 0.5472] h2=[0.5037, 0.5255, 0.5255, 0.5472] giong_het_nhau=True
```

Trọng số KHÔNG đứng yên — cả hai neuron ẩn thay đổi giá trị qua mỗi bước
(`0.5 → 0.5022 → ... → 0.5037`), mạng CÓ học. Nhưng ở MỌI bước, `h1` và
`h2` — đầu ra của hai neuron ẩn KHÁC NHAU — luôn bằng nhau tuyệt đối. Hai
neuron này, dù huấn luyện bao lâu, sẽ không bao giờ tách ra học hai đặc
trưng khác nhau — về mặt chức năng, tầng ẩn `2` neuron này hoạt động y hệt
tầng ẩn CHỈ `1` neuron, lãng phí một nửa.
::::

::::example{#khoi_tao_qua_lon_bao_hoa}
So sánh độ lớn đạo hàm `sigmoid'` trung bình, khi `z` (tổng có trọng số)
được lấy mẫu từ hai thang khởi tạo khác nhau — độ lệch chuẩn `1.0` (nhỏ,
hợp lý) so với `10.0` (lớn gấp `10` lần):

```python title=readonly
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def sigmoid_dao_ham(z):
    s = sigmoid(z)
    return s * (1 - s)

rng_nho = np.random.default_rng(1)
Z_nho = rng_nho.normal(0, 1.0, size=1000)
rng_lon = np.random.default_rng(1)
Z_lon = rng_lon.normal(0, 10.0, size=1000)

dh_nho = np.mean(np.abs(sigmoid_dao_ham(Z_nho)))
dh_lon = np.mean(np.abs(sigmoid_dao_ham(Z_lon)))
print("do lon dao ham TB, khoi tao nho:", round(dh_nho, 4))
print("do lon dao ham TB, khoi tao lon:", round(dh_lon, 4))
```

```text title=readonly
do lon dao ham TB, khoi tao nho: 0.2078
do lon dao ham TB, khoi tao lon: 0.0415
```

Khởi tạo LỚN (`std=10`) cho độ lớn đạo hàm trung bình `0.0415` — chưa bằng
một phần năm mức `0.2078` của khởi tạo NHỎ (`std=1`). Cùng `1000` mẫu ngẫu
nhiên, cùng công thức `sigmoid'`, chỉ đổi độ LỚN của `z` đầu vào — kết quả
là gradient trung bình yếu đi rõ rệt, đúng hiện tượng bão hoà đã đo ở bài
`ham-kich-hoat`.
::::

::::predict{#doan_hang_so_khac_0 commitOnce}
Ví dụ đầu tiên khởi tạo TOÀN BỘ trọng số bằng `0`, và hai neuron ẩn giữ
nguyên giống hệt nhau mãi mãi.

**Trước khi đọc lại phần lý thuyết**, bạn đoán: nếu thay vì `0`, khởi tạo
CẢ HAI hàng của `W1` (trọng số vào hai neuron ẩn) bằng CÙNG một hằng số
khác — ví dụ `0.3` cho mọi phần tử — hai neuron ẩn có còn giống hệt nhau
mãi mãi không?

:::opt{correct}
Có — vấn đề không nằm ở việc hằng số đó là `0`, mà ở việc CẢ HAI neuron
nhận CÙNG một trọng số; bất kỳ hằng số nào giống nhau giữa chúng cũng gây
ra đúng hiện tượng đối xứng y hệt
:::

:::opt
Không — chỉ riêng `0` mới gây ra vấn đề, vì `0` là giá trị ĐẶC BIỆT (nhân
với `0` luôn triệt tiêu), còn một hằng số khác `0.3` thì không có tính chất
đặc biệt đó
::why
Gần đúng ở việc `0` đúng là có một tính chất TOÁN HỌC đặc biệt (nhân với
`0` triệt tiêu) mà `0.3` không có — quan sát đó không sai.

Chỗ lệch: hiện tượng đối xứng ở đây không tới từ tính chất ĐẶC BIỆT của số
`0` — nó tới từ việc hai neuron có TRỌNG SỐ GIỐNG HỆT NHAU, nên tính ra
cùng `z`, cùng `a`, và (quan trọng hơn) cùng gradient ở bước tiếp theo, bất
kể giá trị chung đó là gì. Khởi tạo `0.3` cho cả hai vẫn khiến chúng nhận
đúng cùng gradient mỗi bước, nên vẫn giữ nguyên giống hệt nhau mãi mãi —
đối xứng không cần `0` để xảy ra.
::
:::

:::opt
Không xác định được nếu không chạy thử — mỗi hằng số cho một hành vi khác
nhau, không có quy luật chung nào áp dụng được cho MỌI hằng số
::why
Gần đúng ở tinh thần cẩn trọng — đúng là giá trị CỤ THỂ của hằng số ảnh
hưởng tới TỐC ĐỘ hội tụ và có bão hoà hay không (một hằng số lớn vẫn gây
bão hoà, như ví dụ hai đã đo).

Chỗ lệch: dù tốc độ và mức bão hoà có thể khác nhau giữa các hằng số, CÂU
HỎI ở đây chỉ hỏi về ĐỐI XỨNG (hai neuron có giống hệt nhau mãi mãi không)
— và với câu hỏi cụ thể đó, có một quy luật chung áp dụng cho MỌI hằng số
giống nhau: hai neuron nhận cùng trọng số ban đầu thì nhận cùng gradient ở
mọi bước sau, không phụ thuộc giá trị hằng số đó là bao nhiêu.
::
:::
::::

::::code{#hoan_thien_kiem_tra_khoi_tao}
Hoàn thiện bốn hàm: `dong_bo` (kiểm tra hai cột của `H` có giống hệt nhau
không), `do_lon_dao_ham_trung_binh` (độ lớn trung bình của `sigmoid'` trên
một mảng `z`), và hai công thức khởi tạo `phuong_sai_xavier`/`phuong_sai_
he`.

```python title=starter
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def sigmoid_dao_ham(z):
    s = sigmoid(z)
    return s * (1 - s)

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0.,1.,1.,1.])

def forward(X, W1, b1, w2, b2):
    H = sigmoid(X @ W1 + b1)
    out = sigmoid(H @ w2 + b2)
    return H, out

def mat_mat(flat):
    W1 = flat[:4].reshape(2, 2); b1 = flat[4:6]; w2 = flat[6:8]; b2 = flat[8]
    _, p = forward(X, W1, b1, w2, b2)
    eps = 1e-12
    return -np.mean(y * np.log(p + eps) + (1 - y) * np.log(1 - p + eps))

def gop(W1, b1, w2, b2):
    return np.concatenate([W1.ravel(), b1.ravel(), w2.ravel(), np.array([b2])])

def tach(flat):
    return flat[:4].reshape(2, 2), flat[4:6], flat[6:8], flat[8]

def dao_ham_so(flat, eps=1e-4):
    g = np.zeros_like(flat)
    for i in range(len(flat)):
        cong = flat.copy(); cong[i] += eps
        tru = flat.copy(); tru[i] -= eps
        g[i] = (mat_mat(cong) - mat_mat(tru)) / (2 * eps)
    return g

def huan_luyen(flat_khoi_dau, lr, so_buoc):
    flat = flat_khoi_dau.copy()
    cac_H = []
    for _ in range(so_buoc):
        g = dao_ham_so(flat)
        flat = flat - lr * g
        W1, b1, w2, b2 = tach(flat)
        H, _ = forward(X, W1, b1, w2, b2)
        cac_H.append(H)
    return flat, cac_H

flat0 = gop(np.zeros((2, 2)), np.zeros(2), np.zeros(2), 0.0)
_, cac_H_zero = huan_luyen(flat0, 2.0, 5)

def dong_bo(H):
    return ___                     # np.allclose(H[:, 0], H[:, 1])

dong_bo_moi_buoc = [bool(dong_bo(H)) for H in cac_H_zero]

rng_nho = np.random.default_rng(1)
Z_nho = rng_nho.normal(0, 1.0, size=1000)
rng_lon = np.random.default_rng(1)
Z_lon = rng_lon.normal(0, 10.0, size=1000)

def do_lon_dao_ham_trung_binh(Z):
    return ___                     # np.mean(np.abs(sigmoid_dao_ham(Z)))

dh_nho = do_lon_dao_ham_trung_binh(Z_nho)
dh_lon = do_lon_dao_ham_trung_binh(Z_lon)

def phuong_sai_xavier(so_dau_vao, so_dau_ra):
    return ___                     # 2 / (so_dau_vao + so_dau_ra)

def phuong_sai_he(so_dau_vao):
    return ___                     # 2 / so_dau_vao

print(dong_bo_moi_buoc)
print(round(dh_nho, 4), round(dh_lon, 4))
print(round(phuong_sai_xavier(4, 2), 4))
print(round(phuong_sai_he(4), 4))
```

```python title=solution
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def sigmoid_dao_ham(z):
    s = sigmoid(z)
    return s * (1 - s)

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0.,1.,1.,1.])

def forward(X, W1, b1, w2, b2):
    H = sigmoid(X @ W1 + b1)
    out = sigmoid(H @ w2 + b2)
    return H, out

def mat_mat(flat):
    W1 = flat[:4].reshape(2, 2); b1 = flat[4:6]; w2 = flat[6:8]; b2 = flat[8]
    _, p = forward(X, W1, b1, w2, b2)
    eps = 1e-12
    return -np.mean(y * np.log(p + eps) + (1 - y) * np.log(1 - p + eps))

def gop(W1, b1, w2, b2):
    return np.concatenate([W1.ravel(), b1.ravel(), w2.ravel(), np.array([b2])])

def tach(flat):
    return flat[:4].reshape(2, 2), flat[4:6], flat[6:8], flat[8]

def dao_ham_so(flat, eps=1e-4):
    g = np.zeros_like(flat)
    for i in range(len(flat)):
        cong = flat.copy(); cong[i] += eps
        tru = flat.copy(); tru[i] -= eps
        g[i] = (mat_mat(cong) - mat_mat(tru)) / (2 * eps)
    return g

def huan_luyen(flat_khoi_dau, lr, so_buoc):
    flat = flat_khoi_dau.copy()
    cac_H = []
    for _ in range(so_buoc):
        g = dao_ham_so(flat)
        flat = flat - lr * g
        W1, b1, w2, b2 = tach(flat)
        H, _ = forward(X, W1, b1, w2, b2)
        cac_H.append(H)
    return flat, cac_H

flat0 = gop(np.zeros((2, 2)), np.zeros(2), np.zeros(2), 0.0)
_, cac_H_zero = huan_luyen(flat0, 2.0, 5)

def dong_bo(H):
    return np.allclose(H[:, 0], H[:, 1])

dong_bo_moi_buoc = [bool(dong_bo(H)) for H in cac_H_zero]

rng_nho = np.random.default_rng(1)
Z_nho = rng_nho.normal(0, 1.0, size=1000)
rng_lon = np.random.default_rng(1)
Z_lon = rng_lon.normal(0, 10.0, size=1000)

def do_lon_dao_ham_trung_binh(Z):
    return np.mean(np.abs(sigmoid_dao_ham(Z)))

dh_nho = do_lon_dao_ham_trung_binh(Z_nho)
dh_lon = do_lon_dao_ham_trung_binh(Z_lon)

def phuong_sai_xavier(so_dau_vao, so_dau_ra):
    return 2 / (so_dau_vao + so_dau_ra)

def phuong_sai_he(so_dau_vao):
    return 2 / so_dau_vao

print(dong_bo_moi_buoc)
print(round(dh_nho, 4), round(dh_lon, 4))
print(round(phuong_sai_xavier(4, 2), 4))
print(round(phuong_sai_he(4), 4))
```

```python title=test
assert dong_bo_moi_buoc == [True, True, True, True, True], f"dong_bo_moi_buoc sai -- dang ra {dong_bo_moi_buoc}"
assert round(dh_nho, 4) == 0.2078, f"dh_nho sai -- dang ra {round(dh_nho, 4)}"
assert round(dh_lon, 4) == 0.0415, f"dh_lon sai -- dang ra {round(dh_lon, 4)}"
assert dh_nho > dh_lon, "khoi tao NHO phai co do lon dao ham trung binh CAO HON khoi tao LON (bao hoa)"
assert round(phuong_sai_xavier(4, 2), 4) == 0.3333, f"xavier(4,2) sai -- dang ra {round(phuong_sai_xavier(4, 2), 4)}"
assert round(phuong_sai_he(4), 4) == 0.5, f"he(4) sai -- dang ra {round(phuong_sai_he(4), 4)}"
```

:::hints
- kind: attention
  body: Bốn chỗ trống. `dong_bo`: hai cột `H[:,0]` và `H[:,1]` có "gần bằng nhau" không — dùng `np.allclose`, không phải `==` (so sánh float). `do_lon_dao_ham_trung_binh`: trị tuyệt đối của `sigmoid_dao_ham(Z)`, rồi lấy trung bình — `np.mean(np.abs(...))`. Hai công thức khởi tạo là hai phân số đã nêu ở phần giải thích: Xavier chia cho TỔNG hai số (`n_vào + n_ra`), He chỉ chia cho `n_vào`.
- kind: strategy
  body: 'dong_bo: `np.allclose(H[:, 0], H[:, 1])`. do_lon_dao_ham_trung_binh: `np.mean(np.abs(sigmoid_dao_ham(Z)))`. phuong_sai_xavier: `2 / (so_dau_vao + so_dau_ra)`. phuong_sai_he: `2 / so_dau_vao`.'
- kind: one-line
  body: 'Bốn chỗ trống lần lượt là `np.allclose(H[:, 0], H[:, 1])`, `np.mean(np.abs(sigmoid_dao_ham(Z)))`, `2 / (so_dau_vao + so_dau_ra)`, và `2 / so_dau_vao`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: dong_bo phai goi THAT np.allclose; do_lon_dao_ham_trung_binh phai goi THAT np.mean va np.abs tren sigmoid_dao_ham(Z); ca hai cong thuc khoi tao phai tinh THAT tu so_dau_vao/so_dau_ra, khong duoc chep san ket qua
  requireAst:
  - kind: uses-call, target: allclose, min: 1
  - kind: uses-call, target: mean, min: 2
  - kind: uses-call, target: abs, min: 1
  - kind: uses-name, target: so_dau_vao, min: 2
  - kind: uses-name, target: so_dau_ra, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca nam luat qua sach (mean=2: mot lan trong mat_mat, mot
  # lan trong do_lon_dao_ham_trung_binh; so_dau_vao=2: mot lan trong xavier,
  # mot lan trong he). Cheat "dong_bo tra ve True chep san" lam "allclose" ve
  # 0 -- bi chan. Cheat "do_lon_dao_ham_trung_binh chep san 0.2" lam "mean"
  # tut ve 1 va "abs" ve 0 -- bi chan boi ca hai. Cheat "phuong_sai_xavier
  # chep san 0.3333" lam "so_dau_vao" tut ve 1 va "so_dau_ra" ve 0 -- bi
  # chan boi ca hai. Cheat "phuong_sai_he chep san 0.5" lam "so_dau_vao" tut
  # ve 1 -- bi chan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[True, True, True, True, True\\]\\n0\\.2078 0\\.0415\\n0\\.3333\\n0\\.5\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`dong_bo_moi_buoc = [True]*5` — hai neuron học GIỐNG HỆT NHAU suốt cả năm
bước. Khởi tạo ngẫu nhiên, nhỏ — Xavier hoặc He — phá vỡ đối xứng đó ngay
từ đầu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài này so sánh HAI mạng nhỏ — một tầng ẩn `2` neuron. Nếu tăng số neuron
trong tầng ẩn đó lên `8`, hay thêm HẲN một tầng ẩn nữa, mạng có khả năng
biểu diễn (capacity) những hàm số phức tạp tới đâu thay đổi thế nào — và
đánh đổi gì so với một mạng nhỏ hơn?
::::

::::checkpoint{mastery=0.8}
::::
