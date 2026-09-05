---
id: tri-tue-nhan-tao.co-che-attention.tra-cuu-embedding
title: "Tra cứu embedding: từ token ID sang vector, và gotcha ID lặp lại"
summary: "Tensor.embedding_lookup(ids): forward tra numpy fancy-indexing bang.data[ids] (mỗi ID lấy ra MỘT HÀNG), backward SCATTER-ADD -- gradient của MỖI hàng CỘNG DỒN từ MỌI vị trí dùng ID đó trong câu, không GHI ĐÈ. Trên bảng (4,3) với ids=[3,1,3] (ID 3 lặp lại), gradient hàng 3 là TỔNG hai vị trí -- một fancy-index += ngây thơ chỉ cho [0,0,1] (ghi đè, mất vị trí đầu) thay vì [1,0,1] đúng. Kiểm bằng finite-difference THẬT: sai số dưới 1e-4, xác nhận cách cộng dồn đúng."
locale: vi
track: tri-tue-nhan-tao
module: co-che-attention
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.tra-cuu-embedding]
requires: [ai.boss-dong-co-tensor]
concepts: [ai.tra-cuu-embedding]
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
Token đã là số nguyên (quest trước, BPE). Nhưng một số nguyên không mang NGHĨA gì cả — attention cần một VECTOR cho mỗi token. Bước đầu tiên: bảng embedding, tra cứu bằng ID.
::::

::::explain{#tra_cuu_embedding}
Một **bảng embedding** là một `Tensor` shape `(vocab, dim)` — MỖI HÀNG là vector của đúng MỘT token trong từ vựng, một tham số HỌC ĐƯỢC (cùng bản chất với `W` của một phép chiếu tuyến tính, không khác gì). BPE (quest trước) biến một câu thành một DANH SÁCH token ID (số nguyên); **tra cứu embedding** biến danh sách ID đó thành MỘT MA TRẬN mới, shape `(so_token, dim)` — mỗi vị trí trong câu lấy ra đúng HÀNG tương ứng với ID của nó.

Forward dùng NGUYÊN `numpy` fancy indexing: `bang.data[ids]` (`ids` một danh sách số nguyên) trả về một mảng MỚI, shape `(so_token, dim)` — hàng thứ `i` của kết quả là `bang.data[ids[i]]`.

Backward là chỗ MỚI, chưa từng gặp: gradient của MỘT HÀNG bảng embedding phải CỘNG DỒN từ MỌI vị trí trong câu dùng ID đó. Nếu một ID xuất hiện HAI LẦN trong câu, HÀNG embedding của nó nhận gradient từ CẢ HAI vị trí — TỔNG, không phải GHI ĐÈ bởi vị trí sau cùng. Đây là biến thể MỚI của gotcha "node dùng lặp lại" (đã gặp ở `tich-luy-gradient`, q8.2b, khi một `Value` dùng làm CẢ HAI toán hạng của một phép nhân; và ở `chuyen-vi-va-dinh-hinh-lai`, q8.3b, khi một `Tensor` dùng làm CẢ HAI toán hạng của một `matmul` qua `transpose()`) — nhưng lần này việc DÙNG LẶP LẠI xảy ra qua một CHỈ SỐ (index), không qua việc trực tiếp dùng lại cùng một object Python trong đồ thị tính toán.

Cách viết ĐÚNG: một vòng lặp Python, CỘNG DỒN gradient của từng vị trí vào ĐÚNG hàng ID của nó:

> `for vi_tri, tid in enumerate(ids): self.grad[tid] += out.grad[vi_tri]`

Cách viết SAI mà TRÔNG có vẻ ổn (và KHÔNG ném lỗi gì): `self.grad[ids] += out.grad` — dùng fancy indexing để "cộng dồn một lượt", không cần vòng lặp Python. Khi `ids` KHÔNG có phần tử lặp lại, cách này cho kết quả ĐÚNG. Nhưng khi một ID xuất hiện NHIỀU LẦN, `numpy` không cộng dồn các lần GHI TRÙNG chỉ số — nó đọc `self.grad[ids]` một lượt (mọi bản sao của ID trùng đọc CÙNG giá trị ban đầu), cộng `out.grad` vào, rồi GHI LẠI theo thứ tự — lần ghi SAU CÙNG đè lên lần ghi trước, mất hẳn đóng góp của những vị trí sớm hơn dùng CÙNG ID đó.
::::

::::example{#embedding_lookup_forward_backward_that}
`Bang` shape `(4, 3)`, `ids = [3, 1, 3]` (ID `3` xuất hiện HAI LẦN) — tra cứu, rồi gán TAY một gradient đến và gọi `_backward()` một bước:

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


Bang = Tensor([[1., 0., -1.], [2., 1., 0.], [0., 1., 1.], [1., -1., 2.]])   # shape (4, 3)
ids = [3, 1, 3]

Ra = Bang.embedding_lookup(ids)
print(np.round(Ra.data, 6).tolist())

Ra.grad = np.array([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])   # dong vai gradient tu tang sau
Ra._backward()
print(np.round(Bang.grad, 6).tolist())
```

```text title=readonly
[[1.0, -1.0, 2.0], [2.0, 1.0, 0.0], [1.0, -1.0, 2.0]]
[[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 1.0]]
```

`Ra.data` có `3` hàng — hàng `0` và `2` GIỐNG HỆT nhau, cả hai đều là hàng `3` của `Bang` (vì `ids[0] = ids[2] = 3`). `Bang.grad` sau `_backward()`: hàng `1` nhận đúng `Ra.grad[1]` (chỉ MỘT vị trí dùng ID `1`). Hàng `3` nhận đóng góp từ CẢ HAI vị trí dùng ID đó — vị trí `0` (`Ra.grad[0] = [1, 0, 0]`) VÀ vị trí `2` (`Ra.grad[2] = [0, 0, 1]`) — cộng lại đúng bằng hàng `3` in ra ở trên, không phải giá trị của riêng một vị trí. Hàng `0` và `2` của `Bang` giữ nguyên `0` — không vị trí nào trong câu dùng tới ID `0` hay `2`.
::::

::::example{#embedding_lookup_sai_ghi_de}
Cách viết SAI — fancy-index `+=` thay cho vòng lặp — trên ĐÚNG cùng dữ liệu:

```python title=readonly
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def embedding_lookup_sai(self, ids):
        out = Tensor(self.data[ids], (self,), 'embedding_lookup')
        def _backward():
            self.grad[ids] += out.grad     # SAI: ghi de, khong cong don, khi ids co phan tu lap
        out._backward = _backward
        return out


Bang = Tensor([[1., 0., -1.], [2., 1., 0.], [0., 1., 1.], [1., -1., 2.]])
ids = [3, 1, 3]

Ra = Bang.embedding_lookup_sai(ids)
Ra.grad = np.array([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
Ra._backward()
print(np.round(Bang.grad, 6).tolist())
```

```text title=readonly
[[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]]
```

Hàng `3` giờ chỉ là `[0, 0, 1]` — ĐÚNG BẰNG `Ra.grad[2]` (vị trí SAU CÙNG dùng ID `3`), mất hẳn đóng góp `[1, 0, 0]` của vị trí `0`. `self.grad[ids] += out.grad` đọc `self.grad[[3, 1, 3]]` (ba hàng, hai hàng đầu và cuối TRÙNG NHAU vì cùng là hàng `3`), cộng `out.grad` vào từng bản sao đó, rồi GHI LẠI theo thứ tự `[3, 1, 3]` — bản ghi ở vị trí thứ ba (lại là hàng `3`) ĐÈ LÊN bản ghi ở vị trí thứ nhất. Không có `ValueError` nào cả — đây là lỗi ÂM THẦM, nguy hiểm hơn hẳn một lỗi ném ra ngay.
::::

::::example{#kiem_finite_difference_embedding}
Kiểm ĐỘC LẬP bằng sai phân trung tâm — một hàm `numpy` THUẦN, không chạm `Tensor`, dùng ĐÚNG cách viết ĐÚNG ở trên:

```python title=readonly
import numpy as np

bang_data = np.array([[1., 0., -1.], [2., 1., 0.], [0., 1., 1.], [1., -1., 2.]])
ids = [3, 1, 3]
dY = np.array([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])

grad_giai_tich = np.array([[0., 0., 0.], [0., 1., 0.], [0., 0., 0.], [1., 0., 1.]])


def L(bang_arr):
    return np.sum(bang_arr[ids] * dY)


h = 1e-5
sai_so_toi_da = 0.0
for i in range(4):
    for j in range(3):
        bp = bang_data.copy(); bp[i, j] += h
        bm = bang_data.copy(); bm[i, j] -= h
        so = (L(bp) - L(bm)) / (2 * h)
        sai_so_toi_da = max(sai_so_toi_da, abs(so - grad_giai_tich[i, j]))

print(sai_so_toi_da < 1e-4)
```

```text title=readonly
True
```

Sai số tối đa đo được (đo bằng Python thật) là dưới `2e-11` — thấp hơn `1e-4` ở CẢ `12` phần tử của bảng. `L` không hề import hay gọi `Tensor` — nó là bằng chứng ĐỘC LẬP HOÀN TOÀN rằng công thức cộng dồn theo vòng lặp, không phải công thức fancy-index ghi đè, mới khớp đạo hàm THẬT.
::::

::::predict{#doan_id_lap_ba_lan commitOnce}
Ví dụ trên: `ids = [3, 1, 3]` (ID `3` lặp lại HAI lần, ở vị trí `0` và `2`), `Bang.grad[3]` sau `_backward()` (cách viết ĐÚNG, vòng lặp) là tổng của `Ra.grad[0]` và `Ra.grad[2]`.

**Trước khi tính**, bạn đoán: nếu câu dài hơn, `ids = [3, 1, 3, 3]` (ID `3` xuất hiện BA lần, ở vị trí `0`, `2`, `3`), và `Ra.grad` gán bốn hàng gradient riêng biệt cho bốn vị trí, `Bang.grad[3]` sau `_backward()` (vẫn cách viết ĐÚNG) sẽ là gì?

:::opt{correct}
Tổng của CẢ BA hàng `Ra.grad` ứng với vị trí `0`, `2`, `3` — vòng lặp `for vi_tri, tid in enumerate(ids): self.grad[tid] += out.grad[vi_tri]` không quan tâm một ID xuất hiện bao nhiêu lần; mỗi lần xuất hiện đều CỘNG THÊM đúng một số hạng vào `self.grad[tid]`, bất kể là lần thứ hai hay thứ ba
:::

:::opt
Tổng của HAI hàng `Ra.grad` đầu tiên trùng ID `3` (vị trí `0` và `2`), bỏ qua vị trí `3` cuối cùng vì "trùng lặp thêm một lần nữa thì không tính nữa"
::why
Gần đúng ở việc để ý ĐÚNG rằng có NHIỀU vị trí cùng dùng ID `3` cần cộng dồn — quan sát về việc phải cộng dồn nhiều lần không sai.

Chỗ lệch: không có giới hạn "tối đa hai lần cộng dồn" nào trong công thức `self.grad[tid] += out.grad[vi_tri]`. Vòng lặp chạy qua ĐÚNG mọi vị trí trong `ids`, không phân biệt đây là lần dùng ID đó thứ mấy. Ba vị trí cùng dùng ID `3` thì CẢ BA đều cộng thêm, không chỉ hai vị trí đầu.
::
:::

:::opt
Chỉ bằng hàng `Ra.grad` của vị trí CUỐI CÙNG dùng ID `3` (vị trí `3`) — giống hệt kết quả mà cách viết SAI (`self.grad[ids] += out.grad`) đã cho ở ví dụ TRÊN
::why
Gần đúng ở việc nhận ra ĐÂY CHÍNH LÀ kết quả của cách viết SAI đã minh hoạ ở trên — quan sát về sự tồn tại của lỗi ghi đè không sai.

Chỗ lệch: câu hỏi hỏi về `Bang.grad[3]` khi dùng cách viết ĐÚNG (vòng lặp `for vi_tri, tid in enumerate(ids)`), không phải cách viết SAI (fancy-index `+=`). Cách viết đúng CỘNG DỒN qua vòng lặp Python — không GHI ĐÈ như fancy-index — nên kết quả là TỔNG cả ba đóng góp, không phải chỉ đóng góp của vị trí cuối cùng.
::
:::
::::

::::code{#viet_embedding_lookup}
Hoàn thiện `embedding_lookup`: forward tra cứu bằng fancy indexing, backward cộng dồn qua vòng lặp.

```python title=starter
import numpy as np


class Tensor:
    def __init__(self, data, _prev=(), _op=''):
        self.data = np.array(data, dtype=float)
        self.grad = np.zeros_like(self.data)
        self._prev = set(_prev)
        self._op = _op
        self._backward = lambda: None

    def embedding_lookup(self, ids):
        out = Tensor(___, (self,), 'embedding_lookup')      # self.data[ids]
        def _backward():
            for vi_tri, tid in enumerate(ids):
                self.grad[tid] += ___                        # out.grad[vi_tri]
        out._backward = _backward
        return out


Bang = Tensor([[1., 0., -1.], [2., 1., 0.], [0., 1., 1.], [1., -1., 2.]])
ids = [3, 1, 3]

Ra = Bang.embedding_lookup(ids)
print(np.round(Ra.data, 6).tolist())

Ra.grad = np.array([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
Ra._backward()
print(np.round(Bang.grad, 6).tolist())
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

    def embedding_lookup(self, ids):
        out = Tensor(self.data[ids], (self,), 'embedding_lookup')
        def _backward():
            for vi_tri, tid in enumerate(ids):
                self.grad[tid] += out.grad[vi_tri]
        out._backward = _backward
        return out


Bang = Tensor([[1., 0., -1.], [2., 1., 0.], [0., 1., 1.], [1., -1., 2.]])
ids = [3, 1, 3]

Ra = Bang.embedding_lookup(ids)
print(np.round(Ra.data, 6).tolist())

Ra.grad = np.array([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
Ra._backward()
print(np.round(Bang.grad, 6).tolist())
```

```python title=test
import numpy as np

assert np.round(Ra.data, 6).tolist() == [[1.0, -1.0, 2.0], [2.0, 1.0, 0.0], [1.0, -1.0, 2.0]], f"Ra.data sai -- dang ra {np.round(Ra.data, 6).tolist()}"
assert np.round(Bang.grad, 6).tolist() == [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 1.0]], f"Bang.grad sai -- dang ra {np.round(Bang.grad, 6).tolist()}"

# rieng kiem tra CONG DON (khong GHI DE) tren ID lap lai
assert Bang.grad[3].tolist() == [1.0, 0.0, 1.0], f"Bang.grad[3] phai la TONG hai vi tri (khong GHI DE) -- dang ra {Bang.grad[3].tolist()}"

# KIEM finite-difference THAT, doc lap voi Tensor
bang_data = np.array([[1., 0., -1.], [2., 1., 0.], [0., 1., 1.], [1., -1., 2.]])
dY = np.array([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])

def L(bang_arr):
    return np.sum(bang_arr[ids] * dY)

h = 1e-5
for i in range(4):
    for j in range(3):
        bp = bang_data.copy(); bp[i, j] += h
        bm = bang_data.copy(); bm[i, j] -= h
        so = (L(bp) - L(bm)) / (2 * h)
        sai_so = abs(so - Bang.grad[i, j])
        assert sai_so < 1e-4, f"gradient Bang[{i},{j}] lech qua nhieu voi finite-difference -- sai so {sai_so}"

# rieng kiem tra tren mot BO ID KHAC, khong lap lai -- dam bao cong thuc TONG QUAT dung ca khi khong co ID nao trung
ids_khac = [0, 2]
Bang2 = Tensor([[1., 0., -1.], [2., 1., 0.], [0., 1., 1.], [1., -1., 2.]])
Ra2 = Bang2.embedding_lookup(ids_khac)
Ra2.grad = np.array([[1., 1., 1.], [2., 2., 2.]])
Ra2._backward()
assert Bang2.grad[0].tolist() == [1.0, 1.0, 1.0], f"Bang2.grad[0] sai -- dang ra {Bang2.grad[0].tolist()}"
assert Bang2.grad[2].tolist() == [2.0, 2.0, 2.0], f"Bang2.grad[2] sai -- dang ra {Bang2.grad[2].tolist()}"
assert Bang2.grad[1].tolist() == [0.0, 0.0, 0.0], f"Bang2.grad[1] phai con 0 (khong dung toi) -- dang ra {Bang2.grad[1].tolist()}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu (forward) là numpy fancy indexing lấy ra CÁC HÀNG tương ứng với `ids` từ `self.data` — `self.data[ids]`. Chỗ hai (backward, TRONG vòng lặp) cộng dồn gradient của vị trí `vi_tri` vào ĐÚNG hàng `tid` — `out.grad[vi_tri]`, KHÔNG dùng fancy-index `out.grad[ids]` nguyên khối (sẽ ghi đè, không cộng dồn, khi `ids` có phần tử lặp).
- kind: strategy
  body: 'Chỗ đầu: `self.data[ids]`. Chỗ hai: `out.grad[vi_tri]`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `self.data[ids]` và `out.grad[vi_tri]`.'
:::

:::validate
- tier: run
  timeoutMs: 10000
- tier: static
  onFail: forward phai dung fancy-indexing THAT self.data[ids] (khong duoc chep san shape hay dung vong lap Python rieng); backward TRONG vong lap phai dung out.grad[vi_tri] (dung dung chi so vi_tri cua tung vi tri, khong duoc hardcode mot chi so co dinh)
  requireAst:
  - kind: uses-name, target: ids, min: 2
  - kind: uses-name, target: vi_tri, min: 1
  - kind: uses-name, target: self, min: 7
  - kind: uses-call, target: enumerate, min: 1
  # Da thu that (goi kiemAst that tren code DAY DU cua solution, gom ca
  # harness Bang/ids/Ra): loi giai dung dat=true. ids=2 (mot lan trong
  # "enumerate(ids)" co san trong starter, mot lan trong blank1
  # "self.data[ids]" -- ten tham so "ids" trong chu ky ham KHONG dem, chi
  # dem cho Load trong than ham). vi_tri=1 (chi trong blank2
  # "out.grad[vi_tri]" -- "vi_tri" trong "for vi_tri, tid in ..." la Store,
  # khong dem). self=7 tren toan bo file. enumerate=1 (co san trong
  # starter, khong phai blank, nhung van phai CO MAT vi neu learner xoa het
  # vong lap thi enumerate=0 se bi bat). Cheat "out.grad[0]" (hardcode chi
  # so co dinh, bo qua vi_tri) lam "vi_tri" tut xuong 0 -- bi chan RIENG, VA
  # da tu kiem chung bang Python that: cheat nay lam Bang.grad[3] thanh
  # [2,0,0] (cong hai lan out.grad[0]=[1,0,0]) thay vi [1,0,1] dung -- bi
  # bat DOC LAP boi assert gia tri Bang.grad. Cheat "self.data[ids].copy()"
  # hay bien the tuong tu khong doi so dem ids/self nen khong bi static bat
  # rieng, nhung khong thay doi KET QUA (copy() cho cung gia tri) nen van
  # qua -- chap nhan duoc vi day khong phai mot cheat that.
  # Cheat "self.grad[ids] += out.grad" (fancy-index nguyen khoi, bo qua
  # toan bo vong lap/enumerate) SE lam ids tut xuong 1 (mat lan Load trong
  # blank1 vi blank1 gio la "self.grad[ids] += out.grad" -- nhung day la
  # SUA CA DONG, khong phai chi dien vao blank -- ngoai pham vi blank don
  # le) VA da tu kiem chung bang Python that no cho ket qua SAI (da minh
  # hoa trong example rieng cua bai) -- neu learner co gang lam vay bang
  # cach xoa het than vong lap va dien blank2 thanh mot bieu thuc tinh ca
  # dong, no se bi bat boi assert gia tri Bang.grad[3] doc lap.
- tier: tests
  timeoutMs: 10000
- tier: output
  match: regex
  expect: "^\\[\\[1\\.0, -1\\.0, 2\\.0\\], \\[2\\.0, 1\\.0, 0\\.0\\], \\[1\\.0, -1\\.0, 2\\.0\\]\\]\\n\\[\\[0\\.0, 0\\.0, 0\\.0\\], \\[0\\.0, 1\\.0, 0\\.0\\], \\[0\\.0, 0\\.0, 0\\.0\\], \\[1\\.0, 0\\.0, 1\\.0\\]\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Token ID giờ đã có vector — và một ID lặp lại trong câu cộng dồn đúng, không ghi đè. Bài sau: attention tự nó không phân biệt được token nào đứng TRƯỚC, token nào đứng SAU — cần bơm thêm thông tin vị trí.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảng embedding vừa xây cho MỖI token một vector — nhưng nếu đảo NGƯỢC thứ tự các token trong một câu (giữ nguyên tập hợp ID, chỉ đổi chỗ), `embedding_lookup` sẽ tra ra CÙNG các hàng đó, chỉ khác THỨ TỰ sắp xếp trong kết quả. Bản thân phép tra cứu này có "biết" token nào đứng ở vị trí nào trong câu không, hay nó chỉ đơn thuần là một phép tra bảng độc lập với vị trí?
::::

::::checkpoint{mastery=0.8}
::::
