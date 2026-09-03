---
id: toan.tap-hop-quan-he-anh-xa.boss-khu-vuon-day-du
title: "BOSS — Khu vườn đầy đủ"
summary: "Hai mươi bảy bài ghép lại trên MỘT khu vườn bốn luống: tập hợp (⊆/∪/∩), quan hệ tương đương (cùng lịch tưới), và ánh xạ song ánh (ai phụ trách luống nào) — cùng lúc, cùng một dữ liệu."
locale: vi
track: toan
module: tap-hop-quan-he-anh-xa
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [math.subset, math.set-union, math.set-intersection, math.equivalence-relation, math.bijective]
requires: [math.set, math.set-element, math.set-not-element, math.set-builder, math.set-empty, math.subset, math.set-equality, math.set-union, math.set-intersection, math.set-disjoint, math.set-difference, math.set-complement, math.inclusion-exclusion, math.ordered-pair, math.cartesian-product, math.relation, math.relation-table, math.relation-domain-range, math.relation-reflexive, math.relation-symmetric, math.relation-transitive, math.equivalence-relation, math.equivalence-class, math.function-as-relation, math.injective, math.surjective, math.bijective, math.relation-composition]
concepts: [math.khu-vuon-day-du, math.ba-manh-ghep-toan]
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
Hai mươi bảy bài, ba mảnh riêng: tập hợp, quan hệ, ánh xạ. Byte muốn
kiểm CẢ BA cùng lúc, trên đúng MỘT khu vườn. Ghép được không?
::::

::::explain{#khu-vuon-bon-luong}
Không có khái niệm mới ở bài này — CHỈ ghép lại. Khu vườn có bốn
luống, mỗi luống mang BA thứ đã học:

```python title=readonly
tap_rau = {
    "luong_1": {"ca_chua", "xa_lach"},
    "luong_2": {"ca_rot", "xa_lach"},
    "luong_3": {"ca_chua", "xa_lach"},
    "luong_4": {"bap_cai"},
}
lich = {
    "luong_1": {"Hai", "Nam"},
    "luong_2": {"Ba"},
    "luong_3": {"Hai", "Nam"},
    "luong_4": {"Ba"},
}
phan_cong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu"), ("luong_4", "Hoa")}
```

- **Tập hợp** (bài 1-13): `tap_rau` gán MỖI luống một tập rau —
  đúng cách viết của bài 1.
- **Quan hệ** (bài 14-23): `lich` gán MỖI luống một tập THỨ tưới;
  "cùng lịch tưới" LÀ quan hệ tương đương (bài 22) DỰNG từ nó.
- **Ánh xạ** (bài 24-27): `phan_cong` LÀ tập cặp `(luống, người)` —
  bốn luống, bốn người KHÁC nhau, không ai đứng NGOÀI.

Việc còn lại: viết MỘT hàm kiểm CẢ BA mảnh trên đúng dữ liệu này.
::::

::::example{#ba-manh-rieng-le}
Trước khi ghép, xem TỪNG mảnh cho ra gì — dùng lại đúng những hàm
bài 9 (giao), bài 6 (tập con), bài 25-26 (đơn ánh, toàn ánh) đã viết:

```python title=readonly
def giao(a, b):
    return a & b

def la_tap_con(a, b):
    return all(x in b for x in a)

def la_song_anh(phan_cong, b):
    nguoi_co_viec = {y for (x, y) in phan_cong}
    don_anh = len(nguoi_co_viec) == len(phan_cong)
    toan_anh = nguoi_co_viec == b
    return don_anh and toan_anh


tap_rau = {"luong_1": {"ca_chua", "xa_lach"}, "luong_3": {"ca_chua", "xa_lach"}}
print(sorted(giao(tap_rau["luong_1"], tap_rau["luong_3"])))

phan_cong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu"), ("luong_4", "Hoa")}
nguoi_lam_vuon = {"Lan", "Minh", "Tu", "Hoa"}
print(la_song_anh(phan_cong, nguoi_lam_vuon))
```

```text title=readonly
['ca_chua', 'xa_lach']
True
```

`luong_1` VÀ `luong_3` chung `ca_chua` VÀ `xa_lach`. Phân công BỐN
người BỐN luống, KHÔNG ai trùng, KHÔNG ai đứng ngoài — song ánh.
KHÔNG hàm nào Ở TRÊN LÀ mới; cả ba đã có TRƯỚC bài này.
::::

::::predict{#doan-hong-mot-manh commitOnce}
Byte đổi `phan_cong` — Lan phụ trách CẢ `luong_1` lẫn `luong_2`,
`luong_4` bị bỏ TRỐNG (không ai phụ trách). Hai mảnh ĐẦU (tập hợp,
quan hệ) KHÔNG đổi gì. Gọi lại hàm `kiem_vuon()` (ghép cả ba mảnh
bằng `assert`, viết Ở bước sau). Điều gì xảy ra?

```python
phan_cong_hong = {("luong_1", "Lan"), ("luong_2", "Lan"), ("luong_3", "Tu")}
# ... kiem_vuon() chạy qua mảnh tập hợp, rồi mảnh quan hệ, rồi tới
# assert cuối cùng kiểm phan_cong_hong có song ánh không.
```

Chuyện gì xảy ra khi gọi `kiem_vuon()` với `phan_cong_hong`?

:::opt{correct}
Chương trình DỪNG ngay tại `assert` kiểm song ánh — hai mảnh ĐẦU đã
chạy qua trót lọt, NHƯNG dòng `return` không bao giờ tới
:::

:::opt
Hàm vẫn trả VỀ đủ ba giá trị NHƯ cũ, chỉ riêng phần song ánh đổi
thành `False` — `assert` chỉ GHI NHẬN câu Đ/S, không CHẶN chương
trình chạy tiếp
::why
Gần đúng ở việc bạn tính ĐÚNG `la_song_anh(phan_cong_hong, ...)` LÀ
`False` (Lan trùng luống, `luong_4` bỏ trống) — quan sát ĐÓ chính
xác.

Chỗ lệch: `assert` KHÔNG PHẢI một câu ghi chú — GẶP điều kiện SAI, nó
NÉM `AssertionError` VÀ dừng hàm NGAY LẬP TỨC, y hệt bài 32 T2.3 đã
dạy. Dòng `return sorted(rau_chung), la_tuong_duong, song_anh` nằm
SAU `assert` cuối — KHÔNG BAO GIỜ được chạy tới. Hàm KHÔNG trả VỀ gì
cả; nó NÉM lỗi.
::
:::

:::opt
Chương trình lỗi NGAY từ mảnh ĐẦU tiên (tập hợp) — VÌ `phan_cong_hong`
bị đổi thì TOÀN BỘ dữ liệu bên trong hàm coi như KHÔNG còn đáng tin,
mảnh nào cũng phải chạy LẠI từ đầu VÀ báo lỗi
::why
Gần đúng ở việc bạn nghĩ TỚI một sự cố LAN RỘNG — đổi một biến thì cả
hàm "không đáng tin" nữa, một trực giác thận trọng.

Chỗ lệch: `giao`, `la_tap_con` (mảnh tập hợp) VÀ ba biến `phan_xa`,
`doi_xung`, `bac_cau` (mảnh quan hệ) KHÔNG hề đọc `phan_cong` — chúng
CHỈ dùng `tap_rau` VÀ `lich`, cả hai đều KHÔNG đổi. Hai mảnh ĐẦU chạy
qua TRÓT LỌT, không một `assert` nào Ở đó bị chạm tới; lỗi CHỈ nổ ra
Ở đúng `assert` cuối, nơi `phan_cong` MỚI thật sự được đọc tới.
::
:::
::::

::::code{#viet_kiem_vuon}
Ba hàm bên dưới ĐÃ viết sẵn — không hàm nào mới. Việc của bạn: ghép
CHÚNG lại thành `kiem_vuon()`, điền đúng BA chỗ trống, MỖI chỗ ứng
với MỘT mảnh (tập hợp, quan hệ, ánh xạ).

```python title=starter
def giao(a, b):
    return a & b

def hop(a, b):
    return a | b

def la_tap_con(a, b):
    return all(x in b for x in a)

def la_tuong_duong_tu(phan_xa, doi_xung, bac_cau):
    return ___

def la_song_anh(phan_cong, b):
    nguoi_co_viec = {y for (x, y) in phan_cong}
    don_anh = len(nguoi_co_viec) == len(phan_cong)
    toan_anh = nguoi_co_viec == b
    return don_anh and toan_anh


def kiem_vuon():
    tap_rau = {
        "luong_1": {"ca_chua", "xa_lach"},
        "luong_2": {"ca_rot", "xa_lach"},
        "luong_3": {"ca_chua", "xa_lach"},
        "luong_4": {"bap_cai"},
    }
    lich = {
        "luong_1": {"Hai", "Nam"},
        "luong_2": {"Ba"},
        "luong_3": {"Hai", "Nam"},
        "luong_4": {"Ba"},
    }
    luong = set(tap_rau.keys())
    quan_he_tuoi = {(a, b) for a in luong for b in luong if lich[a] == lich[b]}
    nguoi_lam_vuon = {"Lan", "Minh", "Tu", "Hoa"}
    phan_cong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu"), ("luong_4", "Hoa")}

    # Mảnh 1 -- tập hợp: luong_1 và luong_3 trồng CHUNG loại rau nào?
    rau_chung = ___
    assert rau_chung == {"ca_chua", "xa_lach"}, "giao cua tap rau luong_1 va luong_3"
    assert la_tap_con(rau_chung, hop(tap_rau["luong_1"], tap_rau["luong_2"])), "phan giao van la tap con cua hop"

    # Mảnh 2 -- quan hệ: "cùng lịch tưới" có phải tương đương không?
    phan_xa = all((x, x) in quan_he_tuoi for x in luong)
    doi_xung = all((b, a) in quan_he_tuoi for (a, b) in quan_he_tuoi)
    bac_cau = all((a, c) in quan_he_tuoi for (a, b) in quan_he_tuoi for (b2, c) in quan_he_tuoi if b == b2)
    la_tuong_duong = la_tuong_duong_tu(phan_xa, doi_xung, bac_cau)
    assert la_tuong_duong is True, "phan xa, doi xung, bac cau -- du ca ba"

    # Mảnh 3 -- ánh xạ: phân công phụ trách có song ánh không?
    song_anh = ___
    assert song_anh is True, "bon nguoi, bon luong, khop khit -- doi nguoc duoc"

    return sorted(rau_chung), la_tuong_duong, song_anh


print(kiem_vuon())
```

```python title=solution
def giao(a, b):
    return a & b

def hop(a, b):
    return a | b

def la_tap_con(a, b):
    return all(x in b for x in a)

def la_tuong_duong_tu(phan_xa, doi_xung, bac_cau):
    return phan_xa and doi_xung and bac_cau

def la_song_anh(phan_cong, b):
    nguoi_co_viec = {y for (x, y) in phan_cong}
    don_anh = len(nguoi_co_viec) == len(phan_cong)
    toan_anh = nguoi_co_viec == b
    return don_anh and toan_anh


def kiem_vuon():
    tap_rau = {
        "luong_1": {"ca_chua", "xa_lach"},
        "luong_2": {"ca_rot", "xa_lach"},
        "luong_3": {"ca_chua", "xa_lach"},
        "luong_4": {"bap_cai"},
    }
    lich = {
        "luong_1": {"Hai", "Nam"},
        "luong_2": {"Ba"},
        "luong_3": {"Hai", "Nam"},
        "luong_4": {"Ba"},
    }
    luong = set(tap_rau.keys())
    quan_he_tuoi = {(a, b) for a in luong for b in luong if lich[a] == lich[b]}
    nguoi_lam_vuon = {"Lan", "Minh", "Tu", "Hoa"}
    phan_cong = {("luong_1", "Lan"), ("luong_2", "Minh"), ("luong_3", "Tu"), ("luong_4", "Hoa")}

    # Mảnh 1 -- tập hợp: luong_1 và luong_3 trồng CHUNG loại rau nào?
    rau_chung = giao(tap_rau["luong_1"], tap_rau["luong_3"])
    assert rau_chung == {"ca_chua", "xa_lach"}, "giao cua tap rau luong_1 va luong_3"
    assert la_tap_con(rau_chung, hop(tap_rau["luong_1"], tap_rau["luong_2"])), "phan giao van la tap con cua hop"

    # Mảnh 2 -- quan hệ: "cùng lịch tưới" có phải tương đương không?
    phan_xa = all((x, x) in quan_he_tuoi for x in luong)
    doi_xung = all((b, a) in quan_he_tuoi for (a, b) in quan_he_tuoi)
    bac_cau = all((a, c) in quan_he_tuoi for (a, b) in quan_he_tuoi for (b2, c) in quan_he_tuoi if b == b2)
    la_tuong_duong = la_tuong_duong_tu(phan_xa, doi_xung, bac_cau)
    assert la_tuong_duong is True, "phan xa, doi xung, bac cau -- du ca ba"

    # Mảnh 3 -- ánh xạ: phân công phụ trách có song ánh không?
    song_anh = la_song_anh(phan_cong, nguoi_lam_vuon)
    assert song_anh is True, "bon nguoi, bon luong, khop khit -- doi nguoc duoc"

    return sorted(rau_chung), la_tuong_duong, song_anh


print(kiem_vuon())
```

```python title=test
assert kiem_vuon() == (["ca_chua", "xa_lach"], True, True), "khu vuon bon luong phai qua ca ba manh ghep"
assert giao({"a", "b"}, {"b", "c"}) == {"b"}, "giao dung nghia toan hoc, khong rieng cua khu vuon"
assert la_tap_con({"a"}, {"a", "b"}) is True, "tap con dung nghia toan hoc"
assert la_tuong_duong_tu(True, True, True) is True, "ca ba dieu kien deu dung thi tuong duong"
assert la_tuong_duong_tu(False, True, True) is False, "thieu phan xa thi khong con tuong duong"
assert la_tuong_duong_tu(True, False, True) is False, "thieu doi xung thi khong con tuong duong"
assert la_tuong_duong_tu(True, True, False) is False, "thieu bac cau thi khong con tuong duong"
assert la_song_anh({("x", "1"), ("y", "2")}, {"1", "2"}) is True, "song anh tren du lieu khac khu vuon"
assert la_song_anh({("x", "1"), ("y", "1")}, {"1", "2"}) is False, "khong don anh thi khong song anh, du la du lieu nao"
```

:::hints
- kind: attention
  body: "Ba cho trong ung voi ba manh da hoc: mang 1 (trong kiem_vuon) goi ham giao() tren tap_rau; mang 2 (trong ham la_tuong_duong_tu o TREN kiem_vuon) ghep ba tham so Đ/S bang and; mang 3 (trong kiem_vuon) goi ham la_song_anh() tren phan_cong."
- kind: strategy
  body: "Mang 1: giao(tap_rau[\"luong_1\"], tap_rau[\"luong_3\"]). Mang 2 (trong la_tuong_duong_tu): phan_xa and doi_xung and bac_cau. Mang 3: la_song_anh(phan_cong, nguoi_lam_vuon)."
- kind: one-line
  body: "Ba dong: giao(tap_rau[\"luong_1\"], tap_rau[\"luong_3\"]) -- roi (trong la_tuong_duong_tu) phan_xa and doi_xung and bac_cau -- roi la_song_anh(phan_cong, nguoi_lam_vuon)."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: ba cho trong phai GOI lai dung ham/bien da co -- mang 1 goi giao(), mang 2 ghep ba bien bang and, mang 3 goi la_song_anh()
  requireAst:
  - kind: uses-call, target: giao, min: 1
  - kind: uses-operator, target: 'and', min: 1
  - kind: uses-call, target: la_song_anh, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(\['ca_chua', 'xa_lach'\], True, True\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba assert, không assert nào là khái niệm mới. Vậy mà chúng kiểm được
NGUYÊN một khu vườn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp — câu cuối của track này.

Nhìn lại hai mươi bảy bài vừa qua. Bài 1 lấy cái rổ đã quen từ T1.4,
đặt tên toán học LÀ tập hợp. Bài 14 định nghĩa lại quan hệ CHỈ LÀ một
tập hợp CÁC cặp — để MỌI kỹ thuật của bài 1-13 dùng LẠI nguyên vẹn.
Bài 24 định nghĩa ánh xạ QUA quan hệ — để đơn ánh, toàn ánh chỉ LÀ
hai câu hỏi đếm KHÁC nhau trên đúng MỘT bảng phân công. Ba tầng, MỘT
nền — bài 28 vừa kiểm CẢ BA cùng lúc, trên đúng MỘT khu vườn.

Mọi luống giờ có: một tập rau, một chỗ trong quan hệ tưới, một người
phụ trách. NHƯNG "mấy CÂY mỗi loại", "mấy KILÔGAM thu hoạch" — tập
hợp KHÔNG trả lời được (bài 1 đã bỏ hẳn số lượng, chỉ giữ lại CÓ hay
KHÔNG). Track sau đếm được không, và đếm bằng cách nào?
::::

::::checkpoint{mastery=0.85}
::::
