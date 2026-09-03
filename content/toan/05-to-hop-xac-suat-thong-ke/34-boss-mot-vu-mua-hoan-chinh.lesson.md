---
id: toan.to-hop-xac-suat-thong-ke.boss-mot-vu-mua-hoan-chinh
title: "BOSS — Một vụ mùa hoàn chỉnh"
summary: "Ghép đếm (chọn k luống bằng C(n,k)) + xác suất (rút hạt từ túi trộn) + biến ngẫu nhiên/kỳ vọng (E[X] số hạt nảy mầm) + thống kê (x̄, trung vị, ngoại lệ trên sáu mùa thật) TRÊN MỘT vụ mùa của khu vườn."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 34
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [math.combination-select-k, math.classical-probability, math.expectation, math.sample-mean, math.median, math.outlier-detection]
requires: [math.combination-select-k, math.classical-probability, math.expectation, math.sample-mean, math.median, math.outlier-detection]
concepts: [math.mot-vu-mua-hoan-chinh]
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
Ba mươi ba bài, bốn mảnh riêng: đếm, xác suất, biến ngẫu nhiên,
thống kê. Byte muốn kiểm CẢ BỐN cùng lúc, trên MỘT vụ mùa. Ghép được
không?
::::

::::explain{#mot-vu-mua-la-gi}
Không có khái niệm mới Ở bài này — CHỈ ghép lại. Một vụ mùa của Byte
mang BỐN thứ đã học:

- **Đếm** (bài 1-10): sáng nay Byte CHỈ có thời gian thu hoạch `2`
  trong `4` luống — `C(4,2)` cách chọn.
- **Xác suất** (bài 11-21): túi mười hạt giống, rút MỘT hạt — `P(cà
  chua)`.
- **Biến ngẫu nhiên & kỳ vọng** (bài 22-26): gieo bốn hạt, MỖI hạt
  độc lập nảy mầm hay không — `E[X]`, số hạt nảy TRUNG BÌNH.
- **Thống kê** (bài 27-33): sáu mùa THẬT đã thu hoạch — `x̄`, trung
  vị, VÀ mùa nào LÀ ngoại lệ.

Việc còn lại: viết MỘT hàm kiểm CẢ BỐN mảnh trên đúng vụ mùa này.
::::

::::example{#bon-manh-rieng-le}
Trước khi ghép, xem TỪNG mảnh cho ra gì — dùng LẠI đúng những hàm
bài 7 (tổ hợp), bài 12 (xác suất cổ điển), bài 24 (kỳ vọng), bài 27
(trung bình) đã viết:

```python title=readonly
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

def to_hop(n, k):
    return hoan_vi(n, k) // giai_thua(k)

def la_xac_suat(a, omega):
    return len(a) / len(omega)


print(to_hop(4, 2))

omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
print(la_xac_suat(ca_chua, omega))
```

```text title=readonly
6
0.5
```

`C(4,2)=6` cách chọn hai luống thu hoạch sáng nay. `P(\text{cà
chua})=0.5`. KHÔNG hàm nào Ở TRÊN LÀ mới; cả hai đã có TRƯỚC bài
này.
::::

::::predict{#doan-mot-manh-hong commitOnce}
Byte đổi mùa CUỐI — thay vì `5` kg (mất mùa nặng), giờ LÀ `39` kg
(một mùa BÌNH THƯỜNG, KHÔNG còn là ngoại lệ). Ba mảnh ĐẦU (đếm, xác
suất, biến ngẫu nhiên) KHÔNG đổi gì. Gọi lại hàm `kiem_vu_mua()`
(ghép cả bốn mảnh bằng `assert`, viết Ở bước sau). Điều gì xảy ra?

```python
mua_moi = [40, 42, 41, 39, 43, 39]
# ... kiem_vu_mua() chạy qua mảnh đếm, xác suất, biến ngẫu nhiên,
# rồi tới assert kiểm mùa cuối (39 kg) có phải ngoại lệ hay không.
```

Chuyện gì xảy ra khi gọi `kiem_vu_mua()` với `mua_moi`?

:::opt{correct}
Chương trình DỪNG ngay tại `assert` kiểm ngoại lệ — BA mảnh ĐẦU đã
chạy qua trót lọt, NHƯNG dòng `return` không bao giờ tới
:::

:::opt
Hàm vẫn trả VỀ đủ bốn giá trị NHƯ cũ, chỉ riêng phần "ngoại lệ" đổi
thành `False` — `assert` chỉ GHI NHẬN câu Đ/S, không CHẶN chương
trình chạy tiếp
::why
Gần đúng ở việc bạn tính ĐÚNG `39` KHÔNG còn LÀ ngoại lệ (nằm gọn
trong khoảng `[29,53]`, bài 33) — quan sát ĐÓ chính xác.

Chỗ lệch: `assert` KHÔNG PHẢI một câu ghi chú — GẶP điều kiện SAI,
nó NÉM `AssertionError` VÀ dừng hàm NGAY LẬP TỨC, y hệt bài 32 T2.3
VÀ BOSS T2.4 đã dạy. Dòng `return` nằm SAU `assert` ngoại lệ — KHÔNG
BAO GIỜ được chạy tới. Hàm KHÔNG trả VỀ gì cả; nó NÉM lỗi.
::
:::

:::opt
Chương trình lỗi NGAY từ mảnh ĐẦU tiên (đếm) — VÌ `mua_moi` bị đổi
thì TOÀN BỘ dữ liệu bên trong hàm coi như KHÔNG còn đáng tin, mảnh
nào cũng phải chạy LẠI từ đầu VÀ báo lỗi
::why
Gần đúng ở việc bạn nghĩ TỚI một sự cố LAN RỘNG — đổi một biến thì
cả hàm "không đáng tin" nữa, một trực giác thận trọng.

Chỗ lệch: `to_hop(4,2)` (mảnh đếm), `la_xac_suat(...)` (mảnh xác
suất), VÀ `ky_vong(pp)` (mảnh biến ngẫu nhiên) KHÔNG hề đọc `mua`
— chúng CHỈ dùng dữ liệu RIÊNG của mình, KHÔNG đổi. Ba mảnh ĐẦU chạy
qua TRÓT LỌT; lỗi CHỈ nổ ra Ở đúng `assert` cuối, nơi `mua` MỚI thật
sự được đọc tới.
::
:::
::::

::::code{#viet_kiem_vu_mua}
Bốn hàm bên dưới ĐÃ viết sẵn — không hàm nào mới. Việc của bạn: ghép
CHÚNG lại thành `kiem_vu_mua()`, điền đúng BỐN chỗ trống, MỖI chỗ
ứng với MỘT mảnh (đếm, xác suất, biến ngẫu nhiên, thống kê).

```python title=starter
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

def to_hop(n, k):
    return hoan_vi(n, k) // giai_thua(k)

def la_xac_suat(a, omega):
    return len(a) / len(omega)

def ky_vong(pp):
    return sum(x * p for x, p in pp.items())

def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)

def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def la_ngoai_le(x, q1, q3):
    iqr = q3 - q1
    return x < q1 - 1.5 * iqr or x > q3 + 1.5 * iqr


def kiem_vu_mua():
    # Mảnh 1 -- đếm: chọn 2 trong 4 luống để thu hoạch sáng nay
    cach_chon = ___
    assert cach_chon == 6, "C(4,2) phai la 6"

    # Mảnh 2 -- xác suất: rút một hạt từ túi mười hạt, được cà chua
    omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
    ca_chua = {"h1", "h2", "h3", "h4", "h5"}
    xac_suat_ca_chua = ___
    assert xac_suat_ca_chua == 0.5, "P(ca chua) phai la 0.5"

    # Mảnh 3 -- biến ngẫu nhiên: E[X], so hat nay trung binh trong 4 hat
    pp = {0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}
    ky_vong_x = ___
    assert ky_vong_x == 2.0, "E[X] phai la 2.0"

    # Mảnh 4 -- thống kê: sáu mùa thật đã thu hoạch
    mua = [40, 42, 41, 39, 43, 5]
    x_tb = trung_binh(mua)
    assert x_tb == 35.0, "x-bar phai la 35.0"
    la_bat_thuong = la_ngoai_le(mua[-1], 38, 44)
    assert la_bat_thuong is True, "mua cuoi (5kg) phai la ngoai le"
    tv = ___
    assert tv == 40.5, "trung vi phai la 40.5"

    return cach_chon, xac_suat_ca_chua, ky_vong_x, tv


print(kiem_vu_mua())
```

```python title=solution
def giai_thua(n):
    ket_qua = 1
    for i in range(1, n + 1):
        ket_qua = ket_qua * i
    return ket_qua

def hoan_vi(n, k):
    return giai_thua(n) // giai_thua(n - k)

def to_hop(n, k):
    return hoan_vi(n, k) // giai_thua(k)

def la_xac_suat(a, omega):
    return len(a) / len(omega)

def ky_vong(pp):
    return sum(x * p for x, p in pp.items())

def trung_binh(du_lieu):
    return sum(du_lieu) / len(du_lieu)

def trung_vi(du_lieu):
    s = sorted(du_lieu)
    n = len(s)
    giua = n // 2
    if n % 2 == 0:
        return (s[giua - 1] + s[giua]) / 2
    return s[giua]

def la_ngoai_le(x, q1, q3):
    iqr = q3 - q1
    return x < q1 - 1.5 * iqr or x > q3 + 1.5 * iqr


def kiem_vu_mua():
    # Mảnh 1 -- đếm: chọn 2 trong 4 luống để thu hoạch sáng nay
    cach_chon = to_hop(4, 2)
    assert cach_chon == 6, "C(4,2) phai la 6"

    # Mảnh 2 -- xác suất: rút một hạt từ túi mười hạt, được cà chua
    omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
    ca_chua = {"h1", "h2", "h3", "h4", "h5"}
    xac_suat_ca_chua = la_xac_suat(ca_chua, omega)
    assert xac_suat_ca_chua == 0.5, "P(ca chua) phai la 0.5"

    # Mảnh 3 -- biến ngẫu nhiên: E[X], so hat nay trung binh trong 4 hat
    pp = {0: 0.0625, 1: 0.25, 2: 0.375, 3: 0.25, 4: 0.0625}
    ky_vong_x = ky_vong(pp)
    assert ky_vong_x == 2.0, "E[X] phai la 2.0"

    # Mảnh 4 -- thống kê: sáu mùa thật đã thu hoạch
    mua = [40, 42, 41, 39, 43, 5]
    x_tb = trung_binh(mua)
    assert x_tb == 35.0, "x-bar phai la 35.0"
    la_bat_thuong = la_ngoai_le(mua[-1], 38, 44)
    assert la_bat_thuong is True, "mua cuoi (5kg) phai la ngoai le"
    tv = trung_vi(mua)
    assert tv == 40.5, "trung vi phai la 40.5"

    return cach_chon, xac_suat_ca_chua, ky_vong_x, tv


print(kiem_vu_mua())
```

```python title=test
assert kiem_vu_mua() == (6, 0.5, 2.0, 40.5), "vu mua hoan chinh phai qua ca bon manh ghep"
assert to_hop(5, 3) == 10, "to hop dung nghia toan hoc, khong rieng cua vu mua"
assert la_xac_suat({"a", "b"}, {"a", "b", "c", "d"}) == 0.5, "xac suat dung nghia toan hoc"
assert ky_vong({7: 1.0}) == 7.0, "ky vong dung nghia toan hoc"
assert trung_vi([1, 2, 3, 4]) == 2.5, "trung vi dung nghia toan hoc, khong rieng cua vu mua"
```

:::hints
- kind: attention
  body: "Bon cho trong ung voi bon manh da hoc: manh 1 goi to_hop(4, 2); manh 2 goi la_xac_suat(ca_chua, omega); manh 3 goi ky_vong(pp); manh 4 goi trung_vi(mua)."
- kind: strategy
  body: "Manh 1: to_hop(4, 2). Manh 2: la_xac_suat(ca_chua, omega). Manh 3: ky_vong(pp). Manh 4: trung_vi(mua)."
- kind: one-line
  body: "Bon dong: to_hop(4, 2) -- la_xac_suat(ca_chua, omega) -- ky_vong(pp) -- trung_vi(mua)."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: bon cho trong phai GOI lai dung ham da co -- to_hop, la_xac_suat, ky_vong, trung_vi
  requireAst:
  - kind: uses-call, target: to_hop, min: 1
  - kind: uses-call, target: la_xac_suat, min: 1
  - kind: uses-call, target: ky_vong, min: 1
  - kind: uses-call, target: trung_vi, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(6, 0\.5, 2\.0, 40\.5\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn assert, không assert nào là khái niệm mới. Vậy mà chúng kiểm
được NGUYÊN một vụ mùa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp — câu cuối của track này.

Nhìn lại ba mươi ba bài vừa qua. Bài 1 mở bằng quy tắc cộng — trường
hợp RIÊNG của bù trừ T2.4 đã dạy. Bài 11 định nghĩa xác suất CHỈ LÀ
một tập con của không gian mẫu — để MỌI kỹ thuật tập hợp (T2.4) dùng
LẠI nguyên vẹn. Bài 22 định nghĩa biến ngẫu nhiên QUA ánh xạ — để kỳ
vọng, phương sai chỉ LÀ những phép tính KHÁC trên đúng một hàm số.
Bốn tầng, MỘT nền — bài 34 vừa kiểm CẢ BỐN cùng lúc, trên đúng MỘT
vụ mùa.

Mọi luống giờ có: một TẬP loại rau (T2.4), một chỗ trong quan hệ
tưới (T2.4), một người phụ trách (T2.4), VÀ giờ THÊM một dãy con số
kilôgam thu hoạch mỗi mùa (T2.5) — đếm được, đo được, dự đoán được.
NHƯNG "luống NÀO nên trồng gì, khi NÀO" — một câu hỏi cần nhìn XA
hơn một vụ, sang cấu trúc LẶP LẠI qua thời gian — track NÀY chưa trả
lời được. Cấu trúc lặp lại ấy viết ra thành cái gì mà tính toán
được?
::::

::::checkpoint{mastery=0.85}
::::
