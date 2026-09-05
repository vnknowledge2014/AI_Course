---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.logistic-regression-tu-so-0
title: "Logistic regression từ số 0"
summary: "Tám email tự bịa (số từ viết hoa: 0,1,2,3,5,6,7,9; nhãn spam: 0,0,0,1,1,1,1,1) khớp bằng logistic regression huấn luyện qua gradient descent (500 bước, lr=0.1, khởi tạo w=b=0): ra w≈1.3885, b≈-3.2266, cross-entropy loss≈0.1302 (từ 0.6931 ban đầu) — gradient của cross-entropy hoá ra có ĐÚNG hình dạng (1/n)Σx(p−y), giống hệt gradient MSE của track trước (2/n)Σx(ŷ−y) chỉ khác hệ số 2 và ŷ giờ là sigmoid(wx+b) thay vì tuyến tính thẳng."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.logistic-regression]
requires: [ai.boss-hoi-quy-va-gradient-descent]
concepts: [ai.logistic-regression]
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
Track trước dự đoán một CON SỐ — giá nhà. Giờ đổi câu hỏi: không phải "bao
nhiêu", mà "có hay không" — email này có phải spam?
::::

::::explain{#tu-so-sang-nhan}
Chín track — à, mười bài — của `hoi-quy-va-gradient-descent` đều dự đoán
một con số liên tục: giá nhà có thể là bất kỳ số thực nào. Nhưng nhiều câu
hỏi thực tế chỉ có hai đáp án: email này CÓ phải spam hay KHÔNG, khối u này
LÀNH hay ÁC, giao dịch này AN TOÀN hay GIAN LẬN. Đây là bài toán **phân
loại nhị phân** (binary classification) — nhãn `y` chỉ nhận đúng hai giá
trị, quy ước `0` hoặc `1`.

Đưa thẳng `y = w·x + b` vào bài toán này có một vấn đề rõ ràng: vế phải có
thể ra bất kỳ số thực nào — âm vô cực tới dương vô cực — trong khi nhãn chỉ
có `0` hoặc `1`. Cần một hàm NÉN mọi số thực về khoảng `(0, 1)`, để đọc kết
quả như một XÁC SUẤT. Hàm đó gọi là **sigmoid**:

> `sigmoid(z) = 1 / (1 + e⁻ᶻ)`

Khi `z` rất lớn (dương), `e⁻ᶻ` gần `0`, `sigmoid(z)` tiến gần `1`. Khi `z`
rất nhỏ (âm), `e⁻ᶻ` rất lớn, `sigmoid(z)` tiến gần `0`. Tại `z = 0`,
`sigmoid(0) = 1/(1+1) = 0.5` — đúng điểm giữa. Mô hình **logistic
regression** định nghĩa: `z = w·x + b` (y hệt hồi quy tuyến tính), rồi
`p = sigmoid(z)` là xác suất dự đoán `y = 1`.

Hàm mất mát cũng phải đổi. `ham-mat-mat-mse-mae` (T8.1a) dùng MSE cho hồi
quy — nhưng ghép MSE với sigmoid tạo ra một mặt mất mát KHÔNG LỒI (không
chỉ có một đáy duy nhất, mà nhiều "hố" cục bộ) vì sigmoid tự nó đã là một
hàm phi tuyến bên trong phép bình phương — gradient descent dễ mắc kẹt ở
một hố không phải hố sâu nhất. Cách chuẩn cho phân loại là **binary
cross-entropy**:

> `L = -(1/n) · Σ [yᵢ·log(pᵢ) + (1−yᵢ)·log(1−pᵢ)]`

Nhìn kỹ: khi `yᵢ = 1`, số hạng còn lại là `-log(pᵢ)` — phạt NẶNG nếu `pᵢ`
gần `0` (dự đoán tự tin nhưng SAI), phạt NHẸ nếu `pᵢ` gần `1` (dự đoán tự
tin và ĐÚNG). Khi `yᵢ = 0`, đối xứng ngược lại qua số hạng `-log(1−pᵢ)`. Ghép
với sigmoid, cross-entropy tạo ra một mặt mất mát LỒI — đúng một đáy, không
có hố giả nào để gradient descent mắc kẹt.

Điều bất ngờ nằm ở đạo hàm. Tính `∂L/∂w` và `∂L/∂b` bằng quy tắc chuỗi (đạo
hàm của `log(p)` nhân đạo hàm của `sigmoid` nhân đạo hàm của `z` theo `w`)
— các số hạng phức tạp TRIỆT TIÊU lẫn nhau theo một cách đẹp, ra kết quả:

> `∂L/∂w = (1/n) · Σ xᵢ·(pᵢ − yᵢ)`
>
> `∂L/∂b = (1/n) · Σ (pᵢ − yᵢ)`

So với gradient MSE của `gradient-descent-tu-so-0` (viết lại theo cùng
chiều dấu): `∂L/∂w = (2/n)·Σxᵢ·(dự_đoánᵢ − yᵢ)`. ĐÚNG CÙNG HÌNH DẠNG —
`(hằng_số/n)·Σx·(dự_đoán − y)` — chỉ khác hệ số (`1` thay vì `2`, hai hệ số
đó triệt tiêu khác nhau trong hai phép đạo hàm khác nhau) và khác định
nghĩa của "dự đoán" (`sigmoid(wx+b)` thay vì `wx+b` trần trụi). Đây không
phải trùng hợp ngẫu nhiên — nó là lý do cross-entropy được CHỌN làm hàm mất
mát chuẩn cho phân loại: nó cho một gradient đơn giản y hệt hồi quy tuyến
tính, dù công thức mất mát ban đầu (`log`, tích) trông phức tạp hơn nhiều.
Nghĩa là TOÀN BỘ vòng lặp gradient descent đã viết ở T8.1a dùng lại được
gần như nguyên vẹn — chỉ đổi định nghĩa của "dự đoán".
::::

::::example{#huan-luyen-loc-thu-spam}
Tám email tự bịa, một đặc trưng duy nhất: số từ viết hoa. Nhãn: `1` là
spam, `0` là không:

```python title=readonly
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
la_spam = np.array([0, 0, 0, 1, 1, 1, 1, 1], dtype=float)
n = len(so_tu_hoa)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def mat_mat(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    eps = 1e-12
    return -np.mean(la_spam * np.log(p + eps) + (1 - la_spam) * np.log(1 - p + eps))

def gradient(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    dw = np.mean(so_tu_hoa * (p - la_spam))
    db = np.mean(p - la_spam)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w -= lr * dw
        b -= lr * db
    return w, b

print("loss tai w=0,b=0:", round(mat_mat(0.0, 0.0), 4))

w, b = huan_luyen(0.1, 500)
print("w =", round(w, 4))
print("b =", round(b, 4))
print("loss sau 500 buoc:", round(mat_mat(w, b), 4))
print("xac suat du doan tren train:", np.round(sigmoid(w * so_tu_hoa + b), 4).tolist())
```

```text title=readonly
loss tai w=0,b=0: 0.6931
w = 1.3885
b = -3.2266
loss sau 500 buoc: 0.1302
xac suat du doan tren train: [0.0382, 0.1373, 0.3894, 0.7188, 0.9762, 0.994, 0.9985, 0.9999]
```

Tại `w=0, b=0`, mọi `z` bằng `0`, mọi `p` bằng `0.5` — loss ban đầu
`0.6931` chính là `-log(0.5)`, đúng mức "không biết gì cả". Sau `500` bước,
loss tụt xuống `0.1302`, và `w > 0` xác nhận đúng trực giác: càng nhiều từ
viết hoa, xác suất spam càng cao. Nhìn xác suất dự đoán trên chính tám
điểm train: điểm có `3` từ viết hoa (nhãn `1`) được gán xác suất `0.7188` —
đúng phía trên `0.5`; điểm có `2` từ viết hoa (nhãn `0`) được gán `0.3894`
— đúng phía dưới `0.5`. Mô hình phân tách hai lớp khá rõ, dù chỉ dùng một
đặc trưng số duy nhất.
::::

::::predict{#doan-xac-suat-email-moi commitOnce}
Mô hình đã khớp xong: `w ≈ 1.3885`, `b ≈ -3.2266`. Một email MỚI, có đúng
`4` từ viết hoa — con số này CHƯA hề xuất hiện trong tám điểm train (train
chỉ có `0,1,2,3,5,6,7,9`, thiếu đúng số `4`).

**Trước khi tính**, bạn đoán: mô hình dự đoán xác suất spam của email này
khoảng bao nhiêu?

:::opt{correct}
Khoảng `0.9111` — `sigmoid(1.3885 × 4 − 3.2266) ≈ sigmoid(2.3272) ≈ 0.9111`
:::

:::opt
Khoảng `2.3272` — chỉ cần tính `z = w·x + b` là xong, không cần thêm bước
nào nữa
::why
Gần đúng ở phép tính `z = w × 4 + b ≈ 2.3272` — con số đó không sai, và nó
đúng là bước ĐẦU của quy trình dự đoán.

Chỗ lệch: `z` chưa phải xác suất — nó là một số THỰC bất kỳ (ở đây còn lớn
hơn `1`, không thể là một xác suất hợp lệ). Xác suất phải nằm trong khoảng
`(0, 1)`, và bước NÉN `z` về khoảng đó chính là `sigmoid` — thiếu bước này,
`2.3272` chỉ là một toạ độ trung gian, không phải câu trả lời cho câu hỏi
"xác suất bao nhiêu".
::
:::

:::opt
Khoảng `0.0889` — lấy `1` trừ đi xác suất spam để ra xác suất "không phải
spam" của email này
::why
Gần đúng ở việc bạn có tính đúng `sigmoid(z) ≈ 0.9111` trước, rồi mới trừ
`1` — phép trừ đó tự nó không sai TOÁN HỌC.

Chỗ lệch: câu hỏi hỏi xác suất SPAM (`p = sigmoid(z)`), không phải xác
suất KHÔNG spam (`1 − p`). `0.0889` là câu trả lời đúng cho một câu hỏi
khác — "xác suất email này KHÔNG phải spam là bao nhiêu" — chứ không phải
câu hỏi đã hỏi.
::
:::
::::

::::code{#huan_luyen_logistic_regression}
Viết nốt `sigmoid` và `gradient` (đạo hàm riêng của cross-entropy theo `w`
và `b`, đúng công thức `(1/n)·Σx·(p−y)` và `(1/n)·Σ(p−y)` vừa học), rồi
huấn luyện qua `500` bước gradient descent.

```python title=starter
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
la_spam = np.array([0, 0, 0, 1, 1, 1, 1, 1], dtype=float)
n = len(so_tu_hoa)

def sigmoid(z):
    return ___                       # 1 / (1 + e^-z)

def mat_mat(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    eps = 1e-12
    return -np.mean(la_spam * np.log(p + eps) + (1 - la_spam) * np.log(1 - p + eps))

def gradient(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    dw = ___                          # (1/n) * tong(so_tu_hoa * (p - la_spam))
    db = ___                          # (1/n) * tong(p - la_spam)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w -= lr * dw
        b -= lr * db
    return w, b

w, b = huan_luyen(0.1, 500)

print(round(w, 4))
print(round(b, 4))
print(round(mat_mat(w, b), 4))
```

```python title=solution
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
la_spam = np.array([0, 0, 0, 1, 1, 1, 1, 1], dtype=float)
n = len(so_tu_hoa)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def mat_mat(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    eps = 1e-12
    return -np.mean(la_spam * np.log(p + eps) + (1 - la_spam) * np.log(1 - p + eps))

def gradient(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    dw = np.mean(so_tu_hoa * (p - la_spam))
    db = np.mean(p - la_spam)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w -= lr * dw
        b -= lr * db
    return w, b

w, b = huan_luyen(0.1, 500)

print(round(w, 4))
print(round(b, 4))
print(round(mat_mat(w, b), 4))
```

```python title=test
assert round(w, 4) == 1.3885, f"w phai la 1.3885 -- dang ra {round(w, 4)}"
assert round(b, 4) == -3.2266, f"b phai la -3.2266 -- dang ra {round(b, 4)}"
assert round(mat_mat(w, b), 4) == 0.1302, f"loss sau 500 buoc phai la 0.1302 -- dang ra {round(mat_mat(w, b), 4)}"
assert w > 0, "cang nhieu tu viet hoa thi xac suat spam phai cang cao -- w phai duong"
p4 = 1 / (1 + np.exp(-(w * 4 + b)))
assert p4 > 0.5, "email co 4 tu viet hoa phai duoc du doan la spam (xac suat > 0.5)"
```

:::hints
- kind: attention
  body: Ba chỗ trống đều là CÔNG THỨC, không phải con số chép sẵn. Chỗ đầu là `sigmoid(z)` đã viết ở phần giải thích — `1 / (1 + e^-z)`, dùng `np.exp` cho `e^-z`. Hai chỗ dưới là đạo hàm riêng của cross-entropy — cùng hình dạng `(1/n)·Σ(...)`, tính bằng `np.mean` thay vì tự chia `n`.
- kind: strategy
  body: 'sigmoid: `1 / (1 + np.exp(-z))`. dw: `np.mean(so_tu_hoa * (p - la_spam))`. db: `np.mean(p - la_spam)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `1 / (1 + np.exp(-z))`, `np.mean(so_tu_hoa * (p - la_spam))`, và `np.mean(p - la_spam)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: sigmoid phai THAT SU dung np.exp, va ca dw lan db phai tinh tu p/so_tu_hoa/la_spam that su -- khong duoc chep hang so hay quen nhan so_tu_hoa trong dw (thieu no se lam dw sai du van "trong giong" mot gradient)
  requireAst:
  - kind: uses-call, target: exp, min: 1
  - kind: uses-name, target: p, min: 2
  - kind: uses-name, target: la_spam, min: 4
  - kind: uses-name, target: so_tu_hoa, min: 4
  # Da thu that (goi kiemAst that): dien sigmoid tra ve "z" (khong exp), va
  # dw=1.0/db=0.0 (hang so) cho exp=0, la_spam=3 -- duoi ca hai nguong (can
  # 1 va 4), bi chan. Rieng kieu "gan dung" hon: dw = np.mean(p - la_spam)
  # (QUEN nhan so_tu_hoa, dung dung cong thuc cua db cho ca hai) van giu
  # nguyen p=2, la_spam=4 nhung so_tu_hoa tut con 3 (mat lan dung trong dw)
  # -- duoi nguong 4, van bi chan rieng boi luat so_tu_hoa. Loi giai that cho
  # dung [exp=1, p=2, la_spam=4, so_tu_hoa=4], qua sach ca bon.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^1\\.3885\\n-3\\.2266\\n0\\.1302\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Loss từ `0.6931` xuống `0.1302` — và công thức cập nhật hoá ra không hề xa
lạ. Vẫn `w ← w − lr·∂L/∂w`, chỉ đổi định nghĩa của dự đoán.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mô hình vừa khớp cho ra `w`, `b` — và từ đó, một xác suất cho MỌI giá trị
`x` có thể có, không chỉ tám điểm train. Nhưng để phân loại một email cụ
thể là "spam" hay "không spam" (một QUYẾT ĐỊNH nhị phân, không phải một
xác suất), cần thêm một bước: chọn một NGƯỠNG (thường là `0.5`) để cắt xác
suất thành hai phía.

Ngưỡng đó vạch ra một điểm cụ thể trên trục `x` — phía trên điểm đó dự đoán
`1`, phía dưới dự đoán `0`. Điểm đó nằm ở đâu, và nó có ý nghĩa hình học gì?
::::

::::checkpoint{mastery=0.8}
::::
