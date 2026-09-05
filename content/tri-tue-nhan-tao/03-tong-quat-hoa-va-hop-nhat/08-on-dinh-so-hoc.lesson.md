---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.on-dinh-so-hoc
title: "Ổn định số học: log(0) và tràn số"
summary: "Hai lỗi số học kinh điển, đo bằng số cụ thể. (a) Cross-entropy không có epsilon bảo vệ: p=0.0 cho log(0)=-inf, cross-entropy=inf (vỡ hoàn toàn); có epsilon (1e-12) cho 13.8682 — vẫn cao (mô hình sai chỗ đó) nhưng KHÔNG vỡ. (b) Softmax không trừ max trước exp: z=[1000,1001,999] cho exp=[inf,inf,inf], softmax=[nan,nan,nan]; trừ max trước cho đúng [0.244728, 0.665241, 0.090031] — CÙNG một kết quả toán học, chỉ khác cách tính có tràn số hay không."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.on-dinh-so-hoc]
requires: [ai.xu-ly-lech-lop]
concepts: [ai.on-dinh-so-hoc]
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
`softmax-da-lop` (track trước) đã LẶNG LẼ trừ giá trị lớn nhất trước khi
lấy mũ, không giải thích kỹ vì sao. Bài này mổ xẻ đúng chỗ đó — và một lỗi
song sinh của nó ở cross-entropy.
::::

::::explain{#hai-loi-so-hoc-kinh-dien}
Máy tính lưu số thực bằng một số LƯỢNG BIT CỐ ĐỊNH (`float64` — độ chính
xác kép). Hai phép toán quen thuộc của học máy — `log` và `exp` — đều có
thể "vỡ" trên những giá trị hoàn toàn hợp lệ về mặt TOÁN HỌC, chỉ vì giới
hạn biểu diễn số của máy tính.

**Lỗi thứ nhất — `log(0)`.** Cross-entropy (`binary-cross-entropy`,
`logistic-regression-tu-so-0`) có công thức `−[y·log(p) + (1−y)·log(1−p)]`.
Nếu mô hình dự đoán xác suất TUYỆT ĐỐI `p = 0` cho một điểm có nhãn thật
`y = 1`, biểu thức có `log(0)` — về mặt toán học, giới hạn của `log(p)` khi
`p → 0` là `−∞`, và `float64` biểu diễn đúng điều đó bằng giá trị đặc biệt
`-inf`. Từ đó, TOÀN BỘ cross-entropy (trung bình cộng có chứa một số hạng
`-inf`) trở thành `inf` — một con số VÔ DỤNG, không đo lường được gì thêm,
dù chỉ MỘT điểm dữ liệu gây ra nó.

Cách sửa chuẩn: **epsilon clipping** — kẹp xác suất dự đoán vào một khoảng
AN TOÀN, hơi lùi vào bên trong `(0, 1)`, trước khi đưa vào `log`:

> `p_an_toan = clip(p, ε, 1 − ε)`

với `ε` (epsilon) là một số CỰC NHỎ (ví dụ `1e-12`). Phép kẹp này gần như
không đổi GIÁ TRỊ Ý NGHĨA của `p` (`1e-12` quá nhỏ để ảnh hưởng thực chất
tới việc mô hình "đúng" hay "sai"), nhưng đảm bảo `log` không bao giờ nhận
đúng `0` hay đúng `1` làm đầu vào.

**Lỗi thứ hai — tràn số trong softmax.** `softmax(z)ₖ = eᶻᵏ / Σⱼ eᶻʲ`. Nếu
`z` chứa một giá trị LỚN (ví dụ `1000` — hoàn toàn có thể xảy ra nếu trọng
số mô hình chưa được chuẩn hoá tốt, hay dữ liệu đầu vào có thang đo lớn),
`e^1000` VƯỢT QUÁ số lớn nhất mà `float64` biểu diễn được — kết quả là
`inf`. Và `inf / inf` không phải một phép chia có kết quả xác định — nó là
`nan` (not a number), một giá trị "không tính được", lan ra MỌI lớp, không
chỉ lớp có `z` lớn.

Cách sửa: trừ giá trị LỚN NHẤT của `z` khỏi MỌI phần tử trước khi lấy mũ:

> `softmax(z)ₖ = e^(zₖ − max(z)) / Σⱼ e^(zⱼ − max(z))`

Về mặt ĐẠI SỐ, đây là phép nhân cả tử số lẫn mẫu số với CÙNG một hằng số
`e^(−max(z))` — không đổi GIÁ TRỊ của kết quả cuối cùng. Nhưng sau khi trừ,
giá trị LỚN NHẤT trong `z` luôn là đúng `0` — và `e⁰ = 1`, một con số hoàn
toàn AN TOÀN, không tràn số nào cả.
::::

::::example{#do-hai-loi-bang-so-that}
**(a) log(0) trong cross-entropy** — bốn điểm, một điểm có `p = 0.0` dự
đoán tuyệt đối SAI cho nhãn thật `y = 1`:

```python title=readonly
import numpy as np

y_that = np.array([1, 0, 1, 0])
p_du_doan = np.array([0.0, 1.0, 0.9, 0.1])

with np.errstate(divide='ignore'):
    ce_khong_eps = -np.mean(y_that*np.log(p_du_doan) + (1-y_that)*np.log(1-p_du_doan))
print("cross-entropy KHONG epsilon:", ce_khong_eps)

EPS = 1e-12
p_an_toan = np.clip(p_du_doan, EPS, 1 - EPS)
ce_co_eps = -np.mean(y_that*np.log(p_an_toan) + (1-y_that)*np.log(1-p_an_toan))
print("cross-entropy CO epsilon:", round(ce_co_eps, 4))
```

```text title=readonly
cross-entropy KHONG epsilon: inf
cross-entropy CO epsilon: 13.8682
```

Không epsilon: `inf` — cross-entropy VỠ HOÀN TOÀN, không còn phân biệt được
"mô hình này tệ một chút" với "mô hình này tệ RẤT nhiều" — mọi trường hợp
đều báo `inf` như nhau. Có epsilon: `13.8682` — vẫn là một con số RẤT CAO
(đúng bản chất: mô hình sai chỗ đó thật sự tệ), nhưng là một con số HỮU
HẠN, có thể so sánh, có thể lấy gradient, có thể dùng để huấn luyện tiếp.

**(b) Tràn số trong softmax** — ba giá trị logit LỚN, chỉ chênh nhau vài
đơn vị:

```python title=readonly
import numpy as np

z_lon = np.array([1000.0, 1001.0, 999.0])

with np.errstate(over='ignore', invalid='ignore'):
    e_khong_tru_max = np.exp(z_lon)
    softmax_khong_tru_max = e_khong_tru_max / np.sum(e_khong_tru_max)
print("exp KHONG tru max:", e_khong_tru_max.tolist())
print("softmax KHONG tru max:", softmax_khong_tru_max.tolist())

z_sau_tru_max = z_lon - z_lon.max()
e_co_tru_max = np.exp(z_sau_tru_max)
softmax_co_tru_max = e_co_tru_max / np.sum(e_co_tru_max)
print("z SAU KHI tru max:", z_sau_tru_max.tolist())
print("softmax CO tru max:", np.round(softmax_co_tru_max, 6).tolist())
```

```text title=readonly
exp KHONG tru max: [inf, inf, inf]
softmax KHONG tru max: [nan, nan, nan]
z SAU KHI tru max: [-1.0, 0.0, -2.0]
softmax CO tru max: [0.244728, 0.665241, 0.090031]
```

`z = 1000` (và các giá trị lân cận) khiến `exp` tràn số thành `inf` ở CẢ BA
phần tử — rồi `inf / inf` cho `nan` ở CẢ BA, dù ba giá trị logit gốc khác
nhau rõ ràng (`999`, `1000`, `1001` — logit lớn nhất PHẢI có xác suất cao
nhất). Trừ giá trị lớn nhất trước: `z` mới chỉ còn `[-1, 0, -2]` — nhỏ, an
toàn — và softmax ra đúng `[0.244728, 0.665241, 0.090031]`, ĐÚNG thứ tự
mong đợi (logit `1001` — lớn nhất — có xác suất `0.665241`, cao nhất trong
ba). Hai cách tính CÙNG một công thức toán học, chỉ khác việc có tràn số
hay không.
::::

::::predict{#doan-neu-z-nho commitOnce}
Vẫn công thức softmax trên. Giả sử `z = [1.0, 2.0, 0.5]` — những giá trị
NHỎ, hoàn toàn bình thường, không có nguy cơ tràn số nào.

**Trước khi tính**, bạn đoán: áp dụng bước "trừ giá trị lớn nhất trước khi
lấy mũ" lên `z` NHỎ này có làm kết quả softmax cuối cùng thay đổi so với
KHÔNG trừ gì cả không?

:::opt{correct}
Không — trừ giá trị lớn nhất là một phép biến đổi ĐẠI SỐ không đổi giá trị
softmax cuối cùng, dù `z` lớn hay nhỏ; nó chỉ là một biện pháp AN TOÀN, cần
thiết cụ thể khi `z` đủ lớn để gây tràn số, không phải một công thức khác
cho `z` nhỏ
:::

:::opt
Có — với `z` nhỏ, phép trừ vẫn làm thay đổi kết quả một chút, chỉ là ít
nghiêm trọng hơn trường hợp `z` lớn
::why
Gần đúng ở việc bạn để ý bài này có bàn tới ĐỘ LỚN của `z` như một yếu tố
quan trọng — quan sát đó không sai về ngữ cảnh chung.

Chỗ lệch: phép trừ giá trị lớn nhất là một CHIA TỬ SỐ VÀ MẪU SỐ cho cùng
một hằng số `e^(max(z))` — về mặt ĐẠI SỐ THUẦN TUÝ, phép biến đổi này luôn
cho ra ĐÚNG cùng kết quả cuối cùng, bất kể `z` lớn hay nhỏ, không có "ít hay
nhiều" ở đây. Khác biệt DUY NHẤT là: với `z` nhỏ, phép tính TRỰC TIẾP (không
trừ) cũng đã đủ an toàn rồi — nên khác biệt giữa hai cách tính chỉ lộ ra khi
`z` đủ lớn để một trong hai cách bắt đầu tràn số.
::
:::

:::opt
Không xác định được nếu không biết chính xác công thức exp của numpy hoạt
động ra sao ở mức bit
::why
Gần đúng ở việc bạn cẩn trọng về CHI TIẾT triển khai của `np.exp` — một
thái độ hợp lý khi làm việc với số học dấu phẩy động nói chung.

Chỗ lệch: câu hỏi không đòi hỏi biết chi tiết CÁCH `np.exp` cài đặt bên
trong — nó chỉ cần dùng đúng CÔNG THỨC ĐẠI SỐ đã học (trừ rồi cộng lại đúng
hằng số vào cả tử và mẫu không đổi tỉ lệ giữa chúng). Với `z` nhỏ, không hề
có rủi ro tràn số ở BẤT KỲ cách tính nào trong hai cách, nên kết quả HOÀN
TOÀN xác định và bằng nhau, không cần biết thêm chi tiết bit nào.
::
:::
::::

::::code{#sua_hai_loi_so_hoc}
Hoàn thiện `p_an_toan` (kẹp xác suất vào khoảng an toàn bằng `np.clip`) và
`z_sau_tru_max` (trừ giá trị lớn nhất của `z_lon` khỏi chính nó).

```python title=starter
import numpy as np

y_that = np.array([1, 0, 1, 0])
p_du_doan = np.array([0.0, 1.0, 0.9, 0.1])

with np.errstate(divide='ignore'):
    ce_khong_eps = -np.mean(y_that*np.log(p_du_doan) + (1-y_that)*np.log(1-p_du_doan))

EPS = 1e-12
p_an_toan = ___                          # np.clip(p_du_doan, EPS, 1 - EPS)
ce_co_eps = -np.mean(y_that*np.log(p_an_toan) + (1-y_that)*np.log(1-p_an_toan))

z_lon = np.array([1000.0, 1001.0, 999.0])
with np.errstate(over='ignore', invalid='ignore'):
    e_khong_tru_max = np.exp(z_lon)
    softmax_khong_tru_max = e_khong_tru_max / np.sum(e_khong_tru_max)

z_sau_tru_max = ___                      # z_lon - z_lon.max()
e_co_tru_max = np.exp(z_sau_tru_max)
softmax_co_tru_max = e_co_tru_max / np.sum(e_co_tru_max)

print(ce_khong_eps)
print(round(ce_co_eps, 4))
print(softmax_khong_tru_max.tolist())
print(np.round(softmax_co_tru_max, 6).tolist())
```

```python title=solution
import numpy as np

y_that = np.array([1, 0, 1, 0])
p_du_doan = np.array([0.0, 1.0, 0.9, 0.1])

with np.errstate(divide='ignore'):
    ce_khong_eps = -np.mean(y_that*np.log(p_du_doan) + (1-y_that)*np.log(1-p_du_doan))

EPS = 1e-12
p_an_toan = np.clip(p_du_doan, EPS, 1 - EPS)
ce_co_eps = -np.mean(y_that*np.log(p_an_toan) + (1-y_that)*np.log(1-p_an_toan))

z_lon = np.array([1000.0, 1001.0, 999.0])
with np.errstate(over='ignore', invalid='ignore'):
    e_khong_tru_max = np.exp(z_lon)
    softmax_khong_tru_max = e_khong_tru_max / np.sum(e_khong_tru_max)

z_sau_tru_max = z_lon - z_lon.max()
e_co_tru_max = np.exp(z_sau_tru_max)
softmax_co_tru_max = e_co_tru_max / np.sum(e_co_tru_max)

print(ce_khong_eps)
print(round(ce_co_eps, 4))
print(softmax_khong_tru_max.tolist())
print(np.round(softmax_co_tru_max, 6).tolist())
```

```python title=test
assert np.isinf(ce_khong_eps), f"cross-entropy KHONG epsilon phai la inf -- dang ra {ce_khong_eps}"
assert round(ce_co_eps, 4) == 13.8682, f"cross-entropy CO epsilon phai la 13.8682 -- dang ra {round(ce_co_eps, 4)}"
assert np.isfinite(ce_co_eps), "cross-entropy CO epsilon phai la mot so HUU HAN, khong duoc la inf hay nan"
assert np.all(np.isnan(softmax_khong_tru_max)), "softmax KHONG tru max (voi z=1000) phai la nan o CA BA phan tu"
assert z_sau_tru_max.max() == 0.0, f"gia tri LON NHAT cua z_sau_tru_max phai dung bang 0 -- dang ra {z_sau_tru_max.max()}"
assert np.round(softmax_co_tru_max, 6).tolist() == [0.244728, 0.665241, 0.090031], f"softmax CO tru max sai -- dang ra {np.round(softmax_co_tru_max, 6).tolist()}"
assert np.isclose(np.sum(softmax_co_tru_max), 1.0), "tong softmax CO tru max phai dung bang 1.0"
assert np.argmax(softmax_co_tru_max) == 1, "logit lon nhat (z_lon[1]=1001) phai co xac suat softmax LON NHAT"
```

:::hints
- kind: attention
  body: Hai chỗ trống. `p_an_toan` kẹp `p_du_doan` vào khoảng `[EPS, 1 - EPS]` bằng `np.clip(mang, min, max)`. `z_sau_tru_max` trừ giá trị LỚN NHẤT của `z_lon` (`z_lon.max()`) khỏi chính `z_lon` — một phép trừ áp dụng đồng loạt lên MỌI phần tử.
- kind: strategy
  body: 'p_an_toan: `np.clip(p_du_doan, EPS, 1 - EPS)`. z_sau_tru_max: `z_lon - z_lon.max()`.'
- kind: one-line
  body: 'Hai chỗ trống: `np.clip(p_du_doan, EPS, 1 - EPS)` và `z_lon - z_lon.max()`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: p_an_toan phai goi THAT np.clip tren p_du_doan voi EPS (khong duoc chep lai p_du_doan nguyen si, se van log(0)); z_sau_tru_max phai THAT SU tru z_lon.max() khoi z_lon (khong duoc chep lai z_lon nguyen si, se van tran so)
  requireAst:
  - kind: uses-call, target: clip, min: 1
  - kind: uses-name, target: EPS, min: 2
  - kind: uses-name, target: z_lon, min: 3
  - kind: uses-call, target: max, min: 1
  # Da thu that (goi kiemAst that tren code day du): loi giai dung dat=true,
  # ca bon luat qua sach. Cheat p_an_toan=p_du_doan (khong clip) VA
  # z_sau_tru_max=z_lon (khong tru gi) lam CA BON luat cung roi xuong duoi
  # nguong -- bi chan; chay THAT cheat nay xac nhan ce_co_eps van la inf va
  # softmax van la [nan,nan,nan], dung dieu tier tests se bat qua
  # np.isfinite/np.isnan.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^inf\\n13\\.8682\\n\\[nan, nan, nan\\]\\n\\[0\\.244728, 0\\.665241, 0\\.090031\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`inf` và `nan` không phải lỗi hiếm gặp trong lý thuyết — chúng chờ sẵn ở
đúng những giá trị hoàn toàn hợp lý về mặt toán học. Hai lá chắn nhỏ —
epsilon và trừ max — đủ để tránh cả hai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cả hai lỗi vừa sửa đều nằm ở TẦNG CÔNG THỨC — chỗ nào trong phép tính có
thể tràn số hay chia cho một giá trị cực nhỏ. Nhưng có một câu hỏi khác,
rộng hơn: khi một mô hình đã fit xong, làm sao biết ĐẶC TRƯNG nào thật sự
đang QUYẾT ĐỊNH dự đoán của nó — nhìn vào hệ số thô có đủ tin cậy không,
nếu các đặc trưng có thang đo khác hẳn nhau?

Bài sau trả lời — quay lại đúng câu hỏi mà `chuan-hoa-dac-trung` để lại từ
đầu track hồi quy.
::::

::::checkpoint{mastery=0.8}
::::
