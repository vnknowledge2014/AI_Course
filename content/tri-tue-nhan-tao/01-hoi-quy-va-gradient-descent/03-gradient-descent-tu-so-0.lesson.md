---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.gradient-descent-tu-so-0
title: "Gradient descent từ số 0"
summary: "Trên bài toán giá nhà của T8.1.1 (7 điểm, w,b khởi tạo 0): learning rate vừa (0.0001) đưa loss từ 2 116 107 xuống 3706.2 chỉ sau 8 bước; learning rate quá nhỏ (0.000001) sau 8 bước loss vẫn còn 1 861 299 — gần như chưa nhúc nhích; learning rate quá lớn (0.0003) làm loss NỔ từ 2 116 107 lên 462 922 134 chỉ sau 8 bước, với w đảo dấu liên tục mỗi bước — phân kỳ thấy rõ bằng số thật, không chỉ bằng lời."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.gradient-descent]
requires: [ai.ham-mat-mat]
concepts: [ai.gradient-descent]
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
Không dò công thức đóng nữa. Lần này, `w` và `b` bắt đầu từ 0 — và TỰ BƯỚC
dần tới đáp án, từng bước một.
::::

::::explain{#dao-ham-va-buoc-cap-nhat}
Bài 1 giải `w`, `b` tối ưu bằng một công thức đóng — chỉ tồn tại vì bài
toán "khớp một đường thẳng" đơn giản. **Gradient descent** là một cách khác:
bắt đầu từ một cặp `(w, b)` bất kỳ (thường là `0, 0`), rồi LẶP LẠI một bước
nhỏ đưa `(w, b)` tiến dần về phía làm MSE giảm — không cần công thức đóng
cho bài toán cụ thể, chỉ cần biết đạo hàm của hàm mất mát.

MSE theo `w` và `b` là:

> `L(w, b) = (1/n) · Σ (yᵢ − (w·xᵢ + b))²`

Đạo hàm riêng của `L` theo từng biến (tốc độ `L` đổi khi CHỈ biến đó nhích,
biến kia đứng yên) là:

> `∂L/∂w = −(2/n) · Σ xᵢ·(yᵢ − dự_đoánᵢ)`
>
> `∂L/∂b = −(2/n) · Σ (yᵢ − dự_đoánᵢ)`

Hai đạo hàm này gộp lại gọi là **gradient** — một cặp số chỉ HƯỚNG mà `L`
tăng nhanh nhất nếu `(w, b)` nhích theo hướng đó. Muốn `L` GIẢM, nhích
`(w, b)` theo hướng NGƯỢC gradient — đây là lý do MSE (đạo hàm mượt khắp
nơi, bài trước đã chỉ ra) dễ dùng hơn MAE cho việc này.

Bước cập nhật, lặp lại nhiều lần:

> `w ← w − lr · ∂L/∂w`
>
> `b ← b − lr · ∂L/∂b`

`lr` (learning rate) là một số dương nhỏ, quyết định bước nhích DÀI hay
NGẮN mỗi lần. Cùng một công thức, ba giá trị `lr` khác nhau cho ra ba số
phận khác nhau hẳn — bài này đo cả ba bằng số thật, trên chính bảy điểm
giá nhà của bài 1.
::::

::::example{#ba-so-phan-cua-learning-rate}
Ba learning rate khác nhau, cùng khởi động từ `w=0, b=0`, cùng chạy 8
bước trên bảy điểm giá nhà của bài 1 (`dien_tich`/`gia`). Loss ban đầu
(tại `w=0, b=0`) là `2 116 107.14`:

```python title=readonly
import numpy as np

dien_tich = np.array([30, 40, 50, 60, 70, 80, 90], dtype=float)
gia = np.array([780, 1010, 1170, 1425, 1585, 1820, 1990], dtype=float)
n = len(dien_tich)

def mat_mat(w, b):
    du_doan = w * dien_tich + b
    return np.mean((gia - du_doan) ** 2)

def gradient(w, b):
    du_doan = w * dien_tich + b
    sai_so = gia - du_doan
    dw = -2 / n * np.sum(dien_tich * sai_so)
    db = -2 / n * np.sum(sai_so)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    losses = [mat_mat(w, b)]
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w = w - lr * dw
        b = b - lr * db
        losses.append(mat_mat(w, b))
    return w, b, losses

for ten, lr in [("qua nho", 0.000001), ("vua", 0.0001), ("qua lon", 0.0003)]:
    w, b, losses = huan_luyen(lr, 8)
    print(ten, "lr=" + str(lr), "-> loss sau 8 buoc:", round(losses[-1], 2))
```

```text title=readonly
qua nho lr=1e-06 -> loss sau 8 buoc: 1861299.03
vua lr=0.0001 -> loss sau 8 buoc: 3706.2
qua lon lr=0.0003 -> loss sau 8 buoc: 462922133.91
```

`lr` **quá nhỏ** (`0.000001`): sau 8 bước, loss mới giảm từ `2 116 107` xuống
`1 861 299` — giảm chỉ khoảng 12%. Bước nhích quá ngắn để tiến được bao xa.

`lr` **vừa** (`0.0001`): sau 8 bước, loss đã tụt xuống `3706.2` — gần sát
mức tối ưu `363.14` mà bài 1 tính được bằng công thức đóng.

`lr` **quá lớn** (`0.0003`): loss KHÔNG giảm — nó tăng vọt, từ `2 116 107`
lên `462 922 134`, gấp hơn 200 lần, chỉ sau 8 bước. Nhìn `w` qua từng bước
sẽ thấy tại sao: `55.15 → −22.09 → 86.09 → −65.42 → 146.78 → −150.42 → ...`
— đảo dấu liên tục, mỗi lần vọt xa hơn lần trước. Bước nhích quá dài khiến
mỗi lần cập nhật "nhảy vọt qua" điểm tối ưu, rồi lại nhảy vọt theo hướng
ngược lại, xa hơn — đây là hiện tượng **phân kỳ** (divergence), không phải
một dạng hội tụ chậm.
::::

::::predict{#doan-vi-tri-hoi-tu commitOnce}
Vẫn ba `lr` ở trên, cùng chạy trên bảy điểm giá nhà, cùng xuất phát từ
`w=0, b=0`.

**Trước khi đọc lại bảng trên**, bạn đoán: sau ĐÚNG một bước cập nhật đầu
tiên (không phải 8 bước), `lr` nào cho ra loss THẤP NHẤT?

:::opt{correct}
`lr = 0.0001` (mức "vừa") — bước đầu đã đưa loss từ `2 116 107` xuống còn
khoảng `88 051`, thấp hơn hẳn cả `lr` quá nhỏ lẫn `lr` quá lớn
:::

:::opt
`lr = 0.0003` (mức "quá lớn") — bước nhích dài nhất, hẳn phải tiến xa nhất
về phía tối ưu trong MỘT bước
::why
Gần đúng ở việc bạn suy luận đúng CHIỀU: bước nhích dài hơn thì DI CHUYỂN
xa hơn trong không gian `(w, b)` — quan sát đó không sai.

Chỗ lệch: "di chuyển xa" không có nghĩa "di chuyển ĐÚNG HƯỚNG một cách hợp
lý". Gradient chỉ đúng hướng TẠI điểm hiện tại — nó không đảm bảo hướng đó
vẫn đúng sau khi đã đi một quãng quá dài. Với `lr = 0.0003`, bước đầu đã
đưa loss lên `4 147 205` — TĂNG so với `2 116 107` ban đầu, không hề giảm.
Bước nhích dài đã "vọt qua" điểm tối ưu, rơi vào một vùng còn tệ hơn.
::
:::

:::opt
`lr = 0.000001` (mức "quá nhỏ") — bước nhích thận trọng nhất nên hẳn phải
an toàn nhất, cho loss thấp nhất
::why
Gần đúng ở việc "quá nhỏ" đúng là AN TOÀN theo nghĩa không phân kỳ — loss
của nó không hề tăng vọt như `lr = 0.0003`.

Chỗ lệch: an toàn không đồng nghĩa hiệu quả. Sau một bước, `lr = 0.000001`
chỉ đưa loss từ `2 116 107` xuống khoảng `2 082 436` — giảm chưa tới 2%.
`lr = 0.0001` giảm mạnh hơn nhiều (`2 116 107 → 88 051`) mà vẫn không phân
kỳ — nó nằm đúng ở vùng "đủ lớn để tiến nhanh, đủ nhỏ để không vọt quá".
::
:::
::::

::::code{#viet_gradient_descent}
Viết nốt `gradient` (đạo hàm riêng theo `w` và `b`) và `huan_luyen` (vòng
lặp cập nhật), rồi so sánh loss sau 8 bước giữa `lr` vừa (`0.0001`) và `lr`
quá nhỏ (`0.000001`).

```python title=starter
import numpy as np

dien_tich = np.array([30, 40, 50, 60, 70, 80, 90], dtype=float)
gia = np.array([780, 1010, 1170, 1425, 1585, 1820, 1990], dtype=float)
n = len(dien_tich)

def mat_mat(w, b):
    du_doan = w * dien_tich + b
    return np.mean((gia - du_doan) ** 2)

def gradient(w, b):
    du_doan = w * dien_tich + b
    sai_so = gia - du_doan
    dw = ___                      # -2/n * tong(dien_tich * sai_so)
    db = ___                      # -2/n * tong(sai_so)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w = ___                   # w tru lr nhan dw
        b = ___                   # b tru lr nhan db
    return w, b

w_vua, b_vua = huan_luyen(0.0001, 8)
w_nho, b_nho = huan_luyen(0.000001, 8)

print(round(mat_mat(w_vua, b_vua), 2))
print(round(mat_mat(w_nho, b_nho), 2))
```

```python title=solution
import numpy as np

dien_tich = np.array([30, 40, 50, 60, 70, 80, 90], dtype=float)
gia = np.array([780, 1010, 1170, 1425, 1585, 1820, 1990], dtype=float)
n = len(dien_tich)

def mat_mat(w, b):
    du_doan = w * dien_tich + b
    return np.mean((gia - du_doan) ** 2)

def gradient(w, b):
    du_doan = w * dien_tich + b
    sai_so = gia - du_doan
    dw = -2 / n * np.sum(dien_tich * sai_so)
    db = -2 / n * np.sum(sai_so)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w = w - lr * dw
        b = b - lr * db
    return w, b

w_vua, b_vua = huan_luyen(0.0001, 8)
w_nho, b_nho = huan_luyen(0.000001, 8)

print(round(mat_mat(w_vua, b_vua), 2))
print(round(mat_mat(w_nho, b_nho), 2))
```

```python title=test
assert round(mat_mat(w_vua, b_vua), 2) == 3706.2, f"loss voi lr vua sau 8 buoc phai la 3706.2 -- dang ra {round(mat_mat(w_vua, b_vua), 2)}"
assert round(mat_mat(w_nho, b_nho), 2) == 1861299.03, f"loss voi lr qua nho sau 8 buoc phai la 1861299.03 -- dang ra {round(mat_mat(w_nho, b_nho), 2)}"
assert mat_mat(w_vua, b_vua) < mat_mat(w_nho, b_nho), "lr vua phai cho loss THAP HON han lr qua nho sau cung 8 buoc"
assert w_vua > 0 and w_nho > 0, "ca hai w phai duong -- gia nha tang theo dien tich"
```

:::hints
- kind: attention
  body: Bốn chỗ trống chia hai việc. Hai chỗ trong `gradient` là công thức đạo hàm riêng đã viết ở phần giải thích — dùng đúng `sai_so` và `dien_tich` đã có sẵn trong hàm. Hai chỗ trong `huan_luyen` là bước cập nhật `w ← w − lr·dw` và `b ← b − lr·db` — dấu TRỪ, không phải cộng, vì đi NGƯỢC hướng gradient mới làm loss giảm.
- kind: strategy
  body: 'gradient: `dw = -2 / n * np.sum(dien_tich * sai_so)`, `db = -2 / n * np.sum(sai_so)`. huan_luyen: `w = w - lr * dw`, `b = b - lr * db`.'
- kind: one-line
  body: 'Bốn chỗ trống: `-2 / n * np.sum(dien_tich * sai_so)`, `-2 / n * np.sum(sai_so)`, `w - lr * dw`, `b - lr * db`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: gradient phải tính từ sai_so và dien_tich (không phải hằng số cố định), và bước cập nhật phải TRỪ đi lr nhân đạo hàm (không phải cộng, không phải đảo dấu) -- kiểm bằng cách đếm số lần các tên này thật sự được dùng
  requireAst:
  - kind: uses-name, target: sai_so, min: 2
  - kind: uses-name, target: dw, min: 2
  - kind: uses-name, target: db, min: 2
  - kind: uses-name, target: dien_tich, min: 4
  - kind: uses-operator, target: "-", min: 4
  # Da thu that (ast.parse, khong doan): dien True/1/0 vao ca bon cho trong
  # cho [sai_so=0, dw=1, db=1, dien_tich=3, "-"=2] -- duoi ca nam nguong (can
  # 2/2/2/4/4). Doi dau cong-tru trong hai buoc cap nhat (w=w+lr*dw thay vi
  # w-lr*dw) giu nguyen [sai_so=2, dw=2, db=2, dien_tich=4] nhung "-" tut con
  # 2 (chi con hai lan "-" co san trong gradient, mat hai lan cua buoc cap
  # nhat) -- duoi nguong 4, bi chan. Loi giai that cho dung [2,2,2,4,4].
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^3706\\.2\\n1861299\\.03\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`lr` vừa: gần sát tối ưu chỉ sau 8 bước. `lr` quá nhỏ: ì ạch. `lr` quá lớn:
nổ tung. Ba con số thật, ba số phận khác hẳn nhau — cùng MỘT công thức.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`lr = 0.0001` hoạt động tốt trên bài toán này — diện tích chạy từ `30` tới
`90`, giá chạy từ `780` tới `1990`. Nhưng nếu diện tích đổi đơn vị, đo bằng
mm2 thay vì m2 (nhân mọi giá trị `x` lên `1 000 000` lần), thì CÙNG `lr` đó
có còn hoạt động tốt không? Gradient `∂L/∂w` phụ thuộc trực tiếp vào độ lớn
của `x` — đổi thang đo của `x`, gradient cũng đổi thang đo theo.

Nói cách khác: `lr` "vừa" cho một bài toán có thể là `lr` "quá lớn" hoặc
"quá nhỏ" cho một bài toán CÙNG BẢN CHẤT nhưng khác THANG ĐO. Có cách nào
làm cho việc chọn `lr` bớt phụ thuộc vào đơn vị đo của dữ liệu không?

Bài sau, sau khi ghé qua đặc trưng đa thức, sẽ trả lời trực tiếp câu hỏi
này.
::::

::::checkpoint{mastery=0.8}
::::
