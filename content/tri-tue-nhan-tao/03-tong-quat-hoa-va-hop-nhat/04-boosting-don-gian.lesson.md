---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.boosting-don-gian
title: "Boosting đơn giản: sửa lỗi tuần tự"
summary: "10 điểm (y=3+2x+0.4x²+nhiễu): dự đoán hằng số 0 ban đầu cho MSE=1239.967. Năm vòng gradient boosting cực đơn giản (mỗi vòng: một 'regression stump' — một ngưỡng chia đôi trục x — khớp vào PHẦN DƯ của vòng trước, rồi cộng thêm lr=0.5 lần dự đoán đó vào tổng): MSE giảm dần 373.6711 → 133.1749 → 49.1171 → 26.0287 → 16.5137 — mỗi vòng chỉ sửa đúng phần lỗi mà các vòng trước còn để lại, không học lại từ đầu."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.boosting]
requires: [ai.bagging]
concepts: [ai.boosting]
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
Bagging huấn luyện nhiều mô hình ĐỘC LẬP, không cái nào biết cái nào. Lần
này, mỗi mô hình MỚI sẽ nhìn thẳng vào đúng chỗ những mô hình TRƯỚC nó còn
sai.
::::

::::explain{#sua-loi-tuan-tu}
Bagging (bài trước) huấn luyện `B` mô hình song song, độc lập, rồi trung
bình hoá — mục tiêu là GIẢM VARIANCE. **Boosting** theo một triết lý khác
hẳn: huấn luyện mô hình TUẦN TỰ, mỗi mô hình MỚI cố sửa đúng phần lỗi mà
TỔNG các mô hình trước nó còn để lại — mục tiêu là GIẢM BIAS, bằng cách
cộng dồn nhiều mô hình YẾU (mỗi cái chỉ giỏi một phần nhỏ của bài toán)
thành một mô hình TỔNG mạnh hơn hẳn từng phần riêng lẻ.

Phiên bản đơn giản nhất — **gradient boosting** cho hồi quy trên MỘT đặc
trưng — hoạt động qua các bước:

1. Bắt đầu với một dự đoán CỰC ĐƠN GIẢN cho mọi điểm (ở đây: hằng số `0`).
2. Tính **phần dư** (residual) — sai số còn lại: `y thật − dự đoán hiện tại`.
3. Huấn luyện một **mô hình yếu** (ở đây: một `stump` — một NGƯỠNG chia
   trục `x` làm hai nửa, mỗi nửa dự đoán bằng TRUNG BÌNH phần dư của chính
   nửa đó, chọn ngưỡng sao cho tổng bình phương sai số — SSE — nhỏ nhất) để
   khớp CHÍNH phần dư đó, KHÔNG khớp `y` gốc.
4. Cộng dự đoán của mô hình yếu đó (nhân với một **learning rate** `lr` nhỏ,
   để mỗi vòng chỉ tiến một bước KHIÊM TỐN) vào dự đoán TỔNG hiện tại.
5. Lặp lại bước `2`–`4` nhiều vòng — mỗi vòng, phần dư MỚI (sau khi đã cộng
   dự đoán của vòng trước) thường NHỎ hơn phần dư vòng trước, vì phần lớn
   lỗi cũ đã được một mô hình yếu nào đó sửa.

Điểm khác biệt cốt lõi với bagging: mô hình ở vòng thứ `k` không hề nhìn
`y` gốc — nó chỉ nhìn PHẦN CÒN SAI sau `k − 1` vòng trước, nên nó tự động
"chú ý nhiều hơn" vào đúng những điểm mà các vòng trước dự đoán tệ nhất
(phần dư ở đó LỚN hơn).
::::

::::example{#boosting-giam-mse-tung-vong}
Mười điểm (`x = 1..10`, quan hệ thật `y = 3 + 2x + 0.4x²` cộng nhiễu),
`stump_fit` tìm ngưỡng chia SSE nhỏ nhất trên PHẦN DƯ hiện tại, `lr = 0.5`,
chạy `5` vòng:

```python title=readonly
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

x = np.arange(1, 11, dtype=float)
rng = np.random.default_rng(1)
y = true_fn(x) + rng.normal(0, 2.0, size=len(x))

def stump_fit(x, r):
    nguong_list = (x[:-1] + x[1:]) / 2
    best_sse = np.inf
    best = None
    for t in nguong_list:
        trai = r[x < t]
        phai = r[x >= t]
        m_trai = trai.mean() if len(trai) > 0 else 0.0
        m_phai = phai.mean() if len(phai) > 0 else 0.0
        sse = np.sum((trai - m_trai)**2) + np.sum((phai - m_phai)**2)
        if sse < best_sse:
            best_sse = sse
            best = (t, m_trai, m_phai)
    return best

def stump_predict(x, stump):
    t, m_trai, m_phai = stump
    return np.where(x < t, m_trai, m_phai)

LR = 0.5
SO_VONG = 5
pred = np.zeros(len(x))
print("mse ban dau (chua boost gi):", round(np.mean((y - pred) ** 2), 4))

for vong in range(SO_VONG):
    residual = y - pred
    stump = stump_fit(x, residual)
    pred = pred + LR * stump_predict(x, stump)
    mse = np.mean((y - pred) ** 2)
    print(f"vong {vong+1}: nguong={round(stump[0],2)} mse={round(float(mse),4)}")
```

```text title=readonly
mse ban dau (chua boost gi): 1239.967
vong 1: nguong=6.5 mse=373.6711
vong 2: nguong=4.5 mse=133.1749
vong 3: nguong=8.5 mse=49.1171
vong 4: nguong=7.5 mse=26.0287
vong 5: nguong=9.5 mse=16.5137
```

Trước khi boost, dự đoán hằng số `0` cho MSE `1239.967` — rất tệ. Sau MỖI
vòng, MSE giảm liên tục: `373.6711 → 133.1749 → 49.1171 → 26.0287 →
16.5137` — không vòng nào làm nó tăng lên. Mỗi vòng chỉ thêm ĐÚNG một
ngưỡng chia đôi trục `x` (một mô hình cực yếu, chỉ một lần rẽ nhánh), nhưng
`5` mô hình yếu CỘNG DỒN lại — mỗi cái sửa đúng phần lỗi còn sót của các
vòng trước — giảm MSE xuống chưa tới `1.5%` so với lúc bắt đầu.
::::

::::predict{#doan-neu-lr-qua-lon commitOnce}
Vẫn thí nghiệm trên. Giả sử đổi `lr` từ `0.5` lên `2.0` (một bước "cộng
thêm" LỚN hơn mỗi vòng, thay vì khiêm tốn).

**Trước khi chạy**, bạn đoán: điều gì nhiều khả năng xảy ra với MSE qua các
vòng?

:::opt{correct}
MSE có thể GIẢM nhanh hơn ở vài vòng đầu, nhưng cũng dễ VỌT QUA đích rồi
dao động hoặc tăng trở lại ở vòng sau — cùng bản chất với learning rate quá
lớn của gradient descent (`gradient-descent-tu-so-0`): bước cộng thêm quá
dài dễ đi xa hơn cả điểm cần dừng
:::

:::opt
MSE sẽ giảm nhanh hơn ở MỌI vòng, không có rủi ro gì — `lr` càng lớn thì
mỗi vòng sửa lỗi càng nhiều
::why
Gần đúng ở việc `lr` lớn hơn đúng là làm MỖI vòng CỘNG THÊM một lượng lớn
hơn vào dự đoán tổng — quan sát đó không sai về mặt số học đơn thuần.

Chỗ lệch: "cộng thêm nhiều hơn mỗi vòng" không đồng nghĩa "luôn tiến gần
đích hơn". Mỗi mô hình yếu chỉ ước lượng THÔ phần dư (một ngưỡng chia đôi,
không khớp chính xác tuyệt đối) — nhân ước lượng thô đó với một hệ số quá
lớn có thể làm dự đoán tổng VƯỢT quá giá trị đúng, y hệt cách một bước
gradient descent quá dài có thể vọt qua điểm tối ưu rồi dao động hoặc phân
kỳ, thay vì hội tụ êm.
::
:::

:::opt
Không có gì thay đổi — `lr` chỉ ảnh hưởng tới TỐC ĐỘ hiển thị kết quả,
không ảnh hưởng tới MSE cuối cùng
::why
Gần đúng ở việc bạn phân biệt đúng khái niệm "tốc độ" khỏi "kết quả cuối" —
một sự phân biệt hợp lý nói chung.

Chỗ lệch: ở đây `lr` không phải một tham số về TỐC ĐỘ HIỂN THỊ — nó nhân
trực tiếp vào GIÁ TRỊ được cộng thêm mỗi vòng (`pred = pred + lr *
stump_predict(...)`), nên nó thực sự thay đổi CHÍNH dự đoán tổng, không chỉ
thay đổi cách trình bày. Đổi `lr` đổi thẳng số học của quá trình, không chỉ
đổi cách nó được in ra.
::
:::
::::

::::code{#boosting_giam_mse_dan}
Hoàn thiện vòng lặp boosting: tính phần dư (`y` thật trừ dự đoán TỔNG hiện
tại), rồi cộng thêm `LR` lần dự đoán của stump vào dự đoán tổng.

```python title=starter
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

x = np.arange(1, 11, dtype=float)
rng = np.random.default_rng(1)
y = true_fn(x) + rng.normal(0, 2.0, size=len(x))

def stump_fit(x, r):
    nguong_list = (x[:-1] + x[1:]) / 2
    best_sse = np.inf
    best = None
    for t in nguong_list:
        trai = r[x < t]
        phai = r[x >= t]
        m_trai = trai.mean() if len(trai) > 0 else 0.0
        m_phai = phai.mean() if len(phai) > 0 else 0.0
        sse = np.sum((trai - m_trai)**2) + np.sum((phai - m_phai)**2)
        if sse < best_sse:
            best_sse = sse
            best = (t, m_trai, m_phai)
    return best

def stump_predict(x, stump):
    t, m_trai, m_phai = stump
    return np.where(x < t, m_trai, m_phai)

LR = 0.5
SO_VONG = 5
pred = np.zeros(len(x))
mse_ban_dau = np.mean((y - pred) ** 2)

mse_theo_vong = []
for vong in range(SO_VONG):
    residual = ___                       # y tru pred hien tai
    stump = stump_fit(x, residual)
    pred = ___                           # pred cong LR nhan stump_predict(x, stump)
    mse_theo_vong.append(np.mean((y - pred) ** 2))

print(round(mse_ban_dau, 4))
for m in mse_theo_vong:
    print(round(float(m), 4))
```

```python title=solution
import numpy as np

def true_fn(x):
    return 3 + 2*x + 0.4*x**2

x = np.arange(1, 11, dtype=float)
rng = np.random.default_rng(1)
y = true_fn(x) + rng.normal(0, 2.0, size=len(x))

def stump_fit(x, r):
    nguong_list = (x[:-1] + x[1:]) / 2
    best_sse = np.inf
    best = None
    for t in nguong_list:
        trai = r[x < t]
        phai = r[x >= t]
        m_trai = trai.mean() if len(trai) > 0 else 0.0
        m_phai = phai.mean() if len(phai) > 0 else 0.0
        sse = np.sum((trai - m_trai)**2) + np.sum((phai - m_phai)**2)
        if sse < best_sse:
            best_sse = sse
            best = (t, m_trai, m_phai)
    return best

def stump_predict(x, stump):
    t, m_trai, m_phai = stump
    return np.where(x < t, m_trai, m_phai)

LR = 0.5
SO_VONG = 5
pred = np.zeros(len(x))
mse_ban_dau = np.mean((y - pred) ** 2)

mse_theo_vong = []
for vong in range(SO_VONG):
    residual = y - pred
    stump = stump_fit(x, residual)
    pred = pred + LR * stump_predict(x, stump)
    mse_theo_vong.append(np.mean((y - pred) ** 2))

print(round(mse_ban_dau, 4))
for m in mse_theo_vong:
    print(round(float(m), 4))
```

```python title=test
assert round(mse_ban_dau, 4) == 1239.967, f"mse ban dau phai la 1239.967 -- dang ra {round(mse_ban_dau, 4)}"
assert [round(float(m), 4) for m in mse_theo_vong] == [373.6711, 133.1749, 49.1171, 26.0287, 16.5137], f"mse theo vong sai -- dang ra {[round(float(m), 4) for m in mse_theo_vong]}"
assert all(mse_theo_vong[i] > mse_theo_vong[i+1] for i in range(len(mse_theo_vong)-1)), "mse phai GIAM DAN qua tung vong, khong duoc tang o vong nao"
assert mse_theo_vong[-1] < mse_ban_dau / 50, "sau 5 vong, mse phai thap hon han (duoi 1/50) mse ban dau"
```

:::hints
- kind: attention
  body: Hai chỗ trống. `residual` là phần LỖI CÒN LẠI — `y` thật trừ đi dự đoán TỔNG hiện tại (`pred`, đã cộng dồn từ các vòng trước, KHÔNG phải `y` trừ một stump riêng lẻ). `pred` được CẬP NHẬT bằng cách cộng thêm `LR` nhân dự đoán của `stump` vừa khớp — không thay THẾ `pred` cũ, mà CỘNG DỒN vào nó.
- kind: strategy
  body: 'residual: `y - pred`. pred: `pred + LR * stump_predict(x, stump)`.'
- kind: one-line
  body: 'Hai chỗ trống: `y - pred` và `pred + LR * stump_predict(x, stump)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: residual phai la y TRU pred hien tai (khong duoc doi thu tu thanh pred tru y, se dao nguoc huong sua loi); pred phai CONG DON LR nhan stump_predict(x, stump) vao pred cu, khong duoc thay the hay tru di
  requireAst:
  - kind: uses-name, target: residual, min: 1
  - kind: uses-name, target: pred, min: 4
  - kind: uses-call, target: stump_predict, min: 1
  - kind: uses-call, target: stump_fit, min: 1
  # Da thu that (goi kiemAst that tren code day du): loi giai dung dat=true,
  # ca bon luat qua sach. Doi thu tu residual = pred - y (dao nguoc dau) VAN
  # giu nguyen so lan xuat hien cua ten "residual"/"pred" (khong bi static
  # bat rieng phep doi dau nay) nhung bi tier tests chan rat manh: chay THAT
  # xac nhan mse tang vot qua tung vong (2683.79 -> 66691.55, thay vi giam
  # dan xuong 16.5137) -- dung phan cong hai tang, khong can them boundary
  # rieng vi day khong phai loai loi "bien khong cham toi" (gia tri sai het
  # ca 5 vong, sai rat lon, khong phai mot diem hoa hiem gap).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^1239\\.967\\n373\\.6711\\n133\\.1749\\n49\\.1171\\n26\\.0287\\n16\\.5137\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Từ `1239.967` xuống `16.5137` — không phải một mô hình mạnh duy nhất, mà
năm ngưỡng chia đôi cực đơn giản, mỗi cái chỉ sửa đúng phần lỗi còn sót của
những cái trước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bagging và boosting đều kết hợp NHIỀU mô hình — nhưng bagging huấn luyện
chúng SONG SONG, độc lập, để giảm variance; boosting huấn luyện chúng TUẦN
TỰ, mỗi cái sửa lỗi của tổng trước nó, để giảm bias. Cả hai đều là những
lựa chọn CÓ SẴN — nhưng cả hai đều cần một cách khác để chọn: chọn SỐ VÒNG
boosting, chọn `B` của bagging, chọn `lr`, ... — làm sao chọn những con số
đó một cách CÓ HỆ THỐNG, thay vì dò từng giá trị bằng tay?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
