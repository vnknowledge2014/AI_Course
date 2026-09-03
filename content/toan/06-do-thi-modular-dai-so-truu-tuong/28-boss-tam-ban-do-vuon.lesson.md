---
id: toan.do-thi-modular-dai-so-truu-tuong.boss-tam-ban-do-vuon
title: "BOSS — Tấm bản đồ vườn"
summary: "Ghép đồ thị (bậc, liên thông, bài 1-12) + modular (đồng dư, GCD, nghịch đảo, bài 13-20) + đại số trừu tượng (kiểm bốn tiên đề nhóm, bài 21-27) TRÊN MỘT sơ đồ tưới nước hoàn chỉnh của khu vườn."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
requires: [math.connected-graph, math.euler-existence-theorem, math.graph-coloring, math.generalized-modular-arithmetic, math.group, math.function-as-value]
concepts: [math.boss-t26]
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
Đồ thị, modular, đại số trừu tượng — BA chủ đề tưởng KHÔNG liên
quan. Sẵn sàng ghép TẤT CẢ lên MỘT tấm bản đồ vườn chưa?
::::

::::explain{#tam-ban-do-vuon}
Track NÀY dạy BA chủ đề dùng CHUNG một Ý TƯỞNG cốt LÕI: **cấu trúc
LẶP LẠI/ĐÓNG dưới một phép toán**. Đồ thị: bậc/liên thông LÀ tính
chất BẤT BIẾN dưới các phép biến ĐỔI (bài 1-12). Modular: phép
cộng/nhân "quay VÒNG" rồi ĐÓNG lại trong một tập hữu HẠN (bài
13-20). Nhóm: bốn tiên đề mô TẢ CHÍNH XÁC khi nào một phép toán
"đóng VÀ quay vòng TỐT" (bài 21-27). Một "tấm BẢN đồ vườn" hoàn
chỉnh cần CẢ hai điều kiện — sơ đồ TƯỚI LIÊN THÔNG, VÀ lịch tưới
theo chu KỲ `n` phải LÀ một nhóm modular:

```python title=readonly
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

def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)

def ban_do_vuon_hoan_chinh(luong, E, n):
    if not la_lien_thong(luong, E):
        return False
    return la_nhom(set(range(n)), lambda a, b: (a + b) % n)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_4"), ("luong_4", "luong_3")}

print(ban_do_vuon_hoan_chinh(luong, E, 3))
```

```text title=readonly
True
```

BỐN luống nối LIỀN thành một đường (liên thông, bài 5) VÀ lịch tưới
chu kỳ BA ngày LÀ một nhóm modular (bài 25, 26) — CẢ hai điều kiện
ĐỀU thoả, tấm bản đồ vườn HOÀN chỉnh.
::::

::::example{#dut-doan}
BỚT một ống — sơ đồ ĐỨT đoạn, DÙ lịch tưới VẪN LÀ một nhóm HOÀN
hảo:

```python title=readonly
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

def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)

def ban_do_vuon_hoan_chinh(luong, E, n):
    if not la_lien_thong(luong, E):
        return False
    return la_nhom(set(range(n)), lambda a, b: (a + b) % n)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E_dut = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(ban_do_vuon_hoan_chinh(luong, E_dut, 3))
```

```text title=readonly
False
```

BỚT ống nối `luong_3-luong_4` — `luong_4` bị CÔ LẬP, đồ thị KHÔNG
còn liên thông. Lịch tưới `n=3` VẪN LÀ một nhóm modular HOÀN hảo,
NHƯNG `ban_do_vuon_hoan_chinh` trả VỀ `False` NGAY (kiểm liên thông
TRƯỚC) — CẦN CẢ hai điều kiện, KHÔNG chỉ một.
::::

::::predict{#doan-doi-chu-ky commitOnce}
Byte đổi lịch tưới TỪ chu kỳ BA ngày sang chu kỳ MƯỜI HAI giờ
(`n=12`, đúng "đồng hồ" bài 20), TRÊN sơ đồ vườn LIÊN THÔNG:

```python
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

def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)

def ban_do_vuon_hoan_chinh(luong, E, n):
    if not la_lien_thong(luong, E):
        return False
    return la_nhom(set(range(n)), lambda a, b: (a + b) % n)

luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_4"), ("luong_4", "luong_3")}
print(ban_do_vuon_hoan_chinh(luong, E, 12))
```

Dòng cuối in ra gì?

:::opt{correct}
`True`
:::

:::opt
`False` — vì `n=12` LỚN hơn HẲN số luống (`4`), VÀ số CHU kỳ PHẢI
KHỚP đúng SỐ đỉnh của đồ thị để tấm bản đồ HOÀN chỉnh
::why
Gần đúng ở việc bạn để ý ĐÚNG `12` khác HẲN `4` — một quan sát VỀ
CON số.

Chỗ lệch: HAI điều kiện của `ban_do_vuon_hoan_chinh` HOÀN TOÀN ĐỘC
LẬP — liên thông ĐỒ thị (phụ THUỘC `luong`, `E`) VÀ nhóm modular
(phụ THUỘC `n`) KHÔNG cần "khớp SỐ" với nhau GÌ cả. `(ℤₙ, +mod n)`
LUÔN LÀ một nhóm VỚI **MỌI** `n` (bài 26), BẤT kể `n` bằng bao
nhiêu SO với số luống.
::
:::

:::opt
Máy báo lỗi khi chạy — hàm `ban_do_vuon_hoan_chinh` được GỌI hai
lần TRONG cùng phiên (một Ở dòng TRƯỚC cho `n=3`, MỘT Ở đây CHO
`n=12`), Python không cho GỌI LẶP một hàm VỚI đối số khác nhau
::why
Gần đúng ở việc bạn để ý ĐÚNG hàm được GỌI NHIỀU lần — một quan sát
VỀ hình thức đúng.

Chỗ lệch: GỌI LẶP một hàm VỚI đối số KHÁC nhau LÀ điều BÌNH thường
NHẤT trong lập trình — KHÔNG có RÀNG buộc nào ngăn CẤM. Biên dịch
sạch, chạy sạch.
::
:::
::::

::::code{#viet_ban_do_vuon_hoan_chinh}
Viết `ban_do_vuon_hoan_chinh(luong, E, n)` — kiểm sơ đồ tưới
`(luong, E)` LIÊN THÔNG, VÀ lịch tưới chu kỳ `n` LÀ một nhóm
modular.

```python title=starter
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

def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)

def ban_do_vuon_hoan_chinh(luong, E, n):
    if not ___:
        return False
    return ___


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_4"), ("luong_4", "luong_3")}

print(ban_do_vuon_hoan_chinh(luong, E, 3))
```

```python title=solution
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

def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)

def ban_do_vuon_hoan_chinh(luong, E, n):
    if not la_lien_thong(luong, E):
        return False
    return la_nhom(set(range(n)), lambda a, b: (a + b) % n)


luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_4"), ("luong_4", "luong_3")}

print(ban_do_vuon_hoan_chinh(luong, E, 3))
```

```python title=test
assert ban_do_vuon_hoan_chinh(set(), set(), 3) is True, "vuon rong -- lien thong (chan ly rong) va n=3 la nhom"
luong = {"luong_1", "luong_2", "luong_3", "luong_4"}
E = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_4"), ("luong_4", "luong_3")}
assert ban_do_vuon_hoan_chinh(luong, E, 3) is True, "vuon lien thong, chu ky 3 -- hoan chinh"
assert ban_do_vuon_hoan_chinh(luong, E, 12) is True, "vuon lien thong, chu ky 12 -- van hoan chinh, n bat ky deu la nhom"
E_dut = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
assert ban_do_vuon_hoan_chinh(luong, E_dut, 3) is False, "vuon dut doan -- khong hoan chinh du lich tuoi van la nhom"
```

:::hints
- kind: attention
  body: "Blank dau: kiem la_lien_thong(luong, E). Blank hai: kiem la_nhom tren set(range(n)) voi cong modular."
- kind: strategy
  body: "la_lien_thong(luong, E) ... la_nhom(set(range(n)), lambda a, b: (a + b) % n)"
- kind: one-line
  body: "if not la_lien_thong(luong, E): return False\\nreturn la_nhom(set(range(n)), lambda a, b: (a + b) % n)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem la_lien_thong(luong,E) truoc, roi la_nhom tren chu ky n
  requireAst:
  - kind: uses-call, target: la_lien_thong, min: 1
  - kind: uses-call, target: la_nhom, min: 1
  - kind: uses-call, target: range, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tấm bản đồ vườn HOÀN CHỈNH — đồ thị, modular, đại số trừu tượng gặp
nhau TRÊN CÙNG một khu vườn. Nhưng "MẤY cây", "MẤY kilôgam" LÀ số
RỜI RẠC — làm sao ĐO một luống dài BAO NHIÊU MÉT, TRƠN VÀ liên tục?
::::

::::reflect{#nghi-lai}
Track NÀY đóng bằng việc NHÌN lại đồ thị VÀ modular QUA lăng kính
nhóm. NHƯNG "mấy CÂY", "mấy KILÔGAM" (T2.5) LÀ số RỜI RẠC — máy tính
CÒN cần XỬ LÝ số LIÊN TỤC (đo đạc, dự đoán TRƠN) để LÀM được AI
thật. Số liên tục VÀ phép TÍNH trên nó viết RA thành CÁI GÌ?
::::

::::checkpoint{mastery=0.85}
::::
