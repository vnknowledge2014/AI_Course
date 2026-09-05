---
id: tri-tue-nhan-tao.co-che-attention.scale-va-softmax
title: "Chia tỉ lệ và softmax: từ điểm thô sang trọng số attention"
summary: "Chia điểm attention cho √d_k (ổn định số học -- điểm quá lớn làm softmax bão hoà, gần one-hot), rồi Tensor.softmax() TÁI DÙNG NGUYÊN áp lên hàng đã chia tỉ lệ. Trên S (4,4) của bài trước, d_k=4 nên chia cho 2: hàng cuối KHÔNG chia tỉ lệ cho xác suất lớn nhất 0,803738 (gần bão hoà); CÓ chia cho 0,554415 (mềm hơn hẳn). Chia cho d_k=4 (sai lầm thường gặp, thay vì √d_k) làm phân phối CÀNG PHẲNG hơn nữa (0,395909) -- đo bằng số thật, không suy luận suông."
locale: vi
track: tri-tue-nhan-tao
module: co-che-attention
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.scale-va-softmax]
requires: [ai.diem-attention-tinh-tay]
concepts: [ai.scale-va-softmax]
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
Điểm attention thô `S` có thể là số bất kỳ — bài trước đã thấy `S` chứa cả `10` lẫn `-2,58909`. Trước khi biến nó thành trọng số, cần một bước ổn định số học quen thuộc.
::::

::::explain{#scale_va_softmax}
Điểm attention thô `S[i, j] = Q[i] · K[j]` là TỔNG của `d_k` số hạng (`d_k` = số chiều của `Q`/`K`, ở đây `4`). Càng nhiều số hạng cộng lại, độ lớn TRUNG BÌNH của tổng càng có xu hướng tăng — không phải vì bản chất "liên quan" giữa hai token tăng lên, mà thuần tuý vì CÓ NHIỀU số hạng hơn để cộng. Điểm thô lớn đưa vào `softmax` làm phân phối kết quả gần như **one-hot** — một trọng số gần `1`, các trọng số còn lại gần `0` — đúng kiểu bão hoà đã gặp ở `on-dinh-so-hoc` (T8.1c): logit quá lớn làm `softmax` mất khả năng phân biệt "khá liên quan" với "rất liên quan", và gradient qua vùng bão hoà gần như triệt tiêu.

Cách khắc phục chuẩn của Transformer: chia điểm thô cho `√d_k` TRƯỚC khi đưa vào `softmax`:

> `S_scaled = S / √d_k`, rồi `P = S_scaled.softmax()`

`softmax` ở đây là `Tensor.softmax()` ĐÃ CÓ SẴN, ĐÃ ỔN ĐỊNH SỐ HỌC (trừ giá trị lớn nhất mỗi hàng trước khi lấy mũ, `softmax-qua-tensor`, q8.3b) — TÁI DÙNG NGUYÊN, không viết lại một dòng công thức nào. Việc MỚI ở đây chỉ là bước CHIA TỈ LỆ đứng trước nó.
::::

::::example{#scale_softmax_that}
`S` từ bài trước (`4` token, `d_k = 4` nên `√d_k = 2`) — softmax KHÔNG chia tỉ lệ, rồi CÓ chia tỉ lệ:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out


S = Tensor([[5., 5., 6., 3.49246], [6., 9., 9., 0.542022], [5., 10., 9., -2.58909], [1.391794, -0.488424, 0.542022, 3.259085]])

d_k = 4
S_scaled = Tensor(S.data / np.sqrt(d_k))

P_khong_scale = S.softmax()
P_co_scale = S_scaled.softmax()

print(np.round(P_khong_scale.data, 6).tolist())
print(np.round(P_co_scale.data, 6).tolist())
print(round(float(P_khong_scale.data[3].max()), 6))
print(round(float(P_co_scale.data[3].max()), 6))
```

```text title=readonly
[[0.20244, 0.20244, 0.550289, 0.044831], [0.024286, 0.487805, 0.487805, 0.000104], [0.004902, 0.727473, 0.267622, 2e-06], [0.124211, 0.018949, 0.053102, 0.803738]]
[[0.242759, 0.242759, 0.400242, 0.11424], [0.099714, 0.446888, 0.446888, 0.00651], [0.048558, 0.591554, 0.358796, 0.001092], [0.217951, 0.085128, 0.142506, 0.554415]]
0.803738
0.554415
```

Hàng CUỐI cho contrast rõ nhất: KHÔNG chia tỉ lệ, xác suất lớn nhất của hàng đó là `0,803738` — gần `4/5` tổng trọng số dồn vào MỘT vị trí, các vị trí còn lại nhận rất ít (`0,124211`, `0,018949`, `0,053102`). CÓ chia tỉ lệ, xác suất lớn nhất giảm còn `0,554415` — vẫn là vị trí được chú ý NHIỀU NHẤT, nhưng phân phối MỀM hơn hẳn, các vị trí khác vẫn giữ được trọng số đáng kể. Cả bốn hàng đều theo xu hướng này: chia tỉ lệ luôn làm phân phối bớt cực đoan hơn, không đảo ngược thứ hạng "vị trí nào được chú ý nhất" (thứ hạng giữ nguyên), chỉ làm nó bớt áp đảo.
::::

::::predict{#doan_chia_cho_dk commitOnce}
Ví dụ trên chia `S` cho `√d_k = 2` (`d_k = 4`), cho xác suất lớn nhất hàng cuối là `0,554415` — mềm hơn hẳn so với không chia (`0,803738`).

**Trước khi tính**, bạn đoán: nếu (nhầm lẫn) chia `S` cho CHÍNH `d_k = 4` thay vì `√d_k = 2` (một số chia LỚN HƠN), xác suất lớn nhất của hàng cuối sẽ LỚN HƠN hay NHỎ HƠN `0,554415`?

:::opt{correct}
Nhỏ hơn — chia cho một số LỚN HƠN (`4` thay vì `2`) làm MỌI điểm attention thu nhỏ lại NHIỀU HƠN nữa trước khi vào `softmax`; điểm số càng gần nhau (chênh lệch càng nhỏ) thì `softmax` càng cho phân phối GẦN ĐỀU hơn — xác suất lớn nhất sẽ tiếp tục giảm, không tăng
:::

:::opt
Lớn hơn — chia cho một số lớn hơn "chia hết hơn", làm phép tính "chuẩn hoá kỹ hơn", nên phân phối phải TẬP TRUNG hơn, không phải phân tán hơn
::why
Gần đúng ở việc trực giác "chia cho số lớn hơn thì chuẩn hoá kỹ hơn" nghe có vẻ hợp lý về mặt ngôn từ — quan sát về việc số chia lớn hơn LÀ một phép biến đổi mạnh hơn không sai.

Chỗ lệch: "chuẩn hoá kỹ hơn" ở đây có nghĩa cụ thể là THU NHỎ khoảng cách tương đối giữa các điểm số — và khoảng cách càng thu nhỏ, `softmax` càng khó phân biệt điểm nào lớn hơn điểm nào, nên phân phối kết quả càng ĐỀU hơn (PHẲNG hơn), không phải tập trung hơn. Chia cho số càng lớn thì kết quả càng gần phân phối ĐỀU (`1/4` mỗi vị trí, với `4` token), không phải gần one-hot.
::
:::

:::opt
Bằng nhau — `softmax` đã ỔN ĐỊNH SỐ HỌC (trừ max trước khi lấy mũ), nên số chia trước đó không ảnh hưởng gì tới kết quả cuối cùng
::why
Gần đúng ở việc nhớ đúng rằng `Tensor.softmax()` có bước ổn định số học (trừ giá trị lớn nhất mỗi hàng) — quan sát đó không sai.

Chỗ lệch: bước ổn định số học chỉ NGĂN TRÀN SỐ (tránh `exp` của một số cực lớn cho `inf`/`nan`) — nó KHÔNG làm softmax "không phân biệt được" các số chia khác nhau ở TRƯỚC bước trừ max. Trừ cho max là một phép DỊCH (trừ một hằng số CHUNG cho cả hàng), còn chia cho `√d_k` hay `d_k` là một phép NHÂN TỈ LỆ (co giãn KHOẢNG CÁCH tương đối giữa các điểm) — hai phép khác nhau, và phép co giãn ĐÓ vẫn ảnh hưởng trực tiếp tới hình dạng phân phối cuối cùng.
::
:::
::::

::::code{#viet_scale_softmax}
Hoàn thiện hai chỗ trống: chia tỉ lệ điểm attention cho `√d_k`, rồi gọi `softmax()` đã có sẵn.

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out


S = Tensor([[5., 5., 6., 3.49246], [6., 9., 9., 0.542022], [5., 10., 9., -2.58909], [1.391794, -0.488424, 0.542022, 3.259085]])

d_k = 4
S_scaled = Tensor(S.data / ___)      # np.sqrt(d_k)
P = ___                              # S_scaled.softmax()

print(np.round(P.data, 6).tolist())
print(round(float(P.data[3].max()), 6))
```

```python title=solution
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out


S = Tensor([[5., 5., 6., 3.49246], [6., 9., 9., 0.542022], [5., 10., 9., -2.58909], [1.391794, -0.488424, 0.542022, 3.259085]])

d_k = 4
S_scaled = Tensor(S.data / np.sqrt(d_k))
P = S_scaled.softmax()

print(np.round(P.data, 6).tolist())
print(round(float(P.data[3].max()), 6))
```

```python title=test
import numpy as np

assert np.round(P.data, 6).tolist() == [[0.242759, 0.242759, 0.400242, 0.11424], [0.099714, 0.446888, 0.446888, 0.00651], [0.048558, 0.591554, 0.358796, 0.001092], [0.217951, 0.085128, 0.142506, 0.554415]], f"P.data sai -- dang ra {np.round(P.data, 6).tolist()}"
assert np.allclose(np.sum(P.data, axis=-1), 1.0), f"moi hang cua P.data phai cong dung 1.0 -- dang ra {np.sum(P.data, axis=-1).tolist()}"

# rieng kiem tra DA CHIA DUNG cho sqrt(d_k), khong phai d_k hay mot hang so khac:
# sosanh voi ket qua neu chia cho d_k (phai KHAC, phang hon)
S_chia_dk = np.array(S.data) / 4.0
def sm(z):
    zs = z - np.max(z, axis=-1, keepdims=True); e = np.exp(zs); return e / np.sum(e, axis=-1, keepdims=True)
P_chia_dk = sm(S_chia_dk)
assert round(float(P.data[3].max()), 6) != round(float(P_chia_dk[3].max()), 6), "P phai khac ket qua neu chia cho d_k (thay vi sqrt(d_k)) -- kiem tra da dung sqrt chua"
assert round(float(P.data[3].max()), 6) > round(float(P_chia_dk[3].max()), 6), f"chia cho sqrt(d_k) (so nho hon) phai cho phan phoi TAP TRUNG HON chia cho d_k -- dang ra {round(float(P.data[3].max()), 6)} va {round(float(P_chia_dk[3].max()), 6)}"

# rieng kiem tra tinh mem hon han so voi KHONG scale
P_khong_scale = sm(np.array(S.data))
assert round(float(P.data[3].max()), 6) < round(float(P_khong_scale[3].max()), 6), "co scale phai cho phan phoi MEM HON (xac suat lon nhat NHO HON) so voi khong scale"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là mẫu số chia tỉ lệ — CĂN BẬC HAI của `d_k`, `np.sqrt(d_k)` (không phải chính `d_k`, cũng không phải một hằng số chép tay). Chỗ hai gọi PHƯƠNG THỨC `softmax()` ĐÃ CÓ SẴN của `Tensor`, trên `S_scaled` (đã chia tỉ lệ) — `S_scaled.softmax()`, không phải trên `S` gốc (chưa chia tỉ lệ).
- kind: strategy
  body: 'Chỗ đầu: `np.sqrt(d_k)`. Chỗ hai: `S_scaled.softmax()`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `np.sqrt(d_k)` và `S_scaled.softmax()`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: S_scaled phai chia S.data cho np.sqrt(d_k) (CAN BAC HAI cua d_k, khong phai chinh d_k hay mot hang so chep tay); P phai goi S_scaled.softmax() (tren du lieu DA chia ti le, khong phai goi tren S goc)
  requireAst:
  - kind: uses-call, target: sqrt, min: 1
  - kind: uses-name, target: d_k, min: 1
  - kind: uses-call, target: softmax, min: 1
  - kind: uses-name, target: S_scaled, min: 1
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # class Tensor + harness): loi giai dung dat=true. sqrt=1 (chi blank1).
  # d_k=1 (dinh nghia "d_k = 4" la Store, khong dem -- Load duy nhat la
  # trong blank1 "np.sqrt(d_k)"). softmax=1 (chi blank2 "S_scaled.softmax()"
  # -- dinh nghia phuong thuc khong tinh). S_scaled=1 (dinh nghia "S_scaled
  # = Tensor(...)" la Store cho ten do, khong dem; Load duy nhat la trong
  # blank2 "S_scaled.softmax()").
  # Cheat "S_scaled = Tensor(S.data / d_k)" (chia cho d_k, quen sqrt) lam
  # "sqrt" tut xuong 0 -- bi chan RIENG, VA da tu kiem chung bang Python
  # that: P.data[3].max() khi do la 0.395909, KHAC 0.554415 dung -- bi bat
  # DOC LAP boi assert gia tri P.data. Cheat "P = S.softmax()" (goi tren S
  # goc, bo qua S_scaled) lam "S_scaled" tut xuong 0 (khong con Load nao)
  # -- bi chan RIENG, VA da tu kiem chung: P.data[3].max() khi do la
  # 0.803738 (gan bao hoa) thay vi 0.554415 -- bi bat DOC LAP boi assert
  # gia tri.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[\\[0\\.242759, 0\\.242759, 0\\.400242, 0\\.11424\\], \\[0\\.099714, 0\\.446888, 0\\.446888, 0\\.00651\\], \\[0\\.048558, 0\\.591554, 0\\.358796, 0\\.001092\\], \\[0\\.217951, 0\\.085128, 0\\.142506, 0\\.554415\\]\\]\\n0\\.554415\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Điểm thô giờ đã thành trọng số attention — mỗi hàng một phân phối xác suất, mềm hơn nhờ chia tỉ lệ. Bài sau: dùng trọng số này để thực sự LẤY thông tin ra từ Value.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mỗi hàng của `P` (sau scale + softmax) là một phân phối xác suất trên `4` vị trí — nó nói "vị trí này nên chú ý bao nhiêu tới mỗi vị trí khác". Nhưng bản thân `P` chưa mang nội dung gì — nó chỉ là TRỌNG SỐ. Phép toán `Tensor` nào (đã học ở bài `query-key-value`) sẽ kết hợp trọng số `P` với `V` (Value, nội dung thật) để tạo ra đầu ra CUỐI CÙNG của một tầng attention?
::::

::::checkpoint{mastery=0.85}
::::
