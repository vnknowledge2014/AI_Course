---
id: tri-tue-nhan-tao.hnsw-tu-cai.thuat-toan-chen-hnsw
title: "Thuật toán chèn HNSW: nhiều tầng, tìm tham lam mỗi tầng"
summary: "chen_hnsw(vectors, tang_diem, M) chen tung diem: dung tang_diem (bai truoc) de biet diem moi xuat hien toi tang nao; tu diem vao hien tai, TIM THAM LAM (bai 3) qua tung tang tu tang cao nhat xuong toi tang cua diem moi, NOI canh (co cat bot ve M, bai 2) tai moi tang no xuat hien. Tren N=20 diem, seed=29, M=3: diem_vao=4, tang_cao_nhat=4, so_diem_moi_tang=[20, 12, 4, 2, 2], so_canh_moi_tang=[16, 9, 3, 1, 1] -- ca so diem LAN so canh deu giam dan len tang cao, dung mot lan chay that."
locale: vi
track: tri-tue-nhan-tao
module: hnsw-tu-cai
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.thuat-toan-chen-hnsw]
requires: [ai.gan-tang-ngau-nhien-co-seed]
concepts: [ai.thuat-toan-chen-hnsw]
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
Ba mảnh đã có: tìm tham lam MỘT tầng (bài `3`), nối cạnh CÓ cắt bớt (bài
`2`), gán tầng ngẫu nhiên có seed (bài `5`). Bài này ráp cả ba thành thuật
toán CHÈN đầy đủ, nhiều tầng.
::::

::::explain{#thuat_toan_chen_hnsw}
Chèn một điểm mới `i` vào đồ thị HNSW nhiều tầng, theo đúng ba mảnh đã xây:

> **(a) Gán tầng** (bài `5`) — điểm `i` được gán một tầng cao nhất `L`
> (tính trước, dùng `gan_tang_ngau_nhien`); điểm này sẽ xuất hiện ở MỌI
> tầng từ `0` tới `L`.
>
> **(b) Đi xuống, KHÔNG nối cạnh** — nếu đồ thị đã có tầng cao HƠN `L` (từ
> những điểm chèn trước), tìm tham lam (bài `3`) qua các tầng đó CHỈ để cập
> nhật điểm hiện tại `hien_tai` thành điểm gần `i` nhất tìm được — chưa nối
> cạnh gì, vì `i` KHÔNG tồn tại ở những tầng này.
>
> **(c) Đi xuống, CÓ nối cạnh** — từ tầng `L` (hoặc tầng cao nhất hiện có,
> nếu thấp hơn `L`) xuống tới tầng `0`: tại MỖI tầng, tìm tham lam (bài `3`)
> để tìm láng giềng gần `i` nhất hiện có tại tầng đó, rồi NỐI cạnh giữa `i`
> và láng giềng đó (bài `2` — có cắt bớt nếu vượt `M`).
>
> **(d) Điểm ĐẦU TIÊN** — điểm được chèn đầu tiên không có gì để so sánh;
> nó trở thành **điểm vào** (entry point) mặc định.
>
> **(e) Điểm vào MỚI** — nếu tầng `L` của điểm vừa chèn CAO HƠN tầng cao
> nhất hiện có, điểm đó trở thành điểm vào MỚI (và tầng cao nhất cũng cập
> nhật theo).

Kết quả: một danh sách đồ thị, MỘT đồ thị cho MỖI tầng — càng lên cao, càng
ÍT điểm (chỉ những điểm được gán tầng đủ cao mới xuất hiện) và càng ÍT cạnh.
::::

::::example{#chen_hnsw_20_diem}
Chèn `20` điểm (bài `1`–`3`) vào một đồ thị nhiều tầng, dùng `tang_diem` từ
`gan_tang_ngau_nhien(29, 20, 0.5)` (bài trước) và `M=3`:

```python title=readonly
import math
import random
from collections import Counter

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


DIEM = [
    "may tinh chay phan mem manh", "lap trinh vien viet thuat toan hay", "vi xu ly xu ly du lieu nhanh",
    "ket noi mang giup ung dung on dinh", "thuat toan sap xep du lieu chuan", "ung dung may tinh xu ly du lieu lon",
    "vi xu ly manh giup ung dung nhanh", "phan mem lap trinh toi uu thuat toan", "du lieu duoc may tinh xu ly tu dong",
    "ung dung ket noi mang truyen du lieu",
    "mon an ngon can gia vi cong thuc", "dau bep nau an trong nha bep sach", "thuc pham tuoi giup mon an ngon",
    "cong thuc nau an don gian de lam", "mon trang mieng ngot ngao sau bua an", "nha bep sach se giup dau bep thoai mai",
    "gia vi dam da giup mon an hap dan", "dau bep gioi che bien thuc pham tuoi", "mon an voi cong thuc gia vi vua an",
    "thuc pham nau thanh mon trang mieng ngon",
]
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DIEM]
N = len(VEC)

tang_diem = gan_tang_ngau_nhien(29, N, 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

so_diem_moi_tang = [len(g) for g in do_thi]
so_canh_moi_tang = [sum(len(v) for v in g.values()) // 2 for g in do_thi]

print(diem_vao)
print(tang_cao_nhat)
print(so_diem_moi_tang)
print(so_canh_moi_tang)
```

```text title=readonly
4
4
[20, 12, 4, 2, 2]
[16, 9, 3, 1, 1]
```

Điểm `4` là điểm vào (điểm ĐẦU TIÊN được gán tầng cao nhất, `4`, trong lần
chèn của nó). `so_diem_moi_tang` khớp ĐÚNG với phân bố tầng đã đo ở bài
trước (`[20, 12, 4, 2, 2]`). `so_canh_moi_tang` cũng giảm dần: `16` cạnh ở
tầng `0` (gần với `30` cạnh của đồ thị MỘT tầng, bài `2` — không bằng hệt,
vì thứ tự chèn và cách nối bây giờ khác), chỉ còn `1` cạnh ở hai tầng cao
nhất (`3` và `4`, mỗi tầng chỉ có `2` điểm, nối với nhau đúng `1` cạnh).
::::

::::predict{#doan_so_canh_giam_dan commitOnce}
Xét đúng ví dụ trên: `so_diem_moi_tang = [20, 12, 4, 2, 2]`.

**Trước khi chạy thử**, bạn đoán: `so_canh_moi_tang` (số cạnh mỗi tầng) có
XU HƯỚNG giảm dần khi lên tầng cao hơn, giống `so_diem_moi_tang`, hay
KHÔNG?

:::opt{correct}
Có — càng lên cao, càng ÍT điểm tham gia tầng đó (`so_diem_moi_tang` giảm
dần), nên số cạnh CÓ THỂ tạo ra giữa chúng cũng giảm theo — tầng `0` có
`16` cạnh, tầng `3` và `4` chỉ còn `1` cạnh mỗi tầng
:::

:::opt
Không — số cạnh phải giữ NGUYÊN ở mọi tầng, vì mỗi điểm luôn nối tới đúng
`M=3` láng giềng bất kể đang ở tầng nào
::why
Gần đúng ở việc `M=3` LÀ giới hạn TỐI ĐA áp dụng cho mọi tầng — quan sát về
con số `M` đó đúng.

Chỗ lệch: `M` là một TRẦN (tối đa), không phải một SÀN bắt buộc — một tầng
chỉ có `2` điểm thì KHÔNG THỂ có `3` láng giềng mỗi điểm (chỉ có `1` láng
giềng khả dĩ: điểm còn lại). Số cạnh thực tế phụ thuộc vào SỐ ĐIỂM có mặt ở
tầng đó, không phải luôn đạt trần `M`.
::
:::

:::opt
Không — số cạnh phải TĂNG dần khi lên tầng cao, vì càng lên cao, những điểm
còn lại càng "quan trọng" nên cần nối nhiều cạnh hơn để đảm bảo kết nối tốt
::why
Gần đúng ở trực giác "tầng cao chứa điểm quan trọng hơn" — có liên quan
tới vai trò tầng cao trong việc nhảy xa của HNSW.

Chỗ lệch: "quan trọng hơn" không đồng nghĩa với "cần nhiều cạnh hơn" trong
cách CHÈN đã xây — số cạnh tối đa mỗi đỉnh vẫn bị giới hạn bởi `M`, và số
điểm khả dĩ để nối càng lên cao càng ÍT (`so_diem_moi_tang` giảm dần), nên
tổng số cạnh KHÔNG THỂ tăng — nó giảm dần đúng theo số điểm.
::
:::
::::

::::code{#viet_chen_hnsw}
Hoàn thiện `chen_hnsw`: tại bước "đi xuống, CÓ nối cạnh", dùng
`tim_tham_lam` (bài `3`) để cập nhật điểm gần nhất tìm được tại tầng đó,
rồi nối cạnh giữa điểm mới và điểm gần nhất đó.

```python title=starter
import math
import random
from collections import Counter

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
            hien_tai, _ = ___(do_thi[t], vectors, vectors[i], hien_tai)   # tim_tham_lam
            noi_canh_va_cat(do_thi[t], i, ___, vectors, M)                # hien_tai
        if L > tang_cao_nhat:
            diem_vao = i
            tang_cao_nhat = L
    return do_thi, diem_vao, tang_cao_nhat


DIEM = [
    "may tinh chay phan mem manh", "lap trinh vien viet thuat toan hay", "vi xu ly xu ly du lieu nhanh",
    "ket noi mang giup ung dung on dinh", "thuat toan sap xep du lieu chuan", "ung dung may tinh xu ly du lieu lon",
    "vi xu ly manh giup ung dung nhanh", "phan mem lap trinh toi uu thuat toan", "du lieu duoc may tinh xu ly tu dong",
    "ung dung ket noi mang truyen du lieu",
    "mon an ngon can gia vi cong thuc", "dau bep nau an trong nha bep sach", "thuc pham tuoi giup mon an ngon",
    "cong thuc nau an don gian de lam", "mon trang mieng ngot ngao sau bua an", "nha bep sach se giup dau bep thoai mai",
    "gia vi dam da giup mon an hap dan", "dau bep gioi che bien thuc pham tuoi", "mon an voi cong thuc gia vi vua an",
    "thuc pham nau thanh mon trang mieng ngon",
]
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DIEM]
N = len(VEC)

tang_diem = gan_tang_ngau_nhien(29, N, 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

so_diem_moi_tang = [len(g) for g in do_thi]
so_canh_moi_tang = [sum(len(v) for v in g.values()) // 2 for g in do_thi]

print(diem_vao)
print(tang_cao_nhat)
print(so_diem_moi_tang)
print(so_canh_moi_tang)
```

```python title=solution
import math
import random
from collections import Counter

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


DIEM = [
    "may tinh chay phan mem manh", "lap trinh vien viet thuat toan hay", "vi xu ly xu ly du lieu nhanh",
    "ket noi mang giup ung dung on dinh", "thuat toan sap xep du lieu chuan", "ung dung may tinh xu ly du lieu lon",
    "vi xu ly manh giup ung dung nhanh", "phan mem lap trinh toi uu thuat toan", "du lieu duoc may tinh xu ly tu dong",
    "ung dung ket noi mang truyen du lieu",
    "mon an ngon can gia vi cong thuc", "dau bep nau an trong nha bep sach", "thuc pham tuoi giup mon an ngon",
    "cong thuc nau an don gian de lam", "mon trang mieng ngot ngao sau bua an", "nha bep sach se giup dau bep thoai mai",
    "gia vi dam da giup mon an hap dan", "dau bep gioi che bien thuc pham tuoi", "mon an voi cong thuc gia vi vua an",
    "thuc pham nau thanh mon trang mieng ngon",
]
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DIEM]
N = len(VEC)

tang_diem = gan_tang_ngau_nhien(29, N, 0.5)
do_thi, diem_vao, tang_cao_nhat = chen_hnsw(VEC, tang_diem, 3)

so_diem_moi_tang = [len(g) for g in do_thi]
so_canh_moi_tang = [sum(len(v) for v in g.values()) // 2 for g in do_thi]

print(diem_vao)
print(tang_cao_nhat)
print(so_diem_moi_tang)
print(so_canh_moi_tang)
```

```python title=test
assert diem_vao == 4, f"diem vao phai la 4 -- dang ra {diem_vao}"
assert tang_cao_nhat == 4, f"tang cao nhat phai la 4 -- dang ra {tang_cao_nhat}"
assert so_diem_moi_tang == [20, 12, 4, 2, 2], f"so diem moi tang sai -- dang ra {so_diem_moi_tang}"
assert so_canh_moi_tang == [16, 9, 3, 1, 1], f"so canh moi tang sai -- dang ra {so_canh_moi_tang}"
assert all(so_canh_moi_tang[t] >= so_canh_moi_tang[t + 1] for t in range(len(so_canh_moi_tang) - 1)), "so canh khong duoc TANG khi len tang cao hon"

# kiem tra truc tiep tren mot vi du nho, tu tinh tay duoc: 4 diem, 2 tang
vec_nho = [[10, 0], [9, 1], [0, 10], [1, 9]]
tang_nho = [1, 0, 1, 0]
do_thi_nho, diem_vao_nho, tang_cao_nhat_nho = chen_hnsw(vec_nho, tang_nho, 2)
assert diem_vao_nho == 0, f"diem vao phai la 0 (diem dau tien, tang cao nhat=1) -- dang ra {diem_vao_nho}"
assert tang_cao_nhat_nho == 1, f"tang cao nhat phai la 1 -- dang ra {tang_cao_nhat_nho}"
assert do_thi_nho == [{0: [1], 1: [0, 2], 2: [1, 3], 3: [2]}, {0: [2], 2: [0]}], f"cau truc do thi nho sai -- dang ra {do_thi_nho}"

# bien: moi diem PHAI xuat hien o tang 0 (tang thap nhat), khong duoc thieu diem nao
assert len(do_thi[0]) == N, "tang 0 phai chua DU moi diem, khong thieu diem nao"
```

:::hints
- kind: attention
  body: Hai chỗ trống, trong vòng lặp `for t in range(min(L, tang_cao_nhat), -1, -1):` (bước "đi xuống, CÓ nối cạnh"). Chỗ đầu là TÊN HÀM cập nhật `hien_tai` bằng cách tìm tham lam TẠI TẦNG `t` — chính là hàm đã viết ở bài `3`. Chỗ hai là đối số THỨ BA của `noi_canh_va_cat` — láng giềng vừa tìm được để nối cạnh với điểm `i` (giá trị VỪA được cập nhật ở dòng ngay phía trên).
- kind: strategy
  body: 'Chỗ đầu: `tim_tham_lam` — gọi với `(do_thi[t], vectors, vectors[i], hien_tai)`. Chỗ hai: `hien_tai` — kết quả tìm tham lam vừa tính ở dòng trên, dùng làm láng giềng để nối.'
- kind: one-line
  body: 'Chỗ đầu là `tim_tham_lam`, chỗ hai là `hien_tai`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT tim_tham_lam(do_thi[t], vectors, vectors[i], hien_tai) de cap nhat hien_tai tai TUNG tang; VA cho trong hai phai dung DUNG bien 'hien_tai' (lang gieng vua tim duoc) lam doi so thu ba cua noi_canh_va_cat
  requireAst:
  - kind: uses-call, target: tim_tham_lam, min: 2
  - kind: uses-name, target: hien_tai, min: 7
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [2, 7] cho hai
  # luat theo dung thu tu khai bao o tren.
  # tim_tham_lam=2 (TONG THAT, da xac nhan bang cong cu, khong doan tay):
  # MOT lan CO SAN o vong lap PHIA TRUOC (buoc "di xuong, KHONG noi canh"),
  # MOT lan CHINH la cho trong dau. Neu chi dat min=1 (ngay tho), mot mutant
  # BO SOT cho trong dau (vi du dien "co_lang_gieng_gan_hon" nham ham, sai
  # kieu tra ve -- ham nay tra ve MOT gia tri, khong phai tuple hai gia tri,
  # se nem loi ngay) van co the qua NEU dung mot bieu thuc khac hop le --
  # nhung con lai 1 lan tim_tham_lam o vong truoc du de KHONG bat duoc bang
  # min=1 -- GOTCHA "boilerplate-threshold-masking"; da tu kiem chung bang
  # cong cu, min=2 (dung, TONG THAT) moi chan duoc dung.
  # hien_tai=7 (TONG THAT, da xac nhan bang cong cu): BON lan Load BEN
  # TRONG chinh than ham tim_tham_lam ("duong_di = [hien_tai]",
  # "co_lang_gieng_gan_hon(..., hien_tai)", "hien_tai = ke_tiep" la Store
  # khong dem, "duong_di.append(hien_tai)", "return hien_tai, duong_di" --
  # bon lan Load ke tren), CONG voi BA lan trong chinh chen_hnsw (vong truoc
  # goi tim_tham_lam(..., hien_tai) 1 lan, vong nay goi tim_tham_lam(...,
  # hien_tai) 1 lan nhu boilerplate, VA cho trong hai CHINH la lan thu ba).
  # Neu chi dat min=1 (ngay tho), mot mutant dien mot ten khac (vi du "i")
  # vao cho trong hai van qua duoc vi con lai 6 lan doc "hien_tai" ram rap o
  # noi khac -- GOTCHA "boilerplate-threshold-masking" RO RET nhat trong ca
  # quest nay; da tu kiem chung bang cong cu, CHI min=7 (TONG THAT) moi chan
  # duoc mutant nay.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 de xac nhan, khong doan tay): dien "hien_tai" vao cho trong dau
  # ("hien_tai, _ = hien_tai(do_thi[t], vectors, vectors[i], hien_tai)") VA
  # dien "tim_tham_lam" vao cho trong hai ("noi_canh_va_cat(do_thi[t], i,
  # tim_tham_lam, vectors, M)") -- KHAC VOI CAC BAI TRUOC, o day static
  # THAT SU BAT DUOC mutant nay: dem lai bang cong cu cho ket qua [1, 7]
  # (khong con [2, 7]) -- vi cho trong dau, hien_tai GIO O VI TRI GOI HAM
  # (Call), khong con la mot Load Name binh thuong, nen 'hien_tai' o do
  # KHONG duoc uses-call dem (dung); dong thoi 'tim_tham_lam' o cho trong
  # hai GIO O VI TRI THAM SO (Load, chi truyen di nhu mot gia tri), KHONG
  # con la mot Call, nen KHONG duoc uses-call dem nua -- lam tong so lan
  # GOI THAT 'tim_tham_lam' giam tu 2 xuong 1, duoi nguong min=2, static
  # chan duoc NGAY. Day la vi du cho thay hai cho trong o BAI NAY khong
  # DOI XUNG vai tro cu phap (mot ben la VI TRI GOI HAM, mot ben la VI TRI
  # THAM SO don thuan) nen hoan doi lam doi HINH DANG AST, khac han cac cap
  # cho trong CUNG kieu (hai ten don thuan) o cac bai khac cua quest nay.
  # Da tu chay THAT qua python3 xac nhan CA HAI: static day loi (dung, nhu
  # mo ta o tren), VA rieng mat 'run' cung nem TypeError ngay lap tuc ('int'
  # object is not callable, vi hien_tai luc do la mot so nguyen) -- hai
  # tang bat doc lap nhau, khong chi dua vao mot tang duy nhat.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^4\\n4\\n\\[20, 12, 4, 2, 2\\]\\n\\[16, 9, 3, 1, 1\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`[20, 12, 4, 2, 2]` điểm, `[16, 9, 3, 1, 1]` cạnh — giảm dần đều lên tầng
cao, đo được bằng số thật. Bài sau viết thuật toán TÌM KIẾM trên đúng cấu
trúc nhiều tầng này.
::::

::::reflect{#nghi-lai}
`chen_hnsw` không phải một thuật toán mới — nó là BA mảnh đã học (tìm tham
lam, nối cạnh có cắt bớt, gán tầng ngẫu nhiên) ráp lại theo đúng một trình
tự: xuống các tầng cao (chỉ để định vị, không nối cạnh, vì điểm mới không
tồn tại ở đó), rồi xuống các tầng của chính nó (định vị VÀ nối cạnh). Kết
quả đo được: một cấu trúc nhiều tầng có số điểm VÀ số cạnh đều giảm dần khi
lên cao — đúng tính chất "tầng trên thưa, tầng dưới đông" mà bài `gioi-han-
mot-tang-va-y-tuong-nhieu-tang` đã đặt ra làm mục tiêu. Bài sau dùng chính
cấu trúc này để TÌM KIẾM: bắt đầu ở điểm vào của tầng cao nhất, tìm tham lam
xuống dần từng tầng, và đo xem cấu trúc nhiều tầng có thực sự sửa được giới
hạn đã đo ở bài `4` hay không.
::::

::::checkpoint{mastery=0.85}
::::
