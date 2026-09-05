---
id: tri-tue-nhan-tao.neuron-va-mang-nhieu-tang.ham-kich-hoat
title: "Hàm kích hoạt"
summary: "Bốn hàm kích hoạt tại z=[-10,-1,0,1,10]: sigmoid cho đạo hàm [0.000045, 0.196612, 0.25, 0.196612, 0.000045] — bão hoà nặng ở hai đầu, tại z=10 đạo hàm nhỏ hơn tại z=0 gần 5507 lần. tanh cùng kiểu bão hoà, còn nặng hơn (tại z=10 nhỏ hơn tại z=0 hơn 121 triệu lần). ReLU cho đạo hàm [0,0,0,1,1] và Leaky ReLU [0.01,0.01,0.01,1,1] — cả hai KHÔNG bão hoà ở phía dương, đạo hàm giữ nguyên =1 dù z lớn cỡ nào."
locale: vi
track: tri-tue-nhan-tao
module: neuron-va-mang-nhieu-tang
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.ham-kich-hoat]
requires: [ai.neuron-don]
concepts: [ai.ham-kich-hoat]
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
Bài trước dùng đúng một hàm kích hoạt: `sigmoid`. Có những lựa chọn khác —
và chọn sai có thể làm cả một mạng ngừng học.
::::

::::explain{#bon-ham-kich-hoat}
Bài `neuron-don` giới thiệu `σ` như MỘT khe cắm — bất kỳ hàm nào nén `z` về
một khoảng hợp lý đều dùng được ở đó. Bốn lựa chọn phổ biến nhất:

> **sigmoid** — `σ(z) = 1 / (1 + e^-z)`, nén về `(0, 1)`. Đạo hàm:
> `σ'(z) = σ(z)·(1 − σ(z))`.
>
> **tanh** — `tanh(z) = (e^z − e^-z) / (e^z + e^-z)`, nén về `(-1, 1)` (đối
> xứng qua `0`, khác `sigmoid` không đối xứng). Đạo hàm: `tanh'(z) = 1 −
> tanh(z)²`.
>
> **ReLU** (rectified linear unit) — `ReLU(z) = max(0, z)`. Đạo hàm: `1`
> khi `z > 0`, `0` khi `z ≤ 0` — không nén về khoảng nào cả, giữ nguyên mọi
> giá trị DƯƠNG.
>
> **Leaky ReLU** — gần giống `ReLU`, nhưng phía ÂM không cắt hẳn về `0` mà
> nhân với một hệ số nhỏ `α` (thường `0.01`): `z` khi `z > 0`, `α·z` khi
> `z ≤ 0`. Đạo hàm: `1` khi `z > 0`, `α` khi `z ≤ 0`.

Bốn đạo hàm này CHƯA dùng tới trong track — chưa có bước huấn luyện nào ở
q8.2a (mọi mạng trong quest này dùng trọng số CHO SẴN, không học). Nhưng
đạo hàm của hàm kích hoạt là đúng thứ **backprop** (q8.2b) sẽ cần: cập nhật
một trọng số ở tầng ẩn đòi hỏi biết `σ` thay đổi nhanh hay chậm quanh giá
trị `z` hiện tại — đúng việc `σ'(z)` đo được.
::::

::::example{#sigmoid_tanh_bao_hoa}
`sigmoid` và `tanh` tại năm giá trị `z`, từ rất âm tới rất dương:

```python title=readonly
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def sigmoid_dao_ham(z):
    s = sigmoid(z)
    return s * (1 - s)

def tanh_dao_ham(z):
    return 1 - np.tanh(z) ** 2

zs = np.array([-10.0, -1.0, 0.0, 1.0, 10.0])

print("sigmoid:", np.round(sigmoid(zs), 4).tolist())
print("sigmoid':", np.round(sigmoid_dao_ham(zs), 6).tolist())
print("tanh:", np.round(np.tanh(zs), 4).tolist())
print("tanh':", np.round(tanh_dao_ham(zs), 6).tolist())
```

```text title=readonly
sigmoid: [0.0, 0.2689, 0.5, 0.7311, 1.0]
sigmoid': [4.5e-05, 0.196612, 0.25, 0.196612, 4.5e-05]
tanh: [-1.0, -0.7616, 0.0, 0.7616, 1.0]
tanh': [0.0, 0.419974, 1.0, 0.419974, 0.0]
```

Tại `z = 0`, cả hai đạo hàm đạt mức CAO NHẤT (`sigmoid' = 0.25`, `tanh' =
1.0`) — vùng này `σ` đổi nhanh nhất theo `z`. Tại `z = ±10`, cả hai đạo hàm
gần như `0` (`sigmoid' ≈ 0.000045` — nhỏ hơn giá trị tại `z=0` khoảng
`5507` lần). Đây là hiện tượng **bão hoà** (saturation): khi `|z|` đủ lớn,
`sigmoid` và `tanh` gần như phẳng — đổi `z` thêm một chút không còn làm
`σ(z)` đổi gì đáng kể, và đạo hàm gần `0` phản ánh đúng điều đó.
::::

::::example{#relu_khong_bao_hoa}
`ReLU` và `Leaky ReLU` tại đúng năm giá trị `z` như trên:

```python title=readonly
import numpy as np

def relu(z):
    return np.maximum(0, z)

def relu_dao_ham(z):
    return np.where(z > 0, 1.0, 0.0)

def leaky_relu(z, alpha=0.01):
    return np.where(z > 0, z, alpha * z)

def leaky_relu_dao_ham(z, alpha=0.01):
    return np.where(z > 0, 1.0, alpha)

zs = np.array([-10.0, -1.0, 0.0, 1.0, 10.0])

print("ReLU:", relu(zs).tolist())
print("ReLU':", relu_dao_ham(zs).tolist())
print("Leaky:", np.round(leaky_relu(zs), 4).tolist())
print("Leaky':", leaky_relu_dao_ham(zs).tolist())
```

```text title=readonly
ReLU: [0.0, 0.0, 0.0, 1.0, 10.0]
ReLU': [0.0, 0.0, 0.0, 1.0, 1.0]
Leaky: [-0.1, -0.01, 0.0, 1.0, 10.0]
Leaky': [0.01, 0.01, 0.01, 1.0, 1.0]
```

Ở phía DƯƠNG, đạo hàm của cả hai giữ nguyên `1` — tại `z=10` HỆT tại `z=1`,
không hề bão hoà, khác hẳn `sigmoid`/`tanh` ở ví dụ trước. Ở phía ÂM, `ReLU`
cắt đạo hàm về `0` tuyệt đối (một neuron ReLU với `z` luôn âm sẽ không bao
giờ cập nhật nữa — vấn đề gọi là "neuron chết"); `Leaky ReLU` giữ đạo hàm
ở mức nhỏ (`0.01`) thay vì `0` tuyệt đối, đúng lý do nó tồn tại.
::::

::::predict{#doan_dao_ham_lon_hay_nho commitOnce}
Tại `z = 10` (dương, khá lớn), bốn đạo hàm vừa tính là: `sigmoid' ≈
0.000045`, `tanh' ≈ 0.0` (còn nhỏ hơn), `ReLU' = 1.0`, `Leaky' = 1.0`.

**Trước khi đọc lại**, bạn đoán: nếu tăng `z` lên `100` (dương, lớn hơn
nhiều), đạo hàm nào trong bốn cái trên sẽ ĐỔI, và đạo hàm nào giữ nguyên?

:::opt{correct}
`sigmoid'` và `tanh'` sẽ càng gần `0` hơn nữa (bão hoà càng nặng khi `|z|`
càng lớn); `ReLU'` và `Leaky'` giữ nguyên đúng `1.0` — công thức của chúng
không phụ thuộc ĐỘ LỚN của `z` dương, chỉ phụ thuộc DẤU của nó
:::

:::opt
Cả bốn đạo hàm đều giảm dần về `0` khi `z` tăng — "bão hoà" là hiện tượng
chung của MỌI hàm kích hoạt khi `z` đủ lớn
::why
Gần đúng ở việc bão hoà đúng là hiện tượng có thật, và đúng là XẢY RA với
hai trong bốn hàm kích hoạt vừa xét — quan sát đó không sai.

Chỗ lệch: bão hoà không phải tính chất CHUNG của mọi hàm kích hoạt. Công
thức đạo hàm của `ReLU` (`1` khi `z>0`, `0` khi `z≤0`) và `Leaky ReLU` (`1`
khi `z>0`, `α` khi `z≤0`) không hề tham chiếu tới ĐỘ LỚN của `z` — chỉ tham
chiếu DẤU của nó. `z=10` và `z=100` cùng dương, nên cả hai cho đúng cùng
đạo hàm `1.0`, không hề giảm.
::
:::

:::opt
Chỉ `ReLU'` và `Leaky'` đổi (vì công thức của chúng có `max`, mà `max` phụ
thuộc độ lớn); `sigmoid'` và `tanh'` giữ nguyên vì đã bão hoà sẵn rồi, tăng
`z` thêm không còn ý nghĩa gì với chúng
::why
Gần đúng ở việc nhận ra `sigmoid'`/`tanh'` ĐÃ ở gần `0` tại `z=10`, nên trực
giác "đằng nào cũng gần 0 rồi" không hoàn toàn vô lý.

Chỗ lệch: "gần `0`" không phải "đúng bằng `0`, không đổi được nữa" — hàm
mũ `e^-z` vẫn tiếp tục giảm khi `z` tăng, nên `sigmoid'` VẪN tiếp tục nhỏ
đi (dù chênh lệch quá nhỏ để thấy bằng mắt ở `4` chữ số thập phân). Còn
`ReLU'`/`Leaky'` mới là hai đạo hàm THỰC SỰ không đổi — công thức `max(0,
z)` có `max`, nhưng ĐẠO HÀM của nó chỉ hỏi "z dương hay không", không hỏi
"z dương bao nhiêu".
::
:::
::::

::::code{#hoan_thien_bon_dao_ham}
Hoàn thiện đạo hàm của `tanh`, `ReLU`, và `Leaky ReLU`. Chú ý: điều kiện
biên dùng `z > 0` (không phải `z >= 0`) — tại đúng `z = 0`, cả `ReLU'` lẫn
`Leaky'` phải trả về nhánh "z không dương" (`0.0` và `α`, chứ không phải
`1.0`).

```python title=starter
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def sigmoid_dao_ham(z):
    s = sigmoid(z)
    return s * (1 - s)

def tanh_dao_ham(z):
    return ___                       # 1 - np.tanh(z)**2

def relu(z):
    return np.maximum(0, z)

def relu_dao_ham(z):
    return ___                       # np.where(z > 0, 1.0, 0.0)

def leaky_relu(z, alpha=0.01):
    return np.where(z > 0, z, alpha * z)

def leaky_relu_dao_ham(z, alpha=0.01):
    return ___                       # np.where(z > 0, 1.0, alpha)

zs = np.array([-10.0, -1.0, 0.0, 1.0, 10.0])

print(np.round(sigmoid_dao_ham(zs), 6).tolist())
print(np.round(tanh_dao_ham(zs), 6).tolist())
print(relu_dao_ham(zs).tolist())
print(leaky_relu_dao_ham(zs).tolist())
print(round(float(relu_dao_ham(np.array([0.0]))[0]), 4), round(float(leaky_relu_dao_ham(np.array([0.0]))[0]), 4))
```

```python title=solution
import numpy as np

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def sigmoid_dao_ham(z):
    s = sigmoid(z)
    return s * (1 - s)

def tanh_dao_ham(z):
    return 1 - np.tanh(z) ** 2

def relu(z):
    return np.maximum(0, z)

def relu_dao_ham(z):
    return np.where(z > 0, 1.0, 0.0)

def leaky_relu(z, alpha=0.01):
    return np.where(z > 0, z, alpha * z)

def leaky_relu_dao_ham(z, alpha=0.01):
    return np.where(z > 0, 1.0, alpha)

zs = np.array([-10.0, -1.0, 0.0, 1.0, 10.0])

print(np.round(sigmoid_dao_ham(zs), 6).tolist())
print(np.round(tanh_dao_ham(zs), 6).tolist())
print(relu_dao_ham(zs).tolist())
print(leaky_relu_dao_ham(zs).tolist())
print(round(float(relu_dao_ham(np.array([0.0]))[0]), 4), round(float(leaky_relu_dao_ham(np.array([0.0]))[0]), 4))
```

```python title=test
assert np.round(sigmoid_dao_ham(zs), 6).tolist() == [4.5e-05, 0.196612, 0.25, 0.196612, 4.5e-05], f"sigmoid_dao_ham sai -- dang ra {np.round(sigmoid_dao_ham(zs), 6).tolist()}"
assert np.round(tanh_dao_ham(zs), 6).tolist() == [0.0, 0.419974, 1.0, 0.419974, 0.0], f"tanh_dao_ham sai -- dang ra {np.round(tanh_dao_ham(zs), 6).tolist()}"
assert relu_dao_ham(zs).tolist() == [0.0, 0.0, 0.0, 1.0, 1.0], f"relu_dao_ham sai -- dang ra {relu_dao_ham(zs).tolist()}"
assert leaky_relu_dao_ham(zs).tolist() == [0.01, 0.01, 0.01, 1.0, 1.0], f"leaky_relu_dao_ham sai -- dang ra {leaky_relu_dao_ham(zs).tolist()}"

# rieng kiem tra BIEN z=0: du lieu cua zs (-10,-1,0,1,10) da co san z=0, nhung
# assert tren tolist() chi so sanh CA MANG -- rieng bien nay can mot loi goi
# TRUC TIEP de khang dinh dung nhanh nao duoc chon, phong khi mang zs doi.
assert round(float(relu_dao_ham(np.array([0.0]))[0]), 4) == 0.0, "relu_dao_ham(0.0) phai la 0.0 (dung '>', khong phai '>=')"
assert round(float(leaky_relu_dao_ham(np.array([0.0]))[0]), 4) == 0.01, "leaky_relu_dao_ham(0.0) phai la alpha=0.01 (dung '>', khong phai '>=')"
```

:::hints
- kind: attention
  body: Ba chỗ trống, ba công thức đã nêu ở phần giải thích. `tanh_dao_ham`: `1 - tanh(z)²`, dùng `np.tanh(z) ** 2`. `relu_dao_ham` và `leaky_relu_dao_ham`: dùng `np.where(dieu_kien, gia_tri_neu_dung, gia_tri_neu_sai)`, điều kiện là `z > 0` (không phải `>=`) cho cả hai — nhìn cách `leaky_relu` (đã viết sẵn ở trên) dùng đúng điều kiện đó.
- kind: strategy
  body: 'tanh_dao_ham: `1 - np.tanh(z) ** 2`. relu_dao_ham: `np.where(z > 0, 1.0, 0.0)`. leaky_relu_dao_ham: `np.where(z > 0, 1.0, alpha)`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `1 - np.tanh(z) ** 2`, `np.where(z > 0, 1.0, 0.0)`, và `np.where(z > 0, 1.0, alpha)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: tanh_dao_ham phai THAT SU goi np.tanh; relu_dao_ham va leaky_relu_dao_ham phai dung np.where voi dieu kien so sanh THAT (z > 0, khong duoc chep san mang hay dung cong thuc khac nhu np.sign/np.maximum de lach)
  requireAst:
  - kind: uses-call, target: tanh, min: 1
  - kind: uses-call, target: where, min: 3
  - kind: uses-operator, target: ">", min: 3
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca ba luat qua sach (tanh=1 lan trong tanh_dao_ham; where
  # va > moi thu co 3 lan -- 1 trong leaky_relu da cho san, 1 trong
  # relu_dao_ham, 1 trong leaky_relu_dao_ham). Cheat "doi > thanh >= trong
  # relu_dao_ham" lam "> " tut tu 3 xuong 2 -- bi chan (leaky_relu cho san va
  # leaky_relu_dao_ham dung van con giu > , nhung mat dung 1 lan tu
  # relu_dao_ham la du roi xuong duoi nguong 3). Cheat tuong tu doi > thanh
  # >= trong leaky_relu_dao_ham cung bi chan cung cach. Cheat "tanh_dao_ham
  # dung np.where lach thay vi goi np.tanh" lam "tanh" ve 0 -- bi chan rieng.
  # Cheat "relu_dao_ham dung np.sign(np.maximum(z,0)) thay vi where/>" lam ca
  # where va > deu tut xuong 2 -- bi chan boi ca hai luat.
  #
  # MUTATION-TESTING KIEU BIEN: doi > thanh >= chi trong MOT ham (rieng le)
  # khong doi ket qua tren mang zs=(-10,-1,0,1,10) NEU chi nhin gia tri tai
  # nhung z do (khong co z=0 o day thi khong lo, nhung zs THAT SU co z=0!) --
  # da kiem tra that: neu relu_dao_ham dung >=, ket qua tai z=0 se thanh 1.0
  # thay vi 0.0, nen tier output/tests da tu bat duoc mutation nay qua chinh
  # mang zs. Rieng them assert goi TRUC TIEP voi np.array([0.0]) o tier tests
  # de dam bao boundary duoc kiem dinh KHONG PHU THUOC vao viec mang zs vi du
  # co doi hay khong trong tuong lai.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\[4\\.5e-05, 0\\.196612, 0\\.25, 0\\.196612, 4\\.5e-05\\]\\n\\[0\\.0, 0\\.419974, 1\\.0, 0\\.419974, 0\\.0\\]\\n\\[0\\.0, 0\\.0, 0\\.0, 1\\.0, 1\\.0\\]\\n\\[0\\.01, 0\\.01, 0\\.01, 1\\.0, 1\\.0\\]\\n0\\.0 0\\.01\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`sigmoid`/`tanh` bão hoà — đạo hàm gần `0` khi `|z|` lớn. `ReLU`/`Leaky
ReLU` không bão hoà ở phía dương — đúng lý do chúng phổ biến trong các tầng
ẩn hiện đại.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài `vi-sao-can-phi-tuyen` sắp thử phân loại bốn điểm XOR bằng MỘT neuron
tuyến tính duy nhất — không hàm kích hoạt phi tuyến nào cả, chỉ `z = w·x +
b` trần trụi. Bốn hàm kích hoạt vừa học đều là hàm PHI TUYẾN (không phải
đường thẳng). Điều đó có liên quan gì tới việc một neuron tuyến tính có
giải được XOR hay không?
::::

::::checkpoint{mastery=0.8}
::::
