---
id: tri-tue-nhan-tao.chunking-va-embedding-tu-che.boss-chia-va-tinh-vector-tai-lieu
title: "BOSS — Chia và tính vector tài liệu: ráp trọn quest, đóng q8.5a tại 6/6"
summary: "Mot tai lieu 4 cau tu nghi (2 cau ve cong nghe, 2 cau ve am thuc) duoc CHIA bang chia_theo_cau (bai 2) thanh dung 4 doan, TINH vector moi doan bang tinh_vector_dem_tu tren bo tu vung 16 muc (bai 4), roi DO tich_vo_huong (bai 5) giua CA SAU cap doan (C(4,2)=6 cap). Ket qua THAT: hai cap CUNG chu de cho dot product 2 va 2; bon cap KHAC chu de deu cho dot product 0. Dem duoc: DUNG 2/2 cap cung-chu-de co dot product CAO HON MOI cap khac-chu-de — bang chung so hoc cu the, khong suy doan. Dong quest q8.5a tai 6/6 bai."
locale: vi
track: tri-tue-nhan-tao
module: chunking-va-embedding-tu-che
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-chia-va-tinh-vector-tai-lieu]
requires: [ai.do-tuong-dong-hai-doan-van]
concepts: [ai.boss-chia-va-tinh-vector-tai-lieu]
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

::::byte{trigger=enter mood=happy pose=jump}
Năm bài: vì sao cần RAG, chia theo câu, cửa sổ trượt có chồng lấn, vector
đếm từ, tích vô hướng. Bài này ráp CẢ NĂM vào một quy trình duy nhất, trên
một tài liệu hoàn toàn MỚI — và đóng quest `q8.5a` tại `6/6`.
::::

::::explain{#rap_toan_bo_quy_trinh}
Một quy trình "chia và tính vector tài liệu" ĐÚNG CÁCH gồm ĐÚNG ba bước đã
xây, chạy theo thứ tự:

> **(a) Chia thành đoạn** (`chia-van-ban-thanh-doan`, bài `2`) — tách một
> tài liệu dài thành nhiều đoạn nhỏ, mỗi đoạn đủ ngắn để tính toán và truy
> xuất riêng lẻ.
>
> **(b) Tính vector mỗi đoạn** (`vector-tu-che-dem-tu`, bài `4`) — biến mỗi
> đoạn văn bản thành một vector đếm từ có độ dài CỐ ĐỊNH, trên MỘT bộ từ
> vựng chung cho toàn tài liệu.
>
> **(c) Đo tích vô hướng MỌI cặp đoạn** (`do-tuong-dong-hai-doan-van`, bài
> `5`) — với `N` đoạn, có `C(N,2)` cặp khác nhau; tính tích vô hướng cho
> TỪNG cặp.

Bài này dùng một tài liệu `4` câu, TỰ NGHĨ, chưa từng xuất hiện ở bài nào
trước — hai câu đầu về chủ đề công nghệ, hai câu sau về chủ đề ẩm thực (dùng
lại đúng bộ từ vựng `16` mục của bài `4`/`5`). Với `4` đoạn, số cặp là
`C(4,2) = 6`: hai cặp CÙNG chủ đề (đoạn `0`-`1`, đoạn `2`-`3`) và bốn cặp
KHÁC chủ đề (mọi cặp còn lại). *(Giữ `N = 4` — nhỏ, đủ minh hoạ — thay vì
một tài liệu lớn hơn: số cặp tăng theo `N²`, và mỗi cặp cần tính lại vector
+ tích vô hướng, không cần thiết cho một bài minh hoạ cơ chế.)*

Đo bằng số: đếm bao nhiêu cặp CÙNG chủ đề có tích vô hướng CAO HƠN **MỌI**
cặp KHÁC chủ đề — một bằng chứng số học cụ thể rằng cơ chế "chia → vector →
tích vô hướng" thật sự phân biệt được chủ đề, không phải trùng hợp trên một
ví dụ đã dựng sẵn từ trước.
::::

::::example{#boss_do_ca_sau_cap}
Tài liệu `4` câu tự nghĩ, chưa từng dùng ở bài nào trước — chạy trọn quy
trình ba bước trên nó:

```python title=readonly
import itertools

def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]
CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def phan_tich_tai_lieu(tai_lieu, chu_de, tu_vung, cum_tu_goc):
    doan = chia_theo_cau(tai_lieu)
    vec = [tinh_vector_dem_tu(d, tu_vung, cum_tu_goc) for d in doan]
    cung_chu_de = []
    khac_chu_de = []
    for i, j in itertools.combinations(range(len(doan)), 2):
        d = tich_vo_huong(vec[i], vec[j])
        if chu_de[i] == chu_de[j]:
            cung_chu_de.append(d)
        else:
            khac_chu_de.append(d)
    return doan, vec, cung_chu_de, khac_chu_de


TAI_LIEU_BOSS = (
    "may tinh hien dai giup lap trinh vien viet phan mem xu ly du lieu nhanh chong. "
    "vi xu ly ben trong may tinh chay thuat toan xu ly du lieu phuc tap trong tich tac. "
    "mon an ngon can co gia vi dam da va cong thuc nau chuan. "
    "dau bep gioi biet chon gia vi phu hop cho moi mon an va lam viec trong nha bep sach se."
)
CHU_DE = ["cong_nghe", "cong_nghe", "am_thuc", "am_thuc"]

doan, vec, cung_chu_de, khac_chu_de = phan_tich_tai_lieu(TAI_LIEU_BOSS, CHU_DE, TU_VUNG, CUM_TU_GOC)
so_cap_vuot_troi = sum(1 for d_cung in cung_chu_de if all(d_cung > d_khac for d_khac in khac_chu_de))

print(len(doan))
print(cung_chu_de)
print(khac_chu_de)
print(so_cap_vuot_troi, "/", len(cung_chu_de))
```

```text title=readonly
4
[2, 2]
[0, 0, 0, 0]
2 / 2
```

Tài liệu `4` câu chia đúng `4` đoạn (hai công nghệ, hai ẩm thực). `C(4,2) =
6` cặp, tách thành `2` cặp cùng chủ đề (`[2, 2]`) và `4` cặp khác chủ đề
(`[0, 0, 0, 0]`). Cả HAI cặp cùng chủ đề (`2` và `2`) đều CAO HƠN cả BỐN cặp
khác chủ đề (toàn `0`) — `so_cap_vuot_troi = 2 / 2`: `100%` số cặp cùng chủ
đề vượt qua MỌI cặp khác chủ đề, trên một tài liệu hoàn toàn mới, chưa từng
dùng để "luyện" các bài trước. Quy trình ba bước — chia, tính vector, đo
tích vô hướng — hoạt động đúng như thiết kế.
::::

::::predict{#doan_so_cap_vuot_troi commitOnce}
Xét đúng `TAI_LIEU_BOSS` (`4` câu, `2` công nghệ + `2` ẩm thực) ở ví dụ
trên — `cung_chu_de = [2, 2]`, `khac_chu_de = [0, 0, 0, 0]`.

**Trước khi chạy thử**, bạn đoán: `so_cap_vuot_troi` (số cặp cùng chủ đề có
tích vô hướng cao hơn **MỌI** cặp khác chủ đề) là bao nhiêu, trên tổng số
`2` cặp cùng chủ đề?

:::opt{correct}
`2 / 2` — cả hai giá trị trong `cung_chu_de` (`2` và `2`) đều LỚN HƠN cả
bốn giá trị trong `khac_chu_de` (toàn `0`); vì `2 > 0` đúng với MỌI cặp so
sánh, cả hai cặp cùng chủ đề đều vượt qua điều kiện "cao hơn MỌI cặp khác
chủ đề"
:::

:::opt
`1 / 2` — vì một trong hai cặp cùng chủ đề (đoạn `2`-`3`, về ẩm thực) có
tích vô hướng THẤP hơn cặp còn lại (đoạn `0`-`1`, về công nghệ), nên chỉ
MỘT cặp thật sự "vượt trội"
::why
Gần đúng ở việc nghi ngờ hai cặp cùng chủ đề có thể có giá trị KHÁC nhau —
một khả năng hợp lý nói chung (không có gì đảm bảo trước hai cặp phải bằng
nhau).

Chỗ lệch: trên đúng tài liệu này, cả hai cặp cùng chủ đề TÌNH CỜ đều cho
CÙNG một giá trị (`2` và `2`, bằng nhau) — không phải một cặp cao, một cặp
thấp. Và điều kiện "vượt trội" ở đây so với `khac_chu_de` (toàn `0`), không
so hai cặp cùng chủ đề VỚI NHAU — nên dù giá trị của chúng có bằng nhau hay
khác nhau, cả hai vẫn cùng vượt qua ngưỡng `0` như nhau.
::
:::

:::opt
`0 / 2` — vì với chỉ `4` câu ngắn, từ vựng chung giữa các đoạn quá ít để
tạo ra một tín hiệu tích vô hướng đủ mạnh, nên không cặp nào thật sự "vượt
trội" một cách đáng tin cậy
::why
Gần đúng ở lo ngại "tài liệu ngắn có thể cho tín hiệu yếu" — một trực giác
hợp lý khi làm việc với dữ liệu nhỏ nói chung.

Chỗ lệch: dù tài liệu ngắn, tích vô hướng vẫn hoạt động đúng cơ chế của nó
— chỉ cần MỘT chỉ số cùng khác `0` cũng đủ cho tích vô hướng dương, và ở
đây có `4` chỉ số như vậy cho mỗi cặp cùng chủ đề (`= 2`, xem bài `4`/`5`).
Bốn cặp khác chủ đề đều cho ĐÚNG `0` (không phải một số dương nhỏ, gần
`0`) — nên ranh giới `2 > 0` không hề mong manh, nó tuyệt đối rõ ràng.
::
:::
::::

::::code{#viet_phan_tich_tai_lieu}
Hoàn thiện `phan_tich_tai_lieu`: phân loại đúng mỗi cặp dot product vào
danh sách CÙNG chủ đề hay KHÁC chủ đề.

```python title=starter
import itertools

def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]
CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def phan_tich_tai_lieu(tai_lieu, chu_de, tu_vung, cum_tu_goc):
    doan = chia_theo_cau(tai_lieu)
    vec = [tinh_vector_dem_tu(d, tu_vung, cum_tu_goc) for d in doan]
    cung_chu_de = []
    khac_chu_de = []
    for i, j in itertools.combinations(range(len(doan)), 2):
        d = tich_vo_huong(vec[i], vec[j])
        if chu_de[i] == chu_de[j]:
            ___.append(d)                                  # cung_chu_de
        else:
            ___.append(d)                                  # khac_chu_de
    return doan, vec, cung_chu_de, khac_chu_de


TAI_LIEU_BOSS = (
    "may tinh hien dai giup lap trinh vien viet phan mem xu ly du lieu nhanh chong. "
    "vi xu ly ben trong may tinh chay thuat toan xu ly du lieu phuc tap trong tich tac. "
    "mon an ngon can co gia vi dam da va cong thuc nau chuan. "
    "dau bep gioi biet chon gia vi phu hop cho moi mon an va lam viec trong nha bep sach se."
)
CHU_DE = ["cong_nghe", "cong_nghe", "am_thuc", "am_thuc"]

doan, vec, cung_chu_de, khac_chu_de = phan_tich_tai_lieu(TAI_LIEU_BOSS, CHU_DE, TU_VUNG, CUM_TU_GOC)
so_cap_vuot_troi = sum(1 for d_cung in cung_chu_de if all(d_cung > d_khac for d_khac in khac_chu_de))

print(len(doan))
print(cung_chu_de)
print(khac_chu_de)
print(so_cap_vuot_troi, "/", len(cung_chu_de))
```

```python title=solution
import itertools

def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]
CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def phan_tich_tai_lieu(tai_lieu, chu_de, tu_vung, cum_tu_goc):
    doan = chia_theo_cau(tai_lieu)
    vec = [tinh_vector_dem_tu(d, tu_vung, cum_tu_goc) for d in doan]
    cung_chu_de = []
    khac_chu_de = []
    for i, j in itertools.combinations(range(len(doan)), 2):
        d = tich_vo_huong(vec[i], vec[j])
        if chu_de[i] == chu_de[j]:
            cung_chu_de.append(d)
        else:
            khac_chu_de.append(d)
    return doan, vec, cung_chu_de, khac_chu_de


TAI_LIEU_BOSS = (
    "may tinh hien dai giup lap trinh vien viet phan mem xu ly du lieu nhanh chong. "
    "vi xu ly ben trong may tinh chay thuat toan xu ly du lieu phuc tap trong tich tac. "
    "mon an ngon can co gia vi dam da va cong thuc nau chuan. "
    "dau bep gioi biet chon gia vi phu hop cho moi mon an va lam viec trong nha bep sach se."
)
CHU_DE = ["cong_nghe", "cong_nghe", "am_thuc", "am_thuc"]

doan, vec, cung_chu_de, khac_chu_de = phan_tich_tai_lieu(TAI_LIEU_BOSS, CHU_DE, TU_VUNG, CUM_TU_GOC)
so_cap_vuot_troi = sum(1 for d_cung in cung_chu_de if all(d_cung > d_khac for d_khac in khac_chu_de))

print(len(doan))
print(cung_chu_de)
print(khac_chu_de)
print(so_cap_vuot_troi, "/", len(cung_chu_de))
```

```python title=test
assert len(doan) == 4, f"tai lieu boss phai chia thanh 4 doan -- dang ra {len(doan)}"
assert cung_chu_de == [2, 2], f"cung_chu_de phai la [2, 2] -- dang ra {cung_chu_de}"
assert khac_chu_de == [0, 0, 0, 0], f"khac_chu_de phai la [0, 0, 0, 0] -- dang ra {khac_chu_de}"
assert so_cap_vuot_troi == 2, f"so_cap_vuot_troi phai la 2 -- dang ra {so_cap_vuot_troi}"
assert len(cung_chu_de) == 2, f"phai co dung 2 cap cung chu de (C(4,2)=6 cap, 2 cung + 4 khac) -- dang ra {len(cung_chu_de)}"
assert len(khac_chu_de) == 4, f"phai co dung 4 cap khac chu de -- dang ra {len(khac_chu_de)}"

# xac nhan truc tiep: MOI gia tri trong cung_chu_de phai CAO HON MOI gia tri
# trong khac_chu_de -- khong chi dua vao tong hop so_cap_vuot_troi
for d_cung in cung_chu_de:
    for d_khac in khac_chu_de:
        assert d_cung > d_khac, f"moi cap cung chu de ({d_cung}) phai cao hon moi cap khac chu de ({d_khac})"

# bien: tai lieu chi co 1 chu de duy nhat (toan bo cung chu de) -- khac_chu_de
# phai rong, va so_cap_vuot_troi phai bang 0 (vi all() tren danh sach RONG la
# True, nhung ham so_cap_vuot_troi dem qua MOI d_cung -- kiem tra rieng)
doan_1cd, vec_1cd, cung_1cd, khac_1cd = phan_tich_tai_lieu(
    "cau mot ve cong nghe. cau hai ve cong nghe.", ["cong_nghe", "cong_nghe"], TU_VUNG, CUM_TU_GOC
)
assert khac_1cd == [], f"chi 1 chu de duy nhat thi khac_chu_de phai rong -- dang ra {khac_1cd}"
assert len(cung_1cd) == 1, f"2 doan cung chu de chi co dung 1 cap -- dang ra {len(cung_1cd)}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, trong hai nhánh `if`/`else` của cùng một vòng lặp. Nhánh `if` (khi `chu_de[i] == chu_de[j]`, hai đoạn CÙNG chủ đề) phải thêm `d` vào danh sách `cung_chu_de`. Nhánh `else` (hai đoạn KHÁC chủ đề) phải thêm `d` vào danh sách `khac_chu_de` — đừng nhầm lẫn hai danh sách này với nhau, thứ tự SAI sẽ đảo ngược hoàn toàn kết luận cuối cùng.
- kind: strategy
  body: 'Nhánh `if` (cùng chủ đề): `cung_chu_de.append(d)`. Nhánh `else` (khác chủ đề): `khac_chu_de.append(d)`. Tên biến đã nói rõ ý nghĩa — đọc kỹ điều kiện `if` phía trên để biết mình đang ở nhánh nào.'
- kind: one-line
  body: 'Nhánh `if` là `cung_chu_de.append(d)`, nhánh `else` là `khac_chu_de.append(d)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: nhanh if (cung chu de) phai them vao DUNG danh sach cung_chu_de, nhanh else (khac chu de) phai them vao DUNG danh sach khac_chu_de -- khong duoc dao nguoc hai nhanh
  requireAst:
  - kind: uses-name, target: cung_chu_de, min: 5
  - kind: uses-name, target: khac_chu_de, min: 4
  - kind: uses-call, target: combinations, min: 1
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [5, 4, 1] cho ba
  # luat theo dung thu tu khai bao o tren.
  # cung_chu_de=5 (TONG THAT, da xac nhan bang cong cu, khong doan tay):
  # doc (Load) o dung 5 cho -- (1) cho trong nhanh if (".append(d)"),
  # (2) trong "return doan, vec, cung_chu_de, khac_chu_de", (3) trong
  # "for d_cung in cung_chu_de" (dong tinh so_cap_vuot_troi), (4) trong
  # "print(cung_chu_de)", (5) trong "len(cung_chu_de)" (dong print cuoi).
  # Dong khoi tao "cung_chu_de = []" VA dong gan tu tuple-unpack
  # "doan, vec, cung_chu_de, khac_chu_de = phan_tich_tai_lieu(...)" la Store,
  # KHONG duoc dem.
  # khac_chu_de=4 (TONG THAT): tuong tu, doc o 4 cho -- nhanh else, return,
  # "for d_khac in khac_chu_de" (ben trong all(...)), va "print(khac_chu_de)".
  # combinations=1: dung mot lan GOI THAT (itertools.combinations(...)).
  #
  # 🔴🔴🔴🔴 GOTCHA QUAN TRONG NHAT, DA TU KIEM CHUNG BANG CACH CHAY THAT
  # (khong doan tay): mot mutant HOAN DOI CA HAI nhanh append (dien
  # "khac_chu_de.append(d)" vao nhanh if, VA "cung_chu_de.append(d)" vao
  # nhanh else) KHONG DOI TONG SO DEM AST cho CA HAI ten -- moi ten van
  # duoc DOC dung dung so lan nhu cu (5 va 4), CHI la mot occurrence "di
  # chuyen" tu dong nay sang dong kia (mot lan .append cua "cung_chu_de" di
  # tu nhanh if sang nhanh else, va nguoc lai voi "khac_chu_de") -- tong
  # khong doi BAT KE dat min bao nhieu (ke ca min=5 va min=4 la TONG THAT).
  # Da tu dung mutant nay va chay THAT qua ham _dem() de xac nhan: ket qua
  # AST la [5, 4, 1] -- Y HET ban dung. Static KHONG bat duoc cheat nay,
  # BAT KE nguong min duoc dat cao hay thap.
  # Mutant nay BI BAT DOC LAP boi tests/output: hoan doi hai nhanh lam
  # cung_chu_de tro thanh [0, 0, 0, 0] (dung ra phai la [2, 2]) va
  # khac_chu_de tro thanh [2, 2] (dung ra phai la [0, 0, 0, 0]) -- hoan toan
  # DAO NGUOC. Da tu chay THAT mutant nay qua python3 de xac nhan dung hai
  # gia tri sai nhu mo ta, va cac assert rieng "cung_chu_de == [2, 2]" /
  # "khac_chu_de == [0, 0, 0, 0]" (o tren, dat TRUOC ca "so_cap_vuot_troi")
  # bat duoc NGAY LAP TUC, doc lap voi static va doc lap voi thu tu cac
  # dong print.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^4\\n\\[2, 2\\]\\n\\[0, 0, 0, 0\\]\\n2 / 2\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`2/2` cặp cùng chủ đề vượt qua MỌI cặp khác chủ đề, trên một tài liệu hoàn
toàn mới. Chia, tính vector, đo tích vô hướng — ba mảnh ráp lại thành một
cơ chế truy xuất hoạt động đúng. Quest `chunking-va-embedding-tu-che`
(q8.5a) đóng tại `6/6` bài.
::::

::::reflect{#nghi-lai}
Quest này mở ra với một câu hỏi đơn giản: một LLM (mô phỏng, vì sandbox này
không có mạng) không biết một sự kiện thì làm sao cho nó biết mà không cần
huấn luyện lại? Sáu bài đã trả lời từng mảnh một:

> **`vi-sao-can-rag`** (bài `1`) — kiến thức của một LLM đóng băng ở thời
> điểm huấn luyện, cửa sổ ngữ cảnh có trần; RAG giải quyết bằng cách truy
> xuất một đoạn văn bản liên quan TỪ BÊN NGOÀI, đưa vào ngữ cảnh TRƯỚC khi
> hỏi — đo được: cùng một câu hỏi, có ngữ cảnh trả lời đúng, không có thì
> ảo giác.
>
> **`chia-van-ban-thanh-doan`** (bài `2`) — chia theo câu: đơn giản nhất,
> một tài liệu `N` câu cho ra đúng `N` đoạn.
>
> **`cua-so-truot-co-chong-lap`** (bài `3`) — chia theo câu có thể cắt đứt
> một ý trải dài qua nhiều câu; cửa sổ trượt theo số từ, có chồng lấn, đảm
> bảo một câu ở gần ranh giới vẫn xuất hiện trọn vẹn trong ít nhất một đoạn.
>
> **`vector-tu-che-dem-tu`** (bài `4`) — "embedding" tự chế: một vector đếm
> từ trên một từ vựng cố định nhỏ — KHÔNG phải word2vec/BERT hay bất kỳ mô
> hình embedding thật nào đã huấn luyện trước, chỉ dạy CƠ CHẾ.
>
> **`do-tuong-dong-hai-doan-van`** (bài `5`) — tích vô hướng, MỘT con số đo
> "gần nhau" giữa hai vector; cặp cùng chủ đề luôn cao hơn cặp khác chủ đề.
>
> **`boss-chia-va-tinh-vector-tai-lieu`** (bài `6`, quest này) — ráp cả ba
> bước trên một tài liệu mới, đo `2/2` cặp cùng chủ đề vượt MỌI cặp khác
> chủ đề.

Một điều CỐ Ý xuyên suốt cả `6` bài: mọi "vector"/"embedding" đều là một
hàm Python thuần, tất định — đếm từ trên bảng tra cứu cố định, không mô
hình học máy nào đứng sau, không mạng nơ-ron, không API embedding thật.
Track `T8.5` "RAG từ số `0`" tiếp tục từ đây với `tim-kiem-vector-va-do-
tuong-dong` (q8.5b): tích vô hướng THÔ (bài này) chưa công bằng giữa một
đoạn dài và một đoạn ngắn — quest sau chuẩn hoá nó thành độ tương đồng
cô-sin (cosine similarity) đầy đủ, rồi xây thuật toán tìm `k` lân cận gần
nhất bằng VÉT CẠN — nền tảng để so sánh với chỉ mục HNSW tự cài ở quest sau
nữa (q8.5c).
::::

::::checkpoint{mastery=0.9}
::::
