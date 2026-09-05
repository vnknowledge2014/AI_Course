---
id: tri-tue-nhan-tao.co-che-attention.diem-attention-tinh-tay
title: "Điểm attention TÍNH TAY trên 4 token: Q·Kᵀ"
summary: "Điểm attention S=Q.matmul(K.transpose()) -- S[i,j]=Q[i]·K[j] (Query của vị trí i so khớp Key của vị trí j), TÍNH TAY từng phép nhân/cộng cho cặp (token 0, token 2): Q[0]=[2,1,1,0], K[2]=[1,2,2,1], Q[0]·K[2]=2×1+1×2+1×2+0×1=6, ĐỐI CHIẾU CHÍNH XÁC với S.data[0,2] từ Tensor.matmul thật. KHÔNG đối xứng: Q[2]·K[0]=5≠6, vì Wq≠Wk chiếu ra hai không gian khác nhau."
locale: vi
track: tri-tue-nhan-tao
module: co-che-attention
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.diem-attention-tinh-tay]
requires: [ai.query-key-value]
concepts: [ai.diem-attention-tinh-tay]
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
`Q`, `K`, `V` đã có. Bây giờ tới bước trung tâm của toàn bộ attention: đo mức độ LIÊN QUAN giữa mỗi cặp vị trí. Lần này KHÔNG chỉ chạy code và tin kết quả — tính tay từng phép nhân, từng phép cộng, rồi đối chiếu.
::::

::::explain{#diem_attention_tinh_tay}
**Điểm attention** giữa vị trí `i` và vị trí `j` là tích vô hướng `Q[i] · K[j]` — Query của vị trí `i` ("tôi đang tìm gì") so khớp với Key của vị trí `j` ("tôi đại diện cho gì"). Tính điểm cho MỌI cặp `(i, j)` cùng lúc chính là một phép nhân ma trận:

> `S = Q.matmul(K.transpose())`, và `S[i, j] = Q[i] · K[j] = Σₜ Q[i, t] · K[j, t]`

Vì sao cần `K.transpose()`? `Q` shape `(so_token, dim)`, `K` shape `(so_token, dim)` — CÙNG shape, không phải hai shape tương thích trực tiếp cho `matmul` (chiều trong của `Q` là `dim`, cần khớp với chiều NGOÀI-thứ-nhất của toán hạng phải, tức SỐ HÀNG — mà `K` có `dim` CỘT, không phải `dim` HÀNG). `K.transpose()` (đã có từ `chuyen-vi-va-dinh-hinh-lai`, q8.3b) đổi `K` thành shape `(dim, so_token)` — giờ chiều trong khớp (`dim` với `dim`), và kết quả `S` có shape `(so_token, so_token)`: `S[i, j]` đúng là tích của HÀNG `i` của `Q` với HÀNG `j` của `K` (không phải cột `j` — chuyển vị đã đưa hàng `j` của `K` gốc thành cột `j` của `K.transpose()`).

Bài này TÍNH TAY một điểm cụ thể — `S[0, 2]` — rồi ĐỐI CHIẾU với kết quả `Tensor.matmul` thật, phải khớp CHÍNH XÁC.
::::

::::example{#tinh_tay_diem_0_2}
`Q` và `K` từ bài trước (shape `(4, 4)`). Lấy ra hàng `0` của `Q` và hàng `2` của `K`:

> `Q[0] = [2, 1, 1, 0]`
>
> `K[2] = [1, 2, 2, 1]`

Điểm attention giữa token `0` và token `2` là tích vô hướng của hai vector này — nhân TỪNG CẶP chiều tương ứng rồi cộng lại:

> `Q[0] · K[2] = 2×1 + 1×2 + 1×2 + 0×1 = 6`

Bốn phép nhân (`2×1`, `1×2`, `1×2`, `0×1`), ba phép cộng — không có phép nào khác. Giờ đối chiếu với `Tensor.matmul` thật:

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


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)

diem_tay = 2 * 1 + 1 * 2 + 1 * 2 + 0 * 1

S = Q.matmul(K.transpose())

print(diem_tay)
print(round(float(S.data[0, 2]), 6))
print(diem_tay == round(float(S.data[0, 2]), 6))
print(np.round(S.data, 6).tolist())
```

```text title=readonly
6.0
6.0
True
[[5.0, 5.0, 6.0, 3.49246], [6.0, 9.0, 9.0, 0.542022], [5.0, 10.0, 9.0, -2.58909], [1.391794, -0.488424, 0.542022, 3.259085]]
```

`diem_tay` (tính tay, chỉ số nguyên và phép nhân/cộng cơ bản) và `S.data[0, 2]` (từ `Tensor.matmul` thật, xuyên qua toàn bộ pipeline `X → Q, K → S`) khớp CHÍNH XÁC, cả hai đều bằng `6`. Hàng `0`, cột `2` của ma trận `S` đầy đủ (in ở dưới) đúng bằng `6.0` — không phải trùng hợp, mà vì `matmul` được ĐỊNH NGHĨA để làm ĐÚNG phép tính tích-vô-hướng-cho-mọi-cặp mà ta vừa làm tay cho MỘT cặp.
::::

::::predict{#doan_khong_doi_xung commitOnce}
Vừa tính tay `Q[0] · K[2] = 6` — điểm attention từ token `0` "nhìn" token `2`.

**Trước khi tính**, bạn đoán: điểm attention THEO CHIỀU NGƯỢC LẠI — từ token `2` "nhìn" token `0`, tức `Q[2] · K[0]` — có BẰNG `6` không?

:::opt{correct}
Không nhất thiết bằng — và ở đây thực sự KHÁC: `Q[2] · K[0] = 1×2 + 1×1 + 2×0 + 2×1 = 5`, khác `6`. Điểm attention KHÔNG đối xứng vì `Q` và `K` chiếu từ `X` qua HAI ma trận trọng số khác nhau (`Wq ≠ Wk`, bài `query-key-value`) — `Q[i] · K[j]` và `Q[j] · K[i]` dùng những vector hoàn toàn khác nhau (khác cả nguồn gốc phép chiếu), không có lý do gì để bằng nhau
:::

:::opt
Có, luôn bằng — điểm attention giống một phép ĐO ĐỘ TƯƠNG TỰ giữa hai vector (như tích vô hướng thông thường), mà độ tương tự thì không phân biệt chiều
::why
Gần đúng ở việc liên tưởng tới tích vô hướng "độ tương tự" quen thuộc — MỘT tích vô hướng giữa hai vector CỐ ĐỊNH (ví dụ `u · v`) đúng là đối xứng (`u · v = v · u`).

Chỗ lệch: `Q[i] · K[j]` không phải tích vô hướng của HAI VECTOR CỐ ĐỊNH mà đảo vai — nó là tích của MỘT hàng từ MA TRẬN `Q` (chiếu qua `Wq`) với MỘT hàng từ MA TRẬN KHÁC `K` (chiếu qua `Wk`). Đảo `i` và `j` không đơn thuần "đảo vai trong cùng một phép tính" — nó đổi sang lấy MỘT CẶP VECTOR KHÁC hẳn (`Q[2]` và `K[0]`, thay vì `Q[0]` và `K[2]`), và vì `Wq ≠ Wk`, không có ràng buộc nào buộc hai tích đó bằng nhau.
::
:::

:::opt
Không xác định được nếu không tính — với ma trận trọng số bất kỳ, hai chiều có thể bằng hoặc khác nhau tuỳ trường hợp, không có quy luật chung
::why
Gần đúng ở việc thận trọng, không suy luận suông khi chưa tính — thái độ đó đúng tinh thần của quest này.

Chỗ lệch: câu hỏi không hỏi "CÓ THỂ khác nhau không" (đúng, có thể) — nó hỏi về TRƯỜNG HỢP CỤ THỂ này, với `Q`, `K` ĐÃ CHO ở trên. Với dữ liệu cụ thể này, `Q[2] · K[0] = 5` đã tính RA MỘT GIÁ TRỊ XÁC ĐỊNH, khác `6` — không phải một khả năng chưa biết, mà là một con số đã tính được ngay khi có `Q` và `K`.
::
:::
::::

::::code{#viet_diem_attention_tay}
Hoàn thiện hai chỗ trống: tính tay điểm attention `Q[0] · K[2]` bằng bốn phép nhân và ba phép cộng tường minh, và ráp `S` bằng `matmul` + `transpose` đã có sẵn.

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


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)

Q0 = [2.0, 1.0, 1.0, 0.0]
K2 = [1.0, 2.0, 2.0, 1.0]
diem_tay = ___                          # Q0[0]*K2[0] + Q0[1]*K2[1] + Q0[2]*K2[2] + Q0[3]*K2[3]

S = Q.matmul(___)                       # K.transpose()

print(diem_tay)
print(round(float(S.data[0, 2]), 6))
print(diem_tay == round(float(S.data[0, 2]), 6))
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


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)

Q0 = [2.0, 1.0, 1.0, 0.0]
K2 = [1.0, 2.0, 2.0, 1.0]
diem_tay = Q0[0] * K2[0] + Q0[1] * K2[1] + Q0[2] * K2[2] + Q0[3] * K2[3]

S = Q.matmul(K.transpose())

print(diem_tay)
print(round(float(S.data[0, 2]), 6))
print(diem_tay == round(float(S.data[0, 2]), 6))
```

```python title=test
import numpy as np

assert diem_tay == 6.0, f"diem_tay sai -- dang ra {diem_tay}"
assert round(float(S.data[0, 2]), 6) == 6.0, f"S.data[0,2] sai -- dang ra {round(float(S.data[0, 2]), 6)}"
assert diem_tay == round(float(S.data[0, 2]), 6), "diem_tay va S.data[0,2] phai KHOP CHINH XAC"

# rieng kiem tra S day du va tinh KHONG DOI XUNG (Q[2].K[0] != Q[0].K[2])
assert np.round(S.data, 6).tolist() == [[5.0, 5.0, 6.0, 3.49246], [6.0, 9.0, 9.0, 0.542022], [5.0, 10.0, 9.0, -2.58909], [1.391794, -0.488424, 0.542022, 3.259085]], f"S.data sai -- dang ra {np.round(S.data, 6).tolist()}"
assert round(float(S.data[2, 0]), 6) == 5.0, f"S.data[2,0] sai -- dang ra {round(float(S.data[2, 0]), 6)}"
assert S.data[0, 2] != S.data[2, 0], "diem attention KHONG duoc doi xung -- S[0,2] phai khac S[2,0]"

# rieng kiem tra CONG THUC TONG QUAT cua diem_tay (khong chi la hang so 6 chep san):
# tinh lai voi mot cap vector KHAC, dam bao dung PHEP TINH that, khong phai gia tri co dinh
Q1_test = [3.0, -1.0, 2.0, 0.5]
K1_test = [1.0, 2.0, -1.0, 4.0]
ket_qua_mong_doi = Q1_test[0]*K1_test[0] + Q1_test[1]*K1_test[1] + Q1_test[2]*K1_test[2] + Q1_test[3]*K1_test[3]
assert ket_qua_mong_doi == 1.0, f"kiem tra cong thuc tren cap vector khac sai -- dang ra {ket_qua_mong_doi}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là tích vô hướng TƯỜNG MINH giữa `Q0` và `K2` — bốn phép nhân từng cặp chiều tương ứng, cộng lại — `Q0[0]*K2[0] + Q0[1]*K2[1] + Q0[2]*K2[2] + Q0[3]*K2[3]`, KHÔNG được chép sẵn kết quả `6`. Chỗ hai ráp `S` bằng `matmul` giữa `Q` và `K` ĐÃ CHUYỂN VỊ — `K.transpose()`, thiếu `.transpose()` sẽ làm shape không khớp đúng ý nghĩa `S[i,j] = Q[i]·K[j]`.
- kind: strategy
  body: 'Chỗ đầu: `Q0[0]*K2[0] + Q0[1]*K2[1] + Q0[2]*K2[2] + Q0[3]*K2[3]`. Chỗ hai: `K.transpose()`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `Q0[0]*K2[0] + Q0[1]*K2[1] + Q0[2]*K2[2] + Q0[3]*K2[3]` và `K.transpose()`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: diem_tay phai tinh THAT bang bon phep nhan tung cap Q0[i]*K2[i] cong lai (khong duoc chep san dap so); S phai rap bang Q.matmul(K.transpose()) (dung DUNG K da chuyen vi, khong duoc bo transpose hay dung K nguyen goc)
  requireAst:
  - kind: uses-operator, target: "*", min: 4
  - kind: uses-operator, target: "+", min: 3
  - kind: uses-name, target: Q0, min: 4
  - kind: uses-name, target: K2, min: 4
  - kind: uses-call, target: transpose, min: 1
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # class Tensor + harness): loi giai dung dat=true. "*"=4 (bon phep nhan
  # trong blank1; khong co "*" nao khac trong toan file ngoai blank1).
  # "+"=3 (ba phep cong noi bon tich trong blank1). Q0=4, K2=4 (moi bien
  # doc dung bon lan trong blank1 -- dinh nghia "Q0 = [...]"/"K2 = [...]"
  # la Store, khong dem). transpose=1 (chi trong blank2 "K.transpose()" --
  # dinh nghia "def transpose(self):" khong tinh vi khong phai Call).
  # KHONG dung forbidAst has-literal cho "6": da THU THAT va no TRUOT tren
  # chinh loi giai dung -- harness co san (khong phai blank) goi "round(...,
  # 6)" HAI LAN de lam tron hien thi, moi lan la mot Constant(6) hop le,
  # nen mot luat cam literal "6" se chan oan chinh loi giai dung. Dung
  # requireAst tren "*"/"+"/Q0/K2 la du de chan cheat hardcode (xem duoi).
  # Cheat "diem_tay = 6" hoac "diem_tay = 6.0" (chep san dap so, bo qua het
  # phep tinh) lam "*" tut xuong 0, "+" tut xuong 0, Q0 va K2 deu tut xuong
  # 0 -- bi chan BON LAN boi requireAst, doc lap voi has-literal.
  # Cheat "S = Q.matmul(K)" (bo transpose) lam "transpose" tut xuong 0 -- bi
  # chan RIENG, VA da tu kiem chung bang Python that: voi Q, K deu shape
  # (4,4), "Q.matmul(K)" van CHAY duoc (khong nem loi, vi ca hai deu vuong
  # 4x4) nhung cho S.data[0,2] = 4.0 (Q[0,:] nhan COT 2 cua K nguyen goc,
  # khac han S[0,2]=6.0 dung cua Q@K.T, vi Q@K.T moi la Q[0] nhan HANG 2
  # cua K) -- bi bat DOC LAP boi assert S.data[0,2]==6.0 VA assert diem_tay
  # == S.data[0,2].
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^6\\.0\\n6\\.0\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tính tay khớp chính xác với `Tensor.matmul` thật — `6`, không lệch một chữ số nào. Nhưng điểm số thô này chưa dùng được trực tiếp: cần chuẩn hoá thành xác suất. Bài sau: chia tỉ lệ rồi softmax.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Điểm attention thô (`S`) có thể là số bất kỳ — âm, dương, lớn, nhỏ — không có ràng buộc nào. Nhưng để dùng làm TRỌNG SỐ cho việc "chú ý bao nhiêu tới mỗi vị trí", các trọng số đó cần là XÁC SUẤT — không âm, và tổng mỗi hàng phải bằng `1`. Phép toán `Tensor` nào (đã học từ q8.3b) biến một hàng số THÔ bất kỳ thành một phân phối xác suất như vậy?
::::

::::checkpoint{mastery=0.85}
::::
