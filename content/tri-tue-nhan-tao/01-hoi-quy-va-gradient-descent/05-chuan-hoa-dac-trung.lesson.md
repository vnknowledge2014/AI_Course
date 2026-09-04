---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.chuan-hoa-dac-trung
title: "Chuẩn hoá đặc trưng"
summary: "Trên bài toán giá nhà (diện tích trung bình 60, độ lệch chuẩn 20): chuẩn hoá diện tích về thang [-1.5, 1.5] rồi chạy gradient descent với lr=0.1 chỉ cần 25 bước để loss xuống dưới 400 (còn 393.33); không chuẩn hoá, lr lớn nhất KHÔNG phân kỳ được là 0.0001, và phải mất tới 112 719 bước mới xuống dưới cùng ngưỡng — chênh nhau hơn 4500 lần về số bước."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.chuan-hoa-dac-trung]
requires: [ai.dac-trung-da-thuc]
concepts: [ai.chuan-hoa-dac-trung]
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
Bài `gradient-descent-tu-so-0` để lại một câu hỏi treo lơ lửng: đổi thang đo
của `x`, learning rate "vừa" có còn vừa không?
::::

::::explain{#chuan-hoa-la-gi}
Diện tích trong bài toán giá nhà chạy từ `30` tới `90` — trung bình `60`,
**độ lệch chuẩn** (một con số đo dữ liệu "tản ra" xa trung bình bao nhiêu,
tính bằng căn bậc hai của trung bình các bình phương độ lệch) đúng bằng
`20`. Gradient theo `w` (bài trước đã viết: `∂L/∂w = −(2/n)·Σxᵢ·sai_sốᵢ`)
tỷ lệ THUẬN với chính giá trị của `x` — `x` càng lớn, gradient theo `w`
càng lớn, và bước cập nhật `lr · ∂L/∂w` càng dễ "vọt quá xa" nếu `lr` không
đủ nhỏ để bù lại.

**Chuẩn hoá** (standardize) đưa mọi đặc trưng về CÙNG một thang đo, không
phụ thuộc đơn vị gốc: trừ đi trung bình, rồi chia cho độ lệch chuẩn.

> `x_chuẩn = (x − trung_bình) / độ_lệch_chuẩn`

Sau phép biến đổi này, `x_chuẩn` luôn có trung bình `0` và độ lệch chuẩn
`1` — bất kể `x` gốc đo bằng m2, mm2, hay bất kỳ đơn vị nào. Diện tích
`[30, 40, 50, 60, 70, 80, 90]` (trung bình `60`, độ lệch chuẩn `20`) chuẩn
hoá thành `[-1.5, -1, -0.5, 0, 0.5, 1, 1.5]` — một thang đo gọn, đối xứng
quanh `0`.

Vì gradient tỷ lệ thuận với thang đo của `x`, chuẩn hoá làm gradient theo
`w` nằm trong một khoảng dự đoán được, không phụ thuộc đơn vị đo gốc —
NHỜ đó có thể chọn một `lr` LỚN hơn nhiều mà vẫn không phân kỳ, và mỗi bước
tiến được xa hơn. Bài này đo trực tiếp: cùng bài toán giá nhà, chuẩn hoá
giúp gradient descent hội tụ nhanh hơn bao nhiêu bước.
::::

::::example{#so-buoc-hoi-tu-truoc-sau-chuan-hoa}
Cùng bảy điểm giá nhà, cùng đích "loss xuống dưới `400`" (gần sát mức tối
ưu `363.14` của bài 1) — đếm số bước cần thiết, có chuẩn hoá và không:

```python title=readonly
import numpy as np

dien_tich = np.array([30, 40, 50, 60, 70, 80, 90], dtype=float)
gia = np.array([780, 1010, 1170, 1425, 1585, 1820, 1990], dtype=float)
n = len(dien_tich)

trung_binh = dien_tich.mean()
do_lech_chuan = dien_tich.std()
x_chuan = (dien_tich - trung_binh) / do_lech_chuan
print("trung binh:", trung_binh, " do lech chuan:", do_lech_chuan)
print("x_chuan:", x_chuan.tolist())

def mat_mat(w, b, x):
    du_doan = w * x + b
    return np.mean((gia - du_doan) ** 2)

def gradient(w, b, x):
    du_doan = w * x + b
    sai_so = gia - du_doan
    dw = -2 / n * np.sum(x * sai_so)
    db = -2 / n * np.sum(sai_so)
    return dw, db

def so_buoc_toi_muc(x, lr, muc):
    w, b = 0.0, 0.0
    buoc = 0
    while mat_mat(w, b, x) >= muc and buoc < 200000:
        dw, db = gradient(w, b, x)
        w -= lr * dw
        b -= lr * db
        buoc += 1
    return buoc, mat_mat(w, b, x)

buoc_chuan, loss_chuan = so_buoc_toi_muc(x_chuan, 0.1, 400)
buoc_goc, loss_goc = so_buoc_toi_muc(dien_tich, 0.0001, 400)

print("co chuan hoa (lr=0.1):", buoc_chuan, "buoc, loss cuoi", round(loss_chuan, 2))
print("khong chuan hoa (lr=0.0001, lr lon nhat KHONG phan ky):", buoc_goc, "buoc, loss cuoi", round(loss_goc, 2))
```

```text title=readonly
trung binh: 60.0  do lech chuan: 20.0
x_chuan: [-1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5]
co chuan hoa (lr=0.1): 25 buoc, loss cuoi 393.33
khong chuan hoa (lr=0.0001, lr lon nhat KHONG phan ky): 112719 buoc, loss cuoi 400.0
```

Cùng đích, cùng dữ liệu — chỉ khác việc `x` có chuẩn hoá hay không. Có
chuẩn hoá: `25` bước là đủ, với `lr = 0.1` (một `lr` mà nếu áp trực tiếp
lên `dien_tich` gốc sẽ phân kỳ ngay lập tức — thang đo `x_chuan` nhỏ hơn
hẳn nên chịu được `lr` lớn hơn hẳn). Không chuẩn hoá: phải dùng `lr` nhỏ
hơn `1000` lần (`0.0001`, mức lớn nhất còn AN TOÀN đã tìm ở bài trước), và
cần tới `112 719` bước — gấp hơn `4500` lần. Bài toán TOÁN HỌC giống hệt
nhau; khác biệt duy nhất là thang đo của đầu vào.
::::

::::predict{#doan-tb-do-lech-cua-x-chuan commitOnce}
`x_chuan` được tính từ `dien_tich` bằng công thức `(x − trung_bình) /
độ_lệch_chuẩn`.

**Trước khi tính**, bạn đoán: trung bình và độ lệch chuẩn của CHÍNH
`x_chuan` (không phải của `dien_tich` gốc) là bao nhiêu?

:::opt{correct}
Trung bình đúng bằng `0`, độ lệch chuẩn đúng bằng `1` — luôn luôn đúng như
vậy với BẤT KỲ dữ liệu nào đưa qua công thức chuẩn hoá, không riêng gì
diện tích
:::

:::opt
Trung bình vẫn là `60`, độ lệch chuẩn vẫn là `20` — chuẩn hoá chỉ đổi TÊN
biến, không đổi con số bên trong
::why
Gần đúng ở việc bạn nhớ đúng các con số GỐC (`60` và `20`) của `dien_tich`
— hai con số đó không sai.

Chỗ lệch: chuẩn hoá KHÔNG phải đổi tên suông — nó thật sự TRỪ đi `60` khỏi
mọi giá trị (đưa trung bình về `0`) rồi CHIA cho `20` (đưa độ lệch chuẩn về
`1`). Sau hai phép toán đó, thang đo của dữ liệu đã đổi hẳn — `x_chuan`
mang trung bình `0` và độ lệch chuẩn `1`, không còn giữ `60` và `20` của
`dien_tich` gốc nữa.
::
:::

:::opt
Trung bình đúng bằng `0`, nhưng độ lệch chuẩn vẫn còn phụ thuộc đơn vị đo
gốc, không cố định
::why
Gần đúng ở nửa đầu: trừ đi trung bình đúng là đưa trung bình về `0` — phần
đó chính xác.

Chỗ lệch nằm ở nửa sau: chia cho ĐÚNG độ lệch chuẩn của chính dữ liệu đó
(không phải một hằng số cố định nào khác) luôn đưa độ lệch chuẩn mới về
đúng `1`, bất kể đơn vị gốc là gì. Đó chính là lý do phép chuẩn hoá này
"gột sạch" đơn vị đo — dù `x` gốc đo bằng m2, mm2, hay bất kỳ đơn vị nào,
`x_chuan` sau chuẩn hoá luôn có cùng một thang đo: trung bình `0`, độ lệch
chuẩn `1`.
::
:::
::::

::::code{#chuan-hoa-va-so-sanh-hoi-tu}
Viết hàm `chuan_hoa` theo công thức `(x − trung_bình) / độ_lệch_chuẩn`, rồi
chạy `huan_luyen` (gradient descent có sẵn) 25 bước trên dữ liệu ĐÃ chuẩn
hoá (`lr=0.1`) và trên dữ liệu GỐC (`lr=0.0001`), so sánh loss.

```python title=starter
import numpy as np

dien_tich = np.array([30, 40, 50, 60, 70, 80, 90], dtype=float)
gia = np.array([780, 1010, 1170, 1425, 1585, 1820, 1990], dtype=float)
n = len(dien_tich)

def chuan_hoa(x):
    trung_binh = x.mean()
    do_lech_chuan = x.std()
    return ___, trung_binh, do_lech_chuan     # (x - trung_binh) / do_lech_chuan

x_chuan, tb, sd = chuan_hoa(dien_tich)

def mat_mat(w, b, x):
    du_doan = w * x + b
    return np.mean((gia - du_doan) ** 2)

def gradient(w, b, x):
    du_doan = w * x + b
    sai_so = gia - du_doan
    dw = -2 / n * np.sum(x * sai_so)
    db = -2 / n * np.sum(sai_so)
    return dw, db

def huan_luyen(x, lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b, x)
        w = w - lr * dw
        b = b - lr * db
    return w, b

w_chuan, b_chuan = huan_luyen(___, 0.1, 25)      # dung x_chuan
w_goc, b_goc = huan_luyen(___, 0.0001, 25)        # dung dien_tich goc

print(round(mat_mat(w_chuan, b_chuan, x_chuan), 2))
print(round(mat_mat(w_goc, b_goc, dien_tich), 2))
```

```python title=solution
import numpy as np

dien_tich = np.array([30, 40, 50, 60, 70, 80, 90], dtype=float)
gia = np.array([780, 1010, 1170, 1425, 1585, 1820, 1990], dtype=float)
n = len(dien_tich)

def chuan_hoa(x):
    trung_binh = x.mean()
    do_lech_chuan = x.std()
    return (x - trung_binh) / do_lech_chuan, trung_binh, do_lech_chuan

x_chuan, tb, sd = chuan_hoa(dien_tich)

def mat_mat(w, b, x):
    du_doan = w * x + b
    return np.mean((gia - du_doan) ** 2)

def gradient(w, b, x):
    du_doan = w * x + b
    sai_so = gia - du_doan
    dw = -2 / n * np.sum(x * sai_so)
    db = -2 / n * np.sum(sai_so)
    return dw, db

def huan_luyen(x, lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b, x)
        w = w - lr * dw
        b = b - lr * db
    return w, b

w_chuan, b_chuan = huan_luyen(x_chuan, 0.1, 25)
w_goc, b_goc = huan_luyen(dien_tich, 0.0001, 25)

print(round(mat_mat(w_chuan, b_chuan, x_chuan), 2))
print(round(mat_mat(w_goc, b_goc, dien_tich), 2))
```

```python title=test
assert round(tb, 4) == 60.0 and round(sd, 4) == 20.0, "trung binh/do lech chuan cua dien_tich phai la 60.0/20.0"
assert round(x_chuan.mean(), 6) == 0.0, "x_chuan phai co trung binh dung bang 0"
assert round(x_chuan.std(), 6) == 1.0, "x_chuan phai co do lech chuan dung bang 1"
assert round(mat_mat(w_chuan, b_chuan, x_chuan), 2) == 393.33, f"loss tren du lieu da chuan hoa sau 25 buoc phai la 393.33 -- dang ra {round(mat_mat(w_chuan, b_chuan, x_chuan), 2)}"
assert round(mat_mat(w_goc, b_goc, dien_tich), 2) == 3703.93, f"loss tren du lieu goc sau 25 buoc phai la 3703.93 -- dang ra {round(mat_mat(w_goc, b_goc, dien_tich), 2)}"
assert mat_mat(w_chuan, b_chuan, x_chuan) < mat_mat(w_goc, b_goc, dien_tich), "sau CUNG 25 buoc, du lieu da chuan hoa phai cho loss THAP HON han du lieu goc"
```

:::hints
- kind: attention
  body: Chỗ trống đầu nằm trong `chuan_hoa` — công thức là `(x - trung_binh) / do_lech_chuan`, dùng đúng hai biến cục bộ vừa tính ở hai dòng trên nó. Hai chỗ trống dưới chỉ là chọn ĐÚNG dữ liệu để truyền vào `huan_luyen`: bản đã chuẩn hoá cho lệnh gọi dùng `lr=0.1`, bản gốc cho lệnh gọi dùng `lr=0.0001`.
- kind: strategy
  body: 'Chỗ trống 1: `(x - trung_binh) / do_lech_chuan`. Chỗ trống 2: `x_chuan`. Chỗ trống 3: `dien_tich`.'
- kind: one-line
  body: 'Ba chỗ trống: `(x - trung_binh) / do_lech_chuan`, `x_chuan`, `dien_tich`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chuan_hoa phai THAT SU tru trung_binh roi chia do_lech_chuan (khong duoc tra ve x nguyen si), va huan_luyen phai duoc goi voi dung du lieu (x_chuan cho lr lon, dien_tich cho lr nho)
  requireAst:
  - kind: uses-name, target: trung_binh, min: 2
  - kind: uses-name, target: do_lech_chuan, min: 2
  - kind: uses-name, target: x_chuan, min: 2
  - kind: uses-name, target: dien_tich, min: 4
  - kind: uses-operator, target: "/", min: 3
  # Da thu that (ast.parse): dien "x" (khong chuan hoa) vao cho trong 1 cho
  # [trung_binh=1, do_lech_chuan=1, x_chuan=2, dien_tich=4, "/"=2] -- tut
  # duoi nguong trung_binh/do_lech_chuan/"/", bi chan. Loi giai that cho
  # dung [2,2,2,4,3], qua sach ca nam nguong.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^393\\.33\\n3703\\.93\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
25 bước so với hơn một trăm nghìn bước — cùng bài toán, chỉ khác thang đo.
Chuẩn hoá không đổi đáp án tối ưu, chỉ đổi con đường tới đó nhanh cỡ nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài `dac-trung-da-thuc` đã thêm `x²` làm đặc trưng thứ hai. Nếu `x` chạy từ
`1` tới `7`, thì `x²` chạy từ `1` tới `49` — hai đặc trưng đó có thang đo
KHÁC NHAU ngay trong CÙNG một mô hình, không cần đợi đổi đơn vị đo gì cả.

Chuẩn hoá — bài này vừa học — có cần thiết hơn NỮA khi một mô hình dùng
nhiều đặc trưng có thang đo tự nhiên khác xa nhau như vậy không? Bài
`regularization-l1-l2` sẽ chạm lại đúng câu hỏi này, từ một góc nhìn khác:
phạt trọng số lớn có công bằng không, nếu các đặc trưng còn chưa cùng thang
đo?
::::

::::checkpoint{mastery=0.8}
::::
