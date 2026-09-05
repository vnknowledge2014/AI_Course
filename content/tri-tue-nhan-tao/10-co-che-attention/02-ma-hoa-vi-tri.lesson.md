---
id: tri-tue-nhan-tao.co-che-attention.ma-hoa-vi-tri
title: "Positional encoding: bơm vị trí vào embedding"
summary: "Attention (bài sau) so mọi cặp vị trí độc lập -- tự nó KHÔNG biết token nào đứng trước, token nào đứng sau. Positional encoding cộng thêm một vector VỊ TRÍ cố định (công thức Transformer gốc: PE(pos,2i)=sin(pos/10000^(2i/dim)), PE(pos,2i+1)=cos(...)), qua Tensor.__add__ có sẵn (element-wise, cùng shape). Trên câu 'A B C' và 'C B A' (cùng 3 token, khác thứ tự): embedding của token 'A' CỘNG vị trí 0 khác hẳn embedding của CHÍNH token 'A' đó cộng vị trí 2 -- cùng một hàng bảng embedding, khác vector cuối cùng, đo bằng numpy thật."
locale: vi
track: tri-tue-nhan-tao
module: co-che-attention
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.ma-hoa-vi-tri]
requires: [ai.tra-cuu-embedding]
concepts: [ai.ma-hoa-vi-tri]
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
Bảng embedding tra ra một vector cho MỖI token — nhưng phép tra đó độc lập hoàn toàn với việc token đứng ở vị trí nào trong câu. Attention (bài sau) so sánh MỌI cặp vị trí — nếu không ai nói cho nó biết thứ tự, nó không thể tự đoán ra.
::::

::::explain{#ma_hoa_vi_tri}
`embedding_lookup` (bài trước) tra ra một vector CHO MỖI token, chỉ dựa vào ID của nó — hai câu khác nhau nhưng có CÙNG một token ở đâu đó sẽ tra ra ĐÚNG cùng một hàng, bất kể token đó đứng ở vị trí thứ mấy. Bản thân phép tra cứu này KHÔNG biết gì về VỊ TRÍ — và attention (cơ chế trung tâm của Transformer, quest sau cài) tính điểm liên quan giữa MỌI cặp vị trí bằng một phép nhân ma trận đối xứng theo cấu trúc — nếu không có gì phân biệt "vị trí 0" với "vị trí 2", đảo ngược thứ tự token trong câu sẽ cho ra kết quả GIỐNG HỆT, chỉ khác thứ tự sắp xếp — sai với thực tế ngôn ngữ, nơi thứ tự từ đổi hẳn nghĩa câu.

**Positional encoding** giải quyết đúng vấn đề này: cộng thêm một vector VỊ TRÍ cố định (không học được, tính sẵn bằng công thức) vào embedding, để mỗi vị trí có một "dấu vân tay" số học riêng. Công thức của Transformer gốc, dùng sin/cos xen kẽ theo từng cặp chiều:

> `PE(pos, 2i) = sin(pos / 10000^(2i/dim))`
>
> `PE(pos, 2i+1) = cos(pos / 10000^(2i/dim))`

`pos` là vị trí trong câu (`0`, `1`, `2`, ...), `i` chạy qua các CẶP chiều (`2i` và `2i+1` là hai chiều liên tiếp, cùng dùng một tần số). Chiều đầu tiên (`i = 0`) có số chia `10000^0 = 1` — góc bằng ĐÚNG `pos`, đổi nhanh theo từng vị trí. Chiều CÀNG VỀ SAU (`i` càng lớn), số chia CÀNG LỚN — góc đổi CÀNG CHẬM theo `pos`. Kết quả: mỗi vị trí có một vector `PE` riêng biệt, và các chiều khác nhau "dao động" ở tần số khác nhau — không có hai vị trí nào (trong phạm vi hợp lý) cho ra ĐÚNG cùng một vector.

Cộng `PE` vào embedding chỉ cần `Tensor.__add__` ĐÃ CÓ SẴN (`lop-tensor-co-ban`, q8.3b) — element-wise, hai `Tensor` CÙNG shape `(so_token, dim)`. Không cần công thức đạo hàm MỚI: `__add__` đã đúng, và `PE` là một hằng số (không học được) nên nó không cần `.grad` nào lan về.
::::

::::example{#pe_cong_thuc_that}
Tính `PE` bằng `numpy` thật cho `4` vị trí, `dim = 4` (`2` cặp chiều: `i = 0` dùng số chia `1`, `i = 1` dùng số chia `10000^(2/4) = 100`):

```python title=readonly
import numpy as np


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = np.sin(goc[:, 0::2])
    pe[:, 1::2] = np.cos(goc[:, 1::2])
    return pe


PE = ma_hoa_vi_tri(4, 4)
print(np.round(PE, 6).tolist())
```

```text title=readonly
[[0.0, 1.0, 0.0, 1.0], [0.841471, 0.540302, 0.01, 0.99995], [0.909297, -0.416147, 0.019999, 0.9998], [0.14112, -0.989992, 0.029996, 0.99955]]
```

Vị trí `0` cho vector đặc biệt `[0, 1, 0, 1]` (`sin(0) = 0`, `cos(0) = 1`, ở CẢ HAI cặp chiều — vì góc luôn là `0/số_chia = 0` bất kể số chia là gì). Cột đầu (`sin(pos)`, số chia `1`) đổi RẤT NHANH qua `4` vị trí — từ `0` lên `0,841471` rồi `0,909297` rồi lại giảm về `0,14112`. Cột thứ ba (`sin(pos/100)`, số chia `100`) đổi RẤT CHẬM — chỉ từ `0` lên `0,01` rồi `0,019999` rồi `0,029996`, gần như tuyến tính trong phạm vi `4` vị trí này. Hai cột dao động ở hai TẦN SỐ khác hẳn nhau, đúng như công thức thiết kế.
::::

::::example{#pe_thu_tu_quan_trong}
Hai câu CÙNG `3` token (`A`, `B`, `C`) nhưng khác THỨ TỰ: câu `1` là "A B C", câu `2` là "C B A". Cộng `PE` vào embedding của mỗi câu:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def embedding_lookup(self, ids):
        out = Tensor(self.data[ids], (self,), 'embedding_lookup')
        def _backward():
            for vi_tri, tid in enumerate(ids):
                self.grad[tid] += out.grad[vi_tri]
        out._backward = _backward
        return out

    def __add__(self, other):
        out = Tensor(self.data + other.data, (self, other), '+')
        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = np.sin(goc[:, 0::2])
    pe[:, 1::2] = np.cos(goc[:, 1::2])
    return pe


Bang = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.]])   # A=0, B=1, C=2
PE = Tensor(ma_hoa_vi_tri(3, 4))

cau_1 = [0, 1, 2]   # "A B C"
cau_2 = [2, 1, 0]   # "C B A"

X1 = Bang.embedding_lookup(cau_1) + PE
X2 = Bang.embedding_lookup(cau_2) + PE

print(np.round(X1.data, 6).tolist())
print(np.round(X2.data, 6).tolist())
print(np.round(X1.data[0], 6).tolist())    # token A, o VI TRI 0 trong cau 1
print(np.round(X2.data[2], 6).tolist())    # token A, o VI TRI 2 trong cau 2
```

```text title=readonly
[[1.0, 1.0, 1.0, 1.0], [0.841471, 1.540302, 0.01, 1.99995], [1.909297, 0.583853, 0.019999, 0.9998]]
[[1.0, 2.0, 0.0, 1.0], [0.841471, 1.540302, 0.01, 1.99995], [1.909297, -0.416147, 1.019999, 0.9998]]
[1.0, 1.0, 1.0, 1.0]
[1.909297, -0.416147, 1.019999, 0.9998]
```

Token `A` (hàng `0` của `Bang`, LUÔN cùng một vector embedding gốc) xuất hiện ở vị trí `0` trong câu `1` và vị trí `2` trong câu `2` — CÙNG một hàng embedding, nhưng SAU khi cộng `PE`, kết quả khác hẳn nhau: `[1, 1, 1, 1]` so với `[1,909297; -0,416147; 1,019999; 0,9998]`. Vị trí thay đổi ĐÃ thay đổi vector cuối cùng biểu diễn token đó — attention (bài sau) nhận được đủ thông tin để phân biệt "A đứng đầu câu" với "A đứng cuối câu", điều mà `embedding_lookup` một mình KHÔNG làm được.
::::

::::predict{#doan_tan_so_cham commitOnce}
Ví dụ đầu tính `PE` cho `4` vị trí, `dim = 4`: cột đầu (`i = 0`, số chia `1`) đổi nhanh qua các vị trí; cột thứ ba (`i = 1`, số chia `100`) đổi chậm.

**Trước khi tính**, bạn đoán: nếu tăng `dim` lên rất lớn (như `512`, kích thước thật của nhiều mô hình), CỘT CUỐI CÙNG (`i` gần `dim/2 - 1`, số chia gần `10000^1 = 10000`) sẽ đổi NHANH hay CHẬM khi `pos` tăng từ `0` đến `100`?

:::opt{correct}
Rất CHẬM — số chia của cột đó gần `10000`, làm góc `pos / 10000` chỉ tăng từ `0` lên `100/10000 = 0,01` khi `pos` chạy từ `0` đến `100`; góc thay đổi RẤT NHỎ nên `sin`/`cos` của nó gần như không đổi trong suốt khoảng đó — đúng xu hướng đã thấy ở cột thứ ba của ví dụ (`i = 1`, số chia `100`, đã đổi chậm hơn hẳn cột đầu ngay cả trên `4` vị trí)
:::

:::opt
Rất NHANH — `dim` càng lớn thì công thức càng phức tạp, và độ phức tạp đó khiến giá trị dao động mạnh hơn theo `pos`
::why
Gần đúng ở việc để ý `dim` LỚN đúng là làm công thức có NHIỀU chiều hơn, phức tạp hơn về tổng thể — quan sát về độ phức tạp không sai.

Chỗ lệch: "công thức phức tạp hơn" (nhiều chiều hơn) không có nghĩa là "MỖI chiều dao động nhanh hơn". Tốc độ dao động của một CHIỀU CỤ THỂ chỉ phụ thuộc số chia riêng của chiều đó (`10000^(2i/dim)`) — với `i` gần `dim/2 - 1`, số chia đó LUÔN gần `10000` bất kể `dim` là `4` hay `512`, nên góc vẫn đổi RẤT CHẬM, không nhanh hơn.
::
:::

:::opt
Không xác định được — tốc độ đổi của mỗi cột phụ thuộc GIÁ TRỊ CỤ THỂ của embedding, không chỉ phụ thuộc công thức `PE`
::why
Gần đúng ở việc thận trọng khi một đại lượng có vẻ phụ thuộc nhiều yếu tố — thái độ đó hợp lý với nhiều phép biến đổi khác đã học (như `layernorm`, phụ thuộc giá trị cụ thể của dữ liệu).

Chỗ lệch: `PE(pos, chieu)` là một hàm CHỈ của `pos` và chỉ số chiều — hoàn toàn không phụ thuộc bất kỳ giá trị embedding nào (khác `layernorm`, vốn tính trên DỮ LIỆU thật). Tốc độ đổi của một cột hoàn toàn xác định bởi công thức và số chia của nó, không phụ thuộc gì thêm.
::
:::
::::

::::code{#viet_ma_hoa_vi_tri}
Hoàn thiện `ma_hoa_vi_tri`: điền công thức `sin` cho các chiều CHẴN và `cos` cho các chiều LẺ.

```python title=starter
import numpy as np


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = ___     # np.sin(goc[:, 0::2])
    pe[:, 1::2] = ___     # np.cos(goc[:, 1::2])
    return pe


PE = ma_hoa_vi_tri(4, 4)
print(np.round(PE, 6).tolist())
```

```python title=solution
import numpy as np


def ma_hoa_vi_tri(so_token, dim):
    pos = np.arange(so_token)[:, None].astype(float)
    i = np.arange(dim)[None, :].astype(float)
    goc = pos / np.power(10000.0, (2 * (i // 2)) / dim)
    pe = np.zeros((so_token, dim))
    pe[:, 0::2] = np.sin(goc[:, 0::2])
    pe[:, 1::2] = np.cos(goc[:, 1::2])
    return pe


PE = ma_hoa_vi_tri(4, 4)
print(np.round(PE, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(PE, 6).tolist() == [[0.0, 1.0, 0.0, 1.0], [0.841471, 0.540302, 0.01, 0.99995], [0.909297, -0.416147, 0.019999, 0.9998], [0.14112, -0.989992, 0.029996, 0.99955]], f"PE sai -- dang ra {np.round(PE, 6).tolist()}"

# rieng kiem tra vi tri 0 la [0,1,0,1] DUNG BAT KE dim/so_chia -- sin(0)=0, cos(0)=1 luon dung
assert np.round(PE[0], 6).tolist() == [0.0, 1.0, 0.0, 1.0], f"PE[0] sai -- dang ra {np.round(PE[0], 6).tolist()}"

# rieng kiem tra CONG THUC TONG QUAT tren mot kich thuoc KHAC (so_token=2, dim=2) -- chan cheat chep san bang tren
PE_khac = ma_hoa_vi_tri(2, 2)
assert np.round(PE_khac, 6).tolist() == [[0.0, 1.0], [0.841471, 0.540302]], f"ma_hoa_vi_tri(2,2) sai -- dang ra {np.round(PE_khac, 6).tolist()}"

# rieng kiem tra hai cot dao dong o hai TAN SO khac nhau (cot dau nhanh hon cot ba, tren du lieu bai)
doi_cot_dau = abs(PE[1, 0] - PE[0, 0])
doi_cot_ba = abs(PE[1, 2] - PE[0, 2])
assert doi_cot_dau > doi_cot_ba, f"cot dau (tan so cao) phai doi NHANH HON cot ba (tan so thap) -- dang ra {doi_cot_dau} va {doi_cot_ba}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu điền cho các CHIỀU CHẴN (`pe[:, 0::2]`, ứng với công thức `PE(pos, 2i)`) — dùng `np.sin` trên góc tương ứng, `np.sin(goc[:, 0::2])`. Chỗ hai điền cho các CHIỀU LẺ (`pe[:, 1::2]`, ứng với `PE(pos, 2i+1)`) — dùng `np.cos`, `np.cos(goc[:, 1::2])`. Đừng lẫn `sin`/`cos` giữa hai chỗ.
- kind: strategy
  body: 'Chỗ đầu: `np.sin(goc[:, 0::2])`. Chỗ hai: `np.cos(goc[:, 1::2])`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `np.sin(goc[:, 0::2])` và `np.cos(goc[:, 1::2])`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: chieu CHAN (pe[:, 0::2]) phai dung np.sin, chieu LE (pe[:, 1::2]) phai dung np.cos -- ca hai deu tren bien goc, khong duoc hoan doi hay chep san mot mang hang so
  requireAst:
  - kind: uses-call, target: sin, min: 1
  - kind: uses-call, target: cos, min: 1
  - kind: uses-name, target: goc, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true. sin=1 (chi trong blank1). cos=1 (chi trong blank2). goc=2
  # (mot lan Load trong moi blank -- "goc" duoc GAN o dong "goc = pos /
  # ..." la Store, khong dem; hai lan CON LAI la hai blank). Cheat "pe[:,
  # 0::2] = np.cos(goc[:, 0::2])" (hoan doi sin/cos) lam "sin" tut xuong 0
  # -- bi chan RIENG, VA da tu kiem chung bang Python that: PE[0] khi do la
  # [1,1,1,1] thay vi [0,1,0,1] dung (cos(0)=1 o ca hai vi tri thay vi
  # sin(0)=0 o vi tri chan) -- bi bat DOC LAP boi assert PE[0]. Cheat "pe[:,
  # 0::2] = np.sin(pos[:, 0::2])" (dung pos thay vi goc, bo qua so chia
  # 10000^...) lam "goc" tut xuong 1 -- bi chan RIENG, va da tu kiem chung:
  # ket qua PE[1,0] khac han 0,841471 dung (vi thieu phep chia).
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[\\[0\\.0, 1\\.0, 0\\.0, 1\\.0\\], \\[0\\.841471, 0\\.540302, 0\\.01, 0\\.99995\\], \\[0\\.909297, -0\\.416147, 0\\.019999, 0\\.9998\\], \\[0\\.14112, -0\\.989992, 0\\.029996, 0\\.99955\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Embedding giờ đã mang cả VỊ TRÍ — cùng một token, khác chỗ đứng, cho ra vector khác nhau. Bài sau: ba phép chiếu tuyến tính lấy vector này làm gì — Query, Key, Value.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Positional encoding cộng THẲNG một vector vị trí cố định vào embedding — nó KHÔNG học được, không có tham số nào để huấn luyện. Attention (các bài sau) cần so sánh MỌI cặp vị trí bằng một phép NHÂN MA TRẬN — nhưng phép nhân đó không chạy trực tiếp trên embedding-đã-cộng-vị-trí (`X`) mà cần BA phép CHIẾU riêng biệt từ `X`. Ba phép chiếu đó dùng phép toán `Tensor` nào đã có sẵn từ q8.3b, và tại sao lại cần TỚI BA (không phải một)?
::::

::::checkpoint{mastery=0.8}
::::
