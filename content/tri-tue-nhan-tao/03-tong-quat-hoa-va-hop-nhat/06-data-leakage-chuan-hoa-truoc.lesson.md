---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.data-leakage-chuan-hoa-truoc
title: "Data leakage: chuẩn hoá trước khi tách"
summary: "8 điểm train + 3 điểm test nội suy (y=3+2x+0.4x²+nhiễu), ridge bậc 2 (lambda=5.0): chuẩn hoá ĐÚNG (trung_bình/độ_lệch_chuẩn chỉ từ train) cho test MSE=5.8796 — con số trung thực. Chuẩn hoá RÒ RỈ (tính từ train+test gộp lại TRƯỚC khi tách) cho test MSE=4.9015 — THẤP hơn, có vẻ 'đẹp' hơn, nhưng chỉ vì thống kê chuẩn hoá đã ngầm 'nhìn thấy' test trước khi mô hình chạm vào nó — một con số lạc quan giả tạo, không phải mô hình tốt hơn thật."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.data-leakage]
requires: [ai.hyperparameter-tuning]
concepts: [ai.data-leakage]
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
`boss-hoi-quy-va-gradient-descent` từng nhắc qua chuyện này ở một câu
predict, coi như một lưu ý phụ. Bài này mổ xẻ nó thành trọng tâm chính —
và đo bằng số THẬT con số bị lệch đi bao nhiêu.
::::

::::explain{#ro-ri-qua-chuan-hoa}
`chuan-hoa-dac-trung` (track hồi quy) dạy công thức chuẩn hoá:
`x_chuẩn = (x − trung_bình) / độ_lệch_chuẩn`. Câu hỏi ít ai để ý: hai con số
`trung_bình` và `độ_lệch_chuẩn` đó phải tính từ ĐÂU?

Kỷ luật ĐÚNG: tính CHỈ từ **train** — rồi áp DỤNG LẠI đúng hai con số đó
lên val và test. Test không hề tham gia vào việc tính ra `trung_bình`/
`độ_lệch_chuẩn` — nó chỉ được BIẾN ĐỔI bằng công thức đã chốt từ train.

Lỗi kinh điển: tính `trung_bình`/`độ_lệch_chuẩn` từ TOÀN BỘ dữ liệu (train
VÀ test GỘP LẠI) TRƯỚC khi tách ra hai tập — rồi mới tách, huấn luyện trên
phần train, và đánh giá trên phần test. Nghe qua có vẻ vô hại (test không
hề được dùng để khớp `w`, `b` — dùng gradient descent hay công thức đóng
đều chỉ chạy trên train), nhưng đây VẪN LÀ rò rỉ: hai con số
`trung_bình`/`độ_lệch_chuẩn` — thứ QUYẾT ĐỊNH mọi giá trị đưa vào mô hình,
kể cả giá trị train — đã được tính với một phần thông tin LẤY TỪ test.
Test không còn hoàn toàn "chưa từng chạm" nữa, dù nó chưa hề xuất hiện
trong công thức khớp `w, b`.

Với regularization (L2/ridge, đã học ở `regularization-l1-l2`), rò rỉ này
không chỉ là một chi tiết lý thuyết — nó THỰC SỰ đổi con số cuối cùng: mức
phạt `λ` tác động lên các hệ số ĐÃ CHUẨN HOÁ, nên hai cách tính
`trung_bình`/`độ_lệch_chuẩn` khác nhau khiến CÙNG một `λ` phạt theo hai mức
độ khác nhau trên thang đo GỐC — dẫn tới hai mô hình fit khác nhau thật,
không chỉ đổi tên biến. Bài này đo trực tiếp sự khác biệt đó.
::::

::::example{#do-chenh-lech-that-khi-ro-ri}
Tám điểm train, ba điểm test (nội suy — nằm TRONG khoảng train, không phải
ngoại suy), cùng quan hệ thật `y = 3 + 2x + 0.4x²` cộng nhiễu. Đặc trưng đa
thức bậc `2`, ridge với `λ = 5.0` cố định — so sánh chuẩn hoá ĐÚNG (chỉ từ
train) với chuẩn hoá RÒ RỈ (từ train+test gộp lại):

```python title=readonly
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def ridge_fit(X, y, lam):
    n, d = X.shape
    Xb = np.hstack([X, np.ones((n, 1))])
    I = np.eye(d + 1); I[-1, -1] = 0.0
    he_so = np.linalg.solve(Xb.T @ Xb + lam * I, Xb.T @ y)
    return he_so[:-1], he_so[-1]

BAC = 2
LAM = 5.0

x_train = np.arange(1, 9, dtype=float)
rng = np.random.default_rng(0)
y_train = true_fn(x_train) + rng.normal(0, 2.0, size=8)

x_test = np.array([2.5, 4.5, 6.5])
rng_t = np.random.default_rng(50)
y_test = true_fn(x_test) + rng_t.normal(0, 2.0, size=3)

X_tho_train = dac_trung(x_train, BAC)
X_tho_test = dac_trung(x_test, BAC)

# DUNG: trung_binh/do_lech_chuan CHI tu train
tb_dung, sd_dung = X_tho_train.mean(axis=0), X_tho_train.std(axis=0)
X_train_dung = (X_tho_train - tb_dung) / sd_dung
X_test_dung = (X_tho_test - tb_dung) / sd_dung
w_dung, b_dung = ridge_fit(X_train_dung, y_train, LAM)
mse_dung = np.mean((y_test - (X_test_dung @ w_dung + b_dung)) ** 2)

# RO RI: trung_binh/do_lech_chuan tu TOAN BO (train + test) gop lai, TRUOC khi tach
X_ca_hai = np.vstack([X_tho_train, X_tho_test])
tb_ro_ri, sd_ro_ri = X_ca_hai.mean(axis=0), X_ca_hai.std(axis=0)
X_train_ro_ri = (X_tho_train - tb_ro_ri) / sd_ro_ri
X_test_ro_ri = (X_tho_test - tb_ro_ri) / sd_ro_ri
w_ro_ri, b_ro_ri = ridge_fit(X_train_ro_ri, y_train, LAM)
mse_ro_ri = np.mean((y_test - (X_test_ro_ri @ w_ro_ri + b_ro_ri)) ** 2)

print("mse dung (chuan hoa CHI tu train):", round(mse_dung, 4))
print("mse ro_ri (chuan hoa tu ca train+test):", round(mse_ro_ri, 4))
```

```text title=readonly
mse dung (chuan hoa CHI tu train): 5.8796
mse ro_ri (chuan hoa tu ca train+test): 4.9015
```

`mse_ro_ri` (`4.9015`) THẤP hơn `mse_dung` (`5.8796`) — nhìn thoáng qua,
phiên bản "rò rỉ" trông như một mô hình TỐT HƠN. Nhưng nó không tốt hơn —
nó chỉ được ĐÁNH GIÁ theo một cách đã ngầm cho chuẩn hoá "nhìn thấy" chính
ba điểm test trước khi tách. Khi triển khai mô hình thật, dữ liệu MỚI hoàn
toàn không tồn tại tại thời điểm tính `trung_bình`/`độ_lệch_chuẩn` — nên
con số `4.9015` không phải điều mô hình sẽ đạt được trên dữ liệu thật sự
mới, chỉ là ảo giác từ việc chuẩn hoá đã "biết trước" đúng ba điểm sẽ dùng
để đánh giá nó. `5.8796` — con số CAO hơn, kém "đẹp" hơn — mới là con số
đáng tin.
::::

::::predict{#doan-neu-test-nam-xa-train commitOnce}
Vẫn bài toán trên. Giả sử `x_test` không còn nằm nội suy (trong khoảng
train `1..8`) mà đổi thành những điểm nằm HẲN ngoài khoảng đó (ví dụ
`x = 20, 22, 24` — ngoại suy xa).

**Trước khi đọc tiếp**, bạn đoán: việc RÒ RỈ khi chuẩn hoá (tính
`trung_bình`/`độ_lệch_chuẩn` từ train+test gộp) có còn XẢY RA không, dù
hướng chênh lệch cụ thể (cao hơn hay thấp hơn) có thể đổi khác?

:::opt{correct}
Có — rò rỉ xảy ra bất kể `test` nằm gần hay xa `train`, vì bản chất của nó
là việc TÍNH TOÁN thống kê chuẩn hoá có dùng dữ liệu test hay không, không
phụ thuộc vị trí cụ thể của test trên trục số
:::

:::opt
Không — nếu test nằm quá xa train, hai con số `trung_bình`/`độ_lệch_chuẩn`
gộp sẽ gần như không đổi so với chỉ tính từ train, vì test chỉ là vài điểm
lẻ
::why
Gần đúng ở trực giác "vài điểm lẻ có thể ảnh hưởng ít tới trung bình của
một tập LỚN hơn nhiều" — một cân nhắc hợp lý khi tập train rất lớn.

Chỗ lệch: ở đây tập train chỉ có `8` điểm — không đủ lớn để "vài điểm lẻ"
trở nên không đáng kể. Test nằm CÀNG XA train thực ra càng kéo
`trung_bình`/`độ_lệch_chuẩn` gộp lệch CÀNG MẠNH khỏi giá trị tính riêng từ
train (không phải càng ít ảnh hưởng) — cơ chế rò rỉ không biến mất, mà có
thể còn RÕ RỆT hơn.
::
:::

:::opt
Không — rò rỉ khi chuẩn hoá chỉ xảy ra khi test nằm NỘI SUY (trong khoảng
train), vì đó là trường hợp duy nhất bài này đã kiểm tra
::why
Gần đúng ở việc bạn để ý ĐÚNG: ví dụ trong bài dùng `x_test` nội suy — quan
sát về THIẾT KẾ của ví dụ cụ thể này không sai.

Chỗ lệch: đó là lựa chọn CỤ THỂ của ví dụ, không phải một điều kiện CẦN của
hiện tượng rò rỉ. Cơ chế rò rỉ nằm ở việc `trung_bình`/`độ_lệch_chuẩn` được
TÍNH TỪ ĐÂU — hễ còn tính từ dữ liệu gộp cả test, con số đó vẫn mang thông
tin của test, bất kể test nội suy hay ngoại suy. Vị trí của test chỉ quyết
định CHIỀU và ĐỘ LỚN của chênh lệch, không quyết định RÒ RỈ có xảy ra hay
không.
::
:::
::::

::::code{#do_ro_ri_chuan_hoa}
Hoàn thiện phần chuẩn hoá ĐÚNG (`tb_dung`, `sd_dung` chỉ từ `X_tho_train`)
và phần chuẩn hoá RÒ RỈ (`X_ca_hai` gộp cả train và test, rồi `tb_ro_ri`,
`sd_ro_ri` tính từ đó).

```python title=starter
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def ridge_fit(X, y, lam):
    n, d = X.shape
    Xb = np.hstack([X, np.ones((n, 1))])
    I = np.eye(d + 1); I[-1, -1] = 0.0
    he_so = np.linalg.solve(Xb.T @ Xb + lam * I, Xb.T @ y)
    return he_so[:-1], he_so[-1]

BAC = 2
LAM = 5.0

x_train = np.arange(1, 9, dtype=float)
rng = np.random.default_rng(0)
y_train = true_fn(x_train) + rng.normal(0, 2.0, size=8)

x_test = np.array([2.5, 4.5, 6.5])
rng_t = np.random.default_rng(50)
y_test = true_fn(x_test) + rng_t.normal(0, 2.0, size=3)

X_tho_train = dac_trung(x_train, BAC)
X_tho_test = dac_trung(x_test, BAC)

tb_dung, sd_dung = ___                    # X_tho_train.mean(axis=0), X_tho_train.std(axis=0) -- CHI tu train
X_train_dung = (X_tho_train - tb_dung) / sd_dung
X_test_dung = (X_tho_test - tb_dung) / sd_dung
w_dung, b_dung = ridge_fit(X_train_dung, y_train, LAM)
mse_dung = np.mean((y_test - (X_test_dung @ w_dung + b_dung)) ** 2)

X_ca_hai = ___                            # np.vstack([X_tho_train, X_tho_test]) -- GOP ca hai TRUOC khi chuan hoa
tb_ro_ri, sd_ro_ri = X_ca_hai.mean(axis=0), X_ca_hai.std(axis=0)
X_train_ro_ri = (X_tho_train - tb_ro_ri) / sd_ro_ri
X_test_ro_ri = (X_tho_test - tb_ro_ri) / sd_ro_ri
w_ro_ri, b_ro_ri = ridge_fit(X_train_ro_ri, y_train, LAM)
mse_ro_ri = np.mean((y_test - (X_test_ro_ri @ w_ro_ri + b_ro_ri)) ** 2)

print(round(mse_dung, 4))
print(round(mse_ro_ri, 4))
```

```python title=solution
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

def dac_trung(x, bac):
    return np.vstack([x**k for k in range(1, bac+1)]).T

def ridge_fit(X, y, lam):
    n, d = X.shape
    Xb = np.hstack([X, np.ones((n, 1))])
    I = np.eye(d + 1); I[-1, -1] = 0.0
    he_so = np.linalg.solve(Xb.T @ Xb + lam * I, Xb.T @ y)
    return he_so[:-1], he_so[-1]

BAC = 2
LAM = 5.0

x_train = np.arange(1, 9, dtype=float)
rng = np.random.default_rng(0)
y_train = true_fn(x_train) + rng.normal(0, 2.0, size=8)

x_test = np.array([2.5, 4.5, 6.5])
rng_t = np.random.default_rng(50)
y_test = true_fn(x_test) + rng_t.normal(0, 2.0, size=3)

X_tho_train = dac_trung(x_train, BAC)
X_tho_test = dac_trung(x_test, BAC)

tb_dung, sd_dung = X_tho_train.mean(axis=0), X_tho_train.std(axis=0)
X_train_dung = (X_tho_train - tb_dung) / sd_dung
X_test_dung = (X_tho_test - tb_dung) / sd_dung
w_dung, b_dung = ridge_fit(X_train_dung, y_train, LAM)
mse_dung = np.mean((y_test - (X_test_dung @ w_dung + b_dung)) ** 2)

X_ca_hai = np.vstack([X_tho_train, X_tho_test])
tb_ro_ri, sd_ro_ri = X_ca_hai.mean(axis=0), X_ca_hai.std(axis=0)
X_train_ro_ri = (X_tho_train - tb_ro_ri) / sd_ro_ri
X_test_ro_ri = (X_tho_test - tb_ro_ri) / sd_ro_ri
w_ro_ri, b_ro_ri = ridge_fit(X_train_ro_ri, y_train, LAM)
mse_ro_ri = np.mean((y_test - (X_test_ro_ri @ w_ro_ri + b_ro_ri)) ** 2)

print(round(mse_dung, 4))
print(round(mse_ro_ri, 4))
```

```python title=test
assert round(mse_dung, 4) == 5.8796, f"mse dung (chuan hoa CHI tu train) phai la 5.8796 -- dang ra {round(mse_dung, 4)}"
assert round(mse_ro_ri, 4) == 4.9015, f"mse ro_ri (chuan hoa tu ca train+test) phai la 4.9015 -- dang ra {round(mse_ro_ri, 4)}"
assert mse_dung != mse_ro_ri, "hai cach chuan hoa phai cho ra HAI MSE KHAC NHAU -- neu bang nhau thi tb_ro_ri/sd_ro_ri chua thuc su duoc tinh tu du lieu gop"
assert X_ca_hai.shape == (11, 2), f"X_ca_hai phai gop du 8 dong train + 3 dong test (11 dong, 2 cot) -- dang ra {X_ca_hai.shape}"
assert not np.allclose(tb_dung, tb_ro_ri), "tb_dung (chi tu train) phai KHAC tb_ro_ri (tu ca train+test) -- neu giong het thi X_ca_hai chua gop dung test vao"
```

:::hints
- kind: attention
  body: Hai chỗ trống. `tb_dung, sd_dung` chỉ tính từ `X_tho_train` — `X_tho_train.mean(axis=0), X_tho_train.std(axis=0)` — TUYỆT ĐỐI không đụng tới `X_tho_test`. `X_ca_hai` GỘP cả hai bằng `np.vstack([X_tho_train, X_tho_test])` — đây chính là bước tạo ra rò rỉ, vì `tb_ro_ri`/`sd_ro_ri` (đã có sẵn ở dòng dưới) sẽ tính từ mảng gộp này.
- kind: strategy
  body: 'tb_dung, sd_dung: `X_tho_train.mean(axis=0), X_tho_train.std(axis=0)`. X_ca_hai: `np.vstack([X_tho_train, X_tho_test])`.'
- kind: one-line
  body: 'Hai chỗ trống: `X_tho_train.mean(axis=0), X_tho_train.std(axis=0)` và `np.vstack([X_tho_train, X_tho_test])`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: tb_dung/sd_dung phai tinh CHI tu X_tho_train (khong duoc dung X_tho_test); X_ca_hai phai GOP X_tho_train VA X_tho_test bang np.vstack -- thieu no thi tb_ro_ri/sd_ro_ri se tinh sai (khong con la 'ro ri' nhu bai muon minh hoa)
  requireAst:
  - kind: uses-name, target: X_tho_train, min: 4
  - kind: uses-name, target: X_tho_test, min: 2
  - kind: uses-call, target: vstack, min: 1
  - kind: uses-name, target: X_ca_hai, min: 2
  # Da thu that (goi kiemAst that tren code day du): loi giai dung dat=true,
  # ca bon luat qua sach. Cheat "ro ri" nhung thuc te lai dung lai
  # tb_dung/sd_dung (tb_ro_ri, sd_ro_ri = tb_dung, sd_dung, khong tao
  # X_ca_hai) lam luat X_ca_hai tut ve 0 -- bi chan; chay THAT cheat nay cho
  # mse_ro_ri == mse_dung (5.8796 ca hai, thay vi 4.9015 ky vong) -- bi chan
  # doc lap boi tier tests qua assert mse_dung != mse_ro_ri.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^5\\.8796\\n4\\.9015\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`4.9015` trông đẹp hơn `5.8796` — nhưng "đẹp hơn" ở đây là cái giá của việc
để test rò vào chính bước chuẩn hoá. Con số xấu hơn, khi tính đúng cách,
mới là con số đáng tin.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Rò rỉ vừa thấy nằm ở bước CHUẨN HOÁ — một bước tưởng như "chỉ là tiền xử
lý", không phải một phần của việc "huấn luyện mô hình". Có một tình huống
khác, quen thuộc hơn nhiều, mà một lỗi tương tự — để test ảnh hưởng ngược
lại một QUYẾT ĐỊNH của pipeline — cũng có thể xảy ra: khi dữ liệu bị LỆCH
LỚP nặng, và cần chọn một NGƯỠNG hay một cách xử lý để mô hình không bỏ
qua lớp thiểu số. Việc đó có rò rỉ theo cách tương tự không?

Bài sau quay lại đúng bài toán lệch lớp đã gặp ở track trước — lần này, để
XỬ LÝ nó, không chỉ để phát hiện.
::::

::::checkpoint{mastery=0.8}
::::
