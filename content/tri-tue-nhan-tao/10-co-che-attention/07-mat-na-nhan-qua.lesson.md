---
id: tri-tue-nhan-tao.co-che-attention.mat-na-nhan-qua
title: "Causal mask: che tương lai trước khi softmax"
summary: "Mô hình ngôn ngữ tự hồi quy KHÔNG được nhìn token TƯƠNG LAI (vị trí j>i). Cách làm: gán điểm attention tại các vị trí bị cấm thành -inf THẬT (float('-inf')) TRƯỚC softmax -- vì Tensor.softmax đã trừ max mỗi hàng trước exp (ổn định số học có sẵn), -inf trừ một số hữu hạn vẫn là -inf, exp(-inf)=0 sạch, KHÔNG nan. Mask PHẢI dựng bằng np.where, KHÔNG dùng triu(...)×(-inf) -- phép nhân 0×(-inf) cho nan (gotcha thật, đã tự kiểm chứng). Trên 4 token: hàng cuối (token cuối, không có tương lai) KHÔNG đổi sau mask; ba hàng đầu đổi rõ, ví dụ hàng 0 từ phân phối 4 phần tử sụp về [1,0,0,0]."
locale: vi
track: tri-tue-nhan-tao
module: co-che-attention
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.mat-na-nhan-qua]
requires: [ai.dau-ra-attention]
concepts: [ai.mat-na-nhan-qua]
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
Attention vừa xây cho MỌI vị trí nhìn thấy MỌI vị trí khác — kể cả tương lai. Với một mô hình SINH văn bản (đoán token tiếp theo), đó là gian lận: token đang sinh ra không được biết trước token sẽ đứng SAU nó.
::::

::::explain{#mat_na_nhan_qua}
Một mô hình ngôn ngữ **tự hồi quy** (autoregressive — sinh từng token một, dựa vào những token ĐÃ sinh trước đó) không được để vị trí `i` nhìn thấy vị trí `j > i` (tương lai của nó, chưa tồn tại tại thời điểm dự đoán). **Causal mask** ("mặt nạ nhân quả") thực thi đúng ràng buộc này: TRƯỚC khi `softmax`, gán điểm attention tại MỌI cặp `(i, j)` với `j > i` thành `-∞`.

Sau `softmax`, `e^(-∞) = 0` — trọng số của các vị trí bị cấm trở thành ĐÚNG `0`, không góp một chút nào vào đầu ra (bài `dau-ra-attention`). Dùng `-∞` THẬT (`float('-inf')`, không phải một số cực âm hữu hạn như `-1e9`) hoạt động SẠCH SẼ ở đây, chính vì `Tensor.softmax()` ĐÃ ỔN ĐỊNH SỐ HỌC (trừ giá trị lớn nhất mỗi hàng trước khi lấy mũ, `softmax-qua-tensor`, q8.3b): mỗi hàng LUÔN có ít nhất một vị trí KHÔNG bị cấm (vị trí `i` luôn được phép nhìn chính nó, `j = i`), nên giá trị lớn nhất mỗi hàng luôn HỮU HẠN — `-∞` trừ một số hữu hạn vẫn là `-∞`, và `e^(-∞) = 0` là một phép tính HOÀN TOÀN xác định, không sinh ra `nan`.

**Cách dựng mask ĐÚNG** — dùng `np.where` để chọn giữa hai giá trị:

> `mask = np.where(tam_giac_tren == 1, float('-inf'), 0.0)`

**Cách dựng mask SAI, trông có vẻ gọn hơn** — nhân một mảng `0`/`1` với `-∞`:

> `mask = tam_giac_tren * float('-inf')`   ← SAI

Nghe hợp lý (`1 × (-∞) = -∞` đúng như mong muốn), nhưng `numpy` định nghĩa `0 × (-∞) = nan` (một phép nhân KHÔNG XÁC ĐỊNH về mặt toán học — không có giới hạn duy nhất khi một thừa số tiến về `0` và thừa số kia tiến về vô cực). MỌI vị trí ĐƯỢC PHÉP (đánh dấu `0` trong ma trận tam giác) sẽ biến thành `nan` thay vì `0` — phá huỷ toàn bộ mask, kể cả những vị trí lẽ ra phải giữ nguyên điểm số gốc.
::::

::::example{#mask_dung_va_sai}
Dựng mask cho `4` token bằng CẢ HAI cách, để thấy khác biệt:

```python title=readonly
import numpy as np

so_token = 4
tam_giac_tren = np.triu(np.ones((so_token, so_token)), k=1)   # 1 o vi tri BI CAM (j>i), 0 o vi tri duoc phep
print(tam_giac_tren.tolist())

mask_dung = np.where(tam_giac_tren == 1, float('-inf'), 0.0)
print(mask_dung.tolist())

mask_sai = tam_giac_tren * float('-inf')
print(mask_sai.tolist())
```

```text title=readonly
[[0.0, 1.0, 1.0, 1.0], [0.0, 0.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0]]
[[0.0, -inf, -inf, -inf], [0.0, 0.0, -inf, -inf], [0.0, 0.0, 0.0, -inf], [0.0, 0.0, 0.0, 0.0]]
[[nan, -inf, -inf, -inf], [nan, nan, -inf, -inf], [nan, nan, nan, -inf], [nan, nan, nan, nan]]
```

`mask_dung` giữ ĐÚNG `0` ở mọi vị trí được phép (không đổi điểm gốc khi cộng vào) và `-inf` ở vị trí bị cấm. `mask_sai` biến MỌI vị trí `0` (được phép — kể cả toàn bộ hàng cuối, vốn không bị cấm gì cả) thành `nan` — `0 × (-∞)` không có giá trị xác định. Cộng một `nan` vào bất kỳ điểm attention nào cũng biến CẢ HÀNG thành `nan` sau `softmax` — hỏng hoàn toàn, kể cả những vị trí lẽ ra an toàn.
::::

::::example{#mask_ap_dung_that}
Áp `mask_dung` vào điểm attention ĐÃ chia tỉ lệ (`S_scaled`, bài trước), TRƯỚC `softmax`, rồi so sánh với không mask:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        return out


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
S = Q.matmul(K.transpose())
d_k = 4
S_scaled = Tensor(S.data / np.sqrt(d_k))
P_khong_mask = S_scaled.softmax()

so_token = 4
mask_data = np.where(np.triu(np.ones((so_token, so_token)), k=1) == 1, float('-inf'), 0.0)
mask = Tensor(mask_data)
S_masked = S_scaled + mask
P_mask = S_masked.softmax()

print(np.round(P_khong_mask.data, 6).tolist())
print(np.round(P_mask.data, 6).tolist())
```

```text title=readonly
[[0.242759, 0.242759, 0.400242, 0.11424], [0.099714, 0.446888, 0.446888, 0.00651], [0.048558, 0.591554, 0.358796, 0.001092], [0.217951, 0.085128, 0.142506, 0.554415]]
[[1.0, 0.0, 0.0, 0.0], [0.182426, 0.817574, 0.0, 0.0], [0.048611, 0.592201, 0.359188, 0.0], [0.217951, 0.085128, 0.142506, 0.554415]]
```

Hàng `0` (token đầu tiên) trước mask có trọng số RẢI trên cả `4` vị trí (`[0,242759; 0,242759; 0,400242; 0,11424]`) — sau mask, sụp về `[1, 0, 0, 0]`: token `0` không có vị trí nào ở PHÍA TRƯỚC hay chính NÓ ngoài chính nó, nên toàn bộ trọng số dồn về đúng một chỗ. Hàng `1` và `2` cũng đổi rõ — trọng số bị cấm chuyển thành `0`, phần còn lại được PHÂN PHỐI LẠI (tổng vẫn `= 1`). Hàng `3` (token CUỐI CÙNG) hoàn toàn KHÔNG ĐỔI — nó không có vị trí tương lai nào để che, nên mask không xoá bớt thông tin gì của riêng nó.
::::

::::predict{#doan_hang_cuoi_khong_doi commitOnce}
Ba hàng đầu của `P` đều thay đổi rõ sau khi áp causal mask (bài trên). Hàng CUỐI (`vị trí 3`, token cuối cùng trong câu `4` token) thì KHÔNG đổi — trước và sau mask đều là `[0,217951; 0,085128; 0,142506; 0,554415]`.

**Trước khi tổng quát hoá**, bạn đoán: điều này ĐÚNG cho MỌI câu (bất kể bao nhiêu token), hay chỉ là trùng hợp của riêng ví dụ `4` token này?

:::opt{correct}
Đúng cho MỌI câu — causal mask chỉ che các vị trí `j > i` (tương lai); token CUỐI CÙNG trong bất kỳ câu nào (vị trí `so_token - 1`) không có vị trí nào LỚN HƠN nó, nên hàng mask của nó LUÔN toàn `0` — cộng một hàng toàn `0` vào điểm attention không đổi gì cả, bất kể câu dài bao nhiêu
:::

:::opt
Chỉ là trùng hợp của ví dụ này — với câu khác (số token khác, giá trị `Q`/`K` khác), hàng cuối SẼ đổi sau mask
::why
Gần đúng ở việc thận trọng, không vội tổng quát hoá từ MỘT ví dụ cụ thể — thái độ đó thường đúng.

Chỗ lệch: ở đây có một quy luật CẤU TRÚC, không phụ thuộc giá trị cụ thể của `Q`/`K` — mask được định nghĩa bằng CHỈ SỐ vị trí (`j > i`), không phụ thuộc GIÁ TRỊ điểm attention. Với `i` là vị trí CUỐI CÙNG của bất kỳ câu nào (dài `1` hay `100` token), không tồn tại `j` nào lớn hơn `i` — hàng mask của vị trí đó LUÔN toàn `0`, một sự thật về CẤU TRÚC chỉ số, đúng cho MỌI câu.
::
:::

:::opt
Đúng, nhưng chỉ vì hàng cuối của `V` (Value) tình cờ có giá trị đặc biệt trong ví dụ này
::why
Gần đúng ở việc để ý `V` đúng là THAM GIA vào bước SAU (đầu ra attention, bài `dau-ra-attention`) — quan sát về vai trò của `V` không sai.

Chỗ lệch: câu hỏi ở đây chỉ về `P` (trọng số attention, TRƯỚC khi nhân với `V`) — mask tác động lên `S_scaled` (điểm số), hoàn toàn KHÔNG liên quan gì tới giá trị của `V`. Hàng cuối của `P` không đổi vì hàng mask của nó toàn `0` — một sự thật về CHỈ SỐ vị trí, không phụ thuộc `V` chứa gì.
::
:::
::::

::::code{#viet_causal_mask}
Hoàn thiện hai chỗ trống: dựng mask ĐÚNG bằng `np.where` (không phải phép nhân), và áp nó vào `S_scaled` bằng `__add__` đã có sẵn.

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        return out


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
S = Q.matmul(K.transpose())
d_k = 4
S_scaled = Tensor(S.data / np.sqrt(d_k))

so_token = 4
mask_data = np.where(np.triu(np.ones((so_token, so_token)), k=1) == 1, ___, 0.0)     # float('-inf')
mask = Tensor(mask_data)
S_masked = S_scaled + ___                                                            # mask
P = S_masked.softmax()

print(np.round(P.data, 6).tolist())
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

    def matmul(self, other):
        out = Tensor(self.data @ other.data, (self, other), '@')
        return out

    def transpose(self):
        out = Tensor(self.data.T, (self,), 'T')
        return out

    def softmax(self):
        z = self.data
        z_shift = z - np.max(z, axis=-1, keepdims=True)
        e = np.exp(z_shift)
        s = e / np.sum(e, axis=-1, keepdims=True)
        out = Tensor(s, (self,), 'softmax')
        return out

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        return out


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
S = Q.matmul(K.transpose())
d_k = 4
S_scaled = Tensor(S.data / np.sqrt(d_k))

so_token = 4
mask_data = np.where(np.triu(np.ones((so_token, so_token)), k=1) == 1, float('-inf'), 0.0)
mask = Tensor(mask_data)
S_masked = S_scaled + mask
P = S_masked.softmax()

print(np.round(P.data, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(P.data, 6).tolist() == [[1.0, 0.0, 0.0, 0.0], [0.182426, 0.817574, 0.0, 0.0], [0.048611, 0.592201, 0.359188, 0.0], [0.217951, 0.085128, 0.142506, 0.554415]], f"P.data sai -- dang ra {np.round(P.data, 6).tolist()}"
assert np.allclose(np.sum(P.data, axis=-1), 1.0), f"moi hang cua P.data phai cong dung 1.0 -- dang ra {np.sum(P.data, axis=-1).tolist()}"
assert not np.any(np.isnan(P.data)), "P.data khong duoc chua nan -- kiem tra cach dung mask (np.where, khong phai phep nhan)"

# rieng kiem tra CAC VI TRI BI CAM that su la 0 (khong chi gan dung, ma DUNG BANG 0)
assert P.data[0, 1] == 0.0 and P.data[0, 2] == 0.0 and P.data[0, 3] == 0.0, f"hang 0 phai co dung 3 vi tri bi cam bang 0 -- dang ra {P.data[0].tolist()}"
assert P.data[1, 2] == 0.0 and P.data[1, 3] == 0.0, f"hang 1 phai co dung 2 vi tri bi cam bang 0 -- dang ra {P.data[1].tolist()}"
assert P.data[2, 3] == 0.0, f"hang 2 phai co dung 1 vi tri bi cam bang 0 -- dang ra {P.data[2].tolist()}"

# rieng kiem tra hang CUOI KHONG DOI so voi khong mask (khong co tuong lai nao de che)
P_khong_mask = S_scaled.softmax()
assert np.allclose(P.data[3], P_khong_mask.data[3]), f"hang cuoi phai GIONG HET truoc/sau mask -- dang ra {P.data[3].tolist()} va {P_khong_mask.data[3].tolist()}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là giá trị gán cho vị trí BỊ CẤM trong `np.where` — `float('-inf')` (một số ÂM VÔ CỰC THẬT, không phải một số cực âm hữu hạn tuỳ ý). Chỗ hai áp `mask` vào `S_scaled` bằng phép cộng ĐÃ CÓ SẴN — `mask` (biến `Tensor` vừa dựng ở dòng trên), KHÔNG phải `mask_data` (mảng `numpy` thô, chưa bọc `Tensor`).
- kind: strategy
  body: "Chỗ đầu: `float('-inf')`. Chỗ hai: `mask`."
- kind: one-line
  body: "Hai chỗ trống lần lượt là `float('-inf')` và `mask`."
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: gia tri gan cho vi tri bi cam trong np.where phai la float('-inf') (khong phai mot so hang chep tay khac); S_masked phai cong VOI DUNG bien Tensor "mask" (khong phai mask_data tho chua boc Tensor)
  requireAst:
  - kind: uses-call, target: float, min: 1
  - kind: uses-name, target: mask, min: 1
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # class Tensor + harness): loi giai dung dat=true. float=1 (chi trong
  # blank1 "float('-inf')" -- khong co loi goi float() nao khac trong file).
  # mask=1 (dinh nghia "mask = Tensor(mask_data)" la Store cho ten "mask",
  # khong dem; Load duy nhat la trong blank2 "S_scaled + mask").
  # KHONG dung has-literal cho "-inf": day la mot Call (float('-inf')),
  # khong phai Constant so, nen has-literal khong ap dung duoc du sao --
  # dung uses-call target float thay the.
  # Cheat "np.where(..., -1e9, 0.0)" (so cuc am HUU HAN thay vi -inf that)
  # lam "float" tut xuong 0 -- bi chan RIENG. Rieng ve mat GIA TRI, -1e9 sau
  # khi scale/softmax van cho e^(rat am) xap xi 0 -- co the VAN qua duoc
  # cac assert gia tri o day (vi -1e9 du am de lam trong so xap xi 0 trong
  # pham vi lam tron 6 chu so) -- day la ly do luat static PHAI bat rieng
  # qua ten ham "float", khong chi dua vao tier tests.
  # Cheat "S_masked = S_scaled + Tensor(mask_data)" (bo qua bien mask, tao
  # Tensor moi tu mask_data ngay tai cho) GIU gia tri KET QUA dung (vi
  # Tensor(mask_data) cho cung du lieu voi mask) nen KHONG bi bat boi test
  # gia tri -- nhung lam "mask" tut xuong duoi 2 (chi con 0 hoac 1 Load tuy
  # cach viet) -- bi chan boi luat static (day la mot cheat "khong sai gia
  # tri nhung di vong qua bien da dat ten", van bi chan de giu dung Y cua
  # bai la TAI SU DUNG bien mask da dung, khong phai tao lai tu dau).
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^\\[\\[1\\.0, 0\\.0, 0\\.0, 0\\.0\\], \\[0\\.182426, 0\\.817574, 0\\.0, 0\\.0\\], \\[0\\.048611, 0\\.592201, 0\\.359188, 0\\.0\\], \\[0\\.217951, 0\\.085128, 0\\.142506, 0\\.554415\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tương lai đã bị che — mỗi vị trí chỉ nhìn thấy chính nó và quá khứ. Bài sau: chạy NHIỀU attention song song, mỗi cái nhìn một không gian con khác nhau — multi-head.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Causal mask vừa xây hoạt động trên MỘT ma trận điểm attention `(so_token, so_token)` — một "đầu" attention duy nhất. Multi-head (bài sau) chạy NHIỀU đầu song song, mỗi đầu tính điểm attention TRÊN KHÔNG GIAN CON RIÊNG của nó (một phần chiều của `Q`/`K`/`V`). Causal mask (ràng buộc "không nhìn tương lai") có cần đổi gì theo từng đầu, hay áp dụng GIỐNG HỆT nhau cho MỌI đầu?
::::

::::checkpoint{mastery=0.85}
::::
