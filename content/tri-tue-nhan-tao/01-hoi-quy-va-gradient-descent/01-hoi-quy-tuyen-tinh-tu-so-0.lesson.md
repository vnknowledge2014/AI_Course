---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.hoi-quy-tuyen-tinh-tu-so-0
title: "Hồi quy tuyến tính từ số 0"
summary: "Khớp một đường thẳng y = w·x + b qua 7 điểm giá nhà tự bịa (diện tích 30-90 m2, giá 780-1990 triệu đồng) bằng công thức đóng least-squares tự viết tay (không np.polyfit) — ra w ≈ 20.2321, b ≈ 183.2143, sai số bình phương trung bình ≈ 363.14 — rồi đối chiếu với np.linalg.lstsq để xác nhận công thức tay đúng."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.hoi-quy-tuyen-tinh]
requires: []
concepts: [ai.hoi-quy-tuyen-tinh]
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
Bảy điểm rải rác trên giấy, không điểm nào thẳng hàng tuyệt đối. Vẫn có một
đường thẳng "khớp" chúng tốt nhất — và có một công thức tính RA đúng đường
đó, không cần dò thử.
::::

::::explain{#mo-hinh-va-sai-so}
Byte ghi lại bảy căn nhà: diện tích (m2) và giá bán (triệu đồng). Câu hỏi:
nếu biết diện tích một căn MỚI, chưa từng bán, giá của nó khoảng bao nhiêu?

Cách đơn giản nhất: giả định giá phụ thuộc diện tích theo một đường thẳng —

> `y = w·x + b`

`x` là diện tích, `y` là giá dự đoán, `w` là độ dốc (giá tăng thêm bao
nhiêu cho mỗi m2), `b` là hằng số cộng thêm (bài `cho-bat-dau-cua-duong-thang`
của track Toán đã đặt tên hai con số này — ở đây chúng có thêm một vai trò
mới: là thứ một MÔ HÌNH học được từ dữ liệu, không phải thứ đề bài cho sẵn).

Bảy điểm thật không nằm đúng trên một đường thẳng nào cả — luôn có sai lệch.
Với một cặp `(w, b)` bất kỳ, mỗi điểm dữ liệu có một **sai số**: giá THẬT trừ
giá mô hình DỰ ĐOÁN. Đường thẳng "khớp tốt" là đường có các sai số, xét
TỔNG THỂ, nhỏ nhất — và cách đo "nhỏ nhất" chuẩn của hồi quy tuyến tính là
**tổng bình phương các sai số** (bài sau sẽ giải thích kỹ tại sao bình
phương chứ không phải trị tuyệt đối). Cách khớp đường theo tiêu chí này gọi
là **least squares** — bình phương tối thiểu.

Có một công thức đại số tính thẳng ra `w` và `b` tối ưu, không cần dò từng
giá trị:

> `w = Σ(xᵢ − x̄)(yᵢ − ȳ) / Σ(xᵢ − x̄)²`
>
> `b = ȳ − w·x̄`

trong đó `x̄` là trung bình của mọi `x`, `ȳ` là trung bình của mọi `y`. Công
thức này tính bằng đại số thuần — không lặp, không dò thử, ra kết quả ngay
lập tức. (Bài `gradient-descent-tu-so-0` sẽ cho xem một cách KHÁC để tới
cùng đáp án: lặp dần từng bước nhỏ. Ở đây dùng công thức đóng vì bài toán
này đủ đơn giản để có một công thức đóng.)

Từ bài này, dữ liệu được lưu bằng `numpy` — một thư viện tính toán trên
DÃY SỐ. `np.array([...])` tạo một dãy số; `.mean()` tính trung bình của cả
dãy; và các phép `+`, `-`, `*`, `**` áp dụng đồng loạt lên TỪNG phần tử, không
cần viết vòng lặp. `dien_tich - x_tb` trừ `x_tb` khỏi MỌI phần tử của
`dien_tich` cùng lúc, ra một dãy số mới cùng độ dài.
::::

::::example{#tinh-w-b-that}
Bảy căn nhà của Byte, và công thức least-squares áp dụng bằng `numpy`:

```python title=readonly
import numpy as np

dien_tich = np.array([30, 40, 50, 60, 70, 80, 90], dtype=float)
gia = np.array([780, 1010, 1170, 1425, 1585, 1820, 1990], dtype=float)

x_tb = dien_tich.mean()
y_tb = gia.mean()

tu_so = np.sum((dien_tich - x_tb) * (gia - y_tb))
mau_so = np.sum((dien_tich - x_tb) ** 2)

w = tu_so / mau_so
b = y_tb - w * x_tb

print("w =", round(w, 4))
print("b =", round(b, 4))
print("mse =", round(np.mean((gia - (w * dien_tich + b)) ** 2), 4))

# doi chieu: np.linalg.lstsq giai cung bai toan bang dai so tuyen tinh
A = np.vstack([dien_tich, np.ones(len(dien_tich))]).T
w_doi_chieu, b_doi_chieu = np.linalg.lstsq(A, gia, rcond=None)[0]
print("doi chieu:", round(w_doi_chieu, 4), round(b_doi_chieu, 4))
```

```text title=readonly
w = 20.2321
b = 183.2143
mse = 363.1378
doi chieu: 20.2321 183.2143
```

Công thức tay và `np.linalg.lstsq` (giải bằng đại số tuyến tính, không phải
thư viện học máy đóng gói) ra ĐÚNG cùng một cặp số — xác nhận công thức tay
viết đúng. `w ≈ 20.23` nghĩa là mỗi m2 thêm vào, giá dự đoán tăng khoảng
20.23 triệu đồng. `b ≈ 183.21` là giá trị đường thẳng cắt trục khi diện tích
bằng 0 — một hằng số toán học của đường thẳng, không phải giá một căn nhà
0 m2 có thật. `mse ≈ 363.14` là tổng bình phương sai số CHIA cho số điểm — một
con số đo "khớp tốt tới đâu", sẽ được mổ xẻ kỹ ở bài sau.
::::

::::predict{#doan-gia-nha-55m2 commitOnce}
Mô hình đã khớp xong: `w ≈ 20.2321`, `b ≈ 183.2143`. Một căn nhà MỚI, diện
tích 55 m2 — chưa hề có trong bảy điểm dữ liệu.

**Trước khi tính**, bạn đoán: mô hình `y = w·x + b` dự đoán giá căn nhà này
khoảng bao nhiêu?

:::opt{correct}
Khoảng 1296 triệu đồng — `20.2321 × 55 + 183.2143 ≈ 1295.98`
:::

:::opt
Khoảng 1113 triệu đồng — chỉ cần nhân `w` với diện tích là đủ
::why
Gần đúng ở phần nhân `w × 55 ≈ 1112.77` — đúng phần ĐÓNG GÓP từ độ dốc.

Chỗ lệch: phần đó mới chỉ là một nửa công thức. `b` không phải một chi tiết
phụ có thể bỏ qua — nó là phần CỐ ĐỊNH cộng thêm vào MỌI dự đoán, bất kể
diện tích bao nhiêu. Thiếu `b`, đường dự đoán sẽ đi qua gốc toạ độ — trong
khi đường khớp thật của bảy điểm này cắt trục ở `183.21`, không phải `0`.
::
:::

:::opt
Khoảng 10 097 triệu đồng — nhân `b` với diện tích rồi cộng `w`
::why
Gần đúng ở việc bạn dùng đúng cả hai con số `w` và `b` — không bỏ sót cái
nào.

Chỗ lệch: vai trò của `w` và `b` bị đảo ngược. `w` — con số NHÂN với diện
tích — là ĐỘ DỐC (đơn vị triệu đồng/m2); `b` — con số CỘNG vào cuối — là
HẰNG SỐ (đơn vị triệu đồng). Nhân `b ≈ 183.21` với diện tích cho ra một con
số lớn gấp cả nghìn lần giá thật, vì `b` không hề mang đơn vị "trên mỗi m2".
::
:::

:::opt
Một số âm, khoảng −930 triệu đồng — lấy `b` trừ `w` nhân diện tích
::why
Gần đúng ở việc bạn có dùng cả `w`, `b`, và diện tích — đủ ba thành phần.

Chỗ lệch nằm ở phép toán: công thức là `w·x + b` — NHÂN rồi CỘNG — không
phải `b − w·x`. Đổi phép cộng thành trừ và đổi thứ tự hai số hạng đảo
ngược hẳn ý nghĩa: giá nhà không thể âm, và dấu âm ở đây là dấu hiệu công
thức đã bị lắp sai, không phải một kết quả cần chấp nhận.
::
:::
::::

::::code{#khop-duong-least-squares}
Viết nốt công thức least-squares: tính tử số của `w` (còn thiếu vế
`(gia − y_tb)`), rồi tính `w` và `b` từ `tu_so`/`mau_so`/`x_tb`/`y_tb` đã có
sẵn. Cuối cùng, dùng mô hình dự đoán giá một căn nhà 55 m2.

```python title=starter
import numpy as np

dien_tich = np.array([30, 40, 50, 60, 70, 80, 90], dtype=float)
gia = np.array([780, 1010, 1170, 1425, 1585, 1820, 1990], dtype=float)

x_tb = dien_tich.mean()
y_tb = gia.mean()

tu_so = np.sum((dien_tich - x_tb) * ___)     # ve con thieu: (gia - y_tb)
mau_so = np.sum((dien_tich - x_tb) ** 2)

w = ___                                      # tu_so chia mau_so
b = ___                                      # y_tb tru w nhan x_tb

gia_du_doan_55 = w * 55 + b

print(round(w, 4))
print(round(b, 4))
print(round(gia_du_doan_55, 2))
```

```python title=solution
import numpy as np

dien_tich = np.array([30, 40, 50, 60, 70, 80, 90], dtype=float)
gia = np.array([780, 1010, 1170, 1425, 1585, 1820, 1990], dtype=float)

x_tb = dien_tich.mean()
y_tb = gia.mean()

tu_so = np.sum((dien_tich - x_tb) * (gia - y_tb))
mau_so = np.sum((dien_tich - x_tb) ** 2)

w = tu_so / mau_so
b = y_tb - w * x_tb

gia_du_doan_55 = w * 55 + b

print(round(w, 4))
print(round(b, 4))
print(round(gia_du_doan_55, 2))
```

```python title=test
assert round(w, 4) == 20.2321, f"w phai la 20.2321 -- dang ra {round(w, 4)}"
assert round(b, 4) == 183.2143, f"b phai la 183.2143 -- dang ra {round(b, 4)}"
assert round(gia_du_doan_55, 2) == 1295.98, f"gia du doan tai x=55 phai la 1295.98 -- dang ra {round(gia_du_doan_55, 2)}"
assert w > 0, "dien tich lon hon thi gia phai cao hon -- w phai duong"
```

:::hints
- kind: attention
  body: Ba chỗ trống đều là PHÉP TÍNH, không phải con số chép sẵn. Chỗ đầu nằm trong `np.sum(...)` — nó nhân với vế `(dien_tich - x_tb)` đã có sẵn, còn thiếu đúng vế còn lại của công thức `Σ(xᵢ−x̄)(yᵢ−ȳ)`. Hai chỗ dưới là công thức `w` và `b` đã viết ở phần giải thích, chỉ cần chép lại bằng đúng tên biến `tu_so`, `mau_so`, `x_tb`, `y_tb` đã có trong khung.
- kind: strategy
  body: 'Chỗ trống 1 là `(gia - y_tb)` — cùng hình dạng với `(dien_tich - x_tb)` đứng ngay trước nó, chỉ đổi dãy số. Chỗ trống 2 là phép chia `tu_so / mau_so`. Chỗ trống 3 là `y_tb - w * x_tb` — dùng `w` vừa tính ở dòng trên.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `(gia - y_tb)`, `tu_so / mau_so`, và `y_tb - w * x_tb`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả ba chỗ trống phải THẬT SỰ tính toán từ tu_so/mau_so/x_tb/y_tb — không được chép sẵn con số 20.2321 hay 183.2143, vì bài này dạy CÔNG THỨC, không dạy học thuộc đáp án
  requireAst:
  - kind: uses-name, target: tu_so, min: 1
  - kind: uses-name, target: mau_so, min: 1
  - kind: uses-name, target: y_tb, min: 2
  - kind: uses-name, target: x_tb, min: 3
  - kind: uses-operator, target: "/", min: 1
  # Đã thử thật (script ngoài, dùng ast.parse trực tiếp): điền True/1/0 vào
  # cả ba chỗ trống đồng loạt cho tu_so=0, mau_so=0, y_tb=0, x_tb=2, "/"=0 —
  # dưới NGƯỠNG ở cả năm luật (min lần lượt 1/1/2/3/1). Điền ĐÚNG chỗ trống 1
  # nhưng CHÉP HẰNG SỐ cho w và b (vd w=20.2321, b=183.2143) cho tu_so=0,
  # mau_so=0, y_tb=1, x_tb=2 — vẫn dưới ngưỡng tu_so/mau_so/x_tb, bị chặn.
  # Lời giải thật cho đúng [1, 1, 2, 3, 1] — qua sạch cả năm luật.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^20\\.2321\\n183\\.2143\\n1295\\.98\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`w ≈ 20.23`, `b ≈ 183.21` — không dò thử, một công thức tính thẳng ra. Nhưng
công thức này ngầm chọn "bình phương sai số" làm thước đo khớp tốt. Sao lại
là bình phương, chứ không phải khoảng cách thường?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Công thức least-squares vừa viết tối ưu hoá MỘT con số cụ thể: tổng BÌNH
PHƯƠNG các sai số. Nhưng "sai số" cũng đo được bằng cách khác — chẳng hạn
tổng TRỊ TUYỆT ĐỐI của sai số, không bình phương gì cả.

Hai cách đo đó có luôn chọn ra CÙNG một đường thẳng `w, b` không? Và nếu
không — cách nào mới là lựa chọn "đúng" để một MÔ HÌNH tự học, chứ không
phải cách nào rẻ hơn để tính bằng tay?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
