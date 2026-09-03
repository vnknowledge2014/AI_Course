---
id: toan.do-thi-modular-dai-so-truu-tuong.dieu-kien-ton-tai-duong-di-euler
title: Điều kiện tồn tại đường đi Euler
summary: "Định lý Euler — một đồ thị liên thông có chu trình Euler ⟺ MỌI đỉnh bậc CHẴN; có đường đi Euler ⟺ ĐÚNG 0 hoặc 2 đỉnh bậc LẺ — giải quyết TRỌN VẸN bài toán Königsberg (bốn đỉnh bậc lẻ, bài 9)."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.euler-existence-theorem]
requires: [math.euler-path, math.handshake-lemma]
concepts: [math.dinh-ly-euler]
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
Königsberg CÓ bốn vùng bậc lẻ (bài 9) — KHÔNG đường Euler nào. Đường
thẳng của Byte CHỈ có HAI đỉnh bậc lẻ, VÀ CÓ đường Euler (bài 10).
Ranh giới NẰM Ở ĐÂU?
::::

::::explain{#dinh-ly-euler}
NẰM Ở con số `0` VÀ `2`. **Định lý Euler**: một đồ thị liên thông
CÓ chu trình Euler (khép kín) ⟺ MỌI đỉnh bậc CHẴN; CÓ đường đi Euler
(không cần đóng vòng) ⟺ ĐÚNG `0` HOẶC `2` đỉnh bậc LẺ — KHÔNG con số
nào KHÁC được phép:

```python title=readonly
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def la_lien_thong(luong, E):
    if not luong:
        return True
    v = next(iter(luong))
    return dinh_toi_duoc(v, E) == luong

def dem_dinh_bac_le(luong, E):
    return sum(1 for v in luong if bac(v, E) % 2 == 1)

def co_duong_di_euler(luong, E):
    if not la_lien_thong(luong, E):
        return False
    return dem_dinh_bac_le(luong, E) in {0, 2}


luong = {"luong_1", "luong_2", "luong_3"}
E_vong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}

print(co_duong_di_euler(luong, E_vong))
```

```text title=readonly
True
```

Tam giác — MỌI đỉnh bậc `2` (chẵn), `dem_dinh_bac_le` trả VỀ `0` —
`0 ∈ {0,2}` — CÓ đường Euler (chính LÀ chu trình Euler bài 10 ĐÃ
thấy).
::::

::::example{#konigsberg-that-bai}
Áp dụng ĐÚNG định lý lên số liệu Königsberg (bài 9: `dem_dinh_bac_le
= 4`):

```python title=readonly
bac_konigsberg = {"A": 5, "B": 3, "C": 3, "D": 3}
dem_le = sum(1 for v in bac_konigsberg if bac_konigsberg[v] % 2 == 1)

print(dem_le in {0, 2})
```

```text title=readonly
False
```

`4 ∉ {0,2}` — ĐÚNG như Euler CHỨNG minh năm 1736: KHÔNG đường đi
nào đi qua ĐỦ bảy cầu, MỖI cầu MỘT lần. Bài toán MỞ Ở bài 9 giờ ĐÃ
GIẢI trọn vẹn, KHÔNG cần thử ĐI thử một lần nào.
::::

::::predict{#doan-hai-tam-giac-tach-biet commitOnce}
Byte dựng HAI tam giác TÁCH biệt (KHÔNG cạnh nào nối chúng) — MỌI
đỉnh vẫn bậc `2` (chẵn):

```python
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def la_lien_thong(luong, E):
    if not luong:
        return True
    v = next(iter(luong))
    return dinh_toi_duoc(v, E) == luong

def dem_dinh_bac_le(luong, E):
    return sum(1 for v in luong if bac(v, E) % 2 == 1)

def co_duong_di_euler(luong, E):
    if not la_lien_thong(luong, E):
        return False
    return dem_dinh_bac_le(luong, E) in {0, 2}

luong6 = {"luong_1", "luong_2", "luong_3", "luong_4", "luong_5", "luong_6"}
E_tg1 = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}
E_tg2 = {("luong_4", "luong_5"), ("luong_5", "luong_4"), ("luong_5", "luong_6"), ("luong_6", "luong_5"), ("luong_6", "luong_4"), ("luong_4", "luong_6")}
E = E_tg1 | E_tg2
print(co_duong_di_euler(luong6, E))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `dem_dinh_bac_le(luong6, E)` trả VỀ `0` (MỌI đỉnh bậc
chẵn, ĐÚNG điều kiện `0 ∈ {0,2}`), VÀ định lý Euler CHỈ nói VỀ bậc
::why
Gần đúng ở việc bạn tính ĐÚNG `dem_dinh_bac_le` — MỌI đỉnh Ở CẢ hai
tam giác đều bậc `2`, dem RA `0`, một phép TÍNH chính xác.

Chỗ lệch: định lý Euler đòi CẢ HAI điều kiện — "đồ thị LIÊN THÔNG"
VÀ "0 hoặc 2 đỉnh bậc lẻ", KHÔNG chỉ điều kiện bậc MỘT mình (đúng
LỐI bài 7: số cạnh khớp `n−1` chưa đủ, còn cần liên thông). HAI tam
giác TÁCH biệt — `la_lien_thong` trả `False` NGAY, `co_duong_di_euler`
trả VỀ `False`, CHƯA BAO GIỜ chạy tới phép đếm bậc lẻ.
::
:::

:::opt
Máy báo lỗi khi chạy — `E = E_tg1 | E_tg2` GỘP hai tập cạnh, NHƯNG
`luong6` LẠI khai BÁO trước KHI `E` được tính, Python đòi biến PHẢI
khai theo ĐÚNG thứ tự sử DỤNG
::why
Gần đúng ở việc bạn để ý thứ tự khai BÁO CÁC biến trong đoạn mã —
một quan sát VỀ CẤU TRÚC.

Chỗ lệch: `luong6` VÀ `E` LÀ hai biến TÁCH biệt, KHÔNG phụ thuộc lẫn
nhau VỀ thứ tự khai báo (chỉ cần MỖI biến được gán TRƯỚC khi dùng —
`luong6` dùng Ở dòng CUỐI, sau khi CẢ hai đã có giá trị). Biên dịch
sạch, chạy sạch.
::
:::
::::

::::code{#viet_co_duong_di_euler}
Viết PHẦN CÒN LẠI của `co_duong_di_euler(luong, E)` — áp dụng định
lý Euler: liên thông VÀ đúng `0` hoặc `2` đỉnh bậc lẻ.

```python title=starter
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def la_lien_thong(luong, E):
    if not luong:
        return True
    v = next(iter(luong))
    return dinh_toi_duoc(v, E) == luong

def dem_dinh_bac_le(luong, E):
    return sum(1 for v in luong if bac(v, E) % 2 == 1)

def co_duong_di_euler(luong, E):
    if not la_lien_thong(luong, E):
        return False
    return ___


luong = {"luong_1", "luong_2", "luong_3"}
E_duong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(co_duong_di_euler(luong, E_duong))
```

```python title=solution
def bac(v, E):
    return sum(1 for (a, b) in E if a == v)

def dinh_toi_duoc(v, E):
    da_tham = {v}
    while True:
        moi = da_tham | {b for (a, b) in E if a in da_tham}
        if moi == da_tham:
            return da_tham
        da_tham = moi

def la_lien_thong(luong, E):
    if not luong:
        return True
    v = next(iter(luong))
    return dinh_toi_duoc(v, E) == luong

def dem_dinh_bac_le(luong, E):
    return sum(1 for v in luong if bac(v, E) % 2 == 1)

def co_duong_di_euler(luong, E):
    if not la_lien_thong(luong, E):
        return False
    return dem_dinh_bac_le(luong, E) in {0, 2}


luong = {"luong_1", "luong_2", "luong_3"}
E_duong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(co_duong_di_euler(luong, E_duong))
```

```python title=test
assert co_duong_di_euler(set(), set()) is True, "do thi rong -- lien thong (chan ly rong) va 0 dinh bac le"
assert co_duong_di_euler({"x"}, set()) is True, "mot dinh, khong canh -- lien thong, bac 0 (chan)"
luong = {"luong_1", "luong_2", "luong_3"}
E_vong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}
assert co_duong_di_euler(luong, E_vong) is True, "tam giac -- 0 dinh bac le"
E_duong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
assert co_duong_di_euler(luong, E_duong) is True, "duong thang -- 2 dinh bac le"
bac_konigsberg = {"A": 5, "B": 3, "C": 3, "D": 3}
dem_le = sum(1 for v in bac_konigsberg if bac_konigsberg[v] % 2 == 1)
assert dem_le not in {0, 2}, "konigsberg co 4 dinh bac le -- khong khop dieu kien"
```

:::hints
- kind: attention
  body: "Dung dem_dinh_bac_le(luong, E) da co, kiem no co nam trong {0, 2} khong."
- kind: strategy
  body: "dem_dinh_bac_le(luong, E) in {0, 2}"
- kind: one-line
  body: "___ = dem_dinh_bac_le(luong, E) in {0, 2}"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai goi dem_dinh_bac_le(luong, E) va kiem ket qua co trong {0, 2} khong
  requireAst:
  - kind: uses-call, target: dem_dinh_bac_le, min: 1
  - kind: has-literal, target: '0', min: 1
  - kind: has-literal, target: '2', min: 2
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Königsberg giờ đã có câu trả lời TRỌN VẸN — KHÔNG thể. Sang một bài
toán đồ thị KHÁC hẳn: xếp lịch tưới sao KHÔNG hai luống KỀ nhau tưới
CÙNG giờ?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bốn luống, MỌI cặp ĐỀU kề nhau (đồ thị ĐẦY ĐỦ — MỖI luống nối VỚI cả
ba luống còn lại). Cần ÍT NHẤT bao nhiêu "giờ tưới" khác nhau để
KHÔNG hai luống kề nào tưới CÙNG giờ?
::::

::::checkpoint{mastery=0.8}
::::
