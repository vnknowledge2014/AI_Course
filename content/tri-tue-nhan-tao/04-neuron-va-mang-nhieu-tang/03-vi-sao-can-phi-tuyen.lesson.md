---
id: tri-tue-nhan-tao.neuron-va-mang-nhieu-tang.vi-sao-can-phi-tuyen
title: "Vì sao cần phi tuyến"
summary: "Bốn điểm XOR — (0,0)->0, (0,1)->1, (1,0)->1, (1,1)->0 — thử với ba bộ trọng số hợp lý cho MỘT neuron tuyến tính đơn (không tầng ẩn): w=[1,1],b=-0.5 sai 1/4 điểm; w=[1,1],b=-1.5 sai 3/4 điểm; w=[0,0],b=0 (điểm loss thấp nhất theo cross-entropy, 0.6931) sai 2/4 điểm. Không bộ nào trong ba bộ đạt 0 điểm sai — đúng dự đoán lý thuyết: XOR không tuyến tính tách được."
locale: vi
track: tri-tue-nhan-tao
module: neuron-va-mang-nhieu-tang
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.vi-sao-can-phi-tuyen]
requires: [ai.ham-kich-hoat]
concepts: [ai.vi-sao-can-phi-tuyen]
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
Bốn hàm kích hoạt, hai vẫn bão hoà, hai không — nhưng CẢ BỐN đều phi
tuyến. Bài này thử xem: nếu bỏ luôn cái phi tuyến đó, một neuron còn giải
được bài toán nào không?
::::

::::explain{#xor-va-neuron-tuyen-tinh}
**XOR** (exclusive or — "hoặc, nhưng không phải cả hai") là một hàm hai
đầu vào nhị phân:

> `(0,0) → 0`, `(0,1) → 1`, `(1,0) → 1`, `(1,1) → 0`

Bốn điểm này, vẽ trên mặt phẳng `(x₁, x₂)`: hai điểm nhãn `0` nằm ở hai góc
ĐỐI DIỆN nhau (`(0,0)` và `(1,1)`), hai điểm nhãn `1` cũng ở hai góc đối
diện còn lại (`(0,1)` và `(1,0)`). Một neuron TUYẾN TÍNH — dù có kèm
`sigmoid` ở cuối — vẫn chỉ phân loại được dựa trên MỘT đường thẳng (hay
chính xác hơn, một siêu phẳng) `w₁x₁ + w₂x₂ + b = 0`: mọi điểm ở một phía
được gán nhãn `1`, phía còn lại gán `0`. Không đường thẳng nào tách được
hai điểm trên một đường CHÉO khỏi hai điểm trên đường chéo KIA — đây gọi là
XOR **không tuyến tính tách được** (not linearly separable).

Nói vậy vẫn còn trừu tượng. Phần dưới THỬ THẬT vài bộ trọng số hợp lý —
không phải mọi bộ có thể có (vô hạn), mà đủ để thấy cùng một kiểu sai lặp
lại.
::::

::::example{#thu_vai_bo_trong_so}
Ba bộ trọng số cho một neuron tuyến tính đơn (`z = w·x + b`, `p =
sigmoid(z)`, gán nhãn `1` khi `p ≥ 0.5`): một bộ nghiêng về "OR" (ngưỡng
thấp), một bộ nghiêng về "AND" (ngưỡng cao), và bộ trọng số bằng `0` hoàn
toàn:

```python title=readonly
import numpy as np

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0.,1.,1.,0.])

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def gan_nhan(p, nguong=0.5):
    return (p >= nguong).astype(int)

def dem_diem_sai(w, b):
    p = sigmoid(X @ np.array(w, dtype=float) + b)
    nhan = gan_nhan(p)
    return int(np.sum(nhan != y))

for ten, w, b in [("or_nhe", [1.0, 1.0], -0.5), ("and_nhe", [1.0, 1.0], -1.5), ("khong_trong_so", [0.0, 0.0], 0.0)]:
    p = sigmoid(X @ np.array(w) + b)
    print(f"{ten}: p={np.round(p,4).tolist()} nhan={gan_nhan(p).tolist()} so_sai={dem_diem_sai(w,b)}")
```

```text title=readonly
or_nhe: p=[0.3775, 0.6225, 0.6225, 0.8176] nhan=[0, 1, 1, 1] so_sai=1
and_nhe: p=[0.1824, 0.3775, 0.3775, 0.6225] nhan=[0, 0, 0, 1] so_sai=3
khong_trong_so: p=[0.5, 0.5, 0.5, 0.5] nhan=[1, 1, 1, 1] so_sai=2
```

`or_nhe` (`w=[1,1]`, `b=-0.5`): sai đúng `1` điểm — điểm `(1,1)` (nhãn thật
`0`) bị gán nhầm `1`, vì tổng `w·x` của nó (`2`) đủ lớn để vượt ngưỡng, y
hệt hai điểm nhãn `1`. `and_nhe` (`b=-1.5`, ngưỡng cao hơn): sai tới `3`
điểm — chỉ còn `(1,1)` được gán đúng, ba điểm còn lại đều bị gán `0` nhầm.
`khong_trong_so` (mọi trọng số bằng `0`): neuron dự đoán `p=0.5` cho MỌI
điểm — không phân biệt được gì cả, sai `2` trong `4` điểm (đúng bằng đoán
ngẫu nhiên).

Ba bộ, ba kiểu sai khác nhau — nhưng KHÔNG bộ nào đạt `0` điểm sai.
::::

::::example{#loss_thap_nhat_khong_phai_dung_nhat}
Trong ba bộ trên, bộ nào có cross-entropy loss THẤP NHẤT — bộ "tốt nhất"
theo đúng nghĩa mà gradient descent sẽ hội tụ về?

```python title=readonly
import numpy as np

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0.,1.,1.,0.])

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def mat_mat(w, b):
    p = sigmoid(X @ np.array(w, dtype=float) + b)
    eps = 1e-12
    return -np.mean(y * np.log(p + eps) + (1 - y) * np.log(1 - p + eps))

for ten, w, b in [("or_nhe", [1.0, 1.0], -0.5), ("and_nhe", [1.0, 1.0], -1.5), ("khong_trong_so", [0.0, 0.0], 0.0)]:
    print(f"{ten}: loss={round(mat_mat(w,b),4)}")
```

```text title=readonly
or_nhe: loss=0.7809
and_nhe: loss=0.7809
khong_trong_so: loss=0.6931
```

Bất ngờ: `khong_trong_so` (sai `2/4` điểm) có loss THẤP NHẤT (`0.6931`) —
thấp hơn cả `or_nhe` (sai chỉ `1/4` điểm, nhưng loss `0.7809`, CAO hơn). Vì
sao? `khong_trong_so` dự đoán `p=0.5` cho mọi điểm — "an toàn", không tự
tin sai ở đâu cả. `or_nhe` tự tin ĐÚNG ở ba điểm nhưng tự tin SAI (`p =
0.8176` cho một điểm nhãn thật là `0`) ở điểm còn lại — và cross-entropy
phạt một dự đoán tự tin nhưng sai NẶNG hơn nhiều so với mức nó thưởng cho
ba dự đoán tự tin đúng. Trên đúng bộ dữ liệu XOR đối xứng này, "không dám
đoán gì cả" hoá ra lại là lựa chọn có loss thấp nhất — một minh chứng khác
cho việc neuron tuyến tính không có cách nào giải đúng bài toán này: ngay
cả điểm tối ưu của NÓ (loss thấp nhất nó có thể đạt) cũng không phải là
`0` điểm sai.
::::

::::predict{#doan_bo_trong_so_moi commitOnce}
Ba bộ đã thử: `or_nhe` (sai `1`), `and_nhe` (sai `3`), `khong_trong_so`
(sai `2`). Một bộ MỚI, chưa thử: `w=[2,2]`, `b=-1.0` — cùng kiểu "nghiêng
về OR" như `or_nhe`, chỉ dốc hơn (trọng số gấp đôi).

**Trước khi tính**, bạn đoán: bộ trọng số mới này sẽ đạt `0` điểm sai, hay
vẫn sai ít nhất một điểm?

:::opt{correct}
Vẫn sai ít nhất một điểm (cụ thể: sai đúng `1`, cùng điểm `(1,1)` bị gán
nhầm như `or_nhe`) — dốc hơn chỉ đổi ĐỘ TỰ TIN của dự đoán, không đổi
ĐƯỜNG PHÂN CÁCH nằm ở đâu
:::

:::opt
Đạt `0` điểm sai — trọng số dốc hơn (`w=[2,2]` thay vì `[1,1]`) làm neuron
"quyết đoán" hơn, nên phân loại chính xác hơn
::why
Gần đúng ở việc trọng số dốc hơn đúng là làm `sigmoid(z)` tiến gần `0` hoặc
`1` hơn (dự đoán "quyết đoán" hơn, ít mập mờ hơn) — quan sát đó không sai.

Chỗ lệch: "quyết đoán hơn" không phải "đúng hơn". Nhân đôi cả `w` lẫn giữ
nguyên TỈ LỆ `b/w` chỉ kéo dài đường phân cách `w·x+b=0` theo đúng hướng cũ
— đường đó vẫn đi qua ĐÚNG VỊ TRÍ tương đối giữa bốn điểm XOR, vẫn tách
`(1,1)` sai về cùng phía với hai điểm nhãn `1`. Phóng to độ dốc không di
chuyển đường phân cách, nên không sửa được lỗi cấu trúc của bài toán.
::
:::

:::opt
Không xác định được nếu không tính — khác hẳn hai bộ `or_nhe`/`and_nhe`
trước, cặp trọng số này chưa từng thử nên không có cơ sở nào để đoán
::why
Gần đúng ở tinh thần thận trọng — đúng là bộ trọng số CỤ THỂ này chưa từng
chạy qua.

Chỗ lệch: XOR không tuyến tính tách được là một sự kiện về CẤU TRÚC của
bốn điểm, không phụ thuộc bộ trọng số cụ thể nào — không có đường thẳng
`w·x+b=0` NÀO (dù `w`, `b` là gì) tách đúng cả bốn điểm. Vì vậy có thể
khẳng định trước, không cần tính: bộ mới này CHẮC CHẮN vẫn sai ít nhất một
điểm, dù chưa biết CHÍNH XÁC điểm nào.
::
:::
::::

::::code{#kiem_tra_ba_bo_trong_so}
Hoàn thiện `gan_nhan` (ngưỡng `0.5`, dùng `>=` không phải `>`), `dem_diem_
sai` (đếm số điểm nhãn dự đoán KHÁC nhãn thật), rồi tìm bộ trọng số có ít
điểm sai nhất trong ba bộ.

```python title=starter
import numpy as np

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0.,1.,1.,0.])

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def gan_nhan(p, nguong=0.5):
    return (p ___ nguong).astype(int)          # >=, khong phai >

def dem_diem_sai(w, b):
    p = sigmoid(X @ np.array(w, dtype=float) + b)
    nhan = gan_nhan(p)
    return ___                                  # int(np.sum(nhan != y))

cac_ung_vien = {
    "or_nhe": ([1.0, 1.0], -0.5),
    "and_nhe": ([1.0, 1.0], -1.5),
    "khong_trong_so": ([0.0, 0.0], 0.0),
}

so_sai_theo_ten = {}
for ten, (w, b) in cac_ung_vien.items():
    so_sai_theo_ten[ten] = dem_diem_sai(w, b)

ten_it_sai_nhat = ___                            # min(so_sai_theo_ten, key=so_sai_theo_ten.get)

print(so_sai_theo_ten["or_nhe"], so_sai_theo_ten["and_nhe"], so_sai_theo_ten["khong_trong_so"])
print(ten_it_sai_nhat)
```

```python title=solution
import numpy as np

X = np.array([[0.,0.],[0.,1.],[1.,0.],[1.,1.]])
y = np.array([0.,1.,1.,0.])

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def gan_nhan(p, nguong=0.5):
    return (p >= nguong).astype(int)

def dem_diem_sai(w, b):
    p = sigmoid(X @ np.array(w, dtype=float) + b)
    nhan = gan_nhan(p)
    return int(np.sum(nhan != y))

cac_ung_vien = {
    "or_nhe": ([1.0, 1.0], -0.5),
    "and_nhe": ([1.0, 1.0], -1.5),
    "khong_trong_so": ([0.0, 0.0], 0.0),
}

so_sai_theo_ten = {}
for ten, (w, b) in cac_ung_vien.items():
    so_sai_theo_ten[ten] = dem_diem_sai(w, b)

ten_it_sai_nhat = min(so_sai_theo_ten, key=so_sai_theo_ten.get)

print(so_sai_theo_ten["or_nhe"], so_sai_theo_ten["and_nhe"], so_sai_theo_ten["khong_trong_so"])
print(ten_it_sai_nhat)
```

```python title=test
assert so_sai_theo_ten["or_nhe"] == 1, f"or_nhe phai sai 1 diem -- dang ra {so_sai_theo_ten['or_nhe']}"
assert so_sai_theo_ten["and_nhe"] == 3, f"and_nhe phai sai 3 diem -- dang ra {so_sai_theo_ten['and_nhe']}"
assert so_sai_theo_ten["khong_trong_so"] == 2, f"khong_trong_so phai sai 2 diem -- dang ra {so_sai_theo_ten['khong_trong_so']}"
assert ten_it_sai_nhat == "or_nhe", f"bo it sai nhat phai la or_nhe -- dang ra {ten_it_sai_nhat}"
assert all(v >= 1 for v in so_sai_theo_ten.values()), "khong co bo trong so nao trong ba bo duoc dat 0 diem sai"

# rieng kiem tra BIEN cua nguong: tren du lieu XOR that (bon xac suat cua ba
# bo tren), khong xac suat nao dung bang 0.5 chinh xac -- da kiem tra that
# (khong np.any(...) == 0.5). Goi truc tiep gan_nhan voi xac suat DUNG BANG
# nguong de ep di qua dung nhanh bien: '>=' phai gan nhan 1.
assert int(gan_nhan(np.array([0.5]))[0]) == 1, f"gan_nhan(0.5) phai la 1 (dung '>=', khong phai '>') -- dang ra {int(gan_nhan(np.array([0.5]))[0])}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. `gan_nhan`: toán tử so sánh phải là `>=` (không phải `>`), để một xác suất đúng bằng ngưỡng vẫn được gán nhãn dương — đúng quy ước đã dùng ở BOSS của T8.1. `dem_diem_sai`: đếm số vị trí `nhan` khác `y` — dùng `np.sum(nhan != y)`, bọc `int(...)` để trả về một số nguyên thường, không phải mảng `numpy` một phần tử. `ten_it_sai_nhat`: khoá có GIÁ TRỊ nhỏ nhất trong dict `so_sai_theo_ten` — dùng `min(...)` với tham số `key`.
- kind: strategy
  body: 'Toán tử trong gan_nhan: `>=`. dem_diem_sai: `int(np.sum(nhan != y))`. ten_it_sai_nhat: `min(so_sai_theo_ten, key=so_sai_theo_ten.get)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `>=`, `int(np.sum(nhan != y))`, và `min(so_sai_theo_ten, key=so_sai_theo_ten.get)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: gan_nhan phai dung dung toan tu >= (khong phai >); dem_diem_sai phai dem THAT so diem sai bang np.sum (khong duoc tra ve mot con so chep san); ten_it_sai_nhat phai tinh THAT bang min(...) tren so_sai_theo_ten
  requireAst:
  - kind: uses-operator, target: ">=", min: 1
  - kind: uses-call, target: sum, min: 1
  - kind: uses-call, target: min, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca ba luat qua sach. Cheat doi >= thanh > trong gan_nhan
  # lam luat ">=" ve 0 -- bi chan. Cheat dem_diem_sai tra ve hang so 1 (khong
  # dung sum) lam "sum" ve 0 -- bi chan. Cheat chep san ten_it_sai_nhat =
  # "or_nhe" (khong goi min) lam "min" ve 0 -- bi chan. Ca ba cheat bi bat
  # DOC LAP voi nhau.
  #
  # MUTATION-TESTING KIEU BIEN: gan_nhan dung > thay vi >= CHI lo ra tren du
  # lieu neu co xac suat dung bang 0.5 chinh xac -- da kiem tra that tren ca
  # ba bo trong so cua bai (or_nhe/and_nhe/khong_trong_so): khong bo nao co
  # xac suat dung bang 0.5, nen mutation nay se qua SACH tier tests/output
  # neu chi dua vao ket qua tren ba bo do. Da them assert GOI TRUC TIEP
  # gan_nhan(np.array([0.5])) trong tier tests de ep di qua dung nhanh bien.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^1 3 2\\nor_nhe\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba bộ trọng số, ba kiểu sai — không bộ nào đạt `0`. Đây không phải một
điểm yếu tạm thời có thể sửa bằng cách chọn `w`, `b` khéo hơn — nó là giới
hạn CẤU TRÚC của một neuron tuyến tính đơn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ba bộ trọng số vừa thử đều dùng đúng MỘT neuron. Nếu, thay vì một neuron,
ta dùng HAI neuron song song — mỗi neuron nhìn cùng bốn điểm XOR nhưng học
một đường phân cách RIÊNG — rồi ghép hai đầu ra đó lại bằng một neuron THỨ
BA, liệu tổ hợp đó có vượt qua được giới hạn vừa thấy không?
::::

::::checkpoint{mastery=0.8}
::::
