---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.bagging
title: "Bagging: trung bình nhiều mô hình bootstrap"
summary: "Dự đoán 1-NN (hàng xóm gần nhất) tại một điểm cố định, qua 40 lần lặp lại với nhiễu khác nhau: một mô hình ĐƠN LẺ (huấn luyện trực tiếp trên dữ liệu train) có độ lệch chuẩn của dự đoán là 5.253. Bagging — với MỖI trong 40 lần đó, huấn luyện 30 mô hình 1-NN trên 30 mẫu bootstrap khác nhau của CÙNG dữ liệu rồi lấy TRUNG BÌNH — kéo độ lệch chuẩn xuống còn 3.5643, thấp hơn khoảng 1.47 lần: kết hợp nhiều mô hình bootstrap làm giảm variance thật sự, đo được bằng số."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.bagging]
requires: [ai.bootstrap-resampling]
concepts: [ai.bagging]
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
Bootstrap tạo ra nhiều PHIÊN BẢN hơi khác của cùng dữ liệu. Bài trước dùng
chúng để ĐO độ không chắc chắn. Lần này, dùng chúng để HUẤN LUYỆN — và kết
hợp kết quả lại.
::::

::::explain{#bagging-la-gi}
`bias-variance-tradeoff` để lại một câu hỏi: một mô hình variance cao (dự
đoán đổi mạnh theo từng tập train cụ thể) có cách nào bớt nhạy đi không, mà
KHÔNG cần đổi bản thân mô hình? **Bagging** (bootstrap aggregating) trả lời
bằng đúng công cụ vừa học ở bài trước:

1. Từ MỘT tập dữ liệu train, tạo ra `B` mẫu **bootstrap** (lấy có hoàn lại,
   mỗi mẫu cùng kích thước `n` với dữ liệu gốc).
2. Huấn luyện `B` mô hình — CÙNG một loại mô hình, CÙNG một cách huấn luyện
   — mỗi mô hình trên MỘT mẫu bootstrap RIÊNG.
3. Dự đoán cuối cùng cho một điểm mới = **TRUNG BÌNH** của `B` dự đoán đó.

Trực giác: mỗi mô hình bootstrap học trên một phiên bản dữ liệu hơi khác
(một vài điểm bị lặp lại, một vài điểm bị bỏ sót), nên MỖI mô hình riêng lẻ
vẫn có thể sai lệch theo hướng riêng của nó — nhưng những hướng sai lệch đó
có xu hướng KHÁC NHAU giữa các mô hình, không cùng một hướng. Lấy trung
bình của nhiều dự đoán có sai lệch khác hướng làm những sai lệch đó triệt
tiêu bớt lẫn nhau — kết quả trung bình ổn định hơn BẤT KỲ một mô hình đơn
lẻ nào trong số chúng.

Bagging phát huy tác dụng rõ nhất với những mô hình VỐN có variance cao —
những mô hình nhạy với từng điểm dữ liệu cụ thể. **k-NN với `k=1`**
(`k-nn-phan-loai`) là một ví dụ điển hình: dự đoán chỉ dựa vào ĐÚNG MỘT
hàng xóm gần nhất, nên đổi một điểm trong tập train (hay đổi việc điểm đó
có mặt hay không, đúng như bootstrap làm) có thể đổi hẳn hàng xóm gần nhất
được chọn.
::::

::::example{#gian-so-bagging-giam-variance}
Một bài toán hồi quy một chiều (`x_train = 1..10`, quan hệ thật
`y = 3 + 2x + 0.4x²` cộng nhiễu khá lớn), dự đoán bằng **1-NN**: giá trị
`y` của điểm train GẦN NHẤT. Đo tại một điểm cố định `x0 = 5.5`, lặp lại
toàn bộ thí nghiệm `40` lần (mỗi lần một tập train nhiễu khác) để đo
variance của: (a) MỘT mô hình 1-NN đơn lẻ, và (b) bagging của `30` mô hình
1-NN, mỗi cái trên một mẫu bootstrap RIÊNG của CHÍNH tập train đó:

```python title=readonly
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

def nn1_predict(x_tr, y_tr, x0):
    d = np.abs(x_tr - x0)
    idx = np.argmin(d)
    return y_tr[idx]

x_train = np.arange(1, 11, dtype=float)
n = len(x_train)
x0 = 5.5
NOISE_SD = 5.0
R = 40   # so lan lap lai (tap train khac nhau)
B = 30   # so mo hinh bootstrap trong MOI lan bagging

du_doan_don = np.zeros(R)
du_doan_bagging = np.zeros(R)
for r in range(R):
    rng_r = np.random.default_rng(r)
    y_tr = true_fn(x_train) + rng_r.normal(0, NOISE_SD, size=n)

    # (a) MOT mo hinh don, huan luyen truc tiep tren (x_train, y_tr)
    du_doan_don[r] = nn1_predict(x_train, y_tr, x0)

    # (b) bagging: B mo hinh, moi cai tren MOT mau bootstrap cua (x_train, y_tr)
    rng_boot = np.random.default_rng(1000 + r)
    preds = np.zeros(B)
    for i in range(B):
        idx = rng_boot.integers(0, n, size=n)
        xb, yb = x_train[idx], y_tr[idx]
        preds[i] = nn1_predict(xb, yb, x0)
    du_doan_bagging[r] = preds.mean()

print("std du doan DON LE:", round(du_doan_don.std(), 4))
print("std du doan BAGGING:", round(du_doan_bagging.std(), 4))
print("trung binh don le:", round(du_doan_don.mean(), 4))
print("trung binh bagging:", round(du_doan_bagging.mean(), 4))
print("gia tri that tai x0:", round(true_fn(x0), 4))
```

```text title=readonly
std du doan DON LE: 5.253
std du doan BAGGING: 3.5643
trung binh don le: 24.4943
trung binh bagging: 26.6109
gia tri that tai x0: 26.1
```

Độ lệch chuẩn của mô hình ĐƠN LẺ là `5.253` — qua `40` lần lặp (mỗi lần một
tập train nhiễu khác), dự đoán tại `x0` dao động khá mạnh, vì 1-NN chỉ dựa
vào đúng MỘT điểm gần nhất, và điểm đó (hay giá trị nhiễu của nó) đổi theo
từng lần. Độ lệch chuẩn của BAGGING chỉ còn `3.5643` — THẤP hơn khoảng
`1.47` lần. Đồng thời, trung bình của bagging (`26.6109`) cũng gần giá trị
THẬT tại `x0` (`26.1`) hơn trung bình của mô hình đơn lẻ (`24.4943`) — bagging
không chỉ ổn định hơn, ở đây nó còn CHÍNH XÁC hơn trên trung bình.
::::

::::predict{#doan-neu-tang-b commitOnce}
Vẫn thí nghiệm trên. Giả sử tăng `B` (số mô hình bootstrap trong MỖI lần
bagging) từ `30` lên `300` — gấp `10` lần, nhưng giữ nguyên `R = 40` (số
lần lặp lại toàn bộ thí nghiệm) và giữ nguyên dữ liệu gốc.

**Trước khi chạy**, bạn đoán: độ lệch chuẩn của `du_doan_bagging` (qua `40`
lần lặp) sẽ thay đổi ra sao?

:::opt{correct}
Giảm thêm một chút nữa rồi CHỮNG LẠI — trung bình của `300` mô hình bootstrap
ổn định hơn trung bình của `30` mô hình (ít nhiễu ngẫu nhiên hơn trong chính
bước lấy trung bình), nhưng không thể giảm variance xuống dưới mức mà CHÍNH
dữ liệu train gốc (không đổi giữa `30` và `300`) cho phép
:::

:::opt
Giảm về đúng `0` — tăng đủ số mô hình bootstrap sẽ triệt tiêu HOÀN TOÀN mọi
dao động
::why
Gần đúng ở HƯỚNG: tăng `B` đúng là làm bước lấy trung bình bên trong MỖI lần
bagging ổn định hơn — quan sát đó không sai.

Chỗ lệch: `R = 40` (số lần lặp lại thí nghiệm) không đổi, và dữ liệu train
GỐC ở mỗi lần lặp vẫn là `10` điểm CỤ THỂ, nhiễu CỤ THỂ của lần đó — bagging
chỉ trung bình hoá các mô hình SINH TỪ CÙNG một tập train đó, nó không tạo
ra dữ liệu train mới. Variance CÒN LẠI qua `40` lần lặp phản ánh chính sự
khác biệt GIỮA các tập train khác nhau (nhiễu khác nhau ở từng lần `r`) —
một nguồn dao động mà tăng `B` không chạm tới được, nên không thể ép về
đúng `0`.
::
:::

:::opt
Tăng lên, vì có nhiều mô hình bootstrap hơn nghĩa là nhiều nguồn nhiễu ngẫu
nhiên hơn được đưa vào
::why
Gần đúng ở việc bạn để ý ĐÚNG: có `300` phép lấy mẫu bootstrap ngẫu nhiên
thay vì `30`, tức là NHIỀU bước ngẫu nhiên hơn về mặt số lượng thao tác.

Chỗ lệch: nhiều bước ngẫu nhiên hơn không có nghĩa nhiều dao động hơn ở kết
quả CUỐI — vì kết quả cuối là TRUNG BÌNH của tất cả chúng. Trung bình của
NHIỀU biến ngẫu nhiên (ở đây là nhiều dự đoán bootstrap) luôn ổn định HƠN
(hoặc bằng) trung bình của ÍT biến ngẫu nhiên hơn — đây chính là bản chất
của việc lấy trung bình, không phải ngược lại.
::
:::
::::

::::code{#do_giam_variance_bang_bagging}
Hoàn thiện vòng lặp bagging: lấy chỉ số bootstrap CÓ HOÀN LẠI từ CHÍNH tập
train hiện tại, tạo mẫu bootstrap, rồi lấy TRUNG BÌNH của `B` dự đoán 1-NN.

```python title=starter
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

def nn1_predict(x_tr, y_tr, x0):
    d = np.abs(x_tr - x0)
    idx = np.argmin(d)
    return y_tr[idx]

x_train = np.arange(1, 11, dtype=float)
n = len(x_train)
x0 = 5.5
NOISE_SD = 5.0
R = 40
B = 30

du_doan_don = np.zeros(R)
du_doan_bagging = np.zeros(R)
for r in range(R):
    rng_r = np.random.default_rng(r)
    y_tr = true_fn(x_train) + rng_r.normal(0, NOISE_SD, size=n)

    du_doan_don[r] = nn1_predict(x_train, y_tr, x0)

    rng_boot = np.random.default_rng(1000 + r)
    preds = np.zeros(B)
    for i in range(B):
        idx = ___                          # rng_boot.integers(0, n, size=n) -- CO HOAN LAI
        xb, yb = ___, ___                  # x_train[idx], y_tr[idx]
        preds[i] = nn1_predict(xb, yb, x0)
    du_doan_bagging[r] = ___               # preds.mean()

print(round(du_doan_don.std(), 4))
print(round(du_doan_bagging.std(), 4))
print(round(du_doan_don.mean(), 4))
print(round(du_doan_bagging.mean(), 4))
```

```python title=solution
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

def nn1_predict(x_tr, y_tr, x0):
    d = np.abs(x_tr - x0)
    idx = np.argmin(d)
    return y_tr[idx]

x_train = np.arange(1, 11, dtype=float)
n = len(x_train)
x0 = 5.5
NOISE_SD = 5.0
R = 40
B = 30

du_doan_don = np.zeros(R)
du_doan_bagging = np.zeros(R)
for r in range(R):
    rng_r = np.random.default_rng(r)
    y_tr = true_fn(x_train) + rng_r.normal(0, NOISE_SD, size=n)

    du_doan_don[r] = nn1_predict(x_train, y_tr, x0)

    rng_boot = np.random.default_rng(1000 + r)
    preds = np.zeros(B)
    for i in range(B):
        idx = rng_boot.integers(0, n, size=n)
        xb, yb = x_train[idx], y_tr[idx]
        preds[i] = nn1_predict(xb, yb, x0)
    du_doan_bagging[r] = preds.mean()

print(round(du_doan_don.std(), 4))
print(round(du_doan_bagging.std(), 4))
print(round(du_doan_don.mean(), 4))
print(round(du_doan_bagging.mean(), 4))
```

```python title=test
assert round(du_doan_don.std(), 4) == 5.253, f"std du doan don le phai la 5.253 -- dang ra {round(du_doan_don.std(), 4)}"
assert round(du_doan_bagging.std(), 4) == 3.5643, f"std du doan bagging phai la 3.5643 -- dang ra {round(du_doan_bagging.std(), 4)}"
assert round(du_doan_don.mean(), 4) == 24.4943, f"trung binh don le phai la 24.4943 -- dang ra {round(du_doan_don.mean(), 4)}"
assert round(du_doan_bagging.mean(), 4) == 26.6109, f"trung binh bagging phai la 26.6109 -- dang ra {round(du_doan_bagging.mean(), 4)}"
assert du_doan_bagging.std() < du_doan_don.std(), "std cua bagging phai THAP HON han std cua mo hinh don le"
```

:::hints
- kind: attention
  body: Ba chỗ trống, đúng bốn bước của MỘT lần lấy mẫu bootstrap (giống hệt bài `bootstrap-resampling`, chỉ đổi dữ liệu nguồn từ `do_tre` sang `(x_train, y_tr)`). `idx` là `n` chỉ số ngẫu nhiên CÓ HOÀN LẠI — `rng_boot.integers(0, n, size=n)`. `xb, yb` là `x_train`/`y_tr` LẤY THEO đúng những chỉ số đó. Dòng cuối lấy TRUNG BÌNH của mảng `preds` (không phải một phần tử đơn lẻ của nó).
- kind: strategy
  body: 'idx: `rng_boot.integers(0, n, size=n)`. xb, yb: `x_train[idx], y_tr[idx]`. du_doan_bagging[r]: `preds.mean()`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `rng_boot.integers(0, n, size=n)`, `x_train[idx], y_tr[idx]`, và `preds.mean()`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: idx phai goi THAT rng_boot.integers(0, n, size=n) (lay mau CO HOAN LAI, khong duoc dung mot day chi so co dinh nhu np.arange); xb/yb phai lay tu x_train/y_tr bang idx; du_doan_bagging[r] phai la preds.mean() (trung binh CA mang), khong phai mot phan tu don le cua preds
  requireAst:
  - kind: uses-call, target: integers, min: 1
  - kind: uses-name, target: xb, min: 1
  - kind: uses-name, target: yb, min: 1
  - kind: uses-call, target: nn1_predict, min: 2
  - kind: uses-name, target: preds, min: 2
  # Da thu that (goi kiemAst that tren code day du): loi giai dung dat=true,
  # ca nam luat qua sach. Cheat idx=np.arange(n) (mat lay mau ngau nhien) lam
  # luat "integers" tut ve 0 -- bi chan; cheat rieng
  # du_doan_bagging[r]=preds[0] (khong lay trung binh) khong bi static chan
  # (van du ten preds) nhung bi tier tests chan qua gia tri sai (std_bagging
  # trung het voi std_don, khac han 3.5643 ky vong) -- da chay THAT xac nhan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^5\\.253\\n3\\.5643\\n24\\.4943\\n26\\.6109\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`3.5643` so với `5.253` — không đổi mô hình, không đổi dữ liệu gốc, chỉ
huấn luyện nhiều bản sao trên nhiều mẫu bootstrap rồi lấy trung bình.
Variance giảm thật, đo được thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bagging huấn luyện `B` mô hình HOÀN TOÀN ĐỘC LẬP — mỗi mô hình không hề
biết các mô hình khác đang làm gì, không mô hình nào "sửa lỗi" cho mô hình
khác. Có cách nào để các mô hình SAU chú ý nhiều hơn vào đúng những điểm mà
các mô hình TRƯỚC đã dự đoán sai — thay vì mỗi mô hình học độc lập trên một
mẫu ngẫu nhiên?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
