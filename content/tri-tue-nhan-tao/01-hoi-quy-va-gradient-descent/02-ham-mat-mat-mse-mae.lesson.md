---
id: tri-tue-nhan-tao.hoi-quy-va-gradient-descent.ham-mat-mat-mse-mae
title: "Hàm mất mát: MSE và MAE"
summary: "Trên năm sai số [-2, 1, 0, 3, -1]: MSE=3.0, MAE=1.4. Đổi giá trị cuối thành ngoại lai [-2, 1, 0, 3, -10]: MSE nhảy lên 22.8 (gấp 7.6 lần) trong khi MAE chỉ lên 3.2 (gấp 2.29 lần) — MSE phạt sai số lớn nặng hơn hẳn, còn đạo hàm của nó (2e) mượt ở mọi nơi, trong khi đạo hàm của MAE (dấu của e) gãy đúng tại e=0."
locale: vi
track: tri-tue-nhan-tao
module: hoi-quy-va-gradient-descent
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.ham-mat-mat]
requires: [ai.hoi-quy-tuyen-tinh]
concepts: [ai.ham-mat-mat]
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
Bài trước dùng `mse` mà chưa hỏi: vì sao BÌNH PHƯƠNG sai số, không phải chỉ
đo khoảng cách?
::::

::::explain{#hai-cach-do-sai}
Một **hàm mất mát** (loss function) là một con số duy nhất đo "mô hình đang
sai bao nhiêu" trên toàn bộ dữ liệu — càng nhỏ càng khớp tốt. Bài trước đã
dùng một hàm mất mát mà chưa gọi tên đầy đủ: **MSE** — Mean Squared Error,
trung bình của BÌNH PHƯƠNG từng sai số.

> `MSE = (1/n) · Σ eᵢ²`  , với `eᵢ = yᵢ − dự_đoánᵢ`

Có một cách đo khác, cũng hợp lý: **MAE** — Mean Absolute Error, trung bình
của TRỊ TUYỆT ĐỐI từng sai số, không bình phương gì cả.

> `MAE = (1/n) · Σ |eᵢ|`

Cả hai đều là "trung bình mức độ sai", nhưng phản ứng khác nhau trước MỘT
loại sai số: sai số RẤT LỚN, tức **ngoại lai** (outlier). Bình phương một số
lớn cho ra một số lớn hơn NHIỀU — sai số `10` đóng góp `100` vào tổng của
MSE, trong khi chỉ đóng góp `10` vào tổng của MAE. MSE vì vậy PHẠT NẶNG các
sai số lớn hơn hẳn mức phạt của MAE.

Có một khác biệt thứ hai, quan trọng ngang vậy cho bài sau: **đạo hàm**.
Đạo hàm của `e²` theo `e` là `2e` — một hàm TUYẾN TÍNH, xác định và MƯỢT
(khả vi) ở MỌI giá trị của `e`, kể cả `e = 0`. Đạo hàm của `|e|` theo `e`
là `+1` khi `e > 0`, `−1` khi `e < 0` — nhưng tại đúng `e = 0`, hai phía cho
ra hai độ dốc khác nhau, nên đạo hàm ở đó GÃY, không xác định theo nghĩa
thông thường. `gradient-descent-tu-so-0` (bài kế) cần đạo hàm của hàm mất
mát để biết nên chỉnh `w`, `b` theo hướng nào — một đạo hàm mượt khắp nơi
dễ dùng hơn hẳn một đạo hàm có điểm gãy. Đây là lý do MSE, chứ không phải
MAE, là lựa chọn mặc định cho hồi quy tuyến tính huấn luyện bằng gradient.
::::

::::example{#outlier-lam-lech-can}
Năm sai số "bình thường", rồi đổi sai số cuối thành một ngoại lai lớn:

```python title=readonly
import numpy as np

sai_so_thuong = np.array([-2.0, 1.0, 0.0, 3.0, -1.0])
sai_so_ngoai_lai = np.array([-2.0, 1.0, 0.0, 3.0, -10.0])

mse_thuong = np.mean(sai_so_thuong ** 2)
mae_thuong = np.mean(np.abs(sai_so_thuong))
mse_ngoai_lai = np.mean(sai_so_ngoai_lai ** 2)
mae_ngoai_lai = np.mean(np.abs(sai_so_ngoai_lai))

print("binh thuong:", "MSE =", mse_thuong, " MAE =", mae_thuong)
print("co ngoai lai:", "MSE =", mse_ngoai_lai, " MAE =", mae_ngoai_lai)
print("MSE tang:", round(mse_ngoai_lai / mse_thuong, 4), "lan")
print("MAE tang:", round(mae_ngoai_lai / mae_thuong, 4), "lan")
```

```text title=readonly
binh thuong: MSE = 3.0  MAE = 1.4
co ngoai lai: MSE = 22.8  MAE = 3.2
MSE tang: 7.6 lan
MAE tang: 2.2857 lan
```

Đổi ĐÚNG một sai số, từ `-1` thành `-10` (lớn gấp 10 lần) — `MSE` tăng gấp
`7.6` lần, trong khi `MAE` chỉ tăng gấp `2.29` lần. Cùng một thay đổi dữ
liệu, MSE phản ứng mạnh hơn MAE rất nhiều — đúng hệ quả của việc bình
phương một số lớn cho ra một số lớn hơn nhiều.
::::

::::predict{#doan-phan-ung-truoc-ngoai-lai commitOnce}
Vẫn hai bộ sai số ở trên: bộ thường `[-2, 1, 0, 3, -1]` và bộ có ngoại lai
`[-2, 1, 0, 3, -10]` — chỉ khác đúng một giá trị cuối.

**Trước khi tính**, bạn đoán: khi đổi sang bộ có ngoại lai, MSE hay MAE tăng
theo TỶ LỆ (phần trăm so với giá trị ban đầu) lớn hơn?

:::opt{correct}
MSE tăng theo tỷ lệ lớn hơn hẳn — vì bình phương khuếch đại đúng sai số lớn
nhất, còn MAE chỉ cộng thêm phần chênh lệch trị tuyệt đối
:::

:::opt
MAE tăng theo tỷ lệ lớn hơn — vì MAE đo trực tiếp bằng đơn vị gốc, nên nhạy
với thay đổi hơn MSE (đơn vị đã bị bình phương)
::why
Gần đúng ở việc bạn để ý ĐÚNG rằng MSE và MAE có ĐƠN VỊ khác nhau — MSE
mang đơn vị bình phương của sai số, MAE mang đơn vị gốc. Quan sát đó không
sai.

Chỗ lệch: đơn vị khác nhau không quyết định TỶ LỆ tăng khi có ngoại lai.
Chính vì bị bình phương, một sai số lớn (`10`) đóng góp `100` vào tổng MSE —
một tỷ trọng áp đảo so với các sai số nhỏ còn lại. MAE chỉ cộng thêm đúng
phần chênh `|−10| − |−1| = 9` vào tổng, không bị khuếch đại. Số liệu thật đã
cho thấy: MSE tăng 7.6 lần, MAE chỉ tăng 2.29 lần — MSE tăng NHIỀU hơn, không
phải ít hơn.
::
:::

:::opt
Cả hai tăng theo đúng cùng một tỷ lệ — vì cả hai đều là "trung bình mức độ
sai" trên cùng một bộ dữ liệu
::why
Gần đúng ở việc cả hai ĐÚNG LÀ được tính trên cùng một bộ sai số, không
phải hai bộ dữ liệu khác nhau.

Chỗ lệch: "cùng bộ dữ liệu" không có nghĩa "cùng công thức khuếch đại sai
số". MSE bình phương từng sai số trước khi cộng — một phép toán KHÔNG
TUYẾN TÍNH, khuếch đại số lớn nhiều hơn số nhỏ theo tỷ lệ không đều. MAE
cộng trực tiếp trị tuyệt đối — tuyến tính, mỗi đơn vị chênh lệch đóng góp
như nhau. Hai công thức khác bản chất nên hai tỷ lệ tăng khác nhau hẳn:
7.6 lần so với 2.29 lần, không hề bằng nhau.
::
:::
::::

::::code{#viet_mse_mae}
Viết hai hàm `mse(sai_so)` và `mae(sai_so)` bằng `numpy`, rồi áp dụng lên
hai bộ sai số: bộ thường và bộ có một giá trị ngoại lai.

```python title=starter
import numpy as np

def mse(sai_so):
    return np.mean(___)              # trung binh cua BINH PHUONG tung sai so

def mae(sai_so):
    return np.mean(___)              # trung binh cua TRI TUYET DOI tung sai so

sai_so_thuong = np.array([-2.0, 1.0, 0.0, 3.0, -1.0])
sai_so_ngoai_lai = np.array([-2.0, 1.0, 0.0, 3.0, -10.0])

mse_thuong = mse(sai_so_thuong)
mae_thuong = mae(sai_so_thuong)
mse_ngoai_lai = mse(sai_so_ngoai_lai)
mae_ngoai_lai = mae(sai_so_ngoai_lai)

print(round(mse_thuong, 2), round(mae_thuong, 2))
print(round(mse_ngoai_lai, 2), round(mae_ngoai_lai, 2))
```

```python title=solution
import numpy as np

def mse(sai_so):
    return np.mean(sai_so ** 2)

def mae(sai_so):
    return np.mean(np.abs(sai_so))

sai_so_thuong = np.array([-2.0, 1.0, 0.0, 3.0, -1.0])
sai_so_ngoai_lai = np.array([-2.0, 1.0, 0.0, 3.0, -10.0])

mse_thuong = mse(sai_so_thuong)
mae_thuong = mae(sai_so_thuong)
mse_ngoai_lai = mse(sai_so_ngoai_lai)
mae_ngoai_lai = mae(sai_so_ngoai_lai)

print(round(mse_thuong, 2), round(mae_thuong, 2))
print(round(mse_ngoai_lai, 2), round(mae_ngoai_lai, 2))
```

```python title=test
assert round(mse_thuong, 2) == 3.0, f"mse cua bo thuong phai la 3.0 -- dang ra {round(mse_thuong, 2)}"
assert round(mae_thuong, 2) == 1.4, f"mae cua bo thuong phai la 1.4 -- dang ra {round(mae_thuong, 2)}"
assert round(mse_ngoai_lai, 2) == 22.8, f"mse cua bo co ngoai lai phai la 22.8 -- dang ra {round(mse_ngoai_lai, 2)}"
assert round(mae_ngoai_lai, 2) == 3.2, f"mae cua bo co ngoai lai phai la 3.2 -- dang ra {round(mae_ngoai_lai, 2)}"
assert (mse_ngoai_lai / mse_thuong) > (mae_ngoai_lai / mae_thuong), "MSE phai tang theo ty le LON HON mae khi co ngoai lai -- dung ban chat cua binh phuong"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trong `np.mean(...)` của hai hàm khác nhau. `mse` cần BÌNH PHƯƠNG từng sai số trước khi lấy trung bình. `mae` cần TRỊ TUYỆT ĐỐI từng sai số trước khi lấy trung bình — `numpy` có sẵn `np.abs(...)` cho việc đó.
- kind: strategy
  body: 'mse: `sai_so ** 2` (numpy bình phương từng phần tử). mae: `np.abs(sai_so)` (numpy lấy trị tuyệt đối từng phần tử).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `sai_so ** 2` và `np.abs(sai_so)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: mse phải THẬT SỰ bình phương sai_so (dùng **), mae phải THẬT SỰ lấy trị tuyệt đối (dùng np.abs) — không phải một hằng số cố định, vì hai hàm này còn phải chạy đúng trên bộ dữ liệu KHÁC ở ngay phần dưới
  requireAst:
  - kind: uses-operator, target: "**", min: 1
  - kind: uses-call, target: abs, min: 1
  - kind: uses-name, target: sai_so, min: 2
  # Đã thử thật: điền True/1/0 vào cả hai chỗ trống cho **=0, abs=0, sai_so=0
  # -- dưới ca ba nguong (min 1/1/2). Vi mse/mae duoc goi tren HAI bo du lieu
  # khac nhau (thuong va ngoai lai), MOT hang so co dinh cung khong the qua
  # duoc ca bon assert gia tri trong tier tests -- static o day la lop chan
  # thu hai, khong phai lop chan duy nhat.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^3\\.0 1\\.4\\n22\\.8 3\\.2\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
7.6 lần so với 2.29 lần — cùng một ngoại lai, hai cách đo phản ứng khác hẳn
nhau. MSE nhạy hơn, và đạo hàm của nó mượt khắp nơi — hai lý do MSE là mặc
định cho bài tiếp theo.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Công thức least-squares ở bài trước tính RA `w`, `b` bằng đại số, ngay lập
tức — không cần biết gì về "đạo hàm mượt" hay "đạo hàm gãy" cả. Vậy tại sao
bài này lại nhấn mạnh chuyện đạo hàm của MSE có mượt hay không?

Vì công thức đóng đó chỉ tồn tại cho MỘT bài toán cụ thể: khớp một đường
THẲNG. Phần lớn các mô hình AI phức tạp hơn nhiều — không có công thức đóng
nào giải được chúng bằng vài dòng đại số. Chúng cần một cách khác để tìm
`w`, `b` tốt nhất: dò dần, từng bước nhỏ, theo hướng mà đạo hàm chỉ ra.

Bài sau bắt đầu đúng cách đó.
::::

::::checkpoint{mastery=0.8}
::::
