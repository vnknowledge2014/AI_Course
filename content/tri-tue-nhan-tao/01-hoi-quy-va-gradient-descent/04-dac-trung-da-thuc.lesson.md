---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.dac-trung-da-thuc
title: "Đặc trưng đa thức"
summary: "Bảy điểm chi phí quảng cáo (1-7 triệu đồng) và đơn hàng thu được có dạng cong (lợi ích giảm dần): hồi quy tuyến tính trên x thuần cho MSE=4.7216; mở rộng đặc trưng thành [x, x^2] rồi VẪN dùng hồi quy tuyến tính (trên đặc trưng mở rộng) cho MSE=0.3154 — giảm gần 15 lần — vì mô hình tuyến tính theo THAM SỐ (w1, w2, b), không phải theo dữ liệu gốc x."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.dac-trung-da-thuc]
requires: [ai.gradient-descent]
concepts: [ai.dac-trung-da-thuc]
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
Dữ liệu cong hẳn, không hề thẳng. "Hồi quy TUYẾN TÍNH" có còn dùng được
không?
::::

::::explain{#tuyen-tinh-theo-tham-so}
Byte đo chi phí quảng cáo (triệu đồng) và số đơn hàng thu về trong bảy
tuần. Chấm bảy điểm lên giấy: chúng không nằm gần một đường thẳng nào cả —
đơn hàng tăng nhanh lúc đầu, rồi tăng CHẬM DẦN khi chi phí đã lớn (lợi ích
giảm dần — một đồng quảng cáo cuối cùng không hiệu quả bằng một đồng đầu
tiên). Đường cong này KHÔNG khớp được bằng `y = w·x + b`.

Cách xử lý không phải đổi mô hình — mà đổi ĐẦU VÀO của mô hình. Thay vì chỉ
đưa `x` vào, đưa cả `x` LẪN `x²` vào, coi chúng là HAI đặc trưng riêng biệt:

> `y = w₁·x + w₂·x² + b`

Nhìn kỹ công thức này: nó vẫn là một TỔNG có TRỌNG SỐ của các đặc trưng,
cộng thêm một hằng số — đúng hình dạng mà least-squares và gradient descent
của ba bài trước đã biết cách khớp. Cái mới chỉ là: một trong các đặc trưng
đó (`x²`) được TÍNH TRƯỚC từ `x`, chứ không phải chính `x`. Với TỪNG cặp
`(w₁, w₂)`, hàm số theo `x` có thể cong — nhưng hàm số theo `(w₁, w₂, b)`
vẫn phẳng, tuyến tính hoàn toàn. Đây là điểm mấu chốt hay bị hiểu lầm: hồi
quy "tuyến tính" nói về việc mô hình tuyến tính THEO THAM SỐ (`w₁`, `w₂`,
`b` chỉ nhân rồi cộng, không lồng vào hàm phi tuyến nào), KHÔNG bắt buộc
tuyến tính theo dữ liệu gốc (`x`).

Cách thêm `x²`, `x³`, ... vào làm đặc trưng mới gọi là **đặc trưng đa thức**
(polynomial features). Khớp được đường cong bằng cách nới rộng ĐẦU VÀO, giữ
nguyên CÁCH khớp.
::::

::::example{#khop-duong-cong-quang-cao}
Bảy điểm chi phí quảng cáo và đơn hàng — khớp bằng hồi quy bậc 1 (chỉ `x`)
rồi bậc 2 (`x` và `x²`), đối chiếu sai số:

```python title=readonly
import numpy as np

chi_phi = np.array([1, 2, 3, 4, 5, 6, 7], dtype=float)
don_hang = np.array([13.8, 20.0, 27.5, 31.1, 35.5, 36.6, 38.7])
n = len(chi_phi)

# bac 1: chi dung x
A1 = np.vstack([chi_phi, np.ones(n)]).T
he_so_1, *_ = np.linalg.lstsq(A1, don_hang, rcond=None)
mse_1 = np.mean((don_hang - A1 @ he_so_1) ** 2)

# bac 2: dung ca x va x^2
A2 = np.vstack([chi_phi, chi_phi ** 2, np.ones(n)]).T
he_so_2, *_ = np.linalg.lstsq(A2, don_hang, rcond=None)
mse_2 = np.mean((don_hang - A2 @ he_so_2) ** 2)

print("bac 1: he_so =", np.round(he_so_1, 4).tolist(), " mse =", round(mse_1, 4))
print("bac 2: he_so =", np.round(he_so_2, 4).tolist(), " mse =", round(mse_2, 4))
```

```text title=readonly
bac 1: he_so = [4.1393, 12.4714]  mse = 4.7216
bac 2: he_so = [8.9869, -0.606, 5.2]  mse = 0.3154
```

Bậc 1 chỉ có một độ dốc CỐ ĐỊNH (`4.1393`), không đổi dù `x` là `1` hay
`7` — không thể theo kịp một đường cong lợi ích giảm dần. Bậc 2 giảm MSE
xuống còn `0.3154` — gần 15 lần thấp hơn. Nhìn hệ số bậc 2: hệ số của `x`
là `8.9869` (dương — mỗi đồng quảng cáo ban đầu vẫn kéo thêm đơn hàng), hệ
số của `x²` là `−0.606` (ÂM — càng chi nhiều, phần LỢI thêm càng bị bớt đi,
đúng hình dạng lợi ích giảm dần). Cả hai mô hình đều được khớp bằng
`np.linalg.lstsq` — CÙNG một thuật toán, chỉ khác số cột đưa vào.
::::

::::predict{#doan-y-nghia-he-so-am commitOnce}
Mô hình bậc 2 vừa khớp: `y = 8.9869·x − 0.606·x² + 5.2`. Hệ số của `x²` là
số ÂM.

**Trước khi đọc tiếp**, bạn đoán: nếu tiếp tục tăng `x` (chi quảng cáo)
LÊN RẤT CAO, ngoài xa phạm vi 7 điểm dữ liệu đã đo, mô hình bậc 2 này dự
đoán đơn hàng sẽ:

:::opt{correct}
Tăng dần tới một đỉnh rồi QUAY ĐẦU GIẢM XUỐNG — vì hệ số `x²` âm, số hạng
`−0.606·x²` áp đảo dần số hạng `8.9869·x` khi `x` đủ lớn
:::

:::opt
Tăng chậm dần rồi tiến gần một mức TRẦN cố định, không bao giờ vượt qua —
đúng hình dạng "lợi ích giảm dần" thường thấy trong thực tế
::why
Gần đúng ở việc bạn nắm đúng Ý TƯỞNG "lợi ích giảm dần" — quan sát trên
bảy điểm dữ liệu THẬT SỰ trông giống một đường tiệm cận một mức trần.

Chỗ lệch: đó là hình dạng của DỮ LIỆU trong phạm vi đã đo, không phải hình
dạng của CÔNG THỨC đã khớp. `y = 8.9869x − 0.606x²` là một parabol quay
xuống (vì hệ số `x²` âm) — nó không hề có một mức trần nào để tiệm cận tới.
Ngoài phạm vi dữ liệu đã thấy, một đa thức bậc 2 LUÔN quay đầu giảm, không
bao giờ giữ mức phẳng mãi mãi.
::
:::

:::opt
Tiếp tục tăng mãi, chỉ là tăng chậm hơn — vì đơn hàng trong thực tế không
thể "giảm" khi chi tiêu quảng cáo tăng thêm
::why
Gần đúng ở trực giác đời thường: chi thêm tiền quảng cáo hiếm khi làm đơn
hàng THẬT SỰ giảm — quan sát đó hợp lý ngoài đời.

Chỗ lệch: câu hỏi hỏi về mô hình TOÁN HỌC đã khớp, không hỏi về thực tế
ngoài đời. Mô hình chỉ "biết" những gì bảy điểm dữ liệu trong phạm vi
`x = 1` tới `x = 7` cho nó thấy — nó không có cơ chế nào tự nhận ra "đừng
giảm" khi ngoại suy ra ngoài phạm vi đó. Với công thức `8.9869x − 0.606x²`
cụ thể này, phần `−0.606x²` chắc chắn áp đảo khi `x` đủ lớn, kéo `y` xuống
— đây chính là rủi ro của việc NGOẠI SUY (extrapolate) một đa thức ra ngoài
vùng dữ liệu đã học.
::
:::
::::

::::code{#khop-dac-trung-da-thuc}
Mở rộng đặc trưng thành `[chi_phi, chi_phi²]`, khớp bằng `np.linalg.lstsq`,
rồi so sánh MSE với mô hình bậc 1 chỉ dùng `chi_phi`.

```python title=starter
import numpy as np

chi_phi = np.array([1, 2, 3, 4, 5, 6, 7], dtype=float)
don_hang = np.array([13.8, 20.0, 27.5, 31.1, 35.5, 36.6, 38.7])
n = len(chi_phi)

A1 = np.vstack([chi_phi, np.ones(n)]).T
he_so_1, resid1, rank1, sv1 = np.linalg.lstsq(A1, don_hang, rcond=None)
du_doan_1 = A1 @ he_so_1
mse_1 = np.mean((don_hang - du_doan_1) ** 2)

A2 = np.vstack([___, ___, np.ones(n)]).T     # chi_phi va chi_phi binh phuong
he_so_2, resid2, rank2, sv2 = np.linalg.lstsq(A2, don_hang, rcond=None)
du_doan_2 = A2 @ he_so_2
mse_2 = ___                                   # trung binh binh phuong sai so cua bac 2

print(round(mse_1, 4))
print(round(mse_2, 4))
```

```python title=solution
import numpy as np

chi_phi = np.array([1, 2, 3, 4, 5, 6, 7], dtype=float)
don_hang = np.array([13.8, 20.0, 27.5, 31.1, 35.5, 36.6, 38.7])
n = len(chi_phi)

A1 = np.vstack([chi_phi, np.ones(n)]).T
he_so_1, resid1, rank1, sv1 = np.linalg.lstsq(A1, don_hang, rcond=None)
du_doan_1 = A1 @ he_so_1
mse_1 = np.mean((don_hang - du_doan_1) ** 2)

A2 = np.vstack([chi_phi, chi_phi ** 2, np.ones(n)]).T
he_so_2, resid2, rank2, sv2 = np.linalg.lstsq(A2, don_hang, rcond=None)
du_doan_2 = A2 @ he_so_2
mse_2 = np.mean((don_hang - du_doan_2) ** 2)

print(round(mse_1, 4))
print(round(mse_2, 4))
```

```python title=test
assert round(mse_1, 4) == 4.7216, f"mse bac 1 phai la 4.7216 -- dang ra {round(mse_1, 4)}"
assert round(mse_2, 4) == 0.3154, f"mse bac 2 phai la 0.3154 -- dang ra {round(mse_2, 4)}"
assert mse_2 < mse_1, "them dac trung x^2 phai lam mse GIAM, khop tot hon"
assert A2.shape == (7, 3), f"A2 phai co 3 cot (x, x^2, hang so) -- dang ra {A2.shape}"
```

:::hints
- kind: attention
  body: Hai chỗ trống đầu là hai cột của ma trận `A2` — cột đầu là chính `chi_phi`, cột hai là `chi_phi` BÌNH PHƯƠNG (đặc trưng mới). Chỗ trống cuối là công thức MSE, giống hệt cách `mse_1` đã tính ở trên nhưng dùng `du_doan_2`.
- kind: strategy
  body: 'Hai cột của A2: `chi_phi` và `chi_phi ** 2`. mse_2: `np.mean((don_hang - du_doan_2) ** 2)`.'
- kind: one-line
  body: 'Ba chỗ trống: `chi_phi`, `chi_phi ** 2`, và `np.mean((don_hang - du_doan_2) ** 2)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: A2 phai co dung hai dac trung (chi_phi va chi_phi binh phuong), va mse_2 phai tinh THAT tu du_doan_2 -- khong duoc chep con so 0.3154 hay bo qua phep binh phuong o dac trung thu hai
  requireAst:
  - kind: uses-name, target: chi_phi, min: 4
  - kind: uses-operator, target: "**", min: 3
  - kind: uses-name, target: du_doan_2, min: 1
  - kind: uses-name, target: don_hang, min: 4
  # Da thu that (ast.parse): dien True/1/0 vao ca ba cho trong cho
  # [chi_phi=2, **=1, du_doan_2=0, don_hang=3] -- duoi ca bon nguong (can
  # 4/3/1/4). Quen binh phuong (dien chi_phi,chi_phi thay vi chi_phi,
  # chi_phi**2) cho [chi_phi=4, **=2, du_doan_2=1, don_hang=4] -- rieng **
  # tut con 2, duoi nguong 3, van bi chan. Loi giai that cho dung [4,3,1,4].
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^4\\.7216\\n0\\.3154\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một cách khớp, chỉ đổi đầu vào — MSE giảm gần 15 lần. Đường cong hoá
ra vẫn "tuyến tính", chỉ là tuyến tính theo một tập đặc trưng rộng hơn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bậc 2 đã khớp tốt hơn hẳn bậc 1. Vậy bậc 3, bậc 5, hay bậc 7 — thêm CÀNG
NHIỀU đặc trưng đa thức — có khớp CÀNG TỐT hơn nữa không? Và nếu MSE trên
chính bảy điểm đã dùng để khớp cứ giảm dần khi bậc tăng, điều đó có chắc
là một tin tốt?

Bài `chuan-hoa-dac-trung` sẽ ghé qua một vấn đề khác trước — nhưng câu hỏi
"thêm đặc trưng có luôn tốt không" sẽ quay lại, và câu trả lời không đơn
giản như con số MSE một mình có thể nói hết.
::::

::::checkpoint{mastery=0.8}
::::
