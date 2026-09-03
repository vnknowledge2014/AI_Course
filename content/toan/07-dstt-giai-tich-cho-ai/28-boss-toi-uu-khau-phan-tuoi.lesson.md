---
id: toan.dstt-giai-tich-cho-ai.boss-toi-uu-khau-phan-tuoi
title: "BOSS — Tối ưu khẩu phần tưới"
summary: "Ghép vector (hồ sơ luống, bài 1-8) + ma trận (cả vườn, bài 9-15) + đạo hàm/gradient (bài 16-27): dùng gradient descent TÌM khẩu phần tưới TỐI ƯU giảm một hàm chi PHÍ đơn giản, VÀ dùng tích vô hướng SO sánh độ tương đồng hai luống."
locale: vi
track: toan
module: dstt-giai-tich-cho-ai
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
requires: [math.cosine-similarity, math.matrix-transpose, math.iterative-gradient-descent]
concepts: [math.boss-t27]
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
Track NÀY đóng bằng gradient descent — VECTOR, tích vô hướng, đạo
hàm, gradient — TẤT CẢ GẶP nhau Ở đâu trên MỘT bài toán DUY nhất?
::::

::::explain{#toi-uu-khau-phan-tuoi}
TRÊN bài toán "tìm khẩu phần TƯỚI tối ưu". Byte muốn TÌM khẩu phần
`(x,y)` (nước, phân bón) SAO cho GẦN NHẤT khẩu phần LÝ tưởng
`(2,3)` — dùng **hàm chi PHÍ** `(x−2)²+(y−3)²` (khoảng CÁCH bình
phương TỚI khẩu phần lý TƯỞNG, đúng lối bài 5) VÀ **gradient
descent** (bài 25-27) để TÌM `(x,y)` GIẢM chi phí ĐÓ VỀ gần `0`
nhất:

```python title=readonly
def chi_phi(x, y):
    return (x - 2) ** 2 + (y - 3) ** 2

def gradient_chi_phi(x, y):
    return (2 * (x - 2), 2 * (y - 3))

def mot_buoc(x, y, alpha):
    gx, gy = gradient_chi_phi(x, y)
    return (x - alpha * gx, y - alpha * gy)

def toi_uu(x, y, alpha, so_lan_lap):
    for _ in range(so_lan_lap):
        x, y = mot_buoc(x, y, alpha)
    return (x, y)


ket_qua = toi_uu(5.0, 7.0, 0.1, 100)
print(round(ket_qua[0], 4), round(ket_qua[1], 4))
print(round(chi_phi(*ket_qua), 6))
```

```text title=readonly
2.0 3.0
0.0
```

XUẤT phát TỪ khẩu phần `(5,7)` (QUÁ nhiều), LẶP `100` bước GRADIENT
descent — HỘI tụ CHÍNH XÁC về `(2,3)`, chi PHÍ VỀ `0`. Đây LÀ khẩu
phần TỐI ưu.
::::

::::example{#so-sanh-ho-so-cosine}
SO sánh độ tương đồng cosine (bài 8) GIỮA hai hồ sơ luống:

```python title=readonly
def tich_vo_huong(u, v):
    return sum(a * b for a, b in zip(u, v))

def do_dai(v):
    return sum(x ** 2 for x in v) ** 0.5

def do_tuong_dong_cosine(u, v):
    return tich_vo_huong(u, v) / (do_dai(u) * do_dai(v))


ho_so_a = (2.0, 3.0, 5.0)
ho_so_b = (4.0, 6.0, 10.0)

print(round(do_tuong_dong_cosine(ho_so_a, ho_so_b), 4))
```

```text title=readonly
1.0
```

`ho_so_b` LÀ `ho_so_a` nhân GẤP `2` — CÙNG tỉ lệ, độ tương ĐỒNG
cosine LÀ `1.0` — hai luống "GIỐNG hệt" VỀ hình DẠNG hồ sơ, dù độ
LỚN khác nhau.
::::

::::predict{#doan-diem-xuat-phat-khac commitOnce}
Byte tối ưu HOÁ, NHƯNG xuất phát TỪ khẩu phần `(0,0)` (QUÁ ít) thay
VÌ `(5,7)`:

```python
def chi_phi(x, y):
    return (x - 2) ** 2 + (y - 3) ** 2

def gradient_chi_phi(x, y):
    return (2 * (x - 2), 2 * (y - 3))

def mot_buoc(x, y, alpha):
    gx, gy = gradient_chi_phi(x, y)
    return (x - alpha * gx, y - alpha * gy)

def toi_uu(x, y, alpha, so_lan_lap):
    for _ in range(so_lan_lap):
        x, y = mot_buoc(x, y, alpha)
    return (x, y)

ket_qua = toi_uu(0.0, 0.0, 0.1, 100)
print(round(ket_qua[0], 4), round(ket_qua[1], 4))
```

Dòng cuối in ra gì?

:::opt{correct}
`2.0 3.0`
:::

:::opt
`0.0 0.0` — vì XUẤT phát TỪ `(0,0)`, gradient descent GIỮ nguyên
điểm bắt đầu nếu KHÔNG "tình cờ" ở gần khẩu phần LÝ tưởng
::why
Gần đúng ở việc bạn nghĩ điểm XUẤT phát QUYẾT định hoàn TOÀN kết
quả — một trực GIÁC cẩn TRỌNG (LO ngại "kẹt" ở điểm khởi đầu).

Chỗ lệch: hàm chi PHÍ `(x−2)²+(y−3)²` LÀ một "CÁI bát" LỒI DUY nhất
(chỉ MỘT đáy, KHÔNG có đáy GIẢ), NÊN gradient descent hội TỤ về
CÙNG MỘT điểm tối ưu `(2,3)` DÙ xuất phát TỪ bất KỲ đâu — `(5,7)`
hay `(0,0)` đều HỘI tụ VỀ cùng đích, CHỈ khác đường ĐI.
::
:::

:::opt
Máy báo lỗi khi chạy — `(0,0)` LÀ điểm GỐC, VÀ `gradient_chi_phi`
KHÔNG tính được đạo hàm TẠI CHÍNH gốc toạ độ
::why
Gần đúng ở việc bạn để ý ĐÚNG `(0,0)` LÀ điểm ĐẶC biệt — một quan
sát VỀ vị trí.

Chỗ lệch: `gradient_chi_phi` CHỈ LÀ công THỨC đại số (`2(x−2)`,
`2(y−3)`), tính được TẠI BẤT KỲ điểm nào, KỂ CẢ `(0,0)` — KHÔNG cần
xấp xỉ SỐ hay giới hạn Ở ĐÂY. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_toi_uu}
Viết `toi_uu(x, y, alpha, so_lan_lap)` — lặp `so_lan_lap` bước
gradient descent để TÌM khẩu phần TỐI ưu.

```python title=starter
def chi_phi(x, y):
    return (x - 2) ** 2 + (y - 3) ** 2

def gradient_chi_phi(x, y):
    return (2 * (x - 2), 2 * (y - 3))

def mot_buoc(x, y, alpha):
    gx, gy = gradient_chi_phi(x, y)
    return (x - alpha * gx, y - alpha * gy)

def toi_uu(x, y, alpha, so_lan_lap):
    for _ in range(so_lan_lap):
        ___
    return (x, y)


ket_qua = toi_uu(5.0, 7.0, 0.1, 1)
print(ket_qua)
```

```python title=solution
def chi_phi(x, y):
    return (x - 2) ** 2 + (y - 3) ** 2

def gradient_chi_phi(x, y):
    return (2 * (x - 2), 2 * (y - 3))

def mot_buoc(x, y, alpha):
    gx, gy = gradient_chi_phi(x, y)
    return (x - alpha * gx, y - alpha * gy)

def toi_uu(x, y, alpha, so_lan_lap):
    for _ in range(so_lan_lap):
        x, y = mot_buoc(x, y, alpha)
    return (x, y)


ket_qua = toi_uu(5.0, 7.0, 0.1, 1)
print(ket_qua)
```

```python title=test
assert toi_uu(5.0, 7.0, 0.1, 0) == (5.0, 7.0), "khong lap -- khong doi"
r1 = toi_uu(5.0, 7.0, 0.1, 100)
assert round(r1[0], 4) == 2.0 and round(r1[1], 4) == 3.0, "hoi tu tu (5,7)"
r2 = toi_uu(0.0, 0.0, 0.1, 100)
assert round(r2[0], 4) == 2.0 and round(r2[1], 4) == 3.0, "hoi tu tu (0,0) -- cung dich"
assert round(chi_phi(*r1), 4) == 0.0, "chi phi ve gan 0"
```

:::hints
- kind: attention
  body: "Goi mot_buoc(x, y, alpha) va gan lai CA x, y tu ket qua."
- kind: strategy
  body: "x, y = mot_buoc(x, y, alpha)"
- kind: one-line
  body: "x, y = mot_buoc(x, y, alpha)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi mot_buoc(x, y, alpha) va gan lai x, y
  requireAst:
  - kind: uses-call, target: mot_buoc, min: 1
  - kind: gan-ten, target: x, min: 1
  - kind: gan-ten, target: y, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(4\.4, 6\.2\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khẩu phần TỐI ưu — tìm được BẰNG gradient descent. Track NÀY khép
LẠI toàn bộ ROADMAP Toán (R2) — sáu track, MỘT bộ CÔNG cụ.
::::

::::reflect{#nghi-lai}
Track NÀY khép LẠI toàn bộ ROADMAP Toán (R2). Số rời rạc (đếm),
logic (chứng minh), tập hợp (nhóm lại), tổ hợp/xác suất (đo bất
ĐỊNH), đồ thị/modular/nhóm (cấu TRÚC lặp), VÀ giờ VECTOR/đạo hàm (đo
LIÊN tục VÀ tối ưu) — sáu track, MỘT bộ CÔNG cụ. Byte sẵn SÀNG bước
sang lập trình HÀM (R4) — CODE thật, KHÔNG chỉ Ý nghĩa toán.
::::

::::checkpoint{mastery=0.85}
::::
