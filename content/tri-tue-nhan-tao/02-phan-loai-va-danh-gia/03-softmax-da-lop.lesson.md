---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.softmax-da-lop
title: "Softmax đa lớp"
summary: "Chín email tự bịa, ba lớp (0=bình thường, 1=quảng cáo, 2=spam) theo số từ viết hoa [0,1,2,4,5,6,8,9,10]: softmax huấn luyện qua gradient descent (500 bước, lr=0.5) đạt loss=0.0703, dự đoán ĐÚNG cả 9/9 điểm, và tổng xác suất ba lớp luôn bằng 1.0 tại mọi điểm. Đối chiếu one-vs-rest (ba mô hình nhị phân riêng, cùng lr/bước): chỉ đúng 6/9 điểm — lớp GIỮA (quảng cáo) không bao giờ thắng ở BẤT KỲ điểm nào, vì mỗi mô hình nhị phân chỉ biết 'lớp mình so với phần còn lại', không có cơ chế nào ép đúng MỘT lớp phải thắng."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.softmax]
requires: [ai.ranh-gioi-quyet-dinh]
concepts: [ai.softmax]
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
`sigmoid` cho đúng MỘT xác suất — đủ cho hai lớp. Ba lớp trở lên thì sao?
::::

::::explain{#mot-ham-cho-nhieu-lop}
Email không chỉ có hai loại. Byte muốn phân ba loại: `bình thường`, `quảng
cáo`, `spam`. `sigmoid` chỉ trả về MỘT con số — không đủ để nói "xác suất
thuộc lớp nào trong BA lớp". Cần một hàm nhận vào MỘT điểm số cho MỖI lớp
(gọi là **logit**, `zₖ = wₖ·x + bₖ` — mỗi lớp có `w`, `b` RIÊNG), rồi trả
về MỘT xác suất cho MỖI lớp, sao cho tất cả cộng lại đúng bằng `1`. Hàm đó
là **softmax**:

> `softmax(z)ₖ = e^(zₖ) / Σⱼ e^(zⱼ)`

Chia cho TỔNG của mọi lớp (không chỉ lớp `k`) là điều làm nên tính chất
"cộng lại bằng 1": tử số của lớp `k` chỉ là MỘT phần của đúng cái mẫu số
đó. Với đúng `K = 2` lớp, softmax rút gọn về ĐÚNG hình dạng của sigmoid
(tự kiểm bằng đại số: `e^(z₀)/(e^(z₀)+e^(z₁)) = 1/(1+e^(z₁−z₀))` — một
sigmoid trên hiệu hai logit) — softmax không phải một ý tưởng khác, mà là
BẢN MỞ RỘNG của sigmoid cho nhiều hơn hai lớp.

(Một chi tiết kỹ thuật: tính thẳng `e^z` có thể tràn số nếu `z` lớn — cách
sửa chuẩn là trừ đi giá trị lớn nhất của `z` trước khi lấy mũ, không đổi
kết quả toán học nhưng tránh tràn số. Bài này dùng cách sửa đó trong code,
chi tiết ĐẦY ĐỦ vì sao cần nó dành cho track sau.)

Huấn luyện softmax bằng gradient descent, dùng cross-entropy đa lớp
(`L = -(1/n)·Σᵢ log(p_{i, yᵢ})`, chỉ phạt XÁC SUẤT GÁN CHO ĐÚNG LỚP thật của
mỗi điểm). Đúng như bài `logistic-regression-tu-so-0` đã chỉ ra cho hai
lớp, đạo hàm ở đây GIỮ NGUYÊN hình dạng đẹp đó — mở rộng cho nhiều lớp:

> `∂L/∂wₖ = (1/n) · Σᵢ xᵢ·(p_{i,k} − y_{i,k})`

với `y_{i,k}` là `1` nếu điểm `i` thuộc lớp `k`, `0` nếu không (gọi là
**one-hot**). Vẫn `(dự_đoán − nhãn)·x`, chỉ lặp lại cho từng lớp.

Có một cách KHÁC để xử lý nhiều lớp mà không cần softmax: **one-vs-rest**
(một-so-với-phần-còn-lại) — huấn luyện `K` mô hình logistic NHỊ PHÂN độc
lập, mỗi mô hình trả lời đúng một câu hỏi "điểm này CÓ thuộc lớp `k` hay
KHÔNG", rồi khi dự đoán, chọn lớp có xác suất CAO NHẤT trong `K` mô hình
đó. Nghe hợp lý — nhưng `K` mô hình huấn luyện ĐỘC LẬP, không hề "biết"
tới nhau, và không có gì đảm bảo đúng MỘT mô hình sẽ thắng áp đảo tại mọi
điểm. Bài này đo trực tiếp hậu quả của điều đó.
::::

::::example{#softmax-doi-dau-one-vs-rest}
Chín email tự bịa, ba lớp theo số từ viết hoa (`0`=bình thường, `1`=quảng
cáo, `2`=spam):

```python title=readonly
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 4, 5, 6, 8, 9, 10], dtype=float)
loai = np.array([0, 0, 0, 1, 1, 1, 2, 2, 2])
n = len(so_tu_hoa)
K = 3

Y = np.zeros((n, K))
Y[np.arange(n), loai] = 1.0

def softmax(Z):
    Zs = Z - Z.max(axis=1, keepdims=True)   # tru max truoc khi mu hoa -- tranh tran so
    E = np.exp(Zs)
    return E / E.sum(axis=1, keepdims=True)

def huan_luyen_softmax(lr, so_buoc):
    W, B = np.zeros(K), np.zeros(K)
    for _ in range(so_buoc):
        Z = np.outer(so_tu_hoa, W) + B
        P = softmax(Z)
        dZ = (P - Y) / n
        W -= lr * (so_tu_hoa @ dZ)
        B -= lr * dZ.sum(axis=0)
    return W, B

W, B = huan_luyen_softmax(0.5, 500)
P = softmax(np.outer(so_tu_hoa, W) + B)
loss = -np.mean(np.sum(Y * np.log(P + 1e-12), axis=1))
pred_softmax = np.argmax(P, axis=1)

print("W =", np.round(W, 4).tolist())
print("B =", np.round(B, 4).tolist())
print("loss =", round(loss, 4))
print("tong xac suat moi hang:", np.round(P.sum(axis=1), 4).tolist())
print(f"du doan softmax: {pred_softmax.tolist()}  (that: {loai.tolist()})")
print(f"so diem dung (softmax): {int(np.sum(pred_softmax == loai))} / {n}")
```

```text title=readonly
W = [-2.0539, 0.1278, 1.9261]
B = [8.236, 2.0797, -10.3158]
loss = 0.0703
tong xac suat moi hang: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
du doan softmax: [0, 0, 0, 1, 1, 1, 2, 2, 2]  (that: [0, 0, 0, 1, 1, 1, 2, 2, 2])
so diem dung (softmax): 9 / 9
```

Softmax dự đoán ĐÚNG cả chín điểm, và tổng ba xác suất luôn ĐÚNG bằng `1.0`
tại mọi điểm — đúng tính chất đã hứa. Giờ, cùng dữ liệu, huấn luyện
one-vs-rest — ba mô hình logistic nhị phân độc lập, cùng `lr` và số bước:

```python title=readonly
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 4, 5, 6, 8, 9, 10], dtype=float)
loai = np.array([0, 0, 0, 1, 1, 1, 2, 2, 2])
n = len(so_tu_hoa)
K = 3

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def huan_luyen_nhi_phan(y_bin, lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        p = sigmoid(w * so_tu_hoa + b)
        dw = np.mean(so_tu_hoa * (p - y_bin))
        db = np.mean(p - y_bin)
        w -= lr * dw
        b -= lr * db
    return w, b

P_ovr = np.zeros((n, K))
for k in range(K):
    y_bin = (loai == k).astype(float)
    w, b = huan_luyen_nhi_phan(y_bin, 0.5, 500)
    P_ovr[:, k] = sigmoid(w * so_tu_hoa + b)

pred_ovr = np.argmax(P_ovr, axis=1)
print("P_ovr (lam tron 4 so):")
for hang in np.round(P_ovr, 4).tolist():
    print(" ", hang)
print(f"du doan one-vs-rest: {pred_ovr.tolist()}  (that: {loai.tolist()})")
print(f"so diem dung (one-vs-rest): {int(np.sum(pred_ovr == loai))} / {n}")
```

```text title=readonly
P_ovr (lam tron 4 so):
  [0.9982, 0.3464, 0.0001]
  [0.9833, 0.1725, 0.0004]
  [0.8635, 0.0758, 0.0015]
  [0.0678, 0.0125, 0.0214]
  [0.0077, 0.005, 0.0778]
  [0.0008, 0.002, 0.2455]
  [0.0, 0.0003, 0.8286]
  [0.0, 0.0001, 0.9491]
  [0.0, 0.0, 0.9863]
du doan one-vs-rest: [0, 0, 0, 0, 2, 2, 2, 2, 2]  (that: [0, 0, 0, 1, 1, 1, 2, 2, 2])
so diem dung (one-vs-rest): 6 / 9
```

Kết quả gây bất ngờ: one-vs-rest chỉ đúng `6/9`, và nhìn kỹ CỘT GIỮA
(`P_ovr[:, 1]`, xác suất "lớp quảng cáo") — không có hàng nào mà cột đó là
số LỚN NHẤT trong hàng. Mô hình "quảng cáo so với phần còn lại" KHÔNG BAO
GIỜ thắng, ở bất kỳ điểm nào, kể cả ba điểm thật sự thuộc lớp `1`
(`x=4,5,6`). Ba mô hình nhị phân được huấn luyện HOÀN TOÀN ĐỘC LẬP — mô
hình lớp `0` chỉ cố phân biệt "có phải lớp `0` không", không hề "biết" mô
hình lớp `1` hay lớp `2` đang nói gì. Với một lớp nằm Ở GIỮA hai lớp khác
trên trục dữ liệu (đúng tình huống của lớp "quảng cáo"), không mô hình nhị
phân nào có động lực đủ mạnh để thắng áp đảo — softmax, huấn luyện CÙNG
LÚC cả ba lớp với ràng buộc "tổng xác suất bằng 1", không gặp vấn đề này.
::::

::::predict{#doan-diem-x5 commitOnce}
Điểm `x = 5` có nhãn thật là `1` (quảng cáo) — nằm chính giữa dữ liệu.

**Trước khi đọc lại bảng trên**, bạn đoán: one-vs-rest có phân loại ĐÚNG
điểm này không?

:::opt{correct}
Không — one-vs-rest dự đoán lớp `2` (spam) cho điểm này, sai hẳn so với
nhãn thật `1`; chỉ softmax dự đoán đúng
:::

:::opt
Có — vì `x = 5` nằm giữa `4` và `6`, cả hai đều được cả softmax lẫn
one-vs-rest phân loại đúng, nên điểm giữa cũng phải đúng theo
::why
Gần đúng ở việc `x=4` và `x=6` đúng là được SOFTMAX phân loại đúng cả hai
— nhưng đó không phải điều one-vs-rest đã làm.

Chỗ lệch: nhìn lại đúng dòng dự đoán one-vs-rest — `[0, 0, 0, 0, 2, 2, 2,
2, 2]` — vị trí của `x=4` (chỉ số `3`) đã SAI (dự đoán `0`, thật là `1`),
và `x=5` (chỉ số `4`, dự đoán `2`, thật là `1`) cũng sai. One-vs-rest sai
liên tiếp trên CẢ BA điểm thuộc lớp giữa — không có điểm nào của lớp `1`
được nó gọi đúng.
::
:::

:::opt
Có, nhưng chỉ với softmax — one-vs-rest không áp dụng được cho bài toán
CÓ THỨ TỰ giữa các lớp (bình thường < quảng cáo < spam) như thế này
::why
Gần đúng ở kết luận CUỐI: đúng là chỉ softmax phân loại đúng điểm này.

Chỗ lệch nằm ở LÝ DO: one-vs-rest không thất bại vì các lớp "có thứ tự" —
về mặt thuật toán, one-vs-rest không hề biết (và không cần biết) các lớp
có thứ tự hay không. Nó thất bại vì đây là một lớp GIỮA hai lớp khác trên
trục đặc trưng — một tình huống hình học cụ thể khiến MỖI mô hình nhị
phân, huấn luyện riêng lẻ, không có động lực đủ mạnh để thắng áp đảo tại
đó. Vấn đề nằm ở CÁCH one-vs-rest huấn luyện (độc lập, không ràng buộc
tổng xác suất), không nằm ở việc lớp có thứ tự hay không.
::
:::
::::

::::code{#huan_luyen_softmax_3_lop}
Viết nốt `softmax` (chuẩn hoá `E` để tổng theo hàng bằng `1`) và bước cập
nhật của `huan_luyen_softmax` (dùng `dZ = (P − Y)/n`, rồi `dW = x·dZ`,
`dB = Σ dZ` theo cột).

```python title=starter
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 4, 5, 6, 8, 9, 10], dtype=float)
loai = np.array([0, 0, 0, 1, 1, 1, 2, 2, 2])
n = len(so_tu_hoa)
K = 3

Y = np.zeros((n, K))
Y[np.arange(n), loai] = 1.0

def softmax(Z):
    Zs = Z - Z.max(axis=1, keepdims=True)
    E = np.exp(Zs)
    return ___                          # E chia tong cua E theo hang (axis=1, keepdims=True)

def huan_luyen_softmax(lr, so_buoc):
    W, B = np.zeros(K), np.zeros(K)
    for _ in range(so_buoc):
        Z = np.outer(so_tu_hoa, W) + B
        P = softmax(Z)
        dZ = (P - Y) / n
        dW = ___                        # so_tu_hoa nhan ma tran voi dZ
        dB = ___                        # tong dZ theo cot (axis=0)
        W -= lr * dW
        B -= lr * dB
    return W, B

W, B = huan_luyen_softmax(0.5, 500)
P = softmax(np.outer(so_tu_hoa, W) + B)
loss = -np.mean(np.sum(Y * np.log(P + 1e-12), axis=1))

print(np.round(W, 4).tolist())
print(np.round(B, 4).tolist())
print(round(loss, 4))
```

```python title=solution
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 4, 5, 6, 8, 9, 10], dtype=float)
loai = np.array([0, 0, 0, 1, 1, 1, 2, 2, 2])
n = len(so_tu_hoa)
K = 3

Y = np.zeros((n, K))
Y[np.arange(n), loai] = 1.0

def softmax(Z):
    Zs = Z - Z.max(axis=1, keepdims=True)
    E = np.exp(Zs)
    return E / E.sum(axis=1, keepdims=True)

def huan_luyen_softmax(lr, so_buoc):
    W, B = np.zeros(K), np.zeros(K)
    for _ in range(so_buoc):
        Z = np.outer(so_tu_hoa, W) + B
        P = softmax(Z)
        dZ = (P - Y) / n
        dW = so_tu_hoa @ dZ
        dB = dZ.sum(axis=0)
        W -= lr * dW
        B -= lr * dB
    return W, B

W, B = huan_luyen_softmax(0.5, 500)
P = softmax(np.outer(so_tu_hoa, W) + B)
loss = -np.mean(np.sum(Y * np.log(P + 1e-12), axis=1))

print(np.round(W, 4).tolist())
print(np.round(B, 4).tolist())
print(round(loss, 4))
```

```python title=test
assert np.round(W, 4).tolist() == [-2.0539, 0.1278, 1.9261], f"W sai -- dang ra {np.round(W, 4).tolist()}"
assert np.round(B, 4).tolist() == [8.236, 2.0797, -10.3158], f"B sai -- dang ra {np.round(B, 4).tolist()}"
assert round(loss, 4) == 0.0703, f"loss phai la 0.0703 -- dang ra {round(loss, 4)}"
assert np.allclose(P.sum(axis=1), 1.0), "moi hang cua P phai co tong dung bang 1 -- dinh nghia cua softmax"
pred = np.argmax(P, axis=1)
assert np.all(pred == loai), f"softmax phai du doan DUNG ca 9 diem -- dang ra {pred.tolist()}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu chuẩn hoá `E` — chia cho tổng của chính `E` theo TỪNG HÀNG (`axis=1`, giữ chiều bằng `keepdims=True`, đã dùng đúng kiểu ở dòng `Zs` ngay trên). Hai chỗ dưới là gradient — `dW` nhân MA TRẬN `so_tu_hoa` (một chiều) với `dZ` (hai chiều) bằng `@`; `dB` cộng `dZ` theo CỘT (`axis=0`, gộp mọi điểm dữ liệu lại, giữ lại mỗi lớp một số).
- kind: strategy
  body: 'softmax: `E / E.sum(axis=1, keepdims=True)`. dW: `so_tu_hoa @ dZ`. dB: `dZ.sum(axis=0)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `E / E.sum(axis=1, keepdims=True)`, `so_tu_hoa @ dZ`, và `dZ.sum(axis=0)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: softmax phai THAT SU chia E cho tong cua chinh no (khong duoc tra ve E chua chuan hoa, se lam tong xac suat khac 1), dW phai nhan THAT voi so_tu_hoa (thieu no thi moi lop cap nhat giong het nhau, khong con phu thuoc dac trung), va dB phai tinh THAT tu dZ
  requireAst:
  - kind: uses-name, target: E, min: 2
  - kind: uses-name, target: so_tu_hoa, min: 4
  - kind: uses-name, target: dZ, min: 2
  # Da thu that (goi kiemAst that tren code day du):
  # - loi giai dung: dat=true, ca ba luat qua sach.
  # - softmax tra ve E (khong chuan hoa, thieu phep chia): E tut tu 2 xuong
  #   1 (mat lan Load trong bieu thuc chia) -- duoi nguong 2, bi chan.
  # - dW = dZ.sum(axis=0) (quen nhan so_tu_hoa, giong cong thuc cua dB):
  #   so_tu_hoa tut tu 4 xuong 3 (mat lan dung trong dW) -- duoi nguong 4,
  #   bi chan.
  # - dB = np.zeros(K) (hang so, khong bao gio cap nhat bias): dZ tut tu 2
  #   xuong 1 (mat lan dung trong dB) -- duoi nguong 2, bi chan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[-2\\.0539, 0\\.1278, 1\\.9261\\]\\n\\[8\\.236, 2\\.0797, -10\\.3158\\]\\n0\\.0703\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
6/9 so với 9/9 — cùng dữ liệu, cùng số bước, chỉ khác việc ba lớp có được
huấn luyện CÙNG LÚC (softmax) hay tách rời (one-vs-rest).
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Softmax dự đoán đúng cả chín điểm — một kết quả HOÀN HẢO trên chính dữ
liệu đã dùng để huấn luyện. Nhưng "đúng cả chín điểm" chỉ là MỘT con số —
**accuracy** (tỉ lệ dự đoán đúng). Với hai lớp lệch nhau rất nhiều về số
lượng (ví dụ 99 email bình thường, chỉ 1 email spam), một mô hình "luôn
đoán bình thường" cũng đạt accuracy rất cao — mà vẫn hoàn toàn VÔ DỤNG cho
việc thật sự cần làm: bắt được email spam hiếm hoi đó.

Trước khi đo những chỉ số tinh vi hơn, cần một công cụ đơn giản hơn: đếm
CHÍNH XÁC từng loại đúng/sai theo bốn cách khác nhau, không chỉ một con số
tổng accuracy. Bài sau giới thiệu công cụ đó.
::::

::::checkpoint{mastery=0.8}
::::
