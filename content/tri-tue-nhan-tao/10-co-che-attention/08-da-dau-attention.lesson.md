---
id: tri-tue-nhan-tao.co-che-attention.da-dau-attention
title: "Multi-head attention: nhiều đầu chú ý song song"
summary: "Multi-head: tách Q/K/V (so_token,dim) thành so_head đầu nhỏ hơn (so_token,dim/so_head) bằng Tensor.reshape TÁI DÙNG (q8.3b), tính attention (scale+mask+softmax+matmul V) ĐỘC LẬP trên mỗi đầu, rồi NỐI lại bằng np.concatenate. Trên Q,K,V (4,4) tách thành 2 đầu (dim 2 mỗi đầu): đầu 0 và đầu 1 cho hai ma trận điểm KHÁC HẲN nhau (mỗi đầu chỉ thấy MỘT nửa chiều), kết quả nối lại đúng shape (4,4) -- mỗi đầu có thể học một kiểu quan hệ riêng."
locale: vi
track: tri-tue-nhan-tao
module: co-che-attention
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.da-dau-attention]
requires: [ai.mat-na-nhan-qua]
concepts: [ai.da-dau-attention]
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
Một tầng attention (bảy bài trước) chỉ có MỘT "cách nhìn" — một bộ `Q`/`K`/`V`, một ma trận điểm số. Nếu chạy NHIỀU cách nhìn song song, mỗi cách chỉ thấy một PHẦN các chiều, có khi mỗi đầu học được một kiểu quan hệ khác nhau.
::::

::::explain{#da_dau_attention}
**Multi-head attention** chạy NHIỀU tầng attention nhỏ SONG SONG, mỗi tầng ("đầu") chỉ hoạt động trên MỘT PHẦN các chiều của `Q`/`K`/`V`, rồi NỐI (concatenate) kết quả của tất cả các đầu lại thành một đầu ra CÓ CÙNG shape với đầu vào.

Ba bước, dùng ĐÚNG các phép toán đã có:

> **Tách** — `Q`, `K`, `V` (mỗi cái shape `(so_token, dim)`) tách thành `so_head` đầu, mỗi đầu shape `(so_token, dim/so_head)`. Dùng `Tensor.reshape` ĐÃ CÓ SẴN (`chuyen-vi-va-dinh-hinh-lai`, q8.3b): reshape `(so_token, dim)` thành `(so_token, so_head, dim/so_head)` — vì `numpy` lưu dữ liệu theo THỨ TỰ HÀNG, việc này tách CHÍNH XÁC các chiều LIÊN TIẾP vào từng đầu (đầu `0` lấy các chiều `0..dim/so_head-1`, đầu `1` lấy các chiều TIẾP THEO, ...) mà KHÔNG xáo trộn thứ tự token nào. Sau `reshape`, mỗi đầu được TÁCH RIÊNG ra thành một `Tensor` `(so_token, dim/so_head)` bằng cách đọc trực tiếp lát cắt tương ứng của mảng đã reshape.
>
> **Tính attention độc lập trên mỗi đầu** — mỗi đầu chạy TRỌN VẸN pipeline đã học (scale, causal mask, softmax, nhân với phần `V` tương ứng của chính đầu đó) — hoàn toàn TÁCH BIỆT với các đầu khác, không chia sẻ điểm số hay trọng số nào.
>
> **Nối lại** — ghép đầu ra của `so_head` đầu (mỗi cái `(so_token, dim/so_head)`) theo TRỤC CHIỀU CUỐI, dùng `np.concatenate`, cho lại đúng shape gốc `(so_token, dim)`.

Trực giác: mỗi đầu chỉ "nhìn" một PHẦN không gian biểu diễn — đầu này có thể học quan hệ cú pháp (từ nào bổ nghĩa cho từ nào), đầu khác có thể học quan hệ chủ đề (từ nào cùng nhắc tới một ý), vì mỗi đầu tính điểm attention TRÊN MỘT TẬP CHIỀU KHÁC NHAU của `Q`/`K` — không có gì ép hai đầu phải học CÙNG một kiểu quan hệ. Bài này CHỈ demo phần FORWARD (tách, tính, nối) — thao tác trực tiếp trên `.data` của mỗi đầu để tách/nối, giống cách `nhan-ma-tran-forward` (q8.3b) demo `matmul` CHỈ forward trước khi có backward.
::::

::::example{#multi_head_that}
`Q`, `K`, `V` shape `(4, 4)` (từ các bài trước) — tách thành `2` đầu, mỗi đầu `2` chiều, tính attention (scale + causal mask + softmax + nhân `V`) ĐỘC LẬP trên từng đầu, rồi nối lại:

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

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
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


def tach_dau(T, so_dau):
    n_tok, d = T.data.shape
    dd = d // so_dau
    T3 = T.reshape((n_tok, so_dau, dd))
    return [Tensor(T3.data[:, h, :]) for h in range(so_dau)]


def ghep_dau(danh_sach_dau):
    return Tensor(np.concatenate([t.data for t in danh_sach_dau], axis=-1))


X = Tensor([[1., 0., 1., 0.], [0., 1., 1., 1.], [1., 1., 0., 1.], [1.14112, -1.989992, 1.029996, -0.00045]])
Wq = Tensor([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 1., 0., 0.], [0., 0., 1., 1.]])
Wk = Tensor([[1., 1., 0., 0.], [0., 0., 1., 1.], [1., 0., 0., 1.], [0., 1., 1., 0.]])
Wv = Tensor([[1., 0., 0., 1.], [0., 1., 1., 0.], [1., 1., 0., 0.], [0., 0., 1., 1.]])

Q = X.matmul(Wq)
K = X.matmul(Wk)
V = X.matmul(Wv)

so_token, dim = Q.data.shape
so_head = 2
dim_dau = dim // so_head

so_token_ = so_token
mask = Tensor(np.where(np.triu(np.ones((so_token_, so_token_)), k=1) == 1, float('-inf'), 0.0))

Qh = tach_dau(Q, so_head)
Kh = tach_dau(K, so_head)
Vh = tach_dau(V, so_head)

dau_ra_moi_dau = []
for h in range(so_head):
    Sh = Qh[h].matmul(Kh[h].transpose())
    Sh_scaled = Tensor(Sh.data / np.sqrt(dim_dau))
    Ph = (Sh_scaled + mask).softmax()
    dau_ra_moi_dau.append(Ph.matmul(Vh[h]))
    print(f"dau {h}, S:", np.round(Sh.data, 6).tolist())

ket_qua = ghep_dau(dau_ra_moi_dau)
print(np.round(ket_qua.data, 6).tolist())
print(ket_qua.data.shape)
```

```text title=readonly
dau 0, S: [[5.0, 3.0, 4.0, 5.482902], [4.0, 3.0, 5.0, 4.452456], [3.0, 2.0, 3.0, 3.311786], [3.382236, 1.21112, 0.251124, 3.618706]]
dau 1, S: [[0.0, 2.0, 2.0, -1.990442], [2.0, 6.0, 4.0, -3.910434], [2.0, 8.0, 6.0, -5.900876], [-1.990442, -1.699544, 0.290898, -0.359621]]
[[2.0, 1.0, 0.0, 1.0], [1.669762, 1.330238, 1.888386, 1.0], [1.401112, 1.197776, 1.977146, 1.193335], [1.951166, 0.161714, 0.594117, 1.524645]]
[4, 4]
```

Điểm số của đầu `0` và đầu `1` HOÀN TOÀN KHÁC NHAU — đầu `0` chỉ thấy hai chiều ĐẦU của `Q`/`K` (`S[0,0] = 5`), đầu `1` chỉ thấy hai chiều SAU (`S[0,0] = 0`, khác hẳn) — mỗi đầu đang "nhìn" một không gian con RIÊNG, hoàn toàn có thể học ra hai kiểu quan hệ khác nhau giữa các token. Kết quả nối lại `ket_qua` có shape `(4, 4)` — ĐÚNG BẰNG shape gốc của `Q`/`K`/`V`, vì tổng số chiều của `2` đầu (`2 + 2`) luôn cộng lại đúng bằng `dim` gốc (`4`).
::::

::::predict{#doan_shape_bon_dau commitOnce}
Ví dụ trên tách thành `2` đầu (mỗi đầu `2` chiều), nối lại cho shape `(4, 4)` — đúng bằng shape gốc của `Q`.

**Trước khi tính**, bạn đoán: nếu tách thành `4` đầu (mỗi đầu chỉ còn `1` chiều duy nhất) THAY VÌ `2` đầu, shape của kết quả SAU KHI NỐI LẠI sẽ là gì?

:::opt{correct}
Vẫn `(4, 4)` — số đầu (`so_head`) và số chiều mỗi đầu (`dim/so_head`) có thể đổi tuỳ ý, nhưng TỔNG chiều sau khi nối luôn là `so_head × (dim/so_head) = dim`, không đổi; nối lại `4` đầu mỗi đầu `1` chiều cũng cho lại đúng `4` chiều, y hệt nối `2` đầu mỗi đầu `2` chiều
:::

:::opt
`(4, 8)` — nhiều đầu hơn thì càng cần nhiều KHÔNG GIAN hơn để chứa hết thông tin của tất cả các đầu, nên tổng chiều phải TĂNG lên
::why
Gần đúng ở việc "nhiều đầu hơn" nghe có vẻ như "cần nhiều chỗ hơn" — trực giác đó dễ nảy sinh khi nghĩ về việc thêm THÀNH PHẦN.

Chỗ lệch: mỗi đầu không MANG THÊM chiều mới — nó chỉ lấy một PHẦN của các chiều ĐÃ CÓ SẴN (`dim` chiều gốc, chia đều cho `so_head` đầu). Tăng số đầu làm mỗi đầu NHỎ HƠN (`dim/so_head` giảm), không làm tổng chiều tăng lên. Nối lại luôn khôi phục ĐÚNG `dim` chiều gốc, dù chia thành bao nhiêu đầu.
::
:::

:::opt
`(4, 1)` — với `4` đầu mỗi đầu chỉ `1` chiều, kết quả nối lại cũng chỉ còn `1` chiều duy nhất, giống một đầu đại diện chung
::why
Gần đúng ở việc để ý ĐÚNG rằng mỗi đầu RIÊNG LẺ giờ chỉ còn `1` chiều — quan sát về kích thước của MỘT đầu không sai.

Chỗ lệch: câu hỏi hỏi về kích thước SAU KHI NỐI LẠI CẢ `so_head` đầu, không phải kích thước của MỘT đầu riêng lẻ. `np.concatenate` ghép TẤT CẢ `so_head` đầu (mỗi đầu `1` chiều) lại THEO TRỤC CHIỀU — với `4` đầu, kết quả có `4 × 1 = 4` chiều, không phải chỉ `1`.
::
:::
::::

::::code{#viet_multi_head}
Hoàn thiện hai chỗ trống: `tach_dau` (dùng `Tensor.reshape` để tách thành `so_dau` đầu) và `ghep_dau` (dùng `np.concatenate` để nối lại).

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        return out


def tach_dau(T, so_dau):
    n_tok, d = T.data.shape
    dd = d // so_dau
    T3 = T.reshape(___)                       # (n_tok, so_dau, dd)
    return [Tensor(T3.data[:, h, :]) for h in range(so_dau)]


def ghep_dau(danh_sach_dau):
    return Tensor(___)                        # np.concatenate([t.data for t in danh_sach_dau], axis=-1)


Q = Tensor([[2., 1., 1., 0.], [1., 2., 1., 2.], [1., 1., 2., 2.], [2.171116, -0.959996, 1.14067, -1.990442]])

Qh = tach_dau(Q, 2)
print(np.round(Qh[0].data, 6).tolist())
print(np.round(Qh[1].data, 6).tolist())

ket_qua = ghep_dau(Qh)
print(np.round(ket_qua.data, 6).tolist())
print(ket_qua.data.shape)
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

    def reshape(self, shape):
        out = Tensor(self.data.reshape(shape), (self,), 'reshape')
        return out


def tach_dau(T, so_dau):
    n_tok, d = T.data.shape
    dd = d // so_dau
    T3 = T.reshape((n_tok, so_dau, dd))
    return [Tensor(T3.data[:, h, :]) for h in range(so_dau)]


def ghep_dau(danh_sach_dau):
    return Tensor(np.concatenate([t.data for t in danh_sach_dau], axis=-1))


Q = Tensor([[2., 1., 1., 0.], [1., 2., 1., 2.], [1., 1., 2., 2.], [2.171116, -0.959996, 1.14067, -1.990442]])

Qh = tach_dau(Q, 2)
print(np.round(Qh[0].data, 6).tolist())
print(np.round(Qh[1].data, 6).tolist())

ket_qua = ghep_dau(Qh)
print(np.round(ket_qua.data, 6).tolist())
print(ket_qua.data.shape)
```

```python title=test
import numpy as np

assert np.round(Qh[0].data, 6).tolist() == [[2.0, 1.0], [1.0, 2.0], [1.0, 1.0], [2.171116, -0.959996]], f"Qh[0] sai -- dang ra {np.round(Qh[0].data, 6).tolist()}"
assert np.round(Qh[1].data, 6).tolist() == [[1.0, 0.0], [1.0, 2.0], [2.0, 2.0], [1.14067, -1.990442]], f"Qh[1] sai -- dang ra {np.round(Qh[1].data, 6).tolist()}"
assert np.round(ket_qua.data, 6).tolist() == [[2.0, 1.0, 1.0, 0.0], [1.0, 2.0, 1.0, 2.0], [1.0, 1.0, 2.0, 2.0], [2.171116, -0.959996, 1.14067, -1.990442]], f"ket_qua.data sai -- dang ra {np.round(ket_qua.data, 6).tolist()}"
assert tuple(ket_qua.data.shape) == (4, 4), f"ket_qua.data.shape sai -- dang ra {tuple(ket_qua.data.shape)}"

# rieng kiem tra nối lại DUNG BANG chinh Q goc (tach roi ghep phai la phep NGHICH DAO cua nhau)
assert np.allclose(ket_qua.data, Q.data), "tach_dau roi ghep_dau phai tra ve DUNG Q ban dau"

# rieng kiem tra CONG THUC TONG QUAT: tach thanh 4 dau (khong phai 2) tren mot Tensor KHAC
Z = Tensor([[1., 2., 3., 4.], [5., 6., 7., 8.]])
Zh = tach_dau(Z, 4)
assert len(Zh) == 4, f"tach_dau(Z, 4) phai tra ve 4 dau -- dang ra {len(Zh)}"
assert np.round(Zh[2].data, 6).tolist() == [[3.0], [7.0]], f"Zh[2] sai -- dang ra {np.round(Zh[2].data, 6).tolist()}"
Z_ghep = ghep_dau(Zh)
assert tuple(Z_ghep.data.shape) == (2, 4), f"Z_ghep.data.shape sai -- dang ra {tuple(Z_ghep.data.shape)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. `tach_dau`, chỗ trống là SHAPE MỚI truyền vào `reshape` — ba chiều `(n_tok, so_dau, dd)` (số token giữ nguyên, tách chiều cuối thành `so_dau × dd`). `ghep_dau`, chỗ trống là lời gọi `np.concatenate` nối DANH SÁCH mảng `.data` của từng đầu THEO TRỤC CUỐI — `np.concatenate([t.data for t in danh_sach_dau], axis=-1)`.
- kind: strategy
  body: 'tach_dau: `(n_tok, so_dau, dd)`. ghep_dau: `np.concatenate([t.data for t in danh_sach_dau], axis=-1)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `(n_tok, so_dau, dd)` và `np.concatenate([t.data for t in danh_sach_dau], axis=-1)`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: tach_dau phai reshape THAT ve shape ba chieu (n_tok, so_dau, dd) (dung DUNG bien so_dau va dd, khong duoc chep san so nguyen co dinh); ghep_dau phai goi THAT np.concatenate tren danh sach .data cua tung dau, axis=-1
  requireAst:
  - kind: uses-name, target: so_dau, min: 3
  - kind: uses-name, target: dd, min: 1
  - kind: uses-call, target: concatenate, min: 1
  - kind: comprehension, min: 1
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # class Tensor + ham + harness): loi giai dung dat=true. so_dau=3 (tham
  # so ham "def tach_dau(T, so_dau)" la ast.arg khong dem; Load xuat hien
  # trong "d // so_dau" (co san trong starter, khong phai blank), trong
  # blank1 "(n_tok, so_dau, dd)", VA trong "range(so_dau)" o dong return
  # (co san trong starter) -- ba lan). dd=1 (dinh nghia "dd = d // so_dau"
  # la Store, khong dem; Load duy nhat la trong blank1 "(n_tok, so_dau,
  # dd)" -- KHONG co noi nao khac dung "dd").
  # concatenate=1 (chi trong blank2). comprehension=1 (list comprehension
  # "[t.data for t in danh_sach_dau]" trong blank2 -- day la ListComp, dem
  # duoc boi kind "comprehension"; rieng list comprehension CO SAN trong
  # than ham tach_dau, "[Tensor(...) for h in range(so_dau)]", cung dem
  # them 1 nua nhung khong anh huong nguong min:1).
  # Cheat "T3 = T.reshape((4, so_dau, dd))" (chep san so token co dinh la 4
  # thay vi dung bien n_tok) KHONG doi so dem so_dau/dd (van dung ca hai
  # ten) nen KHONG bi static bat rieng qua cac luat nay -- nhung da tu kiem
  # chung bang Python that: tren test rieng voi Tensor Z shape (2,4) (KHAC
  # 4 hang), cheat nay se NEM loi reshape (khong the reshape mang co 2*4=8
  # phan tu thanh shape (4, so_dau, dd) doi hoi 4*so_dau*dd phan tu) -- bi
  # chan boi tier `run` tren chinh test rieng nay (Z shape (2,4), tach_dau
  # 4 dau).
  # Cheat "np.stack(...)" thay vi "np.concatenate(...)" (nham lan hai ham)
  # lam "concatenate" tut xuong 0 -- bi chan RIENG, VA da tu kiem chung
  # bang Python that: np.stack THEM MOT TRUC MOI (cho shape (2, 4, 2) thay
  # vi (4, 4) mong doi) -- bi bat DOC LAP boi assert shape.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^\\[\\[2\\.0, 1\\.0\\], \\[1\\.0, 2\\.0\\], \\[1\\.0, 1\\.0\\], \\[2\\.171116, -0\\.959996\\]\\]\\n\\[\\[1\\.0, 0\\.0\\], \\[1\\.0, 2\\.0\\], \\[2\\.0, 2\\.0\\], \\[1\\.14067, -1\\.990442\\]\\]\\n\\[\\[2\\.0, 1\\.0, 1\\.0, 0\\.0\\], \\[1\\.0, 2\\.0, 1\\.0, 2\\.0\\], \\[1\\.0, 1\\.0, 2\\.0, 2\\.0\\], \\[2\\.171116, -0\\.959996, 1\\.14067, -1\\.990442\\]\\]\\n\\(4, 4\\)\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhiều đầu, mỗi đầu nhìn một không gian con riêng — nối lại đúng shape gốc. Bài BOSS: ráp TẤT CẢ (embedding → vị trí → Q/K/V → scale+mask+softmax → đầu ra → multi-head) trên một câu 4 token thật, đối chiếu số tính tay từ bài `diem-attention-tinh-tay`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu chỉ dùng `so_head = 1` (MỘT đầu duy nhất, không thực sự "tách" gì), `tach_dau` trả về đúng MỘT phần tử — chính `Q`/`K`/`V` gốc, không bị cắt bớt chiều nào. Chạy toàn bộ pipeline attention với `so_head = 1` có cho ra ĐÚNG kết quả giống hệt pipeline "một đầu duy nhất" đã xây ở các bài TRƯỚC (không multi-head) hay không?
::::

::::checkpoint{mastery=0.9}
::::
