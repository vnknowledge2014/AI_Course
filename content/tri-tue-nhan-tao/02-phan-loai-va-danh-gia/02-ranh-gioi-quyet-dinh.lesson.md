---
id: tri-tue-nhan-tao.phan-loai-va-danh-gia.ranh-gioi-quyet-dinh
title: "Ranh giới quyết định"
summary: "Mô hình lọc spam đã khớp ở bài trước (w≈1.3885, b≈-3.2266) áp lên lưới 11 điểm x=0..10: ngưỡng xác suất 0.5 vạch ra một điểm CỤ THỂ x=-b/w≈2.3239 — dưới điểm đó (x=0,1,2, ba điểm) dự đoán KHÔNG spam, từ điểm đó trở lên (x=3..10, tám điểm) dự đoán spam; đổi ngưỡng từ 0.5 lên 0.7 đẩy ranh giới từ 2.3239 sang 2.9342 (dịch phải, cần nhiều bằng chứng hơn mới bị gọi là spam)."
locale: vi
track: tri-tue-nhan-tao
module: phan-loai-va-danh-gia
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.ranh-gioi-quyet-dinh]
requires: [ai.logistic-regression]
concepts: [ai.ranh-gioi-quyet-dinh]
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
Mô hình cho xác suất, không cho nhãn. Vậy chỗ nào trên trục số là ranh giới
giữa "spam" và "không spam"?
::::

::::explain{#duong-cat-hai-vung}
Bài trước khớp `p = sigmoid(w·x + b)` — một con số trong khoảng `(0, 1)`
cho MỌI giá trị `x`, không chỉ tám điểm train. Để biến xác suất đó thành
một QUYẾT ĐỊNH nhị phân (spam hay không), cần một **ngưỡng** — thường là
`0.5`: `p ≥ 0.5` thì dự đoán `1`, ngược lại dự đoán `0`.

Ngưỡng đó không rơi lộn xộn khắp nơi — nó vạch ra một điểm CỤ THỂ trên trục
`x`, gọi là **ranh giới quyết định** (decision boundary). Vì `sigmoid(z) =
0.5` xảy ra ĐÚNG khi `z = 0` (đã thấy ở bài trước: `sigmoid(0) = 0.5`), và
`z = w·x + b`, ranh giới chính là điểm `x` làm `w·x + b = 0`:

> `x_biên = −b / w`

Với một đặc trưng DUY NHẤT, ranh giới là một ĐIỂM, chia trục số thành hai
nửa: nửa dự đoán `0`, nửa dự đoán `1`. Với HAI đặc trưng, cùng lý luận
(`w₁·x₁ + w₂·x₂ + b = 0`) vạch ra một ĐƯỜNG THẲNG trên mặt phẳng — cũng chia
không gian thành đúng hai vùng. Với nhiều đặc trưng hơn nữa, ranh giới là
một MẶT PHẲNG (hyperplane) trong không gian nhiều chiều — nhưng ý tưởng
không đổi: logistic regression luôn vạch ra một ranh giới THẲNG (phẳng),
không cong, vì nó dựa trên `w·x + b = 0` — một phương trình bậc một.

Ranh giới này không cố định — nó phụ thuộc TRỰC TIẾP vào `w`, `b` đã khớp,
VÀ vào chính ngưỡng đã chọn. Đổi ngưỡng từ `0.5` sang một số khác (chẳng
hạn `0.7`, đòi hỏi mô hình "tự tin hơn" mới dám gọi là spam) dịch chuyển
ranh giới sang một điểm KHÁC — bài này đo cả hai bằng số thật.
::::

::::example{#luoi-11-diem-va-diem-chia}
Mô hình lọc spam của bài trước (`w ≈ 1.3885`, `b ≈ -3.2266`), áp lên một
lưới `11` điểm nguyên `x = 0, 1, ..., 10` (không phải chỉ tám điểm train):

```python title=readonly
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
la_spam = np.array([0, 0, 0, 1, 1, 1, 1, 1], dtype=float)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def gradient(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    dw = np.mean(so_tu_hoa * (p - la_spam))
    db = np.mean(p - la_spam)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w -= lr * dw
        b -= lr * db
    return w, b

w, b = huan_luyen(0.1, 500)

luoi = np.arange(0, 11, dtype=float)
p_luoi = sigmoid(w * luoi + b)
nhan_du_doan = (p_luoi >= 0.5).astype(int)

bien = -b / w

print("luoi:      ", luoi.tolist())
print("xac suat:  ", np.round(p_luoi, 4).tolist())
print("nhan:      ", nhan_du_doan.tolist())
print("bien quyet dinh:", round(bien, 4))
print("so diem lop 0:", int(np.sum(nhan_du_doan == 0)), " so diem lop 1:", int(np.sum(nhan_du_doan == 1)))
```

```text title=readonly
luoi:       [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
xac suat:   [0.0382, 0.1373, 0.3894, 0.7188, 0.9111, 0.9762, 0.994, 0.9985, 0.9996, 0.9999, 1.0]
nhan:       [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1]
bien quyet dinh: 2.3239
so diem lop 0: 3  so diem lop 1: 8
```

Ranh giới rơi đúng ở `x ≈ 2.3239` — giữa điểm `x=2` (xác suất `0.3894`,
dưới `0.5`) và `x=3` (xác suất `0.7188`, trên `0.5`). Ba điểm đầu (`0, 1,
2`) rơi vào vùng dự đoán `0`; tám điểm còn lại (`3` tới `10`) rơi vào vùng
dự đoán `1`. Không có điểm nào nằm "lưng chừng" — mỗi điểm thuộc đúng MỘT
trong hai vùng, phân chia bởi đúng MỘT ranh giới thẳng.
::::

::::predict{#doan-bien-khi-doi-nguong commitOnce}
Vẫn mô hình trên (`w ≈ 1.3885`, `b ≈ -3.2266`), ranh giới tại ngưỡng `0.5`
là `x ≈ 2.3239`. Giờ đổi ngưỡng: chỉ gọi là spam khi xác suất từ `0.7` trở
lên (đòi hỏi mô hình tự tin HƠN mức mặc định).

**Trước khi tính**, bạn đoán: ranh giới quyết định mới (tại ngưỡng `0.7`)
nằm ở đâu so với `2.3239`?

:::opt{correct}
Dịch sang PHẢI, tới khoảng `x ≈ 2.9342` — ngưỡng cao hơn đòi `z = w·x+b`
phải lớn hơn `0` (đủ để `sigmoid(z) ≥ 0.7`, không chỉ `≥ 0.5`), nên `x`
cũng phải lớn hơn
:::

:::opt
Dịch sang TRÁI, tới một giá trị nhỏ hơn `2.3239` — ngưỡng cao hơn nghĩa là
"khó tính hơn", nên biên phải siết CHẶT lại, về phía gần `0`
::why
Gần đúng ở trực giác "ngưỡng cao hơn = khó tính hơn" — quan sát đó không
sai VỀ Ý NGHĨA của ngưỡng cao.

Chỗ lệch nằm ở HƯỚNG dịch chuyển trên trục `x`. "Khó tính hơn" ở đây nghĩa
là cần MỘT `x` LỚN HƠN mới đạt xác suất `0.7` (vì `w > 0`, xác suất tăng
theo `x`) — nên ranh giới dịch sang PHẢI (`x` lớn hơn), không phải sang
trái. Số liệu thật xác nhận: `2.3239` (ngưỡng `0.5`) dịch thành `2.9342`
(ngưỡng `0.7`) — `2.9342 > 2.3239`.
::
:::

:::opt
Giữ nguyên `x ≈ 2.3239` — ranh giới chỉ phụ thuộc `w` và `b` đã khớp, không
phụ thuộc ngưỡng chọn để đọc kết quả
::why
Gần đúng ở việc `w` và `b` đúng là KHÔNG đổi khi đổi ngưỡng — mô hình vẫn
là chính nó, không huấn luyện lại gì cả.

Chỗ lệch: ranh giới không chỉ phụ thuộc `w`, `b` — nó là điểm `x` làm xác
suất CHẠM đúng ngưỡng đang dùng để quyết định. Ngưỡng `0.5` cho biên tại
điểm làm `sigmoid(z) = 0.5` (tức `z = 0`); ngưỡng `0.7` cho biên tại điểm
KHÁC, làm `sigmoid(z) = 0.7` (tức `z` bằng một số dương cụ thể, không còn
là `0`). Đổi ngưỡng thật sự dịch chuyển ranh giới, dù `w`, `b` đứng yên.
::
:::
::::

::::code{#dem_diem_hai_vung}
Hoàn thiện: gán nhãn cho lưới `11` điểm bằng ngưỡng `0.5`, tính ranh giới
quyết định, rồi đếm số điểm mỗi lớp.

```python title=starter
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
la_spam = np.array([0, 0, 0, 1, 1, 1, 1, 1], dtype=float)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def gradient(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    dw = np.mean(so_tu_hoa * (p - la_spam))
    db = np.mean(p - la_spam)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w -= lr * dw
        b -= lr * db
    return w, b

w, b = huan_luyen(0.1, 500)

luoi = np.arange(0, 11, dtype=float)
p_luoi = sigmoid(w * luoi + b)

def gan_nhan(p, nguong):
    return ___                        # (p >= nguong).astype(int)

nhan_du_doan = gan_nhan(p_luoi, 0.5)
bien = ___                            # -b / w
so_diem_lop1 = ___                    # dem so phan tu cua nhan_du_doan bang 1

print(round(bien, 4))
print(int(np.sum(nhan_du_doan == 0)), so_diem_lop1)
```

```python title=solution
import numpy as np

so_tu_hoa = np.array([0, 1, 2, 3, 5, 6, 7, 9], dtype=float)
la_spam = np.array([0, 0, 0, 1, 1, 1, 1, 1], dtype=float)

def sigmoid(z):
    return 1 / (1 + np.exp(-z))

def gradient(w, b):
    p = sigmoid(w * so_tu_hoa + b)
    dw = np.mean(so_tu_hoa * (p - la_spam))
    db = np.mean(p - la_spam)
    return dw, db

def huan_luyen(lr, so_buoc):
    w, b = 0.0, 0.0
    for _ in range(so_buoc):
        dw, db = gradient(w, b)
        w -= lr * dw
        b -= lr * db
    return w, b

w, b = huan_luyen(0.1, 500)

luoi = np.arange(0, 11, dtype=float)
p_luoi = sigmoid(w * luoi + b)

def gan_nhan(p, nguong):
    return (p >= nguong).astype(int)

nhan_du_doan = gan_nhan(p_luoi, 0.5)
bien = -b / w
so_diem_lop1 = int(np.sum(nhan_du_doan == 1))

print(round(bien, 4))
print(int(np.sum(nhan_du_doan == 0)), so_diem_lop1)
```

```python title=test
assert round(bien, 4) == 2.3239, f"bien quyet dinh phai la 2.3239 -- dang ra {round(bien, 4)}"
assert nhan_du_doan.tolist() == [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1], f"nhan du doan tren luoi sai -- dang ra {nhan_du_doan.tolist()}"
assert int(np.sum(nhan_du_doan == 0)) == 3, "phai co dung 3 diem lop 0 (x=0,1,2)"
assert so_diem_lop1 == 8, f"phai co dung 8 diem lop 1 (x=3..10) -- dang ra {so_diem_lop1}"

# luoi 11 diem KHONG co gia tri nao dung bang nguong 0.5 -- khong phan biet
# duoc '>=' voi '>'. Goi gan_nhan truc tiep voi mot mang tong hop co DUNG
# gia tri bang nguong de ep di qua bien that su.
assert gan_nhan(np.array([0.3, 0.5, 0.7]), 0.5).tolist() == [0, 1, 1], f"gan_nhan([0.3,0.5,0.7], 0.5): gia tri DUNG BANG nguong phai duoc xep vao lop 1 (>=, khong phai >), dang ra {gan_nhan(np.array([0.3, 0.5, 0.7]), 0.5).tolist()}"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu nằm TRONG hàm `gan_nhan(p, nguong)` — phép so sánh `p >= nguong` rồi đổi kiểu số nguyên bằng `.astype(int)`, dùng đúng hai THAM SỐ của hàm (không phải `p_luoi`/`0.5` trực tiếp). `bien` là công thức `-b / w` đã học ở phần giải thích. `so_diem_lop1` đếm bao nhiêu phần tử của `nhan_du_doan` bằng `1` — dùng `np.sum(...)` trên một phép so sánh, giống cách dòng `print` cuối đã đếm lớp `0`.
- kind: strategy
  body: 'gan_nhan: `(p >= nguong).astype(int)`. bien: `-b / w`. so_diem_lop1: `int(np.sum(nhan_du_doan == 1))`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `(p >= nguong).astype(int)`, `-b / w`, và `int(np.sum(nhan_du_doan == 1))`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: gan_nhan phai THAT SU so sanh THAM SO p voi THAM SO nguong (khong phai hang so cu the), bien phai co dau TRU dang truoc b (thieu dau tru se doi huong bien sai han), va so_diem_lop1 phai dem THAT tu nhan_du_doan -- khong duoc chep san con so 8
  requireAst:
  - kind: uses-name, target: p, min: 1
  - kind: uses-name, target: nguong, min: 1
  - kind: uses-name, target: nhan_du_doan, min: 2
  - kind: uses-operator, target: "dau-am", min: 2
  # Da thu that (goi kiemAst that tren code day du, khong rut gon):
  # - loi giai dung: dat=true, ca bon luat qua sach.
  # - quen dau tru (bien = b / w thay vi -b / w): dau-am tut tu 2 xuong 1
  #   (chi con dau tru ben trong sigmoid, "-z") -- duoi nguong 2, bi chan.
  # - gan_nhan hardcode np.zeros (bo qua nguong): nguong tut ve 0 -- duoi
  #   nguong 1, bi chan.
  # - gan_nhan dung dau '>' thay vi '>=': KHONG bi static chan (van dung du
  #   p/nguong) -- day chinh la loai lo mutation-testing THAT da tim thay
  #   (luoi 11 diem khong co gia tri nao dung bang nguong nen khong tu
  #   nhien lo ra) -- sua bang mot assertion RIENG trong tier tests, goi
  #   gan_nhan voi mang tong hop co dung gia tri bang nguong.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^2\\.3239\\n3 8\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một điểm duy nhất, `x ≈ 2.32`, chia cả trục số thành hai vùng — và điểm đó
dịch chuyển ngay khi đổi ngưỡng, dù mô hình bên dưới không hề đổi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ranh giới quyết định vừa thấy là một ĐIỂM, vì bài toán chỉ có một đặc
trưng. Nhưng bài toán phân loại "spam hay không" thật ngoài đời hiếm khi
chỉ dùng một đặc trưng số — và nhiều bài toán phân loại còn không chỉ có
HAI lớp. Một email có thể được gắn nhãn "quan trọng", "quảng cáo", hay
"spam" — BA lựa chọn, không phải hai.

`sigmoid` chỉ cho ra MỘT xác suất, đúng cho bài toán hai lớp. Với ba lớp
trở lên, cần một hàm khác — bài sau giới thiệu nó.
::::

::::checkpoint{mastery=0.8}
::::
