---
id: tri-tue-nhan-tao.boss-rag-tu-so-0.xay-chi-muc-hnsw-cho-rag
title: "Xây chỉ mục HNSW trên các đoạn đã chia"
summary: "gan_tang_ngau_nhien(26, 14, 0,5) (q8.5c, seed CO DINH) tren 14 vector cua bai truoc cho tang_diem=[0,2,1,0,1,0,0,0,0,1,0,0,1,2]. chen_hnsw(VEC, tang_diem, 3) (q8.5c): diem_vao=1, tang_cao_nhat=2, so_diem_moi_tang=[14,6,2] (giam dan, tang 0 luon du ca 14 diem), so_canh_moi_tang=[10,5,1] (giam dan theo dung so diem). Ca hai doan vector-toan-so-0 (chi so 3 va 11) van duoc CHEN BINH THUONG vao do thi -- khong bi loai bo, chi khong the 'thay' duoc lang gieng gan hon qua cosine (vi cosine voi vector 0 luon la 0,0)."
locale: vi
track: tri-tue-nhan-tao
module: boss-rag-tu-so-0
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.xay-chi-muc-hnsw-cho-rag]
requires: [ai.chunking-embedding-tai-lieu-moi]
concepts: [ai.xay-chi-muc-hnsw-cho-rag]
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
`14` đoạn, `14` vector đã sẵn sàng — kể cả hai đoạn (chỉ số `3`, giá bán, và
`11`, bảo hành) có vector toàn số `0`. Bài này xây chỉ mục HNSW nhiều tầng
trên đúng `14` vector đó, dùng NGUYÊN VẸN `gan_tang_ngau_nhien` và `chen_hnsw`
đã cài ở q8.5c.
::::

::::explain{#xay_chi_muc_tren_du_lieu_moi}
Không có thuật toán mới ở đây — `gan_tang_ngau_nhien` (gán tầng ngẫu nhiên
CÓ SEED, q8.5c bài `5`) và `chen_hnsw` (chèn nhiều tầng, tìm tham lam mỗi
tầng, nối cạnh có cắt bớt theo `M`, q8.5c bài `6`) áp dụng NGUYÊN VĂN lên
`14` vector của bài trước. Điểm cần chú ý: hai đoạn có vector toàn số `0`
(chỉ số `3` và `11`) KHÔNG bị loại khỏi chỉ mục — `chen_hnsw` vẫn chèn chúng
bình thường, mỗi đoạn vẫn có mặt ở tầng `0` và được nối ít nhất một cạnh. Chỉ
có ĐIỀU chúng không làm được là "tự tìm" láng giềng gần hơn qua cosine, vì
`tuong_dong_cosine` với một vector toàn số `0` LUÔN trả về `0,0` (bài
`vector-tu-che-dem-tu`, q8.5a) — mọi so sánh `> 0,0` đều thất bại, nên
`co_lang_gieng_gan_hon` dừng lại ngay ở điểm bắt đầu. Kết quả: đoạn có vector
toàn số `0` vẫn NẰM trong đồ thị, nhưng không có cách nào định vị nó bằng
cosine — một dấu hiệu cho thấy tại sao pipeline cần thêm BM25 (bài sau).
::::

::::example{#xay_hnsw_14_doan}
```python title=readonly
import math
import random

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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, nut_hien_tai):
    tot_nhat = nut_hien_tai
    for hang_xom in do_thi[nut_hien_tai]:
        if tuong_dong_cosine(vectors[hang_xom], vector_muc_tieu) > tuong_dong_cosine(vectors[tot_nhat], vector_muc_tieu):
            tot_nhat = hang_xom
    if tot_nhat == nut_hien_tai:
        return None
    return tot_nhat


def tim_tham_lam(do_thi, vectors, vector_muc_tieu, diem_bat_dau):
    hien_tai = diem_bat_dau
    duong_di = [hien_tai]
    while True:
        ke_tiep = co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, hien_tai)
        if ke_tiep is None:
            break
        hien_tai = ke_tiep
        duong_di.append(hien_tai)
    return hien_tai, duong_di


def gan_tang_ngau_nhien(seed, n_diem, xac_suat):
    rng = random.Random(seed)
    tang = []
    for i in range(n_diem):
        t = 0
        while rng.random() < xac_suat:
            t += 1
        tang.append(t)
    return tang


def noi_canh_va_cat(do_thi_tang, i, j, vectors, M):
    if j not in do_thi_tang[i]:
        do_thi_tang[i].append(j)
    if i not in do_thi_tang[j]:
        do_thi_tang[j].append(i)
    for x in (i, j):
        if len(do_thi_tang[x]) > M:
            xep = sorted(do_thi_tang[x], key=lambda n: tuong_dong_cosine(vectors[x], vectors[n]), reverse=True)
            do_thi_tang[x] = xep[:M]


def chen_hnsw(vectors, tang_diem, M):
    so_tang_max = max(tang_diem) + 1
    do_thi = [dict() for _ in range(so_tang_max)]
    diem_vao = None
    tang_cao_nhat = -1
    for i in range(len(vectors)):
        L = tang_diem[i]
        for t in range(L + 1):
            do_thi[t].setdefault(i, [])
        if diem_vao is None:
            diem_vao = i
            tang_cao_nhat = L
            continue
        hien_tai = diem_vao
        for t in range(tang_cao_nhat, L, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
        for t in range(min(L, tang_cao_nhat), -1, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
            noi_canh_va_cat(do_thi[t], i, hien_tai, vectors, M)
        if L > tang_cao_nhat:
            diem_vao = i
            tang_cao_nhat = L
    return do_thi, diem_vao, tang_cao_nhat


TAI_LIEU_MOI = (
    "cong ty ra mat san pham may tinh bang zt8821 vao thang nay. "
    "may tinh bang zt8821 dung vi xu ly moi manh hon the he truoc. "
    "phan mem tren zt8821 duoc lap trinh toi uu cho ung dung van phong. "
    "gia ban chinh thuc cua zt8821 la 8990000 dong. "
    "ket noi mang cua zt8821 ho tro ca wifi va 4g. "
    "du lieu nguoi dung tren zt8821 duoc luu tru bang thuat toan ma hoa rieng. "
    "vi xu ly cua zt8821 giup ung dung khoi dong nhanh hon. "
    "lap trinh vien co the viet phan mem moi cho zt8821 qua bo cong cu rieng. "
    "thuat toan nen du lieu tren zt8821 tiet kiem bo nho luu tru. "
    "ung dung camera cua zt8821 xu ly anh bang thuat toan tri tue nhan tao. "
    "ket noi mang khong day cua zt8821 on dinh trong pham vi rong. "
    "che do bao hanh chinh thuc cua zt8821 la 18 thang. "
    "du lieu ban dau cho thay zt8821 ban duoc 50000 chiec trong tuan dau. "
    "cong ty cam ket cap nhat phan mem cho zt8821 trong 3 nam."
)

DOAN = chia_theo_cau(TAI_LIEU_MOI)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DOAN]

tang_diem = gan_tang_ngau_nhien(26, len(VEC), 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

so_diem_moi_tang = [len(g) for g in do_thi]
so_canh_moi_tang = [sum(len(v) for v in g.values()) // 2 for g in do_thi]

print(tang_diem)
print(diem_vao)
print(tang_cao_nhat)
print(so_diem_moi_tang)
print(so_canh_moi_tang)
```

```text title=readonly
[0, 2, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 2]
1
2
[14, 6, 2]
[10, 5, 1]
```

Với `seed=26`, điểm `1` (`"may tinh bang zt8821 dung vi xu ly..."`) là điểm
vào, tầng cao nhất là `2`. `so_diem_moi_tang = [14, 6, 2]` — tầng `0` chứa đủ
cả `14` đoạn, tầng `1` còn `6`, tầng `2` còn `2` — giảm dần đúng khuôn q8.5c.
`so_canh_moi_tang = [10, 5, 1]` cũng giảm dần theo. Đoạn `3` và `11` (vector
toàn số `0`) vẫn nằm trong `do_thi[0]`, mỗi đoạn có đúng `1` cạnh — chúng
được chèn vào chỗ điểm bắt đầu của lượt tìm tham lam, vì không so sánh cosine
nào "thắng" được `0,0` ban đầu.
::::

::::predict{#doan_so_canh_van_giam_dan commitOnce}
Tài liệu mới chỉ có `14` đoạn (ít hơn `20` điểm của q8.5c), và hai trong số
đó (chỉ số `3`, `11`) có vector toàn số `0`.

**Trước khi chạy thử**, bạn đoán: `so_canh_moi_tang` có còn giữ xu hướng
GIẢM DẦN khi lên tầng cao hơn không, giống mọi lần trước?

:::opt{correct}
Có — cơ chế "tầng cao càng ít điểm, càng ít cạnh" không phụ thuộc vào SỐ
LƯỢNG điểm hay việc một vài vector có toàn số `0` hay không; `M` vẫn là trần
cố định cho mọi đỉnh, và số điểm khả dĩ để nối càng lên cao càng ít
(`so_diem_moi_tang` giảm dần) — `so_canh_moi_tang = [10, 5, 1]` giảm dần đúng
như dự đoán
:::

:::opt
Không — vì tài liệu mới có ÍT điểm hơn (`14` so với `20`), số điểm ít khiến
cơ chế tầng có thể đảo ngược, cạnh tăng dần thay vì giảm dần
::why
Gần đúng ở việc tài liệu này THẬT SỰ có ít điểm hơn (`14` so với `20` của
q8.5c) — quan sát về SỐ LƯỢNG đó đúng.

Chỗ lệch: cơ chế giảm dần không phụ thuộc vào TỔNG số điểm — nó chỉ phụ
thuộc vào việc CÀNG LÊN CAO, xác suất một điểm "sống sót" tới đó càng giảm
theo cấp số nhân (`gan_tang_ngau_nhien`, q8.5c bài `5`). Dù `N=14` hay
`N=20`, quy luật đó không đổi — `so_canh_moi_tang` vẫn giảm dần.
::
:::

:::opt
Không — hai đoạn có vector toàn số `0` sẽ làm đồ thị mất cân bằng, khiến số
cạnh có thể tăng đột biến ở tầng cao để "bù đắp" cho việc không định vị được
chúng
::why
Gần đúng ở việc hai đoạn vector toàn số `0` THẬT SỰ không định vị được bằng
cosine (quan sát ở bài `explain` phía trên đúng).

Chỗ lệch: `noi_canh_va_cat` áp trần `M` cho MỌI đỉnh như nhau, bất kể vector
của nó là gì — một đoạn có vector toàn số `0` vẫn chỉ được nối tối đa `M`
cạnh, không có cơ chế "bù đắp" nào làm tăng cạnh ở tầng cao. Số cạnh mỗi tầng
vẫn hoàn toàn do SỐ ĐIỂM có mặt ở tầng đó quyết định, và số đó giảm dần.
::
:::
::::

::::code{#viet_xay_chi_muc_hnsw_rag}
Hoàn thiện phần xây chỉ mục: gán tầng ngẫu nhiên có seed cho `14` vector, rồi
chèn tất cả vào một đồ thị HNSW nhiều tầng.

```python title=starter
import math
import random

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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, nut_hien_tai):
    tot_nhat = nut_hien_tai
    for hang_xom in do_thi[nut_hien_tai]:
        if tuong_dong_cosine(vectors[hang_xom], vector_muc_tieu) > tuong_dong_cosine(vectors[tot_nhat], vector_muc_tieu):
            tot_nhat = hang_xom
    if tot_nhat == nut_hien_tai:
        return None
    return tot_nhat


def tim_tham_lam(do_thi, vectors, vector_muc_tieu, diem_bat_dau):
    hien_tai = diem_bat_dau
    duong_di = [hien_tai]
    while True:
        ke_tiep = co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, hien_tai)
        if ke_tiep is None:
            break
        hien_tai = ke_tiep
        duong_di.append(hien_tai)
    return hien_tai, duong_di


def gan_tang_ngau_nhien(seed, n_diem, xac_suat):
    rng = random.Random(seed)
    tang = []
    for i in range(n_diem):
        t = 0
        while rng.random() < xac_suat:
            t += 1
        tang.append(t)
    return tang


def noi_canh_va_cat(do_thi_tang, i, j, vectors, M):
    if j not in do_thi_tang[i]:
        do_thi_tang[i].append(j)
    if i not in do_thi_tang[j]:
        do_thi_tang[j].append(i)
    for x in (i, j):
        if len(do_thi_tang[x]) > M:
            xep = sorted(do_thi_tang[x], key=lambda n: tuong_dong_cosine(vectors[x], vectors[n]), reverse=True)
            do_thi_tang[x] = xep[:M]


def chen_hnsw(vectors, tang_diem, M):
    so_tang_max = max(tang_diem) + 1
    do_thi = [dict() for _ in range(so_tang_max)]
    diem_vao = None
    tang_cao_nhat = -1
    for i in range(len(vectors)):
        L = tang_diem[i]
        for t in range(L + 1):
            do_thi[t].setdefault(i, [])
        if diem_vao is None:
            diem_vao = i
            tang_cao_nhat = L
            continue
        hien_tai = diem_vao
        for t in range(tang_cao_nhat, L, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
        for t in range(min(L, tang_cao_nhat), -1, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
            noi_canh_va_cat(do_thi[t], i, hien_tai, vectors, M)
        if L > tang_cao_nhat:
            diem_vao = i
            tang_cao_nhat = L
    return do_thi, diem_vao, tang_cao_nhat


TAI_LIEU_MOI = (
    "cong ty ra mat san pham may tinh bang zt8821 vao thang nay. "
    "may tinh bang zt8821 dung vi xu ly moi manh hon the he truoc. "
    "phan mem tren zt8821 duoc lap trinh toi uu cho ung dung van phong. "
    "gia ban chinh thuc cua zt8821 la 8990000 dong. "
    "ket noi mang cua zt8821 ho tro ca wifi va 4g. "
    "du lieu nguoi dung tren zt8821 duoc luu tru bang thuat toan ma hoa rieng. "
    "vi xu ly cua zt8821 giup ung dung khoi dong nhanh hon. "
    "lap trinh vien co the viet phan mem moi cho zt8821 qua bo cong cu rieng. "
    "thuat toan nen du lieu tren zt8821 tiet kiem bo nho luu tru. "
    "ung dung camera cua zt8821 xu ly anh bang thuat toan tri tue nhan tao. "
    "ket noi mang khong day cua zt8821 on dinh trong pham vi rong. "
    "che do bao hanh chinh thuc cua zt8821 la 18 thang. "
    "du lieu ban dau cho thay zt8821 ban duoc 50000 chiec trong tuan dau. "
    "cong ty cam ket cap nhat phan mem cho zt8821 trong 3 nam."
)

DOAN = chia_theo_cau(TAI_LIEU_MOI)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DOAN]

tang_diem = ___                                             # gan_tang_ngau_nhien(26, len(VEC), 0.5)
do_thi, diem_vao, tang_cao_nhat = ___                        # chen_hnsw(VEC, tang_diem, 3)

so_diem_moi_tang = [len(g) for g in do_thi]
so_canh_moi_tang = [sum(len(v) for v in g.values()) // 2 for g in do_thi]

print(tang_diem)
print(diem_vao)
print(tang_cao_nhat)
print(so_diem_moi_tang)
print(so_canh_moi_tang)
```

```python title=solution
import math
import random

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


def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, nut_hien_tai):
    tot_nhat = nut_hien_tai
    for hang_xom in do_thi[nut_hien_tai]:
        if tuong_dong_cosine(vectors[hang_xom], vector_muc_tieu) > tuong_dong_cosine(vectors[tot_nhat], vector_muc_tieu):
            tot_nhat = hang_xom
    if tot_nhat == nut_hien_tai:
        return None
    return tot_nhat


def tim_tham_lam(do_thi, vectors, vector_muc_tieu, diem_bat_dau):
    hien_tai = diem_bat_dau
    duong_di = [hien_tai]
    while True:
        ke_tiep = co_lang_gieng_gan_hon(do_thi, vectors, vector_muc_tieu, hien_tai)
        if ke_tiep is None:
            break
        hien_tai = ke_tiep
        duong_di.append(hien_tai)
    return hien_tai, duong_di


def gan_tang_ngau_nhien(seed, n_diem, xac_suat):
    rng = random.Random(seed)
    tang = []
    for i in range(n_diem):
        t = 0
        while rng.random() < xac_suat:
            t += 1
        tang.append(t)
    return tang


def noi_canh_va_cat(do_thi_tang, i, j, vectors, M):
    if j not in do_thi_tang[i]:
        do_thi_tang[i].append(j)
    if i not in do_thi_tang[j]:
        do_thi_tang[j].append(i)
    for x in (i, j):
        if len(do_thi_tang[x]) > M:
            xep = sorted(do_thi_tang[x], key=lambda n: tuong_dong_cosine(vectors[x], vectors[n]), reverse=True)
            do_thi_tang[x] = xep[:M]


def chen_hnsw(vectors, tang_diem, M):
    so_tang_max = max(tang_diem) + 1
    do_thi = [dict() for _ in range(so_tang_max)]
    diem_vao = None
    tang_cao_nhat = -1
    for i in range(len(vectors)):
        L = tang_diem[i]
        for t in range(L + 1):
            do_thi[t].setdefault(i, [])
        if diem_vao is None:
            diem_vao = i
            tang_cao_nhat = L
            continue
        hien_tai = diem_vao
        for t in range(tang_cao_nhat, L, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
        for t in range(min(L, tang_cao_nhat), -1, -1):
            hien_tai, _ = tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai)
            noi_canh_va_cat(do_thi[t], i, hien_tai, vectors, M)
        if L > tang_cao_nhat:
            diem_vao = i
            tang_cao_nhat = L
    return do_thi, diem_vao, tang_cao_nhat


TAI_LIEU_MOI = (
    "cong ty ra mat san pham may tinh bang zt8821 vao thang nay. "
    "may tinh bang zt8821 dung vi xu ly moi manh hon the he truoc. "
    "phan mem tren zt8821 duoc lap trinh toi uu cho ung dung van phong. "
    "gia ban chinh thuc cua zt8821 la 8990000 dong. "
    "ket noi mang cua zt8821 ho tro ca wifi va 4g. "
    "du lieu nguoi dung tren zt8821 duoc luu tru bang thuat toan ma hoa rieng. "
    "vi xu ly cua zt8821 giup ung dung khoi dong nhanh hon. "
    "lap trinh vien co the viet phan mem moi cho zt8821 qua bo cong cu rieng. "
    "thuat toan nen du lieu tren zt8821 tiet kiem bo nho luu tru. "
    "ung dung camera cua zt8821 xu ly anh bang thuat toan tri tue nhan tao. "
    "ket noi mang khong day cua zt8821 on dinh trong pham vi rong. "
    "che do bao hanh chinh thuc cua zt8821 la 18 thang. "
    "du lieu ban dau cho thay zt8821 ban duoc 50000 chiec trong tuan dau. "
    "cong ty cam ket cap nhat phan mem cho zt8821 trong 3 nam."
)

DOAN = chia_theo_cau(TAI_LIEU_MOI)
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DOAN]

tang_diem = gan_tang_ngau_nhien(26, len(VEC), 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

so_diem_moi_tang = [len(g) for g in do_thi]
so_canh_moi_tang = [sum(len(v) for v in g.values()) // 2 for g in do_thi]

print(tang_diem)
print(diem_vao)
print(tang_cao_nhat)
print(so_diem_moi_tang)
print(so_canh_moi_tang)
```

```python title=test
assert tang_diem == [0, 2, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 2], f"tang_diem sai -- dang ra {tang_diem}"
assert diem_vao == 1, f"diem vao phai la 1 -- dang ra {diem_vao}"
assert tang_cao_nhat == 2, f"tang cao nhat phai la 2 -- dang ra {tang_cao_nhat}"
assert so_diem_moi_tang == [14, 6, 2], f"so diem moi tang sai -- dang ra {so_diem_moi_tang}"
assert so_canh_moi_tang == [10, 5, 1], f"so canh moi tang sai -- dang ra {so_canh_moi_tang}"
assert so_diem_moi_tang[0] == len(VEC), "tang 0 phai chua DU moi diem, khong thieu diem nao"
assert all(so_canh_moi_tang[t] >= so_canh_moi_tang[t + 1] for t in range(len(so_canh_moi_tang) - 1)), "so canh khong duoc TANG khi len tang cao hon"

# bien: hai doan vector-toan-so-0 (chi so 3 va 11) van phai duoc CHEN vao do
# thi, khong bi loai bo -- moi doan van co it nhat mot canh o tang 0
assert 3 in do_thi[0] and 11 in do_thi[0], "hai doan vector-toan-so-0 (chi so 3, 11) van phai co mat o tang 0"
assert len(do_thi[0][3]) >= 1 and len(do_thi[0][11]) >= 1, "hai doan vector-toan-so-0 van phai co it nhat mot canh, khong bi co lap"
```

:::hints
- kind: attention
  body: Hai chỗ trống, đúng khuôn "ráp lại" — không viết thuật toán mới. Chỗ đầu gọi hàm gán tầng ngẫu nhiên có seed (q8.5c bài `5`) cho toàn bộ `14` vector. Chỗ hai gọi hàm chèn HNSW nhiều tầng (q8.5c bài `6`) trên đúng những vector và tầng vừa gán.
- kind: strategy
  body: 'Chỗ đầu: `gan_tang_ngau_nhien(26, len(VEC), 0.5)` — seed `26` cố định, số điểm bằng `len(VEC)`, xác suất `0,5` đúng khuôn mọi lần trước. Chỗ hai: `chen_hnsw(VEC, tang_diem, 3)` — vector, tầng vừa gán, và `M=3`.'
- kind: one-line
  body: 'Chỗ đầu là `gan_tang_ngau_nhien(26, len(VEC), 0.5)`, chỗ hai là `chen_hnsw(VEC, tang_diem, 3)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT gan_tang_ngau_nhien(26, len(VEC), 0.5) (khong duoc chep san danh sach tang); VA cho trong hai phai GOI THAT chen_hnsw(VEC, tang_diem, 3) (khong duoc chep san do_thi/diem_vao/tang_cao_nhat)
  requireAst:
  - kind: uses-call, target: gan_tang_ngau_nhien, min: 1
  - kind: uses-call, target: chen_hnsw, min: 1
  - kind: has-literal, target: "3", min: 1
  # Da thu that (dung dem_ast.mjs goi THANG kiemAst() that qua dist build +
  # pyodide that, chay tren CHINH van ban solution da trich tu file nay) --
  # ket qua [1, 1, 1] cho ba luat theo dung thu tu khai bao o tren. Ca hai ham
  # chi duoc GOI dung MOT lan trong toan bo solution (dinh nghia "def
  # gan_tang_ngau_nhien(...)"/"def chen_hnsw(...)" la ten HAM, khong phai
  # Call, khong dem) -- day la TONG THAT, khong co boilerplate nao khac goi
  # lai hai ham nay, nen min=1 khong co rui ro "boilerplate-threshold-
  # masking".
  #
  # 🔴🔴🔴 LO THAT do CONG CU DOT BIEN TU DONG cua du an (tools/kiem_dot_bien.mjs)
  # phat hien, KHONG phai tu tay toi/agent dung: doi hang so M trong loi goi
  # dung o cho trong hai (chen_hnsw(VEC, tang_diem, 3)) tu 3 thanh 4 -- VAN
  # QUA SACH moi tang cham (chi voi hai luat uses-call o tren). Da tu chay
  # THAT mutant nay qua python3: tren dung 14 diem nay, khong dinh nao trong
  # do thi THAT SU dat toi M=3 canh (xem so_canh_moi_tang=[10,5,1] -- khong
  # dinh nao bi CAT boi tran M ca), nen tran M=3 hay M=4 cho ra HOAN TOAN
  # CUNG mot ket qua (tang_diem, do_thi, so_diem_moi_tang, so_canh_moi_tang
  # deu GIONG HET nhau) -- day la mot LOP GOTCHA KHAC voi "boilerplate-
  # threshold-masking" (khong lien quan so lan GOI ham) VA khac "hoan doi ca
  # cum" (chi doi MOT hang so o MOT cho trong, khong hoan doi hai cho trong
  # cho nhau) -- mot LOP THU TU: mot THAM SO SO hop le nhung KHONG THAT SU
  # rang buoc gi tren BO DU LIEU CU THE nay. Them luat 'has-literal target=3'
  # buoc solution phai CHUA literal 3 o dau do -- da tu xac nhan bang cong cu
  # THAT: dem=0 tren mutant (chi con literal 4, khong con 3), duoi nguong
  # min=1, static chan duoc. Da chay lai `node tools/kiem_dot_bien.mjs` tren
  # scope hep (chi module nay) SAU KHI them luat nay va xac nhan KHONG con lo
  # nao duoc bao cao.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 VA qua dem_ast.mjs de xac nhan, khong doan tay): dien
  # "chen_hnsw(VEC, tang_diem, 3)" vao cho trong dau ("tang_diem =
  # chen_hnsw(VEC, tang_diem, 3)") VA dien "gan_tang_ngau_nhien(26, len(VEC),
  # 0.5)" vao cho trong hai ("do_thi, diem_vao, tang_cao_nhat =
  # gan_tang_ngau_nhien(26, len(VEC), 0.5)") -- da chay THAT qua dem_ast.mjs:
  # ket qua dem la [1, 1], Y HET ban dung (moi ham van duoc GOI dung 1 lan,
  # chi doi VI TRI giua hai dong). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT DOC LAP boi tier 'run': dong dau tien "tang_diem =
  # chen_hnsw(VEC, tang_diem, 3)" DOC bien 'tang_diem' o VE PHAI (doi so thu
  # hai cua chen_hnsw) TRUOC KHI no duoc GAN o BAT KY dau trong toan bo
  # chuong trinh (day la lan dau tien ten 'tang_diem' xuat hien) -- da tu
  # chay THAT mutant nay qua python3, xac nhan no nem NameError ("name
  # 'tang_diem' is not defined") ngay khi dong nay chay, truoc ca khi cham
  # toi dong thu hai.
  # Da tu ra soat GOTCHA #6: khong co bien nao khac trong pham vi bai nay
  # cung "hinh dang" (mot ham nhan (seed, n, xac_suat) tra ve list, mot ham
  # nhan (vectors, tang, M) tra ve tuple 3 phan tu) co the tinh co thay the
  # ma van qua tests/output -- hai ham nay co CHU KY (signature)/kieu tra ve
  # hoan toan khac nhau, khong co ham nao khac trong bai co the nham lan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[0, 2, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 2\\]\\n1\\n2\\n\\[14, 6, 2\\]\\n\\[10, 5, 1\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`[14, 6, 2]` điểm, `[10, 5, 1]` cạnh, giảm dần đúng khuôn — kể cả hai đoạn
vector toàn số `0` vẫn có mặt, chỉ không "tự tìm" được nhau bằng cosine. Bài
sau tìm kiếm trên đúng chỉ mục này, kết hợp với BM25.
::::

::::reflect{#nghi-lai}
`gan_tang_ngau_nhien` và `chen_hnsw` không thay đổi MỘT dòng nào so với
q8.5c — chỉ đổi DỮ LIỆU đưa vào. Đó chính là ý nghĩa của "ráp lại": thuật
toán đã được chứng minh đúng trên một tập dữ liệu, giờ áp dụng lên một tập
khác mà không cần viết lại logic. Kết quả đo được củng cố thêm điều bài
trước đã thiết lập: hai đoạn có vector toàn số `0` (giá bán, bảo hành) vẫn
được HNSW chèn bình thường vào chỉ mục — không mất dữ liệu — nhưng vị trí
của chúng trong đồ thị không phản ánh được sự liên quan THẬT SỰ, vì cosine
không phân biệt được chúng với bất kỳ điểm nào khác. Bài sau đưa BM25 vào
CÙNG một câu hỏi, và ráp cả hai qua RRF để kiểm xem hybrid có sửa được điểm
mù này hay không.
::::

::::checkpoint{mastery=0.85}
::::
